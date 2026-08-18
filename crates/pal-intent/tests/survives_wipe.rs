//! **파생층을 지워도 사람의 노동이 남는가** — [R-21] · 옛 F03 §7 · `[f03.3.pass]` ①.
//!
//! # 빈 방에 자물쇠를 걸지 않는다
//!
//! S3 게이트가 결박에 대해 이것을 시험했고 `cargo xtask check` 의 「의도 저장소 폐기
//! 경로 부재」가 **정적으로** 지킨다. 그런데 F03 이 그 방에 **별칭**이라는 새 항목을
//! 넣었고, 정적 검사는 *"`pal-store` 가 그 경로를 언급하지 않는다"* 까지만 말한다.
//!
//! **여기서는 실제로 넣고, 파생층을 지우고, 남아 있는지 본다.**
//!
//! [R-21]: ../../../docs/plan/00-risks.md#r-21

use std::path::PathBuf;

use pal_core::{RepoAlias, RepoId};
use pal_intent::IntentStore;

/// `<임시>/파생/`(지울 것)과 `<임시>/의도/`(안 지울 것)를 만든다.
fn 방(tag: &str) -> (PathBuf, PathBuf, PathBuf) {
    let root = std::env::temp_dir().join(format!("pal-intent-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    let derived = root.join("derived");
    let intent = root.join("intent");
    std::fs::create_dir_all(&derived).expect("파생층");
    std::fs::create_dir_all(&intent).expect("의도 저장소");
    (root, derived, intent.join("intent.redb"))
}

#[test]
fn 별칭은_파생층을_지워도_남는다() {
    let (root, derived, intent_file) = 방("alias");

    // 파생층에 무언가 있다고 치고 — 재구축하면 지워지는 것들이다.
    std::fs::write(derived.join("index.redb"), b"2 layer").expect("파생 쓰기");

    let store = IntentStore::open(&intent_file).expect("의도 저장소");
    let alias = RepoAlias::new(RepoId::new("order-svc"), RepoId::new("order"), "저장소를 나눴다");
    store.record_alias(&alias).expect("별칭");
    drop(store);

    // **파생층을 통째로 지운다** — `pal doctor --rebuild` 가 하는 일이다.
    std::fs::remove_dir_all(&derived).expect("파생층 삭제");

    let store = IntentStore::open(&intent_file).expect("의도 저장소 재개");
    let 남은 = store.aliases().expect("읽기");
    assert_eq!(남은, vec![alias.clone()], "파생층을 지웠더니 사람의 선언이 사라졌다");
    assert_eq!(
        store.resolve_repo(&RepoId::new("order-svc")).expect("해소"),
        RepoId::new("order")
    );

    // **★ 반대 방향** — 선언되지 않은 이름은 그대로다. 자동으로 흡수하지 않는다.
    assert_eq!(
        store.resolve_repo(&RepoId::new("모르는-저장소")).expect("해소"),
        RepoId::new("모르는-저장소"),
        "선언 없이 재배치를 흡수했다 — 그것이 옛 F03 §5 가 기각한 자동 재결박이다"
    );

    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn 별칭_사슬이_한_바퀴_돌아도_멈춘다() {
    // `a → b → a` 는 사람이 만들 수 있는 선언이다. 여기서 고칠 수 없지만
    // **좌표를 만드는 쪽이 멈추지 않는 것**은 이 코드의 책임이다.
    let (root, _derived, intent_file) = 방("cycle");
    let store = IntentStore::open(&intent_file).expect("의도 저장소");
    store.record_alias(&RepoAlias::new(RepoId::new("a"), RepoId::new("b"), "")).expect("a→b");
    store.record_alias(&RepoAlias::new(RepoId::new("b"), RepoId::new("a"), "")).expect("b→a");
    let got = store.resolve_repo(&RepoId::new("a")).expect("해소");
    assert!(got == RepoId::new("a") || got == RepoId::new("b"), "돌지 않고 하나를 냈다");
    let _ = std::fs::remove_dir_all(&root);
}
