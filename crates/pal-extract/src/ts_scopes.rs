//! **TypeScript 의 노드 종류 표** — 뼈대는 [`crate::scopes`] 가 진다.
//!
//! 2026-09-08 에 `scopes.rs` 에서 갈라 나왔다. **한 줄도 안 고쳤다** — 이 표는 실물에서
//! 사서 넣은 것이고(주석마다 그 사연이 적혀 있다), 가르면서 고치면 무엇이 갈라서
//! 움직인 것인지 알 수 없게 된다. 골든 `ditto.symbols.tsv` 가 그 불변을 잰다.
//!
//! # 참조로 세는 것과 세지 않는 것
//!
//! | 노드 | 잰다 | 왜 |
//! |---|---|---|
//! | `identifier` | ✅ 값 자리 | |
//! | `shorthand_property_identifier` | ✅ 값 자리 | `{ a }` 의 `a` 는 값 참조다 |
//! | `type_identifier` | ✅ 타입 자리 | |
//! | `property_identifier` | ❌ | `obj.foo` 의 `foo` 는 **스코프 참조가 아니다.** 멤버 해소는 L2c 이고 F07 에서도 안 한다 |
//! | `predefined_type` | ❌ | `string`·`number` 는 선언이 아니다 |

use pal_core::{Namespace, ResolveRule, ScopeIx, ScopeKind};
use tree_sitter::Node;

use crate::scopes::{Builder, ScopeRules};

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

/// TypeScript 표. **무상태다.**
pub(crate) struct TypeScriptRules;

impl ScopeRules for TypeScriptRules {
    fn rule(&self) -> ResolveRule {
        ResolveRule::Tdz
    }

    fn opens(&self, node: Node<'_>) -> Option<ScopeKind> {
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

    fn unnameable(&self, kind: &str) -> bool {
        UNNAMEABLE_PATTERN.contains(&kind)
    }

    /// 스코프를 여는 노드 자신이 만드는 이름들 — 바깥에 놓을 것과 안에 놓을 것.
    fn declare_own(&self, b: &mut Builder<'_, '_>, node: Node<'_>, outer: ScopeIx, inner: ScopeIx) {
        let k = node.kind();
        // 함수·클래스의 **이름은 바깥**에 있다. 파라미터·타입 파라미터는 **안**이다.
        if k == "function_declaration"
            || k == "generator_function_declaration"
            || k == "function_signature"
        {
            let home = b.hoist_home(outer);
            b.bind_named(self, home, node, Namespace::Value, true);
        } else if k == "class_declaration" || k == "abstract_class_declaration" {
            // 클래스는 **두 이름 공간에 다 있다** — `new C()` 와 `x: C` 가 둘 다 성립한다.
            b.bind_named(self, outer, node, Namespace::Value, false);
            b.bind_named(self, outer, node, Namespace::Type, false);
        } else if k == "interface_declaration" {
            // 타입은 끌어올려진다 — 선언보다 앞에서 써도 된다.
            b.bind_named(self, outer, node, Namespace::Type, true);
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
                b.bind(self, inner, left, Namespace::Value, true);
            }
        }

        let mut cursor = node.walk();
        let kids: Vec<Node<'_>> = node.children(&mut cursor).collect();
        drop(cursor);
        for child in kids {
            match child.kind() {
                "formal_parameters" => self.declare_parameters(b, child, inner),
                "type_parameters" => self.declare_type_parameters(b, child, inner),
                _ => {}
            }
        }
    }

    /// 스코프를 열지 않는 선언들.
    fn declare_plain(&self, b: &mut Builder<'_, '_>, node: Node<'_>, scope: ScopeIx) {
        match node.kind() {
            "type_alias_declaration" => b.bind_named(self, scope, node, Namespace::Type, true),
            "enum_declaration" => {
                b.bind_named(self, scope, node, Namespace::Value, false);
                b.bind_named(self, scope, node, Namespace::Type, false);
            }
            // `let`·`const` 는 이 스코프에 갇히고 끌어올려지지 않는다 — **TDZ**.
            "lexical_declaration" => self.declare_declarators(b, node, scope, false),
            // `var` 는 가장 가까운 함수까지 끌어올려진다.
            "variable_declaration" => {
                let home = b.hoist_home(scope);
                self.declare_declarators(b, node, home, true);
            }
            "import_statement" => self.declare_imports(b, node, scope),
            _ => {}
        }
    }

    /// 이 노드가 **스코프 참조**인가 — 그렇다면 어느 이름 공간인가.
    fn reference_namespace(&self, node: Node<'_>) -> Option<Namespace> {
        match node.kind() {
            "identifier" | "shorthand_property_identifier" => Some(Namespace::Value),
            "type_identifier" => Some(Namespace::Type),
            _ => None,
        }
    }

    fn skips(&self, node: Node<'_>) -> bool {
        in_module_clause(node)
    }

    fn protects(&self, node: Node<'_>) -> bool {
        node.kind() == "shorthand_property_identifier"
    }
}

impl TypeScriptRules {
    /// **주석은 파라미터가 아니다** (F03-2 · #52).
    ///
    /// tree-sitter 에서 주석은 **이름 있는 노드**라 `named_children` 에 섞여 나온다.
    /// 거르지 않으면 `pattern` 필드가 없는 그 노드가 아래 `None` 팔로 떨어지고,
    /// **주석 한 줄이 통째로 「선언된 이름」이 된다.**
    ///
    /// ditto 실측에서 그런 바인딩이 **32 개**(파일 5)였다. 스코프 표가 오염되는 것만이
    /// 아니라, 그 「이름」이 참조와 우연히 맞으면 **해소가 조용히 틀린다.**
    fn declare_parameters(&self, b: &mut Builder<'_, '_>, params: Node<'_>, scope: ScopeIx) {
        let mut cursor = params.walk();
        let kids: Vec<Node<'_>> = params.named_children(&mut cursor).collect();
        drop(cursor);
        for p in kids {
            if p.kind().contains("comment") {
                continue;
            }
            // **파라미터는 본문 전체에서 보인다** — 호이스팅과 같은 취급이다.
            match p.child_by_field_name("pattern") {
                Some(pattern) => b.bind(self, scope, pattern, Namespace::Value, true),
                None => b.bind(self, scope, p, Namespace::Value, true),
            }
        }
    }

    fn declare_type_parameters(&self, b: &mut Builder<'_, '_>, params: Node<'_>, scope: ScopeIx) {
        let mut cursor = params.walk();
        let kids: Vec<Node<'_>> = params.named_children(&mut cursor).collect();
        drop(cursor);
        for p in kids {
            if p.kind().contains("comment") {
                continue;
            }
            b.bind_named(self, scope, p, Namespace::Type, true);
        }
    }

    fn declare_declarators(
        &self,
        b: &mut Builder<'_, '_>,
        node: Node<'_>,
        scope: ScopeIx,
        hoisted: bool,
    ) {
        let mut cursor = node.walk();
        let kids: Vec<Node<'_>> = node.named_children(&mut cursor).collect();
        drop(cursor);
        for d in kids {
            if d.kind() == "variable_declarator" {
                b.bind_named(self, scope, d, Namespace::Value, hoisted);
            }
        }
    }

    /// import 로 들어온 이름도 **이 파일의 선언이다.**
    ///
    /// 안 잡으면 그 이름의 참조가 전부 `OutsideFile` 이 되고, 그러면
    /// 그 값이 *"전역"* 과 *"import"* 를 뭉갠다. 무엇을 가리키는지(어느 파일인지)는
    /// F07 이고, **여기 있다는 사실**은 이 파일만 보고 안다.
    fn declare_imports(&self, b: &mut Builder<'_, '_>, node: Node<'_>, scope: ScopeIx) {
        let mut stack = vec![node];
        while let Some(n) = stack.pop() {
            match n.kind() {
                "import_specifier" | "namespace_import" => {
                    let name =
                        n.child_by_field_name("alias").or_else(|| n.child_by_field_name("name"));
                    if let Some(x) = name {
                        // `import { type Foo }` 여부를 이 문법에서 값싸게 못 가른다.
                        // **두 공간에 다 놓는다** — 한쪽만 놓으면 나머지 자리의 참조가
                        // 조용히 `OutsideFile` 이 되고, 그것이 곧 틀린 해소다.
                        b.bind(self, scope, x, Namespace::Value, true);
                        b.bind(self, scope, x, Namespace::Type, true);
                    } else {
                        let mut c = n.walk();
                        let kids: Vec<Node<'_>> = n.named_children(&mut c).collect();
                        drop(c);
                        stack.extend(kids);
                    }
                }
                "identifier" if n.parent().is_some_and(|p| p.kind() == "import_clause") => {
                    b.bind(self, scope, n, Namespace::Value, true);
                    b.bind(self, scope, n, Namespace::Type, true);
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
