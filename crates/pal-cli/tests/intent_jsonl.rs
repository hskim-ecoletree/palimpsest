//! **의도 저장소의 JSONL 왕복** — 재구축 불가한 것이 나갔다 그대로 돌아오는가 (`[f05.4]`).
//!
//! F05 §2 의 넷째: *"2층은 캐시라서 자체 구현의 최대 비용이 0 이다. **의도 저장소는 이
//! 면제를 받지 못한다** — 거기만 스키마 버전과 JSONL 내보내기를 진다."*
//! §6 의 표: *"재구축 불가. JSONL 내보내기에서 복구. **그래서 내보내기가 상시 유지된다.**"*
//!
//! **상시 유지되지 않는 내보내기는 없는 것과 같다.** 그래서 판정이 「기능이 있다」가
//! 아니라 **「왕복이 항등이다」** 이고, CI 가 그것을 상시 돌린다.

mod common;

use common::{pal, 저장소};

fn 결박_수(repo: &std::path::Path) -> usize {
    let intent = repo.join(".palimpsest/intent.redb");
    let s = pal_intent::IntentStore::open_read_only(&intent).expect("의도 저장소");
    s.count().expect("결박 수")
}

#[test]
fn 내보내고_지우고_읽으면_값이_그대로다() {
    let repo = 저장소("jsonl");
    let intent = repo.join(".palimpsest/intent.redb");
    let 파일 = repo.join("intent.jsonl");

    pal(&repo, &["ledger", ".", "--json"]);
    pal(&repo, &["bind", "알파", "--note", "첫째 결박"]);
    pal(&repo, &["bind", "Gamma", "--note", "둘째 결박 — Kotlin 쪽이다"]);
    // **하한이다** — 결박이 0 건이면 왕복이 공짜로 항등이다.
    assert_eq!(결박_수(&repo), 2, "결박이 둘이 아니다");

    let 만짐_전 = pal(&repo, &["touch", "알파", "--json"]);
    pal(&repo, &["intent", "export", "--out", 파일.to_str().expect("경로")]);
    let 내보낸 = std::fs::read_to_string(&파일).expect("JSONL");
    assert!(내보낸.lines().count() >= 3, "머리 + 결박 둘이 아니다: {내보낸}");
    assert!(내보낸.lines().next().expect("머리").contains("schema_version"), "머리 줄이 없다");

    // **의도 저장소를 통째로 지운다.** 이 층에는 재구축이 없다 — 여기서 되살아나지
    // 않으면 사람이 지불한 노동이 사라진 것이다.
    std::fs::remove_file(&intent).expect("삭제");
    assert_eq!(결박_수(&repo), 0, "지웠는데 남아 있다");

    pal(&repo, &["intent", "import", 파일.to_str().expect("경로")]);
    assert_eq!(결박_수(&repo), 2, "읽었는데 둘이 아니다");

    let 만짐_후 = pal(&repo, &["touch", "알파", "--json"]);
    assert!(만짐_후.contains("첫째 결박"), "조각이 안 돌아왔다: {만짐_후}");
    assert_eq!(만짐_후, 만짐_전, "왕복 뒤 답이 달라졌다 — 항등이 아니다");

    let _ = std::fs::remove_dir_all(&repo);
}

#[test]
fn 읽기는_더하기이지_바꿔치기가_아니다() {
    // ★ `[f05.4.pass]` ② — **지우는 API 가 없다는 대응이 이 명령에서도 참이어야 한다.**
    let repo = 저장소("jsonl-add");
    let 파일 = repo.join("only-a.jsonl");

    pal(&repo, &["ledger", ".", "--json"]);
    pal(&repo, &["bind", "알파", "--note", "A"]);
    // A 만 담긴 파일을 뜬다.
    pal(&repo, &["intent", "export", "--out", 파일.to_str().expect("경로")]);
    let a만 = std::fs::read_to_string(&파일).expect("JSONL");
    assert_eq!(a만.lines().count(), 2, "머리 + 결박 하나가 아니다");

    // 그 뒤에 B 를 건다. 저장소에는 둘, 파일에는 하나.
    pal(&repo, &["bind", "Gamma", "--note", "B"]);
    assert_eq!(결박_수(&repo), 2);

    // A 만 담긴 파일을 읽는다 — **B 가 남아야 한다.**
    let 회계 = pal(&repo, &["intent", "import", 파일.to_str().expect("경로"), "--json"]);
    let v: serde_json::Value = serde_json::from_str(&회계).expect("회계 JSON");
    assert_eq!(v["bindings"], 1, "파일에 있던 결박이 하나가 아니다");
    assert_eq!(v["already_present"], 1, "이미 있던 것을 안 셌다");
    assert_eq!(결박_수(&repo), 2, "읽기가 저장소를 파일의 모습으로 만들었다 — **R-21**");
    assert!(pal(&repo, &["touch", "Gamma", "--json"]).contains("\"B\""), "B 가 사라졌다");

    // ★ **내보내지 않은 것은 안 돌아온다.** 이것이 없으면 *"전부 복원됐다"* 가
    // 「원래 있던 것」인지 「읽어 온 것」인지 구별되지 않는다.
    assert!(!a만.contains("\"B\""), "A 만 담은 파일에 B 가 있다 — 이 시험은 아무것도 안 잰다");

    let _ = std::fs::remove_dir_all(&repo);
}

#[test]
fn 깨진_의도_저장소를_조용히_빈_채로_열지_않는다() {
    // `[f05.4.pass]` ③ — F05 §6: *"사용자에게 유실 범위를 명시 — **조용히 빈 채로 열지
    // 않는다**"*.
    let repo = 저장소("jsonl-corrupt");
    let intent = repo.join(".palimpsest/intent.redb");
    pal(&repo, &["ledger", ".", "--json"]);
    pal(&repo, &["bind", "알파", "--note", "A"]);
    assert_eq!(결박_수(&repo), 1);

    // 머리를 망가뜨린다. **지우는 것이 아니다** — 크기가 그대로임을 함께 확인한다.
    let 원래 = std::fs::read(&intent).expect("읽기");
    let mut 망가진 = 원래.clone();
    for b in 망가진.iter_mut().take(64) {
        *b ^= 0xFF;
    }
    std::fs::write(&intent, &망가진).expect("쓰기");
    assert_eq!(std::fs::metadata(&intent).expect("크기").len(), 원래.len() as u64);

    let out = std::process::Command::new(common::PAL)
        .args(["touch", "알파", "--json"])
        .current_dir(&repo)
        .output()
        .expect("pal");
    assert!(!out.status.success(), "깨진 의도 저장소인데 성공했다");
    let 말 = String::from_utf8_lossy(&out.stderr);
    assert!(
        말.contains("intent") || 말.contains("의도"),
        "무엇이 깨졌는지 안 말했다: {말}"
    );
    assert!(말.contains("import"), "복구 경로를 안 말했다: {말}");

    let _ = std::fs::remove_dir_all(&repo);
}

#[test]
fn 읽기만_하는_명령이_의도_저장소를_안_건드린다() {
    // ★ `[f05.4.pass]` ④ — **F04 가 여기로 넘긴 것이다.**
    //
    // F04 는 `Database::create` 가 열기만 해도 쓴다는 것을 재고(110 바이트),
    // 재구축 등가성의 ③ 을 **바이트가 아니라 값으로** 재야 했다.
    let repo = 저장소("jsonl-readonly");
    let intent = repo.join(".palimpsest/intent.redb");
    pal(&repo, &["ledger", ".", "--json"]);
    // **결박을 넣은 뒤에 잰다** — 빈 저장소는 안 자라서 공짜로 통과한다.
    pal(&repo, &["bind", "알파", "--note", "A"]);

    pal(&repo, &["touch", "알파", "--json"]);
    let 전 = std::fs::read(&intent).expect("읽기");
    pal(&repo, &["touch", "알파", "--json"]);
    let 후 = std::fs::read(&intent).expect("읽기");

    assert_eq!(전.len(), 후.len(), "길이가 변했다");
    let 다른_바이트 = 전.iter().zip(&후).filter(|(a, b)| a != b).count();
    assert_eq!(다른_바이트, 0, "읽기만 했는데 {다른_바이트} 바이트가 변했다 — 열기가 쓴다");

    // 내보내기도 읽기다.
    pal(&repo, &["intent", "export"]);
    assert_eq!(std::fs::read(&intent).expect("읽기"), 후, "내보내기가 저장소를 건드렸다");

    let _ = std::fs::remove_dir_all(&repo);
}
