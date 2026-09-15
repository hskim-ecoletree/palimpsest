//! 사용자별 외부 command-oracle 승인 저장소.

use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};

use pal_git::GixRepo;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::ledger::Oracle;

const DOMAIN: &[u8] = b"pal.round.approval.v1\0";
const FINALIZATION_DOMAIN: &[u8] = b"pal.round.finalization.v1\0";

#[derive(Clone, Debug)]
pub struct Binding {
    pub digest: String,
    pub shell: PathBuf,
    pub cwd: PathBuf,
}

#[derive(Debug, Error)]
pub enum ApprovalError {
    #[error("approval 저장소 오류: {0}")]
    Store(String),
    #[error("approval identity 오류: {0}")]
    Identity(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Record {
    version: u32,
    digest: String,
}

#[allow(clippy::too_many_arguments)]
pub fn binding(
    repo: &Path,
    slug: &str,
    id: &str,
    oracle: &Oracle,
    projected_digest: &str,
    requested_shell: Option<&Path>,
    timeout_secs: u64,
    output_limit: usize,
) -> Result<Binding, ApprovalError> {
    let repo = repo
        .canonicalize()
        .map_err(|error| ApprovalError::Identity(format!("repo: {error}")))?;
    let shell = resolve_shell(requested_shell)?;
    let cwd = repo.join(&oracle.cwd).canonicalize().map_err(|error| {
        ApprovalError::Identity(format!("oracle cwd `{}`: {error}", oracle.cwd))
    })?;
    if !cwd.starts_with(&repo) {
        return Err(ApprovalError::Identity(
            "oracle cwd가 symlink를 통해 저장소 밖으로 나간다".to_owned(),
        ));
    }
    let root = repository_root_identity(&repo)?;
    let path = std::env::var_os("PATH").unwrap_or_default();
    let path_digest = blake3::hash(path.to_string_lossy().as_bytes()).to_hex();
    let shell_bytes = std::fs::read(&shell)
        .map_err(|error| ApprovalError::Identity(format!("shell을 읽지 못했다: {error}")))?;
    let shell_digest = blake3::hash(&shell_bytes).to_hex();
    let values = vec![
        root.to_string(),
        slug.to_owned(),
        id.to_owned(),
        oracle.digest.clone(),
        oracle.negative_for.clone().unwrap_or_default(),
        oracle.cwd.clone(),
        shell.to_string_lossy().to_string(),
        shell_digest.to_string(),
        path_digest.to_string(),
        timeout_secs.to_string(),
        output_limit.to_string(),
        projected_digest.to_owned(),
    ];
    let mut hasher = blake3::Hasher::new();
    hasher.update(DOMAIN);
    for value in values {
        let bytes = value.as_bytes();
        hasher.update(&(bytes.len() as u64).to_le_bytes());
        hasher.update(bytes);
    }
    Ok(Binding {
        digest: hasher.finalize().to_hex().to_string(),
        shell,
        cwd,
    })
}

pub(super) fn repository_root_identity(repo: &Path) -> Result<String, ApprovalError> {
    let git = GixRepo::open(repo).map_err(|error| ApprovalError::Identity(error.to_string()))?;
    git.stable_repository_identity()
        .map_err(|error| ApprovalError::Identity(error.to_string()))
}

pub fn finalization_digest(
    repo: &Path,
    slug: &str,
    projected_digest: &str,
    aggregate_digest: &str,
) -> Result<String, ApprovalError> {
    let project = repository_root_identity(repo)?;
    let mut hasher = blake3::Hasher::new();
    hasher.update(FINALIZATION_DOMAIN);
    for value in [project.as_str(), slug, projected_digest, aggregate_digest] {
        hasher.update(&(value.len() as u64).to_le_bytes());
        hasher.update(value.as_bytes());
    }
    Ok(hasher.finalize().to_hex().to_string())
}

pub fn store_dir(repo: &Path, requested: Option<&Path>) -> Result<PathBuf, ApprovalError> {
    let path = store_location(requested)?;
    create_store(&path)?;
    private_directory(&path)?;
    let canonical = path
        .canonicalize()
        .map_err(|error| ApprovalError::Store(format!("{}: {error}", path.display())))?;
    let repo = repo
        .canonicalize()
        .map_err(|error| ApprovalError::Store(format!("repo: {error}")))?;
    if canonical.starts_with(&repo) {
        return Err(ApprovalError::Store(
            "approval 저장소는 repository 밖이어야 한다".to_owned(),
        ));
    }
    Ok(canonical)
}

pub(super) fn store_location(requested: Option<&Path>) -> Result<PathBuf, ApprovalError> {
    if let Some(path) = requested
        .map(Path::to_path_buf)
        .or_else(|| std::env::var_os("PAL_APPROVAL_DIR").map(PathBuf::from))
    {
        Ok(path)
    } else {
        default_store()
    }
}

/// 명령 승인 · 종료 봉인 `<digest>.json` 을 쓴다 — **형식은 v1 바이트 그대로**이고, 옆에
/// `<digest>.project`(프로젝트 식별자)를 먼저 쓴다. 옛 바이너리는 표시 파일을 모르고 v1 을 그대로 읽는다.
pub fn approve(dir: &Path, digest: &str, project: &str) -> Result<(), ApprovalError> {
    let target = dir.join(format!("{digest}.json"));
    reject_link(&target, true)?;
    ensure_project_marker(&target, project)?;
    let mut body = serde_json::to_vec(&Record {
        version: 1,
        digest: digest.to_owned(),
    })
    .map_err(|error| ApprovalError::Store(error.to_string()))?;
    body.push(b'\n');
    write_private_atomic(&target, &body)
}

/// 표시 파일의 확장자 — 기록 `<이름>.json` 옆의 `<이름>.project`.
pub(super) const PROJECT_MARKER_EXTENSION: &str = "project";

/// 표시 파일이 말하는 것.
#[derive(Debug, PartialEq, Eq)]
pub(super) enum ProjectMarker {
    /// 표시 파일이 없다 — 이 바이너리 전에 쓴 기록이거나 표시 파일이 지워졌다.
    Absent,
    /// 이 프로젝트 식별자의 기록이다.
    Project(String),
    /// 있는데 읽을 수 없다(일반 파일이 아니거나 내용이 한 줄이 아니다) — 어느 몫인지 모른다.
    Unreadable,
}

/// 기록 `record` 옆의 표시 파일 경로.
pub(super) fn project_marker_path(record: &Path) -> PathBuf {
    record.with_extension(PROJECT_MARKER_EXTENSION)
}

/// 기록 `record` 의 표시 파일을 읽는다.
pub(super) fn read_project_marker(record: &Path) -> ProjectMarker {
    let marker = project_marker_path(record);
    match std::fs::symlink_metadata(&marker) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => ProjectMarker::Absent,
        Ok(metadata) if metadata.is_file() => match std::fs::read_to_string(&marker) {
            Ok(text) => {
                let value = text.strip_suffix('\n').unwrap_or(&text);
                if value.is_empty() || value.contains(['\n', '\r']) {
                    ProjectMarker::Unreadable
                } else {
                    ProjectMarker::Project(value.to_owned())
                }
            }
            Err(_) => ProjectMarker::Unreadable,
        },
        _ => ProjectMarker::Unreadable,
    }
}

/// 기록 `record` 옆에 `project` 를 적은 표시 파일이 있게 한다 — 이미 같으면 안 쓴다.
pub(super) fn ensure_project_marker(record: &Path, project: &str) -> Result<(), ApprovalError> {
    if read_project_marker(record) == ProjectMarker::Project(project.to_owned()) {
        return Ok(());
    }
    let marker = project_marker_path(record);
    reject_link(&marker, true)?;
    write_private_atomic(&marker, format!("{project}\n").as_bytes())
}

/// 같은 디렉터리의 임시 파일에 쓰고 바꿔 끼운다. 권한은 `private_file` 규율 그대로다.
fn write_private_atomic(target: &Path, body: &[u8]) -> Result<(), ApprovalError> {
    let dir = target
        .parent()
        .ok_or_else(|| ApprovalError::Store(format!("{}: 부모가 없다", target.display())))?;
    let name = target
        .file_name()
        .ok_or_else(|| ApprovalError::Store(format!("{}: 파일 이름이 없다", target.display())))?
        .to_string_lossy();
    let temporary = dir.join(format!(".{name}.{}.tmp", std::process::id()));
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options
        .open(&temporary)
        .map_err(|error| ApprovalError::Store(format!("{}: {error}", temporary.display())))?;
    file.write_all(body)
        .and_then(|()| file.sync_all())
        .map_err(|error| ApprovalError::Store(format!("approval 기록: {error}")))?;
    drop(file);
    #[cfg(windows)]
    if target.exists() {
        reject_link(target, false)?;
        std::fs::remove_file(target).map_err(|error| {
            ApprovalError::Store(format!("기존 approval record를 교체하지 못했다: {error}"))
        })?;
    }
    std::fs::rename(&temporary, target)
        .map_err(|error| ApprovalError::Store(format!("approval atomic rename: {error}")))?;
    private_file(target)?;
    Ok(())
}

/// 기본 저장소 자리의 뿌리 이름 — `…/palimpsest/approvals`.
const STORE_ROOT_NAME: &str = "palimpsest";
const STORE_NAME: &str = "approvals";

/// `palimpsest/` 안에서 **기록을 처음 쓸 때 새로 만든 조상**을 적는 파일.
pub(super) const CREATED_ANCESTORS: &str = "created-ancestors.json";

/// `palimpsest/created-ancestors.json` — `palimpsest/` 의 부모부터 위로, 새로 만든 디렉터리 이름.
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct CreatedAncestors {
    version: u32,
    created: Vec<String>,
}

/// 저장소가 `…/palimpsest/approvals` 꼴이면 그 `palimpsest/`. 아니면 조상을 적지도 걷지도 않는다.
pub(super) fn store_root(store: &Path) -> Option<&Path> {
    let parent = store.parent()?;
    (store.file_name()? == STORE_NAME && parent.file_name()? == STORE_ROOT_NAME).then_some(parent)
}

/// `palimpsest/` 에 적힌 새로 만든 조상 — 가까운 것부터. 없거나 못 읽으면 비었다(안 걷는다).
pub(super) fn created_ancestors(root: &Path) -> Vec<String> {
    let path = root.join(CREATED_ANCESTORS);
    match std::fs::symlink_metadata(&path) {
        Ok(metadata) if metadata.is_file() => std::fs::read(&path)
            .ok()
            .and_then(|bytes| serde_json::from_slice::<CreatedAncestors>(&bytes).ok())
            .filter(|record| record.version == 1)
            .map(|record| record.created)
            .unwrap_or_default(),
        _ => Vec::new(),
    }
}

/// 저장소 디렉터리를 만든다 — `create_dir_all` 이 **새로 만든 조상**을 `palimpsest/` 안에 적는다.
fn create_store(path: &Path) -> Result<(), ApprovalError> {
    let mut missing = Vec::new();
    let mut cursor = Some(path);
    while let Some(dir) = cursor.filter(|dir| !dir.as_os_str().is_empty()) {
        match std::fs::symlink_metadata(dir) {
            Ok(_) => break,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                missing.push(dir.to_path_buf());
            }
            Err(error) => {
                return Err(ApprovalError::Store(format!("{}: {error}", dir.display())));
            }
        }
        cursor = dir.parent();
    }
    std::fs::create_dir_all(path)
        .map_err(|error| ApprovalError::Store(format!("{}: {error}", path.display())))?;
    let Some(root) = store_root(path) else {
        return Ok(());
    };
    if !missing.iter().any(|dir| dir == root) {
        return Ok(());
    }
    // `missing` 은 저장소에서 위로 쌓였다 — `palimpsest/` 보다 위의 것만, 가까운 것부터.
    let created: Vec<String> = missing
        .iter()
        .filter(|dir| dir.as_path() != root && root.starts_with(dir))
        .filter_map(|dir| dir.file_name().map(|name| name.to_string_lossy().into_owned()))
        .collect();
    if created.is_empty() {
        return Ok(());
    }
    let mut body = serde_json::to_vec(&CreatedAncestors {
        version: 1,
        created,
    })
    .map_err(|error| ApprovalError::Store(error.to_string()))?;
    body.push(b'\n');
    write_private_atomic(&root.join(CREATED_ANCESTORS), &body)
}

pub fn is_approved(dir: &Path, digest: &str) -> Result<bool, ApprovalError> {
    let path = dir.join(format!("{digest}.json"));
    if !path.exists() {
        return Ok(false);
    }
    reject_link(&path, false)?;
    private_file(&path)?;
    let bytes = std::fs::read(&path)
        .map_err(|error| ApprovalError::Store(format!("{}: {error}", path.display())))?;
    let record: Record = serde_json::from_slice(&bytes)
        .map_err(|error| ApprovalError::Store(format!("approval record가 malformed다: {error}")))?;
    Ok(record.version == 1 && record.digest == digest)
}

fn reject_link(path: &Path, missing_ok: bool) -> Result<(), ApprovalError> {
    match std::fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_symlink() => Err(ApprovalError::Store(format!(
            "symlink approval target을 거부한다: {}",
            path.display()
        ))),
        Ok(_) => Ok(()),
        Err(error) if missing_ok && error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(ApprovalError::Store(format!("{}: {error}", path.display()))),
    }
}

#[cfg(unix)]
fn private_directory(path: &Path) -> Result<(), ApprovalError> {
    use std::os::unix::fs::{MetadataExt, PermissionsExt};
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o700))
        .map_err(|error| ApprovalError::Store(format!("directory permission: {error}")))?;
    let metadata =
        std::fs::symlink_metadata(path).map_err(|error| ApprovalError::Store(error.to_string()))?;
    if metadata.file_type().is_symlink()
        || metadata.mode() & 0o077 != 0
        || metadata.uid() != rustix::process::getuid().as_raw()
    {
        return Err(ApprovalError::Store(
            "approval directory owner/permission이 private가 아니다".to_owned(),
        ));
    }
    Ok(())
}

#[cfg(windows)]
fn private_directory(path: &Path) -> Result<(), ApprovalError> {
    let metadata =
        std::fs::symlink_metadata(path).map_err(|error| ApprovalError::Store(error.to_string()))?;
    if metadata.file_type().is_symlink() {
        return Err(ApprovalError::Store(
            "approval directory가 symlink다".to_owned(),
        ));
    }
    secure_windows_acl(path, true)
}

#[cfg(unix)]
pub(super) fn private_file(path: &Path) -> Result<(), ApprovalError> {
    use std::os::unix::fs::MetadataExt;
    let metadata =
        std::fs::symlink_metadata(path).map_err(|error| ApprovalError::Store(error.to_string()))?;
    if !metadata.is_file()
        || metadata.nlink() != 1
        || metadata.mode() & 0o077 != 0
        || metadata.uid() != rustix::process::getuid().as_raw()
    {
        return Err(ApprovalError::Store(
            "approval record owner/link/permission이 private가 아니다".to_owned(),
        ));
    }
    Ok(())
}

#[cfg(windows)]
pub(super) fn private_file(path: &Path) -> Result<(), ApprovalError> {
    let metadata =
        std::fs::symlink_metadata(path).map_err(|error| ApprovalError::Store(error.to_string()))?;
    if !metadata.is_file() || metadata.file_type().is_symlink() {
        return Err(ApprovalError::Store(
            "approval record가 regular file이 아니다".to_owned(),
        ));
    }
    secure_windows_acl(path, false)
}

#[cfg(windows)]
fn secure_windows_acl(path: &Path, directory: bool) -> Result<(), ApprovalError> {
    use std::os::windows::fs::OpenOptionsExt;
    use windows_permissions::constants::{
        AccessRights, AceFlags, AceType, SeObjectType, SecurityInformation,
    };
    use windows_permissions::{LocalBox, SecurityDescriptor, Sid};

    const READ_CONTROL: u32 = 0x0002_0000;
    const WRITE_DAC: u32 = 0x0004_0000;
    const WRITE_OWNER: u32 = 0x0008_0000;
    const FILE_FLAG_BACKUP_SEMANTICS: u32 = 0x0200_0000;
    const FILE_FLAG_OPEN_REPARSE_POINT: u32 = 0x0020_0000;

    let sid_text = windows_token::Token::open_current_process()
        .and_then(|token| token.user_sid())
        .map_err(|error| ApprovalError::Store(format!("current token SID: {error}")))?
        .to_string();
    let sid: LocalBox<Sid> = sid_text
        .parse()
        .map_err(|error| ApprovalError::Store(format!("current SID parse: {error}")))?;
    let ace_flags = if directory { "OICI" } else { "" };
    let descriptor: LocalBox<SecurityDescriptor> =
        format!("O:{sid_text}D:P(A;{ace_flags};FA;;;{sid_text})")
            .parse()
            .map_err(|error| ApprovalError::Store(format!("private descriptor: {error}")))?;
    let mut options = OpenOptions::new();
    options
        .access_mode(READ_CONTROL | WRITE_DAC | WRITE_OWNER)
        .share_mode(0)
        .custom_flags(
            FILE_FLAG_OPEN_REPARSE_POINT
                | if directory {
                    FILE_FLAG_BACKUP_SEMANTICS
                } else {
                    0
                },
        );
    let mut handle = options
        .open(path)
        .map_err(|error| ApprovalError::Store(format!("private handle: {error}")))?;
    windows_permissions::wrappers::SetSecurityInfo(
        &mut handle,
        SeObjectType::SE_FILE_OBJECT,
        SecurityInformation::Owner
            | SecurityInformation::Dacl
            | SecurityInformation::ProtectedDacl,
        Some(&*sid),
        None,
        descriptor.dacl(),
        None,
    )
    .map_err(|error| ApprovalError::Store(format!("set private owner/descriptor: {error}")))?;
    let actual = windows_permissions::wrappers::GetSecurityInfo(
        &handle,
        SeObjectType::SE_FILE_OBJECT,
        SecurityInformation::Owner | SecurityInformation::Dacl,
    )
    .map_err(|error| ApprovalError::Store(format!("read private descriptor: {error}")))?;
    let dacl = actual
        .dacl()
        .ok_or_else(|| ApprovalError::Store("approval DACL이 없다".to_owned()))?;
    let sddl = windows_permissions::wrappers::ConvertSecurityDescriptorToStringSecurityDescriptor(
        &actual,
        SecurityInformation::Owner | SecurityInformation::Dacl,
    )
    .map_err(|error| ApprovalError::Store(format!("render private descriptor: {error}")))?;
    let ace = dacl
        .get_ace(0)
        .ok_or_else(|| ApprovalError::Store("approval DACL이 비었다".to_owned()))?;
    let expected_flags = if directory {
        AceFlags::ObjectInherit | AceFlags::ContainerInherit
    } else {
        AceFlags::empty()
    };
    if !sddl.to_string_lossy().contains("D:P")
        || actual.owner() != Some(&*sid)
        || dacl.len() != 1
        || ace.ace_type() != AceType::ACCESS_ALLOWED_ACE_TYPE
        || ace.sid() != Some(&*sid)
        || ace.mask() != AccessRights::FileAllAccess
        || ace.flags() != expected_flags
    {
        return Err(ApprovalError::Store(
            "approval owner/DACL이 현재 SID 하나로 고정되지 않았다".to_owned(),
        ));
    }
    Ok(())
}

fn resolve_shell(requested: Option<&Path>) -> Result<PathBuf, ApprovalError> {
    let default = default_shell()?
        .canonicalize()
        .map_err(|error| ApprovalError::Identity(format!("platform default shell: {error}")))?;
    let path = requested.map_or_else(|| default.clone(), Path::to_path_buf);
    if !path.is_absolute() {
        return Err(ApprovalError::Identity(
            "shell은 PATH 검색 없는 absolute path여야 한다".to_owned(),
        ));
    }
    let canonical = path
        .canonicalize()
        .map_err(|error| ApprovalError::Identity(format!("shell: {error}")))?;
    if canonical != default {
        return Err(ApprovalError::Identity(
            "승인 가능한 shell은 platform default 하나뿐이다".to_owned(),
        ));
    }
    Ok(canonical)
}

#[cfg(unix)]
fn default_shell() -> Result<PathBuf, ApprovalError> {
    Ok(PathBuf::from("/bin/sh"))
}

#[cfg(windows)]
fn default_shell() -> Result<PathBuf, ApprovalError> {
    known_folders::get_known_folder_path(known_folders::KnownFolder::System)
        .map(|path| path.join("cmd.exe"))
        .ok_or_else(|| ApprovalError::Identity("Windows System known folder가 없다".to_owned()))
}

fn default_store() -> Result<PathBuf, ApprovalError> {
    #[cfg(windows)]
    {
        return known_folders::get_known_folder_path(known_folders::KnownFolder::LocalAppData)
            .map(|path| path.join("palimpsest").join("approvals"))
            .ok_or_else(|| {
                ApprovalError::Store("Windows LocalAppData known folder가 없다".to_owned())
            });
    }
    #[cfg(target_os = "macos")]
    {
        return std::env::var_os("HOME")
            .map(PathBuf::from)
            .map(|path| path.join("Library/Application Support/palimpsest/approvals"))
            .ok_or_else(|| {
                ApprovalError::Store("HOME이 없어 private store를 정할 수 없다".to_owned())
            });
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        if let Some(path) = std::env::var_os("XDG_DATA_HOME") {
            return Ok(PathBuf::from(path).join("palimpsest/approvals"));
        }
        std::env::var_os("HOME")
            .map(PathBuf::from)
            .map(|path| path.join(".local/share/palimpsest/approvals"))
            .ok_or_else(|| {
                ApprovalError::Store("HOME이 없어 private store를 정할 수 없다".to_owned())
            })
    }
}
