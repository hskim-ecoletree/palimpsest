//! 줄바꿈 — **대조는 맞춰서, 되쓰기는 있던 대로.**
//!
//! # 소유자 결정 (2026-08-16)
//!
//! > **줄바꿈을 정규화해서 비교한다.**
//!
//! `git -c core.autocrlf=true clone` 으로 받은 워킹트리에서 우리 파일 다섯이 전부 CRLF
//! 가 되고, 그러면 `doctor` 는 다섯을 전부 빨갛게 내고 `install` 재실행은 다섯을
//! `user_modified` 로 도장 찍고 **`uninstall` 은 통째로 거부한다** — 블록 제거가 바이트
//! 완전 일치인데 실물이 CRLF 이기 때문이다. **걷어낼 방법이 없어진다.**
//!
//! # 무엇을 안 골랐는가
//!
//! | 후보 | 왜 안 골랐나 |
//! |---|---|
//! | 사용자 프로젝트에 `.gitattributes` 를 놓는다 | **우리가 소유하는 파일이 하나 더 는다.** 병합 표면이 커지고, 그 파일은 남의 규칙이 이미 사는 자리다 |
//! | 우리 파일을 언제나 CRLF 로 쓴다 | 유닉스 워킹트리에서 반대 방향으로 같은 문제가 난다 |
//! | 되쓸 때 언제나 LF 로 통일한다 | 사용자의 `git status` 에 우리 파일이 **매번** 뜬다 |
//! | **정규화해서 대조 + 되쓸 때 보존** ← 고른 것 | 판정은 내용으로 하고 바이트는 있던 대로 둔다 |
//!
//! # 두 함수가 짝이다
//!
//! [`정규화`] 는 *"같다고 볼 것인가"* 를 정하고, [`맞춘다`] 는 *"어떤 바이트로 쓸
//! 것인가"* 를 정한다. **둘 중 하나만 있으면 안 된다** — 정규화만 있으면 우리가 넣은
//! 줄만 LF 로 튀고, 보존만 있으면 대조가 여전히 깨진다.
//!
//! ⚠ **경계**: 여기서 다루는 것은 `\r\n` 하나다. 고전 Mac 의 홑 `\r` 은 **안 건드린다** —
//! git 의 `core.autocrlf` 도 그것을 안 만들고, 손대면 사용자 바이트를 우리가 해석하는
//! 일이 된다.

/// `\r\n` → `\n`. **홑 `\r` 은 안 건드린다.**
#[must_use]
pub fn 정규화(bytes: &[u8]) -> Vec<u8> {
    사상(bytes).0
}

/// 정규화한 바이트열과 **사상** — `사상[i]` 는 정규화 자리 `i` 가 원본의 어디였나.
///
/// 길이는 정규화 길이 **+ 1** 이다. 마지막 칸이 원본의 끝을 가리키므로, 정규화 공간의
/// 구간 `[a, b)` 를 원본의 `[사상[a], 사상[b])` 로 그대로 옮길 수 있다.
///
/// ★ **`\r\n` 은 `\r` 자리로 사상한다.** `\n` 자리로 사상하면 우리가 넣은 블록 앞의
/// 개행을 뺄 때 **`\r` 하나가 남는다.**
#[must_use]
pub fn 사상(bytes: &[u8]) -> (Vec<u8>, Vec<usize>) {
    let mut out = Vec::with_capacity(bytes.len());
    let mut map = Vec::with_capacity(bytes.len() + 1);
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'\r' && bytes.get(i + 1) == Some(&b'\n') {
            out.push(b'\n');
            map.push(i);
            i += 2;
            continue;
        }
        out.push(bytes[i]);
        map.push(i);
        i += 1;
    }
    map.push(bytes.len());
    (out, map)
}

/// 이 내용이 CRLF 를 쓰나 — **첫 줄바꿈이 정한다.**
///
/// 다수결이 아니라 첫 줄이다. 섞여 있는 파일에서 우리가 고를 것은 하나뿐이고, 첫 줄은
/// 어느 편집기가 열어도 같은 답을 낸다.
#[must_use]
pub fn crlf_인가(bytes: &[u8]) -> bool {
    match bytes.iter().position(|b| *b == b'\n') {
        Some(0) | None => false,
        Some(at) => bytes[at - 1] == b'\r',
    }
}

/// LF 로 쓰인 본문을 그 파일의 줄바꿈에 맞춘다.
#[must_use]
pub fn 맞춘다(본문: &[u8], crlf: bool) -> Vec<u8> {
    if !crlf {
        return 본문.to_vec();
    }
    let mut out = Vec::with_capacity(본문.len() + 16);
    for (i, b) in 본문.iter().enumerate() {
        if *b == b'\n' && (i == 0 || 본문[i - 1] != b'\r') {
            out.push(b'\r');
        }
        out.push(*b);
    }
    out
}

/// 그 파일의 줄바꿈. 파일이 없으면 LF 다 — **새로 만드는 것은 우리 것이다.**
#[must_use]
pub fn 그_파일의_줄바꿈(기존: Option<&[u8]>) -> bool {
    기존.is_some_and(crlf_인가)
}

#[cfg(test)]
mod tests {
    use super::{crlf_인가, 그_파일의_줄바꿈, 맞춘다, 사상, 정규화};

    #[test]
    fn crlf_만_lf_로_맞춘다() {
        assert_eq!(정규화(b"a\r\nb\r\n"), b"a\nb\n");
        assert_eq!(정규화(b"a\nb\n"), b"a\nb\n");
        // 홑 `\r` 은 안 건드린다.
        assert_eq!(정규화(b"a\rb"), b"a\rb");
        // `\r\r\n` 은 앞의 `\r` 이 남고 뒤가 줄바꿈이다.
        assert_eq!(정규화(b"a\r\r\n"), b"a\r\n");
    }

    /// ★ **정규화 공간의 구간을 원본 구간으로 그대로 옮길 수 있어야 한다.**
    #[test]
    fn 사상이_구간을_그대로_옮긴다() {
        for 원본 in [
            &b"a\r\nBLOCK\r\nz\r\n"[..],
            &b"a\nBLOCK\nz\n"[..],
            &b"a\r\nBLOCK\r\n"[..],
            &b"BLOCK\r\n"[..],
        ] {
            let (정규, map) = 사상(원본);
            assert_eq!(map.len(), 정규.len() + 1, "사상의 길이가 하나 모자라다");
            let at = 정규
                .windows(5)
                .position(|w| w == b"BLOCK")
                .expect("BLOCK 이 정규화 뒤에 없다");
            let mut 뺀것 = 원본.to_vec();
            뺀것.drain(map[at]..map[at + 5]);
            assert_eq!(
                정규화(&뺀것),
                정규화(&{
                    let mut v = 정규.clone();
                    v.drain(at..at + 5);
                    v
                }),
                "구간이 어긋났다: {원본:?}"
            );
        }
    }

    /// ★ **블록 앞에 넣은 개행을 뺄 때 `\r` 이 남으면 안 된다.**
    ///
    /// 사상이 `\n` 자리를 가리키면 여기가 깨진다.
    #[test]
    fn 개행부터_빼도_잔해가_없다() {
        let 원본 = b"a\r\nBLOCK\r\n";
        let (정규, map) = 사상(원본);
        // 정규화 공간에서 `\nBLOCK\n` 을 뺀다.
        let at = 정규.iter().position(|b| *b == b'\n').expect("개행");
        let mut 뺀것 = 원본.to_vec();
        뺀것.drain(map[at]..map[정규.len()]);
        assert_eq!(뺀것, b"a", "잔해가 남았다: {뺀것:?}");
    }

    #[test]
    fn 첫_줄바꿈이_정한다() {
        assert!(crlf_인가(b"a\r\nb\n"));
        assert!(!crlf_인가(b"a\nb\r\n"));
        assert!(!crlf_인가("줄바꿈이 없다".as_bytes()));
        assert!(!crlf_인가(b""));
        assert!(!crlf_인가(b"\n"));
        assert!(그_파일의_줄바꿈(Some(b"a\r\n")));
        // 없던 파일은 우리 것이다 — LF.
        assert!(!그_파일의_줄바꿈(None));
    }

    #[test]
    fn 맞추면_왕복한다() {
        let 본문 = b"a\nb\n";
        assert_eq!(맞춘다(본문, true), b"a\r\nb\r\n");
        assert_eq!(맞춘다(본문, false), 본문);
        assert_eq!(정규화(&맞춘다(본문, true)), 본문);
        // 이미 CRLF 인 것을 두 번 맞춰도 안 는다.
        assert_eq!(맞춘다(b"a\r\n", true), b"a\r\n");
    }
}
