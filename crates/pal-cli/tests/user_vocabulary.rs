//! **사용자 화면에 이 저장소의 작업 기록 어휘가 안 나간다** — 회차 · 조건 ID · 이슈 번호 ·
//! 기능 코드 · 규칙 ID · 문서 절.
//!
//! 회차 `2026-09-13-first-release-elsewhere` 의 `D1`·`D1-a`. 소유자의 선(2026-09-13)은
//! **회차·이슈·기능 코드**이고, `결박`·`반경` 같은 제품 용어는 이 검사 밖이다(#117).
//!
//! # 실행한 출력을 잰다 — 소스를 훑지 않는다
//!
//! 기능 코드는 문자열 리터럴이 아니라 **능력 데이터**(`CapabilityId::feature`)에서 화면으로
//! 온다. 소스를 `grep` 하면 그 몫이 안 걸리고 초록이 거짓이 된다. 그래서 바이너리를 돌려
//! 표준출력과 표준오류를 받아 잰다.
//!
//! # 패턴은 여기 한 벌이다
//!
//! 같은 파일의 `#[ignore]` 시험이 `PAL_VOCAB_SCAN=<파일들>` 로 **임의의 출력 파일**을 같은
//! 함수로 잰다 — 남의 저장소에서 뜬 화면도 이 함수를 지난다. 다른 언어로 다시 적지 않는다.

mod common;

use common::{git, PAL};
use regex::Regex;
use std::path::{Path, PathBuf};
use std::process::Command;

/// 금지 패턴 — **잠긴 의도에 등록한 문법 그대로다.** 지운 문구 목록에서 만들지 않는다.
///
/// 단어 경계는 **ASCII 경계 `(?-u:\b)`** 다. 유니코드 `\b` 는 한글도 단어 문자로 쳐서
/// `A5는` · `#133에서` 처럼 조사가 바로 붙은 꼴을 못 잡는다.
const 금지_패턴: &[(&str, &str)] = &[
    ("이 회차", r"이 회차"),
    ("앞 회차", r"앞 회차"),
    ("회차의 범위", r"회차의 범위"),
    ("잠근 축", r"잠근 축"),
    ("조건 ID", r"(?-u:\b)[A-KM-Z][0-9]+(-[a-z])?(?-u:\b)"),
    ("조건 ID (L5~)", r"(?-u:\b)L[5-9][0-9]*(?-u:\b)"),
    ("규칙 ID", r"(?-u:\b)[A-Z]-[0-9]+(?-u:\b)"),
    ("이슈 번호", r"(?:^|[^0-9A-Za-z])#[0-9]{1,5}(?-u:\b)"),
    ("기능 코드", r"(?-u:\b)F[0-9]{2}[a-z]?(?-u:\b)"),
    ("기능 절", r"\[f[0-9]{2}"),
    ("ADR", r"ADR-[0-9]{4}"),
    ("문서 절", r"§"),
];

/// 화면 하나에서 **`pal` 이 쓴 문구**의 금지 패턴 — `(패턴 이름, 맞은 문자열, 줄)`.
///
/// `사용자_내용` 은 그 화면에 실린 **사용자 저장소의 내용**(경로 · 앵커 · 심볼 이름 · 조각
/// 본문)이다. 그 부분 문자열을 지운 뒤 잰다 — 남의 저장소의 `ADR-0001-…` 는 그 저장소의
/// 이름이지 이 저장소의 어휘가 아니다.
fn 걸린_것(화면: &str, 사용자_내용: &[String]) -> Vec<(String, String, String)> {
    let mut 남은 = 화면.to_owned();
    let mut 내용: Vec<&String> = 사용자_내용.iter().filter(|s| !s.is_empty()).collect();
    // 긴 것부터 — 짧은 것이 긴 것의 조각을 먼저 지우면 긴 것이 안 지워진다.
    내용.sort_by_key(|s| std::cmp::Reverse(s.len()));
    for s in 내용 {
        남은 = 남은.replace(s.as_str(), " ");
    }
    let mut out = Vec::new();
    for 줄 in 남은.lines() {
        for (이름, 식) in 금지_패턴 {
            let re = Regex::new(식).expect("등록한 패턴이 regex 문법이 아니다");
            for m in re.find_iter(줄) {
                out.push(((*이름).to_owned(), m.as_str().trim().to_owned(), 줄.trim().to_owned()));
            }
        }
    }
    out
}

/// `--json` 산출에서 **사용자 저장소의 내용**만 뽑는다 — 이 키들의 문자열 값.
fn 사용자_내용(json: &serde_json::Value) -> Vec<String> {
    const 키: &[&str] = &["path", "anchor", "name", "body", "note", "target"];
    fn 걷는다(v: &serde_json::Value, out: &mut Vec<String>) {
        match v {
            serde_json::Value::Object(m) => {
                for (k, x) in m {
                    if let (true, Some(s)) = (키.contains(&k.as_str()), x.as_str()) {
                        out.push(s.to_owned());
                    }
                    걷는다(x, out);
                }
            }
            serde_json::Value::Array(a) => a.iter().for_each(|x| 걷는다(x, out)),
            _ => {}
        }
    }
    let mut out = Vec::new();
    걷는다(json, &mut out);
    out
}

// ─────────────────────────────────────────────────────────────────────────────
// D1-a — 음성 대조. **검사 함수가 고장이면 여기서 드러난다.**
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn d1a_심은_문자열은_띄어_쓴_꼴과_조사가_붙은_꼴_둘_다_걸린다() {
    let 심은 = [
        ("이 회차", "이 회차"),
        ("앞 회차", "앞 회차"),
        ("회차의 범위", "회차의 범위"),
        ("잠근 축", "잠근 축"),
        ("조건 ID", "A5"),
        ("조건 ID", "A5-a"),
        ("조건 ID (L5~)", "L7"),
        ("규칙 ID", "R-21"),
        ("이슈 번호", "#133"),
        ("기능 코드", "F13"),
        ("기능 절", "[f11"),
        ("ADR", "ADR-0023"),
        ("문서 절", "§"),
    ];
    for (이름, s) in 심은 {
        for 화면 in [format!("앞말 {s} 는 뒷말"), format!("앞말 {s}는 뒷말"), format!("앞말 {s}에서 뒷말")] {
            let 걸림 = 걸린_것(&화면, &[]);
            assert!(
                걸림.iter().any(|(n, _, _)| n == 이름),
                "패턴 「{이름}」 가 `{화면}` 를 안 잡는다 — 그 칸은 아무것도 안 잰다: {걸림:?}"
            );
        }
    }
}

#[test]
fn d1a_사용자_내용을_지워도_pal_이_쓴_문구는_걸린다() {
    let 화면 = "  fun · docs/ADR-0001-stack.md:3 · 미구축 F13\n  ### D1 — 결정";
    let 걸림 = 걸린_것(화면, &["docs/ADR-0001-stack.md".to_owned(), "### D1 — 결정".to_owned()]);
    // ⚠ **집합으로 댄다.** `F13` 은 등록한 패턴 둘(조건 ID · 기능 코드)에 함께 맞으므로
    //   같은 문자열이 두 번 나오는 것이 등록 그대로의 산출이다.
    let 맞은: std::collections::BTreeSet<&str> = 걸림.iter().map(|(_, m, _)| m.as_str()).collect();
    assert_eq!(맞은, std::collections::BTreeSet::from(["F13"]), "사용자 내용은 빠지고 `pal` 문구만 남아야 한다");
}

#[test]
fn d1a_제품_낱말과_해시와_등급은_안_걸린다() {
    // 선이 넓으면 검사가 제품 용어까지 지우라고 요구한다 — 소유자의 선은 작업 기록 어휘다.
    let 화면 = "회차를 연다 · L2 exact · L0 unavailable · UTF-8 · repo@aded7ce+worktree#335e5bc23a81 · body eace1ea25a2d · p95";
    let 걸림 = 걸린_것(화면, &[]);
    assert!(걸림.is_empty(), "걸리면 안 되는 것이 걸렸다: {걸림:?}");
}

// ─────────────────────────────────────────────────────────────────────────────
// D1 — 장면 명령의 사람 화면 전부
// ─────────────────────────────────────────────────────────────────────────────

/// 실패해도 멈추지 않고 표준출력과 표준오류를 **둘 다** 돌려준다 — 거부·오류 화면도 모집단이다.
fn 돌린다(cwd: &Path, args: &[&str]) -> String {
    let out = Command::new(PAL).args(args).current_dir(cwd).output().expect("pal 을 못 돌렸다");
    format!("{}\n{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr))
}

/// `--json` 호출의 **표준출력만** 값으로 읽는다. 못 읽으면 멈춘다 — 사용자 내용 지우기가
/// 조용히 비면 남의 이름이 걸림으로 세어지거나(거짓 빨강) 반대로 검사가 느슨해진다.
fn 값(cwd: &Path, args: &[&str]) -> serde_json::Value {
    let out = Command::new(PAL).args(args).current_dir(cwd).output().expect("pal 을 못 돌렸다");
    serde_json::from_slice(&out.stdout).unwrap_or_else(|e| {
        panic!("pal {args:?} 의 표준출력이 JSON 이 아니다({e}):\n{}", String::from_utf8_lossy(&out.stdout))
    })
}

fn 픽스처(tag: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("pal-vocab-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    let files: &[(&str, &str)] = &[
        ("tsconfig.json", r#"{"compilerOptions":{"moduleResolution":"bundler"}}"#),
        ("src/a/one.ts", "export function shared() { return 1; }\nexport function onlyA() { return shared(); }\n"),
        ("src/b/two.ts", "export function shared() { return 2; }\n"),
        ("src/b/use.ts", "import { onlyA } from '../a/one';\nimport { z } from 'zod';\nexport function user() { return onlyA() + z.length; }\n"),
        ("docs/decision.md", "# 결정\n\n## 결정\n\n`onlyA` 는 `shared` 를 부른다.\n\n## 다른 결정\n\n`user` 가 `onlyA` 를 부른다.\n"),
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

#[test]
fn d1_장면_명령의_사람_화면에_작업_기록_어휘가_없다() {
    let root = 픽스처("d1");
    let mut 화면들: Vec<(String, String, Vec<String>)> = Vec::new();
    let mut 잰다 = |이름: &str, args: &[&str], json_args: Option<&[&str]>| {
        let 화면 = 돌린다(&root, args);
        let 내용 = json_args.map(|j| 사용자_내용(&값(&root, j))).unwrap_or_default();
        화면들.push((이름.to_owned(), 화면, 내용));
    };

    잰다("pal --help", &["--help"], None);
    for c in ["install", "ledger", "narrative", "touch", "query"] {
        잰다(&format!("pal {c} --help"), &[c, "--help"], None);
    }
    잰다("install", &["install"], None);
    잰다("ledger", &["ledger"], Some(&["ledger", "--json"]));
    // 목록을 만들기 **전** — 승인 대기 구역이 「아직 만들지 않았다」를 싣는다.
    잰다("touch 목록 없음", &["touch", "onlyA"], Some(&["touch", "onlyA", "--json"]));
    잰다("narrative 인입", &["narrative"], Some(&["narrative", "--json"]));

    // 승인·거부 — 개체와 좌표는 `--json` 에서 뜬다.
    let 인입 = 값(&root, &["narrative", "--json"]);
    let 후보들: Vec<(String, Vec<String>)> = 인입["proposals"]
        .as_array()
        .expect("proposals")
        .iter()
        .filter_map(|p| {
            let c = p["class"]["candidates"].as_array()?;
            Some((p["item"]["id"].as_str()?.to_owned(), c.iter().filter_map(|x| x.as_str().map(str::to_owned)).collect()))
        })
        .collect();
    assert!(후보들.len() >= 2, "픽스처가 후보 있는 조각 둘을 못 만들었다: {후보들:?}");
    let (승인할, 좌표들) = &후보들[0];
    잰다("narrative 승인 성공", &["narrative", "--approve", 승인할, "--pick", &좌표들[0]], None);
    잰다("narrative 승인 거부(후보 밖 좌표)", &["narrative", "--approve", &후보들[1].0, "--pick", "0000000000"], None);
    잰다("narrative 거부", &["narrative", "--refuse", &후보들[1].0, "--pick", &후보들[1].1[0], "--reason", "시험"], None);

    잰다("touch 찾음", &["touch", "onlyA"], Some(&["touch", "onlyA", "--json"]));
    잰다("touch 후보 여럿", &["touch", "shared"], Some(&["touch", "shared", "--json"]));
    잰다("touch 못 찾음", &["touch", "nothingHere"], Some(&["touch", "nothingHere", "--json"]));
    잰다("touch 파일 간 까닭", &["touch", "user"], Some(&["touch", "user", "--json"]));
    잰다("query 후보 여럿", &["query", "symbol.callers", "shared"], Some(&["query", "symbol.callers", "shared", "--json"]));
    잰다("touch 지목", &["touch", "shared", "--pick", "0000000000"], Some(&["touch", "shared", "--pick", "0000000000", "--json"]));
    // 코드가 바뀐 뒤 — 승인 대기 구역이 「이 스냅샷의 것이 아니다」를 싣는다. **맨 끝에 둔다.**
    let 파일 = root.join("src/a/one.ts");
    let mut 본문 = std::fs::read_to_string(&파일).expect("읽기");
    본문.push_str("export function later() { return 3; }\n");
    std::fs::write(&파일, 본문).expect("쓰기");
    잰다("touch 목록 낡음", &["touch", "onlyA"], Some(&["touch", "onlyA", "--json"]));

    let mut 전부 = Vec::new();
    for (이름, 화면, 내용) in &화면들 {
        for (패턴, 맞은, 줄) in 걸린_것(화면, 내용) {
            전부.push(format!("[{이름}] 「{패턴}」 `{맞은}` — {줄}"));
        }
    }
    assert!(전부.is_empty(), "사람 화면에 작업 기록 어휘가 {}곳 남았다:\n{}", 전부.len(), 전부.join("\n"));
    let _ = std::fs::remove_dir_all(&root);
}

/// **남의 저장소에서 뜬 화면도 같은 함수로 잰다** — `PAL_VOCAB_SCAN` 에 파일 경로를 `:` 로
/// 이어 준다. 각 파일 옆에 `<파일>.json` 이 있으면 그것을 같은 호출의 `--json` 으로 읽어
/// 사용자 내용을 지운다.
#[test]
#[ignore = "PAL_VOCAB_SCAN 을 준 사람만 돌린다"]
fn d1_scan_주어진_출력_파일을_같은_함수로_잰다() {
    let 목록 = std::env::var("PAL_VOCAB_SCAN").expect("PAL_VOCAB_SCAN 이 없다");
    let mut 전부 = Vec::new();
    for 파일 in 목록.split(':').filter(|s| !s.is_empty()) {
        let 화면 = std::fs::read_to_string(파일).unwrap_or_else(|e| panic!("{파일}: {e}"));
        let 내용 = std::fs::read_to_string(format!("{파일}.json"))
            .ok()
            .and_then(|t| serde_json::from_str::<serde_json::Value>(&t).ok())
            .map(|v| 사용자_내용(&v))
            .unwrap_or_default();
        for (패턴, 맞은, 줄) in 걸린_것(&화면, &내용) {
            전부.push(format!("[{파일}] 「{패턴}」 `{맞은}` — {줄}"));
        }
    }
    println!("걸린 자리 {}", 전부.len());
    assert!(전부.is_empty(), "작업 기록 어휘가 {}곳 남았다:\n{}", 전부.len(), 전부.join("\n"));
}
