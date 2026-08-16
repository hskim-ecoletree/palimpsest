//! 남의 파일에 **블록 하나를 더하고 그것만 되돌린다** — `.gitignore` · `CLAUDE.md`.
//!
//! # 실측으로 깨진 것들. 여기가 그것을 전부 막는 자리다
//!
//! | 깨진 형태 | 여기서 막는 방법 |
//! |---|---|
//! | 텍스트 필터로 재작성해 **NUL 바이트에서 사용자 줄이 잘렸다** | 줄로 안 읽는다. `Vec<u8>` 로 읽고 바이트로 이어 붙인다 |
//! | **모드가 600/755 → 644 로 소실**됐다 | 임시 파일 + rename 을 **안 쓴다.** 있는 파일을 열어 **제자리에** 쓴다 |
//! | **심링크가 일반 파일로 바뀌었다** | 같은 이유로 안 바뀐다 — 열어서 쓰면 링크 대상에 쓰인다 |
//! | **하드링크가 끊겼다** | 같은 이유로 안 끊긴다 — inode 가 그대로다 |
//! | **쓰기 실패를 안 검사해 거짓 성공(rc=0)** | 모든 쓰기가 `?` 를 지고, 마지막에 `sync_all` 까지 본다 |
//! | **끝 개행 없는 파일에 그냥 append 해 마지막 규칙과 우리 규칙이 둘 다 파괴** | 끝이 개행이 아니면 개행을 **먼저 넣고**, 넣었다는 사실을 매니페스트가 진다 |
//! | 마커 없이 내용 일치로 지워 **사용자가 먼저 써 둔 같은 줄을 지웠다** | 지우는 단위는 **우리가 넣은 바이트열 그대로**다. 한 줄씩 안 본다 |
//! | **stale 마커가 사용자가 나중에 만든 파일을 지웠다** | 파일을 지우는 조건은 *"우리가 만들었고"* **그리고** *"우리 블록을 뺀 나머지가 비었고"* 둘 다다 |
//!
//! # 손으로 고쳤으면 **고치려 들지 않는다**
//!
//! 우리가 넣은 바이트열이 그대로 안 보이면 제거를 **거부하고 사람에게 넘긴다.**
//! 마커를 다시 찾아 그 사이를 지우는 «복구»는 사용자가 그 안에 써 넣은 줄을 함께
//! 지운다 — 그것이 이 파일이 막으려는 바로 그 형태다.

use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use anyhow::{Context, Result, bail};

use super::layout::Markers;

/// 블록을 넣은 결과.
pub enum Added {
    /// 넣었다. 넣은 **바이트열 그대로**와, 파일을 우리가 만들었는지.
    Inserted { bytes: String, created: bool },
    /// 이미 우리 블록이 있다 — 아무것도 안 했다(**멱등**).
    AlreadyThere,
}

/// 여는 표식 · 본문 · 닫는 표식으로 블록 하나를 만든다. **끝에 개행이 있다.**
#[must_use]
pub fn compose(markers: &Markers, body: &[String]) -> String {
    let mut out = String::new();
    out.push_str(markers.begin);
    out.push('\n');
    for line in body {
        out.push_str(line);
        out.push('\n');
    }
    out.push_str(markers.end);
    out.push('\n');
    out
}

/// 파일 끝에 블록을 더한다.
///
/// # Errors
/// 읽지 못하거나 쓰지 못하면. **쓰기 실패를 삼키지 않는다.**
pub fn add(path: &Path, markers: &Markers, block: &str) -> Result<Added> {
    if !path.exists() {
        write_new(path, block.as_bytes())?;
        return Ok(Added::Inserted { bytes: block.to_owned(), created: true });
    }

    let existing = std::fs::read(path)
        .with_context(|| format!("읽지 못했다: {}", path.display()))?;
    if find(&existing, markers.begin.as_bytes()).is_some() {
        return Ok(Added::AlreadyThere);
    }

    // **끝 개행이 없으면 개행을 먼저 넣는다.** 안 넣으면 사용자의 마지막 규칙과 우리
    // 첫 줄이 한 줄로 붙어 **둘 다 파괴된다**(실측).
    let needs_newline = !existing.is_empty() && !existing.ends_with(b"\n");
    let inserted = if needs_newline { format!("\n{block}") } else { block.to_owned() };

    let mut next = existing;
    next.extend_from_slice(inserted.as_bytes());
    write_in_place(path, &next)?;
    Ok(Added::Inserted { bytes: inserted, created: false })
}

/// 블록을 뺀 결과.
pub enum Removed {
    /// 뺐다.
    Removed,
    /// 파일을 통째로 지웠다 — 우리가 만들었고 나머지가 비었다.
    FileGone,
    /// 파일이 이미 없다.
    Missing,
}

/// 우리가 넣은 바이트열 **그대로**를 뺀다.
///
/// # Errors
/// 파일은 있는데 그 바이트열이 안 보이면 — **손으로 고쳐졌다.** 고치려 들지 않고
/// 거부한다.
pub fn remove(path: &Path, inserted: &str, created: bool) -> Result<Removed> {
    if !path.exists() {
        return Ok(Removed::Missing);
    }
    let existing = std::fs::read(path)
        .with_context(|| format!("읽지 못했다: {}", path.display()))?;
    let Some(at) = find(&existing, inserted.as_bytes()) else {
        bail!(
            "{} 의 palimpsest 블록이 우리가 넣은 것과 다르다 — **손으로 고쳐졌거나 마커가 \
             훼손됐다.** 고치려 들지 않는다: 그 블록을 손으로 지운 뒤 다시 돌리십시오",
            path.display()
        );
    };

    let mut next = existing;
    next.drain(at..at + inserted.len());

    if created && next.is_empty() {
        std::fs::remove_file(path)
            .with_context(|| format!("지우지 못했다: {}", path.display()))?;
        return Ok(Removed::FileGone);
    }
    write_in_place(path, &next)?;
    Ok(Removed::Removed)
}

/// 우리 블록이 지금 파일에 있는가 — **제거 전 검증에 쓴다.**
///
/// # Errors
/// 읽지 못하면.
pub fn present(path: &Path, inserted: &str) -> Result<bool> {
    if !path.exists() {
        return Ok(false);
    }
    let existing = std::fs::read(path)
        .with_context(|| format!("읽지 못했다: {}", path.display()))?;
    Ok(find(&existing, inserted.as_bytes()).is_some())
}

/// 바이트열 안에서 바늘의 첫 자리.
fn find(hay: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() || needle.len() > hay.len() {
        return None;
    }
    (0..=hay.len() - needle.len()).find(|&i| &hay[i..i + needle.len()] == needle)
}

/// 없던 파일을 만든다. **부모 디렉터리는 부르는 쪽이 만든다.**
fn write_new(path: &Path, bytes: &[u8]) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .with_context(|| format!("만들지 못했다: {}", path.display()))?;
    finish(&mut file, bytes, path)
}

/// **있는 파일에 제자리로 쓴다** — 모드·심링크·하드링크가 살아 있는 유일한 길.
pub fn write_in_place(path: &Path, bytes: &[u8]) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .open(path)
        .with_context(|| format!("쓰려고 열지 못했다: {}", path.display()))?;
    file.set_len(bytes.len() as u64)
        .with_context(|| format!("길이를 맞추지 못했다: {}", path.display()))?;
    finish(&mut file, bytes, path)
}

/// 쓰고 **끝까지 확인한다.** 여기서 `?` 를 빼면 쓰기 불가 디렉터리에서 rc=0 이 난다.
fn finish(file: &mut std::fs::File, bytes: &[u8], path: &Path) -> Result<()> {
    file.write_all(bytes).with_context(|| format!("쓰지 못했다: {}", path.display()))?;
    file.flush().with_context(|| format!("비우지 못했다: {}", path.display()))?;
    file.sync_all().with_context(|| format!("동기화하지 못했다: {}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::{Added, Removed, add, compose, remove};
    use crate::install::layout::IGNORE_MARKERS;

    fn 방(tag: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir()
            .join(format!("pal-blocks-{tag}-{}-{:?}", std::process::id(), std::thread::current().id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("방");
        dir
    }

    fn 블록() -> String {
        compose(&IGNORE_MARKERS, &["/x/".to_owned()])
    }

    /// **왕복이 바이트 동일**이다 — 이 시험이 이 파일의 하중 전부를 진다.
    #[test]
    fn 왕복하면_바이트가_같다() {
        let dir = 방("왕복");
        for 원본 in [
            &b"a\nb\n"[..],
            // 끝 개행 없음 — 실측에서 마지막 규칙과 우리 규칙이 둘 다 깨진 자리.
            &b"a\nb"[..],
            // NUL 바이트 — 텍스트 필터가 사용자 줄을 자른 자리.
            &b"a\n\0\xff\xfe\nb\n"[..],
            &b""[..],
        ] {
            let path = dir.join("f");
            std::fs::write(&path, 원본).expect("원본");
            let Added::Inserted { bytes, created } = add(&path, &IGNORE_MARKERS, &블록()).expect("더하기")
            else {
                panic!("이미 있다고 나왔다");
            };
            assert!(!created, "있는 파일을 새로 만들었다고 적었다");
            assert!(std::fs::read(&path).expect("읽기").starts_with(원본), "사용자 바이트가 깨졌다");
            remove(&path, &bytes, false).expect("빼기");
            assert_eq!(std::fs::read(&path).expect("읽기"), 원본, "왕복이 원본과 다르다");
        }
    }

    #[test]
    fn 두_번_더해도_블록은_하나다() {
        let dir = 방("멱등");
        let path = dir.join("f");
        std::fs::write(&path, b"a\n").expect("원본");
        add(&path, &IGNORE_MARKERS, &블록()).expect("첫 번째");
        let 둘째 = add(&path, &IGNORE_MARKERS, &블록()).expect("두 번째");
        assert!(matches!(둘째, Added::AlreadyThere));
        let text = std::fs::read_to_string(&path).expect("읽기");
        assert_eq!(text.matches(IGNORE_MARKERS.begin).count(), 1);
    }

    /// **없던 파일은 우리가 만들었고, 나머지가 비면 지운다.**
    #[test]
    fn 우리가_만든_파일은_통째로_사라진다() {
        let dir = 방("생성");
        let path = dir.join("새것");
        let Added::Inserted { bytes, created } = add(&path, &IGNORE_MARKERS, &블록()).expect("더하기")
        else {
            panic!("이미 있다고 나왔다");
        };
        assert!(created);
        assert!(matches!(remove(&path, &bytes, created).expect("빼기"), Removed::FileGone));
        assert!(!path.exists());
    }

    /// ★ **stale 마커가 사용자가 나중에 만든 파일을 지운 형태** — 여기서 막힌다.
    #[test]
    fn 사용자가_나중에_만든_파일을_안_지운다() {
        let dir = 방("stale");
        let path = dir.join("나중것");
        let Added::Inserted { bytes, created } = add(&path, &IGNORE_MARKERS, &블록()).expect("더하기")
        else {
            panic!("이미 있다고 나왔다");
        };
        // 사용자가 우리 파일을 지우고 자기 것을 새로 썼다.
        std::fs::remove_file(&path).expect("지우기");
        let 사용자_것 = "사용자가 쓴 것\n";
        std::fs::write(&path, 사용자_것).expect("사용자 파일");

        // 우리 바이트열이 안 보이므로 **거부한다** — 지우지 않는다.
        assert!(remove(&path, &bytes, created).is_err());
        assert_eq!(std::fs::read_to_string(&path).expect("읽기"), 사용자_것);
    }

    /// 블록이 **손으로 고쳐졌으면** 고치려 들지 않고 거부한다.
    #[test]
    fn 손으로_고친_블록은_거부한다() {
        let dir = 방("훼손");
        let path = dir.join("f");
        std::fs::write(&path, b"a\n").expect("원본");
        let Added::Inserted { bytes, .. } = add(&path, &IGNORE_MARKERS, &블록()).expect("더하기")
        else {
            panic!("이미 있다고 나왔다");
        };
        let 훼손 = std::fs::read_to_string(&path).expect("읽기").replace("/x/", "/y/");
        std::fs::write(&path, &훼손).expect("훼손");
        assert!(remove(&path, &bytes, false).is_err());
        assert_eq!(std::fs::read_to_string(&path).expect("읽기"), 훼손, "거부했는데 파일이 바뀌었다");
    }

    /// ★ **사용자가 우리보다 먼저 써 둔 같은 줄을 안 지운다.**
    #[test]
    fn 우리_블록_밖의_같은_줄은_남는다() {
        let dir = 방("같은줄");
        let path = dir.join("f");
        std::fs::write(&path, b"/x/\n").expect("원본");
        let Added::Inserted { bytes, .. } = add(&path, &IGNORE_MARKERS, &블록()).expect("더하기")
        else {
            panic!("이미 있다고 나왔다");
        };
        remove(&path, &bytes, false).expect("빼기");
        assert_eq!(std::fs::read(&path).expect("읽기"), b"/x/\n");
    }
}
