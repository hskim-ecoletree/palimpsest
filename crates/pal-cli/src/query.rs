//! `pal query` — 이름 붙은 질의 하나.
//!
//! # 이 명령이 하는 일은 **조립뿐이다**
//!
//! 대장은 `ledger`, 스티칭은 `pal-store`, 실행은 `pal-query` 다. 여기 있는 것은
//! *"어느 예산으로 어느 질의를"* 뿐이고, **정책이 여기 있으면 안 된다.**
//!
//! # 예산을 손잡이로 준다 — 그리고 그 값이 답에 실린다
//!
//! `--depth-max` · `--node-max` 로 낮출 수 있다. **끄는 손잡이는 없다** — `Budget` 에
//! `Default` 도 `unlimited()` 도 없고, 안 주면 자리표시가 들어간다.
//! 낮추면 절단이 일어나고 **어느 상한에 얼마나 걸렸는지가 봉투에 실린다.**

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use pal_core::{
    Budget, CANDIDATE_LIMIT, Capable, Envelope, PROVISIONAL_PATH_PRODUCT_MAX,
    PROVISIONAL_STITCH_BATCH, PROVISIONAL_TRAVERSAL_DEPTH, PROVISIONAL_VIEW_NODE_MAX, QueryName,
};
use pal_query::{NamedQuery, QueryCtx, QueryResult};
use pal_store::Projection;

use crate::ledger;

pub struct Args<'a> {
    pub name: &'a str,
    pub arg: Option<&'a str>,
    pub list: bool,
    pub repo: &'a Path,
    pub rev: Option<&'a str>,
    pub cache_dir: Option<PathBuf>,
    pub index: Option<PathBuf>,
    pub depth_max: Option<usize>,
    pub node_max: Option<usize>,
    pub json: bool,
}

/// # Errors
/// 저장소·캐시·2층 중 하나에 닿지 못하거나 질의 이름을 모르면.
pub fn run(a: Args) -> Result<()> {
    if a.list {
        return list(a.json);
    }

    let Some(query) = NamedQuery::parse(a.name, a.arg) else {
        // **"모르는 이름"과 "인자가 없다"를 가른다.** 뭉개면 사용자가 무엇을 고칠지 모른다.
        anyhow::bail!(
            "질의 `{}`{} — 아는 것은 {} 다",
            a.name,
            if QueryName::parse(a.name).is_some() { " 에 인자가 필요하다" } else { " 를 모른다" },
            QueryName::ALL.map(QueryName::name).join(" · ")
        );
    };

    let report = ledger::compute(a.repo, a.rev, a.cache_dir)?;
    let index = a.index.unwrap_or_else(|| a.repo.join(".palimpsest/index.redb"));
    let projection = Projection::open(&index).context("2층을 열지 못했다")?;

    let built_for = report.ledger.snapshot.to_string();
    let stitched = projection
        .stitch(&built_for, &report.stitches, PROVISIONAL_STITCH_BATCH)
        .context("2층을 세우지 못했다")?;

    let counts = report.ledger.counts();
    let out_of_scope = counts.values().sum::<usize>()
        - counts.get(&pal_core::Bucket::Parsed).copied().unwrap_or(0)
        - counts.get(&pal_core::Bucket::Partial).copied().unwrap_or(0);

    let ctx = QueryCtx {
        projection: &projection,
        snapshot: report.ledger.snapshot.clone(),
        ledger: pal_core::LedgerRef::of(&report.ledger),
        freshness: pal_query::freshness(
            Capable::Present(report.worktree.matches(&report.ledger.snapshot_tree())),
            projection.rebuilding().unwrap_or(false),
            projection.built_for().unwrap_or_default().is_some_and(|s| s == built_for),
            stitched.symbols,
        ),
        capabilities: pal_query::capabilities(),
        // **넷을 전부 넘긴다.** 안 넘길 수 있는 경로가 없다.
        budget: Budget::new(
            CANDIDATE_LIMIT,
            PROVISIONAL_PATH_PRODUCT_MAX,
            a.depth_max.unwrap_or(PROVISIONAL_TRAVERSAL_DEPTH),
            a.node_max.unwrap_or(PROVISIONAL_VIEW_NODE_MAX),
        ),
        out_of_scope_files: out_of_scope,
    };

    let envelope = pal_query::execute(&query, &ctx).context("질의가 실패했다")?;
    if a.json {
        println!("{}", serde_json::to_string_pretty(&envelope)?);
    } else {
        print_screen(&query, &envelope);
    }
    Ok(())
}

/// `--list` — **답하는 것과 아직 못 만든 것을 함께 낸다.**
///
/// # 왜 둘을 함께 내는가
///
/// 카탈로그가 **여섯만** 담는다(`[f06].catalog_scope_decision`). 문서 §3 의 표는 26 인데
/// 이 빌드가 답하는 것은 여섯이고, 목록이 여섯만 보이면 소비자가 *"이것이 이 제품의
/// 전부"* 로 읽는다. 그래서 **못 만든 것이 같은 화면에 선다.**
///
/// **그러나 못 만든 것은 이름으로 적지 않는다** — 이름을 적으면 그것이 곧 스무 개의
/// 빈 자리이고 *"있는데 비어 있다"* 로 읽힌다(S2 의 규율). 기능 번호와 능력 이름으로
/// 적는다. 그것이 [`pal_core::CapabilitySet`] 이 이미 지고 있는 형태다.
///
/// **저장소를 안 읽는다.** 이 경로가 호스트도 저장소도 없이 서는 것이
/// *"호스트 없이도 코어가 답한다"* 의 가장 얕은 층이다.
fn list(json: bool) -> Result<()> {
    let caps = pal_query::capabilities();
    if json {
        // **기계가 읽는 표면.** 사람용 장식이 하나도 없다 — 파이프의 다음 단계가
        // 이것을 파싱한다(`[f06.3.pass]` ②).
        let built: Vec<_> = QueryName::ALL
            .into_iter()
            .map(|q| {
                serde_json::json!({
                    "name": q.name(),
                    "summary": q.summary(),
                    "args": q.arg_names().iter().zip(q.arg_types())
                        .map(|(n, t)| serde_json::json!({"name": n, "type": t, "required": true}))
                        .collect::<Vec<_>>(),
                    "returns": q.returns(),
                    "introduced": q.introduced(),
                })
            })
            .collect();
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "built": built,
                "not_built": caps.not_built,
            }))?
        );
        return Ok(());
    }

    println!();
    println!("■ 이 빌드가 답하는 질의 {}", QueryName::ALL.len());
    for q in QueryName::ALL {
        let args = q.arg_names().iter().map(|n| format!("<{n}>")).collect::<Vec<_>>().join(" ");
        println!("  {:<32} {}", format!("{} {args}", q.name()).trim_end(), q.summary());
    }
    println!();
    println!("■ 이 빌드가 아직 못 만든 능력 {}", caps.not_built.len());
    for c in &caps.not_built {
        println!("  {:<5} {}", c.feature, c.what);
    }
    println!();
    println!("  **이름을 적지 않습니다** — 아직 만들지 않은 질의의 이름을 적으면");
    println!("  그것이 곧 빈 자리이고, 빈 자리는 「있는데 비어 있다」로 읽힙니다.");
    println!();
    Ok(())
}

fn print_screen(q: &NamedQuery, e: &Envelope<QueryResult>) {
    println!();
    println!("■ {}  {}", q.name().name(), q.args());
    println!();
    match &e.answer {
        QueryResult::Ledger { ledger } => {
            println!("  파일 {} · parsed {} · partial {}", ledger.files_total, ledger.parsed, ledger.partial);
        }
        // 둘의 화면이 같다 — 답의 모양이 같고, 다른 것은 봉투가 진다.
        QueryResult::Symbols { symbols } | QueryResult::Reached { symbols, .. } => {
            print_symbols(symbols);
        }
        QueryResult::Graph { nodes, edges } => {
            println!("  노드 {} · 엣지 {}", nodes.len(), edges.len());
        }
        QueryResult::Ambiguous { name, candidates } => {
            println!("  `{name}` 의 후보가 {}건입니다. 하나를 고르지 않습니다.", candidates.len());
            print_symbols(candidates);
        }
        QueryResult::Unknown { name } => {
            println!("  `{name}` 을 이 스냅샷에서 찾지 못했습니다.");
            println!();
            println!("  **없다는 뜻이 아닙니다** — 아래 근거가 무엇을 보았는지 말합니다.");
        }
    }

    println!();
    println!("■ 이 답의 근거");
    println!("  Snapshot  {}", e.snapshot);
    println!("  2층       심볼 {} 색인됨", e.projection.symbols_indexed);
    println!(
        "  범위      미해소 {} · 범위 밖 {} 파일 · 최저 등급 {} · identity {}",
        e.coverage.unresolved,
        e.coverage.out_of_scope_files,
        e.coverage.lowest_grade.name(),
        e.coverage.identity.name()
    );
    print_elision(e);
    crate::evidence::print(e);
    println!(
        "  능력      {} · 미구축 {}",
        e.capabilities.built.join(" · "),
        e.capabilities.not_built.iter().map(|c| c.feature).collect::<Vec<_>>().join(" · ")
    );
    println!();
}

/// **자른 것을 화면에도 적는다.** 산출에만 있고 화면에 없으면 사람은 그 공백을 못 본다.
fn print_elision(e: &Envelope<QueryResult>) {
    if e.elision.is_none() {
        println!("  절단      없음 (명시)");
        return;
    }
    println!("  절단      {}건", e.elision.dropped());
    for t in &e.elision.truncated {
        println!("            {} {}건", t.reason.name(), t.count);
    }
    for l in &e.elision.limits_hit {
        println!("            ← {} = {}", l.limit.name(), l.value);
    }
}

fn print_symbols(symbols: &[pal_core::SymbolNode]) {
    if symbols.is_empty() {
        // **능력이 있고 값이 없다.** 그 빈 목록은 정직하다 — 근거가 아래 붙는다.
        println!("  (없음)");
        return;
    }
    for s in symbols {
        println!("  {:<10} {:<24} {}:{}", s.kind.name(), s.name, s.path, s.span.line_start);
    }
}
