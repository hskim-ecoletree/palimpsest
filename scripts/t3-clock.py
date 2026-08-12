#!/usr/bin/env python3
"""T3 측정의 시계. 절차: corpus/tasks/t3-protocol.md §다른 PC에서 실행하는 절차

  t3-clock.py present <id>                     제시 시각을 찍는다. id = 1..38 · B1 · B2 · E1 · E2 · F1 · F2
  t3-clock.py answer  <id> "<판정>" ["<메모>"]   답 도착 시각을 찍고 경과를 기록한다
  t3-clock.py note    "<한 줄>"                 사건을 원자료에 남긴다 (중단·재질문 등)

**id 는 문자열이다.** 묶음(B1·B2)의 소요는 8건 총시간이며 건당 값은 t3-report.py 가 나눈다.
"""
import sys, time, os, json

LOG = 'corpus/tasks/t3-timing-log.tsv'
STAMP = '.t3-stamp.json'
BLOCK = {**{str(i): b for i, b in
            list(zip(range(1, 5), ['A1'] * 4)) + list(zip(range(5, 13), ['D'] * 8))
            + list(zip(range(13, 21), ['B1'] * 8)) + list(zip(range(21, 29), ['B2'] * 8))
            + list(zip(range(29, 35), ['C'] * 6)) + list(zip(range(35, 39), ['A2'] * 4))},
         'B1': 'B1', 'B2': 'B2', 'E1': 'E', 'E2': 'E', 'F1': 'F', 'F2': 'F'}

def write(row):
    new = not os.path.exists(LOG)
    with open(LOG, 'a') as f:
        if new:
            f.write('id\t블록\t소요초\t판정\t메모\n')
        f.write('\t'.join(row) + '\n')

cmd = sys.argv[1]
if cmd == 'note':
    write(['—', '—', '—', '—', sys.argv[2]])
    print('기록')
elif cmd == 'present':
    i = sys.argv[2]
    assert i in BLOCK, f'모르는 id: {i}'
    json.dump({'id': i, 't': time.time()}, open(STAMP, 'w'))
    print(f'{i} ({BLOCK[i]}) 제시 시각 기록')
else:
    i, verdict = sys.argv[2], sys.argv[3]
    memo = sys.argv[4] if len(sys.argv) > 4 else ''
    s = json.load(open(STAMP))
    assert s['id'] == i, f"제시한 것({s['id']})과 답({i})이 다르다"
    sec = round(time.time() - s['t'], 1)
    write([i, BLOCK[i], f'{sec}', verdict, memo])
    per = f' (건당 {sec/8:.1f}초)' if i in ('B1', 'B2') else ''
    print(f'{i} 기록 — {sec}초{per}')
