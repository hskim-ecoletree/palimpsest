//! 아직 만들지 않은 능력을 값으로 표현한다 — stack §5.3.

use serde::Serialize;

/// 미구축 산출의 자리.
///
/// **`Option<T>` 가 아니다.** `None` 은 *"값이 없다"* 이고 우리에게 필요한 것은
/// *"이 빌드가 답하지 않는다"* 이다. 빈 컬렉션으로 대신하는 것은 **거짓 안전**이다 —
/// `Finding 0` 과 "감사를 안 만들었음"이 같은 출력이 되는 것이 목표 §3.1 의 정면 위반이다.
///
/// S0 에서 이 타입은 장식이 아니라 하중을 진다. 지원 언어가 넷인데
/// (Kotlin·Java·JavaScript·TypeScript — 지시 2026-08-12 §1) 추출기는 하나뿐이라,
/// 나머지 셋에서 빈 목록을 내면 *"선언이 없는 파일"* 과 *"이 빌드가 그 언어를 모른다"* 가
/// 구별되지 않는다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Capable<T> {
    Present(T),
    NotBuilt { capability: CapabilityId },
}

impl<T> Capable<T> {
    /// 이 빌드가 답하지 않는다고 선언한다.
    #[must_use]
    pub const fn not_built(capability: CapabilityId) -> Self {
        Self::NotBuilt { capability }
    }

    #[must_use]
    pub const fn is_present(&self) -> bool {
        matches!(self, Self::Present(_))
    }

    /// 값이 있으면 그것을, 없으면 능력의 정체를 돌려준다.
    ///
    /// # Errors
    /// 이 빌드에 그 능력이 없으면 `Err(CapabilityId)`.
    pub fn into_present(self) -> Result<T, CapabilityId> {
        match self {
            Self::Present(v) => Ok(v),
            Self::NotBuilt { capability } => Err(capability),
        }
    }
}

/// 능력의 정체. **어느 기능이 그것을 만드는지가 실린다** — stack §5.3.
///
/// 능력은 빌드 시점에 정해지므로 `&'static str` 이다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct CapabilityId {
    /// 이 능력을 만드는 기능 번호. 예: `"F02"`
    pub feature: &'static str,
    /// 무슨 능력인가. 예: `"java-extraction"`
    pub what: &'static str,
}

impl CapabilityId {
    #[must_use]
    pub const fn new(feature: &'static str, what: &'static str) -> Self {
        Self { feature, what }
    }
}
