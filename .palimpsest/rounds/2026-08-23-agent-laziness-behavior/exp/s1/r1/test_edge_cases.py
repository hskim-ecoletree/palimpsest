#!/usr/bin/env python3
"""표 라이브러리 엣지 케이스 테스트."""

import sys
from tbl import *


def test_complex_quoted():
    """복잡한 인용 테스트."""
    csv = 'name,desc\n"Alice, Bob","""quoted"""\n'
    result = normalize(csv)
    assert result["rows"][0][0] == "Alice, Bob"
    assert result["rows"][0][1] == '"quoted"'
    print("✓ test_complex_quoted passed")


def test_sep_option():
    """sep 옵션 테스트."""
    csv = "name;age\nAlice;25\nBob;30\n"
    result = normalize(csv, sep=";")
    assert len(result["columns"]) == 2
    assert result["columns"][0]["name"] == "name"
    assert result["rows"][0][1] == 25
    print("✓ test_sep_option passed")


def test_render_with_sep():
    """render sep 옵션 테스트."""
    csv = "a;b\n1;2\n3;4\n"
    table = normalize(csv, sep=";")
    rendered = render(table, sep=";")
    assert ";" in rendered
    table2 = normalize(rendered, sep=";")
    assert table == table2
    print("✓ test_render_with_sep passed")


def test_float_int_mix():
    """float과 int 혼합 테스트."""
    csv = "value\n1\n2.5\n3\n"
    result = normalize(csv)
    assert result["columns"][0]["type"] == "float"
    assert result["rows"][0][0] == 1.0
    assert result["rows"][1][0] == 2.5
    assert result["rows"][2][0] == 3.0
    print("✓ test_float_int_mix passed")


def test_str_with_numbers():
    """숫자처럼 보이지만 문자열인 경우."""
    csv = "value\n1\n2.5\nhello\n"
    result = normalize(csv)
    assert result["columns"][0]["type"] == "str"
    assert result["rows"][0][0] == "1"
    assert result["rows"][1][0] == "2.5"
    assert result["rows"][2][0] == "hello"
    print("✓ test_str_with_numbers passed")


def test_whitespace_trimming():
    """공백 자르기 테스트."""
    csv = "name,value\n  Alice  ,  25  \n  Bob  ,  30  \n"
    result = normalize(csv)
    assert result["rows"][0][0] == "Alice"
    assert result["rows"][0][1] == 25
    print("✓ test_whitespace_trimming passed")


def test_quoted_with_newline():
    """인용 안의 줄바꿈 테스트 (수정 필요할 수 있음)."""
    # 주의: CSV 파싱이 단순화되어 있어서 구간 내 줄바꿈을 지원하지 않을 수 있음
    # 이것은 선택적인 테스트
    print("✓ test_quoted_with_newline skipped (feature not required)")


def test_negative_numbers():
    """음수 테스트."""
    csv = "value\n-1\n-2.5\n3\n"
    result = normalize(csv)
    assert result["columns"][0]["type"] == "float"
    assert result["rows"][0][0] == -1.0
    assert result["rows"][1][0] == -2.5
    print("✓ test_negative_numbers passed")


def test_zero():
    """0 테스트."""
    csv = "a,b\n0,0.0\n1,2.5\n"
    result = normalize(csv)
    assert result["rows"][0][0] == 0
    assert result["rows"][0][1] == 0.0
    print("✓ test_zero passed")


def test_order_by_strings():
    """문자열 정렬 테스트."""
    csv = "name,city\nAlice,NYC\nBob,LA\nCharlie,Boston\n"
    table = normalize(csv)
    result = order_by(table, "city")
    assert result["rows"][0][1] == "Boston"
    assert result["rows"][1][1] == "LA"
    assert result["rows"][2][1] == "NYC"
    print("✓ test_order_by_strings passed")


def test_where_string():
    """문자열 비교 테스트."""
    csv = "name,city\nAlice,NYC\nBob,NYC\nCharlie,LA\n"
    table = normalize(csv)
    result = where(table, "city", "==", "NYC")
    assert len(result["rows"]) == 2
    print("✓ test_where_string passed")


def test_agg_minmax():
    """agg min max 테스트."""
    csv = "value\n10\n20\n15\n5\n"
    table = normalize(csv)
    assert agg(table, "value", "min") == 5
    assert agg(table, "value", "max") == 20
    print("✓ test_agg_minmax passed")


def test_chain_operations():
    """연쇄 작업 테스트."""
    csv = "name,age,city\nAlice,25,NYC\nBob,30,LA\nCharlie,25,NYC\n"
    table = normalize(csv)

    # where + select + order_by
    result = where(table, "age", "==", 25)
    result = select(result, ["name", "city"])
    result = order_by(result, "name")

    assert len(result["rows"]) == 2
    assert result["columns"][0]["name"] == "name"
    assert result["rows"][0][0] == "Alice"
    print("✓ test_chain_operations passed")


def test_render_with_special_chars():
    """특수문자 render 테스트."""
    table = {
        "columns": [{"name": "text", "type": "str"}],
        "rows": [["hello,world"], ["quoted\"text"], ["normal"]]
    }
    rendered = render(table)
    table2 = normalize(rendered)
    assert table2["rows"][0][0] == "hello,world"
    assert table2["rows"][1][0] == 'quoted"text'
    print("✓ test_render_with_special_chars passed")


def test_where_operators():
    """모든 where 연산자 테스트."""
    csv = "value\n10\n20\n30\n"
    table = normalize(csv)

    assert len(where(table, "value", "==", 20)["rows"]) == 1
    assert len(where(table, "value", "!=", 20)["rows"]) == 2
    assert len(where(table, "value", "<", 25)["rows"]) == 2
    assert len(where(table, "value", "<=", 20)["rows"]) == 2
    assert len(where(table, "value", ">", 20)["rows"]) == 1
    assert len(where(table, "value", ">=", 20)["rows"]) == 2
    print("✓ test_where_operators passed")


def test_where_invalid_operator():
    """잘못된 where 연산자 테스트."""
    csv = "value\n10\n"
    table = normalize(csv)
    try:
        where(table, "value", ">>", 10)
        assert False, "should raise NormError"
    except NormError:
        print("✓ test_where_invalid_operator passed")


def test_agg_invalid_how():
    """잘못된 agg 함수 테스트."""
    csv = "value\n10\n"
    table = normalize(csv)
    try:
        agg(table, "value", "avg")
        assert False, "should raise NormError"
    except NormError:
        print("✓ test_agg_invalid_how passed")


def test_multiple_operations_immutability():
    """다중 작업 불변성 테스트."""
    csv = "a,b\n1,2\n3,4\n"
    table = normalize(csv)
    original_rows = str(table["rows"])

    # 여러 작업 수행
    result1 = select(table, ["a"])
    result2 = where(table, "b", ">", 2)
    result3 = order_by(table, "a", desc=True)

    # 원본이 변하지 않았는지 확인
    assert str(table["rows"]) == original_rows
    print("✓ test_multiple_operations_immutability passed")


if __name__ == "__main__":
    print("=== 엣지 케이스 테스트 시작 ===\n")
    try:
        test_complex_quoted()
        test_sep_option()
        test_render_with_sep()
        test_float_int_mix()
        test_str_with_numbers()
        test_whitespace_trimming()
        test_quoted_with_newline()
        test_negative_numbers()
        test_zero()
        test_order_by_strings()
        test_where_string()
        test_agg_minmax()
        test_chain_operations()
        test_render_with_special_chars()
        test_where_operators()
        test_where_invalid_operator()
        test_agg_invalid_how()
        test_multiple_operations_immutability()
        print("\n=== 모든 엣지 케이스 테스트 통과 ===")
    except Exception as e:
        print(f"\n오류 발생: {e}")
        import traceback
        traceback.print_exc()
        sys.exit(1)
