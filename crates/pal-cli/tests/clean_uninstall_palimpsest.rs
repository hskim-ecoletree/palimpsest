//! **`.palimpsest/` 안의 분류** — 회차 `2026-09-15-clean-uninstall` 완수 조건 B1 · B2 · B3 · B4 · B5 · B6 · B7.
//!
//! 스냅샷은 [`common::snapshot`] 한 자리의 자를 쓴다(조건 머리말 「스냅샷의 정의」).
//! **모든 스냅샷 시험은 대상이 실제로 생겼음을 uninstall 전에 단언한다** — 안 생긴 것을 안 남았다고 세지 않는다.
//!
//! 밖의 저장소는 시험마다 `PAL_APPROVAL_DIR` 로 방 밖의 임시 자리에 둔다 — 이 파일이 재는 것은 워킹트리다.

mod common;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use common::snapshot::{self, 갈린_경로, 워킹트리};
use common::{PAL, git, path_앞에};

// ─────────────────────────────────────────────────────────────────────────────
// 방
// ─────────────────────────────────────────────────────────────────────────────

struct 방 {
    base: PathBuf,
    repo: PathBuf,
}

impl 방 {
    /// 빈 저장소 — 파일은 부르는 쪽이 놓고 [`방::커밋`] 한다.
    fn 새(tag: &str) -> Self {
        let base = std::env::temp_dir().join(format!("pal-cu-p-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
        let repo = base.join("repo");
        std::fs::create_dir_all(&repo).expect("방");
        std::fs::create_dir_all(base.join("appr")).expect("밖의 저장소");
        git(&repo, &["init", "-q", "."]);
        Self { base, repo }
    }

    /// 문서가 있어 `narrative` 가 개체를 만드는 방.
    fn 문서(tag: &str) -> Self {
        let 방 = Self::새(tag);
        방.쓴다("tsconfig.json", r#"{"compilerOptions":{"moduleResolution":"bundler"}}"#);
        방.쓴다(
            "src/core.ts",
            "export function helper() { return 0; }\nexport function target() { return helper(); }\n",
        );
        방.쓴다("docs/decisions.md", "# 결정들\n\n## 첫 결정\n\n`target` 은 `helper` 를 부른다.\n");
        방.쓴다("README.md", "hello\n");
        방.커밋("첫");
        방
    }

    fn 자리(&self, rel: &str) -> PathBuf {
        self.repo.join(rel)
    }

    fn 쓴다(&self, rel: &str, body: &str) {
        let p = self.자리(rel);
        std::fs::create_dir_all(p.parent().expect("부모")).expect("디렉터리");
        std::fs::write(p, body).expect("쓰기");
    }

    fn 덧붙인다(&self, rel: &str, body: &str) {
        let mut 지금 = std::fs::read(self.자리(rel)).expect("읽기");
        지금.extend_from_slice(body.as_bytes());
        std::fs::write(self.자리(rel), 지금).expect("쓰기");
    }

    fn 커밋(&self, msg: &str) {
        git(&self.repo, &["add", "-A"]);
        git(&self.repo, &["-c", "user.email=t@e", "-c", "user.name=t", "commit", "-qm", msg]);
    }

    fn 돌린다(&self, args: &[&str]) -> Output {
        Command::new(PAL)
            .args(args)
            .current_dir(&self.repo)
            .env("PATH", path_앞에(Path::new(PAL).parent().expect("pal 의 부모")))
            .env("PAL_APPROVAL_DIR", self.base.join("appr"))
            .output()
            .expect("pal 을 못 돌렸다")
    }

    fn 성공(&self, args: &[&str]) -> String {
        let out = self.돌린다(args);
        assert!(
            out.status.success(),
            "pal {args:?} rc={:?}\nstdout: {}\nstderr: {}",
            out.status.code(),
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
        String::from_utf8_lossy(&out.stdout).into_owned()
    }

    fn git_출력(&self, args: &[&str]) -> String {
        let out = Command::new("git").args(args).current_dir(&self.repo).output().expect("git");
        assert!(out.status.success(), "git {args:?}: {}", String::from_utf8_lossy(&out.stderr));
        String::from_utf8_lossy(&out.stdout).into_owned()
    }

    /// `narrative` 의 후보 제안 하나 — (개체 id, 지목 둘).
    fn 후보(&self) -> (String, Vec<String>) {
        let v: serde_json::Value =
            serde_json::from_str(&self.성공(&["narrative", "--json"])).expect("narrative JSON");
        let p = v["proposals"]
            .as_array()
            .expect("proposals")
            .iter()
            .find(|p| p["class"]["class"] == "candidates")
            .unwrap_or_else(|| panic!("후보 제안이 없다 — 이 방이 재려는 것이 안 생겼다: {v}"))
            .clone();
        let id = p["item"]["id"].as_str().expect("id").to_owned();
        let picks = p["class"]["candidates"]
            .as_array()
            .expect("candidates")
            .iter()
            .map(|c| c.as_str().expect("후보")[..12].to_owned())
            .collect();
        (id, picks)
    }

    /// 의도 저장소를 다시 열어 읽은 (결박 수 · 거부 수 · 개체 수). **닫고 돌려준다** — Windows 는 열린 파일을 못 지운다.
    fn 의도_수(&self) -> (usize, usize, usize) {
        let s = pal_intent::IntentStore::open_read_only(&self.자리(".palimpsest/intent.redb"))
            .expect("의도 저장소를 못 열었다");
        (
            s.count().expect("결박 수"),
            s.refusals().expect("거부").len(),
            s.entity_count().expect("개체 수"),
        )
    }
}

impl Drop for 방 {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.base);
    }
}

/// `pal intent export` 로 결박 정본 파일을 만든다 — 사용자가 하는 그대로.
fn 내보낸다(방: &방) {
    std::fs::create_dir_all(방.자리(".palimpsest/intent")).expect("intent/");
    방.성공(&["intent", "export", "--out", ".palimpsest/intent/bindings.jsonl"]);
}

/// 스냅샷에서 접두사 아래만.
fn 아래(s: &BTreeMap<String, snapshot::항목>, 접두사: &[&str]) -> BTreeMap<String, snapshot::항목> {
    s.iter()
        .filter(|(k, _)| 접두사.iter().any(|p| k.starts_with(p)))
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect()
}

/// 파생물 자리의 이름 — 시험이 스스로 적는 기대값이다(제품 목록을 베끼지 않는다).
fn 파생물_경로인가(rel: &str) -> bool {
    rel.starts_with(".palimpsest/cache")
        || rel == ".palimpsest/index.redb"
        || rel == ".palimpsest/narrative-pending.json"
        || (rel.starts_with(".palimpsest/radius-base-") && Path::new(rel).extension().is_some_and(|e| e == "redb"))
}

fn 파생물이_생겼다(방: &방) {
    for rel in [".palimpsest/cache", ".palimpsest/index.redb", ".palimpsest/intent.redb", ".palimpsest/narrative-pending.json"] {
        assert!(방.자리(rel).exists(), "전제 — 사용이 {rel} 를 만들어야 이 시험이 무언가를 잰다");
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// B1
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn b1_palimpsest_가_없던_방은_사용하고_거두면_워킹트리가_설치_전과_같다() {
    let 방 = 방::문서("b1");
    assert!(!방.자리(".palimpsest").exists(), "전제 — `.palimpsest/` 가 없던 방이다");
    let s0 = 워킹트리(&방.repo);

    방.성공(&["install"]);
    방.성공(&["touch", "target"]);
    방.성공(&["narrative"]);
    파생물이_생겼다(&방);
    let (결박, 거부, 개체) = 방.의도_수();
    assert!(개체 > 0, "전제 — narrative 가 intent.redb 에 개체를 만들어야 한다");
    assert_eq!((결박, 거부), (0, 0), "전제 — 결박·거부가 없는 방이다(그 방은 B3)");

    방.성공(&["uninstall"]);
    let 갈림 = 갈린_경로(&s0, &워킹트리(&방.repo));
    assert!(갈림.is_empty(), "설치 전과 갈린 경로가 있다: {갈림:#?}");
}

// ─────────────────────────────────────────────────────────────────────────────
// B2
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn b2_설치_전의_정본과_남의_파일은_그대로이고_설치_전의_파생물은_지우고_말한다() {
    let 방 = 방::문서("b2");
    방.쓴다(".palimpsest/rounds/old/intent.md", "# 옛 회차\n");
    방.쓴다(".palimpsest/rounds/old/notes.md", "메모\n");
    내보낸다(&방);
    방.커밋("정본");
    // 설치 전부터 있던 파생물 — pal 을 설치 없이 돌리면 생긴다.
    방.성공(&["touch", "target"]);
    방.성공(&["narrative"]);
    방.쓴다(".palimpsest/radius-base-0123456789ab.redb", "남은 기준 원장\n");
    for rel in [".palimpsest/index.redb", ".palimpsest/cache", ".palimpsest/intent/bindings.jsonl", ".palimpsest/rounds/old/intent.md"] {
        assert!(방.자리(rel).exists(), "전제 — {rel} 가 설치 전에 있어야 한다");
    }
    let s0 = 워킹트리(&방.repo);

    방.성공(&["install"]);
    방.성공(&["touch", "target"]);
    방.성공(&["narrative"]);
    let 화면 = 방.성공(&["uninstall"]);
    let s2 = 워킹트리(&방.repo);

    // 정본과 남의 파일 — 바이트 그대로.
    for (k, v) in &s0 {
        if 파생물_경로인가(k) || k == ".palimpsest/intent.redb" {
            continue;
        }
        assert_eq!(s2.get(k), Some(v), "{k} 가 설치 전과 다르다");
    }
    // 갈린 것은 파생물뿐이고, 그것은 지워졌다.
    for k in 갈린_경로(&s0, &s2) {
        assert!(파생물_경로인가(&k) || k == ".palimpsest/intent.redb", "파생물이 아닌 {k} 가 갈렸다");
        assert!(!s2.contains_key(&k), "파생물 {k} 가 남았다");
    }
    for rel in [".palimpsest/index.redb", ".palimpsest/cache/", ".palimpsest/radius-base-0123456789ab.redb", ".palimpsest/narrative-pending.json"] {
        assert!(
            화면.lines().any(|l| l.contains("지웠다") && l.contains(rel)),
            "지운 파생물 {rel} 를 출력하지 않았다:\n{화면}"
        );
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// B3
// ─────────────────────────────────────────────────────────────────────────────

/// 결박이나 거부를 남긴 방 — `.palimpsest/` 는 설치 전부터 있다(커밋된 저장소 선언).
fn b3(tag: &str, 승인: bool) {
    let 방 = 방::문서(tag);
    방.쓴다(".gitignore", "node_modules/\n");
    방.쓴다(".palimpsest/manifest.toml", "[[repo]]\nid = \"repo\"\npath = \".\"\n");
    방.커밋("선언");
    let s0 = 워킹트리(&방.repo);
    let st0 = 방.git_출력(&["status", "--porcelain", "-uall"]);
    let gi0 = std::fs::read(방.자리(".gitignore")).expect(".gitignore");

    방.성공(&["install"]);
    let (id, picks) = 방.후보();
    if 승인 {
        방.성공(&["narrative", "--approve", &id, "--pick", &picks[0]]);
    } else {
        방.성공(&["narrative", "--refuse", &id, "--pick", &picks[0], "--reason", "시험"]);
    }
    let 직전 = 방.의도_수();
    assert_eq!((직전.0, 직전.1), if 승인 { (1, 0) } else { (0, 1) }, "전제 — 결박·거부가 생겨야 한다");

    let 화면 = 방.성공(&["uninstall"]);

    // 첫 단언이 「다시 열어 읽은 수」다 — 파일이 없으면 저장소는 비어 열리고(결박 0 · 거부 0) 여기서 빨개진다.
    let 뒤 = 방.의도_수();
    assert_eq!((뒤.0, 뒤.1), (직전.0, 직전.1), "다시 열어 읽은 결박·거부 수가 uninstall 직전과 다르다");
    assert_eq!(
        갈린_경로(&s0, &워킹트리(&방.repo)),
        vec![".palimpsest/.gitignore".to_owned(), ".palimpsest/intent.redb".to_owned()],
        "설치 전과 정확히 두 경로만 달라야 한다:\n{화면}"
    );
    assert_eq!(std::fs::read(방.자리(".gitignore")).expect(".gitignore"), gi0, "사용자 .gitignore 가 설치 전 바이트가 아니다");
    assert_eq!(방.git_출력(&["status", "--porcelain", "-uall"]), st0, "git status 가 설치 전과 다르다");
    for 말 in [format!("결박 {}", 직전.0), format!("거부 {}", 직전.1), "pal intent export".to_owned(), "--purge".to_owned()] {
        assert!(화면.contains(&말), "화면에 「{말}」이 없다:\n{화면}");
    }
}

#[test]
fn b3_결박을_승인하고_내보내지_않은_방은_intent_redb_를_남기고_가린다() {
    b3("b3-승인", true);
}

#[test]
fn b3_거부만_한_방도_intent_redb_를_남기고_가린다() {
    b3("b3-거부", false);
}

// ─────────────────────────────────────────────────────────────────────────────
// B4 — `--purge`
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn b4_1_설치_뒤에_생긴_추적_안_된_정본은_purge_가_걷고_전부_출력한다() {
    let 방 = 방::문서("b4-1");
    let s0 = 워킹트리(&방.repo);

    방.성공(&["install"]);
    let (id, picks) = 방.후보();
    방.성공(&["narrative", "--approve", &id, "--pick", &picks[0]]);
    내보낸다(&방);
    방.쓴다(".palimpsest/rounds/r1/intent.md", "# 회차\n");
    방.쓴다(".palimpsest/rounds/r1/notes/a.md", "메모\n");
    let 정본 = [
        ".palimpsest/intent/bindings.jsonl",
        ".palimpsest/rounds/r1/intent.md",
        ".palimpsest/rounds/r1/notes/a.md",
        ".palimpsest/intent.redb",
    ];
    for rel in 정본 {
        assert!(방.자리(rel).exists(), "전제 — {rel} 가 생겨야 한다");
    }
    assert_eq!(방.의도_수().0, 1, "전제 — intent.redb 가 정본이어야 한다");

    let 화면 = 방.성공(&["uninstall", "--purge"]);
    let 갈림 = 갈린_경로(&s0, &워킹트리(&방.repo));
    assert!(갈림.is_empty(), "purge 뒤 설치 전과 갈린 경로가 있다: {갈림:#?}\n{화면}");
    for rel in 정본 {
        assert!(화면.lines().any(|l| l.contains("지웠다") && l.contains(rel)), "지운 {rel} 를 출력하지 않았다:\n{화면}");
    }
}

/// 설치 전부터 추적하던 정본을 둔 방.
fn 추적_정본_방(tag: &str) -> 방 {
    let 방 = 방::문서(tag);
    방.쓴다(".palimpsest/rounds/old/intent.md", "# 옛 회차\n");
    내보낸다(&방);
    방.커밋("정본");
    방
}

#[test]
fn b4_2_설치_전부터_추적하던_정본은_purge_도_남기고_말한다() {
    let 방 = 추적_정본_방("b4-2");
    let s0 = 워킹트리(&방.repo);

    방.성공(&["install"]);
    방.성공(&["narrative"]);
    let 화면 = 방.성공(&["uninstall", "--purge"]);

    let s2 = 워킹트리(&방.repo);
    for rel in [".palimpsest/rounds/old/intent.md", ".palimpsest/intent/bindings.jsonl"] {
        assert!(s0.contains_key(rel), "전제 — {rel}");
        assert_eq!(s2.get(rel), s0.get(rel), "추적 중인 {rel} 를 purge 가 건드렸다");
        assert!(
            화면.lines().any(|l| l.contains(rel) && l.contains("추적 중이라 남겼다")),
            "{rel} 를 「추적 중이라 남겼다」로 말하지 않았다:\n{화면}"
        );
    }
    let 갈림 = 갈린_경로(&s0, &s2);
    assert!(갈림.is_empty(), "purge 뒤 설치 전과 갈린 경로가 있다: {갈림:#?}");
}

#[test]
fn b4_3_설치_뒤_봉인하고_커밋한_회차_기록은_purge_도_남긴다() {
    let 방 = 방::문서("b4-3");
    방.성공(&["install"]);
    let d = ".palimpsest/rounds/r1";
    방.쓴다(&format!("{d}/intent.md"), "# 회차\n\n## 완수 조건\n\n- [ ] A1 condition A1\n");
    방.쓴다(
        &format!("{d}/verification.log"),
        "{\"kind\":\"schema\",\"version\":3,\"round\":\"r1\"}\n\
         {\"kind\":\"oracle\",\"id\":\"A1\",\"mode\":\"command\",\"check\":\"echo ROUND_OK\",\"expect\":{\"literal\":\"ROUND_OK\"},\"cwd\":\".\"}\n",
    );
    방.쓴다(
        &format!("{d}/report.md"),
        "# report\n\n## 남지 않은 것\n없음.\n\n## 다음 회차가 받는 것\n없음.\n\n## 범위 밖\n없음.\n\n## 원리상 못 잰 것\n없음.\n\n## 능력 부재\n없음.\n",
    );
    방.쓴다(&format!("{d}/findings.jsonl"), "{\"schema_version\":3,\"종류\":\"레코드\",\"회차\":\"r1\"}\n");
    git(&방.repo, &["add", d]);
    git(&방.repo, &["-c", "user.email=t@e", "-c", "user.name=t", "commit", "-qm", "회차"]);
    방.성공(&["round", "approve", "--round", "r1", "--id", "A1", "--json"]);
    방.성공(&["round", "verify", "--round", "r1", "--id", "A1", "--json"]);
    let 봉인 = 방.성공(&["round", "verify", "--round", "r1", "--all", "--json"]);
    assert!(봉인.contains("\"completion\":\"complete\""), "전제 — 회차가 종료 봉인까지 가야 한다: {봉인}");
    git(&방.repo, &["add", d]);
    git(&방.repo, &["-c", "user.email=t@e", "-c", "user.name=t", "commit", "-qm", "봉인"]);
    let 기록 = 아래(&워킹트리(&방.repo), &[d]);
    assert!(기록.len() >= 4, "전제 — 회차 기록이 워킹트리에 있어야 한다: {기록:?}");

    let 화면 = 방.성공(&["uninstall", "--purge"]);
    assert_eq!(아래(&워킹트리(&방.repo), &[d]), 기록, "커밋한 회차 기록을 purge 가 건드렸다:\n{화면}");
    assert!(
        화면.lines().any(|l| l.contains(d) && l.contains("추적 중이라 남겼다")),
        "회차 기록을 「추적 중이라 남겼다」로 말하지 않았다:\n{화면}"
    );
}

#[test]
fn b4_4_추적_중인_정본에_줄이_더해지면_purge_가_남기고_고쳐졌다고_말한다() {
    let 방 = 추적_정본_방("b4-4");
    방.성공(&["install"]);
    let rel = ".palimpsest/rounds/old/intent.md";
    방.덧붙인다(rel, "설치 뒤에 더한 줄\n");
    let 직전 = std::fs::read(방.자리(rel)).expect("읽기");

    let 화면 = 방.성공(&["uninstall", "--purge"]);
    assert_eq!(std::fs::read(방.자리(rel)).ok(), Some(직전), "추적 중이고 고쳐진 정본을 purge 가 건드렸다");
    assert!(
        화면.lines().any(|l| l.contains(rel) && l.contains("추적 중 · 고쳐짐")),
        "「추적 중 · 고쳐짐」을 말하지 않았다:\n{화면}"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// B5
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn b5_1_사용_중_git_status_에_파생물이_안_뜬다() {
    let 방 = 방::문서("b5-1");
    방.성공(&["install"]);
    방.성공(&["touch", "target"]);
    방.성공(&["narrative"]);
    방.쓴다(".palimpsest/radius-base-0123456789ab.redb", "중단된 반경 계산이 남긴 것\n");
    파생물이_생겼다(&방);

    let 상태 = 방.git_출력(&["status", "--porcelain", "-uall"]);
    let 뜬: Vec<&str> = 상태
        .lines()
        .filter(|l| {
            let p = l.get(3..).unwrap_or("");
            파생물_경로인가(p) || p == ".palimpsest/intent.redb"
        })
        .collect();
    assert!(뜬.is_empty(), "사용 중 파생물이 git status 에 뜬다: {뜬:#?}");
}

/// 착수 커밋(`acd7e82`) 바이너리의 `.gitignore` 블록 — **그 빌드가 넣는 바이트 그대로**다(고정).
const 착수_블록: &str = "# pal:begin — palimpsest. `pal uninstall` 이 이 블록을 걷어낸다\n\
/.palimpsest/cache/\n/.palimpsest/index.redb\n/.palimpsest/intent.redb\n# pal:end\n";

/// 착수 바이너리로 설치한 방의 모양 — 버전과 `.gitignore` 블록(파일 · 매니페스트 `inserted`)을 그 빌드의 것으로 되돌린다.
///
/// 실제 착수 바이너리로 같은 왕복을 돌린 산출은 `oracle/T2-b5-start-binary.txt` 에 있다.
fn 착수_바이너리의_방으로(방: &방) {
    let path = 방.자리(".claude/pal/manifest.json");
    let mut m: serde_json::Value = serde_json::from_slice(&std::fs::read(&path).expect("매니페스트")).expect("JSON");
    m["pal_version"] = serde_json::json!("0.1.1+acd7e820fca3");
    let blocks = m["blocks"].as_array_mut().expect("blocks");
    let ig = blocks.iter_mut().find(|b| b["path"] == ".gitignore").expect("`.gitignore` 블록 기록");
    let 지금 = ig["inserted"].as_str().expect("inserted").to_owned();
    ig["inserted"] = serde_json::json!(착수_블록);
    std::fs::write(&path, serde_json::to_string_pretty(&m).expect("직렬화")).expect("쓰기");
    let 파일 = std::fs::read_to_string(방.자리(".gitignore")).expect(".gitignore");
    assert!(파일.contains(&지금), "전제 — 설치가 넣은 블록이 파일에 있어야 한다");
    std::fs::write(방.자리(".gitignore"), 파일.replace(&지금, 착수_블록)).expect("쓰기");
}

#[test]
fn b5_2_착수_바이너리로_설치한_방을_update_하고_uninstall_하면_설치_전과_같다() {
    let 방 = 방::문서("b5-2");
    방.쓴다(".gitignore", "node_modules/\n");
    방.커밋("무시");
    let s0 = 워킹트리(&방.repo);

    방.성공(&["install"]);
    착수_바이너리의_방으로(&방);
    방.성공(&["update"]);
    let 덮임 = Command::new("git")
        .args(["check-ignore", "-q", "--no-index", "--", ".palimpsest/narrative-pending.json"])
        .current_dir(&방.repo)
        .status()
        .expect("git");
    assert!(덮임.success(), "update 가 `.gitignore` 블록을 새 목록으로 갈아 끼우지 않았다");

    방.성공(&["uninstall"]);
    let 갈림 = 갈린_경로(&s0, &워킹트리(&방.repo));
    assert!(갈림.is_empty(), "update → uninstall 뒤 설치 전과 갈린 경로가 있다: {갈림:#?}");
}

// ─────────────────────────────────────────────────────────────────────────────
// B6
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn b6_사용자가_고친_pal_파일은_지우고_경고한다() {
    let 방 = 방::새("b6");
    방.쓴다("README.md", "hello\n");
    방.커밋("첫");
    let s0 = 워킹트리(&방.repo);

    방.성공(&["install"]);
    let 고친 = [".claude/agents/pal-orchestrator.md", ".claude/commands/pal/touch.md", ".claude/skills/pal-round/SKILL.md"];
    for rel in 고친 {
        assert!(방.자리(rel).is_file(), "전제 — install 이 {rel} 를 놓아야 한다");
        방.덧붙인다(rel, "\n사용자가 더한 줄\n");
    }

    let 화면 = 방.성공(&["uninstall"]);
    let 갈림 = 갈린_경로(&s0, &워킹트리(&방.repo));
    assert!(갈림.is_empty(), "고친 pal 파일을 남겼다: {갈림:#?}");
    let 경고 = 화면
        .split_once(&format!("사람이 고친 파일 {}개를 지웠다", 고친.len()))
        .unwrap_or_else(|| panic!("경고 블록이 없다:\n{화면}"))
        .1;
    for rel in 고친 {
        assert!(경고.contains(rel), "경고 블록에 {rel} 가 없다:\n{화면}");
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// B7
// ─────────────────────────────────────────────────────────────────────────────

fn b7(tag: &str, 추적: bool) {
    let 방 = 방::문서(tag);
    방.성공(&["install"]);
    내보낸다(&방);
    방.쓴다(".palimpsest/rounds/r1/intent.md", "# 회차\n");
    방.쓴다(".palimpsest/rounds/r1/notes/a.md", "메모\n");
    if 추적 {
        git(&방.repo, &["add", ".palimpsest/rounds", ".palimpsest/intent"]);
        git(&방.repo, &["-c", "user.email=t@e", "-c", "user.name=t", "commit", "-qm", "정본"]);
    }
    let 정본 = [".palimpsest/rounds/", ".palimpsest/intent/"];
    let 직전 = 아래(&워킹트리(&방.repo), &정본);
    assert!(직전.len() >= 3, "전제 — 정본이 생겨야 한다: {직전:?}");

    let 화면 = 방.성공(&["uninstall"]);
    assert_eq!(아래(&워킹트리(&방.repo), &정본), 직전, "기본 uninstall 이 정본을 건드렸다:\n{화면}");
}

#[test]
fn b7_기본_uninstall_은_설치_뒤에_생긴_추적_중인_정본을_남긴다() {
    b7("b7-추적", true);
}

#[test]
fn b7_기본_uninstall_은_설치_뒤에_생긴_추적_안_된_정본을_남긴다() {
    b7("b7-안추적", false);
}
