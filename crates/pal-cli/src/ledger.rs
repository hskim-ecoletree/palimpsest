//! `pal ledger` — 저장소 하나의 관측 범위 대장.
//!
//! **첫 화면이 답이 아니라 대장인 이유**(how-it-works §2.2): 대부분의 도구는 곧바로
//! *"심볼 21,904개를 찾았습니다"* 라고 말한다. 그 말은 참이지만 듣는 사람은 그것이
//! **전부**인지 **찾은 것뿐**인지 알 수 없다.
//!
//! # 조립만 한다
//!
//! git 접근은 `pal-git`, 분류는 `pal-extract`, 캐시는 `pal-store` 다. 표면은 그 셋을
//! 잇고 사람이 읽는 표로 낸다 — **정책이 여기 있으면 안 된다.**

use std::collections::BTreeMap;
use std::num::NonZeroUsize;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use pal_core::{
    Attributes, Bucket, DetectorFreshness, Discriminator, ExtractGrade, FileState, IdentityGrade,
    LanguageCapability, LanguageId, Ledger, LedgerEntry, Manifest, RepoId, RepoPath, ScopeSource,
    Snapshot, SymbolId, SymbolNode, TreeRef, UnsupportedReason,
};
use pal_extract::{FileOutcome, OVERSIZE_BYTES};
use pal_git::{GitAccess, GixRepo, WorktreeState};
use pal_store::{BlobCache, CacheKey, CacheStats};
use rayon::prelude::*;
use serde::Serialize;

/// 한 번에 손에 드는 파일 수 — **메모리가 파일 수에 비례하지 않게 하는 값이다.**
///
/// 전부 읽고 전부 병렬로 돌리면 소스 바이트를 파일 수만큼 동시에 든다. 10⁵ 에서 그것이
/// 터진다(F02 §4). 이 값이 곧 동시 상주의 상한이고, **파일 수와 무관하다.**
///
/// **자리표시다** — 어느 측정도 이 숫자를 정하지 않았다. 확정은 예산 회귀(F05)의 것이다.
const EXTRACT_CHUNK: usize = 256;

/// 대장 + 그것을 만드는 데 든 캐시 회계.
///
/// **둘을 한 구조에 담되 분리한다.** 합격선이 *"두 회차의 대장 산출이 같아야 한다"* 인데
/// 캐시 수는 당연히 다르므로(1회차 미스, 2회차 적중) 비교 대상이 갈려야 한다
/// (`corpus/criteria.toml` `[s1.pass]`).
#[derive(Debug, Serialize)]
pub struct LedgerReport {
    pub ledger: Ledger,
    pub cache: CacheStats,
    /// 2층에 들어갈 심볼들. **표에는 안 나오고 `pal touch` 가 쓴다.**
    #[serde(skip)]
    pub symbols: Vec<SymbolNode>,
    /// 지금 워킹트리 — **`--at` 으로 과거를 보더라도 잰다.**
    ///
    /// *"이 답이 선 트리가 지금 워킹트리와 같은가"* 는 어느 트리를 보든 답에 실려야
    /// 하는 사실이고([`Envelope`] 의 `projection.matches_worktree`), 그것을 재려면
    /// 워킹트리를 봐야 한다.
    ///
    /// [`Envelope`]: pal_core::Envelope
    pub worktree: WorktreeState,
}

/// 대장을 계산한다.
///
/// # Errors
/// 저장소를 열지 못하거나, 트리를 읽지 못하거나, 추출기가 고장났으면.
pub fn compute(
    repo_path: &Path,
    rev: Option<&str>,
    cache_dir: Option<PathBuf>,
) -> Result<LedgerReport> {
    let repo = GixRepo::open(repo_path)
        .with_context(|| format!("git 저장소가 아니다: {}", repo_path.display()))?;

    // **워킹트리를 언제나 잰다.** `--at` 으로 과거를 보더라도 *"지금 워킹트리가 그것과
    // 같은가"* 는 답에 실려야 한다 — 그것이 `matches_worktree` 이고, 그 자리는 지금까지
    // `NotBuilt{F01}` 로 비어 있었다.
    let worktree = repo.worktree_state().context("워킹트리를 읽지 못했다")?;

    let tree = match rev {
        Some(r) => TreeRef::Committed(
            repo.resolve_commit(r).with_context(|| format!("가리키는 것이 없다: {r}"))?,
        ),
        // **기본이 워킹트리다** (F01 §3.2 · [R-06]). 이 제품의 1순위 사용 장면(적시 제시)은
        // 커밋 전 순간에 일어나고, 그 순간에 HEAD 를 보여주면 사용자가 방금 고친 것이
        // 답에서 사라진다. **커밋 축은 `--at` 을 준 사람이 명시적으로 고르는 것이다.**
        None => worktree.tree_ref(),
    };

    // **범위는 선언에서 온다** (DESIGN §4.3). 없으면 없다고 적는다 — 조용히 추정하면
    // `asserted` 와 추정이 같아 보인다.
    let manifest = load_manifest(repo_path)?;
    // `.gitattributes` — 언어 인식 ③ 단계가 읽는다(F01 §3.3). blob 이름 계산에 쓰는
    // 것과 **같은 파일이고 같은 파서**다(`pal-git` 이 clean 필터에 쓴다).
    let attributes = read_attributes(&repo, &tree)?;

    let cache_root = cache_dir.unwrap_or_else(|| repo_path.join(".palimpsest/cache"));
    let cache = BlobCache::open(cache_root).context("캐시를 열지 못했다")?;
    let version = pal_extract::version();

    let mut excluded: BTreeMap<RepoPath, pal_core::ExclusionRuleId> = BTreeMap::new();
    let mut files = repo.list_tree(&tree).context("트리를 읽지 못했다")?;
    // **정렬은 여기서 한다.** 산출이 결정적이어야 두 회차를 바이트로 비교할 수 있다.
    files.sort_by(|a, b| a.0.cmp(&b.0));

    // **저장소 식별자는 선언이 정본이다** ([R-08]). 경로에서 유도한 이름은 저장소를
    // 옮기면 바뀌고, 그러면 결박이 가리키는 좌표가 통째로 흔들린다.
    let repo_id = manifest
        .as_ref()
        .and_then(|m| m.repos.first())
        .map_or_else(|| RepoId::new(repo_name(repo_path)), |r| r.id.clone());
    let mut stats = CacheStats::default();
    let mut entries = Vec::with_capacity(files.len());
    let mut symbols: Vec<SymbolNode> = Vec::new();

    // **덩어리 하나씩 — 읽기는 직렬, 추출은 병렬**(F02 §3.6 · `[f02.4]`).
    //
    // # 왜 통째로 병렬이 아닌가
    //
    // git 객체 읽기가 직렬로 남는다. `gix::Repository` 는 `!Sync` 이고(객체 캐시에 내부
    // 가변성이 있다) 그것을 스레드마다 여는 것은 `pal-git` 의 표면을 바꾸는 일이라
    // [R-15](저장 기술이 밖으로 새지 않는다)를 건드린다. **비싼 쪽은 파싱이다** —
    // 그것을 병렬로 돌린다.
    //
    // # 왜 덩어리인가 — **이것이 `[f02.4.pass]` ⑤ 다**
    //
    // 전부 읽고 전부 병렬로 돌리면 소스 바이트를 파일 수만큼 동시에 든다. 10⁵ 에서
    // 그것이 터진다(F02 §4). 덩어리로 끊으면 **동시 상주가 덩어리 크기에 비례하고
    // 파일 수와 무관하다** — 트리도 마찬가지로 `FileGraph` 로 바뀌는 즉시 버려진다.
    //
    // # 순서가 결정적이다
    //
    // 덩어리 안에서 `par_iter` 가 어떤 순서로 끝나든 결과를 **입력 순서 그대로** 모은다
    // (rayon 의 `map`+`collect` 가 그것을 보장한다). 완료 순서로 모으면 `symbol_id` 가
    // 회차마다 움직이고 결박이 조용히 `orphaned` 가 된다.
    for chunk in files.chunks(EXTRACT_CHUNK) {
        // ① 직렬 — 캐시를 보고, 미스면 소스를 읽는다.
        let mut pending: Vec<(usize, Vec<u8>, Option<String>)> = Vec::new();
        let mut outcomes: Vec<Option<FileOutcome>> = Vec::with_capacity(chunk.len());
        for (i, (path, blob)) in chunk.iter().enumerate() {
            if let Some(rule) = manifest.as_ref().and_then(|m| m.excluded_by(&repo_id, path)) {
                excluded.insert(path.clone(), rule.id.clone());
                outcomes.push(None);
                continue;
            }
            let declared = attributes.of(path).language;
            let key = CacheKey::new(*blob, version, path, declared.as_deref());
            if let Some(hit) = cache.get::<FileOutcome>(&key)? {
                stats.hit();
                outcomes.push(Some(hit));
                continue;
            }
            stats.miss();
            // **워킹트리 파일은 객체 저장소에 없을 수 있다** — 아직 커밋되지 않았으면
            // 그 blob 이름으로 조회가 실패한다. 읽는 곳이 트리에 따라 갈린다.
            let source = if tree.is_committed() {
                repo.read_blob(*blob).with_context(|| format!("{path}"))?
            } else {
                repo.read_worktree_file(path).with_context(|| format!("{path}"))?
            };
            pending.push((i, source, declared));
            outcomes.push(None);
        }

        // ② 병렬 — 분류·추출. **파일 간 의존이 없으므로 완전 병렬이다.**
        let fresh: Vec<Result<FileOutcome>> = pending
            .par_iter()
            .map(|(i, source, declared)| {
                let path = &chunk[*i].0;
                pal_extract::classify(path, source, OVERSIZE_BYTES, declared.as_deref())
                    .with_context(|| format!("분류 실패: {path}"))
            })
            .collect();

        // ③ 직렬 — 캐시에 넣고 입력 순서로 되꽂는다.
        for ((i, source, declared), outcome) in pending.into_iter().zip(fresh) {
            let outcome = outcome?;
            let (path, blob) = &chunk[i];
            let key = CacheKey::new(*blob, version, path, declared.as_deref());
            cache.put(&key, &outcome)?;
            drop(source); // 소스도 트리와 함께 버린다 — 덩어리 밖으로 들고 가지 않는다
            outcomes[i] = Some(outcome);
        }

        for ((path, _), outcome) in chunk.iter().zip(outcomes) {
            let Some(outcome) = outcome else {
                // **제외는 파일을 읽기 전에 판정된다.** 규칙에 걸린 파일은 내용을 보지
                // 않고, 그래서 캐시도 건드리지 않는다 — 범위 밖은 "보지 않음"이다.
                let rule = excluded.remove(path).expect("제외되지 않았는데 산출이 없다");
                entries.push(LedgerEntry { path: path.clone(), state: FileState::Excluded { rule } });
                continue;
            };
            symbols.extend(nodes_of(&repo_id, path, &outcome.symbols));
            entries.push(LedgerEntry { path: path.clone(), state: outcome.state });
        }
    }

    let languages = language_capabilities(&entries);
    let ledger = Ledger {
        snapshot: Snapshot::single(repo_id, tree),
        // **선언된 저장소 수** — 매니페스트가 있으면 그것이 세고, 없으면 1 이다.
        // 멀티레포 스티칭은 F14 이고, 여기서 2 이상이 되어도 **보는 것은 여전히 하나**다.
        // 그 차이가 §4.3 이 말한 뿌리의 공백이고 대장이 두 수를 나란히 적는다.
        repos_declared: manifest
            .as_ref()
            .and_then(|m| NonZeroUsize::new(m.repos.len()))
            .unwrap_or_else(|| NonZeroUsize::new(1).expect("1 은 0 이 아니다")),
        entries,
        languages,
        scope: manifest.as_ref().map_or(ScopeSource::InferredFromPath, |m| ScopeSource::Declared {
            repos: m.repos.len(),
            rules: m.rule_count(),
        }),
        detector: DetectorFreshness {
            grammar: version.grammar.to_owned(),
            extractor: version.extractor.to_owned(),
            head_now: worktree.base,
        },
    };
    Ok(LedgerReport { ledger, cache: stats, symbols, worktree })
}

/// 파일 하나의 심볼들에 좌표를 붙인다.
///
/// # `ordinal` 을 여기서 센다 ([R-16])
///
/// 같은 (이름, 종류)가 한 파일에 여럿이면 **선언 순서**로 가른다. 그러면 순서가 바뀌는
/// 것만으로 정체성이 뒤바뀌므로, 그런 심볼은 정체성 등급이 `Ordinal` 로 묶인다 —
/// [`Discriminator::identity_ceiling`] 이 그것을 강제한다.
pub(crate) fn nodes_of(repo: &RepoId, path: &RepoPath, symbols: &[pal_core::Symbol]) -> Vec<SymbolNode> {
    let mut seen: BTreeMap<(&str, &str), u32> = BTreeMap::new();
    let mut out = Vec::with_capacity(symbols.len());
    for s in symbols {
        let slot = seen.entry((s.name.as_str(), s.kind.name())).or_insert(0);
        let discriminator = Discriminator::new(s.kind, *slot);
        *slot += 1;

        out.push(SymbolNode {
            id: SymbolId::compute(repo, path, &[], &s.name, &discriminator),
            path: path.clone(),
            name: s.name.clone(),
            kind: s.kind,
            body: s.body,
            span: s.span,
            // 언어 등급이 아니라 **심볼**의 것이다 — R-22. 둘 중 낮은 쪽을 쓴다.
            //
            // **상한이 둘이다.** `Discriminator` 의 것(같은 이름·종류가 여럿이면 순서로
            // 가르므로 `ordinal`)과 추출기가 잰 것(`Symbol::identity` — 스코프 해소가
            // 실패하면 `ordinal`). 어느 하나라도 못 미치면 못 미친다.
            //
            // 옛 코드는 뒤쪽을 `ExtractGrade::L1` 로 **박아 두었다.** 그러면 추출기가
            // 무엇을 재든 대장은 언제나 `ordinal` 이고, 심볼 단위 실측이 대장에 닿지
            // 못한다(#48 · `[f02.3.pass]` ②).
            identity: discriminator.identity_ceiling().min(s.identity),
        });
    }
    out
}

/// 매니페스트를 읽는다. **없는 것과 깨진 것은 다르다.**
///
/// 없으면 `None` 이고 대장이 [`ScopeSource::InferredFromPath`] 를 싣는다. **깨졌으면
/// 오류다** — 잘못 쓴 매니페스트를 없는 것으로 삼키면 사용자가 선언한 제외 규칙이
/// 조용히 안 걸리고, 대장은 그것을 *"제외 0 건"* 으로 낸다.
fn load_manifest(repo_path: &Path) -> Result<Option<Manifest>> {
    let file = repo_path.join(".palimpsest/manifest.toml");
    let text = match std::fs::read_to_string(&file) {
        Ok(t) => t,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(e) => return Err(anyhow::anyhow!("{}: {e}", file.display())),
    };
    Manifest::parse(&text)
        .map(Some)
        .map_err(|e| anyhow::anyhow!("{}: {e}", file.display()))
}

/// 이 트리의 `.gitattributes` 들.
///
/// **커밋을 보고 있으면 그 커밋의 것을 읽는다.** 워킹트리의 파일을 읽으면 과거 대장이
/// 지금 설정으로 계산되고, 그러면 같은 커밋의 대장이 시점마다 달라진다.
fn read_attributes(repo: &GixRepo, at: &TreeRef) -> Result<Attributes> {
    let files = repo.list_tree(at).context("트리를 읽지 못했다")?;
    let mut found = Vec::new();
    for (path, blob) in files {
        let Some(dir) = path.as_str().strip_suffix(".gitattributes") else { continue };
        let dir = dir.trim_end_matches('/').to_owned();
        let raw = if at.is_committed() {
            repo.read_blob(blob).with_context(|| format!("{path}"))?
        } else {
            repo.read_worktree_file(&path).with_context(|| format!("{path}"))?
        };
        // 읽을 수 없는 바이트는 규칙이 아니다 — 손실 변환으로 넘긴다.
        found.push((dir, String::from_utf8_lossy(&raw).into_owned()));
    }
    Ok(Attributes::parse(&found))
}

/// 디렉터리 이름을 저장소 식별자로 쓴다.
///
/// **매니페스트가 없을 때만 쓴다** (2026-08-13 · F01). 정본은 매니페스트가 선언하는
/// 안정 식별자다(F01 §3.5 · [R-08]).
///
/// **경로에서 유도한 이름은 저장소를 옮기면 바뀐다.** 그것이 R-08 이 경고한 형태이고,
/// 그래서 이 경로로 왔다는 사실이 [`ScopeSource::InferredFromPath`] 로 산출에 실린다 —
/// 임시방편을 쓰는 것보다 **임시방편을 쓴다고 말하지 않는 것**이 나쁘다.
pub(crate) fn repo_name(path: &Path) -> String {
    path.canonicalize()
        .ok()
        .as_deref()
        .and_then(Path::file_name)
        .map_or_else(|| "?".to_owned(), |n| n.to_string_lossy().into_owned())
}

/// 언어별 능력 표. **파일 수 내림차순, 동수면 이름순** — 결정적이어야 한다.
fn language_capabilities(entries: &[LedgerEntry]) -> Vec<LanguageCapability> {
    let mut by_language: BTreeMap<LanguageId, (ExtractGrade, usize)> = BTreeMap::new();
    for e in entries {
        let (language, grade) = match &e.state {
            FileState::Parsed { language, grade } | FileState::Partial { language, grade, .. } => {
                (language.clone(), *grade)
            }
            // **읽지 못한 파일의 등급은 L0 이다.** 그 파일에서 아무것도 못 뽑으므로
            // 심볼 정체성이 없고, 대장 머리에 "결박 불가"로 선다(DESIGN §4.1).
            //
            // 이유 둘을 여기서 가르지 않는다 — 등급은 **그 파일에서 무엇을 뽑았는가**의
            // 함수이고 둘 다 0 이다. 이유가 갈리는 곳은 아래 `bucket_note` 다.
            FileState::Unsupported { language, .. } => (language.clone(), ExtractGrade::L0),
            _ => continue,
        };
        let slot = by_language.entry(language).or_insert((grade, 0));
        slot.0 = slot.0.max(grade);
        slot.1 += 1;
    }

    let mut out: Vec<LanguageCapability> = by_language
        .into_iter()
        .map(|(language, (grade, files))| LanguageCapability {
            language,
            grade,
            identity: grade.identity(),
            files,
        })
        .collect();
    out.sort_by(|a, b| b.files.cmp(&a.files).then_with(|| a.language.cmp(&b.language)));
    out
}

/// how-it-works §2.2 의 화면.
/// 좌표를 붙인 심볼을 **한 줄에 하나씩** 낸다 — F03 §6.3 의 골든이 읽는 표면.
///
/// # 왜 줄 단위인가
///
/// 골든의 일은 *"추출기 버전을 올릴 때 얼마나 움직이는지가 보이는 것"* 이다(F03 §6.3).
/// 한 덩어리 JSON 으로 내면 심볼 하나가 움직여도 전체가 달라 보이고, 그러면 골든이
/// 답하는 것은 *"움직였는가"* 이지 *"무엇이 움직였는가"* 가 아니다. **줄로 내면 `diff`
/// 가 곧 움직인 것의 목록이고**, `[f03.1.pass].on_failure` 이 요구하는 것이 그 목록이다.
///
/// **순서는 대장이 정한 순서 그대로다** — 경로 정렬 후 파일 안 선언 순서. 다시 정렬하지
/// 않는다. 정렬을 여기서 또 하면 대장의 순서가 결정적이라는 사실이 이 표면에서 안 보인다.
///
/// # Errors
/// 직렬화가 실패하면.
pub fn print_symbols(report: &LedgerReport) -> Result<()> {
    let mut out = String::new();
    for s in &report.symbols {
        out.push_str(&serde_json::to_string(s)?);
        out.push('\n');
    }
    print!("{out}");
    Ok(())
}

pub fn print_table(report: &LedgerReport) {
    let l = &report.ledger;
    let counts = l.counts();

    println!();
    // **집합의 모든 트리를 검사한다** — 하나만 보고 "(커밋)" 이라 적으면 나머지가 감춰진다.
    let 전부_커밋 = l.snapshot.entries().all(|(_, t)| t.is_committed());
    println!("Snapshot  {}  {}", l.snapshot,
             if 전부_커밋 { "(커밋)" } else { "(워킹트리)" });
    // **선언된 것과 본 것을 나란히 적는다** — 그 차이가 §4.3 이 말한 뿌리의 공백이다.
    println!("저장소    선언 {} · 본 것 {}", l.repos_declared, l.snapshot.len());
    // **범위가 어디서 왔는가.** 선언과 추정이 같아 보이면 `asserted` 가 뜻을 잃는다.
    println!("범위      {}", l.scope.describe());
    println!();
    // **워킹트리를 언제나 적는다.** 커밋을 보고 있어도 *"지금 워킹트리가 그것과
    // 같은가"* 는 사용자가 알아야 하는 사실이다 — 다르면 지금 화면이 방금 고친 것을
    // 담고 있지 않다는 뜻이다.
    let w = &report.worktree;
    let dirty = w.dirty_paths.len();
    println!(
        "워킹트리  {}  ·  인덱스 신뢰 {} · 다시 잼 {}",
        if dirty == 0 {
            format!("{} 와 같음", &w.base.to_hex()[..7])
        } else {
            format!("{} 와 다른 파일 {dirty}개", &w.base.to_hex()[..7])
        },
        w.trusted_from_index,
        w.rehashed
    );
    println!();
    println!("파일      {}", l.total());

    // **일곱 칸을 전부 낸다. 0 도 낸다** — 생략하면 "그 칸이 없다"와 "0 건"이 같아진다.
    for b in Bucket::ALL {
        let n = counts.get(&b).copied().unwrap_or(0);
        let note = match b {
            // **이유를 뭉개지 않는다.** *"추출기 없음"* 은 로드맵의 자리이고
            // *"문법이 못 읽음"* 은 문법의 자리다. 한 줄로 적으면 사용자가 고칠 곳을
            // 로드맵에서 찾는다 — `UnsupportedReason` 이 존재하는 이유다(#47).
            Bucket::Unsupported if n > 0 => {
                let defeated = l
                    .entries
                    .iter()
                    .filter(|e| {
                        matches!(
                            &e.state,
                            FileState::Unsupported {
                                reason: UnsupportedReason::GrammarDefeated { .. },
                                ..
                            }
                        )
                    })
                    .count();
                if defeated == 0 {
                    "  추출기 없음(로드맵)".to_owned()
                } else {
                    format!(
                        "  추출기 없음(로드맵) {} · 문법이 못 읽음 {defeated}",
                        n - defeated
                    )
                }
            }
            Bucket::Unrecognized if n > 0 => "  언어 미인식".to_owned(),
            Bucket::Partial if n > 0 => "  회복 지점 기록됨".to_owned(),
            Bucket::Excluded if n > 0 => {
                let rules: Vec<String> = l
                    .exclusions_by_rule()
                    .into_iter()
                    .map(|(r, c)| format!("{r}({c})"))
                    .collect();
                format!("  규칙: {}", rules.join(" · "))
            }
            _ => String::new(),
        };
        println!("  {:<16}{:>6}{note}", b.name(), n);
    }

    println!();
    if l.languages.is_empty() {
        println!("언어      (없음)");
    } else {
        for (i, c) in l.languages.iter().enumerate() {
            let head = if i == 0 { "언어    " } else { "        " };
            let identity = match c.identity {
                IdentityGrade::Unavailable => "결박 불가".to_owned(),
                g => format!("identity: {}", g.name()),
            };
            println!("{head}  {:<18}{:<4}{:<14}{:>6} 파일", c.language.as_str(), c.grade.name(), identity, c.files);
        }
        let unbindable = l.unbindable_languages();
        if !unbindable.is_empty() {
            let n: usize = unbindable.iter().map(|c| c.files).sum();
            println!();
            println!("          ← 결박 불가 언어 {}개 · {n} 파일. 이 파일들에는 좌표가 없습니다",
                     unbindable.len());
        }
    }

    println!();
    println!("캐시      적중 {} · 빗나감 {}", report.cache.hits, report.cache.misses);

    // **아직 만들지 않은 것을 빈 값으로 내지 않는다** — stack §5.3.
    println!();
    println!("provider  (이 빌드에 provider 포트가 없습니다 — F21 미구축)");
    println!("조달      (이 빌드에 관측 수용이 없습니다 — F16 미구축)");
    // **낡음을 재는 자의 낡음** — F01 이 이 줄을 값으로 바꿨다.
    println!(
        "감지기    추출기 {} · 문법 {} · HEAD {}",
        &l.detector.extractor,
        &l.detector.grammar[..7.min(l.detector.grammar.len())],
        if l.head_moved() {
            format!("{} 로 움직였습니다 — 이 대장은 그 뒤를 보지 않았습니다",
                    &l.detector.head_now.to_hex()[..7])
        } else {
            "그대로".to_owned()
        }
    );
    println!();
}
