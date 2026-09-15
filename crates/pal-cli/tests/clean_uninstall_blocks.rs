//! **손으로 고친 pal 블록** — 회차 `2026-09-15-clean-uninstall` 완수 조건 C1 · C2.
//!
//! `doctor` 를 보는 시험은 `PATH` 에 시험 대상 `pal` 을 둔다(조건 머리말).

mod common;

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use common::snapshot::워킹트리;
use common::{PAL, git, path_앞에};

const 검사_이름: &str = "pal 블록이 넣은 그대로인가";
const 원래_지시: &str = "# 내 규칙\n지키자\n";
const 원래_무시: &str = "node_modules/\n";
const 더한_줄: &str = "사용자가 블록 안에 더한 줄";

struct 방 {
    base: PathBuf,
    repo: PathBuf,
}

impl 방 {
    fn 새(tag: &str) -> Self {
        let base = std::env::temp_dir().join(format!("pal-cu-b-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
        let repo = base.join("repo");
        std::fs::create_dir_all(&repo).expect("방");
        std::fs::create_dir_all(base.join("appr")).expect("밖의 저장소");
        std::fs::write(repo.join("README.md"), "hello\n").expect("README");
        std::fs::write(repo.join("CLAUDE.md"), 원래_지시).expect("CLAUDE.md");
        std::fs::write(repo.join(".gitignore"), 원래_무시).expect(".gitignore");
        git(&repo, &["init", "-q", "."]);
        git(&repo, &["add", "-A"]);
        git(&repo, &["-c", "user.email=t@e", "-c", "user.name=t", "commit", "-qm", "첫"]);
        let 방 = Self { base, repo };
        방.성공(&["install"]);
        방
    }

    fn 자리(&self, rel: &str) -> PathBuf {
        self.repo.join(rel)
    }

    fn 읽는다(&self, rel: &str) -> String {
        std::fs::read_to_string(self.자리(rel)).expect("읽기")
    }

    fn 고친다(&self, rel: &str, 앞: &str, 뒤: &str) {
        let 지금 = self.읽는다(rel);
        assert!(지금.contains(앞), "전제 — {rel} 에 「{앞}」가 있어야 한다:\n{지금}");
        std::fs::write(self.자리(rel), 지금.replacen(앞, 뒤, 1)).expect("쓰기");
    }

    fn 돌린다(&self, args: &[&str]) -> Output {
        Command::new(PAL)
            .args(args)
            .current_dir(&self.repo)
            .env("PATH", path_앞에(Path::new(PAL).parent().expect("pal 의 부모")))
            .env("PAL_APPROVAL_DIR", self.base.join("appr"))
            .output()
            .expect("pal 을 못 돌렸다")
    }

    fn 성공(&self, args: &[&str]) -> String {
        let out = self.돌린다(args);
        assert!(
            out.status.success(),
            "pal {args:?} rc={:?}\nstdout: {}\nstderr: {}",
            out.status.code(),
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
        String::from_utf8_lossy(&out.stdout).into_owned()
    }

    fn 검사들(&self) -> Vec<serde_json::Value> {
        let out = self.돌린다(&["doctor", "--install", "--json"]);
        let v: serde_json::Value = serde_json::from_slice(&out.stdout)
            .unwrap_or_else(|e| panic!("doctor JSON: {e}\n{}", String::from_utf8_lossy(&out.stderr)));
        v.as_array().expect("배열").clone()
    }

    fn 블록_검사(&self) -> serde_json::Value {
        let c = self.검사들();
        c.iter()
            .find(|x| x["name"] == 검사_이름)
            .unwrap_or_else(|| panic!("검사 「{검사_이름}」이 없다: {c:#?}"))
            .clone()
    }
}

impl Drop for 방 {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.base);
    }
}

/// 블록 안에 한 줄을 더한다 — 마커는 그대로다.
fn 블록_안을_고친다(방: &방, rel: &str) {
    let 안쪽 = if rel == ".gitignore" { "/.palimpsest/index.redb\n" } else { "@.claude/pal/INSTRUCTIONS.md\n" };
    방.고친다(rel, 안쪽, &format!("{안쪽}{더한_줄}\n"));
}

fn 합친_출력(out: &Output) -> String {
    format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr))
}

// ─────────────────────────────────────────────────────────────────────────────
// C1
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn c1_손으로_고친_블록은_doctor_가_빨갛고_update_가_그_파일을_말한다() {
    for rel in ["CLAUDE.md", ".gitignore"] {
        let 방 = 방::새(&format!("c1-{}", rel.trim_start_matches('.')));
        블록_안을_고친다(&방, rel);

        let c = 방.블록_검사();
        assert_eq!(c["outcome"], "failed", "{rel}: 고친 블록인데 검사가 빨갛지 않다: {c}");
        assert!(c["detail"].as_str().unwrap_or("").contains(rel), "{rel}: 검사가 그 파일을 지목하지 않는다: {c}");

        let 화면 = 합친_출력(&방.돌린다(&["update"]));
        assert!(
            화면.lines().any(|l| l.contains(rel) && l.contains("고친 블록")),
            "{rel}: update 출력이 고친 블록의 파일을 담지 않는다:\n{화면}"
        );
    }
}

#[test]
fn c1_안_고친_방과_줄바꿈만_바뀐_방은_doctor_의_모든_검사가_ok_다() {
    let 방 = 방::새("c1-그대로");
    let c = 방.검사들();
    assert!(c.iter().any(|x| x["name"] == 검사_이름), "검사 「{검사_이름}」이 없다: {c:#?}");
    for x in &c {
        assert_eq!(x["outcome"], "ok", "안 고친 방에서 검사가 ok 가 아니다: {x}");
    }

    let 방 = 방::새("c1-crlf");
    for rel in ["CLAUDE.md", ".gitignore"] {
        let lf = 방.읽는다(rel);
        assert!(!lf.contains('\r'), "전제");
        std::fs::write(방.자리(rel), lf.replace('\n', "\r\n")).expect("CRLF");
    }
    let c = 방.검사들();
    assert!(c.iter().any(|x| x["name"] == 검사_이름), "검사 「{검사_이름}」이 없다: {c:#?}");
    for x in &c {
        assert_eq!(x["outcome"], "ok", "줄바꿈만 바뀐 방에서 검사가 ok 가 아니다: {x}");
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// C2
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn c2_force_는_짝이_맞는_마커_사이를_걷고_지운_줄을_출력한다() {
    let 방 = 방::새("c2-force");
    블록_안을_고친다(&방, "CLAUDE.md");

    let 전 = 워킹트리(&방.repo);
    let 플래그_없이 = 방.돌린다(&["uninstall"]);
    assert_eq!(플래그_없이.status.code(), Some(1), "플래그 없는 uninstall 이 rc=1 이 아니다:\n{}", 합친_출력(&플래그_없이));
    assert_eq!(워킹트리(&방.repo), 전, "플래그 없는 uninstall 이 거부하면서 무언가를 썼다");

    let 화면 = 방.성공(&["uninstall", "--force"]);
    assert!(화면.contains(더한_줄), "지운 줄을 출력하지 않았다:\n{화면}");
    assert_eq!(방.읽는다("CLAUDE.md"), 원래_지시, "CLAUDE.md 가 설치 전 바이트가 아니다");
    assert_eq!(방.읽는다(".gitignore"), 원래_무시, ".gitignore 가 설치 전 바이트가 아니다");
}

#[test]
fn c2_닫는_마커가_없거나_여는_마커가_둘이면_force_도_거부하고_한_바이트도_안_쓴다() {
    let 여는 = "<!-- pal:begin — palimpsest. `pal uninstall` 이 이 블록을 걷어낸다 -->\n";
    let 닫는 = "<!-- pal:end -->\n";

    let 방 = 방::새("c2-닫는없음");
    방.고친다("CLAUDE.md", 닫는, "");
    let 전 = 워킹트리(&방.repo);
    let out = 방.돌린다(&["uninstall", "--force"]);
    assert_eq!(out.status.code(), Some(1), "닫는 마커가 없는데 --force 가 rc=1 이 아니다:\n{}", 합친_출력(&out));
    assert_eq!(워킹트리(&방.repo), 전, "거부했는데 무언가를 썼다");

    let 방 = 방::새("c2-여는둘");
    방.고친다("CLAUDE.md", "@.claude/pal/INSTRUCTIONS.md\n", &format!("@.claude/pal/INSTRUCTIONS.md\n{여는}"));
    let 전 = 워킹트리(&방.repo);
    let out = 방.돌린다(&["uninstall", "--force"]);
    assert_eq!(out.status.code(), Some(1), "여는 마커가 둘인데 --force 가 rc=1 이 아니다:\n{}", 합친_출력(&out));
    assert_eq!(워킹트리(&방.repo), 전, "거부했는데 무언가를 썼다");
}
