//! Rust 선언 추출 — **중첩 순회 + 스코프 체인. 등급은 `L1` 이다.**
//!
//! 결정: [ADR-0027](../../../docs/adr/0027-the-instrument-must-reach-its-own-floor.md) ·
//! #66 · #130 · 소유자 지시 2026-08-20 §2 · 2026-09-07.
//!
//! # 두 결정이 여기서 갈린다
//!
//! Kotlin 은 쿼리(최상위만), TypeScript 는 순회 + 스코프다. Rust 는 **둘을 따로 골랐다.**
//!
//! | 축 | 값 | 언제 정해졌나 |
//! |---|---|---|
//! | 심볼을 어떻게 찾나 | **중첩 순회** — `impl`·`mod` 안까지 | #66 (2026-08-20) |
//! | 이름을 해소하나 | **한다** — [`crate::scopes`] 뼈대 + [`crate::rust_scopes`] 표 | #130 (2026-09-08) |
//! | 언어 등급 | **`L1`** — `body_digest` 는 지역 이름을 안 지운다 | #130. 승격은 [#133] |
//!
//! 셋째 줄이 둘째 줄과 어긋나 보인다. 어긋나지 않는다 — **등급 글자가 지는 것은
//! *요약이 리네임에 흔들리는가*이고, 참조 엣지가 지는 것은 *이름이 어느 선언을
//! 가리키는가*다.** 까닭은 [`crate::grade_of`] 옆에 적혀 있다.
//!
//! # 못 세는 몫 — **분모와 함께 적는다** (2026-09-08 실측 · **회차 착수 시점의 추적 파일 134 개**)
//!
//! ⚠ **지금 `git ls-files '*.rs'` 는 138 을 산출한다** — 이 회차가 더한 넷(`rust_scopes.rs` ·
//! `ts_scopes.rs` · `examples/scope_variants.rs` · `tests/rust_references.rs`)이 그 뒤에
//! 실렸다. 아래 세 수는 그 넷이 없는 134 개에서 쟀고 **다시 안 쟀다.**
//!
//! | 무엇 | 못 세는 수 / 후보 | 왜 원리상 못 보나 |
//! |---|--:|---|
//! | 포맷 문자열 캡처 (`println!("{x}")` 의 `x`) | **987 / 987** | 문자열 리터럴 안이라 문법 트리에 이름이 없다 |
//! | 멤버 호출 (`x.foo()` 의 `foo`) | **13,951 / 13,951** | `field_identifier` 는 스코프 참조가 아니다. 멤버 해소는 L2c 이고 F07 에서도 안 한다 |
//! | 경로 호출 (`S::new()` 의 `new`) | **4,217 / 4,217** | 경로 꼬리는 머리를 풀어야 알고 그것이 F07(L2b)이다 |
//!
//! 비교: 같은 저장소에서 **실제로 서는 참조가 6,639 · 엣지 4,255** 다(변형 대조 장치의 셈 ·
//! 138 파일). 그 둘은 위 세 수와 분모가 다르다. 세 몫을 안 적으면
//! *"파일 안에서 아무도 안 부른다"* 는 0 이 **아무도 안 부른다**로 읽힌다.
//!
//! ⚠ **넷째 몫이 있다** — 같은 스코프에 동명 아이템이 둘이면 해소하지 않는다
//! ([`pal_core::RefResolution::Ambiguous`] · 실측 참조 47 건). **해소해도 엣지가 늘지 않는다** —
//! 변형 `V10`(둘 중 앞선 것을 고른다)이 엣지 4,244 를 산출하고 그 안에 **가짜 31 · 누락 42** 가 있다.
//!
//! **그 47 건은 한 가지가 아니다**(정반합 판 1 이 갈랐다). 자리 21 곳 중 도구가 찍는
//! 보기 12 개를 합(合)이 전부 열어 **10 은 `cfg` 쌍둥이 · 2 는 아니다** 로 갈랐고,
//! 나머지 9 곳은 안 쟀다:
//!
//! - **`cfg` 쌍둥이** — 원리상 못 가른다. 추출기가 `cfg` 를 해석하지 않기로 했다
//! - **같은 이름의 값 아이템과 타입 아이템**(`pub enum 상태` 와 `pub fn 상태`) —
//!   **Rust 는 이름 공간으로 가른다.** 못 가르는 것은 이 표가 타입 아이템을
//!   `Namespace::Value` 에도 묶기 때문이고(값 자리 358 건을 얻는 대가), 그것은
//!   원리상 한계가 아니라 **고칠 수 있는 결함**이다. 승격은 [#133]
//!
//! [#133]: https://github.com/hskim-ecoletree/palimpsest/issues/133
//!
//! # 세는 단위는 이 파일이 정하지 않는다
//!
//! 정본은 `corpus/tasks/rust-recall-sample.tsv` 의 머리말이고 **그 파일이 이 코드보다
//! 먼저 커밋됐다**(`git log` 가 증거다). 여기 있는 것은 그 규칙의 구현이다 —
//! 반대 방향이 아니다. 어긋나면 게이트에 목록으로 적고 손 목록을 고치지 않는다.

use std::collections::HashMap;

use pal_core::{
    BodyDigest, Capable, Containment, ExportSet, ExtractGrade, FileGraph, ImportSet, ImportedItem,
    Language, LanguageId, LocalIx, Span, Symbol, SymbolKind,
};
use tree_sitter::Node;

use crate::extractor::LanguageExtractor;
use crate::parse::{ExtractError, normalize, parse_with, recovery_sites};
use crate::rust_scopes::RustScopeRules;
use crate::scopes;

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
/// **그래도 안 잰다** — 가르는 것은 「아이템인가」가 아니라 **「함수 안인가」**다
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
    extract_with(source, RustScopeRules::기준)
}

/// 표를 골라 뽑는다 — **변형 대조 전용 입구**(`V1`~`V10`).
///
/// 추출기는 [`RustScopeRules::기준`] 만 쓴다. 다른 조합으로 부르는 자리는
/// `--example scope_variants` 하나이고, 그것이 이 회차의 완수 증인이다.
///
/// # Errors
/// 문법을 붙이지 못하거나 파싱이 중단되면 [`ExtractError`].
pub fn extract_with(source: &[u8], rules: RustScopeRules) -> Result<FileGraph, ExtractError> {
    let language = tree_sitter::Language::new(tree_sitter_rust::LANGUAGE);
    let tree = parse_with(&language, source)?;

    let mut walk = 순회::new();
    walk.walk(tree.root_node(), source, None);
    walk.impl_을_해소한다();

    // **선언 순회가 끝난 뒤에 스코프를 세운다.** 순서가 규율이다 — 스코프가 심볼 목록을
    // 건드리면 손 표본으로 잰 리콜이 움직인다. 여기서 늘어나는 것은 `ScopeChain` 뿐이고
    // `Symbol` 은 한 자리도 안 바뀐다(`identity` 는 여전히 `grade_of(Rust).identity()` 다).
    let symbol_at: HashMap<usize, LocalIx> = walk
        .symbols
        .iter()
        .enumerate()
        .map(|(i, c)| (c.node.start_byte(), LocalIx(u32::try_from(i).unwrap_or(u32::MAX))))
        .collect();
    let scoped = scopes::build(tree.root_node(), source, &symbol_at, &rules);

    let symbols: Vec<Symbol> = walk
        .symbols
        .iter()
        .map(|c| Symbol {
            name: c.name.clone(),
            kind: c.kind,
            body: BodyDigest::of_normalized(&normalize(c.node, source)),
            // **L1 이라 심볼 단위로도 `ordinal` 이다.** 스코프 체인이 서도 그대로다 —
            // `body_digest` 가 지역 이름을 지우기 시작하면 결박 25 건이 통째로 `stale`
            // 이 되고, 이 회차는 그것을 안 하기로 했다(`C1`·`C2`).
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

    let (mut exports, mut imports) = 표면(tree.root_node(), source);
    // **집합이므로 정렬·중복 제거한다.** 소스 순서에 의존하면 `use` 를 재배열하는 것만으로
    // `export_digest` 가 움직이고, 그것이 의존 파일 전체를 이유 없이 무효화한다(R-05).
    exports.names.sort_unstable();
    exports.names.dedup();
    exports.star_from.sort_unstable();
    exports.star_from.dedup();
    imports.modules.sort_unstable();
    imports.modules.dedup();
    imports.normalize_items();

    // **정렬·중복 제거가 끝난 뒤에 잰다.**
    let export_digest = Capable::Present(exports.digest());
    Ok(FileGraph {
        language: LanguageId::new(Language::Rust.name()),
        grade: crate::grade_of(Language::Rust),
        symbols,
        contains: walk.contains,
        exports: Capable::Present(exports),
        imports: Capable::Present(imports),
        export_digest,
        scopes: Capable::Present(scoped.chain),
        recovery_sites: recovery_sites(tree.root_node()),
    })
}

/// 이 파일이 밖에 노출하는 것과 참조하는 모듈.
///
/// # `pub` 의 뜻을 **하나로** 정한다 — 최상위이고 `pub` 인 것만 (`A13`)
///
/// 셋 중 하나를 골라야 했다.
///
/// | 후보 | 왜 안 골랐나 |
/// |---|---|
/// | 중첩 `pub` 도 담는다 | `stitch_of` 가 **최상위만** EXPORTS 로 옮긴다 — `n.container.is_empty()`(`ledger.rs:377`). 담으면 `export_digest` 와 EXPORTS 가 **서로 다른 모집단**을 재고, 그 어긋남은 화면 어디에도 안 나온다 |
/// | `pub(crate)` 도 담는다 | `ExportSet` 의 뜻이 *"밖에 노출하는 것"* 인데 `pub(crate)` 는 크레이트 밖에 안 나간다. 담으면 실제 노출과 크레이트 안 가시성이 한 집합이 된다 |
///
/// 그래서 **최상위 + `visibility_modifier` 가 정확히 `pub`** 이다.
/// `pub(crate)`·`pub(super)`·`pub(in …)` 과 중첩 `pub` 은 **안 담는다.**
fn 표면(root: Node<'_>, source: &[u8]) -> (ExportSet, ImportSet) {
    let mut exports = ExportSet::default();
    // **`building()` 이지 `default()` 가 아니다** — 이름 임포트가 하나도 없는 파일이
    // *"이 빌드가 안 만든다"* 로 나가지 않게 한다.
    let mut imports = ImportSet::building();

    // **`use` 는 파일 어디에나 있다**(함수 안 · `mod` 안). 전부 훑는다.
    let mut stack = vec![root];
    while let Some(n) = stack.pop() {
        if n.kind() == "use_declaration" {
            if let Some(m) = 모듈_경로(n, source) {
                imports.modules.push(m);
            }
            항목을_담는다(n, source, &mut imports);
            if 최상위인가(n) && 공개인가(n, source) {
                재수출을_담는다(n, source, &mut exports);
            }
        }
        let mut c = n.walk();
        let kids: Vec<Node<'_>> = n.named_children(&mut c).collect();
        drop(c);
        stack.extend(kids);
    }

    let mut c = root.walk();
    let kids: Vec<Node<'_>> = root.named_children(&mut c).collect();
    drop(c);
    for item in kids {
        if item.kind() == "use_declaration" || !공개인가(item, source) {
            continue;
        }
        if let Some(name) = item.child_by_field_name("name") {
            exports.names.push(원문(name, source));
        }
    }
    (exports, imports)
}

/// `source_file` 의 직계 자식인가.
fn 최상위인가(node: Node<'_>) -> bool {
    node.parent().is_some_and(|p| p.kind() == "source_file")
}

/// `visibility_modifier` 가 **정확히 `pub`** 인가.
fn 공개인가(node: Node<'_>, source: &[u8]) -> bool {
    let mut c = node.walk();
    let kids: Vec<Node<'_>> = node.named_children(&mut c).collect();
    drop(c);
    kids.iter().any(|k| k.kind() == "visibility_modifier" && 원문(*k, source) == "pub")
}

/// `pub use` — 이름 재수출과 `*` 재수출을 가른다.
///
/// `pub use a::*;` 이 무슨 이름을 내보내는지는 **그 모듈을 읽어야 알고 그것은 F07** 이다.
/// 그래서 이름이 아니라 대상 모듈로 남는다 — 모르는 것을 안다고 하지 않는다.
fn 재수출을_담는다(node: Node<'_>, source: &[u8], exports: &mut ExportSet) {
    let Some(arg) = node.child_by_field_name("argument") else { return };
    let mut stack = vec![arg];
    while let Some(n) = stack.pop() {
        match n.kind() {
            "use_wildcard" => {
                if let Some(m) = 경로_머리(n, source) {
                    exports.star_from.push(m);
                }
            }
            "use_as_clause" => {
                if let Some(a) = n.child_by_field_name("alias") {
                    exports.names.push(원문(a, source));
                }
            }
            "scoped_identifier" => {
                if let Some(x) = n.child_by_field_name("name") {
                    exports.names.push(원문(x, source));
                }
            }
            "identifier" | "type_identifier" => exports.names.push(원문(n, source)),
            _ => {
                let mut c = n.walk();
                let kids: Vec<Node<'_>> = n.named_children(&mut c).collect();
                drop(c);
                // `use_list` 안의 `path` 는 모듈이라 이름이 아니다.
                stack.extend(kids.into_iter().filter(|k| {
                    n.child_by_field_name("path").map(|p| p.id()) != Some(k.id())
                }));
            }
        }
    }
}

/// `use` 한 줄이 들여오는 **항목**을 전부 담는다 — [`ImportedItem`] 셋을 함께.
///
/// # 접두를 누적해서 내려간다
///
/// 평평하게 훑으면 중첩 목록에서 **실모듈이 뭉개진다** — `use a::{c::C, d::D};` 는
/// [`모듈_경로`] 에게는 모듈 하나(`a`)지만 `C` 의 실모듈은 `a::c` 이고 `D` 의 것은 `a::d` 다.
/// 그래서 목록에 들어갈 때마다 그 목록의 `path` 를 접두에 이어 붙인다.
///
/// # 갈래마다 무엇을 담나
///
/// | 쓴 것 | `module` | `name` | `local` |
/// |---|---|---|---|
/// | `use a::b::C;` | `a::b` | `C` | `C` |
/// | `use a::b::C as D;` | `a::b` | `C` | **`D`** |
/// | `use a::{c::C, d::D};` | `a::c` · `a::d` | `C` · `D` | 같음 |
/// | `use a::b::{self};` | `a` | `b` | `b` |
/// | `use a;` | **빈 문자열** | `a` | `a` |
/// | `use a::*;` | **안 담는다** | | |
///
/// 마지막 둘이 왜 그런지:
///
/// - `use a;` 의 `a` 는 접두가 없다. 빈 모듈은 *"경로 뿌리"* 이고, 그것을 `a` 로 적으면
///   **항목 자신이 자기 모듈이 되어** 해소기가 `a` 안에서 `a` 를 찾는다.
///   [`모듈_경로`] 는 같은 줄을 모듈 `a` 로 적는데 **어긋난 것이 아니다** — 저쪽은
///   *"이 파일이 참조하는 모듈"* 을 헤아리고 이쪽은 *"들여온 항목"* 을 헤아린다.
/// - `use a::*;` 는 무슨 이름이 들어오는지 파일 하나만 보고 모른다. **지어내지 않는다.**
fn 항목을_담는다(node: Node<'_>, source: &[u8], imports: &mut ImportSet) {
    let Some(arg) = node.child_by_field_name("argument") else { return };
    항목_갈래(arg, "", source, imports);
}

/// 경로 접두 하나를 이어 붙인다. 접두가 비면 뒤엣것이 통째로 경로다.
fn 경로를_잇는다(prefix: &str, tail: &str) -> String {
    if prefix.is_empty() { tail.to_owned() } else { format!("{prefix}::{tail}") }
}

/// `a::b::C` 를 `("a::b", "C")` 로 가른다. 세그먼트가 하나면 모듈이 빈 문자열이다.
fn 꼬리를_뗀다(path: &str) -> (String, String) {
    path.rsplit_once("::")
        .map_or_else(|| (String::new(), path.to_owned()), |(m, n)| (m.to_owned(), n.to_owned()))
}

/// `use` 트리 한 마디. `prefix` 는 여기까지 누적된 모듈 경로다.
fn 항목_갈래(node: Node<'_>, prefix: &str, source: &[u8], imports: &mut ImportSet) {
    match node.kind() {
        "identifier" | "type_identifier" => {
            let name = 원문(node, source);
            imports.push_item(ImportedItem {
                module: prefix.to_owned(),
                local: name.clone(),
                name,
                    at: vec![node.start_byte()],
            });
        }
        // `use a::b::{self};` — 들여오는 이름은 접두의 **마지막 세그먼트**다.
        "self" => {
            if !prefix.is_empty() {
                let (module, name) = 꼬리를_뗀다(prefix);
                // ⚠ `use a::{self};` 는 스코프 바인딩이 안 서므로 짝이 안 지어진다 —
                //    `at` 을 이 노드로 두되 그 자리는 오늘까지의 동작 그대로다.
                imports.push_item(ImportedItem {
                    module,
                    local: name.clone(),
                    name,
                        at: vec![node.start_byte()],
                });
            }
        }
        "scoped_identifier" => {
            let Some(name) = node.child_by_field_name("name") else { return };
            let module = node
                .child_by_field_name("path")
                .map_or_else(|| prefix.to_owned(), |p| 경로를_잇는다(prefix, &원문(p, source)));
            let at = vec![name.start_byte()];
            let name = 원문(name, source);
            imports.push_item(ImportedItem { module, local: name.clone(), name, at });
        }
        // `use a::B as C;` — **원본 이름과 부르는 이름이 갈린다.** 둘 다 담는다.
        "use_as_clause" => {
            let (Some(path), Some(alias)) =
                (node.child_by_field_name("path"), node.child_by_field_name("alias"))
            else {
                return;
            };
            let local = 원문(alias, source);
            let at = vec![alias.start_byte()];
            let (module, name) = match path.kind() {
                "self" if !prefix.is_empty() => 꼬리를_뗀다(prefix),
                "scoped_identifier" => {
                    let m = path
                        .child_by_field_name("path")
                        .map_or_else(|| prefix.to_owned(), |p| 경로를_잇는다(prefix, &원문(p, source)));
                    let Some(n) = path.child_by_field_name("name") else { return };
                    (m, 원문(n, source))
                }
                "identifier" | "type_identifier" => (prefix.to_owned(), 원문(path, source)),
                _ => return,
            };
            imports.push_item(ImportedItem { module, name, local, at });
        }
        "scoped_use_list" | "use_list" => {
            let path = node.child_by_field_name("path");
            let 안쪽 = path.map_or_else(|| prefix.to_owned(), |p| 경로를_잇는다(prefix, &원문(p, source)));
            let mut cursor = node.walk();
            let kids: Vec<Node<'_>> = node.named_children(&mut cursor).collect();
            drop(cursor);
            for child in kids {
                // `path` 는 모듈 쪽이라 항목이 아니다 — 이미 접두로 들어갔다.
                if path.map(|p| p.id()) == Some(child.id()) {
                    continue;
                }
                항목_갈래(child, &안쪽, source, imports);
            }
        }
        // `use a::*;` — 무엇이 들어오는지 모른다. **지어내지 않는다.**
        _ => {}
    }
}

/// `use` 한 줄이 가리키는 **모듈**.
///
/// 마지막 세그먼트는 항목 이름이고 그 앞이 모듈이다 — `use a::b::C;` 는 `a::b`.
/// 목록과 `*` 는 앞이 통째로 모듈이다 — `use a::b::{c, d};` 도 `a::b`.
/// `use a;` 처럼 세그먼트가 하나면 그 자체가 모듈이다.
fn 모듈_경로(node: Node<'_>, source: &[u8]) -> Option<String> {
    let arg = node.child_by_field_name("argument")?;
    match arg.kind() {
        "scoped_identifier" => Some(원문(arg.child_by_field_name("path")?, source)),
        "scoped_use_list" => Some(원문(arg.child_by_field_name("path")?, source)),
        "use_wildcard" => 경로_머리(arg, source),
        "use_as_clause" => {
            let path = arg.child_by_field_name("path")?;
            match path.kind() {
                "scoped_identifier" => Some(원문(path.child_by_field_name("path")?, source)),
                _ => Some(원문(path, source)),
            }
        }
        "identifier" => Some(원문(arg, source)),
        _ => None,
    }
}

/// `a::b::*` 의 `a::b`.
fn 경로_머리(wildcard: Node<'_>, source: &[u8]) -> Option<String> {
    let mut c = wildcard.walk();
    let kids: Vec<Node<'_>> = wildcard.named_children(&mut c).collect();
    drop(c);
    kids.first().map(|k| 원문(*k, source))
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
        // `return` 뒤의 진짜 아이템도 안 잰다(표본 작성이 지목한 가장 큰 판단).
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
    fn 매크로는_정의만_잰다() {
        // 규칙 ⑦ — 본문의 `fn $x` 는 선언이 아니라 틀이다.
        let s = 심볼("macro_rules! m { () => { fn generated() {} } }");
        assert_eq!(s, [("m".to_owned(), SymbolKind::Macro)]);
    }

    #[test]
    fn cfg_test_도_잰다() {
        // 소유자 지시 2026-08-20 §3 — *"번복할게 #[cfg(test)] 도 진행해"*.
        // 추출기는 `cfg` 를 해석하지 않는다.
        let s = 심볼("#[cfg(test)]\nmod tests { fn t() {} }");
        assert_eq!(s.len(), 2, "cfg(test) 를 걸렀다");
    }

    #[test]
    fn 종류_열이_다_성립한다() {
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
    fn 본문_없는_trait_시그니처는_안_잰다() {
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
        assert_ne!(top_부모, nested_부모, "동명 타입 둘이 한 컨테이너로 뭉개졌다");
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

    // ── 스코프와 참조 엣지 (`A1`~`A17`) ────────────────────────────────
    //
    // **`file_edges` 를 직접 부른다.** 「스코프 사슬이 이렇게 생겼다」로 재면 해소가
    // 틀린 채로도 초록이 된다 — 사전부검이 A2 를 두고 그것을 지적했다.

    fn 사슬(src: &str) -> pal_core::ScopeChain {
        let g = extract_detailed(src.as_bytes()).unwrap();
        match g.scopes {
            Capable::Present(c) => c,
            Capable::NotBuilt { .. } => panic!("스코프를 안 만들었다"),
        }
    }

    /// `(출발 이름 → 도착 이름)` 쌍 — **엣지 집합 그 자체다.**
    fn 엣지(src: &str) -> Vec<(String, String)> {
        let g = extract_detailed(src.as_bytes()).unwrap();
        let Capable::Present(chain) = &g.scopes else { panic!("스코프를 안 만들었다") };
        let path = pal_core::RepoPath::new("a.rs");
        let nodes: Vec<pal_core::SymbolNode> = g
            .symbols
            .iter()
            .enumerate()
            .map(|(i, s)| pal_core::SymbolNode {
                id: pal_core::SymbolId::compute(
                    &pal_core::RepoId::new("r"),
                    &path,
                    &[],
                    &format!("#{i}"),
                    &pal_core::Discriminator::new(s.kind, 0),
                ),
                path: path.clone(),
                container: Vec::new(),
                name: s.name.clone(),
                kind: s.kind,
                body: s.body,
                span: s.span,
                identity: s.identity,
            })
            .collect();
        let 스냅샷 = pal_core::Snapshot::single(
            pal_core::RepoId::new("r"),
            pal_core::TreeRef::Committed(pal_core::ObjectName::from_bytes([0u8; 20])),
        );
        let pal_core::FileRefs { edges, .. } =
            pal_core::file_edges(&g.symbols, &nodes, chain, &[], &스냅샷);
        let 이름 = |id: pal_core::SymbolId| {
            nodes.iter().position(|n| n.id == id).map(|i| g.symbols[i].name.clone()).unwrap()
        };
        let mut out: Vec<(String, String)> = edges.iter().map(|e| (이름(e.from), 이름(e.to))).collect();
        out.sort();
        out
    }

    /// 이 이름이 `refs` 에 참조로 들어 있나.
    fn 참조에_있나(src: &str, name: &str) -> bool {
        사슬(src).refs.iter().any(|r| r.name == name)
    }

    /// 이 소스의 갈래별 건수 — **임포트 항목을 실제로 넘겨서 잰다.**
    ///
    /// 추출기가 산출한 [`pal_core::ImportSet`] 을 그대로 쓴다. 손으로 지어낸 항목을
    /// 넘기면 *"추출기가 항목을 뽑는다"* 와 *"`file_edges` 가 그것을 쓴다"* 중 뒤엣것만
    /// 재고, 앞엣것이 깨져도 초록이 된다.
    fn 갈래(src: &str) -> pal_core::RefCounts {
        let g = extract_detailed(src.as_bytes()).unwrap();
        let Capable::Present(chain) = &g.scopes else { panic!("스코프를 안 만들었다") };
        let Capable::Present(set) = &g.imports else { panic!("임포트를 안 만들었다") };
        let pal_core::Slot::Built(items) = &set.items else { panic!("항목 축을 안 만들었다") };
        let path = pal_core::RepoPath::new("a.rs");
        let nodes: Vec<pal_core::SymbolNode> = g
            .symbols
            .iter()
            .enumerate()
            .map(|(i, s)| pal_core::SymbolNode {
                id: pal_core::SymbolId::compute(
                    &pal_core::RepoId::new("r"),
                    &path,
                    &[],
                    &format!("#{i}"),
                    &pal_core::Discriminator::new(s.kind, 0),
                ),
                path: path.clone(),
                container: Vec::new(),
                name: s.name.clone(),
                kind: s.kind,
                body: s.body,
                span: s.span,
                identity: s.identity,
            })
            .collect();
        let 스냅샷 = pal_core::Snapshot::single(
            pal_core::RepoId::new("r"),
            pal_core::TreeRef::Committed(pal_core::ObjectName::from_bytes([0u8; 20])),
        );
        pal_core::file_edges(&g.symbols, &nodes, chain, items, &스냅샷).counts
    }

    #[test]
    fn a2_임포트한_이름의_참조가_locals_로_안_샌다() {
        // `Rel` 은 `use inside::Rel;` 로 들어온 이름이고 `n` 은 지역 변수다.
        // **둘이 같은 칸에 들어가면 파일 간 엣지의 분모가 사라진다.**
        let src = "use inside::Rel;
fn f() { let n = 1; let _ = Rel; let _ = n; }
";
        let c = 갈래(src);
        assert_eq!(c.imported, 1, "임포트 참조 `Rel` 이 임포트 갈래에 있어야 한다: {c:?}");
        assert_eq!(c.locals, 1, "지역 변수 참조 `n` 만 지역 갈래에 남아야 한다: {c:?}");
    }

    #[test]
    fn a2_같은_이름의_지역변수는_임포트로_안_세어진다() {
        // **음성 대조** — 이름만으로 가르면 이 지역 변수가 임포트로 세어진다.
        // 그러면 위 시험은 통과하면서 분모가 부풀고, 그 부풀음은 화면에 안 나온다.
        let src = "use inside::Rel;
fn f() { let Rel = 1; let _ = Rel; }
";
        let c = 갈래(src);
        assert_eq!(c.imported, 0, "함수 안의 `Rel` 은 지역 변수다: {c:?}");
        assert_eq!(c.locals, 1, "그 참조는 지역 갈래에 있어야 한다: {c:?}");
    }

    #[test]
    fn a2_임포트가_없으면_임포트_갈래가_0_이다() {
        // **음성 대조** — 이 갈래가 무엇이든 잡아 세면 여기서 0 이 아니게 된다.
        let src = "fn f() { let n = 1; let _ = n; }
";
        let c = 갈래(src);
        assert_eq!(c.imported, 0, "임포트가 없다: {c:?}");
    }

    #[test]
    fn a2_갈래의_합이_참조_수와_같다() {
        // `RefCounts::total` 의 불변식. 갈래를 더하면서 어느 하나가 새면 여기서 갈린다.
        let src = "use inside::Rel;
use other::Root as R;
                   fn f() { let n = 1; let _ = Rel; let _ = R; let _ = n; }
";
        let c = 갈래(src);
        let 참조 = 사슬(src).refs.len();
        assert_eq!(c.total(), 참조, "갈래의 합이 참조 수와 갈렸다: {c:?}");
        assert_eq!(c.imported, 2, "`Rel` 과 별칭 `R` 둘: {c:?}");
    }

    #[test]
    fn a1_스코프_사슬이_성립한다() {
        let c = 사슬("mod m { fn f() { let x = 1; } }");
        assert!(c.scopes.len() >= 3, "스코프가 셋 미만이다: {}", c.scopes.len());
        assert!(!c.refs.is_empty(), "참조가 하나도 안 잡혔다");
    }

    #[test]
    fn a2_impl_이_스코프를_연다() {
        // **엣지 0 이 조건이다** — 사슬 모양만 보면 `hoist_home` 오배치에서도 초록이 된다.
        let src = "struct A; struct B; impl A { fn new(){} } impl B { fn new(){} }";
        assert_eq!(엣지(src), Vec::<(String, String)>::new(), "동명 메서드가 서로를 가리켰다");
        let c = 사슬(src);
        let 스코프: Vec<pal_core::ScopeIx> = c
            .scopes
            .iter()
            .enumerate()
            .filter(|(_, s)| s.bindings.iter().any(|b| b.name == "new"))
            .map(|(i, _)| pal_core::ScopeIx(u32::try_from(i).unwrap()))
            .collect();
        assert_eq!(스코프.len(), 2, "두 `new` 가 한 스코프에 섰다");
        assert_ne!(스코프[0], 스코프[1]);
    }

    #[test]
    fn a3_평범한_호출이_엣지가_된다() {
        assert_eq!(엣지("fn a(){} fn b(){ a(); }"), [("b".to_owned(), "a".to_owned())]);
    }

    #[test]
    fn a4_매크로_안_호출이_엣지가_된다() {
        assert_eq!(
            엣지("fn a()->u32{0} fn b(){ assert_eq!(a(), 0); }"),
            [("b".to_owned(), "a".to_owned())]
        );
    }

    #[test]
    fn a5_매크로_안_멤버_이름은_참조가_아니다() {
        // `token_tree` 안에서는 `field_identifier` 가 안 생긴다 — 앞 형제가 유일한 열쇠다.
        assert!(!참조에_있나("fn f(){ assert!(x.foo()); }", "foo"));
    }

    #[test]
    fn a6_매크로_안_경로_꼬리는_참조가_아니다() {
        assert!(!참조에_있나("fn f(){ matches!(k, S::Var); }", "Var"));
        // **머리는 참조다** — 안 그러면 배제가 너무 넓다.
        assert!(참조에_있나("fn f(){ matches!(k, S::Var); }", "S"));
    }

    #[test]
    fn a7_매크로_밖_경로_꼬리도_참조가_아니다() {
        // ⚠ **선언 자리의 이름도 참조로 실린다**(`file_edges` 가 그것을 거른다).
        //   그래서 「이름이 있나」가 아니라 **몇 자리가 실렸나**로 잰다 — 꼬리를 세면 둘이다.
        let src = "struct A; impl A { fn new()->Self{ Self::new() } }";
        let 자리: Vec<usize> = 사슬(src).refs.iter().filter(|r| r.name == "new").map(|r| r.at).collect();
        assert_eq!(자리.len(), 1, "`Self::new()` 의 꼬리를 참조로 셌다: {자리:?}");
        assert_eq!(자리[0], src.find("fn new").unwrap() + 3, "실린 자리가 선언이 아니다");
    }

    #[test]
    fn a8_use_절_안의_이름은_참조가_아니다() {
        let src = "mod t { use super::*; fn f(){} }";
        assert!(!참조에_있나(src, "super"));
        // `use x::f;` 옆에 `fn f` 가 있어도 그 `f` 는 참조가 아니다.
        assert!(!참조에_있나("use x::f; fn g(){}", "f"));
        // ★★ **이 줄이 이 시험의 유일한 증인이다** (2026-09-08 · 정반합 판 2 · 소유자 결정).
        //   앞의 둘은 `use_절_배제` 를 꺼도 성립한다 — `super` 는 식별자가 아니고
        //   `f` 는 경로 꼬리라 다른 규칙이 먼저 지운다. **규칙을 껐을 때 실제로
        //   늘어나는 이름은 경로 머리 `x` 하나뿐인데 앞의 둘은 `x` 를 안 묻는다.**
        //   그래서 이 시험은 규칙을 꺼도 초록이었다 — 살아 있는 증인이 0 이었다.
        assert!(!참조에_있나("use x::f; fn g(){}", "x"));
    }

    #[test]
    fn a9_필드_식별자는_참조가_아니다() {
        assert!(!참조에_있나("fn f(x: S){ x.foo(); }", "foo"));
    }

    #[test]
    fn a10_타입_이름이_값_자리에서도_해소된다() {
        // 실측 358 건 — 안 넣으면 그만큼 리콜이 빈다.
        assert_eq!(엣지("struct S; fn f(){ let _ = S; }"), [("f".to_owned(), "S".to_owned())]);
    }

    #[test]
    fn a11_use_네_갈래가_각각_무엇을_담나() {
        let 뽑는다 = |src: &str| {
            let g = extract_detailed(src.as_bytes()).unwrap();
            let Capable::Present(i) = g.imports else { panic!("imports 가 안 만들어졌다") };
            i.modules
        };
        assert_eq!(뽑는다("use a::b;"), ["a"], "마지막 세그먼트는 항목 이름이다");
        assert_eq!(뽑는다("use a::{b,c};"), ["a"]);
        assert_eq!(뽑는다("use a as e;"), ["a"], "별칭은 모듈을 안 바꾼다");
        assert_eq!(뽑는다("use a::*;"), ["a"]);
        assert_eq!(뽑는다("use a::b::C;"), ["a::b"]);

        // 스코프에 들어가는 **이름**은 모듈과 다른 축이다.
        let 이름 = |src: &str| {
            let mut v: Vec<String> = 사슬(src).scopes[0]
                .bindings
                .iter()
                .filter(|b| b.namespace == pal_core::Namespace::Value)
                .map(|b| b.name.clone())
                .collect();
            v.sort();
            v.dedup();
            v
        };
        assert_eq!(이름("use a::b;"), ["b"]);
        assert_eq!(이름("use a::{b,c};"), ["b", "c"]);
        assert_eq!(이름("use a as e;"), ["e"]);
        assert_eq!(이름("use a::*;"), Vec::<String>::new(), "별표가 이름을 지어냈다");
    }

    /// 이 소스의 임포트 항목을 `(모듈, 원본, 부르는 이름)` 으로 뽑는다.
    fn 항목(src: &str) -> Vec<(String, String, String)> {
        let g = extract_detailed(src.as_bytes()).unwrap();
        let Capable::Present(i) = g.imports else { panic!("imports 가 안 만들어졌다") };
        let pal_core::Slot::Built(items) = i.items else {
            panic!("Rust 추출기가 항목 축을 안 만들었다")
        };
        items.into_iter().map(|x| (x.module, x.name, x.local)).collect()
    }

    #[test]
    fn a3_잠근_축1_의_임포트_형태_넷이_모듈과_항목으로_갈린다() {
        // ★ 조건 `A3` — 잠근 축1 의 **넷 각각**이다. 하나라도 빠지면 그 형태의 파일 간
        //   엣지가 원리상 서지 않는다.
        let 짝 = |m: &str, n: &str| (m.to_owned(), n.to_owned(), n.to_owned());

        // ① 균일 경로 형제 모듈
        assert_eq!(
            항목("use inside::{Rel, Root};"),
            [짝("inside", "Rel"), 짝("inside", "Root")],
            "형제 모듈의 목록이 안 갈렸다"
        );
        // ② 형제 크레이트 깊은 경로
        assert_eq!(
            항목("use pal_intent::round_condition::ConditionsReport;"),
            [짝("pal_intent::round_condition", "ConditionsReport")],
            "깊은 경로에서 모듈과 항목이 안 갈렸다"
        );
        // ③ 형제 크레이트 평평 경로 — 대상이 그 크레이트 루트의 최상위 `pub`
        assert_eq!(항목("use pal_git::GixRepo;"), [짝("pal_git", "GixRepo")]);
        // ④ `crate::`·`super::`·`self::` 접두 — 2026-09-08 에 범위 안으로 들어왔다
        assert_eq!(항목("use crate::scope::ScopeChain;"), [짝("crate::scope", "ScopeChain")]);
        assert_eq!(항목("use super::Walk;"), [짝("super", "Walk")]);
        assert_eq!(항목("use self::inner::T;"), [짝("self::inner", "T")]);
    }

    #[test]
    fn a4_별칭과_중첩_목록에서_원본_이름과_실모듈이_안_사라진다() {
        // ★ 조건 `A4` — 평평한 `Vec` 하나를 더 붙이면 **정확히 여기서** 깨진다.
        //   별칭은 원본 이름을 지우고 중첩 목록은 실모듈을 `a` 로 뭉갠다.
        assert_eq!(
            항목("use a::Error as IoError;"),
            [("a".to_owned(), "Error".to_owned(), "IoError".to_owned())],
            "별칭이 원본 이름을 지웠다"
        );
        assert_eq!(
            항목("use b::{c::C, d::D};"),
            [
                ("b::c".to_owned(), "C".to_owned(), "C".to_owned()),
                ("b::d".to_owned(), "D".to_owned(), "D".to_owned()),
            ],
            "중첩 목록이 실모듈을 뭉갰다"
        );
        // 둘이 겹친 자리 — 목록 안의 별칭
        assert_eq!(
            항목("use b::{c::C as X, d::D};"),
            [
                ("b::c".to_owned(), "C".to_owned(), "X".to_owned()),
                ("b::d".to_owned(), "D".to_owned(), "D".to_owned()),
            ],
        );
        // `self` — 들여오는 이름은 접두의 마지막 세그먼트다
        assert_eq!(
            항목("use a::b::{self, C};"),
            [
                ("a".to_owned(), "b".to_owned(), "b".to_owned()),
                ("a::b".to_owned(), "C".to_owned(), "C".to_owned()),
            ],
            "`self` 가 접두의 꼬리를 안 집었다"
        );
        // 세그먼트가 하나면 모듈이 **빈 문자열**이다 — 자기 자신이 자기 모듈이 되면 안 된다
        assert_eq!(항목("use serde;"), [(String::new(), "serde".to_owned(), "serde".to_owned())]);
        // 별표는 무슨 이름이 들어오는지 모른다 — **지어내지 않는다**
        assert_eq!(항목("use a::*;"), Vec::<(String, String, String)>::new(), "별표가 항목을 지어냈다");
    }

    #[test]
    fn 항목이_정렬_중복제거_뒤에_나온다() {
        // `modules` 와 같은 계약이다 — 소스 순서에 의존하면 `use` 재배열이 산출을 움직인다.
        assert_eq!(항목("use a::Z; use a::A;"), 항목("use a::A; use a::Z;"));
        assert_eq!(항목("use a::A; use a::A;").len(), 1, "중복이 안 지워졌다");
    }

    #[test]
    fn a12_exports_는_정렬_중복제거_뒤에_요약된다() {
        // 소스 순서 위에서 재면 `pub` 을 재배열하는 것만으로 의존 파일이 무효화된다(R-05).
        let 앞 = extract_detailed(b"pub fn a(){} pub fn b(){}").unwrap();
        let 뒤 = extract_detailed(b"pub fn b(){} pub fn a(){}").unwrap();
        let (Capable::Present(x), Capable::Present(y)) = (&앞.exports, &뒤.exports) else {
            panic!("exports 가 안 만들어졌다")
        };
        assert_eq!(x.names, ["a", "b"]);
        assert_eq!(x, y);
        assert_eq!(앞.export_digest, 뒤.export_digest, "소스 순서가 요약을 움직였다");
        assert!(!x.names.is_empty(), "빈 집합은 「안 만듦」이지 「없다」가 아니다");
    }

    #[test]
    fn a13_pub_의_뜻은_최상위_pub_하나다() {
        // ★ 근거: `stitch_of` 가 **최상위만** EXPORTS 로 옮긴다 —
        //   `n.container.is_empty()` (`crates/pal-cli/src/ledger.rs:377`).
        //   중첩 `pub` 을 담으면 `export_digest` 와 EXPORTS 가 서로 다른 모집단을 잰다.
        let 이름 = |src: &str| {
            let g = extract_detailed(src.as_bytes()).unwrap();
            let Capable::Present(e) = g.exports else { panic!("exports 가 안 만들어졌다") };
            e.names
        };
        assert_eq!(이름("pub fn a(){}"), ["a"]);
        assert_eq!(이름("pub(crate) fn a(){}"), Vec::<String>::new(), "pub(crate) 를 담았다");
        assert_eq!(이름("pub(super) fn a(){}"), Vec::<String>::new());
        assert_eq!(이름("mod m { pub fn a(){} }"), Vec::<String>::new(), "중첩 pub 을 담았다");
        assert_eq!(이름("pub use x::y;"), ["y"], "재수출이 빠졌다");
        let g = extract_detailed(b"pub use x::*;").unwrap();
        let Capable::Present(e) = g.exports else { panic!("exports 가 안 만들어졌다") };
        assert_eq!(e.star_from, ["x"], "별 재수출은 이름이 아니라 대상 모듈이다");
        assert!(e.names.is_empty());
    }

    #[test]
    fn a14_지역_이름이_묶인다() {
        // 다섯 자리 — 파라미터 · `let` · `match` 팔 패턴 · `for x in` · `if let Some(x)`.
        // **밖으로 새면** 동명 최상위 심볼로 해소돼 가짜 엣지가 된다(실측 52~58).
        for (src, 이름) in [
            ("fn p(){} fn f(p: u8){ let _ = p; }", "p"),
            ("fn q(){} fn f(){ let q = 1; let _ = q; }", "q"),
            ("fn r(){} fn f(k: u8){ match k { Some(r) => { let _ = r; }, _ => {} } }", "r"),
            ("fn s(){} fn f(){ for s in 0..3 { let _ = s; } }", "s"),
            ("fn t(){} fn f(){ if let Some(t) = o() { let _ = t; } }", "t"),
        ] {
            assert_eq!(엣지(src), Vec::<(String, String)>::new(), "`{이름}` 이 스코프 밖으로 샜다");
        }
    }

    #[test]
    fn a14_지역이_자기_초기화식을_안_가린다() {
        // ★ **정반합 판 1 의 합(合)이 격리 파일로 실증했다.** `let root = root("x");` 의
        //   초기화식 `root("x")` 는 **바깥의 `fn root`** 다. 자리만으로 재면 그 참조가
        //   자기 자신으로 해소돼 **참 엣지가 조용히 사라진다**(저장소 후보 62 자리).
        let src = "fn root()->u32{0} fn caller()->u32{ let root = root(); root }";
        assert_eq!(엣지(src), [("caller".to_owned(), "root".to_owned())]);
        // **선언문이 끝난 뒤에는 지역이 이긴다** — 섀도잉은 그대로 산다.
        let 뒤 = "fn root()->u32{0} fn caller()->u32{ let root = 1; root }";
        assert_eq!(엣지(뒤), Vec::<(String, String)>::new(), "섀도잉이 안 걸렸다");
    }

    #[test]
    fn a15_속성_안의_이름은_참조가_아니다() {
        // 실측: `#[cfg(test)]` 의 `test` 가 `xtask/src/main.rs` 의 `fn test` 로 59 건 해소됐다.
        assert_eq!(엣지("#[cfg(test)] fn a(){} fn test(){}"), Vec::<(String, String)>::new());
        assert!(!참조에_있나("#[derive(Debug)] struct S;", "Debug"));
    }

    #[test]
    fn a16_impl_메서드_이름이_모듈_스코프로_안_올라간다() {
        // ⚠ 아이템 호이스팅을 `hoist_home` 으로 표현하면 `impl` 스코프를 건너뛰어
        //   A2 가 **무효**가 된다. 그래서 「메서드가 모듈 스코프에 없다」를 직접 잰다.
        let c = 사슬("struct A; impl A { fn m(){} }");
        assert!(
            !c.scopes[0].bindings.iter().any(|b| b.name == "m"),
            "impl 메서드가 모듈 스코프로 올라갔다"
        );
        assert!(
            c.scopes.iter().skip(1).any(|s| s.kind == pal_core::ScopeKind::Impl
                && s.bindings.iter().any(|b| b.name == "m")),
            "impl 스코프에 메서드가 안 들어갔다"
        );
    }

    #[test]
    fn a17_cfg_쌍둥이는_엣지를_안_만든다() {
        // `stitch_of` 가 EXPORTS 에서 하는 것과 같다 —
        // *"둘 이상이면 담지 않는다 — 하나를 고르면 그것이 조용한 오답이다"*.
        let src = "#[cfg(unix)] fn s(){} #[cfg(windows)] fn s(){} fn c(){ s(); }";
        assert_eq!(엣지(src), Vec::<(String, String)>::new(), "쌍둥이 중 하나를 골랐다");
        let c = 사슬(src);
        assert!(
            c.refs.iter().any(|r| r.resolved == pal_core::RefResolution::Ambiguous),
            "모호로 안 적고 조용히 넘어갔다"
        );
    }

    #[test]
    fn a6_매크로_안_구조체_리터럴_필드도_참조가_아니다() {
        // ★ **독립 리뷰 R1 이 격리 저장소에 심어 잡았다.** 실코드의 `Foo { bar: 2 }` 는
        //   `field_identifier` 라 종류로 걸러지는데 매크로 안에서는 벗은 `identifier` 다.
        //   앞 형제가 `{`·`,` 라 앞형제 규칙에 안 걸리고, 같은 파일의 `fn bar` 로
        //   가짜 엣지가 만들어진다.
        let src = "struct S{bar:u32} fn bar()->u32{0} fn t(g:S){ assert_eq!(g, S{bar:2}); }";
        let 엣지들 = 엣지(src);
        assert!(
            !엣지들.iter().any(|(_, to)| to == "bar"),
            "매크로 안 필드명이 `fn bar` 로 가는 엣지가 됐다: {엣지들:?}"
        );
        // **타입 참조는 남는다** — `g: S` 와 `S{…}` 의 머리는 참 엣지다.
        assert!(엣지들.iter().any(|(from, to)| from == "t" && to == "S"));
        // **경로 머리는 그대로 참조다** — `::` 와 `:` 는 다른 마디다.
        assert!(참조에_있나("fn f(){ matches!(k, S::Var); }", "S"));
    }

    #[test]
    fn c1_rust_는_l1_로_잠겨_있다() {
        // ★ **이 회차가 등급을 안 올린다.** 올리면 `body_digest` 가 움직여 결박 25 건이
        //   통째로 `stale` 이 된다. `L2` 로 바꾼 사본에서 이 시험이 빨개진다.
        assert_eq!(crate::grade_of(Language::Rust), ExtractGrade::L1);
        let g = extract_detailed(b"fn f(){}").unwrap();
        assert_eq!(g.grade, ExtractGrade::L1);
        assert_eq!(g.symbols[0].identity, pal_core::IdentityGrade::Ordinal);
    }

    #[test]
    fn b3_세_자리가_전부_present_이고_비지_않았다() {
        let g = extract_detailed(b"use a::b; pub fn f(){ let x = 1; }").unwrap();
        let (Capable::Present(sc), Capable::Present(im), Capable::Present(ex)) =
            (&g.scopes, &g.imports, &g.exports)
        else {
            panic!("세 자리 중 하나가 아직 `not_built` 다")
        };
        assert!(sc.scopes.len() > 1 && !sc.refs.is_empty());
        assert_eq!(im.modules, ["a"]);
        assert_eq!(ex.names, ["f"]);
    }

    #[test]
    fn 속성은_span_에_안_든다() {
        // ⚠ 이것을 바꾸면 `narrative` 의 자리 맵과 주석 인접 판정이 갈린다.
        let g = extract_detailed(b"#[must_use]\nfn f() {}").unwrap();
        let src = "#[must_use]\nfn f() {}";
        assert_eq!(&src[g.symbols[0].span.byte_start..][..2], "fn");
    }
}
