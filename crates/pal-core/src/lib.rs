//! 도메인 타입·불변식.
//!
//! **이 크레이트는 워크스페이스 내 어떤 크레이트에도 의존하지 않는다**(stack §4.1).
//! 파서·저장 기술이 좌표계에 새는 것을 막는다 — `tree-sitter`·`redb`·`gix` 도 여기 없다.
//!
//! S0 시점의 범위는 좁다. `Coord`·`Envelope` 는 아직 없다 —
//! 각각 F01·F03 과 F05 의 것이고, 계획 §4 가 그 검사를 그 기능에 배정해 뒀다.
//! **없는 것을 미리 흉내 내지 않는다.**

#![forbid(unsafe_code)]

mod capable;
mod language;
mod symbol;
mod version;

pub use capable::{Capable, CapabilityId};
pub use language::Language;
pub use symbol::{Span, Symbol, SymbolKind};
pub use version::ExtractorVersion;
