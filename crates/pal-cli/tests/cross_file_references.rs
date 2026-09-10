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

/// `E4` — **화면이 능력 축 셋을 가르고 파일 간 회계를 찍는다.**
///
/// # 이 시험이 결박 `91cdc3add647a3f7` 의 결정 명제를 잰다
///
/// 그 결박의 명제는 *"이 라벨은 두 방향을 다 적어야 한다"* 이고 둘은 이것이다 —
/// **호출이 아닌 것을 담는다**(타입 참조·구조체 리터럴·경로 머리)와 **호출인데 안 담는
/// 것도 있다**. *"둘 중 하나만 적으면 0 이 「아무도 안 부른다」로 읽힌다."*
///
/// 이 회차가 그 화면을 고쳤으므로 명제가 아직 참인지 여기서 잰다.
#[test]
fn e4_화면이_능력_축_셋을_가르고_회계를_찍는다() {
    let repo = 저장소("e4");
    let 화면 = pal(&repo, &["touch", "씀"]);

    // ── 방향 ① — **호출이 아닌 것을 담는다.** 명제의 앞 절반.
    assert!(
        화면.contains("호출뿐 아니라 타입 참조"),
        "「호출이 아닌 것도 담는다」가 화면에서 사라졌다 — 0 이 「안 부른다」로 읽힌다:\n{화면}"
    );

    // ── 방향 ② — **호출인데 안 담는 것.** 명제의 뒤 절반.
    //
    //    ⚠ 그 목록이 줄었다. `S::foo()` 는 이제 담으므로 `x.foo()` 만 남고, 그것은
    //    **능력 부재가 아니라 범위 밖**이다. 화면이 그 둘을 갈라야 한다.
    assert!(
        화면.contains("`x.foo()` 는 아직 안 셉니다"),
        "멤버 해소가 값을 안 만든다는 것을 화면이 안 적는다:\n{화면}"
    );
    assert!(
        화면.contains("범위 밖"),
        "`x.foo()` 를 「능력 부재」로 적으면 사람이 「곧 될 것」으로 읽는다:\n{화면}"
    );
    assert!(
        !화면.contains("`x.foo()` 와 `S::foo()` 는 아직 안 셉니다"),
        "`S::foo()` 를 아직 못 본다고 적는데 오늘 그것은 거짓이다:\n{화면}"
    );

    // ── 회계 — ⓐ·ⓑ 가 **갈려서** 찍힌다. 합치면 `E2` 의 두 하한을 못 잰다.
    assert!(
        화면.contains("ⓐ `cross-file-import`") && 화면.contains("ⓑ `path-resolution`"),
        "파일 간 회계가 갈래별로 안 찍힌다:\n{화면}"
    );
    // 못 선 까닭도 갈래별이어야 무엇이 막혔는지 읽힌다.
    assert!(
        화면.contains("ⓐ 가 못 선 까닭") || 화면.contains("ⓑ 가 못 선 까닭"),
        "못 선 까닭이 안 찍힌다 — 수만 있으면 무엇이 막혔는지 모른다:\n{화면}"
    );

    // ── 이관표 — **못 선 몫이 가는 문.** 까닭의 셈만으로는 결함과 경계가 안 갈린다.
    //
    //    잠긴 의도가 이것을 게이트·이 화면·잔여 **셋 다**에 요구한다 —
    //    *"셋 중 하나라도 빠지면 그것이 조용한 축소다"*. 여기가 그중 화면 몫이다.
    assert!(
        화면.contains("못 선 몫이 가는 문"),
        "이관표가 화면에 없다 — 못 푼 것이 결함인지 경계인지 사람이 못 읽는다:\n{화면}"
    );
    for 문 in ["`A5`·`A5-a`", "`#133`(L2)", "후보 생성의 모호"] {
        assert!(화면.contains(문), "이관표에 {문} 로 가는 문이 없다:\n{화면}");
    }
    // ★ **열쇠와 문이 짝이어야 한다.** 회계가 안 가른 이름을 화면이 부르면 그 문은
    //   아무 몫도 안 받고, 그것이 「적었다」의 외양만 남는 자리다.
    for 열쇠 in ["no_symbol_at_crate_root", "no_symbol", "outside_repo", "ambiguous"] {
        assert!(
            화면.contains(&format!("`{열쇠}` →")),
            "이관표가 `{열쇠}` 의 문을 안 적는다:\n{화면}"
        );
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// B — **못 푼 참조가 표시된다.** 잠근 원문의 뒤 절반(*"경계 + 못 푼 참조"*)이다.
//
// ⚠ **이 넷은 A 계열과 짝이다.** 해소가 서면 「푼 것」이 관계가 되고, 이 넷이 있어야
//    「못 푼 것」도 관계가 된다. 뒤엣것이 없으면 화면은 *"몇 건이 막혔다"* 까지만 말하고
//    **어느 자리의 무엇이 왜 막혔는지**를 못 말한다.
// ═════════════════════════════════════════════════════════════════════════════

/// 이 저장소가 산출한 못 푼 참조 전량. **[`None`] 이면 패스를 안 지난 것이라 실패다.**
fn 못푼참조(p: &Projection) -> Vec<pal_core::UnresolvedRef> {
    p.unresolved_refs()
        .expect("못 푼 참조를 읽지 못했다")
        .expect("이 투영이 파일 간 해소 패스를 안 지났다 — 「0 건」이 아니다")
}

#[test]
fn b1_unresolved_ref_가_사전_등록된_네_칸_그대로_있다() {
    let repo = 저장소("b1");
    pal(&repo, &["touch", "씀", "--json"]);
    let p = 투영(&repo);
    let 목록 = 못푼참조(&p);
    assert!(!목록.is_empty(), "못 푼 참조가 0 건이다 — 아래 단언이 전부 공짜로 통과한다");

    // 스키마가 사전 등록한 넷이 **그대로** 서는가. `at` 는 `REFERS_UNRESOLVED` 가
    // 요구하는 공통 넷의 넷째다.
    let v = serde_json::to_value(&목록[0]).expect("직렬화");
    let mut 칸: Vec<&str> = v.as_object().expect("객체").keys().map(String::as_str).collect();
    칸.sort_unstable();
    assert_eq!(
        칸,
        vec!["at", "attempts", "name", "reason", "site"],
        "스키마가 사전 등록한 칸과 실물이 갈렸다"
    );
}

#[test]
fn b2_attempts_가_빈_배열이_아니다() {
    let repo = 저장소("b2");
    pal(&repo, &["touch", "씀", "--json"]);
    let p = 투영(&repo);
    let 목록 = 못푼참조(&p);
    assert!(!목록.is_empty(), "못 푼 참조가 0 건이다");

    // ⚠ **빈 배열로 채우고 통과라 적지 않는다** — 스키마 주석이 이름 붙인 실패 형태이고
    //    잠긴 의도의 `## 차선책` 이 그것을 명시로 금한다.
    let 빈것: Vec<&str> =
        목록.iter().filter(|u| u.attempts.is_empty()).map(|u| u.name.as_str()).collect();
    assert!(빈것.is_empty(), "`attempts` 가 빈 행이 있다: {빈것:?}");

    // **걸음이 실제로 지난 것인가** — 첫 걸음은 언제나 임포트 항목 찾기다. 그 순서가
    // 안 지켜지면 이 목록은 관측이 아니라 재구성이다.
    for u in &목록 {
        assert_eq!(
            u.attempts[0].step,
            pal_core::AttemptStep::ImportItem,
            "첫 걸음이 임포트 항목 찾기가 아니다: {} {:?}", u.name, u.attempts
        );
        // 마지막 걸음이 끊긴 자리다. **끊김은 「0」이 아니라 「유일하지 않다」**이고
        // 그 둘이 각각 `no_*` 와 `Ambiguous` 로 간다 — 잠근 문면의 *"exact 만"* 이
        // 그 자리다. 그래서 재는 값은 0 이 아니라 **1 이 아님**이다.
        let 마지막 = u.attempts.last().expect("걸음").found;
        assert_ne!(
            마지막, 1,
            "마지막 걸음이 후보를 하나로 좁혔는데 못 푼 참조로 실렸다: {} {:?}",
            u.name, u.attempts
        );
    }
}

#[test]
fn b3_화면이_능력_부재_대신_값을_찍는다() {
    let repo = 저장소("b3");
    let 화면 = pal(&repo, &["touch", "씀"]);

    assert!(화면.contains("■ 내가 모르는 것"), "그 구역이 화면에 없다:\n{화면}");
    // 착수 시점의 문자열. **이것이 남아 있으면 값이 안 선 것이다.**
    assert!(
        !화면.contains("이 빌드에는 unresolved-refs 능력이 없습니다"),
        "능력 부재 선언이 그대로 남았다 — `E5-a` 가 반증으로 잡는 자리다:\n{화면}"
    );
    assert!(
        !화면.contains("F08 미구축"),
        "F08 을 미구축으로 적는데 값이 서 있다:\n{화면}"
    );
    // 값이다 — 이름과 까닭과 지난 걸음이 함께 찍힌다.
    assert!(
        화면.contains("지난 걸음 import_item"),
        "못 푼 참조가 「(있음)」으로만 찍힌다 — 수만 있으면 고칠 자리를 안 준다:\n{화면}"
    );
}

#[test]
fn b4_참조_자리에_잇는_엣지가_등록돼_있다() {
    let schema = pal_core::GraphSchema::parse(include_str!("../../../schema/graph.toml"))
        .expect("스키마 정본");
    let e = schema
        .edges
        .get("REFERS_UNRESOLVED")
        .expect("`UnresolvedRef` 를 참조 자리에 잇는 엣지가 등록돼 있지 않다");

    // **이름·양 끝·운반 자리** 셋을 다 잰다. 하나라도 안 재면 아무 엣지나 등록해도 닫힌다.
    assert_eq!(e.from, "UnresolvedRef", "출발점이 다르다");
    assert_eq!(e.to, vec!["Symbol".to_owned()], "도착점이 다르다");
    let c = e.carried_by.carrier().expect("실린 자리가 없다");
    assert_eq!(c.rust_type, "UnresolvedRef");
    assert_eq!(c.field, "site", "참조가 일어난 자리를 지는 필드가 아니다");

    // 그 노드가 더 이상 `not_built` 가 아니다 — 값이 서는데 자리만 있다고 적으면 거짓이다.
    let n = schema.nodes.get("UnresolvedRef").expect("노드 선언");
    assert_eq!(n.status, pal_core::NodeStatus::Built, "값이 서는데 `not_built` 로 적혀 있다");
}

// ═════════════════════════════════════════════════════════════════════════════
// 효과 실행이 잡은 것 — **「0 건」이 「못 읽었다」였다**
//
// 규약 §8 이 요구하는 효과(*"테스트도 CI 도 아닌 것이 그 회차의 산출을 돌린 출력"*)를
// 내려고 `pal touch print_facts` 를 돌렸더니 「이 좌표에 걸린 것 (0) 아직 없습니다」가
// 나왔다. 그런데 `.palimpsest/intent/bindings.jsonl` 에는 **그 심볼 다이제스트를
// 지켜보는 결박이 실재했다.** 파생 저장소가 없어서 못 읽은 것이고, 화면이 그것을
// 「0 건」으로 적고 있었다 — 금지역 「사실이 아닌 것을 사실로」다.
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn 파생_저장소가_없으면_화면이_0_건이라_안_적는다() {
    let repo = 저장소("intentless");
    // **정본은 있고 파생물은 없다** — 이 저장소의 실제 상태이고, `.gitignore` 가
    // 파생물을 지우므로 새로 받은 사람에게 언제나 이 상태다.
    std::fs::create_dir_all(repo.join(".palimpsest/intent")).expect("의도 자리");
    std::fs::write(
        repo.join(".palimpsest/intent/bindings.jsonl"),
        "{\"kind\":\"header\",\"schema_version\":2}\n",
    )
    .expect("정본");
    assert!(!repo.join(".palimpsest/intent.redb").exists(), "파생물이 있으면 이 시험이 무의미하다");

    let 화면 = pal(&repo, &["touch", "씀"]);
    assert!(
        화면.contains("못 읽었습니다 — 「0 건」이 아닙니다"),
        "파생 저장소가 없는데 화면이 그 사실을 안 말한다:\n{화면}"
    );
    assert!(
        !화면.contains("아직 없습니다"),
        "「0 건」으로 적었다 — 못 읽은 것과 없는 것이 같은 말이 됐다:\n{화면}"
    );
    // **구제를 이름으로 적는다** — 사유만 적으면 사람이 다음 걸음을 모른다.
    assert!(화면.contains("pal intent import"), "세우는 방법이 화면에 없다:\n{화면}");

    // ── 음성 대조 — **정본도 없으면 「0 건」이 참이다.** 그때까지 이 문구가 나오면
    //    이 검사는 파생 저장소의 부재가 아니라 **언제나**를 재는 것이다.
    let 빈곳 = 저장소("intentless-neg");
    let 화면2 = pal(&빈곳, &["touch", "씀"]);
    assert!(
        화면2.contains("아직 없습니다"),
        "정본이 없는데도 「못 읽었다」로 적는다 — 이 자가 부재를 안 가른다:\n{화면2}"
    );
}
