//! `pal doctor` — **저장된 그래프가 자기 규칙을 지키는가** (F22-4 · #45).
//!
//! # 이 파일이 하는 일은 하나다 — **뷰를 채우는 것**
//!
//! 불변식 여덟은 [`pal_core::doctor`] 에 있고 그것은 저장 기술을 모른다. 여기서는
//! 2층·의도 저장소·대장을 읽어 [`GraphView`] 를 세우고, **이 빌드가 무엇을 담을 수
//! 없는지를 선언한다.**
//!
//! # 담을 수 없는 것을 세는 것이 이 명령의 절반이다
//!
//! 이 빌드의 저장소에는 심볼과 결박뿐이다. `Change`·`Actor`·`Defect` 는
//! `pal defect` 가 **계산만 하고 저장하지 않고**, `inferred` 노드도 후보 집합도 저장된
//! 잔여도 없다. 그것을 *"위반 0"* 으로 내면 이 도구가 자기가 고발한 문제를 저지른다 —
//! 그래서 [`ViewCoverage`] 가 라벨마다 **어느 기능이 그것을 만드는지**를 싣고,
//! `doctor` 는 그 자리를 `not_built` 로 낸다.
//!
//! # 심볼의 낡음 등급을 `live` 로 적는 근거
//!
//! [`NodeFreshness`] 는 §6.4 의 등급이고 그 정의는 파생물에 대한 것이다. 심볼은
//! 파생물이 아니라 **이 스냅샷의 코드 자체**다 — 감시 집합도 입력도 없으므로 낡을
//! 대상이 없다. 결박은 감시 집합을 갖지만 그것은 *자기* 감시 집합이고
//! (`BindingStatus::evaluate` 가 그것을 계산한다) §6.4 가 말하는 **입력**이 아니다.
//! 그래서 이 빌드에는 `stale-derived` 가 설 자리가 없고, 그 사실이 불변식 ⑧ 의
//! `not_built` 로 나온다.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use pal_core::{
    Anchor, BINDING_INDEX_KIND, Capable, CapabilityId, CapabilitySet, Coord, Coverage,
    DERIVED_KIND, Diagnosis, DoctorScope, Elision, Envelope, ExtractGrade, GraphSchema,
    GraphView, IdentityGrade, InvariantOutcome, LedgerRef, NodeInstance, NodeKey, Producer,
    Fold, FoldedPart, LogStatus, NotRecorded, Provenance, ProjectionFreshness, QueryName,
    RESIDUAL_KIND, ResolutionGrade, SCOPE_REDUCTION_KIND,
    Snapshot, SymbolNode,
};
use pal_intent::IntentStore;

use crate::attach;
use crate::ledger;
use crate::touch::intent_file;

/// 스키마 정본. **읽는 경로가 하나다** — `pal_core::GraphSchema::parse`(F22-1).
const SCHEMA: &str = include_str!("../../../schema/graph.toml");

/// 이 빌드가 답하는 것과 아직 못 만든 것.
fn capabilities() -> CapabilitySet {
    CapabilitySet::new(
        vec![
            QueryName::LedgerSnapshot.name().to_owned(),
            QueryName::SymbolResolve.name().to_owned(),
            "graph.doctor".to_owned(),
        ],
        vec![
            CapabilityId::new("F05", "graph-storage"),
            CapabilityId::new("F08", "unresolved-refs"),
            CapabilityId::new("F15", "judgment"),
            CapabilityId::new("F17", "synthesis"),
            CapabilityId::new("F20", "conformance"),
        ],
    )
}

/// 이 답에서 접힌 것 — **대장 하나.**
fn 접힌_대장(report: &ledger::LedgerReport) -> Fold {
    let mut fold = Fold::none();
    fold.push(FoldedPart::Ledger, report.ledger.total(), QueryName::LedgerSnapshot);
    fold
}

/// `pal doctor` 를 돌린다.
///
/// # Errors
/// 스키마가 읽히지 않거나, 저장소·캐시·2층·의도 저장소 중 하나에 닿지 못하면.
pub fn run(
    repo_path: &Path,
    rev: Option<&str>,
    cache_dir: Option<PathBuf>,
    index_path: Option<PathBuf>,
    intent_path: Option<PathBuf>,
    scope: DoctorScope,
    json: bool,
) -> Result<()> {
    let schema = GraphSchema::parse(SCHEMA).map_err(|e| anyhow::anyhow!("{e}"))?;

    let report = ledger::compute(repo_path, rev, cache_dir)?;
    let index = index_path.unwrap_or_else(|| repo_path.join(".palimpsest/index.redb"));
    // ⚠ **통째로 다시 만든다 — 그리고 그것이 엣지를 지운다**(F06 게이트 §6-가-2).
    // §12.7 이 든 네 위협 중 *부분 갱신*은 이 경로가 없어서 이 빌드에서 일어나지
    // 않는다. **`Stitching` 으로 옮기는 것이 옳지만 그러면 이 명령의 불변식 모집단이
    // 바뀌고 `[f22.4]` 의 판정이 움직인다** — F22 후속이다.
    let indexed = attach::attach(&index, &report, attach::How::SymbolsOnly)?.indexed;

    let intent = IntentStore::open_read_only(&intent_file(repo_path, intent_path))
        .context("의도 저장소를 열지 못했다")?;
    let bindings = intent.all().context("결박을 읽지 못했다")?;

    let view = build_view(&report.ledger.snapshot, &report.symbols, &bindings);
    let diagnosis = pal_core::doctor(&schema, &view, scope);

    let envelope = Envelope::new(
        diagnosis,
        report.ledger.snapshot.clone(),
        ProjectionFreshness {
            // **F01 이 이 자리를 값으로 바꿨다.** 워킹트리를 재고 이 답이 선 트리와
            // 대므로 이제 *"모른다"* 가 아니라 *"같다 / 다르다"* 를 적을 수 있다.
            matches_worktree: Capable::Present(
                report.worktree.matches(&report.ledger.snapshot_tree()),
            ),
            // 재구축 중인지 이 빌드는 모른다 — 관측 경로가 F05 다. DESIGN §12.7 격리 3번.
            rebuild: Capable::not_built(CapabilityId::new("F05", "rebuild-progress")),
            built_for_this_snapshot: true,
            symbols_indexed: indexed,
        },
        Coverage {
            unresolved: 0,
            out_of_scope_files: report.ledger.counts().values().sum::<usize>()
                - report.ledger.counts().get(&pal_core::Bucket::Parsed).copied().unwrap_or(0)
                - report.ledger.counts().get(&pal_core::Bucket::Partial).copied().unwrap_or(0),
            lowest_grade: ExtractGrade::L0,
            identity: IdentityGrade::Ordinal,
        },
        capabilities(),
        LedgerRef::of(&report.ledger),
        // 검사가 절단하는 것은 없다 — 표본은 **잔여**로 나가고 그것이 절단과 다른 것이다.
        Elision::none(),
        // **대장이 접혀 있다** — 절단이 아니라 부피를 옮긴 것이다(F06 §4.3).
        접힌_대장(&report),
        // ⚠ **이 표면은 질의 로그를 안 쓴다.** `pal touch` 와 같은 자리다.
        LogStatus::NotRecorded { why: NotRecorded::SurfaceDoesNotLog },
    );

    if json {
        println!("{}", serde_json::to_string_pretty(&envelope)?);
    } else {
        print_screen(&envelope);
    }
    Ok(())
}

/// 2층·의도 저장소에서 뷰를 세운다.
fn build_view(at: &Snapshot, symbols: &[SymbolNode], bindings: &[pal_core::Binding]) -> GraphView {
    let (repo, tree) = at.entries().next().expect("스냅샷은 비어 있을 수 없다");
    let coord = |s: pal_core::SymbolId| Coord {
        repo: repo.clone(),
        tree: *tree,
        extractor: pal_extract::version(),
        symbol: s,
    };
    let symbol_key = |s: pal_core::SymbolId| NodeKey::new("Symbol", s.to_hex());

    // ── 심볼 — **이 빌드에서 값이 실제로 서는 유일한 노드다** ────────────────
    //
    // 속성 여섯은 `schema/graph.toml` 의 `[node.Symbol]` 그대로이고 전부
    // `extractor` 다. 인스턴스가 그것을 실제로 싣고 있는지가 불변식 ②③ 이다.
    let mut nodes: Vec<NodeInstance> = symbols
        .iter()
        .map(|s| {
            NodeInstance::new(
                symbol_key(s.id),
                Provenance::Extracted,
                Anchor::At(coord(s.id)),
            )
            .with_attr("path", Producer::Extractor)
            // **`symbol_id` 의 성분이므로 스키마가 필수로 적는다**(F03-1). 빠뜨리면
            // `pal doctor` 의 불변식 2 가 심볼 전부를 위반으로 센다 — 코퍼스에서
            // 1,296 건이 그렇게 나왔다.
            .with_attr("container", Producer::Extractor)
            .with_attr("name", Producer::Extractor)
            .with_attr("kind", Producer::Extractor)
            .with_attr("body", Producer::Extractor)
            .with_attr("span", Producer::Extractor)
            .with_attr("identity", Producer::Extractor)
        })
        .collect();

    // ── 결박 — 의도 저장소가 소유한다(R-21). 2층에는 색인만 있다 ─────────────
    let mut edges = Vec::with_capacity(bindings.len());
    for b in bindings {
        nodes.push(
            NodeInstance::new(
                NodeKey::new("Binding", b.id.as_str()),
                Provenance::Asserted,
                Anchor::At(coord(b.target)),
            )
            // **F09 가 셋을 더했다.** 스키마가 `required` 로 적었으므로 여기서 안 실으면
            // 불변식 ②가 결박 전부를 위반으로 센다 — 그것이 F22-1 의 계약이고,
            // *"스키마를 코드에 맞추지 않고 코드를 스키마에 맞춘다"* 의 실제 하중이다.
            .with_attr("subject", Producer::Human)
            .with_attr("note", Producer::Human)
            .with_attr("bound_at", Producer::MachineRecord)
            .with_attr("bound_at_time", Producer::MachineRecord)
            .with_attr("radius", Producer::Human)
            .with_attr("watch", Producer::MachineRecord)
            // **F10 이 하나를 더했고 바로 위 주석이 그 자리를 예언했다** — 스키마가
            // `required` 로 적었는데 여기서 안 실어 `f22-4` 의 불변식 ②가 결박 하나를
            // 위반으로 셌다. 장치가 설계대로 일했고, 그 관측이 F10 게이트에 있다.
            .with_attr("promoted_by", Producer::MachineRecord),
        );
        edges.push(pal_core::EdgeInstance::one(
            "BOUND_TO",
            NodeKey::new("Binding", b.id.as_str()),
            symbol_key(b.target),
            ResolutionGrade::Exact,
            Provenance::Asserted,
            b.bound_at.clone(),
        ));
    }

    GraphView::new(at.clone(), coverage())
        .with_nodes(nodes)
        .with_edges(edges)
        // **결박 색인을 넣지 않는다.** 아래 `coverage` 의 주석을 보라.
        .with_binding_index(Vec::new(), BTreeSet::new())
}

/// 이 뷰가 담을 수 있는 것 — **선언이 빠지면 `doctor` 가 구멍으로 센다.**
fn coverage() -> pal_core::ViewCoverage {
    pal_core::ViewCoverage::new()
        // 값이 서는 둘.
        .holding("Symbol")
        .holding("Binding")
        .holding("BOUND_TO")
        // ── 계산만 하고 저장하지 않는 셋과 그 엣지 여섯 ──────────────────────
        //
        // `pal defect` 가 `Change`·`Actor`·`Defect` 를 git 에서 만들지만 **저장하지
        // 않는다.** 저장 자리를 만드는 것은 F05 이고, 없는 자리를 미리 만들지 않는다는
        // 판단은 S2 게이트에 있다.
        .absent("Change", CapabilityId::new("F05", "graph-storage"))
        .absent("Actor", CapabilityId::new("F05", "graph-storage"))
        .absent("Defect", CapabilityId::new("F05", "graph-storage"))
        .absent("AUTHORED_BY", CapabilityId::new("F05", "graph-storage"))
        .absent("TOUCHES", CapabilityId::new("F05", "graph-storage"))
        .absent("FOLLOWS", CapabilityId::new("F05", "graph-storage"))
        .absent("MANIFESTS_AT", CapabilityId::new("F05", "graph-storage"))
        .absent("INTRODUCED_BY", CapabilityId::new("F05", "graph-storage"))
        .absent("RESOLVED_BY", CapabilityId::new("F05", "graph-storage"))
        // ── F05 가 세웠는데 **이 뷰가 아직 안 싣는 둘** ──────────────────────
        //
        // 2층에 실제로 값이 있다(`FILE` · `EDGE_OUT`/`EDGE_IN`). 그런데 이 뷰는 심볼과
        // 결박만 싣는다 — **담을 수 없는 것이 아니라 이 빌드가 안 담는 것**이고,
        // `Capable` 이 적는 것이 정확히 그 구별이다.
        //
        // **왜 지금 안 싣는가**: 지금 참조 엣지는 전부 `scoped` 이고 파일 **안**에서만
        // 서며 종류가 하나다. 불변식 넷(공통 넷 · 출처 동질성 · 등급 규칙 · 근거 규칙)이
        // 구조상 어긋날 수 없고, 그것을 "실물에서 통과"로 세는 것은 *작아서 안 걸린 것*을
        // *성해서 안 걸린 것*으로 읽는 것이다 — 바로 아래 `BINDING_INDEX_KIND` 와 같은
        // 판단이다. **파일 간 해소(F07)가 후보 엣지와 근거를 만들 때 이 자리가 하중을
        // 진다.**
        .absent("File", CapabilityId::new("F07", "graph-view-stitched-nodes"))
        .absent("REFERENCES", CapabilityId::new("F07", "graph-view-stitched-nodes"))
        // ── 스키마가 이미 `not_built` 로 적은 둘 ─────────────────────────────
        //
        // 여기 적지 않아도 `doctor` 가 스키마에서 파생시키지만, **선언을 빠뜨리면
        // 구멍으로 세어진다**(`coverage_gaps`). 그 검사가 무엇을 세는지 흐리지 않으려고
        // 여기서도 적는다.
        .absent("Journey", CapabilityId::new("F19", "journey-authoring"))
        .absent("UnresolvedRef", CapabilityId::new("F08", "unresolved-refs"))
        // ── F10 이 더한 둘 — **담을 수 없는 것이 아니라 이 뷰가 안 담는 것** ──
        //
        // `NarrativeItem`(제안)은 **저장되지 않는다** — 결정론적 파생이라 다시 계산하고,
        // 2층에 두면 `[f05.2]` ④ 의 모집단이 는다(`[f10].queue_placement`).
        // `NarrativeRefusal`(거부)은 의도 저장소에 **저장되지만** 이 뷰가 안 싣는다 —
        // 2층에 색인이 없어 대조할 상대가 없고, 그것은 아래 `BINDING_INDEX_KIND` 와
        // **같은 판단**이다(같은 저장소에서 온 둘을 대조하면 구조상 안 어긋난다).
        //
        // ⚠ **선언을 빠뜨리면 구멍으로 세어진다** — F10 이 실제로 그것을 밟았고
        // `f22-4` 가 `덮개 구멍: ['NarrativeItem', 'NarrativeRefusal']` 로 잡았다.
        // **그리고 선언이 없으면 불변식 ④(`inferred` 의 근거)가 `NarrativeItem` 을
        // 모집단으로 삼아 「모집단 0 · 위반 0」으로 바뀐다** — `[f22.4]` 의 판정이
        // 움직이는 것이고, 선언하면 그 자리가 `not_built` 로 되돌아간다.
        //
        // 담기는 시점은 **의도층 노드가 그래프에 설 때**다(F09 게이트 §7 이 그것을
        // F12 로 넘겼다).
        .absent("NarrativeItem", CapabilityId::new("F12", "intent-layer-nodes"))
        .absent("NarrativeRefusal", CapabilityId::new("F12", "intent-layer-nodes"))
        // ── 스키마 라벨이 아닌 넷 ────────────────────────────────────────────
        .absent(RESIDUAL_KIND, CapabilityId::new("F15", "judgment"))
        .absent(SCOPE_REDUCTION_KIND, CapabilityId::new("F20", "conformance"))
        // **의도 저장소 안의 `BOUND_BY` 를 여기 넣지 않는다.** 불변식 ⑦ 이 재는 것은
        // *"2층의 색인이 가리키는 실체가 **다른** 저장소에 있는가"* 이고, 지금 색인과
        // 실체는 **같은 파일 안에** 있다. 같은 저장소에서 온 둘을 대조하면 구조상
        // 어긋날 수 없고, 그것을 "실물에서 통과"로 세는 것은 *작아서 안 걸린 것*을
        // *성해서 안 걸린 것*으로 읽는 것이다.
        //
        // ⚠ **옛 판은 이 자리를 `F05` 로 적었다** — *"F05 가 색인을 2층으로 옮길 때"*.
        // **F05 는 안 옮기기로 했다**([ADR-0009] · 재생 경로가 없는 파생은 안 세운다).
        // 기능이 닫혔는데 그 기능을 가리키는 능력 선언이 남으면 **아무도 그것을 안
        // 잡는다** — 그래서 하중을 실제로 지는 자리로 옮긴다. 결박의 역방향 조회가
        // 성능 문제로 관측되는 곳은 F09 다.
        //
        // [ADR-0009]: ../../../docs/adr/0009-a-derived-index-needs-its-rebuild-path-in-the-same-commit.md
        .absent(BINDING_INDEX_KIND, CapabilityId::new("F09", "binding-index-in-projection"))
        .absent(DERIVED_KIND, CapabilityId::new("F17", "synthesis"))
}

// ─────────────────────────────────────────────────────────────────────────────
// 화면
// ─────────────────────────────────────────────────────────────────────────────

/// 담지 못하는 자리를 한 줄로. **라벨과 만드는 기능이 함께 나온다.**
fn absences(list: &[pal_core::Absence]) -> String {
    list.iter()
        .map(|a| format!("{}({})", a.label, a.built_by))
        .collect::<Vec<_>>()
        .join(" · ")
}

fn print_screen(envelope: &Envelope<Diagnosis>) {
    let d = &envelope.answer;
    println!();
    match d.scope {
        DoctorScope::Full => println!("■ 불변식 여덟 — **전수**"),
        DoctorScope::Sample { max } => {
            println!("■ 불변식 여덟 — 표본 (불변식마다 최대 {max} 단위)");
        }
    }
    println!();
    for r in &d.invariants {
        match &r.outcome {
            InvariantOutcome::NotBuilt => {
                // **"위반 0" 이 아니다.** 모집단이 존재할 수 없다는 뜻이고,
                // 어느 기능이 그것을 만드는지가 함께 실린다.
                println!("  {}  (모집단이 없습니다)", r.number);
                println!("      {}", r.statement);
                println!("      담지 못하는 자리: {}", absences(&r.absent));
            }
            InvariantOutcome::Checked(o) => {
                let mark = if o.violations == 0 { "ok  " } else { "위반" };
                println!(
                    "  {}  {mark}  검사 {} · 표본 밖 {} · 위반 {}",
                    r.number, o.checked, o.skipped, o.violations
                );
                println!("      {}", r.statement);
                if !r.absent.is_empty() {
                    println!("      담지 못하는 자리: {}", absences(&r.absent));
                }
            }
        }
    }

    println!();
    println!("■ 위반 ({})", d.violations.len());
    if d.violations.is_empty() {
        // **"이상 없음"이라고 적지 않는다.** 위 표가 무엇을 봤는지 말하고 있다.
        println!("  없습니다. **`clean` 이 아닙니다** — 위 표의 모집단과 표본을 함께 읽으십시오.");
    } else {
        for v in &d.violations {
            println!("  [{}] {}", v.invariant.number(), v.subject);
            println!("      {}", v.detail);
            if let Anchor::At(c) = &v.anchor {
                println!("      {c}");
            }
        }
    }

    println!();
    println!("■ 잔여 ({}) — **검사하지 못한 것은 \"이상 없음\"이 아닙니다**", d.residuals.len());
    for r in &d.residuals {
        println!("  {} · 좌표 {}건", r.reason.label(), r.bound_to().len());
        println!("      {}", r.predicate);
        println!("      해소: {}", r.resolved_when);
    }

    if !d.coverage_gaps.is_empty() {
        println!();
        println!("■ ⚠ 이 검사의 구멍 ({})", d.coverage_gaps.len());
        println!("  스키마가 선언했는데 이 빌드가 담을 수 있는지 말하지 않은 자리입니다.");
        for g in &d.coverage_gaps {
            println!("  {g}");
        }
    }
    if !d.unanchored_cutoff.is_empty() {
        println!();
        println!("■ ⚠ 결박하지 못한 예산 초과 ({})", d.unanchored_cutoff.len());
        for k in &d.unanchored_cutoff {
            println!("  {k}");
        }
    }

    let e = envelope;
    println!();
    println!("■ 이 답의 근거");
    println!("  Snapshot  {}", e.snapshot);
    println!(
        "  대장      parsed {} · partial {} · unsupported {} · unrecognized {} / {} 파일",
        e.ledger.parsed,
        e.ledger.partial,
        e.ledger.unsupported,
        e.ledger.unrecognized,
        e.ledger.files_total
    );
    println!("  2층       심볼 {} 색인됨", e.projection.symbols_indexed);
    println!("  절단      {}", if e.elision.is_none() { "없음 (명시)" } else { "있음" });
    crate::evidence::print(e);
    println!(
        "  능력      {} · 미구축 {}",
        e.capabilities.built.join(" · "),
        e.capabilities.not_built.iter().map(|c| c.feature).collect::<Vec<_>>().join(" · ")
    );
    println!();
}
