//! Kotlin 최상위 선언 추출.
//!
//! **한 쿼리 매치가 선언 하나다.** CLI 레퍼런스(`scripts/s0-reference.py`)가 세는 단위와
//! 같아야 하고, 같은 쿼리 파일을 쓰므로 같다.

use pal_core::{
    BodyDigest, Capable, CapabilityId, ExtractGrade, FileGraph, Language, LanguageId, Span, Symbol,
    SymbolKind,
};
use streaming_iterator::StreamingIterator;
use tree_sitter::{Parser, Query, QueryCursor};

use crate::extractor::LanguageExtractor;

/// 레지스트리가 잡는 자리. **무상태다** — #49 가 이것을 `par_iter` 안에서 부른다.
pub(crate) static KOTLIN: KotlinExtractor = KotlinExtractor;

/// S0 이 세운 최상위 쿼리 추출기.
///
/// **이 조각은 이것을 다시 짓지 않는다.** `queries/kotlin/top-level.scm` 을 CLI
/// 레퍼런스와 공유해야 `corpus/tasks/s0-reference-vector.tsv`(1,126 줄) 대조가
/// 성립하고, F01 의 골든 997 항목이 그 위에 선다. `FileGraph` 로 올리는 것은 그 둘을
/// 동시에 흔드는 일이라 **빚으로 적고 넘긴다**(`[f02.1.pass]` ④).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KotlinExtractor;

impl LanguageExtractor for KotlinExtractor {
    fn language(&self) -> Language {
        Language::Kotlin
    }

    fn grade(&self) -> ExtractGrade {
        crate::grade_of(Language::Kotlin)
    }

    fn extract(&self, source: &[u8]) -> Result<FileGraph, ExtractError> {
        extract_detailed(source)
    }
}

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

/// 최상위 선언을 소스 순서로 + **파싱이 성했는가**.
///
/// **S0 이 대조한 것이 이 함수의 `symbols` 다.** 그 값이 그대로 유지되어야
/// `corpus/tasks/s0-reference-vector.tsv`(1,126 줄)와의 대조가 계속 성립하고,
/// F01 의 골든 997 항목이 그 위에 선다.
///
/// 옛 `kotlin::extract(source) -> Vec<Symbol>` 는 없앴다. 트레잇이 그 자리를 가졌고
/// (`LanguageExtractor::extract`), **부르는 경로만 바뀌었지 세는 방식은 같다** —
/// `pal_extract::extract` 가 레지스트리를 거쳐 이 함수에 그대로 닿는다.
///
/// 대장은 `parsed` 와 `partial` 을 갈라야 하고(DESIGN §4.1) 그러려면 오류 회복이
/// 일어났는지를 알아야 한다. S0 은 그것을 묻지 않았으므로 버렸던 값이다.
///
/// # 산출 타입이 `FileGraph` 로 바뀌었다 — **값은 그대로다**
///
/// 옛 `Extraction { symbols, recovery_sites }` 가 [`FileGraph`] 에 흡수됐다. 늘어난
/// 것은 `language`·`grade`(둘 다 이 추출기의 상수)와 `contains` 뿐이고, **`contains`
/// 는 빈다.** 이 추출기는 `source_file` 의 직계 자식만 보므로 담긴 심볼을 애초에
/// 뽑지 않는다 — 담는 관계가 없는 것이 정확한 값이다.
///
/// # Errors
/// 문법·쿼리·파싱 중 하나가 실패하면 [`ExtractError`].
pub fn extract_detailed(source: &[u8]) -> Result<FileGraph, ExtractError> {
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
            body: BodyDigest::of_normalized(&normalize(decl_node, source)),
            span: Span {
                byte_start: decl_node.start_byte(),
                byte_end: decl_node.end_byte(),
                line_start: u32::try_from(decl_node.start_position().row).unwrap_or(u32::MAX) + 1,
                line_end: u32::try_from(decl_node.end_position().row).unwrap_or(u32::MAX) + 1,
            },
        });
    }
    Ok(FileGraph::flat(
        LanguageId::new(Language::Kotlin.name()),
        crate::grade_of(Language::Kotlin),
        symbols,
        count_error_nodes(tree.root_node()),
        // **빈 집합이 아니라 안 만들었다고 적는다.** 이 추출기는 `source_file` 의 직계
        // 자식만 보고 가시성·`import` 절을 아예 읽지 않는다. 빈 `ExportSet` 은
        // *"아무것도 안 내보낸다"* 는 뜻이고 Kotlin 최상위 선언에 대해 그것은 거짓이다.
        Capable::not_built(CapabilityId::new("F02", "kotlin-exports")),
        Capable::not_built(CapabilityId::new("F02", "kotlin-imports")),
    ))
}

/// 선언 하나의 **정규형** — 주석·공백·포매팅을 지운 바이트열.
///
/// # 무엇을 지우고 무엇을 남기는가 ([F03 §3.1](../../../docs/plan/features/F03-identity.md))
///
/// | 지운다 | 왜 |
/// |---|---|
/// | 공백·줄바꿈·들여쓰기 | 포매터가 매일 바꾼다. **리프 토큰만 모으므로 자동으로 사라진다** |
/// | 주석 | 문서 수정이 코드 변경이 아니다 |
/// | 선택적 세미콜론 | Kotlin 에서 스타일이다 |
///
/// **지역 변수명·파라미터명은 지우지 않는다.** [R-22] 가 경고한 자리다 — 그것을 지우려면
/// 그 심볼의 스코프 해소가 성공해야 하는데 이 추출기는 L1(구조)이라 스코프가 없다.
/// 등급이 못 미치는데 지우면 **서로 다른 코드가 같은 요약을 갖는다.**
///
/// 토큰 사이에 `` 를 넣는다. 넣지 않으면 `fun f` 와 `funf` 가 같은 바이트열이 된다.
fn normalize(node: tree_sitter::Node<'_>, source: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    normalize_into(&mut out, node, source);
    out
}

/// **커서를 넘기지 않고 각 층에서 만든다.** 넘기면 커서의 수명이 자식 노드의 수명과
/// 얽혀 재귀가 서지 않는다 — 빌림 하나를 아끼려다 타입이 막는 자리였다.
fn normalize_into(out: &mut Vec<u8>, node: tree_sitter::Node<'_>, source: &[u8]) {
    let kind = node.kind();
    if kind.contains("comment") {
        return;
    }
    if node.child_count() == 0 {
        // 세미콜론은 Kotlin 에서 선택적이다 — 있고 없고가 의미를 바꾸지 않는다.
        if kind == ";" {
            return;
        }
        out.extend_from_slice(&source[node.byte_range()]);
        out.push(0x1f);
        return;
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        normalize_into(out, child, source);
    }
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
