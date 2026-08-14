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
    PROVISIONAL_TRAVERSAL_DEPTH, PROVISIONAL_VIEW_NODE_MAX, QueryName,
};
use pal_query::{NamedQuery, QueryCtx, QueryResult};

use pal_intent::IntentStore;

use crate::attach;
use crate::ledger;
use crate::touch;

pub struct Args<'a> {
    pub name: &'a str,
    pub arg: Option<&'a str>,
    pub list: bool,
    pub repo: &'a Path,
    pub rev: Option<&'a str>,
    pub cache_dir: Option<PathBuf>,
    pub index: Option<PathBuf>,
    /// 의도 저장소 위치. 기본값은 `<저장소>/.palimpsest/intent.redb`
    pub intent: Option<PathBuf>,
    pub depth_max: Option<usize>,
    pub node_max: Option<usize>,
    /// **읽기 전용으로 붙는다** — 스티칭을 안 하고 질의 로그를 못 남긴다.
    pub read_only: bool,
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

    // **붙는 방법이 둘이고 그 갈림이 답에 실린다**(`[f06.3.pass]` ③).
    //
    // 기본은 **쓰기**다. 읽기가 기본이면 질의 로그가 조용히 안 쌓이고, F17 은
    // 데이터가 없어 착수할 수 없다(F05 §5.3). `--read-only` 는 명시해야 켜진다.
    let attached = attach::attach(
        &index,
        &report,
        if a.read_only { attach::How::ReadOnly } else { attach::How::Stitching },
    )?;
    let built_for_this = attached.built_for_this_snapshot();
    let attach::Attached { projection, indexed, .. } = attached;

    // **의도 저장소는 읽기로만 연다** — 이 명령은 결박을 안 만든다.
    // 파일이 없으면 결박이 0 건이고 **그것이 정확한 값**이다(아직 아무도 안 걸었다).
    let intent = IntentStore::open_read_only(&touch::intent_file(a.repo, a.intent))
        .context("의도 저장소를 열지 못했다")?;
    let bindings = intent.all().context("결박을 읽지 못했다")?;

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
            built_for_this,
            indexed,
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
        bindings,
        // **대장에서 뜬다** — 이름으로 세면 칸이 하나 늘 때 조용히 빠진다.
        partial_files: report
            .ledger
            .entries
            .iter()
            .filter(|e| e.state.bucket() == pal_core::Bucket::Partial)
            .map(|e| e.path.clone())
            .collect(),
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
        QueryResult::Bindings { bindings } => print_bindings(bindings),
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

/// 결박마다 한 줄 — **상태 · 반경 · 무엇이 켰는가를 함께 낸다.**
///
/// F09 §5 의 마지막 행이 요구한 것이다: *"`stale` 출력에 **`triggered_by` 와 반경을
/// 항상 붙여** 행동 가능하게 만든다."* 상태만 적으면 사람이 어디를 볼지 모르고,
/// 그러면 표시를 무시하기 시작한다 — 그것이 [목표 G1] 의 반증 조건이다.
///
/// [목표 G1]: ../../../docs/plan/00-goals.md
fn print_bindings(bindings: &[pal_core::BindingReport]) {
    if bindings.is_empty() {
        // **빈 목록이 정직하다** — 능력이 있고 값이 없는 것이다. `not_built` 가 아니다.
        println!("  결박이 아직 없습니다.");
        return;
    }
    println!("  결박 {}건", bindings.len());
    println!();
    for b in bindings {
        let mark = match &b.status.code {
            pal_core::CodeFreshness::Live => "live".to_owned(),
            pal_core::CodeFreshness::Stale { triggered_by } => {
                format!("STALE ← {} 개가 변했습니다", triggered_by.len())
            }
            pal_core::CodeFreshness::Orphaned { missing } => {
                format!("ORPHANED ← 좌표 {} 개가 사라졌습니다", missing.len())
            }
            // **`live` 와 같은 화면이 되면 안 된다** — *"유효하다"* 와 *"유효한지 알 수
            // 없다"* 가 같은 줄로 나오는 것이 R16 이 겨냥한 실패다.
            pal_core::CodeFreshness::Undeterminable { reason, at } => {
                format!("판정 불가 ← {} ({} 개 좌표)", reason.name(), at.len())
            }
        };
        let 계보 = match &b.status.lineage {
            pal_core::Lineage::Current => String::new(),
            pal_core::Lineage::Superseded { by } => format!(" · 대체됨 → {}", by.to_display()),
        };
        println!("  [{}] {mark}{계보}", b.binding.as_str());
        // **반경이 상태와 같은 줄에 있다** — *"이 결정은 `symbol` 반경에서 live"* 는
        // *"이 결정은 유효하다"* 와 다른 문장이다(F09 §3).
        let 등급 = b
            .watch_grades
            .iter()
            .map(|(g, n)| format!("{g} {n}"))
            .collect::<Vec<_>>()
            .join(" · ");
        println!("      반경 {} · 감시 {} 개 · 등급 {{{등급}}}", b.radius, b.watch);
        println!("      {}  ·  {}", b.subject, 시각(b.bound_at_time));
        for line in b.note.lines() {
            println!("      {line}");
        }
        println!();
    }
    println!("  **반경 밖의 변경은 여기 안 뜹니다** — 거짓 음성은 원리적으로 안 닫힙니다.");
    println!("  선언된 반경이 위에 적혀 있고, 그것이 이 도구가 할 수 있는 전부입니다.");
}

/// 결박한 코드의 시각 — **표시용이다.** *"3주 전 코드 기준"* 이 *"12커밋 전"* 보다 읽힌다.
fn 시각(t: pal_core::BoundTime) -> String {
    match t {
        pal_core::BoundTime::Committed { epoch_secs } => format!("커밋 시각 {epoch_secs}"),
        pal_core::BoundTime::Worktree => "워킹트리 (커밋 없음)".to_owned(),
        // **「없다」와 「모른다」를 가른다.** 옛 판 파일에서 읽은 결박이다.
        pal_core::BoundTime::Unrecorded => "시각 안 적힘 (옛 판)".to_owned(),
    }
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
