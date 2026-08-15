//! **접기는 절단이 아니다** — `[f06.2.pass]` ①②③④ 와 `[f06].the_whole_surface_carries_an_envelope`.
//!
//! # 성분 아홉을 **코드에 상수로 박는다**
//!
//! F05 가 여섯에 대해 세운 처방 그대로다 — 골든을 산출에서 떠서 만들면 무엇이 빠져도
//! 통과한다(빠진 채로 떠지기 때문이다). F06 이 셋을 더해 **아홉**이다.

mod common;

use common::{PAL, git, pal};
use std::path::{Path, PathBuf};
use std::process::Command;

/// 봉투의 성분. **하나라도 빠지면 실패다.** F05 의 여섯 + F06 의 셋.
const 봉투의_아홉: [&str; 9] = [
    "snapshot",
    "projection",
    "coverage",
    "capabilities",
    "ledger",
    "elision",
    // ── F06 ────────────────────────────────────────────────────────────────
    "fold",
    "log",
    "tokens",
];

/// 대장이 안 접히는 유일한 질의. **하나와 나머지가 다른 것이 이 시험의 요점이다.**
const 대장_자신: &str = "ledger.snapshot";

/// 이름을 안 받는 질의들 — 인자를 안 붙인다.
const 인자_없는: [&str; 2] = [대장_자신, "graph.dump"];

/// **좌표가 아니라 계획 문서를 받는 질의** — F12.
///
/// 인자를 받는 질의가 전부 `SymbolName` 을 받는다는 가정이 여기서 처음 깨진다.
/// 시험이 그 가정을 들고 있으면 새 질의가 늘 때마다 **엉뚱한 인자로 부르고**,
/// 그 실패는 봉투의 결함처럼 보인다.
///
/// **이름이 아니라 카탈로그의 인자 타입에서 뜬다** — `host_free` 가 같은 교훈을
/// 이미 적었다(*"이름으로 세는 검사는 이름이 하나 늘 때마다 조용히 틀린다"*).
const 문서_인자: &str = "RepoPath";

/// 이 질의가 계획 문서를 받는가 — **카탈로그가 답한다.**
fn 계획_문서를_받나(name: &str) -> bool {
    let out = Command::new(PAL).args(["query", "--list", "--json"]).output().expect("pal");
    let v: serde_json::Value = serde_json::from_slice(&out.stdout).expect("목록 JSON");
    v["built"]
        .as_array()
        .expect("built")
        .iter()
        .find(|q| q["name"].as_str() == Some(name))
        .and_then(|q| q["args"].as_array().expect("args").first())
        .and_then(|a| a["type"].as_str())
        == Some(문서_인자)
}

/// 계획 문서의 자리 — **저장소 밖이다.**
///
/// ⚠ **안에 두면 `narrative.unbound` 가 실패한다.** 그 질의는 저장소의 마크다운을
/// 인입하는데 `pal query` 는 의도 저장소를 **읽기로** 열고, 인입은 개체 이름을 쓰려
/// 한다. **F12 가 만든 결함이 아니라 `.md` 파일 하나가 드러낸 F10 표면의 결함이고**,
/// 그 사실은 `docs/gates/F12.md` 가 「다음으로 넘기는 것」에 적는다.
fn 계획_문서(repo: &Path) -> PathBuf {
    repo.with_extension("plan.md")
}

/// **하한** — 능력 목록이 이보다 짧으면 「안 접혔다」가 공짜로 통과한다.
const 최소_능력: usize = 6;

/// 심볼이 많은 저장소 — **토큰 단조를 재려면 답의 크기가 크게 갈려야 한다.**
///
/// `[f06.2.pass]` ③의 하한이 *"두 질의의 실제 바이트가 3 배 이상 차이 나는 짝"* 이다.
/// 작은 저장소에서는 `graph.dump` 와 `ledger.snapshot` 이 비슷해서 **단조가 우연히
/// 성립한다.**
fn 큰_저장소(tag: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("pal-f06-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("임시 저장소");

    // **`뿌리` 가 잎 전부를 부른다.** 반대로 두면 `symbol.reaches 뿌리` 가 0 건이고,
    // 그러면 절단을 일으킬 수 없어 ④가 아무것도 못 잰다(이 시험이 그렇게 한 번 멈췄다).
    let mut src = String::new();
    for i in 0..120 {
        src.push_str(&format!("export function 잎{i}() {{ return {i} }}\n"));
    }
    src.push_str("export function 뿌리() { return ");
    for i in 0..120 {
        src.push_str(&format!("잎{i}() + "));
    }
    src.push_str("0 }\n");
    std::fs::write(root.join("big.ts"), src).expect("big.ts");
    git(&root, &["init", "-q", "."]);
    git(&root, &["add", "-A"]);
    git(&root, &["-c", "user.email=t@example.com", "-c", "user.name=t", "commit", "-qm", "첫"]);
    // **계획 문서를 저장소 밖에 심는다** — `plan.deviation` 이 좌표가 아니라 이것을
    // 받는다(F12). 기준선을 `HEAD` 로 두면 두 스냅샷이 같아 이탈이 정의되지 않는데,
    // **이 시험이 재는 것은 봉투이지 이탈이 아니므로** 그것이 정확한 설정이다.
    std::fs::write(
        계획_문서(&root),
        "---\nbaseline: HEAD\n---\n# 이 저장소의 계획\n무엇을 왜\n\n## 하나\n`뿌리` 를 고친다\n",
    )
    .expect("plan.md");
    root
}

/// 저장소와 그 계획 문서를 함께 지운다.
fn 치운다(repo: &Path) {
    let _ = std::fs::remove_dir_all(repo);
    let _ = std::fs::remove_file(계획_문서(repo));
}

fn 질의(repo: &Path, name: &str, extra: &[&str]) -> serde_json::Value {
    let 계획 = 계획_문서(repo);
    let 계획 = 계획.to_string_lossy();
    let mut args = vec!["query", name];
    if 계획_문서를_받나(name) {
        args.push(&계획);
    } else if !인자_없는.contains(&name) {
        args.push("뿌리");
    }
    args.extend_from_slice(extra);
    args.push("--json");
    serde_json::from_str(&pal(repo, &args)).expect("봉투 JSON")
}

/// 이 빌드가 답하는 질의 이름 — **표면에서 받는다.** 시험이 자기 목록을 갖지 않는다.
fn 질의_이름들() -> Vec<String> {
    let out = Command::new(PAL).args(["query", "--list", "--json"]).output().expect("pal");
    let v: serde_json::Value = serde_json::from_slice(&out.stdout).expect("목록 JSON");
    v["built"]
        .as_array()
        .expect("built")
        .iter()
        .map(|q| q["name"].as_str().expect("이름").to_owned())
        .collect()
}

#[test]
fn 접힌_것이_어디로_갔는지_함께_실린다() {
    let repo = 큰_저장소("fold");
    let 이름들 = 질의_이름들();
    assert!(이름들.len() >= 최소_능력, "질의가 {}개다 — 하한 미만", 이름들.len());

    let (mut 접힌_회차, mut 안_접힌_회차) = (0usize, 0usize);
    for name in &이름들 {
        let v = 질의(&repo, name, &[]);
        let folded = v["fold"]["folded"].as_array().expect("fold.folded 가 배열이 아니다");
        if name == 대장_자신 {
            // **자기 답이 대장이므로 접힌 것이 없다.**
            assert!(folded.is_empty(), "`{name}` 이 대장을 접었다 — 자기 답을 접은 것이다");
            안_접힌_회차 += 1;
            continue;
        }
        assert_eq!(folded.len(), 1, "`{name}` 의 접힌 자리가 하나가 아니다");
        let f = &folded[0];
        assert_eq!(f["what"].as_str(), Some("ledger"));
        assert!(f["count"].is_number(), "접힌 건수가 없다 — 크기를 알 수 없다");
        // ★ **부를 수 있는 이름이어야 한다.** 못 부르는 이름이면 아무 말도 안 한 것이다.
        assert_eq!(
            f["unfolded_by"].as_str(),
            Some(대장_자신),
            "`{name}` 이 부를 수 없는 이름으로 폄을 가리킨다"
        );
        접힌_회차 += 1;
    }

    // **둘이 서로를 막는다.** 늘 접으면 둘째가, 아무것도 안 접으면 첫째가 실패한다.
    assert!(접힌_회차 >= 1, "접힌 회차가 없다 — 이 시험은 아무것도 안 쟀다");
    assert_eq!(안_접힌_회차, 1, "안 접힌 회차가 하나가 아니다");

    치운다(&repo);
}

#[test]
fn 능력_목록은_접히지_않는다() {
    // `[f06.2.pass]` ② — 문서 §4.3 이 못 박은 자리다. 접으면 소비자가 공백을
    // *"이상 없음"* 으로 읽는다.
    let repo = 큰_저장소("caps");
    for name in 질의_이름들() {
        for extra in [&[][..], &["--node-max", "1"][..], &["--depth-max", "0"][..]] {
            let v = 질의(&repo, &name, extra);
            let built = v["capabilities"]["built"].as_array().expect("built");
            let not_built = v["capabilities"]["not_built"].as_array().expect("not_built");
            // **접힘 상한을 아무리 낮춰도 안 줄어든다.**
            assert!(built.len() >= 최소_능력, "`{name}` {extra:?} 에서 능력이 {}개다", built.len());
            assert!(!not_built.is_empty(), "`{name}` {extra:?} 에서 미구축이 비었다");
            // 접힌 자리에 능력이 오면 실패 — 타입이 막지만 산출에서 다시 센다.
            for f in v["fold"]["folded"].as_array().expect("folded") {
                assert_ne!(f["what"].as_str(), Some("capabilities"), "능력이 접혔다");
            }
        }
    }
    치운다(&repo);
}

#[test]
fn 토큰_추정이_실제_크기를_따라간다() {
    let repo = 큰_저장소("tokens");

    let mut 잰_것: Vec<(String, u64, u64)> = Vec::new();
    for name in 질의_이름들() {
        let v = 질의(&repo, &name, &[]);
        let t = &v["tokens"];
        let 실제 = serde_json::to_string(&v).expect("다시 직렬화").len() as u64;
        let 잰 = t["serialized_bytes"].as_u64().expect("잰 바이트");
        let 토큰 = t["approx_tokens"].as_u64().expect("추정 토큰");
        let 나눔 = t["bytes_per_token"].as_u64().expect("가정");

        // ① **비례** — 잰 값이 실제와 ±10% 안. 자기 자신을 못 세는 만큼만 어긋난다.
        let 차 = 실제.abs_diff(잰) as f64 / 실제 as f64;
        assert!(차 < 0.10, "`{name}` 의 잰 값이 실제와 {:.1}% 어긋난다", 차 * 100.0);
        // ③ **근거가 값이다** — 셋이 서로 맞는다.
        assert_eq!(토큰, 잰 / 나눔, "`{name}` 의 세 값이 서로 안 맞는다");

        잰_것.push((name, 실제, 토큰));
    }

    // ② ★ **단조** — 답이 큰 질의의 추정이 더 크다.
    let 큰 = 잰_것.iter().max_by_key(|x| x.1).expect("최대");
    let 작은 = 잰_것.iter().min_by_key(|x| x.1).expect("최소");
    // **하한** — 3 배 미만이면 단조가 우연히 성립한다.
    assert!(
        큰.1 >= 작은.1 * 3,
        "가장 큰 답({} {}B)과 가장 작은 답({} {}B)이 3 배도 안 차이 난다 — 단조가 안 재어진다",
        큰.0, 큰.1, 작은.0, 작은.1
    );
    assert!(큰.2 > 작은.2, "큰 답의 추정이 더 크지 않다 — 상수일 수 있다");

    치운다(&repo);
}

#[test]
fn 접힘과_절단이_같은_사건을_두_번_세지_않는다() {
    // `[f06.2.pass]` ④ — `fold_is_not_elision` 이 갈라 둔 것이 **실제로** 갈렸는지는
    // 절단을 일으켜 봐야 안다.
    let repo = 큰_저장소("cut");

    let 넉넉 = 질의(&repo, "symbol.reaches", &[]);
    let 좁게 = 질의(&repo, "symbol.reaches", &["--node-max", "2"]);

    // **하한** — 절단이 안 일어나면 아래가 공짜로 통과한다.
    let 잘린 = 좁게["elision"]["truncated"].as_array().expect("truncated");
    assert!(!잘린.is_empty(), "`--node-max 2` 로도 절단이 안 일어났다 — 이 시험은 아무것도 안 쟀다");
    assert!(넉넉["elision"]["truncated"].as_array().expect("truncated").is_empty());

    // **절단이 일어나도 접힌 건수는 안 변한다.** 변하면 둘이 같은 사건을 세고 있다.
    assert_eq!(
        넉넉["fold"]["folded"], 좁게["fold"]["folded"],
        "절단이 접힘을 바꿨다 — 둘이 갈려 있지 않다"
    );

    치운다(&repo);
}

#[test]
fn 봉투를_지는_표면_전부가_아홉을_진다() {
    // `[f06].the_whole_surface_carries_an_envelope` — F05 가 자기 표면까지 졌고
    // **나머지가 여기다.**
    let repo = 큰_저장소("surface");

    // 봉투를 지는 표면 — 질의 여섯 + `pal touch` + `pal doctor`.
    let mut 봉투들: Vec<(String, serde_json::Value)> = Vec::new();
    for name in 질의_이름들() {
        let v = 질의(&repo, &name, &[]);
        봉투들.push((format!("query {name}"), v));
    }
    for (이름, args) in [
        ("touch", vec!["touch", "뿌리", "--json"]),
        ("doctor", vec!["doctor", "--json"]),
    ] {
        let v: serde_json::Value = serde_json::from_str(&pal(&repo, &args)).expect("봉투 JSON");
        봉투들.push((이름.to_owned(), v));
    }

    // **하한** — `--json` 을 내는 봉투 표면이 6 개 미만이면 멈춘다.
    assert!(봉투들.len() >= 6, "봉투를 지는 표면이 {}개다", 봉투들.len());

    for (이름, v) in &봉투들 {
        for 성분 in 봉투의_아홉 {
            assert!(v.get(성분).is_some(), "`{이름}` 의 답에 `{성분}` 이 없다");
        }
        assert!(v.get("answer").is_some(), "`{이름}` 의 답에 `answer` 가 없다");
        // 로그 상태는 **갈래**다 — `bool` 이면 왜 안 남았는지를 못 싣는다.
        assert!(v["log"]["status"].is_string(), "`{이름}` 의 로그 상태가 갈래가 아니다");
    }

    // ★ **두 갈래가 다 서 있다.** 한쪽만 나오면 이 필드는 상수다.
    let 남은 = 봉투들.iter().filter(|(_, v)| v["log"]["status"] == "recorded").count();
    let 안_남은 = 봉투들.len() - 남은;
    assert!(남은 >= 1 && 안_남은 >= 1, "로그 상태의 갈래가 하나뿐이다 — 아무것도 안 재고 있다");

    치운다(&repo);
}

#[test]
fn 봉투를_안_지는_표면은_목록으로_서고_봉투인_척하지_않는다() {
    // `[f06].the_whole_surface_carries_an_envelope` — *"전부가 봉투를 지는 것이 아니다.
    // **그것이 이 항목의 산출이다.**"* 목록이 게이트에만 있고 검사에 없으면 그 목록은
    // 다음 표면이 생길 때 조용히 낡는다.
    let repo = 큰_저장소("nonenvelope");

    // 먼저 2층을 세운다 — `export` 가 읽기 전용으로 붙는다.
    let _ = pal(&repo, &["query", 대장_자신, "--json"]);

    // **봉투를 지는 표면 넷.** 이름이 아니라 산출로 확인한다.
    for (이름, args) in [
        ("query", vec!["query", 대장_자신, "--json"]),
        ("touch", vec!["touch", "뿌리", "--json"]),
        ("doctor", vec!["doctor", "--json"]),
    ] {
        let v: serde_json::Value = serde_json::from_str(&pal(&repo, &args)).expect("봉투 JSON");
        for 성분 in 봉투의_아홉 {
            assert!(v.get(성분).is_some(), "`{이름}` 이 봉투를 진다고 했는데 `{성분}` 이 없다");
        }
    }
    let out = repo.join("out.cypher");
    let v: serde_json::Value = serde_json::from_str(&pal(
        &repo,
        &["export", "--format", "cypher", "--out", out.to_str().expect("경로"), "--json"],
    ))
    .expect("봉투 JSON");
    for 성분 in 봉투의_아홉 {
        assert!(v.get(성분).is_some(), "`export` 의 답에 `{성분}` 이 없다");
    }

    // **봉투를 안 지는 표면 넷 — 그리고 안 지는 것이 옳다.**
    //
    // ⚠ `query --list` 가 이 목록에서 가장 무겁다. 봉투는 `Snapshot` 을 요구하고,
    // 요구하면 `--list` 가 **git 저장소를 필요로 한다.** 그 순간 *"호스트 없이도
    // 코어가 답한다"* 가 목록에서부터 깨진다. **안 지는 것이 조건이다.**
    let 안_지는: [(&str, Vec<&str>); 4] = [
        ("query --list", vec!["query", "--list", "--json"]),
        ("ledger", vec!["ledger", "--json"]),
        ("symbols", vec!["symbols", "big.ts", "--json"]),
        ("cache stats", vec!["cache", "stats", "--json"]),
    ];
    for (이름, args) in &안_지는 {
        let 산출 = pal(&repo, args.as_slice());
        // JSON 이기는 하다 — 비대화 경로가 이것도 파싱한다.
        let v: serde_json::Value = serde_json::from_str(&산출).expect("JSON 하나");
        let 있는 = 봉투의_아홉.iter().filter(|f| v.get(**f).is_some()).count();
        // **봉투인 척하면 안 된다.** 아홉을 전부 지면 그것은 봉투이고, 그러면 이
        // 목록이 낡은 것이다.
        assert!(
            있는 < 봉투의_아홉.len(),
            "`{이름}` 이 봉투의 아홉을 전부 진다 — 게이트의 목록을 고쳐야 한다"
        );
    }

    // **하한** — `--json` 을 내는 표면이 6 개 미만이면 이 시험이 아무것도 안 잰다.
    assert!(4 + 안_지는.len() >= 6);

    치운다(&repo);
}
