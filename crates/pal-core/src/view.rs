//! `doctor` 가 볼 그래프 — **"저장된 그래프를 훑는다"가 이 빌드에서는 성립하지 않는다.**
//!
//! # 왜 뷰가 필요한가 (F22-4 의 판단 · 2026-08-12)
//!
//! [DESIGN §12.7](../../../docs/DESIGN.md) 은 `doctor` 를 *"저장된 그래프가 자기 규칙을
//! 지키는가"* 로 정의했다. 그런데 이 빌드의 저장소에는 **엣지 자리가 없다** — 2층은
//! `SYMBOL`·`BY_NAME` 둘뿐이고 의도 저장소가 결박과 그 역방향 색인을 갖는다. `Change`·
//! `Defect` 의 엣지 여섯은 `pal defect` 가 **계산만 하고 저장하지 않는다**. 그래서
//! *"저장된 것을 훑는" 코드를 그대로 쓰면 훑을 것이 없다.*
//!
//! 셋 중에서 골랐다.
//!
//! | 후보 | 왜 아닌가 |
//! |---|---|
//! | 엣지 저장 자리를 먼저 만든다 | **F05 의 것**이다. 채울 기능이 없는 자리를 만드는 것을 S2 가 이미 기각했다(그 판단의 근거는 S2 게이트) |
//! | `doctor` 가 2층·의도 저장소를 직접 읽는다 | `pal-core` 가 `redb` 에 의존하게 된다 — stack §4.1 이 기계로 막는다. 그리고 **깨진 그래프를 만들 방법이 없어진다**(저장 계층이 애초에 안 받는다) |
//! | **읽기 전용 뷰를 코어에 두고 채우는 쪽을 바깥에 둔다** | ← 택함 |
//!
//! 뷰는 **픽스처가 곧바로 만들 수 있고** 2층·의도 저장소가 그것을 채운다. `doctor` 는
//! 뷰와 스키마만 본다. 그래서 *"깨진 그래프"* 가 저장 계층을 우회해서 만들어질 수 있고,
//! 그것이 [f22.4] 합격선 ① 의 전제다.
//!
//! # 그리고 뷰는 **자기가 담을 수 없는 것을 선언한다** ([`ViewCoverage`])
//!
//! 이것이 이 모듈에서 가장 중요한 결정이다. 불변식 여덟 중 여섯은 이 빌드에 모집단이
//! 없다 — `inferred` 노드도 후보 집합도 저장된 잔여도 파생 사슬도 0 이다.
//! **모집단이 없는 검사가 "위반 0 건"을 내면 그것이 곧 `Finding 0` 과 "감사를 안
//! 만들었음"이 같은 출력이 되는 것**([목표 §3.1](../../../docs/plan/00-goals.md))이고,
//! 이 도구가 자기가 고발한 문제를 저지르는 것이다.
//!
//! 그래서 뷰가 라벨마다 *담을 수 있는가*를 [`Capable`] 로 선언하고, `doctor` 는
//! 모집단 0 을 **위반 0 이 아니라 능력 부재**로 낸다. 게이트가 손으로 세야 했던
//! *"실물에서 시험된 것 / 픽스처에서만 시험된 것"* 이 그래서 기계 산출이 된다.

use std::collections::{BTreeMap, BTreeSet};

use serde::Serialize;

use crate::capable::{Capable, CapabilityId};
use crate::cascade::NodeFreshness;
use crate::coord::{Coord, SymbolId};
use crate::graph::{Producer, Provenance, ResolutionGrade};
use crate::judgment::Residual;
use crate::repo::Snapshot;

/// 노드 하나를 가리키는 이름 — `(라벨, 인스턴스 식별자)`.
///
/// 라벨이 성분인 이유는 불변식 1·2 다. *"엣지가 가리키는 노드가 존재한다"* 는 식별자만
/// 대조해서는 답할 수 없다 — 다른 라벨의 같은 식별자가 존재를 만족시켜 버린다.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct NodeKey {
    pub label: String,
    pub id: String,
}

impl NodeKey {
    #[must_use]
    pub fn new(label: impl Into<String>, id: impl Into<String>) -> Self {
        Self { label: label.into(), id: id.into() }
    }
}

impl std::fmt::Display for NodeKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.label, self.id)
    }
}

/// 이 노드가 코드 좌표 위에 서는가.
///
/// # `Option<Coord>` 가 아닌 이유
///
/// *"좌표가 없다"* 와 *"좌표를 가질 수 없다"* 는 다르다. `Change`·`Actor` 는 후자다 —
/// 커밋은 심볼이 아니다. 그 차이가 **표본 규칙을 정한다**: 좌표 없는 것은 표본에서
/// 빼지 않는다. 빼면 그 사실을 결박할 자리가 없고, 결박 없는 잔여는 불변식 6 이
/// *유령*이라 부르는 것이다. **불변식 하나가 표본 정책을 강제하는 자리다.**
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Anchor {
    At(Coord),
    /// 코드 좌표를 가질 수 없다 — **표본에서 빼지 않는다.**
    Coordless,
}

impl Anchor {
    #[must_use]
    pub const fn coord(&self) -> Option<&Coord> {
        match self {
            Self::At(c) => Some(c),
            Self::Coordless => None,
        }
    }
}

/// 그래프에 실제로 들어 있는 노드 하나.
///
/// **스키마의 선언이 아니라 인스턴스다.** 스키마는 *"`Symbol` 의 속성 여섯이 전부
/// `extractor` 다"* 를 말하고 이것은 *"이 심볼이 실제로 무엇을 싣고 있는가"* 를 말한다.
/// 로딩 시점 검사(F22-1)가 앞의 것을 보고 `doctor` 가 뒤의 것을 본다 — 손상 · 부분 갱신 ·
/// 스키마 진화 · 중단된 트랜잭션은 **앞의 검사를 통과한다**(§12.7).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NodeInstance {
    pub key: NodeKey,
    /// 이 인스턴스가 싣고 있다고 주장하는 출처.
    pub provenance: Provenance,
    /// 실제로 실린 속성 — 이름 → 생산자.
    pub attrs: BTreeMap<String, Producer>,
    pub anchor: Anchor,
    /// `inferred` 노드가 실은 근거 — **불변식 4 의 모집단.** 다른 출처에서는 비는 것이 정상이다.
    pub evidence_refs: Vec<Coord>,
    /// 저장된 낡음 등급 — **불변식 8 이 이것을 전파 규칙에 댄다.**
    pub freshness: NodeFreshness,
    /// 이 파생 노드의 입력. 파생이 아니면 빈다(§6.4 *"모든 파생 노드는 자기 입력 좌표
    /// 집합에 결박된다"*).
    pub inputs: Vec<NodeKey>,
}

impl NodeInstance {
    /// 최소 형태 — 속성도 근거도 입력도 없고 `live` 다.
    #[must_use]
    pub fn new(key: NodeKey, provenance: Provenance, anchor: Anchor) -> Self {
        Self {
            key,
            provenance,
            attrs: BTreeMap::new(),
            anchor,
            evidence_refs: Vec::new(),
            freshness: NodeFreshness::Live,
            inputs: Vec::new(),
        }
    }

    #[must_use]
    pub fn with_attr(mut self, name: impl Into<String>, producer: Producer) -> Self {
        self.attrs.insert(name.into(), producer);
        self
    }

    #[must_use]
    pub fn with_evidence(mut self, refs: Vec<Coord>) -> Self {
        self.evidence_refs = refs;
        self
    }

    #[must_use]
    pub fn with_inputs(mut self, inputs: Vec<NodeKey>) -> Self {
        self.inputs = inputs;
        self
    }

    #[must_use]
    pub fn with_freshness(mut self, freshness: NodeFreshness) -> Self {
        self.freshness = freshness;
        self
    }
}

/// 엣지가 무엇을 가리키는가.
///
/// **`Candidate` 등급은 엣지 N 개가 아니라 후보 집합 하나다**([DESIGN §5.1](../../../docs/DESIGN.md)).
/// 그래서 여기 변형이 둘이고, 둘째가 상한 `K` 와 강등 기록을 함께 진다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EdgeTarget {
    One(NodeKey),
    /// 후보 집합 하나.
    Candidates {
        /// 실제로 저장된 후보. **불변식 5 가 `K` 와 댄다.**
        kept: Vec<NodeKey>,
        /// 절단 전의 후보 수. `kept.len()` 과 다르면 잘린 것이다.
        total: usize,
        /// 초과분에 대응하는 `UnresolvedRef`. **없으면 절단이 조용해진다**(§5.1).
        demoted_to: Option<NodeKey>,
    },
}

/// 그래프에 실제로 들어 있는 엣지 하나. **공통 넷을 값으로 진다.**
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct EdgeInstance {
    /// 엣지 타입 이름 — 스키마의 `[edge.X]` 의 `X`.
    pub kind: String,
    pub from: NodeKey,
    pub to: EdgeTarget,
    // ── 공통 넷 ──────────────────────────────────────────────────────────────
    pub grade: ResolutionGrade,
    pub provenance: Provenance,
    /// 근거. `inferred` 로 설 때 비면 저장 거부다(§5.2) — **불변식 2 가 인스턴스에서 본다.**
    pub evidence: Vec<Coord>,
    pub at: Snapshot,
}

impl EdgeInstance {
    /// 대상 하나를 가리키는 엣지.
    #[must_use]
    pub fn one(
        kind: impl Into<String>,
        from: NodeKey,
        to: NodeKey,
        grade: ResolutionGrade,
        provenance: Provenance,
        at: Snapshot,
    ) -> Self {
        Self {
            kind: kind.into(),
            from,
            to: EdgeTarget::One(to),
            grade,
            provenance,
            evidence: Vec::new(),
            at,
        }
    }

    #[must_use]
    pub fn with_evidence(mut self, evidence: Vec<Coord>) -> Self {
        self.evidence = evidence;
        self
    }

    /// 이 엣지가 가리키는 노드 전부 — 후보 집합이면 후보 전부.
    pub fn targets(&self) -> impl Iterator<Item = &NodeKey> {
        match &self.to {
            EdgeTarget::One(k) => std::slice::from_ref(k).iter(),
            EdgeTarget::Candidates { kept, .. } => kept.iter(),
        }
    }
}

/// 2층의 결박 색인 한 줄 — **불변식 7 의 모집단.**
///
/// [R-21](../../../docs/plan/00-risks.md#r-21) 이 여기 걸린다. 색인이 가리키는 실체가
/// 의도 저장소에 없으면 **승인 노동의 유실이 조용히 일어난 것**이다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BindingIndexEntry {
    /// 색인이 붙은 좌표.
    pub target: NodeKey,
    /// 그 좌표에 걸렸다고 색인이 말하는 결박.
    pub binding: NodeKey,
}

/// 이 뷰가 담을 수 있는 것 — **관측 범위 대장이 그래프에 대해 하는 일**(§4.1 과 같은 형태).
///
/// 담을 수 없는 라벨은 **어느 기능이 그것을 만드는지와 함께** 선언된다. 그것이
/// [`Capable`] 를 쓰는 이유이고, 그래서 모집단 0 이 *"이상 없음"* 으로 새지 않는다.
#[derive(Debug, Clone, Default)]
pub struct ViewCoverage {
    holds: BTreeMap<String, Capable<()>>,
}

impl ViewCoverage {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// 이 라벨·엣지 타입의 인스턴스를 담을 수 있다.
    #[must_use]
    pub fn holding(mut self, label: impl Into<String>) -> Self {
        self.holds.insert(label.into(), Capable::Present(()));
        self
    }

    /// 담을 수 없다 — **왜 못 담는지와 함께.**
    #[must_use]
    pub fn absent(mut self, label: impl Into<String>, capability: CapabilityId) -> Self {
        self.holds.insert(label.into(), Capable::not_built(capability));
        self
    }

    /// 이 라벨을 담을 수 있는가. **선언되지 않은 라벨은 `None` 이다** — 그것은
    /// 뷰의 공백이고 [`crate::doctor`] 가 그것을 산출에 남긴다.
    #[must_use]
    pub fn of(&self, label: &str) -> Option<&Capable<()>> {
        self.holds.get(label)
    }

    /// 선언된 라벨 전부.
    pub fn declared(&self) -> impl Iterator<Item = &String> {
        self.holds.keys()
    }
}

/// `doctor` 가 보는 그래프 전부. **읽기 전용이다.**
#[derive(Debug, Clone)]
pub struct GraphView {
    /// 이 그래프가 선 `Snapshot`. 잔여가 이것을 진다.
    pub at: Snapshot,
    nodes: Vec<NodeInstance>,
    edges: Vec<EdgeInstance>,
    /// 저장된 잔여 — **불변식 6 의 모집단.**
    residuals: Vec<Residual>,
    binding_index: Vec<BindingIndexEntry>,
    /// 의도 저장소가 실제로 갖고 있는 결박.
    intent_entities: BTreeSet<NodeKey>,
    coverage: ViewCoverage,
}

impl GraphView {
    #[must_use]
    pub fn new(at: Snapshot, coverage: ViewCoverage) -> Self {
        Self {
            at,
            nodes: Vec::new(),
            edges: Vec::new(),
            residuals: Vec::new(),
            binding_index: Vec::new(),
            intent_entities: BTreeSet::new(),
            coverage,
        }
    }

    #[must_use]
    pub fn with_nodes(mut self, nodes: Vec<NodeInstance>) -> Self {
        self.nodes = nodes;
        self
    }

    #[must_use]
    pub fn with_edges(mut self, edges: Vec<EdgeInstance>) -> Self {
        self.edges = edges;
        self
    }

    #[must_use]
    pub fn with_residuals(mut self, residuals: Vec<Residual>) -> Self {
        self.residuals = residuals;
        self
    }

    #[must_use]
    pub fn with_binding_index(
        mut self,
        index: Vec<BindingIndexEntry>,
        intent_entities: BTreeSet<NodeKey>,
    ) -> Self {
        self.binding_index = index;
        self.intent_entities = intent_entities;
        self
    }

    #[must_use]
    pub fn push_node(mut self, node: NodeInstance) -> Self {
        self.nodes.push(node);
        self
    }

    #[must_use]
    pub fn push_edge(mut self, edge: EdgeInstance) -> Self {
        self.edges.push(edge);
        self
    }

    #[must_use]
    pub fn nodes(&self) -> &[NodeInstance] {
        &self.nodes
    }

    #[must_use]
    pub fn edges(&self) -> &[EdgeInstance] {
        &self.edges
    }

    #[must_use]
    pub fn residuals(&self) -> &[Residual] {
        &self.residuals
    }

    #[must_use]
    pub fn binding_index(&self) -> &[BindingIndexEntry] {
        &self.binding_index
    }

    #[must_use]
    pub fn intent_entities(&self) -> &BTreeSet<NodeKey> {
        &self.intent_entities
    }

    #[must_use]
    pub const fn coverage(&self) -> &ViewCoverage {
        &self.coverage
    }

    /// 존재하는 노드 이름 전부 — **불변식 1 이 이것에 대조한다.**
    #[must_use]
    pub fn keys(&self) -> BTreeSet<&NodeKey> {
        self.nodes.iter().map(|n| &n.key).collect()
    }

    /// 이 그래프에 실재하는 심볼 좌표 전부 — **불변식 6 이 이것에 대조한다.**
    #[must_use]
    pub fn anchored_symbols(&self) -> BTreeSet<SymbolId> {
        self.nodes.iter().filter_map(|n| n.anchor.coord().map(|c| c.symbol)).collect()
    }

    /// 키로 노드 하나. 없으면 `None` — **그 없음이 불변식 1 의 위반이다.**
    #[must_use]
    pub fn node(&self, key: &NodeKey) -> Option<&NodeInstance> {
        self.nodes.iter().find(|n| &n.key == key)
    }
}
