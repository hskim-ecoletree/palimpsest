"""과제 B 의 참조 구현 — 표 라이브러리. **에이전트가 못 닿는 곳에 산다.**"""
import datetime
import json
import re

class NormError(Exception):
    pass

_DATE = re.compile(r"^\d{4}-\d{2}-\d{2}$")
_OPS = {"==": lambda a, b: a == b, "!=": lambda a, b: a != b,
        "<": lambda a, b: a < b, "<=": lambda a, b: a <= b,
        ">": lambda a, b: a > b, ">=": lambda a, b: a >= b}


def _split(line, sep):
    out, cur, q, i = [], [], False, 0
    while i < len(line):
        c = line[i]
        if c == '"':
            if q and i + 1 < len(line) and line[i + 1] == '"':
                cur.append('"'); i += 2; continue
            q = not q; i += 1; continue
        if c == sep and not q:
            out.append("".join(cur)); cur = []; i += 1; continue
        cur.append(c); i += 1
    if q:
        raise NormError("따옴표가 안 닫혔다")
    out.append("".join(cur))
    return out


def _cast(s):
    if s is None:
        return None
    t = s.strip()
    if t == "":
        return None
    if _DATE.match(t):
        try:
            return datetime.date.fromisoformat(t)
        except ValueError:
            return t
    if t.lower() in ("true", "false"):
        return t.lower() == "true"
    try:
        return int(t)
    except ValueError:
        pass
    try:
        return float(t)
    except ValueError:
        pass
    return t


def _type_of(vals):
    seen = {type(v) for v in vals if v is not None}
    if not seen:
        return "str"
    if seen == {bool}:
        return "bool"
    if seen == {datetime.date}:
        return "date"
    if seen == {int}:
        return "int"
    if seen <= {int, float}:
        return "float"
    return "str"


def _coerce(v, t):
    if v is None:
        return None
    if t == "float" and isinstance(v, int) and not isinstance(v, bool):
        return float(v)
    if t == "str" and not isinstance(v, str):
        return "true" if v is True else "false" if v is False else str(v)
    return v


def normalize(text: str, sep: str = ",") -> dict:
    lines = [l for l in text.replace("\r\n", "\n").split("\n") if l.strip() != ""]
    if not lines:
        return {"columns": [], "rows": []}
    header = [h.strip() for h in _split(lines[0], sep)]
    if len(set(header)) != len(header):
        raise NormError("열 이름이 겹친다")
    raw = []
    for line in lines[1:]:
        cells = _split(line, sep)
        if len(cells) > len(header):
            raise NormError("열이 헤더보다 많다")
        cells += [None] * (len(header) - len(cells))
        raw.append([_cast(c) for c in cells])
    cols = [{"name": n, "type": _type_of([r[i] for r in raw])} for i, n in enumerate(header)]
    for r in raw:
        for i, c in enumerate(cols):
            r[i] = _coerce(r[i], c["type"])
    return {"columns": cols, "rows": raw}


def _fmt(v):
    if v is None:
        return ""
    if v is True:
        return "true"
    if v is False:
        return "false"
    if isinstance(v, datetime.date):
        return v.isoformat()
    s = str(v)
    return '"' + s.replace('"', '""') + '"' if ("," in s or '"' in s or "\n" in s) else s


def render(table: dict, sep: str = ",") -> str:
    if not table["columns"]:
        return ""
    out = [sep.join(_fmt(c["name"]) for c in table["columns"])]
    for r in table["rows"]:
        out.append(sep.join(_fmt(v) for v in r))
    return "\n".join(out) + "\n"


def _idx(table, name):
    for i, c in enumerate(table["columns"]):
        if c["name"] == name:
            return i
    raise NormError(f"그런 열이 없다: {name}")


def select(table, names):
    ii = [_idx(table, n) for n in names]
    return {"columns": [dict(table["columns"][i]) for i in ii],
            "rows": [[r[i] for i in ii] for r in table["rows"]]}


def where(table, name, op, value):
    if op not in _OPS:
        raise NormError(f"모르는 연산자: {op}")
    i = _idx(table, name)
    f = _OPS[op]
    keep = []
    for r in table["rows"]:
        if r[i] is None:
            continue
        try:
            if f(r[i], value):
                keep.append(list(r))
        except TypeError:
            raise NormError("비교할 수 없는 값")
    return {"columns": [dict(c) for c in table["columns"]], "rows": keep}


def order_by(table, name, desc=False):
    i = _idx(table, name)
    있음 = [r for r in table["rows"] if r[i] is not None]
    없음 = [r for r in table["rows"] if r[i] is None]
    있음 = sorted(있음, key=lambda r: r[i], reverse=desc)
    return {"columns": [dict(c) for c in table["columns"]],
            "rows": [list(r) for r in 있음 + 없음]}


def agg(table, name, how):
    i = _idx(table, name)
    vals = [r[i] for r in table["rows"] if r[i] is not None]
    if how == "count":
        return len(vals)
    if not vals:
        return None
    if how == "sum":
        return sum(vals)
    if how == "mean":
        return sum(vals) / len(vals)
    if how == "min":
        return min(vals)
    if how == "max":
        return max(vals)
    raise NormError(f"모르는 집계: {how}")


def group_count(table, name):
    i = _idx(table, name)
    out = {}
    for r in table["rows"]:
        out[r[i]] = out.get(r[i], 0) + 1
    return out


def rename(table, old, new):
    i = _idx(table, old)
    names = [c["name"] for c in table["columns"]]
    if new in names and new != old:
        raise NormError("열 이름이 겹친다")
    cols = [dict(c) for c in table["columns"]]
    cols[i]["name"] = new
    return {"columns": cols, "rows": [list(r) for r in table["rows"]]}


def add_column(table, name, values):
    if any(c["name"] == name for c in table["columns"]):
        raise NormError("열 이름이 겹친다")
    if len(values) != len(table["rows"]):
        raise NormError("값의 수가 행 수와 다르다")
    return {"columns": [dict(c) for c in table["columns"]] + [{"name": name, "type": _type_of(values)}],
            "rows": [list(r) + [v] for r, v in zip(table["rows"], values)]}


def to_json(table) -> str:
    def enc(v):
        return v.isoformat() if isinstance(v, datetime.date) else v
    recs = [{c["name"]: enc(r[i]) for i, c in enumerate(table["columns"])}
            for r in table["rows"]]
    return json.dumps(recs, ensure_ascii=False)


def join(a, b, on):
    ia, ib = _idx(a, on), _idx(b, on)
    bcols = [c for j, c in enumerate(b["columns"]) if j != ib]
    cols = [dict(c) for c in a["columns"]] + [dict(c) for c in bcols]
    if len({c["name"] for c in cols}) != len(cols):
        raise NormError("열 이름이 겹친다")
    rows = []
    for ra in a["rows"]:
        for rb in b["rows"]:
            if ra[ia] is not None and ra[ia] == rb[ib]:
                rows.append(list(ra) + [v for j, v in enumerate(rb) if j != ib])
    return {"columns": cols, "rows": rows}


def missing_report(table) -> dict:
    n = len(table["rows"])
    return {c["name"]: (0.0 if n == 0 else
                        sum(1 for r in table["rows"] if r[i] is None) / n)
            for i, c in enumerate(table["columns"])}


def pivot_count(table, row, col):
    ir, ic = _idx(table, row), _idx(table, col)
    out = {}
    for r in table["rows"]:
        out.setdefault(r[ir], {})
        out[r[ir]][r[ic]] = out[r[ir]].get(r[ic], 0) + 1
    return out
