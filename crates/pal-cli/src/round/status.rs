//! verification ledger를 조건·회차 상태로 축약한다.

use std::collections::BTreeSet;
use std::path::Path;

use pal_intent::round_condition::ConditionsReport;
use serde::Serialize;
use thiserror::Error;

use super::ledger::{self, LedgerError};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum VerificationState {
    Unregistered,
    InProgress,
    Met,
    #[allow(dead_code)]
    Invalid,
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

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ConditionView {
    pub id: String,
    pub state: ConditionState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oracle_digest: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StatusView {
    pub outcome: &'static str,
    pub round: String,
    pub verification: VerificationState,
    pub terminal: Terminal,
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

    let mut conditions = Vec::new();
    for id in ids {
        let state = ledger.as_ref().and_then(|l| l.conditions.get(&id));
        let (condition_state, digest) = match state.and_then(|s| s.oracle.as_ref().map(|o| (s, o)))
        {
            None => (ConditionState::Unregistered, None),
            Some((state, oracle)) => {
                let condition_state = match &state.evidence {
                    None if state.had_evidence_before_current_oracle => ConditionState::Stale,
                    None => ConditionState::Pending,
                    Some(evidence) if evidence.oracle_digest != oracle.digest => {
                        ConditionState::Stale
                    }
                    Some(evidence) if evidence.exit == 0 && evidence.matched => ConditionState::Met,
                    Some(_) => ConditionState::Unmet,
                };
                (condition_state, Some(oracle.digest.clone()))
            }
        };
        conditions.push(ConditionView {
            id,
            state: condition_state,
            oracle_digest: digest,
        });
    }
    let verification = aggregate(&conditions, ledger.is_some());
    Ok(StatusView {
        outcome: "status",
        round: slug.to_owned(),
        verification,
        terminal,
        conditions,
    })
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
    for condition in &view.conditions {
        println!("{}: {}", condition.id, condition_name(condition.state));
    }
}

fn verification_name(state: VerificationState) -> &'static str {
    match state {
        VerificationState::Unregistered => "unregistered",
        VerificationState::InProgress => "in_progress",
        VerificationState::Met => "met",
        VerificationState::Invalid => "invalid",
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
