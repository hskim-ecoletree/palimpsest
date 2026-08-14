//! 결박과 낡음 — **제품의 심장**(F09).
//!
//! > 의도층의 모든 항목은 사실층의 좌표에 결박된다.
//! > **그 좌표의 코드가 변하면 기계는 그 결박이 낡았음을 안다.**
//!
//! 문서가 낡는 것은 막을 수 없다. 낡았다는 사실의 감지는 기계적으로 가능하고,
//! **낡음이 조용한 거짓 신호가 아니라 표시된 상태가 되는 순간 낡은 문서는 다시
//! 안전해진다.**

use serde::{Deserialize, Serialize};

use crate::coord::{BodyDigest, SymbolId};
use crate::entity::EntityId;
use crate::radius::Radius;
use crate::repo::Snapshot;

/// 결박 하나의 이름. 내용에서 유도하므로 같은 결박은 같은 이름을 갖는다.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct BindingId(String);

impl BindingId {
    #[must_use]
    pub fn new(raw: impl Into<String>) -> Self {
        Self(raw.into())
    }

    /// `(대상, 조각)` 에서 유도한다 — 같은 것을 두 번 결박해도 하나다.
    #[must_use]
    pub fn derive(target: SymbolId, note: &str) -> Self {
        let mut h = blake3::Hasher::new();
        h.update(b"pal-binding-v1\0");
        h.update(target.as_bytes());
        h.update(b"\0");
        h.update(note.as_bytes());
        Self(h.finalize().to_hex().as_str()[..16].to_owned())
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// 감시 집합의 한 항목 — **결박 시점의 스냅샷이다.**
///
/// 이 값이 현재와 다르면 그 심볼이 변한 것이고, 그것이 낡음의 유일한 근거다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WatchEntry {
    pub symbol: SymbolId,
    /// **결박할 때의** 본문 요약. 현재 값과 비교한다.
    pub digest: BodyDigest,
}

/// 결박한 코드의 커밋 시각 — **표시용이다. 앵커가 아니다.**
///
/// # 앵커로 쓰지 않는 것이 이 타입의 전부다 (F09 §6)
///
/// 선행 구현은 `code_bound_at`(커밋 시각)을 앵커로 썼고, 그러면 **포매팅 커밋에도
/// `stale` 이 켜진다** — [R-07](../../../docs/plan/00-risks.md#r-07)이 치명이라 부른
/// 실패를 그대로 맞는다. `body_digest` 가 더 강하다.
///
/// > **다만 시각은 표시용으로 함께 싣는다** — *"3주 전 코드 기준"* 이 *"12커밋 전"* 보다
/// > 읽힌다.
///
/// **그래서 이 값은 [`BindingStatus::evaluate`] 에 안 들어간다.** 계산 경로와 표시
/// 경로가 갈려 있어야 하고, `cargo xtask check` 가 그 갈림을 지킨다.
///
/// # `Option<i64>` 가 아닌 이유
///
/// `None` 이 *"모른다"* 인지 *"없다"* 인지 구별되지 않는다(stack §5.4 · ADR-0005).
/// 워킹트리에 건 결박에는 **커밋이 없으므로 시각도 없고**, 그것은 모르는 것이 아니다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
// **내부 태그를 안 쓴다** — `postcard` 가 못 싣는다([`Radius`] 와 같은 자리).
#[serde(rename_all = "snake_case")]
pub enum BoundTime {
    /// 커밋의 시각(에포크 초).
    Committed { epoch_secs: i64 },
    /// 워킹트리에 걸었다 — **커밋이 없으므로 시각도 없다.** 「모른다」가 아니라 「없다」다.
    Worktree,
    /// **옛 판(JSONL 1)이라 안 적혔다.** [`Self::Worktree`] 와 다른 사건이다 —
    /// 저기는 *"없다"* 이고 여기는 *"모른다"* 다. 뭉개면 화면이 *"워킹트리 기준"* 이라
    /// 적는데 실제로는 커밋에 걸린 결박이다. 그리고 0 을 넣으면 *"1970년 코드 기준"* 이 뜬다.
    Unrecorded,
}

/// 사람이 넣은 것 하나 — **의도 저장소가 소유한다.**
///
/// # 이 타입이 파생층에 살면 안 되는 이유 ([R-21](../../../docs/plan/00-risks.md#r-21))
///
/// 2층에 의도가 살면 *"지우고 재구축"* 이 **사람의 노동을 지우는 명령**이 되고,
/// 재구축 등가성 검사는 그 상태에서도 통과하므로 **검사가 유실을 정상으로 승인한다.**
/// 그래서 결박의 실체는 `pal-intent` 에 있고 파생층에는 색인만 둔다.
///
/// # 밖에서는 [`Binding::new`] 로만 만든다 ([F03 §3.3])
///
/// `#[non_exhaustive]` 가 크레이트 밖의 구조체 리터럴을 막는다. 그것이 *"L0 에서
/// 결박을 만들 수 없다"* 를 **타입으로** 세우는 자리다 — 생성자가 [`SymbolId`] 를
/// 요구하고, [`crate::SymbolIdentity::Unavailable`] 에서는 그 값을 꺼낼 수 없다.
///
/// 리터럴을 열어 두면 그 강제는 문장일 뿐이다. **읽는 것은 그대로 열려 있다** —
/// 막으려는 것은 *"없는 좌표로 결박을 만드는 것"* 이지 *"결박을 읽는 것"* 이 아니다.
///
/// ```compile_fail
/// # use pal_core::{Binding, BindingId, BoundTime, EntityId, EntityKind, EntityOrigin,
/// #                ObjectName, Radius, RepoId, Snapshot, SymbolId, TreeRef};
/// # let id = SymbolId::from_bytes([0; 32]);
/// // 크레이트 밖에서는 리터럴로 만들 수 없다 — 그래야 생성자를 지나간다.
/// let _ = Binding {
///     id: BindingId::derive(id, "메모"),
///     subject: EntityId::mint(EntityKind::new("decision"), EntityOrigin::Hand),
///     target: id,
///     note: "메모".to_owned(),
///     bound_at: Snapshot::single(RepoId::new("r"), TreeRef::Committed(ObjectName::from_bytes([0; 20]))),
///     bound_at_time: BoundTime::Worktree,
///     radius: Radius::Symbol,
///     watch: Vec::new(),
/// };
/// ```
///
/// **[graph-node] `Binding`** — `schema/graph.toml`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Binding {
    pub id: BindingId,
    /// **무엇이** 걸렸나 — 결정·계획·라벨 같은 비코드 개체([`crate::EntityId`] · F09 §4.3).
    ///
    /// `note` 는 그 개체의 **본문**이고 이것은 그 개체의 **이름**이다. 둘이 갈려 있어야
    /// 같은 결정을 여러 좌표에 걸 수 있고(F10·F12), 문서가 이동해도 이름이 남는다.
    pub subject: EntityId,
    /// 무엇에 걸었나.
    ///
    /// **[graph-edge] `BOUND_TO`** — `schema/graph.toml`. 이 엣지는 별도 자리가 아니라
    /// 여기 실려 있고, 공통 넷은 등급 `exact`(구조상 하나뿐) · 출처 `asserted` ·
    /// 근거 없음(`inferred` 로 설 수 없다) · 발생 `Snapshot`([`Binding::bound_at`])이다.
    pub target: SymbolId,
    /// 무엇을 걸었나. **S3 에서는 텍스트 조각 하나다** — 문서 인입은 F10.
    pub note: String,
    /// 언제 걸었나. 낡음은 이 시점 이후의 변화다.
    ///
    /// # `TreeRef` 가 아니라 `Snapshot` 이다 (F22 의 정본화 · 2026-08-12)
    ///
    /// 이 자리는 `BOUND_TO` 엣지가 지는 **발생 `Snapshot`** 이고, 그것은 모든 엣지가
    /// 공통으로 지는 넷 중 하나다([DESIGN §1.2](../../../docs/DESIGN.md)).
    /// S3 는 여기에 `TreeRef` 를 넣었다 — 저장소가 하나뿐이라 트리 하나가 곧 "지금"
    /// 이었기 때문이다. **멀티레포에서는 그것이 성립하지 않는다**(§1.1).
    ///
    /// 결박은 여러 저장소에 걸친 감시 집합을 가질 수 있으므로(반경은 F09) 이 값이
    /// 집합이 아니면 *"어느 저장소의 그때인가"* 를 적을 자리가 없다.
    pub bound_at: Snapshot,
    /// 그 커밋의 시각 — **표시용이다.** [`BoundTime`] 의 머리가 근거를 진다.
    pub bound_at_time: BoundTime,
    /// **무엇까지** 지켜보나 — 선언이지 계산이 아니다([`Radius`] · F09 §3).
    ///
    /// 이 값이 **판정 결과에 함께 출력된다.** *"이 결정은 `symbol` 반경에서 live"* 는
    /// *"이 결정은 유효하다"* 와 다른 문장이고, 그 차이가 산출에 남는 것이 요구다.
    pub radius: Radius,
    /// 무엇을 지켜보나 — [`Radius`] 가 편 결과.
    ///
    /// **결박 시점에 기계가 대상 좌표에서 읽은 값이다.** 생산자의 신고를 여기 넣는
    /// 경로가 없고([`Binding`] 머리 · F09 §4.1 D32), `cargo xtask check` 가 그 부재를 센다.
    pub watch: Vec<WatchEntry>,
}

impl Binding {
    /// 결박 하나를 만든다. **[`SymbolId`] 를 요구한다 — 그것이 이 생성자의 전부다.**
    ///
    /// [`crate::SymbolIdentity`] 를 받지 않는 이유가 F03 §3.3 이다: `Unavailable` 에는
    /// 실린 좌표가 없으므로, `SymbolId` 를 요구하면 **L0 에서 결박을 시도하는 코드가
    /// 컴파일되지 않는다.** 런타임 검사로 대신하면 그것은 규율이고, 규율은 잊힌다.
    ///
    /// `id` 는 `(대상, 조각)` 에서 유도한다 — 같은 것을 두 번 걸어도 하나다.
    ///
    /// # `id` 가 `subject` 를 안 쓴다
    ///
    /// **같은 조각을 같은 좌표에 두 번 걸면 하나다.** `subject` 를 열쇠에 넣으면 두
    /// 번째 결박이 새 개체를 만들어 **같은 것이 둘이 된다** — 그래서 부르는 쪽이
    /// 기존 결박의 `subject` 를 물려준다(`pal bind`).
    #[must_use]
    pub fn new(
        subject: EntityId,
        target: SymbolId,
        note: &str,
        bound_at: Snapshot,
        bound_at_time: BoundTime,
        radius: Radius,
        watch: Vec<WatchEntry>,
    ) -> Self {
        Self {
            id: BindingId::derive(target, note),
            subject,
            target,
            note: note.to_owned(),
            bound_at,
            bound_at_time,
            radius,
            watch,
        }
    }

    /// **판 1 의 결박을 그대로 되살린다** — JSONL 읽기 전용 문이다.
    ///
    /// # 이 함수가 [`Binding::new`] 와 다른 점 하나
    ///
    /// `id` 를 **유도하지 않고 받는다.** 옛 파일의 id 를 그대로 지켜야 하기 때문이다 —
    /// 다시 유도하면 같은 결박이 새 이름을 갖고, 그러면 **읽기가 더하기가 아니라
    /// 복제가 된다**(`[f05.4]` ②).
    ///
    /// # `subject` 를 뽑지 않고 **유도한다**
    ///
    /// [`crate::EntityId::mint`] 를 부르면 같은 파일을 두 번 읽을 때 개체가 둘이 된다.
    /// 읽기는 더하기이므로 두 번 읽는 것이 정상 경로이고, 그때 왕복이 항등이 아니게 된다.
    /// **여기는 옛 파일을 올리는 자리이지 개체를 만드는 자리가 아니다.**
    ///
    /// # 왜 `#[non_exhaustive]` 를 뚫는 문이 하나 필요한가
    ///
    /// 크레이트 밖에서 리터럴을 막는 것이 노리는 것은 *"없는 좌표로 결박을 만드는 것"*
    /// 이다([`Binding`] 머리). 이 함수는 **이미 있는 좌표를 되살린다** — `target` 을
    /// 지어내지 않고 파일에서 읽은 것을 그대로 쓴다. 그래서 그 강제를 안 깬다.
    #[must_use]
    pub fn from_v1(
        id: BindingId,
        target: SymbolId,
        note: &str,
        bound_at: Snapshot,
        watch: Vec<WatchEntry>,
    ) -> Self {
        Self {
            subject: crate::EntityId::derived(
                crate::EntityKind::new("decision"),
                crate::EntityOrigin::Hand,
                id.as_str().as_bytes(),
            ),
            id,
            target,
            note: note.to_owned(),
            bound_at,
            // **판 1 은 시각을 안 실었다.** `Worktree` 로 적으면 *"없다"* 가 되는데
            // 실제로는 *"모른다"* 다.
            bound_at_time: BoundTime::Unrecorded,
            // 판 1 의 감시 집합은 **언제나 대상 하나**였다. `Symbol` 로 올리는 것은
            // 추측이 아니라 **그 판의 사실을 적는 것**이다.
            radius: Radius::Symbol,
            watch,
        }
    }
}

/// **판정할 수 없는 이유** — F09 §2.1 · [R16].
///
/// 선행 구현은 앵커를 계산할 수 없을 때 **낡지 않은 것으로 접었다**(`stale=False`).
/// 방향은 같다 — 모르는 것을 낡았다고 하지 않는다. 그러나 **`false` 와 「판정 불가」를
/// 구별하지 않은 결과는 *"이 결정은 유효합니다"* 와 *"유효한지 알 수 없습니다"* 가
/// 같은 화면이 되는 것**이고, 그것이 [목표 §3.1] 의 정면 위반이다.
///
/// # ⚠ `ordinal` 은 여기 **없다** — 문서 §2.1 과 어긋나고, 어긋난 것은 문서다
///
/// 문서가 [`Self::IdentityGrade`] 를 *"L0 심볼 / **ordinal** 이라 digest 비교가 무의미"*
/// 라고 적었다. **`ordinal` 을 넣으면 이 코퍼스가 통째로 「판정 불가」가 된다** —
/// Kotlin 은 L1 이라 **전 심볼이 `Ordinal`** 이고(`ExtractGrade::L1.identity()`),
/// portal-backend 1,340 심볼이 전부 여기 걸린다. 그것이 DESIGN §15-42 가 경고한
/// *"지배하면 정직하지만 쓸모없다"* 이고, **S3 게이트가 이미 그 반증을 갖고 있다** —
/// Kotlin 결박이 본문 변경에 `stale` 로 실제로 켜졌다.
///
/// **약한 것과 불가능한 것을 뭉개지 않는다:**
///
/// | `ordinal` 이 뜻하는 것 | 처분 |
/// |---|---|
/// | 좌표가 **선언 순서**에 의존한다 | **약하다.** 숨기지 않고 산출에 싣는다 |
/// | 지역 이름을 **안 지운다**(ADR-0006) | **약하다.** 거짓 양성의 원천이고 `[f09.4]` 가 잰다 |
/// | 요약 값이 **있다** | **비교가 가능하다.** 판정 불가가 아니다 |
///
/// 그래서 [`Self::IdentityGrade`] 는 **비교할 값이 아예 없을 때**만 켠다 —
/// 등급이 [`crate::IdentityGrade::Unavailable`](L0)일 때다.
/// 근거 전문은 `corpus/criteria.toml` `[f09].ordinal_is_not_undeterminable`.
///
/// [R16]: ../../../docs/evidence-map.md
/// [목표 §3.1]: ../../../docs/plan/00-goals.md
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UndeterminableReason {
    /// 감시 집합 원소의 등급이 `Unavailable`(L0) 이라 **요약 자체가 없다.**
    ///
    /// 대상(`target`)은 여기 걸릴 수 없다 — [`Binding`] 이 `SymbolId` 를 요구하고
    /// `Unavailable` 에서는 그 값을 꺼낼 수 없다(F03 §3.3). 그러므로 이 사유는
    /// **반경이 `symbol` 보다 넓을 때** 처음 하중을 진다.
    IdentityGrade,
    /// 감시 집합 원소가 **partial 파일 안**에 있다(F02-2). 그 파일의 산출은 일부다.
    PartialParse,
    /// 감시 집합 원소가 사라졌다 — **`target` 은 살아 있다.**
    ///
    /// `target` 이 사라진 것은 [`CodeFreshness::Orphaned`] 이고 **다른 사건이다.**
    /// 저기는 *"결정을 다시 해야 한다"* 이고 여기는 *"지켜보던 것 하나를 못 본다"* 다.
    WatchMemberGone,
    /// 투영이 아직 그 `Snapshot` 을 안 봤다 — `built_for_this_snapshot == false`.
    ///
    /// **F06 이 이것을 관측 가능하게 만들었다**(`--read-only`). 그전에는 `pal query` 가
    /// 스티칭을 다시 돌려 이 상태 자체가 안 보였다.
    ProjectionStale,
}

impl UndeterminableReason {
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::IdentityGrade => "identity-grade",
            Self::PartialParse => "partial-parse",
            Self::WatchMemberGone => "watch-member-gone",
            Self::ProjectionStale => "projection-stale",
        }
    }

    /// 사유 넷 — **`[f09.2.pass]` 가 넷을 각각 만들어 `Live` 가 안 나오는지 센다.**
    pub const ALL: [Self; 4] =
        [Self::IdentityGrade, Self::PartialParse, Self::WatchMemberGone, Self::ProjectionStale];
}

/// 감시 집합 원소 하나의 **지금** — [`BindingStatus::evaluate`] 가 보는 전부.
///
/// # 왜 `Option<BodyDigest>` 가 아닌가
///
/// `None` 이 *"사라졌다"* 인지 *"비교할 수 없다"* 인지 구별되지 않는다. 그 구별이
/// 이 기능의 전부이고([R16]), 뭉개면 판정 불가가 조용히 `Orphaned` 나 `Live` 로 샌다.
///
/// [R16]: ../../../docs/evidence-map.md
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Now {
    /// 요약이 있다. **비교한다.**
    Digest(BodyDigest),
    /// 좌표가 사라졌다.
    Gone,
    /// **비교할 값이 없다** — 사유와 함께.
    Undeterminable(UndeterminableReason),
}

/// 코드가 변했는가 — **기계가 계산한다.**
///
/// F09 는 여섯 변형을 적었고 여기 **넷**이 있다. 없는 둘의 근거는
/// `corpus/criteria.toml` `[f09].freshness_boundary` 에 있다:
///
///   · **`Pending`** — 좌표가 아직 없는 결박은 **타입상 존재할 수 없다.**
///     [`Binding`] 이 `SymbolId` 를 요구하고 그것이 F03 §3.3 이 타입으로 세운 것이다.
///     `subject` 만 있고 `target` 이 없는 결박은 F10·F12 의 것이다
///   · **`StaleDerived`** — 결박의 **파생 입력을 주는 것이 이 빌드에 없다.**
///     그리고 [`crate::NodeFreshness`] 와 **합치지 않는다** — 모집단이 다르고
///     (그래프 노드 vs 결박), 합치면 `[f22.4]` 불변식 8 의 모집단이 움직인다.
///     ★ 문서 §2 가 물은 것에는 답한다: **입력 좌표가 사라진 경우는 `Orphaned` 다.**
///     가르는 기준이 *"기계가 닫을 수 있는가"* 이고([`crate::cascade`] 의 표),
///     **사라진 좌표는 갱신할 입력이 없어 기계가 못 닫는다**
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "freshness")]
pub enum CodeFreshness {
    /// 좌표가 있고 감시 집합 전체의 요약이 그대로다.
    Live,
    /// 감시 집합의 무언가가 변했다. **무엇이 켰는지 함께 싣는다** —
    /// *"본체가 변해서"* 와 *"호출자가 변해서"* 를 사람이 다르게 처리하기 때문이다.
    Stale { triggered_by: Vec<SymbolId> },
    /// 좌표가 사라졌다. **`Stale` 이 아니다** — 구현 제거는 코드 변경과 다른 사건이고
    /// 사람의 판단을 부른다(F09 §2). 뭉개면 *"고치면 되는 것"* 과 *"결정을 다시 해야
    /// 하는 것"* 이 같은 화면이 된다.
    ///
    /// **`target` 이 사라졌을 때만이다.** 감시 집합의 다른 원소가 사라진 것은
    /// [`UndeterminableReason::WatchMemberGone`] 이고 다른 사건이다.
    Orphaned { missing: Vec<SymbolId> },
    /// **판정할 수 없다.** `Live` 로 접지 않는다 — 그것이 [R16] 의 자리다.
    ///
    /// `at` 은 그 사유를 진 감시 원소들이다([`Stale::triggered_by`] 와 같은 형태) —
    /// *"어디를 못 보는가"* 가 실려야 사람이 무엇을 고칠지 안다.
    /// [`UndeterminableReason::ProjectionStale`] 은 결박이 아니라 투영의 사정이므로
    /// `at` 이 비어 있고, **그 빈 것이 정확한 값이다.**
    ///
    /// # 사유가 여럿이면 하나만 싣는다 — 그리고 그 순서가 결정론의 조건이다
    ///
    /// 문서 §2.1 이 `reason` 을 **하나**로 적었다. 여럿이 겹치면
    /// [`UndeterminableReason::ALL`] 의 순서에서 **첫째**를 싣는다. 회차마다 다른 사유가
    /// 나오면 *"밀도가 지도다"* 라는 요구(§2.1)가 성립하지 않는다.
    ///
    /// [R16]: ../../../docs/evidence-map.md
    Undeterminable { reason: UndeterminableReason, at: Vec<SymbolId> },
}

/// 대체됐는가 — **사람 또는 승인된 추론이 만든다.**
///
/// 코드 신선도와 **다른 축**이다. 한 열거에 넣으면 *"대체됐고 코드도 변했다"* 를
/// 표현할 수 없다(F09 §6 이 초안을 기각한 이유).
///
/// # 이 빌드에서 [`Self::Superseded`] 를 만드는 경로는 **픽스처뿐이다**
///
/// 그리고 그것이 [`CodeFreshness`] 의 `Pending`·`StaleDerived` 와 다른 처분인 근거가
/// 셋이다(`[f09].freshness_boundary` ⓒ):
///
///   1. **두 축의 독립성은 F09 의 설계 결정이다**(§2). 한 축이 값 하나뿐이면
///      *"두 축이다"* 라는 주장이 **산출로 서지 않는다**
///   2. **문서 §7 이 산출 방법을 이름으로 적었다** — *"`superseded ∧ stale` 조합이
///      실제로 산출되는가(**픽스처**)"*. 픽스처가 산출하면 그 변형은 **거주 가능**하고,
///      [ADR-0012] 가 금한 것은 **값을 만들 수 없는 이름**이다
///   3. **값을 만드는 자리가 하나뿐이고 명시적이다** — [`BindingStatus::evaluate`] 가
///      이 값을 **인자로 받는다.** 결박에 **저장하지 않는다**(§4.4 가 대체를
///      *"이벤트를 덧붙이는 형태"* 라고 적었고 그 이벤트는 F12 다)
///
/// **인자로 두었기 때문에 F12 가 승인 흐름을 세울 때 이 열거를 안 건드린다.**
///
/// [ADR-0012]: ../../../docs/adr/0012-a-single-truth-file-declares-only-what-has-a-counterpart-in-code.md
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Lineage {
    Current,
    /// 다른 개체가 이것을 대체했다. **코드 신선도는 계속 계산된다** — 그것이 축이 둘인
    /// 이유이고, 한 열거에 넣으면 그 계산이 사라진다.
    Superseded { by: crate::EntityId },
}

/// 결박의 상태 — **두 축이다.**
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BindingStatus {
    pub code: CodeFreshness,
    pub lineage: Lineage,
}

impl BindingStatus {
    /// 감시 집합을 현재 값과 대조한다.
    ///
    /// `now` 는 *"그 심볼의 지금"* 을 주는 조회다([`Now`]).
    /// `lineage` 는 **인자다** — 결박에 저장하지 않는다([`Lineage`] 머리).
    ///
    /// # 판정의 순서 — **이 순서가 이 함수의 전부다**
    ///
    /// ```text
    /// ① target 이 사라졌나        → Orphaned      (결정적이다. 더 볼 것이 없다)
    /// ② 못 보는 원소가 있나        → Undeterminable (**Live 로 접지 않는다** · R16)
    /// ③ 변한 원소가 있나          → Stale
    /// ④ 아니면                   → Live
    /// ```
    ///
    /// **②가 ③보다 먼저인 것이 요구다.** 뒤로 보내면 *"하나는 못 보지만 나머지가
    /// 안 변했으니 Live"* 가 되고, 그것이 선행 구현이 `stale=False` 로 접었던 자리다.
    ///
    /// **①이 ②보다 먼저인 것도 요구다.** 좌표가 사라졌으면 *"판정할 수 없다"* 가 아니라
    /// *"결정을 다시 해야 한다"* 이고, 둘은 사람이 다르게 처리한다.
    ///
    /// # 이 계산은 [`Binding::bound_at_time`] 을 **안 읽는다**
    ///
    /// 커밋 시각을 앵커로 쓰면 포매팅 커밋에도 `stale` 이 켜진다(F09 §6 · R-07).
    /// 계산 경로와 표시 경로가 갈려 있어야 하고, `cargo xtask check` 가 그 갈림을 센다.
    #[must_use]
    pub fn evaluate(binding: &Binding, lineage: Lineage, now: impl Fn(SymbolId) -> Now) -> Self {
        let mut changed = Vec::new();
        let mut gone = Vec::new();
        // 사유별로 모은다 — 여럿이면 `ALL` 의 순서에서 첫째를 싣는다.
        let mut 못_봄: Vec<(UndeterminableReason, SymbolId)> = Vec::new();

        for w in &binding.watch {
            match now(w.symbol) {
                Now::Digest(d) if d != w.digest => changed.push(w.symbol),
                Now::Digest(_) => {}
                Now::Gone => gone.push(w.symbol),
                Now::Undeterminable(r) => 못_봄.push((r, w.symbol)),
            }
        }

        // ① **target 이 사라진 것이 먼저다.** 사라진 심볼을 "변했다"로 적으면 사람이
        //    코드를 고치러 가는데 고칠 코드가 없다.
        if gone.contains(&binding.target) {
            return Self { code: CodeFreshness::Orphaned { missing: gone }, lineage };
        }
        // 대상이 아닌 원소가 사라진 것은 **다른 사건**이다 — 지켜보던 것 하나를 못 본다.
        못_봄.extend(gone.into_iter().map(|s| (UndeterminableReason::WatchMemberGone, s)));

        // ② **못 보는 것이 있으면 `Live` 가 될 수 없다** — R16 의 자리.
        if let Some(reason) = UndeterminableReason::ALL
            .into_iter()
            .find(|r| 못_봄.iter().any(|(had, _)| had == r))
        {
            let at = 못_봄.iter().filter(|(r, _)| *r == reason).map(|(_, s)| *s).collect();
            return Self { code: CodeFreshness::Undeterminable { reason, at }, lineage };
        }

        // ③④
        let code = if changed.is_empty() {
            CodeFreshness::Live
        } else {
            CodeFreshness::Stale { triggered_by: changed }
        };
        Self { code, lineage }
    }

    /// 투영이 이 스냅샷을 안 봤다 — **감시 집합을 보기도 전에 판정 불가다.**
    ///
    /// [`UndeterminableReason::ProjectionStale`] 은 결박이 아니라 **투영의 사정**이므로
    /// 원소별로 나오지 않는다. 그래서 자리가 따로 있고, `at` 이 비어 있는 것이
    /// **정확한 값**이다.
    ///
    /// 부르는 쪽이 `built_for_this_snapshot` 을 먼저 보게 하는 것이 이 함수의 요점이다 —
    /// [`Self::evaluate`] 안에서 처리하면 그 조건이 조회 클로저 뒤에 숨는다.
    #[must_use]
    pub fn projection_stale(lineage: Lineage) -> Self {
        Self {
            code: CodeFreshness::Undeterminable {
                reason: UndeterminableReason::ProjectionStale,
                at: Vec::new(),
            },
            lineage,
        }
    }

    /// 이 상태가 **판정 입력 자격**을 갖는가 — [`crate::NodeFreshness::admissible`] 과
    /// 같은 자리이고 같은 규율이다.
    ///
    /// `Live ∧ Current` 만 갖는다. **`Undeterminable` 은 판정 입력에서
    /// `Residual{사유=결박 판정 불가}` 가 된다**(F09 §2.1) — 안 그러면 *"화면에는 뜨는데
    /// 판정은 그것을 유효로 센다"* 가 된다.
    #[must_use]
    pub const fn admissible(&self) -> bool {
        matches!(self.code, CodeFreshness::Live) && matches!(self.lineage, Lineage::Current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coord::Discriminator;
    use crate::repo::{ObjectName, RepoId, RepoPath, TreeRef};
    use crate::symbol::SymbolKind;

    fn 심볼(n: &str) -> SymbolId {
        SymbolId::compute(
            &RepoId::new("r"),
            &RepoPath::new("a.kt"),
            &[],
            n,
            &Discriminator::new(SymbolKind::Function, 0),
        )
    }

    fn 결박(target: SymbolId, digest: BodyDigest) -> Binding {
        결박_반경(target, &[(target, digest)], crate::Radius::Symbol)
    }

    fn 결박_반경(target: SymbolId, watch: &[(SymbolId, BodyDigest)], r: crate::Radius) -> Binding {
        Binding::new(
            crate::EntityId::mint(crate::EntityKind::new("decision"), crate::EntityOrigin::Hand),
            target,
            "메모",
            Snapshot::single(RepoId::new("r"), TreeRef::Committed(ObjectName::from_bytes([0; 20]))),
            BoundTime::Committed { epoch_secs: 1_700_000_000 },
            r,
            watch.iter().map(|(s, d)| WatchEntry { symbol: *s, digest: *d }).collect(),
        )
    }

    fn 상태(b: &Binding, now: impl Fn(SymbolId) -> Now) -> CodeFreshness {
        BindingStatus::evaluate(b, Lineage::Current, now).code
    }

    #[test]
    fn 안_변하면_살아_있다() {
        let s = 심볼("f");
        let d = BodyDigest::of_normalized(b"x");
        assert_eq!(상태(&결박(s, d), |_| Now::Digest(d)), CodeFreshness::Live);
    }

    #[test]
    fn 변하면_낡고_무엇이_켰는지_실린다() {
        let s = 심볼("f");
        let d = BodyDigest::of_normalized(b"x");
        let CodeFreshness::Stale { triggered_by } =
            상태(&결박(s, d), |_| Now::Digest(BodyDigest::of_normalized(b"y")))
        else {
            panic!("stale 이 아니다");
        };
        assert_eq!(triggered_by, vec![s]);
    }

    #[test]
    fn 사라진_것은_변한_것이_아니다() {
        // **★ 반대 방향 ④ — `Orphaned` ≠ `Stale`.** 뭉개면 *"고치면 되는 것"* 과
        // *"결정을 다시 해야 하는 것"* 이 같은 화면이 된다.
        let s = 심볼("f");
        let d = BodyDigest::of_normalized(b"x");
        assert_eq!(상태(&결박(s, d), |_| Now::Gone), CodeFreshness::Orphaned { missing: vec![s] });
    }

    #[test]
    fn 판정_불가는_live_로_새지_않는다() {
        // **★ 반대 방향 ③ — R16 의 자리다.** 선행 구현이 `stale=False` 로 접었던 그것이고,
        // **사유 넷을 각각** 센다 — 하나만 시험하면 나머지 셋이 접혀도 통과한다.
        let s = 심볼("f");
        let d = BodyDigest::of_normalized(b"x");
        for r in UndeterminableReason::ALL {
            if r == UndeterminableReason::ProjectionStale {
                // 투영의 사정이라 원소별로 안 나온다 — 자리가 따로 있다.
                let st = BindingStatus::projection_stale(Lineage::Current);
                assert_eq!(
                    st.code,
                    CodeFreshness::Undeterminable { reason: r, at: Vec::new() },
                    "{}", r.name()
                );
                assert!(!st.admissible(), "{} 가 판정 입력 자격을 가졌다", r.name());
                continue;
            }
            let code = 상태(&결박(s, d), |_| Now::Undeterminable(r));
            assert_eq!(
                code,
                CodeFreshness::Undeterminable { reason: r, at: vec![s] },
                "{} 가 판정 불가로 안 나온다", r.name()
            );
            assert_ne!(code, CodeFreshness::Live, "{} 가 Live 로 샜다", r.name());
        }
    }

    #[test]
    fn 못_보는_것이_있으면_나머지가_그대로여도_live_가_아니다() {
        // **②가 ③보다 먼저인 것이 요구다.** 뒤로 보내면 *"하나는 못 보지만 나머지가
        // 안 변했으니 Live"* 가 되고, 그것이 접는 자리다.
        let a = 심볼("a");
        let b = 심볼("b");
        let d = BodyDigest::of_normalized(b"x");
        let 결박 = 결박_반경(a, &[(a, d), (b, d)], crate::Radius::Callers);
        let code = 상태(&결박, |s| {
            if s == b { Now::Undeterminable(UndeterminableReason::PartialParse) } else { Now::Digest(d) }
        });
        assert_eq!(
            code,
            CodeFreshness::Undeterminable { reason: UndeterminableReason::PartialParse, at: vec![b] }
        );
    }

    #[test]
    fn 감시_원소가_사라진_것과_대상이_사라진_것은_다른_사건이다() {
        // **★ `WatchMemberGone` ≠ `Orphaned`.** 문서 §2.1 이 *"target 은 살아 있다"* 를
        // 사유의 정의에 적었다.
        let a = 심볼("a");
        let b = 심볼("b");
        let d = BodyDigest::of_normalized(b"x");
        let 결박 = 결박_반경(a, &[(a, d), (b, d)], crate::Radius::Callers);

        // 대상이 사라지면 Orphaned — 다른 원소가 어떻든.
        assert!(matches!(
            상태(&결박, |s| if s == a { Now::Gone } else { Now::Digest(d) }),
            CodeFreshness::Orphaned { .. }
        ));
        // 대상이 아닌 원소가 사라지면 판정 불가.
        assert_eq!(
            상태(&결박, |s| if s == b { Now::Gone } else { Now::Digest(d) }),
            CodeFreshness::Undeterminable {
                reason: UndeterminableReason::WatchMemberGone,
                at: vec![b],
            }
        );
    }

    #[test]
    fn 사유가_여럿이면_정해진_순서의_첫째다() {
        // **회차마다 다른 사유가 나오면 「밀도가 지도다」가 성립하지 않는다**(§2.1).
        let a = 심볼("a");
        let b = 심볼("b");
        let d = BodyDigest::of_normalized(b"x");
        let 결박 = 결박_반경(a, &[(a, d), (b, d)], crate::Radius::Callers);
        let code = 상태(&결박, |s| {
            if s == a {
                Now::Digest(d)
            } else if s == b {
                Now::Undeterminable(UndeterminableReason::PartialParse)
            } else {
                Now::Undeterminable(UndeterminableReason::IdentityGrade)
            }
        });
        // 대상은 값이 있으므로 못 보는 것은 `b` 하나다.
        assert_eq!(
            code,
            CodeFreshness::Undeterminable { reason: UndeterminableReason::PartialParse, at: vec![b] }
        );
        // 그리고 `ALL` 의 순서가 곧 우선순위다.
        assert!(UndeterminableReason::IdentityGrade < UndeterminableReason::PartialParse);
    }

    #[test]
    fn 두_축이_독립이다() {
        // **★ 문서 §7 이 픽스처로 적은 그것이다.** 네 조합을 전부 만든다 —
        // 하나라도 표현되지 않으면 두 축이 아니라 한 축이다.
        let s = 심볼("f");
        let d = BodyDigest::of_normalized(b"x");
        let 대체자 = crate::EntityId::mint(crate::EntityKind::new("decision"), crate::EntityOrigin::Hand);
        let b = 결박(s, d);

        let 조합 = |lineage: Lineage, 변했나: bool| {
            BindingStatus::evaluate(&b, lineage, |_| {
                Now::Digest(if 변했나 { BodyDigest::of_normalized(b"y") } else { d })
            })
        };

        let current_live = 조합(Lineage::Current, false);
        let current_stale = 조합(Lineage::Current, true);
        let sup = Lineage::Superseded { by: 대체자 };
        let superseded_live = 조합(sup.clone(), false);
        let superseded_stale = 조합(sup, true);

        assert_eq!(current_live.code, CodeFreshness::Live);
        assert!(matches!(current_stale.code, CodeFreshness::Stale { .. }));
        // **대체된 뒤에도 코드 신선도가 계속 계산된다** — 축이 둘인 이유가 그것이다.
        assert_eq!(superseded_live.code, CodeFreshness::Live);
        assert!(matches!(superseded_stale.code, CodeFreshness::Stale { .. }),
                "대체되자 코드 신선도가 굳었다 — 한 열거로 접힌 것과 같다");
        assert!(matches!(superseded_live.lineage, Lineage::Superseded { .. }));

        // 판정 입력 자격은 `Live ∧ Current` 뿐이다.
        assert!(current_live.admissible());
        assert!(!current_stale.admissible());
        assert!(!superseded_live.admissible(), "대체된 결정이 유효로 보증됐다 — 낡음보다 나쁜 거짓 신호다");
    }

    #[test]
    fn 같은_것을_두_번_걸면_같은_결박이다() {
        let s = 심볼("f");
        assert_eq!(BindingId::derive(s, "메모"), BindingId::derive(s, "메모"));
        assert_ne!(BindingId::derive(s, "메모"), BindingId::derive(s, "다른 메모"));
    }

    #[test]
    fn 시각은_판정에_안_들어간다() {
        // **★ 반대 방향** — 시각만 바뀌고 요약이 그대로면 `Live` 여야 한다.
        // 켜지면 R-07 이 치명이라 부른 실패(포매팅 커밋에 stale)를 그대로 맞는다.
        let s = 심볼("f");
        let d = BodyDigest::of_normalized(b"x");
        let mut 늦은 = 결박(s, d);
        늦은.bound_at_time = BoundTime::Committed { epoch_secs: 1 };
        let mut 이른 = 결박(s, d);
        이른.bound_at_time = BoundTime::Worktree;
        assert_eq!(상태(&늦은, |_| Now::Digest(d)), 상태(&이른, |_| Now::Digest(d)));
        assert_eq!(상태(&늦은, |_| Now::Digest(d)), CodeFreshness::Live);
    }
}
