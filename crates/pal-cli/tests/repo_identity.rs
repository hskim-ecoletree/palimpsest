//! **저장소 식별자가 디렉터리 이름에 안 묶인다** — `pal install` 이 식별자를 선언으로 적고,
//! 선언이 없는 저장소에서도 `touch` 가 거짓 0 을 사실로 적지 않는다.
//!
//! 회차 `2026-09-14-first-release` 의 `B1`·`B1-a`.
//!
//! # 왜 디렉터리 이름이 문제인가
//!
//! 저장소 식별자는 심볼 좌표의 해시 성분이다(`SymbolId::compute`). 매니페스트가 없으면
//! 식별자가 디렉터리 이름이라, 같은 커밋을 **다른 이름으로 클론**하면 좌표가 통째로 바뀌고
//! 결박이 하나도 안 걸린다 — 그런데 화면은 「아직 없습니다」를 사실로 적었다.
//!
//! # 실물 git 저장소 둘을 쓴다
//!
//! 재는 것이 **디렉터리 이름**과 **클론**이라 API 로는 못 잰다. 방 하나 아래에 이름을 골라
//! 저장소를 세우고 `git clone` 으로 다른 이름을 만든다.

mod common;

use common::{PAL, git, path_앞에};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// `pal` 이 있는 디렉터리 — 설치가 훅 등록 문자열을 `PATH` 에서 찾는 정상 조건.
fn pal_dir() -> PathBuf {
    Path::new(PAL).parent().expect("pal 의 부모").to_path_buf()
}

fn 돌린다(cwd: &Path, args: &[&str]) -> Output {
    Command::new(PAL)
        .args(args)
        .current_dir(cwd)
        .env("PATH", path_앞에(&pal_dir()))
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
    String::from_utf8(out.stdout).expect("UTF-8")
}

/// 저장소를 **이름을 골라** 세울 부모 디렉터리.
fn 방(tag: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("pal-repoid-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("방");
    root
}

fn 커밋한다(root: &Path, 메시지: &str) {
    git(root, &["-c", "user.email=t@example.com", "-c", "user.name=t", "commit", "-qm", 메시지]);
}

/// 코드만 있는 저장소 — **디렉터리 이름이 `이름` 이다.**
///
/// 이름은 ASCII 로 둔다. 한글 디렉터리 이름은 파일시스템마다 정규화가 달라 식별자 비교가
/// 이 시험이 재려는 것과 무관하게 흔들린다.
fn 코드_저장소(방: &Path, 이름: &str) -> PathBuf {
    let root = 방.join(이름);
    std::fs::create_dir_all(root.join("src")).expect("src");
    std::fs::write(
        root.join("src/core.ts"),
        "export function helper() { return 0; }\nexport function target() { return helper(); }\n",
    )
    .expect("core.ts");
    git(&root, &["init", "-q", "."]);
    git(&root, &["add", "-A"]);
    커밋한다(&root, "첫 커밋");
    root
}

/// 결박 하나를 걸고 **커밋되는 정본**(`bindings.jsonl`)으로 내보낸다. 커밋은 부르는 쪽이 한다.
fn 결박을_건다(root: &Path) {
    성공(root, &["bind", "target", "--note", "target 은 helper 를 한 번만 부른다"]);
    std::fs::create_dir_all(root.join(".palimpsest/intent")).expect("intent 디렉터리");
    성공(root, &["intent", "export", "--out", ".palimpsest/intent/bindings.jsonl"]);
}

/// 결박이 선 저장소 — **정본만 커밋한다**(파생 저장소·캐시는 커밋하지 않는다).
fn 결박_저장소(방: &Path, 이름: &str) -> PathBuf {
    let root = 코드_저장소(방, 이름);
    결박을_건다(&root);
    git(&root, &["add", ".palimpsest/intent/bindings.jsonl"]);
    커밋한다(&root, "결박 정본");
    root
}

fn 클론한다(방: &Path, 원래: &Path, 이름: &str) -> PathBuf {
    let 클론 = 방.join(이름);
    git(
        방,
        &["clone", "-q", &원래.display().to_string(), &클론.display().to_string()],
    );
    클론
}

/// 사람 화면의 `■ 이 좌표에 걸린 것 (N)` 의 N.
fn 걸린_수(화면: &str) -> usize {
    화면.lines()
        .find_map(|l| l.strip_prefix("■ 이 좌표에 걸린 것 (")?.strip_suffix(')')?.parse().ok())
        .unwrap_or_else(|| panic!("「걸린 것」 구역이 없다:\n{화면}"))
}

/// 제목 줄부터 다음 `■` 전까지.
fn 구역(화면: &str, 머리: &str) -> String {
    let mut out = Vec::new();
    let mut 안 = false;
    for 줄 in 화면.lines() {
        if 줄.starts_with(머리) {
            안 = true;
        } else if 줄.starts_with('■') {
            안 = false;
        }
        if 안 {
            out.push(줄);
        }
    }
    out.join("\n")
}

/// 매니페스트가 선언한 `id` 들 — **파서를 새로 들지 않고 줄에서 뜬다.**
fn 선언된_식별자(root: &Path) -> Vec<String> {
    let text = std::fs::read_to_string(root.join(".palimpsest/manifest.toml"))
        .unwrap_or_else(|e| panic!("{} 에 매니페스트가 없다: {e}", root.display()));
    text.lines()
        .filter_map(|l| {
            let 값 = l.trim().strip_prefix("id")?.trim_start().strip_prefix('=')?.trim();
            Some(값.trim_matches('"').to_owned())
        })
        .collect()
}

fn 상태(root: &Path) -> String {
    let out = Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(root)
        .output()
        .expect("git status");
    String::from_utf8(out.stdout).expect("UTF-8")
}

fn json(cwd: &Path, args: &[&str]) -> serde_json::Value {
    let mut all = args.to_vec();
    all.push("--json");
    serde_json::from_str(&성공(cwd, &all)).expect("JSON")
}

// ─────────────────────────────────────────────────────────────────────────────
// B1
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn b1_install_이_식별자를_선언하고_다른_이름의_클론이_같은_수를_보인다() {
    let 방 = 방("b1");
    let 원래 = 코드_저장소(&방, "origin-name");
    결박을_건다(&원래);

    성공(&원래, &["install"]);
    assert_eq!(
        선언된_식별자(&원래),
        vec!["origin-name".to_owned()],
        "`pal install` 이 `[[repo]] id` 를 결박이 선 식별자로 안 적었다"
    );
    let 원래_수 = 걸린_수(&성공(&원래, &["touch", "target"]));
    assert!(원래_수 >= 1, "하한 — 원래 저장소에서 결박이 안 떴다. 이 시험은 아무것도 안 잰다");

    git(&원래, &["add", "-A"]);
    커밋한다(&원래, "설치와 결박 정본");

    let 클론 = 클론한다(&방, &원래, "other-name");
    성공(&클론, &["intent", "import", ".palimpsest/intent/bindings.jsonl"]);
    let 화면 = 성공(&클론, &["touch", "target"]);
    assert_eq!(
        걸린_수(&화면),
        원래_수,
        "다른 디렉터리 이름으로 클론했더니 걸린 것 수가 달라졌다:\n{화면}"
    );

    let _ = std::fs::remove_dir_all(&방);
}

// ─────────────────────────────────────────────────────────────────────────────
// B1-a — 음성 대조 네 갈래
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn b1a_1_선언이_없는_저장소를_다른_이름으로_클론해도_아직_없습니다가_안_나온다() {
    let 방 = 방("b1a-1");
    let 원래 = 결박_저장소(&방, "origin-name");
    assert!(
        !원래.join(".palimpsest/manifest.toml").exists(),
        "전제 — 이 갈래는 매니페스트가 없는 저장소다"
    );

    let 클론 = 클론한다(&방, &원래, "other-name");
    성공(&클론, &["intent", "import", ".palimpsest/intent/bindings.jsonl"]);

    let 화면 = 성공(&클론, &["touch", "target"]);
    assert_eq!(걸린_수(&화면), 0, "전제 — 식별자가 달라 이 좌표에는 안 걸린다:\n{화면}");
    let 걸린 = 구역(&화면, "■ 이 좌표에 걸린 것");
    assert!(
        !걸린.contains("아직 없습니다"),
        "다른 식별자에 걸린 결박이 있는데 「아직 없습니다」를 사실로 적었다:\n{걸린}"
    );

    let v = json(&클론, &["touch", "target"]);
    let 다른 = &v["answer"]["store"]["other_repo"];
    let n = 다른["bindings"].as_u64().unwrap_or_else(|| panic!("--json 에 다른 식별자 결박 수가 없다: {v}"));
    assert!(n >= 1, "다른 식별자에 걸린 결박을 0 으로 보았다: {다른}");
    assert_eq!(다른["current"], "other-name", "지금 식별자가 틀렸다: {다른}");
    assert_eq!(다른["repos"], serde_json::json!(["origin-name"]), "다른 식별자 목록이 틀렸다: {다른}");
    assert!(
        걸린.contains("다른 저장소 식별자") && 걸린.contains(&format!("{n}건")) && 걸린.contains("`origin-name`"),
        "사람 화면에 그 사실과 수가 없다:\n{걸린}"
    );

    // **짝** — 식별자가 같은 원래 저장소에서는 그 말이 안 나온다. 늘 나오면 아무것도 안 가른다.
    let 원래_v = json(&원래, &["touch", "target"]);
    assert_eq!(원래_v["answer"]["store"]["other_repo"]["bindings"], 0, "원래 저장소에서도 다른 식별자로 보았다");
    assert!(!성공(&원래, &["touch", "target"]).contains("다른 저장소 식별자"));

    let _ = std::fs::remove_dir_all(&방);
}

#[test]
fn b1a_2_매니페스트가_이미_있으면_install_이_바이트로_안_바꾼다() {
    let 방 = 방("b1a-2");
    let root = 코드_저장소(&방, "declared");
    std::fs::create_dir_all(root.join(".palimpsest")).expect(".palimpsest");
    let 자리 = root.join(".palimpsest/manifest.toml");
    std::fs::write(&자리, "# 사람이 쓴 선언\r\n[[repo]]\r\nid   = \"my-own-id\"\r\npath = \".\"\r\n")
        .expect("선언");
    let 전 = std::fs::read(&자리).expect("읽기");

    성공(&root, &["install"]);
    assert_eq!(std::fs::read(&자리).expect("읽기"), 전, "install 이 이미 있는 매니페스트를 바꿨다");
    성공(&root, &["uninstall"]);
    assert_eq!(std::fs::read(&자리).expect("읽기"), 전, "uninstall 이 사람의 선언을 건드렸다");

    let _ = std::fs::remove_dir_all(&방);
}

#[test]
fn b1a_3_결박이_palimpsest_로_선_저장소를_pal_에서_install_하면_id_가_palimpsest_다() {
    let 방 = 방("b1a-3");
    let 원래 = 결박_저장소(&방, "palimpsest");
    let 원래_수 = 걸린_수(&성공(&원래, &["touch", "target"]));
    assert!(원래_수 >= 1, "하한 — 원래 저장소에서 결박이 안 떴다");

    let 클론 = 클론한다(&방, &원래, "pal");
    성공(&클론, &["install"]);
    assert_eq!(
        선언된_식별자(&클론),
        vec!["palimpsest".to_owned()],
        "디렉터리 이름(`pal`)을 선언으로 굳혔다 — 결박이 선 식별자는 `palimpsest` 다"
    );

    // 그 선언이 실제로 좌표를 되돌린다.
    성공(&클론, &["intent", "import", ".palimpsest/intent/bindings.jsonl"]);
    assert_eq!(걸린_수(&성공(&클론, &["touch", "target"])), 원래_수);

    let _ = std::fs::remove_dir_all(&방);
}

#[test]
fn b1a_4_커밋_전이면_uninstall_이_되돌리고_커밋_뒤면_매니페스트가_남는다() {
    let 방 = 방("b1a-4");
    let root = 코드_저장소(&방, "roundtrip");
    let 자리 = root.join(".palimpsest/manifest.toml");
    let 전 = 상태(&root);
    assert!(전.is_empty(), "전제 — 깨끗한 저장소여야 한다: {전}");

    // ── 커밋 전 ──
    성공(&root, &["install"]);
    assert!(자리.exists(), "전제 — install 이 매니페스트를 만들어야 이 대조가 무언가를 잰다");
    assert_eq!(선언된_식별자(&root), vec!["roundtrip".to_owned()], "결박이 없으면 디렉터리 이름이다");
    성공(&root, &["uninstall"]);
    assert_eq!(상태(&root), 전, "커밋 전 install → uninstall 이 흔적을 남겼다");

    // ── 커밋 뒤 ──
    성공(&root, &["install"]);
    let 바이트 = std::fs::read(&자리).expect("매니페스트");
    git(&root, &["add", "-A"]);
    커밋한다(&root, "설치");
    성공(&root, &["uninstall"]);
    assert_eq!(
        std::fs::read(&자리).ok(),
        Some(바이트),
        "커밋된 매니페스트를 uninstall 이 지우거나 바꿨다 — 그것은 이제 사용자의 선언이다"
    );

    let _ = std::fs::remove_dir_all(&방);
}
