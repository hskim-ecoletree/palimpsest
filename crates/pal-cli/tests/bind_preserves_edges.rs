//! **`pal bind` 가 2층의 엣지를 지우지 않는다** (`[f09.1.pass].bind_preserves_edges`).
//!
//! # 이 시험이 존재하는 이유 — [F06 게이트](../../../docs/gates/F06.md) §6-가-2
//!
//! F06 이 핸드오프를 쓰면서 실측했다. 표면 넷 중 둘이 `Projection::rebuild` 를 불렀고
//! 그것은 **심볼만** 쓰고 `built_for` 를 **빈 문자열로** 갈아 끼운다:
//!
//! ```text
//! pal query graph.dump --json          → 노드 4578 · 엣지 4601 · built_for_this_snapshot true
//! pal bind recoverForRetry --note "…"
//! pal query graph.dump --read-only     → 노드 4578 · 엣지 **0** · built_for_this_snapshot **false**
//! ```
//!
//! (ditto @ `aded7ce7f88f` 실측. F06 은 노드 2개 픽스처에서 봤고 여기서 실 코퍼스 규모로
//! 재현됐다.)
//!
//! # 왜 F09 가 고치는가
//!
//! [`pal_core::Radius::Callers`] 가 **엣지를 요구한다.** `pal bind` 가 엣지를 지우는 채로
//! 반경을 세우면 **감시 집합이 조용히 빈다** — 그리고 **빈 감시 집합은 언제나 `Live`** 다.
//! 그러면 이 기능의 반대 방향 넷 중 셋이 **공짜로 통과한다.**
//!
//! # ⚠ `--read-only` 가 없으면 이 시험이 아무것도 안 잰다
//!
//! `pal query` 는 기본이 **쓰기**라서 스티칭을 다시 돌리고 **엣지를 복구해 버린다.**
//! F06 이 `--read-only` 를 만들었기 때문에 이 증상이 보인다 — 대조가 꺼지는 형태의
//! **다섯째**(도구가 무엇을 읽는지)가 정확히 이 자리다.
//!
//! # 하한 — **시험되지 않은 대조는 `–` 가 아니라 실패다** (`2e2eb3f`)
//!
//! `bind` **전**의 엣지가 0 이면 *"안 지웠다"* 가 공짜로 참이다. 그래서 전 회차의 엣지
//! 수에 하한을 박는다.

mod common;

use common::{pal, 저장소};

/// 봉투에서 `(노드 수, 엣지 수, built_for_this_snapshot)`.
fn 그래프(out: &str) -> (usize, usize, bool) {
    let v: serde_json::Value = serde_json::from_str(out).expect("봉투 JSON");
    (
        v["answer"]["nodes"].as_array().expect("nodes").len(),
        v["answer"]["edges"].as_array().expect("edges").len(),
        v["projection"]["built_for_this_snapshot"].as_bool().expect("built_for_this_snapshot"),
    )
}

#[test]
fn bind_는_엣지를_지우지_않는다() {
    let repo = 저장소("bind-edges");

    // ── 전 — **쓰기로 붙어 스티칭한다** ────────────────────────────────────
    let (노드_전, 엣지_전, 선_전) = 그래프(&pal(&repo, &["query", "graph.dump", "--json"]));

    // **하한.** `delta.ts` 의 `부름() → 도움()` 이 엣지 하나를 낸다. 0 이면 이 시험은
    // 아무것도 안 잰다.
    assert!(엣지_전 > 0, "결박 전에 엣지가 0 이다 — 이 시험이 아무것도 안 잰다");
    assert!(노드_전 > 0, "결박 전에 노드가 0 이다");
    assert!(선_전, "결박 전에 2층이 이 스냅샷 것이 아니다 — 스티칭이 안 돌았다");

    // ── 결박 ───────────────────────────────────────────────────────────────
    pal(&repo, &["bind", "도움", "--note", "이 함수의 계약"]);

    // ── 후 — **읽기 전용이어야 한다.** 쓰기로 물으면 `pal query` 가 스티칭을 다시
    //         돌려 엣지를 복구해 버리고, 그러면 이 시험은 언제나 통과한다.
    let (노드_후, 엣지_후, 선_후) =
        그래프(&pal(&repo, &["query", "graph.dump", "--read-only", "--json"]));

    assert_eq!(
        엣지_후, 엣지_전,
        "`pal bind` 가 엣지를 지웠다 ({엣지_전} → {엣지_후}) — `Projection::rebuild` 를 부르고 있다"
    );
    assert_eq!(노드_후, 노드_전, "`pal bind` 가 노드를 지웠다 ({노드_전} → {노드_후})");
    assert!(
        선_후,
        "`pal bind` 뒤에 `built_for_this_snapshot` 이 false 다 — `rebuild` 가 `built_for` 를 빈 문자열로 갈아 끼웠다"
    );

    let _ = std::fs::remove_dir_all(&repo);
}

/// **★ 반대 방향** — 이 시험이 무언가를 재고 있다는 증거.
///
/// `--read-only` 없이 물으면 `pal query` 가 스티칭을 다시 돌린다. 즉 **엣지가 지워진
/// 상태에서도 이 경로는 4,601 을 낸다.** 그래서 위 시험이 `--read-only` 를 쓰는 것이
/// 우연이 아니라 **요구**임을 여기서 붙든다.
#[test]
fn 쓰기로_물으면_스티칭이_다시_돌아_증상이_가려진다() {
    let repo = 저장소("bind-edges-masked");

    let (_, 엣지_전, _) = 그래프(&pal(&repo, &["query", "graph.dump", "--json"]));
    assert!(엣지_전 > 0, "엣지가 0 이면 이 시험이 아무것도 안 잰다");

    pal(&repo, &["bind", "도움", "--note", "이 함수의 계약"]);

    // **쓰기로 물으면 스티칭이 다시 돈다** — 그러므로 이 값은 `bind` 가 무엇을 했든
    // 같다. 이 사실이 참이어야 `--read-only` 가 필요한 이유가 성립한다.
    let (_, 엣지_쓰기, 선_쓰기) = 그래프(&pal(&repo, &["query", "graph.dump", "--json"]));
    assert_eq!(엣지_쓰기, 엣지_전, "쓰기 경로가 스티칭을 다시 안 돌렸다");
    assert!(선_쓰기, "쓰기 경로 뒤에도 2층이 이 스냅샷 것이 아니다");

    let _ = std::fs::remove_dir_all(&repo);
}
