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

mod attributes;
mod binding;
mod budget;
mod capable;
mod cascade;
mod chain;
mod coord;
mod derived;
mod doctor;
mod envelope;
mod file_graph;
mod glob;
mod graph;
mod judgment;
mod language;
mod ledger;
mod manifest;
mod repo;
mod schema;
mod scope;
mod symbol;
mod touch;
mod version;
mod view;

pub use binding::{
    Binding, BindingId, BindingStatus, CodeFreshness, Lineage, WatchEntry,
};
pub use budget::PROVISIONAL_ERROR_RATIO_PERCENT;
pub use capable::{Capable, CapabilityId};
pub use cascade::{Cascade, NodeFreshness, PROVISIONAL_CASCADE_DEPTH, cascade};
pub use chain::{
    Actor, ActorId, Change, ChangeId, ChangeKind, Confidence, Defect, Introduction, Journey,
    NotFoundReason, Retrobinding, RetrobindingSummary, Uncapturable,
};
pub use coord::{BodyDigest, Coord, Discriminator, ExportDigest, SymbolId, SymbolIdentity};
pub use doctor::{
    Absence, BINDING_INDEX_KIND, CANDIDATE_LIMIT, DERIVED_KIND, Diagnosis, DoctorScope,
    InvariantId, InvariantOutcome, InvariantReport, Outcome, PROVISIONAL_SAMPLE_MAX,
    RESIDUAL_KIND, SCOPE_REDUCTION_KIND, Violation, run as doctor,
};
pub use derived::{DerivedId, NodeRef, ReproInput};
pub use attributes::{Attributes, FileAttributes};
pub use file_graph::{
    Containment, ExportSet, FileGraph, ImportSet, LocalIx, RecoveryKind, RecoverySite,
};
pub use glob::{Glob, GlobError};
pub use graph::{AssertedVia, Producer, Provenance, ResolutionGrade};
pub use envelope::{
    CapabilitySet, Coverage, Elision, Envelope, LedgerRef, ProjectionFreshness, RebuildState,
};
pub use judgment::{Residual, ResidualReason};
pub use manifest::{ExclusionRule, Manifest, ManifestError, RepoDecl, ScopeSource};
pub use language::Language;
pub use ledger::{
    Bucket, BinaryReason, DetectorFreshness, ExclusionRuleId, ExtractGrade, FileState,
    GeneratedEvidence, IdentityGrade, LanguageCapability, LanguageId, Ledger, LedgerEntry,
    UnsupportedReason,
};
pub use schema::{
    AttrDecl, Cardinality, Carrier, EdgeDecl, EvidenceRule, GradeRule, GraphSchema, NodeDecl,
    NodeStatus, Requirement, SchemaError,
};
pub use repo::{Digest, ObjectName, RepoId, RepoPath, Snapshot, TreeRef};
pub use scope::{
    BoundSymbol, LocalRef, Namespace, RefResolution, Scope, ScopeBinding, ScopeChain, ScopeIx,
    ScopeKind, ScopeParent,
};
pub use symbol::{Span, Symbol, SymbolKind};
pub use touch::{
    BoundItem, EffectSet, JudgmentSummary, SymbolFacts, SymbolNode, TouchAnswer, TouchResult,
    UnresolvedRef,
};
pub use version::ExtractorVersion;
pub use view::{
    Anchor, BindingIndexEntry, EdgeInstance, EdgeTarget, GraphView, NodeInstance, NodeKey,
    ViewCoverage,
};
