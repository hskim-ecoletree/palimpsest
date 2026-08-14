//! 질의 실행기 — **모든 답이 봉투를 지고 나간다** (F05 §5).
//!
//! > 실행기 진입점이 `Envelope` 만 반환한다 → 벌거벗은 답을 낼 방법이 없다.
//!
//! # 이 크레이트가 존재하는 이유
//!
//! [`pal_store::Projection`] 은 *"이 좌표의 심볼"* 에 답한다. 그것은 조회이고 질의가
//! 아니다. **질의는 자기가 무엇을 못 봤는지와 무엇을 잘랐는지를 함께 낸다** —
//! 그 조립이 여기 있다.
//!
//! # 여기 없는 것
//!
//! **파일 경계를 넘는 해소가 없다**(F05 §0). `symbol.reaches` 가 걷는 엣지는 파일 **안**
//! 의 것뿐이고, 파일 밖으로 나간 참조는 `coverage.unresolved` 에 **수로** 실린다.
//! 빈 답이 아니라 **등급이 낮은 답**이고, 그것이 이 제품의 정상 상태다.
//!
//! # 후보 엣지가 이 빌드에 없다 — **그러므로 K·B 는 모집단이 0 이다**
//!
//! 파일 안 해소는 스코프 체인이 유일하게 풀 때만 엣지를 낸다
//! ([`pal_core::ResolutionGrade::Scoped`]). 후보 집합이 없으므로
//! [`pal_core::ElisionReason::CandidateOverflow`] 와
//! [`pal_core::ElisionReason::PathProductExceeded`] 는 **이 빌드에서 일어날 수 없다.**
//! 규칙은 서 있고 시험되지만(`pal_core::traverse` 의 단위 시험) **실물 모집단이 0** 이고,
//! [ADR-0002](../../../docs/adr/0002-empty-population-is-not-zero-violations.md) 그대로
//! 그것을 *"절단 없음"* 으로 세지 않는다. 후보 엣지를 만드는 것은 F07 이다.

#![forbid(unsafe_code)]

use std::collections::BTreeSet;
use std::time::Instant;

use pal_core::{
    Binding, BindingReport, BindingStatus, Budget, Capable, CapabilitySet, CodeFreshness, Coverage,
    DetectorReport,
    Elision, Envelope, ExtractGrade, Fold, FoldedPart, IdentityGrade, LedgerRef, Lineage, LogStatus,
    Now, NotRecorded, ProjectionFreshness, QueryLogEntry, QueryName, RepoPath, Slot, Snapshot, Step,
    SymbolId, SymbolNode, UndeterminableReason, traverse,
};
use pal_store::{Projection, ProjectionError};
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum QueryError {
    #[error("2층을 읽지 못했다: {0}")]
    Projection(#[from] ProjectionError),
}

/// 이 빌드가 답하는 질의 하나 — **이름과 인자.**
///
/// 열린 문자열이 아니다. 오타가 새 질의가 되면 F17 이 로그를 셀 때 그것을 질의로 센다.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NamedQuery {
    /// 이 스냅샷의 관측 범위 대장.
    LedgerSnapshot,
    /// 이름 하나 → 후보 심볼들. **여럿인 것이 정상이다.**
    SymbolResolve { name: String },
    /// 이 심볼이 담는 것들 — 컨테이너 체인으로.
    SymbolContains { name: String },
    /// 이 심볼을 가리키는 것들 — 1홉 역방향.
    SymbolCallers { name: String },
    /// 이 심볼에서 닿는 것들 — **예산 절단이 있는 BFS.**
    SymbolReaches { name: String },
    /// 노드와 엣지 전부 — 바깥 오라클이 읽는 창.
    GraphDump,
    /// 결박마다 상태 + **반경** + 무엇이 켰는가.
    BindingStatus,
}

impl NamedQuery {
    #[must_use]
    pub const fn name(&self) -> QueryName {
        match self {
            Self::LedgerSnapshot => QueryName::LedgerSnapshot,
            Self::SymbolResolve { .. } => QueryName::SymbolResolve,
            Self::SymbolContains { .. } => QueryName::SymbolContains,
            Self::SymbolCallers { .. } => QueryName::SymbolCallers,
            Self::SymbolReaches { .. } => QueryName::SymbolReaches,
            Self::GraphDump => QueryName::GraphDump,
            Self::BindingStatus => QueryName::BindingStatus,
        }
    }

    /// 인자를 한 줄로 — 로그의 요약이 이것을 먹는다.
    #[must_use]
    pub fn args(&self) -> &str {
        match self {
            Self::LedgerSnapshot | Self::GraphDump | Self::BindingStatus => "",
            Self::SymbolResolve { name }
            | Self::SymbolContains { name }
            | Self::SymbolCallers { name }
            | Self::SymbolReaches { name } => name,
        }
    }

    /// 이름과 인자로 만든다. **아는 이름이 아니면 `None`.**
    #[must_use]
    pub fn parse(name: &str, arg: Option<&str>) -> Option<Self> {
        let named = |f: fn(String) -> Self| arg.map(|a| f(a.to_owned()));
        match QueryName::parse(name)? {
            QueryName::LedgerSnapshot => Some(Self::LedgerSnapshot),
            QueryName::GraphDump => Some(Self::GraphDump),
            QueryName::BindingStatus => Some(Self::BindingStatus),
            QueryName::SymbolResolve => named(|name| Self::SymbolResolve { name }),
            QueryName::SymbolContains => named(|name| Self::SymbolContains { name }),
            QueryName::SymbolCallers => named(|name| Self::SymbolCallers { name }),
            QueryName::SymbolReaches => named(|name| Self::SymbolReaches { name }),
        }
    }
}

/// 엣지 하나 — 산출에 실리는 형태.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DumpEdge {
    pub from: SymbolId,
    pub to: SymbolId,
}

/// 질의 하나의 답.
///
/// **`Unknown` 과 `Ambiguous` 가 변형인 이유**: 빈 목록으로 답하면 *"없다"* 와
/// *"하나로 못 좁혔다"* 가 같은 출력이 된다. 후자에서 하나를 고르면 그것이 조용한 오답이다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case", tag = "outcome")]
pub enum QueryResult {
    Ledger { ledger: LedgerRef },
    Symbols { symbols: Vec<SymbolNode> },
    Reached { start: SymbolId, symbols: Vec<SymbolNode> },
    Graph { nodes: Vec<SymbolNode>, edges: Vec<DumpEdge> },
    /// 결박마다 한 줄. **빈 목록이 정직한 답이다** — 능력이 있고 값이 없는 것이다.
    ///
    /// `detector` 는 **낡음을 재는 자의 낡음**이다(F09 §5). 안 실으면 낡은 감지기가 낸
    /// `Live` 가 지금의 `Live` 로 읽힌다 — 그것이 *"감지기가 낡는다"* 의 실패 형태다.
    Bindings { bindings: Vec<BindingReport>, detector: DetectorReport },
    /// 이름이 여럿으로 해소됐다. **하나를 고르지 않는다.**
    Ambiguous { name: String, candidates: Vec<SymbolNode> },
    /// 이 스냅샷에서 못 찾았다. **없다는 뜻이 아니다** — 근거는 봉투가 진다.
    Unknown { name: String },
}

/// 질의 하나가 서는 바닥.
///
/// **봉투의 성분을 부르는 쪽이 지고 온다.** 대장을 만드는 것은 표면이고(`pal-cli`),
/// 이 크레이트가 그것을 다시 계산하면 같은 사실이 두 곳에서 계산된다.
pub struct QueryCtx<'a> {
    pub projection: &'a Projection,
    pub snapshot: Snapshot,
    pub ledger: LedgerRef,
    pub freshness: ProjectionFreshness,
    pub capabilities: CapabilitySet,
    /// **넷을 손으로 넘겨야 만들 수 있다** — 끄는 손잡이가 없다(`[f05.1.pass]` ④).
    pub budget: Budget,
    pub out_of_scope_files: usize,
    /// 이 저장소의 결박 전부 — **부르는 쪽이 지고 온다.**
    ///
    /// # 왜 이 크레이트가 `pal-intent` 에 의존하지 않는가
    ///
    /// [R-21] 이 금한 것은 *"파생층의 폐기 경로가 의도에 닿는 것"* 이고
    /// `cargo xtask check` 는 `pal-store → pal-intent` 만 막는다. 여기는 읽기 경로라
    /// 그 규칙에 안 걸린다 — **그래도 안 붙인다.**
    ///
    /// 이 구조체의 머리가 이미 그 근거를 적었다: *"봉투의 성분을 부르는 쪽이 지고 온다.
    /// 대장을 만드는 것은 표면이고, 이 크레이트가 그것을 다시 계산하면 같은 사실이 두
    /// 곳에서 계산된다."* **결박도 같은 자격이다** — 표면이 이미 의도 저장소를 연다
    /// (`pal touch`). 여기서 또 열면 **한 명령이 같은 파일을 두 번 연다.**
    ///
    /// [R-21]: ../../../docs/plan/00-risks.md#r-21
    pub bindings: Vec<Binding>,
    /// **낡음을 재는 자의 낡음** — 대장에서 온다(F01). 표면이 지고 온다.
    pub detector: DetectorReport,
    /// 대장이 `Partial` 로 적은 파일들 — [`UndeterminableReason::PartialParse`] 의 입력.
    ///
    /// **이름으로 세지 않고 대장에서 뜬다.** 이름으로 세면 칸이 하나 늘 때 조용히 빠진다.
    pub partial_files: BTreeSet<RepoPath>,
}

/// 질의 하나를 돌린다. **반환 타입이 봉투뿐이다.**
///
/// # Errors
/// 2층을 읽지 못하면.
pub fn execute(q: &NamedQuery, ctx: &QueryCtx) -> Result<Envelope<QueryResult>, QueryError> {
    let started = Instant::now();
    let mut elision = Elision::none();
    let mut accessed: Vec<SymbolId> = Vec::new();

    let answer = run(q, ctx, &mut elision, &mut accessed)?;
    let coverage = coverage_of(ctx, &accessed)?;
    let fold = fold_of(&answer, &ctx.ledger);

    // **로그는 답보다 먼저 남는다** — 답을 못 낸 질의도 일어난 사건이다.
    // 그런데 절단과 걸린 시간은 답을 낸 뒤에야 안다. 그래서 여기다.
    //
    // ⚠ **읽기 전용으로 붙었으면 못 남긴다.** 조용히 건너뛰지 않는다 — F17 이 그
    // 공백을 「조회 안 됨」으로 세면 미조회를 **과대 계상**하고, 그것이 이 제품이
    // 고발하는 형태다(`[f06].readonly_and_the_query_log`).
    let log = if ctx.projection.is_read_only() {
        LogStatus::NotRecorded { why: NotRecorded::ReadOnlyAttach }
    } else {
        let entry = QueryLogEntry {
            query: q.name(),
            args_digest: QueryLogEntry::digest_of(q.args()),
            accessed: accessed.clone(),
            elision: elision.clone(),
            duration_micros: u64::try_from(started.elapsed().as_micros()).unwrap_or(u64::MAX),
        };
        ctx.projection.log_query(&ctx.snapshot.to_string(), &entry)?;
        LogStatus::Recorded
    };

    Ok(Envelope::new(
        answer,
        ctx.snapshot.clone(),
        ctx.freshness.clone(),
        coverage,
        ctx.capabilities.clone(),
        ctx.ledger.clone(),
        elision,
        fold,
        log,
    ))
}

/// 이 답에서 **부피가 다른 질의로 옮겨진** 자리 (F06 §4.3 · `[f06.2.pass]` ①).
///
/// # 접기는 이미 일어나고 있었다 — 없던 것은 그 사실의 기록이다
///
/// 모든 봉투가 [`LedgerRef`] 를 싣는데 그것은 대장 전체가 아니라 **요약 여섯 값**이다.
/// 즉 부피는 이미 옮겨져 있고, 옮겼다는 사실만 산출에 없었다. 그것이 이 함수가
/// 닫는 구멍이다.
///
/// **`ledger.snapshot` 만 안 접힌다** — 그 질의의 답이 대장 자신이기 때문이다.
/// 그 하나와 나머지 다섯이 다른 것이 이 값이 무언가를 재고 있다는 증거다.
fn fold_of(answer: &QueryResult, ledger: &LedgerRef) -> Fold {
    let mut fold = Fold::none();
    if !matches!(answer, QueryResult::Ledger { .. }) {
        fold.push(FoldedPart::Ledger, ledger.files_total, QueryName::LedgerSnapshot);
    }
    fold
}

fn run(
    q: &NamedQuery,
    ctx: &QueryCtx,
    elision: &mut Elision,
    accessed: &mut Vec<SymbolId>,
) -> Result<QueryResult, QueryError> {
    let p = ctx.projection;
    match q {
        NamedQuery::LedgerSnapshot => Ok(QueryResult::Ledger { ledger: ctx.ledger.clone() }),
        NamedQuery::BindingStatus => Ok(QueryResult::Bindings {
            bindings: binding_reports(ctx, accessed),
            detector: ctx.detector.clone(),
        }),
        NamedQuery::GraphDump => {
            let (nodes, edges) = p.dump()?;
            accessed.extend(nodes.iter().map(|n| n.id));
            Ok(QueryResult::Graph {
                nodes,
                edges: edges.into_iter().map(|(from, to)| DumpEdge { from, to }).collect(),
            })
        }
        NamedQuery::SymbolResolve { name } => {
            let symbols = p.resolve_name(name)?;
            accessed.extend(symbols.iter().map(|s| s.id));
            if symbols.is_empty() {
                return Ok(QueryResult::Unknown { name: name.clone() });
            }
            Ok(QueryResult::Symbols { symbols })
        }
        NamedQuery::SymbolContains { name } => {
            let start = match unique(p, name, accessed)? {
                Ok(s) => s,
                Err(other) => return Ok(other),
            };
            // 같은 파일의 심볼 중 **컨테이너 체인이 이 심볼 아래인 것**.
            let mut out: Vec<SymbolNode> = p
                .symbols_of(&start.path)?
                .into_iter()
                .filter(|s| under(&start, s))
                .collect();
            out.sort_by_key(|s| s.span.byte_start);
            accessed.extend(out.iter().map(|s| s.id));
            Ok(QueryResult::Symbols { symbols: out })
        }
        NamedQuery::SymbolCallers { name } => {
            let start = match unique(p, name, accessed)? {
                Ok(s) => s,
                Err(other) => return Ok(other),
            };
            let mut out = Vec::new();
            for id in p.callers(start.id)? {
                if let Some(n) = p.symbol(id)? {
                    out.push(n);
                }
            }
            out.sort_by(|a, b| a.path.cmp(&b.path).then(a.span.byte_start.cmp(&b.span.byte_start)));
            accessed.extend(out.iter().map(|s| s.id));
            Ok(QueryResult::Symbols { symbols: out })
        }
        NamedQuery::SymbolReaches { name } => {
            let start = match unique(p, name, accessed)? {
                Ok(s) => s,
                Err(other) => return Ok(other),
            };
            // **예산을 들고 걷는다.** 자른 것은 `elision` 에 쌓인다.
            let reached = traverse(&start.id, &ctx.budget, elision, |id| {
                p.callees(*id).unwrap_or_default().into_iter().map(Step::exact).collect()
            });
            let mut out = Vec::new();
            for id in &reached {
                if let Some(n) = p.symbol(*id)? {
                    out.push(n);
                }
            }
            accessed.extend(out.iter().map(|s| s.id));
            Ok(QueryResult::Reached { start: start.id, symbols: out })
        }
    }
}

/// 이름 하나를 **유일하게** 해소한다.
///
/// 여럿이면 [`QueryResult::Ambiguous`], 없으면 [`QueryResult::Unknown`] 이다 —
/// **하나를 고르지 않는다.** 고르면 그것이 조용한 오답이고, 그 순간 답을 받은 쪽은
/// 자기가 무엇을 보고 있는지 모른다.
fn unique(
    p: &Projection,
    name: &str,
    accessed: &mut Vec<SymbolId>,
) -> Result<Result<SymbolNode, QueryResult>, QueryError> {
    let mut found = p.resolve_name(name)?;
    accessed.extend(found.iter().map(|s| s.id));
    match found.len() {
        0 => Ok(Err(QueryResult::Unknown { name: name.to_owned() })),
        1 => Ok(Ok(found.remove(0))),
        _ => Ok(Err(QueryResult::Ambiguous { name: name.to_owned(), candidates: found })),
    }
}

/// `s` 가 `parent` 안인가 — 컨테이너 체인으로.
fn under(parent: &SymbolNode, s: &SymbolNode) -> bool {
    if s.id == parent.id {
        return false;
    }
    let mut want = parent.container.clone();
    want.push(parent.name.clone());
    s.container.starts_with(&want)
}

/// 이 답이 **무엇을 못 봤는가** — 만진 좌표가 사는 파일들에서 온다.
///
/// # 질의마다 다른 값이어야 한다 (`[f05.3.pass]` ⑤)
///
/// 저장소 전체의 미해소 수를 복사하면 그것은 답의 성질이 아니라 저장소의 성질이다.
/// 서로 다른 두 질의가 같은 `coverage` 를 내면 그 숫자는 아무것도 안 말한다.
///
/// **스코프 체인이 없는 파일은 셀 수 없다** — 그 사실은 파일 노드가 [`Capable`] 로
/// 지고 있고, 여기서는 **셀 수 있는 것만 더한다.**
fn coverage_of(ctx: &QueryCtx, accessed: &[SymbolId]) -> Result<Coverage, QueryError> {
    let p = ctx.projection;
    let mut paths: BTreeSet<RepoPath> = BTreeSet::new();
    let mut identity = IdentityGrade::Exact;
    for id in accessed {
        if let Some(n) = p.symbol(*id)? {
            identity = identity.min(n.identity);
            paths.insert(n.path);
        }
    }

    let mut unresolved = 0usize;
    let mut lowest = None;
    for path in &paths {
        let Some(f) = p.file(path)? else { continue };
        lowest = Some(lowest.map_or(f.grade, |g: ExtractGrade| g.min(f.grade)));
        if let Slot::Built(c) = f.refs {
            unresolved += c.unresolved;
        }
    }

    Ok(Coverage {
        unresolved,
        out_of_scope_files: ctx.out_of_scope_files,
        // **닿은 파일이 없으면 `L0` 이다** — 아무것도 안 봤다는 뜻이고, 그것이 정확하다.
        lowest_grade: lowest.unwrap_or(ExtractGrade::L0),
        // 아무 심볼도 안 만졌으면 이 답이 선 정체성은 가장 낮은 것이다.
        identity: if accessed.is_empty() { IdentityGrade::Ordinal } else { identity },
    })
}

/// 이 빌드가 답하는 것과 아직 못 만든 것 — **응답마다 실린다**(stack §5.3).
#[must_use]
pub fn capabilities() -> CapabilitySet {
    CapabilitySet::new(
        QueryName::ALL.iter().map(|q| q.name().to_owned()).collect(),
        vec![
            pal_core::CapabilityId::new("F07", "cross-file-resolution"),
            pal_core::CapabilityId::new("F08", "unresolved-refs"),
            pal_core::CapabilityId::new("F13", "effects"),
            pal_core::CapabilityId::new("F15", "judgment"),
        ],
    )
}

/// 이 빌드가 재구축 진행을 관측할 수 있는가 — **값이다.**
#[must_use]
pub fn freshness(
    matches_worktree: Capable<bool>,
    rebuilding: bool,
    built_for_this_snapshot: bool,
    symbols_indexed: usize,
) -> ProjectionFreshness {
    ProjectionFreshness {
        matches_worktree,
        rebuild: Capable::Present(if rebuilding {
            pal_core::RebuildState::Rebuilding
        } else {
            pal_core::RebuildState::Settled
        }),
        built_for_this_snapshot,
        symbols_indexed,
    }
}

/// 결박마다 산출 한 줄 — `binding.status` 의 몸통.
///
/// # 판정 불가가 이 함수의 절반이다 (F09 §2.1 · [R16])
///
/// 조회가 [`Now`] 를 낸다. `Option<BodyDigest>` 였으면 *"사라졌다"* 와 *"비교할 수
/// 없다"* 가 같은 값이 되고, **그 구별이 이 기능의 전부다.**
///
/// | 사유 | 여기서 어떻게 아나 |
/// |---|---|
/// | `ProjectionStale` | 2층이 이 스냅샷 것이 아니다 — **감시 집합을 보기도 전이다** |
/// | `IdentityGrade` | 감시 원소의 등급이 `Unavailable`(L0) — 요약 자체가 없다 |
/// | `PartialParse` | 그 원소가 사는 파일이 대장에서 `Partial` 이다 |
/// | `WatchMemberGone` | 조회가 비었는데 **대상은 살아 있다** — `evaluate` 가 가른다 |
///
/// # `ordinal` 은 여기 없다 — **접지 않고 대신 싣는다**
///
/// `ordinal` 좌표는 **비교가 가능하지만 약하다.** 판정 불가로 접으면 Kotlin 코퍼스가
/// 통째로 판정 불가가 되고 그것이 *"지배하면 정직하지만 쓸모없다"* 다.
/// 그래서 [`BindingReport::watch_grades`] 가 등급 분포를 산출에 싣는다 —
/// 반경을 산출에 싣는 것과 **정확히 같은 자리**다(§3: 닫히지 않는 것을 선언으로 다룬다).
///
/// [R16]: ../../../docs/evidence-map.md
fn binding_reports(ctx: &QueryCtx, accessed: &mut Vec<SymbolId>) -> Vec<BindingReport> {
    let p = ctx.projection;
    let 이_스냅샷 = ctx.freshness.built_for_this_snapshot;

    let mut out = Vec::with_capacity(ctx.bindings.len());
    for b in &ctx.bindings {
        accessed.push(b.target);
        accessed.extend(b.watch.iter().map(|w| w.symbol));

        // **등급 분포는 상태와 무관하게 센다** — 판정 불가여도 *"어떤 좌표 위에 서
        // 있는가"* 는 알 수 있고, 그것이 이 값이 지도인 이유다.
        let mut grades: std::collections::BTreeMap<&'static str, usize> =
            std::collections::BTreeMap::new();
        for w in &b.watch {
            if let Ok(Some(n)) = p.symbol(w.symbol) {
                *grades.entry(n.identity.name()).or_insert(0) += 1;
            }
        }

        let status = if 이_스냅샷 {
            BindingStatus::evaluate(b, Lineage::Current, |id| match p.symbol(id) {
                Ok(Some(n)) if n.identity == IdentityGrade::Unavailable => {
                    Now::Undeterminable(UndeterminableReason::IdentityGrade)
                }
                Ok(Some(n)) if ctx.partial_files.contains(&n.path) => {
                    Now::Undeterminable(UndeterminableReason::PartialParse)
                }
                Ok(Some(n)) => Now::Digest(n.body),
                Ok(None) => Now::Gone,
                // **읽기 실패를 「사라졌다」로 적지 않는다.** 못 읽은 것과 없는 것은
                // 다른 사건이고, 뭉개면 저장 오류가 `Orphaned` 로 나가 사람이 코드를
                // 고치러 간다.
                Err(_) => Now::Undeterminable(UndeterminableReason::ProjectionStale),
            })
        } else {
            // **감시 집합을 보기도 전에 판정 불가다.** 여기서 요약을 대면 옛 세대의
            // 값과 지금의 결박을 대는 것이 된다.
            BindingStatus::projection_stale(Lineage::Current)
        };

        out.push(BindingReport {
            binding: b.id.clone(),
            subject: b.subject.to_display(),
            note: b.note.clone(),
            target: b.target,
            radius: b.radius.name(),
            watch: b.watch.len(),
            watch_grades: grades,
            status,
            bound_at: b.bound_at.clone(),
            bound_at_time: b.bound_at_time,
        });
    }
    // **결박 id 순.** 회차마다 순서가 달라지면 사람이 보는 목록이 흔들리고,
    // 흔들리는 목록은 행동의 근거가 못 된다.
    out.sort_by(|a, b| a.binding.cmp(&b.binding));
    out
}

/// 이 답에서 낡음이 켜진 결박의 수 — **화면과 종료 코드가 함께 쓴다.**
#[must_use]
pub fn stale_count(r: &QueryResult) -> usize {
    match r {
        QueryResult::Bindings { bindings, .. } => bindings
            .iter()
            .filter(|b| matches!(b.status.code, CodeFreshness::Stale { .. }))
            .count(),
        _ => 0,
    }
}
