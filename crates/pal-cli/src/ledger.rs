//! `pal ledger` — 저장소 하나의 관측 범위 대장.
//!
//! **첫 화면이 답이 아니라 대장인 이유**(옛 `how-it-works §2.2` · 2026-08-18 폐기): 대부분의 도구는 곧바로
//! *"심볼 21,904개를 찾았습니다"* 라고 말한다. 그 말은 참이지만 듣는 사람은 그것이
//! **전부**인지 **찾은 것뿐**인지 알 수 없다.
//!
//! # 조립만 한다
//!
//! git 접근은 `pal-git`, 분류는 `pal-extract`, 캐시는 `pal-store` 다. 표면은 그 셋을
//! 잇고 사람이 읽는 표로 제출한다 — **정책이 여기 있으면 안 된다.**

use std::collections::BTreeMap;
use std::num::NonZeroUsize;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use pal_core::{
    Attributes, Bucket, CORRUPT_NOTES, Containment, DetectorFreshness, Discriminator,
    EXTRACT_CHUNK, ExtractGrade, FileRow, FileState, IdentityGrade, OVERSIZE_BYTES, RefCounts,
    ReferenceEdge,
    Slot,
    LanguageCapability, LanguageId, Ledger, LedgerEntry, Manifest, RepoId, RepoPath,
    ScopeSource, Snapshot, SymbolId, SymbolNode, TreeRef, UnsupportedReason,
};
use pal_extract::FileOutcome;
use pal_git::{GitAccess, GixRepo, WorktreeState};
use pal_store::{BlobCache, CacheKey, CacheStats, ExtractCache as _, FileStitch, Lookup};
use rayon::prelude::*;
use serde::Serialize;

// **덩어리 크기와 화면 예산은 여기 없다.** `pal-core::budget` 한 곳이다
// (stack §5.5 · `[f05.1.pass]` ①).

/// 대장 + 그것을 만드는 데 든 캐시 회계.
///
/// **둘을 한 구조에 담되 분리한다.** 합격선이 *"두 회차의 대장 산출이 같아야 한다"* 인데
/// 캐시 수는 당연히 다르므로(1회차 미스, 2회차 적중) 비교 대상이 갈려야 한다
/// (`corpus/criteria.toml` `[s1.pass]`).
#[derive(Debug, Serialize)]
pub struct LedgerReport {
    pub ledger: Ledger,
    pub cache: CacheStats,
    /// 심볼 단위 정체성을 넷으로 가른 수 ([`IdentityTally`]).
    ///
    /// **`#[serde(skip)]` 이 아니다** — `pal ledger --json` 이 이 구조를 그대로 직렬화하고,
    /// `#79` 가 요구한 *"그 수를 산출하는 경로"* 가 그것이다.
    pub identity: IdentityTally,
    /// 2층에 들어갈 심볼들. **표에는 안 나오고 `pal touch` 가 쓴다.**
    #[serde(skip)]
    pub symbols: Vec<SymbolNode>,
    /// 2층에 들어갈 파일치 — **1패스 스티칭의 입력**(옛 F05 §4).
    ///
    /// **여기서 만들어지는 것이 요점이다.** 1층 캐시가 이미 `scopes`·`export_digest` 를
    /// 싣고 있으므로(F04) **재파싱 없이** 만들어진다 — 적중한 파일도 엣지를 산출한다.
    #[serde(skip)]
    pub stitches: Vec<FileStitch>,
    /// 깨져서 격리된 엔트리의 자리 몇 — **수는 `cache.corrupt` 가 전부 잰다.**
    ///
    /// 비어 있는 것이 정상 상태다. 비어 있지 않으면 화면에 뜬다.
    pub corrupt: Vec<String>,
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
        // **기본이 워킹트리다** (옛 F01 §3.2 · [R-06]). 이 제품의 1순위 사용 장면(적시 제시)은
        // 커밋 전 순간에 일어나고, 그 순간에 HEAD 를 보여주면 사용자가 방금 고친 것이
        // 답에서 사라진다. **커밋 축은 `--at` 을 준 사람이 명시적으로 고르는 것이다.**
        None => worktree.tree_ref(),
    };

    // **범위는 선언에서 온다** (옛 DESIGN §4.3). 없으면 없다고 적는다 — 조용히 추정하면
    // `asserted` 와 추정이 같아 보인다.
    let manifest = load_manifest(repo_path)?;
    // `.gitattributes` — 언어 인식 ③ 단계가 읽는다(옛 F01 §3.3). blob 이름 계산에 쓰는
    // 것과 **같은 파일이고 같은 파서**다(`pal-git` 이 clean 필터에 쓴다).
    let attributes = read_attributes(&repo, &tree)?;

    let cache_root = cache_dir.unwrap_or_else(|| repo_path.join(".palimpsest/cache"));
    let cache = BlobCache::open(cache_root).context("캐시를 열지 못했다")?;
    let version = pal_extract::version();
    // **이 빌드가 무슨 능력을 만드는가** — 키의 다섯째 성분(F04 · ADR-0004).
    // 한 번 재고 파일마다 다시 재지 않는다.
    let capabilities = pal_extract::capability_axis();

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
    // 엣지가 지는 **공통 넷의 넷째**다. 덩어리를 돌기 전에 한 번 만든다.
    let snapshot = Snapshot::single(repo_id.clone(), tree);
    let mut stats = CacheStats::default();
    let mut corrupt: Vec<String> = Vec::new();
    let mut entries = Vec::with_capacity(files.len());
    let mut symbols: Vec<SymbolNode> = Vec::new();
    let mut stitches: Vec<FileStitch> = Vec::new();
    // 심볼 단위 정체성 — 파일마다 `nodes_of` 가 돌려준 것을 더한다 ([#79]).
    let mut identity = IdentityTally::default();

    // **덩어리 하나씩 — 읽기는 직렬, 추출은 병렬**(옛 F02 §3.6 · `[f02.4]`).
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
    // 그것이 터진다(옛 F02 §4). 덩어리로 끊으면 **동시 상주가 덩어리 크기에 비례하고
    // 파일 수와 무관하다** — 트리도 마찬가지로 `FileGraph` 로 바뀌는 즉시 버려진다.
    //
    // # 순서가 결정적이다
    //
    // 덩어리 안에서 `par_iter` 가 어떤 순서로 끝나든 결과를 **입력 순서 그대로** 모은다
    // (rayon 의 `map`+`collect` 가 그것을 보장한다). 완료 순서로 모으면 `symbol_id` 가
    // 회차마다 움직이고 결박이 조용히 `orphaned` 가 된다.
    for chunk in files.chunks(EXTRACT_CHUNK) {
        // ① 직렬 — 캐시를 보고, 미스면 소스를 읽는다.
        let mut pending: Vec<(usize, Vec<u8>, pal_core::Declared<String>)> = Vec::new();
        let mut outcomes: Vec<Option<FileOutcome>> = Vec::with_capacity(chunk.len());
        for (i, (path, blob)) in chunk.iter().enumerate() {
            if let Some(rule) = manifest.as_ref().and_then(|m| m.excluded_by(&repo_id, path)) {
                excluded.insert(path.clone(), rule.id.clone());
                outcomes.push(None);
                continue;
            }
            let declared = attributes.of(path).language;
            let key = CacheKey::new(*blob, version, path, declared.as_deref(), capabilities);
            match cache.lookup::<FileOutcome>(&key)? {
                Lookup::Hit(hit) => {
                    stats.hit();
                    outcomes.push(Some(hit));
                    continue;
                }
                Lookup::Miss => stats.miss(),
                // **깨진 것은 미스가 아니다.** 진행하되(재계산) 조용하지 않다 —
                // 수가 산출에 실리고, 첫 건은 자리까지 적는다. 격리된 바이트는 남는다.
                Lookup::Corrupt { quarantined, cause } => {
                    stats.corrupt();
                    if corrupt.len() < CORRUPT_NOTES {
                        corrupt.push(format!("{path} — {cause} (격리: {})", quarantined.display()));
                    }
                }
            }
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
            let key = CacheKey::new(*blob, version, path, declared.as_deref(), capabilities);
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
            let (nodes, 파일치) = nodes_of(&repo_id, path, outcome.graph.symbols(), outcome.graph.contains());
            identity.합친다(파일치);
            if let Some(stitch) = stitch_of(&snapshot, path, &outcome.graph, &nodes) {
                stitches.push(stitch);
            }
            symbols.extend(nodes);
            entries.push(LedgerEntry { path: path.clone(), state: outcome.state });
        }
    }

    let ledger = assemble(repo_id, tree, manifest.as_ref(), entries, version, worktree.base);
    Ok(LedgerReport { ledger, cache: stats, identity, corrupt, symbols, stitches, worktree })
}

/// 센 것을 대장으로 조립한다. **정책이 없다** — 세는 일은 위에서 끝났다.
fn assemble(
    repo_id: RepoId,
    tree: TreeRef,
    manifest: Option<&Manifest>,
    entries: Vec<LedgerEntry>,
    version: pal_core::ExtractorVersion,
    head_now: pal_core::ObjectName,
) -> Ledger {
    let languages = language_capabilities(&entries);
    Ledger {
        snapshot: Snapshot::single(repo_id, tree),
        // **선언된 저장소 수** — 매니페스트가 있으면 그것이 세고, 없으면 1 이다.
        // 멀티레포 스티칭은 F14 이고, 여기서 2 이상이 되어도 **보는 것은 여전히 하나**다.
        // 그 차이가 §4.3 이 말한 뿌리의 공백이고 대장이 두 수를 나란히 적는다.
        repos_declared: manifest
            .and_then(|m| NonZeroUsize::new(m.repos.len()))
            .unwrap_or_else(|| NonZeroUsize::new(1).expect("1 은 0 이 아니다")),
        entries,
        languages,
        scope: manifest.map_or(ScopeSource::InferredFromPath, |m| ScopeSource::Declared {
            repos: m.repos.len(),
            rules: m.rule_count(),
        }),
        detector: DetectorFreshness {
            grammar: version.grammar.to_owned(),
            extractor: version.extractor.to_owned(),
            head_now,
        },
    }
}

/// 각 심볼의 **컨테이너 체인** — 바깥에서 안으로.
///
/// # 이것이 없으면 좌표가 `ordinal` 위에 세워진다
///
/// 옛 F03 §3.2 가 체인을 `symbol_id` 의 성분으로 적었다. 비워 두면 같은 파일의
/// `class A { m() {} }` 와 `class B { m() {} }` 가 **컨테이너가 아니라 선언 순서로만**
/// 갈리고, 클래스 순서를 바꾸는 것만으로 두 `m` 의 정체성이 맞바뀐다 —
/// [R-16] 이 경고한 조용한 재결박이다.
///
/// **깊이를 심볼 수로 막는다.** `contains` 에 순환이 있으면 이 순회가 멈추지 않는다.
/// 순환은 추출기의 결함이고 여기서 고칠 수 없지만, **좌표를 만드는 쪽이 멈추지 않는
/// 것**은 여기의 책임이다.
fn container_chains(symbols: &[pal_core::Symbol], contains: &[Containment]) -> Vec<Vec<String>> {
    let parent: BTreeMap<u32, u32> = contains.iter().map(|c| (c.child.0, c.parent.0)).collect();
    let mut out = Vec::with_capacity(symbols.len());
    for i in 0..symbols.len() {
        let mut chain = Vec::new();
        let mut cursor = u32::try_from(i).unwrap_or(u32::MAX);
        for _ in 0..symbols.len() {
            let Some(p) = parent.get(&cursor) else { break };
            let Some(s) = symbols.get(*p as usize) else { break };
            chain.push(s.name.clone());
            cursor = *p;
        }
        // 안에서 밖으로 걸었으므로 뒤집는다 — 체인은 **바깥에서 안으로**다.
        chain.reverse();
        out.push(chain);
    }
    out
}

/// 심볼 단위 정체성을 **넷으로 가른 수**. ([#79])
///
/// # 왜 필요한가 — `min` 이 두 모집단을 한 글자로 만든다
///
/// [`nodes_of`] 는 심볼의 정체성 등급을 `discriminator.identity_ceiling().min(s.identity)`
/// 로 산출한다. 소비자에게는 그 **낮은 쪽**이 맞는 값이지만, `ordinal` 이라는 한 글자가 두 가지
/// 서로 다른 사실을 덮는다:
///
/// - **순서에 취약하다** — 같은 (체인·이름·종류)가 여럿이라 **선언 순서**로 가렸다.
///   `impl` 순서를 바꾸면 정체성이 맞바뀐다([R-16] 의 조용한 재결박).
/// - **등급이 낮다** — 판별자가 순서로 **안 가린** 것이고, 추출기가 스코프를 못 풀어
///   `ordinal` 이다.
///
/// ⚠⚠ **한때 이 자리가 「둘째 버킷은 순서 위험이 없다」로 적혀 있었고 그것이 거짓이었다.**
/// `ordinal` 은 0 부터이므로 중복 그룹의 첫 선언도 `ordinal == 0` 인데,
/// [`SymbolId::compute`] 가 `discriminator.ordinal` 을 해시에 넣으므로
/// (`crates/pal-core/src/coord.rs`) 같은 그룹의 선언 **순서를 바꾸면 그 첫 선언의 정체성도
/// 맞바뀐다** — 같은 파일의 시험 `체인이_없으면_순서가_정체성을_흔든다` 가 그것을 고정한다.
/// **그래서 2026-09-12 에 가르는 술어가 그룹 크기로 바뀌었다**(아래 ★). 지금 둘째 버킷은
/// **이름이 안 겹친 것만** 담는다.
///
/// 앞의 것은 **고칠 수 있는 위험**이고 뒤의 것은 **언어·추출기의 한계**다. 한 글자로 덮으면
/// 어느 쪽이 몇인지 셀 수 없고, 그러면 *"손으로 센 수를 판정 표에 싣는"* 길만 남는다
/// (`#79` 가 닫는 조건으로 적은 자리다).
///
/// # 합이 분모와 같다
///
/// 넷은 **배타적이고 전체를 덮는다** — 먼저 **그 (체인·이름·종류) 그룹이 여럿인가**로
/// 가르고, 그렇지 않은 것을 추출기 등급으로 셋으로 나눈다. [`합`](Self::합) 이 심볼 수와
/// 다르면 버킷이 겹치거나 빠진 것이고, `pal ledger` 가 그 검산을 화면에 적는다.
///
/// # ★ 가르는 술어가 `ordinal > 0` 이 아니라 **그룹 크기**다 (2026-09-12)
///
/// 사전 등록(`plan/79-pre.md:105-110`)은 ① 을 `ordinal > 0` 으로 적었다. **그것은 참인
/// 셈법이 아니다** — [`SymbolId::compute`] 가 `discriminator.ordinal` 을 해시에 넣으므로
/// (`crates/pal-core/src/coord.rs:126-127`) 중복 그룹의 **첫 선언**도 선언 순서가 바뀌면
/// `SymbolId` 가 움직인다. 같은 파일의 `체인이_없으면_순서가_정체성을_흔든다` 가 그것을
/// 이미 고정하고 있다.
///
/// 정반합 라운드 1 판 1 의 반(反) R1·R6 이 잡았고 소유자가 **「그룹 단위로 다시 헤아린다」**를
/// 골랐다(`intent.md` `## 승격` 칸 1). 사전 등록은 **안 고친다** — 봉인(`A1-b`)이 걸려
/// 있고, 이 어긋남은 `C1` 의 차이로 싣는다.
///
/// [#79]: https://github.com/hskim-ecoletree/palimpsest/issues/79
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
pub struct IdentityTally {
    /// **그 (체인·이름·종류) 그룹이 둘 이상이다** — 선언 순서로 가렸다. 순서가 바뀌면
    /// 그룹 **전원**의 정체성이 맞바뀌므로 첫 선언도 여기 든다.
    pub 순서에_취약: usize,
    /// 그룹이 하나뿐인데 추출기 등급이 `ordinal` — 스코프를 못 푼 것이다.
    pub 등급이_낮음: usize,
    /// 그룹이 하나뿐이고 추출기 등급이 `exact` — 이름으로 유일하고 참조가 해소된다.
    pub 정확: usize,
    /// 그룹이 하나뿐이고 추출기 등급이 `unavailable`.
    pub 불가: usize,
}

impl IdentityTally {
    /// 심볼 하나를 헤아린다 — **가르는 규칙이 이 한 자리에만 있다.**
    ///
    /// `그룹_크기` 는 그 심볼과 (체인·이름·종류)가 같은 선언의 수이고 `grade` 는
    /// 추출기가 심볼 단위로 잰 등급이다. **둘을 여기서 처음이자 마지막으로 함께 본다.**
    ///
    /// ⚠ **`Discriminator::identity_ceiling` 을 안 받는다.** 그 값은 `ordinal == 0` 하나로
    /// `Exact` 를 내므로 중복 그룹의 첫 선언을 안 가른다 — 그것이 앞 판의 거짓이었다.
    fn 헤아린다(&mut self, 그룹_크기: usize, grade: IdentityGrade) {
        if 그룹_크기 > 1 {
            self.순서에_취약 += 1;
            return;
        }
        match grade {
            IdentityGrade::Exact => self.정확 += 1,
            IdentityGrade::Ordinal => self.등급이_낮음 += 1,
            IdentityGrade::Unavailable => self.불가 += 1,
        }
    }

    /// 넷을 더한 것 — **분모다.** 2층에 들어가는 심볼 수와 같아야 한다.
    #[must_use]
    pub const fn 합(&self) -> usize {
        self.순서에_취약 + self.등급이_낮음 + self.정확 + self.불가
    }

    /// 다른 파일치를 더한다 — 대장이 파일마다 부른다.
    fn 합친다(&mut self, 다른: Self) {
        self.순서에_취약 += 다른.순서에_취약;
        self.등급이_낮음 += 다른.등급이_낮음;
        self.정확 += 다른.정확;
        self.불가 += 다른.불가;
    }
}

/// 파일 하나의 심볼들에 좌표를 붙인다. **그리고 정체성 상한을 버리지 않고 세어 돌려준다.**
///
/// # `ordinal` 을 여기서 헤아린다 — **그리고 컨테이너마다 따로 헤아린다** ([R-16])
///
/// 같은 (컨테이너 체인, 이름, 종류)가 여럿이면 **선언 순서**로 가른다. 그러면 순서가
/// 바뀌는 것만으로 정체성이 뒤바뀌므로, 그런 심볼은 정체성 등급이 `Ordinal` 로 묶인다 —
/// [`Discriminator::identity_ceiling`] 이 그것을 강제한다.
///
/// **체인을 열쇠에 넣지 않으면 컨테이너를 성분으로 넣은 뜻이 절반 사라진다.**
/// `class A { m() {} } class B { m() {} }` 에서 둘째 `m` 이 `ordinal = 1` 을 받고,
/// 그러면 체인이 갈라 놓은 두 심볼이 **다시 순서에 묶인다** — 등급이 `Ordinal` 로
/// 떨어지므로 조용하지도 않다. 이 파일 안에서 그 이름이 유일하다는 사실이 좌표에
/// 실려야 한다.
pub(crate) fn nodes_of(
    repo: &RepoId,
    path: &RepoPath,
    symbols: &[pal_core::Symbol],
    contains: &[Containment],
) -> (Vec<SymbolNode>, IdentityTally) {
    let chains = container_chains(symbols, contains);
    // ★ **1 패스 — 그룹 크기를 먼저 구한다** (2026-09-12 · 정반합 판 1).
    //   버킷 ① 의 술어가 「그룹이 여럿인가」이므로 **첫 선언을 만날 때 그 답이 이미
    //   있어야 한다.** 한 패스로는 원리상 못 구한다 — 뒤에 같은 이름이 또 올지 모른다.
    let mut 그룹: BTreeMap<(&[String], &str, &str), usize> = BTreeMap::new();
    for (i, s) in symbols.iter().enumerate() {
        *그룹.entry((chains[i].as_slice(), s.name.as_str(), s.kind.name())).or_insert(0) += 1;
    }
    let mut seen: BTreeMap<(&[String], &str, &str), u32> = BTreeMap::new();
    let mut out = Vec::with_capacity(symbols.len());
    let mut tally = IdentityTally::default();
    for (i, s) in symbols.iter().enumerate() {
        let chain = &chains[i];
        let slot = seen.entry((chain.as_slice(), s.name.as_str(), s.kind.name())).or_insert(0);
        let discriminator = Discriminator::new(s.kind, *slot);
        *slot += 1;

        let chain_refs: Vec<&str> = chain.iter().map(String::as_str).collect();
        out.push(SymbolNode {
            id: SymbolId::compute(repo, path, &chain_refs, &s.name, &discriminator),
            path: path.clone(),
            container: chain.clone(),
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
        // ★ **버리지 않는다** ([#79]). 위의 `min` 은 소비자가 쓰는 값이라 그대로 두고,
        //   합치기 전의 두 값을 여기서 헤아린다. 이것이 없으면 「순서에 취약」과 「등급이 낮음」을
        //   가르는 수가 **어느 명령으로도 안 난다.**
        let 그룹_크기 = 그룹[&(chain.as_slice(), s.name.as_str(), s.kind.name())];
        tally.헤아린다(그룹_크기, s.identity);
    }
    (out, tally)
}

/// 파일 하나치의 2층 입력 — **1패스가 파일마다 만드는 것**(옛 F05 §4).
///
/// # 그래프가 없으면 파일 노드도 없다
///
/// 이진·생성물·범위 밖·미인식·미지원은 **추출의 대상이 아니었다.** 2층에 빈 노드를
/// 세우면 *"참조가 없는 파일"* 처럼 보이고, 그 파일들이 왜 없는지는 **대장이 싣는다** —
/// 같은 사실을 두 곳에 적으면 그것이 곧 drift 다.
///
/// # `EXPORTS` 는 **유일하게 해소되는 최상위 이름만** 담는다
///
/// `ExportSet.names` 는 이름이고 2층의 열쇠는 좌표다. 같은 이름의 최상위 선언이 둘이면
/// 어느 것을 내보내는지 **파일 하나만 보고 알 수 없고**(재선언·오버로드), 하나를 고르면
/// 그것이 조용한 오답이다. `export * from` 과 기본 내보내기도 이름이 아니라 여기 없다 —
/// 그 둘의 해소는 F07 이다.
fn stitch_of(
    snapshot: &Snapshot,
    path: &RepoPath,
    graph: &pal_extract::Extraction,
    nodes: &[SymbolNode],
) -> Option<FileStitch> {
    let g = graph.graph()?;

    // **임포트 항목을 함께 넘긴다.** `Slot::NotBuilt` 인 언어는 빈 슬라이스를 받고,
    // 그러면 `file_edges` 가 임포트 갈래를 안 세운다 — *"항목 축을 안 만든다"* 와
    // *"임포트가 0 건이다"* 가 여기서 갈린다. 뭉개면 Kotlin 처럼 항목 축이 없는
    // 언어의 파일이 *"밖에서 이름을 하나도 안 쓴다"* 로 나간다.
    let 임포트: &[pal_core::ImportedItem] = match &g.imports {
        Slot::Built(set) => match &set.items {
            Slot::Built(items) => items.as_slice(),
            Slot::NotBuilt => &[],
        },
        Slot::NotBuilt => &[],
    };

    // 스코프 체인이 있으면 엣지와 갈래별 건수가 나온다. 없으면 **0 이 아니라 안 만듦**.
    let (edges, refs, pending) = match &g.scopes {
        Slot::Built(chain) => {
            let out = pal_core::file_edges(&g.symbols, nodes, chain, 임포트, snapshot);
            (out.edges, Slot::Built(out.counts), out.pending)
        }
        Slot::NotBuilt => {
            (Vec::<ReferenceEdge>::new(), Slot::<RefCounts>::NotBuilt, Vec::new())
        }
    };

    let exports = match &g.exports {
        Slot::NotBuilt => Vec::new(),
        Slot::Built(set) => set
            .names
            .iter()
            .filter_map(|name| {
                // ★ **이 한 줄이 `ExportSet` 의 모집단을 정한다** (2026-09-08 · #130).
                //   `n.container.is_empty()` 라 **최상위만** EXPORTS 로 옮겨진다. 그래서
                //   Rust 추출기도 `pub` 을 **최상위 + 정확히 `pub`** 으로 좁혔다
                //   (`rust.rs` 의 `표면` · 조건 `A13`). 중첩 `pub` 을 담으면
                //   `export_digest` 는 움직이는데 EXPORTS 는 안 서서 **두 값이 서로 다른
                //   모집단을 재고, 그 어긋남이 화면 어디에도 안 나온다.**
                let mut hit = nodes.iter().filter(|n| n.container.is_empty() && &n.name == name);
                let first = hit.next()?;
                // **둘 이상이면 담지 않는다.** 하나를 고르면 그것이 조용한 오답이다.
                if hit.next().is_some() { None } else { Some((name.clone(), first.id)) }
            })
            .collect(),
    };

    Some(FileStitch {
        file: FileRow {
            path: path.clone(),
            language: g.language.clone(),
            grade: g.grade,
            export_digest: g.export_digest.clone(),
            refs,
        },
        symbols: nodes.to_vec(),
        exports,
        edges,
        imports: g.imports.clone(),
        pending,
    })
}

/// 매니페스트를 읽는다. **없는 것과 깨진 것은 다르다.**
///
/// 없으면 `None` 이고 대장이 [`ScopeSource::InferredFromPath`] 를 싣는다. **깨졌으면
/// 오류다** — 잘못 쓴 매니페스트를 없는 것으로 삼키면 사용자가 선언한 제외 규칙이
/// 조용히 안 걸리고, 대장은 그것을 *"제외 0 건"* 으로 산출한다.
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
/// 안정 식별자다(옛 F01 §3.5 · [R-08]).
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
            // 심볼 정체성이 없고, 대장 머리에 "결박 불가"로 잡힌다(옛 DESIGN §4.1).
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

/// 옛 `how-it-works §2.2` 의 화면 (그 문서는 2026-08-18 에 지웠다 — `docs/plan/disposal-map.md`).
/// 좌표를 붙인 심볼을 **한 줄에 하나씩** 산출한다 — 옛 F03 §6.3 의 골든이 읽는 표면.
///
/// # 왜 줄 단위인가
///
/// 골든의 일은 *"추출기 버전을 올릴 때 얼마나 움직이는지가 보이는 것"* 이다(옛 F03 §6.3).
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

/// 캐시 회계 — 적중·빗나감, 그리고 **사건이 있으면** 그것.
fn print_cache(report: &LedgerReport) {
    println!("캐시      적중 {} · 빗나감 {}", report.cache.hits, report.cache.misses);
    print_corrupt(report);
}

/// 깨져서 격리된 것 — **0 이면 한 줄도 안 적는다.**
///
/// 손상은 사건이고 사건이 없는 것이 정상 상태다. 늘 적으면 `Finding 0` 이 되고,
/// 그것이 이 도구가 고발하는 형태다.
fn print_corrupt(report: &LedgerReport) {
    if report.cache.corrupt == 0 {
        return;
    }
    println!("          ⚠ 깨져서 격리 {} 건", report.cache.corrupt);
    for note in &report.corrupt {
        println!("            {note}");
    }
    if report.cache.corrupt > report.corrupt.len() {
        println!("            … 그 밖 {} 건", report.cache.corrupt - report.corrupt.len());
    }
}

/// 심볼 단위 정체성 넷 — **언어 단위 등급 아래에 따로 적는다.** ([#79])
///
/// 위의 「언어」 블록은 `Rust L1 ordinal` 처럼 **언어**의 등급을 적는다. 그 줄만 보면
/// *"Rust 심볼은 다 `ordinal` 이다"* 로 읽히고, 그 안에서 **순서에 취약한 것**과
/// **등급이 낮은 것**이 몇인지는 알 수 없다. 이 블록이 그 둘을 가른다.
///
/// **검산을 함께 적는다** — 넷의 합과 2층에 들어가는 심볼 수가 같아야 한다. 다르면
/// 버킷이 겹치거나 빠진 것이고, 그때 화면은 조용히 틀린 수를 싣는다.
fn print_identity(report: &LedgerReport) {
    let t = &report.identity;
    println!();
    if t.합() == 0 {
        // **0 을 침묵으로 두지 않는다** — 심볼이 없는 것과 이 블록이 없는 것은 다르다.
        println!("정체성    (심볼 0 — 좌표를 받은 선언이 없습니다)");
        return;
    }
    println!("정체성    심볼 {}", t.합());
    println!("  {:<16}{:>6}  {}", "순서에 취약", t.순서에_취약,
             "같은 이름·종류가 여럿 · 선언 순서로 가렸습니다 — 순서가 바뀌면 정체성이 맞바뀝니다");
    // ★ **그룹 전원이다 — 초과분이 아니다** (2026-09-12 · 정반합 판 1 · 소유자 결정).
    //   앞 판은 `ordinal > 0` 만 세고 *"그룹마다 첫 선언은 빠집니다"* 를 화면에 적었다.
    //   첫 선언도 순서가 바뀌면 `SymbolId` 가 움직이므로 그 셈법이 참이 아니었다.
    println!("  {:<16}{:>6}  {}", "", "", "↑ **겹친 자리의 심볼 전원**입니다(첫 선언 포함) — `#79` 본문의 `ordinal>0` 보다 큽니다");
    println!("  {:<16}{:>6}  {}", "등급이 낮음", t.등급이_낮음,
             "이름이 겹치지 않았고 추출기가 스코프를 못 풀었습니다");
    println!("  {:<16}{:>6}  {}", "정확", t.정확, "이름이 겹치지 않았고 참조가 해소됩니다");
    println!("  {:<16}{:>6}  {}", "불가", t.불가, "좌표를 세울 수 없습니다");
    // ⚠ **여기서 두 수를 나란히 적는다.** 합이 심볼 수와 다르면 사람이 바로 본다.
    println!("          ← 검산 {} + {} + {} + {} = {} · 2층에 들어가는 심볼 {}",
             t.순서에_취약, t.등급이_낮음, t.정확, t.불가, t.합(), report.symbols.len());
    if t.합() != report.symbols.len() {
        println!("          ⚠ **합이 심볼 수와 다릅니다** — 버킷이 겹치거나 빠졌습니다");
    }
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

    // **일곱 칸을 전부 출력한다. 0 도 출력한다** — 생략하면 "그 칸이 없다"와 "0 건"이 같아진다.
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
        // **고정폭 열에는 원 표기만 넣는다** — `{:<N}` 은 char 수로 채우고 한글은
        // 터미널에서 두 열을 먹어 정렬이 어긋난다. 병기는 열 뒤에 붙인다(ADR-0033 §3).
        println!("  {:<16}{:>6}  {}{note}", b.name(), n, crate::label::파일_상태(b).사용자_언어);
    }

    println!();
    if l.languages.is_empty() {
        println!("언어      (없음)");
    } else {
        for (i, c) in l.languages.iter().enumerate() {
            let head = if i == 0 { "언어    " } else { "        " };
            let identity = c.identity.name();
            // 같은 규칙이다 — 고정폭 열은 원 표기, 병기는 줄 끝. 앞 판은 `{:<14}` 안에
            // 「결박 불가」를 넣어 그 열의 정렬이 이미 어긋나 있었다.
            println!(
                "{head}  {:<18}{:<4}{:<14}{:>6} 파일  ·  {} · {}",
                c.language.as_str(),
                c.grade.name(),
                identity,
                c.files,
                crate::label::추출_등급(c.grade).사용자_언어,
                crate::label::정체성_등급(c.identity).사용자_언어,
            );
        }
        let unbindable = l.unbindable_languages();
        if !unbindable.is_empty() {
            let n: usize = unbindable.iter().map(|c| c.files).sum();
            println!();
            println!("          ← 결박 불가 언어 {}개 · {n} 파일. 이 파일들에는 좌표가 없습니다",
                     unbindable.len());
        }
    }

    print_identity(report);

    println!();
    print_cache(report);

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

#[cfg(test)]
mod tests {
    use super::*;
    use pal_core::IdentityGrade;
    use pal_core::{BodyDigest, LocalIx, Span, Symbol, SymbolKind};

    fn 심볼(name: &str, kind: SymbolKind, at: usize) -> Symbol {
        Symbol {
            name: name.to_owned(),
            kind,
            span: Span { byte_start: at, byte_end: at + 1, line_start: 1, line_end: 1 },
            body: BodyDigest::of_normalized(name.as_bytes()),
            identity: IdentityGrade::Exact,
        }
    }

    /// `class A { m() {} } class B { m() {} }` — 자리 0·1·2·3.
    fn 두_클래스() -> (Vec<Symbol>, Vec<Containment>) {
        (
            vec![
                심볼("A", SymbolKind::Class, 0),
                심볼("m", SymbolKind::Method, 10),
                심볼("B", SymbolKind::Class, 20),
                심볼("m", SymbolKind::Method, 30),
            ],
            vec![
                Containment { parent: LocalIx(0), child: LocalIx(1) },
                Containment { parent: LocalIx(2), child: LocalIx(3) },
            ],
        )
    }

    fn 좌표(symbols: &[Symbol], contains: &[Containment]) -> Vec<SymbolNode> {
        nodes_of(&RepoId::new("r"), &RepoPath::new("a.ts"), symbols, contains).0
    }

    /// 추출기 등급을 심볼마다 지정한 판 — `IdentityTally` 는 **두 값을 함께** 본다.
    fn 등급_심볼(name: &str, at: usize, grade: IdentityGrade) -> Symbol {
        let mut s = 심볼(name, SymbolKind::Function, at);
        s.identity = grade;
        s
    }

    fn 세어_본다(symbols: &[Symbol]) -> (Vec<SymbolNode>, IdentityTally) {
        nodes_of(&RepoId::new("r"), &RepoPath::new("a.rs"), symbols, &[])
    }

    /// ★ **RED 는 이것이었다** ([#79]) — `min` 이 합친 뒤에는 세 심볼이 **같은 글자**다.
    ///
    /// 같은 이름·종류 둘(`dup`)과 유일한 하나(`solo`)를 한 파일에 둔다. 추출기 등급은
    /// 셋 다 `ordinal`(Rust 의 L1)이다. 합친 값(`SymbolNode::identity`)으로는 **셋을
    /// 가를 수가 없고**, tally 는 `순서에 취약 2 · 등급이 낮음 1` 로 가른다.
    ///
    /// ★ **`dup` 둘 다 「취약」이다** (2026-09-12) — 순서를 바꾸면 첫 선언의 `SymbolId` 도
    /// 움직인다(`체인이_없으면_순서가_정체성을_흔든다` 가 같은 파일에서 그것을 고정한다).
    /// 그래서 이 수는 **겹친 자리의 심볼 전원**이고 `#79` 본문의 `ordinal>0` 보다 크다.
    #[test]
    fn 순서로_가린_것과_등급이_낮은_것이_갈린다() {
        let symbols = vec![
            등급_심볼("dup", 0, IdentityGrade::Ordinal),
            등급_심볼("dup", 10, IdentityGrade::Ordinal),
            등급_심볼("solo", 20, IdentityGrade::Ordinal),
        ];
        let (nodes, tally) = 세어_본다(&symbols);

        // ① 합친 값은 셋을 못 가른다 — 그것이 이 이슈가 말하는 「같은 글자」다.
        assert!(
            nodes.iter().all(|n| n.identity == IdentityGrade::Ordinal),
            "합친 값이 이미 갈려 있다면 이 시험은 아무것도 안 잰다"
        );
        // ② tally 는 가른다.
        assert_eq!(tally.순서에_취약, 2, "`dup` 둘 다 — 첫 선언도 순서가 바뀌면 정체성이 맞바뀐다");
        assert_eq!(tally.등급이_낮음, 1, "이름이 안 겹친 `solo` 하나만 이 버킷이다");
        assert_eq!(tally.정확, 0);
        assert_eq!(tally.불가, 0);
    }

    /// 넷의 합이 **분모**와 같다 — 다르면 버킷이 겹치거나 빠진 것이고, 그때 화면은
    /// 조용히 틀린 수를 싣는다. `#79` 가 *"손으로 센 수를 판정 표에 싣지 마라"* 라고
    /// 적은 자리를 검산으로 막는다.
    #[test]
    fn 합이_분모와_같다() {
        let symbols = vec![
            등급_심볼("dup", 0, IdentityGrade::Ordinal),
            등급_심볼("dup", 10, IdentityGrade::Unavailable),
            등급_심볼("solo", 20, IdentityGrade::Exact),
            등급_심볼("또", 30, IdentityGrade::Ordinal),
        ];
        let (nodes, tally) = 세어_본다(&symbols);
        assert_eq!(tally.합(), nodes.len(), "합이 심볼 수와 다르다");
        assert_eq!(tally.합(), 4);
        // 등급이 섞여도 넷이 배타적이다 — `dup` 둘은 등급이 무엇이든
        // **그룹이 여럿인 것**이 먼저 이긴다.
        assert_eq!(tally.순서에_취약, 2);
        assert_eq!(tally.정확, 1);
        assert_eq!(tally.등급이_낮음, 1);
        assert_eq!(tally.불가, 0, "`dup` 둘이 `순서에_취약` 으로 갔다");

        // 심볼이 없으면 0 — 화면은 그때 「심볼 0」을 적고 침묵하지 않는다.
        let (빈, 빈_tally) = 세어_본다(&[]);
        assert!(빈.is_empty());
        assert_eq!(빈_tally.합(), 0);
    }

    /// ⟨`MS-07` 을 닫는 시험⟩ ㉡ 의 산출이 준 **실물 입력**이다.
    ///
    /// `pal touch check_ledger_pair` 가 `identity ordinal` 을 찍었고(`touch/126.txt:17`),
    /// 그것만 보면 그 심볼이 **선언 순서에 취약한지** 알 수 없다. `grep -n
    /// "check_ledger_pair" xtask/src/main.rs` 로 확인한 사실은 **선언이 하나**라는 것이고,
    /// 그러면 그 (체인·이름·종류) 그룹의 크기가 1 이므로 이 심볼은 ②(등급이 낮음)여야 한다.
    ///
    /// **이 시험은 그 규칙을 재고, 실물 심볼이 그 규칙의 입력임을 위 두 사실이 잇는다.**
    /// 파일을 읽어 재지 않는 까닭은 그 파일이 회차마다 바뀌기 때문이다 — 바뀌는 것을
    /// 시험 입력으로 쓰면 시험이 무엇을 재는지가 회차마다 달라진다.
    /// ⚠ **짝을 함께 잰다.** 앞 판은 「하나면 ②」만 재서, 가르는 자리를 **끄면
    /// 전부 ②가 되므로 그때도 초록**이었다(사전 등록 §6 의 음성 대조 예상이 그래서
    /// 틀렸다 — 실측으로 확인했다). 같은 이름이 **둘일 때 ①이 되는 것**을 같은 시험에서
    /// 요구하면 그 구멍이 닫힌다.
    #[test]
    fn check_ledger_pair_는_순서에_취약하지_않다() {
        let symbols = vec![등급_심볼("check_ledger_pair", 0, IdentityGrade::Ordinal)];
        let (nodes, tally) = 세어_본다(&symbols);
        assert_eq!(nodes[0].identity, IdentityGrade::Ordinal, "합친 값은 여전히 `ordinal` 이다");
        assert_eq!(tally.순서에_취약, 0, "선언이 하나인데 순서로 가렸다고 셌다");
        assert_eq!(tally.등급이_낮음, 1, "추출기 등급 때문인 것을 그렇게 세지 않았다");

        // 짝 — 같은 이름이 둘이면 **둘 다** ①이다. 가르는 자리를 끄면 **이쪽이 빨개진다.**
        let 둘 = vec![
            등급_심볼("check_ledger_pair", 0, IdentityGrade::Ordinal),
            등급_심볼("check_ledger_pair", 10, IdentityGrade::Ordinal),
        ];
        let (_, 둘_tally) = 세어_본다(&둘);
        assert_eq!(둘_tally.순서에_취약, 2, "같은 이름 둘이면 첫 선언까지 전원이 ①이다");
        assert_eq!(둘_tally.등급이_낮음, 0, "겹친 자리인데 하나가 ②에 남았다");
    }

    #[test]
    fn 불변식_e_컨테이너가_같은_이름을_가른다() {
        let (s, c) = 두_클래스();
        let n = 좌표(&s, &c);
        assert_eq!(n[1].container, vec!["A".to_owned()]);
        assert_eq!(n[3].container, vec!["B".to_owned()]);
        assert_ne!(n[1].id, n[3].id, "서로 다른 클래스의 같은 이름 메서드가 한 좌표다");
    }

    #[test]
    fn 불변식_f_컨테이너_순서를_바꿔도_정체성이_그대로다() {
        // **★ 반대 방향이고 이 조각에서 가장 무겁다.** E 만 보면 컨테이너 대신
        // 선언 순서를 넣는 옛 코드도 통과한다 — 갈리는가만 물으면 **무엇으로**
        // 갈리는지는 안 물어진다. 순서가 정체성을 흔드는 것이 R-16 의 조용한 재결박이다.
        let (s, c) = 두_클래스();
        let 원래 = 좌표(&s, &c);

        // B 를 앞에, A 를 뒤에. 자리 번호가 통째로 바뀐다.
        let 뒤바꾼 = vec![
            심볼("B", SymbolKind::Class, 0),
            심볼("m", SymbolKind::Method, 10),
            심볼("A", SymbolKind::Class, 20),
            심볼("m", SymbolKind::Method, 30),
        ];
        let 뒤바꾼_포함 = vec![
            Containment { parent: LocalIx(0), child: LocalIx(1) },
            Containment { parent: LocalIx(2), child: LocalIx(3) },
        ];
        let 지금 = 좌표(&뒤바꾼, &뒤바꾼_포함);

        let 찾기 = |v: &[SymbolNode], 컨테이너: &str| {
            v.iter()
                .find(|n| n.name == "m" && n.container == vec![컨테이너.to_owned()])
                .expect("메서드를 못 찾았다")
                .id
        };
        assert_eq!(찾기(&원래, "A"), 찾기(&지금, "A"), "A.m 이 클래스 순서에 흔들렸다");
        assert_eq!(찾기(&원래, "B"), 찾기(&지금, "B"), "B.m 이 클래스 순서에 흔들렸다");
    }

    #[test]
    fn 체인이_없으면_순서가_정체성을_흔든다() {
        // **이 검사가 고장 났다면 어떻게 드러나는가** — 불변식 F 의 음성 대조다.
        // 포함 관계를 빼고 같은 심볼 목록을 넣으면 두 `m` 이 순서로만 갈리고,
        // 클래스를 맞바꾸면 **정체성이 서로 맞바뀐다.** F 가 없애는 것이 이것이고,
        // 이 시험이 통과하지 않으면 F 는 아무것도 안 재고 있는 것이다.
        let (s, _) = 두_클래스();
        let 원래 = 좌표(&s, &[]);
        let 뒤바꾼 = vec![
            심볼("B", SymbolKind::Class, 0),
            심볼("m", SymbolKind::Method, 10),
            심볼("A", SymbolKind::Class, 20),
            심볼("m", SymbolKind::Method, 30),
        ];
        let 지금 = 좌표(&뒤바꾼, &[]);
        // 자리 1 은 원래 `A.m`, 뒤바꾼 뒤에는 `B.m` 이다. **그런데 좌표가 같다.**
        assert_eq!(원래[1].id, 지금[1].id, "체인 없이도 순서가 정체성을 안 흔들었다면 F 는 무의미하다");
        assert_eq!(원래[3].id, 지금[3].id);
    }

    #[test]
    fn 컨테이너가_다르면_ordinal_이_다시_0_이다() {
        // 체인을 열쇠에 안 넣으면 둘째 `m` 이 `ordinal = 1` 을 받고, 체인이 갈라 놓은
        // 두 심볼이 **다시 순서에 묶인다** — 등급까지 `Ordinal` 로 떨어진다.
        let (s, c) = 두_클래스();
        let n = 좌표(&s, &c);
        assert_eq!(n[1].identity, IdentityGrade::Exact, "A.m 이 순서로 갈렸다");
        assert_eq!(n[3].identity, IdentityGrade::Exact, "B.m 이 순서로 갈렸다");
    }

    #[test]
    fn 같은_컨테이너의_오버로드는_여전히_순서로_갈린다() {
        // **컨테이너를 넣었다고 R-16 이 닫히지 않는다.** 같은 자리의 같은 이름은
        // 여전히 순서에 매이고, 등급이 그 사실을 싣는다.
        let s = vec![심볼("f", SymbolKind::Function, 0), 심볼("f", SymbolKind::Function, 10)];
        let n = 좌표(&s, &[]);
        assert_ne!(n[0].id, n[1].id);
        assert_eq!(n[0].identity, IdentityGrade::Exact);
        assert_eq!(n[1].identity, IdentityGrade::Ordinal, "순서로 가른 심볼이 exact 다");
    }

    #[test]
    fn 재선언은_한_좌표로_뭉개지지_않고_후보로_남는다() {
        // **정체성 규칙 ⑤** (옛 F03 §3.4) — *"같은 좌표에 둘 이상이면 후보 집합으로 저장"*.
        // 실물에서 그 「후보 집합」이 서는 방식은 **둘을 다 남기는 것**이다: 판별자가
        // 순서로 갈라 서로 다른 좌표를 주고, 이름으로 찾을 때 둘이 함께 나온다
        // (`Projection::resolve_name` → `TouchResult::Ambiguous`).
        //
        // 하나로 뭉개면 뒤에 선 선언이 앞의 것을 **조용히 덮고**, 그 순간 결박은
        // 자기가 무엇을 가리키는지 모른 채 살아 있게 된다.
        let s = vec![
            심볼("dup", SymbolKind::Variable, 0),
            심볼("dup", SymbolKind::Variable, 10),
        ];
        let n = 좌표(&s, &[]);
        assert_eq!(n.len(), 2, "재선언이 하나로 뭉개졌다");
        assert_ne!(n[0].id, n[1].id, "두 선언이 같은 좌표를 받았다");
    }

    #[test]
    fn 불변식_g_파일을_옮기면_정체성만_바뀐다() {
        // 이동은 *변경*이 아니라 *정체성 사건*이다 — 그 분리가 재결박 제안의 근거다(R-08).
        let (s, c) = 두_클래스();
        let 여기 = nodes_of(&RepoId::new("r"), &RepoPath::new("a.ts"), &s, &c).0;
        let 저기 = nodes_of(&RepoId::new("r"), &RepoPath::new("b/a.ts"), &s, &c).0;
        assert_ne!(여기[1].id, 저기[1].id, "옮겼는데 정체성이 그대로다");
        assert_eq!(여기[1].body, 저기[1].body, "옮겼는데 본문 요약이 움직였다");
    }

    #[test]
    fn 최상위_선언은_체인이_빈다() {
        // **빈 것이 정확한 값이다** — 담는 것이 없다.
        let s = vec![심볼("f", SymbolKind::Function, 0)];
        assert!(좌표(&s, &[])[0].container.is_empty());
    }

    #[test]
    fn 포함_관계에_순환이_있어도_멈춘다() {
        // 순환은 추출기의 결함이고 여기서 고칠 수 없다. **좌표를 만드는 쪽이 멈추지
        // 않는 것**은 여기의 책임이다.
        let s = vec![심볼("A", SymbolKind::Class, 0), 심볼("B", SymbolKind::Class, 10)];
        let c = vec![
            Containment { parent: LocalIx(0), child: LocalIx(1) },
            Containment { parent: LocalIx(1), child: LocalIx(0) },
        ];
        let n = 좌표(&s, &c);
        assert_eq!(n.len(), 2, "순환에서 좌표가 안 나왔다");
    }
}
