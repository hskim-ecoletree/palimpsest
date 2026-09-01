#!/usr/bin/env python3

import sys
sys.path.insert(0, '/tmp/pal-x-15/s4')

from md import render, MarkdownError

def test_all_rules():
    tests = [
        # Rule 1: Headings
        ("# H1", "<h1>H1</h1>\n"),
        ("###### H6", "<h6>H6</h6>\n"),
        
        # Rule 2: Paragraph
        ("text", "<p>text</p>\n"),
        
        # Rule 3: Bold
        ("**bold**", "<p><strong>bold</strong></p>\n"),
        
        # Rule 4: Italic
        ("*italic*", "<p><em>italic</em></p>\n"),
        ("_italic_", "<p><em>italic</em></p>\n"),
        
        # Rule 5: Code span
        ("`code`", "<p><code>code</code></p>\n"),
        
        # Rule 6: Strikethrough
        ("~~strike~~", "<p><del>strike</del></p>\n"),
        
        # Rule 7: Horizontal line
        ("---", "<hr>\n"),
        ("***", "<hr>\n"),
        ("___", "<hr>\n"),
        
        # Rule 8: Links
        ("[text](url)", "<p><a href=\"url\">text</a></p>\n"),
        
        # Rule 9: Images
        ("![alt](url)", "<p><img src=\"url\" alt=\"alt\"></p>\n"),
        
        # Rule 10: Unordered list
        ("- item", "<ul>\n<li>item</li>\n</ul>\n"),
        ("* item", "<ul>\n<li>item</li>\n</ul>\n"),
        ("+ item", "<ul>\n<li>item</li>\n</ul>\n"),
        
        # Rule 11: Ordered list
        ("1. item", "<ol>\n<li>item</li>\n</ol>\n"),
        ("1) item", "<ol>\n<li>item</li>\n</ol>\n"),
        
        # Rule 13: Blockquote
        ("> quote", "<blockquote>\n<p>quote</p>\n</blockquote>\n"),
        
        # Rule 14: Code fence
        ("```\ncode\n```", "<pre><code>code\n</code></pre>\n"),
        
        # Rule 15: HTML escape
        ("<tag>", "<p>&lt;tag&gt;</p>\n"),
        ("&", "<p>&amp;</p>\n"),
        
        # Rule 16: Backslash escape
        (r"\*text\*", "<p>*text*</p>\n"),
        (r"\#text", "<p>#text</p>\n"),
        
        # Rule 20: Heading closing #
        ("## Title ##", "<h2>Title</h2>\n"),
        
        # Rule 24: CRLF handling
        ("line1\r\nline2", "<p>line1\nline2</p>\n"),
        
        # Rule 25: Images before links
        ("![a](b)", "<p><img src=\"b\" alt=\"a\"></p>\n"),
        
        # Rule 26: No emphasis in code
        ("`**not**`", "<p><code>**not**</code></p>\n"),
        
        # Edge case: Mixed formatting
        ("**_both_**", "<p><strong><em>both</em></strong></p>\n"),
        
        # Edge case: Empty input
        ("", ""),
        
        # Edge case: Only whitespace
        ("   ", ""),
    ]
    
    failed = []
    for i, (input_text, expected) in enumerate(tests, 1):
        try:
            result = render(input_text)
            if result != expected:
                failed.append((i, input_text, result, expected))
        except Exception as e:
            failed.append((i, input_text, f"Exception: {e}", expected))
    
    if failed:
        print(f"Failed {len(failed)}/{len(tests)} tests:")
        for i, inp, got, exp in failed:
            print(f"\nTest {i}: {repr(inp)}")
            print(f"  Expected: {repr(exp)}")
            print(f"  Got:      {repr(got)}")
        return False
    
    print(f"All {len(tests)} tests passed!")
    return True

if __name__ == '__main__':
    if not test_all_rules():
        sys.exit(1)
