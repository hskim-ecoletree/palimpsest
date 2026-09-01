import re
from enum import Enum

class MarkdownError(Exception):
    pass

class ListType(Enum):
    UNORDERED = "ul"
    ORDERED = "ol"

def _escape_html(text: str) -> str:
    """Escape HTML special characters."""
    text = text.replace("&", "&amp;")
    text = text.replace("<", "&lt;")
    text = text.replace(">", "&gt;")
    return text

def _unescape_char(text: str) -> str:
    r"""Handle backslash escapes like \* \# etc."""
    # Replace \X with just X, where X is a special markdown char
    result = []
    i = 0
    while i < len(text):
        if text[i] == "\\" and i + 1 < len(text):
            next_char = text[i + 1]
            # Backslash escapes markdown special chars
            if next_char in "*_`~[!#+-":
                result.append(next_char)
                i += 2
            else:
                result.append(text[i])
                i += 1
        else:
            result.append(text[i])
            i += 1
    return "".join(result)

def _render_inline(text: str) -> str:
    """Render inline markdown: emphasis, links, code, etc."""
    # First, handle escape sequences - we need to track them
    # to avoid processing escaped characters

    # Process in order: code > escape > others

    result = []
    i = 0

    while i < len(text):
        # Check for backslash escape
        if text[i] == "\\" and i + 1 < len(text):
            next_char = text[i + 1]
            if next_char in "*_`~[!#+-":
                result.append(next_char)
                i += 2
                continue

        # Check for code span (single backtick)
        if text[i] == "`":
            end = i + 1
            while end < len(text) and text[end] != "`":
                end += 1
            if end < len(text):
                code_text = text[i+1:end]
                code_text = _escape_html(code_text)
                result.append(f"<code>{code_text}</code>")
                i = end + 1
                continue
            else:
                result.append(text[i])
                i += 1
                continue

        # Check for images ![alt](url)
        if text[i] == "!" and i + 1 < len(text) and text[i+1] == "[":
            alt_start = i + 2
            alt_end = text.find("]", alt_start)
            if alt_end != -1 and alt_end + 1 < len(text) and text[alt_end + 1] == "(":
                url_start = alt_end + 2
                url_end = text.find(")", url_start)
                if url_end != -1:
                    alt_text = text[alt_start:alt_end]
                    url = text[url_start:url_end]
                    result.append(f'<img src="{url}" alt="{alt_text}">')
                    i = url_end + 1
                    continue

        # Check for links [text](url)
        if text[i] == "[":
            link_end = text.find("]", i)
            if link_end != -1 and link_end + 1 < len(text) and text[link_end + 1] == "(":
                url_start = link_end + 2
                url_end = text.find(")", url_start)
                if url_end != -1:
                    link_text = text[i+1:link_end]
                    url = text[url_start:url_end]
                    # Recursively render link text
                    link_text = _render_inline(link_text)
                    result.append(f'<a href="{url}">{link_text}</a>')
                    i = url_end + 1
                    continue

        # Check for strong emphasis (**text**)
        if text[i:i+2] == "**":
            end = i + 2
            while end + 1 < len(text):
                if text[end:end+2] == "**":
                    inner = text[i+2:end]
                    inner = _render_inline(inner)
                    result.append(f"<strong>{inner}</strong>")
                    i = end + 2
                    break
                end += 1
            else:
                result.append(text[i])
                i += 1
            continue

        # Check for emphasis (*text* or _text_)
        if text[i] in "*_":
            delim = text[i]
            end = i + 1
            while end < len(text):
                if text[end] == delim:
                    inner = text[i+1:end]
                    inner = _render_inline(inner)
                    result.append(f"<em>{inner}</em>")
                    i = end + 1
                    break
                end += 1
            else:
                result.append(text[i])
                i += 1
            continue

        # Check for strikethrough ~~text~~
        if text[i:i+2] == "~~":
            end = i + 2
            while end + 1 < len(text):
                if text[end:end+2] == "~~":
                    inner = text[i+2:end]
                    inner = _render_inline(inner)
                    result.append(f"<del>{inner}</del>")
                    i = end + 2
                    break
                end += 1
            else:
                result.append(text[i])
                i += 1
            continue

        # Check for line break (two spaces at end)
        if text[i:].startswith("  \n") or text[i:].startswith("  \r\n"):
            result.append("<br>")
            if text[i:].startswith("  \r\n"):
                i += 4
            else:
                i += 3
            continue

        # Regular character
        result.append(_escape_html(text[i]))
        i += 1

    return "".join(result)

def _detect_list_type(line: str) -> tuple[ListType | None, int]:
    """Detect if line is a list item and return (type, indent)."""
    stripped = line.lstrip()
    indent = len(line) - len(stripped)

    # Check for unordered list
    if stripped and stripped[0] in "-*+" and len(stripped) > 1 and stripped[1] == " ":
        return ListType.UNORDERED, indent

    # Check for ordered list (1. or 1))
    match = re.match(r"^(\d+)[.)]\s", stripped)
    if match:
        return ListType.ORDERED, indent

    return None, 0

def render(text: str) -> str:
    """Render markdown text to HTML."""
    if not text:
        return ""

    lines = text.split("\n")

    # Check for unclosed code fences
    fence_count = 0
    for line in lines:
        if line.strip().startswith("```"):
            fence_count += 1
    if fence_count % 2 != 0:
        raise MarkdownError("Unclosed code fence")

    result = []
    i = 0

    while i < len(lines):
        line = lines[i]

        # Empty line
        if not line.strip():
            i += 1
            continue

        # Code fence
        if line.strip().startswith("```"):
            fence_start = i
            code_lines = []
            lang = line.strip()[3:].strip()

            i += 1
            while i < len(lines):
                if lines[i].strip().startswith("```"):
                    break
                code_lines.append(lines[i])
                i += 1

            i += 1  # Skip closing fence

            code_content = "\n".join(code_lines)
            code_content = _escape_html(code_content)
            if lang:
                result.append(f'<pre><code class="language-{lang}">{code_content}\n</code></pre>')
            else:
                result.append(f'<pre><code>{code_content}\n</code></pre>')
            continue

        # Heading
        if line.startswith("#"):
            level_match = re.match(r"^(#+)\s+(.+?)(?:\s+#+)?$", line)
            if level_match:
                level = len(level_match.group(1))
                content = level_match.group(2)
                content = _render_inline(content)
                result.append(f"<h{level}>{content}</h{level}>")
                i += 1
                continue

        # Horizontal rule
        if re.match(r"^(---|___|\*\*\*)$", line.strip()):
            result.append("<hr>")
            i += 1
            continue

        # Blockquote
        if line.startswith(">"):
            quote_lines = []
            while i < len(lines) and lines[i].startswith(">"):
                quote_content = lines[i][1:].lstrip()
                quote_lines.append(quote_content)
                i += 1

            # Skip empty lines between blockquotes
            while i < len(lines) and not lines[i].strip():
                i += 1
                if i < len(lines) and not lines[i].startswith(">"):
                    break

            # Recursively render blockquote content
            inner_text = "\n".join(quote_lines)
            inner_html = render(inner_text)
            # Remove trailing newline for inner HTML
            if inner_html.endswith("\n"):
                inner_html = inner_html[:-1]
            result.append(f"<blockquote>\n{inner_html}\n</blockquote>")
            continue

        # List detection
        list_type, indent = _detect_list_type(line)

        if list_type:
            # Collect all list items
            list_items = []
            start_indent = indent
            current_level = 0
            expected_type = list_type

            while i < len(lines):
                current_line = lines[i]

                if not current_line.strip():
                    i += 1
                    continue

                current_type, current_indent = _detect_list_type(current_line)

                if current_type is None:
                    break

                if current_type != expected_type:
                    break

                if current_indent < start_indent:
                    break

                # Extract item text
                stripped = current_line.lstrip()
                if current_type == ListType.UNORDERED:
                    item_text = stripped[2:]  # Remove "- " or "* " or "+ "
                else:
                    # Remove "1. " or "1) "
                    match = re.match(r"^\d+[.)]\s", stripped)
                    if match:
                        item_text = stripped[len(match.group(0)):]
                    else:
                        item_text = stripped

                item_indent = current_indent - start_indent

                list_items.append((item_indent, item_text, current_type))
                i += 1

            # Build nested list HTML
            def build_list_html(items, level=0):
                if not items:
                    return []

                html = []
                i = 0
                current_type = items[0][2]

                html.append(f"<{current_type.value}>")

                while i < len(items):
                    indent, text, item_type = items[i]

                    if indent == level and item_type == current_type:
                        html.append(f"<li>{_render_inline(text)}")

                        # Check for nested items
                        nested = []
                        j = i + 1
                        while j < len(items) and items[j][0] > level:
                            nested.append(items[j])
                            j += 1

                        if nested:
                            html.extend(build_list_html(nested, level + 2))

                        html.append("</li>")
                        i = j
                    else:
                        i += 1

                html.append(f"</{current_type.value}>")
                return html

            result.extend(build_list_html(list_items))
            continue

        # Paragraph
        para_lines = []
        while i < len(lines) and lines[i].strip():
            line_item = lines[i]
            if line_item.startswith("#") or line_item.startswith(">") or _detect_list_type(line_item)[0] or re.match(r"^(---|___|\*\*\*)$", line_item.strip()):
                break
            if line_item.strip().startswith("```"):
                break
            para_lines.append(line_item)
            i += 1

        if para_lines:
            para_text = "\n".join(para_lines)
            para_text = _render_inline(para_text)
            result.append(f"<p>{para_text}</p>")

    return "\n".join(result) + "\n" if result else ""
