//! **증인을 한 흐름으로** — 회차 `2026-09-15-clean-uninstall` 완수 조건 G1.
//!
//! 문서가 있는 방에서 install → `touch` → `narrative` → 결박 하나 → `round approve` →
//! `round verify`(met) → 종료 봉인 → `stop enable` → 읽을 수 있는 transcript 의 Stop 훅 →
//! uninstall 을 **한 줄로** 밟고 워킹트리와 HOME 을 설치 전과 댄다.
//!
//! # 걸음마다 대상이 생겼음을 uninstall 전에 단언한다
//!
//! 안 생긴 것을 안 남았다고 헤아리지 않는다(사전부검 R1 3·8). 흐름이 중간에 조용히
//! 멈추면 스냅샷은 저절로 같아지고, 그 통과는 아무것도 재지 않은 통과다.
//!
//! # 저장소 자리
//!
//! 기본 저장소 자리를 그대로 쓴다(`PAL_APPROVAL_DIR` 를 지운다) — [`clean_uninstall_external`] 과 같은 규율이다.
//! - macOS · Linux: 격리 `HOME`(Linux 는 `XDG_DATA_HOME` 도 지운다) — 스냅샷은 **HOME 전체**.
//! - Windows: 실제 `%LOCALAPPDATA%\palimpsest` — 격리할 수 없으므로 CI(`CI` 환경 변수)에서만 돌고
//!   **시작 전에 없음을 단언**한다. 같은 자리를 나눠 쓰므로 이 바이너리의 시험은 한 줄로 돈다.
//!
//! # 회차가 둘인 까닭
//!
//! `report.md` 가 있는 회차는 `Reported` 라 `round stop enable` 이 거부한다
//! (`round/status.rs` 의 `terminal`). 그래서 **봉인할 회차**와 **Stop 을 거는 열린 회차**를
//! 갈라 둔다 — 흐름의 순서(승인 → 검증 → 봉인 → Stop 활성화 → 훅)는 그대로다.
//!
//! [`clean_uninstall_external`]: ../clean_uninstall_external/index.html

mod common;

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::sync::{Mutex, MutexGuard};

use common::snapshot::{self, 갈린_경로};
use common::{PAL, 상대_경로};
use serde_json::{Value, json};

/// Stop 을 거는 회차 — `report.md` 를 안 두어 `Open` 으로 남는다.
const 열린: &str = "open-round";
/// 종료 봉인까지 가는 회차.
const 닫힌: &str = "sealed-round";
/// `touch` · `bind` 가 짚는 좌표 — 방 안에서 이름이 하나뿐이다.
const 심볼: &str = "target";

/// Windows 는 실제 사용자 자리를 나눠 쓴다 — 시험끼리 겹치면 「시작 전에 없음」이 깨진다.
static 자리_잠금: Mutex<()> = Mutex::new(());

// ─────────────────────────────────────────────────────────────────────────────
// 방
// ─────────────────────────────────────────────────────────────────────────────

struct 방 {
    base: PathBuf,
    repo: PathBuf,
    home: PathBuf,
    _잠금: Option<MutexGuard<'static, ()>>,
}

impl 방 {
    /// 문서가 있어 `narrative` 가 개체를 만드는 방 — origin 이 프로젝트 식별자를 정한다.
    fn new(tag: &str) -> Self {
        let 잠금 = if cfg!(windows) {
            Some(자리_잠금.lock().unwrap_or_else(std::sync::PoisonError::into_inner))
        } else {
            None
        };
        let base = std::env::temp_dir().join(format!("pal-cu-flow-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
        let home = base.join("home");
        std::fs::create_dir_all(&home).expect("격리 HOME");
        #[cfg(windows)]
        {
            assert!(
                std::env::var_os("CI").is_some(),
                "이 시험은 Windows 에서 실제 %LOCALAPPDATA%\\palimpsest 를 쓴다 — 사용자 기록을 건드리지 않도록 CI(`CI` 환경 변수)에서만 돈다"
            );
            assert!(
                !밖_뿌리().exists(),
                "시작 전에 {} 가 이미 있다 — 설치 전 상태가 아니다",
                밖_뿌리().display()
            );
        }
        let repo = base.join("repo");
        std::fs::create_dir_all(repo.join("src")).expect("src");
        std::fs::create_dir_all(repo.join("docs")).expect("docs");
        let 방 = Self { base, repo, home, _잠금: 잠금 };
        방.쓴다("tsconfig.json", r#"{"compilerOptions":{"moduleResolution":"bundler"}}"#);
        방.쓴다(
            "src/core.ts",
            "export function helper() { return 0; }\nexport function target() { return helper(); }\n",
        );
        방.쓴다("docs/decisions.md", "# 결정들\n\n## 첫 결정\n\n`target` 은 `helper` 를 부른다.\n");
        방.쓴다("README.md", "hello\n");
        git(&방.repo, &["init", "-q"]);
        git(&방.repo, &["config", "user.email", "fixture@example.invalid"]);
        git(&방.repo, &["config", "user.name", "Fixture"]);
        git(&방.repo, &["remote", "add", "origin", &format!("https://example.invalid/flow-{tag}.git")]);
        git(&방.repo, &["add", "-A"]);
        git(&방.repo, &["commit", "-q", "-m", "fixture"]);
        방
    }

    fn 자리(&self, rel: &str) -> PathBuf {
        self.repo.join(rel)
    }

    fn 쓴다(&self, rel: &str, body: &str) {
        let p = self.자리(rel);
        std::fs::create_dir_all(p.parent().expect("부모")).expect("디렉터리");
        std::fs::write(p, body).expect("쓰기");
    }

    /// 스냅샷의 뿌리(밖) — macOS·Linux 는 HOME 전체, Windows 는 `%LOCALAPPDATA%\palimpsest`.
    fn 뿌리(&self) -> PathBuf {
        #[cfg(windows)]
        {
            밖_뿌리()
        }
        #[cfg(not(windows))]
        {
            self.home.clone()
        }
    }

    /// `palimpsest/` 의 부모 — 기록을 처음 쓸 때 없으면 만들어지는 조상.
    fn 조상(&self) -> PathBuf {
        #[cfg(windows)]
        {
            PathBuf::from(std::env::var_os("LOCALAPPDATA").expect("LOCALAPPDATA"))
        }
        #[cfg(target_os = "macos")]
        {
            self.home.join("Library").join("Application Support")
        }
        #[cfg(all(unix, not(target_os = "macos")))]
        {
            self.home.join(".local").join("share")
        }
    }

    fn 저장소(&self) -> PathBuf {
        self.조상().join("palimpsest").join("approvals")
    }

    /// 조상 기록 — 걷은 뒤 어느 디렉터리까지 지워도 되는지 적어 둔 자리.
    fn 조상_기록(&self) -> PathBuf {
        self.조상().join("palimpsest").join("created-ancestors.json")
    }

    fn 워킹트리(&self) -> BTreeMap<String, snapshot::항목> {
        snapshot::워킹트리(&self.repo)
    }

    fn 밖(&self) -> BTreeMap<String, snapshot::항목> {
        snapshot::트리(&self.뿌리())
    }

    fn 밖_상대(&self, path: &Path) -> String {
        상대_경로(&self.뿌리(), path)
    }

    fn 명령(&self) -> Command {
        let mut command = Command::new(PAL);
        command.current_dir(&self.repo).env_remove("PAL_APPROVAL_DIR");
        if cfg!(unix) {
            command.env("HOME", &self.home).env_remove("XDG_DATA_HOME");
        }
        command
    }

    fn 돌린다(&self, args: &[&str]) -> Output {
        self.명령().args(args).output().expect("pal 을 못 돌렸다")
    }

    fn 성공(&self, args: &[&str]) -> String {
        let out = self.돌린다(args);
        assert!(out.status.success(), "pal {args:?}\n{}", 문자열(&out));
        String::from_utf8(out.stdout).expect("UTF-8")
    }

    /// 의도 저장소를 다시 열어 읽은 (결박 수 · 개체 수). **닫고 돌려준다** — Windows 는 열린 파일을 못 지운다.
    fn 의도_수(&self) -> (usize, usize) {
        let s = pal_intent::IntentStore::open_read_only(&self.자리(".palimpsest/intent.redb"))
            .expect("의도 저장소를 못 열었다");
        (s.count().expect("결박 수"), s.entity_count().expect("개체 수"))
    }

    fn stop_훅(&self, transcript: &Path) -> Output {
        use std::io::Write;
        let payload = json!({
            "session_id": "clean-uninstall-flow",
            "transcript_path": transcript,
            "cwd": self.repo,
            "hook_event_name": "Stop",
            "stop_hook_active": false,
            "last_assistant_message": "done"
        });
        let mut child = self
            .명령()
            .args(["hook", "Stop"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("hook");
        child
            .stdin
            .as_mut()
            .expect("stdin")
            .write_all(payload.to_string().as_bytes())
            .expect("payload");
        child.wait_with_output().expect("hook wait")
    }
}

impl Drop for 방 {
    fn drop(&mut self) {
        #[cfg(windows)]
        {
            let _ = std::fs::remove_dir_all(밖_뿌리());
        }
        let _ = std::fs::remove_dir_all(&self.base);
    }
}

#[cfg(windows)]
fn 밖_뿌리() -> PathBuf {
    PathBuf::from(std::env::var_os("LOCALAPPDATA").expect("LOCALAPPDATA")).join("palimpsest")
}

// ─────────────────────────────────────────────────────────────────────────────
// 흐름
// ─────────────────────────────────────────────────────────────────────────────

/// 흐름이 밖에 남긴 기록의 이름.
struct 흔적 {
    승인: String,
    봉인: String,
    활성화_digest: String,
    식별자: String,
}

impl 흔적 {
    fn 승인_json(&self, store: &Path) -> PathBuf {
        store.join(format!("{}.json", self.승인))
    }
    fn 승인_표시(&self, store: &Path) -> PathBuf {
        store.join(format!("{}.project", self.승인))
    }
    fn 봉인_json(&self, store: &Path) -> PathBuf {
        store.join(format!("{}.json", self.봉인))
    }
    fn 봉인_표시(&self, store: &Path) -> PathBuf {
        store.join(format!("{}.project", self.봉인))
    }
    /// 기본 uninstall 이 남기는 밖의 정본 넷.
    fn 밖의_정본(&self, store: &Path) -> Vec<PathBuf> {
        vec![self.승인_json(store), self.승인_표시(store), self.봉인_json(store), self.봉인_표시(store)]
    }
    /// 기본 uninstall 이 걷는 운영 상태.
    fn 운영_상태(&self, store: &Path) -> Vec<PathBuf> {
        vec![
            store.join(format!("round-stop-activation-{}.json", self.식별자)),
            store.join(format!("round-stop-progress-{}.json", self.활성화_digest)),
            store.join(format!("round-stop-progress-{}.lock", self.활성화_digest)),
            store.join(format!("round-stop-progress-{}.project", self.활성화_digest)),
        ]
    }
}

/// 회차 둘을 워킹트리에 쓴다 — **커밋하지 않는다**(설치 뒤에 생긴 추적 안 된 정본이다).
fn 회차를_쓴다(방: &방) {
    for slug in [열린, 닫힌] {
        let d = 방.자리(".palimpsest/rounds").join(slug);
        std::fs::create_dir_all(&d).expect("회차 디렉터리");
        std::fs::write(d.join("intent.md"), "# fixture\n\n## 완수 조건\n\n- [ ] A1 condition A1\n")
            .expect("intent.md");
        std::fs::write(
            d.join("verification.log"),
            format!(
                "{}\n{}\n",
                json!({"kind":"schema","version":3,"round":slug}),
                json!({"kind":"oracle","id":"A1","mode":"command","check":"echo ROUND_OK","expect":{"literal":"ROUND_OK"},"cwd":"."})
            ),
        )
        .expect("verification.log");
    }
    // 봉인할 회차만 종결 문서를 갖는다 — 열린 회차에 `report.md` 를 두면 `Reported` 가 되어
    // `round stop enable` 이 거부한다.
    let d = 방.자리(".palimpsest/rounds").join(닫힌);
    std::fs::write(
        d.join("report.md"),
        "# report\n\n## 남지 않은 것\n없음.\n\n## 다음 회차가 받는 것\n없음.\n\n## 범위 밖\n없음.\n\n## 원리상 못 잰 것\n없음.\n\n## 능력 부재\n없음.\n",
    )
    .expect("report.md");
    std::fs::write(
        d.join("findings.jsonl"),
        format!("{}\n", json!({"schema_version":3,"종류":"레코드","회차":닫힌})),
    )
    .expect("findings.jsonl");
}

/// **한 흐름** — 걸음마다 대상이 생겼음을 단언하고 밖의 기록 이름을 돌려준다.
fn 흐름(방: &방) -> 흔적 {
    방.성공(&["install"]);
    assert!(방.자리(".claude/pal/manifest.json").is_file(), "전제 — install 이 매니페스트를 놓아야 한다");

    방.성공(&["touch", 심볼]);
    for rel in [".palimpsest/index.redb", ".palimpsest/cache"] {
        assert!(방.자리(rel).exists(), "전제 — `touch` 가 {rel} 를 만들어야 한다");
    }

    방.성공(&["narrative"]);
    assert!(
        방.자리(".palimpsest/narrative-pending.json").is_file(),
        "전제 — `narrative` 가 진행 중인 인입을 적어야 한다"
    );
    let (결박_전, 개체) = 방.의도_수();
    assert!(개체 > 0, "전제 — `narrative` 가 intent.redb 에 개체를 만들어야 한다");
    assert_eq!(결박_전, 0, "전제 — 아직 결박이 없어야 한다");

    방.성공(&["bind", 심볼, "--note", "이 함수는 helper 를 부른다"]);
    assert_eq!(방.의도_수().0, 1, "전제 — 결박 하나가 생겨야 한다");

    회차를_쓴다(방);
    let approved: Value =
        serde_json::from_str(&방.성공(&["round", "approve", "--round", 닫힌, "--id", "A1", "--json"]))
            .expect("approve JSON");
    let 승인 = approved["approval_digest"].as_str().expect("approval_digest").to_owned();

    let verified: Value =
        serde_json::from_str(&방.성공(&["round", "verify", "--round", 닫힌, "--id", "A1", "--json"]))
            .expect("verify JSON");
    assert_eq!(verified["met"], true, "전제 — 승인한 oracle 이 met 이어야 한다: {verified}");

    let finalized: Value =
        serde_json::from_str(&방.성공(&["round", "verify", "--round", 닫힌, "--all", "--json"]))
            .expect("finalize JSON");
    assert_eq!(finalized["completion"], "complete", "전제 — 종료 봉인까지 가야 한다: {finalized}");
    let 봉인 = 봉인_digest(방);

    let enabled: Value =
        serde_json::from_str(&방.성공(&["round", "stop", "enable", "--round", 열린, "--json"]))
            .expect("enable JSON");
    let 활성화_digest = enabled["activation_digest"].as_str().expect("activation_digest").to_owned();

    let transcript = 방.base.join("transcript.jsonl");
    std::fs::write(&transcript, "한 흐름\n").expect("transcript");
    let hook = 방.stop_훅(&transcript);
    assert!(
        String::from_utf8_lossy(&hook.stdout).contains("\"block\""),
        "Stop 훅이 진행 파일을 쓰는 자리(차단)까지 가지 않았다\n{}",
        문자열(&hook)
    );

    let 식별자 = origin_식별자(&git_출력(&방.repo, &["config", "--get", "remote.origin.url"]));
    let 흔적 = 흔적 { 승인, 봉인, 활성화_digest, 식별자 };
    let store = 방.저장소();
    for path in 흔적.밖의_정본(&store).into_iter().chain(흔적.운영_상태(&store)) {
        assert!(path.is_file(), "생겼어야 할 밖의 기록이 없다: {}", path.display());
    }
    assert!(방.조상_기록().is_file(), "생겼어야 할 조상 기록이 없다: {}", 방.조상_기록().display());
    흔적
}

fn 봉인_digest(방: &방) -> String {
    std::fs::read_to_string(방.자리(".palimpsest/rounds").join(닫힌).join("verification.log"))
        .expect("원장")
        .lines()
        .filter_map(|line| serde_json::from_str::<Value>(line).ok())
        .filter(|event| event["kind"] == "checkpoint")
        .filter_map(|event| event["finalization_seal"].as_str().map(str::to_owned))
        .next_back()
        .expect("checkpoint 가 없다 — 종료 봉인이 안 됐다")
}

/// `pal-git` 의 `stable_repository_identity` 를 명세대로 다시 계산한다(origin 이 있는 저장소).
fn origin_식별자(url: &str) -> String {
    let bytes = url.trim().as_bytes();
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"pal.repository.origin.v1\0");
    hasher.update(&(bytes.len() as u64).to_le_bytes());
    hasher.update(bytes);
    hasher.finalize().to_hex().to_string()
}

fn git(cwd: &Path, args: &[&str]) {
    let out = Command::new("git").args(args).current_dir(cwd).output().expect("git");
    assert!(out.status.success(), "git {args:?}: {}", String::from_utf8_lossy(&out.stderr));
}

fn git_출력(cwd: &Path, args: &[&str]) -> String {
    let out = Command::new("git").args(args).current_dir(cwd).output().expect("git");
    assert!(out.status.success(), "git {args:?}: {}", String::from_utf8_lossy(&out.stderr));
    String::from_utf8(out.stdout).expect("UTF-8")
}

fn 문자열(out: &Output) -> String {
    format!(
        "rc={:?}\nstdout:\n{}\nstderr:\n{}",
        out.status.code(),
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    )
}

// ─────────────────────────────────────────────────────────────────────────────
// 허용 목록 `L`
// ─────────────────────────────────────────────────────────────────────────────

/// **워킹트리 쪽 `L`** — 기본 uninstall 뒤 남아도 되는 자리(조건 머리말).
///
/// `.palimpsest` 자신은 목록에 적힌 자리들의 **부모 디렉터리**다 — 그것이 없으면 안의 자리가 남을 수 없다.
fn l_워킹트리인가(rel: &str) -> bool {
    rel == ".palimpsest"
        || rel == ".palimpsest/intent.redb"
        || rel == ".palimpsest/.gitignore"
        || rel == ".palimpsest/intent"
        || rel.starts_with(".palimpsest/intent/")
        || rel == ".palimpsest/rounds"
        || rel.starts_with(".palimpsest/rounds/")
}

/// **워킹트리 쪽 `L` 의 정본 자리** — 남으면 uninstall 직전 바이트와 같아야 하는 것.
///
/// `.palimpsest/.gitignore` 는 여기 없다 — **uninstall 이 그때 두는 가림 파일**이라 직전 바이트가 없다.
fn 정본_자리인가(rel: &str) -> bool {
    rel == ".palimpsest/intent.redb"
        || rel.starts_with(".palimpsest/intent/")
        || rel.starts_with(".palimpsest/rounds/")
}

/// **밖의 `L`** — 승인·봉인과 표시 파일 · 조상 기록 · 그 자리의 부모 디렉터리.
fn l_밖(방: &방, 흔적: &흔적) -> BTreeSet<String> {
    let store = 방.저장소();
    let mut out: BTreeSet<String> =
        흔적.밖의_정본(&store).iter().map(|p| 방.밖_상대(p)).collect();
    out.insert(방.밖_상대(&방.조상_기록()));
    let 뿌리 = 방.뿌리();
    let mut 부모 = Some(store.as_path());
    while let Some(p) = 부모 {
        if p == 뿌리 {
            break;
        }
        let rel = 방.밖_상대(p);
        if rel.is_empty() {
            break;
        }
        out.insert(rel);
        부모 = p.parent();
    }
    out
}

// ─────────────────────────────────────────────────────────────────────────────
// G1
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn g1_한_흐름을_밟고_purge_하면_워킹트리와_home_이_설치_전과_같다() {
    let 방 = 방::new("purge");
    assert!(!방.자리(".palimpsest").exists(), "전제 — `.palimpsest/` 가 없던 방이다");
    let s0 = 방.워킹트리();
    let h0 = 방.밖();

    흐름(&방);

    let out = 방.돌린다(&["uninstall", "--purge"]);
    assert!(out.status.success(), "{}", 문자열(&out));
    let 워킹트리_갈림 = 갈린_경로(&s0, &방.워킹트리());
    assert!(워킹트리_갈림.is_empty(), "워킹트리가 설치 전과 갈렸다: {워킹트리_갈림:#?}\n{}", 문자열(&out));
    let 밖_갈림 = 갈린_경로(&h0, &방.밖());
    assert!(밖_갈림.is_empty(), "HOME 이 설치 전과 갈렸다: {밖_갈림:#?}\n{}", 문자열(&out));
}

#[test]
fn g1_같은_흐름의_기본_uninstall_은_갈림이_l_안에만_있고_정본이_직전_바이트다() {
    let 방 = 방::new("default");
    let s0 = 방.워킹트리();
    let h0 = 방.밖();

    let 흔적 = 흐름(&방);
    let store = 방.저장소();
    // uninstall 직전 — `L` 의 정본 자리를 댈 바이트.
    let s1 = 방.워킹트리();
    let h1 = 방.밖();

    let out = 방.돌린다(&["uninstall"]);
    assert!(out.status.success(), "{}", 문자열(&out));
    let s2 = 방.워킹트리();
    let h2 = 방.밖();

    // ① 갈림은 `L` 안에만 있다.
    for rel in 갈린_경로(&s0, &s2) {
        assert!(l_워킹트리인가(&rel), "워킹트리의 갈림 `{rel}` 이 허용 목록 L 밖이다\n{}", 문자열(&out));
    }
    let 밖의_l = l_밖(&방, &흔적);
    for rel in 갈린_경로(&h0, &h2) {
        assert!(밖의_l.contains(&rel), "밖의 갈림 `{rel}` 이 허용 목록 L 밖이다\n{}", 문자열(&out));
    }

    // ② `L` 의 정본 자리는 uninstall 직전 바이트 그대로 남는다.
    let 남은_정본: Vec<&String> = s1.keys().filter(|k| 정본_자리인가(k)).collect();
    assert!(
        남은_정본.len() >= 5,
        "전제 — 잴 정본 자리가 있어야 한다(결박이 든 intent.redb · 회차 둘): {남은_정본:?}"
    );
    for rel in 남은_정본 {
        assert_eq!(s2.get(rel), s1.get(rel), "정본 `{rel}` 이 uninstall 직전 바이트와 다르다\n{}", 문자열(&out));
    }
    for path in 흔적.밖의_정본(&store) {
        let rel = 방.밖_상대(&path);
        assert!(h1.contains_key(&rel), "전제 — 밖의 정본 `{rel}` 이 uninstall 전에 있어야 한다");
        assert_eq!(h2.get(&rel), h1.get(&rel), "밖의 정본 `{rel}` 이 uninstall 직전 바이트와 다르다\n{}", 문자열(&out));
    }

    // ③ 운영 상태는 걷혔다 — 남는 것만 재면 「아무것도 안 걷는 빌드」가 통과한다.
    for path in 흔적.운영_상태(&store) {
        assert!(!path.exists(), "기본 uninstall 이 운영 상태를 남겼다: {}\n{}", path.display(), 문자열(&out));
    }
}
