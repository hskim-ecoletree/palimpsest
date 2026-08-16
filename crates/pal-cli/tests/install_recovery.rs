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

// ─────────────────────────────────────────────────────────────────────────────
// 7. **순서에 기대지 않는다. 그리고 못 지웠으면 말한다**
// ─────────────────────────────────────────────────────────────────────────────

/// ★ **`created_dirs` 의 순서에 기대면 동시 설치 뒤에 뒤집힌다.**
///
/// `install` 이 옛 매니페스트에서 목록을 물려받은 **뒤에** `.claude` 를 `push` 하므로,
/// 다른 프로세스가 `.claude` 를 먼저 만든 회차에서는 순서가 뒤집힌다. 옛 제거는
/// 역순으로 돌면서 `.claude` 를 자식보다 **먼저** 지우려다 실패했고,
/// `if path.is_dir() && remove_dir(path).is_ok()` 가 **그 실패를 삼켰다** — rc=0,
/// 화면에 `.claude/` 없음, 디렉터리 남음.
#[test]
fn 제거는_깊은_것부터_지운다() {
    let root = 프로젝트("순서");
    성공(&root, &["install"]);
    순서를_뒤집는다(&root);

    let report = 성공(&root, &["uninstall"]);
    assert!(
        !root.join(".claude").exists(),
        "`.claude/` 가 남았다 — 순서에 기댔다:\n{report}"
    );
    assert!(report.contains(".claude/"), "지웠으면서 말하지 않았다:\n{report}");
}

/// 측정자가 확인한 결정론적 재현 — `.claude` 를 목록의 **끝**으로 옮긴다.
fn 순서를_뒤집는다(root: &Path) {
    let path = root.join(".claude/pal/manifest.json");
    let mut m: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&path).expect("읽기")).expect("JSON");
    let dirs = m["created_dirs"].as_array_mut().expect("created_dirs");
    let at = dirs.iter().position(|d| d == ".claude").expect("`.claude` 가 목록에 없다");
    let claude = dirs.remove(at);
    dirs.push(claude);
    std::fs::write(&path, serde_json::to_string_pretty(&m).expect("직렬화")).expect("쓰기");
}

/// ★ **못 지웠으면 말한다** — 게이트 ④ 의 *"밟지 않는 것과 말하지 않는 것은 다르다"*.
///
/// 남의 것이 들어와 있으면 그 자리는 이제 우리 것이 아니고 **안 지우는 것이 맞다**(⑥).
/// 그런데 안 지웠다는 사실을 안 말하면 사용자는 제거가 끝났다고 믿는다.
#[test]
fn 못_지운_디렉터리를_말한다() {
    let root = 프로젝트("남긴다");
    성공(&root, &["install"]);
    // 남의 파일이 우리가 만든 디렉터리에 들어왔다.
    std::fs::write(root.join(".claude/agents/남의것.md"), "남의 에이전트\n").expect("남의 것");

    let report = 성공(&root, &["uninstall"]);
    assert!(root.join(".claude/agents").is_dir(), "남의 것이 든 자리를 지웠다");
    assert!(
        report.contains("남겼다") && report.contains(".claude/agents"),
        "못 지웠는데 말하지 않았다:\n{report}"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// 8. **기록이 걸음마다 앞선다 — 제거 쪽에도**
// ─────────────────────────────────────────────────────────────────────────────

/// ★ **파일 루프 중간에서 실패한 `uninstall` 이 다시 돌 수 있어야 한다.**
///
/// 관측(고치기 전): 이미 `CLAUDE.md`·`.gitignore`·파일 몇 개를 지운 뒤 같은 항목에서
/// **매번 rc=1**. 매니페스트는 그대로 남고 오류 문구가 회복 방법을 안 줬다.
/// **「기록이 걸음마다 앞선다」가 `install` 에는 있고 `uninstall` 에는 없었다** —
/// 지운 것을 매니페스트에서 빼지 않았다.
///
/// 그리고 걸림돌을 치운 뒤에도 못 돌았다: 남은 것이 전부 이미 없으니 ⑥-b 의
/// *"하나도 못 찾았다"* 가 **자기가 지운 자리를 보고** 거짓 경보를 냈다.
#[test]
fn 중간에서_실패한_제거가_이어서_끝난다() {
    let root = 프로젝트("이어서");
    let s0 = 스냅샷(&root);
    성공(&root, &["install"]);

    // 걸림돌 — 페이로드 파일 하나가 **비어 있지 않은 디렉터리**가 됐다.
    let 걸림돌 = root.join(".claude/commands/pal/plan.md");
    std::fs::remove_file(&걸림돌).expect("지우기");
    std::fs::create_dir(&걸림돌).expect("걸림돌");
    std::fs::write(걸림돌.join("남의것"), "x\n").expect("안의 것");

    let out = 돌린다(&root, &["uninstall"]);
    assert!(!out.status.success(), "걸림돌을 보고도 성공을 냈다");

    // ① **진행한 만큼 기록이 줄었다.**
    let m: serde_json::Value =
        serde_json::from_slice(&std::fs::read(root.join(".claude/pal/manifest.json")).expect("읽기"))
            .expect("JSON");
    assert!(m["blocks"].as_array().expect("blocks").is_empty(), "걷은 블록이 기록에 남았다: {m}");
    let 남은: Vec<&str> =
        m["files"].as_array().expect("files").iter().map(|f| f["path"].as_str().expect("p")).collect();
    assert!(!남은.is_empty(), "이 시험이 재려는 상태가 아니다 — 남은 것이 없다");
    assert!(
        남은.contains(&".claude/commands/pal/plan.md") && !남은.contains(&".claude/pal/INSTRUCTIONS.md"),
        "지운 것이 기록에서 안 빠졌다: {남은:?}"
    );

    // ② 오류 문구가 **회복 방법**을 준다.
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("다시 돌리"), "회복 방법을 안 줬다:\n{stderr}");

    // ③ 걸림돌을 치우면 **이어서 끝낸다.**
    std::fs::remove_dir_all(&걸림돌).expect("걸림돌 치우기");
    성공(&root, &["uninstall"]);
    assert_eq!(스냅샷(&root), s0, "제거 후가 설치 전과 다르다");
}

/// 트리 전체의 `(상대 경로 → 길이·합)`. **`.git/` 은 뺀다.**
fn 스냅샷(root: &Path) -> std::collections::BTreeMap<String, String> {
    let mut out = std::collections::BTreeMap::new();
    훑기(root, root, &mut out);
    out
}

fn 훑기(root: &Path, dir: &Path, out: &mut std::collections::BTreeMap<String, String>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        let rel = path.strip_prefix(root).unwrap_or(&path).display().to_string();
        if rel.starts_with(".git/") || rel == ".git" {
            continue;
        }
        if path.is_dir() {
            out.insert(rel, "<디렉터리>".to_owned());
            훑기(root, &path, out);
        } else {
            let bytes = std::fs::read(&path).unwrap_or_default();
            out.insert(rel, format!("{}·{:x}", bytes.len(), 합(&bytes)));
        }
    }
}

fn 합(bytes: &[u8]) -> u64 {
    bytes.iter().fold(1_469_598_103_934_665_603_u64, |h, b| {
        (h ^ u64::from(*b)).wrapping_mul(1_099_511_628_211)
    })
}
