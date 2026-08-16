//! **놓고 갱신하고 걷어낸다** — `[f24]` ①②③④⑤⑥⑥-b.
//!
//! # 왜 전부 실물 파일시스템인가
//!
//! 여기서 재는 것은 전부 **바이트 위의 사실**이다. *"이 함수가 저 함수를 안 부른다"*
//! 로는 모드가 소실됐는지·하드링크가 끊겼는지·NUL 바이트에서 사용자 줄이 잘렸는지
//! 알 수 없다. **떠서 대야** 안다.
//!
//! # ⚠ 게이트 ① 의 예외 목록이 하나가 아니다 — 여기서 재는 것을 밝혀 둔다
//!
//! 게이트 ① 은 *"예외는 `settings.json` 하나다"* 라고 적었다. 그런데 같은 회차의
//! 설치 산출물 목록이 **`CLAUDE.md` 와 `.gitignore` 도 병합 대상**으로 적는다
//! (`@` 임포트 한 줄 · 파생 경로 등재). 셋은 성질이 다르므로 여기서 갈라 잰다:
//!
//! | 대상 | 재는 문장 |
//! |---|---|
//! | 그 밖의 모든 기존 경로 | **sha256 이 하나도 안 바뀐다** (① 그대로) |
//! | `settings.json` | 설치 전의 **모든 키·값이 그대로** (① 의 부분집합 검사) |
//! | `CLAUDE.md` · `.gitignore` | 설치 전 바이트가 **접두사로 그대로 남는다**, 그리고 **왕복하면 바이트 동일** |
//!
//! 셋째 줄은 ① 보다 **약하지 않다** — 사용자 바이트가 한 개도 안 사라지는 것을
//! 접두사 검사와 왕복 검사가 함께 잡는다.

mod common;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use common::{PAL, git, path_앞에, 상대_경로, 해시};

/// `pal` 이 있는 디렉터리 — 설치 검사 4(`PATH` 에 `pal` 이 있는가)의 정상 조건.
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
    String::from_utf8_lossy(&out.stdout).into_owned()
}

fn 실패(cwd: &Path, args: &[&str]) -> String {
    let out = 돌린다(cwd, args);
    assert!(
        !out.status.success(),
        "pal {args:?} 가 성공했다 — 실패해야 한다\nstdout: {}",
        String::from_utf8_lossy(&out.stdout)
    );
    String::from_utf8_lossy(&out.stderr).into_owned()
}

// ─────────────────────────────────────────────────────────────────────────────
// fixture
// ─────────────────────────────────────────────────────────────────────────────

fn 방(tag: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("pal-f24-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("방");
    root
}

/// (a) 빈 프로젝트 — git 저장소이되 우리 것이 없다.
fn 빈_프로젝트(tag: &str) -> PathBuf {
    let root = 방(tag);
    std::fs::write(root.join("README.md"), "hello\n").expect("README");
    git(&root, &["init", "-q", "."]);
    git(&root, &["add", "-A"]);
    git(&root, &["-c", "user.email=t@e", "-c", "user.name=t", "commit", "-qm", "첫"]);
    root
}

/// (b) 사용자 키가 든 `settings.json` · 자기 `CLAUDE.md` · 자기 `.gitignore` 가 있다.
fn 살고_있는_프로젝트(tag: &str) -> PathBuf {
    let root = 빈_프로젝트(tag);
    std::fs::create_dir_all(root.join(".claude")).expect(".claude");
    std::fs::write(
        root.join(".claude/settings.json"),
        "{\n  \"env\": {\"A\": \"1\"},\n  \"permissions\": {\"allow\": [\"Bash(ls:*)\"]}\n}\n",
    )
    .expect("settings");
    std::fs::write(root.join("CLAUDE.md"), "# 내 규칙\n지키자\n").expect("CLAUDE.md");
    std::fs::write(root.join(".gitignore"), "node_modules/\n").expect(".gitignore");
    root
}

/// 트리 전체의 `(상대 경로 → sha256)`. **`.git/` 은 뺀다** — git 이 자기 일로 바꾼다.
fn 스냅샷(root: &Path) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    훑기(root, root, &mut out);
    out
}

fn 훑기(root: &Path, dir: &Path, out: &mut BTreeMap<String, String>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.file_name().is_some_and(|n| n == ".git") {
            continue;
        }
        if path.is_dir() {
            훑기(root, &path, out);
        } else {
            let bytes = std::fs::read(&path).unwrap_or_default();
            out.insert(상대_경로(root, &path), format!("{:x}-{}", bytes.len(), 합(&bytes)));
        }
    }
}

/// 시험 안의 값 — **바이트가 같은지만** 가르면 되므로 길이와 합으로 충분하다.
fn 합(bytes: &[u8]) -> u64 {
    bytes.iter().fold(1_469_598_103_934_665_603_u64, |h, b| {
        (h ^ u64::from(*b)).wrapping_mul(1_099_511_628_211)
    })
}

/// 병합 대상 셋 — ① 의 예외로 갈라 재는 자리.
const 병합_대상: &[&str] = &[".claude/settings.json", "CLAUDE.md", ".gitignore"];

fn 값(path: &Path) -> serde_json::Value {
    serde_json::from_slice(&std::fs::read(path).expect("읽기")).expect("JSON")
}

// ─────────────────────────────────────────────────────────────────────────────
// ① 설치는 대상에 없는 것만 쓴다
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn 설치는_기존_파일을_안_밟는다() {
    for (tag, root) in [("빈", 빈_프로젝트("a-빈")), ("살고있는", 살고_있는_프로젝트("a-살"))] {
        let 전 = 스냅샷(&root);
        let 원본: BTreeMap<String, Vec<u8>> = 병합_대상
            .iter()
            .filter(|p| root.join(p).exists())
            .map(|p| ((*p).to_owned(), std::fs::read(root.join(p)).expect("읽기")))
            .collect();

        성공(&root, &["install"]);
        let 후 = 스냅샷(&root);

        for (path, sha) in &전 {
            if 병합_대상.contains(&path.as_str()) {
                continue;
            }
            assert_eq!(후.get(path), Some(sha), "{tag}: 기존 파일이 밟혔다 — {path}");
        }
        // **사용자 바이트가 접두사로 그대로 남는다** — 블록은 뒤에 붙는다.
        for (path, bytes) in &원본 {
            // `settings.json` 은 병합이라 접두사가 안 남는다 — 그쪽은 값 단위로 잰다.
            if path == ".claude/settings.json" {
                continue;
            }
            let 지금 = std::fs::read(root.join(path)).expect("읽기");
            assert!(지금.starts_with(bytes), "{tag}: 사용자 바이트가 사라졌다 — {path}");
        }
        assert!(후.len() > 전.len(), "{tag}: 아무것도 안 생겼다");
    }
}

/// ① 의 부분집합 검사 — **설치 전의 모든 키·값이 설치 후에도 그대로.**
#[test]
fn 설치는_설정의_키와_값을_하나도_안_잃는다() {
    let root = 살고_있는_프로젝트("a-키");
    let path = root.join(".claude/settings.json");
    let 전 = 값(&path);

    성공(&root, &["install"]);

    let 후 = 값(&path);
    for (key, value) in 전.as_object().expect("객체") {
        assert_eq!(후.get(key), Some(value), "키 `{key}` 가 사라졌거나 값이 달라졌다");
    }
    assert_eq!(후.get("agent"), Some(&serde_json::json!("pal-orchestrator")));
}

/// (c) 이미 깔린 곳 — **두 번째 설치가 첫 번째와 같은 상태를 낸다.**
#[test]
fn 두_번째_설치가_같은_상태를_낸다() {
    let root = 살고_있는_프로젝트("a-멱등");
    성공(&root, &["install"]);
    let 첫째 = 스냅샷(&root);
    성공(&root, &["install"]);
    assert_eq!(스냅샷(&root), 첫째, "두 번째 설치가 상태를 바꿨다");
}

// ─────────────────────────────────────────────────────────────────────────────
// ② 파싱 실패 시 아무것도 안 쓴다
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn 깨진_설정이면_한_바이트도_안_쓴다() {
    for (tag, 본문) in
        [("끝중괄호누락", "{\n  \"env\": {\"A\": \"1\"}\n"), ("후행쉼표", "{\"a\": 1,}"), ("빈파일", "")]
    {
        let root = 빈_프로젝트(&format!("b-{tag}"));
        std::fs::create_dir_all(root.join(".claude")).expect(".claude");
        std::fs::write(root.join(".claude/settings.json"), 본문).expect("settings");

        let 전 = 스냅샷(&root);
        let stderr = 실패(&root, &["install"]);

        assert_eq!(스냅샷(&root), 전, "{tag}: 부분 설치가 남았다");
        assert!(stderr.contains("settings.json"), "{tag}: 어느 파일인지 안 적었다 — {stderr}");
        // **어느 줄이 왜** — 게이트 ② 가 표준오류에 요구하는 것.
        assert!(
            stderr.contains("settings.json:"),
            "{tag}: 줄/칸을 안 적었다 — {stderr}"
        );
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// ③ 매니페스트 — 목록 · sha256 · 버전
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn 매니페스트가_실물과_양방향으로_맞는다() {
    let root = 살고_있는_프로젝트("c");
    성공(&root, &["install"]);

    let m = 값(&root.join(".claude/pal/manifest.json"));
    let 적힌: BTreeMap<String, String> = m["files"]
        .as_array()
        .expect("files")
        .iter()
        .map(|f| (f["path"].as_str().expect("path").to_owned(), f["sha256"].as_str().expect("sha").to_owned()))
        .collect();

    // **트리를 훑어서 집합을 뜬다 — 이름으로 세지 않는다.**
    let mut 실물 = BTreeMap::new();
    for dir in m["roots"]["dirs"].as_array().expect("dirs") {
        훑어_해시(&root, &root.join(dir.as_str().expect("dir")), &mut 실물);
    }
    for file in m["roots"]["files"].as_array().expect("files 뿌리") {
        let rel = file.as_str().expect("파일 뿌리");
        if root.join(rel).is_file() {
            실물.insert(rel.to_owned(), 해시(&root.join(rel)));
        }
    }
    실물.remove(m["manifest_path"].as_str().expect("manifest_path"));

    let 없는것: Vec<_> = 적힌.keys().filter(|k| !실물.contains_key(*k)).collect();
    let 안적힌것: Vec<_> = 실물.keys().filter(|k| !적힌.contains_key(*k)).collect();
    assert!(없는것.is_empty(), "적혔는데 없다: {없는것:?}");
    assert!(안적힌것.is_empty(), "생겼는데 안 적혔다: {안적힌것:?}");
    assert_eq!(적힌, 실물, "sha256 이 실물과 다르다");
    assert!(!적힌.is_empty(), "매니페스트가 비었다 — 이 시험은 아무것도 안 재고 있다");

    // **버전 문자열 = `pal --version` 의 출력.**
    let version = 성공(&root, &["--version"]);
    let 적힌_버전 = m["pal_version"].as_str().expect("pal_version");
    assert_eq!(version.trim(), format!("pal {적힌_버전}"));
}

/// ★ **이름으로 세면 다음에 생기는 파일이 빠진다** — 훑어서 걸리는지 본다.
#[test]
fn 나중에_생긴_파일이_대조에_걸린다() {
    let root = 살고_있는_프로젝트("c-새것");
    성공(&root, &["install"]);
    std::fs::write(root.join(".claude/commands/pal/나중것.md"), "새로 생겼다\n").expect("새 파일");

    let out = 돌린다(&root, &["doctor", "--install", "--json"]);
    let checks: serde_json::Value =
        serde_json::from_slice(&out.stdout).expect("JSON");
    let 매니페스트_검사 = &checks.as_array().expect("배열")[1];
    assert_eq!(매니페스트_검사["outcome"], "failed", "안 적힌 파일이 안 걸렸다: {매니페스트_검사}");
    assert!(매니페스트_검사["detail"].as_str().expect("detail").contains("나중것"));
}

fn 훑어_해시(root: &Path, dir: &Path, out: &mut BTreeMap<String, String>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            훑어_해시(root, &path, out);
        } else {
            out.insert(상대_경로(root, &path), 해시(&path));
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// ④ 갱신 — 사람이 고친 것은 밟지 않는다. **그리고 말한다**
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn 갱신은_고친_것을_밟지_않고_말한다() {
    let root = 살고_있는_프로젝트("d");
    성공(&root, &["install"]);

    let 고친것 = root.join(".claude/commands/pal/touch.md");
    let 안고친것 = root.join(".claude/pal/INSTRUCTIONS.md");
    let 사람의_내용 = "# 내가 고쳤다\n";
    std::fs::write(&고친것, 사람의_내용).expect("사람의 수정");
    let 안고친것_원본 = std::fs::read(&안고친것).expect("읽기");

    // **모집단 조건** — 안 고친 것 ≥ 1 · 고친 것 ≥ 1. 둘 중 하나라도 0 이면 대조 불가.
    낡게_만든다(&root);

    let report = 성공(&root, &["update"]);

    // ① 고친 것은 그대로다.
    assert_eq!(std::fs::read_to_string(&고친것).expect("읽기"), 사람의_내용, "사용자 수정이 밟혔다");
    // ② 보고에 그 경로가 「사용자 수정 — 건너뜀」으로 나온다.
    assert!(
        report.contains("사용자 수정 — 건너뜀") && report.contains("touch.md"),
        "밟지 않았지만 말하지 않았다:\n{report}"
    );
    // ③ 안 고친 것은 교체됐다 — 그리고 매니페스트 sha 가 실물과 같다.
    assert_eq!(std::fs::read(&안고친것).expect("읽기"), 안고친것_원본);

    // ④ **차이는 sha 가 아니라 종류가 진다.**
    //
    // ⚠ 옛 회차는 여기서 *"매니페스트의 sha 가 실물과 **다르다**"* 를 요구했다. 그것이
    // 곧 `doctor` 를 속이는 형태였고 — 그 차이를 무마하려고 넣은 `Origin::UserModified`
    // 가 **그 경로의 내용 검사를 통째로 껐다**(`사용자_수정_뒤에도_내용을_계속_본다`).
    // 지금은 sha 가 **그 시점의 사람 내용**이고, *"우리 것이 아니다"* 는 종류가 진다.
    let 고친것_항목 = 항목(&root, ".claude/commands/pal/touch.md");
    assert_eq!(고친것_항목["sha256"], serde_json::json!(해시(&고친것)), "그 시점의 sha 를 안 적었다");
    assert_eq!(고친것_항목["origin"], serde_json::json!("user_modified"), "종류를 안 적었다");

    let 안고친것_항목 = 항목(&root, ".claude/pal/INSTRUCTIONS.md");
    assert_eq!(안고친것_항목["sha256"], serde_json::json!(해시(&안고친것)));
    assert_eq!(안고친것_항목["origin"], serde_json::json!("ours"));
}

/// ★ **정상적인 `update` 뒤에 `doctor` 가 빨개지지 않는다.**
///
/// `update` 는 사용자가 고친 파일을 안 밟고 매니페스트에 **옛 sha 를 그대로 둔다**(④ 가
/// 요구하는 것이다). 그런데 그 다음 `doctor` 검사 2 가 그 차이를 **고장**으로 읽으면,
/// 사용자는 정상 경로를 따랐는데 진단이 빨간 채로 굳고 **지울 방법이 없다.**
///
/// ADR-0005 — *"부재는 종류를 싣는다. 상태를 늘리는 대신 이유를 값으로 둔다."*
/// 여기서는 **다름의 종류**를 매니페스트가 싣고, `doctor` 가 그것으로 고장과 사용자
/// 수정을 가른다. 검사의 수(칸)는 안 늘린다.
#[test]
fn 갱신_뒤에_진단이_빨개지지_않는다() {
    let root = 살고_있는_프로젝트("d-진단");
    성공(&root, &["install"]);

    let 고친것 = root.join(".claude/commands/pal/touch.md");
    std::fs::write(&고친것, "# 내가 고쳤다\n").expect("사람의 수정");
    낡게_만든다(&root);
    let report = 성공(&root, &["update"]);
    assert!(report.contains("사용자 수정 — 건너뜀"), "밟지 않았지만 말하지 않았다:\n{report}");

    // ① 진단이 초록이고, **무엇 때문에 다른지를 말한다.**
    let c = 검사들(&root, None);
    assert_eq!(결말(&c, 2), "ok", "정상 경로를 따랐는데 진단이 빨갛다: {}", c[1]);
    let detail = c[1]["detail"].as_str().expect("detail");
    assert!(detail.contains("사용자 수정"), "왜 다른지를 안 말했다: {detail}");

    // ② 그런데 **진짜 고장은 여전히 빨갛다** — 이 줄이 없으면 위가 공짜로 선다.
    std::fs::write(root.join(".claude/pal/INSTRUCTIONS.md"), "망가뜨렸다\n").expect("고장");
    let c = 검사들(&root, None);
    assert_eq!(결말(&c, 2), "failed", "고장을 못 봤다: {}", c[1]);
    assert!(c[1]["detail"].as_str().expect("detail").contains("INSTRUCTIONS.md"));
}

/// ★ **「사용자가 고쳤다」와 「내용을 모른다」는 다르다.**
///
/// 앞 회차가 `갱신_뒤에_진단이_빨개지지_않는다` 를 세우려고 넣은 `Origin::UserModified`
/// 가 그 경로의 **sha 대조를 통째로 껐다.** 관측: 그 파일을 통째로 다른 내용으로
/// 바꿔도, 0바이트로 비워도 `doctor` 는 **초록**이었다 — 없어지면 잡히는데 **바뀌면
/// 안 잡혔다.**
///
/// 그 파일들은 `.claude/commands/pal/*.md` — **에이전트에게 그대로 먹이는 지시문**이다.
/// 「우리 것과 다르다」를 고장으로 안 세는 것과 **「무슨 내용인지 안 본다」는 다르다.**
#[test]
fn 사용자_수정_뒤에도_내용을_계속_본다() {
    let root = 살고_있는_프로젝트("d-계속본다");
    성공(&root, &["install"]);

    let 고친것 = root.join(".claude/commands/pal/touch.md");
    let 사람의_내용 = "# 내가 고쳤다\n";
    std::fs::write(&고친것, 사람의_내용).expect("사람의 수정");
    낡게_만든다(&root);
    성공(&root, &["update"]);

    // ① **고친 시점의 sha 가 기록됐다** — 종류와 함께.
    let e = 항목(&root, ".claude/commands/pal/touch.md");
    assert_eq!(e["sha256"], serde_json::json!(해시(&고친것)), "그 시점의 sha 를 안 적었다: {e}");
    assert_eq!(e["origin"], serde_json::json!("user_modified"), "종류를 안 적었다: {e}");

    // ② 그리고 진단은 초록이다 — 정상 경로를 따른 사용자가 빨간 화면을 안 본다.
    let c = 검사들(&root, None);
    assert_eq!(결말(&c, 2), "ok", "정상 경로를 따랐는데 진단이 빨갛다: {}", c[1]);

    // ③ ★ **그 뒤 또 바뀌면 진단이 말한다.** 통째로 갈아끼워도 · 0바이트로 비워도.
    for 나중 in ["# 남이 갈아끼운 지시\n", ""] {
        std::fs::write(&고친것, 나중).expect("또 바뀜");
        let c = 검사들(&root, None);
        assert_eq!(결말(&c, 2), "failed", "사용자 수정 뒤의 변화를 못 봤다: {}", c[1]);
        assert!(
            c[1]["detail"].as_str().expect("detail").contains("touch.md"),
            "어느 파일인지 안 적었다: {}",
            c[1]
        );
    }

    // ④ 그래도 **갱신은 여전히 밟지 않는다** — ④ 를 안 깬다.
    낡게_만든다(&root);
    let report = 성공(&root, &["update"]);
    assert_eq!(std::fs::read_to_string(&고친것).expect("읽기"), "", "사용자 수정이 밟혔다");
    assert!(report.contains("사용자 수정 — 건너뜀"), "밟지 않았지만 말하지 않았다:\n{report}");
}

/// 매니페스트의 파일 항목 하나.
fn 항목(root: &Path, rel: &str) -> serde_json::Value {
    값(&root.join(".claude/pal/manifest.json"))["files"]
        .as_array()
        .expect("files")
        .iter()
        .find(|f| f["path"] == serde_json::json!(rel))
        .unwrap_or_else(|| panic!("{rel} 이 매니페스트에 없다"))
        .clone()
}

/// **버전만으로 「이미 최신」과 「낡음」이 갈리는가**(⑨ 의 뒷문장).
#[test]
fn 이미_최신이면_아무것도_안_한다() {
    let root = 살고_있는_프로젝트("d-최신");
    성공(&root, &["install"]);
    let 전 = 스냅샷(&root);

    let report = 성공(&root, &["update"]);
    assert!(report.contains("이미 최신"), "낡지 않았는데 갱신했다:\n{report}");
    assert_eq!(스냅샷(&root), 전);

    낡게_만든다(&root);
    let report = 성공(&root, &["update"]);
    assert!(report.contains("낡음"), "낡았는데 최신이라고 했다:\n{report}");
}

/// 매니페스트의 버전을 낮춘다 — **버전 하나만 바꾼다.**
fn 낡게_만든다(root: &Path) {
    let path = root.join(".claude/pal/manifest.json");
    let mut m = 값(&path);
    m["pal_version"] = serde_json::json!("0.0.0+옛날");
    std::fs::write(&path, serde_json::to_string_pretty(&m).expect("직렬화")).expect("쓰기");
}

// ─────────────────────────────────────────────────────────────────────────────
// ⑥ 제거 — 매니페스트에 적힌 것만
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn 제거하면_설치_전으로_돌아간다() {
    for (tag, root) in [("빈", 빈_프로젝트("f-빈")), ("살고있는", 살고_있는_프로젝트("f-살"))] {
        let s0 = 스냅샷(&root);
        성공(&root, &["install"]);
        let s1 = 스냅샷(&root);
        let 적힌_경로 = 매니페스트_경로들(&root);
        성공(&root, &["uninstall"]);
        let s2 = 스냅샷(&root);

        // `settings.json` 은 값 단위로 본다 — 직렬화 형태가 우리 것이 된다.
        let mut s0v = s0.clone();
        let mut s2v = s2.clone();
        s0v.remove(".claude/settings.json");
        s2v.remove(".claude/settings.json");
        assert_eq!(s2v, s0v, "{tag}: 제거 후가 설치 전과 다르다");

        // **`S1 − S2` 가 매니페스트 항목 집합의 부분집합이다.**
        for path in s1.keys().filter(|k| !s2.contains_key(*k)) {
            assert!(적힌_경로.contains(path), "{tag}: 매니페스트 밖의 것이 사라졌다 — {path}");
        }
        assert!(!적힌_경로.is_empty(), "{tag}: 매니페스트가 비었다");
    }
}

/// 매니페스트가 소유를 선언한 경로 전부 — 파일 · 블록 · 설정 · 자기 자신.
fn 매니페스트_경로들(root: &Path) -> Vec<String> {
    let m = 값(&root.join(".claude/pal/manifest.json"));
    let mut out = vec![m["manifest_path"].as_str().expect("manifest_path").to_owned()];
    for key in ["files", "blocks"] {
        for e in m[key].as_array().expect(key) {
            out.push(e["path"].as_str().expect("path").to_owned());
        }
    }
    if let Some(s) = m["settings"].as_object() {
        out.push(s["path"].as_str().expect("path").to_owned());
    }
    out
}

/// ★ **왕복 후 바이트 동일** — 디스크에 뜬 사본과 실물을 바이트로 대는 자리.
///
/// ⚠ 옛 회차는 여기를 `Command::new("cmp")` 로 댔다. 그 도구는 유닉스 밖에 없어서
/// **이 시험이 Windows 에서 `NotFound` 로 죽었고**, 그래서 왕복 동일성이 이 플랫폼에서
/// 한 번도 안 재졌다. 바이트 대조에 바깥 프로세스가 필요하지 않다.
///
/// **사본은 그대로 둔다** — 메모리의 `원본` 만 대면 *"설치가 파일을 안 건드렸다"* 는
/// 재지만 *"설치 중에 디스크에 뜬 것이 그대로다"* 는 못 잰다. 사본은 설치가 지나간
/// 뒤에도 디스크에 살아 있는 **바깥 증인**이다.
#[test]
fn 왕복하면_사용자_파일이_바이트로_같다() {
    let root = 살고_있는_프로젝트("f-바이트");
    let 원본: Vec<(PathBuf, Vec<u8>)> = ["CLAUDE.md", ".gitignore"]
        .iter()
        .map(|p| (root.join(p), std::fs::read(root.join(p)).expect("읽기")))
        .collect();
    for (path, bytes) in &원본 {
        std::fs::write(path.with_extension("원본"), bytes).expect("사본");
    }

    성공(&root, &["install"]);
    성공(&root, &["uninstall"]);

    for (path, bytes) in &원본 {
        let 지금 = std::fs::read(path).expect("읽기");
        assert_eq!(&지금, bytes, "{} 가 원본과 다르다", path.display());
        let 사본 = std::fs::read(path.with_extension("원본")).expect("사본 읽기");
        assert_eq!(지금, 사본, "{} 가 디스크의 사본과 갈렸다", path.display());
    }
}

/// ⑥-b — **지울 게 없었으니 성공**은 거짓말이다.
#[test]
fn 리소스를_하나도_못_찾으면_실패한다() {
    let root = 살고_있는_프로젝트("f-없음");
    성공(&root, &["install"]);

    let m = 값(&root.join(".claude/pal/manifest.json"));
    for f in m["files"].as_array().expect("files") {
        std::fs::remove_file(root.join(f["path"].as_str().expect("path"))).expect("지우기");
    }
    let stderr = 실패(&root, &["uninstall"]);
    assert!(stderr.contains("하나도 못 찾았다"), "까닭을 안 적었다: {stderr}");

    // 매니페스트가 아예 없는 자리도 마찬가지다.
    let 빈방 = 빈_프로젝트("f-매니페스트없음");
    let stderr = 실패(&빈방, &["uninstall"]);
    assert!(stderr.contains("설치를 찾지 못했다"), "{stderr}");
}

/// ★ **제거가 사용자 수정 파일을 말없이 지우지 않는다.**
///
/// `update` 는 손으로 고친 파일을 「사용자 수정 — 건너뜀」으로 지키는데 `uninstall` 은
/// 같은 파일을 sha 대조 없이 지웠다. 게이트 ④ 가 세운 *"밟지 않는 것과 말하지 않는
/// 것은 다르다"* 를 **제거 쪽에도** 세운다.
///
/// ⚠ **지우는 것 자체는 그대로다** — ⑥ 이 `S2 == S0` 을 요구하므로 남기면 그것이 반증이다.
/// 여기서 더하는 것은 **말**이다.
#[test]
fn 제거는_사용자_수정을_말하고_지운다() {
    let root = 살고_있는_프로젝트("f-사용자수정");
    let s0 = 스냅샷(&root);
    성공(&root, &["install"]);

    let 고친것 = root.join(".claude/commands/pal/touch.md");
    std::fs::write(&고친것, "# 내가 고쳤다\n").expect("사람의 수정");

    let report = 성공(&root, &["uninstall"]);
    assert!(
        report.lines().any(|l| l.contains("사용자 수정") && l.contains("touch.md")),
        "고친 파일을 말없이 지웠다:\n{report}"
    );
    // **안 고친 것은 그 말이 안 붙는다** — 붙으면 이 시험은 아무것도 안 가른다.
    assert!(
        !report.lines().any(|l| l.contains("사용자 수정") && l.contains("INSTRUCTIONS.md")),
        "안 고친 것에도 같은 말을 붙였다:\n{report}"
    );

    // 그리고 ⑥ 은 그대로 선다 — 제거 후가 설치 전이다.
    let mut s2 = 스냅샷(&root);
    let mut s0v = s0.clone();
    s2.remove(".claude/settings.json");
    s0v.remove(".claude/settings.json");
    assert_eq!(s2, s0v, "제거 후가 설치 전과 다르다");
}

/// ★ **설정 키의 「사용자 수정」도 말한다.**
///
/// 파일에는 `사용자 수정 — 지웠다` 를 붙였는데 `settings.json` 의 키에는 그 대칭이
/// 없었다. 사용자가 `agent` 값을 자기 것으로 바꿔도 키를 통째로 지우고, 우리가 만든
/// 파일이면 **파일까지 지우면서** 화면은 `키 뺌` 한 줄이었다.
///
/// ⚠ **지우는 것 자체는 그대로다** — ⑥ 이 `S2 == S0` 을 요구한다. 더하는 것은 **말**이다.
#[test]
fn 제거가_설정_키의_사용자_수정도_말한다() {
    let root = 빈_프로젝트("f-설정수정");
    성공(&root, &["install"]);

    // 사용자가 **우리가 더한 키의 값**을 자기 것으로 바꿨다.
    let path = root.join(".claude/settings.json");
    let mut v = 값(&path);
    v["agent"] = serde_json::json!("내 오케스트레이터");
    std::fs::write(&path, serde_json::to_string_pretty(&v).expect("직렬화")).expect("쓰기");

    let report = 성공(&root, &["uninstall"]);
    assert!(!path.exists(), "우리가 만든 파일이 안 지워졌다");
    assert!(
        report.contains("사용자 수정") && report.contains("agent"),
        "사용자가 바꾼 값을 지우면서 말하지 않았다:\n{report}"
    );
    assert!(
        report.contains(".claude/settings.json") && report.contains("파일째"),
        "파일까지 지운 것을 말하지 않았다:\n{report}"
    );
}

/// 블록이 **손으로 고쳐졌으면** 아무것도 안 지우고 거부한다.
#[test]
fn 손으로_고친_블록이_있으면_아무것도_안_지운다() {
    let root = 살고_있는_프로젝트("f-훼손");
    성공(&root, &["install"]);
    let 훼손 = std::fs::read_to_string(root.join(".gitignore"))
        .expect("읽기")
        .replace("pal:end", "pal:끝");
    std::fs::write(root.join(".gitignore"), &훼손).expect("훼손");

    let 전 = 스냅샷(&root);
    let stderr = 실패(&root, &["uninstall"]);
    assert!(stderr.contains("손으로 고쳐졌"), "{stderr}");
    assert_eq!(스냅샷(&root), 전, "거부했는데 무언가를 지웠다");
}

// ─────────────────────────────────────────────────────────────────────────────
// `.gitignore` — 실측으로 깨진 것들
// ─────────────────────────────────────────────────────────────────────────────

/// **rc=128 을 rc=1 과 뭉개면 저장소가 아닌 곳에 `.gitignore` 를 만든다.**
#[test]
fn 저장소가_아니면_gitignore_를_안_만든다() {
    let root = 방("g-비저장소");
    std::fs::write(root.join("README.md"), "x\n").expect("README");
    let report = 성공(&root, &["install"]);
    assert!(!root.join(".gitignore").exists(), "worktree 가 아닌데 `.gitignore` 를 만들었다");
    assert!(report.contains("worktree"), "왜 건너뛰었는지 안 적었다:\n{report}");
}

/// ★ **사용자가 `!` 로 되살린 것을 조용히 뒤집지 않는다.**
///
/// ⚠ 텍스트 grep 으로 찾으면 놓친다 — `.git/info/exclude` 에 두어 그것을 못박는다.
#[test]
fn 사용자가_되살린_경로를_안_뒤집는다() {
    let root = 빈_프로젝트("g-되살림");
    // 넓은 규칙은 **전역 제외 파일**에, 되살림은 **`.git/info/exclude`** 에 둔다.
    // `.gitignore` 를 텍스트로 grep 하면 **둘 다 안 보인다** — 그것이 이 시험의 요점이다.
    std::fs::write(root.join("전역무시"), "*.redb\n").expect("전역무시");
    git(&root, &["config", "core.excludesFile", &root.join("전역무시").display().to_string()]);
    std::fs::write(root.join(".git/info/exclude"), "!.palimpsest/index.redb\n").expect("exclude");
    std::fs::write(root.join(".gitignore"), "node_modules/\n").expect(".gitignore");

    let report = 성공(&root, &["install"]);
    assert!(report.contains("되살렸다"), "되살린 것을 못 알아봤다:\n{report}");
    let ignore = std::fs::read_to_string(root.join(".gitignore")).expect("읽기");
    assert!(
        !ignore.contains("/.palimpsest/index.redb"),
        "사용자가 되살린 경로를 다시 무시로 돌렸다:\n{ignore}"
    );
}

/// ★ **디렉터리 경로의 되살림도 안 뒤집는다** — 네 위치 전부에서.
///
/// # 실측이 이 시험의 모양을 정했다 (git 2.50.1)
///
/// `check-ignore` 는 **디렉터리가 디스크에 있을 때만** 디렉터리 형태의 `!` 패턴을
/// 낸다. 없으면 어떤 질의 형태로도 **패턴을 하나도 안 낸다**:
///
/// ```text
/// .gitignore = "!.palimpsest/cache/"     디렉터리 없음
///   check-ignore -v    -- '.palimpsest/cache/'   rc=1  출력 없음
///   check-ignore -v    -- '.palimpsest/cache'    rc=1  출력 없음
///   check-ignore -v -n -- '.palimpsest/cache/'   rc=1  "::\t.palimpsest/cache/"  ← 빈 패턴
///                                        디렉터리 있음
///   check-ignore -v    -- '.palimpsest/cache'    rc=0  "!.palimpsest/cache/"
/// ```
///
/// **그래서 두 상태를 다 잰다.** 있는 쪽은 git 이 답하고, 없는 쪽은 답할 사람이
/// 없어서 소스를 직접 읽어야 한다.
#[test]
fn 되살린_디렉터리_경로도_안_뒤집는다() {
    for 위치 in ["gitignore", "exclude", "전역", "중첩"] {
        for 디스크에 in [false, true] {
            let tag = format!("g-되살림-{위치}-{디스크에}");
            let root = 빈_프로젝트(&tag);
            match 위치 {
                "gitignore" => {
                    std::fs::write(root.join(".gitignore"), "!.palimpsest/cache/\n").expect("쓰기");
                }
                "exclude" => {
                    std::fs::write(root.join(".git/info/exclude"), "!.palimpsest/cache/\n")
                        .expect("쓰기");
                }
                "전역" => {
                    std::fs::write(root.join("전역무시"), "!.palimpsest/cache/\n").expect("쓰기");
                    git(
                        &root,
                        &["config", "core.excludesFile", &root.join("전역무시").display().to_string()],
                    );
                }
                _ => {
                    std::fs::create_dir_all(root.join(".palimpsest")).expect("중첩 디렉터리");
                    std::fs::write(root.join(".palimpsest/.gitignore"), "!cache/\n").expect("쓰기");
                }
            }
            if 디스크에 {
                std::fs::create_dir_all(root.join(".palimpsest/cache")).expect("cache");
                std::fs::write(root.join(".palimpsest/cache/x"), "x\n").expect("x");
            }

            let report = 성공(&root, &["install"]);
            assert!(
                report.contains("되살렸다"),
                "{tag}: 되살린 것을 못 알아봤다:\n{report}"
            );
            // ★ **git 의 답이 안 뒤집혔는가** — 이것이 이 시험의 하중이다.
            let out = Command::new("git")
                .args(["-C", &root.display().to_string()])
                .args(["check-ignore", "-q", "--no-index", "--", ".palimpsest/cache/"])
                .output()
                .expect("git");
            assert_eq!(
                out.status.code(),
                Some(1),
                "{tag}: 사용자가 되살린 디렉터리가 다시 무시로 뒤집혔다"
            );
        }
    }
}

/// **이미 덮여 있으면 더하지 않는다** — `cache/**` 형태가 슬래시 없는 질의를 속인 자리.
#[test]
fn 이미_덮인_경로는_안_더한다() {
    let root = 빈_프로젝트("g-이미덮임");
    std::fs::write(root.join(".gitignore"), ".palimpsest/cache/**\n").expect(".gitignore");
    let report = 성공(&root, &["install"]);
    assert!(report.contains("이미 등재됨"), "덮여 있는데 못 알아봤다:\n{report}");
    let ignore = std::fs::read_to_string(root.join(".gitignore")).expect("읽기");
    assert_eq!(ignore.matches(".palimpsest/cache/").count(), 1, "덮인 규칙을 또 적었다:\n{ignore}");
}

/// ★ **추적 중인 것은 파일 경로에서도 말한다.**
///
/// ignore 규칙은 **이미 추적 중인 파일을 배제하지 못한다** — 그래서 규칙만 더하고
/// 말하지 않으면 사용자는 파생이 빠졌다고 믿는다.
///
/// # 실측 (git 2.50.1) — `ls-files` 와 `check-ignore` 의 규칙이 다르다
///
/// ```text
/// ls-files --error-unmatch -- '.palimpsest/index.redb/'  rc=1  (pathspec did not match)
/// ls-files --error-unmatch -- '.palimpsest/index.redb'   rc=0  (실제로 추적 중)
/// ls-files --error-unmatch -- '.palimpsest/cache/'       rc=0
/// ls-files --error-unmatch -- '.palimpsest/cache'        rc=0
/// ```
///
/// **후행 슬래시는 `check-ignore` 쪽에만 필요하다.** `ls-files` 에 붙이면 파일 경로가
/// 영원히 rc=1 이 된다.
#[test]
fn 추적_중이면_파일_경로에서도_말한다() {
    let root = 방("g-추적");
    std::fs::write(root.join("README.md"), "hello\n").expect("README");
    git(&root, &["init", "-q", "."]);
    std::fs::create_dir_all(root.join(".palimpsest/cache")).expect("cache");
    std::fs::write(root.join(".palimpsest/cache/c"), "c\n").expect("c");
    std::fs::write(root.join(".palimpsest/index.redb"), "i\n").expect("index");
    std::fs::write(root.join(".palimpsest/intent.redb"), "n\n").expect("intent");
    git(&root, &["add", "-A", "-f"]);
    git(&root, &["-c", "user.email=t@e", "-c", "user.name=t", "commit", "-qm", "첫"]);

    let report = 성공(&root, &["install"]);
    for 파생 in [".palimpsest/cache/", ".palimpsest/index.redb", ".palimpsest/intent.redb"] {
        assert!(
            report.lines().any(|l| l.contains("추적 중") && l.contains(파생)),
            "{파생} 가 추적 중인데 말하지 않았다:\n{report}"
        );
    }
}

/// **끝 개행이 없는 파일** — 그냥 append 하면 마지막 규칙과 우리 규칙이 둘 다 깨진다.
/// 그리고 **NUL 바이트** — 텍스트 필터가 사용자 줄을 자른 자리.
///
/// ⚠ NUL 을 `.gitignore` 가 아니라 `CLAUDE.md` 에 둔다. **git 은 NUL 이 든
/// `.gitignore` 의 그 줄을 「빈 패턴 = 전부 매치」로 읽고**(이 회차의 실측), 그러면
/// 모든 파생 경로가 「이미 덮임」이 되어 블록이 아예 안 생긴다 — 재려는 것이 사라진다.
#[test]
fn 끝_개행_없음과_널_바이트가_살아_있다() {
    let root = 빈_프로젝트("g-바이트");
    let 지시: &[u8] = b"# \x00\xff\xfe \xeb\x82\xb4 \xea\xb2\x83\n\xeb\x81\x9d\xea\xb0\x9c\xed\x96\x89 \xec\x97\x86\xec\x9d\x8c";
    let 무시: &[u8] = b"node_modules/";
    std::fs::write(root.join("CLAUDE.md"), 지시).expect("CLAUDE.md");
    std::fs::write(root.join(".gitignore"), 무시).expect(".gitignore");

    성공(&root, &["install"]);
    for (rel, 원본) in [("CLAUDE.md", 지시), (".gitignore", 무시)] {
        let 후 = std::fs::read(root.join(rel)).expect("읽기");
        assert!(후.starts_with(원본), "{rel}: 사용자 바이트가 깨졌다");
        assert!(후[원본.len()..].starts_with(b"\n"), "{rel}: 끝 개행을 안 넣어 줄 둘이 붙었다");
    }

    성공(&root, &["uninstall"]);
    for (rel, 원본) in [("CLAUDE.md", 지시), (".gitignore", 무시)] {
        assert_eq!(std::fs::read(root.join(rel)).expect("읽기"), 원본, "{rel}: 왕복이 원본과 다르다");
    }
}

/// **모드 · 심링크가 살아 있다.**
///
/// ⚠ **하드링크는 여기서 빠졌다.** 옛 회차는 *"하드링크가 안 끊긴다"* 를 여기서
/// 쟀는데, 그 성질과 **하드링크를 통해 대상 밖이 새는 것**은 같은 동작의 앞뒷면이다
/// (실측: 밖의 파일이 0바이트가 됐고 rc=0). **밖으로 새는 것을 막는 쪽을 이기게
/// 했다** — 지금은 하드링크가 걸린 자리에 아예 안 쓰고 멈춘다. 그것을 재는 자리는
/// `tests/install_hostile.rs` 다.
#[test]
#[cfg(unix)]
fn 모드와_심링크가_살아_있다() {
    use std::os::unix::fs::PermissionsExt;

    let root = 빈_프로젝트("g-메타");
    // 모드 — `CLAUDE.md` 를 600 으로.
    std::fs::write(root.join("CLAUDE.md"), "# 내 것\n").expect("CLAUDE.md");
    std::fs::set_permissions(root.join("CLAUDE.md"), std::fs::Permissions::from_mode(0o600))
        .expect("chmod");
    // 심링크 — `.gitignore` 가 다른 파일을 가리킨다.
    std::fs::write(root.join("진짜무시목록"), "node_modules/\n").expect("진짜");
    std::os::unix::fs::symlink("진짜무시목록", root.join(".gitignore")).expect("symlink");

    성공(&root, &["install"]);

    let mode = std::fs::metadata(root.join("CLAUDE.md")).expect("stat").permissions().mode();
    assert_eq!(mode & 0o777, 0o600, "모드가 소실됐다");
    assert!(
        std::fs::symlink_metadata(root.join(".gitignore")).expect("lstat").file_type().is_symlink(),
        "심링크가 일반 파일로 바뀌었다"
    );
    assert!(
        std::fs::read_to_string(root.join("진짜무시목록")).expect("읽기").contains("pal:begin"),
        "심링크 대상에 안 쓰였다"
    );
}

/// ★ **유닉스 밖에서 이 성질은 안 선다 — 그 사실이 시끄러워야 한다.**
///
/// ⚠ **여기는 「아직 안 쟀다」가 아니다.** 셋 중 둘은 **일부러 안 지킨다**:
///
/// | 성질 | 이 플랫폼에서 |
/// |---|---|
/// | 모드 600 이 살아 있다 | **개념이 없다.** Windows 에 모드 비트가 없다 |
/// | 심링크가 일반 파일로 안 바뀐다 | **fixture 를 못 세운다**(파일 심링크는 개발자 모드/`SeCreateSymbolicLinkPrivilege` 필요) **그리고 지키지도 않는다** — D34 가 이 플랫폼의 제자리 쓰기를 「끊고 쓰기」로 바꿨다 |
/// | 심링크 대상에 쓰인다 | 위와 같다 |
///
/// **그래도 초록을 안 낸다.** 통과로 바꾸면 이 자리는 *"아무것도 안 재면서 초록"* 이
/// 되고, 그것이 이 저장소가 짝 없는 `cfg` 에 대해 금지한 바로 그 상태다. 성질을
/// 되찾거나(안정 채널이 심링크 생성을 열거나) 포기 결정을 뒤집을 때 이 자리가 움직인다.
#[test]
#[cfg(not(unix))]
fn 모드와_심링크_보존이_이_플랫폼에서는_안_재진다() {
    panic!(
        "제자리 쓰기가 모드와 심링크를 살리는지를 이 플랫폼에서 **안 잰다. 그리고 \
         일부는 일부러 안 지킨다** — ① 모드 비트는 이 플랫폼에 개념이 없다 \
         ② 파일 심링크는 개발자 모드나 `SeCreateSymbolicLinkPrivilege` 없이 못 만들어 \
         fixture 가 안 선다(junction 은 디렉터리에만 걸린다) ③ **D34**(DESIGN §12.9)가 \
         이 플랫폼의 제자리 쓰기를 「끊고 쓰기」로 바꿨으므로 파일 정체성 보존은 \
         **포기한 값**이다 — 하드링크로 대상 밖이 새는 것을 막는 대가다"
    );
}

/// **쓰기 실패를 검사하지 않으면 쓰기 불가 디렉터리에서 rc=0 이 난다.**
#[test]
#[cfg(unix)]
fn 쓰기_불가_디렉터리에서_거짓_성공하지_않는다() {
    use std::os::unix::fs::PermissionsExt;

    let root = 빈_프로젝트("g-읽기전용");
    std::fs::set_permissions(&root, std::fs::Permissions::from_mode(0o555)).expect("chmod");
    let out = 돌린다(&root, &["install"]);
    // 되돌려 놓지 않으면 시험 방을 못 지운다.
    std::fs::set_permissions(&root, std::fs::Permissions::from_mode(0o755)).expect("chmod");

    assert!(!out.status.success(), "쓰기 불가 디렉터리에서 rc=0 을 냈다");
}

/// ★ **쓰기 불가 「디렉터리」는 이 플랫폼에서 아직 fixture 가 안 선다.**
///
/// ⚠ **파일 축은 이제 재진다** — `쓸_수_없는_자리는_미리_끊는다` 가 `set_readonly`
/// 로 이식되어 이 플랫폼에서도 `[f24]` ②(부분 설치 금지)의 트리거를 실제로 만든다.
/// 그래서 여기 남은 것은 **디렉터리 하나**다.
///
/// 실측(2026-08-17) — Windows 에서 디렉터리의 읽기 전용 속성은 쓰기를 안 막는다:
///
/// ```text
/// dir readonly attr  = true      ← 속성은 붙는다
/// dir create file    = Ok(())    ← 그런데 파일이 그대로 만들어진다
/// ```
///
/// 그 속성은 *"쓰지 마라"* 가 아니라 *"커스터마이즈된 폴더다"* 를 뜻한다. 그래서
/// 진짜 쓰기 불가 디렉터리는 **ACL(`icacls`)의 일**이고 std 밖이다.
/// 같은 실측이 제품 쪽 결함 하나도 냈다 — `install.rs` 의 `읽기_전용이_쓰기를_막는_종류인가`.
///
/// ★ **하나의 미측정 사실은 한 곳에서만 외친다.** 앞 회차는 이것을 두 자리에서
/// 외쳤고(되감기 쪽 · 거짓 성공 쪽), 같은 것을 두 곳에 적는 것이 곧 drift 다
/// (진행 규칙 4).
#[test]
#[cfg(not(unix))]
fn 쓰기_불가_디렉터리가_이_플랫폼에서는_안_재진다() {
    panic!(
        "**쓰기 불가 디렉터리**에서 rc=0 을 내는지 · 그때 되감기 (a) 가 서는지를 이 \
         플랫폼에서 아직 안 잰다 — 실측(2026-08-17): 디렉터리에 \
         `FILE_ATTRIBUTE_READONLY` 를 붙여도 `dir create file = Ok(())` 라 fixture 가 \
         안 선다. 진짜 쓰기 불가 디렉터리는 ACL(`icacls`)이고 std 밖이다.\n    \
         ★ **파일 축은 이제 재진다** — `쓸_수_없는_자리는_미리_끊는다`"
    );
}

/// **동시 설치 8회 → 블록 8개**(실측 · check-then-act 경쟁). 여기서 하나여야 한다.
#[test]
fn 동시_설치_여덟이_블록을_하나만_만든다() {
    let root = 살고_있는_프로젝트("g-경쟁");
    let 아이들: Vec<_> = (0..8)
        .map(|_| {
            Command::new(PAL)
                .args(["install"])
                .current_dir(&root)
                .env("PATH", path_앞에(&pal_dir()))
                .stdout(std::process::Stdio::null())
                .spawn()
                .expect("spawn")
        })
        .collect();
    let mut 성공수 = 0;
    for mut child in 아이들 {
        if child.wait().expect("wait").success() {
            성공수 += 1;
        }
    }
    assert!(성공수 >= 1, "여덟이 전부 실패했다");

    for file in ["CLAUDE.md", ".gitignore"] {
        let text = std::fs::read_to_string(root.join(file)).expect("읽기");
        assert_eq!(text.matches("pal:begin").count(), 1, "{file} 에 블록이 여럿이다:\n{text}");
    }
}

/// ★ **동시 설치가 되돌리기 기록을 날린다** — 파일 안 블록 수만 보면 이것이 안 걸린다.
///
/// 이전 매니페스트를 **잠금 밖에서** 읽으면 경쟁 프로세스가 전부 「이전 = 없음」을
/// 보고, 마지막 회차가 `blocks: []` · `settings: null` · `created_dirs: []` 인
/// 매니페스트를 쓴다. 그러면 `uninstall` 이 **rc=0 으로 「제거」 화면을 내면서** 블록도
/// 설정 키도 빈 디렉터리도 전부 남긴다 — **거짓 성공**이다.
#[test]
fn 동시_설치가_되돌리기_기록을_안_잃는다() {
    let root = 살고_있는_프로젝트("g-기록");
    let 아이들: Vec<_> = (0..8)
        .map(|_| {
            Command::new(PAL)
                .args(["install"])
                .current_dir(&root)
                .env("PATH", path_앞에(&pal_dir()))
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .spawn()
                .expect("spawn")
        })
        .collect();
    let mut 성공수 = 0;
    for mut child in 아이들 {
        if child.wait().expect("wait").success() {
            성공수 += 1;
        }
    }
    assert!(성공수 >= 1, "여덟이 전부 실패했다");

    // ① 매니페스트가 되돌릴 것을 전부 지고 있는가.
    let m = 값(&root.join(".claude/pal/manifest.json"));
    assert!(!m["blocks"].as_array().expect("blocks").is_empty(), "블록 기록이 사라졌다: {m}");
    assert!(m["settings"].is_object(), "설정 기록이 사라졌다: {m}");
    assert!(
        !m["created_dirs"].as_array().expect("created_dirs").is_empty(),
        "만든 디렉터리 기록이 사라졌다: {m}"
    );

    // ② 그래서 제거가 실제로 걷어내는가 — **거짓 성공 금지.**
    성공(&root, &["uninstall"]);
    for file in ["CLAUDE.md", ".gitignore"] {
        let text = std::fs::read_to_string(root.join(file)).expect("읽기");
        assert!(!text.contains("pal:begin"), "{file} 에 블록이 남았다:\n{text}");
    }
    let settings = 값(&root.join(".claude/settings.json"));
    assert!(settings.get("agent").is_none(), "설정 키가 남았다: {settings}");
    assert!(!root.join(".claude/pal").exists(), "빈 디렉터리가 남았다");
}

// ─────────────────────────────────────────────────────────────────────────────
// ⑤ doctor — 검사 다섯
// ─────────────────────────────────────────────────────────────────────────────

fn 검사들(root: &Path, path_env: Option<&str>) -> serde_json::Value {
    let mut cmd = Command::new(PAL);
    cmd.args(["doctor", "--install", "--json"]).current_dir(root);
    if let Some(p) = path_env {
        cmd.env("PATH", p);
    } else {
        cmd.env("PATH", path_앞에(&pal_dir()));
    }
    let out = cmd.output().expect("pal doctor");
    serde_json::from_slice(&out.stdout).expect("JSON")
}

fn 결말(checks: &serde_json::Value, number: usize) -> String {
    checks.as_array().expect("배열")[number - 1]["outcome"].as_str().expect("outcome").to_owned()
}

/// ★ **정상 fixture 에서 다섯이 전부 초록이다.** 이 줄이 없으면 항상 빨간 `doctor` 가
/// 아래 고장 다섯을 공짜로 통과한다.
#[test]
fn 정상이면_다섯이_전부_초록이다() {
    let root = 살고_있는_프로젝트("e-정상");
    성공(&root, &["install"]);
    let c = 검사들(&root, None);
    for n in 1..=5 {
        assert_eq!(결말(&c, n), "ok", "검사 {n} 이 초록이 아니다: {}", c[n - 1]);
    }
}

#[test]
fn 고장_다섯을_각각_지목한다() {
    // 1 — 깨진 JSON.
    let root = 살고_있는_프로젝트("e-1");
    성공(&root, &["install"]);
    std::fs::write(root.join(".claude/settings.json"), "{").expect("깨뜨리기");
    assert_eq!(결말(&검사들(&root, None), 1), "failed");

    // 2 — 파일 하나를 지우고, 하나를 고친다.
    let root = 살고_있는_프로젝트("e-2a");
    성공(&root, &["install"]);
    std::fs::remove_file(root.join(".claude/pal/INSTRUCTIONS.md")).expect("지우기");
    let c = 검사들(&root, None);
    assert_eq!(결말(&c, 2), "failed");
    assert!(c[1]["detail"].as_str().expect("detail").contains("INSTRUCTIONS.md"));

    let root = 살고_있는_프로젝트("e-2b");
    성공(&root, &["install"]);
    std::fs::write(root.join(".claude/pal/INSTRUCTIONS.md"), "고쳤다\n").expect("고치기");
    let c = 검사들(&root, None);
    assert_eq!(결말(&c, 2), "failed");
    assert!(c[1]["detail"].as_str().expect("detail").contains("sha256"));

    // 3 — 하위 디렉터리에서 실행.
    let root = 살고_있는_프로젝트("e-3");
    성공(&root, &["install"]);
    let 아래 = root.join("하위");
    std::fs::create_dir_all(&아래).expect("하위");
    let c = 검사들(&아래, None);
    assert_eq!(결말(&c, 3), "failed", "{}", c[2]);

    // 4 — `PATH` 에서 뺀 셸. **`git` 은 남긴다** — 그것까지 빼면 재는 것이 흐려진다.
    let root = 살고_있는_프로젝트("e-4");
    성공(&root, &["install"]);
    assert_eq!(결말(&검사들(&root, Some("/usr/bin:/bin")), 4), "failed");

    // 5 — 등재를 지운 프로젝트.
    let root = 살고_있는_프로젝트("e-5");
    성공(&root, &["install"]);
    std::fs::write(root.join(".gitignore"), "node_modules/\n").expect("등재 지우기");
    let c = 검사들(&root, None);
    assert_eq!(결말(&c, 5), "failed", "{}", c[4]);
}

/// ★ **사각지대가 조용하면 사각지대인 줄 모른다.**
///
/// `.claude/agents/` 는 **남의 에이전트가 함께 사는 곳**이라 매니페스트가 그쪽만
/// 「파일 하나짜리 뿌리」로 잡는다. 그래서 남의 파일이 들어와도 대조가 못 본다 —
/// **그것은 의도된 설계이고 안 바꾼다.** 바꾸는 것은 **말하는가**뿐이다.
#[test]
fn 진단이_에이전트_디렉터리의_남의_것을_보여준다() {
    let root = 살고_있는_프로젝트("e-남의에이전트");
    성공(&root, &["install"]);
    std::fs::write(root.join(".claude/agents/남의것.md"), "남의 에이전트\n").expect("남의 것");

    let c = 검사들(&root, None);
    // 설계대로 **고장이 아니다.**
    assert_eq!(결말(&c, 2), "ok", "의도된 설계인데 빨개졌다: {}", c[1]);
    let detail = c[1]["detail"].as_str().expect("detail");
    assert!(
        detail.contains("남의것.md") && detail.contains(".claude/agents"),
        "사각지대가 조용하다 — 남의 것이 들어온 사실이 화면에 없다: {detail}"
    );
}

/// 검사할 수 없는 것은 **`Residual`** 로 낸다 — `pal doctor` 가 이미 쓰는 어휘다.
#[test]
fn 설치가_없으면_잔여로_낸다() {
    let root = 빈_프로젝트("e-잔여");
    let c = 검사들(&root, None);
    assert_eq!(결말(&c, 2), "residual");
    assert_eq!(결말(&c, 3), "residual");
}

// ─────────────────────────────────────────────────────────────────────────────
// 부분 설치 — **되감거나 기록이 남거나 둘 중 하나다**
// ─────────────────────────────────────────────────────────────────────────────

/// ★ **쓸 수 없는 자리는 1단계에서 미리 끊는다** — 그러면 아무것도 안 남는다(되감기 (a)).
///
/// 관측된 트리거 셋 — `.gitignore` 444 · `CLAUDE.md` 444 · `settings.json` 444.
/// 셋 다 **읽기는 성공하고 쓰기만 실패해서** 옛 검증(`settings::read` 하나)을 통과했다.
/// ★ **모든 플랫폼에서 잰다** — fixture 를 모드 비트에서 **읽기 전용 속성**으로 옮겼다.
///
/// 옛 회차는 `PermissionsExt::from_mode(0o444)` 로 만들어서 `#[cfg(unix)]` 였고,
/// 다른 플랫폼에는 *"안 재진다"* 외침만 있었다. 그런데 [`std::fs::Permissions`] 의
/// `set_readonly` 는 **이식 가능**하고, 실측(2026-08-17)이 그것으로 충분함을 보였다:
///
/// | | `readonly()` 가 참이 되나 | 쓰기가 실제로 막히나 |
/// |---|---|---|
/// | 유닉스 (`0o644` → `0o444`) | 된다 | **막힌다** |
/// | Windows (`FILE_ATTRIBUTE_READONLY`) | 된다 | **막힌다** |
///
/// ⚠ **디렉터리는 이 fixture 로 못 만든다** — 같은 실측에서 Windows 는 디렉터리에
/// 속성이 붙어도 파일이 그대로 만들어졌다. 그쪽은 아래 짝이 계속 외친다.
#[test]
fn 쓸_수_없는_자리는_미리_끊는다() {
    for 이름 in [".gitignore", "CLAUDE.md", ".claude/settings.json"] {
        let root = 살고_있는_프로젝트(&format!("h-{}", 이름.replace(['/', '.'], "-")));
        let path = root.join(이름);
        읽기_전용(&path, true);

        let 전 = 스냅샷(&root);
        let stderr = 실패(&root, &["install"]);
        // 되돌려 놓지 않으면 시험 방을 못 지운다.
        읽기_전용(&path, false);

        assert_eq!(스냅샷(&root), 전, "{이름}: 부분 설치가 남았다");
        assert!(!root.join(".claude/pal").exists(), "{이름}: 디렉터리가 남았다");
        assert!(stderr.contains("쓸 수 없다"), "{이름}: 까닭을 안 적었다 — {stderr}");
    }
}

/// 읽기 전용을 켜고 끈다 — **이식 가능한 유일한 축.**
fn 읽기_전용(path: &Path, 켤까: bool) {
    let mut p = std::fs::metadata(path).expect("stat").permissions();
    p.set_readonly(켤까);
    std::fs::set_permissions(path, p).expect("권한");
}

// ⚠ **디렉터리 축의 외침은 여기 없다** — `쓰기_불가_디렉터리가_이_플랫폼에서는_안_재진다`
// 하나가 그 사실을 진다. 같은 미측정을 두 곳에서 외치면 그것이 곧 drift 다(진행 규칙 4).

/// ★ **아무것도 아직 못 놓은 자리에서 죽어도 걷어낼 수 있다.**
///
/// 기록이 살 집(`.claude/` · `.claude/pal/`)을 세운 직후가 가장 이른 실패 지점이다.
/// 그때 매니페스트는 `files: []` 이고, ⑥-b 의 *"하나도 못 찾았다"* 를 **적은 것이 0
/// 개인 자리에까지** 적용하면 제거가 거부되어 사용자에게 잔해만 남는다.
///
/// ⚠ **⑥-b 는 안 낮춘다.** 매니페스트가 리소스를 **적었는데** 하나도 못 찾은 자리는
/// 그대로 rc≠0 이다 — `리소스를_하나도_못_찾으면_실패한다` 가 그것을 계속 잰다.
#[test]
fn 아직_아무것도_못_놓은_잔해도_걷어낸다() {
    let root = 빈_프로젝트("h-이른죽음");
    let s0 = 스냅샷(&root);
    // `.claude/agents` 가 **파일**이면 디렉터리 세우기가 깨진다 — 권한 검사로는 못 보는
    // 자리이고, 그때는 기록의 집만 서 있고 놓인 것이 하나도 없다.
    std::fs::create_dir_all(root.join(".claude")).expect(".claude");
    std::fs::write(root.join(".claude/agents"), "걸림돌\n").expect("걸림돌");

    실패(&root, &["install"]);
    let m = 값(&root.join(".claude/pal/manifest.json"));
    assert!(m["files"].as_array().expect("files").is_empty(), "이 시험이 재려는 상태가 아니다: {m}");

    std::fs::remove_file(root.join(".claude/agents")).expect("걸림돌 치우기");
    성공(&root, &["uninstall"]);
    assert_eq!(스냅샷(&root), s0, "제거 후가 설치 전과 다르다");
    // **우리가 만든 것만 지운다** — `.claude/` 는 이 시험이 걸림돌을 놓으려고 먼저
    // 만들었으니 남는 것이 맞다. 우리가 만든 `.claude/pal/` 은 사라져야 한다.
    assert!(!root.join(".claude/pal").exists(), "우리가 만든 빈 디렉터리가 남았다");
}

/// ★ **미리 못 보는 자리에서 실패해도 `uninstall` 이 걷어낼 수 있다**(기록 (b)).
///
/// `CLAUDE.md` 가 **디렉터리**면 블록 단계에서 읽기가 깨진다 — 권한 검사로는 못 보는
/// 자리이고, 그때는 이미 파일 다섯과 설정 병합이 끝나 있다. 옛 코드는 매니페스트를
/// **마지막에** 썼으므로 그 오염에 **기록이 없었고**, `doctor` 는 *"설치를 찾지 못했다"*
/// 를, `uninstall` 은 rc=1 을 냈다 — 사용자에게 남는 길이 손으로 지우는 것뿐이었다.
#[test]
fn 미리_못_보는_실패도_기록을_남긴다() {
    let root = 빈_프로젝트("h-기록");
    let s0 = 스냅샷(&root);
    std::fs::create_dir_all(root.join("CLAUDE.md")).expect("걸림돌");

    실패(&root, &["install"]);

    // ① 오염이 남았다 — 그리고 **그 오염에 기록이 있다.**
    assert!(root.join(".claude/pal/manifest.json").is_file(), "기록이 없다");
    let c = 검사들(&root, None);
    assert_ne!(결말(&c, 2), "residual", "진단이 설치를 못 봤다: {}", c[1]);

    // ② 걸림돌을 치우면 제거가 걷어낸다 — **설치 전으로 돌아간다.**
    std::fs::remove_dir(root.join("CLAUDE.md")).expect("걸림돌 치우기");
    성공(&root, &["uninstall"]);

    let mut s2 = 스냅샷(&root);
    s2.remove(".claude/settings.json");
    assert_eq!(s2, s0, "제거 후가 설치 전과 다르다");
}
