//! **파일 경계를 넘는 참조가 사슬 전체를 지난다** — 추출 → 1층 캐시 → `stitch_of` →
//! 파일 간 해소 → `pal touch`.
//!
//! 회차 `2026-09-08-cross-file-references` 의 `A1`~`A6`. 합격선 정본은 그 회차의 잠긴
//! 의도이고 판정은 `docs/gates/cross-file-references.md`.
//!
//! # ⚠ 하한을 박는다 — 파일 간 엣지가 0 이면 아래가 전부 공짜로 통과한다
//!
//! 착수 시점의 실측이 **파일 간 0 · 파일 안 4,294** 였다. 그 상태에서도 「엣지가 있다」는
//! 참이므로, 이 파일의 단언은 전부 **파일 간**을 이름으로 집어야 한다.

mod common;

use common::{git, pal};
use pal_core::RepoPath;
use pal_store::Projection;
use std::path::PathBuf;

/// 잠근 축1 의 임포트 형태 **넷**과 그 음성 대조를 한 저장소에 심는다.
///
/// | 파일 | 무엇을 심었나 |
/// |---|---|
/// | `src/lib.rs` | 크레이트 루트. `mod` 선언과 **재수출 하나**(`A5` 의 음성 대조) |
/// | `src/inside.rs` | 형제 모듈 — 균일 경로. `Root`·`Rel` 과 `impl Root { fn new }` |
/// | `src/deep/mod.rs` · `src/deep/leaf.rs` | 깊은 경로와 `super::` |
/// | `src/alias.rs` | **별칭과 중첩 목록** — `A4` |
/// | `src/twin.rs` | **동명 심볼 둘** — `A6` |
/// | `src/user.rs` | 위를 전부 쓰는 자리. 여기서 엣지가 나간다 |
fn 저장소(tag: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("pal-xfile-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join("src/deep")).expect("임시 저장소");

    let w = |p: &str, s: &str| std::fs::write(root.join(p), s).expect("쓰기");

    // 크레이트 루트. **`Hidden` 은 재수출만 지난다** — `A5` 의 음성 대조가 이것을 쓴다.
    w("src/lib.rs",
      "pub mod inside;\n\
       pub mod deep;\n\
       pub mod alias;\n\
       pub mod twin;\n\
       pub mod user;\n\
       pub mod hide;\n\
       pub use hide::Hidden;\n");

    w("src/inside.rs",
      "pub struct Root;\n\
       pub struct Rel;\n\
       impl Root { pub fn make() -> Root { Root } }\n\
       impl Rel { pub fn make() -> Rel { Rel } }\n");

    w("src/deep/mod.rs", "pub mod leaf;\n");
    // ⚠ **`use` 문의 접두를 재는 것이지 인라인 경로가 아니다.** 잠근 축1 의 넷째 칸이
    //    *"임포트 문의 경로 접두"* 이고, `use` 를 안 지난 이름은 임포트 항목에 없어
    //    이 패스에 원리상 안 들어온다.
    w("src/deep/leaf.rs",
      "use super::super::inside::Root;\n\
       pub struct Leaf;\n\
       impl Leaf { pub fn build() -> Leaf { Leaf } }\n\
       pub fn 위를_본다() { let _ = Root::make(); }\n");

    // `A4` — 별칭과 중첩 목록. 원본 이름은 `Origin` 이고 부르는 이름은 `Alias` 다.
    w("src/alias.rs", "pub struct Origin;\nimpl Origin { pub fn mk() -> Origin { Origin } }\n");

    // `A6` — **같은 파일에 동명 심볼 둘.** 후보가 유일하지 않으면 엣지가 아니다.
    w("src/twin.rs",
      "pub struct Twin;\n\
       #[cfg(unix)] pub fn 쌍둥이() {}\n\
       #[cfg(windows)] pub fn 쌍둥이() {}\n");

    // 재수출로만 닿는 것.
    w("src/hide.rs", "pub struct Hidden;\nimpl Hidden { pub fn only() -> Hidden { Hidden } }\n");

    // 쓰는 자리. **여기서 나가는 엣지가 이 시험의 대상이다.**
    w("src/user.rs",
      "use crate::inside::{Rel, Root};\n\
       use crate::deep::leaf::Leaf;\n\
       use crate::alias::Origin as Alias;\n\
       use crate::twin::{Twin, 쌍둥이};\n\
       use crate::Hidden;\n\
       pub fn 씀() {\n\
       \x20   let _a = Root::make();\n\
       \x20   let _b: Rel = Rel::make();\n\
       \x20   let _c = Leaf::build();\n\
       \x20   let _d = Alias::mk();\n\
       \x20   let _e: Twin = Twin;\n\
       \x20   쌍둥이();\n\
       \x20   let _f = Hidden::only();\n\
       }\n");

    git(&root, &["init", "-q", "."]);
    git(&root, &["add", "-A"]);
    git(&root, &["-c", "user.email=t@example.com", "-c", "user.name=t", "commit", "-qm", "첫 커밋"]);
    root
}

fn 투영(repo: &std::path::Path) -> Projection {
    Projection::open(&repo.join(".palimpsest/index.redb")).expect("2층")
}

/// `from` 이 가리키는 것 중 **다른 파일에 있는** 심볼들.
fn 파일_간_대상(p: &Projection, from: pal_core::SymbolId, 내파일: &str) -> Vec<String> {
    p.callees(from)
        .expect("정방향")
        .into_iter()
        .filter_map(|id| p.symbol(id).ok().flatten())
        .filter(|s| s.path.as_str() != 내파일)
        .map(|s| s.name.clone())
        .collect()
}

fn 씀(p: &Projection) -> pal_core::SymbolId {
    p.resolve_name("씀").expect("이름").into_iter().next().expect("씀 이 없다").id
}

#[test]
fn a1_임포트한_이름이_다른_파일의_심볼을_가리킨다() {
    let repo = 저장소("a1");
    pal(&repo, &["touch", "씀", "--json"]);
    let p = 투영(&repo);
    let 대상 = 파일_간_대상(&p, 씀(&p), "src/user.rs");

    assert!(
        !대상.is_empty(),
        "파일 간 엣지가 **0** 이다 — 이 파일의 나머지 단언이 전부 공짜로 통과한다"
    );
    assert!(대상.contains(&"Root".to_owned()), "`Root` 로 가는 파일 간 엣지가 없다: {대상:?}");
}

#[test]
fn a2_임포트_참조가_locals_로_안_샌다() {
    let repo = 저장소("a2");
    pal(&repo, &["touch", "씀", "--json"]);
    let p = 투영(&repo);
    let f = p.file(&RepoPath::new("src/user.rs")).expect("파일 노드").expect("없다");
    let pal_core::Slot::Built(c) = f.refs else { panic!("참조 수가 「안 만듦」이다") };

    assert!(c.imported > 0, "임포트 갈래가 0 이다 — 옛날처럼 `locals` 로 샜다");
    // **합이 맞아야 한다** — 갈래를 더하면서 불변식이 깨지면 무엇을 세는지 알 수 없다.
    assert_eq!(
        c.total(),
        c.declarations + c.edges + c.locals + c.top_level + c.unresolved
            + c.before_declaration + c.ambiguous + c.imported,
        "갈래 하나가 샜다"
    );
}

#[test]
fn a3_잠근_축1_의_임포트_형태_넷이_각각_엣지가_된다() {
    let repo = 저장소("a3");
    pal(&repo, &["touch", "씀", "--json"]);
    let p = 투영(&repo);
    let 대상 = 파일_간_대상(&p, 씀(&p), "src/user.rs");

    // ① 균일 경로 형제 모듈 · ② 깊은 경로 · ③ 별칭이 걸린 형제 모듈
    for (형태, 이름) in [
        ("균일 경로 형제 모듈", "Root"),
        ("깊은 경로", "Leaf"),
        ("별칭이 걸린 이름의 원본", "Origin"),
    ] {
        assert!(대상.contains(&이름.to_owned()), "{형태}: `{이름}` 이 안 섰다 — {대상:?}");
    }

    // ④ `super::` 접두 — `deep/leaf.rs` 가 `super::super::inside::Root` 를 부른다.
    let 위 = p.resolve_name("위를_본다").expect("이름").into_iter().next().expect("없다");
    let 위_대상 = 파일_간_대상(&p, 위.id, "src/deep/leaf.rs");
    assert!(
        위_대상.contains(&"Root".to_owned()),
        "`super::` 접두가 안 섰다 — {위_대상:?}"
    );
}

#[test]
fn a4_별칭은_원본을_가리키고_중첩_목록도_엣지가_된다() {
    let repo = 저장소("a4");
    pal(&repo, &["touch", "씀", "--json"]);
    let p = 투영(&repo);
    let 대상 = 파일_간_대상(&p, 씀(&p), "src/user.rs");

    // **부르는 이름이 아니라 원본으로 간다.** `Alias` 라는 심볼은 어디에도 없다.
    assert!(대상.contains(&"Origin".to_owned()), "별칭이 원본을 안 가리켰다 — {대상:?}");
    assert!(!대상.contains(&"Alias".to_owned()), "부르는 이름이 심볼로 둔갑했다 — {대상:?}");

    // 중첩 목록 `use crate::inside::{Rel, Root};` 의 **둘 다**.
    for 이름 in ["Rel", "Root"] {
        assert!(대상.contains(&이름.to_owned()), "중첩 목록의 `{이름}` 이 안 섰다 — {대상:?}");
    }
}

#[test]
fn a5_재수출만_지나는_이름은_엣지가_아니다() {
    let repo = 저장소("a5");
    pal(&repo, &["touch", "씀", "--json"]);
    let p = 투영(&repo);
    let 대상 = 파일_간_대상(&p, 씀(&p), "src/user.rs");

    // `use crate::Hidden;` 은 `lib.rs` 의 `pub use hide::Hidden` 을 지난다. 그 파일에는
    // **정의가 없고** 재수출 추적은 잠근 축1 의 밖이다.
    assert!(
        !대상.contains(&"Hidden".to_owned()),
        "재수출을 따라갔다 — 잠근 축1 의 밖이다: {대상:?}"
    );
    assert!(
        !대상.contains(&"only".to_owned()),
        "재수출 경유의 꼬리까지 따라갔다: {대상:?}"
    );
}

#[test]
fn a6_후보가_유일하지_않으면_엣지가_아니다() {
    let repo = 저장소("a6");
    pal(&repo, &["touch", "씀", "--json"]);
    let p = 투영(&repo);
    let 씀_id = 씀(&p);

    // `쌍둥이` 는 `cfg` 로 둘이다. **하나를 고르면 그것이 조용한 오답이다.**
    let 쌍 = p.resolve_name("쌍둥이").expect("이름");
    assert_eq!(쌍.len(), 2, "픽스처가 동명 심볼 둘을 안 만들었다 — 이 시험이 아무것도 안 잰다");
    let 부름: Vec<_> = p.callees(씀_id).expect("정방향");
    for t in &쌍 {
        assert!(
            !부름.contains(&t.id),
            "후보가 둘인데 하나를 골랐다 — `exact` 만 싣기로 잠갔다"
        );
    }

    // 그래도 **유일한 것에는 엣지가 있다** — 이 단언이 없으면 위가
    // 「아무것도 안 만들어진다」로도 통과한다.
    let 대상 = 파일_간_대상(&p, 씀_id, "src/user.rs");
    assert!(대상.contains(&"Twin".to_owned()), "유일한 후보까지 버렸다 — {대상:?}");
}

#[test]
fn a7_경로_호출의_꼬리가_대상_파일에서만_찾아진다() {
    let repo = 저장소("a7");
    pal(&repo, &["touch", "씀", "--json"]);
    let p = 투영(&repo);
    let 대상 = 파일_간_대상(&p, 씀(&p), "src/user.rs");

    // `Root::make()` 의 `make` — **머리가 곧 담은 것이다.** `Rel` 에도 동명 `make` 가
    // 있으므로, 담은 것을 안 보면 여기가 모호로 떨어지고 꼬리에 엣지가 하나도 없다.
    assert!(
        대상.contains(&"make".to_owned()),
        "경로 호출의 꼬리가 안 섰다 — {대상:?}"
    );
    assert!(대상.contains(&"build".to_owned()), "깊은 경로의 꼬리가 안 섰다 — {대상:?}");
    assert!(대상.contains(&"mk".to_owned()), "별칭 머리의 꼬리가 안 섰다 — {대상:?}");
}
