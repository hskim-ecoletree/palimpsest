//! 적시 제시의 반환 타입 — **빈 자리가 이 모듈의 요점이다.**
//!
//! # 아직 만들지 않은 산출을 어떻게 표현하는가 ([옛 F11 §2.1](../../../docs/plan/disposal-map.md))
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
///
/// **[graph-node] `Symbol`** — `schema/graph.toml`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, serde::Deserialize)]
pub struct SymbolNode {
    pub id: SymbolId,
    pub path: RepoPath,
    /// 파일 → 클래스 → (중첩)클래스. **[`id`] 의 성분이다**(F03 §3.2).
    ///
    /// # 왜 유도하지 않고 싣는가
    ///
    /// 체인은 [`crate::FileGraph::contains`] 에서 나오는데 **2 층에는 파일 그래프가
    /// 없다.** 여기 없으면 `pal touch` 가 `OrderService.cancel` 을 낼 수 없고,
    /// 그것이 F03 §1 이 이 기능의 목적으로 적은 좌표 그 자체다.
    ///
    /// 최상위 선언에서 **빈 목록인 것이 정확한 값이다** — 담는 것이 없다.
    ///
    /// [`id`]: SymbolNode::id
    pub container: Vec<String>,
    pub name: String,
    pub kind: SymbolKind,
    /// **변했는가**에 답하는 값. 정체성(`id`)과 다른 축이다.
    pub body: BodyDigest,
    pub span: Span,
    /// 이 심볼의 정체성을 얼마나 믿을 수 있는가. **언어 단위가 아니라 심볼 단위다**([R-22]).
    pub identity: IdentityGrade,
}

/// 이 결박이 **어디에** 걸렸나 — `touch` 한 좌표인가, 다른 좌표인가.
///
/// # 이 값이 없으면 두 문장이 한 줄로 뭉친다 (`[f11.pass]` ⑤)
///
/// *"내 코드에 걸린 결정"* 과 *"남의 코드에 걸렸는데 나를 지켜보는 결정"* 은 **고치러
/// 갈 자리가 다르다.** 뭉치면 사람이 엉뚱한 좌표를 연다.
///
/// 그리고 후자가 [F11 이 실제로 겨냥한 형태]다 — `corpus/tasks/recurrence.toml` 이
/// *"재발의 지배적 형태는 「몰랐다」가 아니라 **「경로 하나를 빠뜨렸다」**"* 라고 적었다.
///
/// [F11 이 실제로 겨냥한 형태]: ../../../corpus/tasks/recurrence.toml
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case", tag = "at")]
pub enum BoundTarget {
    /// 결박의 대상이 **지금 만진 좌표 그 자체**다.
    Here,
    /// 대상이 **다른 좌표**다 — 이 좌표는 그 결박의 **감시 집합**에 들어 있을 뿐이다.
    ///
    /// 감시 집합의 크기는 결박이 선언한 [`crate::Radius`] 가 정한다. **선언이지
    /// 계산이 아니고**, 그 사실이 [`BoundItem::Note::radius`] 로 함께 실린다.
    Elsewhere {
        symbol: SymbolId,
        /// 그 좌표가 어디인가. **못 찾은 것도 값이다**(ADR-0005).
        place: TargetPlace,
    },
}

/// 다른 좌표의 자리.
///
/// **`Option` 이 아니다** — `None` 이 *"2층에 없다"* 인지 *"안 찾아봤다"* 인지 구별되지
/// 않고, 앞의 것은 `Orphaned` 와 같은 사건이라 화면에 떠야 한다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum TargetPlace {
    Known { path: RepoPath, container: Vec<String>, name: String, line: u32 },
    /// 2층에 그 심볼이 없다 — **결박은 있는데 대상이 사라졌다.**
    Gone,
}

/// 이 좌표에 걸린 것.
///
/// F11 §2 는 일곱 종류를 적었다 — 결정 · 대체 이력 · 라벨 · 계획 항목 · 결함 계보 ·
/// 잔여 · 범위 축소. **여기 하나뿐인 것은 나머지를 만들 기능이 아직 없어서다.**
/// 만들 수 없는 변형을 미리 두면 그것이 곧 "있는데 안 나오는" 상태가 된다.
/// 그 판단의 전문은 `corpus/criteria.toml` `[f11].bounditem_ruling` 에 있다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum BoundItem {
    /// 사람이 손으로 건 조각 — **S3 가 만드는 유일한 종류다.**
    Note {
        binding: crate::binding::BindingId,
        /// **무엇이** 걸렸나 — 조각의 본문(`note`)과 다른 축이다. 같은 결정이 여러
        /// 좌표에 걸리면 이 이름이 같다.
        subject: crate::entity::EntityId,
        note: String,
        /// 두 축 — 코드 신선도와 계보.
        status: crate::binding::BindingStatus,
        /// 무엇까지 지켜보나 — **선언이지 계산이 아니다**(F09 §3).
        ///
        /// *"이 결정은 `symbol` 반경에서 live"* 는 *"이 결정은 유효하다"* 와 **다른
        /// 문장**이고, 그 차이가 산출에 남는 것이 F09 의 요구다.
        radius: String,
        /// 감시 집합의 크기. 반경 이름만으로는 `files:3` 이 몇 개를 지켜보는지 모른다.
        watch: usize,
        /// 언제 걸었나 — **표시용이다. 앵커가 아니다**(F09 §6).
        ///
        /// 낡음 판정은 이 값을 안 읽는다. **정렬은 읽는다** — 정렬은 화면의 일이고
        /// [F11 §3.3] 이 *"결박 시점이 최근인 것 우선"* 을 사실 기반 정렬로 요구했다.
        bound_at_time: crate::binding::BoundTime,
        /// **여기인가 다른 좌표인가.**
        at: BoundTarget,
    },
}

/// 두 이름이 「가깝다」의 갈래 — **점수가 아니라 예/아니오다** (`[f11.pass]` ③).
///
/// [F11 §5] 가 정렬에서 관련도 점수를 기각했다: *"점수는 근거가 없다."*
/// **편집거리 임계도 같은 종류다** — 임계를 우리가 정하는 순간 그 숫자가 산출을
/// 정한다. 그래서 임계가 없는 두 술어만 쓴다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NearKind {
    /// 소문자화하고 `_` 와 `-` 를 지운 형태가 **같다** — `resolve_claim_branch` ↔
    /// `resolveClaimBranch`.
    Spelling,
    /// 한쪽이 다른 쪽의 **부분 문자열**이다(대소문자 무시) — [F11 §4] 의 *"부분 매칭"*.
    Substring,
}

impl NearKind {
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Spelling => "표기",
            Self::Substring => "부분",
        }
    }
}

/// 표기만 다른 같은 이름인가를 재기 위한 정규화.
fn 표기_지움(s: &str) -> String {
    s.chars().filter(|c| *c != '_' && *c != '-').flat_map(char::to_lowercase).collect()
}

/// 입력과 후보 이름이 가까운가. **가깝지 않으면 `None`.**
///
/// 두 술어가 다 참이면 [`NearKind::Spelling`] 이 이긴다 — 표기 차이가 더 강한 사실이다.
#[must_use]
pub fn near_kind(input: &str, name: &str) -> Option<NearKind> {
    if input == name {
        // 같은 이름은 「가까운 후보」가 아니다 — 그것은 답이다.
        return None;
    }
    if 표기_지움(input) == 표기_지움(name) {
        return Some(NearKind::Spelling);
    }
    let (a, b) = (input.to_lowercase(), name.to_lowercase());
    // **빈 입력은 모든 이름의 부분 문자열이다.** 그것을 후보로 세면 전부가 후보가 된다.
    if !a.is_empty() && (b.contains(&a) || a.contains(&b)) {
        return Some(NearKind::Substring);
    }
    None
}

/// 가까운 이름 하나 — **하나를 고르지 않는다**(P6 · `[f11.pass]` ③).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NearName {
    pub name: String,
    pub kind: NearKind,
}

/// 결박을 화면에 세울 순서 — **사실 기반 정렬이다. 점수가 아니다**([F11 §3.3]).
///
/// ```text
/// ① stale · orphaned · 판정 불가   ← 낡은 것이 안 보이면 이 기능의 존재 이유가 사라진다
/// ② superseded (계보)
/// ③ 결박 시점이 최근인 것
/// ④ 결박 id                        ← 같은 저장소가 같은 순서를 낸다
/// ```
///
/// **②가 「pending」이다.** [F11 §3.3] 은 `stale > pending > live` 라고 적었는데 이
/// 빌드의 계보 축은 [`crate::Lineage`] 둘(`Current` · `Superseded`)이고, *"대체됐다"*
/// 가 그 자리다. **없는 상태를 만들어 셋을 맞추지 않는다.**
#[must_use]
pub fn 정렬_열쇠(item: &BoundItem) -> (u8, u8, i64, String) {
    let BoundItem::Note { binding, status, bound_at_time, .. } = item;
    let 신선도 = match &status.code {
        // **`stale` 과 `orphaned` 가 같은 칸이다** — 둘 다 *"코드가 움직였다"* 이고,
        // 사람이 봐야 하는 것에는 차이가 없다. **다르다는 사실은 지워지지 않는다** —
        // 화면이 둘을 다른 문장으로 낸다(F09 §5).
        crate::binding::CodeFreshness::Stale { .. }
        | crate::binding::CodeFreshness::Orphaned { .. } => 0,
        crate::binding::CodeFreshness::Undeterminable { .. } => 1,
        crate::binding::CodeFreshness::Live => 3,
    };
    let 계보 = match &status.lineage {
        crate::binding::Lineage::Superseded { .. } => 0,
        crate::binding::Lineage::Current => 1,
    };
    let 시각 = match bound_at_time {
        crate::binding::BoundTime::Committed { epoch_secs } => -*epoch_secs,
        // 워킹트리는 **커밋보다 최근이다** — 아직 커밋되지 않았다.
        crate::binding::BoundTime::Worktree => i64::MIN,
        // 모르는 것은 뒤로. **0(1970년)으로 접지 않는다.**
        crate::binding::BoundTime::Unrecorded => i64::MAX,
    };
    (신선도, 계보, 시각, binding.as_str().to_owned())
}

/// **낡은 것은 상한에 걸려도 실린다** ([F11 §3.3] · `[f11.pass]` ④).
///
/// > `stale` 은 항상 보인다 — 상한에 걸려도 `stale` 항목은 우선 포함.
/// > **낡은 것이 안 보이면 이 기능의 존재 이유가 사라진다.**
#[must_use]
pub fn 낡았나(item: &BoundItem) -> bool {
    let BoundItem::Note { status, .. } = item;
    !matches!(status.code, crate::binding::CodeFreshness::Live)
}

/// 이 심볼이 하는 것 — **F07(참조 해소)이 채운다.**
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SymbolFacts {
    pub callers: usize,
    pub callees: usize,
}

/// 내가 모르는 것 — **F08 이 채운다.**
///
/// **[graph-node] `UnresolvedRef`** — `schema/graph.toml` (`status = "not_built"`)
///
/// **변형이 없는 것이 스키마가 요구하는 상태다.** 자리만 만든 노드의 타입이 거주
/// 가능하면 누군가 빈 값을 채워 넣을 수 있고, 그 순간 *"안 만들었음"* 과 *"없음"* 이
/// 같은 출력이 된다. `xtask` 의 스키마 정합 검사가 이 거주 불가능성을 센다.
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
    /// 이 좌표에 **걸린** 것 — F09
    pub bindings: Capable<Vec<BoundItem>>,
    /// ★ 이 좌표를 **지켜보는** 것 — 대상이 **다른 좌표**인 결박들 (`[f11.pass]` ⑤).
    ///
    /// # 왜 `bindings` 와 한 목록이 아닌가
    ///
    /// *"내 코드에 걸린 결정"* 과 *"남의 코드에 걸렸는데 나를 지켜보는 결정"* 은
    /// **고치러 갈 자리가 다르다.** 그리고 후자가 재발의 지배적 형태다 —
    /// `recurrence.toml` 이 *"경로 하나를 빠뜨렸다"* 로 이름 붙였다.
    ///
    /// 실체는 의도 저장소의 `WATCH` 색인이고 F09 가 증분 갱신을 위해 세운 것이다.
    /// **같은 색인이 이 질의를 받는다** — 반경이 `symbol` 이면 이 목록이 비고,
    /// 그것이 *"아무도 나를 안 지켜본다"* 라는 정확한 답이다.
    pub watching: Capable<Vec<BoundItem>>,
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
    ///
    /// **`near` 가 비어 있는 것과 목록이 있는 것은 다른 답이다** — 앞은 *"가까운 것도
    /// 없다"* 이고 뒤는 *"이것을 뜻했습니까"* 다. ⚠ **하나를 고르지 않는다**(P6).
    Unknown { name: String, near: Vec<NearName> },
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
