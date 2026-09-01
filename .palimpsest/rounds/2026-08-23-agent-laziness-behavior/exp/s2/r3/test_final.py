#!/usr/bin/env python3
"""Final verification test for all 24 markdown rules."""
import sys
sys.path.insert(0, '/tmp/pal-x-34/s2')
from md import render, MarkdownError

tests_passed = 0
tests_failed = 0

def test(name, md_input, expected):
    global tests_passed, tests_failed
    try:
        result = render(md_input)
        if result == expected:
            tests_passed += 1
            print(f"✓ {name}")
            return True
        else:
            tests_failed += 1
            print(f"✗ {name}")
            print(f"  Expected: {repr(expected)}")
            print(f"  Got:      {repr(result)}")
            return False
    except Exception as e:
        tests_failed += 1
        print(f"✗ {name} - {type(e).__name__}: {e}")
        return False

def test_error(name, md_input, error_type):
    global tests_passed, tests_failed
    try:
        result = render(md_input)
        tests_failed += 1
        print(f"✗ {name} - No exception raised")
        return False
    except error_type:
        tests_passed += 1
        print(f"✓ {name}")
        return True
    except Exception as e:
        tests_failed += 1
        print(f"✗ {name} - Wrong exception: {type(e).__name__}")
        return False

print("=" * 60)
print("MARKDOWN RENDERING VERIFICATION TEST")
print("=" * 60)

print("\n1. HEADINGS (Rule 1)")
test("H1", "# 제목\n", "<h1>제목</h1>\n")
test("H2", "## 제목\n", "<h2>제목</h2>\n")
test("H3", "### 제목\n", "<h3>제목</h3>\n")
test("H4", "#### 제목\n", "<h4>제목</h4>\n")
test("H5", "##### 제목\n", "<h5>제목</h5>\n")
test("H6", "###### 제목\n", "<h6>제목</h6>\n")
test("H7 (invalid - should be paragraph)", "####### Too many\n", "<p>####### Too many</p>\n")

print("\n2. PARAGRAPHS (Rule 2)")
test("Simple paragraph", "보통 글\n", "<p>보통 글</p>\n")
test("Paragraph with multiple words", "여러 단어 문단\n", "<p>여러 단어 문단</p>\n")

print("\n3. EMPHASIS AND FORMATTING")
test("Strong/Bold (Rule 3)", "**굵게**\n", "<p><strong>굵게</strong></p>\n")
test("Emphasis with * (Rule 4)", "*글*\n", "<p><em>글</em></p>\n")
test("Emphasis with _ (Rule 4)", "_글_\n", "<p><em>글</em></p>\n")
test("Code span (Rule 5)", "`코드`\n", "<p><code>코드</code></p>\n")
test("Strikethrough (Rule 6)", "~~글~~\n", "<p><del>글</del></p>\n")

print("\n4. HORIZONTAL RULES (Rule 7)")
test("HR with ---", "---\n", "<hr>\n")
test("HR with ***", "***\n", "<hr>\n")
test("HR with ___", "___\n", "<hr>\n")

print("\n5. LINKS AND IMAGES (Rule 8, 9)")
test("Link", "[글자](주소)\n", "<p><a href=\"주소\">글자</a></p>\n")
test("Image", "![대체](주소)\n", "<p><img src=\"주소\" alt=\"대체\"></p>\n")
test("Link with formatting", "[**bold**](url)\n", "<p><a href=\"url\"><strong>bold</strong></a></p>\n")

print("\n6. LISTS (Rule 10, 11, 12)")
test("Unordered list -", "- 항목\n", "<ul>\n<li>항목</li>\n</ul>\n")
test("Unordered list *", "* 항목\n", "<ul>\n<li>항목</li>\n</ul>\n")
test("Unordered list +", "+ 항목\n", "<ul>\n<li>항목</li>\n</ul>\n")
test("Ordered list 1.", "1. 항목\n", "<ol>\n<li>항목</li>\n</ol>\n")
test("Ordered list 1)", "1) 항목\n", "<ol>\n<li>항목</li>\n</ol>\n")
test("Nested list", "- 항목1\n  - 항목2\n", "<ul>\n<li>항목1\n<ul>\n<li>항목2</li>\n</ul>\n</li>\n</ul>\n")

print("\n7. BLOCKQUOTES (Rule 13)")
test("Simple blockquote", "> 글\n", "<blockquote>\n<p>글</p>\n</blockquote>\n")
test("Blockquote with formatting", "> **bold** text\n", "<blockquote>\n<p><strong>bold</strong> text</p>\n</blockquote>\n")

print("\n8. CODE BLOCKS (Rule 14)")
test("Code fence", "```\ncode\n```\n", "<pre><code>code\n</code></pre>\n")
test("Code fence with language", "```python\ncode\n```\n", '<pre><code class="language-python">code\n</code></pre>\n')

print("\n9. ESCAPING (Rule 15, 16)")
test("HTML escape in code", "`<>`\n", "<p><code>&lt;&gt;</code></p>\n")
test("HTML escape in fence", "```\n<>\n```\n", "<pre><code>&lt;&gt;\n</code></pre>\n")
test("Backslash escape *", r"\*" + "\n", "<p>*</p>\n")
test("Backslash escape #", r"\#" + "\n", "<p>#</p>\n")
test("Backslash escape [", r"\[" + "\n", "<p>[</p>\n")

print("\n10. LINE BREAKS (Rule 17)")
test("Line break with spaces", "line1  \nline2\n", "<p>line1<br>\nline2</p>\n")

print("\n11. PARAGRAPH SEPARATION (Rule 18)")
test("Empty line breaks paragraphs", "a\n\nb\n", "<p>a</p>\n<p>b</p>\n")
test("Three paragraphs", "a\n\nb\n\nc\n", "<p>a</p>\n<p>b</p>\n<p>c</p>\n")

print("\n12. CODE FENCE VALIDATION (Rule 19)")
test_error("Unclosed fence error", "```\ncode\n", MarkdownError)

print("\n13. HEADING CLOSING # (Rule 20)")
test("Closing # removed", "## 제목 ##\n", "<h2>제목</h2>\n")

print("\n14. TABLES (Rule 21, 22, 23)")
test("Simple table", "| H1 | H2 |\n| --- | --- |\n| C1 | C2 |\n", "<table>\n<thead>\n<tr>\n<th>H1</th>\n<th>H2</th>\n</tr>\n</thead>\n<tbody>\n<tr>\n<td>C1</td>\n<td>C2</td>\n</tr>\n</tbody>\n</table>\n")
test("Table alignment left", "| Left |\n| :-- |\n| C1 |\n", '<table>\n<thead>\n<tr>\n<th>Left</th>\n</tr>\n</thead>\n<tbody>\n<tr>\n<td style="text-align:left">C1</td>\n</tr>\n</tbody>\n</table>\n')
test("Table alignment right", "| Right |\n| --: |\n| C1 |\n", '<table>\n<thead>\n<tr>\n<th>Right</th>\n</tr>\n</thead>\n<tbody>\n<tr>\n<td style="text-align:right">C1</td>\n</tr>\n</tbody>\n</table>\n')
test("Table alignment center", "| Center |\n| :-: |\n| C1 |\n", '<table>\n<thead>\n<tr>\n<th>Center</th>\n</tr>\n</thead>\n<tbody>\n<tr>\n<td style="text-align:center">C1</td>\n</tr>\n</tbody>\n</table>\n')
test("Table with inline formatting", "| **Bold** |\n| --- |\n| `code` |\n", "<table>\n<thead>\n<tr>\n<th><strong>Bold</strong></th>\n</tr>\n</thead>\n<tbody>\n<tr>\n<td><code>code</code></td>\n</tr>\n</tbody>\n</table>\n")

print("\n15. LINE ENDING HANDLING (Rule 24)")
test("CRLF normalization", "text\r\n", "<p>text</p>\n")
test("Mixed line endings with break", "a\r\n\r\nb\n", "<p>a</p>\n<p>b</p>\n")

print("\n16. SPECIAL CASES")
test("Empty input", "", "")
test("Multiple spaces in text", "a   b\n", "<p>a   b</p>\n")

print("\n" + "=" * 60)
print(f"RESULTS: {tests_passed} passed, {tests_failed} failed")
print("=" * 60)

if tests_failed == 0:
    print("✓ ALL TESTS PASSED!")
    sys.exit(0)
else:
    print(f"✗ {tests_failed} test(s) failed")
    sys.exit(1)
