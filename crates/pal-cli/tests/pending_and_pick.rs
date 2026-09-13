//! **승인 대기와 동명 고르기** — 남의 저장소 사용자가 코드를 만지려는 순간 스스로 닿는 입구.
//!
//! 회차 `2026-09-13-first-release-elsewhere` 의 `B1`·`B1-a`·`B2`·`B3`·`C1`·`C1-a`.

mod common;

use common::{git, pal, PAL};
use std::path::{Path, PathBuf};
use std::process::Command;

fn 저장소(tag: &str, files: &[(&str, &str)]) -> PathBuf {
    let root = std::env::temp_dir().join(format!("pal-pending-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    for (p, text) in files {
        let path = root.join(p);
        std::fs::create_dir_all(path.parent().expect("부모")).expect("디렉터리");
        std::fs::write(path, text).expect("쓰기");
    }
    git(&root, &["init", "-q", "."]);
    git(&root, &["add", "-A"]);
    git(&root, &["-c", "user.email=t@example.com", "-c", "user.name=t", "commit", "-qm", "첫 커밋"]);
    root
}

/// 승인 대기 픽스처 — `target` 을 가리키는 조각 둘(분류 `candidates` 하나 · `bound` 하나)과
/// 아무 조각도 안 가리키는 `lonely`.
fn 대기_저장소(tag: &str) -> PathBuf {
    저장소(
        tag,
        &[
            ("tsconfig.json", r#"{"compilerOptions":{"moduleResolution":"bundler"}}"#),
            (
                "src/core.ts",
                "export function helper() { return 0; }\n\
                 // @decision: target 은 한 번만 부른다\n\
                 export function target() { return helper(); }\n\
                 export function lonely() { return 1; }\n",
            ),
            ("docs/decisions.md", "# 결정들\n\n## 첫 결정\n\n`target` 은 `helper` 를 부른다.\n"),
        ],
    )
}

/// 사람 화면에서 **승인 대기 구역**만 뽑는다 — 제목 줄부터 다음 `■` 전까지.
fn 대기_구역(화면: &str) -> String {
    let mut out = Vec::new();
    let mut 안 = false;
    for 줄 in 화면.lines() {
        if 줄.starts_with("■ 승인 대기") {
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

/// 화면이 안내한 승인 명령들 — `승인: ` 뒤를 **고치지 않고** 뽑는다.
fn 안내된_명령(화면: &str, 머리: &str) -> Vec<Vec<String>> {
    화면.lines()
        .filter_map(|l| l.split_once(머리).map(|(_, rest)| rest.trim().to_owned()))
        .map(|cmd| cmd.split_whitespace().map(str::to_owned).collect())
        .collect()
}

/// 안내된 명령을 그대로 돌린다. 첫 낱말은 `pal` 이어야 한다.
fn 그대로_돌린다(cwd: &Path, cmd: &[String]) -> std::process::Output {
    assert_eq!(cmd.first().map(String::as_str), Some("pal"), "안내가 `pal` 로 시작하지 않는다: {cmd:?}");
    Command::new(PAL).args(&cmd[1..]).current_dir(cwd).output().expect("pal 을 못 돌렸다")
}

#[test]
fn b1_touch_가_승인_대기_조각을_경로_앵커_후보_수_승인_명령과_함께_싣는다() {
    let root = 대기_저장소("b1");
    pal(&root, &["narrative"]);
    let 화면 = pal(&root, &["touch", "target"]);
    let 구역 = 대기_구역(&화면);
    assert!(구역.contains("■ 승인 대기 — 이 좌표를 가리키는 문서 조각 (2)"), "조각 둘이 실려야 한다:\n{화면}");
    assert!(구역.contains("docs/decisions.md · 첫-결정 · 후보 "), "문서 조각(분류 candidates):\n{구역}");
    assert!(구역.contains("src/core.ts · L2 · 후보 1곳"), "주석 조각(분류 bound):\n{구역}");
    assert_eq!(안내된_명령(&구역, "승인: ").len(), 2, "조각마다 승인 명령이 있어야 한다:\n{구역}");
    assert!(화면.contains("■ 이 좌표에 걸린 것 (0)"), "승인 대기는 「걸린 것」에 섞이지 않는다:\n{화면}");

    // `B1-a` — 음성 대조. 아무 조각도 안 가리키는 좌표에는 구역이 0 건이다.
    let 외톨이 = 대기_구역(&pal(&root, &["touch", "lonely"]));
    assert!(외톨이.contains("(0)") && 외톨이.contains("없습니다"), "아무 좌표에나 조각을 붙인다:\n{외톨이}");
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn b2_화면이_안내한_명령을_그대로_돌리면_통한다() {
    let root = 대기_저장소("b2");
    let 인입 = pal(&root, &["narrative"]);
    // `narrative` 화면의 안내도 그대로 돌아야 한다 — 앞 판은 없는 하위 명령을 안내했다.
    let 예 = 안내된_명령(&인입, "예: ");
    assert_eq!(예.len(), 1, "narrative 화면에 실제로 도는 예시가 없다:\n{인입}");
    assert!(그대로_돌린다(&root, &예[0]).status.success(), "narrative 의 안내가 안 돈다: {:?}", 예[0]);

    let 화면 = pal(&root, &["touch", "target"]);
    let 명령들 = 안내된_명령(&대기_구역(&화면), "승인: ");
    assert!(!명령들.is_empty(), "승인 대기가 비었다 — 예시가 이미 승인했다면 다른 조각이 남아야 한다:\n{화면}");
    let 첫 = &명령들[0];
    let out = 그대로_돌린다(&root, 첫);
    assert!(out.status.success(), "touch 의 안내가 안 돈다: {첫:?}\n{}", String::from_utf8_lossy(&out.stderr));

    let 뒤 = pal(&root, &["touch", "target"]);
    assert!(!뒤.contains("■ 이 좌표에 걸린 것 (0)"), "승인했는데 걸린 것이 0 이다:\n{뒤}");
    assert!(!대기_구역(&뒤).contains(&첫.join(" ")), "승인한 조각이 대기에 남았다:\n{뒤}");
    assert!(!뒤.contains("이 스냅샷의 것이 아닙니다"), "승인은 코드를 안 바꾼다 — 낡음이 뜨면 거짓이다:\n{뒤}");

    // 거부한 조각도 빠진다.
    let 남은 = 안내된_명령(&대기_구역(&뒤), "승인: ");
    if let Some(cmd) = 남은.first() {
        let item = &cmd[cmd.iter().position(|a| a == "--approve").expect("--approve") + 1];
        let pick = &cmd[cmd.iter().position(|a| a == "--pick").expect("--pick") + 1];
        pal(&root, &["narrative", "--refuse", item, "--pick", pick, "--reason", "시험"]);
        let 거부_뒤 = 대기_구역(&pal(&root, &["touch", "target"]));
        assert!(!거부_뒤.contains(item.as_str()), "거부한 조각이 대기에 남았다:\n{거부_뒤}");
    }
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn b3_목록이_없거나_낡으면_0_을_안_찍는다() {
    let root = 대기_저장소("b3");
    // ⑴ 한 번도 인입하지 않았다.
    let 없음 = 대기_구역(&pal(&root, &["touch", "target"]));
    assert!(없음.contains("아직 만들지 않았습니다") && !없음.contains("(0)"), "목록이 없는데 0 을 찍는다:\n{없음}");

    // ⑶ 인입 직후 두 번 — 첫 `touch` 의 세대 교체가 목록을 지우면 둘째가 「없음」이 된다.
    pal(&root, &["narrative"]);
    let 첫 = 대기_구역(&pal(&root, &["touch", "target"]));
    let 둘 = 대기_구역(&pal(&root, &["touch", "target"]));
    assert!(첫.contains("(2)"), "인입 직후 목록이 비었다:\n{첫}");
    assert_eq!(첫, 둘, "두 번째 touch 에서 대기 구역이 달라졌다");

    // ⑵-a 워킹트리 편집 하나.
    let 파일 = root.join("src/core.ts");
    let 원본 = std::fs::read_to_string(&파일).expect("읽기");
    std::fs::write(&파일, format!("{원본}export function later() {{ return 2; }}\n")).expect("쓰기");
    let 낡음 = 대기_구역(&pal(&root, &["touch", "target"]));
    assert!(낡음.contains("이 스냅샷의 것이 아닙니다"), "워킹트리가 바뀌었는데 옛 목록을 싣는다:\n{낡음}");

    // ⑵-b 커밋 하나 — 다시 인입한 뒤 커밋이 늘면 또 낡는다.
    pal(&root, &["narrative"]);
    git(&root, &["-c", "user.email=t@example.com", "-c", "user.name=t", "commit", "-qam", "둘째"]);
    std::fs::write(&파일, format!("{원본}export function later() {{ return 3; }}\n")).expect("쓰기");
    git(&root, &["-c", "user.email=t@example.com", "-c", "user.name=t", "commit", "-qam", "셋째"]);
    let 커밋_뒤 = 대기_구역(&pal(&root, &["touch", "target"]));
    assert!(커밋_뒤.contains("이 스냅샷의 것이 아닙니다"), "커밋이 늘었는데 옛 목록을 싣는다:\n{커밋_뒤}");
    let _ = std::fs::remove_dir_all(&root);
}

/// 동명 픽스처 — 다른 파일의 `shared` 둘, 같은 파일의 메서드 `run` 둘, 둘을 가리키는 문서.
fn 동명_저장소(tag: &str) -> PathBuf {
    저장소(
        tag,
        &[
            ("tsconfig.json", r#"{"compilerOptions":{"moduleResolution":"bundler"}}"#),
            ("src/a.ts", "export function shared() { return 1; }\n"),
            ("src/b.ts", "export function shared() { return 2; }\nexport function useB() { return shared(); }\n"),
            ("src/jobs.ts", "export class First { run() { return 1; } }\nexport class Second { run() { return 2; } }\n"),
            ("docs/note.md", "# 메모\n\n## 공유\n\n`shared` 를 쓴다.\n"),
        ],
    )
}

/// 후보 화면의 지목 문자열 — `(경로:줄, 지목)`.
fn 지목들(화면: &str) -> Vec<(String, String)> {
    화면.lines()
        .filter_map(|l| {
            let (앞, 지목) = l.rsplit_once("  지목 ")?;
            let 자리 = 앞.split_whitespace().last()?.to_owned();
            Some((자리, 지목.trim().to_owned()))
        })
        .collect()
}

#[test]
fn c1_후보_화면의_지목_문자열을_그대로_주면_하나가_나온다() {
    let root = 동명_저장소("c1");
    for (이름, 경로들) in [("shared", ["src/a.ts:1", "src/b.ts:1"]), ("run", ["src/jobs.ts:1", "src/jobs.ts:2"])] {
        let 후보 = pal(&root, &["touch", 이름]);
        assert!(후보.contains("하나를 지목하려면 같은 명령에 --pick <지목> 을 붙이십시오"), "지목 방법이 없다:\n{후보}");
        let 목록 = 지목들(&후보);
        assert_eq!(목록.iter().map(|(p, _)| p.as_str()).collect::<Vec<_>>(), 경로들, "후보마다 지목이 있어야 한다:\n{후보}");
        for (자리, 지목) in &목록 {
            let 찾음 = pal(&root, &["touch", 이름, "--pick", 지목]);
            assert!(찾음.contains(&format!("· {자리} ·")), "지목 {지목} 으로 {자리} 가 안 나온다:\n{찾음}");
            let 질의 = pal(&root, &["query", "symbol.callers", 이름, "--pick", 지목]);
            assert!(!질의.contains("후보가"), "`pal query` 가 지목을 안 받는다:\n{질의}");
        }
    }

    // 같은 지목 문자열이 `pal narrative --pick` 에도 통한다.
    let 인입: serde_json::Value = serde_json::from_str(&pal(&root, &["narrative", "--json"])).expect("json");
    let 조각 = 인입["proposals"]
        .as_array()
        .expect("proposals")
        .iter()
        .find(|p| p["fragment"]["path"] == "docs/note.md")
        .expect("docs/note.md 조각");
    let item = 조각["item"]["id"].as_str().expect("id").to_owned();
    let (_, 지목) = 지목들(&pal(&root, &["touch", "shared"])).remove(0);
    let out = Command::new(PAL)
        .args(["narrative", "--approve", &item, "--pick", &지목])
        .current_dir(&root)
        .output()
        .expect("pal");
    assert!(out.status.success(), "narrative --pick 이 같은 지목을 안 받는다: {}", String::from_utf8_lossy(&out.stderr));
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn c1a_어느_후보에도_안_맞는_지목은_고르지_않는다() {
    let root = 동명_저장소("c1a");
    for args in [&["touch", "shared", "--pick", "0123456789ab"][..], &["query", "symbol.callers", "shared", "--pick", "0123456789ab"][..]] {
        let 화면 = pal(&root, args);
        assert!(화면.contains("`shared` 의 후보가 2건입니다"), "틀린 지목으로 하나를 골랐다 {args:?}:\n{화면}");
    }
    let _ = std::fs::remove_dir_all(&root);
}
