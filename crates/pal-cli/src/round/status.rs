//! verification ledger를 조건·회차 상태로 축약한다.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use pal_intent::round_condition::ConditionsReport;
use pal_core::ROUND_VERIFICATION_FILE_MAX_BYTES;
use serde::Serialize;
use thiserror::Error;

use super::ledger::{self, LedgerError};
use super::approval;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum VerificationState {
    Unregistered,
    InProgress,
    Met,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConditionState {
    Unregistered,
    Pending,
    Stale,
    Met,
    Unmet,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Terminal {
    Open,
    Reported,
    Folded,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CompletionState {
    Unavailable,
    InProgress,
    Complete,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ConditionView {
    pub id: String,
    pub state: ConditionState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oracle_digest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_digest: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StatusView {
    pub outcome: &'static str,
    pub round: String,
    pub verification: VerificationState,
    pub terminal: Terminal,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<u32>,
    pub completion: CompletionState,
    pub findings_current: bool,
    pub open_harmful_findings: usize,
    pub terminal_document_current: bool,
    pub aggregate_digest: String,
    pub conditions: Vec<ConditionView>,
}

pub enum Outcome {
    NoActiveRound,
    Status(StatusView),
}

#[derive(Debug, Error)]
pub enum StatusError {
    #[error("schema 오류: {0}")]
    InvalidSchema(String),
    #[error("상태 전이 오류: {0}")]
    InvalidTransition(String),
    #[error("회차 해소 오류: {0}")]
    Resolve(String),
    #[error("I/O 오류: {0}")]
    Io(String),
}

impl StatusError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::InvalidSchema(_) => "invalid_schema",
            Self::InvalidTransition(_) => "invalid_transition",
            Self::Resolve(_) => "resolve_error",
            Self::Io(_) => "io_error",
        }
    }
}

impl From<LedgerError> for StatusError {
    fn from(value: LedgerError) -> Self {
        match value {
            LedgerError::Io(error) => Self::Io(error.to_string()),
            LedgerError::Schema(message) => Self::InvalidSchema(message),
            LedgerError::Transition(message) => Self::InvalidTransition(message),
        }
    }
}

pub fn read(repo: &Path, requested: Option<&str>) -> Result<Outcome, StatusError> {
    let rounds = repo.join(".palimpsest/rounds");
    let slug = match requested {
        Some(slug) => {
            if !valid_slug(slug) {
                return Err(StatusError::Resolve(format!(
                    "유효하지 않은 round slug `{slug}`"
                )));
            }
            let dir = rounds.join(slug);
            if !dir.is_dir() {
                return Err(StatusError::Resolve(format!("round `{slug}`가 없다")));
            }
            slug.to_owned()
        }
        None => match resolve_active(&rounds)? {
            Some(slug) => slug,
            None => return Ok(Outcome::NoActiveRound),
        },
    };
    read_round(&rounds.join(&slug), &slug).map(Outcome::Status)
}

fn resolve_active(rounds: &Path) -> Result<Option<String>, StatusError> {
    if !rounds.is_dir() {
        return Ok(None);
    }
    let entries = std::fs::read_dir(rounds).map_err(|e| StatusError::Io(e.to_string()))?;
    let mut candidates = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|e| StatusError::Io(e.to_string()))?;
        let dir = entry.path();
        if !dir.is_dir() || !dir.join("verification.log").is_file() {
            continue;
        }
        terminal(&dir)?;
        if !dir.join("report.md").is_file() && !dir.join("folded.md").is_file() {
            candidates.push(entry.file_name().to_string_lossy().to_string());
        }
    }
    candidates.sort();
    match candidates.as_slice() {
        [] => Ok(None),
        [slug] => Ok(Some(slug.clone())),
        _ => Err(StatusError::Resolve(format!(
            "active round 후보가 {}개다: {}",
            candidates.len(),
            candidates.join(" · ")
        ))),
    }
}

fn read_round(dir: &Path, slug: &str) -> Result<StatusView, StatusError> {
    let terminal = terminal(dir)?;
    let intent_path = dir.join("intent.md");
    if !intent_path.is_file() {
        return Err(StatusError::Resolve(format!("{slug}/intent.md가 없다")));
    }
    let body = std::fs::read_to_string(&intent_path).map_err(|e| StatusError::Io(e.to_string()))?;
    let report = ConditionsReport::parse(intent_path.to_string_lossy(), &body);
    if !report.is_valid() {
        return Err(StatusError::InvalidSchema(format!(
            "intent condition 형식 오류 {}건",
            report.error_count
        )));
    }
    let mut ids = BTreeSet::new();
    for condition in &report.conditions {
        let Some(id) = &condition.id else {
            return Err(StatusError::InvalidSchema(
                "intent condition ID가 없다".to_owned(),
            ));
        };
        ids.insert(id.as_str().to_owned());
    }

    let ledger_path = dir.join("verification.log");
    let ledger = if ledger_path.is_file() {
        Some(ledger::read(&ledger_path, slug)?)
    } else {
        None
    };
    if let Some(ledger) = &ledger {
        let unknown: Vec<&str> = ledger
            .conditions
            .keys()
            .filter(|id| !ids.contains(id.as_str()))
            .map(String::as_str)
            .collect();
        if !unknown.is_empty() {
            return Err(StatusError::InvalidSchema(format!(
                "intent 밖 oracle ID: {}",
                unknown.join(" · ")
            )));
        }
    }

    let projected = match &ledger {
        Some(ledger) if ledger.schema_version >= 2 => Some(
            super::verify::projected_digest(
                dir.parent()
                    .and_then(Path::parent)
                    .and_then(Path::parent)
                    .ok_or_else(|| {
                        StatusError::Resolve("repository root를 해소하지 못했다".to_owned())
                    })?,
                slug,
            )
            .map_err(|error| StatusError::Io(error.to_string()))?,
        ),
        _ => None,
    };
    let repo = dir
        .parent()
        .and_then(Path::parent)
        .and_then(Path::parent)
        .ok_or_else(|| StatusError::Resolve("repository root를 해소하지 못했다".to_owned()))?;
    let mut raw = BTreeMap::new();
    for id in &ids {
        let state = ledger.as_ref().and_then(|l| l.conditions.get(id.as_str()));
        let (condition_state, digest, evidence_digest) = match state
            .and_then(|s| s.oracle.as_ref().map(|o| (s, o)))
        {
            None => match state.and_then(|state| state.judgment.as_ref()) {
                None => (ConditionState::Unregistered, None, None),
                Some(judgment) => {
                    let current = judgment_current(repo, judgment)?;
                    let state = if !current {
                        ConditionState::Stale
                    } else if judgment.met {
                        ConditionState::Met
                    } else {
                        ConditionState::Unmet
                    };
                    (
                        state,
                        None,
                        Some(judgment_digest(judgment)),
                    )
                }
            },
            Some((state, oracle)) => {
                let condition_state = match &state.evidence {
                    None if state.had_evidence_before_current_oracle => ConditionState::Stale,
                    None => ConditionState::Pending,
                    Some(evidence) if evidence.oracle_digest != oracle.digest => {
                        ConditionState::Stale
                    }
                    Some(evidence)
                        if evidence.projected_digest.as_ref().is_some_and(|digest| {
                            projected.as_ref().is_some_and(|current| digest != current)
                        }) =>
                    {
                        ConditionState::Stale
                    }
                    Some(evidence) if evidence.exit == 0 && evidence.matched => ConditionState::Met,
                    Some(_) => ConditionState::Unmet,
                };
                (
                    condition_state,
                    Some(oracle.digest.clone()),
                    state.evidence.as_ref().map(|evidence| {
                        let mut hasher = blake3::Hasher::new();
                        hasher.update(b"pal.round.command-evidence.v1\0");
                        hasher.update(evidence.oracle_digest.as_bytes());
                        hasher.update(&evidence.exit.to_le_bytes());
                        hasher.update(&[u8::from(evidence.matched)]);
                        if let Some(projected) = &evidence.projected_digest {
                            hasher.update(projected.as_bytes());
                        }
                        hasher.finalize().to_hex().to_string()
                    }),
                )
            }
        };
        raw.insert(id.clone(), (condition_state, digest, evidence_digest));
    }
    if let Some(ledger) = &ledger {
        let mut controls: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
        for (id, state) in &ledger.conditions {
            if let Some(base) = state
                .oracle
                .as_ref()
                .and_then(|oracle| oracle.negative_for.as_deref())
            {
                controls.entry(base).or_default().push(id);
            }
        }
        for (base, control_ids) in controls {
            if raw.get(base).map(|(state, _, _)| *state) != Some(ConditionState::Met) {
                continue;
            }
            let mut replacement = None;
            for control in control_ids {
                let control_state = raw
                    .get(control)
                    .map_or(ConditionState::Unregistered, |(state, _, _)| *state);
                if control_state != ConditionState::Met {
                    replacement = Some(match control_state {
                        ConditionState::Unmet => ConditionState::Unmet,
                        ConditionState::Stale => ConditionState::Stale,
                        ConditionState::Pending | ConditionState::Unregistered => {
                            ConditionState::Pending
                        }
                        ConditionState::Met => unreachable!(),
                    });
                    break;
                }
            }
            if let Some(replacement) = replacement {
                raw.get_mut(base).expect("base exists").0 = replacement;
            }
        }
    }
    let mut conditions = Vec::new();
    for id in ids {
        let (condition_state, digest, evidence_digest) =
            raw.remove(&id).expect("all intent ids reduced");
        conditions.push(ConditionView {
            id,
            state: condition_state,
            oracle_digest: digest,
            evidence_digest,
        });
    }
    let verification = aggregate(&conditions, ledger.is_some());
    let (findings_current, open_harmful_findings) = findings_state(dir)?;
    let (terminal_document_current, terminal_document_digest) =
        terminal_document_state(repo, slug, terminal);
    let schema_version = ledger.as_ref().map(|ledger| ledger.schema_version);
    let aggregate_digest = aggregate_digest(
        slug,
        schema_version,
        terminal,
        verification,
        &conditions,
        findings_current,
        open_harmful_findings,
        terminal_document_current,
        &terminal_document_digest,
    );
    let completion = match (&ledger, &projected) {
        (Some(ledger), Some(projected))
            if ledger.schema_version == 3
                && terminal == Terminal::Reported
                && terminal_document_current
                && verification == VerificationState::Met
                && findings_current
                && open_harmful_findings == 0
                && ledger.checkpoint.as_ref().is_some_and(|checkpoint| {
                    checkpoint.projected_digest == *projected
                        && checkpoint.aggregate_digest == aggregate_digest
                        && checkpoint.finalization_seal
                            == approval::finalization_digest(
                                repo,
                                slug,
                                projected,
                                &aggregate_digest,
                            )
                            .unwrap_or_default()
                        && approval::store_location(None)
                            .ok()
                            .is_some_and(|store| {
                                approval::is_approved(
                                    &store,
                                    &checkpoint.finalization_seal,
                                )
                                .unwrap_or(false)
                            })
                }) =>
        {
            CompletionState::Complete
        }
        (Some(ledger), _) if ledger.schema_version == 3 => CompletionState::InProgress,
        _ => CompletionState::Unavailable,
    };
    Ok(StatusView {
        outcome: "status",
        round: slug.to_owned(),
        verification,
        terminal,
        schema_version,
        completion,
        findings_current,
        open_harmful_findings,
        terminal_document_current,
        aggregate_digest,
        conditions,
    })
}

fn judgment_current(repo: &Path, judgment: &ledger::Judgment) -> Result<bool, StatusError> {
    for evidence in [
        &judgment.thesis,
        &judgment.antithesis,
        &judgment.synthesis,
    ] {
        let path = repo.join(&evidence.path);
        let metadata = match std::fs::symlink_metadata(&path) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(false),
            Err(error) => return Err(StatusError::Io(format!("{}: {error}", evidence.path))),
        };
        if !metadata.is_file() || metadata.file_type().is_symlink() {
            return Ok(false);
        }
        let bytes = std::fs::read(&path).map_err(|error| StatusError::Io(error.to_string()))?;
        if blake3::hash(&bytes).to_hex().as_str() != evidence.digest {
            return Ok(false);
        }
    }
    Ok(true)
}

fn judgment_digest(judgment: &ledger::Judgment) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"pal.round.dialectic-evidence.v1\0");
    hasher.update(&[u8::from(judgment.met)]);
    for evidence in [
        &judgment.thesis,
        &judgment.antithesis,
        &judgment.synthesis,
    ] {
        hasher.update(evidence.path.as_bytes());
        hasher.update(evidence.digest.as_bytes());
    }
    hasher.finalize().to_hex().to_string()
}

fn findings_state(dir: &Path) -> Result<(bool, usize), StatusError> {
    let path = dir.join("findings.jsonl");
    if !path.is_file() {
        return Ok((false, 0));
    }
    let body = std::fs::read_to_string(&path).map_err(|error| StatusError::Io(error.to_string()))?;
    let mut lines = body.lines();
    let Some(header) = lines.next() else {
        return Ok((false, 0));
    };
    let header: serde_json::Value = serde_json::from_str(header)
        .map_err(|error| StatusError::InvalidSchema(format!("findings header: {error}")))?;
    if header.get("schema_version").and_then(serde_json::Value::as_u64) != Some(3)
        || header.get("종류").and_then(serde_json::Value::as_str) != Some("레코드")
        || header.get("회차").and_then(serde_json::Value::as_str)
            != dir.file_name().and_then(std::ffi::OsStr::to_str)
    {
        return Ok((false, 0));
    }
    let mut harmful = 0;
    for (index, line) in lines.enumerate() {
        let row: serde_json::Value = serde_json::from_str(line).map_err(|error| {
            StatusError::InvalidSchema(format!("findings {}행: {error}", index + 2))
        })?;
        let required_strings = [
            "id", "출처", "모집단", "유효성", "해악도", "처분", "경로", "요약",
        ];
        if row.get("라운드").and_then(serde_json::Value::as_u64).is_none()
            || required_strings.iter().any(|field| {
                row.get(*field)
                    .and_then(serde_json::Value::as_str)
                    .is_none_or(str::is_empty)
            })
        {
            return Ok((false, 0));
        }
        if !matches!(
            row.get("출처").and_then(serde_json::Value::as_str),
            Some("독립리뷰" | "사전부검" | "인터뷰" | "실측")
        ) || !matches!(
            row.get("모집단").and_then(serde_json::Value::as_str),
            Some("원의도" | "저장소" | "자기장치" | "회차기록" | "규약")
        ) || !matches!(
            row.get("유효성").and_then(serde_json::Value::as_str),
            Some("참" | "추정" | "거짓")
        ) || !matches!(
            row.get("처분").and_then(serde_json::Value::as_str),
            Some("정정" | "확대" | "축소" | "전환" | "범위밖" | "기각")
        ) {
            return Ok((false, 0));
        }
        let state = row.get("상태").and_then(serde_json::Value::as_str);
        let severity = row.get("해악도").and_then(serde_json::Value::as_str);
        if !matches!(state, Some("열림" | "닫힘"))
            || !matches!(severity, Some("금지역" | "실패" | "거짓신호" | "미관"))
        {
            return Ok((false, 0));
        }
        if state == Some("닫힘")
            && row
                .get("닫은커밋")
                .and_then(serde_json::Value::as_str)
                .is_none_or(str::is_empty)
        {
            return Ok((false, 0));
        }
        if state == Some("열림") && matches!(severity, Some("금지역" | "실패")) {
            harmful += 1;
        }
    }
    Ok((true, harmful))
}

#[allow(clippy::too_many_arguments)]
fn aggregate_digest(
    slug: &str,
    schema_version: Option<u32>,
    terminal: Terminal,
    verification: VerificationState,
    conditions: &[ConditionView],
    findings_current: bool,
    harmful: usize,
    terminal_document_current: bool,
    terminal_document_digest: &str,
) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"pal.round.completion-aggregate.v1\0");
    for value in [
        slug.to_owned(),
        schema_version.map_or_else(String::new, |value| value.to_string()),
        format!("{terminal:?}"),
        format!("{verification:?}"),
        findings_current.to_string(),
        harmful.to_string(),
        terminal_document_current.to_string(),
        terminal_document_digest.to_owned(),
    ] {
        hasher.update(value.as_bytes());
        hasher.update(&[0]);
    }
    for condition in conditions {
        hasher.update(condition.id.as_bytes());
        hasher.update(format!("{:?}", condition.state).as_bytes());
        hasher.update(condition.oracle_digest.as_deref().unwrap_or("").as_bytes());
        hasher.update(condition.evidence_digest.as_deref().unwrap_or("").as_bytes());
    }
    hasher.finalize().to_hex().to_string()
}

pub(crate) fn valid_terminal_document(
    repo: &Path,
    slug: &str,
    terminal: Terminal,
) -> Result<(), String> {
    let dir = repo.join(".palimpsest/rounds").join(slug);
    let (path, headings): (std::path::PathBuf, &[&str]) = match terminal {
        Terminal::Open => return Ok(()),
        Terminal::Reported => (
            dir.join("report.md"),
            &[
                "## 남지 않은 것",
                "## 다음 회차가 받는 것",
                "## 범위 밖",
                "## 원리상 못 잰 것",
                "## 능력 부재",
            ],
        ),
        Terminal::Folded => (
            dir.join("folded.md"),
            &[
                "## 왜 접었나",
                "## 접으면서 남기는 것과 버리는 것",
                "## 다음에 여는 것",
            ],
        ),
    };
    let metadata = std::fs::symlink_metadata(&path)
        .map_err(|error| format!("종료문 metadata를 읽지 못했다: {error}"))?;
    if !metadata.is_file() || metadata.file_type().is_symlink() {
        return Err("종료문이 regular file이 아니다".to_owned());
    }
    if metadata.len() > ROUND_VERIFICATION_FILE_MAX_BYTES {
        return Err("종료문이 8 MiB 상한을 넘었다".to_owned());
    }
    let body = std::fs::read_to_string(&path)
        .map_err(|error| format!("종료문을 읽지 못했다: {error}"))?;
    let lines: Vec<&str> = body.lines().collect();
    for heading in headings {
        let Some(index) = lines.iter().position(|line| line.trim_end() == *heading) else {
            return Err(format!("필수 절 `{heading}`이 없다"));
        };
        let has_body = lines[index + 1..]
            .iter()
            .take_while(|line| !line.starts_with("## "))
            .any(|line| {
                let line = line.trim();
                !line.is_empty() && !line.starts_with("<!--") && !line.ends_with("-->")
            });
        if !has_body {
            return Err(format!("필수 절 `{heading}`의 본문이 비었다"));
        }
    }
    if terminal == Terminal::Folded {
        let state = std::fs::read_to_string(dir.join("state.md"))
            .map_err(|error| format!("folded 회차의 state.md를 읽지 못했다: {error}"))?;
        if !state.contains("## 지금 단계")
            || !state.contains("접힘")
            || !state.contains("folded.md")
        {
            return Err("state.md가 접힘 단계와 folded.md를 가리키지 않는다".to_owned());
        }
    }
    Ok(())
}

fn terminal_document_state(repo: &Path, slug: &str, terminal: Terminal) -> (bool, String) {
    if terminal == Terminal::Open {
        return (true, String::new());
    }
    let name = if terminal == Terminal::Reported {
        "report.md"
    } else {
        "folded.md"
    };
    let path = repo.join(".palimpsest/rounds").join(slug).join(name);
    let digest = std::fs::read(path)
        .map(|bytes| blake3::hash(&bytes).to_hex().to_string())
        .unwrap_or_default();
    (valid_terminal_document(repo, slug, terminal).is_ok(), digest)
}

fn aggregate(conditions: &[ConditionView], has_ledger: bool) -> VerificationState {
    if !has_ledger
        || conditions
            .iter()
            .all(|c| c.state == ConditionState::Unregistered)
    {
        VerificationState::Unregistered
    } else if !conditions.is_empty() && conditions.iter().all(|c| c.state == ConditionState::Met) {
        VerificationState::Met
    } else {
        VerificationState::InProgress
    }
}

fn terminal(dir: &Path) -> Result<Terminal, StatusError> {
    let report = dir.join("report.md").is_file();
    let folded = dir.join("folded.md").is_file();
    match (report, folded) {
        (false, false) => Ok(Terminal::Open),
        (true, false) => Ok(Terminal::Reported),
        (false, true) => Ok(Terminal::Folded),
        (true, true) => Err(StatusError::InvalidTransition(
            "report.md와 folded.md가 함께 있다".to_owned(),
        )),
    }
}

fn valid_slug(value: &str) -> bool {
    !value.is_empty()
        && (value.as_bytes()[0].is_ascii_lowercase() || value.as_bytes()[0].is_ascii_digit())
        && value
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}

pub fn print_human(view: &StatusView) {
    println!("round: {}", view.round);
    println!("verification: {}", verification_name(view.verification));
    println!("terminal: {}", terminal_name(view.terminal));
    println!("completion: {}", completion_name(view.completion));
    println!("open harmful findings: {}", view.open_harmful_findings);
    for condition in &view.conditions {
        println!("{}: {}", condition.id, condition_name(condition.state));
    }
}

fn completion_name(state: CompletionState) -> &'static str {
    match state {
        CompletionState::Unavailable => "unavailable",
        CompletionState::InProgress => "in_progress",
        CompletionState::Complete => "complete",
    }
}

fn verification_name(state: VerificationState) -> &'static str {
    match state {
        VerificationState::Unregistered => "unregistered",
        VerificationState::InProgress => "in_progress",
        VerificationState::Met => "met",
    }
}

fn condition_name(state: ConditionState) -> &'static str {
    match state {
        ConditionState::Unregistered => "unregistered",
        ConditionState::Pending => "pending",
        ConditionState::Stale => "stale",
        ConditionState::Met => "met",
        ConditionState::Unmet => "unmet",
    }
}

fn terminal_name(terminal: Terminal) -> &'static str {
    match terminal {
        Terminal::Open => "open",
        Terminal::Reported => "reported",
        Terminal::Folded => "folded",
    }
}
