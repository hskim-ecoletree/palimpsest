//! **호스트 없이 · 대화 없이 · 우리 밖 도구로도** — `[f06.3.pass]` ①②③④.
//!
//! # ★ 2026-08-18 — 이 시험이 무엇을 재는지 다시 못 박는다
//!
//! [ADR-0025](../../../docs/adr/0025-the-harness-that-reads-the-graph-is-the-same-product.md)
//! 가 **호스트 중립을 초석에서 내렸다**(Claude Code 전용). 그래서 이 시험의 옛 근거
//! 문서 셋(`WHITEPAPER.md` 의 P7 · `docs/DESIGN.md` 의 P7 반증 조건 ·
//! `docs/how-it-works.md`)이 2026-08-18 재고 처분으로 사라진다.
//!
//! **그렇다고 이 시험이 고아가 되지는 않는다.** ADR-0025 가 내린 것은 **하네스 층**의
//! 호스트 중립이고, 같은 문서가 *"다른 호스트 사용자는 코어(`pal` CLI)만 쓴다"* 를
//! 남겼다. 이 파일이 재는 것은 정확히 **그 코어**다 — 어떤 호스트도 없는 환경에서
//! 전 질의가 답(공백 포함)을 내는가. 그 계약은 살아 있고, 근거는 이제 ADR-0025 다.
//!
//! ⚠ **왜 이걸 적어 두나**: 근거 문서가 사라진 채 초록으로 도는 검사는
//! *"왜 있는지 아무도 모르는 불변식"* 이 되고, 그것이 이 저장소가 없애려는 거짓
//! 신호의 코드판이다.
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
/// 인자를 안 받고 **빈 목록으로 답하는** 질의 — F10.
const 서술물_질의: &str = "narrative.unbound";
/// **좌표가 아니라 계획 문서를 받는** 질의 — F12.
const 이탈_질의: &str = "plan.deviation";
/// 계획 문서가 받는 인자의 타입 — 카탈로그가 적은 이름 그대로.
const 문서_인자: &str = "RepoPath";

/// 계획 문서의 자리 — **저장소 밖이다.**
///
/// ⚠ **안에 두면 `narrative.unbound` 가 실패한다.** 그 질의는 저장소의 마크다운을
/// 인입하는데 `pal query` 는 의도 저장소를 **읽기로** 열고, 인입은 개체 이름을 쓰려
/// 한다. **F12 가 만든 결함이 아니라 `.md` 파일 하나가 드러낸 F10 표면의 결함이고**,
/// 그 사실은 `docs/gates/F12.md` 가 「다음으로 넘기는 것」에 적는다.
fn 계획_문서(repo: &Path) -> PathBuf {
    let path = repo.with_extension("plan.md");
    // 기준선을 `HEAD` 로 둔다 — 두 스냅샷이 같아 이탈이 **정의되지 않는데**, 이 시험이
    // 재는 것은 *"답의 갈래와 종료 코드"* 이지 이탈의 값이 아니다.
    std::fs::write(&path, "---\nbaseline: HEAD\n---\n# 계획\n무엇을 왜\n\n## 하나\n`도움` 을 고친다\n")
        .expect("plan.md");
    path
}

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
///
/// # ⚠ 그리고 **인자의 타입도 거기서 뜬다** (F12 가 그 자리를 늘렸다)
///
/// 옛 판은 인자를 받는 질의가 **전부 `SymbolName` 을 받는다**고 가정하고 없는 이름을
/// 넘겼다. `plan.deviation` 은 **계획 문서의 경로**를 받으므로 그 가정이 깨진다 —
/// 없는 파일을 넘기면 그것은 *"못 찾은 이름"*(답이다)이 아니라 **잘못된 호출**이고,
/// 종료 코드가 1 인 것이 옳다. 위의 「이름으로 세지 않는다」와 **같은 교훈이다.**
fn 질의_이름들() -> Vec<(String, Option<String>)> {
    let out = Command::new(PAL).args(["query", "--list", "--json"]).output().expect("pal");
    let v: serde_json::Value = serde_json::from_slice(&out.stdout).expect("목록 JSON");
    v["built"]
        .as_array()
        .expect("built")
        .iter()
        .map(|q| {
            let 인자_타입 = q["args"]
                .as_array()
                .expect("args")
                .first()
                .map(|a| a["type"].as_str().expect("인자 타입").to_owned());
            (q["name"].as_str().expect("이름").to_owned(), 인자_타입)
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

/// 쓰기로 붙는다 — **유계 재시도.** 준비 전용이고 **단정에 쓰지 않는다.**
///
/// ★ **왜 필요한가** (실측 2026-08-18 · CI 회차 32049575037 · `macos-latest`):
/// 읽기 전용 핸들을 `drop` 한 **직후** 같은 파일을 쓰기로 여는 자리에서
/// `Database already open. Cannot acquire lock.` 이 났다. 이 기계에서는 몇 번을 돌려도
/// 통과하고 전체 시험도 통과하므로 **부하 의존**이다 — 파일 락이 핸들이 닫힌 직후에도
/// 아주 잠깐 남는다.
///
/// ⚠ **수를 안 적는다.** 「시험 N 개가 통과한다」는 다음 커밋에 낡는 캐시다
/// (실제로 이 회차의 MCP 삭제가 774 를 755 로 만들었다). 재는 방법은 `cargo xtask test` 다.
///
/// ★ **같은 형태가 이 파일에 네 자리 있다** — 자식 프로세스가 배타 락을 놓은 직후,
/// 임시 핸들이 문장 끝에서 drop 된 직후. 넷 다 이 헬퍼를 지난다. 하나만 고치면
/// 깜빡임이 옆줄로 옮겨갈 뿐이다.
///
/// ⚠ **재시도를 단정에 걸면 안 된다.** 이 파일이 세우는 단정은
/// *"쓰기와 읽기는 공존하지 않는다"* 이고, 그 자리에 재시도를 걸면 단정이 재시도로
/// 뭉개진다. 여기서 쓰기를 얻는 것은 그 단정을 **세우기 위한 준비**다.
fn 쓰기로_붙는다(index: &Path) -> pal_store::Projection {
    붙는다(index, "쓰기", pal_store::Projection::open)
}

/// 읽기 전용으로 붙는다 — 같은 이유의 유계 재시도. **준비 전용이다.**
fn 읽기로_붙는다(index: &Path) -> pal_store::Projection {
    붙는다(index, "읽기 전용", pal_store::Projection::open_read_only)
}

fn 붙는다(
    index: &Path,
    무엇: &str,
    열기: fn(&Path) -> Result<pal_store::Projection, pal_store::ProjectionError>,
) -> pal_store::Projection {
    let mut 마지막 = None;
    for _ in 0..50 {
        match 열기(index) {
            Ok(p) => return p,
            Err(e) => {
                마지막 = Some(e);
                std::thread::sleep(std::time::Duration::from_millis(40));
            }
        }
    }
    panic!("{무엇}으로 붙지 못했다 (2초 재시도): {:?}", 마지막.expect("오류가 하나는 있다"));
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

    let 계획 = 계획_문서(&repo);
    let 계획 = 계획.to_string_lossy();
    let mut 갈래: Vec<(String, String)> = Vec::new();
    for (name, 인자_타입) in &이름들 {
        let mut args = vec!["query", name.as_str()];
        // ⚠ **상수를 `match` 패턴에 쓰지 않는다.** 이름이 바인딩으로 읽히면 모든
        // 질의가 계획 문서를 받고, 그래도 이 시험은 **통과한다**(경로는 심볼 이름으로
        // 안 풀려 `unknown` 이 나온다). 조용히 꺼지는 대조라 `==` 로 적는다.
        if 인자_타입.as_deref() == Some(문서_인자) {
            args.push(&계획);
        } else if 인자_타입.is_some() {
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
            // ★ **문서 조각이 0 건인 것도 「안 만듦」이 아니다.** 빈 저장소에는
            // 문서가 없고, 그 사실이 `narrative` 갈래의 **빈 목록**으로 나온다 —
            // `not_built` 로 내면 거짓말이 되고 `unknown` 으로 내면 *"못 찾았다"* 가
            // 된다. 둘 다 아니다: **물었고, 없었다.**
            서술물_질의 => "narrative",
            // ★ **계획이 좌표를 하나도 못 풀어도 `unknown` 이 아니다.** 물었고,
            // 답이 나왔고, 못 잰 것이 **`unmeasurable` 로 갈려 있다**(F12 §2).
            이탈_질의 => "deviation",
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
    let _ = std::fs::remove_file(repo.with_extension("plan.md"));
}

#[test]
fn 비대화_경로가_전_질의에_닿는다() {
    let repo = 있는_저장소("pipe");
    let 이름들 = 질의_이름들();

    let mut 종료_갈래: std::collections::BTreeSet<i32> = std::collections::BTreeSet::new();

    let 계획 = 계획_문서(&repo);
    let 계획 = 계획.to_string_lossy();
    for (name, 인자_타입) in &이름들 {
        let mut args = vec!["query", name.as_str()];
        if 인자_타입.as_deref() == Some(문서_인자) {
            args.push(&계획);
        } else if 인자_타입.is_some() {
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
    let _ = std::fs::remove_file(repo.with_extension("plan.md"));
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
        쓰기로_붙는다(&index).count().expect("셈");
    // **하한** — 2층이 비면 아래의 락 대조가 무엇에 대한 것인지 알 수 없다.
    assert!(세운_뒤 >= 1, "2층에 심볼이 {세운_뒤}개다");

    // ── ① 읽기 전용 **둘**이 동시에 붙는다 ───────────────────────────────────
    let 읽기_하나 = 읽기로_붙는다(&index);
    let 읽기_둘 = 읽기로_붙는다(&index);
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
    let 쓰는_쪽 = 쓰기로_붙는다(&index);
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
    let _ = std::fs::remove_file(repo.with_extension("plan.md"));
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
        let p = 쓰기로_붙는다(&index);
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
    let _ = std::fs::remove_file(repo.with_extension("plan.md"));
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
    let _ = std::fs::remove_file(repo.with_extension("plan.md"));
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
    let _ = std::fs::remove_file(repo.with_extension("plan.md"));
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
    let _ = std::fs::remove_file(repo.with_extension("plan.md"));
}
