//! Kotlin 최상위 선언 추출.
//!
//! **한 쿼리 매치가 선언 하나다.** CLI 레퍼런스(`scripts/s0-reference.py`)가 세는 단위와
//! 같아야 하고, 같은 쿼리 파일을 쓰므로 같다.

use pal_core::{Span, Symbol, SymbolKind};
use streaming_iterator::StreamingIterator;
use tree_sitter::{Parser, Query, QueryCursor};

/// 쿼리 원문. **CLI 레퍼런스가 읽는 그 파일이다.**
const TOP_LEVEL_QUERY: &str = include_str!("../queries/kotlin/top-level.scm");

/// 쿼리의 패턴 순서 → 선언 종류. **순서가 곧 계약이다** — 쿼리 파일과 함께 움직인다.
const KIND_BY_PATTERN: [SymbolKind; 5] = [
    SymbolKind::Class,
    SymbolKind::Function,
    SymbolKind::Object,
    SymbolKind::TypeAlias,
    SymbolKind::Property,
];

#[derive(Debug, thiserror::Error)]
pub enum ExtractError {
    #[error("문법을 붙이지 못했다: {0}")]
    Language(#[from] tree_sitter::LanguageError),
    #[error("쿼리가 잘못됐다: {0}")]
    Query(#[from] tree_sitter::QueryError),
    #[error("파싱이 중단됐다")]
    ParseAborted,
    #[error("쿼리에 `@{0}` 캡처가 없다")]
    MissingCapture(&'static str),
    #[error("쿼리 패턴이 {0}개다 — {1}개를 기대했다")]
    PatternCount(usize, usize),
    #[error("소스가 UTF-8 이 아니다")]
    NotUtf8,
}

/// 최상위 선언을 소스 순서로 낸다.
///
/// # Errors
/// 문법·쿼리·파싱 중 하나가 실패하면 [`ExtractError`].
pub fn extract(source: &[u8]) -> Result<Vec<Symbol>, ExtractError> {
    let language = tree_sitter::Language::new(tree_sitter_kotlin_ng::LANGUAGE);

    let mut parser = Parser::new();
    parser.set_language(&language)?;
    let tree = parser.parse(source, None).ok_or(ExtractError::ParseAborted)?;

    let query = Query::new(&language, TOP_LEVEL_QUERY)?;
    if query.pattern_count() != KIND_BY_PATTERN.len() {
        return Err(ExtractError::PatternCount(query.pattern_count(), KIND_BY_PATTERN.len()));
    }

    let decl_ix = query
        .capture_index_for_name("decl")
        .ok_or(ExtractError::MissingCapture("decl"))?;
    let name_ix = query
        .capture_index_for_name("name")
        .ok_or(ExtractError::MissingCapture("name"))?;

    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&query, tree.root_node(), source);

    let mut symbols = Vec::new();
    while let Some(m) = matches.next() {
        let kind = KIND_BY_PATTERN[m.pattern_index];

        // 한 매치에 `@name` 이 여럿일 수 있다(예: `val a, b`). **첫 번째를 쓴다.**
        let Some(name_node) = m.captures.iter().find(|c| c.index == name_ix).map(|c| c.node) else {
            continue;
        };
        let Some(decl_node) = m.captures.iter().find(|c| c.index == decl_ix).map(|c| c.node) else {
            continue;
        };

        let name = name_node.utf8_text(source).map_err(|_| ExtractError::NotUtf8)?;
        symbols.push(Symbol {
            name: name.to_owned(),
            kind,
            span: Span {
                byte_start: decl_node.start_byte(),
                byte_end: decl_node.end_byte(),
                line_start: u32::try_from(decl_node.start_position().row).unwrap_or(u32::MAX) + 1,
                line_end: u32::try_from(decl_node.end_position().row).unwrap_or(u32::MAX) + 1,
            },
        });
    }
    Ok(symbols)
}
