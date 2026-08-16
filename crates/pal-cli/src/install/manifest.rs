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

/// 매니페스트가 지는 경로의 **종류.**
///
/// 경로마다 **우리가 그 자리에 무엇을 할 수 있는지**가 다르다 — 파일은 지우고,
/// 디렉터리는 비었을 때만 지우고, 블록은 남의 파일 안의 바이트열만 뺀다. 그러니
/// 「대상 안인가」 하나로는 부족하고, **어느 종류로 적혔는가**가 함께 다녀야 한다.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum 종류 {
    /// 우리가 통째로 소유하는 파일 하나.
    파일,
    /// 우리가 만들 수 있는 디렉터리 하나.
    디렉터리,
    /// 남의 파일 — 우리가 넣은 블록만 뺀다.
    블록,
    /// 대상 설정 파일 — 우리가 더한 키만 뺀다.
    설정,
}

impl Manifest {
    /// **이 매니페스트에서 유도되는 모든 경로**와 그 종류.
    ///
    /// ★ **경로 필드를 더하는 사람은 여기도 더해야 한다.** 안 더하면 그 경로는
    /// [`자리들`] 에 안 실리고 [`Places::자리`] 가 **실패한다** — 조용히 통과하지
    /// 않는다. 아래 `경로를_하나도_안_빠뜨린다` 가 그 빠뜨림을 시험으로 잡는다.
    #[must_use]
    pub fn 경로들(&self) -> Vec<(종류, &Rel)> {
        let mut out = vec![(종류::파일, &self.own_path)];
        out.extend(self.roots.dirs.iter().map(|r| (종류::디렉터리, r)));
        out.extend(self.roots.files.iter().map(|r| (종류::파일, r)));
        out.extend(self.files.iter().map(|f| (종류::파일, &f.path)));
        out.extend(self.blocks.iter().map(|b| (종류::블록, &b.path)));
        out.extend(self.settings.iter().map(|s| (종류::설정, &s.path)));
        out.extend(self.created_dirs.iter().map(|r| (종류::디렉터리, r)));
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
/// **문이 둘이다** — *"대상 안인가"*([`Root::join`])와 *"우리가 놓을 수 있는
/// 자리인가"*([`우리_자리인가`]). 앞의 것만으로는 **대상 안의 남의 파일**을 못
/// 가른다.
///
/// # Errors
/// 하나라도 대상 밖을 가리키거나, **우리가 놓을 수 없는 자리를 가리키면.**
pub fn 자리들(root: &Root, m: &Manifest) -> Result<Places> {
    let mut out = BTreeMap::new();
    for (종류, rel) in m.경로들() {
        let path = root.join(rel)?;
        우리_자리인가(종류, rel)?;
        out.insert(rel.clone(), path);
    }
    Ok(Places(out))
}

/// ★ **되돌릴 수 있는 것은 놓을 수 있는 자리뿐이다.**
///
/// 상한을 **매니페스트가 아니라 컴파일된 상수**([`super::layout`])로 잡는다.
/// 매니페스트의 `roots` 를 상한으로 쓰면 그것도 남이 쓴 값이라 상한이 아니다.
///
/// # Errors
/// 그 종류로 우리가 만들 수 없는 자리면.
pub fn 우리_자리인가(종류: 종류, rel: &Rel) -> Result<()> {
    use super::layout;

    let s = rel.as_str();
    if layout::절대_안_건드리나(s) {
        bail!(
            "`{rel}` 은 **어떤 경우에도 안 건드린다** — 매니페스트가 그것을 적었다는 \
             사실 자체가 이 매니페스트를 사람이 봐야 한다는 뜻이다.\n    \
             매니페스트는 대상 프로젝트 안에 사는 파일이고 커밋과 함께 이동한다"
        );
    }
    let 된다 = match 종류 {
        self::종류::파일 => layout::놓을_수_있는_파일인가(s),
        self::종류::디렉터리 => layout::만들_수_있는_디렉터리인가(s),
        self::종류::블록 => layout::블록을_넣을_수_있는_파일인가(s),
        self::종류::설정 => layout::설정_파일인가(s),
    };
    if !된다 {
        bail!(
            "`{rel}` 은 **우리가 놓을 수 있는 자리가 아니다**({종류:?} 로 적혔다) — \
             건드리지 않는다.\n    \
             매니페스트는 대상 프로젝트 안에 사는 평범한 파일이라 남이 커밋해 보낼 수 \
             있다. **되돌릴 수 있는 것은 놓을 수 있는 자리뿐이고**, 그 상한은 \
             매니페스트가 아니라 이 바이너리가 진다. 사람이 봐야 한다"
        );
    }
    Ok(())
}

/// 매니페스트를 읽는다.
///
/// # Errors
/// 없거나, 못 읽거나, JSON 이 아니면. **손상된 매니페스트를 조용히 덮지 않는다** —
/// 그것을 덮으면 *"무엇을 되돌려야 하는가"* 의 기록이 사라진다.
pub fn read(path: &Path) -> Result<Manifest> {
    let bytes = super::guard::읽는다(path)
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
    super::guard::쓴다(path, text.as_bytes())
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
            m.경로들().into_iter().map(|(_, r)| r.to_string()).collect();
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

    /// ★ **우리가 실제로 놓는 것은 전부 통과한다.**
    ///
    /// 이 시험이 없으면 상한이 조여질 때 **설치가 조용히 자기 것을 못 되돌리게** 된다.
    /// 리소스를 더하는 사람은 여기서 먼저 걸린다.
    #[test]
    fn 우리가_놓는_자리는_전부_통과한다() {
        use super::super::layout;
        use super::우리_자리인가;

        for res in layout::PAYLOAD {
            우리_자리인가(super::종류::파일, &Rel::new(res.path))
                .unwrap_or_else(|e| panic!("{}: {e}", res.path));
        }
        for r in [layout::MANIFEST].iter().chain(layout::OWNED_FILES) {
            우리_자리인가(super::종류::파일, &Rel::new(r)).unwrap_or_else(|e| panic!("{r}: {e}"));
        }
        for d in layout::DIRS.iter().chain(layout::OWNED_DIRS) {
            우리_자리인가(super::종류::디렉터리, &Rel::new(d))
                .unwrap_or_else(|e| panic!("{d}: {e}"));
        }
        for b in [layout::ROOT_INSTRUCTION_FILE, layout::IGNORE_FILE] {
            우리_자리인가(super::종류::블록, &Rel::new(b)).unwrap_or_else(|e| panic!("{b}: {e}"));
        }
        우리_자리인가(super::종류::설정, &Rel::new(layout::SETTINGS)).expect("설정");
    }

    /// ★ **대상 안이어도 우리 자리가 아니면 막힌다.** `Root::join` 은 여기를 못 본다.
    #[test]
    fn 대상_안의_남의_자리는_막힌다() {
        use super::우리_자리인가;

        for 남의것 in [".git/config", ".git", "README.md", "src/main.rs", ".claude/settings.json"] {
            assert!(
                우리_자리인가(super::종류::파일, &Rel::new(남의것)).is_err(),
                "`{남의것}` 을 우리 파일로 읽었다"
            );
        }
        for 남의것 in [".git", "src", ".claude/agents/남의것.md"] {
            assert!(
                우리_자리인가(super::종류::디렉터리, &Rel::new(남의것)).is_err(),
                "`{남의것}` 을 우리 디렉터리로 읽었다"
            );
        }
        for 남의것 in ["README.md", ".git/config", ".claude/settings.json"] {
            assert!(
                우리_자리인가(super::종류::블록, &Rel::new(남의것)).is_err(),
                "`{남의것}` 을 우리 블록 자리로 읽었다"
            );
        }
        assert!(우리_자리인가(super::종류::설정, &Rel::new("CLAUDE.md")).is_err());
    }

    /// **`.git/` 은 어떤 종류로 적혀도 막힌다** — 목록의 부수효과가 아니라 못박은 줄이다.
    #[test]
    fn git_디렉터리는_어떤_종류로도_막힌다() {
        use super::우리_자리인가;

        for 종류 in
            [super::종류::파일, super::종류::디렉터리, super::종류::블록, super::종류::설정]
        {
            for rel in [".git", ".git/config", ".git/hooks/pre-commit"] {
                assert!(우리_자리인가(종류, &Rel::new(rel)).is_err(), "{종류:?} {rel}");
            }
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
