"""과제 A 의 참조 구현. **에이전트가 못 닿는 곳에 산다.**

A2(오라클 음성 대조)의 입력이다 — 여기에 흠집을 내서 다섯 축이 발화하는지 본다.
"""


class CalcError(Exception):
    pass


_TWO = {"**"}
_OPS = {"+", "-", "*", "/", "%", "**"}


def _tokenize(s):
    out, i, n = [], 0, len(s)
    while i < n:
        c = s[i]
        if c.isspace():
            i += 1
            continue
        if c.isdigit() or (c == "." and i + 1 < n and s[i + 1].isdigit()):
            j = i
            seen_dot = False
            while j < n and (s[j].isdigit() or (s[j] == "." and not seen_dot)):
                if s[j] == ".":
                    seen_dot = True
                j += 1
            out.append(("num", float(s[i:j]) if seen_dot else int(s[i:j])))
            i = j
            continue
        if s[i:i + 2] == "**":
            out.append(("op", "**"))
            i += 2
            continue
        if c in "+-*/%":
            out.append(("op", c))
            i += 1
            continue
        if c in "()":
            out.append((c, c))
            i += 1
            continue
        raise CalcError("알 수 없는 문자: %r" % c)
    return out


def _parse(tokens):
    pos = [0]

    def peek():
        return tokens[pos[0]] if pos[0] < len(tokens) else (None, None)

    def eat():
        t = peek()
        pos[0] += 1
        return t

    def unary():
        k, v = peek()
        if k == "op" and v in ("+", "-"):
            eat()
            r = unary()
            return -r if v == "-" else r
        return power()

    def power():
        base = atom()
        k, v = peek()
        if k == "op" and v == "**":
            eat()
            return base ** unary()   # 우결합 · 지수 쪽에 단항 허용
        return base

    def atom():
        k, v = eat()
        if k == "num":
            return v
        if k == "(":
            r = expr()
            k2, _ = eat()
            if k2 != ")":
                raise CalcError("괄호가 안 닫혔다")
            return r
        raise CalcError("식이 아니다")

    def term():
        r = unary()
        while True:
            k, v = peek()
            if k == "op" and v in ("*", "/", "%"):
                eat()
                rhs = unary()
                if v == "*":
                    r = r * rhs
                elif v == "/":
                    if rhs == 0:
                        raise CalcError("0 으로 나눈다")
                    r = r / rhs
                else:
                    if rhs == 0:
                        raise CalcError("0 으로 나눈다")
                    r = r % rhs
            else:
                return r

    def expr():
        r = term()
        while True:
            k, v = peek()
            if k == "op" and v in ("+", "-"):
                eat()
                rhs = term()
                r = r + rhs if v == "+" else r - rhs
            else:
                return r

    r = expr()
    if pos[0] != len(tokens):
        raise CalcError("남은 토큰이 있다")
    return r


def evaluate(expr: str) -> float:
    if not expr or not expr.strip():
        raise CalcError("빈 식")
    return float(_parse(_tokenize(expr)))
