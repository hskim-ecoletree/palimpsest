#!/usr/bin/env python3
"""F02-3(#48) 대조 — **`body_digest` 정규화가 요구하는 것이 서는가.** [R-22] 의 판정 문장이다.

합격선 정본은 `corpus/criteria.toml` `[f02.3]` 이고 판정은 `docs/gates/F02-3-scope.md` 다.

이 스크립트가 재는 것:

    ①  스코프 체인이 `FileGraph` 에 실리는가 — 그리고 **Kotlin 에는 안 실리는가**
    ②  `identity_grade` 가 **심볼 단위**인가 — 언어 표는 선언 상한으로 남는가
    ③  정규화 불변식 넷 — **B 가 반대 방향이고 이 조각의 다섯째다**
    ④  값/타입 이름 공간이 갈리는가 — 실물 하중과 함께
    ⑤  호이스팅 · TDZ · 섀도잉 — 실물 하중과 함께

## ③ 이 코퍼스 실물 위에 서는 이유

불변식은 *"이름만 바꾼 두 소스"* 를 요구한다. 합성 픽스처로 만들면 tree-sitter 가 실물에서
만나는 경우의 수를 만나지 않는다(규율 4). 그래서 **고정 SHA 의 실재 파일에서 실재
식별자를 바꾼다** — 대상이 소스에 없으면 `✓` 를 내는 대신 멈춘다.

## ⚠ A 만 재면 *"항상 지운다"* 가 만점을 받는다

그것이 [R-22] 가 경고한 **"서로 다른 코드가 같은 digest"** 의 정확한 형태다. 그래서
**B** — `ordinal` 심볼에서는 지역 이름을 **안** 지운다 — 를 같은 무게로 잰다.

[R-22]: ../docs/plan/00-risks.md#r-22

사용:
    ./scripts/f02-3-verify.py

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
DITTO_SHA = "aded7ce7f88feb3c03238c5f9760f3a2ade4a6c1"

# ── ③ 불변식 넷 — **고정 SHA 의 실재 경로와 실재 식별자에만 묶는다** ──────────────
#
# (이름, 파일, 심볼, 기대, [(찾을 것, 바꿀 것), …])
#
# **`같다` 를 기대하는 변이는 이름만 바꾼다.** 한 글자라도 의미가 바뀌면 그것은
# 불변식 A 가 아니라 D 를 재는 것이 된다.
INVARIANTS = [
    {
        "label": "A · exact 에서 지역 이름을 바꾼다",
        "path": "rebuild/memory/query.ts",
        "symbol": "queryMemory",
        "expect": "same",
        "grade": "exact",
        "edits": [
            ("const query = opts.text", "const 지역이름 = opts.text"),
            ("if (query.length === 0)", "if (지역이름.length === 0)"),
            ("const queryTokens = query.split", "const 토큰들 = 지역이름.split"),
            ("body.includes(query)", "body.includes(지역이름)"),
            ("return queryTokens.some", "return 토큰들.some"),
        ],
    },
    {
        "label": "B · ordinal 에서 지역 이름을 바꾼다",
        "path": "src/cli/commands/impact.ts",
        "symbol": "impactCommand",
        "expect": "differ",
        "grade": "ordinal",
        "edits": [("repoRoot", "저장소뿌리")],
    },
    {
        # **기대가 「같다」인 것이 이 검사의 요점이다.**
        #
        # 안쪽 콜백의 지역 `body` 를 **바깥 파라미터와 같은 이름**(`events`)으로 바꾼다.
        # 그것은 이름만 바꾼 것이고(안쪽에서 바깥 `events` 를 쓰지 않는다) 의미가 같으므로
        # 요약도 같아야 한다.
        #
        # **이름 표였으면 달라진다.** 이름으로만 자리를 잡으면 안쪽 `events` 가 파라미터
        # `events` 의 자리를 물려받고, 그 자리는 `body` 의 자리와 다르다. 즉 이 검사는
        # *"안팎을 다른 선언으로 보았는가"* 를 요약 하나로 되묻는다 — 뭉갰으면 여기서
        # 산출이 움직인다.
        "label": "C · 안쪽 지역을 바깥 이름과 같게 바꾼다",
        "path": "rebuild/memory/query.ts",
        "symbol": "queryMemory",
        "expect": "same",
        "grade": "exact",
        "edits": [
            ("const body = e.text.toLowerCase();", "const events = e.text.toLowerCase();"),
            ("if (body.includes(query))", "if (events.includes(query))"),
            ("return queryTokens.some((t) => body.includes(t));",
             "return queryTokens.some((t) => events.includes(t));"),
        ],
    },
    {
        "label": "D · exact 에서 의미를 바꾼다",
        "path": "rebuild/memory/query.ts",
        "symbol": "queryMemory",
        "expect": "differ",
        "grade": "exact",
        "edits": [("t.length >= 3", "t.length >= 4")],
    },
    {
        "label": "D · ordinal 에서 의미를 바꾼다",
        "path": "src/cli/commands/impact.ts",
        "symbol": "impactCommand",
        "expect": "differ",
        "grade": "ordinal",
        "edits": [("args.language ?? 'javascript'", "args.language ?? 'typescript'")],
    },
]


def die(msg: str) -> None:
    raise SystemExit(f"대조가 성립하지 않는다: {msg}")


def git_show(repo: Path, path: str) -> bytes:
    r = subprocess.run(
        ["git", "-C", str(repo), "show", f"{DITTO_SHA}:{path}"], capture_output=True, check=False
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


def chain_of(graph: dict) -> dict:
    scopes = graph["scopes"]
    if "present" not in scopes:
        die("스코프 체인이 `NotBuilt` 다 — TypeScript 는 만들어야 한다")
    return scopes["present"]


def symbol(graph: dict, name: str) -> dict:
    hits = [s for s in graph["symbols"] if s["name"] == name]
    if len(hits) != 1:
        die(f"심볼 `{name}` 이 {len(hits)} 개다 — 하나여야 한다")
    return hits[0]


def apply_edits(source: bytes, edits: list[tuple[str, str]], label: str) -> bytes:
    text = source.decode("utf-8")
    for find, repl in edits:
        if find not in text:
            raise SystemExit(
                f"변이 대상을 찾지 못했다 — 「{label}」\n  찾은 것: {find!r}\n"
                "  **코퍼스 핀이 움직였거나 변이가 낡았다.** 고치지 않으면 이 자리가 조용히 꺼진다."
            )
        text = text.replace(find, repl)
    return text.encode("utf-8")


def bound(ref: dict) -> tuple[int, int] | None:
    r = ref["resolved"]
    return (r["bound"]["scope"], r["bound"]["binding"]) if isinstance(r, dict) else None


def ancestors(chain: dict, scope: int) -> list[int]:
    out, cur = [], scope
    while True:
        p = chain["scopes"][cur]["parent"]
        if p == "root":
            return out
        cur = p["enclosing"]
        out.append(cur)


def main() -> int:  # noqa: PLR0915 — 다섯 검사가 한 흐름으로 읽혀야 한다
    ap = argparse.ArgumentParser()
    ap.add_argument("--ditto", type=Path, default=Path("~/dev/projects/ditto"))
    ap.add_argument("--bin", type=Path, default=ROOT / "target/release/pal")
    a = ap.parse_args()
    pal, ditto = a.bin, a.ditto.expanduser()
    if not pal.exists():
        die(f"바이너리가 없다: {pal} — `cargo build --release` 를 먼저 하라")

    failures: list[str] = []
    names = subprocess.run(
        ["git", "-C", str(ditto), "ls-tree", "-r", "--name-only", DITTO_SHA],
        capture_output=True, text=True, check=True,
    ).stdout.split()
    ts_files = [n for n in names if n.endswith(".ts")]
    if not ts_files:
        die("ditto 에서 `.ts` 를 하나도 못 읽었다")

    # ── ① 스코프 체인이 FileGraph 에 실린다 ────────────────────────────────
    print("── ① 스코프 체인이 `FileGraph` 에 — 그리고 **Kotlin 에는 안 실린다** ──")
    ts = graph_of(pal, git_show(ditto, "rebuild/memory/query.ts"))
    kt = graph_of(pal, b"class A\nfun b() {}\n", suffix=".kt")
    has_ts = "present" in ts["scopes"]
    kt_not_built = "not_built" in json.dumps(kt["scopes"])
    print(f"  TypeScript  {'Present' if has_ts else '**NotBuilt**'} · "
          f"스코프 {len(chain_of(ts)['scopes'])} · 참조 {len(chain_of(ts)['refs'])}")
    print(f"  Kotlin      {'NotBuilt — 안 만들었다고 적었다' if kt_not_built else '**딸려 올라갔다**'}"
          f" · 등급 {kt['grade']} (기대 l1) · 심볼 등급 {kt['symbols'][0]['identity']} (기대 ordinal)")
    print(f"  언어 등급    TypeScript {ts['grade']} (기대 l2)")
    if not has_ts:
        failures.append("① TypeScript 에 스코프 체인이 없다 — R-22 의 판정 문장이 서지 않았다")
    if not kt_not_built or kt["grade"] != "l1" or kt["symbols"][0]["identity"] != "ordinal":
        failures.append("① Kotlin 이 딸려 올라갔다 — `f01-verify` 의 등급 음성 대조가 무의미해진다")
    if ts["grade"] != "l2":
        failures.append(f"① TypeScript 언어 등급이 {ts['grade']} 다 — L2 여야 한다")

    # ── ② 심볼 단위 등급 — 코퍼스 전수 ─────────────────────────────────────
    print()
    print("── ② `identity_grade` 가 **심볼 단위**인가 — 코퍼스 전수 ─────────")
    per_file = {}
    exact = ordinal = 0
    resolved = tdz = outside = 0
    shadow_files = 0
    clash = []
    for n in ts_files:
        g = graph_of(pal, git_show(ditto, n))
        ch = chain_of(g)
        per_file[n] = (g, ch)
        for s in g["symbols"]:
            if s["identity"] == "exact":
                exact += 1
            else:
                ordinal += 1
        seen: dict[tuple[str, str], set] = {}
        for r in ch["refs"]:
            b = bound(r)
            if b is not None:
                resolved += 1
                seen.setdefault((r["name"], r["namespace"]), set()).add(b)
            elif r["resolved"] == "before_declaration":
                tdz += 1
            else:
                outside += 1
        # **진짜 섀도잉만 센다** — 형제 스코프의 같은 이름은 섀도잉이 아니다.
        for (_, _), places in seen.items():
            if len(places) < 2:
                continue
            ps = sorted(places)
            if any(ps[i][0] in ancestors(ch, ps[j][0]) for i in range(len(ps)) for j in range(len(ps)) if i != j):
                shadow_files += 1
                break
        for si, sc in enumerate(ch["scopes"]):
            by: dict[tuple[str, str], set] = {}
            for b2 in sc["bindings"]:
                by.setdefault((b2["name"], b2["namespace"]), set()).add(b2["declared_at"])
            for (nm, ns), ats in by.items():
                if ns == "value" and (t := by.get((nm, "type"))) and t != ats:
                    clash.append((n, nm, si))

    total = exact + ordinal
    print(f"  심볼 {total} · exact {exact} · ordinal {ordinal}  "
          f"(exact {exact * 100 / max(total, 1):.1f}%)")
    if ordinal == 0:
        failures.append("② `ordinal` 로 내려앉은 심볼이 0 이다 — 등급이 언어 단위에 머물러 있다")
    if exact == 0:
        failures.append("② `exact` 인 심볼이 0 이다 — 스코프 해소가 아무 심볼도 올리지 못했다")
    print("  **해소 실패가 0 이어야 하는 것이 아니다** — 실패를 실패로 적는 것이 합격선이다")

    # ── ③ 불변식 넷 ────────────────────────────────────────────────────────
    print()
    print("── ③ 정규화 불변식 — **B 가 반대 방향이다** ─────────────────────")
    tested = 0
    for inv in INVARIANTS:
        base = git_show(ditto, inv["path"])
        g0 = graph_of(pal, base)
        s0 = symbol(g0, inv["symbol"])
        if s0["identity"] != inv["grade"]:
            die(f"「{inv['label']}」 의 심볼 등급이 {s0['identity']} 다 — {inv['grade']} 를 기대했다. "
                "**변이가 낡았거나 등급 규칙이 바뀌었다**")
        g1 = graph_of(pal, apply_edits(base, inv["edits"], inv["label"]))
        s1 = symbol(g1, inv["symbol"])
        same = s0["body"] == s1["body"]
        ok = same if inv["expect"] == "same" else not same
        tested += 1
        말 = "요약이 같다" if same else "요약이 바뀌었다"
        print(f"  {'✓' if ok else '✗'} {inv['label']:<28} {말:<12} ({inv['grade']} · {inv['path']})")
        if not ok:
            want = "같아야" if inv["expect"] == "same" else "달라야"
            failures.append(f"③ 「{inv['label']}」 — {want} 하는데 아니다. R-22 가 경고한 자리다")
    if tested != len(INVARIANTS):
        failures.append("③ 시험되지 않은 불변식이 있다 — 「–」 는 통과가 아니다")
    print(f"  시험한 불변식 {tested}/{len(INVARIANTS)}")

    # **C 의 구조적 절반** — 요약이 아니라 해소 자체를 본다.
    #
    # 위의 C 는 *"뭉갰으면 요약이 움직인다"* 를 되묻는 형태이고, 여기서는 안쪽과 바깥쪽이
    # 실제로 **다른 선언**을 가리키는지를 곧바로 확인한다. 둘 다 있어야 C 가 선다.
    sh = chain_of(graph_of(pal, b"function f() { const x = 1; use(x); "
                                b"{ const x = 2; use(x); } return x; }"))
    x_refs = [bound(r) for r in sh["refs"] if r["name"] == "x"]
    distinct = len({b for b in x_refs if b is not None})
    print(f"  {'✓' if distinct == 2 else '✗'} {'C · 안팎이 다른 선언으로 해소된다':<28} "
          f"서로 다른 선언 {distinct} 개 (기대 2)")
    if distinct != 2:
        failures.append("③ 섀도잉된 이름이 한 선언으로 뭉개졌다 — 스코프 체인이 아니라 이름 표다")

    # ── ④ 값/타입 이름 공간 ────────────────────────────────────────────────
    print()
    print("── ④ 값/타입 이름 공간이 갈리는가 — 실물 하중과 함께 ────────────")
    probe = graph_of(pal, b"interface Foo { a: string }\nconst Foo = 1;\n"
                          b"const x: Foo = null as unknown as Foo;\nconst y = Foo;\n")
    ch = chain_of(probe)
    t_ref = next((r for r in ch["refs"] if r["name"] == "Foo" and r["namespace"] == "type"), None)
    v_ref = next((r for r in reversed(ch["refs"]) if r["name"] == "Foo" and r["namespace"] == "value"), None)
    split = t_ref is not None and v_ref is not None and bound(t_ref) != bound(v_ref)
    print(f"  갈린다   {split}  (`interface Foo` 와 `const Foo` 가 다른 선언으로 해소됨)")
    print(f"  실물 하중  값/타입 이름이 실제로 충돌하는 자리 **{len(clash)} 건**")
    if not clash:
        print("           **이 코퍼스에서는 켜지지 않는다** — 그 사실이 기록이고,")
        print("           이 규칙은 픽스처에서만 시험됐다는 뜻이다")
    for c in clash[:5]:
        print(f"      {c[0]}  `{c[1]}`")
    if not split:
        failures.append("④ 두 이름 공간이 한쪽으로 뭉개졌다 — 해소가 조용히 틀린다")

    # ── ⑤ 호이스팅 · TDZ · 섀도잉 ──────────────────────────────────────────
    print()
    print("── ⑤ 호이스팅 · TDZ · 섀도잉 ────────────────────────────────────")
    h = chain_of(graph_of(pal, b"function outer() { return later(); }\nfunction later() { return 1; }\n"))
    hoisted = bound(next(r for r in h["refs"] if r["name"] == "later")) is not None
    z = chain_of(graph_of(pal, b"function outer() { const a = b; const b = 1; return a; }"))
    tdz_caught = next(r for r in z["refs"] if r["name"] == "b")["resolved"] == "before_declaration"
    # **함수 경계를 지나면 TDZ 가 아니다** — 이 규칙이 없으면 실물에서 거짓 양성이 난다.
    l = chain_of(graph_of(pal, b"function f() { return LATER; }\nconst LATER = 1;\n"))
    across = bound(next(r for r in l["refs"] if r["name"] == "LATER")) is not None
    print(f"  호이스팅   함수 선언을 뒤에 두어도 해소된다      {hoisted}")
    print(f"  TDZ       같은 스코프의 선언 전 참조를 안 푼다   {tdz_caught}")
    print(f"  TDZ 경계   함수 안에서 뒤에 선 상수는 해소된다    {across}")
    print(f"  섀도잉     실물 하중 — 안팎이 다른 선언으로 해소된 파일 **{shadow_files}/{len(ts_files)}**")
    print(f"  해소       파일 안 {resolved} · 선언 전 참조 {tdz} · 파일 밖 {outside}"
          f"   → 해소율 {resolved * 100 / max(resolved + tdz, 1):.2f}% "
          f"(**합격선이 아니라 관측이다**)")
    for name, ok, msg in (
        ("호이스팅", hoisted, "함수 선언이 호이스팅되지 않았다"),
        ("TDZ", tdz_caught, "선언 전 참조를 해소해 버렸다 — 스코프 체인이 아니라 이름 표다"),
        ("TDZ 경계", across, "함수 안에서 뒤에 선 상수를 TDZ 로 잡았다 — 실물에서 거짓 양성이 난다"),
    ):
        if not ok:
            failures.append(f"⑤ {name} — {msg}")
    if shadow_files == 0:
        failures.append("⑤ 섀도잉이 실물에서 한 번도 안 일어났다 — 그러면 C 가 픽스처에서만 섰다")

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
