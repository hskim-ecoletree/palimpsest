//! 빌드 시각에 **커밋을 바이너리에 박는다** — `[f24]` ⑨.
//!
//! # 이 파일이 안 하는 것
//!
//! **판단을 안 한다.** 버전 문자열을 어떻게 조립하는지는 [`pal-cli/src/version.rs`] 가
//! 지고, 여기는 *"커밋이 무엇인가"* 하나만 답한다. 그래야 조립 규칙이 단위 시험으로
//! 재어진다 — 빌드 스크립트 안의 규칙은 아무도 못 잰다.
//!
//! # 깨지면 안 되는 자리
//!
//! **git 이 없거나 저장소가 아닌 곳에서 빌드해도 빌드가 서야 한다.** 릴리스 tarball 은
//! `.git/` 없이 풀린다. 그래서 이 스크립트는 **어떤 경우에도 실패하지 않고**, 커밋을
//! 못 알아내면 환경 변수를 **안 내보낸다**. 그때 `option_env!("PAL_COMMIT")` 이 `None`
//! 이 되고 버전은 패키지 버전 하나로 선다.
//!
//! ⚠ **홈을 안 읽는다** — `[f24]` ⑦ 이 재는 자리이고 빌드 스크립트도 그 안이다.

#![forbid(unsafe_code)]

use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    // 소스가 안 바뀌어도 **커밋이 바뀌면 다시 돌아야 한다.** 안 그러면 두 커밋이
    // 같은 값을 내고, 그것이 ⑨ 의 반증 형태 그대로다.
    let manifest = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default());
    if let Some(git) = git_dir(&manifest) {
        for hint in ["HEAD", "packed-refs"] {
            println!("cargo:rerun-if-changed={}", git.join(hint).display());
        }
        if let Some(reference) = head_reference(&git) {
            println!("cargo:rerun-if-changed={}", git.join(reference).display());
        }
    }
    println!("cargo:rerun-if-env-changed=PAL_COMMIT");

    if let Some(commit) = commit(&manifest) {
        println!("cargo:rustc-env=PAL_COMMIT={commit}");
    }
}

/// `.git` 을 위로 올라가며 찾는다. **못 찾아도 실패가 아니다.**
///
/// ★★ **worktree 에서는 `.git` 이 디렉터리가 아니라 파일이다.** (2026-08-24)
/// 그 파일 한 줄이 `gitdir: <경로>` 로 진짜 자리를 가리킨다. 앞 판은 `is_dir()` 만
/// 봐서 worktree 체크아웃에서 **`None` 을 돌려줬고**, 그러면 위의
/// `cargo:rerun-if-changed` 가 **한 줄도 등록되지 않아 커밋이 바뀌어도 build.rs 가
/// 안 돈다.** 실측 2026-08-24: 두 커밋 연속으로 `pal --version` 이 **앞 커밋의 SHA** 를
/// 냈다 — 이 파일 머리가 *"두 커밋이 같은 값을 내고, 그것이 ⑨ 의 반증 형태 그대로다"*
/// 라고 적은 바로 그 상태다. 시험 `버전에_커밋이_실려_있다` 가 그것을 잡는다.
fn git_dir(from: &Path) -> Option<PathBuf> {
    let mut here = Some(from);
    while let Some(dir) = here {
        let candidate = dir.join(".git");
        if candidate.is_dir() {
            return Some(candidate);
        }
        // worktree — `.git` 파일의 `gitdir:` 줄을 따라간다.
        if candidate.is_file() {
            if let Some(실제) = gitdir_파일이_가리키는_곳(&candidate) {
                return Some(실제);
            }
        }
        here = dir.parent();
    }
    None
}

/// worktree 의 `.git` **파일**이 가리키는 진짜 git 디렉터리.
///
/// 형식은 `gitdir: <경로>` 한 줄이고 경로는 절대일 수도 상대일 수도 있다.
fn gitdir_파일이_가리키는_곳(git_파일: &Path) -> Option<PathBuf> {
    let text = std::fs::read_to_string(git_파일).ok()?;
    let 경로 = text.trim().strip_prefix("gitdir:")?.trim();
    let p = PathBuf::from(경로);
    let p = if p.is_absolute() {
        p
    } else {
        git_파일.parent()?.join(p)
    };
    p.is_dir().then_some(p)
}

/// `.git/HEAD` 가 가리키는 ref 의 상대 경로. 분리 HEAD 면 `None`.
fn head_reference(git: &Path) -> Option<String> {
    let text = std::fs::read_to_string(git.join("HEAD")).ok()?;
    let reference = text.trim().strip_prefix("ref: ")?;
    Some(reference.to_owned())
}

/// 지금 커밋. **git 이 없거나 저장소가 아니면 `None`.**
fn commit(cwd: &Path) -> Option<String> {
    let out = Command::new("git")
        .args(["rev-parse", "--short=12", "HEAD"])
        .current_dir(cwd)
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let sha = String::from_utf8(out.stdout).ok()?.trim().to_owned();
    if sha.is_empty() { None } else { Some(sha) }
}
