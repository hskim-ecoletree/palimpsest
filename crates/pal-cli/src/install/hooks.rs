//! 대상 `settings.json` 의 훅 구역 — **등록 · 갱신 · 제거**(`[f24]` ⑧).
//!
//! # 실측이 이 파일의 모든 줄을 정했다
//!
//! - **등록 문자열은 `/bin/sh -c "<원문>"` 으로 실행된다.** 셸 메타문자가 들어가면
//!   셸이 해석하고, **경로에 공백이 있으면 따옴표가 필요하다.** 그래서 실행 파일
//!   경로를 언제나 **홑따옴표**로 감싼다 — 겹따옴표는 `$`·`` ` ``·`\` 를 살려 두지만
//!   홑따옴표는 전부 죽인다.
//! - **중복 제거는 명령 문자열의 완전 일치 기준이다. 공백 하나만 달라도 두 번 돈다.**
//!   그래서 이 파일이 만드는 문자열은 **바이트 단위로 안정적**이어야 하고, 제거도
//!   완전 일치로만 한다.
//! - **훅은 전 레이어의 합집합**이다. 우리는 프로젝트 레이어 하나만 만지고, 남이
//!   같은 사건에 걸어 둔 것을 **하나도 안 건드린다.**
//!
//! # ★ PATH 이름으로 등록하지 않는다
//!
//! 실측상 PATH 이름 등록도 동작한다(네이티브 바이너리의 `argv[0]` 이 맨 이름 그대로
//! 들어온다). 그런데 **실행 파일을 못 찾으면 exit 127 이고 그 실패는 완전히
//! 침묵한다** — `claude` 의 종료 코드는 0 이고 트랜스크립트에도 대화형 화면에도 흔적이
//! 없다. 그래서 **설치 시점에 해석한 절대 경로**로 등록한다.
//!
//! # 남의 구조를 고치려 들지 않는다
//!
//! `hooks` 가 객체가 아니거나 사건 자리가 배열이 아니면 **고치지 않고 멈춘다.**
//! 반쯤 고친 설정은 하네스의 `-p` 에서 완전히 침묵하고, 그러면 아무도 그 상태를 모른다.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use serde_json::{Map, Value, json};

use super::manifest::HookEntry;

/// 설정 안의 최상위 훅 구역.
const HOOKS: &str = "hooks";
/// 묶음 안의 명령 목록.
const GROUP: &str = "hooks";
const COMMAND: &str = "command";
const KIND: &str = "type";
const KIND_COMMAND: &str = "command";

// ─────────────────────────────────────────────────────────────────────────────
// 등록 문자열 — **바이트 단위로 안정적이어야 한다**
// ─────────────────────────────────────────────────────────────────────────────

/// 이 설치가 등록할 것. 사건 목록이 비면 실행 파일도 안 찾는다.
///
/// # Errors
/// 자기 실행 파일의 경로를 못 알아내면.
pub fn desired(events: &[&str]) -> Result<Vec<HookEntry>> {
    if events.is_empty() {
        return Ok(Vec::new());
    }
    let exe = 실행_파일()?;
    Ok(events
        .iter()
        .map(|e| HookEntry { event: (*e).to_owned(), command: command(&exe, e) })
        .collect())
}

/// **지금 도는 이 바이너리**의 절대 경로.
///
/// ⚠ 홈을 안 읽는다 — `[f24]` ⑦ 이 재는 자리다.
fn 실행_파일() -> Result<PathBuf> {
    let exe = std::env::current_exe().context(
        "자기 실행 파일의 경로를 못 알아냈다 — 훅을 등록할 절대 경로가 없다",
    )?;
    // 심링크를 풀어 둔다. 안 풀면 링크가 갈릴 때 등록 문자열이 **조용히** 딴 것을 가리킨다.
    Ok(exe.canonicalize().unwrap_or(exe))
}

/// 등록 문자열 하나.
#[must_use]
pub fn command(exe: &Path, event: &str) -> String {
    format!("{} hook {event}", 홑따옴표(&exe.to_string_lossy()))
}

/// POSIX 홑따옴표 — 안의 `'` 는 `'\''` 로 닫았다 다시 연다.
fn 홑따옴표(s: &str) -> String {
    format!("'{}'", s.replace('\'', r"'\''"))
}

// ─────────────────────────────────────────────────────────────────────────────
// 무엇을 더하고 무엇을 뺄 것인가 — **순수 함수. 그래서 시험이 잡는다**
// ─────────────────────────────────────────────────────────────────────────────

/// 더할 것과 뺄 것.
#[derive(Default)]
pub struct Plan {
    /// 아직 안 걸린 것.
    pub add: Vec<HookEntry>,
    /// **우리가 걸었는데 이제 바라지 않는 것** — 실행 파일이 옮겨 갔거나 사건 목록이
    /// 줄었을 때 여기가 찬다. 안 빼면 죽은 등록이 남고, 그 실패는 침묵한다.
    pub remove: Vec<HookEntry>,
}

impl Plan {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.add.is_empty() && self.remove.is_empty()
    }
}

/// 지금 설정 · 우리가 적어 둔 것 · 바라는 것 셋을 대서 계획을 낸다.
#[must_use]
pub fn plan(
    current: Option<&Map<String, Value>>,
    recorded: &[HookEntry],
    desired: &[HookEntry],
) -> Plan {
    let add = desired
        .iter()
        .filter(|d| !registered(current, &d.event, &d.command))
        .cloned()
        .collect();
    let remove = recorded
        .iter()
        .filter(|r| !desired.iter().any(|d| d == *r))
        .filter(|r| registered(current, &r.event, &r.command))
        .cloned()
        .collect();
    Plan { add, remove }
}

/// 그 명령이 그 사건에 **완전 일치로** 걸려 있는가.
#[must_use]
pub fn registered(current: Option<&Map<String, Value>>, event: &str, command: &str) -> bool {
    let Some(map) = current else { return false };
    let Some(Value::Array(groups)) = map.get(HOOKS).and_then(|h| h.get(event)) else {
        return false;
    };
    groups.iter().any(|g| {
        g.get(GROUP).and_then(Value::as_array).is_some_and(|cmds| {
            cmds.iter().any(|c| c.get(COMMAND).and_then(Value::as_str) == Some(command))
        })
    })
}

/// 계획을 설정 지도에 적용한다. **더한 것이 있으면 `hooks` 키를 우리가 만들었는지**를
/// 함께 낸다 — 제거가 그것만 되돌린다.
///
/// # Errors
/// `hooks` 가 객체가 아니거나 사건 자리가 배열이 아니면. **고치려 들지 않는다.**
pub fn apply(map: &mut Map<String, Value>, plan: &Plan) -> Result<bool> {
    for entry in &plan.remove {
        뺀다(map, &entry.event, &entry.command);
    }
    let mut 우리가_만들었나 = false;
    for entry in &plan.add {
        우리가_만들었나 |= 더한다(map, &entry.event, &entry.command)?;
    }
    치운다(map, false);
    Ok(우리가_만들었나)
}

/// 우리가 적어 둔 것을 전부 뺀다 — **제거의 자리.**
///
/// `hooks_key_created` 가 참일 때만 빈 `hooks` 키를 지운다. 사용자가 원래 `"hooks": {}`
/// 를 두었으면 그것은 사용자의 것이다.
pub fn strip(map: &mut Map<String, Value>, recorded: &[HookEntry], hooks_key_created: bool) -> bool {
    let mut 뺐다 = false;
    for entry in recorded {
        뺐다 |= 뺀다(map, &entry.event, &entry.command);
    }
    치운다(map, hooks_key_created);
    뺐다
}

fn 더한다(map: &mut Map<String, Value>, event: &str, command: &str) -> Result<bool> {
    let 없었다 = !map.contains_key(HOOKS);
    let hooks = map.entry(HOOKS).or_insert_with(|| json!({}));
    let Value::Object(hooks) = hooks else {
        bail!("`{HOOKS}` 가 객체가 아니다 — 남의 구조를 고치려 들지 않는다");
    };
    let groups = hooks.entry(event).or_insert_with(|| json!([]));
    let Value::Array(groups) = groups else {
        bail!("`{HOOKS}.{event}` 이 배열이 아니다 — 남의 구조를 고치려 들지 않는다");
    };
    // **우리 묶음 하나를 따로 넣는다.** 남의 묶음에 끼워 넣으면 제거가 남의 것을 건드린다.
    groups.push(json!({ GROUP: [{ KIND: KIND_COMMAND, COMMAND: command }] }));
    Ok(없었다)
}

fn 뺀다(map: &mut Map<String, Value>, event: &str, command: &str) -> bool {
    let Some(Value::Object(hooks)) = map.get_mut(HOOKS) else { return false };
    let Some(Value::Array(groups)) = hooks.get_mut(event) else { return false };

    let mut 뺐다 = false;
    groups.retain_mut(|g| {
        let Some(Value::Array(cmds)) = g.get_mut(GROUP) else { return true };
        let 전 = cmds.len();
        cmds.retain(|c| c.get(COMMAND).and_then(Value::as_str) != Some(command));
        if cmds.len() == 전 {
            return true;
        }
        뺐다 = true;
        // **우리가 비운 묶음만 지운다.** 남의 것이 함께 든 묶음은 남는다.
        !cmds.is_empty()
    });
    if 뺐다 && groups.is_empty() {
        hooks.remove(event);
    }
    뺐다
}

/// 빈 자리를 치운다. `hooks` 키 자체는 **우리가 만들었을 때만** 지운다.
fn 치운다(map: &mut Map<String, Value>, 우리가_만들었나: bool) {
    if !우리가_만들었나 {
        return;
    }
    if map.get(HOOKS).and_then(Value::as_object).is_some_and(Map::is_empty) {
        map.remove(HOOKS);
    }
}

#[cfg(test)]
mod tests {
    use super::{HookEntry, apply, command, plan, registered, strip};
    use serde_json::{Map, Value, json};
    use std::path::Path;

    fn 지도(v: &Value) -> Map<String, Value> {
        v.as_object().expect("객체").clone()
    }

    fn 바람(command: &str) -> Vec<HookEntry> {
        vec![HookEntry { event: "SubagentStop".to_owned(), command: command.to_owned() }]
    }

    /// 적어 둔 것 없이 새로 건다 — 시험마다 두 줄이 되는 자리를 접는다.
    fn 건다(map: &mut Map<String, Value>, 바람: &[HookEntry]) -> bool {
        let p = plan(Some(&*map), &[], 바람);
        apply(map, &p).expect("적용")
    }

    /// **경로에 공백이 있으면 따옴표가 필요하다** — 없으면 셸이 갈라 읽고 exit 127 이
    /// 나며 그 실패는 침묵한다.
    #[test]
    fn 공백이_든_경로가_따옴표로_묶인다() {
        let c = command(Path::new("/opt/pal 도구/pal"), "SubagentStop");
        assert_eq!(c, "'/opt/pal 도구/pal' hook SubagentStop");
    }

    /// 홑따옴표가 든 경로도 셸에서 한 낱말이다.
    #[test]
    fn 홑따옴표가_든_경로도_한_낱말이다() {
        let c = command(Path::new("/opt/it's/pal"), "SubagentStop");
        assert_eq!(c, r"'/opt/it'\''s/pal' hook SubagentStop");
    }

    /// ★ **같은 설치에서 두 번 등록하면 두 번 돈다** — 중복 제거가 완전 일치 기준이므로
    /// 두 번째 계획은 비어야 한다.
    #[test]
    fn 두_번째_계획은_비어_있다() {
        let mut map = Map::new();
        let 바람 = 바람("'/bin/pal' hook SubagentStop");
        let p = plan(Some(&map), &[], &바람);
        assert_eq!(p.add.len(), 1);
        apply(&mut map, &p).expect("적용");

        let p2 = plan(Some(&map), &바람, &바람);
        assert!(p2.is_empty(), "두 번째가 또 등록하려 한다");
    }

    /// **실행 파일이 옮겨 가면 옛 등록을 뺀다** — 안 빼면 죽은 등록이 침묵으로 남는다.
    #[test]
    fn 옮겨_가면_옛_등록을_뺀다() {
        let mut map = Map::new();
        let 옛 = 바람("'/옛/pal' hook SubagentStop");
        건다(&mut map, &옛);

        let 새 = 바람("'/새/pal' hook SubagentStop");
        let p = plan(Some(&map), &옛, &새);
        assert_eq!(p.add.len(), 1);
        assert_eq!(p.remove.len(), 1);
        apply(&mut map, &p).expect("적용");

        assert!(!registered(Some(&map), "SubagentStop", &옛[0].command));
        assert!(registered(Some(&map), "SubagentStop", &새[0].command));
    }

    /// ★ **남이 같은 사건에 걸어 둔 것을 하나도 안 건드린다.**
    #[test]
    fn 남의_등록은_왕복해도_그대로다() {
        let 남의것 = json!({
            "hooks": {
                "SubagentStop": [{"hooks": [{"type": "command", "command": "남의 것.sh"}]}],
                "SessionStart": [{"hooks": [{"type": "command", "command": "남의 시작.sh"}]}]
            }
        });
        let mut map = 지도(&남의것);
        let 바람 = 바람("'/bin/pal' hook SubagentStop");
        건다(&mut map, &바람);
        assert_eq!(map["hooks"]["SubagentStop"].as_array().expect("배열").len(), 2);

        strip(&mut map, &바람, false);
        assert_eq!(Value::Object(map), 남의것, "왕복이 남의 것을 바꿨다");
    }

    /// **우리가 만든 `hooks` 키는 비면 사라진다.** 사용자가 만든 것은 안 사라진다.
    #[test]
    fn 우리가_만든_훅_키만_사라진다() {
        let 바람 = 바람("'/bin/pal' hook SubagentStop");

        let mut 우리것 = Map::new();
        let 만들었나 = 건다(&mut 우리것, &바람);
        assert!(만들었나);
        strip(&mut 우리것, &바람, 만들었나);
        assert!(우리것.is_empty(), "우리가 만든 키가 남았다: {우리것:?}");

        let mut 남의것 = 지도(&json!({"hooks": {}}));
        let 만들었나 = 건다(&mut 남의것, &바람);
        assert!(!만들었나, "남이 만든 키를 우리가 만들었다고 적었다");
        strip(&mut 남의것, &바람, 만들었나);
        assert_eq!(Value::Object(남의것), json!({"hooks": {}}));
    }

    /// **남의 구조를 고치려 들지 않는다** — 모양이 다르면 멈춘다.
    #[test]
    fn 모양이_다르면_멈춘다() {
        for 이상한 in [json!({"hooks": "문자열"}), json!({"hooks": {"SubagentStop": 1}})] {
            let mut map = 지도(&이상한);
            let 바람 = 바람("'/bin/pal' hook SubagentStop");
            let p = plan(Some(&map), &[], &바람);
            assert!(apply(&mut map, &p).is_err(), "{이상한} 에서 안 멈췄다");
        }
    }

    /// 사건 자리가 비면 그 열쇠도 사라진다 — 빈 배열이 남으면 그것이 곧 잔해다.
    #[test]
    fn 우리만_있던_사건_자리는_통째로_사라진다() {
        let mut map = Map::new();
        let 바람 = 바람("'/bin/pal' hook SubagentStop");
        let 만들었나 = 건다(&mut map, &바람);
        strip(&mut map, &바람, 만들었나);
        assert!(map.is_empty());
    }
}
