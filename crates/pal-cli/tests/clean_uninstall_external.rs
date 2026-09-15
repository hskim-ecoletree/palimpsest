//! **프로젝트 밖 저장소의 이 프로젝트 몫** — 회차 `2026-09-15-clean-uninstall` 조건 D1 · D2 · D3 · D4 · B4 ③(봉인).
//!
//! # 저장소 자리
//!
//! 기본 저장소 자리를 그대로 쓴다(`PAL_APPROVAL_DIR` 를 지운다).
//! - macOS · Linux: 격리 `HOME`(Linux 는 `XDG_DATA_HOME` 도 지운다) — 스냅샷은 **HOME 전체**.
//! - Windows: 실제 `%LOCALAPPDATA%\palimpsest` — 격리할 수 없으므로 CI(`CI` 환경 변수)에서만 돌고
//!   **시작 전에 없음을 단언**한다. 같은 자리를 나눠 쓰므로 이 바이너리의 시험은 한 줄로 돈다.
//!
//! # 모든 스냅샷 시험은 대상이 실제로 생겼음을 uninstall 전에 단언한다
//!
//! 안 생긴 것을 안 남았다고 세지 않는다(사전부검 R1 3·8).

mod common;

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::sync::{Mutex, MutexGuard};

use common::{PAL, snapshot};
use serde_json::{Value, json};

const 열린: &str = "open-round";
const 닫힌: &str = "sealed-round";

/// Windows 는 실제 사용자 자리를 나눠 쓴다 — 시험끼리 겹치면 「시작 전에 없음」이 깨진다.
static 자리_잠금: Mutex<()> = Mutex::new(());

struct 방 {
    base: PathBuf,
    home: PathBuf,
    _잠금: Option<MutexGuard<'static, ()>>,
}

impl 방 {
    fn new(tag: &str) -> Self {
        let 잠금 = if cfg!(windows) {
            Some(자리_잠금.lock().unwrap_or_else(std::sync::PoisonError::into_inner))
        } else {
            None
        };
        let base = std::env::temp_dir().join(format!("pal-cu-ext-{tag}-{}", std::process::id()));
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
        Self { base, home, _잠금: 잠금 }
    }

    /// 스냅샷의 뿌리 — macOS·Linux 는 HOME 전체, Windows 는 `%LOCALAPPDATA%\palimpsest`.
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

    fn 스냅샷(&self) -> std::collections::BTreeMap<String, snapshot::항목> {
        snapshot::트리(&self.뿌리())
    }

    fn 상대(&self, path: &Path) -> String {
        common::상대_경로(&self.뿌리(), path)
    }

    fn 명령(&self, cwd: &Path) -> Command {
        let mut command = Command::new(PAL);
        command.current_dir(cwd).env_remove("PAL_APPROVAL_DIR");
        if cfg!(unix) {
            command.env("HOME", &self.home).env_remove("XDG_DATA_HOME");
        }
        command
    }

    fn pal(&self, cwd: &Path, args: &[&str]) -> Output {
        self.명령(cwd).args(args).output().expect("pal")
    }

    fn 성공(&self, cwd: &Path, args: &[&str]) -> String {
        let out = self.pal(cwd, args);
        assert!(
            out.status.success(),
            "pal {args:?} rc={:?}\nstdout: {}\nstderr: {}",
            out.status.code(),
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
        String::from_utf8(out.stdout).expect("UTF-8")
    }

    /// 저장소 하나 — origin 이 식별자를 정한다. 회차 둘(열린 · 봉인할)을 워킹트리에 쓰고 **커밋하지 않는다**.
    fn 프로젝트(&self, name: &str, origin: &str) -> PathBuf {
        let repo = self.base.join(name);
        std::fs::create_dir_all(&repo).expect("repo");
        git(&repo, &["init", "-q"]);
        git(&repo, &["config", "user.email", "fixture@example.invalid"]);
        git(&repo, &["config", "user.name", "Fixture"]);
        git(&repo, &["remote", "add", "origin", origin]);
        std::fs::write(repo.join("tracked.txt"), "tracked\n").expect("tracked");
        git(&repo, &["add", "tracked.txt"]);
        git(&repo, &["commit", "-q", "-m", "fixture"]);
        회차를_쓴다(&repo);
        repo
    }

    /// install → 명령 승인 → 종료 봉인 → Stop 활성화 → 읽을 수 있는 transcript 로 Stop 훅.
    /// 걸음마다 대상이 생겼음을 단언한다.
    fn 쌓는다(&self, repo: &Path) -> 기록 {
        self.성공(repo, &["install"]);
        let approved: Value =
            serde_json::from_str(&self.성공(repo, &["round", "approve", "--round", 열린, "--id", "A1", "--json"]))
                .expect("approve JSON");
        let 승인 = approved["approval_digest"].as_str().expect("approval_digest").to_owned();
        self.성공(repo, &["round", "verify", "--round", 닫힌, "--all", "--json"]);
        let 봉인 = 봉인_digest(repo);
        let enabled: Value =
            serde_json::from_str(&self.성공(repo, &["round", "stop", "enable", "--round", 열린, "--json"]))
                .expect("enable JSON");
        let 활성화_digest = enabled["activation_digest"].as_str().expect("activation_digest").to_owned();
        let name = repo.file_name().expect("name").to_string_lossy().into_owned();
        let transcript = self.base.join(format!("{name}-transcript.jsonl"));
        std::fs::write(&transcript, format!("{name}\n")).expect("transcript");
        let hook = self.stop_훅(repo, &transcript);
        assert!(
            String::from_utf8_lossy(&hook.stdout).contains("\"block\""),
            "Stop 훅이 진행 파일을 쓰는 자리(차단)까지 가지 않았다: {} / {}",
            String::from_utf8_lossy(&hook.stdout),
            String::from_utf8_lossy(&hook.stderr)
        );
        let 식별자 = origin_식별자(&git_출력(repo, &["config", "--get", "remote.origin.url"]));
        let 기록 = 기록 { 식별자, 승인, 봉인, 활성화_digest };
        let store = self.저장소();
        for path in [
            기록.승인_json(&store),
            기록.봉인_json(&store),
            기록.활성화(&store),
            기록.진행(&store),
            기록.잠금(&store),
        ] {
            assert!(path.is_file(), "생겼어야 할 밖의 기록이 없다: {}", path.display());
        }
        기록
    }

    fn stop_훅(&self, repo: &Path, transcript: &Path) -> Output {
        use std::io::Write;
        let payload = json!({
            "session_id": "clean-uninstall",
            "transcript_path": transcript,
            "cwd": repo,
            "hook_event_name": "Stop",
            "stop_hook_active": false,
            "last_assistant_message": "done"
        });
        let mut child = self
            .명령(repo)
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

struct 기록 {
    식별자: String,
    승인: String,
    봉인: String,
    활성화_digest: String,
}

impl 기록 {
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
    fn 활성화(&self, store: &Path) -> PathBuf {
        store.join(format!("round-stop-activation-{}.json", self.식별자))
    }
    fn 진행(&self, store: &Path) -> PathBuf {
        store.join(format!("round-stop-progress-{}.json", self.활성화_digest))
    }
    fn 잠금(&self, store: &Path) -> PathBuf {
        store.join(format!("round-stop-progress-{}.lock", self.활성화_digest))
    }
    fn 진행_표시(&self, store: &Path) -> PathBuf {
        store.join(format!("round-stop-progress-{}.project", self.활성화_digest))
    }
    /// 기본 uninstall 이 걷는 운영 상태.
    fn 운영_상태(&self, store: &Path) -> Vec<PathBuf> {
        vec![self.활성화(store), self.진행(store), self.잠금(store), self.진행_표시(store)]
    }
    /// `--purge` 가 더 걷는 정본.
    fn 정본(&self, store: &Path) -> Vec<PathBuf> {
        vec![self.승인_json(store), self.승인_표시(store), self.봉인_json(store), self.봉인_표시(store)]
    }
}

fn 회차를_쓴다(repo: &Path) {
    let open = repo.join(".palimpsest/rounds").join(열린);
    std::fs::create_dir_all(&open).expect("open round");
    std::fs::write(open.join("intent.md"), "# fixture\n\n## 완수 조건\n\n- [ ] A1 condition A1\n").expect("intent");
    std::fs::write(
        open.join("verification.log"),
        format!(
            "{}\n{}\n",
            json!({"kind":"schema","version":2,"round":열린}),
            json!({"kind":"oracle","id":"A1","mode":"command","check":"echo ROUND_OK","expect":{"literal":"ROUND_OK"},"cwd":"."})
        ),
    )
    .expect("open ledger");

    let sealed = repo.join(".palimpsest/rounds").join(닫힌);
    std::fs::create_dir_all(&sealed).expect("sealed round");
    std::fs::write(sealed.join("intent.md"), "# fixture\n\n## 완수 조건\n\n- [ ] D1 condition D1\n").expect("intent");
    for name in ["thesis.md", "antithesis.md", "synthesis.md"] {
        std::fs::write(repo.join(name), format!("{name}\n")).expect("dialectic ref");
    }
    let reference = |name: &str| {
        json!({
            "path": name,
            "digest": blake3::hash(&std::fs::read(repo.join(name)).expect("ref")).to_hex().to_string()
        })
    };
    std::fs::write(
        sealed.join("verification.log"),
        format!(
            "{}\n{}\n",
            json!({"kind":"schema","version":3,"round":닫힌}),
            json!({
                "kind":"judgment", "id":"D1", "verdict":"met",
                "thesis":reference("thesis.md"),
                "antithesis":reference("antithesis.md"),
                "synthesis":reference("synthesis.md")
            })
        ),
    )
    .expect("sealed ledger");
    std::fs::write(
        sealed.join("findings.jsonl"),
        format!("{}\n", json!({"schema_version":3,"종류":"레코드","회차":닫힌})),
    )
    .expect("findings");
    std::fs::write(
        sealed.join("report.md"),
        "# report\n\n## 남지 않은 것\n없음.\n\n## 다음 회차가 받는 것\n없음.\n\n## 범위 밖\n없음.\n\n## 원리상 못 잰 것\n없음.\n\n## 능력 부재\n없음.\n",
    )
    .expect("report");
}

fn 봉인_digest(repo: &Path) -> String {
    std::fs::read_to_string(repo.join(".palimpsest/rounds").join(닫힌).join("verification.log"))
        .expect("ledger")
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

fn 갈린_집합(
    앞: &std::collections::BTreeMap<String, snapshot::항목>,
    뒤: &std::collections::BTreeMap<String, snapshot::항목>,
) -> BTreeSet<String> {
    snapshot::갈린_경로(앞, 뒤).into_iter().collect()
}

fn 집합(방: &방, paths: &[PathBuf]) -> BTreeSet<String> {
    paths.iter().map(|p| 방.상대(p)).collect()
}

// ─────────────────────────────────────────────────────────────────────────────
// D1 — v1 바이트 그대로 · 옆에 표시 파일 · 표시 파일 없이도 읽힌다
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn d1_기록은_v1_바이트_그대로이고_옆에_표시_파일이_있으며_표시_파일_없이도_verify_met_status_complete() {
    let 방 = 방::new("d1");
    let repo = 방.프로젝트("x", "https://example.invalid/d1-x.git");
    let 기록 = 방.쌓는다(&repo);
    let store = 방.저장소();

    // 착수 커밋의 v1 직렬화 — `{"version":1,"digest":"…"}` + LF.
    for digest in [&기록.승인, &기록.봉인] {
        let bytes = std::fs::read(store.join(format!("{digest}.json"))).expect("v1 record");
        assert_eq!(
            String::from_utf8_lossy(&bytes),
            format!("{{\"version\":1,\"digest\":\"{digest}\"}}\n"),
            "v1 직렬화가 바이트로 달라졌다"
        );
    }
    let 표시 = format!("{}\n", 기록.식별자);
    for path in [기록.승인_표시(&store), 기록.봉인_표시(&store), 기록.진행_표시(&store)] {
        assert_eq!(
            std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("표시 파일이 없다 {}: {e}", path.display())),
            표시,
            "표시 파일의 프로젝트 식별자가 다르다: {}",
            path.display()
        );
    }

    // 표시 파일 없는 v1 승인으로 verify 가 met.
    std::fs::remove_file(기록.승인_표시(&store)).expect("승인 표시 파일 지우기");
    let verified: Value =
        serde_json::from_str(&방.성공(&repo, &["round", "verify", "--round", 열린, "--id", "A1", "--json"]))
            .expect("verify JSON");
    assert_eq!(verified["met"], true, "표시 파일 없는 v1 승인으로 verify 가 met 이 아니다: {verified}");
    assert!(!기록.승인_표시(&store).exists(), "verify 가 표시 파일을 되살렸다 — 읽기만 재야 한다");

    // 표시 파일 없는 v1 봉인으로 status 가 Complete.
    std::fs::remove_file(기록.봉인_표시(&store)).expect("봉인 표시 파일 지우기");
    let status: Value =
        serde_json::from_str(&방.성공(&repo, &["round", "status", "--round", 닫힌, "--json"])).expect("status JSON");
    assert_eq!(status["completion"], "complete", "표시 파일 없는 v1 봉인을 Complete 로 안 읽었다: {status}");
}

#[test]
fn d1_음성_대조_진행_파일의_표시가_다른_프로젝트면_기본_uninstall_이_안_걷는다() {
    let 방 = 방::new("d1-neg-progress");
    let repo = 방.프로젝트("x", "https://example.invalid/d1-neg-x.git");
    let 기록 = 방.쌓는다(&repo);
    let store = 방.저장소();
    let 남의 = format!("{}\n", "e".repeat(64));
    std::fs::write(기록.진행_표시(&store), &남의).expect("다른 프로젝트 표시");

    let 앞 = 방.스냅샷();
    let out = 방.pal(&repo, &["uninstall"]);
    assert!(out.status.success(), "{}", 문자열(&out));
    let 뒤 = 방.스냅샷();
    assert_eq!(
        갈린_집합(&앞, &뒤),
        집합(&방, &[기록.활성화(&store)]),
        "다른 프로젝트 표시가 붙은 진행 파일 · 잠금이 이 프로젝트 몫으로 걷혔다(또는 활성화가 안 걷혔다)\n{}",
        문자열(&out)
    );
    assert!(
        !String::from_utf8_lossy(&out.stdout).contains("가를 수 없다(밖)"),
        "표시 파일로 가른 기록을 가를 수 없는 것으로 셌다\n{}",
        문자열(&out)
    );
}

#[test]
fn d1_음성_대조_승인의_표시가_다른_프로젝트면_purge_가_안_걷는다() {
    let 방 = 방::new("d1-neg-approval");
    let repo = 방.프로젝트("x", "https://example.invalid/d1-neg-a.git");
    let 기록 = 방.쌓는다(&repo);
    let store = 방.저장소();
    let 남의 = format!("{}\n", "e".repeat(64));
    std::fs::write(기록.승인_표시(&store), &남의).expect("다른 프로젝트 표시");
    let 승인_바이트 = std::fs::read(기록.승인_json(&store)).expect("승인");

    let out = 방.pal(&repo, &["uninstall", "--purge"]);
    assert!(out.status.success(), "{}", 문자열(&out));
    assert_eq!(std::fs::read(기록.승인_json(&store)).ok(), Some(승인_바이트), "다른 프로젝트 표시가 붙은 승인을 걷었다");
    assert_eq!(std::fs::read_to_string(기록.승인_표시(&store)).ok(), Some(남의), "다른 프로젝트의 표시 파일을 걷었다");
    assert!(!기록.봉인_json(&store).exists(), "이 프로젝트의 봉인은 걷혀야 한다 — 걷기 자체가 안 돌았다");
}

// ─────────────────────────────────────────────────────────────────────────────
// D2 — 프로젝트 둘 · 기본 저장소 자리
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn d2_기본_uninstall_은_x_의_활성화_진행_잠금과_표시만_걷고_나머지는_바이트_그대로다() {
    let 방 = 방::new("d2-default");
    let x_repo = 방.프로젝트("x", "https://example.invalid/d2-x.git");
    let y_repo = 방.프로젝트("y", "https://example.invalid/d2-y.git");
    let 기록x = 방.쌓는다(&x_repo);
    let _기록y = 방.쌓는다(&y_repo);
    let store = 방.저장소();

    let 걷기_전 = 방.스냅샷();
    let out = 방.pal(&x_repo, &["uninstall"]);
    assert!(out.status.success(), "{}", 문자열(&out));
    let 걷은_뒤 = 방.스냅샷();
    assert_eq!(
        갈린_집합(&걷기_전, &걷은_뒤),
        집합(&방, &기록x.운영_상태(&store)),
        "X 기본 uninstall 이 X 의 운영 상태만 걷지 않았다\n{}",
        문자열(&out)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("같은 원격의 다른 체크아웃도 Stop 정책이 비활성화된다"),
        "활성화를 걷고 같은 원격의 다른 체크아웃 안내를 출력하지 않았다\n{stdout}"
    );
    for p in 기록x.정본(&store) {
        assert!(stdout.contains(&p.display().to_string()), "남긴 정본의 경로를 출력하지 않았다: {}\n{stdout}", p.display());
    }
}

fn d2_purge_흐름(tag: &str, 조상_미리: bool) {
    let 방 = 방::new(tag);
    if 조상_미리 {
        std::fs::create_dir_all(방.조상()).expect("빈 조상");
    }
    let 설치_전 = 방.스냅샷();
    let x = 방.프로젝트("x", &format!("https://example.invalid/{tag}-x.git"));
    let y = 방.프로젝트("y", &format!("https://example.invalid/{tag}-y.git"));
    let 기록x = 방.쌓는다(&x);
    let 기록y = 방.쌓는다(&y);
    let store = 방.저장소();
    for p in [기록x.승인_표시(&store), 기록x.봉인_표시(&store), 기록x.진행_표시(&store)] {
        assert!(p.is_file(), "생겼어야 할 표시 파일이 없다: {}", p.display());
    }

    let s1 = 방.스냅샷();
    let out = 방.pal(&x, &["uninstall"]);
    assert!(out.status.success(), "{}", 문자열(&out));
    let s2 = 방.스냅샷();
    assert_eq!(갈린_집합(&s1, &s2), 집합(&방, &기록x.운영_상태(&store)), "X 기본\n{}", 문자열(&out));

    방.성공(&x, &["install"]);
    assert!(갈린_집합(&s2, &방.스냅샷()).is_empty(), "다시 install 이 밖을 건드렸다");
    let out = 방.pal(&x, &["uninstall", "--purge"]);
    assert!(out.status.success(), "{}", 문자열(&out));
    let s3 = 방.스냅샷();
    assert_eq!(
        갈린_집합(&s2, &s3),
        집합(&방, &기록x.정본(&store)),
        "X --purge 가 X 의 승인 · 봉인(표시 파일 포함)만 걷지 않았다\n{}",
        문자열(&out)
    );
    for p in 기록y.운영_상태(&store).into_iter().chain(기록y.정본(&store)) {
        assert!(p.is_file(), "Y 의 기록이 사라졌다: {}", p.display());
    }

    let out = 방.pal(&y, &["uninstall", "--purge"]);
    assert!(out.status.success(), "{}", 문자열(&out));
    let 끝 = 방.스냅샷();
    assert!(
        갈린_집합(&설치_전, &끝).is_empty(),
        "둘 다 --purge 한 뒤 HOME 스냅샷이 설치 전과 다르다: {:?}\n{}",
        갈린_집합(&설치_전, &끝),
        문자열(&out)
    );
}

#[test]
fn d2_purge_둘_다_걷으면_home_이_설치_전과_같다_조상을_안_만든_home() {
    d2_purge_흐름("d2-purge-bare", false);
}

#[test]
fn d2_purge_둘_다_걷으면_home_이_설치_전과_같다_빈_조상을_미리_만든_home() {
    d2_purge_흐름("d2-purge-ancestors", true);
}

#[test]
fn d2_봉인의_표시_파일을_미리_지운_방에서도_purge_가_원장으로_봉인을_걷는다() {
    let 방 = 방::new("d2-seal-ledger");
    let 설치_전 = 방.스냅샷();
    let x = 방.프로젝트("x", "https://example.invalid/d2-seal-x.git");
    let 기록 = 방.쌓는다(&x);
    let store = 방.저장소();
    std::fs::remove_file(기록.봉인_표시(&store)).expect("봉인 표시 파일 지우기");

    let out = 방.pal(&x, &["uninstall", "--purge"]);
    assert!(out.status.success(), "{}", 문자열(&out));
    assert!(!기록.봉인_json(&store).exists(), "표시 파일이 사라진 봉인을 원장으로 가르지 못했다\n{}", 문자열(&out));
    let 끝 = 방.스냅샷();
    assert!(갈린_집합(&설치_전, &끝).is_empty(), "{:?}\n{}", 갈린_집합(&설치_전, &끝), 문자열(&out));
}

// ─────────────────────────────────────────────────────────────────────────────
// D3 — 가를 수 없는 것
// ─────────────────────────────────────────────────────────────────────────────

struct D3방 {
    방: 방,
    x: PathBuf,
    기록y: 기록,
    둘: [PathBuf; 2],
}

fn d3_방(tag: &str) -> D3방 {
    let 방 = 방::new(tag);
    let x = 방.프로젝트("x", &format!("https://example.invalid/{tag}-x.git"));
    let y = 방.프로젝트("y", &format!("https://example.invalid/{tag}-y.git"));
    방.쌓는다(&x);
    let 기록y = 방.쌓는다(&y);
    let store = 방.저장소();
    let 승인 = blake3::hash(b"d3 unattributable approval").to_hex().to_string();
    let 진행 = blake3::hash(b"d3 unattributable progress").to_hex().to_string();
    let 둘 = [
        store.join(format!("{승인}.json")),
        store.join(format!("round-stop-progress-{진행}.json")),
    ];
    std::fs::write(&둘[0], format!("{{\"version\":1,\"digest\":\"{승인}\"}}\n")).expect("표시 없는 승인");
    std::fs::write(&둘[1], "{}\n").expect("표시 없는 진행 파일");
    D3방 { 방, x, 기록y, 둘 }
}

fn d3_판정(d3: &D3방, args: &[&str]) {
    let 앞: Vec<Vec<u8>> = d3.둘.iter().map(|p| std::fs::read(p).expect("둘")).collect();
    let out = d3.방.pal(&d3.x, args);
    assert!(out.status.success(), "{}", 문자열(&out));
    let stdout = String::from_utf8_lossy(&out.stdout);
    let 줄: Vec<&str> = stdout.lines().filter(|l| l.contains("가를 수 없다(밖)")).collect();
    assert!(
        줄.iter().any(|l| l.contains("2개")),
        "가를 수 없는 것의 수가 2 가 아니다\n{stdout}"
    );
    assert_eq!(줄.len(), 3, "가를 수 없다 줄은 수 한 줄 + 경로 둘이어야 한다: {줄:#?}");
    for p in &d3.둘 {
        assert!(줄.iter().any(|l| l.contains(&p.display().to_string())), "경로가 없다: {}\n{stdout}", p.display());
    }
    let 자리 = d3.방.저장소().display().to_string();
    assert!(
        stdout.lines().any(|l| l.contains("들여다본 자리(밖)") && l.contains(&자리)),
        "들여다본 저장소 자리를 출력하지 않았다({자리})\n{stdout}"
    );
    for (p, bytes) in d3.둘.iter().zip(앞) {
        assert_eq!(std::fs::read(p).ok(), Some(bytes), "가를 수 없는 것을 지웠다: {}", p.display());
    }
    let store = d3.방.저장소();
    for p in d3.기록y.운영_상태(&store).into_iter().chain(d3.기록y.정본(&store)) {
        assert!(p.is_file(), "Y 의 기록이 사라졌다: {}", p.display());
        assert!(!줄.iter().any(|l| l.contains(&p.display().to_string())), "Y 의 기록을 가를 수 없다로 셌다");
    }
}

#[test]
fn d3_기본_uninstall_은_표시_없는_승인과_slug_에_안_맞는_진행_파일만_가를_수_없다로_헤아린다() {
    let d3 = d3_방("d3-default");
    d3_판정(&d3, &["uninstall"]);
}

#[test]
fn d3_purge_도_표시_없는_승인과_slug_에_안_맞는_진행_파일만_가를_수_없다로_헤아린다() {
    let d3 = d3_방("d3-purge");
    d3_판정(&d3, &["uninstall", "--purge"]);
}

// ─────────────────────────────────────────────────────────────────────────────
// D4 — 같은 저장소의 다른 worktree · 같은 origin 의 별도 클론
// ─────────────────────────────────────────────────────────────────────────────

fn d4_worktree_방(tag: &str) -> (방, PathBuf, PathBuf) {
    let 방 = 방::new(tag);
    let main = 방.프로젝트("main-checkout", &format!("https://example.invalid/{tag}.git"));
    let 기록 = 방.쌓는다(&main);
    let store = 방.저장소();
    for p in [기록.활성화(&store), 기록.진행(&store), 기록.잠금(&store), 기록.승인_json(&store), 기록.봉인_json(&store)] {
        assert!(p.is_file(), "밖에 이 프로젝트의 기록이 있어야 한다: {}", p.display());
    }
    let linked = 방.base.join("linked-wt");
    git(&main, &["worktree", "add", "-q", linked.to_str().expect("path")]);
    (방, main, linked)
}

fn d4_거부_판정(방: &방, cwd: &Path, args: &[&str], rc: i32, 다른_쪽: &str) {
    let 앞 = 방.스냅샷();
    let out = 방.pal(cwd, args);
    let 뒤 = 방.스냅샷();
    assert_eq!(out.status.code(), Some(rc), "{}", 문자열(&out));
    assert!(갈린_집합(&앞, &뒤).is_empty(), "worktree 가 있는데 밖을 건드렸다: {:?}\n{}", 갈린_집합(&앞, &뒤), 문자열(&out));
    let all = 문자열(&out);
    assert!(
        all.lines().any(|l| l.contains(다른_쪽) && l.contains("worktree")),
        "다른 worktree 경로({다른_쪽})를 출력하지 않았다\n{all}"
    );
    assert!(all.contains("하나도 건드리지 않았다"), "까닭을 출력하지 않았다\n{all}");
}

#[test]
fn d4_1_연결된_worktree_가_있으면_주_체크아웃의_기본_uninstall_도_밖을_안_건드리고_rc_0() {
    let (방, main, _linked) = d4_worktree_방("d4-1");
    d4_거부_판정(&방, &main, &["uninstall"], 0, "linked-wt");
}

#[test]
fn d4_2_연결된_worktree_가_있으면_주_체크아웃의_purge_는_밖을_안_건드리고_rc_1() {
    let (방, main, _linked) = d4_worktree_방("d4-2");
    d4_거부_판정(&방, &main, &["uninstall", "--purge"], 1, "linked-wt");
}

#[test]
fn d4_3_연결된_worktree_쪽의_purge_도_밖을_안_건드리고_rc_1() {
    let (방, _main, linked) = d4_worktree_방("d4-3");
    방.성공(&linked, &["install"]);
    d4_거부_판정(&방, &linked, &["uninstall", "--purge"], 1, "main-checkout");
}

#[test]
fn d4_4_같은_origin_의_별도_클론_둘은_가를_수_없어_밖을_걷고_경고한다() {
    let 방 = 방::new("d4-4");
    let 설치_전 = 방.스냅샷();
    let origin = "https://example.invalid/d4-4-shared.git";
    let a = 방.프로젝트("clone-a", origin);
    let 기록 = 방.쌓는다(&a);
    let b = 방.base.join("clone-b");
    git(&방.base, &["clone", "-q", a.to_str().expect("path"), b.to_str().expect("path")]);
    git(&b, &["remote", "set-url", "origin", origin]);
    방.성공(&b, &["install"]);

    let out = 방.pal(&b, &["uninstall", "--purge"]);
    assert_eq!(out.status.code(), Some(0), "{}", 문자열(&out));
    let store = 방.저장소();
    for p in 기록.운영_상태(&store).into_iter().chain(기록.정본(&store)) {
        assert!(!p.exists(), "같은 origin 의 다른 클론 기록이 남았다: {}\n{}", p.display(), 문자열(&out));
    }
    assert!(
        String::from_utf8_lossy(&out.stdout).contains("같은 원격의 다른 클론의 기록도 함께 지웠다"),
        "클론 경고를 출력하지 않았다\n{}",
        문자열(&out)
    );
    assert!(갈린_집합(&설치_전, &방.스냅샷()).is_empty(), "{:?}", 갈린_집합(&설치_전, &방.스냅샷()));
}

// ─────────────────────────────────────────────────────────────────────────────
// B4 ③ — 추적 중인 회차 원장에 적힌 봉인은 --purge 도 남긴다
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn b4_3_추적_중인_회차_원장에_적힌_봉인은_purge_도_남기고_재설치_뒤_complete() {
    let 방 = 방::new("b4-3");
    let x = 방.프로젝트("x", "https://example.invalid/b4-3.git");
    // 봉인이 읽는 투영은 추적 파일이다 — 회차 기록을 먼저 커밋하고, 봉인 뒤 원장을 커밋한다.
    git(&x, &["add", ".palimpsest/rounds", "thesis.md", "antithesis.md", "synthesis.md"]);
    git(&x, &["commit", "-q", "-m", "round records"]);
    let 기록 = 방.쌓는다(&x);
    git(&x, &["add", ".palimpsest/rounds"]);
    git(&x, &["commit", "-q", "-m", "seal checkpoint"]);
    let store = 방.저장소();
    let status = |방: &방| -> Value {
        serde_json::from_str(&방.성공(&x, &["round", "status", "--round", 닫힌, "--json"])).expect("status JSON")
    };
    assert_eq!(status(&방)["completion"], "complete", "봉인 직후 Complete 가 아니다");
    let 봉인_바이트 = std::fs::read(기록.봉인_json(&store)).expect("봉인");

    let out = 방.pal(&x, &["uninstall", "--purge"]);
    assert!(out.status.success(), "{}", 문자열(&out));
    assert_eq!(
        std::fs::read(기록.봉인_json(&store)).ok(),
        Some(봉인_바이트),
        "추적 중인 원장에 적힌 봉인을 걷었다\n{}",
        문자열(&out)
    );
    assert!(기록.봉인_표시(&store).is_file(), "남긴 봉인의 표시 파일을 걷었다");
    assert!(!기록.승인_json(&store).exists(), "--purge 가 이 프로젝트의 명령 승인을 안 걷었다 — 걷기 자체가 안 돌았다");

    방.성공(&x, &["install"]);
    assert_eq!(status(&방)["completion"], "complete", "재설치 뒤 남긴 회차가 Complete 로 안 읽힌다");
}
