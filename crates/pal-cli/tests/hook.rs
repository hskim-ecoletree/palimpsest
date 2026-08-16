//! `pal hook <event>` — **하네스가 부르는 자리**. `[f24]` ⑧.
//!
//! # 여기서 재는 것은 규약이지 정책이 아니다
//!
//! 훅의 정책은 이 회차에서 **최소**이고 나중에 갈아끼워진다. 갈아끼워도 안 움직여야
//! 하는 것이 아래 셋이다:
//!
//! 1. **오작동해도 사람의 작업을 안 막는다** — 깨진 입력 · 모르는 사건 · 빈 입력에서
//!    **조용히 통과**(exit 0 · 표준출력 0 바이트).
//! 2. **`stop_hook_active` 가 참이면 절대 차단하지 않는다.** 실측: 텍스트만 뱉는
//!    서브에이전트는 9회에서 끊겼지만 **매 라운드 도구를 부르는 서브에이전트는 100회까지
//!    한 번도 안 멈췄다.** 하네스가 못 멈추므로 **훅이 스스로 멈춰야 한다.**
//! 3. 차단할 때 내는 바이트가 **실측된 규약 그대로**다 — `exit 0` + 표준출력
//!    `{"decision":"block","reason":…}`.
//!
//! # ★ 왜 `/bin/sh -c` 로도 한 번 돌리는가
//!
//! 실측: **훅 커맨드는 `/bin/sh -c "<등록 문자열 원문>"` 으로 실행된다.** 그래서
//! 바이너리를 직접 부르는 시험만 있으면 **등록 문자열이 셸을 통과하는지**는 아무도
//! 안 잰다 — 경로에 공백 하나면 그 자리가 무너진다.

use std::io::Write;
use std::path::Path;
use std::process::{Command, Output, Stdio};

const PAL: &str = env!("CARGO_BIN_EXE_pal");

/// 훅을 부른다 — **바이너리를 직접.**
fn 훅(event: &str, stdin: &str) -> Output {
    돌린다(Command::new(PAL).args(["hook", event]), stdin)
}

/// 훅을 부른다 — **실측된 규약 그대로 `/bin/sh -c` 를 거쳐서.**
fn 셸을_거쳐(command: &str, stdin: &str) -> Output {
    돌린다(Command::new("/bin/sh").arg("-c").arg(command), stdin)
}

fn 돌린다(cmd: &mut Command, stdin: &str) -> Output {
    let mut child = cmd
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("훅을 못 돌렸다");
    child.stdin.as_mut().expect("stdin").write_all(stdin.as_bytes()).expect("쓰기");
    child.wait_with_output().expect("wait")
}

fn 출력(out: &Output) -> String {
    String::from_utf8_lossy(&out.stdout).into_owned()
}

fn 진단(out: &Output) -> String {
    String::from_utf8_lossy(&out.stderr).into_owned()
}

/// 실측된 `SubagentStop` 페이로드의 모양 — 필드 이름을 여기 한 번만 적는다.
fn 페이로드(마지막_말: Option<&str>, 반복중: bool) -> String {
    let mut v = serde_json::json!({
        "session_id": "s-1",
        "transcript_path": "/tmp/t.jsonl",
        "cwd": "/tmp",
        "hook_event_name": "SubagentStop",
        "agent_id": "a-1",
        "agent_type": "general-purpose",
        "stop_hook_active": 반복중,
    });
    if let Some(말) = 마지막_말 {
        v["last_assistant_message"] = serde_json::json!(말);
    }
    v.to_string()
}

// ─────────────────────────────────────────────────────────────────────────────
// (a) 발화 — **부르면 흔적이 남는다**
// ─────────────────────────────────────────────────────────────────────────────

/// 훅은 **언제나** 표준오류에 한 줄을 남긴다. 실측: `exit 0` 이면 하네스가 표준오류를
/// 무시하므로 이 줄은 사용자의 작업을 방해하지 않고, `--debug` 에서만 보인다.
///
/// 그리고 그 줄이 **어느 빌드가 대답했는지**를 함께 적는다 — `pal doctor` 의 여섯째
/// 검사가 그것으로 「등록된 명령이 실제로 도는가」를 가른다.
#[test]
fn 부르면_표준오류에_흔적이_남는다() {
    let 버전 = String::from_utf8_lossy(
        &Command::new(PAL).arg("--version").output().expect("--version").stdout,
    )
    .trim()
    .to_owned();
    let 버전 = 버전.strip_prefix("pal ").expect("`pal <버전>` 형태").to_owned();

    let out = 훅("SubagentStop", &페이로드(Some("다 했다"), false));
    assert!(out.status.success(), "통과인데 exit 0 이 아니다");
    let 진단 = 진단(&out);
    assert!(진단.contains("pal hook"), "표식이 없다: {진단}");
    assert!(진단.contains(&버전), "어느 빌드가 대답했는지 안 적었다: {진단}");
    assert!(출력(&out).is_empty(), "통과인데 표준출력이 있다: {}", 출력(&out));
}

// ─────────────────────────────────────────────────────────────────────────────
// (b) 차단 — **실측된 규약 그대로**
// ─────────────────────────────────────────────────────────────────────────────

/// 실측: `exit 0` + 표준출력 `{"decision":"block","reason":…}` → `reason` 이 주입된다.
/// **`{"continue":false}` · `exit 1/3/42` · plain text 는 안 먹는다.**
#[test]
fn 차단은_exit_0_에_결정_json_이다() {
    let out = 훅("SubagentStop", &페이로드(Some("   "), false));
    assert!(out.status.success(), "차단인데 exit 0 이 아니다 — 실측상 exit 1 은 규약이 아니다");

    let v: serde_json::Value = serde_json::from_str(출력(&out).trim()).expect("표준출력이 JSON 이 아니다");
    assert_eq!(v["decision"], "block", "차단 결정이 안 나왔다: {v}");
    let reason = v["reason"].as_str().expect("reason 이 문자열이 아니다");
    assert!(!reason.trim().is_empty(), "까닭이 비었다 — 기본 문구로 떨어진다");
    assert_eq!(v.as_object().expect("객체").len(), 2, "규약 밖의 키가 섞였다: {v}");
}

/// ★ **`stop_hook_active` 가 참이면 절대 차단하지 않는다.**
///
/// 같은 페이로드가 1회차에는 차단을 내고 2회차에는 안 낸다 — 그 차이가 이 시험이
/// 재는 전부다.
#[test]
fn 반복_회차에서는_절대_차단하지_않는다() {
    let 첫째 = 훅("SubagentStop", &페이로드(Some(""), false));
    assert!(!출력(&첫째).is_empty(), "1회차가 차단을 안 냈다 — 이 시험은 아무것도 안 재고 있다");

    let 둘째 = 훅("SubagentStop", &페이로드(Some(""), true));
    assert!(둘째.status.success());
    assert!(출력(&둘째).is_empty(), "2회차가 또 차단했다 — 하네스가 이것을 못 멈춘다: {}", 출력(&둘째));
}

// ─────────────────────────────────────────────────────────────────────────────
// **가장 나쁜 실패는 훅이 오작동해서 사람의 작업을 막는 것이다**
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn 모르는_입력에서_조용히_통과한다() {
    for (tag, event, stdin) in [
        ("깨진 JSON", "SubagentStop", "{\"a\":"),
        ("빈 입력", "SubagentStop", ""),
        ("JSON 이 아닌 텍스트", "SubagentStop", "hello"),
        ("최상위가 배열", "SubagentStop", "[1,2]"),
        ("모르는 사건", "무엇인가", "{}"),
        ("빈 객체", "SubagentStop", "{}"),
        ("마지막 말이 없는 페이로드", "SubagentStop", "{\"hook_event_name\":\"SubagentStop\"}"),
    ] {
        let out = 훅(event, stdin);
        assert!(out.status.success(), "{tag}: exit 0 이 아니다");
        assert!(출력(&out).is_empty(), "{tag}: 차단했다 — {}", 출력(&out));
        assert!(!진단(&out).is_empty(), "{tag}: 진단을 아무 데도 안 적었다");
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// ★ 셸을 거친다 — **등록 문자열은 `/bin/sh -c` 로 실행된다**(실측)
// ─────────────────────────────────────────────────────────────────────────────

/// 경로에 **공백이 있어도** 셸을 거쳐 같은 답이 나온다. 따옴표가 없으면 여기서
/// exit 127 이 나고, 실측상 그 실패는 **하네스에서 완전히 침묵한다.**
#[test]
fn 공백이_든_경로가_셸을_거쳐도_돈다() {
    let dir = std::env::temp_dir().join(format!("pal f24 훅 {}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("방");
    let exe = dir.join("pal");
    std::fs::copy(PAL, &exe).expect("복사");
    복사한_것에_실행_권한(&exe);

    let 따옴표_없이 = format!("{} hook SubagentStop", exe.display());
    let out = 셸을_거쳐(&따옴표_없이, &페이로드(Some(""), false));
    assert!(출력(&out).is_empty(), "따옴표 없이도 돌았다 — 이 시험의 전제가 틀렸다");

    let 따옴표로 = format!("'{}' hook SubagentStop", exe.display());
    let out = 셸을_거쳐(&따옴표로, &페이로드(Some(""), false));
    let v: serde_json::Value =
        serde_json::from_str(출력(&out).trim()).expect("셸을 거치니 답이 안 나왔다");
    assert_eq!(v["decision"], "block");

    let _ = std::fs::remove_dir_all(&dir);
}

#[cfg(unix)]
fn 복사한_것에_실행_권한(exe: &Path) {
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(exe, std::fs::Permissions::from_mode(0o755)).expect("chmod");
}

#[cfg(not(unix))]
fn 복사한_것에_실행_권한(_exe: &Path) {}
