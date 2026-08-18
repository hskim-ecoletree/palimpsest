#!/usr/bin/env python3
"""F02-1(#46) 대조 — **선언을 빠짐없이 뽑는가, 그리고 그것을 골든이 아닌 것으로 잴 수 있는가.**

합격선 정본은 `corpus/criteria.toml` `[f02.1]` 이고 판정은 `docs/gates/F02-1-extractor.md` 다.

이 스크립트가 재는 것:

    ①  심볼 리콜  — `corpus/tasks/f02-recall-sample.tsv`(손 목록)와 **집합으로** 같은가
    ②  파일 격리  — 같은 blob 을 다른 경로·다른 이름에 두면 `FileGraph` 가 바이트 단위로 같은가
    ③  음성 대조  — 넷은 반드시 바꾸고 **둘은 반드시 안 바꾼다**
    ④  Kotlin 불변 — S0 전수 대조 불일치 0 · `grade_of(Kotlin) == L1`
    ⑤  능력       — TypeScript 가 `Present` 로 뒤집히고 JavaScript 는 `NotBuilt` 로 남는가

**손 목록은 이 코드보다 먼저 커밋됐다**(`d77c04f`). 어긋난 것이 나오면 여기에 목록으로
찍고, 손 목록을 고치지 않는다 — 고치면 그것이 대조를 사후 조정하는 일이다.

## 음성 대조가 무엇을 비교하는가 — **좌표는 빼고 본다**

`span` 은 바이트 오프셋이라 **어떤 편집이든 움직인다.** 주석 한 줄을 더해도 뒤의 모든
선언이 밀린다. 그래서 span 까지 넣고 비교하면 *"주석만 바꾸면 산출이 안 바뀐다"* 가
**성립할 수 없는 명제**가 되고, 반대로 *"바뀌어야 한다"* 쪽 넷은 아무 편집으로나
통과한다 — 둘 다 검사가 아니게 된다.

그래서 비교 대상은 **낡음을 판정하는 축**이다: 이름·종류·`body_digest`·포함 관계·
export·import. 좌표가 움직이는 것은 정상이고, `body_digest` 를 span 과 다른 축에 둔
이유가 그것이다(옛 DESIGN §2.2 · F03 §2).

사용:
    ./scripts/f02-1-verify.py --ditto ~/dev/projects/ditto --s0-corpus /tmp/s0-corpus

종료 코드:
    0  다섯 다 통과
    1  어긋난 것이 있다 · 또는 대조가 성립하지 않았다
"""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SAMPLE = ROOT / "corpus" / "tasks" / "f02-recall-sample.tsv"

# 코퍼스 핀. **팁이 아니다** — corpus/manifest.toml `[[corpus]] id="ditto"`.
DITTO_SHA = "aded7ce7f88feb3c03238c5f9760f3a2ade4a6c1"

# ── 음성 대조 — **고정 SHA 의 실재 경로와 실재 식별자에만 묶는다** ──────────────
#
# 자라는 값(파일 수·심볼 수)에 묶으면 코퍼스가 자랄 때 조용히 꺼진다(`7fe6b62`).
# 치환 대상이 소스에 없으면 `✓` 를 내는 대신 **멈춘다**.
#
# (이름, 파일, 찾을 것, 바꿀 것, 꼬리에 붙일 것)
MUST_CHANGE = [
    (
        "선언 하나를 지운다",
        "src/core/land-commit.ts",
        "const RUN_ARTIFACT_PREFIX = '.ditto/local/runs/';\n",
        "",
        "",
    ),
    (
        "선언 하나의 이름을 바꾼다",
        "src/core/teardown.ts",
        "const BACKUP_SUFFIX = '.ditto_bak';",
        "const BACKUP_SUFFIX_RENAMED = '.ditto_bak';",
        "",
    ),
    (
        "컨테이너 안으로 선언을 옮긴다",
        "rebuild/state/relock.ts",
        "export function relockRoute(",
        "export class Holder {\n  relockRoute(",
        "}\n",
    ),
    (
        "export 를 뗀다",
        "src/core/land-commit.ts",
        "export async function landCommit(",
        "async function landCommit(",
        "",
    ),
]

MUST_NOT_CHANGE = [
    (
        "주석만 고친다",
        "rebuild/memory/query.ts",
        "  const query = opts.text.trim().toLowerCase();",
        "  // 질의를 정규화한다 — 이 주석은 코드가 아니다\n  const query = opts.text.trim().toLowerCase();",
        "",
    ),
    (
        "포매팅만 바꾼다",
        "rebuild/memory/query.ts",
        "  const query = opts.text.trim().toLowerCase();",
        "  const query   =   opts.text.trim().toLowerCase()  ;",
        "",
    ),
]


def die(msg: str) -> None:
    raise SystemExit(f"대조가 성립하지 않는다: {msg}")


def hand_list(path: Path) -> dict[str, list[tuple[int, str, str, str]]]:
    """손 목록. 선언 0 인 파일은 **빈 목록으로 남는다** — 빠뜨림과 구별되어야 한다."""
    if not path.exists():
        die(f"손 목록이 없다: {path}")
    out: dict[str, list[tuple[int, str, str, str]]] = {}
    for line in path.read_text(encoding="utf-8").splitlines():
        if line.startswith("#") or not line.strip():
            continue
        cols = line.split("\t")
        if cols[0] == "path":
            continue
        if len(cols) != 5:
            die(f"손 목록의 열이 다섯이 아니다: {line!r}")
        p, ordinal, container, name, kind = cols
        out.setdefault(p, [])
        if kind == "none":
            continue  # 선언 0 인 파일 — 자리는 잡고 항목은 없다
        out[p].append((int(ordinal), container, name, kind))
    return out


def blob(ditto: Path, path: str) -> bytes:
    r = subprocess.run(
        ["git", "-C", str(ditto), "show", f"{DITTO_SHA}:{path}"],
        capture_output=True,
        check=False,
    )
    if r.returncode != 0:
        die(f"코퍼스에서 읽지 못했다 ({DITTO_SHA}:{path}) — 핀이 도달 가능한가")
    return r.stdout


def graph_of(pal: Path, source: bytes, suffix: str = ".ts") -> dict:
    with tempfile.TemporaryDirectory() as td:
        f = Path(td) / f"input{suffix}"
        f.write_bytes(source)
        r = subprocess.run([str(pal), "symbols", "--graph", str(f)], capture_output=True, text=True)
    if r.returncode != 0:
        die(f"`pal symbols --graph` 가 실패했다: {r.stderr.strip()}")
    return json.loads(r.stdout)


def observed(graph: dict) -> list[tuple[str, str, str]]:
    """추출 목록을 손 목록과 **같은 어휘**로 편다 — (container, name, kind)."""
    symbols = graph["symbols"]
    parent = {c["child"]: c["parent"] for c in graph["contains"]}
    rows = []
    for i, s in enumerate(symbols):
        p = parent.get(i)
        container = "-" if p is None else symbols[p]["name"]
        rows.append((container, s["name"], s["kind"]))
    return rows


def compare(hand: list, got: list) -> tuple[list, list]:
    """**집합 비교.** 앞이 빠뜨린 것, 뒤가 잘못 잡은 것.

    개수가 아니라 목록을 돌려준다 — 개수만 맞추면 하나를 빠뜨리고 하나를 잘못 잡은
    파일이 통과한다.
    """
    return ([r for r in hand if r not in got], [r for r in got if r not in hand])


def freshness_axis(graph: dict) -> dict:
    """**낡음을 판정하는 축만.** `span` 은 뺀다 — 어떤 편집이든 움직인다."""
    symbols = graph["symbols"]
    parent = {c["child"]: c["parent"] for c in graph["contains"]}
    return {
        "symbols": [
            {
                "name": s["name"],
                "kind": s["kind"],
                "body": s["body"],
                "container": None if parent.get(i) is None else symbols[parent[i]]["name"],
            }
            for i, s in enumerate(symbols)
        ],
        "exports": graph["exports"],
        "imports": graph["imports"],
        "grade": graph["grade"],
        "language": graph["language"],
    }


def apply_mutation(source: bytes, find: str, replace: str, tail: str, label: str) -> bytes:
    text = source.decode("utf-8")
    if find not in text:
        raise SystemExit(
            f"변이 대상을 찾지 못했다 — 「{label}」\n  찾은 것: {find!r}\n"
            "  **코퍼스 핀이 움직였거나 변이가 낡았다.** 고치지 않으면 이 자리가 조용히 꺼진다."
        )
    return (text.replace(find, replace, 1) + tail).encode("utf-8")


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--ditto", type=Path, default=Path("~/dev/projects/ditto"))
    ap.add_argument("--s0-corpus", type=Path, required=True,
                    help="`scripts/s0-corpus.sh` 가 만든 Kotlin 1,122 파일 — ④ 가 쓴다")
    ap.add_argument("--bin", type=Path, default=ROOT / "target/release/pal")
    a = ap.parse_args()

    ditto = a.ditto.expanduser()
    pal = a.bin
    if not pal.exists():
        die(f"바이너리가 없다: {pal} — `cargo build --release` 를 먼저 하라")

    failures: list[str] = []

    # ── ① 심볼 리콜 — 개수가 아니라 목록 ──────────────────────────────────
    print("── ① 심볼 리콜 — 손 목록과 집합으로 같은가 ──────────────────────")
    hands = hand_list(SAMPLE)
    print(f"  표본 {len(hands)} 파일 · 손으로 센 선언 {sum(len(v) for v in hands.values())}")

    missing: list[str] = []   # 손 목록에 있는데 추출이 못 낸 것
    spurious: list[str] = []  # 추출이 냈는데 손 목록에 없는 것
    extracted: dict[str, list[tuple[str, str, str]]] = {}
    for path in sorted(hands):
        hand = sorted((c, n, k) for _, c, n, k in hands[path])
        got = sorted(observed(graph_of(pal, blob(ditto, path))))
        extracted[path] = got
        gone, extra = compare(hand, got)
        missing += [f"{path}  {n} ({k}, container={c})" for c, n, k in gone]
        spurious += [f"{path}  {n} ({k}, container={c})" for c, n, k in extra]

    # **이 대조가 자기에 대해 거짓말하지 않는가**(규율 7 · R-18).
    #
    # `0 · 0` 은 *"둘이 같다"* 일 수도 있고 *"비교를 안 했다"* 일 수도 있다. 그래서
    # **같은 비교 함수에** 일부러 어긋난 입력을 먹여 본다 — 한쪽에서 한 줄을 빼고,
    # 다른 쪽에 없는 줄을 지어 넣는다. 둘 다 정확히 1 건으로 잡혀야 한다.
    probe = max(hands, key=lambda p: len(hands[p]))
    hand = sorted((c, n, k) for _, c, n, k in hands[probe])
    got = extracted[probe]
    if len(hand) < 2 or not got:
        die("① 의 자기 대조를 걸 표본이 없다")
    planted = ("-", "이_이름은_소스에_없다", "function")
    a_gone, a_extra = compare(hand[1:], got)          # 손 목록에서 한 줄을 뺐다 → 잘못 잡음 1
    b_gone, b_extra = compare(hand + [planted], got)  # 없는 것을 넣었다 → 빠뜨림 1
    self_ok = (len(a_extra), len(a_gone), len(b_gone), len(b_extra)) == (1, 0, 1, 0)
    print(
        f"  자기 대조     한 줄 빼면 잘못 잡음 {len(a_extra)}(기대 1) ·"
        f" 없는 줄 넣으면 빠뜨림 {len(b_gone)}(기대 1)   ({probe}, 손 {len(hand)})"
    )
    if not self_ok:
        failures.append("① 대조가 어긋남을 잡지 못한다 — `0 · 0` 이 비교했다는 뜻이 아니다")

    print(f"  빠뜨린 것 {len(missing)} · 잘못 잡은 것 {len(spurious)}   (합격선: 0 · 0)")
    # **건수만 적고 목록을 생략하는 것은 이 합격선의 위반이다.**
    for m in missing:
        print(f"    − {m}")
    for s in spurious:
        print(f"    + {s}")
    if missing or spurious:
        failures.append(f"① 손 목록과 어긋났다 — 빠뜨림 {len(missing)} · 잘못 잡음 {len(spurious)}")

    # ── ② 파일 격리 — 경로가 산출에 새지 않는가 ────────────────────────────
    print()
    print("── ② 파일 격리 — 같은 blob · 다른 경로 · 다른 이름 ──────────────")
    probe = "src/core/coverage-manager.ts"
    src = blob(ditto, probe)
    with tempfile.TemporaryDirectory() as d1, tempfile.TemporaryDirectory() as d2:
        f1 = Path(d1) / "a-repo" / "deep" / "coverage-manager.ts"
        f2 = Path(d2) / "b-repo" / "totally" / "other" / "renamed.ts"
        for f in (f1, f2):
            f.parent.mkdir(parents=True, exist_ok=True)
            f.write_bytes(src)
        outs = []
        for f in (f1, f2):
            r = subprocess.run([str(pal), "symbols", "--graph", str(f)], capture_output=True, text=True)
            if r.returncode != 0:
                die(f"`--graph` 가 실패했다: {r.stderr.strip()}")
            outs.append(r.stdout)
    same = outs[0] == outs[1]
    print(f"  두 산출이 바이트 단위로 {'같다' if same else '**다르다**'}  ({probe})")
    if not same:
        failures.append("② 같은 blob 이 경로에 따라 다른 FileGraph 를 냈다 — 파일 밖을 보고 있다")

    graph = json.loads(outs[0])
    leaked = sorted(k for k in ("repo", "tree", "coord", "extractor", "path") if k in graph)
    print(f"  파일 밖의 사실이 실렸나: {'없다' if not leaked else '**' + ', '.join(leaked) + '**'}")
    if leaked:
        failures.append(f"② FileGraph 에 파일 밖의 사실이 실렸다: {leaked}")

    # ── ③ 음성 대조 — 넷은 바뀌고 둘은 안 바뀐다 ───────────────────────────
    print()
    print("── ③ 음성 대조 — 넷은 반드시 바꾸고 **둘은 반드시 안 바꾼다** ────")
    tested = 0
    for label, path, find, repl, tail in MUST_CHANGE:
        base = blob(ditto, path)
        mutated = apply_mutation(base, find, repl, tail, label)
        a_axis = freshness_axis(graph_of(pal, base))
        b_axis = freshness_axis(graph_of(pal, mutated))
        ok = a_axis != b_axis
        tested += 1
        print(f"  {'✓' if ok else '✗'} {label:<28} {'산출이 바뀌었다' if ok else '**안 바뀌었다**'}   ({path})")
        if not ok:
            failures.append(f"③ 「{label}」 가 산출을 안 바꿨다 — 그 고장을 아무도 못 잡는다")

    for label, path, find, repl, tail in MUST_NOT_CHANGE:
        base = blob(ditto, path)
        mutated = apply_mutation(base, find, repl, tail, label)
        a_axis = freshness_axis(graph_of(pal, base))
        b_axis = freshness_axis(graph_of(pal, mutated))
        ok = a_axis == b_axis
        tested += 1
        print(f"  {'✓' if ok else '✗'} {label:<28} {'산출이 그대로다' if ok else '**바뀌었다**'}   ({path})")
        if not ok:
            failures.append(
                f"③ 「{label}」 가 산출을 바꿨다 — 포매터 한 번에 전 심볼이 stale 로 켜진다(R-07)"
            )

    if tested != len(MUST_CHANGE) + len(MUST_NOT_CHANGE):
        failures.append("③ 시험되지 않은 대조가 있다")
    print(f"  시험한 대조 {tested}/{len(MUST_CHANGE) + len(MUST_NOT_CHANGE)}")

    # ── ④ Kotlin 이 움직이지 않는다 ────────────────────────────────────────
    print()
    print("── ④ Kotlin 불변 — 회귀가 산출로 위장하지 않게 ──────────────────")
    r = subprocess.run(
        [sys.executable, str(ROOT / "scripts/s0-compare.py"), "--corpus", str(a.s0_corpus.expanduser())],
        capture_output=True, text=True,
    )
    line = [x for x in r.stdout.splitlines() if "불일치" in x]
    print(f"  S0 전수 대조   {line[-1].strip() if line else '**돌지 않았다**'}")
    if r.returncode != 0:
        failures.append("④ S0 전수 대조가 불일치를 냈다 — Kotlin 산출이 움직였다")

    kt = next(a.s0_corpus.expanduser().rglob("*.kt"), None)
    if kt is None:
        die("S0 코퍼스에 `.kt` 가 없다 — `scripts/s0-corpus.sh` 를 먼저 돌려라")
    g = graph_of(pal, kt.read_bytes(), suffix=".kt")
    print(f"  grade_of(Kotlin)  {g['grade']}   (기대 l1)")
    if g["grade"] != "l1":
        failures.append(f"④ grade_of(Kotlin) 이 {g['grade']} 다 — L1 이어야 한다")
    for field in ("exports", "imports"):
        made = "not_built" not in json.dumps(g[field])
        print(f"  Kotlin {field:<8}  {'**빈 집합으로 위장했다**' if made else 'not_built — 안 만들었다고 적었다'}")
        if made:
            failures.append(f"④ Kotlin 의 {field} 가 빈 집합으로 나왔다 — 거짓 안전이다")
    print("  골든 대장(997 항목)은 `f01-verify.py` ⑦ 이 판정한다 — 이 스크립트가 겹쳐 세지 않는다")

    # ── ⑤ 능력 — 켠 것만 켜졌는가 ──────────────────────────────────────────
    print()
    print("── ⑤ 능력 — TypeScript 만 켜졌는가 ──────────────────────────────")
    ts = graph_of(pal, b"export const x = 1;\n", suffix=".ts")
    print(f"  TypeScript  {'Present' if 'symbols' in ts else '**NotBuilt**'}")
    if "symbols" not in ts:
        failures.append("⑤ TypeScript 능력이 안 켜졌다")

    js_path = "scripts/build-bin.mjs"
    js = graph_of(pal, blob(ditto, js_path), suffix=".mjs")
    cap = js.get("not_built", {}).get("capability", {})
    ok = cap.get("what") == "javascript-extraction"
    print(f"  JavaScript  {'NotBuilt(' + cap.get('what', '?') + ')' if cap else '**Present — 켜면 안 된다**'}   ({js_path})")
    if not ok:
        failures.append("⑤ JavaScript 가 켜졌다 — 넷이 같은 층에 선다는 것이 넷을 한꺼번에 켠다는 뜻이 아니다")

    print()
    if failures:
        print("반증 — 어긋난 것:")
        for f in failures:
            print(f"  · {f}")
        return 1
    print("다섯 다 통과")
    return 0


if __name__ == "__main__":
    sys.exit(main())
