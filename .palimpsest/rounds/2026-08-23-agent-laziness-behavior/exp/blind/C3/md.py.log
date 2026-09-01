import re
from typing import List, Tuple, Optional
import hashlib


# Global storage for protected code spans during processing
_CODE_SPANS = {}


class MarkdownError(Exception):
    """Exception raised for invalid markdown."""
    pass


def render(text: str) -> str:
    """Render markdown text to HTML string."""
    if not text:
        return ""

    # Handle \r\n line endings (convert to \n)
    text = text.replace('\r\n', '\n')

    # Split into lines
    lines = text.split('\n')

    # Process blocks
    html_blocks = []
    i = 0

    while i < len(lines):
        line = lines[i]

        # Check for code fence start
        if line.strip().startswith('```'):
            # Process code block
            result, next_i = _process_code_fence(lines, i)
            html_blocks.append(result)
            i = next_i
            continue

        # Check for heading
        if line.startswith('#'):
            level, content = _parse_heading(line)
            if level:
                html_blocks.append(_render_heading(level, content))
                i += 1
                continue

        # Check for horizontal line
        if _is_horizontal_line(line):
            html_blocks.append('<hr>')
            i += 1
            continue

        # Check for blockquote (must have > followed by space or be empty after >)
        if line.startswith('> ') or (line.strip() == '>' and i + 1 < len(lines)):
            result, next_i = _process_blockquote(lines, i)
            html_blocks.append(result)
            i = next_i
            continue

        # Check for list (ordered or unordered)
        if _is_list_start(line):
            result, next_i = _process_list(lines, i)
            html_blocks.append(result)
            i = next_i
            continue

        # Check for table
        if _is_table_start(lines, i):
            result, next_i = _process_table(lines, i)
            html_blocks.append(result)
            i = next_i
            continue

        # Check for empty line
        if not line.strip():
            i += 1
            continue

        # Regular paragraph - group consecutive non-empty lines
        para_lines = [line]
        i += 1
        while i < len(lines) and lines[i].strip():
            # Stop if we hit a special block
            next_line = lines[i]
            if (next_line.strip().startswith('```') or
                next_line.startswith('#') or
                _is_horizontal_line(next_line) or
                next_line.startswith('>') or
                _is_list_start(next_line) or
                _is_table_start(lines, i)):
                break
            para_lines.append(next_line)
            i += 1

        # Join lines with special handling for line breaks (rule 17)
        para_text = _join_paragraph_lines(para_lines)
        html_blocks.append('<p>' + _render_inline(para_text) + '</p>')

    if not html_blocks:
        return ""

    return '\n'.join(html_blocks) + '\n'


def _join_paragraph_lines(lines: List[str]) -> str:
    """Join paragraph lines, handling line breaks (rule 17).

    Lines ending with two spaces get a <br> and newline preserved.
    Other lines are joined normally.
    """
    if not lines:
        return ""

    result = []
    for i, line in enumerate(lines):
        # Check if line ends with two spaces
        if line.endswith('  '):
            # Remove the two spaces and add <br> marker
            # We'll process this in _render_inline
            result.append(line.rstrip(' ') + '\x02BR\x02')
        else:
            result.append(line)

    # Join with newlines to preserve structure
    return '\n'.join(result)


def _is_horizontal_line(line: str) -> bool:
    """Check if line is a horizontal line."""
    stripped = line.strip()
    return (stripped == '---' or stripped == '***' or stripped == '___')


def _parse_heading(line: str) -> Tuple[Optional[int], str]:
    """Parse heading level and content. Returns (level, content) or (None, '') if not a heading."""
    match = re.match(r'^(#{1,6})\s+(.*)$', line)
    if not match:
        return None, ''

    level = len(match.group(1))
    content = match.group(2)

    # Remove closing # characters
    content = re.sub(r'\s*#+\s*$', '', content)

    return level, content


def _render_heading(level: int, content: str) -> str:
    """Render heading HTML."""
    rendered = _render_inline(content)
    return f'<h{level}>{rendered}</h{level}>'


def _is_list_start(line: str) -> bool:
    """Check if line starts a list."""
    stripped = line.lstrip()
    # Unordered list
    if re.match(r'^[-*+]\s', stripped):
        return True
    # Ordered list
    if re.match(r'^\d+[.)]\s', stripped):
        return True
    return False


def _get_list_indent(line: str) -> int:
    """Get the indentation level of a list item (number of leading spaces)."""
    return len(line) - len(line.lstrip())


def _is_list_item(line: str) -> bool:
    """Check if line is a list item (unordered or ordered)."""
    stripped = line.lstrip()
    if re.match(r'^[-*+]\s', stripped):
        return True
    if re.match(r'^\d+[.)]\s', stripped):
        return True
    return False


def _get_list_type(line: str) -> Optional[str]:
    """Get list type: 'ul', 'ol', or None."""
    stripped = line.lstrip()
    if re.match(r'^[-*+]\s', stripped):
        return 'ul'
    if re.match(r'^\d+[.)]\s', stripped):
        return 'ol'
    return None


def _extract_list_content(line: str) -> str:
    """Extract content from list item, removing the marker."""
    stripped = line.lstrip()
    # Remove unordered marker
    match = re.match(r'^[-*+]\s+(.*)$', stripped)
    if match:
        return match.group(1)
    # Remove ordered marker
    match = re.match(r'^\d+[.)]\s+(.*)$', stripped)
    if match:
        return match.group(1)
    return stripped


def _process_list(lines: List[str], start_idx: int) -> Tuple[str, int]:
    """Process a list block (unordered or ordered, potentially nested)."""
    list_type = _get_list_type(lines[start_idx])
    if not list_type:
        return '<p>' + _render_inline(lines[start_idx]) + '</p>', start_idx + 1

    # Get the base indentation level
    base_indent = _get_list_indent(lines[start_idx])

    html_parts = [f'<{list_type}>']
    i = start_idx

    while i < len(lines):
        line = lines[i]

        # Empty line ends list
        if not line.strip():
            break

        # Check if this is still a list item
        if not _is_list_item(line):
            break

        current_type = _get_list_type(line)
        current_indent = _get_list_indent(line)

        # Type change ends current list
        if current_type != list_type:
            break

        # If indentation is lower than base, we're back at a higher level, so stop
        if current_indent < base_indent:
            break

        # Process item at base level
        content = _extract_list_content(line)

        # Look ahead for nested items
        if i + 1 < len(lines):
            next_line = lines[i + 1]
            next_indent = _get_list_indent(next_line)

            if _is_list_item(next_line) and next_indent > current_indent:
                # This item will have nested content
                html_parts.append(f'<li>{_render_inline(content)}')

                # Process nested list
                nested_html, next_i = _process_list(lines, i + 1)
                html_parts.append(nested_html)
                html_parts.append('</li>')
                i = next_i
            else:
                html_parts.append(f'<li>{_render_inline(content)}</li>')
                i += 1
        else:
            html_parts.append(f'<li>{_render_inline(content)}</li>')
            i += 1

    html_parts.append(f'</{list_type}>')
    return '\n'.join(html_parts), i


def _process_blockquote(lines: List[str], start_idx: int) -> Tuple[str, int]:
    """Process a blockquote block."""
    blockquote_lines = []
    i = start_idx

    while i < len(lines):
        line = lines[i]

        if not (line.startswith('> ') or line.strip() == '>'):
            break

        # Remove the '> ' prefix
        content = line[1:]
        if content.startswith(' '):
            content = content[1:]

        blockquote_lines.append(content)
        i += 1

    # Render the blockquote content as markdown
    blockquote_text = '\n'.join(blockquote_lines)
    inner_html = render(blockquote_text).rstrip('\n')

    # Split inner_html back into lines for blockquote format
    inner_lines = inner_html.split('\n')

    html_parts = ['<blockquote>']
    html_parts.extend(inner_lines)
    html_parts.append('</blockquote>')

    return '\n'.join(html_parts), i


def _process_code_fence(lines: List[str], start_idx: int) -> Tuple[str, int]:
    """Process a code fence block."""
    fence_line = lines[start_idx].strip()

    # Extract language (if any)
    match = re.match(r'^```(\w*)$', fence_line)
    if not match:
        raise MarkdownError(f"Invalid code fence: {fence_line}")

    language = match.group(1) if match.group(1) else None

    # Find closing fence
    code_lines = []
    i = start_idx + 1

    while i < len(lines):
        if lines[i].strip() == '```':
            # Found closing fence
            code_content = '\n'.join(code_lines)
            escaped_content = _escape_html(code_content)

            if language:
                return f'<pre><code class="language-{language}">{escaped_content}\n</code></pre>', i + 1
            else:
                return f'<pre><code>{escaped_content}\n</code></pre>', i + 1

        code_lines.append(lines[i])
        i += 1

    # No closing fence found
    raise MarkdownError("Unclosed code fence")


def _is_table_start(lines: List[str], idx: int) -> bool:
    """Check if this is the start of a table."""
    if idx + 1 >= len(lines):
        return False

    current_line = lines[idx].strip()
    next_line = lines[idx + 1].strip()

    # Must have pipes and next line must be separator
    if '|' not in current_line or '|' not in next_line:
        return False

    # Check if next line is a valid separator
    return _is_table_separator(next_line)


def _is_table_separator(line: str) -> bool:
    """Check if line is a valid table separator line."""
    # Remove leading/trailing pipes and whitespace
    line = line.strip()
    if line.startswith('|'):
        line = line[1:]
    if line.endswith('|'):
        line = line[:-1]

    # Split by pipe
    cells = [cell.strip() for cell in line.split('|')]

    if not cells:
        return False

    # Each cell must be dashes with optional colons for alignment
    for cell in cells:
        if not cell:
            return False
        # Must match pattern: optional colon, dashes, optional colon
        if not re.match(r'^:?-+:?$', cell):
            return False

    return True


def _parse_table_alignment(cell: str) -> Optional[str]:
    """Parse alignment from separator cell. Returns 'left', 'right', 'center', or None."""
    cell = cell.strip()
    left_colon = cell.startswith(':')
    right_colon = cell.endswith(':')

    if left_colon and right_colon:
        return 'center'
    elif right_colon:
        return 'right'
    elif left_colon:
        return 'left'
    return None


def _process_table(lines: List[str], start_idx: int) -> Tuple[str, int]:
    """Process a markdown table."""
    html_parts = ['<table>']

    # Parse header row
    header_line = lines[start_idx].strip()
    header_cells = _parse_table_row(header_line)

    # Parse separator row to get alignments
    sep_line = lines[start_idx + 1].strip()
    sep_cells = _parse_table_separator_row(sep_line)
    alignments = [_parse_table_alignment(cell) for cell in sep_cells]

    # Build thead
    html_parts.append('<thead>')
    html_parts.append('<tr>')
    for cell, align in zip(header_cells, alignments):
        rendered_cell = _render_inline(cell)
        if align:
            html_parts.append(f'<th style="text-align:{align}">{rendered_cell}</th>')
        else:
            html_parts.append(f'<th>{rendered_cell}</th>')
    html_parts.append('</tr>')
    html_parts.append('</thead>')

    # Build tbody
    html_parts.append('<tbody>')
    i = start_idx + 2

    while i < len(lines):
        line = lines[i].strip()

        # Check if this is still a table row
        if not line or '|' not in line:
            break

        # Check if it looks like a table row (has pipes)
        cells = _parse_table_row(line)

        if len(cells) == 0:
            break

        html_parts.append('<tr>')
        for cell, align in zip(cells, alignments):
            rendered_cell = _render_inline(cell)
            if align:
                html_parts.append(f'<td style="text-align:{align}">{rendered_cell}</td>')
            else:
                html_parts.append(f'<td>{rendered_cell}</td>')
        html_parts.append('</tr>')

        i += 1

    html_parts.append('</tbody>')
    html_parts.append('</table>')

    return '\n'.join(html_parts), i


def _parse_table_row(line: str) -> List[str]:
    """Parse a table row and return list of cell contents."""
    line = line.strip()

    # Remove leading and trailing pipes
    if line.startswith('|'):
        line = line[1:]
    if line.endswith('|'):
        line = line[:-1]

    # Split by pipe and trim whitespace
    cells = [cell.strip() for cell in line.split('|')]
    return cells


def _parse_table_separator_row(line: str) -> List[str]:
    """Parse a table separator row and return list of separator cells."""
    line = line.strip()

    # Remove leading and trailing pipes
    if line.startswith('|'):
        line = line[1:]
    if line.endswith('|'):
        line = line[:-1]

    # Split by pipe and trim whitespace
    cells = [cell.strip() for cell in line.split('|')]
    return cells


def _render_inline(text: str) -> str:
    """Render inline markdown (emphasis, links, etc.)."""
    global _CODE_SPANS

    # Reset code spans storage
    _CODE_SPANS.clear()

    # Process in order: escape HTML, handle escapes, then inline formatting

    # Handle backslash escapes first - mark them to protect from further processing
    text = _handle_backslash_escapes(text)

    # Escape HTML entities (but not in code spans or already escaped text)
    text = _escape_html_except_protected(text)

    # Process line breaks (two spaces at end)
    text = _process_line_breaks(text)

    # Process inline elements - ORDER MATTERS
    text = _process_code_spans(text)
    text = _process_images(text)  # MUST come before links!
    text = _process_links(text)
    text = _process_strong(text)
    text = _process_em(text)
    text = _process_strikethrough(text)

    # Restore code spans after all other formatting
    text = _restore_code_spans(text)

    # Restore escaped characters
    text = _restore_escaped_chars(text)

    return text


def _handle_backslash_escapes(text: str) -> str:
    r"""Handle backslash escapes like \* \# etc."""
    # Replace \X with a placeholder that won't be affected by markdown processing
    result = []
    i = 0
    while i < len(text):
        if i < len(text) - 1 and text[i] == '\\':
            next_char = text[i + 1]
            # Only escape special markdown chars
            if next_char in '*_#[]()!`~<>&\\':
                # Use a special marker for escaped chars - format: \x00\x01{charcode}\x01\x00
                # This format avoids interference with markdown patterns
                result.append(f'\x00\x01{ord(next_char):03d}\x01\x00')
                i += 2
            else:
                result.append(text[i])
                i += 1
        else:
            result.append(text[i])
            i += 1
    return ''.join(result)


def _restore_escaped_chars(text: str) -> str:
    """Restore escaped characters back to their original form."""
    import re
    def replace_escaped(match):
        char_code = int(match.group(1))
        return chr(char_code)
    return re.sub(r'\x00\x01(\d{3})\x01\x00', replace_escaped, text)


def _escape_html_except_protected(text: str) -> str:
    """Escape HTML entities, except for protected escaped chars."""
    # Temporarily replace protected chars
    text = text.replace('&', '\x01AMP\x01')
    text = text.replace('<', '\x01LT\x01')
    text = text.replace('>', '\x01GT\x01')

    # Escape to entities
    text = text.replace('\x01AMP\x01', '&amp;')
    text = text.replace('\x01LT\x01', '&lt;')
    text = text.replace('\x01GT\x01', '&gt;')

    return text


def _escape_html(text: str) -> str:
    """Escape HTML special characters."""
    text = text.replace('&', '&amp;')
    text = text.replace('<', '&lt;')
    text = text.replace('>', '&gt;')
    return text


def _process_line_breaks(text: str) -> str:
    """Process line breaks (two spaces at end of line)."""
    # Replace the BR marker with actual <br> and newline
    text = text.replace('\x02BR\x02', '<br>')
    return text


def _process_code_spans(text: str) -> str:
    """Process inline code spans with backticks and protect from further processing."""
    global _CODE_SPANS

    # Match backtick code spans
    def replace_code(match):
        code = match.group(1)
        # Protect code content from further formatting by using special marker
        code_hash = hashlib.md5(code.encode()).hexdigest()[:8]
        # Store the code content in a global dict for later restoration
        _CODE_SPANS[code_hash] = f'<code>{code}</code>'
        return f'\x03CODE{code_hash}\x03'

    # Use non-greedy matching for code spans
    text = re.sub(r'`([^`]+)`', replace_code, text)
    return text


def _restore_code_spans(text: str) -> str:
    """Restore protected code spans after formatting."""
    for code_hash, code_html in _CODE_SPANS.items():
        text = text.replace(f'\x03CODE{code_hash}\x03', code_html)
    return text


def _process_strong(text: str) -> str:
    """Process **strong** text."""
    def replace_strong(match):
        content = match.group(1)
        # Don't process nested markdown, just preserve content
        return f'<strong>{content}</strong>'

    # Match ** with content that doesn't start or end with space
    # Pattern: ** followed by (non-space, optionally anything, non-space) or (just non-space) followed by **
    text = re.sub(r'\*\*(\S(?:.*?\S)?)\*\*', replace_strong, text)
    return text


def _process_em(text: str) -> str:
    """Process *emphasis* and _emphasis_ text."""
    def replace_em(match):
        content = match.group(1)
        # Don't process nested markdown, just preserve content
        return f'<em>{content}</em>'

    # Match *text* but not **text**
    text = re.sub(r'(?<!\*)\*([^*]+)\*(?!\*)', replace_em, text)
    # Match _text_ but not __text__
    text = re.sub(r'(?<!_)_([^_]+)_(?!_)', replace_em, text)

    return text


def _process_strikethrough(text: str) -> str:
    """Process ~~strikethrough~~ text."""
    def replace_strike(match):
        content = match.group(1)
        # Don't process nested markdown, just preserve content
        return f'<del>{content}</del>'

    text = re.sub(r'~~(.+?)~~', replace_strike, text)
    return text


def _process_links(text: str) -> str:
    """Process [text](url) links."""
    def replace_link(match):
        link_text = match.group(1)
        url = match.group(2)
        # Preserve the link text as-is (already escaped)
        return f'<a href="{url}">{link_text}</a>'

    # Match [text](url)
    text = re.sub(r'\[([^\]]+)\]\(([^)]+)\)', replace_link, text)
    return text


def _process_images(text: str) -> str:
    """Process ![alt](url) images."""
    def replace_image(match):
        alt = match.group(1)
        url = match.group(2)
        return f'<img src="{url}" alt="{alt}">'

    # Match ![alt](url) - must come before link processing
    text = re.sub(r'!\[([^\]]*)\]\(([^)]+)\)', replace_image, text)
    return text


