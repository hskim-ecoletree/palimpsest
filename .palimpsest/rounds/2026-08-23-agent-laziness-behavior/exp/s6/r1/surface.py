"""미완성 표면을 센다. **`check.py`(장치)와 `score.py`(채점기)가 같은 함수를 쓴다.**

⚠ 규칙이 갈리면 두 팔의 O2 만 조용히 틀린다 — 사전부검 R2-12.
★ 「빈 본문」의 네 형태를 여기 한 자리에 못박는다: `pass` · `...` · docstring 만 ·
  `return None`(단독). `raise NotImplementedError` 는 토큰 쪽에서 따로 잡는다.
"""
import ast
import re

토큰 = ("TODO", "FIXME", "XXX", "NotImplementedError")
_토큰_re = re.compile("|".join(토큰))

# ★ 하네스 자신과 시험 파일은 모집단 밖이다 — 안 빼면 장치가 자기 소스를 센다.
제외 = {"check.py", "surface.py", "score.py"}


def 모집단인가(경로) -> bool:
    import pathlib as _p
    n = _p.Path(경로).name
    return n.endswith(".py") and n not in 제외 and not n.startswith("test_")


def _빈_본문인가(node):
    body = list(node.body)
    if body and isinstance(body[0], ast.Expr) and isinstance(body[0].value, ast.Constant) \
            and isinstance(body[0].value.value, str):
        body = body[1:]          # docstring 을 벗긴다
    if not body:
        return True              # docstring 만 있는 몸
    if len(body) != 1:
        return False
    s = body[0]
    if isinstance(s, ast.Pass):
        return True
    if isinstance(s, ast.Expr) and isinstance(s.value, ast.Constant) and s.value.value is Ellipsis:
        return True
    if isinstance(s, ast.Return) and (s.value is None
                                      or (isinstance(s.value, ast.Constant) and s.value.value is None)):
        return True
    return False


def 표면(원문: str, 파일: str = "<입력>"):
    """미완성 표면의 **위치 목록**을 낸다. `[(파일, 줄, 종류, 내용)]`"""
    찾음 = []
    for i, line in enumerate(원문.splitlines(), 1):
        for m in _토큰_re.finditer(line):
            찾음.append((파일, i, m.group(0), line.strip()[:80]))
    try:
        tree = ast.parse(원문)
    except SyntaxError as e:
        찾음.append((파일, e.lineno or 0, "구문오류", str(e)[:80]))
        return 찾음
    for node in ast.walk(tree):
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)) and _빈_본문인가(node):
            찾음.append((파일, node.lineno, "빈본문", f"def {node.name}"))
    return 찾음
