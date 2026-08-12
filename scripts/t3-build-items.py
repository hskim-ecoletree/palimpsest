#!/usr/bin/env python3
"""t3-items/ 를 고정 SHA 의 코드까지 박아서 생성한다 — **boxwood 없는 PC 에서 판정하기 위해서다.**

단건은 한 파일에 한 건, 일괄(B1·B2)은 한 파일에 8건을 묶는다. 묶어 제시하는 것이
그 장치의 형태이므로 제시 형태가 곧 처치다.
"""
import tomllib, subprocess, os

B = os.path.expanduser('~/dev/projects/boxwood')
PIN = {'portal-backend': ('portal-backend', 'a29cad0bf6a8'),
       'boxwood-packages': ('boxwood-packages', '2e9198716796')}
BEFORE, AFTER = 8, 16
OUT = 'corpus/tasks/t3-items'
os.makedirs(OUT, exist_ok=True)

cands = tomllib.load(open('corpus/tasks/t3-candidates.toml', 'rb'))['candidate']

def snippet(c):
    repo, sha = PIN[c['file'].split('/')[0]]
    rel = c['file'].split('/', 1)[1]
    src = subprocess.run(['git', '-C', os.path.join(B, repo), 'show', f'{sha}:{rel}'],
                         capture_output=True, text=True, check=True).stdout.split('\n')
    line = c['line']
    lo, hi = max(0, line - 1 - BEFORE), min(len(src), line + AFTER)
    body = []
    for i in range(lo, hi):
        mark = '▶' if i + 1 == line else ' '
        body.append(f'{i+1:5d} {mark} {src[i]}')
    return (f'**{c["file"]}:{line}** · 심볼 `{c["symbol"]}`\n\n'
            '```kotlin\n' + '\n'.join(body) + '\n```\n')

ASK = ('*`▶` 가 후보 줄이다. 앞뒤는 문맥이다.*\n\n'
       '**이것이 인가 가드인가 · 근거 좌표는 어디인가**\n'
       '(가드 예/아니오 + 근거 좌표(파일·심볼) 또는 "없음")\n')

singles = [c for c in cands if c['block'] not in ('B1', 'B2')]
for c in singles:
    n = c['seq']
    open(f'{OUT}/{n:02d}.md', 'w').write(
        f'# {n} / 42\n\n' + snippet(c) + '\n---\n\n' + ASK)

for blk, title in (('B1', '묶음 1'), ('B2', '묶음 2')):
    g = [c for c in cands if c['block'] == blk]
    parts = [f'# {title} — 8건을 한 번에 판정한다 (시퀀스 {g[0]["seq"]}–{g[-1]["seq"]} / 42)\n',
             '**개별 확인이 아니라 표본 확인이다.** 전부 열어 볼 필요는 없다 — '
             '몇 건을 보고 나머지를 함께 판정해도 된다. 그것이 이 장치의 형태다.\n',
             '판정은 **8건 전부에 대해 한 번에** 준다. 답과 함께 **실제로 열어 본 건수**를 적어 달라.\n',
             '\n---\n']
    for i, c in enumerate(g, 1):
        parts.append(f'\n## {i} / 8\n\n' + snippet(c))
    parts += ['\n---\n\n**8건 각각: 인가 가드인가 · 근거 좌표는 어디인가**\n',
              '형식 예 — `1 예 (파일:심볼) · 2 아니오 · 3 예 (…) · …` + `열어 본 건수: n`\n']
    open(f'{OUT}/{blk}.md', 'w').write('\n'.join(parts))

# F — 일괄 검산. 사전 지정(각 묶음 3번째)이라 지금 만들 수 있다
for k, blk in enumerate(('B1', 'B2'), 1):
    c = [x for x in cands if x['block'] == blk][2]
    open(f'{OUT}/F{k}.md', 'w').write(
        f'# F{k} — 단건 (시퀀스 41–42 / 42 중 하나)\n\n' + snippet(c) + '\n---\n\n' + ASK)

print('단건', len(singles), '· 묶음 2 · F 2 →', OUT)
