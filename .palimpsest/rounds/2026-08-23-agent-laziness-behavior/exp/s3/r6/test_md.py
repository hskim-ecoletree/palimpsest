"""마크다운 렌더러 테스트."""

from md import render, MarkdownError
import sys

def test(name, markdown, expected):
    """테스트 케이스를 실행한다."""
    try:
        result = render(markdown)
        if result == expected:
            print(f"✓ {name}")
            return True
        else:
            print(f"✗ {name}")
            print(f"  입력: {repr(markdown)}")
            print(f"  예상: {repr(expected)}")
            print(f"  결과: {repr(result)}")
            return False
    except Exception as e:
        print(f"✗ {name} - 예외: {e}")
        return False

def test_error(name, markdown, error_type):
    """예외가 나는 테스트."""
    try:
        result = render(markdown)
        print(f"✗ {name} - 예외 없음")
        return False
    except error_type:
        print(f"✓ {name}")
        return True
    except Exception as e:
        print(f"✗ {name} - 잘못된 예외: {type(e).__name__}")
        return False

tests_passed = 0
tests_total = 0

# 규칙 1: 헤딩 여섯 단계
tests_total += 1
if test("Rule 1a: h1", "# 제목", "<h1>제목</h1>\n"):
    tests_passed += 1
tests_total += 1
if test("Rule 1b: h6", "###### 제목", "<h6>제목</h6>\n"):
    tests_passed += 1

# 규칙 2: 문단
tests_total += 1
if test("Rule 2: paragraph", "보통 글", "<p>보통 글</p>\n"):
    tests_passed += 1

# 규칙 3: 강조
tests_total += 1
if test("Rule 3: strong", "**굵게**", "<p><strong>굵게</strong></p>\n"):
    tests_passed += 1

# 규칙 4: 기울임
tests_total += 1
if test("Rule 4a: em *", "*글*", "<p><em>글</em></p>\n"):
    tests_passed += 1
tests_total += 1
if test("Rule 4b: em _", "_글_", "<p><em>글</em></p>\n"):
    tests_passed += 1

# 규칙 5: 코드 스팬
tests_total += 1
if test("Rule 5: code span", "`코드`", "<p><code>코드</code></p>\n"):
    tests_passed += 1

# 규칙 6: 취소선
tests_total += 1
if test("Rule 6: del", "~~글~~", "<p><del>글</del></p>\n"):
    tests_passed += 1

# 규칙 7: 수평선
tests_total += 1
if test("Rule 7a: hr ---", "---", "<hr>\n"):
    tests_passed += 1
tests_total += 1
if test("Rule 7b: hr ***", "***", "<hr>\n"):
    tests_passed += 1
tests_total += 1
if test("Rule 7c: hr ___", "___", "<hr>\n"):
    tests_passed += 1

# 규칙 8: 링크
tests_total += 1
if test("Rule 8: link", "[글자](주소)", "<p><a href=\"주소\">글자</a></p>\n"):
    tests_passed += 1

# 규칙 9: 이미지
tests_total += 1
if test("Rule 9: image", "![대체](주소)", "<p><img src=\"주소\" alt=\"대체\"></p>\n"):
    tests_passed += 1

# 규칙 10: 순서 없는 목록
tests_total += 1
if test("Rule 10a: ul -", "- 항목", "<ul>\n<li>항목</li>\n</ul>\n"):
    tests_passed += 1
tests_total += 1
if test("Rule 10b: ul *", "* 항목", "<ul>\n<li>항목</li>\n</ul>\n"):
    tests_passed += 1
tests_total += 1
if test("Rule 10c: ul +", "+ 항목", "<ul>\n<li>항목</li>\n</ul>\n"):
    tests_passed += 1

# 규칙 11: 순서 있는 목록
tests_total += 1
if test("Rule 11a: ol .", "1. 항목", "<ol>\n<li>항목</li>\n</ol>\n"):
    tests_passed += 1
tests_total += 1
if test("Rule 11b: ol )", "1) 항목", "<ol>\n<li>항목</li>\n</ol>\n"):
    tests_passed += 1

# 규칙 12: 중첩 목록
tests_total += 1
if test("Rule 12: nested list", "- 외\n  - 안", "<ul>\n<li>외\n<ul>\n<li>안</li>\n</ul>\n</li>\n</ul>\n"):
    tests_passed += 1

# 규칙 13: 인용문
tests_total += 1
if test("Rule 13: blockquote", "> 글", "<blockquote>\n<p>글</p>\n</blockquote>\n"):
    tests_passed += 1

# 규칙 14: 코드 펜스
tests_total += 1
if test("Rule 14a: code fence", "```\n코드\n```", "<pre><code>코드\n</code></pre>\n"):
    tests_passed += 1
tests_total += 1
if test("Rule 14b: code fence with lang", "```python\ncode\n```", '<pre><code class="language-python">code\n</code></pre>\n'):
    tests_passed += 1

# 규칙 15: HTML 이스케이프
tests_total += 1
if test("Rule 15a: escape <", "<태그>", "<p>&lt;태그&gt;</p>\n"):
    tests_passed += 1
tests_total += 1
if test("Rule 15b: escape &", "a&b", "<p>a&amp;b</p>\n"):
    tests_passed += 1
tests_total += 1
if test("Rule 15c: escape in code", "`<tag>`", "<p><code>&lt;tag&gt;</code></p>\n"):
    tests_passed += 1

# 규칙 16: 역슬래시 이스케이프
tests_total += 1
if test("Rule 16: backslash", r"\*별이 아님\*", "<p>*별이 아님*</p>\n"):
    tests_passed += 1

# 규칙 17: 줄바꿈
tests_total += 1
if test("Rule 17: line break", "줄1  \n줄2", "<p>줄1<br>\n줄2</p>\n"):
    tests_passed += 1

# 규칙 18: 빈 줄이 문단을 나눈다
tests_total += 1
if test("Rule 18: blank line", "문단1\n\n문단2", "<p>문단1</p>\n<p>문단2</p>\n"):
    tests_passed += 1

# 규칙 19: 안 닫힌 코드 펜스
tests_total += 1
if test_error("Rule 19: unclosed fence", "```\n코드", MarkdownError):
    tests_passed += 1

# 규칙 20: 헤딩 뒤의 닫는 #
tests_total += 1
if test("Rule 20: closing #", "## 제목 ##", "<h2>제목</h2>\n"):
    tests_passed += 1

# 규칙 21: 표
tests_total += 1
table_input = "| a | b |\n| --- | --- |\n| 1 | 2 |"
table_expected = "<table>\n<thead>\n<tr>\n<th>a</th>\n<th>b</th>\n</tr>\n</thead>\n<tbody>\n<tr>\n<td>1</td>\n<td>2</td>\n</tr>\n</tbody>\n</table>\n"
if test("Rule 21: table", table_input, table_expected):
    tests_passed += 1

# 규칙 22: 정렬
tests_total += 1
align_input = "| 좌 | 중 | 우 |\n| :--- | :---: | ---: |\n| L | C | R |"
align_expected = '<table>\n<thead>\n<tr>\n<th style="text-align:left">좌</th>\n<th style="text-align:center">중</th>\n<th style="text-align:right">우</th>\n</tr>\n</thead>\n<tbody>\n<tr>\n<td style="text-align:left">L</td>\n<td style="text-align:center">C</td>\n<td style="text-align:right">R</td>\n</tr>\n</tbody>\n</table>\n'
if test("Rule 22: alignment", align_input, align_expected):
    tests_passed += 1

# 규칙 23: 표 셀 안에 인라인 서식
tests_total += 1
cell_input = "| a |\n| --- |\n| **굵게** |"
cell_expected = "<table>\n<thead>\n<tr>\n<th>a</th>\n</tr>\n</thead>\n<tbody>\n<tr>\n<td><strong>굵게</strong></td>\n</tr>\n</tbody>\n</table>\n"
if test("Rule 23: cell inline", cell_input, cell_expected):
    tests_passed += 1

# 규칙 24: \r\n
tests_total += 1
if test("Rule 24: CRLF", "줄1\r\n줄2", "<p>줄1\n줄2</p>\n"):
    tests_passed += 1

# 규칙 25: 이미지가 링크보다 먼저
tests_total += 1
if test("Rule 25: image before link", "![a](b)", "<p><img src=\"b\" alt=\"a\"></p>\n"):
    tests_passed += 1

# 규칙 26: 코드 스팬 안에서는 강조가 안 먹는다
tests_total += 1
if test("Rule 26: no emphasis in code", "`**굵게 아님**`", "<p><code>**굵게 아님**</code></p>\n"):
    tests_passed += 1

# 규칙 27: 인용 안에 목록
tests_total += 1
quote_list = "> - 항목"
quote_list_expected = "<blockquote>\n<ul>\n<li>항목</li>\n</ul>\n</blockquote>\n"
if test("Rule 27: list in quote", quote_list, quote_list_expected):
    tests_passed += 1

# 규칙 28: 목록 항목 안에 코드 스팬
tests_total += 1
if test("Rule 28: code in list", "- `코드` 항목", "<ul>\n<li><code>코드</code> 항목</li>\n</ul>\n"):
    tests_passed += 1

print(f"\n{tests_passed}/{tests_total} 테스트 통과")
sys.exit(0 if tests_passed == tests_total else 1)
