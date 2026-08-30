//! 사용자별 외부 command-oracle 승인 저장소.

use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};

use pal_git::{GitAccess, GixRepo};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::ledger::Oracle;

const DOMAIN: &[u8] = b"pal.round.approval.v1\0";

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
    let git = GixRepo::open(&repo).map_err(|error| ApprovalError::Identity(error.to_string()))?;
    let head = git
        .head()
        .map_err(|error| ApprovalError::Identity(error.to_string()))?;
    let ancestors = git
        .first_parent_walk(head, usize::MAX)
        .map_err(|error| ApprovalError::Identity(error.to_string()))?;
    let root = ancestors
        .last()
        .ok_or_else(|| ApprovalError::Identity("repository root commit이 없다".to_owned()))?;
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

pub fn store_dir(repo: &Path, requested: Option<&Path>) -> Result<PathBuf, ApprovalError> {
    let path = if let Some(path) = requested
        .map(Path::to_path_buf)
        .or_else(|| std::env::var_os("PAL_APPROVAL_DIR").map(PathBuf::from))
    {
        path
    } else {
        default_store()?
    };
    std::fs::create_dir_all(&path)
        .map_err(|error| ApprovalError::Store(format!("{}: {error}", path.display())))?;
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

pub fn approve(dir: &Path, digest: &str) -> Result<(), ApprovalError> {
    let target = dir.join(format!("{digest}.json"));
    reject_link(&target, true)?;
    let temporary = dir.join(format!(".{digest}.{}.tmp", std::process::id()));
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
    let body = serde_json::to_vec(&Record {
        version: 1,
        digest: digest.to_owned(),
    })
    .map_err(|error| ApprovalError::Store(error.to_string()))?;
    file.write_all(&body)
        .and_then(|()| file.write_all(b"\n"))
        .and_then(|()| file.sync_all())
        .map_err(|error| ApprovalError::Store(format!("approval 기록: {error}")))?;
    drop(file);
    #[cfg(windows)]
    if target.exists() {
        reject_link(&target, false)?;
        std::fs::remove_file(&target).map_err(|error| {
            ApprovalError::Store(format!("기존 approval record를 교체하지 못했다: {error}"))
        })?;
    }
    std::fs::rename(&temporary, &target)
        .map_err(|error| ApprovalError::Store(format!("approval atomic rename: {error}")))?;
    private_file(&target)?;
    Ok(())
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
fn private_file(path: &Path) -> Result<(), ApprovalError> {
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
fn private_file(path: &Path) -> Result<(), ApprovalError> {
    use std::os::windows::fs::MetadataExt;

    let metadata =
        std::fs::symlink_metadata(path).map_err(|error| ApprovalError::Store(error.to_string()))?;
    if !metadata.is_file() || metadata.file_type().is_symlink() || metadata.number_of_links() != 1 {
        return Err(ApprovalError::Store(
            "approval record가 단일 regular file이 아니다".to_owned(),
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
        .access_mode(READ_CONTROL | WRITE_DAC)
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
    let before = windows_permissions::wrappers::GetSecurityInfo(
        &handle,
        SeObjectType::SE_FILE_OBJECT,
        SecurityInformation::Owner,
    )
    .map_err(|error| ApprovalError::Store(format!("read approval owner: {error}")))?;
    if before.owner() != Some(&*sid) {
        return Err(ApprovalError::Store(
            "approval owner가 current token SID가 아니다".to_owned(),
        ));
    }
    windows_permissions::wrappers::SetSecurityInfo(
        &mut handle,
        SeObjectType::SE_FILE_OBJECT,
        SecurityInformation::Dacl | SecurityInformation::ProtectedDacl,
        None,
        None,
        descriptor.dacl(),
        None,
    )
    .map_err(|error| ApprovalError::Store(format!("set private descriptor: {error}")))?;
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
