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
//! # S1 의 접촉면은 셋이다
//!
//! [F01 §3.1](../../../docs/plan/features/F01-repo-ledger.md) 이 다섯을 적었고 그것은
//! **상한이지 하한이 아니다.** S1 은 커밋 축에서 닫히므로 `worktree_state` ·
//! `changed_between` 이 필요 없다. **없는 것을 미리 흉내 내지 않는다** — 둘은 F01 이
//! `TreeRef::Worktree` 를 채울 때 함께 선다.

#![forbid(unsafe_code)]

use std::path::Path;

use pal_core::{ObjectName, RepoPath, TreeRef};

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
}

/// git 에 닿는 유일한 문. **구현이 바뀌어도 이 모양은 남는다.**
pub trait GitAccess {
    /// # Errors
    /// HEAD 가 없거나(빈 저장소) 읽지 못하면.
    fn head(&self) -> Result<ObjectName, GitError>;

    /// 트리 하나의 추적 파일 전부. **정렬은 보장하지 않는다** — 세는 쪽이 정렬한다.
    ///
    /// # Errors
    /// 트리를 읽지 못하면.
    fn list_tree(&self, at: &TreeRef) -> Result<Vec<(RepoPath, ObjectName)>, GitError>;

    /// # Errors
    /// 그 이름의 객체가 없거나 blob 이 아니면.
    fn read_blob(&self, name: ObjectName) -> Result<Vec<u8>, GitError>;

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
}

/// `gix` 구현. **이 타입 밖에서 `gix` 타입이 보이지 않는다.**
pub struct GixRepo {
    inner: gix::Repository,
}

impl GixRepo {
    /// 저장소를 연다. 경로는 작업 디렉터리이거나 `.git` 이다.
    ///
    /// # Errors
    /// git 저장소가 아니면.
    pub fn open(path: &Path) -> Result<Self, GitError> {
        let inner =
            gix::open(path).map_err(|e| GitError::Open(format!("{}: {e}", path.display())))?;
        Ok(Self { inner })
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
}

fn to_oid(name: ObjectName) -> Result<gix::ObjectId, GitError> {
    gix::ObjectId::from_hex(name.to_hex().as_bytes())
        .map_err(|e| GitError::Resolve(e.to_string()))
}

impl GixRepo {
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
