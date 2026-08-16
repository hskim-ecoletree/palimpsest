//! `.gitignore` 를 **git 에게 물어서** 다룬다 — 텍스트로 안 읽는다.
//!
//! # 판정 명령은 하나다. 다른 조합을 쓰면 틀린다
//!
//! ```text
//! git -C <d> check-ignore -q --no-index -- '<경로>/'
//!   rc=0 규칙이 덮는다 · rc=1 안 덮는다 · rc=128 worktree 아님
//! ```
//!
//! **세 요소가 전부 필수다.** 실측된 실패 조건:
//!
//! | 뺐을 때 | 무엇이 틀리나 |
//! |---|---|
//! | `-q` 대신 `-v` 의 종료 코드 | `-v` 의 rc=0 은 「무시됨」이 아니라 **「부정 패턴 포함 아무 패턴이나 매치됨」**이다. 사용자가 `!` 로 되살린 경로를 「무시됨」으로 오판한다 |
//! | `--no-index` | 그 경로 **아래에 추적 중인 파일이 하나라도 있으면** rc=1(「규칙 없음」)로 오판한다 |
//! | 후행 슬래시 | 디렉터리가 **아직 디스크에 없을 때**, 그리고 규칙이 `cache/**`·`cache/*` 형태일 때 오판한다. 규칙 14종 대조에서 슬래시 질의는 14/14 일치, 슬래시 없는 질의는 **2/14 오답** |
//! | rc=128 을 rc=1 과 뭉갬 | **저장소가 아닌 곳에 `.gitignore` 를 만든다** |
//!
//! # `!` 를 텍스트 grep 으로 찾지 않는다 — **실측에서 놓쳤다**
//!
//! `!` 는 `.git/info/exclude` · 전역 `core.excludesFile` · 중첩 `.gitignore` 에도 살 수
//! 있고 텍스트 스캔은 그것들을 못 본다. 그 결과 **사용자가 일부러 추적하기로 한 것을
//! 조용히 뒤집었다.** 그래서 git 에게 `-v --no-index` 로 마지막 매치 패턴을 받아
//! `!` 로 시작하면 **거부한다.**
//!
//! ⚠ **부정 패턴은 슬래시 없는 질의에만 잡히므로 두 형태를 다 묻는다.**
//!
//! # 추적 충돌은 **별개로** 묻는다
//!
//! ignore 규칙은 **이미 추적 중인 파일을 배제하지 못한다.**
//!
//! ```text
//! git -C <d> ls-files --error-unmatch -- '<경로>/'   rc=0 있음 · rc=1 없음
//! ```
//!
//! `--error-unmatch` 없이 종료 코드로 판정하면 **언제나 rc=0** 이다. 그리고
//! **bare 저장소에서는 128 이 아니라 1** 이 나온다.

use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result};

/// 한 경로에 대해 git 이 답한 것.
pub enum Verdict {
    /// 규칙이 이 경로를 덮는다 — **더할 것이 없다.**
    Covered,
    /// 안 덮는다 — 더할 후보다.
    Uncovered,
    /// 사용자가 `!` 로 되살렸다 — **더하면 그 결정을 조용히 뒤집는다. 거부한다.**
    Revived { pattern: String },
    /// worktree 가 아니다 — **`.gitignore` 를 만들지 않는다.**
    NotAWorktree,
}

/// 이 경로를 규칙이 덮는가. **판정 명령은 하나다.**
///
/// # Errors
/// `git` 을 못 돌리면.
pub fn verdict(dir: &Path, path: &str) -> Result<Verdict> {
    let slashed = with_slash(path);
    let code = run_code(dir, &["check-ignore", "-q", "--no-index", "--", &slashed])?;
    match code {
        Some(0) => Ok(Verdict::Covered),
        // **rc=128 을 rc=1 과 뭉개지 않는다.**
        Some(128) | None => Ok(Verdict::NotAWorktree),
        _ => match negation(dir, path)? {
            Some(pattern) => Ok(Verdict::Revived { pattern }),
            None => Ok(Verdict::Uncovered),
        },
    }
}

/// 마지막 매치 패턴이 `!` 로 시작하는가. **두 형태를 다 묻는다.**
fn negation(dir: &Path, path: &str) -> Result<Option<String>> {
    for query in [with_slash(path), path.trim_end_matches('/').to_owned()] {
        let out = git(dir, &["check-ignore", "-v", "--no-index", "--", &query])?;
        for line in String::from_utf8_lossy(&out.stdout).lines() {
            if let Some(pattern) = last_pattern(line) {
                if pattern.starts_with('!') {
                    return Ok(Some(pattern.to_owned()));
                }
            }
        }
    }
    Ok(None)
}

/// `<source>:<line>:<pattern>\t<pathname>` 에서 패턴만.
fn last_pattern(line: &str) -> Option<&str> {
    let left = line.split('\t').next()?;
    let mut parts = left.splitn(3, ':');
    parts.next()?;
    parts.next()?;
    parts.next().map(str::trim)
}

/// 이 경로 아래에 **추적 중인 파일이 있는가.** 있으면 규칙만으로는 안 빠진다.
///
/// # Errors
/// `git` 을 못 돌리면.
pub fn tracked(dir: &Path, path: &str) -> Result<bool> {
    let slashed = with_slash(path);
    // **`--error-unmatch` 가 없으면 언제나 rc=0 이다.**
    Ok(run_code(dir, &["ls-files", "--error-unmatch", "--", &slashed])? == Some(0))
}

/// **후행 슬래시는 필수다** — 없으면 규칙 14종 중 둘에서 오답이 났다.
fn with_slash(path: &str) -> String {
    if path.ends_with('/') { path.to_owned() } else { format!("{path}/") }
}

fn run_code(dir: &Path, args: &[&str]) -> Result<Option<i32>> {
    Ok(git(dir, args)?.status.code())
}

fn git(dir: &Path, args: &[&str]) -> Result<std::process::Output> {
    Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(args)
        .output()
        .with_context(|| format!("git {args:?} 을 돌리지 못했다"))
}

#[cfg(test)]
mod tests {
    use super::{last_pattern, with_slash};

    #[test]
    fn 후행_슬래시를_붙인다() {
        assert_eq!(with_slash("a/b"), "a/b/");
        assert_eq!(with_slash("a/b/"), "a/b/");
    }

    #[test]
    fn 부정_패턴을_알아본다() {
        let line = ".gitignore:7:!.palimpsest/cache/\t.palimpsest/cache/";
        assert_eq!(last_pattern(line), Some("!.palimpsest/cache/"));
    }

    /// 패턴 자체에 `:` 가 있어도 잘리지 않는다.
    #[test]
    fn 패턴_안의_콜론을_안_자른다() {
        let line = ".git/info/exclude:3:a:b/\ta:b/";
        assert_eq!(last_pattern(line), Some("a:b/"));
    }
}
