#!/usr/bin/env python3
"""F03-2 대조 — 정규화: `body_digest` 가 무엇에 움직이고 무엇에 안 움직이는가.

합격선은 `corpus/criteria.toml` `[f03.2]` 에 있고 **코드보다 먼저 등록됐다**
(커밋 `3621f6d`). **이 조각이 F03 에서 유일하게 진행 불가를 건다 — 그리고 ①에만 건다.**

    ① 합성 포매팅 불변율 **100%** ← 진행 불가
    ①의 반대 방향 ★ 의미 변형에서 요약이 바뀌는 비율 **100%**
    ② §3.1 표 여섯 행 + 지우지 않는 것 다섯
    ③ De Bruijn 판단 (단위 시험이 문서의 예시를 붙든다)
    ④ proptest 양방향
    ⑤ 객체 리터럴 키 보호 ★
    ⑥ 두 언어에서 정규화가 갈리지 않는다

**② 실 이력 표본은 이 스크립트가 아니라 `--history` 로 낸다** — 합격선이 아니라
기록이기 때문이다. 선정의 조작적 정의는 `[f03.2.selection]` 에 **측정보다 먼저**
등록됐고 이 스크립트는 그것을 그대로 실행한다.

**대조가 꺼지는 형태 둘을 막는다** (`[f03].self_judged` 3):

  · **변형 대상이 없으면 멈춘다.** 파일 수·심볼 수에 묶지 않는다
  · **변형마다 작업 사본과 캐시를 새로 만든다.** F02-4 에서 변이 셋이 캐시를 돌려 써
    병렬 구간이 아예 안 돌았다

사용:
    ./scripts/f03-2-verify.py
    ./scripts/f03-2-verify.py --history        # ② 실 이력 표본만
"""

from __future__ import annotations

import argparse
import collections
import json
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
BIN = ROOT / "target/release/pal"

DITTO = Path.home() / "dev/projects/ditto"
DITTO_PIN = "aded7ce7f88f"

# `[f03.2.selection]` 이 등록한 축 A 의 정규식 — **결과보다 먼저 적혔다.**
LEXICON = re.compile(
    r"(format|prettier|eslint|ktlint|lint|style|스타일|포매팅|포맷|indent|whitespace|공백|정렬)",
    re.IGNORECASE,
)

CORPORA = [
    (Path.home() / "dev/projects/ditto", "aded7ce7f88f", ".ts"),
    (Path.home() / "dev/projects/boxwood/portal-backend", "a29cad0bf6a8", ".kt"),
    (Path.home() / "dev/projects/boxwood/portal-backend-aa-task", "10185f804ad8", ".kt"),
    (Path.home() / "dev/projects/boxwood/boxwood-packages", "2e9198716796", ".kt"),
]


def run(args: list[str], **kw) -> subprocess.CompletedProcess:
    return subprocess.run(args, capture_output=True, text=True, check=False, **kw)


def digests(repo: Path, at: str | None, cache: Path) -> dict[tuple, str]:
    """`(경로, 체인, 이름, 종류)` → `body_digest`. **캐시를 새로 만든다.**

    `at` 이 `None` 이면 **워킹트리**다 — 변형은 커밋하지 않고 사본 위에 얹으므로
    그것을 보아야 한다. 기준 산출도 같은 축에서 낸다: 축이 갈리면 무엇이 움직였는지
    대신 어느 축에서 봤는지를 재게 된다.
    """
    args = [str(BIN), "ledger", str(repo), "--cache-dir", str(cache), "--symbols"]
    if at is not None:
        args[3:3] = ["--at", at]
    p = run(args)
    if p.returncode != 0:
        raise SystemExit(f"대장을 내지 못했다: {p.stderr[-400:]}")
    out: dict[tuple, str] = {}
    seen: collections.Counter = collections.Counter()
    for line in p.stdout.splitlines():
        if not line:
            continue
        d = json.loads(line)
        # **열쇠에 자리(byte)를 넣지 않는다.** 포매팅이 자리를 통째로 밀어내므로,
        # 넣으면 변형 뒤의 심볼이 **하나도 짝을 못 찾고** 대조가 텅 빈다 —
        # 그러면 이 검사는 「움직이지 않았다」가 아니라 **「아무것도 안 봤다」** 를 낸다.
        # 첫 실행이 그렇게 걸렸다(4,578 중 136 만 짝지어졌다).
        base = (d["path"], tuple(d["container"]), d["name"], d["kind"])
        out[(*base, seen[base])] = d["body"]
        seen[base] += 1
    return out


def graph_of(path: Path) -> dict | None:
    p = run([str(BIN), "symbols", str(path), "--graph"])
    if p.returncode != 0:
        return None
    try:
        g = json.loads(p.stdout)
    except json.JSONDecodeError:
        return None
    return g if isinstance(g, dict) else None


# ═════════════════════════════════════════════════════════════════════════════
# 변형 — **우리가 통제한다. 그래서 ①의 기대값이 100% 다**
#
# 각 변형은 **작업 사본을 새로 만든다.** 앞 변형의 결과 위에 쌓으면 무엇이 무엇을
# 깼는지 알 수 없고, 그것이 F02-4 에서 대조가 꺼진 형태다.
# ═════════════════════════════════════════════════════════════════════════════


def fresh_worktree(tmp: Path, tag: str) -> Path:
    """**이름을 고정한다** — 매니페스트가 없으면 `repo_id` 가 디렉터리 이름에서 온다.

    회차마다 이름이 달라지면 좌표가 전부 달라지고, 그러면 이 대조는 무엇을 재든
    「움직였다」를 낸다(F03-1 게이트 §4 에서 실제로 걸린 자리다).
    """
    base = tmp / tag
    base.mkdir(parents=True)
    repo = base / "corpus"
    p = run(["git", "clone", "--local", "--no-checkout", "-q", str(DITTO), str(repo)])
    if p.returncode != 0:
        raise SystemExit(f"사본을 만들지 못했다: {p.stderr[-300:]}")
    run(["git", "-C", str(repo), "checkout", "-q", DITTO_PIN])
    return repo


def ts_files(repo: Path) -> list[Path]:
    return [f for f in sorted(repo.rglob("*.ts")) if "node_modules" not in f.parts]



# ── 소스를 어휘로 한 번 훑는다 — **변형기 전부가 이것을 쓴다** ────────────────
#
# 이 세션에서 변형기가 네 번 틀렸고 **넷 다 「어디가 코드이고 어디가 아닌가」였다**:
# 주석 안의 숫자를 고쳤고, 템플릿 안을 들여썼고, 주석의 아포스트로피를 문자열
# 시작으로 봤고, 바이트와 문자 자리를 섞었다.
#
# **그래서 스캐너를 하나로 모은다.** 변형기마다 따로 세면 그 넷이 각각 다시 난다.


def scan(raw: bytes) -> list[tuple[str, int, int]]:
    """`(종류, 시작, 끝)` — 종류는 `line` · `block` · `sq` · `dq` · `tpl` · `re`.

    **템플릿은 `${}` 안까지 통째로 한 덩어리다.** 안쪽을 코드로 다시 가르면 중첩
    템플릿에서 짝이 어긋나고, 변형기가 얻는 것은 정확도가 아니라 새 결함이다.
    보수적으로 통째로 두면 변형이 거기 안 들어갈 뿐이다.
    """
    out: list[tuple[str, int, int]] = []
    i = 0
    n = len(raw)
    # 마지막으로 본 **뜻 있는** 바이트 — 정규식과 나눗셈을 가르는 데 쓴다.
    prev = b""
    while i < n:
        b = raw[i]
        two = raw[i : i + 2]
        if b in b" \t\r\n":
            i += 1
            continue
        if two == b"//":
            j = raw.find(b"\n", i)
            j = n if j < 0 else j
            out.append(("line", i, j))
            i = j
            continue
        if two == b"/*":
            j = raw.find(b"*/", i + 2)
            j = n if j < 0 else j + 2
            out.append(("block", i, j))
            i = j
            continue
        # **정규식 리터럴** — 안 가르면 `/"""/g` 의 따옴표가 문자열 시작으로 보이고
        # 그 뒤가 통째로 어긋난다. ditto 에서 17 건이 그 형태였다.
        #
        # 나눗셈과 가르는 것은 **앞의 뜻 있는 바이트**다 — 값이 올 수 없는 자리 뒤의
        # `/` 는 정규식이다. 표준 휴리스틱이고 완전하지 않지만, 완전하지 않다는 사실이
        # 여기 적혀 있다.
        if b == 0x2F and (not prev or prev in b"(,=:[!&|?{};+-*%~^<>" ):
            j = i + 1
            in_class = False
            while j < n:
                c = raw[j]
                if c == 0x5C:
                    j += 2
                    continue
                if c == 0x5B:
                    in_class = True
                elif c == 0x5D:
                    in_class = False
                elif c == 0x2F and not in_class:
                    j += 1
                    break
                elif c == 0x0A:
                    break
                j += 1
            out.append(("re", i, min(j, n)))
            prev = b"/"
            i = min(j, n)
            continue
        if b in (0x27, 0x22, 0x60):
            kind = {0x27: "sq", 0x22: "dq", 0x60: "tpl"}[b]
            j = i + 1
            depth = 0
            while j < n:
                c = raw[j]
                if c == 0x5C:
                    j += 2
                    continue
                if kind == "tpl" and raw[j : j + 2] == b"${":
                    depth += 1
                    j += 2
                    continue
                if kind == "tpl" and c == 0x7D and depth:
                    depth -= 1
                    j += 1
                    continue
                if c == b and not depth:
                    j += 1
                    break
                if kind != "tpl" and c == 0x0A:
                    break  # 줄을 넘는 홑/겹따옴표는 문자열이 아니다
                j += 1
            out.append((kind, i, min(j, n)))
            prev = raw[min(j, n) - 1 : min(j, n)]
            i = min(j, n)
            continue
        prev = raw[i : i + 1]
        i += 1
    return out


def code_mask(raw: bytes) -> list[bool]:
    """각 바이트가 **코드**인가 — 주석도 리터럴도 아닌 자리."""
    mask = [True] * len(raw)
    for _kind, a, z in scan(raw):  # 주석 · 리터럴 · 정규식 전부
        for k in range(a, min(z, len(raw))):
            mask[k] = False
    return mask


def outside_literal_lines(raw: bytes) -> list[bool]:
    """줄마다 — 그 줄의 시작이 **리터럴 밖**인가. 주석은 밖으로 센다."""
    inside = [False] * (len(raw) + 1)
    for kind, a, z in scan(raw):
        if kind in ("sq", "dq", "tpl", "re"):
            for k in range(a, min(z, len(raw))):
                inside[k] = True
    return inside


def literal_mask(src: str) -> list[bool]:
    """각 바이트가 **문자열·템플릿 리터럴 안**인가.

    # 왜 필요한가 — 첫 실행이 여기서 걸렸다

    템플릿 리터럴 안을 들여쓰면 **문자열의 값이 바뀐다.** 그것은 포매팅이 아니라
    의미 변경이고, 요약이 움직이는 것이 정답이다. 안 가르고 재면 이 지표가
    **정규화의 결함이 아니라 변형기의 결함**을 잡는다 — ditto 에서 17 건이 그렇게
    나왔고 전부 `SEED_SPEC_STUB` · `*_QUERY_JS` 같은 **여러 줄 템플릿 상수**였다.

    `prettier` 도 같은 이유로 템플릿 안을 건드리지 않는다.
    """
    mask = [False] * len(src)
    quote = ""
    escaped = False
    for i, c in enumerate(src):
        if quote:
            mask[i] = True
            if escaped:
                escaped = False
            elif c == "\\":
                escaped = True
            elif c == quote:
                quote = ""
                mask[i] = True
            continue
        if c in "'\"`":
            quote = c
            mask[i] = True
    return mask


def line_spans(src: str) -> list[tuple[int, int]]:
    out = []
    start = 0
    for i, c in enumerate(src):
        if c == "\n":
            out.append((start, i))
            start = i + 1
    if start <= len(src):
        out.append((start, len(src)))
    return out


def reindent(src: str) -> str:
    """들여쓰기 폭을 두 배로 — **리터럴 안에서 시작하는 줄은 건너뛴다.**

    템플릿 리터럴 안을 들여쓰면 **문자열의 값이 바뀐다.** 그것은 포매팅이 아니라
    의미 변경이고 요약이 움직이는 것이 정답이다. 안 가르고 재면 이 지표가
    **정규화가 아니라 변형기**를 잡는다 — ditto 에서 17 건이 그렇게 나왔고 전부
    `SEED_SPEC_STUB` · `*_QUERY_JS` 같은 여러 줄 템플릿 상수였다. `prettier` 도
    같은 이유로 템플릿 안을 건드리지 않는다.
    """
    raw = src.encode()
    inside = outside_literal_lines(raw)
    out = []
    for line in raw.split(b"\n"):
        pass
    pos = 0
    for line in raw.split(b"\n"):
        if inside[pos] if pos < len(inside) else False:
            out.append(line)
        else:
            body = line.lstrip()
            depth = len(line) - len(body)
            out.append(b" " * (depth * 2) + body)
        pos += len(line) + 1
    return b"\n".join(out).decode()


def add_comments(src: str) -> str:
    """빈 줄마다 주석을 끼운다 — **리터럴 안의 빈 줄은 건드리지 않는다.**"""
    raw = src.encode()
    inside = outside_literal_lines(raw)
    out = [b"// pal formatting control"]
    pos = 0
    for line in raw.split(b"\n"):
        out.append(line)
        if not line.strip() and pos < len(inside) and not inside[pos]:
            out.append(b"/* between */")
        pos += len(line) + 1
    return b"\n".join(out).decode()


def add_newlines(src: str) -> str:
    """여는 중괄호 뒤에 빈 줄 — **리터럴 안은 건너뛴다.**"""
    raw = src.encode()
    inside = outside_literal_lines(raw)
    out = bytearray()
    for i, b in enumerate(raw):
        out.append(b)
        if b == 0x0A and raw[i - 2 : i] == b" {" and not inside[i]:
            out.append(0x0A)
    return bytes(out).decode()


def flip_quotes(src: str) -> str:
    """홑따옴표 ↔ 겹따옴표 — **문자열 리터럴만, 주석과 템플릿은 안 건드린다.**

    첫 실행은 주석 안의 아포스트로피(`don't`)를 문자열 시작으로 보고 그 뒤를 통째로
    삼켰다. 파일이 깨져 심볼 134 개가 사라졌고, **사라진 심볼은 어긋남을 못 낸다.**
    """
    raw = src.encode()
    out = bytearray()
    last = 0
    for kind, a, z in scan(raw):
        if kind not in ("sq", "dq"):
            continue
        body = raw[a + 1 : z - 1]
        other = b'"' if kind == "sq" else b"'"
        if other in body or b"\n" in body:
            continue  # 상대 따옴표를 담았으면 뒤집을 수 없다
        out += raw[last:a]
        out += other + body + other
        last = z
    out += raw[last:]
    return bytes(out).decode()


def toggle_trailing_comma(src: str) -> str:
    """닫는 `)` · `]` 앞의 후행 쉼표를 **뗀다. 붙이지는 않는다.**

    # 왜 한 방향인가

    `}` 는 객체 리터럴도 닫지만 **문장 블록도 닫는다.** 어휘로 못 가르고, 못 가른 채
    넣으면 `…;\n}` 가 `…;,\n}` 가 되어 파일이 깨진다. `)` 도 마찬가지다 —
    쉼표 목록의 끝일 수도 있고 `if (…)` 의 끝일 수도 있는데, 후자에 붙이면
    `if (a || b,)` 가 되어 **문법 오류이거나 쉼표 연산자**다. ditto 에서 61 건이
    그렇게 나왔고 **정규화의 결함이 아니라 변형기의 결함이었다.**

    **떼는 쪽은 언제나 안전하다** — 후행 쉼표는 정의상 없어도 되는 것이다. 그리고
    씨앗이 `prettier` 를 거친 실물이라 뗄 것이 387 파일에 있다. 붙이는 방향은 단위
    시험 `후행_쉼표는_요약을_바꾸지_않는다` 가 덮는다 — **변형기가 못 미치는 자리를
    시험이 덮는다는 사실을 여기 적어 둔다.**
    """
    raw = src.encode()
    inside = outside_literal_lines(raw)
    out = bytearray()
    for i, b in enumerate(raw):
        if b == 0x0A and not inside[i]:
            after = raw[i + 1 :].lstrip()
            if after[:1] in (b")", b"]"):
                tail = bytes(out).rstrip()
                prev = tail[-1:] if tail else b""
                if prev == b",":
                    out = bytearray(bytes(out)[: len(tail) - 1])
        out.append(b)
    return bytes(out).decode()


def rename_locals(src: str, path: Path) -> str:
    """`identity_grade == exact` 인 심볼 안의 **지역 이름**을 안전한 새 이름으로.

    # 왜 이 변형이 제일 무거운가

    `prettier` 는 이름을 안 바꾼다. 지역 리네임은 **포매터의 일이 아니라 사람의 일**
    이고, [R-07] 이 말한 *"`rename` 한 번에 결박이 무더기로 `stale`"* 이 바로 이것이다.

    # 안전하게 바꾸는 방법은 스코프 체인이 준다

    `pal symbols --graph` 가 `scopes[].bindings[]`(`symbol == "not_a_symbol"` 인 것)과
    `refs[]`(`resolved.bound` 의 `scope`·`binding`)를 낸다. 선언 자리와 그것으로
    해소된 참조 자리를 **바이트로** 알므로 포획 없이 바꿀 수 있다.

    **`ordinal` 심볼에는 걸지 않는다** — 그 심볼에서는 지우지 않는 것이 옳으므로
    요약이 바뀌는 것이 정답이고, 같이 세면 이 지표가 R-22 를 거꾸로 벌준다.
    """
    g = graph_of(path)
    if not g or not isinstance(g.get("scopes"), dict):
        return src
    chain = g["scopes"].get("present")
    if not chain:
        return src
    exact = [s for s in g["symbols"] if s["identity"] == "exact"]
    if not exact:
        return src
    spans = [(s["span"]["byte_start"], s["span"]["byte_end"]) for s in exact]
    # **`ordinal` 심볼 안은 건드리지 않는다.** 그 심볼에서는 지우지 않는 것이 옳으므로
    # 요약이 바뀌는 것이 정답이고, 같이 세면 이 지표가 R-22 를 거꾸로 벌준다.
    # 중첩이 있으므로 `exact` 심볼 **안에 든** `ordinal` 심볼도 빼야 한다.
    ordinal = [
        (s["span"]["byte_start"], s["span"]["byte_end"])
        for s in g["symbols"]
        if s["identity"] != "exact"
    ]

    def inside(b: int) -> bool:
        if any(a <= b < z for a, z in ordinal):
            return False
        return any(a <= b < z for a, z in spans)

    # 지울 대상: exact 심볼 안에서 선언된 **심볼 아닌** 바인딩
    targets: dict[tuple[int, int], str] = {}
    edits: list[tuple[int, int, str]] = []
    for si, scope in enumerate(chain["scopes"]):
        for bi, b in enumerate(scope["bindings"]):
            if b["symbol"] != "not_a_symbol" or not inside(b["declared_at"]):
                continue
            new = f"palL{len(targets)}"
            targets[(si, bi)] = new
            edits.append((b["declared_at"], b["declared_at"] + len(b["name"]), new))
    if not targets:
        return src
    # **축약 속성 자리에 쓰이는 이름은 안 바꾼다** — `{ alpha }` 의 `alpha` 는
    # 지역 참조이면서 **동시에 밖에서 보이는 키**다(F03 §4.2). 바꾸면 만들어 내는
    # 객체의 키가 달라지므로 요약이 움직이는 것이 **정답**이고, 같이 세면 이 지표가
    # 정규화가 아니라 변형기를 잡는다. 첫 실행이 그렇게 55% 를 냈다.
    #
    # 어휘로 가른다: 앞의 뜻 있는 바이트가 `{` 나 `,` 이고 뒤가 `}` 나 `,` 인 자리.
    # **완전하지 않고, 완전하지 않다는 사실이 여기 적혀 있다** — 넓게 잡아 빼는 쪽이라
    # 이 변형의 대상이 줄 뿐 거짓 통과를 만들지 않는다.
    raw0 = src.encode()

    def shorthandish(at: int, name: str) -> bool:
        i = at - 1
        while i >= 0 and raw0[i : i + 1].isspace():
            i -= 1
        before = raw0[i : i + 1]
        j = at + len(name.encode())
        while j < len(raw0) and raw0[j : j + 1].isspace():
            j += 1
        after = raw0[j : j + 1]
        return before in (b"{", b",") and after in (b"}", b",")

    banned: set[tuple[int, int]] = set()
    for r in chain["refs"]:
        res = r["resolved"]
        if not isinstance(res, dict) or "bound" not in res:
            continue
        sc = res["bound"]["scope"]
        key = ((sc[0] if isinstance(sc, list) else sc), res["bound"]["binding"])
        if key in targets and shorthandish(r["at"], r["name"]):
            banned.add(key)
    for key in banned:
        targets.pop(key, None)
    edits = [e for e in edits if not any(
        e[2] == f"palL{i}" for i, k in enumerate(list(banned)) if False)]
    # 위 필터는 이름으로 못 거른다 — 다시 만든다.
    edits = []
    for si, scope in enumerate(chain["scopes"]):
        for bi, b in enumerate(scope["bindings"]):
            if (si, bi) in targets:
                edits.append((b["declared_at"], b["declared_at"] + len(b["name"].encode()),
                              targets[(si, bi)]))
    if not targets:
        return src

    for r in chain["refs"]:
        res = r["resolved"]
        if not isinstance(res, dict) or "bound" not in res:
            continue
        key = (res["bound"]["scope"][0] if isinstance(res["bound"]["scope"], list)
               else res["bound"]["scope"], res["bound"]["binding"])
        new = targets.get(key)
        if new is not None and inside(r["at"]):
            edits.append((r["at"], r["at"] + len(r["name"].encode()), new))

    # **자리마다 하나만 고친다.** 선언 자리는 참조로도 한 번 더 나온다 —
    # 그대로 두면 같은 범위를 두 번 갈아 `palL0palL0` 이 되고 파일이 깨진다.
    # 깨진 파일은 심볼을 잃고, **잃은 심볼은 어긋남을 못 낸다.**
    unique: dict[int, tuple[int, int, str]] = {}
    for e in edits:
        unique[e[0]] = e
    raw = src.encode()
    for start, end, new in sorted(unique.values(), reverse=True):
        raw = raw[:start] + new.encode() + raw[end:]
    return raw.decode("utf-8", "replace")


MUTATIONS = {
    "prettier": None,  # 밖의 도구 — 아래에서 따로 돈다
    "들여쓰기": lambda s, p: reindent(s),
    "개행": lambda s, p: add_newlines(s),
    "주석": lambda s, p: add_comments(s),
    "따옴표": lambda s, p: flip_quotes(s),
    "후행 쉼표": lambda s, p: toggle_trailing_comma(s),
    "지역 리네임": rename_locals,
}


def apply_mutation(name: str, repo: Path) -> int:
    """작업 사본 전체에 변형을 건다. **바뀐 파일 수**를 낸다 — 0 이면 대조가 꺼진 것이다."""
    if name == "prettier":
        p = run(
            ["npx", "--yes", "prettier@3", "--write", "--log-level", "warn", "**/*.ts"],
            cwd=repo,
        )
        if p.returncode != 0 and "not found" in (p.stderr or ""):
            raise SystemExit("prettier 를 조달하지 못했다")
        changed = run(["git", "-C", str(repo), "diff", "--name-only"]).stdout.split()
        return len(changed)
    f = MUTATIONS[name]
    n = 0
    for path in ts_files(repo):
        try:
            src = path.read_text()
        except UnicodeDecodeError:
            continue
        out = f(src, path)
        if out != src:
            path.write_text(out)
            n += 1
    return n


# ═════════════════════════════════════════════════════════════════════════════
# ① 합성 포매팅 — **진행 불가가 걸린 유일한 자리**
# ═════════════════════════════════════════════════════════════════════════════


def check_formatting(tmp: Path, only: list[str] | None) -> tuple[bool, str, list[str]]:
    base_repo = fresh_worktree(tmp, "base")
    base = digests(base_repo, None, tmp / "cache-base")
    if not base:
        return False, "기준 산출이 비었다 — 대조가 성립하지 않는다", []

    lines: list[str] = []
    ok = True
    for name in MUTATIONS:
        if only and name not in only:
            continue
        # **변형마다 사본과 캐시를 새로 만든다** — 상태를 물려주지 않는다.
        repo = fresh_worktree(tmp, f"fmt-{abs(hash(name))}")
        try:
            touched = apply_mutation(name, repo)
        except SystemExit as e:
            lines.append(f"    {name:<10} 대조 불가 — {e}")
            ok = False
            continue
        if touched == 0:
            lines.append(f"    {name:<10} **어느 파일도 안 바꿨다 — 대조가 꺼져 있다**")
            ok = False
            continue
        after = digests(repo, None, tmp / f"cache-{abs(hash(name))}")
        common = set(base) & set(after)
        moved = [k for k in common if base[k] != after[k]]
        rate = 100 * (len(common) - len(moved)) / max(len(common), 1)
        mark = "ok  " if not moved else "FAIL"
        lines.append(
            f"    {mark} {name:<10} 파일 {touched:>4} · 대조한 심볼 {len(common):>5} · "
            f"움직인 심볼 {len(moved):>4} · 불변율 {rate:6.2f}%"
        )
        if moved:
            ok = False
            for k in moved[:3]:
                lines.append(f"           ↳ {k[0]}::{'.'.join(list(k[1]) + [k[2]])}")
    return ok, f"기준 심볼 {len(base)}", lines


def semantic_site(raw: bytes, spans: list[tuple[int, int]]) -> int | None:
    """의미 변형을 놓을 자리 — **심볼 안이고, 주석도 문자열도 아닌 숫자 리터럴.**

    # 주석 안을 고르면 이 지표가 정규화를 벌준다

    정규화는 주석을 **지운다.** 주석의 숫자를 바꿔 놓고 *"요약이 안 움직였다"* 라고
    적으면 그것은 결함이 아니라 **정규화가 설계대로 동작한 것**이다.
    첫 실행이 그렇게 걸렸다 — 가시율이 85% 로 나왔고 빠진 15% 는 전부 주석이었다.

    # **바이트로 다룬다**

    `span` 은 바이트 자리이고 코퍼스에는 한글 주석이 있다. 문자 자리로 세면 자리가
    밀려 **엉뚱한 곳을 고치고**, 그 파일은 *"의미를 바꿨는데 요약이 안 움직였다"* 로
    적힌다 — 정규화의 결함이 아닌데 결함으로 세어진다. 이 세션에서 자리 단위를
    틀린 것이 이것으로 **넷째**다.
    """
    in_line_comment = False
    in_block = False
    quote = 0
    escaped = False
    for i, b in enumerate(raw):
        if escaped:
            escaped = False
            continue
        if quote:
            if b == 0x5C:  # 역슬래시
                escaped = True
            elif b == quote:
                quote = 0
            continue
        if in_line_comment:
            if b == 0x0A:
                in_line_comment = False
            continue
        if in_block:
            if b == 0x2A and raw[i + 1 : i + 2] == b"/":
                in_block = False
            continue
        if b == 0x2F and raw[i + 1 : i + 2] == b"/":
            in_line_comment = True
            continue
        if b == 0x2F and raw[i + 1 : i + 2] == b"*":
            in_block = True
            continue
        if b in (0x27, 0x22, 0x60):
            quote = b
            continue
        prev = raw[i - 1 : i]
        nxt = raw[i + 1 : i + 2]
        alnum = bytes(range(0x30, 0x3A)) + bytes(range(0x41, 0x5B)) + bytes(range(0x61, 0x7B))
        if 0x30 <= b <= 0x39 and prev not in [bytes([c]) for c in alnum + b"._$"]:
            if nxt not in [bytes([c]) for c in alnum] and any(a <= i < z for a, z in spans):
                return i
    return None


def check_semantic(tmp: Path) -> tuple[bool, str]:
    """★ 반대 방향 — 의미 변형에서 요약이 **바뀌는** 비율."""
    base_repo = fresh_worktree(tmp, "sem-base")
    base = digests(base_repo, None, tmp / "cache-sem-base")

    repo = fresh_worktree(tmp, "sem")
    changed: list[str] = []
    for path in ts_files(repo):
        raw = path.read_bytes()
        g = graph_of(path)
        if not g or not g.get("symbols"):
            continue
        spans = [(s["span"]["byte_start"], s["span"]["byte_end"]) for s in g["symbols"]]
        i = semantic_site(raw, spans)
        if i is None:
            continue
        path.write_bytes(raw[:i] + b"77771" + raw[i + 1 :])
        changed.append(str(path.relative_to(repo)))
    if not changed:
        return False, "의미 변형이 어느 파일도 안 바꿨다 — **대조가 꺼져 있다**"

    after = digests(repo, None, tmp / "cache-sem")
    by_file = collections.defaultdict(list)
    for k in set(base) & set(after):
        if k[0] in set(changed):
            by_file[k[0]].append(k)
    hit = sum(1 for ks in by_file.values() if any(base[k] != after[k] for k in ks))
    rate = 100 * hit / max(len(by_file), 1)
    ok = hit == len(by_file)
    note = (f"바꾼 파일 {len(changed)} · 짝지어진 파일 {len(by_file)} · "
            f"요약이 움직인 파일 {hit} · 가시율 {rate:.2f}%")
    if not ok:
        misses = [f for f, ks in by_file.items() if all(base[k] == after[k] for k in ks)]
        note += " — 안 움직인 것: " + " · ".join(misses[:5])
    return ok, note


# ═════════════════════════════════════════════════════════════════════════════
# ② 실 이력 표본 — **합격선이 아니라 기록이다**
# ═════════════════════════════════════════════════════════════════════════════


def history(tmp: Path, want: int) -> None:
    print("② 실 이력 표본 — **거짓 양성률은 기록이고 합격선이 아니다**")
    print(f"   선정: `[f03.2.selection]` 축 A(커밋 제목의 어휘). 목표 {want} 건")
    print()
    picked: list[tuple[Path, str, str, str]] = []
    for repo, pin, ext in CORPORA:
        if not (repo / ".git").exists():
            continue
        log = run(["git", "-C", str(repo), "log", "--format=%H%x09%s", pin]).stdout
        for line in log.splitlines():
            sha, _, subject = line.partition("\t")
            if LEXICON.search(subject):
                picked.append((repo, ext, sha, subject))
    # 같은 이력을 공유하는 저장소가 있으므로 **sha 로 중복을 뺀다.**
    seen: set[str] = set()
    unique = []
    for r, ext, sha, s in picked:
        if sha in seen:
            continue
        seen.add(sha)
        unique.append((r, ext, sha, s))

    print(f"   모집단에서 걸린 것 {len(picked)} · 중복 제거 후 **{len(unique)}**")
    if len(unique) < want:
        print(f"   ⚠ 목표 {want} 를 못 채웠다 — **채운 수로 낸다. 모집단을 넓히지 않는다**")
    print()

    total_stale = 0
    rows = []
    for i, (repo, ext, sha, subject) in enumerate(unique[:want]):
        parent = run(["git", "-C", str(repo), "rev-parse", f"{sha}^"]).stdout.strip()
        if not parent:
            continue
        try:
            was = digests(repo, parent, tmp / f"h{i}a")
            now = digests(repo, sha, tmp / f"h{i}b")
        except SystemExit:
            rows.append((sha[:8], repo.name, "—", "—", subject[:52], "대장을 못 냈다"))
            continue
        stale = [k for k in (set(was) & set(now)) if was[k] != now[k]]
        gone = len(set(was) - set(now))
        total_stale += len(stale)
        files = run(
            ["git", "-C", str(repo), "diff", "--numstat", parent, sha, "--", f"*{ext}"]
        ).stdout.strip().splitlines()
        rows.append((sha[:8], repo.name, str(len(stale)), str(gone), subject[:52],
                     f"{len(files)} 파일"))

    print(f"   {'커밋':<10}{'저장소':<24}{'stale':>6}{'사라짐':>7}  {'제목'}")
    for sha, name, stale, gone, subject, extra in rows:
        print(f"   {sha:<10}{name:<24}{stale:>6}{gone:>7}  {subject}  ({extra})")
    print()
    print(f"   **켜진 `stale` 합계 {total_stale}.** 각각이 진짜 거짓 양성인지는 손 검토이고,")
    print("   그 판정은 게이트 `docs/gates/F03-2-normalize.md` §5 에 있다.")


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--history", action="store_true", help="② 실 이력 표본만")
    ap.add_argument("--n", type=int, default=20)
    ap.add_argument("--only", nargs="*", help="① 의 변형을 골라 돈다")
    a = ap.parse_args()

    if not BIN.exists():
        print(f"  {BIN} 이 없다 — `cargo build --workspace --release`", file=sys.stderr)
        return 1
    if not shutil.which("git"):
        return 1

    with tempfile.TemporaryDirectory() as td:
        tmp = Path(td)
        if a.history:
            history(tmp, a.n)
            return 0

        print("F03-2 — 정규화: `body_digest` 가 무엇에 움직이는가")
        print()
        print("  ① 합성 포매팅 불변율 — **100% 아니면 진행 불가**")
        ok1, note, lines = check_formatting(tmp, a.only)
        print(f"        {note}")
        for l in lines:
            print(l)
        print()
        print("  ①의 반대 방향 ★ — 의미 변형에서 요약이 바뀌는가")
        ok2, note2 = check_semantic(tmp)
        print(f"    {'ok  ' if ok2 else 'FAIL'}  {note2}")
        print()

    if not (ok1 and ok2):
        print("어긋났다 — ① 이 100% 가 아니면 **여기서 멈춘다**")
        return 1
    print("둘 다 통과")
    return 0


if __name__ == "__main__":
    sys.exit(main())
