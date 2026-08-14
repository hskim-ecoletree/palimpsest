#!/usr/bin/env python3
"""F06(#9) 대조 — **목록이 계약인가. 그리고 그 계약이 호스트 없이 지켜지는가.**

합격선 정본은 `corpus/criteria.toml` `[f06]`·`[f06.1]`~`[f06.3]` 이고 판정은
`docs/gates/F06.md` 다.

이 스크립트가 재는 것 넷. 나머지는 `cargo test` 가 상시로 잰다
(`catalog_surface.rs` · `envelope_two_layer.rs` · `host_free.rs`):

    ①  ★ **카탈로그 음성 대조 여섯** — 방향을 각각 망가뜨리면 각각 실패하는가
    ②  **호스트 없는 코어** — 표면 크레이트 없이 코어~질의가 전건 통과하는가
    ③  **실물 코퍼스의 내보내기** — `pal export` 의 건수가 `graph.dump` 와 같은가
    ④  **실물 코퍼스의 토큰 추정** — 잰 값이 실제와 맞고 단조인가

## ① 이 이 기능에서 가장 중요하다 (R-18)

**표면에는 댈 바깥이 없다.** *"이 목록이 옳은 목록인가"* 를 말해 줄 표가 존재하지
않는다 — 그 파일이 곧 정본이기 때문이고, F22-1 이 스키마에서 만난 것과 같은 자리다.
그래서 오라클을 **대조의 양방향성**으로 잡는다.

F22-1 은 음성 대조 **9/9** 로 각 방향을 **망가뜨려서** 세웠다. 여기서 그 자격을
낮추지 않는다 — 검사가 통과한다는 사실은 검사가 무언가를 센다는 증거가 아니다.

### 변형마다 셋을 확인한다

    ⓐ  변형 **전**에 검사가 통과한다      ← 아니면 그 회차는 아무것도 안 쟀다
    ⓑ  변형이 파일을 **실제로 바꿨다**    ← 아니면 아무것도 안 망가뜨린 것이다
    ⓒ  변형 **후**에 검사가 실패한다      ← 이것이 재려는 것
    그리고 회차마다 원본을 되돌리고 ⓐ 로 확인한다

⚠ **이 대조는 소스를 변이시킨다** — `surface/queries.toml` 하나다. **빌드를 다시 하지
않는다**(카탈로그는 `cargo xtask check` 가 파일에서 읽는다). 그래도 **도는 동안
커밋하지 마라.**

## ③④ 는 실물에서 잰다

`cargo test` 의 시험 저장소는 심볼 백 단위다. `ditto` 는 **노드 4,578 · 엣지 4,601**
이고 그 값이 F05 에서 이미 관측됐다 — **하한이자 대조값**으로 쓴다.

사용:
    ./scripts/f06-verify.py
    ./scripts/f06-verify.py --skip-corpus    # ③④ 를 건너뛴다 (코퍼스가 없을 때)

종료 코드:
    0  전부 통과
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
BIN = ROOT / "target/release/pal"
CATALOG = ROOT / "surface/queries.toml"

# ⚠ **핀을 건다.** 워킹트리를 재면 F05 가 관측한 값과 다른 나무를 재게 되고,
# 그러면 대조값이 어긋나는 것이 결함이 아니라 **잰 대상이 다른 것**이다
# (`corpus/manifest.toml` 의 `[corpus.ditto].head`).
DITTO = (Path.home() / "dev/projects/ditto", "aded7ce7f88f", "ditto")

# **하한이다.** 이보다 적으면 시험되지 않은 것이고, 시험되지 않은 대조는 `–` 가 아니라
# 실패다(`2e2eb3f`).
최소_질의 = 6
최소_노드 = 100
최소_변형 = 6
# `ditto` 에서 F05 가 관측한 값. **대조값이자 하한이다.**
DITTO_노드 = 4578
DITTO_엣지 = 4601
# 토큰 추정의 허용 오차 — 자기 자신을 못 세는 만큼만 어긋나야 한다.
토큰_오차 = 0.10
# 단조를 재려면 두 답이 이만큼 갈려야 한다. 비슷한 둘로 재면 우연히 성립한다.
단조_배수 = 3


class 결과:
    def __init__(self) -> None:
        self.줄: list[str] = []
        self.어긋남: list[str] = []
        self.대조불가: list[str] = []

    def ok(self, 무엇: str, 값: str) -> None:
        self.줄.append(f"  ok    {무엇}  — {값}")

    def fail(self, 무엇: str, 값: str) -> None:
        self.줄.append(f"  FAIL  {무엇}  — {값}")
        self.어긋남.append(f"{무엇}: {값}")

    def 기권(self, 무엇: str, 값: str) -> None:
        self.줄.append(f"  –     {무엇}  — {값}")
        self.대조불가.append(f"{무엇}: {값}")


def xtask() -> tuple[bool, str]:
    """`cargo xtask check` 를 돌린다. 통과 여부와 카탈로그 줄을 낸다."""
    p = subprocess.run(
        ["cargo", "run", "-q", "-p", "xtask", "--", "check"],
        cwd=ROOT,
        capture_output=True,
        text=True,
    )
    return p.returncode == 0, p.stdout + p.stderr


# ─────────────────────────────────────────────────────────────────────────────
# ① 카탈로그 음성 대조 — **방향마다 하나씩 망가뜨린다**
# ─────────────────────────────────────────────────────────────────────────────

def 변형들() -> list[tuple[str, callable, str]]:
    """`(이름, 원본→변형, 걸려야 하는 방향)`.

    **여섯이다.** 하나라도 통과하면 그 방향은 아무것도 안 세고 있다.
    """
    def 이름_뺀다(s: str) -> str:
        # `graph.dump` 절 전체를 지운다 — 코드에 있는데 카탈로그에 없다.
        i = s.index('[query."graph.dump"]')
        return s[:i]

    def 이름_더한다(s: str) -> str:
        # 카탈로그에 있는데 코드에 없다.
        return s + (
            '\n[query."refs.callers"]\n'
            'summary    = "아직 없는 것"\n'
            "args       = [{ name = \"name\", type = \"SymbolName\", required = true }]\n"
            'returns    = "Symbols"\n'
            'introduced = "F07"\n'
        )

    def 인자_비운다(s: str) -> str:
        return s.replace(
            'args       = [{ name = "name", type = "SymbolName", required = true }]',
            "args       = []",
            1,
        )

    def 인자_더한다(s: str) -> str:
        # 인자를 안 받는 질의(`graph.dump`)에 하나를 붙인다.
        i = s.index('[query."graph.dump"]')
        return s[:i] + s[i:].replace(
            "args       = []",
            'args       = [{ name = "무엇", type = "SymbolName", required = true }]',
            1,
        )

    def 철자_바꾼다(s: str) -> str:
        return s.replace('[query."symbol.callers"]', '[query."symbol.caller"]', 1)

    def 반환_바꾼다(s: str) -> str:
        i = s.index('[query."graph.dump"]')
        return s[:i] + s[i:].replace('returns    = "Graph"', 'returns    = "Nodes"', 1)

    return [
        ("이름을 뺀다", 이름_뺀다, "방향 2 — 코드에 있는데 카탈로그에 없다"),
        ("없는 이름을 더한다", 이름_더한다, "방향 1 — 카탈로그에 있는데 코드에 없다"),
        ("인자를 비운다", 인자_비운다, "방향 3 — 인자 이름"),
        ("인자를 더한다", 인자_더한다, "방향 3 — 인자 이름"),
        ("철자를 바꾼다", 철자_바꾼다, "방향 1·2 둘 다"),
        ("반환을 바꾼다", 반환_바꾼다, "방향 3 — 반환"),
    ]


def 카탈로그_음성_대조(r: 결과) -> None:
    원본 = CATALOG.read_text()
    질의_수 = 원본.count("[query.")
    if 질의_수 < 최소_질의:
        r.fail("① 카탈로그 하한", f"질의가 {질의_수}개다 — {최소_질의}개 미만")
        return

    # ⓐ **변형 전에 통과해야 한다.** 이미 실패하고 있으면 아래가 아무것도 안 잰다.
    통과, 산출 = xtask()
    if not 통과:
        r.fail("① 기준선", f"변형 전에 `cargo xtask check` 가 이미 실패한다\n{산출[-800:]}")
        return

    변형 = 변형들()
    if len(변형) < 최소_변형:
        r.fail("① 변형 하한", f"변형이 {len(변형)}개다")
        return

    잡은 = 0
    try:
        for 이름, 만들기, 방향 in 변형:
            바뀐 = 만들기(원본)
            # ⓑ **변형이 아무것도 안 바꾸면 실패다** (대조가 꺼지는 형태 ①).
            if 바뀐 == 원본:
                r.fail(f"① {이름}", "변형이 파일을 안 바꿨다")
                continue
            CATALOG.write_text(바뀐)
            통과, 산출 = xtask()
            CATALOG.write_text(원본)
            # ⓒ
            if 통과:
                r.fail(f"① {이름}", f"망가뜨렸는데 검사가 통과했다 ({방향})")
            else:
                if "카탈로그 정합" not in 산출:
                    r.fail(f"① {이름}", "실패했는데 카탈로그 정합이 아니다")
                else:
                    잡은 += 1
    finally:
        # **되돌린다.** 도중에 죽어도 원본이 남아야 한다.
        CATALOG.write_text(원본)

    # 되돌린 뒤 다시 통과해야 한다 — 아니면 이 스크립트가 저장소를 망가뜨린 것이다.
    통과, _ = xtask()
    if not 통과:
        r.fail("① 복원", "변형을 되돌렸는데 검사가 실패한다")
        return

    if 잡은 == len(변형):
        r.ok("① 카탈로그 음성 대조", f"{잡은}/{len(변형)} — 방향마다 각각 잡았다")


# ─────────────────────────────────────────────────────────────────────────────
# ② 호스트 없는 코어 — **표면 크레이트 없이 전건 통과**
# ─────────────────────────────────────────────────────────────────────────────

코어_크레이트 = ["pal-core", "pal-query", "pal-store"]


def 호스트_없는_코어(r: 결과) -> None:
    args = ["cargo", "test"]
    for c in 코어_크레이트:
        args += ["-p", c]
    p = subprocess.run(args, cwd=ROOT, capture_output=True, text=True)
    통과 = sum(
        int(l.split()[3])
        for l in p.stdout.splitlines()
        if l.startswith("test result: ok.")
    )
    if p.returncode != 0:
        r.fail("② 호스트 없는 코어", f"{' · '.join(코어_크레이트)} 가 전건 통과하지 않는다")
        return
    # **하한** — 0 건이면 `-p` 가 잘못돼 아무것도 안 돈 것이다.
    if 통과 == 0:
        r.fail("② 호스트 없는 코어", "돈 시험이 0 건이다")
        return
    r.ok("② 호스트 없는 코어", f"{' · '.join(코어_크레이트)} {통과}건 통과 — pal-cli 없이")

    # `pal-mcp` 는 **없다.** 없는 것이 정상 상태이고, 그러므로 *"어댑터를 빼도
    # 통과한다"* 는 **모집단 0** 이다. 통과로 세지 않는다(ADR-0002).
    if not (ROOT / "crates/pal-mcp").exists():
        r.기권("② 어댑터 부재 통과", "`pal-mcp` 가 아직 없다 — 모집단 0 (P1)")


# ─────────────────────────────────────────────────────────────────────────────
# ③④ 실물 코퍼스
# ─────────────────────────────────────────────────────────────────────────────

def pal(repo: Path, args: list[str], index: Path, rev: str) -> subprocess.CompletedProcess:
    return subprocess.run(
        [str(BIN), *args, "--repo", str(repo), "--index", str(index), "--at", rev],
        cwd=ROOT,
        capture_output=True,
        text=True,
    )


def 코퍼스(r: 결과) -> None:
    repo, rev, 이름 = DITTO
    if not repo.exists():
        r.기권("③④ 실물 코퍼스", f"{repo} 가 없다")
        return

    with tempfile.TemporaryDirectory(prefix="pal-f06-") as tmp:
        index = Path(tmp) / "index.redb"
        out = Path(tmp) / "out.cypher"

        # 먼저 쓰기로 세운다 — 읽기 전용은 없는 2층에 안 붙는다.
        p = pal(repo, ["query", "graph.dump", "--json"], index, rev)
        if p.returncode != 0:
            r.fail("③ 2층", f"{이름} 에서 `graph.dump` 가 실패했다: {p.stderr[-400:]}")
            return
        dump = json.loads(p.stdout)
        노드 = len(dump["answer"]["nodes"])
        엣지 = len(dump["answer"]["edges"])
        if 노드 < 최소_노드:
            r.fail("③ 하한", f"{이름} 의 노드가 {노드}개다")
            return

        # F05 가 관측한 값과 댄다. **움직였으면 그것은 발견이다.**
        if (노드, 엣지) != (DITTO_노드, DITTO_엣지):
            r.fail(
                "③ 코퍼스 대조값",
                f"{이름} 이 노드 {노드} · 엣지 {엣지} — F05 는 {DITTO_노드}·{DITTO_엣지} 였다",
            )

        p = subprocess.run(
            [
                str(BIN), "export", "--format", "cypher",
                "--repo", str(repo), "--index", str(index), "--at", rev,
                "--out", str(out), "--json",
            ],
            cwd=ROOT, capture_output=True, text=True,
        )
        if p.returncode != 0:
            r.fail("③ 내보내기", f"실패했다: {p.stderr[-400:]}")
            return
        보고 = json.loads(p.stdout)["answer"]
        낸 = {c["label"]: c["count"] for c in 보고["exported"]}
        if 낸.get("Symbol") != 노드 or 낸.get("REFERENCES") != 엣지:
            r.fail(
                "③ 내보내기 건수",
                f"Symbol {낸.get('Symbol')} · REFERENCES {낸.get('REFERENCES')} "
                f"— `graph.dump` 는 {노드}·{엣지}",
            )
        else:
            사유 = {m["why"] for m in 보고["missing"]}
            r.ok(
                "③ 내보내기",
                f"{이름} Symbol {낸['Symbol']} · REFERENCES {낸['REFERENCES']} · "
                f"못 낸 라벨 {len(보고['missing'])}개 (사유 {len(사유)}갈래)",
            )

        # **Cypher 문법을 우리가 검증하지 못한다** — 파서가 없다. 대조 불가로 적는다.
        r.기권("③ Cypher 문법", "우리 밖 파서가 없다 — 균형만 셌다(`cargo test`)")

        # ④ 토큰 추정 — 실물에서.
        잰_것 = []
        for 이름_q, 인자 in [
            ("graph.dump", None),
            ("ledger.snapshot", None),
            ("symbol.resolve", "test"),
        ]:
            args = ["query", 이름_q] + ([인자] if 인자 else []) + ["--json"]
            p = pal(repo, args, index, rev)
            if p.returncode != 0:
                r.fail("④ 토큰", f"`{이름_q}` 가 실패했다")
                return
            v = json.loads(p.stdout)
            실제 = len(json.dumps(v, separators=(",", ":"), ensure_ascii=False).encode())
            잰 = v["tokens"]["serialized_bytes"]
            차 = abs(실제 - 잰) / 실제
            if 차 >= 토큰_오차:
                r.fail("④ 토큰 비례", f"`{이름_q}` 가 {차 * 100:.1f}% 어긋난다")
                return
            잰_것.append((이름_q, 실제, v["tokens"]["approx_tokens"]))

        큰 = max(잰_것, key=lambda x: x[1])
        작은 = min(잰_것, key=lambda x: x[1])
        if 큰[1] < 작은[1] * 단조_배수:
            r.fail("④ 토큰 단조", f"가장 큰 답과 작은 답이 {단조_배수}배도 안 차이 난다")
        elif 큰[2] <= 작은[2]:
            r.fail("④ 토큰 단조", "큰 답의 추정이 더 크지 않다 — 상수일 수 있다")
        else:
            r.ok(
                "④ 토큰 추정",
                f"비례 오차 < {토큰_오차 * 100:.0f}% · 단조 {큰[0]}({큰[2]}) > {작은[0]}({작은[2]})",
            )


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--skip-corpus", action="store_true", help="③④ 를 건너뛴다")
    a = ap.parse_args()

    if not BIN.exists():
        print(f"release 바이너리가 없다: {BIN}", file=sys.stderr)
        print("  cargo build --workspace --release", file=sys.stderr)
        return 1

    r = 결과()
    print("F06 대조 — 목록이 계약인가. 그리고 호스트 없이 지켜지는가.\n")
    카탈로그_음성_대조(r)
    호스트_없는_코어(r)
    if a.skip_corpus:
        r.기권("③④ 실물 코퍼스", "--skip-corpus")
    else:
        코퍼스(r)

    for l in r.줄:
        print(l)
    print()
    print(f"어긋남 {len(r.어긋남)} · 대조 불가 {len(r.대조불가)}")
    for x in r.어긋남:
        print(f"  FAIL  {x}")
    return 1 if r.어긋남 else 0


if __name__ == "__main__":
    sys.exit(main())
