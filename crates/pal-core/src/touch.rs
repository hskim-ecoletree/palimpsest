//! 적시 제시의 반환 타입 — **빈 자리가 이 모듈의 요점이다.**
//!
//! # 아직 만들지 않은 산출을 어떻게 표현하는가 ([F11 §2.1](../../../docs/plan/features/F11-touch.md))
//!
//! `touch` 는 P1 기능인데 P2·P3 의 산출을 자리로 갖는다. 그 자리를 빈 `Vec` 으로 두면
//! **"판정 결과 위반 없음"과 "감사 능력이 없음"이 같은 화면이 된다** — 이 제품이 고발한
//! 문제를 스스로 저지르는 것이다.
//!
//! 그래서 자리는 [`Capable`] 이 잡고, 값은 `NotBuilt{기능번호}` 다. 출력은 이렇게 된다:
//!
//! ```text
//! ■ 판정
//!   (이 빌드에는 감사 능력이 없습니다 — F15 미구축)
//! ```
//!
//! # 아래 타입들이 변형 없이 비어 있는 것은 의도다
//!
//! `BoundItem` · `UnresolvedRef` · `EffectSet` · `JudgmentSummary` 는 **자리를 잡되
//! 값을 만들 수 없다.** 그 기능이 도착하면 변형과 필드가 생기고, 그때까지 타입 시스템이
//! *"여기에 값이 있을 수 없다"* 를 보증한다. 자리표시 문자열이나 `()` 로 두면 그 보증이
//! 사라지고 누군가 빈 값을 채워 넣을 수 있게 된다.

use serde::Serialize;

use crate::capable::Capable;
use crate::coord::{BodyDigest, Coord, SymbolId};
use crate::ledger::IdentityGrade;
use crate::repo::RepoPath;
use crate::symbol::{Span, SymbolKind};

/// 2층에 사는 심볼 하나.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, serde::Deserialize)]
pub struct SymbolNode {
    pub id: SymbolId,
    pub path: RepoPath,
    pub name: String,
    pub kind: SymbolKind,
    /// **변했는가**에 답하는 값. 정체성(`id`)과 다른 축이다.
    pub body: BodyDigest,
    pub span: Span,
    /// 이 심볼의 정체성을 얼마나 믿을 수 있는가. **언어 단위가 아니라 심볼 단위다**([R-22]).
    pub identity: IdentityGrade,
}

/// 이 좌표에 걸린 것 — **F09~F12 가 채운다.**
///
/// 변형이 없다. 결박이 아직 존재하지 않으므로 값을 만들 수 없고, 그것이 정확한 상태다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum BoundItem {}

/// 이 심볼이 하는 것 — **F07(참조 해소)이 채운다.**
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SymbolFacts {
    pub callers: usize,
    pub callees: usize,
}

/// 내가 모르는 것 — **F08 이 채운다.**
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum UnresolvedRef {}

/// 효과 집합 — **F13 이 채운다.**
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum EffectSet {}

/// 판정 요약 — **F15 가 채운다.**
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum JudgmentSummary {}

/// `touch` 하나의 답.
///
/// **일곱 자리를 한 번에 반환하는 것이 요점이다**(F11 §2) — 사람은 *"이 코드에 관련된 게
/// 뭐가 있지"* 를 종류별로 묻지 않는다. 지금은 그중 다섯이 `NotBuilt` 이고, 그 사실이
/// [`crate::CapabilitySet`] 으로 응답마다 설명된다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TouchResult {
    pub target: Coord,
    /// **이것만 실제 값이다.** 2층이 붙었다는 증거다.
    pub symbol: SymbolNode,
    /// 이 좌표에 걸린 것 — F09
    pub bindings: Capable<Vec<BoundItem>>,
    /// 이 심볼이 하는 것 — F07
    pub facts: Capable<SymbolFacts>,
    /// 내가 모르는 것 — F08
    pub unresolved: Capable<Vec<UnresolvedRef>>,
    /// 효과 — F13
    pub effects: Capable<EffectSet>,
    /// 판정 — F15
    pub judgments: Capable<JudgmentSummary>,
}

/// `touch` 질의의 답 — **못 찾은 것도 답이다.**
///
/// 이름이 2층에 없을 때 오류를 내지 않는다. *"모른다"* 는 정상적인 답이고 봉투가 그
/// 답의 근거(무엇을 보았고 무엇을 안 보았는가)를 함께 싣는다. 오류로 처리하면 봉투가
/// 나가지 못하고, 그러면 사용자는 **왜** 못 찾았는지 알 수 없다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case", tag = "outcome")]
pub enum TouchAnswer {
    /// 유일하게 찾았다.
    ///
    /// **`Box` 인 것은 크기 때문이다.** 이 변형만 448바이트라 열거형 전체가 그만큼
    /// 커지고, 못 찾은 경우까지 그 비용을 낸다. 직렬화 형태는 바뀌지 않는다.
    Found(Box<TouchResult>),
    /// 후보가 여럿이다. **하나를 고르지 않는다** — 고르는 것은 에이전트의 일이다(P6).
    Ambiguous { name: String, candidates: Vec<SymbolNode> },
    /// 2층에 그 이름이 없다.
    Unknown { name: String },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capable::CapabilityId;

    #[test]
    fn 미구축_자리는_빈_배열로_직렬화되지_않는다() {
        // **이것이 S2 합격선 ② 다.** `[]` 가 나오면 "없음"과 "안 만듦"이 같아진다.
        let v: Capable<Vec<BoundItem>> =
            Capable::not_built(CapabilityId::new("F09", "binding"));
        let json = serde_json::to_string(&v).unwrap();
        assert!(json.contains("not_built"), "{json}");
        assert!(json.contains("F09"), "{json}");
        assert_ne!(json, "[]");
    }

    #[test]
    fn 값이_있는_자리는_그대로_실린다() {
        let v: Capable<SymbolFacts> = Capable::Present(SymbolFacts { callers: 9, callees: 14 });
        let json = serde_json::to_string(&v).unwrap();
        assert!(json.contains("\"callers\":9"), "{json}");
    }
}
