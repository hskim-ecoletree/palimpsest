#!/usr/bin/env python3
"""T2 대화형 측정의 시계. 프로토콜: corpus/tasks/t2-protocol.md

  t2-clock.py present <n>                       그 건을 제시한 시각을 찍는다
  t2-clock.py answer  <n> <가드> <좌표> [flags]  답 도착 시각을 찍고 경과를 계산한다
                                                 flags: 다시 / 중단 (쉼표 구분)
"""
import sys, time, os, json

LOG = 'corpus/tasks/t2-timing-log.tsv'
STAMP = '.t2-stamp.json'

def main():
    cmd, n = sys.argv[1], int(sys.argv[2])
    now = time.time()
    if cmd == 'present':
        json.dump({'n': n, 't': now}, open(STAMP, 'w'))
        print(f'{n}/33 제시 시각 기록')
        return
    guard, coord = sys.argv[3], sys.argv[4]
    flags = sys.argv[5] if len(sys.argv) > 5 else ''
    s = json.load(open(STAMP))
    assert s['n'] == n, f"제시된 건({s['n']})과 답({n})이 다르다"
    sec = round(now - s['t'], 1)
    new = not os.path.exists(LOG)
    with open(LOG, 'a') as f:
        if new:
            f.write('n\t소요초\t가드\t근거좌표\t표시\n')
        f.write(f'{n}\t{sec}\t{guard}\t{coord}\t{flags}\n')
    print(f'{n}/33 기록 — {sec}초')

main()
