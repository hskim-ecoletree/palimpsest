//! 심볼과 그 자리.

use serde::{Deserialize, Serialize};

/// 선언의 종류.
///
/// 앞의 다섯이 Kotlin 최상위 선언 10종(class/interface/object/enum/data/annotation/
/// typealias/fun/val/var)을 전부 덮는다 — T7 이 먼저 확인했고 S0 의 쿼리가 그것을 따른다.
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
    /// 세면 폭발한다(F02 §3.3).
    Variable,
    /// 클래스 본문의 `method_definition`. 포함 관계(C1)로 클래스에 매인다.
    Method,
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
    /// 그대로이고, 그 차이가 재결박 제안의 근거가 된다([R-08] · [F03 §2]).
    pub body: crate::coord::BodyDigest,
}
