//! 회차 `2026-09-15-clean-uninstall` 갈래 T1 — **`settings.json` 위치 보존 편집**(완수 조건 A1 · A2 · A3 · A4).
//!
//! 무엇을 재나:
//!
//! | 조건 | 방 | 기대 |
//! |---|---|---|
//! | A1 | 사용자 파일 형태 모집단(열아홉) | install → uninstall 뒤 **바이트 동일** |
//! | A2 | 같은 모집단 | install 결과가 **착수 커밋 `acd7e82` 빌드로 뜬 골든**과 값으로 같다 |
//! | A3 | 설치 뒤 사용자가 고친 방 셋 | uninstall 결과가 원본에 **같은 텍스트 편집**을 적용한 바이트 |
//! | A4 | 착수 규칙(재직렬화)으로 설치된 방 넷 | `HEAD` 바이트로 되쓰기 · 아니면 값만 되돌리고 그렇게 출력 |
//!
//! ★ **모든 방은 대상이 실제로 생겼음을 uninstall 전에 단언한다** — 안 생긴 것을 안 남았다고 세지 않는다.
//!
//! 픽스처는 `tests/fixtures/clean_uninstall/settings/` 에 있고 그 자리의 `.gitattributes` 가 줄바꿈 변환을 끈다 —
//! 체크아웃이 바이트를 바꾸면 플랫폼마다 다른 입력을 재게 된다(ADR-0023). 시험이 만드는 저장소도
//! `core.autocrlf=false` 로 세운다. 골든과 옛 설치 fixture 를 뜬 절차와 출력은
//! `.palimpsest/rounds/2026-09-15-clean-uninstall/oracle/T1-golden-generation.txt` 에 있다.

mod common;

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use serde_json::Value;

use common::{PAL, git};

const 픽스처: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/clean_uninstall/settings");
const 설정: &str = ".claude/settings.json";

/// A1 · A2 의 모집단 — 이름이 곧 형태다.
const 모집단: &[&str] = &[
    "a1-01-indent2",
    "a1-02-indent4",
    "a1-03-tab",
    "a1-04-one-line",
    "a1-05-crlf",
    "a1-06-no-final-newline",
    "a1-07-empty-object",
    "a1-08-same-event-other-hook",
    "a1-09-other-event-hook",
    "a1-10-no-hooks",
    "a1-11-empty-event-array",
    "a1-12-two-empty-event-arrays-and-other-key",
    "a1-13-key-order",
    "a1-14-escapes-and-numbers",
    "a1-15-inline-pal-event-array",
    "a1-16-empty-hooks-object",
    "a1-17-bare-empty-object",
    "a1-18-crlf-indent4-no-final-newline",
    "a1-19-compact-no-spaces",
];

const 값만: &str = "값만 되돌렸다";

// ─────────────────────────────────────────────────────────────────────────────
// 방
// ─────────────────────────────────────────────────────────────────────────────

fn 픽스처_바이트(이름: &str) -> Vec<u8> {
    let p = Path::new(픽스처).join(이름);
    std::fs::read(&p).unwrap_or_else(|e| panic!("픽스처를 못 읽었다: {} — {e}", p.display()))
}

/// git 저장소 하나 — HOME 은 그 옆의 빈 디렉터리다.
fn 방(tag: &str) -> PathBuf {
    let base = std::env::temp_dir().join(format!("pal-cu-settings-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    let root = base.join("repo");
    std::fs::create_dir_all(root.join(".claude")).expect("방");
    std::fs::create_dir_all(base.join("home")).expect("HOME");
    git(&root, &["init", "-q", "."]);
    git(&root, &["config", "core.autocrlf", "false"]);
    std::fs::write(root.join("README.md"), "hello\n").expect("README");
    root
}

fn 커밋(root: &Path, paths: &[&str], 메시지: &str) {
    let mut args = vec!["add", "--"];
    args.extend_from_slice(paths);
    git(root, &args);
    git(root, &["-c", "user.email=t@example.com", "-c", "user.name=t", "commit", "-qm", 메시지]);
}

fn 돌린다(root: &Path, args: &[&str]) -> Output {
    let home = root.parent().expect("부모").join("home");
    Command::new(PAL)
        .args(args)
        .current_dir(root)
        .env("HOME", &home)
        .env("XDG_DATA_HOME", home.join(".local/share"))
        .env("XDG_CONFIG_HOME", home.join(".config"))
        .output()
        .expect("pal 을 못 돌렸다")
}

fn 성공(root: &Path, args: &[&str]) -> String {
    let out = 돌린다(root, args);
    let 화면 = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(out.status.success(), "pal {args:?} 실패:\n{화면}");
    화면
}

fn 바이트(root: &Path) -> Vec<u8> {
    std::fs::read(root.join(설정)).expect("settings.json 을 못 읽었다")
}

fn 값(bytes: &[u8]) -> Value {
    serde_json::from_slice(bytes).unwrap_or_else(|e| {
        panic!("JSON 이 아니다 — {e}:\n{}", String::from_utf8_lossy(bytes))
    })
}

/// ★ **생김 단언** — 설치가 우리 몫(`agent` · 두 사건의 우리 훅)을 실제로 더했다.
fn 우리_몫이_생겼다(v: &Value, 어디: &str) {
    assert_eq!(v["agent"], "pal-orchestrator", "{어디}: agent 가 안 생겼다: {v}");
    for 사건 in ["Stop", "SubagentStop"] {
        let 걸림 = v["hooks"][사건].as_array().is_some_and(|groups| {
            groups.iter().any(|g| {
                g["hooks"].as_array().is_some_and(|cmds| {
                    cmds.iter().any(|c| c["command"] == "pal" && c["args"] == serde_json::json!(["hook", 사건]))
                })
            })
        });
        assert!(걸림, "{어디}: {사건} 에 우리 훅이 안 생겼다: {v}");
    }
}

fn 보인다(bytes: &[u8]) -> String {
    format!("{:?}", String::from_utf8_lossy(bytes))
}

// ─────────────────────────────────────────────────────────────────────────────
// A1 — 모집단 왕복이 바이트로 같다
// ─────────────────────────────────────────────────────────────────────────────

/// **A1** — 사용자 `settings.json` 형태 모집단마다 install → uninstall 뒤 **바이트 동일**.
///
/// 한 형태가 어긋나도 나머지를 끝까지 재서 어긋난 형태 전부를 한 번에 적는다.
#[test]
fn a1_모집단마다_설치했다_걷으면_바이트로_같다() {
    assert!(모집단.len() >= 15, "모집단이 열다섯 가지보다 적다: {}", 모집단.len());
    let mut 어긋남 = Vec::new();
    for 이름 in 모집단 {
        let 원본 = 픽스처_바이트(&format!("{이름}.json"));
        let root = 방(&format!("a1-{이름}"));
        std::fs::write(root.join(설정), &원본).expect("쓰기");

        성공(&root, &["install"]);
        우리_몫이_생겼다(&값(&바이트(&root)), 이름);
        성공(&root, &["uninstall"]);

        let 뒤 = 바이트(&root);
        if 뒤 != 원본 {
            어긋남.push(format!("  {이름}\n    원본: {}\n    뒤:   {}", 보인다(&원본), 보인다(&뒤)));
        }
    }
    assert!(
        어긋남.is_empty(),
        "왕복 뒤 바이트가 원본과 다른 형태 {}/{}:\n{}",
        어긋남.len(),
        모집단.len(),
        어긋남.join("\n")
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// A2 — install 결과가 착수 빌드의 골든과 값으로 같다
// ─────────────────────────────────────────────────────────────────────────────

/// **A2** — 같은 모집단에서 install 결과가 **착수 커밋 `acd7e82` 빌드로 뜬 골든 JSON** 과 값으로 같다.
///
/// 기대값을 지금 빌드의 병합 코드로 계산하지 않는다 — 그러면 변이가 기대값도 함께 바꾼다.
#[test]
fn a2_설치_결과가_착수_빌드의_골든과_값으로_같다() {
    let mut 어긋남 = Vec::new();
    for 이름 in 모집단 {
        let 원본 = 픽스처_바이트(&format!("{이름}.json"));
        let 골든 = 값(&픽스처_바이트(&format!("{이름}.golden.json")));
        // 골든 자체가 우리 몫을 담고 있다 — 빈 골든이면 이 대조가 공짜로 통과한다.
        우리_몫이_생겼다(&골든, &format!("{이름}.golden.json"));

        let root = 방(&format!("a2-{이름}"));
        std::fs::write(root.join(설정), &원본).expect("쓰기");
        성공(&root, &["install"]);
        let 지금 = 값(&바이트(&root));
        if 지금 != 골든 {
            어긋남.push(format!("  {이름}\n    골든: {골든}\n    지금: {지금}"));
        }
    }
    assert!(
        어긋남.is_empty(),
        "설치 결과가 착수 빌드의 골든과 값으로 다른 형태 {}/{}:\n{}",
        어긋남.len(),
        모집단.len(),
        어긋남.join("\n")
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// A3 — 설치 뒤 사용자가 고친 방 셋
// ─────────────────────────────────────────────────────────────────────────────

/// 딱 한 번 나오는 조각의 자리. 두 번 이상이면 편집이 모호하다 — 시험이 멈춘다.
fn 한_번(text: &str, 조각: &str, 어디: &str) -> usize {
    let 수 = text.matches(조각).count();
    assert_eq!(수, 1, "{어디}: `{조각}` 이 {수} 번 나온다 — 편집 자리가 모호하다:\n{text}");
    text.find(조각).expect("자리")
}

/// 편집 ㉠ — `닻` 줄 바로 뒤에 `새_줄` 을 넣는다(줄바꿈은 LF — 픽스처가 LF 다).
fn 줄_뒤에_넣는다(text: &str, 닻: &str, 새_줄: &str, 어디: &str) -> String {
    let 줄 = format!("\n{닻}\n");
    let at = 한_번(text, &줄, 어디) + 줄.len();
    format!("{}{새_줄}\n{}", &text[..at], &text[at..])
}

/// 편집 ㉡ — 내용이 `내용` 인 줄의 들여쓰기를 `새_들여` 로 바꾼다.
fn 들여쓰기를_바꾼다(text: &str, 옛_줄: &str, 새_줄: &str, 어디: &str) -> String {
    let 옛 = format!("\n{옛_줄}\n");
    let at = 한_번(text, &옛, 어디);
    format!("{}\n{새_줄}\n{}", &text[..at], &text[at + 옛.len()..])
}

/// 편집 ③ — `Stop` 배열의 닫는 줄 바로 앞에 사용자 묶음 원소를 넣는다.
fn stop_배열_끝에_묶음을_넣는다(text: &str, 어디: &str) -> String {
    let 여는 = 한_번(text, "\"Stop\": [", 어디);
    let 닫는 = text[여는..].find("\n    ]").map(|i| 여는 + i).expect("Stop 배열의 닫는 줄");
    format!(
        "{},\n      {{\"hooks\": [{{\"type\": \"command\", \"command\": \"mine.sh\"}}]}}{}",
        &text[..닫는],
        &text[닫는..]
    )
}

fn a3_설치한_방(tag: &str) -> (PathBuf, String) {
    let 원본 = String::from_utf8(픽스처_바이트("a3-base.json")).expect("UTF-8");
    let root = 방(tag);
    std::fs::write(root.join(설정), &원본).expect("쓰기");
    성공(&root, &["install"]);
    우리_몫이_생겼다(&값(&바이트(&root)), tag);
    (root, 원본)
}

fn 같아야_한다(뒤: &[u8], 기대: &str, 어디: &str) {
    assert!(
        뒤 == 기대.as_bytes(),
        "{어디}: uninstall 결과가 기대 바이트와 다르다\n  기대: {}\n  뒤:   {}",
        보인다(기대.as_bytes()),
        보인다(뒤)
    );
}

/// **A3 ①** — ㉠ 최상위에 키 한 줄을 더하고 ㉡ pal 이 안 만진 원래 멤버 한 줄의 들여쓰기를 바꿨다.
/// 기대 바이트는 원본에 같은 두 편집을 적용한 것이다.
#[test]
fn a3_1_사용자가_줄을_더하고_들여쓰기를_바꾼_방() {
    let (root, 원본) = a3_설치한_방("a3-1");
    let 편집 = |t: &str, 어디: &str| {
        let t = 줄_뒤에_넣는다(t, "  \"model\": \"opus\",", "  \"mine\": true,", 어디);
        들여쓰기를_바꾼다(&t, "  \"env\": {", "    \"env\": {", 어디)
    };
    let 설치된 = String::from_utf8(바이트(&root)).expect("UTF-8");
    std::fs::write(root.join(설정), 편집(&설치된, "설치된 파일")).expect("쓰기");

    성공(&root, &["uninstall"]);
    같아야_한다(&바이트(&root), &편집(&원본, "원본"), "A3 ①");
}

/// **A3 ②** — pal 이 더한 `agent` 값을 사용자가 자기 값으로 바꿨다. 그 키는 **남는다** —
/// `agent` 줄이 사용자 값으로 원본 끝 멤버 뒤에 남는다.
#[test]
fn a3_2_사용자가_agent_값을_바꾼_방() {
    let (root, 원본) = a3_설치한_방("a3-2");
    let 설치된 = String::from_utf8(바이트(&root)).expect("UTF-8");
    let at = 한_번(&설치된, "\"agent\": \"pal-orchestrator\"", "설치된 파일");
    let 바꾼 = format!(
        "{}\"agent\": \"내 에이전트\"{}",
        &설치된[..at],
        &설치된[at + "\"agent\": \"pal-orchestrator\"".len()..]
    );
    std::fs::write(root.join(설정), 바꾼).expect("쓰기");

    let 화면 = 성공(&root, &["uninstall"]);
    let 끝 = 원본.rfind("\n}").expect("원본의 닫는 줄");
    let 기대 = format!("{},\n  \"agent\": \"내 에이전트\"{}", &원본[..끝], &원본[끝..]);
    같아야_한다(&바이트(&root), &기대, "A3 ②");
    assert!(화면.contains("agent"), "사용자가 바꾼 키를 남기면서 말하지 않았다:\n{화면}");
}

/// **A3 ③** — pal 훅과 같은 사건 배열(`Stop`)에 사용자가 자기 훅 묶음을 더했다.
/// 사용자 묶음이 그 배열에 남는다.
#[test]
fn a3_3_사용자가_같은_사건_배열에_묶음을_더한_방() {
    let (root, 원본) = a3_설치한_방("a3-3");
    let 설치된 = String::from_utf8(바이트(&root)).expect("UTF-8");
    std::fs::write(root.join(설정), stop_배열_끝에_묶음을_넣는다(&설치된, "설치된 파일")).expect("쓰기");

    성공(&root, &["uninstall"]);
    같아야_한다(&바이트(&root), &stop_배열_끝에_묶음을_넣는다(&원본, "원본"), "A3 ③");
}

// ─────────────────────────────────────────────────────────────────────────────
// A4 — 착수 규칙(재직렬화)으로 설치된 방 넷
// ─────────────────────────────────────────────────────────────────────────────

const 옛_설치: &str = "a4-old-install";

/// 착수 커밋 빌드가 설치한 상태를 세운다 — 매니페스트 · 설정 · 블록 · 선언은 **그 빌드가 쓴 바이트**다.
///
/// 우리가 놓은 리소스 파일(에이전트 정의 · 스킬 …)은 설정과 무관해 fixture 로 싣지 않고, 옆 방에서 지금
/// 빌드로 설치해 그 파일을 옮겨 온다. 내용이 달라져도 uninstall 은 그 파일을 지우고 경고할 뿐이다(B6).
fn 옛_설치_방(tag: &str, 설정을_커밋: bool) -> PathBuf {
    let root = 방(tag);
    std::fs::write(root.join(설정), 픽스처_바이트(&format!("{옛_설치}/settings.before.json"))).expect("쓰기");
    if 설정을_커밋 {
        커밋(&root, &["README.md", 설정], "설치 전");
    } else {
        커밋(&root, &["README.md"], "설치 전");
    }

    let 옆 = 방(&format!("{tag}-payload"));
    성공(&옆, &["install"]);
    let 매니페스트 = 픽스처_바이트(&format!("{옛_설치}/manifest.json"));
    let m = 값(&매니페스트);
    assert!(m["settings"].get("edits").is_none(), "옛 매니페스트에 편집 기록이 있다 — 착수 형식이 아니다");
    for f in m["files"].as_array().expect("files") {
        let rel = f["path"].as_str().expect("path");
        let 목적 = root.join(rel);
        std::fs::create_dir_all(목적.parent().expect("부모")).expect("디렉터리");
        std::fs::copy(옆.join(rel), &목적).unwrap_or_else(|e| panic!("{rel} 을 못 옮겼다: {e}"));
    }
    for (원천, 목적) in [
        ("manifest.json", ".claude/pal/manifest.json"),
        ("settings.installed.json", 설정),
        ("CLAUDE.md.txt", "CLAUDE.md"),
        ("gitignore.txt", ".gitignore"),
        ("manifest.toml.txt", ".palimpsest/manifest.toml"),
    ] {
        let p = root.join(목적);
        std::fs::create_dir_all(p.parent().expect("부모")).expect("디렉터리");
        std::fs::write(&p, 픽스처_바이트(&format!("{옛_설치}/{원천}"))).expect("쓰기");
    }
    // ★ 생김 단언 — 옛 설치의 우리 몫이 실제로 파일에 있다.
    우리_몫이_생겼다(&값(&바이트(&root)), tag);
    root
}

fn 설치_전_값() -> Value {
    값(&픽스처_바이트(&format!("{옛_설치}/settings.before.json")))
}

/// **A4 ①** — 설정이 설치 전에 커밋돼 있고 편집 없음 → uninstall 뒤 **`HEAD` 바이트와 같다**.
#[test]
fn a4_1_커밋된_설정은_head_바이트로_돌아온다() {
    let root = 옛_설치_방("a4-1", true);
    let 화면 = 성공(&root, &["uninstall"]);
    let head = 픽스처_바이트(&format!("{옛_설치}/settings.before.json"));
    assert!(
        바이트(&root) == head,
        "A4 ①: HEAD 바이트가 아니다\n  HEAD: {}\n  뒤:   {}",
        보인다(&head),
        보인다(&바이트(&root))
    );
    assert!(!화면.contains(값만), "HEAD 로 되돌렸는데 값만 되돌렸다고 말했다:\n{화면}");
}

/// **A4 ②** — ①에서 설치 뒤 사용자가 키를 더했다 → `HEAD` 로 되쓰지 않고 값이 「원본 + 사용자 키」이며
/// 「값만 되돌렸다」 출력.
#[test]
fn a4_2_사용자가_키를_더했으면_값만_되돌린다() {
    let root = 옛_설치_방("a4-2", true);
    let 설치된 = String::from_utf8(바이트(&root)).expect("UTF-8");
    assert!(설치된.starts_with("{\n"), "재직렬화된 파일의 머리가 아니다: {설치된:?}");
    std::fs::write(root.join(설정), format!("{{\n  \"mine\": 1,\n{}", &설치된[2..])).expect("쓰기");

    let 화면 = 성공(&root, &["uninstall"]);
    let mut 기대 = 설치_전_값();
    기대["mine"] = serde_json::json!(1);
    assert_eq!(값(&바이트(&root)), 기대, "A4 ②: 값이 「원본 + 사용자 키」가 아니다");
    assert!(화면.contains(값만), "A4 ②: 「{값만}」을 출력하지 않았다:\n{화면}");
}

/// **A4 ③** — 설치 뒤 pal 키가 든 채로 커밋된 방 → `HEAD` 로 되쓰지 않고 pal 키가 없어지며 「값만 되돌렸다」 출력.
#[test]
fn a4_3_pal_키가_든_채로_커밋됐으면_값만_되돌린다() {
    let root = 옛_설치_방("a4-3", true);
    커밋(&root, &[설정], "설치 뒤 설정을 커밋");

    let 화면 = 성공(&root, &["uninstall"]);
    let 뒤 = 값(&바이트(&root));
    assert!(뒤.get("agent").is_none() && 뒤.get("hooks").is_none(), "A4 ③: pal 키가 남았다: {뒤}");
    assert_eq!(뒤, 설치_전_값(), "A4 ③: 값이 설치 전과 다르다");
    assert!(화면.contains(값만), "A4 ③: 「{값만}」을 출력하지 않았다:\n{화면}");
}

/// **A4 ④** — 추적 안 된 방 → 값이 원본과 같고 「값만 되돌렸다」 출력.
#[test]
fn a4_4_추적_안_된_설정은_값만_되돌린다() {
    let root = 옛_설치_방("a4-4", false);
    let 화면 = 성공(&root, &["uninstall"]);
    assert_eq!(값(&바이트(&root)), 설치_전_값(), "A4 ④: 값이 원본과 다르다");
    assert!(화면.contains(값만), "A4 ④: 「{값만}」을 출력하지 않았다:\n{화면}");
}
