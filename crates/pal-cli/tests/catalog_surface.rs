//! **카탈로그가 표면에 그대로 서는가** — `[f06.1.pass]` ①의 방향 4 와 ④.
//!
//! # 왜 여기가 xtask 가 아닌가
//!
//! `cargo xtask check` 는 **정적**이다 — 빌드 산출에 의존하지 않는다(그래야 빌드가
//! 깨진 상태에서도 돈다). 그래서 xtask 는 *"CLI 소스에 질의 이름이 리터럴로 박혔는가"*
//! 를 재고, **산출 쪽 대조**(`pal query --list` 의 줄이 카탈로그와 같은가)는 여기가 진다.
//!
//! 둘이 함께 있어야 방향 4 가 성립한다 — 소스에 리터럴이 없어도 렌더링이 목록을
//! 빠뜨릴 수 있고, 산출이 맞아도 소스가 두 번째 목록을 키울 수 있다.

mod common;

use common::{PAL, 저장소};
use std::process::Command;

/// 정본. **런타임에 안 읽히지만 시험은 읽는다** — 그것이 이 대조의 요점이다.
const 카탈로그: &str = include_str!("../../../surface/queries.toml");

/// **하한** — 이보다 적으면 아래가 전부 공짜로 통과한다.
const 최소_질의: usize = 6;

fn 목록(json: bool) -> String {
    let mut args = vec!["query", "--list"];
    if json {
        args.push("--json");
    }
    let out = Command::new(PAL).args(&args).output().expect("pal 을 못 돌렸다");
    assert!(out.status.success(), "pal {args:?} 가 실패했다");
    String::from_utf8(out.stdout).expect("UTF-8")
}

#[test]
fn 카탈로그의_이름이_표면에_그대로_선다() {
    let c = pal_core::QueryCatalog::parse(카탈로그).expect("카탈로그가 읽힌다");
    assert!(c.queries.len() >= 최소_질의, "카탈로그가 {}개다 — 하한 미만", c.queries.len());

    let v: serde_json::Value = serde_json::from_str(&목록(true)).expect("목록 JSON");
    let mut 산출: Vec<String> = v["built"]
        .as_array()
        .expect("built 가 배열이 아니다")
        .iter()
        .map(|q| q["name"].as_str().expect("이름").to_owned())
        .collect();
    산출.sort();

    let mut 카탈로그의_이름: Vec<String> = c.names().into_iter().map(str::to_owned).collect();
    카탈로그의_이름.sort();

    assert_eq!(산출, 카탈로그의_이름, "표면이 내는 목록과 카탈로그가 어긋난다");

    // 인자·반환·도입도 함께 선다 — 이름만 맞고 나머지가 갈리면 계약이 아니다.
    for q in v["built"].as_array().expect("배열") {
        let name = q["name"].as_str().expect("이름");
        let decl = &c.queries[name];
        assert_eq!(q["returns"].as_str(), Some(decl.returns.as_str()), "{name} 의 반환");
        assert_eq!(q["introduced"].as_str(), Some(decl.introduced.as_str()), "{name} 의 도입");
        assert_eq!(
            q["args"].as_array().expect("인자 배열").len(),
            decl.args.len(),
            "{name} 의 인자 수"
        );
    }
}

#[test]
fn 답하는_것과_못_만든_것이_함께_서고_모양이_다르다() {
    // `[f06.1.pass]` ④ — 뭉개면 소비자가 *"있다"* 와 *"아직 없다"* 를 구별 못 한다.
    let v: serde_json::Value = serde_json::from_str(&목록(true)).expect("목록 JSON");
    let built = v["built"].as_array().expect("built");
    let not_built = v["not_built"].as_array().expect("not_built");

    // **하한 둘.** 한쪽이 0 이면 *"함께 낸다"* 가 검사되지 않는다.
    assert!(built.len() >= 최소_질의, "답하는 것이 {}개다", built.len());
    assert!(!not_built.is_empty(), "못 만든 것이 0 개다 — 이 빌드는 전부를 만들지 않았다");

    // **모양이 다르다.** 답하는 것은 `name` 을 지고, 못 만든 것은 **이름이 없고**
    // 기능 번호를 진다 — 이름을 적으면 그것이 곧 빈 자리다.
    for q in built {
        assert!(q.get("name").is_some(), "답하는 질의에 이름이 없다");
    }
    for c in not_built {
        assert!(c.get("feature").is_some(), "못 만든 능력에 기능 번호가 없다");
        assert!(c.get("name").is_none(), "못 만든 능력에 **질의 이름**이 붙었다 — 빈 자리다");
    }

    // 사람이 읽는 화면에서도 두 목록이 갈린다.
    let 화면 = 목록(false);
    assert!(화면.contains("답하는 질의"), "화면에 답하는 목록의 머리가 없다");
    assert!(화면.contains("못 만든 능력"), "화면에 미구축 목록의 머리가 없다");
}

#[test]
fn 목록은_저장소_없이_선다() {
    // *"호스트 없이도 코어가 답한다"* 의 가장 얕은 층이다. `--list` 가 저장소를 읽으면
    // 이 경로가 git 에 의존하게 되고, 그 순간 목록조차 환경에 종속된다.
    let 빈방 = std::env::temp_dir().join(format!("pal-f06-list-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&빈방);
    std::fs::create_dir_all(&빈방).expect("빈 방");
    let out = Command::new(PAL)
        .args(["query", "--list", "--json"])
        .current_dir(&빈방)
        .output()
        .expect("pal");
    assert!(out.status.success(), "git 저장소가 아닌 곳에서 `--list` 가 실패했다");
    let v: serde_json::Value = serde_json::from_slice(&out.stdout).expect("목록 JSON");
    assert!(v["built"].as_array().expect("built").len() >= 최소_질의);
    let _ = std::fs::remove_dir_all(&빈방);
}

#[test]
fn 모르는_이름은_봉투_없이_1_이고_못_찾은_이름은_봉투와_함께_0_이다() {
    // `[f06].exit_code_decision` — **「못 찾았다」는 실패가 아니다.**
    let repo = 저장소("f06-exit");

    let 모름 = Command::new(PAL)
        .args(["query", "refs.callers", "무엇"])
        .current_dir(&repo)
        .output()
        .expect("pal");
    assert_eq!(모름.status.code(), Some(1), "모르는 질의가 0 으로 끝났다");
    assert!(모름.stdout.is_empty(), "모르는 질의가 표준출력에 무언가를 냈다");
    assert!(!모름.stderr.is_empty(), "모르는 질의가 아무 말도 안 했다");

    let 못찾음 = Command::new(PAL)
        .args(["query", "symbol.resolve", "이런것은없다", "--json"])
        .current_dir(&repo)
        .output()
        .expect("pal");
    assert_eq!(못찾음.status.code(), Some(0), "못 찾은 이름이 실패로 끝났다");
    let v: serde_json::Value = serde_json::from_slice(&못찾음.stdout).expect("봉투 JSON");
    assert_eq!(v["answer"]["outcome"].as_str(), Some("unknown"), "빈 목록으로 답했다");
    // **봉투가 근거를 지고 있다** — 그래서 이것이 실패가 아니라 답이다.
    assert!(v["coverage"].is_object() && v["capabilities"].is_object());

    let _ = std::fs::remove_dir_all(&repo);
}
