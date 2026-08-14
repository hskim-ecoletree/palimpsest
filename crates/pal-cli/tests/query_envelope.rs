//! **질의 실행기** — 봉투를 벗을 수 없고, 로그가 쌓이고, 범위가 질의마다 다르다.
//!
//! 합격선 정본은 `corpus/criteria.toml` `[f05.3.pass]` ①②⑤ 와
//! `[f05].pass.everything_that_answers_carries_an_envelope`.
//!
//! # 필드 이름 여섯을 **코드에 상수로 박는다**
//!
//! 골든을 산출에서 떠서 만들면 무엇이 빠져도 통과한다 — 빠진 채로 떠지기 때문이다.
//! 그래서 봉투의 여섯을 여기 적어 두고 골든과 **따로** 센다. 골든이 바뀌어도 이 여섯은
//! 코드에 남는다.

mod common;

use common::{git, pal};
use std::path::PathBuf;

/// F05 §5.1 이 적은 봉투의 성분. **하나라도 빠지면 실패다.**
const 봉투의_여섯: [&str; 6] =
    ["snapshot", "projection", "coverage", "capabilities", "ledger", "elision"];

/// 이 빌드가 답하는 질의와 그 인자.
const 질의들: [(&str, Option<&str>); 6] = [
    ("ledger.snapshot", None),
    ("symbol.resolve", Some("도움")),
    ("symbol.contains", Some("도움")),
    ("symbol.callers", Some("도움")),
    ("symbol.reaches", Some("부름")),
    ("graph.dump", None),
];

fn 저장소(tag: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("pal-f05q-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("임시 저장소");
    std::fs::write(
        root.join("a.ts"),
        "export function 도움() { return console }\n\
         export function 부름() { return 도움() }\n\
         export class 담는것 { 메서드() { return 1 } }\n",
    )
    .expect("a.ts");
    std::fs::write(root.join("b.kt"), "class Kt { fun m() {} }\n").expect("b.kt");
    git(&root, &["init", "-q", "."]);
    git(&root, &["add", "-A"]);
    git(&root, &["-c", "user.email=t@example.com", "-c", "user.name=t", "commit", "-qm", "첫"]);
    root
}

fn 질의(repo: &std::path::Path, name: &str, arg: Option<&str>) -> serde_json::Value {
    let mut args = vec!["query", name];
    if let Some(a) = arg {
        args.push(a);
    }
    args.push("--json");
    serde_json::from_str(&pal(repo, &args)).expect("봉투 JSON")
}

#[test]
fn 모든_질의가_봉투를_지고_나온다() {
    let repo = 저장소("envelope");
    // **하한이다** — 질의 목록이 비면 아래가 공짜로 통과한다.
    let 목록 = pal(&repo, &["query", "--list"]);
    assert_eq!(목록.lines().count(), 질의들.len(), "질의 목록과 시험 표가 어긋났다");

    for (name, arg) in 질의들 {
        let v = 질의(&repo, name, arg);
        for 필드 in 봉투의_여섯 {
            assert!(v.get(필드).is_some(), "`{name}` 의 답에 `{필드}` 가 없다");
        }
        assert!(v.get("answer").is_some(), "`{name}` 의 답에 `answer` 가 없다");
        // **절단이 없어도 실린다.** `Elision::none()` 이 타입 수준의 장치이고
        // 이 줄이 그것의 산출 수준 검사다.
        assert!(v["elision"]["truncated"].is_array(), "`{name}` 의 절단이 배열이 아니다");
        assert!(v["elision"]["limits_hit"].is_array());
    }

    // `pal touch` 도 같은 봉투를 진다 — F05 가 지는 표면이 둘이다.
    let t = 질의(&repo, "symbol.resolve", Some("도움"));
    let _ = t;
    let v: serde_json::Value =
        serde_json::from_str(&pal(&repo, &["touch", "도움", "--json"])).expect("touch JSON");
    for 필드 in 봉투의_여섯 {
        assert!(v.get(필드).is_some(), "`pal touch` 의 답에 `{필드}` 가 없다");
    }

    let _ = std::fs::remove_dir_all(&repo);
}

#[test]
fn 질의_로그가_쌓이고_재구축이_지우지_않는다() {
    // `[f05.3.pass]` ② — **처음부터 켜지 않으면 F17 은 데이터가 없어 착수할 수 없다.**
    let repo = 저장소("log");
    let index = repo.join(".palimpsest/index.redb");

    // 질의를 열 번 돌린다. 매 회차가 스티칭을 다시 하므로 **재구축이 끼어 있다.**
    for _ in 0..10 {
        질의(&repo, "symbol.resolve", Some("도움"));
    }

    // ⚠ **2층을 연 채로 `pal` 을 부르면 안 된다** — `redb` 가 배타 락을 잡는다.
    // 그 사실 자체가 이 세션의 발견이고 게이트에 적혀 있다(F05 §6 은 *"읽기는 동시
    // 가능"* 이라 적었다).
    let 읽기 = |index: &std::path::Path| {
        let p = pal_store::Projection::open(index).expect("2층");
        let 스냅샷 = p.built_for().expect("메타").expect("스냅샷");
        p.query_log(&스냅샷).expect("질의 로그")
    };

    let 줄 = 읽기(&index);
    assert_eq!(줄.len(), 10, "질의 열 번에 {}줄이다 — 재구축이 로그를 지웠거나 안 남겼다", 줄.len());
    assert!(줄.iter().all(|e| e.query == pal_core::QueryName::SymbolResolve));
    // 같은 인자는 같은 요약이다.
    assert_eq!(줄[0].args_digest, 줄[9].args_digest);
    // **접근한 좌표가 실린다** — 비면 F17 이 셀 것이 없다.
    assert!(!줄[0].accessed.is_empty(), "접근 좌표가 비었다");

    // 다른 질의를 하나 더 — 줄이 늘고 앞의 열은 그대로다.
    질의(&repo, "ledger.snapshot", None);
    let 줄2 = 읽기(&index);
    assert_eq!(줄2.len(), 11);
    assert_eq!(줄2[..10], 줄[..], "앞의 줄이 덮였다 — append-only 가 아니다");

    let _ = std::fs::remove_dir_all(&repo);
}

#[test]
fn 범위는_질의마다_다른_값이다() {
    // `[f05.3.pass]` ⑤ — **전역 합을 복사하면 답의 성질이 아니라 저장소의 성질이 된다.**
    let repo = 저장소("coverage");

    // `도움` 은 `console` 을 부른다 — 그 파일에 미해소가 있다.
    let a = 질의(&repo, "symbol.resolve", Some("도움"));
    // Kotlin 쪽은 스코프 체인이 없어 **셀 수 없다** — 0 이고, 그것은 「없음」이 아니라
    // 「안 만듦」이라 파일 노드가 진다.
    let b = 질의(&repo, "symbol.resolve", Some("Kt"));

    let 미해소 = |v: &serde_json::Value| v["coverage"]["unresolved"].as_u64().expect("unresolved");
    assert!(미해소(&a) > 0, "TypeScript 쪽 미해소가 0 이다 — 이 시험은 아무것도 안 잰다");
    assert_ne!(미해소(&a), 미해소(&b), "서로 다른 두 질의가 같은 범위를 냈다");

    // 최저 등급도 질의마다 다르다 — TypeScript 는 L2, Kotlin 은 L1.
    assert_ne!(a["coverage"]["lowest_grade"], b["coverage"]["lowest_grade"]);

    let _ = std::fs::remove_dir_all(&repo);
}

#[test]
fn 예산을_낮추면_절단이_정확한_사유와_상한으로_실린다() {
    // `[f05.1.pass]` ②③ 을 **표면에서** 다시 잰다 — 단위 시험은 순수 계산만 봤다.
    let repo = 저장소("elision");

    // ★ 넉넉하면 절단 0. **늘 자르는 구현이 아래를 통과한다.**
    let 넉넉 = 질의(&repo, "symbol.reaches", Some("부름"));
    assert!(넉넉["elision"]["truncated"].as_array().expect("truncated").is_empty(), "넉넉한데 잘랐다");
    let 닿은 = 넉넉["answer"]["symbols"].as_array().expect("symbols").len();
    assert!(닿은 >= 2, "닿은 것이 {닿은} 개다 — 절단을 유발할 그래프가 없다");

    // 노드 상한을 1 로 낮추면 **그 상한만** 걸린다.
    let 좁게 = serde_json::from_str::<serde_json::Value>(&pal(
        &repo,
        &["query", "symbol.reaches", "부름", "--node-max", "1", "--json"],
    ))
    .expect("봉투 JSON");
    let t = 좁게["elision"]["truncated"].as_array().expect("truncated");
    assert_eq!(t.len(), 1, "사유가 하나가 아니다: {t:?}");
    assert_eq!(t[0]["reason"], "node_max_exceeded");
    assert!(t[0]["count"].as_u64().expect("count") >= 1);
    let l = 좁게["elision"]["limits_hit"].as_array().expect("limits_hit");
    assert_eq!(l.len(), 1, "다른 상한이 함께 섰다: {l:?}");
    assert_eq!(l[0]["limit"], "node_max");
    assert_eq!(l[0]["value"], 1);

    let _ = std::fs::remove_dir_all(&repo);
}
