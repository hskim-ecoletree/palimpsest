"""과제 B 오라클 — 스펙 항목 하나에 시험 하나. 이름의 `B<번호>` 가 스펙 항목 ID 다."""
import datetime
import importlib.util
import os
import sys

import pytest

_대상 = os.environ.get("PAL_TARGET")
if not _대상:
    raise RuntimeError("PAL_TARGET 이 없다 — 채점기가 산출 경로를 준다")

_spec = importlib.util.spec_from_file_location("norm_under_test", _대상)
norm = importlib.util.module_from_spec(_spec)
sys.modules["norm_under_test"] = norm
_spec.loader.exec_module(norm)

NormError = getattr(norm, "NormError", Exception)


def _t(text):
    return norm.normalize(text)


def _types(tb):
    return {c["name"]: c["type"] for c in tb["columns"]}


# ── 기초 (라운드 1 부터) ───────────────────────────────────────
def test_B01_헤더():
    tb = _t("a,b,c\n1,2,3\n")
    assert [c["name"] for c in tb["columns"]] == ["a", "b", "c"]


def test_B02_정수열():
    assert _types(_t("n\n1\n2\n"))["n"] == "int"
    assert _t("n\n1\n2\n")["rows"] == [[1], [2]]


def test_B03_실수열():
    tb = _t("x\n1\n2.5\n")
    assert _types(tb)["x"] == "float"
    assert tb["rows"] == [[1.0], [2.5]]


def test_B04_섞이면_문자열():
    tb = _t("v\n1\nabc\n")
    assert _types(tb)["v"] == "str"
    assert tb["rows"] == [["1"], ["abc"]]


def test_B05_공백_트림():
    tb = _t(" a , b \n  1 ,  x \n")
    assert [c["name"] for c in tb["columns"]] == ["a", "b"]
    assert tb["rows"] == [[1, "x"]]


def test_B06_빈칸은_결측():
    tb = _t("a,b\n1,\n,2\n")
    assert tb["rows"] == [[1, None], [None, 2]]


def test_B07_짧은_행은_채운다():
    tb = _t("a,b,c\n1,2\n")
    assert tb["rows"] == [[1, 2, None]]


def test_B08_긴_행은_예외():
    with pytest.raises(NormError):
        _t("a,b\n1,2,3\n")


def test_B09_따옴표_안의_쉼표():
    tb = _t('a,b\n"x,y",2\n')
    assert tb["rows"] == [["x,y", 2]]


def test_B10_왕복():
    tb = _t("a,b\n1,x\n2,y\n")
    assert norm.normalize(norm.render(tb)) == tb


# ── 추가 요구 1 (라운드 2 부터) ────────────────────────────────
def test_B11_불린():
    tb = _t("f\ntrue\nFALSE\n")
    assert _types(tb)["f"] == "bool"
    assert tb["rows"] == [[True], [False]]


def test_B12_전부_결측인_열은_문자열():
    tb = _t("a,b\n1,\n2,\n")
    assert _types(tb)["b"] == "str"


# ── 추가 요구 2 (라운드 4 부터) ────────────────────────────────
def test_B13_날짜():
    tb = _t("d\n2026-08-23\n")
    assert _types(tb)["d"] == "date"
    assert tb["rows"] == [[datetime.date(2026, 8, 23)]]


def test_B14_결측_비율():
    tb = _t("a,b\n1,\n,\n")
    rep = norm.missing_report(tb)
    assert abs(rep["a"] - 0.5) < 1e-9
    assert abs(rep["b"] - 1.0) < 1e-9
