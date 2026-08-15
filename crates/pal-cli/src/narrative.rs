//! `pal narrative` — **문서를 코드 좌표에 건다** (F10).
//!
//! # 이 명령이 하는 일은 조립뿐이다
//!
//! 조각화는 `pal-extract`, 해소는 `pal-core::narrative`, 저장은 `pal-intent` 다.
//! 여기 있는 것은 *"어느 문서를 어느 스냅샷에서"* 뿐이고 **정책이 여기 있으면 안 된다.**
//!
//! # 제안을 저장하지 않는다 — **다시 계산한다**
//!
//! 같은 문서·같은 스냅샷·같은 계단식이면 같은 제안이 나온다(자연어 유사도를 안 쓰는
//! 것이 그것을 보장한다). 저장하면 재생 경로를 하나 더 지어야 하고, **2층에 두면
//! `[f05.2]` ④ 의 모집단이 늘어 남의 합격선이 움직인다.**
//!
//! **저장되는 것은 둘뿐이다**: 개체의 이름(민팅해서 재계산 불가)과 **거부 기록**
//! (사람이 한 일이라 계산에서 안 나온다). 근거 전문은
//! `corpus/criteria.toml` `[f10].queue_placement`.
//!
//! # 인입 대상은 **대장이 정한다**
//!
//! 확장자가 `md`·`mdx` 인 대장 항목 전부다. 도구 설정 디렉터리(`.claude/` 등)를
//! **제품이 빼지 않는다** — 그것은 이 저장소가 쓰는 관례이지 제품의 규칙이 아니다.
//! 측정에서 빼는 것은 `scripts/f10-verify.py` 이고 그 규칙은 **재기 전에 등록됐다**
//! (`[f10].input_quality`).

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use pal_core::{
    Classification, EntityId, EntityKind, EntityOrigin, Fragment, PROVISIONAL_HISTORY_BUDGET,
    Proposal, RepoPath, ResolutionSignal, SymbolId,
};
use pal_git::{GitAccess, GixRepo};
use pal_intent::IntentStore;

use crate::{attach, ledger, touch};

/// 인입 대상 문서의 확장자. **대장이 무엇을 담는지는 대장이 정하고, 무엇이 서술물인지는
/// 여기가 정한다.**
const DOCUMENT_EXTENSIONS: [&str; 2] = ["md", "mdx"];

/// 표식 있는 주석 — 문서 §3.4. **`ADR-` 가 넓은 것은 의도다**: ADR 을 인용하는 주석은
/// **구조상** 결정에 관한 것이다. 표식 없는 일반 주석은 §3.4 가 기각했다.
const COMMENT_MARKERS: [&str; 2] = ["@decision:", "ADR-"];

pub struct Args<'a> {
    pub repo: &'a Path,
    pub rev: Option<&'a str>,
    pub cache_dir: Option<PathBuf>,
    pub index: Option<PathBuf>,
    pub intent: Option<PathBuf>,
    pub json: bool,
    pub what: What<'a>,
}

/// 이 명령의 갈래 셋.
pub enum What<'a> {
    /// 문서를 읽고 3분류를 낸다. **아무것도 승인하지 않는다.**
    Ingest,
    /// 제안 하나를 승인해 `asserted` 결박을 낳는다.
    Approve { item: &'a str, pick: Option<&'a str>, all_of: Option<&'a str> },
    /// 제안 하나를 거부하고 **그 사실을 남긴다**.
    Refuse { item: &'a str, pick: &'a str, reason: &'a str },
}

/// 인입 한 회차의 산출 — **건수가 아니라 회계다.**
pub struct Ingested {
    pub proposals: Vec<Proposal>,
    pub docs: usize,
    pub fragments: usize,
    /// 새로 만들어진 개체. ★ **두 번째 인입에서 0 이어야 한다**(`[f10.1.pass]` ①).
    pub minted: usize,
    /// 이력을 얼마나 훑었나. **훑은 창 밖의 문서는 동반 변경 신호가 없다** —
    /// 조용한 절단 금지.
    pub history_window: usize,
    /// 그 창 안에서 마지막 변경을 못 찾은 문서 수.
    pub outside_window: usize,
}

impl Ingested {
    /// 분류별 건수 — **셋이 전부 실린다.** 하나라도 0 이면 그 사실이 보인다.
    #[must_use]
    pub fn counts(&self) -> BTreeMap<&'static str, usize> {
        let mut m: BTreeMap<&'static str, usize> =
            [("bound", 0), ("candidates", 0), ("unbound", 0)].into_iter().collect();
        for p in &self.proposals {
            *m.entry(p.class.name()).or_insert(0) += 1;
        }
        m
    }

    /// 무엇이 걸었나 — 신호별 건수. **뭉치면 「강 신호로 걸렸다」가 검사되지 않는다.**
    #[must_use]
    pub fn by_signal(&self) -> BTreeMap<&'static str, usize> {
        let mut m = BTreeMap::new();
        for p in &self.proposals {
            if let Some(s) = p.signal() {
                *m.entry(s.name()).or_insert(0) += 1;
            }
        }
        m
    }
}

/// # Errors
/// 저장소·2층·의도 저장소 중 하나에 닿지 못하거나, 승인할 것을 못 찾으면.
pub fn run(a: Args) -> Result<()> {
    let report = ledger::compute(a.repo, a.rev, a.cache_dir)?;
    let index = a.index.unwrap_or_else(|| a.repo.join(".palimpsest/index.redb"));
    // **스티칭한다** — 해소가 이름 인덱스와 파일 목록을 요구한다. `pal bind` 와 같은 자리.
    let attached = attach::attach(&index, &report, attach::How::Stitching)?;
    let intent = IntentStore::open(&touch::intent_file(a.repo, a.intent))
        .context("의도 저장소를 열지 못했다")?;

    let got = ingest(a.repo, &report, &attached.projection, &intent)?;

    match a.what {
        What::Ingest => 화면(&got, a.json),
        What::Approve { item, pick, all_of } => {
            승인(&got, &intent, &report, &attached.projection, item, pick, all_of)
        }
        What::Refuse { item, pick, reason } => 거부(&got, &intent, &report, item, pick, reason),
    }
}

/// 문서를 읽고 제안을 만든다. **아무것도 승인하지 않는다.**
///
/// # Errors
/// 저장소를 읽지 못하거나 개체 이름을 남기지 못하면.
pub fn ingest(
    repo: &Path,
    report: &ledger::LedgerReport,
    projection: &pal_store::Projection,
    intent: &IntentStore,
) -> Result<Ingested> {
    let git = GixRepo::open(repo).context("저장소를 열지 못했다")?;
    let at = &report.ledger.snapshot_tree();

    // ── 동반 변경 — **이력을 한 번만 훑는다** ────────────────────────────────
    //
    // 문서마다 이력을 훑으면 비용이 `문서 수 × 이력 깊이` 다. 한 번 훑어
    // `경로 → 마지막으로 바꾼 커밋` 을 만들면 `이력 깊이` 하나로 끝난다.
    let (마지막_변경, 커밋의_변경) = 이력(&git, at.base())?;

    let mut proposals = Vec::new();
    let mut docs = 0;
    let mut fragments = 0;
    let mut minted = 0;
    let mut outside = 0;

    for entry in &report.ledger.entries {
        let path = &entry.path;
        let code = DOCUMENT_EXTENSIONS.contains(&path.extension());
        let 표식 = 표식_주석(&git, at, path, projection)?;
        if !code && 표식.is_empty() {
            continue;
        }
        let mut 조각들 = if code {
            docs += 1;
            let Some(src) = 읽는다(&git, at, path)? else { continue };
            let Ok(text) = String::from_utf8(src) else {
                // **바이너리로 읽히는 `.md` 는 조용히 건너뛰지 않는다** — 대장이
                // 이미 그것을 `binary` 로 세고 있고, 여기서 다시 세면 두 곳에 적힌다.
                continue;
            };
            pal_extract::fragment(path, &text)
        } else {
            Vec::new()
        };
        조각들.extend(표식);

        // 동반 변경 신호 — **이 문서가 마지막으로 바뀐 커밋이 함께 바꾼 것들.**
        let 동반: Vec<RepoPath> = if let Some(c) = 마지막_변경.get(path) {
            커밋의_변경
                .get(c)
                .map(|v| v.iter().filter(|p| *p != path).cloned().collect())
                .unwrap_or_default()
        } else {
            outside += 1;
            Vec::new()
        };

        for mut f in 조각들 {
            fragments += 1;
            f.signals.co_changed.clone_from(&동반);
            let origin = 출처_열쇠(&f);
            let 이미 = intent.entity_of(&origin).context("개체를 읽지 못했다")?;
            let item = if let Some(id) = 이미 {
                id
            } else {
                    // ★ **민팅은 처음 한 번뿐이다.** 매번 뽑으면 같은 문서를 두 번 읽을 때
                    //   개체가 둘이 되고, **읽기가 더하기가 아니라 복제가 된다**.
                    let id = EntityId::mint(
                        EntityKind::new("decision"),
                        EntityOrigin::Document {
                            path: f.path.as_str().to_owned(),
                            anchor: f.anchor.clone(),
                        },
                    );
                    intent.keep_entity(&origin, &id).context("개체를 남기지 못했다")?;
                    minted += 1;
                    id
            };
            let class = pal_core::resolve(&f, projection);
            proposals.push(Proposal { item, fragment: f, class });
        }
    }

    // **결정적 순서** — 흔들리면 사람이 보는 목록이 흔들리고, 흔들리는 목록은 승인의
    // 근거가 못 된다(`rebind::propose_with_shape` 와 같은 자리).
    proposals.sort_by(|a, b| {
        (a.fragment.path.as_str(), a.fragment.anchor.as_str())
            .cmp(&(b.fragment.path.as_str(), b.fragment.anchor.as_str()))
    });

    Ok(Ingested {
        proposals,
        docs,
        fragments,
        minted,
        history_window: PROVISIONAL_HISTORY_BUDGET,
        outside_window: outside,
    })
}

/// 조각 하나의 출처 열쇠 — `<경로>\0<앵커>`.
///
/// **정체성이 아니라 「이 자리를 이미 봤는가」의 열쇠다.** 문서가 이동하면 이 값이
/// 바뀌고 새 개체가 생긴다 — 이동 시 재연결은 F10 의 범위 밖이고 게이트에 적혀 있다.
fn 출처_열쇠(f: &Fragment) -> String {
    format!("{}\u{0}{}", f.path, f.anchor)
}

/// 이력을 한 번 훑어 둘을 만든다 — `경로 → 마지막으로 바꾼 커밋` 과 `커밋 → 바꾼 경로들`.
/// 마지막으로 바꾼 커밋 · 커밋이 바꾼 경로들.
type 이력표 = (
    BTreeMap<RepoPath, pal_core::ObjectName>,
    BTreeMap<pal_core::ObjectName, Vec<RepoPath>>,
);

fn 이력(git: &GixRepo, from: pal_core::ObjectName) -> Result<이력표> {
    let mut 마지막 = BTreeMap::new();
    let mut 변경 = BTreeMap::new();
    let commits = git
        .first_parent_walk(from, PROVISIONAL_HISTORY_BUDGET)
        .context("이력을 읽지 못했다")?;
    for c in commits {
        let Ok(paths) = git.changed_in(c) else { continue };
        for p in &paths {
            // **가장 최근 것이 이긴다** — 훑는 순서가 최신부터다.
            마지막.entry(p.clone()).or_insert(c);
        }
        변경.insert(c, paths);
    }
    Ok((마지막, 변경))
}

/// 스냅샷에서 파일 하나를 읽는다. **워킹트리를 물으면 워킹트리를 읽는다.**
fn 읽는다(
    git: &GixRepo,
    at: &pal_core::TreeRef,
    path: &RepoPath,
) -> Result<Option<Vec<u8>>> {
    if at.is_committed() {
        let Some(oid) = git.path_at(at, path).context("트리를 읽지 못했다")? else {
            return Ok(None);
        };
        return Ok(Some(git.read_blob(oid).context("blob 을 읽지 못했다")?));
    }
    Ok(git.read_worktree_file(path).ok())
}

/// 표식 있는 주석을 조각으로 — **좌표는 계산하지 않는다. 이미 있다**(§3.4).
fn 표식_주석(
    git: &GixRepo,
    at: &pal_core::TreeRef,
    path: &RepoPath,
    projection: &pal_store::Projection,
) -> Result<Vec<Fragment>> {
    let Some(language) = pal_core::Language::from_extension(path.extension()) else {
        return Ok(Vec::new());
    };
    let pal_core::Capable::Present(extractor) = pal_extract::extractor_for(language) else {
        return Ok(Vec::new());
    };
    let Some(src) = 읽는다(git, at, path)? else { return Ok(Vec::new()) };
    let Ok(comments) = extractor.marked_comments(&src, &COMMENT_MARKERS) else {
        return Ok(Vec::new());
    };
    if comments.is_empty() {
        return Ok(Vec::new());
    }

    // 붙을 선언을 **바이트 자리로** 찾는다 — 이름으로 찾으면 같은 이름이 여럿일 때
    // 엉뚱한 것에 붙는다.
    let 심볼들 = projection.symbols_of(path).unwrap_or_default();
    let 자리: BTreeMap<_, SymbolId> =
        심볼들.iter().map(|s| (s.span.byte_start, s.id)).collect();

    let mut out = Vec::new();
    for c in comments {
        let attached: Vec<SymbolId> =
            c.attaches_to_byte.and_then(|b| 자리.get(&b).copied()).into_iter().collect();
        out.push(Fragment {
            path: path.clone(),
            // 주석에는 헤딩이 없다. **줄 번호가 문서 안에서 유일한 자리다.**
            anchor: format!("L{}", c.span.line_start),
            body: c.text,
            signals: pal_core::RawSignals { attached, ..pal_core::RawSignals::default() },
        });
    }
    Ok(out)
}

// ─────────────────────────────────────────────────────────────────────────────
// 승인과 거부 — **둘 다 사람의 행위다**
// ─────────────────────────────────────────────────────────────────────────────

fn 찾는다<'a>(got: &'a Ingested, item: &str) -> Result<&'a Proposal> {
    got.proposals
        .iter()
        .find(|p| p.item.to_display() == item || p.item.id.to_string() == item)
        .ok_or_else(|| {
            anyhow::anyhow!(
                "제안 `{item}` 을 이 스냅샷에서 못 찾았습니다 — `pal narrative` 로 목록을 보십시오.\n\
                 **제안은 저장되지 않고 다시 계산됩니다**: 문서나 스냅샷이 바뀌면 이름도 바뀝니다"
            )
        })
}

fn 승인(
    got: &Ingested,
    intent: &IntentStore,
    report: &ledger::LedgerReport,
    projection: &pal_store::Projection,
    item: &str,
    pick: Option<&str>,
    all_of: Option<&str>,
) -> Result<()> {
    if let Some(prefix) = all_of {
        return 일괄_승인(got, intent, report, projection, prefix);
    }
    let p = 찾는다(got, item)?;
    let choices = p.choices();
    let target = match (choices.as_slice(), pick) {
        ([], _) => bail!(
            "이 조각에는 좌표 후보가 없습니다 — 승인할 것이 없고 `narrative.unbound` 에 남습니다"
        ),
        ([only], None) => *only,
        (many, None) => bail!(
            "후보가 {}건입니다 — **하나를 골라 드리지 않습니다**. `--pick <좌표>` 로 고르십시오:\n  {}",
            many.len(),
            many.iter().map(ToString::to_string).collect::<Vec<_>>().join("\n  ")
        ),
        (_, Some(raw)) => 좌표(raw, &choices)?,
    };
    if intent.refused(&p.item, target).context("거부 기록을 읽지 못했다")? {
        bail!(
            "이 짝은 이미 거부됐습니다 — **재질문 제거가 승인 비용 절감의 대부분입니다**(F10 §3.3).\n\
             다시 걸려면 `pal bind` 로 손으로 거십시오. 그러면 `hand` 로 남습니다"
        );
    }
    let b = 결박한다(p, target, report, projection)?;
    intent.record(&b).context("결박을 남기지 못했다")?;
    println!();
    println!("승인했습니다 — **새 `asserted` 결박이 생겼고 제안은 그대로 남습니다.**");
    println!("  개체    {}", p.item.to_display());
    println!("  좌표    {target}");
    println!("  걸린 것 {}", p.signal().map_or("—", ResolutionSignal::name));
    println!("  결박    [{}]", b.id.as_str());
    println!();
    Ok(())
}

/// 같은 파일의 조각들을 한 번에 — **F09 의 거부 규칙을 그대로 쓴다**(문서 §4).
///
/// # 왜 `approve_batch` 를 그대로 못 쓰는가 — **그리고 규칙을 복제하지 않는다**
///
/// [`pal_core::approve_batch`] 는 `RebindProposal`(옛 좌표 → 새 좌표)을 받는다.
/// 문서 조각의 제안에는 **옛 좌표가 없다** — 처음 거는 것이기 때문이다. 타입이 안 맞고,
/// 억지로 맞추면 `orphaned` 자리에 뜻 없는 값을 넣게 된다.
///
/// **그래서 규칙을 복제하지 않고 형태만 지킨다**: ① 하나라도 걸리면 묶음 전체 거부
/// ② 후보가 여럿이면 안 고른다 ③ 거부 이유가 값으로 남는다.
/// 그 셋이 `approve_batch` 가 지키는 것이고, **여기서는 「신호 셋이 전부 맞아야 한다」가
/// 「확인된 신호로 걸렸어야 한다」로 바뀐다** — 축이 다르기 때문이다.
/// 그 사실을 게이트에 적는다.
fn 일괄_승인(
    got: &Ingested,
    intent: &IntentStore,
    report: &ledger::LedgerReport,
    projection: &pal_store::Projection,
    prefix: &str,
) -> Result<()> {
    let 묶음: Vec<&Proposal> = got
        .proposals
        .iter()
        .filter(|p| p.fragment.path.as_str().starts_with(prefix))
        .filter(|p| !matches!(p.class, Classification::Unbound))
        .collect();
    if 묶음.is_empty() {
        bail!("`{prefix}` 아래에 승인할 제안이 없습니다");
    }

    // ★ **거부를 먼저 전부 모은다.** 하나라도 걸리면 묶음 전체를 거부한다 —
    //   부분 승인은 *"어디까지 승인했나"* 를 사람이 다시 세게 한다(F09 의 판단).
    let mut 거부들 = Vec::new();
    for p in &묶음 {
        match &p.class {
            Classification::Candidates { candidates, .. } => 거부들.push(format!(
                "{} — 후보가 {}건입니다. **하나를 골라 드리지 않습니다**",
                p.item.to_display(),
                candidates.len()
            )),
            // ⚠ **이 사유는 이제 구조적으로 도달 불가다** (2026-08-15 ·
            //   `[f10.5.pass].batch_refusal_grounds`). `Classification::Bound` 가
            //   `ConfirmingSignal` 을 지므로 **거리 있는 신호가 여기 올 수 없다.**
            //
            //   ★ **그런데 지우지 않는다.** 나중에 거리 있는 신호가 확정을 내게 되면
            //   **이 사유가 다시 켜져야** 하고, 지우면 그때 아무것도 안 막는다.
            //   모집단이 0 이 된 사실은 게이트가 적는다 — *"안 켜진다"* 와 *"없다"* 를
            //   가르는 것이 [ADR-0002] 다.
            Classification::Bound { by, .. } if !by.signal().can_confirm_subject() => {
                거부들.push(format!(
                    "{} — `{}` 는 판단이 드는 신호입니다. 일괄의 대상이 아닙니다",
                    p.item.to_display(),
                    by.name()
                ));
            }
            _ => {}
        }
    }
    if !거부들.is_empty() {
        bail!(
            "일괄 승인을 거부합니다 — **{}건이 걸렸고 묶음 전체가 거부됩니다**\n  {}\n\n  \
             부분 승인은 「어디까지 승인했나」를 사람이 다시 세게 합니다. 하나씩 보십시오",
            거부들.len(),
            거부들.join("\n  ")
        );
    }

    let mut n = 0;
    for p in &묶음 {
        let Classification::Bound { target, .. } = &p.class else { continue };
        if intent.refused(&p.item, *target).context("거부 기록을 읽지 못했다")? {
            continue;
        }
        let b = 결박한다(p, *target, report, projection)?;
        intent.record(&b).context("결박을 남기지 못했다")?;
        n += 1;
    }
    println!();
    println!("일괄 승인했습니다 — `{prefix}` 아래 **{n}건**.");
    println!();
    Ok(())
}

fn 거부(
    got: &Ingested,
    intent: &IntentStore,
    report: &ledger::LedgerReport,
    item: &str,
    pick: &str,
    reason: &str,
) -> Result<()> {
    if reason.trim().is_empty() {
        bail!(
            "거부 이유가 비었습니다 — *\"거부했다\"* 만 적으면 다음 사람이 **왜인지 모른 채 \
             같은 후보를 다시 봅니다**(F10 §3.3)"
        );
    }
    let p = 찾는다(got, item)?;
    let target = 좌표(pick, &p.choices())?;
    intent
        .keep_refusal(&pal_core::Refusal {
            item: p.item.clone(),
            target,
            at: report.ledger.snapshot.clone(),
            reason: reason.to_owned(),
        })
        .context("거부를 남기지 못했다")?;
    println!();
    println!("거부를 남겼습니다 — **다시 묻지 않습니다.**");
    println!("  개체    {}", p.item.to_display());
    println!("  좌표    {target}");
    println!("  이유    {reason}");
    println!();
    Ok(())
}

/// 사람이 고른 좌표가 **후보 안인지** 본다.
///
/// ★ **후보 밖을 승인할 수 있으면 그것은 승격이 아니라 지어낸 결박이다**
/// ([`pal_core::PromotionRefusal::NotACandidate`] 가 같은 판단을 타입으로 진다).
fn 좌표(raw: &str, choices: &[SymbolId]) -> Result<SymbolId> {
    choices
        .iter()
        .find(|c| c.to_string() == raw || c.short() == raw)
        .copied()
        .ok_or_else(|| {
            anyhow::anyhow!(
                "`{raw}` 는 이 제안의 후보 {}건에 없습니다 — **후보 밖의 좌표를 승인하면 \
                 그것은 승격이 아니라 새로 지어낸 결박입니다.** `pal bind` 를 쓰십시오",
                choices.len()
            )
        })
}

/// 승인된 제안 하나를 결박으로 — **감시 집합을 여기서 편다.**
///
/// 반경은 `symbol` 이다. F09 가 *"`callers` 로 올리는 것은 실측으로 검토"* 를 적었고
/// **그 입력이 아직 없다**(F09 게이트 §4-라: 두 반경의 표본이 같았다).
fn 결박한다(
    p: &Proposal,
    target: SymbolId,
    report: &ledger::LedgerReport,
    projection: &pal_store::Projection,
) -> Result<pal_core::Binding> {
    let radius = pal_core::Radius::Symbol;
    let 감시 = pal_core::expand(target, &radius, projection);
    let mut watch = Vec::with_capacity(감시.len());
    for s in 감시 {
        let Some(실물) = projection.symbol(s).context("2층을 읽지 못했다")? else {
            bail!("`{s}` 를 2층에서 읽지 못했다 — 반경을 펴는 중에 투영이 갈렸다");
        };
        watch.push(pal_core::WatchEntry { symbol: s, digest: 실물.body });
    }
    pal_core::check_budget(1, watch.len()).map_err(|e| anyhow::anyhow!("{e}"))?;
    pal_core::Binding::promote(
        p,
        target,
        pal_core::PromotionSite {
            bound_at: report.ledger.snapshot.clone(),
            bound_at_time: pal_core::BoundTime::Worktree,
            radius,
            watch,
        },
    )
    .map_err(|e| anyhow::anyhow!("{e}"))
}

// ─────────────────────────────────────────────────────────────────────────────
// 화면
// ─────────────────────────────────────────────────────────────────────────────

fn 화면(got: &Ingested, json: bool) -> Result<()> {
    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "docs": got.docs,
                "fragments": got.fragments,
                "minted": got.minted,
                "history_window": got.history_window,
                "outside_window": got.outside_window,
                "counts": got.counts(),
                "by_signal": got.by_signal(),
                "proposals": got.proposals,
            }))?
        );
        return Ok(());
    }

    let c = got.counts();
    println!();
    println!("■ 서술물 인입");
    println!("  문서 {} · 조각 {} · 새 개체 {}", got.docs, got.fragments, got.minted);
    println!(
        "  결박됨 {} · 후보 있음 {} · 미결박 {}",
        c["bound"], c["candidates"], c["unbound"]
    );
    println!();
    println!("■ 무엇이 걸었나");
    if got.by_signal().is_empty() {
        println!("  (아무 신호도 안 걸렸습니다)");
    }
    for (s, n) in got.by_signal() {
        println!("  {s:<22} {n}");
    }
    println!();
    // **조용한 절단 금지** — 훑은 창 밖의 문서는 동반 변경 신호가 아예 없다.
    println!("■ 이 인입이 못 본 것");
    println!("  이력 창 {} 커밋 · 창 밖에서 마지막으로 바뀐 문서 {}", got.history_window, got.outside_window);
    println!("  **그 문서들에는 「같은 커밋」 신호가 없습니다** — 없는 것이지 0 이 아닙니다.");
    println!();
    println!("  **아무것도 승인하지 않았습니다.** `inferred` 는 사람의 승인으로만");
    println!("  `asserted` 가 됩니다 — `pal narrative approve <개체>`.");
    println!();
    Ok(())
}
