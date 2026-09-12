//! `pal radius <결박> --to <반경>` — **반경만 바꾼다. 결박을 새로 만들지 않는다.**
//!
//! # ⚠ 이 명령은 **「재결박」이 아니다** — 그래서 `pal rebind` 가 아니다
//!
//! 처음에 `pal rebind` 로 세웠다가 고쳤다. [`pal_core::rebind`] 가 이미 있고 그것은
//! **재결박 제안**이다 — *"사라진 좌표 하나에 대한 새 좌표 후보"*(옛 F03 §4.2). 그리고
//! `[f09.pass]` ⑦ 이 **「재결박 일괄 승인 정확도」**로 그 이름을 게이트에 싣고 있다.
//!
//! **이 명령은 좌표를 안 바꾼다.** `target` 이 그대로고 반경만 바뀐다. `rebind` 라 부르면
//! 읽는 사람이 그 게이트 줄로 가고, 거기서 만나는 것은 다른 기능이다. 이름이 잘못
//! 읽히는 것은 병기로 안 고쳐진다(ADR-0033).
//!
//! # 왜 `pal bind` 로는 안 되나
//!
//! 반경을 바꾸는 유일한 길이 `pal bind` 재호출이었고 그것은 **`bound_at` 과 감시
//! 다이제스트를 HEAD 로 재기준한다.** 그러면 지금 `stale` 인 결박이 조용히 `fresh` 가
//! 된다 — 격리 실측에서 `stale 7 → 6` 이었다. **반경을 넓히는 일이 원리상 데이터
//! 손실**이라는 뜻이고, 이 명령이 그것을 막는다.
//!
//! 보존하는 여섯: `id` · `subject` · `note` · `bound_at` · `bound_at_time` · **넓히기 전
//! 감시 집합의 `(symbol, digest)` 쌍 전부.** 앞 다섯은 [`pal_core::Binding::with_radius`]
//! 의 `..self` 가 지고 여섯째는 그 함수가 진다.
//!
//! # 「의도층의 유일한 입구」를 안 깬다
//!
//! [`crate::bind`] 가 *"이것이 의도층의 유일한 입구다"* 라 적었다. 이 명령은 **입구가
//! 아니다** — 없는 결박을 지목하면 실패하고(종료값 ≠ 0) 새 `subject` 도 새 `id` 도
//! 만들지 않는다. 결박의 **수를 못 늘린다.**
//!
//! # ★ 새 감시 원소의 기준 시점은 `bound_at` 의 **base 커밋**이다
//!
//! 넓히면 감시 원소가 새로 든다. 그 `digest` 를 **어느 스냅샷에서 읽는가**가 이 명령의
//! 핵심 결정이고, 소유자가 2026-09-13 에 답했다 — **`bound_at` 의 base 커밋**이다.
//!
//! | 갈래 | 무엇이 되나 |
//! |---|---|
//! | HEAD 에서 읽는다 | 새 원소가 **반드시 `fresh`** 다. *"결정 뒤에 호출자가 변했나"* 를 **영구히 못 묻게 된다** |
//! | **base 커밋에서 읽는다** | 넓히는 즉시 `stale` 이 날 수 있다. **답이 넓어진다** |
//! | 새 `Undeterminable` 사유를 만든다 | `evaluate` 의 ②가 ③④ 앞에서 반환하므로 **`Stale` 을 덮는다** |
//!
//! ⚠⚠ **근사다 — 그 사실이 산출에 실린다.** 이 저장소의 결박 37/37 이 `bound_at` 에
//! **워킹트리 스냅샷**을 싣고 그 `tree_digest` 는 재현할 수 없다. 그래서 base 커밋은
//! *"결박이 걸린 **커밋** 상태"* 이고 *"결박한 순간의 워킹트리"* 가 **아니다.**
//! 두 값이 갈리는지는 **대상 심볼로 잰다** — 저장된 대상 `digest` 와 base 커밋에서 읽은
//! 대상 `digest` 가 다르면 그 결박에서 base 커밋은 충실한 대리가 아니고, 산출이 그것을
//! `대상digest차이` 로 적는다.
//!
//! # 반경은 HEAD 에서 편다 — base 에서 펴지 않는다
//!
//! **감시 집합은 HEAD 의 이웃에서 펴고**(`expand`), 그 원소들의 **기준값만** base 커밋에서
//! 읽는다. base 에서 펴면 지금 없어진 심볼이 감시에 들어오고, 그러면
//! `UndeterminableReason::WatchMemberGone` 이 `Stale` 을 **덮는다**(`evaluate` ②). 그것이
//! 이 회차가 막아야 할 퇴행이고, 조건 `C4` 가 그 상한을 진다.
//!
//! base 커밋에 **없던** 새 원소는 기준값이 원리상 없다. 그때는 HEAD 값을 쓰고(그 원소는
//! 지금부터 지켜본다) **건수를 산출에 적는다** — 조용히 넘기면 *"그때의 값"* 으로 읽힌다.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use pal_core::{
    BindingId, BodyDigest, ObjectName, Radius, SymbolId, WatchEntry, check_budget, expand,
};
use pal_intent::IntentStore;

use crate::{attach, ledger, touch};

/// 이 명령이 받는 것 — `bind::Args` 와 같은 형태다.
pub struct Args<'a> {
    pub repo: &'a Path,
    pub cache_dir: Option<PathBuf>,
    pub index: Option<PathBuf>,
    pub intent: Option<PathBuf>,
    /// 어느 결박인가 — **이름이 아니라 결박 id 다.** 이름을 받으면 새 결박을 만들 수 있다.
    pub id: &'a str,
    /// 무엇까지 지켜보나 — [`Radius::parse`] 가 읽는다.
    pub radius: &'a str,
    /// **재기만 한다** — 의도 저장소에 안 쓴다.
    ///
    /// # 왜 이 손잡이가 있나
    ///
    /// 37 건을 넓히기 **전에** 판정과 근거를 보이는 자리에 세워야 한다(조건 `B2-a`).
    /// 그 산출을 **별도 스크립트**로 만들면 재는 경로와 넓히는 경로가 갈리고, 그러면
    /// 표에 적힌 수가 실제로 일어날 일과 같다는 보장이 없다. **같은 함수가 재고 같은
    /// 함수가 쓴다** — 갈릴 자리를 안 만든다.
    pub dry_run: bool,
}

/// 한 결박의 반경을 바꾼 결과 — **산출이 이 값을 그대로 찍는다.**
pub struct Report {
    /// 무엇에 걸린 결박인가 — 표가 좌표를 져야 한다.
    pub 대상심볼: String,
    pub 대상파일: String,
    pub 옛반경: String,
    pub 새반경: String,
    pub 감시전: usize,
    pub 감시후: usize,
    /// 기준 시점 — `bound_at` 의 base 커밋.
    pub 기준시점커밋: String,
    /// base 커밋에 없어서 기준값을 HEAD 에서 쓴 새 원소 수.
    pub base에없던새원소: usize,
    /// 저장된 대상 `digest` 와 base 커밋에서 읽은 대상 `digest` 가 다른가 —
    /// **근사의 크기**다(모듈 머리의 ⚠⚠).
    pub 대상digest차이: bool,
    /// 새로 든 감시 원소 중 **대상과 다른 파일**에 있는 것의 수.
    ///
    /// # 왜 이 수를 세나 — 목적 기여의 전제가 여기 걸려 있다
    ///
    /// 이 회차가 지금인 근거는 *"순서표 §2 의 2 번(파일 경계를 넘는 참조 해소)이 서면서
    /// **넓은 반경이 빈 집합이 아니게 됐다**"* 다. 새로 든 원소가 **전부 같은 파일 안**
    /// 이었다면 이 일은 2 단계 전에도 가능했고 그 전제가 성립하지 않는다.
    /// **그 반증 가능성을 열어 두는 것이 이 열이다**(조건 `B8`).
    pub 타파일새원소: usize,
    /// 재기만 했나 — `--dry-run`.
    pub 재기만: bool,
}

/// 결박 하나의 반경을 바꾼다.
///
/// # Errors
/// 저장소·2층·의도 저장소 중 하나에 닿지 못하거나, **그 결박이 없거나**, 반경을 모르거나,
/// 반경이 저장 시점 예산을 넘거나, base 커밋의 투영을 세우지 못하면.
pub fn run(a: Args) -> Result<Report> {
    let Args { repo: repo_path, cache_dir, index: index_path, intent: intent_path, id, radius, dry_run } = a;
    let Some(radius) = Radius::parse(radius) else {
        bail!("반경 `{radius}` 를 모른다 — 아는 것은 {} 다", Radius::NAMES.join(" · "));
    };

    let intent = IntentStore::open(&touch::intent_file(repo_path, intent_path))
        .context("의도 저장소를 열지 못했다")?;
    let id = BindingId::new(id);
    // **없는 결박은 여기서 멈춘다.** 이 명령은 결박을 만들지 못한다 — 그것이 `pal bind` 다.
    let Some(옛) = intent.get(&id).context("의도 저장소를 읽지 못했다")? else {
        bail!(
            "결박 `{}` 이 없다 — `pal radius` 는 있는 결박의 반경만 바꾼다. \
             새로 걸려면 `pal bind <이름> --note <조각>` 이다",
            id.as_str()
        );
    };

    // ── HEAD 의 투영 — 여기서 반경을 편다 ────────────────────────────────────
    let head = ledger::compute(repo_path, None, cache_dir.clone())?;
    let index = index_path.unwrap_or_else(|| repo_path.join(".palimpsest/index.redb"));
    let head_attached = attach::attach(&index, &head, attach::How::Stitching)?;
    let head_p = &head_attached.projection;

    // ── `bound_at` 의 base 커밋 ──────────────────────────────────────────────
    let base = base_commit(&옛.bound_at, &head)?;
    let base_hex = base.to_hex();

    // ── base 커밋의 투영 — **별도 색인에 붙인다** ────────────────────────────
    //
    // 정본 색인에 과거 스냅샷을 스티칭하면 `built_for_this_snapshot` 이 그 과거로
    // 갈아 끼워지고, 그러면 **다음 질의에서 판정 불가가 나온다**
    // (`UndeterminableReason::ProjectionStale`). 읽으러 가는 것이 정본을 움직이면 안 된다.
    let base_index = 임시_색인(repo_path, &base_hex);
    let base_report = ledger::compute(repo_path, Some(&base_hex), cache_dir)
        .with_context(|| format!("base 커밋 `{}` 의 원장을 못 세웠다", &base_hex[..7]))?;
    let base_attached = attach::attach(&base_index, &base_report, attach::How::Stitching)
        .with_context(|| format!("base 커밋 `{}` 의 2층을 못 세웠다", &base_hex[..7]))?;
    let base_p = &base_attached.projection;

    // ── 근사의 크기 — 대상의 저장된 값과 base 커밋 값이 갈리나 ───────────────
    let 저장된_대상 = 옛.watch.iter().find(|w| w.symbol == 옛.target).map(|w| w.digest);
    let base_대상 = base_p.symbol(옛.target).context("2층을 읽지 못했다")?.map(|s| s.body);
    let 대상digest차이 = match (저장된_대상, base_대상) {
        (Some(a), Some(b)) => a != b,
        // 한쪽이 없으면 「다르다」로 적는다 — 모르는 것을 같다고 하지 않는다.
        _ => true,
    };

    // ── 감시 집합을 HEAD 에서 펴고 기준값을 base 에서 읽는다 ─────────────────
    let 대상_실물 = head_p
        .symbol(옛.target)
        .context("2층을 읽지 못했다")?
        .context("대상 좌표가 지금 2층에 없다 — 넓히기 전에 그 사실을 먼저 적어야 한다")?;
    let 대상파일 = 대상_실물.path.as_str().to_owned();

    let 감시 = expand(옛.target, &radius, head_p);
    let mut 넓힌: Vec<WatchEntry> = Vec::with_capacity(감시.len());
    let mut base에없던새원소 = 0usize;
    let mut 타파일새원소 = 0usize;
    let 옛_원소: Vec<SymbolId> = 옛.watch.iter().map(|w| w.symbol).collect();
    for s in 감시 {
        // **새로 든 원소가 대상과 다른 파일에 있나** — 조건 `B8` 의 축.
        if !옛_원소.contains(&s) {
            let 어디 = head_p.symbol(s).context("2층을 읽지 못했다")?.map(|n| n.path);
            if 어디.is_some_and(|p| p.as_str() != 대상파일) {
                타파일새원소 += 1;
            }
        }
        // 옛 원소는 `with_radius` 가 옛 값으로 덮는다 — 여기서 읽은 값은 안 쓰인다.
        // 그래도 **읽어서 넣는다**: 그래야 그 함수의 보존이 실제로 일했는지 시험이 잰다.
        let base_body: Option<BodyDigest> =
            base_p.symbol(s).context("2층을 읽지 못했다")?.map(|n| n.body);
        let digest = match base_body {
            Some(d) => d,
            None => {
                // base 커밋에 없던 좌표다 — 기준값이 원리상 없다.
                if !옛_원소.contains(&s) {
                    base에없던새원소 += 1;
                }
                let Some(실물) = head_p.symbol(s).context("2층을 읽지 못했다")? else {
                    bail!("`{s}` 를 2층에서 읽지 못했다 — 반경을 펴는 중에 투영이 갈렸다");
                };
                실물.body
            }
        };
        넓힌.push(WatchEntry { symbol: s, digest });
    }

    let 옛반경 = 옛.radius.name();
    let 감시전 = 옛.watch.len();
    let 대상심볼 = 옛.target.to_hex();
    // ★ **여기가 보존이 일어나는 한 자리다** — 옛 원소의 digest 를 그 함수가 덮는다.
    let 새 = 옛.with_radius(radius, 넓힌);
    let 감시후 = 새.watch.len();

    // ── 저장 시점의 예산 — 건수는 안 늘어난다(있는 결박이다) ────────────────
    let 건수 = intent.count().unwrap_or(0);
    check_budget(건수, 새.watch.len()).map_err(|e| anyhow::anyhow!("{e}"))?;

    // **`--dry-run` 은 여기서만 갈린다.** 예산까지 같은 길을 지나므로, 재기만 한 산출이
    // *"쓰면 거부될 것"* 을 숨기지 않는다.
    if !dry_run {
        intent.record(&새).context("결박을 남기지 못했다")?;
    }

    // 임시 색인은 쓰고 버린다 — 남기면 다음 회차가 그것을 2층으로 오독한다.
    let _ = std::fs::remove_file(&base_index);

    Ok(Report {
        대상심볼,
        대상파일,
        옛반경,
        새반경: 새.radius.name(),
        감시전,
        감시후,
        기준시점커밋: base_hex,
        base에없던새원소,
        대상digest차이,
        타파일새원소,
        재기만: dry_run,
    })
}

/// `bound_at` 이 딛고 선 커밋 — 이 저장소의 것.
///
/// 결박은 여러 저장소에 걸친 감시 집합을 가질 수 있으므로([`pal_core::Binding::bound_at`])
/// **어느 저장소의 그때인가**를 골라야 한다. 지금 붙어 있는 저장소의 것을 쓴다.
fn base_commit(bound_at: &pal_core::Snapshot, head: &ledger::LedgerReport) -> Result<ObjectName> {
    let 여기 = head
        .ledger
        .snapshot
        .entries()
        .next()
        .map(|(r, _)| r.clone())
        .context("지금 스냅샷에 저장소가 없다")?;
    if let Some(t) = bound_at.tree_of(&여기) {
        return Ok(t.base());
    }
    // 이름이 안 맞으면 **하나뿐일 때만** 그것을 쓴다. 여럿이면 고르지 않는다 —
    // 잘못 고른 기준 시점은 조용히 틀린 낡음을 만든다.
    let mut it = bound_at.entries();
    match (it.next(), it.next()) {
        (Some((_, t)), None) => Ok(t.base()),
        _ => bail!(
            "이 결박의 `bound_at` 에 저장소 `{}` 가 없고 후보가 하나가 아니다 — \
             기준 시점을 고를 수 없다",
            여기.as_str()
        ),
    }
}

/// base 커밋 투영을 붙일 **임시 색인** 자리.
fn 임시_색인(repo: &Path, base_hex: &str) -> PathBuf {
    repo.join(".palimpsest").join(format!("radius-base-{}.redb", &base_hex[..12]))
}
