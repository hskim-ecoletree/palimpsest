//! `pal touch <이름>` — 적시 제시. **F11 이 목적이라고 부른 자리다.**
//!
//! # 이 명령이 S2 에서 증명한 것은 **빈 답의 정직성**이었다
//!
//! 결박도 참조 해소도 판정도 없던 시절, 화면이 거의 비는데 그 빈 자리가 `[]` 나
//! `Finding 0` 으로 나오면 이 도구는 자기가 고발한 문제를 스스로 저지른다.
//! 그래서 자리마다 **어느 기능이 그것을 만드는지**가 적힌다. 그 규율은 그대로다.
//!
//! # F11 이 더한 것 — **걸린 것과 지켜보는 것을 가른다**
//!
//! `corpus/tasks/recurrence.toml` 이 재발 다섯을 읽고 이렇게 적었다:
//!
//! > 재발의 지배적 형태는 '몰랐다'가 아니라 **'경로 하나를 빠뜨렸다'** 이다.
//! > `touch(좌표)` 만으로는 부족하고 **"이 규칙을 지켜야 하는 다른 좌표가 어디인가"**
//! > 라는 역방향 질의가 필요하다.
//!
//! 그래서 화면에 구역이 둘이다 — *"이 좌표에 걸린 것"* 과 *"이 좌표를 지켜보는 것"*.
//! 뒤엣것의 실체는 의도 저장소의 `WATCH` 색인이고 F09 가 증분 갱신을 위해 세웠다.
//!
//! # 그리고 계산이 실행기로 옮겨 갔다
//!
//! 옛 판은 이 파일이 답을 **직접 조립**했다. 그러면 `binding.touch` 를 카탈로그에
//! 올릴 수 없고(코드 쪽 짝이 표면에만 있다), **질의 로그도 안 남는다** —
//! `LogStatus::NotRecorded{SurfaceDoesNotLog}` 가 그 자리였고 F17 이 그것을 미조회로
//! 과대 계상한다. 지금은 [`pal_query::touch`] 하나를 지나고 화면만 여기 있다.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use pal_core::{
    Binding, BoundItem, BoundTarget, Budget, CANDIDATE_LIMIT, Capable,
    CapabilitySet, Elision, Envelope, PROVISIONAL_PATH_PRODUCT_MAX, PROVISIONAL_TOUCH_BINDING_MAX,
    PROVISIONAL_TRAVERSAL_DEPTH, PROVISIONAL_VIEW_NODE_MAX, RebuildState, SymbolId, TargetPlace,
    TouchAnswer,
};
use pal_intent::IntentStore;
use pal_query::{BoundIndex, QueryCtx, QueryError};

use crate::attach;
use crate::ledger;

/// 이 빌드가 답하는 것과 아직 못 만든 것. **응답마다 실린다**(stack §5.3).
///
/// **질의 이름을 손으로 안 적는다** — [`pal_query::capabilities`] 가 `QueryName::ALL`
/// 에서 낸다. 손으로 적으면 카탈로그가 늘 때 이 목록만 뒤처지고, 그 어긋남은
/// `cargo xtask check` 의 카탈로그 정합이 **표면의 능력 목록까지는 안 본다.**
fn capabilities() -> CapabilitySet {
    let base = pal_query::capabilities();
    CapabilitySet::new(
        base.built
            .into_iter()
            // **F05 가 더한 것** — 파일 **안**의 참조 관계. 파일 경계를 넘는 것은 F07 이고,
            // 그 사실이 `coverage.unresolved` 에 수로 실린다.
            .chain(std::iter::once("symbol.references".to_owned()))
            .collect(),
        base.not_built,
    )
}

/// 의도 저장소를 [`BoundIndex`] 로 보이게 한다 — **색인 조회이지 전수 훑기가 아니다.**
///
/// `pal-query` 는 `pal-intent` 를 모른다. 그 경계가 이 타입이고,
/// [`pal_core::BindingStatus::evaluate`] 가 조회를 클로저로 받는 것과 같은 형태다.
pub struct IntentIndex<'a>(&'a IntentStore);

/// 의도 저장소를 색인으로 감싼다 — `pal query` 도 같은 것을 지난다.
#[must_use]
pub fn index_of(intent: &IntentStore) -> IntentIndex<'_> {
    IntentIndex(intent)
}

impl BoundIndex for IntentIndex<'_> {
    fn bound_to(&self, target: SymbolId) -> Result<Vec<Binding>, QueryError> {
        self.0.bound_to(target).map_err(|e| QueryError::BoundIndex(e.to_string()))
    }

    fn watching(&self, member: SymbolId) -> Result<Vec<Binding>, QueryError> {
        self.0
            .bindings_watching(&[member])
            .map_err(|e| QueryError::BoundIndex(e.to_string()))
    }
}

/// `pal touch` 하나의 인자.
///
/// **구조체인 것은 `bind`·`query` 와 같은 형태다.** 자리 여덟을 위치로 넘기면 같은 타입
/// 셋(`Option<PathBuf>`)이 나란히 서서 **바꿔 넣어도 컴파일된다** — 캐시와 2층과 의도가
/// 서로의 자리에 들어가는 것이 이 저장소에서 가장 비싼 실수다(방마다 파일이 따로여야
/// 대조가 안 꺼진다 · `[f04].self_judged` ③).
pub struct Args<'a> {
    pub repo: &'a Path,
    pub rev: Option<&'a str>,
    pub cache_dir: Option<PathBuf>,
    pub index: Option<PathBuf>,
    pub intent: Option<PathBuf>,
    pub name: &'a str,
    /// 한 구역이 싣는 결박의 상한. `None` 이면 자리표시.
    pub binding_max: Option<usize>,
    /// 걸린 시간을 **표준오류**로 낸다 — `elapsed_micros=<n>`.
    ///
    /// # 왜 산출이 아니라 표준오류인가
    ///
    /// 시간은 **답의 성질이 아니다**([`pal_core::LogStatus::Recorded`]). 산출에 섞으면
    /// 답의 바이트 동일성이 시간에 대해 깨지고, 이 저장소가 그 위에 세운 검사 둘이
    /// 무너진다. `pal ledger` 가 산출과 근거를 다른 줄기로 보내는 것과 같은 자리다.
    ///
    /// **손잡이가 있는 이유**: 소비자가 있다 — `[f11.pass]` ⑦ 이 이 값을 잰다.
    /// 없는 자리를 미리 만드는 것이 아니다.
    pub timing: bool,
    pub json: bool,
}

/// 좌표 하나를 만진다.
///
/// # Errors
/// 저장소·캐시·2층·의도 저장소 중 하나에 닿지 못하면.
pub fn run(a: Args) -> Result<()> {
    let Args { repo: repo_path, rev, cache_dir, index: index_path, intent: intent_path, name,
               binding_max, timing, json } = a;
    let 프로세스_시작 = std::time::Instant::now();
    // 대장을 먼저 만든다. **답의 근거가 먼저 서야 답이 나간다.**
    let report = ledger::compute(repo_path, rev, cache_dir)?;

    let index = index_path.unwrap_or_else(|| repo_path.join(".palimpsest/index.redb"));
    // **1패스 스티칭.** 무대에 배치로 쓰고 한 트랜잭션에서 교체한다 — 읽는 쪽은
    // 옛 세대 전체 아니면 새 세대 전체만 본다(F05 §4 · `[f05.2.pass]` ③).
    let attached = attach::attach(&index, &report, attach::How::Stitching)?;
    let built_for_this = attached.built_for_this_snapshot();
    let attach::Attached { projection, indexed, .. } = attached;

    // **의도 저장소는 파생층과 다른 파일이다** — R-21. 2층을 지워도 이쪽은 남는다.
    let intent = IntentStore::open_read_only(&intent_file(repo_path, intent_path))
        .context("의도 저장소를 열지 못했다")?;
    let bound = IntentIndex(&intent);

    let counts = report.ledger.counts();
    let out_of_scope = counts.values().sum::<usize>()
        - counts.get(&pal_core::Bucket::Parsed).copied().unwrap_or(0)
        - counts.get(&pal_core::Bucket::Partial).copied().unwrap_or(0);

    let ctx = QueryCtx {
        projection: &projection,
        snapshot: report.ledger.snapshot.clone(),
        ledger: pal_core::LedgerRef::of(&report.ledger),
        freshness: pal_query::freshness(
            // **F01 이 이 자리를 값으로 바꿨다.** 워킹트리를 재고 이 답이 선 트리와
            // 대므로 이제 *"모른다"* 가 아니라 *"같다 / 다르다"* 를 적을 수 있다.
            Capable::Present(report.worktree.matches(&report.ledger.snapshot_tree())),
            projection.rebuilding().unwrap_or(false),
            built_for_this,
            indexed,
        ),
        capabilities: capabilities(),
        budget: Budget::new(
            CANDIDATE_LIMIT,
            PROVISIONAL_PATH_PRODUCT_MAX,
            PROVISIONAL_TRAVERSAL_DEPTH,
            PROVISIONAL_VIEW_NODE_MAX,
        ),
        out_of_scope_files: out_of_scope,
        // **이 질의는 문서를 안 읽는다.** 비어 있는 것은 *"미결박이 0"* 이 아니라
        // *"안 물었다"* 이고, 그 구별이 `QueryCtx::narrative` 의 머리에 적혀 있다.
        narrative: Vec::new(),
        // ⚠ **전수를 안 싣는다.** `touch` 는 좌표 하나에 답하고, 색인은 `bound` 다.
        bindings: Vec::new(),
        // **이 질의는 계획을 안 읽는다.** *"이탈이 0"* 이 아니라 *"안 물었다"* 다.
        deviation: pal_query::DeviationInput::NotAsked,
        bound: &bound,
        binding_max: binding_max.unwrap_or(PROVISIONAL_TOUCH_BINDING_MAX),
        extractor: pal_extract::version(),
        detector: pal_core::DetectorReport {
            grammar: report.ledger.detector.grammar.clone(),
            extractor: report.ledger.detector.extractor.clone(),
            matches_head: report.ledger.detector.head_now == report.ledger.snapshot_tree().base(),
        },
        // **대장에서 뜬다** — 이름으로 세면 칸이 하나 늘 때 조용히 빠진다.
        partial_files: report
            .ledger
            .entries
            .iter()
            .filter(|e| e.state.bucket() == pal_core::Bucket::Partial)
            .map(|e| e.path.clone())
            .collect(),
    };

    let envelope = pal_query::touch(&ctx, name).context("질의가 실패했다")?;

    if json {
        println!("{}", serde_json::to_string_pretty(&envelope)?);
    } else {
        print_screen(&envelope);
    }

    // **두 시계를 둘 다 낸다** — 합격선은 질의 시간에만 걸리고(`[f11.pass]` ⑦),
    // 프로세스 시간은 기록이다. 하나만 내면 *"500ms 안에 답한다"* 가 사용자가 겪을 수
    // 없는 문장이 되거나(질의만), `touch` 가 아닌 것을 재게 된다(프로세스만).
    if timing {
        let 질의 = envelope.log.duration_micros().map_or_else(
            // **안 잰 것을 0 으로 접지 않는다** — 읽기 전용은 로그를 못 남기고,
            // 그러면 시간도 안 남는다.
            || "none".to_owned(),
            |v| v.to_string(),
        );
        let 프로세스 = 프로세스_시작.elapsed().as_micros();
        eprintln!("elapsed_micros={질의} process_micros={프로세스}");
    }
    Ok(())
}

/// 의도 저장소 위치. **기본값이 2층과 다른 파일이다**(stack §2.4).
pub fn intent_file(repo_path: &Path, given: Option<PathBuf>) -> PathBuf {
    given.unwrap_or_else(|| repo_path.join(".palimpsest/intent.redb"))
}

/// 답의 모양 한 줄 — `pal query binding.touch` 가 이것만 낸다.
///
/// **전문을 두 표면이 각자 그리지 않는다.** 같은 답이 표면마다 다른 모양으로 나가면
/// 그것이 곧 두 곳에 적힌 같은 것이다(계획 §7 의 넷째).
#[must_use]
pub fn 한_줄_found(r: &pal_core::TouchResult) -> String {
    format!(
        "{} — 걸린 것 {} · 지켜보는 것 {}",
        r.symbol.name,
        수(&r.bindings),
        수(&r.watching),
    )
}

/// 능력이 없으면 수가 아니라 그 사실을 낸다.
fn 수(v: &Capable<Vec<BoundItem>>) -> String {
    match v {
        Capable::Present(items) => items.len().to_string(),
        Capable::NotBuilt { capability } => format!("(미구축 {})", capability.feature),
    }
}

/// 옛 `how-it-works §2.3` 의 화면 (그 문서는 2026-08-18 에 지웠다 — `docs/plan/disposal-map.md`).
fn print_screen(envelope: &Envelope<TouchAnswer>) {
    println!();
    match &envelope.answer {
        TouchAnswer::Unknown { name, near } => {
            println!("  `{name}` 을 이 스냅샷에서 찾지 못했습니다.");
            println!();
            println!("  **없다는 뜻이 아닙니다** — 아래 근거가 무엇을 보았는지 말합니다.");
            print_near(near, &envelope.elision);
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
            print_bindings("이 좌표에 걸린 것", &r.bindings, &envelope.elision);
            // ★ **다른 구역이다.** *"내 코드에 걸린 결정"* 과 *"남의 코드에 걸렸는데
            // 나를 지켜보는 결정"* 은 고치러 갈 자리가 다르다.
            print_bindings("이 좌표를 지켜보는 것", &r.watching, &envelope.elision);
            print_facts(&r.facts);
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
    println!("  절단      {}", if e.elision.is_none() {
        "없음 (명시)".to_owned()
    } else {
        format!("{}건 — 상한을 넘어 잘렸습니다", e.elision.dropped())
    });
    crate::evidence::print(e);
    println!("  능력      {} · 미구축 {}",
             e.capabilities.built.join(" · "),
             e.capabilities.not_built.iter().map(|c| c.feature).collect::<Vec<_>>().join(" · "));
    println!();
}

/// 가까운 이름들 — **하나를 고르지 않는다**(P6).
///
/// 빈 목록과 목록이 있는 것은 **다른 답**이다. 앞은 *"가까운 것도 없다"* 이고 그것은
/// 이 스냅샷에 대한 사실이다.
pub fn print_near(near: &[pal_core::NearName], elision: &Elision) {
    println!();
    println!("■ 이것을 뜻했습니까 ({})", near.len());
    if near.is_empty() {
        println!("  가까운 이름도 없습니다.");
        return;
    }
    for n in near {
        println!("  [{}] {}", n.kind.name(), n.name);
    }
    let 자른 = elision.count_of(pal_core::ElisionReason::BindingMaxExceeded);
    if 자른 > 0 {
        let 상한 = PROVISIONAL_TOUCH_BINDING_MAX;
        println!("  … 그 밖 {자른}건 — **잘렸습니다. 상한이 {상한}입니다**");
    }
    println!();
    println!("  **하나를 고르지 않습니다** — 고르는 것은 사람이나 에이전트의 일입니다.");
}

/// 결박을 띄운다 — **제품의 형태가 보이는 자리다.**
fn print_bindings(title: &str, value: &Capable<Vec<BoundItem>>, elision: &Elision) {
    let Capable::Present(items) = value else {
        println!("■ {title}");
        println!("  (이 빌드에는 binding 능력이 없습니다)");
        return;
    };
    println!("■ {title} ({})", items.len());
    if items.is_empty() {
        // **여기의 빈 목록은 정직하다** — 능력이 있고 값이 없는 것이다.
        println!("  아직 없습니다.");
        return;
    }
    for item in items {
        let BoundItem::Note { binding, note, status, radius, watch, at, .. } = item;
        let mark = match &status.code {
            pal_core::CodeFreshness::Live => "live".to_owned(),
            pal_core::CodeFreshness::Stale { triggered_by } =>
                format!("STALE ← {} 개가 변했습니다", triggered_by.len()),
            pal_core::CodeFreshness::Orphaned { missing } =>
                format!("ORPHANED ← 좌표 {} 개가 사라졌습니다", missing.len()),
            // **`live` 와 같은 화면이 되면 안 된다** — *"유효하다"* 와 *"유효한지 알 수
            // 없다"* 가 같은 줄로 나오는 것이 R16 이 겨냥한 실패다.
            pal_core::CodeFreshness::Undeterminable { reason, at } =>
                format!("판정 불가 ← {} ({} 개 좌표)", reason.name(), at.len()),
        };
        // **반경을 함께 낸다** — *"이 결정은 `symbol` 반경에서 live"* 는 *"이 결정은
        // 유효하다"* 와 다른 문장이다(옛 F09 §3).
        println!("  [{}] {mark}  ·  {radius} 반경 · 감시 {watch}", binding.as_str());
        // ★ **어디에 걸렸는지가 다음 행동을 정한다.**
        if let BoundTarget::Elsewhere { symbol, place } = at {
            match place {
                TargetPlace::Known { path, container, name, line } => {
                    let 이름 = if container.is_empty() {
                        name.clone()
                    } else {
                        format!("{}.{name}", container.join("."))
                    };
                    println!("      ↳ 걸린 자리  {이름}  ·  {path}:{line}");
                }
                TargetPlace::Gone => println!(
                    "      ↳ 걸린 자리  {}  — **2층에 없습니다**", symbol.short()),
            }
        }
        for line in note.lines() {
            println!("      {line}");
        }
    }
    let 자른 = elision.count_of(pal_core::ElisionReason::BindingMaxExceeded);
    if 자른 > 0 {
        println!("  … 그 밖 {자른}건 — **잘렸습니다. 낡은 것은 잘리지 않습니다**");
    }
}

/// 이 심볼이 하는 것 — **수를 낸다. `(있음)` 은 아무것도 안 말한다.**
fn print_facts(value: &Capable<pal_core::SymbolFacts>) {
    println!("■ 이 심볼이 하는 것");
    match value {
        Capable::NotBuilt { capability } => println!(
            "  (이 빌드에는 {} 능력이 없습니다 — {} 미구축)", capability.what, capability.feature),
        Capable::Present(f) => {
            println!("  호출자 {} · 피호출자 {}", f.callers, f.callees);
            // ⚠ **파일 경계를 넘는 것은 여기 없다.** 그 수는 봉투의 `coverage.unresolved`
            // 가 지고, 그 사실을 화면에서 지우면 0 이 *"아무도 안 부른다"* 로 읽힌다.
            println!("  **파일 안의 관계만입니다** — 파일 경계를 넘는 것은 F07 미구축입니다");
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
