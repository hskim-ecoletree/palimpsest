#!/usr/bin/env python3
"""F02-2(#47) 대조 — **깨진 파일에서도 부분 결과가 나오는가, 그리고 못 읽은 자리가 산출에 남는가.**

합격선 정본은 `corpus/criteria.toml` `[f02.2]` 이고 판정은 `docs/gates/F02-2-partial.md` 다.

이 스크립트가 재는 것:

    ①  깨진 영역 **밖**의 선언을 하나도 잃지 않는가 — 그리고 요약도 안 바뀌는가
    ②  `ERROR` 노드 **안쪽**에서 건진 선언이 실물에 몇 건인가 — **전수 실측**
    ③  회복 지점이 개수가 아니라 **자리**인가
    ④  `ERROR` 비율 임계가 실물에서 몇 건을 강등하는가 — 그리고 **이유가 뭉개지지 않는가**
    ⑤  음성 대조 — 셋은 반드시 `partial`/강등을 만들고 **하나는 반드시 아무것도 안 움직인다**

## ① 이 "깨진 영역 밖"을 어떻게 정의하는가 — **이것이 판단이다**

깨뜨리면 뒤의 바이트가 전부 밀리므로 좌표로는 전후를 비교할 수 없다. 그래서 이렇게 잡는다:

> **회복 지점 중 가장 앞선 것보다 span 이 완전히 앞에서 끝나는 선언** — 그것이 깨진 영역
> 밖이고, 그 선언들은 이름·종류·컨테이너·`body_digest` 가 **하나도 안 바뀌어야 한다.**

`byte_start` 가 앞이면 되는 것이 아니라 `byte_end` 가 앞이어야 한다. 깨진 자리를 **품고
있는** 컨테이너는 시작이 앞서도 밖이 아니다 — 그것을 밖으로 세면 이 검사가 통과할 수
없는 것을 통과시킨다.

**뒤쪽은 재지 않는다.** 닫는 중괄호를 지우면 그 컨테이너가 나머지를 삼키는 것이 정상
회복이고(옛 F02 §4), 삼킨 범위 안에서 무엇이 살아남는지는 tree-sitter 의 회복 품질이지
우리 코드가 아니다(`[f02.2.does_not_prove].not_recovery_quality`). **삼킨 뒤에 무엇을
잃었는지는 관측으로 적는다.**

## ⑤ 가 기대값을 `partial 30` 으로 박지 않는 이유

`[f02.2.oracle].corpus_has_no_broken_kotlin` — 실물 30 건은 **깨진 코드가 아니라 우리
문법이 못 읽는 유효한 Kotlin** 이다. 30 을 정답으로 등록하면 문법을 고치는 것이 회귀로
판정된다. **그래서 재는 것은 값이 아니라 차이다** — 기준선은 그 회차의 무변이 실행이다.

사용:
    ./scripts/f02-2-verify.py --s0-corpus /tmp/s0-corpus

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

# 코퍼스 핀. **팁이 아니다** — corpus/manifest.toml.
DITTO_SHA = "aded7ce7f88feb3c03238c5f9760f3a2ade4a6c1"
PB_SHA = "a29cad0bf6a873020edd8b97b13a6430d08c7f55"

# ── 음성 대조 — **고정 SHA 의 실재 경로와 실재 식별자에만 묶는다** ──────────────
#
# 자라는 값(파일 수·심볼 수·상태별 개수)에 묶으면 코퍼스가 자랄 때 조용히 꺼진다
# (`7fe6b62`). 치환 대상이 소스에 없으면 `✓` 를 내는 대신 **멈춘다**.
#
# 셋이 각각 옛 F02 §3.4 가 이름 붙인 다른 깨짐이다.
MUST_BREAK = [
    {
        "label": "닫는 중괄호를 지운다",
        "repo": "pb",
        "path": "src/main/kotlin/kr/co/ecoletree/boxwood/common/util/StringUtil.kt",
        # **파일 뒤쪽을 고른다.** 앞쪽을 깨면 「밖」이 한 줄뿐이라 이 검사가 거의 아무것도
        # 재지 않는다 — 밖이 넓어야 *"잃지 않는다"* 가 뜻을 갖는다.
        "find": "fun isEmpty(value: Any?): Boolean {\n    return value == null || value.toString().isEmpty()\n}",
        "repl": "fun isEmpty(value: Any?): Boolean {\n    return value == null || value.toString().isEmpty()\n",
    },
    {
        "label": "문법에 없는 토큰을 넣는다",
        "repo": "ditto",
        "path": "src/core/coverage-manager.ts",
        "find": "export interface PlanDialogUserQa {",
        "repl": "§§§ ¿¿¿ &&& ⟦⟧\nexport interface PlanDialogUserQa {",
    },
    {
        "label": "파일 끝을 잘라낸다",
        "repo": "pb",
        "path": "src/main/kotlin/kr/co/ecoletree/boxwood/role/model/dto/RoleGroupDto.kt",
        # 잘라내는 자리도 **실재 식별자**에 묶는다 — 바이트 수에 묶으면 코퍼스가 움직일 때
        # 엉뚱한 자리가 잘리고, 그래도 `partial` 이 나오므로 조용히 통과한다.
        "truncate_after": "data class CreateRoleGroupRequest(\n    val groupName: String,",
    },
]


def die(msg: str) -> None:
    raise SystemExit(f"대조가 성립하지 않는다: {msg}")


def git_show(repo: Path, sha: str, path: str) -> bytes:
    r = subprocess.run(
        ["git", "-C", str(repo), "show", f"{sha}:{path}"], capture_output=True, check=False
    )
    if r.returncode != 0:
        die(f"코퍼스에서 읽지 못했다 ({sha}:{path}) — 핀이 도달 가능한가")
    return r.stdout


def graph_of(pal: Path, source: bytes, suffix: str) -> dict:
    with tempfile.TemporaryDirectory() as td:
        f = Path(td) / f"input{suffix}"
        f.write_bytes(source)
        r = subprocess.run([str(pal), "symbols", "--graph", str(f)], capture_output=True, text=True)
    if r.returncode != 0:
        die(f"`pal symbols --graph` 가 실패했다: {r.stderr.strip()}")
    return json.loads(r.stdout)


def rows(graph: dict) -> list[tuple[str, str, str, str, int, int]]:
    """(컨테이너, 이름, 종류, 요약, 시작, 끝) — 좌표는 ① 의 전후 판정에만 쓴다."""
    symbols = graph["symbols"]
    parent = {c["child"]: c["parent"] for c in graph["contains"]}
    out = []
    for i, s in enumerate(symbols):
        p = parent.get(i)
        out.append((
            "-" if p is None else symbols[p]["name"],
            s["name"],
            s["kind"],
            s["body"],
            s["span"]["byte_start"],
            s["span"]["byte_end"],
        ))
    return out


def first_site(graph: dict) -> int | None:
    sites = graph["recovery_sites"]
    return min(s["span"]["byte_start"] for s in sites) if sites else None


def outside(rs: list, boundary: int) -> list:
    """**`byte_end` 가 앞이어야 밖이다.** 깨진 자리를 품은 컨테이너는 밖이 아니다."""
    return [r[:4] for r in rs if r[5] <= boundary]


def mutate(source: bytes, m: dict) -> bytes:
    text = source.decode("utf-8")
    if "truncate_after" in m:
        anchor = m["truncate_after"]
        if anchor not in text:
            raise SystemExit(
                f"변이 대상을 찾지 못했다 — 「{m['label']}」\n  찾은 것: {anchor!r}\n"
                "  **코퍼스 핀이 움직였거나 변이가 낡았다.** 고치지 않으면 이 자리가 조용히 꺼진다."
            )
        return text[: text.index(anchor) + len(anchor)].encode("utf-8")
    if m["find"] not in text:
        raise SystemExit(
            f"변이 대상을 찾지 못했다 — 「{m['label']}」\n  찾은 것: {m['find']!r}\n"
            "  **코퍼스 핀이 움직였거나 변이가 낡았다.** 고치지 않으면 이 자리가 조용히 꺼진다."
        )
    if text.count(m["find"]) != 1:
        die(f"「{m['label']}」 의 치환 대상이 {text.count(m['find'])} 번 나온다 — 자리가 유일해야 한다")
    return text.replace(m["find"], m["repl"], 1).encode("utf-8")


def inside_error(graph: dict) -> list[str]:
    """`ERROR` 자리 **안쪽**에서 나온 선언들."""
    sites = graph["recovery_sites"]
    return [
        s["name"]
        for s in graph["symbols"]
        if any(x["span"]["byte_start"] <= s["span"]["byte_start"] < x["span"]["byte_end"] for x in sites)
    ]


def main() -> int:  # noqa: PLR0915 — 다섯 검사가 한 흐름으로 읽혀야 한다
    ap = argparse.ArgumentParser()
    ap.add_argument("--ditto", type=Path, default=Path("~/dev/projects/ditto"))
    ap.add_argument("--pb", type=Path, default=Path("~/dev/projects/boxwood/portal-backend"))
    ap.add_argument("--s0-corpus", type=Path, required=True,
                    help="`scripts/s0-corpus.sh` 가 만든 Kotlin 1,122 파일 — ② 가 전수로 쓴다")
    ap.add_argument("--bin", type=Path, default=ROOT / "target/release/pal")
    a = ap.parse_args()

    pal = a.bin
    if not pal.exists():
        die(f"바이너리가 없다: {pal} — `cargo build --release` 를 먼저 하라")
    repos = {"ditto": (a.ditto.expanduser(), DITTO_SHA, ".ts"),
             "pb": (a.pb.expanduser(), PB_SHA, ".kt")}
    s0 = a.s0_corpus.expanduser()
    if not s0.is_dir():
        die(f"S0 코퍼스가 없다: {s0} — `scripts/s0-corpus.sh` 를 먼저 돌려라")

    failures: list[str] = []

    # ── ① 깨진 영역 밖의 선언을 잃지 않는다 ────────────────────────────────
    print("── ① 깨진 영역 **밖**의 선언 — 하나도 잃지 않고 요약도 안 바뀐다 ──")
    lost_total = 0
    probes = []
    for m in MUST_BREAK:
        repo, sha, suffix = repos[m["repo"]]
        base = git_show(repo, sha, m["path"])
        before = graph_of(pal, base, suffix)
        if before["recovery_sites"]:
            die(f"「{m['label']}」 의 대상이 이미 깨져 있다 ({m['path']}) — 성한 파일이어야 한다")
        broken = graph_of(pal, mutate(base, m), suffix)
        site = first_site(broken)
        if site is None:
            failures.append(f"① 「{m['label']}」 가 회복 지점을 만들지 못했다 — 변이가 낡았다")
            continue

        want = outside(rows(before), site)
        got = rows(broken)
        got_set = {r[:4] for r in got}
        gone = [r for r in want if r not in got_set]
        lost_total += len(gone)
        probes.append((m, before, broken, site, want, got))
        print(f"  {'✓' if not gone else '✗'} {m['label']:<24} "
              f"밖의 선언 {len(want):>3} · 잃은 것 {len(gone)}   ({m['path']})")
        for g in gone:
            print(f"      − {g[1]} ({g[2]}, container={g[0]})")
        if gone:
            failures.append(f"① 「{m['label']}」 가 깨진 영역 밖의 선언을 잃었다: {[g[1] for g in gone]}")

    # **이 대조가 자기에 대해 거짓말하지 않는가** (규율 7 · R-18).
    #
    # `잃은 것 0` 은 *"다 살아남았다"* 일 수도 있고 *"밖이 비어 있었다"* 일 수도 있다.
    # 그래서 ① 각 변이에서 **밖이 실제로 비어 있지 않았는지**를 따로 세고, 같은 비교에
    # 소스에 없는 선언을 심어 그것이 잡히는지 본다.
    if len(probes) != len(MUST_BREAK):
        failures.append("① 시험되지 않은 변이가 있다 — 「–」 는 통과가 아니다")
    else:
        empty = [m["label"] for m, _, _, _, want, _ in probes if not want]
        planted = ("-", "이_이름은_소스에_없다", "function", "0" * 16)
        _, _, _, _, want0, got0 = probes[0]
        caught = planted not in {r[:4] for r in got0}
        print(f"  자기 대조     밖이 빈 변이 {len(empty)}(기대 0) · "
              f"없는 선언을 심으면 잡힌다 {caught}(기대 True)")
        if empty:
            failures.append(f"① 밖의 선언이 0 인 변이가 있다 — 그 변이는 아무것도 재지 않는다: {empty}")
        if not caught:
            failures.append("① 비교가 없는 선언을 잡지 못한다 — `잃은 것 0` 이 뜻을 갖지 않는다")

    print(f"  깨진 영역 밖에서 잃은 선언 {lost_total}   (합격선: 0)")

    # **삼킨 뒤는 관측이다** — 판정하지 않고 적는다.
    print("  삼킨 범위 안 — 관측(판정 아님):")
    for m, before, broken, site, _, got in probes:
        after_names = {(r[0], r[1], r[2]) for r in got}
        swallowed = [r[1] for r in rows(before) if r[5] > site and (r[0], r[1], r[2]) not in after_names]
        print(f"      {m['label']:<24} 삼킨 범위 뒤로 사라진 선언 {len(swallowed)}  {swallowed[:6]}")

    # ── ② ERROR 안쪽에서 건진 선언 — 전수 실측 ─────────────────────────────
    print()
    print("── ② `ERROR` 안쪽에서 건진 선언 — **두 코퍼스 전수** ────────────")
    total_inside = 0
    scanned = {}
    for label, files, suffix in (
        ("Kotlin (S0 코퍼스)", [(str(p), p.read_bytes()) for p in sorted(s0.rglob("*.kt"))], ".kt"),
        ("TypeScript (ditto)", None, ".ts"),
    ):
        if files is None:
            repo, sha, _ = repos["ditto"]
            names = subprocess.run(
                ["git", "-C", str(repo), "ls-tree", "-r", "--name-only", sha],
                capture_output=True, text=True, check=True,
            ).stdout.split()
            files = [(n, git_show(repo, sha, n)) for n in names if n.endswith(".ts")]
        n_partial = n_inside = 0
        names_inside: list[str] = []
        for path, data in files:
            g = graph_of(pal, data, suffix)
            if not g["recovery_sites"]:
                continue
            n_partial += 1
            hit = inside_error(g)
            n_inside += len(hit)
            if hit:
                names_inside.append(f"{path}  {hit[:4]}")
        total_inside += n_inside
        scanned[label] = (len(files), n_partial, n_inside)
        print(f"  {label:<22} 파일 {len(files):>5} · 회복 있는 파일 {n_partial:>3} · "
              f"**안쪽에서 건진 선언 {n_inside}**")
        for x in names_inside[:8]:
            print(f"      + {x}")
        if len(files) == 0:
            failures.append(f"② {label} 에서 잰 파일이 0 이다 — 대조가 성립하지 않았다")

    print(f"  실측 합계 {total_inside} 건 — **이 수가 옛 F02 §4 의 처분을 정한다**")

    # ── ③ 회복 지점이 자리인가 ─────────────────────────────────────────────
    print()
    print("── ③ 회복 지점 — 개수가 아니라 **자리** ─────────────────────────")
    _, broken, site = probes[0][0], probes[0][2], probes[0][3]
    sites = broken["recovery_sites"]
    has_span = all("span" in s and "kind" in s for s in sites)
    widths = [s["span"]["byte_end"] - s["span"]["byte_start"] for s in sites]
    errs = [s for s in sites if s["kind"] == "error"]
    ordered = all(
        sites[i]["span"]["byte_start"] <= sites[i + 1]["span"]["byte_start"] for i in range(len(sites) - 1)
    )
    print(f"  자리 {len(sites)} · 전부 span 과 kind 를 싣는다 {has_span} · 소스 순서 {ordered}")
    print(f"  너비 {widths} · `error` 자리 중 너비 0 인 것 "
          f"{sum(1 for s in errs if s['span']['byte_end'] == s['span']['byte_start'])}(기대 0)")
    if not has_span:
        failures.append("③ 회복 지점이 span 을 싣지 않는다 — 사용자가 어디를 못 읽었는지 모른다")
    if not ordered:
        failures.append("③ 회복 지점이 소스 순서가 아니다 — 「첫 번째 공백」이 뜻을 잃는다")
    if any(s["span"]["byte_end"] == s["span"]["byte_start"] for s in errs):
        failures.append("③ 너비 0 인 `error` 자리가 있다 — `missing` 이 `error` 로 적혔다")

    # ── ④ 임계 강등 — 실물에서 몇 건이고, 이유가 뭉개지지 않는가 ──────────
    print()
    print("── ④ `ERROR` 비율 임계 — 실물 하중과 **이유** ───────────────────")
    led = ledger_of(pal, a.pb.expanduser(), PB_SHA)
    defeated, no_extractor, partial = [], 0, []
    for e in led["entries"]:
        st = e["state"]
        if "unsupported" in st:
            r = st["unsupported"]["reason"]
            if isinstance(r, dict) and "grammar_defeated" in r:
                defeated.append((e["path"], r["grammar_defeated"]["error_ratio_percent"]))
            else:
                no_extractor += 1
        elif "partial" in st:
            partial.append(e["path"])
    print(f"  강등된 파일 {len(defeated)} · `추출기 없음` {no_extractor} · `partial` 로 남은 것 {len(partial)}")
    print(f"  강등 비율의 분포 {sorted({r for _, r in defeated})}")
    if not defeated:
        print("  **이 코퍼스에서는 임계가 켜지지 않는다** — 그 사실이 기록이다")
    for p, r in sorted(defeated, key=lambda x: -x[1])[:5]:
        print(f"      {r:>3}%  {p}")
    # 이유가 뭉개지면 대장 머리가 「추출기 없음」이라 적고 사용자가 로드맵에서 고칠 곳을 찾는다.
    if defeated and no_extractor == 0:
        failures.append("④ `추출기 없음` 이 0 이다 — 두 이유가 한쪽으로 뭉개졌다")
    for p in partial:
        st = next(e["state"]["partial"] for e in led["entries"] if e["path"] == p)
        if st["recovery_sites"] == 0:
            failures.append(f"④ 회복 지점 0 인데 `partial` 이다: {p}")

    # ── ⑤ 음성 대조 — 셋은 깨고 하나는 안 깬다 ─────────────────────────────
    print()
    print("── ⑤ 음성 대조 — 셋은 반드시 깨고 **하나는 반드시 안 깬다** ─────")
    tested = 0
    for m, before, broken, _, _, _ in probes:
        ok = bool(broken["recovery_sites"]) and not before["recovery_sites"]
        tested += 1
        print(f"  {'✓' if ok else '✗'} {m['label']:<24} "
              f"{'회복 지점이 생겼다' if ok else '**안 생겼다**'}   ({m['path']})")
        if not ok:
            failures.append(f"⑤ 「{m['label']}」 가 회복을 만들지 않았다 — 그 깨짐을 아무도 못 잡는다")

    # **반대 방향이 무겁다** — 성한 파일이 `partial` 로 나오면 대장의 분할이 거짓말이 되고
    # 사용자는 **읽힌 것을 안 읽혔다고 믿는다.**
    #
    # 그래서 **코퍼스 실물 997 전수**를 워킹트리에서 재고, 우리가 깨뜨린 파일 **말고는**
    # 상태가 하나도 안 움직이는지 본다. 재는 것은 값이 아니라 **차이**다 — 기준선은
    # 이 회차의 무변이 실행이지 등록된 `partial 30` 이 아니다
    # (`[f02.2.oracle].corpus_has_no_broken_kotlin`).
    kt_muts = [m for m in MUST_BREAK if m["repo"] == "pb"]
    for m, moved, states in isolation_runs(pal, a.pb.expanduser(), PB_SHA, kt_muts):
        others = [p for p in moved if p != m["path"]]
        target_moved = m["path"] in moved
        now = states.get(m["path"], "?")
        broke = "partial" in now or "grammar_defeated" in now
        ok = target_moved and broke and not others
        tested += 1
        print(f"  {'✓' if ok else '✗'} {'성한 파일 0 이동 — ' + m['label']:<24} "
              f"움직인 파일 {len(moved)}/{len(states)} · 그중 우리가 깨뜨린 것 말고 {len(others)}")
        if not target_moved:
            failures.append(f"⑤ 워킹트리에서 깨뜨렸는데 대장이 안 움직였다: {m['path']}")
        elif not broke:
            failures.append(f"⑤ 깨뜨린 파일이 `partial`/강등이 아니다: {m['path']} → {now[:60]}")
        if others:
            failures.append(f"⑤ 안 깨뜨린 파일의 상태가 움직였다: {others[:5]}")

    if tested != len(MUST_BREAK) + len(kt_muts):
        failures.append("⑤ 시험되지 않은 대조가 있다 — 「–」 는 통과가 아니다")
    print(f"  시험한 대조 {tested}/{len(MUST_BREAK) + len(kt_muts)}")

    print()
    if failures:
        print("반증 — 어긋난 것:")
        for f in failures:
            print(f"  · {f}")
        return 1
    print("다섯 다 통과")
    return 0


def isolation_runs(pal: Path, repo: Path, sha: str, mutations: list[dict]):
    """사본의 **워킹트리**를 하나씩 깨뜨리고 대장 전수를 다시 잰다.

    **원본은 읽기만 한다**(규율 4 · `f01-verify` 와 같은 형태). 워킹트리를 보는 이유는
    깨뜨린 것이 커밋에 없기 때문이고, 그래서 `--at` 을 주지 않는다.
    """
    with tempfile.TemporaryDirectory() as td:
        copy = Path(td) / "repo"
        cache = Path(td) / "cache"
        subprocess.run(
            ["git", "clone", "--quiet", "--local", "--no-hardlinks", str(repo), str(copy)],
            check=True,
        )
        subprocess.run(["git", "-C", str(copy), "checkout", "--quiet", sha], check=True)

        def ledger() -> dict[str, str]:
            r = subprocess.run(
                [str(pal), "ledger", str(copy), "--cache-dir", str(cache), "--json"],
                capture_output=True, text=True, check=False,
            )
            if r.returncode != 0:
                die(f"`pal ledger`(워킹트리) 가 실패했다: {r.stderr.strip()}")
            return {
                e["path"]: json.dumps(e["state"], sort_keys=True)
                for e in json.loads(r.stdout)["ledger"]["entries"]
            }

        base = ledger()
        for m in mutations:
            f = copy / m["path"]
            original = f.read_bytes()
            f.write_bytes(mutate(original, m))
            after = ledger()
            f.write_bytes(original)
            moved = [p for p, s in after.items() if base.get(p) != s]
            yield m, moved, after
            # **되돌린 뒤 원래대로인지 확인한다** — 아니면 다음 변이의 기준선이 오염된다.
            if ledger() != base:
                die(f"「{m['label']}」 를 되돌렸는데 대장이 기준선으로 안 돌아왔다")


def ledger_of(pal: Path, repo: Path, sha: str) -> dict:
    """**사본에서 잰다** — 원본 워킹트리를 건드리지 않는다(규율 4)."""
    with tempfile.TemporaryDirectory() as td:
        copy = Path(td) / "repo"
        subprocess.run(
            ["git", "clone", "--quiet", "--local", "--no-hardlinks", str(repo), str(copy)],
            check=True,
        )
        subprocess.run(["git", "-C", str(copy), "checkout", "--quiet", sha], check=True)
        r = subprocess.run(
            [str(pal), "ledger", str(copy), "--at", sha, "--cache-dir", str(Path(td) / "c"), "--json"],
            capture_output=True, text=True, check=False,
        )
        if r.returncode != 0:
            die(f"`pal ledger` 가 실패했다: {r.stderr.strip()}")
        return json.loads(r.stdout)["ledger"]


if __name__ == "__main__":
    sys.exit(main())
