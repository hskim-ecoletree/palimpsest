//! 봉투 — **모든 질의의 반환 타입.**
//!
//! > 답만 돌려주는 경로가 타입 수준에 존재하지 않는다. (stack §5.2)
//!
//! 모든 응답은 자기 답이 **어느 범위 위에서 계산됐는지**를 동반한다(DESIGN §4.2).
//! 백서 §6.3 의 *"하한임이 표시되어야 한다"* 가 문장이 아니라 데이터가 되는 지점이다.

use serde::{Deserialize, Serialize};

use crate::capable::{Capable, CapabilityId};
use crate::ledger::{Bucket, ExtractGrade, IdentityGrade};
use crate::repo::Snapshot;

/// 2층이 지금 다시 만들어지는 중인가 — DESIGN §12.7 의 스냅샷 격리 3번.
///
/// **답을 막지 않고 진행 중임을 싣는다**(§2.3 과 같은 규칙). 재구축은 질의를 거절할
/// 사유가 아니라 답에 붙는 사실이다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RebuildState {
    /// 재구축이 돌고 있지 않다 — 이 답은 가만히 선 2층 위에 있다.
    Settled,
    /// 재구축 중이다. 답은 열린 트랜잭션의 스냅샷 위에서 나왔고 섞이지 않았다(격리 1번).
    Rebuilding,
}

/// 2층이 얼마나 신선한가.
///
/// **낡음 감지기 자신이 낡을 수 있다**(DESIGN §12). 감지기가 3주 낡았으면 낡음 표시들도
/// 3주 낡았다는 사실이 응답에 붙어야 한다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProjectionFreshness {
    /// 이 답이 선 트리가 워킹트리와 일치하는가.
    ///
    /// **`bool` 이 아니라 [`Capable`] 이다.** 커밋 트리를 읽은 빌드는 워킹트리가 그것과
    /// 같은지 **모른다** — 알려면 워킹트리 머클이 필요하고 그것은 F01 §3.2 다.
    /// `false` 로 적으면 *"다르다"* 는 거짓말이 되고 `true` 로 적으면 더 나쁘다.
    pub matches_worktree: Capable<bool>,
    /// 2층이 이 답을 내는 동안 재구축 중이었는가.
    ///
    /// **`RebuildState` 가 아니라 [`Capable`] 인 이유가 위와 같다.** DESIGN §12.7 격리
    /// 3번은 재구축을 값으로 표시하라고 적었지만, 그것을 **관측할 경로가 이 빌드에 없다**
    /// — 재구축의 시작과 끝을 아는 것은 2층을 소유한 쪽이고 그것은 F05 다.
    ///
    /// [`RebuildState::Settled`] 로 고정하면 죽은 필드이자 거짓말이다. *"재구축 중이
    /// 아니다"* 는 관측이지 기본값이 아니고, 관측하지 않고 적으면 진짜로 재구축 중인
    /// 경우와 구별되지 않는다. `NotBuilt` 는 **참인 선언**이다 — *"이 빌드는 모른다."*
    ///
    /// 이 자리가 아예 없던 동안 그 사실은 산출에서 빠져 있었고, 빠진 것은 소비자가
    /// 셀 수 없다. 그것이 이 제품이 고발한 조용한 공백의 형태다(목표 §3.1).
    pub rebuild: Capable<RebuildState>,
    /// 2층이 이 스냅샷에서 만들어졌는가. 아니면 그 사실이 실린다.
    pub built_for_this_snapshot: bool,
    /// 2층에 들어 있는 심볼 수. 0 이면 "인덱스가 비어 있다"가 답에 실린다.
    pub symbols_indexed: usize,
}

/// 이 답이 무엇을 못 봤는가 — **공백을 데이터로 만든다.**
///
/// 예: *"이 엔드포인트에 닿는 경로"* 질의는 경로 집합과 함께
/// `{미해소 12, 범위 밖 3, L1 경유 2}` 를 반환한다(DESIGN §4.2).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Coverage {
    /// 해소하지 못한 참조 수.
    pub unresolved: usize,
    /// 관측 범위 밖이라 보지 않은 파일 수.
    pub out_of_scope_files: usize,
    /// 이 답이 경유한 가장 낮은 추출 등급.
    pub lowest_grade: ExtractGrade,
    /// 이 답이 선 정체성 등급.
    pub identity: IdentityGrade,
}

/// 무엇 때문에 잘렸는가 — **사유가 값이다.**
///
/// # 이 계열 다섯은 되읽힌다 — [`Envelope`] 와 다르다
///
/// [`Envelope`] 에 `Deserialize` 가 없는 이유는 [`CapabilitySet`] 이 [`CapabilityId`] 를
/// 싣고 그것이 **이 빌드에 박힌 상수**이기 때문이다. **절단에는 그 문제가 없다** —
/// 사유도 상한도 닫힌 열거이고 빌드의 사실을 담지 않는다. 그리고 **질의 로그가
/// 되읽혀야 한다**(F17 이 F05 부터 쌓인 것을 읽는다 · §5.3).
///
/// 넷을 가르지 않으면 *"자르긴 했다"* 밖에 말할 수 없고, 그러면 사용자가 **무엇을 올려야
/// 답이 완전해지는지** 모른다. 그것이 `LIMIT` 이 표현하지 못하는 바로 그것이다
/// (stack §2.3 의 결정적 이유).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ElisionReason {
    /// 후보 집합이 `K` 를 넘어 **그 가지를 버렸다.**
    CandidateOverflow,
    /// 경로 곱이 `B` 를 넘어 **탐색을 멈췄다.**
    PathProductExceeded,
    /// 깊이 상한 너머라 **가지 않았다.**
    DepthExceeded,
    /// 답이 담을 수 있는 노드 수를 넘었다.
    NodeMaxExceeded,
}

impl ElisionReason {
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::CandidateOverflow => "candidate_overflow",
            Self::PathProductExceeded => "path_product_exceeded",
            Self::DepthExceeded => "depth_exceeded",
            Self::NodeMaxExceeded => "node_max_exceeded",
        }
    }
}

/// 어느 상한인가 — [`crate::Budget`] 의 넷과 하나씩 짝이다.
///
/// **[`ElisionReason`] 과 다른 타입이다.** 사유는 *"무엇이 일어났나"* 이고 이것은
/// *"어느 손잡이를 돌리면 되나"* 다. 하나로 합치면 답이 사용자에게 처방을 못 준다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BudgetName {
    CandidateSetMax,
    PathProductMax,
    DepthMax,
    NodeMax,
}

impl BudgetName {
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::CandidateSetMax => "candidate_set_max",
            Self::PathProductMax => "path_product_max",
            Self::DepthMax => "depth_max",
            Self::NodeMax => "node_max",
        }
    }
}

/// 사유 하나와 그 건수.
///
/// # 왜 `(ElisionReason, usize)` 가 아닌가
///
/// F05 §5.2 는 벌거벗은 쌍으로 적었다. **이름을 붙여 가른다** — [`crate::Containment`]
/// 와 같은 자리다. 벌거벗은 쌍은 읽는 쪽이 `.0` 이 무엇인지 기억해야 하고, 기억은
/// 검사되지 않는다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Truncation {
    pub reason: ElisionReason,
    /// **건수다.** *"잘렸다"* 가 아니라 *"몇 개가 잘렸다"* 여야 사용자가 크기를 안다.
    pub count: usize,
}

/// 걸린 상한 하나와 그 값.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct LimitHit {
    pub limit: BudgetName,
    /// **그때 이 상한이 얼마였는가.** 값이 없으면 사용자가 얼마로 올려야 할지 모른다.
    /// `usize::MAX` 를 넣어 사실상 끈 경우에도 **그 값이 여기 실린다.**
    pub value: u64,
}

/// 예산에 걸려 잘린 것. **없어도 명시해야 한다.**
///
/// # 조용한 절단 금지가 타입으로 서는 자리 (stack §5.4)
///
/// [`Envelope`] 를 만들려면 이 값을 반드시 넘겨야 하고, 자를 것이 없으면
/// [`Elision::none`] 을 **명시적으로** 부른다. 기본값을 두지 않는 것이 요점이다 —
/// 기본값이 있으면 절단을 적는 것을 잊는 경로가 생긴다.
///
/// # 형태가 F05 §5.2 로 왔다
///
/// 옛 판은 `{dropped: usize, reasons: Vec<String>}` 이었다. **사유별 건수도 어느
/// 상한에 걸렸는지도 담지 못한다** — 문자열 목록은 세어지지 않고, 상한의 값이 없으면
/// 사용자가 무엇을 얼마로 올려야 하는지 모른다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Elision {
    /// 사유별 건수. **비어 있으면 자른 것이 없다.**
    pub truncated: Vec<Truncation>,
    /// 어느 상한에 얼마나. **비어 있으면 아무 상한에도 안 걸렸다.**
    pub limits_hit: Vec<LimitHit>,
}

impl Elision {
    /// **자른 것이 없다고 명시한다.** 이 함수를 부르는 것 자체가 기록이다.
    #[must_use]
    pub const fn none() -> Self {
        Self { truncated: Vec::new(), limits_hit: Vec::new() }
    }

    /// 사유 하나를 `n` 건 더한다. **같은 사유는 합쳐진다** — 목록이 아니라 계수기다.
    pub fn push(&mut self, reason: ElisionReason, n: usize) {
        if let Some(t) = self.truncated.iter_mut().find(|t| t.reason == reason) {
            t.count += n;
        } else {
            self.truncated.push(Truncation { reason, count: n });
        }
    }

    /// 상한 하나가 걸렸다고 적는다. **같은 상한을 두 번 적지 않는다.**
    pub fn hit(&mut self, limit: BudgetName, value: u64) {
        if !self.limits_hit.iter().any(|l| l.limit == limit) {
            self.limits_hit.push(LimitHit { limit, value });
        }
    }

    /// 잘린 것의 **총 건수.** 화면이 이것을 쓴다.
    #[must_use]
    pub fn dropped(&self) -> usize {
        self.truncated.iter().map(|t| t.count).sum()
    }

    /// 사유 하나의 건수 — **없으면 0 이고, 그것이 정확한 값이다.**
    #[must_use]
    pub fn count_of(&self, reason: ElisionReason) -> usize {
        self.truncated.iter().find(|t| t.reason == reason).map_or(0, |t| t.count)
    }

    #[must_use]
    pub fn is_none(&self) -> bool {
        self.truncated.is_empty() && self.limits_hit.is_empty()
    }
}

/// 이 빌드가 실제로 산출할 수 있는 것.
///
/// **소비자가 능력 유무를 질의 없이 안다**(stack §5.3). 미구축 능력이 목록에 서 있고,
/// 그래서 빈 답이 "없음"인지 "안 만듦"인지 소비자가 판별할 수 있다.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize)]
pub struct CapabilitySet {
    /// 이 빌드가 답하는 것.
    pub built: Vec<String>,
    /// 자리는 있으나 아직 만들지 않은 것 — 기능 번호와 함께.
    pub not_built: Vec<CapabilityId>,
}

impl CapabilitySet {
    #[must_use]
    pub fn new(built: Vec<String>, not_built: Vec<CapabilityId>) -> Self {
        Self { built, not_built }
    }
}

/// 대장 참조 — 답에 실리는 요약.
///
/// 대장 전체를 매 응답에 실으면 컨텍스트를 잡아먹는다([R-11](../../../docs/plan/00-risks.md#r-11)).
/// 요약 한 줄을 싣고 상세는 `pal ledger` 로 옮긴다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LedgerRef {
    pub files_total: usize,
    pub parsed: usize,
    pub partial: usize,
    pub unsupported: usize,
    pub unrecognized: usize,
    /// 결박이 성립하지 않는 언어 수. 0 이 아니면 그 사실이 답마다 실린다.
    pub unbindable_languages: usize,
}

impl LedgerRef {
    /// 대장에서 요약을 뽑는다.
    #[must_use]
    pub fn of(ledger: &crate::ledger::Ledger) -> Self {
        let c = ledger.counts();
        let n = |b: Bucket| c.get(&b).copied().unwrap_or(0);
        Self {
            files_total: ledger.total(),
            parsed: n(Bucket::Parsed),
            partial: n(Bucket::Partial),
            unsupported: n(Bucket::Unsupported),
            unrecognized: n(Bucket::Unrecognized),
            unbindable_languages: ledger.unbindable_languages().len(),
        }
    }
}

/// 모든 질의의 반환 타입.
///
/// # 되읽지 않는다 — 봉투는 산출이다
///
/// `Serialize` 만 있다. [`CapabilitySet`] 이 [`CapabilityId`] 를 싣고 그것이
/// `&'static str` — **이 빌드에 박힌 상수**이기 때문이다. 밖에서 온 봉투의 능력 목록을
/// 이 빌드의 상수로 되읽으면 *"다른 빌드가 못 만든 것"* 과 *"내가 못 만든 것"* 이
/// 구별되지 않는다. 소비자는 JSON 스키마로 읽는다.
///
/// **필드를 전부 넘겨야 만들 수 있다.** `Default` 도 빌더도 두지 않는다 — 하나라도
/// 빠뜨릴 수 있는 경로가 생기면 그것이 곧 조용한 답이 나가는 경로다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Envelope<T> {
    pub answer: T,
    pub snapshot: Snapshot,
    pub projection: ProjectionFreshness,
    pub coverage: Coverage,
    pub capabilities: CapabilitySet,
    pub ledger: LedgerRef,
    /// **`Elision::none()` 이라도 명시적으로 넘어온다.**
    pub elision: Elision,
}

impl<T> Envelope<T> {
    /// 봉투를 씌운다. 인자 일곱이 곧 계약이다.
    pub const fn new(
        answer: T,
        snapshot: Snapshot,
        projection: ProjectionFreshness,
        coverage: Coverage,
        capabilities: CapabilitySet,
        ledger: LedgerRef,
        elision: Elision,
    ) -> Self {
        Self { answer, snapshot, projection, coverage, capabilities, ledger, elision }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 자른_것이_없어도_명시된다() {
        let e = Elision::none();
        assert!(e.is_none());
        assert_eq!(e.dropped(), 0);
        assert!(e.truncated.is_empty() && e.limits_hit.is_empty());
    }

    #[test]
    fn 자른_것에는_사유와_건수가_붙는다() {
        let mut e = Elision::none();
        e.push(ElisionReason::CandidateOverflow, 7);
        e.hit(BudgetName::CandidateSetMax, 32);
        assert!(!e.is_none());
        assert_eq!(e.dropped(), 7);
        assert_eq!(e.count_of(ElisionReason::CandidateOverflow), 7);
        // **다른 사유는 0 이고 그것이 정확한 값이다** — 넷을 다 적는 구현은
        // 아무것도 안 재고 있다(`[f05.1.pass]` ③).
        assert_eq!(e.count_of(ElisionReason::DepthExceeded), 0);
        assert_eq!(e.limits_hit.len(), 1, "다른 상한이 함께 섰다");
    }

    #[test]
    fn 같은_사유는_합쳐지고_같은_상한은_두_번_안_선다() {
        // 목록이 아니라 계수기다 — 목록이면 같은 사유가 여러 줄로 서고 건수가 안 세어진다.
        let mut e = Elision::none();
        e.push(ElisionReason::DepthExceeded, 2);
        e.push(ElisionReason::DepthExceeded, 3);
        e.hit(BudgetName::DepthMax, 3);
        e.hit(BudgetName::DepthMax, 3);
        assert_eq!(e.truncated.len(), 1);
        assert_eq!(e.count_of(ElisionReason::DepthExceeded), 5);
        assert_eq!(e.limits_hit.len(), 1);
    }

    #[test]
    fn 사유_넷과_상한_넷이_서로_다른_값이다() {
        // 뭉개면 `[f05.1.pass]` ③ 의 표가 전부 같은 값이 되고, 그 표는 아무것도 안 잰다.
        use std::collections::BTreeSet;
        let reasons: BTreeSet<&str> = [
            ElisionReason::CandidateOverflow,
            ElisionReason::PathProductExceeded,
            ElisionReason::DepthExceeded,
            ElisionReason::NodeMaxExceeded,
        ]
        .into_iter()
        .map(ElisionReason::name)
        .collect();
        assert_eq!(reasons.len(), 4);
        let limits: BTreeSet<&str> = [
            BudgetName::CandidateSetMax,
            BudgetName::PathProductMax,
            BudgetName::DepthMax,
            BudgetName::NodeMax,
        ]
        .into_iter()
        .map(BudgetName::name)
        .collect();
        assert_eq!(limits.len(), 4);
    }

    #[test]
    fn 재구축_자리는_settled_로_고정되지_않는다() {
        // DESIGN §12.7 격리 3번의 자리다. 관측 경로가 없는 빌드가 `Settled` 를 적으면
        // **재구축 중이 아니라고 말한 것**이 되고, 그것은 관측이 아니라 기본값이다.
        // `NotBuilt` 만이 참이다 — 그리고 그 사실이 직렬화된 산출에 실려야 한다.
        let p = ProjectionFreshness {
            matches_worktree: Capable::not_built(CapabilityId::new("F01", "worktree-state")),
            rebuild: Capable::not_built(CapabilityId::new("F05", "rebuild-progress")),
            built_for_this_snapshot: true,
            symbols_indexed: 3,
        };
        let json = serde_json::to_value(&p).unwrap();
        assert_eq!(json["rebuild"]["not_built"]["capability"]["feature"], "F05");
        assert!(!p.rebuild.is_present());
    }

    #[test]
    fn 능력_집합은_미구축을_기능번호와_함께_싣는다() {
        let c = CapabilitySet::new(
            vec!["symbol.resolve".into()],
            vec![CapabilityId::new("F15", "judgment")],
        );
        assert_eq!(c.not_built[0].feature, "F15");
    }
}
