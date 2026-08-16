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
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
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

/// 이 항목의 sha256 이 실물과 **다를 때 그것이 무엇인가.**
///
/// # ADR-0005 를 그대로 따른다
///
/// *"부재는 종류를 싣는다. 상태를 늘리는 대신 이유를 값으로 둔다."* 여기서도 칸(=
/// `doctor` 의 검사 목록)을 **안 늘린다.** 검사 2 는 그대로 하나이고, **다름의 이유**를
/// 값으로 실어 고장과 사용자 수정을 가른다.
///
/// 판별식도 그 ADR 의 것이다 — *"집계 표에서 따로 세어야 하면 칸, 같은 칸 안에서
/// 다르게 **행동**해야 하면 이유."* 여기서 갈리는 행동은 **빨강이냐 초록이냐**이고,
/// 검사의 수는 안 갈린다. 그러니 이유다.
#[derive(Serialize, Deserialize, Clone, Copy, Default, PartialEq, Eq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Origin {
    /// 우리가 놓은 바이트 그대로. **다르면 고장이다.**
    #[default]
    Ours,
    /// `update` 가 밟지 않고 지나간 자리 — **사람이 고쳤다.** 다른 것이 정상이다.
    UserModified,
}

/// 우리가 통째로 소유하는 파일 하나.
#[derive(Serialize, Deserialize, Clone)]
pub struct FileEntry {
    pub path: Rel,
    /// **설치 시점에 실물에서 뜬 값**이다 — 실물과 다르면 사람이 고친 것이고,
    /// 그 차이가 곧 `update` 의 3분기다(`[f24]` ④).
    pub sha256: String,
    /// 이 sha 가 실물과 다를 때 그것이 무엇인가. **`#[serde(default)]` 이라 옛
    /// 매니페스트는 전부 `Ours` 로 읽힌다** — 옛 기록에는 사용자 수정이 안 실려 있었고,
    /// 없던 것을 있었다고 읽지 않는다.
    #[serde(default)]
    pub origin: Origin,
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
    /// **이 매니페스트에서 유도되는 모든 경로.**
    ///
    /// ★ **경로 필드를 더하는 사람은 여기도 더해야 한다.** 안 더하면 그 경로는
    /// [`자리들`] 에 안 실리고 [`Places::자리`] 가 **실패한다** — 조용히 통과하지
    /// 않는다. 아래 `경로를_하나도_안_빠뜨린다` 가 그 빠뜨림을 시험으로 잡는다.
    #[must_use]
    pub fn 경로들(&self) -> Vec<&Rel> {
        let mut out = vec![&self.own_path];
        out.extend(&self.roots.dirs);
        out.extend(&self.roots.files);
        out.extend(self.files.iter().map(|f| &f.path));
        out.extend(self.blocks.iter().map(|b| &b.path));
        out.extend(self.settings.iter().map(|s| &s.path));
        out.extend(&self.created_dirs);
        out
    }

    /// 이 매니페스트가 적은 (경로 → sha256).
    #[must_use]
    pub fn recorded(&self) -> BTreeMap<String, String> {
        self.files.iter().map(|f| (f.path.to_string(), f.sha256.clone())).collect()
    }
}

/// 매니페스트가 가리키는 **실제 자리들.**
///
/// ★ **대상 안임이 확인된 것만 든다.** 하나라도 밖을 가리키면 여기서 통째로 실패하고,
/// 그러면 부르는 쪽은 **아직 아무것도 안 건드린 상태**다.
pub struct Places(BTreeMap<Rel, PathBuf>);

impl Places {
    /// 이 경로의 실제 자리.
    ///
    /// # Errors
    /// 등록되지 않은 경로면 — **새 필드를 더한 사람이 [`Manifest::경로들`] 을
    /// 빠뜨렸다는 뜻이다.** 조용히 통과시키지 않는다.
    pub fn 자리(&self, rel: &Rel) -> Result<&Path> {
        match self.0.get(rel) {
            Some(p) => Ok(p.as_path()),
            None => bail!(
                "`{rel}` 이 매니페스트의 경로 목록에 없다 — `Manifest::경로들` 에 \
                 안 실린 필드가 있다. 경계 검사를 못 지났으므로 건드리지 않는다"
            ),
        }
    }
}

/// 매니페스트의 **모든** 경로를 대상 안으로 확정한다.
///
/// # Errors
/// 하나라도 대상 밖을 가리키면.
pub fn 자리들(root: &Root, m: &Manifest) -> Result<Places> {
    let mut out = BTreeMap::new();
    for rel in m.경로들() {
        let path = root.join(rel)?;
        out.insert(rel.clone(), path);
    }
    Ok(Places(out))
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
        훑기(root.path(), &root.join(dir)?, &mut out)?;
    }
    for file in &roots.files {
        let path = root.join(file)?;
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
    /// 있는데 sha 가 다른 것 — (경로, 적힌 값, 실물 값). **고장이다.**
    pub changed: Vec<(String, String, String)>,
    /// 다른데 **사람이 고친 것**이다. 고장이 아니다 — `update` 가 안 밟고 지나갔다.
    pub user_modified: Vec<String>,
}

impl Diff {
    #[must_use]
    pub fn is_clean(&self) -> bool {
        self.missing.is_empty() && self.unrecorded.is_empty() && self.changed.is_empty()
    }
}

/// 매니페스트와 실물을 **양방향**으로 뺀다.
///
/// **항목을 통째로 받는다** — sha 만 받으면 다름의 **종류**가 여기 못 온다.
#[must_use]
pub fn diff(recorded: &[FileEntry], actual: &BTreeMap<String, String>) -> Diff {
    let mut missing = Vec::new();
    let mut changed = Vec::new();
    let mut user_modified = Vec::new();
    for entry in recorded {
        let path = entry.path.to_string();
        match actual.get(&path) {
            None => missing.push(path),
            Some(there) if *there != entry.sha256 => match entry.origin {
                Origin::Ours => changed.push((path, entry.sha256.clone(), there.clone())),
                Origin::UserModified => user_modified.push(path),
            },
            Some(_) => {}
        }
    }
    let 적힌: std::collections::BTreeSet<String> =
        recorded.iter().map(|e| e.path.to_string()).collect();
    let unrecorded = actual.keys().filter(|p| !적힌.contains(*p)).cloned().collect::<Vec<_>>();
    Diff { missing, unrecorded, changed, user_modified }
}

#[cfg(test)]
mod tests {
    use super::{BlockEntry, FileEntry, Manifest, Origin, Rel, Roots, SettingsEntry, diff};
    use std::collections::BTreeMap;

    /// 경로 필드에만 붙이는 표식 — 아래 시험이 이것으로 샌 필드를 찾는다.
    const 표식: &str = "경계시험/";

    /// ★ **경로 필드를 더하고 [`Manifest::경로들`] 을 안 고치면 여기서 걸린다.**
    ///
    /// 아래 리터럴은 `..` 를 안 쓴다 — 필드를 더하면 **컴파일이 먼저 깨지고**, 그때
    /// 이웃을 따라 `표식` 이 붙은 값을 넣으면 이 시험이 빠뜨림을 잡는다.
    ///
    /// ⚠ 한계: 새 필드에 표식이 안 붙은 값을 넣으면 못 잡는다. 그 자리를 메우는 것은
    /// **`Rel` 타입 자체**다 — `Root::join` 말고는 파일시스템 경로가 되는 길이 없다.
    #[test]
    fn 경로를_하나도_안_빠뜨린다() {
        let m = Manifest {
            pal_version: "0.0.0".to_owned(),
            roots: Roots {
                dirs: vec![Rel::new(&format!("{표식}뿌리디렉터리"))],
                files: vec![Rel::new(&format!("{표식}뿌리파일"))],
            },
            own_path: Rel::new(&format!("{표식}자기자신")),
            files: vec![FileEntry {
                path: Rel::new(&format!("{표식}파일")),
                sha256: "0".repeat(64),
                origin: Origin::Ours,
            }],
            blocks: vec![BlockEntry {
                path: Rel::new(&format!("{표식}블록")),
                inserted: "x".to_owned(),
                created: false,
            }],
            settings: Some(SettingsEntry {
                path: Rel::new(&format!("{표식}설정")),
                added_keys: Vec::new(),
                hooks: Vec::new(),
                hooks_key_created: false,
                created: false,
            }),
            created_dirs: vec![Rel::new(&format!("{표식}만든디렉터리"))],
        };

        let 적힌: std::collections::BTreeSet<String> =
            m.경로들().into_iter().map(ToString::to_string).collect();
        let mut 실린 = std::collections::BTreeSet::new();
        표식_모으기(&serde_json::to_value(&m).expect("직렬화"), &mut 실린);

        assert!(!실린.is_empty(), "이 시험이 아무것도 안 세고 있다");
        assert_eq!(실린, 적힌, "직렬화에는 있는데 `경로들` 이 안 내는 경로가 있다");
    }

    fn 표식_모으기(v: &serde_json::Value, out: &mut std::collections::BTreeSet<String>) {
        match v {
            serde_json::Value::String(s) if s.starts_with(표식) => {
                out.insert(s.clone());
            }
            serde_json::Value::Array(a) => a.iter().for_each(|x| 표식_모으기(x, out)),
            serde_json::Value::Object(o) => o.values().for_each(|x| 표식_모으기(x, out)),
            _ => {}
        }
    }

    #[test]
    fn 양쪽이_같으면_차집합이_비어_있다() {
        let recorded = 적힌(&[("x", "1"), ("y", "2")]);
        let actual = 실험용(&[("x", "1"), ("y", "2")]);
        assert!(diff(&recorded, &actual).is_clean());
    }

    /// **적혔는데 없는 것**과 **생겼는데 안 적힌 것**을 갈라 센다.
    #[test]
    fn 양방향을_갈라_센다() {
        let recorded = 적힌(&[("x", "1")]);
        let actual = 실험용(&[("y", "2")]);
        let d = diff(&recorded, &actual);
        assert_eq!(d.missing, vec!["x".to_owned()]);
        assert_eq!(d.unrecorded, vec!["y".to_owned()]);
        assert!(!d.is_clean());
    }

    /// ★ **다음에 생기는 파일** — 이름으로 세면 여기가 안 걸린다.
    #[test]
    fn 나중에_생긴_파일이_걸린다() {
        let recorded = 적힌(&[("x", "1")]);
        let mut actual = 실험용(&[("x", "1")]);
        actual.insert("나중것".to_owned(), "9".to_owned());
        assert!(!diff(&recorded, &actual).is_clean());
    }

    #[test]
    fn sha_가_다르면_바뀐_것으로_센다() {
        let recorded = 적힌(&[("x", "1")]);
        let actual = 실험용(&[("x", "2")]);
        let d = diff(&recorded, &actual);
        assert_eq!(d.changed.len(), 1);
        assert!(d.user_modified.is_empty());
    }

    /// ★ **같은 「다름」인데 종류가 갈린다** — 고장은 빨갛고 사용자 수정은 아니다.
    #[test]
    fn 사용자_수정은_고장으로_안_센다() {
        let mut recorded = 적힌(&[("x", "1")]);
        recorded[0].origin = Origin::UserModified;
        let d = diff(&recorded, &실험용(&[("x", "2")]));
        assert!(d.changed.is_empty(), "사용자 수정을 고장으로 셌다");
        assert_eq!(d.user_modified, vec!["x".to_owned()]);
        assert!(d.is_clean());
    }

    /// **사용자 수정이라도 사라지면 그것은 고장이다.**
    #[test]
    fn 사용자_수정이라도_없으면_걸린다() {
        let mut recorded = 적힌(&[("x", "1")]);
        recorded[0].origin = Origin::UserModified;
        assert!(!diff(&recorded, &실험용(&[])).is_clean());
    }

    fn 적힌(pairs: &[(&str, &str)]) -> Vec<FileEntry> {
        pairs
            .iter()
            .map(|(a, b)| FileEntry {
                path: Rel::new(a),
                sha256: (*b).to_owned(),
                origin: Origin::Ours,
            })
            .collect()
    }

    fn 실험용(pairs: &[(&str, &str)]) -> BTreeMap<String, String> {
        pairs.iter().map(|(a, b)| ((*a).to_owned(), (*b).to_owned())).collect()
    }
}
