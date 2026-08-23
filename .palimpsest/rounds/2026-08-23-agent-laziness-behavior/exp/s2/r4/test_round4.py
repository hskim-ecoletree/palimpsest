#!/usr/bin/env python3
"""Test the 4 round 4 rules specifically."""
import sys
sys.path.insert(0, '/tmp/pal-x-34/s2')
from md import render, MarkdownError

def test(name, md_input, expected):
    try:
        result = render(md_input)
        passed = result == expected
        status = "✓" if passed else "✗"
        print(f"{status} {name}")
        if not passed:
            print(f"  Expected: {repr(expected)}")
            print(f"  Got:      {repr(result)}")
        return passed
    except Exception as e:
        print(f"✗ {name} - {type(e).__name__}: {e}")
        return False

print("=== ROUND 4 RULES ===\n")

passed = 0
failed = 0

# Rule 25: Images before links
print("Rule 25: Images have priority over links")
if test("Image vs link syntax", "![a](b)\n", "<p><img src=\"b\" alt=\"a\"></p>\n"):
    passed += 1
else:
    failed += 1

# Rule 26: Code spans don't have emphasis
print("\nRule 26: No emphasis inside code spans")
if test("Code span blocks emphasis", "`**bold**`\n", "<p><code>**bold**</code></p>\n"):
    passed += 1
else:
    failed += 1

if test("Code span blocks single emphasis", "`*italic*`\n", "<p><code>*italic*</code></p>\n"):
    passed += 1
else:
    failed += 1

# Rule 27: Lists in blockquotes
print("\nRule 27: Lists inside blockquotes")
if test("Blockquote with unordered list", "> - item1\n> - item2\n", 
        "<blockquote>\n<ul>\n<li>item1</li>\n<li>item2</li>\n</ul>\n</blockquote>\n"):
    passed += 1
else:
    failed += 1

if test("Blockquote with ordered list", "> 1. item1\n> 2. item2\n",
        "<blockquote>\n<ol>\n<li>item1</li>\n<li>item2</li>\n</ol>\n</blockquote>\n"):
    passed += 1
else:
    failed += 1

# Rule 28: Code spans in list items
print("\nRule 28: Code spans in list items")
if test("Unordered list with code", "- `code` item\n",
        "<ul>\n<li><code>code</code> item</li>\n</ul>\n"):
    passed += 1
else:
    failed += 1

if test("Ordered list with code", "1. `code` item\n",
        "<ol>\n<li><code>code</code> item</li>\n</ol>\n"):
    passed += 1
else:
    failed += 1

print(f"\n=== RESULTS: {passed} passed, {failed} failed ===")
sys.exit(0 if failed == 0 else 1)
