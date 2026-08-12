#!/usr/bin/env python3
"""T3 원자료 → 장치별 절감률. 산식은 t3-protocol.md §지표와 산식에 등록돼 있다."""
import statistics as st, os

rows = [l.split('\t') for l in open('corpus/tasks/t3-timing-log.tsv').read().splitlines()[1:]]
rows = [r for r in rows if r[0] != '—']
sec = {r[0]: float(r[2]) for r in rows}
blk = {}
for r in rows:
    blk.setdefault(r[1], []).append((r[0], float(r[2])))

def med(v):
    return st.median(v) if v else float('nan')

base = [s for i, s in blk.get('A1', []) + blk.get('A2', [])]
B = med(base)
print(f'기준선 (A1+A2, n={len(base)}) — 중앙값 {B:.1f}초 · 평균 {st.mean(base):.1f}초' if base else '기준선 없음')

a1, a2 = [s for _, s in blk.get('A1', [])], [s for _, s in blk.get('A2', [])]
if a1 and a2:
    d = med(a2) / med(a1)
    flag = '' if 0.7 <= d <= 1.3 else '  ← **등록 범위 밖. 점추정으로 쓰지 않는다**'
    print(f'드리프트 A2/A1 = {d:.2f}{flag}')

print()
for name, label in (('D', 'D 올라타기(대리·하한)'), ('C', 'C 진척 가시화'), ('E', 'E 거부 재제시'), ('F', 'F 일괄 검산')):
    v = [s for _, s in blk.get(name, [])]
    if not v:
        continue
    m = med(v)
    print(f'{label:24s} 중앙값 {m:7.1f}초 · 절감률 {1 - m / B:+.1%}' if base else f'{label} 중앙값 {m:.1f}초')

bat = [(i, s) for i, s in blk.get('B1', []) + blk.get('B2', []) if i in ('B1', 'B2')]
if bat:
    per = [s / 8 for _, s in bat]
    m = med(per)
    print(f'{"B 일괄 승인 (건당)":24s} 중앙값 {m:7.1f}초 · 절감률 {1 - m / B:+.1%}' if base else '')
    print(f'   묶음 총시간: ' + ' · '.join(f'{i} {s:.1f}초' for i, s in bat))

print('\nD 안에서 — 파일 첫 건 대 후속 3건 (올라타기의 실체)')
d = blk.get('D', [])
if d:
    first = [s for i, s in d if int(i) in (5, 9)]
    rest = [s for i, s in d if int(i) not in (5, 9)]
    if first and rest:
        print(f'   첫 건 중앙값 {med(first):.1f}초 · 후속 중앙값 {med(rest):.1f}초 · 차이 {1 - med(rest)/med(first):+.1%}')
