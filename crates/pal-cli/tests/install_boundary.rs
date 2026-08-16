//! ★ **매니페스트가 적은 경로가 대상 밖을 가리키면 건드리지 않는다** — `[f24]` ⑥⑦.
//!
//! # 왜 이것이 매니페스트의 문제인가
//!
//! 매니페스트는 **대상 프로젝트 안에 사는 파일**이다. 남의 저장소에
//! `.claude/pal/manifest.json` 이 커밋돼 있으면 `pal uninstall` 한 번이 **그 파일이 적은
//! 아무 경로나** 지운다. `Path::join` 은 **절대 경로를 받으면 base 를 통째로 버리고**,
//! `..` 하나면 경계가 사라진다.
//!
//! # 희생양은 전부 시험 방 안에 산다
//!
//! 탈출을 재려면 **탈출당할 자리**가 있어야 한다. 그 자리는 시험 방의 형제 디렉터리로
//! 만든다 — 실제로 지워지면 안 되는 곳을 대상으로 삼지 않는다.

mod common;

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use common::{PAL, git, path_앞에};

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

/// 방 하나 — `밖/` 과 `안/` 이 형제로 산다.
struct 방 {
    base: PathBuf,
    밖: PathBuf,
    안: PathBuf,
}

fn 방(tag: &str) -> 방 {
    let base = std::env::temp_dir().join(format!("pal-f24-경계-{tag}-{}", std::process::id()));
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

fn 매니페스트(안: &Path) -> serde_json::Value {
    serde_json::from_slice(&std::fs::read(안.join(".claude/pal/manifest.json")).expect("읽기"))
        .expect("JSON")
}

fn 매니페스트_쓰기(안: &Path, m: &serde_json::Value) {
    std::fs::write(
        안.join(".claude/pal/manifest.json"),
        serde_json::to_string_pretty(m).expect("직렬화"),
    )
    .expect("쓰기");
}

/// ★ **`files` 가 밖을 가리키면 지우지 않는다.** 상대(`..`)와 절대 둘 다.
#[test]
fn 매니페스트가_적은_밖의_파일을_안_지운다() {
    for (tag, 만든다) in [
        ("상대", (|밖: &Path| {
            let _ = 밖;
            "../밖/희생양.txt".to_owned()
        }) as fn(&Path) -> String),
        ("절대", |밖: &Path| 밖.join("희생양.txt").display().to_string()),
    ] {
        let 방 = 방(tag);
        성공(&방.안, &["install"]);

        let mut m = 매니페스트(&방.안);
        let 밖의_경로 = 만든다(&방.밖);
        m["files"].as_array_mut().expect("files").push(serde_json::json!({
            "path": 밖의_경로,
            "sha256": "0".repeat(64),
        }));
        매니페스트_쓰기(&방.안, &m);

        let out = 돌린다(&방.안, &["uninstall"]);
        let 희생양 = 방.밖.join("희생양.txt");
        assert!(희생양.exists(), "{tag}: 대상 밖의 파일이 사라졌다 — {}", 희생양.display());
        assert!(
            !out.status.success(),
            "{tag}: 밖을 가리키는 항목을 보고도 성공을 냈다\nstdout: {}",
            String::from_utf8_lossy(&out.stdout)
        );
        let stderr = String::from_utf8_lossy(&out.stderr);
        assert!(stderr.contains("대상 밖"), "{tag}: 까닭을 안 적었다 — {stderr}");
    }
}

/// **`blocks` 는 파일 내용을 다시 쓴다** — 같은 경계가 필요하다.
#[test]
fn 매니페스트가_적은_밖의_블록을_안_고친다() {
    let 방 = 방("블록");
    성공(&방.안, &["install"]);

    let 희생양 = 방.밖.join("희생양.txt");
    let 원본 = std::fs::read(&희생양).expect("읽기");

    let mut m = 매니페스트(&방.안);
    m["blocks"].as_array_mut().expect("blocks").push(serde_json::json!({
        "path": "../밖/희생양.txt",
        "inserted": "건드리면 안 된다\n",
        "created": true,
    }));
    매니페스트_쓰기(&방.안, &m);

    let out = 돌린다(&방.안, &["uninstall"]);
    assert!(희생양.exists(), "대상 밖의 파일이 사라졌다");
    assert_eq!(std::fs::read(&희생양).expect("읽기"), 원본, "대상 밖의 파일이 고쳐졌다");
    assert!(!out.status.success(), "밖을 가리키는 블록을 보고도 성공을 냈다");
}

/// **`created_dirs` 는 디렉터리를 지운다** — 같은 경계가 필요하다.
#[test]
fn 매니페스트가_적은_밖의_디렉터리를_안_지운다() {
    let 방 = 방("디렉터리");
    성공(&방.안, &["install"]);

    let 빈_디렉터리 = 방.밖.join("빈방");
    std::fs::create_dir_all(&빈_디렉터리).expect("빈방");

    let mut m = 매니페스트(&방.안);
    m["created_dirs"].as_array_mut().expect("created_dirs").push(serde_json::json!("../밖/빈방"));
    매니페스트_쓰기(&방.안, &m);

    let out = 돌린다(&방.안, &["uninstall"]);
    assert!(빈_디렉터리.is_dir(), "대상 밖의 디렉터리가 사라졌다");
    assert!(!out.status.success(), "밖을 가리키는 디렉터리를 보고도 성공을 냈다");
}

// ─────────────────────────────────────────────────────────────────────────────
// 심링크 — **안을 가리키는 것은 살리고 밖으로 나가는 것은 막는다**
//
// ★ **유닉스 전용 가정이 여기 있다.** 링크를 거는 것(`std::os::unix::fs::symlink`)도
// 그것이 경계를 넘는 것도 유닉스 형태로 fixture 를 만든다. 그래서 **짝 없는
// `#[cfg(unix)]` 을 안 단다** — 아래 짝이 다른 플랫폼에서 **시끄럽게 실패한다.**
// ─────────────────────────────────────────────────────────────────────────────

/// ★ **쓰기 대상이 심링크로 대상 밖을 가리키면 쓰지 않는다.**
///
/// 소유자가 쓴 문장은 이것이다 — *"`~/.claude/` 하위에 기대는 구조는 절대 있어서는
/// 안 돼"*. `.claude → ~/.claude` 는 dotfiles 를 홈에 모으는 **흔한 형태**이고,
/// 그때 설치가 그 밖에 쓰면 `[f24]` ⑦ 이 무너진다.
#[test]
#[cfg(unix)]
fn 밖을_가리키는_심링크에는_안_쓴다() {
    for (tag, 이름, 대상이_디렉터리인가) in
        [("클로드디렉터리", ".claude", true), ("지시파일", "CLAUDE.md", false), ("무시목록", ".gitignore", false)]
    {
        let 방 = 방(tag);
        let 밖의_자리 = 방.밖.join(format!("남의-{이름}"));
        if 대상이_디렉터리인가 {
            std::fs::create_dir_all(&밖의_자리).expect("밖 디렉터리");
        } else {
            std::fs::write(&밖의_자리, "남의 것\n").expect("밖 파일");
        }
        std::os::unix::fs::symlink(&밖의_자리, 방.안.join(이름)).expect("symlink");

        let 밖_전 = 훑기(&방.밖);
        let out = 돌린다(&방.안, &["install"]);
        assert_eq!(훑기(&방.밖), 밖_전, "{tag}: 대상 밖이 바뀌었다");
        assert!(
            !out.status.success(),
            "{tag}: 밖을 가리키는 심링크에 쓰고도 성공을 냈다\nstdout: {}",
            String::from_utf8_lossy(&out.stdout)
        );
        let stderr = String::from_utf8_lossy(&out.stderr);
        assert!(stderr.contains("대상 밖"), "{tag}: 까닭을 안 적었다 — {stderr}");
    }
}

/// **안을 가리키는 심링크는 살린다** — 그 구분이 이 회차가 세우는 것이다.
#[test]
#[cfg(unix)]
fn 안을_가리키는_심링크는_살린다() {
    let 방 = 방("안쪽심링크");
    std::fs::write(방.안.join("진짜무시목록"), "node_modules/\n").expect("진짜");
    std::os::unix::fs::symlink("진짜무시목록", 방.안.join(".gitignore")).expect("symlink");

    성공(&방.안, &["install"]);

    assert!(
        std::fs::symlink_metadata(방.안.join(".gitignore"))
            .expect("lstat")
            .file_type()
            .is_symlink(),
        "심링크가 일반 파일로 바뀌었다"
    );
    assert!(
        std::fs::read_to_string(방.안.join("진짜무시목록")).expect("읽기").contains("pal:begin"),
        "심링크 대상에 안 쓰였다"
    );
}

/// ★ **읽기에도 경계가 있다** — `doctor` 가 대상 밖 파일을 읽지 않는다.
///
/// 관측(고치기 전): `doctor` 의 검사 1·2·6 이 `Root::join` 을 안 지나고
/// `target.join(...)` 을 직접 썼다. `.claude/settings.json` 이 홈을 가리키는
/// 심링크면 **대상 밖 파일을 읽어서** 그 내용으로 판정했다(실측: `$HOME` 쪽 파일의
/// 파싱 오류가 화면에 났다). **쓰기는 안 일어난다** — 그래서 게이트 ⑦ 은 초록이었고
/// 이 자리는 ⑦ 의 뒷면인 **읽기 경계**다.
#[test]
#[cfg(unix)]
fn 진단이_대상_밖을_안_읽는다() {
    for (tag, 안의_자리, 밖의_내용) in [
        ("설정", ".claude/settings.json", "{\"밖의키\": 1}\n"),
        // 매니페스트가 밖을 가리키면 검사 2·6 이 남의 파일을 읽는다.
        ("매니페스트", ".claude/pal/manifest.json", "{\"pal_version\":\"밖\"}\n"),
    ] {
        let 방 = 방(&format!("읽기경계-{tag}"));
        let 밖의_자리 = 방.밖.join(format!("남의-{tag}"));
        std::fs::write(&밖의_자리, 밖의_내용).expect("밖 파일");
        let 안 = 방.안.join(안의_자리);
        std::fs::create_dir_all(안.parent().expect("부모")).expect("부모 만들기");
        std::os::unix::fs::symlink(&밖의_자리, &안).expect("symlink");

        let out = 돌린다(&방.안, &["doctor", "--install", "--json"]);
        let 화면 = String::from_utf8_lossy(&out.stdout).into_owned();
        assert!(
            화면.contains("대상 밖"),
            "{tag}: 밖을 가리키는 자리를 그대로 읽었다\n{화면}"
        );
        // 밖의 내용이 화면에 실려 나가지도 않는다.
        assert!(!화면.contains("밖의키"), "{tag}: 남의 파일 내용이 화면에 실렸다\n{화면}");
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Windows — **junction 이 경계를 넘는 유일한 문이다**
// ─────────────────────────────────────────────────────────────────────────────
//
// 앞 회차는 여기 전체를 *"이 플랫폼에서는 안 재진다"* 외침 하나로 뒀다. 그 외침이
// 적은 이유는 **절반만 맞았다** — 심링크(`symlink_dir`/`symlink_file`)는 정말로
// 개발자 모드나 `SeCreateSymbolicLinkPrivilege` 를 요구한다. 그런데 **디렉터리
// junction 은 일반 사용자 권한으로 만들어진다**(실측 2026-08-16). 그리고 `.claude`
// 를 밖으로 빼는 형태가 정확히 소유자가 금지한 그 구조다 —
// *"`~/.claude/` 하위에 기대는 구조는 절대 있어서는 안 돼"*.
//
// 그래서 **디렉터리 축은 승격하고, 파일 축만 외침으로 남긴다.**

/// junction 을 건다 — **`cmd` 를 쓰는 것을 여기서 정당화한다.**
///
/// junction 은 std 로 못 만든다(`CreateFile` + `FSCTL_SET_REPARSE_POINT` 는 raw FFI
/// 이고 `unsafe 금지` 게이트가 막는다). `mklink` 는 `cmd.exe` 의 내장 명령이고
/// **`cmd.exe` 는 이 플랫폼의 일부**다 — 걷어낸 `shasum`·`cmp` 와 다르다. 그것들은
/// 이 플랫폼에 **없는** 도구였고, 이것은 **재려는 대상이 플랫폼 기능 그 자체**다.
#[cfg(windows)]
fn junction(링크: &Path, 대상: &Path) {
    let out = Command::new("cmd")
        .args(["/C", "mklink", "/J"])
        .arg(링크)
        .arg(대상)
        .output()
        .expect("cmd 를 못 돌렸다");
    assert!(
        out.status.success(),
        "junction 을 못 걸었다 — fixture 가 안 섰다: {}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
}

/// ★ **`.claude` 가 junction 으로 대상 밖을 가리키면 쓰지 않는다.**
///
/// 유닉스의 `밖을_가리키는_심링크에는_안_쓴다` 와 **같은 문장을 같은 단언으로** 잰다 —
/// `Root::join` 이 `canonicalize` 로 신원을 풀어 밖이라고 판정하는 자리이고, junction
/// 도 심링크와 마찬가지로 그 해석을 지난다.
#[test]
#[cfg(windows)]
fn 밖을_가리키는_junction_에는_안_쓴다() {
    let 방 = 방("junction");
    let 밖의_자리 = 방.밖.join("남의-claude");
    std::fs::create_dir_all(&밖의_자리).expect("밖 디렉터리");
    junction(&방.안.join(".claude"), &밖의_자리);

    let 밖_전 = 훑기(&방.밖);
    let out = 돌린다(&방.안, &["install"]);

    // ① 밖이 한 바이트도 안 바뀌었다 — ⑦ 이 여기서 선다.
    assert_eq!(훑기(&방.밖), 밖_전, "대상 밖이 바뀌었다");
    // ② 그리고 **성공을 안 냈다.** 조용히 아무것도 안 하는 것과 다르다.
    assert!(
        !out.status.success(),
        "밖을 가리키는 junction 에 쓰고도 성공을 냈다\nstdout: {}",
        String::from_utf8_lossy(&out.stdout)
    );
    // ③ 까닭을 적었다.
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("대상 밖"), "까닭을 안 적었다 — {stderr}");
}

/// ★ **파일 축은 이 플랫폼에서 아직 안 재진다 — 그 사실이 시끄러워야 한다.**
///
/// 디렉터리 축(`.claude` → 밖)은 위에서 junction 으로 승격됐다. 남은 것 둘은
/// **파일 심링크**를 요구하고, 그것은 개발자 모드나 `SeCreateSymbolicLinkPrivilege`
/// 없이는 못 만든다(실측 2026-08-16: 관리자 아님 · 개발자 모드 꺼짐 → 심링크 생성 불가).
/// junction 은 **디렉터리에만** 걸린다.
#[test]
#[cfg(not(unix))]
fn 파일_심링크_경계가_이_플랫폼에서는_안_재진다() {
    panic!(
        "**파일**이 밖을 가리킬 때(`CLAUDE.md`·`.gitignore` 가 밖의 파일을 가리키는 \
         심링크) 쓰는지 · 안을 가리키는 파일 심링크를 살리는지 · 진단이 그것을 통해 \
         밖을 읽는지를 이 플랫폼에서 아직 안 잰다 — fixture 가 파일 심링크를 요구하고, \
         이 플랫폼에서 그것은 개발자 모드나 `SeCreateSymbolicLinkPrivilege` 가 있어야 \
         만들어진다(junction 은 디렉터리에만 걸린다).\n    \
         ★ **디렉터리 축은 이제 재진다** — `밖을_가리키는_junction_에는_안_쓴다`"
    );
}

/// 트리 전체의 `(상대 경로 → 내용 표식)`. 디렉터리도 센다.
fn 훑기(root: &Path) -> std::collections::BTreeMap<String, String> {
    let mut out = std::collections::BTreeMap::new();
    모은다(root, root, &mut out);
    out
}

fn 모은다(root: &Path, dir: &Path, out: &mut std::collections::BTreeMap<String, String>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        let rel = path.strip_prefix(root).unwrap_or(&path).display().to_string();
        if path.is_dir() {
            out.insert(rel, "<디렉터리>".to_owned());
            모은다(root, &path, out);
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
