//! `pal install` · `pal update` · `pal uninstall` — **남의 프로젝트에 놓고 갱신하고
//! 걷어낸다.** 그리고 **어느 경로에서도 그 프로젝트 바깥을 안 건드린다**(`[f24]` ⑦).
//!
//! # 이 파일에 없는 것이 이 파일의 절반이다
//!
//! `home_dir` · `$HOME` 읽기 · `~` 전개 · `dirs::` 계열이 **하나도 없다.** 그것을
//! `cargo xtask check` 의 열여섯째 검사가 센다. ⚠ 그러나 **문자열 스캔만으로는
//! 부족하다** — F04 가 이미 적었듯 *"낱말 없이도 상위 디렉터리를 지울 수 있고 `..`
//! 하나면 경계가 사라진다."* **실물 하중은 격리 HOME 스냅샷 시험이 진다**
//! (`tests/install_stays_inside.rs`).
//!
//! # 두 단계다 — **검증이 끝나기 전에는 한 바이트도 안 쓴다**
//!
//! 게이트 ② 가 재는 것이 그것이다: 대상 `settings.json` 이 안 읽히면 **부분 설치가
//! 남으면 안 된다.** 깨진 설정은 하네스의 `-p` 에서 **완전히 침묵하므로**(실측)
//! 반쯤 오염된 프로젝트는 아무도 모른다.
//!
//! # 이 회차가 안 만드는 것
//!
//! **훅을 등록하지 않는다.** 훅 규약 측정이 아직 서지 않았고(`[f24]` ⑧ 이 *"형태를
//! 합격선에 안 박는다"* 로 남긴 자리), 지금 등록하면 **측정이 답을 정하는 것이 아니라
//! 등록이 답을 정한다.**

mod blocks;
mod child;
mod doctor;
mod guard;
mod hooks;
mod ignore;
mod inside;
mod layout;
mod manifest;
mod settings;
mod sha256;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use serde_json::{Value, json};

pub use doctor::{Check, checks, print};

use inside::{Rel, Root};
use layout::{
    AGENT_KEY, AGENT_VALUE, CLAUDE_DIR, DERIVED, DIRS, HOOK_EVENTS, IGNORE_FILE, IGNORE_MARKERS,
    IMPORT_LINE, LOCK, MANIFEST, MANIFEST_HOME, MD_MARKERS, OWNED_DIRS, OWNED_FILES, PAYLOAD,
    ROOT_INSTRUCTION_FILE, SETTINGS,
};
use manifest::{BlockEntry, FileEntry, Manifest, Origin, Roots, SettingsEntry, 자리들};

/// 잠금을 기다리는 시간(밀리초)과 간격.
///
/// **없으면 동시 설치 8회가 블록 8개를 만든다**(실측 · check-then-act 경쟁).
const LOCK_WAIT_MS: u64 = 20_000;
const LOCK_POLL_MS: u64 = 25;

// ─────────────────────────────────────────────────────────────────────────────
// 잠금 — 파일 하나에 건 **권고 잠금**. 커널이 소유를 판정한다
// ─────────────────────────────────────────────────────────────────────────────
//
// # ★ 「누가 쥐고 있는가」를 어떻게 식별하기로 정했는가
//
// 앞 회차가 *"잠금 소유를 어떻게 식별할지는 설계 결정이라 안 정했다"* 로 남긴 자리다.
// **정했다: 커널의 권고 파일 잠금([`std::fs::File::try_lock`])이 소유를 진다.**
//
// | 후보 | 왜 안 골랐나 |
// |---|---|
// | 디렉터리(옛것) | `create_dir` 이 원자적이라 배타는 되는데 **죽음을 못 본다.** `SIGKILL` 8지점 전부에서 잔해가 남았다 |
// | 잠금 안에 **PID** 를 적는다 | PID 는 **재사용된다.** 「그 PID 가 살아 있다」와 「그 잠금의 주인이 살아 있다」가 다르고, 시작 시각까지 실어 좁히려면 플랫폼마다 다른 문을 열어야 한다 |
// | 만료 시각(임대) | 시계에 기댄다. 느린 디스크에서 **산 잠금을 걷는다** |
// | **권고 파일 잠금** ← 고른 것 | 판정을 **커널이 한다.** 프로세스가 어떻게 죽든(`SIGKILL` 포함) fd 가 닫히며 잠금이 풀린다. 우리가 「죽었나」를 추측할 자리가 없다 |
//
// 마지막 줄이 이 결정의 전부다 — **죽음의 판정을 우리가 하지 않는다.**
//
// ⚠ **이식성**: `File::try_lock` 은 유닉스(`flock`)와 Windows(`LockFileEx`) 양쪽에
// 있다. 이 자리에 `#[cfg(unix)]` 가 하나도 없는 것은 그래서다 — 아래 **동일성 재확인**
// 하나만 유닉스 전용이고, 그 사실을 [`Lock::같은_자리인가`] 가 적고 있다.

struct Lock {
    path: PathBuf,
    file: std::fs::File,
}

impl Lock {
    fn take(root: &Root) -> Result<Self> {
        let path = root.join(&Rel::new(LOCK))?;
        // **옛 빌드가 남긴 디렉터리 잔해**를 먼저 치운다. 비어 있을 때만 지워지고,
        // 안 비었으면 아래 열기가 실패해 사람에게 넘어간다.
        if path.is_dir() {
            let _ = std::fs::remove_dir(&path);
        }
        // ★ **여는 자리는 전부 종류를 먼저 묻는다 — 여기도.** 잠금은 `쓸_수_있나` 의
        // 목록에 없었고([`쓸_수_있나`] 가 이제 든다) 여는 자리에도 문이 없었다.
        // 그래서 이 경로가 FIFO 면 아래 `open` 이 **영원히** 매달렸고, 잠금은
        // `install`·`update`·`uninstall` **셋 다** 지나는 자리다.
        guard::일반_파일이거나_없나(&path)?;
        let mut waited = 0;
        loop {
            let file = std::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(false)
                .open(&path)
                .with_context(|| format!("잠금을 열지 못했다: {}", path.display()))?;
            match file.try_lock() {
                Ok(()) => {
                    // ★ **잡고 나서 그 자리가 아직 그 파일인지 다시 본다.** 앞 주인이
                    // 놓으면서 파일을 지우고 다른 회차가 새로 만들었으면 우리가 쥔 것은
                    // **이미 이름 없는 옛 inode** 이고, 그러면 둘이 동시에 들어온다.
                    if Self::같은_자리인가(&file, &path) {
                        return Ok(Self { path, file });
                    }
                    continue;
                }
                Err(std::fs::TryLockError::WouldBlock) => {
                    if waited >= LOCK_WAIT_MS {
                        bail!(
                            "다른 `pal` 이 {} 를 쥐고 있다 — **살아 있는 프로세스다**(커널이 \
                             그렇게 답했다). 그것이 끝나기를 기다리십시오.\n    \
                             죽은 잠금은 이 명령이 스스로 걷으므로 손으로 지울 일이 없다",
                            path.display()
                        );
                    }
                    std::thread::sleep(std::time::Duration::from_millis(LOCK_POLL_MS));
                    waited += LOCK_POLL_MS;
                }
                Err(std::fs::TryLockError::Error(e)) => {
                    return Err(e)
                        .with_context(|| format!("잠금을 잡지 못했다: {}", path.display()));
                }
            }
        }
    }

    /// 우리가 쥔 파일이 **아직 그 경로에 앉아 있는가.**
    ///
    /// ⚠ **유닉스 전용 가정**(소유자 결정 2026-08-16 · windows 대응 가정): 장치·inode
    /// 번호로 댄다. Windows 에는 std 로 여는 등가 문이 없고 — 그쪽은 열려 있는 파일을
    /// 지우는 것 자체가 기본적으로 막혀 이 경합이 거의 안 난다 — 그래서 그 플랫폼에서는
    /// **재확인 없이 통과**시킨다. 여기가 그 사실을 적는 자리다.
    fn 같은_자리인가(file: &std::fs::File, path: &Path) -> bool {
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            let (Ok(a), Ok(b)) = (file.metadata(), std::fs::metadata(path)) else {
                return false;
            };
            return a.dev() == b.dev() && a.ino() == b.ino();
        }
        #[cfg(not(unix))]
        {
            let _ = (file, path);
            true
        }
    }
}

impl Drop for Lock {
    /// **잠금 파일까지 치운다** — `.claude/` 를 비우려면 이것이 남으면 안 되고,
    /// 남기면 사용자의 `git status` 에 우리 잔해가 매번 뜬다.
    ///
    /// 지우는 것과 놓는 것의 순서가 중요하다: **지운 뒤에 놓는다.** 반대로 하면 놓는
    /// 순간부터 지우기 전까지 남이 그 inode 를 잡고, 그 뒤 우리가 지워서 **그 남이
    /// 이름 없는 잠금을 쥔 채로 돌게** 된다. 그래도 남는 틈은
    /// [`Lock::같은_자리인가`] 가 받는다.
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
        // fd 가 닫히면 커널이 푼다. 명시적으로 한 번 더 부르는 것은 **의도를 적기
        // 위해서**이고, 실패해도 할 수 있는 것이 없다.
        let _ = self.file.unlock();
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 보고 — **밟지 않는 것과 말하지 않는 것은 다르다**(`[f24]` ④)
// ─────────────────────────────────────────────────────────────────────────────

/// 사용자 수정을 건너뛴 줄의 **정확한 낱말**. 게이트 ④ 가 보고에서 이것을 찾는다.
const SKIPPED: &str = "사용자 수정 — 건너뜀";

/// 제거가 사용자 수정을 **지우면서 말하는** 줄. 갱신은 지키고 제거는 지운다 —
/// 그 차이를 사용자가 화면에서 봐야 한다.
const 지운_사용자_수정: &str = "사용자 수정 — 지웠다";

struct Report {
    lines: Vec<String>,
}

impl Report {
    fn new() -> Self {
        Self { lines: Vec::new() }
    }
    fn say(&mut self, tag: &str, what: &str) {
        self.lines.push(format!("  {tag:<22}{what}"));
    }
    fn print(&self, head: &str) {
        println!();
        println!("■ {head}");
        for line in &self.lines {
            println!("{line}");
        }
        println!();
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 설치
// ─────────────────────────────────────────────────────────────────────────────

/// 대상 프로젝트에 놓는다.
///
/// # Errors
/// 대상이 없거나, 설정이 안 읽히거나, 쓰지 못하면.
pub fn install(target: &Path) -> Result<()> {
    let root = Root::세운다(target)?;

    // ── 1단계 · 검증. **여기까지 한 바이트도 안 쓴다** ──────────────────────
    let settings_path = root.join(&Rel::new(SETTINGS))?;
    let manifest_path = root.join(&Rel::new(MANIFEST))?;
    settings::read(&settings_path)?;
    쓸_수_있나(&root)?;

    // ── 2단계 · 잠금 ────────────────────────────────────────────────────────
    //
    // ★ **이전 상태를 읽는 것부터 매니페스트를 쓰는 것까지가 이 안에 있다.** 밖에서
    // 읽으면 경쟁 프로세스가 전부 「이전 = 없음」을 보고, 마지막 회차가 `blocks: []` ·
    // `settings: null` 인 매니페스트를 쓴다 — 그러면 제거가 **거짓 성공**한다.
    //
    // 잠금은 `.claude/` 안에 사니까 그것만 먼저 세운다. **없던 것만 적는다**: 있던
    // 디렉터리를 「우리가 만들었다」고 적으면 제거가 남의 자리를 노린다.
    let claude_rel = Rel::new(CLAUDE_DIR);
    let claude = root.join(&claude_rel)?;
    let 우리가_만든다 = !claude.is_dir();
    std::fs::create_dir_all(&claude)
        .with_context(|| format!("만들지 못했다: {}", claude.display()))?;
    let _lock = Lock::take(&root)?;

    let 이전 = if manifest_path.exists() { Some(manifest::read(&manifest_path)?) } else { None };
    // 잠금 밖에서 읽은 설정은 이미 남이 바꿨을 수 있다 — **안에서 다시 읽는다.**
    let read = settings::read(&settings_path)?;
    let mut created_dirs = 이전.as_ref().map(|m| m.created_dirs.clone()).unwrap_or_default();
    if 우리가_만든다 && !created_dirs.contains(&claude_rel) {
        created_dirs.push(claude_rel);
    }

    // ── 3단계 · 적용. **기록이 걸음마다 앞선다** ────────────────────────────
    //
    // ★ **기록이 살 자리를 먼저 세우고 그 다음 아무것도 안 한다.** 디렉터리 다섯을 다
    // 세우고 적으면 그 사이에 죽었을 때 **기록 없는 빈 디렉터리 다섯**이 남는다.
    // 여기서 남을 수 있는 것은 매니페스트의 집 둘(`.claude/`·`.claude/pal/`)뿐이다.
    let mut report = Report::new();
    집을_세운다(&root, &mut created_dirs)?;

    // 옛 기록을 지고 시작한다 — 다시 설치하다 죽어도 **먼젓번 것을 되돌릴 수 있어야**
    // 한다. 새 설치면 비어 있고, 한 걸음마다 찬다.
    let mut 기록 = Journal {
        path: manifest_path,
        m: Manifest {
            pal_version: crate::version::describe().to_owned(),
            roots: Roots {
                dirs: OWNED_DIRS.iter().map(|s| Rel::new(s)).collect(),
                files: OWNED_FILES.iter().map(|s| Rel::new(s)).collect(),
            },
            own_path: Rel::new(MANIFEST),
            files: Vec::new(),
            blocks: 이전.as_ref().map(|m| m.blocks.clone()).unwrap_or_default(),
            settings: 이전.as_ref().and_then(|m| m.settings.clone()),
            created_dirs,
            // 설치는 제거 중이 아니다. 앞선 제거가 걸린 채로 남았어도 이 설치가
            // 그 상태를 덮으므로 여기서 거짓으로 되돌린다.
            removing: false,
        },
    };
    기록.적는다()?;

    디렉터리_세우기(&root, &mut 기록)?;
    파일_놓기(&root, &mut 기록, &mut report)?;
    기록.m.settings = 설정_병합(&settings_path, &read, 이전.as_ref(), &mut report)?;
    기록.적는다()?;
    블록_넣기(&root, 이전.as_ref(), &mut 기록, &mut report)?;

    report.say("매니페스트", &format!("{MANIFEST}  ·  pal {}", 기록.m.pal_version));
    report.print(&format!("설치 — {root}"));
    Ok(())
}

/// **기록이 걸음마다 앞선다** — 매 변경 뒤에 매니페스트를 다시 쓴다.
///
/// # 왜 되감기((a))가 아니라 기록((b))을 골랐는가
///
/// 관측된 실패 트리거 넷 중 **하나가 `SIGKILL`** 이다. 프로세스가 그 자리에서 죽으면
/// 되감을 코드가 돌 기회 자체가 없다 — (a) 는 그 트리거를 **원리상** 못 덮는다.
/// 반면 기록이 앞서 있으면 죽은 자리와 무관하게 **`uninstall` 이 걷어낼 수 있다.**
/// 미리 볼 수 있는 것([`쓸_수_있나`])은 1단계에서 끊어 (a) 로 처리하고, 못 보는 자리는
/// (b) 가 받는다.
struct Journal {
    path: PathBuf,
    m: Manifest,
}

impl Journal {
    fn 적는다(&mut self) -> Result<()> {
        manifest::write(&self.path, &self.m)
    }
}

/// **1단계에서 미리 볼 수 있는 것** — 쓸 수 없는 자리를 여기서 끊는다.
///
/// 관측된 트리거 셋(`.gitignore` 444 · `CLAUDE.md` 444 · `settings.json` 444)은
/// **읽기는 성공하고 쓰기만 실패해서** 옛 검증(`settings::read` 하나)을 통과했다.
///
/// ⚠ **이 검사가 못 보는 것**: 모드 비트만 본다. 남의 소유라 못 쓰는 자리·ACL·읽기
/// 전용 마운트는 여기를 통과하고, 그때는 기록([`Journal`])이 받는다.
fn 쓸_수_있나(root: &Root) -> Result<()> {
    쓸_수_있는가(root.path())?;
    // ⚠ **잠금(`LOCK`)이 이 목록에 없었다.** 그 한 칸이 비어서 FIFO 하나가
    // `install`·`update`·`uninstall` 을 전부 매달았다. 여는 자리
    // ([`Lock::take`])에도 문을 세웠고, 여기 드는 것은 **1단계에서 미리 보기** 위해서다.
    for rel in [CLAUDE_DIR, SETTINGS, ROOT_INSTRUCTION_FILE, IGNORE_FILE, LOCK]
        .into_iter()
        .chain(DIRS.iter().copied())
    {
        쓸_수_있는가(&root.join(&Rel::new(rel))?)?;
    }
    // ★ **우리 대신 읽는 프로세스의 자리도 여기서 본다.** `git check-ignore` 는
    // 중첩 `.gitignore` 와 `.git/info/exclude` 도 읽고, 그중 하나가 FIFO 면 **git 이**
    // 매달린다. 1단계에서 끊어야 반쯤 설치된 프로젝트가 안 남는다.
    ignore::점검(root)?;
    Ok(())
}

fn 쓸_수_있는가(path: &Path) -> Result<()> {
    // **하드링크가 걸린 자리는 제자리 쓰기가 밖으로 샌다** — 1단계에서 끊는다.
    guard::제자리에_써도_되나(path)?;
    // **일반 파일이 아닌 자리는 열자마자 매달린다** — 여기서도 1단계에서 끊는다.
    // ⚠ 디렉터리 목록(`DIRS`)도 이 함수를 지나므로 디렉터리는 아래에서 갈라 본다.
    if !path.is_dir() {
        guard::일반_파일이거나_없나(path)?;
    }
    // 없는 자리는 못 본다 — 그 부모는 위에서 이미 봤다.
    let Ok(meta) = std::fs::metadata(path) else { return Ok(()) };
    if meta.permissions().readonly() {
        bail!(
            "{} 에 **쓸 수 없다**(읽기 전용) — 설치는 여기서 멈춘다.\n    \
             읽기는 되고 쓰기만 안 되는 자리라 예전에는 **반쯤 설치하고 나갔다.** \
             권한을 고친 뒤 다시 돌리십시오",
            path.display()
        );
    }
    Ok(())
}

/// **기록의 집** — 여기가 서기 전에는 적을 자리가 없다. 그래서 이 둘만은 적기 전에
/// 만들고, 그 창(디렉터리 둘)이 이 설계가 못 덮는 유일한 잔해다.
fn 집을_세운다(root: &Root, created: &mut Vec<Rel>) -> Result<()> {
    for dir in MANIFEST_HOME {
        let rel = Rel::new(dir);
        let path = root.join(&rel)?;
        if path.is_dir() {
            continue;
        }
        std::fs::create_dir_all(&path)
            .with_context(|| format!("만들지 못했다: {}", path.display()))?;
        if !created.contains(&rel) {
            created.push(rel);
        }
    }
    Ok(())
}

/// 나머지 디렉터리 — ★ **적고 나서 만든다.**
///
/// 만들고 적는 사이에 죽으면 그 디렉터리는 **기록에 없고** 제거가 못 걷어낸다.
/// 반대 순서의 사고(적었는데 못 만듦)는 제거가 `is_dir()` 로 걸러서 무해하다.
/// **두 사고의 값이 다르므로 순서가 정해진다.**
fn 디렉터리_세우기(root: &Root, 기록: &mut Journal) -> Result<()> {
    for dir in DIRS {
        let rel = Rel::new(dir);
        let path = root.join(&rel)?;
        if path.is_dir() {
            continue;
        }
        if !기록.m.created_dirs.contains(&rel) {
            기록.m.created_dirs.push(rel);
            기록.적는다()?;
        }
        std::fs::create_dir_all(&path)
            .with_context(|| format!("만들지 못했다: {}", path.display()))?;
    }
    Ok(())
}

/// **대상에 없는 것만 쓴다**(`[f24]` ①). 있는 것은 안 건드리고 그 사실을 적는다.
fn 파일_놓기(root: &Root, 기록: &mut Journal, report: &mut Report) -> Result<()> {
    for res in PAYLOAD {
        let rel = Rel::new(res.path);
        let path = root.join(&rel)?;
        if path.exists() {
            report.say("이미 있음", res.path);
        } else {
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)
                    .with_context(|| format!("만들지 못했다: {}", parent.display()))?;
            }
            guard::쓴다(&path, res.body.as_bytes())?;
            report.say("놓았다", res.path);
        }
        // **실물에서 뜬다** — 매니페스트가 적는 값이 곧 디스크의 값이어야 ③ 이 선다.
        let sha = sha256::hex(&guard::읽는다(&path)?);
        // ★ **적는 sha 는 언제나 「우리가 마지막으로 본 바이트」다.** 종류가 가르는
        // 것은 *"그 바이트가 우리 것인가"* 하나뿐이다 — 우리 바이트와 다르면 그것은
        // 사람의 것이고, 재설치가 그것을 「우리 것」으로 다시 도장 찍으면 다음
        // `update` 가 사람의 수정을 밟는다.
        let origin = if sha == sha256::hex(res.body.as_bytes()) {
            Origin::Ours
        } else {
            Origin::UserModified
        };
        기록.m.files.push(FileEntry { path: rel.clone(), sha256: sha, origin });
        // **한 걸음마다 적는다.** 다섯을 다 놓고 적으면 셋째에서 죽었을 때 기록이 없다.
        기록.적는다()?;
    }
    Ok(())
}

fn 설정_병합(
    path: &Path,
    read: &settings::Read,
    이전: Option<&Manifest>,
    report: &mut Report,
) -> Result<Option<SettingsEntry>> {
    let mut want: BTreeMap<String, Value> = BTreeMap::new();
    want.insert(AGENT_KEY.to_owned(), json!(AGENT_VALUE));

    let 옛것 = 이전.and_then(|m| m.settings.clone());
    // **적어 둔 것과 바라는 것을 대서 계획을 낸다.** 실행 파일이 옮겨 갔으면 옛 등록이
    // 여기서 빠진다 — 안 빼면 죽은 등록이 남고 그 실패는 침묵한다.
    let 적힌_훅 = 옛것.as_ref().map(|e| e.hooks.clone()).unwrap_or_default();
    let 바라는_훅 = hooks::desired(HOOK_EVENTS)?;
    let plan = hooks::plan(read.current.as_ref(), &적힌_훅, &바라는_훅);
    for entry in &plan.remove {
        report.say("훅 뺌", &format!("{}  ·  {}", entry.event, entry.보임()));
    }
    for entry in &plan.add {
        report.say("훅 등록", &format!("{}  ·  {}", entry.event, entry.보임()));
    }
    if plan.is_empty() && !바라는_훅.is_empty() {
        report.say("이미 등록됨", &format!("훅 {}개", 바라는_훅.len()));
    }

    let merged = settings::merge(path, read, &want, &plan)?;

    // **이미 우리가 더해 둔 것이면 그 기록을 잃지 않는다** — 잃으면 제거가 못 되돌린다.
    //
    // ⚠ 훅은 **바라는 것 전부**를 적는다(더한 것만이 아니다). 사용자가 우연히 똑같은
    // 절대 경로 문자열을 손으로 걸어 뒀다면 제거가 그것을 걷는데, 그 문자열은 이
    // 설치본의 절대 경로 + 우리 인자 형태라 실질적으로 우리 것이다.
    // **우리가 넣은 값**을 함께 적는다 — 되돌릴 때 *"그 값이 아직 우리 것인가"* 를
    // 대는 자리이고, 안 적으면 사용자가 바꾼 값을 **조용히** 지운다.
    let 더한_값: BTreeMap<String, Value> = merged
        .added_keys
        .iter()
        .filter_map(|k| want.get(k).map(|v| (k.clone(), v.clone())))
        .collect();
    let entry = match 옛것 {
        Some(mut old) => {
            for key in merged.added_keys {
                if !old.added_keys.contains(&key) {
                    old.added_keys.push(key);
                }
            }
            old.added_values.extend(더한_값);
            old.hooks = 바라는_훅;
            old.hooks_key_created |= merged.hooks_key_created;
            Some(old)
        }
        None if merged.added_keys.is_empty() && 바라는_훅.is_empty() => None,
        None => Some(SettingsEntry {
            path: Rel::new(SETTINGS),
            added_keys: merged.added_keys,
            added_values: 더한_값,
            hooks: 바라는_훅,
            hooks_key_created: merged.hooks_key_created,
            created: merged.created,
        }),
    };

    match &entry {
        Some(e) if merged.wrote => {
            let 키 =
                if e.added_keys.is_empty() { "없음".to_owned() } else { e.added_keys.join(" · ") };
            report.say("병합", &format!("{SETTINGS}  (더한 키: {키})"));
        }
        Some(_) => report.say("이미 병합됨", SETTINGS),
        None => report.say("건드리지 않음", &format!("{SETTINGS}  (이미 자기 값이 있다)")),
    }
    Ok(entry)
}

fn 블록_넣기(
    root: &Root,
    이전: Option<&Manifest>,
    기록: &mut Journal,
    report: &mut Report,
) -> Result<()> {
    // ★ **옛 기록을 비우지 않는다.** 옛 코드는 목록을 `clear()` 하고 매니페스트에
    // 적은 **다음** 하나씩 다시 채웠다. 그 사이에 죽거나 실패하면 **파일에는 블록이
    // 있는데 기록에는 없는** 상태가 남고, 그러면 `uninstall` 이 그 블록을 영영 못
    // 걷어낸다(이 회차의 실측: CLAUDE.md 에서 실패한 설치가 `.gitignore` 의 기록을
    // 함께 잃었다). 이제 [`블록_하나`] 가 **경로 단위로 갈아 끼운다.**
    //
    // ── CLAUDE.md — `@` 임포트 한 줄 ────────────────────────────────────────
    //
    // ⚠ **`AGENTS.md` 에 규율을 담지 않는다. 자동 주입되지 않는다**(실측).
    let 지시 = blocks::compose(&MD_MARKERS, &[IMPORT_LINE.to_owned()]);
    블록_하나(root, ROOT_INSTRUCTION_FILE, &지시, 이전, 기록, report)?;

    // ── .gitignore — 파생 경로. **git 에게 물어서 정한다** ──────────────────
    //
    // ⚠ **우리 블록이 이미 있으면 다시 세지 않는다.** 세면 그 경로들이 이제 「덮여
    // 있음」이라 더할 것이 없어지고, 그러면 **블록 기록이 매니페스트에서 사라진다** —
    // 두 번째 설치가 첫 번째와 다른 상태를 내고 제거가 그 블록을 못 되돌린다.
    let ignore_rel = Rel::new(IGNORE_FILE);
    let 옛_등재 = 이전.and_then(|m| m.blocks.iter().find(|b| b.path == ignore_rel).cloned());
    if let Some(old) = 옛_등재 {
        let path = root.join(&ignore_rel)?;
        match blocks::상태(&path, 마커(&ignore_rel), &old.inserted)? {
            // 기록은 이미 [`Journal`] 이 옛 매니페스트에서 지고 왔다 — 적을 것이 없다.
            blocks::상태::그대로 => {
                report.say("이미 있음", IGNORE_FILE);
                return Ok(());
            }
            // **거짓 성공을 안 낸다.** 마커가 남았는데 우리 바이트가 아니면 제거가
            // 걸리는 자리이고, 그 사실을 설치가 조용히 지나가면 아무도 모른다.
            blocks::상태::훼손 => bail!("{}", blocks::훼손_문구(&path, 마커(&ignore_rel))),
            blocks::상태::사라짐 => {}
        }
    }

    let mut 등재 = Vec::new();
    let mut worktree = true;
    for path in DERIVED {
        match ignore::verdict(root, path)? {
            ignore::Verdict::Covered => report.say("이미 등재됨", path),
            ignore::Verdict::NotAWorktree => {
                worktree = false;
                break;
            }
            ignore::Verdict::Revived { pattern } => {
                // **사용자가 일부러 되살린 것을 조용히 뒤집지 않는다.**
                report.say("건드리지 않음", &format!("{path}  (사용자가 `{pattern}` 로 되살렸다)"));
            }
            ignore::Verdict::Uncovered => {
                if ignore::tracked(root, path)? {
                    report.say(
                        "⚠ 추적 중",
                        &format!("{path}  (규칙만으로는 배제되지 않는다 — `git rm --cached` 가 필요하다)"),
                    );
                }
                등재.push(format!("/{}", path.trim_start_matches('/')));
            }
        }
    }
    if worktree {
        if 등재.is_empty() {
            report.say("건드리지 않음", &format!("{IGNORE_FILE}  (더할 것이 없다)"));
        } else {
            let block = blocks::compose(&IGNORE_MARKERS, &등재);
            블록_하나(root, IGNORE_FILE, &block, 이전, 기록, report)?;
        }
    } else {
        // **rc=128 을 rc=1 과 뭉개면 저장소가 아닌 곳에 `.gitignore` 를 만든다.**
        report.say("건너뜀", &format!("{IGNORE_FILE}  (git worktree 가 아니다)"));
    }
    Ok(())
}

fn 블록_하나(
    root: &Root,
    rel: &str,
    block: &str,
    이전: Option<&Manifest>,
    기록: &mut Journal,
    report: &mut Report,
) -> Result<()> {
    let rel = Rel::new(rel);
    let path = root.join(&rel)?;
    let 옛것 = 이전.and_then(|m| m.blocks.iter().find(|b| b.path == rel).cloned());
    match blocks::add(&path, 마커(&rel), block)? {
        blocks::Added::Inserted { bytes, created } => {
            report.say("블록", rel.as_str());
            갈아_끼운다(기록, BlockEntry { path: rel, inserted: bytes, created });
        }
        blocks::Added::AlreadyThere => {
            // **옛 기록을 그대로 지고 간다** — 잃으면 제거가 못 되돌린다.
            let Some(old) = 옛것 else {
                // ★ **되돌리기 기록을 잃은 채 성공을 보고하지 않는다.** 우리 마커가
                // 파일에 있는데 매니페스트에 그 블록이 없으면 제거가 이것을 못 걷어내고,
                // 그때 `uninstall` 은 **rc=0 으로 「제거」 화면을 내면서 블록을 남긴다.**
                bail!(
                    "{rel} 에 우리 마커가 있는데 매니페스트에 그 기록이 없다 — \
                     **제거가 이 블록을 못 되돌린다.**\n    \
                     성공이라고 적지 않는다: 그 블록을 손으로 지운 뒤 다시 돌리거나, \
                     기록이 살아 있는 매니페스트를 되살리십시오"
                );
            };
            // ★ **기록은 있는데 실물이 다르면 그것도 거짓 성공이다.** 옛 코드는 이
            // 자리에서 rc=0 「설치」 화면을 내면서 **아무것도 안 고쳤고**, 그 뒤
            // `uninstall` 은 영영 rc=1 이었다. 무엇을 지워야 하는지 여기서 말한다.
            if blocks::상태(&path, 마커(&rel), &old.inserted)? == blocks::상태::훼손 {
                bail!("{}", blocks::훼손_문구(&path, 마커(&rel)));
            }
            report.say("이미 있음", rel.as_str());
            갈아_끼운다(기록, old);
        }
    }
    // **넣자마자 적는다** — 넣고 죽으면 그 블록은 아무도 못 걷어낸다.
    기록.적는다()
}

/// 이 경로의 기록을 **갈아 끼운다** — 목록을 통째로 비웠다 다시 채우지 않는다.
fn 갈아_끼운다(기록: &mut Journal, entry: BlockEntry) {
    기록.m.blocks.retain(|b| b.path != entry.path);
    기록.m.blocks.push(entry);
}

fn 마커(rel: &Rel) -> &'static layout::Markers {
    if rel.as_str() == IGNORE_FILE { &IGNORE_MARKERS } else { &MD_MARKERS }
}

// ─────────────────────────────────────────────────────────────────────────────
// 갱신 — **밟지 않는 것과 말하지 않는 것은 다르다**
// ─────────────────────────────────────────────────────────────────────────────

/// 안 고친 것만 교체하고 고친 것은 **보고한다**(`[f24]` ④). 그리고 **훅 등록을
/// 지금 실행 파일에 맞춘다.**
///
/// ★ 버전이 같아도 훅은 갱신한다. 실행 파일이 옮겨 가면 옛 등록이 죽은 경로를 가리키고,
/// **그 실패(exit 127)는 완전히 침묵한다** — 버전만 보고 일찍 나가면 아무도 모른다.
///
/// # Errors
/// 설치를 못 찾거나, 못 읽거나, 못 쓰면.
pub fn update(target: &Path) -> Result<()> {
    let root = Root::세운다(target)?;
    let manifest_path = root.join(&Rel::new(MANIFEST))?;
    if !manifest_path.exists() {
        bail!("설치를 찾지 못했다: {} 가 없다", manifest_path.display());
    }
    // ★ **잠금을 판정보다 먼저 잡는다.**
    //
    // 옛 코드는 *"이미 최신"* 을 잠금 **밖에서** 판정하고 rc=0 으로 나갔다. 그래서
    // 반쯤 설치되고 잠긴 프로젝트에 `update` **만** 0초에 *"이미 최신"* 이라고
    // 답했다 — 세 경로 중 하나만 잠긴 프로젝트를 **못 보는** 상태였다.
    let _lock = Lock::take(&root)?;
    let mut m = manifest::read(&manifest_path)?;

    // ── 1단계 · 검증. **여기까지 한 바이트도 안 쓴다** ──────────────────────
    //
    // ★ **경로 봉쇄를 갱신에도 세운다.** 갱신이 쓰는 자리는 컴파일된 목록이지만,
    // 매니페스트가 밖이나 남의 자리를 적고 있으면 그 매니페스트를 **이어서 쓰는 것**
    // 자체가 다음 `uninstall` 에 그 항목을 넘기는 일이다.
    자리들(&root, &m)?;
    let now = crate::version::describe();
    let settings_path = root.join(&Rel::new(SETTINGS))?;
    let read = settings::read(&settings_path)?;
    let 훅_계획 = hooks::plan(
        read.current.as_ref(),
        &m.settings.as_ref().map(|e| e.hooks.clone()).unwrap_or_default(),
        &hooks::desired(HOOK_EVENTS)?,
    );
    let 낡음 = m.pal_version != now;
    if !낡음 && 훅_계획.is_empty() {
        println!();
        println!("■ 갱신 — {root}");
        println!("  이미 최신입니다  ·  pal {now}");
        println!();
        return Ok(());
    }

    // ── 2단계 · 적용 ────────────────────────────────────────────────────────
    let mut report = Report::new();
    if 낡음 {
        report.say("낡음", &format!("{} → {now}", m.pal_version));
    } else {
        report.say("최신", &format!("pal {now}  (훅 등록만 갱신한다)"));
    }
    m.settings = 설정_병합(&settings_path, &read, Some(&m), &mut report)?;
    if !낡음 {
        manifest::write(&manifest_path, &m)?;
        report.print(&format!("갱신 — {root}"));
        return Ok(());
    }

    let 적힌: BTreeMap<String, FileEntry> =
        m.files.iter().map(|f| (f.path.to_string(), f.clone())).collect();
    let mut files = Vec::new();
    for res in PAYLOAD {
        let rel = Rel::new(res.path);
        let path = root.join(&rel)?;
        let 새_sha = sha256::hex(res.body.as_bytes());
        let 실물 =
            if path.exists() { Some(sha256::hex(&guard::읽는다(&path)?)) } else { None };

        // ★ **사람의 것인가** — 종류로도 대고 sha 로도 댄다.
        //
        // - 종류만 보면 **설치 뒤에 손댄 것**을 놓친다(그 항목은 `Ours` 로 적혀 있다).
        // - sha 만 보면, 우리가 사람의 sha 를 적어 둔 다음 회차에서 **기록과 실물이
        //   같아지고** 그 순간 그 파일이 다시 「우리 것」이 되어 갱신이 밟는다.
        //
        // 그리고 실물이 **새 본문과 같으면** 사람이 우리 쪽으로 맞춘 것이니 우리 것이다.
        let 사람의_것 = match (적힌.get(res.path), 실물.as_ref()) {
            (_, None) => None,
            (_, Some(지금)) if *지금 == 새_sha => None,
            (Some(옛), Some(지금)) if 옛.origin == Origin::UserModified || 옛.sha256 != *지금 => {
                Some(지금.clone())
            }
            _ => None,
        };
        if let Some(지금) = 사람의_것 {
            // **밟지 않고 말한다.** 그리고 **지금 내용의 sha 를 적는다** — 옛 sha 를
            // 지고 가면 그 경로의 내용 대조가 통째로 꺼지고, 그 뒤 무엇으로 갈아
            // 끼워져도 `doctor` 가 초록이다.
            report.say(SKIPPED, res.path);
            files.push(FileEntry { path: rel, sha256: 지금, origin: Origin::UserModified });
            continue;
        }
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("만들지 못했다: {}", parent.display()))?;
        }
        guard::쓴다(&path, res.body.as_bytes())?;
        report.say("교체", res.path);
        files.push(FileEntry { path: rel, sha256: 새_sha, origin: Origin::Ours });
    }

    m.files = files;
    now.clone_into(&mut m.pal_version);
    manifest::write(&manifest_path, &m)?;
    report.say("매니페스트", &format!("{MANIFEST}  ·  pal {now}"));
    report.print(&format!("갱신 — {root}"));
    Ok(())
}

// ─────────────────────────────────────────────────────────────────────────────
// 제거 — **매니페스트에 적힌 것만**
// ─────────────────────────────────────────────────────────────────────────────

/// 걷어낸다.
///
/// # Errors
/// 설치를 못 찾거나, **리소스를 하나도 못 찾거나**(⑥-b), 블록이 손으로 고쳐졌으면.
pub fn uninstall(target: &Path) -> Result<()> {
    let root = Root::세운다(target)?;
    let manifest_path = root.join(&Rel::new(MANIFEST))?;
    if !manifest_path.exists() {
        bail!(
            "설치를 찾지 못했다: {} 가 없다 — **지울 게 없었으니 성공**은 거짓말이다",
            manifest_path.display()
        );
    }
    let mut m = manifest::read(&manifest_path)?;

    // ── 1단계 · 검증. **여기까지 한 바이트도 안 지운다** ────────────────────
    // ★ **경로 봉쇄를 여기서 한 번에 세운다** — 파일·블록·설정·디렉터리·매니페스트
    // 자신까지, 매니페스트에서 유도되는 **모든** 경로가 이 한 줄을 지난다. 하나라도
    // 밖을 가리키면 **아무것도 안 지우고** 사람에게 넘긴다.
    let 자리 = 자리들(&root, &m)?;
    let mut 찾은_파일 = 0;
    for f in &m.files {
        if 자리.자리(&f.path)?.exists() {
            찾은_파일 += 1;
        }
    }
    // ⑥-b — **지울 게 없었으니 성공**은 거짓말이다.
    //
    // ⚠ **적은 것이 0 개인 자리는 이 문장의 모집단이 아니다**(ADR-0002 · 「모집단이 0 인
    // 자리는 통과로도 반증으로도 안 센다」). 그때 매니페스트는 *"놓았다"* 고 주장한 적이
    // 없다 — 기록의 집만 세우고 죽은 부분 설치가 바로 그 모양이고, 여기서 거부하면
    // 사용자에게 **잔해만 남고 걷어낼 길이 없다.** 적은 것이 하나라도 있는데 하나도
    // 못 찾은 자리는 그대로 실패다.
    //
    // ⚠ **이미 제거가 시작된 매니페스트도 모집단이 아니다.** 그 목록에 남은 것은 앞
    // 회차가 **아직 못 지운 것**이고, 그중 일부가 이미 없는 것은 정상이다. 그 자리에
    // 이 문장을 적용하면 **자기가 지운 자리를 보고 거짓 경보**를 내며 이어서 도는 길을
    // 막는다(실측: 걸림돌을 치운 뒤에도 영영 rc=1).
    if !m.removing && !m.files.is_empty() && 찾은_파일 == 0 {
        bail!(
            "매니페스트가 적은 리소스 {}개를 **하나도 못 찾았다** — 지울 게 없었으니 \
             성공이라고 적지 않는다. 사람이 봐야 한다",
            m.files.len()
        );
    }
    // ★ **「없다」와 「고쳐졌다」를 가른다.** 옛 코드는 *"우리 바이트가 안 보인다"*
    // 하나로 판정해서, 사용자가 **블록을 통째로 지운** 자리도 훼손으로 읽었다 —
    // 그러면 오류 문구가 시킨 그대로 해도 영영 rc=1 이다.
    let mut 훼손 = Vec::new();
    for b in &m.blocks {
        let path = 자리.자리(&b.path)?;
        if blocks::상태(path, 마커(&b.path), &b.inserted)? == blocks::상태::훼손 {
            훼손.push(blocks::훼손_문구(path, 마커(&b.path)));
        }
    }
    if !훼손.is_empty() {
        bail!("아무것도 지우지 않았다.\n\n{}", 훼손.join("\n\n"));
    }

    // ── 2단계 · 적용. ★ **기록이 걸음마다 앞선다 — 그러나 걸음마다 쓰지는 않는다** ──
    //
    // `install` 은 걸음마다 매니페스트를 다시 썼는데 `uninstall` 은 안 썼다. 그래서
    // 파일 루프 중간에서 실패하면 **이미 지운 것이 기록에 그대로 남고**, 다시 돌려도
    // 같은 자리에서 같은 실패를 반복했다. 그래서 앞 회차가 **한 항목마다 매니페스트
    // 전체를 다시 쓰게** 했고, 그것이 항목 수에 대한 **O(n²)** 를 만들었다(실측:
    // 3,200개 18.1초). 매니페스트는 커밋되는 파일이라 그 크기가 남의 손에 있다.
    //
    // # 왜 걸음마다 안 써도 재개가 서는가 — **걷기가 멱등이기 때문이다**
    //
    // | 항목 | 이미 걷힌 자리를 다시 만나면 |
    // |---|---|
    // | 파일 | 없으니 `이미 없음`. 그리고 `removing` 이 참이라 ⑥-b 의 거짓 경보도 안 난다 |
    // | 블록 | 우리 바이트도 마커도 없으니 `사라짐` → `이미 없음`(이 회차가 갈랐다) |
    // | 설정 | 우리가 더한 키가 이미 없으니 `이미 없음` |
    //
    // 그래서 **실패한 자리에서만** 기록을 줄여 적는다 — 재개는 그대로 서고, 값은
    // 항목 수에 선형이 된다.
    let lock = Lock::take(&root)?;
    let mut report = Report::new();
    // **제거가 시작됐다는 사실 자체를 먼저 적는다.** 이 한 줄이 없으면 다음 회차가
    // 줄어든 목록을 「하나도 못 찾았다」로 읽는다(⑥-b 의 거짓 경보).
    m.removing = true;
    manifest::write(&manifest_path, &m)?;

    let mut 걷은 = 0usize;
    let 결과 = m.blocks.iter().try_for_each(|b| -> Result<()> {
        match blocks::remove(자리.자리(&b.path)?, 마커(&b.path), &b.inserted, b.created)? {
            blocks::Removal::Block => report.say("블록 뺌", b.path.as_str()),
            blocks::Removal::FileGone => report.say("지웠다", b.path.as_str()),
            blocks::Removal::Missing => report.say("이미 없음", b.path.as_str()),
        }
        걷은 += 1;
        Ok(())
    });
    m.blocks.drain(..걷은);
    let 쓰기 = manifest::write(&manifest_path, &m);
    결과?;
    쓰기?;

    if let Some(s) = m.settings.clone() {
        설정_되돌리기(&자리, &s, &mut report)?;
        m.settings = None;
        manifest::write(&manifest_path, &m)?;
    }

    let mut 걷은 = 0usize;
    let 결과 = m.files.iter().try_for_each(|f| -> Result<()> {
        // **회복 방법을 문구가 준다.** 걷은 만큼은 기록에서 빠지므로, 걸린 자리를
        // 손으로 치운 뒤 다시 돌리면 나머지를 이어서 걷는다.
        파일_하나_걷기(&자리, f, &mut report).with_context(|| {
            format!(
                "{} 에서 멈췄다 — 여기까지 걷은 것은 매니페스트에서 이미 빠졌다. \
                 이 자리를 손으로 치운 뒤 `pal uninstall` 을 **다시 돌리면** 나머지를 \
                 이어서 걷는다",
                f.path
            )
        })?;
        걷은 += 1;
        Ok(())
    });
    if let Err(e) = 결과 {
        m.files.drain(..걷은);
        let 쓰기 = manifest::write(&manifest_path, &m);
        쓰기?;
        return Err(e);
    }

    std::fs::remove_file(&manifest_path)
        .with_context(|| format!("지우지 못했다: {}", manifest_path.display()))?;
    report.say("지웠다", MANIFEST);

    // 잠금을 먼저 놓는다 — 안 놓으면 `.claude` 가 비어 있지 않아 안 지워진다.
    drop(lock);
    디렉터리_걷기(&m, &자리, &mut report)?;
    report.print(&format!("제거 — {root}"));
    Ok(())
}

/// **우리가 만든 디렉터리를 걷는다** — ★ **순서에 안 기대고, 못 지우면 말한다.**
///
/// # 왜 목록의 순서를 안 믿는가
///
/// `install` 은 옛 매니페스트에서 목록을 물려받은 **뒤에** `.claude` 를 `push` 한다.
/// 다른 프로세스가 `.claude` 를 먼저 만든 회차에서는 순서가 뒤집히고, 역순으로 도는
/// 제거가 `.claude` 를 자식보다 **먼저** 지우려다 실패했다. 옛 코드의
/// `if path.is_dir() && remove_dir(path).is_ok()` 가 **그 실패를 삼켜** rc=0 과
/// *"제거 완료"* 화면을 냈고 디렉터리는 남았다.
///
/// 그래서 **깊은 것부터** 지운다. 목록이 어떤 순서로 적혀 있든 결과가 같다.
///
/// # 그리고 못 지웠으면 말한다
///
/// 남의 것이 들어와 있으면 그 자리는 이제 우리 것이 아니라 **안 지우는 것이 맞다**(⑥).
/// 그런데 **안 지웠다는 사실을 안 말하면** 사용자는 제거가 끝났다고 믿는다 — 게이트
/// ④ 가 세운 *"밟지 않는 것과 말하지 않는 것은 다르다"* 를 여기에도 세운다.
fn 디렉터리_걷기(m: &Manifest, 자리: &manifest::Places, report: &mut Report) -> Result<()> {
    // ⚠ **경로 구분자 가정**: `Rel` 은 언제나 `/` 로 갈린다(`layout` 의 상수와
    // `manifest::walk` 가 그 규칙 하나 위에 선다). 깊이는 그 개수다.
    let mut dirs: Vec<&Rel> = m.created_dirs.iter().collect();
    dirs.sort_by_key(|r| std::cmp::Reverse(r.as_str().matches('/').count()));
    for dir in dirs {
        let path = 자리.자리(dir)?;
        if !path.is_dir() {
            report.say("이미 없음", &format!("{dir}/"));
            continue;
        }
        match std::fs::remove_dir(path) {
            Ok(()) => report.say("지웠다", &format!("{dir}/")),
            Err(e) => report.say("남겼다", &format!("{dir}/  ({})", 왜_못_지웠나(path, &e))),
        }
    }
    Ok(())
}

/// 왜 못 지웠는지 — **비어 있지 않으면 무엇이 남았는지까지.**
fn 왜_못_지웠나(path: &Path, e: &std::io::Error) -> String {
    let mut 남은: Vec<String> = std::fs::read_dir(path)
        .into_iter()
        .flatten()
        .flatten()
        .map(|d| d.file_name().to_string_lossy().into_owned())
        .collect();
    if 남은.is_empty() {
        return format!("{e}");
    }
    남은.sort();
    let 몇 = 남은.len();
    남은.truncate(5);
    format!("비어 있지 않다 — 남의 것 {몇}개가 산다: {}", 남은.join(" · "))
}

/// 파일 하나를 걷는다 — **말없이 지우지 않는다.**
///
/// `update` 가 「사용자 수정 — 건너뜀」으로 지킨 파일을 제거는 sha 대조 없이 지웠다.
/// 게이트 ④ 가 세운 *"밟지 않는 것과 말하지 않는 것은 다르다"* 를 여기에도 세운다.
///
/// ⚠ **지우는 것 자체는 그대로다** — ⑥ 이 `S2 == S0` 을 요구하므로 남기면 그것이
/// 반증이다. 여기서 더하는 것은 **말**이다. sha 로 대는 이유는 기록의 종류
/// (`Origin`)만 보면 설치 뒤에 손댄 것을 놓치기 때문이다.
fn 파일_하나_걷기(자리: &manifest::Places, f: &FileEntry, report: &mut Report) -> Result<()> {
    let path = 자리.자리(&f.path)?;
    if !path.exists() {
        report.say("이미 없음", f.path.as_str());
        return Ok(());
    }
    let 고쳤나 = sha256::hex(&guard::읽는다(path)?) != f.sha256;
    std::fs::remove_file(path)
        .with_context(|| format!("지우지 못했다: {}", path.display()))?;
    if 고쳤나 {
        report.say(지운_사용자_수정, f.path.as_str());
    } else {
        report.say("지웠다", f.path.as_str());
    }
    Ok(())
}

/// 설정에서 우리가 더한 것만 뺀다.
fn 설정_되돌리기(
    자리: &manifest::Places,
    s: &SettingsEntry,
    report: &mut Report,
) -> Result<()> {
    let 결과 = settings::unmerge(자리.자리(&s.path)?, s)?;
    if !결과.뺐다 {
        report.say("이미 없음", s.path.as_str());
        return Ok(());
    }
    let 뺀것 = s
        .added_keys
        .iter()
        .cloned()
        .chain(s.hooks.iter().map(|h| format!("훅 {}", h.event)))
        .collect::<Vec<_>>();
    report.say("키 뺌", &format!("{}  ({})", s.path, 뺀것.join(" · ")));
    // ★ **설정 키의 「사용자 수정」도 말한다.** 파일 쪽에만 붙어 있던 대칭을 여기에도
    // 세운다 — 사용자가 자기 값으로 바꿔 둔 키를 조용히 지우면 화면은 `키 뺌` 한 줄뿐이다.
    if !결과.사용자가_바꾼_키.is_empty() {
        report.say(
            지운_사용자_수정,
            &format!(
                "{}  (우리가 넣은 값이 아니었다: {})",
                s.path,
                결과.사용자가_바꾼_키.join(" · ")
            ),
        );
    }
    // 그리고 **파일까지 지웠으면** 그것도 말한다 — 키만 뺐다고 읽히면 안 된다.
    if 결과.파일째_지웠다 {
        report.say("파일째 지웠다", &format!("{}  (우리가 만들었고 나머지가 비었다)", s.path));
    }
    Ok(())
}
