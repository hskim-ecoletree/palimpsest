//! **Rust 선언 밖의 속성과 수신자가 변하면 결박이 `stale` 이다** — #77 · 회차
//! `2026-09-14-first-release` 조건 `A2` · `A2-a` ⑴.
//!
//! # 무엇이 틀렸었나
//!
//! `body_digest` 는 선언 마디의 정규형이다. Rust 에서 속성(`#[…]`)은 선언 마디 **안**이
//! 아니라 **앞 형제**이고, 메서드 수신자의 `&` 는 정규화가 「선행 구분자」로 벗겼다.
//! 그래서 `#[derive(Debug)]` → `#[derive(Debug, Clone)]` 도, `&self` → `self` 도 요약이
//! 같았고 **그 변경에 결박이 `fresh` 로 남았다** — 사실이 아닌 것을 사실로 냈다.
//!
//! # 갈래마다 따로 잰다
//!
//! 다섯을 한 시험에서 연달아 바꾸면 **첫 갈래 하나만 잡혀도 나머지가 공짜로 `stale`** 이다.
//! 그래서 갈래마다 새 저장소를 세우고, 되돌리면 다시 `fresh` 인 것까지 본다 — 안 보면
//! *"무엇을 바꾸든 `stale`"* 인 감지기가 통과한다.
//!
//! # 음성 대조 — 속성 안의 주석·공백은 안 켠다 (ADR-0007)
//!
//! 속성을 **원문 바이트**로 요약하면 위 다섯이 다 잡히고 포매터 한 번에 결박이 무더기로
//! 켜진다(R-07). 그 반대 방향을 같은 픽스처가 잰다.

mod common;

use common::{git, pal};
use std::path::{Path, PathBuf};

const 원본: &str = "#[derive(Debug)]\n\
#[cfg(feature = \"a\")]\n\
#[serde(rename = \"one\")]\n\
#[must_use]\n\
pub struct Sample {\n\
    pub n: u8,\n\
}\n\
\n\
impl Sample {\n\
    pub fn read(&self) -> u8 {\n\
        self.n\n\
    }\n\
}\n";

const 구조체_조각: &str = "구조체의 계약";
const 메서드_조각: &str = "메서드의 계약";

fn 커밋(repo: &Path, msg: &str) {
    git(repo, &["add", "-A"]);
    git(repo, &["-c", "user.email=t@example.com", "-c", "user.name=t", "commit", "-qm", msg]);
}

/// 픽스처 저장소 — 결박 둘(`Sample` · `read`)이 선 채로.
fn 저장소(tag: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("pal-a2-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("임시 저장소");
    std::fs::write(root.join("lib.rs"), 원본).expect("lib.rs");
    git(&root, &["init", "-q", "."]);
    커밋(&root, "첫");
    pal(&root, &["bind", "Sample", "--note", 구조체_조각]);
    pal(&root, &["bind", "read", "--note", 메서드_조각]);
    root
}

fn 바꿔_커밋(repo: &Path, from: &str, to: &str) {
    let path = repo.join("lib.rs");
    let 전 = std::fs::read_to_string(&path).expect("읽기");
    assert!(전.contains(from), "픽스처에 `{from}` 이 없다 — 이 갈래가 아무것도 안 바꾼다");
    let 후 = 전.replacen(from, to, 1);
    // **하한** — 변형이 파일을 안 바꿨으면 뒤의 단언이 공짜다.
    assert_ne!(전, 후, "변형이 파일을 안 바꿨다 — 이 시험이 아무것도 안 잰다");
    std::fs::write(&path, 후).expect("쓰기");
    커밋(repo, &format!("{from} → {to}"));
}

/// 조각으로 고른 결박 하나의 응답 행.
fn 결박(repo: &Path, 조각: &str) -> serde_json::Value {
    let v: serde_json::Value =
        serde_json::from_str(&pal(repo, &["query", "binding.status", "--json"])).expect("응답 묶음 JSON");
    let list = v["answer"]["bindings"].as_array().expect("bindings 가 배열이 아니다");
    assert_eq!(list.len(), 2, "결박이 둘이 아니다 — 픽스처가 안 섰다: {v}");
    list.iter()
        .find(|b| b["note"].as_str() == Some(조각))
        .unwrap_or_else(|| panic!("조각 `{조각}` 의 결박이 없다: {v}"))
        .clone()
}

fn 신선도(repo: &Path, 조각: &str) -> String {
    결박(repo, 조각)["status"]["code"]["freshness"].as_str().expect("freshness").to_owned()
}

/// 한 갈래 — 착수 `fresh` → 바꾸면 `stale` → 되돌리면 `fresh`.
fn 갈래(tag: &str, 조각: &str, from: &str, to: &str) {
    let repo = 저장소(tag);
    assert_eq!(신선도(&repo, 조각), "fresh", "막 걸었는데 fresh 가 아니다");

    바꿔_커밋(&repo, from, to);
    assert_eq!(
        신선도(&repo, 조각),
        "stale",
        "★ `{from}` → `{to}` 를 커밋했는데 결박이 stale 이 아니다 — 사실이 아닌 것을 사실로 낸다"
    );

    // **되돌리면 다시 fresh** — 안 보면 「무엇이든 stale」 인 감지기가 통과한다.
    바꿔_커밋(&repo, to, from);
    assert_eq!(신선도(&repo, 조각), "fresh", "되돌렸는데 fresh 가 아니다 — 기준값이 흔들린다");

    let _ = std::fs::remove_dir_all(&repo);
}

#[test]
fn a2_1_derive_인자가_바뀌면_stale() {
    갈래("derive", 구조체_조각, "#[derive(Debug)]", "#[derive(Debug, Clone)]");
}

#[test]
fn a2_2_cfg_인자가_바뀌면_stale() {
    갈래("cfg", 구조체_조각, "#[cfg(feature = \"a\")]", "#[cfg(feature = \"b\")]");
}

#[test]
fn a2_3_serde_rename_값이_바뀌면_stale() {
    갈래("serde", 구조체_조각, "#[serde(rename = \"one\")]", "#[serde(rename = \"two\")]");
}

#[test]
fn a2_4_must_use_유무가_바뀌면_stale() {
    갈래("must-use", 구조체_조각, "#[must_use]\npub struct", "pub struct");
}

#[test]
fn a2_5_수신자_참조_self_가_값_self_로_바뀌면_stale() {
    갈래("receiver", 메서드_조각, "pub fn read(&self)", "pub fn read(self)");
}

/// ★ **음성 대조 `A2-a` ⑴** — 속성 안의 주석·공백만 바꾸면 `fresh` 로 남는다.
#[test]
fn a2a_1_속성_안의_주석과_공백만_바꾸면_fresh() {
    let repo = 저장소("decor-format");
    바꿔_커밋(&repo, "#[derive(Debug)]", "#[derive( Debug /* 주석 */ )]");
    바꿔_커밋(&repo, "#[cfg(feature = \"a\")]", "#[cfg( feature=\"a\" )] // 뒤 주석");
    바꿔_커밋(&repo, "pub fn read(&self)", "pub fn read( & self )");
    assert_eq!(신선도(&repo, 구조체_조각), "fresh", "★ 속성 안의 주석·공백만 바꿨는데 낡음이 켜졌다 (R-07)");
    assert_eq!(신선도(&repo, 메서드_조각), "fresh", "★ 수신자의 공백만 바꿨는데 낡음이 켜졌다 (R-07)");
    let _ = std::fs::remove_dir_all(&repo);
}

/// **옛 판 결박은 속성·수신자 축을 안 보고, 그 사실을 싣는다** — 계획 2 ㉡.
///
/// # 왜 이 시험이 있나 — 반대 방향의 거짓
///
/// 착수 때 이 저장소 결박 42 중 20 이 속성 또는 `self` 수신자를 진 Rust 심볼을
/// 감시한다. 그 결박들의 기준값은 속성·수신자 없이 섰으므로, 새 축을 **없는 기준값과**
/// 대면 코드가 안 바뀌었는데 `stale` 로 뒤집힌다. 그래서 그 축은 **기준값이 있는
/// 원소에서만** 비교하고, 기준값이 없는 원소 수를 산출에 싣는다 — 조용히 안 보는 것이
/// 아니라 **안 본다고 말한다.**
///
/// 옛 판은 판 2 JSONL 로 만든다 — 판 2 에는 `decor` 가 없다.
#[test]
fn 옛_판_결박은_속성_축을_안_보고_그_사실을_싣는다() {
    let repo = 저장소("legacy");

    // 새 결박은 그 축을 본다 — 기준값 없는 원소 0.
    assert_eq!(결박(&repo, 구조체_조각)["decor_unwatched"].as_u64(), Some(0), "새 결박이 속성 축을 안 본다");

    // ── 판 2 로 되돌려 다시 들인다 ─────────────────────────────────────────
    let 내보냄 = pal(&repo, &["intent", "export"]);
    let mut 줄들 = Vec::new();
    for line in 내보냄.lines().filter(|l| !l.trim().is_empty()) {
        let mut v: serde_json::Value = serde_json::from_str(line).expect("JSONL 한 줄");
        if v["kind"] == "header" {
            v["schema_version"] = serde_json::json!(2);
        }
        if let Some(watch) = v.get_mut("watch").and_then(|w| w.as_array_mut()) {
            for w in watch {
                let 지웠나 = w.as_object_mut().expect("감시 원소").remove("decor");
                assert!(지웠나.is_some(), "내보낸 감시 원소에 `decor` 가 없다 — 이 시험이 판 2 를 못 만든다");
            }
        }
        줄들.push(serde_json::to_string(&v).expect("직렬화"));
    }
    let 판2 = repo.join("v2.jsonl");
    std::fs::write(&판2, 줄들.join("\n") + "\n").expect("판 2 파일");
    std::fs::remove_file(repo.join(".palimpsest/intent.redb")).expect("파생 저장소를 지운다");
    pal(&repo, &["intent", "import", 판2.to_str().expect("경로")]);
    // 판 2 파일은 커밋 대상이 아니다 — 지우고 속성을 바꾼다.
    std::fs::remove_file(&판2).expect("판 2 파일 지우기");

    let 옛 = 결박(&repo, 구조체_조각);
    assert_eq!(옛["status"]["code"]["freshness"], "fresh");
    assert_eq!(
        옛["decor_unwatched"].as_u64(),
        옛["watch"].as_u64(),
        "옛 판 결박의 감시 원소가 전부 「속성 축 기준값 없음」으로 안 실렸다: {옛}"
    );

    바꿔_커밋(&repo, "#[derive(Debug)]", "#[derive(Debug, Clone)]");
    let 옛 = 결박(&repo, 구조체_조각);
    assert_eq!(
        옛["status"]["code"]["freshness"], "fresh",
        "옛 판 결박이 없는 기준값과 대어져 뒤집혔다 — 반대 방향의 거짓이다: {옛}"
    );

    // 사람 화면도 그 사실을 말한다.
    let 화면 = pal(&repo, &["touch", "Sample"]);
    assert!(
        화면.contains("속성·수신자 변경은 감시 안 함"),
        "옛 판 결박인데 touch 화면이 속성·수신자를 안 본다고 말하지 않았다:\n{화면}"
    );

    let _ = std::fs::remove_dir_all(&repo);
}
