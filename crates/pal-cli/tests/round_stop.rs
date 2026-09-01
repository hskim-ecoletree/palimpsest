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
    let out = Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .expect("git");
    assert!(
        out.status.success(),
        "git {args:?}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
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
    let path = repo
        .join(".palimpsest/rounds")
        .join(SLUG)
        .join("verification.log");
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open(path)
        .expect("ledger");
    writeln!(file, "{event}").expect("append");
}

fn write_report(repo: &Path) {
    let path = repo
        .join(".palimpsest/rounds")
        .join(SLUG)
        .join("report.md");
    std::fs::write(
        path,
        "# report\n\n## 남지 않은 것\n없음.\n\n## 다음 회차가 받는 것\n없음.\n\n## 범위 밖\n없음.\n\n## 원리상 못 잰 것\n없음.\n\n## 능력 부재\n없음.\n",
    )
    .expect("report");
}

fn write_folded(repo: &Path) {
    let dir = repo.join(".palimpsest/rounds").join(SLUG);
    std::fs::write(
        dir.join("folded.md"),
        "# 접힘 — fixture\n\n## 왜 접었나\n목표 밖이다.\n\n## 접으면서 남기는 것과 버리는 것\n없음.\n\n## 다음에 여는 것\n없음.\n",
    )
    .expect("folded");
    std::fs::write(
        dir.join("state.md"),
        "# 상태\n\n## 지금 단계\n접힘 — `folded.md`를 본다.\n",
    )
    .expect("state");
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
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
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
    child
        .stdin
        .as_mut()
        .expect("stdin")
        .write_all(payload.to_string().as_bytes())
        .expect("write");
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
    assert!(
        !blocked(&stop(&repo, &store, &p)),
        "등록 전제가 정책 활성으로 새었다"
    );

    let activation = enable(&repo, &store);
    assert_eq!(activation["outcome"], "enabled");
    assert!(
        blocked(&stop(&repo, &store, &p)),
        "pending round를 차단하지 않았다"
    );

    let nested = repo.join("nested/worktree/cwd");
    std::fs::create_dir_all(&nested).expect("nested cwd");
    let mut nested_payload = payload(&repo, "nested", &transcript, json!(false));
    nested_payload["cwd"] = json!(nested);
    assert!(
        blocked(&stop(&repo, &store, &nested_payload)),
        "하위 cwd가 activation을 우회했다"
    );

    let out = pal(&repo, &store, &["round", "stop", "disable", "--json"]);
    assert!(out.status.success());
    assert!(
        !blocked(&stop(&repo, &store, &p)),
        "disable 뒤에도 차단했다"
    );

    enable(&repo, &store);
    let status = pal(&repo, &store, &["round", "stop", "status", "--json"]);
    let state: Value = serde_json::from_slice(&status.stdout).expect("status JSON");
    assert_eq!(
        state["no_progress"], 0,
        "재활성화가 stale counter를 물려받았다"
    );
}

#[test]
fn reentry와_unknown은_항상_먼저_통과하고_active_malformed는_차단한다() {
    let (repo, store) = root("dispatch");
    enable(&repo, &store);
    let missing = repo.parent().expect("base").join("missing-transcript");

    let reentry = payload(&repo, "s1", &missing, json!(true));
    assert!(
        !blocked(&stop(&repo, &store, &reentry)),
        "reentry가 상태 읽기보다 뒤에 섰다"
    );

    for active in [Value::Null, json!("false"), json!(0)] {
        let out = stop(&repo, &store, &payload(&repo, "s2", &missing, active));
        assert!(blocked(&out), "active Stop의 잘못된 타입을 통과시켰다");
    }
    let mut absent = payload(&repo, "s3", &missing, json!(false));
    absent
        .as_object_mut()
        .expect("object")
        .remove("stop_hook_active");
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
    let ledger = repo
        .join(".palimpsest/rounds")
        .join(SLUG)
        .join("verification.log");
    let formatted = format!(
        "{}\n{}\n",
        serde_json::to_string_pretty(&json!({"kind":"schema","version":1,"round":SLUG}))
            .expect("json")
            .replace('\n', " "),
        serde_json::to_string_pretty(&oracle())
            .expect("json")
            .replace('\n', " ")
    );
    std::fs::write(&ledger, formatted).expect("rewrite");
    std::fs::write(&transcript, "format-only\n").expect("transcript");
    assert!(blocked(&stop(
        &repo,
        &store,
        &payload(&repo, "s3", &transcript, json!(false))
    )));

    append(&repo, &evidence(true));
    std::fs::write(&transcript, "met\n").expect("transcript");
    assert!(
        blocked(&stop(
            &repo,
            &store,
            &payload(&repo, "s4", &transcript, json!(false))
        )),
        "report 없는 met round를 통과시켰다"
    );

    let status = pal(&repo, &store, &["round", "stop", "status", "--json"]);
    let state: Value = serde_json::from_slice(&status.stdout).expect("status JSON");
    assert_eq!(state["no_progress"], 0, "의미 진행 뒤 counter가 0이 아니다: {state}");

    std::fs::write(
        repo.join(".palimpsest/rounds").join(SLUG).join("report.md"),
        "# report\n",
    )
    .expect("report");
    std::fs::write(&transcript, "reported\n").expect("transcript");
    assert!(
        blocked(&stop(
            &repo,
            &store,
            &payload(&repo, "s5", &transcript, json!(false))
        )),
        "빈 종료 보고를 완료로 통과시켰다"
    );
    write_report(&repo);
    std::fs::write(&transcript, "reported-valid\n").expect("transcript");
    assert!(
        blocked(&stop(
            &repo,
            &store,
            &payload(&repo, "s6", &transcript, json!(false))
        )),
        "schema 1 성공 기록을 Stop 종료 근거로 통과시켰다"
    );
}

#[test]
fn schema3의_정반합_finding_전수재검증_checkpoint만_complete다() {
    let (repo, store) = root("schema3-complete");
    enable(&repo, &store);
    let dir = repo.join(".palimpsest/rounds").join(SLUG);
    let refs = ["thesis.md", "antithesis.md", "synthesis.md"];
    for name in refs {
        std::fs::write(repo.join(name), format!("{name}\n")).expect("dialectic ref");
    }
    let reference = |name: &str| {
        json!({
            "path": name,
            "digest": blake3::hash(&std::fs::read(repo.join(name)).expect("ref")).to_hex().to_string()
        })
    };
    std::fs::write(
        dir.join("verification.log"),
        format!(
            "{}\n{}\n",
            json!({"kind":"schema","version":3,"round":SLUG}),
            json!({
                "kind":"judgment", "id":"A1", "verdict":"met",
                "thesis":reference("thesis.md"),
                "antithesis":reference("antithesis.md"),
                "synthesis":reference("synthesis.md")
            })
        ),
    )
    .expect("schema3 ledger");
    std::fs::write(
        dir.join("findings.jsonl"),
        format!(
            "{}\n{}\n",
            json!({"schema_version":3,"종류":"레코드","회차":SLUG}),
            json!({
                "id":"F1","라운드":1,"출처":"실측","모집단":"자기장치",
                "유효성":"참","해악도":"금지역","처분":"정정",
                "경로":"tracked.txt","요약":"closed fixture",
                "상태":"닫힘","닫은커밋":"abc1234"
            })
        ),
    )
    .expect("findings");
    write_report(&repo);
    git(&repo, &["add", "."]);
    git(&repo, &["commit", "-q", "-m", "schema3 terminal"]);

    let finalized = pal(
        &repo,
        &store,
        &["round", "verify", "--round", SLUG, "--all", "--json"],
    );
    assert!(
        finalized.status.success(),
        "{}",
        String::from_utf8_lossy(&finalized.stderr)
    );
    let transcript = repo.parent().expect("base").join("transcript.jsonl");
    std::fs::write(&transcript, "complete\n").expect("transcript");
    assert!(
        !blocked(&stop(
            &repo,
            &store,
            &payload(&repo, "complete", &transcript, json!(false))
        )),
        "schema 3 completion checkpoint를 통과시키지 않았다"
    );
}

#[test]
fn 필수_heading만_있고_본문이_빈_report와_folded는_차단한다() {
    let (repo, store) = root("heading-only-terminal");
    enable(&repo, &store);
    let dir = repo.join(".palimpsest/rounds").join(SLUG);
    let transcript = repo.parent().expect("base").join("transcript.jsonl");
    std::fs::write(&transcript, "heading-only\n").expect("transcript");
    std::fs::write(
        dir.join("report.md"),
        "# report\n\n## 남지 않은 것\n\n## 다음 회차가 받는 것\n\n## 범위 밖\n\n## 원리상 못 잰 것\n\n## 능력 부재\n",
    )
    .expect("report");
    let report = stop(
        &repo,
        &store,
        &payload(&repo, "report", &transcript, json!(false)),
    );
    assert!(blocked(&report));
    assert!(String::from_utf8_lossy(&report.stdout).contains("본문이 비었다"));

    std::fs::remove_file(dir.join("report.md")).expect("remove report");
    std::fs::write(
        dir.join("folded.md"),
        "# folded\n\n## 왜 접었나\n\n## 접으면서 남기는 것과 버리는 것\n\n## 다음에 여는 것\n",
    )
    .expect("folded");
    std::fs::write(
        dir.join("state.md"),
        "# state\n\n## 지금 단계\n접힘 — `folded.md`\n",
    )
    .expect("state");
    let folded = stop(
        &repo,
        &store,
        &payload(&repo, "folded", &transcript, json!(false)),
    );
    assert!(blocked(&folded));
    assert!(String::from_utf8_lossy(&folded.stdout).contains("본문이 비었다"));
}

#[test]
fn replay는_한번만_세고_여섯_무진행에서_truthful_handoff로_끝난다() {
    let (repo, store) = root("cap");
    enable(&repo, &store);
    let transcript = repo.parent().expect("base").join("transcript.jsonl");
    let before = std::fs::read(
        repo.join(".palimpsest/rounds")
            .join(SLUG)
            .join("verification.log"),
    )
    .expect("before");

    std::fs::write(&transcript, "same\n").expect("transcript");
    let same = payload(&repo, "same-session", &transcript, json!(false));
    for _ in 0..3 {
        assert!(
            blocked(&stop(&repo, &store, &same)),
            "replay만으로 상한에 닿았다"
        );
    }
    for n in 2..=5 {
        std::fs::write(&transcript, format!("attempt {n}\n")).expect("transcript");
        assert!(blocked(&stop(
            &repo,
            &store,
            &payload(&repo, "same-session", &transcript, json!(false))
        )));
    }
    std::fs::write(&transcript, "attempt 6\n").expect("transcript");
    let released = stop(
        &repo,
        &store,
        &payload(&repo, "same-session", &transcript, json!(false)),
    );
    assert!(!blocked(&released), "6회 무진행에서 session을 풀지 않았다");

    let status = pal(&repo, &store, &["round", "stop", "status", "--json"]);
    let state: Value = serde_json::from_slice(&status.stdout).expect("status JSON");
    assert_eq!(state["handoff"], "blocked");
    assert_eq!(state["no_progress"], 6);
    assert_eq!(
        before,
        std::fs::read(
            repo.join(".palimpsest/rounds")
                .join(SLUG)
                .join("verification.log")
        )
        .expect("after")
    );
    assert!(
        !repo
            .join(".palimpsest/rounds")
            .join(SLUG)
            .join("report.md")
            .exists()
    );
}

#[test]
fn 긴_transcript도_streaming_hash해_자기_상한을_죽이지_않는다() {
    let (repo, store) = root("large-transcript");
    enable(&repo, &store);
    let transcript = repo.parent().expect("base").join("transcript.jsonl");
    std::fs::write(&transcript, vec![b'x'; 8 * 1024 * 1024 + 1]).expect("large transcript");
    for n in 1..=6 {
        let out = stop(
            &repo,
            &store,
            &payload(&repo, &format!("large-{n}"), &transcript, json!(false)),
        );
        assert_eq!(blocked(&out), n < 6, "large transcript attempt {n}");
    }
    let status = pal(&repo, &store, &["round", "stop", "status", "--json"]);
    let state: Value = serde_json::from_slice(&status.stdout).expect("status JSON");
    assert_eq!(state["no_progress"], 6);
    assert_eq!(state["handoff"], "blocked");
}

#[test]
fn 손상된_active_round는_block하고_손상_activation도_disable된다() {
    let (repo, store) = root("corrupt");
    enable(&repo, &store);
    let transcript = repo.parent().expect("base").join("transcript.jsonl");
    std::fs::write(&transcript, "corrupt\n").expect("transcript");
    std::fs::write(
        repo.join(".palimpsest/rounds")
            .join(SLUG)
            .join("verification.log"),
        "{\n",
    )
    .expect("corrupt ledger");
    assert!(blocked(&stop(
        &repo,
        &store,
        &payload(&repo, "s", &transcript, json!(false))
    )));

    let activation_path = std::fs::read_dir(&store)
        .expect("store")
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .find(|path| {
            path.file_name()
                .is_some_and(|name| name.to_string_lossy().starts_with("round-stop-activation-"))
        })
        .expect("activation record");
    std::fs::write(&activation_path, "{").expect("corrupt activation");
    let disabled = pal(&repo, &store, &["round", "stop", "disable", "--json"]);
    assert!(
        disabled.status.success(),
        "{}",
        String::from_utf8_lossy(&disabled.stderr)
    );
    assert!(!blocked(&stop(
        &repo,
        &store,
        &payload(&repo, "s2", &transcript, json!(false))
    )));
}

#[test]
fn unregistered_unmet_stale_없는회차_terminal충돌을_모두_차단하고_folded는_통과한다() {
    let (repo, store) = root("states");
    enable(&repo, &store);
    let transcript = repo.parent().expect("base").join("transcript.jsonl");
    let dir = repo.join(".palimpsest/rounds").join(SLUG);

    std::fs::remove_file(dir.join("verification.log")).expect("remove ledger");
    std::fs::write(&transcript, "unregistered\n").expect("transcript");
    assert!(blocked(&stop(
        &repo,
        &store,
        &payload(&repo, "u", &transcript, json!(false))
    )));

    std::fs::write(
        dir.join("verification.log"),
        format!(
            "{}\n{}\n{}\n",
            json!({"kind":"schema","version":1,"round":SLUG}),
            oracle(),
            evidence(false)
        ),
    )
    .expect("unmet ledger");
    std::fs::write(&transcript, "unmet\n").expect("transcript");
    assert!(blocked(&stop(
        &repo,
        &store,
        &payload(&repo, "m", &transcript, json!(false))
    )));

    append(&repo, &oracle());
    std::fs::write(&transcript, "stale\n").expect("transcript");
    assert!(blocked(&stop(
        &repo,
        &store,
        &payload(&repo, "s", &transcript, json!(false))
    )));

    write_report(&repo);
    std::fs::write(dir.join("folded.md"), "## 왜 접었나\nfixture\n").expect("folded");
    std::fs::write(&transcript, "conflict\n").expect("transcript");
    assert!(blocked(&stop(
        &repo,
        &store,
        &payload(&repo, "c", &transcript, json!(false))
    )));

    std::fs::remove_file(dir.join("report.md")).expect("remove report");
    std::fs::write(&transcript, "folded\n").expect("transcript");
    assert!(
        blocked(&stop(
            &repo,
            &store,
            &payload(&repo, "f0", &transcript, json!(false))
        )),
        "불완전한 folded 종료문을 통과시켰다"
    );
    write_folded(&repo);
    std::fs::write(&transcript, "folded-valid\n").expect("transcript");
    assert!(!blocked(&stop(
        &repo,
        &store,
        &payload(&repo, "f", &transcript, json!(false))
    )));

    std::fs::remove_dir_all(&dir).expect("remove round");
    std::fs::write(&transcript, "missing\n").expect("transcript");
    assert!(blocked(&stop(
        &repo,
        &store,
        &payload(&repo, "x", &transcript, json!(false))
    )));
}

#[test]
fn regression과_진동은_progress_reset이_아니다() {
    let (repo, store) = root("regression");
    enable(&repo, &store);
    let transcript = repo.parent().expect("base").join("transcript.jsonl");

    std::fs::write(&transcript, "pending\n").expect("transcript");
    assert!(blocked(&stop(
        &repo,
        &store,
        &payload(&repo, "p", &transcript, json!(false))
    )));
    append(&repo, &evidence(true));
    std::fs::write(&transcript, "met\n").expect("transcript");
    assert!(blocked(&stop(
        &repo,
        &store,
        &payload(&repo, "m", &transcript, json!(false))
    )));
    append(&repo, &oracle());
    std::fs::write(&transcript, "stale\n").expect("transcript");
    assert!(blocked(&stop(
        &repo,
        &store,
        &payload(&repo, "s", &transcript, json!(false))
    )));
    append(&repo, &evidence(true));
    std::fs::write(&transcript, "met-again\n").expect("transcript");
    assert!(blocked(&stop(
        &repo,
        &store,
        &payload(&repo, "a", &transcript, json!(false))
    )));

    let status = pal(&repo, &store, &["round", "stop", "status", "--json"]);
    let state: Value = serde_json::from_slice(&status.stdout).expect("status JSON");
    assert_eq!(
        state["no_progress"], 2,
        "regression 또는 같은 최고점이 reset됐다: {state}"
    );
}

#[test]
fn corrupt_progress와_trailing_partial_crlf를_보수적으로_판정한다() {
    let (repo, store) = root("files");
    enable(&repo, &store);
    let transcript = repo.parent().expect("base").join("transcript.jsonl");
    std::fs::write(&transcript, "first\n").expect("transcript");
    assert!(blocked(&stop(
        &repo,
        &store,
        &payload(&repo, "a", &transcript, json!(false))
    )));

    let progress = std::fs::read_dir(&store)
        .expect("store")
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .find(|path| {
            path.file_name().is_some_and(|name| {
                name.to_string_lossy().starts_with("round-stop-progress-")
                    && name.to_string_lossy().ends_with(".json")
            })
        })
        .expect("progress");
    let mut contradictory: Value =
        serde_json::from_slice(&std::fs::read(&progress).expect("progress bytes"))
            .expect("progress json");
    contradictory["handoff"] = json!("blocked");
    contradictory["no_progress"] = json!(1);
    std::fs::write(&progress, serde_json::to_vec(&contradictory).expect("json")).expect("write");
    assert!(
        blocked(&stop(
            &repo,
            &store,
            &payload(&repo, "a", &transcript, json!(false))
        )),
        "의미상 모순인 progress의 replay가 handoff로 통과했다"
    );
    std::fs::write(&progress, "{").expect("corrupt progress");
    std::fs::write(&transcript, "second\n").expect("transcript");
    assert!(blocked(&stop(
        &repo,
        &store,
        &payload(&repo, "b", &transcript, json!(false))
    )));

    let ledger = repo
        .join(".palimpsest/rounds")
        .join(SLUG)
        .join("verification.log");
    std::fs::write(
        &ledger,
        format!(
            "{}\r\n{}",
            json!({"kind":"schema","version":1,"round":SLUG}),
            oracle()
        ),
    )
    .expect("partial CRLF");
    std::fs::write(&transcript, "partial\n").expect("transcript");
    assert!(blocked(&stop(
        &repo,
        &store,
        &payload(&repo, "c", &transcript, json!(false))
    )));
}

#[test]
fn 동시_session은_원자적으로_여섯에서_멈춘다() {
    let (repo, store) = root("concurrent");
    enable(&repo, &store);
    let transcript = repo.parent().expect("base").join("transcript.jsonl");
    std::fs::write(&transcript, "shared\n").expect("transcript");

    let mut threads = Vec::new();
    for n in 0..8 {
        let repo = repo.clone();
        let store = store.clone();
        let transcript = transcript.clone();
        threads.push(std::thread::spawn(move || {
            stop(
                &repo,
                &store,
                &payload(&repo, &format!("session-{n}"), &transcript, json!(false)),
            )
        }));
    }
    let outputs: Vec<_> = threads
        .into_iter()
        .map(|thread| thread.join().expect("thread"))
        .collect();
    assert!(outputs.iter().all(|out| out.status.success()));
    let status = pal(&repo, &store, &["round", "stop", "status", "--json"]);
    let state: Value = serde_json::from_slice(&status.stdout).expect("status JSON");
    assert_eq!(state["no_progress"], 6);
    assert_eq!(state["handoff"], "blocked");
}

#[test]
fn 죽은_process의_lock은_재사용되고_uninstall은_activation을_남기지_않는다() {
    let (repo, store) = root("rollback");
    let installed = pal(&repo, &store, &["install"]);
    assert!(
        installed.status.success(),
        "{}",
        String::from_utf8_lossy(&installed.stderr)
    );
    enable(&repo, &store);
    let transcript = repo.parent().expect("base").join("transcript.jsonl");
    std::fs::write(&transcript, "first\n").expect("transcript");
    assert!(blocked(&stop(
        &repo,
        &store,
        &payload(&repo, "a", &transcript, json!(false))
    )));

    let progress = std::fs::read_dir(&store)
        .expect("store")
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .find(|path| {
            path.file_name().is_some_and(|name| {
                name.to_string_lossy().starts_with("round-stop-progress-")
                    && name.to_string_lossy().ends_with(".json")
            })
        })
        .expect("progress");
    let lock = progress.with_extension("lock");
    std::fs::write(&lock, r#"{"token":"dead","created_millis":0}"#).expect("stale lock");
    std::fs::write(&transcript, "second\n").expect("transcript");
    assert!(blocked(&stop(
        &repo,
        &store,
        &payload(&repo, "b", &transcript, json!(false))
    )));
    assert!(lock.exists(), "커널 잠금의 안정된 inode가 사라졌다");

    let uninstalled = pal(&repo, &store, &["uninstall"]);
    assert!(
        uninstalled.status.success(),
        "{}",
        String::from_utf8_lossy(&uninstalled.stderr)
    );
    let reinstalled = pal(&repo, &store, &["install"]);
    assert!(reinstalled.status.success());
    std::fs::write(&transcript, "after reinstall\n").expect("transcript");
    assert!(
        !blocked(&stop(
            &repo,
            &store,
            &payload(&repo, "c", &transcript, json!(false)),
        )),
        "uninstall 뒤 activation이 되살아났다"
    );
}
