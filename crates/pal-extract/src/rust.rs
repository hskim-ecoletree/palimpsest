//! Rust 선언 추출 — **중첩 순회 · 스코프 없음**.
//!
//! 결정: [ADR-0027](../../../docs/adr/0027-the-instrument-must-reach-its-own-floor.md) ·
//! #66 · 소유자 지시 2026-08-20 §2.
//!
//! # 왜 순회이면서 L1 인가
//!
//! Kotlin 은 쿼리(최상위만), TypeScript 는 순회 + 스코프(L2)다. Rust 는 **그 사이**다.
//!
//! 처음 계획은 「TypeScript 급」이었는데 사전부검이 그것을 **두 결정이 접힌 것**으로
//! 갈랐다. `impl`·`mod` 안의 표식을 잡는 데 필요한 것은 **중첩 순회**이지 스코프
//! 체인이 아니다 — 스코프가 사는 이유는 `body_digest` 가 지역 이름을 지우는 것이고,
//! 그것은 *낡음의 정밀도* 문제이지 *결박의 존재* 문제가 아니다.
//!
//! **L1 을 고른 대가는 [`crate::grade_of`] 에 적혀 있다.**
//!
//! # 세는 단위는 이 파일이 정하지 않는다
//!
//! 정본은 `corpus/tasks/rust-recall-sample.tsv` 의 머리말이고 **그 파일이 이 코드보다
//! 먼저 커밋됐다**(`git log` 가 증거다). 여기 있는 것은 그 규칙의 구현이다 —
//! 반대 방향이 아니다. 어긋나면 게이트에 목록으로 적고 손 목록을 고치지 않는다.

use pal_core::{
    BodyDigest, Capable, CapabilityId, Containment, ExtractGrade, FileGraph, Language, LanguageId,
    LocalIx, RecoverySite, Span, Symbol, SymbolKind,
};
use tree_sitter::Node;

use crate::extractor::LanguageExtractor;
use crate::parse::{ExtractError, normalize, parse_with, recovery_sites};

/// 레지스트리가 잡는 자리. **무상태다** — #49 가 이것을 `par_iter` 안에서 부른다.
pub(crate) static RUST: RustExtractor = RustExtractor;

/// **벗길 래퍼가 없다.**
///
/// Rust 에서 가시성(`pub`)은 선언 마디 **안**의 토큰이고, 속성(`#[…]`)은 감싸는
/// 마디가 아니라 **앞 형제**다. 그래서 [`crate::parse::벗긴다`] 가 할 일이 없다 —
/// 속성은 [`crate::parse::다음_선언`] 이 **건너뛰기**로 처리한다.
///
/// ⚠ 그 둘은 **다른 축**이다. 앞 판의 계획은 속성을 이 목록에 넣으려 했는데,
/// `벗긴다` 는 **포함 관계**를 벗기는 함수라 형제를 못 넘는다(#66 사전부검).
const 래퍼: [&str; 0] = [];

/// Rust 추출기.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RustExtractor;

impl LanguageExtractor for RustExtractor {
    fn language(&self) -> Language {
        Language::Rust
    }

    fn grade(&self) -> ExtractGrade {
        crate::grade_of(Language::Rust)
    }

    fn extract(&self, source: &[u8]) -> Result<FileGraph, ExtractError> {
        extract_detailed(source)
    }

    fn marked_comments(
        &self,
        source: &[u8],
        markers: &[&str],
    ) -> Result<Vec<crate::parse::MarkedComment>, ExtractError> {
        let language = tree_sitter::Language::new(tree_sitter_rust::LANGUAGE);
        let tree = parse_with(&language, source)?;
        Ok(crate::parse::marked_comments(tree.root_node(), source, markers, &래퍼))
    }
}

/// 마디 이름 → 심볼 종류. **없으면 심볼이 아니다.**
///
/// `impl_item` 이 여기 **없는 것이 결정이다** — 이름이 없으므로 심볼이 아니고
/// 컨테이너로만 쓴다.
fn kind_of(kind: &str) -> Option<SymbolKind> {
    Some(match kind {
        "function_item" => SymbolKind::Function,
        "struct_item" => SymbolKind::Struct,
        "enum_item" => SymbolKind::Enum,
        "trait_item" => SymbolKind::Trait,
        "type_item" => SymbolKind::TypeAlias,
        "const_item" => SymbolKind::Const,
        "static_item" => SymbolKind::Static,
        "mod_item" => SymbolKind::Module,
        "union_item" => SymbolKind::Union,
        "macro_definition" => SymbolKind::Macro,
        _ => return None,
    })
}

/// 안으로 들어가는 마디 — **선언을 담을 수 있는 것.**
///
/// `function_item` 이 여기 **없는 것이 결정이다.** Rust 는 함수 본문 어디에나
/// `fn`·`struct`·`const` 를 놓을 수 있고 그것은 클로저가 아니라 진짜 아이템인데,
/// **그래도 안 센다** — 가르는 것은 「아이템인가」가 아니라 **「함수 안인가」**다
/// (표본 규칙 ②). 세면 폭발한다.
///
/// `macro_definition` 도 없다 — 본문의 `fn $field` 는 선언이 아니라 **틀**이다.
const 내려간다: [&str; 5] =
    ["source_file", "mod_item", "impl_item", "trait_item", "declaration_list"];

/// 순회 중의 심볼 하나 — 마디를 들고 있다가 마지막에 [`Symbol`] 이 된다.
struct 후보<'t> {
    node: Node<'t>,
    name: String,
    kind: SymbolKind,
}

struct 순회<'t> {
    symbols: Vec<후보<'t>>,
    contains: Vec<Containment>,
    /// `impl` 안의 심볼들 — **순회가 끝난 뒤에** 대상 타입에 붙인다.
    ///
    /// ⚠ **앞 판은 순회 중에 붙였고, 그래서 선언 순서에 의존했다.**
    /// `self.symbols.iter().position(…)` 은 **이미 순회한 것만** 보므로
    /// `impl Foo { … }` 가 `struct Foo;` 보다 **위에 있으면** 같은 파일에 있어도
    /// 컨테이너가 안 붙었다 — 구조체를 impl 위로 옮기기만 해도 그래프가 조용히
    /// 바뀌는 형태다(독립 리뷰 R1 이 격리 파일로 잡았다).
    ///
    /// 열: (대상 타입 이름, `impl` 을 감싼 부모, 그 안에서 나온 심볼들)
    미해소_impl: Vec<(String, Option<LocalIx>, Vec<LocalIx>)>,
}

impl<'t> 순회<'t> {
    fn new() -> Self {
        Self { symbols: Vec::new(), contains: Vec::new(), 미해소_impl: Vec::new() }
    }

    /// 순회가 끝난 뒤 `impl` 을 대상 타입에 붙인다 — **파일 전체를 본 뒤**라 순서에
    /// 안 매인다.
    ///
    /// 대상 타입이 **이 파일에 없으면**(외부 타입 `impl` — Rust 의 관용) 아무것도
    /// 안 붙인다. `Containment` 는 `LocalIx` 쌍이라 심볼이 있어야 걸 수 있고,
    /// 없는 것을 심볼로 만들면 손 표본 규칙 ①을 뒤집는 일이다. **그 잔여는 #78 이 진다.**
    fn impl_을_해소한다(&mut self) {
        // **부모 사슬** — 심볼 i 를 담는 심볼. `contains` 를 뒤집어 만든다.
        let mut 담는이: Vec<Option<LocalIx>> = vec![None; self.symbols.len()];
        for c in &self.contains {
            if let Some(slot) = 담는이.get_mut(c.child.0 as usize) {
                *slot = Some(c.parent);
            }
        }
        for (대상, 감싼_부모, 자식들) in std::mem::take(&mut self.미해소_impl) {
            // **컨테이너 후보는 「담을 수 있는 종류」만이다.** 같은 이름의 함수가
            // 있어도 그것은 `impl` 의 대상이 아니다.
            //
            // ★ **그리고 같은 모듈 안이어야 한다.** 앞 판은 파일 전체의 첫 일치를
            // 잡아 **모듈 경계를 넘었다** — `mod a { struct S; } mod b { impl S { fn f } }`
            // 에서 `f` 가 `a::S` 에 붙었고, 최상위 `S` 와 `mod m { struct S; }` 가
            // 함께 있으면 `m::S` 의 메서드까지 최상위 `S` 로 갔다.
            // cargo 380 파일 중 **11(2.9%)** 이 그 형태다(독립 리뷰 R2).
            let 부모 = self.symbols.iter().enumerate().position(|(i, s)| {
                s.name == 대상
                    && matches!(
                        s.kind,
                        SymbolKind::Struct
                            | SymbolKind::Enum
                            | SymbolKind::Trait
                            | SymbolKind::Union
                            | SymbolKind::TypeAlias
                    )
                    // `impl` 을 감싼 것과 대상을 담는 것이 **같아야** 한다.
                    && 담는이.get(i).copied().flatten().map(|p| p.0) == 감싼_부모.map(|p| p.0)
            });
            let Some(ix) = 부모.map(|i| LocalIx(u32::try_from(i).unwrap_or(u32::MAX))) else {
                // 못 찾았다 — 외부 타입이다. `impl` 을 감싼 부모가 있으면 그것에 붙인다
                // (`mod m { impl 외부타입 { fn f } }` 의 `f` 는 적어도 `m` 안이다).
                if let Some(p) = 감싼_부모 {
                    self.contains.extend(자식들.into_iter().map(|c| Containment { parent: p, child: c }));
                }
                continue;
            };
            self.contains.extend(자식들.into_iter().map(|c| Containment { parent: ix, child: c }));
        }
    }

    /// 마디 하나와 그 자식들.
    ///
    /// `parent` 는 **담는 심볼의 자리**다. `impl` 은 심볼이 아니므로 자기 부모의
    /// 자리를 그대로 물려준다 — 그러면 `impl Foo` 안의 `fn bar` 가 `Foo` 에 담긴다.
    fn walk(&mut self, node: Node<'t>, source: &[u8], parent: Option<LocalIx>) {
        let mut cursor = node.walk();
        let kids: Vec<Node<'t>> = node.children(&mut cursor).collect();
        drop(cursor);

        for child in kids {
            let kind = child.kind();

            // ── 심볼인가 ──────────────────────────────────────────────
            if let Some(sym_kind) = kind_of(kind) {
                let Some(name) = 이름(child, source) else { continue };
                let ix = LocalIx(u32::try_from(self.symbols.len()).unwrap_or(u32::MAX));
                self.symbols.push(후보 { node: child, name, kind: sym_kind });
                if let Some(p) = parent {
                    self.contains.push(Containment { parent: p, child: ix });
                }
                // **자기 자신을 부모로 삼아 안으로 들어간다** — `mod m { fn f }` 에서
                // `f` 가 `m` 에 담긴다. `fn` 안으로는 안 들어간다(`내려간다` 참조).
                if 내려간다.contains(&kind) {
                    self.walk(child, source, Some(ix));
                }
                continue;
            }

            // ── 심볼은 아니지만 안에 선언이 있을 수 있다 ──────────────
            if 내려간다.contains(&kind) {
                if kind == "impl_item" {
                    // ★ **여기서 해소하지 않는다.** 대상 타입이 파일 뒤쪽에 있을 수
                    // 있고, 순회 중에 찾으면 **선언 순서에 매인다.**
                    let 앞 = self.symbols.len();
                    self.walk(child, source, None);
                    let 자식들: Vec<LocalIx> = (앞..self.symbols.len())
                        .map(|i| LocalIx(u32::try_from(i).unwrap_or(u32::MAX)))
                        .collect();
                    if let Some(대상) = impl_대상(child, source) {
                        self.미해소_impl.push((대상, parent, 자식들));
                    } else if let Some(p) = parent {
                        // 대상 타입이 이름을 안 갖는다(튜플·배열·함수 포인터).
                        self.contains
                            .extend(자식들.into_iter().map(|c| Containment { parent: p, child: c }));
                    }
                    continue;
                }
                self.walk(child, source, parent);
            }
        }
    }
}

/// 선언의 이름 — `name` 필드가 정본이다.
fn 이름(node: Node<'_>, source: &[u8]) -> Option<String> {
    let n = node.child_by_field_name("name")?;
    n.utf8_text(source).ok().map(std::borrow::ToOwned::to_owned)
}

/// `impl` 의 대상 타입 이름 — **벗기는 것 셋**(표본 규칙 ①).
///
/// 타입 인자·수명(`BuildContext<'_, '_>` → `BuildContext`) · 경로 자격
/// (`http::response::Parts` → `Parts`) · 참조(`&T` → `T`).
///
/// ⚠ **트레잇 이름은 안 싣는다.** `impl From<A> for Error` 와 `impl Error` 가 같은
/// 컨테이너 이름을 갖고 그 안의 동명 함수가 좌표를 다툰다(R-16).
/// **이 회차의 범위 밖**이고 [#78] 이 진다.
///
/// **세는 자리는 `--example coord_collisions` 다** — 수를 여기 안 적는다.
/// 앞 판이 적은 464(6.1%)는 격리 스파이크의 값이었고 재현되지 않았다(독립 리뷰 R3).
///
/// [#78]: https://github.com/hskim-ecoletree/palimpsest/issues/78
fn impl_대상(node: Node<'_>, source: &[u8]) -> Option<String> {
    let t = node.child_by_field_name("type")?;
    Some(마지막_이름(t, source))
}

/// 타입 마디에서 이름 하나를 꺼낸다 — 위 셋을 벗긴 결과.
fn 마지막_이름(node: Node<'_>, source: &[u8]) -> String {
    match node.kind() {
        // `Foo<T>` → `Foo`
        "generic_type" => node
            .child_by_field_name("type")
            .map_or_else(|| 원문(node, source), |t| 마지막_이름(t, source)),
        // `a::b::C` → `C`
        "scoped_type_identifier" => node
            .child_by_field_name("name")
            .map_or_else(|| 원문(node, source), |t| 마지막_이름(t, source)),
        // `&T` · `&mut T` → `T`
        "reference_type" => node
            .child_by_field_name("type")
            .map_or_else(|| 원문(node, source), |t| 마지막_이름(t, source)),
        _ => 원문(node, source),
    }
}

fn 원문(node: Node<'_>, source: &[u8]) -> String {
    node.utf8_text(source).unwrap_or_default().to_owned()
}

/// 선언들을 소스 순서로 + **파싱이 성했는가**.
///
/// # Errors
/// 문법을 붙이지 못하거나 파싱이 중단되면 [`ExtractError`].
pub fn extract_detailed(source: &[u8]) -> Result<FileGraph, ExtractError> {
    let language = tree_sitter::Language::new(tree_sitter_rust::LANGUAGE);
    let tree = parse_with(&language, source)?;

    let mut walk = 순회::new();
    walk.walk(tree.root_node(), source, None);
    walk.impl_을_해소한다();

    let symbols: Vec<Symbol> = walk
        .symbols
        .iter()
        .map(|c| Symbol {
            name: c.name.clone(),
            kind: c.kind,
            body: BodyDigest::of_normalized(&normalize(c.node, source)),
            // **L1 이라 심볼 단위로도 `ordinal` 이다.** 스코프가 없으므로 어느 이름이
            // 지역인지 모르고, 모르면 지우지 않는다 — R-22 의 요구다.
            identity: crate::grade_of(Language::Rust).identity(),
            // ⚠ **속성은 span 에 안 든다.** `#[must_use] fn f` 의 시작은 `fn` 이다.
            // 넓히면 `pal narrative` 의 자리 맵(정확 일치)이 주석 인접 판정과
            // 서로 반대 방향으로 갈려 결박이 43 → 16 이 된다(#66 사전부검 R3).
            //
            // **대가**: `#[derive(Debug)]` ↔ `#[derive(Debug, Clone)]` 의
            // `body_digest` 가 **같다**. 그 축은 이 회차의 범위 밖이고 분할이 진다.
            span: Span {
                byte_start: c.node.start_byte(),
                byte_end: c.node.end_byte(),
                line_start: u32::try_from(c.node.start_position().row).unwrap_or(u32::MAX) + 1,
                line_end: u32::try_from(c.node.end_position().row).unwrap_or(u32::MAX) + 1,
            },
        })
        .collect();

    let sites: Vec<RecoverySite> = recovery_sites(tree.root_node());
    let mut graph = FileGraph::flat(
        LanguageId::new(Language::Rust.name()),
        crate::grade_of(Language::Rust),
        symbols,
        sites,
        // **빈 집합이 아니라 안 만들었다고 적는다.** 이 추출기는 `pub` 를 안 읽는다.
        // 빈 `ExportSet` 은 *"아무것도 안 내보낸다"* 는 뜻이고 그것은 거짓이다.
        Capable::not_built(CapabilityId::new("F02", "rust-exports")),
        Capable::not_built(CapabilityId::new("F02", "rust-imports")),
        // **스코프를 안 만든다** — L1 을 고른 것이 이 자리다.
        Capable::not_built(CapabilityId::new("F02", "rust-scopes")),
    );
    // `flat` 은 포함 관계를 비우므로 여기서 채운다. **이 추출기는 중첩을 본다.**
    graph.contains = walk.contains;
    Ok(graph)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn 심볼(src: &str) -> Vec<(String, SymbolKind)> {
        extract_detailed(src.as_bytes())
            .unwrap()
            .symbols
            .into_iter()
            .map(|s| (s.name, s.kind))
            .collect()
    }

    #[test]
    fn 중첩을_본다() {
        // **이 회차가 존재하는 이유다.** 최상위만 세면 `impl`·`mod` 안이 사라진다.
        let g = extract_detailed(b"mod m { struct S; impl S { fn f() {} } }").unwrap();
        let names: Vec<&str> = g.symbols.iter().map(|s| s.name.as_str()).collect();
        assert_eq!(names, ["m", "S", "f"], "중첩 순회가 안 돈다");
        // `S` 는 `m` 에, `f` 는 `S` 에 담긴다 — `impl` 은 심볼이 아니라 부모를 갈아 낀다.
        assert_eq!(g.contains.len(), 2);
    }

    #[test]
    fn 함수_안은_안_본다() {
        // 규칙 ② — 가르는 것은 「아이템인가」가 아니라 「함수 안인가」다.
        // `return` 뒤의 진짜 아이템도 안 센다(표본 작성이 지목한 가장 큰 판단).
        assert_eq!(심볼("fn outer() { fn inner() {} struct Local; }").len(), 1);
    }

    #[test]
    fn impl_대상이_벗겨진다() {
        // 표본 규칙 ① — 타입 인자·경로 자격·참조를 벗긴다. 두 에이전트가 독립적으로
        // 같은 판단을 했고 그것이 규칙이 됐다.
        for src in [
            "struct B; impl B<'_> { fn f() {} }",
            "struct B; impl a::b::B { fn f() {} }",
            "struct B; impl B { fn f() {} }",
        ] {
            let g = extract_detailed(src.as_bytes()).unwrap();
            assert_eq!(g.contains.len(), 1, "{src} — impl 대상이 안 붙었다");
        }
    }

    #[test]
    fn 매크로는_정의만_센다() {
        // 규칙 ⑦ — 본문의 `fn $x` 는 선언이 아니라 틀이다.
        let s = 심볼("macro_rules! m { () => { fn generated() {} } }");
        assert_eq!(s, [("m".to_owned(), SymbolKind::Macro)]);
    }

    #[test]
    fn cfg_test_도_센다() {
        // 소유자 지시 2026-08-20 §3 — *"번복할게 #[cfg(test)] 도 진행해"*.
        // 추출기는 `cfg` 를 해석하지 않는다.
        let s = 심볼("#[cfg(test)]\nmod tests { fn t() {} }");
        assert_eq!(s.len(), 2, "cfg(test) 를 걸렀다");
    }

    #[test]
    fn 종류_열이_다_선다() {
        let s = 심볼(
            "struct S; enum E {} trait T {} type A = u8; const C: u8 = 0;\n\
             static X: u8 = 0; mod m {} union U { a: u8 } fn f() {}",
        );
        let kinds: Vec<SymbolKind> = s.into_iter().map(|(_, k)| k).collect();
        assert!(kinds.contains(&SymbolKind::Struct));
        assert!(kinds.contains(&SymbolKind::Trait));
        assert!(kinds.contains(&SymbolKind::Union));
        assert!(kinds.contains(&SymbolKind::Static));
    }

    #[test]
    fn 본문_없는_trait_시그니처는_안_센다() {
        // 규칙 ⑤ — 가르는 것은 **본문의 유무**다. 표본 작성이 12 건을 이 규칙으로 뺐고,
        // 안 지키는 추출기는 정확히 그만큼 과잉 검출한다.
        let s = 심볼("trait T { fn 시그니처(&self); fn 기본(&self) {} }");
        let names: Vec<String> = s.into_iter().map(|(n, _)| n).collect();
        assert!(names.contains(&"기본".to_owned()), "기본 구현이 빠졌다");
        assert!(!names.contains(&"시그니처".to_owned()), "본문 없는 시그니처를 셌다");
    }

    #[test]
    fn impl_이_타입보다_앞에_있어도_붙는다() {
        // ★ **독립 리뷰 R1 이 격리 파일로 잡은 자리다.** 앞 판은 순회 중에 대상을
        // 찾아서 `impl Foo` 가 `struct Foo` 보다 위에 있으면 컨테이너가 안 붙었다 —
        // **구조체를 impl 위로 옮기기만 해도 그래프가 조용히 바뀌었다.**
        let 앞 = extract_detailed(b"impl Foo { fn early() {} }\nstruct Foo;").unwrap();
        let 뒤 = extract_detailed(b"struct Foo;\nimpl Foo { fn late() {} }").unwrap();
        assert_eq!(앞.contains.len(), 1, "impl 이 앞에 있을 때 안 붙었다");
        assert_eq!(뒤.contains.len(), 1, "impl 이 뒤에 있을 때 안 붙었다");
    }

    #[test]
    fn 동명_타입이_모듈마다_있어도_제_것에_붙는다() {
        // ★ **독립 리뷰 R2 가 격리 빌드로 잡은 자리다.** 앞 판은 파일 전체의 첫
        // 일치를 잡아 모듈 경계를 넘었다 — cargo 380 중 11(2.9%)이 그 형태다.
        let g = extract_detailed(
            b"struct S; impl S { fn top() {} }\nmod m { struct S; impl S { fn nested() {} } }",
        )
        .unwrap();
        let 이름 = |ix: LocalIx| g.symbols[ix.0 as usize].name.clone();
        // `top` 은 최상위 `S`(ix 0)에, `nested` 는 `m` 안의 `S` 에 붙어야 한다.
        let top_부모 = g
            .contains
            .iter()
            .find(|c| 이름(c.child) == "top")
            .map(|c| c.parent.0)
            .expect("top 이 안 붙었다");
        let nested_부모 = g
            .contains
            .iter()
            .find(|c| 이름(c.child) == "nested")
            .map(|c| c.parent.0)
            .expect("nested 가 안 붙었다");
        assert_ne!(top_부모, nested_부모, "동명 타입 둘이 한 컨테이너로 접혔다");
    }

    #[test]
    fn 모듈_밖의_동명_타입에_안_붙는다() {
        // `mod a { struct S; }` 와 `mod b { impl S { … } }` 는 **다른 타입**이다.
        let g = extract_detailed(b"mod a { struct S; }\nmod b { impl S { fn f() {} } }").unwrap();
        let f = g.symbols.iter().position(|s| s.name == "f").expect("f 가 없다");
        let 부모 = g.contains.iter().find(|c| c.child.0 as usize == f).map(|c| c.parent);
        // `b` 안에 `S` 가 없으므로 `impl` 을 감싼 `b` 에 붙는다 — `a::S` 가 아니다.
        assert_eq!(부모.map(|p| g.symbols[p.0 as usize].name.clone()), Some("b".to_owned()));
    }

    #[test]
    fn 외부_타입_impl_은_안_붙는다() {
        // 대상이 이 파일에 없으면 `Containment` 를 만들 수 없다 — 심볼이 있어야
        // 걸 수 있고, 없는 것을 심볼로 만들면 손 표본 규칙 ①을 뒤집는다. 잔여는 #78.
        let g = extract_detailed(b"impl std::fmt::Display for u8 { fn fmt() {} }").unwrap();
        assert_eq!(g.symbols.len(), 1);
        assert_eq!(g.contains.len(), 0, "외부 타입에 억지로 붙였다");
    }

    #[test]
    fn 같은_이름의_함수는_impl_대상이_아니다() {
        // 컨테이너 후보는 「담을 수 있는 종류」만이다.
        let g = extract_detailed(b"fn Foo() {}\nstruct Foo;\nimpl Foo { fn m() {} }").unwrap();
        let 부모 = g.contains.first().map(|c| g.symbols[c.parent.0 as usize].kind);
        assert_eq!(부모, Some(SymbolKind::Struct), "함수에 붙었다");
    }

    #[test]
    fn 속성은_span_에_안_든다() {
        // ⚠ 이것을 바꾸면 `narrative` 의 자리 맵과 주석 인접 판정이 갈린다.
        let g = extract_detailed(b"#[must_use]\nfn f() {}").unwrap();
        let src = "#[must_use]\nfn f() {}";
        assert_eq!(&src[g.symbols[0].span.byte_start..][..2], "fn");
    }
}
