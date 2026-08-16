//! ★ **대상 프로젝트의 파일은 전부 남이 쓴 것이다** — `[f24]` ⑥⑦ 의 뒷면.
//!
//! # 왜 이 파일이 따로 서는가
//!
//! `.claude/pal/manifest.json` 과 `.claude/settings.json` 은 **대상 프로젝트 안의 평범한
//! 파일**이고 `.gitignore` 에 없어서 **커밋되고 clone 과 함께 이동한다.** 그래서 그 둘의
//! 내용은 **입력이지 사실이 아니다.** 서명도 소유 확인도 없다.
//!
//! | 재는 것 | 왜 |
//! |---|---|
//! | 저장소에서 읽은 문자열을 **실행하지 않는다** | 임의 코드 실행. `pal doctor` 한 번이 남의 문자열을 셸에 넘겼다 |

mod common;

use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

use common::{PAL, git};

// ─────────────────────────────────────────────────────────────────────────────
// 방 — 대상 프로젝트 하나
// ─────────────────────────────────────────────────────────────────────────────

struct 방 {
    base: PathBuf,
    안: PathBuf,
}

fn 방(tag: &str) -> 방 {
    let base = std::env::temp_dir().join(format!("pal-f24-적대-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    let 안 = base.join("안");
    std::fs::create_dir_all(&안).expect("안");
    std::fs::write(안.join("README.md"), "hello\n").expect("README");
    git(&안, &["init", "-q", "."]);
    방 { base, 안 }
}

impl Drop for 방 {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.base);
    }
}

fn 돌린다(cwd: &Path, args: &[&str]) -> Output {
    let path = std::env::var("PATH").unwrap_or_default();
    let pal_dir = Path::new(PAL).parent().expect("pal 의 부모");
    Command::new(PAL)
        .args(args)
        .current_dir(cwd)
        .env("PATH", format!("{}:{path}", pal_dir.display()))
        .output()
        .expect("pal 을 못 돌렸다")
}

fn 성공(cwd: &Path, args: &[&str]) -> String {
    let out = 돌린다(cwd, args);
    assert!(
        out.status.success(),
        "pal {args:?}\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8_lossy(&out.stdout).into_owned()
}

/// ★ **매달릴 수 있는 자리는 시간 상한을 걸고 돌린다.** 이 기계에 `timeout` 이 없다.
fn 시간_안에(cwd: &Path, args: &[&str], 상한_ms: u64) -> Output {
    let path = std::env::var("PATH").unwrap_or_default();
    let pal_dir = Path::new(PAL).parent().expect("pal 의 부모");
    let mut child = Command::new(PAL)
        .args(args)
        .current_dir(cwd)
        .env("PATH", format!("{}:{path}", pal_dir.display()))
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("pal 을 못 돌렸다");
    let mut 기다린 = 0;
    loop {
        match child.try_wait().expect("try_wait") {
            Some(_) => return child.wait_with_output().expect("wait"),
            None if 기다린 >= 상한_ms => {
                let _ = child.kill();
                let _ = child.wait();
                panic!("pal {args:?} 가 {상한_ms}ms 안에 안 끝났다 — 매달렸다");
            }
            None => {
                std::thread::sleep(std::time::Duration::from_millis(25));
                기다린 += 25;
            }
        }
    }
}

fn 값(path: &Path) -> serde_json::Value {
    serde_json::from_slice(&std::fs::read(path).expect("읽기")).expect("JSON")
}

fn 쓴다(path: &Path, v: &serde_json::Value) {
    std::fs::write(path, serde_json::to_string_pretty(v).expect("직렬화")).expect("쓰기");
}

fn 매니페스트_자리(root: &Path) -> PathBuf {
    root.join(".claude/pal/manifest.json")
}

fn 설정_자리(root: &Path) -> PathBuf {
    root.join(".claude/settings.json")
}

// ─────────────────────────────────────────────────────────────────────────────
// 1. **저장소에서 읽은 문자열을 실행하지 않는다**
// ─────────────────────────────────────────────────────────────────────────────

/// `/tmp` 아래의 표식 하나 — **부작용이 일어났는지**를 이것 하나로 가른다.
fn 표식(tag: &str) -> PathBuf {
    let p = PathBuf::from(format!("/tmp/pal-f24-PWNED-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_file(&p);
    p
}

/// 매니페스트와 `settings.json` 의 훅 명령을 **관측 가능한 부작용을 내는 문자열**로
/// 바꾼다. 둘을 같이 바꿔야 「등록돼 있다」 검사를 지나 탐침까지 간다.
fn 훅을_심는다(root: &Path, 명령: &str) {
    let mp = 매니페스트_자리(root);
    let mut m = 값(&mp);
    let hooks = m["settings"]["hooks"].as_array_mut().expect("훅 목록");
    assert!(!hooks.is_empty(), "이 시험이 재려는 상태가 아니다 — 등록된 훅이 없다");
    for h in hooks.iter_mut() {
        h["command"] = serde_json::json!(명령);
    }
    쓴다(&mp, &m);

    let sp = 설정_자리(root);
    let mut s = 값(&sp);
    for (_, groups) in s["hooks"].as_object_mut().expect("훅 구역").iter_mut() {
        for g in groups.as_array_mut().expect("묶음") {
            for c in g["hooks"].as_array_mut().expect("명령들") {
                c["command"] = serde_json::json!(명령);
            }
        }
    }
    쓴다(&sp, &s);
}

/// ★ **`pal doctor` 가 저장소에 커밋된 문자열을 셸로 실행하지 않는다.**
///
/// 관측(고치기 전): `touch …/PWNED` 를 심었더니 **사용자 uid 로 실행됐다.**
/// `pal doctor --install` 과 인자 없는 평범한 `pal doctor` 둘 다.
#[test]
fn 진단이_매니페스트의_문자열을_실행하지_않는다() {
    let 방 = 방("실행");
    성공(&방.안, &["install"]);

    for (tag, args) in [("설치검사", &["doctor", "--install"][..]), ("전체", &["doctor"][..])] {
        let 흔적 = 표식(tag);
        훅을_심는다(&방.안, &format!("touch '{}'", 흔적.display()));

        let out = 시간_안에(&방.안, args, 60_000);
        assert!(
            !흔적.exists(),
            "{tag}: **저장소의 문자열이 실행됐다** — {} 가 생겼다\nstdout: {}",
            흔적.display(),
            String::from_utf8_lossy(&out.stdout)
        );
        let _ = std::fs::remove_file(&흔적);
    }
}

/// ★ **설치 루트 탐색이 대상 경계를 넘어 올라가지 않는다.**
///
/// 조상 디렉터리에 매니페스트를 심어 두면 아무 관계 없는 하위 디렉터리에서 돌려도
/// 그것을 찾아 **실행했다.** `--repo` 가 경계가 아니었다.
#[test]
fn 진단이_조상의_매니페스트를_찾아가지_않는다() {
    let 방 = 방("조상");
    성공(&방.안, &["install"]);

    let 흔적 = 표식("조상");
    훅을_심는다(&방.안, &format!("touch '{}'", 흔적.display()));

    // 대상은 **자기 경계를 가진 남의 자리**다 — 조상의 설치와 아무 관계가 없다.
    let 무관 = 방.안.join("무관한-하위");
    std::fs::create_dir_all(&무관).expect("무관");
    git(&무관, &["init", "-q", "."]);

    let out = 시간_안에(&무관, &["doctor", "--install", "--repo", "."], 60_000);
    assert!(
        !흔적.exists(),
        "**조상의 문자열이 실행됐다** — {} 가 생겼다",
        흔적.display()
    );
    let 화면 = String::from_utf8_lossy(&out.stdout);
    assert!(
        !화면.contains(&방.안.display().to_string()),
        "경계 밖의 설치를 찾아갔다:\n{화면}"
    );
    let _ = std::fs::remove_file(&흔적);
}

