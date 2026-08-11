#!/usr/bin/env python3
"""T2 정답 키 검증 — **판정을 마친 뒤에 실행한다.**

씨앗에서 중복 3건의 정체와 자리를 다시 유도해 `[sequence].order` 33건을 재구성한다.
일치하면 에이전트가 추첨 후에 자리를 옮기지 않았다는 뜻이다.
"""
import tomllib, hashlib, sys

c = tomllib.load(open('corpus/tasks/label-candidates.toml', 'rb'))
k = tomllib.load(open('corpus/tasks/label-answer-key.toml', 'rb'))['key']
U = [(x['file'], x['line'], x['symbol']) for x in c['candidate']]

h = lambda i: int(hashlib.sha256(f"{k['seed']}|{i}".encode()).hexdigest(), 16)

D, i = [], 0
while len(D) < 3:
    v = h(i) % 25          # d <= 24 — 33칸에서 `원본+5` 가 존재하려면
    if v not in D:
        D.append(v)
    i += 1
assert D == k['targets_in_draw_order'], '중복 대상이 씨앗에서 재현되지 않는다'

seq = [None] * 33
for d, pos in k['duplicate_pairs']:
    seq[pos - 1] = d
it = iter(range(30))
seq = [x if x is not None else next(it) for x in seq]

rebuilt = [f'{f}:{l} {s}' for f, l, s in (U[d] for d in seq)]
assert rebuilt == c['sequence']['order'], '시퀀스가 재구성되지 않는다'

orig = {}
for j, d in enumerate(seq):
    orig.setdefault(d, j + 1)
P = [pos for _, pos in k['duplicate_pairs']]
assert all(pos - orig[d] >= 5 for d, pos in k['duplicate_pairs']), '간격 5 위반'
assert all(p >= 8 for p in P), '자리 8 미만'
assert all(abs(a - b) >= 2 for a in P for b in P if a != b), '자리 인접'

print('재구성 일치 — 자리는 씨앗이 정한 그대로다')
print('중복 자리(시퀀스 번호):', sorted(P))
print('재질문율 = 그 세 자리에서 ○ 를 적은 수 / 3')
