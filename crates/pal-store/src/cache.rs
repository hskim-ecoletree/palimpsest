//! 1층 캐시 — 콘텐츠 주소 파일 저장.
//!
//! # 왜 2층 엔진에 넣지 않는가 (stack §2.4)
//!
//! `.palimpsest/cache/ab/cdef….bin` 이다. 병렬 쓰기가 락 없이 되고, OS 페이지 캐시가
//! 그대로 값을 내며, **부분 손상이 그 파일 하나로 격리된다.** 2층 인덱스는 다른 파일이고
//! 다른 기능(F05)의 것이다.
//!
//! # 키가 커밋이 아닌 것이 이 설계의 전부다
//!
//! **성분은 다섯이고 커밋은 그중에 없다** — `(blob 이름, 추출기 버전, 경로, 선언된
//! 언어, 능력 축)`. 커밋이 키에 없으므로 **워킹트리 파일도 같은 캐시를 쓴다** —
//! [`pal_core::TreeRef::Worktree`] 가 공짜로 성립하는 이유가 이것이다(R-06).
//! 3년 전 커밋으로 체크아웃해도 내용이 같고 **경로가 같은** 파일은 이미 캐시에 있다.
//!
//! 추출기 버전이 키에 들어가는 이유는 stack §5.1 이다 — 문법이나 추출기 코드가 바뀌면
//! **1층 전량이 무효화되어야** 하고, 키에 있으면 그것이 삭제 없이 일어난다.
//! 나머지 셋의 근거는 [`CacheKey::new`] 에 있고, 규칙 하나는 [ADR-0004] 다 —
//! **산출을 정하는 입력을 전부 담는다.**
//!
//! [ADR-0004]: ../../../docs/adr/0004-cache-key-covers-every-input-that-decides-the-output.md
//!
//! # 이 크레이트는 의도 저장소에 닿지 않는다
//!
//! 지우는 API 가 여기 살기 때문이다(R-21). 아직 그런 API 는 없지만 — `prune` 은
//! F04 다 — 경계는 내용보다 먼저 선다.

use std::fs;
use std::path::{Path, PathBuf};

use pal_core::{ExtractorVersion, ObjectName, RepoPath};
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

/// 1층 캐시의 키 — **`(blob 이름, 추출기 버전, 경로, 선언된 언어, 능력 축)`**.
///
/// 값은 blake3 32바이트의 16진이다. blob 이름을 그대로 쓰지 않는 이유는 나머지 넷을
/// 섞어야 하기 때문이고, 섞은 결과가 다시 콘텐츠 주소이므로 디렉터리 분산이 고르다.
///
/// **성분이 다섯인 근거는 [ADR-0004] 하나다** — *"캐시 키는 내용이 아니라 산출을 정하는
/// 입력 전부를 담는다."* 성분이 왜 그것들인지는 [`CacheKey::new`] 에 하나씩 적혀 있다.
///
/// [ADR-0004]: ../../../docs/adr/0004-cache-key-covers-every-input-that-decides-the-output.md
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CacheKey(String);

impl CacheKey {
    /// 캐시 키 — `(blob, 추출기 버전, **분류 맥락**, **능력 축**)`.
    ///
    /// # 왜 blob 만으로는 안 되는가 (2026-08-13 · F01)
    ///
    /// 캐시가 담는 것은 [`FileOutcome`] 이고 그것은 **분류 결과**다. 그런데 분류는
    /// 내용만 보지 않는다 — 언어를 확장자·파일 이름에서 얻고, 생성물 판정이 경로
    /// 패턴을 보며, `.gitattributes` 의 `linguist-language` 가 그 전부를 이긴다.
    ///
    /// **키가 blob 뿐이면 내용이 같고 경로가 다른 두 파일이 한 칸을 쓴다.** 빈 파일이
    /// 그 자리다 — 빈 `.kt` 와 `.gitkeep` 은 같은 blob(`e69de29`)이고, 먼저 온 쪽의
    /// 분류가 뒤에 온 쪽에 그대로 나간다. 그러면 **대장이 파일의 언어를 틀리게 적고
    /// 그 틀림이 캐시에 굳는다.**
    ///
    /// 그래서 경로와 선언된 언어를 키에 넣는다. **값을 치르는 자리**: 파일을 옮기면
    /// 내용이 같아도 미스가 난다. 대장이 거짓말하지 않는 값으로 싼 편이다.
    ///
    /// # 왜 **능력 축**이 다섯째인가 (2026-08-13 · F04)
    ///
    /// 캐시가 담는 것이 그래프 전부가 되면서 [`pal_core::Capable`] 자리 넷이 함께
    /// 실린다. **능력은 빌드의 사실이지 저장된 사실이 아니다** — 안 만든 능력을 되읽으면
    /// *"아무것도 없음"* 으로 위장하고, 그것이 F22-3 이 고친 병이다.
    ///
    /// 그것을 막는 방법이 둘이었다: `Capable` 을 직렬화 가능하게 만들거나, **능력을
    /// 키의 성분으로 보내거나.** 후자를 골랐다 — [ADR-0004] 가 요구하는 것이
    /// *"산출을 정하는 입력 전부"* 이고 **이 빌드가 무슨 능력을 만드는가는 그 입력이기
    /// 때문**이다. 축은 `pal_extract::capability_axis()` 가 **빌드에서 재현**한다
    /// (손으로 적은 목록이 아니다 — [ADR-0004] 가 경고한 자리다).
    ///
    /// **문자열을 그대로 넣는다.** 미리 요약하면 성분이 하나 줄지 않고 읽을 수 없어질
    /// 뿐이다 — 어차피 여기서 다시 요약된다.
    ///
    /// [`FileOutcome`]: https://docs.rs/
    /// [ADR-0004]: ../../../docs/adr/0004-cache-key-covers-every-input-that-decides-the-output.md
    #[must_use]
    pub fn new(
        blob: ObjectName,
        version: ExtractorVersion,
        path: &RepoPath,
        declared: Option<&str>,
        capabilities: &str,
    ) -> Self {
        let mut hasher = blake3::Hasher::new();
        // 층 표시 + 키 형태 버전. **형태가 바뀌면 올린다** — 지금이 그 자리다
        // (성분이 넷에서 다섯으로 늘었다). 값이 바뀌는 것과는 다른 사건이고,
        // 그 구별을 [ADR-0004] 가 정했다.
        hasher.update(b"pal3\0");
        hasher.update(blob.as_bytes());
        hasher.update(b"\0");
        hasher.update(version.grammar.as_bytes());
        hasher.update(b"\0");
        hasher.update(version.extractor.as_bytes());
        hasher.update(b"\0");
        // 길이 접두사 — 없으면 `("ab", None)` 과 `("a", Some("b"))` 가 같은 키가 된다.
        let path = path.as_str().as_bytes();
        hasher.update(&(path.len() as u64).to_le_bytes());
        hasher.update(path);
        let declared = declared.unwrap_or("").as_bytes();
        hasher.update(&(declared.len() as u64).to_le_bytes());
        hasher.update(declared);
        let capabilities = capabilities.as_bytes();
        hasher.update(&(capabilities.len() as u64).to_le_bytes());
        hasher.update(capabilities);
        Self(hasher.finalize().to_hex().to_string())
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// 1층 캐시의 **포트** — stack §5.2 · F04 §2.
///
/// # 왜 구체 타입 하나로 두지 않는가
///
/// F04 §5 가 `rkyv` 를 기각하면서 *"postcard 로 시작하고 병목이 확인되면 교체 —
/// **트레잇 뒤라 교체 비용이 낮다**"* 라고 적었다. **그 트레잇이 없으면 그 문장이
/// 근거가 아니라 소원이다.** 교체 자리를 만드는 것이 여기까지이고, 바꾸는 것은
/// 여기가 아니다(`[f04.does_not_prove]`).
///
/// # 문서의 시그니처와 다른 두 자리
///
///   · **값이 `FileGraph` 로 고정되지 않고 제네릭이다.** 실제로 실리는 것은
///     [`pal_extract::FileOutcome`] 이고(분류 결과까지 담아야 한다 — [ADR-0004]),
///     이 크레이트는 `pal-extract` 에 의존하지 않는다(의존 방향 · stack §4.1).
///     그래서 값의 타입을 아는 것은 **부르는 쪽**이다
///   · **오류 타입이 [`CacheError`] 로 고정이다.** 저장 기술이 바뀌어도 부르는 쪽이
///     보는 것은 *"읽지 못했다 · 풀지 못했다"* 셋이고, 그것을 연관 타입으로 열면
///     [R-15](저장 기술이 밖으로 새지 않는다)가 트레잇 자신에서 깨진다
///
/// 제네릭 메서드가 있어 `dyn` 이 되지 않는다. **되지 않아야 한다** — 1층은 실행
/// 중에 갈아 끼우는 것이 아니라 빌드에서 고르는 것이다.
///
/// [ADR-0004]: ../../../docs/adr/0004-cache-key-covers-every-input-that-decides-the-output.md
/// [`pal_extract::FileOutcome`]: https://docs.rs/
pub trait ExtractCache: Send + Sync {
    /// 있으면 값을, 없으면 `None`.
    ///
    /// # Errors
    /// 파일은 있는데 읽지 못하거나 풀지 못하면.
    fn get<T: DeserializeOwned>(&self, key: &CacheKey) -> Result<Option<T>, CacheError>;

    /// 값을 넣는다. **원자적이다.**
    ///
    /// # Errors
    /// 직렬화·쓰기·이동 중 하나가 실패하면.
    fn put<T: Serialize>(&self, key: &CacheKey, value: &T) -> Result<(), CacheError>;
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

    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }
}

impl ExtractCache for BlobCache {
    /// 있으면 값을, 없으면 `None`.
    ///
    /// **`None` 은 "캐시에 없다"이지 "값이 없다"가 아니다** — 조회 결과이므로
    /// `Option` 이 맞다(stack §5.4 의 허용 자리).
    ///
    /// # Errors
    /// 파일은 있는데 읽지 못하거나 풀지 못하면. **깨진 캐시를 조용히 미스로 만들지
    /// 않는다** — 그러면 손상이 성능 저하로만 보이고 영원히 발견되지 않는다.
    fn get<T: DeserializeOwned>(&self, key: &CacheKey) -> Result<Option<T>, CacheError> {
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
    /// # ⚠ 임시 이름이 **키마다 하나면 그 보장이 깨진다**
    ///
    /// 옛 코드는 `<키>.tmp` 하나를 썼다. 같은 키를 두 쓰는 이가 동시에 넣으면 **둘이 같은
    /// 임시 파일에 겹쳐 쓰고**, 한쪽이 반쯤 쓴 것을 다른 쪽이 `rename` 한다 — 그 결과가
    /// 정상 캐시 파일로 남는다. 락이 없어도 되는 이유가 *"각자 자기 임시 파일에 쓴다"*
    /// 인데 그 전제가 없었던 것이다.
    ///
    /// 병렬 추출(#49)이 이 자리를 실제로 밟는다. 임시 이름에 **프로세스와 스레드**를
    /// 넣어 쓰는 이마다 갈랐다.
    ///
    /// # Errors
    /// 직렬화·쓰기·이동 중 하나가 실패하면.
    fn put<T: Serialize>(&self, key: &CacheKey, value: &T) -> Result<(), CacheError> {
        let path = self.path_of(key);
        let dir = path.parent().unwrap_or(&self.root);
        fs::create_dir_all(dir)
            .map_err(|e| CacheError::Create(format!("{}: {e}", dir.display())))?;

        let raw = postcard::to_allocvec(value)
            .map_err(|e| CacheError::Write(format!("직렬화: {e}")))?;
        let packed = zstd::encode_all(raw.as_slice(), ZSTD_LEVEL)
            .map_err(|e| CacheError::Write(format!("압축: {e}")))?;

        // **쓰는 이마다 다른 임시 이름.** 위 주석의 이유다.
        let tmp = path.with_extension(format!(
            "{}.{:?}.tmp",
            std::process::id(),
            std::thread::current().id()
        ));
        fs::write(&tmp, &packed)
            .map_err(|e| CacheError::Write(format!("{}: {e}", tmp.display())))?;
        fs::rename(&tmp, &path)
            .map_err(|e| CacheError::Write(format!("{}: {e}", path.display())))?;
        Ok(())
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

    /// 시험용 능력 축 — 실물은 `pal_extract::capability_axis()` 가 낸다.
    const 능력: &str = "Kotlin|exports=not-built:F02/kotlin-exports";

    fn 키(blob: ObjectName, v: ExtractorVersion) -> CacheKey {
        CacheKey::new(blob, v, &RepoPath::new("a/b.kt"), None, 능력)
    }

    #[test]
    fn 넣은_것이_나온다() {
        let c = BlobCache::open(임시()).unwrap();
        let k = 키(ObjectName::from_bytes([1; 20]), V);
        assert_eq!(c.get::<값>(&k).unwrap(), None);
        c.put(&k, &값 { n: 7, s: "가".into() }).unwrap();
        assert_eq!(c.get::<값>(&k).unwrap(), Some(값 { n: 7, s: "가".into() }));
        let _ = fs::remove_dir_all(c.root());
    }

    #[test]
    fn 추출기_버전이_바뀌면_다른_키다() {
        // **이것이 없으면 문법을 바꿔도 옛 값이 나온다** — stack §5.1.
        let blob = ObjectName::from_bytes([2; 20]);
        let a = 키(blob, V);
        let b = 키(blob, ExtractorVersion { grammar: "g2", extractor: "e" });
        assert_ne!(a, b);
    }

    #[test]
    fn blob_이_바뀌면_다른_키다() {
        let a = 키(ObjectName::from_bytes([3; 20]), V);
        let b = 키(ObjectName::from_bytes([4; 20]), V);
        assert_ne!(a, b);
    }

    #[test]
    fn 같은_blob_이라도_경로가_다르면_다른_키다() {
        // **빈 파일이 이 자리다.** 빈 `.kt` 와 `.gitkeep` 은 같은 blob 이고, 키가
        // blob 뿐이면 먼저 온 쪽의 **분류**가 뒤에 온 쪽에 그대로 나간다.
        let blob = ObjectName::from_bytes([5; 20]);
        let a = CacheKey::new(blob, V, &RepoPath::new("a/x.kt"), None, 능력);
        let b = CacheKey::new(blob, V, &RepoPath::new("a/.gitkeep"), None, 능력);
        assert_ne!(a, b);
    }

    #[test]
    fn 선언된_언어가_다르면_다른_키다() {
        // `.gitattributes` 의 `linguist-language` 가 바뀌면 분류가 바뀐다.
        let blob = ObjectName::from_bytes([6; 20]);
        let path = RepoPath::new("a/x.txt");
        let a = CacheKey::new(blob, V, &path, None, 능력);
        let b = CacheKey::new(blob, V, &path, Some("Kotlin"), 능력);
        assert_ne!(a, b);
    }

    #[test]
    fn 능력_축이_바뀌면_다른_키다() {
        // **이것이 없으면 스코프 체인을 만들기 시작한 빌드가 옛 항목을 그대로 읽는다** —
        // 안 만든 능력이 빈 값으로 위장하는 F22-3 의 병이 캐시를 통해 돌아온다.
        let blob = ObjectName::from_bytes([7; 20]);
        let path = RepoPath::new("a/x.ts");
        let a = CacheKey::new(blob, V, &path, None, "TypeScript|scopes=built");
        let b = CacheKey::new(blob, V, &path, None, "TypeScript|scopes=not-built:F02/x");
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
