#!/usr/bin/env python3
"""
Test suite for tbl.py table library.
"""

import sys
sys.path.insert(0, '/tmp/pal-x-68/s5')

from tbl import (
    NormError, normalize, render, select, where, order_by, agg,
    group_count, rename, add_column, to_json, join
)


def test_basic_normalize():
    """Test rule 1-6: Basic normalization"""
    csv = "a,b,c\n1,2,3\n4,5,6"
    result = normalize(csv)
    assert len(result["columns"]) == 3
    assert result["columns"][0]["name"] == "a"
    assert result["columns"][0]["type"] == "int"
    assert result["rows"][0] == [1, 2, 3]
    print("✓ Basic normalize")


def test_type_inference():
    """Test rules 2-4: Type inference"""
    # All ints
    csv = "nums\n1\n2\n3"
    result = normalize(csv)
    assert result["columns"][0]["type"] == "int"
    assert all(isinstance(v, int) for v in [r[0] for r in result["rows"] if r[0] is not None])

    # Mix of int and float
    csv = "nums\n1\n2.5\n3"
    result = normalize(csv)
    assert result["columns"][0]["type"] == "float"
    assert all(isinstance(v, float) for v in [r[0] for r in result["rows"] if r[0] is not None])

    # Non-numeric
    csv = "data\n1\nabc\n3"
    result = normalize(csv)
    assert result["columns"][0]["type"] == "str"
    assert all(isinstance(v, str) for v in [r[0] for r in result["rows"] if r[0] is not None])
    print("✓ Type inference")


def test_whitespace_trimming():
    """Test rule 5: Whitespace trimming"""
    csv = "  name  ,  value  \n  hello  ,  123  "
    result = normalize(csv)
    assert result["columns"][0]["name"] == "name"
    assert result["columns"][1]["name"] == "value"
    assert result["rows"][0][0] == "hello"
    assert result["rows"][0][1] == 123
    print("✓ Whitespace trimming")


def test_missing_values():
    """Test rule 6: Missing values"""
    csv = "a,b,c\n1,,3\n,,\n4,5,6"
    result = normalize(csv)
    assert result["rows"][0][1] is None
    assert result["rows"][1][0] is None
    print("✓ Missing values")


def test_padding():
    """Test rule 7: Padding short rows"""
    csv = "a,b,c\n1,2\n3"
    result = normalize(csv)
    assert len(result["rows"][0]) == 3
    assert result["rows"][0][2] is None
    assert result["rows"][1][1] is None
    print("✓ Padding short rows")


def test_too_many_columns():
    """Test rule 8: Error on too many columns"""
    csv = "a,b\n1,2,3"
    try:
        normalize(csv)
        assert False, "Should raise NormError"
    except NormError:
        pass
    print("✓ Too many columns error")


def test_quoted_values():
    """Test rule 9: Quoted comma handling"""
    csv = 'a,b\n"hello, world",123\ntest,"a""b"'
    result = normalize(csv)
    assert result["rows"][0][0] == "hello, world"
    assert result["rows"][1][1] == 'a"b'
    print("✓ Quoted values")


def test_round_trip():
    """Test rule 10: normalize(render(t)) == t"""
    csv = "a,b,c\n1,2,3\n4,5,6"
    t1 = normalize(csv)
    rendered = render(t1)
    t2 = normalize(rendered)
    assert t1 == t2
    print("✓ Round-trip normalize/render")


def test_select():
    """Test rule 11: Select columns"""
    csv = "a,b,c\n1,2,3\n4,5,6"
    t = normalize(csv)
    result = select(t, ["c", "a"])
    assert len(result["columns"]) == 2
    assert result["columns"][0]["name"] == "c"
    assert result["columns"][1]["name"] == "a"
    assert result["rows"][0] == [3, 1]

    # Error on missing column
    try:
        select(t, ["d"])
        assert False, "Should raise NormError"
    except NormError:
        pass
    print("✓ Select columns")


def test_where():
    """Test rule 12: Where filtering"""
    csv = "a,b\n1,10\n2,20\n3,30"
    t = normalize(csv)
    result = where(t, "a", ">=", 2)
    assert len(result["rows"]) == 2
    assert result["rows"][0][0] == 2

    # With missing values
    csv = "a,b\n1,10\n,20\n3,30"
    t = normalize(csv)
    result = where(t, "a", ">", 1)
    assert len(result["rows"]) == 1
    assert result["rows"][0][0] == 3

    # Unknown operator
    try:
        where(t, "a", "??", 1)
        assert False, "Should raise NormError"
    except NormError:
        pass
    print("✓ Where filtering")


def test_order_by():
    """Test rule 13: Order by with missing values at end"""
    csv = "a,b\n3,x\n1,y\n,z\n2,w"
    t = normalize(csv)
    result = order_by(t, "a")
    assert result["rows"][0][0] == 1
    assert result["rows"][1][0] == 2
    assert result["rows"][2][0] == 3
    assert result["rows"][3][0] is None

    # Descending
    result = order_by(t, "a", desc=True)
    assert result["rows"][0][0] == 3
    assert result["rows"][-1][0] is None
    print("✓ Order by")


def test_agg():
    """Test rule 14: Aggregation functions"""
    csv = "nums\n1\n2\n3\n"
    t = normalize(csv)

    assert agg(t, "nums", "sum") == 6
    assert agg(t, "nums", "mean") == 2.0
    assert agg(t, "nums", "min") == 1
    assert agg(t, "nums", "max") == 3
    assert agg(t, "nums", "count") == 3

    # Empty column
    csv = "nums\n\n\n"
    t = normalize(csv)
    assert agg(t, "nums", "count") == 0
    assert agg(t, "nums", "sum") is None
    print("✓ Aggregation")


def test_group_count():
    """Test rule 15: Group count with missing"""
    csv = "a\n1\n1\n2\n\n1"
    t = normalize(csv)
    result = group_count(t, "a")
    assert result[1] == 3
    assert result[2] == 1
    assert result[None] == 1
    print("✓ Group count")


def test_rename():
    """Test rule 16: Rename column"""
    csv = "a,b\n1,2\n3,4"
    t = normalize(csv)
    result = rename(t, "a", "x")
    assert result["columns"][0]["name"] == "x"

    # Duplicate name error
    try:
        rename(t, "a", "b")
        assert False, "Should raise NormError"
    except NormError:
        pass
    print("✓ Rename column")


def test_add_column():
    """Test rule 17: Add column"""
    csv = "a,b\n1,2\n3,4"
    t = normalize(csv)
    result = add_column(t, "c", [10, 20])
    assert len(result["columns"]) == 3
    assert result["columns"][2]["name"] == "c"
    assert result["rows"][0][2] == 10

    # Duplicate name error
    try:
        add_column(t, "a", [10, 20])
        assert False, "Should raise NormError"
    except NormError:
        pass

    # Length mismatch error
    try:
        add_column(t, "c", [10])
        assert False, "Should raise NormError"
    except NormError:
        pass
    print("✓ Add column")


def test_duplicate_names():
    """Test rule 18: Duplicate column names error"""
    csv = "a,a,b\n1,2,3"
    try:
        normalize(csv)
        assert False, "Should raise NormError"
    except NormError:
        pass
    print("✓ Duplicate names error")


def test_separator():
    """Test rule 19: Custom separator"""
    csv = "a;b;c\n1;2;3\n4;5;6"
    t = normalize(csv, sep=";")
    assert len(t["columns"]) == 3
    rendered = render(t, sep=";")
    t2 = normalize(rendered, sep=";")
    assert t == t2
    print("✓ Custom separator")


def test_empty_input():
    """Test rule 20: Empty input"""
    result = normalize("")
    assert result == {"columns": [], "rows": []}
    result = normalize("   ")
    assert result == {"columns": [], "rows": []}
    print("✓ Empty input")


def test_bool_type():
    """Test rule 21: Boolean type inference"""
    # All booleans (case-insensitive)
    csv = "flag\nTrue\nFalse\ntrue\nfalse"
    result = normalize(csv)
    assert result["columns"][0]["type"] == "bool"
    assert result["rows"][0][0] is True
    assert result["rows"][1][0] is False
    assert result["rows"][2][0] is True
    assert result["rows"][3][0] is False

    # Mixed bool and non-bool becomes string
    csv = "flag\ntrue\nmaybe"
    result = normalize(csv)
    assert result["columns"][0]["type"] == "str"
    print("✓ Boolean type")


def test_all_missing_column():
    """Test rule 22: All-missing column becomes string"""
    csv = "a,b,c\n1,,3\n2,,4\n3,,5"
    result = normalize(csv)
    assert result["columns"][1]["type"] == "str"
    assert all(v is None for v in [r[1] for r in result["rows"]])
    print("✓ All-missing column")


def test_to_json():
    """Test rule 23: to_json conversion"""
    csv = "a,b\n1,hello\n2,world"
    t = normalize(csv)
    json_str = to_json(t)

    import json
    parsed = json.loads(json_str)
    assert len(parsed) == 2
    assert parsed[0] == {"a": 1, "b": "hello"}
    assert parsed[1] == {"a": 2, "b": "world"}

    # With missing values
    csv = "a,b\n1,\n,world"
    t = normalize(csv)
    json_str = to_json(t)
    parsed = json.loads(json_str)
    assert parsed[0]["b"] is None
    assert parsed[1]["a"] is None
    print("✓ to_json conversion")


def test_join():
    """Test rule 24: Inner join"""
    csv_a = "id,name\n1,Alice\n2,Bob\n3,Charlie"
    csv_b = "id,score\n1,90\n2,85\n4,95"

    t_a = normalize(csv_a)
    t_b = normalize(csv_b)

    result = join(t_a, t_b, "id")
    assert len(result["rows"]) == 2
    assert result["rows"][0] == [1, "Alice", 90]
    assert result["rows"][1] == [2, "Bob", 85]

    # Check columns
    assert len(result["columns"]) == 3
    assert [c["name"] for c in result["columns"]] == ["id", "name", "score"]

    # With missing values in join column - should be excluded
    csv_a = "id,name\n1,Alice\n,Bob\n3,Charlie"
    csv_b = "id,score\n1,90\n2,85"
    t_a = normalize(csv_a)
    t_b = normalize(csv_b)
    result = join(t_a, t_b, "id")
    assert len(result["rows"]) == 1
    assert result["rows"][0] == [1, "Alice", 90]

    # Column name overlap error (excluding 'on')
    csv_a = "id,name\n1,Alice"
    csv_b = "id,name\n1,Bob"
    t_a = normalize(csv_a)
    t_b = normalize(csv_b)
    try:
        join(t_a, t_b, "id")
        assert False, "Should raise NormError"
    except NormError as e:
        assert "overlap" in str(e).lower()

    print("✓ Join operation")


if __name__ == "__main__":
    try:
        test_basic_normalize()
        test_type_inference()
        test_whitespace_trimming()
        test_missing_values()
        test_padding()
        test_too_many_columns()
        test_quoted_values()
        test_round_trip()
        test_select()
        test_where()
        test_order_by()
        test_agg()
        test_group_count()
        test_rename()
        test_add_column()
        test_duplicate_names()
        test_separator()
        test_empty_input()
        test_bool_type()
        test_all_missing_column()
        test_to_json()
        test_join()
        print("\n✓ All tests passed!")
    except Exception as e:
        print(f"\n✗ Test failed: {e}", file=sys.stderr)
        import traceback
        traceback.print_exc()
        sys.exit(1)
