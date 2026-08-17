//! 계획-구현 결박과 이탈 — **이미 지불되고 있는 승인에 올라탄다**(F12).
//!
//! > 의도층의 최대 비용은 **사람의 노동**이다([R-10]). 그런데 계획 검토는 **이미
//! > 일어나고 있다** — 거기에 좌표를 붙이면 **새 노동 없이** 의도층이 채워진다.
//!
//! # 이 모듈이 결박을 만들지 않는다 — 그리고 그것이 이 모듈의 가장 무거운 결정이다
//!
//! [F12 §3.1] 의 표는 *"기존 심볼 → 즉시 해소 → **결박**"* 이라고 적었다.
//! **이 구현은 그 줄을 따르지 않고, 안 따르는 이유를 적는다.**
//!
//! [ADR-0015] 가 F10 의 반증에서 나왔다 — *"**기계가 확인한 것은 이름의 유일성이지
//! 주제의 일치가 아니다.**"* 실측이 `span` 신호로 걸린 결박의 **48.9%** 가 엉뚱한
//! 좌표임을 보였고, 그래서 [`crate::ConfirmingSignal`] 이 **거리 0 인 신호만** 확정을
//! 낼 수 있게 타입으로 막았다.
//!
//! **F12 의 좌표 후보는 셋 다 거리가 있다** — 계획 문장에 이름이 나온다는 것과 그
//! 항목이 그 좌표를 건드릴 것이라는 것은 다른 문장이다. 그러므로 여기서 `asserted`
//! 결박을 만들면 그것이 정확히 F10 이 반증당한 형태다.
//!
//! [F12 §4] 가 이 기능의 성질을 이미 적었으므로 어긋나지 않는다:
//!
//! > `unplanned` 가 나쁜 것이 아니다. **판정이 아니라 관측이다.** 분류만 하고
//! > 평가하지 않는다.
//!
//! **관측은 [`PlanBinding`] 이 지고, 확정은 여전히 `pal bind` 하나뿐이다.**
//! 그래서 [`crate::PromotedBy`] 가 **둘 그대로**이고 이 모듈이 그것을 안 늘린다 —
//! 늘리면 세탁 금지가 타입에서 문장으로 내려앉는다(F10 §3.3).
//!
//! # `pending` 이 [`crate::Binding`] 이 아닌 이유
//!
//! [F03 §3.3] 이 *"L0 에서 결박을 만들 수 없다"* 를 **타입으로** 세웠다 —
//! [`crate::NewBinding`] 이 [`crate::SymbolId`] 를 요구하고, 그래서 **좌표 없는 결박은
//! 이 빌드에서 타입상 존재할 수 없다.** `[f09].freshness_boundary` ⓐ 가 그것을 적고
//! *"`subject` 만 있고 `target` 이 없는 결박은 F10·F12 의 것"* 이라고 넘겼다.
//!
//! **여기가 그 만기이고, 처분은 「[`crate::CodeFreshness`] 에 `Pending` 을 더하는 것」이
//! 아니다.** 같은 절 ⓑ 가 근거의 형태를 이미 세웠다 — `NodeFreshness` 와 안 합친 이유가
//! *"**모집단이 다르다.** 합치면 `[f22.4]` 불변식 8 의 모집단이 바뀐다"* 였다.
//! [`crate::CodeFreshness`] 를 만드는 자리는 [`crate::BindingStatus::evaluate`] 하나이고
//! 그 입력인 [`crate::Binding`] 은 **정의상 `pending` 일 수 없다.** 더하면 아무도 못
//! 만드는 변형([ADR-0012] 가 금한 짝 없는 이름)이거나, 두 번째 생산자를 두어
//! **한 열거가 두 모집단 위에 서게** 된다.
//!
//! 그래서 [`PlanBindingState`] 는 **자기 열거**이고 [`crate::Binding`] 은 한 글자도
//! 안 움직인다.
//!
//! [R-10]: ../../../docs/plan/00-risks.md#r-10
//! [ADR-0012]: ../../../docs/adr/0012-a-single-truth-file-declares-only-what-has-a-counterpart-in-code.md
//! [ADR-0015]: ../../../docs/adr/0015-a-machine-confirmed-signal-must-say-what-it-confirmed.md
//! [ADR-0019]: ../../../docs/adr/0019-the-site-of-the-repair-is-not-the-site-of-the-defect.md
//! [F03 §3.3]: ../../../docs/plan/disposal-map.md
//! [F12 §3.1]: ../../../docs/plan/disposal-map.md
//! [F12 §4]: ../../../docs/plan/disposal-map.md

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::coord::SymbolId;
use crate::glob::Glob;
use crate::narrative::{Coordinates, NamedCoord};
use crate::repo::RepoPath;
use crate::touch::SymbolNode;

/// 계획 하나의 이름. **내용에서 유도한다** — 같은 문서를 두 번 읽어도 하나다.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct PlanId(String);

/// 계획 항목 하나의 이름.
///
/// # 왜 [`crate::EntityId`] 가 아닌가
///
/// [`crate::EntityId::mint`] 는 부를 때마다 다른 이름을 내므로 **산출이 회차마다
/// 달라진다**, 그리고 [`crate::EntityId::derived`] 는 자기 문서가 *"옛 판을 올릴 때만
/// 쓴다"* 라고 못 박았다. **이 모듈은 아무것도 저장하지 않으므로**(제안과 달리 승인의
/// 대상이 아니다) 이름이 계산에서 나와야 하고, 그 형태는
/// [`crate::BindingId::derive`] 가 이미 세웠다.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct PlanItemId(String);

impl PlanId {
    /// `(문서 경로, 머리말)` 에서 유도한다.
    #[must_use]
    pub fn derive(source: &RepoPath, headline: &str) -> Self {
        Self(digest16(b"pal-plan-v1\0", &[source.as_str().as_bytes(), headline.as_bytes()]))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl PlanItemId {
    /// `(계획, 앵커, 문장)` 에서 유도한다 — **문장이 들어가는 것이 요점이다.**
    /// 같은 앵커의 항목이 고쳐지면 다른 항목이고, 그래야 이탈이 옛 문장에 안 걸린다.
    #[must_use]
    pub fn derive(plan: &PlanId, anchor: &str, statement: &str) -> Self {
        Self(digest16(
            b"pal-plan-item-v1\0",
            &[plan.as_str().as_bytes(), anchor.as_bytes(), statement.as_bytes()],
        ))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PlanId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl fmt::Display for PlanItemId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

fn digest16(domain: &[u8], parts: &[&[u8]]) -> String {
    let mut h = blake3::Hasher::new();
    h.update(domain);
    for p in parts {
        h.update(p);
        h.update(b"\0");
    }
    h.finalize().to_hex().as_str()[..16].to_owned()
}

// ─────────────────────────────────────────────────────────────────────────────
// 예상 좌표 — **세 형태**([F12 §3.1] 의 표)
// ─────────────────────────────────────────────────────────────────────────────

/// **무엇이** 이 후보를 냈나 — 산출에 실린다.
///
/// # 왜 이 값이 필요한가
///
/// [ADR-0015] 가 요구한 것은 *"확인된 명제를 문장으로"* 다. F12 의 후보는 셋 다
/// **거리가 있으므로** 확정을 못 내는데, 그렇다고 무엇이 냈는지를 지우면 게이트가
/// 신호별로 갈라 셀 수 없다 — `[f10.pass]` ①의 층화와 같은 자리다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PatternSource {
    /// 계획이 좌표를 **명시**했다 — `좌표:` 줄. 사람이 적은 것이라 가장 강하다.
    ///
    /// [F12 §4] 가 이슈의 대응 ③ 으로 적은 *"계획 작성 시 좌표를 요구하는 템플릿"* 이
    /// 이 신호를 낸다. ⚠ **실 코퍼스에서 0 일 수 있고, 0 이면 그 사실을 적는다** —
    /// ditto 의 계획 항목은 이 표기를 안 쓴다.
    Declared,
    /// 인라인 코드 스팬 — `` `OrderService.cancel` ``.
    Span,
    /// 본문의 camelCase·PascalCase 토큰.
    ///
    /// ⚠ **이것이 가장 약하다.** [F10] 은 이 형태를 아예 안 봤다(스팬 안만 봤다).
    /// F12 가 범위를 넓힌 근거는 실측이다 — ditto 의 계획 항목 575 중 백틱을 쓴 것이
    /// **5** 건이라, F10 의 규칙을 그대로 대면 후보가 코퍼스의 성질이 아니라
    /// **우리가 고른 규칙의 성질**로 0 이 된다.
    Identifier,
    /// 본문의 경로처럼 생긴 토큰.
    Path,
}

impl PatternSource {
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Declared => "declared",
            Self::Span => "span",
            Self::Identifier => "identifier",
            Self::Path => "path",
        }
    }

    /// 넷 — 게이트가 층화해 셀 때 쓴다.
    pub const ALL: [Self; 4] = [Self::Declared, Self::Span, Self::Identifier, Self::Path];
}

/// 계획 항목이 **어디를 건드릴 것인가** — 좌표가 아니라 패턴이다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "form")]
pub enum CoordPattern {
    /// **기존 심볼.** 기준선에서 해소돼야 한다([ADR-0019] 의 자격 검사).
    Symbol { name: String, by: PatternSource },
    /// **아직 없는 심볼** — 계획이 `(신규)` 를 명시했다.
    ///
    /// ⚠ **명시가 조건이다.** 기준선에 없는 이름을 「신규」로 자동 승격하면, 계획
    /// 문장에 우연히 나온 낱말이 나중에 심볼이 되기만 해도 「계획대로」가 공짜로 는다 —
    /// **답을 보고 분류하는 것**이다.
    NewSymbol { name: String, by: PatternSource },
    /// **경로 패턴.** 정밀도는 낮지만 `unmeasurable` 보다 낫다([F12 §3.1]).
    Paths { glob: Glob },
}

impl CoordPattern {
    /// 산출과 화면이 쓰는 한 줄.
    #[must_use]
    pub fn display(&self) -> String {
        match self {
            Self::Symbol { name, by } => format!("{name} ({})", by.name()),
            Self::NewSymbol { name, .. } => format!("{name} (신규)"),
            Self::Paths { glob } => format!("{} (경로)", glob.as_str()),
        }
    }

    /// 어느 신호가 냈나. **경로 패턴은 [`PatternSource::Path`] 다.**
    #[must_use]
    pub const fn source(&self) -> PatternSource {
        match self {
            Self::Symbol { by, .. } | Self::NewSymbol { by, .. } => *by,
            Self::Paths { .. } => PatternSource::Path,
        }
    }
}

/// **무엇으로 됐다고 판정할 것인가**([F12 §2]).
///
/// # `Option<String>` 이 아닌 이유
///
/// `None` 이 *"안 적혔다"* 인지 *"모른다"* 인지 구별되지 않는다([ADR-0005]).
/// 계획 문서가 판정 방법을 안 적은 것은 **사실이지 결측이 아니고**, 그 사실이
/// 이 기능의 산출에 남아야 한다 — 계획 관행을 재는 값이기 때문이다.
///
/// [ADR-0005]: ../../../docs/adr/0005-absence-carries-its-kind.md
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum VerificationStep {
    /// 계획이 적었다 — 본문의 `검증:` 줄.
    Stated { how: String },
    /// **안 적혔다.** 「모른다」가 아니다.
    NotStated,
}

impl VerificationStep {
    #[must_use]
    pub const fn is_stated(&self) -> bool {
        matches!(self, Self::Stated { .. })
    }
}

/// 계획 항목 하나 — **결정**(§3.3 의 2 단 중 아래).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanItem {
    pub id: PlanItemId,
    /// 문서 안의 자리 — 헤딩 앵커이거나 `<앵커>#<n>`(체크박스 줄).
    pub anchor: String,
    /// 무엇을 할 것인가. **문장 그대로다.**
    pub statement: String,
    /// 어디를 건드릴 것인가 — **비어 있는 것이 정확한 값일 수 있다.**
    pub expected: Vec<CoordPattern>,
    pub verification: VerificationStep,
}

/// 이 계획의 **기준선**([F12 §4]).
///
/// > **base 커밋 선택** — 어디부터가 이 계획의 변경인가 …
/// > **계획 승인 시점의 Snapshot 을 계획에 기록**
///
/// ⚠ **`--base <ref>` 를 안 만든다.** 그 손잡이의 소유자는 [F23 §7] 이고, 거기 완료
/// 체크리스트가 `briefing · conformance · **deviation**` 셋을 한 줄로 묶어 적었다.
/// 여기서 만들면 F23 을 당겨오는 것이다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum PlanBaseline {
    /// 프론트매터가 `baseline: <rev>` 를 적었다.
    Declared { rev: String },
    /// **안 적혔다.** 이탈을 계산할 수 없고, 그 사실이 값이다.
    NotDeclared,
}

/// 계획 하나 — **기획 하나와 결정 여럿**(§3.3).
///
/// # `items` 가 비공개인 것이 [F12 §2] 의 `NonEmpty` 다
///
/// 문서가 `Plan { items: NonEmpty<PlanItem> }` 이라고 적었다. 이 저장소에 `NonEmpty`
/// 타입이 없으므로 **생성자가 그 불변식을 진다** — 항목 0 인 계획은
/// [`PlanRefusal::NoItems`] 로 **거부되고**, 그 거부가 §3.3 의 1 단(기획→결정)
/// 미해소를 세는 자리다. 조용히 빈 계획을 만들면 그 수가 사라진다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Plan {
    pub id: PlanId,
    /// 어느 문서인가. **정체성이 아니라 추적용이다.**
    pub source: RepoPath,
    /// **기획** — 문서의 첫 조각(§3.3 의 2 단 중 위).
    pub headline: String,
    pub baseline: PlanBaseline,
    items: Vec<PlanItem>,
}

/// 계획을 세울 수 없는 이유 — **값으로 남는다.**
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case", tag = "why")]
pub enum PlanRefusal {
    /// 항목이 하나도 없다 — **기획은 있는데 결정이 없다**(§3.3 의 1 단 미해소).
    NoItems { source: RepoPath },
}

impl fmt::Display for PlanRefusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoItems { source } => write!(
                f,
                "`{source}` 에 계획 항목이 없습니다 — 헤딩 조각도 체크박스 줄도 \
                 없습니다. **기획은 있는데 결정이 없는 것**이고, 그 자체가 F12 §3.3 이 \
                 재는 값입니다"
            ),
        }
    }
}

impl std::error::Error for PlanRefusal {}

impl Plan {
    /// 계획 하나. **항목이 비면 거부한다.**
    ///
    /// # Errors
    /// 항목이 하나도 없으면.
    pub fn new(
        source: RepoPath,
        headline: String,
        baseline: PlanBaseline,
        items: Vec<PlanItem>,
    ) -> Result<Self, PlanRefusal> {
        if items.is_empty() {
            return Err(PlanRefusal::NoItems { source });
        }
        Ok(Self { id: PlanId::derive(&source, &headline), source, headline, baseline, items })
    }

    /// 항목들. **비어 있지 않다** — 생성자가 그것을 진다.
    #[must_use]
    pub fn items(&self) -> &[PlanItem] {
        &self.items
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 해소 — **`pending` 이 여기 산다**
// ─────────────────────────────────────────────────────────────────────────────

/// 왜 아직 못 걸었나.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PendingReason {
    /// 계획이 `(신규)` 를 **명시**했다 — 아직 안 만들었다.
    DeclaredNew,
    /// 경로 패턴이 기준선에서 파일을 하나도 안 맞춘다 — **자리가 아직 없다.**
    PathAbsent,
}

/// 왜 좌표로 안 좁혀졌나 — **`unmeasurable` 의 사유가 이것이다.**
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum UnresolvedWhy {
    /// ★ **이름이 기준선의 2 층에서 해소되지 않는다** — [ADR-0019] 의 자격 검사.
    ///
    /// > 좌표는 사건 **직전** 스냅샷에서 해소되는 것이어야 한다. 표본을 채울 때
    /// > *"이 이름이 `c^` 의 2 층에 서는가"* 를 **자격으로** 검사한다.
    ///
    /// ⚠ **여기서 머리 스냅샷을 보지 않는다.** 보면 계획 문장에 우연히 나온 낱말이
    /// 나중에 심볼이 되기만 해도 「계획대로」가 되고, 그것이 **답을 보고 분류하는 것**이다.
    NotAtBaseline,
    /// 여럿으로 해소된다 — **하나를 고르지 않는다.**
    Many,
    /// 패턴이 맞추는 파일이 상한을 넘었다 — *"좁히십시오"*([F12 §4]).
    PatternTooBroad,
}

impl UnresolvedWhy {
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::NotAtBaseline => "not-at-baseline",
            Self::Many => "many",
            Self::PatternTooBroad => "pattern-too-broad",
        }
    }

    pub const ALL: [Self; 3] = [Self::NotAtBaseline, Self::Many, Self::PatternTooBroad];
}

/// 패턴 하나의 상태.
///
/// ⚠ **[`crate::CodeFreshness`] 를 안 쓴다.** 모집단이 다르다 — 저기는 **결박** 위에
/// 서고 여기는 **계획 항목** 위에 선다. 합치면 `[f22.4]` 불변식 8 의 모집단이 바뀐다
/// (`[f09].freshness_boundary` ⓑ 와 같은 근거).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "state")]
pub enum PlanBindingState {
    /// 좌표가 섰다. **[F12 §3.1] 의 `live` 가 이 상태다** — 새 이름이 아니라
    /// *"상태를 계산할 자격을 얻었다"* 이고, 그 계산은 [`crate::BindingStatus`] 의 것이다.
    Bound { targets: Vec<SymbolId> },
    /// ★ **기준선에 아직 없다** — 계획이 코드를 선행한 자리.
    ///
    /// [F12 §3.1]: *"신생 프로젝트에서는 의도가 코드를 선행하므로 결박할 좌표가 없다.
    /// **「아직 만들지 않았다」와 「만든 뒤 어긋났다」를 구별하는 것**이 F09 가 5 상태를
    /// 가진 이유다."*
    Pending { why: PendingReason },
    /// 안 좁혀졌다. **하나를 고르지 않는다**(P6).
    Unresolved { why: UnresolvedWhy, candidates: Vec<SymbolId> },
}

impl PlanBindingState {
    /// 걸린 좌표들. **`Bound` 가 아니면 빈 목록이고 그것이 정확한 값이다.**
    #[must_use]
    pub fn targets(&self) -> &[SymbolId] {
        match self {
            Self::Bound { targets } => targets,
            Self::Pending { .. } | Self::Unresolved { .. } => &[],
        }
    }

    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Bound { .. } => "bound",
            Self::Pending { .. } => "pending",
            Self::Unresolved { .. } => "unresolved",
        }
    }
}

/// 계획 항목 하나가 패턴 하나에 대해 얻은 자리 — **결박이 아니다.**
///
/// 이름이 `PlanBinding` 인 것은 `[f09].freshness_boundary` ⓐ 가 이 자리를
/// *"`subject` 만 있고 `target` 이 없는 결박"* 이라고 부른 것을 따른 것이다.
/// **그러나 [`crate::Binding`] 이 아니고, 만들어지지도 않는다**(모듈 머리).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanBinding {
    pub item: PlanItemId,
    pub expected: CoordPattern,
    pub state: PlanBindingState,
}

/// 항목 하나의 해소 — **두 스냅샷에서 각각.**
///
/// # 왜 둘인가 — `pending → live` 전이가 여기서만 보인다
///
/// 기준선에서 `Pending` 이던 것이 머리에서 `Bound` 가 되는 것이 [F12 §7] 의
/// *"`pending` 결박 → 좌표 생성 시 자동 `live` 전이"* 다. 한쪽만 계산하면 그 전이가
/// 산출에서 사라진다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ItemResolution {
    pub item: PlanItemId,
    pub at_baseline: Vec<PlanBinding>,
    pub at_head: Vec<PlanBinding>,
}

impl ItemResolution {
    /// ★ **이 항목의 예상 좌표 집합 `E_i`.**
    ///
    /// # 머리 스냅샷의 해소를 **선별해서만** 받는다
    ///
    /// 기준선에서 `Pending` 이던 패턴(신규 선언 · 아직 없는 경로)만 머리의 해소를
    /// 받는다. **기준선에서 `Unresolved{NotAtBaseline}` 이던 이름은 안 받는다** —
    /// 받으면 계획 문장에 우연히 나온 낱말이 나중에 심볼이 되기만 해도 「계획대로」가
    /// 되고, 그것이 답을 보고 분류하는 것이다.
    #[must_use]
    pub fn expected_coords(&self) -> BTreeSet<SymbolId> {
        let mut out: BTreeSet<SymbolId> = BTreeSet::new();
        for b in &self.at_baseline {
            out.extend(b.state.targets().iter().copied());
        }
        // **패턴은 항목마다 몇 개뿐이다** — `Ord` 를 요구하는 집합 대신 훑는다.
        // `CoordPattern` 에 `Ord` 를 붙이면 [`Glob`] 까지 딸려 오고, 그 순서에는
        // 아무 뜻이 없다.
        let pending: Vec<&CoordPattern> = self
            .at_baseline
            .iter()
            .filter(|b| matches!(b.state, PlanBindingState::Pending { .. }))
            .map(|b| &b.expected)
            .collect();
        for b in &self.at_head {
            if pending.contains(&&b.expected) {
                out.extend(b.state.targets().iter().copied());
            }
        }
        out
    }

    /// ★ **계획이 자리를 적었는데 그 자리가 아직 없는가** — [F12 §4] 의 한 줄이다.
    ///
    /// > **`pending` 이 영원히 안 풀림** | 계획했는데 안 만듦 | 정상이다.
    /// > **`unimplemented` 로 잡히는 것이 이 기능의 산출**
    ///
    /// ⚠ **그러므로 이 경우는 `unmeasurable` 이 아니다.** 좌표가 없는 것은 같지만
    /// 사건이 다르다 — *"어디를 건드릴지 못 알아냈다"* 와 *"어디를 건드릴지 적었고
    /// 안 건드렸다"* 는 사람이 다르게 처리한다.
    #[must_use]
    pub fn still_pending(&self) -> bool {
        self.at_head.iter().any(|b| matches!(b.state, PlanBindingState::Pending { .. }))
    }

    /// 이 항목이 **왜 못 재는가** — 걸린 좌표가 하나도 없을 때의 사유.
    ///
    /// 사유가 여럿이면 [`UnresolvedWhy::ALL`] 의 순서에서 첫째를 싣는다
    /// ([`crate::CodeFreshness::Undeterminable`] 과 같은 규율 — 회차마다 다른 사유가
    /// 나오면 밀도가 지도가 못 된다).
    #[must_use]
    pub fn unmeasurable_why(&self) -> UnresolvedWhy {
        let mut seen: BTreeSet<UnresolvedWhy> = BTreeSet::new();
        for b in self.at_baseline.iter().chain(&self.at_head) {
            if let PlanBindingState::Unresolved { why, .. } = &b.state {
                seen.insert(*why);
            }
        }
        UnresolvedWhy::ALL
            .into_iter()
            .find(|w| seen.contains(w))
            // 후보가 아예 없는 항목 — 좌표를 하나도 안 적은 계획 문장이다.
            .unwrap_or(UnresolvedWhy::NotAtBaseline)
    }

    /// 기준선에서 `Pending` 이던 것이 머리에서 `Bound` 가 됐는가 — **전이의 관측.**
    #[must_use]
    pub fn promoted_from_pending(&self) -> usize {
        self.at_baseline
            .iter()
            .filter(|b| matches!(b.state, PlanBindingState::Pending { .. }))
            .filter(|b| {
                self.at_head.iter().any(|h| {
                    h.expected == b.expected && matches!(h.state, PlanBindingState::Bound { .. })
                })
            })
            .count()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 스냅샷 하나의 창 — **2층을 모른다**([`Coordinates`] 와 같은 형태)
// ─────────────────────────────────────────────────────────────────────────────

/// 스냅샷 하나에서 이름과 경로를 묻는 자리.
///
/// [`Coordinates`] 를 구현한다 — 그래야 F10 이 세운 이름 해소 규칙(컨테이너 꼬리
/// 일치 등)을 **다시 안 쓴다.** `files` 가 따로 있는 것은 **선언이 하나도 없는 파일**
/// 때문이다: 심볼에서 경로를 뽑으면 그런 파일이 사라지고, 그러면 *"경로가 아직 없다"*
/// 와 *"그 파일에 선언이 없다"* 가 같은 답이 된다.
#[derive(Debug, Clone, Copy)]
pub struct SnapshotView<'a> {
    pub symbols: &'a [SymbolNode],
    pub files: &'a [RepoPath],
}

impl Coordinates for SnapshotView<'_> {
    fn by_name(&self, name: &str) -> Vec<NamedCoord> {
        self.symbols
            .iter()
            .filter(|s| s.name == name)
            .map(|s| NamedCoord {
                id: s.id,
                name: s.name.clone(),
                container: s.container.clone(),
                path: s.path.clone(),
            })
            .collect()
    }

    fn in_path(&self, path: &RepoPath) -> Vec<NamedCoord> {
        self.symbols
            .iter()
            .filter(|s| &s.path == path)
            .map(|s| NamedCoord {
                id: s.id,
                name: s.name.clone(),
                container: s.container.clone(),
                path: s.path.clone(),
            })
            .collect()
    }
}

/// 패턴 하나를 스냅샷 하나에 댄다.
///
/// `pattern_file_max` 는 [`crate::PROVISIONAL_PLAN_PATTERN_FILE_MAX`] 다 —
/// **부르는 쪽이 지고 온다**(예산은 손잡이가 아니라 인자다).
#[must_use]
pub fn resolve_pattern(
    pattern: &CoordPattern,
    at: &SnapshotView<'_>,
    pattern_file_max: usize,
) -> PlanBindingState {
    match pattern {
        CoordPattern::Symbol { name, .. } => by_name(name, at),
        CoordPattern::NewSymbol { name, .. } => {
            let found = by_name(name, at);
            // **아직 없으면 그것이 정상이다.** 계획이 코드를 선행한 자리다.
            if matches!(&found, PlanBindingState::Unresolved { why: UnresolvedWhy::NotAtBaseline, .. })
            {
                return PlanBindingState::Pending { why: PendingReason::DeclaredNew };
            }
            found
        }
        CoordPattern::Paths { glob } => {
            let hit: Vec<&RepoPath> = at.files.iter().filter(|p| glob.matches(p.as_str())).collect();
            if hit.len() > pattern_file_max {
                // ⚠ **거부는 조용하지 않다** — 사유가 값으로 실리고 화면이 좁히라고 말한다.
                return PlanBindingState::Unresolved {
                    why: UnresolvedWhy::PatternTooBroad,
                    candidates: Vec::new(),
                };
            }
            if hit.is_empty() {
                // **경로는 산문 낱말일 수 없다.** 없으면 그것이 *"아직 없는 자리"* 다.
                return PlanBindingState::Pending { why: PendingReason::PathAbsent };
            }
            let mut targets: Vec<SymbolId> = Vec::new();
            for p in hit {
                targets.extend(at.in_path(p).into_iter().map(|n| n.id));
            }
            targets.sort();
            targets.dedup();
            if targets.is_empty() {
                // 파일은 있는데 선언이 없다 — **재지 못하는 것이지 아직 없는 것이 아니다.**
                return PlanBindingState::Unresolved {
                    why: UnresolvedWhy::NotAtBaseline,
                    candidates: Vec::new(),
                };
            }
            PlanBindingState::Bound { targets }
        }
    }
}

/// 이름 하나 → 상태. **여럿이면 안 고른다.**
fn by_name(raw: &str, at: &SnapshotView<'_>) -> PlanBindingState {
    let (chain, name) = split_qualified(raw);
    let mut found: Vec<SymbolId> = at
        .by_name(&name)
        .into_iter()
        .filter(|n| chain_is_tail(&chain, &n.container))
        .map(|n| n.id)
        .collect();
    found.sort();
    found.dedup();
    match found.len() {
        0 => PlanBindingState::Unresolved {
            why: UnresolvedWhy::NotAtBaseline,
            candidates: Vec::new(),
        },
        1 => PlanBindingState::Bound { targets: found },
        _ => PlanBindingState::Unresolved { why: UnresolvedWhy::Many, candidates: found },
    }
}

/// `A.B.c` → (`["A","B"]`, `"c"`). [`crate::narrative`] 의 같은 규칙이다.
fn split_qualified(raw: &str) -> (Vec<&str>, String) {
    let mut parts: Vec<&str> = raw.split(['.', '#']).filter(|p| !p.is_empty()).collect();
    let name = parts.pop().unwrap_or("").to_owned();
    (parts, name)
}

/// 적힌 체인이 실제 체인의 **꼬리**인가 — 계획도 전체 경로를 안 적는다.
fn chain_is_tail(written: &[&str], actual: &[String]) -> bool {
    if written.is_empty() {
        return true;
    }
    written.len() <= actual.len()
        && actual[actual.len() - written.len()..].iter().zip(written).all(|(a, w)| a == w)
}

/// 계획 하나를 두 스냅샷에 댄다.
#[must_use]
pub fn resolve(
    plan: &Plan,
    baseline: &SnapshotView<'_>,
    head: &SnapshotView<'_>,
    pattern_file_max: usize,
) -> Vec<ItemResolution> {
    plan.items()
        .iter()
        .map(|item| ItemResolution {
            item: item.id.clone(),
            at_baseline: item
                .expected
                .iter()
                .map(|p| PlanBinding {
                    item: item.id.clone(),
                    expected: p.clone(),
                    state: resolve_pattern(p, baseline, pattern_file_max),
                })
                .collect(),
            at_head: item
                .expected
                .iter()
                .map(|p| PlanBinding {
                    item: item.id.clone(),
                    expected: p.clone(),
                    state: resolve_pattern(p, head, pattern_file_max),
                })
                .collect(),
        })
        .collect()
}

// ─────────────────────────────────────────────────────────────────────────────
// 심볼 단위 diff — **파일 단위로 하면 포매팅에 반응한다**([F12 §3.2] · §5)
// ─────────────────────────────────────────────────────────────────────────────

/// 두 스냅샷 사이에서 **의미가 변한 좌표들.**
///
/// # 왜 셋인가
///
/// `body_digest` 대조만 하면 **추가와 삭제가 사라진다** — 새 파일을 통째로 만든 변경이
/// *"변경 0"* 이 되고, 그러면 이탈률의 분모가 0 이 되어 이 지표가 침묵한다.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SymbolDelta {
    /// 좌표는 같은데 `body_digest` 가 다르다 — **의미가 변했다.**
    pub changed: Vec<SymbolId>,
    /// 머리에만 있다.
    pub added: Vec<SymbolId>,
    /// 기준선에만 있다.
    pub removed: Vec<SymbolId>,
}

impl SymbolDelta {
    /// 셋의 합집합 — 이탈 대조의 `A` 다.
    #[must_use]
    pub fn all(&self) -> BTreeSet<SymbolId> {
        self.changed.iter().chain(&self.added).chain(&self.removed).copied().collect()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.changed.len() + self.added.len() + self.removed.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// 두 심볼 집합을 댄다 — **`body_digest` 가 앵커다**(F03 · [R-07]).
///
/// 커밋 시각도 파일 mtime 도 안 읽는다. 포매터가 하루에 한 번 파일을 다 바꾸는
/// 저장소에서 그 둘은 전부 「변했다」를 내고, 그것이 [R-07] 이 치명이라 부른 실패다.
///
/// [R-07]: ../../../docs/plan/00-risks.md#r-07
#[must_use]
pub fn symbol_delta(base: &[SymbolNode], head: &[SymbolNode]) -> SymbolDelta {
    let b: BTreeMap<SymbolId, &SymbolNode> = base.iter().map(|s| (s.id, s)).collect();
    let h: BTreeMap<SymbolId, &SymbolNode> = head.iter().map(|s| (s.id, s)).collect();
    let mut d = SymbolDelta::default();
    for (id, hs) in &h {
        match b.get(id) {
            None => d.added.push(*id),
            Some(bs) if bs.body != hs.body => d.changed.push(*id),
            Some(_) => {}
        }
    }
    for id in b.keys() {
        if !h.contains_key(id) {
            d.removed.push(*id);
        }
    }
    // **정렬한다** — 산출이 회차마다 달라지면 골든도 대조도 안 선다.
    d.changed.sort();
    d.added.sort();
    d.removed.sort();
    d
}

// ─────────────────────────────────────────────────────────────────────────────
// 이탈 — **넷이고, `unmeasurable` 이 분리돼 있다**([F12 §2] · §5)
// ─────────────────────────────────────────────────────────────────────────────

/// 계획대로 간 짝 하나 — **무엇이 그 좌표를 냈는지를 함께 싣는다.**
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Planned {
    pub item: PlanItemId,
    pub coord: SymbolId,
    /// 어느 신호가 이 좌표를 냈나 — 게이트가 **층화해서** 센다.
    pub by: PatternSource,
}

/// 못 잰 항목 하나 — **사유가 값이다.**
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Unmeasured {
    pub item: PlanItemId,
    pub why: UnresolvedWhy,
}

/// 계획과 실제의 갈림 — **넷이다.**
///
/// # `unmeasurable` 을 분리하는 것이 이 타입의 요점이다 ([F12 §2])
///
/// > 좌표로 해소되지 않은 계획 항목을 「계획대로」나 「미구현」에 섞으면 **이탈률이
/// > 거짓말이 된다.** 못 잰 것은 못 쟀다고 적는다.
///
/// 그리고 [F12 §5] 가 *"`unmeasurable` 을 미구현에 합산"* 을 **기각한 대안**으로 적었다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Deviation {
    pub plan: PlanId,
    /// 계획대로 — **짝마다 한 줄**이다.
    pub as_planned: Vec<Planned>,
    /// 계획에 없던 변경. ⚠ **나쁜 것이 아니다**([F12 §4]) — 관측이다.
    pub unplanned: Vec<SymbolId>,
    /// 계획했으나 그 좌표가 안 변했다.
    pub unimplemented: Vec<PlanItemId>,
    /// ★ **좌표가 해소되지 않아 판정 불가.** 위 셋 중 어디에도 안 섞인다.
    pub unmeasurable: Vec<Unmeasured>,
    /// 실제 변경의 규모 — 이탈률의 분모가 어디서 왔는지.
    pub delta: SymbolDelta,
    /// `pending` 이던 것이 `live` 로 간 수 — [F12 §7] 의 전이.
    pub promoted_from_pending: usize,
}

impl Deviation {
    /// 이탈률 = `|A ∖ D| / |A|` — `[outcome]` M2 의 정의 그대로.
    ///
    /// # 왜 [`Option`] 이 아니라 [`DeviationRate`] 인가
    ///
    /// 실제 변경이 0 이면 비율이 **정의되지 않는다.** `None` 으로 내면 소비자가
    /// 그것을 0 으로 접고, 그러면 *"하나도 안 벗어났다"* 와 *"잴 것이 없었다"* 가
    /// 같은 화면이 된다([ADR-0005]).
    #[must_use]
    pub fn rate(&self) -> DeviationRate {
        let a = self.delta.len();
        if a == 0 {
            return DeviationRate::Undefined;
        }
        #[allow(clippy::cast_precision_loss)]
        DeviationRate::Rate { value: self.unplanned.len() as f64 / a as f64, changed: a }
    }

    /// **판정할 수 있었던 항목의 비율** — [F12 §6] 의 좌표 해소율.
    ///
    /// 분자는 `as_planned ∪ unimplemented` 의 항목 수, 즉 **`unmeasurable` 이 아닌 것**이다.
    /// ⚠ **「좌표가 걸렸다」와 같지 않다** — 계획이 자리를 적었는데 그 자리가 아직 없는
    /// 항목([`ItemResolution::still_pending`])도 여기 든다. 그 항목에 대해 우리는
    /// *"계획했고 안 만들었다"* 를 **말할 수 있고**, 그것이 [F12 §4] 가 요구한 산출이다.
    ///
    /// **분모는 계획 항목 전부다.** `unmeasurable` 을 빼면 이 값이 정의상 1.0 이 된다.
    #[must_use]
    pub fn resolution(&self) -> Resolution {
        let total = self.measured_items() + self.unmeasurable.len();
        Resolution { resolved: self.measured_items(), total }
    }

    /// 좌표가 해소된 항목의 수 — `as_planned` 와 `unimplemented` 의 항목 합집합.
    #[must_use]
    pub fn measured_items(&self) -> usize {
        let mut s: BTreeSet<&PlanItemId> = self.as_planned.iter().map(|p| &p.item).collect();
        s.extend(self.unimplemented.iter());
        s.len()
    }

    /// 신호별 층화 — `as_planned` 가 무엇 위에 섰는지.
    #[must_use]
    pub fn by_source(&self) -> BTreeMap<&'static str, usize> {
        let mut out: BTreeMap<&'static str, usize> =
            PatternSource::ALL.into_iter().map(|s| (s.name(), 0)).collect();
        for p in &self.as_planned {
            *out.entry(p.by.name()).or_insert(0) += 1;
        }
        out
    }

    /// `unmeasurable` 의 사유별 분해 — 뭉치면 *"왜 못 쟀는가"* 가 사라진다.
    #[must_use]
    pub fn unmeasurable_by_reason(&self) -> BTreeMap<&'static str, usize> {
        let mut out: BTreeMap<&'static str, usize> =
            UnresolvedWhy::ALL.into_iter().map(|w| (w.name(), 0)).collect();
        for u in &self.unmeasurable {
            *out.entry(u.why.name()).or_insert(0) += 1;
        }
        out
    }
}

/// 이탈률 — **정의되지 않는 것과 0 이 다른 값이다.**
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum DeviationRate {
    Rate { value: f64, changed: usize },
    /// 실제 변경이 0 이다 — **잴 것이 없었다.** 「안 벗어났다」가 아니다.
    Undefined,
}

/// 좌표 해소율 — **분자와 분모를 함께 낸다**(값 하나만 내는 보고는 `[outcome]` 위반).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Resolution {
    pub resolved: usize,
    pub total: usize,
}

/// 이탈을 계산한다 — **규칙은 `[f12].deviation_rule` 이 재기 전에 고정했다.**
#[must_use]
pub fn deviate(plan: &Plan, resolutions: &[ItemResolution], delta: &SymbolDelta) -> Deviation {
    let actual = delta.all();
    let mut as_planned = Vec::new();
    let mut unimplemented = Vec::new();
    let mut unmeasurable = Vec::new();
    let mut declared: BTreeSet<SymbolId> = BTreeSet::new();
    let mut promoted = 0usize;

    // 항목 → 패턴의 출처. `as_planned` 줄에 무엇이 그 좌표를 냈는지를 싣기 위해서다.
    let source_of: BTreeMap<(&PlanItemId, SymbolId), PatternSource> = resolutions
        .iter()
        .flat_map(|r| {
            r.at_baseline.iter().chain(&r.at_head).flat_map(move |b| {
                b.state.targets().iter().map(move |t| ((&r.item, *t), b.expected.source()))
            })
        })
        .collect();

    for r in resolutions {
        promoted += r.promoted_from_pending();
        let e = r.expected_coords();
        if e.is_empty() {
            if r.still_pending() {
                // ★ **[F12 §4]** — *"계획했는데 안 만듦. 정상이다. `unimplemented` 로
                // 잡히는 것이 이 기능의 산출."* 좌표가 없는 것은 아래와 같지만
                // **사건이 다르다.**
                unimplemented.push(r.item.clone());
            } else {
                // ★ **여기가 [F12 §2] 가 분리를 요구한 자리다.**
                unmeasurable.push(Unmeasured { item: r.item.clone(), why: r.unmeasurable_why() });
            }
            continue;
        }
        declared.extend(e.iter().copied());
        let hit: Vec<SymbolId> = e.intersection(&actual).copied().collect();
        if hit.is_empty() {
            unimplemented.push(r.item.clone());
        } else {
            for coord in hit {
                as_planned.push(Planned {
                    item: r.item.clone(),
                    coord,
                    by: source_of
                        .get(&(&r.item, coord))
                        .copied()
                        // 짝이 없을 수 없다 — `e` 가 `targets()` 에서 나왔다.
                        .unwrap_or(PatternSource::Identifier),
                });
            }
        }
    }

    let mut unplanned: Vec<SymbolId> = actual.difference(&declared).copied().collect();
    unplanned.sort();
    as_planned.sort_by(|a, b| (a.item.as_str(), a.coord).cmp(&(b.item.as_str(), b.coord)));
    unimplemented.sort();
    unmeasurable.sort_by(|a, b| a.item.as_str().cmp(b.item.as_str()));

    Deviation {
        plan: plan.id.clone(),
        as_planned,
        unplanned,
        unimplemented,
        unmeasurable,
        delta: delta.clone(),
        promoted_from_pending: promoted,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coord::{BodyDigest, Discriminator};
    use crate::ledger::IdentityGrade;
    use crate::repo::RepoId;
    use crate::symbol::{Span, SymbolKind};

    fn 심볼(name: &str, path: &str, body: u8) -> SymbolNode {
        let id = SymbolId::compute(
            &RepoId::new("r"),
            &RepoPath::new(path),
            &[],
            name,
            &Discriminator::new(SymbolKind::Function, 0),
        );
        SymbolNode {
            id,
            path: RepoPath::new(path),
            container: Vec::new(),
            name: name.to_owned(),
            kind: SymbolKind::Function,
            body: BodyDigest::of_normalized(&[body]),
            span: Span { line_start: 1, line_end: 2, byte_start: 0, byte_end: 1 },
            identity: IdentityGrade::Exact,
        }
    }

    fn 계획(items: Vec<PlanItem>) -> Plan {
        Plan::new(
            RepoPath::new("docs/plan.md"),
            "기획".to_owned(),
            PlanBaseline::Declared { rev: "abc".to_owned() },
            items,
        )
        .expect("항목이 있다")
    }

    fn 항목(anchor: &str, statement: &str, expected: Vec<CoordPattern>) -> PlanItem {
        let plan = PlanId::derive(&RepoPath::new("docs/plan.md"), "기획");
        PlanItem {
            id: PlanItemId::derive(&plan, anchor, statement),
            anchor: anchor.to_owned(),
            statement: statement.to_owned(),
            expected,
            verification: VerificationStep::NotStated,
        }
    }

    #[test]
    fn 항목이_없는_계획은_거부된다() {
        // **기획은 있는데 결정이 없다** — §3.3 의 1 단 미해소이고, 조용히 빈 계획을
        // 만들면 그 수가 사라진다.
        let r = Plan::new(
            RepoPath::new("docs/x.md"),
            "머리".to_owned(),
            PlanBaseline::NotDeclared,
            Vec::new(),
        );
        assert!(matches!(r, Err(PlanRefusal::NoItems { .. })));
    }

    #[test]
    fn 포매팅만_바뀌면_변경_심볼이_0_이고_본문이_바뀌면_1_이다() {
        // ★ [F12 §5] 가 파일 단위 diff 를 기각한 이유가 이것이다 — **양쪽을 함께 센다.**
        let base = vec![심볼("a", "src/a.ts", 1), 심볼("b", "src/a.ts", 2)];
        let 같음 = vec![심볼("a", "src/a.ts", 1), 심볼("b", "src/a.ts", 2)];
        assert!(symbol_delta(&base, &같음).is_empty(), "포매팅에 반응했다");

        let 다름 = vec![심볼("a", "src/a.ts", 9), 심볼("b", "src/a.ts", 2)];
        let d = symbol_delta(&base, &다름);
        assert_eq!(d.changed.len(), 1);
        assert!(d.added.is_empty() && d.removed.is_empty());
    }

    #[test]
    fn 추가와_삭제가_변경에_들어간다() {
        // 안 넣으면 **새 파일을 통째로 만든 변경**이 「변경 0」이 되고 지표가 침묵한다.
        let base = vec![심볼("a", "src/a.ts", 1)];
        let head = vec![심볼("b", "src/b.ts", 1)];
        let d = symbol_delta(&base, &head);
        assert_eq!((d.added.len(), d.removed.len(), d.changed.len()), (1, 1, 0));
        assert_eq!(d.all().len(), 2);
    }

    #[test]
    fn 이름이_기준선에_없으면_후보가_아니다() {
        // ★ [ADR-0019] 의 자격 검사. 머리에 있다고 승격하지 않는다 —
        // 승격하면 계획 문장의 낱말이 나중에 심볼이 되기만 해도 「계획대로」가 된다.
        let base: Vec<SymbolNode> = Vec::new();
        let head = vec![심볼("cancelOrder", "src/o.ts", 1)];
        let files = [RepoPath::new("src/o.ts")];
        let b = SnapshotView { symbols: &base, files: &files };
        let h = SnapshotView { symbols: &head, files: &files };

        let p = CoordPattern::Symbol {
            name: "cancelOrder".to_owned(),
            by: PatternSource::Identifier,
        };
        let plan = 계획(vec![항목("a-1", "취소를 고친다", vec![p])]);
        let res = resolve(&plan, &b, &h, 32);
        assert!(res[0].expected_coords().is_empty(), "머리의 해소가 새어 들어왔다");

        let d = deviate(&plan, &res, &symbol_delta(&base, &head));
        assert_eq!(d.unmeasurable.len(), 1);
        assert_eq!(d.unmeasurable[0].why, UnresolvedWhy::NotAtBaseline);
        // ★ **미구현에 합산되지 않는다** — [F12 §5] 가 기각한 형태다.
        assert!(d.unimplemented.is_empty());
        assert!(d.as_planned.is_empty());
    }

    #[test]
    fn 신규로_명시한_것은_pending_이고_좌표가_생기면_bound_다() {
        let base: Vec<SymbolNode> = Vec::new();
        let head = vec![심볼("refund", "src/o.ts", 1)];
        let files = [RepoPath::new("src/o.ts")];
        let b = SnapshotView { symbols: &base, files: &files };
        let h = SnapshotView { symbols: &head, files: &files };

        let plan = 계획(vec![항목(
            "a-1",
            "환불을 만든다",
            vec![CoordPattern::NewSymbol {
                name: "refund".to_owned(),
                by: PatternSource::Declared,
            }],
        )]);
        let res = resolve(&plan, &b, &h, 32);
        assert!(matches!(
            res[0].at_baseline[0].state,
            PlanBindingState::Pending { why: PendingReason::DeclaredNew }
        ));
        assert!(matches!(res[0].at_head[0].state, PlanBindingState::Bound { .. }));
        assert_eq!(res[0].promoted_from_pending(), 1);

        let d = deviate(&plan, &res, &symbol_delta(&base, &head));
        assert_eq!(d.as_planned.len(), 1, "전이한 좌표가 계획대로로 안 갔다");
        assert_eq!(d.promoted_from_pending, 1);
    }

    #[test]
    fn 여럿으로_해소되면_고르지_않는다() {
        let base = vec![심볼("run", "src/a.ts", 1), 심볼("run", "src/b.ts", 1)];
        let files = [RepoPath::new("src/a.ts"), RepoPath::new("src/b.ts")];
        let v = SnapshotView { symbols: &base, files: &files };
        let s = resolve_pattern(
            &CoordPattern::Symbol { name: "run".to_owned(), by: PatternSource::Identifier },
            &v,
            32,
        );
        let PlanBindingState::Unresolved { why, candidates } = s else {
            panic!("여럿인데 하나를 골랐다");
        };
        assert_eq!(why, UnresolvedWhy::Many);
        assert_eq!(candidates.len(), 2);
    }

    #[test]
    fn 넓은_패턴은_거부된다() {
        // [F12 §4] — *"`src/**` 는 무의미. 매칭 파일 수에 상한."*
        let files: Vec<RepoPath> =
            (0..40).map(|i| RepoPath::new(format!("src/f{i}.ts"))).collect();
        let symbols: Vec<SymbolNode> = Vec::new();
        let v = SnapshotView { symbols: &symbols, files: &files };
        let s = resolve_pattern(
            &CoordPattern::Paths { glob: Glob::new("src/**").expect("패턴") },
            &v,
            32,
        );
        assert!(matches!(
            s,
            PlanBindingState::Unresolved { why: UnresolvedWhy::PatternTooBroad, .. }
        ));
    }

    #[test]
    fn 아직_없는_경로는_pending_이다() {
        let symbols: Vec<SymbolNode> = Vec::new();
        let files: Vec<RepoPath> = Vec::new();
        let v = SnapshotView { symbols: &symbols, files: &files };
        let s = resolve_pattern(
            &CoordPattern::Paths { glob: Glob::new("src/new.ts").expect("패턴") },
            &v,
            32,
        );
        assert!(matches!(s, PlanBindingState::Pending { why: PendingReason::PathAbsent }));
    }

    #[test]
    fn 넷이_각각_선다() {
        // ★ `[f12.pass]` ① — 셋 중 하나라도 0 이면 그 분류는 이름만 있는 자리다.
        // 그리고 ② — `unmeasurable` 이 나머지에 안 섞인다.
        let base = vec![
            심볼("keep", "src/a.ts", 1),
            심볼("touched", "src/a.ts", 2),
            심볼("planned_gone", "src/b.ts", 3),
        ];
        let head = vec![
            심볼("keep", "src/a.ts", 1),
            심볼("touched", "src/a.ts", 9), // 변했다 → 계획대로
            심볼("planned_gone", "src/b.ts", 3), // 계획했는데 안 변했다 → 미구현
            심볼("surprise", "src/c.ts", 1), // 계획에 없던 변경
        ];
        let files = [RepoPath::new("src/a.ts"), RepoPath::new("src/b.ts")];
        let head_files =
            [RepoPath::new("src/a.ts"), RepoPath::new("src/b.ts"), RepoPath::new("src/c.ts")];
        let b = SnapshotView { symbols: &base, files: &files };
        let h = SnapshotView { symbols: &head, files: &head_files };

        let plan = 계획(vec![
            항목(
                "a-1",
                "만진다",
                vec![CoordPattern::Symbol {
                    name: "touched".to_owned(),
                    by: PatternSource::Span,
                }],
            ),
            항목(
                "a-2",
                "안 만진다",
                vec![CoordPattern::Symbol {
                    name: "planned_gone".to_owned(),
                    by: PatternSource::Span,
                }],
            ),
            항목("a-3", "좌표가 없는 문장", Vec::new()),
        ]);
        let d = deviate(&plan, &resolve(&plan, &b, &h, 32), &symbol_delta(&base, &head));

        assert_eq!(d.as_planned.len(), 1, "{:?}", d.as_planned);
        assert_eq!(d.unimplemented.len(), 1);
        assert_eq!(d.unplanned.len(), 1);
        assert_eq!(d.unmeasurable.len(), 1);
        // 이탈률 = |A∖D| / |A| = 1/2.
        assert_eq!(d.rate(), DeviationRate::Rate { value: 0.5, changed: 2 });
        // 해소율의 분모가 **항목 전부**다 — `unmeasurable` 을 빼면 정의상 1.0 이 된다.
        assert_eq!(d.resolution(), Resolution { resolved: 2, total: 3 });
    }

    #[test]
    fn 계획한_자리를_끝내_안_만들면_미구현이다() {
        // ★ [F12 §4] — *"`pending` 이 영원히 안 풀림 … **`unimplemented` 로 잡히는
        // 것이 이 기능의 산출**"*. `unmeasurable` 로 접으면 *"못 알아냈다"* 와
        // *"안 만들었다"* 가 같은 줄이 된다.
        let base = vec![심볼("a", "src/a.ts", 1)];
        let head = vec![심볼("a", "src/a.ts", 9)];
        let files = [RepoPath::new("src/a.ts")];
        let b = SnapshotView { symbols: &base, files: &files };
        let h = SnapshotView { symbols: &head, files: &files };
        let plan = 계획(vec![항목(
            "a-1",
            "환불을 만든다",
            vec![CoordPattern::NewSymbol {
                name: "refund".to_owned(),
                by: PatternSource::Declared,
            }],
        )]);
        let d = deviate(&plan, &resolve(&plan, &b, &h, 32), &symbol_delta(&base, &head));
        assert_eq!(d.unimplemented.len(), 1, "{d:?}");
        assert!(d.unmeasurable.is_empty(), "안 만든 것이 못 잰 것으로 갔다");
    }

    #[test]
    fn 변경이_0_이면_이탈률이_정의되지_않는다() {
        // *"하나도 안 벗어났다"* 와 *"잴 것이 없었다"* 는 다른 답이다.
        let base = vec![심볼("a", "src/a.ts", 1)];
        let files = [RepoPath::new("src/a.ts")];
        let v = SnapshotView { symbols: &base, files: &files };
        let plan = 계획(vec![항목(
            "a-1",
            "무엇",
            vec![CoordPattern::Symbol { name: "a".to_owned(), by: PatternSource::Span }],
        )]);
        let d = deviate(&plan, &resolve(&plan, &v, &v, 32), &symbol_delta(&base, &base));
        assert_eq!(d.rate(), DeviationRate::Undefined);
    }
}
