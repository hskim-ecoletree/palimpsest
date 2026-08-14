//! `binding.status` — **반대 방향 넷이 서로를 막는다** (`[f09.2.pass]` · `[f09.4.pass]`).
//!
//! # 「낡음을 감지한다」는 말하기 가장 쉽다
//!
//! **아무것도 안 켜는 감지기도, 전부 켜는 감지기도 그 문장을 만족한다.** 그래서 이
//! 파일은 방향 넷을 **한 저장소에서 연달아** 만든다 — 따로 만들면 한쪽만 도는 것을
//! 못 잡는다.
//!
//! ```text
//! ① 포매팅만 바꾸면 stale 0     ← 안 켜져야 할 때 안 켜진다 (진행 불가 조건)
//! ② 의미를 바꾸면 반드시 stale   ← ①의 짝. 없으면 ①이 「아무것도 안 켜는 감지기」로 통과한다
//! ③ 판정 불가가 live 로 안 샌다  ← R16 의 자리
//! ④ Orphaned ≠ Stale           ← 지우면 Orphaned · 고치면 Stale
//! ```
//!
//! # ★ 이 표면이 따로 필요한 이유 — `pal touch` 로는 ④ 가 안 보인다
//!
//! `pal touch <이름>` 은 **이름으로 먼저 찾는다.** 좌표가 사라진 결박은 `unknown` 이
//! 되어 **결박에 닿지 못하고**, `Orphaned` 가 화면에 영영 안 뜬다. 이 기능이 가장
//! 보여야 하는 상태 하나가 그것이다.
//!
//! # 하한 — **시험되지 않은 대조는 `–` 가 아니라 실패다** (`2e2eb3f`)
//!
//! 결박이 0 건이면 아래 전부가 공짜로 통과한다. 그리고 **포매팅 변형이 소스를 안
//! 바꿨으면 ①의 「stale 0」은 공짜다** — 그래서 **바이트가 실제로 달라졌는지**를 센다
//! (대조가 꺼지는 첫째 형태).

mod common;

use common::{git, pal};
use std::path::{Path, PathBuf};

/// 결박 하나와 그 상태를 재는 저장소.
///
/// `부름` 이 `도움` 을 부르므로 **`callers` 반경이 실제로 자란다** — 감시 집합이 2 다.
/// 반경이 안 자라면 `callers` 와 `symbol` 이 같은 것을 재고, 그러면 반경이 아무것도
/// 안 가른다.
fn 저장소(tag: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("pal-f09-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("임시 저장소");
    std::fs::write(&root.join("a.ts"), 소스_원본()).expect("a.ts");
    git(&root, &["init", "-q", "."]);
    git(&root, &["add", "-A"]);
    git(&root, &["-c", "user.email=t@example.com", "-c", "user.name=t", "commit", "-qm", "첫"]);
    root
}

fn 소스_원본() -> &'static str {
    "export function 도움(n: number) { return n + 1 }\n\
     export function 부름() { return 도움(2) }\n"
}

/// **의미가 같고 글자만 다르다** — 들여쓰기 · 개행 · 주석 · 후행 공백.
///
/// `prettier` 를 부르지 않는 이유: 이 시험은 `cargo test` 로 상시 돈다.
/// **코퍼스 규모의 포매터 대조는 `scripts/f09-verify.py` 가 진다**(`[f09.4.pass]` ②).
fn 소스_포매팅만() -> &'static str {
    "// 주석을 넣는다\n\
     export function 도움(n: number) {\n\
     \x20   return n + 1\n\
     }\n\
     \n\
     export function 부름() {\n\
     \x20   return 도움(2)   \n\
     }\n"
}

/// **의미가 다르다** — `+ 1` 이 `+ 2` 다.
fn 소스_의미변경() -> &'static str {
    "export function 도움(n: number) { return n + 2 }\n\
     export function 부름() { return 도움(2) }\n"
}

/// 결박의 상태 한 줄 — `(freshness, 감시 수, 반경)`.
fn 상태(repo: &Path, args: &[&str]) -> (String, u64, String) {
    let mut all = vec!["query", "binding.status"];
    all.extend_from_slice(args);
    all.push("--json");
    let v: serde_json::Value = serde_json::from_str(&pal(repo, &all)).expect("봉투 JSON");
    let b = &v["answer"]["bindings"];
    let list = b.as_array().expect("bindings 가 배열이 아니다");
    assert_eq!(list.len(), 1, "결박이 1 건이 아니다 — 이 시험이 아무것도 안 잰다");
    let one = &list[0];
    (
        one["status"]["code"]["freshness"].as_str().expect("freshness").to_owned(),
        one["watch"].as_u64().expect("watch"),
        one["radius"].as_str().expect("radius").to_owned(),
    )
}

fn 쓰기(repo: &Path, src: &str) -> usize {
    let path = repo.join("a.ts");
    let 전 = std::fs::read(&path).expect("읽기");
    std::fs::write(&path, src).expect("쓰기");
    let 후 = std::fs::read(&path).expect("읽기");
    // **하한** — 변형이 아무것도 안 바꿨으면 뒤의 단언이 공짜다(대조가 꺼지는 첫째 형태).
    assert_ne!(전, 후, "변형이 파일을 안 바꿨다 — 이 시험이 아무것도 안 잰다");
    후.len()
}

#[test]
fn 방향_넷이_서로를_막는다() {
    let repo = 저장소("directions");

    // ── 결박 — `callers` 반경 ────────────────────────────────────────────────
    pal(&repo, &["bind", "도움", "--note", "이 함수의 계약", "--radius", "callers"]);

    let (code, watch, radius) = 상태(&repo, &[]);
    assert_eq!(code, "live", "막 걸었는데 live 가 아니다");
    assert_eq!(radius, "callers");
    // **반경이 실제로 자랐다** — 1 이면 `callers` 가 `symbol` 과 같은 것을 잰 것이고,
    // 그러면 반경이 아무것도 안 가른다. `pal bind` 가 엣지를 지우면 여기서 잡힌다.
    assert_eq!(watch, 2, "`callers` 반경인데 감시 집합이 {watch} 개다 — 엣지가 안 붙었다");

    // ── ① 포매팅만 바꾸면 stale 0 ────────────────────────────────────────────
    쓰기(&repo, 소스_포매팅만());
    let (code, _, _) = 상태(&repo, &[]);
    assert_eq!(code, "live", "★ 포매팅만 바꿨는데 낡음이 켜졌다 — R-07 이 치명이라 부른 실패다");

    // ── ② 의미를 바꾸면 반드시 stale ─────────────────────────────────────────
    //
    // **①의 짝이다.** 없으면 ①이 「아무것도 안 켜는 감지기」로 만점을 받는다.
    쓰기(&repo, 소스_의미변경());
    let (code, _, _) = 상태(&repo, &[]);
    assert_eq!(code, "stale", "★ 의미를 바꿨는데 낡음이 안 켜졌다 — 감지기가 아무것도 안 켠다");

    // ── ④ Orphaned ≠ Stale ──────────────────────────────────────────────────
    //
    // **고치면 Stale · 지우면 Orphaned.** 뭉개면 *"고치면 되는 것"* 과 *"결정을 다시
    // 해야 하는 것"* 이 같은 화면이 된다.
    쓰기(&repo, "export function 전혀다른것() { return 0 }\n");
    let (code, _, _) = 상태(&repo, &[]);
    assert_eq!(code, "orphaned", "★ 심볼을 지웠는데 orphaned 가 아니다");

    let _ = std::fs::remove_dir_all(&repo);
}

#[test]
fn 판정_불가가_live_로_새지_않는다() {
    // ── ③ **R16 의 자리다.** 선행 구현이 `stale=False` 로 접었던 그것.
    //
    // 2층이 **다른 스냅샷**에 서 있는 채로 읽기 전용으로 물으면 판정할 수 없다 —
    // 여기서 요약을 대면 **옛 세대의 값과 지금의 결박**을 대는 것이 된다.
    //
    // ⚠ **`--read-only` 가 없으면 이 시험이 아무것도 안 잰다** — `pal query` 가 기본으로
    // 스티칭을 다시 돌려 2층을 이 스냅샷 것으로 만들어 버린다. F06 이 그 손잡이를
    // 만들었기 때문에 이 상태가 관측된다(대조가 꺼지는 다섯째 · 도구가 무엇을 읽는지).
    let repo = 저장소("undeterminable");
    let head = || {
        String::from_utf8(
            std::process::Command::new("git")
                .args(["rev-parse", "HEAD"])
                .current_dir(&repo)
                .output()
                .expect("git")
                .stdout,
        )
        .expect("UTF-8")
        .trim()
        .to_owned()
    };

    // `저장소()` 가 이미 한 번 커밋했다 — 그것이 c1 이다.
    let c1 = head();
    pal(&repo, &["bind", "도움", "--note", "계약", "--at", &c1]);
    쓰기(&repo, 소스_의미변경());
    git(&repo, &["add", "-A"]);
    git(&repo, &["-c", "user.email=t@example.com", "-c", "user.name=t", "commit", "-qm", "둘째"]);
    let c2 = head();
    assert_ne!(c1, c2, "커밋이 안 갈렸다 — 이 시험이 아무것도 안 잰다");

    // 2층은 c1 에 서 있다.
    let (code, _, _) = 상태(&repo, &["--at", &c1]);
    assert_eq!(code, "live");

    // **c2 를 읽기 전용으로 묻는다** — 2층은 여전히 c1 것이다.
    let v: serde_json::Value = serde_json::from_str(&pal(
        &repo,
        &["query", "binding.status", "--at", &c2, "--read-only", "--json"],
    ))
    .expect("봉투 JSON");
    assert_eq!(
        v["projection"]["built_for_this_snapshot"].as_bool(),
        Some(false),
        "2층이 이 스냅샷 것이라고 나온다 — 이 시험이 아무것도 안 잰다"
    );
    let one = &v["answer"]["bindings"][0]["status"]["code"];
    assert_eq!(
        one["freshness"].as_str(),
        Some("undeterminable"),
        "★ 판정할 수 없는데 판정했다 — 「유효하다」와 「유효한지 알 수 없다」가 같은 화면이 됐다"
    );
    assert_eq!(one["reason"].as_str(), Some("projection_stale"));

    let _ = std::fs::remove_dir_all(&repo);
}

#[test]
fn 반경이_넓어지면_감시_집합이_커진다() {
    // **★ 반대 방향** — `callers` 가 `symbol` 과 같은 것을 재면 반경이 아무것도 안
    // 가르고, 그러면 *"반경을 선언한다"* 는 이 설계의 대응이 산출에서 성립하지 않는다.
    let repo = 저장소("radius");

    pal(&repo, &["bind", "도움", "--note", "좁게", "--radius", "symbol"]);
    let (_, 좁은, r) = 상태(&repo, &[]);
    assert_eq!(r, "symbol");
    assert_eq!(좁은, 1);

    // 같은 좌표에 **다른 조각**을 걸면 다른 결박이다 — 앞의 것을 지우고 다시 잰다.
    let _ = std::fs::remove_file(repo.join(".palimpsest/intent.redb"));
    pal(&repo, &["bind", "도움", "--note", "넓게", "--radius", "callers"]);
    let (_, 넓은, r) = 상태(&repo, &[]);
    assert_eq!(r, "callers");
    assert!(넓은 > 좁은, "반경을 넓혔는데 감시 집합이 안 커졌다 ({좁은} → {넓은})");

    let _ = std::fs::remove_dir_all(&repo);
}

#[test]
fn 결박이_없어도_봉투를_지고_빈_목록으로_답한다() {
    // **결박 0 건과 「안 만듦」은 다르다.** 이 빌드에는 결박 능력이 있고 아무도 안
    // 걸었을 뿐이다 — `not_built` 로 내면 거짓말이고, 그것이 이 도구가 고발하는 형태다.
    let repo = 저장소("empty");
    let v: serde_json::Value =
        serde_json::from_str(&pal(&repo, &["query", "binding.status", "--json"])).expect("봉투");
    assert_eq!(v["answer"]["outcome"].as_str(), Some("bindings"));
    assert!(v["answer"]["bindings"].as_array().expect("배열").is_empty());
    // 봉투는 그대로 진다.
    for 필드 in ["snapshot", "projection", "coverage", "capabilities", "ledger", "elision"] {
        assert!(v.get(필드).is_some(), "결박이 0 건인데 `{필드}` 가 빠졌다");
    }

    let _ = std::fs::remove_dir_all(&repo);
}
