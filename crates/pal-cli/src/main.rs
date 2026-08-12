//! `pal` — 1급 표면.
//!
//! S0 이 뚫은 것: `pal symbols <파일>` — blob 하나 → tree-sitter → 심볼 목록.
//! S1 이 뚫는 것: `pal ledger` — 저장소 하나 → git 접근 · 분류 · 캐시 → 관측 범위 대장.

#![forbid(unsafe_code)]

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use pal_core::{Capable, Language};

mod bind;
mod defect;
mod ledger;
mod touch;

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
    /// 조각 하나를 좌표에 손으로 건다 — **사람이 넣는 자리**
    Bind {
        /// 심볼 이름
        name: String,
        /// 걸 조각
        #[arg(long)]
        note: String,
        /// 저장소 경로
        #[arg(long, default_value = ".")]
        repo: PathBuf,
        /// 어느 커밋인가
        #[arg(long)]
        at: Option<String>,
        #[arg(long)]
        cache_dir: Option<PathBuf>,
        #[arg(long)]
        index: Option<PathBuf>,
        /// 의도 저장소 위치. 기본값은 `<저장소>/.palimpsest/intent.redb`
        #[arg(long)]
        intent: Option<PathBuf>,
    },
    /// 수정 커밋 하나에서 결함을 소급 결박한다 — **못 담은 것도 센다**
    Defect {
        /// 수정 커밋
        rev: String,
        /// 저장소 경로
        #[arg(long, default_value = ".")]
        repo: PathBuf,
        /// 이력을 얼마나 거슬러 올라가는가. 걸리면 그 사실이 산출에 남는다
        #[arg(long)]
        history_limit: Option<usize>,
        /// 사람이 읽는 화면 대신 JSON 으로 낸다
        #[arg(long)]
        json: bool,
    },
    /// 좌표 하나를 만진다 — **빈 답도 정직하게 낸다**
    Touch {
        /// 심볼 이름
        name: String,
        /// 저장소 경로. 기본값은 현재 디렉터리
        #[arg(long, default_value = ".")]
        repo: PathBuf,
        /// 어느 커밋인가. 기본값은 HEAD
        #[arg(long)]
        at: Option<String>,
        /// 1층 캐시 위치
        #[arg(long)]
        cache_dir: Option<PathBuf>,
        /// 2층 인덱스 위치. 기본값은 `<저장소>/.palimpsest/index.redb`
        #[arg(long)]
        index: Option<PathBuf>,
        /// 의도 저장소 위치. 기본값은 `<저장소>/.palimpsest/intent.redb`
        #[arg(long)]
        intent: Option<PathBuf>,
        /// 사람이 읽는 화면 대신 JSON 으로 낸다
        #[arg(long)]
        json: bool,
    },
    /// 저장소 하나의 관측 범위 대장을 낸다 — **무엇을 보았고 무엇을 보지 않았는가**
    Ledger {
        /// 저장소 경로. 기본값은 현재 디렉터리
        #[arg(default_value = ".")]
        path: PathBuf,
        /// 어느 커밋인가. 기본값은 HEAD
        #[arg(long)]
        at: Option<String>,
        /// 1층 캐시 위치. 기본값은 `<저장소>/.palimpsest/cache`
        #[arg(long)]
        cache_dir: Option<PathBuf>,
        /// 사람이 읽는 표 대신 JSON 으로 낸다
        #[arg(long)]
        json: bool,
    },
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Command::Symbols { path, json } => symbols(&path, json),
        Command::Bind { name, note, repo, at, cache_dir, index, intent } => {
            bind::run(&repo, at.as_deref(), cache_dir, index, intent, &name, &note)
        }
        Command::Defect { rev, repo, history_limit, json } => {
            let report = defect::run(
                &rev,
                &repo,
                history_limit.unwrap_or_else(defect::default_budget),
            )?;
            if json {
                defect::print_json(&report)
            } else {
                defect::print(&report);
                Ok(())
            }
        }
        Command::Touch { name, repo, at, cache_dir, index, intent, json } => {
            touch::run(&repo, at.as_deref(), cache_dir, index, intent, &name, json)
        }
        Command::Ledger { path, at, cache_dir, json } => {
            let report = ledger::compute(&path, at.as_deref(), cache_dir)?;
            if json {
                println!("{}", serde_json::to_string_pretty(&report)?);
            } else {
                ledger::print_table(&report);
            }
            Ok(())
        }
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
