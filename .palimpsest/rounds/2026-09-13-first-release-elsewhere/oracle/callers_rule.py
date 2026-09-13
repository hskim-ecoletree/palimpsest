#!/usr/bin/env python3
"""`E2` ⑵ · `E4` 의 호출자 읽기 규칙 — `intent.md` `## 개정`(판 `p2-caller-sites` 정정)을 그대로 옮긴다.

`touch` 는 호출자의 **수**만 싣는다. 그 수의 내용은 같은 스냅샷에서 한 심볼로 풀리고 줄 수가 같은
`pal query symbol.callers` 목록으로 읽는다.

심볼 NN 마다:
  - `touch` 산출 = `03-touch/NN-*-pick.json.txt` 가 있으면 그것, 없으면 `NN-*.json.txt`
  - 수 = `answer.facts.present.callers` · 한 심볼 답 = `answer` 에 `facts` 가 있다(후보 화면은 `candidates`)
  - 스냅샷 꼬리표 = 사람 화면 머리 둘째 줄의 `#` 앞 · 목록의 `Snapshot` 줄
  - 목록 = `04-callers/NN-*.txt` 의 `  종류 이름 경로:줄` 줄

사용(모듈): `읽기(effect_dir, 변이)` → 행 목록. 변이는 음성 대조용이다:
  `{"count": {"05": 3}}` — 그 심볼의 수를 바꿔 넣는다 · `{"drop_line": ["05"]}` — 목록 첫 줄을 지운다
"""
import json
import re
from pathlib import Path

줄꼴 = re.compile(r"^\s{2}\S+\s+(\S+)\s+(\S+):(\d+)\s*$")


def 읽기(effect_dir, 변이=None):
    변이 = 변이 or {}
    fx = Path(effect_dir)
    행 = []
    for base in sorted(p for p in (fx / "03-touch").glob("[0-9][0-9]-*.json.txt") if "-pick" not in p.name):
        nn = base.name[:2]
        pick = base.with_name(base.name.replace(".json.txt", "-pick.json.txt"))
        tj = pick if pick.exists() else base
        tt = tj.with_name(tj.name.replace(".json.txt", ".txt"))
        ans = json.loads(tj.read_text(encoding="utf-8"))["answer"]
        한_심볼 = "facts" in ans
        수 = ans["facts"]["present"]["callers"] if 한_심볼 else None
        if nn in 변이.get("count", {}):
            수 = 변이["count"][nn]
        머리 = tt.read_text(encoding="utf-8").splitlines()
        꼬리표_t = 머리[1].split("·")[-1].strip().split("#")[0] if len(머리) > 1 else ""
        자기_파일 = 머리[2].split("·")[1].strip().rsplit(":", 1)[0] if len(머리) > 2 and "·" in 머리[2] else ""
        cfs = sorted((fx / "04-callers").glob(f"{nn}-*.txt"))
        목록, 꼬리표_c = [], ""
        if cfs:
            for l in cfs[0].read_text(encoding="utf-8").splitlines():
                m = 줄꼴.match(l)
                if m:
                    목록.append((m.group(1), m.group(2), int(m.group(3))))
                if l.strip().startswith("Snapshot"):
                    꼬리표_c = l.split("Snapshot", 1)[1].strip()
        if nn in 변이.get("drop_line", []) and 목록:
            목록 = 목록[1:]
        같은_꼬리표 = bool(꼬리표_t) and 꼬리표_t == 꼬리표_c
        규칙 = 한_심볼 and 같은_꼬리표 and 수 is not None and len(목록) == 수
        행.append(dict(nn=nn, 이름=base.name[3:].replace(".json.txt", ""), 지목=tj is pick, 한_심볼=한_심볼, 수=수,
                       목록_줄=len(목록), 같은_꼬리표=같은_꼬리표, 규칙=규칙, 자기_파일=자기_파일,
                       경로=[p for _, p, _ in 목록], 다른_파일=[f"{n} {p}:{ln}" for n, p, ln in 목록 if p != 자기_파일]))
    return 행


def 표(행):
    out = ["| NN | 심볼 | 지목(-pick) | 한 심볼 답 | touch 수 | 목록 줄 | 같은 스냅샷 | 규칙 | 다른 파일 호출자 |", "|---|---|---|---|---|---|---|---|---|"]
    for r in 행:
        out.append(f"| {r['nn']} | `{r['이름']}` | {'예' if r['지목'] else '아니오'} | {'예' if r['한_심볼'] else '아니오'} | {r['수']} | {r['목록_줄']} | {'예' if r['같은_꼬리표'] else '아니오'} | {'채움' if r['규칙'] else '못 채움'} | {' · '.join(r['다른_파일']) or '—'} |")
    return out


if __name__ == "__main__":
    import sys
    변이 = json.loads(sys.argv[2]) if len(sys.argv) > 2 else None
    행 = 읽기(sys.argv[1], 변이)
    if "--c" in sys.argv:
        수1 = [r for r in 행 if (r["수"] or 0) >= 1]
        if any(not r["규칙"] for r in 수1):
            print("__대조불가__"); sys.exit(0)
        print("\n".join(sorted({p for r in 수1 for p in r["경로"]})))
    else:
        print("\n".join(표(행)))
