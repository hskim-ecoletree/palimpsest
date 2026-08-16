//! **이름이 대소문자만 다른 것과 부딪히는가** — 파일시스템이 답을 다르게 낸다.
//!
//! # 왜 이 문이 필요한가
//!
//! 이 저장소의 다른 플랫폼 분기는 전부 **API 의 차이**였다(모드 비트 · 확장자 ·
//! 링크 수). 이것은 다르다 — **같은 코드가 같은 호출을 하는데 파일시스템이 다른
//! 답을 낸다.**
//!
//! | 사용자 프로젝트에 `Claude.md` 가 있고 우리가 `CLAUDE.md` 를 놓을 때 | 무엇이 일어나나 |
//! |---|---|
//! | Windows(NTFS) · macOS(APFS 기본) | **같은 파일이다.** 우리 블록이 `Claude.md` 에 들어가고 이름은 `Claude.md` 로 남는다 |
//! | 리눅스(ext4·btrfs) | **다른 파일이다.** `CLAUDE.md` 가 새로 생기고 `Claude.md` 는 그대로 |
//!
//! 실측(2026-08-17 · 이 기계 · NTFS): `Claude.md` 가 있는 방에서
//! `echo other > CLAUDE.md` 뒤 `ls` 는 여전히 `Claude.md` 하나이고 그 내용이 바뀐다.
//!
//! # ★ 둘 다 「그 플랫폼에서는」 맞다 — 그래서 더 나쁘다
//!
//! 어느 쪽도 결함이 아니다. Windows·macOS 에서 하네스도 대소문자를 안 가리므로
//! `Claude.md` 는 **정말로 그 사용자의 지시 파일**이고, 리눅스에서 그것은 하네스가
//! 안 읽는 **무관한 파일**이다. 그래서 각자의 자리에서는 둘 다 옳다.
//!
//! 무너지는 것은 **공유되는 저장소**다. 이 층의 소비자가 그것이다(ADR-0023):
//!
//! ```text
//! Windows 에서 pal install  →  Claude.md 에 블록이 들어가고 그대로 커밋된다
//! 리눅스에서 clone          →  pal doctor 가 CLAUDE.md 를 찾는다 → 없다 → 빨강
//!                              그리고 하네스도 그 지시를 안 읽는다
//! ```
//!
//! # 무엇을 골랐는가 — **양쪽이 할 수 있는 것**
//!
//! ADR-0023 이 정한 고르는 법은 *"플랫폼이 무언가를 못 볼 때 고를 축은 「볼 수 있는
//! 쪽」이 아니라 **양쪽이 할 수 있는 것**"* 이다.
//!
//! | 후보 | 가능한가 |
//! |---|---|
//! | 양쪽 다 **대소문자를 안 가린다** | **불가능.** 리눅스에서 `CLAUDE.md` 로 연 파일이 `Claude.md` 가 되게 만들 수 없다 |
//! | 양쪽 다 **있는 이름을 따른다** | 리눅스에서는 하네스가 안 읽는 파일에 쓰게 된다 — 설치가 **아무 효과가 없는 상태**를 rc=0 으로 낸다 |
//! | 양쪽 다 **멈추고 말한다** ← 고른 것 | 가능하다. 탐지가 [`std::fs::read_dir`] 위에 서고 그것은 어디서나 같은 답을 낸다 |
//!
//! ⚠ **잃는 것을 적어 둔다.** 리눅스에서 `Claude.md` 라는 **무관한 파일**을 가진
//! 사용자가 설치를 못 한다. 그 사람이 할 일은 `git mv` 한 번이고, 그 대신 얻는 것은
//! *"이 저장소를 세 플랫폼 중 어디서 clone 해도 같은 것이 선다"* 이다. 그리고 그
//! 사람의 저장소는 **우리와 무관하게 이미** Windows 동료에게서 깨져 있었다 — 이
//! 문은 그 사실을 설치 시점에 보이게 만들 뿐이다.
//!
//! # ⚠ ASCII 로만 접는다
//!
//! [`str::eq_ignore_ascii_case`] 를 쓴다. 우리가 놓는 이름은 전부 ASCII 이고
//! (`CLAUDE.md`·`.gitignore`·`.claude`…), 유니코드 대소문자 접기는 **파일시스템마다
//! 규칙이 다르다**(APFS 는 NFD 정규화까지 한다). 우리 이름 밖의 접기를 흉내 내면
//! 이 문이 무엇을 재는지 흐려지고, 흉내가 틀린 자리에서 **거짓 거부**가 난다.

use std::path::Path;

/// 이 자리에 **대소문자만 다른 이름**이 이미 있으면 그 이름.
///
/// 없거나, 정확히 같은 이름이 있거나, 부모를 못 읽으면 `None`.
///
/// ★ **정확히 같은 이름은 부딪힘이 아니다.** 대소문자를 안 가리는 파일시스템에서
/// `read_dir` 은 **디스크에 적힌 그대로**를 낸다(NTFS·APFS 는 대소문자를 보존한다).
/// 그래서 우리가 만든 `CLAUDE.md` 는 다음 회차에서도 `CLAUDE.md` 로 보이고 이 문을
/// 그냥 지난다 — 멱등이다.
#[must_use]
pub fn 대소문자만_다른_이름(path: &Path) -> Option<String> {
    let 이름 = path.file_name()?.to_str()?;
    let dir = path.parent()?;
    let entries = std::fs::read_dir(dir).ok()?;
    for e in entries.flatten() {
        let 있는 = e.file_name();
        let Some(있는) = 있는.to_str() else { continue };
        if 있는 != 이름 && 있는.eq_ignore_ascii_case(이름) {
            return Some(있는.to_owned());
        }
    }
    None
}

/// 부딪히면 사람에게 낼 **한 문장** — 없으면 `None`.
#[must_use]
pub fn 부딪힘(path: &Path) -> Option<String> {
    let 있는 = 대소문자만_다른_이름(path)?;
    let 우리 = path.file_name()?.to_string_lossy().into_owned();
    Some(format!(
        "{} 에 **대소문자만 다른 이름**이 이미 있다 — 우리는 `{우리}` 를 놓는데 거기 \
         `{있는}` 가 있다.\n    \
         이 상태의 답이 **파일시스템마다 다르다**: 대소문자를 안 가리는 곳\
         (Windows·macOS 기본)에서는 그 둘이 **같은 파일**이라 우리 블록이 `{있는}` 에 \
         들어가고, 가리는 곳(리눅스)에서는 **다른 파일**이라 `{우리}` 가 새로 생긴다.\n    \
         그러면 이 저장소를 어느 플랫폼에서 clone 했느냐에 따라 다른 것이 서고, \
         한쪽에서는 하네스가 그 지시를 아예 안 읽는다. **그래서 여기서 멈춘다.**\n    \
         `{있는}` 의 이름을 정하십시오 — 우리 것이면 `git mv` 로 `{우리}` 에 맞추고, \
         무관한 파일이면 다른 이름을 주십시오",
        super::winpath::사람이_읽는(path.parent().unwrap_or(path)),
    ))
}

#[cfg(test)]
mod tests {
    use super::{대소문자만_다른_이름, 부딪힘};

    fn 방(tag: &str) -> std::path::PathBuf {
        let d = std::env::temp_dir().join(format!("pal-casing-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).expect("방");
        d
    }

    /// ★ **어느 플랫폼에서나 같은 답을 낸다** — 그것이 이 모듈의 존재 이유다.
    ///
    /// 탐지가 `read_dir` 의 **이름 문자열** 위에 서기 때문에 파일시스템이 대소문자를
    /// 가리든 말든 답이 같다. 가리는 곳에서는 두 파일이 따로 있고, 안 가리는 곳에서는
    /// `Claude.md` 하나가 그 이름으로 보인다 — **두 경우 다 「다른 이름이 있다」**이다.
    #[test]
    fn 대소문자만_다르면_찾아낸다() {
        let d = 방("찾기");
        std::fs::write(d.join("Claude.md"), "남의 것\n").expect("쓰기");
        assert_eq!(대소문자만_다른_이름(&d.join("CLAUDE.md")), Some("Claude.md".to_owned()));
        assert!(부딪힘(&d.join("CLAUDE.md")).is_some_and(|s| s.contains("Claude.md")));
    }

    /// ★ **정확히 같은 이름은 부딪힘이 아니다** — 이 줄이 없으면 두 번째 설치가 막힌다.
    #[test]
    fn 정확히_같은_이름은_안_걸린다() {
        let d = 방("같음");
        std::fs::write(d.join("CLAUDE.md"), "우리 것\n").expect("쓰기");
        assert_eq!(대소문자만_다른_이름(&d.join("CLAUDE.md")), None);
        assert_eq!(부딪힘(&d.join("CLAUDE.md")), None);
    }

    /// 없는 자리와 못 읽는 부모는 조용하다 — 여기는 **판정하는 문이지 만드는 문이 아니다.**
    #[test]
    fn 없으면_조용하다() {
        let d = 방("없음");
        assert_eq!(대소문자만_다른_이름(&d.join("CLAUDE.md")), None);
        assert_eq!(대소문자만_다른_이름(&d.join("없는방/CLAUDE.md")), None);
    }

    /// ★ **무관한 이름은 안 걸린다.** 접두사가 같다고 걸면 `.gitignore` 가
    /// `.gitignore.bak` 에 걸린다 — 그러면 이 문이 거짓 거부를 낸다.
    #[test]
    fn 다른_이름은_안_걸린다() {
        let d = 방("무관");
        std::fs::write(d.join(".gitignore.bak"), "x\n").expect("쓰기");
        std::fs::write(d.join("CLAUDE.md.orig"), "x\n").expect("쓰기");
        assert_eq!(대소문자만_다른_이름(&d.join(".gitignore")), None);
        assert_eq!(대소문자만_다른_이름(&d.join("CLAUDE.md")), None);
    }
}
