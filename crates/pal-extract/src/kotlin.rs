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
/// **S0 이 대조한 것이 이 함수의 산출이다.** 시그니처와 결과가 그대로 유지되어야
/// `corpus/tasks/s0-reference-vector.tsv` 와의 대조가 계속 성립한다.
///
/// # Errors
/// 문법·쿼리·파싱 중 하나가 실패하면 [`ExtractError`].
pub fn extract(source: &[u8]) -> Result<Vec<Symbol>, ExtractError> {
    extract_detailed(source).map(|e| e.symbols)
}

/// 선언 + **파싱이 성했는가**.
///
/// 대장은 `parsed` 와 `partial` 을 갈라야 하고(DESIGN §4.1) 그러려면 오류 회복이
/// 일어났는지를 알아야 한다. S0 은 그것을 묻지 않았으므로 [`extract`] 가 버렸다.
/// **버리던 정보를 되살릴 뿐 세는 방식은 같다** — 위 함수가 이 함수를 그대로 탄다.
///
/// # Errors
/// 문법·쿼리·파싱 중 하나가 실패하면 [`ExtractError`].
pub fn extract_detailed(source: &[u8]) -> Result<Extraction, ExtractError> {
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
    Ok(Extraction { symbols, recovery_sites: count_error_nodes(tree.root_node()) })
}

/// 추출 결과와 파싱 건강 상태.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Extraction {
    pub symbols: Vec<Symbol>,
    /// tree-sitter 가 오류 회복한 지점의 수. **0 이면 `parsed`, 아니면 `partial`.**
    pub recovery_sites: usize,
}

/// `ERROR` · `MISSING` 노드를 센다.
///
/// **회복 지점의 좌표(`Site`)는 F03 이후다** — 좌표에 `symbol` 성분이 필요하다.
/// 여기서는 개수만 세고, 그 개수가 `partial` 의 근거가 된다.
fn count_error_nodes(root: tree_sitter::Node<'_>) -> usize {
    if !root.has_error() {
        return 0; // 흔한 경우를 순회 없이 끝낸다
    }
    let mut n = 0;
    let mut cursor = root.walk();
    let mut stack = vec![root];
    while let Some(node) = stack.pop() {
        if node.is_error() || node.is_missing() {
            n += 1;
            // 오류 노드 **안쪽은 세지 않는다** — 하나의 회복 지점이다.
            continue;
        }
        if node.has_error() {
            stack.extend(node.children(&mut cursor));
        }
    }
    n
}
