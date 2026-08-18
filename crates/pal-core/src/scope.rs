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
    Class,
    /// 중괄호 하나. `let`·`const`·`class` 가 여기 갇히고 `var` 는 안 갇힌다.
    ///
    /// **이름이 `Block` 이 아니다** — `block` 이 `pal-core` 의 금지어다(모듈 주석).
    Braced,
}

/// TypeScript 의 두 이름 공간 — **뭉개면 해소가 조용히 틀린다.**
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
/// **셋을 가르는 것이 이 타입의 전부다.** 뭉개면 *"파일 밖의 이름"*(정상)과
/// *"선언 전 참조"*(TDZ)가 같은 출력이 되고, 그러면 해소율이 무엇을 세는지 알 수 없다.
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
}

/// 파일 하나의 스코프 체인.
///
/// **`scopes[0]` 이 모듈 스코프다.** 비어 있는 체인은 만들지 않는다 — 파일이 있으면
/// 모듈 스코프는 있다.
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
    #[must_use]
    pub fn resolve(&self, from: ScopeIx, name: &str, namespace: Namespace, at: usize) -> RefResolution {
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
    fn 해소율의_분모는_파일_밖을_빼고_센다() {
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
    fn 안에_있는가는_자기_자신을_포함한다() {
        let mut c = ScopeChain::new();
        let a = c.open(ScopeKind::Function, 모듈, BoundSymbol::NotASymbol);
        let b = c.open(ScopeKind::Braced, a, BoundSymbol::NotASymbol);
        assert!(c.is_within(b, a));
        assert!(c.is_within(a, a));
        assert!(!c.is_within(a, b));
    }
}
