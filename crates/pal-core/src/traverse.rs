//! 예산을 들고 다니는 넓이 우선 탐색 — **F05 §5.2. 이 실행기가 존재하는 이유다.**
//!
//! > 잘랐다는 사실, 왜 잘랐는지, 몇 건인지가 전부 결과에 실린다.
//! > **이 코드가 남의 질의 언어로는 표현되지 않는다.**
//!
//! # 여기 저장 기술이 없다
//!
//! 이웃을 **함수로 받는다.** 2층(`redb`)을 아는 것은 `pal-query` 이고, 이 크레이트는
//! 저장 기술에 의존하지 않는다(stack §4.1). 그리고 그 형태 덕에 **이 절단 규칙이
//! 저장소 없이 시험된다** — `[f05.1]` 이 `[f05.2]` 와 독립인 이유가 이것이다.
//!
//! # 넓이 우선인 이유는 [`crate::cascade`] 와 같다
//!
//! 깊이 우선이면 같은 노드가 먼 경로로 먼저 닿아 예산에 걸리고, 그러면 예산이 그래프의
//! 모양이 아니라 **순회 순서**를 잰다.
//!
//! # 건수의 정의 — **손으로 셀 수 있어야 한다** (`[f05.1.pass]` ③)
//!
//! *"자르긴 했다"* 는 검사되지 않는다. 넷의 건수를 여기서 못 박는다:
//!
//! | 사유 | 건수 |
//! |---|---|
//! | [`ElisionReason::CandidateOverflow`] | `K` 를 넘겨 버린 **가지의 수** |
//! | [`ElisionReason::PathProductExceeded`] | `B` 를 넘긴 가지 **하나** + 그 순간 **아직 펼치지 않은 대기열의 노드 수** |
//! | [`ElisionReason::DepthExceeded`] | 깊이 상한 **너머에 있어 안 간** 서로 다른 노드의 수 |
//! | [`ElisionReason::NodeMaxExceeded`] | 답이 차서 **못 담은** 서로 다른 노드의 수 |
//!
//! 둘째가 왜 저렇게 정의되는가: `B` 에 걸리면 §5.2 가 *"탐색 중단"* 이라 적었고, 중단하면
//! **대기열에 남은 것들도 안 간다.** 그것을 안 세면 *"한 건 잘랐다"* 가 되고 실제로는
//! 훨씬 많이 안 본 것이다 — 그것이 이 제품이 고발하는 조용한 절단이다.

use std::collections::{BTreeSet, VecDeque};

use crate::budget::Budget;
use crate::envelope::{BudgetName, Elision, ElisionReason};

/// 이웃 하나 — **후보 수를 함께 준다.**
///
/// # 왜 `candidates` 가 필요한가
///
/// 해소 등급이 `Candidate` 인 엣지는 **엣지 N 개가 아니라 후보 집합 하나**로 저장된다
/// ([`crate::ResolutionGrade::Candidate`]). 그 집합의 크기가 곧 이 홉이 경로 곱에 싣는
/// 값이고, 그것 없이는 `B` 가 잴 것이 없다.
///
/// **유일 해소(`Exact`·`Scoped`)는 `candidates = 1` 이다** — 0 이 아니다. 0 이면 곱이
/// 0 이 되어 어떤 상한도 안 걸리고, 그 순간 `B` 는 꺼진 예산이다.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Step<N> {
    pub to: N,
    /// 이 홉의 후보 수. 유일 해소면 **1**.
    pub candidates: usize,
}

impl<N> Step<N> {
    /// 유일 해소 한 홉.
    #[must_use]
    pub const fn exact(to: N) -> Self {
        Self { to, candidates: 1 }
    }

    /// 후보 집합 한 홉.
    #[must_use]
    pub const fn candidates(to: N, candidates: usize) -> Self {
        Self { to, candidates }
    }
}

/// 예산을 들고 걷는다. **닿은 노드를 방문 순서로 낸다** (`start` 를 포함한다).
///
/// 자른 것은 `elision` 에 **쌓인다** — 이 함수는 `elision` 을 비우지 않는다.
/// 여러 질의가 한 봉투에 실릴 수 있고, 그때 절단은 합산되어야 한다.
///
/// # 왜 `Budget` 을 요구하는가
///
/// 안 주고 부를 수 있는 경로가 없다 — `[f05.1.pass]` ④. `Budget` 에 `Default` 가
/// 없으므로 부르는 쪽이 넷을 명시해야 하고, 그 넷이 답에 실린다.
pub fn traverse<N, F>(start: &N, budget: &Budget, elision: &mut Elision, neighbors: F) -> Vec<N>
where
    N: Ord + Clone,
    F: Fn(&N) -> Vec<Step<N>>,
{
    let mut seen: BTreeSet<N> = BTreeSet::new();
    let mut out: Vec<N> = Vec::new();
    let mut queue: VecDeque<(N, usize, u64)> = VecDeque::new();

    seen.insert(start.clone());
    // **`node_max` 가 0 이면 출발점도 안 담긴다.** 그것이 정확한 값이고, 그 사실이
    // `NodeMaxExceeded` 1 건으로 실린다 — 빈 답과 구별되어야 한다.
    if out.len() < budget.node_max {
        out.push(start.clone());
    } else {
        elision.push(ElisionReason::NodeMaxExceeded, 1);
        elision.hit(BudgetName::NodeMax, budget.node_max as u64);
    }
    queue.push_back((start.clone(), 0, 1));

    let mut halted = false;
    while let Some((node, depth, product)) = queue.pop_front() {
        let steps = neighbors(&node);

        if depth >= budget.depth_max {
            // **깊이 너머는 「안 간」 것이지 「없는」 것이 아니다.** 서로 다른 노드만 센다 —
            // 같은 노드로 가는 두 엣지를 둘로 세면 건수가 그래프의 모양이 아니라
            // 엣지의 수를 잰다.
            let beyond: BTreeSet<&N> =
                steps.iter().map(|s| &s.to).filter(|n| !seen.contains(*n)).collect();
            if !beyond.is_empty() {
                elision.push(ElisionReason::DepthExceeded, beyond.len());
                elision.hit(BudgetName::DepthMax, budget.depth_max as u64);
            }
            continue;
        }

        for step in steps {
            if step.candidates > budget.candidate_set_max {
                // **이 가지를 버린다** — 탐색은 계속된다(§5.2 의 `continue`).
                elision.push(ElisionReason::CandidateOverflow, 1);
                elision.hit(BudgetName::CandidateSetMax, budget.candidate_set_max as u64);
                continue;
            }
            let next_product = product.saturating_mul(step.candidates.max(1) as u64);
            if next_product > budget.path_product_max {
                // **탐색을 멈춘다**(§5.2 의 `break`). 그리고 **대기열에 남은 것도 안 간다** —
                // 그것을 안 세면 「한 건 잘랐다」가 거짓이 된다.
                elision.push(ElisionReason::PathProductExceeded, 1 + queue.len());
                elision.hit(BudgetName::PathProductMax, budget.path_product_max);
                halted = true;
                break;
            }
            if !seen.insert(step.to.clone()) {
                continue;
            }
            if out.len() >= budget.node_max {
                elision.push(ElisionReason::NodeMaxExceeded, 1);
                elision.hit(BudgetName::NodeMax, budget.node_max as u64);
                continue;
            }
            out.push(step.to.clone());
            queue.push_back((step.to, depth + 1, next_product));
        }
        if halted {
            break;
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::budget::{CANDIDATE_LIMIT, PROVISIONAL_PATH_PRODUCT_MAX};

    /// `0 → 1 → 2 → … → n`. 유일 해소만 있는 사슬.
    fn 사슬(n: u32) -> impl Fn(&u32) -> Vec<Step<u32>> {
        move |k: &u32| if *k < n { vec![Step::exact(k + 1)] } else { Vec::new() }
    }

    /// 가지가 `width` 인 나무. 노드 이름은 `(부모 * width) + i + 1`.
    fn 나무(width: u32, depth_stop: u32) -> impl Fn(&u32) -> Vec<Step<u32>> {
        move |k: &u32| {
            if *k >= depth_stop {
                return Vec::new();
            }
            (0..width).map(|i| Step::exact(k * width + i + 1)).collect()
        }
    }

    fn 넉넉한() -> Budget {
        Budget::new(CANDIDATE_LIMIT, PROVISIONAL_PATH_PRODUCT_MAX, 100, 10_000)
    }

    #[test]
    fn 넉넉하면_절단이_0_이다() {
        // ★ **`[f05.1.pass]` ② 다.** 늘 자르는 구현이 ③ 을 통과하므로 이것이 먼저 선다.
        let mut el = Elision::none();
        let got = traverse(&0, &넉넉한(), &mut el, 사슬(10));
        // **하한이다** — 그래프가 비면 절단 0 은 아무것도 말하지 않는다.
        assert_eq!(got.len(), 11, "탐색이 아무 데도 안 갔다");
        assert!(el.is_none(), "넉넉한데 잘랐다: {el:?}");
        assert!(el.truncated.is_empty() && el.limits_hit.is_empty());
    }

    #[test]
    fn 깊이만_낮추면_깊이만_걸린다() {
        // `[f05.1.pass]` ③ 의 표 셋째 줄. **다른 상한은 안 서야 한다.**
        let mut el = Elision::none();
        let b = Budget::new(CANDIDATE_LIMIT, PROVISIONAL_PATH_PRODUCT_MAX, 3, 10_000);
        let got = traverse(&0, &b, &mut el, 사슬(10));
        assert_eq!(got, vec![0, 1, 2, 3], "3 홉까지만 가야 한다");
        assert_eq!(el.count_of(ElisionReason::DepthExceeded), 1);
        assert_eq!(el.limits_hit.len(), 1, "다른 상한이 함께 섰다: {el:?}");
        assert_eq!(el.limits_hit[0].limit, BudgetName::DepthMax);
        assert_eq!(el.limits_hit[0].value, 3);
    }

    #[test]
    fn 노드_상한만_낮추면_노드_상한만_걸린다() {
        // 가지 셋짜리 나무. 깊이는 넉넉하고 노드만 4 로 막는다.
        let mut el = Elision::none();
        let b = Budget::new(CANDIDATE_LIMIT, PROVISIONAL_PATH_PRODUCT_MAX, 100, 4);
        let got = traverse(&0, &b, &mut el, 나무(3, 3));
        assert_eq!(got.len(), 4, "노드 상한을 넘겼다");
        // 뿌리의 이웃 셋 중 셋이 담기고, 그 뒤로 담기지 못한 것이 세어진다.
        assert!(el.count_of(ElisionReason::NodeMaxExceeded) > 0);
        assert_eq!(el.count_of(ElisionReason::DepthExceeded), 0);
        assert_eq!(el.limits_hit.len(), 1);
        assert_eq!(el.limits_hit[0].limit, BudgetName::NodeMax);
    }

    #[test]
    fn 후보_상한만_낮추면_그_가지만_버리고_탐색은_계속된다() {
        // `K` 를 넘는 가지 하나와 유일 해소 하나. **후보 쪽만 버려진다.**
        let mut el = Elision::none();
        let b = Budget::new(2, PROVISIONAL_PATH_PRODUCT_MAX, 100, 10_000);
        let got = traverse(
            &0,
            &b,
            &mut el,
            |k: &u32| {
                if *k == 0 {
                    vec![Step::candidates(1, 5), Step::exact(2)]
                } else {
                    Vec::new()
                }
            },
        );
        assert_eq!(got, vec![0, 2], "버린 가지가 답에 실렸거나 탐색이 멈췄다");
        assert_eq!(el.count_of(ElisionReason::CandidateOverflow), 1);
        assert_eq!(el.count_of(ElisionReason::PathProductExceeded), 0, "멈추면 안 된다");
        assert_eq!(el.limits_hit.len(), 1);
        assert_eq!(el.limits_hit[0].limit, BudgetName::CandidateSetMax);
        assert_eq!(el.limits_hit[0].value, 2);
    }

    #[test]
    fn 경로_곱만_낮추면_탐색이_멈추고_남은_대기열이_세어진다() {
        // 뿌리에서 후보 3 짜리 가지 셋. `B = 2` 면 첫 가지에서 이미 넘는다.
        //
        // **손으로 세면**: 넘긴 가지 하나 + 그 순간 대기열 0 = **1**.
        let mut el = Elision::none();
        let b = Budget::new(CANDIDATE_LIMIT, 2, 100, 10_000);
        let got = traverse(&0, &b, &mut el, |k: &u32| {
            if *k == 0 { vec![Step::candidates(1, 3), Step::exact(2)] } else { Vec::new() }
        });
        assert_eq!(got, vec![0], "멈추지 않았다");
        assert_eq!(el.count_of(ElisionReason::PathProductExceeded), 1);
        assert_eq!(el.count_of(ElisionReason::CandidateOverflow), 0);
        assert_eq!(el.limits_hit.len(), 1);
        assert_eq!(el.limits_hit[0].limit, BudgetName::PathProductMax);
    }

    #[test]
    fn 멈출_때_대기열에_남은_것이_건수에_들어간다() {
        // ★ **이것이 「한 건 잘랐다」가 거짓이 되는 자리다.**
        //
        // `0` 의 이웃이 `1`·`2` 이고 둘 다 유일 해소 — 대기열에 둘이 들어간다.
        // `1` 을 펼칠 때 후보 곱이 넘으면 **`2` 도 안 간다.**
        // 손으로 세면: 넘긴 가지 1 + 대기열에 남은 `2` 하나 = **2**.
        let mut el = Elision::none();
        let b = Budget::new(CANDIDATE_LIMIT, 4, 100, 10_000);
        let got = traverse(&0, &b, &mut el, |k: &u32| match k {
            0 => vec![Step::exact(1), Step::exact(2)],
            1 => vec![Step::candidates(3, 5)],
            _ => Vec::new(),
        });
        assert_eq!(got, vec![0, 1, 2]);
        assert_eq!(
            el.count_of(ElisionReason::PathProductExceeded),
            2,
            "대기열에 남은 것을 안 셌다 — 조용한 절단이다"
        );
    }

    #[test]
    fn 유일_해소는_경로_곱을_안_올린다() {
        // `candidates = 1` 이 아니라 0 이면 곱이 0 이 되어 `B` 가 꺼진다.
        // 여기서는 반대로 **1 이라서 안 올라간다**는 것을 잰다.
        let mut el = Elision::none();
        let b = Budget::new(CANDIDATE_LIMIT, 1, 100, 10_000);
        let got = traverse(&0, &b, &mut el, 사슬(5));
        assert_eq!(got.len(), 6, "유일 해소만 있는데 경로 곱에 걸렸다");
        assert_eq!(el.count_of(ElisionReason::PathProductExceeded), 0);
    }

    #[test]
    fn 같은_노드로_가는_두_엣지는_깊이_초과를_둘로_세지_않는다() {
        // 건수가 그래프의 모양이 아니라 엣지 수를 재면 안 된다.
        let mut el = Elision::none();
        let b = Budget::new(CANDIDATE_LIMIT, PROVISIONAL_PATH_PRODUCT_MAX, 1, 10_000);
        let _ = traverse(&0, &b, &mut el, |k: &u32| match k {
            0 => vec![Step::exact(1)],
            1 => vec![Step::exact(2), Step::exact(2)],
            _ => Vec::new(),
        });
        assert_eq!(el.count_of(ElisionReason::DepthExceeded), 1);
    }

    #[test]
    fn 이미_본_노드는_깊이_초과로_안_세어진다() {
        // 순환에서 뒤로 가는 엣지는 *"안 간 것"* 이 아니라 *"이미 본 것"* 이다.
        let mut el = Elision::none();
        let b = Budget::new(CANDIDATE_LIMIT, PROVISIONAL_PATH_PRODUCT_MAX, 1, 10_000);
        let _ = traverse(&0, &b, &mut el, |k: &u32| match k {
            0 => vec![Step::exact(1)],
            1 => vec![Step::exact(0)],
            _ => Vec::new(),
        });
        assert!(el.is_none(), "이미 본 노드를 잘랐다고 적었다: {el:?}");
    }

    #[test]
    fn 절단은_쌓인다() {
        // 여러 질의가 한 봉투에 실리면 절단이 합산되어야 한다.
        let mut el = Elision::none();
        let b = Budget::new(CANDIDATE_LIMIT, PROVISIONAL_PATH_PRODUCT_MAX, 1, 10_000);
        let _ = traverse(&0, &b, &mut el, 사슬(5));
        let _ = traverse(&0, &b, &mut el, 사슬(5));
        assert_eq!(el.count_of(ElisionReason::DepthExceeded), 2);
        assert_eq!(el.limits_hit.len(), 1, "같은 상한이 두 번 섰다");
    }
}
