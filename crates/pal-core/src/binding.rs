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

/// 사람이 넣은 것 하나 — **의도 저장소가 소유한다.**
///
/// # 이 타입이 파생층에 살면 안 되는 이유 ([R-21](../../../docs/plan/00-risks.md#r-21))
///
/// 2층에 의도가 살면 *"지우고 재구축"* 이 **사람의 노동을 지우는 명령**이 되고,
/// 재구축 등가성 검사는 그 상태에서도 통과하므로 **검사가 유실을 정상으로 승인한다.**
/// 그래서 결박의 실체는 `pal-intent` 에 있고 파생층에는 색인만 둔다.
///
/// **[graph-node] `Binding`** — `schema/graph.toml`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Binding {
    pub id: BindingId,
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
    /// 무엇을 지켜보나. **S3 에서는 대상 심볼 하나뿐이다** — 반경은 F09.
    pub watch: Vec<WatchEntry>,
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
        Binding {
            id: BindingId::derive(target, "메모"),
            target,
            note: "메모".into(),
            bound_at: Snapshot::single(
                RepoId::new("r"),
                TreeRef::Committed(ObjectName::from_bytes([0; 20])),
            ),
            watch: vec![WatchEntry { symbol: target, digest }],
        }
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
