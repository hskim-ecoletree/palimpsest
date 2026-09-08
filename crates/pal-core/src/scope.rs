//! 파일 **안**의 스코프 체인 — L2a ([R-22](../../../docs/plan/00-risks.md#r-22)).
//!
//! # 왜 이것이 F07 이 아니라 F02 에 있는가
//!
//! F03 의 `body_digest` 정규화가 이것 없이는 성립하지 않는다. 지역 변수·파라미터명을
//! 지우려면 **어느 이름이 어느 선언을 가리키는지** 알아야 하고, 그것을 P1 까지 미루면
//! P0 에서 만든 좌표와 digest 가 F07 완료일에 **전부 이동한다.** 그 사이에 쌓인 결박은
//! 전부 `orphaned` 가 된다(옛 F02 §3.5).
//!
//! 그리고 이 연산은 **파일 하나만 본다.** 1층의 성질을 깨지 않는다 — 완전 병렬이고
//! 콘텐츠 주소 캐시의 값이 될 수 있다. F07 에 남는 것은 파일 **간** 연산(L2b 모듈 해소 ·
//! L2c 멤버)뿐이다.
//!
//! # 이름이 `ScopeTable` 이 아닌 이유
//!
//! 옛 F02 §2·§3.5 는 `ScopeTable` 로 적었다. **`table` 이 `pal-core` 의 금지어 16 개에
//! 있고**(stack §4.2 · `mutable`·`immutable`·`portable` 을 부분 문자열로 잡기 위한
//! 것이다) `cargo xtask check` 가 코드 어휘를 부분 문자열로 검사한다. `Ledger` 가
//! `languages` 로 피한 것과 같은 자리이고, 여기서는 [`ScopeChain`] 으로 간다.
//!
//! `ScopeKind::Block` 도 같은 이유로 쓰지 않는다 — `block` 이 금지어다. [`ScopeKind::Braced`].
//!
//! # 여기 있는 `Binding` 은 **결박이 아니다**
//!
//! [`crate::Binding`] 은 문서와 코드를 잇는 **결박**이고, [`ScopeBinding`] 은 *"이 스코프에
//! 이 이름이 선언되어 있다"* 는 언어의 사실이다. 영어 낱말이 겹칠 뿐 다른 것이라 접두어로
//! 갈라 둔다.

use serde::{Deserialize, Serialize};

use crate::file_graph::LocalIx;

/// [`ScopeChain::scopes`] 안의 자리.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ScopeIx(pub u32);

/// 스코프가 무엇으로 열렸나 — **무엇을 담을 수 있는지가 여기서 갈린다.**
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeKind {
    /// 파일 하나 = 모듈 하나. 최상위 선언이 여기 산다.
    Module,
    /// 함수 본문 — **파라미터가 여기 산다.** `var` 와 함수 선언이 여기까지 끌어올려진다.
    Function,
    /// 클래스 본문 — 타입 파라미터와 멤버. 멤버 해소(L2c)는 **F07 이고 P1 에서도 안 한다**.
    ///
    /// ⚠ **TypeScript 의 `class` 다.** Rust 의 `impl` 을 여기 얹지 않는다 — 그러면 1층
    /// 캐시에 실리는 이 값의 뜻이 언어마다 갈린다(사전부검 R2). Rust 는 [`ScopeKind::Impl`].
    Class,
    /// `impl` 또는 `trait` 의 본문 하나 — **Rust 전용.**
    ///
    /// 이름이 `Impl` 인데 `trait` 도 여는 까닭: 둘이 담는 것이 같다 — **연관 항목**
    /// (메서드 · 연관 상수 · 연관 타입)이고, 그 이름은 바깥 모듈로 안 올라간다.
    /// 두 값으로 가르면 `hoist_home` 류의 분기가 둘을 같은 자리로 다루는지 매번
    /// 물어야 하고, 지금 그 답은 언제나 「같게」다.
    ///
    /// # 왜 [`ScopeKind::Class`] 를 재사용하지 않는가
    ///
    /// `impl A { fn new(){} }` 와 `impl B { fn new(){} }` 의 두 `new` 는 **서로 다른
    /// 스코프에 서야 한다.** 안 그러면 [`ScopeChain::resolve`] 가 둘째 `new` 의 선언
    /// 이름을 첫째로 해소하고, 그것이 그대로 가짜 REFERENCES 엣지가 된다(실측: 이
    /// 저장소 `.rs` 138 파일에서 `impl` 이 스코프를 안 열면 동명 재선언이
    /// **83 건** · 파일 30 이고, 열면 21 건 · 파일 10 으로 준다).
    ///
    /// 이름을 갈라 두는 까닭은 **뜻이 다르기 때문**이다. `Class` 는 *"타입 파라미터와
    /// 멤버를 담는 자리"* 이고 `Impl` 은 *"한 타입에 붙인 구현 하나"* 다. 한 파일에
    /// 여럿 서고, 서로를 못 본다.
    Impl,
    /// 중괄호 하나. `let`·`const`·`class` 가 여기 갇히고 `var` 는 안 갇힌다.
    ///
    /// **이름이 `Block` 이 아니다** — `block` 이 `pal-core` 의 금지어다(모듈 주석).
    Braced,
}

/// 이름 공간 둘 — **뭉개면 해소가 조용히 틀린다.**
///
/// ⚠ **Rust 의 셋째 공간(매크로)을 못 담는다.** `macro_rules! m` 과 `fn m` 은 Rust 에서
/// 공존하는데 이 타입은 둘을 같은 `Value` 로 본다. ADR-0027 §② 가 등록한 금지역이고
/// 2026-09-08 개정이 *"남아 있다 — 이 저장소에서 모집단이 0"* 으로 적었다. 승격은 #133.
///
/// `interface Foo` 와 `const Foo` 는 공존한다(옛 F02 §3.5). 한 공간으로 뭉개면 둘 중 하나가
/// 다른 하나를 가리고, 그러면 `Foo` 를 타입 자리에서 쓴 참조가 **값 선언으로 해소된다.**
/// 틀린 해소는 틀린 정규화이고 틀린 정규화는 **서로 다른 코드가 같은 digest** 다(R-22).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Namespace {
    /// 값 자리 — 함수 · 클래스 · enum · 변수 · 파라미터.
    Value,
    /// 타입 자리 — 인터페이스 · 타입 별칭 · 클래스 · enum · 타입 파라미터.
    Type,
}

/// 이 스코프에 선언된 이름 하나.
///
/// **[`crate::Binding`](결박)이 아니다** — 모듈 주석.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeBinding {
    pub name: String,
    pub namespace: Namespace,
    /// 선언이 시작하는 바이트.
    ///
    /// **이 값 하나가 TDZ 를 판정한다** — `let`/`const` 는 이보다 앞에서 참조될 수 없다.
    pub declared_at: usize,
    /// 이 이름이 **어느 바이트부터 보이는가**. 대개 [`Self::declared_at`] 과 같다.
    ///
    /// # 갈라야 하는 자리가 하나 있다 — `let x = x(…)`
    ///
    /// Rust 의 `let root = root("x");` 에서 초기화식의 `root` 는 **바깥의 그것**이다.
    /// 그런데 선언의 자리는 이름 토큰이고 그것이 초기화식보다 **앞**이라, 자리만으로
    /// 재면 그 참조가 자기 자신으로 해소돼 **참 엣지가 조용히 사라진다**(실측: 이
    /// 저장소에 후보 62 자리 · 정반합 판 1 의 합(合)이 격리 파일로 재현했다).
    ///
    /// 그래서 `let` 은 이 값을 **선언문이 끝나는 바이트**로 둔다. 두 값을 갈라 두는
    /// 까닭: [`Self::declared_at`] 은 `file_edges` 가 **선언 자리 그 자체**를 거르는 데
    /// 쓰고, 그 자는 이름 토큰이어야 한다.
    pub visible_from: usize,
    /// 쓰이는 자리보다 **뒤에 있어도 해소되는가**. 함수 선언과 `var` 가 참이다.
    ///
    /// 거짓인 이름을 선언 전에 참조하면 그것은 TDZ 이고 [`RefResolution::BeforeDeclaration`]
    /// 이다. **그것을 해소해 버리면 이것은 스코프 체인이 아니라 이름 표다**(옛 DESIGN §5.1).
    pub hoisted: bool,
    /// 이 이름이 심볼이기도 한가 — 최상위 선언은 그렇고 지역 변수·파라미터는 아니다.
    pub symbol: BoundSymbol,
}

/// [`ScopeBinding`] 이 심볼과 이어지는가.
///
/// `Option` 을 쓰지 않는다 — stack §5.4 는 직렬화되는 도메인 값에서 `Option` 을 금한다.
/// *"심볼이 아니다"* 는 조회 실패가 아니라 **사실**이다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundSymbol {
    /// 이 이름은 심볼이다 — [`crate::FileGraph::symbols`] 의 자리.
    Symbol(LocalIx),
    /// 심볼이 아니다 — 지역 변수 · 파라미터 · 타입 파라미터. **세면 폭발한다**(옛 F02 §3.3).
    NotASymbol,
}

/// 스코프 하나.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Scope {
    pub kind: ScopeKind,
    pub parent: ScopeParent,
    pub bindings: Vec<ScopeBinding>,
    /// 이 스코프를 연 심볼 — 없으면 [`BoundSymbol::NotASymbol`].
    ///
    /// **정규화가 이것을 쓴다.** 어떤 이름이 *"이 심볼의 지역"* 인지는 그 이름의 스코프가
    /// 그 심볼 안에 있는가로 정해진다.
    pub owner: BoundSymbol,
}

/// 스코프의 바깥.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeParent {
    /// 모듈 스코프. 이 위는 파일 밖이다.
    Root,
    Enclosing(ScopeIx),
}

/// 파일 안에서 일어난 이름 참조 하나 — **해소 결과와 함께.**
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalRef {
    pub name: String,
    pub namespace: Namespace,
    /// 참조가 일어난 바이트.
    pub at: usize,
    pub resolved: RefResolution,
}

/// 이름 하나가 어디로 해소됐나.
///
/// **넷을 가르는 것이 이 타입의 전부다.** 뭉개면 *"파일 밖의 이름"*(정상)과
/// *"선언 전 참조"*(TDZ)와 *"후보가 둘이라 안 고른다"*(모호)가 같은 출력이 되고,
/// 그러면 해소율이 무엇을 세는지 알 수 없다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefResolution {
    /// 이 파일 안의 선언으로 해소됐다.
    Bound { scope: ScopeIx, binding: u32 },
    /// 이 파일 안에 그 이름의 선언이 없다 — import · 전역(`console`·`Promise`).
    ///
    /// **실패가 아니다.** 파일 하나만 보는 연산에서 이것은 정확한 답이고, 푸는 것은
    /// F07(L2b)이다. 실패로 세면 해소율이 *"이 파일이 얼마나 자족적인가"* 를 재게 된다.
    OutsideFile,
    /// 선언보다 앞에서 참조했다 — `let`/`const` 의 TDZ.
    ///
    /// **해소하지 않는 것이 옳다.** 해소해 버리면 스코프 체인이 아니라 이름 표다.
    BeforeDeclaration,
    /// 같은 스코프에 그 이름의 선언이 **둘 이상**이고 자리로 못 가른다.
    ///
    /// # 하나를 고르면 그것이 조용한 오답이다
    ///
    /// Rust 의 `cfg` 쌍둥이가 실물이다 — `#[cfg(unix)] fn spawn_child` 와
    /// `#[cfg(windows)] fn spawn_child` 가 같은 파일 같은 스코프에 함께 놓인다. 추출기는
    /// `cfg` 를 해석하지 않으므로(그것이 결정이다) **어느 쪽인지 모른다.**
    ///
    /// 같은 저장소가 EXPORTS 에서 이미 그렇게 한다 — `stitch_of` 가
    /// *"둘 이상이면 담지 않는다 — 하나를 고르면 그것이 조용한 오답이다"*.
    /// 여기서 고르면 Windows 에서 도는 코드가 unix 선언을 가리키는 엣지가 나온다.
    ///
    /// **[`Self::OutsideFile`] 과 뭉개지 않는다** — 저쪽은 *"이 파일에 없다"* 이고
    /// 이쪽은 *"이 파일에 너무 많다"* 다. 뭉개면 F07 이 풀 수 있는 것과 원리상 못 푸는
    /// 것이 한 숫자가 된다.
    Ambiguous,
}

/// 이름 하나를 어느 규칙으로 찾는가 — **언어마다 다르다.**
///
/// # 왜 규칙이 값인가
///
/// 뼈대는 하나이고([`ScopeChain`]) 그 위의 규칙이 언어마다 갈린다. 규칙을 코드에 박으면
/// 뒤에 온 언어가 앞 언어의 규칙을 조용히 물려받는다 — 실측: Rust 가 TypeScript 의 TDZ
/// 방어를 그대로 받으면 비함수 스코프의 전방 참조 **52 건**이 「선언 전 참조」로 뒤집힌다.
/// Rust 아이템에는 TDZ 가 없다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResolveRule {
    /// **TypeScript.** 같은 스코프의 동명 선언 중 가장 앞선 것 · TDZ 가 있고 바깥으로
    /// 안 나간다 · 함수 경계를 지나면 자리를 비교하지 않는다.
    Tdz,
    /// **Rust.** 섀도잉이 관용이라 **보이는 것 중 가장 뒤**를 쓴다 · TDZ 가 없어 아직
    /// 안 선 이름은 바깥에서 찾는다 · 자리로 못 가르는 동명 아이템이 둘이면
    /// [`RefResolution::Ambiguous`].
    Shadowing,
}

/// 파일 하나의 스코프 체인.
///
/// **`scopes[0]` 이 파일의 모듈 스코프다.** 비어 있는 체인은 만들지 않는다 — 파일이
/// 있으면 모듈 스코프는 있다.
///
/// ⚠ **`Module` 인 스코프가 하나라는 뜻은 아니다.** Rust 의 중첩 `mod` 가 각각
/// [`ScopeKind::Module`] 을 연다(2026-09-08 · #130). `scopes[0]` 이 **뿌리**라는 것만
/// 불변이고, 그것은 [`ScopeParent::Root`] 가 진다.
///
/// # 언어마다 다른 것은 규칙이지 이 타입이 아니다
///
/// 이 타입은 *"어느 스코프에 어떤 이름이 선언돼 있나"* 만 담는다. **찾는 규칙**은
/// [`ResolveRule`] 이 값으로 지고 [`Self::resolve_with`] 가 그것을 받는다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeChain {
    pub scopes: Vec<Scope>,
    /// 이 파일 안에서 일어난 이름 참조 — **소스 순서.**
    pub refs: Vec<LocalRef>,
}

impl ScopeChain {
    /// 모듈 스코프 하나만 있는 체인.
    #[must_use]
    pub fn new() -> Self {
        Self {
            scopes: vec![Scope {
                kind: ScopeKind::Module,
                parent: ScopeParent::Root,
                bindings: Vec::new(),
                owner: BoundSymbol::NotASymbol,
            }],
            refs: Vec::new(),
        }
    }

    /// 스코프를 하나 연다.
    pub fn open(&mut self, kind: ScopeKind, parent: ScopeIx, owner: BoundSymbol) -> ScopeIx {
        let ix = ScopeIx(u32::try_from(self.scopes.len()).unwrap_or(u32::MAX));
        self.scopes.push(Scope {
            kind,
            parent: ScopeParent::Enclosing(parent),
            bindings: Vec::new(),
            owner,
        });
        ix
    }

    /// 이름 하나를 선언한다.
    pub fn declare(&mut self, scope: ScopeIx, binding: ScopeBinding) {
        if let Some(s) = self.scopes.get_mut(scope.0 as usize) {
            s.bindings.push(binding);
        }
    }

    /// `at` 바이트에서 `name` 을 찾는다 — **안에서 밖으로.**
    ///
    /// # 규칙 셋이 여기 있다
    ///
    /// - **섀도잉** — 안쪽 스코프를 먼저 본다. 찾으면 거기서 멈춘다
    /// - **호이스팅** — `hoisted` 인 이름은 `at` 보다 뒤에 선언돼도 해소된다
    /// - **TDZ** — `hoisted` 가 아닌 이름을 선언 전에 참조하면 [`RefResolution::BeforeDeclaration`].
    ///   **바깥으로 더 나가지 않는다** — JavaScript 에서 그 참조는 바깥 이름을 보는 것이
    ///   아니라 오류다. 나가면 *"바깥에 같은 이름이 있으면 조용히 그것을 가리키는"* 답이 된다
    ///
    /// # ⚠ 함수 경계를 지나면 자리 비교를 하지 않는다
    ///
    /// TDZ 는 **참조가 선언보다 먼저 실행될 때**의 규칙이다. 함수 본문 안에서 뒤에 선
    /// 모듈 상수를 부르는 것은 **완전히 정상이고 흔하다** — 그 본문은 나중에 실행된다.
    ///
    /// ```text
    /// function f() { return LATER }     ← 자리로만 보면 「선언 전 참조」다
    /// const LATER = 1                    ← 그러나 f 가 불릴 때는 이미 있다
    /// ```
    ///
    /// 이 규칙 없이 실물(ditto 496 파일)에 대 보면 **14 건이 전부 거짓 양성**이었고,
    /// 그것이 그대로 심볼을 `ordinal` 로 떨어뜨렸다. 그러므로 참조 자리에서 바인딩의
    /// 스코프까지 올라가는 길에 **함수 스코프를 하나라도 지났으면** 자리를 비교하지 않는다.
    ///
    /// 같은 스코프에 같은 이름이 여럿이면(오버로드 · 재선언) **가장 앞선 것**을 쓴다.
    ///
    /// ⚠ **이것은 [`ResolveRule::Tdz`] 한 규칙이다.** 다른 규칙이 필요한 언어는
    /// [`Self::resolve_with`] 를 부른다 — 여기 팔을 더하면 TypeScript 해소가 함께
    /// 움직이고, 그것은 골든 두 파일이 잡는 자리다.
    ///
    /// ⚠⚠ **추출기는 이 함수를 안 부른다.** 2026-09-08 부터 두 추출기가 전부
    /// [`Self::resolve_with`] 를 부르고, 이 자리는 **기본값을 고정하는 껍데기와
    /// 시험의 입구**로 남았다. 「부르는 곳이 없다」가 아니라 「기본값이 여기 있다」다.
    #[must_use]
    pub fn resolve(&self, from: ScopeIx, name: &str, namespace: Namespace, at: usize) -> RefResolution {
        self.resolve_with(from, name, namespace, at, ResolveRule::Tdz)
    }

    /// 규칙을 골라 해소한다.
    ///
    /// **뼈대는 하나이고 규칙만 갈린다** — [`ResolveRule`] 이 그 갈림을 값으로 진다.
    #[must_use]
    pub fn resolve_with(
        &self,
        from: ScopeIx,
        name: &str,
        namespace: Namespace,
        at: usize,
        rule: ResolveRule,
    ) -> RefResolution {
        match rule {
            ResolveRule::Tdz => self.resolve_tdz(from, name, namespace, at),
            ResolveRule::Shadowing => self.resolve_shadowing(from, name, namespace, at),
        }
    }

    /// [`ResolveRule::Shadowing`] — Rust.
    ///
    /// # 세 규칙이 TypeScript 와 반대로 돈다
    ///
    /// - **가장 뒤** — `let x = 1; use(x); let x = 2; use(x);` 에서 둘째 `use` 는 둘째
    ///   `x` 를 본다. 「가장 앞선 것」으로 재면 이 저장소에서 선언 판정 **214 건**이
    ///   참조로 오분류된다
    /// - **TDZ 가 없다** — 아직 안 선 지역 이름은 오류가 아니라 **바깥의 그 이름**이다.
    ///   `const X: u32 = 1; fn f(){ let y = X; let X = 2; }` 의 `X` 는 상수를 가리킨다
    /// - **아이템은 자리가 없다** — `hoisted` 인 선언은 스코프 전체에서 보이므로 순서로
    ///   못 가른다. 둘이면 [`RefResolution::Ambiguous`]
    ///
    /// 아이템과 지역이 함께 있으면 **지역이 이긴다** — 그 자리부터는 지역이 아이템을
    /// 가린다. 그래서 아이템의 자리를 0 으로 두고 지역은 `declared_at + 1` 로 잰다.
    fn resolve_shadowing(
        &self,
        from: ScopeIx,
        name: &str,
        namespace: Namespace,
        at: usize,
    ) -> RefResolution {
        let mut cursor = from;
        loop {
            let Some(scope) = self.scopes.get(cursor.0 as usize) else {
                return RefResolution::OutsideFile;
            };
            let mut best: Option<(usize, u32, bool)> = None;
            let mut 아이템_수 = 0usize;
            for (i, b) in scope.bindings.iter().enumerate() {
                if b.name != name || b.namespace != namespace {
                    continue;
                }
                // ★ **선언 자리 그 자체는 언제나 자기 자신이다.** [`ScopeBinding::visible_from`]
                //   이 선언문 끝으로 밀리면 그 이름 토큰이 **바깥의 동명 아이템**으로
                //   해소되고, 그러면 `let q = 1;` 한 줄이 `fn q` 로 가는 가짜 엣지를 낳는다.
                //   `file_edges` 의 선언 거르기는 **뽑힌 바인딩**의 자리를 보므로 그 뒤에
                //   서면 늦다.
                if b.declared_at == at {
                    return RefResolution::Bound {
                        scope: cursor,
                        binding: u32::try_from(i).unwrap_or(u32::MAX),
                    };
                }
                let key = if b.hoisted {
                    아이템_수 += 1;
                    0
                } else if b.visible_from <= at {
                    b.visible_from + 1
                } else {
                    // 아직 안 섰다 — **오류가 아니다.** 바깥에서 찾는다.
                    continue;
                };
                let ix = u32::try_from(i).unwrap_or(u32::MAX);
                if best.is_none_or(|(k, ..)| key >= k) {
                    best = Some((key, ix, b.hoisted));
                }
            }
            if let Some((_, binding, hoisted)) = best {
                if hoisted && 아이템_수 > 1 {
                    return RefResolution::Ambiguous;
                }
                return RefResolution::Bound { scope: cursor, binding };
            }
            match scope.parent {
                ScopeParent::Root => return RefResolution::OutsideFile,
                ScopeParent::Enclosing(next) => cursor = next,
            }
        }
    }

    /// [`ResolveRule::Tdz`] — TypeScript. 위 [`Self::resolve`] 의 문서가 이 규칙이다.
    fn resolve_tdz(&self, from: ScopeIx, name: &str, namespace: Namespace, at: usize) -> RefResolution {
        let mut cursor = from;
        let mut crossed_function = false;
        loop {
            let Some(scope) = self.scopes.get(cursor.0 as usize) else {
                return RefResolution::OutsideFile;
            };
            let mut shadowed = false;
            for (i, b) in scope.bindings.iter().enumerate() {
                if b.name != name || b.namespace != namespace {
                    continue;
                }
                if b.hoisted || crossed_function || b.declared_at <= at {
                    let binding = u32::try_from(i).unwrap_or(u32::MAX);
                    return RefResolution::Bound { scope: cursor, binding };
                }
                shadowed = true;
            }
            if shadowed {
                return RefResolution::BeforeDeclaration;
            }
            crossed_function = crossed_function || scope.kind == ScopeKind::Function;
            match scope.parent {
                ScopeParent::Root => return RefResolution::OutsideFile,
                ScopeParent::Enclosing(next) => cursor = next,
            }
        }
    }

    /// `scope` 가 `ancestor` 안(자기 자신 포함)인가.
    #[must_use]
    pub fn is_within(&self, scope: ScopeIx, ancestor: ScopeIx) -> bool {
        let mut cursor = scope;
        loop {
            if cursor == ancestor {
                return true;
            }
            match self.scopes.get(cursor.0 as usize).map(|s| s.parent) {
                Some(ScopeParent::Enclosing(next)) => cursor = next,
                _ => return false,
            }
        }
    }

    /// 파일 안 참조 중 **이 파일의 선언으로 해소된** 비율 — 백분율, 내림.
    ///
    /// **분모가 함께 실려야 한다.** [`RefResolution::OutsideFile`] 은 실패가 아니므로
    /// 분모에서 뺀다 — 넣으면 이 값이 *"이 파일이 얼마나 자족적인가"* 를 재게 되고,
    /// import 를 많이 쓰는 파일이 해소를 못 한 것처럼 보인다.
    #[must_use]
    pub fn resolution_percent(&self) -> (usize, usize) {
        let inside: Vec<&LocalRef> = self
            .refs
            .iter()
            .filter(|r| !matches!(r.resolved, RefResolution::OutsideFile))
            .collect();
        let bound = inside.iter().filter(|r| matches!(r.resolved, RefResolution::Bound { .. })).count();
        (bound, inside.len())
    }
}

impl Default for ScopeChain {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn 이름(name: &str, at: usize, hoisted: bool) -> ScopeBinding {
        ScopeBinding {
            name: name.to_owned(),
            namespace: Namespace::Value,
            declared_at: at,
            visible_from: at,
            hoisted,
            symbol: BoundSymbol::NotASymbol,
        }
    }

    const 모듈: ScopeIx = ScopeIx(0);

    #[test]
    fn 안쪽이_바깥쪽을_가린다() {
        let mut c = ScopeChain::new();
        c.declare(모듈, 이름("x", 0, false));
        let inner = c.open(ScopeKind::Braced, 모듈, BoundSymbol::NotASymbol);
        c.declare(inner, 이름("x", 10, false));
        assert_eq!(
            c.resolve(inner, "x", Namespace::Value, 20),
            RefResolution::Bound { scope: inner, binding: 0 },
            "섀도잉이 안 걸렸다 — 바깥 이름을 봤다"
        );
    }

    #[test]
    fn 호이스팅된_이름은_뒤에_선언돼도_해소된다() {
        let mut c = ScopeChain::new();
        c.declare(모듈, 이름("f", 100, true));
        assert!(matches!(c.resolve(모듈, "f", Namespace::Value, 0), RefResolution::Bound { .. }));
    }

    #[test]
    fn tdz_는_해소되지_않고_바깥으로_나가지도_않는다() {
        // **이 시험이 「스코프 체인」과 「이름 표」를 가른다**(옛 DESIGN §5.1).
        //
        // 바깥에 같은 이름이 있는데도 안쪽의 선언 전 참조가 바깥으로 새면, 그 답은
        // *"조용히 다른 선언을 가리키는"* 답이다. JavaScript 에서 그 참조는 오류다.
        let mut c = ScopeChain::new();
        c.declare(모듈, 이름("x", 0, false));
        let inner = c.open(ScopeKind::Braced, 모듈, BoundSymbol::NotASymbol);
        c.declare(inner, 이름("x", 50, false));
        assert_eq!(
            c.resolve(inner, "x", Namespace::Value, 10),
            RefResolution::BeforeDeclaration,
            "선언 전 참조가 바깥 `x` 로 샜다"
        );
    }

    #[test]
    fn 두_이름_공간이_갈린다() {
        // `interface Foo` 와 `const Foo` 가 공존한다. 뭉개면 해소가 조용히 틀린다.
        let mut c = ScopeChain::new();
        c.declare(모듈, 이름("Foo", 0, false));
        c.declare(
            모듈,
            ScopeBinding { namespace: Namespace::Type, ..이름("Foo", 20, false) },
        );
        let RefResolution::Bound { binding: v, .. } = c.resolve(모듈, "Foo", Namespace::Value, 40)
        else {
            panic!("값 자리가 해소되지 않았다")
        };
        let RefResolution::Bound { binding: t, .. } = c.resolve(모듈, "Foo", Namespace::Type, 40)
        else {
            panic!("타입 자리가 해소되지 않았다")
        };
        assert_ne!(v, t, "두 이름 공간이 한쪽으로 뭉개졌다");
    }

    #[test]
    fn 함수_경계를_지나면_뒤에_선_이름도_해소된다() {
        // **실물에서 TDZ 거짓 양성 14 건이 전부 이 형태였다.** 함수 본문은 나중에 실행된다.
        let mut c = ScopeChain::new();
        c.declare(모듈, 이름("LATER", 100, false));
        let body = c.open(ScopeKind::Function, 모듈, BoundSymbol::NotASymbol);
        assert!(
            matches!(c.resolve(body, "LATER", Namespace::Value, 10), RefResolution::Bound { .. }),
            "함수 안에서 뒤에 선 모듈 상수를 부르는 것을 TDZ 로 잡았다"
        );
        // **같은 스코프에서는 여전히 TDZ 다** — 규칙을 통째로 끈 것이 아니다.
        assert_eq!(
            c.resolve(모듈, "LATER", Namespace::Value, 10),
            RefResolution::BeforeDeclaration
        );
    }

    #[test]
    fn 파일_밖의_이름은_실패가_아니다() {
        let c = ScopeChain::new();
        assert_eq!(c.resolve(모듈, "console", Namespace::Value, 0), RefResolution::OutsideFile);
    }

    #[test]
    fn 해소율의_분모는_파일_밖을_빼고_잰다() {
        // 넣으면 import 를 많이 쓰는 파일이 해소를 못 한 것처럼 보인다.
        let mut c = ScopeChain::new();
        c.declare(모듈, 이름("x", 0, false));
        c.refs.push(LocalRef {
            name: "x".to_owned(),
            namespace: Namespace::Value,
            at: 5,
            resolved: RefResolution::Bound { scope: 모듈, binding: 0 },
        });
        c.refs.push(LocalRef {
            name: "console".to_owned(),
            namespace: Namespace::Value,
            at: 9,
            resolved: RefResolution::OutsideFile,
        });
        assert_eq!(c.resolution_percent(), (1, 1));
    }

    #[test]
    fn 섀도잉_규칙은_보이는_것_중_가장_뒤를_쓴다() {
        // Rust: `let x = 1; use(x); let x = 2; use(x);` — 둘째 `use` 는 둘째 `x` 다.
        let mut c = ScopeChain::new();
        c.declare(모듈, 이름("x", 0, false));
        c.declare(모듈, 이름("x", 50, false));
        assert_eq!(
            c.resolve_with(모듈, "x", Namespace::Value, 20, ResolveRule::Shadowing),
            RefResolution::Bound { scope: 모듈, binding: 0 },
            "첫째 `x` 만 보이는 자리에서 둘째를 골랐다"
        );
        assert_eq!(
            c.resolve_with(모듈, "x", Namespace::Value, 80, ResolveRule::Shadowing),
            RefResolution::Bound { scope: 모듈, binding: 1 },
            "섀도잉이 안 걸렸다 — 가려진 첫째를 봤다"
        );
        // **같은 자리를 TDZ 규칙으로 재면 첫째가 나온다** — 규칙이 실제로 갈린다.
        assert_eq!(
            c.resolve(모듈, "x", Namespace::Value, 80),
            RefResolution::Bound { scope: 모듈, binding: 0 }
        );
    }

    #[test]
    fn 섀도잉_규칙에는_tdz_가_없다() {
        // `const X = 1; fn f(){ let y = X; let X = 2; }` — 앞의 `X` 는 상수다.
        let mut c = ScopeChain::new();
        c.declare(모듈, 이름("X", 0, true));
        let 안 = c.open(ScopeKind::Function, 모듈, BoundSymbol::NotASymbol);
        c.declare(안, 이름("X", 100, false));
        assert_eq!(
            c.resolve_with(안, "X", Namespace::Value, 50, ResolveRule::Shadowing),
            RefResolution::Bound { scope: 모듈, binding: 0 },
            "아직 안 선 지역 이름이 바깥을 가렸다 — Rust 아이템에 TDZ 는 없다"
        );
        // TDZ 규칙은 같은 자리를 「선언 전 참조」로 판정한다.
        assert_eq!(c.resolve(안, "X", Namespace::Value, 50), RefResolution::BeforeDeclaration);
    }

    #[test]
    fn 동명_아이템이_둘이면_해소하지_않는다() {
        // `#[cfg(unix)] fn spawn_child` · `#[cfg(windows)] fn spawn_child`.
        // **하나를 고르면 그것이 조용한 오답이다** — `stitch_of` 가 EXPORTS 에서 하는 것과 같다.
        let mut c = ScopeChain::new();
        c.declare(모듈, 이름("spawn_child", 10, true));
        c.declare(모듈, 이름("spawn_child", 90, true));
        assert_eq!(
            c.resolve_with(모듈, "spawn_child", Namespace::Value, 200, ResolveRule::Shadowing),
            RefResolution::Ambiguous
        );
        // **지역이 가리면 모호하지 않다** — 그 자리부터는 후보가 하나다.
        c.declare(모듈, 이름("spawn_child", 150, false));
        assert_eq!(
            c.resolve_with(모듈, "spawn_child", Namespace::Value, 200, ResolveRule::Shadowing),
            RefResolution::Bound { scope: 모듈, binding: 2 }
        );
    }

    #[test]
    fn 서로_다른_impl_스코프의_동명_메서드는_서로를_안_본다() {
        // `impl A { fn new(){} } impl B { fn new(){} }` — 둘째 `new` 의 **선언 이름**이
        // 첫째로 해소되면 그것이 그대로 가짜 엣지가 된다(실측 72 건).
        let mut c = ScopeChain::new();
        let a = c.open(ScopeKind::Impl, 모듈, BoundSymbol::NotASymbol);
        c.declare(a, 이름("new", 10, true));
        let b = c.open(ScopeKind::Impl, 모듈, BoundSymbol::NotASymbol);
        c.declare(b, 이름("new", 40, true));
        assert_eq!(
            c.resolve_with(b, "new", Namespace::Value, 40, ResolveRule::Shadowing),
            RefResolution::Bound { scope: b, binding: 0 },
            "둘째 impl 의 `new` 가 첫째 impl 로 샜다"
        );
    }

    #[test]
    fn 안에_있는가는_자기_자신을_포함한다() {
        let mut c = ScopeChain::new();
        let a = c.open(ScopeKind::Function, 모듈, BoundSymbol::NotASymbol);
        let b = c.open(ScopeKind::Braced, a, BoundSymbol::NotASymbol);
        assert!(c.is_within(b, a));
        assert!(c.is_within(a, a));
        assert!(!c.is_within(a, b));
    }
}
