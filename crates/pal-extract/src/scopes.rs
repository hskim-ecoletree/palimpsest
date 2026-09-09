//! 파일 하나의 **스코프 체인과 이름 해소** — L2a (옛 F02 §3.5 · [R-22]).
//!
//! # 이 파일은 **언어 중립 뼈대**다
//!
//! 2026-09-08 회차에서 갈랐다. 그 전까지 여기 있던 TypeScript 의 노드 종류 표는
//! [`crate::ts_scopes`] 로 나갔고, Rust 표는 [`crate::rust_scopes`] 가 새로 세웠다.
//! **뼈대는 하나이고 표만 언어별로 갈린다** — 소유자 답 2026-09-07(*"TypeScript 가
//! 실물에서 사서 넣은 방어 넷을 Rust 가 그대로 받는다"*).
//!
//! 그 방어 넷이 여기 남는다:
//!
//! 1. **2 패스** — 선언을 다 모으고 나서 참조를 푼다. 한 번에 하면 뒤에 선언된 이름을
//!    참조하는 자리가 아직 없는 바인딩을 찾는다
//! 2. **`scope_at` 을 [`tree_sitter::Node::id`] 로 잡는다** — 방문 순서로 맞추면 두 순회가
//!    한 자리만 어긋나도 해소가 통째로 틀리고, **틀린 채로 조용히 답이 나온다**
//! 3. **`symbol_at` 을 선언 노드의 시작 바이트로 잇는다** — *"이 바인딩이 심볼이기도 한가"*
//!    가 두 순회에서 같은 답이 된다
//! 4. **선언의 자리는 이름 토큰의 자리다** — 선언문 전체의 시작으로 잡으면 `const x = x`
//!    같은 자기 참조가 TDZ 를 벗어난다
//!
//! # 왜 선언 순회와 따로 도는가
//!
//! 선언 순회(`typescript::Walk` · `rust::순회`)는 **무엇이 심볼인가**에 답하고 여기는
//! **어느 이름이 어느 선언을 가리키는가**에 답한다. 둘을 한 순회에 넣으면 전자의
//! 규칙(모듈 스코프만 · 익명은 심볼이 아니다)이 후자의 규칙(모든 이름이 어딘가에
//! 매인다)과 섞인다 — 그리고 **섞이는 순간 #46 의 리콜 172 개가 움직인다.**
//!
//! 그래서 이 조각은 심볼 목록을 **건드리지 않는다.**
//!
//! [R-22]: ../../../docs/plan/00-risks.md#r-22

use std::collections::{HashMap, HashSet};

use pal_core::{
    BoundSymbol, LocalIx, LocalRef, Namespace, ResolveRule, ScopeBinding, ScopeChain, ScopeIx,
    ScopeKind,
};
use tree_sitter::Node;

/// 언어 하나의 **노드 종류 표.** 여섯 물음에 답한다.
///
/// | # | 물음 | 메서드 |
/// |--:|---|---|
/// | 1 | 무엇이 스코프를 여나 | [`opens`] |
/// | 2 | **무엇이 이름을 선언하나** | [`declare_own`] · [`declare_plain`] |
/// | 3 | 무엇을 참조로 세나 (어느 이름 공간인가) | [`reference_namespace`] |
/// | 4 | 무엇을 안 세나 | [`skips`] |
/// | 5 | 호이스팅 규칙 · 해소 규칙 | [`rule`] · `declare_*` 의 `hoisted` 인자 |
/// | 6 | 이름을 정할 수 없는 자리 | [`unnameable`] |
///
/// ★ **둘째 행이 2026-09-08 이전 계획에 없었다.** TypeScript 의 선언 쪽이 뼈대에 박혀
/// 있어서 표에 열이 필요 없어 보였는데, Rust 는 그 마디 이름이 **하나도 안 겹친다** —
/// 사전부검 R2 가 그것을 가짜 엣지 52~58 로 실측했다.
///
/// [`opens`]: ScopeRules::opens
/// [`declare_own`]: ScopeRules::declare_own
/// [`declare_plain`]: ScopeRules::declare_plain
/// [`reference_namespace`]: ScopeRules::reference_namespace
/// [`skips`]: ScopeRules::skips
/// [`rule`]: ScopeRules::rule
/// [`unnameable`]: ScopeRules::unnameable
pub(crate) trait ScopeRules {
    /// 이 언어의 이름 해소 규칙.
    fn rule(&self) -> ResolveRule;

    /// 이 노드가 스코프를 연다면 그 종류.
    fn opens(&self, node: Node<'_>) -> Option<ScopeKind>;

    /// **파일 하나만 보고 이름을 정할 수 없는** 바인딩 자리인가 — 구조 분해 따위.
    fn unnameable(&self, kind: &str) -> bool;

    /// 스코프를 **여는** 노드 자신이 만드는 이름들 — 바깥에 놓을 것과 안에 놓을 것.
    fn declare_own(&self, b: &mut Builder<'_, '_>, node: Node<'_>, outer: ScopeIx, inner: ScopeIx);

    /// 스코프를 **안 여는** 선언들.
    fn declare_plain(&self, b: &mut Builder<'_, '_>, node: Node<'_>, scope: ScopeIx);

    /// 이 노드가 스코프 참조인가 — 그렇다면 어느 이름 공간인가.
    fn reference_namespace(&self, node: Node<'_>) -> Option<Namespace>;

    /// 참조처럼 생겼지만 **세지 않는** 자리인가.
    fn skips(&self, node: Node<'_>) -> bool;

    /// 참조로 세되 **정규화가 지우면 안 되는** 자리인가.
    fn protects(&self, node: Node<'_>) -> bool;

    /// 이 참조가 **경로 호출의 머리**라면 그 꼬리 노드 — `S::foo()` 의 `foo`.
    ///
    /// 꼬리를 참조로 만들지 않고 머리에 실어 보내는 자리다. 까닭은
    /// [`pal_core::LocalRef::tail`] 이 진다 — 꼬리를 참조로 세면 같은 파일의 동명
    /// 선언에 잘못 붙는다.
    ///
    /// **기본값은 [`None`] 이다.** 이 축을 안 만드는 언어는 구현하지 않는다.
    fn call_tail<'t>(&self, _node: Node<'t>) -> Option<Node<'t>> {
        None
    }
}

/// 이 파일의 스코프 체인 + 이름을 못 잡은 자리들.
pub(crate) struct Scoped {
    pub chain: ScopeChain,
    /// 이름을 정하지 못한 바인딩 자리의 바이트 — **심볼 등급이 이것을 본다.**
    pub unnameable: Vec<usize>,
    /// 참조가 일어난 바이트 → [`ScopeChain::refs`] 의 자리. 정규화가 쓴다.
    pub ref_at: HashMap<usize, usize>,
    /// **지우면 안 되는 참조 자리** — 객체 리터럴의 축약 속성(`{ a }` 의 `a`).
    ///
    /// # 참조로 세는 것은 옳다. 지우는 것이 틀리다
    ///
    /// `{ a }` 의 `a` 는 값 참조가 맞고 해소도 되어야 한다. 그런데 그 자리는 **동시에
    /// 밖에서 보이는 키 이름**이다(옛 F03 §4.2). 지우면
    ///
    /// ```text
    /// function f() { const alpha = 1; return { alpha }; }
    /// function f() { const beta  = 1; return { beta  }; }
    /// ```
    ///
    /// 둘이 **같은 요약**을 갖는다 — 만들어 산출하는 객체의 키가 다른데도.
    /// [R-22] 가 경고한 *"서로 다른 코드가 같은 digest"* 의 정확한 형태다.
    pub protected: HashSet<usize>,
}

/// 파일 하나의 스코프를 세우고 모든 이름 참조를 해소한다.
///
/// `symbol_at` 은 **선언 노드의 시작 바이트 → 심볼 자리**다. 선언 순회가 심볼을 산출한 그
/// 노드로 만들어야 하고, 그래야 *"이 바인딩이 심볼이기도 한가"* 가 두 순회에서 같은 답이
/// 된다.
pub(crate) fn build(
    root: Node<'_>,
    source: &[u8],
    symbol_at: &HashMap<usize, LocalIx>,
    rules: &dyn ScopeRules,
) -> Scoped {
    let mut b = Builder {
        source,
        symbol_at,
        chain: ScopeChain::new(),
        unnameable: Vec::new(),
        refs: Vec::new(),
        scope_at: HashMap::new(),
        protected: HashSet::new(),
    };
    // **선언을 먼저 전부 모으고 그다음 참조를 푼다.** 한 번에 하면 뒤에 선언된 이름을
    // 참조하는 자리(호이스팅)가 아직 없는 바인딩을 찾게 되고, 그러면 호이스팅이 성립하지
    // 않는다.
    b.declare_pass(rules, root, ScopeIx(0));
    b.reference_pass(rules, root, ScopeIx(0));

    let mut chain = b.chain;
    let mut ref_at = HashMap::with_capacity(b.refs.len());
    b.refs.sort_by_key(|(at, ..)| *at);
    let rule = rules.rule();
    for (at, name, namespace, scope, tail) in b.refs {
        let resolved = chain.resolve_with(scope, &name, namespace, at, rule);
        ref_at.insert(at, chain.refs.len());
        chain.refs.push(LocalRef { name, namespace, at, resolved, tail });
    }
    Scoped { chain, unnameable: b.unnameable, ref_at, protected: b.protected }
}

pub(crate) struct Builder<'a, 'm> {
    source: &'a [u8],
    symbol_at: &'m HashMap<usize, LocalIx>,
    chain: ScopeChain,
    unnameable: Vec<usize>,
    /// (바이트, 이름, 이름 공간, 그 자리의 스코프) — 해소는 선언을 다 모은 뒤에 한다.
    refs: Vec<(usize, String, Namespace, ScopeIx, Option<String>)>,
    /// 객체 리터럴 축약 속성의 바이트 — **정규화가 이 자리를 지우면 안 된다.**
    protected: HashSet<usize>,
    /// 1 차가 연 스코프 — **노드 신원으로 잡는다.**
    ///
    /// 방문 순서로 맞추면 두 순회가 한 자리만 어긋나도 해소가 통째로 틀리고, **틀린 채로
    /// 조용히 답이 나온다.** `Node::id` 는 한 트리 안에서 안정적이라 그 위험이 없다.
    scope_at: HashMap<usize, ScopeIx>,
}

impl Builder<'_, '_> {
    pub(crate) fn text(&self, node: Node<'_>) -> String {
        String::from_utf8_lossy(&self.source[node.byte_range()]).into_owned()
    }

    /// `var` 와 함수 선언이 끌어올려지는 자리 — 가장 가까운 함수 또는 모듈 스코프.
    ///
    /// ⚠ **이것은 TypeScript 의 장치다.** Rust 의 아이템 호이스팅을 이것으로 표현하면
    /// [`ScopeKind::Impl`] 을 **건너뛰어** 「`impl` 이 스코프를 연다」를 정확히 상쇄한다 —
    /// 사전부검 R2 가 두 변형의 엣지 집합이 비트 단위로 같은 것을 관측했다(가짜 78 ·
    /// 누락 17). Rust 는 [`ScopeBinding::hoisted`] 만 쓰고 자리는 안 옮긴다.
    pub(crate) fn hoist_home(&self, mut scope: ScopeIx) -> ScopeIx {
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

    fn symbol_of(&self, at: usize) -> BoundSymbol {
        self.symbol_at.get(&at).map_or(BoundSymbol::NotASymbol, |ix| BoundSymbol::Symbol(*ix))
    }

    /// 이름 토큰 하나를 묶는다.
    pub(crate) fn bind(
        &mut self,
        rules: &dyn ScopeRules,
        scope: ScopeIx,
        node: Node<'_>,
        namespace: Namespace,
        hoisted: bool,
    ) {
        self.bind_visible_from(rules, scope, node, namespace, hoisted, 0);
    }

    /// [`Self::bind`] 인데 **보이기 시작하는 바이트**를 따로 준다.
    ///
    /// `let x = x(…)` 의 초기화식이 자기 자신을 보지 않게 하는 자리다 —
    /// [`ScopeBinding::visible_from`] 의 문서가 그 사연을 진다.
    pub(crate) fn bind_visible_from(
        &mut self,
        rules: &dyn ScopeRules,
        scope: ScopeIx,
        node: Node<'_>,
        namespace: Namespace,
        hoisted: bool,
        visible_from: usize,
    ) {
        if rules.unnameable(node.kind()) {
            self.unnameable.push(node.start_byte());
            return;
        }
        let symbol = self.symbol_of(node.start_byte());
        let binding = ScopeBinding {
            name: self.text(node),
            namespace,
            declared_at: node.start_byte(),
            visible_from: visible_from.max(node.start_byte()),
            hoisted,
            symbol,
        };
        self.chain.declare(scope, binding);
    }

    /// 선언 노드의 `name` 을 묶는다 — **심볼과 잇는 열쇠는 선언 노드의 시작 바이트다.**
    pub(crate) fn bind_named(
        &mut self,
        rules: &dyn ScopeRules,
        scope: ScopeIx,
        decl: Node<'_>,
        namespace: Namespace,
        hoisted: bool,
    ) {
        let Some(name) = decl.child_by_field_name("name") else { return };
        if rules.unnameable(name.kind()) {
            self.unnameable.push(name.start_byte());
            return;
        }
        let symbol = self.symbol_of(decl.start_byte());
        let binding = ScopeBinding {
            name: self.text(name),
            namespace,
            // **선언의 자리는 이름 토큰의 자리다.** 선언문 전체의 시작으로 잡으면
            // `const x = x` 같은 자기 참조가 TDZ 를 벗어난다.
            declared_at: name.start_byte(),
            visible_from: name.start_byte(),
            hoisted,
            symbol,
        };
        self.chain.declare(scope, binding);
    }

    // ── 1 차: 선언을 모은다 ────────────────────────────────────────────────
    fn declare_pass(&mut self, rules: &dyn ScopeRules, node: Node<'_>, scope: ScopeIx) {
        let here = if let Some(kind) = rules.opens(node) {
            let owner = self.symbol_of(node.start_byte());
            let inner = self.chain.open(kind, scope, owner);
            self.scope_at.insert(node.id(), inner);
            rules.declare_own(self, node, scope, inner);
            inner
        } else {
            rules.declare_plain(self, node, scope);
            scope
        };
        let mut cursor = node.walk();
        let kids: Vec<Node<'_>> = node.children(&mut cursor).collect();
        drop(cursor);
        for child in kids {
            self.declare_pass(rules, child, here);
        }
    }

    // ── 2 차: 참조를 모은다 ────────────────────────────────────────────────
    fn reference_pass(&mut self, rules: &dyn ScopeRules, node: Node<'_>, scope: ScopeIx) {
        // 1 차가 이 노드에 열어 둔 스코프를 그대로 따라간다.
        let here = self.scope_at.get(&node.id()).copied().unwrap_or(scope);
        if let Some(namespace) = rules.reference_namespace(node)
            && !rules.skips(node)
        {
            // **참조로 세되 지우지는 않는다** — 위 `Scoped::protected` 의 이유.
            if rules.protects(node) {
                self.protected.insert(node.start_byte());
            }
            let tail = rules.call_tail(node).map(|t| self.text(t));
            self.refs.push((node.start_byte(), self.text(node), namespace, here, tail));
        }
        let mut cursor = node.walk();
        let kids: Vec<Node<'_>> = node.children(&mut cursor).collect();
        drop(cursor);
        for child in kids {
            self.reference_pass(rules, child, here);
        }
    }
}
