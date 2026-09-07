//! **Rust 의 노드 종류 표** — 뼈대는 [`crate::scopes`] 가 진다.
//!
//! # 이 표가 TypeScript 와 갈리는 자리
//!
//! | 물음 | TypeScript | Rust |
//! |---|---|---|
//! | 스코프를 여는 것 | 함수류 8 · 클래스류 4 · 중괄호 5 | `mod_item` · `function_item` · `block` · `closure_expression` · `match_arm` · **`impl_item`** · `for_expression` · `if_expression` |
//! | **무엇이 이름을 선언하나** | `formal_parameters` · `lexical_declaration` · `variable_declaration` · `import_statement` | **하나도 안 겹친다** — `parameters` · `closure_parameters` · `let_declaration` · `match_arm` 의 패턴 · `for_expression` · `let_condition` · `use_declaration` |
//! | 속성 | 없다 | **`attribute_item` 아래는 전부 배제한다** |
//! | 같은 이름이 둘일 때 | 드물다 | **`cfg` 쌍둥이가 실재한다** — 해소하지 않는다 |
//! | 호이스팅 | `function` 은 되고 `let`/`const` 는 TDZ | 아이템은 순서 무관, `let` 은 순서 있다. **TDZ 는 없다** |
//! | 섀도잉 | 드물다 | **관용이다** — [`ResolveRule::Shadowing`] |
//! | 선언 이름의 노드 | `identifier` 계열 | **종류마다 다르다** — `struct_item`·`trait_item` 은 `type_identifier` |
//! | 이름 공간 | 클래스는 값·타입 둘 다 | **타입도 값 자리에서 `identifier` 로 쓰인다**(실측 358 건) — 둘 다에 넣는다 |
//! | 매크로 | 없다 | **`token_tree` 안은 구조가 없다** — 앞 형제 토큰(`.`·`::`)을 보고 거른다 |
//!
//! ★ **마지막 행이 가장 조심할 자리다.** 같은 규칙이 매크로 안팎에서 반대로 돈다 —
//! 안 거르면 같은 파일의 아이템 이름과 겹치는 **509 건**이 가짜 엣지가 된다.
//!
//! # 왜 스위치가 붙어 있는가 — **변형 대조가 완수의 증인이다**
//!
//! [`RustScopeRules`] 의 필드 열은 규칙마다 하나씩이고 [`RustScopeRules::기준`] 이 옳은
//! 조합이다. 규칙 하나를 끈 변형에서 이 저장소를 돌렸을 때 엣지 집합이 **안 달라지면
//! 그 규칙은 아무것도 안 재고 있다** — 사전부검 R2 가 「기여 파일 80 개」라는 옛 증인이
//! 변형 13 개에서 전부 126 을 산출하는 것을 관측했고, 그래서 증인이 수에서 변형 대조로
//! 옮겨졌다(소유자 답 2026-09-08).
//!
//! 도는 자리는 `cargo run -p pal-extract --example scope_variants` 다.

use pal_core::{Namespace, ResolveRule, ScopeIx, ScopeKind};
use tree_sitter::Node;

use crate::scopes::{Builder, ScopeRules};

/// Rust 표. **필드 하나가 규칙 하나이고, 끄면 변형이 된다.**
///
/// `기준` 아닌 값으로 만드는 것은 **변형 대조 전용**이다. 추출기는 [`Self::기준`] 만 쓴다.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RustScopeRules {
    /// `V1` — 파라미터가 이름을 선언하나. 끄면 실측 가짜 엣지 52.
    pub 파라미터_선언: bool,
    /// `V2` — `let`·`match` 팔·`for`·`if let` 의 패턴이 이름을 선언하나. 끄면 실측 58.
    pub 패턴_선언: bool,
    /// `V3` — `impl`·`trait` 본문이 스코프를 여나. 끄면 실측 가짜 78 · 누락 17.
    pub impl_이_연다: bool,
    /// `V4` — 아이템 선언을 `hoist_home` 으로 옮기나. **켜면 `V3` 를 정확히 상쇄한다.**
    pub 아이템을_hoist_home_에: bool,
    /// `V5` — `#[…]` 안의 이름을 배제하나. 끄면 실측 가짜 60(`#[cfg(test)]` 의 `test`).
    pub 속성_배제: bool,
    /// `V6` — 매크로 `token_tree` 안에서 앞 형제가 `.`·`::` 이면 배제하나.
    pub 매크로_앞형제_거르기: bool,
    /// `V7` — `a::b` 의 `b`(경로 꼬리)를 배제하나. 끄면 실측 후보 6,961 이 참조가 된다.
    pub 경로_꼬리_배제: bool,
    /// `V8` — `x.foo` 의 `foo`(`field_identifier`)를 배제하나.
    pub 필드_식별자_배제: bool,
    /// `V9` — `use` 절 안의 이름을 참조에서 배제하나. 끄면 실측 후보 2,601.
    pub use_절_배제: bool,
    /// `V10` — `cfg` 쌍둥이를 조용히 하나로 고르나. **켜면 실측 모호 39 · 선언자리 13.**
    pub cfg_쌍둥이_해소: bool,
}

impl RustScopeRules {
    /// 추출기가 쓰는 조합. **여기 있는 값 하나하나가 완수 조건 `V1`~`V10` 이다.**
    pub const 기준: Self = Self {
        파라미터_선언: true,
        패턴_선언: true,
        impl_이_연다: true,
        아이템을_hoist_home_에: false,
        속성_배제: true,
        매크로_앞형제_거르기: true,
        경로_꼬리_배제: true,
        필드_식별자_배제: true,
        use_절_배제: true,
        cfg_쌍둥이_해소: false,
    };

    /// 아이템 선언이 앉는 자리.
    ///
    /// ⚠ **기준은 `outer` 그대로다.** `hoist_home` 으로 옮기면 [`ScopeKind::Impl`] 을
    /// 건너뛰어 「`impl` 이 스코프를 연다」가 상쇄된다 — 두 변형의 엣지 집합이 비트
    /// 단위로 같았다(사전부검 R2). **Rust 의 「순서 무관」은 자리가 아니라 `hoisted`
    /// 로 표현한다.**
    fn 아이템_자리(self, b: &Builder<'_, '_>, outer: ScopeIx) -> ScopeIx {
        if self.아이템을_hoist_home_에 { b.hoist_home(outer) } else { outer }
    }
}

impl Default for RustScopeRules {
    fn default() -> Self {
        Self::기준
    }
}

/// 조상 중에 이 종류가 있나 — `source_file` 까지 올라간다.
fn 조상에_있나(node: Node<'_>, kinds: &[&str]) -> bool {
    let mut cursor = node.parent();
    while let Some(p) = cursor {
        if kinds.contains(&p.kind()) {
            return true;
        }
        cursor = p.parent();
    }
    false
}

/// 이 노드가 부모의 `field` 필드인가.
fn 필드인가(node: Node<'_>, field: &str) -> bool {
    node.parent()
        .and_then(|p| p.child_by_field_name(field))
        .is_some_and(|f| f.id() == node.id())
}

/// 경로의 꼬리인가 — `a::b` 의 `b`.
///
/// **머리만 참조다.** `b` 가 무엇인지는 `a` 를 풀어야 알고 그것은 F07(L2b)이다.
/// 여기서 세면 같은 파일의 동명 선언으로 해소돼 **조용한 오답**이 된다.
fn 경로_꼬리인가(node: Node<'_>) -> bool {
    node.parent().is_some_and(|p| {
        matches!(p.kind(), "scoped_identifier" | "scoped_type_identifier") && 필드인가(node, "name")
    })
}

/// 매크로 토큰 열 안에서 앞 형제가 `.` 이나 `::` 인가.
///
/// # 같은 규칙이 매크로 안팎에서 반대로 돈다
///
/// 실코드의 `x.foo()` 는 `field_identifier` 를 내고 `S::Var` 는 `scoped_identifier` 를
/// 산출한다 — 문법이 갈라 준다. **`token_tree` 안에서는 둘 다 벗은 `identifier` 다.**
/// 그래서 앞 형제 토큰을 보는 것 말고 가를 길이 없다.
fn 매크로_안_꼬리인가(node: Node<'_>) -> bool {
    if !조상에_있나(node, &["token_tree"]) {
        return false;
    }
    node.prev_sibling().is_some_and(|s| matches!(s.kind(), "." | "::"))
}

/// 선언 이름 토큰이라 참조가 아닌 자리 — enum 변형의 이름.
///
/// **선언하지도 않는다.** `E::A` 는 경로 꼬리라 참조가 아니고, `use E::*` 뒤의 벗은
/// `A` 만 남는데 그것을 위해 변형 이름을 스코프에 넣으면 동명 아이템과 [`cfg` 쌍둥이가
/// 아닌] 모호를 만든다.
///
/// [`cfg` 쌍둥이가 아닌]: pal_core::RefResolution::Ambiguous
fn 변형_이름인가(node: Node<'_>) -> bool {
    node.parent().is_some_and(|p| p.kind() == "enum_variant") && 필드인가(node, "name")
}

impl ScopeRules for RustScopeRules {
    fn rule(&self) -> ResolveRule {
        if self.cfg_쌍둥이_해소 { ResolveRule::Tdz } else { ResolveRule::Shadowing }
    }

    fn opens(&self, node: Node<'_>) -> Option<ScopeKind> {
        match node.kind() {
            // 중첩 `mod` 도 모듈이다 — 한 파일에 `Module` 스코프가 여럿 성립한다.
            "mod_item" => Some(ScopeKind::Module),
            "function_item" | "function_signature_item" | "closure_expression" => {
                Some(ScopeKind::Function)
            }
            "impl_item" | "trait_item" if self.impl_이_연다 => Some(ScopeKind::Impl),
            // `if_expression`·`while_expression` 이 여는 까닭: `if let Some(z) = …` 의
            // `z` 는 **`let_condition` 노드 밖**(consequence 블록)에서 보여야 한다.
            // `let_condition` 에 열면 그 블록이 스코프 밖이라 `z` 가 파일 밖으로 샌다.
            "block" | "match_arm" | "for_expression" | "if_expression" | "while_expression" => {
                Some(ScopeKind::Braced)
            }
            _ => None,
        }
    }

    /// **Rust 에는 없다.** 패턴을 풀 수 있으므로 「이름을 못 정하는 자리」가 안 생긴다.
    fn unnameable(&self, _kind: &str) -> bool {
        false
    }

    fn declare_own(&self, b: &mut Builder<'_, '_>, node: Node<'_>, outer: ScopeIx, inner: ScopeIx) {
        let 자리 = self.아이템_자리(b, outer);
        match node.kind() {
            // 아이템의 이름은 **바깥**이고 순서와 무관하다(`hoisted`).
            "function_item" | "function_signature_item" | "mod_item" => {
                b.bind_named(self, 자리, node, Namespace::Value, true);
            }
            "trait_item" => {
                // 타입도 값 자리에서 `identifier` 로 쓰인다 — **둘 다에 넣는다.**
                b.bind_named(self, 자리, node, Namespace::Value, true);
                b.bind_named(self, 자리, node, Namespace::Type, true);
            }
            "for_expression" => {
                if self.패턴_선언
                    && let Some(p) = node.child_by_field_name("pattern")
                {
                    self.패턴을_묶는다(b, p, inner);
                }
            }
            "match_arm" => {
                if self.패턴_선언
                    && let Some(p) = node.child_by_field_name("pattern")
                {
                    self.패턴을_묶는다(b, p, inner);
                }
            }
            "if_expression" | "while_expression" => {
                if self.패턴_선언
                    && let Some(c) = node.child_by_field_name("condition")
                {
                    self.let_조건을_묶는다(b, c, inner);
                }
            }
            _ => {}
        }

        let mut cursor = node.walk();
        let kids: Vec<Node<'_>> = node.children(&mut cursor).collect();
        drop(cursor);
        for child in kids {
            match child.kind() {
                "parameters" | "closure_parameters" if self.파라미터_선언 => {
                    self.파라미터를_묶는다(b, child, inner);
                }
                "type_parameters" => self.타입_파라미터를_묶는다(b, child, inner),
                _ => {}
            }
        }
    }

    fn declare_plain(&self, b: &mut Builder<'_, '_>, node: Node<'_>, scope: ScopeIx) {
        let 자리 = self.아이템_자리(b, scope);
        match node.kind() {
            // 타입 이름은 **두 이름 공간에 다 있다** — `let _ = S;` 가 성립한다(실측 358 건).
            "struct_item" | "enum_item" | "union_item" | "type_item" | "associated_type" => {
                b.bind_named(self, 자리, node, Namespace::Value, true);
                b.bind_named(self, 자리, node, Namespace::Type, true);
            }
            "const_item" | "static_item" | "macro_definition" => {
                b.bind_named(self, 자리, node, Namespace::Value, true);
            }
            // `let` 은 **자리가 있다** — 그 앞의 같은 이름은 바깥 것이다.
            "let_declaration" => {
                if self.패턴_선언
                    && let Some(p) = node.child_by_field_name("pattern")
                {
                    self.패턴을_묶되_자리를_잰다(b, p, scope);
                }
            }
            "use_declaration" => self.use_를_묶는다(b, node, 자리),
            _ => {}
        }
    }

    fn reference_namespace(&self, node: Node<'_>) -> Option<Namespace> {
        match node.kind() {
            "identifier" => Some(Namespace::Value),
            "type_identifier" => Some(Namespace::Type),
            // **변형 대조 전용 팔이다** — 기준에서 `field_identifier` 는 참조가 아니다.
            "field_identifier" | "shorthand_field_identifier" if !self.필드_식별자_배제 => {
                Some(Namespace::Value)
            }
            _ => None,
        }
    }

    fn skips(&self, node: Node<'_>) -> bool {
        // `'a` 는 `lifetime > identifier` 다 — 안 거르면 `a` 라는 이름의 참조가 된다.
        if 조상에_있나(node, &["lifetime"]) {
            return true;
        }
        // 매크로 **정의**의 본문은 틀이지 참조가 아니다(`rust.rs` 의 규칙 ⑦ 과 같은 자).
        if 조상에_있나(node, &["macro_rule"]) {
            return true;
        }
        if 변형_이름인가(node) {
            return true;
        }
        if self.속성_배제 && 조상에_있나(node, &["attribute_item", "inner_attribute_item"]) {
            return true;
        }
        if self.use_절_배제 && 조상에_있나(node, &["use_declaration"]) {
            return true;
        }
        if self.경로_꼬리_배제 && 경로_꼬리인가(node) {
            return true;
        }
        if self.매크로_앞형제_거르기 && 매크로_안_꼬리인가(node) {
            return true;
        }
        false
    }

    /// **Rust 에는 없다.** 정규화가 지우는 것은 `exact` 심볼뿐이고 Rust 는 전부
    /// `ordinal` 이다(`grade_of(Rust) == L1`).
    fn protects(&self, _node: Node<'_>) -> bool {
        false
    }
}

impl RustScopeRules {
    /// 패턴 안의 이름을 전부 묶는다 — **호이스팅**(그 스코프 전체에서 보인다).
    ///
    /// `match` 팔 · `for` · `if let` 의 패턴이 여기 온다. 그 스코프가 패턴 자신을 담으므로
    /// 자리로 재면 패턴의 이름이 자기보다 앞에 없어 안 보인다.
    fn 패턴을_묶는다(&self, b: &mut Builder<'_, '_>, pattern: Node<'_>, scope: ScopeIx) {
        self.패턴_순회(b, pattern, scope, true);
    }

    /// `let` 의 패턴 — **자리를 잰다.** `let x = 1; let x = f(x);` 의 `f(x)` 는 앞의 `x` 다.
    fn 패턴을_묶되_자리를_잰다(
        &self,
        b: &mut Builder<'_, '_>,
        pattern: Node<'_>,
        scope: ScopeIx,
    ) {
        self.패턴_순회(b, pattern, scope, false);
    }

    fn 패턴_순회(
        &self,
        b: &mut Builder<'_, '_>,
        node: Node<'_>,
        scope: ScopeIx,
        hoisted: bool,
    ) {
        match node.kind() {
            // 경로는 **선언이 아니다** — `Some(x)` 의 `Some` · `S::Var` 의 전부.
            "scoped_identifier" | "scoped_type_identifier" | "field_identifier" => return,
            // `A(x) | B(x)` — **갈래마다 같은 이름을 묶는다**(Rust 가 그것을 요구한다).
            // 전부 묶으면 같은 스코프에 동명 바인딩이 여럿 서고, 그러면 그 이름이 통째로
            // [`pal_core::RefResolution::Ambiguous`] 가 된다 — `cfg` 쌍둥이가 아닌데도.
            // **첫 갈래만 묶는다.**
            "or_pattern" => {
                let mut cursor = node.walk();
                let 첫 = node.named_children(&mut cursor).next();
                drop(cursor);
                if let Some(첫) = 첫 {
                    self.패턴_순회(b, 첫, scope, hoisted);
                }
                return;
            }
            "identifier" => {
                // `Some(x)` 의 `Some` 은 부모의 `type` 필드다.
                if !필드인가(node, "type") {
                    b.bind(self, scope, node, Namespace::Value, hoisted);
                }
                return;
            }
            // `T { a, b: bb }` 의 `a` — 축약 필드는 **이름을 묶는다.**
            "shorthand_field_identifier" => {
                b.bind(self, scope, node, Namespace::Value, hoisted);
                return;
            }
            _ => {}
        }
        let 타입 = node.child_by_field_name("type").map(|t| t.id());
        // ⚠ **`match_pattern` 은 가드를 담는다** — `b if b < K` 의 `b < K` 가 `condition`
        //   필드에 있다. 안 거르면 가드 안의 **참조**가 선언으로 묶여 같은 스코프에
        //   동명 아이템이 여럿 서고, 그 자리가 통째로 `Ambiguous` 가 된다.
        //   실측: 이 저장소에서 그렇게 만들어진 자리가 **62 곳**이었다.
        let 가드 = node.child_by_field_name("condition").map(|t| t.id());
        let mut cursor = node.walk();
        let kids: Vec<Node<'_>> = node.named_children(&mut cursor).collect();
        drop(cursor);
        for child in kids {
            if Some(child.id()) == 타입 || Some(child.id()) == 가드 {
                continue;
            }
            self.패턴_순회(b, child, scope, hoisted);
        }
    }

    /// `if let` · `while let` 의 조건에서 패턴을 찾아 묶는다.
    ///
    /// 조건이 `a && let Some(x) = y` 처럼 겹칠 수 있어 **조건 전체를 훑는다.**
    fn let_조건을_묶는다(&self, b: &mut Builder<'_, '_>, node: Node<'_>, scope: ScopeIx) {
        if node.kind() == "let_condition" {
            if let Some(p) = node.child_by_field_name("pattern") {
                self.패턴을_묶는다(b, p, scope);
            }
            return;
        }
        let mut cursor = node.walk();
        let kids: Vec<Node<'_>> = node.named_children(&mut cursor).collect();
        drop(cursor);
        for child in kids {
            self.let_조건을_묶는다(b, child, scope);
        }
    }

    /// 파라미터 — **본문 전체에서 보인다**(호이스팅과 같은 취급).
    ///
    /// `self` 는 이름이 아니라 토큰(`self_parameter`)이라 여기 안 걸린다.
    fn 파라미터를_묶는다(&self, b: &mut Builder<'_, '_>, params: Node<'_>, scope: ScopeIx) {
        let mut cursor = params.walk();
        let kids: Vec<Node<'_>> = params.named_children(&mut cursor).collect();
        drop(cursor);
        for p in kids {
            if p.kind().contains("comment") || p.kind() == "self_parameter" {
                continue;
            }
            match p.child_by_field_name("pattern") {
                Some(pattern) => self.패턴을_묶는다(b, pattern, scope),
                // 클로저 파라미터는 `pattern` 필드 없이 이름이 바로 온다.
                None => self.패턴을_묶는다(b, p, scope),
            }
        }
    }

    fn 타입_파라미터를_묶는다(&self, b: &mut Builder<'_, '_>, params: Node<'_>, scope: ScopeIx) {
        let mut cursor = params.walk();
        let kids: Vec<Node<'_>> = params.named_children(&mut cursor).collect();
        drop(cursor);
        for p in kids {
            match p.kind() {
                "type_parameter" => b.bind_named(self, scope, p, Namespace::Type, true),
                "const_parameter" => b.bind_named(self, scope, p, Namespace::Value, true),
                // 수명은 이름 공간이 따로다 — 참조에서도 배제하므로 묶지 않는다.
                _ => {}
            }
        }
    }

    /// `use` 로 들어온 이름도 **이 파일의 선언이다.**
    ///
    /// 안 잡으면 그 이름의 참조가 전부 `OutsideFile` 이 되거나, 더 나쁘게는 같은 파일의
    /// 동명 아이템으로 해소돼 **조용한 오답**이 된다.
    ///
    /// 네 갈래를 각각 이렇게 읽는다:
    ///
    /// | 쓴 것 | 묶는 이름 |
    /// |---|---|
    /// | `use a::b;` | `b` |
    /// | `use a::{c, d};` | `c` · `d` |
    /// | `use a as e;` · `use a::b as e;` | `e` |
    /// | `use a::*;` | **없다** — 무엇이 들어오는지 이 파일만 보고 모른다 |
    fn use_를_묶는다(&self, b: &mut Builder<'_, '_>, node: Node<'_>, scope: ScopeIx) {
        let Some(arg) = node.child_by_field_name("argument") else { return };
        self.use_갈래(b, arg, scope);
    }

    fn use_갈래(&self, b: &mut Builder<'_, '_>, node: Node<'_>, scope: ScopeIx) {
        match node.kind() {
            "identifier" | "type_identifier" => {
                // **두 공간에 다 놓는다** — `use x::Foo` 의 `Foo` 가 타입일 수도 값일
                // 수도 있고, 이 파일만 보고 못 가른다.
                b.bind(self, scope, node, Namespace::Value, true);
                b.bind(self, scope, node, Namespace::Type, true);
            }
            "scoped_identifier" => {
                if let Some(name) = node.child_by_field_name("name") {
                    self.use_갈래(b, name, scope);
                }
            }
            "use_as_clause" => {
                if let Some(alias) = node.child_by_field_name("alias") {
                    self.use_갈래(b, alias, scope);
                }
            }
            "scoped_use_list" | "use_list" => {
                let mut cursor = node.walk();
                let kids: Vec<Node<'_>> = node.named_children(&mut cursor).collect();
                drop(cursor);
                for child in kids {
                    // `path` 는 모듈 쪽이라 이름을 안 묶는다.
                    if 필드인가(child, "path") {
                        continue;
                    }
                    self.use_갈래(b, child, scope);
                }
            }
            // `use a::*;` — 무엇이 들어오는지 모른다. **지어내지 않는다.**
            _ => {}
        }
    }
}
