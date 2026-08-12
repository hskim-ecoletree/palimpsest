//! 저장소 좌표 — `RepoId` · `TreeRef` · `Snapshot`.
//!
//! **`Coord` 는 아직 없다.** 좌표의 네 성분 중 `symbol` 을 채우려면 F03 의 정규화가
//! 필요하고 S1 은 파일 단위에서 닫힌다. 여기 있는 것은 그 앞의 셋 — *어느 저장소의*,
//! *어느 트리에서*, *무엇을* 이다.

use std::fmt;

use serde::{Deserialize, Serialize};

/// 저장소의 안정 식별자. **경로도 원격 URL 도 아니다** — 둘 다 움직인다([R-08]).
///
/// 매니페스트가 선언하며 출처는 `asserted` 다. "어떤 저장소들이 한 프로젝트인가"는
/// 코드에 없기 때문이다(DESIGN §4.3).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct RepoId(String);

impl RepoId {
    #[must_use]
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for RepoId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// 저장소 루트 기준 상대 경로. **구분자는 항상 `/` 다** — git 이 그렇게 센다.
///
/// 플랫폼 경로(`std::path::Path`)로 두지 않는 이유: 대장은 기계 사이를 오가는 산출이고
/// Windows 에서 만든 대장과 macOS 에서 만든 대장이 달라지면 대조가 성립하지 않는다.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct RepoPath(String);

impl RepoPath {
    #[must_use]
    pub fn new(path: impl Into<String>) -> Self {
        Self(path.into())
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// 마지막 `.` 뒤의 것. 없으면 빈 문자열이다.
    ///
    /// **점으로 시작하는 이름은 확장자가 없다** — `.gitignore` 의 확장자는 `gitignore`
    /// 가 아니다.
    #[must_use]
    pub fn extension(&self) -> &str {
        let name = self.0.rsplit('/').next().unwrap_or("");
        match name.rfind('.') {
            Some(0) | None => "",
            Some(i) => &name[i + 1..],
        }
    }

    /// 경로 마지막 성분.
    #[must_use]
    pub fn file_name(&self) -> &str {
        self.0.rsplit('/').next().unwrap_or("")
    }
}

impl fmt::Display for RepoPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// git 객체 이름 20바이트(SHA-1).
///
/// **git 의 것을 그대로 받는다.** 우리가 계산하는 해시(`blake3`)와 타입이 다른 것이
/// 중요하다 — 하나는 git 이 정한 정체성이고 다른 하나는 우리 캐시의 키다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ObjectName([u8; 20]);

impl ObjectName {
    #[must_use]
    pub const fn from_bytes(raw: [u8; 20]) -> Self {
        Self(raw)
    }

    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 20] {
        &self.0
    }

    /// 40자 소문자 16진.
    #[must_use]
    pub fn to_hex(self) -> String {
        let mut s = String::with_capacity(40);
        for b in self.0 {
            use fmt::Write as _;
            let _ = write!(s, "{b:02x}");
        }
        s
    }
}

impl fmt::Display for ObjectName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_hex())
    }
}

impl Serialize for ObjectName {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.to_hex())
    }
}

impl<'de> Deserialize<'de> for ObjectName {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let hex = String::deserialize(d)?;
        let raw = from_hex::<20>(&hex).ok_or_else(|| {
            serde::de::Error::custom(format!("40자 16진이 아니다: {hex}"))
        })?;
        Ok(Self(raw))
    }
}

/// 우리가 계산하는 32바이트 요약(blake3). 값은 계산하는 크레이트가 채운다.
///
/// **`pal-core` 는 이것을 계산하지 않는다** — 해시 크레이트에 의존하지 않기 위해서다.
/// 타입만 여기 있고 계산은 `pal-git`·`pal-store` 의 일이다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Digest([u8; 32]);

impl Digest {
    #[must_use]
    pub const fn from_bytes(raw: [u8; 32]) -> Self {
        Self(raw)
    }

    #[must_use]
    pub fn to_hex(self) -> String {
        let mut s = String::with_capacity(64);
        for b in self.0 {
            use fmt::Write as _;
            let _ = write!(s, "{b:02x}");
        }
        s
    }
}

impl fmt::Display for Digest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_hex())
    }
}

impl Serialize for Digest {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.to_hex())
    }
}

impl<'de> Deserialize<'de> for Digest {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let hex = String::deserialize(d)?;
        let raw = from_hex::<32>(&hex).ok_or_else(|| {
            serde::de::Error::custom(format!("64자 16진이 아니다: {hex}"))
        })?;
        Ok(Self(raw))
    }
}

/// 32바이트 16진. `coord` 의 요약 타입들이 쓴다.
pub(crate) fn hex32(hex: &str) -> Option<[u8; 32]> {
    from_hex::<32>(hex)
}

/// 16진 문자열을 고정 길이 바이트로. **길이가 정확히 맞아야 한다.**
///
/// 짧은 것을 0 으로 채우면 서로 다른 이름이 같은 값이 된다 — 그것이 좌표를 뭉갠다.
fn from_hex<const N: usize>(hex: &str) -> Option<[u8; N]> {
    if hex.len() != N * 2 {
        return None;
    }
    let mut out = [0u8; N];
    for (i, b) in out.iter_mut().enumerate() {
        *b = u8::from_str_radix(hex.get(i * 2..i * 2 + 2)?, 16).ok()?;
    }
    Some(out)
}

/// 무엇을 읽었는가 — 커밋이거나 워킹트리다.
///
/// # 이것이 F01 이 내리는 가장 중요한 결정이다 ([R-06])
///
/// 설계는 커밋을 시간축으로 삼았지만 이 제품의 1순위 사용 장면(적시 제시)은
/// **커밋 전 순간**에 일어난다. 워킹트리에 좌표가 없으면 그 장면이 통째로 죽는다.
///
/// **왜 공짜로 성립하는가** — 1층 캐시 키가 `(blob, extractor_version)` 이지 커밋이
/// 아니다. 워킹트리 파일의 blob 이름을 직접 계산하면 파싱 파이프라인은 커밋을
/// **전혀 모른 채** 그대로 돈다. 커밋 축이 필요한 곳은 좌표 표기와 결박뿐이다.
///
/// **S1 은 `Committed` 만 돌린다.** `Worktree` 는 타입으로 서 있고 값은 F01 이 채운다 —
/// 머클 계산과 git 인덱스 캐시가 거기 있다([F01 §3.2](../../../docs/plan/features/F01-repo-ledger.md)).
/// 자리를 미리 비워두는 것과 없는 것은 다르다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TreeRef {
    Committed(ObjectName),
    Worktree { base: ObjectName, tree_digest: Digest },
}

impl TreeRef {
    /// 이 트리가 딛고 선 커밋.
    #[must_use]
    pub const fn base(&self) -> ObjectName {
        match self {
            Self::Committed(c) | Self::Worktree { base: c, .. } => *c,
        }
    }

    /// 워킹트리와 일치하는가. 표면이 이 문장을 그대로 낸다.
    #[must_use]
    pub const fn is_committed(&self) -> bool {
        matches!(self, Self::Committed(_))
    }
}

impl fmt::Display for TreeRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hex = self.base().to_hex();
        match self {
            Self::Committed(_) => write!(f, "{}", &hex[..7]),
            Self::Worktree { .. } => write!(f, "{}+worktree", &hex[..7]),
        }
    }
}

/// 무엇을 언제 보았는가. **모든 산출이 이것을 동반한다.**
///
/// # `Copy` 가 아니고 이름을 소유한다 — 그 이유가 R-01 의 관측이다
///
/// 처음에는 `repo: &'static str` 로 두어 `Snapshot` 을 `Copy` 로 만들려 했다.
/// 수명 파라미터를 피하려는 선택이었고 그것이 곧 [R-01](../../../docs/plan/00-risks.md#r-01)
/// 의 회피 동작이었다. **역직렬화에서 막혔다** — `Deserialize<'de>` 는 `'de` 가
/// `'static` 을 넘어 산다고 요구할 수 없다.
///
/// 소유로 바꾸는 것이 답이었고, 실제로 그것이 맞다. 대장은 **기계 사이를 오가는 산출**
/// 이라 되읽을 수 있어야 하고, `&'static str` 은 되읽을 수 없는 것을 타입으로 선언한
/// 셈이었다. 편의를 위해 고른 수명이 산출물의 성질과 충돌한 자리다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Snapshot {
    pub repo: RepoId,
    pub tree: TreeRef,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 확장자는_마지막_점_뒤다() {
        assert_eq!(RepoPath::new("src/main/kotlin/A.kt").extension(), "kt");
        assert_eq!(RepoPath::new("a/b.test.ts").extension(), "ts");
        assert_eq!(RepoPath::new("Makefile").extension(), "");
    }

    #[test]
    fn 점으로_시작하는_이름은_확장자가_없다() {
        // `.gitignore` 의 확장자가 `gitignore` 가 되면 대장이 그것을 언어로 오인한다.
        assert_eq!(RepoPath::new(".gitignore").extension(), "");
        assert_eq!(RepoPath::new("a/b/.env").extension(), "");
        // 다만 점으로 시작하고 확장자도 있는 것은 확장자를 갖는다.
        assert_eq!(RepoPath::new(".eslintrc.json").extension(), "json");
    }

    #[test]
    fn 디렉터리의_점은_파일_확장자가_아니다() {
        assert_eq!(RepoPath::new("v1.2/README").extension(), "");
    }

    #[test]
    fn 이름은_16진_왕복을_견딘다() {
        let c = ObjectName::from_bytes([0x0a, 0xff, 0x10, 0x00, 0x99, 1, 2, 3, 4, 5,
                                        6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
        let json = serde_json::to_string(&c).unwrap();
        assert_eq!(serde_json::from_str::<ObjectName>(&json).unwrap(), c);
    }

    #[test]
    fn 길이가_다른_16진은_거부된다() {
        // 짧은 것을 0 으로 채우면 서로 다른 이름이 같은 값이 된다.
        assert!(serde_json::from_str::<ObjectName>("\"abcd\"").is_err());
        assert!(from_hex::<20>("").is_none());
    }

    #[test]
    fn 트리참조는_딛고_선_커밋을_안다() {
        let c = ObjectName::from_bytes([0xab; 20]);
        assert_eq!(TreeRef::Committed(c).base(), c);
        assert!(TreeRef::Committed(c).is_committed());
        let w = TreeRef::Worktree { base: c, tree_digest: Digest::from_bytes([0; 32]) };
        assert_eq!(w.base(), c);
        assert!(!w.is_committed());
    }
}
