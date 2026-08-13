//! tree-sitter 추출기.
//!
//! **쿼리 파일은 CLI 레퍼런스와 공유한다**(`queries/kotlin/top-level.scm`).
//! 대조(`corpus/criteria.toml` `[s0]`)가 *코드 경로*만의 차이가 되려면 그래야 한다 —
//! 한쪽만 고치면 그것은 대조를 사후 조정하는 일이다.

#![forbid(unsafe_code)]

mod classify;
mod extractor;
mod kotlin;
mod parse;
mod recognize;

use pal_core::{Capable, ExtractorVersion, Language, Symbol};

pub use classify::{FileOutcome, OVERSIZE_BYTES, classify, grade_of};
pub use extractor::{LanguageExtractor, extractor_for};
pub use kotlin::KotlinExtractor;
pub use parse::ExtractError;
pub use recognize::{Recognition, recognize};

/// 판정용 문법의 고정 커밋 — `corpus/criteria.toml` `[s0.grammar].judging`.
pub const GRAMMAR_REV: &str = "3dea6dfa9c0129deb7c4315afbda806c85c41667";

/// 추출기 코드 버전. 문법과 **다른 축이다**(stack §5.1).
///
/// # 이 값을 올리는 것이 곧 1층 캐시 전량 무효화다
///
/// S2 에서 `Symbol` 에 `body_digest` 가 붙어 추출 산출이 바뀌었다. 값을 올리지 않았더니
/// **옛 캐시를 새 스키마로 읽으려다 실패했다** — 캐시 키에 이 값을 넣은 이유가 그
/// 자리에서 관측된 것이다. 올리면 키가 달라지므로 옛 항목은 조회되지 않고 조용히
/// 남았다가 `prune`(F04)이 걷어간다.
///
/// **문법 rev 는 그대로다.** 축이 둘인 이유가 이것이다 — 추출기 코드가 바뀌었다고
/// 문법이 바뀐 것은 아니다.
pub const EXTRACTOR_REV: &str = "s2";

#[must_use]
pub const fn version() -> ExtractorVersion {
    ExtractorVersion { grammar: GRAMMAR_REV, extractor: EXTRACTOR_REV }
}

/// 이 빌드가 그 언어를 추출할 수 있으면 시도한다.
///
/// **빌드되지 않은 언어에서 빈 `Vec` 을 돌려주지 않는다** — 그것이 거짓 안전이다.
/// 자리는 [`Capable`] 이 잡는다.
///
/// # Errors
/// 언어가 빌드되어 있고 파싱·쿼리가 실패하면 [`ExtractError`].
#[must_use]
pub fn extract(language: Language, source: &[u8]) -> Capable<Result<Vec<Symbol>, ExtractError>> {
    match extractor_for(language) {
        Capable::Present(e) => Capable::Present(e.extract(source).map(|g| g.symbols)),
        Capable::NotBuilt { capability } => Capable::NotBuilt { capability },
    }
}

/// 이 빌드가 그 언어를 추출할 수 있는가 — **소스 없이 묻는다.**
///
/// 대장은 파일을 읽기 전에 이것을 알아야 한다. [`extract`] 와 **같은 표를 본다** —
/// 둘이 갈리면 `pal symbols` 가 답하는 언어와 대장이 `parsed` 로 세는 언어가 달라진다.
/// 그 표가 [`extractor_for`] 이고, 셋이 전부 그것 하나를 탄다.
#[must_use]
pub fn capability(language: Language) -> Capable<()> {
    match extractor_for(language) {
        Capable::Present(_) => Capable::Present(()),
        Capable::NotBuilt { capability } => Capable::NotBuilt { capability },
    }
}
