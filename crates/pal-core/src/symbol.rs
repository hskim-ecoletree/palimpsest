//! 심볼과 그 자리.

use serde::{Deserialize, Serialize};

/// 선언의 종류.
///
/// 앞의 다섯이 Kotlin 최상위 선언 10종(class/interface/object/enum/data/annotation/
/// typealias/fun/val/var)을 전부 덮는다 — T7 이 먼저 확인했고 S0 의 쿼리가 그것을 따른다.
///
/// # 일곱이 더 늘었다 — Rust (ADR-0027 · #66 · 2026-08-20)
///
/// **접지 않고 늘린 것이 결정이다** — ADR-0027 §③. 접으면 cargo 코퍼스에서 좌표
/// **종류는 `SymbolId::compute` 의 성분이라 정직한 이름이 곧 정직한 좌표다** —
/// `pal symbols` 가 `struct` 를 `class` 라 부르면 그 거짓이 좌표에 실린다.
///
/// ⚠ **처음에 적은 근거(「접으면 충돌이 두 배로 는다」)는 반증됐다.** 그 수는
/// 추출기가 서기 전에 격리 스파이크로 잰 것이고 재현되지 않았다 — 접어도 안 늘었다.
/// **세는 자리는 `--example coord_collisions` 다**(독립 리뷰 R3·R4).
///
/// **앞의 아홉은 이름도 값도 안 건드렸다.** `SymbolId::compute` 는
/// `discriminator.kind.name()` **문자열**을 쓰므로 뒤에 더한 변형이 기존 이름을
/// 안 움직인다 — Kotlin·TypeScript 의 좌표가 그대로다.
///
/// # 넷이 늘었다 — TypeScript (F02-1 · #46)
///
/// **기존 다섯의 이름도 값도 건드리지 않았다.** 변형을 더하는 것은 Kotlin 산출을
/// 움직이지 않는다(`[f02.1.pass]` ④). Kotlin 은 `interface` 를 `Class` 로,
/// `enum class` 를 `Class` 로 접는데 그것은 S0 의 쿼리가 그렇게 세기 때문이고
/// **그 셈을 바꾸면 `s0-reference-vector.tsv` 대조가 깨진다.** 그래서 새 변형은
/// TypeScript 만 쓴다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SymbolKind {
    Class,
    Function,
    Object,
    TypeAlias,
    Property,
    /// TypeScript `interface_declaration`.
    Interface,
    /// TypeScript `enum_declaration`.
    Enum,
    /// 모듈 스코프 `variable_declarator`. **함수 내부 지역 변수는 심볼이 아니다** —
    /// 세면 폭발한다(옛 F02 §3.3).
    Variable,
    /// 클래스 본문의 `method_definition`. 포함 관계(C1)로 클래스에 매인다.
    Method,
    /// Rust `struct_item`. **`Class` 로 접지 않는다** — 접으면 `struct Error` 와
    /// 그 `impl Error` 안의 심볼이 같은 열쇠가 되어 cargo 코퍼스에서 충돌이
    /// **`Class` 로 접지 않는 것이 결정이다**(ADR-0027 §③) — 근거는 충돌 감소가
    /// 아니라 **이름의 정직성**이다. 세는 자리는 `--example coord_collisions`.
    Struct,
    /// Rust `trait_item`.
    Trait,
    /// Rust `mod_item` — 인라인(`mod m { … }`)과 파일 참조(`mod m;`) 둘 다.
    Module,
    /// Rust `const_item` — 연관 상수 포함.
    Const,
    /// Rust `static_item`.
    Static,
    /// Rust `macro_definition`(`macro_rules!`). **호출이 만드는 것은 심볼이 아니다** —
    /// 추출기가 매크로를 확장하지 않는다.
    Macro,
    /// Rust `union_item`.
    Union,
}

impl SymbolKind {
    /// 사람이 읽는 이름. **`Function` 이 `"fun"` 인 것은 Kotlin 표기이고, 그것을
    /// 바꾸면 `pal symbols` 의 표 출력이 움직인다** — 기계 대조는 `serde` 의
    /// `snake_case` 를 쓴다(`function` · `type_alias` · …).
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Class => "class",
            Self::Function => "fun",
            Self::Object => "object",
            Self::TypeAlias => "typealias",
            Self::Property => "property",
            Self::Interface => "interface",
            Self::Enum => "enum",
            Self::Variable => "variable",
            Self::Method => "method",
            Self::Struct => "struct",
            Self::Trait => "trait",
            Self::Module => "mod",
            Self::Const => "const",
            Self::Static => "static",
            Self::Macro => "macro",
            Self::Union => "union",
        }
    }
}

/// 소스 안의 자리. **줄 번호는 1부터다** — 사람이 읽는 좌표다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Span {
    pub byte_start: usize,
    pub byte_end: usize,
    pub line_start: u32,
    pub line_end: u32,
}

/// 추출된 심볼 하나.
///
/// `Coord` 는 여기 없다 — 좌표는 저장소·트리·추출기 버전을 알아야 하고 그것들은 파일
/// 하나 바깥의 사실이다. 추출기는 **파일 안에서 아는 것만** 낸다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub span: Span,
    /// **변했는가**에 답하는 값 — 주석·공백·포매팅을 지운 정규형의 요약.
    ///
    /// 정체성(`SymbolId`)과 다른 축이다. 파일을 옮기면 정체성은 끊기지만 이 값은
    /// 그대로이고, 그 차이가 재결박 제안의 근거가 된다([R-08] · [옛 F03 §2]).
    pub body: crate::coord::BodyDigest,
    /// **이 심볼에서 실제로 도달한 정체성 등급** — 언어 단위 선언이 아니라 심볼 단위 실측.
    ///
    /// # 언어 표만으로는 거짓말이 된다 ([R-22](../../../docs/plan/00-risks.md#r-22))
    ///
    /// > `identity_grade` 는 **언어 단위 선언이 아니라 심볼 단위 실측**이 된다 — 같은 언어
    /// > 안에서도 스코프 해소에 실패한 심볼은 `ordinal` 로 떨어지고, **그 심볼에서는 지역
    /// > 변수명을 지우지 않는다.**
    ///
    /// 언어 표(`LanguageCapability.identity`)는 **선언값**이고 이 값이 **실측**이다.
    /// 둘이 함께 서야 한다 — 언어 표만 남기면 해소에 실패한 심볼이 성공한 것처럼 보이고,
    /// 심볼 등급만 남기면 대장 머리의 *"결박 불가 언어 N 개"* 가 설 자리를 잃는다
    /// (옛 DESIGN §2.2).
    ///
    /// **그리고 이 값이 [`body`] 를 정한다.** `Exact` 면 지역 이름을 지운 정규형이고
    /// `Ordinal` 이면 지우지 않은 정규형이다 — 등급이 못 미치는데 지우면 **서로 다른
    /// 코드가 같은 요약**을 갖는다.
    ///
    /// [`body`]: Symbol::body
    pub identity: crate::ledger::IdentityGrade,
}
