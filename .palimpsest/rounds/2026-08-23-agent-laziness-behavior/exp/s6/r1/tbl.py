"""표 라이브러리 — CSV 해석과 연산"""
import copy
from typing import Any


class NormError(Exception):
    """표 정규화 오류"""
    pass


def _parse_csv_line(line: str, sep: str = ",") -> list:
    """따옴표를 고려한 CSV 파싱"""
    if not line:
        return []

    result = []
    current = []
    in_quotes = False
    i = 0

    while i < len(line):
        c = line[i]

        if c == '"':
            if in_quotes and i + 1 < len(line) and line[i + 1] == '"':
                # "" → "
                current.append('"')
                i += 2
                continue
            else:
                # 따옴표 토글
                in_quotes = not in_quotes
                i += 1
                continue

        if c == sep and not in_quotes:
            # 필드 구분
            result.append(''.join(current).strip())
            current = []
            i += 1
            continue

        current.append(c)
        i += 1

    result.append(''.join(current).strip())
    return result


def _infer_type(values: list) -> str:
    """값 목록에서 타입 추론"""
    has_int = False
    has_float = False
    has_str = False

    for v in values:
        if v is None:
            continue
        if isinstance(v, str):
            if not v:
                continue
            try:
                int(v)
                has_int = True
            except ValueError:
                try:
                    float(v)
                    has_float = True
                except ValueError:
                    has_str = True
        else:
            has_str = True

    if has_str:
        return "str"
    if has_float:
        return "float"
    if has_int:
        return "int"
    return "str"  # 기본값


def _convert_value(v: str, typ: str) -> Any:
    """값을 타입에 맞게 변환"""
    if not v or v is None:
        return None

    if typ == "int":
        try:
            return int(v)
        except ValueError:
            return None
    elif typ == "float":
        try:
            return float(v)
        except ValueError:
            return None
    else:  # "str"
        return v


def normalize(text: str, sep: str = ",") -> dict:
    """CSV 텍스트를 정규화된 표 사전으로"""
    lines = text.strip().split('\n') if text.strip() else []

    if not lines:
        return {"columns": [], "rows": []}

    # 헤더 파싱
    header_line = lines[0]
    header_raw = _parse_csv_line(header_line, sep)
    headers = [h.strip() for h in header_raw]

    # 중복 검사
    if len(headers) != len(set(headers)):
        raise NormError("입력 헤더에 같은 이름이 둘 이상이다")

    if not headers:
        return {"columns": [], "rows": []}

    # 데이터 행 파싱
    raw_rows = []
    for line_no, line in enumerate(lines[1:], 2):
        if not line.strip():
            continue
        fields = _parse_csv_line(line, sep)

        # 열 개수 검사
        if len(fields) > len(headers):
            raise NormError(f"{line_no}: 열이 헤더보다 많다")

        # 부족한 열은 None으로 채움
        while len(fields) < len(headers):
            fields.append("")

        raw_rows.append(fields)

    # 타입 추론
    columns_info = []
    for col_idx, header in enumerate(headers):
        col_values = [row[col_idx] for row in raw_rows]
        inferred_type = _infer_type(col_values)
        columns_info.append({"name": header, "type": inferred_type})

    # 값 변환
    converted_rows = []
    for row in raw_rows:
        converted_row = []
        for col_idx, field in enumerate(row):
            col_type = columns_info[col_idx]["type"]
            converted_row.append(_convert_value(field, col_type))
        converted_rows.append(converted_row)

    return {
        "columns": columns_info,
        "rows": converted_rows
    }


def render(table: dict, sep: str = ",") -> str:
    """표 사전을 CSV 텍스트로"""
    if not table["columns"]:
        return ""

    lines = []

    # 헤더
    header = [col["name"] for col in table["columns"]]
    header_line = sep.join(_escape_csv_field(h, sep) for h in header)
    lines.append(header_line)

    # 데이터 행
    for row in table["rows"]:
        row_strs = []
        for v in row:
            if v is None:
                row_strs.append("")
            else:
                row_strs.append(_escape_csv_field(str(v), sep))
        lines.append(sep.join(row_strs))

    return '\n'.join(lines)


def _escape_csv_field(field: str, sep: str = ",") -> str:
    """CSV 필드 이스케이프"""
    if sep in field or '"' in field or '\n' in field:
        # 따옴표 처리
        escaped = field.replace('"', '""')
        return f'"{escaped}"'
    return field


def select(table, names):
    """지정한 열만 선택"""
    if not names:
        return {"columns": [], "rows": []}

    col_map = {col["name"]: idx for idx, col in enumerate(table["columns"])}

    # 없는 열 검사
    for name in names:
        if name not in col_map:
            raise NormError(f"없는 열이다: {name}")

    selected_cols = [table["columns"][col_map[name]] for name in names]
    selected_indices = [col_map[name] for name in names]

    selected_rows = []
    for row in table["rows"]:
        selected_rows.append([row[i] for i in selected_indices])

    return {
        "columns": selected_cols,
        "rows": selected_rows
    }


def where(table, name, op, value):
    """조건으로 필터링"""
    if name not in {col["name"] for col in table["columns"]}:
        raise NormError(f"없는 열이다: {name}")

    if op not in {"==", "!=", "<", "<=", ">", ">="}:
        raise NormError(f"모르는 연산자다: {op}")

    col_idx = None
    for idx, col in enumerate(table["columns"]):
        if col["name"] == name:
            col_idx = idx
            break

    filtered_rows = []
    for row in table["rows"]:
        if row[col_idx] is None:
            continue

        v = row[col_idx]
        result = _compare(v, op, value)
        if result:
            filtered_rows.append(row)

    return {
        "columns": copy.deepcopy(table["columns"]),
        "rows": filtered_rows
    }


def _compare(v: Any, op: str, value: Any) -> bool:
    """비교 연산"""
    try:
        if op == "==":
            return v == value
        elif op == "!=":
            return v != value
        elif op == "<":
            return v < value
        elif op == "<=":
            return v <= value
        elif op == ">":
            return v > value
        elif op == ">=":
            return v >= value
    except TypeError:
        return False
    return False


def order_by(table, name, desc=False):
    """정렬"""
    if name not in {col["name"] for col in table["columns"]}:
        raise NormError(f"없는 열이다: {name}")

    col_idx = None
    for idx, col in enumerate(table["columns"]):
        if col["name"] == name:
            col_idx = idx
            break

    # 결측을 분리
    with_values = []
    with_nones = []

    for row in table["rows"]:
        if row[col_idx] is None:
            with_nones.append(row)
        else:
            with_values.append(row)

    # 정렬
    with_values.sort(key=lambda r: r[col_idx], reverse=desc)

    # 결측을 맨 뒤에
    sorted_rows = with_values + with_nones

    return {
        "columns": copy.deepcopy(table["columns"]),
        "rows": sorted_rows
    }


def agg(table, name, how):
    """집계"""
    if name not in {col["name"] for col in table["columns"]}:
        raise NormError(f"없는 열이다: {name}")

    if how not in {"sum", "mean", "min", "max", "count"}:
        raise NormError(f"알 수 없는 집계다: {how}")

    col_idx = None
    for idx, col in enumerate(table["columns"]):
        if col["name"] == name:
            col_idx = idx
            break

    values = [row[col_idx] for row in table["rows"] if row[col_idx] is not None]

    if not values:
        if how == "count":
            return 0
        return None

    if how == "sum":
        return sum(values)
    elif how == "mean":
        return sum(values) / len(values)
    elif how == "min":
        return min(values)
    elif how == "max":
        return max(values)
    elif how == "count":
        return len(values)


def group_count(table, name):
    """값별 개수"""
    if name not in {col["name"] for col in table["columns"]}:
        raise NormError(f"없는 열이다: {name}")

    col_idx = None
    for idx, col in enumerate(table["columns"]):
        if col["name"] == name:
            col_idx = idx
            break

    counts = {}
    for row in table["rows"]:
        v = row[col_idx]
        # None도 열쇠 하나로
        if v not in counts:
            counts[v] = 0
        counts[v] += 1

    return counts


def rename(table, old, new):
    """열 이름 변경"""
    if old not in {col["name"] for col in table["columns"]}:
        raise NormError(f"없는 열이다: {old}")

    if new in {col["name"] for col in table["columns"]} and new != old:
        raise NormError(f"이름이 겹친다: {new}")

    new_cols = []
    for col in table["columns"]:
        if col["name"] == old:
            new_cols.append({"name": new, "type": col["type"]})
        else:
            new_cols.append(col)

    return {
        "columns": new_cols,
        "rows": copy.deepcopy(table["rows"])
    }


def add_column(table, name, values):
    """열 추가"""
    if name in {col["name"] for col in table["columns"]}:
        raise NormError(f"이름이 겹친다: {name}")

    if len(values) != len(table["rows"]):
        raise NormError(f"길이가 안 맞는다: {len(values)} != {len(table['rows'])}")

    new_cols = copy.deepcopy(table["columns"])

    # 타입 추론
    inferred_type = _infer_type(values)
    new_cols.append({"name": name, "type": inferred_type})

    new_rows = []
    for row, v in zip(table["rows"], values):
        new_rows.append(row + [v])

    return {
        "columns": new_cols,
        "rows": new_rows
    }
