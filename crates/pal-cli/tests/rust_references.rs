//! **Rust 참조 엣지가 사슬 전체를 지난다** — 추출 → 1층 캐시 → `stitch_of` → `pal touch`.
//!
//! 회차 `2026-09-07-rust-scope-references` 의 `C6`. 합격선 정본은 그 회차의 잠긴 의도이고
//! 판정은 `docs/gates/rust-scope-references.md`.
//!
//! # 왜 이 파일이 필요한가
//!
//! `crates/pal-cli/tests/` 의 픽스처가 **전부 `.ts`·`.kt` 였다.** 그래서 Rust 추출기가
//! 산출하는 것이 CI 의 어느 통합 시험도 안 지났다 — 단위 시험은 `extract_detailed` 한 마디만
//! 재고, 캐시 왕복·스티칭·화면은 안 잰다. 사전부검 R1 이 그 구멍을 잡았다.
//!
//! # ⚠ 하한을 박는다 — 엣지가 0 이면 아래가 전부 공짜로 통과한다

mod common;

use common::{git, pal};
use pal_core::{RefCounts, RepoPath, Slot};
use pal_store::Projection;
use std::path::PathBuf;

/// 갈래가 여럿 나오는 Rust 저장소.
///
/// | 갈래 | 이 소스의 어디 |
/// |---|---|
/// | 선언 자리 | `fn helper` 의 `helper` |
/// | 엣지 | `caller` 안의 `helper(x)` · `Cfg` 타입 참조 |
/// | 지역 | `caller` 의 파라미터 `x` |
/// | 파일 밖 | `String` |
/// | 모호 | `cfg` 쌍둥이 `spawn` 을 부르는 자리 |
///
/// **`impl` 둘에 동명 `new` 를 넣는다** — `impl` 이 스코프를 안 열면 서로를 가리켜
/// 가짜 엣지가 서고, 이 시험이 그것을 잡는다.
fn 저장소(tag: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("pal-rust-refs-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("임시 저장소");

    std::fs::write(
        root.join("lib.rs"),
        "pub struct Cfg;\n\
         pub struct Other;\n\
         impl Cfg { pub fn new() -> Cfg { Cfg } }\n\
         impl Other { pub fn new() -> Other { Other } }\n\
         pub fn helper(n: u32) -> u32 { n + 1 }\n\
         pub fn caller(x: u32) -> u32 { let c: Cfg = Cfg::new(); let _ = c; helper(x) }\n\
         pub fn wide() -> String { String::new() }\n\
         #[cfg(unix)] fn spawn() {}\n\
         #[cfg(windows)] fn spawn() {}\n\
         pub fn twin() { spawn(); }\n",
    )
    .expect("lib.rs");
    // **TypeScript 도 함께 둔다** — 한 언어만 있으면 「두 언어가 같은 뼈대를 탄다」가
    // 안 걸린다.
    std::fs::write(root.join("a.ts"), "export function ts() { return 1 }\n").expect("a.ts");

    git(&root, &["init", "-q", "."]);
    git(&root, &["add", "-A"]);
    git(&root, &["-c", "user.email=t@example.com", "-c", "user.name=t", "commit", "-qm", "첫 커밋"]);
    root
}

fn 투영(repo: &std::path::Path) -> Projection {
    Projection::open(&repo.join(".palimpsest/index.redb")).expect("2층")
}

fn 참조_수(p: &Projection, path: &str) -> Slot<RefCounts> {
    p.file(&RepoPath::new(path)).expect("파일 노드 읽기").expect("파일 노드가 없다").refs
}

#[test]
fn rust_참조가_2층까지_도착하고_화면이_답한다() {
    let repo = 저장소("chain");
    let 답 = pal(&repo, &["touch", "helper", "--json"]);
    let p = 투영(&repo);

    // ── ① 파일 노드에 참조 수가 값으로 섰다 ─────────────────────────────
    let Slot::Built(c) = 참조_수(&p, "lib.rs") else {
        panic!("Rust 파일의 참조 수가 「안 만듦」이다 — 스코프 체인이 2층에 안 실렸다");
    };
    assert!(c.declarations > 0, "선언 자리를 하나도 안 셌다");
    assert!(c.edges > 0, "엣지가 0 이다 — 아래 전부가 공짜로 통과한다");
    assert!(c.locals > 0, "지역 변수 참조를 하나도 안 셌다");
    assert!(c.unresolved > 0, "파일 밖 참조를 하나도 안 셌다");
    // **`cfg` 쌍둥이가 모호로 적힌다** — 하나를 골랐으면 이 값이 0 이고 엣지가 하나 는다.
    assert!(c.ambiguous > 0, "cfg 쌍둥이를 조용히 하나로 골랐다");
    assert_eq!(
        c.total(),
        c.declarations
            + c.edges
            + c.locals
            + c.top_level
            + c.unresolved
            + c.before_declaration
            + c.ambiguous,
        "갈래 하나가 샜다"
    );

    // ── ② 출발점과 도착점이 옳다 ────────────────────────────────────────
    let helper = p.resolve_name("helper").expect("이름").into_iter().next().expect("helper");
    let caller = p.resolve_name("caller").expect("이름").into_iter().next().expect("caller");
    let 부르는 = p.callers(helper.id).expect("역방향");
    assert!(부르는.contains(&caller.id), "`caller` 가 `helper` 를 부른 것이 안 실렸다");
    assert!(!부르는.contains(&helper.id), "선언 자리가 자기 엣지로 남았다");

    // ── ③ `impl` 둘의 동명 `new` 가 서로를 안 가리킨다 ──────────────────
    for n in p.resolve_name("new").expect("이름") {
        let 이것을_부르는 = p.callers(n.id).expect("역방향");
        assert!(
            !이것을_부르는.iter().any(|id| p
                .resolve_name("new")
                .expect("이름")
                .iter()
                .any(|m| m.id == *id)),
            "다른 `impl` 의 동명 메서드가 서로를 가리켰다 — impl 이 스코프를 안 열었다"
        );
    }

    // ── ④ 그리고 그것이 화면의 답에 실린다 ──────────────────────────────
    let v: serde_json::Value = serde_json::from_str(&답).expect("응답 묶음 JSON");
    let facts = &v["answer"]["facts"]["present"];
    assert_eq!(facts["callers"].as_u64().expect("callers"), 부르는.len() as u64);
    assert!(facts["callers"].as_u64().expect("callers") > 0, "호출자 칸이 0 이다");

    // ── ⑤ TypeScript 는 그대로다 ────────────────────────────────────────
    assert!(matches!(참조_수(&p, "a.ts"), Slot::Built(_)), "TypeScript 쪽이 안 만듦이 됐다");

    let _ = std::fs::remove_dir_all(&repo);
}

#[test]
fn rust_의_내보내기가_최상위_pub_만_담는다() {
    // `A13` 의 통합 쪽 — `stitch_of` 가 최상위만 EXPORTS 로 옮기므로 그 모집단과
    // `ExportSet` 의 모집단이 같아야 한다.
    let repo = 저장소("exports");
    pal(&repo, &["touch", "helper", "--json"]);
    let p = 투영(&repo);

    assert!(
        p.export(&RepoPath::new("lib.rs"), "helper").expect("내보내기").is_some(),
        "최상위 `pub fn helper` 가 EXPORTS 에 없다"
    );
    assert!(
        p.export(&RepoPath::new("lib.rs"), "spawn").expect("내보내기").is_none(),
        "`pub` 이 아닌 `spawn` 이 EXPORTS 에 섰다"
    );
    // 동명 `new` 가 둘이라 **담지 않는다** — 하나를 고르면 조용한 오답이다.
    assert!(
        p.export(&RepoPath::new("lib.rs"), "new").expect("내보내기").is_none(),
        "동명 `new` 둘 중 하나를 골랐다"
    );

    let _ = std::fs::remove_dir_all(&repo);
}
