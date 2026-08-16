//! ★ **세 경로 어디서도 대상 프로젝트 바깥에 안 쓴다** — `[f24]` ⑦.
//!
//! # 재는 문장이 무엇인가
//!
//! 격리 HOME 을 쓰면 재는 문장이 *"진짜 사용자 홈이 안 변했다"* 가 아니라
//! **"홈이 무엇이든 그 아래에 안 쓴다"** 가 된다. **후자가 더 센 문장이고**, 동시에
//! 이 측정이 실제 사용자 홈을 건드릴 위험을 0 으로 만든다.
//!
//! # 감시 뿌리 셋
//!
//! - 격리 `$HOME` 전체 — `~/.claude/` · `~/.config/` · `~/.local/` ·
//!   `~/Library/Application Support/` 를 **미리 채워 둔다**. 비어 있으면 이 측정이
//!   **아무것도 안 세고 통과한다**(하한).
//! - 격리 `$TMPDIR`
//! - 대상 프로젝트의 **부모 디렉터리** — 형제 경로에 새는지.
//!
//! # ⚠ 이 측정의 한계를 미리 적는다
//!
//! 스냅샷 비교는 **실행 중에 생겼다가 지워진 것을 못 잡는다.** 그것까지 잡으려면
//! syscall 추적(`dtruss`·`strace`)이 필요하고 **이 회차는 하지 않는다.** 나중에
//! *"쟀는데 못 잡았다"* 가 아니라 **처음부터 안 잰 범위**로 남긴다.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;

const PAL: &str = env!("CARGO_BIN_EXE_pal");

/// 감시 뿌리 하나의 상태 — 디렉터리는 있음/없음, 파일은 크기·mtime·내용.
type Snapshot = BTreeMap<String, String>;

struct 방 {
    base: PathBuf,
    home: PathBuf,
    tmp: PathBuf,
    parent: PathBuf,
    target: PathBuf,
}

impl 방 {
    fn 세운다(tag: &str) -> Self {
        let base =
            std::env::temp_dir().join(format!("pal-f24-바깥-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
        let home = base.join("home");
        let tmp = base.join("tmp");
        let parent = base.join("parent");
        let target = parent.join("proj");

        // ★ **홈을 미리 채운다.** 비어 있으면 「차이 0」이 공짜로 선다.
        for (dir, name) in [
            (".claude", "settings.json"),
            (".config", "some.conf"),
            (".local/share", "data.bin"),
            ("Library/Application Support", "state.plist"),
        ] {
            let d = home.join(dir);
            std::fs::create_dir_all(&d).expect("홈 채우기");
            std::fs::write(d.join(name), format!("사용자의 {name}\n")).expect("홈 파일");
        }
        std::fs::create_dir_all(&tmp).expect("tmp");
        std::fs::write(tmp.join("남의것"), "건드리면 안 된다\n").expect("tmp 파일");

        // 형제 경로 — 부모 아래에 우리 것이 아닌 것이 산다.
        std::fs::create_dir_all(parent.join("형제")).expect("형제");
        std::fs::write(parent.join("형제/파일"), "형제의 것\n").expect("형제 파일");

        std::fs::create_dir_all(&target).expect("대상");
        std::fs::write(target.join("README.md"), "hello\n").expect("README");
        std::fs::write(target.join("CLAUDE.md"), "# 내 규칙\n").expect("CLAUDE.md");

        let me = Self { base, home, tmp, parent, target };
        me.git(&["init", "-q", "."]);
        me.git(&["add", "-A"]);
        me.git(&["-c", "user.email=t@e", "-c", "user.name=t", "commit", "-qm", "첫"]);
        me
    }

    /// 격리 환경 그대로 명령을 만든다 — **HOME 도 XDG 도 TMPDIR 도 우리 것이다.**
    fn 명령(&self, program: &str) -> Command {
        let mut cmd = Command::new(program);
        cmd.current_dir(&self.target)
            .env("HOME", &self.home)
            .env("XDG_CONFIG_HOME", self.home.join(".config"))
            .env("XDG_DATA_HOME", self.home.join(".local/share"))
            .env("XDG_CACHE_HOME", self.home.join(".cache"))
            .env("TMPDIR", &self.tmp);
        cmd
    }

    fn git(&self, args: &[&str]) {
        let out = self.명령("git").args(args).output().expect("git");
        assert!(out.status.success(), "git {args:?}: {}", String::from_utf8_lossy(&out.stderr));
    }

    fn pal(&self, args: &[&str]) -> String {
        let out = self.명령(PAL).args(args).output().expect("pal");
        assert!(
            out.status.success(),
            "pal {args:?}: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        String::from_utf8_lossy(&out.stdout).into_owned()
    }

    /// 감시 뿌리 셋을 한 번에 뜬다. **대상 프로젝트는 뺀다** — 거기는 바뀌어야 한다.
    fn 감시(&self) -> Snapshot {
        let mut out = BTreeMap::new();
        훑기(&self.home, &self.home, "HOME", &self.target, &mut out);
        훑기(&self.tmp, &self.tmp, "TMPDIR", &self.target, &mut out);
        훑기(&self.parent, &self.parent, "부모", &self.target, &mut out);
        out
    }
}

impl Drop for 방 {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.base);
    }
}

fn 훑기(root: &Path, dir: &Path, 이름: &str, 뺄것: &Path, out: &mut Snapshot) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path == 뺄것 {
            continue;
        }
        let rel = format!("{이름}:{}", path.strip_prefix(root).unwrap_or(&path).display());
        if path.is_dir() {
            out.insert(rel, "<디렉터리>".to_owned());
            훑기(root, &path, 이름, 뺄것, out);
        } else {
            let meta = path.symlink_metadata().expect("stat");
            let bytes = std::fs::read(&path).unwrap_or_default();
            let mtime = meta
                .modified()
                .ok()
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map_or(0, |d| d.as_nanos());
            out.insert(rel, format!("{}·{mtime}·{:x}", meta.len(), 합(&bytes)));
        }
    }
}

fn 합(bytes: &[u8]) -> u64 {
    bytes.iter().fold(1469598103934665603u64, |h, b| {
        (h ^ u64::from(*b)).wrapping_mul(1099511628211)
    })
}

fn 갈린다(전: &Snapshot, 후: &Snapshot) -> Vec<String> {
    let mut out = Vec::new();
    for (k, v) in 후 {
        match 전.get(k) {
            None => out.push(format!("생겼다: {k}")),
            Some(before) if before != v => out.push(format!("바뀌었다: {k}")),
            Some(_) => {}
        }
    }
    for k in 전.keys() {
        if !후.contains_key(k) {
            out.push(format!("사라졌다: {k}"));
        }
    }
    out
}

/// ★ **세 경로 각각에서 감시 뿌리의 차이가 0.**
#[test]
fn 세_경로_어디서도_대상_바깥을_안_건드린다() {
    let 방 = 방::세운다("셋");

    // 하한 — 감시할 것이 실제로 있는가. 0 이면 이 시험은 아무것도 안 세고 통과한다.
    let 처음 = 방.감시();
    assert!(처음.len() >= 10, "감시 뿌리가 비었다 — 이 시험은 아무것도 안 재고 있다: {}", 처음.len());

    for 경로 in ["install", "update", "uninstall"] {
        // `update` 가 실제로 일할 조건을 만든다 — 「이미 최신」이면 아무것도 안 하고,
        // 그러면 그 회차는 **대조 불가**이지 통과가 아니다.
        if 경로 == "update" {
            낡게_만든다(&방.target);
        }
        let 전 = 방.감시();
        방.pal(&[경로]);
        let 갈림 = 갈린다(&전, &방.감시());
        assert!(갈림.is_empty(), "`pal {경로}` 가 대상 바깥을 건드렸다:\n  {}", 갈림.join("\n  "));
    }

    // 그리고 실제로 설치가 서 있었는가 — 아니면 세 번 다 아무 일도 안 한 것이다.
    assert!(!방.target.join(".claude/pal").exists(), "제거가 안 됐다");
}

fn 낡게_만든다(target: &Path) {
    let path = target.join(".claude/pal/manifest.json");
    let mut m: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&path).expect("매니페스트")).expect("JSON");
    m["pal_version"] = serde_json::json!("0.0.0+옛날");
    std::fs::write(&path, serde_json::to_string_pretty(&m).expect("직렬화")).expect("쓰기");
}

/// **`pal doctor --install` 도 같은 자리에 선다** — 진단이 홈을 읽으면 ⑦ 이 무너진다.
#[test]
fn 진단도_대상_바깥을_안_건드린다() {
    let 방 = 방::세운다("진단");
    방.pal(&["install"]);
    let 전 = 방.감시();
    방.pal(&["doctor", "--install"]);
    let 갈림 = 갈린다(&전, &방.감시());
    assert!(갈림.is_empty(), "`pal doctor --install` 이 대상 바깥을 건드렸다:\n  {}", 갈림.join("\n  "));
}
