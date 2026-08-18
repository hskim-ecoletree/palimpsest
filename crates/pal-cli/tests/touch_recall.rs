//! **적시 제시** — 걸린 것과 지켜보는 것 · 점진 회상 · 근접 후보 (F11).
//!
//! 합격선 정본은 `corpus/criteria.toml` `[f11.pass]` ②③④⑤.
//!
//! # 왜 실물 바이너리인가
//!
//! 여기서 재는 것 넷 중 셋이 **표면의 사실**이다 — 상한이 실제로 걸리는가, 잘린 수가
//! 산출에 실리는가, 낡은 것이 상한을 이기는가. API 를 부르면 *"이 함수가 이렇게 센다"*
//! 까지만 알고, `pal touch` 가 그 함수를 지나는지는 모른다. **F11 이 반증될 수 있는
//! 자리가 정확히 거기다** — 계산은 옳은데 화면에 안 오는 것.

mod common;

use common::{git, pal};
use std::path::{Path, PathBuf};

/// `[f11.pass].top_n_default` — `F11 §3.3` 이 적은 값이고 이 시험이 정하지 않았다.
const 상한: usize = 10;

/// 저장소 하나 — **한 파일에 심볼 셋.** 반경 `files:` 가 셋을 다 지켜본다.
fn 저장소(tag: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("pal-f11-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("임시 저장소");
    std::fs::write(
        root.join("core.ts"),
        "export function deriveVerdicts() { return 1 }\n\
         export function writeHandoff() { return deriveVerdicts() }\n\
         export function mirrorVerdicts() { return 2 }\n",
    )
    .expect("core.ts");
    // **다른 파일** — 반경 밖이 실재해야 ④(무관 좌표)의 모집단이 0 이 아니다.
    std::fs::write(root.join("other.ts"), "export function unrelatedThing() { return 3 }\n")
        .expect("other.ts");
    git(&root, &["init", "-q", "."]);
    git(&root, &["add", "-A"]);
    git(&root, &["-c", "user.email=t@example.com", "-c", "user.name=t", "commit", "-qm", "첫"]);
    root
}

fn 방(root: &Path, tag: &str) -> Vec<String> {
    vec![
        "--repo".into(),
        root.display().to_string(),
        "--cache-dir".into(),
        root.join(format!(".box-{tag}/cache")).display().to_string(),
        "--index".into(),
        root.join(format!(".box-{tag}/index.redb")).display().to_string(),
        "--intent".into(),
        root.join(format!(".box-{tag}/intent.redb")).display().to_string(),
    ]
}

fn touch(root: &Path, 방: &[String], name: &str, 더: &[&str]) -> serde_json::Value {
    let mut args: Vec<&str> = vec!["touch", name];
    args.extend(방.iter().map(String::as_str));
    args.extend(더);
    args.push("--json");
    serde_json::from_str(&pal(root, &args)).expect("봉투 JSON")
}

fn bind(root: &Path, 방: &[String], target: &str, note: &str, radius: &str) {
    let mut args: Vec<&str> =
        vec!["bind", target, "--note", note, "--radius", radius];
    args.extend(방.iter().map(String::as_str));
    pal(root, &args);
}

fn 걸린_것(v: &serde_json::Value) -> &Vec<serde_json::Value> {
    v["answer"]["bindings"]["present"].as_array().expect("걸린 것")
}

fn 지켜보는_것(v: &serde_json::Value) -> &Vec<serde_json::Value> {
    v["answer"]["watching"]["present"].as_array().expect("지켜보는 것")
}

fn 잘린_수(v: &serde_json::Value) -> u64 {
    v["elision"]["truncated"]
        .as_array()
        .expect("truncated")
        .iter()
        .filter(|t| t["reason"] == "binding_max_exceeded")
        .map(|t| t["count"].as_u64().unwrap_or(0))
        .sum()
}

// ═════════════════════════════════════════════════════════════════════════════
// ⑤ **걸린 것과 지켜보는 것은 다른 목록이다** — 그리고 ②의 「다른 좌표」가 여기서 산다
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn 다른_좌표에_걸린_규칙이_이_좌표에서_뜬다() {
    let root = 저장소("watch");
    let 방 = 방(&root, "watch");

    // `deriveVerdicts` 에 걸고 **파일 전체**를 지켜본다 — 반경은 선언이다(F09 §3).
    bind(&root, &방, "deriveVerdicts", "완료 경로는 같은 함수를 부른다", "files:core.ts");

    // ★ **결박은 다른 좌표에 걸렸는데 여기서 떠야 한다.**
    // `recurrence.toml` 이 *"경로 하나를 빠뜨렸다"* 로 이름 붙인 형태다.
    let v = touch(&root, &방, "mirrorVerdicts", &[]);
    assert_eq!(v["answer"]["outcome"], "found");
    assert!(걸린_것(&v).is_empty(), "이 좌표에 직접 걸린 것은 없어야 한다");
    let w = 지켜보는_것(&v);
    assert_eq!(w.len(), 1, "지켜보는 것이 안 떴다: {v}");
    // **어디에 걸렸는지가 다음 행동을 정한다.** 좌표가 없으면 고치러 갈 데를 모른다.
    assert_eq!(w[0]["at"]["at"], "elsewhere");
    assert_eq!(w[0]["at"]["place"]["name"], "deriveVerdicts");
    // **반경이 판정과 함께 실린다** — *"`files:1` 반경에서 live"* 는 *"유효하다"* 와 다르다.
    assert_eq!(w[0]["radius"], "files:1");

    // ★ **대상 좌표에서는 「걸린 것」이고 「지켜보는 것」이 아니다** — 두 목록이 겹치면
    // 사람이 같은 결박을 둘로 읽는다.
    let v = touch(&root, &방, "deriveVerdicts", &[]);
    assert_eq!(걸린_것(&v).len(), 1);
    assert_eq!(걸린_것(&v)[0]["at"]["at"], "here");
    assert!(지켜보는_것(&v).is_empty(), "대상이 두 목록에 실렸다: {v}");

    let _ = std::fs::remove_dir_all(&root);
}

// ═════════════════════════════════════════════════════════════════════════════
// ④ **반대 방향** — 무관 좌표에서는 안 뜬다. 모집단이 0 이 아님을 함께 단언한다
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn 감시_집합_밖의_좌표에서는_안_뜬다() {
    let root = 저장소("unrelated");
    let 방 = 방(&root, "unrelated");
    bind(&root, &방, "deriveVerdicts", "완료 경로는 같은 함수를 부른다", "files:core.ts");

    // **하한** — 같은 결박이 반경 안에서는 실제로 뜬다. 안 그러면 아래의 0 이
    // *"안 뜬다"* 가 아니라 *"아무것도 안 만든다"* 를 재는 것이 된다.
    let 안 = touch(&root, &방, "writeHandoff", &[]);
    assert_eq!(지켜보는_것(&안).len(), 1, "반경 안에서 안 떴다");

    // ★ **밖에서는 0.** 조회 거리를 열면 이것이 깨진다.
    let 밖 = touch(&root, &방, "unrelatedThing", &[]);
    assert!(걸린_것(&밖).is_empty());
    assert!(지켜보는_것(&밖).is_empty(), "반경 밖에서 떴다: {밖}");

    let _ = std::fs::remove_dir_all(&root);
}

// ═════════════════════════════════════════════════════════════════════════════
// ④ **점진 회상** — 상한 · 잘린 수 · 그리고 **낡은 것은 상한을 이긴다**
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn 상한을_넘으면_자르고_그_수를_싣는다() {
    let root = 저장소("recall");
    let 방 = 방(&root, "recall");
    // **상한보다 둘 많게** 건다. `BindingId` 가 `(대상, 조각)` 에서 나오므로 조각이
    // 다르면 다른 결박이다.
    for i in 0..(상한 + 2) {
        bind(&root, &방, "deriveVerdicts", &format!("규칙 {i}"), "symbol");
    }

    let v = touch(&root, &방, "deriveVerdicts", &[]);
    assert_eq!(걸린_것(&v).len(), 상한, "상한이 안 걸렸다: {}", 걸린_것(&v).len());
    // **조용한 절단이 없다.**
    assert_eq!(잘린_수(&v), 2, "잘린 수가 안 실렸다: {v}");

    // 손잡이를 낮추면 더 잘린다 — 상한이 실재한다는 증거다.
    let v = touch(&root, &방, "deriveVerdicts", &["--binding-max", "3"]);
    assert_eq!(걸린_것(&v).len(), 3);
    assert_eq!(잘린_수(&v), (상한 + 2 - 3) as u64);

    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn 낡은_것은_상한에_걸려도_실린다() {
    let root = 저장소("stale");
    let 방 = 방(&root, "stale");
    // 셋을 **`mirrorVerdicts` 를 지켜보게** 걸어 둔다 — 그 셋이 나중에 전부 낡는다.
    for i in 0..3 {
        bind(&root, &방, "mirrorVerdicts", &format!("낡을 규칙 {i}"), "symbol");
    }
    // 그리고 낡지 않을 것 셋을 다른 좌표에 건다.
    for i in 0..3 {
        bind(&root, &방, "deriveVerdicts", &format!("성한 규칙 {i}"), "symbol");
    }

    // **`mirrorVerdicts` 의 본문만 바꾼다** — 셋이 `stale` 이 된다.
    std::fs::write(
        root.join("core.ts"),
        "export function deriveVerdicts() { return 1 }\n\
         export function writeHandoff() { return deriveVerdicts() }\n\
         export function mirrorVerdicts() { return 2 + 40 }\n",
    )
    .expect("core.ts");

    // 상한을 **1** 로 낮춰도 낡은 셋이 전부 실려야 한다.
    let v = touch(&root, &방, "mirrorVerdicts", &["--binding-max", "1"]);
    let items = 걸린_것(&v);
    let 낡은: Vec<&serde_json::Value> =
        items.iter().filter(|i| i["status"]["code"]["state"] != "live").collect();
    assert_eq!(낡은.len(), 3, "낡은 것이 상한에 잘렸다: {v}");
    assert_eq!(items.len(), 3, "낡지 않은 것까지 실렸다: {v}");
    // 낡은 것만 남았으므로 자를 것이 없다 — **상한이 낡음을 이기지 않는다.**
    assert_eq!(잘린_수(&v), 0);

    // ★ **낡은 것이 맨 앞이다** — 정렬이 사실 기반인지의 산출 수준 검사.
    let v = touch(&root, &방, "mirrorVerdicts", &[]);
    assert_ne!(걸린_것(&v)[0]["status"]["code"]["state"], "live");

    let _ = std::fs::remove_dir_all(&root);
}

// ═════════════════════════════════════════════════════════════════════════════
// ③ **근접 후보** — 두 갈래 · 하나를 안 고른다 · 빈 목록도 답이다
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn 못_찾으면_가까운_이름을_내고_하나를_고르지_않는다() {
    let root = 저장소("near");
    let 방 = 방(&root, "near");

    // 표기만 다른 입력.
    let v = touch(&root, &방, "derive_verdicts", &[]);
    assert_eq!(v["answer"]["outcome"], "unknown", "고르면 안 된다: {v}");
    let near = v["answer"]["near"].as_array().expect("near");
    assert_eq!(near.len(), 1, "{v}");
    assert_eq!(near[0]["name"], "deriveVerdicts");
    assert_eq!(near[0]["kind"], "spelling");

    // 부분 매칭 — `F11 §4` 가 *"`pal touch cancel` 이 후보를 보여 준다"* 로 적은 형태.
    let v = touch(&root, &방, "Verdicts", &[]);
    let near = v["answer"]["near"].as_array().expect("near");
    let 이름들: Vec<&str> = near.iter().map(|n| n["name"].as_str().expect("name")).collect();
    assert!(이름들.contains(&"deriveVerdicts") && 이름들.contains(&"mirrorVerdicts"), "{v}");
    assert!(near.iter().all(|n| n["kind"] == "substring"), "{v}");

    // ★ **가까운 것이 하나도 없는 것도 답이다** — `unknown` 인 것은 같고 `near` 가 빈다.
    let v = touch(&root, &방, "zzzz", &[]);
    assert_eq!(v["answer"]["outcome"], "unknown");
    assert!(v["answer"]["near"].as_array().expect("near").is_empty(), "{v}");

    let _ = std::fs::remove_dir_all(&root);
}

// ═════════════════════════════════════════════════════════════════════════════
// ⑥ **두 표면이 한 실행기를 지난다** — 그리고 질의 로그가 켜졌다
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn 두_표면이_같은_답을_내고_로그를_남긴다() {
    let root = 저장소("surface");
    let 방 = 방(&root, "surface");
    bind(&root, &방, "deriveVerdicts", "완료 경로는 같은 함수를 부른다", "files:core.ts");

    let t = touch(&root, &방, "writeHandoff", &[]);
    // ⚠ **옛 판은 여기가 `not_recorded{surface_does_not_log}` 였다.** 그러면 F17 이
    // 이 조회를 「안 일어난 것」으로 세고 미조회를 과대 계상한다.
    assert_eq!(t["log"]["status"], "recorded", "질의 로그가 안 남았다: {t}");

    let mut args: Vec<&str> = vec!["query", "binding.touch", "writeHandoff"];
    args.extend(방.iter().map(String::as_str));
    args.push("--json");
    let q: serde_json::Value = serde_json::from_str(&pal(&root, &args)).expect("봉투 JSON");
    // 같은 실행기를 지나므로 **답의 알맹이가 같다.** 겉옷만 다르다.
    assert_eq!(q["answer"]["outcome"], "touch");
    assert_eq!(q["answer"]["result"]["symbol"]["name"], "writeHandoff");
    assert_eq!(
        q["answer"]["result"]["watching"]["present"].as_array().expect("watching").len(),
        지켜보는_것(&t).len()
    );

    let _ = std::fs::remove_dir_all(&root);
}

// ═════════════════════════════════════════════════════════════════════════════
// ⑦ **두 시계** — 그리고 **시간이 답에 안 섞인다**
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn 시간은_표준오류로_가고_답은_두_번_돌려도_같다() {
    let root = 저장소("timing");
    let 방 = 방(&root, "timing");
    bind(&root, &방, "deriveVerdicts", "완료 경로는 같은 함수를 부른다", "files:core.ts");

    let mut args: Vec<&str> = vec!["touch", "deriveVerdicts"];
    args.extend(방.iter().map(String::as_str));
    args.extend(["--timing", "--json"]);

    let 한번 = std::process::Command::new(common::PAL)
        .args(&args)
        .current_dir(&root)
        .output()
        .expect("pal");
    assert!(한번.status.success());
    let 오류 = String::from_utf8(한번.stderr).expect("UTF-8");
    // 두 시계가 **둘 다** 나온다 — 하나만 내면 무엇을 쟀는지 갈리지 않는다.
    assert!(오류.contains("elapsed_micros="), "질의 시간이 없다: {오류}");
    assert!(오류.contains("process_micros="), "프로세스 시간이 없다: {오류}");

    let 두번 = std::process::Command::new(common::PAL)
        .args(&args)
        .current_dir(&root)
        .output()
        .expect("pal");
    // ★ **답은 바이트로 같다.** 시간이 산출에 섞였다면 여기서 깨진다 —
    // 재구축 등가성(F04)과 왕복 항등(F05)이 그 위에 서 있다.
    assert_eq!(한번.stdout, 두번.stdout, "같은 질문에 다른 바이트가 나왔다");
    // 그리고 산출에는 시간이 없다.
    let 산출 = String::from_utf8(두번.stdout).expect("UTF-8");
    assert!(!산출.contains("duration_micros"), "시간이 산출에 섞였다: {산출}");

    let _ = std::fs::remove_dir_all(&root);
}
