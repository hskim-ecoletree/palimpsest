//! 명시적으로 활성화된 Stop 정책과 진행 인지형 자기 상한.

use std::fs::{OpenOptions, symlink_metadata};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::approval;
use super::status::{self, ConditionState, StatusView, Terminal, VerificationState};

const ACTIVATION_VERSION: u32 = 1;
const PROGRESS_VERSION: u32 = 1;
const POLICY: &str = "round-stop-progress-guard-v1";
const ACTIVATION_DOMAIN: &[u8] = b"pal.round.stop.activation.v1\0";
const SEMANTIC_DOMAIN: &[u8] = b"pal.round.stop.semantic.v1\0";
const EVENT_DOMAIN: &[u8] = b"pal.round.stop.event.v1\0";
use pal_core::{
    ROUND_STOP_EVENT_HISTORY_MAX as EVENT_HISTORY_MAX,
    ROUND_STOP_NO_PROGRESS_LIMIT as NO_PROGRESS_LIMIT,
    ROUND_STOP_TRANSCRIPT_MAX_BYTES as TRANSCRIPT_MAX_BYTES,
};
const LOCK_STALE_MILLIS: u64 = 30_000;
const LOCK_WAIT_MILLIS: u64 = 2_000;

#[derive(Debug)]
pub enum PolicyDecision {
    Pass(String),
    Block(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Activation {
    version: u32,
    project: String,
    round: String,
    policy: String,
    no_progress_limit: u32,
    digest: String,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ProgressRank {
    terminal: u32,
    met: u32,
    observed: u32,
    registered: u32,
}

impl ProgressRank {
    fn advances(self, previous: Self) -> bool {
        let current = [self.terminal, self.met, self.observed, self.registered];
        let previous = [
            previous.terminal,
            previous.met,
            previous.observed,
            previous.registered,
        ];
        current.iter().zip(previous).all(|(a, b)| *a >= b)
            && current.iter().zip(previous).any(|(a, b)| *a > b)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Progress {
    version: u32,
    activation_digest: String,
    semantic_digest: String,
    best: ProgressRank,
    no_progress: u32,
    event_hashes: Vec<String>,
    handoff: Option<String>,
}

#[derive(Serialize)]
struct CommandView<'a> {
    outcome: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    round: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    activation_digest: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    no_progress: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    handoff: Option<&'a str>,
}

enum ActivationState {
    Inactive,
    Active(Activation, PathBuf),
    Corrupt(String),
}

pub fn command_enable(repo: &Path, slug: &str, requested: Option<&Path>, json: bool) -> Result<()> {
    let repo = canonical_repo(repo)?;
    let status = match status::read(&repo, Some(slug)) {
        Ok(status::Outcome::Status(view)) => view,
        Ok(status::Outcome::NoActiveRound) => bail!("round `{slug}`가 없다"),
        Err(error) => bail!("활성화할 round 상태가 유효하지 않다: {error}"),
    };
    if status.terminal != Terminal::Open {
        bail!("열린 round만 Stop 정책을 활성화할 수 있다");
    }
    let project = approval::repository_root_identity(&repo).map_err(anyhow::Error::from)?;
    let digest = activation_digest(&project, slug);
    let activation = Activation {
        version: ACTIVATION_VERSION,
        project,
        round: slug.to_owned(),
        policy: POLICY.to_owned(),
        no_progress_limit: NO_PROGRESS_LIMIT,
        digest,
    };
    let store = approval::store_dir(&repo, requested).map_err(anyhow::Error::from)?;
    let path = activation_path(&store, &activation.project);
    let stale_progress = progress_path(&store, &activation.digest);
    match std::fs::remove_file(&stale_progress) {
        Ok(()) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => return Err(error).context("stale Stop progress 제거"),
    }
    write_private_json(&path, &activation)?;
    let view = CommandView {
        outcome: "enabled",
        round: Some(&activation.round),
        activation_digest: Some(&activation.digest),
        no_progress: Some(0),
        handoff: None,
    };
    print_view(json, &view, &format!("Stop enabled: {}", activation.round));
    Ok(())
}

pub fn command_disable(repo: &Path, requested: Option<&Path>, json: bool) -> Result<()> {
    disable(repo, requested)?;
    let view = CommandView {
        outcome: "disabled",
        round: None,
        activation_digest: None,
        no_progress: None,
        handoff: None,
    };
    print_view(json, &view, "Stop disabled");
    Ok(())
}

pub(crate) fn disable(repo: &Path, requested: Option<&Path>) -> Result<()> {
    let repo = canonical_repo(repo)?;
    let project = approval::repository_root_identity(&repo).map_err(anyhow::Error::from)?;
    let store = approval::store_location(requested).map_err(anyhow::Error::from)?;
    let path = activation_path(&store, &project);
    match std::fs::remove_file(&path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => {
            Err(error).with_context(|| format!("Stop activation 제거: {}", path.display()))
        }
    }
}

pub(crate) fn disable_if_supported(repo: &Path) -> Result<()> {
    let repo = repo
        .canonicalize()
        .with_context(|| format!("repo `{}`", repo.display()))?;
    let project = match approval::repository_root_identity(&repo) {
        Ok(project) => project,
        Err(_) => return Ok(()),
    };
    let store = approval::store_location(None).map_err(anyhow::Error::from)?;
    let path = activation_path(&store, &project);
    match std::fs::remove_file(&path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => {
            Err(error).with_context(|| format!("Stop activation 제거: {}", path.display()))
        }
    }
}

pub fn command_status(repo: &Path, requested: Option<&Path>, json: bool) -> Result<()> {
    let repo = canonical_repo(repo)?;
    match activation_state(&repo, requested) {
        ActivationState::Inactive => {
            let view = CommandView {
                outcome: "disabled",
                round: None,
                activation_digest: None,
                no_progress: None,
                handoff: None,
            };
            print_view(json, &view, "Stop disabled");
        }
        ActivationState::Corrupt(error) => bail!("Stop activation이 손상됐다: {error}"),
        ActivationState::Active(activation, path) => {
            let progress = read_progress(&progress_path(
                path.parent().expect("activation parent"),
                &activation.digest,
            ))?;
            let view = CommandView {
                outcome: "enabled",
                round: Some(&activation.round),
                activation_digest: Some(&activation.digest),
                no_progress: progress.as_ref().map(|state| state.no_progress).or(Some(0)),
                handoff: progress.as_ref().and_then(|state| state.handoff.as_deref()),
            };
            print_view(json, &view, &format!("Stop enabled: {}", activation.round));
        }
    }
    Ok(())
}

pub fn decide(payload: &Value) -> PolicyDecision {
    let Some(cwd) = payload
        .get("cwd")
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
    else {
        return PolicyDecision::Pass(
            "cwd가 없어 activation을 확인할 수 없는 입력이다 — 기존 fail-open을 지킨다".to_owned(),
        );
    };
    let repo = match canonical_repo(Path::new(cwd)) {
        Ok(repo) => repo,
        Err(error) => {
            return PolicyDecision::Pass(format!(
                "repository를 해소하지 못해 activation을 확인할 수 없다 — 기존 fail-open: {error}"
            ));
        }
    };
    let (activation, activation_path) = match activation_state(&repo, None) {
        ActivationState::Inactive => {
            return PolicyDecision::Pass("Stop 정책이 활성화되지 않았다".to_owned());
        }
        ActivationState::Corrupt(error) => {
            return PolicyDecision::Block(format!("Stop activation이 손상됐다: {error}"));
        }
        ActivationState::Active(activation, path) => (activation, path),
    };

    if payload.get("hook_event_name").and_then(Value::as_str) != Some("Stop") {
        return PolicyDecision::Block(
            "활성 Stop payload의 hook_event_name이 Stop이 아니다".to_owned(),
        );
    }
    if payload.get("stop_hook_active") != Some(&Value::Bool(false)) {
        return PolicyDecision::Block(
            "활성 Stop payload의 stop_hook_active가 false boolean이 아니다".to_owned(),
        );
    }
    let Some(session_id) = payload
        .get("session_id")
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
    else {
        return PolicyDecision::Block("활성 Stop payload의 session_id가 없다".to_owned());
    };
    let Some(transcript_path) = payload
        .get("transcript_path")
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
    else {
        return PolicyDecision::Block("활성 Stop payload의 transcript_path가 없다".to_owned());
    };
    let event_hash = match event_hash(session_id, Path::new(transcript_path)) {
        Ok(hash) => hash,
        Err(error) => {
            return PolicyDecision::Block(format!("Stop transcript를 읽지 못했다: {error}"));
        }
    };

    let view = match stable_status(&repo, &activation.round) {
        Ok(view) => view,
        Err(error) => return PolicyDecision::Block(format!("활성 round 상태가 손상됐다: {error}")),
    };
    if view.terminal == Terminal::Folded {
        return PolicyDecision::Pass("회차가 folded 종료문을 가졌다".to_owned());
    }
    if view.terminal == Terminal::Reported && view.verification == VerificationState::Met {
        return PolicyDecision::Pass(
            "필수 condition의 current evidence와 report가 모두 있다".to_owned(),
        );
    }

    let semantic_digest = semantic_digest(&view);
    let rank = progress_rank(&view);
    let store = activation_path.parent().expect("activation parent");
    let progress_path = progress_path(store, &activation.digest);
    match record_attempt(
        &progress_path,
        &activation,
        &semantic_digest,
        rank,
        &event_hash,
    ) {
        Ok(progress) if progress.handoff.as_deref() == Some("blocked") => {
            PolicyDecision::Pass(format!(
                "같은 의미 상태에서 무진행 {}회에 도달했다. 회차는 완료로 바꾸지 않았고 blocked handoff를 남겼다",
                progress.no_progress
            ))
        }
        Ok(progress) => PolicyDecision::Block(block_reason(&view, progress.no_progress)),
        Err(error) => PolicyDecision::Block(format!(
            "Stop 진행 상태를 안전하게 기록하지 못했다: {error}"
        )),
    }
}

fn canonical_repo(repo: &Path) -> Result<PathBuf> {
    let repo = repo
        .canonicalize()
        .with_context(|| format!("repo `{}`", repo.display()))?;
    approval::repository_root_identity(&repo).map_err(anyhow::Error::from)?;
    Ok(repo)
}

fn activation_state(repo: &Path, requested: Option<&Path>) -> ActivationState {
    let project = match approval::repository_root_identity(repo) {
        Ok(project) => project,
        Err(error) => return ActivationState::Corrupt(error.to_string()),
    };
    let store = match approval::store_location(requested) {
        Ok(store) => store,
        Err(error) => return ActivationState::Corrupt(error.to_string()),
    };
    if !store.is_dir() {
        return ActivationState::Inactive;
    }
    let path = activation_path(&store, &project);
    if !path.exists() {
        return ActivationState::Inactive;
    }
    let activation: Activation = match read_json(&path) {
        Ok(record) => record,
        Err(error) => return ActivationState::Corrupt(error.to_string()),
    };
    let expected = activation_digest(&project, &activation.round);
    if activation.version != ACTIVATION_VERSION
        || activation.project != project
        || activation.policy != POLICY
        || activation.no_progress_limit != NO_PROGRESS_LIMIT
        || activation.digest != expected
    {
        return ActivationState::Corrupt("activation identity가 현재 policy와 다르다".to_owned());
    }
    ActivationState::Active(activation, path)
}

fn activation_digest(project: &str, slug: &str) -> String {
    digest_values(
        ACTIVATION_DOMAIN,
        &[project, slug, POLICY, &NO_PROGRESS_LIMIT.to_string()],
    )
}

fn activation_path(store: &Path, project: &str) -> PathBuf {
    store.join(format!("round-stop-activation-{project}.json"))
}

fn progress_path(store: &Path, activation_digest: &str) -> PathBuf {
    store.join(format!("round-stop-progress-{activation_digest}.json"))
}

fn stable_status(repo: &Path, slug: &str) -> Result<StatusView> {
    for _ in 0..3 {
        let first = read_status(repo, slug)?;
        let first_digest = semantic_digest(&first);
        let second = read_status(repo, slug)?;
        if first_digest == semantic_digest(&second) {
            return Ok(second);
        }
    }
    bail!("동시 갱신 중 안정된 round snapshot을 얻지 못했다")
}

fn read_status(repo: &Path, slug: &str) -> Result<StatusView> {
    match status::read(repo, Some(slug)) {
        Ok(status::Outcome::Status(view)) => Ok(view),
        Ok(status::Outcome::NoActiveRound) => bail!("round `{slug}`가 없다"),
        Err(error) => Err(anyhow::anyhow!(error)),
    }
}

fn semantic_digest(view: &StatusView) -> String {
    let mut values = vec![
        view.round.clone(),
        format!("{:?}", view.verification),
        format!("{:?}", view.terminal),
    ];
    for condition in &view.conditions {
        values.push(condition.id.clone());
        values.push(format!("{:?}", condition.state));
        values.push(condition.oracle_digest.clone().unwrap_or_default());
    }
    let refs: Vec<&str> = values.iter().map(String::as_str).collect();
    digest_values(SEMANTIC_DOMAIN, &refs)
}

fn progress_rank(view: &StatusView) -> ProgressRank {
    let mut rank = ProgressRank {
        terminal: u32::from(view.terminal != Terminal::Open),
        ..ProgressRank::default()
    };
    for condition in &view.conditions {
        if condition.state != ConditionState::Unregistered {
            rank.registered += 1;
        }
        if matches!(condition.state, ConditionState::Met | ConditionState::Unmet) {
            rank.observed += 1;
        }
        if condition.state == ConditionState::Met {
            rank.met += 1;
        }
    }
    rank
}

fn event_hash(session_id: &str, transcript: &Path) -> Result<String> {
    let metadata =
        symlink_metadata(transcript).with_context(|| "transcript metadata를 읽지 못했다")?;
    if !metadata.is_file() || metadata.file_type().is_symlink() {
        bail!("transcript가 regular file이 아니다");
    }
    if metadata.len() > TRANSCRIPT_MAX_BYTES {
        bail!("transcript가 8 MiB 상한을 넘었다");
    }
    let bytes = std::fs::read(transcript).with_context(|| "transcript를 읽지 못했다")?;
    let transcript_digest = blake3::hash(&bytes).to_hex().to_string();
    Ok(digest_values(
        EVENT_DOMAIN,
        &[session_id, &transcript_digest],
    ))
}

fn record_attempt(
    path: &Path,
    activation: &Activation,
    semantic_digest: &str,
    rank: ProgressRank,
    event_hash: &str,
) -> Result<Progress> {
    let _lease = Lease::acquire(&path.with_extension("lock"))?;
    let existing = read_progress(path)?;
    let mut progress = match existing {
        None => Progress {
            version: PROGRESS_VERSION,
            activation_digest: activation.digest.clone(),
            semantic_digest: semantic_digest.to_owned(),
            best: rank,
            no_progress: 0,
            event_hashes: Vec::new(),
            handoff: None,
        },
        Some(progress)
            if progress.version == PROGRESS_VERSION
                && progress.activation_digest == activation.digest =>
        {
            progress
        }
        Some(_) => bail!("progress identity가 현재 activation과 다르다"),
    };
    if progress.event_hashes.iter().any(|seen| seen == event_hash) {
        return Ok(progress);
    }
    if rank.advances(progress.best) {
        progress.best = rank;
        progress.no_progress = 1;
        progress.event_hashes.clear();
        progress.handoff = None;
    } else {
        progress.no_progress = progress.no_progress.saturating_add(1);
    }
    progress.semantic_digest = semantic_digest.to_owned();
    progress.event_hashes.push(event_hash.to_owned());
    if progress.event_hashes.len() > EVENT_HISTORY_MAX {
        let remove = progress.event_hashes.len() - EVENT_HISTORY_MAX;
        progress.event_hashes.drain(..remove);
    }
    if progress.no_progress >= activation.no_progress_limit {
        progress.no_progress = activation.no_progress_limit;
        progress.handoff = Some("blocked".to_owned());
    }
    write_private_json(path, &progress)?;
    Ok(progress)
}

fn read_progress(path: &Path) -> Result<Option<Progress>> {
    if !path.exists() {
        return Ok(None);
    }
    let progress: Progress = read_json(path)?;
    if progress.version != PROGRESS_VERSION {
        bail!("알 수 없는 progress version {}", progress.version);
    }
    Ok(Some(progress))
}

fn block_reason(view: &StatusView, no_progress: u32) -> String {
    let mut not_met: Vec<String> = view
        .conditions
        .iter()
        .filter(|condition| condition.state != ConditionState::Met)
        .take(16)
        .map(|condition| format!("{}={:?}", condition.id, condition.state).to_lowercase())
        .collect();
    if view
        .conditions
        .iter()
        .filter(|condition| condition.state != ConditionState::Met)
        .count()
        > not_met.len()
    {
        not_met.push("…".to_owned());
    }
    format!(
        "round `{}`가 아직 종료 조건을 충족하지 않았다: terminal={:?}, verification={:?}, conditions=[{}]. 무진행 {}/{}. 상태를 실제로 진척시키거나 `pal round stop disable`로 복구하라",
        view.round,
        view.terminal,
        view.verification,
        not_met.join(", "),
        no_progress,
        NO_PROGRESS_LIMIT,
    )
}

fn digest_values(domain: &[u8], values: &[&str]) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(domain);
    for value in values {
        hasher.update(&(value.len() as u64).to_le_bytes());
        hasher.update(value.as_bytes());
    }
    hasher.finalize().to_hex().to_string()
}

fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T> {
    let metadata =
        symlink_metadata(path).with_context(|| format!("{} metadata", path.display()))?;
    if !metadata.is_file() || metadata.file_type().is_symlink() {
        bail!("record가 regular file이 아니다");
    }
    let bytes = std::fs::read(path).with_context(|| format!("{} 읽기", path.display()))?;
    serde_json::from_slice(&bytes).with_context(|| "record JSON이 malformed다")
}

fn write_private_json(path: &Path, value: &impl Serialize) -> Result<()> {
    let parent = path.parent().context("private record parent가 없다")?;
    std::fs::create_dir_all(parent)?;
    if path.exists() {
        let metadata = symlink_metadata(path)?;
        if metadata.file_type().is_symlink() || !metadata.is_file() {
            bail!("private record target이 regular file이 아니다");
        }
    }
    let mut temporary = tempfile::NamedTempFile::new_in(parent)?;
    #[cfg(unix)]
    temporary
        .as_file()
        .set_permissions(std::os::unix::fs::PermissionsExt::from_mode(0o600))?;
    serde_json::to_writer(&mut temporary, value)?;
    temporary.write_all(b"\n")?;
    temporary.as_file().sync_all()?;
    let persisted = temporary.persist(path).map_err(|error| error.error)?;
    persisted.sync_all()?;
    approval::private_file(path).map_err(anyhow::Error::from)?;
    Ok(())
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct LeaseRecord {
    token: String,
    created_millis: u64,
}

struct Lease {
    path: PathBuf,
    token: String,
}

impl Lease {
    fn acquire(path: &Path) -> Result<Self> {
        let token = format!("{}-{}", std::process::id(), now_millis());
        let started = now_millis();
        loop {
            let record = LeaseRecord {
                token: token.clone(),
                created_millis: now_millis(),
            };
            match OpenOptions::new().write(true).create_new(true).open(path) {
                Ok(mut file) => {
                    #[cfg(unix)]
                    file.set_permissions(std::os::unix::fs::PermissionsExt::from_mode(0o600))?;
                    serde_json::to_writer(&mut file, &record)?;
                    file.write_all(b"\n")?;
                    file.sync_all()?;
                    return Ok(Self {
                        path: path.to_path_buf(),
                        token,
                    });
                }
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                    if stale_lease(path)? {
                        let _ = std::fs::remove_file(path);
                        continue;
                    }
                    if now_millis().saturating_sub(started) >= LOCK_WAIT_MILLIS {
                        bail!("progress lock을 2초 안에 얻지 못했다");
                    }
                    std::thread::sleep(Duration::from_millis(10));
                }
                Err(error) => return Err(error).context("progress lock 생성"),
            }
        }
    }
}

impl Drop for Lease {
    fn drop(&mut self) {
        let owned = read_json::<LeaseRecord>(&self.path)
            .ok()
            .is_some_and(|record| record.token == self.token);
        if owned {
            let _ = std::fs::remove_file(&self.path);
        }
    }
}

fn stale_lease(path: &Path) -> Result<bool> {
    let record: LeaseRecord = match read_json(path) {
        Ok(record) => record,
        Err(_) => {
            let modified = symlink_metadata(path)?
                .modified()
                .unwrap_or(SystemTime::now());
            return Ok(SystemTime::now()
                .duration_since(modified)
                .unwrap_or_default()
                .as_millis()
                > u128::from(LOCK_STALE_MILLIS));
        }
    };
    Ok(now_millis().saturating_sub(record.created_millis) > LOCK_STALE_MILLIS)
}

fn now_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        .try_into()
        .unwrap_or(u64::MAX)
}

fn print_view(json: bool, value: &impl Serialize, human: &str) {
    if json {
        println!(
            "{}",
            serde_json::to_string(value).expect("serializable Stop view")
        );
    } else {
        println!("{human}");
    }
}
