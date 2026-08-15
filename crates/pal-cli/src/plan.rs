//! `pal plan` · `pal deviation` — **계획에 좌표를 붙이고 실제와 댄다**(F12).
//!
//! # 이 명령이 2 층을 안 연다
//!
//! [`crate::ledger::compute`] 가 **심볼과 파일 목록을 이미 들고 있다**(1 층 캐시가
//! 대부분 적중한다). 이탈 대조가 필요로 하는 것은 두 스냅샷의
//! `(symbol_id, body_digest)` 와 파일 목록뿐이고, 그것을 위해 2 층을 두 번 스티칭하면
//! **같은 색인 파일을 두 스냅샷이 번갈아 덮어쓴다** — 그러면 뒤엣것이 앞엣것을 지운다.
//!
//! # `--base <ref>` 가 없다 — **[F23] 의 것이다**
//!
//! [F23 §7] 의 완료 체크리스트가 `--base <ref>` 를 `briefing · conformance ·
//! **deviation**` 셋에 대해 한 줄로 적었고 소유자가 거기다. [F12 §4] 는 그것 없이
//! 서는 길을 이미 적었다 — *"계획 승인 시점의 Snapshot 을 계획에 기록"*.
//! **그래서 기준선은 계획 문서의 프론트매터가 진다**([`PlanBaseline`]).
//!
//! [F23 §7]: ../../../docs/plan/features/F23-git-integration.md

use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use pal_core::{
    Deviation, DeviationRate, ItemResolution, PROVISIONAL_PLAN_PATTERN_FILE_MAX, Plan,
    PlanBaseline, PlanBindingState, RepoPath, SnapshotView, Snapshot, SymbolNode, UnresolvedWhy,
    VerificationStep, deviate, resolve_pattern, resolve_plan, symbol_delta,
};

use crate::ledger;

pub struct Args<'a> {
    pub repo: &'a Path,
    /// 머리 스냅샷. 기본값은 HEAD.
    pub rev: Option<&'a str>,
    pub cache_dir: Option<PathBuf>,
    /// 계획 문서.
    pub plan: &'a Path,
    pub json: bool,
}

/// 한 스냅샷에서 대조에 필요한 것 — **심볼과 파일 목록.**
struct 스냅샷 {
    snapshot: Snapshot,
    symbols: Vec<SymbolNode>,
    files: Vec<RepoPath>,
}

impl 스냅샷 {
    fn view(&self) -> SnapshotView<'_> {
        SnapshotView { symbols: &self.symbols, files: &self.files }
    }
}

fn 스냅샷을_잰다(repo: &Path, rev: Option<&str>, cache_dir: Option<PathBuf>) -> Result<스냅샷> {
    let r = ledger::compute(repo, rev, cache_dir)?;
    Ok(스냅샷 {
        snapshot: r.ledger.snapshot.clone(),
        files: r.ledger.entries.iter().map(|e| e.path.clone()).collect(),
        symbols: r.symbols,
    })
}

fn 계획을_읽는다(a: &Args) -> Result<Plan> {
    let text = std::fs::read_to_string(a.plan)
        .with_context(|| format!("계획 문서를 읽지 못했다: {}", a.plan.display()))?;
    // **경로는 정체성이 아니라 추적용이다** — 저장소 안이면 상대 경로로 적는다.
    let rel = a.plan.strip_prefix(a.repo).unwrap_or(a.plan);
    pal_extract::ingest_plan(&RepoPath::new(rel.to_string_lossy().into_owned()), &text)
        .map_err(|e| anyhow::anyhow!("{e}"))
}

/// `pal plan <문서>` — **계획을 읽고 지금 스냅샷에 대 본다.**
///
/// 이탈을 계산하지 않는다. 그것은 [`deviation`] 이고 기준선을 요구한다.
///
/// # Errors
/// 문서를 못 읽거나, 항목이 하나도 없거나, 대장을 못 만들면.
pub fn plan(a: &Args) -> Result<()> {
    let plan = 계획을_읽는다(a)?;
    let now = 스냅샷을_잰다(a.repo, a.rev, a.cache_dir.clone())?;
    let view = now.view();

    let 상태: Vec<Vec<PlanBindingState>> = plan
        .items()
        .iter()
        .map(|i| {
            i.expected
                .iter()
                .map(|p| resolve_pattern(p, &view, PROVISIONAL_PLAN_PATTERN_FILE_MAX))
                .collect()
        })
        .collect();

    if a.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "plan": plan,
                "at": now.snapshot.to_string(),
                "states": 상태,
            }))?
        );
        return Ok(());
    }

    println!();
    println!("계획  {}", plan.headline);
    println!("  문서    {}", plan.source);
    match &plan.baseline {
        PlanBaseline::Declared { rev } => println!("  기준선  {rev}"),
        // **안 적힌 것이 값이다** — 이탈을 계산할 수 없다는 사실이 여기서 보여야 한다.
        PlanBaseline::NotDeclared => println!(
            "  기준선  **선언되지 않았습니다** — 프론트매터에 `baseline: <rev>` 가 \
             있어야 `pal deviation` 이 섭니다"
        ),
    }
    println!("  지금    {}", now.snapshot);
    let 검증_있음 = plan.items().iter().filter(|i| i.verification.is_stated()).count();
    println!("  항목    {} · 검증이 적힌 것 {}", plan.items().len(), 검증_있음);
    println!();

    println!("■ 항목");
    for (item, states) in plan.items().iter().zip(&상태) {
        println!("  [{}] {}", &item.id.as_str()[..8], 한_줄(&item.statement));
        if item.expected.is_empty() {
            // **빈 목록이 정직한 답이다** — 계획 문장이 좌표를 하나도 안 적었다.
            println!("      예상 좌표  없습니다 — 이 항목은 **판정 불가**로 셉니다");
        }
        for (p, s) in item.expected.iter().zip(states) {
            println!("      예상 좌표  {}  →  {}", p.display(), 상태_한_줄(s));
        }
        match &item.verification {
            VerificationStep::Stated { how } => println!("      검증        {how}"),
            VerificationStep::NotStated => println!("      검증        (안 적혔습니다)"),
        }
    }
    println!();
    요약(&plan, &상태);
    Ok(())
}

fn 요약(plan: &Plan, 상태: &[Vec<PlanBindingState>]) {
    let mut 해소 = 0usize;
    let mut pending = 0usize;
    let mut 사유: std::collections::BTreeMap<&'static str, usize> =
        UnresolvedWhy::ALL.into_iter().map(|w| (w.name(), 0)).collect();
    for states in 상태 {
        if states.iter().any(|s| matches!(s, PlanBindingState::Bound { .. })) {
            해소 += 1;
        }
        for s in states {
            match s {
                PlanBindingState::Pending { .. } => pending += 1,
                PlanBindingState::Unresolved { why, .. } => *사유.entry(why.name()).or_insert(0) += 1,
                PlanBindingState::Bound { .. } => {}
            }
        }
    }
    println!("■ 요약");
    println!("  좌표가 해소된 항목  {} / {}", 해소, plan.items().len());
    println!("  아직 없는 좌표(pending)  {pending}");
    println!(
        "  못 좁힌 패턴  {}",
        사유.iter().map(|(k, v)| format!("{k} {v}")).collect::<Vec<_>>().join(" · ")
    );
    println!();
}

/// 계획 하나와 두 스냅샷에서 계산된 것 — `pal query` 도 이것을 지난다.
pub struct Computed {
    pub plan: Plan,
    pub deviation: Deviation,
    pub base: Snapshot,
    pub head: Snapshot,
    pub resolutions: Vec<ItemResolution>,
}

/// 이탈을 계산한다 — **`pal deviation` 과 `plan.deviation` 이 같은 것을 지난다.**
///
/// # Errors
/// 기준선이 선언되지 않았거나, 두 스냅샷 중 하나를 못 만들면.
pub fn compute(a: &Args) -> Result<Computed> {
    let plan = 계획을_읽는다(a)?;
    let PlanBaseline::Declared { rev } = &plan.baseline else {
        // ⚠ **조용히 HEAD 를 기준선으로 삼지 않는다.** 그러면 이탈률이 언제나 0 이
        // 되고, 그 0 이 *"계획대로 했다"* 로 읽힌다.
        bail!(
            "`{}` 에 기준선이 없습니다 — 프론트매터에 `baseline: <rev>` 를 적으십시오. \
             **어디부터가 이 계획의 변경인가**를 계획이 정해야 하고(F12 §4), \
             `--base` 는 이 명령의 손잡이가 아닙니다(F23)",
            plan.source
        );
    };
    let base = 스냅샷을_잰다(a.repo, Some(rev), a.cache_dir.clone())?;
    let head = 스냅샷을_잰다(a.repo, a.rev, a.cache_dir.clone())?;

    let resolutions =
        resolve_plan(&plan, &base.view(), &head.view(), PROVISIONAL_PLAN_PATTERN_FILE_MAX);
    let delta = symbol_delta(&base.symbols, &head.symbols);
    let deviation = deviate(&plan, &resolutions, &delta);
    Ok(Computed { plan, deviation, base: base.snapshot, head: head.snapshot, resolutions })
}

/// `pal deviation <문서>`.
///
/// # Errors
/// [`compute`] 와 같다.
pub fn deviation(a: &Args) -> Result<()> {
    let c = compute(a)?;
    if a.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "plan": c.plan,
                "base": c.base.to_string(),
                "head": c.head.to_string(),
                "deviation": c.deviation,
                "resolutions": c.resolutions,
            }))?
        );
        return Ok(());
    }
    print_screen(&c);
    Ok(())
}

/// 화면 — **넷이 각각 줄을 갖는다.**
///
/// `unmeasurable` 을 다른 셋과 합쳐 적으면 [F12 §2] 가 *"이탈률이 거짓말이 된다"* 고
/// 적은 상태 그 자체다. 산출에서 갈라 놓고 화면에서 뭉치면 **사람이 그 구별을 못 본다.**
pub fn print_screen(c: &Computed) {
    let d = &c.deviation;
    println!();
    println!("이탈  {}", c.plan.headline);
    println!("  기준선  {}", c.base);
    println!("  머리    {}", c.head);
    println!(
        "  실제 변경  심볼 {} (변경 {} · 추가 {} · 삭제 {})",
        d.delta.len(),
        d.delta.changed.len(),
        d.delta.added.len(),
        d.delta.removed.len()
    );
    println!();

    println!("■ 계획대로 ({})", d.as_planned.len());
    for p in &d.as_planned {
        println!("  [{}] {}  ·  {}", &p.item.as_str()[..8], p.coord.short(), p.by.name());
    }
    if d.as_planned.is_empty() {
        println!("  없습니다.");
    }
    println!();

    // ⚠ **나쁜 것이 아니다**([F12 §4]) — 판정이 아니라 관측이다.
    println!("■ 계획에 없던 변경 ({})", d.unplanned.len());
    for s in &d.unplanned {
        println!("  {}", s.short());
    }
    if d.unplanned.is_empty() {
        println!("  없습니다.");
    }
    println!("  ⚠ **나쁜 것이 아닙니다** — 분류만 하고 평가하지 않습니다(F12 §4).");
    println!();

    println!("■ 계획했으나 없는 변경 ({})", d.unimplemented.len());
    for i in &d.unimplemented {
        println!("  [{}]", &i.as_str()[..8]);
    }
    if d.unimplemented.is_empty() {
        println!("  없습니다.");
    }
    println!();

    // ★ 넷째. **위 셋 어디에도 안 섞인다.**
    println!("■ 못 잰 항목 ({})", d.unmeasurable.len());
    println!(
        "  사유  {}",
        d.unmeasurable_by_reason()
            .iter()
            .map(|(k, v)| format!("{k} {v}"))
            .collect::<Vec<_>>()
            .join(" · ")
    );
    println!("  ★ **위 셋에 합산하지 않습니다** — 섞으면 이탈률이 거짓말이 됩니다(F12 §2·§5).");
    println!();

    println!("■ 값");
    match d.rate() {
        DeviationRate::Rate { value, changed } => {
            println!("  이탈률       {value:.3}   = |A∖D| / |A| · A = {changed}");
        }
        // *"하나도 안 벗어났다"* 와 *"잴 것이 없었다"* 는 다른 답이다.
        DeviationRate::Undefined => println!(
            "  이탈률       **정의되지 않습니다** — 실제 변경이 0 입니다. \
             「안 벗어났다」가 아닙니다"
        ),
    }
    let r = d.resolution();
    println!(
        "  좌표 해소율  {} / {}   (분모는 **항목 전부**입니다 — 못 잰 것을 빼면 정의상 1.0)",
        r.resolved, r.total
    );
    println!(
        "  신호별       {}",
        d.by_source().iter().map(|(k, v)| format!("{k} {v}")).collect::<Vec<_>>().join(" · ")
    );
    println!("  pending→live {}", d.promoted_from_pending);
    println!();
}

/// 답의 모양 한 줄 — `pal query plan.deviation` 이 이것만 낸다.
#[must_use]
pub fn 한_줄_deviation(d: &Deviation) -> String {
    format!(
        "계획대로 {} · 계획에 없던 {} · 미구현 {} · 못 잼 {}",
        d.as_planned.len(),
        d.unplanned.len(),
        d.unimplemented.len(),
        d.unmeasurable.len()
    )
}

fn 상태_한_줄(s: &PlanBindingState) -> String {
    match s {
        PlanBindingState::Bound { targets } => format!("bound {}", targets.len()),
        PlanBindingState::Pending { why } => format!("pending ({why:?})"),
        PlanBindingState::Unresolved { why, candidates } => {
            format!("unresolved ({} · 후보 {})", why.name(), candidates.len())
        }
    }
}

fn 한_줄(s: &str) -> String {
    let one = s.lines().next().unwrap_or("");
    if one.chars().count() <= 72 {
        one.to_owned()
    } else {
        format!("{}…", one.chars().take(72).collect::<String>())
    }
}
