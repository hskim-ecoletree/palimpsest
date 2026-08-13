//! 언어와 무관한 파싱 뒷일 — 오류 타입 · 정규형 · 회복 지점 세기.
//!
//! **셋 다 언어에 의존하지 않는다.** 리프 토큰을 모으고, 주석을 버리고, 선택적
//! 세미콜론을 버리고, `ERROR`·`MISSING` 노드를 센다. 언어마다 따로 쓰면 **두 언어의
//! `body_digest` 가 서로 다른 규칙 위에 서게 되고**, 그러면 같은 리팩터가 한 언어에서만
//! 결박을 `stale` 로 만든다.

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

/// `ERROR` · `MISSING` 노드를 센다.
///
/// **회복 지점의 좌표(`Site`)는 F03 이후다** — 좌표에 `symbol` 성분이 필요하다.
/// 여기서는 개수만 세고, 그 개수가 `partial` 의 근거가 된다. 회복을 1급으로 다루는 것은
/// **#47**.
#[must_use]
pub fn count_error_nodes(root: Node<'_>) -> usize {
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
