//! 승인된 oracle 실행, bounded process tree, append-only evidence.

use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use pal_core::RepoPath;
use pal_git::{GitAccess, GixRepo};
use serde::Serialize;
use thiserror::Error;

use super::approval::{self, ApprovalError};
use super::ledger::{self, Oracle, VerificationLedger};

#[derive(Clone, Debug)]
pub struct Config<'a> {
    pub repo: &'a Path,
    pub slug: &'a str,
    pub id: &'a str,
    pub approval_dir: Option<&'a Path>,
    pub shell: Option<&'a Path>,
    pub timeout_secs: u64,
    pub output_limit: usize,
}

#[derive(Debug, Serialize)]
pub struct ApproveView {
    pub outcome: &'static str,
    pub round: String,
    pub id: String,
    pub approval_digest: String,
}

#[derive(Debug, Serialize)]
pub struct VerifyView {
    pub outcome: &'static str,
    pub round: String,
    pub id: String,
    pub met: bool,
    pub exit: i32,
    pub matched: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fault: Option<&'static str>,
}

#[derive(Debug, Error)]
pub enum VerifyError {
    #[error("approval이 필요하다")]
    ApprovalRequired,
    #[error("실행 결과를 폐기했다: {0}")]
    Discarded(String),
    #[error("approve/verify 입력 오류: {0}")]
    Invalid(String),
    #[error("approve/verify I/O 오류: {0}")]
    Io(String),
}

impl From<ApprovalError> for VerifyError {
    fn from(value: ApprovalError) -> Self {
        Self::Invalid(value.to_string())
    }
}

pub fn approve(config: &Config<'_>) -> Result<ApproveView, VerifyError> {
    validate_config(config)?;
    let (_, ledger, oracle) = load(config)?;
    if ledger.schema_version != 2 {
        return Err(VerifyError::Invalid(
            "approve/verify는 schema 2의 새 회차에서만 쓴다".to_owned(),
        ));
    }
    let projected = projected_digest(config.repo, config.slug)?;
    let binding = approval::binding(
        config.repo,
        config.slug,
        config.id,
        &oracle,
        &projected,
        config.shell,
        config.timeout_secs,
        config.output_limit,
    )?;
    let dir = approval::store_dir(config.repo, config.approval_dir)?;
    approval::approve(&dir, &binding.digest)?;
    Ok(ApproveView {
        outcome: "approved",
        round: config.slug.to_owned(),
        id: config.id.to_owned(),
        approval_digest: binding.digest,
    })
}

pub fn verify(config: &Config<'_>) -> Result<VerifyView, VerifyError> {
    validate_config(config)?;
    let (ledger_path, ledger_before, oracle_before) = load(config)?;
    if ledger_before.schema_version != 2 {
        return Err(VerifyError::Invalid(
            "approve/verify는 schema 2의 새 회차에서만 쓴다".to_owned(),
        ));
    }
    let projected_before = projected_digest(config.repo, config.slug)?;
    let binding_before = approval::binding(
        config.repo,
        config.slug,
        config.id,
        &oracle_before,
        &projected_before,
        config.shell,
        config.timeout_secs,
        config.output_limit,
    )?;
    let approval_dir = approval::store_dir(config.repo, config.approval_dir)?;
    if !approval::is_approved(&approval_dir, &binding_before.digest).unwrap_or(false) {
        return Err(VerifyError::ApprovalRequired);
    }

    // 승인을 본 뒤 spawn 직전에 같은 record를 다시 연다. 교체·부분 쓰기는 실행을 못 연다.
    if !approval::is_approved(&approval_dir, &binding_before.digest).unwrap_or(false) {
        return Err(VerifyError::ApprovalRequired);
    }
    let execution = execute(
        &binding_before.shell,
        &binding_before.cwd,
        &oracle_before.check,
        &oracle_before.literal,
        config.timeout_secs,
        config.output_limit,
    )?;

    // 같은 pal writer의 post-check와 append를 한 임계구역으로 묶는다. lock을 무시하는 외부
    // 편집은 아래의 반복 currentness 검사와 atomic replace에서 fail-closed한다.
    let append_guard = AppendGuard::acquire(&ledger_path)?;
    let (_, ledger_after, oracle_after) = load(config)?;
    let projected_after = projected_digest(config.repo, config.slug)?;
    let binding_after = approval::binding(
        config.repo,
        config.slug,
        config.id,
        &oracle_after,
        &projected_after,
        config.shell,
        config.timeout_secs,
        config.output_limit,
    )?;
    if oracle_before.digest != oracle_after.digest
        || oracle_before.negative_for != oracle_after.negative_for
        || projected_before != projected_after
        || binding_before.digest != binding_after.digest
        || !approval::is_approved(&approval_dir, &binding_after.digest).unwrap_or(false)
    {
        return Err(VerifyError::Discarded(
            "실행 중 oracle, projected tree 또는 approval identity가 바뀌었다".to_owned(),
        ));
    }
    if ledger_after.schema_version != 2 {
        return Err(VerifyError::Discarded(
            "schema가 실행 중 바뀌었다".to_owned(),
        ));
    }

    let event = serde_json::json!({
        "kind": "evidence",
        "id": config.id,
        "oracle_digest": oracle_before.digest,
        "exit": execution.exit,
        "matched": execution.matched,
        "output_digest": execution.output_digest,
        "output_bytes": execution.output_bytes,
        "projected_digest": projected_before,
    });
    append_line(&append_guard, &event.to_string())?;
    Ok(VerifyView {
        outcome: "verified",
        round: config.slug.to_owned(),
        id: config.id.to_owned(),
        met: execution.exit == 0 && execution.matched && execution.fault.is_none(),
        exit: execution.exit,
        matched: execution.matched,
        fault: execution.fault,
    })
}

fn validate_config(config: &Config<'_>) -> Result<(), VerifyError> {
    if config.timeout_secs == 0 || config.timeout_secs > 86_400 {
        return Err(VerifyError::Invalid(
            "timeout은 1..=86400초여야 한다".to_owned(),
        ));
    }
    if config.output_limit == 0 || config.output_limit > 16 * 1024 * 1024 {
        return Err(VerifyError::Invalid(
            "output-limit은 1..=16777216 바이트여야 한다".to_owned(),
        ));
    }
    Ok(())
}

fn load(config: &Config<'_>) -> Result<(PathBuf, VerificationLedger, Oracle), VerifyError> {
    let outcome = super::status::read(config.repo, Some(config.slug))
        .map_err(|error| VerifyError::Invalid(error.to_string()))?;
    let super::status::Outcome::Status(view) = outcome else {
        return Err(VerifyError::Invalid(
            "명시한 round를 해소하지 못했다".to_owned(),
        ));
    };
    if !view
        .conditions
        .iter()
        .any(|condition| condition.id == config.id)
    {
        return Err(VerifyError::Invalid(format!(
            "intent condition `{}`가 없다",
            config.id
        )));
    }
    let dir = config.repo.join(".palimpsest/rounds").join(config.slug);
    let path = dir.join("verification.log");
    let ledger = ledger::read(&path, config.slug)
        .map_err(|error| VerifyError::Invalid(error.to_string()))?;
    let oracle = ledger
        .conditions
        .get(config.id)
        .and_then(|state| state.oracle.clone())
        .ok_or_else(|| VerifyError::Invalid(format!("oracle `{}`가 없다", config.id)))?;
    Ok((path, ledger, oracle))
}

pub fn projected_digest(repo: &Path, slug: &str) -> Result<String, VerifyError> {
    let git = GixRepo::open(repo).map_err(|error| VerifyError::Invalid(error.to_string()))?;
    let excluded = RepoPath::new(format!(".palimpsest/rounds/{slug}/verification.log"));
    git.worktree_digest_excluding(&[excluded])
        .map(|digest| digest.to_string())
        .map_err(|error| VerifyError::Invalid(error.to_string()))
}

struct Execution {
    exit: i32,
    matched: bool,
    output_digest: String,
    output_bytes: usize,
    fault: Option<&'static str>,
}

fn execute(
    shell: &Path,
    cwd: &Path,
    check: &str,
    literal: &str,
    timeout_secs: u64,
    output_limit: usize,
) -> Result<Execution, VerifyError> {
    let mut command = Command::new(shell);
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        command.arg("-c").arg(check).process_group(0);
    }
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NEW_PROCESS_GROUP: u32 = 0x0000_0200;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        command
            .args(["/D", "/S", "/C", check])
            .creation_flags(CREATE_NEW_PROCESS_GROUP | CREATE_NO_WINDOW);
    }
    let mut child = command
        .current_dir(cwd)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| VerifyError::Io(format!("oracle spawn: {error}")))?;

    let stdout = Arc::new(Mutex::new(Vec::new()));
    let stderr = Arc::new(Mutex::new(Vec::new()));
    let bytes = Arc::new(AtomicUsize::new(0));
    let overflow = Arc::new(AtomicBool::new(false));
    let stdout_done = Arc::new(AtomicBool::new(false));
    let stderr_done = Arc::new(AtomicBool::new(false));
    let stdout_thread = drain(
        child.stdout.take().expect("piped stdout"),
        Arc::clone(&stdout),
        Arc::clone(&bytes),
        Arc::clone(&overflow),
        Arc::clone(&stdout_done),
        output_limit,
    );
    let stderr_thread = drain(
        child.stderr.take().expect("piped stderr"),
        Arc::clone(&stderr),
        Arc::clone(&bytes),
        Arc::clone(&overflow),
        Arc::clone(&stderr_done),
        output_limit,
    );

    let deadline = Instant::now() + Duration::from_secs(timeout_secs);
    let mut fault = None;
    let mut status = None;
    loop {
        if status.is_none() {
            status = child
                .try_wait()
                .map_err(|error| VerifyError::Io(format!("oracle wait: {error}")))?;
        }
        let drains_done = stdout_done.load(Ordering::SeqCst) && stderr_done.load(Ordering::SeqCst);
        if overflow.load(Ordering::SeqCst) {
            fault = Some("output_limit");
            if status.is_none() || !drains_done {
                terminate_tree(&mut child)?;
            }
            if status.is_none() {
                status = bounded_wait(&mut child);
            }
            break;
        }
        if Instant::now() >= deadline {
            fault = Some("timeout");
            terminate_tree(&mut child)?;
            if status.is_none() {
                status = bounded_wait(&mut child);
            }
            break;
        }
        if status.is_some() && drains_done {
            break;
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    join_drain(stdout_thread, "stdout")?;
    join_drain(stderr_thread, "stderr")?;
    let stdout = stdout.lock().expect("stdout buffer").clone();
    let stderr = stderr.lock().expect("stderr buffer").clone();
    let mut output = stdout;
    if !output.is_empty() && !stderr.is_empty() {
        output.push(b'\n');
    }
    output.extend_from_slice(&stderr);
    let matched = fault.is_none()
        && !literal.is_empty()
        && String::from_utf8_lossy(&output).contains(literal);
    Ok(Execution {
        exit: status.and_then(|status| status.code()).unwrap_or(-1),
        matched,
        output_digest: blake3::hash(&output).to_hex().to_string(),
        output_bytes: bytes.load(Ordering::SeqCst),
        fault,
    })
}

fn drain<R: Read + Send + 'static>(
    mut reader: R,
    buffer: Arc<Mutex<Vec<u8>>>,
    bytes: Arc<AtomicUsize>,
    overflow: Arc<AtomicBool>,
    done: Arc<AtomicBool>,
    limit: usize,
) -> JoinHandle<()> {
    std::thread::spawn(move || {
        let mut chunk = [0_u8; 8192];
        loop {
            let Ok(count) = reader.read(&mut chunk) else {
                break;
            };
            if count == 0 {
                break;
            }
            let before = bytes.fetch_add(count, Ordering::SeqCst);
            if before < limit {
                let keep = count.min(limit - before);
                buffer
                    .lock()
                    .expect("output buffer")
                    .extend_from_slice(&chunk[..keep]);
            }
            if before.saturating_add(count) > limit {
                overflow.store(true, Ordering::SeqCst);
            }
        }
        done.store(true, Ordering::SeqCst);
    })
}

fn join_drain(handle: JoinHandle<()>, stream: &str) -> Result<(), VerifyError> {
    handle
        .join()
        .map_err(|_| VerifyError::Io(format!("{stream} drain thread가 panic했다")))
}

fn bounded_wait(child: &mut Child) -> Option<ExitStatus> {
    let deadline = Instant::now() + Duration::from_millis(1500);
    while Instant::now() < deadline {
        if let Ok(Some(status)) = child.try_wait() {
            return Some(status);
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    let _ = child.kill();
    child.wait().ok()
}

#[cfg(unix)]
fn terminate_tree(child: &mut Child) -> Result<(), VerifyError> {
    let pid = rustix::process::Pid::from_raw(child.id() as i32)
        .ok_or_else(|| VerifyError::Io("child PID가 유효하지 않다".to_owned()))?;
    match rustix::process::kill_process_group(pid, rustix::process::Signal::KILL) {
        Ok(()) | Err(rustix::io::Errno::SRCH) => Ok(()),
        Err(error) => Err(VerifyError::Io(format!("process-group cleanup: {error}"))),
    }
}

#[cfg(windows)]
fn terminate_tree(child: &mut Child) -> Result<(), VerifyError> {
    let taskkill = approval::cleanup_program()?;
    let pid = child.id().to_string();
    let mut killer = Command::new(taskkill)
        .args(["/pid", pid.as_str(), "/f", "/t"])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|error| VerifyError::Io(format!("taskkill: {error}")))?;
    let deadline = Instant::now() + Duration::from_secs(5);
    let result = loop {
        if let Some(status) = killer
            .try_wait()
            .map_err(|error| VerifyError::Io(format!("taskkill wait: {error}")))?
        {
            break Some(status);
        }
        if Instant::now() >= deadline {
            let _ = killer.kill();
            break killer.wait().ok();
        }
        std::thread::sleep(Duration::from_millis(10));
    };
    if !result.is_some_and(|status| status.success()) {
        let _ = child.kill();
        return Err(VerifyError::Io(
            "taskkill이 5초 안에 process tree를 종료하지 못했다".to_owned(),
        ));
    }
    Ok(())
}

fn append_line(guard: &AppendGuard, line: &str) -> Result<(), VerifyError> {
    let path = &guard.ledger;
    let metadata = std::fs::symlink_metadata(path)
        .map_err(|error| VerifyError::Io(format!("ledger metadata: {error}")))?;
    if !metadata.is_file() || metadata.file_type().is_symlink() {
        return Err(VerifyError::Io("ledger가 regular file이 아니다".to_owned()));
    }
    let current = std::fs::read(path)
        .map_err(|error| VerifyError::Io(format!("ledger read before replace: {error}")))?;
    if !current.ends_with(b"\n") {
        return Err(VerifyError::Io(
            "ledger가 완전한 줄바꿈으로 끝나지 않는다".to_owned(),
        ));
    }
    let parent = path
        .parent()
        .ok_or_else(|| VerifyError::Io("ledger parent가 없다".to_owned()))?;
    let mut temporary = tempfile::NamedTempFile::new_in(parent)
        .map_err(|error| VerifyError::Io(format!("evidence temp: {error}")))?;
    std::io::Write::write_all(&mut temporary, &current)
        .and_then(|()| std::io::Write::write_all(&mut temporary, line.as_bytes()))
        .and_then(|()| std::io::Write::write_all(&mut temporary, b"\n"))
        .map_err(|error| VerifyError::Io(format!("complete evidence ledger: {error}")))?;
    temporary
        .as_file()
        .set_permissions(metadata.permissions())
        .and_then(|()| temporary.as_file().sync_all())
        .map_err(|error| VerifyError::Io(format!("evidence temp sync: {error}")))?;
    let persisted = temporary
        .persist(path)
        .map_err(|error| VerifyError::Io(format!("evidence atomic replace: {}", error.error)))?;
    persisted
        .sync_all()
        .map_err(|error| VerifyError::Io(format!("evidence sync: {error}")))?;
    #[cfg(unix)]
    std::fs::File::open(parent)
        .and_then(|directory| directory.sync_all())
        .map_err(|error| VerifyError::Io(format!("evidence directory sync: {error}")))?;
    Ok(())
}

struct AppendGuard {
    ledger: PathBuf,
    lock: PathBuf,
    _file: std::fs::File,
}

impl AppendGuard {
    fn acquire(path: &Path) -> Result<Self, VerifyError> {
        let lock = path.with_file_name("verification.log.append.lock");
        let lock_file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&lock)
            .map_err(|error| VerifyError::Io(format!("append lock: {error}")))?;
        Ok(Self {
            ledger: path.to_path_buf(),
            lock,
            _file: lock_file,
        })
    }
}

impl Drop for AppendGuard {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.lock);
    }
}
