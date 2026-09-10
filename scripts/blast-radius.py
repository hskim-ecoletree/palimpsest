#!/usr/bin/env python3
"""결박의 **사정권**과 완수 조건을 교차한다 — 소유자 승격 칸 3 이 요구한 자.

    python3 scripts/blast-radius.py <intent.md>

★ **이 스크립트가 있는 까닭**: 앞 판은 같은 표를 손으로 산출하고 **그 자를 안 남겼다.**
그래서 「밖 18 · 못 쟀다 15」가 어느 조건인지 원장이 재검할 길이 없었다(독립 리뷰
라운드 2 · 항 1·10). **산출한 자가 없으면 그 표는 판정이 아니라 주장이다.**

## 무엇을 재나

⑴ `pal query binding.status --json` 으로 결박이 지켜보는 **심볼 id** 를 전부 뜨고,
⑵ `pal export --format cypher` 로 그 id 를 **(이름, 파일)** 좌표로 되푼 뒤,
⑶ `pal round conditions --file <intent.md> --json` 의 조건 문면에서 그 이름이나 파일을
   찾는다.

## 이 자의 한계 — **문면 대조다**

조건이 감시 좌표를 **이름으로 적었을 때만** 걸린다. 조건이 그 코드를 산문으로만 가리키면
「못 쟀다」로 간다. 그러므로 **「밖」은 「안 걸린다」가 아니라 「이 자로는 안 걸린다」**다.
그 사실을 산출에 함께 적는다 — 안 적으면 이 표가 자기 한계를 감춘다.
"""

import json
import re
import subprocess
import sys
from pathlib import Path

PAL = "./target/release/pal"


def 돌린다(args: list[str]) -> str:
    p = subprocess.run([PAL, *args], capture_output=True, text=True, check=False)
    if p.returncode != 0:
        raise SystemExit(f"{' '.join(args)} 가 {p.returncode} 로 끝났다:\n{p.stderr[-800:]}")
    return p.stdout


def main() -> int:
    if len(sys.argv) != 2:
        raise SystemExit(__doc__)
    의도 = sys.argv[1]

    조건 = [(c["id"], c["원문"]) for c in json.loads(돌린다(["round", "conditions", "--file", 의도, "--json"]))["조건"]]
    결박 = json.loads(돌린다(["query", "binding.status", "--json"]))["answer"]["bindings"]
    사이퍼 = 돌린다(["export", "--format", "cypher"])
    심볼 = {
        m.group(1): (m.group(2), m.group(3))
        for m in re.finditer(
            r'CREATE \(:Symbol \{id: "([0-9a-f]+)", name: "([^"]*)", kind: "[^"]*", path: "([^"]*)"',
            사이퍼,
        )
    }

    감시: dict[tuple[str, str], set[str]] = {}
    좌표없음: list[str] = []
    for b in 결박:
        t = b.get("target")
        if t in 심볼:
            감시.setdefault(심볼[t], set()).add(b["binding"][:8])
        else:
            좌표없음.append(b["binding"][:8])

    안: list[tuple[str, list[str]]] = []
    밖: list[str] = []
    for cid, 원문 in 조건:
        닿는 = sorted(
            {n for (n, f) in 감시 if re.search(rf"`{re.escape(n)}`", 원문) or f in 원문}
        )
        (안.append((cid, 닿는)) if 닿는 else 밖.append(cid))

    print(f"결박 {len(결박)} · 감시 좌표 {len(감시)} · 좌표를 못 되푼 결박 {len(좌표없음)}")
    print(f"조건 {len(조건)} — **안 {len(안)} · 밖 {len(밖)} · 합 {len(안) + len(밖)}**")
    print()
    print("| 판정 | 수 | 조건 |")
    print("|---|---:|---|")
    안_칸 = " · ".join(f"`{c}`({', '.join('`' + w + '`' for w in ws)})" for c, ws in 안)
    print(f"| **사정권 안** | **{len(안)}** | {안_칸 or '—'} |")
    print(f"| 사정권 밖 | {len(밖)} | {' '.join('`' + c + '`' for c in 밖) or '—'} |")
    print()
    print("⚠ **「밖」은 「안 걸린다」가 아니라 「문면 대조로는 안 걸린다」다.**")
    return 0


if __name__ == "__main__":
    sys.exit(main())
