//! **최신인지 판단할 근거가 바이너리 안에 있는가** — `[f24]` ⑨.
//!
//! 게이트가 재는 문장은 *"서로 다른 두 빌드의 `--version` 이 갈리는가"* 다. 두 커밋에서
//! 실제로 빌드해 대는 것은 이 시험 하나가 할 수 없다 — 그래서 **가르는 성분이 실제로
//! 실렸는지**를 잰다. 커밋이 실려 있으면 두 커밋의 값은 정의상 갈리고, 안 실려 있으면
//! 정의상 같다. 성분이 갈리는지 자체는 `version.rs` 의 단위 시험이 진다.
//!
//! ⚠ **이 저장소가 git 저장소가 아니면 이 시험은 아무것도 안 잰다** — 그때는 조용히
//! 통과하는 대신 **건너뛴 사실을 표준출력에 적는다**(`cargo test -- --nocapture`).

use std::process::Command;

const PAL: &str = env!("CARGO_BIN_EXE_pal");

fn 저장소_루트() -> &'static str {
    env!("CARGO_MANIFEST_DIR")
}

#[test]
fn 버전에_커밋이_실려_있다() {
    let head = Command::new("git")
        .args(["rev-parse", "--short=12", "HEAD"])
        .current_dir(저장소_루트())
        .output();
    let Ok(out) = head else {
        println!("git 을 못 돌렸다 — 이 시험은 아무것도 재지 않았다");
        return;
    };
    if !out.status.success() {
        println!("git 저장소가 아니다 — 이 시험은 아무것도 재지 않았다");
        return;
    }
    let sha = String::from_utf8(out.stdout).expect("UTF-8").trim().to_owned();
    assert!(!sha.is_empty(), "`git rev-parse` 가 빈 값을 냈다");

    let v = Command::new(PAL).arg("--version").output().expect("pal --version");
    assert!(v.status.success(), "pal --version 이 실패했다");
    let printed = String::from_utf8(v.stdout).expect("UTF-8");

    assert!(
        printed.contains(&sha),
        "`pal --version` 에 커밋이 없다 — 두 빌드가 같은 값을 낸다\n  냈다: {}\n  HEAD: {sha}",
        printed.trim()
    );
}
