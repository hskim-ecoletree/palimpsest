//! 심볼과 그 자리.

use serde::{Deserialize, Serialize};

/// 최상위 선언의 종류.
///
/// 다섯이 Kotlin 최상위 선언 10종(class/interface/object/enum/data/annotation/
/// typealias/fun/val/var)을 전부 덮는다 — T7 이 먼저 확인했고 S0 의 쿼리가 그것을 따른다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SymbolKind {
    Class,
    Function,
    Object,
    TypeAlias,
    Property,
}

impl SymbolKind {
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Class => "class",
            Self::Function => "fun",
            Self::Object => "object",
            Self::TypeAlias => "typealias",
            Self::Property => "property",
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
/// `Coord`(repo·tree·extractor·symbol) 는 아직 없다 — F01·F03 의 것이다.
/// S0 은 파일 하나 안에서 닫히므로 좌표계를 미리 흉내 내지 않는다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub span: Span,
}
