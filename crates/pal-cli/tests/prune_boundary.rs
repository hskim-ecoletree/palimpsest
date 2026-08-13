//! **캐시 폐기 격리** — `pal cache prune` 이 `cache/` 밖을 안 건드리는가. CI 상시.
//!
//! 합격선 정본은 `corpus/criteria.toml` `[f04.pass].prune_touches_only_cache` 이고
//! 판정은 `docs/gates/F04.md` 다.
//!
//! # 이 시험이 왜 따로 있어야 하는가
//!
//! `cargo xtask check` 에 「의도 저장소 폐기 경로 부재」가 이미 있다. **그러나 그것은
//! 문자열 스캔이라 *"`pal-store` 소스에 그 낱말이 없다"* 만 말한다.** 낱말이 없어도
//! 상위 디렉터리를 지우는 코드는 쓸 수 있고, `..` 하나면 경계가 사라진다.
//!
//! 게이트 셋이 이 시험을 기다리며 같은 문장을 적고 넘겼다 —
//! [S1](../../../docs/gates/S1-ledger.md) · [S3](../../../docs/gates/S3-intent.md) §140 ·
//! [F22-4](../../../docs/gates/F22-4-doctor.md): *"`pal cache prune` 이 생길 때
//! [R-21](../../../docs/plan/00-risks.md#r-21) 의 검사가 실제 하중을 진다."*
//! **셋이 여기서 만기가 된다.**
//!
//! # 재는 방식 — 이름이 아니라 전수
//!
//! `intent.redb` 만 대면 다음에 생기는 파일이 빠진다. **`cache/` 를 뺀 전부**를 떠서
//! 바이트로 댄다.

mod common;

use common::{바이트_전부, 저장소, 캐시_엔트리_수, pal};

#[test]
fn prune_은_cache_밖의_바이트를_하나도_안_바꾼다() {
    let repo = 저장소("prune");
    let pal_dir = repo.join(".palimpsest");

    // ① 파생층과 의도층을 실제로 채운다. **빈 방에 자물쇠를 걸지 않는다.**
    pal(&repo, &["ledger", ".", "--json"]);
    pal(&repo, &["bind", "알파", "--note", "이 클래스는 사람이 손으로 건 것이다"]);
    pal(&repo, &["bind", "베타", "--note", "둘째 결박"]);

    let 전 = 바이트_전부(&pal_dir, "cache");
    let 캐시_전 = 캐시_엔트리_수(&pal_dir.join("cache"));

    // **하한이다.** 셋 다 0 이면 이 시험은 아무것도 재지 않는다
    // (`[f04].self_judged` ④ — 시험되지 않은 대조는 `–` 가 아니라 실패다).
    assert!(캐시_전 >= 3, "캐시가 안 찼다 — 엔트리 {캐시_전}");
    assert!(전.len() >= 2, "지키려는 파일이 {} 개뿐이다", 전.len());
    assert!(
        전.keys().any(|k| k.contains("intent")),
        "의도 저장소가 없다 — 지켜야 할 것이 없으면 통과가 뜻이 없다: {:?}",
        전.keys().collect::<Vec<_>>()
    );

    // ② **예산 0.** 지울 수 있는 것은 전부 지우라는 뜻이고, 경계를 가장 세게 민다.
    let out = pal(&repo, &["cache", "prune", "--repo", ".", "--budget", "0"]);

    // ③ 판정 — 바이트로.
    let 후 = 바이트_전부(&pal_dir, "cache");
    assert_eq!(
        전.keys().collect::<Vec<_>>(),
        후.keys().collect::<Vec<_>>(),
        "`prune` 이 `cache/` 밖의 파일 목록을 바꿨다"
    );
    for (name, before) in &전 {
        assert_eq!(
            before,
            &후[name],
            "`prune` 이 `{name}` 의 바이트를 바꿨다 — **R-21 이 깨졌다**"
        );
    }

    // ④ **★ 그런데 실제로 지우기는 했는가.** 아무것도 안 지우는 `prune` 은 ③ 을
    //    공짜로 통과한다. 이 줄이 없으면 위 전부가 장식이다.
    assert_eq!(캐시_엔트리_수(&pal_dir.join("cache")), 0, "예산 0 인데 엔트리가 남았다\n{out}");

    // ⑤ 결박이 그대로다 — 바이트가 같으므로 참이지만, **읽어서도 확인한다.**
    //    바이트 비교는 *"파일이 안 변했다"* 이고 이쪽은 *"값이 살아 있다"* 다.
    let touch = pal(&repo, &["touch", "알파", "--json"]);
    assert!(touch.contains("이 클래스는 사람이 손으로 건 것이다"), "결박이 사라졌다: {touch}");

    let _ = std::fs::remove_dir_all(&repo);
}
