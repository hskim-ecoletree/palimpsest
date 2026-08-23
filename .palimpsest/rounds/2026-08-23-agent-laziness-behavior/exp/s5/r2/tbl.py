"""
Table library module for CSV data processing.
Handles type inference, normalization, and operations on tabular data.
"""

import csv
from io import StringIO
from typing import Any, Dict, List, Optional


class NormError(Exception):
    """Exception raised for normalization and operation errors."""
    pass


def normalize(text: str, sep: str = ",") -> dict:
    """
    Normalize CSV text into a structured table format.

    Returns:
        {"columns": [{"name": str, "type": str}, …], "rows": [[값, …], …]}

    Raises:
        NormError: If input is malformed or rules are violated.
    """
    if not text or text.strip() == "":
        return {"columns": [], "rows": []}

    # Parse CSV with proper quote handling
    lines = _parse_csv_lines(text, sep)

    if not lines:
        return {"columns": [], "rows": []}

    # First line is header
    header_raw = lines[0]
    header_names = [name.strip() for name in header_raw]

    # Check for duplicate column names
    if len(header_names) != len(set(header_names)):
        raise NormError("Duplicate column names in header")

    # Parse data rows
    rows_raw = lines[1:]
    normalized_rows = []

    for row_raw in rows_raw:
        if len(row_raw) > len(header_names):
            raise NormError(f"Row has more columns than header")

        # Pad with None if fewer columns
        if len(row_raw) < len(header_names):
            row_raw = row_raw + [""] * (len(header_names) - len(row_raw))

        # Trim whitespace from values
        row_trimmed = [val.strip() if isinstance(val, str) else val for val in row_raw]
        normalized_rows.append(row_trimmed)

    # Infer column types
    columns = []
    for col_idx, col_name in enumerate(header_names):
        # Collect all values for this column
        col_values = [row[col_idx] if col_idx < len(row) else "" for row in normalized_rows]

        # Infer type and convert values
        col_type, converted_values = _infer_column_type(col_values)

        # Update rows with converted values
        for row_idx, val in enumerate(converted_values):
            normalized_rows[row_idx][col_idx] = val

        columns.append({"name": col_name, "type": col_type})

    return {"columns": columns, "rows": normalized_rows}


def _parse_csv_lines(text: str, sep: str) -> List[List[str]]:
    """
    Parse CSV text handling quoted values and escaped quotes.
    Returns list of rows, each row is list of field values.
    """
    lines = []
    reader = csv.reader(StringIO(text), delimiter=sep, quotechar='"', doublequote=True)
    for row in reader:
        lines.append(row)
    return lines


def _infer_column_type(values: List[str]) -> tuple:
    """
    Infer column type based on values.
    Returns (type_str, converted_values)

    Rules:
    - All booleans → "bool" with bool values
    - All integers → "int" with int values
    - Mix of int and float → "float" with float values
    - Non-numeric values → "str" with string values
    - All empty values → "str" with None values
    - Empty values → None
    """
    converted = []
    has_float = False
    has_non_numeric = False
    has_bool = False
    has_non_empty = False

    for val in values:
        if val == "":
            converted.append(None)
        else:
            has_non_empty = True
            # Try bool first
            if val.lower() in ("true", "false"):
                converted.append(val.lower() == "true")
                has_bool = True
            else:
                # Try int
                try:
                    converted.append(int(val))
                except ValueError:
                    # Try float
                    try:
                        float_val = float(val)
                        converted.append(float_val)
                        has_float = True
                    except ValueError:
                        # Non-numeric
                        converted.append(val)
                        has_non_numeric = True

    # If all values were empty, return str type
    if not has_non_empty:
        return "str", [None] * len(values)

    # Determine final type
    if has_non_numeric or (has_bool and (has_float or any(isinstance(v, int) and v is not None for v in converted if not isinstance(v, bool)))):
        # Convert all to strings (bool + numeric is treated as string)
        final_type = "str"
        final_values = []
        for val in converted:
            if val is None:
                final_values.append(None)
            else:
                final_values.append(str(val))
    elif has_bool:
        # All booleans (or None)
        final_type = "bool"
        final_values = converted
    elif has_float:
        # Convert all to float
        final_type = "float"
        final_values = []
        for val in converted:
            if val is None:
                final_values.append(None)
            else:
                final_values.append(float(val))
    else:
        # All integers (or None)
        final_type = "int"
        final_values = converted

    return final_type, final_values


def render(table: dict, sep: str = ",") -> str:
    """
    Render a table back to CSV text format.

    Args:
        table: Dictionary with "columns" and "rows" keys
        sep: Field separator (default ",")

    Returns:
        CSV text representation
    """
    if not table.get("columns"):
        return ""

    lines = []

    # Header
    header = [col["name"] for col in table["columns"]]
    lines.append(_format_csv_row(header, sep))

    # Data rows
    for row in table.get("rows", []):
        formatted_row = []
        for val in row:
            if val is None:
                formatted_row.append("")
            else:
                formatted_row.append(str(val))
        lines.append(_format_csv_row(formatted_row, sep))

    return "\n".join(lines)


def _format_csv_row(values: List[str], sep: str) -> str:
    """Format a row for CSV output, handling quoting and escaping."""
    output = StringIO()
    writer = csv.writer(output, delimiter=sep, quotechar='"', doublequote=True)
    writer.writerow(values)
    return output.getvalue().rstrip('\r\n')


def select(table: dict, names: List[str]) -> dict:
    """
    Select columns by name in the specified order.

    Args:
        table: Input table
        names: List of column names to select

    Returns:
        New table with selected columns

    Raises:
        NormError: If a column name doesn't exist
    """
    col_map = {col["name"]: idx for idx, col in enumerate(table["columns"])}

    # Check all names exist
    for name in names:
        if name not in col_map:
            raise NormError(f"Column '{name}' not found")

    # Build new table
    new_columns = [table["columns"][col_map[name]] for name in names]
    new_rows = []

    for row in table["rows"]:
        new_row = [row[col_map[name]] for name in names]
        new_rows.append(new_row)

    return {"columns": new_columns, "rows": new_rows}


def where(table: dict, name: str, op: str, value: Any) -> dict:
    """
    Filter rows based on a condition.

    Args:
        table: Input table
        name: Column name to filter on
        op: Operator (==, !=, <, <=, >, >=)
        value: Value to compare against

    Returns:
        New table with filtered rows

    Raises:
        NormError: If column not found or operator unknown

    Note:
        Rows with missing values (None) are always excluded.
    """
    valid_ops = {"==", "!=", "<", "<=", ">", ">="}
    if op not in valid_ops:
        raise NormError(f"Unknown operator: {op}")

    col_idx = None
    for idx, col in enumerate(table["columns"]):
        if col["name"] == name:
            col_idx = idx
            break

    if col_idx is None:
        raise NormError(f"Column '{name}' not found")

    new_rows = []
    for row in table["rows"]:
        cell_val = row[col_idx]

        # Exclude rows with missing values
        if cell_val is None:
            continue

        # Apply operator
        if _apply_operator(cell_val, op, value):
            new_rows.append(row)

    return {"columns": table["columns"], "rows": new_rows}


def _apply_operator(left: Any, op: str, right: Any) -> bool:
    """Apply comparison operator."""
    if op == "==":
        return left == right
    elif op == "!=":
        return left != right
    elif op == "<":
        return left < right
    elif op == "<=":
        return left <= right
    elif op == ">":
        return left > right
    elif op == ">=":
        return left >= right
    else:
        return False


def order_by(table: dict, name: str, desc: bool = False) -> dict:
    """
    Sort rows by a column.

    Args:
        table: Input table
        name: Column name to sort by
        desc: If True, sort descending

    Returns:
        New table with sorted rows

    Raises:
        NormError: If column not found

    Note:
        Missing values (None) are always sorted to the end.
    """
    col_idx = None
    for idx, col in enumerate(table["columns"]):
        if col["name"] == name:
            col_idx = idx
            break

    if col_idx is None:
        raise NormError(f"Column '{name}' not found")

    # Separate rows with and without missing values
    rows_with_values = []
    rows_with_none = []

    for row in table["rows"]:
        if row[col_idx] is None:
            rows_with_none.append(row)
        else:
            rows_with_values.append(row)

    # Sort rows with values
    sorted_rows = sorted(rows_with_values, key=lambda r: r[col_idx], reverse=desc)

    # Append missing value rows at the end
    sorted_rows.extend(rows_with_none)

    return {"columns": table["columns"], "rows": sorted_rows}


def agg(table: dict, name: str, how: str) -> Any:
    """
    Aggregate a column using the specified function.

    Args:
        table: Input table
        name: Column name to aggregate
        how: Aggregation function (sum, mean, min, max, count)

    Returns:
        Aggregated value

    Raises:
        NormError: If column not found or unknown aggregation function

    Note:
        Missing values (None) are ignored.
        count returns the number of non-missing values.
        If column is empty, count returns 0, others return None.
    """
    valid_aggs = {"sum", "mean", "min", "max", "count"}
    if how not in valid_aggs:
        raise NormError(f"Unknown aggregation: {how}")

    col_idx = None
    for idx, col in enumerate(table["columns"]):
        if col["name"] == name:
            col_idx = idx
            break

    if col_idx is None:
        raise NormError(f"Column '{name}' not found")

    # Collect non-None values
    values = [row[col_idx] for row in table["rows"] if row[col_idx] is not None]

    if not values:
        if how == "count":
            return 0
        else:
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


def group_count(table: dict, name: str) -> dict:
    """
    Count occurrences of each value in a column.

    Args:
        table: Input table
        name: Column name to group by

    Returns:
        Dictionary mapping values to counts

    Raises:
        NormError: If column not found

    Note:
        Missing values (None) are counted as one key.
    """
    col_idx = None
    for idx, col in enumerate(table["columns"]):
        if col["name"] == name:
            col_idx = idx
            break

    if col_idx is None:
        raise NormError(f"Column '{name}' not found")

    counts = {}
    for row in table["rows"]:
        val = row[col_idx]
        # Use a special key for None to make it hashable in dict
        key = val if val is not None else None
        counts[key] = counts.get(key, 0) + 1

    return counts


def rename(table: dict, old: str, new: str) -> dict:
    """
    Rename a column.

    Args:
        table: Input table
        old: Current column name
        new: New column name

    Returns:
        New table with renamed column

    Raises:
        NormError: If old column not found or new name already exists
    """
    # Check old column exists
    old_idx = None
    for idx, col in enumerate(table["columns"]):
        if col["name"] == old:
            old_idx = idx
            break

    if old_idx is None:
        raise NormError(f"Column '{old}' not found")

    # Check new name doesn't already exist
    for col in table["columns"]:
        if col["name"] == new and col["name"] != old:
            raise NormError(f"Column '{new}' already exists")

    # Create new table with renamed column
    new_columns = []
    for idx, col in enumerate(table["columns"]):
        if idx == old_idx:
            new_columns.append({"name": new, "type": col["type"]})
        else:
            new_columns.append(col)

    return {"columns": new_columns, "rows": [row[:] for row in table["rows"]]}


def add_column(table: dict, name: str, values: List[Any]) -> dict:
    """
    Add a new column to the table.

    Args:
        table: Input table
        name: Name for the new column
        values: List of values for the new column

    Returns:
        New table with added column

    Raises:
        NormError: If name already exists or value count doesn't match row count
    """
    # Check name doesn't already exist
    for col in table["columns"]:
        if col["name"] == name:
            raise NormError(f"Column '{name}' already exists")

    # Check value count matches row count
    if len(values) != len(table["rows"]):
        raise NormError(f"Value count ({len(values)}) doesn't match row count ({len(table['rows'])})")

    # Infer type of new column
    col_type, converted_values = _infer_column_type([str(v) if v is not None else "" for v in values])

    # Create new table
    new_columns = table["columns"] + [{"name": name, "type": col_type}]
    new_rows = []

    for row_idx, row in enumerate(table["rows"]):
        new_row = row + [converted_values[row_idx]]
        new_rows.append(new_row)

    return {"columns": new_columns, "rows": new_rows}


def to_json(table: dict) -> str:
    """
    Convert table to JSON array format.

    Args:
        table: Input table

    Returns:
        JSON string with array of row objects
        Each row is {column_name: value, ...}
        Missing values become null
    """
    import json

    rows_as_dicts = []
    for row in table["rows"]:
        row_dict = {}
        for col_idx, col in enumerate(table["columns"]):
            val = row[col_idx]
            row_dict[col["name"]] = val
        rows_as_dicts.append(row_dict)

    return json.dumps(rows_as_dicts)


def join(a: dict, b: dict, on: str) -> dict:
    """
    Perform inner join on two tables.

    Args:
        a: First table
        b: Second table
        on: Column name to join on (must exist in both tables)

    Returns:
        New table with columns from a + columns from b (except 'on')

    Raises:
        NormError: If 'on' column not found in either table, or if column names overlap

    Note:
        Rows where 'on' column is missing (None) are not included in result.
    """
    # Check 'on' column exists in both tables
    a_on_idx = None
    for idx, col in enumerate(a["columns"]):
        if col["name"] == on:
            a_on_idx = idx
            break

    if a_on_idx is None:
        raise NormError(f"Column '{on}' not found in first table")

    b_on_idx = None
    for idx, col in enumerate(b["columns"]):
        if col["name"] == on:
            b_on_idx = idx
            break

    if b_on_idx is None:
        raise NormError(f"Column '{on}' not found in second table")

    # Check for overlapping column names (excluding 'on')
    a_names = {col["name"] for col in a["columns"]}
    b_names = {col["name"] for col in b["columns"] if col["name"] != on}
    overlap = a_names & b_names
    if overlap:
        raise NormError(f"Column names overlap: {', '.join(overlap)}")

    # Build result columns: all from a + all from b except 'on'
    result_columns = list(a["columns"]) + [col for col in b["columns"] if col["name"] != on]

    # Build result rows with inner join
    result_rows = []

    # Create lookup table for b
    b_lookup = {}
    for b_row in b["rows"]:
        b_key = b_row[b_on_idx]
        if b_key is not None:  # Skip rows with None in 'on' column
            if b_key not in b_lookup:
                b_lookup[b_key] = []
            b_lookup[b_key].append(b_row)

    # Join rows
    for a_row in a["rows"]:
        a_key = a_row[a_on_idx]
        if a_key is not None and a_key in b_lookup:  # Only join if key is not None
            for b_row in b_lookup[a_key]:
                # Combine rows: all from a + all from b except 'on'
                combined_row = list(a_row) + [b_row[j] for j in range(len(b_row)) if j != b_on_idx]
                result_rows.append(combined_row)

    return {"columns": result_columns, "rows": result_rows}
