#!/usr/bin/env python3
"""F05(#8) 대조 — **바깥 오라클이 우리 인덱스를 채점한다.**

합격선 정본은 `corpus/criteria.toml` `[f05]` 이고 판정은 `docs/gates/F05.md` 다.

이 스크립트가 재는 것 둘. 나머지는 `cargo test` 가 상시로 재고, 벤치 넷은
`cargo test -p pal-query --release -- --ignored` 가 잰다:

    ③  ★ **SQLite 재귀 CTE 와 답이 일치하는가** — `[f05.3.pass]`
    ⑨  **F04 가 넘긴 「옳은 선」** — `[f05].bench_ratio_correction`

## ③ 이 이 기능에서 가장 중요하다 (R-18)

F05 문서 §8 의 검증 다섯 중 **넷이 우리 구현끼리의 비교**다. 재구축 등가성도 예산
절단도 봉투 골든도 벤치도 전부 우리 산출 대 우리 산출이다. **바깥에 있는 것은
SQLite 하나뿐이고**, 그것이 없으면 이 게이트는 자기가 만든 인덱스를 자기가 채점한다.

**`rusqlite` 크레이트가 아니라 Python 표준 라이브러리의 `sqlite3` 를 쓴다.** 등록
(`[f05].sqlite_cte_oracle`)은 *"시스템 `sqlite3`"* 라고 적었고, Python 의 `sqlite3` 는
**같은 엔진을 표준 라이브러리로 부르는 것**이다 — 우리 코드가 아니라는 성질이 그대로이고
가용성만 낫다. 없으면 **대조 불가**로 적는다. 조용히 건너뛰지 않는다.

### ★ CTE 에도 반대 방향을 건다 — 셋

    ㉠  **엣지 방향을 뒤집으면 답이 갈려야 한다.** 안 갈리면 그래프가 대칭이거나
        CTE 가 아무것도 안 재고 있다
    ㉡  **깊이 상한을 올리면 답이 늘어야 한다.** 안 늘면 깊이가 안 걸린 것이다
    ㉢  **엣지 수 하한.** 엣지가 0 이면 두 답이 공짜로 같다

G50 이 F02-2 의 실측을 그렇게 잡았다 — **남의 측정을 근거로 삼을 때 그 측정에도
반대 방향을 걸어라.**

## ⑨ 는 F04 에서 어긋난 값을 옳은 선으로 다시 잰다

F04 의 *"증분이 콜드보다 10배"* 가 `ditto` 에서 **6.5배**로 어긋났다. 원인은 캐시가
아니라 **비율이 코퍼스 구성을 못 나누는 것**이다 — `ditto` 는 2,451 중 491 만 추출
대상이고, 나머지의 고정 비용이 분모와 분자에 똑같이 실려 비율을 1 로 끌어당긴다.

    합격선: (콜드 − 고정비) ÷ (증분 − 고정비) ≥ 10
    고정비: **추출 대상이 0 인 회차**의 벽시계 — 제외 규칙으로 실측해 뜬다

**고정비가 콜드의 50% 를 넘으면 대조 불가로 적는다** — 그 코퍼스에서는 추출이
지배적이지 않다는 뜻이고, 그 사실 자체가 산출이다.

⚠ **저장소에 `.palimpsest/manifest.toml` 을 잠시 쓴다.** 있던 것은 떠 두고 되돌린다.

사용:
    ./scripts/f05-verify.py
    ./scripts/f05-verify.py --skip-bench     # ⑨ 를 건너뛴다 (회차 넷을 아낀다)

종료 코드:
    0  둘 다 통과
    1  어긋난 것이 있다 · 또는 대조가 성립하지 않았다
"""

from __future__ import annotations

import argparse
import json
import platform
import shutil
import subprocess
import sys
import tempfile
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
BIN = ROOT / "target/release/pal"

DITTO = (Path.home() / "dev/projects/ditto", "aded7ce7f88f", "ditto")
PORTAL = (Path.home() / "dev/projects/boxwood/portal-backend", "a29cad0bf6a8", "portal-backend")

# **하한이다.** 이보다 적으면 시험되지 않은 것이고, 시험되지 않은 대조는 `–` 가 아니라
# 실패다(`2e2eb3f`).
최소_엣지 = 50
최소_출발점 = 10
# 깊이 상한 — 자리표시 3 과 그 두 배. ㉡ 이 둘을 비교한다.
깊이 = 3
깊은_깊이 = 6

실패: list[str] = []
대조불가: list[str] = []
기록: list[str] = []


def 적음(line: str = "") -> None:
    print(line)


def ok(line: str) -> None:
    적음(f"  ok    {line}")


def 어긋남(line: str) -> None:
    적음(f"  FAIL  {line}")
    실패.append(line)


def 기권(line: str) -> None:
    적음(f"  –     {line}")
    대조불가.append(line)


def pal(args: list[str]) -> str:
    p = subprocess.run([str(BIN), *args], capture_output=True, text=True, check=False)
    if p.returncode != 0:
        raise SystemExit(f"pal {args} 실패:\n{p.stderr[-1200:]}")
    return p.stdout


def 방(tag: str) -> Path:
    """**회차마다 새 방.** 돌려 쓰면 한 회차가 다른 회차의 캐시를 본다."""
    return Path(tempfile.mkdtemp(prefix=f"f05-{tag}-"))


# ═════════════════════════════════════════════════════════════════════════════
# ③ SQLite 재귀 CTE 대조
# ═════════════════════════════════════════════════════════════════════════════


def 도달_CTE(conn, start: str, depth: int, reversed_edges: bool = False) -> set[str]:
    """재귀 CTE 로 `start` 에서 `depth` 홉 안에 닿는 것 — **출발점을 포함한다.**

    우리 `traverse` 가 출발점을 답에 담으므로 여기서도 담아야 같은 것을 센다.
    """
    src, dst = ("dst", "src") if reversed_edges else ("src", "dst")
    rows = conn.execute(
        f"""
        WITH RECURSIVE reach(id, hops) AS (
            SELECT ?, 0
            UNION
            SELECT e.{dst}, r.hops + 1
              FROM edge e JOIN reach r ON e.{src} = r.id
             WHERE r.hops < ?
        )
        SELECT DISTINCT id FROM reach
        """,
        (start, depth),
    ).fetchall()
    return {r[0] for r in rows}


def 대조_CTE(repo: Path, at: str, tag: str) -> None:
    적음(f"── ③ SQLite 재귀 CTE 대조 — {tag} ──────────────────────────────")
    try:
        import sqlite3
    except ImportError:  # pragma: no cover
        기권("③ Python 에 `sqlite3` 가 없다 — 바깥 오라클이 없으므로 대조 불가")
        return

    room = 방(tag)
    dump = json.loads(
        pal([
            "query", "graph.dump", "--json",
            "--repo", str(repo), "--at", at,
            "--cache-dir", str(room / "cache"), "--index", str(room / "index.redb"),
        ])
    )
    nodes = dump["answer"]["nodes"]
    edges = dump["answer"]["edges"]

    # ── ㉢ 하한 — 엣지가 0 이면 두 답이 공짜로 같다 ─────────────────────────
    #
    # **0 은 「없음」이 아니라 「안 만듦」일 수 있다.** Kotlin 추출기는 스코프 체인을
    # 안 만들고, 그러면 그 코퍼스의 파일 내 엣지는 **모집단이 0** 이다 —
    # [ADR-0002] 그대로 그것을 통과로도 어긋남으로도 세지 않는다(`[f05].self_judged` ⑤).
    if len(edges) == 0:
        만듦 = sum(1 for n in nodes if n)  # 노드는 섰다 — 엣지만 없다
        기권(
            f"③ {tag} — 엣지가 0 이다. 노드는 {만듦}개 섰으므로 스티칭은 돌았고, "
            "이 코퍼스의 추출기가 스코프 체인을 안 만든다 — **모집단이 0 이라 대조 불가**"
        )
        shutil.rmtree(room, ignore_errors=True)
        return
    if len(edges) < 최소_엣지:
        어긋남(f"③ {tag} 의 엣지가 {len(edges)}개다 (하한 {최소_엣지}) — 이 대조는 성립하지 않는다")
        shutil.rmtree(room, ignore_errors=True)
        return

    conn = sqlite3.connect(":memory:")
    conn.execute("CREATE TABLE node(id TEXT PRIMARY KEY, name TEXT)")
    conn.execute("CREATE TABLE edge(src TEXT, dst TEXT)")
    conn.executemany("INSERT INTO node VALUES (?, ?)", [(n["id"], n["name"]) for n in nodes])
    conn.executemany("INSERT INTO edge VALUES (?, ?)", [(e["from"], e["to"]) for e in edges])
    conn.execute("CREATE INDEX i_src ON edge(src)")
    conn.execute("CREATE INDEX i_dst ON edge(dst)")

    # 출발점은 **이름이 유일하게 해소되고 나가는 엣지가 있는** 심볼이다.
    # 이름이 여럿이면 `pal query` 가 `ambiguous` 로 답하고, 그것은 이 대조의 대상이 아니다.
    유일한_이름 = {
        r[0] for r in conn.execute("SELECT name FROM node GROUP BY name HAVING COUNT(*) = 1")
    }
    나가는 = {r[0] for r in conn.execute("SELECT DISTINCT src FROM edge")}
    후보 = [n for n in nodes if n["id"] in 나가는 and n["name"] in 유일한_이름]
    후보.sort(key=lambda n: n["id"])
    출발점 = 후보[:25]

    if len(출발점) < 최소_출발점:
        어긋남(f"③ {tag} 의 출발점이 {len(출발점)}개다 (하한 {최소_출발점}) — 시험되지 않았다")
        shutil.rmtree(room, ignore_errors=True)
        return

    갈림: list[str] = []
    뒤집어_갈린_수 = 0
    얕은_합 = 0
    깊은_합 = 0
    절단_실린_수 = 0

    # ── ★ 반대 방향 ㉠㉡ 은 **CTE 만으로** 잰다 ────────────────────────────
    #
    # ⚠ 이 자리가 한 번 꺼졌다. 처음 판은 ㉠㉡ 을 우리 답과의 비교 **뒤**에 뒀고,
    # 비교를 건너뛴 출발점에서 그 둘도 함께 건너뛰어졌다. 그래서 *"깊이를 올려도 답이
    # 안 늘었다"* 가 나왔다 — **깊이 너머로 가는 출발점만 정확히 빠졌기 때문이다.**
    # 통제는 통제하려는 것과 같은 흐름에 두면 안 된다.
    for n in 출발점:
        theirs = 도달_CTE(conn, n["id"], 깊이)
        if 도달_CTE(conn, n["id"], 깊이, reversed_edges=True) != theirs:
            뒤집어_갈린_수 += 1
        얕은_합 += len(theirs)
        깊은_합 += len(도달_CTE(conn, n["id"], 깊은_깊이))

    for n in 출발점:
        우리 = json.loads(
            pal([
                "query", "symbol.reaches", n["name"], "--json",
                "--repo", str(repo), "--at", at,
                "--cache-dir", str(room / "cache"), "--index", str(room / "index.redb"),
                "--depth-max", str(깊이),
            ])
        )
        if 우리["answer"]["outcome"] != "reached":
            갈림.append(f"{n['name']}: 우리 답이 `{우리['answer']['outcome']}` 다")
            continue

        잘린것 = {t["reason"] for t in 우리["elision"]["truncated"]}
        # **깊이 절단은 비교를 깨지 않는다** — CTE 에도 같은 깊이 상한을 걸었다.
        # 오히려 이것이 stack §2.3 의 논증이 실물에서 보이는 자리다: 두 답이 같은데
        # **우리 답만 「무엇을 왜 안 봤는지」를 싣는다.** CTE 는 그 사실을 낼 수 없다.
        if 잘린것 - {"depth_exceeded"}:
            # 노드 상한 등은 CTE 에 없다 — 그때는 비교가 성립하지 않는다.
            갈림.append(f"{n['name']}: CTE 에 없는 절단이 일어났다 {sorted(잘린것)}")
            continue
        if 잘린것:
            절단_실린_수 += 1

        ours = {s["id"] for s in 우리["answer"]["symbols"]}
        theirs = 도달_CTE(conn, n["id"], 깊이)
        if ours != theirs:
            갈림.append(
                f"{n['name']}: 우리 {len(ours)} · CTE {len(theirs)} · "
                f"우리만 {len(ours - theirs)} · CTE 만 {len(theirs - ours)}"
            )

    if 갈림:
        어긋남(f"③ {tag} — 자체 인덱스와 CTE 가 갈렸다 ({len(갈림)}건): {갈림[:5]}")
    else:
        ok(f"③ {tag} — 출발점 {len(출발점)}개 · 엣지 {len(edges)} · 자체 인덱스 = CTE")
    # ★ **답이 같은데 우리 것만 절단을 싣는다.** stack §2.3 의 결정적 이유가 여기서
    # 관측된다 — `LIMIT` 은 *"한도에 걸린 지점의 사유별 분해"* 를 표현하지 못한다.
    적음(f"        └ 그중 {절단_실린_수}개는 **우리 답에만 절단이 실렸다** (CTE 는 못 낸다)")
    기록.append(f"{tag}: 답 일치 {len(출발점) - len(갈림)}/{len(출발점)} · 절단이 실린 답 {절단_실린_수}")

    # ── ★ 반대 방향 ─────────────────────────────────────────────────────────
    if 뒤집어_갈린_수 == 0:
        어긋남(f"③㉠ {tag} — 엣지를 뒤집어도 답이 그대로다. CTE 가 아무것도 안 재고 있다")
    else:
        ok(f"③㉠ {tag} — 방향을 뒤집으면 {뒤집어_갈린_수}/{len(출발점)} 에서 갈린다")

    if 깊은_합 <= 얕은_합:
        어긋남(f"③㉡ {tag} — 깊이를 {깊이}→{깊은_깊이} 로 올려도 답이 안 늘었다 ({얕은_합} → {깊은_합})")
    else:
        ok(f"③㉡ {tag} — 깊이 {깊이}→{깊은_깊이} 에서 도달이 {얕은_합} → {깊은_합}")

    기록.append(f"{tag}: 노드 {len(nodes)} · 엣지 {len(edges)} · 출발점 {len(출발점)}")
    conn.close()
    shutil.rmtree(room, ignore_errors=True)


# ═════════════════════════════════════════════════════════════════════════════
# ⑨ F04 가 넘긴 「옳은 선」 — 고정비를 빼고 잰다
# ═════════════════════════════════════════════════════════════════════════════

전부_제외 = (
    '[[repo]]\nid = "{id}"\npath = "."\n'
    '[repo.exclude]\nrules = [{{ id = "all", glob = "**" }}]\n'
)


def 벽시계(args: list[str]) -> float:
    t = time.perf_counter()
    pal(args)
    return time.perf_counter() - t


def 추출_대상(js: str) -> int:
    d = json.loads(js)
    n = 0
    for e in d["ledger"]["entries"]:
        k = next(iter(e["state"])) if isinstance(e["state"], dict) else e["state"]
        if k in ("parsed", "partial"):
            n += 1
    return n


def 대조_벤치(repo: Path, at: str, tag: str) -> None:
    적음(f"── ⑨ 콜드/증분 비율 — 고정비를 뺀다 · {tag} ────────────────────")
    room = 방(f"bench-{tag}")
    cache = room / "cache"

    ledger_args = ["ledger", str(repo), "--at", at, "--cache-dir", str(cache), "--json"]
    콜드 = 벽시계(ledger_args)
    js = pal(ledger_args)  # 이 회차는 전량 적중이다 — 대상 수만 센다
    증분 = 벽시계(ledger_args)
    대상 = 추출_대상(js)
    if 대상 == 0:
        어긋남(f"⑨ {tag} — 추출 대상이 0 이다. 이 대조는 성립하지 않는다")
        shutil.rmtree(room, ignore_errors=True)
        return

    # 고정비 — **추출 대상이 0 인 회차**. 매니페스트로 전부 범위 밖으로 만든다.
    manifest = repo / ".palimpsest/manifest.toml"
    떠둔것 = manifest.read_text() if manifest.exists() else None
    manifest.parent.mkdir(parents=True, exist_ok=True)
    try:
        manifest.write_text(전부_제외.format(id=tag))
        고정비_방 = 방(f"fixed-{tag}")
        확인 = pal(["ledger", str(repo), "--at", at, "--cache-dir", str(고정비_방 / "cache"), "--json"])
        if 추출_대상(확인) != 0:
            어긋남(f"⑨ {tag} — 전부 제외했는데 추출 대상이 {추출_대상(확인)} 이다. 변형이 안 먹었다")
            return
        고정비 = 벽시계(["ledger", str(repo), "--at", at, "--cache-dir", str(고정비_방 / "cache"), "--json"])
        shutil.rmtree(고정비_방, ignore_errors=True)
    finally:
        if 떠둔것 is None:
            manifest.unlink(missing_ok=True)
            try:
                manifest.parent.rmdir()
            except OSError:
                pass
        else:
            manifest.write_text(떠둔것)

    적음(f"  콜드 {콜드:.2f}s · 증분 {증분:.2f}s · 고정비 {고정비:.2f}s · 추출 대상 {대상}")
    기록.append(
        f"{tag}: 콜드 {콜드:.2f}s · 증분 {증분:.2f}s · 고정비 {고정비:.2f}s · 대상 {대상} 파일"
    )

    if 고정비 > 콜드 * 0.5:
        기권(
            f"⑨ {tag} — 고정비가 콜드의 {고정비 / 콜드:.0%} 다 (>50%). "
            "이 코퍼스에서는 추출이 지배적이지 않다 — **대조 불가**"
        )
        shutil.rmtree(room, ignore_errors=True)
        return

    분모 = 증분 - 고정비
    if 분모 <= 0:
        기권(f"⑨ {tag} — 증분({증분:.2f}s)이 고정비({고정비:.2f}s) 이하다 — 잴 것이 없다")
        shutil.rmtree(room, ignore_errors=True)
        return

    비율 = (콜드 - 고정비) / 분모
    옛_비율 = 콜드 / 증분
    적음(f"  옛 선(콜드÷증분) {옛_비율:.1f}배 · **옳은 선** {비율:.1f}배")
    기록.append(f"{tag}: 옛 선 {옛_비율:.1f}배 · 옳은 선 {비율:.1f}배")
    if 비율 >= 10:
        ok(f"⑨ {tag} — 고정비를 뺀 비율 {비율:.1f}배 ≥ 10")
    else:
        어긋남(f"⑨ {tag} — 고정비를 뺀 비율이 {비율:.1f}배다 (선 10배)")

    shutil.rmtree(room, ignore_errors=True)


# ═════════════════════════════════════════════════════════════════════════════


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--skip-bench", action="store_true", help="⑨ 를 건너뛴다")
    a = ap.parse_args()

    if not BIN.exists():
        raise SystemExit(f"{BIN} 가 없다 — `cargo build --workspace --release` 를 먼저")

    적음()
    적음("F05 대조 — 바깥 오라클이 우리 인덱스를 채점한다")
    적음(f"  {platform.platform()} · python {platform.python_version()}")
    적음()

    for repo, at, tag in (DITTO, PORTAL):
        if not repo.exists():
            기권(f"{tag} 코퍼스가 없다: {repo} — 대조 불가")
            continue
        대조_CTE(repo, at, tag)
        적음()
        if not a.skip_bench:
            대조_벤치(repo, at, tag)
            적음()

    적음("── 기록 ───────────────────────────────────────────────────────────")
    for line in 기록:
        적음(f"  {line}")
    적음()

    if 대조불가:
        적음(f"대조 불가 {len(대조불가)}건:")
        for line in 대조불가:
            적음(f"  – {line}")
        적음()
    if 실패:
        적음(f"어긋남 {len(실패)}건:")
        for line in 실패:
            적음(f"  · {line}")
        return 1
    적음("어긋남 0")
    return 0


if __name__ == "__main__":
    sys.exit(main())
