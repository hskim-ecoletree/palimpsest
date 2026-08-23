"""과제 B 의 참조 구현. **에이전트가 못 닿는 곳에 산다.**"""
import datetime
import re


class NormError(Exception):
    pass


_DATE = re.compile(r"^\d{4}-\d{2}-\d{2}$")


def _split(line):
    out, cur, q, i = [], [], False, 0
    while i < len(line):
        c = line[i]
        if c == '"':
            if q and i + 1 < len(line) and line[i + 1] == '"':
                cur.append('"')
                i += 2
                continue
            q = not q
            i += 1
            continue
        if c == "," and not q:
            out.append("".join(cur))
            cur = []
            i += 1
            continue
        cur.append(c)
        i += 1
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
    if seen <= {int} and bool not in seen:
        return "int"
    if seen <= {int, float} and bool not in seen:
        return "float"
    return "str"


def normalize(text: str) -> dict:
    lines = [l for l in text.split("\n") if l.strip() != ""]
    if not lines:
        return {"columns": [], "rows": []}
    header = [h.strip() for h in _split(lines[0])]
    raw = []
    for line in lines[1:]:
        cells = _split(line)
        if len(cells) > len(header):
            raise NormError("열이 헤더보다 많다")
        cells = cells + [None] * (len(header) - len(cells))
        raw.append([_cast(c) for c in cells])
    cols = []
    for i, name in enumerate(header):
        t = _type_of([r[i] for r in raw])
        cols.append({"name": name, "type": t})
    for r in raw:
        for i, c in enumerate(cols):
            if c["type"] == "float" and isinstance(r[i], int) and not isinstance(r[i], bool):
                r[i] = float(r[i])
            if c["type"] == "str" and r[i] is not None and not isinstance(r[i], str):
                r[i] = "true" if r[i] is True else "false" if r[i] is False else str(r[i])
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
    if "," in s or '"' in s or "\n" in s:
        return '"' + s.replace('"', '""') + '"'
    return s


def render(table: dict) -> str:
    if not table["columns"]:
        return ""
    out = [",".join(_fmt(c["name"]) for c in table["columns"])]
    for r in table["rows"]:
        out.append(",".join(_fmt(v) for v in r))
    return "\n".join(out) + "\n"


def missing_report(table: dict) -> dict:
    n = len(table["rows"])
    rep = {}
    for i, c in enumerate(table["columns"]):
        miss = sum(1 for r in table["rows"] if r[i] is None)
        rep[c["name"]] = 0.0 if n == 0 else miss / n
    return rep
