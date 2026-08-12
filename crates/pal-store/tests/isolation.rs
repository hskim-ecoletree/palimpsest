//! 스냅샷 격리 — **재구축 중에 부분 갱신이 보이는 창이 있는가** (DESIGN §12.7 · U15-d).
//!
//! 합격선 정본은 `corpus/criteria.toml` `[f22.4].pass` — 100 회 반복에서 **0 회**여야 하고
//! **1 회라도 보이면 반증이다.** 그리고 그 경우 게이트 실패가 아니라 **저장 계약의
//! 실패**이며 F05 착수 전에 격리 모델을 다시 본다.
//!
//! # 왜 여기 있고 `pal-core` 에 없는가
//!
//! 이 검사가 재는 것은 도메인 규칙이 아니라 **`redb` 의 MVCC** 다. F22 §3.4 가
//! *"새로 만들 것이 없고 쓰지 않으면 안 되는 것"* 이라 적은 그것이고, 쓰지 않으면
//! 이 자리가 검사된 적 없는 계약으로 남는다.
//!
//! # 어떻게 찢어진 상태를 볼 수 있게 만드는가
//!
//! `count()` 와 `resolve_name()` 을 따로 부르면 그 사이에 갱신이 끼어들 수 있고, 그것은
//! **찢어진 것이 아니라 서로 다른 두 스냅샷**이다. 그 차이를 구별하려면 **한 읽기
//! 트랜잭션 안에서 여러 값을 봐야** 한다.
//!
//! [`Projection::resolve_name`] 이 그렇게 되어 있다 — `BY_NAME` 과 `SYMBOL` 을 **한
//! 트랜잭션에서** 읽는다. 그래서 심볼 전부에 같은 이름을 주면 호출 한 번이 심볼 N 개를
//! 한 스냅샷에서 돌려주고, **세대 표식(`body`)이 섞여 있으면 그것이 곧 찢어진 상태다.**
//!
//! ```text
//! 세대 A   같은_이름 × N,  body = h("A")
//! 세대 B   같은_이름 × N,  body = h("B")     ← 같은 id, 다른 body
//! ```
//!
//! # 음성 대조 — **두 세대를 다 봐야 이 검사가 의미를 갖는다**
//!
//! 쓰기가 실제로 진행되는 동안 읽지 않았다면 0/100 은 아무것도 말하지 않는다. 그래서
//! 100 회 안에 **두 세대가 모두 관측됐는지**를 함께 요구한다. 그것이 이 시험의
//! *"검사가 고장 났다면 어떻게 드러나는가"* 다.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use pal_core::{
    BodyDigest, Discriminator, IdentityGrade, RepoId, RepoPath, Span, SymbolId, SymbolKind,
    SymbolNode,
};
use pal_store::Projection;

/// 한 세대의 심볼 수. **쓰기가 잴 수 있을 만큼 걸려야 읽기가 그 안에 들어갈 수 있다.**
const N: usize = 2_000;
/// 합격선이 등록한 반복 수.
const TRIALS: usize = 100;
/// 심볼 전부가 지는 이름 — 한 트랜잭션에서 N 개를 돌려받기 위한 장치.
const SAME: &str = "같은_이름";

fn 세대(mark: &str) -> Vec<SymbolNode> {
    let repo = RepoId::new("r");
    let body = BodyDigest::of_normalized(mark.as_bytes());
    (0..N)
        .map(|i| {
            let path = RepoPath::new(format!("p{i}.kt"));
            let id = SymbolId::compute(
                &repo,
                &path,
                &[],
                SAME,
                &Discriminator::new(SymbolKind::Function, 0),
            );
            SymbolNode {
                id,
                path,
                name: SAME.to_owned(),
                kind: SymbolKind::Function,
                body,
                span: Span { byte_start: 0, byte_end: 1, line_start: 1, line_end: 1 },
                identity: IdentityGrade::Exact,
            }
        })
        .collect()
}

#[test]
fn 재구축_중의_질의는_부분_갱신을_보지_않는다() {
    let dir = std::env::temp_dir().join(format!("pal-isolation-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let path = dir.join("index.redb");
    let projection = Arc::new(Projection::open(&path).expect("2층을 열지 못했다"));

    let a = 세대("A");
    let b = 세대("B");
    let a_body = a[0].body;
    let b_body = b[0].body;
    assert_ne!(a_body, b_body, "두 세대의 표식이 같으면 이 시험은 아무것도 재지 않는다");

    // 첫 세대를 깔아 둔다. 읽기가 빈 인덱스를 보는 것은 격리와 무관하다.
    projection.rebuild(&a).expect("첫 재구축");

    let done = Arc::new(AtomicBool::new(false));
    let rebuilds = Arc::new(AtomicUsize::new(0));

    let writer = {
        let p = Arc::clone(&projection);
        let done = Arc::clone(&done);
        let rebuilds = Arc::clone(&rebuilds);
        std::thread::spawn(move || {
            let mut turn = 0usize;
            while !done.load(Ordering::Relaxed) {
                let 세대 = if turn % 2 == 0 { &b } else { &a };
                p.rebuild(세대).expect("재구축");
                rebuilds.fetch_add(1, Ordering::Relaxed);
                turn += 1;
            }
        })
    };

    let mut sightings: Vec<String> = Vec::new();
    let mut seen_a = 0usize;
    let mut seen_b = 0usize;

    for trial in 0..TRIALS {
        // **한 트랜잭션.** 여기서 돌아온 N 개는 같은 스냅샷의 것이어야 한다.
        let got = projection.resolve_name(SAME).expect("2층을 읽지 못했다");

        if got.len() != N {
            sightings.push(format!("{trial}: 심볼이 {}개다 (N={N})", got.len()));
            continue;
        }
        let first = got[0].body;
        if got.iter().any(|s| s.body != first) {
            let 섞임 = got.iter().filter(|s| s.body != first).count();
            sightings.push(format!("{trial}: 한 답 안에 두 세대가 섞였다 — {섞임}개"));
            continue;
        }
        if first == a_body {
            seen_a += 1;
        } else if first == b_body {
            seen_b += 1;
        } else {
            sightings.push(format!("{trial}: 모르는 세대 표식 {}", first.short()));
        }
    }

    done.store(true, Ordering::Relaxed);
    writer.join().expect("쓰기 스레드");

    let 쓰기 = rebuilds.load(Ordering::Relaxed);
    println!(
        "격리 {TRIALS} 회 · 부분 갱신 관측 {}회 · 세대 A {seen_a} · 세대 B {seen_b} · 재구축 {쓰기}회",
        sightings.len()
    );

    assert!(
        sightings.is_empty(),
        "부분 갱신이 보였다 — **저장 계약의 실패다**:\n  {}",
        sightings.join("\n  ")
    );
    // ── 음성 대조 ────────────────────────────────────────────────────────────
    assert!(쓰기 > 0, "재구축이 한 번도 안 돌았다 — 0/100 이 아무것도 말하지 않는다");
    assert!(
        seen_a > 0 && seen_b > 0,
        "100 회 안에 한 세대만 봤다 (A {seen_a} · B {seen_b}) — \
         읽기가 쓰기와 겹치지 않았고, 그러면 이 시험은 격리를 잰 것이 아니다"
    );

    let _ = std::fs::remove_dir_all(&dir);
}
