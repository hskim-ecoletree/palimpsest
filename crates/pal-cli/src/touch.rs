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
/// 에서 산출한다. 손으로 적으면 카탈로그가 늘 때 이 목록만 뒤처지고, 그 어긋남은
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
/// 대조가 안 멎는다 · `[f04].self_judged` ③).
pub struct Args<'a> {
    pub repo: &'a Path,
    pub rev: Option<&'a str>,
    pub cache_dir: Option<PathBuf>,
    pub index: Option<PathBuf>,
    pub intent: Option<PathBuf>,
    pub name: &'a str,
    /// 한 구역이 싣는 결박의 상한. `None` 이면 자리표시.
    pub binding_max: Option<usize>,
    /// 걸린 시간을 **표준오류**로 출력한다 — `elapsed_micros=<n>`.
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
    // 옛 세대 전체 아니면 새 세대 전체만 본다(옛 F05 §4 · `[f05.2.pass]` ③).
    let attached = attach::attach(&index, &report, attach::How::Stitching)?;
    let built_for_this = attached.built_for_this_snapshot();
    let attach::Attached { projection, indexed, .. } = attached;

    // **의도 저장소는 파생층과 다른 파일이다** — R-21. 2층을 지워도 이쪽은 남는다.
    // ⚠ **그러나 없을 수도 있다** — `intent.redb` 는 커밋된 `intent/bindings.jsonl` 에서
    // 세우는 파생물이고 `.gitignore` 가 지운다. 그 부재가 답에 실려야 빈 목록이
    // 「0 건」으로 안 읽힌다.
    let 의도_자리 = intent_file(repo_path, intent_path);
    let intent_store = pal_core::IntentStorePresence::of(
        의도_자리.exists(),
        의도_자리.display().to_string(),
        intent_canonical(repo_path),
    );
    let intent =
        IntentStore::open_read_only(&의도_자리).context("의도 저장소를 열지 못했다")?;
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
        intent_store,
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
        print_screen(&envelope, attached.cross.as_ref());
    }

    // **두 시계를 둘 다 산출한다** — 합격선은 질의 시간에만 걸리고(`[f11.pass]` ⑦),
    // 프로세스 시간은 기록이다. 하나만 내면 *"500ms 안에 답한다"* 가 사용자가 겪을 수
    // 없는 문장이 되거나(질의만), `touch` 가 아닌 것을 재게 된다(프로세스만).
    if timing {
        let 질의 = envelope.log.duration_micros().map_or_else(
            // **안 잰 것을 0 으로 뭉개지 않는다** — 읽기 전용은 로그를 못 남기고,
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

/// 결박의 **커밋된 정본** — 파생 저장소(`intent.redb`)를 여기서 세운다.
///
/// ★ **이 자리를 재는 것이 「0 건」과 「못 읽었다」를 가르는 유일한 자다.** 파생 파일이
/// 없는 것만으로는 못 가른다 — 아무도 안 건 저장소도 그렇고, 세우기 전의 저장소도
/// 그렇다. 정본이 있으면 뒤의 것이다.
#[must_use]
pub fn intent_canonical(repo_path: &Path) -> pal_core::CanonicalSource {
    let p = repo_path.join(".palimpsest/intent/bindings.jsonl");
    if p.exists() {
        pal_core::CanonicalSource::Present(p.display().to_string())
    } else {
        pal_core::CanonicalSource::Absent
    }
}

/// 답의 모양 한 줄 — `pal query binding.touch` 가 이것만 출력한다.
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

/// 능력이 없으면 수가 아니라 그 사실을 산출한다.
fn 수(v: &Capable<Vec<BoundItem>>) -> String {
    match v {
        Capable::Present(items) => items.len().to_string(),
        Capable::NotBuilt { capability } => format!("(미구축 {})", capability.feature),
    }
}

/// 옛 `how-it-works §2.3` 의 화면 (그 문서는 2026-08-18 에 지웠다 — `docs/plan/disposal-map.md`).
fn print_screen(envelope: &Envelope<TouchAnswer>, cross: Option<&pal_core::CrossFileReport>) {
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
            print_facts(&r.facts, cross);
            print_unresolved(&r.unresolved);
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
    // **모른다는 것도 화면에 성립한다.** 산출에만 있고 화면에 없으면 사람은 그 공백을 못 본다.
    match &e.projection.rebuild {
        Capable::Present(RebuildState::Rebuilding) =>
            println!("  재구축    진행 중 — 이 답은 열린 스냅샷 위에 섰습니다"),
        Capable::Present(RebuildState::Settled) => println!("  재구축    아님"),
        Capable::NotBuilt { capability } => println!(
            "  재구축    (이 빌드는 재구축 중인지 모릅니다 — {} 미구축)", capability.feature),
    }
    println!("  생략      {}", if e.elision.is_none() {
        "없음 (명시)".to_owned()
    } else {
        format!("{}건 — 상한을 넘어 생략했습니다", e.elision.dropped())
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
        println!("  [{}] {}", crate::label::가까움(n.kind).병기(), n.name);
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
        // **병기는 `label` 이 진다** — `pal-core` 의 `name()` 에 얹으면 그것이 그대로
        // 와이어로 나간다(`label` 모듈 머리 · ADR-0033).
        let 병기 = crate::label::신선도(&status.code).병기();
        let mark = match &status.code {
            pal_core::CodeFreshness::Fresh => 병기.to_owned(),
            pal_core::CodeFreshness::Stale { triggered_by } =>
                format!("{병기} ← {} 개가 변했습니다", triggered_by.len()),
            pal_core::CodeFreshness::Orphaned { missing } =>
                format!("{병기} ← 좌표 {} 개가 사라졌습니다", missing.len()),
            // **`fresh` 와 같은 화면이 되면 안 된다** — *"유효하다"* 와 *"유효한지 알 수
            // 없다"* 가 같은 줄로 나오는 것이 R16 이 겨냥한 실패다.
            pal_core::CodeFreshness::Undeterminable { reason, at } =>
                format!("{병기} ← {} ({} 개 좌표)", crate::label::판정_불가_사유(*reason).병기(), at.len()),
        };
        // **반경을 함께 싣는다** — *"이 결정은 `symbol` 반경에서 fresh"* 는 *"이 결정은
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

/// 이 심볼이 하는 것 — **수를 산출한다. `(있음)` 은 아무것도 안 말한다.**
///
/// # 능력 축이 **셋**이다 (2026-09-09 · `E4`)
///
/// 앞 판은 `x.foo()` 와 `S::foo()` 를 한 줄에 묶어 *"멤버·경로 해소가 F07 미구축"* 이라
/// 적었다. **그 묶음이 이제 거짓이다** — `S::foo()` 에는 값이 있고 `x.foo()` 에는 없다.
/// 묶어 두면 선 것과 안 선 것이 같은 문장을 지고, 어느 쪽이 참인지 화면이 말하지 못한다.
///
/// | 축 | 오늘 | 까닭 |
/// |---|---|---|
/// | `cross-file-import` | **값이 있다** | 임포트한 이름이 다른 파일의 심볼에 붙는다 |
/// | `path-resolution` | **값이 있다** | `S::foo()` 의 꼬리를 대상 파일에서만 찾는다 |
/// | `member-resolution` | **범위 밖** | `x.foo()` 는 타입 추론이 필요하다 |
///
/// ★ **마지막 줄이 「능력 부재」가 아니라 「범위 밖」인 것이 요점이다.** 부재는 언젠가
/// 만들 것이고 범위 밖은 이 회차가 안 하기로 정한 것이다 — 화면이 그 둘을 안 가르면
/// 사람이 *"곧 될 것"* 으로 읽는다.
fn print_facts(
    value: &Capable<pal_core::SymbolFacts>,
    cross: Option<&pal_core::CrossFileReport>,
) {
    println!("■ 이 심볼이 하는 것");
    match value {
        Capable::NotBuilt { capability } => println!(
            "  (이 빌드에는 {} 능력이 없습니다 — {} 미구축)", capability.what, capability.feature),
        Capable::Present(f) => {
            println!("  호출자 {} · 피호출자 {}", f.callers, f.callees);
            // ★ **라벨이 「호출자」인데 담는 것은 호출만이 아니다.**
            //   실측(2026-09-08 · Rust 엣지 4,258): 타입 참조 1,198 · 호출·매크로
            //   3,843 · 그 밖 1,598. **엣지의 상당수가 호출이 아니다.**
            //   ⚠ 갈래 셋의 합 6,639 는 엣지 수가 아니라 **엣지가 된 참조** 수다 —
            //   여럿이 한 심볼 쌍으로 모인다.
            //   스키마가 그 사실을 못 박았다 — *"`CALLS` 가 아니다 … `CALLS` 로 적으면
            //   타입 참조가 호출로 둔갑한다"*(`schema/graph.toml` `[edge.REFERENCES]`).
            //   화면에서 지우면 사람이 「부르는 자리」로만 읽고 엉뚱한 곳을 고친다.
            println!(
                "  ※ 「호출자·피호출자」는 **참조 엣지**입니다 — 호출뿐 아니라 \
타입 참조·구조체 리터럴·경로 머리도 셉니다"
            );
            // ★ **반대 방향도 적는다.** 효과 관측에서 `resolve_shadowing` 이
            //   `호출자 0` 을 돌려줬는데 그 함수는 바로 옆에서 `self.resolve_shadowing(…)`
            //   으로 불린다 — 멤버 호출은 `field_identifier` 라 참조가 아니고(멤버 해소는
            //   L2c · F07), 경로 호출(`S::new()`)의 꼬리도 마찬가지다. 이 줄이 없으면
            //   그 0 이 **「아무도 안 부른다」로 읽힌다.**
            println!(
                "  ※ **`x.foo()` 는 아직 안 셉니다** — 멤버 해소(`member-resolution`)는 \
타입 추론이 필요해 **이 회차의 범위 밖**입니다. 능력 부재가 아니라 안 하기로 정한 자리입니다"
            );
            print_cross(cross);
        }
    }
}

/// 파일 경계를 넘는 해소의 회계 — **`cross-file-import`(ⓐ)와 `path-resolution`(ⓑ)**.
///
/// # 수를 안 찍을 때는 **「안 잼」이라 적는다**
///
/// [`None`] 은 이 투영이 그 패스를 안 지났다는 뜻이지 0 건이라는 뜻이 아니다. 0 으로
/// 적으면 *"파일 간 엣지가 없다"* 가 사실로 나가고, 그것이 이 회차 착수 관측의 형태다.
///
/// ⚠ **ⓑ 의 분모는 [`pal_core::RefCounts`] 에 못 넣는다** — 그 타입은
/// `total() == refs.len()` 을 지고 꼬리는 참조 자리가 아니다. 그래서 이 회계가 그
/// 자리를 대신 지고, 화면은 여기서만 ⓑ 를 읽는다.
fn print_cross(cross: Option<&pal_core::CrossFileReport>) {
    let Some(c) = cross else {
        println!(
            "  ※ **파일 간 해소를 이 답에서는 안 쟀습니다** — 0 건이라는 뜻이 아닙니다"
        );
        return;
    };
    println!(
        "  ※ 파일 간 해소 — ⓐ `cross-file-import` {}/{} · ⓑ `path-resolution` {}/{} (선 것/짝)",
        c.a_edges, c.a_pending, c.b_edges, c.b_pending
    );
    for (축, m) in [("ⓐ", &c.unresolved), ("ⓑ", &c.b_unresolved)] {
        if m.is_empty() {
            continue;
        }
        let 까닭: Vec<String> = m.iter().map(|(k, v)| format!("{k} {v}")).collect();
        println!("  ※ {축} 가 못 선 까닭 — {}", 까닭.join(" · "));
    }
    print_transfer();
}

/// 이관표 — **못 선 몫이 어느 문으로 가나.**
///
/// # 왜 화면이 이것을 져야 하나
///
/// 못 선 까닭의 셈만 찍으면 화면은 *"못 풀었다"* 만 말하고 **그것이 결함인지 경계인지**를
/// 못 말한다. 잠긴 의도가 그 갈래를 셋 다에 적으라고 요구한다 — 게이트 · 이 화면
/// (`E4` 의 `path-resolution` 축) · 잔여. *"셋 중 하나라도 빠지면 그것이 조용한 축소다."*
///
/// # 수를 안 적는 것이 요점이다
///
/// 문만 적고 셈은 위 줄이 진다. 앞 판은 이 갈래를 [`pal_core::Unresolved`] 의 주석에
/// **비율로** 적어 뒀고(*"340 중 298"*), 값이 움직이자 그 문장이 거짓이 됐다. 그래서
/// 갈래를 **회계 열쇠**로 올리고 화면은 열쇠와 문의 대응만 찍는다.
fn print_transfer() {
    println!("  ※ 못 선 몫이 가는 문 — 이 회차가 안 세우기로 정한 자리와 못 세우는 자리를 가릅니다");
    println!("      ⚠ 갈래 하나는 **아직 안 갈렸습니다** — 그 자리는 문이 아니라 「미측정」을 적습니다");
    for (열쇠, 문) in [
        ("outside_repo", "**경계** — 저장소 밖(`std::*` 등)이라 원리상 못 섭니다"),
        ("no_symbol_at_crate_root", "`A5`·`A5-a` — 재수출 경유는 잠근 축1 의 **밖**입니다"),
        ("no_symbol", "**갈래가 아직 안 갈렸습니다** — `#133`(L2)로 갈 몫과 이 회차의 구현 여지가 섞여 있고 그 크기는 **미측정**입니다"),
        ("no_target_file", "**이 회차의 구현** — 모듈 경로를 파일로 못 폈습니다"),
        ("ambiguous", "**후보 생성의 모호** — 모듈 경로가 여러 파일로 읽힙니다. 하나를 고르면 조용한 오답이라 **안 고르는 것이 설계입니다**"),
    ] {
        println!("      `{열쇠}` → {문}");
    }
}

/// 미구축 자리를 **빈 목록이 아니라 문장으로** 산출한다.
/// **내가 모르는 것** — `F08` 의 값 (`B3`).
///
/// # 왜 `slot` 이 아닌가
///
/// [`slot`] 은 `Capable::Present` 를 *"(있음)"* 으로만 찍는다. 그것은 자리가 비어 있던
/// 동안에는 정확한 화면이었다 — 값이 없으니 말할 것도 없었다. 값이 서면 그 화면이
/// **거짓 신호**가 된다: *"있음"* 은 몇 건이 무엇 때문에 막혔는지를 안 말하면서
/// 말했다는 인상만 준다.
///
/// ⚠ **`NotBuilt` 는 이제 「이 빌드가 못 만든다」가 아니다** — 이 빌드는 만든다.
/// 남은 뜻은 하나뿐이다: **이 투영이 파일 간 해소 패스를 안 지났다.** 그 문장을
/// 화면이 그대로 적는다. 안 그러면 능력 부재 선언과 실물이 어긋난 채 남고, 그것이
/// `E5-a` 가 반증으로 잡는 자리다.
fn print_unresolved(value: &Capable<Vec<pal_core::UnresolvedRef>>) {
    println!("■ 내가 모르는 것");
    let items = match value {
        Capable::NotBuilt { .. } => {
            println!("  (이 투영은 파일 간 해소 패스를 안 지났습니다 — 「0 건」이 아닙니다)");
            return;
        }
        Capable::Present(v) => v,
    };
    if items.is_empty() {
        println!("  없음 — 이 좌표에서 못 푼 참조가 0 건입니다");
        return;
    }
    println!("  {} 건", items.len());
    for u in items {
        // 마지막 걸음이 끊긴 자리다. **`reason` 과 같은 사실의 두 표현이고**,
        // 걸음을 안 찍으면 *"못 풀었다"* 만 남아 고칠 자리를 안 준다.
        let 걸음: Vec<String> = u
            .attempts
            .iter()
            .map(|a| format!("{}({}→{})", a.step.as_str(), a.tried.len(), a.found))
            .collect();
        println!("  · {} — {} · 지난 걸음 {}", u.name, u.reason.as_str(), 걸음.join(" "));
    }
}

fn slot<T>(title: &str, value: &Capable<T>) {
    println!("■ {title}");
    match value {
        Capable::NotBuilt { capability } => {
            println!("  (이 빌드에는 {} 능력이 없습니다 — {} 미구축)", capability.what, capability.feature);
        }
        Capable::Present(_) => println!("  (있음)"),
    }
}
