//! approve/verify 실행 경계의 black-box 공격 모집단.

mod common;

use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::Duration;

use common::PAL;
use serde_json::{Value, json};

const SLUG: &str = "fixture-approve-verify";

fn root(tag: &str) -> (PathBuf, PathBuf, PathBuf) {
    let base = std::env::temp_dir().join(format!("pal-round-approve-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    let repo = base.join("repo");
    let approvals = base.join("approvals");
    std::fs::create_dir_all(repo.join(".palimpsest/rounds")).expect("round root");
    std::fs::create_dir_all(&approvals).expect("approval root");
    git(&repo, &["init", "-q"]);
    git(&repo, &["config", "user.email", "fixture@example.invalid"]);
    git(&repo, &["config", "user.name", "Fixture"]);
    std::fs::write(repo.join("tracked.txt"), "tracked\n").expect("tracked");
    (base, repo, approvals)
}

fn git(repo: &Path, args: &[&str]) {
    let out = Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .expect("git fixture");
    assert!(
        out.status.success(),
        "git {args:?}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

fn commit_fixture(repo: &Path) {
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "fixture"]);
}

fn round(repo: &Path, ids: &[&str], oracles: &[Value]) -> PathBuf {
    round_version(repo, 2, ids, oracles)
}

fn round_version(repo: &Path, version: u32, ids: &[&str], events: &[Value]) -> PathBuf {
    let dir = repo.join(".palimpsest/rounds").join(SLUG);
    std::fs::create_dir_all(&dir).expect("round dir");
    let mut intent = String::from("# fixture\n\n## 완수 조건\n\n");
    for id in ids {
        intent.push_str(&format!("- [ ] {id} condition {id}\n"));
    }
    std::fs::write(dir.join("intent.md"), intent).expect("intent");
    let mut lines = vec![json!({"kind":"schema","version":version,"round":SLUG})];
    lines.extend_from_slice(events);
    let body = lines
        .iter()
        .map(Value::to_string)
        .collect::<Vec<_>>()
        .join("\n")
        + "\n";
    std::fs::write(dir.join("verification.log"), body).expect("ledger");
    dir
}

fn oracle(id: &str, mode: &str, negative_for: Option<&str>) -> Value {
    let mut value = json!({
        "kind": "oracle",
        "id": id,
        "mode": "command",
        "check": helper_command(mode, None),
        "expect": {"literal":"ROUND_OK"},
        "cwd": "."
    });
    if let Some(base) = negative_for {
        value["negative_for"] = Value::String(base.to_owned());
    }
    value
}

fn helper_command(mode: &str, target: Option<&Path>) -> String {
    let exe = std::env::current_exe().expect("test exe");
    #[cfg(unix)]
    {
        let target = target.map_or_else(String::new, |p| {
            format!(" PAL_HELPER_TARGET='{}'", p.display())
        });
        format!(
            "PAL_HELPER_MODE='{mode}'{target} '{}' --exact process_helper --nocapture",
            exe.display()
        )
    }
    #[cfg(windows)]
    {
        let target = target.map_or_else(String::new, |p| {
            format!("set \"PAL_HELPER_TARGET={}\"&& ", p.display())
        });
        format!(
            "set \"PAL_HELPER_MODE={mode}\"&& {target}\"{}\" --exact process_helper --nocapture",
            exe.display()
        )
    }
}

fn run(repo: &Path, approvals: &Path, args: &[&str]) -> Output {
    Command::new(PAL)
        .args(args)
        .env("PAL_APPROVAL_DIR", approvals)
        .current_dir(repo)
        .output()
        .expect("pal")
}

fn verify_with_path(repo: &Path, approvals: &Path, id: &str, path: &str) -> Output {
    Command::new(PAL)
        .args(["round", "verify", "--round", SLUG, "--id", id, "--json"])
        .env("PAL_APPROVAL_DIR", approvals)
        .env("PATH", path)
        .current_dir(repo)
        .output()
        .expect("pal")
}

fn approve(repo: &Path, approvals: &Path, id: &str, extra: &[&str]) -> Output {
    let mut args = vec!["round", "approve", "--round", SLUG, "--id", id, "--json"];
    args.extend_from_slice(extra);
    run(repo, approvals, &args)
}

fn finalize(repo: &Path, approvals: &Path) -> Output {
    run(
        repo,
        approvals,
        &["round", "verify", "--round", SLUG, "--all", "--json"],
    )
}

fn verify(repo: &Path, approvals: &Path, id: &str, extra: &[&str]) -> Output {
    let mut args = vec!["round", "verify", "--round", SLUG, "--id", id, "--json"];
    args.extend_from_slice(extra);
    run(repo, approvals, &args)
}

fn status(repo: &Path) -> Value {
    let out = Command::new(PAL)
        .args(["round", "status", "--round", SLUG, "--json"])
        .current_dir(repo)
        .output()
        .expect("status");
    serde_json::from_slice(&out.stdout).unwrap_or_else(|error| {
        panic!(
            "status JSON: {error}: {} / {}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        )
    })
}

fn value(out: &Output) -> Value {
    serde_json::from_slice(&out.stdout).unwrap_or_else(|error| {
        panic!(
            "JSON: {error}: {} / {}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        )
    })
}

fn assert_success(out: &Output) {
    assert!(
        out.status.success(),
        "exit={:?}\nstdout={}\nstderr={}",
        out.status.code(),
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
}

fn evidence_count(dir: &Path) -> usize {
    std::fs::read_to_string(dir.join("verification.log"))
        .expect("ledger")
        .lines()
        .filter(|line| line.contains("\"kind\":\"evidence\""))
        .count()
}

#[test]
fn intent밖에_주입한_oracle은_승인과_실행_모두_거부된다() {
    let (_base, repo, approvals) = root("intent-boundary");
    let counter = repo.join("counter.txt");
    round(
        &repo,
        &["A1"],
        &[json!({
            "kind":"oracle", "id":"INJECTED", "mode":"command",
            "check":helper_command("counter", Some(&counter)),
            "expect":{"literal":"ROUND_OK"}, "cwd":"."
        })],
    );
    commit_fixture(&repo);

    assert_eq!(approve(&repo, &approvals, "INJECTED", &[]).status.code(), Some(2));
    assert_eq!(verify(&repo, &approvals, "INJECTED", &[]).status.code(), Some(2));
    assert!(!counter.exists());
}

#[test]
fn process_helper() {
    let Ok(mode) = std::env::var("PAL_HELPER_MODE") else {
        return;
    };
    let target = std::env::var_os("PAL_HELPER_TARGET").map(PathBuf::from);
    match mode.as_str() {
        "success" | "negative-success" => println!("ROUND_OK"),
        "no-marker" => println!("ordinary output"),
        "nonzero-marker" => {
            println!("ROUND_OK");
            std::process::exit(7);
        }
        "spam" => {
            let text = "X".repeat(128 * 1024);
            println!("{text}ROUND_OK");
        }
        "sleep" => std::thread::sleep(Duration::from_secs(10)),
        "modify-tracked" => {
            std::fs::write(target.expect("target"), "changed\n").expect("modify");
            println!("ROUND_OK");
        }
        "modify-oracle" => {
            let path = target.expect("ledger");
            let event = json!({
                "kind":"oracle", "id":"A1", "mode":"command",
                "check":"changed command", "expect":{"literal":"ROUND_OK"}, "cwd":"."
            });
            let mut file = OpenOptions::new()
                .append(true)
                .open(path)
                .expect("append oracle");
            writeln!(file, "{event}").expect("oracle line");
            println!("ROUND_OK");
        }
        "counter" => {
            let path = target.expect("counter");
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
                .expect("counter");
            writeln!(file, "run").expect("counter line");
            println!("ROUND_OK");
        }
        "spawn-descendant" => {
            let mut child = Command::new(std::env::current_exe().expect("self"))
                .args(["--exact", "process_helper", "--nocapture"])
                .env("PAL_HELPER_MODE", "delayed-marker")
                .env("PAL_HELPER_TARGET", target.expect("marker"))
                .spawn()
                .expect("descendant");
            let _ = child.wait();
        }
        "spawn-detached-descendant" => {
            let _child = Command::new(std::env::current_exe().expect("self"))
                .args(["--exact", "process_helper", "--nocapture"])
                .env("PAL_HELPER_MODE", "delayed-marker")
                .env("PAL_HELPER_TARGET", target.expect("marker"))
                .spawn()
                .expect("detached descendant");
        }
        "delayed-marker" => {
            std::thread::sleep(Duration::from_secs(3));
            std::fs::write(target.expect("marker"), "escaped\n").expect("marker");
        }
        other => panic!("unknown helper mode {other}"),
    }
}

fn terminal_report(dir: &Path) {
    std::fs::write(
        dir.join("report.md"),
        "# report\n\n## 남지 않은 것\n없음.\n\n## 다음 회차가 받는 것\n없음.\n\n## 범위 밖\n없음.\n\n## 원리상 못 잰 것\n없음.\n\n## 능력 부재\n없음.\n",
    )
    .expect("report");
}

fn dialectic_event(repo: &Path, id: &str) -> Value {
    for name in ["thesis.md", "antithesis.md", "synthesis.md"] {
        std::fs::write(repo.join(name), format!("{name}\n")).expect("dialectic ref");
    }
    let reference = |name: &str| {
        json!({
            "path": name,
            "digest": blake3::hash(&std::fs::read(repo.join(name)).expect("ref")).to_hex().to_string()
        })
    };
    json!({
        "kind":"judgment", "id":id, "verdict":"met",
        "thesis":reference("thesis.md"),
        "antithesis":reference("antithesis.md"),
        "synthesis":reference("synthesis.md")
    })
}

#[test]
fn 종료직전_전수재실행은_이미_met인_command와_정반합_finding을_같이_닫는다() {
    let (base, repo, approvals) = root("finalize-all");
    let counter = base.join("finalize-counter.txt");
    let judgment = dialectic_event(&repo, "D1");
    let dir = round_version(
        &repo,
        3,
        &["A1", "D1"],
        &[
            json!({
                "kind":"oracle", "id":"A1", "mode":"command",
                "check":helper_command("counter", Some(&counter)),
                "expect":{"literal":"ROUND_OK"}, "cwd":"."
            }),
            judgment,
        ],
    );
    std::fs::write(
        dir.join("findings.jsonl"),
        format!(
            "{}\n{}\n",
            json!({"schema_version":3,"종류":"레코드","회차":SLUG}),
            json!({"id":"F1","상태":"닫힘","해악도":"금지역","닫은커밋":"abc1234"})
        ),
    )
    .expect("findings");
    terminal_report(&dir);
    commit_fixture(&repo);

    assert_success(&approve(&repo, &approvals, "A1", &[]));
    assert_success(&verify(&repo, &approvals, "A1", &[]));
    assert_eq!(
        std::fs::read_to_string(&counter).expect("counter").lines().count(),
        1
    );
    let before = status(&repo);
    assert_eq!(before["verification"], "met");
    assert_eq!(before["completion"], "in_progress");

    let out = finalize(&repo, &approvals);
    assert_success(&out);
    assert_eq!(value(&out)["rerun"], json!(["A1"]));
    assert_eq!(
        std::fs::read_to_string(&counter).expect("counter").lines().count(),
        2,
        "이미 met인 command를 종료 직전에 다시 실행하지 않았다"
    );
    assert_eq!(status(&repo)["completion"], "complete");

    std::fs::write(repo.join("tracked.txt"), "changed after completion\n").expect("tracked");
    let stale = status(&repo);
    assert_eq!(stale["conditions"][0]["state"], "stale");
    assert_eq!(stale["completion"], "in_progress");
}

#[test]
fn 열린_금지역실패와_구형_findings는_complete_checkpoint를_거부한다() {
    for (tag, findings) in [
        (
            "open-harmful",
            format!(
                "{}\n{}\n",
                json!({"schema_version":3,"종류":"레코드","회차":SLUG}),
                json!({"id":"F1","상태":"열림","해악도":"실패"})
            ),
        ),
        (
            "legacy-findings",
            format!(
                "{}\n{}\n",
                json!({"schema_version":2,"종류":"레코드","회차":SLUG}),
                json!({"id":"F1","해악도":"미관"})
            ),
        ),
        (
            "malformed-current-findings",
            format!(
                "{}\n{}\n",
                json!({"schema_version":3,"종류":"레코드","회차":SLUG}),
                json!({"id":"F1","상태":"닫힘","해악도":"금지역"})
            ),
        ),
    ] {
        let (_base, repo, approvals) = root(tag);
        let judgment = dialectic_event(&repo, "D1");
        let dir = round_version(&repo, 3, &["D1"], &[judgment]);
        std::fs::write(dir.join("findings.jsonl"), findings).expect("findings");
        terminal_report(&dir);
        commit_fixture(&repo);
        let out = finalize(&repo, &approvals);
        assert_eq!(out.status.code(), Some(2), "{tag}: {}", String::from_utf8_lossy(&out.stdout));
        assert_ne!(status(&repo)["completion"], "complete");
    }
}

#[test]
fn depth1_shallow_clone에서_approve와_stop_activation이_서고_deepen과_local_commit뒤에도_유지된다() {
    let (base, source, _unused) = root("shallow-source");
    round(&source, &["A1"], &[oracle("A1", "success", None)]);
    commit_fixture(&source);
    std::fs::write(source.join("second.txt"), "tip\n").expect("second");
    commit_fixture(&source);

    let shallow = base.join("shallow");
    let clone = Command::new("git")
        .args(["clone", "-q", "--depth", "1", "--no-local"])
        .arg(&source)
        .arg(&shallow)
        .output()
        .expect("shallow clone");
    assert!(clone.status.success(), "{}", String::from_utf8_lossy(&clone.stderr));
    let shallow_check = Command::new("git")
        .args(["rev-parse", "--is-shallow-repository"])
        .current_dir(&shallow)
        .output()
        .expect("shallow check");
    assert_eq!(String::from_utf8_lossy(&shallow_check.stdout).trim(), "true");
    let count = Command::new("git")
        .args(["rev-list", "--count", "HEAD"])
        .current_dir(&shallow)
        .output()
        .expect("count");
    assert_eq!(String::from_utf8_lossy(&count.stdout).trim(), "1");
    let parent = Command::new("git")
        .args(["cat-file", "-e", "HEAD^"])
        .current_dir(&shallow)
        .output()
        .expect("parent probe");
    assert!(!parent.status.success(), "parent object가 있어 shallow 공격이 서지 않았다");

    let approvals = base.join("shallow-approvals");
    std::fs::create_dir_all(&approvals).expect("approvals");
    assert_success(&approve(&shallow, &approvals, "A1", &[]));
    let enabled = run(
        &shallow,
        &approvals,
        &["round", "stop", "enable", "--round", SLUG, "--json"],
    );
    assert_success(&enabled);

    git(&shallow, &["fetch", "--unshallow"]);
    let deepened = run(
        &shallow,
        &approvals,
        &["round", "stop", "status", "--json"],
    );
    assert_success(&deepened);
    assert_eq!(value(&deepened)["outcome"], "enabled");

    git(&shallow, &["config", "user.email", "fixture@example.invalid"]);
    git(&shallow, &["config", "user.name", "Fixture"]);
    std::fs::write(shallow.join("local.txt"), "local\n").expect("local");
    commit_fixture(&shallow);
    let status = run(
        &shallow,
        &approvals,
        &["round", "stop", "status", "--json"],
    );
    assert_success(&status);
    assert_eq!(value(&status)["outcome"], "enabled");
}

#[test]
fn 미승인_oracle과_변경된_path_cwd_shell_budget은_spawn전에_거부된다() {
    let (_base, repo, approvals) = root("approval-boundary");
    let counter = repo.join("counter.txt");
    let dir = round(
        &repo,
        &["A1"],
        &[json!({
            "kind":"oracle", "id":"A1", "mode":"command",
            "check":helper_command("counter", Some(&counter)),
            "expect":{"literal":"ROUND_OK"}, "cwd":"."
        })],
    );
    commit_fixture(&repo);

    let denied = verify(&repo, &approvals, "A1", &[]);
    assert_eq!(
        denied.status.code(),
        Some(3),
        "stdout={}\nstderr={}",
        String::from_utf8_lossy(&denied.stdout),
        String::from_utf8_lossy(&denied.stderr)
    );
    assert_eq!(value(&denied)["outcome"], "approval_required");
    assert!(!counter.exists());

    assert_success(&approve(&repo, &approvals, "A1", &[]));
    let changed_path = verify_with_path(&repo, &approvals, "A1", "");
    assert_eq!(changed_path.status.code(), Some(3));
    assert!(!counter.exists());

    #[cfg(unix)]
    let default_shell = PathBuf::from("/bin/sh");
    #[cfg(windows)]
    let default_shell = PathBuf::from(std::env::var_os("ComSpec").expect("ComSpec"));
    let alternate_shell = approvals.join(if cfg!(windows) {
        "alternate.exe"
    } else {
        "alternate-sh"
    });
    std::fs::copy(&default_shell, &alternate_shell).expect("alternate shell fixture");
    let alternate = alternate_shell.to_str().expect("UTF-8 fixture path");
    let changed_shell = verify(&repo, &approvals, "A1", &["--shell", alternate]);
    assert_eq!(changed_shell.status.code(), Some(2));
    assert_eq!(
        approve(&repo, &approvals, "A1", &["--shell", alternate])
            .status
            .code(),
        Some(2)
    );
    assert!(!counter.exists());

    let changed_budget = verify(&repo, &approvals, "A1", &["--timeout", "1"]);
    assert_eq!(changed_budget.status.code(), Some(3));
    assert!(!counter.exists());

    let ledger = std::fs::read_to_string(dir.join("verification.log")).expect("ledger");
    let changed = ledger.replace("\"cwd\":\".\"", "\"cwd\":\"nested\"");
    std::fs::create_dir_all(repo.join("nested")).expect("nested");
    std::fs::write(dir.join("verification.log"), changed).expect("changed cwd");
    let changed_cwd = verify(&repo, &approvals, "A1", &[]);
    assert_eq!(changed_cwd.status.code(), Some(3));
    assert!(!counter.exists());
}

#[test]
fn exit_zero_no_expect와_nonzero_marker는_둘다_unmet이다() {
    for (tag, mode) in [("no-marker", "no-marker"), ("nonzero", "nonzero-marker")] {
        let (_base, repo, approvals) = root(tag);
        round(&repo, &["A1"], &[oracle("A1", mode, None)]);
        commit_fixture(&repo);
        assert!(approve(&repo, &approvals, "A1", &[]).status.success());
        let out = verify(&repo, &approvals, "A1", &[]);
        assert_eq!(
            out.status.code(),
            Some(1),
            "{tag}: {}",
            String::from_utf8_lossy(&out.stdout)
        );
        assert_eq!(status(&repo)["conditions"][0]["state"], "unmet");
    }
}

#[test]
fn 실행된_현재_negative_control없이는_주조건도_met이_아니다() {
    let (_base, repo, approvals) = root("negative-control");
    round(
        &repo,
        &["A1", "A1-n"],
        &[
            oracle("A1", "success", None),
            oracle("A1-n", "no-marker", Some("A1")),
        ],
    );
    commit_fixture(&repo);
    assert!(approve(&repo, &approvals, "A1", &[]).status.success());
    assert_success(&verify(&repo, &approvals, "A1", &[]));
    let before = status(&repo);
    assert_ne!(before["conditions"][0]["state"], "met");
    assert_eq!(before["verification"], "in_progress");

    assert!(approve(&repo, &approvals, "A1-n", &[]).status.success());
    let failed_control = verify(&repo, &approvals, "A1-n", &[]);
    assert_eq!(failed_control.status.code(), Some(1));
    assert_ne!(status(&repo)["conditions"][0]["state"], "met");

    let path = repo
        .join(".palimpsest/rounds")
        .join(SLUG)
        .join("verification.log");
    let body = std::fs::read_to_string(&path).expect("ledger");
    let old = serde_json::to_string(&helper_command("no-marker", None)).expect("old JSON string");
    let new =
        serde_json::to_string(&helper_command("negative-success", None)).expect("new JSON string");
    let repaired = body.replace(
        old.strip_prefix('"')
            .and_then(|value| value.strip_suffix('"'))
            .expect("quoted old JSON string"),
        new.strip_prefix('"')
            .and_then(|value| value.strip_suffix('"'))
            .expect("quoted new JSON string"),
    );
    assert_ne!(body, repaired, "control oracle fixture must be rewritten");
    std::fs::write(&path, repaired).expect("new control oracle");
    git(&repo, &["add", "."]);
    git(&repo, &["commit", "-q", "-m", "repair control"]);
    assert!(approve(&repo, &approvals, "A1-n", &[]).status.success());
    assert_success(&verify(&repo, &approvals, "A1-n", &[]));
    let after = status(&repo);
    assert_eq!(after["conditions"][0]["state"], "met");
    assert_eq!(after["conditions"][1]["state"], "met");
    assert_eq!(after["verification"], "met");
}

#[test]
fn 과거_evidence를_negative_control로_재분류하면_stale이다() {
    let (_base, repo, approvals) = root("control-role-tamper");
    let dir = round(
        &repo,
        &["A1", "C1"],
        &[
            oracle("A1", "success", None),
            oracle("C1", "negative-success", None),
        ],
    );
    commit_fixture(&repo);
    for id in ["A1", "C1"] {
        assert!(approve(&repo, &approvals, id, &[]).status.success());
        assert_success(&verify(&repo, &approvals, id, &[]));
    }
    assert_eq!(status(&repo)["conditions"][0]["state"], "met");

    let path = dir.join("verification.log");
    let body = std::fs::read_to_string(&path).expect("ledger");
    let changed = body.replace(
        "\"id\":\"C1\",\"kind\":\"oracle\"",
        "\"id\":\"C1\",\"kind\":\"oracle\",\"negative_for\":\"A1\"",
    );
    assert_ne!(body, changed, "oracle fixture must be rewritten");
    std::fs::write(path, changed).expect("role change");
    let got = status(&repo);
    assert_eq!(got["conditions"][0]["state"], "stale");
    assert_eq!(got["conditions"][1]["state"], "stale");
}

#[test]
fn timeout_output_cap과_descendant는_bounded_cleanup된다() {
    for (tag, mode, extra) in [
        ("timeout", "sleep", vec!["--timeout", "1"]),
        ("cap", "spam", vec!["--output-limit", "1024"]),
    ] {
        let (_base, repo, approvals) = root(tag);
        round(&repo, &["A1"], &[oracle("A1", mode, None)]);
        commit_fixture(&repo);
        assert!(approve(&repo, &approvals, "A1", &extra).status.success());
        let started = std::time::Instant::now();
        let out = verify(&repo, &approvals, "A1", &extra);
        assert_eq!(
            out.status.code(),
            Some(1),
            "{tag}: {} / {}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
        assert!(
            started.elapsed() < Duration::from_secs(6),
            "{tag} was unbounded"
        );
        assert_eq!(status(&repo)["conditions"][0]["state"], "unmet");
    }

    let (_base, repo, approvals) = root("descendant");
    let marker = repo.parent().expect("base").join("escaped.txt");
    round(
        &repo,
        &["A1"],
        &[json!({
            "kind":"oracle", "id":"A1", "mode":"command",
            "check":helper_command("spawn-descendant", Some(&marker)),
            "expect":{"literal":"ROUND_OK"}, "cwd":"."
        })],
    );
    commit_fixture(&repo);
    let extra = ["--timeout", "1"];
    assert!(approve(&repo, &approvals, "A1", &extra).status.success());
    assert_eq!(
        verify(&repo, &approvals, "A1", &extra).status.code(),
        Some(1)
    );
    std::thread::sleep(Duration::from_secs(4));
    assert!(!marker.exists(), "descendant escaped cleanup");

    let (_base, repo, approvals) = root("detached-descendant");
    let marker = repo.parent().expect("base").join("detached-escaped.txt");
    round(
        &repo,
        &["A1"],
        &[json!({
            "kind":"oracle", "id":"A1", "mode":"command",
            "check":helper_command("spawn-detached-descendant", Some(&marker)),
            "expect":{"literal":"ROUND_OK"}, "cwd":"."
        })],
    );
    commit_fixture(&repo);
    let extra = ["--timeout", "1"];
    assert!(approve(&repo, &approvals, "A1", &extra).status.success());
    let started = std::time::Instant::now();
    assert_eq!(
        verify(&repo, &approvals, "A1", &extra).status.code(),
        Some(1)
    );
    assert!(started.elapsed() < Duration::from_secs(4));
    std::thread::sleep(Duration::from_secs(3));
    assert!(!marker.exists(), "detached descendant escaped cleanup");
}

#[test]
fn 실행중_oracle이나_projected_tree변화는_evidence없이_폐기된다() {
    for (tag, mode, target_kind) in [
        ("mid-tree", "modify-tracked", "tracked"),
        ("mid-oracle", "modify-oracle", "ledger"),
    ] {
        let (_base, repo, approvals) = root(tag);
        let dir = repo.join(".palimpsest/rounds").join(SLUG);
        let target = if target_kind == "tracked" {
            repo.join("tracked.txt")
        } else {
            dir.join("verification.log")
        };
        round(
            &repo,
            &["A1"],
            &[json!({
                "kind":"oracle", "id":"A1", "mode":"command",
                "check":helper_command(mode, Some(&target)),
                "expect":{"literal":"ROUND_OK"}, "cwd":"."
            })],
        );
        commit_fixture(&repo);
        assert!(approve(&repo, &approvals, "A1", &[]).status.success());
        let before = evidence_count(&dir);
        let out = verify(&repo, &approvals, "A1", &[]);
        assert_eq!(
            out.status.code(),
            Some(3),
            "{tag}: stdout={} stderr={}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
        assert_eq!(value(&out)["outcome"], "discarded");
        assert_eq!(evidence_count(&dir), before);
    }
}

#[test]
fn append실패는_재실행하지_않고_partial_line은_invalid다() {
    let (_base, repo, approvals) = root("append-failure");
    let counter = repo.parent().expect("base").join("counter.txt");
    let dir = round(
        &repo,
        &["A1"],
        &[json!({
            "kind":"oracle", "id":"A1", "mode":"command",
            "check":helper_command("counter", Some(&counter)),
            "expect":{"literal":"ROUND_OK"}, "cwd":"."
        })],
    );
    commit_fixture(&repo);
    assert!(approve(&repo, &approvals, "A1", &[]).status.success());
    std::fs::create_dir(dir.join("verification.log.append.lock")).expect("held append lock");
    let out = verify(&repo, &approvals, "A1", &[]);
    assert_eq!(out.status.code(), Some(2));
    assert_eq!(
        std::fs::read_to_string(&counter)
            .expect("one run")
            .lines()
            .count(),
        1
    );
    assert_eq!(evidence_count(&dir), 0);

    let mut file = OpenOptions::new()
        .append(true)
        .open(dir.join("verification.log"))
        .expect("ledger");
    write!(file, "{{\"kind\":\"evidence\"").expect("partial");
    let got = status(&repo);
    assert_eq!(got["outcome"], "invalid");
    assert_eq!(got["code"], "invalid_schema");
}

#[test]
fn approval_record변조와_stale_projected_evidence는_fail_closed다() {
    let (_base, repo, approvals) = root("tamper-stale");
    round(&repo, &["A1"], &[oracle("A1", "success", None)]);
    commit_fixture(&repo);
    assert!(approve(&repo, &approvals, "A1", &[]).status.success());
    let record = std::fs::read_dir(&approvals)
        .expect("approval dir")
        .find_map(|entry| {
            let path = entry.ok()?.path();
            path.is_file().then_some(path)
        })
        .expect("approval record");
    std::fs::write(&record, "not approval\n").expect("tamper");
    assert_eq!(verify(&repo, &approvals, "A1", &[]).status.code(), Some(3));

    assert!(approve(&repo, &approvals, "A1", &[]).status.success());
    assert_success(&verify(&repo, &approvals, "A1", &[]));
    assert_eq!(status(&repo)["conditions"][0]["state"], "met");
    std::fs::write(repo.join("tracked.txt"), "later\n").expect("later tree");
    assert_eq!(status(&repo)["conditions"][0]["state"], "stale");
}

#[test]
fn 같은_size_mtime의_tracked변조도_승인을_무효화한다() {
    let (_base, repo, approvals) = root("racy-stat");
    round(&repo, &["A1"], &[oracle("A1", "success", None)]);
    commit_fixture(&repo);
    assert!(approve(&repo, &approvals, "A1", &[]).status.success());

    let tracked = repo.join("tracked.txt");
    let metadata = std::fs::metadata(&tracked).expect("tracked metadata");
    let mtime = filetime::FileTime::from_last_modification_time(&metadata);
    let atime = filetime::FileTime::from_last_access_time(&metadata);
    std::fs::write(&tracked, "mutated\n").expect("same-size mutation");
    filetime::set_file_times(&tracked, atime, mtime).expect("restore stat timestamp");

    let denied = verify(&repo, &approvals, "A1", &[]);
    assert_eq!(denied.status.code(), Some(3));
    assert_eq!(value(&denied)["outcome"], "approval_required");
}

#[test]
fn 같은_oracle재실행은_새_current_evidence를_append한다() {
    let (_base, repo, approvals) = root("rerun");
    let dir = round(&repo, &["A1"], &[oracle("A1", "success", None)]);
    commit_fixture(&repo);
    assert!(approve(&repo, &approvals, "A1", &[]).status.success());
    assert_success(&verify(&repo, &approvals, "A1", &[]));
    assert_success(&verify(&repo, &approvals, "A1", &[]));
    assert_eq!(evidence_count(&dir), 2);
    assert_eq!(status(&repo)["conditions"][0]["state"], "met");
}

#[test]
fn evidence_append가_reader파일상한을_넘기기_전에_거부된다() {
    use pal_core::{ROUND_VERIFICATION_FILE_MAX_BYTES, ROUND_VERIFICATION_LINE_MAX_BYTES};

    let (_base, repo, approvals) = root("ledger-file-cap");
    let dir = round(&repo, &["A1"], &[oracle("A1", "success", None)]);
    commit_fixture(&repo);
    assert!(approve(&repo, &approvals, "A1", &[]).status.success());

    let ledger = dir.join("verification.log");
    let mut body = std::fs::read(&ledger).expect("initial ledger");
    let oracle_line = oracle("A1", "success", None).to_string();
    let target = ROUND_VERIFICATION_FILE_MAX_BYTES as usize - 1;
    let remaining = target - body.len();
    let max_total = ROUND_VERIFICATION_LINE_MAX_BYTES + 1;
    let line_count = remaining.div_ceil(max_total);
    for index in 0..line_count {
        let lines_left = line_count - index;
        let bytes_left = target - body.len();
        let total = bytes_left / lines_left;
        assert!(total > oracle_line.len() && total <= max_total);
        body.extend_from_slice(oracle_line.as_bytes());
        body.resize(body.len() + total - oracle_line.len() - 1, b' ');
        body.push(b'\n');
    }
    assert_eq!(body.len(), target);
    std::fs::write(&ledger, &body).expect("near-limit valid ledger");

    let denied = verify(&repo, &approvals, "A1", &[]);
    assert_eq!(denied.status.code(), Some(2));
    assert_eq!(std::fs::read(&ledger).expect("unchanged ledger"), body);
    assert_eq!(evidence_count(&dir), 0);
}
