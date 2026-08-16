//! 훅이 무엇을 보고 무엇을 막는가 — **갈아끼우는 자리.**
//!
//! # 이 회차의 정책은 최소다. 그리고 그것이 의도다
//!
//! `[f24]` ⑧ 은 *"등록 형태를 합격선에 안 박는다 — 지금 박으면 측정이 답을 정하는 것이
//! 아니라 등록이 답을 정한다"* 고 적었다. 정책도 같은 자리에 있다. **배관이 먼저 서고
//! 정책은 나중에 소유자가 정한다.** 그래서 이 파일은 판정 규칙 하나만 지고, 나머지
//! 전부(규약 · 종료 코드 · 등록 · 진단)는 이 파일 밖에 있다.
//!
//! # 지금 막는 것 하나 — **돌려준 말이 비어 있는 서브에이전트**
//!
//! 부모는 서브에이전트의 **마지막 말**로만 결과를 받는다(도구 산출은 부모 문맥에 안
//! 남는다). 그 말이 비면 부모는 아무것도 못 받고, 그런데도 하네스는 **아무 말도 안
//! 한다.** 이것이 지금 아는 것 중 *"조용히 아무것도 안 돌려준다"* 에 가장 가깝고,
//! 정상 회차에서는 거의 안 걸린다.
//!
//! # ★ 모르는 형태에서는 차단하지 않는다
//!
//! 필드가 **아예 없으면** 통과다. 하네스가 페이로드를 바꾸면 「없음」이 「비어 있음」으로
//! 읽히고, 그러면 이 훅이 **모든 서브에이전트를 막는다.** 있는데 빈 것과 아예 없는 것을
//! 가르는 줄이 그 사고를 막는다.
//!
//! # ★ 그리고 반복 회차에서는 절대 차단하지 않는다
//!
//! 실측: `stop_hook_active` 는 1회차 `false`, 2회차부터 `true` 로 들어온다. 그리고
//! **매 라운드 도구를 부르는 서브에이전트는 100회까지 한 번도 안 멈췄다**(측정자가
//! SIGTERM 으로 끊었다). **하네스가 반복 차단을 못 멈춘다** — 훅이 스스로 멈춰야 하고
//! 이건 선택이 아니다.

use serde_json::Value;

/// 우리가 판정하는 사건. 여기 없는 것은 **전부 통과**다.
///
/// 등록도 이 목록으로 한다 — 판정하는 자리와 등록하는 자리가 갈리면 **등록만 남고
/// 판정이 사라진 훅**이 조용히 돈다.
pub const EVENTS: &[&str] = &["SubagentStop"];

/// 훅 하나의 판정.
pub enum Decision {
    /// 통과 — **아무것도 안 낸다.** 까닭은 진단으로만 나간다.
    Pass(&'static str),
    /// 차단 — 까닭이 모델에 닿는다.
    Block(String),
}

/// 차단할 때 모델에 닿는 말. **기본 문구(`Blocked by hook`)로 떨어지지 않게** 우리가 적는다.
const 빈_말: &str =
    "이 서브에이전트가 부모에게 돌려준 마지막 말이 비어 있다. 부모는 그 말로만 결과를 \
     받는다 — 무엇을 했고 무엇을 못 했고 다음에 무엇이 남았는지 한 문단으로 적고 끝내라.";

/// 무엇을 할 것인가.
#[must_use]
pub fn decide(event: &str, payload: &Value) -> Decision {
    if !EVENTS.contains(&event) {
        return Decision::Pass("우리가 판정하는 사건이 아니다");
    }
    // ★ **반복 회차에서는 절대 차단하지 않는다.** 다른 어떤 규칙보다 먼저 선다.
    if payload.get("stop_hook_active").and_then(Value::as_bool) == Some(true) {
        return Decision::Pass("반복 회차다 — 하네스가 이것을 못 멈추므로 훅이 스스로 멈춘다");
    }
    match payload.get("last_assistant_message") {
        None => Decision::Pass("돌려준 말이 페이로드에 없다 — 모르는 형태에서 차단하지 않는다"),
        Some(Value::String(s)) if s.trim().is_empty() => Decision::Block(빈_말.to_owned()),
        Some(Value::String(_)) => Decision::Pass("돌려준 말이 있다"),
        Some(_) => Decision::Pass("돌려준 말이 문자열이 아니다 — 모르는 형태에서 차단하지 않는다"),
    }
}

#[cfg(test)]
mod tests {
    use super::{Decision, decide};
    use serde_json::json;

    fn 차단인가(d: &Decision) -> bool {
        matches!(d, Decision::Block(_))
    }

    #[test]
    fn 돌려준_말이_비면_차단한다() {
        for 말 in ["", "   ", "\n\t "] {
            let v = json!({"stop_hook_active": false, "last_assistant_message": 말});
            assert!(차단인가(&decide("SubagentStop", &v)), "`{말:?}` 가 안 걸렸다");
        }
    }

    /// ★ **반복 회차는 무슨 일이 있어도 통과다.**
    #[test]
    fn 반복_회차는_같은_페이로드에서도_통과한다() {
        let v = json!({"stop_hook_active": true, "last_assistant_message": ""});
        assert!(!차단인가(&decide("SubagentStop", &v)));
    }

    /// **모르는 형태에서 차단하지 않는다** — 필드가 없는 것과 빈 것은 다르다.
    #[test]
    fn 모르는_형태는_전부_통과한다() {
        for v in [
            json!({}),
            json!({"stop_hook_active": false}),
            json!({"last_assistant_message": null}),
            json!({"last_assistant_message": 0}),
            json!({"last_assistant_message": ["x"]}),
            json!({"stop_hook_active": "true", "last_assistant_message": "다 했다"}),
        ] {
            assert!(!차단인가(&decide("SubagentStop", &v)), "{v} 에서 차단했다");
        }
    }

    #[test]
    fn 우리_사건이_아니면_보지도_않는다() {
        let v = json!({"stop_hook_active": false, "last_assistant_message": ""});
        for event in ["Stop", "SessionStart", "PreToolUse", ""] {
            assert!(!차단인가(&decide(event, &v)), "{event} 에서 차단했다");
        }
    }
}
