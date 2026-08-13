//! `pal defect <수정 커밋>` — **결함의 소급 결박.**
//!
//! [T10ⓐ](../../../docs/gates/preflight.md#t10--여정결함의-올라탈-곳)가 손으로 잰 것을
//! 코드가 재현한다. 그때의 방법은 **삭제된 줄의 blame** 이었고, 여기서는
//! **`body_digest` 가 마지막으로 변한 조상**이다. 바꾼 이유는 [`pal_core::Defect`] 의
//! 문서에 있다 — 줄은 이 좌표계가 아니고, 줄 위에서 결박하면 포매팅 커밋이 도입 커밋으로
//! 지목된다([R-07]).
//!
//! # 세 단계
//!
//! ```text
//! ① 수정 커밋과 부모의 트리를 대조해 **변한 파일**을 찾는다
//! ② 그 파일들에서 **body_digest 가 변한 심볼** = 발현 좌표
//! ③ 각 발현 좌표에 대해 부모부터 거슬러 올라가 **그 digest 가 처음 달라지는 조상**의
//!    바로 앞 커밋 = 그 좌표의 도입 커밋. 최빈이 결함의 도입 커밋이고 **비율이 신뢰도다**
//! ```
//!
//! **①에서 아무것도 안 나오거나 ②가 비면 그것은 실패가 아니라 `Uncapturable` 이다** —
//! 세어서 표시하는 것이 이 명령의 절반이다(T10 표본 5 건 중 1 건이 그 자리였다).

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use anyhow::{Context, Result};
use pal_core::{
    BodyDigest, Change, ChangeId, ChangeKind, Confidence, Defect, DerivedId, Introduction,
    CapabilityId, Language, NodeRef, NotFoundReason, ObjectName, RepoPath, ReproInput,
    Retrobinding, RetrobindingSummary, Snapshot, SymbolId, TreeRef, Uncapturable,
};
use pal_git::{GitAccess, GixRepo};

use crate::ledger;

/// 이력을 얼마나 거슬러 올라가는가. **예산이고, 걸리면 그 사실이 산출에 남는다.**
///
/// 값은 자리표시다(stack §5.5) — 어느 측정도 이 숫자를 정하지 않았고, DESIGN §12.4 의
/// 예산 표에 `미측정` 으로 서 있다(2026-08-13). 재는 것은 **F05** 의 예산 회귀다.
/// 걸린 것과 정말 없는 것을 [`NotFoundReason::HistoryBudget`] 이 구별한다.
const PROVISIONAL_HISTORY_BUDGET: usize = 400;

/// 한 커밋에서 뽑은 것 전부 — `Change` 와 소급 결박 결과.
#[derive(Debug, serde::Serialize)]
pub struct DefectReport {
    pub change: Change,
    pub result: Retrobinding,
    pub summary: RetrobindingSummary,
    /// 변했는데 **이 빌드가 읽지 못한** 파일들. **빈 목록이어야 답이 온전하다.**
    ///
    /// `pal defect` 는 아직 `Envelope` 를 돌려주지 않는다(`pal touch` 만 그렇다).
    /// 그때까지 이 자리가 **커버리지의 구멍을 산출에 남기는** 유일한 곳이다 —
    /// 없으면 부분만 본 답이 전부 본 답처럼 보인다.
    pub not_extracted: Vec<(RepoPath, CapabilityId)>,
}

/// # Errors
/// 저장소를 열지 못하거나 커밋을 읽지 못하면.
pub fn run(rev: &str, repo_path: &Path, budget: usize) -> Result<DefectReport> {
    let git = GixRepo::open(repo_path)
        .with_context(|| format!("git 저장소가 아니다: {}", repo_path.display()))?;
    let fix = git.resolve_commit(rev).with_context(|| format!("커밋을 찾지 못했다: {rev}"))?;
    let meta = git.commit(fix).context("커밋을 읽지 못했다")?;

    let repo_id = pal_core::RepoId::new(ledger::repo_name(repo_path));
    let at = Snapshot::single(repo_id.clone(), TreeRef::Committed(fix));
    let change_id = ChangeId::new(fix.to_hex());

    let Some(&parent) = meta.parents.first() else {
        // 뿌리 커밋에는 대조할 앞이 없다. **오류가 아니라 담기지 않은 것이다.**
        return Ok(report(
            &meta,
            &at,
            Retrobinding::Missed(Uncapturable::NoSemanticChange { change: change_id }),
            Vec::new(),
            Vec::new(),
        ));
    };

    let here = TreeRef::Committed(fix);
    let before = TreeRef::Committed(parent);

    // ── ① 변한 파일 ─────────────────────────────────────────────────────────
    let changed = changed_files(&git, &before, &here)?;
    let (recognized, opaque): (Vec<_>, Vec<_>) =
        changed.iter().partition(|p| Language::from_extension(p.extension()).is_some());

    // **"언어를 모른다" 와 "언어는 아는데 추출기가 없다" 는 다른 사건이다.**
    // 뭉개면 추출기 없는 파일이 "변한 것 없음" 으로 세어지고, 그것이 곧 이 제품이
    // 고발한 형태다(목표 §3.1). 아래 `not_built` 가 그 경계다.
    let mut not_built: Vec<(RepoPath, CapabilityId)> = Vec::new();
    let mut extractable: Vec<&RepoPath> = Vec::new();
    for p in recognized {
        let language = Language::from_extension(p.extension()).expect("인식된 것이다");
        match pal_extract::capability(language) {
            pal_core::Capable::Present(()) => extractable.push(p),
            pal_core::Capable::NotBuilt { capability } => not_built.push((p.clone(), capability)),
        }
    }

    if extractable.is_empty() && !not_built.is_empty() {
        let capability = not_built[0].1;
        return Ok(report(
            &meta,
            &at,
            Retrobinding::Missed(Uncapturable::NoExtractor {
                change: change_id,
                capability,
                files: not_built.iter().map(|(p, _)| p.clone()).collect(),
            }),
            Vec::new(),
            not_built,
        ));
    }

    if extractable.is_empty() {
        // **T10 표본 5 건 중 1 건이 여기였다** — 결함이 코드가 아니라 지시 문서에 있었다.
        return Ok(report(
            &meta,
            &at,
            Retrobinding::Missed(Uncapturable::OutsideCode {
                change: change_id,
                files: opaque.into_iter().cloned().collect(),
            }),
            Vec::new(),
            not_built,
        ));
    }

    // ── ② 발현 좌표 ─────────────────────────────────────────────────────────
    let manifests = manifestations(&git, &repo_id, &before, &here, &extractable)?;
    let total_manifests: usize = manifests.values().map(Vec::len).sum();
    if total_manifests == 0 {
        return Ok(report(
            &meta,
            &at,
            Retrobinding::Missed(Uncapturable::NoSemanticChange { change: change_id }),
            Vec::new(),
            not_built,
        ));
    }

    // ── ③ 도입 커밋 ─────────────────────────────────────────────────────────
    let introduced_by = introduction(&git, &repo_id, parent, &manifests, budget)?;

    let manifest_ids: Vec<SymbolId> =
        manifests.values().flatten().map(|(id, _)| *id).collect();
    let repro_base = match &introduced_by {
        Introduction::Found { change, .. } => ObjectName::from_bytes(
            hex20(change.as_str()).unwrap_or_else(|| *parent.as_bytes()),
        ),
        Introduction::NotFound { .. } => parent,
    };

    let defect = Defect {
        // **파생 노드의 id 규칙을 따른다** — 같은 수정 커밋을 도구가 다시 읽어도
        // 출처·생산자가 다르므로 다른 노드로 선다(F22-2).
        id: DerivedId::compute(
            "Defect",
            &manifest_ids.iter().copied().map(NodeRef::Symbol).collect::<Vec<_>>(),
            pal_core::Provenance::Extracted,
            &pal_core::Producer::Extractor,
            &ReproInput::History { base: repro_base, head: fix },
        ),
        description: meta.summary.clone(),
        manifests_at: manifest_ids,
        introduced_by,
        resolved_by: change_id,
        at: at.clone(),
    };

    let touched = manifests.values().flatten().map(|(id, _)| *id).collect();
    Ok(report(&meta, &at, Retrobinding::Bound(Box::new(defect)), touched, not_built))
}

/// 수정 커밋에서 **의미가 변한 심볼** — 발현 좌표.
///
/// **새로 생긴 심볼은 발현이 아니다.** 고치기 전에 존재하지 않았으므로 결함이 드러난
/// 자리가 아니다. 사라진 심볼은 발현이다 — 버그가 있던 코드를 지운 것이 수정일 수 있다.
fn manifestations(
    git: &GixRepo,
    repo_id: &pal_core::RepoId,
    before: &TreeRef,
    here: &TreeRef,
    files: &[&RepoPath],
) -> Result<BTreeMap<RepoPath, Vec<(SymbolId, BodyDigest)>>> {
    let mut out: BTreeMap<RepoPath, Vec<(SymbolId, BodyDigest)>> = BTreeMap::new();
    for path in files {
        let was = digests_at(git, repo_id, before, path)?;
        let now = digests_at(git, repo_id, here, path)?;
        for (id, old) in &was {
            if now.get(id).is_none_or(|new| new != old) {
                out.entry((*path).clone()).or_default().push((*id, *old));
            }
        }
    }
    Ok(out)
}

/// 각 발현 좌표가 **마지막으로 변한 조상**을 찾고, 그 최빈을 도입 변경으로 본다.
///
/// **표를 던지지 못한 좌표는 분모에도 없다** — 예산에 걸린 것은 동의도 반대도 아니다.
/// 그 사실은 `confidence.total` 이 발현 좌표 수보다 작다는 것으로 드러난다.
fn introduction(
    git: &GixRepo,
    repo_id: &pal_core::RepoId,
    parent: ObjectName,
    manifests: &BTreeMap<RepoPath, Vec<(SymbolId, BodyDigest)>>,
    budget: usize,
) -> Result<Introduction> {
    let mut votes: BTreeMap<ObjectName, usize> = BTreeMap::new();
    let mut walked_max = 0usize;

    for (path, symbols) in manifests {
        let ancestors = git.first_parent_walk(parent, budget)?;
        walked_max = walked_max.max(ancestors.len());
        // 파일 하나를 한 번만 거슬러 올라간다 — 좌표마다 이력을 되풀이하지 않는다.
        let mut previous = parent;
        let mut pending: BTreeMap<SymbolId, BodyDigest> = symbols.iter().copied().collect();
        for &ancestor in ancestors.iter().skip(1) {
            if pending.is_empty() {
                break;
            }
            let there = digests_at(git, repo_id, &TreeRef::Committed(ancestor), path)?;
            let settled: Vec<SymbolId> = pending
                .iter()
                .filter(|(id, want)| there.get(*id).is_none_or(|d| d != *want))
                .map(|(id, _)| *id)
                .collect();
            for id in settled {
                pending.remove(&id);
                *votes.entry(previous).or_insert(0) += 1;
            }
            previous = ancestor;
        }
    }

    let cast: usize = votes.values().sum();
    if cast == 0 {
        return Ok(Introduction::NotFound {
            reason: NotFoundReason::HistoryBudget { walked: walked_max },
        });
    }
    // 동수면 이름이 작은 쪽 — **결정적이어야 한다.**
    let (&top, &agreeing) = votes
        .iter()
        .max_by_key(|(id, n)| (**n, std::cmp::Reverse(**id)))
        .expect("비어 있지 않다");
    Ok(Introduction::Found {
        change: ChangeId::new(top.to_hex()),
        confidence: Confidence::new(agreeing, cast),
        // **최빈이 아닌 후보를 버리지 않는다** — 조용한 절단 금지(stack §5.4).
        others: votes
            .keys()
            .filter(|id| **id != top)
            .map(|id| ChangeId::new(id.to_hex()))
            .collect(),
    })
}

fn report(
    meta: &pal_git::CommitMeta,
    at: &Snapshot,
    result: Retrobinding,
    touches: Vec<SymbolId>,
    not_extracted: Vec<(RepoPath, CapabilityId)>,
) -> DefectReport {
    let change = Change {
        id: ChangeId::new(meta.id.to_hex()),
        kind: ChangeKind::Commit,
        summary: meta.summary.clone(),
        author: pal_core::ActorId::new(meta.author_id.clone()),
        touches,
        parents: meta.parents.iter().map(|p| ChangeId::new(p.to_hex())).collect(),
        at: at.clone(),
    };
    let summary = RetrobindingSummary::of(std::slice::from_ref(&result));
    DefectReport { change, result, summary, not_extracted }
}

/// 두 트리를 대조해 blob 이 달라진 경로 전부. **추가·삭제도 변한 것이다.**
fn changed_files(
    git: &GixRepo,
    before: &TreeRef,
    here: &TreeRef,
) -> Result<Vec<RepoPath>> {
    let a: BTreeMap<RepoPath, ObjectName> = git.list_tree(before)?.into_iter().collect();
    let b: BTreeMap<RepoPath, ObjectName> = git.list_tree(here)?.into_iter().collect();
    let mut out: BTreeSet<RepoPath> = BTreeSet::new();
    for (p, o) in &a {
        if b.get(p) != Some(o) {
            out.insert(p.clone());
        }
    }
    for p in b.keys() {
        if !a.contains_key(p) {
            out.insert(p.clone());
        }
    }
    Ok(out.into_iter().collect())
}

/// 그 커밋의 그 파일에서 `symbol_id → body_digest`. **파일이 없으면 빈 표다.**
fn digests_at(
    git: &GixRepo,
    repo: &pal_core::RepoId,
    at: &TreeRef,
    path: &RepoPath,
) -> Result<BTreeMap<SymbolId, BodyDigest>> {
    let Some(blob) = git.path_at(at, path)? else { return Ok(BTreeMap::new()) };
    let Some(language) = Language::from_extension(path.extension()) else {
        return Ok(BTreeMap::new());
    };
    let source = git.read_blob(blob)?;
    // **여기 도달할 때는 추출기가 있다** — 부르는 쪽이 `pal_extract::capability` 로 이미
    // 걸렀다. 여기서 `NotBuilt` 를 빈 표로 바꾸면 "안 만들었음"이 "없음"이 되고,
    // F22-3 의 첫 실행이 정확히 그 상태였다.
    //
    // **`extract` 가 아니라 레지스트리를 직접 탄다** — 전자는 `Vec<Symbol>` 만 내고
    // 포함 관계를 버린다. 버리면 여기서 계산하는 좌표가 **대장이 계산하는 좌표와
    // 달라진다**(F03-1). 두 경로가 다른 `symbol_id` 를 내면 결함 계보는 조용히 아무것도
    // 못 찾고, 그 침묵이 *"결함이 없다"* 처럼 보인다.
    let pal_core::Capable::Present(extractor) = pal_extract::extractor_for(language) else {
        return Ok(BTreeMap::new());
    };
    let Ok(graph) = extractor.extract(&source) else { return Ok(BTreeMap::new()) };
    Ok(ledger::nodes_of(repo, path, &graph.symbols, &graph.contains)
        .into_iter()
        .map(|n| (n.id, n.body))
        .collect())
}

fn hex20(hex: &str) -> Option<[u8; 20]> {
    if hex.len() != 40 {
        return None;
    }
    let mut out = [0u8; 20];
    for (i, b) in out.iter_mut().enumerate() {
        *b = u8::from_str_radix(hex.get(i * 2..i * 2 + 2)?, 16).ok()?;
    }
    Some(out)
}

/// 사람이 읽는 화면.
pub fn print(report: &DefectReport) {
    println!();
    println!("  {}  {}", report.change.id.short(), report.change.summary);
    println!("  저자   {}", report.change.author.as_str());
    println!();
    match &report.result {
        Retrobinding::Bound(d) => {
            println!("■ 발현 좌표 ({})", d.manifests_at.len());
            for s in d.manifests_at.iter().take(10) {
                println!("    {}", s.short());
            }
            if d.manifests_at.len() > 10 {
                println!("    … 외 {}개", d.manifests_at.len() - 10);
            }
            println!();
            print!("■ 도입 변경  ");
            match &d.introduced_by {
                Introduction::Found { change, confidence, others } => {
                    println!(
                        "{}  ({}/{} = {}%{})",
                        change.short(),
                        confidence.agreeing,
                        confidence.total,
                        confidence.percent(),
                        if confidence.is_strict_majority() {
                            ""
                        } else if confidence.is_majority() {
                            " · 과반이지만 엄격 과반은 아니다"
                        } else {
                            " · 과반이 아니다"
                        }
                    );
                    if !others.is_empty() {
                        println!("    나머지 후보 {}건 — 버리지 않는다: {}", others.len(),
                                 others.iter().map(ChangeId::short).collect::<Vec<_>>().join(" "));
                    }
                }
                Introduction::NotFound { reason } => println!("지목 못 함 — {reason:?}"),
            }
        }
        Retrobinding::Missed(u) => {
            println!("■ **담기지 않았다** — 세어서 표시한다");
            match u {
                Uncapturable::NoExtractor { capability, files, .. } => {
                    println!(
                        "    이 빌드에 그 언어의 추출기가 없습니다 — {} 미구축 ({})",
                        capability.feature, capability.what
                    );
                    println!("    **변한 것이 없다는 뜻이 아닙니다.** 읽지 못했다는 뜻입니다.");
                    for f in files.iter().take(8) {
                        println!("      {f}");
                    }
                    if files.len() > 8 {
                        println!("      … 외 {}개", files.len() - 8);
                    }
                }
                Uncapturable::OutsideCode { files, .. } => {
                    println!("    결함이 코드 밖에 있다. 좌표를 세울 수 있는 파일이 0개다.");
                    for f in files.iter().take(8) {
                        println!("      {f}");
                    }
                }
                Uncapturable::NoSemanticChange { .. } => {
                    println!("    코드는 변했는데 의미가 변한 심볼이 없다 (포매팅·주석·신규 파일)");
                }
            }
        }
    }
    if !report.not_extracted.is_empty() {
        println!();
        println!("■ 읽지 못한 파일 ({}) — **답이 부분적입니다**", report.not_extracted.len());
        for (p, c) in report.not_extracted.iter().take(5) {
            println!("    {p}  ({} 미구축)", c.feature);
        }
    }
    println!();
}

/// JSON 산출.
///
/// # Errors
/// 직렬화에 실패하면.
pub fn print_json(report: &DefectReport) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(report)?);
    Ok(())
}

/// 기본 예산.
#[must_use]
pub const fn default_budget() -> usize {
    PROVISIONAL_HISTORY_BUDGET
}
