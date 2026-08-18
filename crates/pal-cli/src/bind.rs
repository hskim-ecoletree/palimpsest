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
use pal_core::{
    Binding, BindingId, BoundTime, EntityId, EntityKind, EntityOrigin, NewBinding, Radius,
    SymbolIdentity,
    WatchEntry, check_budget, expand,
};
use pal_git::{GitAccess, GixRepo};
use pal_intent::IntentStore;

use crate::{attach, ledger, touch};

/// 이 명령이 받는 것 — **`query::Args` 와 같은 형태다.**
///
/// 손잡이를 늘리면 인자가 늘고, 인자가 여덟이면 부르는 쪽이 순서를 틀린다.
/// 구조체로 묶으면 이름이 붙는다.
pub struct Args<'a> {
    pub repo: &'a Path,
    pub rev: Option<&'a str>,
    pub cache_dir: Option<PathBuf>,
    pub index: Option<PathBuf>,
    pub intent: Option<PathBuf>,
    pub name: &'a str,
    pub note: &'a str,
    /// 무엇까지 지켜보나 — 손잡이의 날것. [`Radius::parse`] 가 읽는다.
    pub radius: &'a str,
}

/// 좌표 하나에 조각 하나를 건다.
///
/// # Errors
/// 저장소·2층·의도 저장소 중 하나에 닿지 못하거나, 이름이 유일하지 않거나,
/// 반경을 모르거나, 반경이 저장 시점 예산을 넘으면.
pub fn run(a: Args) -> Result<()> {
    let Args { repo: repo_path, rev, cache_dir, index: index_path, intent: intent_path, name, note, radius } = a;
    // **모르는 반경은 여기서 멈춘다.** 조용히 `symbol` 로 되돌아가면 사용자가 **더
    // 넓다고 믿는 반경**에서 좁은 감시를 받는다 — 거짓 음성을 선언으로 다룬다는
    // 이 설계의 정면 위반이다(옛 F09 §3).
    let Some(radius) = Radius::parse(radius) else {
        bail!("반경 `{radius}` 를 모른다 — 아는 것은 {} 다", Radius::NAMES.join(" · "));
    };
    let report = ledger::compute(repo_path, rev, cache_dir)?;

    let index = index_path.unwrap_or_else(|| repo_path.join(".palimpsest/index.redb"));
    // **스티칭한다 — `SymbolsOnly` 가 아니다.**
    //
    // 옛 판은 `Projection::rebuild` 를 불렀고 그것이 **엣지를 지웠다**(F06 게이트
    // §6-가-2 · 실측 ditto 4,601 → 0 · `built_for_this_snapshot` true → false).
    // S3 가 그렇게 썼고 F05 가 `stitch` 를 세우면서 안 옮긴 자리다.
    //
    // **F09 가 고치는 자리인 이유**: `Radius::Callers` 가 **엣지를 요구한다.** 엣지를
    // 지우는 채로 반경을 세우면 **감시 집합이 조용히 빈다** — 그리고 빈 감시 집합은
    // 언제나 `Live` 이므로 이 기능의 반대 방향 넷 중 셋이 **공짜로 통과한다.**
    //
    // 회귀는 `tests/bind_preserves_edges.rs` 가 막는다. **`--read-only` 로 물어야
    // 보인다** — 쓰기로 물으면 `pal query` 가 스티칭을 다시 돌려 증상을 가린다.
    let attached = attach::attach(&index, &report, attach::How::Stitching)?;
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

    // **좌표를 꺼내는 길이 둘뿐이다** (옛 F03 §3.3). `Unavailable` 에는 실린 좌표가
    // 없으므로 여기서 결박이 끝난다 — 그 판정이 `if` 가 아니라 **타입**이다.
    let target = match SymbolIdentity::new(symbol.identity, symbol.id) {
        SymbolIdentity::Exact(id) | SymbolIdentity::Ordinal(id) => id,
        SymbolIdentity::Unavailable => bail!(
            "`{name}` 은 좌표가 없다 — 이 언어의 추출 등급이 L0 이라 결박이 성립하지 않는다"
        ),
    };

    // **감시 집합을 반경으로 편다.** `expand` 는 2층을 모르고 `Neighborhood` 위에서
    // 돈다 — 투영이 그 트레잇의 실물 구현이다.
    let 감시 = expand(target, &radius, projection);
    // 요약을 **투영에서 읽는다.** 생산자의 신고를 여기 넣는 경로가 없고, 그것이
    // 옛 F09 §4.1(D32)이 요구한 *"`watch_snapshot` 은 신고받지 않는다"* 다.
    let mut watch = Vec::with_capacity(감시.len());
    for s in 감시 {
        let Some(실물) = projection.symbol(s).context("2층을 읽지 못했다")? else {
            // 방금 편 좌표가 2층에 없다 — 읽는 중에 갈렸다는 뜻이다. **조용히
            // 빠뜨리지 않는다**: 빠뜨리면 감시 집합이 줄고 그만큼 덜 지켜본다.
            bail!("`{s}` 를 2층에서 읽지 못했다 — 반경을 펴는 중에 투영이 갈렸다");
        };
        watch.push(WatchEntry { symbol: s, digest: 실물.body });
    }

    let intent = IntentStore::open(&touch::intent_file(repo_path, intent_path))
        .context("의도 저장소를 열지 못했다")?;

    // ── 저장 시점의 예산 (옛 F09 §3) ──────────────────────────────────────────
    //
    // **런타임에 조용히 느려지는 대신 여기서 실패한다.** 이미 있는 결박이면 건수가
    // 안 늘므로 지금 수를, 새 결박이면 +1 을 쓴다.
    let id = BindingId::derive(target, note);
    let 이미 = intent.get(&id).context("의도 저장소를 읽지 못했다")?;
    let 건수 = intent.count().unwrap_or(0) + usize::from(이미.is_none());
    check_budget(건수, watch.len()).map_err(|e| anyhow::anyhow!("{e}"))?;

    // ── 개체 (옛 F09 §4.3) ────────────────────────────────────────────────────
    //
    // **같은 조각을 같은 좌표에 두 번 걸면 하나다.** 이미 있으면 그 개체를 물려받는다 —
    // 새로 뽑으면 같은 것이 둘이 된다.
    let subject = 이미.map_or_else(
        || EntityId::mint(EntityKind::new("decision"), EntityOrigin::Hand),
        |b| b.subject,
    );

    let binding = Binding::new(NewBinding {
        subject,
        target,
        note: note.to_owned(),
        bound_at: report.ledger.snapshot.clone(),
        bound_at_time: 커밋_시각(repo_path, &report),
        radius,
        watch,
    });
    intent.record(&binding).context("결박을 남기지 못했다")?;

    println!();
    println!("결박했습니다.");
    println!("  대상    {} · {} · {}:{}", symbol.name, symbol.kind.name(), symbol.path, symbol.span.line_start);
    println!("  좌표    {}#{}", report.ledger.snapshot, symbol.id.short());
    println!("  본문    {}  ← 이 값이 바뀌면 낡음이 표시됩니다", symbol.body.short());
    println!("  개체    {}", binding.subject.to_display());
    // **반경과 감시 집합 크기를 함께 낸다.** *"이 결정은 `symbol` 반경에서 live"* 는
    // *"이 결정은 유효하다"* 와 다른 문장이고, 그 차이가 산출에 남아야 한다(옛 F09 §3).
    println!("  반경    {} · 감시 {} 개", binding.radius.name(), binding.watch.len());
    println!("  결박    [{}]", binding.id.as_str());
    println!();
    println!("  의도 저장소에 {} 건 있습니다. **파생층을 지워도 남습니다.**",
             intent.count().unwrap_or(0));
    println!();
    Ok(())
}

/// 결박한 코드의 커밋 시각 — **표시용이다. 앵커가 아니다**([`BoundTime`]).
///
/// 못 읽어도 멈추지 않는다. 표시용 값 하나 때문에 결박이 실패하면 그것이 더 나쁘고,
/// **모른다는 사실이 값으로 남는다**([`BoundTime::Unrecorded`]) — 조용히 0 을 넣으면
/// 화면에 *"1970년 코드 기준"* 이 뜬다.
fn 커밋_시각(repo_path: &Path, report: &ledger::LedgerReport) -> BoundTime {
    let tree = report.ledger.snapshot_tree();
    if !tree.is_committed() {
        return BoundTime::Worktree;
    }
    GixRepo::open(repo_path)
        .and_then(|r| r.commit(tree.base()))
        .map_or(BoundTime::Unrecorded, |m| BoundTime::Committed { epoch_secs: m.epoch_secs })
}
