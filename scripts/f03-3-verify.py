#!/usr/bin/env python3
"""F03-3 대조 — 이동과 골든: 좌표가 움직인 것이 보이는가.

합격선은 `corpus/criteria.toml` `[f03.3]` 에 있고 **코드보다 먼저 등록됐다**
(커밋 `3621f6d`).

    ① 별칭 테이블이 **의도 저장소**에 산다 · 파생층을 지워도 남는다 ★
    ② 재결박 제안 신호 — **자동이 아니다** ★
    ③ `(symbol_id, body_digest)` 골든 스냅샷
    ④ 선택 필드 금지 CI 검사 1단계 ★

**① · ② · ④ 는 단위·통합 시험이 진다** — 이 스크립트는 그것이 실제로 돌았는지
확인하고, **③ 만 코퍼스에 댄다.**

# 골든이 무엇을 말하고 무엇을 말하지 않는가

골든은 ***변하지 않았음***만 말하고 ***빠뜨리지 않았음***은 말하지 않는다
(F02 §6 · F02 지붕 §2). 빠뜨리지 않았음을 지는 것은 손 목록
(`corpus/tasks/f02-recall-sample.tsv`)이다. **둘을 바꿔 읽으면 안 된다.**

사용:
    ./scripts/f03-3-verify.py
    ./scripts/f03-3-verify.py --bless      # 움직인 것을 목록으로 낸 뒤 축복한다
"""

from __future__ import annotations

import argparse
import collections
import json
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
BIN = ROOT / "target/release/pal"

# **골든 둘.** 두 언어가 서로 다른 것을 내므로 한 파일로 접지 않는다.
GOLDEN = [
    ("ditto", Path.home() / "dev/projects/ditto", "aded7ce7f88f",
     ROOT / "corpus/golden/ditto.symbols.tsv"),
    ("portal-backend", Path.home() / "dev/projects/boxwood/portal-backend", "a29cad0bf6a8",
     ROOT / "corpus/golden/portal-backend.symbols.tsv"),
]

HEADER = "path\tcontainer\tname\tkind\tidentity\tsymbol_id\tbody_digest"


def run(args: list[str], **kw) -> subprocess.CompletedProcess:
    return subprocess.run(args, capture_output=True, text=True, check=False, **kw)


def snapshot(repo: Path, at: str, cache: Path) -> list[str]:
    """골든 한 벌 — **대장이 낸 순서 그대로.**

    다시 정렬하지 않는다. 대장의 순서가 결정적이라는 사실이 이 파일에서도 보여야 한다.
    """
    p = run([str(BIN), "ledger", str(repo), "--at", at, "--cache-dir", str(cache), "--symbols"])
    if p.returncode != 0:
        raise SystemExit(f"대장을 내지 못했다: {p.stderr[-400:]}")
    rows = [HEADER]
    for line in p.stdout.splitlines():
        if not line:
            continue
        d = json.loads(line)
        rows.append(
            "\t".join(
                [
                    d["path"],
                    ".".join(d["container"]),
                    d["name"],
                    d["kind"],
                    d["identity"],
                    d["id"],
                    d["body"],
                ]
            )
        )
    return rows


def key(row: str) -> tuple:
    f = row.split("\t")
    return (f[0], f[1], f[2], f[3])


def diff(old: list[str], new: list[str]) -> dict[str, list[str]]:
    """**건수가 아니라 목록.** `[f03.3.pass]` ③ 이 요구하는 형태다."""
    o = {key(r): r for r in old[1:]}
    n = {key(r): r for r in new[1:]}
    out: dict[str, list[str]] = {"사라짐": [], "새로": [], "좌표 이동": [], "요약 이동": []}
    for k in o.keys() - n.keys():
        out["사라짐"].append("\t".join(k))
    for k in n.keys() - o.keys():
        out["새로"].append("\t".join(k))
    for k in o.keys() & n.keys():
        of = o[k].split("\t")
        nf = n[k].split("\t")
        if of[5] != nf[5]:
            out["좌표 이동"].append(f"{'.'.join(k[:3])}  {of[5][:12]} → {nf[5][:12]}")
        if of[6] != nf[6]:
            out["요약 이동"].append(f"{'.'.join(k[:3])}  {of[6][:12]} → {nf[6][:12]}")
    return out


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--bless", action="store_true",
                    help="움직인 것을 목록으로 낸 뒤 골든을 다시 축복한다")
    a = ap.parse_args()

    if not BIN.exists():
        print(f"  {BIN} 이 없다 — `cargo build --workspace --release`", file=sys.stderr)
        return 1

    print("F03-3 — 이동과 골든")
    print()

    # ── ① · ② · ④ 가 실제로 돌았는가 ────────────────────────────────────────
    listed = run(["cargo", "test", "--workspace", "--", "--list"], cwd=ROOT).stdout
    필수 = {
        "① 별칭이 파생층 삭제를 견딘다": "별칭은_파생층을_지워도_남는다",
        "① 별칭 사슬이 멈춘다": "별칭_사슬이_한_바퀴_돌아도_멈춘다",
        "② 후보가 없으면 빈 목록 ★": "후보가_없으면_빈_목록이다",
        "② 살아 있는 좌표는 제안 안 함": "살아_있는_좌표는_제안하지_않는다",
        "② 신호 셋이 값으로 남는다": "신호_셋이_후보를_좁히고_그_이유가_값으로_남는다",
    }
    missing = [n for n, t in 필수.items() if t not in listed]
    if missing:
        print(f"  FAIL  시험이 없다: {missing}")
        return 1
    print(f"  ok    ① · ② 의 시험 {len(필수)} 개가 선다")

    x = run(["cargo", "xtask", "check"], cwd=ROOT)
    if "선택 필드 금지" not in x.stdout:
        print("  FAIL  ④ 선택 필드 금지 검사가 `cargo xtask check` 에 없다")
        return 1
    if x.returncode != 0:
        print(f"  FAIL  ④ `cargo xtask check` 가 실패했다\n{x.stdout[-600:]}")
        return 1
    print("  ok    ④ 선택 필드 금지 검사가 돌고 통과한다")
    print()

    # ── ② 자동 적용 경로가 없다 ★ ──────────────────────────────────────────
    # **없는 것을 시험한다.** 있으면 F03 §5 가 기각한 자동 재결박이 서 버린다.
    intent = (ROOT / "crates/pal-intent/src").rglob("*.rs")
    적용 = [f for f in intent if "RebindProposal" in f.read_text()]
    if 적용:
        print(f"  FAIL  ② 의도 저장소가 제안을 받는다 — 자동 적용 경로다: {적용}")
        return 1
    print("  ok    ② 의도 저장소에 제안을 받는 경로가 없다 ★")
    print()

    # ── ③ 골든 ─────────────────────────────────────────────────────────────
    bad = False
    with tempfile.TemporaryDirectory() as td:
        tmp = Path(td)
        for name, repo, pin, path in GOLDEN:
            if not (repo / ".git").exists():
                print(f"  FAIL  ③ 코퍼스가 없다: {repo}")
                return 1
            now = snapshot(repo, pin, tmp / name)
            if not path.exists():
                if not a.bless:
                    print(f"  FAIL  ③ 골든이 없다: {path.name} — `--bless` 로 만든다")
                    bad = True
                    continue
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text("\n".join(now) + "\n")
                print(f"  냈다  ③ {path.name}  {len(now) - 1} 줄")
                continue

            old = path.read_text().splitlines()
            if old == now:
                print(f"  ok    ③ {name}  {len(now) - 1} 줄 · 움직인 것 0")
                continue

            d = diff(old, now)
            total = sum(len(v) for v in d.values())
            print(f"  {'냈다' if a.bless else 'FAIL'}  ③ {name}  움직인 것 {total}")
            for 종류, items in d.items():
                if not items:
                    continue
                print(f"        {종류} {len(items)}")
                for it in items[:10]:
                    print(f"          {it}")
                if len(items) > 10:
                    print(f"          … {len(items) - 10} 더 (전부는 게이트에)")
            if a.bless:
                path.write_text("\n".join(now) + "\n")
            else:
                bad = True

    print()
    if bad:
        print("골든이 움직였다 — **목록을 게이트에 적고 나서** `--bless` 한다")
        return 1
    print("넷 다 통과")
    return 0


if __name__ == "__main__":
    sys.exit(main())
