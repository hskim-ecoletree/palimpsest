//! **호스트 없이 · 대화 없이 · 우리 밖 도구로도** — `[f06.3.pass]` ①②③④.
//!
//! # 「답이 비었다」가 실패가 아닌 경우를 **먼저** 세운다
//!
//! 대조가 꺼지는 **열셋째** 형태가 이 자리다 — 옳은 산출을 어긋남으로 읽는 것.
//! 관측 0 건에서 `symbol.resolve X` 가 `Unknown` 을 내는 것은 **옳다.** 그것을
//! *"답이 없다"* 로 세면 옳은 산출이 반증으로 적힌다. 그래서 아래는 *"답이 있다"* 가
//! 아니라 ***"답의 갈래가 무엇인가"***를 잰다.

mod common;

use common::{PAL, git};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// 이름을 받는 질의는 **없는 이름**에 `unknown` 으로 답해야 한다.
/// `graph.dump` 만 빈 목록이 옳다 — 그 질의는 *"전부"* 가 답이고 전부가 0 건이다.
const 전부가_답인_질의: &str = "graph.dump";
const 대장_질의: &str = "ledger.snapshot";
/// 인자를 안 받고 **빈 목록이 정직한** 질의 — 능력이 있고 값이 없는 것이다.
const 결박_질의: &str = "binding.status";

fn 빈_저장소(tag: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("pal-f06-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("임시 저장소");
    git(&root, &["init", "-q", "."]);
    git(
        &root,
        &["-c", "user.email=t@example.com", "-c", "user.name=t", "commit", "-q", "--allow-empty", "-m", "빈 첫 커밋"],
    );
    root
}

fn 있는_저장소(tag: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("pal-f06-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("임시 저장소");
    std::fs::write(
        root.join("a.ts"),
        "export function 도움() { return 1 }\nexport function 부름() { return 도움() }\n",
    )
    .expect("a.ts");
    git(&root, &["init", "-q", "."]);
    git(&root, &["add", "-A"]);
    git(&root, &["-c", "user.email=t@example.com", "-c", "user.name=t", "commit", "-qm", "첫"]);
    root
}

/// 이 빌드가 답하는 질의 — **이름과 「인자를 받는가」를 함께.**
///
/// # 이름을 손으로 나열하지 않는다
///
/// 옛 판은 인자를 받는 질의를 *"`graph.dump` 와 `ledger.snapshot` 이 아닌 것"* 으로
/// 셌다. **F09 가 `binding.status`(인자 없음)를 더하자 그 셈이 틀렸다** — 없는 이름을
/// 인자로 넘겨 `unknown` 을 기대했는데 그 질의는 인자를 안 받는다.
///
/// 카탈로그가 이미 그 사실을 안다(`arg_names()`). **거기서 뜬다** — 이름으로 세는
/// 검사는 이름이 하나 늘 때마다 조용히 틀린다.
fn 질의_이름들() -> Vec<(String, bool)> {
    let out = Command::new(PAL).args(["query", "--list", "--json"]).output().expect("pal");
    let v: serde_json::Value = serde_json::from_slice(&out.stdout).expect("목록 JSON");
    v["built"]
        .as_array()
        .expect("built")
        .iter()
        .map(|q| {
            let 인자를_받나 = !q["args"].as_array().expect("args").is_empty();
            (q["name"].as_str().expect("이름").to_owned(), 인자를_받나)
        })
        .collect()
}

/// **파이프로 돌린다** — 표준입력이 tty 가 아니고 표준출력이 파이프다.
/// 셸이 tty 를 물려주면 「파이프여도」가 안 재어진다(`[f06.3.pass]` ②).
fn 파이프로(repo: &Path, args: &[&str]) -> (Option<i32>, String, String) {
    let out = Command::new(PAL)
        .args(args)
        .current_dir(repo)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("pal 을 못 돌렸다");
    (
        out.status.code(),
        String::from_utf8(out.stdout).expect("UTF-8"),
        String::from_utf8(out.stderr).expect("UTF-8"),
    )
}

#[test]
fn 관측_0_건에서_여섯이_전부_답한다() {
    let repo = 빈_저장소("empty");
    let 이름들 = 질의_이름들();
    assert!(이름들.len() >= 6, "질의가 {}개다", 이름들.len());

    // **하한** — 진짜로 비어 있는지 먼저 단언한다. 앞 회차의 `.palimpsest/` 가 남아
    // 있거나 파일이 하나라도 있으면 이 시험이 무엇을 재는지 알 수 없다.
    let (코드, 대장, _) = 파이프로(&repo, &["query", 대장_질의, "--json"]);
    assert_eq!(코드, Some(0));
    let v: serde_json::Value = serde_json::from_str(&대장).expect("봉투 JSON");
    assert_eq!(v["answer"]["ledger"]["files_total"].as_u64(), Some(0), "저장소가 안 비었다");

    let mut 갈래: Vec<(String, String)> = Vec::new();
    for (name, 인자를_받나) in &이름들 {
        let mut args = vec!["query", name.as_str()];
        if *인자를_받나 {
            args.push("이런것은없다");
        }
        args.push("--json");
        let (코드, 산출, 오류) = 파이프로(&repo, &args);
        // ★ **여섯 다 종료 0 이고 봉투를 진다.** 하나라도 실패하면 호스트 독립성이 깨진다.
        assert_eq!(코드, Some(0), "`{name}` 이 빈 저장소에서 실패했다: {오류}");
        let v: serde_json::Value = serde_json::from_str(&산출).expect("봉투 JSON");
        // **소비자가 질의 없이 능력을 안다** — 빈 답이 「없음」인지 「안 만듦」인지 가른다.
        assert!(!v["capabilities"]["not_built"].as_array().expect("not_built").is_empty());
        갈래.push((name.clone(), v["answer"]["outcome"].as_str().expect("outcome").to_owned()));
    }

    for (name, outcome) in &갈래 {
        let 옳은 = match name.as_str() {
            대장_질의 => "ledger",
            전부가_답인_질의 => "graph",
            // **결박이 0 건인 것과 「안 만듦」은 다르다.** 이 빌드에는 결박 능력이
            // 있고 아무도 안 걸었을 뿐이다 — `not_built` 로 내면 거짓말이 된다.
            결박_질의 => "bindings",
            // ★ **`symbols` 가 아니라 `unknown` 이다.** 빈 목록으로 답하면
            // *"없다"* 와 *"못 찾았다"* 가 같은 출력이 된다.
            _ => "unknown",
        };
        assert_eq!(outcome, 옳은, "`{name}` 의 답의 갈래가 틀렸다");
    }

    // **하한** — 갈래가 하나뿐이면 위의 표가 아무것도 안 가른다.
    let 갈래_수: std::collections::BTreeSet<&str> =
        갈래.iter().map(|(_, o)| o.as_str()).collect();
    assert!(갈래_수.len() >= 3, "답의 갈래가 {}가지뿐이다", 갈래_수.len());

    let _ = std::fs::remove_dir_all(&repo);
}

#[test]
fn 비대화_경로가_전_질의에_닿는다() {
    let repo = 있는_저장소("pipe");
    let 이름들 = 질의_이름들();

    let mut 종료_갈래: std::collections::BTreeSet<i32> = std::collections::BTreeSet::new();

    for (name, 인자를_받나) in &이름들 {
        let mut args = vec!["query", name.as_str()];
        if *인자를_받나 {
            args.push("도움");
        }
        args.push("--json");
        let (코드, 산출, 오류) = 파이프로(&repo, &args);
        종료_갈래.insert(코드.expect("종료 코드"));
        assert_eq!(코드, Some(0), "`{name}`: {오류}");
        // **표준 라이브러리 파서로 읽힌다** — `jq` 같은 도구가 없어도 된다.
        let _: serde_json::Value = serde_json::from_str(&산출).expect("파싱되는 JSON 하나");
        // **사람용 장식이 안 샌다.**
        assert!(!산출.contains('\u{1b}'), "`{name}` 의 `--json` 에 ANSI 이스케이프가 있다");
        assert!(!산출.contains('■'), "`{name}` 의 `--json` 에 사람용 머리글이 샜다");
    }

    // 봉투가 안 나가는 쪽 — **오류는 표준오류로 간다.**
    let (코드, 산출, 오류) = 파이프로(&repo, &["query", "이런질의는없다"]);
    종료_갈래.insert(코드.expect("종료 코드"));
    assert_eq!(코드, Some(1));
    assert!(산출.is_empty(), "봉투가 안 나갔는데 표준출력에 무언가 있다");
    assert!(!오류.is_empty(), "아무 말도 안 했다");

    // ★ **「못 찾았다」는 실패가 아니다** — 봉투가 나갔으므로 0 이다.
    let (코드, 산출, _) = 파이프로(&repo, &["query", "symbol.resolve", "없는이름", "--json"]);
    assert_eq!(코드, Some(0), "「못 찾았다」가 실패로 끝났다");
    let v: serde_json::Value = serde_json::from_str(&산출).expect("봉투 JSON");
    assert_eq!(v["answer"]["outcome"].as_str(), Some("unknown"));

    // **하한** — 갈래가 하나뿐이면 종료 코드 계약이 안 재어진다.
    assert!(종료_갈래.len() >= 2, "종료 코드 갈래가 {}가지뿐이다", 종료_갈래.len());

    let _ = std::fs::remove_dir_all(&repo);
}

/// `[f06.3.pass]` ③ — F05 가 넘긴 자리다. **그리고 여기서 등록이 반증됐다.**
///
/// # 등록한 것과 실물이 다르다 — 적어 둔다
///
/// `[f06.3.pass]` ③ 이 ★ 로 요구한 것은 *"쓰기로 붙은 프로세스가 살아 있는 동안 읽기
/// 프로세스가 답을 낸다"* 였다. **`redb` 4.1 에서 그것은 성립하지 않는다.**
/// `Builder::open_read_only` 의 문서가 명시한다:
///
/// > If the file has been opened for writing (i.e. as a `Database`)
/// > `DatabaseError::DatabaseAlreadyOpen` will be returned on platforms which support
/// > file locks (macOS, Windows, Linux).
///
/// 공유 락(`try_lock_shared`)은 **다른 읽기와만** 공존한다. F05 §6 의 표가 적은
/// *"읽기는 동시 가능, 쓰기는 하나"* 중 **앞 절반만 참**이고, *"CLI 는 읽기 전용으로
/// 붙는다"* 가 해결하는 것은 **읽기 여럿의 공존**이지 쓰기와의 공존이 아니다.
///
/// **그래서 이 시험은 실물이 주는 것을 잰다** — 그리고 등록이 요구한 쪽은
/// `docs/gates/F06.md` 에 **어긋남**으로 적힌다. 낮춰서 통과시키지 않는다.
#[test]
fn 읽기_전용_여럿이_동시에_붙고_쓰기는_배타다() {
    let repo = 있는_저장소("lock");
    let index = repo.join(".palimpsest/index.redb");

    // 먼저 한 번 쓰기로 세운다 — 읽기 전용은 없는 2층에 못 붙는다.
    let (코드, _, 오류) = 파이프로(&repo, &["query", 대장_질의, "--json"]);
    assert_eq!(코드, Some(0), "2층을 세우지 못했다: {오류}");
    let 세운_뒤 =
        pal_store::Projection::open(&index).expect("붙는다").count().expect("셈");
    // **하한** — 2층이 비면 아래의 락 대조가 무엇에 대한 것인지 알 수 없다.
    assert!(세운_뒤 >= 1, "2층에 심볼이 {세운_뒤}개다");

    // ── ① 읽기 전용 **둘**이 동시에 붙는다 ───────────────────────────────────
    let 읽기_하나 = pal_store::Projection::open_read_only(&index).expect("첫 읽기");
    let 읽기_둘 = pal_store::Projection::open_read_only(&index).expect("둘째 읽기");
    assert!(읽기_하나.is_read_only() && 읽기_둘.is_read_only());
    // **둘 다 답한다** — 붙기만 하고 못 읽으면 아무 말도 안 한 것이다.
    assert_eq!(읽기_하나.count().expect("셈"), 세운_뒤);
    assert_eq!(읽기_둘.count().expect("셈"), 세운_뒤);

    // 그리고 **세 번째가 프로세스 밖에서** 붙는다 — 같은 프로세스 안이라 붙는 것이
    // 아니라는 증거다(파일 락은 프로세스 단위다).
    let (읽기_코드, 산출, 읽기_오류) =
        파이프로(&repo, &["query", 대장_질의, "--read-only", "--json"]);
    assert_eq!(읽기_코드, Some(0), "읽기 둘이 붙어 있는데 셋째가 실패했다: {읽기_오류}");
    let v: serde_json::Value = serde_json::from_str(&산출).expect("봉투 JSON");
    // ★ **못 남긴 사실이 답에 실린다.** 조용히 빠지면 F17 이 미조회를 과대 계상한다.
    assert_eq!(v["log"]["status"].as_str(), Some("not_recorded"));
    assert_eq!(v["log"]["why"].as_str(), Some("read_only_attach"));

    // 내보내기도 읽기 전용이라 같은 자리에 붙는다.
    let (내보내기_코드, cypher, _) = 파이프로(&repo, &["export", "--format", "cypher"]);
    assert_eq!(내보내기_코드, Some(0), "읽기 둘이 붙어 있는데 내보내기가 실패했다");
    assert!(cypher.contains("CREATE"), "Cypher 가 비었다");

    // ★ **통제 — 같은 상황에서 쓰기는 실패해야 한다.**
    //   실패하지 않으면 락이 애초에 안 겹친 것이고 위의 성공이 아무 말도 안 한다.
    let (쓰기_코드, _, 쓰기_오류) = 파이프로(&repo, &["query", 대장_질의, "--json"]);
    assert_eq!(쓰기_코드, Some(1), "읽기가 붙어 있는데 쓰기가 성공했다");
    assert!(쓰기_오류.contains("2층"), "실패 사유가 2층이 아니다: {쓰기_오류}");

    drop(읽기_하나);
    drop(읽기_둘);

    // ── ② ⚠ **등록이 요구한 쪽은 성립하지 않는다** — 실물을 적는다 ────────────
    let 쓰는_쪽 = pal_store::Projection::open(&index).expect("쓰기로 붙는다");
    assert!(!쓰는_쪽.is_read_only());
    let (코드, _, 오류) = 파이프로(&repo, &["query", 대장_질의, "--read-only", "--json"]);
    assert_eq!(
        코드,
        Some(1),
        "쓰기와 읽기가 공존했다 — `redb` 4.1 의 문서와 다르다. \
         그렇다면 `docs/gates/F06.md` 의 어긋남 기록을 지워야 한다"
    );
    assert!(오류.contains("읽기 전용"), "{오류}");
    drop(쓰는_쪽);

    let _ = std::fs::remove_dir_all(&repo);
}

/// 이 저장소의 스냅샷 열쇠 — 사람이 읽는 화면이 그것을 적는다.
fn 스냅샷_열쇠(repo: &Path) -> String {
    let (_, 화면, _) = 파이프로(repo, &["query", 대장_질의]);
    화면
        .lines()
        .find_map(|l| l.trim().strip_prefix("Snapshot "))
        .map(|s| s.trim().to_owned())
        .expect("화면에 Snapshot 이 없다")
}

#[test]
fn 로그_줄이_실제로_늘고_읽기_전용에서는_안_는다() {
    // `[f06.2.pass]` ⑤ — **값을 안 보고 필드만 보면 늘 통과한다.**
    // 로그 줄 수를 세는 것이 이 필드가 거짓말하는지 아는 유일한 방법이다.
    let repo = 있는_저장소("log");
    let index = repo.join(".palimpsest/index.redb");

    let (코드, _, 오류) = 파이프로(&repo, &["query", 대장_질의, "--json"]);
    assert_eq!(코드, Some(0), "{오류}");
    let 열쇠 = 스냅샷_열쇠(&repo);

    let 세기 = |키: &str| -> usize {
        let p = pal_store::Projection::open(&index).expect("붙는다");
        p.query_log(키).expect("로그를 읽는다").len()
    };

    let 처음 = 세기(&열쇠);
    // **하한** — 로그가 0 줄이면 아래의 「늘었다」가 무엇에 대한 것인지 알 수 없다.
    assert!(처음 >= 1, "로그가 {처음}줄이다 — 앞의 질의가 안 남았다");

    // 쓰기로 붙은 질의 하나 → **한 줄 는다.**
    let (코드, 산출, _) = 파이프로(&repo, &["query", "symbol.resolve", "도움", "--json"]);
    assert_eq!(코드, Some(0));
    let v: serde_json::Value = serde_json::from_str(&산출).expect("봉투");
    assert_eq!(v["log"]["status"].as_str(), Some("recorded"));
    let 쓴_뒤 = 세기(&열쇠);
    assert_eq!(쓴_뒤, 처음 + 1, "`recorded` 라고 적었는데 로그가 안 늘었다");

    // 읽기 전용으로 붙은 질의 하나 → **안 는다.**
    let (코드, 산출, 오류) =
        파이프로(&repo, &["query", "symbol.resolve", "도움", "--read-only", "--json"]);
    assert_eq!(코드, Some(0), "{오류}");
    let v: serde_json::Value = serde_json::from_str(&산출).expect("봉투");
    assert_eq!(v["log"]["status"].as_str(), Some("not_recorded"));
    assert_eq!(세기(&열쇠), 쓴_뒤, "`not_recorded` 라고 적었는데 로그가 늘었다");

    let _ = std::fs::remove_dir_all(&repo);
}

#[test]
fn 읽기_전용은_없는_2층에_조용히_안_붙는다() {
    // 되돌아가면 `--read-only` 가 거짓말이 되고, 부르는 쪽은 자기가 **배타 락을
    // 쥐었다는 것**을 모른다.
    let repo = 있는_저장소("noindex");
    let (코드, 산출, 오류) = 파이프로(&repo, &["query", 대장_질의, "--read-only", "--json"]);
    assert_eq!(코드, Some(1), "2층이 없는데 읽기 전용이 성공했다");
    assert!(산출.is_empty(), "실패했는데 표준출력에 무언가 나갔다");
    assert!(오류.contains("읽기 전용"), "사유가 읽기 전용이 아니다: {오류}");
    // 그리고 **2층을 만들지 않았다** — 만들었으면 조용히 쓰기로 되돌아간 것이다.
    assert!(!repo.join(".palimpsest/index.redb").exists(), "읽기 전용이 2층을 만들었다");
    let _ = std::fs::remove_dir_all(&repo);
}

#[test]
fn 내보내기의_라벨이_스키마에서_오고_못_낸_것을_적는다() {
    // `[f06.3.pass]` ④ — F05 §2 의 다섯째 근거가 검사되는 자리.
    let repo = 있는_저장소("export");
    let (코드, _, _) = 파이프로(&repo, &["query", 대장_질의, "--json"]);
    assert_eq!(코드, Some(0));

    let cypher_path = repo.join("out.cypher");
    let (코드, 산출, 오류) = 파이프로(
        &repo,
        &["export", "--format", "cypher", "--out", cypher_path.to_str().expect("경로"), "--json"],
    );
    assert_eq!(코드, Some(0), "{오류}");
    let v: serde_json::Value = serde_json::from_str(&산출).expect("봉투 JSON");
    let text = std::fs::read_to_string(&cypher_path).expect("Cypher");

    // **하한** — 노드가 0 개면 아래가 전부 공짜로 통과한다.
    let 낸_것 = v["answer"]["exported"].as_array().expect("exported");
    let 노드_합: u64 = 낸_것
        .iter()
        .filter(|c| c["label"].as_str() != Some("REFERENCES"))
        .map(|c| c["count"].as_u64().expect("건수"))
        .sum();
    assert!(노드_합 >= 1, "내보낸 노드가 0 개다");

    // ★ **라벨이 스키마의 키다.** 손으로 쓴 라벨이 하나라도 있으면 여기서 갈린다.
    let schema = pal_core::GraphSchema::parse(include_str!("../../../schema/graph.toml"))
        .expect("스키마");
    for c in 낸_것 {
        let label = c["label"].as_str().expect("라벨");
        assert!(
            schema.nodes.contains_key(label) || schema.edges.contains_key(label),
            "`{label}` 이 스키마에 없다 — 손으로 쓴 라벨이다"
        );
        assert!(text.contains(label), "Cypher 에 `{label}` 이 없다");
    }

    // ★ **못 낸 것을 0 건이 아니라 사유와 함께 적는다**(ADR-0002).
    let 못_낸 = v["answer"]["missing"].as_array().expect("missing");
    assert!(!못_낸.is_empty(), "스키마의 라벨 열여섯 중 못 낸 것이 0 개일 수 없다");
    let 사유들: std::collections::BTreeSet<&str> =
        못_낸.iter().map(|m| m["why"].as_str().expect("사유")).collect();
    assert!(사유들.contains("not_built"), "`not_built` 사유가 하나도 없다");
    assert!(사유들.contains("not_stored"), "`not_stored` 사유가 하나도 없다");
    // 갈래가 하나면 *"안 만들었다"* 와 *"여기 안 산다"* 가 뭉개진 것이다.
    assert_eq!(사유들.len(), 2, "못 낸 사유의 갈래가 둘이 아니다");

    // **개수가 `graph.dump` 와 같다.**
    let (_, dump, _) = 파이프로(&repo, &["query", "graph.dump", "--json"]);
    let d: serde_json::Value = serde_json::from_str(&dump).expect("봉투");
    let 노드 = d["answer"]["nodes"].as_array().expect("nodes").len() as u64;
    let 엣지 = d["answer"]["edges"].as_array().expect("edges").len() as u64;
    let 낸_심볼 = 낸_것
        .iter()
        .find(|c| c["label"].as_str() == Some("Symbol"))
        .and_then(|c| c["count"].as_u64())
        .expect("Symbol 건수");
    let 낸_엣지 = 낸_것
        .iter()
        .find(|c| c["label"].as_str() == Some("REFERENCES"))
        .and_then(|c| c["count"].as_u64())
        .expect("REFERENCES 건수");
    assert_eq!(낸_심볼, 노드, "내보낸 심볼 수가 `graph.dump` 와 다르다");
    assert_eq!(낸_엣지, 엣지, "내보낸 엣지 수가 `graph.dump` 와 다르다");

    // **문법을 우리가 검증하지 못한다** — Cypher 파서가 없다. 최소한 균형은 센다.
    // 그 사실은 게이트에 **대조 불가**로 적는다.
    assert_eq!(text.matches('{').count(), text.matches('}').count(), "중괄호가 안 맞는다");
    assert_eq!(text.matches('(').count(), text.matches(')').count(), "괄호가 안 맞는다");
    assert!(text.matches('"').count() % 2 == 0, "따옴표 수가 홀수다");

    let _ = std::fs::remove_dir_all(&repo);
}

#[test]
fn 산출과_근거가_다른_줄기로_간다() {
    // `--out` 없이 내보내면 Cypher 가 표준출력을 쓴다. **근거를 버리지 않고**
    // 표준오류로 보낸다 — 섞으면 파이프의 다음 단계가 깨진다.
    let repo = 있는_저장소("streams");
    let (코드, _, _) = 파이프로(&repo, &["query", 대장_질의, "--json"]);
    assert_eq!(코드, Some(0));

    let (코드, 산출, 오류) = 파이프로(&repo, &["export", "--format", "cypher"]);
    assert_eq!(코드, Some(0));
    assert!(산출.starts_with("// pal export"), "표준출력이 Cypher 가 아니다");
    assert!(!산출.contains("■"), "근거가 산출에 섞였다");
    assert!(오류.contains("못 낸 라벨"), "근거가 표준오류로 안 갔다");

    // **둘 다 표준출력으로 갈 수는 없다.**
    let (코드, _, 오류) = 파이프로(&repo, &["export", "--format", "cypher", "--json"]);
    assert_eq!(코드, Some(1), "`--json` 이 `--out` 없이 성공했다");
    assert!(오류.contains("--out"));

    let _ = std::fs::remove_dir_all(&repo);
}
