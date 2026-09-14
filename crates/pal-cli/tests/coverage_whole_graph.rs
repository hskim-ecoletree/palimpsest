//! **그래프 전체를 만지는 명령의 `coverage` 가 `graph.dump` 와 같은 값이다** — #127 ①.
//!
//! 회차 `2026-09-14-first-release` 의 `A1`·`A1-a`. 합격선 정본은 그 회차의 잠긴 의도다.
//!
//! # 무엇이 틀렸었나
//!
//! `pal doctor` 와 `pal export` 가 응답 묶음의 `coverage` 를 `unresolved: 0` ·
//! `lowest_grade: L0` 으로 **손으로 박았다.** 같은 인덱스의 `graph.dump` 는 착수 실측에서
//! `13404 · l1` 이었다 — 팀원에게 **「못 본 것이 0」이라는 거짓**이 나가던 자리다.
//!
//! # ⚠ 전제를 먼저 단언한다 (`A1-a`)
//!
//! 픽스처의 `graph.dump` 값이 `unresolved 0 · lowest_grade l0` 이면 하드코딩과 **바이트로
//! 같아서** 아래 대조가 공짜로 통과한다. 그래서 그 두 값이 아님을 먼저 잰다.
//!
//! ⚠ **질의마다 다른 값이라는 설계는 안 건드린다** — `query_envelope.rs` 의
//! `범위는_질의마다_다른_값이다` 가 그것을 잰다. 두 명령이 예외인 까닭은 **만지는 범위가
//! 그래프 전체**이기 때문이고, 그 범위의 질의가 `graph.dump` 다.

mod common;

use common::{git, pal};
use std::path::PathBuf;

/// 못 푼 참조가 있고 최저 등급이 `l0` 이 아닌 저장소.
///
/// | 파일 | 무엇을 심었나 |
/// |---|---|
/// | `a.ts` | `console` 을 부른다 — 파일 안에서 못 푸는 참조 |
/// | `src/lib.rs` | 없는 이름을 부른다 — 언어가 둘이어도 합이 한 규칙으로 서는지 |
fn 저장소(tag: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("pal-coverage-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join("src")).expect("임시 저장소");
    let w = |p: &str, s: &str| std::fs::write(root.join(p), s).expect("쓰기");
    w("a.ts",
      "export function 도움() { return console }\n\
       export function 부름() { return 도움() }\n");
    w("src/lib.rs", "pub fn 러스트() { 없는것(); }\n");
    git(&root, &["init", "-q", "."]);
    git(&root, &["add", "-A"]);
    git(&root, &["-c", "user.email=t@example.com", "-c", "user.name=t", "commit", "-qm", "첫"]);
    root
}

fn json(s: &str) -> serde_json::Value {
    serde_json::from_str(s).expect("응답 묶음 JSON")
}

#[test]
fn 전_그래프_명령의_범위가_graph_dump_와_같은_값이다() {
    let repo = 저장소("whole");

    // 2 층을 먼저 세운다 — `export` 는 읽기 전용으로 붙는다.
    let dump = json(&pal(&repo, &["query", "graph.dump", "--json"]));
    let 미해소 = &dump["coverage"]["unresolved"];
    let 최저 = &dump["coverage"]["lowest_grade"];

    // ── A1-a — **전제.** 하드코딩과 같은 값이면 아래 대조가 아무것도 안 잰다.
    assert!(
        미해소.as_u64().expect("unresolved") >= 1,
        "픽스처의 graph.dump 미해소가 0 이다 — 하드코딩과 같아 이 시험이 공짜로 통과한다: {dump:#}"
    );
    assert_ne!(
        최저.as_str().expect("lowest_grade"),
        "l0",
        "픽스처의 graph.dump 최저 등급이 l0 이다 — 하드코딩과 같아 이 시험이 공짜로 통과한다"
    );

    // ── A1 — `pal doctor --json`.
    let doctor = json(&pal(&repo, &["doctor", "--json"]));
    assert_eq!(&doctor["coverage"]["unresolved"], 미해소, "doctor 의 미해소가 graph.dump 와 다르다");
    assert_eq!(&doctor["coverage"]["lowest_grade"], 최저, "doctor 의 최저 등급이 graph.dump 와 다르다");

    // ── A1 — `pal export` 의 응답 묶음. `--json` 은 `--out` 과 함께만 쓸 수 있다.
    let out = repo.join("out.cypher");
    let export = json(&pal(
        &repo,
        &["export", "--format", "cypher", "--out", out.to_str().expect("경로"), "--json"],
    ));
    assert_eq!(&export["coverage"]["unresolved"], 미해소, "export 의 미해소가 graph.dump 와 다르다");
    assert_eq!(&export["coverage"]["lowest_grade"], 최저, "export 의 최저 등급이 graph.dump 와 다르다");

    let _ = std::fs::remove_dir_all(&repo);
}
