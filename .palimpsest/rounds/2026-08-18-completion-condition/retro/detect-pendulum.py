# -*- coding: utf-8 -*-
"""진자 검출 — 지표 P1·P2·P3. 등록: retro/06-pendulum-metrics.md (커밋 8134c93)"""
import subprocess, sys, collections, re

def sh(c):
    r = subprocess.run(c, shell=True, capture_output=True, text=True)
    return r.stdout

def paths(rng):
    out = collections.Counter()
    for line in sh(f"git log --format='@%h' --name-only {rng}").split('\n'):
        line = line.strip()
        if not line or line.startswith('@'):
            continue
        out[line] += 1
    return out

def P1(rng, start, end, touched):
    """순변경 공집합: 2회 이상 커밋됐는데 착수↔종료 diff 가 비었다"""
    hits = []
    for p, n in touched.items():
        if n < 2:
            continue
        if not sh(f"git diff {start} {end} -- '{p}'").strip():
            hits.append((p, n))
    return hits

def P2(rng, touched):
    """되돌린 줄: 뒤 커밋이 앞 커밋이 추가한 줄을 지웠다"""
    hits = []
    for p, n in touched.items():
        if n < 2:
            continue
        added, removed = collections.Counter(), collections.Counter()
        cur = None
        for line in sh(f"git log --reverse --format='@%h' -p --unified=0 {rng} -- '{p}'").split('\n'):
            if line.startswith('@') and len(line) < 12:
                cur = line[1:]; continue
            if line.startswith('+++') or line.startswith('---'):
                continue
            if line.startswith('+'):
                t = line[1:].strip()
                if len(t) > 12: added[t] = cur
            elif line.startswith('-'):
                t = line[1:].strip()
                if len(t) > 12 and t in added and added[t] != cur:
                    removed[t] = cur
        if removed:
            hits.append((p, n, len(removed), list(removed)[:1]))
    return hits

def P3(rng, start, end, tokens):
    hits = []
    for t in tokens:
        cs = [x for x in sh(f"git log --format='%h' -S'{t}' {rng}").split() if x]
        if len(cs) < 3:
            continue
        a = sh(f"git grep -c '{t}' {start} -- . 2>/dev/null | wc -l").strip()
        b = sh(f"git grep -c '{t}' {end} -- . 2>/dev/null | wc -l").strip()
        hits.append((t, len(cs), a, b, a == b))
    return hits

TOKENS = ['계획 §', '옛', 'DESIGN §', 'WHITEPAPER', 'how-it-works', 'docs/adr/README.md']

def run(name, start, end):
    rng = f"{start}..{end}"
    touched = paths(rng)
    n_all = len(touched)
    n_multi = sum(1 for v in touched.values() if v >= 2)
    print(f"===== {name} · {rng} =====")
    print(f"만진 경로 {n_all} · 2회 이상 {n_multi} ({100.0*n_multi/n_all:.0f}%)" if n_all else "경로 0")
    p1 = P1(rng, start, end, touched)
    print(f"\n[P1 순변경 공집합] {len(p1)}  (상한 {int(n_all*0.05)} = 5%)")
    for p, n in sorted(p1): print(f"    {n}회  {p}")
    p2 = P2(rng, touched)
    print(f"\n[P2 되돌린 줄] {len(p2)}  (상한 {int(n_all*0.10)} = 10%)")
    for p, n, k, ex in sorted(p2): print(f"    {n}회 · 왕복줄 {k}  {p}\n           예: {ex[0][:70]}")
    p3 = P3(rng, start, end, TOKENS)
    print(f"\n[P3 토큰 왕복] 후보 {len(TOKENS)} 중 {len(p3)} 걸림")
    for t, c, a, b, same in p3: print(f"    '{t}'  {c}커밋  파일수 {a}→{b}  최종동일={same}")
    union = set(p for p, _ in p1) | set(p for p, _, _, _ in p2)
    lim = int(n_all*0.20)
    print(f"\n[합집합] {len(union)} / {n_all}  (상한 {lim} = 20%)  → {'통과' if len(union) <= lim else '★ 상한 초과'}")
    print()
    return len(union), lim

if __name__ == '__main__':
    run(sys.argv[1], sys.argv[2], sys.argv[3])
