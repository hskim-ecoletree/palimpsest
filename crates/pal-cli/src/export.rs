//! `pal export` — **자유 탐색을 여는 자리** (옛 F06 §4.1 · `[f06.3.pass]` ④).
//!
//! # 이 명령이 검사하는 것은 F05 의 근거 하나다
//!
//! 옛 F05 §2 가 그래프 DB 를 안 쓰기로 한 근거 다섯 중 마지막이
//! *"그래프 DB 방향은 `pal export`(Cypher/GraphML/Parquet)로 만족시킨다"* 였다.
//! **그 만족이 없는 동안 그 근거는 검사되지 않는 주장이었다.** 형식 하나만 서면
//! 주장이 관측이 된다.
//!
//! 그리고 옛 F06 §6 이 자유 질의 언어 노출(Cypher 패스스루)을 기각하면서 적었다 —
//! *"저장 기술이 표면 계약이 되어 교체 불가능해진다. **자유 탐색은 `pal export` 로
//! 연다**"*. 내보내기는 **한 방향**이라 저장 기술을 계약으로 만들지 않는다.
//!
//! # 라벨을 손으로 안 쓴다 — 스키마에서 온다
//!
//! `schema/graph.toml` 의 머리가 파생 다섯 중 ④를 *"`pal export` 의 매핑"* 이라 적고
//! *"없는 소비자를 위해 파생을 만들면 그것이 곧 검사되지 않는 산출이다"* 라고 적었다.
//! **이 명령이 그 소비자다.** 그래서 라벨은 Rust 타입 이름으로 스키마를 **찾아서**
//! 온다 — `"Symbol"` 이라는 문자열이 이 파일에 없다.
//!
//! # 못 낸 것을 센다
//!
//! 스키마의 노드 여덟 · 엣지 여덟 중 2층에 실제로 사는 것은 일부다. 나머지를 조용히
//! 빼면 소비자는 **이 그래프가 전부인 줄 안다.** 그래서 못 낸 라벨을 사유와 함께
//! 적는다 — `not_built`(그 기능이 아직 안 만들었다)와 `not_stored`(계산은 되지만
//! 2층에 안 산다)를 **가른다**([ADR-0002]).
//!
//! [ADR-0002]: ../../../docs/adr/0002-empty-population-is-not-zero-violations.md

use std::fmt::Write as _;
use std::path::PathBuf;

use anyhow::{Context, Result};
use pal_core::{
    Capable, CapabilityId, Coverage, Elision, Envelope, ExtractGrade, Fold, FoldedPart,
    GraphSchema, IdentityGrade, LedgerRef, LogStatus, NodeStatus, NotRecorded,
    ProjectionFreshness, QueryName, RebuildState,
};
use pal_store::Projection;
use serde::Serialize;

use crate::ledger;

/// 스키마 정본. **읽는 경로가 하나다** — `pal doctor` 와 같은 파일을 읽는다.
const SCHEMA: &str = include_str!("../../../schema/graph.toml");

/// 이 명령이 아는 형식. **하나다** — 그리고 그것이 정직한 상태다.
///
/// `graphml`·`parquet` 은 크레이트를 요구하고 [스택 §3.4] 가 근거를 요구한다.
/// 옛 F06 §8 이 *"형식 하나만이라도"* 라고 적었고, 하나가 서면 F05 의 근거가 검사된다.
///
/// [스택 §3.4]: ../../../docs/plan/00-stack.md
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum Format {
    /// 텍스트다. **새 의존을 안 들인다.**
    Cypher,
}

pub struct Args {
    pub repo: PathBuf,
    pub rev: Option<String>,
    pub cache_dir: Option<PathBuf>,
    pub index: Option<PathBuf>,
    pub format: Format,
    /// 낼 파일. 없으면 표준출력으로 가고, 그때 `--json` 은 쓸 수 없다.
    pub out: Option<PathBuf>,
    pub json: bool,
}

/// 내보내기 한 회차의 답 — **무엇을 냈고 무엇을 못 냈는가.**
#[derive(Debug, Clone, Serialize)]
pub struct ExportReport {
    pub format: &'static str,
    /// 라벨별 건수. **낸 것.**
    pub exported: Vec<Counted>,
    /// 못 낸 라벨 — 사유와 함께. **0 건이 아니라 「없음」이다.**
    pub missing: Vec<Missing>,
    pub bytes: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct Counted {
    pub label: String,
    pub count: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct Missing {
    pub label: String,
    #[serde(flatten)]
    pub why: MissingReason,
}

/// 왜 못 냈는가. **둘을 가른다** — 뭉개면 *"안 만들었다"* 와 *"여기 안 산다"* 가 같아진다.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case", tag = "why")]
pub enum MissingReason {
    /// 스키마가 `not_built` 이라 적었다 — 그 기능이 아직 안 만들었다.
    NotBuilt { by: String },
    /// 값은 계산되지만 **2층에 안 산다.** 결박은 의도 저장소에, 사슬은 `pal defect` 에.
    NotStored { lives_in: &'static str },
}

/// # Errors
/// 저장소·2층에 닿지 못하거나, 스키마가 안 읽히거나, 파일을 쓰지 못하면.
pub fn run(a: Args) -> Result<()> {
    if a.json && a.out.is_none() {
        // 둘 다 표준출력으로 가면 소비자가 어느 것을 파싱할지 모른다.
        anyhow::bail!("`--json` 은 `--out <파일>` 과 함께 써야 한다 — 산출 둘이 한 줄기로 못 간다");
    }

    let schema = GraphSchema::parse(SCHEMA).map_err(|e| anyhow::anyhow!("{e}"))?;
    let report = ledger::compute(&a.repo, a.rev.as_deref(), a.cache_dir)?;
    let index = a.index.unwrap_or_else(|| a.repo.join(".palimpsest/index.redb"));

    // ★ **읽기 전용으로 붙는다.** 내보내기가 2층을 쓰면 그것은 내보내기가 아니다.
    // 그리고 이것이 `[f06.3.pass]` ③ 의 첫 소비자다 — 쓰는 프로세스가 살아 있어도 붙는다.
    let projection = Projection::open_read_only(&index)
        .context("2층에 읽기 전용으로 붙지 못했다 — 먼저 `pal query` 로 한 번 세워야 한다")?;

    let (text, exported) = match a.format {
        Format::Cypher => cypher(&projection, &schema)?,
    };

    let missing = missing_labels(&schema, &exported);
    let out_report = ExportReport {
        format: "cypher",
        exported,
        missing,
        bytes: text.len(),
    };

    match &a.out {
        Some(path) => {
            std::fs::write(path, &text)
                .with_context(|| format!("쓰지 못했다: {}", path.display()))?;
        }
        None => print!("{text}"),
    }

    let envelope = envelope(&report, &projection, out_report);
    if a.json {
        println!("{}", serde_json::to_string_pretty(&envelope)?);
    } else if a.out.is_some() {
        print_screen(&envelope);
    } else {
        // 산출이 표준출력을 쓰고 있다. **근거는 표준오류로 간다** — 섞으면
        // 파이프의 다음 단계가 깨진다(`[f06.3.pass]` ②).
        eprint_screen(&envelope);
    }
    Ok(())
}

/// 라벨 하나를 **스키마에서** 찾는다 — Rust 타입 이름으로.
///
/// 이 함수가 없으면 `"Symbol"` 같은 문자열이 이 파일에 박히고, 그 순간 라벨이
/// 스키마와 **두 곳**에 산다.
fn node_label(schema: &GraphSchema, rust_type: &str) -> Result<String> {
    schema
        .nodes
        .values()
        .find(|n| n.rust_type == rust_type)
        .map(|n| n.label.clone())
        .with_context(|| format!("스키마에 `{rust_type}` 을 담는 노드가 없다"))
}

/// 엣지 라벨을 **스키마에서** 찾는다 — 운반자 Rust 타입 이름으로.
fn edge_label(schema: &GraphSchema, rust_type: &str) -> Result<String> {
    schema
        .edges
        .iter()
        .find(|(_, e)| e.carried_by.carrier().is_some_and(|c| c.rust_type == rust_type))
        .map(|(label, _)| label.clone())
        .with_context(|| format!("스키마에 `{rust_type}` 이 운반하는 엣지가 없다"))
}

fn cypher(p: &Projection, schema: &GraphSchema) -> Result<(String, Vec<Counted>)> {
    let file_label = node_label(schema, "FileNode")?;
    let symbol_label = node_label(schema, "SymbolNode")?;
    let ref_label = edge_label(schema, "ReferenceEdge")?;

    let files = p.files().context("파일을 읽지 못했다")?;
    let (symbols, edges) = p.dump().context("2층을 읽지 못했다")?;

    let mut o = String::new();
    o.push_str("// pal export --format cypher\n");
    o.push_str("// 라벨은 schema/graph.toml 에서 온다. 손으로 쓰지 않는다.\n");
    o.push_str("// **여기 없는 라벨은 0 건이 아니라 「이 빌드가 안 담는다」이다** — 동반 산출을 보라.\n\n");

    for f in &files {
        let _ = writeln!(
            o,
            "CREATE (:{file_label} {{path: {}, language: {}, grade: {}}});",
            quote(f.path.as_str()),
            quote(f.language.as_str()),
            quote(grade_name(f.grade))
        );
    }
    for s in &symbols {
        let _ = writeln!(
            o,
            "CREATE (:{symbol_label} {{id: {}, name: {}, kind: {}, path: {}, \
             line_start: {}, identity: {}}});",
            quote(&s.id.to_string()),
            quote(&s.name),
            quote(s.kind.name()),
            quote(s.path.as_str()),
            s.span.line_start,
            quote(identity_name(s.identity))
        );
    }
    for (from, to) in &edges {
        let _ = writeln!(
            o,
            "MATCH (a:{symbol_label} {{id: {}}}), (b:{symbol_label} {{id: {}}}) \
             CREATE (a)-[:{ref_label}]->(b);",
            quote(&from.to_string()),
            quote(&to.to_string())
        );
    }

    let counted = vec![
        Counted { label: file_label, count: files.len() },
        Counted { label: symbol_label, count: symbols.len() },
        Counted { label: ref_label, count: edges.len() },
    ];
    Ok((o, counted))
}

/// Cypher 문자열 하나. **따옴표와 역슬래시를 벗어난다** — 이름에 따옴표가 있는
/// 심볼 하나가 산출 전체를 깨뜨린다.
fn quote(s: &str) -> String {
    let mut o = String::with_capacity(s.len() + 2);
    o.push('"');
    for c in s.chars() {
        match c {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            '\n' => o.push_str("\\n"),
            '\r' => o.push_str("\\r"),
            _ => o.push(c),
        }
    }
    o.push('"');
    o
}

const fn grade_name(g: ExtractGrade) -> &'static str {
    g.name()
}

const fn identity_name(i: IdentityGrade) -> &'static str {
    i.name()
}

/// 스키마에 있는데 **안 나간** 라벨 — 사유와 함께.
///
/// **0 건으로 세지 않는다.** 소비자가 이 Cypher 를 그래프 DB 에 붓고 *"결박이 없네"*
/// 라고 읽으면 그것은 이 도구가 만든 오해다([ADR-0002]).
fn missing_labels(schema: &GraphSchema, exported: &[Counted]) -> Vec<Missing> {
    let 나간: Vec<&str> = exported.iter().map(|c| c.label.as_str()).collect();
    let mut out = Vec::new();

    for (label, decl) in &schema.nodes {
        if 나간.contains(&label.as_str()) {
            continue;
        }
        out.push(Missing {
            label: label.clone(),
            why: match &decl.status {
                NodeStatus::NotBuilt { by } => MissingReason::NotBuilt { by: by.clone() },
                NodeStatus::Built => MissingReason::NotStored { lives_in: lives_in(label) },
            },
        });
    }
    for label in schema.edges.keys() {
        if 나간.contains(&label.as_str()) {
            continue;
        }
        out.push(Missing {
            label: label.clone(),
            why: MissingReason::NotStored { lives_in: lives_in(label) },
        });
    }
    out
}

/// 이 라벨의 값은 **어디 사는가.** 2층이 아닌 자리를 이름으로 적는다.
fn lives_in(label: &str) -> &'static str {
    match label {
        "Binding" | "BOUND_TO" => "의도 저장소 (intent.redb · R-21 로 파일이 갈려 있다)",
        "Actor" | "Change" | "Defect" | "AUTHORED_BY" | "TOUCHES" | "FOLLOWS"
        | "MANIFESTS_AT" | "INTRODUCED_BY" | "RESOLVED_BY" => {
            "`pal defect` 가 계산만 하고 저장하지 않는다"
        }
        _ => "이 빌드의 2층에 없다",
    }
}

fn envelope(
    report: &ledger::LedgerReport,
    projection: &Projection,
    answer: ExportReport,
) -> Envelope<ExportReport> {
    let counts = report.ledger.counts();
    let out_of_scope = counts.values().sum::<usize>()
        - counts.get(&pal_core::Bucket::Parsed).copied().unwrap_or(0)
        - counts.get(&pal_core::Bucket::Partial).copied().unwrap_or(0);
    let mut fold = Fold::none();
    fold.push(FoldedPart::Ledger, report.ledger.total(), QueryName::LedgerSnapshot);

    Envelope::new(
        answer,
        report.ledger.snapshot.clone(),
        ProjectionFreshness {
            matches_worktree: Capable::Present(
                report.worktree.matches(&report.ledger.snapshot_tree()),
            ),
            // 읽기 전용으로 붙었으므로 무대를 볼 수 있다 — 재구축 중인지 관측된다.
            rebuild: Capable::Present(if projection.rebuilding().unwrap_or(false) {
                RebuildState::Rebuilding
            } else {
                RebuildState::Settled
            }),
            built_for_this_snapshot: projection
                .built_for()
                .unwrap_or_default()
                .is_some_and(|s| s == report.ledger.snapshot.to_string()),
            symbols_indexed: projection.count().unwrap_or(0),
        },
        Coverage {
            unresolved: 0,
            out_of_scope_files: out_of_scope,
            lowest_grade: ExtractGrade::L0,
            identity: IdentityGrade::Ordinal,
        },
        pal_query::capabilities(),
        LedgerRef::of(&report.ledger),
        // 내보내기는 **전부를 낸다** — 자르지 않는다. 그래서 명시적으로 없음이다.
        Elision::none(),
        fold,
        // ★ 읽기 전용으로 붙었으므로 못 남긴다. **조용히 안 남기지 않는다.**
        LogStatus::NotRecorded { why: NotRecorded::ReadOnlyAttach },
    )
}

fn lines(e: &Envelope<ExportReport>) -> Vec<String> {
    let mut o = vec![
        String::new(),
        format!("■ pal export --format {}", e.answer.format),
        String::new(),
        format!("  냈다      {} 바이트", e.answer.bytes),
    ];
    for c in &e.answer.exported {
        o.push(format!("            {:<14} {}건", c.label, c.count));
    }
    o.push(String::new());
    o.push(format!("  못 낸 라벨 {}개 — **0 건이 아닙니다**", e.answer.missing.len()));
    for m in &e.answer.missing {
        let 사유 = match &m.why {
            MissingReason::NotBuilt { by } => format!("아직 안 만들었습니다 — {by} 가 만듭니다"),
            MissingReason::NotStored { lives_in } => format!("2층에 안 삽니다 — {lives_in}"),
        };
        o.push(format!("            {:<14} {사유}", m.label));
    }
    o.push(String::new());
    o.push("■ 이 답의 근거".to_owned());
    o.push(format!("  Snapshot  {}", e.snapshot));
    o.push(format!("  2층       심볼 {} 색인됨", e.projection.symbols_indexed));
    o.push(format!(
        "  능력      {} · 미구축 {}",
        e.capabilities.built.join(" · "),
        e.capabilities.not_built.iter().map(|c: &CapabilityId| c.feature).collect::<Vec<_>>().join(" · ")
    ));
    o
}

fn print_screen(e: &Envelope<ExportReport>) {
    for l in lines(e) {
        println!("{l}");
    }
    for l in crate::evidence::lines(e) {
        println!("{l}");
    }
    println!();
}

/// 산출이 표준출력을 쓰고 있을 때. **근거를 버리지 않고 표준오류로 보낸다.**
fn eprint_screen(e: &Envelope<ExportReport>) {
    for l in lines(e).into_iter().chain(crate::evidence::lines(e)) {
        eprintln!("{l}");
    }
    eprintln!();
}
