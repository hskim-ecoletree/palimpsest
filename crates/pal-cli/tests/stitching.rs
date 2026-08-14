//! **1패스 스티칭** — 1층의 값이 손실 없이 2층으로 옮겨졌는가 (`[f05.2]`).
//!
//! 합격선 정본은 `corpus/criteria.toml` `[f05.2.pass]` 이고 판정은 `docs/gates/F05.md`.
//!
//! # 왜 실물 저장소인가
//!
//! 여기서 재는 것은 **1층 캐시가 실은 값이 2층에 그대로 도착했는가**이고, 그 사슬에는
//! git 접근 · 분류 · 캐시 · 스티칭이 전부 들어 있다. API 를 불러서는 그 사슬이 아니라
//! 마지막 마디만 재게 된다.
//!
//! # ⚠ 하한을 박는다 — **시험되지 않은 대조는 `–` 가 아니라 실패다** (`2e2eb3f`)
//!
//! 엣지가 0 이면 아래 전부가 공짜로 통과한다. 그래서 소스를 **여섯 갈래가 전부 1 건
//! 이상 나오도록** 짜고, 갈래마다 0 이 아님을 요구한다.

mod common;

use common::{git, pal};
use pal_core::{RefCounts, RepoPath, Slot};
use pal_store::Projection;
use std::path::PathBuf;

/// 여섯 갈래가 **전부** 나오는 저장소.
///
/// | 갈래 | 이 소스의 어디 |
/// |---|---|
/// | 선언 자리 | `function helper` 의 `helper` — 스코프 체인이 선언도 참조로 싣는다 |
/// | 엣지 | `caller` 안의 `helper(x)` |
/// | 지역 | `caller` 안의 `x` |
/// | 최상위 | 어느 선언에도 안 담긴 `helper(1)` 한 줄 |
/// | 파일 밖 | `console` |
/// | TDZ | `tdz` 안의 `z` 선언 전 참조 |
fn 저장소(tag: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("pal-f05-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("임시 저장소");

    std::fs::write(
        root.join("a.ts"),
        "export function helper(n: number) { return n + 1 }\n\
         export function caller() { const x = 2; return helper(x) }\n\
         export function outer() { console.log(1) }\n\
         export function tdz() { z; let z = 1; return z }\n\
         helper(1)\n",
    )
    .expect("a.ts");
    // **Kotlin 은 스코프 체인을 안 만든다.** 그 파일의 참조 수는 0 이 아니라 「안 만듦」이고,
    // 그것이 ② 다. TypeScript 만 있는 저장소로 시험하면 이 항목이 아예 안 걸린다.
    std::fs::write(root.join("gamma.kt"), "class Gamma { fun method() {} }\n").expect("gamma.kt");

    git(&root, &["init", "-q", "."]);
    git(&root, &["add", "-A"]);
    git(&root, &["-c", "user.email=t@example.com", "-c", "user.name=t", "commit", "-qm", "첫 커밋"]);
    root
}

fn 투영(repo: &std::path::Path) -> Projection {
    Projection::open(&repo.join(".palimpsest/index.redb")).expect("2층")
}

fn 참조_수(p: &Projection, path: &str) -> Slot<RefCounts> {
    p.file(&RepoPath::new(path)).expect("파일 노드 읽기").expect("파일 노드가 없다").refs
}

#[test]
fn 파일_내_엣지가_스코프_해소와_일치한다() {
    let repo = 저장소("edges");
    // 스티칭은 `pal touch` 가 돌린다 — 대장을 세우고 2층을 세운 뒤 답한다.
    let 답 = pal(&repo, &["touch", "helper", "--json"]);
    let p = 투영(&repo);

    // ── ① 여섯 갈래가 전부 선다 ────────────────────────────────────────────
    let Slot::Built(c) = 참조_수(&p, "a.ts") else {
        panic!("TypeScript 파일의 참조 수가 「안 만듦」이다 — 스코프 체인이 안 실렸다");
    };
    assert!(c.declarations > 0, "선언 자리를 하나도 안 셌다");
    assert!(c.edges > 0, "엣지가 0 이다 — 아래 전부가 공짜로 통과한다");
    assert!(c.locals > 0, "지역 변수 참조를 하나도 안 셌다");
    assert!(c.top_level > 0, "최상위 참조를 하나도 안 셌다");
    assert!(c.unresolved > 0, "파일 밖 참조를 하나도 안 셌다");
    assert!(c.before_declaration > 0, "TDZ 를 하나도 안 셌다");
    assert_eq!(c.total(), c.declarations + c.edges + c.locals + c.top_level + c.unresolved + c.before_declaration);

    // ── 두 방향의 행 수가 같다 ─────────────────────────────────────────────
    let 정 = p.edge_count().expect("정방향");
    let 역 = p.reverse_edge_count().expect("역방향");
    assert!(정 > 0, "엣지가 하나도 안 섰다");
    assert_eq!(정, 역, "같은 엣지의 두 방향인데 행 수가 다르다 — 한쪽 쓰기가 빠졌다");

    // ── 출발점과 도착점이 옳다 ─────────────────────────────────────────────
    let helper = p.resolve_name("helper").expect("이름").into_iter().next().expect("helper");
    let caller = p.resolve_name("caller").expect("이름").into_iter().next().expect("caller");
    let 부르는 = p.callers(helper.id).expect("역방향");
    assert!(부르는.contains(&caller.id), "`caller` 가 `helper` 를 부른 것이 안 실렸다");
    // **선언 자리를 걸렀으므로 자기 자신은 없다.** 안 거르면 모든 선언이 자기 엣지를 낳는다.
    assert!(!부르는.contains(&helper.id), "선언 자리가 자기 엣지로 남았다");

    // ── 그리고 그것이 답에 실린다 ──────────────────────────────────────────
    let v: serde_json::Value = serde_json::from_str(&답).expect("봉투 JSON");
    let facts = &v["answer"]["facts"]["present"];
    assert_eq!(facts["callers"].as_u64().expect("callers"), 부르는.len() as u64);
    assert!(v["coverage"]["unresolved"].as_u64().expect("unresolved") > 0, "미해소가 0 이다");

    let _ = std::fs::remove_dir_all(&repo);
}

#[test]
fn kotlin_의_참조는_0_이_아니라_안_만듦이다() {
    // ★ **ADR-0002 가 걸리는 자리다.** `0` 으로 적으면 *"참조가 없는 파일"* 과
    // *"참조를 안 보는 빌드"* 가 같은 출력이 된다.
    let repo = 저장소("kotlin");
    pal(&repo, &["touch", "Gamma", "--json"]);
    let p = 투영(&repo);

    assert!(
        matches!(참조_수(&p, "gamma.kt"), Slot::NotBuilt),
        "Kotlin 파일의 참조 수가 값으로 섰다 — 안 만든 능력이 0 으로 위장했다"
    );
    // **하한** — TypeScript 쪽은 값이어야 한다. 둘 다 「안 만듦」이면 이 시험은
    // 아무것도 안 재고 있다.
    assert!(matches!(참조_수(&p, "a.ts"), Slot::Built(_)), "TypeScript 쪽도 안 만듦이다");

    let _ = std::fs::remove_dir_all(&repo);
}

#[test]
fn 파일_노드와_내보내기가_실제로_찬다() {
    let repo = 저장소("exports");
    let 대장 = pal(&repo, &["ledger", ".", "--json"]);
    pal(&repo, &["touch", "helper", "--json"]);
    let p = 투영(&repo);

    let v: serde_json::Value = serde_json::from_str(&대장).expect("대장 JSON");
    let 센다 = |b: &str| {
        v["ledger"]["entries"]
            .as_array()
            .expect("entries")
            .iter()
            .filter(|e| e["state"].as_object().is_some_and(|o| o.contains_key(b)))
            .count()
    };
    let 그래프_있는 = 센다("parsed") + 센다("partial");
    assert!(그래프_있는 >= 2, "그래프가 있는 파일이 {그래프_있는} 개뿐이다");
    assert_eq!(
        p.file_count().expect("파일 수"),
        그래프_있는,
        "대장의 parsed+partial 과 2층의 파일 노드 수가 다르다"
    );

    // TypeScript 의 `export` 가 `EXPORTS` 에 선다. **하한이다** — 0 이면 안 세운 것이다.
    assert!(p.export_count().expect("내보내기 수") > 0, "`EXPORTS` 가 비었다");
    let helper = p.resolve_name("helper").expect("이름").into_iter().next().expect("helper");
    assert_eq!(
        p.export(&RepoPath::new("a.ts"), "helper").expect("내보내기"),
        Some(helper.id),
        "`helper` 가 `EXPORTS` 에서 안 나온다"
    );
    // Kotlin 은 `exports: NotBuilt` 이므로 안 선다 — **0 건이 아니라 능력의 부재다.**
    assert_eq!(p.export(&RepoPath::new("gamma.kt"), "Gamma").expect("내보내기"), None);

    let _ = std::fs::remove_dir_all(&repo);
}

#[test]
fn 봉투가_재구축_상태와_스냅샷을_값으로_싣는다() {
    // 이 둘은 지금까지 **관측이 아니라 기본값**이었다 —
    // `rebuild` 는 `NotBuilt{F05}` 였고 `built_for_this_snapshot` 은 `true` 로 박혀 있었다.
    let repo = 저장소("envelope");
    let 답 = pal(&repo, &["touch", "helper", "--json"]);
    let v: serde_json::Value = serde_json::from_str(&답).expect("봉투 JSON");

    assert_eq!(
        v["projection"]["rebuild"]["present"], "settled",
        "재구축 상태가 값이 아니다: {}", v["projection"]["rebuild"]
    );
    assert_eq!(
        v["projection"]["built_for_this_snapshot"], true,
        "방금 세운 스냅샷인데 아니라고 적었다"
    );
    assert!(v["projection"]["symbols_indexed"].as_u64().expect("symbols_indexed") >= 4);

    let _ = std::fs::remove_dir_all(&repo);
}
