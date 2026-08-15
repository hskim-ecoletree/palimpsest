//! Kotlin 최상위 선언 추출.
//!
//! **한 쿼리 매치가 선언 하나다.** CLI 레퍼런스(`scripts/s0-reference.py`)가 세는 단위와
//! 같아야 하고, 같은 쿼리 파일을 쓰므로 같다.

use pal_core::{
    BodyDigest, Capable, CapabilityId, ExtractGrade, FileGraph, Language, LanguageId, Span, Symbol,
    SymbolKind,
};
use streaming_iterator::StreamingIterator;
use tree_sitter::{Query, QueryCursor};

use crate::extractor::LanguageExtractor;
use crate::parse::{ExtractError, normalize, parse_with, recovery_sites};

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

    fn marked_comments(
        &self,
        source: &[u8],
        markers: &[&str],
    ) -> Result<Vec<crate::parse::MarkedComment>, ExtractError> {
        let language = tree_sitter::Language::new(brokk_tree_sitter_kotlin::LANGUAGE);
        let tree = parse_with(&language, source)?;
        Ok(crate::parse::marked_comments(tree.root_node(), source, markers))
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
    let language = tree_sitter::Language::new(brokk_tree_sitter_kotlin::LANGUAGE);
    // **스레드당 파서를 재사용한다**(#49 · F02 §3.1). `Parser::new()` 는 싸지 않다.
    let tree = parse_with(&language, source)?;

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
            // **L1 이라 심볼 단위로도 `ordinal` 이다.** 스코프가 없으므로 어느 이름이
            // 지역인지 모르고, 모르면 지우지 않는다 — 그것이 R-22 의 요구다.
            // TypeScript 가 L2 로 오를 때 **이 팔이 딸려 오르면 안 된다**(`grade_of` 의
            // 주석과 같은 자리).
            identity: crate::grade_of(Language::Kotlin).identity(),
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
        recovery_sites(tree.root_node()),
        // **빈 집합이 아니라 안 만들었다고 적는다.** 이 추출기는 `source_file` 의 직계
        // 자식만 보고 가시성·`import` 절을 아예 읽지 않는다. 빈 `ExportSet` 은
        // *"아무것도 안 내보낸다"* 는 뜻이고 Kotlin 최상위 선언에 대해 그것은 거짓이다.
        Capable::not_built(CapabilityId::new("F02", "kotlin-exports")),
        Capable::not_built(CapabilityId::new("F02", "kotlin-imports")),
        // **빈 체인이 아니라 안 만들었다고 적는다.** 빈 `ScopeChain` 은 *"스코프가 없는
        // 파일"* 이라는 뜻이고 그것은 어떤 Kotlin 파일에 대해서도 참이 아니다.
        Capable::not_built(CapabilityId::new("F02", "kotlin-scopes")),
    ))
}
