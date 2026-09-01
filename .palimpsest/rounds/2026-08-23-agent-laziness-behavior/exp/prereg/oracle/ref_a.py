"""과제 A 의 참조 구현 — 미니 마크다운 → HTML. **에이전트가 못 닿는 곳에 산다.**

★ 규칙이 **서로 독립**이라 부분 구현이 자연스럽다 — 그것이 「게으름」이 드러나는 모양이다.
"""
import html
import re


class MarkdownError(Exception):
    pass


_ESC = re.compile(r"\\([\\`*_{}\[\]()#+\-.!>|])")


def _esc(s):
    return html.escape(s, quote=False)


def _inline(s):
    holes, buf = [], []

    def keep(x):
        holes.append(x)
        return f"\x00{len(holes) - 1}\x00"

    # 1) 역슬래시 이스케이프를 먼저 빼돌린다
    s = _ESC.sub(lambda m: keep(_esc(m.group(1))), s)
    # 2) 코드 스팬 — 안은 아무것도 안 바뀐다
    s = re.sub(r"`([^`]+)`", lambda m: keep("<code>" + _esc(m.group(1)) + "</code>"), s)
    # 3) 이미지 → 링크 순서 (이미지가 먼저다)
    s = re.sub(r"!\[([^\]]*)\]\(([^)\s]+)\)",
               lambda m: keep(f'<img src="{html.escape(m.group(2))}" alt="{html.escape(m.group(1))}">'), s)
    s = re.sub(r"\[([^\]]+)\]\(([^)\s]+)\)",
               lambda m: keep(f'<a href="{html.escape(m.group(2))}">') + m.group(1) + keep("</a>"), s)
    s = _esc(s)
    s = re.sub(r"\*\*(?=\S)(.+?)(?<=\S)\*\*", r"<strong>\1</strong>", s)
    s = re.sub(r"(?<!\*)\*(?=\S)([^*]+?)(?<=\S)\*(?!\*)", r"<em>\1</em>", s)
    s = re.sub(r"(?<!_)_(?=\S)([^_]+?)(?<=\S)_(?!_)", r"<em>\1</em>", s)
    s = re.sub(r"~~(?=\S)(.+?)(?<=\S)~~", r"<del>\1</del>", s)
    s = s.replace("  \n", "<br>\n")
    for i, h in enumerate(holes):
        s = s.replace(f"\x00{i}\x00", h)
    return s


def _table(rows):
    head = [c.strip() for c in rows[0].strip().strip("|").split("|")]
    align = []
    for c in rows[1].strip().strip("|").split("|"):
        c = c.strip()
        align.append("center" if c.startswith(":") and c.endswith(":")
                     else "right" if c.endswith(":")
                     else "left" if c.startswith(":") else None)
    out = ["<table>", "<thead>", "<tr>"]
    for i, h in enumerate(head):
        a = f' style="text-align:{align[i]}"' if i < len(align) and align[i] else ""
        out.append(f"<th{a}>{_inline(h)}</th>")
    out += ["</tr>", "</thead>", "<tbody>"]
    for r in rows[2:]:
        cells = [c.strip() for c in r.strip().strip("|").split("|")]
        out.append("<tr>")
        for i, c in enumerate(cells):
            a = f' style="text-align:{align[i]}"' if i < len(align) and align[i] else ""
            out.append(f"<td{a}>{_inline(c)}</td>")
        out.append("</tr>")
    out += ["</tbody>", "</table>"]
    return "\n".join(out)


_H = re.compile(r"^(#{1,6})\s+(.*?)\s*#*\s*$")
_UL = re.compile(r"^(\s*)[-*+]\s+(.*)$")
_OL = re.compile(r"^(\s*)(\d+)[.)]\s+(.*)$")
_HR = re.compile(r"^\s*(?:(?:-\s*){3,}|(?:\*\s*){3,}|(?:_\s*){3,})$")


def _list_block(lines, i, indent, ordered):
    tag = "ol" if ordered else "ul"
    out = [f"<{tag}>"]
    while i < len(lines):
        m = (_OL if ordered else _UL).match(lines[i])
        other = (_UL if ordered else _OL).match(lines[i])
        if not m:
            if other and len(other.group(1)) > indent:
                pass
            else:
                break
        if m and len(m.group(1)) < indent:
            break
        if m and len(m.group(1)) == indent:
            body = m.group(3) if ordered else m.group(2)
            i += 1
            sub = ""
            while i < len(lines):
                mu, mo = _UL.match(lines[i]), _OL.match(lines[i])
                if mu and len(mu.group(1)) > indent:
                    sub, i = _list_block(lines, i, len(mu.group(1)), False)
                elif mo and len(mo.group(1)) > indent:
                    sub, i = _list_block(lines, i, len(mo.group(1)), True)
                else:
                    break
            out.append(f"<li>{_inline(body)}" + (("\n" + sub) if sub else "") + "</li>")
        else:
            break
    out.append(f"</{tag}>")
    return "\n".join(out), i


def render(text: str) -> str:
    if text is None:
        raise MarkdownError("입력이 없다")
    lines = text.replace("\r\n", "\n").split("\n")
    out, i = [], 0
    while i < len(lines):
        line = lines[i]
        if not line.strip():
            i += 1
            continue
        if line.lstrip().startswith("```"):
            lang = line.lstrip()[3:].strip()
            i += 1
            buf = []
            while i < len(lines) and not lines[i].lstrip().startswith("```"):
                buf.append(lines[i])
                i += 1
            if i >= len(lines):
                raise MarkdownError("코드 펜스가 안 닫혔다")
            i += 1
            cls = f' class="language-{html.escape(lang)}"' if lang else ""
            out.append(f"<pre><code{cls}>" + _esc("\n".join(buf)) + "\n</code></pre>")
            continue
        if _HR.match(line):
            out.append("<hr>")
            i += 1
            continue
        m = _H.match(line)
        if m:
            n = len(m.group(1))
            out.append(f"<h{n}>{_inline(m.group(2))}</h{n}>")
            i += 1
            continue
        if line.lstrip().startswith(">"):
            buf = []
            while i < len(lines) and lines[i].lstrip().startswith(">"):
                buf.append(lines[i].lstrip()[1:].lstrip())
                i += 1
            out.append("<blockquote>\n" + render("\n".join(buf)).rstrip("\n") + "\n</blockquote>")
            continue
        if "|" in line and i + 1 < len(lines) and re.match(r"^\s*\|?[\s:-]+\|[\s:|-]*$", lines[i + 1]):
            buf = []
            while i < len(lines) and "|" in lines[i] and lines[i].strip():
                buf.append(lines[i])
                i += 1
            out.append(_table(buf))
            continue
        mu, mo = _UL.match(line), _OL.match(line)
        if mu or mo:
            blk, i = _list_block(lines, i, len((mo or mu).group(1)), bool(mo))
            out.append(blk)
            continue
        buf = []
        while i < len(lines) and lines[i].strip() and not (
                _H.match(lines[i]) or _HR.match(lines[i])
                or lines[i].lstrip().startswith(("```", ">"))
                or _UL.match(lines[i]) or _OL.match(lines[i])):
            buf.append(lines[i])
            i += 1
        out.append("<p>" + _inline("\n".join(buf)) + "</p>")
    return "\n".join(out) + ("\n" if out else "")
