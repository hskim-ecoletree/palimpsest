//! `pal bind <이름> --note <조각>` — **사람이 손으로 넣는 자리.**
//!
//! 이것이 의도층의 유일한 입구다(S3 기준). 문서에서 조각을 잘라 신호로 좌표를 찾는
//! 일은 F10 이고, 여기서는 **사람이 이름으로 직접 지정한다.**
//!
//! # 결박은 파생층에 쓰이지 않는다 ([R-21])
//!
//! 쓰는 곳은 `pal-intent` 의 `intent.redb` 하나다. 2층(`index.redb`)에는 아무것도
//! 남기지 않는다 — 남기면 *"2층을 지우고 재구축"* 이 그것을 지운다.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use pal_core::{Binding, SymbolIdentity, WatchEntry};
use pal_intent::IntentStore;

use crate::{attach, ledger, touch};

/// 좌표 하나에 조각 하나를 건다.
///
/// # Errors
/// 저장소·2층·의도 저장소 중 하나에 닿지 못하거나, 이름이 유일하지 않으면.
pub fn run(
    repo_path: &Path,
    rev: Option<&str>,
    cache_dir: Option<PathBuf>,
    index_path: Option<PathBuf>,
    intent_path: Option<PathBuf>,
    name: &str,
    note: &str,
) -> Result<()> {
    let report = ledger::compute(repo_path, rev, cache_dir)?;

    let index = index_path.unwrap_or_else(|| repo_path.join(".palimpsest/index.redb"));
    let attached = attach::attach(&index, &report, attach::How::SymbolsOnly)?;
    let projection = &attached.projection;

    let found = projection.resolve_name(name).context("2층을 읽지 못했다")?;
    let symbol = match found.len() {
        0 => bail!("`{name}` 을 이 스냅샷에서 찾지 못했다 — `pal touch {name}` 이 근거를 낸다"),
        1 => found.into_iter().next().expect("길이가 1 이다"),
        n => {
            // **하나를 골라주지 않는다.** 고르는 것은 사람의 일이고, 잘못 고른 결박은
            // 조용히 틀린 곳을 가리킨다.
            let where_ = found
                .iter()
                .map(|c| format!("{}:{}", c.path, c.span.line_start))
                .collect::<Vec<_>>()
                .join(" · ");
            bail!("`{name}` 의 후보가 {n} 건이다 — 하나로 좁혀야 한다: {where_}");
        }
    };

    // **좌표를 꺼내는 길이 둘뿐이다** (F03 §3.3). `Unavailable` 에는 실린 좌표가
    // 없으므로 여기서 결박이 끝난다 — 그 판정이 `if` 가 아니라 **타입**이다.
    let target = match SymbolIdentity::new(symbol.identity, symbol.id) {
        SymbolIdentity::Exact(id) | SymbolIdentity::Ordinal(id) => id,
        SymbolIdentity::Unavailable => bail!(
            "`{name}` 은 좌표가 없다 — 이 언어의 추출 등급이 L0 이라 결박이 성립하지 않는다"
        ),
    };

    // **감시 집합은 대상 심볼 하나다.** 반경(무엇까지 지켜볼 것인가)은 F09 다.
    let binding = Binding::new(
        target,
        note,
        report.ledger.snapshot.clone(),
        vec![WatchEntry { symbol: target, digest: symbol.body }],
    );

    let intent = IntentStore::open(&touch::intent_file(repo_path, intent_path))
        .context("의도 저장소를 열지 못했다")?;
    intent.record(&binding).context("결박을 남기지 못했다")?;

    println!();
    println!("결박했습니다.");
    println!("  대상    {} · {} · {}:{}", symbol.name, symbol.kind.name(), symbol.path, symbol.span.line_start);
    println!("  좌표    {}#{}", report.ledger.snapshot, symbol.id.short());
    println!("  본문    {}  ← 이 값이 바뀌면 낡음이 표시됩니다", symbol.body.short());
    println!("  결박    [{}]", binding.id.as_str());
    println!();
    println!("  의도 저장소에 {} 건 있습니다. **파생층을 지워도 남습니다.**",
             intent.count().unwrap_or(0));
    println!();
    Ok(())
}
