//! 언어와 무관한 파싱 뒷일 — 오류 타입 · 정규형 · 회복 지점 세기.
//!
//! **셋 다 언어에 의존하지 않는다.** 리프 토큰을 모으고, 주석을 버리고, 선택적
//! 세미콜론을 버리고, `ERROR`·`MISSING` 노드를 센다. 언어마다 따로 쓰면 **두 언어의
//! `body_digest` 가 서로 다른 규칙 위에 서게 되고**, 그러면 같은 리팩터가 한 언어에서만
//! 결박을 `stale` 로 만든다.

use pal_core::{RecoveryKind, RecoverySite, Span};
use tree_sitter::Node;

/// 토큰 사이 구분자. 넣지 않으면 `fun f` 와 `funf` 가 같은 바이트열이 된다.
const TOKEN_SEPARATOR: u8 = 0x1f;

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

/// 선언 하나의 **정규형** — 주석·공백·포매팅을 지운 바이트열.
///
/// # 무엇을 지우고 무엇을 남기는가 ([F03 §3.1](../../../docs/plan/features/F03-identity.md))
///
/// | 지운다 | 왜 |
/// |---|---|
/// | 공백·줄바꿈·들여쓰기 | 포매터가 매일 바꾼다. **리프 토큰만 모으므로 자동으로 사라진다** |
/// | 주석 | 문서 수정이 코드 변경이 아니다 |
/// | 선택적 세미콜론 | Kotlin 에서 스타일이고, TypeScript 에서도 ASI 가 있어 스타일이다 |
///
/// **지역 변수명·파라미터명은 지우지 않는다.** [R-22] 가 경고한 자리다 — 그것을 지우려면
/// 그 심볼의 스코프 해소가 성공해야 하는데 지금 두 추출기 다 L1(구조)이라 스코프가 없다.
/// 등급이 못 미치는데 지우면 **서로 다른 코드가 같은 요약을 갖는다.** 지우는 것은 **#48**.
#[must_use]
pub fn normalize(node: Node<'_>, source: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    normalize_into(&mut out, node, source);
    out
}

/// **커서를 넘기지 않고 각 층에서 만든다.** 넘기면 커서의 수명이 자식 노드의 수명과
/// 얽혀 재귀가 서지 않는다 — 빌림 하나를 아끼려다 타입이 막는 자리였다.
fn normalize_into(out: &mut Vec<u8>, node: Node<'_>, source: &[u8]) {
    let kind = node.kind();
    if kind.contains("comment") {
        return;
    }
    if node.child_count() == 0 {
        if kind == ";" {
            return;
        }
        out.extend_from_slice(&source[node.byte_range()]);
        out.push(TOKEN_SEPARATOR);
        return;
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        normalize_into(out, child, source);
    }
}

/// `ERROR` · `MISSING` 노드를 **자리로** 걷는다 — 소스 순서.
///
/// # 개수에서 자리로 (#47 · `[f02.2.pass]` ③)
///
/// 옛 `count_error_nodes` 는 *"회복 지점의 좌표(`Site`)는 F03 이후다 — 좌표에 `symbol`
/// 성분이 필요하다"* 라고 적고 개수만 셌다. 그 이유가 성립하지 않는다 — 필요한 것은
/// [`RecoverySite`] 의 `span` 이고 그것은 파일 하나만 보면 안다.
///
/// # 오류 노드 **안쪽은 자리로 세지 않는다** — 그리고 그것이 문서와 어긋나지 않는다
///
/// F02 §4 는 *"ERROR 노드 안쪽도 순회해서 인식 가능한 선언을 건진다"* 이고 이 함수는
/// 안쪽으로 내려가지 않는다. **둘은 다른 것을 말한다** — 전자는 *선언을 건지는 순회*이고
/// (그것은 각 추출기의 순회가 한다) 여기는 *회복 지점을 세는 단위*다.
///
/// 중첩된 `ERROR` 를 따로 세면 **자리들이 서로 겹치고**, 그러면
/// [`FileGraph::error_ratio_percent`] 의 합이 소스 길이를 넘는다. 하나의 회복이 하나의
/// 자리다.
///
/// [`FileGraph::error_ratio_percent`]: pal_core::FileGraph::error_ratio_percent
#[must_use]
pub fn recovery_sites(root: Node<'_>) -> Vec<RecoverySite> {
    if !root.has_error() {
        return Vec::new(); // 흔한 경우를 순회 없이 끝낸다
    }
    let mut out = Vec::new();
    collect_sites(&mut out, root);
    out
}

/// **소스 순서로** 모은다 — 자식을 앞에서 뒤로 본다.
///
/// 커서를 넘기지 않고 각 층에서 만든다 — `normalize_into` 와 같은 이유다(커서의 수명이
/// 자식 노드의 수명과 얽혀 재귀가 서지 않는다).
fn collect_sites(out: &mut Vec<RecoverySite>, node: Node<'_>) {
    if node.is_error() || node.is_missing() {
        out.push(RecoverySite { kind: site_kind(node), span: span_of(node) });
        return;
    }
    if !node.has_error() {
        return;
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_sites(out, child);
    }
}

/// **`MISSING` 을 먼저 본다.** 지어 넣은 노드가 `ERROR` 로도 보이는 문법이 있으면 순서를
/// 뒤집는 순간 너비 0 인 자리가 `Error` 로 적히고, 그러면 강등 비율이 그것을 세려 든다.
fn site_kind(node: Node<'_>) -> RecoveryKind {
    if node.is_missing() { RecoveryKind::Missing } else { RecoveryKind::Error }
}

fn span_of(node: Node<'_>) -> Span {
    Span {
        byte_start: node.start_byte(),
        byte_end: node.end_byte(),
        line_start: u32::try_from(node.start_position().row).unwrap_or(u32::MAX) + 1,
        line_end: u32::try_from(node.end_position().row).unwrap_or(u32::MAX) + 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tree_sitter::Parser;

    fn 자리들(src: &str) -> Vec<RecoverySite> {
        let language = tree_sitter::Language::new(tree_sitter_typescript::LANGUAGE_TYPESCRIPT);
        let mut parser = Parser::new();
        parser.set_language(&language).expect("문법");
        let tree = parser.parse(src, None).expect("파싱");
        recovery_sites(tree.root_node())
    }

    #[test]
    fn 성한_소스는_자리가_없다() {
        assert!(자리들("export const a = 1;\n").is_empty());
    }

    #[test]
    fn 자리는_개수가_아니라_범위를_싣는다() {
        // **이 조각이 바꾼 것이 이 한 줄이다.** 개수만으로는 어디를 못 읽었는지 모른다.
        let sites = 자리들("export const a = 1;\n@@@ !!! @@@\nexport const b = 2;\n");
        assert!(!sites.is_empty(), "깨졌는데 자리가 없다");
        let s = sites[0];
        assert!(s.span.byte_start < s.span.byte_end, "범위가 비었다");
        assert!(s.span.line_start >= 2, "첫 줄은 성했는데 그 줄을 가리킨다");
    }

    #[test]
    fn 자리는_소스_순서다() {
        // 순서가 뒤집히면 *"첫 번째 공백"* 이 뜻을 잃는다.
        let sites = 자리들("const a = 1;\n@@@\nconst b = 2;\n###\nconst c = 3;\n");
        assert!(sites.len() >= 2, "자리가 둘 이상 나와야 하는 소스다");
        for w in sites.windows(2) {
            assert!(w[0].span.byte_start <= w[1].span.byte_start, "소스 순서가 아니다");
        }
    }

    #[test]
    fn 중첩_오류는_하나의_자리다() {
        // **안쪽을 따로 세면 자리들이 겹치고 비율의 합이 소스 길이를 넘는다.**
        let src = "class C { fun( { fun( { fun( {\n";
        let sites = 자리들(src);
        let covered: usize = sites.iter().map(RecoverySite::width).sum();
        assert!(covered <= src.len(), "자리들이 겹쳤다 — 덮인 넓이가 소스보다 크다");
        for w in sites.windows(2) {
            assert!(w[0].span.byte_end <= w[1].span.byte_start, "자리가 서로 겹친다");
        }
    }
}
