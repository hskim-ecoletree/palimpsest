//! **등록된 훅 명령이 발화하고 차단 결정이 전달된다** — `[f24]` ⑧.
//!
//! # 여기서 재는 것과 못 재는 것을 먼저 가른다
//!
//! 게이트 ⑧ 의 두 문장은 *"등록된 훅 명령이 발화한다"* 와 *"차단 결정이 하네스에
//! 전달된다"* 이다. 이 시험이 서는 자리는 **등록 문자열까지**다:
//!
//! | 재는 것 | 어떻게 |
//! |---|---|
//! | 등록된 항목이 **실제로 도는가** | 실측된 **exec form** 규약 그대로 `command` 를 실행 파일로 직접 띄우고 `args` 를 그대로 넘긴다. **셸이 없다** |
//! | 그 실행이 **차단 바이트를 내는가** | 표준출력이 `{"decision":"block","reason":…}` · 종료 코드 0 |
//! | **두 번 설치해도 한 번만 도는가** | 등록이 하나여야 한다 |
//! | 남의 등록을 **안 건드리는가** | 사용자 훅이 든 fixture 에서 왕복 후 값 비교 |
//! | **옛 형태를 새 형태로 옮기는가** | 손으로 shell form 을 심고 `update` 뒤에 남은 것을 센다 |
//! | **옛 형태도 걷어내는가** | 같은 fixture 에서 `uninstall` 뒤 사용자 설정과 바이트 비교 |
//!
//! **못 재는 것**: 하네스가 그 바이트를 받아 실제로 서브에이전트를 막는 마지막 한 칸.
//! 그것은 `claude` 세션을 실제로 돌려야 보이고 **이 회차는 안 했다.** 그래서 그 한 칸은
//! 통과로도 반증으로도 세지 않는다 — 이 파일이 재는 것은 **우리가 내는 바이트가 실측된
//! 규약과 같은가**까지다.
//!
//! # ★ 왜 공백이 든 경로에 바이너리를 복사해서 설치하는가
//!
//! 옛 형태(shell form)에서는 등록 문자열이 셸을 거쳐서 **공백이 있으면 따옴표가
//! 필요했다.** 새 형태에서는 `command` 가 실행 파일 경로 그 자체라 **따옴표가 있으면
//! 오히려 못 찾는다.** 어느 쪽이든 **공백 없는 경로로만 재면 그 차이가 안 드러난다** —
//! 우리 시험 바이너리는 `target/debug/deps/` 아래라 공백이 없다. 그래서 복사한다.

mod common;

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

use common::{PAL, git, path_앞에};

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

/// 이 플랫폼에서 **실제로 띄울 수 있는 파일 이름** — Windows 는 `.exe` 가 붙는다.
///
/// ★ 옛 fixture 는 `dir.join("pal")` 로 확장자 없는 사본을 만들었다. 그것은 Windows 에서
/// **애초에 실행되지 않는 파일**이라(실측: `Executable not found`) *"정상 설치"* 를 재는
/// 자리가 정상이 아니었다.
fn 실행_이름(stem: &str) -> String {
    format!("{stem}{}", std::env::consts::EXE_SUFFIX)
}

/// **공백이 든 디렉터리**에 `pal` 을 복사한다 — 따옴표를 재는 자리.
fn 공백이_든_곳의_pal(root: &Path, 이름: &str) -> PathBuf {
    let dir = root.join(format!("도구 {이름}"));
    std::fs::create_dir_all(&dir).expect("도구 방");
    let exe = dir.join(실행_이름("pal"));
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

/// 등록에 실리는 문자열 — **`PATH` 의 이름 하나.**
///
/// ★ 이름을 손으로 안 적는다. **빌드된 바이너리의 이름에서 유도한다** — 그것이
/// `layout::COMMAND_NAME` 이 뜻하는 바로 그 값이고(*"`PATH` 에서 우리를 부르는 이름"*),
/// 여기 `"pal"` 을 박으면 그것이 두 번째 목록이 된다. 확장자는 플랫폼이 붙이므로 뗀다.
fn 등록되는_이름() -> String {
    Path::new(PAL)
        .file_stem()
        .expect("pal 의 이름")
        .to_string_lossy()
        .into_owned()
}

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

/// 그 사건에 걸린 **항목** 전부 — **묶음 구조를 여기 한 번만 안다.**
fn 걸린_항목(설정: &serde_json::Value, event: &str) -> Vec<serde_json::Value> {
    설정["hooks"][event]
        .as_array()
        .map(|groups| {
            groups.iter().filter_map(|g| g["hooks"].as_array()).flatten().cloned().collect()
        })
        .unwrap_or_default()
}

/// 그 사건에 걸린 `command` 문자열 전부.
fn 걸린_명령(설정: &serde_json::Value, event: &str) -> Vec<String> {
    걸린_항목(설정, event)
        .iter()
        .filter_map(|c| c["command"].as_str())
        .map(str::to_owned)
        .collect()
}

/// 우리 등록 하나 — 없거나 둘이면 여기서 걸린다.
fn 우리_항목(root: &Path) -> serde_json::Value {
    let 전부 = 걸린_항목(&설정(root), "SubagentStop");
    let 우리것: Vec<_> = 전부.iter().filter(|c| c.get("args").is_some()).collect();
    assert_eq!(우리것.len(), 1, "우리 등록이 하나가 아니다: {전부:?}");
    우리것[0].clone()
}

// ─────────────────────────────────────────────────────────────────────────────
// ★ 실측된 규약 그대로 — **exec form. 셸이 없다**
//
// 스키마 원문: *"Argument list for exec form. When present, `command` is resolved as
// an executable and spawned directly with these arguments — **no shell**."*
// 그래서 이 자리도 `/bin/sh -c` 를 안 쓴다. **따옴표를 붙이면 오히려 못 찾는다.**
// ─────────────────────────────────────────────────────────────────────────────

fn 하네스처럼(exe: &Path, 항목: &serde_json::Value, payload: &str) -> Output {
    let command = 항목["command"].as_str().expect("command 가 문자열이 아니다");
    let args: Vec<&str> = 항목["args"]
        .as_array()
        .expect("args 가 배열이 아니다 — exec form 이 아니다")
        .iter()
        .map(|a| a.as_str().expect("인자가 문자열이 아니다"))
        .collect();
    // ★ **`PATH` 를 얹고 띄운다 — 하네스가 하는 그대로.**
    //
    // 등록 문자열은 이제 `PATH` 의 **이름 하나**다(`install/hooks.rs` 머리말). 하네스는
    // 사용자 환경의 `PATH` 로 그 이름을 푼다. 그래서 이 fixture 도 pal 이 사는 곳을
    // `PATH` 에 얹어야 「하네스가 하는 일」과 같은 모양이 된다 — 안 얹으면 이 시험은
    // 하네스가 안 겪는 조건을 재게 된다.
    let mut child = Command::new(command)
        .args(&args)
        .env("PATH", path_앞에(exe.parent().expect("pal 의 부모")))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("등록된 실행 파일을 못 돌렸다");
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

/// ★ **등록된 그 항목이 실제로 돌고, 차단 바이트를 낸다.**
///
/// 「`settings.json` 에 적혀 있다」로는 부족하다 — 파일이 사라지거나 실행 권한을 잃어도
/// 하네스는 그 실패를 **완전히 삼킨다.** 그래서 적힌 항목을 **실행해서** 잰다.
///
/// 그리고 **exec form 의 모양 자체**를 여기서 못박는다 — `command` 는 실행 파일 경로
/// **그 자체**이고(따옴표 없음), `args` 는 `["hook", "<사건>"]` 이며 **비어 있지 않다.**
/// ⚠ 실측: `args: []` 도 exec form 이고, 그때는 `command` **문자열 전체**가 실행 파일
/// 경로가 되어 ENOENT 로 죽는다.
#[test]
fn 등록된_명령이_돌고_차단을_낸다() {
    let root = 프로젝트("발화");
    let exe = 공백이_든_곳의_pal(root.parent().expect("부모"), "발화");
    성공(&exe, &root, &["install"]);

    let 항목 = 우리_항목(&root);
    assert_eq!(
        항목["command"].as_str(),
        Some(등록되는_이름().as_str()),
        "`command` 가 `PATH` 의 이름이 아니다 — 경로를 실으면 다른 기계에서 반드시 틀린다: {항목}"
    );
    assert_eq!(
        항목["args"],
        serde_json::json!(["hook", "SubagentStop"]),
        "`args` 가 우리 형태가 아니다: {항목}"
    );
    assert_eq!(항목["type"], "command");
    // ★ **셸 인용이 하나도 안 남았다.** 남으면 exec form 이 그것을 경로의 일부로 읽는다.
    assert!(
        !항목["command"].as_str().expect("command").contains('\''),
        "exec form 에 홑따옴표가 남았다: {항목}"
    );
    // ⚠ **`shell` 키를 안 쓴다** — enum 밖 값을 넣으면 그 훅 배열 전체가 조용히 사라진다.
    assert!(항목.get("shell").is_none(), "`shell` 키를 썼다: {항목}");

    // (a) 발화 — 부르면 흔적이 남는다.
    let out = 하네스처럼(&exe, &항목, &페이로드("다 했다", false));
    assert!(out.status.success(), "등록된 명령이 exit 0 을 안 냈다");
    assert!(
        String::from_utf8_lossy(&out.stderr).contains("pal hook"),
        "발화 흔적이 없다 — stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(out.stdout.is_empty(), "통과인데 표준출력이 있다");

    // (b) 차단 — 실측된 규약 그대로의 바이트가 나온다.
    let out = 하네스처럼(&exe, &항목, &페이로드("", false));
    assert!(out.status.success(), "차단인데 exit 0 이 아니다");
    let v: serde_json::Value = serde_json::from_slice(&out.stdout).expect("표준출력이 JSON 이 아니다");
    assert_eq!(v["decision"], "block", "차단 결정이 안 나왔다: {v}");
    assert!(!v["reason"].as_str().expect("reason").trim().is_empty());

    // 그리고 반복 회차에서는 같은 페이로드가 통과다.
    let out = 하네스처럼(&exe, &항목, &페이로드("", true));
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

/// ★ **실행 파일이 어디 있든 등록은 안 바뀐다 — 그리고 그것이 요점이다.**
///
/// ⚠ **옛 회차는 여기서 정반대를 쟀다**: *"실행 파일이 옮겨 가면 `update` 가 따라간다"*.
/// 그 시험이 통과한다는 것은 곧 **등록이 그 기계의 경로를 지고 있다**는 뜻이었고,
/// 그것이 다른 기계에서 훅을 확정적으로 죽였다(`install/hooks.rs` 머리말의 실측 표).
///
/// 지금 등록은 `PATH` 의 **이름 하나**다. 그래서 같은 상황에서:
///
/// - 등록 문자열은 **안 움직인다** — 그러니 추적 파일에 diff 가 안 생긴다
/// - 두 사람이 각자 다른 자리에 pal 을 두어도 **같은 문자열**을 쓴다
/// - 그 이름이 무엇을 가리키는지는 **그 기계의 `PATH`** 가 정한다
#[test]
fn 실행_파일이_옮겨_가도_등록이_안_흔들린다() {
    let root = 프로젝트("이사");
    let 부모 = root.parent().expect("부모").to_path_buf();
    let 옛 = 공백이_든_곳의_pal(&부모, "옛");
    성공(&옛, &root, &["install"]);
    let 옛_항목 = 우리_항목(&root);
    let 옛_설정 = std::fs::read(root.join(".claude/settings.json")).expect("읽기");
    let 옛_매니페스트 = std::fs::read(root.join(".claude/pal/manifest.json")).expect("읽기");

    // **다른 자리의 pal** 로 갱신한다 — 다른 사람의 기계를 흉내 내는 자리다.
    let 새 = 공백이_든_곳의_pal(&부모, "새");
    성공(&새, &root, &["update"]);

    let 지금 = 우리_항목(&root);
    assert_eq!(지금, 옛_항목, "등록이 기계를 탔다: {옛_항목} → {지금}");
    assert_eq!(지금["command"].as_str(), Some(등록되는_이름().as_str()));
    assert_eq!(걸린_명령(&설정(&root), "SubagentStop").len(), 1, "죽은 등록이 남았다");

    // ★ **추적 파일이 바이트로 안 움직인다.** 이 줄이 이 회차가 고친 것의 전부다 —
    // 움직이면 두 사람이 서로의 diff 를 되돌리는 핑퐁이 시작된다.
    assert_eq!(
        std::fs::read(root.join(".claude/settings.json")).expect("읽기"),
        옛_설정,
        "다른 자리의 pal 로 갱신했더니 `settings.json` 이 바뀌었다"
    );
    assert_eq!(
        std::fs::read(root.join(".claude/pal/manifest.json")).expect("읽기"),
        옛_매니페스트,
        "다른 자리의 pal 로 갱신했더니 매니페스트가 바뀌었다"
    );

    성공(&새, &root, &["uninstall"]);
    assert!(!root.join(".claude/settings.json").exists(), "우리가 만든 설정이 남았다");
}

// ─────────────────────────────────────────────────────────────────────────────
// ★ 옛 형태(shell form) — **옮기고, 걷어낸다**
//
// 이미 설치된 프로젝트는 `'<경로>' hook <사건>` 한 문자열로 걸려 있다. `update` 가
// 그것을 안 옮기면 **그 프로젝트들만 영원히 셸을 거치고**, `uninstall` 이 그것을 못
// 걷으면 **죽은 등록이 남는다.** 둘 다 실패가 침묵하는 자리다.
// ─────────────────────────────────────────────────────────────────────────────

fn 매니페스트(root: &Path) -> serde_json::Value {
    serde_json::from_slice(&std::fs::read(root.join(".claude/pal/manifest.json")).expect("읽기"))
        .expect("JSON")
}

fn 매니페스트_쓰기(root: &Path, m: &serde_json::Value) {
    std::fs::write(
        root.join(".claude/pal/manifest.json"),
        serde_json::to_string_pretty(m).expect("직렬화"),
    )
    .expect("쓰기");
}

/// 설치된 새 형태를 **옛 형태로 되돌린다** — 이 회차 이전의 설치본을 그대로 재현한다.
///
/// 옛 등록 문자열은 `'<경로>' hook <사건>` 이었고 매니페스트에 `args` 가 없었다.
fn 옛_형태로_되돌린다(root: &Path) -> String {
    let 옛_명령 = {
        let 항목 = 우리_항목(root);
        format!("'{}' hook SubagentStop", 항목["command"].as_str().expect("command"))
    };

    let mut s = 설정(root);
    s["hooks"]["SubagentStop"] =
        serde_json::json!([{"hooks": [{"type": "command", "command": 옛_명령}]}]);
    std::fs::write(
        root.join(".claude/settings.json"),
        serde_json::to_string_pretty(&s).expect("직렬화"),
    )
    .expect("쓰기");

    let mut m = 매니페스트(root);
    m["settings"]["hooks"] =
        serde_json::json!([{"event": "SubagentStop", "command": 옛_명령}]);
    매니페스트_쓰기(root, &m);
    옛_명령
}

/// ★ **`update` 가 옛 형태를 새 형태로 옮긴다.** 안 옮기면 이미 설치된 프로젝트가
/// 옛 형태로 남는다 — 그리고 그 프로젝트만 계속 셸을 거친다.
#[test]
fn 갱신이_옛_형태를_새_형태로_옮긴다() {
    let root = 프로젝트("이주");
    let exe = 공백이_든_곳의_pal(root.parent().expect("부모"), "이주");
    성공(&exe, &root, &["install"]);
    let 옛_명령 = 옛_형태로_되돌린다(&root);

    let report = 성공(&exe, &root, &["update"]);
    assert!(report.contains("훅"), "훅을 갱신했다고 말하지 않았다:\n{report}");

    // 옛 문자열이 하나도 안 남았다 — 남으면 같은 훅이 두 번 돈다.
    let 전부 = 걸린_명령(&설정(&root), "SubagentStop");
    assert_eq!(전부.len(), 1, "옛 등록이 남았다: {전부:?}");
    assert!(!전부.contains(&옛_명령), "옛 형태가 그대로다: {전부:?}");

    let 지금 = 우리_항목(&root);
    assert_eq!(지금["command"].as_str(), Some(등록되는_이름().as_str()));
    assert_eq!(지금["args"], serde_json::json!(["hook", "SubagentStop"]));

    // 매니페스트도 새 형태를 적었다 — 안 적으면 제거가 새 형태를 못 되돌린다.
    let m = 매니페스트(&root);
    assert_eq!(m["settings"]["hooks"][0]["args"], serde_json::json!(["hook", "SubagentStop"]));

    // 그리고 옮긴 뒤에도 그 항목이 실제로 돈다.
    let out = 하네스처럼(&exe, &지금, &페이로드("", false));
    let v: serde_json::Value = serde_json::from_slice(&out.stdout).expect("표준출력이 JSON 이 아니다");
    assert_eq!(v["decision"], "block");
}

/// ★ **`uninstall` 이 옛 형태도 걷어낸다.** 갱신을 안 거치고 바로 지우는 경로다.
#[test]
fn 제거가_옛_형태도_걷어낸다() {
    let root = 프로젝트("옛제거");
    let exe = 공백이_든_곳의_pal(root.parent().expect("부모"), "옛제거");
    std::fs::create_dir_all(root.join(".claude")).expect(".claude");
    let 원본 = serde_json::json!({
        "env": {"A": "1"},
        "hooks": {"SubagentStop": [{"hooks": [{"type": "command", "command": "내 것.sh"}]}]}
    });
    std::fs::write(
        root.join(".claude/settings.json"),
        serde_json::to_string_pretty(&원본).expect("직렬화"),
    )
    .expect("settings");

    성공(&exe, &root, &["install"]);
    옛_형태로_되돌린다(&root);
    assert_eq!(걸린_명령(&설정(&root), "SubagentStop").len(), 1, "이 시험이 재려는 상태가 아니다");

    성공(&exe, &root, &["uninstall"]);
    // 옛 형태를 걷어내되 **남의 것은 안 건드린다.** 위에서 우리가 사건 배열을 통째로
    // 갈아 놨으므로 남의 등록이 사라진 상태이고, 그것까지 되살리지는 않는다 —
    // 여기서 재는 것은 **우리 옛 등록이 남지 않는가**다.
    let 남은 = 걸린_명령(&설정(&root), "SubagentStop");
    assert!(남은.is_empty(), "옛 형태가 제거 뒤에 남았다: {남은:?}");
    assert_eq!(설정(&root)["env"], serde_json::json!({"A": "1"}), "사용자 키가 사라졌다");
}

// ─────────────────────────────────────────────────────────────────────────────
// `doctor` — **「적혀 있다」로는 부족하다**
// ─────────────────────────────────────────────────────────────────────────────

/// 설치 검사를 JSON 으로 뜬다 — **`PATH` 를 얹고**.
///
/// # ★ 이 한 줄이 없어서 Windows 만 초록이었다 (실측 2026-08-17 · CI 1회차)
///
/// 등록 문자열은 `PATH` 의 **이름 하나**이므로 검사 4·6 은 그 이름을 `PATH` 에서
/// 푼다. 그런데 여기는 `PATH` 를 안 얹었고, 그래도 **Windows 에서는 통과했다** —
/// `cargo test` 가 동적 라이브러리 탐색 경로에 `target/debug` 를 얹는데 **그 변수가
/// 이 플랫폼에서는 `PATH` 자신**이기 때문이다. 유닉스에서 같은 값은
/// `LD_LIBRARY_PATH`(리눅스)·`DYLD_FALLBACK_LIBRARY_PATH`(macOS) 로 가고 `PATH` 는
/// 안 건드린다. 그래서 **같은 fixture 구멍이 한쪽에서만 빨갰다.**
///
/// ⚠ 그때 Windows 가 통과한 것은 **재려던 것을 잰 결과가 아니다.** 「하네스가 이
/// 이름을 풀 수 있는가」가 아니라 「cargo 가 마침 그 디렉터리를 얹어 줬는가」를
/// 쟀다. 얹어야 [`하네스처럼`] 과 같은 모양이 되고, 그것이 이 파일의 규율이다.
fn 검사들(exe: &Path, cwd: &Path) -> serde_json::Value {
    let out = Command::new(PAL)
        .args(["doctor", "--install", "--json"])
        .current_dir(cwd)
        .env("PATH", path_앞에(exe.parent().expect("pal 의 부모")))
        .output()
        .expect("pal doctor");
    serde_json::from_slice(&out.stdout).expect("JSON")
}

fn 훅_검사(exe: &Path, cwd: &Path) -> serde_json::Value {
    let c = 검사들(exe, cwd);
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
    let c = 훅_검사(&exe, &root);
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
    let c = 훅_검사(&exe, &root);
    assert_eq!(c["outcome"], "failed", "등록이 사라졌는데 안 걸렸다: {c}");
    std::fs::write(root.join(".claude/settings.json"), &원래_설정).expect("되돌리기");

    // ── 있는데 안 열린다 — 빨강 (exit 126) ─────────────────────────────────
    //
    // 유닉스는 모드 비트로, Windows 는 확장자로 만든다 — **축이 다르고 사건은 같다.**
    let 막힌 = 못_열리게_한다(&root, &exe);
    assert!(막힌.is_file(), "등록된 자리에 파일이 없다 — 그러면 재려는 것이 「없다」가 된다");
    let c = 훅_검사(&exe, &root);
    assert_eq!(c["outcome"], "failed", "실행될 수 없는데 안 걸렸다: {c}");
    // ★ **어느 겹이 걸었는지까지 못 박는다.** 「빨갛다」만 재면 바이트 대조가 대신
    // 걸어도 통과하고, 그러면 이 칸은 자기가 재려던 것을 안 잰다.
    assert!(
        c["detail"].as_str().expect("detail").contains("실행될 수 없다"),
        "다른 겹이 걸었다 — 실행 가능성 겹이 안 섰다: {c}"
    );
    다시_열리게_한다(&root, &exe, &막힌);
    assert_eq!(훅_검사(&exe, &root)["outcome"], "ok", "되돌렸는데 초록이 아니다");

    // ── 파일이 사라졌다 — 빨강 (exit 127) ──────────────────────────────────
    std::fs::remove_file(&exe).expect("지우기");
    let c = 훅_검사(&exe, &root);
    assert_eq!(c["outcome"], "failed", "실행 파일이 없는데 안 걸렸다: {c}");
    assert!(
        c["detail"].as_str().expect("detail").contains("127"),
        "왜인지 안 적었다: {c}"
    );
}

/// ★ **옛 형태로 걸린 등록을 지목한다 — 그리고 「우리 것이 아니다」라고 안 한다.**
///
/// 옛 형태는 우리 것이 맞다. 사람에게 필요한 말은 *"형태가 아니다"* 가 아니라
/// ***"`pal update` 를 돌리십시오"*** 다.
#[test]
fn 진단이_옛_형태를_지목한다() {
    let root = 프로젝트("진단-옛형태");
    let exe = 공백이_든_곳의_pal(root.parent().expect("부모"), "진단옛");
    성공(&exe, &root, &["install"]);
    옛_형태로_되돌린다(&root);

    let c = 훅_검사(&exe, &root);
    assert_eq!(c["outcome"], "failed", "옛 형태인데 초록이다: {c}");
    let detail = c["detail"].as_str().expect("detail");
    assert!(detail.contains("update"), "무엇을 하라고 안 적었다: {detail}");
}

/// ★ **남이 심은 항목은 절대 안 돌린다.** 매니페스트도 설정도 남이 커밋해 보낼 수 있다.
///
/// 옛 회차가 `command` 를 `/bin/sh -c` 로 돌려서 `pal doctor` 한 번이 임의 코드 실행
/// 이었다. exec form 은 셸을 안 거치지만 **`command` 를 실행 파일로 직접 띄우므로**
/// 규율은 그대로여야 한다 — `args` 가 우리 것이 아니면 **띄우지 않는다.**
#[test]
fn 진단이_남이_심은_항목을_안_돌린다() {
    let 흔적 = std::env::temp_dir().join(format!("pal-f24-PWNED-{}", std::process::id()));
    let _ = std::fs::remove_file(&흔적);

    let root = 프로젝트("진단-남의것");
    let exe = 공백이_든_곳의_pal(root.parent().expect("부모"), "진단남");
    성공(&exe, &root, &["install"]);

    let 남의_것 = serde_json::json!({
        "type": "command",
        "command": "/usr/bin/touch",
        "args": [흔적.display().to_string()],
    });
    let mut s = 설정(&root);
    s["hooks"]["SubagentStop"] = serde_json::json!([{"hooks": [남의_것.clone()]}]);
    std::fs::write(
        root.join(".claude/settings.json"),
        serde_json::to_string_pretty(&s).expect("직렬화"),
    )
    .expect("쓰기");
    let mut m = 매니페스트(&root);
    m["settings"]["hooks"] = serde_json::json!([{
        "event": "SubagentStop",
        "command": "/usr/bin/touch",
        "args": [흔적.display().to_string()],
    }]);
    매니페스트_쓰기(&root, &m);

    let c = 훅_검사(&exe, &root);
    assert_eq!(c["outcome"], "failed", "남이 심은 항목을 초록으로 냈다: {c}");
    assert!(!흔적.exists(), "진단이 남이 심은 명령을 실행했다: {}", 흔적.display());
    let _ = std::fs::remove_file(&흔적);
}

/// ★ **훅 `command` 만 경계 검사의 모집단 밖이었다.**
///
/// `settings.hooks[].command` 는 `Rel` 이 아니라 `String` 이라 `Manifest::경로들` ·
/// `자리들` 이 **원리상** 그것을 못 본다. 그래서 매니페스트와 `settings.json` 의
/// `command` 를 **대상 밖 실행 파일**로 맞춰 두면 검사 6 이 이렇게 답했다(실측):
///
/// ```text
/// ok   6  등록된 훅이 실제로 도는가
///      등록된 1개가 설정과 맞고 그 자리가 실행될 수 있다.
/// ```
///
/// 하네스가 `SubagentStop` 에 실제로 띄울 것은 **그 파일**인데 진단은 「이상 없음」이다.
/// 탐침이 「돈다」고 말할 때 실제로 돌려 본 것은 **지금 도는 이 실행 파일**이라, 둘이
/// 다르면 그 확인이 등록된 것에 대해 **아무것도 말하지 않는다.**
///
/// ⚠ **판정은 「대상 안인가」가 아니다.** 우리가 등록하는 것은 설치 시점에 해석한 `pal`
/// 의 절대 경로라 **대상 밖이 정상**이다. 그러니 **「우리가 등록한 그것인가」**로 댄다.
///
/// ⚠ **그리고 여전히 안 돌린다.** 저장소에서 읽은 문자열을 실행하는 구멍은 앞 회차가
/// 막았고, 이 회차는 그 규율 위에서 **대조만** 더한다.
#[test]
fn 진단이_대상_밖_실행_파일을_초록으로_안_낸다() {
    let 흔적 = std::env::temp_dir().join(format!("pal-f24-훅경계-{}", std::process::id()));
    let _ = std::fs::remove_file(&흔적);

    let root = 프로젝트("진단-밖의exe");
    성공(Path::new(PAL), &root, &["install"]);

    // **대상 밖**에 실행 파일 하나를 심는다 — 있고, 일반 파일이고, 실행 권한이 있다.
    // 그 셋이 `실행할_수_있나` 가 보는 전부라 옛 코드는 여기서 초록이었다.
    let 밖 = root.parent().expect("부모").join("밖");
    std::fs::create_dir_all(&밖).expect("밖");
    // ⚠ **이 플랫폼에서 실제로 띄울 수 있는 이름이어야 한다.** 확장자 없는 이름으로
    // 심으면 Windows 에서는 **바깥 겹(실행 가능한가)이 먼저 걸려서** 이 시험이 재려는
    // 겹(「우리가 등록한 그것인가」)에 닿지도 못한다 — 빨강은 나지만 다른 이유로 난다.
    let 심은것 = 밖.join(실행_이름("남의것"));
    std::fs::write(&심은것, format!("#!/bin/sh\ntouch {}\n", 흔적.display())).expect("심기");
    실행_권한(&심은것);

    let 자리 = 심은것.display().to_string();
    let mut s = 설정(&root);
    s["hooks"]["SubagentStop"] = serde_json::json!([{
        "hooks": [{"type": "command", "command": 자리, "args": ["hook", "SubagentStop"]}]
    }]);
    std::fs::write(
        root.join(".claude/settings.json"),
        serde_json::to_string_pretty(&s).expect("직렬화"),
    )
    .expect("쓰기");
    let mut m = 매니페스트(&root);
    m["settings"]["hooks"] = serde_json::json!([{
        "event": "SubagentStop", "command": 자리, "args": ["hook", "SubagentStop"]
    }]);
    매니페스트_쓰기(&root, &m);

    let c = 훅_검사(Path::new(PAL), &root);
    assert_eq!(c["outcome"], "failed", "대상 밖 실행 파일을 초록으로 냈다: {c}");
    let detail = c["detail"].as_str().expect("detail");
    assert!(detail.contains("update"), "무엇을 하라고 안 적었다: {detail}");

    // ★ **완화는 그대로다** — 진단이 그것을 돌리지 않는다.
    assert!(!흔적.exists(), "진단이 심어 둔 실행 파일을 돌렸다: {}", 흔적.display());
    let _ = std::fs::remove_file(&흔적);
}

/// 설치가 없으면 **`Residual`** 이다 — 검사하지 못한 것은 「이상 없음」이 아니다.
#[test]
fn 설치가_없으면_훅_검사가_잔여다() {
    let root = 프로젝트("진단-잔여");
    assert_eq!(훅_검사(Path::new(PAL), &root)["outcome"], "residual");
}

// ─────────────────────────────────────────────────────────────────────────────
// ★ 「있는데 안 열린다」 — **축은 플랫폼마다 다르고, 사건은 같다**
// ─────────────────────────────────────────────────────────────────────────────
//
// 재려는 상태 하나: **등록된 자리에 파일이 있고 바이트도 우리 것인데, OS 가 그것을
// 못 띄운다.** 하네스는 그 실패를 완전히 삼키므로 `pal doctor` 가 유일한 문이다.
//
// | 플랫폼 | 그 상태를 어떻게 만드나 |
// |---|---|
// | 유닉스 | `chmod -x` — 모드 비트가 실행을 정한다 |
// | Windows | **확장자를 뗀다** — 확장자가 실행을 정한다(실측: 옆에 `.exe` 가 없는 확장자 없는 이름은 `Executable not found`) |
//
// ⚠ 옛 회차는 Windows 쪽을 **무동작 stub** 으로 뒀고, 그 뒤의 단언은 *"빨개져야 한다"*
// 였다. 아무것도 안 바꾸고 빨강을 요구했으니 **이 시험은 Windows 에서 언제나 실패**했다 —
// 그리고 그 실패는 제품의 결함이 아니라 fixture 의 결함이었다. 무동작 stub 은 짝이
// 아니다. 짝이 없으면 그 자리는 **초록을 내며 아무것도 안 재거나, 빨강을 내며 아무것도
// 안 가리킨다.**

/// 등록된 자리를 **있지만 안 열리는 상태**로 만든다. 반환은 **지금 등록된 자리**.
///
/// # ★ **등록을 절대 경로로 옮기는 것이 두 축에 다 필요하다** (실측 2026-08-17 · CI 2회차)
///
/// 앞 판은 Windows 쪽만 [`등록을_옮긴다`] 를 불렀다. 유닉스 쪽은 `chmod -x` 만 하고
/// 등록을 **이름 하나(`pal`)로 둔 채**였고, 그래서 그 플랫폼에서는 **다른 겹이 먼저
/// 걸렸다**:
///
/// ```text
/// 유닉스 (앞 판)  →  "PATH 어디에도 `pal` 이 없다"     ← 이름 해석 겹
/// Windows        →  "등록된 자리가 실행될 수 없다"      ← 실행 가능성 겹  ← 재려던 것
/// ```
///
/// 까닭은 `exe::명령을_찾는다` 가 유닉스에서 **실행 비트를 보고 거른다**는 것이다 —
/// 실행 비트를 떼는 순간 그 이름은 `PATH` 에서 아예 안 잡힌다. 그러니 「등록된 자리에
/// 파일이 있는데 안 열린다」를 재려면 **등록이 그 자리를 직접 가리켜야** 한다.
///
/// 두 플랫폼이 **같은 겹**을 재게 맞춘다. 축은 여전히 다르다(모드 비트 대 확장자) —
/// 그것은 없앨 수 없는 차이이고, 없앨 수 있었던 것은 **fixture 의 모양**이었다.
fn 못_열리게_한다(root: &Path, exe: &Path) -> PathBuf {
    #[cfg(unix)]
    let 막힌 = {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(exe, std::fs::Permissions::from_mode(0o644)).expect("chmod");
        exe.to_path_buf()
    };
    // Windows — 확장자를 뗀다. 파일은 그 자리에 있고 OS 만 그것을 못 띄운다.
    #[cfg(windows)]
    let 막힌 = {
        let 확장자없음 = exe.with_extension("");
        std::fs::rename(exe, &확장자없음).expect("확장자 떼기");
        확장자없음
    };
    // ★ **등록을 그 자리로 옮긴다.** 안 옮기면 이름 해석 겹이 먼저 걸려서 이 시험이
    // 재려는 겹에 닿지도 못한다 — 위 실측이 그것이다.
    등록을_옮긴다(root, &막힌);
    막힌
}

fn 다시_열리게_한다(root: &Path, exe: &Path, 막힌: &Path) {
    #[cfg(unix)]
    {
        let _ = 막힌;
        실행_권한(exe);
    }
    #[cfg(windows)]
    std::fs::rename(막힌, exe).expect("확장자 되붙이기");
    등록을_옮긴다(root, exe);
}

/// 설정과 매니페스트의 `command` 를 함께 옮긴다 — **둘이 어긋나면 앞 겹이 먼저 걸린다.**
fn 등록을_옮긴다(root: &Path, 자리: &Path) {
    let 자리 = 자리.display().to_string();
    let mut s = 설정(root);
    s["hooks"]["SubagentStop"] = serde_json::json!([{
        "hooks": [{"type": "command", "command": 자리, "args": ["hook", "SubagentStop"]}]
    }]);
    std::fs::write(
        root.join(".claude/settings.json"),
        serde_json::to_string_pretty(&s).expect("직렬화"),
    )
    .expect("쓰기");

    let mut m = 매니페스트(root);
    m["settings"]["hooks"] = serde_json::json!([{
        "event": "SubagentStop", "command": 자리, "args": ["hook", "SubagentStop"]
    }]);
    매니페스트_쓰기(root, &m);
}
