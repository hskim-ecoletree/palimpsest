//! TypeScript 파일 하나의 **스코프 체인과 이름 해소** — L2a (F02 §3.5 · [R-22]).
//!
//! # 왜 선언 순회와 따로 도는가
//!
//! 선언 순회(`typescript::Walk`)는 **무엇이 심볼인가**에 답하고 여기는 **어느 이름이 어느
//! 선언을 가리키는가**에 답한다. 둘을 한 순회에 넣으면 전자의 규칙(모듈 스코프만 · 익명은
//! 심볼이 아니다)이 후자의 규칙(모든 이름이 어딘가에 매인다)과 섞인다 — 그리고 **섞이는
//! 순간 #46 의 리콜 172 개가 움직인다.**
//!
//! 그래서 이 조각은 심볼 목록을 **건드리지 않는다.** 늘어나는 것은 각 심볼의
//! `identity` 와 그것이 정하는 `body_digest` 뿐이다.
//!
//! # 참조로 세는 것과 세지 않는 것
//!
//! | 노드 | 센다 | 왜 |
//! |---|---|---|
//! | `identifier` | ✅ 값 자리 | |
//! | `shorthand_property_identifier` | ✅ 값 자리 | `{ a }` 의 `a` 는 값 참조다 |
//! | `type_identifier` | ✅ 타입 자리 | |
//! | `property_identifier` | ❌ | `obj.foo` 의 `foo` 는 **스코프 참조가 아니다.** 멤버 해소는 L2c 이고 F07 에서도 안 한다 |
//! | `predefined_type` | ❌ | `string`·`number` 는 선언이 아니다 |
//!
//! [R-22]: ../../../docs/plan/00-risks.md#r-22

use std::collections::HashMap;

use pal_core::{
    BoundSymbol, LocalIx, LocalRef, Namespace, ScopeBinding, ScopeChain, ScopeIx, ScopeKind,
};
use tree_sitter::Node;

/// 스코프를 여는 노드들.
const FUNCTION_LIKE: [&str; 8] = [
    "function_declaration",
    "generator_function_declaration",
    "function_expression",
    "generator_function",
    "arrow_function",
    "method_definition",
    "function_signature",
    "method_signature",
];
const CLASS_LIKE: [&str; 4] =
    ["class_declaration", "abstract_class_declaration", "class", "interface_declaration"];
const BRACED: [&str; 5] =
    ["statement_block", "for_statement", "for_in_statement", "catch_clause", "switch_body"];

/// **이름을 정할 수 없는 바인딩 자리.** 하나라도 심볼 안에 있으면 그 심볼은 `ordinal` 이다.
///
/// 구조 분해(`const { a, b } = x`)가 무슨 이름을 묶는지는 패턴을 풀어야 알고, 우리는 풀지
/// 않는다. **모르는 것을 지어내지 않는다** — 그런데 모르면 본문의 어떤 이름이 그것을
/// 가리키는지도 모르고, 그러면 **지우면 안 된다**(R-22).
const UNNAMEABLE_PATTERN: [&str; 4] =
    ["object_pattern", "array_pattern", "rest_pattern", "computed_property_name"];

/// 이 파일의 스코프 체인 + 이름을 못 잡은 자리들.
pub(crate) struct Scoped {
    pub chain: ScopeChain,
    /// 이름을 정하지 못한 바인딩 자리의 바이트 — **심볼 등급이 이것을 본다.**
    pub unnameable: Vec<usize>,
    /// 참조가 일어난 바이트 → [`ScopeChain::refs`] 의 자리. 정규화가 쓴다.
    pub ref_at: HashMap<usize, usize>,
}

/// 파일 하나의 스코프를 세우고 모든 이름 참조를 해소한다.
///
/// `symbol_at` 은 **선언 노드의 시작 바이트 → 심볼 자리**다. 선언 순회가 심볼을 낸 그
/// 노드로 만들어야 하고, 그래야 *"이 바인딩이 심볼이기도 한가"* 가 두 순회에서 같은 답이
/// 된다.
pub(crate) fn build(root: Node<'_>, source: &[u8], symbol_at: &HashMap<usize, LocalIx>) -> Scoped {
    let mut b = Builder {
        source,
        symbol_at,
        chain: ScopeChain::new(),
        unnameable: Vec::new(),
        refs: Vec::new(),
        scope_at: HashMap::new(),
    };
    // **선언을 먼저 전부 모으고 그다음 참조를 푼다.** 한 번에 하면 뒤에 선언된 이름을
    // 참조하는 자리(호이스팅)가 아직 없는 바인딩을 찾게 되고, 그러면 호이스팅이 성립하지
    // 않는다.
    b.declare_pass(root, ScopeIx(0));
    b.reference_pass(root, ScopeIx(0));

    let mut chain = b.chain;
    let mut ref_at = HashMap::with_capacity(b.refs.len());
    b.refs.sort_by_key(|(at, ..)| *at);
    for (at, name, namespace, scope) in b.refs {
        let resolved = chain.resolve(scope, &name, namespace, at);
        ref_at.insert(at, chain.refs.len());
        chain.refs.push(LocalRef { name, namespace, at, resolved });
    }
    Scoped { chain, unnameable: b.unnameable, ref_at }
}

struct Builder<'a, 'm> {
    source: &'a [u8],
    symbol_at: &'m HashMap<usize, LocalIx>,
    chain: ScopeChain,
    unnameable: Vec<usize>,
    /// (바이트, 이름, 이름 공간, 그 자리의 스코프) — 해소는 선언을 다 모은 뒤에 한다.
    refs: Vec<(usize, String, Namespace, ScopeIx)>,
    /// 1 차가 연 스코프 — **노드 신원으로 잡는다.**
    ///
    /// 방문 순서로 맞추면 두 순회가 한 자리만 어긋나도 해소가 통째로 틀리고, **틀린 채로
    /// 조용히 답이 나온다.** `Node::id` 는 한 트리 안에서 안정적이라 그 위험이 없다.
    scope_at: HashMap<usize, ScopeIx>,
}

impl Builder<'_, '_> {
    fn text(&self, node: Node<'_>) -> String {
        String::from_utf8_lossy(&self.source[node.byte_range()]).into_owned()
    }

    /// 이 노드가 스코프를 연다면 그 종류.
    fn opens(node: Node<'_>) -> Option<ScopeKind> {
        let k = node.kind();
        if FUNCTION_LIKE.contains(&k) {
            Some(ScopeKind::Function)
        } else if CLASS_LIKE.contains(&k) {
            Some(ScopeKind::Class)
        } else if BRACED.contains(&k) {
            Some(ScopeKind::Braced)
        } else {
            None
        }
    }

    /// `var` 와 함수 선언이 끌어올려지는 자리 — 가장 가까운 함수 또는 모듈 스코프.
    fn hoist_home(&self, mut scope: ScopeIx) -> ScopeIx {
        loop {
            let Some(s) = self.chain.scopes.get(scope.0 as usize) else { return ScopeIx(0) };
            match s.kind {
                ScopeKind::Function | ScopeKind::Module => return scope,
                _ => match s.parent {
                    pal_core::ScopeParent::Root => return ScopeIx(0),
                    pal_core::ScopeParent::Enclosing(next) => scope = next,
                },
            }
        }
    }

    fn bind(&mut self, scope: ScopeIx, node: Node<'_>, namespace: Namespace, hoisted: bool) {
        if UNNAMEABLE_PATTERN.contains(&node.kind()) {
            self.unnameable.push(node.start_byte());
            return;
        }
        let symbol = self
            .symbol_at
            .get(&node.start_byte())
            .map_or(BoundSymbol::NotASymbol, |ix| BoundSymbol::Symbol(*ix));
        let binding = ScopeBinding {
            name: self.text(node),
            namespace,
            declared_at: node.start_byte(),
            hoisted,
            symbol,
        };
        self.chain.declare(scope, binding);
    }

    /// 선언 노드의 `name` 을 묶는다 — **심볼과 잇는 열쇠는 선언 노드의 시작 바이트다.**
    fn bind_named(
        &mut self,
        scope: ScopeIx,
        decl: Node<'_>,
        namespace: Namespace,
        hoisted: bool,
    ) {
        let Some(name) = decl.child_by_field_name("name") else { return };
        if UNNAMEABLE_PATTERN.contains(&name.kind()) {
            self.unnameable.push(name.start_byte());
            return;
        }
        let symbol = self
            .symbol_at
            .get(&decl.start_byte())
            .map_or(BoundSymbol::NotASymbol, |ix| BoundSymbol::Symbol(*ix));
        let binding = ScopeBinding {
            name: self.text(name),
            namespace,
            // **선언의 자리는 이름 토큰의 자리다.** 선언문 전체의 시작으로 잡으면
            // `const x = x` 같은 자기 참조가 TDZ 를 벗어난다.
            declared_at: name.start_byte(),
            hoisted,
            symbol,
        };
        self.chain.declare(scope, binding);
    }

    // ── 1 차: 선언을 모은다 ────────────────────────────────────────────────
    fn declare_pass(&mut self, node: Node<'_>, scope: ScopeIx) {
        let here = if let Some(kind) = Self::opens(node) {
            let owner = self
                .symbol_at
                .get(&node.start_byte())
                .map_or(BoundSymbol::NotASymbol, |ix| BoundSymbol::Symbol(*ix));
            let inner = self.chain.open(kind, scope, owner);
            self.scope_at.insert(node.id(), inner);
            self.declare_own(node, scope, inner);
            inner
        } else {
            self.declare_plain(node, scope);
            scope
        };
        let mut cursor = node.walk();
        let kids: Vec<Node<'_>> = node.children(&mut cursor).collect();
        drop(cursor);
        for child in kids {
            self.declare_pass(child, here);
        }
    }

    /// 스코프를 여는 노드 자신이 만드는 이름들 — 바깥에 놓을 것과 안에 놓을 것.
    fn declare_own(&mut self, node: Node<'_>, outer: ScopeIx, inner: ScopeIx) {
        let k = node.kind();
        // 함수·클래스의 **이름은 바깥**에 산다. 파라미터·타입 파라미터는 **안**이다.
        if k == "function_declaration"
            || k == "generator_function_declaration"
            || k == "function_signature"
        {
            let home = self.hoist_home(outer);
            self.bind_named(home, node, Namespace::Value, true);
        } else if k == "class_declaration" || k == "abstract_class_declaration" {
            // 클래스는 **두 이름 공간에 다 있다** — `new C()` 와 `x: C` 가 둘 다 선다.
            self.bind_named(outer, node, Namespace::Value, false);
            self.bind_named(outer, node, Namespace::Type, false);
        } else if k == "interface_declaration" {
            // 타입은 끌어올려진다 — 선언보다 앞에서 써도 된다.
            self.bind_named(outer, node, Namespace::Type, true);
        } else if k == "for_in_statement" && node.child_by_field_name("kind").is_some() {
            // `for (const x of xs)` — **`lexical_declaration` 이 아니다.** 문법이 `left`
            // 필드에 이름을 바로 단다. 그리고 이 노드는 스코프를 **여는** 쪽이라
            // `declare_plain` 에 닿지 않는다 — 여기가 그 자리다.
            //
            // 안 잡으면 `x` 의 참조가 파일 밖으로 새거나 **뒤에 선 같은 이름으로**
            // 해소된다. 실물에서 그것이 「선언 전 참조」 거짓 양성 3 건이었다.
            //
            // **`hoisted` 로 둔다** — 루프 변수는 그 루프 전체에서 보이고, 자리로 재면
            // `for (const x of xs)` 의 `x` 자신이 선언 전 참조가 된다.
            if let Some(left) = node.child_by_field_name("left") {
                self.bind(inner, left, Namespace::Value, true);
            }
        }

        let mut cursor = node.walk();
        let kids: Vec<Node<'_>> = node.children(&mut cursor).collect();
        drop(cursor);
        for child in kids {
            match child.kind() {
                "formal_parameters" => self.declare_parameters(child, inner),
                "type_parameters" => self.declare_type_parameters(child, inner),
                _ => {}
            }
        }
    }

    fn declare_parameters(&mut self, params: Node<'_>, scope: ScopeIx) {
        let mut cursor = params.walk();
        let kids: Vec<Node<'_>> = params.named_children(&mut cursor).collect();
        drop(cursor);
        for p in kids {
            // **파라미터는 본문 전체에서 보인다** — 호이스팅과 같은 취급이다.
            match p.child_by_field_name("pattern") {
                Some(pattern) => self.bind(scope, pattern, Namespace::Value, true),
                None => self.bind(scope, p, Namespace::Value, true),
            }
        }
    }

    fn declare_type_parameters(&mut self, params: Node<'_>, scope: ScopeIx) {
        let mut cursor = params.walk();
        let kids: Vec<Node<'_>> = params.named_children(&mut cursor).collect();
        drop(cursor);
        for p in kids {
            self.bind_named(scope, p, Namespace::Type, true);
        }
    }

    /// 스코프를 열지 않는 선언들.
    fn declare_plain(&mut self, node: Node<'_>, scope: ScopeIx) {
        match node.kind() {
            "type_alias_declaration" => self.bind_named(scope, node, Namespace::Type, true),
            "enum_declaration" => {
                self.bind_named(scope, node, Namespace::Value, false);
                self.bind_named(scope, node, Namespace::Type, false);
            }
            // `let`·`const` 는 이 스코프에 갇히고 끌어올려지지 않는다 — **TDZ**.
            "lexical_declaration" => self.declare_declarators(node, scope, false),
            // `var` 는 가장 가까운 함수까지 끌어올려진다.
            "variable_declaration" => {
                let home = self.hoist_home(scope);
                self.declare_declarators(node, home, true);
            }
            "import_statement" => self.declare_imports(node, scope),
            _ => {}
        }
    }

    fn declare_declarators(&mut self, node: Node<'_>, scope: ScopeIx, hoisted: bool) {
        let mut cursor = node.walk();
        let kids: Vec<Node<'_>> = node.named_children(&mut cursor).collect();
        drop(cursor);
        for d in kids {
            if d.kind() == "variable_declarator" {
                self.bind_named(scope, d, Namespace::Value, hoisted);
            }
        }
    }

    /// import 로 들어온 이름도 **이 파일의 선언이다.**
    ///
    /// 안 잡으면 그 이름의 참조가 전부 `OutsideFile` 이 되고, 그러면
    /// 그 값이 *"전역"* 과 *"import"* 를 뭉갠다. 무엇을 가리키는지(어느 파일인지)는
    /// F07 이고, **여기 있다는 사실**은 이 파일만 보고 안다.
    fn declare_imports(&mut self, node: Node<'_>, scope: ScopeIx) {
        let mut stack = vec![node];
        while let Some(n) = stack.pop() {
            match n.kind() {
                "import_specifier" | "namespace_import" => {
                    let name = n.child_by_field_name("alias").or_else(|| n.child_by_field_name("name"));
                    if let Some(x) = name {
                        // `import { type Foo }` 여부를 이 문법에서 값싸게 못 가른다.
                        // **두 공간에 다 놓는다** — 한쪽만 놓으면 나머지 자리의 참조가
                        // 조용히 `OutsideFile` 이 되고, 그것이 곧 틀린 해소다.
                        self.bind(scope, x, Namespace::Value, true);
                        self.bind(scope, x, Namespace::Type, true);
                    } else {
                        let mut c = n.walk();
                        let kids: Vec<Node<'_>> = n.named_children(&mut c).collect();
                        drop(c);
                        stack.extend(kids);
                    }
                }
                "identifier" if n.parent().is_some_and(|p| p.kind() == "import_clause") => {
                    self.bind(scope, n, Namespace::Value, true);
                    self.bind(scope, n, Namespace::Type, true);
                }
                _ => {
                    let mut c = n.walk();
                    let kids: Vec<Node<'_>> = n.named_children(&mut c).collect();
                    drop(c);
                    stack.extend(kids);
                }
            }
        }
    }

    // ── 2 차: 참조를 모은다 ────────────────────────────────────────────────
    fn reference_pass(&mut self, node: Node<'_>, scope: ScopeIx) {
        // 1 차가 이 노드에 열어 둔 스코프를 그대로 따라간다.
        let here = self.scope_at.get(&node.id()).copied().unwrap_or(scope);
        if let Some(namespace) = reference_namespace(node)
            && !in_module_clause(node)
        {
            self.refs.push((node.start_byte(), self.text(node), namespace, here));
        }
        let mut cursor = node.walk();
        let kids: Vec<Node<'_>> = node.children(&mut cursor).collect();
        drop(cursor);
        for child in kids {
            self.reference_pass(child, here);
        }
    }

}

/// 이 이름이 **모듈 절 안**에 있는가 — `import {a as b}` 의 `a`, `export {a}` 의 `a`.
///
/// # 그것은 스코프 참조가 아니다
///
/// `import { preToolUseHandler as legacy } from './x'` 의 `preToolUseHandler` 는 **저쪽
/// 모듈의 export 이름**이지 이 파일의 스코프에서 찾을 이름이 아니다. 참조로 세면 이 파일
/// 어딘가의 같은 이름으로 해소되고, 실물에서 그것이 「선언 전 참조」 거짓 양성이 됐다.
///
/// 그리고 **인덱스 시그니처의 파라미터**(`{ [k: number]: string }` 의 `k`)도 참조가
/// 아니다 — 그 자리에서 이름은 문서일 뿐이고 어떤 선언도 가리키지 않는다.
fn in_module_clause(node: Node<'_>) -> bool {
    let mut cursor = node.parent();
    while let Some(p) = cursor {
        match p.kind() {
            "import_statement" | "export_clause" | "index_signature" => return true,
            // 이름이 붙은 선언 안까지 올라갔으면 더 볼 것 없다.
            "program" | "statement_block" | "class_body" => return false,
            _ => cursor = p.parent(),
        }
    }
    false
}

/// 이 노드가 **스코프 참조**인가 — 그렇다면 어느 이름 공간인가.
fn reference_namespace(node: Node<'_>) -> Option<Namespace> {
    match node.kind() {
        "identifier" | "shorthand_property_identifier" => Some(Namespace::Value),
        "type_identifier" => Some(Namespace::Type),
        _ => None,
    }
}
