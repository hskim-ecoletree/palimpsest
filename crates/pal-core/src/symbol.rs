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
