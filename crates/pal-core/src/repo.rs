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

    /// **파일시스템 경로에서 만드는 유일한 문.**
    ///
    /// 위 불변식(*"Windows 에서 만든 대장과 macOS 에서 만든 대장이 달라지면 대조가
    /// 성립하지 않는다"*)은 [`RepoPath::new`] 가 **혼자 지지 못한다** — 그것은 문자열을
    /// 그대로 받고, `std::path::Path` 의 구분자는 플랫폼마다 다르다. 그래서
    /// 파일시스템 경로가 좌표가 되는 자리는 전부 이 문을 지난다.
    ///
    /// ⚠ **`\` 를 무조건 `/` 로 바꾼다.** 유닉스에서는 파일 이름에 `\` 가 들어갈 수
    /// 있으므로 그런 이름은 여기서 갈린다. 그 값을 이기게 한 이유는 이 타입이 지는
    /// 것이 **파일 이름이 아니라 기계 사이를 오가는 좌표**이기 때문이다 — 같은 판단을
    /// `pal-cli` 의 매니페스트 훑기가 이미 하고 있고, 여기가 그 짝이다.
    #[must_use]
    pub fn from_fs(path: &std::path::Path) -> Self {
        Self(path.to_string_lossy().replace('\\', "/"))
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
/// # 집합이다 — 쌍이 아니다 (F22 의 정본화 · 2026-08-12)
///
/// [DESIGN §1.1](../../../docs/DESIGN.md) 은 `Snapshot` 을 `{(repo_id, TreeRef)}` 로
/// 적었다: **"멀티레포의 '지금'은 하나가 아니라 집합"**. S1 은 그것을 `{repo, tree}`
/// 쌍으로 만들었고 저장소가 하나뿐이라 그 차이가 드러나지 않았다.
///
/// **그런데 코드가 이미 그 차이를 우회하고 있었다** — [`crate::Ledger`] 가
/// `repos_declared` 를 따로 들고 있는 것이 그 흔적이다. 쌍으로는 두 저장소의 "지금"을
/// 적을 수 없으니 개수만 세어 머리에 적은 것이다. F22 가 스키마를 세우면서 이 자리를
/// 되돌렸다: **어긋난 것은 스키마가 아니라 코드였다.**
///
/// `repos_declared` 는 그대로 남는다. **선언된 것과 본 것은 다르고, 그 차이가 곧
/// §4.3 이 말한 뿌리의 공백**이다 — 여기 없는 저장소를 지나는 경로는 조용히 사라진다.
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
#[serde(transparent)]
pub struct Snapshot(Vec<(RepoId, TreeRef)>);

impl Snapshot {
    /// 저장소 하나의 지금. **가장 흔한 경우이고, 그래도 집합이다.**
    #[must_use]
    pub fn single(repo: RepoId, tree: TreeRef) -> Self {
        Self(vec![(repo, tree)])
    }

    /// 여럿의 지금. **비어 있으면 만들 수 없다** — 아무것도 보지 않은 산출에는
    /// 좌표가 없고, 좌표 없는 산출은 나가면 안 된다.
    ///
    /// 저장소 이름으로 정렬한다. 같은 저장소가 두 번 오면 **뒤의 것이 이긴다** —
    /// 한 저장소의 "지금"이 둘일 수는 없기 때문이다.
    #[must_use]
    pub fn of(pairs: impl IntoIterator<Item = (RepoId, TreeRef)>) -> Option<Self> {
        let mut v: Vec<(RepoId, TreeRef)> = pairs.into_iter().collect();
        if v.is_empty() {
            return None;
        }
        v.sort_by(|a, b| a.0.cmp(&b.0));
        v.dedup_by(|a, b| a.0 == b.0);
        Some(Self(v))
    }

    /// 이 스냅샷이 덮는 (저장소, 트리) 전부. **정렬돼 있다.**
    pub fn entries(&self) -> impl Iterator<Item = &(RepoId, TreeRef)> {
        self.0.iter()
    }

    /// 그 저장소의 트리. **없으면 그 저장소를 보지 않은 것이다** — 조회 결과이지
    /// 도메인 값이 아니므로 `Option` 이 여기 있는 것이 stack §5.4 에 맞는다.
    #[must_use]
    pub fn tree_of(&self, repo: &RepoId) -> Option<&TreeRef> {
        self.0.iter().find(|(r, _)| r == repo).map(|(_, t)| t)
    }

    /// 덮는 저장소 수. 대장의 `repos_declared` 와 다르면 그 차이가 공백이다.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// **언제나 거짓이다.** [`Snapshot::of`] 가 빈 것을 만들지 않기 때문이고,
    /// 그 사실을 타입 밖에서도 확인할 수 있게 남긴다.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl fmt::Display for Snapshot {
    /// `repo@tree`. 여럿이면 첫 것과 **나머지 개수**를 함께 적는다 — 뒤를 감추지 않는다.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut it = self.0.iter();
        let Some((r, t)) = it.next() else { return f.write_str("(빈 스냅샷)") };
        write!(f, "{r}@{t}")?;
        let rest = self.0.len() - 1;
        if rest > 0 {
            write!(f, " 외 {rest}개 저장소")?;
        }
        Ok(())
    }
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

    /// ★ **파일시스템 경로가 좌표가 될 때 구분자가 하나로 모인다.**
    ///
    /// 이 타입의 머리말이 적은 불변식 — *"Windows 에서 만든 대장과 macOS 에서 만든
    /// 대장이 달라지면 대조가 성립하지 않는다"* — 은 **문자열을 그대로 받는
    /// [`RepoPath::new`] 로는 안 선다.** Windows 의 `Path` 는 `\` 를 내고, 그 값이
    /// 그대로 좌표가 되면 같은 파일이 두 이름을 갖는다.
    ///
    /// 리터럴을 쓰는 이유: `Path::new(r"a\b")` 는 유닉스에서 **성분 하나**라 플랫폼
    /// 분기 없이도 「구분자가 아닌 `\`」를 그대로 재현한다.
    #[test]
    fn 파일시스템_경로는_구분자를_한쪽으로_모은다() {
        use std::path::Path;
        assert_eq!(RepoPath::from_fs(Path::new(r"docs\plan\00-stack.md")).as_str(), "docs/plan/00-stack.md");
        // 이미 `/` 인 것은 그대로다 — 유닉스에서 만든 값이 안 움직인다.
        assert_eq!(RepoPath::from_fs(Path::new("docs/plan/00-stack.md")).as_str(), "docs/plan/00-stack.md");
        // 그리고 두 플랫폼의 산출이 **같은 값**이 된다. 이것이 재려는 문장이다.
        assert_eq!(
            RepoPath::from_fs(Path::new(r"docs\plan\00-stack.md")),
            RepoPath::from_fs(Path::new("docs/plan/00-stack.md"))
        );
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
    fn 스냅샷은_집합이고_비어_있을_수_없다() {
        // DESIGN §1.1 — 멀티레포의 "지금"은 하나가 아니라 집합이다.
        let c = ObjectName::from_bytes([0xab; 20]);
        let s = Snapshot::single(RepoId::new("a"), TreeRef::Committed(c));
        assert_eq!(s.len(), 1);
        assert!(!s.is_empty());
        assert_eq!(s.tree_of(&RepoId::new("a")), Some(&TreeRef::Committed(c)));
        assert_eq!(s.tree_of(&RepoId::new("b")), None);

        assert!(Snapshot::of(Vec::new()).is_none());

        let 둘 = Snapshot::of([
            (RepoId::new("z"), TreeRef::Committed(c)),
            (RepoId::new("a"), TreeRef::Committed(c)),
        ])
        .unwrap();
        assert_eq!(둘.len(), 2);
        // 정렬돼 있다 — 같은 스냅샷이 같은 순서를 내야 산출을 비교할 수 있다.
        assert_eq!(둘.entries().next().unwrap().0, RepoId::new("a"));
    }

    #[test]
    fn 한_저장소의_지금은_둘일_수_없다() {
        let c1 = ObjectName::from_bytes([1; 20]);
        let c2 = ObjectName::from_bytes([2; 20]);
        let s = Snapshot::of([
            (RepoId::new("a"), TreeRef::Committed(c1)),
            (RepoId::new("a"), TreeRef::Committed(c2)),
        ])
        .unwrap();
        assert_eq!(s.len(), 1);
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

/// 저장소 하나가 **다른 이름으로도 불린 적이 있다**는 선언.
///
/// # 왜 필요한가 ([R-08](../../../docs/plan/00-risks.md#r-08) · F03 §4.2)
///
/// `repo_id` 가 `symbol_id` 의 해시 성분이라, 저장소를 나누거나 합치면 **전 심볼의
/// 정체성이 한 번에 끊긴다.** 별칭이 그 재배치를 흡수한다.
///
/// # 이것은 **사람이 선언하는 것**이다 — 그래서 의도 저장소가 소유한다 ([R-21])
///
/// *"이 저장소가 저 저장소였다"* 는 코드에서 유도되지 않는다. 파생층에 두면
/// *"지우고 재구축"* 이 그 선언을 지우고, **재구축 등가성 검사는 그 상태에서도
/// 통과하므로 검사가 유실을 정상으로 승인한다.**
///
/// # 흡수되지 않은 재배치는 **관측 가능한 사건**이다
///
/// 별칭이 없으면 전 심볼이 `orphaned` 가 된다. F03 §4.2 가 그것을 *"조용한 정체성
/// 유실보다 낫다"* 로 판단했고, 이 타입은 그 판단을 뒤집지 않는다 — **자동으로
/// 흡수하지 않고 선언된 것만 흡수한다.**
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct RepoAlias {
    /// 옛 이름.
    pub was: RepoId,
    /// 지금 이름.
    pub now: RepoId,
    /// 누가·왜 — **사람이 적는다.** 빈 문자열이 아니어야 한다는 강제는 F09 다.
    pub note: String,
}

impl RepoAlias {
    #[must_use]
    pub fn new(was: RepoId, now: RepoId, note: impl Into<String>) -> Self {
        Self { was, now, note: note.into() }
    }
}
