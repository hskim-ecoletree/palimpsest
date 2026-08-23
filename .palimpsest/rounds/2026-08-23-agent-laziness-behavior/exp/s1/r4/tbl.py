"""CSV 텍스트를 받아 열 타입을 추론하고, 그 표 위에서 도는 연산을 갖춘 모듈."""

import json
import datetime
from typing import Any, Dict, List, Optional


class NormError(Exception):
    """표 정규화 중 발생하는 오류."""
    pass


def _parse_csv_line(line: str, sep: str = ",") -> List[str]:
    """CSV 라인을 파싱해서 값의 리스트를 반환한다.

    규칙 9: 큰따옴표 안의 쉼표는 값의 일부다. 안의 ""는 따옴표 하나다.
    규칙 5: 값의 앞뒤 공백은 버린다.
    """
    values = []
    current = ""
    in_quotes = False
    i = 0

    while i < len(line):
        c = line[i]

        if c == '"':
            if in_quotes:
                # 따옴표 안에 있음
                if i + 1 < len(line) and line[i + 1] == '"':
                    # "" → "로 치환
                    current += '"'
                    i += 2
                    continue
                else:
                    # 따옴표 닫음
                    in_quotes = False
                    i += 1
            else:
                # 따옴표 열기
                in_quotes = True
                i += 1
        elif c == sep and not in_quotes:
            # 필드 구분자
            values.append(current.strip())
            current = ""
            i += 1
        else:
            current += c
            i += 1

    values.append(current.strip())
    return values


def _is_date(v: str) -> bool:
    """YYYY-MM-DD 형식의 날짜인지 확인한다."""
    try:
        datetime.datetime.strptime(v, "%Y-%m-%d")
        return True
    except ValueError:
        return False


def _infer_type(values: List[Optional[str]]) -> str:
    """값들의 리스트에서 타입을 추론한다.

    규칙 2-4, 21-22, 25:
    - 모든 값이 None/empty → "str" (규칙 22)
    - bool 값만 있으면 → "bool" (규칙 21)
    - YYYY-MM-DD 형식만 있으면 → "date" (규칙 25)
    - 전부 정수 → "int"
    - 정수와 실수 섞임 → "float"
    - 수로 못 읽는 값 섞임 → "str"
    """
    has_bool = False
    has_int = False
    has_float = False
    has_date = False
    has_str = False
    has_non_empty = False

    for v in values:
        if v is None or v == "":
            continue

        has_non_empty = True

        # bool 확인 (규칙 21: 대소문자 무관)
        if v.lower() in ("true", "false"):
            has_bool = True
            continue

        # 날짜 확인 (규칙 25)
        if _is_date(v):
            has_date = True
            continue

        # 정수 확인
        try:
            int(v)
            has_int = True
            continue
        except ValueError:
            pass

        # 실수 확인
        try:
            float(v)
            has_float = True
            continue
        except ValueError:
            pass

        # 문자열
        has_str = True

    # 규칙 22: 값이 전부 결측인 열은 "str"
    if not has_non_empty:
        return "str"

    if has_str:
        return "str"
    elif has_bool:
        # bool과 다른 타입이 섞이면 str
        if has_int or has_float or has_date:
            return "str"
        return "bool"
    elif has_date:
        # date와 다른 타입이 섞이면 str
        if has_int or has_float:
            return "str"
        return "date"
    elif has_float:
        return "float"
    else:
        return "int"


def _convert_value(v: Optional[str], col_type: str) -> Any:
    """값을 지정된 타입으로 변환한다."""
    if v is None or v == "":
        return None

    if col_type == "bool":
        # 규칙 21: true/false 대소문자 무관
        if v.lower() == "true":
            return True
        elif v.lower() == "false":
            return False
        else:
            return None
    elif col_type == "int":
        try:
            return int(v)
        except ValueError:
            return None
    elif col_type == "float":
        try:
            return float(v)
        except ValueError:
            return None
    elif col_type == "date":
        # 규칙 25: YYYY-MM-DD → datetime.date
        try:
            return datetime.datetime.strptime(v, "%Y-%m-%d").date()
        except ValueError:
            return None
    else:  # str
        return v


def normalize(text: str, sep: str = ",") -> dict:
    """CSV 텍스트를 정규화해서 표 사전으로 변환한다.

    반환값: {"columns": [{"name": str, "type": str}, …], "rows": [[값, …], …]}

    규칙 1: 첫 줄이 헤더다. 이름의 앞뒤 공백은 버린다.
    규칙 6: 빈 칸은 결측(None)이다.
    규칙 7: 열이 헤더보다 적은 행은 뒤를 None으로 채운다.
    규칙 8: 열이 헤더보다 많은 행은 NormError.
    규칙 18: 입력 헤더에 같은 이름이 둘 이상이면 NormError.
    규칙 20: 빈 입력은 {"columns": [], "rows": []} 다.
    """
    lines = text.split('\n')

    # 빈 입력 처리 (규칙 20)
    if not text.strip():
        return {"columns": [], "rows": []}

    # 헤더 파싱 (규칙 1)
    if not lines[0].strip():
        return {"columns": [], "rows": []}

    header_raw = _parse_csv_line(lines[0], sep)
    columns = [{"name": name.strip(), "type": ""} for name in header_raw]

    # 중복된 헤더 확인 (규칙 18)
    col_names = [col["name"] for col in columns]
    if len(col_names) != len(set(col_names)):
        raise NormError("입력 헤더에 같은 이름이 둘 이상이다")

    num_cols = len(columns)

    # 행 파싱
    rows_raw = []
    for line_idx in range(1, len(lines)):
        line = lines[line_idx]
        if not line.strip():
            continue

        values = _parse_csv_line(line, sep)

        # 규칙 8: 열이 헤더보다 많은 행은 NormError
        if len(values) > num_cols:
            raise NormError(f"행 {line_idx + 1}: 열 개수가 헤더보다 많다")

        # 규칙 7: 열이 헤더보다 적은 행은 뒤를 None으로 채운다
        while len(values) < num_cols:
            values.append(None)

        rows_raw.append(values)

    # 타입 추론 (규칙 2-4)
    for col_idx, col in enumerate(columns):
        col_values = [row[col_idx] for row in rows_raw]
        col["type"] = _infer_type(col_values)

    # 값 변환
    rows = []
    for row in rows_raw:
        converted_row = []
        for col_idx, val in enumerate(row):
            col_type = columns[col_idx]["type"]
            converted_row.append(_convert_value(val, col_type))
        rows.append(converted_row)

    return {"columns": columns, "rows": rows}


def render(table: dict, sep: str = ",") -> str:
    """표를 CSV 텍스트로 변환한다.

    규칙 10: normalize(render(t)) == t
    규칙 21: bool은 "true"/"false"로 렌더링
    규칙 25: date는 "YYYY-MM-DD"로 렌더링
    """
    if not table.get("columns"):
        return ""

    lines = []

    # 헤더 행
    header_values = [col["name"] for col in table["columns"]]
    header_line = _render_csv_line(header_values, sep)
    lines.append(header_line)

    # 데이터 행
    for row in table.get("rows", []):
        row_str_values = []
        for val in row:
            if val is None:
                row_str_values.append("")
            elif isinstance(val, datetime.date):
                # 규칙 25: date는 YYYY-MM-DD 형식
                row_str_values.append(val.strftime("%Y-%m-%d"))
            elif isinstance(val, bool):
                # 규칙 21: bool은 소문자로
                row_str_values.append("true" if val else "false")
            else:
                row_str_values.append(str(val))
        row_line = _render_csv_line(row_str_values, sep)
        lines.append(row_line)

    return '\n'.join(lines)


def _render_csv_line(values: List[str], sep: str = ",") -> str:
    """값들의 리스트를 CSV 라인으로 렌더링한다."""
    output = []
    for v in values:
        # 값에 sep, 줄바꿈, 따옴표가 있으면 큰따옴표로 감싸고 따옴표를 ""로 치환
        if sep in v or '\n' in v or '\r' in v or '"' in v:
            escaped = v.replace('"', '""')
            output.append(f'"{escaped}"')
        else:
            output.append(v)
    return sep.join(output)


def select(table: dict, names: List[str]) -> dict:
    """그 순서로 열을 고른다. 없는 이름이면 NormError.

    규칙 11: select(table, names) — 그 순서로 열을 고른다. 없는 이름이면 NormError.
    """
    col_names = {col["name"]: idx for idx, col in enumerate(table["columns"])}

    # 없는 이름 확인
    for name in names:
        if name not in col_names:
            raise NormError(f"열 '{name}'이 없다")

    # 새로운 테이블 구성
    new_columns = []
    col_indices = []
    for name in names:
        idx = col_names[name]
        col_indices.append(idx)
        new_columns.append(table["columns"][idx])

    new_rows = []
    for row in table["rows"]:
        new_row = [row[idx] for idx in col_indices]
        new_rows.append(new_row)

    return {"columns": new_columns, "rows": new_rows}


def where(table: dict, name: str, op: str, value: Any) -> dict:
    """조건에 맞는 행을 필터링한다.

    규칙 12: op는 == != < <= > >=
    규칙 12: 그 열이 결측인 행은 언제나 빠진다.
    규칙 12: 모르는 연산자면 NormError.
    """
    valid_ops = {"==", "!=", "<", "<=", ">", ">="}
    if op not in valid_ops:
        raise NormError(f"모르는 연산자: {op}")

    col_names = {col["name"]: idx for idx, col in enumerate(table["columns"])}
    if name not in col_names:
        raise NormError(f"열 '{name}'이 없다")

    col_idx = col_names[name]

    new_rows = []
    for row in table["rows"]:
        cell_val = row[col_idx]

        # 규칙 12: 결측인 행은 언제나 빠진다
        if cell_val is None:
            continue

        # 조건 확인
        if op == "==":
            if cell_val == value:
                new_rows.append(row)
        elif op == "!=":
            if cell_val != value:
                new_rows.append(row)
        elif op == "<":
            if cell_val < value:
                new_rows.append(row)
        elif op == "<=":
            if cell_val <= value:
                new_rows.append(row)
        elif op == ">":
            if cell_val > value:
                new_rows.append(row)
        elif op == ">=":
            if cell_val >= value:
                new_rows.append(row)

    return {"columns": table["columns"], "rows": new_rows}


def order_by(table: dict, name: str, desc: bool = False) -> dict:
    """열을 기준으로 정렬한다.

    규칙 13: 결측은 언제나 맨 뒤다.
    """
    col_names = {col["name"]: idx for idx, col in enumerate(table["columns"])}
    if name not in col_names:
        raise NormError(f"열 '{name}'이 없다")

    col_idx = col_names[name]

    # 규칙 13: 결측과 비결측 분리
    non_null_rows = []
    null_rows = []

    for row in table["rows"]:
        if row[col_idx] is None:
            null_rows.append(row)
        else:
            non_null_rows.append(row)

    # 비결측 행 정렬
    non_null_rows.sort(key=lambda row: row[col_idx], reverse=desc)

    # 규칙 13: 결측은 맨 뒤
    sorted_rows = non_null_rows + null_rows

    return {"columns": table["columns"], "rows": sorted_rows}


def agg(table: dict, name: str, how: str) -> Any:
    """집계 함수를 적용한다.

    규칙 14: how는 sum mean min max count
    규칙 14: 결측은 무시한다.
    규칙 14: count는 결측 아닌 값의 수다. 값이 하나도 없으면 count는 0, 나머지는 None.
    """
    valid_hows = {"sum", "mean", "min", "max", "count"}
    if how not in valid_hows:
        raise NormError(f"모르는 집계: {how}")

    col_names = {col["name"]: idx for idx, col in enumerate(table["columns"])}
    if name not in col_names:
        raise NormError(f"열 '{name}'이 없다")

    col_idx = col_names[name]

    # 규칙 14: 결측은 무시
    values = [row[col_idx] for row in table["rows"] if row[col_idx] is not None]

    if how == "count":
        return len(values)

    if not values:
        # 규칙 14: 값이 하나도 없으면 count는 0, 나머지는 None
        return None

    if how == "sum":
        return sum(values)
    elif how == "mean":
        return sum(values) / len(values)
    elif how == "min":
        return min(values)
    elif how == "max":
        return max(values)


def group_count(table: dict, name: str) -> dict:
    """값별로 개수를 센다.

    규칙 15: 값 → 개수의 사전. 결측도 열쇠 하나다.
    """
    col_names = {col["name"]: idx for idx, col in enumerate(table["columns"])}
    if name not in col_names:
        raise NormError(f"열 '{name}'이 없다")

    col_idx = col_names[name]

    counts = {}
    for row in table["rows"]:
        val = row[col_idx]
        # 규칙 15: 결측도 열쇠 하나다
        key = val if val is not None else None
        counts[key] = counts.get(key, 0) + 1

    return counts


def rename(table: dict, old: str, new: str) -> dict:
    """열 이름을 바꾼다.

    규칙 16: 이름이 겹치면 NormError.
    """
    col_names = {col["name"]: idx for idx, col in enumerate(table["columns"])}

    if old not in col_names:
        raise NormError(f"열 '{old}'이 없다")

    if new != old and new in col_names:
        raise NormError(f"열 '{new}'이 이미 존재한다")

    new_columns = []
    for col in table["columns"]:
        if col["name"] == old:
            new_columns.append({"name": new, "type": col["type"]})
        else:
            new_columns.append(col)

    return {"columns": new_columns, "rows": table["rows"]}


def add_column(table: dict, name: str, values: List[Any]) -> dict:
    """새 열을 추가한다.

    규칙 17: 이름이 겹치거나 길이가 안 맞으면 NormError.
    규칙 21: bool 타입 지원
    """
    col_names = {col["name"]: idx for idx, col in enumerate(table["columns"])}

    if name in col_names:
        raise NormError(f"열 '{name}'이 이미 존재한다")

    if len(values) != len(table["rows"]):
        raise NormError(f"값의 개수({len(values)})가 행 개수({len(table['rows'])})와 맞지 않다")

    # 값을 문자열로 변환 (bool은 소문자로)
    str_values = []
    for v in values:
        if v is None:
            str_values.append(None)
        elif isinstance(v, bool):
            str_values.append("true" if v else "false")
        else:
            str_values.append(str(v))

    # 열의 타입 추론
    inferred_type = _infer_type(str_values)

    # 값 변환
    converted_values = []
    for str_v in str_values:
        if str_v is None:
            converted_values.append(None)
        else:
            converted_values.append(_convert_value(str_v, inferred_type))

    new_columns = table["columns"] + [{"name": name, "type": inferred_type}]

    new_rows = []
    for row_idx, row in enumerate(table["rows"]):
        new_rows.append(row + [converted_values[row_idx]])

    return {"columns": new_columns, "rows": new_rows}


def to_json(table: dict) -> str:
    """표를 JSON 배열 문자열로 변환한다.

    규칙 23: 행마다 {열이름: 값} 인 JSON 배열 문자열. 결측은 null 이다.
    """
    rows_as_dicts = []
    for row in table.get("rows", []):
        row_dict = {}
        for col_idx, col in enumerate(table["columns"]):
            row_dict[col["name"]] = row[col_idx]
        rows_as_dicts.append(row_dict)

    return json.dumps(rows_as_dicts, ensure_ascii=False)


def join(a: dict, b: dict, on: str) -> dict:
    """`on` 열로 inner join한다.

    규칙 24:
    - `on` 열로 inner join
    - 결과 열: `a`의 전부 + `b`의 `on` 아닌 것
    - `on` 이 결측인 행은 안 붙는다
    - 열 이름이 겹치면 `NormError`
    """
    # `on` 열이 두 표에 모두 있는지 확인
    a_col_names = {col["name"]: idx for idx, col in enumerate(a["columns"])}
    b_col_names = {col["name"]: idx for idx, col in enumerate(b["columns"])}

    if on not in a_col_names:
        raise NormError(f"표 a에 열 '{on}'이 없다")
    if on not in b_col_names:
        raise NormError(f"표 b에 열 '{on}'이 없다")

    # 결과 표의 열 구성
    # a의 모든 열 + b의 on 이 아닌 열
    result_columns = list(a["columns"])
    a_col_names_set = {col["name"] for col in a["columns"]}

    for col in b["columns"]:
        if col["name"] == on:
            continue
        if col["name"] in a_col_names_set:
            raise NormError(f"열 '{col['name']}'이 두 표에 모두 있다")
        result_columns.append(col)

    # join 수행
    result_rows = []
    on_col_a_idx = a_col_names[on]
    on_col_b_idx = b_col_names[on]

    for a_row in a["rows"]:
        a_on_val = a_row[on_col_a_idx]

        # 규칙 24: on 이 결측인 행은 안 붙는다
        if a_on_val is None:
            continue

        for b_row in b["rows"]:
            b_on_val = b_row[on_col_b_idx]

            # 규칙 24: on 이 결측인 행은 안 붙는다
            if b_on_val is None:
                continue

            # 조인 조건 확인
            if a_on_val == b_on_val:
                # 행 결합
                # a의 모든 값 + b의 on 이 아닌 값
                joined_row = list(a_row)
                for b_col_idx, b_col in enumerate(b["columns"]):
                    if b_col["name"] != on:
                        joined_row.append(b_row[b_col_idx])

                result_rows.append(joined_row)

    return {"columns": result_columns, "rows": result_rows}


def missing_report(table: dict) -> dict:
    """열별 결측 비율을 반환한다.

    규칙 26: 열 이름 → 결측 비율(0.0~1.0)
    """
    result = {}

    if not table.get("rows"):
        # 행이 없으면 모든 열의 결측 비율은 0.0
        for col in table.get("columns", []):
            result[col["name"]] = 0.0
        return result

    total_rows = len(table["rows"])

    for col_idx, col in enumerate(table["columns"]):
        col_name = col["name"]
        missing_count = sum(1 for row in table["rows"] if row[col_idx] is None)
        result[col_name] = missing_count / total_rows

    return result


def pivot_count(table: dict, row: str, col: str) -> dict:
    """`row` 값별로 `col` 값의 개수를 센다.

    규칙 27: {행값: {열값: 개수}}
    """
    # 열 인덱스 확인
    col_names = {c["name"]: idx for idx, c in enumerate(table["columns"])}

    if row not in col_names:
        raise NormError(f"열 '{row}'이 없다")
    if col not in col_names:
        raise NormError(f"열 '{col}'이 없다")

    row_col_idx = col_names[row]
    col_col_idx = col_names[col]

    result = {}

    for r in table["rows"]:
        row_val = r[row_col_idx]
        col_val = r[col_col_idx]

        # 행 값을 키로 사용
        if row_val not in result:
            result[row_val] = {}

        # 열 값을 부분 키로 사용
        if col_val not in result[row_val]:
            result[row_val][col_val] = 0

        result[row_val][col_val] += 1

    return result
