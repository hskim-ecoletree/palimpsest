//! 비코드 개체의 식별 — **지금 정한다** (옛 F09 §4.3).
//!
//! [`crate::Coord`] 는 코드 전용이다(`repo_id, commit, extractor, symbol_id`). 그런데
//! **결정·계획·잔여·라벨은 자기 식별자가 필요하다.**
//!
//! > **지금 정하지 않으면 나중엔 마이그레이션이다.** 의도층 개체가 쌓인 뒤에 식별
//! > 체계를 바꾸면 전부 다시 결박해야 한다. **비용이 지금은 0 이다.**
//!
//! # [`EntityKind`] 가 닫힌 열거가 아닌 이유 — **두 규칙이 충돌한다**
//!
//! 옛 F09 §4.3 바로 아래 문장이 *"노드·엣지 타입을 닫힌 열거로 하드코딩하지 않는다"* 인데
//! `Decision | … | Concept` 은 **그 자체가 닫힌 열거**다. 게다가 `Concept`(F18 · P3)을
//! 미리 나열하는 것은 *"목록은 데이터"* 라는 결정을 **코드로 부정하는 것**이다.
//!
//! 그래서 **등록형**이다. 코어가 부팅 시 다섯을 등록하고([`EntityRegistry::core`]),
//! 팩이 더한다(F18) — **코어 개정 없이.**
//!
//! # 등록형이어도 결박·낡음 계산은 영향받지 않는다
//!
//! 그 계산은 `target: SymbolId` 와 반경만 쓰고 **개체의 종류를 모른다.** 종류를 아는
//! 것은 **표현**뿐이고, 거기서는 미등록 종류를 *"알 수 없는 종류의 결박 N건"* 으로
//! **보이게** 처리한다 — [`EntityRegistry::describe`]. **조용히 버리지 않는다.**
//!
//! # `Ulid` 를 크레이트로 안 들인다 — 그리고 그 판단을 여기 적는다
//!
//! 옛 F09 §4.3 이 `id: Ulid` 라고 이름으로 적었다. `ulid` 크레이트(3.0)는 **열다섯 개를
//! 끌고 온다** — `rand` · `chacha20` · `futures-util` · `js-sys` · **`wasm-bindgen`**.
//! [스택 §3.4](../../../docs/plan/00-stack.md) 가 *"P0 에서 외부 크레이트 신규 추가는
//! 근거를 남긴다"* 이고 이 저장소는 그보다 훨씬 싼 것도 기각했다(`criterion` ·
//! `rusqlite` · `blame`). **식별자 하나에 wasm 바인딩을 치를 수 없다.**
//!
//! 그래서 여기서 만든다 — **48비트 밀리초 + 80비트 무작위 · Crockford base32 26자**.
//! 형식은 ULID 그대로이므로 나중에 크레이트로 갈아도 값이 안 움직인다.
//!
//! **이것은 암호 식별자가 아니고 그럴 필요도 없다.** 막으려는 것은 *"두 저장소를
//! JSONL 로 합쳤을 때 서로 다른 개체가 같은 이름을 갖는 것"* 이지 추측 불가능성이 아니다.

use std::collections::BTreeMap;
use std::hash::{BuildHasher, Hash, Hasher};
use std::sync::atomic::{AtomicU64, Ordering};

use serde::{Deserialize, Serialize};

/// 개체 종류의 이름. **닫힌 열거가 아니다** — 목록은 데이터다.
///
/// `cargo xtask check` 가 코어에 `enum EntityKind` 가 생기는 것을 막는다.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct EntityKind(String);

impl EntityKind {
    #[must_use]
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for EntityKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// 이 개체가 어디서 왔나 — **추적용이고 정체성이 아니다.**
///
/// 문서가 이동하면 이 값이 바뀌지만 [`EntityId::id`] 는 안 바뀐다. **그 구별이
/// 이 타입이 존재하는 이유다** — 경로 해시를 정체성으로 쓰면 문서 하나를 옮길 때마다
/// 결박이 전멸한다.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
// **내부 태그를 안 쓴다** — `postcard` 가 못 싣는다([`crate::Radius`] 와 같은 자리).
#[serde(rename_all = "snake_case")]
pub enum EntityOrigin {
    /// 사람이 명령줄에서 직접 넣었다 — **이 빌드의 유일한 입구다**(S3 · F09).
    /// 문서 인입은 F10 이고 그때 [`Self::Document`] 가 처음 하중을 진다.
    Hand,
    /// 어느 문서의 어느 섹션에서 왔나.
    Document { path: String, anchor: String },
}

/// 비코드 개체 하나의 이름.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct EntityId {
    pub kind: EntityKind,
    /// **안정 식별자.** 경로 해시가 **아니다** — 문서가 이동해도 유지되어야 한다.
    pub id: Ulid,
    /// 어디서 왔나. **정체성이 아니다.**
    pub origin: EntityOrigin,
}

impl EntityId {
    /// 새 개체 하나. **부를 때마다 다른 이름이 나온다** — 그것이 요점이다.
    #[must_use]
    pub fn mint(kind: EntityKind, origin: EntityOrigin) -> Self {
        Self { kind, id: Ulid::now(), origin }
    }

    /// 씨앗에서 **결정적으로** 유도한다 — **옛 판을 올릴 때만 쓴다.**
    ///
    /// # 왜 이 문이 필요한가
    ///
    /// [`Self::mint`] 를 부르면 같은 파일을 두 번 읽을 때 **개체가 둘이 된다.**
    /// 의도 저장소의 읽기는 **더하기이지 바꿔치기가 아니므로**(`[f05.4]` ②) 두 번
    /// 읽는 것은 정상 경로이고, 그때 왕복이 항등이 아니게 된다.
    ///
    /// **새 개체는 이 함수로 만들지 않는다.** 유도한 값은 씨앗에 묶여 있으므로,
    /// 씨앗이 같은 서로 다른 개체가 같은 이름을 갖는다.
    #[must_use]
    pub fn derived(kind: EntityKind, origin: EntityOrigin, seed: &[u8]) -> Self {
        Self { kind, id: Ulid::derived(seed), origin }
    }

    /// 화면과 산출에 싣는 형태 — `decision/01J...`.
    #[must_use]
    pub fn to_display(&self) -> String {
        format!("{}/{}", self.kind, self.id)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Ulid — 48비트 밀리초 + 80비트 무작위. **크레이트를 안 들인다**(모듈 머리)
// ─────────────────────────────────────────────────────────────────────────────

/// Crockford base32 — `I`·`L`·`O`·`U` 가 없다(눈으로 옮겨 적을 때 헷갈리는 것들).
const CROCKFORD: &[u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";

/// 같은 밀리초 안에서 갈리게 하는 자리 — **무작위원과 곱해 쓰지 않고 더해 쓴다.**
static COUNTER: AtomicU64 = AtomicU64::new(0);

/// 시간순으로 정렬되는 128비트 이름. **26자 Crockford base32.**
///
/// # 직렬화가 숫자가 아니라 **글자**다
///
/// 이유가 둘이고 하나는 강제다:
///
///   · **강제** — `serde_json` 은 `u128` 을 *"지원하지 않는다"* 로 거절한다. 그리고
///     JSONL 내보내기는 [R-21] 의 **유일한 복구 경로**이므로 이 타입이 거기 못 실리면
///     결박이 못 나간다
///   · **옳다** — 복구 파일은 사람이 읽고 고칠 수 있어야 한다. `01JQ…` 는 읽히고
///     `2071394...` 는 안 읽힌다
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ulid(u128);

impl Serialize for Ulid {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for Ulid {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let raw = String::deserialize(d)?;
        // **조용히 0 으로 접지 않는다** — 서로 다른 문자열이 같은 이름이 된다.
        Self::parse(&raw).ok_or_else(|| {
            serde::de::Error::custom(format!("ULID 가 아니다: `{raw}` (26자 Crockford base32)"))
        })
    }
}

impl Ulid {
    /// 지금 시각으로 하나.
    ///
    /// # 무작위원이 어디서 오나 — **표준 라이브러리뿐이다**
    ///
    /// [`std::collections::hash_map::RandomState`] 는 **OS 가 씨를 준다**. 인스턴스마다
    /// 다른 씨를 갖고, 그것으로 값 하나를 해싱하면 64비트가 나온다. 둘을 뽑아 80비트를
    /// 채운다.
    ///
    /// **암호 난수가 아니다.** 막으려는 것은 *"두 저장소를 합쳤을 때 서로 다른 개체가
    /// 같은 이름을 갖는 것"* 이지 추측 불가능성이 아니다.
    #[must_use]
    pub fn now() -> Self {
        // **48비트로 자른다.** 2^48 ms 는 서기 10889 년이므로 이 자르기가 실물에서
        // 값을 바꾸지 않는다 — 그래도 `try_from` 으로 적는다. `as` 는 조용히 자른다.
        let millis = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_millis());
        let ms = u64::try_from(millis).unwrap_or(u64::MAX) & 0x0000_FFFF_FFFF_FFFF;
        let n = COUNTER.fetch_add(1, Ordering::Relaxed);
        let a = u128::from(임의값(n));
        let b = u128::from(임의값(n.wrapping_add(0x9E37_79B9_7F4A_7C15)));
        let rand80 = ((a << 16) ^ b) & 0x0000_0000_0000_FFFF_FFFF_FFFF_FFFF_FFFF;
        Self((u128::from(ms) << 80) | rand80)
    }

    /// 씨앗에서 **결정적으로**. **옛 판을 올릴 때만 쓴다**([`EntityId::derived`]).
    ///
    /// 시각 자리도 씨앗에서 나오므로 **시간순 정렬이 뜻을 잃는다.** 그것이 이 함수를
    /// 일반 경로에 두지 않는 이유이고, 이름이 그렇게 말한다.
    #[must_use]
    pub fn derived(seed: &[u8]) -> Self {
        let mut h = blake3::Hasher::new();
        h.update(b"pal-entity-v1\0");
        h.update(seed);
        let mut raw = [0u8; 16];
        raw.copy_from_slice(&h.finalize().as_bytes()[..16]);
        Self(u128::from_be_bytes(raw))
    }

    /// 이 이름이 실린 밀리초. **표시용이다** — 앵커가 아니다.
    #[must_use]
    pub const fn millis(self) -> u64 {
        (self.0 >> 80) as u64
    }

    /// 26자에서 읽는다. **길이나 글자가 다르면 `None`** — 조용히 0 으로 접지 않는다.
    #[must_use]
    pub fn parse(raw: &str) -> Option<Self> {
        if raw.len() != 26 {
            return None;
        }
        let mut v: u128 = 0;
        for c in raw.bytes() {
            let i = CROCKFORD.iter().position(|k| *k == c.to_ascii_uppercase())?;
            v = v.checked_mul(32)?.checked_add(i as u128)?;
        }
        Some(Self(v))
    }
}

fn 임의값(salt: u64) -> u64 {
    let state = std::collections::hash_map::RandomState::new();
    let mut h = state.build_hasher();
    salt.hash(&mut h);
    // 힙 주소도 섞는다 — ASLR 이 회차마다 다른 값을 준다.
    let probe = Box::new(salt);
    (std::ptr::from_ref::<u64>(&*probe) as usize as u64).hash(&mut h);
    h.finish()
}

impl std::fmt::Display for Ulid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = [b'0'; 26];
        let mut v = self.0;
        for slot in buf.iter_mut().rev() {
            *slot = CROCKFORD[(v % 32) as usize];
            v /= 32;
        }
        f.write_str(std::str::from_utf8(&buf).unwrap_or("?"))
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 등록부 — **목록은 데이터다**
// ─────────────────────────────────────────────────────────────────────────────

/// 무엇이 결박 가능한 개체인가. **코어가 다섯을 등록하고 팩이 더한다**(F18).
#[derive(Debug, Clone, Default)]
pub struct EntityRegistry {
    kinds: BTreeMap<EntityKind, &'static str>,
}

impl EntityRegistry {
    /// 코어가 부팅 시 등록하는 다섯 — 옛 F09 §4.3 그대로.
    #[must_use]
    pub fn core() -> Self {
        let mut kinds = BTreeMap::new();
        kinds.insert(EntityKind::new("decision"), "사람이 내린 결정");
        kinds.insert(EntityKind::new("plan"), "계획 항목");
        kinds.insert(EntityKind::new("label"), "붙인 이름");
        kinds.insert(EntityKind::new("residual"), "검사하지 못한 자리");
        kinds.insert(EntityKind::new("scope-reduction"), "범위를 줄인 기록");
        Self { kinds }
    }

    /// 팩이 하나를 더한다. **이미 있으면 안 덮어쓴다** — 덮어쓰면 코어의 뜻이 조용히 바뀐다.
    pub fn register(&mut self, kind: EntityKind, what: &'static str) -> bool {
        if self.kinds.contains_key(&kind) {
            return false;
        }
        self.kinds.insert(kind, what);
        true
    }

    #[must_use]
    pub fn knows(&self, kind: &EntityKind) -> bool {
        self.kinds.contains_key(kind)
    }

    #[must_use]
    pub fn names(&self) -> Vec<&EntityKind> {
        self.kinds.keys().collect()
    }

    /// 사람이 읽는 한 줄. **모르는 종류도 답이 있다 — 조용히 버리지 않는다.**
    ///
    /// 옛 F09 §4.3 이 요구한 것이 정확히 이것이다: *"미등록 종류를 「알 수 없는 종류의
    /// 결박 N건」으로 **보이게** 처리한다."*
    #[must_use]
    pub fn describe(&self, kind: &EntityKind) -> String {
        self.kinds.get(kind).map_or_else(
            || format!("알 수 없는 종류 `{kind}` — 이 빌드에 등록되지 않았습니다"),
            |what| (*what).to_owned(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 이름이_회차마다_다르다() {
        // **같으면 두 개체가 한 이름을 갖는다.** 이것이 이 타입의 유일한 요구다.
        let n = 1_000;
        let 집합: std::collections::BTreeSet<_> = (0..n).map(|_| Ulid::now()).collect();
        assert_eq!(집합.len(), n, "{n} 개를 뽑았는데 {} 개만 갈렸다", 집합.len());
    }

    #[test]
    fn 스물여섯자이고_왕복한다() {
        let u = Ulid::now();
        let s = u.to_string();
        assert_eq!(s.len(), 26, "{s}");
        assert!(s.bytes().all(|c| CROCKFORD.contains(&c)), "{s}");
        assert_eq!(Ulid::parse(&s), Some(u));
    }

    #[test]
    fn 모르는_글자는_조용히_0_이_되지_않는다() {
        // **★ 반대 방향.** 접어서 읽으면 서로 다른 문자열이 같은 이름이 된다.
        assert_eq!(Ulid::parse(""), None);
        assert_eq!(Ulid::parse("짧다"), None);
        assert_eq!(Ulid::parse(&"I".repeat(26)), None, "Crockford 에 없는 글자다");
        assert_eq!(Ulid::parse(&"U".repeat(26)), None);
        assert_eq!(Ulid::parse(&"0".repeat(27)), None);
    }

    #[test]
    fn 시각이_실리고_시간순으로_정렬된다() {
        let a = Ulid::now();
        std::thread::sleep(std::time::Duration::from_millis(2));
        let b = Ulid::now();
        assert!(b.millis() > a.millis(), "{} !> {}", b.millis(), a.millis());
        assert!(b > a, "시간순으로 정렬되지 않는다");
        assert!(b.to_string() > a.to_string(), "문자열도 시간순이어야 한다");
    }

    #[test]
    fn 코어가_다섯을_등록하고_팩이_더한다() {
        let mut r = EntityRegistry::core();
        assert_eq!(r.names().len(), 5);
        assert!(r.knows(&EntityKind::new("decision")));
        assert!(r.register(EntityKind::new("concept"), "F18 의 개념"));
        // **이미 있으면 안 덮어쓴다** — 덮어쓰면 코어의 뜻이 조용히 바뀐다.
        assert!(!r.register(EntityKind::new("decision"), "다른 뜻"));
        assert_eq!(r.describe(&EntityKind::new("decision")), "사람이 내린 결정");
    }

    #[test]
    fn 미등록_종류를_조용히_버리지_않는다() {
        // **★ 옛 F09 §4.3 이 요구한 자리다.** 모르는 종류도 답이 있다.
        let r = EntityRegistry::core();
        let 모르는 = EntityKind::new("아무거나");
        assert!(!r.knows(&모르는));
        let 설명 = r.describe(&모르는);
        assert!(설명.contains("알 수 없는 종류"), "{설명}");
        assert!(설명.contains("아무거나"), "이름이 안 실렸다: {설명}");
    }

    #[test]
    fn 개체의_이름은_종류와_id_를_함께_낸다() {
        let e = EntityId::mint(EntityKind::new("decision"), EntityOrigin::Hand);
        let s = e.to_display();
        assert!(s.starts_with("decision/"), "{s}");
        assert_eq!(s.len(), "decision/".len() + 26);
    }
}
