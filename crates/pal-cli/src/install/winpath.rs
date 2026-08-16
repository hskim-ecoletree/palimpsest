//! **`\\?\` 를 사람 앞에 안 낸다** — 그러나 그것이 필요한 자리에서는 안 벗긴다.
//!
//! # 무엇이 새고 있었나 (실측 2026-08-16)
//!
//! ```text
//! ■ 설치 — \\?\C:\Users\me\proj
//! 훅 등록  SubagentStop  ·  \\?\C:\tools\pal.exe hook SubagentStop
//! ```
//!
//! [`std::fs::canonicalize`] 는 Windows 에서 **verbatim 형태**(`\\?\`)를 낸다. 우리는
//! 대상 뿌리([`super::inside::Root`])와 훅에 등록할 실행 파일([`super::hooks`]) 둘 다
//! 정규화하므로, 그 접두사가 **화면과 `settings.json` 에 그대로 실렸다.**
//!
//! **동작은 한다**(실측: 그 형태로 등록된 훅이 정상 발화한다). 문제는 다른 것이다:
//!
//! - `settings.json` 은 **커밋되고 사람이 읽는 파일**이다. `\\?\` 는 `canonicalize` 의
//!   내부 사정이지 사용자에게 뜻이 있는 글자가 아니다 — 그런데 그 사람이 그 줄을
//!   복사해서 쓰거나 손으로 고칠 수 있다.
//! - 화면의 한 줄이 유닉스와 **다르게 생긴다.** 이 회차의 문장은 *"같은 결과를 낸다"* 다.
//!
//! # ★ 그런데 왜 조건부인가 — 벗기면 잃는 것이 있다
//!
//! `\\?\` 는 장식이 아니다. **`MAX_PATH`(260) 제한을 푸는 유일한 표기**다. 길이가 그
//! 한계를 넘는 자리에서 접두사를 벗기면 **동작하던 경로가 안 열린다** — 그러니 「보기
//! 좋게」가 「안 돌게」가 되면 안 된다.
//!
//! 그래서 규칙 하나: **벗긴 결과가 `MAX_PATH` 안에 들어올 때만 벗긴다.** 흔한 경로는
//! 전부 깨끗해지고, 긴 경로는 능력을 그대로 지킨다.
//!
//! # 무엇을 안 골랐는가
//!
//! | 후보 | 왜 안 골랐나 |
//! |---|---|
//! | 화면만 벗기고 등록 문자열은 verbatim 으로 둔다 | **두 문자열이 갈린다.** 사람이 화면에서 본 것과 파일에 적힌 것이 다르면, 그 차이를 설명할 자리가 어디에도 없다 |
//! | 길이와 무관하게 언제나 벗긴다 | 260 을 넘는 자리에서 **동작을 깬다.** 보기 좋자고 깨는 것은 거래가 안 된다 |
//! | 아예 `canonicalize` 를 안 한다 | 그것이 심링크 경계 방어의 근거다(`Root`). 못 뺀다 |

use std::path::Path;

/// Windows 의 전통적 경로 길이 한계. 이것을 넘는 자리에서만 `\\?\` 가 필요하다.
#[cfg(windows)]
const MAX_PATH: usize = 260;

/// 사람에게 낼 경로 문자열 — **`\\?\` 를 뗀다(뗄 수 있을 때만).**
///
/// 화면과 `settings.json` 의 `command` 가 **같은 함수를 지난다.** 둘이 갈리면 사람이
/// 본 것과 파일에 적힌 것이 달라진다.
#[must_use]
pub fn 사람이_읽는(path: &Path) -> String {
    #[cfg(windows)]
    {
        let s = path.to_string_lossy();
        // `\\?\UNC\server\share` → `\\server\share`
        if let Some(rest) = s.strip_prefix(r"\\?\UNC\") {
            let 벗긴 = format!(r"\\{rest}");
            return if 벗긴.len() < MAX_PATH { 벗긴 } else { s.into_owned() };
        }
        // `\\?\C:\...` → `C:\...`. **드라이브 형태일 때만** 뗀다 — `\\?\Volume{…}` 같은
        // 것은 접두사가 표기의 일부라 떼면 다른 경로가 된다.
        if let Some(rest) = s.strip_prefix(r"\\?\") {
            let 드라이브 = rest.as_bytes().first().is_some_and(u8::is_ascii_alphabetic)
                && rest.as_bytes().get(1) == Some(&b':');
            if 드라이브 && rest.len() < MAX_PATH {
                return rest.to_owned();
            }
        }
        s.into_owned()
    }
    #[cfg(not(windows))]
    {
        path.to_string_lossy().into_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::사람이_읽는;
    use std::path::Path;

    /// 유닉스에서는 아무것도 안 바뀐다 — 그리고 어느 플랫폼에서도 평범한 경로는 그대로다.
    #[test]
    fn 평범한_경로는_그대로다() {
        for p in ["/usr/local/bin/pal", "relative/path"] {
            assert_eq!(사람이_읽는(Path::new(p)), p);
        }
    }

    #[test]
    #[cfg(windows)]
    fn verbatim_을_뗀다() {
        assert_eq!(사람이_읽는(Path::new(r"\\?\C:\tools\pal.exe")), r"C:\tools\pal.exe");
        assert_eq!(사람이_읽는(Path::new(r"\\?\UNC\srv\share\p")), r"\\srv\share\p");
        // 이미 평범한 것은 안 건드린다.
        assert_eq!(사람이_읽는(Path::new(r"C:\tools\pal.exe")), r"C:\tools\pal.exe");
    }

    /// ★ **길면 안 뗀다.** 이 줄이 없으면 「보기 좋게」가 「안 돌게」가 된다.
    #[test]
    #[cfg(windows)]
    fn max_path_를_넘으면_안_뗀다() {
        let 긴것 = format!(r"\\?\C:\{}", "a".repeat(super::MAX_PATH));
        assert_eq!(사람이_읽는(Path::new(&긴것)), 긴것, "긴 경로에서 접두사를 뗐다");
    }

    /// 드라이브 형태가 아닌 verbatim 은 접두사가 표기의 일부다 — 안 뗀다.
    #[test]
    #[cfg(windows)]
    fn 볼륨_guid_형태는_안_뗀다() {
        let v = r"\\?\Volume{00000000-0000-0000-0000-000000000000}\x";
        assert_eq!(사람이_읽는(Path::new(v)), v);
    }
}
