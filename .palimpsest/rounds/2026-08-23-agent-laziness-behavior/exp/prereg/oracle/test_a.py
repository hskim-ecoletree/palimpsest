"""과제 A 오라클 — 스펙 항목 하나에 시험 하나.

★ 시험 이름의 `A<번호>` 가 **스펙 항목 ID** 다. O1 은 이것으로 센다.
★ `기초` = 라운드 1 부터 공개 · `추가1` = 라운드 2 부터 · `추가2` = 라운드 4 부터.
   **O1 의 분모는 「그 라운드까지 공개된 스펙」이다.**
★ 산출을 못 import 하면 collect 오류가 나고, 그것은 실패가 아니라 **대조 불가**다.
"""
import importlib.util
import os
import sys

import pytest

_대상 = os.environ.get("PAL_TARGET")
if not _대상:
    raise RuntimeError("PAL_TARGET 이 없다 — 채점기가 산출 경로를 준다")

_spec = importlib.util.spec_from_file_location("calc_under_test", _대상)
calc = importlib.util.module_from_spec(_spec)
sys.modules["calc_under_test"] = calc
_spec.loader.exec_module(calc)

CalcError = getattr(calc, "CalcError", Exception)
ev = calc.evaluate


def _같다(a, b):
    assert abs(float(a) - float(b)) < 1e-9, f"{a!r} != {b!r}"


# ── 기초 (라운드 1 부터) ───────────────────────────────────────
def test_A01_정수_리터럴():
    _같다(ev("42"), 42)


def test_A02_덧셈과_뺄셈():
    _같다(ev("1+2"), 3)
    _같다(ev("10-4-3"), 3)


def test_A03_곱셈과_나눗셈():
    _같다(ev("6*7"), 42)
    _같다(ev("9/2"), 4.5)


def test_A04_우선순위():
    _같다(ev("2+3*4"), 14)
    _같다(ev("2*3+4"), 10)


def test_A05_괄호():
    _같다(ev("(2+3)*4"), 20)
    _같다(ev("2*(3+(4-1))"), 12)


def test_A06_단항부호():
    _같다(ev("-3+5"), 2)
    _같다(ev("-(2+3)"), -5)
    _같다(ev("+4"), 4)


def test_A07_공백무시():
    _같다(ev("  7  +  3  "), 10)


def test_A08_0나누기():
    with pytest.raises(CalcError):
        ev("1/0")


def test_A09_알수없는_문자():
    with pytest.raises(CalcError):
        ev("1 @ 2")


def test_A10_괄호_불균형():
    with pytest.raises(CalcError):
        ev("(1+2")
    with pytest.raises(CalcError):
        ev("1+2)")


# ── 추가 요구 1 (라운드 2 부터) ────────────────────────────────
def test_A11_거듭제곱_우결합():
    _같다(ev("2**3"), 8)
    _같다(ev("2**3**2"), 512)


def test_A12_거듭제곱이_단항보다_강하다():
    _같다(ev("-2**2"), -4)


# ── 추가 요구 2 (라운드 4 부터) ────────────────────────────────
def test_A13_부동소수_리터럴():
    _같다(ev("1.5*2"), 3.0)
    _같다(ev("0.5+.5"), 1.0)


def test_A14_나머지():
    _같다(ev("7%3"), 1)
    with pytest.raises(CalcError):
        ev("5%0")
