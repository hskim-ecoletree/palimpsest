//! 사슬의 마디 — `Change` · `Actor` · `Defect` · `Journey`.
//!
//! **넷 다 새 요구가 아니다.** 다른 절이 이미 *반환한다*고 적었거나 *검사한다*고
//! 적었는데 타입이 없던 것이다([옛 DESIGN §1.1](../../../docs/plan/disposal-map.md) D26).
//!
//! # 넷의 처지가 다르다 — 그리고 그 차이가 여기 타입으로 있다
//!
//! | | 어떻게 서나 | 여기서 |
//! |---|---|---|
//! | `Change` · `Actor` | git 에서 **결정론적으로** 나온다 | 값이 선다 |
//! | `Defect` | 수정 커밋에서 **소급 결박**된다([T10ⓐ](../../../docs/gates/preflight.md)) | 값이 선다 — 단, 못 지목한 것을 **세어서 표시한다** |
//! | `Journey` | **저작 노동이다.** 진입점 좌표 0/3 으로 반증됐다(T10ⓑ) | **자리만.** 값을 만들 수 없다 |
//!
//! # 소급 결박이 줄이 아니라 심볼 위에서 일어나는 이유
//!
//! [T10](../../../docs/gates/preflight.md#t10--여정결함의-올라탈-곳)은 삭제된 **줄**을
//! blame 해서 도입 커밋을 지목했다. 그때는 그것이 유일한 수단이었다.
//!
//! **이 좌표계에서 줄은 좌표가 아니다.** [`Coord`](crate::Coord)에 `span` 이 없고
//! (옛 DESIGN §2.1) 줄 번호는 포매팅으로 움직인다. 줄 위에서 결박하면 [R-07] 이 지목한
//! 거짓 양성이 결함 계보 안에서 그대로 재생산된다 — 포매팅 커밋 하나가 도입 커밋으로
//! 지목된다.
//!
//! 그래서 여기서는 **`body_digest` 가 마지막으로 변한 조상**을 도입 커밋으로 본다.
//! 정규화가 이미 포매팅을 지웠으므로(F03) 그 커밋은 *의미를 바꾼* 커밋이다.

use serde::{Deserialize, Serialize};

use crate::coord::SymbolId;
use crate::repo::{RepoPath, Snapshot};

// ─────────────────────────────────────────────────────────────────────────────
// Actor
// ─────────────────────────────────────────────────────────────────────────────

/// 주체의 안정 식별자. **git 에서는 저자 이메일이다.**
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ActorId(String);

impl ActorId {
    #[must_use]
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// 누가 만들었나 — **최소 형태다.**
///
/// **[graph-node] `Actor`** — `schema/graph.toml`
///
/// # `종류(human|agent|tool)` 가 여기 없다 — F22-3 이 내린 판단
///
/// [옛 DESIGN §1.1](../../../docs/plan/disposal-map.md)은 `Actor` 를 `{종류, 안정 식별자, 표시 이름}`
/// 으로 적고 출처를 `asserted`(등록)라고 했다. **그 셋은 한 노드에 설 수 없다.**
///
/// git 커밋의 저자 이름·이메일은 `(재현 입력, 추출기 버전)` 만으로 재현되므로 배정
/// 규칙 1 에 걸려 `extracted` 다. 반면 *"이 식별자는 사람이 아니라 에이전트다"* 는
/// **코드에서 나오지 않는다** — 누군가 선언해야 하고 그것은 `asserted` 다.
/// 둘을 한 노드에 넣으면 [옛 DESIGN §3.4](../../../docs/plan/disposal-map.md)의 속성 출처 동질성이 깨지고,
/// 그 규칙의 처방은 *"섞으려면 노드를 쪼개고 엣지로 잇는다"* 이다.
///
/// 그래서 이 노드는 **git 정체성**만 진다. 종류를 싣는 등록 노드는 그것을 저작할
/// 기능이 없어 **만들지 않았다** — 없는 것을 미리 선언하면 스키마가 만들 수 없는 것을
/// 실은 채 자란다. 근거와 귀속은 `docs/gates/F22-3-chain-nodes.md` 에 있다.
///
/// 조직도·권한·평가의 어휘를 들이지 않는 것은 그대로다 — 들이면 P7 이 인사 도메인
/// 쪽으로 깨진다. 필요한 것은 **동일성 비교와 계보**뿐이다(§9.2 의 생산자 분리 검사).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Actor {
    pub id: ActorId,
    /// 사람이 보는 이름. 동일성 비교에 쓰지 않는다 — 같은 사람이 여러 이름을 쓴다.
    pub display: String,
}

// ─────────────────────────────────────────────────────────────────────────────
// Change
// ─────────────────────────────────────────────────────────────────────────────

/// 변경의 종류. **셋이고, 지금 서는 것은 하나다.**
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChangeKind {
    Commit,
    /// 트래커에서 온다 — 만들 기능이 없다(F23).
    PullRequest,
    /// git 이 아닌 VCS. 자리만 있다.
    Changeset,
}

impl ChangeKind {
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Commit => "commit",
            Self::PullRequest => "pull-request",
            Self::Changeset => "changeset",
        }
    }
}

/// 변경 하나의 이름. **vcs 식별자 그대로다** — 우리가 새 이름을 만들지 않는다.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ChangeId(String);

impl ChangeId {
    #[must_use]
    pub fn new(raw: impl Into<String>) -> Self {
        Self(raw.into())
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// 사람이 보는 짧은 형태. **비교에 쓰지 않는다.**
    #[must_use]
    pub fn short(&self) -> &str {
        &self.0[..self.0.len().min(8)]
    }
}

/// 변경 하나 — **[옛 DESIGN §12.6](../../../docs/plan/disposal-map.md)이 "변경의 1급화"를 수입한다고
/// 적고 타입을 만들지 않았던 자리다.**
///
/// **[graph-node] `Change`** — `schema/graph.toml`
///
/// [옛 F23](../../../docs/plan/disposal-map.md)의 git 결합과
/// [옛 F20](../../../docs/plan/disposal-map.md)의 델타가 전부 이
/// 마디 위에 선다. 없으면 델타에 기준선이 없다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Change {
    pub id: ChangeId,
    pub kind: ChangeKind,
    /// 첫 줄. 서술이지 판정이 아니다.
    pub summary: String,
    /// 누가.
    ///
    /// **[graph-edge] `AUTHORED_BY`** — `schema/graph.toml`
    pub author: ActorId,
    /// 무엇을 건드렸나 — **`body_digest` 가 변한 심볼들.**
    ///
    /// **[graph-edge] `TOUCHES`** — `schema/graph.toml`
    ///
    /// 파일이 변했다가 아니라 **의미가 변했다**이다. 포매팅만 바뀐 파일의 심볼은 여기
    /// 없다 — 정규화가 이미 그것을 지웠다(F03).
    pub touches: Vec<SymbolId>,
    /// 부모.
    ///
    /// **[graph-edge] `FOLLOWS`** — `schema/graph.toml`
    pub parents: Vec<ChangeId>,
    /// 발생 `Snapshot` — 엣지 공통 넷의 넷째를 이 노드가 싣는다.
    pub at: Snapshot,
}

// ─────────────────────────────────────────────────────────────────────────────
// Defect
// ─────────────────────────────────────────────────────────────────────────────

/// 도입 커밋을 지목했는가 — **못 지목한 것도 답이다.**
///
/// `Option` 이 아니다. *"못 찾았다"* 는 정상적인 결과이고 **왜 못 찾았는지가 함께 실려야**
/// 한다. `None` 으로 두면 *"안 찾아봤다"* 와 구별되지 않는다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "outcome")]
pub enum Introduction {
    /// 지목했다. **신뢰도와 나머지 후보가 함께 실린다.**
    ///
    /// `change` 는 *"하나를 고른 것"* 이 아니라 **순위의 머리**다. 최빈이 아닌 후보를
    /// 버리면 그것이 곧 조용한 절단이고, 이 제품이 금지한 것이다(stack §5.4).
    Found { change: ChangeId, confidence: Confidence, others: Vec<ChangeId> },
    /// 지목하지 못했다.
    NotFound { reason: NotFoundReason },
}

/// 도입 커밋 지목의 신뢰도 — **과반 비율을 숨기지 않는다.**
///
/// # 이 값이 실리지 않으면 50% 와 100% 가 같은 화면이 된다
///
/// [T10](../../../docs/gates/preflight.md#t10--여정결함의-올라탈-곳)이 정확히 그것을
/// 경고했다: 도입 커밋 성공 4 건 중 **2 건이 정확히 50%** 였고, 임계를 `>50%` 로 바꾸면
/// 4/5 가 **2/5 로 떨어진다.** *"80% 는 상한이고 40% 가 하한이다. 대표값 하나로 적지
/// 않는다."*
///
/// **감시 대상이 적을수록 최빈값은 신호가 아니다.** T10 의 `7f6b0a58` 은 삭제 줄이 둘이고
/// 그 둘이 서로 다른 커밋에서 왔다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Confidence {
    /// 최빈 커밋에 동의한 좌표 수.
    pub agreeing: usize,
    /// 지목에 쓰인 좌표 전부.
    pub total: usize,
}

impl Confidence {
    #[must_use]
    pub const fn new(agreeing: usize, total: usize) -> Self {
        Self { agreeing, total }
    }

    /// 과반인가 — **`>= 50%`.** T10 이 등록한 임계다.
    #[must_use]
    pub const fn is_majority(&self) -> bool {
        self.total > 0 && self.agreeing * 2 >= self.total
    }

    /// 엄격 임계 — **`> 50%`.** T10 의 민감도 분석이 쓴 쪽이다.
    ///
    /// **둘 다 있는 것이 요점이다.** 하나만 두면 그 순간 T10 이 금지한
    /// *"대표값 하나로 적기"* 가 된다.
    #[must_use]
    pub const fn is_strict_majority(&self) -> bool {
        self.total > 0 && self.agreeing * 2 > self.total
    }

    /// 백분율(내림). 산출에 그대로 실린다.
    #[must_use]
    pub const fn percent(&self) -> usize {
        if self.total == 0 { 0 } else { self.agreeing * 100 / self.total }
    }
}

/// 도입 커밋을 못 찾은 이유.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotFoundReason {
    /// 발현 좌표가 없어서 거슬러 올라갈 것이 없다.
    NoManifestation,
    /// 이력을 예산까지 거슬러 올라갔는데 그 심볼이 변한 조상이 없다.
    ///
    /// **조용히 멈추지 않는다** — 예산에 걸린 것과 정말 없는 것은 다른 사건이고,
    /// 이 변형이 그 사실을 산출에 남긴다.
    HistoryBudget { walked: usize },
    /// 심볼이 그 조상에 아예 없었다 — 도입이 이 파일 밖이다.
    OutsideFile,
}

/// 결함 하나 — **소급 결박된다.**
///
/// **[graph-node] `Defect`** — `schema/graph.toml`
///
/// # `원인 좌표[]` 가 여기 없다 — F22-3 이 내린 판단
///
/// [옛 DESIGN §1.1](../../../docs/plan/disposal-map.md)은 `{서술, 발현 좌표[], 원인 좌표[], 도입
/// Change, 해소 Change, 출처}` 를 적었다. **소급 결박에서 원인은 발현과 구별되지 않는다** —
/// 우리가 아는 것은 *"이 심볼이 수정 커밋에서 변했다"* 와 *"그 전에 마지막으로 변한
/// 커밋이 이것이다"* 뿐이고, 그 둘 다 같은 좌표를 가리킨다.
///
/// 구별하려면 **인과 판정**이 필요하고 그것은 F15 다. 지금 넣으면 발현의 복사본이 되고,
/// [옛 DESIGN §3.1](../../../docs/plan/disposal-map.md)은 그런 필드를 두지 말라고 적었다 —
/// *"빈 필드는 이 시스템은 이것을 다룬다는 거짓 신호"*.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Defect {
    /// **파생 노드의 id 규칙을 따른다**([`crate::DerivedId`]) — 출처·생산자·재현 입력이
    /// 성분이다. 같은 수정 커밋을 정적 도구가 다시 읽어도 **다른 노드로 선다.**
    pub id: crate::DerivedId,
    /// 수정 커밋의 첫 줄.
    pub description: String,
    /// 어디서 드러났나 — 수정 커밋에서 `body_digest` 가 변한 심볼들.
    ///
    /// **[graph-edge] `MANIFESTS_AT`** — `schema/graph.toml`
    pub manifests_at: Vec<SymbolId>,
    /// 언제 들어왔나.
    ///
    /// **[graph-edge] `INTRODUCED_BY`** — `schema/graph.toml`
    pub introduced_by: Introduction,
    /// 무엇이 고쳤나 — 이 결함을 만들어 낸 그 커밋.
    ///
    /// **[graph-edge] `RESOLVED_BY`** — `schema/graph.toml`
    pub resolved_by: ChangeId,
    /// 발생 `Snapshot`.
    pub at: Snapshot,
}

/// 결함이 담기지 않은 이유 — **조용히 빠지는 것만 막는다.**
///
/// # 왜 이 변형이 존재하는가 ([T10](../../../docs/gates/preflight.md))
///
/// 표본 5 건 중 **1 건**은 결함이 코드가 아니라 **에이전트의 지시 문서**(`agents/*.md`)에
/// 있었고 `src/**` 변경이 0 줄이었다. `발현 좌표[]` 를 코드 심볼로만 좁히면 그 종류는
/// **통째로 담기지 않는다.**
///
/// [옛 F22 §4](../../../docs/plan/disposal-map.md)는 그 자리를
/// *"아직 정하지 않는다 — 잔여로 기록만 한다"* 로 두었다. 관측 1 건으로 스키마를 넓힐
/// 근거가 얇기 때문이다. **그러면 남는 요구는 하나다: 세어서 표시한다.**
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case", tag = "why")]
pub enum Uncapturable {
    /// 변경된 파일에 좌표를 세울 수 있는 것이 없다 — 지시 문서 · 설정 · 문서.
    OutsideCode { change: ChangeId, files: Vec<RepoPath> },
    /// **이 빌드에 그 언어의 추출기가 없다.**
    ///
    /// # 이 변형이 없으면 이 도구가 자기가 고발한 문제를 저지른다
    ///
    /// 없으면 추출기가 없는 파일이 *"의미가 변한 심볼 0 개"* 로 세어지고,
    /// **"안 만들었음"과 "없음"이 같은 출력이 된다**(목표 §3.1 · S2 합격선 ②).
    /// 실제로 F22-3 의 첫 실행이 그 상태였고, T10 코퍼스가 TypeScript 라서
    /// **다섯 건이 전부 조용히 "변한 것 없음"으로 나왔다.**
    NoExtractor { change: ChangeId, capability: crate::CapabilityId, files: Vec<RepoPath> },
    /// 코드는 변했는데 **의미가 변한 심볼이 없다** — 포매팅·주석만 바뀌었거나
    /// 파일이 통째로 새로 생겼다.
    NoSemanticChange { change: ChangeId },
}

impl Uncapturable {
    #[must_use]
    pub const fn change(&self) -> &ChangeId {
        match self {
            Self::OutsideCode { change, .. }
            | Self::NoExtractor { change, .. }
            | Self::NoSemanticChange { change } => change,
        }
    }
}

/// 소급 결박 한 건의 결과 — **담긴 것과 담기지 않은 것.**
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case", tag = "outcome")]
pub enum Retrobinding {
    Bound(Box<Defect>),
    /// **건수가 아니라 이유와 함께 남는다.**
    Missed(Uncapturable),
}

/// 여럿을 소급 결박한 결과의 요약 — **분모가 함께 실린다.**
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetrobindingSummary {
    /// 시도한 수정 변경 수.
    pub attempted: usize,
    /// 발현 좌표를 지목한 수.
    pub manifestation_bound: usize,
    /// 도입 변경을 **과반(`>=50%`)** 으로 지목한 수.
    pub introduction_at_majority: usize,
    /// 도입 변경을 **엄격 과반(`>50%`)** 으로 지목한 수. **상한과 하한을 둘 다 적는다.**
    pub introduction_at_strict: usize,
    /// **담기지 않은 수.** 0 이면 계수기를 의심해야 한다 — T10 표본에서는 5 중 1 이었다.
    pub uncapturable: usize,
}

impl RetrobindingSummary {
    /// 결과 목록에서 센다. **따로 들고 있지 않는 것이 합의 보증이다** —
    /// [`crate::Ledger::counts`] 와 같은 규율이다.
    #[must_use]
    pub fn of(results: &[Retrobinding]) -> Self {
        let mut s = Self {
            attempted: results.len(),
            manifestation_bound: 0,
            introduction_at_majority: 0,
            introduction_at_strict: 0,
            uncapturable: 0,
        };
        for r in results {
            match r {
                Retrobinding::Bound(d) => {
                    if !d.manifests_at.is_empty() {
                        s.manifestation_bound += 1;
                    }
                    if let Introduction::Found { confidence, .. } = &d.introduced_by {
                        if confidence.is_majority() {
                            s.introduction_at_majority += 1;
                        }
                        if confidence.is_strict_majority() {
                            s.introduction_at_strict += 1;
                        }
                    }
                }
                Retrobinding::Missed(_) => s.uncapturable += 1,
            }
        }
        s
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Journey
// ─────────────────────────────────────────────────────────────────────────────

/// 사용자 여정 — **자리만 있다. 값을 만들 수 없다.**
///
/// **[graph-node] `Journey`** — `schema/graph.toml` (`status = "not_built"`)
///
/// # 변형이 없는 것이 [T10ⓑ](../../../docs/gates/preflight.md)의 판정이다
///
/// 옛 백서 §7 사슬의 `여정` 이고 U11-c 가 *"사용자 여정 추적"* 을 명시 요구했다. 그런데
/// **여정은 코드에서 도출되지 않는다** — 진입점은 추출되지만 *"이것들이 한 여정이다"* 는
/// 사람의 선언이다.
///
/// T10ⓑ 가 그것을 실측했고 **진입점 좌표 0/3 으로 반증됐다.** goals §0.1 의 (c)가
/// *"진입점에서 시작하는 도달 하한"* 으로 줄었고, 그 하한은 이 타입 없이 선다.
///
/// **그래서 여기 값이 없다.** 만들 수 없는 변형을 미리 두면 그것이 곧 *"있는데 안 나오는"*
/// 상태가 되고, 이 제품이 고발한 문제를 스스로 저지르는 것이다. 저작 경로는 F19 다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum Journey {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 신뢰도는_상한과_하한을_함께_말한다() {
        // T10 의 `7f6b0a58` — 2 중 1. 과반이지만 엄격 과반은 아니다.
        let 반반 = Confidence::new(1, 2);
        assert!(반반.is_majority());
        assert!(!반반.is_strict_majority());
        assert_eq!(반반.percent(), 50);

        // `2550118a` — 가장 깨끗한 건.
        let 전부 = Confidence::new(6, 6);
        assert!(전부.is_majority() && 전부.is_strict_majority());
        assert_eq!(전부.percent(), 100);

        // 소수는 어느 쪽도 아니다.
        let 소수 = Confidence::new(1, 3);
        assert!(!소수.is_majority());
        assert_eq!(소수.percent(), 33);
    }

    #[test]
    fn 빈_분모는_과반이_아니다() {
        // 0/0 을 참으로 두면 아무것도 못 지목한 것이 만점이 된다.
        let 없음 = Confidence::new(0, 0);
        assert!(!없음.is_majority());
        assert!(!없음.is_strict_majority());
        assert_eq!(없음.percent(), 0);
    }

    #[test]
    fn 요약은_담기지_않은_것을_센다() {
        // **T10 의 표본이 이 모양이다** — 5 중 1 이 코드 밖이었다.
        let 놓침 = Retrobinding::Missed(Uncapturable::OutsideCode {
            change: ChangeId::new("afcfefab"),
            files: vec![RepoPath::new("agents/reviewer.md")],
        });
        let s = RetrobindingSummary::of(&[놓침]);
        assert_eq!(s.attempted, 1);
        assert_eq!(s.uncapturable, 1);
        assert_eq!(s.manifestation_bound, 0);
    }

    #[test]
    fn 여정은_값을_만들_수_없다() {
        // 타입 수준의 사실이다 — 이 시험은 그것이 컴파일된다는 것으로 성립한다.
        fn 받는다(_: &Journey) {}
        let _ = 받는다;
        assert_eq!(std::mem::size_of::<Journey>(), 0);
    }
}
