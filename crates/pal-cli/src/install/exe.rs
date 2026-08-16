//! **「이 자리가 실행될 수 있는가」** — 플랫폼이 답을 다르게 낸다.
//!
//! # 왜 한 자리에 모으는가
//!
//! 이 물음을 묻는 자리가 둘인데 **둘 다 유닉스의 답만 알고 있었다**:
//!
//! | 자리 | 무엇을 물었나 | Windows 에서 무엇이 됐나 |
//! |---|---|---|
//! | [`super::doctor`] 검사 4 | `PATH` 안에 `pal` 이 있는가 | `dir.join("pal")` — 확장자가 없다. **정상 설치가 빨강**이고 실행조차 안 되는 배치만 초록이었다 |
//! | [`super::hooks::실행할_수_있나`] | 등록된 자리가 실행될 수 있는가 | 모드 비트 검사가 `cfg(unix)` 안이라 **한 겹이 통째로 빠졌다** |
//!
//! 둘은 **같은 축**이다. 유닉스에서 「실행될 수 있는가」를 정하는 것은 모드 비트이고,
//! Windows 에서 그것을 정하는 것은 **확장자**다. 그러니 분기는 호출자마다가 아니라
//! **여기 한 번**이어야 한다 — 두 곳에서 각자 답하면 한쪽이 조용히 낡는다.
//!
//! # 실측이 규칙을 정했다 (2026-08-16 · Windows 11 · Claude Code 2.1.233)
//!
//! - `PATH` 에 `pal.exe` 를 두면 그 이름으로 돈다. **확장자 없는 사본 `pal` 은
//!   `Executable not found`** — 파일은 거기 있고 읽히는데 **OS 가 안 띄운다.**
//! - 등록된 절대 경로에 확장자가 없으면 **`.exe` 를 붙여서 찾는다** — 옆에 `.exe` 가
//!   있으면 그것이 뜨고, 없으면 못 뜬다(`CreateProcess` 의 규칙 그대로).
//!
//! ★ **이것이 유닉스의 `chmod -x` 와 정확히 같은 사건이다** — 파일은 있고 바이트도
//! 우리 것인데 **하네스가 그것을 못 띄우고, 그 실패를 완전히 삼킨다.** 그래서 이
//! 겹을 「이 플랫폼에는 없다」로 두면 안 된다. 축이 다를 뿐 겹은 있다.
//!
//! # 무엇을 안 골랐는가
//!
//! | 후보 | 왜 안 골랐나 |
//! |---|---|
//! | `EXE_SUFFIX` 하나만 본다 | 짧지만 **셸이 실제로 하는 일이 아니다.** `pal.cmd` 래퍼로 설치한 사람이 거짓 빨강을 본다 |
//! | ACL 로 실행 권한을 본다 | std 에 문이 없고, `unsafe 금지` 게이트가 raw FFI 를 막는다. **없는 것을 근거로 삼지 않는다** — 대신 [`권한_겹`] 이 그 부재를 화면에 낸다 |
//! | 실제로 띄워 본다 | 남이 커밋해 보낸 문자열을 돌리는 일이다. 이 파일의 모든 문은 **`stat` 까지만** 간다 |

use std::path::{Path, PathBuf};

/// `PATHEXT` 가 없을 때 Windows 가 쓰는 기본값.
///
/// 레지스트리 기본은 이보다 길지만(`.VBS`·`.JS`·`.WSF`…) **스크립트 호스트 확장자는
/// 안 넣는다** — 우리가 찾는 것은 `pal` 이라는 프로그램이고, 그것이 `.vbs` 로 설치되는
/// 경로는 없다. 목록을 넓히면 이 검사가 무엇을 재는지 흐려진다.
#[cfg(windows)]
const 기본_PATHEXT: &str = ".COM;.EXE;.BAT;.CMD";

/// 확장자 없는 이름에 OS 가 붙이는 것 — `CreateProcess` 는 **`.exe` 하나만** 붙인다.
#[cfg(windows)]
const 암묵_확장자: &str = "exe";

/// 이 플랫폼에서 실행으로 인정되는 확장자들 — 전부 소문자.
#[cfg(windows)]
fn 실행_확장자들() -> Vec<String> {
    let raw = std::env::var("PATHEXT").unwrap_or_else(|_| 기본_PATHEXT.to_owned());
    raw.split(';')
        .map(|e| e.trim().trim_start_matches('.').to_ascii_lowercase())
        .filter(|e| !e.is_empty())
        .collect()
}

/// `dir` 안에서 `stem` 이라는 **명령 이름**으로 실제로 도는 파일.
///
/// 유닉스는 이름 그대로이고, Windows 는 `PATHEXT` 를 붙여 본다 — 셸이 하는 일 그대로다.
/// `PATH` 를 훑는 쪽([`super::doctor`] 검사 4)이 쓴다.
#[must_use]
pub fn 명령을_찾는다(dir: &Path, stem: &str) -> Option<PathBuf> {
    #[cfg(windows)]
    {
        for ext in 실행_확장자들() {
            let c = dir.join(format!("{stem}.{ext}"));
            if c.is_file() {
                return Some(c);
            }
        }
        // ★ **확장자 없는 파일은 일부러 안 받는다.** 거기 있어도 OS 가 안 띄운다 —
        // 그것을 초록으로 세면 사용자가 「돈다」고 믿는 배치가 실제로는 안 돈다.
        None
    }
    #[cfg(not(windows))]
    {
        let c = dir.join(stem);
        if 모드가_실행을_허락하나(&c) { Some(c) } else { None }
    }
}

/// 이 **자리**(절대 경로)를 띄우면 실제로 열리는 파일 — 안 열리면 `None`.
///
/// [`명령을_찾는다`] 와 다른 함수인 이유: 여기 오는 것은 **이미 경로**이고, 하네스는
/// 그것을 셸 없이(exec form) 띄운다. 그러면 `PATHEXT` 탐색이 아니라 `CreateProcess`
/// 의 규칙이 적용된다 — **확장자가 없을 때 `.exe` 하나만 붙인다.**
#[must_use]
pub fn 자리가_열리나(path: &Path) -> Option<PathBuf> {
    #[cfg(windows)]
    {
        match path.extension() {
            // 확장자가 없다 — OS 가 `.exe` 를 붙인다.
            None => {
                let c = path.with_extension(암묵_확장자);
                if c.is_file() { Some(c) } else { None }
            }
            Some(ext) => {
                let ext = ext.to_string_lossy().to_ascii_lowercase();
                if !실행_확장자들().contains(&ext) {
                    return None;
                }
                if path.is_file() { Some(path.to_path_buf()) } else { None }
            }
        }
    }
    #[cfg(not(windows))]
    {
        if 모드가_실행을_허락하나(path) { Some(path.to_path_buf()) } else { None }
    }
}

#[cfg(not(windows))]
fn 모드가_실행을_허락하나(path: &Path) -> bool {
    let Ok(meta) = std::fs::metadata(path) else { return false };
    if !meta.is_file() {
        return false;
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        return meta.permissions().mode() & 0o111 != 0;
    }
    // 유닉스도 Windows 도 아닌 자리 — 있고 일반 파일이면 거기까지다.
    #[cfg(not(unix))]
    true
}

/// 이 자리가 왜 안 열리는지 — 사람에게 낼 **한 문장**.
///
/// 「없다」와 「있는데 안 열린다」를 가른다. 그 둘은 사용자가 할 일이 다르다.
#[must_use]
pub fn 안_열리는_까닭(path: &Path) -> String {
    #[cfg(windows)]
    {
        if path.extension().is_none() {
            format!(
                "확장자가 없고 옆에 `{}` 도 없다 — Windows 는 이 이름을 안 띄운다",
                path.with_extension(암묵_확장자).display()
            )
        } else {
            format!(
                "확장자가 실행 확장자가 아니다({}) — `PATHEXT` 에 없는 것은 안 뜬다",
                실행_확장자들().join(" · ")
            )
        }
    }
    #[cfg(not(windows))]
    {
        let _ = path;
        "실행 권한이 없다".to_owned()
    }
}

/// 이 플랫폼에서 **못 재는 겹**이 있으면 그 이름 — 없으면 `None`.
///
/// ★ 침묵하지 않기 위한 자리다. Windows 에서는 확장자까지만 보고 **ACL 은 못 본다**
/// (std 에 문이 없고 `unsafe 금지` 게이트가 raw FFI 를 막는다). 그 사실을 화면에 안
/// 내면 사용자는 이 검사가 재지 않는 것을 쟀다고 믿는다 — `pal doctor` 가 사각지대를
/// 띄우는 규율([`super::doctor`] 의 `남의_에이전트`)과 같은 형태다.
// **`Option` 은 플랫폼마다 한쪽으로 접힌다.** 한 플랫폼만 보면 언제나 `Some` 이거나
// 언제나 `None` 이라 clippy 가 「감쌀 필요 없다」고 한다 — 그런데 감싼 이유가 정확히
// **플랫폼마다 답이 다르다**는 것이다. 빈 문자열로 「없음」을 나타내면 호출자가 그
// 약속을 기억해야 하고, 그 기억이 다음 플랫폼에서 깨진다.
#[allow(clippy::unnecessary_wraps)]
#[must_use]
pub fn 못_재는_겹() -> Option<&'static str> {
    #[cfg(windows)]
    {
        Some("ACL — 확장자까지만 본다. 실행이 ACL 로 막힌 자리는 이 검사가 못 본다")
    }
    #[cfg(not(windows))]
    {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{명령을_찾는다, 자리가_열리나};

    /// 없는 자리는 어느 플랫폼에서도 `None` 이다.
    #[test]
    fn 없는_자리는_안_열린다() {
        let 방 = std::env::temp_dir().join(format!("pal-exe-{}", std::process::id()));
        let _ = std::fs::create_dir_all(&방);
        assert!(자리가_열리나(&방.join("없다")).is_none());
        assert!(명령을_찾는다(&방, "없다").is_none());
        // 디렉터리는 실행 파일이 아니다.
        assert!(자리가_열리나(&방).is_none());
    }

    /// ★ **지금 도는 이 실행 파일은 열린다.** 이 줄이 없으면 위가 공짜로 통과한다 —
    /// 언제나 `None` 을 내는 구현도 저 시험을 지난다.
    #[test]
    fn 지금_도는_것은_열린다() {
        let me = std::env::current_exe().expect("current_exe");
        assert!(자리가_열리나(&me).is_some(), "지금 도는 실행 파일이 안 열린다고 나왔다: {}", me.display());
    }

    /// ★ **확장자가 갈라야 하는 자리** — Windows 에서만 판정한다.
    ///
    /// 유닉스에는 이 축이 없다. 짝을 안 달면 이 시험이 한쪽에서 아무것도 안 잰다.
    #[test]
    #[cfg(windows)]
    fn 확장자가_없으면_안_열린다() {
        let 방 = std::env::temp_dir().join(format!("pal-exe-확장자-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&방);
        std::fs::create_dir_all(&방).expect("방");

        let 확장자없음 = 방.join("pal");
        std::fs::write(&확장자없음, b"MZ").expect("쓰기");
        assert!(자리가_열리나(&확장자없음).is_none(), "확장자 없는 파일이 열린다고 나왔다");
        assert!(명령을_찾는다(&방, "pal").is_none(), "확장자 없는 파일이 명령으로 잡혔다");

        // 옆에 `.exe` 가 생기면 **그 이름이 열린다** — 실측한 조건부 동작 그대로다.
        std::fs::write(방.join("pal.exe"), b"MZ").expect("쓰기");
        assert_eq!(자리가_열리나(&확장자없음), Some(방.join("pal.exe")));
        assert_eq!(명령을_찾는다(&방, "pal"), Some(방.join("pal.exe")));

        // 실행 확장자가 아닌 것은 안 열린다.
        let 텍스트 = 방.join("pal.txt");
        std::fs::write(&텍스트, b"MZ").expect("쓰기");
        assert!(자리가_열리나(&텍스트).is_none(), "`.txt` 가 열린다고 나왔다");
    }

    /// ★ 유닉스 쪽 짝 — **모드 비트가 갈라야 한다.**
    #[test]
    #[cfg(unix)]
    fn 실행_비트가_없으면_안_열린다() {
        use std::os::unix::fs::PermissionsExt;
        let 방 = std::env::temp_dir().join(format!("pal-exe-모드-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&방);
        std::fs::create_dir_all(&방).expect("방");

        let p = 방.join("pal");
        std::fs::write(&p, b"#!/bin/sh\n").expect("쓰기");
        std::fs::set_permissions(&p, std::fs::Permissions::from_mode(0o644)).expect("chmod");
        assert!(자리가_열리나(&p).is_none(), "실행 비트가 없는데 열린다고 나왔다");
        assert!(명령을_찾는다(&방, "pal").is_none());

        std::fs::set_permissions(&p, std::fs::Permissions::from_mode(0o755)).expect("chmod");
        assert_eq!(자리가_열리나(&p), Some(p.clone()));
        assert_eq!(명령을_찾는다(&방, "pal"), Some(p));
    }
}
