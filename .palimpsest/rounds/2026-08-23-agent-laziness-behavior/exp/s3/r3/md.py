"""마크다운 렌더러."""

import re
from typing import List, Tuple


class MarkdownError(Exception):
    """마크다운 렌더링 오류."""
    pass


def render(text: str) -> str:
    """마크다운 텍스트를 HTML로 렌더링한다.

    Args:
        text: 마크다운 문자열

    Returns:
        HTML 문자열 (블록마다 한 줄, 끝에 개행 하나)

    Raises:
        MarkdownError: 잘못된 입력
    """
    if not text:
        return ""

    # 규칙 24: \r\n 을 \n 과 같게 다룬다
    text = text.replace("\r\n", "\n")

    # 코드 펜스 검증
    _validate_code_fences(text)

    # 블록 단위로 분해
    blocks = _parse_blocks(text)

    # 각 블록을 HTML로 변환
    html_lines = []
    for block_type, content in blocks:
        html = _render_block(block_type, content)
        if html:
            html_lines.append(html)

    if not html_lines:
        return ""

    return "\n".join(html_lines) + "\n"


def _validate_code_fences(text: str) -> None:
    """코드 펜스가 제대로 닫혀 있는지 확인."""
    lines = text.split("\n")
    in_fence = False
    for line in lines:
        if line.strip().startswith("```"):
            in_fence = not in_fence
    if in_fence:
        raise MarkdownError("안 닫힌 코드 펜스")


def _is_table_start(line: str) -> bool:
    """라인이 표의 헤더 줄인지 확인한다."""
    return "|" in line and line.strip().startswith("|")


def _is_table_separator(line: str) -> bool:
    """라인이 표의 구분선인지 확인한다."""
    if not ("|" in line):
        return False
    # 파이프로 분리된 각 셀이 --- 패턴인지 확인
    cells = line.split("|")
    for cell in cells:
        cell = cell.strip()
        if not cell:
            continue
        # --- 또는 :--  또는 --: 또는 :-: 등의 정렬 표시 확인
        if not re.match(r"^:?-+:?$", cell):
            return False
    return True


def _parse_blocks(text: str) -> List[Tuple[str, str]]:
    """텍스트를 블록으로 분해한다."""
    lines = text.split("\n")
    blocks = []
    i = 0

    while i < len(lines):
        line = lines[i]

        # 빈 줄 건너뛰기
        if not line.strip():
            i += 1
            continue

        # 코드 펜스 (``` 로 시작)
        if line.strip().startswith("```"):
            fence_lines = [line]
            i += 1
            while i < len(lines):
                fence_lines.append(lines[i])
                if lines[i].strip().startswith("```"):
                    break
                i += 1
            blocks.append(("code_fence", "\n".join(fence_lines)))
            i += 1
            continue

        # 수평선
        if re.match(r"^(---+|\*\*\*+|___+)$", line.strip()):
            blocks.append(("hr", ""))
            i += 1
            continue

        # 헤딩
        heading_match = re.match(r"^(#{1,6})\s+(.+?)(\s*#+)?$", line)
        if heading_match:
            level = len(heading_match.group(1))
            content = heading_match.group(2)
            blocks.append(("heading", f"{level}|{content}"))
            i += 1
            continue

        # 표 (규칙 21-23)
        if _is_table_start(line) and i + 1 < len(lines):
            if _is_table_separator(lines[i + 1]):
                # 테이블 파싱
                table_lines = [line, lines[i + 1]]
                i += 2
                while i < len(lines) and lines[i].strip() and lines[i].startswith("|"):
                    table_lines.append(lines[i])
                    i += 1
                blocks.append(("table", "\n".join(table_lines)))
                continue

        # 순서 있는 목록
        if re.match(r"^\s*\d+[.)]\s+", line):
            list_lines = []
            while i < len(lines) and (not lines[i].strip() or re.match(r"^(\s*(\d+[.)]\s+|  ))", lines[i])):
                if not lines[i].strip():
                    break
                list_lines.append(lines[i])
                i += 1
            blocks.append(("ol", "\n".join(list_lines)))
            continue

        # 순서 없는 목록
        if re.match(r"^\s*[-*+]\s+", line):
            list_lines = []
            while i < len(lines) and (not lines[i].strip() or re.match(r"^(\s*([-*+]\s+|  ))", lines[i])):
                if not lines[i].strip():
                    break
                list_lines.append(lines[i])
                i += 1
            blocks.append(("ul", "\n".join(list_lines)))
            continue

        # 인용문
        if line.startswith(">"):
            quote_lines = []
            while i < len(lines) and (lines[i].startswith(">") or (not lines[i].strip() and i + 1 < len(lines) and lines[i + 1].startswith(">"))):
                if lines[i].startswith(">"):
                    quote_lines.append(lines[i][1:].lstrip() if len(lines[i]) > 1 else "")
                else:
                    quote_lines.append("")
                i += 1
            blocks.append(("blockquote", "\n".join(quote_lines)))
            continue

        # 일반 문단
        para_lines = []
        while i < len(lines) and lines[i].strip() and not re.match(r"^(#{1,6}\s+|---+|\*\*\*+|___+|\s*\d+[.)]\s+|\s*[-*+]\s+|>)", lines[i]):
            para_lines.append(lines[i])
            i += 1

        if para_lines:
            blocks.append(("paragraph", "\n".join(para_lines)))

    return blocks


def _render_block(block_type: str, content: str) -> str:
    """블록을 HTML로 렌더링한다."""
    if block_type == "heading":
        level, text = content.split("|", 1)
        level = int(level)
        html_content = _render_inline(text)
        return f"<h{level}>{html_content}</h{level}>"

    elif block_type == "paragraph":
        # 줄바꿈 처리
        lines = content.split("\n")
        rendered_lines = []
        for line in lines:
            html_content = _render_inline(line)
            rendered_lines.append(html_content)

        full_html = "\n".join(rendered_lines)
        # 줄 끝 공백 두 개를 <br>로
        full_html = re.sub(r"  \n", "<br>\n", full_html)

        return f"<p>{full_html}</p>"

    elif block_type == "hr":
        return "<hr>"

    elif block_type == "code_fence":
        lines = content.split("\n")
        first_line = lines[0]

        # 언어 추출
        lang_match = re.match(r"^```(\w+)?", first_line)
        lang = lang_match.group(1) if lang_match and lang_match.group(1) else None

        # 코드 내용 (첫 줄과 마지막 줄 제외)
        code_lines = lines[1:-1] if len(lines) > 2 else []
        code_content = "\n".join(code_lines)

        # HTML 이스케이프
        code_content = _escape_html(code_content)

        if lang:
            return f'<pre><code class="language-{lang}">{code_content}\n</code></pre>'
        else:
            return f"<pre><code>{code_content}\n</code></pre>"

    elif block_type == "ul":
        return _render_list(content, "ul")

    elif block_type == "ol":
        return _render_list(content, "ol")

    elif block_type == "blockquote":
        inner_html = render(content)
        if inner_html.endswith("\n"):
            inner_html = inner_html[:-1]
        lines = inner_html.split("\n") if inner_html else []
        result = "<blockquote>\n" + "\n".join(lines) + "\n</blockquote>"
        return result

    elif block_type == "table":
        return _render_table(content)

    return ""


def _render_list(content: str, list_type: str) -> str:
    """목록을 HTML로 렌더링한다."""
    lines = content.split("\n")

    # 목록 항목과 들여쓰기 수준 파싱
    items = []
    for line in lines:
        if not line.strip():
            continue

        # 들여쓰기 수준 계산 (2칸 = 1 레벨)
        indent = (len(line) - len(line.lstrip())) // 2

        # 마크 제거
        match = re.match(r"^\s*([-*+]|\d+[.)])\s+(.*)$", line)
        if match:
            text = match.group(2)
            items.append((indent, text))

    if not items:
        return f"<{list_type}></{list_type}>"

    # 중첩 구조로 HTML 생성
    return _build_nested_list(items, list_type, 0)


def _build_nested_list(items: List[Tuple[int, str]], list_type: str, target_indent: int) -> str:
    """중첩된 목록을 HTML로 구성한다."""
    if not items or items[0][0] < target_indent:
        return ""

    html_lines = [f"<{list_type}>"]
    i = 0

    while i < len(items):
        indent, text = items[i]

        if indent < target_indent:
            break

        if indent == target_indent:
            html_content = _render_inline(text)

            # 다음 항목이 중첩인지 확인
            if i + 1 < len(items) and items[i + 1][0] > target_indent:
                # 중첩된 목록이 있음
                nested_items = []
                j = i + 1
                while j < len(items) and items[j][0] > target_indent:
                    nested_items.append(items[j])
                    j += 1

                # 중첩 목록 생성 (같은 리스트 타입 사용)
                nested_html = _build_nested_list(nested_items, list_type, target_indent + 1)

                html_lines.append(f"<li>{html_content}")
                html_lines.append(nested_html)
                html_lines.append("</li>")

                i = j
            else:
                html_lines.append(f"<li>{html_content}</li>")
                i += 1
        else:
            break

    html_lines.append(f"</{list_type}>")
    return "\n".join(html_lines)


def _render_table(content: str) -> str:
    """표를 HTML로 렌더링한다 (규칙 21-23)."""
    lines = content.split("\n")
    if len(lines) < 2:
        return ""

    # 헤더 줄 파싱
    header_cells = _parse_table_row(lines[0])
    if not header_cells:
        return ""

    # 구분선 파싱 (정렬 정보 추출)
    separator_cells = _parse_table_row(lines[1])
    alignments = []
    for sep in separator_cells:
        alignments.append(_get_alignment(sep))

    # HTML 생성 시작
    html_lines = ["<table>", "<thead>", "<tr>"]

    # 헤더 셀 렌더링
    for i, cell in enumerate(header_cells):
        html_content = _render_inline(cell)
        alignment = alignments[i] if i < len(alignments) else None
        if alignment:
            html_lines.append(f'<th style="text-align:{alignment}">{html_content}</th>')
        else:
            html_lines.append(f"<th>{html_content}</th>")

    html_lines.extend(["</tr>", "</thead>", "<tbody>"])

    # 본문 줄 렌더링
    for line in lines[2:]:
        if not line.strip():
            continue
        body_cells = _parse_table_row(line)
        html_lines.append("<tr>")
        for i, cell in enumerate(body_cells):
            html_content = _render_inline(cell)
            alignment = alignments[i] if i < len(alignments) else None
            if alignment:
                html_lines.append(f'<td style="text-align:{alignment}">{html_content}</td>')
            else:
                html_lines.append(f"<td>{html_content}</td>")
        html_lines.append("</tr>")

    html_lines.extend(["</tbody>", "</table>"])

    return "\n".join(html_lines)


def _parse_table_row(line: str) -> List[str]:
    """표의 한 줄을 파싱하여 셀 목록을 반환한다."""
    # 양쪽 파이프 제거
    line = line.strip()
    if line.startswith("|"):
        line = line[1:]
    if line.endswith("|"):
        line = line[:-1]

    # 파이프로 분리하고 공백 제거
    cells = [cell.strip() for cell in line.split("|")]
    return cells


def _get_alignment(sep_cell: str) -> str:
    """구분선 셀에서 정렬을 추출한다."""
    sep_cell = sep_cell.strip()

    if sep_cell.startswith(":") and sep_cell.endswith(":"):
        return "center"
    elif sep_cell.endswith(":"):
        return "right"
    elif sep_cell.startswith(":"):
        return "left"

    return ""


def _render_inline(text: str) -> str:
    """인라인 요소를 렌더링한다."""
    # 역슬래시 이스케이프 (모든 것보다 먼저)
    text = _handle_backslash_escapes(text)

    # HTML 이스케이프
    text = _escape_html(text)

    # 코드 스팬 (중요: 다른 마크업보다 먼저)
    text = _render_code_span(text)

    # 강조 및 기타 인라인 마크업
    text = _render_emphasis(text)

    return text


def _handle_backslash_escapes(text: str) -> str:
    """역슬래시 이스케이프를 처리한다."""
    # 이스케이프된 문자들을 플레이스홀더로 변환
    escapes = {
        r"\*": "\x00STAR\x00",
        r"\_": "\x00USCORE\x00",
        r"\#": "\x00HASH\x00",
        r"\-": "\x00DASH\x00",
        r"\~": "\x00TILDE\x00",
        r"\[": "\x00LBRACK\x00",
        r"\!": "\x00EXCL\x00",
        r"\\": "\x00BSLASH\x00",
    }
    for escaped, placeholder in escapes.items():
        text = text.replace(escaped, placeholder)
    return text


def _restore_backslash_escapes(text: str) -> str:
    """이스케이프 플레이스홀더를 원래 문자로 복원한다."""
    restores = {
        "\x00STAR\x00": "*",
        "\x00USCORE\x00": "_",
        "\x00HASH\x00": "#",
        "\x00DASH\x00": "-",
        "\x00TILDE\x00": "~",
        "\x00LBRACK\x00": "[",
        "\x00EXCL\x00": "!",
        "\x00BSLASH\x00": "\\",
    }
    for placeholder, char in restores.items():
        text = text.replace(placeholder, char)
    return text


def _escape_html(text: str) -> str:
    """HTML 특수 문자를 이스케이프한다."""
    text = text.replace("&", "&amp;")
    text = text.replace("<", "&lt;")
    text = text.replace(">", "&gt;")
    return text


def _render_code_span(text: str) -> str:
    """코드 스팬을 렌더링한다 (백틱)."""
    # 백틱 안의 내용은 이미 이스케이프됨
    pattern = r"`([^`]+)`"
    return re.sub(pattern, r"<code>\1</code>", text)


def _render_emphasis(text: str) -> str:
    """강조, 기울임, 취소선을 렌더링한다."""
    # 강조 (굵게) - **...** 먼저 (non-bracket, non-star)
    text = re.sub(r"\*\*([^*\[\]]+?)\*\*", r"<strong>\1</strong>", text)
    text = re.sub(r"__([^_\[\]]+?)__", r"<strong>\1</strong>", text)

    # 기울임 - *...* 또는 _..._
    text = re.sub(r"\*([^*\[\]]+?)\*", r"<em>\1</em>", text)
    text = re.sub(r"_([^_\[\]]+?)_", r"<em>\1</em>", text)

    # 취소선 - ~~...~~
    text = re.sub(r"~~([^~\[\]]+?)~~", r"<del>\1</del>", text)

    # 이미지 - ![대체](주소) (먼저 처리)
    text = re.sub(r"!\[([^\]]*)\]\(([^)]+)\)", r'<img src="\2" alt="\1">', text)

    # 링크 - [텍스트](주소)
    text = re.sub(r"\[([^\]]+)\]\(([^)]+)\)", r'<a href="\2">\1</a>', text)

    # 이스케이프 플레이스홀더 복원
    text = _restore_backslash_escapes(text)

    return text
