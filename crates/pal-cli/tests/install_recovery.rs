//! **죽은 자리에서 다시 설 수 있는가** — `install.rs` 머리말이 약속한 것의 뒷면.
//!
//! # 그 약속
//!
//! > *"기록이 앞서 있으면 죽은 자리와 무관하게 `uninstall` 이 걷어낼 수 있다."*
//!
//! 관측된 실패 트리거 넷 중 하나가 `SIGKILL` 이고, 그때 되감을 코드는 돌 기회가 없다.
//! 그래서 기록이 걸음마다 앞선다 — **그런데 자기 잠금이 그 문장을 막았다.** `SIGKILL`
//! 8지점 **전부**에서 `.claude/.pal.lock` 이 남았고, 그 뒤 `install`·`uninstall` 이
//! 20초를 기다린 뒤 rc=1 을 냈다. 걷어낼 수 있다던 것을 **잠금이 못 걷게 했다.**

mod common;

use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::time::{Duration, Instant};

use common::{PAL, git};

fn 프로젝트(tag: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("pal-f24-회복-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("방");
    std::fs::write(root.join("README.md"), "hello\n").expect("README");
    git(&root, &["init", "-q", "."]);
    root
}

fn 돌린다(cwd: &Path, args: &[&str]) -> Output {
    let path = std::env::var("PATH").unwrap_or_default();
    let pal_dir = Path::new(PAL).parent().expect("pal 의 부모");
    Command::new(PAL)
        .args(args)
        .current_dir(cwd)
        .env("PATH", format!("{}:{path}", pal_dir.display()))
        .output()
        .expect("pal 을 못 돌렸다")
}

fn 성공(cwd: &Path, args: &[&str]) -> String {
    let out = 돌린다(cwd, args);
    assert!(
        out.status.success(),
        "pal {args:?}\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8_lossy(&out.stdout).into_owned()
}

fn 잠금_자리(root: &Path) -> PathBuf {
    root.join(".claude/.pal.lock")
}

/// **죽은 프로세스가 남기고 간 잠금**을 흉내 낸다.
///
/// ⚠ `SIGKILL` 을 실제로 보내는 대신 **잔해를 직접 놓는다.** 살아 있는 프로세스를
/// 잠금 안에서 멈춰 세울 확실한 자리가 없어서다(설치는 밀리초 단위로 끝난다).
/// 대신 **두 형태를 다 놓는다** — 옛 빌드가 남기던 **디렉터리**와 지금 빌드가 남기는
/// **파일**. 어느 쪽도 다음 실행을 막으면 안 된다.
fn 죽은_잠금을_남긴다(root: &Path, 디렉터리로: bool) {
    let p = 잠금_자리(root);
    let _ = std::fs::remove_file(&p);
    let _ = std::fs::remove_dir_all(&p);
    if 디렉터리로 {
        std::fs::create_dir(&p).expect("잠금 디렉터리");
    } else {
        std::fs::write(&p, b"").expect("잠금 파일");
    }
}

/// ★ **죽은 잠금은 스스로 걷는다.** 세 경로 전부에서.
///
/// 관측(고치기 전): `SIGKILL` 8지점 전부에서 잠금이 남았고 그 뒤 `install`·`uninstall`
/// 이 **20초 대기 후 rc=1**. 사용자에게 남는 길은 그 자리를 손으로 지우는 것뿐이었다.
#[test]
fn 죽은_잠금은_다음_실행을_안_막는다() {
    for 디렉터리로 in [true, false] {
        let tag = if 디렉터리로 { "디렉터리" } else { "파일" };
        let root = 프로젝트(&format!("죽은잠금-{tag}"));
        성공(&root, &["install"]);

        for args in [&["install"][..], &["update"][..], &["uninstall"][..]] {
            죽은_잠금을_남긴다(&root, 디렉터리로);
            let t0 = Instant::now();
            성공(&root, args);
            assert!(
                t0.elapsed() < Duration::from_secs(10),
                "{tag} · pal {args:?}: 죽은 잠금을 기다렸다 ({:?})",
                t0.elapsed()
            );
        }

        // ★ **제거가 끝나면 잠금도 안 남는다** — 남으면 `.claude/` 를 못 비운다.
        assert!(!잠금_자리(&root).exists(), "{tag}: 제거 뒤에 잠금이 남았다");
        assert!(!root.join(".claude").exists(), "{tag}: `.claude/` 가 남았다");
    }
}

/// ★ **`update` 도 잠긴 프로젝트를 본다.**
///
/// 관측(고치기 전): `update` **만** rc=0 을 **0초**에 냈다 — 반쯤 설치되고 잠긴
/// 프로젝트에 *"이미 최신"* 이라고 답했다. 잠금을 판정 **뒤에** 잡았기 때문이다.
#[test]
fn 갱신이_잠긴_프로젝트를_그냥_지나치지_않는다() {
    let root = 프로젝트("갱신-잠금");
    성공(&root, &["install"]);
    죽은_잠금을_남긴다(&root, true);

    let report = 성공(&root, &["update"]);
    assert!(
        !잠금_자리(&root).exists(),
        "잠금을 안 보고 지나쳤다 — 죽은 잠금이 그대로 남았다:\n{report}"
    );
}

/// ★ **산 잠금은 안 걷는다.** 죽은 것을 걷는 문이 산 것도 걷으면 동시 실행이 깨진다.
///
/// 이 시험이 잠금의 **모양**을 안다 — `.claude/.pal.lock` 파일에 걸린 권고 잠금.
/// 그것이 이 회차가 고른 설계이고, 고르면 재야 한다.
#[test]
fn 산_잠금은_기다린다() {
    let root = 프로젝트("산잠금");
    성공(&root, &["install"]);

    // 이 시험 프로세스가 **살아서** 잠금을 쥔다.
    let held = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(false)
        .open(잠금_자리(&root))
        .expect("잠금 파일");
    held.try_lock().expect("이 시험이 잠금을 못 쥐었다");

    let path = std::env::var("PATH").unwrap_or_default();
    let pal_dir = Path::new(PAL).parent().expect("pal 의 부모");
    let mut child = Command::new(PAL)
        .args(["install"])
        .current_dir(&root)
        .env("PATH", format!("{}:{path}", pal_dir.display()))
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("pal 을 못 돌렸다");

    // ① 우리가 쥐고 있는 동안에는 안 끝난다.
    std::thread::sleep(Duration::from_millis(3_000));
    assert!(
        child.try_wait().expect("try_wait").is_none(),
        "산 잠금을 걷고 들어왔다 — 동시 실행이 깨진다"
    );

    // ② 놓으면 들어와서 끝낸다.
    held.unlock().expect("unlock");
    drop(held);
    let out = child.wait_with_output().expect("wait");
    assert!(out.status.success(), "잠금을 놓았는데 못 들어왔다");
}
