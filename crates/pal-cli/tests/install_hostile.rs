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
//! | **하드링크**로 대상 밖이 안 샌다 | 심링크는 `canonicalize` 가 막지만 하드링크는 「밖」이라는 신원이 없다 |
//! | 매니페스트가 대상 **안**의 아무 파일이나 못 지운다 | 악성 PR 하나 + `pal uninstall` 한 번 |

mod common;

use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

use common::{PAL, git};

// ─────────────────────────────────────────────────────────────────────────────
// 방 — `밖/` 과 `안/` 이 형제로 산다
// ─────────────────────────────────────────────────────────────────────────────

struct 방 {
    base: PathBuf,
    밖: PathBuf,
    안: PathBuf,
}

fn 방(tag: &str) -> 방 {
    let base = std::env::temp_dir().join(format!("pal-f24-적대-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    let 밖 = base.join("밖");
    let 안 = base.join("안");
    std::fs::create_dir_all(&밖).expect("밖");
    std::fs::create_dir_all(&안).expect("안");
    std::fs::write(밖.join("희생양.txt"), "건드리면 안 된다\n").expect("희생양");
    std::fs::write(안.join("README.md"), "hello\n").expect("README");
    git(&안, &["init", "-q", "."]);
    방 { base, 밖, 안 }
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

// ─────────────────────────────────────────────────────────────────────────────
// 2. **하드링크로 대상 밖이 새지 않는다**
//
// ★ **유닉스 전용 가정이 여기 있다.** 링크를 거는 것도(`std::fs::hard_link` 는 이식
// 가능하지만 이 시험이 재는 성질은 `nlink` 위에 선다) 세는 것도 유닉스 형태다. 그래서
// **짝 없는 `#[cfg(unix)]` 을 안 단다** — 짝이 없으면 다른 플랫폼에서 이 방어가
// **조용히 사라지고**, 사라졌다는 사실조차 안 보인다. 아래 `#[cfg(not(unix))]` 짝이
// 그 자리에서 **시끄럽게** 실패한다.
// ─────────────────────────────────────────────────────────────────────────────

/// ★ **하드링크는 「밖」이라는 신원이 없다.** 심링크는 `canonicalize` 가 푸는데
/// 하드링크는 원리상 못 본다 — 그래서 **제자리 쓰기 자체를 안 한다.**
///
/// 관측(고치기 전): 밖의 `희생양.txt` 가 0바이트가 됐고 **rc=0**.
#[test]
#[cfg(unix)]
fn 설치가_하드링크를_통해_밖을_안_고친다() {
    let 방 = 방("하드-설치");
    let 희생양 = 방.밖.join("희생양.txt");
    let 원본 = std::fs::read(&희생양).expect("읽기");
    // 대상 **안**의 `CLAUDE.md` 가 밖의 파일과 **같은 inode** 다.
    std::fs::hard_link(&희생양, 방.안.join("CLAUDE.md")).expect("hard_link");

    let out = 돌린다(&방.안, &["install"]);
    assert_eq!(std::fs::read(&희생양).expect("읽기"), 원본, "밖의 파일이 바뀌었다");
    assert!(
        !out.status.success(),
        "하드링크를 보고도 성공을 냈다\nstdout: {}",
        String::from_utf8_lossy(&out.stdout)
    );
    assert!(
        String::from_utf8_lossy(&out.stderr).contains("하드링크"),
        "까닭을 안 적었다 — {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

/// **제거 쪽도 같은 문**을 지난다.
#[test]
#[cfg(unix)]
fn 제거가_하드링크를_통해_밖을_안_고친다() {
    let 방 = 방("하드-제거");
    std::fs::write(방.안.join("CLAUDE.md"), "내 지시\n").expect("CLAUDE.md");
    성공(&방.안, &["install"]);

    // 설치가 끝난 뒤 사용자가(혹은 남이) 밖에서 링크를 걸었다.
    let 링크 = 방.밖.join("링크.txt");
    std::fs::hard_link(방.안.join("CLAUDE.md"), &링크).expect("hard_link");
    let 밖_전 = std::fs::read(&링크).expect("읽기");

    let out = 돌린다(&방.안, &["uninstall"]);
    assert_eq!(std::fs::read(&링크).expect("읽기"), 밖_전, "밖의 파일이 바뀌었다");
    assert!(!out.status.success(), "하드링크를 보고도 성공을 냈다");
}

/// ★ **유닉스 밖에서 이 방어는 아직 안 재진다 — 그 사실이 시끄러워야 한다.**
///
/// 소유자 결정(2026-08-16): *"windows 를 대응한다는 가정하에 앞으로 모든 설계와
/// 개발이 되어야 해."* 짝 없는 `#[cfg(unix)]` 시험은 다른 플랫폼에서 **조용히
/// 사라지고**, 그러면 경계 방어가 사라진 줄도 모른다. 그래서 짝을 단다.
#[test]
#[cfg(not(unix))]
fn 하드링크_방어가_이_플랫폼에서는_안_재진다() {
    panic!(
        "하드링크로 대상 밖이 새는지를 이 플랫폼에서 아직 안 잰다 — \
         `install/guard.rs` 의 링크 수 검사가 `#[cfg(unix)]` 안에 있다. \
         이 플랫폼의 등가 개념(NTFS 하드링크 · `GetFileInformationByHandle` 의 \
         `nNumberOfLinks`)으로 재는 자리를 세우기 전까지 이 방어는 **없다**"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// 3. **매니페스트가 대상 안의 아무 파일이나 못 지운다**
// ─────────────────────────────────────────────────────────────────────────────

/// ★ **우리가 되돌릴 수 있는 것은 우리가 놓을 수 있는 자리뿐이다.**
///
/// 관측(고치기 전): `.git/config` 와 `README.md` 를 각각 지웠다(**rc=0**).
#[test]
fn 매니페스트가_적은_안의_남의_파일을_안_지운다() {
    for 노린_것 in [".git/config", "README.md"] {
        let 방 = 방(&format!("안쪽-{}", 노린_것.replace(['/', '.'], "-")));
        성공(&방.안, &["install"]);

        let mp = 매니페스트_자리(&방.안);
        let mut m = 값(&mp);
        m["files"].as_array_mut().expect("files").push(serde_json::json!({
            "path": 노린_것,
            "sha256": "0".repeat(64),
        }));
        쓴다(&mp, &m);

        let out = 돌린다(&방.안, &["uninstall"]);
        assert!(
            방.안.join(노린_것).exists(),
            "{노린_것} 이 사라졌다 — 매니페스트가 대상 안의 남의 파일을 지웠다"
        );
        assert!(
            !out.status.success(),
            "{노린_것}: 우리가 놓을 수 없는 자리를 보고도 성공을 냈다\nstdout: {}",
            String::from_utf8_lossy(&out.stdout)
        );
    }
}

/// **블록에도 같은 문이 선다.**
#[test]
fn 매니페스트가_적은_안의_남의_블록을_안_건드린다() {
    let 방 = 방("안쪽-블록");
    성공(&방.안, &["install"]);
    let 원본 = std::fs::read(방.안.join("README.md")).expect("읽기");

    let mp = 매니페스트_자리(&방.안);
    let mut m = 값(&mp);
    m["blocks"].as_array_mut().expect("blocks").push(serde_json::json!({
        "path": "README.md",
        "inserted": "hello\n",
        "created": true,
    }));
    쓴다(&mp, &m);
    let out = 돌린다(&방.안, &["uninstall"]);
    assert!(방.안.join("README.md").exists(), "남의 파일이 사라졌다");
    assert_eq!(std::fs::read(방.안.join("README.md")).expect("읽기"), 원본, "남의 파일이 바뀌었다");
    assert!(!out.status.success(), "남의 블록을 보고도 성공을 냈다");
}

/// **디렉터리에도 같은 문이 선다.**
#[test]
fn 매니페스트가_적은_안의_남의_디렉터리를_안_지운다() {
    let 방 = 방("안쪽-디렉터리");
    성공(&방.안, &["install"]);
    let 남의_방 = 방.안.join("남의방");
    std::fs::create_dir_all(&남의_방).expect("남의방");
    let mp = 매니페스트_자리(&방.안);
    let mut m = 값(&mp);
    m["created_dirs"].as_array_mut().expect("created_dirs").push(serde_json::json!("남의방"));
    쓴다(&mp, &m);
    let out = 돌린다(&방.안, &["uninstall"]);
    assert!(남의_방.is_dir(), "남의 디렉터리가 사라졌다");
    assert!(!out.status.success(), "남의 디렉터리를 보고도 성공을 냈다");
}

