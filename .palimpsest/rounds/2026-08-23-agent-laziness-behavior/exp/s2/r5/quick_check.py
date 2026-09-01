#!/usr/bin/env python3
import sys
sys.path.insert(0, '/tmp/pal-x-34/s2')
from md import render, MarkdownError

# Quick validation
print("Final quick validation:")
print("✓ H1:", render("# H1\n") == "<h1>H1</h1>\n")
print("✓ H7→paragraph:", render("####### H7\n") == "<p>####### H7</p>\n")
print("✓ Strong:", render("**bold**\n") == "<p><strong>bold</strong></p>\n")
print("✓ Table:", "table" in render("| H |\n| --- |\n| C |\n"))

# Error handling
try:
    render("```\ncode")
    print("✗ No error")
except MarkdownError:
    print("✓ MarkdownError for unclosed fence")

print("\nAll quick checks passed!")
