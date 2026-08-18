//! 판정 산출 — **3분할이고 `clean` 이 없다** ([옛 DESIGN §8](../../../docs/plan/disposal-map.md) · D9).
//!
//! ```text
//! Finding   — 위반이 확정된 것. 반례가 첨부된다. (존재 주장)
//! Residual  — 판정할 수 없는 것. 사유 + 무엇이 있으면 판정 가능한지.
//! OutOfScope— 범위 밖. 대장 참조.
//! ```
//!
//! # 이 모듈에 `Residual` 하나만 있는 이유
//!
//! `Finding` 은 **판정마다 반례의 형태가 다르다** — 감사의 반례는 경로이고
//! [`crate::doctor`] 의 반례는 위반한 노드·엣지다. 그래서 공통 타입을 미리 만들지 않고
//! 판정하는 자리가 자기 반례 타입을 짓는다. `OutOfScope` 는 *"대장 참조"* 이고
//! 그 참조는 이미 [`crate::Envelope`] 의 `ledger` · `coverage` 가 싣는다 —
//! **같은 것을 두 번 두지 않는다.**
//!
//! # 잔여는 목록이 아니라 결박물이다 ([옛 DESIGN §8.1](../../../docs/plan/disposal-map.md) · R6)
//!
//! 선행 하네스가 잔여 어휘를 갖고도 실패한 이유가 여기 있다 — *표시는 목록으로
//! 존재했고 목록은 아무도 다시 열지 않았다*([연구 E §4]). 그래서 이 타입은 **좌표에
//! 결박된다**. 그 좌표를 다시 만지는 순간 잔여가 사람이 있는 곳으로 온다(§11.3).
//!
//! **결박 좌표가 빈 잔여는 만들 수 없다.** [`Residual::new`] 가 첫 좌표를 별개 인자로
//! 받는 것이 그 강제의 형태다 — 빈 목록을 넘길 자리가 없다. 그것이 없으면
//! `doctor` 의 불변식 6(*잔여가 실재하는 좌표에 결박되어 있다*)이 자기 자신의 산출에서
//! 먼저 깨진다.

use serde::Serialize;

use crate::coord::Coord;
use crate::repo::Snapshot;

/// 왜 판정하지 못했는가.
///
/// # 어휘가 열거인 이유
///
/// [옛 DESIGN §8](../../../docs/plan/disposal-map.md) 이 사유 아홉을 **낱말로** 적었다. 문자열로 두면
/// 같은 사유가 두 이름을 갖고, 그 순간 *"사유별로 정렬되어 사람이 라벨 하나를 승인할
/// 때마다 줄어든다"* 는 §8 말미의 진척 표시가 성립하지 않는다.
///
/// # 여기에 둘이 더 있다 — **F22-4 가 더했고 근거가 게이트에 있다**
///
/// | 더한 것 | 어디가 요구하나 |
/// |---|---|
/// | [`Self::CascadeBudgetExceeded`] | §6.4-2 — *"예산에 걸리면 전파를 멈추는 것이 아니라 `Residual{사유=낡음 전파 예산 초과}` 를 낸다"* |
/// | [`Self::OutsideSample`] | §12.7 — *"표본이었다는 사실이 산출에 실린다"*. 표본 밖은 **"이상 없음"이 아니다** |
///
/// 둘 다 §8 의 목록에 없다. **목록이 닫힌 것이 아니라 §8 이 감사 판정을 두고 쓴
/// 목록이기 때문이다** — `doctor` 는 §8 이 쓰일 때 존재하지 않았다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ResidualReason {
    // ── 옛 DESIGN §8 의 아홉 ────────────────────────────────────────────────────
    /// 미해소 참조 경유.
    ViaUnresolvedRef,
    /// 라벨 없음.
    NoLabel,
    /// 언어 등급 미달.
    LanguageGradeBelow,
    /// 범위 밖 저장소 경유.
    ViaOutOfScopeRepo,
    /// 동적 디스패치.
    DynamicDispatch,
    /// `candidate` 집합 과다.
    CandidateSetTooLarge,
    /// 관측 낡음.
    ObservationStale,
    /// 검증 커버리지 미달.
    VerificationCoverageBelow,
    /// 탐색 예산 초과.
    SearchBudgetExceeded,
    // ── F22-4 가 더한 둘 ─────────────────────────────────────────────────────
    /// 낡음 전파 예산 초과 — §6.4-2.
    CascadeBudgetExceeded,
    /// 표본 밖이라 보지 않았다 — §12.7. **"이상 없음"과 다르다.**
    OutsideSample,
}

impl ResidualReason {
    /// 사람이 읽는 이름. **옛 DESIGN §8 의 낱말을 그대로 쓴다.**
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::ViaUnresolvedRef => "미해소 참조 경유",
            Self::NoLabel => "라벨 없음",
            Self::LanguageGradeBelow => "언어 등급 미달",
            Self::ViaOutOfScopeRepo => "범위 밖 저장소 경유",
            Self::DynamicDispatch => "동적 디스패치",
            Self::CandidateSetTooLarge => "candidate 집합 과다",
            Self::ObservationStale => "관측 낡음",
            Self::VerificationCoverageBelow => "검증 커버리지 미달",
            Self::SearchBudgetExceeded => "탐색 예산 초과",
            Self::CascadeBudgetExceeded => "낡음 전파 예산 초과",
            Self::OutsideSample => "표본 밖",
        }
    }
}

/// 판정할 수 없었던 것 하나 — **목록이 아니라 결박물이다**(§8.1).
///
/// 필드 다섯이 §8.1 의 다섯이다. 하나도 선택이 아니다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Residual {
    pub reason: ResidualReason,
    /// 판정 대상 술어 — **무엇을 판정하려 했는가.** 사유만 있으면 무엇이 판정되지
    /// 않았는지 알 수 없다.
    pub predicate: String,
    /// 결박 좌표. **비어 있을 수 없다** — [`Residual::new`] 를 보라.
    bound_to: Vec<Coord>,
    /// 발생 `Snapshot`. 잔여도 언제의 것인지를 진다.
    pub at: Snapshot,
    /// 해소 조건 — **무엇이 있으면 판정 가능한가.** 이것이 비면 잔여가 영구 변명이 된다.
    pub resolved_when: String,
}

impl Residual {
    /// 잔여를 만든다.
    ///
    /// # 첫 좌표가 별개 인자인 것이 강제다
    ///
    /// 좌표 목록 하나를 받으면 빈 목록을 넘길 수 있고, 그러면 결박 없는 잔여가
    /// 만들어진다. 결박 없는 잔여는 `doctor` 의 불변식 6 이 *유령*이라 부르는 것이고,
    /// 이 도구가 자기 산출에서 그것을 만들면 검사가 먼저 거짓이 된다.
    #[must_use]
    pub fn new(
        reason: ResidualReason,
        predicate: impl Into<String>,
        anchor: Coord,
        more: Vec<Coord>,
        at: Snapshot,
        resolved_when: impl Into<String>,
    ) -> Self {
        let mut bound_to = Vec::with_capacity(1 + more.len());
        bound_to.push(anchor);
        bound_to.extend(more);
        Self {
            reason,
            predicate: predicate.into(),
            bound_to,
            at,
            resolved_when: resolved_when.into(),
        }
    }

    /// 결박 좌표 전부. **언제나 하나 이상이다.**
    #[must_use]
    pub fn bound_to(&self) -> &[Coord] {
        &self.bound_to
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coord::{Discriminator, SymbolId};
    use crate::repo::{ObjectName, RepoId, RepoPath, TreeRef};
    use crate::symbol::SymbolKind;
    use crate::version::ExtractorVersion;

    fn 좌표(name: &str) -> Coord {
        Coord {
            repo: RepoId::new("r"),
            tree: TreeRef::Committed(ObjectName::from_bytes([1; 20])),
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

    fn 스냅샷() -> Snapshot {
        Snapshot::single(RepoId::new("r"), TreeRef::Committed(ObjectName::from_bytes([1; 20])))
    }

    #[test]
    fn 잔여는_좌표_없이_만들_수_없다() {
        // **인자에 빈 목록을 넘길 자리가 없다.** 그것이 강제의 형태다 —
        // 검사로 막으면 검사를 잊는 경로가 남는다.
        let r = Residual::new(
            ResidualReason::OutsideSample,
            "불변식 1 이 이 엣지에서 성립한다",
            좌표("f"),
            Vec::new(),
            스냅샷(),
            "전수 검사(`--full`)",
        );
        assert_eq!(r.bound_to().len(), 1);
    }

    #[test]
    fn 결박_좌표가_여럿이면_전부_남는다() {
        let r = Residual::new(
            ResidualReason::CascadeBudgetExceeded,
            "낡음 등급이 전파 규칙과 정합한다",
            좌표("a"),
            vec![좌표("b"), 좌표("c")],
            스냅샷(),
            "전파 깊이 예산을 올린다",
        );
        assert_eq!(r.bound_to().len(), 3);
    }

    #[test]
    fn 사유_어휘가_사람이_읽는_이름을_진다() {
        // §8 의 낱말 그대로여야 사유별 정렬이 문서와 같은 어휘로 읽힌다.
        assert_eq!(ResidualReason::CandidateSetTooLarge.label(), "candidate 집합 과다");
        assert_eq!(ResidualReason::OutsideSample.label(), "표본 밖");
    }
}
