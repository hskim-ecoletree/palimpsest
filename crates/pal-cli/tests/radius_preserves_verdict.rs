//! **`pal radius` 가 반경만 바꾸고 낡음 판정을 보존한다** — 조건 `A1`·`A3`·`A4`.
//!
//! # 이 시험이 존재하는 이유
//!
//! 반경을 바꾸는 유일한 길이 `pal bind` 재호출이었고 그것은 `bound_at` 과 감시
//! 다이제스트를 **HEAD 로 재기준한다.** 격리 실측에서 `stale 7 → 6` 이었다 —
//! **반경을 넓히는 일이 원리상 데이터 손실**이었다.
//!
//! # 픽스처가 이 시험을 가능하게 하는 자리
//!
//! `common::저장소` 의 `delta.ts` 가 `부름() → 도움()` 을 담는다. 그래서
//! `도움` 에 결박을 걸고 `callers` 로 넓히면 감시 집합에 **`부름` 이 새로 든다** —
//! 그것이 이 회차가 재려는 바로 그 사건이다.

mod common;

use common::{PAL, git, pal, 저장소};

/// 정본 JSONL 에서 결박 한 행.
fn 결박(repo: &std::path::Path) -> serde_json::Value {
    let out = pal(repo, &["intent", "export"]);
    for line in out.lines() {
        let v: serde_json::Value = match serde_json::from_str(line) {
            Ok(v) => v,
            Err(_) => continue,
        };
        if v["kind"] == "binding" {
            return v;
        }
    }
    panic!("정본 JSONL 에 결박이 없다:\n{out}");
}

fn 판정(repo: &std::path::Path) -> serde_json::Value {
    let out = pal(repo, &["query", "binding.status", "--json"]);
    let v: serde_json::Value = serde_json::from_str(&out).expect("응답 묶음 JSON");
    v["answer"]["bindings"][0].clone()
}

/// `(symbol, digest)` 쌍 집합.
fn 감시(b: &serde_json::Value) -> std::collections::BTreeSet<(String, String)> {
    b["watch"]
        .as_array()
        .expect("watch")
        .iter()
        .map(|w| {
            (
                w["symbol"].as_str().expect("symbol").to_owned(),
                w["digest"].as_str().expect("digest").to_owned(),
            )
        })
        .collect()
}

/// ★ **조건 `A1`** — 반경을 바꿔도 여섯이 보존된다.
///
/// 다섯은 바이트로 같고, 여섯째 — **넓히기 전 감시 집합의 `(symbol, digest)` 쌍 전부** —
/// 는 넓힌 뒤 집합의 **부분집합**이다.
#[test]
fn 반경만_바뀌고_여섯이_보존된다() {
    let repo = 저장소("rebind-preserve");
    pal(&repo, &["bind", "도움", "--note", "이 함수의 계약"]);

    let 전 = 결박(&repo);
    let id = 전["id"].as_str().expect("id").to_owned();
    assert_eq!(전["radius"], "symbol", "착수 반경이 symbol 이 아니면 이 시험이 아무것도 안 잰다");
    let 감시_전 = 감시(&전);
    assert_eq!(감시_전.len(), 1, "symbol 반경의 감시 집합은 대상 하나다");

    pal(&repo, &["radius", &id, "--to", "callers"]);

    let 후 = 결박(&repo);

    // ── 다섯은 바이트로 같다 ──────────────────────────────────────────────
    for 칸 in ["id", "subject", "note", "bound_at", "bound_at_time", "promoted_by"] {
        assert_eq!(전[칸], 후[칸], "`{칸}` 이 바뀌었다 — 보존 경로가 아니다");
    }

    // ── 반경은 바뀌었다 ───────────────────────────────────────────────────
    assert_eq!(후["radius"], "callers", "반경이 안 바뀌었다 — 이 시험이 아무것도 안 잰다");

    // ── 여섯째 — 옛 쌍 전부가 부분집합이다 ────────────────────────────────
    let 감시_후 = 감시(&후);
    assert!(
        감시_전.is_subset(&감시_후),
        "옛 감시 원소의 (symbol, digest) 쌍이 보존되지 않았다\n전: {감시_전:?}\n후: {감시_후:?}"
    );
    // **감시 집합이 실제로 커졌다** — 안 커지면 위 부분집합이 공짜로 참이다.
    assert!(
        감시_후.len() > 감시_전.len(),
        "감시 집합이 안 커졌다 ({} → {}) — `부름() → 도움()` 엣지가 안 섰다는 뜻이고, \
         그러면 이 시험의 부분집합 주장이 공짜로 통과한다",
        감시_전.len(),
        감시_후.len()
    );

    let _ = std::fs::remove_dir_all(&repo);
}

/// ★★ **조건 `A3`** — 새 감시 원소의 기준 시점이 **`bound_at` 의 base 커밋**이다.
///
/// 결박한 **뒤에** 호출자를 고치고 넓히면, 새 원소의 기준값이 base 커밋의 것이므로
/// 판정이 **`stale` 이 되고 `triggered_by` 에 그 호출자가 실린다.**
///
/// # 왜 이것이 이 회차의 핵심인가
///
/// HEAD 에서 읽으면 새 원소가 **반드시 `fresh`** 이고, 그러면 *"결정 뒤에 호출자가
/// 변했나"* 를 **영구히 못 묻게 된다.** 이 시험이 그 갈래를 붙든다.
#[test]
fn 새_감시_원소의_기준은_bound_at_의_base_커밋이다() {
    let repo = 저장소("rebind-baseline");
    pal(&repo, &["bind", "도움", "--note", "이 함수의 계약"]);

    let 전 = 결박(&repo);
    let id = 전["id"].as_str().expect("id").to_owned();
    let 판정_전 = 판정(&repo);
    assert_eq!(
        판정_전["status"]["code"]["freshness"], "fresh",
        "넓히기 전에 이미 fresh 가 아니면 이 시험이 무엇을 재는지 갈린다"
    );

    // ── 결박한 **뒤에** 호출자를 고친다 ───────────────────────────────────
    std::fs::write(
        repo.join("delta.ts"),
        "export function 도움() { return 1 }\nexport function 부름() { return 도움() + 2 }\n",
    )
    .expect("delta.ts");
    git(&repo, &["add", "-A"]);
    git(&repo, &["-c", "user.email=t@example.com", "-c", "user.name=t", "commit", "-qm", "호출자를 고친다"]);

    // ── 넓힌다 ────────────────────────────────────────────────────────────
    pal(&repo, &["radius", &id, "--to", "callers"]);

    let 판정_후 = 판정(&repo);
    assert_eq!(
        판정_후["status"]["code"]["freshness"], "stale",
        "넓혔는데 stale 이 아니다 — 새 감시 원소의 기준을 HEAD 에서 읽었다는 뜻이고, \
         그러면 「결정 뒤에 호출자가 변했나」를 원리상 못 묻는다\n{판정_후}"
    );
    let 켠_것 = 판정_후["status"]["code"]["triggered_by"].as_array().expect("triggered_by");
    assert_eq!(켠_것.len(), 1, "켠 것이 하나가 아니다: {판정_후}");
    // **대상이 아니라 호출자가 켰다.** 대상(`도움`)의 본문은 안 고쳤다.
    assert_ne!(
        켠_것[0], 후_대상(&repo),
        "켠 것이 대상 자신이다 — 호출자가 아니라 대상이 변했다는 뜻이고 이 시험이 다른 것을 잰다"
    );

    let _ = std::fs::remove_dir_all(&repo);
}

fn 후_대상(repo: &std::path::Path) -> serde_json::Value {
    결박(repo)["target"].clone()
}

/// ★★ **조건 `A1-a`·`A2-a` 의 음성 대조** — `pal bind` 재호출은 여섯을 **안** 보존한다.
///
/// # 이 시험이 없으면 `A1`·`A2` 가 아무것도 안 잰다
///
/// 위 두 시험이 *"여섯이 보존됐다"* 를 재는데, **두 경로가 어차피 같다면** 그 통과는
/// 공짜다. 이 시험이 *"다른 경로로 하면 정말 깨진다"* 를 붙들어 그 가능성을 없앤다.
///
/// # 무엇이 깨지는지 둘을 함께 잰다
///
///   · **`A1-a`** — `bound_at` 이 HEAD 로 재기준된다(다섯 중 하나가 바뀐다)
///   · **`A2-a`** — 그래서 **`stale` 이 사라진다.** 격리 실측의 `stale 7 → 6` 이 그 형태다
#[test]
fn pal_bind_재호출은_여섯을_안_보존하고_stale_을_지운다() {
    let repo = 저장소("radius-negative");
    pal(&repo, &["bind", "도움", "--note", "이 함수의 계약"]);

    let 전 = 결박(&repo);
    let 감시_전 = 감시(&전);

    // ── 대상을 고쳐서 `stale` 을 만든다 ───────────────────────────────────
    std::fs::write(
        repo.join("delta.ts"),
        "export function 도움() { return 99 }\nexport function 부름() { return 도움() }\n",
    )
    .expect("delta.ts");
    git(&repo, &["add", "-A"]);
    git(&repo, &["-c", "user.email=t@example.com", "-c", "user.name=t", "commit", "-qm", "대상을 고친다"]);

    let 판정_전 = 판정(&repo);
    assert_eq!(
        판정_전["status"]["code"]["freshness"], "stale",
        "고쳤는데 stale 이 아니면 이 음성 대조가 아무것도 안 잰다"
    );

    // ── `pal bind` 를 **같은 이름·같은 조각으로 다시** 부른다 ─────────────
    //
    // `id` 는 `(대상, 조각)` 에서 유도되므로 **같은 결박**이다. 그런데…
    pal(&repo, &["bind", "도움", "--note", "이 함수의 계약"]);

    let 후 = 결박(&repo);
    assert_eq!(전["id"], 후["id"], "id 가 달라지면 이 시험이 다른 결박을 보고 있다");

    // ── `A1-a` — 다섯 중 `bound_at` 이 HEAD 로 재기준됐다 ─────────────────
    assert_ne!(
        전["bound_at"], 후["bound_at"],
        "`pal bind` 재호출이 bound_at 을 그대로 뒀다 — 그러면 `A1` 이 두 경로를 못 가르고 \
         보존 경로를 세운 근거가 사라진다"
    );

    // ── `A2-a` — 옛 감시 원소의 digest 가 덮여 `stale` 이 사라졌다 ────────
    let 감시_후 = 감시(&후);
    assert!(
        !감시_전.is_subset(&감시_후),
        "`pal bind` 재호출이 옛 (symbol, digest) 쌍을 보존했다 — 그러면 `A1` 의 여섯째가 \
         아무것도 안 잰다\n전: {감시_전:?}\n후: {감시_후:?}"
    );
    assert_eq!(
        판정(&repo)["status"]["code"]["freshness"], "fresh",
        "★ `pal bind` 재호출 뒤에도 stale 이 남았다 — 격리 실측의 `stale 7 → 6` 이 \
         재현되지 않는다는 뜻이고, 그러면 이 회차가 보존 경로를 세운 근거가 거짓이다"
    );

    let _ = std::fs::remove_dir_all(&repo);
}

/// ★ **조건 `A4`** — 보존 경로는 결박을 **새로 만들지 못한다.**
#[test]
fn 없는_결박을_지목하면_실패한다() {
    let repo = 저장소("rebind-absent");
    pal(&repo, &["bind", "도움", "--note", "이 함수의 계약"]);
    let 전 = pal(&repo, &["intent", "export"]);

    let out = std::process::Command::new(PAL)
        .args(["radius", "0000000000000000", "--to", "callers"])
        .current_dir(&repo)
        .output()
        .expect("pal 을 못 돌렸다");
    assert!(
        !out.status.success(),
        "없는 결박을 지목했는데 종료값이 0 이다 — 이 명령이 결박을 만들 수 있다는 뜻이다\n{}",
        String::from_utf8_lossy(&out.stdout)
    );

    // **아무것도 안 더해졌다.** 종료값만 보면 「실패했는데 더했다」를 못 잡는다.
    assert_eq!(전, pal(&repo, &["intent", "export"]), "실패한 radius 가 정본을 바꿨다");

    let _ = std::fs::remove_dir_all(&repo);
}
