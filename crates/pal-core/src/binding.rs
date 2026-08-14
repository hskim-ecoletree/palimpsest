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

/// 코드가 변했는가 — **기계가 계산한다.**
///
/// F09 는 여섯 변형을 적었고 여기 셋이 있다. `StaleDerived`(F22) · `Pending` ·
/// `Undeterminable` 은 **그것을 만들 수 있는 기능이 아직 없어서 두지 않았다** —
/// 만들 수 없는 변형을 미리 두면 그것이 곧 "있는데 안 나오는" 상태가 된다.
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
    Orphaned { missing: Vec<SymbolId> },
}

/// 대체됐는가 — **사람 또는 승인된 추론이 만든다.**
///
/// 코드 신선도와 **다른 축**이다. 한 열거에 넣으면 *"대체됐고 코드도 변했다"* 를
/// 표현할 수 없다. S3 는 `Current` 만 만든다 — 대체 흐름은 F12 다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Lineage {
    Current,
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
    /// `current` 는 *"그 심볼의 지금 요약"* 을 주는 조회다. `None` 은 **심볼이 사라졌다**는
    /// 뜻이고 그것이 `Orphaned` 를 만든다.
    #[must_use]
    pub fn evaluate(
        binding: &Binding,
        current: impl Fn(SymbolId) -> Option<BodyDigest>,
    ) -> Self {
        let mut missing = Vec::new();
        let mut changed = Vec::new();
        for w in &binding.watch {
            match current(w.symbol) {
                None => missing.push(w.symbol),
                Some(now) if now != w.digest => changed.push(w.symbol),
                Some(_) => {}
            }
        }

        // **사라진 것이 먼저다.** 사라진 심볼을 "변했다"로 적으면 사람이 코드를 고치러
        // 가는데 고칠 코드가 없다.
        let code = if missing.is_empty() {
            if changed.is_empty() {
                CodeFreshness::Live
            } else {
                CodeFreshness::Stale { triggered_by: changed }
            }
        } else {
            CodeFreshness::Orphaned { missing }
        };
        Self { code, lineage: Lineage::Current }
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
        Binding::new(
            crate::EntityId::mint(crate::EntityKind::new("decision"), crate::EntityOrigin::Hand),
            target,
            "메모",
            Snapshot::single(RepoId::new("r"), TreeRef::Committed(ObjectName::from_bytes([0; 20]))),
            BoundTime::Committed { epoch_secs: 1_700_000_000 },
            crate::Radius::Symbol,
            vec![WatchEntry { symbol: target, digest }],
        )
    }

    #[test]
    fn 안_변하면_살아_있다() {
        let s = 심볼("f");
        let d = BodyDigest::of_normalized(b"x");
        let st = BindingStatus::evaluate(&결박(s, d), |_| Some(d));
        assert_eq!(st.code, CodeFreshness::Live);
    }

    #[test]
    fn 변하면_낡고_무엇이_켰는지_실린다() {
        let s = 심볼("f");
        let d = BodyDigest::of_normalized(b"x");
        let st = BindingStatus::evaluate(&결박(s, d), |_| {
            Some(BodyDigest::of_normalized(b"y"))
        });
        let CodeFreshness::Stale { triggered_by } = st.code else {
            panic!("stale 이 아니다");
        };
        assert_eq!(triggered_by, vec![s]);
    }

    #[test]
    fn 사라진_것은_변한_것이_아니다() {
        // **Orphaned ≠ Stale.** 사라진 심볼을 "변했다"로 적으면 사람이 코드를 고치러
        // 가는데 고칠 코드가 없다.
        let s = 심볼("f");
        let d = BodyDigest::of_normalized(b"x");
        let st = BindingStatus::evaluate(&결박(s, d), |_| None);
        assert_eq!(st.code, CodeFreshness::Orphaned { missing: vec![s] });
    }

    #[test]
    fn 같은_것을_두_번_걸면_같은_결박이다() {
        let s = 심볼("f");
        assert_eq!(BindingId::derive(s, "메모"), BindingId::derive(s, "메모"));
        assert_ne!(BindingId::derive(s, "메모"), BindingId::derive(s, "다른 메모"));
    }
}
