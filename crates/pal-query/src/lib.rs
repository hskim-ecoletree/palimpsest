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
    Budget, Capable, CapabilitySet, Coverage, Elision, Envelope, ExtractGrade, Fold, FoldedPart,
    IdentityGrade, LedgerRef, LogStatus, NotRecorded, ProjectionFreshness, QueryLogEntry, QueryName, RepoPath,
    Slot, Snapshot, Step, SymbolId, SymbolNode, traverse,
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
        }
    }

    /// 인자를 한 줄로 — 로그의 요약이 이것을 먹는다.
    #[must_use]
    pub fn args(&self) -> &str {
        match self {
            Self::LedgerSnapshot | Self::GraphDump => "",
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
