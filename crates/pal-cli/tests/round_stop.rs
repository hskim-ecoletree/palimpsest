//! Stop activation, read-only policy, semantic progress guard의 black-box 계약.

mod common;

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use common::PAL;
use serde_json::{Value, json};

const SLUG: &str = "fixture-stop";

fn root(tag: &str) -> (PathBuf, PathBuf) {
    let base = std::env::temp_dir().join(format!("pal-round-stop-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    let repo = base.join("repo");
    let store = base.join("private-store");
    std::fs::create_dir_all(repo.join(".palimpsest/rounds").join(SLUG)).expect("round");
    std::fs::create_dir_all(&store).expect("store");
    git(&repo, &["init", "-q"]);
    git(&repo, &["config", "user.email", "fixture@example.invalid"]);
    git(&repo, &["config", "user.name", "Fixture"]);
    std::fs::write(repo.join("tracked.txt"), "tracked\n").expect("tracked");
    let dir = repo.join(".palimpsest/rounds").join(SLUG);
    std::fs::write(
        dir.join("intent.md"),
        "# fixture\n\n## 완수 조건\n\n- [ ] A1 fixture condition\n",
    )
    .expect("intent");
    std::fs::write(
        dir.join("verification.log"),
        format!(
            "{}\n{}\n",
            json!({"kind":"schema","version":1,"round":SLUG}),
            oracle()
        ),
    )
    .expect("ledger");
    git(&repo, &["add", "."]);
    git(&repo, &["commit", "-q", "-m", "fixture"]);
    (repo, store)
}

fn git(repo: &Path, args: &[&str]) {
    let out = Command::new("git").args(args).current_dir(repo).output().expect("git");
    assert!(out.status.success(), "git {args:?}: {}", String::from_utf8_lossy(&out.stderr));
}

fn oracle() -> Value {
    json!({
        "kind":"oracle",
        "id":"A1",
        "mode":"command",
        "check":"true",
        "expect":{"literal":"OK"},
        "cwd":"."
    })
}

fn oracle_digest() -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"pal.round.oracle.v1\0");
    for value in ["command", "true", "literal", "OK", "."] {
        hasher.update(&(value.len() as u64).to_le_bytes());
        hasher.update(value.as_bytes());
    }
    hasher.finalize().to_hex().to_string()
}

fn evidence(met: bool) -> Value {
    json!({
        "kind":"evidence",
        "id":"A1",
        "oracle_digest":oracle_digest(),
        "exit": if met { 0 } else { 1 },
        "matched":met,
        "output_digest":"0".repeat(64),
        "output_bytes":0
    })
}

fn append(repo: &Path, event: &Value) {
    use std::io::Write;
    let path = repo.join(".palimpsest/rounds").join(SLUG).join("verification.log");
    let mut file = std::fs::OpenOptions::new().append(true).open(path).expect("ledger");
    writeln!(file, "{event}").expect("append");
}

fn pal(repo: &Path, store: &Path, args: &[&str]) -> Output {
    Command::new(PAL)
        .args(args)
        .env("PAL_APPROVAL_DIR", store)
        .current_dir(repo)
        .output()
        .expect("pal")
}

fn enable(repo: &Path, store: &Path) -> Value {
    let out = pal(
        repo,
        store,
        &["round", "stop", "enable", "--round", SLUG, "--json"],
    );
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    serde_json::from_slice(&out.stdout).expect("enable JSON")
}

fn stop(repo: &Path, store: &Path, payload: &Value) -> Output {
    use std::io::Write;
    let mut child = Command::new(PAL)
        .args(["hook", "Stop"])
        .env("PAL_APPROVAL_DIR", store)
        .current_dir(repo)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("hook");
    child.stdin.as_mut().expect("stdin").write_all(payload.to_string().as_bytes()).expect("write");
    child.wait_with_output().expect("wait")
}

fn payload(repo: &Path, session: &str, transcript: &Path, active: Value) -> Value {
    json!({
        "session_id":session,
        "transcript_path":transcript,
        "cwd":repo,
        "hook_event_name":"Stop",
        "stop_hook_active":active,
        "last_assistant_message":"done"
    })
}

fn blocked(out: &Output) -> bool {
    serde_json::from_slice::<Value>(&out.stdout)
        .ok()
        .is_some_and(|value| value["decision"] == "block")
}

#[test]
fn 등록과_activation은_분리되고_disable은_즉시_복구한다() {
    let (repo, store) = root("activation");
    let transcript = repo.parent().expect("base").join("transcript.jsonl");
    std::fs::write(&transcript, "inactive\n").expect("transcript");
    let p = payload(&repo, "s1", &transcript, json!(false));
    assert!(!blocked(&stop(&repo, &store, &p)), "등록 전제가 정책 활성으로 새었다");

    let activation = enable(&repo, &store);
    assert_eq!(activation["outcome"], "enabled");
    assert!(blocked(&stop(&repo, &store, &p)), "pending round를 차단하지 않았다");

    let out = pal(&repo, &store, &["round", "stop", "disable", "--json"]);
    assert!(out.status.success());
    assert!(!blocked(&stop(&repo, &store, &p)), "disable 뒤에도 차단했다");
}

#[test]
fn reentry와_unknown은_항상_먼저_통과하고_active_malformed는_차단한다() {
    let (repo, store) = root("dispatch");
    enable(&repo, &store);
    let missing = repo.parent().expect("base").join("missing-transcript");

    let reentry = payload(&repo, "s1", &missing, json!(true));
    assert!(!blocked(&stop(&repo, &store, &reentry)), "reentry가 상태 읽기보다 뒤에 섰다");

    for active in [Value::Null, json!("false"), json!(0)] {
        let out = stop(&repo, &store, &payload(&repo, "s2", &missing, active));
        assert!(blocked(&out), "active Stop의 잘못된 타입을 통과시켰다");
    }
    let mut absent = payload(&repo, "s3", &missing, json!(false));
    absent.as_object_mut().expect("object").remove("stop_hook_active");
    assert!(blocked(&stop(&repo, &store, &absent)));

    let unknown = pal(&repo, &store, &["hook", "Unknown"]);
    assert!(unknown.status.success());
    assert!(unknown.stdout.is_empty());
}

#[test]
fn 의미_진행만_counter를_reset하고_complete만_통과한다() {
    let (repo, store) = root("progress");
    enable(&repo, &store);
    let transcript = repo.parent().expect("base").join("transcript.jsonl");

    for n in 1..=2 {
        std::fs::write(&transcript, format!("attempt {n}\n")).expect("transcript");
        assert!(blocked(&stop(
            &repo,
            &store,
            &payload(&repo, &format!("s{n}"), &transcript, json!(false)),
        )));
    }

    // JSON 표현만 바꾼다. reducer의 의미는 같다.
    let ledger = repo.join(".palimpsest/rounds").join(SLUG).join("verification.log");
    let formatted = format!("{}\n{}\n", serde_json::to_string_pretty(&json!({"kind":"schema","version":1,"round":SLUG})).expect("json").replace('\n', " "), serde_json::to_string_pretty(&oracle()).expect("json").replace('\n', " "));
    std::fs::write(&ledger, formatted).expect("rewrite");
    std::fs::write(&transcript, "format-only\n").expect("transcript");
    assert!(blocked(&stop(&repo, &store, &payload(&repo, "s3", &transcript, json!(false)))));

    append(&repo, &evidence(true));
    std::fs::write(&transcript, "met\n").expect("transcript");
    assert!(blocked(&stop(&repo, &store, &payload(&repo, "s4", &transcript, json!(false)))), "report 없는 met round를 통과시켰다");

    std::fs::write(repo.join(".palimpsest/rounds").join(SLUG).join("report.md"), "# report\n").expect("report");
    std::fs::write(&transcript, "reported\n").expect("transcript");
    assert!(!blocked(&stop(&repo, &store, &payload(&repo, "s5", &transcript, json!(false)))), "완료 상태를 통과시키지 않았다");
}

#[test]
fn replay는_한번만_세고_여섯_무진행에서_truthful_handoff로_끝난다() {
    let (repo, store) = root("cap");
    enable(&repo, &store);
    let transcript = repo.parent().expect("base").join("transcript.jsonl");
    let before = std::fs::read(repo.join(".palimpsest/rounds").join(SLUG).join("verification.log")).expect("before");

    std::fs::write(&transcript, "same\n").expect("transcript");
    let same = payload(&repo, "same-session", &transcript, json!(false));
    for _ in 0..3 {
        assert!(blocked(&stop(&repo, &store, &same)), "replay만으로 상한에 닿았다");
    }
    for n in 2..=5 {
        std::fs::write(&transcript, format!("attempt {n}\n")).expect("transcript");
        assert!(blocked(&stop(&repo, &store, &payload(&repo, "same-session", &transcript, json!(false)))));
    }
    std::fs::write(&transcript, "attempt 6\n").expect("transcript");
    let released = stop(&repo, &store, &payload(&repo, "same-session", &transcript, json!(false)));
    assert!(!blocked(&released), "6회 무진행에서 session을 풀지 않았다");

    let status = pal(&repo, &store, &["round", "stop", "status", "--json"]);
    let state: Value = serde_json::from_slice(&status.stdout).expect("status JSON");
    assert_eq!(state["handoff"], "blocked");
    assert_eq!(state["no_progress"], 6);
    assert_eq!(before, std::fs::read(repo.join(".palimpsest/rounds").join(SLUG).join("verification.log")).expect("after"));
    assert!(!repo.join(".palimpsest/rounds").join(SLUG).join("report.md").exists());
}

#[test]
fn 손상된_active_round는_block하고_손상_activation도_disable된다() {
    let (repo, store) = root("corrupt");
    enable(&repo, &store);
    let transcript = repo.parent().expect("base").join("transcript.jsonl");
    std::fs::write(&transcript, "corrupt\n").expect("transcript");
    std::fs::write(repo.join(".palimpsest/rounds").join(SLUG).join("verification.log"), "{\n").expect("corrupt ledger");
    assert!(blocked(&stop(&repo, &store, &payload(&repo, "s", &transcript, json!(false)))));

    let status = pal(&repo, &store, &["round", "stop", "status", "--json"]);
    let state: Value = serde_json::from_slice(&status.stdout).expect("status JSON");
    let activation_path = PathBuf::from(state["activation_record"].as_str().expect("record path"));
    std::fs::write(&activation_path, "{").expect("corrupt activation");
    let disabled = pal(&repo, &store, &["round", "stop", "disable", "--json"]);
    assert!(disabled.status.success(), "{}", String::from_utf8_lossy(&disabled.stderr));
    assert!(!blocked(&stop(&repo, &store, &payload(&repo, "s2", &transcript, json!(false)))));
}
