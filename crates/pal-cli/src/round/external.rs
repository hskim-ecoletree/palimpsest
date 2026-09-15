//! **프로젝트 밖 저장소의 이 프로젝트 몫** — uninstall 이 부르는 단일 접점.
//!
//! 승인 · 종료 봉인 · Stop 활성화 · 진행 파일은 저장소 **밖**(`approval::store_location`)에 있다.
//! 그 자리를 무엇으로 가르고 무엇을 걷는지는 이 모듈 한 자리가 진다 — `install.rs` 는 이것을 부르고
//! 보고를 출력할 뿐이다(회차 `2026-09-15-clean-uninstall` 계획 1 · 4 · 5).
//!
//! # 계약
//!
//! - uninstall 은 **안의 정본을 지우기 전에** 이것을 부른다. `--purge` 가 봉인을 가르려면 회차 원장이
//!   아직 워킹트리에 있어야 한다.
//! - `worktree_거부` 가 비어 있지 않으면 **밖의 기록을 하나도 안 건드렸다**는 뜻이다.
//!
//! ⚠ **지금 본문은 착수 동작을 지키는 스텁이다** — 활성화 파일만 지운다. 걷는 규칙은 뒤 커밋이 채운다.

use std::path::{Path, PathBuf};

use anyhow::Result;

/// 어디까지 걷나.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum 걷기 {
    /// 운영 상태(Stop 활성화 · 진행 파일 · 잠금)만 걷는다. 승인 · 봉인은 정본이라 남긴다.
    기본,
    /// `--purge` — 이 프로젝트의 승인 · 봉인까지 걷는다. 추적 중인 회차 원장에 적힌 봉인은 남긴다.
    // ⚠ 만드는 자리(`uninstall --purge`)가 아직 없다 — 그 플래그를 세우는 커밋이 이 줄을 지운다.
    #[allow(dead_code)]
    전부,
}

/// 밖에서 한 일과 안 한 일. uninstall 이 이것을 그대로 출력한다.
#[derive(Debug, Default)]
pub struct 밖의_보고 {
    /// 들여다본 저장소 자리. 자리를 정하지 못했으면 `None`.
    pub 들여다본_자리: Option<PathBuf>,
    /// 지운 파일.
    pub 지운: Vec<PathBuf>,
    /// 이 프로젝트 몫인데 남긴 파일과 그 까닭.
    pub 남긴: Vec<(PathBuf, String)>,
    /// 어느 프로젝트 몫인지 가를 수 없어 안 지운 파일.
    pub 가를_수_없는: Vec<PathBuf>,
    /// 같은 저장소의 다른 git worktree. 비어 있지 않으면 밖을 하나도 안 건드렸다.
    pub worktree_거부: Vec<PathBuf>,
    /// 같은 원격을 쓰는 다른 클론의 기록도 함께 지웠을 수 있다.
    pub 클론_경고: bool,
}

/// 이 프로젝트 몫을 `방식` 만큼 걷는다.
///
/// # Errors
/// 저장소 식별자를 못 정하거나 파일을 못 지우면.
pub fn 걷는다(repo: &Path, 방식: 걷기) -> Result<밖의_보고> {
    let _ = 방식;
    crate::round::stop::disable_if_supported(repo)?;
    Ok(밖의_보고::default())
}
