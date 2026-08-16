//! **등록된 훅 명령이 발화하고 차단 결정이 전달된다** — `[f24]` ⑧.
//!
//! # 여기서 재는 것과 못 재는 것을 먼저 가른다
//!
//! 게이트 ⑧ 의 두 문장은 *"등록된 훅 명령이 발화한다"* 와 *"차단 결정이 하네스에
//! 전달된다"* 이다. 이 시험이 서는 자리는 **등록 문자열까지**다:
//!
//! | 재는 것 | 어떻게 |
//! |---|---|
//! | 등록된 문자열이 **실제로 도는가** | 실측된 규약 그대로 `/bin/sh -c "<등록 문자열 원문>"` 로 실행 |
//! | 그 실행이 **차단 바이트를 내는가** | 표준출력이 `{"decision":"block","reason":…}` · 종료 코드 0 |
//! | **두 번 설치해도 한 번만 도는가** | 중복 제거가 완전 일치 기준이므로 등록이 하나여야 한다 |
//! | 남의 등록을 **안 건드리는가** | 사용자 훅이 든 fixture 에서 왕복 후 값 비교 |
//!
//! **못 재는 것**: 하네스가 그 바이트를 받아 실제로 서브에이전트를 막는 마지막 한 칸.
//! 그것은 `claude` 세션을 실제로 돌려야 보이고 **이 회차는 안 했다.** 그래서 그 한 칸은
//! 통과로도 반증으로도 세지 않는다 — 이 파일이 재는 것은 **우리가 내는 바이트가 실측된
//! 규약과 같은가**까지다.
//!
//! # ★ 왜 공백이 든 경로에 바이너리를 복사해서 설치하는가
//!
//! 등록 문자열은 셸을 거친다. 우리 시험 바이너리는 `target/debug/deps/` 아래라 공백이
//! 없고, 그러면 **따옴표가 없어도 전부 통과한다.** 따옴표를 재려면 공백이 있어야 한다.

mod common;

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

use common::{PAL, git};

// ─────────────────────────────────────────────────────────────────────────────
// fixture
// ─────────────────────────────────────────────────────────────────────────────

fn 방(tag: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("pal-f24-훅-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("방");
    root
}

/// 대상 프로젝트 하나 — git 저장소이되 우리 것이 없다.
fn 프로젝트(tag: &str) -> PathBuf {
    let root = 방(tag).join("proj");
    std::fs::create_dir_all(&root).expect("proj");
    std::fs::write(root.join("README.md"), "hello\n").expect("README");
    git(&root, &["init", "-q", "."]);
    git(&root, &["add", "-A"]);
    git(&root, &["-c", "user.email=t@e", "-c", "user.name=t", "commit", "-qm", "첫"]);
    root
}

/// **공백이 든 디렉터리**에 `pal` 을 복사한다 — 따옴표를 재는 자리.
fn 공백이_든_곳의_pal(root: &Path, 이름: &str) -> PathBuf {
    let dir = root.join(format!("도구 {이름}"));
    std::fs::create_dir_all(&dir).expect("도구 방");
    let exe = dir.join("pal");
    std::fs::copy(PAL, &exe).expect("복사");
    실행_권한(&exe);
    exe
}

#[cfg(unix)]
fn 실행_권한(exe: &Path) {
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(exe, std::fs::Permissions::from_mode(0o755)).expect("chmod");
}

#[cfg(not(unix))]
fn 실행_권한(_exe: &Path) {}

fn 성공(exe: &Path, cwd: &Path, args: &[&str]) -> String {
    let out = Command::new(exe).args(args).current_dir(cwd).output().expect("pal 을 못 돌렸다");
    assert!(
        out.status.success(),
        "pal {args:?}\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8_lossy(&out.stdout).into_owned()
}

fn 설정(root: &Path) -> serde_json::Value {
    serde_json::from_slice(&std::fs::read(root.join(".claude/settings.json")).expect("읽기"))
        .expect("JSON")
}

/// 그 사건에 걸린 명령 문자열 전부 — **묶음 구조를 여기 한 번만 안다.**
fn 걸린_명령(설정: &serde_json::Value, event: &str) -> Vec<String> {
    설정["hooks"][event]
        .as_array()
        .map(|groups| {
            groups
                .iter()
                .filter_map(|g| g["hooks"].as_array())
                .flatten()
                .filter_map(|c| c["command"].as_str())
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default()
}

fn 우리_명령(root: &Path) -> String {
    let 전부 = 걸린_명령(&설정(root), "SubagentStop");
    let 우리것: Vec<_> = 전부.iter().filter(|c| c.contains("hook SubagentStop")).collect();
    assert_eq!(우리것.len(), 1, "우리 등록이 하나가 아니다: {전부:?}");
    우리것[0].clone()
}

// ─────────────────────────────────────────────────────────────────────────────
// ★ 실측된 규약 그대로 — **`/bin/sh -c "<등록 문자열 원문>"`**
// ─────────────────────────────────────────────────────────────────────────────

fn 하네스처럼(command: &str, payload: &str) -> Output {
    let mut child = Command::new("/bin/sh")
        .arg("-c")
        .arg(command)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("셸을 못 돌렸다");
    child.stdin.as_mut().expect("stdin").write_all(payload.as_bytes()).expect("쓰기");
    child.wait_with_output().expect("wait")
}

fn 페이로드(마지막_말: &str, 반복중: bool) -> String {
    serde_json::json!({
        "session_id": "s-1",
        "transcript_path": "/tmp/t.jsonl",
        "cwd": "/tmp",
        "hook_event_name": "SubagentStop",
        "agent_id": "a-1",
        "stop_hook_active": 반복중,
        "last_assistant_message": 마지막_말,
    })
    .to_string()
}

// ─────────────────────────────────────────────────────────────────────────────
// (a) 발화 · (b) 차단
// ─────────────────────────────────────────────────────────────────────────────

/// ★ **등록된 그 문자열이 실제로 돌고, 차단 바이트를 낸다.**
///
/// 「`settings.json` 에 적혀 있다」로는 부족하다 — 파일이 사라지거나 실행 권한을 잃어도
/// 하네스는 exit 126/127 을 **완전히 삼킨다.** 그래서 적힌 문자열을 **실행해서** 잰다.
#[test]
fn 등록된_명령이_돌고_차단을_낸다() {
    let root = 프로젝트("발화");
    let exe = 공백이_든_곳의_pal(root.parent().expect("부모"), "발화");
    성공(&exe, &root, &["install"]);

    let command = 우리_명령(&root);
    assert!(command.contains(&exe.display().to_string()), "절대 경로로 안 걸렸다: {command}");
    assert!(command.starts_with('\''), "공백이 든 경로가 따옴표로 안 묶였다: {command}");

    // (a) 발화 — 부르면 흔적이 남는다.
    let out = 하네스처럼(&command, &페이로드("다 했다", false));
    assert!(out.status.success(), "등록된 명령이 exit 0 을 안 냈다");
    assert!(
        String::from_utf8_lossy(&out.stderr).contains("pal hook"),
        "발화 흔적이 없다 — stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(out.stdout.is_empty(), "통과인데 표준출력이 있다");

    // (b) 차단 — 실측된 규약 그대로의 바이트가 나온다.
    let out = 하네스처럼(&command, &페이로드("", false));
    assert!(out.status.success(), "차단인데 exit 0 이 아니다");
    let v: serde_json::Value = serde_json::from_slice(&out.stdout).expect("표준출력이 JSON 이 아니다");
    assert_eq!(v["decision"], "block", "차단 결정이 안 나왔다: {v}");
    assert!(!v["reason"].as_str().expect("reason").trim().is_empty());

    // 그리고 반복 회차에서는 같은 페이로드가 통과다.
    let out = 하네스처럼(&command, &페이로드("", true));
    assert!(out.stdout.is_empty(), "반복 회차에서 또 차단했다");
}

/// ★ **두 번 설치해도 훅은 하나다.** 중복 제거가 완전 일치 기준이라, 공백 하나만 달라도
/// 같은 훅이 두 번 돈다.
#[test]
fn 두_번_설치해도_훅이_하나다() {
    let root = 프로젝트("멱등");
    let exe = 공백이_든_곳의_pal(root.parent().expect("부모"), "멱등");
    성공(&exe, &root, &["install"]);
    let 첫째 = std::fs::read(root.join(".claude/settings.json")).expect("읽기");
    성공(&exe, &root, &["install"]);
    let 둘째 = std::fs::read(root.join(".claude/settings.json")).expect("읽기");

    assert_eq!(첫째, 둘째, "두 번째 설치가 설정 바이트를 바꿨다");
    assert_eq!(걸린_명령(&설정(&root), "SubagentStop").len(), 1, "훅이 둘이다");
}

/// **남이 같은 사건에 걸어 둔 것을 하나도 안 건드린다** — 그리고 왕복하면 그것만 남는다.
#[test]
fn 남의_훅은_왕복해도_그대로다() {
    let root = 프로젝트("남의것");
    let exe = 공백이_든_곳의_pal(root.parent().expect("부모"), "남의것");
    std::fs::create_dir_all(root.join(".claude")).expect(".claude");
    let 원본 = serde_json::json!({
        "env": {"A": "1"},
        "hooks": {
            "SubagentStop": [{"hooks": [{"type": "command", "command": "내 것.sh"}]}],
            "SessionStart": [{"hooks": [{"type": "command", "command": "내 시작.sh"}]}]
        }
    });
    std::fs::write(
        root.join(".claude/settings.json"),
        serde_json::to_string_pretty(&원본).expect("직렬화"),
    )
    .expect("settings");

    성공(&exe, &root, &["install"]);
    let 걸린 = 걸린_명령(&설정(&root), "SubagentStop");
    assert_eq!(걸린.len(), 2, "남의 것 옆에 우리 것이 안 걸렸거나 남의 것이 사라졌다: {걸린:?}");
    assert!(걸린.contains(&"내 것.sh".to_owned()), "남의 등록이 사라졌다: {걸린:?}");

    성공(&exe, &root, &["uninstall"]);
    assert_eq!(설정(&root), 원본, "왕복이 사용자 설정을 바꿨다");
}

/// ★ **실행 파일이 옮겨 가면 `update` 가 따라간다.**
///
/// 안 따라가면 옛 경로가 그대로 남고, 그 경로가 사라진 뒤에는 **exit 127 이 완전히
/// 침묵한다** — 아무도 훅이 죽은 것을 모른다. 버전이 같아도 이 갱신은 일어나야 한다.
#[test]
fn 갱신이_옮겨간_실행_파일을_따라간다() {
    let root = 프로젝트("이사");
    let 부모 = root.parent().expect("부모").to_path_buf();
    let 옛 = 공백이_든_곳의_pal(&부모, "옛");
    성공(&옛, &root, &["install"]);
    let 옛_명령 = 우리_명령(&root);

    let 새 = 공백이_든_곳의_pal(&부모, "새");
    let report = 성공(&새, &root, &["update"]);
    assert!(report.contains("훅"), "훅을 갱신했다고 말하지 않았다:\n{report}");

    let 지금 = 우리_명령(&root);
    assert_ne!(지금, 옛_명령, "옛 등록을 그대로 뒀다");
    assert!(지금.contains(&새.display().to_string()), "새 경로로 안 갈렸다: {지금}");
    assert_eq!(걸린_명령(&설정(&root), "SubagentStop").len(), 1, "죽은 등록이 남았다");

    // 그리고 매니페스트가 지금 걸린 것을 적고 있다 — 안 적으면 제거가 못 되돌린다.
    let m: serde_json::Value =
        serde_json::from_slice(&std::fs::read(root.join(".claude/pal/manifest.json")).expect("읽기"))
            .expect("JSON");
    assert_eq!(m["settings"]["hooks"][0]["command"], serde_json::json!(지금));

    성공(&새, &root, &["uninstall"]);
    assert!(!root.join(".claude/settings.json").exists(), "우리가 만든 설정이 남았다");
}

// ─────────────────────────────────────────────────────────────────────────────
// `doctor` — **「적혀 있다」로는 부족하다**
// ─────────────────────────────────────────────────────────────────────────────

/// 설치 검사를 JSON 으로 뜬다.
fn 검사들(cwd: &Path) -> serde_json::Value {
    let out = Command::new(PAL)
        .args(["doctor", "--install", "--json"])
        .current_dir(cwd)
        .output()
        .expect("pal doctor");
    serde_json::from_slice(&out.stdout).expect("JSON")
}

fn 훅_검사(cwd: &Path) -> serde_json::Value {
    let c = 검사들(cwd);
    let 배열 = c.as_array().expect("배열").clone();
    let 마지막 = 배열.last().expect("검사가 하나도 없다").clone();
    assert_eq!(마지막["number"], 6, "훅 검사가 여섯째가 아니다: {c}");
    마지막
}

/// ★ **등록된 명령을 실제로 실행해서 응답을 확인한다.**
///
/// 파일이 사라지거나 실행 권한을 잃으면 `/bin/sh` 가 exit 127·126 을 내는데, 하네스는
/// 그것을 **완전히 삼킨다** — 세션은 계속되고 `claude` 의 종료 코드는 0 이며 트랜스크립트
/// 에도 대화형 화면에도 한 글자도 안 나온다. 그래서 **여기가 유일한 문이다.**
#[test]
fn 진단이_등록된_훅을_실제로_돌려본다() {
    // ── 정상 — 초록 ─────────────────────────────────────────────────────────
    let root = 프로젝트("진단-정상");
    let exe = 공백이_든_곳의_pal(root.parent().expect("부모"), "진단");
    성공(&exe, &root, &["install"]);
    let c = 훅_검사(&root);
    assert_eq!(c["outcome"], "ok", "정상인데 초록이 아니다: {c}");

    // ── 등록이 사라졌다 — 빨강 ──────────────────────────────────────────────
    let 원래_설정 = std::fs::read(root.join(".claude/settings.json")).expect("읽기");
    let mut v: serde_json::Value = serde_json::from_slice(&원래_설정).expect("JSON");
    v.as_object_mut().expect("객체").remove("hooks");
    std::fs::write(
        root.join(".claude/settings.json"),
        serde_json::to_string_pretty(&v).expect("직렬화"),
    )
    .expect("쓰기");
    let c = 훅_검사(&root);
    assert_eq!(c["outcome"], "failed", "등록이 사라졌는데 안 걸렸다: {c}");
    std::fs::write(root.join(".claude/settings.json"), &원래_설정).expect("되돌리기");

    // ── 실행 권한을 잃었다 — 빨강 (exit 126) ───────────────────────────────
    권한을_뺀다(&exe);
    let c = 훅_검사(&root);
    assert_eq!(c["outcome"], "failed", "실행 권한이 없는데 안 걸렸다: {c}");
    실행_권한(&exe);

    // ── 파일이 사라졌다 — 빨강 (exit 127) ──────────────────────────────────
    std::fs::remove_file(&exe).expect("지우기");
    let c = 훅_검사(&root);
    assert_eq!(c["outcome"], "failed", "실행 파일이 없는데 안 걸렸다: {c}");
    assert!(
        c["detail"].as_str().expect("detail").contains("127"),
        "왜인지 안 적었다: {c}"
    );
}

/// 설치가 없으면 **`Residual`** 이다 — 검사하지 못한 것은 「이상 없음」이 아니다.
#[test]
fn 설치가_없으면_훅_검사가_잔여다() {
    let root = 프로젝트("진단-잔여");
    assert_eq!(훅_검사(&root)["outcome"], "residual");
}

#[cfg(unix)]
fn 권한을_뺀다(exe: &Path) {
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(exe, std::fs::Permissions::from_mode(0o644)).expect("chmod");
}

#[cfg(not(unix))]
fn 권한을_뺀다(_exe: &Path) {}
