//! `pal cache stats|prune` — **이 저장소에서 유일하게 되돌릴 수 없는 명령이다.**
//!
//! # 왜 이 파일이 조심스러운가 ([R-21])
//!
//! 캐시를 **만드는** 것은 틀려도 재계산된다. **지우는** 것은 틀리면 사람의 노동이
//! 사라진다. 그래서 게이트 셋이 이 명령을 기다리며 같은 문장을 적고 넘겼다:
//!
//!   · [S1](../../../docs/gates/S1-ledger.md) — *"`pal cache prune`(지우는 API)은
//!     없는 것이 지금의 정상 상태다. 생길 때 R-21 의 검사가 실제 하중을 진다"*
//!   · [S3](../../../docs/gates/S3-intent.md) §140 — 같은 문장
//!   · [F22-4](../../../docs/gates/F22-4-doctor.md) — *"불변식 7 이 하중을 지는 것은
//!     `pal cache prune` 이 생길 때다"*
//!
//! **셋이 여기서 동시에 만기가 된다.**
//!
//! # 경계를 코드의 형태로 세운다
//!
//! `cargo xtask check` 의 「의도 저장소 폐기 경로 부재」는 **문자열 스캔**이라
//! *"소스에 낱말이 없다"* 만 말한다. 여기서 지는 것은 그것보다 앞의 것이다 —
//! **이 파일은 지울 경로를 스스로 만들지 않는다.** 지우는 일은 전부
//! [`pal_store::BlobCache::evict_to`] 안에서 일어나고 그것은 **캐시의 뿌리 아래만**
//! 훑는다. 여기서 정하는 것은 뿌리 하나이고, 그 뿌리는 언제나 `…/cache` 다.
//!
//! 실물 검사는 `crates/pal-cli/tests/prune_boundary.rs` 다 — *"`prune` 뒤에
//! `cache/` 밖의 모든 파일이 바이트로 같은가"*.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use pal_store::{BlobCache, EvictReport, ExtractCache as _, SweepReport};

// **기본 예산은 여기 없다.** `pal-core::budget::DEFAULT_CACHE_BUDGET_BYTES` 한 곳이다
// (stack §5.5 · `[f05.1.pass]` ①). 넘겨서 줄이는 것은 `--budget` 이다.

/// `pal cache prune` 의 인자. **손잡이가 다섯이라 이름으로 받는다** —
/// 위치 인자 다섯은 뒤바뀌어도 타입이 안 잡는다.
pub struct PruneArgs {
    pub repo: PathBuf,
    pub cache_dir: Option<PathBuf>,
    pub budget: u64,
    /// 격리 방을 이 예산까지 줄인다. **`None` 이면 한 바이트도 안 지운다.**
    pub sweep_quarantine: Option<u64>,
    /// 죽은 `.tmp` 를 지운다. **거짓이면 한 개도 안 지운다.**
    pub sweep_stray: bool,
    /// `.tmp` 를 죽은 것으로 보기까지의 나이(초).
    pub stray_age: u64,
    pub json: bool,
}

/// 캐시 뿌리. **`<저장소>/.palimpsest/cache` 하나뿐이다.**
fn root_of(repo: &Path, cache_dir: Option<PathBuf>) -> PathBuf {
    cache_dir.unwrap_or_else(|| repo.join(".palimpsest/cache"))
}

/// 얼마나 차 있는가.
///
/// # Errors
/// 캐시를 열거나 훑지 못하면.
pub fn stats(repo: &Path, cache_dir: Option<PathBuf>, json: bool) -> Result<()> {
    let root = root_of(repo, cache_dir);
    let cache = BlobCache::open(&root).context("캐시를 열지 못했다")?;
    let usage = cache.usage().context("캐시를 훑지 못했다")?;

    if json {
        println!("{}", serde_json::to_string_pretty(&usage)?);
        return Ok(());
    }

    println!();
    println!("캐시      {}", root.display());
    println!("엔트리    {} · {}", usage.entries, 사람이_읽는(usage.bytes));
    if usage.entries > 0 {
        // **파일당 평균이 합격선이다** — F04 §3.2 의 「2KB 이하」.
        println!(
            "파일당    평균 {}",
            사람이_읽는(usage.bytes / usage.entries as u64)
        );
    }
    // **0 이면 적지 않는다.** 사건이 없는 것이 정상 상태이고, 늘 적으면 `Finding 0` 이다.
    if usage.quarantined_entries > 0 {
        println!(
            "⚠ 격리    {} · {}  ← 깨진 바이트다. 축출이 안 건드린다",
            usage.quarantined_entries,
            사람이_읽는(usage.quarantined_bytes)
        );
    }
    if usage.stray_bytes > 0 {
        println!(
            "  임시    {}  ← 도는 쓰기이거나 죽은 쓰기의 흔적. 지우지 않는다",
            사람이_읽는(usage.stray_bytes)
        );
    }
    println!();
    println!("능력 축   {}", pal_extract::capability_axis());
    println!();
    Ok(())
}

/// 예산까지 줄인다. **`cache/` 밖은 건드리지 않는다.**
///
/// # Errors
/// 캐시를 열거나 지우지 못하면.
pub fn prune(a: PruneArgs) -> Result<()> {
    let PruneArgs { repo, cache_dir, budget, sweep_quarantine, sweep_stray, stray_age, json } = a;
    let root = root_of(&repo, cache_dir);
    let cache = BlobCache::open(&root).context("캐시를 열지 못했다")?;
    let before = cache.usage().context("캐시를 훑지 못했다")?;
    let report = cache.evict_to(budget).context("축출하지 못했다")?;
    // **지운 뒤에 다시 센다.** 보고가 스스로를 확인하면 그것은 확인이 아니다 —
    // 숫자만 내고 안 지우는 구현이 통과한다(`[f04.pass]` ④).
    // ── F04 가 넘긴 둘 — **기본은 안 지운다** (`[f05.5]`) ────────────────────
    //
    // 격리된 바이트는 **결함의 증거**이고 `.tmp` 는 **도는 쓰기일 수 있다.**
    // 둘 다 기본으로 지우면 되돌릴 수 없는 쪽이라 부르는 쪽이 명시해야 한다.
    let 격리: Option<EvictReport> = match sweep_quarantine {
        Some(b) => Some(cache.sweep_quarantine(b).context("격리 방을 줄이지 못했다")?),
        None => None,
    };
    let 임시: Option<SweepReport> = if sweep_stray {
        Some(cache.sweep_stray(std::time::Duration::from_secs(stray_age)).context("`.tmp` 를 지우지 못했다")?)
    } else {
        None
    };

    let after = cache.usage().context("캐시를 다시 훑지 못했다")?;

    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "before": before,
                "report": report,
                "quarantine": 격리,
                "stray": 임시,
                "after": after,
            }))?
        );
        return Ok(());
    }

    println!();
    println!("캐시      {}", root.display());
    println!("예산      {}", 사람이_읽는(budget));
    println!(
        "훑음      {} · 지움 {} · 남김 {}",
        report.scanned, report.removed, report.kept_entries
    );
    println!(
        "바이트    {} → {}  (푼 것 {})",
        사람이_읽는(before.bytes),
        사람이_읽는(after.bytes),
        사람이_읽는(report.freed_bytes)
    );
    // **안 부른 손잡이는 한 줄도 안 적는다.** 늘 적으면 `Finding 0` 이 되고,
    // 그것이 이 도구가 고발하는 형태다.
    if let Some(q) = 격리 {
        println!(
            "격리 방   훑음 {} · 지움 {} · 남김 {} · 푼 것 {}",
            q.scanned, q.removed, q.kept_entries, 사람이_읽는(q.freed_bytes)
        );
    }
    if let Some(s) = 임시 {
        println!(
            "`.tmp`    훑음 {} · 지움 {} · **어려서 남김 {}** · 푼 것 {}",
            s.scanned, s.removed, s.too_young, 사람이_읽는(s.freed_bytes)
        );
    }
    // **보고와 실물을 나란히 적는다.** 다르면 그것이 곧 꺼진 대조의 신호다.
    if after.entries != report.kept_entries {
        println!(
            "⚠ 보고    남긴다고 적은 {} 와 실제 {} 가 다르다",
            report.kept_entries, after.entries
        );
    }
    println!();
    println!("의도 저장소는 건드리지 않았습니다 — 이 명령이 닿는 곳은 위 디렉터리뿐입니다.");
    println!();
    Ok(())
}

/// 사람이 읽는 크기. **반올림하지 않고 내림한다** — 예산과 대는 값이라 부풀면 안 된다.
fn 사람이_읽는(bytes: u64) -> String {
    const K: u64 = 1024;
    match bytes {
        b if b < K => format!("{b} B"),
        b if b < K * K => format!("{} KiB", b / K),
        b if b < K * K * K => format!("{} MiB", b / (K * K)),
        b => format!("{} GiB", b / (K * K * K)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 크기는_내림이다() {
        // 부풀리면 예산과 댈 때 *"안 넘었다"* 가 넘은 것을 가린다.
        assert_eq!(사람이_읽는(1023), "1023 B");
        assert_eq!(사람이_읽는(2047), "1 KiB");
        assert_eq!(사람이_읽는(2 * 1024 * 1024 * 1024), "2 GiB");
    }

    #[test]
    fn 뿌리는_언제나_cache_다() {
        // **이 명령이 지울 수 있는 곳을 정하는 유일한 자리다.**
        let r = root_of(Path::new("/x/repo"), None);
        assert!(r.ends_with(".palimpsest/cache"), "{}", r.display());
    }
}
