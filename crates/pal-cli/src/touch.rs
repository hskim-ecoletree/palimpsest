//! `pal touch <이름>` — 적시 제시.
//!
//! # 이 명령이 S2 에서 증명하는 것은 **빈 답의 정직성**이다
//!
//! 결박도 참조 해소도 판정도 아직 없다. 화면이 거의 비는데, 그 빈 자리가 `[]` 나
//! `Finding 0` 으로 나오면 이 도구는 자기가 고발한 문제를 스스로 저지른다.
//! 그래서 자리마다 **어느 기능이 그것을 만드는지**가 적힌다.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use pal_core::{
    Capable, CapabilityId, CapabilitySet, Coord, Coverage, Elision, Envelope, ExtractGrade,
    IdentityGrade, LedgerRef, ProjectionFreshness, SymbolNode, TouchAnswer, TouchResult,
};
use pal_store::Projection;

use crate::ledger;

/// 이 빌드가 답하는 것과 아직 못 만든 것. **응답마다 실린다**(stack §5.3).
fn capabilities() -> CapabilitySet {
    CapabilitySet::new(
        vec![
            "ledger.snapshot".to_owned(),
            "symbol.resolve".to_owned(),
        ],
        vec![
            CapabilityId::new("F07", "reference-resolution"),
            CapabilityId::new("F08", "unresolved-refs"),
            CapabilityId::new("F09", "binding"),
            CapabilityId::new("F13", "effects"),
            CapabilityId::new("F15", "judgment"),
        ],
    )
}

/// 좌표 하나를 만진다.
///
/// # Errors
/// 저장소·캐시·2층 중 하나에 닿지 못하면.
pub fn run(
    repo_path: &Path,
    rev: Option<&str>,
    cache_dir: Option<PathBuf>,
    index_path: Option<PathBuf>,
    name: &str,
    json: bool,
) -> Result<()> {
    // 대장을 먼저 만든다. **답의 근거가 먼저 서야 답이 나간다.**
    let report = ledger::compute(repo_path, rev, cache_dir)?;

    let index = index_path.unwrap_or_else(|| repo_path.join(".palimpsest/index.redb"));
    let projection = Projection::open(&index).context("2층을 열지 못했다")?;
    // **통째로 다시 만든다.** 2층은 캐시이고 증분 갱신은 F05 의 것이다.
    let indexed = projection.rebuild(&report.symbols).context("2층을 세우지 못했다")?;

    let found = projection.resolve_name(name).context("2층을 읽지 못했다")?;
    let answer = match found.len() {
        0 => TouchAnswer::Unknown { name: name.to_owned() },
        1 => {
            let symbol = found.into_iter().next().expect("길이가 1 이다");
            TouchAnswer::Found(Box::new(touch_result(&report, symbol)))
        }
        _ => TouchAnswer::Ambiguous { name: name.to_owned(), candidates: found },
    };

    let envelope = Envelope::new(
        answer,
        report.ledger.snapshot.clone(),
        ProjectionFreshness {
            // **모른다고 적는다.** 커밋 트리를 읽은 빌드는 워킹트리가 같은지 알 수 없다.
            matches_worktree: Capable::not_built(CapabilityId::new("F01", "worktree-state")),
            built_for_this_snapshot: true,
            symbols_indexed: indexed,
        },
        Coverage {
            // 참조 해소가 없으므로 미해소를 **셀 수조차 없다**. 0 은 "없다"가 아니라
            // "이 빌드가 참조를 보지 않는다"이고, 그 사실은 capabilities 가 말한다.
            unresolved: 0,
            out_of_scope_files: report.ledger.counts().values().sum::<usize>()
                - report.ledger.counts().get(&pal_core::Bucket::Parsed).copied().unwrap_or(0)
                - report.ledger.counts().get(&pal_core::Bucket::Partial).copied().unwrap_or(0),
            lowest_grade: ExtractGrade::L0,
            identity: IdentityGrade::Ordinal,
        },
        capabilities(),
        LedgerRef::of(&report.ledger),
        // 자를 만큼의 답이 아직 없다. **그래도 명시한다.**
        Elision::none(),
    );

    if json {
        println!("{}", serde_json::to_string_pretty(&envelope)?);
    } else {
        print_screen(&envelope);
    }
    Ok(())
}

fn touch_result(report: &ledger::LedgerReport, symbol: SymbolNode) -> TouchResult {
    TouchResult {
        target: Coord {
            repo: report.ledger.snapshot.repo.clone(),
            tree: report.ledger.snapshot.tree,
            extractor: pal_extract::version(),
            symbol: symbol.id,
        },
        symbol,
        bindings: Capable::not_built(CapabilityId::new("F09", "binding")),
        facts: Capable::not_built(CapabilityId::new("F07", "reference-resolution")),
        unresolved: Capable::not_built(CapabilityId::new("F08", "unresolved-refs")),
        effects: Capable::not_built(CapabilityId::new("F13", "effects")),
        judgments: Capable::not_built(CapabilityId::new("F15", "judgment")),
    }
}

/// how-it-works §2.3 의 화면.
fn print_screen(envelope: &Envelope<TouchAnswer>) {
    println!();
    match &envelope.answer {
        TouchAnswer::Unknown { name } => {
            println!("  `{name}` 을 이 스냅샷에서 찾지 못했습니다.");
            println!();
            println!("  **없다는 뜻이 아닙니다** — 아래 근거가 무엇을 보았는지 말합니다.");
        }
        TouchAnswer::Ambiguous { name, candidates } => {
            println!("  `{name}` 의 후보가 {}건입니다. 하나를 고르지 않습니다.", candidates.len());
            println!();
            for c in candidates {
                println!("  {:<10} {:<24} {}:{}", c.kind.name(), c.name, c.path, c.span.line_start);
            }
        }
        TouchAnswer::Found(r) => {
            println!("  {}  ·  {}", r.symbol.name, r.target);
            println!("  {} · {}:{} · identity {} · body {}",
                     r.symbol.kind.name(), r.symbol.path, r.symbol.span.line_start,
                     r.symbol.identity.name(), r.symbol.body.short());
            println!();
            slot("이 좌표에 걸린 것", &r.bindings);
            slot("이 심볼이 하는 것", &r.facts);
            slot("내가 모르는 것", &r.unresolved);
            slot("효과", &r.effects);
            slot("판정", &r.judgments);
        }
    }

    let e = envelope;
    println!();
    println!("■ 이 답의 근거");
    println!("  Snapshot  {}@{}", e.snapshot.repo, e.snapshot.tree);
    println!("  대장      parsed {} · partial {} · unsupported {} · unrecognized {} / {} 파일",
             e.ledger.parsed, e.ledger.partial, e.ledger.unsupported, e.ledger.unrecognized,
             e.ledger.files_total);
    if e.ledger.unbindable_languages > 0 {
        println!("            결박 불가 언어 {}개 — 그 파일들에는 좌표가 없습니다",
                 e.ledger.unbindable_languages);
    }
    println!("  2층       심볼 {} 색인됨", e.projection.symbols_indexed);
    match &e.projection.matches_worktree {
        Capable::Present(v) => println!("  워킹트리  {}", if *v { "일치" } else { "다름" }),
        Capable::NotBuilt { capability } => println!(
            "  워킹트리  (이 빌드는 워킹트리 상태를 모릅니다 — {} 미구축)", capability.feature),
    }
    println!("  절단      {}", if e.elision.is_none() { "없음 (명시)" } else { "있음" });
    println!("  능력      {} · 미구축 {}",
             e.capabilities.built.join(" · "),
             e.capabilities.not_built.iter().map(|c| c.feature).collect::<Vec<_>>().join(" · "));
    println!();
}

/// 미구축 자리를 **빈 목록이 아니라 문장으로** 낸다.
fn slot<T>(title: &str, value: &Capable<T>) {
    println!("■ {title}");
    match value {
        Capable::NotBuilt { capability } => {
            println!("  (이 빌드에는 {} 능력이 없습니다 — {} 미구축)", capability.what, capability.feature);
        }
        Capable::Present(_) => println!("  (있음)"),
    }
}
