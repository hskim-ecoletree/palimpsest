//! git 접근 — **`gix` 가 이 크레이트 밖으로 새지 않는다.**
//!
//! # 왜 트레잇 뒤에 두는가 ([R-15])
//!
//! `gix` 는 API 가 아직 진화 중이다. 접촉면이 여러 크레이트에 퍼지면 상류가 시그니처를
//! 바꿀 때 고칠 자리가 한 곳이 아니게 되고, R-15 의 대응 *"깨지면 그 모듈만 고친다"* 가
//! 성립하지 않는다. 그래서 **`gix` 를 직접 쓰는 워크스페이스 크레이트는 여기 하나뿐이고,
//! 그 사실을 `cargo xtask check` 가 기계로 검사한다**(criteria `[s1.pass].gix_direct_dependents`).
//!
//! 최악의 경우 `git` CLI 호출 구현으로 대체할 수 있다. 트레잇이 그 자리를 미리 비워둔다.
//!
//! # 접촉면은 여섯이고, F01 §3.1 의 다섯과 같지 않다
//!
//! [옛 F01 §3.1](../../../docs/plan/disposal-map.md) 이 다섯을 적었고 그것은
//! **상한이지 하한이 아니다.** 실제로 선 것은 여섯이고 목록이 다르다:
//!
//! | F01 §3.1 | 여기 | |
//! |---|---|---|
//! | `head` · `list_tree` · `read_blob` | 있다 | S1 |
//! | `worktree_state` | **있다** | F01 — `TreeRef::Worktree` 를 만드는 자리 |
//! | `changed_between` | **있다** — `changed_in` (F10) | 소비자가 생겼다 (아래) |
//! | — | `commit` · `path_at` · `first_parent_walk` | F22-3 이 들였다 |
//!
//! **`changed_between` 이 오래 비어 있었고 그 근거는 *"소비자가 없다"* 였다** —
//! *"없는 소비자를 위한 것을 만들지 않는다. 그것이 곧 검사되지 않는 산출이다."*
//!
//! ★ **F10 이 그 소비자다.** 문서 §3.2 의 신호 넷째가 *"같은 커밋에서 함께 변경된
//! 파일 — git 이력에서 계산"* 이고, [`GitAccess::changed_in`] 이 그 자리다.
//! **모양이 `changed_between` 이 아니라 `changed_in` 인 것**도 소비자가 정했다 —
//! 인입이 묻는 것은 *"두 커밋 사이"* 가 아니라 *"이 커밋이 무엇을 함께 바꿨나"* 다.
//! 그리고 **first parent 와만 댄다** — 병합에서 갈래를 다 따라가면 같은 커밋이
//! 회차마다 다른 답을 내고, 그러면 조각의 후보가 흔들린다
//! ([`GitAccess::first_parent_walk`] 와 같은 근거).

#![forbid(unsafe_code)]

use std::path::Path;
use std::sync::OnceLock;

use pal_core::{Attributes, Digest, ObjectName, RepoPath, TreeRef};

#[derive(Debug, thiserror::Error)]
pub enum GitError {
    #[error("저장소를 열지 못했다: {0}")]
    Open(String),
    #[error("가리키는 것을 찾지 못했다: {0}")]
    Resolve(String),
    #[error("트리를 읽지 못했다: {0}")]
    Tree(String),
    #[error("blob 을 읽지 못했다: {0}")]
    Blob(String),
    #[error("HEAD 가 없다: {0}")]
    Head(String),
    /// **bare 저장소에는 워킹트리가 없다.** 없는 것을 0 으로 흉내 내지 않는다.
    #[error("워킹트리가 없다: {0}")]
    NoWorktree(String),
    #[error("워킹트리를 읽지 못했다: {0}")]
    Worktree(String),
}

/// git 에 닿는 유일한 문. **구현이 바뀌어도 이 모양은 남는다.**
pub trait GitAccess {
    /// # Errors
    /// HEAD 가 없거나(빈 저장소) 읽지 못하면.
    fn head(&self) -> Result<ObjectName, GitError>;

    /// 트리 하나의 추적 파일 전부. **정렬은 보장하지 않는다** — 세는 쪽이 정렬한다.
    ///
    /// [`TreeRef::Worktree`] 를 주면 **워킹트리를 센다.** 커밋 트리를 세고 마는 구현은
    /// 워킹트리 좌표를 조용히 커밋 좌표로 바꿔치기하는 것이다.
    ///
    /// # Errors
    /// 트리를 읽지 못하면.
    fn list_tree(&self, at: &TreeRef) -> Result<Vec<(RepoPath, ObjectName)>, GitError>;

    /// # Errors
    /// 그 이름의 객체가 없거나 blob 이 아니면.
    fn read_blob(&self, name: ObjectName) -> Result<Vec<u8>, GitError>;

    /// **커밋되지 않은 지금** — [`TreeRef::Worktree`] 가 여기서 나온다 (F01 §3.2).
    ///
    /// # 이것이 F01 이 내리는 가장 중요한 결정의 실행부다 ([R-06])
    ///
    /// 설계는 커밋을 시간축으로 삼았지만 이 제품의 1순위 사용 장면(적시 제시)은
    /// **커밋 전 순간**에 일어난다. 워킹트리에 좌표가 없으면 그 장면이 통째로 죽는다.
    ///
    /// # Errors
    /// 워킹트리가 없거나(bare) 인덱스·파일을 읽지 못하면.
    fn worktree_state(&self) -> Result<WorktreeState, GitError>;

    /// 워킹트리의 파일 하나를 읽는다.
    ///
    /// **[`GitAccess::read_blob`] 으로 대신할 수 없다.** 아직 커밋되지 않은 파일은
    /// 객체 저장소에 없다. 그리고 **심볼릭 링크의 내용은 링크 대상 문자열이다** —
    /// 그냥 열어서 읽으면 대상 파일의 내용이 읽히고, 그러면 내용이 blob 이름과
    /// 어긋난 채 그 이름으로 1층에 캐시된다.
    ///
    /// # Errors
    /// 워킹트리가 없거나 그 경로를 읽지 못하면.
    fn read_worktree_file(&self, path: &RepoPath) -> Result<Vec<u8>, GitError>;

    /// 커밋 하나의 메타 — **`Change` 노드가 여기서 나온다**(F22-3).
    ///
    /// # Errors
    /// 그런 커밋이 없거나 읽지 못하면.
    fn commit(&self, id: ObjectName) -> Result<CommitMeta, GitError>;

    /// 트리 안의 경로 하나. **없으면 그 커밋에 그 파일이 없었다는 뜻이다.**
    ///
    /// `list_tree` 로 대신하지 않는 이유: 소급 결박은 파일 **하나**를 여러 조상에서
    /// 되읽는다. 조상마다 트리 전체를 훑으면 그 비용이 이력 깊이에 곱해진다.
    ///
    /// # Errors
    /// 트리를 읽지 못하면. **경로가 없는 것은 오류가 아니다.**
    fn path_at(&self, at: &TreeRef, path: &RepoPath) -> Result<Option<ObjectName>, GitError>;

    /// first-parent 조상들 — 자신을 포함하고 `limit` 에서 멈춘다.
    ///
    /// **first-parent 인 것이 결정론의 조건이다.** 병합 커밋에서 갈래를 다 따라가면
    /// 순서가 구현에 의존하고, 같은 입력이 같은 답을 낸다는 배정 규칙 1 이 깨진다.
    ///
    /// # Errors
    /// 커밋을 읽지 못하면.
    fn first_parent_walk(
        &self,
        from: ObjectName,
        limit: usize,
    ) -> Result<Vec<ObjectName>, GitError>;

    /// 이 커밋이 **first parent 와 견주어** 바꾼 경로들 — F10 §3.2 의 넷째 신호.
    ///
    /// # 왜 `first parent` 하나와만 대는가
    ///
    /// [`GitAccess::first_parent_walk`] 와 **같은 근거다**: 병합 커밋에서 갈래를 다
    /// 따라가면 *"이 커밋이 무엇을 바꿨나"* 가 하나로 안 정해지고, 그러면 같은 입력이
    /// 같은 답을 낸다는 배정 규칙 1 이 깨진다. 문서 조각의 후보가 회차마다 흔들리면
    /// **거부 기록이 아무것도 안 가린다**(`[f10].queue_placement`).
    ///
    /// **부모가 없으면(최초 커밋) 그 트리 전부다** — 전부가 그 커밋에서 생겼다.
    ///
    /// # Errors
    /// 커밋이나 트리를 읽지 못하면.
    fn changed_in(&self, commit: ObjectName) -> Result<Vec<RepoPath>, GitError>;
}

/// 워킹트리 훑기의 결과 — 목록과 회계 둘.
type WorktreeScan = (Vec<(RepoPath, ObjectName)>, usize, usize);

/// 워킹트리의 지금 — **커밋되지 않은 것도 좌표를 갖는다** (F01 §3.2).
///
/// # 왜 이것이 공짜로 성립하는가
///
/// 1층 캐시 키가 `(blob 이름, 추출기 버전)` 이지 커밋이 아니다. 워킹트리 파일의 blob
/// 이름을 **git 과 똑같이** 계산하면 파싱 파이프라인은 커밋을 전혀 모른 채 그대로 돈다.
/// 커밋 축이 필요한 곳은 좌표 표기와 결박뿐이다.
///
/// **그래서 `blob 이름이 git 의 것과 같은가`가 이 타입의 유일한 외부 오라클이다**
/// (`git hash-object` · criteria `[f01.pass]` ①).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct WorktreeState {
    /// 이 워킹트리가 딛고 선 커밋.
    pub base: ObjectName,
    /// 추적 파일 `(경로, blob 이름)` 목록의 요약. **정렬된 목록에서 나온다.**
    pub tree_digest: Digest,
    /// `base` 의 트리와 다른 경로 — 추가 · 삭제 · 변경 전부.
    pub dirty_paths: Vec<RepoPath>,
    /// 인덱스의 `(mtime, size)` 를 믿고 넘어간 파일 수.
    ///
    /// **git 자신이 쓰는 방법이다**(F01 §3.2). 회계를 싣는 이유는 [`CacheStats`] 와
    /// 같다 — 믿은 것과 다시 잰 것이 구별되지 않으면 *"캐시가 항상 적중"* 이라고
    /// 거짓 보고하는 코드와 진짜가 같아 보인다.
    ///
    /// [`CacheStats`]: https://docs.rs/
    pub trusted_from_index: usize,
    /// stat 이 어긋나 실제로 다시 해시한 파일 수.
    pub rehashed: usize,
}

impl WorktreeState {
    /// 워킹트리가 `base` 의 트리와 같은가.
    #[must_use]
    pub fn matches_base(&self) -> bool {
        self.dirty_paths.is_empty()
    }

    /// **이 답이 선 트리가 지금 워킹트리와 같은가** — `Envelope.projection` 의 그 자리다.
    ///
    /// 커밋을 보고 있으면 *"그 커밋이 HEAD 이고 고친 것이 없는가"* 이고, 워킹트리를 보고
    /// 있으면 *"내가 잰 그 워킹트리인가"* 다. 뒤의 것이 자명해 보이지만 자명하지 않다 —
    /// 대장을 계산하는 동안 사용자가 파일을 고칠 수 있고, 그러면 요약이 달라진다.
    #[must_use]
    pub fn matches(&self, at: &TreeRef) -> bool {
        match at {
            TreeRef::Worktree { tree_digest, .. } => *tree_digest == self.tree_digest,
            TreeRef::Committed(c) => *c == self.base && self.dirty_paths.is_empty(),
        }
    }

    /// 이 상태의 [`TreeRef`].
    #[must_use]
    pub const fn tree_ref(&self) -> TreeRef {
        TreeRef::Worktree { base: self.base, tree_digest: self.tree_digest }
    }
}

/// 정렬된 `(경로, blob 이름)` 목록의 요약.
///
/// # 길이 접두사가 없으면 서로 다른 목록이 같은 요약을 낸다
///
/// `("ab", X), ("c", Y)` 와 `("a", X), ("bc", Y)` 는 바이트를 이어 붙이면 같아진다.
/// 그러면 이름 변경이 요약에 안 잡히고, 그것이 criteria `[f01.pass]` ③ 의 넷째 변이가
/// 세는 고장이다. `pal-core::derived` 의 `field()` 가 같은 이유로 같은 일을 한다.
///
/// **머클 트리가 아니다.** F01 §3.2 는 *"머클 루트"* 라고 적었지만 부분 재계산을 쓰는
/// 소비자가 없다 — 증분은 F05 의 것이다. 지금 필요한 것은 *"같은 목록 → 같은 요약,
/// 다른 목록 → 다른 요약"* 하나이고 순차 요약이 그것을 준다.
#[must_use]
pub fn digest_of(sorted: &[(RepoPath, ObjectName)]) -> Digest {
    let mut h = blake3::Hasher::new();
    h.update(b"palimpsest.worktree.v1");
    for (path, name) in sorted {
        let bytes = path.as_str().as_bytes();
        h.update(&(bytes.len() as u64).to_le_bytes());
        h.update(bytes);
        h.update(name.as_bytes());
    }
    Digest::from_bytes(*h.finalize().as_bytes())
}

/// 커밋 하나에서 읽어낸 것. **판정이 아니라 사실이다.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommitMeta {
    pub id: ObjectName,
    pub parents: Vec<ObjectName>,
    /// 커밋 메시지 첫 줄.
    pub summary: String,
    /// 저자의 안정 식별자 — 이메일. **표시 이름이 아니다**(같은 사람이 여러 이름을 쓴다).
    pub author_id: String,
    pub author_display: String,
    /// 저자 시각(에포크 초) — **표시용이다.**
    ///
    /// # 이 값은 앵커가 아니다 (F09 §6)
    ///
    /// 선행 구현은 커밋 시각을 낡음의 앵커로 썼고, 그러면 **포매팅 커밋에도 `stale` 이
    /// 켜진다** — [R-07] 이 치명이라 부른 실패다. `body_digest` 가 더 강하다.
    /// **다만 시각은 표시용으로 함께 싣는다** — *"3주 전 코드 기준"* 이 *"12커밋 전"*
    /// 보다 읽힌다. 그것이 [`pal_core::BoundTime`] 이다.
    ///
    /// **커미터가 아니라 저자다** — 리베이스가 커미터 시각을 오늘로 바꾸고, 그러면
    /// *"3주 전 코드"* 가 리베이스 한 번에 *"오늘"* 이 된다.
    pub epoch_secs: i64,
}

/// `gix` 구현. **이 타입 밖에서 `gix` 타입이 보이지 않는다.**
pub struct GixRepo {
    inner: gix::Repository,
    /// 워킹트리의 `.gitattributes` 들 — **한 번만 읽는다.**
    ///
    /// 파일마다 다시 읽으면 10⁵ 에서 그 비용이 파일 수에 곱해진다. 값이 이 실행 동안
    /// 바뀌지 않는다는 가정 위에 서고, 그 가정은 대장 계산이 한 순간의 스냅샷이라는
    /// 사실과 같은 것이다.
    attributes: OnceLock<Attributes>,
}

impl GixRepo {
    /// 저장소를 연다. 경로는 작업 디렉터리이거나 `.git` 이다.
    ///
    /// # Errors
    /// git 저장소가 아니면.
    pub fn open(path: &Path) -> Result<Self, GitError> {
        let inner =
            gix::open(path).map_err(|e| GitError::Open(format!("{}: {e}", path.display())))?;
        Ok(Self { inner, attributes: OnceLock::new() })
    }

    /// 사람이 쓴 것을 커밋 이름으로 푼다 — 짧은 SHA · 브랜치 · 태그 전부.
    ///
    /// **S1 의 코퍼스는 12자 축약 SHA 로 고정돼 있다**(`a29cad0bf6a8`). 그것이 브랜치
    /// 팁이 아니라 조상이므로 이름 해소가 필요하다.
    ///
    /// # Errors
    /// 그런 것이 없거나 커밋이 아니면.
    pub fn resolve_commit(&self, rev: &str) -> Result<ObjectName, GitError> {
        let id = self
            .inner
            .rev_parse_single(rev)
            .map_err(|e| GitError::Resolve(format!("{rev}: {e}")))?;
        let object = id
            .object()
            .map_err(|e| GitError::Resolve(format!("{rev}: {e}")))?;
        let commit = object
            .try_into_commit()
            .map_err(|e| GitError::Resolve(format!("{rev} 는 커밋이 아니다: {e}")))?;
        Ok(to_name(commit.id))
    }
}

/// `gix` 의 객체 이름을 우리 것으로. **여기가 유일한 변환 지점이다.**
fn to_name(id: gix::ObjectId) -> ObjectName {
    let mut raw = [0u8; 20];
    // SHA-1 은 20바이트다. 상류가 SHA-256 을 내면 여기서 잘리므로 길이를 확인한다.
    let bytes = id.as_bytes();
    let n = bytes.len().min(20);
    raw[..n].copy_from_slice(&bytes[..n]);
    ObjectName::from_bytes(raw)
}

impl GitAccess for GixRepo {
    fn head(&self) -> Result<ObjectName, GitError> {
        let id = self
            .inner
            .head_id()
            .map_err(|e| GitError::Head(e.to_string()))?;
        Ok(to_name(id.detach()))
    }

    fn list_tree(&self, at: &TreeRef) -> Result<Vec<(RepoPath, ObjectName)>, GitError> {
        // **워킹트리를 물으면 워킹트리를 센다.** `base()` 로 넘겨 버리면 커밋 좌표가
        // 워킹트리 좌표인 척하게 된다 — R-06 이 겨냥한 그 자리를 스스로 지우는 셈이다.
        if !at.is_committed() {
            return self.worktree_list();
        }
        let base = gix::ObjectId::from_hex(at.base().to_hex().as_bytes())
            .map_err(|e| GitError::Resolve(e.to_string()))?;
        let commit = self
            .inner
            .find_object(base)
            .map_err(|e| GitError::Resolve(e.to_string()))?
            .try_into_commit()
            .map_err(|e| GitError::Resolve(e.to_string()))?;
        let tree = commit.tree().map_err(|e| GitError::Tree(e.to_string()))?;

        let mut recorder = gix::traverse::tree::Recorder::default();
        tree.traverse()
            .breadthfirst(&mut recorder)
            .map_err(|e| GitError::Tree(e.to_string()))?;

        let mut out = Vec::with_capacity(recorder.records.len());
        for entry in recorder.records {
            // **디렉터리와 서브모듈은 파일이 아니다.** 대장이 세는 것은 blob 이고,
            // 그것이 `git ls-tree -r` 가 내는 것과 같아야 한다(criteria [s1.oracle]).
            if !entry.mode.is_blob_or_symlink() {
                continue;
            }
            let path = String::from_utf8_lossy(entry.filepath.as_ref()).into_owned();
            out.push((RepoPath::new(path), to_name(entry.oid)));
        }
        Ok(out)
    }

    fn read_blob(&self, name: ObjectName) -> Result<Vec<u8>, GitError> {
        let id = gix::ObjectId::from_hex(name.to_hex().as_bytes())
            .map_err(|e| GitError::Blob(e.to_string()))?;
        let object = self
            .inner
            .find_object(id)
            .map_err(|e| GitError::Blob(format!("{name}: {e}")))?;
        Ok(object.data.clone())
    }

    fn worktree_state(&self) -> Result<WorktreeState, GitError> {
        let base = self.head()?;
        let (list, trusted, rehashed) = self.scan_worktree()?;
        let tree_digest = digest_of(&list);

        // **`base` 의 트리와 대는 것이 `dirty` 의 정의다**(F01 §3.2).
        let mut committed = self.list_tree(&TreeRef::Committed(base))?;
        committed.sort();
        Ok(WorktreeState {
            base,
            tree_digest,
            dirty_paths: differing_paths(&committed, &list),
            trusted_from_index: trusted,
            rehashed,
        })
    }

    fn read_worktree_file(&self, path: &RepoPath) -> Result<Vec<u8>, GitError> {
        let absolute = self.workdir()?.join(path.as_str());
        let meta = std::fs::symlink_metadata(&absolute)
            .map_err(|e| GitError::Worktree(format!("{path}: {e}")))?;
        self.blob_content_of(path, &absolute, &meta)
    }

    fn commit(&self, id: ObjectName) -> Result<CommitMeta, GitError> {
        let commit = self.commit_at(id)?;
        let summary = commit
            .message()
            .map_err(|e| GitError::Resolve(e.to_string()))?
            .summary()
            .to_string();
        let author = commit.author().map_err(|e| GitError::Resolve(e.to_string()))?;
        Ok(CommitMeta {
            id,
            parents: commit.parent_ids().map(|p| to_name(p.detach())).collect(),
            summary,
            author_id: author.email.to_string(),
            author_display: author.name.to_string(),
            epoch_secs: author.time().map_or(0, |t| t.seconds),
        })
    }

    fn path_at(&self, at: &TreeRef, path: &RepoPath) -> Result<Option<ObjectName>, GitError> {
        let base = to_oid(at.base())?;
        let commit = self.commit_at_oid(base)?;
        let tree = commit.tree().map_err(|e| GitError::Tree(e.to_string()))?;
        let entry = tree
            .lookup_entry_by_path(path.as_str())
            .map_err(|e| GitError::Tree(e.to_string()))?;
        Ok(entry.filter(|e| e.mode().is_blob_or_symlink()).map(|e| to_name(e.object_id())))
    }

    fn first_parent_walk(
        &self,
        from: ObjectName,
        limit: usize,
    ) -> Result<Vec<ObjectName>, GitError> {
        let mut out = Vec::new();
        let mut cursor = Some(from);
        while let Some(id) = cursor {
            out.push(id);
            if out.len() >= limit {
                break;
            }
            let commit = self.commit_at(id)?;
            cursor = commit.parent_ids().next().map(|p| to_name(p.detach()));
        }
        Ok(out)
    }

    fn changed_in(&self, commit: ObjectName) -> Result<Vec<RepoPath>, GitError> {
        let this = self.commit_at(commit)?;
        let 지금 = 경로집합(&this.tree().map_err(|e| GitError::Tree(e.to_string()))?)?;

        let Some(parent) = this.parent_ids().next() else {
            // **최초 커밋** — 부모가 없으면 그 트리 전부가 여기서 생겼다.
            let mut out: Vec<RepoPath> = 지금.into_keys().collect();
            out.sort();
            return Ok(out);
        };
        let 부모 = 경로집합(
            &self.commit_at_oid(parent.detach())?.tree().map_err(|e| GitError::Tree(e.to_string()))?,
        )?;

        let mut out = Vec::new();
        for (path, oid) in &지금 {
            // 추가되었거나 내용이 달라졌다.
            if 부모.get(path) != Some(oid) {
                out.push(path.clone());
            }
        }
        // **지워진 것도 「바뀐 것」이다.** 빼면 *"이 커밋이 무엇을 함께 건드렸나"* 가
        // 반쪽이 되고, 문서와 함께 지워진 코드가 신호에서 사라진다.
        // **지워진 것도 「바뀐 것」이다** — 위 주석 그대로.
        out.extend(부모.into_keys().filter(|path| !지금.contains_key(path)));
        out.sort();
        out.dedup();
        Ok(out)
    }
}

/// 트리 하나의 `경로 → blob 이름`. **blob 만 센다** — 디렉터리와 서브모듈은 파일이 아니다.
fn 경로집합(
    tree: &gix::Tree<'_>,
) -> Result<std::collections::BTreeMap<RepoPath, ObjectName>, GitError> {
    let mut recorder = gix::traverse::tree::Recorder::default();
    tree.traverse().breadthfirst(&mut recorder).map_err(|e| GitError::Tree(e.to_string()))?;
    let mut out = std::collections::BTreeMap::new();
    for entry in recorder.records {
        if !entry.mode.is_blob_or_symlink() {
            continue;
        }
        let path = String::from_utf8_lossy(entry.filepath.as_ref()).into_owned();
        out.insert(RepoPath::new(path), to_name(entry.oid));
    }
    Ok(out)
}

/// 인덱스의 stat 이 지금 파일과 같은가 — **같으면 blob 이름을 다시 재지 않는다.**
///
/// # git 자신이 쓰는 방법이고, 그 한계도 같이 물려받는다
///
/// `mtime` 과 크기가 둘 다 같으면 내용이 같다고 본다. 같은 나노초 안에 크기를 유지한 채
/// 내용이 바뀌면 놓친다(git 이 *racy* 라 부르는 자리). **그 한계를 숨기지 않는다** —
/// 대신 criteria `[f01.pass]` ③ 의 첫째 변이가 *"내용 1바이트를 바꾸면 요약이 바뀌는가"*
/// 를 실제로 센다.
///
/// **`.palimpsest/worktree.state` 캐시는 만들지 않는다.** F01 §3.2 는 *"인덱스 mtime 으로
/// 무효화"* 를 적었는데 **그것이 틀렸다** — 파일을 고치고 `git add` 하지 않으면 인덱스
/// mtime 은 그대로이고 워킹트리만 변한다. 그 캐시는 낡은 요약을 돌려주고, 같은 ③ 첫째
/// 변이가 그것을 반증한다.
fn stat_is_unchanged(stat: gix::index::entry::Stat, meta: &std::fs::Metadata) -> bool {
    // 인덱스가 크기를 u32 로 자른다 — 4GB 넘는 파일은 stat 을 믿지 않고 다시 잰다.
    let Ok(size) = u32::try_from(meta.len()) else { return false };
    if stat.size != size {
        return false;
    }
    let Ok(mtime) = meta.modified() else { return false };
    let Ok(since) = mtime.duration_since(std::time::UNIX_EPOCH) else { return false };
    let Ok(secs) = u32::try_from(since.as_secs()) else { return false };
    stat.mtime.secs == secs && stat.mtime.nsecs == since.subsec_nanos()
}

/// 워킹트리 파일의 blob 내용. **심볼릭 링크의 내용은 링크 대상 문자열이다** — git 이
/// 그렇게 저장하고, 다르게 읽으면 blob 이름이 `git hash-object` 와 어긋난다.
fn read_worktree_blob(
    absolute: &Path,
    meta: &std::fs::Metadata,
) -> Result<Vec<u8>, GitError> {
    if meta.is_symlink() {
        let target = std::fs::read_link(absolute)
            .map_err(|e| GitError::Worktree(format!("{}: {e}", absolute.display())))?;
        return Ok(target.to_string_lossy().into_owned().into_bytes());
    }
    std::fs::read(absolute).map_err(|e| GitError::Worktree(format!("{}: {e}", absolute.display())))
}

/// `\r\n` → `\n`. **git 의 clean 필터가 하는 일이다.**
///
/// 홀로 선 `\r` 은 건드리지 않는다 — git 도 그렇다.
fn normalize_crlf(raw: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(raw.len());
    let mut i = 0;
    while i < raw.len() {
        if raw[i] == b'\r' && raw.get(i + 1) == Some(&b'\n') {
            i += 1;
            continue;
        }
        out.push(raw[i]);
        i += 1;
    }
    out
}

/// 두 정렬된 목록에서 다른 경로 — 한쪽에만 있거나 blob 이름이 다른 것.
fn differing_paths(
    committed: &[(RepoPath, ObjectName)],
    worktree: &[(RepoPath, ObjectName)],
) -> Vec<RepoPath> {
    let mut out = Vec::new();
    let (mut i, mut j) = (0, 0);
    while i < committed.len() && j < worktree.len() {
        match committed[i].0.cmp(&worktree[j].0) {
            std::cmp::Ordering::Equal => {
                if committed[i].1 != worktree[j].1 {
                    out.push(committed[i].0.clone());
                }
                i += 1;
                j += 1;
            }
            // 커밋에만 있다 — 워킹트리에서 지워졌다.
            std::cmp::Ordering::Less => {
                out.push(committed[i].0.clone());
                i += 1;
            }
            // 워킹트리에만 있다 — 새로 추적됐다.
            std::cmp::Ordering::Greater => {
                out.push(worktree[j].0.clone());
                j += 1;
            }
        }
    }
    out.extend(committed[i..].iter().map(|(p, _)| p.clone()));
    out.extend(worktree[j..].iter().map(|(p, _)| p.clone()));
    out
}

fn to_oid(name: ObjectName) -> Result<gix::ObjectId, GitError> {
    gix::ObjectId::from_hex(name.to_hex().as_bytes())
        .map_err(|e| GitError::Resolve(e.to_string()))
}

impl GixRepo {
    /// 작업 디렉터리. **bare 저장소에는 없고, 없는 것을 흉내 내지 않는다.**
    fn workdir(&self) -> Result<&Path, GitError> {
        self.inner
            .workdir()
            .ok_or_else(|| GitError::NoWorktree("bare 저장소에는 워킹트리가 없다".to_owned()))
    }

    /// 인덱스를 훑어 `(경로, blob 이름)` 목록을 만든다 — **정렬해서 낸다.**
    ///
    /// 돌려주는 셋째·넷째는 회계다: 인덱스 stat 을 믿은 수와 다시 해시한 수.
    fn scan_worktree(&self) -> Result<WorktreeScan, GitError> {
        let workdir = self.workdir()?.to_owned();
        let index = self
            .inner
            .index()
            .map_err(|e| GitError::Worktree(format!("인덱스를 읽지 못했다: {e}")))?;

        let mut list: Vec<(RepoPath, ObjectName)> = Vec::with_capacity(index.entries().len());
        let mut trusted = 0usize;
        let mut rehashed = 0usize;

        for entry in index.entries() {
            // **디렉터리와 서브모듈은 파일이 아니다** — `list_tree` 와 같은 규칙이다.
            if entry.mode.is_sparse() || entry.mode.is_submodule() {
                continue;
            }
            let path = RepoPath::new(String::from_utf8_lossy(entry.path(&index)).into_owned());
            let absolute = workdir.join(path.as_str());

            // **파일이 없으면 목록에서 빠진다.** 삭제는 요약을 바꿔야 하는 변이 셋째다.
            let Ok(meta) = std::fs::symlink_metadata(&absolute) else { continue };

            if stat_is_unchanged(entry.stat, &meta) {
                trusted += 1;
                list.push((path, to_name(entry.id)));
                continue;
            }
            rehashed += 1;
            let content = self.blob_content_of(&path, &absolute, &meta)?;
            let id =
                gix::objs::compute_hash(gix::hash::Kind::Sha1, gix::object::Kind::Blob, &content)
                    .map_err(|e| GitError::Worktree(format!("{path}: {e}")))?;
            list.push((path, to_name(id)));
        }

        // **정렬은 여기서 한다.** 같은 워킹트리가 같은 요약을 내야 비교가 성립한다.
        list.sort();
        Ok((list, trusted, rehashed))
    }

    fn worktree_list(&self) -> Result<Vec<(RepoPath, ObjectName)>, GitError> {
        Ok(self.scan_worktree()?.0)
    }

    /// 워킹트리의 `.gitattributes` 들. **읽지 못하는 것은 없는 것으로 두지 않는다** —
    /// 읽기 실패는 규칙이 없는 것과 다르지만, 여기서 오류를 내면 `.gitattributes` 가
    /// 없는 저장소도 못 다룬다. 없는 파일은 건너뛰고 **읽다 실패한 것만** 오류다.
    fn attributes(&self) -> &Attributes {
        self.attributes.get_or_init(|| {
            let Ok(workdir) = self.workdir() else { return Attributes::default() };
            let Ok(index) = self.inner.index() else { return Attributes::default() };
            let mut files = Vec::new();
            for entry in index.entries() {
                let path = String::from_utf8_lossy(entry.path(&index)).into_owned();
                let Some(dir) = path.strip_suffix(".gitattributes") else { continue };
                let dir = dir.trim_end_matches('/').to_owned();
                if let Ok(body) = std::fs::read_to_string(workdir.join(&path)) {
                    files.push((dir, body));
                }
            }
            Attributes::parse(&files)
        })
    }

    /// **git 이 저장할 내용** — 워킹트리 파일이 아니라 blob 이 될 바이트다.
    ///
    /// # 이 함수가 없으면 깨끗한 워킹트리가 dirty 로 보인다
    ///
    /// `.gitattributes` 에 `text` 가 걸린 파일은 체크아웃에서 CRLF 가 들어가고
    /// (`eol=crlf`), 저장소의 blob 은 LF 다. 그 파일을 **읽은 그대로** 해시하면
    /// git 의 blob 이름과 다른 값이 나오고, 그러면 아무것도 안 고친 워킹트리가
    /// *"파일 1개가 다르다"* 를 낸다. 실제로 `gradlew.bat` 에서 그렇게 나왔다.
    ///
    /// **`core.autocrlf` 도 본다.** 속성이 미지정인 파일은 그 설정이 정한다 —
    /// `true` 나 `input` 이면 텍스트로 보고 되돌린다.
    fn blob_content_of(
        &self,
        path: &RepoPath,
        absolute: &Path,
        meta: &std::fs::Metadata,
    ) -> Result<Vec<u8>, GitError> {
        let raw = read_worktree_blob(absolute, meta)?;
        // 링크 대상 문자열에는 줄바꿈 변환이 걸리지 않는다.
        if meta.is_symlink() {
            return Ok(raw);
        }
        let text = self.attributes().of(path).text.or_default(self.autocrlf());
        if !text {
            return Ok(raw);
        }
        // **NUL 이 있으면 git 도 손대지 않는다.** `text=auto` 의 그 규칙이고, 여기서는
        // 명시된 `text` 에도 같은 보호를 건다 — 바이너리를 정규화하면 내용이 깨진다.
        if raw.contains(&0) {
            return Ok(raw);
        }
        Ok(normalize_crlf(&raw))
    }

    /// `core.autocrlf` — 미지정 파일의 기본값.
    fn autocrlf(&self) -> bool {
        self.inner
            .config_snapshot()
            .string("core.autocrlf")
            .is_some_and(|v| matches!(&v[..], b"true" | b"input"))
    }

    fn commit_at(&self, id: ObjectName) -> Result<gix::Commit<'_>, GitError> {
        self.commit_at_oid(to_oid(id)?)
    }

    fn commit_at_oid(&self, id: gix::ObjectId) -> Result<gix::Commit<'_>, GitError> {
        self.inner
            .find_object(id)
            .map_err(|e| GitError::Resolve(e.to_string()))?
            .try_into_commit()
            .map_err(|e| GitError::Resolve(e.to_string()))
    }
}



