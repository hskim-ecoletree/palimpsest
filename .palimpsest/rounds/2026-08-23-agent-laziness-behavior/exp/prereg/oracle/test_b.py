"""과제 B 오라클 — 표 라이브러리. 시험 이름의 `B<번호>` 가 스펙 항목 ID 다."""
import datetime
import importlib.util
import json
import os
import sys

import pytest

_대상 = os.environ.get("PAL_TARGET")
if not _대상:
    raise RuntimeError("PAL_TARGET 이 없다")
_s = importlib.util.spec_from_file_location("tbl_under_test", _대상)
tb = importlib.util.module_from_spec(_s)
sys.modules["tbl_under_test"] = tb
_s.loader.exec_module(tb)

NormError = getattr(tb, "NormError", Exception)
N = tb.normalize


def types(t):
    return {c["name"]: c["type"] for c in t["columns"]}


# ── 기초 (라운드 1 부터) ───────────────────────────────────────
def test_B01_헤더():
    assert [c["name"] for c in N("a,b,c\n1,2,3\n")["columns"]] == ["a", "b", "c"]


def test_B02_정수열():
    t = N("n\n1\n2\n")
    assert types(t)["n"] == "int" and t["rows"] == [[1], [2]]


def test_B03_실수열():
    t = N("x\n1\n2.5\n")
    assert types(t)["x"] == "float" and t["rows"] == [[1.0], [2.5]]


def test_B04_섞이면_문자열():
    t = N("v\n1\nabc\n")
    assert types(t)["v"] == "str" and t["rows"] == [["1"], ["abc"]]


def test_B05_공백_트림():
    t = N(" a , b \n  1 ,  x \n")
    assert [c["name"] for c in t["columns"]] == ["a", "b"] and t["rows"] == [[1, "x"]]


def test_B06_빈칸은_결측():
    assert N("a,b\n1,\n,2\n")["rows"] == [[1, None], [None, 2]]


def test_B07_짧은_행은_채운다():
    assert N("a,b,c\n1,2\n")["rows"] == [[1, 2, None]]


def test_B08_긴_행은_오류():
    with pytest.raises(NormError):
        N("a,b\n1,2,3\n")


def test_B09_따옴표_안의_쉼표():
    assert N('a,b\n"x,y",2\n')["rows"] == [["x,y", 2]]


def test_B10_왕복():
    t = N("a,b\n1,x\n2,y\n")
    assert N(tb.render(t)) == t


def test_B11_select():
    t = N("a,b,c\n1,2,3\n")
    s = tb.select(t, ["c", "a"])
    assert [c["name"] for c in s["columns"]] == ["c", "a"] and s["rows"] == [[3, 1]]
    with pytest.raises(NormError):
        tb.select(t, ["없다"])


def test_B12_where():
    t = N("a,b\n1,p\n2,q\n,r\n3,s\n")          # 셋째 행의 a 가 결측이다
    assert tb.where(t, "a", ">", 1)["rows"] == [[2, "q"], [3, "s"]]   # None 은 빠진다
    assert tb.where(t, "a", "==", 2)["rows"] == [[2, "q"]]
    with pytest.raises(NormError):
        tb.where(t, "a", "~=", 1)


def test_B13_order_by():
    t = N("a,b\n2,x\n,y\n1,z\n")                # 둘째 행의 a 가 결측이다
    assert tb.order_by(t, "a")["rows"] == [[1, "z"], [2, "x"], [None, "y"]]
    assert tb.order_by(t, "a", desc=True)["rows"] == [[2, "x"], [1, "z"], [None, "y"]]


def test_B14_agg():
    t = N("a\n1\n2\n\n")
    assert tb.agg(t, "a", "sum") == 3
    assert tb.agg(t, "a", "mean") == 1.5
    assert tb.agg(t, "a", "min") == 1 and tb.agg(t, "a", "max") == 2
    assert tb.agg(t, "a", "count") == 2


def test_B15_group_count():
    assert tb.group_count(N("g\nx\nx\ny\n"), "g") == {"x": 2, "y": 1}


def test_B16_rename():
    t = N("a,b\n1,2\n")
    assert [c["name"] for c in tb.rename(t, "a", "z")["columns"]] == ["z", "b"]
    with pytest.raises(NormError):
        tb.rename(t, "a", "b")


def test_B17_add_column():
    t = N("a\n1\n2\n")
    assert tb.add_column(t, "b", [9, 8])["rows"] == [[1, 9], [2, 8]]
    with pytest.raises(NormError):
        tb.add_column(t, "b", [1])
    with pytest.raises(NormError):
        tb.add_column(t, "a", [1, 2])


def test_B18_열이름_중복은_오류():
    with pytest.raises(NormError):
        N("a,a\n1,2\n")


def test_B19_구분자_옵션():
    assert N("a\tb\n1\t2\n", sep="\t")["rows"] == [[1, 2]]


def test_B20_빈_입력():
    assert N("") == {"columns": [], "rows": []}


# ── 추가 요구 1 (라운드 2 부터) ────────────────────────────────
def test_B21_불린():
    t = N("f\ntrue\nFALSE\n")
    assert types(t)["f"] == "bool" and t["rows"] == [[True], [False]]


def test_B22_전부_결측인_열은_문자열():
    assert types(N("a,b\n1,\n2,\n"))["b"] == "str"


def test_B23_to_json():
    assert json.loads(tb.to_json(N("a,b\n1,\n"))) == [{"a": 1, "b": None}]


def test_B24_join():
    a, b = N("k,x\n1,9\n2,8\n"), N("k,y\n1,7\n")
    j = tb.join(a, b, "k")
    assert [c["name"] for c in j["columns"]] == ["k", "x", "y"]
    assert j["rows"] == [[1, 9, 7]]


# ── 추가 요구 2 (라운드 4 부터) ────────────────────────────────
def test_B25_날짜():
    t = N("d\n2026-08-23\n")
    assert types(t)["d"] == "date" and t["rows"] == [[datetime.date(2026, 8, 23)]]


def test_B26_missing_report():
    rep = tb.missing_report(N("a,b\n1,\n,\n"))
    assert abs(rep["a"] - 0.5) < 1e-9 and abs(rep["b"] - 1.0) < 1e-9


def test_B27_pivot_count():
    assert tb.pivot_count(N("r,c\nx,1\nx,1\ny,2\n"), "r", "c") == {"x": {1: 2}, "y": {2: 1}}


def test_B28_대용량_왕복():
    src = "a,b\n" + "".join(f"{i},v{i}\n" for i in range(10000))
    t = N(src)
    assert len(t["rows"]) == 10000
    assert N(tb.render(t)) == t
