//! 낡음의 이어달리기 — **규칙이 없으면 전파가 멈춘다** ([옛 DESIGN §6.4](../../../docs/plan/disposal-map.md) · D29).
//!
//! > **모든 파생 노드는 자기 입력 좌표 집합에 결박된다. 입력 중 하나가 `stale` 이면
//! > 파생물은 `stale-derived` 다.**
//!
//! # 왜 이름이 `cascade` 인가
//!
//! 설계 문서의 낱말은 *"전파"* 이고 그 영어는 `propagate` 다. **`pal-core` 소스에서
//! 그 낱말을 쓸 수 없다** — stack §4.2 의 금지 어휘 `gate` 를 부분 문자열로 포함하고
//! `cargo xtask check` 가 그것을 잡는다. 어휘 금지가 설계 낱말과 부딪친 첫 자리이고,
//! 처분은 규칙을 느슨하게 하는 것이 아니라 **다른 이름을 쓰는 것**이다(`vocab.toml` 의
//! 허용 목록은 비어 있는 것이 정상 상태다).
//!
//! # 둘을 가르는 이유는 처분이 다르기 때문이다 (§6.4-1)
//!
//! | 등급 | 뜻 | 누가 닫나 |
//! |---|---|---|
//! | [`NodeFreshness::Live`] | 자기 감시 집합도 입력도 그대로 | — |
//! | [`NodeFreshness::Stale`] | **자기** 감시 집합이 변했다 | **사람** — 재판정 큐 |
//! | [`NodeFreshness::StaleDerived`] | 자기는 그대로인데 **입력이** 낡았다 | 대개 **기계** — 입력을 갱신하면 닫힌다 |
//!
//! 하나로 합치면 사람의 재판정 큐에 기계가 닫을 수 있는 것이 섞이고, 그 순간
//! [R-10](../../../docs/plan/00-risks.md#r-10) 의 승인 노동이 부풀어 오른다.
//!
//! # 예산에 걸리면 멈추는 것이 아니다 (§6.4-2)
//!
//! 파생 사슬은 길어질 수 있으므로 이어달리기에도 예산이 붙는다. **걸리면 잔여다** —
//! [`crate::ResidualReason::CascadeBudgetExceeded`]. 조용한 절단 금지가 여기에도 걸린다.
//!
//! **끄는 손잡이를 두지 않는다.** 예산은 값이고 손잡이는 [옛 DESIGN §10](../../../docs/plan/disposal-map.md)
//! 이 세는 협상 대상이다. 끌 수 있으면 그것이 게이트 오염의 가장 값싼 경로가 된다.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use serde::Serialize;

use crate::coord::Coord;
use crate::judgment::{Residual, ResidualReason};
use crate::view::{GraphView, NodeKey};

// **깊이 예산은 여기 없다.** `pal-core::budget` 한 곳이다(stack §5.5 · `[f05.1.pass]` ①).
// 재수출도 남기지 않는다 — 남기면 *"한 곳"* 이 두 곳이 된다.

/// 노드 하나의 낡음 등급 — §6.4 의 셋.
///
/// # `CodeFreshness` 와 다른 타입인 이유
///
/// [`crate::CodeFreshness`] 는 **결박 하나**가 자기 감시 집합을 현재 값에 댄 결과이고
/// `Orphaned`(좌표가 사라짐)를 갖는다. 이것은 **그래프 노드**가 파생 사슬 위에서 지는
/// 등급이다. [옛 F09 §2](../../../docs/plan/disposal-map.md) 는 둘을 한 열거로
/// 합칠 것을 적었지만, **결박의 입력이 낡는 경로가 이 빌드에 없다** — 합치면
/// `CodeFreshness` 에 아무도 만들지 못하는 변형이 하나 생기고 그것이 곧
/// *"있는데 안 나오는"* 자리다. 둘이 만나는 것은 F09 다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case", tag = "freshness")]
pub enum NodeFreshness {
    Live,
    /// 자기 감시 집합이 변했다. **이어달리기의 뿌리다.**
    Stale,
    /// 입력이 낡았다. **경유한 자리를 싣는다** — 어느 입력을 갱신하면 닫히는지가 그것이다.
    StaleDerived { via: Vec<NodeKey> },
}

impl NodeFreshness {
    /// 이 등급이 판정 입력 자격을 갖는가. **`live` 만 갖는다**(§6.4 의 표).
    #[must_use]
    pub const fn admissible(&self) -> bool {
        matches!(self, Self::Live)
    }

    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Stale => "stale",
            Self::StaleDerived { .. } => "stale-derived",
        }
    }
}

/// 이어달리기의 결과.
#[derive(Debug, Clone)]
pub struct Cascade {
    /// 규칙이 요구하는 등급. **저장된 등급과 대조하는 것이 불변식 8 이다.**
    pub grades: BTreeMap<NodeKey, NodeFreshness>,
    /// 예산에 걸려 계산하지 못한 자리 — **잔여로 나간다.**
    pub residuals: Vec<Residual>,
    /// 예산에 걸렸는데 **결박할 좌표를 찾지 못한** 자리.
    ///
    /// 잔여로 낼 수 없으므로(결박 없는 잔여는 유령이다) 이름 그대로 남긴다.
    /// 비어 있지 않으면 그것 자체가 산출에 실린다 — **조용히 사라지지 않는다.**
    pub unanchored_cutoff: Vec<NodeKey>,
}

/// 낡음을 파생 사슬로 흘린다.
///
/// 뿌리는 **저장된 등급이 [`NodeFreshness::Stale`] 인 노드**다 — 자기 감시 집합이
/// 변했다는 것은 코드를 봐야 알 수 있고, 그것은 이 함수의 몫이 아니라 F09 의 몫이다.
/// 여기서 하는 것은 *"뿌리가 이것들일 때 나머지가 무엇이 되어야 하는가"* 뿐이다.
#[must_use]
pub fn cascade(view: &GraphView, depth_budget: usize) -> Cascade {
    // 입력 → 그 입력을 쓰는 것들. 흐르는 방향이 이쪽이다.
    let mut dependents: BTreeMap<&NodeKey, Vec<&NodeKey>> = BTreeMap::new();
    for n in view.nodes() {
        for input in &n.inputs {
            dependents.entry(input).or_default().push(&n.key);
        }
    }

    let mut grades: BTreeMap<NodeKey, NodeFreshness> = BTreeMap::new();
    let mut roots: Vec<&NodeKey> = Vec::new();
    for n in view.nodes() {
        if matches!(n.freshness, NodeFreshness::Stale) {
            grades.insert(n.key.clone(), NodeFreshness::Stale);
            roots.push(&n.key);
        }
    }

    // 넓이 우선 — **가장 짧은 사슬로 재는 것이 예산의 뜻이다.** 깊이 우선이면 같은
    // 노드가 먼 경로로 먼저 닿아 예산에 걸리고, 그러면 예산이 그래프의 모양이 아니라
    // 순회 순서를 잰다.
    let mut seen: BTreeSet<&NodeKey> = roots.iter().copied().collect();
    let mut queue: VecDeque<(&NodeKey, usize)> = roots.iter().map(|k| (*k, 0)).collect();
    let mut cutoff: Vec<&NodeKey> = Vec::new();

    while let Some((key, depth)) = queue.pop_front() {
        let Some(next) = dependents.get(key) else { continue };
        for d in next {
            if !seen.insert(*d) {
                continue;
            }
            if depth + 1 > depth_budget {
                // **멈추는 것이 아니라 잔여다.** 등급을 적지 않고 자리를 남긴다.
                cutoff.push(*d);
                continue;
            }
            queue.push_back((*d, depth + 1));
        }
    }

    // 등급을 적는다. `via` 는 **낡은 입력**이다 — 무엇을 갱신하면 닫히는지가 그것이다.
    for n in view.nodes() {
        if grades.contains_key(&n.key) || !seen.contains(&n.key) || cutoff.contains(&&n.key) {
            continue;
        }
        let via: Vec<NodeKey> = n
            .inputs
            .iter()
            .filter(|i| seen.contains(*i) && !cutoff.contains(i))
            .cloned()
            .collect();
        if via.is_empty() {
            continue;
        }
        grades.insert(n.key.clone(), NodeFreshness::StaleDerived { via });
    }

    // 닿지 않은 것은 `live` 다. **예산에 걸린 자리는 여기서 빠진다** — 계산하지 못한
    // 것을 `live` 로 적으면 그것이 곧 "안 봤다"와 "이상 없다"가 같은 출력이 되는 것이다.
    for n in view.nodes() {
        if cutoff.contains(&&n.key) {
            continue;
        }
        grades.entry(n.key.clone()).or_insert(NodeFreshness::Live);
    }

    let (residuals, unanchored_cutoff) = cut_residuals(view, &cutoff, depth_budget);
    Cascade { grades, residuals, unanchored_cutoff }
}

/// 예산에 걸린 자리를 잔여로 바꾼다.
///
/// **결박 좌표를 셋 중에서 찾는다** — 걸린 노드 자신 → 그 입력 → 이어달리기의 뿌리.
/// §6.4 가 *"모든 파생 노드는 자기 입력 좌표 집합에 결박된다"* 고 적었으므로 둘째가
/// 언제나 있어야 하지만, **있어야 한다와 있다는 다르다.** 셋 다 비면 잔여를 만들지
/// 않고 이름으로 남긴다 — 결박 없는 잔여는 불변식 6 이 유령이라 부르는 것이다.
fn cut_residuals(
    view: &GraphView,
    cutoff: &[&NodeKey],
    depth_budget: usize,
) -> (Vec<Residual>, Vec<NodeKey>) {
    if cutoff.is_empty() {
        return (Vec::new(), Vec::new());
    }

    let mut anchors: Vec<Coord> = Vec::new();
    for key in cutoff {
        if let Some(c) = view.node(key).and_then(|n| n.anchor.coord()) {
            anchors.push(c.clone());
        }
    }
    if anchors.is_empty() {
        for key in cutoff {
            let Some(n) = view.node(key) else { continue };
            for input in &n.inputs {
                if let Some(c) = view.node(input).and_then(|x| x.anchor.coord()) {
                    anchors.push(c.clone());
                }
            }
        }
    }
    if anchors.is_empty() {
        for n in view.nodes() {
            if matches!(n.freshness, NodeFreshness::Stale) {
                if let Some(c) = n.anchor.coord() {
                    anchors.push(c.clone());
                }
            }
        }
    }

    if anchors.is_empty() {
        return (Vec::new(), cutoff.iter().map(|k| (*k).clone()).collect());
    }

    let first = anchors.remove(0);
    let residual = Residual::new(
        ResidualReason::CascadeBudgetExceeded,
        format!(
            "낡음 등급이 전파 규칙과 정합한다 — 사슬 {}마디 너머 {}개 자리",
            depth_budget,
            cutoff.len()
        ),
        first,
        anchors,
        view.at.clone(),
        "이어달리기 깊이 예산을 올리거나 파생 사슬을 줄인다",
    );
    (vec![residual], Vec::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::budget::PROVISIONAL_CASCADE_DEPTH;
    use crate::coord::{Discriminator, SymbolId};
    use crate::graph::Provenance;
    use crate::repo::{ObjectName, RepoId, RepoPath, Snapshot, TreeRef};
    use crate::symbol::SymbolKind;
    use crate::version::ExtractorVersion;
    use crate::view::{Anchor, NodeInstance, ViewCoverage};

    fn 스냅샷() -> Snapshot {
        Snapshot::single(RepoId::new("r"), TreeRef::Committed(ObjectName::from_bytes([7; 20])))
    }

    fn 좌표(name: &str) -> Coord {
        Coord {
            repo: RepoId::new("r"),
            tree: TreeRef::Committed(ObjectName::from_bytes([7; 20])),
            extractor: ExtractorVersion { grammar: "g", extractor: "e" },
            symbol: SymbolId::compute(
                &RepoId::new("r"),
                &RepoPath::new("a.kt"),
                &[],
                name,
                &Discriminator::new(SymbolKind::Function, 0),
            ),
        }
    }

    /// `뿌리 → a → b → c → d` 사슬 하나.
    fn 사슬(길이: usize) -> GraphView {
        let mut nodes = vec![
            NodeInstance::new(
                NodeKey::new("Symbol", "뿌리"),
                Provenance::Extracted,
                Anchor::At(좌표("뿌리")),
            )
            .with_freshness(NodeFreshness::Stale),
        ];
        let mut prev = NodeKey::new("Symbol", "뿌리");
        for i in 0..길이 {
            let key = NodeKey::new("Synthesis", format!("d{i}"));
            nodes.push(
                NodeInstance::new(
                    key.clone(),
                    Provenance::Inferred,
                    Anchor::At(좌표(&format!("d{i}"))),
                )
                .with_inputs(vec![prev.clone()]),
            );
            prev = key;
        }
        GraphView::new(스냅샷(), ViewCoverage::new()).with_nodes(nodes)
    }

    #[test]
    fn 입력이_낡으면_파생물은_stale_derived_다() {
        let c = cascade(&사슬(1), PROVISIONAL_CASCADE_DEPTH);
        let g = &c.grades[&NodeKey::new("Synthesis", "d0")];
        let NodeFreshness::StaleDerived { via } = g else {
            panic!("stale-derived 가 아니다: {g:?}");
        };
        assert_eq!(via, &vec![NodeKey::new("Symbol", "뿌리")]);
    }

    #[test]
    fn 자기가_낡은_것과_입력이_낡은_것은_다른_등급이다() {
        // **처분이 다르기 때문이다** — 앞은 사람의 큐, 뒤는 대개 기계가 닫는다.
        let c = cascade(&사슬(1), PROVISIONAL_CASCADE_DEPTH);
        assert_eq!(c.grades[&NodeKey::new("Symbol", "뿌리")], NodeFreshness::Stale);
        assert!(!c.grades[&NodeKey::new("Synthesis", "d0")].admissible());
    }

    #[test]
    fn 예산에_걸리면_멈추지_않고_잔여를_낸다() {
        // 사슬이 예산보다 한 마디 길다. **그 한 마디가 조용히 사라지지 않는다.**
        let c = cascade(&사슬(PROVISIONAL_CASCADE_DEPTH + 1), PROVISIONAL_CASCADE_DEPTH);
        assert_eq!(c.residuals.len(), 1, "예산 초과가 잔여로 나오지 않았다");
        assert_eq!(c.residuals[0].reason, ResidualReason::CascadeBudgetExceeded);
        assert!(!c.residuals[0].bound_to().is_empty());
        assert!(c.unanchored_cutoff.is_empty());
        // 예산 안쪽은 그대로 등급이 선다.
        let 마지막 = NodeKey::new("Synthesis", format!("d{PROVISIONAL_CASCADE_DEPTH}"));
        assert!(!c.grades.contains_key(&마지막), "예산 밖인데 등급이 적혔다");
    }

    #[test]
    fn 낡은_것이_없으면_전부_live_다() {
        let view = 사슬(2);
        let 성한 = GraphView::new(스냅샷(), ViewCoverage::new()).with_nodes(
            view.nodes()
                .iter()
                .cloned()
                .map(|n| n.with_freshness(NodeFreshness::Live))
                .collect(),
        );
        let c = cascade(&성한, PROVISIONAL_CASCADE_DEPTH);
        assert!(c.grades.values().all(NodeFreshness::admissible));
        assert!(c.residuals.is_empty());
    }
}
