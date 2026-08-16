//! **파일에 손대기 전에 서는 문** — 경로가 아니라 **그 자리의 정체**를 본다.
//!
//! # [`super::inside`] 와 무엇이 다른가
//!
//! `inside` 는 *"이 경로가 대상 안인가"* 를 판정한다. 여기는 *"그 자리가 우리가 다뤄도
//! 되는 종류의 것인가"* 를 판정한다. 둘은 **다른 축**이고, 실측이 그 사실을 정했다:
//!
//! | 형태 | `inside` 가 잡나 | 왜 |
//! |---|---|---|
//! | `.claude → ~/.claude` 심링크 | **잡는다** | `canonicalize` 가 밖이라는 신원을 낸다 |
//! | 대상 안의 파일이 **밖의 파일과 하드링크** | **못 잡는다** | 하드링크에는 「밖」이라는 신원이 **원리상 없다** |
//!
//! 실측: 밖의 `victim.txt` 가 `e3b0c442…`(0바이트)가 됐고 **rc=0** 이었다.
//!
//! # ★ 유닉스 전용 가정 — 이 파일에 하나 있다
//!
//! 소유자 결정(2026-08-16): *"windows 를 대응한다는 가정하에 앞으로 모든 설계와 개발이
//! 되어야 해."* 그래서 **어디가 유닉스 전용인지를 지금 적어 둔다.**
//!
//! [`제자리에_써도_되나`] 의 링크 수 세기는 `std::os::unix::fs::MetadataExt::nlink`
//! 위에 선다. **`#[cfg(unix)]` 밖에서는 이 검사가 아무것도 안 한다** — Windows 에도
//! NTFS 하드링크와 `GetFileInformationByHandle` 의 `nNumberOfLinks` 라는 등가 개념이
//! 있지만 std 에 그 문이 없다. 지금 분기를 만들지 않는다. 대신
//! `tests/install_hostile.rs` 가 **다른 플랫폼에서 시끄럽게 실패하는 짝**을 지고 있어,
//! 이 방어가 없는 플랫폼에서 그 사실이 조용히 묻히지 않는다.

use std::path::Path;

use anyhow::{Context, Result, bail};

/// **우리가 읽고 쓰는 자리는 일반 파일이거나 없거나 둘 중 하나다.**
///
/// # 왜 종류를 먼저 묻는가
///
/// `std::fs::read` 는 **여는 순간 매달린다.** writer 없는 FIFO 를 열면 `open(2)` 이
/// 거기서 잠기고, 시간 상한도 취소도 없다. 실측: `.claude/settings.json` 을 FIFO 로
/// 두면 `pal install` 과 `pal doctor` 가 **영원히** 매달렸다.
///
/// `.claude/pal` 이 FIFO 인 경우는 `create_dir_all` 이 이미 막고 있었다 — **같은
/// 규율을 나머지 자리에도 세운다.**
///
/// ⚠ 이 문은 `stat` 이라 안 매달린다. 그리고 [`std::fs::metadata`] 는 심링크를
/// 따라가므로, 「안을 가리키는 심링크」는 그 **대상의 종류**로 판정된다 — 경계는
/// [`super::inside::Root::join`] 이 이미 봤다.
///
/// # Errors
/// 그 자리가 있는데 일반 파일이 아니면.
pub fn 일반_파일이거나_없나(path: &Path) -> Result<()> {
    let Ok(meta) = std::fs::metadata(path) else { return Ok(()) };
    if meta.is_file() {
        return Ok(());
    }
    bail!(
        "{} 가 **일반 파일이 아니다**({}) — 읽지도 쓰지도 않는다.\n    \
         이름 있는 파이프·장치·소켓을 열면 그 자리에서 **매달리고**(실측: 영원히), \
         디렉터리면 우리가 쓸 자리가 아예 없다. 사람이 봐야 한다",
        path.display(),
        종류(&meta)
    );
}

fn 종류(meta: &std::fs::Metadata) -> &'static str {
    if meta.is_dir() {
        return "디렉터리다";
    }
    #[cfg(unix)]
    {
        // ⚠ **유닉스 전용 가정** — `FileTypeExt` 가 가르는 넷은 유닉스의 종류다.
        use std::os::unix::fs::FileTypeExt;
        let t = meta.file_type();
        if t.is_fifo() {
            return "이름 있는 파이프(FIFO)다";
        }
        if t.is_socket() {
            return "소켓이다";
        }
        if t.is_block_device() || t.is_char_device() {
            return "장치 파일이다";
        }
    }
    "일반 파일이 아니다"
}

/// 종류를 먼저 묻고 읽는다. **읽는 자리는 전부 이 문을 지난다.**
///
/// # Errors
/// 일반 파일이 아니거나 못 읽으면.
pub fn 읽는다(path: &Path) -> Result<Vec<u8>> {
    일반_파일이거나_없나(path)?;
    std::fs::read(path).with_context(|| format!("읽지 못했다: {}", path.display()))
}

/// 종류를 먼저 묻고 통째로 쓴다(없으면 만든다).
///
/// # Errors
/// 일반 파일이 아니거나 못 쓰면.
pub fn 쓴다(path: &Path, bytes: &[u8]) -> Result<()> {
    일반_파일이거나_없나(path)?;
    std::fs::write(path, bytes).with_context(|| format!("쓰지 못했다: {}", path.display()))
}

/// **제자리 쓰기를 해도 되는 자리인가.**
///
/// # 무엇을 이기게 했는가
///
/// 제자리 쓰기를 고른 이유는 *"안쪽 심링크·모드·하드링크를 살린다"* 였다
/// ([`super::blocks`] 머리말). 그런데 하드링크를 살리는 것과 **하드링크를 통해 밖이
/// 새는 것**은 같은 동작의 앞뒷면이다 — 링크된 파일에 제자리로 쓰면 그 바이트가
/// 저쪽에도 그대로 간다.
///
/// **밖으로 새는 것을 막는 쪽을 이기게 했다.** 링크 수가 1 이 아니면 고치지 않고
/// 멈춘다. 잃는 것은 *"하드링크된 `CLAUDE.md` 도 편집해 준다"* 이고, 지키는 것은
/// *"어느 경로에서도 대상 바깥을 안 건드린다"*(`[f24]` ⑦)이다. 후자가 게이트의
/// 가장 센 줄이다.
///
/// # Errors
/// 그 자리가 하드링크로 여러 이름을 지고 있으면.
#[cfg_attr(not(unix), expect(clippy::unnecessary_wraps, reason = "unix 밖에서는 셀 수가 없다"))]
pub fn 제자리에_써도_되나(path: &Path) -> Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        let Ok(meta) = std::fs::metadata(path) else { return Ok(()) };
        if meta.is_file() && meta.nlink() > 1 {
            bail!(
                "{} 에 **하드링크가 걸려 있다**(이름 {}개) — 제자리에서 안 고친다.\n    \
                 하드링크에는 「대상 밖」이라는 신원이 **원리상 없다.** 심링크는 \
                 `canonicalize` 가 풀어 막지만 이것은 못 막는다 — 그래서 여기서 멈춘다.\n    \
                 그 파일의 링크를 끊은 뒤(`cp` 로 복사해 갈아끼우기) 다시 돌리십시오",
                path.display(),
                meta.nlink()
            );
        }
    }
    let _ = path;
    Ok(())
}
