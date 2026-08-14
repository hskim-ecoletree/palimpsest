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
use std::time::Duration;

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
    /// 조회 — **결과가 셋이다.** [`Lookup`] 의 주석이 그 이유다.
    ///
    /// # Errors
    /// 파일시스템이 실패하면. **깨진 값은 오류가 아니다** — [`Lookup::Corrupt`] 다.
    fn lookup<T: DeserializeOwned>(&self, key: &CacheKey) -> Result<Lookup<T>, CacheError>;

    /// 값을 넣는다. **원자적이다.**
    ///
    /// # Errors
    /// 직렬화·쓰기·이동 중 하나가 실패하면.
    fn put<T: Serialize>(&self, key: &CacheKey, value: &T) -> Result<(), CacheError>;

    /// 지금 얼마나 차 있는가 — `pal cache stats` 가 내는 값.
    ///
    /// # Errors
    /// 디렉터리를 훑지 못하면.
    fn usage(&self) -> Result<CacheUsage, CacheError>;

    /// 예산까지 줄인다. **닿는 곳은 이 캐시의 뿌리 아래뿐이다** ([R-21]).
    ///
    /// # Errors
    /// 훑거나 지우지 못하면.
    ///
    /// [R-21]: ../../../docs/plan/00-risks.md#r-21
    fn evict_to(&self, budget_bytes: u64) -> Result<EvictReport, CacheError>;

    /// **격리 방**을 예산까지 줄인다 — 오래된 것부터.
    ///
    /// # 기본으로 일어나지 않는다 (`[f05.5.pass]` ①)
    ///
    /// 격리된 바이트는 **결함의 증거**다. 예산 때문에 지우면 격리가 유예된 삭제가 되고,
    /// 그것이 F04 가 *"축출이 안 건드린다"* 로 정한 이유다. **처분이 생기되 부르는 쪽이
    /// 명시해야 한다.**
    ///
    /// # Errors
    /// 훑거나 지우지 못하면.
    fn sweep_quarantine(&self, budget_bytes: u64) -> Result<EvictReport, CacheError>;

    /// **죽은 `.tmp`** 를 지운다 — `older_than` 보다 오래된 것만.
    ///
    /// # 나이가 유일한 가름이다 (`[f05.5.pass]` ②)
    ///
    /// `.tmp` 는 죽은 쓰기이거나 **지금 도는 쓰기**다. 나이 없이 지우면 도는 쓰기의
    /// `rename` 이 깨진다 — 그것이 F04 가 *"세기는 하고 지우지 않는다"* 로 둔 이유다.
    ///
    /// # Errors
    /// 훑거나 지우지 못하면.
    fn sweep_stray(&self, older_than: Duration) -> Result<SweepReport, CacheError>;
}

/// `.tmp` 청소 한 회차의 회계.
///
/// **본 것과 지운 것을 따로 적는다** — 숫자만 내고 안 지우는 구현이 하나만 보면
/// 통과한다(`[f04.pass]` ④ 와 같은 형태).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
pub struct SweepReport {
    /// 본 `.tmp` 의 수.
    pub scanned: usize,
    pub removed: usize,
    pub freed_bytes: u64,
    /// **어려서 남긴 것.** 0 이 아니면 그 회차에 도는 쓰기가 있었다는 뜻이다.
    pub too_young: usize,
}

/// 콘텐츠 주소 캐시.
pub struct BlobCache {
    root: PathBuf,
}

/// 압축 레벨 — stack §3.1 이 3 으로 고정했다.
const ZSTD_LEVEL: i32 = 3;

/// 깨진 엔트리가 옮겨 가는 방. **캐시 안이고, 축출이 건드리지 않는다.**
const QUARANTINE: &str = ".corrupt";

/// 캐시 엔트리 하나를 훑은 결과 — `(자리, 바이트, 마지막 손댄 때)`.
type Scanned = (PathBuf, u64, std::time::SystemTime);

/// 조회 결과 — **셋이다.**
///
/// `Option` 이 아닌 이유는 [ADR-0005] 그대로다: *"없다"* 와 *"깨졌다"* 는 다른 부재이고,
/// **축출이 생기면서 앞엣것이 정상이 되었다.** 둘을 접으면 축출 뒤의 미스가 사건처럼
/// 보이거나(과잉 경보) 손상이 성능 저하로만 보인다(과소 경보).
///
/// [ADR-0005]: ../../../docs/adr/0005-absence-carries-its-kind.md
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Lookup<T> {
    Hit(T),
    /// 이 키의 엔트리가 없다. **정상이다** — 첫 회차이거나 축출됐다.
    Miss,
    /// 있었는데 풀리지 않았다. **사건이다.** 바이트는 격리 방에 남아 있다.
    Corrupt {
        /// 격리된 바이트가 지금 있는 곳. **지워지지 않았다.**
        quarantined: PathBuf,
        cause: String,
    },
}

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

    /// 격리 방 — 캐시 **안**이다.
    ///
    /// 밖에 두면 [R-21] 의 경계를 이 크레이트가 스스로 넘는다. `.` 으로 시작하므로
    /// 16진 두 글자인 샤드 디렉터리와 절대 섞이지 않는다.
    #[must_use]
    pub fn quarantine_dir(&self) -> PathBuf {
        self.root.join(QUARANTINE)
    }

    /// 캐시 안의 엔트리 전부 — `(자리, 바이트, 마지막 접근)`.
    ///
    /// # 무엇을 세지 **않는가**
    ///
    ///   · **격리 방**(`.corrupt/`) — 축출의 대상이 아니다. 깨진 바이트를 예산 때문에
    ///     지우면 격리가 유예된 삭제가 된다
    ///   · **임시 파일**(`.tmp`) — **쓰는 이가 지금 들고 있을 수 있다.** 지우면
    ///     그 쓰기의 `rename` 이 깨진다. 세기는 하고([`CacheUsage::stray_bytes`])
    ///     지우지 않는다
    fn entries(&self) -> Result<(Vec<Scanned>, u64), CacheError> {
        let mut out = Vec::new();
        let mut stray = 0u64;
        let shards = match fs::read_dir(&self.root) {
            Ok(d) => d,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok((out, 0)),
            Err(e) => return Err(CacheError::Read(format!("{}: {e}", self.root.display()))),
        };
        for shard in shards {
            let shard = shard.map_err(|e| CacheError::Read(e.to_string()))?.path();
            // **깊이 둘의 고정 구조다** — 그래서 `walkdir` 을 안 들인다.
            if !shard.is_dir() || shard.file_name().is_some_and(|n| n == QUARANTINE) {
                continue;
            }
            for file in fs::read_dir(&shard)
                .map_err(|e| CacheError::Read(format!("{}: {e}", shard.display())))?
            {
                let file = file.map_err(|e| CacheError::Read(e.to_string()))?;
                let path = file.path();
                let meta = file.metadata().map_err(|e| CacheError::Read(e.to_string()))?;
                if !meta.is_file() {
                    continue;
                }
                if path.extension().is_some_and(|e| e == "tmp") {
                    stray += meta.len();
                    continue;
                }
                // **접근 시각을 mtime 으로 근사한다**(F04 §3.4) — `noatime` 마운트에서도
                // 값이 있고 별도 메타데이터가 필요 없다. 적중은 mtime 을 안 올리므로
                // 이것은 *"언제 채워졌는가"* 에 가깝다. **그 사실을 적어 둔다.**
                let when = meta.modified().unwrap_or(std::time::UNIX_EPOCH);
                out.push((path, meta.len(), when));
            }
        }
        Ok((out, stray))
    }

    /// 깨진 엔트리를 격리 방으로 옮긴다. **지우지 않는다.**
    fn quarantine(&self, key: &CacheKey, path: &Path) -> Result<PathBuf, CacheError> {
        let dir = self.quarantine_dir();
        fs::create_dir_all(&dir)
            .map_err(|e| CacheError::Create(format!("{}: {e}", dir.display())))?;
        let to = dir.join(format!("{}.bin", key.as_str()));
        // 같은 키가 두 번 깨지면 뒤엣것이 앞엣것을 덮는다. **키가 같으면 같은 사건**이고,
        // 회차마다 파일을 늘리면 격리 방이 캐시보다 커진다.
        fs::rename(path, &to)
            .map_err(|e| CacheError::Write(format!("격리 실패 {}: {e}", to.display())))?;
        Ok(to)
    }
}

impl ExtractCache for BlobCache {
    /// 적중 · 부재 · **손상** 셋으로 답한다.
    ///
    /// # 왜 셋인가 — 문서와 옛 코드가 반대였다 (F04 · #7)
    ///
    /// F04 문서 §4 는 *"역직렬화 실패 시 그 엔트리만 버리고 재계산 + 경고 로그"* 라
    /// 적었고, 옛 코드는 반대로 `Err` 를 냈다 — *"깨진 캐시를 조용히 미스로 만들지
    /// 않는다. 그러면 손상이 성능 저하로만 보이고 영원히 발견되지 않는다."*
    /// **둘 다 근거가 있다.** 문서는 *진행해야 한다*(1층은 순수 캐시다 · §3.1)를,
    /// 코드는 *조용하면 안 된다*를 지킨다.
    ///
    /// 그리고 **축출이 생기면서 문제가 한 겹 깊어졌다** — 축출 뒤에는 **없는 엔트리가
    /// 정상**이다. 그러면 *"없다"* 와 *"깨졌다"* 를 한 값으로 접을 수 없다. 접으면
    /// 적중률 숫자가 무엇을 세는지 알 수 없게 된다.
    ///
    /// 그래서 [ADR-0005](부재는 종류를 싣는다)를 조회에 그대로 적용했다. 진행하고
    /// (재계산), 조용하지 않고(수가 산출에 실린다), **깨진 바이트를 지우지 않는다**
    /// (격리는 삭제가 아니다 — 지우면 사후에 무엇이 깨졌는지 아무도 못 본다).
    ///
    /// # Errors
    /// 파일시스템이 실패하면 — 읽기 권한·격리 이동. **깨진 값은 오류가 아니다.**
    ///
    /// [ADR-0005]: ../../../docs/adr/0005-absence-carries-its-kind.md
    fn lookup<T: DeserializeOwned>(&self, key: &CacheKey) -> Result<Lookup<T>, CacheError> {
        let path = self.path_of(key);
        let packed = match fs::read(&path) {
            Ok(b) => b,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Lookup::Miss),
            Err(e) => return Err(CacheError::Read(format!("{}: {e}", path.display()))),
        };
        let value = zstd::decode_all(packed.as_slice())
            .map_err(|e| e.to_string())
            .and_then(|raw| postcard::from_bytes::<T>(&raw).map_err(|e| e.to_string()));
        match value {
            Ok(v) => Ok(Lookup::Hit(v)),
            Err(cause) => {
                let quarantined = self.quarantine(key, &path)?;
                Ok(Lookup::Corrupt { quarantined, cause })
            }
        }
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

    fn usage(&self) -> Result<CacheUsage, CacheError> {
        let (entries, stray_bytes) = self.entries()?;
        let dir = self.quarantine_dir();
        let mut quarantined = (0usize, 0u64);
        if let Ok(read) = fs::read_dir(&dir) {
            for f in read {
                let f = f.map_err(|e| CacheError::Read(e.to_string()))?;
                let meta = f.metadata().map_err(|e| CacheError::Read(e.to_string()))?;
                if meta.is_file() {
                    quarantined.0 += 1;
                    quarantined.1 += meta.len();
                }
            }
        }
        Ok(CacheUsage {
            entries: entries.len(),
            bytes: entries.iter().map(|(_, n, _)| n).sum(),
            quarantined_entries: quarantined.0,
            quarantined_bytes: quarantined.1,
            stray_bytes,
        })
    }

    /// LRU 축출 — 예산을 넘은 만큼 **오래된 것부터** 지운다.
    ///
    /// # 왜 정책이 이렇게 단순해도 되는가
    ///
    /// F04 §3.4: *"축출은 정확성에 영향이 없다 — 재파싱될 뿐이다."* 틀린 것을 지울
    /// 위험이 없으므로 정교할 이유가 없다. **위험한 것은 지우는 범위이지 지우는
    /// 순서가 아니다.**
    ///
    /// # 보고가 「센 것」과 「지운 것」을 따로 적는다
    ///
    /// 숫자만 내고 안 지우는 구현이 [`EvictReport`] 하나만 보면 통과한다
    /// (`corpus/criteria.toml` `[f04.pass]` ④). 그래서 지운 뒤의 **남은 수**를 함께
    /// 낸다 — 부르는 쪽이 실제 파일 수와 댈 수 있다.
    fn evict_to(&self, budget_bytes: u64) -> Result<EvictReport, CacheError> {
        let (mut entries, _) = self.entries()?;
        let scanned = entries.len();
        let total: u64 = entries.iter().map(|(_, n, _)| n).sum();

        // **오래된 것부터.** 같은 시각이면 자리 순 — 정렬이 결정적이어야 두 회차가
        // 같은 것을 지운다.
        entries.sort_by(|a, b| a.2.cmp(&b.2).then_with(|| a.0.cmp(&b.0)));

        let mut live = total;
        let mut removed = 0usize;
        let mut freed = 0u64;
        for (path, size, _) in entries {
            if live <= budget_bytes {
                break;
            }
            fs::remove_file(&path)
                .map_err(|e| CacheError::Write(format!("{}: {e}", path.display())))?;
            live -= size;
            freed += size;
            removed += 1;
        }
        Ok(EvictReport {
            scanned,
            removed,
            freed_bytes: freed,
            kept_entries: scanned - removed,
            kept_bytes: live,
            budget_bytes,
        })
    }

    /// 격리 방을 예산까지 — **오래된 것부터.**
    ///
    /// 축출과 같은 규칙을 **다른 방**에 적용한다. 한 함수로 합치지 않는 이유: 합치면
    /// 예산 하나가 두 방을 함께 재고, 그러면 *"캐시가 커서 증거가 지워지는"* 경로가 생긴다.
    fn sweep_quarantine(&self, budget_bytes: u64) -> Result<EvictReport, CacheError> {
        let dir = self.quarantine_dir();
        let mut entries: Vec<Scanned> = Vec::new();
        if let Ok(read) = fs::read_dir(&dir) {
            for f in read {
                let f = f.map_err(|e| CacheError::Read(e.to_string()))?;
                let meta = f.metadata().map_err(|e| CacheError::Read(e.to_string()))?;
                if meta.is_file() {
                    entries.push((
                        f.path(),
                        meta.len(),
                        meta.modified().unwrap_or(std::time::UNIX_EPOCH),
                    ));
                }
            }
        }
        let scanned = entries.len();
        let total: u64 = entries.iter().map(|(_, n, _)| n).sum();
        entries.sort_by(|a, b| a.2.cmp(&b.2).then_with(|| a.0.cmp(&b.0)));

        let mut live = total;
        let mut removed = 0usize;
        let mut freed = 0u64;
        for (path, size, _) in entries {
            if live <= budget_bytes {
                break;
            }
            fs::remove_file(&path)
                .map_err(|e| CacheError::Write(format!("{}: {e}", path.display())))?;
            live -= size;
            freed += size;
            removed += 1;
        }
        Ok(EvictReport {
            scanned,
            removed,
            freed_bytes: freed,
            kept_entries: scanned - removed,
            kept_bytes: live,
            budget_bytes,
        })
    }

    /// 죽은 `.tmp` 를 지운다 — **나이로 가른다.**
    fn sweep_stray(&self, older_than: Duration) -> Result<SweepReport, CacheError> {
        let now = std::time::SystemTime::now();
        let mut out = SweepReport::default();
        let shards = match fs::read_dir(&self.root) {
            Ok(d) => d,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(out),
            Err(e) => return Err(CacheError::Read(format!("{}: {e}", self.root.display()))),
        };
        for shard in shards {
            let shard = shard.map_err(|e| CacheError::Read(e.to_string()))?.path();
            if !shard.is_dir() || shard.file_name().is_some_and(|n| n == QUARANTINE) {
                continue;
            }
            for file in fs::read_dir(&shard)
                .map_err(|e| CacheError::Read(format!("{}: {e}", shard.display())))?
            {
                let file = file.map_err(|e| CacheError::Read(e.to_string()))?;
                let path = file.path();
                if path.extension().is_none_or(|e| e != "tmp") {
                    continue;
                }
                let meta = file.metadata().map_err(|e| CacheError::Read(e.to_string()))?;
                out.scanned += 1;
                let 나이 = meta
                    .modified()
                    .ok()
                    .and_then(|m| now.duration_since(m).ok())
                    .unwrap_or_default();
                if 나이 < older_than {
                    // ★ **어린 것은 지금 도는 쓰기일 수 있다.** 지우면 그 쓰기의
                    // `rename` 이 깨진다 — 나이가 둘을 가르는 유일한 값이다.
                    out.too_young += 1;
                    continue;
                }
                let size = meta.len();
                fs::remove_file(&path)
                    .map_err(|e| CacheError::Write(format!("{}: {e}", path.display())))?;
                out.removed += 1;
                out.freed_bytes += size;
            }
        }
        Ok(out)
    }
}

/// 캐시가 지금 얼마나 차 있는가 — `pal cache stats`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
pub struct CacheUsage {
    pub entries: usize,
    pub bytes: u64,
    /// 깨져서 격리된 것 — **축출이 안 건드린다.** 0 이 정상 상태다.
    pub quarantined_entries: usize,
    pub quarantined_bytes: u64,
    /// 남은 임시 파일의 바이트. **죽은 쓰기의 흔적이거나 지금 도는 쓰기다.**
    ///
    /// 지우지 않는다 — 둘을 값싸게 구별할 수 없고, 도는 쓰기의 것을 지우면 그 쓰기의
    /// `rename` 이 깨진다. **보이게 두는 것이 지금의 처분이다.**
    pub stray_bytes: u64,
}

/// 축출 한 번의 보고. **센 것과 지운 것과 남은 것을 따로 적는다.**
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
pub struct EvictReport {
    pub scanned: usize,
    pub removed: usize,
    pub freed_bytes: u64,
    pub kept_entries: usize,
    pub kept_bytes: u64,
    pub budget_bytes: u64,
}

/// 한 회차의 적중·빗나감. **대장이 이것을 보고한다.**
///
/// 캐시는 검증하지 않으면 거짓말하기 가장 쉬운 부품이다 — "적중했다"고 말하는 코드는
/// 아무것도 안 해도 짤 수 있다. 그래서 수를 산출에 실어 **보이게** 한다.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
pub struct CacheStats {
    pub hits: usize,
    pub misses: usize,
    /// 깨져서 격리된 엔트리 수. **`misses` 에 섞지 않는다.**
    ///
    /// 축출이 생긴 뒤로 미스는 **정상**이다. 손상을 미스로 세면 *"캐시가 좀 덜
    /// 맞았다"* 와 *"디스크가 썩고 있다"* 가 같은 숫자가 된다.
    pub corrupt: usize,
}

impl CacheStats {
    pub const fn hit(&mut self) {
        self.hits += 1;
    }

    pub const fn miss(&mut self) {
        self.misses += 1;
    }

    /// 깨진 것을 센다. **미스도 함께 센다** — 값을 못 얻었으므로 재계산이 일어나고,
    /// 그러면 `hits + misses` 가 본 파일 수와 같다는 성질이 유지된다.
    pub const fn corrupt(&mut self) {
        self.corrupt += 1;
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

    /// **시험마다 다른 방.** 같은 디렉터리를 돌려 쓰면 한 시험이 다른 시험의 캐시를
    /// 보고, 그것이 F02-4 에서 병렬 대조를 통째로 꺼뜨린 형태다(`[f04].self_judged` ③).
    fn 임시(tag: &str) -> PathBuf {
        let p = std::env::temp_dir().join(format!("pal-cache-test-{tag}-{}", std::process::id()));
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
        let c = BlobCache::open(임시("기본")).unwrap();
        let k = 키(ObjectName::from_bytes([1; 20]), V);
        assert_eq!(c.lookup::<값>(&k).unwrap(), Lookup::Miss);
        c.put(&k, &값 { n: 7, s: "가".into() }).unwrap();
        assert_eq!(c.lookup::<값>(&k).unwrap(), Lookup::Hit(값 { n: 7, s: "가".into() }));
        let _ = fs::remove_dir_all(c.root());
    }

    #[test]
    fn 깨진_것은_미스가_아니라_사건이고_바이트가_남는다() {
        // **문서(§4)와 옛 코드가 반대였던 자리다.** 진행하되(재계산은 부르는 쪽이),
        // 조용하지 않고(`Corrupt`), **격리는 삭제가 아니다.**
        let c = BlobCache::open(임시("손상")).unwrap();
        let k = 키(ObjectName::from_bytes([9; 20]), V);
        c.put(&k, &값 { n: 1, s: "나".into() }).unwrap();
        let 자리 = c.path_of(&k);
        fs::write(&자리, "zstd 가 아니다").unwrap();

        let Lookup::Corrupt { quarantined, .. } = c.lookup::<값>(&k).unwrap() else {
            panic!("깨진 것을 미스나 적중으로 냈다");
        };
        assert!(quarantined.exists(), "격리한다며 지웠다");
        assert_eq!(fs::read(&quarantined).unwrap(), "zstd 가 아니다".as_bytes());
        assert!(!자리.exists(), "깨진 것이 캐시에 그대로 남았다");
        assert!(quarantined.starts_with(c.quarantine_dir()));

        // **★ 반대 방향** — 격리한 뒤의 조회는 **손상이 아니라 미스**다. 축출 뒤의
        // 미스와 같은 값이어야 한다. 아니면 한 번 깨진 키가 영원히 사건으로 남는다.
        assert_eq!(c.lookup::<값>(&k).unwrap(), Lookup::Miss);
        let _ = fs::remove_dir_all(c.root());
    }

    #[test]
    fn 없는_것은_손상이_아니다() {
        // ★ 반대 방향. 손상 계수기가 아무거나 세면 이 시험이 걸린다.
        let c = BlobCache::open(임시("부재")).unwrap();
        let k = 키(ObjectName::from_bytes([10; 20]), V);
        assert_eq!(c.lookup::<값>(&k).unwrap(), Lookup::Miss);
        assert!(!c.quarantine_dir().exists(), "아무것도 안 깨졌는데 격리 방이 생겼다");
        let _ = fs::remove_dir_all(c.root());
    }

    /// 엔트리 `n` 개를 채운다.
    fn 채움(c: &BlobCache, n: usize) {
        for i in 0..n {
            let mut b = [0u8; 20];
            b[0] = u8::try_from(i % 251).unwrap_or(0);
            b[1] = u8::try_from(i / 251).unwrap_or(0);
            c.put(&키(ObjectName::from_bytes(b), V), &값 { n: i, s: "가".repeat(64) }).unwrap();
        }
    }

    #[test]
    fn 예산이_넉넉하면_한_건도_안_지운다() {
        // **★ 반대 방향이다.** 늘 지우는 `prune` 은 아래 시험의 앞 절을 통과한다.
        let c = BlobCache::open(임시("넉넉")).unwrap();
        채움(&c, 20);
        let 전 = c.usage().unwrap();
        let r = c.evict_to(u64::MAX).unwrap();
        assert_eq!((r.removed, r.freed_bytes), (0, 0));
        assert_eq!(c.usage().unwrap(), 전, "안 지운다면서 무언가 움직였다");
        let _ = fs::remove_dir_all(c.root());
    }

    #[test]
    fn 축출은_실제로_파일을_줄이고_보고가_실물과_맞는다() {
        let c = BlobCache::open(임시("축출")).unwrap();
        채움(&c, 40);
        let 전 = c.usage().unwrap();
        assert_eq!(전.entries, 40, "채우기가 안 됐으면 이 시험은 아무것도 재지 않는다");

        let 예산 = 전.bytes / 4;
        let r = c.evict_to(예산).unwrap();
        let 후 = c.usage().unwrap();

        assert!(r.removed > 0, "예산을 1/4 로 줬는데 한 건도 안 지웠다");
        assert_eq!(후.entries, 전.entries - r.removed, "보고와 실제 파일 수가 다르다");
        assert_eq!(후.entries, r.kept_entries);
        assert_eq!(후.bytes, r.kept_bytes);
        assert!(후.bytes <= 예산, "지우고도 예산을 넘는다 — {} > {예산}", 후.bytes);
        let _ = fs::remove_dir_all(c.root());
    }

    #[test]
    fn 축출은_격리_방과_임시_파일을_건드리지_않는다() {
        // 격리를 예산 때문에 지우면 그것은 **유예된 삭제**이고, 도는 쓰기의 임시 파일을
        // 지우면 그 쓰기의 `rename` 이 깨진다.
        let c = BlobCache::open(임시("경계")).unwrap();
        채움(&c, 10);
        let k = 키(ObjectName::from_bytes([200; 20]), V);
        c.put(&k, &값 { n: 1, s: "나".into() }).unwrap();
        fs::write(c.path_of(&k), "깨진 바이트").unwrap();
        let Lookup::Corrupt { quarantined, .. } = c.lookup::<값>(&k).unwrap() else {
            panic!("격리가 안 일어났다");
        };
        let tmp = c.root().join("aa");
        fs::create_dir_all(&tmp).unwrap();
        let tmp = tmp.join("도는-쓰기.tmp");
        fs::write(&tmp, "쓰는 중").unwrap();

        // **0 예산이다.** 지울 수 있는 것은 전부 지우라는 뜻이다.
        let r = c.evict_to(0).unwrap();
        assert_eq!(c.usage().unwrap().entries, 0, "0 예산인데 엔트리가 남았다");
        assert!(r.removed >= 10);
        assert!(quarantined.exists(), "축출이 격리 방을 지웠다");
        assert!(tmp.exists(), "축출이 도는 쓰기의 임시 파일을 지웠다");
        let _ = fs::remove_dir_all(c.root());
    }

    #[test]
    fn 손상은_빗나감과_따로_세되_총계에는_들어간다() {
        let mut s = CacheStats::default();
        s.hit();
        s.corrupt();
        assert_eq!((s.hits, s.misses, s.corrupt, s.total()), (1, 1, 1, 2));
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
