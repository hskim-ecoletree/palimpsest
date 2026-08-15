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
mod catalog;
mod cascade;
mod chain;
mod coord;
mod derived;
mod entity;
mod doctor;
mod envelope;
mod file_graph;
mod glob;
mod graph;
mod judgment;
mod language;
mod ledger;
mod manifest;
mod narrative;
mod projection;
mod query_log;
mod radius;
mod rebind;
mod repo;
mod schema;
mod scope;
mod slot;
mod symbol;
mod touch;
mod traverse;
mod version;
mod view;

pub use binding::{
    Binding, BindingId, BindingReport, BindingStatus, BoundTime, CodeFreshness,
    DetectorReport, Lineage, NewBinding, Now, PromotedBy, PromotionSite,
    UndeterminableReason, WatchEntry,
};
// **예산 상수는 여기 하나에서 나간다** — stack §5.5 · `[f05.1.pass]` ①.
// 다른 모듈이 같은 이름을 재수출하면 *"한 곳"* 이 두 곳이 된다.
pub use budget::{
    Budget, CANDIDATE_LIMIT, CORRUPT_NOTES, DEFAULT_CACHE_BUDGET_BYTES, EXTRACT_CHUNK, MARKER_SCAN_BYTES,
    OVERSIZE_BYTES, PROVISIONAL_BYTES_PER_TOKEN, PROVISIONAL_CASCADE_DEPTH,
    PROVISIONAL_ERROR_RATIO_PERCENT,
    PROVISIONAL_HISTORY_BUDGET, PROVISIONAL_PATH_PRODUCT_MAX, PROVISIONAL_SAMPLE_MAX,
    PROVISIONAL_QUARANTINE_BUDGET_BYTES, PROVISIONAL_STITCH_BATCH,
    PROVISIONAL_STRAY_TMP_MAX_AGE_SECS, PROVISIONAL_TRAVERSAL_DEPTH,
    PROVISIONAL_VIEW_NODE_MAX, PROVISIONAL_WATCH_PRODUCT_MAX, SHEBANG_SCAN_BYTES,
};
pub use capable::{Capable, CapabilityId, Declared};
pub use catalog::{ArgDecl, CatalogError, QueryCatalog, QueryDecl};
pub use cascade::{Cascade, NodeFreshness, cascade};
pub use chain::{
    Actor, ActorId, Change, ChangeId, ChangeKind, Confidence, Defect, Introduction, Journey,
    NotFoundReason, Retrobinding, RetrobindingSummary, Uncapturable,
};
pub use coord::{BodyDigest, Coord, Discriminator, ExportDigest, SymbolId, SymbolIdentity};
pub use doctor::{
    Absence, BINDING_INDEX_KIND, DERIVED_KIND, Diagnosis, DoctorScope, InvariantId,
    InvariantOutcome, InvariantReport, Outcome, RESIDUAL_KIND, SCOPE_REDUCTION_KIND, Violation,
    run as doctor,
};
pub use derived::{DerivedId, NodeRef, ReproInput};
pub use attributes::{Attributes, FileAttributes};
pub use file_graph::{
    Containment, ExportSet, FileGraph, ImportSet, LocalIx, RecoveryKind, RecoverySite,
};
pub use glob::{Glob, GlobError};
pub use graph::{AssertedVia, Producer, Provenance, ResolutionGrade};
pub use envelope::{
    BudgetName, CapabilitySet, Coverage, Elision, ElisionReason, Envelope, Fold, Folded,
    FoldedPart, LedgerRef, LimitHit, LogStatus, NotRecorded, ProjectionFreshness,
    RebuildState, TokenEstimate, Truncation,
};
pub use judgment::{Residual, ResidualReason};
pub use manifest::{ExclusionRule, Manifest, ManifestError, RepoDecl, ScopeSource};
pub use narrative::{
    ClaimDistance, Classification, ConfirmingSignal, Coordinates, Fragment, NamedCoord,
    PromotionRefusal, Proposal, RawSignals, Refusal, ResolutionSignal, resolve,
};
pub use language::Language;
pub use ledger::{
    Bucket, BinaryReason, DetectorFreshness, ExclusionRuleId, ExtractGrade, FileState,
    GeneratedEvidence, IdentityGrade, LanguageCapability, LanguageId, Ledger, LedgerEntry,
    UnsupportedReason,
};
pub use schema::{
    AttrDecl, Cardinality, Carried, Carrier, EdgeDecl, EvidenceRule, GradeRule, GraphSchema, NodeDecl,
    NodeStatus, Requirement, SchemaError,
};
pub use projection::{FileNode, FileRow, RefCounts, ReferenceEdge, file_edges};
pub use query_log::{QueryLogEntry, QueryName};
pub use entity::{EntityId, EntityKind, EntityOrigin, EntityRegistry, Ulid};
pub use radius::{BudgetRefusal, Neighborhood, Radius, check_budget, expand};
pub use rebind::{
    BatchRefusal, MatchSignals, RebindBatch, RebindProposal, approve_batch, propose,
    propose_with_shape,
};
pub use repo::{Digest, ObjectName, RepoAlias, RepoId, RepoPath, Snapshot, TreeRef};
pub use scope::{
    BoundSymbol, LocalRef, Namespace, RefResolution, Scope, ScopeBinding, ScopeChain, ScopeIx,
    ScopeKind, ScopeParent,
};
pub use slot::{ShellMismatch, Slot};
pub use symbol::{Span, Symbol, SymbolKind};
pub use traverse::{Step, traverse};
pub use touch::{
    BoundItem, EffectSet, JudgmentSummary, SymbolFacts, SymbolNode, TouchAnswer, TouchResult,
    UnresolvedRef,
};
pub use version::ExtractorVersion;
pub use view::{
    Anchor, BindingIndexEntry, EdgeInstance, EdgeTarget, GraphView, NodeInstance, NodeKey,
    ViewCoverage,
};
