//! tree-sitter 추출기.
//!
//! **쿼리 파일은 CLI 레퍼런스와 공유한다**(`queries/kotlin/top-level.scm`).
//! 대조(`corpus/criteria.toml` `[s0]`)가 *코드 경로*만의 차이가 되려면 그래야 한다 —
//! 한쪽만 고치면 그것은 대조를 사후 조정하는 일이다.

#![forbid(unsafe_code)]

mod kotlin;

use pal_core::{Capable, CapabilityId, ExtractorVersion, Language, Symbol};

pub use kotlin::ExtractError;

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
    match language {
        Language::Kotlin => Capable::Present(kotlin::extract(source)),
        Language::Java => Capable::not_built(CapabilityId::new("F02", "java-extraction")),
        Language::JavaScript => Capable::not_built(CapabilityId::new("F02", "javascript-extraction")),
        Language::TypeScript => Capable::not_built(CapabilityId::new("F02", "typescript-extraction")),
    }
}
