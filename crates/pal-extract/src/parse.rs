//! 언어와 무관한 파싱 뒷일 — 오류 타입 · 정규형 · 회복 지점 세기.
//!
//! **셋 다 언어에 의존하지 않는다.** 리프 토큰을 모으고, 주석을 버리고, 선택적
//! 세미콜론을 버리고, `ERROR`·`MISSING` 노드를 센다. 언어마다 따로 쓰면 **두 언어의
//! `body_digest` 가 서로 다른 규칙 위에 서게 되고**, 그러면 같은 리팩터가 한 언어에서만
//! 결박을 `stale` 로 만든다.

use std::cell::RefCell;

use pal_core::{RecoveryKind, RecoverySite, Span};
use tree_sitter::{Node, Parser, Tree};

/// 토큰 사이 구분자. 넣지 않으면 `fun f` 와 `funf` 가 같은 바이트열이 된다.
const TOKEN_SEPARATOR: u8 = 0x1f;

/// 지워진 지역 이름의 표식. **소스에 나타날 수 없는 바이트여야 한다** — 나타나면
/// 이름이 `\x1e0` 인 변수가 첫째 지역과 같은 바이트열이 된다.
const LOCAL_MARKER: u8 = 0x1e;

/// 구문 트리의 마디를 여닫는 표식 — **정규형이 토큰 열이 아니라 트리라는 것.**
///
/// # 왜 필요한가 — `prettier` 가 괄호를 지운다
///
/// 포매터는 **불필요한 그룹 괄호**를 자유롭게 넣고 뺀다. ditto 실측에서 `prettier`
/// 한 번에 심볼 **58** 개(파일 40)가 움직였고 **원인이 그것 하나였다** —
/// `(a, b) => (cond ? x : y)` 를 `(a, b) => cond ? x : y` 로 만든다.
///
/// **그런데 평평한 토큰 열에서 괄호만 지우면 서로 다른 코드가 같아진다** —
/// `(a+b)*c` 와 `a+b*c` 가 한 바이트열이 되고, 그것이 [R-22] 가 경고한 충돌 그
/// 자체다. 괄호가 지우던 일을 **트리 모양**이 대신해야 지울 수 있다.
///
/// 그래서 마디마다 여닫는 표식을 싣고 [`parenthesized_expression`] 은 통째로 벗긴다:
///
/// ```text
/// (a+b)*c  →  ⟨ ⟨a+b⟩ * c ⟩
/// a+b*c    →  ⟨ a + ⟨b*c⟩ ⟩      ← 다르다
/// ```
///
/// [`parenthesized_expression`]: https://github.com/tree-sitter/tree-sitter-typescript
/// [R-22]: ../../../docs/plan/00-risks.md#r-22
const NODE_OPEN: u8 = 0x1c;
const NODE_CLOSE: u8 = 0x1a;

/// 그룹 괄호만 있는 마디 — **벗긴다.** 위 [`NODE_OPEN`] 의 주석이 그 근거다.
///
/// Kotlin 의 같은 자리는 `parenthesized_expression` 으로 이름이 같다.
/// **언어마다 따로 쓰지 않는다** — 두 언어의 `body_digest` 가 서로 다른 규칙 위에
/// 서면 같은 리팩터가 한 언어에서만 결박을 `stale` 로 만든다(모듈 주석).
const TRANSPARENT: [&str; 1] = ["parenthesized_expression"];

/// **자식이 하나뿐이면** 벗기는 마디.
///
/// 선행 `|` 를 지우는 것만으로는 부족하다 — `| A | B` 는 트리에서
/// `union(union(|, A), |, B)` 이고 `A | B` 는 `union(A, |, B)` 다. **마디 하나가
/// 더 있다.** 표식이 트리 모양을 싣게 된 뒤로는 그 차이가 그대로 요약에 남는다.
///
/// 자식이 하나인 합집합·교집합은 **그 자식과 같은 타입이므로** 벗겨도 잃는 것이 없다.
const TRANSPARENT_IF_SINGLE: [&str; 2] = ["union_type", "intersection_type"];

/// 따옴표를 벗긴 문자열 리터럴의 표식.
///
/// **표식 없이 내용만 흘리면 리터럴과 식별자가 같아진다** — `'x'` 와 `x` 가 한
/// 바이트열이 되고, 그러면 `f('x')` 와 `f(x)` 가 같은 요약을 갖는다. 그것은
/// 정규화가 아니라 의미 손실이다.
const STRING_MARKER: u8 = 0x1d;

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

thread_local! {
    /// **스레드당 파서 하나.** F02 §3.1 이 요구한 것이고 #46 이 빚으로 넘긴 자리다.
    ///
    /// `Parser::new()` 가 싸지 않고, rayon 워커마다 하나면 충분하다. 언어마다 문법을
    /// 다시 붙이는 것도 비싸므로 **마지막에 붙인 문법을 기억해 같은 언어면 건너뛴다.**
    ///
    /// # 이것이 결정성을 깨지 않는 이유
    ///
    /// 파서는 파싱이 끝나면 상태를 남기지 않는다 — `parse(source, None)` 은 이전 트리를
    /// 쓰지 않는다(증분 파싱은 둘째 인자를 준다). 그래서 **같은 소스는 파서를 재사용해도
    /// 같은 트리를 낸다**. `[f02.4.pass]` ① 이 그것을 회차 다섯으로 되묻는다.
    static PARSER: RefCell<(Parser, Option<tree_sitter::Language>)> =
        RefCell::new((Parser::new(), None));
}

/// 이 스레드의 파서로 파싱한다.
///
/// # Errors
/// 문법을 붙이지 못하거나 파싱이 중단되면 [`ExtractError`].
pub fn parse_with(language: &tree_sitter::Language, source: &[u8]) -> Result<Tree, ExtractError> {
    PARSER.with(|cell| {
        let (parser, attached) = &mut *cell.borrow_mut();
        if attached.as_ref() != Some(language) {
            parser.set_language(language)?;
            *attached = Some(language.clone());
        }
        parser.parse(source, None).ok_or(ExtractError::ParseAborted)
    })
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
    normalize_erasing(node, source, &|_| None)
}

/// 지역 이름을 **자리 번호로 지운** 정규형 — `identity_grade == exact` 인 심볼의 것.
///
/// # 왜 지우는가, 그리고 왜 아무 때나 지우면 안 되는가
///
/// 지역 변수 이름을 바꾸는 것은 의미 변경이 아니다. 안 지우면 `rename` 한 번에 결박이
/// `stale` 로 켜지고, 사람이 표시를 무시하기 시작하면 제품이 죽는다([R-07]).
///
/// **그런데 어느 이름이 지역인지 모르면 지울 수 없다.** 모르는 채로 지우면 서로 다른
/// 코드가 같은 요약을 갖는다 — [R-22] 가 경고한 정확히 그 형태다. 그래서 이 함수는
/// 호출자가 **스코프 해소로** 정한 자리 번호만 받는다.
///
/// `erase(byte)` 는 그 자리의 토큰이 지워야 할 지역이면 자리 번호를 낸다.
///
/// # 자리 번호가 이름을 대신한다 — 그리고 **번호끼리는 구별된다**
///
/// 전부 같은 바이트로 지우면 `f(a, b)` 와 `f(a, a)` 가 같아진다. 번호를 실어 **어느
/// 지역인지**는 남기고 **그것의 이름**만 지운다.
///
/// [R-07]: ../../../docs/plan/00-risks.md#r-07
/// [R-22]: ../../../docs/plan/00-risks.md#r-22
#[must_use]
pub fn normalize_erasing(
    node: Node<'_>,
    source: &[u8],
    erase: &dyn Fn(usize) -> Option<usize>,
) -> Vec<u8> {
    let mut out = Vec::new();
    normalize_into(&mut out, node, source, erase);
    out
}

/// **커서를 넘기지 않고 각 층에서 만든다.** 넘기면 커서의 수명이 자식 노드의 수명과
/// 얽혀 재귀가 서지 않는다 — 빌림 하나를 아끼려다 타입이 막는 자리였다.
fn normalize_into(
    out: &mut Vec<u8>,
    node: Node<'_>,
    source: &[u8],
    erase: &dyn Fn(usize) -> Option<usize>,
) {
    let kind = node.kind();
    if kind.contains("comment") {
        return;
    }
    // **따옴표 종류는 스타일이다** (F03 §3.1). `prettier` 가 가장 자주 바꾸는 것이고,
    // 안 지우면 포매터 한 번에 문자열을 가진 모든 심볼이 `stale` 로 켜진다.
    if kind == "string" || is_plain_template(node) {
        normalize_string(out, node, source);
        return;
    }
    if node.child_count() == 0 {
        if kind == ";" {
            return;
        }
        match erase(node.start_byte()) {
            Some(slot) => {
                out.push(LOCAL_MARKER);
                out.extend_from_slice(slot.to_string().as_bytes());
            }
            None => out.extend_from_slice(&source[node.byte_range()]),
        }
        out.push(TOKEN_SEPARATOR);
        return;
    }
    let mut cursor = node.walk();
    let kids: Vec<Node<'_>> = node.children(&mut cursor).collect();
    drop(cursor);

    // **그룹 괄호는 마디째 벗긴다.** 표식도 괄호 토큰도 안 남긴다 — 표식만 지우면
    // `(` 와 `)` 가 잎으로 그대로 흘러 아무것도 안 지운 것이 된다.
    let named = kids.iter().filter(|n| n.is_named()).count();
    let transparent =
        TRANSPARENT.contains(&kind) || (named == 1 && TRANSPARENT_IF_SINGLE.contains(&kind));
    if !transparent {
        out.push(NODE_OPEN);
    }
    for (i, child) in kids.iter().enumerate() {
        if is_trailing_comma(&kids, i) {
            continue;
        }
        // 투명한 마디에서는 **이름 있는 자식만** 본다 — 나머지는 괄호뿐이다.
        if transparent && !child.is_named() {
            continue;
        }
        if is_leading_separator(&kids, i) {
            continue;
        }
        normalize_into(out, *child, source, erase);
    }
    if !transparent {
        out.push(NODE_CLOSE);
    }
}

/// `i` 번째 자식이 **후행 쉼표**인가 — 뒤에 내용도 다른 쉼표도 없는 `,`.
///
/// # 왜 「다른 쉼표도 없는」이 붙는가
///
/// `[a,]` 는 길이 1 이고 `[a,,]` 는 길이 2 다(희소 배열). 마지막 쉼표만 지우면 둘이
/// 각각 `[a]` 와 `[a,]` 가 되어 **여전히 갈린다.** 뒤에 쉼표가 있으면 그 쉼표는
/// 후행이 아니라 **자리를 만드는 것**이다.
fn is_trailing_comma(kids: &[Node<'_>], i: usize) -> bool {
    if kids[i].kind() != "," || kids[i].child_count() != 0 {
        return false;
    }
    !kids[i + 1..].iter().any(|n| n.is_named() || n.kind() == ",")
}

/// `i` 번째 자식이 **선행 구분자**인가 — 맨 앞에 온 `|` · `&`.
///
/// # 포매터가 이것을 넣었다 뺐다 한다
///
/// `type X = | A | B` 와 `type X = A | B` 는 **같은 타입**이다. `prettier` 는 줄 폭에
/// 따라 앞의 `|` 를 붙이기도 하고 떼기도 한다 — ditto 실측에서 그것이 마지막 남은
/// 16 건이었다(파일 12).
///
/// **왼쪽 피연산자가 없는 `|` 는 타입 구분자뿐이다** — 비트 연산자는 언제나 왼쪽을
/// 갖는다. 그래서 「맨 앞」 하나로 가른다.
fn is_leading_separator(kids: &[Node<'_>], i: usize) -> bool {
    i == 0 && kids[i].child_count() == 0 && matches!(kids[i].kind(), "|" | "&")
}

/// 문자열 리터럴 하나 — **따옴표를 벗기고 이스케이프를 푼다.**
///
/// `prettier` 는 `"a\"b"` 를 `'a"b'` 로 바꾼다. 따옴표만 통일하고 이스케이프를 안
/// 풀면 그 변형에서 요약이 움직이고, 그러면 `[f03.2.pass]` ① 의 불변율이 100 이 안 된다.
///
/// **보간이 있는 템플릿은 여기 오지 않는다** — 백틱이 스타일이 아니라 표현식을 여는
/// 문법이기 때문이다. 보간이 **없는** 템플릿은 문자열과 같은 값이고, 실제로 린터가
/// 둘을 서로 바꾼다([`is_plain_template`]).
/// 보간이 **없는** 템플릿 리터럴인가 — 그렇다면 문자열과 같은 값이다.
///
/// # 린터가 둘을 서로 바꾼다
///
/// `` `x` `` 와 `'x'` 는 값이 같고, biome·eslint 의 `prefer-template` / 그 반대 규칙이
/// 코드베이스마다 한쪽으로 몰아 넣는다. 갈라 두면 그 정리 커밋 하나에 결박이 켜진다.
///
/// **ditto 의 `0d0f4aab`(*"biome lint:fix template literals"*)이 실제로 그렇게 켰다** —
/// ①(합성 포매팅)이 아니라 **②(실 이력)의 손 검토가 잡은 자리다.**
fn is_plain_template(node: Node<'_>) -> bool {
    if node.kind() != "template_string" {
        return false;
    }
    let mut cursor = node.walk();
    !node.children(&mut cursor).any(|c| c.kind() == "template_substitution")
}

fn normalize_string(out: &mut Vec<u8>, node: Node<'_>, source: &[u8]) {
    out.push(STRING_MARKER);
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        match child.kind() {
            "string_fragment" => out.extend_from_slice(&source[child.byte_range()]),
            "escape_sequence" => decode_escape(out, &source[child.byte_range()]),
            // 따옴표 토큰 — 버린다. 그것이 이 규칙의 전부다.
            _ => {}
        }
    }
    out.push(TOKEN_SEPARATOR);
}

/// `\n` · `\x41` · `\u{1F600}` … 를 그 값으로.
///
/// **모르는 이스케이프는 다음 글자를 그대로 낸다** — JavaScript 의 규칙이고
/// (`\q` 는 `q` 다), 지어내지 않는 쪽이기도 하다.
fn decode_escape(out: &mut Vec<u8>, raw: &[u8]) {
    let Some(&b'\\') = raw.first() else {
        out.extend_from_slice(raw);
        return;
    };
    let rest = &raw[1..];
    let Some(&head) = rest.first() else { return };
    match head {
        b'n' => out.push(b'\n'),
        b't' => out.push(b'\t'),
        b'r' => out.push(b'\r'),
        b'b' => out.push(0x08),
        b'f' => out.push(0x0c),
        b'v' => out.push(0x0b),
        b'0' if rest.len() == 1 => out.push(0),
        // 줄 이음 — 아무것도 내지 않는다.
        b'\n' | b'\r' => {}
        b'x' | b'u' => push_code_point(out, rest),
        // `\\` · `\'` · `\"` · 그 밖 — 다음 글자 그대로.
        _ => out.extend_from_slice(rest),
    }
}

fn push_code_point(out: &mut Vec<u8>, rest: &[u8]) {
    let digits = rest[1..]
        .iter()
        .copied()
        .filter(u8::is_ascii_hexdigit)
        .map(char::from)
        .collect::<String>();
    let Ok(n) = u32::from_str_radix(&digits, 16) else {
        out.extend_from_slice(rest);
        return;
    };
    // **대리 쌍의 반쪽은 문자가 아니다.** 그런 자리는 원문 그대로 둔다 — 지어내면
    // 서로 다른 리터럴이 같은 바이트열이 될 수 있다.
    match char::from_u32(n) {
        Some(c) => {
            let mut buf = [0u8; 4];
            out.extend_from_slice(c.encode_utf8(&mut buf).as_bytes());
        }
        None => out.extend_from_slice(rest),
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

// ─────────────────────────────────────────────────────────────────────────────
// 표식 있는 주석 (F10 §3.4)
//
// > 주석은 **가장 정확한 좌표를 이미 갖고 있다** — 붙어 있는 심볼.
// > 다만 주석이 결정인지 설명인지는 모른다. **표식이 있는 주석만 인입한다.**
//
// ⚠ **정규화 경로를 안 건드린다.** [`normalize`] 가 주석을 버리는 것은 의도한 동작이고
// (F03: *"주석 수정이 결박을 stale 로 만들지 않는다"*), 문서 §3.4 가 스스로 그것을
// 적었다. 그래서 여기 있는 것은 **별도의 수집 경로**이고 `body_digest` 에 안 닿는다.
// 닿으면 골든 넷이 움직이고 그것이 반증이다(`[f10.pass]`).
//
// # 왜 텍스트 스캔이 아니라 구문 트리인가
//
// 이 저장소가 가장 자주 밟은 대조 고장이 *"어디가 코드이고 어디가 아닌가"* 다
// (F03 아홉 중 다섯). 문자열 리터럴 안의 `// @decision:` 을 텍스트로 세면 그것이
// **여섯 번째**가 된다. tree-sitter 는 이미 그 답을 갖고 있다.
// ─────────────────────────────────────────────────────────────────────────────

/// 표식이 붙은 주석 하나 — **그리고 그것이 붙은 선언의 자리.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarkedComment {
    /// 주석 자신의 자리.
    pub span: Span,
    /// 주석의 글자 — 표식을 포함한다. **이것이 조각의 본문이 된다.**
    pub text: String,
    /// **바로 뒤에 오는 선언의 시작 바이트.** 붙일 심볼을 부르는 쪽이 이것으로 찾는다.
    ///
    /// 뒤에 선언이 없으면(파일 끝의 주석) [`None`] 이고, 그때 이 주석은 **좌표가 없다** —
    /// 지어내지 않는다.
    pub attaches_to_byte: Option<usize>,
}

/// 표식이 붙은 주석을 모은다. **표식 없는 주석은 안 본다**(§3.4 가 기각했다).
///
/// # 「붙어 있다」의 정의 — **바로 다음 형제**
///
/// 주석의 부모 안에서 **그 주석 다음에 오는 첫 이름 있는 마디**가 붙는 대상이다.
/// 사이에 다른 주석이 있으면 건너뛴다 — 여러 줄 주석 블록이 흔하기 때문이다.
/// 다음 마디가 없으면 좌표가 **없고**, 없는 것을 지어내지 않는다.
#[must_use]
pub fn marked_comments(root: Node<'_>, source: &[u8], markers: &[&str]) -> Vec<MarkedComment> {
    let mut out = Vec::new();
    모은다(root, source, markers, &mut out);
    // **결정적 순서** — 소스 순서다. 흔들리면 조각의 앵커가 흔들린다.
    out.sort_by_key(|c| c.span.byte_start);
    out
}

fn 모은다(node: Node<'_>, source: &[u8], markers: &[&str], out: &mut Vec<MarkedComment>) {
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        if child.kind().contains("comment") {
            let Ok(text) = std::str::from_utf8(&source[child.byte_range()]) else { continue };
            if markers.iter().any(|m| text.contains(m)) {
                out.push(MarkedComment {
                    span: Span {
                        byte_start: child.start_byte(),
                        byte_end: child.end_byte(),
                        // **자르지 않고 포화시킨다** — 줄 번호는 표시용이고,
                        // `as` 로 자르면 큰 파일에서 조용히 0 이 된다.
                        line_start: u32::try_from(child.start_position().row + 1)
                            .unwrap_or(u32::MAX),
                        line_end: u32::try_from(child.end_position().row + 1).unwrap_or(u32::MAX),
                    },
                    text: text.to_owned(),
                    attaches_to_byte: 다음_선언(child),
                });
            }
            continue;
        }
        모은다(child, source, markers, out);
    }
}

/// 이 주석 **다음에 오는 첫 이름 있는 마디**의 시작 바이트.
///
/// 주석을 건너뛰는 것이 이 함수의 전부다 — 주석 블록이 여러 줄이면 그 전부가 같은
/// 선언에 붙는다. 다음이 없으면 [`None`] 이고, 그것이 *"이 주석에는 좌표가 없다"* 다.
fn 다음_선언(comment: Node<'_>) -> Option<usize> {
    let mut n = comment.next_named_sibling();
    while let Some(x) = n {
        if !x.kind().contains("comment") {
            return Some(x.start_byte());
        }
        n = x.next_named_sibling();
    }
    None
}

#[cfg(test)]
mod marked_comment_tests {
    use super::*;
    use crate::extractor::LanguageExtractor;

    /// 표식 셋 — 문서 §3.4 그대로. **`ADR-` 가 넓은 것은 의도다**:
    /// ADR 을 인용하는 주석은 **구조상** 결정에 관한 것이다.
    const 표식: [&str; 2] = ["@decision:", "ADR-"];

    fn ts(src: &str) -> Vec<MarkedComment> {
        crate::TypeScriptExtractor.marked_comments(src.as_bytes(), &표식).expect("파싱")
    }

    #[test]
    fn 표식_있는_주석만_모은다() {
        let c = ts("// 그냥 설명\n// @decision: 재시도하지 않는다\nexport function f() {}\n");
        assert_eq!(c.len(), 1, "표식 없는 주석까지 모았다");
        assert!(c[0].text.contains("재시도하지 않는다"));
    }

    #[test]
    fn 주석이_다음_선언에_붙는다() {
        // **주석은 가장 정확한 좌표를 이미 갖고 있다**(§3.4) — 붙어 있는 선언.
        let src = "// ADR-0042 이래서 이렇다\nexport function cancel() {}\n";
        let c = ts(src);
        assert_eq!(c.len(), 1);
        let at = c[0].attaches_to_byte.expect("붙을 선언이 있다");
        assert!(src[at..].starts_with("export function cancel"), "{}", &src[at..]);
    }

    #[test]
    fn 주석_블록_전체가_같은_선언에_붙는다() {
        // 여러 줄 주석이 흔하다. 사이의 주석을 건너뛰지 않으면 앞줄이 좌표를 잃는다.
        let src = "// @decision: 첫 줄\n// 이어지는 설명\n// ADR-0007\nexport class C {}\n";
        let c = ts(src);
        assert_eq!(c.len(), 2, "표식 있는 둘이 나와야 한다");
        assert_eq!(c[0].attaches_to_byte, c[1].attaches_to_byte);
    }

    #[test]
    fn 파일_끝의_주석은_좌표가_없다() {
        // **없는 것을 지어내지 않는다.** 좌표 없는 주석은 미결박으로 간다.
        let c = ts("export function f() {}\n// @decision: 뒤에 아무것도 없다\n");
        assert_eq!(c.len(), 1);
        assert_eq!(c[0].attaches_to_byte, None);
    }

    #[test]
    fn 문자열_안의_표식은_주석이_아니다() {
        // **★ 이 저장소가 가장 자주 밟은 고장이 여기다** — *"어디가 코드이고 어디가
        // 아닌가"*(F03 아홉 중 다섯). 텍스트로 세면 이것이 여섯 번째가 된다.
        let c = ts("export const s = \"// @decision: 이건 문자열이다\"\n");
        assert!(c.is_empty(), "문자열 안의 표식을 주석으로 셌다: {c:?}");
    }

    #[test]
    fn 주석_수집이_요약을_안_건드린다() {
        // ⚠ **정규화는 주석을 버린다** — F03 이 세운 것이고 F10 이 안 건드린다.
        // 건드리면 골든 넷이 움직이고 그것이 반증이다(`[f10.pass]`).
        let 없이 = crate::TypeScriptExtractor.extract(b"export function f() { return 1 }\n").expect("추출");
        let 있이 = crate::TypeScriptExtractor
            .extract("// @decision: 무언가\nexport function f() { return 1 }\n".as_bytes())
            .expect("추출");
        assert_eq!(없이.symbols[0].body, 있이.symbols[0].body,
                   "표식 주석이 `body_digest` 를 움직였다");
    }
}
