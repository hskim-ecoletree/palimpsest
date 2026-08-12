//! 봉투 — **모든 질의의 반환 타입.**
//!
//! > 답만 돌려주는 경로가 타입 수준에 존재하지 않는다. (stack §5.2)
//!
//! 모든 응답은 자기 답이 **어느 범위 위에서 계산됐는지**를 동반한다(DESIGN §4.2).
//! 백서 §6.3 의 *"하한임이 표시되어야 한다"* 가 문장이 아니라 데이터가 되는 지점이다.

use serde::Serialize;

use crate::capable::{Capable, CapabilityId};
use crate::ledger::{Bucket, ExtractGrade, IdentityGrade};
use crate::repo::Snapshot;

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

/// 예산에 걸려 잘린 것. **없어도 명시해야 한다.**
///
/// # 조용한 절단 금지가 타입으로 서는 자리 (stack §5.4)
///
/// [`Envelope`] 를 만들려면 이 값을 반드시 넘겨야 하고, 자를 것이 없으면
/// [`Elision::none`] 을 **명시적으로** 부른다. 기본값을 두지 않는 것이 요점이다 —
/// 기본값이 있으면 절단을 적는 것을 잊는 경로가 생긴다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Elision {
    /// 잘린 항목 수.
    pub dropped: usize,
    /// 왜 잘렸는가. 비어 있으면 자른 것이 없다는 뜻이다.
    pub reasons: Vec<String>,
}

impl Elision {
    /// **자른 것이 없다고 명시한다.** 이 함수를 부르는 것 자체가 기록이다.
    #[must_use]
    pub const fn none() -> Self {
        Self { dropped: 0, reasons: Vec::new() }
    }

    #[must_use]
    pub fn dropped(n: usize, reason: impl Into<String>) -> Self {
        Self { dropped: n, reasons: vec![reason.into()] }
    }

    #[must_use]
    pub const fn is_none(&self) -> bool {
        self.dropped == 0
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
        assert_eq!(e.dropped, 0);
        assert!(e.reasons.is_empty());
    }

    #[test]
    fn 자른_것에는_사유가_붙는다() {
        let e = Elision::dropped(7, "후보 상한 K=32");
        assert!(!e.is_none());
        assert_eq!(e.reasons.len(), 1);
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
