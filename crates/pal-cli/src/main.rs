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
mod doctor;
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
        /// 심볼 목록이 아니라 **파일 그래프 전부**를 JSON 으로 낸다.
        ///
        /// `--json` 의 형태를 건드리지 않는 이유: `scripts/s0-compare.py` 가 그것을
        /// **JSON 배열**로 파싱하고, 배열의 길이가 S0 대조의 선언 수다. 형태를 바꾸면
        /// 1,122 파일 대조가 깨진다.
        #[arg(long)]
        graph: bool,
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
    /// 저장된 그래프가 자기 규칙을 지키는지 본다 — **기본은 표본이고 전수는 명시적이다**
    Doctor {
        /// 저장소 경로. 기본값은 현재 디렉터리
        #[arg(long, default_value = ".")]
        repo: PathBuf,
        /// 어느 커밋인가. 기본값은 HEAD
        #[arg(long)]
        at: Option<String>,
        #[arg(long)]
        cache_dir: Option<PathBuf>,
        #[arg(long)]
        index: Option<PathBuf>,
        #[arg(long)]
        intent: Option<PathBuf>,
        /// **전수로 본다.** 기본은 표본이고 그 사실이 산출에 잔여로 실린다
        #[arg(long)]
        full: bool,
        /// 불변식마다 볼 단위 수의 상한. `--full` 과 함께 쓰면 `--full` 이 이긴다
        #[arg(long)]
        sample: Option<usize>,
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
        Command::Symbols { path, json, graph } => symbols(&path, json, graph),
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
        Command::Doctor { repo, at, cache_dir, index, intent, full, sample, json } => {
            let scope = if full {
                pal_core::DoctorScope::Full
            } else {
                pal_core::DoctorScope::Sample {
                    max: sample.unwrap_or(pal_core::PROVISIONAL_SAMPLE_MAX),
                }
            };
            doctor::run(&repo, at.as_deref(), cache_dir, index, intent, scope, json)
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

fn symbols(path: &Path, json: bool, graph: bool) -> Result<()> {
    let source = std::fs::read(path).with_context(|| format!("읽지 못했다: {}", path.display()))?;

    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    let Some(language) = Language::from_extension(ext) else {
        // **"언어를 모른다"와 "추출기가 없다"는 다르다.** 여기는 전자다.
        anyhow::bail!(
            "확장자 `.{ext}` 를 언어로 알지 못한다 — 아는 것은 Kotlin · Java · JavaScript · TypeScript 넷이다"
        );
    };

    if graph {
        return file_graph(&source, language);
    }

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

/// 파일 그래프 전부를 JSON 으로.
///
/// **이것이 `[f02.1.pass]` ② 가 밖에서 잴 수 있는 유일한 창이다** — 같은 blob 을 다른
/// 저장소·다른 경로에 두고 이 산출이 바이트 단위로 같은지 본다. 그러려면 경로가 산출에
/// **실리면 안 되고**, 그래서 여기서 `path` 를 찍지 않는다.
fn file_graph(source: &[u8], language: Language) -> Result<()> {
    match pal_extract::extractor_for(language) {
        Capable::Present(extractor) => {
            let graph = extractor.extract(source).context("추출 실패")?;
            println!("{}", serde_json::to_string_pretty(&graph)?);
        }
        Capable::NotBuilt { capability } => {
            // **빈 그래프를 내지 않는다.** 선언이 없는 파일과 같은 출력이 된다.
            let not_built: Capable<pal_core::FileGraph> = Capable::NotBuilt { capability };
            println!("{}", serde_json::to_string_pretty(&not_built)?);
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
