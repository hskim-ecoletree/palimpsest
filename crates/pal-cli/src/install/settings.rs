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
//! # 이 회차는 훅을 등록하지 않는다
//!
//! 훅 규약 측정이 아직 서지 않았다(`[f24]` ⑧ 이 *"형태를 합격선에 안 박는다"* 로
//! 남겨 둔 자리). 지금 다루는 것은 **최상위 `agent` 키 하나**다.

use std::collections::BTreeMap;
use std::path::Path;

use anyhow::{Context, Result, bail};
use serde_json::{Map, Value};

use super::blocks;

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
    let bytes = std::fs::read(path)
        .with_context(|| format!("읽지 못했다: {}", path.display()))?;
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
    /// 파일을 우리가 만들었는가.
    pub created: bool,
    /// 실제로 쓰기가 일어났는가. 안 일어나면 바이트가 그대로다(**멱등**).
    pub wrote: bool,
}

/// 없는 키만 더한다. **있던 키·값은 하나도 안 건드린다**(`[f24]` ① 의 부분집합 검사).
///
/// # Errors
/// 쓰지 못하면.
pub fn merge(path: &Path, read: &Read, want: &BTreeMap<String, Value>) -> Result<Merged> {
    let mut map = read.current.clone().unwrap_or_default();
    let mut added = Vec::new();
    for (key, value) in want {
        if !map.contains_key(key) {
            map.insert(key.clone(), value.clone());
            added.push(key.clone());
        }
    }

    if added.is_empty() && read.current.is_some() {
        return Ok(Merged { added_keys: added, created: false, wrote: false });
    }

    let mut text = serde_json::to_string_pretty(&Value::Object(map))
        .context("설정을 직렬화하지 못했다")?;
    text.push('\n');

    let created = read.current.is_none();
    if created {
        std::fs::write(path, text.as_bytes())
            .with_context(|| format!("쓰지 못했다: {}", path.display()))?;
    } else {
        // **제자리로 쓴다** — 모드·심링크·하드링크를 살린다.
        blocks::write_in_place(path, text.as_bytes())?;
    }
    Ok(Merged { added_keys: added, created, wrote: true })
}

/// 우리가 더한 키만 뺀다.
///
/// # Errors
/// 못 읽거나(파싱 실패 포함) 못 쓰면.
pub fn unmerge(path: &Path, added_keys: &[String], created: bool) -> Result<bool> {
    if !path.exists() {
        return Ok(false);
    }
    let read = read(path)?;
    let Some(mut map) = read.current else { return Ok(false) };
    for key in added_keys {
        map.remove(key);
    }

    if created && map.is_empty() {
        std::fs::remove_file(path)
            .with_context(|| format!("지우지 못했다: {}", path.display()))?;
        return Ok(true);
    }
    let mut text = serde_json::to_string_pretty(&Value::Object(map))
        .context("설정을 직렬화하지 못했다")?;
    text.push('\n');
    blocks::write_in_place(path, text.as_bytes())?;
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::{merge, read, unmerge};
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
        let m = merge(&path, &r, &바람()).expect("병합");
        assert!(m.added_keys.is_empty(), "이미 있는 키를 더했다고 적었다");
        assert!(!m.wrote, "안 더했는데 썼다");

        let 뒤: Value = serde_json::from_slice(&std::fs::read(&path).expect("읽기")).expect("JSON");
        assert_eq!(뒤, 원본, "사용자 키·값이 달라졌다");
    }

    #[test]
    fn 없던_키만_더하고_왕복하면_값이_돌아온다() {
        let dir = 방("왕복");
        let path = dir.join("settings.json");
        let 원본 = json!({"env": {"A": "1"}});
        std::fs::write(&path, serde_json::to_string_pretty(&원본).expect("직렬화")).expect("쓰기");

        let r = read(&path).expect("읽기");
        let m = merge(&path, &r, &바람()).expect("병합");
        assert_eq!(m.added_keys, vec!["agent".to_owned()]);

        unmerge(&path, &m.added_keys, m.created).expect("되돌리기");
        let 뒤: Value = serde_json::from_slice(&std::fs::read(&path).expect("읽기")).expect("JSON");
        assert_eq!(뒤, 원본);
    }

    #[test]
    fn 우리가_만든_파일은_비면_사라진다() {
        let dir = 방("생성");
        let path = dir.join("settings.json");
        let r = read(&path).expect("읽기");
        let m = merge(&path, &r, &바람()).expect("병합");
        assert!(m.created);
        unmerge(&path, &m.added_keys, m.created).expect("되돌리기");
        assert!(!path.exists());
    }
}
