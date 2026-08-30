//! 읽기 전용 회차 verification 상태 표면.

pub mod ledger;
pub mod status;

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
