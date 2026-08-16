//! ★ **`core.autocrlf` 가 설치를 걷을 수 없게 만들던 자리** — 소유자 결정을 잰다.
//!
//! # 무엇이 깨졌었나 — 실측
//!
//! `git -c core.autocrlf=true clone` 으로 받은 워킹트리에서:
//!
//! | 경로 | 무엇이 났나 |
//! |---|---|
//! | `doctor` 검사 2 | 우리 파일 **다섯 전부 sha256 불일치**(빨강) |
//! | `update` | *"이미 최신입니다"* — 버전이 같아 **파일을 보지도 않는다** |
//! | `install` 재실행 | 다섯을 **`user_modified` 로 도장** → 초록이 되지만 **영원히 갱신 대상에서 빠진다** |
//! | `uninstall` | **통째로 거부** — 블록 제거가 바이트 완전 일치인데 실물이 CRLF. **걷어낼 방법이 없다** |
//!
//! # 소유자 결정 (2026-08-16)
//!
//! > **줄바꿈을 정규화해서 비교한다.**
//!
//! - sha256 을 뜰 때와 블록을 대조할 때 **CRLF→LF 로 맞춘 내용**으로 판정한다.
//! - 파일을 되쓸 때는 **그 파일의 기존 줄바꿈을 보존한다.**
//! - **사용자 프로젝트에 `.gitattributes` 를 추가하지 않는다** — 우리가 소유하는 파일이
//!   하나 더 늘고 병합 표면이 커진다.
//! - **줄바꿈만 다른 변화는 「사용자가 고쳤다」로 안 센다.**
//!
//! # 이 시험이 재는 것은 **전 경로**다
//!
//! `install` → `doctor` → `update` → `install` 재실행 → `uninstall`. 마지막이 **완주해서
//! 설치 전으로 돌아가야** 한다. 하나만 재면 그 사이에서 새는 것을 못 본다.

mod common;

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use common::{PAL, path_앞에};

/// 지시 파일의 원본 — 설치 전의 유일한 내용.
const 지시_원본: &str = "# 내 지시\n";

fn 방(tag: &str) -> PathBuf {
    let base = std::env::temp_dir().join(format!("pal-f24-줄바꿈-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    std::fs::create_dir_all(&base).expect("방");
    base
}

fn git(cwd: &Path, args: &[&str]) {
    let out = Command::new("git")
        .args(args)
        .current_dir(cwd)
        .output()
        .expect("git 을 못 돌렸다");
    assert!(out.status.success(), "git {args:?}: {}", String::from_utf8_lossy(&out.stderr));
}

fn 커밋(cwd: &Path, 말: &str) {
    git(cwd, &["add", "-A"]);
    git(cwd, &["-c", "user.email=t@example.com", "-c", "user.name=t", "commit", "-qm", 말]);
}

fn 돌린다(cwd: &Path, args: &[&str]) -> Output {
    let pal_dir = Path::new(PAL).parent().expect("pal 의 부모");
    Command::new(PAL)
        .args(args)
        .current_dir(cwd)
        .env("PATH", path_앞에(pal_dir))
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

fn 값(path: &Path) -> serde_json::Value {
    serde_json::from_slice(&std::fs::read(path).expect("읽기")).expect("JSON")
}

/// ★ **`core.autocrlf` 로 받은 클론에서 전 경로가 선다.**
#[test]
fn autocrlf_클론에서_설치_진단_갱신_제거가_전부_선다() {
    let base = 방("전경로");
    let src = base.join("src");
    std::fs::create_dir_all(&src).expect("src");
    git(&src, &["init", "-q", "."]);
    std::fs::write(src.join("README.md"), "hello\n").expect("README");
    std::fs::write(src.join("CLAUDE.md"), 지시_원본).expect("CLAUDE.md");
    커밋(&src, "설치 전");

    성공(&src, &["install"]);
    커밋(&src, "설치");

    // ── `core.autocrlf` 를 켠 클론 ────────────────────────────────────────────
    let dst = base.join("dst");
    let out = Command::new("git")
        .args(["clone", "-q", "-c", "core.autocrlf=true"])
        .arg(&src)
        .arg(&dst)
        .output()
        .expect("git clone 을 못 돌렸다");
    assert!(out.status.success(), "clone: {}", String::from_utf8_lossy(&out.stderr));

    // **이 시험이 재려는 상태인지 먼저 확인한다.** CRLF 가 안 들어왔으면 아무것도 안 잰다.
    for 자리 in ["CLAUDE.md", ".claude/pal/INSTRUCTIONS.md", ".gitignore"] {
        let bytes = std::fs::read(dst.join(자리)).expect("읽기");
        assert!(
            bytes.windows(2).any(|w| w == b"\r\n"),
            "{자리} 에 CRLF 가 안 들어왔다 — 이 시험이 재려는 상태가 아니다"
        );
    }

    // ── ① `doctor` 검사 2 가 초록이다 ────────────────────────────────────────
    let 화면 = 성공(&dst, &["doctor", "--install", "--json"]);
    let 검사: serde_json::Value = serde_json::from_str(&화면).expect("JSON");
    let 둘째 = 검사
        .as_array()
        .expect("배열")
        .iter()
        .find(|c| c["number"] == 2)
        .expect("검사 2")
        .clone();
    assert_eq!(
        둘째["outcome"], "ok",
        "줄바꿈만 다른데 매니페스트 대조가 빨갛다: {둘째}"
    );

    // ── ② `update` 가 돈다 ───────────────────────────────────────────────────
    성공(&dst, &["update"]);

    // ── ③ 재설치가 **`user_modified` 로 도장을 안 찍는다** ────────────────────
    성공(&dst, &["install"]);
    let m = 값(&dst.join(".claude/pal/manifest.json"));
    for f in m["files"].as_array().expect("files") {
        assert_eq!(
            f["origin"], "ours",
            "줄바꿈만 다른 것을 사용자 수정으로 셌다: {}",
            f["path"]
        );
    }

    // ── ④ `uninstall` 이 **완주해서 설치 전으로 돌아간다** ────────────────────
    성공(&dst, &["uninstall"]);
    assert!(!dst.join(".claude").exists(), "`.claude/` 가 남았다");
    assert!(!dst.join(".gitignore").exists(), "우리가 만든 `.gitignore` 가 남았다");
    assert_eq!(
        std::fs::read(dst.join("CLAUDE.md")).expect("읽기"),
        b"# \xEB\x82\xB4 \xEC\xA7\x80\xEC\x8B\x9C\r\n",
        "설치 전의 CLAUDE.md(CRLF)로 안 돌아갔다"
    );
    assert_eq!(std::fs::read(dst.join("README.md")).expect("읽기"), b"hello\r\n");
}

/// ★ **CRLF 파일에 블록을 새로 넣을 때도 그 파일의 줄바꿈을 쓴다.**
///
/// 정규화 대조만 있고 되쓰기 보존이 없으면, 우리가 넣은 블록만 LF 가 되어 사용자의
/// `git status` 에 매번 뜬다.
#[test]
fn crlf_파일에_넣은_블록도_crlf_다() {
    let base = 방("넣기");
    let root = base.join("repo");
    std::fs::create_dir_all(&root).expect("repo");
    git(&root, &["init", "-q", "."]);
    std::fs::write(root.join("README.md"), "hello\r\n").expect("README");
    // 사용자의 지시 파일이 **이미 CRLF** 다.
    std::fs::write(root.join("CLAUDE.md"), "# 내 지시\r\n").expect("CLAUDE.md");

    성공(&root, &["install"]);

    let bytes = std::fs::read(root.join("CLAUDE.md")).expect("읽기");
    let 텍스트 = String::from_utf8(bytes).expect("UTF-8");
    let 우리_줄: Vec<&str> =
        텍스트.split_inclusive('\n').filter(|l| l.contains("pal:") || l.contains("@.claude")).collect();
    assert!(!우리_줄.is_empty(), "우리 블록이 안 들어갔다:\n{텍스트}");
    for 줄 in &우리_줄 {
        assert!(줄.ends_with("\r\n"), "우리가 넣은 줄이 LF 다: {줄:?}");
    }

    // 그리고 **왕복하면 원본 바이트로 돌아간다.**
    성공(&root, &["uninstall"]);
    assert_eq!(
        std::fs::read(root.join("CLAUDE.md")).expect("읽기"),
        "# 내 지시\r\n".as_bytes()
    );
}

/// ★ **`settings.json` 도 그 파일의 줄바꿈으로 되쓴다.**
///
/// 블록(`CLAUDE.md`·`.gitignore`)에는 이 규율이 이미 서 있었는데 `settings.json` 만
/// 문 밖에 있었다 — `serde_json::to_string_pretty` 는 언제나 LF 를 낸다. 그래서
/// `core.autocrlf=true` 워킹트리에서는 **되쓸 때마다 파일의 모든 줄이 바뀌고**, git 이
/// *"LF will be replaced by CRLF"* 를 매번 냈다.
///
/// **플랫폼 때문에 결과가 갈리는 자리다** — 유닉스 워킹트리에서는 아무 일도 안 난다.
///
/// ⚠ **직렬화 「형태」는 여기서 안 잰다.** 들여쓰기·키 순서가 우리 것이 되는 것은
/// 플랫폼 무관한 기존 결정이고(`install.rs` 의 ⑥ 이 `settings.json` 을 값 단위로 재는
/// 이유가 그것이다), 이 시험이 못 박는 것은 **줄바꿈 하나**다.
#[test]
fn crlf_설정도_crlf_로_되쓴다() {
    let base = 방("설정줄바꿈");
    let root = base.join("repo");
    std::fs::create_dir_all(root.join(".claude")).expect("repo");
    git(&root, &["init", "-q", "."]);
    std::fs::write(root.join("README.md"), "hello\r\n").expect("README");
    // 사용자의 설정이 **이미 CRLF** 이고, 형태는 이미 우리 직렬화와 같다 —
    // 그래야 이 시험이 **줄바꿈만** 가른다.
    let 원본 = "{\r\n  \"env\": {\r\n    \"A\": \"1\"\r\n  }\r\n}\r\n";
    std::fs::write(root.join(".claude/settings.json"), 원본).expect("settings");

    성공(&root, &["install"]);

    let bytes = std::fs::read(root.join(".claude/settings.json")).expect("읽기");
    // ① 홑 LF 가 하나도 없다 — 우리가 더한 줄도 CRLF 다.
    let 홑lf = bytes
        .iter()
        .enumerate()
        .filter(|(i, b)| **b == b'\n' && (*i == 0 || bytes[i - 1] != b'\r'))
        .count();
    assert_eq!(홑lf, 0, "되쓴 설정에 LF 줄이 {홑lf}개 남았다");
    // ② 그리고 실제로 우리가 뭔가 더했다 — 이 줄이 없으면 ① 이 공짜로 통과한다.
    let 텍스트 = String::from_utf8(bytes).expect("UTF-8");
    assert!(텍스트.contains("hooks"), "설정에 아무것도 안 더했다:\n{텍스트}");

    // ③ 왕복하면 **바이트로 원본이다.** 줄바꿈도 형태도 그대로 돌아온다.
    성공(&root, &["uninstall"]);
    assert_eq!(
        std::fs::read(root.join(".claude/settings.json")).expect("읽기"),
        원본.as_bytes(),
        "왕복 뒤 설정이 원본 바이트와 다르다"
    );
}
