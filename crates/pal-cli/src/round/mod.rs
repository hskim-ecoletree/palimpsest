//! 읽기 전용 회차 verification 상태 표면.

pub mod approval;
pub mod ledger;
pub mod stop;
pub mod status;
pub mod verify;

use std::io::Write;
use std::path::Path;

use anyhow::Result;
use pal_intent::round_condition::ConditionsReport;

pub fn conditions(file: &Path, json: bool) -> Result<()> {
    let display = file.to_string_lossy().to_string();
    let body = match std::fs::read_to_string(file) {
        Ok(body) => body,
        Err(error) => exit_error(json, 2, "io_error", &format!("{}: {error}", file.display())),
    };
    let report = ConditionsReport::parse(display, &body);
    if json {
        write_stdout(&serde_json::to_string_pretty(&report)?);
    } else {
        for condition in &report.conditions {
            let id = condition.id.as_ref().map_or("—", |id| id.as_str());
            println!(
                "{} {} {}",
                if condition.checked { "[x]" } else { "[ ]" },
                id,
                condition.errors.join(" · ")
            );
        }
    }
    if !report.is_valid() {
        std::process::exit(1);
    }
    Ok(())
}

pub fn round_status(round: Option<&str>, json: bool) -> Result<()> {
    match status::read(Path::new("."), round) {
        Ok(status::Outcome::NoActiveRound) => {
            if json {
                write_stdout(r#"{"outcome":"no_active_round"}"#);
            } else {
                println!("active round 없음");
            }
        }
        Ok(status::Outcome::Status(view)) => {
            if json {
                write_stdout(&serde_json::to_string(&view)?);
            } else {
                status::print_human(&view);
            }
        }
        Err(error) => exit_error(json, 2, error.code(), &error.to_string()),
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub fn round_approve(
    repo: &Path,
    slug: &str,
    id: &str,
    approval_dir: Option<&Path>,
    shell: Option<&Path>,
    timeout_secs: u64,
    output_limit: usize,
    json: bool,
) -> Result<()> {
    let config = verify::Config {
        repo,
        slug,
        id,
        approval_dir,
        shell,
        timeout_secs,
        output_limit,
    };
    match verify::approve(&config) {
        Ok(view) => print_view(json, &view, &format!("approved: {slug}/{id}")),
        Err(error) => verify_error(json, error),
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub fn round_verify(
    repo: &Path,
    slug: &str,
    id: &str,
    approval_dir: Option<&Path>,
    shell: Option<&Path>,
    timeout_secs: u64,
    output_limit: usize,
    json: bool,
) -> Result<()> {
    let config = verify::Config {
        repo,
        slug,
        id,
        approval_dir,
        shell,
        timeout_secs,
        output_limit,
    };
    match verify::verify(&config) {
        Ok(view) => {
            let met = view.met;
            print_view(json, &view, &format!("verified: {slug}/{id} met={met}"));
            if !met {
                std::process::exit(1);
            }
        }
        Err(error) => verify_error(json, error),
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub fn round_finalize(
    repo: &Path,
    slug: &str,
    approval_dir: Option<&Path>,
    shell: Option<&Path>,
    timeout_secs: u64,
    output_limit: usize,
    json: bool,
) -> Result<()> {
    let config = verify::Config {
        repo,
        slug,
        id: "",
        approval_dir,
        shell,
        timeout_secs,
        output_limit,
    };
    match verify::finalize(&config) {
        Ok(view) => print_view(json, &view, &format!("finalized: {slug} complete")),
        Err(error) => verify_error(json, error),
    }
    Ok(())
}

fn print_view(json: bool, value: &impl serde::Serialize, human: &str) {
    if json {
        write_stdout(&serde_json::to_string(value).expect("serializable view"));
    } else {
        println!("{human}");
    }
}

fn verify_error(json: bool, error: verify::VerifyError) -> ! {
    let (exit, outcome, code) = match &error {
        verify::VerifyError::ApprovalRequired => (3, "approval_required", "approval_required"),
        verify::VerifyError::Discarded(_) => (3, "discarded", "currentness_changed"),
        verify::VerifyError::Invalid(_) => (2, "invalid", "invalid_schema"),
        verify::VerifyError::Io(_) => (2, "invalid", "io_error"),
    };
    if json {
        let value = serde_json::json!({
            "outcome": outcome,
            "code": code,
            "message": error.to_string(),
        });
        write_stdout(&value.to_string());
    } else {
        eprintln!("{code}: {error}");
    }
    std::process::exit(exit)
}

fn write_stdout(text: &str) {
    let mut stdout = std::io::stdout().lock();
    writeln!(stdout, "{text}").expect("stdout");
}

fn exit_error(json: bool, code: i32, kind: &str, message: &str) -> ! {
    if json {
        let value = serde_json::json!({
            "outcome": "invalid",
            "code": kind,
            "message": message,
        });
        write_stdout(&value.to_string());
    } else {
        eprintln!("{kind}: {message}");
    }
    std::process::exit(code)
}
