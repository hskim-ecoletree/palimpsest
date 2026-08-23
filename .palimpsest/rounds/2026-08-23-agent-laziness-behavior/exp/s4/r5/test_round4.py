#!/usr/bin/env python3

import sys
sys.path.insert(0, '/tmp/pal-x-15/s4')

from md import render, MarkdownError

def test_rule_25():
    """Test: Images come before links"""
    # ![a](b) should be recognized as an image, not a link
    result = render("![a](b)")
    # A standalone image in paragraph context
    assert '<img src="b" alt="a">' in result, f"Rule 25 failed: got {repr(result)}"
    assert '<a' not in result, f"Rule 25 failed: should not have link tags"
    print("✓ Rule 25 passed: ![a](b) is recognized as an image")

def test_rule_26():
    """Test: Emphasis doesn't work inside code spans"""
    result = render("`**not bold**`")
    expected = '<p><code>**not bold**</code></p>\n'
    assert result == expected, f"Rule 26 failed: got {repr(result)}, expected {repr(expected)}"
    print("✓ Rule 26 passed: Emphasis doesn't work in code spans")

def test_rule_27():
    """Test: Lists can appear inside blockquotes"""
    text = """> - item 1
> - item 2"""
    result = render(text)
    # Should have blockquote with ul inside
    assert '<blockquote>' in result and '<ul>' in result, f"Rule 27 failed: got {repr(result)}"
    assert '<li>item 1</li>' in result and '<li>item 2</li>' in result, f"Rule 27 failed: list items not found"
    print("✓ Rule 27 passed: Lists work inside blockquotes")

def test_rule_28():
    """Test: Code spans can appear inside list items"""
    text = """- item with `code` inside"""
    result = render(text)
    assert '<code>code</code>' in result, f"Rule 28 failed: code span not found in {repr(result)}"
    assert '<li>item with <code>code</code> inside</li>' in result or 'item with <code>code</code> inside' in result, f"Rule 28 failed: got {repr(result)}"
    print("✓ Rule 28 passed: Code spans work in list items")

if __name__ == '__main__':
    try:
        test_rule_25()
        test_rule_26()
        test_rule_27()
        test_rule_28()
        print("\nAll round 4 rules passed!")
    except AssertionError as e:
        print(f"Test failed: {e}")
        sys.exit(1)
