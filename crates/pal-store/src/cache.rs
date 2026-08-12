//! 1층 캐시 — 콘텐츠 주소 파일 저장.
//!
//! # 왜 2층 엔진에 넣지 않는가 (stack §2.4)
//!
//! `.palimpsest/cache/ab/cdef….bin` 이다. 병렬 쓰기가 락 없이 되고, OS 페이지 캐시가
//! 그대로 값을 내며, **부분 손상이 그 파일 하나로 격리된다.** 2층 인덱스는 다른 파일이고
//! 다른 기능(F05)의 것이다.
//!
//! # 키가 커밋이 아니라 blob 인 것이 이 설계의 전부다
//!
//! `(blob 이름, 추출기 버전)` 이다. 커밋이 키에 없으므로 **워킹트리 파일도 같은 캐시를
//! 쓴다** — [`pal_core::TreeRef::Worktree`] 가 공짜로 성립하는 이유가 이것이다(R-06).
//! 3년 전 커밋으로 체크아웃해도 내용이 같은 파일은 이미 캐시에 있다.
//!
//! 추출기 버전이 키에 들어가는 이유는 stack §5.1 이다 — 문법이나 추출기 코드가 바뀌면
//! **1층 전량이 무효화되어야** 하고, 키에 있으면 그것이 삭제 없이 일어난다.
//!
//! # 이 크레이트는 의도 저장소에 닿지 않는다
//!
//! 지우는 API 가 여기 살기 때문이다(R-21). 아래에 그런 API 는 아직 없지만 — `prune` 은
//! F04 다 — 경계는 내용보다 먼저 선다.

use std::fs;
use std::path::{Path, PathBuf};

use pal_core::{ExtractorVersion, ObjectName};
use serde::{Serialize, de::DeserializeOwned};

#[derive(Debug, thiserror::Error)]
pub enum CacheError {
    #[error("캐시 디렉터리를 만들지 못했다: {0}")]
    Create(String),
    #[error("캐시를 쓰지 못했다: {0}")]
    Write(String),
    #[error("캐시를 읽지 못했다: {0}")]
    Read(String),
    #[error("캐시 값을 풀지 못했다: {0}")]
    Decode(String),
}

/// 1층 캐시의 키 — **`(blob 이름, 추출기 버전)`**.
///
/// 값은 blake3 32바이트의 16진이다. blob 이름을 그대로 쓰지 않는 이유는 추출기 버전을
/// 섞어야 하기 때문이고, 섞은 결과가 다시 콘텐츠 주소이므로 디렉터리 분산이 고르다.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CacheKey(String);

impl CacheKey {
    #[must_use]
    pub fn new(blob: ObjectName, version: ExtractorVersion) -> Self {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"pal1\0"); // 층 표시. 다른 층이 같은 키 공간을 쓰지 않게 한다
        hasher.update(blob.as_bytes());
        hasher.update(b"\0");
        hasher.update(version.grammar.as_bytes());
        hasher.update(b"\0");
        hasher.update(version.extractor.as_bytes());
        Self(hasher.finalize().to_hex().to_string())
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// 콘텐츠 주소 캐시.
pub struct BlobCache {
    root: PathBuf,
}

/// 압축 레벨 — stack §3.1 이 3 으로 고정했다.
const ZSTD_LEVEL: i32 = 3;

impl BlobCache {
    /// 캐시 디렉터리를 연다. 없으면 만든다.
    ///
    /// # Errors
    /// 디렉터리를 만들지 못하면.
    pub fn open(root: impl Into<PathBuf>) -> Result<Self, CacheError> {
        let root = root.into();
        fs::create_dir_all(&root)
            .map_err(|e| CacheError::Create(format!("{}: {e}", root.display())))?;
        Ok(Self { root })
    }

    fn path_of(&self, key: &CacheKey) -> PathBuf {
        // 앞 두 글자로 갈라 한 디렉터리에 파일이 몰리지 않게 한다.
        self.root.join(&key.0[..2]).join(format!("{}.bin", &key.0[2..]))
    }

    /// 있으면 값을, 없으면 `None`.
    ///
    /// **`None` 은 "캐시에 없다"이지 "값이 없다"가 아니다** — 조회 결과이므로
    /// `Option` 이 맞다(stack §5.4 의 허용 자리).
    ///
    /// # Errors
    /// 파일은 있는데 읽지 못하거나 풀지 못하면. **깨진 캐시를 조용히 미스로 만들지
    /// 않는다** — 그러면 손상이 성능 저하로만 보이고 영원히 발견되지 않는다.
    pub fn get<T: DeserializeOwned>(&self, key: &CacheKey) -> Result<Option<T>, CacheError> {
        let path = self.path_of(key);
        let packed = match fs::read(&path) {
            Ok(b) => b,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(e) => return Err(CacheError::Read(format!("{}: {e}", path.display()))),
        };
        let raw = zstd::decode_all(packed.as_slice())
            .map_err(|e| CacheError::Decode(format!("{}: {e}", path.display())))?;
        let value = postcard::from_bytes(&raw)
            .map_err(|e| CacheError::Decode(format!("{}: {e}", path.display())))?;
        Ok(Some(value))
    }

    /// 값을 넣는다. **원자적이다** — 임시 파일에 쓰고 옮긴다.
    ///
    /// 중간에 죽으면 반쪽 파일이 남고, 그것을 다음 실행이 정상 캐시로 읽으면 조용한
    /// 오답이 된다. 병렬 쓰기가 락 없이 되는 것도 이 덕분이다.
    ///
    /// # Errors
    /// 직렬화·쓰기·이동 중 하나가 실패하면.
    pub fn put<T: Serialize>(&self, key: &CacheKey, value: &T) -> Result<(), CacheError> {
        let path = self.path_of(key);
        let dir = path.parent().unwrap_or(&self.root);
        fs::create_dir_all(dir)
            .map_err(|e| CacheError::Create(format!("{}: {e}", dir.display())))?;

        let raw = postcard::to_allocvec(value)
            .map_err(|e| CacheError::Write(format!("직렬화: {e}")))?;
        let packed = zstd::encode_all(raw.as_slice(), ZSTD_LEVEL)
            .map_err(|e| CacheError::Write(format!("압축: {e}")))?;

        let tmp = path.with_extension("tmp");
        fs::write(&tmp, &packed)
            .map_err(|e| CacheError::Write(format!("{}: {e}", tmp.display())))?;
        fs::rename(&tmp, &path)
            .map_err(|e| CacheError::Write(format!("{}: {e}", path.display())))?;
        Ok(())
    }

    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }
}

/// 한 회차의 적중·빗나감. **대장이 이것을 보고한다.**
///
/// 캐시는 검증하지 않으면 거짓말하기 가장 쉬운 부품이다 — "적중했다"고 말하는 코드는
/// 아무것도 안 해도 짤 수 있다. 그래서 수를 산출에 실어 **보이게** 한다.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
pub struct CacheStats {
    pub hits: usize,
    pub misses: usize,
}

impl CacheStats {
    pub const fn hit(&mut self) {
        self.hits += 1;
    }

    pub const fn miss(&mut self) {
        self.misses += 1;
    }

    #[must_use]
    pub const fn total(&self) -> usize {
        self.hits + self.misses
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct 값 {
        n: usize,
        s: String,
    }

    fn 임시() -> PathBuf {
        let p = std::env::temp_dir().join(format!("pal-cache-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&p);
        p
    }

    const V: ExtractorVersion = ExtractorVersion { grammar: "g", extractor: "e" };

    #[test]
    fn 넣은_것이_나온다() {
        let c = BlobCache::open(임시()).unwrap();
        let k = CacheKey::new(ObjectName::from_bytes([1; 20]), V);
        assert_eq!(c.get::<값>(&k).unwrap(), None);
        c.put(&k, &값 { n: 7, s: "가".into() }).unwrap();
        assert_eq!(c.get::<값>(&k).unwrap(), Some(값 { n: 7, s: "가".into() }));
        let _ = fs::remove_dir_all(c.root());
    }

    #[test]
    fn 추출기_버전이_바뀌면_다른_키다() {
        // **이것이 없으면 문법을 바꿔도 옛 값이 나온다** — stack §5.1.
        let blob = ObjectName::from_bytes([2; 20]);
        let a = CacheKey::new(blob, V);
        let b = CacheKey::new(blob, ExtractorVersion { grammar: "g2", extractor: "e" });
        assert_ne!(a, b);
    }

    #[test]
    fn blob_이_바뀌면_다른_키다() {
        let a = CacheKey::new(ObjectName::from_bytes([3; 20]), V);
        let b = CacheKey::new(ObjectName::from_bytes([4; 20]), V);
        assert_ne!(a, b);
    }

    #[test]
    fn 통계는_전부를_센다() {
        let mut s = CacheStats::default();
        s.hit();
        s.hit();
        s.miss();
        assert_eq!((s.hits, s.misses, s.total()), (2, 1, 3));
    }
}
