//! tree-sitter 추출기.
//!
//! **쿼리 파일은 CLI 레퍼런스와 공유한다**(`queries/kotlin/top-level.scm`).
//! 대조(`corpus/criteria.toml` `[s0]`)가 *코드 경로*만의 차이가 되려면 그래야 한다 —
//! 한쪽만 고치면 그것은 대조를 사후 조정하는 일이다.

#![forbid(unsafe_code)]

mod classify;
mod kotlin;
mod recognize;

use pal_core::{Capable, CapabilityId, ExtractorVersion, Language, Symbol};

pub use classify::{FileOutcome, OVERSIZE_BYTES, classify, grade_of};
pub use kotlin::{ExtractError, Extraction, extract_detailed};
pub use recognize::{Recognition, recognize};

/// 판정용 문법의 고정 커밋 — `corpus/criteria.toml` `[s0.grammar].judging`.
pub const GRAMMAR_REV: &str = "3dea6dfa9c0129deb7c4315afbda806c85c41667";

/// 추출기 코드 버전. 문법과 **다른 축이다**(stack §5.1).
pub const EXTRACTOR_REV: &str = "s0";

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
    match capability(language) {
        Capable::Present(()) => Capable::Present(kotlin::extract(source)),
        Capable::NotBuilt { capability } => Capable::NotBuilt { capability },
    }
}

/// 이 빌드가 그 언어를 추출할 수 있는가 — **소스 없이 묻는다.**
///
/// 대장은 파일을 읽기 전에 이것을 알아야 한다. [`extract`] 와 **같은 표를 본다** —
/// 둘이 갈리면 `pal symbols` 가 답하는 언어와 대장이 `parsed` 로 세는 언어가 달라진다.
#[must_use]
pub const fn capability(language: Language) -> Capable<()> {
    match language {
        Language::Kotlin => Capable::Present(()),
        Language::Java => Capable::not_built(CapabilityId::new("F02", "java-extraction")),
        Language::JavaScript => Capable::not_built(CapabilityId::new("F02", "javascript-extraction")),
        Language::TypeScript => Capable::not_built(CapabilityId::new("F02", "typescript-extraction")),
    }
}
