#!/usr/bin/env python3
"""F11 대조 — **적시 제시 `touch`** (#15).

합격선은 `corpus/criteria.toml` `[f11]` 에 있고 **첫 코드 커밋보다 먼저, 별도 커밋으로**
등록됐다(`3c88077`). `registered_before_any_measurement = true` 이고, 등록 시점에 이미
알고 있던 것 넷은 `[f11].what_was_already_known` 에 있다.

    ① **재발 사례 재현** — `recurrence.toml` 의 다섯 **전수**. 이 기능의 유일한 직접 효능 측정
    ② ①을 **같은 좌표 / 다른 좌표**로 갈라 센다 (다른 좌표 모집단 ≥ 1)
    ③ **근접 후보** — 모집단 ≥ 1 · 후보 ≥ 1 · **하나를 고르지 않는다**
    ④ ★ **반대 방향** — 무관 좌표 0(모집단 ≥ 1) · 상한 10 · `stale` 우선 · `elision` ·
      빈 답의 정직성. **전부 띄우는 `touch` 가 ①에서 만점을 받는다**
    ⑤ **감시로 걸린 결박** 모집단 ≥ 1 — 역방향 질의가 실제로 도는가
    ⑥ **질의 카탈로그** — `binding.touch` 가 네 자리에 함께 선다. MCP 는 **대조 불가**
    ⑦ **지연** — 질의 시간 p95 < 500ms (표본 50). 프로세스 시간은 **기록**
    ⑧ **자기 저장소 실사용** — ⚠ 편의 표본(R-19)

★ **①의 결박 배치는 `[f11].binding_placement` 가 재기 전에 고정했다.** 여기서 좌표나
반경을 고르지 않는다 — 고르면 이 대조는 `touch` 가 아니라 **우리의 배치 솜씨**를 잰다.

사용:
    ./scripts/f11-verify.py
"""

from __future__ import annotations

import importlib.util
import json
import re
import subprocess
import sys
import tempfile
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
BIN = ROOT / "target/release/pal"

_spec = importlib.util.spec_from_file_location("f10_verify", ROOT / "scripts/f10-verify.py")
F10 = importlib.util.module_from_spec(_spec)
assert _spec.loader is not None
_spec.loader.exec_module(F10)

결과 = F10.결과
ok, fail, skip = F10.ok, F10.fail, F10.skip
run, 사본 = F10.run, F10.사본
DITTO, DITTO_PIN = F10.DITTO, F10.DITTO_PIN

# ── `[f11.pass]` 가 등록한 값들. **여기서 정하지 않는다 — 옮겨 적을 뿐이다** ────────
RECURRENCE_TOTAL = 5          # `recurrence.toml` 이 2026-08-11 에 고정한 모집단
RECURRENCE_FIRED_MIN = 5      # ★ **부분 통과선을 안 만든다** — 옛 계획 §6 이 관용을 안 적었다
CROSS_COORD_POPULATION_MIN = 1
NEAR_POPULATION_MIN = 1
NEAR_MIN = 1
TOP_N = 10                    # [F11 §3.3] 의 「상위 N(기본 10)」
UNRELATED_FIRED_MAX = 0
UNRELATED_POPULATION_MIN = 1
ELISION_POPULATION_MIN = 1
WATCH_BOUND_POPULATION_MIN = 1
P95_QUERY_MS_MAX = 500        # [F11 §6] 의 값
LATENCY_SAMPLE = 50           # `[f10.pass].false_binding_sample_size` 에서 옮긴 값
SELF_REPO_BOUND_MIN = 1

RECURRENCE = ROOT / "corpus/tasks/recurrence.toml"
CATALOG = ROOT / "surface/queries.toml"

해시 = re.compile(r"\b([0-9a-f]{7,40})\b")


def pal_raw(args: list[str], repo: Path, box: Path, at: str | None = None):
    cmd = [str(BIN), *args, "--repo", str(repo),
           "--cache-dir", str(box / "cache"), "--index", str(box / "index.redb"),
           "--intent", str(box / "intent.redb")]
    if at:
        cmd += ["--at", at]
    return run(cmd)


def pal(args: list[str], repo: Path, box: Path, at: str | None = None) -> dict:
    p = pal_raw(args, repo, box, at)
    if p.returncode != 0:
        raise SystemExit(f"실패: pal {' '.join(args)}\n{p.stderr[-800:]}")
    return json.loads(p.stdout) if "--json" in args else {}


def git(repo: Path, args: list[str]) -> str:
    p = run(["git", "-C", str(repo), *args])
    if p.returncode != 0:
        raise SystemExit(f"git {args}: {p.stderr[-400:]}")
    return p.stdout


# ═════════════════════════════════════════════════════════════════════════════
# `[f11].binding_placement` — **재기 전에 고정된 규칙. 여기서 고르지 않는다**
# ═════════════════════════════════════════════════════════════════════════════

def 근거_커밋(case: dict) -> str:
    """1. `rule_evidence` 가 적은 커밋 중 **가장 이른 것.**

    표에 시각 오름차순으로 적혀 있으므로 **첫 해시**가 가장 이르다.
    """
    m = 해시.search(case["rule_evidence"])
    if not m:
        raise SystemExit(f"사례 {case['n']} 의 `rule_evidence` 에 커밋 해시가 없다")
    return m.group(1)


def 바뀐_src(repo: Path, sha: str) -> list[tuple[int, str]]:
    """그 커밋이 바꾼 `src/**` 파일과 변경 줄 수(추가+삭제)."""
    out = git(repo, ["show", "--numstat", "--format=", sha])
    rows: list[tuple[int, str]] = []
    for line in out.splitlines():
        parts = line.split("\t")
        if len(parts) != 3:
            continue
        a, d, path = parts
        if not path.startswith("src/"):
            continue
        n = (int(a) if a.isdigit() else 0) + (int(d) if d.isdigit() else 0)
        rows.append((n, path))
    return rows


def 가장_바뀐_파일(rows: list[tuple[int, str]]) -> str | None:
    """`coord_rule` — *"변경 줄 수가 가장 많은 파일(동률이면 경로 사전순)"*."""
    if not rows:
        return None
    return sorted(rows, key=lambda r: (-r[0], r[1]))[0][1]


def 변경_줄(repo: Path, sha: str, path: str) -> set[int]:
    """그 커밋이 그 파일에서 바꾼 **새 파일 기준** 줄 번호들."""
    out = git(repo, ["show", "-U0", "--format=", sha, "--", path])
    줄: set[int] = set()
    for line in out.splitlines():
        if not line.startswith("@@"):
            continue
        m = re.search(r"\+(\d+)(?:,(\d+))?", line)
        if not m:
            continue
        start = int(m.group(1))
        n = int(m.group(2)) if m.group(2) else 1
        줄.update(range(start, start + max(n, 1)))
    return 줄


def 최상위_선언(nodes: list[dict], path: str, 줄: set[int]) -> list[str]:
    """*"그 파일에서 변경 줄을 포함하는 최상위 선언명."*

    **최상위**는 `container` 가 빈 것이다 — F03 의 정의 그대로.
    여럿이면 그대로 낸다(규칙이 하나를 고르라고 하지 않았다). 부르는 쪽이 센다.
    """
    맞는 = [
        n for n in nodes
        if n["path"] == path and not n["container"]
        and any(n["span"]["line_start"] <= l <= n["span"]["line_end"] for l in 줄)
    ]
    # **결정적 순서** — 줄 번호 오름차순.
    맞는.sort(key=lambda n: n["span"]["line_start"])
    return [n["name"] for n in 맞는]


# ═════════════════════════════════════════════════════════════════════════════
# ①② 재발 사례 재현 — **전수 다섯.** 그리고 같은 좌표 / 다른 좌표를 갈라 센다
# ═════════════════════════════════════════════════════════════════════════════

def 재발(tmp: Path) -> dict:
    print("①② 재발 사례 재현 — `[f11].binding_placement` 의 기계 규칙 그대로")
    표 = tomllib.loads(RECURRENCE.read_text(encoding="utf-8"))
    사례 = 표["case"]
    if len(사례) != RECURRENCE_TOTAL:
        fail("① 모집단", f"사례가 {len(사례)}건이다 — 등록된 모집단은 {RECURRENCE_TOTAL}")
        return {}

    repo, box = 사본(tmp, "recurrence", DITTO, DITTO_PIN)
    판정: list[dict] = []
    감시로_걸린 = 0

    for case in 사례:
        n = case["n"]
        e = 근거_커밋(case)
        규칙 = case["rule_that_should_have_fired"]

        # ── 2. 결박 좌표 = `coord_rule` 을 e 에 적용 ──────────────────────────
        rows = 바뀐_src(repo, e)
        파일 = 가장_바뀐_파일(rows)
        if 파일 is None:
            판정.append({"n": n, "결과": "대조 불가", "왜": "(가) e 가 `src/**` 를 안 바꿨다",
                         "e": e})
            continue

        nodes = pal(["query", "graph.dump", "--json"], repo, box, at=e)["answer"]["nodes"]
        후보 = 최상위_선언(nodes, 파일, 변경_줄(repo, e, 파일))
        if not 후보:
            판정.append({"n": n, "결과": "대조 불가",
                         "왜": f"(가) {파일} 의 변경 줄이 최상위 선언 밖이다", "e": e})
            continue
        결박_좌표 = 후보[0]

        # ── 4. 반경 = e 가 바꾼 `src/**` 파일 전부 ──────────────────────────
        반경 = "files:" + ",".join(sorted(p for _, p in rows))

        # ── 3·5. 본문은 문장 그대로 · 시점은 e ────────────────────────────────
        p = pal_raw(["bind", 결박_좌표, "--note", 규칙, "--radius", 반경], repo, box, at=e)
        if p.returncode != 0:
            판정.append({"n": n, "결과": "대조 불가",
                         "왜": f"(나) `{결박_좌표}` 를 유일하게 못 걸었다: "
                               f"{p.stderr.strip().splitlines()[0] if p.stderr.strip() else ''}",
                         "e": e, "결박_좌표": 결박_좌표})
            continue

        # ── 6. 재발 커밋의 **부모**에서 만진다 ────────────────────────────────
        부모 = git(repo, ["rev-parse", f"{case['commit']}^"]).strip()
        env = pal(["touch", case["symbol"], "--json"], repo, box, at=부모)
        answer = env["answer"]
        if answer["outcome"] != "found":
            판정.append({"n": n, "결과": "대조 불가",
                         "왜": f"(라) `{case['symbol']}` 이 `{case['commit']}^` 의 2층에서 "
                               f"{answer['outcome']} 이다",
                         "e": e, "결박_좌표": 결박_좌표})
            continue

        걸린 = answer["bindings"]["present"]
        지켜보는 = answer["watching"]["present"]
        감시로_걸린 += len(지켜보는)
        # ── 7. 떴다 = 본문이 규칙 문장과 **같다** ─────────────────────────────
        뜬_것 = [b for b in 걸린 + 지켜보는 if b["note"] == 규칙]
        판정.append({
            "n": n, "결과": "떴다" if 뜬_것 else "안 떴다",
            "e": e, "결박_좌표": 결박_좌표, "재발_좌표": case["symbol"],
            "같은_좌표": 결박_좌표 == case["symbol"],
            "어디": ("걸린 것" if any(b["at"]["at"] == "here" for b in 뜬_것)
                     else "지켜보는 것" if 뜬_것 else "—"),
            "반경_파일": len(rows),
        })

    떴다 = [p for p in 판정 if p["결과"] == "떴다"]
    안_떴다 = [p for p in 판정 if p["결과"] == "안 떴다"]
    대조불가 = [p for p in 판정 if p["결과"] == "대조 불가"]

    for p in 판정:
        표시 = {"떴다": "  ✓", "안 떴다": "  ✗", "대조 불가": "  –"}[p["결과"]]
        print(f"{표시} 사례 {p['n']}  {p['결과']:<8} "
              f"결박 {p.get('결박_좌표', '—')} → 만짐 {p.get('재발_좌표', '—')}"
              f"{'  · ' + p['어디'] if p.get('어디') and p['어디'] != '—' else ''}"
              f"{'  · ' + p['왜'] if p.get('왜') else ''}")

    if 대조불가:
        skip("① 재발 사례 재현",
             f"떴다 {len(떴다)} · 안 떴다 {len(안_떴다)} · **대조 불가 {len(대조불가)}** "
             f"/ {RECURRENCE_TOTAL} — **전수 통과로 적을 수 없다**([ADR-0002])")
    elif len(떴다) >= RECURRENCE_FIRED_MIN:
        ok("① 재발 사례 재현", f"{len(떴다)}/{RECURRENCE_TOTAL} 이 떴다 (하한 {RECURRENCE_FIRED_MIN})")
    else:
        fail("① 재발 사례 재현",
             f"{len(떴다)}/{RECURRENCE_TOTAL} 만 떴다 (하한 {RECURRENCE_FIRED_MIN}) — "
             f"**반증이다. 실패가 아니다**")

    같은 = [p for p in 떴다 + 안_떴다 if p["같은_좌표"]]
    다른 = [p for p in 떴다 + 안_떴다 if not p["같은_좌표"]]
    if len(다른) >= CROSS_COORD_POPULATION_MIN:
        ok("② 같은 좌표 / 다른 좌표",
           f"같은 좌표 {len(같은)} (뜬 것 {sum(1 for p in 같은 if p['결과'] == '떴다')}) · "
           f"**다른 좌표 {len(다른)}** (뜬 것 {sum(1 for p in 다른 if p['결과'] == '떴다')}) "
           f"— 모집단 하한 {CROSS_COORD_POPULATION_MIN}")
    else:
        fail("② 다른 좌표 모집단",
             f"{len(다른)}건이다 (하한 {CROSS_COORD_POPULATION_MIN}) — "
             f"**①이 「`touch(좌표)` 로 충분하다」 위에서만 섰다**")

    if 감시로_걸린 >= WATCH_BOUND_POPULATION_MIN:
        ok("⑤ 감시로 걸린 결박", f"{감시로_걸린}건이 「지켜보는 것」으로 실렸다 "
                                 f"(하한 {WATCH_BOUND_POPULATION_MIN}) — **역방향 질의가 돈다**")
    else:
        fail("⑤ 감시로 걸린 결박",
             f"{감시로_걸린}건이다 (하한 {WATCH_BOUND_POPULATION_MIN}) — "
             f"**`WATCH` 경로가 안 돈다**")

    # **판정을 사례마다 한 줄로 남긴다** — 이 저장소의 관례다. 남기지 않으면
    # 게이트에 *"2/5 가 떴다"* 만 남고 **무엇이 왜 안 떴는지**가 사라진다.
    표본_파일 = ROOT / "corpus/tasks/f11-recurrence-judgment.tsv"
    줄 = ["사례\t결과\t근거커밋\t결박좌표\t반경파일수\t재발좌표\t같은좌표\t어디\t왜"]
    for p in 판정:
        줄.append("\t".join(str(x) for x in [
            p["n"], p["결과"], p.get("e", ""), p.get("결박_좌표", ""),
            p.get("반경_파일", ""), p.get("재발_좌표", ""),
            "예" if p.get("같은_좌표") else ("아니오" if "같은_좌표" in p else ""),
            p.get("어디", ""), p.get("왜", ""),
        ]))
    표본_파일.write_text("\n".join(줄) + "\n", encoding="utf-8")
    ok("① 판정 기록", f"{표본_파일.relative_to(ROOT)} — 사례마다 한 줄")

    return {"판정": 판정, "repo": repo, "box": box, "감시로_걸린": 감시로_걸린}


# ═════════════════════════════════════════════════════════════════════════════
# ③ 근접 후보 — **하나를 고르지 않는다**
# ═════════════════════════════════════════════════════════════════════════════

def 스네이크(name: str) -> str:
    """camelCase → snake_case. **입력을 우리가 고르지 않는다** — 기계 변환이다."""
    return re.sub(r"(?<!^)(?=[A-Z])", "_", name).lower()


def 근접(repo: Path, box: Path) -> None:
    print("③ 근접 후보 — 오타 입력")
    nodes = pal(["query", "graph.dump", "--json"], repo, box, at=DITTO_PIN)["answer"]["nodes"]
    이름_수: dict[str, int] = {}
    for n in nodes:
        이름_수[n["name"]] = 이름_수.get(n["name"], 0) + 1
    # **유일하고 camelCase 인 이름만** — 변환이 원래 이름과 달라야 오타가 된다.
    후보 = sorted(x for x, c in 이름_수.items() if c == 1 and 스네이크(x) != x)
    if len(후보) < NEAR_POPULATION_MIN:
        skip("③ 근접 후보", f"오타를 만들 이름이 {len(후보)}개다 — **모집단 0 이라 대조 불가**")
        return

    # `[f09.4].sample_selection` 규칙 4 — 사전순 정렬 후 균등 간격. 다섯을 본다.
    간격 = max(1, len(후보) // 5)
    표본 = 후보[::간격][:5]
    맞춘, 고른 = 0, 0
    for 이름 in 표본:
        env = pal(["touch", 스네이크(이름), "--json"], repo, box, at=DITTO_PIN)
        a = env["answer"]
        if a["outcome"] != "unknown":
            고른 += 1
            continue
        near = [x["name"] for x in a["near"]]
        if 이름 in near:
            맞춘 += 1
    if 고른:
        fail("③ 하나를 고르지 않는다", f"표본 {len(표본)} 중 {고른}건이 `unknown` 이 아니다 — "
                                       f"**오타를 답으로 고쳤다**")
    else:
        ok("③ 하나를 고르지 않는다", f"표본 {len(표본)} 전부 `unknown` 이다 — 후보만 낸다")
    if 맞춘 >= NEAR_MIN:
        ok("③ 근접 후보", f"표본 {len(표본)} 중 {맞춘}건에서 원래 이름이 후보에 있다 "
                          f"(하한 {NEAR_MIN} · 모집단 {len(후보)})")
    else:
        fail("③ 근접 후보", f"{맞춘}건이다 (하한 {NEAR_MIN})")

    # ★ **가까운 것이 없는 것도 답이다.**
    env = pal(["touch", "zzzz_없는이름_zzzz", "--json"], repo, box, at=DITTO_PIN)
    a = env["answer"]
    if a["outcome"] == "unknown" and a["near"] == []:
        ok("③ 빈 후보도 답이다", "`unknown` 이고 `near` 가 빈 목록이다 — **없다는 뜻이 아니다**")
    else:
        fail("③ 빈 후보도 답이다", f"{a['outcome']} · near {len(a.get('near', []))}")


# ═════════════════════════════════════════════════════════════════════════════
# ④ ★ 반대 방향 — **전부 띄우는 `touch` 를 막는다**
# ═════════════════════════════════════════════════════════════════════════════

def 반대_방향(repo: Path, box: Path, 판정: list[dict]) -> None:
    print("④ 반대 방향 — 무관 좌표 · 상한 · `stale` 우선 · 빈 답")

    # ── 가. 무관 좌표에서 안 뜬다 ────────────────────────────────────────────
    #
    # **모집단은 「걸린 규칙 문장 전부」다.** 하나라도 무관 좌표에서 뜨면 조회 거리가
    # 열린 것이다. 좌표는 **결박이 하나도 없는 파일**에서 균등 간격으로 뽑는다.
    env = pal(["query", "binding.status", "--json"], repo, box, at=DITTO_PIN)
    걸린_문장 = {b["note"] for b in env["answer"]["bindings"]}
    if not 걸린_문장:
        skip("④ 무관 좌표", "결박이 0 건이다 — **모집단 0 이라 대조 불가**")
        return
    nodes = pal(["query", "graph.dump", "--json"], repo, box, at=DITTO_PIN)["answer"]["nodes"]
    # 결박 대상이 사는 파일과 **감시 원소가 사는 파일**을 뺀다 — 남은 것이 무관 좌표다.
    대상_id = {b["target"] for b in env["answer"]["bindings"]}
    감시_파일 = {n["path"] for n in nodes if n["id"] in 대상_id}
    이름_수: dict[str, int] = {}
    for n in nodes:
        이름_수[n["name"]] = 이름_수.get(n["name"], 0) + 1
    무관 = sorted(
        (n for n in nodes if n["path"] not in 감시_파일 and 이름_수[n["name"]] == 1),
        key=lambda n: n["id"],
    )
    if len(무관) < UNRELATED_POPULATION_MIN:
        skip("④ 무관 좌표", "감시 집합 밖의 유일한 이름이 없다 — **모집단 0 이라 대조 불가**")
    else:
        간격 = max(1, len(무관) // 20)
        표본 = [n["name"] for n in 무관[::간격]][:20]
        샌_것 = []
        for 이름 in 표본:
            a = pal(["touch", 이름, "--json"], repo, box, at=DITTO_PIN)["answer"]
            if a["outcome"] != "found":
                continue
            for b in a["bindings"]["present"] + a["watching"]["present"]:
                if b["note"] in 걸린_문장:
                    샌_것.append((이름, b["binding"]))
        if len(샌_것) <= UNRELATED_FIRED_MAX:
            ok("④ 무관 좌표", f"표본 {len(표본)} (모집단 {len(무관)}) 에서 샌 것 "
                              f"{len(샌_것)} (상한 {UNRELATED_FIRED_MAX})")
        else:
            fail("④ 무관 좌표", f"샌 것 {len(샌_것)}건 — **조회 거리가 열렸다**: {샌_것[:3]}")

    # ── 나. 상한과 절단 ─────────────────────────────────────────────────────
    #
    # 실 코퍼스에서 한 좌표에 상한을 넘는 결박이 없으면 **모집단 0** 이다. 그때는
    # 손잡이(`--binding-max`)로 상한을 낮춰 **기계가 실재함**을 보이고, ⚠ 그것을
    # 실 코퍼스의 절단으로 적지 않는다.
    좌표별: dict[str, int] = {}
    for b in env["answer"]["bindings"]:
        좌표별[b["target"]] = 좌표별.get(b["target"], 0) + 1
    가장_많은 = max(좌표별.values(), default=0)
    id_이름 = {n["id"]: n["name"] for n in nodes}
    붐비는 = max(좌표별, key=lambda k: 좌표별[k]) if 좌표별 else None
    이름 = id_이름.get(붐비는, "")
    if 가장_많은 > TOP_N and 이름:
        a = pal(["touch", 이름, "--json"], repo, box, at=DITTO_PIN)
        실린 = len(a["answer"]["bindings"]["present"])
        잘린 = sum(t["count"] for t in a["elision"]["truncated"]
                   if t["reason"] == "binding_max_exceeded")
        if 실린 <= TOP_N and 잘린 >= ELISION_POPULATION_MIN:
            ok("④ 상한과 절단", f"`{이름}` 에 결박 {가장_많은} · 실린 것 {실린} · 잘린 것 {잘린}")
        else:
            fail("④ 상한과 절단", f"실린 것 {실린} (상한 {TOP_N}) · 잘린 것 {잘린}")
    elif 이름:
        a = pal(["touch", 이름, "--binding-max", "1", "--json"], repo, box, at=DITTO_PIN)
        실린목록 = a["answer"]["bindings"]["present"]
        낡은 = sum(1 for b in 실린목록 if b["status"]["code"].get("freshness") != "live")
        잘린 = sum(t["count"] for t in a["elision"]["truncated"]
                   if t["reason"] == "binding_max_exceeded")
        # ★ **낡은 것은 상한을 안 탄다.** 상한을 1 로 낮춰도 낡은 것이 전부 실리는 것이
        # [F11 §3.3] 이 요구한 그것이고, 그때 「잘린 것 0」은 고장이 아니라 규율이다.
        skip("④ 상한과 절단",
             f"한 좌표에 걸린 것이 최대 {가장_많은}건이라 상한 {TOP_N} 에 안 닿는다 — "
             f"**실 코퍼스에서는 모집단 0 이라 대조 불가.** 손잡이를 1 로 낮추면 "
             f"`{이름}` 에 실린 것 {len(실린목록)}(그중 낡은 것 {낡은}) · 잘린 것 {잘린} — "
             f"⚠ **낡은 것이 상한을 안 타는 것이지 상한이 안 도는 것이 아니다**"
             f"{' (`cargo test --test touch_recall` 이 그 기계를 전수로 잰다)' if 낡은 else ''}")
    else:
        skip("④ 상한과 절단", "결박된 좌표가 2층에 없다 — **대조 불가**")

    # ── 다. 빈 답의 정직성 ───────────────────────────────────────────────────
    빈_좌표 = next((n["name"] for n in 무관 if 이름_수[n["name"]] == 1), None)
    if 빈_좌표:
        a = pal(["touch", 빈_좌표, "--json"], repo, box, at=DITTO_PIN)["answer"]
        비었나 = a["bindings"] == {"present": []} and a["watching"] == {"present": []}
        미구축 = "not_built" in json.dumps(a["bindings"]) + json.dumps(a["watching"])
        if 비었나 and not 미구축:
            ok("④ 빈 답의 정직성",
               f"`{빈_좌표}` — `present: []` 이고 `not_built` 가 아니다. "
               f"**능력이 있고 값이 없는 것**이다")
        else:
            fail("④ 빈 답의 정직성", f"{json.dumps(a['bindings'])[:120]}")
    else:
        skip("④ 빈 답의 정직성", "결박 0 인 좌표를 못 골랐다 — **대조 불가**")


# ═════════════════════════════════════════════════════════════════════════════
# ⑥ 질의 카탈로그 — 그리고 **MCP 는 대조 불가**
# ═════════════════════════════════════════════════════════════════════════════

def 카탈로그(repo: Path, box: Path) -> None:
    print("⑥ 질의 카탈로그")
    목록 = pal(["query", "--list", "--json"], repo, box)
    있나 = any(q["name"] == "binding.touch" for q in 목록["built"])
    파일에 = "binding.touch" in CATALOG.read_text(encoding="utf-8")
    if 있나 and 파일에:
        ok("⑥ 카탈로그 정합", "`binding.touch` 가 코드와 `surface/queries.toml` 둘 다에 있다 "
                              "— **양방향 대조는 `cargo xtask check` 가 진다**")
    else:
        fail("⑥ 카탈로그 정합", f"코드 {있나} · 파일 {파일에}")

    a = pal(["query", "binding.touch", "resolveClaimBranch", "--json"], repo, box, at=DITTO_PIN)
    if a["log"]["status"] == "recorded":
        ok("⑥ 질의 로그", "`binding.touch` 가 로그를 남긴다 — "
                          "**옛 판은 `surface_does_not_log` 였고 F17 이 미조회로 셌다**")
    else:
        fail("⑥ 질의 로그", json.dumps(a["log"]))

    if (ROOT / "crates/pal-mcp").exists():
        fail("⑥ MCP", "`pal-mcp` 가 되살아났다 — ADR-0025 의 되돌리기 조건이 걸렸다")
    else:
        skip("⑥ MCP 경로", "`crates/pal-mcp` 는 **폐기됐다**(ADR-0025 · 2026-08-18) — "
                           "「아직 안 만들어서 대조 불가」가 아니라 **「요구가 철회되어 "
                           "영영 안 잰다」**다. 세 경로 중 CLI 하나가 전부이고, 그것이 "
                           "설계다. 0 을 통과로 세지 않는 것은 그대로다([ADR-0002])")


# ═════════════════════════════════════════════════════════════════════════════
# ⑦ 지연 — **두 시계를 둘 다 낸다. 합격선은 질의 시간에만 걸린다**
# ═════════════════════════════════════════════════════════════════════════════

def 지연(repo: Path, box: Path) -> None:
    print("⑦ 지연 — 질의 시간(합격선) · 프로세스 시간(기록)")
    nodes = pal(["query", "graph.dump", "--json"], repo, box, at=DITTO_PIN)["answer"]["nodes"]
    이름_수: dict[str, int] = {}
    for n in nodes:
        이름_수[n["name"]] = 이름_수.get(n["name"], 0) + 1
    # `[f09.4].sample_selection` 규칙 4 — `symbol_id` 사전순 + 균등 간격.
    유일 = sorted((n for n in nodes if 이름_수[n["name"]] == 1), key=lambda n: n["id"])
    if len(유일) < LATENCY_SAMPLE:
        skip("⑦ 지연", f"유일한 이름이 {len(유일)}개다 (표본 {LATENCY_SAMPLE}) — **대조 불가**")
        return
    간격 = max(1, len(유일) // LATENCY_SAMPLE)
    표본 = [n["name"] for n in 유일[::간격]][:LATENCY_SAMPLE]

    질의_us: list[int] = []
    프로세스_us: list[int] = []
    결박_실린 = 0
    for 이름 in 표본:
        p = pal_raw(["touch", 이름, "--timing", "--json"], repo, box, at=DITTO_PIN)
        if p.returncode != 0:
            continue
        m = re.search(r"elapsed_micros=(\d+) process_micros=(\d+)", p.stderr)
        if not m:
            continue
        질의_us.append(int(m.group(1)))
        프로세스_us.append(int(m.group(2)))
        a = json.loads(p.stdout)["answer"]
        if a["outcome"] == "found" and (a["bindings"]["present"] or a["watching"]["present"]):
            결박_실린 += 1

    if len(질의_us) < LATENCY_SAMPLE:
        skip("⑦ 지연", f"잰 것이 {len(질의_us)}건이다 (표본 {LATENCY_SAMPLE}) — **대조 불가**")
        return

    def p95(xs: list[int]) -> float:
        s = sorted(xs)
        return s[min(len(s) - 1, int(len(s) * 0.95))] / 1000.0

    q, w = p95(질의_us), p95(프로세스_us)
    # **결박이 실린 좌표의 수를 함께 낸다** — 0 이면 조립할 것이 없어서 빠른 것이다.
    기록 = (f"표본 {len(질의_us)} · 결박이 실린 좌표 {결박_실린} · "
            f"프로세스 p95 **{w:.0f}ms**(기록 · 합격선 아님)")
    if q < P95_QUERY_MS_MAX:
        ok("⑦ 질의 시간 p95", f"**{q:.1f}ms** < {P95_QUERY_MS_MAX}ms · {기록}")
    else:
        fail("⑦ 질의 시간 p95", f"**{q:.1f}ms** ≥ {P95_QUERY_MS_MAX}ms · {기록}")


# ═════════════════════════════════════════════════════════════════════════════
# ⑧ 자기 저장소 — ⚠ **편의 표본이다** (R-19)
# ═════════════════════════════════════════════════════════════════════════════

def 자기_저장소(tmp: Path) -> None:
    print("⑧ 자기 저장소 실사용 — ⚠ 편의 표본(R-19)")
    box = tmp / "self"
    box.mkdir(parents=True)
    nodes = pal(["query", "graph.dump", "--json"], ROOT, box)["answer"]["nodes"]
    if not nodes:
        skip("⑧ 자기 저장소", "결박 가능한 좌표가 0 개다 — **대조 불가**")
        return
    이름_수: dict[str, int] = {}
    for n in nodes:
        이름_수[n["name"]] = 이름_수.get(n["name"], 0) + 1
    유일 = sorted((n for n in nodes if 이름_수[n["name"]] == 1), key=lambda n: n["id"])
    if not 유일:
        skip("⑧ 자기 저장소", "유일한 이름이 없다 — **대조 불가**")
        return
    대상 = 유일[0]["name"]
    p = pal_raw(["bind", 대상, "--note", "F11 자기적용 — 이 좌표를 실제로 만져 본다",
                 "--radius", "symbol"], ROOT, box)
    if p.returncode != 0:
        fail("⑧ 자기 저장소", f"`{대상}` 을 못 걸었다: {p.stderr[-200:]}")
        return
    a = pal(["touch", 대상, "--json"], ROOT, box)["answer"]
    걸린 = len(a["bindings"]["present"]) if a["outcome"] == "found" else 0
    파일 = {n["path"] for n in nodes}
    if 걸린 >= SELF_REPO_BOUND_MIN:
        ok("⑧ 자기 저장소", f"`{대상}` 에 결박 {걸린} (하한 {SELF_REPO_BOUND_MIN}) — "
                            f"⚠ **결박 가능한 좌표가 {len(nodes)}개뿐이고 파일 {len(파일)}개다. "
                            f"코어가 Rust 라 추출기가 못 읽는다** — 편의 표본이자 능력 부재")
    else:
        fail("⑧ 자기 저장소", f"결박 {걸린} (하한 {SELF_REPO_BOUND_MIN})")


def main() -> int:
    if not BIN.exists():
        raise SystemExit(f"바이너리가 없다: {BIN} — `cargo build --release`")
    if not DITTO.exists():
        raise SystemExit(f"코퍼스가 없다: {DITTO}")

    with tempfile.TemporaryDirectory(prefix="pal-f11-") as td:
        tmp = Path(td)
        got = 재발(tmp)
        if got:
            근접(got["repo"], got["box"])
            반대_방향(got["repo"], got["box"], got["판정"])
            카탈로그(got["repo"], got["box"])
            지연(got["repo"], got["box"])
        자기_저장소(tmp)

    print()
    for 표시, 이름, 값 in 결과:
        print(f"  {표시:<5} {이름}  — {값}")

    print()
    어긋남 = [r for r in 결과 if r[0] == "FAIL"]
    대조불가 = [r for r in 결과 if r[0] == "–"]
    for 표시, 이름, 값 in 어긋남:
        print(f"  FAIL  {이름}: {값}")
    if 대조불가:
        print(f"대조 불가 {len(대조불가)}건 — **통과로 세지 않는다**")
        for _, 이름, 값 in 대조불가:
            print(f"   – {이름}: {값}")
    print(f"어긋남 {len(어긋남)}")
    return 1 if 어긋남 else 0


if __name__ == "__main__":
    sys.exit(main())
