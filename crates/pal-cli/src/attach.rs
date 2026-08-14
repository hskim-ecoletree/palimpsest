//! 2층에 붙는 자리 — **표면 다섯이 각자 세우던 것을 한 함수로 모은다.**
//!
//! # 왜 모으는가 ([F06 게이트](../../../docs/gates/F06.md) §6-가-2)
//!
//! 흩어져 있으니 **갈렸다.** F06 이 실측했다:
//!
//! ```text
//! pal query  → stitch   (파일 노드 · 엣지 · EXPORTS 전부)
//! pal touch  → stitch
//! pal bind   → rebuild  ← 엣지를 지운다
//! pal doctor → rebuild  ← 엣지를 지운다
//! pal export → 읽기만
//! ```
//!
//! **다섯 자리에 같은 결정이 다섯 번 적혀 있으면 그 다섯이 갈리는 것은 시간 문제이고,
//! 실제로 갈렸다.** 여기로 모으면 *"이 표면은 어떻게 붙는가"* 가 **한 낱말**이 되고,
//! 그 낱말이 [`How`] 다.
//!
//! # ⚠ [`How::SymbolsOnly`] 가 변형으로 남아 있는 것이 기록이다
//!
//! 이 변형은 **엣지를 지운다.** 없애는 것이 옳지만 남은 소비자([`crate::doctor`])의
//! 소유자가 다르다(F22 후속) — 그 명령의 불변식 **모집단이 바뀌면 `[f22.4]` 의 판정이
//! 움직인다.** 한 줄이 싸다는 이유로 남의 게이트를 건드리지 않는다.
//!
//! 그래서 이름이 결함을 **말한다.** 소비자가 하나로 줄면 그 사실 자체가 다음 세션의
//! 근거다.

use std::path::Path;

use anyhow::{Context, Result};
use pal_core::PROVISIONAL_STITCH_BATCH;
use pal_store::Projection;

use crate::ledger::LedgerReport;

/// 이 표면이 2층에 **어떻게** 붙는가.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum How {
    /// **스티칭한다** — 파일 노드 · 엣지 · `EXPORTS` 전부(F05 §4).
    Stitching,
    /// ⚠ **심볼만 다시 만든다 — 엣지를 지운다.**
    ///
    /// `Projection::rebuild` 는 심볼만 쓰고 `built_for` 를 **빈 문자열로** 갈아 끼운다.
    /// S3 가 이렇게 썼고 F05 가 `stitch` 를 세우면서 안 옮겼다. **남은 소비자는
    /// `pal doctor` 하나이고 그 자리는 F22 후속이다.**
    SymbolsOnly,
    /// **읽기만 한다** — 스티칭을 못 한다.
    ///
    /// 그러므로 2층이 이 스냅샷에 대해 이미 서 있지 않으면 답이 낡고, **그 사실이
    /// [`Attached::built_for_this_snapshot`] 에 실린다.** 조용히 쓰기로 되돌아가지 않는다.
    ReadOnly,
}

/// 붙은 결과. **세 값이 함께 나온다** — 따로 꺼내면 또 갈린다.
pub struct Attached {
    pub projection: Projection,
    /// 2층에 실제로 실린 심볼 수.
    pub indexed: usize,
    /// 이 답이 선 스냅샷의 이름. 2층의 `built_for` 와 **대조하는 값**이다.
    pub built_for: String,
}

impl Attached {
    /// 2층이 **이 스냅샷에 대해** 세워졌는가.
    ///
    /// **관측이지 기본값이 아니다.** 옛 판 하나가 `true` 로 박혀 있었고 그것은
    /// *"이 스냅샷에서 만들어졌다"* 를 **확인하지 않고 적은 것**이었다.
    #[must_use]
    pub fn built_for_this_snapshot(&self) -> bool {
        self.projection
            .built_for()
            .unwrap_or_default()
            .is_some_and(|s| s == self.built_for)
    }
}

/// 2층에 붙는다. **이 함수가 유일한 자리다.**
///
/// # Errors
/// 2층을 열지 못하거나 세우지 못하면.
pub fn attach(index: &Path, report: &LedgerReport, how: How) -> Result<Attached> {
    let built_for = report.ledger.snapshot.to_string();
    let (projection, indexed) = match how {
        How::Stitching => {
            let p = Projection::open(index).context("2층을 열지 못했다")?;
            let n = p
                .stitch(&built_for, &report.stitches, PROVISIONAL_STITCH_BATCH)
                .context("2층을 세우지 못했다")?
                .symbols;
            (p, n)
        }
        How::SymbolsOnly => {
            let p = Projection::open(index).context("2층을 열지 못했다")?;
            let n = p.rebuild(&report.symbols).context("2층을 세우지 못했다")?;
            (p, n)
        }
        How::ReadOnly => {
            let p = Projection::open_read_only(index)
                .context("2층에 읽기 전용으로 붙지 못했다 — 먼저 한 번 쓰기로 세워야 한다")?;
            let n = p.count().context("2층을 읽지 못했다")?;
            (p, n)
        }
    };
    Ok(Attached { projection, indexed, built_for })
}
