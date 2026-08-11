#!/usr/bin/env python3
"""T6 대화형 저작 측정의 시계. 절차: corpus/tasks/t6-authoring-sheet.md §대화형(B)

  t6-clock.py present <n>                        n번째 개념을 요청한 시각을 찍는다
  t6-clock.py answer  <n> <개념명> <정의> [관계]   답 도착 시각을 찍고 경과를 계산한다
  t6-clock.py read <분>                           통독 시간을 따로 기록한다 (건당 단가에 합산 금지)
"""
import sys, time, os, json

LOG = 'corpus/tasks/t6-timing-log.tsv'
STAMP = '.t6-stamp.json'

def main():
    cmd = sys.argv[1]
    if cmd == 'read':
        with open(LOG.replace('timing-log.tsv', 'read-time.txt'), 'w') as f:
            f.write(f'출처 통독 시간: {sys.argv[2]}분 — 건당 단가에 합산하지 않는다\n')
        print(f'통독 {sys.argv[2]}분 기록 (측정 밖)')
        return
    n = int(sys.argv[2])
    now = time.time()
    if cmd == 'present':
        json.dump({'n': n, 't': now}, open(STAMP, 'w'))
        print(f'{n}/8 요청 시각 기록')
        return
    name, defn = sys.argv[3], sys.argv[4]
    rel = sys.argv[5] if len(sys.argv) > 5 else ''
    s = json.load(open(STAMP))
    assert s['n'] == n, f"요청한 건({s['n']})과 답({n})이 다르다"
    sec = round(now - s['t'], 1)
    new = not os.path.exists(LOG)
    with open(LOG, 'a') as f:
        if new:
            f.write('n\t소요초\t개념명\t정의\t관계\n')
        f.write(f'{n}\t{sec}\t{name}\t{defn}\t{rel}\n')
    print(f'{n}/8 기록 — {sec}초')

main()
