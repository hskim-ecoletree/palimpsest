//! 서술물의 좌표 해소 — **신호를 강한 것부터** (F10 §3.2 · [R-09]).
//!
//! # 이 모듈에 자연어가 없다. 그것이 이 모듈의 전부다
//!
//! F10 §3.2 의 표는 신호를 다섯 적고 여섯째 줄에 **「본문 자연어의 이름 유사도 —
//! 쓰지 않는다」**를 적었다. 근거는 §5 의 기각과 같다:
//!
//! > 거짓 결박을 만든다. *"주문 취소 로직"* 이 `cancelOrder` 인지 `OrderCanceller` 인지
//! > **기계가 모른다.** 그리고 결박은 정확해야 값이 있다 — **틀린 결박은 없는 결박보다
//! > 나쁘다.**
//!
//! 그래서 여기 있는 신호 다섯은 **전부 조회다**: 이 경로가 대장에 있는가 · 이 이름이
//! 인덱스에 유일한가 · 이 문서와 같은 커밋에서 무엇이 바뀌었나 · 이 디렉터리 아래에
//! 무엇이 있나. **판단이 아니라 사실이고, 그것이 거짓 결박을 구조적으로 막는 유일한
//! 수단이다** — 표본 검토는 표본 밖을 못 본다.
//!
//! **그 부재를 `cargo xtask check` 가 센다**(검사 14). 문장으로 두면 잊힌다.
//!
//! # 같은 강도의 후보가 여럿이면 **더 약한 신호로 내려가지 않는다**
//!
//! 내려가면 그것이 곧 §4 가 적은 거짓 결박의 원인(*"약한 신호로 확정"*)이다.
//! 동점은 [`Classification::Candidates`] 로 **그대로 나가고 승인을 요구한다** —
//! `pal bind` 가 후보 여럿에서 멈추는 것, `rebind::propose` 가 억지로 안 채우는 것,
//! [`crate::TouchAnswer`] 가 `Ambiguous` 를 답으로 내는 것과 **같은 판단이다.**
//!
//! [R-09]: ../../../docs/plan/00-risks.md#r-09

use serde::{Deserialize, Serialize};

use crate::coord::SymbolId;
use crate::repo::RepoPath;

/// 좌표 해소가 요구하는 조회 — **2층을 모른다.**
///
/// [`crate::Neighborhood`] 와 같은 형태다: 구현은 `pal-store` 의 투영이고 시험은 손으로
/// 만든 표다. **둘이 같은 함수를 지나가는 것**이 이 트레잇의 전부다.
///
/// # `Result` 를 안 진다
///
/// 읽기가 실패하면 후보가 **비고**, 조각은 `미결박` 이 된다 — 즉 *"덜 건다"* 이지
/// *"틀린 것을 건다"* 가 아니다. 그리고 그 축소는 조용하지 않다: 미결박은
/// `narrative.unbound` 가 **사람의 작업 목록으로** 낸다.
pub trait Coordinates {
    /// 이름 하나 → 후보. **여럿인 것이 정상이다.**
    fn by_name(&self, name: &str) -> Vec<NamedCoord>;
    /// 이 경로의 심볼 전부. **경로가 대장에 없으면 빈 목록** — 그것이 *"경로가 대장에
    /// 존재하는가"*(§3.2)의 답이다.
    fn in_path(&self, path: &RepoPath) -> Vec<NamedCoord>;
    /// 이 접두어 아래의 심볼 전부 — 디렉터리 근접성이 쓴다.
    fn under_prefix(&self, prefix: &str) -> Vec<NamedCoord>;
}

/// 좌표 하나와 그것을 좁히는 데 필요한 만큼의 이름.
///
/// # 왜 [`SymbolId`] 만으로는 부족한가
///
/// 인라인 스팬은 `` `OrderService.cancel` `` 처럼 **컨테이너를 함께 적는다.**
/// 좌표만 받으면 그 점 앞을 버려야 하고, 버리면 같은 이름의 서로 다른 메서드가
/// 전부 동점이 된다 — **동점은 확정 안 하므로 그 조각이 통째로 미결박이 된다.**
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NamedCoord {
    pub id: SymbolId,
    pub name: String,
    /// 담고 있는 것들 — 바깥부터.
    pub container: Vec<String>,
    pub path: RepoPath,
}

/// 문서 조각 하나가 든 **신호의 날것** — `pal-extract` 가 낸다.
///
/// **여기 있는 것은 전부 텍스트에서 뽑은 것이고 아직 좌표가 아니다.** 좌표로 바꾸는
/// 것이 [`resolve`] 이고, 그 사이에 **판단이 들어가지 않는다.**
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RawSignals {
    /// 이 조각이 **붙어 있는** 좌표 — 표식 주석만 갖는다(§3.4).
    ///
    /// *"주석은 가장 정확한 좌표를 이미 갖고 있다"* — 붙어 있는 선언이 그것이고,
    /// 이 값은 **계산이 아니라 파싱의 산물**이다.
    pub attached: Vec<SymbolId>,
    /// 프론트매터의 명시적 좌표 — `grounds: ["src/order/cancel.ts#OrderService.cancel"]`.
    pub grounds: Vec<String>,
    /// 펜스 친 코드 안에서 **경로처럼 생긴** 것. 대장이 실재를 판정한다.
    pub fenced_paths: Vec<RepoPath>,
    /// 인라인 코드 스팬의 식별자 — `OrderService.cancel` 처럼 점이 있을 수 있다.
    pub spans: Vec<String>,
    /// 이 문서와 **같은 커밋에서 함께 바뀐** 파일들.
    ///
    /// **`pal-core` 가 git 을 모르므로 부르는 쪽이 지고 온다** —
    /// [`crate::Neighborhood`] 가 2층을 모르는 것과 같은 자리다.
    pub co_changed: Vec<RepoPath>,
}

/// 문서 조각 하나 — 조각화의 산물.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Fragment {
    /// 어느 문서인가. **정체성이 아니라 추적용이다**([`crate::EntityOrigin`]).
    pub path: RepoPath,
    /// 문서 안에서 이 조각의 자리 — 헤딩 앵커이거나 `L<줄>`(주석).
    ///
    /// **문서 안에서 유일해야 한다** — 같으면 두 조각이 한 개체가 된다.
    pub anchor: String,
    /// 조각의 본문. **이것이 결박의 `note` 가 된다.**
    pub body: String,
    pub signals: RawSignals,
}

/// 무엇이 이 좌표를 걸었나 — **순서가 곧 강도다** (§3.2 의 표).
///
/// 산출에 실린다. *"이 조각은 **인라인 스팬 유일**로 걸렸다"* 는 *"이 조각은 이 코드에
/// 관한 것이다"* 와 **다른 문장**이고, 그 차이가 남는 것이 요구다 — F09 가 반경을
/// 산출에 실은 것과 같은 자리.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ResolutionSignal {
    /// 주석이 그 선언에 **붙어 있다**. 파싱의 산물이라 가장 강하다(§3.4).
    Attached,
    /// 프론트매터가 좌표를 **명시**했다. 사람이 적은 것이다(§3.2 의 「확정」).
    Frontmatter,
    /// 펜스 친 코드 안의 경로가 **대장에 있다**.
    FencedPath,
    /// 인라인 스팬의 식별자가 **인덱스에서 해소된다**.
    ///
    /// # ⚠ 이름이 `UniqueSpan` 이 아니다 — 그것이 결함이었다 (2026-08-15 · `[f10.pass]` ⑤)
    ///
    /// 처음에는 **유일하게 해소될 때만** 이 신호를 냈다. 그러면 같은 이름이 둘인 스팬은
    /// 신호를 **아예 안 내고**, 그 조각이 더 약한 신호로 떨어져 결국 **미결박**이 된다.
    ///
    /// **그것이 `[f10.pass]` ⑤의 반대 방향이 금지한 바로 그 형태다** — *"동점을
    /// 미결박으로 접으면 그것도 반증이다. 「여럿이라 못 좁혔다」와 「신호가 없다」는
    /// 다른 답이고, 뭉개면 작업 목록에 이미 후보가 있는 것이 섞인다."*
    /// 등록한 합격선이 구현을 잡았고, **`scripts/f10-verify.py` ⑤가 그것을 냈다.**
    ///
    /// 문서 §3.2 의 마지막 줄이 그 답을 이미 적어 두었다: *"같은 강도의 후보가 여럿이면
    /// **확정하지 않는다.** 후보 목록을 제안하고 승인을 요구한다."* — 즉 **유일함은
    /// 이 신호의 조건이 아니라 [`resolve`] 가 후보 하나를 볼 때 하는 일**이다.
    Span,
    /// 문서와 **같은 커밋**에서 함께 바뀌었다.
    SameCommit,
    /// 문서 경로와 **디렉터리가 가깝다** — `docs/order/` ↔ `src/order/`.
    DirectoryProximity,
}

impl ResolutionSignal {
    /// **강한 것부터.** 이 배열의 순서가 계단식이고, 순서를 바꾸면 판정이 바뀐다.
    pub const ALL: [Self; 6] = [
        Self::Attached,
        Self::Frontmatter,
        Self::FencedPath,
        Self::Span,
        Self::SameCommit,
        Self::DirectoryProximity,
    ];

    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Attached => "attached",
            Self::Frontmatter => "frontmatter",
            Self::FencedPath => "fenced-path",
            Self::Span => "span",
            Self::SameCommit => "same-commit",
            Self::DirectoryProximity => "directory-proximity",
        }
    }

    /// 이 신호가 **기계가 확인한 사실**인가 — 판단이 안 들어갔는가.
    ///
    /// # 이 함수가 합격선의 성분이다
    ///
    /// `[f10.2].sample_selection` 이 **거짓 결박률 표본을 신호 종류로 층화**하라고
    /// 요구한다. 확인된 신호만 담으면 그 값이 **0 으로 나오고 아무것도 안 잰다** —
    /// 경로가 대장에 있는지와 이름이 유일한지는 조회이지 판단이 아니기 때문이다.
    #[must_use]
    pub const fn is_confirmed(self) -> bool {
        matches!(self, Self::Attached | Self::Frontmatter | Self::FencedPath | Self::Span)
    }
}

/// 조각 하나의 3분류 (§2).
///
/// # 왜 「후보 있음」과 「미결박」이 갈려 있는가
///
/// 뭉개면 *"여럿이라 못 좁혔다"* 와 *"신호가 없다"* 가 같은 화면이 되고, 그러면
/// 사람의 작업 목록(`narrative.unbound`)에 **이미 후보가 있는 것**이 섞인다.
/// [`crate::TouchAnswer`] 가 `Ambiguous` 를 `Unknown` 과 가른 것과 같은 판단이다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "class")]
pub enum Classification {
    /// 강한 신호로 좌표가 **유일하게** 해소됐다.
    Bound { target: SymbolId, by: ResolutionSignal },
    /// 후보가 여럿이다. **하나를 고르지 않는다** — 고르는 것은 사람의 일이다.
    Candidates { by: ResolutionSignal, candidates: Vec<SymbolId> },
    /// 신호가 없다 — **이것이 사람의 작업 목록이다**(§2).
    Unbound,
}

impl Classification {
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Bound { .. } => "bound",
            Self::Candidates { .. } => "candidates",
            Self::Unbound => "unbound",
        }
    }
}

/// 조각 하나를 좌표에 건다 — **신호를 강한 것부터, 첫 번째로 걸리는 것에서 멈춘다.**
///
/// # 왜 「멈춘다」인가
///
/// 더 약한 신호로 내려가 동점을 깨면 그것이 §4 가 적은 거짓 결박의 원인
/// (*"약한 신호로 확정"*)이다. **여럿이면 여럿으로 나간다.**
///
/// # 왜 신호 하나 안에서만 동점을 보는가
///
/// 신호를 섞어 교집합을 내면 *"강한 신호로 걸렸다"* 가 거짓이 된다 — 실제로 좁힌 것은
/// 약한 신호이기 때문이다. **무엇이 걸었는지가 산출에 실려야 하므로**(모듈 머리)
/// 걸린 신호와 좁힌 신호가 같아야 한다.
#[must_use]
pub fn resolve(f: &Fragment, c: &impl Coordinates) -> Classification {
    for signal in ResolutionSignal::ALL {
        let mut found = candidates(signal, f, c);
        if found.is_empty() {
            continue;
        }
        // **결정적 순서** — 회차마다 순서가 달라지면 후보 목록이 흔들리고,
        // 흔들리는 목록은 승인의 근거가 못 된다(`rebind::propose_with_shape` 와 같은 자리).
        found.sort();
        found.dedup();
        if let [only] = found[..] {
            return Classification::Bound { target: only, by: signal };
        }
        return Classification::Candidates { by: signal, candidates: found };
    }
    Classification::Unbound
}

/// 신호 하나가 내는 후보들. **없으면 빈 목록** — 억지로 채우지 않는다.
fn candidates(s: ResolutionSignal, f: &Fragment, c: &impl Coordinates) -> Vec<SymbolId> {
    match s {
        ResolutionSignal::Attached => f.signals.attached.clone(),
        ResolutionSignal::Frontmatter => {
            f.signals.grounds.iter().flat_map(|g| by_ground(g, c)).collect()
        }
        ResolutionSignal::FencedPath => f
            .signals
            .fenced_paths
            .iter()
            .flat_map(|p| c.in_path(p))
            .map(|n| n.id)
            .collect(),
        ResolutionSignal::Span => {
            // ⚠ **해소되는 것을 전부 낸다.** 유일한 것만 내면 같은 이름이 둘인 스팬이
            // 신호를 아예 못 내고, 그 조각이 **미결박으로 접힌다** — 「여럿이라 못
            // 좁혔다」와 「신호가 없다」가 같은 답이 되는 것이고 `[f10.pass]` ⑤의
            // 반대 방향이 금지한 형태다. 유일함은 [`resolve`] 가 후보 하나를 볼 때 한다.
            f.signals.spans.iter().flat_map(|s| by_span(s, c)).collect()
        }
        ResolutionSignal::SameCommit => f
            .signals
            .co_changed
            .iter()
            .flat_map(|p| c.in_path(p))
            .map(|n| n.id)
            .collect(),
        ResolutionSignal::DirectoryProximity => {
            nearby_prefixes(&f.path).into_iter().flat_map(|p| c.under_prefix(&p)).map(|n| n.id).collect()
        }
    }
}

/// `src/order/cancel.ts#OrderService.cancel` 형태를 좌표로.
///
/// **`#` 앞은 경로이고 뒤는 이름이다.** 뒤가 없으면 그 파일의 심볼 전부이고,
/// 그러면 대개 여럿이라 **확정되지 않는다** — 그것이 정확한 답이다.
fn by_ground(raw: &str, c: &impl Coordinates) -> Vec<SymbolId> {
    let (path, rest) = raw.split_once('#').map_or((raw, ""), |(a, b)| (a, b));
    let in_file = c.in_path(&RepoPath::new(path));
    if rest.is_empty() {
        return in_file.into_iter().map(|n| n.id).collect();
    }
    let (chain, name) = split_qualified(rest);
    in_file
        .into_iter()
        .filter(|n| n.name == name && chain_matches(&chain, &n.container))
        .map(|n| n.id)
        .collect()
}

/// `` `OrderService.cancel` `` 을 좌표로 — **이름으로 찾고 컨테이너로 좁힌다.**
fn by_span(raw: &str, c: &impl Coordinates) -> Vec<SymbolId> {
    let raw = raw.trim();
    if !looks_like_an_identifier(raw) {
        return Vec::new();
    }
    let (chain, name) = split_qualified(raw);
    c.by_name(&name)
        .into_iter()
        .filter(|n| chain_matches(&chain, &n.container))
        .map(|n| n.id)
        .collect()
}

/// `A.B.c` → (`["A", "B"]`, `"c"`). 점이 없으면 체인이 빈다.
fn split_qualified(raw: &str) -> (Vec<&str>, String) {
    let mut parts: Vec<&str> = raw.split(['.', '#']).filter(|p| !p.is_empty()).collect();
    let name = parts.pop().unwrap_or("").to_owned();
    (parts, name)
}

/// 적힌 체인이 실제 체인의 **꼬리**인가.
///
/// 문서는 `OrderService.cancel` 이라고만 적지 전체 경로를 안 적는다. 전체 일치를
/// 요구하면 이 신호가 **아무것도 못 건다** — 그리고 그 실패는 조용하다(미결박이 는다).
fn chain_matches(written: &[&str], actual: &[String]) -> bool {
    if written.is_empty() {
        return true;
    }
    written.len() <= actual.len()
        && actual[actual.len() - written.len()..]
            .iter()
            .zip(written)
            .all(|(a, w)| a == w)
}

/// 식별자처럼 생겼는가 — **문장을 스팬에 넣은 것을 거른다.**
///
/// 인라인 스팬에는 명령줄·URL·산문 조각이 흔하다. 거르지 않으면 `by_name` 이 전부
/// 빈 목록을 내므로 **틀리지는 않지만**, 무엇을 시도했는지가 산출에서 사라진다.
fn looks_like_an_identifier(raw: &str) -> bool {
    !raw.is_empty()
        && raw.len() <= 128
        && raw.chars().all(|ch| ch.is_alphanumeric() || ch == '_' || ch == '.' || ch == '#')
        && raw.chars().next().is_some_and(|ch| ch.is_alphabetic() || ch == '_')
}

/// 문서 경로에서 **가까운 소스 접두어**를 만든다 — `docs/order/x.md` → `src/order/`.
///
/// **가장 약한 신호다.** 이것만으로 걸리는 조각은 대개 여럿을 내고, 여럿은 확정되지
/// 않는다. 그것이 이 신호가 계단식의 바닥에 있는 이유다.
fn nearby_prefixes(doc: &RepoPath) -> Vec<String> {
    let dirs: Vec<&str> = doc.as_str().split('/').collect();
    // 마지막은 파일 이름이다.
    let Some(tail) = dirs.len().checked_sub(1).and_then(|n| dirs.get(..n)) else {
        return Vec::new();
    };
    // **문서 디렉터리의 마지막 성분**이 도메인 이름이라고 본다 — `docs/order/` 의 `order`.
    let Some(domain) = tail.last().copied().filter(|d| !d.is_empty() && d != &"docs") else {
        return Vec::new();
    };
    ["src", "lib", "app"].iter().map(|root| format!("{root}/{domain}/")).collect()
}

// ─────────────────────────────────────────────────────────────────────────────
// 제안 — **`inferred` 다. 승인 없이 `asserted` 가 되지 않는다** (§1 · §3.3)
// ─────────────────────────────────────────────────────────────────────────────

/// 승인을 기다리는 제안 하나 — **`inferred` 다.**
///
/// **[graph-node] `NarrativeItem`** — `schema/graph.toml`
///
/// # 이것이 저장되지 않는다
///
/// **결정론적 파생이다** — 같은 문서·같은 스냅샷·같은 계단식이면 같은 제안이 나온다
/// (자연어 유사도를 안 쓰는 것이 그것을 보장한다). 저장하면 재생 경로를 하나 더
/// 지어야 하고, **2층에 두면 `[f05.2]` ④의 모집단이 늘어 남의 합격선이 움직인다.**
/// 근거 전문은 `corpus/criteria.toml` `[f10].queue_placement`.
///
/// **저장되는 것은 둘뿐이다**: 개체의 이름(민팅해서 재계산이 불가능하다)과
/// **거부 기록**(사람이 한 일이라 계산에서 안 나온다).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Proposal {
    /// 이 문서 조각의 개체 — [`crate::EntityOrigin::Document`] 를 진다.
    pub item: crate::entity::EntityId,
    pub fragment: Fragment,
    pub class: Classification,
}

impl Proposal {
    /// 사람이 고를 수 있는 좌표들. **`Unbound` 면 빈 목록이고 승인할 것이 없다.**
    #[must_use]
    pub fn choices(&self) -> Vec<SymbolId> {
        match &self.class {
            Classification::Bound { target, .. } => vec![*target],
            Classification::Candidates { candidates, .. } => candidates.clone(),
            Classification::Unbound => Vec::new(),
        }
    }

    /// 무엇이 이 후보들을 냈나. **`Unbound` 면 없다.**
    #[must_use]
    pub const fn signal(&self) -> Option<ResolutionSignal> {
        match &self.class {
            Classification::Bound { by, .. } | Classification::Candidates { by, .. } => Some(*by),
            Classification::Unbound => None,
        }
    }
}

/// 승격이 **거부되는** 이유 — 값으로 남는다.
///
/// [`crate::BatchRefusal`] 과 같은 형태다: *"승인할 수 없다"* 만 적으면 사람이 무엇을
/// 손으로 봐야 하는지 모른다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PromotionRefusal {
    /// 후보가 하나도 없다 — **승인할 것이 없다.** 이 조각은 `narrative.unbound` 에 남는다.
    NothingToApprove,
    /// 고른 좌표가 **후보에 없다.**
    ///
    /// ★ **이것이 세탁을 막는 자리다.** 후보 밖의 좌표를 승인할 수 있으면 사람이
    /// *"아마 이것일 것"* 을 넣을 수 있고, 그러면 **제안이 아니라 지어낸 것이 `asserted`
    /// 가 된다.** 지어내려면 `pal bind` 를 써야 하고, 그것은 [`PromotedBy::Hand`] 로
    /// 남는다 — **어느 쪽인지가 값으로 갈린다.**
    NotACandidate { picked: SymbolId, candidates: usize },
}

impl std::fmt::Display for PromotionRefusal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NothingToApprove => f.write_str(
                "이 조각에는 좌표 후보가 없습니다 — 승인할 것이 없고 미결박으로 남습니다",
            ),
            Self::NotACandidate { picked, candidates } => write!(
                f,
                "`{picked}` 은 이 제안의 후보 {candidates}건에 없습니다 — 후보 밖의 좌표를 \
                 승인하면 그것은 승격이 아니라 새로 지어낸 결박입니다. `pal bind` 를 쓰십시오"
            ),
        }
    }
}

impl std::error::Error for PromotionRefusal {}

/// 사람이 **거부한** 제안 하나 — 그리고 그 기록이 남는다 (§3.3).
///
/// **[graph-node] `NarrativeRefusal`** — `schema/graph.toml`
///
/// # 왜 이것이 의도 저장소에 사는가 — **재생 불가하기 때문이다**
///
/// [`Proposal`] 은 다시 계산되지만 *"사람이 이것을 거부했다"* 는 **계산에서 안 나온다.**
/// 그리고 문서 §3.3 이 그 값을 못 박았다:
///
/// > 거부해도 기록된다 ← **재질문 제거가 승인 비용 절감의 대부분**
///
/// 지워지면 다음 인입이 같은 제안을 다시 올리고, 사람은 같은 것을 다시 거부한다.
/// **그 순간 이 기능의 값이 사라진다** — [R-21] 이 가르는 선이 정확히 여기다.
///
/// # 이유가 값이다
///
/// [`crate::BatchRefusal`] 과 같은 판단이다 — *"거부했다"* 만 적으면 다음 사람이
/// **왜 거부됐는지 모른 채 같은 후보를 다시 본다.** 그래서 표면이 이유를 요구한다.
///
/// [R-21]: ../../../docs/plan/00-risks.md#r-21
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Refusal {
    /// 어느 조각인가.
    pub item: crate::entity::EntityId,
    /// 어느 후보를 거부했나. **조각 전체가 아니라 (조각, 좌표) 짝이다** —
    /// 후보 셋 중 하나만 틀렸을 수 있고, 그때 나머지 둘까지 지우면 정보를 버린 것이다.
    pub target: SymbolId,
    /// 언제 거부했나.
    pub at: crate::repo::Snapshot,
    /// 왜 거부했나 — **사람이 적는다.**
    pub reason: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coord::Discriminator;
    use crate::repo::RepoId;
    use crate::symbol::SymbolKind;

    #[derive(Default)]
    struct 표(Vec<NamedCoord>);

    impl 표 {
        fn 더(mut self, path: &str, container: &[&str], name: &str) -> Self {
            let chain: Vec<&str> = container.to_vec();
            self.0.push(NamedCoord {
                id: SymbolId::compute(
                    &RepoId::new("r"),
                    &RepoPath::new(path),
                    &chain,
                    name,
                    &Discriminator::new(SymbolKind::Function, 0),
                ),
                name: name.to_owned(),
                container: container.iter().map(|s| (*s).to_owned()).collect(),
                path: RepoPath::new(path),
            });
            self
        }

        fn 좌표(&self, path: &str, name: &str) -> SymbolId {
            self.0
                .iter()
                .find(|n| n.path.as_str() == path && n.name == name)
                .expect("표에 없다")
                .id
        }
    }

    impl Coordinates for 표 {
        fn by_name(&self, name: &str) -> Vec<NamedCoord> {
            self.0.iter().filter(|n| n.name == name).cloned().collect()
        }
        fn in_path(&self, path: &RepoPath) -> Vec<NamedCoord> {
            self.0.iter().filter(|n| n.path == *path).cloned().collect()
        }
        fn under_prefix(&self, prefix: &str) -> Vec<NamedCoord> {
            self.0.iter().filter(|n| n.path.as_str().starts_with(prefix)).cloned().collect()
        }
    }

    fn 조각(path: &str, signals: RawSignals) -> Fragment {
        Fragment {
            path: RepoPath::new(path),
            anchor: "머리".to_owned(),
            body: "본문".to_owned(),
            signals,
        }
    }

    #[test]
    fn 신호가_없으면_미결박이다() {
        // **빈 것이 정확한 답이다** — 억지로 채우면 그것이 거짓 결박이다.
        let t = 표::default().더("src/order/cancel.ts", &[], "cancel");
        assert_eq!(resolve(&조각("docs/x.md", RawSignals::default()), &t), Classification::Unbound);
    }

    #[test]
    fn 유일한_스팬이_확정한다() {
        let t = 표::default().더("src/order/cancel.ts", &["OrderService"], "cancel");
        let f = 조각(
            "docs/x.md",
            RawSignals { spans: vec!["OrderService.cancel".to_owned()], ..RawSignals::default() },
        );
        assert_eq!(
            resolve(&f, &t),
            Classification::Bound {
                target: t.좌표("src/order/cancel.ts", "cancel"),
                by: ResolutionSignal::Span,
            }
        );
    }

    #[test]
    fn 동점이면_확정하지_않는다() {
        // **★ 반대 방향 ⑤.** 억지로 하나를 고르면 그것이 §5 가 기각한 거짓 결박이다.
        let t = 표::default()
            .더("src/a/cancel.ts", &[], "cancel")
            .더("src/b/cancel.ts", &[], "cancel");
        let f = 조각(
            "docs/x.md",
            RawSignals { spans: vec!["cancel".to_owned()], ..RawSignals::default() },
        );
        // ★ **여럿이면 「후보 있음」이다. 미결박이 아니다.**
        // 「여럿이라 못 좁혔다」와 「신호가 없다」는 다른 답이고, 뭉개면 작업 목록에
        // **이미 후보가 있는 것**이 섞인다(`[f10.pass]` ⑤의 반대 방향).
        let Classification::Candidates { by, candidates } = resolve(&f, &t) else {
            panic!("스팬 동점이 「후보 있음」이 아니다 — 미결박으로 접혔다");
        };
        assert_eq!(by, ResolutionSignal::Span);
        assert_eq!(candidates.len(), 2);

        // 그런데 **경로 신호는 여럿을 낸다** — 그때는 「후보 있음」이지 미결박이 아니다.
        let g = 조각(
            "docs/x.md",
            RawSignals {
                fenced_paths: vec![RepoPath::new("src/a/cancel.ts"), RepoPath::new("src/b/cancel.ts")],
                ..RawSignals::default()
            },
        );
        let Classification::Candidates { by, candidates } = resolve(&g, &t) else {
            panic!("동점이 확정됐다 — 거짓 결박이다");
        };
        assert_eq!(by, ResolutionSignal::FencedPath);
        assert_eq!(candidates.len(), 2);
    }

    #[test]
    fn 동점을_약한_신호로_깨지_않는다() {
        // **★ 이것이 이 모듈의 가장 무거운 판단이다.** 약한 신호로 내려가 동점을 깨면
        // *"강 신호로 걸렸다"* 가 거짓이 되고, 그것이 §4 가 적은 거짓 결박의 원인이다.
        let t = 표::default()
            .더("src/order/a.ts", &[], "cancel")
            .더("src/other/b.ts", &[], "cancel");
        let f = 조각(
            // 디렉터리 근접성이면 `src/order/` 하나로 좁혀질 자리다 — **좁히지 않는다.**
            "docs/order/x.md",
            RawSignals {
                fenced_paths: vec![RepoPath::new("src/order/a.ts"), RepoPath::new("src/other/b.ts")],
                ..RawSignals::default()
            },
        );
        assert!(matches!(resolve(&f, &t), Classification::Candidates { candidates, .. } if candidates.len() == 2));
    }

    #[test]
    fn 강한_신호가_약한_것을_이긴다() {
        // 계단식이 아니라 목록이면 이 단언이 무너진다.
        let t = 표::default()
            .더("src/order/a.ts", &[], "cancel")
            .더("src/order/b.ts", &[], "other");
        let f = 조각(
            "docs/order/x.md",
            RawSignals {
                fenced_paths: vec![RepoPath::new("src/order/a.ts")],
                // 같은 커밋·근접성은 둘을 다 낸다 — 그런데 **더 강한 것이 이미 하나를 냈다.**
                co_changed: vec![RepoPath::new("src/order/b.ts")],
                ..RawSignals::default()
            },
        );
        assert_eq!(
            resolve(&f, &t),
            Classification::Bound {
                target: t.좌표("src/order/a.ts", "cancel"),
                by: ResolutionSignal::FencedPath,
            }
        );
    }

    #[test]
    fn 붙어_있는_주석이_가장_강하다() {
        let t = 표::default().더("src/order/a.ts", &[], "cancel");
        let id = t.좌표("src/order/a.ts", "cancel");
        let f = 조각(
            "src/order/a.ts",
            RawSignals { attached: vec![id], ..RawSignals::default() },
        );
        assert_eq!(
            resolve(&f, &t),
            Classification::Bound { target: id, by: ResolutionSignal::Attached }
        );
    }

    #[test]
    fn 프론트매터가_경로와_이름을_함께_읽는다() {
        let t = 표::default()
            .더("src/order/cancel.ts", &["OrderService"], "cancel")
            .더("src/order/cancel.ts", &[], "helper");
        let f = 조각(
            "docs/x.md",
            RawSignals {
                grounds: vec!["src/order/cancel.ts#OrderService.cancel".to_owned()],
                ..RawSignals::default()
            },
        );
        assert_eq!(
            resolve(&f, &t),
            Classification::Bound {
                target: t.좌표("src/order/cancel.ts", "cancel"),
                by: ResolutionSignal::Frontmatter,
            }
        );
    }

    #[test]
    fn 대장에_없는_경로는_아무것도_안_낸다() {
        // *"경로가 대장에 존재하는가"* 가 이 신호의 정의다(§3.2). 없으면 신호가 없다.
        let t = 표::default().더("src/order/a.ts", &[], "cancel");
        let f = 조각(
            "docs/x.md",
            RawSignals {
                fenced_paths: vec![RepoPath::new("src/없다.ts")],
                ..RawSignals::default()
            },
        );
        assert_eq!(resolve(&f, &t), Classification::Unbound);
    }

    #[test]
    fn 산문은_식별자가_아니다() {
        // 인라인 스팬에는 명령줄·URL·산문이 흔하다. 거르지 않으면 무엇을 시도했는지가
        // 산출에서 사라진다.
        assert!(looks_like_an_identifier("OrderService.cancel"));
        assert!(looks_like_an_identifier("cancel"));
        assert!(!looks_like_an_identifier("주문 취소 로직"));
        assert!(!looks_like_an_identifier("npm run build"));
        assert!(!looks_like_an_identifier("--radius=symbol"));
        assert!(!looks_like_an_identifier(""));
        assert!(!looks_like_an_identifier("1234"));
    }

    #[test]
    fn 컨테이너는_꼬리로_맞춘다() {
        // 문서는 전체 경로를 안 적는다. 전체 일치를 요구하면 이 신호가 아무것도 못 건다.
        assert!(chain_matches(&[], &["A".to_owned()]));
        assert!(chain_matches(&["B"], &["A".to_owned(), "B".to_owned()]));
        assert!(!chain_matches(&["A"], &["A".to_owned(), "B".to_owned()]));
        assert!(!chain_matches(&["A", "B"], &["B".to_owned()]));
    }

    #[test]
    fn 확인된_신호와_판단이_드는_신호가_갈린다() {
        // **`[f10.2].sample_selection` 이 이 구별 위에 선다** — 확인된 것만 표본에 담으면
        // 거짓 결박률이 0 으로 나오고 아무것도 안 잰다.
        let 확인 = ResolutionSignal::ALL.iter().filter(|s| s.is_confirmed()).count();
        let 판단 = ResolutionSignal::ALL.len() - 확인;
        assert!(확인 >= 1 && 판단 >= 1, "두 갈래가 다 서지 않는다");
        assert!(!ResolutionSignal::SameCommit.is_confirmed());
        assert!(ResolutionSignal::Span.is_confirmed());
        assert!(!ResolutionSignal::DirectoryProximity.is_confirmed());
    }

    #[test]
    fn 계단식의_순서가_이름과_함께_고정된다() {
        // 순서를 바꾸면 판정이 바뀐다. **바꾸는 커밋이 이 시험을 지나가야 한다.**
        let 이름: Vec<&str> = ResolutionSignal::ALL.iter().map(|s| s.name()).collect();
        assert_eq!(
            이름,
            vec!["attached", "frontmatter", "fenced-path", "span", "same-commit",
                 "directory-proximity"]
        );
        // 그리고 서로 다르다 — 뭉개지면 산출이 무엇이 걸었는지 못 말한다.
        let 집합: std::collections::BTreeSet<&str> = 이름.iter().copied().collect();
        assert_eq!(집합.len(), 이름.len());
    }
}
