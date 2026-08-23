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
            print(f"  Expected: {repr(expected)}")
            print(f"  Got:      {repr(result)}")
    except Exception as e:
        print(f"ERROR: {name}: {type(e).__name__}: {e}")

print("--- Edge cases ---")

# Emphasis edge cases
test("Emphasis not at word boundary", "word*not*emphasis", "<p>word*not*emphasis</p>\n")
test("Multiple emphasis on line", "*a* and *b*", "<p><em>a</em> and <em>b</em></p>\n")

# Code span edge cases
test("Code span with escaped backticks", "`a` `b`", "<p><code>a</code> <code>b</code></p>\n")

# Link edge cases
test("Link with emphasis inside", "[**bold**](url)", "<p><a href=\"url\"><strong>bold</strong></a></p>\n")
test("Link with code inside", "[`code`](url)", "<p><a href=\"url\"><code>code</code></a></p>\n")

# Nested emphasis
test("Nested strong and em", "***both***", "<p><em><strong>both</strong></em></p>\n")

# List continuation
test("List with multiple items", "- a\n- b\n", "<ul>\n<li>a</li>\n<li>b</li>\n</ul>\n")

# Blockquote with multiple lines
test("Blockquote multi-line", "> line1\n> line2\n", "<blockquote>\n<p>line1</p>\n<p>line2</p>\n</blockquote>\n")

# Heading level 7 (should fail)
print("\nTesting H7 (should create paragraph with #)")
test("Heading level 7", "####### Too many\n", "<p>####### Too many</p>\n")

# Mixed content
test("Paragraph with multiple inline formats", "**bold** and *italic* and `code`\n", "<p><strong>bold</strong> and <em>italic</em> and <code>code</code></p>\n")

# Table with spaces
test("Table with varied spacing", "|  H1  |  H2  |\n|  ---  |  ---  |\n|  C1  |  C2  |\n", "<table>\n<thead>\n<tr>\n<th>H1</th>\n<th>H2</th>\n</tr>\n</thead>\n<tbody>\n<tr>\n<td>C1</td>\n<td>C2</td>\n</tr>\n</tbody>\n</table>\n")

# Escape sequences
test("Escaped bracket", r"\[not a link\]" + "(url)\n", "<p>[not a link](url)</p>\n")

# Line break edge case
test("Line break in paragraph", "line1  \nline2\n", "<p>line1<br>\nline2</p>\n")

# Complex nested list
test("Complex nested list", "- a\n  - b\n  - c\n- d\n", "<ul>\n<li>a\n<ul>\n<li>b</li>\n<li>c</li>\n</ul>\n</li>\n<li>d</li>\n</ul>\n")

# Blockquote with nested markdown
test("Blockquote with **bold**", "> **bold** text\n", "<blockquote>\n<p><strong>bold</strong> text</p>\n</blockquote>\n")

# HR edge cases (should not be confused with emphasis)
test("HR not confused with emphasis", "text ***\n", "<p>text ***</p>\n")
test("HR with no spaces before", "***\n", "<hr>\n")

# Multiple paragraphs
test("Three paragraphs", "a\n\nb\n\nc\n", "<p>a</p>\n<p>b</p>\n<p>c</p>\n")

print("\nAll edge case tests completed!")
