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
//!
//! # ★ 재직렬화하지 않는다 (회차 `2026-09-15-clean-uninstall` 계획 2)
//!
//! 착수 동작은 값을 고친 뒤 `to_string_pretty` 로 되썼다. 그러면 사용자 파일의 들여쓰기 · 키 순서 ·
//! 이스케이프 · 수 표기가 우리 것이 되고, **값을 되돌려도 바이트는 안 돌아온다**(착수 관측 R1).
//! 지금은 [`super::json_edit`] 가 **텍스트 자리**로 더하고 제거가 그 자리만 뺀다. `serde_json` 은 검증과
//! 값 대조에만 쓴다.
//!
//! | 제거가 만나는 방 | 되돌림 |
//! |---|---|
//! | 매니페스트에 편집 기록이 있다 | 우리 멤버 · 훅 묶음만 텍스트 자리로 뺀다. 사용자가 바꾼 `agent` 는 **남긴다** |
//! | 기록이 없다(착수 커밋 `acd7e82` 이하가 재직렬화한 방) | 값으로 뺀 뒤, 추적 중이고 `HEAD` 의 그 파일이 우리 흔적 없이 결과와 같은 값이면 **`HEAD` 바이트로 되쓴다**. 아니면 값만 되돌리고 그렇게 말한다 |

use std::collections::BTreeMap;
use std::path::Path;
use std::process::{Command, Stdio};

use anyhow::{Context, Result, bail};
use serde_json::{Map, Value};

use super::json_edit::{길목, 본문, 조각};
use super::manifest::{SettingsEdits, SettingsEntry};
use super::{blocks, hooks};

/// 병합하기 **전에** 읽어 둔 것. 이것을 만드는 데 실패하면 아무것도 안 쓴다.
pub struct Read {
    /// 지금 파일에 있는 것. 파일이 없으면 `None`.
    pub current: Option<Map<String, Value>>,
    /// 그 값을 읽은 **본문 그대로** — 위치 보존 편집이 이 텍스트 위에서 일한다.
    pub 본문: Option<String>,
}

/// 파일을 읽고 파싱한다. **여기가 ② 의 문이다.**
///
/// # Errors
/// 파일이 있는데 JSON 이 아니거나 최상위가 객체가 아니면.
pub fn read(path: &Path) -> Result<Read> {
    if !path.exists() {
        return Ok(Read { current: None, 본문: None });
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
        Value::Object(map) => Ok(Read { current: Some(map), 본문: Some(text) }),
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
    /// 이번 편집의 자리 기록 — 매니페스트의 [`SettingsEntry::edits`] 로 간다.
    pub edits: SettingsEdits,
}

/// 없는 키만 더하고 훅 계획을 적용한다. **있던 키·값은 하나도 안 건드린다**
/// (`[f24]` ① 의 부분집합 검사) — 그리고 **있던 바이트도 안 건드린다.**
///
/// # Errors
/// 훅 구역의 모양이 다르거나 쓰지 못하면.
pub fn merge(
    path: &Path,
    read: &Read,
    want: &BTreeMap<String, Value>,
    plan: &hooks::Plan,
) -> Result<Merged> {
    let created = read.current.is_none();
    // 파일이 없으면 빈 객체에서 시작한다 — 새로 만드는 것은 우리 것이라 LF · 두 칸이다.
    let 원문 = read.본문.clone().unwrap_or_else(|| "{}\n".to_owned());
    let mut 본 = 본문::읽는다(원문).with_context(|| format!("{}: 편집할 본문을 못 읽었다", path.display()))?;
    let mut edits = SettingsEdits::default();

    let mut added = Vec::new();
    for (key, value) in want {
        if 본.종류(&[길목::키(key)]).is_none() {
            if let Some(안쪽) = 본.멤버를_더한다(&[], key, &조각::값에서(value))? {
                edits.빈_안쪽을_적는다(SettingsEdits::포인터(&[]), &안쪽);
            }
            added.push(key.clone());
        }
    }
    let hooks_key_created = hooks::본문에_적용한다(&mut 본, plan, &mut edits)?;

    // **더할 것도 뺄 것도 없으면 한 바이트도 안 쓴다** — 두 번째 설치가 첫 번째와 같은
    // 상태를 산출해야 한다(`[f24]` ① 의 멱등).
    if added.is_empty() && plan.is_empty() && !created {
        return Ok(Merged { added_keys: added, hooks_key_created, created, wrote: false, edits });
    }

    if created {
        super::guard::쓴다(path, 본.텍스트().as_bytes())?;
    } else {
        // **제자리로 쓴다** — 모드·심링크·하드링크를 살린다.
        blocks::write_in_place(path, 본.텍스트().as_bytes())?;
    }
    Ok(Merged { added_keys: added, hooks_key_created, created, wrote: true, edits })
}

/// 되돌리기가 **어떤 바이트를 썼는가.**
#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum 되돌림 {
    /// 우리 몫만 텍스트 자리로 뺐다 — 나머지 바이트는 그대로다.
    #[default]
    자리,
    /// 옛 설치 — `HEAD` 의 그 파일 바이트로 되썼다.
    커밋된_바이트,
    /// 옛 설치 — 값만 되돌렸다. 원래 바이트는 모른다.
    값만,
}

/// 되돌리기가 **무엇을 했는지.** `bool` 하나로는 화면에 적을 것이 없다.
#[derive(Default)]
pub struct Unmerged {
    /// 실제로 뺀 것이 있는가.
    pub 뺐다: bool,
    /// ★ **우리가 넣은 값이 아니었던 키 — 남겼다.** 설치 뒤 사용자가 자기 값으로 바꿔 둔 자리이고,
    /// 그것은 사용자가 스스로 고친 것이다(계획 2 · 착수 동작은 지웠다).
    pub 남긴_키: Vec<String>,
    /// 파일을 통째로 지웠는가 — 우리가 만들었고 나머지가 비었을 때.
    pub 파일째_지웠다: bool,
    /// 어떤 바이트로 되돌렸는가.
    pub 되돌림: 되돌림,
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
    let (Some(map), Some(원문)) = (read.current, read.본문) else { return Ok(Unmerged::default()) };
    match &entry.edits {
        Some(기록) => 자리로_되돌린다(path, &원문, entry, 기록),
        None => 값으로_되돌린다(path, map, entry),
    }
}

/// 사용자가 **우리가 넣은 값을 바꿨나.** 값이 안 실린 옛 매니페스트면 「모른다」이고 바꿨다고 안 읽는다.
fn 사용자가_바꿨나(entry: &SettingsEntry, key: &str, 지금: &Value) -> bool {
    entry.added_values.get(key).is_some_and(|넣은| 넣은 != 지금)
}

/// 편집 기록이 있는 방 — **넣은 순서의 역순**으로 우리 자리만 뺀다(훅 먼저, 그다음 키).
fn 자리로_되돌린다(
    path: &Path,
    원문: &str,
    entry: &SettingsEntry,
    기록: &SettingsEdits,
) -> Result<Unmerged> {
    let mut 본 = 본문::읽는다(원문.to_owned()).with_context(|| format!("{}: 본문을 못 읽었다", path.display()))?;
    let mut out = Unmerged { 뺐다: true, ..Unmerged::default() };

    hooks::본문에서_뺀다(&mut 본, &entry.hooks, entry.hooks_key_created, 기록)?;
    for key in entry.added_keys.iter().rev() {
        let Some(지금) = 본.값(&[길목::키(key)]) else { continue };
        if 사용자가_바꿨나(entry, key, &지금) {
            out.남긴_키.push(key.clone());
            continue;
        }
        본.멤버를_뺀다(&[], key, 기록.빈_안쪽(&SettingsEdits::포인터(&[])))?;
    }
    out.남긴_키.reverse();

    if entry.created && 본.자식_수(&[]) == Some(0) {
        std::fs::remove_file(path)
            .with_context(|| format!("지우지 못했다: {}", path.display()))?;
        out.파일째_지웠다 = true;
        return Ok(out);
    }
    if 본.텍스트() != 원문 {
        blocks::write_in_place(path, 본.텍스트().as_bytes())?;
    }
    Ok(out)
}

/// 편집 기록이 없는 방(착수 커밋 `acd7e82` 이하가 재직렬화한 방) — 값으로 뺀 뒤 되쓸 바이트를 고른다.
fn 값으로_되돌린다(path: &Path, mut map: Map<String, Value>, entry: &SettingsEntry) -> Result<Unmerged> {
    let mut out = Unmerged { 뺐다: true, 되돌림: 되돌림::값만, ..Unmerged::default() };
    for key in &entry.added_keys {
        let Some(지금) = map.get(key) else { continue };
        if 사용자가_바꿨나(entry, key, 지금) {
            out.남긴_키.push(key.clone());
            continue;
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

    // ★ **`HEAD` 바이트로 되쓸 수 있나** — 추적 중이고, `HEAD` 의 그 파일에 우리 키·훅이 없고, 결과와 같은 값일 때만.
    // 셋 중 하나라도 빠지면 `HEAD` 는 설치 전 원본이 아니다(설치 뒤 커밋했거나 · 사용자가 설치 뒤 고쳤다).
    if let Some(head) = 커밋된_바이트(path) {
        if 설치_전_원본인가(&head, &map, entry) {
            blocks::write_in_place(path, &head)?;
            out.되돌림 = 되돌림::커밋된_바이트;
            return Ok(out);
        }
    }

    let mut text = serde_json::to_string_pretty(&Value::Object(map))
        .context("설정을 직렬화하지 못했다")?;
    text.push('\n');
    blocks::write_in_place(path, &그_파일의_줄바꿈으로(path, &text))?;
    Ok(out)
}

/// `HEAD` 의 바이트가 **우리 흔적이 없고 되돌린 결과와 같은 값**인가.
fn 설치_전_원본인가(head: &[u8], 결과: &Map<String, Value>, entry: &SettingsEntry) -> bool {
    let Ok(Value::Object(h)) = serde_json::from_slice::<Value>(head) else { return false };
    let 우리_키 = entry.added_keys.iter().any(|k| h.contains_key(k));
    let 우리_훅 = entry.hooks.iter().any(|e| hooks::registered(Some(&h), e));
    !우리_키 && !우리_훅 && &h == 결과
}

/// 그 파일이 git 에 **추적 중이면** `HEAD` 의 그 파일을 체크아웃이 쓸 바이트로(`cat-file --filters`).
///
/// 추적 중이 아니거나 · `HEAD` 에 없거나 · git 이 없으면 `None` — 그때는 값만 되돌린다.
/// ⚠ `--filters` 를 쓴다 — 줄바꿈 변환이 걸린 워킹트리에서 blob 바이트는 체크아웃 바이트가 아니다.
fn 커밋된_바이트(path: &Path) -> Option<Vec<u8>> {
    let dir = path.parent()?;
    let 이름 = path.file_name()?.to_str()?;
    git_묻는다(dir, &["ls-files", "--error-unmatch", "--", 이름])?;
    git_묻는다(dir, &["cat-file", "--filters", &format!("HEAD:./{이름}")])
}

/// `git -C <dir> …` 을 **시간 상한 안에서** 돌리고 성공했을 때만 표준출력을 돌려준다.
fn git_묻는다(dir: &Path, args: &[&str]) -> Option<Vec<u8>> {
    let child = Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .ok()?;
    let 대답 = super::child::기다린다(child, super::child::기본_상한, &format!("git {args:?}")).ok()?;
    대답.status.success().then_some(대답.stdout)
}

/// 직렬화한 본문을 **그 파일이 쓰던 줄바꿈에 맞춘다** — 옛 설치를 값만 되돌릴 때만 쓴다.
///
/// `serde_json::to_string_pretty` 는 언제나 LF 를 산출한다. `core.autocrlf=true` 로 클론한
/// 워킹트리에서 `settings.json` 은 CRLF 인데, 우리가 LF 로 되쓰면 **파일 전체의 모든
/// 줄이 바뀐다** — 사용자의 `git status` 에 우리 파일이 매번 뜬다.
fn 그_파일의_줄바꿈으로(path: &Path, text: &str) -> Vec<u8> {
    let 기존 = std::fs::read(path).ok();
    let crlf = super::eol::그_파일의_줄바꿈(기존.as_deref());
    super::eol::맞춘다(text.as_bytes(), crlf)
}

#[cfg(test)]
mod tests {
    use super::{Merged, Read, SettingsEntry, hooks, merge, read, unmerge, 되돌림};
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
        vec![hooks::entry("SubagentStop")]
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
            edits: Some(m.edits.clone()),
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

    /// **키도 훅도 왕복하면 사용자 바이트가 그대로 돌아온다.**
    #[test]
    fn 없던_키만_더하고_왕복하면_바이트가_돌아온다() {
        let dir = 방("왕복");
        let path = dir.join("settings.json");
        let 원본 = "{\n    \"env\": {\"A\": \"1\"}\n}";
        std::fs::write(&path, 원본).expect("쓰기");

        let r = read(&path).expect("읽기");
        let m = merge(&path, &r, &바람(), &계획(&r, &훅())).expect("병합");
        assert_eq!(m.added_keys, vec!["agent".to_owned()]);
        assert!(m.hooks_key_created, "훅 구역을 우리가 만들었는데 안 적었다");

        let 중간: Value = serde_json::from_slice(&std::fs::read(&path).expect("읽기")).expect("JSON");
        assert!(중간["hooks"]["SubagentStop"].is_array(), "훅이 안 걸렸다: {중간}");

        let 결과 = unmerge(&path, &항목(&m, &훅())).expect("되돌리기");
        assert_eq!(결과.되돌림, 되돌림::자리);
        assert_eq!(std::fs::read_to_string(&path).expect("읽기"), 원본);
    }

    /// ★ **사용자가 바꾼 `agent` 는 남는다** — 설치 뒤 사용자가 스스로 고친 것이다.
    #[test]
    fn 사용자가_바꾼_키는_남긴다() {
        let dir = 방("바꾼키");
        let path = dir.join("settings.json");
        std::fs::write(&path, "{\"env\": 1}").expect("쓰기");
        let r = read(&path).expect("읽기");
        let m = merge(&path, &r, &바람(), &계획(&r, &훅())).expect("병합");
        let 바꾼 = std::fs::read_to_string(&path).expect("읽기").replace("\"pal-orchestrator\"", "\"내 것\"");
        std::fs::write(&path, 바꾼).expect("쓰기");

        let 결과 = unmerge(&path, &항목(&m, &훅())).expect("되돌리기");
        assert_eq!(결과.남긴_키, vec!["agent".to_owned()]);
        assert_eq!(std::fs::read_to_string(&path).expect("읽기"), "{\"env\": 1, \"agent\": \"내 것\"}");
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

    /// **편집 기록이 없는 옛 항목은 값으로 되돌린다** — git 밖이라 「값만」이다.
    #[test]
    fn 편집_기록이_없으면_값만_되돌린다() {
        let dir = 방("옛항목");
        let path = dir.join("settings.json");
        std::fs::write(
            &path,
            "{\n  \"agent\": \"pal-orchestrator\",\n  \"hooks\": {\"SubagentStop\": [{\"hooks\": [{\"type\": \"command\", \"command\": \"pal\", \"args\": [\"hook\", \"SubagentStop\"]}]}]},\n  \"env\": 1\n}\n",
        )
        .expect("쓰기");
        let 옛 = SettingsEntry {
            path: Rel::new("settings.json"),
            added_keys: vec!["agent".to_owned()],
            added_values: 바람(),
            hooks: 훅(),
            hooks_key_created: true,
            created: false,
            edits: None,
        };
        let 결과 = unmerge(&path, &옛).expect("되돌리기");
        assert_eq!(결과.되돌림, 되돌림::값만);
        let 뒤: Value = serde_json::from_slice(&std::fs::read(&path).expect("읽기")).expect("JSON");
        assert_eq!(뒤, json!({"env": 1}));
    }
}
