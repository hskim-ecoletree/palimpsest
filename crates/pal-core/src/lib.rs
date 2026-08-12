//! 도메인 타입·불변식.
//!
//! **이 크레이트는 워크스페이스 내 어떤 크레이트에도 의존하지 않는다**(stack §4.1).
//! 파서·저장 기술이 좌표계에 새는 것을 막는다 — `tree-sitter`·`redb`·`gix` 도 여기 없다.
//!
//! S1 시점의 범위는 여전히 좁다. `Envelope` 는 아직 없고(F05), `Coord` 도 없다 —
//! 좌표의 네 성분 중 `symbol` 을 채우려면 F03 의 정규화가 필요하고 S1 은 파일 단위에서
//! 닫힌다. 있는 것은 그 앞의 셋(`RepoId`·`TreeRef`·`Snapshot`)과 대장이다.
//! **없는 것을 미리 흉내 내지 않는다.**

#![forbid(unsafe_code)]

mod capable;
mod language;
mod ledger;
mod repo;
mod symbol;
mod version;

pub use capable::{Capable, CapabilityId};
pub use language::Language;
pub use ledger::{
    Bucket, BinaryReason, ExclusionRuleId, ExtractGrade, FileState, GeneratedEvidence,
    IdentityGrade, LanguageCapability, LanguageId, Ledger, LedgerEntry,
};
pub use repo::{Digest, ObjectName, RepoId, RepoPath, Snapshot, TreeRef};
pub use symbol::{Span, Symbol, SymbolKind};
pub use version::ExtractorVersion;
