//! SHA-256 — **매니페스트가 파일마다 지는 값**(`[f24]` ③).
//!
//! # 왜 크레이트를 안 들이는가
//!
//! stack §3.4 는 *"P0 에서 외부 크레이트 신규 추가는 커밋 메시지에 근거를 남긴다"* 이고,
//! `deny.toml` 은 *"예외를 늘리기 전에 의존을 줄이는 것이 순서"* 라고 적었다. 이 자리에
//! 필요한 것은 **압축 함수 하나**이고, 그것을 위해 `sha2` → `digest` → `block-buffer` →
//! `crypto-common` → `generic-array` → `typenum` 다섯 개를 들이면 라이선스·권고·중복
//! 검사의 표면이 그만큼 는다.
//!
//! **그리고 이 함수는 시험 벡터로 전수 검증된다** — FIPS 180-4 의 공개 벡터가 있고,
//! 그것이 우리가 직접 쓴 것을 크레이트와 같은 자격으로 만든다. `blake3` 를 안 쓰는
//! 이유는 하나다: 게이트가 **`sha256`** 이라고 적었고 매니페스트는 우리 밖에서도
//! 읽히는 파일이다.

/// 라운드 상수 — FIPS 180-4 §4.2.2. 처음 64개 소수의 세제곱근 소수부 32비트.
const K: [u32; 64] = [
    0x428a_2f98, 0x7137_4491, 0xb5c0_fbcf, 0xe9b5_dba5, 0x3956_c25b, 0x59f1_11f1, 0x923f_82a4,
    0xab1c_5ed5, 0xd807_aa98, 0x1283_5b01, 0x2431_85be, 0x550c_7dc3, 0x72be_5d74, 0x80de_b1fe,
    0x9bdc_06a7, 0xc19b_f174, 0xe49b_69c1, 0xefbe_4786, 0x0fc1_9dc6, 0x240c_a1cc, 0x2de9_2c6f,
    0x4a74_84aa, 0x5cb0_a9dc, 0x76f9_88da, 0x983e_5152, 0xa831_c66d, 0xb003_27c8, 0xbf59_7fc7,
    0xc6e0_0bf3, 0xd5a7_9147, 0x06ca_6351, 0x1429_2967, 0x27b7_0a85, 0x2e1b_2138, 0x4d2c_6dfc,
    0x5338_0d13, 0x650a_7354, 0x766a_0abb, 0x81c2_c92e, 0x9272_2c85, 0xa2bf_e8a1, 0xa81a_664b,
    0xc24b_8b70, 0xc76c_51a3, 0xd192_e819, 0xd699_0624, 0xf40e_3585, 0x106a_a070, 0x19a4_c116,
    0x1e37_6c08, 0x2748_774c, 0x34b0_bcb5, 0x391c_0cb3, 0x4ed8_aa4a, 0x5b9c_ca4f, 0x682e_6ff3,
    0x748f_82ee, 0x78a5_636f, 0x84c8_7814, 0x8cc7_0208, 0x90be_fffa, 0xa450_6ceb, 0xbef9_a3f7,
    0xc671_78f2,
];

/// 초기 해시 값 — FIPS 180-4 §5.3.3. 처음 8개 소수의 제곱근 소수부 32비트.
const H0: [u32; 8] = [
    0x6a09_e667, 0xbb67_ae85, 0x3c6e_f372, 0xa54f_f53a, 0x510e_527f, 0x9b05_688c, 0x1f83_d9ab,
    0x5be0_cd19,
];

/// 바이트열의 SHA-256 을 소문자 16진 64자로 낸다.
#[must_use]
pub fn hex(bytes: &[u8]) -> String {
    let digest = digest(bytes);
    let mut out = String::with_capacity(64);
    for b in digest {
        // `write!` 를 안 쓴다 — 실패할 수 없는 자리에 `Result` 를 만들지 않는다.
        out.push(char::from_digit(u32::from(b >> 4), 16).unwrap_or('?'));
        out.push(char::from_digit(u32::from(b & 0x0f), 16).unwrap_or('?'));
    }
    out
}

fn digest(bytes: &[u8]) -> [u8; 32] {
    let mut h = H0;

    // 패딩 — `0x80` 한 바이트, 0 채움, 마지막 8바이트에 비트 길이(빅엔디언).
    let mut padded = Vec::with_capacity(bytes.len() + 72);
    padded.extend_from_slice(bytes);
    padded.push(0x80);
    while padded.len() % 64 != 56 {
        padded.push(0);
    }
    let bits = (bytes.len() as u64).wrapping_mul(8);
    padded.extend_from_slice(&bits.to_be_bytes());

    for block in padded.chunks_exact(64) {
        compress(&mut h, block);
    }

    let mut out = [0u8; 32];
    for (slot, word) in out.chunks_exact_mut(4).zip(h) {
        slot.copy_from_slice(&word.to_be_bytes());
    }
    out
}

// **FIPS 180-4 §6.2.2 의 이름을 그대로 쓴다.** `a`..`h`·`w`·`t1`·`t2` 를 길게 바꾸면
// 명세와 한 줄씩 대 볼 수 없게 되고, 손으로 쓴 압축 함수에서 그것이 유일한 감사 수단이다.
#[allow(clippy::many_single_char_names)]
fn compress(h: &mut [u32; 8], block: &[u8]) {
    let mut w = [0u32; 64];
    for (slot, four) in w[..16].iter_mut().zip(block.chunks_exact(4)) {
        *slot = u32::from_be_bytes([four[0], four[1], four[2], four[3]]);
    }
    for i in 16..64 {
        let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
        let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
        w[i] = w[i - 16]
            .wrapping_add(s0)
            .wrapping_add(w[i - 7])
            .wrapping_add(s1);
    }

    let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut hh] = *h;
    for i in 0..64 {
        let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
        let ch = (e & f) ^ ((!e) & g);
        let t1 = hh
            .wrapping_add(s1)
            .wrapping_add(ch)
            .wrapping_add(K[i])
            .wrapping_add(w[i]);
        let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
        let maj = (a & b) ^ (a & c) ^ (b & c);
        let t2 = s0.wrapping_add(maj);

        hh = g;
        g = f;
        f = e;
        e = d.wrapping_add(t1);
        d = c;
        c = b;
        b = a;
        a = t1.wrapping_add(t2);
    }

    for (slot, word) in h.iter_mut().zip([a, b, c, d, e, f, g, hh]) {
        *slot = slot.wrapping_add(word);
    }
}

#[cfg(test)]
mod tests {
    use super::hex;

    /// FIPS 180-4 의 공개 시험 벡터. **우리가 고른 입력이 아니다** — 그것이 이
    /// 구현을 크레이트와 같은 자격으로 만든다.
    #[test]
    fn 공개_시험_벡터() {
        assert_eq!(hex(b""), "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
        assert_eq!(hex(b"abc"), "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
        assert_eq!(
            hex(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq"),
            "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1"
        );
    }

    /// **블록 경계 셋.** 55·56·64 바이트에서 패딩의 갈래가 갈린다 — 여기가 손으로 쓴
    /// 구현이 조용히 틀리는 자리다.
    #[test]
    fn 패딩_경계() {
        assert_eq!(
            hex(&[b'a'; 55]),
            "9f4390f8d30c2dd92ec9f095b65e2b9ae9b0a925a5258e241c9f1e910f734318"
        );
        assert_eq!(
            hex(&[b'a'; 56]),
            "b35439a4ac6f0948b6d6f9e3c6af0f5f590ce20f1bde7090ef7970686ec6738a"
        );
        assert_eq!(
            hex(&[b'a'; 64]),
            "ffe054fe7ae0cb6dc65c3af9b61d5209f439851db43d0ba5997337df154668eb"
        );
        assert_eq!(
            hex(&[b'a'; 1000]),
            "41edece42d63e8d9bf515a9ba6932e1c20cbc9f5a5d134645adb5db1b9737ea3"
        );
    }

    /// **NUL 바이트를 먹는다.** 텍스트로 다루면 여기서 잘린다.
    #[test]
    fn 널_바이트가_있어도_전부_센다() {
        assert_ne!(hex(b"a\0b"), hex(b"a"));
        assert_ne!(hex(b"a\0b"), hex(b"ab"));
        assert_eq!(hex(b"a\0b").len(), 64);
    }
}
