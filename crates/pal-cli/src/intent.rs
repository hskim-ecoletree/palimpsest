//! `pal intent export|import` — **재구축 불가한 것을 밖으로 내고 되읽는다.**
//!
//! F05 §6 의 표: *"의도 저장소 손상 → **재구축 불가.** JSONL 내보내기에서 복구.
//! 그래서 내보내기가 **상시 유지**된다."*
//!
//! # 이 명령이 지우는 명령이 아니다
//!
//! `import` 는 **더한다.** 파일에 없는 결박은 그대로 남는다 — 저장소를 파일의 모습으로
//! 만들면 그것이 곧 지우는 경로이고, 이 크레이트 계열이 막는 것이 정확히 그것이다
//! ([R-21](../../../docs/plan/00-risks.md#r-21)).

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use pal_intent::IntentStore;

use crate::touch::intent_file;

/// 전부를 JSONL 로 낸다.
///
/// # Errors
/// 의도 저장소를 읽지 못하거나 파일을 쓰지 못하면.
pub fn export(repo: &Path, intent: Option<PathBuf>, out: Option<PathBuf>) -> Result<()> {
    let path = intent_file(repo, intent);
    // **읽기만 한다** — 내보내기가 파일을 키우면 그것 자체가 사건이다.
    let store = IntentStore::open_read_only(&path).context("의도 저장소를 열지 못했다")?;
    let text = store.export_jsonl().context("내보내지 못했다")?;
    match out {
        Some(file) => {
            std::fs::write(&file, &text).with_context(|| format!("{}", file.display()))?;
            // **줄 수를 적는다.** 0 줄짜리 내보내기가 조용히 성공하면 그것이 유실의 형태다.
            println!("{} — {} 줄", file.display(), text.lines().count());
        }
        None => print!("{text}"),
    }
    Ok(())
}

/// JSONL 을 읽어 **더한다.**
///
/// # Errors
/// 파일을 읽지 못하거나, 판이 다르거나, 쓰기가 실패하면.
pub fn import(repo: &Path, intent: Option<PathBuf>, file: &Path, json: bool) -> Result<()> {
    let text = std::fs::read_to_string(file).with_context(|| format!("{}", file.display()))?;
    let store = IntentStore::open(&intent_file(repo, intent)).context("의도 저장소를 열지 못했다")?;
    let report = store.import_jsonl(&text).context("읽지 못했다")?;
    if json {
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else {
        println!(
            "결박 {} · 별칭 {} · 이미 있던 것 {}",
            report.bindings, report.aliases, report.already_present
        );
        println!();
        println!("  **바꿔치기가 아니라 더하기다** — 파일에 없던 결박은 그대로 남아 있습니다.");
    }
    Ok(())
}
