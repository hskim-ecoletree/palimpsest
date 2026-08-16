//! ★ **대상 프로젝트의 파일은 전부 남이 쓴 것이다** — `[f24]` ⑥⑦ 의 뒷면.
//!
//! # 왜 이 파일이 따로 서는가
//!
//! `.claude/pal/manifest.json` 과 `.claude/settings.json` 은 **대상 프로젝트 안의 평범한
//! 파일**이고 `.gitignore` 에 없어서 **커밋되고 clone 과 함께 이동한다.** 그래서 그 둘의
//! 내용은 **입력이지 사실이 아니다.** 서명도 소유 확인도 없다.
//!
//! | 재는 것 | 왜 |
//! |---|---|
//! | 저장소에서 읽은 문자열을 **실행하지 않는다** | 임의 코드 실행. `pal doctor` 한 번이 남의 문자열을 셸에 넘겼다 |
//! | **하드링크**로 대상 밖이 안 샌다 | 심링크는 `canonicalize` 가 막지만 하드링크는 「밖」이라는 신원이 없다 |
//! | 매니페스트가 대상 **안**의 아무 파일이나 못 지운다 | 악성 PR 하나 + `pal uninstall` 한 번 |
//! | FIFO 에서 **안 매달린다** | writer 없는 FIFO 는 `fs::read` 를 영원히 잡는다 |

mod common;

use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

use common::{PAL, git, path_앞에};

// ─────────────────────────────────────────────────────────────────────────────
// 방 — `밖/` 과 `안/` 이 형제로 산다
// ─────────────────────────────────────────────────────────────────────────────

struct 방 {
    base: PathBuf,
    밖: PathBuf,
    안: PathBuf,
}

fn 방(tag: &str) -> 방 {
    let base = std::env::temp_dir().join(format!("pal-f24-적대-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    let 밖 = base.join("밖");
    let 안 = base.join("안");
    std::fs::create_dir_all(&밖).expect("밖");
    std::fs::create_dir_all(&안).expect("안");
    std::fs::write(밖.join("희생양.txt"), "건드리면 안 된다\n").expect("희생양");
    std::fs::write(안.join("README.md"), "hello\n").expect("README");
    git(&안, &["init", "-q", "."]);
    방 { base, 밖, 안 }
}

impl Drop for 방 {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.base);
    }
}

fn 돌린다(cwd: &Path, args: &[&str]) -> Output {
    let pal_dir = Path::new(PAL).parent().expect("pal 의 부모");
    Command::new(PAL)
        .args(args)
        .current_dir(cwd)
        .env("PATH", path_앞에(pal_dir))
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

/// ★ **매달릴 수 있는 자리는 시간 상한을 걸고 돌린다.** 이 기계에 `timeout` 이 없다.
fn 시간_안에(cwd: &Path, args: &[&str], 상한_ms: u64) -> Output {
    let pal_dir = Path::new(PAL).parent().expect("pal 의 부모");
    let mut child = Command::new(PAL)
        .args(args)
        .current_dir(cwd)
        .env("PATH", path_앞에(pal_dir))
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("pal 을 못 돌렸다");
    let mut 기다린 = 0;
    loop {
        match child.try_wait().expect("try_wait") {
            Some(_) => return child.wait_with_output().expect("wait"),
            None if 기다린 >= 상한_ms => {
                let _ = child.kill();
                let _ = child.wait();
                panic!("pal {args:?} 가 {상한_ms}ms 안에 안 끝났다 — 매달렸다");
            }
            None => {
                std::thread::sleep(std::time::Duration::from_millis(25));
                기다린 += 25;
            }
        }
    }
}

fn 값(path: &Path) -> serde_json::Value {
    serde_json::from_slice(&std::fs::read(path).expect("읽기")).expect("JSON")
}

fn 쓴다(path: &Path, v: &serde_json::Value) {
    std::fs::write(path, serde_json::to_string_pretty(v).expect("직렬화")).expect("쓰기");
}

fn 매니페스트_자리(root: &Path) -> PathBuf {
    root.join(".claude/pal/manifest.json")
}

fn 설정_자리(root: &Path) -> PathBuf {
    root.join(".claude/settings.json")
}

// ─────────────────────────────────────────────────────────────────────────────
// 1. **저장소에서 읽은 문자열을 실행하지 않는다**
// ─────────────────────────────────────────────────────────────────────────────

/// `/tmp` 아래의 표식 하나 — **부작용이 일어났는지**를 이것 하나로 가른다.
fn 표식(tag: &str) -> PathBuf {
    let p = PathBuf::from(format!("/tmp/pal-f24-PWNED-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_file(&p);
    p
}

/// 매니페스트와 `settings.json` 의 훅 명령을 **관측 가능한 부작용을 내는 문자열**로
/// 바꾼다. 둘을 같이 바꿔야 「등록돼 있다」 검사를 지나 탐침까지 간다.
fn 훅을_심는다(root: &Path, 명령: &str) {
    let mp = 매니페스트_자리(root);
    let mut m = 값(&mp);
    let hooks = m["settings"]["hooks"].as_array_mut().expect("훅 목록");
    assert!(!hooks.is_empty(), "이 시험이 재려는 상태가 아니다 — 등록된 훅이 없다");
    for h in hooks.iter_mut() {
        h["command"] = serde_json::json!(명령);
    }
    쓴다(&mp, &m);

    let sp = 설정_자리(root);
    let mut s = 값(&sp);
    for (_, groups) in s["hooks"].as_object_mut().expect("훅 구역").iter_mut() {
        for g in groups.as_array_mut().expect("묶음") {
            for c in g["hooks"].as_array_mut().expect("명령들") {
                c["command"] = serde_json::json!(명령);
            }
        }
    }
    쓴다(&sp, &s);
}

/// ★ **`pal doctor` 가 저장소에 커밋된 문자열을 셸로 실행하지 않는다.**
///
/// 관측(고치기 전): `touch …/PWNED` 를 심었더니 **사용자 uid 로 실행됐다.**
/// `pal doctor --install` 과 인자 없는 평범한 `pal doctor` 둘 다.
#[test]
fn 진단이_매니페스트의_문자열을_실행하지_않는다() {
    let 방 = 방("실행");
    성공(&방.안, &["install"]);

    for (tag, args) in [("설치검사", &["doctor", "--install"][..]), ("전체", &["doctor"][..])] {
        let 흔적 = 표식(tag);
        훅을_심는다(&방.안, &format!("touch '{}'", 흔적.display()));

        let out = 시간_안에(&방.안, args, 60_000);
        assert!(
            !흔적.exists(),
            "{tag}: **저장소의 문자열이 실행됐다** — {} 가 생겼다\nstdout: {}",
            흔적.display(),
            String::from_utf8_lossy(&out.stdout)
        );
        let _ = std::fs::remove_file(&흔적);
    }
}

/// ★ **설치 루트 탐색이 대상 경계를 넘어 올라가지 않는다.**
///
/// 조상 디렉터리에 매니페스트를 심어 두면 아무 관계 없는 하위 디렉터리에서 돌려도
/// 그것을 찾아 **실행했다.** `--repo` 가 경계가 아니었다.
#[test]
fn 진단이_조상의_매니페스트를_찾아가지_않는다() {
    let 방 = 방("조상");
    성공(&방.안, &["install"]);

    let 흔적 = 표식("조상");
    훅을_심는다(&방.안, &format!("touch '{}'", 흔적.display()));

    // 대상은 **자기 경계를 가진 남의 자리**다 — 조상의 설치와 아무 관계가 없다.
    let 무관 = 방.안.join("무관한-하위");
    std::fs::create_dir_all(&무관).expect("무관");
    git(&무관, &["init", "-q", "."]);

    let out = 시간_안에(&무관, &["doctor", "--install", "--repo", "."], 60_000);
    assert!(
        !흔적.exists(),
        "**조상의 문자열이 실행됐다** — {} 가 생겼다",
        흔적.display()
    );
    let 화면 = String::from_utf8_lossy(&out.stdout);
    assert!(
        !화면.contains(&방.안.display().to_string()),
        "경계 밖의 설치를 찾아갔다:\n{화면}"
    );
    let _ = std::fs::remove_file(&흔적);
}

// ─────────────────────────────────────────────────────────────────────────────
// 2. **하드링크로 대상 밖이 새지 않는다** — ★ **시험 하나. `cfg` 없음**
//
// 이 절에는 얼마 전까지 시험이 **셋**이었다: 유닉스용 둘(rc≠0 을 요구) + Windows 용
// 하나(rc=0 을 요구) + 그 밖의 플랫폼을 위한 외침. 즉 **같은 상황에 대해 플랫폼마다
// 다른 답을 등록**하고 있었다.
//
// 소유자가 그것을 반증했다 (2026-08-17):
//
// > *"동일한 하네스가 서로 다른 플랫폼에서 일관된 방법과 결과를 주지 않는다면 그
// > 하네스를 쓰는 프로젝트에서는 품질이 들쭉날쭉할 텐데."*
//
// 그래서 제품이 **양쪽 다 끊고 쓰는** 쪽으로 통일됐고(`install/guard.rs` ·
// [ADR-0023]), 그 결과 **이 시험이 하나가 됐다.** `#[cfg]` 가 하나도 없다는 것이
// 곧 *"두 플랫폼이 같은 답을 낸다"* 의 증거다 — 시험 파일이 그 사실을 진다.
//
// `std::fs::hard_link` 는 이식 가능하므로 fixture 도 한 벌이다.
// ─────────────────────────────────────────────────────────────────────────────

/// ★ **하드링크는 「밖」이라는 신원이 없다** — 그래서 **끊고 쓴다.**
///
/// 심링크는 `canonicalize` 가 풀어서 막지만 하드링크는 원리상 못 본다. 제자리로 쓰면
/// 그 바이트가 대상 밖으로 그대로 간다.
///
/// 관측(고치기 전): 유닉스에서 밖의 `희생양.txt` 가 0바이트가 됐고 **rc=0** ·
/// Windows 에서 밖의 파일에 우리 블록이 실렸고 **rc=0**.
///
/// **지금은 양쪽에서 같은 일이 일어난다**: 설치는 **성공**하고, 밖은 **안 바뀌고**,
/// 링크는 **끊긴다.**
#[test]
fn 설치가_하드링크를_끊고_밖을_안_고친다() {
    let 방 = 방("하드-설치");
    let 희생양 = 방.밖.join("희생양.txt");
    let 원본 = std::fs::read(&희생양).expect("읽기");
    let 안의것 = 방.안.join("CLAUDE.md");
    // 대상 **안**의 `CLAUDE.md` 가 밖의 파일과 **같은 실체**다.
    std::fs::hard_link(&희생양, &안의것).expect("hard_link");

    let out = 돌린다(&방.안, &["install"]);
    assert!(
        out.status.success(),
        "끊고 썼어야 한다\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    // ① ★ **밖이 안 바뀌었다** — `[f24]` ⑦ 이 여기서 선다.
    assert_eq!(std::fs::read(&희생양).expect("읽기"), 원본, "밖의 파일이 바뀌었다");

    // ② 그리고 **안에는 실제로 썼다.** 이 줄이 없으면 「아무것도 안 했다」도 ① 을 통과한다.
    let 안의_내용 = std::fs::read(&안의것).expect("읽기");
    assert_ne!(안의_내용, 원본, "안쪽에 아무것도 안 썼다 — ① 이 공짜로 통과했다");
    assert!(안의_내용.starts_with(&원본), "사용자 바이트가 접두사로 안 남았다");

    // ③ ★ **링크가 실제로 끊겼다.** 안쪽을 또 고쳐도 밖이 안 따라온다 —
    //    ① 이 「이번 한 번만 안 새는 것」이 아니라 **링크가 끊긴 것**임을 못 박는다.
    std::fs::write(&안의것, "그 뒤에 또 고친다\n").expect("쓰기");
    assert_eq!(std::fs::read(&희생양).expect("읽기"), 원본, "링크가 안 끊겼다");

    // ④ 잔해가 없다 — 임시 파일이 남으면 그것이 곧 부분 설치다.
    let 잔해: Vec<_> = std::fs::read_dir(&방.안)
        .expect("읽기")
        .flatten()
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|n| n.contains("pal-relink"))
        .collect();
    assert!(잔해.is_empty(), "임시 파일이 남았다: {잔해:?}");
}

/// **제거 쪽도 같은 문**을 지난다 — 설치 뒤에 걸린 링크에도 밖이 안 샌다.
#[test]
fn 제거가_하드링크를_통해_밖을_안_고친다() {
    let 방 = 방("하드-제거");
    std::fs::write(방.안.join("CLAUDE.md"), "내 지시\n").expect("CLAUDE.md");
    성공(&방.안, &["install"]);

    // 설치가 끝난 뒤 사용자가(혹은 남이) 밖에서 링크를 걸었다.
    let 링크 = 방.밖.join("링크.txt");
    std::fs::hard_link(방.안.join("CLAUDE.md"), &링크).expect("hard_link");
    let 밖_전 = std::fs::read(&링크).expect("읽기");

    let out = 돌린다(&방.안, &["uninstall"]);
    assert!(
        out.status.success(),
        "끊고 걷어냈어야 한다\nstderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    // ★ 밖의 파일은 **설치 상태의 내용을 그대로 지고** 남는다 — 우리가 그것을 안 건드렸다.
    assert_eq!(std::fs::read(&링크).expect("읽기"), 밖_전, "밖의 파일이 바뀌었다");
    // 그리고 안쪽은 제거가 됐다 — 블록이 빠졌으니 둘이 갈렸다.
    let 안의것 = std::fs::read(방.안.join("CLAUDE.md")).expect("읽기");
    assert_ne!(안의것, 밖_전, "안쪽에서 블록이 안 빠졌다");
}

/// ★ **끊으면서 침묵하는 플랫폼이 없다** — `cfg` 가 하나도 없다.
///
/// # 앞 판은 여기서 갈렸고, 갈린 쪽이 **말할 것이 더 많은 쪽**이었다
///
/// 유닉스는 링크 수를 셀 수 있으니 걸린 자리마다 말했고, Windows 는 못 세니
/// `하드링크_알림` 이 언제나 `None` 이라 **아무 말도 안 냈다.** 그런데 Windows 는
/// **늘 끊는다** — 즉 정보가 더 필요한 쪽이 조용했다. 그래서 그 자리에는 외침
/// (`끊었다는_말이_이_플랫폼에서는_안_나온다`)이 걸려 있었다.
///
/// **없앴다.** 못 세는 플랫폼은 *"걸렸는지 모르므로 늘 끊는다"* 를 자리 목록과 함께
/// 한 줄로 낸다(`install::쓸_수_있나`). 문구는 여전히 다르지만 — 셀 수 있는 쪽은
/// *"이 파일에 걸려 있다"*, 못 세는 쪽은 *"모르니 늘 끊는다"* — **둘 다 말하고, 둘 다
/// 어느 파일인지 적는다.** 이 시험이 재는 것이 정확히 그 둘이다.
///
/// 문구를 억지로 같게 만들지 않는 이유: 같게 만들려면 볼 수 있는 쪽이 입을 다물어야
/// 하고, 그것은 대칭이 아니라 **정보를 버리는 것**이다(ADR-0023).
#[test]
fn 끊을_때_어느_플랫폼도_침묵하지_않는다() {
    let 방 = 방("하드-알림");
    let 희생양 = 방.밖.join("희생양.txt");
    std::fs::hard_link(&희생양, 방.안.join("CLAUDE.md")).expect("hard_link");

    let out = 돌린다(&방.안, &["install"]);
    let 화면 = String::from_utf8_lossy(&out.stdout);
    assert!(화면.contains("하드링크"), "끊어 놓고 말하지 않았다:\n{화면}");
    assert!(화면.contains("CLAUDE.md"), "어느 파일인지 안 적었다:\n{화면}");
}

// ─────────────────────────────────────────────────────────────────────────────
// 3. **매니페스트가 대상 안의 아무 파일이나 못 지운다**
// ─────────────────────────────────────────────────────────────────────────────

/// ★ **우리가 되돌릴 수 있는 것은 우리가 놓을 수 있는 자리뿐이다.**
///
/// 관측(고치기 전): `.git/config` 와 `README.md` 를 각각 지웠다(**rc=0**).
#[test]
fn 매니페스트가_적은_안의_남의_파일을_안_지운다() {
    for 노린_것 in [".git/config", "README.md"] {
        let 방 = 방(&format!("안쪽-{}", 노린_것.replace(['/', '.'], "-")));
        성공(&방.안, &["install"]);

        let mp = 매니페스트_자리(&방.안);
        let mut m = 값(&mp);
        m["files"].as_array_mut().expect("files").push(serde_json::json!({
            "path": 노린_것,
            "sha256": "0".repeat(64),
        }));
        쓴다(&mp, &m);

        let out = 돌린다(&방.안, &["uninstall"]);
        assert!(
            방.안.join(노린_것).exists(),
            "{노린_것} 이 사라졌다 — 매니페스트가 대상 안의 남의 파일을 지웠다"
        );
        assert!(
            !out.status.success(),
            "{노린_것}: 우리가 놓을 수 없는 자리를 보고도 성공을 냈다\nstdout: {}",
            String::from_utf8_lossy(&out.stdout)
        );
    }
}

/// **블록에도 같은 문이 선다.**
#[test]
fn 매니페스트가_적은_안의_남의_블록을_안_건드린다() {
    let 방 = 방("안쪽-블록");
    성공(&방.안, &["install"]);
    let 원본 = std::fs::read(방.안.join("README.md")).expect("읽기");

    let mp = 매니페스트_자리(&방.안);
    let mut m = 값(&mp);
    m["blocks"].as_array_mut().expect("blocks").push(serde_json::json!({
        "path": "README.md",
        "inserted": "hello\n",
        "created": true,
    }));
    쓴다(&mp, &m);
    let out = 돌린다(&방.안, &["uninstall"]);
    assert!(방.안.join("README.md").exists(), "남의 파일이 사라졌다");
    assert_eq!(std::fs::read(방.안.join("README.md")).expect("읽기"), 원본, "남의 파일이 바뀌었다");
    assert!(!out.status.success(), "남의 블록을 보고도 성공을 냈다");
}

/// **디렉터리에도 같은 문이 선다.**
#[test]
fn 매니페스트가_적은_안의_남의_디렉터리를_안_지운다() {
    let 방 = 방("안쪽-디렉터리");
    성공(&방.안, &["install"]);
    let 남의_방 = 방.안.join("남의방");
    std::fs::create_dir_all(&남의_방).expect("남의방");
    let mp = 매니페스트_자리(&방.안);
    let mut m = 값(&mp);
    m["created_dirs"].as_array_mut().expect("created_dirs").push(serde_json::json!("남의방"));
    쓴다(&mp, &m);
    let out = 돌린다(&방.안, &["uninstall"]);
    assert!(남의_방.is_dir(), "남의 디렉터리가 사라졌다");
    assert!(!out.status.success(), "남의 디렉터리를 보고도 성공을 냈다");
}

// ─────────────────────────────────────────────────────────────────────────────
// 5. **FIFO 에서 안 매달린다**
//
// ★ **유닉스 전용 가정이 여기 있다.** 이름 있는 파이프는 `mkfifo` 로 만들고, 그것이
// `fs::read` 를 잡는 것도 유닉스 형태다. Windows 의 등가 개념(named pipe ·
// `\\.\pipe\`)은 이 자리에 파일 이름으로 앉지 않는다. **짝 없는 `#[cfg(unix)]` 을
// 안 단다** — 아래 짝이 다른 플랫폼에서 시끄럽게 실패한다.
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(unix)]
fn 이름있는_파이프(path: &Path) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("부모");
    }
    // `git init` 이 이미 만들어 두는 자리가 있다(`.git/info/exclude`).
    let _ = std::fs::remove_file(path);
    let out = Command::new("mkfifo").arg(path).output().expect("mkfifo 를 못 돌렸다");
    assert!(out.status.success(), "mkfifo: {}", String::from_utf8_lossy(&out.stderr));
}

/// ★ **우리가 읽고 쓰는 자리는 일반 파일이거나 없거나 둘 중 하나다.**
///
/// 관측(고치기 전): `settings.json` 이 FIFO 면 `install` 과 `doctor` 가 **영원히**
/// 매달렸다. 파일 종류 검사도 시간 상한도 없었다.
///
/// ★ **목록은 「우리가 읽는 자리」로 끝나지 않는다.** `.claude/.pal.lock` 은 잠금이
/// 직접 여는 자리였고, `.palimpsest/.gitignore` 와 `.git/info/exclude` 는
/// **`git check-ignore` 가 읽는 자리**다. *"우리가 읽는 자리는 일반 파일이거나
/// 없거나"* 라는 규율은 **우리 대신 읽는 프로세스에도** 선다.
#[test]
#[cfg(unix)]
fn 파이프가_있으면_매달리지_않고_실패한다() {
    for (tag, 자리) in [
        ("설정", ".claude/settings.json"),
        ("지시", "CLAUDE.md"),
        ("무시목록", ".gitignore"),
        // 우리가 직접 여는 자리인데 목록에 없었다.
        ("잠금", ".claude/.pal.lock"),
        // ★ 아래 셋은 **우리가 아니라 `git check-ignore` 가 읽는다.** 우리 코드에
        // `fs::read` 가 하나도 없어도 매달린다 — 매다는 것이 우리가 부른 프로세스다.
        ("중첩무시목록", ".palimpsest/.gitignore"),
        ("제외목록", ".git/info/exclude"),
    ] {
        let 방 = 방(&format!("파이프-{tag}"));
        이름있는_파이프(&방.안.join(자리));

        let out = 시간_안에(&방.안, &["install"], 15_000);
        assert!(
            !out.status.success(),
            "{tag}: FIFO 를 보고도 성공을 냈다\nstdout: {}",
            String::from_utf8_lossy(&out.stdout)
        );
        // `doctor` 도 같은 자리를 읽는다 — 여기서도 안 매달린다.
        시간_안에(&방.안, &["doctor", "--install", "--json"], 15_000);
    }
}

/// ★ **잠금은 세 경로가 전부 여는 자리다** — 설치된 프로젝트에서 `update` 와
/// `uninstall` 도 여기서 매달렸다.
#[test]
#[cfg(unix)]
fn 잠금이_파이프면_세_경로가_전부_안_매달린다() {
    let 방 = 방("파이프-잠금-셋");
    성공(&방.안, &["install"]);
    이름있는_파이프(&방.안.join(".claude/.pal.lock"));

    for args in [&["install"][..], &["update"][..], &["uninstall"][..]] {
        let out = 시간_안에(&방.안, args, 15_000);
        assert!(
            !out.status.success(),
            "pal {args:?}: 잠금이 FIFO 인데 성공을 냈다\nstdout: {}",
            String::from_utf8_lossy(&out.stdout)
        );
    }
}

/// ★ **재려는 성질은 「FIFO」가 아니라 「일반 파일이 아닌 자리」다** — 그리고 그것은
/// 이식 가능하게 재진다.
///
/// # 무엇이 바뀌었나
///
/// 앞 판은 이 절 전체가 `mkfifo` 위에 서 있었고, 다른 플랫폼에는 외침
/// (`파이프_방어가_이_플랫폼에서는_안_재진다`)만 있었다. 그런데 제품이 세운 문은
/// [`install::guard::일반_파일이거나_없나`] 이고 그 문이 묻는 것은 **`is_file()`**
/// 이다 — FIFO 인지가 아니다. **디렉터리는 어느 플랫폼에서나 그 문에 걸리는 「일반
/// 파일이 아닌 것」이고, `std::fs::create_dir` 로 어디서나 세워진다.**
///
/// 그래서 문 자체는 여기서 **세 플랫폼 전부** 재진다. FIFO 시험은 지우지 않고 남긴다 —
/// 그것은 **매달림**이라는 더 센 사실을 재고, 디렉터리는 그것을 못 잰다
/// (`fs::read` 가 디렉터리에서는 매달리지 않고 오류를 낸다).
///
/// # ⚠ 「fixture 가 없다」와 「위험이 없다」는 다르다 — 후자를 여기 적는다
///
/// Windows 에 **파일 이름으로 앉는 FIFO 는 원리상 없다.** 이름 있는 파이프는
/// `\\.\pipe\` 라는 **별도의 이름공간**에 살고 디렉터리 항목이 되지 않는다 —
/// `CreateFile` 이 그 이름을 열 때 지나는 것은 파일시스템이 아니라 named pipe
/// 파일시스템 드라이버다. 즉 `.claude/settings.json` 이라는 **경로에** 파이프를 놓는
/// 상태 자체가 만들어지지 않는다.
///
/// 남는 형태 하나는 *"그 경로가 `\\.\pipe\…` 를 가리키는 심링크"* 인데, 그것을 재려면
/// **파이프 서버를 띄워야** 하고(서버 없는 파이프 이름은 매달리지 않고 즉시
/// `ERROR_FILE_NOT_FOUND` 다) 서버를 세우는 문은 raw FFI 뿐이라 `unsafe 금지` 게이트가
/// 막는다. **그것이 이 저장소가 그 한 형태를 못 재는 원리상의 이유다** — 「아직」이
/// 아니다. 그리고 그 형태도 경계 검사가 먼저 막는다: `\\.\pipe\…` 는 대상 밖이다.
#[test]
fn 일반_파일이_아닌_자리에서_매달리지_않고_실패한다() {
    for (tag, 자리) in [
        ("설정", ".claude/settings.json"),
        ("지시", "CLAUDE.md"),
        ("무시목록", ".gitignore"),
        ("잠금", ".claude/.pal.lock"),
        ("중첩무시목록", ".palimpsest/.gitignore"),
        ("제외목록", ".git/info/exclude"),
    ] {
        let 방 = 방(&format!("디렉터리-{tag}"));
        let p = 방.안.join(자리);
        if let Some(parent) = p.parent() {
            std::fs::create_dir_all(parent).expect("부모");
        }
        // `git init` 이 이미 만들어 두는 자리가 있다(`.git/info/exclude`).
        let _ = std::fs::remove_file(&p);
        std::fs::create_dir(&p).expect("디렉터리 fixture");
        // ★ **안 비워 둔다 — 그리고 그 이유를 이 시험이 찾아냈다.**
        //
        // 처음 판은 빈 디렉터리였고 `잠금` 칸에서 **설치가 성공했다.** 결함이 아니라
        // 설계다: [`install::Lock::take`] 는 *"옛 빌드가 남긴 디렉터리 잔해"* 를
        // `remove_dir` 로 치우고 지나간다(디렉터리 잠금을 쓰던 시절의 잔해). 빈
        // 디렉터리는 그 경로에서 **정당하게 사라진다.**
        //
        // 그러면 그 칸은 문(`일반_파일이거나_없나`)을 재는 게 아니라 **잔해 정리를**
        // 재게 된다 — 즉 재려던 것이 사라진다. 안 비우면 `remove_dir` 이 실패하고
        // 문이 판정한다. 나머지 다섯 칸에도 해가 없으므로 fixture 를 하나로 둔다.
        std::fs::write(p.join("안_비었다.txt"), b"x").expect("디렉터리 안의 파일");

        let out = 시간_안에(&방.안, &["install"], 15_000);
        assert!(
            !out.status.success(),
            "{tag}: 일반 파일이 아닌 자리를 보고도 성공을 냈다\nstdout: {}",
            String::from_utf8_lossy(&out.stdout)
        );
        // `doctor` 도 같은 자리를 읽는다 — 여기서도 안 매달린다.
        시간_안에(&방.안, &["doctor", "--install", "--json"], 15_000);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 6. **매니페스트의 크기가 공격자 손에 있다**
// ─────────────────────────────────────────────────────────────────────────────

/// 매니페스트의 `files` 를 **정당한 경로로만** 불린다.
///
/// ★ 경계 검사가 못 막는 형태다 — 하나하나가 전부 우리가 놓을 수 있는 자리다.
fn 항목을_불린다(root: &Path, 개수: usize) {
    let mp = 매니페스트_자리(root);
    let mut m = 값(&mp);
    let files = m["files"].as_array_mut().expect("files");
    for i in 0..개수 {
        files.push(serde_json::json!({
            "path": format!(".claude/pal/채움-{i}.md"),
            "sha256": "0".repeat(64),
            "origin": "ours",
        }));
    }
    쓴다(&mp, &m);
}

/// ★ **커밋되는 파일 하나로 거는 DoS.**
///
/// 관측(고치기 전): 항목 200개 0.12s · 800개 1.18s · **3,200개 18.09s** ·
/// 20,000개는 60초에 미완료. 항목 **하나를 걷을 때마다 매니페스트 전체를 다시 썼다**
/// (O(n²)). 매니페스트는 대상 안의 **커밋되는 평범한 파일**이라 크기가 남의 손에 있고,
/// 전부 정당한 경로라 경계 검사가 못 막는다.
#[test]
fn 매니페스트의_항목_수가_무기가_되지_않는다() {
    let 방 = 방("항목수");
    성공(&방.안, &["install"]);
    항목을_불린다(&방.안, 3_200);

    let t0 = std::time::Instant::now();
    let out = 시간_안에(&방.안, &["uninstall"], 30_000);
    assert!(
        !out.status.success(),
        "상한을 넘긴 매니페스트를 그대로 걷었다 ({:?})",
        t0.elapsed()
    );
    assert!(
        t0.elapsed() < std::time::Duration::from_secs(5),
        "항목 수에 값이 붙어 있다 — {:?} 걸렸다",
        t0.elapsed()
    );
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    assert!(stderr.contains("상한"), "무엇이 걸렸는지 안 적었다:\n{stderr}");
}

/// ★ **상한 안이면 값이 항목 수에 선형이다** — 그리고 걷어낸다.
///
/// 상한만으로는 부족하다. 상한 바로 아래에서 O(n²) 가 남아 있으면 같은 형태가
/// 그대로 산다 — 그래서 **상한 가장자리**를 재고, 그 자리가 빨라야 한다.
#[test]
fn 상한_안의_큰_매니페스트도_빠르게_걷힌다() {
    let 방 = 방("상한가장자리");
    성공(&방.안, &["install"]);
    // 설치가 적는 경로가 17개다. 상한(256) 바로 아래까지 채운다.
    항목을_불린다(&방.안, 230);

    let t0 = std::time::Instant::now();
    let out = 시간_안에(&방.안, &["uninstall"], 30_000);
    assert!(
        out.status.success(),
        "상한 안인데 거부했다\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        t0.elapsed() < std::time::Duration::from_secs(3),
        "상한 가장자리에서 {:?} 걸렸다",
        t0.elapsed()
    );
    assert!(!방.안.join(".claude").exists(), "`.claude/` 가 남았다");
}

// ─────────────────────────────────────────────────────────────────────────────
// 7. **설치도 이전 매니페스트를 믿지 않는다**
// ─────────────────────────────────────────────────────────────────────────────

/// ★ **`install` 만 이전 매니페스트에 경계 검사를 안 걸었다.**
///
/// 관측(고치기 전): `settings.path = ".git/config"` 를 심으면 `install` 이 그 값을
/// **새 매니페스트로 그대로 실어 나르면서 rc=0** 「설치」 화면을 냈다. 그 뒤
/// `update`·`uninstall` 은 영원히 rc=1 이고 **되돌릴 수 있는 `pal` 명령이 하나도
/// 없다.** 데이터 손상은 없다 — **거짓 성공 + 되돌림 봉쇄**가 문제다.
#[test]
fn 설치도_오염된_매니페스트를_거부한다() {
    for (tag, 오염) in [
        ("설정", &(|m: &mut serde_json::Value| m["settings"]["path"] = serde_json::json!(".git/config"))
            as &dyn Fn(&mut serde_json::Value)),
        ("파일", &|m: &mut serde_json::Value| {
            m["files"].as_array_mut().expect("files").push(serde_json::json!({
                "path": "README.md", "sha256": "0".repeat(64), "origin": "ours",
            }));
        }),
        ("디렉터리", &|m: &mut serde_json::Value| {
            m["created_dirs"].as_array_mut().expect("created_dirs").push(serde_json::json!(".git"));
        }),
    ] {
        let 방 = 방(&format!("오염-{tag}"));
        성공(&방.안, &["install"]);

        let mp = 매니페스트_자리(&방.안);
        let mut m = 값(&mp);
        오염(&mut m);
        쓴다(&mp, &m);
        let 심은_바이트 = std::fs::read(&mp).expect("읽기");

        let out = 돌린다(&방.안, &["install"]);
        assert!(
            !out.status.success(),
            "{tag}: 오염된 매니페스트를 보고도 rc=0 「설치」 화면을 냈다\nstdout: {}",
            String::from_utf8_lossy(&out.stdout)
        );
        assert_eq!(
            std::fs::read(&mp).expect("읽기"),
            심은_바이트,
            "{tag}: 거부했는데 매니페스트를 되썼다 — 오염이 새 기록으로 실려 나간다"
        );
    }
}
