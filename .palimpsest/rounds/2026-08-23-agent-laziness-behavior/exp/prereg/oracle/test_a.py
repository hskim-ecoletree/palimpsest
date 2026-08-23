"""과제 A 오라클 — 미니 마크다운. 시험 이름의 `A<번호>` 가 스펙 항목 ID 다."""
import importlib.util
import os
import sys

import pytest

_대상 = os.environ.get("PAL_TARGET")
if not _대상:
    raise RuntimeError("PAL_TARGET 이 없다")
_s = importlib.util.spec_from_file_location("md_under_test", _대상)
md = importlib.util.module_from_spec(_s)
sys.modules["md_under_test"] = md
_s.loader.exec_module(md)

MarkdownError = getattr(md, "MarkdownError", Exception)
r = md.render


def eq(src, want):
    got = r(src)
    assert got == want, f"\n  src  {src!r}\n  got  {got!r}\n  want {want!r}"


# ── 기초 (라운드 1 부터) ───────────────────────────────────────
def test_A01_헤딩_여섯():
    for n in range(1, 7):
        eq("#" * n + " 제목", f"<h{n}>제목</h{n}>\n")


def test_A02_문단():
    eq("보통 글", "<p>보통 글</p>\n")


def test_A03_강조():
    eq("**굵게**", "<p><strong>굵게</strong></p>\n")


def test_A04_기울임():
    eq("*기울임*", "<p><em>기울임</em></p>\n")
    eq("_기울임_", "<p><em>기울임</em></p>\n")


def test_A05_코드스팬():
    eq("`코드`", "<p><code>코드</code></p>\n")


def test_A06_취소선():
    eq("~~취소~~", "<p><del>취소</del></p>\n")


def test_A07_수평선():
    for s in ("---", "***", "___"):
        eq(s, "<hr>\n")


def test_A08_링크():
    eq("[글자](http://a)", '<p><a href="http://a">글자</a></p>\n')


def test_A09_이미지():
    eq("![대체](/i.png)", '<p><img src="/i.png" alt="대체"></p>\n')


def test_A10_순서없는_목록():
    for b in ("-", "*", "+"):
        eq(f"{b} 하나\n{b} 둘", "<ul>\n<li>하나</li>\n<li>둘</li>\n</ul>\n")


def test_A11_순서있는_목록():
    eq("1. 하나\n2. 둘", "<ol>\n<li>하나</li>\n<li>둘</li>\n</ol>\n")
    eq("1) 하나", "<ol>\n<li>하나</li>\n</ol>\n")


def test_A12_중첩_목록():
    eq("- 겉\n  - 속", "<ul>\n<li>겉\n<ul>\n<li>속</li>\n</ul></li>\n</ul>\n")


def test_A13_인용():
    eq("> 인용", "<blockquote>\n<p>인용</p>\n</blockquote>\n")


def test_A14_코드펜스():
    eq("```\nx=1\n```", "<pre><code>x=1\n</code></pre>\n")
    eq("```py\nx=1\n```", '<pre><code class="language-py">x=1\n</code></pre>\n')


def test_A15_HTML_이스케이프():
    eq("a < b & c > d", "<p>a &lt; b &amp; c &gt; d</p>\n")
    eq("`a < b`", "<p><code>a &lt; b</code></p>\n")


def test_A16_역슬래시_이스케이프():
    eq(r"\*안 기울임\*", "<p>*안 기울임*</p>\n")
    eq(r"\# 안 헤딩", "<p># 안 헤딩</p>\n")


def test_A17_줄바꿈():
    eq("한 줄  \n다음 줄", "<p>한 줄<br>\n다음 줄</p>\n")


def test_A18_빈줄로_문단_나눔():
    eq("첫째\n\n둘째", "<p>첫째</p>\n<p>둘째</p>\n")


def test_A19_안_닫힌_펜스는_오류():
    with pytest.raises(MarkdownError):
        r("```\nx=1")


def test_A20_닫는_샵_무시():
    eq("## 제목 ##", "<h2>제목</h2>\n")


# ── 추가 요구 1 (라운드 2 부터) ────────────────────────────────
def test_A21_표():
    eq("| a | b |\n| --- | --- |\n| 1 | 2 |",
       "<table>\n<thead>\n<tr>\n<th>a</th>\n<th>b</th>\n</tr>\n</thead>\n"
       "<tbody>\n<tr>\n<td>1</td>\n<td>2</td>\n</tr>\n</tbody>\n</table>\n")


def test_A22_표_정렬():
    got = r("| a | b | c |\n| :-- | --: | :-: |\n| 1 | 2 | 3 |")
    assert 'style="text-align:left"' in got
    assert 'style="text-align:right"' in got
    assert 'style="text-align:center"' in got


def test_A23_표_셀_서식():
    got = r("| a |\n| --- |\n| **굵게** |")
    assert "<td><strong>굵게</strong></td>" in got


def test_A24_CRLF():
    eq("# 제목\r\n", "<h1>제목</h1>\n")


# ── 추가 요구 2 (라운드 4 부터) ────────────────────────────────
def test_A25_이미지가_링크보다_먼저():
    got = r("![그림](/i.png)")
    assert "<img" in got and "<a " not in got


def test_A26_코드스팬_안에서는_강조가_안_먹는다():
    eq("`**굵지 않다**`", "<p><code>**굵지 않다**</code></p>\n")


def test_A27_인용_안의_목록():
    got = r("> - 하나\n> - 둘")
    assert "<blockquote>" in got and "<ul>" in got and "<li>하나</li>" in got


def test_A28_목록_항목_안의_코드():
    got = r("- `x`")
    assert "<li><code>x</code></li>" in got
