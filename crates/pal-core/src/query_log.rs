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
/// # 직렬화된 이름이 **표면 이름과 같다** (F06 · `[f06.1]`)
///
/// 옛 판은 `rename_all = "snake_case"` 라 JSON 에 `"ledger_snapshot"` 이 나갔는데
/// 표면이 받는 이름은 `ledger.snapshot` 이었다. **소비자가 산출에서 읽은 이름으로
/// 이 도구를 부를 수 없었다** — F06 이 `Folded::unfolded_by` 를 세우면서 드러났다
/// (*"어느 질의가 이것을 펴는가"* 가 부를 수 없는 이름이면 아무 말도 안 한 것이다).
///
/// 그래서 변형마다 **표면 이름을 명시한다.** 아래 시험이 둘을 묶는다.
///
/// **질의 로그의 저장 형식은 안 바뀐다** — `postcard` 는 변형의 **번호**로 쓰고
/// 이름을 안 쓴다. 이름은 JSON 처럼 자기서술 형식에서만 나간다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum QueryName {
    /// 이 스냅샷의 관측 범위 대장.
    #[serde(rename = "ledger.snapshot")]
    LedgerSnapshot,
    /// 이름 하나 → 후보 심볼들.
    #[serde(rename = "symbol.resolve")]
    SymbolResolve,
    /// 이 심볼이 담는 것들.
    #[serde(rename = "symbol.contains")]
    SymbolContains,
    /// 이 심볼을 가리키는 것들 — 1홉 역방향.
    #[serde(rename = "symbol.callers")]
    SymbolCallers,
    /// 이 심볼에서 닿는 것들 — 예산 절단이 있는 BFS.
    #[serde(rename = "symbol.reaches")]
    SymbolReaches,
    /// 노드와 엣지 전부 — **바깥 오라클(SQLite CTE)이 읽는 창**이다.
    #[serde(rename = "graph.dump")]
    GraphDump,
    /// 결박마다 **상태 + 반경 + 무엇이 켰는가** (F09 §8).
    ///
    /// # 왜 결박에서 출발하는 표면이 따로 필요한가
    ///
    /// `pal touch <이름>` 은 **이름으로 먼저 찾는다.** 그래서 좌표가 사라진 결박은
    /// `unknown` 이 되어 **결박에 닿지 못하고**, `Orphaned` 가 화면에 안 뜬다 —
    /// 그것이 이 기능이 가장 보여야 하는 상태 하나다.
    #[serde(rename = "binding.status")]
    BindingStatus,
    /// 좌표를 못 찾은 문서 조각들 — **이것이 사람의 작업 목록이다** (F10 §2).
    ///
    /// # 왜 「미결박」이고 「후보 있음」이 아닌가
    ///
    /// 둘은 다른 답이다. *"여럿이라 못 좁혔다"* 는 **이미 후보가 있는 것**이고,
    /// *"신호가 없다"* 는 **사람이 좌표를 붙여야 하는 것**이다. 뭉치면 작업 목록에
    /// 할 일이 아닌 것이 섞이고, 섞이면 목록이 안 읽힌다.
    #[serde(rename = "narrative.unbound")]
    NarrativeUnbound,
    /// ★ **좌표 하나를 만진다** — 걸린 것 · 지켜보는 것 · 이 심볼이 하는 것 · 못 만든 자리
    /// (F11).
    ///
    /// # 왜 `pal touch` 가 이 목록에 이제야 오는가
    ///
    /// S2 부터 서 있었는데 **카탈로그에 짝이 없었다.** 능력 목록(`capabilities.built`)에는
    /// `binding.touch` 가 실려 있고 `surface/queries.toml` 에는 줄이 없었다 —
    /// [ADR-0012] 가 금한 *"짝 없는 이름"* 의 **반대편**이고, 그 자리에서는 양방향
    /// 대조가 꺼진 채로 돌았다.
    ///
    /// **그리고 이름이 여기 서야 질의 로그가 켜진다.** 옛 판의 `pal touch` 는
    /// [`crate::LogStatus::NotRecorded`] 를 `SurfaceDoesNotLog` 로 냈고, 그러면 F17 이
    /// 미조회를 **과대 계상**한다.
    ///
    /// [ADR-0012]: ../../../docs/adr/0012-a-single-truth-file-declares-only-what-has-a-counterpart-in-code.md
    #[serde(rename = "binding.touch")]
    BindingTouch,
    /// ★ **계획과 실제의 갈림** — 넷이고 `unmeasurable` 이 분리돼 있다 (F12).
    ///
    /// # 인자가 **좌표가 아니라 계획 문서**다
    ///
    /// 이름을 받는 다른 다섯은 전부 `SymbolName` 을 받는데 이것은 `RepoPath` 를 받는다.
    /// **기준선이 그 문서 안에 있기 때문이다**([`crate::PlanBaseline`]) — [F12 §4] 가
    /// *"계획 승인 시점의 Snapshot 을 계획에 기록"* 이라고 적었고, `--base <ref>` 는
    /// [F23] 의 손잡이다.
    #[serde(rename = "plan.deviation")]
    PlanDeviation,
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
            Self::BindingStatus => "binding.status",
            Self::NarrativeUnbound => "narrative.unbound",
            Self::BindingTouch => "binding.touch",
            Self::PlanDeviation => "plan.deviation",
        }
    }

    /// 이 빌드가 답하는 질의 전부. **표면이 이것을 그대로 낸다.**
    pub const ALL: [Self; 10] = [
        Self::LedgerSnapshot,
        Self::SymbolResolve,
        Self::SymbolContains,
        Self::SymbolCallers,
        Self::SymbolReaches,
        Self::GraphDump,
        Self::BindingStatus,
        Self::NarrativeUnbound,
        Self::BindingTouch,
        Self::PlanDeviation,
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
    fn 직렬화된_이름이_표면_이름과_같다() {
        // **두 이름이 있으면 산출에서 읽은 이름으로 이 도구를 못 부른다.**
        // F06 이 `Folded::unfolded_by` 를 세우면서 드러난 자리다 — *"어느 질의가
        // 이것을 편다"* 가 부를 수 없는 이름이면 아무 말도 안 한 것이다.
        for q in QueryName::ALL {
            let json = serde_json::to_value(q).expect("직렬화");
            assert_eq!(json.as_str(), Some(q.name()), "{q:?} 의 두 이름이 갈렸다");
            // 되읽히기도 해야 한다 — F17 이 로그를 읽는다.
            let back: QueryName = serde_json::from_value(json).expect("역직렬화");
            assert_eq!(back, q);
        }
    }

    #[test]
    fn 같은_인자는_같은_요약이고_다른_인자는_다르다() {
        assert_eq!(QueryLogEntry::digest_of("helper"), QueryLogEntry::digest_of("helper"));
        assert_ne!(QueryLogEntry::digest_of("helper"), QueryLogEntry::digest_of("helpe"));
    }
}
