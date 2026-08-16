//! 대상 `settings.json` 을 **병합한다** — 그리고 **못 읽으면 아무것도 안 쓴다**(`[f24]` ②).
//!
//! # 왜 이 자리가 게이트에서 가장 조용한 실패 경로인가
//!
//! **실측**: 깨진 `settings.json` 은 `-p` 에서 **완전히 침묵한다**(exit 0 · stderr 0 바이트).
//! 오직 `claude doctor` 와 대화형 다이얼로그만 말한다. 그래서 우리가 반쯤 설치하고
//! 나가면 **아무도 그 상태를 모른다** — `pal doctor` 도 `pal uninstall` 도.
//!
//! 그래서 여기서 하는 일의 순서가 고정돼 있다:
//!
//! 1. **읽는다.** 못 읽으면 **어느 파일의 몇 번째 줄이 왜** 안 읽혔는지 적고 멈춘다.
//! 2. 그 다음에야 쓴다.
//!
//! # 두 가지를 더한다 — 최상위 키와 훅 등록
//!
//! 최상위 키는 **없는 것만** 더한다. 훅 구역은 모양이 달라서 [`super::hooks`] 가
//! 따로 진다 — 거기는 **남의 등록이 함께 사는 배열**이고, 더하고 빼는 규칙이 키와 다르다.

use std::collections::BTreeMap;
use std::path::Path;

use anyhow::{Context, Result, bail};
use serde_json::{Map, Value};

use super::manifest::SettingsEntry;
use super::{blocks, hooks};

/// 병합하기 **전에** 읽어 둔 것. 이것을 만드는 데 실패하면 아무것도 안 쓴다.
pub struct Read {
    /// 지금 파일에 있는 것. 파일이 없으면 `None`.
    pub current: Option<Map<String, Value>>,
}

/// 파일을 읽고 파싱한다. **여기가 ② 의 문이다.**
///
/// # Errors
/// 파일이 있는데 JSON 이 아니거나 최상위가 객체가 아니면.
pub fn read(path: &Path) -> Result<Read> {
    if !path.exists() {
        return Ok(Read { current: None });
    }
    let bytes = super::guard::읽는다(path)?;
    let text = String::from_utf8(bytes).map_err(|e| {
        anyhow::anyhow!("{}: UTF-8 이 아니다 — {e}", path.display())
    })?;

    let value: Value = serde_json::from_str(&text).map_err(|e| {
        // **어느 파일의 몇 번째 줄이 왜** — 게이트 ② 가 표준오류에 요구하는 것 그대로.
        anyhow::anyhow!(
            "{}:{}:{}: JSON 을 읽지 못했다 — {}",
            path.display(),
            e.line(),
            e.column(),
            e
        )
    })?;

    match value {
        Value::Object(map) => Ok(Read { current: Some(map) }),
        other => bail!(
            "{}: 최상위가 객체가 아니다 — {} 이다. 병합할 자리가 없다",
            path.display(),
            종류(&other)
        ),
    }
}

fn 종류(v: &Value) -> &'static str {
    match v {
        Value::Null => "null",
        Value::Bool(_) => "참거짓",
        Value::Number(_) => "수",
        Value::String(_) => "문자열",
        Value::Array(_) => "배열",
        Value::Object(_) => "객체",
    }
}

/// 병합의 결과.
pub struct Merged {
    /// 우리가 **더한** 키. 없던 것만 든다 — 있던 것은 안 건드린다.
    pub added_keys: Vec<String>,
    /// `hooks` 최상위 키를 우리가 만들었는가.
    pub hooks_key_created: bool,
    /// 파일을 우리가 만들었는가.
    pub created: bool,
    /// 실제로 쓰기가 일어났는가. 안 일어나면 바이트가 그대로다(**멱등**).
    pub wrote: bool,
}

/// 없는 키만 더하고 훅 계획을 적용한다. **있던 키·값은 하나도 안 건드린다**
/// (`[f24]` ① 의 부분집합 검사).
///
/// # Errors
/// 훅 구역의 모양이 다르거나 쓰지 못하면.
pub fn merge(
    path: &Path,
    read: &Read,
    want: &BTreeMap<String, Value>,
    plan: &hooks::Plan,
) -> Result<Merged> {
    let mut map = read.current.clone().unwrap_or_default();
    let mut added = Vec::new();
    for (key, value) in want {
        if !map.contains_key(key) {
            map.insert(key.clone(), value.clone());
            added.push(key.clone());
        }
    }
    let hooks_key_created = hooks::apply(&mut map, plan)?;

    // **더할 것도 뺄 것도 없으면 한 바이트도 안 쓴다** — 두 번째 설치가 첫 번째와 같은
    // 상태를 내야 한다(`[f24]` ① 의 멱등).
    if added.is_empty() && plan.is_empty() && read.current.is_some() {
        return Ok(Merged { added_keys: added, hooks_key_created, created: false, wrote: false });
    }

    let mut text = serde_json::to_string_pretty(&Value::Object(map))
        .context("설정을 직렬화하지 못했다")?;
    text.push('\n');

    let created = read.current.is_none();
    if created {
        super::guard::쓴다(path, &그_파일의_줄바꿈으로(path, &text))?;
    } else {
        // **제자리로 쓴다** — 모드·심링크·하드링크를 살린다.
        blocks::write_in_place(path, &그_파일의_줄바꿈으로(path, &text))?;
    }
    Ok(Merged { added_keys: added, hooks_key_created, created, wrote: true })
}

/// 되돌리기가 **무엇을 했는지.** `bool` 하나로는 화면에 적을 것이 없다.
#[derive(Default)]
pub struct Unmerged {
    /// 실제로 뺀 것이 있는가.
    pub 뺐다: bool,
    /// ★ **우리가 넣은 값이 아니었던 키.** 사용자가 자기 값으로 바꿔 둔 자리다 —
    /// 지우는 것은 그대로이고(⑥ 이 `S2 == S0` 을 요구한다) 여기서 더하는 것은 **말**이다.
    pub 사용자가_바꾼_키: Vec<String>,
    /// 파일을 통째로 지웠는가 — 우리가 만들었고 나머지가 비었을 때.
    pub 파일째_지웠다: bool,
}

/// 우리가 더한 키와 우리가 등록한 훅만 뺀다.
///
/// **손잡이를 매니페스트 항목으로 든다** — 위치 인자 넷 중 둘이 `bool` 이면 부르는
/// 자리에서 어느 것이 무엇인지 안 보인다.
///
/// # Errors
/// 못 읽거나(파싱 실패 포함) 못 쓰면.
pub fn unmerge(path: &Path, entry: &SettingsEntry) -> Result<Unmerged> {
    if !path.exists() {
        return Ok(Unmerged::default());
    }
    let read = read(path)?;
    let Some(mut map) = read.current else { return Ok(Unmerged::default()) };
    let mut out = Unmerged { 뺐다: true, ..Unmerged::default() };
    for key in &entry.added_keys {
        // ★ **우리가 넣은 값과 다른가.** 옛 매니페스트에는 값이 안 실려 있어서
        // (`added_values` 가 비어 있어서) 그때는 「모른다」이고 말하지 않는다.
        if let (Some(넣은), Some(지금)) = (entry.added_values.get(key), map.get(key)) {
            if 넣은 != 지금 {
                out.사용자가_바꾼_키.push(key.clone());
            }
        }
        map.remove(key);
    }
    hooks::strip(&mut map, &entry.hooks, entry.hooks_key_created);

    if entry.created && map.is_empty() {
        std::fs::remove_file(path)
            .with_context(|| format!("지우지 못했다: {}", path.display()))?;
        out.파일째_지웠다 = true;
        return Ok(out);
    }
    let mut text = serde_json::to_string_pretty(&Value::Object(map))
        .context("설정을 직렬화하지 못했다")?;
    text.push('\n');
    blocks::write_in_place(path, &그_파일의_줄바꿈으로(path, &text))?;
    Ok(out)
}

/// 직렬화한 본문을 **그 파일이 쓰던 줄바꿈에 맞춘다.**
///
/// # 왜 여기에도 [`super::eol`] 이 필요한가
///
/// `serde_json::to_string_pretty` 는 언제나 LF 를 낸다. `core.autocrlf=true` 로 클론한
/// 워킹트리에서 `settings.json` 은 CRLF 인데, 우리가 LF 로 되쓰면 **파일 전체의 모든
/// 줄이 바뀐다** — 사용자의 `git status` 에 우리 파일이 매번 뜨고, git 이 되쓰기마다
/// *"LF will be replaced by CRLF"* 를 낸다.
///
/// 블록(`CLAUDE.md`·`.gitignore`)에는 이 규율이 이미 서 있었다(소유자 결정 2026-08-16:
/// **판정은 정규화해서, 되쓰기는 있던 대로**). `settings.json` 만 그 문 밖에 있었고,
/// 그것이 **플랫폼 때문에 결과가 갈리는 자리**였다 — 유닉스 워킹트리에서는 아무 일도
/// 안 일어나고 Windows 에서만 매번 파일이 통째로 더러워진다.
///
/// ⚠ **직렬화 「형태」는 여기서 안 고친다.** 들여쓰기·키 순서가 우리 것이 되는 것은
/// 플랫폼과 무관한 기존 결정이고(`tests/install.rs` 의 ⑥ 이 `settings.json` 을 값
/// 단위로 재는 이유가 그것이다), 이 회차의 범위가 아니다.
fn 그_파일의_줄바꿈으로(path: &Path, text: &str) -> Vec<u8> {
    let 기존 = std::fs::read(path).ok();
    let crlf = super::eol::그_파일의_줄바꿈(기존.as_deref());
    super::eol::맞춘다(text.as_bytes(), crlf)
}

#[cfg(test)]
mod tests {
    use super::{Merged, Read, SettingsEntry, hooks, merge, read, unmerge};
    use crate::install::inside::Rel;
    use crate::install::manifest::HookEntry;
    use serde_json::{Value, json};
    use std::collections::BTreeMap;

    fn 방(tag: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("pal-settings-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("방");
        dir
    }

    fn 바람() -> BTreeMap<String, Value> {
        let mut want = BTreeMap::new();
        want.insert("agent".to_owned(), json!("pal-orchestrator"));
        want
    }

    fn 훅() -> Vec<HookEntry> {
        vec![hooks::entry(std::path::Path::new("/bin/pal"), "SubagentStop")]
    }

    fn 계획(r: &Read, 훅: &[HookEntry]) -> hooks::Plan {
        hooks::plan(r.current.as_ref(), &[], 훅)
    }

    /// 매니페스트가 지고 갈 항목 — 되돌리기가 이것 하나만 본다.
    fn 항목(m: &Merged, 훅: &[HookEntry]) -> SettingsEntry {
        SettingsEntry {
            path: Rel::new("settings.json"),
            added_keys: m.added_keys.clone(),
            added_values: 바람()
                .into_iter()
                .filter(|(k, _)| m.added_keys.contains(k))
                .collect(),
            hooks: 훅.to_vec(),
            hooks_key_created: m.hooks_key_created,
            created: m.created,
        }
    }

    /// **깨진 JSON 은 읽기에서 멈춘다** — 쓰기 경로에 못 간다.
    #[test]
    fn 깨진_json_은_줄과_까닭을_적는다() {
        let dir = 방("깨짐");
        for (이름, 본문) in [("끝없음", "{\n \"a\": 1\n"), ("후행쉼표", "{\"a\": 1,}"), ("빈파일", "")] {
            let path = dir.join(이름);
            std::fs::write(&path, 본문).expect("쓰기");
            let Err(e) = read(&path) else { panic!("{이름}: 깨진 JSON 이 읽혔다") };
            let err = e.to_string();
            assert!(err.contains(이름), "어느 파일인지 안 적었다: {err}");
            assert!(err.contains(':'), "줄/칸을 안 적었다: {err}");
        }
    }

    /// **최상위가 객체가 아니면 병합할 자리가 없다** — 조용히 덮지 않는다.
    #[test]
    fn 최상위가_배열이면_거부한다() {
        let dir = 방("배열");
        let path = dir.join("settings.json");
        std::fs::write(&path, "[1,2]").expect("쓰기");
        assert!(read(&path).is_err());
    }

    /// **설치 전의 모든 키·값이 설치 후에도 그대로** — ① 의 부분집합 검사.
    #[test]
    fn 사용자_키가_전부_살아_있다() {
        let dir = 방("보존");
        let path = dir.join("settings.json");
        let 원본 = json!({"agent": "내 것", "env": {"A": "1"}, "permissions": {"allow": ["x"]}});
        std::fs::write(&path, serde_json::to_string_pretty(&원본).expect("직렬화")).expect("쓰기");

        let r = read(&path).expect("읽기");
        let m = merge(&path, &r, &바람(), &hooks::Plan::default()).expect("병합");
        assert!(m.added_keys.is_empty(), "이미 있는 키를 더했다고 적었다");
        assert!(!m.wrote, "안 더했는데 썼다");

        let 뒤: Value = serde_json::from_slice(&std::fs::read(&path).expect("읽기")).expect("JSON");
        assert_eq!(뒤, 원본, "사용자 키·값이 달라졌다");
    }

    /// **키도 훅도 왕복하면 사용자 값이 그대로 돌아온다.**
    #[test]
    fn 없던_키만_더하고_왕복하면_값이_돌아온다() {
        let dir = 방("왕복");
        let path = dir.join("settings.json");
        let 원본 = json!({"env": {"A": "1"}});
        std::fs::write(&path, serde_json::to_string_pretty(&원본).expect("직렬화")).expect("쓰기");

        let r = read(&path).expect("읽기");
        let m = merge(&path, &r, &바람(), &계획(&r, &훅())).expect("병합");
        assert_eq!(m.added_keys, vec!["agent".to_owned()]);
        assert!(m.hooks_key_created, "훅 구역을 우리가 만들었는데 안 적었다");

        let 중간: Value = serde_json::from_slice(&std::fs::read(&path).expect("읽기")).expect("JSON");
        assert!(중간["hooks"]["SubagentStop"].is_array(), "훅이 안 걸렸다: {중간}");

        unmerge(&path, &항목(&m, &훅())).expect("되돌리기");
        let 뒤: Value = serde_json::from_slice(&std::fs::read(&path).expect("읽기")).expect("JSON");
        assert_eq!(뒤, 원본);
    }

    #[test]
    fn 우리가_만든_파일은_비면_사라진다() {
        let dir = 방("생성");
        let path = dir.join("settings.json");
        let r = read(&path).expect("읽기");
        let m = merge(&path, &r, &바람(), &계획(&r, &훅())).expect("병합");
        assert!(m.created);
        unmerge(&path, &항목(&m, &훅())).expect("되돌리기");
        assert!(!path.exists());
    }
}
