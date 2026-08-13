#!/usr/bin/env python3
"""문법 **밖의** 독립 계수기 — Kotlin 최상위 선언 머리를 tree-sitter 없이 센다.

**이것이 `[g50.oracle]` 축 B 의 오라클이고, tree-sitter 를 한 줄도 쓰지 않는다.**
쓰면 순환이다 — *"이 문법이 틀린 트리를 내는가"* 를 그 문법에게 묻는 꼴이 된다.

# 이 계수기는 틀린다. 틀려도 되는 이유

절대값 대조에 쓰지 않는다. 쓰는 방식은 하나다 — **불일치 후보를 뜨는 것**:

    독립 계수 ≥ 1  그런데  문법의 선언 수 == 0   →   손으로 읽을 후보

그러므로 이 계수기의 오류는 후보를 **늘리거나 줄일** 뿐이고, 판정은 손 검토가 한다.

# 어디가 코드이고 어디가 아닌가 — F03 이 아홉 중 다섯을 여기서 잃었다

주석 · 문자열 · 원시 문자열(`\"\"\"`) · 문자 리터럴을 **한 스캐너에서** 가른다
(F03 지붕 §3: *"따로 세면 같은 결함이 변형기마다 다시 난다"*). 지울 때 **줄 구조를
보존한다** — 자리를 바꾸면 「0 열에서 시작하는가」가 무너진다.

사용:
    ./scripts/g50-kotlin-scan.py <파일…>            # path\tcount
    ./scripts/g50-kotlin-scan.py --corpus <디렉터리>
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

# Kotlin 최상위 선언 10종(`queries/kotlin/top-level.scm` 의 머리말과 같은 목록)의
# **머리 낱말**과, 그 앞에 붙을 수 있는 수식자.
KEYWORDS = ("class", "interface", "object", "fun", "val", "var", "typealias")
MODIFIERS = (
    "public", "private", "internal", "protected",
    "open", "abstract", "final", "sealed", "data", "value",
    "inline", "enum", "annotation", "external", "expect", "actual",
    "const", "lateinit", "override", "operator", "infix", "suspend", "tailrec",
    "inner", "noinline", "crossinline", "vararg", "reified",
)
HEAD = re.compile(
    r"^(?:(?:" + "|".join(MODIFIERS) + r")\s+)*(?:" + "|".join(KEYWORDS) + r")\b"
)


def blank_out(src: str) -> str:
    """주석·문자열·문자 리터럴을 공백으로 바꾼다. **줄 바꿈은 그대로 둔다.**

    상태 하나짜리 스캐너다. Kotlin 의 블록 주석은 **중첩된다**.
    """
    out = []
    i, n = 0, len(src)
    depth = 0  # 블록 주석 중첩 깊이

    def push(ch: str) -> None:
        out.append("\n" if ch == "\n" else " ")

    while i < n:
        c = src[i]
        if depth:
            if src.startswith("/*", i):
                depth += 1
                out.append("  ")
                i += 2
                continue
            if src.startswith("*/", i):
                depth -= 1
                out.append("  ")
                i += 2
                continue
            push(c)
            i += 1
            continue
        if src.startswith("/*", i):
            depth = 1
            out.append("  ")
            i += 2
            continue
        if src.startswith("//", i):
            while i < n and src[i] != "\n":
                push(src[i])
                i += 1
            continue
        if src.startswith('"""', i):
            out.append("   ")
            i += 3
            while i < n and not src.startswith('"""', i):
                push(src[i])
                i += 1
            # 닫는 따옴표가 셋을 넘을 수 있다(`\"\"\"…\"\"\"\"` 는 마지막 셋이 닫는다).
            out.append("   ")
            i = min(i + 3, n)
            continue
        if c == '"':
            out.append(" ")
            i += 1
            while i < n and src[i] != '"':
                if src[i] == "\\" and i + 1 < n:
                    out.append("  ")
                    i += 2
                    continue
                if src[i] == "\n":  # 닫히지 않은 문자열 — 줄에서 멈춘다
                    break
                push(src[i])
                i += 1
            if i < n and src[i] == '"':
                out.append(" ")
                i += 1
            continue
        if c == "'":
            # 문자 리터럴. **주석 안의 아포스트로피는 여기 못 온다** — 주석을 먼저 지웠다.
            j = i + 1
            buf = [" "]
            while j < n and src[j] != "'" and src[j] != "\n":
                if src[j] == "\\" and j + 1 < n:
                    buf.append("  ")
                    j += 2
                    continue
                buf.append(" ")
                j += 1
            if j < n and src[j] == "'":
                buf.append(" ")
                out.extend(buf)
                i = j + 1
                continue
            out.append(c)  # 짝이 없다 — 문자 리터럴이 아니다
            i += 1
            continue
        out.append(c)
        i += 1
    return "".join(out)


def count(src: str) -> int:
    """**0 열에서 시작하는** 선언 머리의 수."""
    return sum(1 for line in blank_out(src).split("\n") if HEAD.match(line))


def count_file(p: Path) -> int:
    return count(p.read_text(encoding="utf-8", errors="replace"))


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--corpus", type=Path)
    ap.add_argument("files", nargs="*", type=Path)
    a = ap.parse_args()

    files = sorted(a.corpus.rglob("*.kt")) if a.corpus else a.files
    if not files:
        print("파일이 없다", file=sys.stderr)
        return 1
    total = 0
    for f in files:
        c = count_file(f)
        total += c
        rel = f.relative_to(a.corpus).as_posix() if a.corpus else f.as_posix()
        print(f"{rel}\t{c}")
    print(f"# 파일 {len(files)} · 선언 {total}", file=sys.stderr)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
