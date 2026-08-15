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
    Binding, BindingReport, BindingStatus, BoundItem, BoundTarget, Budget, Capable, CapabilitySet,
    CodeFreshness, Coverage, DetectorReport, SymbolFacts, TargetPlace,
    Elision, ElisionReason, Envelope, ExtractGrade, Fold, FoldedPart, IdentityGrade, LedgerRef,
    Lineage, LogStatus,
    Now, NotRecorded, ProjectionFreshness, QueryLogEntry, QueryName, RepoPath, Slot, Snapshot, Step,
    SymbolId, SymbolNode, UndeterminableReason, traverse,
};
use pal_store::{Projection, ProjectionError};
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum QueryError {
    #[error("2층을 읽지 못했다: {0}")]
    Projection(#[from] ProjectionError),
    /// 의도 저장소의 색인을 못 읽었다.
    ///
    /// **문자열인 것은 이 크레이트가 `pal-intent` 를 모르기 때문이다** —
    /// [`BoundIndex`] 가 그 경계이고, 오류 타입을 들이면 경계가 새어 나온다.
    #[error("결박 색인을 읽지 못했다: {0}")]
    BoundIndex(String),
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
    /// 좌표를 못 찾은 문서 조각들 — **사람의 작업 목록** (F10 §2).
    NarrativeUnbound,
    /// ★ 좌표 하나를 만진다 — **걸린 것**과 **지켜보는 것**을 함께 (F11).
    BindingTouch { name: String },
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
            Self::NarrativeUnbound => QueryName::NarrativeUnbound,
            Self::BindingTouch { .. } => QueryName::BindingTouch,
        }
    }

    /// 인자를 한 줄로 — 로그의 요약이 이것을 먹는다.
    #[must_use]
    pub fn args(&self) -> &str {
        match self {
            Self::LedgerSnapshot | Self::GraphDump | Self::BindingStatus
            | Self::NarrativeUnbound => "",
            Self::SymbolResolve { name }
            | Self::SymbolContains { name }
            | Self::SymbolCallers { name }
            | Self::SymbolReaches { name }
            | Self::BindingTouch { name } => name,
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
            QueryName::NarrativeUnbound => Some(Self::NarrativeUnbound),
            QueryName::SymbolResolve => named(|name| Self::SymbolResolve { name }),
            QueryName::SymbolContains => named(|name| Self::SymbolContains { name }),
            QueryName::SymbolCallers => named(|name| Self::SymbolCallers { name }),
            QueryName::SymbolReaches => named(|name| Self::SymbolReaches { name }),
            QueryName::BindingTouch => named(|name| Self::BindingTouch { name }),
        }
    }
}

/// 좌표를 못 찾은 조각 하나 — 산출에 실리는 형태.
///
/// **본문 전체를 안 싣는다.** 작업 목록은 *"어디를 봐야 하는가"* 에 답하는 것이지
/// 문서를 다시 보여 주는 것이 아니고, 조각 수백 개의 본문이 실리면 목록이 안 읽힌다.
/// 첫 줄과 좌표가 있으면 사람이 문서를 연다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UnboundItem {
    /// 개체의 이름 — `decision/01J…`. **승인·거부가 이 이름으로 부른다.**
    pub item: String,
    pub path: RepoPath,
    pub anchor: String,
    /// 본문의 첫 줄. **전부가 아니다.**
    pub head: String,
    /// 이 조각이 든 신호의 수 — **0 이면 문서가 코드를 아예 안 가리킨다.**
    ///
    /// 0 과 「신호는 있는데 아무것도 못 찾았다」는 다른 사건이다. 뭉개면 *"문서가
    /// 심볼을 안 가리킨다"*([R-09])와 *"계단식이 안 돈다"* 가 같은 숫자가 된다.
    pub signals_seen: usize,
}

/// 신호 하나가 낸 후보 집합들의 크기 — **좁혔는가를 이 값이 말한다.**
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CandidateSpread {
    /// 무엇이 걸었나.
    pub by: &'static str,
    /// 그 신호로 「후보 있음」이 된 조각 수.
    pub items: usize,
    /// 후보 집합 크기의 중앙값. **이 값이 크면 그 신호는 안 좁힌 것이다.**
    pub median: usize,
    pub max: usize,
    /// 후보가 **셋 이하**인 것 — *"사람이 실제로 고를 수 있는 것"* 의 수.
    pub reviewable: usize,
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
    /// 좌표를 못 찾은 문서 조각들 — **이것이 사람의 작업 목록이다** (F10 §2).
    ///
    /// # 후보가 있는 것은 여기 없다
    ///
    /// *"여럿이라 못 좁혔다"* 는 **이미 후보가 있는 것**이고 *"신호가 없다"* 는
    /// **사람이 좌표를 붙여야 하는 것**이다. 섞으면 작업 목록이 안 읽힌다.
    /// 그래서 `candidates` 는 **수로만** 실린다 — 있다는 사실은 남고 목록은 안 섞인다.
    Narrative {
        unbound: Vec<UnboundItem>,
        candidates: usize,
        bound: usize,
        /// ★ **후보가 몇 개짜리인가** — 신호별로.
        ///
        /// # 왜 수만으로는 거짓말이 되는가 (F10 실측 · 2026-08-15)
        ///
        /// *"후보 있음 1,563"* 은 **승인 대기 1,563 건**처럼 읽힌다. 그런데 실측에서
        /// `same-commit` 의 후보 집합은 중앙값이 **229** 였다(최대 658) —
        /// **사람이 볼 수 없는 목록이고, 그것은 제안이 아니다.**
        ///
        /// 크기를 안 실으면 *"후보를 좁혔다"* 와 *"후보를 안 좁혔다"* 가 **같은 줄**이
        /// 된다. F09 가 반경을 산출에 실은 것과 정확히 같은 자리다 — **닫히지 않는
        /// 것을 선언으로 다룬다.**
        candidate_sizes: Vec<CandidateSpread>,
    },
    /// ★ 좌표 하나를 만진 답 — **찾았을 때** (F11).
    ///
    /// 못 찾았거나 여럿이면 [`Self::Unknown`]·[`Self::Ambiguous`] 다. **이 질의만의
    /// 변형을 따로 두지 않는다** — 이름을 받는 질의 다섯이 이미 그 둘로 답하고,
    /// 여기만 다른 갈래를 내면 소비자가 질의마다 다른 표를 읽어야 한다.
    Touch { result: Box<pal_core::TouchResult> },
    /// 이름이 여럿으로 해소됐다. **하나를 고르지 않는다.**
    Ambiguous { name: String, candidates: Vec<SymbolNode> },
    /// 이 스냅샷에서 못 찾았다. **없다는 뜻이 아니다** — 근거는 봉투가 진다.
    ///
    /// # `near` — **이것을 뜻했습니까** (F11 §4)
    ///
    /// 좌표 표기(`repo:path#Container.name`)를 사람이 정확히 쓰지 못하는 것이
    /// [F11 §4] 가 적은 이슈이고, 대응이 *"부분 매칭 + 근접 후보 제안"* 이다.
    ///
    /// **비어 있는 것과 목록이 있는 것은 다른 답이다** — 앞은 *"가까운 것도 없다"* 이고
    /// 그것은 이 스냅샷에 대한 사실이다. ⚠ **하나를 고르지 않는다**(P6).
    ///
    /// 가까움의 정의는 [`pal_core::near_kind`] 이고 **임계값이 없다** —
    /// [F11 §5] 가 점수를 기각했고 편집거리 임계도 같은 종류이기 때문이다.
    Unknown { name: String, near: Vec<pal_core::NearName> },
}

/// 좌표 하나에서 결박으로 가는 **두 방향의 색인.**
///
/// 실체는 의도 저장소의 `BOUND_BY` 와 `WATCH` 이고 둘 다 F09 가 세웠다.
/// **둘이 다른 질문에 답한다** — `[f11.pass]` ⑤ 가 그 갈림을 합격선으로 진다.
pub trait BoundIndex {
    /// 이 좌표에 **걸린** 것 — 결박의 `target` 이 이 심볼인 것.
    ///
    /// # Errors
    /// 색인을 읽지 못하면.
    fn bound_to(&self, target: SymbolId) -> Result<Vec<Binding>, QueryError>;

    /// 이 좌표를 **지켜보는** 것 — 감시 집합에 이 심볼이 든 결박.
    ///
    /// 대상이 이 심볼인 것도 함께 나온다(반경 `symbol` 이면 감시 집합이 대상 하나다).
    /// **가르는 것은 부르는 쪽이다** — 이 함수가 가르면 색인 하나가 두 뜻을 지게 된다.
    ///
    /// # Errors
    /// 색인을 읽지 못하면.
    fn watching(&self, member: SymbolId) -> Result<Vec<Binding>, QueryError>;
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
    /// 이 스냅샷의 문서 제안 전부 — **부르는 쪽이 지고 온다.**
    ///
    /// # 왜 여기서 계산하지 않는가
    ///
    /// [`Self::bindings`] 와 **같은 이유다** — 조립은 표면의 일이고, 이 크레이트가
    /// 문서를 읽으면 같은 사실이 두 곳에서 계산된다. 그리고 인입은 git 이력을 타는데
    /// **이 크레이트는 git 을 모른다.**
    ///
    /// **`narrative.unbound` 가 아닌 질의에서는 비어 있고, 그것이 정확한 값이다** —
    /// 문서를 안 읽었으므로 *"미결박이 0"* 이 아니라 *"안 물었다"* 다.
    pub narrative: Vec<pal_core::Proposal>,
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
    /// ★ 좌표 하나로 결박을 **색인으로** 찾는 자리 (F11 §3.1).
    ///
    /// # 왜 [`Self::bindings`] 를 걸러 쓰지 않는가
    ///
    /// [F11 §3.1] 이 못 박았다 — *"핵심은 `BOUND_BY` 역방향 색인이다. 정방향만 있으면
    /// **O(전체 결박)** 이 되고, 그러면 대화 흐름을 끊는다."*
    /// `bindings` 를 훑는 것이 정확히 그 O(전체 결박)이다. `binding.status` 는 답이
    /// 결박 전부라 그 비용이 답 자체지만, `touch` 는 **좌표 하나**에 답한다.
    ///
    /// # 왜 트레이트인가
    ///
    /// 이 크레이트는 `pal-intent` 를 모른다([`Self::bindings`] 의 머리가 그 근거를
    /// 적었다). [`pal_core::BindingStatus::evaluate`] 가 조회를 **클로저로** 받는 것과
    /// 같은 형태이고 같은 이유다 — **좌표계가 저장 기술을 알면 안 된다**(stack §4.1).
    pub bound: &'a dyn BoundIndex,
    /// 한 답이 싣는 결박의 상한 — **자리표시 [`pal_core::PROVISIONAL_TOUCH_BINDING_MAX`]**.
    ///
    /// [`Budget`] 에 안 넣었다. 그 넷은 **탐색** 예산이고 이것은 **싣는 수**다.
    /// ⚠ **낡은 것은 이 상한을 안 탄다**(F11 §3.3) — 그 비대칭이 이 기능의 요구다.
    pub binding_max: usize,
    /// 추출기 버전 — **좌표의 성분이다**(stack §5.1).
    ///
    /// **부르는 쪽이 지고 온다** — 이 크레이트는 파서를 모른다.
    pub extractor: pal_core::ExtractorVersion,
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
        // **한 번만 잰다.** 로그의 값과 봉투의 값이 같은 `Instant` 에서 나와야
        // *"산출의 숫자와 로그의 숫자가 다르다"* 가 일어나지 않는다.
        let duration_micros =
            u64::try_from(started.elapsed().as_micros()).unwrap_or(u64::MAX);
        let entry = QueryLogEntry {
            query: q.name(),
            args_digest: QueryLogEntry::digest_of(q.args()),
            accessed: accessed.clone(),
            elision: elision.clone(),
            duration_micros,
        };
        ctx.projection.log_query(&ctx.snapshot.to_string(), &entry)?;
        LogStatus::Recorded { duration_micros }
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

/// 좌표를 못 찾은 조각들 — **사람의 작업 목록** (F10 §2).
///
/// [`run`] 에서 떼어 냈다. 거기 두면 함수가 100 줄을 넘고, **길어진 `match` 는 새 질의를
/// 더할 때마다 남의 팔을 읽게 만든다.**
fn 미결박(ctx: &QueryCtx, accessed: &mut Vec<SymbolId>) -> QueryResult {
        let mut unbound = Vec::new();
        let mut candidates = 0;
        let mut bound = 0;
        for p in &ctx.narrative {
            match &p.class {
                pal_core::Classification::Bound { target, .. } => {
                    bound += 1;
                    // **승인된 좌표는 이 답이 만진 것이다** — F17 이 로그를 셀 때
                    // *"인입이 무엇을 봤나"* 가 여기서 나온다.
                    accessed.push(*target);
                }
                pal_core::Classification::Candidates { candidates: c, .. } => {
                    candidates += 1;
                    accessed.extend(c.iter().copied());
                }
                pal_core::Classification::Unbound => unbound.push(UnboundItem {
                    item: p.item.to_display(),
                    path: p.fragment.path.clone(),
                    anchor: p.fragment.anchor.clone(),
                    head: p.fragment.body.lines().next().unwrap_or("").to_owned(),
                    signals_seen: 신호_수(&p.fragment.signals),
                }),
            }
        }
    QueryResult::Narrative {
        unbound,
        candidates,
        bound,
        candidate_sizes: 후보_퍼짐(&ctx.narrative),
    }
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
        NamedQuery::NarrativeUnbound => Ok(미결박(ctx, accessed)),
        NamedQuery::BindingStatus => Ok(QueryResult::Bindings {
            bindings: binding_reports(ctx, accessed),
            detector: ctx.detector.clone(),
        }),
        NamedQuery::BindingTouch { name } => touch_result(ctx, name, elision, accessed),
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
                return Ok(QueryResult::Unknown {
                    name: name.clone(),
                    near: near_names(p, name, elision)?,
                });
            }
            Ok(QueryResult::Symbols { symbols })
        }
        NamedQuery::SymbolContains { name } => {
            let start = match unique(p, name, accessed, elision)? {
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
            let start = match unique(p, name, accessed, elision)? {
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
            let start = match unique(p, name, accessed, elision)? {
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
    elision: &mut Elision,
) -> Result<Result<SymbolNode, QueryResult>, QueryError> {
    let mut found = p.resolve_name(name)?;
    accessed.extend(found.iter().map(|s| s.id));
    match found.len() {
        0 => Ok(Err(QueryResult::Unknown {
            name: name.to_owned(),
            near: near_names(p, name, elision)?,
        })),
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
/// 결박 하나의 두 축 — **`binding.status` 와 `binding.touch` 가 같은 함수를 지난다.**
///
/// 두 벌로 두면 한쪽만 고쳐지고, 그러면 같은 결박이 표면에 따라 다른 상태로 나간다.
/// F09 §2.1 이 요구한 것은 *"못 보는 것을 `Live` 로 접지 않는다"* 이고 그 규율은
/// **표면마다가 아니라 한 곳에** 있어야 한다.
fn 결박_상태(ctx: &QueryCtx, b: &Binding) -> BindingStatus {
    let p = ctx.projection;
    if !ctx.freshness.built_for_this_snapshot {
        // **감시 집합을 보기도 전에 판정 불가다.** 여기서 요약을 대면 옛 세대의
        // 값과 지금의 결박을 대는 것이 된다.
        return BindingStatus::projection_stale(Lineage::Current);
    }
    BindingStatus::evaluate(b, Lineage::Current, |id| match p.symbol(id) {
        Ok(Some(n)) if n.identity == IdentityGrade::Unavailable => {
            Now::Undeterminable(UndeterminableReason::IdentityGrade)
        }
        Ok(Some(n)) if ctx.partial_files.contains(&n.path) => {
            Now::Undeterminable(UndeterminableReason::PartialParse)
        }
        Ok(Some(n)) => Now::Digest(n.body),
        Ok(None) => Now::Gone,
        // **읽기 실패를 「사라졌다」로 적지 않는다.** 못 읽은 것과 없는 것은 다른
        // 사건이고, 뭉개면 저장 오류가 `Orphaned` 로 나가 사람이 코드를 고치러 간다.
        Err(_) => Now::Undeterminable(UndeterminableReason::ProjectionStale),
    })
}

fn binding_reports(ctx: &QueryCtx, accessed: &mut Vec<SymbolId>) -> Vec<BindingReport> {
    let p = ctx.projection;

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

        let status = 결박_상태(ctx, b);

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


// ═════════════════════════════════════════════════════════════════════════════
// F11 — 적시 제시. **걸린 것과 지켜보는 것을 가른다**
// ═════════════════════════════════════════════════════════════════════════════

/// 좌표 하나의 답. **못 찾은 것도 답이다** — 오류가 아니다.
fn touch_result(
    ctx: &QueryCtx,
    name: &str,
    elision: &mut Elision,
    accessed: &mut Vec<SymbolId>,
) -> Result<QueryResult, QueryError> {
    let p = ctx.projection;
    let symbol = match unique(p, name, accessed, elision)? {
        Ok(s) => s,
        // **없다는 뜻이 아니다**(`Unknown` · 근접 후보가 함께 실린다) ·
        // **하나를 고르지 않는다**(`Ambiguous`). 둘 다 실패가 아니라 답이다.
        Err(other) => return Ok(other),
    };
    let (mut here, mut watching) = 걸린_것과_지켜보는_것(ctx, &symbol, accessed)?;
    // **점진 회상은 두 목록에 따로 건다** — 합쳐서 자르면 한쪽이 다른 쪽을 밀어낸다.
    // 그리고 **낡은 것은 상한을 안 탄다**(F11 §3.3).
    회상(&mut here, ctx.binding_max, elision);
    회상(&mut watching, ctx.binding_max, elision);
    let facts = SymbolFacts {
        callers: p.callers(symbol.id)?.len(),
        callees: p.callees(symbol.id)?.len(),
    };
    Ok(QueryResult::Touch {
        result: Box::new(조립(ctx, symbol, here, watching, facts)),
    })
}

/// `BOUND_BY` 와 `WATCH` 를 각각 읽고 **가른다.**
///
/// `watching` 은 대상이 이 심볼인 것도 함께 내므로 여기서 뺀다 — 안 빼면 같은 결박이
/// 두 목록에 실리고, 사람이 *"둘에 걸렸다"* 로 읽는다.
fn 걸린_것과_지켜보는_것(
    ctx: &QueryCtx,
    symbol: &SymbolNode,
    accessed: &mut Vec<SymbolId>,
) -> Result<(Vec<BoundItem>, Vec<BoundItem>), QueryError> {
    let here: Vec<BoundItem> = ctx
        .bound
        .bound_to(symbol.id)?
        .into_iter()
        .map(|b| bound_item(ctx, &b, symbol.id, accessed))
        .collect();
    let watching: Vec<BoundItem> = ctx
        .bound
        .watching(symbol.id)?
        .into_iter()
        .filter(|b| b.target != symbol.id)
        .map(|b| bound_item(ctx, &b, symbol.id, accessed))
        .collect();
    Ok((here, watching))
}

/// 결박 하나를 화면에 실리는 형태로.
fn bound_item(
    ctx: &QueryCtx,
    b: &Binding,
    touched: SymbolId,
    accessed: &mut Vec<SymbolId>,
) -> BoundItem {
    accessed.push(b.target);
    let at = if b.target == touched {
        BoundTarget::Here
    } else {
        // **대상이 사라졌으면 그것도 값이다** — `Option` 으로 접으면 *"2층에 없다"* 와
        // *"안 찾아봤다"* 가 같아진다.
        let place = match ctx.projection.symbol(b.target) {
            Ok(Some(n)) => TargetPlace::Known {
                path: n.path,
                container: n.container,
                name: n.name,
                line: n.span.line_start,
            },
            _ => TargetPlace::Gone,
        };
        BoundTarget::Elsewhere { symbol: b.target, place }
    };
    BoundItem::Note {
        binding: b.id.clone(),
        subject: b.subject.clone(),
        note: b.note.clone(),
        status: 결박_상태(ctx, b),
        radius: b.radius.name(),
        watch: b.watch.len(),
        bound_at_time: b.bound_at_time,
        at,
    }
}

/// 점진 회상 — **요약 + 상위 N, 그리고 낡은 것은 상한을 안 탄다** (F11 §3.3).
///
/// > `stale` 은 항상 보인다 — 상한에 걸려도 우선 포함.
/// > **낡은 것이 안 보이면 이 기능의 존재 이유가 사라진다.**
///
/// 그래서 자르는 것은 **낡지 않은 것의 꼬리**뿐이고, 자른 수가
/// [`ElisionReason::BindingMaxExceeded`] 로 실린다. **조용한 절단이 없다.**
fn 회상(items: &mut Vec<BoundItem>, max: usize, elision: &mut Elision) {
    items.sort_by_key(pal_core::정렬_열쇠);
    if items.len() <= max {
        return;
    }
    let 낡은 = items.iter().filter(|i| pal_core::낡았나(i)).count();
    // **상한이 낡은 것보다 작으면 낡은 것 전부를 싣는다.** 상한이 이기면 이 기능이
    // 존재할 이유가 없어진다.
    let 남길 = max.max(낡은);
    if items.len() <= 남길 {
        return;
    }
    let 자른 = items.len() - 남길;
    items.truncate(남길);
    elision.push(ElisionReason::BindingMaxExceeded, 자른);
}

/// 가까운 이름들 — **하나를 고르지 않는다**(`[f11.pass]` ③).
///
/// 훑는 것은 2층의 이름 전부이고 **자르는 것은 고른 뒤**다. 상한을 넘으면 그 수가
/// `elision` 에 실린다 — 한 글자 입력이 전부를 후보로 만드는 자리가 여기다.
fn near_names(
    p: &Projection,
    input: &str,
    elision: &mut Elision,
) -> Result<Vec<pal_core::NearName>, QueryError> {
    let mut out: Vec<pal_core::NearName> = p
        .names()?
        .into_iter()
        .filter_map(|name| pal_core::near_kind(input, &name).map(|kind| pal_core::NearName { name, kind }))
        .collect();
    // **갈래가 먼저, 그다음 사전순** — 점수가 아니라 사실 기반 정렬이다.
    out.sort_by(|a, b| a.kind.cmp(&b.kind).then_with(|| a.name.cmp(&b.name)));
    if out.len() > pal_core::PROVISIONAL_TOUCH_BINDING_MAX {
        let 자른 = out.len() - pal_core::PROVISIONAL_TOUCH_BINDING_MAX;
        out.truncate(pal_core::PROVISIONAL_TOUCH_BINDING_MAX);
        elision.push(ElisionReason::BindingMaxExceeded, 자른);
    }
    Ok(out)
}

/// 좌표 넷과 자리 다섯. **채워지는 것은 지금 넷이고 나머지는 `NotBuilt` 다.**
fn 조립(
    ctx: &QueryCtx,
    symbol: SymbolNode,
    bindings: Vec<BoundItem>,
    watching: Vec<BoundItem>,
    facts: SymbolFacts,
) -> pal_core::TouchResult {
    // 좌표는 **저장소 하나**를 가리킨다. 스냅샷은 집합이므로 그중 하나를 골라야 하고,
    // 이 빌드는 저장소를 하나만 본다. **멀티레포는 F14 다.**
    let (repo, tree) = ctx
        .snapshot
        .entries()
        .next()
        .expect("스냅샷은 비어 있을 수 없다");
    pal_core::TouchResult {
        target: pal_core::Coord {
            repo: repo.clone(),
            tree: *tree,
            extractor: ctx.extractor,
            symbol: symbol.id,
        },
        symbol,
        bindings: Capable::Present(bindings),
        // ★ **F11 이 이 자리를 만들었다** — 대상이 다른 좌표인 결박들.
        watching: Capable::Present(watching),
        facts: Capable::Present(facts),
        unresolved: Capable::not_built(pal_core::CapabilityId::new("F08", "unresolved-refs")),
        effects: Capable::not_built(pal_core::CapabilityId::new("F13", "effects")),
        judgments: Capable::not_built(pal_core::CapabilityId::new("F15", "judgment")),
    }
}

/// 좌표 하나를 만진다 — **표면이 부르는 자리.**
///
/// [`execute`] 를 그대로 지나므로 **봉투도 질의 로그도 같은 경로에서 난다.**
/// 답의 모양만 벗겨 낸다 — `pal touch --json` 의 형태가 S2 이래 그대로여야 하고,
/// 그것을 위해 계산을 두 벌 두면 그 순간 둘이 갈린다.
///
/// # Errors
/// 2층이나 결박 색인을 읽지 못하면.
pub fn touch(
    ctx: &QueryCtx,
    name: &str,
) -> Result<Envelope<pal_core::TouchAnswer>, QueryError> {
    let env = execute(&NamedQuery::BindingTouch { name: name.to_owned() }, ctx)?;
    Ok(env.map(|r| match r {
        QueryResult::Touch { result } => pal_core::TouchAnswer::Found(result),
        QueryResult::Ambiguous { name, candidates } => {
            pal_core::TouchAnswer::Ambiguous { name, candidates }
        }
        QueryResult::Unknown { name, near } => pal_core::TouchAnswer::Unknown { name, near },
        // `binding.touch` 는 위 셋만 낸다. 다른 것이 오면 `run` 이 바뀐 것이다.
        _ => unreachable!("binding.touch 가 세 갈래 밖의 것을 냈다"),
    }))
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

/// 이 조각이 든 신호가 몇 개인가 — **0 이면 문서가 코드를 아예 안 가리킨다.**
///
/// 붙어 있는 좌표 · 프론트매터 · 펜스 안의 경로 · 인라인 스팬만 센다.
/// **동반 변경은 안 센다** — 그것은 조각이 든 신호가 아니라 **저장소의 사정**이고,
/// 세면 모든 조각이 최소 하나를 갖게 되어 이 값이 아무것도 안 가른다.
fn 신호_수(s: &pal_core::RawSignals) -> usize {
    s.attached.len() + s.grounds.len() + s.fenced_paths.len() + s.spans.len()
}

/// 신호마다 후보 집합이 얼마나 넓은가 — **좁혔는가를 재는 자리.**
///
/// **후보가 셋 이하인 것을 따로 센다.** 그것이 *"사람이 실제로 고를 수 있는 것"* 이고,
/// 나머지는 **제안이 아니라 목록**이다.
fn 후보_퍼짐(proposals: &[pal_core::Proposal]) -> Vec<CandidateSpread> {
    let mut 모음: std::collections::BTreeMap<&'static str, Vec<usize>> =
        std::collections::BTreeMap::new();
    for p in proposals {
        if let pal_core::Classification::Candidates { by, candidates } = &p.class {
            모음.entry(by.name()).or_default().push(candidates.len());
        }
    }
    모음
        .into_iter()
        .map(|(by, mut sizes)| {
            sizes.sort_unstable();
            let median = sizes.get(sizes.len() / 2).copied().unwrap_or(0);
            CandidateSpread {
                by,
                items: sizes.len(),
                median,
                max: sizes.last().copied().unwrap_or(0),
                reviewable: sizes.iter().filter(|n| **n <= 3).count(),
            }
        })
        .collect()
}
