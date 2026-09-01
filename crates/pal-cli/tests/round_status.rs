//! `pal round`의 잠긴 black-box 계약.

mod common;

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use common::PAL;
use serde_json::{Value, json};

const SLUG: &str = "fixture-round";
const ORACLE_DIGEST: &str = "4cf3cb926ab8249a040632d0c1e694509ab40eee2eacc8da15d1353392b026dd";
const OUTPUT_DIGEST: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

fn root(tag: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("pal-round-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join(".palimpsest/rounds")).expect("round root");
    root
}

fn round(root: &Path, slug: &str, ids: &[&str]) -> PathBuf {
    let dir = root.join(".palimpsest/rounds").join(slug);
    std::fs::create_dir_all(&dir).expect("round dir");
    let mut body = String::from("# fixture\n\n## 완수 조건\n\n");
    for id in ids {
        body.push_str(&format!("- [ ] {id} condition {id}\n"));
    }
    std::fs::write(dir.join("intent.md"), body).expect("intent");
    dir
}

fn run(root: &Path, args: &[&str]) -> Output {
    Command::new(PAL)
        .args(args)
        .current_dir(root)
        .output()
        .expect("pal")
}

fn value(out: &Output) -> Value {
    serde_json::from_slice(&out.stdout).unwrap_or_else(|e| {
        panic!(
            "JSON output: {e}\nstdout={}\nstderr={}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        )
    })
}

fn status(root: &Path, slug: &str) -> Output {
    run(root, &["round", "status", "--round", slug, "--json"])
}

fn schema(slug: &str) -> String {
    format!(r#"{{"kind":"schema","version":1,"round":"{slug}"}}"#)
}

fn oracle(id: &str, check: &str) -> String {
    serde_json::to_string(&json!({
        "kind": "oracle",
        "id": id,
        "mode": "command",
        "check": check,
        "expect": {"literal": "ROUND_OK"},
        "cwd": "."
    }))
    .expect("oracle")
}

fn evidence(id: &str, digest: &str, exit: i32, matched: bool) -> String {
    serde_json::to_string(&json!({
        "kind": "evidence",
        "id": id,
        "oracle_digest": digest,
        "exit": exit,
        "matched": matched,
        "output_digest": OUTPUT_DIGEST,
        "output_bytes": 8
    }))
    .expect("evidence")
}

fn ledger(dir: &Path, lines: &[String]) {
    std::fs::write(
        dir.join("verification.log"),
        format!("{}\n", lines.join("\n")),
    )
    .expect("ledger");
}

#[test]
fn 원장이_없으면_unregistered이고_명령은_성공한다() {
    let root = root("no-ledger");
    round(&root, SLUG, &["A1"]);
    let out = status(&root, SLUG);
    assert_eq!(out.status.code(), Some(0));
    let got = value(&out);
    assert_eq!(got["verification"], "unregistered");
    assert_eq!(got["conditions"][0]["state"], "unregistered");
}

#[test]
fn oracle은_intent_id의_부분집합이고_미등록은_남는다() {
    let root = root("subset");
    let dir = round(&root, SLUG, &["A1", "A2"]);
    ledger(&dir, &[schema(SLUG), oracle("A1", "cargo test -q")]);
    let got = value(&status(&root, SLUG));
    assert_eq!(got["verification"], "in_progress");
    assert_eq!(got["conditions"][0]["state"], "pending");
    assert_eq!(got["conditions"][1]["state"], "unregistered");

    ledger(&dir, &[schema(SLUG), oracle("A3", "cargo test -q")]);
    let out = status(&root, SLUG);
    assert_eq!(out.status.code(), Some(2));
    assert_eq!(value(&out)["code"], "invalid_schema");
}

#[test]
fn evidence가_없으면_pending이고_실패관측은_unmet이다() {
    let root = root("pending-unmet");
    let dir = round(&root, SLUG, &["A1"]);
    ledger(&dir, &[schema(SLUG), oracle("A1", "cargo test -q")]);
    assert_eq!(
        value(&status(&root, SLUG))["conditions"][0]["state"],
        "pending"
    );

    ledger(
        &dir,
        &[
            schema(SLUG),
            oracle("A1", "cargo test -q"),
            evidence("A1", ORACLE_DIGEST, 0, false),
        ],
    );
    assert_eq!(
        value(&status(&root, SLUG))["conditions"][0]["state"],
        "unmet"
    );
}

#[test]
fn 현재_oracle_뒤의_evidence만_유효하고_digest가_다르면_stale이다() {
    let root = root("stale");
    let dir = round(&root, SLUG, &["A1"]);
    ledger(
        &dir,
        &[
            schema(SLUG),
            oracle("A1", "cargo test -q"),
            evidence("A1", ORACLE_DIGEST, 0, true),
            oracle("A1", "cargo test -q"),
        ],
    );
    assert_eq!(
        value(&status(&root, SLUG))["conditions"][0]["state"],
        "stale"
    );

    ledger(
        &dir,
        &[
            schema(SLUG),
            oracle("A1", "cargo test -q"),
            evidence("A1", ORACLE_DIGEST, 0, true),
            oracle("A1", "cargo test --all"),
            evidence("A1", ORACLE_DIGEST, 0, true),
        ],
    );
    assert_eq!(
        value(&status(&root, SLUG))["conditions"][0]["state"],
        "stale"
    );
}

#[test]
fn met은_exit_zero와_expect_match를_함께_요구한다() {
    let root = root("met");
    let dir = round(&root, SLUG, &["A1"]);
    ledger(
        &dir,
        &[
            schema(SLUG),
            oracle("A1", "cargo test -q"),
            evidence("A1", ORACLE_DIGEST, 0, true),
        ],
    );
    let got = value(&status(&root, SLUG));
    assert_eq!(got["verification"], "met");
    assert_eq!(got["completion"], "unavailable");
    assert_eq!(got["conditions"][0]["state"], "met");
    assert_eq!(got["conditions"][0]["oracle_digest"], ORACLE_DIGEST);
}

#[test]
fn terminal은_verification과_분리되고_서로_배타다() {
    let root = root("terminal");
    let dir = round(&root, SLUG, &["A1"]);
    std::fs::write(dir.join("folded.md"), "## 왜 접었나\nfixture\n").expect("folded");
    let got = value(&status(&root, SLUG));
    assert_eq!(got["verification"], "unregistered");
    assert_eq!(got["terminal"], "folded");

    std::fs::write(dir.join("report.md"), "# report\n").expect("report");
    let out = status(&root, SLUG);
    assert_eq!(out.status.code(), Some(2));
    assert_eq!(value(&out)["code"], "invalid_transition");
}

#[test]
fn unknown_schema_kind_mode_field는_안정된_code로_거부된다() {
    let cases = [
        r#"{"kind":"schema","version":4,"round":"fixture-round"}"#,
        r#"{"kind":"mystery","version":1,"round":"fixture-round"}"#,
        r#"{"kind":"schema","version":1,"round":"fixture-round","extra":1}"#,
        r#"{"kind":"schema","version":1,"round":"fixture-round"}
{"kind":"oracle","id":"A1","mode":"dialectic","check":"x","expect":{"literal":"y"},"cwd":"."}"#,
        r#"{"kind":"schema","version":1,"round":"fixture-round"}
{"kind":"judgment","id":"A1"}"#,
    ];
    for (i, body) in cases.iter().enumerate() {
        let root = root(&format!("unknown-{i}"));
        let dir = round(&root, SLUG, &["A1"]);
        std::fs::write(dir.join("verification.log"), format!("{body}\n")).expect("ledger");
        let out = status(&root, SLUG);
        assert_eq!(out.status.code(), Some(2), "case {i}");
        assert_eq!(value(&out)["code"], "invalid_schema", "case {i}");
    }
}

#[test]
fn schema3의_명시적_정반합만_command와_같은_aggregate에_들어간다() {
    let root = root("dialectic");
    Command::new("git").args(["init", "-q"]).current_dir(&root).status().expect("git");
    Command::new("git")
        .args(["config", "user.email", "fixture@example.invalid"])
        .current_dir(&root)
        .status()
        .expect("git config");
    Command::new("git")
        .args(["config", "user.name", "Fixture"])
        .current_dir(&root)
        .status()
        .expect("git config");
    let dir = round(&root, SLUG, &["D1"]);
    for name in ["thesis.md", "antithesis.md", "synthesis.md"] {
        std::fs::write(root.join(name), format!("{name}\n")).expect("ref");
    }
    let reference = |name: &str| {
        json!({
            "path": name,
            "digest": blake3::hash(&std::fs::read(root.join(name)).expect("ref")).to_hex().to_string()
        })
    };
    ledger(
        &dir,
        &[
            json!({"kind":"schema","version":3,"round":SLUG}).to_string(),
            json!({
                "kind":"judgment", "id":"D1", "verdict":"met",
                "thesis":reference("thesis.md"),
                "antithesis":reference("antithesis.md"),
                "synthesis":reference("synthesis.md")
            })
            .to_string(),
        ],
    );
    Command::new("git")
        .args(["add", "."])
        .current_dir(&root)
        .status()
        .expect("git add");
    Command::new("git")
        .args(["commit", "-q", "-m", "fixture"])
        .current_dir(&root)
        .status()
        .expect("git commit");
    let got = value(&status(&root, SLUG));
    assert_eq!(got["conditions"][0]["state"], "met");
    assert_eq!(got["verification"], "met");
    assert_eq!(got["completion"], "in_progress");

    std::fs::write(root.join("antithesis.md"), "changed\n").expect("change ref");
    let stale = value(&status(&root, SLUG));
    assert_eq!(stale["conditions"][0]["state"], "stale");
    assert_eq!(stale["verification"], "in_progress");
}

#[test]
fn 불가능한_전이는_구조오류다() {
    let root = root("transition");
    let dir = round(&root, SLUG, &["A1"]);
    ledger(
        &dir,
        &[schema(SLUG), evidence("A1", ORACLE_DIGEST, 0, true)],
    );
    let out = status(&root, SLUG);
    assert_eq!(out.status.code(), Some(2));
    assert_eq!(value(&out)["code"], "invalid_transition");
}

#[test]
fn active_round는_새_원장이_있는_유일한_비종료_회차다() {
    let root = root("active");
    round(&root, "old-round", &["A1"]);
    let current = round(&root, "current-round", &["A1"]);
    ledger(&current, &[schema("current-round")]);
    let out = run(&root, &["round", "status", "--json"]);
    assert_eq!(out.status.code(), Some(0));
    assert_eq!(value(&out)["round"], "current-round");

    let other = round(&root, "other-round", &["A1"]);
    ledger(&other, &[schema("other-round")]);
    let out = run(&root, &["round", "status", "--json"]);
    assert_eq!(out.status.code(), Some(2));
    assert_eq!(value(&out)["code"], "resolve_error");
}

#[test]
fn active_round가_없으면_정상_outcome이다() {
    let root = root("no-active");
    round(&root, "old-round", &["A1"]);
    let out = run(&root, &["round", "status", "--json"]);
    assert_eq!(out.status.code(), Some(0));
    assert_eq!(value(&out), json!({"outcome":"no_active_round"}));
}

#[test]
fn 명시_round의_디렉터리와_intent_부재는_resolve_error다() {
    let root = root("missing-round");
    let out = status(&root, "missing-round");
    assert_eq!(out.status.code(), Some(2));
    assert_eq!(value(&out)["code"], "resolve_error");

    let dir = root.join(".palimpsest/rounds/directory-only");
    std::fs::create_dir_all(&dir).expect("directory only");
    let out = status(&root, "directory-only");
    assert_eq!(out.status.code(), Some(2));
    assert_eq!(value(&out)["code"], "resolve_error");
}

#[test]
fn 원장만_있고_intent가_없어도_resolve_error다() {
    let root = root("ledger-only");
    let dir = root.join(".palimpsest/rounds/ledger-only");
    std::fs::create_dir_all(&dir).expect("ledger only");
    ledger(&dir, &[schema("ledger-only")]);
    let out = status(&root, "ledger-only");
    assert_eq!(out.status.code(), Some(2));
    assert_eq!(value(&out)["code"], "resolve_error");
}

#[test]
fn status는_oracle을_실행하지_않고_입력을_바꾸지_않는다() {
    let root = root("readonly");
    let dir = round(&root, SLUG, &["A1"]);
    ledger(
        &dir,
        &[schema(SLUG), oracle("A1", "touch SHOULD_NOT_EXIST")],
    );
    let before_intent = std::fs::read(dir.join("intent.md")).expect("intent bytes");
    let before_ledger = std::fs::read(dir.join("verification.log")).expect("ledger bytes");
    let out = status(&root, SLUG);
    assert_eq!(out.status.code(), Some(0));
    assert!(!root.join("SHOULD_NOT_EXIST").exists());
    assert_eq!(std::fs::read(dir.join("intent.md")).unwrap(), before_intent);
    assert_eq!(
        std::fs::read(dir.join("verification.log")).unwrap(),
        before_ledger
    );
}

#[test]
fn conditions_json은_전환전_python_golden과_같다() {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    let input = "crates/pal-cli/tests/fixtures/round_conditions_traps.md";
    let golden: Value = serde_json::from_slice(
        &std::fs::read(manifest.join("tests/fixtures/round_conditions_traps.golden.json"))
            .expect("golden"),
    )
    .expect("golden json");
    let out = run(
        manifest.parent().and_then(Path::parent).expect("workspace"),
        &["round", "conditions", "--file", input, "--json"],
    );
    assert_eq!(out.status.code(), Some(1));
    assert_eq!(value(&out), golden);
}

#[test]
fn 사람출력도_같은_상태를_전부_보인다() {
    let root = root("human");
    let dir = round(&root, SLUG, &["A1", "A2"]);
    ledger(&dir, &[schema(SLUG), oracle("A1", "cargo test -q")]);
    let out = run(&root, &["round", "status", "--round", SLUG]);
    assert!(out.status.success());
    let text = String::from_utf8(out.stdout).expect("UTF-8");
    for expected in [
        SLUG,
        "in_progress",
        "open",
        "A1",
        "pending",
        "A2",
        "unregistered",
    ] {
        assert!(
            text.contains(expected),
            "사람 출력에 `{expected}` 가 없다:\n{text}"
        );
    }
}
