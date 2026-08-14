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
    BindingStatus, BoundItem, Capable, CapabilityId, CapabilitySet, Coord, Coverage, Elision,
    Envelope, ExtractGrade, Fold, FoldedPart, IdentityGrade, LedgerRef, LogStatus,
    NotRecorded, PROVISIONAL_STITCH_BATCH,
    ProjectionFreshness, QueryName, RebuildState, Slot, SymbolFacts, SymbolNode, TouchAnswer,
    TouchResult,
};
use pal_intent::IntentStore;
use pal_store::Projection;

use crate::ledger;

/// 이 빌드가 답하는 것과 아직 못 만든 것. **응답마다 실린다**(stack §5.3).
fn capabilities() -> CapabilitySet {
    CapabilitySet::new(
        vec![
            QueryName::LedgerSnapshot.name().to_owned(),
            QueryName::SymbolResolve.name().to_owned(),
            "binding.touch".to_owned(),
            // **F05 가 더한 것** — 파일 **안**의 참조 관계. 파일 경계를 넘는 것은 F07 이고,
            // 그 사실이 `coverage.unresolved` 에 수로 실린다.
            "symbol.references".to_owned(),
        ],
        vec![
            CapabilityId::new("F07", "cross-file-resolution"),
            CapabilityId::new("F08", "unresolved-refs"),
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
    intent_path: Option<PathBuf>,
    name: &str,
    json: bool,
) -> Result<()> {
    // 대장을 먼저 만든다. **답의 근거가 먼저 서야 답이 나간다.**
    let report = ledger::compute(repo_path, rev, cache_dir)?;

    let index = index_path.unwrap_or_else(|| repo_path.join(".palimpsest/index.redb"));
    let projection = Projection::open(&index).context("2층을 열지 못했다")?;
    // **1패스 스티칭.** 무대에 배치로 쓰고 한 트랜잭션에서 교체한다 — 읽는 쪽은
    // 옛 세대 전체 아니면 새 세대 전체만 본다(F05 §4 · `[f05.2.pass]` ③).
    let built_for = report.ledger.snapshot.to_string();
    let stitched = projection
        .stitch(&built_for, &report.stitches, PROVISIONAL_STITCH_BATCH)
        .context("2층을 세우지 못했다")?;
    let indexed = stitched.symbols;

    // **의도 저장소는 파생층과 다른 파일이다** — R-21. 2층을 지워도 이쪽은 남는다.
    let intent = IntentStore::open_read_only(&intent_file(repo_path, intent_path))
        .context("의도 저장소를 열지 못했다")?;

    let found = projection.resolve_name(name).context("2층을 읽지 못했다")?;
    let answer = match found.len() {
        0 => TouchAnswer::Unknown { name: name.to_owned() },
        1 => {
            let symbol = found.into_iter().next().expect("길이가 1 이다");
            let bound = bound_items(&intent, &projection, &symbol)?;
            // **파일 안의 호출 관계가 F05 에서 값이 된다.** 파일 경계를 넘는 것은 안 센다 —
            // 그 수는 `coverage.unresolved` 가 진다.
            let facts = SymbolFacts {
                callers: projection.callers(symbol.id).context("역방향을 읽지 못했다")?.len(),
                callees: projection.callees(symbol.id).context("정방향을 읽지 못했다")?.len(),
            };
            TouchAnswer::Found(Box::new(touch_result(&report, symbol, bound, facts)))
        }
        _ => TouchAnswer::Ambiguous { name: name.to_owned(), candidates: found },
    };

    // **범위는 답보다 먼저 계산한다** — 답이 무엇을 골랐는가가 범위를 정하기 때문이다.
    let coverage = coverage(&report, &projection, &answer);
    let envelope = Envelope::new(
        answer,
        report.ledger.snapshot.clone(),
        ProjectionFreshness {
            // **F01 이 이 자리를 값으로 바꿨다.** 워킹트리를 재고 이 답이 선 트리와
            // 대므로 이제 *"모른다"* 가 아니라 *"같다 / 다르다"* 를 적을 수 있다.
            matches_worktree: Capable::Present(
                report.worktree.matches(&report.ledger.snapshot_tree()),
            ),
            // **F05 가 이 자리를 값으로 바꿨다.** 무대가 서 있으면 재구축 중이고,
            // 그것을 읽을 수 있게 된 것이 배치 커밋의 부산물이다(DESIGN §12.7 격리 3번).
            rebuild: Capable::Present(if projection.rebuilding().unwrap_or(false) {
                RebuildState::Rebuilding
            } else {
                RebuildState::Settled
            }),
            // **관측이지 기본값이 아니다.** 옛 판은 `true` 로 박혀 있었고 그것은
            // *"이 스냅샷에서 만들어졌다"* 를 확인하지 않고 적은 것이었다.
            built_for_this_snapshot: projection
                .built_for()
                .unwrap_or_default()
                .is_some_and(|s| s == built_for),
            symbols_indexed: indexed,
        },
        coverage,
        capabilities(),
        LedgerRef::of(&report.ledger),
        // 자를 만큼의 답이 아직 없다. **그래도 명시한다.**
        Elision::none(),
        // **대장이 접혀 있다.** 이 답에 실린 것은 요약 여섯 값이고 전체는
        // `ledger.snapshot` 이 낸다 — 절단이 아니라 **부피를 옮긴 것**이다(F06 §4.3).
        접힌_대장(&report),
        // ⚠ **이 표면은 질의 로그를 안 쓴다.** F05 §5.3 은 *"모든 질의 실행"* 이라
        // 적었고 `pal touch` 는 아직 그 자리에 없다. **0 으로 세지 않고 이렇게 적는다** —
        // 조용히 빠지면 F17 이 미조회를 과대 계상한다.
        LogStatus::NotRecorded { why: NotRecorded::SurfaceDoesNotLog },
    );

    if json {
        println!("{}", serde_json::to_string_pretty(&envelope)?);
    } else {
        print_screen(&envelope);
    }
    Ok(())
}

/// 이 답에서 접힌 것 — **대장 하나.**
fn 접힌_대장(report: &ledger::LedgerReport) -> Fold {
    let mut fold = Fold::none();
    fold.push(FoldedPart::Ledger, report.ledger.total(), QueryName::LedgerSnapshot);
    fold
}

/// 의도 저장소 위치. **기본값이 2층과 다른 파일이다**(stack §2.4).
pub fn intent_file(repo_path: &Path, given: Option<PathBuf>) -> PathBuf {
    given.unwrap_or_else(|| repo_path.join(".palimpsest/intent.redb"))
}

/// 이 심볼에 걸린 것들과 그 상태.
///
/// **낡음은 여기서 계산된다** — 결박 시점의 요약과 2층의 현재 요약을 댄다.
/// 심볼이 사라졌으면 `Orphaned` 이고 그것은 `Stale` 과 다른 사건이다.
fn bound_items(
    intent: &IntentStore,
    projection: &Projection,
    symbol: &SymbolNode,
) -> Result<Vec<BoundItem>> {
    let bindings = intent.bound_to(symbol.id).context("결박을 읽지 못했다")?;
    let mut out = Vec::with_capacity(bindings.len());
    for b in bindings {
        let status = BindingStatus::evaluate(&b, |id| {
            projection.symbol(id).ok().flatten().map(|n| n.body)
        });
        out.push(BoundItem::Note { binding: b.id, note: b.note, status });
    }
    Ok(out)
}

/// 이 답이 **무엇을 못 봤는가** — 질의마다 다른 값이어야 한다.
///
/// # 왜 전역 합이 아닌가 (`[f05.3.pass]` ⑤)
///
/// `UNRESOLVED` 전체 수를 복사하면 **질의와 무관한 값**이 되고, 서로 다른 두 질의가
/// 같은 `coverage` 를 낸다. 그러면 그 숫자는 답의 성질이 아니라 저장소의 성질이다.
/// 그래서 답을 찾은 경우에는 **그 심볼이 사는 파일**의 미해소 수를 싣는다.
///
/// 못 찾은 경우(`Unknown`·`Ambiguous`)에는 **본 것 전체**가 근거다 — 어느 파일도
/// 고르지 않았으므로 전부가 이 답의 범위다.
fn coverage(
    report: &ledger::LedgerReport,
    projection: &Projection,
    answer: &TouchAnswer,
) -> Coverage {
    let counts = report.ledger.counts();
    let out_of_scope = counts.values().sum::<usize>()
        - counts.get(&pal_core::Bucket::Parsed).copied().unwrap_or(0)
        - counts.get(&pal_core::Bucket::Partial).copied().unwrap_or(0);

    let rows = match answer {
        TouchAnswer::Found(r) => projection
            .file(&r.symbol.path)
            .ok()
            .flatten()
            .map(|f| vec![f])
            .unwrap_or_default(),
        _ => projection.files().unwrap_or_default(),
    };

    // **미해소를 셀 수 있게 된 것이 F05 다.** 옛 판은 0 이었고 그것은 *"없다"* 가 아니라
    // *"이 빌드가 참조를 안 본다"* 였다. 스코프 체인이 없는 파일은 지금도 셀 수 없고,
    // 그 사실은 `Capable` 이 파일 노드에서 진다 — 여기서는 **셀 수 있는 것만 더한다.**
    let unresolved = rows
        .iter()
        .filter_map(|f| match &f.refs {
            Slot::Built(c) => Some(c.unresolved),
            Slot::NotBuilt => None,
        })
        .sum::<usize>();

    Coverage {
        unresolved,
        out_of_scope_files: out_of_scope,
        // **이 답이 경유한 가장 낮은 등급.** 상수가 아니다.
        lowest_grade: rows.iter().map(|f| f.grade).min().unwrap_or(ExtractGrade::L0),
        identity: match answer {
            TouchAnswer::Found(r) => r.symbol.identity,
            // 심볼을 하나로 고르지 못했으면 이 답이 선 정체성은 가장 낮은 것이다.
            _ => IdentityGrade::Ordinal,
        },
    }
}

fn touch_result(
    report: &ledger::LedgerReport,
    symbol: SymbolNode,
    bound: Vec<BoundItem>,
    facts: SymbolFacts,
) -> TouchResult {
    // 좌표는 **저장소 하나**를 가리킨다. 스냅샷은 집합이므로 그중 하나를 골라야 하고,
    // 이 빌드는 저장소를 하나만 본다(대장의 `repos_declared` 가 언제나 1 이다).
    // **멀티레포에서는 심볼이 어느 저장소의 것인지를 `SymbolNode` 가 실어야 하고,
    // 그것은 F14 다** — 여기서 조용히 첫 것을 고르는 대신 그 사실을 적어 둔다.
    let (repo, tree) = report
        .ledger
        .snapshot
        .entries()
        .next()
        .expect("스냅샷은 비어 있을 수 없다");
    TouchResult {
        target: Coord {
            repo: repo.clone(),
            tree: *tree,
            extractor: pal_extract::version(),
            symbol: symbol.id,
        },
        symbol,
        // **S2 에서는 여기가 NotBuilt 였다.** 채워지는 것이 S3 의 인수 기준 ② 다.
        bindings: Capable::Present(bound),
        // **F05 에서 값이 된다** — 파일 **안**의 호출 관계다. 파일 경계를 넘는 것은
        // 여기 없고 그 수는 봉투의 `coverage.unresolved` 가 진다.
        facts: Capable::Present(facts),
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
            print_bindings(&r.bindings);
            slot("이 심볼이 하는 것", &r.facts);
            slot("내가 모르는 것", &r.unresolved);
            slot("효과", &r.effects);
            slot("판정", &r.judgments);
        }
    }

    let e = envelope;
    println!();
    println!("■ 이 답의 근거");
    println!("  Snapshot  {}", e.snapshot);
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
    // **모른다는 것도 화면에 선다.** 산출에만 있고 화면에 없으면 사람은 그 공백을 못 본다.
    match &e.projection.rebuild {
        Capable::Present(RebuildState::Rebuilding) =>
            println!("  재구축    진행 중 — 이 답은 열린 스냅샷 위에 섰습니다"),
        Capable::Present(RebuildState::Settled) => println!("  재구축    아님"),
        Capable::NotBuilt { capability } => println!(
            "  재구축    (이 빌드는 재구축 중인지 모릅니다 — {} 미구축)", capability.feature),
    }
    println!("  절단      {}", if e.elision.is_none() { "없음 (명시)" } else { "있음" });
    crate::evidence::print(e);
    println!("  능력      {} · 미구축 {}",
             e.capabilities.built.join(" · "),
             e.capabilities.not_built.iter().map(|c| c.feature).collect::<Vec<_>>().join(" · "));
    println!();
}

/// 결박을 띄운다 — **제품의 형태가 처음으로 보이는 자리다.**
fn print_bindings(value: &Capable<Vec<BoundItem>>) {
    let Capable::Present(items) = value else {
        println!("■ 이 좌표에 걸린 것");
        println!("  (이 빌드에는 binding 능력이 없습니다)");
        return;
    };
    println!("■ 이 좌표에 걸린 것 ({})", items.len());
    if items.is_empty() {
        // **여기의 빈 목록은 정직하다** — 능력이 있고 값이 없는 것이다.
        println!("  아직 없습니다.");
        return;
    }
    for item in items {
        let BoundItem::Note { binding, note, status } = item;
        let mark = match &status.code {
            pal_core::CodeFreshness::Live => "live".to_owned(),
            pal_core::CodeFreshness::Stale { triggered_by } =>
                format!("STALE ← {} 개가 변했습니다", triggered_by.len()),
            pal_core::CodeFreshness::Orphaned { missing } =>
                format!("ORPHANED ← 좌표 {} 개가 사라졌습니다", missing.len()),
        };
        println!("  [{}] {mark}", binding.as_str());
        for line in note.lines() {
            println!("      {line}");
        }
    }
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
