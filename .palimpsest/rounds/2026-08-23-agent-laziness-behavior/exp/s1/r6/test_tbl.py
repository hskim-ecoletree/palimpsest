#!/usr/bin/env python3
"""표 라이브러리 테스트."""

import sys
from tbl import *

def test_basic_normalize():
    """기본 normalize 테스트."""
    csv = "name,age,score\nAlice,25,85.5\nBob,30,90\n"
    result = normalize(csv)
    assert len(result["columns"]) == 3
    assert result["columns"][0]["name"] == "name"
    assert result["columns"][0]["type"] == "str"
    assert result["columns"][1]["type"] == "int"
    assert result["columns"][2]["type"] == "float"
    assert len(result["rows"]) == 2
    print("✓ test_basic_normalize passed")


def test_empty_input():
    """빈 입력 테스트."""
    result = normalize("")
    assert result == {"columns": [], "rows": []}
    print("✓ test_empty_input passed")


def test_header_trimming():
    """헤더 공백 제거 테스트."""
    csv = " name , age \nAlice,25\n"
    result = normalize(csv)
    assert result["columns"][0]["name"] == "name"
    assert result["columns"][1]["name"] == "age"
    print("✓ test_header_trimming passed")


def test_missing_values():
    """결측값 처리 테스트."""
    csv = "a,b,c\n1,2,3\n4,,6\n"
    result = normalize(csv)
    assert result["rows"][1][1] is None
    assert result["columns"][1]["type"] == "int"
    print("✓ test_missing_values passed")


def test_short_row():
    """짧은 행 채우기 테스트."""
    csv = "a,b,c\n1,2,3\n4,5\n"
    result = normalize(csv)
    assert len(result["rows"][1]) == 3
    assert result["rows"][1][2] is None
    print("✓ test_short_row passed")


def test_long_row_error():
    """긴 행 오류 테스트."""
    csv = "a,b\n1,2,3\n"
    try:
        normalize(csv)
        assert False, "should raise NormError"
    except NormError as e:
        print(f"✓ test_long_row_error passed (error: {e})")


def test_duplicate_header_error():
    """중복 헤더 오류 테스트."""
    csv = "a,a\n1,2\n"
    try:
        normalize(csv)
        assert False, "should raise NormError"
    except NormError as e:
        print(f"✓ test_duplicate_header_error passed (error: {e})")


def test_quoted_csv():
    """인용 CSV 테스트."""
    csv = 'name,desc\nAlice,"Hello, world"\nBob,"She said ""Hi"""\n'
    result = normalize(csv)
    assert result["rows"][0][1] == "Hello, world"
    assert result["rows"][1][1] == 'She said "Hi"'
    print("✓ test_quoted_csv passed")


def test_render_normalize_roundtrip():
    """render-normalize 왕복 테스트."""
    csv = "a,b\n1,2\n3,4\n"
    table = normalize(csv)
    rendered = render(table)
    table2 = normalize(rendered)
    assert table == table2
    print("✓ test_render_normalize_roundtrip passed")


def test_select():
    """select 테스트."""
    csv = "name,age,score\nAlice,25,85\nBob,30,90\n"
    table = normalize(csv)
    result = select(table, ["name", "score"])
    assert len(result["columns"]) == 2
    assert result["columns"][0]["name"] == "name"
    assert result["columns"][1]["name"] == "score"
    assert result["rows"][0] == ["Alice", 85]
    print("✓ test_select passed")


def test_select_error():
    """select 오류 테스트."""
    csv = "name,age\nAlice,25\n"
    table = normalize(csv)
    try:
        select(table, ["name", "missing"])
        assert False, "should raise NormError"
    except NormError as e:
        print(f"✓ test_select_error passed (error: {e})")


def test_where():
    """where 테스트."""
    csv = "name,age\nAlice,25\nBob,30\nCharlie,25\n"
    table = normalize(csv)
    result = where(table, "age", "==", 25)
    assert len(result["rows"]) == 2
    assert result["rows"][0][0] == "Alice"
    assert result["rows"][1][0] == "Charlie"
    print("✓ test_where passed")


def test_where_null():
    """where null 처리 테스트."""
    csv = "name,age\nAlice,25\nBob,\nCharlie,30\n"
    table = normalize(csv)
    result = where(table, "age", ">", 20)
    assert len(result["rows"]) == 2
    assert result["rows"][0][0] == "Alice"
    assert result["rows"][1][0] == "Charlie"
    print("✓ test_where_null passed")


def test_order_by():
    """order_by 테스트."""
    csv = "name,age\nAlice,25\nBob,30\nCharlie,20\n"
    table = normalize(csv)
    result = order_by(table, "age")
    assert result["rows"][0][1] == 20
    assert result["rows"][1][1] == 25
    assert result["rows"][2][1] == 30
    print("✓ test_order_by passed")


def test_order_by_desc():
    """order_by desc 테스트."""
    csv = "name,age\nAlice,25\nBob,30\nCharlie,20\n"
    table = normalize(csv)
    result = order_by(table, "age", desc=True)
    assert result["rows"][0][1] == 30
    assert result["rows"][1][1] == 25
    assert result["rows"][2][1] == 20
    print("✓ test_order_by_desc passed")


def test_order_by_null():
    """order_by null 처리 테스트."""
    csv = "name,age\nAlice,25\nBob,\nCharlie,30\n"
    table = normalize(csv)
    result = order_by(table, "age")
    assert result["rows"][0][1] == 25
    assert result["rows"][1][1] == 30
    assert result["rows"][2][1] is None
    print("✓ test_order_by_null passed")


def test_agg_sum():
    """agg sum 테스트."""
    csv = "name,score\nAlice,85\nBob,90\nCharlie,95\n"
    table = normalize(csv)
    result = agg(table, "score", "sum")
    assert result == 270
    print("✓ test_agg_sum passed")


def test_agg_mean():
    """agg mean 테스트."""
    csv = "name,score\nAlice,85\nBob,90\nCharlie,95\n"
    table = normalize(csv)
    result = agg(table, "score", "mean")
    assert result == 90.0
    print("✓ test_agg_mean passed")


def test_agg_count():
    """agg count 테스트."""
    csv = "name,score\nAlice,85\nBob,\nCharlie,95\n"
    table = normalize(csv)
    result = agg(table, "score", "count")
    assert result == 2
    print("✓ test_agg_count passed")


def test_agg_count_empty():
    """agg count 공값 테스트."""
    csv = "name,score\nAlice,\nBob,\n"
    table = normalize(csv)
    result = agg(table, "score", "count")
    assert result == 0
    print("✓ test_agg_count_empty passed")


def test_agg_empty_other():
    """agg 공값 다른 집계 테스트."""
    csv = "name,score\nAlice,\nBob,\n"
    table = normalize(csv)
    result = agg(table, "score", "sum")
    assert result is None
    print("✓ test_agg_empty_other passed")


def test_group_count():
    """group_count 테스트."""
    csv = "name,age\nAlice,25\nBob,30\nCharlie,25\n"
    table = normalize(csv)
    result = group_count(table, "age")
    assert result[25] == 2
    assert result[30] == 1
    print("✓ test_group_count passed")


def test_group_count_null():
    """group_count null 테스트."""
    csv = "name,age\nAlice,25\nBob,\nCharlie,25\n"
    table = normalize(csv)
    result = group_count(table, "age")
    assert result[25] == 2
    assert result[None] == 1
    print("✓ test_group_count_null passed")


def test_rename():
    """rename 테스트."""
    csv = "name,age\nAlice,25\n"
    table = normalize(csv)
    result = rename(table, "age", "years")
    assert result["columns"][1]["name"] == "years"
    print("✓ test_rename passed")


def test_rename_conflict():
    """rename 충돌 테스트."""
    csv = "name,age\nAlice,25\n"
    table = normalize(csv)
    try:
        rename(table, "age", "name")
        assert False, "should raise NormError"
    except NormError as e:
        print(f"✓ test_rename_conflict passed (error: {e})")


def test_add_column():
    """add_column 테스트."""
    csv = "name,age\nAlice,25\nBob,30\n"
    table = normalize(csv)
    result = add_column(table, "city", ["NYC", "LA"])
    assert len(result["columns"]) == 3
    assert result["columns"][2]["name"] == "city"
    assert result["rows"][0][2] == "NYC"
    print("✓ test_add_column passed")


def test_add_column_length_error():
    """add_column 길이 오류 테스트."""
    csv = "name,age\nAlice,25\nBob,30\n"
    table = normalize(csv)
    try:
        add_column(table, "city", ["NYC"])
        assert False, "should raise NormError"
    except NormError as e:
        print(f"✓ test_add_column_length_error passed (error: {e})")


def test_add_column_conflict():
    """add_column 충돌 테스트."""
    csv = "name,age\nAlice,25\n"
    table = normalize(csv)
    try:
        add_column(table, "name", ["Charlie"])
        assert False, "should raise NormError"
    except NormError as e:
        print(f"✓ test_add_column_conflict passed (error: {e})")


if __name__ == "__main__":
    print("=== 표 라이브러리 테스트 시작 ===\n")
    try:
        test_basic_normalize()
        test_empty_input()
        test_header_trimming()
        test_missing_values()
        test_short_row()
        test_long_row_error()
        test_duplicate_header_error()
        test_quoted_csv()
        test_render_normalize_roundtrip()
        test_select()
        test_select_error()
        test_where()
        test_where_null()
        test_order_by()
        test_order_by_desc()
        test_order_by_null()
        test_agg_sum()
        test_agg_mean()
        test_agg_count()
        test_agg_count_empty()
        test_agg_empty_other()
        test_group_count()
        test_group_count_null()
        test_rename()
        test_rename_conflict()
        test_add_column()
        test_add_column_length_error()
        test_add_column_conflict()
        print("\n=== 모든 테스트 통과 ===")
    except Exception as e:
        print(f"\n오류 발생: {e}")
        import traceback
        traceback.print_exc()
        sys.exit(1)
