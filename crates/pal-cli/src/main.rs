//! `pal` — 1급 표면.
//!
//! S0 이 뚫는 것은 서브커맨드 하나다: `pal symbols <파일>`.
//! blob 하나 → tree-sitter → 심볼 목록.

#![forbid(unsafe_code)]

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use pal_core::{Capable, Language};

#[derive(Parser)]
#[command(name = "pal", version, about = "환경에 종속되지 않는 코드 이해의 큐레이터")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// 파일 하나의 최상위 심볼을 낸다
    Symbols {
        /// 대상 파일
        path: PathBuf,
        /// 사람이 읽는 표 대신 JSON 으로 낸다
        #[arg(long)]
        json: bool,
    },
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Command::Symbols { path, json } => symbols(&path, json),
    }
}

fn symbols(path: &Path, json: bool) -> Result<()> {
    let source = std::fs::read(path).with_context(|| format!("읽지 못했다: {}", path.display()))?;

    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    let Some(language) = Language::from_extension(ext) else {
        // **"언어를 모른다"와 "추출기가 없다"는 다르다.** 여기는 전자다.
        anyhow::bail!(
            "확장자 `.{ext}` 를 언어로 알지 못한다 — 아는 것은 Kotlin · Java · JavaScript · TypeScript 넷이다"
        );
    };

    match pal_extract::extract(language, &source) {
        Capable::Present(result) => {
            let found = result.with_context(|| format!("추출 실패: {}", path.display()))?;
            if json {
                println!("{}", serde_json::to_string_pretty(&found)?);
            } else {
                print_table(path, language, &found);
            }
        }
        Capable::NotBuilt { capability } => {
            // **빈 목록을 내지 않는다.** `Finding 0` 과 "안 만들었음"이 같은 출력이 되는 것을
            // 목표 §3.1 이 금지한다.
            if json {
                let not_built: Capable<Vec<pal_core::Symbol>> = Capable::NotBuilt { capability };
                println!("{}", serde_json::to_string_pretty(&not_built)?);
            } else {
                println!(
                    "  (이 빌드에 {} 추출 능력이 없습니다 — {} 미구축)",
                    language.name(),
                    capability.feature
                );
            }
        }
    }
    Ok(())
}

fn print_table(path: &Path, language: Language, found: &[pal_core::Symbol]) {
    let v = pal_extract::version();
    println!("{}  ·  {}", path.display(), language.name());
    println!("문법 {}  ·  추출기 {}", &v.grammar[..7], v.extractor);
    println!();
    if found.is_empty() {
        println!("  최상위 선언 없음");
    } else {
        for s in found {
            println!("  {:>5}  {:<10}  {}", s.span.line_start, s.kind.name(), s.name);
        }
    }
    println!();
    println!("  선언 {}", found.len());
}
