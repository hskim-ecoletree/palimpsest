//! **`touch` 가 호출자 자리를 싣고, 싣지 않은 나머지를 그대로 쳐서 펴지는 명령으로 안내한다.**
//!
//! 회차 `2026-09-14-first-release` 의 `B2`.
//!
//! # 기대값은 이 파일이 손으로 적는다
//!
//! 화면이 실은 자리를 `symbol.callers` 의 산출과만 대면 **같은 원천끼리 대는 항등식**이다 —
//! 둘 다 틀려도 통과한다. 그래서 픽스처를 쓴 사람이 줄을 세어 적은 집합과 댄다.

mod common;

use common::{PAL, git, pal};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;

/// 표시 상한 — `pal_core::PROVISIONAL_TOUCH_CALLER_PLACE_MAX`. **이 시험이 정한 값이 아니다** —
/// 바뀌면 아래 ⑴ 이 시끄럽게 진다(조용히 통과하지 않는다).
const 표시_상한: usize = 5;

/// `hub` — 같은 파일 셋 · 다른 파일 넷. **상한보다 많다.**
const 허브_기대: &[&str] = &[
    "src/hub.ts:2",
    "src/hub.ts:3",
    "src/hub.ts:4",
    "src/use.ts:2",
    "src/use.ts:3",
    "src/use.ts:4",
    "src/use.ts:5",
];

/// `src/b.ts` 의 `shared` — 동명 후보가 `src/a.ts` 에 하나 더 있다. 호출자 일곱.
const 공유_기대: &[&str] = &[
    "src/b.ts:2",
    "src/b.ts:3",
    "src/b.ts:4",
    "src/b.ts:5",
    "src/b.ts:6",
    "src/b.ts:7",
    "src/b.ts:8",
];

fn 저장소(tag: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("pal-callers-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    let files: &[(&str, &str)] = &[
        ("tsconfig.json", r#"{"compilerOptions":{"moduleResolution":"bundler"}}"#),
        (
            "src/hub.ts",
            "export function hub() { return 1; }\n\
             export function h1() { return hub(); }\n\
             export function h2() { return hub(); }\n\
             export function h3() { return hub(); }\n",
        ),
        (
            "src/use.ts",
            "import { hub } from './hub';\n\
             export function u1() { return hub(); }\n\
             export function u2() { return hub(); }\n\
             export function u3() { return hub(); }\n\
             export function u4() { return hub(); }\n",
        ),
        ("src/a.ts", "export function shared() { return 1; }\n"),
        (
            "src/b.ts",
            "export function shared() { return 2; }\n\
             export function b1() { return shared(); }\n\
             export function b2() { return shared(); }\n\
             export function b3() { return shared(); }\n\
             export function b4() { return shared(); }\n\
             export function b5() { return shared(); }\n\
             export function b6() { return shared(); }\n\
             export function b7() { return shared(); }\n",
        ),
    ];
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

/// 화면이 싣는 호출자 자리 — `호출자 자리` 머리 다음의 네 칸 들여쓴 줄마다 첫 낱말.
fn 실린_자리(화면: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut 안 = false;
    for 줄 in 화면.lines() {
        if 줄.trim_start().starts_with("호출자 자리") {
            안 = true;
            continue;
        }
        if 안 {
            match 줄.strip_prefix("    ") {
                Some(rest) => out.push(rest.split_whitespace().next().expect("자리").to_owned()),
                None => break,
            }
        }
    }
    out
}

/// 나머지 안내 — `(나머지 수, 명령 낱말들)`. 명령은 `전부 보려면: ` 뒤를 **고치지 않고** 뽑는다.
fn 나머지_안내(화면: &str) -> Option<(usize, Vec<String>)> {
    let 줄 = 화면.lines().find(|l| l.contains("전부 보려면: "))?;
    let (앞, 명령) = 줄.split_once("전부 보려면: ").expect("전부 보려면");
    let 수: String = 앞.split_once("그 밖 ")?.1.chars().take_while(char::is_ascii_digit).collect();
    Some((수.parse().ok()?, 명령.split_whitespace().map(str::to_owned).collect()))
}

/// 후보 화면의 지목 — `(경로:줄, 지목)`.
fn 지목들(화면: &str) -> Vec<(String, String)> {
    화면.lines()
        .filter_map(|l| {
            let (앞, 지목) = l.rsplit_once("  지목 ")?;
            Some((앞.split_whitespace().last()?.to_owned(), 지목.trim().to_owned()))
        })
        .collect()
}

/// ⑴ ⑵ ⑶ 을 한 심볼에 대해.
fn 대본다(root: &Path, touch: &[&str], 기대: &[&str]) {
    let 기대: BTreeSet<String> = 기대.iter().map(|s| (*s).to_owned()).collect();
    assert!(기대.len() > 표시_상한, "전제 — 기대 자리가 상한보다 많아야 나머지 경로가 재진다");

    let 화면 = pal(root, touch);

    // ⑴ 실린 자리 ⊂ 기대 · 수 = 상한
    let 실린 = 실린_자리(&화면);
    assert_eq!(실린.len(), 표시_상한, "{touch:?} — 실린 자리 수가 상한과 다르다:\n{화면}");
    let 실린_집합: BTreeSet<String> = 실린.iter().cloned().collect();
    assert_eq!(실린_집합.len(), 실린.len(), "{touch:?} — 같은 자리를 두 번 실었다:\n{화면}");
    assert!(
        실린_집합.is_subset(&기대),
        "{touch:?} — 기대에 없는 자리를 실었다: {:?}\n{화면}",
        실린_집합.difference(&기대).collect::<Vec<_>>()
    );

    // ⑵ 나머지 수 = 기대 크기 − 실린 수
    let (나머지_수, 명령) =
        나머지_안내(&화면).unwrap_or_else(|| panic!("{touch:?} — 나머지 수와 펴는 명령이 없다:\n{화면}"));
    assert_eq!(나머지_수, 기대.len() - 실린.len(), "{touch:?} — 나머지 수가 틀렸다:\n{화면}");

    // ⑶ 안내된 명령을 그대로 돌린 `--json` 의 자리 집합 = 기대
    assert_eq!(명령.first().map(String::as_str), Some("pal"), "안내가 `pal` 로 시작하지 않는다: {명령:?}");
    let out = Command::new(PAL)
        .args(&명령[1..])
        .arg("--json")
        .current_dir(root)
        .output()
        .expect("pal 을 못 돌렸다");
    assert!(out.status.success(), "안내된 명령이 안 돈다: {명령:?}\n{}", String::from_utf8_lossy(&out.stderr));
    let v: serde_json::Value = serde_json::from_slice(&out.stdout).expect("JSON");
    let 편: BTreeSet<String> = v["answer"]["symbols"]
        .as_array()
        .unwrap_or_else(|| panic!("안내된 명령 {명령:?} 이 자리 목록을 안 돌려줬다: {}", v["answer"]))
        .iter()
        .map(|s| format!("{}:{}", s["path"].as_str().expect("path"), s["span"]["line_start"]))
        .collect();
    assert_eq!(편, 기대, "안내된 명령 {명령:?} 이 편 자리가 기대와 다르다");
}

#[test]
fn b2_호출자가_상한보다_많으면_자리를_싣고_나머지를_그대로_펴는_명령을_안내한다() {
    let root = 저장소("b2");

    // 이름이 하나뿐인 심볼.
    대본다(&root, &["touch", "hub"], 허브_기대);

    // 동명 후보가 둘인 심볼 — 지목해서 만진다.
    let 후보 = pal(&root, &["touch", "shared"]);
    let (_, 지목) = 지목들(&후보)
        .into_iter()
        .find(|(자리, _)| 자리 == "src/b.ts:1")
        .unwrap_or_else(|| panic!("`src/b.ts:1` 의 지목이 없다:\n{후보}"));
    대본다(&root, &["touch", "shared", "--pick", &지목], 공유_기대);

    // **짝** — 호출자가 없는 심볼에는 자리도 나머지 안내도 없다. 늘 찍으면 아무것도 안 가른다.
    let 빈 = pal(&root, &["touch", "u1"]);
    assert!(실린_자리(&빈).is_empty() && 나머지_안내(&빈).is_none(), "호출자 0 인데 자리를 찍었다:\n{빈}");

    let _ = std::fs::remove_dir_all(&root);
}
