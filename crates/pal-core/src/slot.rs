//! 저장되는 형태의 능력 자리 — **능력의 정체를 담지 않는다.**
//!
//! # 왜 [`Capable`] 을 그대로 저장할 수 없는가
//!
//! [`Capable::NotBuilt`] 는 [`crate::CapabilityId`] 를 담고 그 필드가 `&'static str` 이라
//! **애초에 역직렬화될 수 없다.** 그것이 우연이 아니다 — *"이 빌드가 무엇을 만드는가"* 는
//! **저장된 사실이 아니라 빌드의 사실**이다. 옛 빌드의 `CapabilityId` 를 새 빌드의 산출에
//! 실으면 *"F02 가 안 만들었다"* 가 F02 가 이미 만든 빌드에서도 그대로 보인다.
//!
//! # 그래서 정체를 저장하지 않고 **되씌운다**
//!
//! 저장되는 것은 [`Slot`] 이고 그것은 *"이 자리가 만들어졌는가"* 만 담는다.
//! 되읽을 때 이 빌드의 껍데기에서 정체를 가져와 씌운다 — 그리고 **씌우기가 어긋나면
//! 오류다.** 능력 축이 키에 있으므로 일어날 수 없고, 일어나면 키가 샌 것이다.
//!
//! # 여기 사는 이유
//!
//! 1층 캐시(`pal-extract::CachedGraph`)와 2층 투영(`pal-store`)이 **둘 다** 이 형태를
//! 쓴다. `pal-store` 는 `pal-extract` 에 의존할 수 없으므로(의존 방향 검사) 두 벌을
//! 만들거나 여기로 오거나 둘 중 하나이고, **두 벌이면 한쪽만 고치는 경로가 생긴다.**
//!
//! [`Capable`]: crate::Capable
//! [`Capable::NotBuilt`]: crate::Capable::NotBuilt

use serde::{Deserialize, Serialize};

use crate::capable::Capable;

/// 저장 안의 자리 하나. **[`Capable`] 이 아니다 — 능력의 정체를 담지 않는다.**
///
/// [`Capable`]: crate::Capable
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Slot<T> {
    /// 이 빌드가 안 만든 자리였다. **어느 능력인지는 여기 없다.**
    NotBuilt,
    Built(T),
}

/// 저장의 자리와 이 빌드의 능력이 어긋났다.
///
/// **능력 축이 키에 있으므로 일어날 수 없다.** 일어나면 키가 새는 것이고, 그 사실이
/// 조용한 오답이 되지 않게 오류로 낸다.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShellMismatch {
    pub slot: &'static str,
    pub cached_built: bool,
}

impl<T> Slot<T> {
    /// 능력에서 자리로. **정체를 버린다.**
    #[must_use]
    pub fn of(c: Capable<T>) -> Self {
        match c {
            Capable::Present(v) => Self::Built(v),
            Capable::NotBuilt { .. } => Self::NotBuilt,
        }
    }

    /// 이 빌드의 껍데기에서 정체를 가져와 씌운다.
    ///
    /// # Errors
    /// 저장의 자리와 이 빌드의 능력이 어긋나면 — **키가 샜다는 뜻이다.**
    pub fn restore(
        self,
        shell: &Capable<()>,
        slot: &'static str,
    ) -> Result<Capable<T>, ShellMismatch> {
        match (self, shell) {
            (Self::Built(v), Capable::Present(())) => Ok(Capable::Present(v)),
            (Self::NotBuilt, Capable::NotBuilt { capability }) => {
                Ok(Capable::NotBuilt { capability: *capability })
            }
            (Self::Built(_), Capable::NotBuilt { .. }) => {
                Err(ShellMismatch { slot, cached_built: true })
            }
            (Self::NotBuilt, Capable::Present(())) => {
                Err(ShellMismatch { slot, cached_built: false })
            }
        }
    }
}
