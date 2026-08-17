//! `doctor` — **저장된 그래프가 자기 규칙을 지키는가** ([옛 DESIGN §12.7](../../../docs/plan/disposal-map.md) · D29).
//!
//! # 재구축 등가성 검사가 말하지 않는 것
//!
//! 계획의 상시 검사 열하나와 오라클 넷은 전부 *빌드·재구축·회귀*에 대한 것이다.
//! 재구축 등가성은 *"두 번 만들면 같은가"* 를 말할 뿐 **"지금 이 그래프가 정합한가"** 를
//! 말하지 않는다 — 손상 · 부분 갱신 · 스키마 진화 · 중단된 트랜잭션은 그 검사를
//! **통과한다.**
//!
//! # 불변식은 스키마에서 파생되며 손으로 세지 않는다
//!
//! 여덟 중 다섯(1·2·3·4·5)이 [`GraphSchema`] 를 읽어 모집단과 규칙을 얻는다. 스키마에
//! 노드가 하나 늘면 검사 코드를 고치지 않아도 그 노드가 검사 대상이 된다. 그리고
//! **스키마가 늘었는데 뷰가 그것을 담을 수 있는지 말하지 않으면** 그 사실이
//! [`Diagnosis::coverage_gaps`] 로 산출에 실린다 — 조용히 0 을 내지 않는다.
//!
//! # 자기 자신에게도 3분할이 걸린다 (§12.7 말미)
//!
//! 기본은 **표본**이고 전수는 명시적 호출이다. 표본만 보고 *"위반 없음"* 을 내면
//! 이 도구가 자기가 고발한 문제를 스스로 저지른다. 그래서 보지 않은 범위는
//! [`crate::Residual`] 이고, 모집단이 존재할 수 없는 불변식은 **위반 0 이 아니라
//! [`InvariantOutcome::NotBuilt`]** 다.

use std::collections::BTreeSet;

use serde::Serialize;

use crate::cascade::{Cascade, NodeFreshness, cascade};
use crate::coord::Coord;
use crate::budget::{CANDIDATE_LIMIT, PROVISIONAL_CASCADE_DEPTH, PROVISIONAL_SAMPLE_MAX};
use crate::graph::{Provenance, ResolutionGrade};
use crate::judgment::{Residual, ResidualReason};
use crate::schema::{EvidenceRule, GradeRule, GraphSchema, NodeStatus, Requirement};
use crate::view::{Anchor, EdgeInstance, EdgeTarget, GraphView, NodeInstance, NodeKey};

// **예산 둘(`K` · 표본 상한)은 여기 없다.** `pal-core::budget` 한 곳이다
// (stack §5.5 · `[f05.1.pass]` ①). 여기서 지는 것은 *"기본이 표본이고 전수가 명시적
// 호출"* 이라는 §12.7 의 분기가 **존재하는가**까지다.

// ── 스키마 라벨이 아닌 모집단 넷 ─────────────────────────────────────────────
//
// 불변식 여덟 중 셋(⑥⑦⑧)의 모집단은 `schema/graph.toml` 의 노드·엣지가 아니다 —
// 판정 산출이거나(잔여·범위 축소), 저장 계층의 자리이거나(결박 색인), 노드의 성질이다
// (입력을 갖는가). **뷰는 이 넷도 담을 수 있는지 선언해야 한다.** 아니면 그 세 불변식이
// 조용히 "위반 0" 이 된다.

/// 잔여 — 불변식 ⑥ 의 모집단 절반.
pub const RESIDUAL_KIND: &str = "Residual";
/// 범위 축소 — 불변식 ⑥ 의 나머지 절반. **타입이 아직 없다**(F20).
pub const SCOPE_REDUCTION_KIND: &str = "ScopeReduction";
/// 2층의 결박 색인. **의도 저장소 안의 역방향 색인과 다른 것이다** — F05 가 옮긴다.
pub const BINDING_INDEX_KIND: &str = "BindingIndex";
/// 입력을 갖는 노드 — 파생물. 불변식 ⑧ 의 모집단.
pub const DERIVED_KIND: &str = "DerivedNode";

/// 불변식 여덟 — §12.7 의 표 그대로.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum InvariantId {
    /// ① 모든 엣지의 양 끝 노드가 존재한다.
    EdgeEndsExist,
    /// ② 모든 노드·엣지가 등록된 라벨이고 필수 속성을 갖는다.
    RegisteredAndRequired,
    /// ③ 속성 `producer` 가 노드 `provenance` 와 정합한다.
    ProducerFitsProvenance,
    /// ④ `inferred` 노드의 `evidence_refs` 가 비어 있지 않다.
    InferredCarriesEvidence,
    /// ⑤ `candidate` 후보 집합이 `K` 이하이고 초과분에 `UnresolvedRef` 가 있다.
    CandidateSetWithinLimit,
    /// ⑥ 모든 `Residual`·`ScopeReduction` 이 실재하는 좌표에 결박되어 있다.
    ResidualAnchored,
    /// ⑦ 2층의 결박 색인이 가리키는 실체가 의도 저장소에 있다.
    BindingIndexResolves,
    /// ⑧ 낡음 등급이 이어달리기 규칙과 정합한다.
    FreshnessConsistent,
}

impl InvariantId {
    /// §12.7 의 순서. **여덟이 전부 여기 있다.**
    pub const ALL: [Self; 8] = [
        Self::EdgeEndsExist,
        Self::RegisteredAndRequired,
        Self::ProducerFitsProvenance,
        Self::InferredCarriesEvidence,
        Self::CandidateSetWithinLimit,
        Self::ResidualAnchored,
        Self::BindingIndexResolves,
        Self::FreshnessConsistent,
    ];

    #[must_use]
    pub const fn number(self) -> u8 {
        match self {
            Self::EdgeEndsExist => 1,
            Self::RegisteredAndRequired => 2,
            Self::ProducerFitsProvenance => 3,
            Self::InferredCarriesEvidence => 4,
            Self::CandidateSetWithinLimit => 5,
            Self::ResidualAnchored => 6,
            Self::BindingIndexResolves => 7,
            Self::FreshnessConsistent => 8,
        }
    }

    /// 불변식의 문장. **§12.7 의 표에서 옮겨 온다.**
    #[must_use]
    pub const fn statement(self) -> &'static str {
        match self {
            Self::EdgeEndsExist => "모든 엣지의 양 끝 노드가 존재한다",
            Self::RegisteredAndRequired => {
                "모든 노드·엣지가 스키마에 등록된 라벨·타입이고 필수 속성을 갖는다"
            }
            Self::ProducerFitsProvenance => {
                "한 노드의 속성 producer 들이 그 노드의 provenance 와 정합한다"
            }
            Self::InferredCarriesEvidence => "inferred 노드의 evidence_refs 가 비어 있지 않다",
            Self::CandidateSetWithinLimit => {
                "candidate 후보 집합의 크기가 K 이하이고 초과분에 대응하는 UnresolvedRef 가 있다"
            }
            Self::ResidualAnchored => "모든 Residual·ScopeReduction 이 실재하는 좌표에 결박되어 있다",
            Self::BindingIndexResolves => "2층의 결박 색인이 가리키는 실체가 의도 저장소에 있다",
            Self::FreshnessConsistent => "낡음 등급이 전파 규칙과 정합한다 — live 노드의 입력에 stale 이 없다",
        }
    }

    /// 깨지면 무엇이 일어나나 — §12.7 의 오른쪽 열.
    #[must_use]
    pub const fn breaks(self) -> &'static str {
        match self {
            Self::EdgeEndsExist => "질의가 조용히 빈 결과를 낸다",
            Self::RegisteredAndRequired => "\"필수이거나 없거나\"가 저장 층에서 무효",
            Self::ProducerFitsProvenance => "출처 파티션이 거짓이 된다",
            Self::InferredCarriesEvidence => "P3 이 문장으로 되돌아간다",
            Self::CandidateSetWithinLimit => "절단이 조용해진다",
            Self::ResidualAnchored => "잔여가 유령이 된다",
            Self::BindingIndexResolves => "승인 노동의 유실이 조용히 일어난다",
            Self::FreshnessConsistent => "KG 의 일부만 신선하다",
        }
    }
}

/// 담을 수 없는 자리 하나.
///
/// # `CapabilityId` 가 아닌 이유
///
/// [`crate::CapabilityId`] 는 `&'static str` 이다 — *"능력은 빌드 시점에 정해진다"*
/// (stack §5.3). 그런데 여기 실리는 이름 중 일부는 **스키마 파일에서 읽은 문자열**이고
/// (`built_by = "F08"`) 그것은 빌드 시점 상수가 아니다. 억지로 상수로 만들면
/// 그 규칙이 거짓이 된다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Absence {
    /// 담을 수 없는 라벨·엣지 타입.
    pub label: String,
    /// 어느 기능이 그것을 만드나.
    pub built_by: String,
}

/// 위반 하나 — **존재 주장이고 반례가 붙는다**(§8).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Violation {
    pub invariant: InvariantId,
    /// 반례 — 어긋난 자리의 이름.
    pub subject: String,
    /// 그 자리의 좌표. **결박되어야 적시 제시가 된다**(§11.3).
    pub anchor: Anchor,
    pub detail: String,
}

/// 불변식 하나를 실제로 돌린 결과.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Outcome {
    /// 실제로 검사한 단위 수.
    pub checked: usize,
    /// 표본 밖이라 보지 않은 단위 수. **0 이 아니면 잔여가 함께 나간다.**
    pub skipped: usize,
    pub violations: usize,
}

/// 불변식 하나의 처지.
///
/// **`NotBuilt` 는 "위반 0" 이 아니다.** 모집단이 존재할 수 없다는 뜻이고,
/// 그 둘을 같은 출력으로 내는 것이 [목표 §3.1](../../../docs/plan/00-goals.md) 의
/// 정면 위반이다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InvariantOutcome {
    Checked(Outcome),
    NotBuilt,
}

/// 불변식 하나에 대한 보고.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct InvariantReport {
    pub number: u8,
    pub invariant: InvariantId,
    pub statement: &'static str,
    /// 이 불변식의 모집단 중 **이 뷰가 담을 수 없는 자리 전부.**
    pub absent: Vec<Absence>,
    pub outcome: InvariantOutcome,
}

/// 얼마나 보는가. **기본은 표본이고 전수는 명시적 호출이다**(§12.7).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DoctorScope {
    Sample { max: usize },
    Full,
}

impl Default for DoctorScope {
    fn default() -> Self {
        Self::Sample { max: PROVISIONAL_SAMPLE_MAX }
    }
}

/// `doctor` 의 답. **[`crate::Envelope`] 에 담겨 나간다.**
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Diagnosis {
    pub scope: DoctorScope,
    pub invariants: Vec<InvariantReport>,
    /// **`clean` 이 없다** — 비어 있으면 비어 있는 것이고 그 사실은 `invariants` 가 말한다.
    pub violations: Vec<Violation>,
    pub residuals: Vec<Residual>,
    /// 스키마가 선언했는데 **뷰가 담을 수 있는지 말하지 않은** 라벨.
    ///
    /// 비어 있지 않으면 이 검사의 커버리지에 구멍이 있다는 뜻이다. 스키마가 자라면
    /// 여기가 먼저 늘어난다 — **검사가 자기도 모르게 좁아지는 것을 막는 자리다.**
    pub coverage_gaps: Vec<String>,
    /// 이어달리기가 예산에 걸렸는데 결박할 좌표를 찾지 못한 자리.
    pub unanchored_cutoff: Vec<NodeKey>,
}

impl Diagnosis {
    /// 위반 건수. **0 이 "이상 없음"을 뜻하지 않는다** — `invariants` 를 함께 읽어야 한다.
    #[must_use]
    pub fn violation_count(&self) -> usize {
        self.violations.len()
    }

    /// 모집단이 실제로 있어서 검사된 불변식 수.
    #[must_use]
    pub fn checked_invariants(&self) -> usize {
        self.invariants
            .iter()
            .filter(|r| matches!(&r.outcome, InvariantOutcome::Checked(o) if o.checked > 0))
            .count()
    }
}

/// 여덟을 돌린다.
#[must_use]
pub fn run(schema: &GraphSchema, view: &GraphView, scope: DoctorScope) -> Diagnosis {
    let ctx = Context::new(schema, view, scope);
    let mut violations = Vec::new();
    let mut residuals = Vec::new();
    let mut invariants = Vec::new();

    let cascaded = cascade(view, PROVISIONAL_CASCADE_DEPTH);
    residuals.extend(cascaded.residuals.clone());

    for id in InvariantId::ALL {
        let absent = ctx.absences(id);
        let report = match id {
            InvariantId::EdgeEndsExist => ctx.edge_ends_exist(&mut violations, &mut residuals),
            InvariantId::RegisteredAndRequired => {
                ctx.registered_and_required(&mut violations, &mut residuals)
            }
            InvariantId::ProducerFitsProvenance => {
                ctx.producer_fits(&mut violations, &mut residuals)
            }
            InvariantId::InferredCarriesEvidence => {
                ctx.inferred_carries_evidence(&mut violations, &mut residuals)
            }
            InvariantId::CandidateSetWithinLimit => {
                ctx.candidate_within_limit(&mut violations, &mut residuals)
            }
            InvariantId::ResidualAnchored => ctx.residual_anchored(&mut violations, &mut residuals),
            InvariantId::BindingIndexResolves => {
                ctx.index_resolves(&mut violations, &mut residuals)
            }
            InvariantId::FreshnessConsistent => {
                ctx.freshness_consistent(&cascaded, &mut violations, &mut residuals)
            }
        };
        // **모집단이 0 인데 담을 자리도 없으면 검사한 것이 아니다.**
        let outcome = if report.checked == 0 && report.skipped == 0 && !absent.is_empty() {
            InvariantOutcome::NotBuilt
        } else {
            InvariantOutcome::Checked(report)
        };
        invariants.push(InvariantReport {
            number: id.number(),
            invariant: id,
            statement: id.statement(),
            absent,
            outcome,
        });
    }

    Diagnosis {
        scope,
        invariants,
        violations,
        residuals,
        coverage_gaps: ctx.coverage_gaps(),
        unanchored_cutoff: cascaded.unanchored_cutoff,
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 안쪽
// ─────────────────────────────────────────────────────────────────────────────

struct Context<'a> {
    schema: &'a GraphSchema,
    view: &'a GraphView,
    scope: DoctorScope,
    keys: BTreeSet<&'a NodeKey>,
    anchored: BTreeSet<crate::coord::SymbolId>,
}

impl<'a> Context<'a> {
    fn new(schema: &'a GraphSchema, view: &'a GraphView, scope: DoctorScope) -> Self {
        Self {
            schema,
            view,
            scope,
            keys: view.keys(),
            anchored: view.anchored_symbols(),
        }
    }

    /// 스키마가 선언했는데 뷰가 말하지 않은 라벨.
    fn coverage_gaps(&self) -> Vec<String> {
        let declared: BTreeSet<&str> =
            self.view.coverage().declared().map(String::as_str).collect();
        let mut gaps: Vec<String> = Vec::new();
        for label in self.schema.nodes.keys().chain(self.schema.edges.keys()) {
            if !declared.contains(label.as_str()) {
                gaps.push(label.clone());
            }
        }
        for extra in [RESIDUAL_KIND, SCOPE_REDUCTION_KIND, BINDING_INDEX_KIND, DERIVED_KIND] {
            if !declared.contains(extra) {
                gaps.push(extra.to_owned());
            }
        }
        gaps
    }

    /// 이 라벨을 담을 수 없으면 그 사실.
    ///
    /// **두 경로가 있다** — 스키마가 `not_built` 이라 값이 설 수 없거나, 뷰가 담지
    /// 못한다고 선언했거나. 앞의 것이 우선이다(스키마가 정본이다).
    fn absence(&self, label: &str) -> Option<Absence> {
        if let Some(n) = self.schema.nodes.get(label) {
            if let NodeStatus::NotBuilt { by } = &n.status {
                return Some(Absence { label: label.to_owned(), built_by: by.clone() });
            }
        }
        match self.view.coverage().of(label) {
            Some(crate::Capable::NotBuilt { capability }) => Some(Absence {
                label: label.to_owned(),
                built_by: capability.feature.to_owned(),
            }),
            _ => None,
        }
    }

    /// 이 불변식이 필요로 하는 라벨 전부.
    fn needs(&self, id: InvariantId) -> Vec<String> {
        match id {
            InvariantId::EdgeEndsExist => self.schema.edges.keys().cloned().collect(),
            InvariantId::RegisteredAndRequired => self
                .schema
                .nodes
                .keys()
                .chain(self.schema.edges.keys())
                .cloned()
                .collect(),
            InvariantId::ProducerFitsProvenance => self.schema.nodes.keys().cloned().collect(),
            InvariantId::InferredCarriesEvidence => self
                .schema
                .nodes
                .iter()
                .filter(|(_, d)| d.provenance == Provenance::Inferred)
                .map(|(l, _)| l.clone())
                .collect(),
            InvariantId::CandidateSetWithinLimit => self
                .schema
                .edges
                .iter()
                .filter(|(_, d)| {
                    matches!(d.grade, GradeRule::PerEdge)
                        || matches!(d.grade, GradeRule::Fixed(ResolutionGrade::Candidate))
                })
                .map(|(n, _)| n.clone())
                .collect(),
            InvariantId::ResidualAnchored => {
                vec![RESIDUAL_KIND.to_owned(), SCOPE_REDUCTION_KIND.to_owned()]
            }
            InvariantId::BindingIndexResolves => vec![BINDING_INDEX_KIND.to_owned()],
            InvariantId::FreshnessConsistent => vec![DERIVED_KIND.to_owned()],
        }
    }

    fn absences(&self, id: InvariantId) -> Vec<Absence> {
        let needed = self.needs(id);
        let mut out: Vec<Absence> = needed.iter().filter_map(|l| self.absence(l)).collect();
        // 필요로 하는 라벨이 **하나도 없으면** 그것 자체가 모집단 부재다
        // — 예: `inferred` 노드를 선언한 스키마가 없다.
        if needed.is_empty() {
            out.push(Absence {
                label: "이 불변식의 모집단이 될 라벨".to_owned(),
                built_by: "스키마에 없다".to_owned(),
            });
        }
        out
    }

    // ── 표본 ─────────────────────────────────────────────────────────────────

    /// 검사할 단위를 고른다.
    ///
    /// # 좌표 없는 것은 표본에서 빼지 않는다
    ///
    /// 뺐다면 그 사실을 결박할 자리가 없고, **결박 없는 잔여는 불변식 6 이 유령이라
    /// 부르는 것**이다. 불변식 하나가 표본 정책을 강제하는 자리다.
    ///
    /// # 등간격이고 난수가 아니다
    ///
    /// 같은 그래프에 두 번 돌리면 같은 표본이어야 산출을 비교할 수 있다.
    /// [T10](../../../docs/gates/preflight.md) 의 선정 규칙과 같은 형태다.
    fn select<T>(&self, units: Vec<(Anchor, T)>) -> (Vec<T>, Vec<Coord>) {
        let max = match self.scope {
            DoctorScope::Full => return (units.into_iter().map(|(_, t)| t).collect(), Vec::new()),
            DoctorScope::Sample { max } => max,
        };

        let mut checked = Vec::new();
        let mut sampleable: Vec<(Coord, T)> = Vec::new();
        for (anchor, item) in units {
            match anchor {
                Anchor::Coordless => checked.push(item),
                Anchor::At(c) => sampleable.push((c, item)),
            }
        }

        if sampleable.len() <= max || max == 0 {
            checked.extend(sampleable.into_iter().map(|(_, t)| t));
            return (checked, Vec::new());
        }

        let stride = sampleable.len().div_ceil(max);
        let mut skipped = Vec::new();
        for (i, (coord, item)) in sampleable.into_iter().enumerate() {
            if i % stride == 0 {
                checked.push(item);
            } else {
                skipped.push(coord);
            }
        }
        (checked, skipped)
    }

    /// 보지 않은 범위를 잔여로 낸다. **"이상 없음"이 아니다.**
    fn skipped_residual(&self, id: InvariantId, mut skipped: Vec<Coord>) -> Option<Residual> {
        if skipped.is_empty() {
            return None;
        }
        let first = skipped.remove(0);
        Some(Residual::new(
            ResidualReason::OutsideSample,
            format!("불변식 {} — {}", id.number(), id.statement()),
            first,
            skipped,
            self.view.at.clone(),
            "전수 검사(`pal doctor --full`)",
        ))
    }

    fn finish(
        &self,
        id: InvariantId,
        checked: usize,
        skipped: Vec<Coord>,
        violations: usize,
        residuals: &mut Vec<Residual>,
    ) -> Outcome {
        let n = skipped.len();
        if let Some(r) = self.skipped_residual(id, skipped) {
            residuals.push(r);
        }
        Outcome { checked, skipped: n, violations }
    }

    fn edge_anchor(&self, e: &EdgeInstance) -> Anchor {
        for key in std::iter::once(&e.from).chain(e.targets()) {
            if let Some(c) = self.view.node(key).and_then(|n| n.anchor.coord()) {
                return Anchor::At(c.clone());
            }
        }
        Anchor::Coordless
    }

    // ── ① 엣지의 양 끝 노드가 존재한다 ───────────────────────────────────────

    fn edge_ends_exist(
        &self,
        violations: &mut Vec<Violation>,
        residuals: &mut Vec<Residual>,
    ) -> Outcome {
        let units: Vec<(Anchor, &EdgeInstance)> =
            self.view.edges().iter().map(|e| (self.edge_anchor(e), e)).collect();
        let (checked, skipped) = self.select(units);

        let mut found = 0;
        for e in &checked {
            let anchor = self.edge_anchor(e);
            let mut missing: Vec<String> = Vec::new();
            if !self.keys.contains(&e.from) {
                missing.push(e.from.to_string());
            }
            for t in e.targets() {
                if !self.keys.contains(t) {
                    missing.push(t.to_string());
                }
            }
            if let EdgeTarget::Candidates { demoted_to: Some(d), .. } = &e.to {
                if !self.keys.contains(d) {
                    missing.push(d.to_string());
                }
            }
            if !missing.is_empty() {
                found += 1;
                violations.push(Violation {
                    invariant: InvariantId::EdgeEndsExist,
                    subject: format!("{}: {}", e.kind, e.from),
                    anchor,
                    detail: format!("가리키는 노드가 없다 — {}", missing.join(" · ")),
                });
            }
        }
        self.finish(InvariantId::EdgeEndsExist, checked.len(), skipped, found, residuals)
    }

    // ── ② 등록된 라벨이고 필수 속성을 갖는다 ─────────────────────────────────

    fn registered_and_required(
        &self,
        violations: &mut Vec<Violation>,
        residuals: &mut Vec<Residual>,
    ) -> Outcome {
        #[derive(Clone, Copy)]
        enum Unit<'u> {
            Node(&'u NodeInstance),
            Edge(&'u EdgeInstance),
        }

        let mut units: Vec<(Anchor, Unit<'_>)> =
            self.view.nodes().iter().map(|n| (n.anchor.clone(), Unit::Node(n))).collect();
        units.extend(self.view.edges().iter().map(|e| (self.edge_anchor(e), Unit::Edge(e))));
        let (checked, skipped) = self.select(units);

        let mut found = 0;
        for unit in &checked {
            let (subject, anchor, problems) = match unit {
                Unit::Node(n) => (n.key.to_string(), n.anchor.clone(), self.node_shape(n)),
                Unit::Edge(e) => {
                    (format!("{}: {}", e.kind, e.from), self.edge_anchor(e), self.edge_shape(e))
                }
            };
            if !problems.is_empty() {
                found += 1;
                violations.push(Violation {
                    invariant: InvariantId::RegisteredAndRequired,
                    subject,
                    anchor,
                    detail: problems.join(" · "),
                });
            }
        }
        self.finish(
            InvariantId::RegisteredAndRequired,
            checked.len(),
            skipped,
            found,
            residuals,
        )
    }

    /// 노드 하나가 선언된 모양을 지고 있는가.
    ///
    /// **양방향이다** — 스키마가 요구한 필수 속성이 없어도, 스키마에 없는 속성을
    /// 싣고 있어도 어긋난 것이다. 뒤의 것이 없으면 **스키마 진화가 조용해진다**:
    /// 옛 코드가 쓰던 속성이 인스턴스에 남아 있는데 아무도 세지 않는다.
    fn node_shape(&self, n: &NodeInstance) -> Vec<String> {
        let Some(decl) = self.schema.nodes.get(&n.key.label) else {
            return vec!["스키마에 없는 노드 라벨이다".to_owned()];
        };
        let mut problems: Vec<String> = Vec::new();
        for a in &decl.attrs {
            let required = match &a.required {
                Requirement::Always => true,
                Requirement::IfProvenance(p) => n.provenance == *p,
            };
            if required && !n.attrs.contains_key(&a.name) {
                problems.push(format!("필수 속성 `{}` 이 없다", a.name));
            }
        }
        for name in n.attrs.keys() {
            if !decl.attrs.iter().any(|a| &a.name == name) {
                problems.push(format!("스키마에 없는 속성 `{name}` 을 싣고 있다"));
            }
        }
        problems
    }

    /// 엣지 하나가 **공통 넷**을 제대로 지고 있는가.
    ///
    /// 엣지의 *"필수 속성"* 은 등급 · 출처 · 근거 · 발생 `Snapshot` 넷이다
    /// ([옛 DESIGN §1.2](../../../docs/plan/disposal-map.md)). 도메인·레인지도 여기서 본다 —
    /// 라벨이 등록됐다는 것은 **그 라벨 사이에** 설 수 있다는 뜻이기 때문이다.
    fn edge_shape(&self, e: &EdgeInstance) -> Vec<String> {
        let Some(decl) = self.schema.edges.get(&e.kind) else {
            return vec!["스키마에 없는 엣지 타입이다".to_owned()];
        };
        let mut problems: Vec<String> = Vec::new();
        if decl.from != e.from.label {
            problems.push(format!(
                "출발 라벨이 `{}` 인데 스키마는 `{}` 이다",
                e.from.label, decl.from
            ));
        }
        for t in e.targets() {
            if !decl.to.contains(&t.label) {
                problems.push(format!("도착 라벨 `{}` 이 스키마에 없다", t.label));
            }
        }
        if let GradeRule::Fixed(g) = decl.grade {
            if g != e.grade {
                problems.push(format!(
                    "등급이 `{}` 인데 스키마는 `{}` 하나뿐이다",
                    e.grade.name(),
                    g.name()
                ));
            }
        }
        if !decl.provenance.contains(&e.provenance) {
            problems.push(format!("출처 `{}` 로 설 수 없는 엣지다", e.provenance.name()));
        }
        if let EvidenceRule::RequiredIfInferred { .. } = decl.evidence {
            if e.provenance == Provenance::Inferred && e.evidence.is_empty() {
                problems.push("inferred 인데 근거가 비어 있다".to_owned());
            }
        }
        problems
    }

    // ── ③ `producer` ↔ `provenance` 정합 ─────────────────────────────────────

    /// # 로딩 시점 검사(F22-1)와 무엇이 다른가
    ///
    /// [`GraphSchema::parse`] 는 **선언**을 본다 — *"`Symbol` 의 속성에 `agent` 생산자를
    /// 적을 수 없다."* 여기서는 **인스턴스**를 본다 — *"이 심볼이 실제로 싣고 있는 출처가
    /// 선언과 같은가."* §12.7 이 든 위협 넷(손상 · 부분 갱신 · 스키마 진화 · 중단된
    /// 트랜잭션)은 전부 **선언을 통과한 뒤에 인스턴스를 어긋나게 만드는 것**이다.
    ///
    /// # 여기서 [`Producer::fits`] 를 다시 부르지 않는다 — **음성 대조가 잡은 것**
    ///
    /// 초안은 §3.4 의 규칙을 그대로 옮겨 `producer.fits(provenance)` 를 여기서 다시
    /// 검사했다. 음성 대조가 그 자리를 지웠는데 **픽스처가 여전히 통과했다** — 검사가
    /// 아니라 장식이었다.
    ///
    /// 도달 불가인 이유가 구조적이다. [`GraphSchema::parse`] 가 *선언된* 생산자는
    /// *선언된* 출처에 맞음을 보장하므로, 인스턴스가 `fits` 를 어기려면 **생산자가
    /// 선언과 다르거나 출처가 선언과 달라야** 한다. 그 둘을 아래에서 이미 본다.
    /// 그래서 `fits` 는 이 함수에서 절대 처음 발화하지 못하고,
    /// **한 번도 발화하지 못하는 검사를 두는 것이 이 프로젝트가 세는 것**이다.
    ///
    /// 규칙이 사라진 것이 아니다 — 규칙은 **로딩 시점 하나**에 있고 여기서는 그 선언에
    /// 인스턴스를 댄다. 같은 규칙을 두 곳에 두면 둘 중 하나가 반드시 죽는다.
    fn producer_fits(
        &self,
        violations: &mut Vec<Violation>,
        residuals: &mut Vec<Residual>,
    ) -> Outcome {
        let units: Vec<(Anchor, &NodeInstance)> =
            self.view.nodes().iter().map(|n| (n.anchor.clone(), n)).collect();
        let (checked, skipped) = self.select(units);

        let mut found = 0;
        for n in &checked {
            let Some(decl) = self.schema.nodes.get(&n.key.label) else { continue };
            let mut problems: Vec<String> = Vec::new();
            if decl.provenance != n.provenance {
                problems.push(format!(
                    "인스턴스 출처가 `{}` 인데 스키마는 `{}` 이다",
                    n.provenance.name(),
                    decl.provenance.name()
                ));
            }
            for (name, producer) in &n.attrs {
                if let Some(a) = decl.attrs.iter().find(|a| &a.name == name) {
                    if &a.producer != producer {
                        problems.push(format!(
                            "`{name}` 의 생산자가 `{}` 인데 스키마는 `{}` 이다",
                            producer.name(),
                            a.producer.name()
                        ));
                    }
                }
            }
            if !problems.is_empty() {
                found += 1;
                violations.push(Violation {
                    invariant: InvariantId::ProducerFitsProvenance,
                    subject: n.key.to_string(),
                    anchor: n.anchor.clone(),
                    detail: problems.join(" · "),
                });
            }
        }
        self.finish(
            InvariantId::ProducerFitsProvenance,
            checked.len(),
            skipped,
            found,
            residuals,
        )
    }

    // ── ④ `inferred` 노드의 `evidence_refs` ──────────────────────────────────

    fn inferred_carries_evidence(
        &self,
        violations: &mut Vec<Violation>,
        residuals: &mut Vec<Residual>,
    ) -> Outcome {
        let units: Vec<(Anchor, &NodeInstance)> = self
            .view
            .nodes()
            .iter()
            .filter(|n| n.provenance == Provenance::Inferred)
            .map(|n| (n.anchor.clone(), n))
            .collect();
        let (checked, skipped) = self.select(units);

        let mut found = 0;
        for n in &checked {
            if n.evidence_refs.is_empty() {
                found += 1;
                violations.push(Violation {
                    invariant: InvariantId::InferredCarriesEvidence,
                    subject: n.key.to_string(),
                    anchor: n.anchor.clone(),
                    detail: "inferred 인데 근거가 비어 있다 — 저장될 수 없는 값이다".to_owned(),
                });
            }
        }
        self.finish(
            InvariantId::InferredCarriesEvidence,
            checked.len(),
            skipped,
            found,
            residuals,
        )
    }

    // ── ⑤ 후보 집합 상한 ─────────────────────────────────────────────────────

    fn candidate_within_limit(
        &self,
        violations: &mut Vec<Violation>,
        residuals: &mut Vec<Residual>,
    ) -> Outcome {
        let units: Vec<(Anchor, &EdgeInstance)> = self
            .view
            .edges()
            .iter()
            .filter(|e| matches!(e.to, EdgeTarget::Candidates { .. }))
            .map(|e| (self.edge_anchor(e), e))
            .collect();
        let (checked, skipped) = self.select(units);

        let mut found = 0;
        for e in &checked {
            let EdgeTarget::Candidates { kept, total, demoted_to } = &e.to else { continue };
            let mut problems: Vec<String> = Vec::new();
            if kept.len() > CANDIDATE_LIMIT {
                problems.push(format!(
                    "후보 {}개가 저장돼 있다 — 상한 K={CANDIDATE_LIMIT}",
                    kept.len()
                ));
            }
            if *total > kept.len() && demoted_to.is_none() {
                problems.push(format!(
                    "후보 {total}개 중 {}개만 남았는데 초과분의 UnresolvedRef 가 없다 — 절단이 조용해진다",
                    kept.len()
                ));
            }
            if !problems.is_empty() {
                found += 1;
                violations.push(Violation {
                    invariant: InvariantId::CandidateSetWithinLimit,
                    subject: format!("{}: {}", e.kind, e.from),
                    anchor: self.edge_anchor(e),
                    detail: problems.join(" · "),
                });
            }
        }
        self.finish(
            InvariantId::CandidateSetWithinLimit,
            checked.len(),
            skipped,
            found,
            residuals,
        )
    }

    // ── ⑥ 잔여가 실재하는 좌표에 결박 ────────────────────────────────────────

    fn residual_anchored(
        &self,
        violations: &mut Vec<Violation>,
        residuals: &mut Vec<Residual>,
    ) -> Outcome {
        let units: Vec<(Anchor, &Residual)> = self
            .view
            .residuals()
            .iter()
            .map(|r| (Anchor::At(r.bound_to()[0].clone()), r))
            .collect();
        let (checked, skipped) = self.select(units);

        let mut found = 0;
        for r in &checked {
            let ghosts: Vec<String> = r
                .bound_to()
                .iter()
                .filter(|c| !self.anchored.contains(&c.symbol))
                .map(|c| c.symbol.short())
                .collect();
            if !ghosts.is_empty() {
                found += 1;
                violations.push(Violation {
                    invariant: InvariantId::ResidualAnchored,
                    subject: format!("Residual{{{}}}", r.reason.label()),
                    anchor: Anchor::At(r.bound_to()[0].clone()),
                    detail: format!(
                        "결박 좌표가 이 그래프에 없다 — {} · 잔여가 유령이 된다",
                        ghosts.join(" · ")
                    ),
                });
            }
        }
        self.finish(InvariantId::ResidualAnchored, checked.len(), skipped, found, residuals)
    }

    // ── ⑦ 결박 색인이 가리키는 실체 ──────────────────────────────────────────

    fn index_resolves(
        &self,
        violations: &mut Vec<Violation>,
        residuals: &mut Vec<Residual>,
    ) -> Outcome {
        let units: Vec<(Anchor, &crate::view::BindingIndexEntry)> = self
            .view
            .binding_index()
            .iter()
            .map(|e| {
                let anchor = self
                    .view
                    .node(&e.target)
                    .and_then(|n| n.anchor.coord().cloned())
                    .map_or(Anchor::Coordless, Anchor::At);
                (anchor, e)
            })
            .collect();
        let (checked, skipped) = self.select(units);

        let mut found = 0;
        for e in &checked {
            if !self.view.intent_entities().contains(&e.binding) {
                found += 1;
                violations.push(Violation {
                    invariant: InvariantId::BindingIndexResolves,
                    subject: e.binding.to_string(),
                    anchor: self
                        .view
                        .node(&e.target)
                        .and_then(|n| n.anchor.coord().cloned())
                        .map_or(Anchor::Coordless, Anchor::At),
                    detail: "색인이 가리키는 결박이 의도 저장소에 없다 — 승인 노동의 유실이다"
                        .to_owned(),
                });
            }
        }
        self.finish(
            InvariantId::BindingIndexResolves,
            checked.len(),
            skipped,
            found,
            residuals,
        )
    }

    // ── ⑧ 낡음 등급이 이어달리기 규칙과 정합 ─────────────────────────────────

    fn freshness_consistent(
        &self,
        cascaded: &Cascade,
        violations: &mut Vec<Violation>,
        residuals: &mut Vec<Residual>,
    ) -> Outcome {
        // **모집단은 파생 노드다.** 입력이 없으면 이어달리기가 걸리지 않고,
        // 그 자리에서 "위반 0" 을 세면 검사가 아니라 장식이 된다.
        let units: Vec<(Anchor, &NodeInstance)> = self
            .view
            .nodes()
            .iter()
            .filter(|n| !n.inputs.is_empty() || !matches!(n.freshness, NodeFreshness::Live))
            .map(|n| (n.anchor.clone(), n))
            .collect();
        let (checked, skipped) = self.select(units);

        let mut found = 0;
        for n in &checked {
            // 예산 밖이라 등급을 계산하지 못한 자리는 위반이 아니라 잔여다.
            let Some(want) = cascaded.grades.get(&n.key) else { continue };
            if want != &n.freshness {
                found += 1;
                violations.push(Violation {
                    invariant: InvariantId::FreshnessConsistent,
                    subject: n.key.to_string(),
                    anchor: n.anchor.clone(),
                    detail: format!(
                        "저장된 등급은 `{}` 인데 전파 규칙은 `{}` 를 요구한다",
                        n.freshness.name(),
                        want.name()
                    ),
                });
            }
        }
        self.finish(
            InvariantId::FreshnessConsistent,
            checked.len(),
            skipped,
            found,
            residuals,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capable::CapabilityId;
    use crate::coord::{Discriminator, SymbolId};
    use crate::graph::Producer;
    use crate::judgment::ResidualReason;
    use crate::repo::{ObjectName, RepoId, RepoPath, Snapshot, TreeRef};
    use crate::symbol::SymbolKind;
    use crate::version::ExtractorVersion;
    use crate::view::{BindingIndexEntry, NodeInstance, ViewCoverage};
    use std::collections::BTreeSet;

    // ── 픽스처의 스키마 ──────────────────────────────────────────────────────
    //
    // **저장소의 `schema/graph.toml` 이 아니다.** 여덟 중 넷(④⑤⑥⑧)의 모집단이 그
    // 스키마에 없기 때문이다 — `inferred` 노드도, 저장되는 후보 집합도, 저장되는 잔여도,
    // 입력을 갖는 노드도 이 빌드에는 없다. 저장소 스키마로 그 넷을 시험하려면 **다른
    // 불변식을 함께 어겨야 하고**(예: `extracted` 노드를 `inferred` 라 적으면 ③ 이
    // 먼저 잡는다) 그러면 어느 검사가 잡았는지 말할 수 없게 된다.
    //
    // 그래서 **넷이 성립할 수 있는 스키마를 만들고 그 위에서 하나씩 깬다.** 이것이
    // 실물이 아니라는 사실은 `[f22.4].does_not_prove` 가 미리 적었고, 무엇이 실물이고
    // 무엇이 픽스처인지는 `doctor` 자신이 산출로 센다(`InvariantOutcome::NotBuilt`).
    const 스키마_원문: &str = r#"
schema_version = 1

[node.Symbol]
provenance = "extracted"
rust_type  = "SymbolNode"
key        = ["id"]
attrs = [
  { name = "name", type = "string", producer = "extractor", required = true },
  { name = "body", type = "digest", producer = "extractor", required = true },
]

[node.Binding]
provenance = "asserted"
rust_type  = "Binding"
key        = ["id"]
attrs = [
  { name = "note",     type = "string",   producer = "human",          required = true },
  { name = "bound_at", type = "snapshot", producer = "machine-record", required = true },
]

[node.Synthesis]
provenance = "inferred"
rust_type  = "Synthesis"
key        = ["id"]
attrs = [
  { name = "body", type = "string", producer = "agent", required = true },
]

[node.Ref]
provenance = "extracted"
rust_type  = "UnresolvedRef"
key        = ["id"]
attrs = [
  { name = "reason", type = "string", producer = "extractor", required = true },
]

[edge.BOUND_TO]
from        = "Binding"
to          = ["Symbol"]
cardinality = "many-to-one"
grade       = "exact"
provenance  = ["asserted"]
evidence    = "not_applicable"
snapshot    = "bound_at"

[edge.MAYBE_CALLS]
from        = "Symbol"
to          = ["Symbol"]
cardinality = "many-to-many"
grade       = "candidate"
provenance  = ["extracted"]
evidence    = "not_applicable"
snapshot    = "at"

[edge.DERIVES_FROM]
from        = "Synthesis"
to          = ["Symbol"]
cardinality = "many-to-many"
grade       = "contract"
provenance  = ["inferred"]
evidence    = "required_if_inferred:evidence"
snapshot    = "at"
"#;

    fn 스키마() -> GraphSchema {
        GraphSchema::parse(스키마_원문).expect("픽스처 스키마가 읽혀야 한다")
    }

    fn 트리() -> TreeRef {
        TreeRef::Committed(ObjectName::from_bytes([3; 20]))
    }

    fn 스냅샷() -> Snapshot {
        Snapshot::single(RepoId::new("r"), 트리())
    }

    fn 심볼_아이디(name: &str) -> SymbolId {
        SymbolId::compute(
            &RepoId::new("r"),
            &RepoPath::new("a.kt"),
            &[],
            name,
            &Discriminator::new(SymbolKind::Function, 0),
        )
    }

    fn 좌표(name: &str) -> Coord {
        Coord {
            repo: RepoId::new("r"),
            tree: 트리(),
            extractor: ExtractorVersion { grammar: "g", extractor: "e" },
            symbol: 심볼_아이디(name),
        }
    }

    fn 심볼_키(name: &str) -> NodeKey {
        NodeKey::new("Symbol", 심볼_아이디(name).to_hex())
    }

    fn 덮개() -> ViewCoverage {
        ViewCoverage::new()
            .holding("Symbol")
            .holding("Binding")
            .holding("Synthesis")
            .holding("Ref")
            .holding("BOUND_TO")
            .holding("MAYBE_CALLS")
            .holding("DERIVES_FROM")
            .holding(RESIDUAL_KIND)
            .holding(SCOPE_REDUCTION_KIND)
            .holding(BINDING_INDEX_KIND)
            .holding(DERIVED_KIND)
    }

    fn 결박_키() -> NodeKey {
        NodeKey::new("Binding", "b1")
    }

    fn 합성_키() -> NodeKey {
        NodeKey::new("Synthesis", "y1")
    }

    /// 성한 그래프 — **아래 픽스처 여덟이 전부 이것에서 한 자리만 바꾼다.**
    fn 성한() -> GraphView {
        let s1 = NodeInstance::new(심볼_키("f"), Provenance::Extracted, Anchor::At(좌표("f")))
            .with_attr("name", Producer::Extractor)
            .with_attr("body", Producer::Extractor);
        let s2 = NodeInstance::new(심볼_키("g"), Provenance::Extracted, Anchor::At(좌표("g")))
            .with_attr("name", Producer::Extractor)
            .with_attr("body", Producer::Extractor);
        let b1 = NodeInstance::new(결박_키(), Provenance::Asserted, Anchor::At(좌표("f")))
            .with_attr("note", Producer::Human)
            .with_attr("bound_at", Producer::MachineRecord);
        let y1 = NodeInstance::new(합성_키(), Provenance::Inferred, Anchor::At(좌표("f")))
            .with_attr("body", Producer::Agent)
            .with_evidence(vec![좌표("f")])
            .with_inputs(vec![심볼_키("f")]);
        let r1 = NodeInstance::new(
            NodeKey::new("Ref", "r1"),
            Provenance::Extracted,
            Anchor::At(좌표("f")),
        )
        .with_attr("reason", Producer::Extractor);

        let 결박_엣지 = EdgeInstance::one(
            "BOUND_TO",
            결박_키(),
            심볼_키("f"),
            ResolutionGrade::Exact,
            Provenance::Asserted,
            스냅샷(),
        );
        let 후보_엣지 = EdgeInstance {
            kind: "MAYBE_CALLS".to_owned(),
            from: 심볼_키("f"),
            to: EdgeTarget::Candidates {
                kept: vec![심볼_키("g")],
                total: 1,
                demoted_to: None,
            },
            grade: ResolutionGrade::Candidate,
            provenance: Provenance::Extracted,
            evidence: Vec::new(),
            at: 스냅샷(),
        };
        let 파생_엣지 = EdgeInstance::one(
            "DERIVES_FROM",
            합성_키(),
            심볼_키("f"),
            ResolutionGrade::Contract,
            Provenance::Inferred,
            스냅샷(),
        )
        .with_evidence(vec![좌표("f")]);

        let 저장된_잔여 = Residual::new(
            ResidualReason::ViaUnresolvedRef,
            "이 심볼에서 나가는 호출이 전부 해소된다",
            좌표("f"),
            Vec::new(),
            스냅샷(),
            "F07 참조 해소",
        );

        GraphView::new(스냅샷(), 덮개())
            .with_nodes(vec![s1, s2, b1, y1, r1])
            .with_edges(vec![결박_엣지, 후보_엣지, 파생_엣지])
            .with_residuals(vec![저장된_잔여])
            .with_binding_index(
                vec![BindingIndexEntry { target: 심볼_키("f"), binding: 결박_키() }],
                BTreeSet::from([결박_키()]),
            )
    }

    fn 전수(view: &GraphView) -> Diagnosis {
        run(&스키마(), view, DoctorScope::Full)
    }

    /// 어긋난 것이 **정확히 그 불변식 하나**인가.
    fn 잡혔다(view: &GraphView, 기대: InvariantId) {
        let d = 전수(view);
        assert!(d.coverage_gaps.is_empty(), "덮개에 구멍이 있다: {:?}", d.coverage_gaps);
        assert!(
            !d.violations.is_empty(),
            "불변식 {} 의 픽스처가 잡히지 않았다 — 이 자리는 검사가 아니라 장식이다",
            기대.number()
        );
        let 잡힌: BTreeSet<InvariantId> = d.violations.iter().map(|v| v.invariant).collect();
        assert_eq!(
            잡힌,
            BTreeSet::from([기대]),
            "불변식 {} 만 어겨야 하는데 {:?} 가 함께 잡혔다 — \
             여러 개를 함께 어기는 픽스처는 어느 검사가 잡았는지 말해 주지 않는다",
            기대.number(),
            잡힌
        );
    }

    // ── 음성 대조 — **성한 것을 잡지 않는가** ────────────────────────────────

    #[test]
    fn 성한_그래프에서는_위반이_0_이다() {
        // ①의 8/8 은 *"무엇이든 위반이라고 말하는"* 검사로도 만점을 받는다.
        let d = 전수(&성한());
        assert_eq!(d.violation_count(), 0, "성한 그래프에서 잡혔다: {:?}", d.violations);
        assert!(d.coverage_gaps.is_empty(), "{:?}", d.coverage_gaps);
    }

    #[test]
    fn 성한_그래프에서_여덟이_전부_모집단을_갖는다() {
        // 이것이 없으면 "위반 0" 이 *"검사를 안 돌렸다"* 와 구별되지 않는다.
        let d = 전수(&성한());
        for r in &d.invariants {
            match &r.outcome {
                InvariantOutcome::Checked(o) => {
                    assert!(o.checked > 0, "불변식 {} 의 모집단이 0 이다", r.number);
                }
                InvariantOutcome::NotBuilt => {
                    panic!("불변식 {} 이 이 픽스처에서 NotBuilt 다", r.number)
                }
            }
        }
        assert_eq!(d.checked_invariants(), 8);
    }

    // ── 깨진 픽스처 여덟 ─────────────────────────────────────────────────────

    #[test]
    fn 불변식_1_엣지의_양_끝_노드가_존재한다() {
        let 없는_심볼 = NodeKey::new("Symbol", 심볼_아이디("사라진").to_hex());
        let mut edges = 성한().edges().to_vec();
        edges[0] = EdgeInstance::one(
            "BOUND_TO",
            결박_키(),
            없는_심볼,
            ResolutionGrade::Exact,
            Provenance::Asserted,
            스냅샷(),
        );
        잡혔다(&성한().with_edges(edges), InvariantId::EdgeEndsExist);
    }

    #[test]
    fn 불변식_2_필수_속성이_있다() {
        let mut nodes = 성한().nodes().to_vec();
        nodes[0].attrs.remove("body");
        잡혔다(&성한().with_nodes(nodes), InvariantId::RegisteredAndRequired);
    }

    #[test]
    fn 불변식_3_생산자가_출처와_정합한다() {
        // `Symbol{name}` 을 에이전트가 만들었다고 적는다 — §3.4 의 정면 위반이다.
        let mut nodes = 성한().nodes().to_vec();
        nodes[0].attrs.insert("name".to_owned(), Producer::Agent);
        잡혔다(&성한().with_nodes(nodes), InvariantId::ProducerFitsProvenance);
    }

    #[test]
    fn 인스턴스_출처가_선언과_다르면_잡힌다() {
        // 불변식 3 의 다른 절반. 위 픽스처는 **생산자**가 선언과 다른 것을 재고
        // 이것은 **출처**가 다른 것을 잰다 — 스키마 진화가 만드는 형태다.
        let mut nodes = 성한().nodes().to_vec();
        nodes[0].provenance = Provenance::Asserted;
        잡혔다(&성한().with_nodes(nodes), InvariantId::ProducerFitsProvenance);
    }

    #[test]
    fn 불변식_4_inferred_는_근거를_싣는다() {
        let mut nodes = 성한().nodes().to_vec();
        for n in &mut nodes {
            if n.provenance == Provenance::Inferred {
                n.evidence_refs.clear();
            }
        }
        잡혔다(&성한().with_nodes(nodes), InvariantId::InferredCarriesEvidence);
    }

    #[test]
    fn 불변식_5_잘린_후보에는_미해소_참조가_붙는다() {
        // **절단이 조용해지는 자리다.** 후보 40 개 중 하나만 남기고 강등 기록을 지운다.
        let mut edges = 성한().edges().to_vec();
        edges[1].to = EdgeTarget::Candidates {
            kept: vec![심볼_키("g")],
            total: 40,
            demoted_to: None,
        };
        잡혔다(&성한().with_edges(edges), InvariantId::CandidateSetWithinLimit);
    }

    #[test]
    fn 불변식_6_잔여가_실재하는_좌표에_결박된다() {
        let 유령 = Residual::new(
            ResidualReason::NoLabel,
            "이 좌표에 가드 라벨이 있다",
            좌표("이_그래프에_없는_심볼"),
            Vec::new(),
            스냅샷(),
            "라벨 승인",
        );
        잡혔다(&성한().with_residuals(vec![유령]), InvariantId::ResidualAnchored);
    }

    #[test]
    fn 불변식_7_색인이_가리키는_결박이_의도_저장소에_있다() {
        // **R-21 이 재는 것** — 색인은 남았는데 실체가 없다.
        let view = 성한().with_binding_index(
            vec![BindingIndexEntry { target: 심볼_키("f"), binding: 결박_키() }],
            BTreeSet::new(),
        );
        잡혔다(&view, InvariantId::BindingIndexResolves);
    }

    #[test]
    fn 불변식_8_live_노드의_입력에_stale_이_없다() {
        let mut nodes = 성한().nodes().to_vec();
        for n in &mut nodes {
            if n.key == 심볼_키("f") {
                n.freshness = NodeFreshness::Stale;
            }
        }
        // 합성물은 `live` 로 남아 있다 — 입력이 낡았는데 그 사실이 흐르지 않았다.
        잡혔다(&성한().with_nodes(nodes), InvariantId::FreshnessConsistent);
    }

    // ── 여덟 밖의 검사 ───────────────────────────────────────────────────────

    #[test]
    fn 후보가_상한을_넘으면_잡힌다() {
        // 불변식 5 의 다른 절반. 위 픽스처는 **강등 기록 부재**를 재고 이것은 **상한**을
        // 잰다. 후보를 전부 실재하는 노드로 만들어야 불변식 1 이 함께 잡히지 않는다.
        let mut nodes = 성한().nodes().to_vec();
        let mut kept = Vec::new();
        for i in 0..=CANDIDATE_LIMIT {
            let name = format!("후보{i}");
            nodes.push(
                NodeInstance::new(심볼_키(&name), Provenance::Extracted, Anchor::At(좌표(&name)))
                    .with_attr("name", Producer::Extractor)
                    .with_attr("body", Producer::Extractor),
            );
            kept.push(심볼_키(&name));
        }
        let total = kept.len();
        let mut edges = 성한().edges().to_vec();
        edges[1].to = EdgeTarget::Candidates { kept, total, demoted_to: None };
        잡혔다(
            &성한().with_nodes(nodes).with_edges(edges),
            InvariantId::CandidateSetWithinLimit,
        );
    }

    #[test]
    fn 표본_밖은_이상_없음이_아니라_잔여다() {
        // 표본을 1 로 줄이면 나머지가 **잔여로** 나와야 한다. "위반 0" 만 나오면
        // 이 도구가 자기가 고발한 문제를 저지른다.
        let d = run(&스키마(), &성한(), DoctorScope::Sample { max: 1 });
        assert_eq!(d.violation_count(), 0);
        let 표본_잔여: Vec<&Residual> =
            d.residuals.iter().filter(|r| r.reason == ResidualReason::OutsideSample).collect();
        assert!(!표본_잔여.is_empty(), "표본이었다는 사실이 산출에 없다");
        assert!(표본_잔여.iter().all(|r| !r.bound_to().is_empty()));
        // 그리고 **보지 않은 수가 불변식마다 실린다.**
        assert!(d.invariants.iter().any(|r| matches!(
            &r.outcome,
            InvariantOutcome::Checked(o) if o.skipped > 0
        )));
    }

    #[test]
    fn 전수는_잔여를_남기지_않는다() {
        let d = 전수(&성한());
        assert!(!d.residuals.iter().any(|r| r.reason == ResidualReason::OutsideSample));
    }

    #[test]
    fn 담을_수_없는_불변식은_위반_0_이_아니라_능력_부재다() {
        // **이 검사가 이 모듈의 정체성이다.** 모집단이 없는데 "위반 0" 을 내면
        // `Finding 0` 과 "안 만들었음"이 같은 출력이 된다.
        let 덮개_없이 = ViewCoverage::new()
            .holding("Symbol")
            .holding("Binding")
            .holding("Synthesis")
            .holding("Ref")
            .holding("BOUND_TO")
            .holding("MAYBE_CALLS")
            .holding("DERIVES_FROM")
            .holding(RESIDUAL_KIND)
            .holding(SCOPE_REDUCTION_KIND)
            .absent(BINDING_INDEX_KIND, CapabilityId::new("F05", "projection-binding-index"))
            .holding(DERIVED_KIND);
        let view = GraphView::new(스냅샷(), 덮개_없이)
            .with_nodes(성한().nodes().to_vec())
            .with_edges(성한().edges().to_vec())
            .with_residuals(성한().residuals().to_vec());
        let d = 전수(&view);
        let 일곱 = d.invariants.iter().find(|r| r.number == 7).expect("불변식 7");
        assert_eq!(일곱.outcome, InvariantOutcome::NotBuilt);
        assert_eq!(일곱.absent.len(), 1);
        assert_eq!(일곱.absent[0].built_by, "F05");
    }

    #[test]
    fn 스키마가_자랐는데_뷰가_말하지_않으면_구멍으로_실린다() {
        // **음성 대조 자체가 낡는 것을 막는 자리다.** 라벨이 하나 늘었는데 뷰가
        // 그것을 담을 수 있는지 말하지 않으면, 그 라벨은 검사되지도 않고 부재로
        // 세어지지도 않는다 — 조용히 좁아진다.
        let 좁은_덮개 = ViewCoverage::new().holding("Symbol");
        let view = GraphView::new(스냅샷(), 좁은_덮개);
        let d = 전수(&view);
        assert!(d.coverage_gaps.contains(&"Binding".to_owned()));
        assert!(d.coverage_gaps.contains(&"MAYBE_CALLS".to_owned()));
        assert!(d.coverage_gaps.contains(&DERIVED_KIND.to_owned()));
    }
}
