//! 질의 로그 — **F05 부터 켠다.**
//!
//! F05 §5.3: *"모든 질의 실행이 접근한 좌표를 `QUERY_LOG` 에 남긴다. F17(커버리지 계산)이
//! 나중에 새 장치를 만들지 않고 **F05 부터 쌓인 로그를 읽게** 하기 위해서다.
//! **처음부터 켜지 않으면 그 기능은 데이터가 없어 착수할 수 없다.**"*
//!
//! # 이것은 파생이 아니다 — 그런데 2층에 산다
//!
//! 재구축으로 다시 만들어지지 않는다. 질의는 일어난 사건이고 1층에도 git 에도 없다.
//! 그러므로 **재구축이 이 자리를 건드리면 안 된다** — `[f05.3.pass]` ② 가 그것을 잰다
//! (스티칭은 교체할 자리의 목록에 이것을 넣지 않는다).
//!
//! 그런데도 의도 저장소가 아니라 2층에 두는 이유: **사람이 지불한 노동이 아니다.**
//! 유실되면 커버리지 계산의 관측 창이 짧아질 뿐 결박이 사라지지 않는다. R-21 이 가르는
//! 선은 *"재생 가능한가"* 가 아니라 *"사람의 노동인가"* 다(§3.1 의 표).

use serde::{Deserialize, Serialize};

use crate::coord::SymbolId;
use crate::envelope::Elision;

/// 질의 하나의 이름 — **열린 문자열이 아니다.**
///
/// 문자열이면 오타가 새 질의 이름이 되고, F17 이 로그를 셀 때 그 오타가 하나의 질의로
/// 잡힌다. **이름이 값이면 세어진다.**
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueryName {
    /// 이 스냅샷의 관측 범위 대장.
    LedgerSnapshot,
    /// 이름 하나 → 후보 심볼들.
    SymbolResolve,
    /// 이 심볼이 담는 것들.
    SymbolContains,
    /// 이 심볼을 가리키는 것들 — 1홉 역방향.
    SymbolCallers,
    /// 이 심볼에서 닿는 것들 — 예산 절단이 있는 BFS.
    SymbolReaches,
    /// 노드와 엣지 전부 — **바깥 오라클(SQLite CTE)이 읽는 창**이다.
    GraphDump,
}

impl QueryName {
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::LedgerSnapshot => "ledger.snapshot",
            Self::SymbolResolve => "symbol.resolve",
            Self::SymbolContains => "symbol.contains",
            Self::SymbolCallers => "symbol.callers",
            Self::SymbolReaches => "symbol.reaches",
            Self::GraphDump => "graph.dump",
        }
    }

    /// 이 빌드가 답하는 질의 전부. **표면이 이것을 그대로 낸다.**
    pub const ALL: [Self; 6] = [
        Self::LedgerSnapshot,
        Self::SymbolResolve,
        Self::SymbolContains,
        Self::SymbolCallers,
        Self::SymbolReaches,
        Self::GraphDump,
    ];

    #[must_use]
    pub fn parse(raw: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|q| q.name() == raw)
    }
}

/// 질의 하나가 남기는 줄 — F05 §5.3 의 일곱 성분.
///
/// `snapshot` 과 `seq` 는 **열쇠라서 값에 없다.** 저장이 그 둘로 키를 만든다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueryLogEntry {
    pub query: QueryName,
    /// 인자의 요약. **인자 자체를 안 담는다** — 이름이 코드일 수 있고, 로그는 산출이다.
    pub args_digest: String,
    /// 이 답이 **실제로 만진** 좌표. F17 의 커버리지가 이것을 센다.
    pub accessed: Vec<SymbolId>,
    /// 잘린 것. **절단이 없어도 실린다** — 로그에서도 조용한 절단은 금지다.
    pub elision: Elision,
    /// 걸린 시간(마이크로초). **벤치가 아니라 관측이다.**
    pub duration_micros: u64,
}

impl QueryLogEntry {
    /// 인자 문자열의 요약. 같은 인자는 같은 값이고, 인자 자체는 안 남는다.
    #[must_use]
    pub fn digest_of(args: &str) -> String {
        let mut h = blake3::Hasher::new();
        h.update(b"pal-query-args-v1\0");
        h.update(args.as_bytes());
        h.finalize().to_hex()[..16].to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 질의_이름_여섯이_서로_다르다() {
        // 뭉개지면 F17 이 로그를 셀 때 두 질의가 하나로 잡힌다.
        let names: std::collections::BTreeSet<&str> =
            QueryName::ALL.into_iter().map(QueryName::name).collect();
        assert_eq!(names.len(), QueryName::ALL.len());
    }

    #[test]
    fn 이름은_되읽힌다() {
        for q in QueryName::ALL {
            assert_eq!(QueryName::parse(q.name()), Some(q));
        }
        assert_eq!(QueryName::parse("symbol.resolvee"), None, "오타가 질의가 됐다");
    }

    #[test]
    fn 같은_인자는_같은_요약이고_다른_인자는_다르다() {
        assert_eq!(QueryLogEntry::digest_of("helper"), QueryLogEntry::digest_of("helper"));
        assert_ne!(QueryLogEntry::digest_of("helper"), QueryLogEntry::digest_of("helpe"));
    }
}
