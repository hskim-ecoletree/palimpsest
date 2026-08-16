//! 매니페스트 — **무엇을 놓았고 무엇을 되돌릴 수 있는가**(`[f24]` ③⑥).
//!
//! # ★ 이름으로 세지 않는다
//!
//! F04 게이트가 같은 자리에서 적었다 — *"`intent.redb` 를 이름으로 적으면 다음에
//! 생기는 파일이 빠지고, **낡은 검사는 통과한다**."* 그래서 매니페스트는 **파일 이름
//! 목록이 아니라 뿌리 목록**을 함께 지고, 대조는 그 뿌리를 **재귀로 훑어서** 집합을
//! 뜬다([`walk`]). 새 리소스가 생기면 목록을 안 고쳐도 실물 쪽 집합이 늘고, 그러면
//! 양방향 차집합이 0 이 아니게 되어 **검사가 걸린다.**
//!
//! # 뿌리가 둘로 갈리는 이유
//!
//! `.claude/pal/` 과 `.claude/commands/pal/` 은 통째로 우리 것이라 **디렉터리째** 훑는다.
//! `.claude/agents/` 는 **남의 에이전트가 함께 사는 곳**이라 훑으면 남의 것을 우리 것으로
//! 센다 — 그래서 그쪽만 **파일 하나짜리 뿌리**다.

use std::collections::BTreeMap;
use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use super::inside::{Rel, Root};
use super::sha256;

/// 훑을 뿌리.
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Roots {
    /// 통째로 우리 것인 디렉터리 — **재귀로 훑는다.**
    pub dirs: Vec<Rel>,
    /// 남의 것이 함께 사는 곳의 파일 하나.
    pub files: Vec<Rel>,
}

/// 우리가 통째로 소유하는 파일 하나.
#[derive(Serialize, Deserialize, Clone)]
pub struct FileEntry {
    pub path: Rel,
    /// **설치 시점에 실물에서 뜬 값**이다 — 실물과 다르면 사람이 고친 것이고,
    /// 그 차이가 곧 `update` 의 3분기다(`[f24]` ④).
    pub sha256: String,
}

/// 남의 파일에 더한 블록 하나.
#[derive(Serialize, Deserialize, Clone)]
pub struct BlockEntry {
    pub path: Rel,
    /// **우리가 넣은 바이트열 그대로.** 제거는 이것과 정확히 일치할 때만 한다.
    pub inserted: String,
    /// 그 파일을 우리가 만들었는가.
    pub created: bool,
}

/// 설정 파일에 등록한 훅 하나.
///
/// **`command` 는 등록 문자열 원문이다.** 실측: 하네스의 중복 제거는 이 문자열의
/// **완전 일치** 기준이라 공백 하나만 달라도 두 번 돈다 — 그래서 제거도 완전 일치로만
/// 하고, 그러려면 우리가 넣은 바이트를 그대로 지고 있어야 한다(블록의 `inserted` 와
/// 같은 판단).
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct HookEntry {
    pub event: String,
    pub command: String,
}

/// 설정 파일에 더한 키들.
#[derive(Serialize, Deserialize, Clone)]
pub struct SettingsEntry {
    pub path: Rel,
    pub added_keys: Vec<String>,
    /// 우리가 등록한 훅. **`#[serde(default)]` 이라 옛 매니페스트도 읽힌다** — 못 읽으면
    /// *"무엇을 되돌려야 하는가"* 의 기록이 통째로 사라진다.
    #[serde(default)]
    pub hooks: Vec<HookEntry>,
    /// `hooks` 최상위 키를 **우리가** 만들었는가. 사용자가 만든 것은 비어도 안 지운다.
    #[serde(default)]
    pub hooks_key_created: bool,
    pub created: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Manifest {
    /// 이것을 놓은 `pal` 의 버전. **`pal --version` 의 출력과 같은 문자열이다.**
    pub pal_version: String,
    pub roots: Roots,
    /// 매니페스트 자신의 자리 — 대조는 이 경로 하나만 뺀다.
    ///
    /// **JSON 의 키 이름은 `manifest_path` 로 남긴다** — 이 파일은 우리 밖에서도
    /// 읽히고, 밖에서 읽는 이름을 안쪽 사정으로 바꾸지 않는다.
    #[serde(rename = "manifest_path")]
    pub own_path: Rel,
    pub files: Vec<FileEntry>,
    pub blocks: Vec<BlockEntry>,
    pub settings: Option<SettingsEntry>,
    /// **우리가 만든** 디렉터리. 만든 순서대로 들어 있고 제거는 역순으로 본다.
    pub created_dirs: Vec<Rel>,
}

impl Manifest {
    /// 이 매니페스트가 적은 (경로 → sha256).
    #[must_use]
    pub fn recorded(&self) -> BTreeMap<String, String> {
        self.files.iter().map(|f| (f.path.to_string(), f.sha256.clone())).collect()
    }
}

/// 매니페스트를 읽는다.
///
/// # Errors
/// 없거나, 못 읽거나, JSON 이 아니면. **손상된 매니페스트를 조용히 덮지 않는다** —
/// 그것을 덮으면 *"무엇을 되돌려야 하는가"* 의 기록이 사라진다.
pub fn read(path: &Path) -> Result<Manifest> {
    let bytes = std::fs::read(path)
        .with_context(|| format!("매니페스트를 읽지 못했다: {}", path.display()))?;
    serde_json::from_slice(&bytes)
        .with_context(|| format!("매니페스트가 JSON 이 아니다: {}", path.display()))
}

/// 매니페스트를 쓴다.
///
/// # Errors
/// 쓰지 못하면.
pub fn write(path: &Path, manifest: &Manifest) -> Result<()> {
    let mut text = serde_json::to_string_pretty(manifest).context("매니페스트 직렬화")?;
    text.push('\n');
    std::fs::write(path, text.as_bytes())
        .with_context(|| format!("매니페스트를 쓰지 못했다: {}", path.display()))
}

/// 실물 — **뿌리를 훑어서 뜬 (상대 경로 → sha256).**
///
/// 매니페스트 자신은 뺀다. 자기 sha 를 자기 안에 적을 수 없기 때문이고, **뺄 경로를
/// 매니페스트가 스스로 선언하므로 검사가 이름을 손에 쥐지 않는다.**
///
/// # Errors
/// 파일을 읽지 못하면.
pub fn walk(root: &Root, roots: &Roots, skip: &Rel) -> Result<BTreeMap<String, String>> {
    let mut out = BTreeMap::new();
    for dir in &roots.dirs {
        훑기(root.path(), &root.join(dir), &mut out)?;
    }
    for file in &roots.files {
        let path = root.join(file);
        if path.is_file() {
            out.insert(file.to_string(), sha256::hex(&std::fs::read(&path)?));
        }
    }
    out.remove(skip.as_str());
    Ok(out)
}

fn 훑기(target: &Path, dir: &Path, out: &mut BTreeMap<String, String>) -> Result<()> {
    let Ok(entries) = std::fs::read_dir(dir) else { return Ok(()) };
    for entry in entries {
        let path = entry?.path();
        if path.is_dir() {
            훑기(target, &path, out)?;
        } else if path.is_file() {
            let rel = path
                .strip_prefix(target)
                .unwrap_or(&path)
                .to_string_lossy()
                .replace('\\', "/");
            let bytes = std::fs::read(&path)
                .with_context(|| format!("읽지 못했다: {}", path.display()))?;
            out.insert(rel, sha256::hex(&bytes));
        }
    }
    Ok(())
}

/// 양방향 차집합 — **둘 다 0 이어야 한다**(`[f24]` ③).
pub struct Diff {
    /// 적혔는데 없는 것.
    pub missing: Vec<String>,
    /// 생겼는데 안 적힌 것.
    pub unrecorded: Vec<String>,
    /// 있는데 sha 가 다른 것 — (경로, 적힌 값, 실물 값).
    pub changed: Vec<(String, String, String)>,
}

impl Diff {
    #[must_use]
    pub fn is_clean(&self) -> bool {
        self.missing.is_empty() && self.unrecorded.is_empty() && self.changed.is_empty()
    }
}

/// 매니페스트와 실물을 **양방향**으로 뺀다.
#[must_use]
pub fn diff(recorded: &BTreeMap<String, String>, actual: &BTreeMap<String, String>) -> Diff {
    let mut missing = Vec::new();
    let mut changed = Vec::new();
    for (path, sha) in recorded {
        match actual.get(path) {
            None => missing.push(path.clone()),
            Some(there) if there != sha => changed.push((path.clone(), sha.clone(), there.clone())),
            Some(_) => {}
        }
    }
    let unrecorded =
        actual.keys().filter(|p| !recorded.contains_key(*p)).cloned().collect::<Vec<_>>();
    Diff { missing, unrecorded, changed }
}

#[cfg(test)]
mod tests {
    use super::diff;
    use std::collections::BTreeMap;

    #[test]
    fn 양쪽이_같으면_차집합이_비어_있다() {
        let a = 실험용(&[("x", "1"), ("y", "2")]);
        assert!(diff(&a, &a).is_clean());
    }

    /// **적혔는데 없는 것**과 **생겼는데 안 적힌 것**을 갈라 센다.
    #[test]
    fn 양방향을_갈라_센다() {
        let recorded = 실험용(&[("x", "1")]);
        let actual = 실험용(&[("y", "2")]);
        let d = diff(&recorded, &actual);
        assert_eq!(d.missing, vec!["x".to_owned()]);
        assert_eq!(d.unrecorded, vec!["y".to_owned()]);
        assert!(!d.is_clean());
    }

    /// ★ **다음에 생기는 파일** — 이름으로 세면 여기가 안 걸린다.
    #[test]
    fn 나중에_생긴_파일이_걸린다() {
        let recorded = 실험용(&[("x", "1")]);
        let mut actual = recorded.clone();
        actual.insert("나중것".to_owned(), "9".to_owned());
        assert!(!diff(&recorded, &actual).is_clean());
    }

    #[test]
    fn sha_가_다르면_바뀐_것으로_센다() {
        let recorded = 실험용(&[("x", "1")]);
        let actual = 실험용(&[("x", "2")]);
        let d = diff(&recorded, &actual);
        assert_eq!(d.changed.len(), 1);
    }

    fn 실험용(pairs: &[(&str, &str)]) -> BTreeMap<String, String> {
        pairs.iter().map(|(a, b)| ((*a).to_owned(), (*b).to_owned())).collect()
    }
}
