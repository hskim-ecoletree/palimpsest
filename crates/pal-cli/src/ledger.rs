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
    Bucket, Discriminator, ExtractGrade, FileState, IdentityGrade, LanguageCapability, LanguageId,
    Ledger, LedgerEntry, RepoId, RepoPath, Snapshot, SymbolId, SymbolNode, TreeRef,
};
use pal_extract::{FileOutcome, OVERSIZE_BYTES};
use pal_git::{GitAccess, GixRepo};
use pal_store::{BlobCache, CacheKey, CacheStats};
use serde::Serialize;

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

    let commit = match rev {
        Some(r) => repo.resolve_commit(r).with_context(|| format!("가리키는 것이 없다: {r}"))?,
        None => repo.head().context("HEAD 를 읽지 못했다")?,
    };
    // **S1 은 커밋 축만 돈다.** `TreeRef::Worktree` 는 타입으로 서 있고 F01 이 채운다.
    let tree = TreeRef::Committed(commit);

    let cache_root = cache_dir.unwrap_or_else(|| repo_path.join(".palimpsest/cache"));
    let cache = BlobCache::open(cache_root).context("캐시를 열지 못했다")?;
    let version = pal_extract::version();

    let mut files = repo.list_tree(&tree).context("트리를 읽지 못했다")?;
    // **정렬은 여기서 한다.** 산출이 결정적이어야 두 회차를 바이트로 비교할 수 있다.
    files.sort_by(|a, b| a.0.cmp(&b.0));

    let repo_id = RepoId::new(repo_name(repo_path));
    let mut stats = CacheStats::default();
    let mut entries = Vec::with_capacity(files.len());
    let mut symbols: Vec<SymbolNode> = Vec::new();

    for (path, blob) in files {
        let key = CacheKey::new(blob, version);
        let outcome: FileOutcome = if let Some(hit) = cache.get::<FileOutcome>(&key)? {
            stats.hit();
            hit
        } else {
            stats.miss();
            let source = repo.read_blob(blob).with_context(|| format!("{path}"))?;
            let fresh = pal_extract::classify(&path, &source, OVERSIZE_BYTES)
                .with_context(|| format!("분류 실패: {path}"))?;
            cache.put(&key, &fresh)?;
            fresh
        };
        symbols.extend(nodes_of(&repo_id, &path, &outcome.symbols));
        entries.push(LedgerEntry { path, state: outcome.state });
    }

    let languages = language_capabilities(&entries);
    let ledger = Ledger {
        snapshot: Snapshot::single(repo_id, tree),
        // **선언된 저장소 수.** S1 은 언제나 1 이다 — 멀티레포는 F14.
        repos_declared: NonZeroUsize::new(1).expect("1 은 0 이 아니다"),
        entries,
        languages,
    };
    Ok(LedgerReport { ledger, cache: stats, symbols })
}

/// 파일 하나의 심볼들에 좌표를 붙인다.
///
/// # `ordinal` 을 여기서 센다 ([R-16])
///
/// 같은 (이름, 종류)가 한 파일에 여럿이면 **선언 순서**로 가른다. 그러면 순서가 바뀌는
/// 것만으로 정체성이 뒤바뀌므로, 그런 심볼은 정체성 등급이 `Ordinal` 로 묶인다 —
/// [`Discriminator::identity_ceiling`] 이 그것을 강제한다.
fn nodes_of(repo: &RepoId, path: &RepoPath, symbols: &[pal_core::Symbol]) -> Vec<SymbolNode> {
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
            identity: discriminator.identity_ceiling().min(ExtractGrade::L1.identity()),
        });
    }
    out
}

/// 디렉터리 이름을 저장소 식별자로 쓴다.
///
/// **임시방편이다.** 정본은 매니페스트가 선언하는 안정 식별자이고(F01 §3.5, R-08),
/// 그 로딩은 TOML 파서를 요구해 P0 의존 목록에 아직 없다. **경로에서 유도한 이름은
/// 저장소를 옮기면 바뀐다** — 그것이 R-08 이 경고한 바로 그 형태이므로 F01 이 고친다.
fn repo_name(path: &Path) -> String {
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
            // **추출기가 없는 언어의 등급은 L0 이다.** 그 언어에서 아무것도 못 뽑으므로
            // 심볼 정체성이 없고, 대장 머리에 "결박 불가"로 선다(DESIGN §4.1).
            FileState::Unsupported { language } => (language.clone(), ExtractGrade::L0),
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
    println!();
    println!("파일      {}", l.total());

    // **일곱 칸을 전부 낸다. 0 도 낸다** — 생략하면 "그 칸이 없다"와 "0 건"이 같아진다.
    for b in Bucket::ALL {
        let n = counts.get(&b).copied().unwrap_or(0);
        let note = match b {
            Bucket::Unsupported if n > 0 => "  언어 인식됨, 추출기 없음".to_owned(),
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
    println!("감지기    (이 빌드에 낡음 감지가 없습니다 — F01 미구축)");
    println!();
}
