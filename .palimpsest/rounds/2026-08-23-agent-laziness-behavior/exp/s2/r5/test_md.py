#!/usr/bin/env python3
import sys
sys.path.insert(0, '/tmp/pal-x-34/s2')
from md import render, MarkdownError

def test(name, md_input, expected):
    try:
        result = render(md_input)
        passed = result == expected
        status = "PASS" if passed else "FAIL"
        print(f"{status}: {name}")
        if not passed:
            print(f"  Input: {repr(md_input)}")
            print(f"  Expected: {repr(expected)}")
            print(f"  Got:      {repr(result)}")
    except Exception as e:
        print(f"ERROR: {name}")
        print(f"  Exception: {type(e).__name__}: {e}")

# Test each rule
test("Rule 1 - H1", "# 제목\n", "<h1>제목</h1>\n")
test("Rule 1 - H6", "###### 제목\n", "<h6>제목</h6>\n")
test("Rule 2 - Paragraph", "보통 글\n", "<p>보통 글</p>\n")
test("Rule 3 - Strong", "**굵게**\n", "<p><strong>굵게</strong></p>\n")
test("Rule 4 - Emphasis *", "*글*\n", "<p><em>글</em></p>\n")
test("Rule 4 - Emphasis _", "_글_\n", "<p><em>글</em></p>\n")
test("Rule 5 - Code span", "`코드`\n", "<p><code>코드</code></p>\n")
test("Rule 6 - Strikethrough", "~~글~~\n", "<p><del>글</del></p>\n")
test("Rule 7 - HR ---", "---\n", "<hr>\n")
test("Rule 7 - HR ***", "***\n", "<hr>\n")
test("Rule 7 - HR ___", "___\n", "<hr>\n")
test("Rule 8 - Link", "[글자](주소)\n", "<p><a href=\"주소\">글자</a></p>\n")
test("Rule 9 - Image", "![대체](주소)\n", "<p><img src=\"주소\" alt=\"대체\"></p>\n")
test("Rule 10 - UL -", "- 항목\n", "<ul>\n<li>항목</li>\n</ul>\n")
test("Rule 10 - UL *", "* 항목\n", "<ul>\n<li>항목</li>\n</ul>\n")
test("Rule 10 - UL +", "+ 항목\n", "<ul>\n<li>항목</li>\n</ul>\n")
test("Rule 11 - OL .", "1. 항목\n", "<ol>\n<li>항목</li>\n</ol>\n")
test("Rule 11 - OL )", "1) 항목\n", "<ol>\n<li>항목</li>\n</ol>\n")
test("Rule 13 - Blockquote", "> 글\n", "<blockquote>\n<p>글</p>\n</blockquote>\n")
test("Rule 14 - Code fence", "```\ncode\n```\n", "<pre><code>code\n</code></pre>\n")
test("Rule 14 - Code fence with lang", "```python\ncode\n```\n", '<pre><code class="language-python">code\n</code></pre>\n')
test("Rule 15 - HTML escape in code span", "`<>`\n", "<p><code>&lt;&gt;</code></p>\n")
test("Rule 15 - HTML escape in fence", "```\n<>\n```\n", "<pre><code>&lt;&gt;\n</code></pre>\n")
test("Rule 16 - Backslash escape star", r"\*" + "\n", "<p>*</p>\n")
test("Rule 16 - Backslash escape hash", r"\#" + "\n", "<p>#</p>\n")
test("Rule 17 - Line break", "text  \nmore\n", "<p>text<br>\nmore</p>\n")
test("Rule 18 - Empty line paragraph break", "a\n\nb\n", "<p>a</p>\n<p>b</p>\n")
test("Rule 20 - Heading closing #", "## 제목 ##\n", "<h2>제목</h2>\n")
test("Rule 24 - CRLF handling", "text\r\n", "<p>text</p>\n")
test("Empty input", "", "")
test("Rule 12 - Nested list", "- 항목1\n  - 항목1.1\n", "<ul>\n<li>항목1\n<ul>\n<li>항목1.1</li>\n</ul>\n</li>\n</ul>\n")

# Unclosed fence test
try:
    render("```\ncode\n")
    print("FAIL: Rule 19 - Unclosed fence error - no exception raised")
except MarkdownError:
    print("PASS: Rule 19 - Unclosed fence error")
except Exception as e:
    print(f"FAIL: Rule 19 - Unclosed fence error - wrong exception: {type(e).__name__}: {e}")

# Table tests
print("\n--- Table tests (Rules 21-23) ---")
test("Rule 21 - Simple table", "| H1 | H2 |\n| --- | --- |\n| C1 | C2 |\n", "<table>\n<thead>\n<tr>\n<th>H1</th>\n<th>H2</th>\n</tr>\n</thead>\n<tbody>\n<tr>\n<td>C1</td>\n<td>C2</td>\n</tr>\n</tbody>\n</table>\n")
test("Rule 22 - Table alignment left", "| Left |\n| :-- |\n| C1 |\n", '<table>\n<thead>\n<tr>\n<th>Left</th>\n</tr>\n</thead>\n<tbody>\n<tr>\n<td style="text-align:left">C1</td>\n</tr>\n</tbody>\n</table>\n')
test("Rule 22 - Table alignment right", "| Right |\n| --: |\n| C1 |\n", '<table>\n<thead>\n<tr>\n<th>Right</th>\n</tr>\n</thead>\n<tbody>\n<tr>\n<td style="text-align:right">C1</td>\n</tr>\n</tbody>\n</table>\n')
test("Rule 22 - Table alignment center", "| Center |\n| :-: |\n| C1 |\n", '<table>\n<thead>\n<tr>\n<th>Center</th>\n</tr>\n</thead>\n<tbody>\n<tr>\n<td style="text-align:center">C1</td>\n</tr>\n</tbody>\n</table>\n')
test("Rule 23 - Table with inline formatting", "| **Bold** |\n| --- |\n| `code` |\n", "<table>\n<thead>\n<tr>\n<th><strong>Bold</strong></th>\n</tr>\n</thead>\n<tbody>\n<tr>\n<td><code>code</code></td>\n</tr>\n</tbody>\n</table>\n")
