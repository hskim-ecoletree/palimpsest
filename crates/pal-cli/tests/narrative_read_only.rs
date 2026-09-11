//! **읽기 표면은 의도 저장소를 안 불린다** — [#129].
//!
//! `pal query narrative.unbound` 는 `IntentStore::open_read_only` 로 열면서도 인입이
//! 개체를 **민팅해서 남기려** 했고, 그래서 *"읽기로 연 의도 저장소에 쓰려 했다"* 로
//! **어떤 입력으로도 안 돌았다.** 광고된 질의 하나가 통째로 죽은 자리다.
//!
//! # 시험 둘이 서로를 받친다
//!
//! 첫째만 두면 **질의를 쓰기로 여는 고침**(한 줄)으로도 초록이 된다. 그 고침은
//! `query.rs` 가 스스로 적어 둔 계약(*"의도 저장소는 읽기로만 연다"*)을 깨고, **읽기가
//! `intent.redb` 를 불리게** 만든다. 둘째가 그 길을 막는다.
//!
//! [#129]: https://github.com/hskim-ecoletree/palimpsest/issues/129

mod common;

use common::{git, PAL};
use std::path::{Path, PathBuf};
use std::process::Command;

/// 문서 조각이 **실재하는** 저장소. 조각이 없으면 민팅할 것도 없어 이 시험이
/// 아무것도 안 잰다.
fn 저장소(tag: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("pal-129-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join("docs")).expect("임시 저장소");
    std::fs::write(root.join("alpha.ts"), "export function 도움() { return 1 }\n")
        .expect("alpha.ts");
    std::fs::write(
        root.join("docs/결정.md"),
        "# 결정 하나\n\n`도움` 을 남긴다. 이 문단이 조각이 된다.\n\n## 둘째 결정\n\n또 하나.\n",
    )
    .expect("결정.md");
    git(&root, &["init", "-q", "."]);
    git(&root, &["add", "-A"]);
    git(&root, &["-c", "user.email=t@example.com", "-c", "user.name=t", "commit", "-qm", "첫"]);
    root
}

fn 돌린다(cwd: &Path, args: &[&str]) -> (bool, String, String) {
    let out = Command::new(PAL).args(args).current_dir(cwd).output().expect("pal 을 못 돌렸다");
    (
        out.status.success(),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

/// ★ **RED 는 이것이었다** — `rc=1` · *"읽기로 연 의도 저장소에 쓰려 했다"*.
#[test]
fn narrative_unbound_는_읽기로도_돈다() {
    let repo = 저장소("runs");
    let (ok, out, err) = 돌린다(&repo, &["query", "narrative.unbound"]);
    assert!(ok, "질의가 실패했다\nstdout: {out}\nstderr: {err}");
    assert!(
        !err.contains("읽기로 연 의도 저장소에 쓰려 했다"),
        "#129 의 그 오류가 그대로다: {err}"
    );
    // **답이 자기가 뺀 것을 진다** — 0 이어도 줄이 있어야 한다.
    assert!(
        out.contains("이름이 아직 없어 뺀 조각"),
        "뺀 조각 수가 답에 없다 — 목록이 조용히 짧아진다\n{out}"
    );
    assert!(out.contains("미결박"), "미결박 줄이 없다\n{out}");
    let _ = std::fs::remove_dir_all(&repo);
}

/// ★★ **이 시험이 「쓰기로 열면 된다」는 길을 막는다.**
///
/// 질의를 돌려도 의도 저장소가 **생기지 않아야** 한다. 생기면 읽기가 쓰기가 된 것이고,
/// `query.rs` 가 그 자리에 적어 둔 계약이 거짓이 된다.
#[test]
fn 읽기_경로는_의도_저장소를_안_불린다() {
    let repo = 저장소("readonly");
    let 의도 = repo.join(".palimpsest/intent.redb");
    assert!(!의도.exists(), "시작 상태가 이미 틀렸다");

    let (ok, _, err) = 돌린다(&repo, &["query", "narrative.unbound"]);
    assert!(ok, "질의가 실패했다: {err}");
    assert!(
        !의도.exists(),
        "질의가 의도 저장소를 **만들었다** — 읽기 표면이 쓰고 있다"
    );

    // 그리고 저장소가 **있을 때**도 바이트가 안 움직여야 한다.
    // `pal narrative` 로 한 번 세우고(그쪽은 쓰기 표면이 맞다) 그 뒤 질의를 돌린다.
    let (세웠나, _, err2) = 돌린다(&repo, &["narrative"]);
    assert!(세웠나, "`pal narrative` 가 실패했다: {err2}");
    assert!(의도.exists(), "쓰기 표면이 저장소를 안 세웠다");
    let 전 = std::fs::read(&의도).expect("읽기");

    let (ok2, _, err3) = 돌린다(&repo, &["query", "narrative.unbound"]);
    assert!(ok2, "둘째 질의가 실패했다: {err3}");
    let 후 = std::fs::read(&의도).expect("읽기");
    assert_eq!(전.len(), 후.len(), "질의 뒤 의도 저장소의 크기가 움직였다");
    assert!(전 == 후, "질의 뒤 의도 저장소의 바이트가 움직였다");

    let _ = std::fs::remove_dir_all(&repo);
}
