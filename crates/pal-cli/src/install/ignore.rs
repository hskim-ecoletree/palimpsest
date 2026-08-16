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
//! # `!` 를 먼저 git 에게 묻는다 — 텍스트는 **git 이 침묵할 때만**
//!
//! `!` 는 `.git/info/exclude` · 전역 `core.excludesFile` · 중첩 `.gitignore` 에도 살 수
//! 있고 **자리를 모르는** 텍스트 스캔은 그것들을 못 본다. 그래서 먼저 git 에게
//! `-v --no-index` 로 마지막 매치 패턴을 받아 `!` 로 시작하면 **거부한다.**
//!
//! ⚠ **부정 패턴은 슬래시 없는 질의에만 잡히므로 두 형태를 다 묻는다.**
//!
//! # ★ 그런데 git 이 침묵하는 자리가 있다 — 실측 (git 2.50.1)
//!
//! `check-ignore` 는 **디렉터리가 디스크에 있을 때만** 디렉터리 형태의 `!` 패턴을 낸다.
//! 없으면 **어떤 질의 형태로도 패턴을 하나도 안 낸다**:
//!
//! ```text
//! .gitignore = "!.palimpsest/cache/"        디렉터리 없음
//!   check-ignore -v    -- '.palimpsest/cache/'   rc=1  출력 없음
//!   check-ignore -v    -- '.palimpsest/cache'    rc=1  출력 없음
//!   check-ignore -v -n -- '.palimpsest/cache/'   rc=1  "::\t.palimpsest/cache/"  ← 빈 패턴
//!                                             디렉터리 있음
//!   check-ignore -v    -- '.palimpsest/cache'    rc=0  "!.palimpsest/cache/"
//! ```
//!
//! ⚠ **`-n` 은 이 자리를 못 메운다.** `-n` 이 더하는 것은 *"안 맞았다"* 를 빈 패턴
//! (`::`)으로 찍어 주는 줄뿐이고, 패턴을 새로 알려 주지 않는다. 없는 것은 **디스크
//! 정보**이지 플래그가 아니다.
//!
//! 그래서 git 이 **「안 덮인다」고 답하면서 패턴을 하나도 안 낸** 자리에서만, git 이
//! 쓰는 소스 넷을 **git 에게 열거시켜** 직접 읽는다([`소스에서_되살림`]). 텍스트를 읽되
//! **자리를 안 빠뜨린다** — 위 머리말이 금한 것은 텍스트 자체가 아니라 자리를 빠뜨리는
//! 것이었다.
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

use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result};

use super::layout::IGNORE_FILE;

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
            // git 이 침묵했다 — 그때만 소스를 읽는다.
            None => match 소스에서_되살림(dir, path)? {
                Some(pattern) => Ok(Verdict::Revived { pattern }),
                None => Ok(Verdict::Uncovered),
            },
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

/// git 이 침묵한 자리 — **소스를 직접 읽어 되살림을 찾는다.**
///
/// # 언제만 부른다
///
/// git 이 이미 **「안 덮인다」**고 답했고 매치된 패턴을 **하나도 안 낸** 자리에서만
/// 부른다. 그러니 여기서 나오는 `!` 는 우선순위 다툼에 진 것이 아니다 — 아무도 안
/// 맞은 자리에 홀로 서 있는 것이다.
///
/// # 자리를 git 에게 열거시킨다
///
/// 뿌리와 조상들의 `.gitignore` · `.git/info/exclude`(`rev-parse --git-path`) ·
/// 전역 `core.excludesFile`(`config --get`). 넷 전부를 **git 이 말해 준 자리로** 연다.
///
/// ⚠ **못 보는 것**: 전역 설정값이 `~` 로 시작하면 **건너뛴다.** 그것을 펴려면 홈을
/// 읽어야 하고, `[f24]` ⑦ 이 그 자리를 닫아 두었다.
fn 소스에서_되살림(dir: &Path, path: &str) -> Result<Option<String>> {
    let 질의 = path.trim_end_matches('/');
    for (source, prefix) in 소스들(dir, 질의)? {
        let Ok(text) = std::fs::read_to_string(&source) else { continue };
        for line in text.lines() {
            let line = line.trim();
            let Some(pattern) = line.strip_prefix('!') else { continue };
            if 가리키나(pattern, &prefix, 질의) {
                return Ok(Some(line.to_owned()));
            }
        }
    }
    Ok(None)
}

/// 읽을 소스와 그 소스가 서 있는 **접두사**(중첩 `.gitignore` 는 자기 디렉터리 기준이다).
fn 소스들(dir: &Path, 질의: &str) -> Result<Vec<(PathBuf, String)>> {
    let mut out = vec![(dir.join(IGNORE_FILE), String::new())];
    // 조상들의 중첩 `.gitignore` — `a/b/c` 면 `a/` 와 `a/b/` 를 본다.
    let mut prefix = String::new();
    let mut parts: Vec<&str> = 질의.split('/').collect();
    parts.pop();
    for part in parts {
        prefix.push_str(part);
        prefix.push('/');
        out.push((dir.join(&prefix).join(IGNORE_FILE), prefix.clone()));
    }
    let exclude = 한_줄(dir, &["rev-parse", "--git-path", "info/exclude"])?;
    if let Some(p) = exclude {
        out.push((dir.join(p), String::new()));
    }
    // 전역 — **`~` 는 안 편다.** 홈을 읽는 것이 `[f24]` ⑦ 이 닫은 자리다.
    if let Some(p) = 한_줄(dir, &["config", "--get", "core.excludesFile"])? {
        if !p.starts_with('~') {
            out.push((dir.join(p), String::new()));
        }
    }
    Ok(out)
}

/// 이 부정 패턴이 그 경로를 가리키나.
///
/// **넓게 잡는다.** 여기서 참을 내면 우리는 규칙을 **안 더하고 그 사실을 말한다** —
/// 틀려도 사용자 파일이 상하지 않는 방향이다. 거짓을 내면 사용자 결정이 뒤집힌다.
fn 가리키나(pattern: &str, prefix: &str, 질의: &str) -> bool {
    let p = pattern.trim().trim_end_matches('/');
    let p = p.strip_prefix('/').unwrap_or(p);
    if p.is_empty() {
        return false;
    }
    if format!("{prefix}{p}") == 질의 {
        return true;
    }
    // 슬래시가 없는 패턴은 **아무 깊이의 같은 이름**을 가리킨다(git 의 규칙).
    !p.contains('/') && 질의.rsplit('/').next() == Some(p)
}

/// 첫 줄만 — 없으면 `None`.
fn 한_줄(dir: &Path, args: &[&str]) -> Result<Option<String>> {
    let out = git(dir, args)?;
    let text = String::from_utf8_lossy(&out.stdout);
    Ok(text.lines().next().map(str::trim).filter(|s| !s.is_empty()).map(ToOwned::to_owned))
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
    use super::{last_pattern, with_slash, 가리키나};

    #[test]
    fn 부정_패턴이_그_경로를_가리키는지_본다() {
        assert!(가리키나(".palimpsest/cache/", "", ".palimpsest/cache"));
        assert!(가리키나("/.palimpsest/cache", "", ".palimpsest/cache"));
        // 중첩 `.gitignore` — 자기 디렉터리가 접두사다.
        assert!(가리키나("cache/", ".palimpsest/", ".palimpsest/cache"));
        // 슬래시 없는 패턴은 아무 깊이의 같은 이름을 가리킨다.
        assert!(가리키나("cache", "", ".palimpsest/cache"));
        // 남의 이름은 안 가리킨다.
        assert!(!가리키나("node_modules", "", ".palimpsest/cache"));
        assert!(!가리키나(".palimpsest/index.redb", "", ".palimpsest/cache"));
        assert!(!가리키나("", "", ".palimpsest/cache"));
    }

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
