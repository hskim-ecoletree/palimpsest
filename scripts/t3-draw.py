#!/usr/bin/env python3
"""T3 후보 추첨 — 등록된 규칙(corpus/tasks/t3-candidates.toml `[selection]`)의 집행.

규칙은 이 스크립트보다 먼저 커밋됐다(90461f0). 여기서 하는 일은 그 규칙을 돌리는 것뿐이며
어떤 수도 결과를 보고 정하지 않는다. 산출: corpus/tasks/t3-candidates.toml 에 [[candidate]] 추가.
"""
import subprocess, os, re, tomllib, collections

B = os.path.expanduser('~/dev/projects/boxwood')
PIN = [('portal-backend', 'a29cad0bf6a8'), ('boxwood-packages', '2e9198716796')]
PAT = {'①': r'fun\s+(check|validate|assert|require|ensure|verify)[A-Za-z0-9_]*',
       '②': r'class\s+([A-Za-z0-9_]+)[^{]*:[^{]*(Interceptor|Filter)\b',
       '③': r'@([A-Za-z]*(Permission|Auth|Secure|Audit|Tenant)[A-Za-z]*)'}

def scan():
    hits = {k: [] for k in PAT}
    for repo, sha in PIN:
        names = subprocess.run(['git', '-C', os.path.join(B, repo), 'ls-tree', '-r', '--name-only', sha],
                               capture_output=True, text=True, check=True).stdout.split('\n')
        for p in names:
            if not p.endswith('.kt') or '/src/main/' not in ('/' + p):
                continue
            full = subprocess.run(['git', '-C', os.path.join(B, repo), 'show', f'{sha}:{p}'],
                                  capture_output=True, text=True, check=True).stdout
            for k, pat in PAT.items():
                for m in re.finditer(pat, full):
                    hits[k].append((f'{repo}/{p}', full[:m.start()].count('\n') + 1, m.group(1)))
    return hits

def spaced(seq, draw):
    """등간격 — g = floor(len/draw), 인덱스 0·g·2g·… (T2·T7 과 동형)"""
    g = len(seq) // draw
    return [seq[i * g] for i in range(draw)]

hits = scan()
assert (len(hits['①']), len(hits['②']), len(hits['③'])) == (89, 10, 596), '풀이 등록값과 다르다'

used = {(c['file'], c['line']) for c in tomllib.load(open('corpus/tasks/label-candidates.toml', 'rb'))['candidate']}
rest = {k: [h for h in hits[k] if (h[0], h[1]) not in used] for k in ('①', '③')}

# ── D 먼저 (등록 규칙: D 확정 → 그 좌표 제외 → 층별 등간격) ──
per = collections.defaultdict(list)
for k in ('①', '③'):
    for f, l, s in rest[k]:
        per[f].append((l, s, k))
ge4 = sorted([f for f, v in per.items() if len(v) >= 4])
assert len(ge4) == 54, f'4건 이상 파일 수가 등록값(54)과 다르다: {len(ge4)}'
d_files = spaced(ge4, 2)
D = []
for f in d_files:
    for l, s, k in sorted(per[f])[:4]:
        D.append((k, f, l, s))
d_coords = {(f, l) for _, f, l, _ in D}

# ── 층별 등간격 15건씩 ──
picks = {}
for k in ('①', '③'):
    pool = [h for h in rest[k] if (h[0], h[1]) not in d_coords]
    picks[k] = [(k,) + h for h in spaced(pool, 15)]

# ── 배분: 뽑힌 순서대로 A1 2 · A2 2 · C 3 · B 8 ──
alloc = {}
for k in ('①', '③'):
    p = picks[k]
    alloc[k] = {'A1': p[0:2], 'A2': p[2:4], 'C': p[4:7], 'B': p[7:15]}

# ── 시퀀스: A1(③2→①2) → D(8) → B(묶음1=③8, 묶음2=①8) → C(③3→①3) → A2(③2→①2) ──
seq = []
def add(block, items, note=''):
    for it in items:
        seq.append({'block': block, 'stratum': it[0], 'file': it[1], 'line': it[2], 'symbol': it[3], 'note': note})
add('A1', alloc['③']['A1'] + alloc['①']['A1'])
add('D', D)
add('B1', alloc['③']['B'])
add('B2', alloc['①']['B'])
add('C', alloc['③']['C'] + alloc['①']['C'])
add('A2', alloc['③']['A2'] + alloc['①']['A2'])

out = ['\n# ═══════════════════════════════════════════════════════════════════════════',
       '# 추첨 결과 — 위 [selection] 규칙의 집행. scripts/t3-draw.py 가 생성했다',
       '# **이 커밋 시각이 측정 시작보다 앞선다.**',
       '# ═══════════════════════════════════════════════════════════════════════════\n',
       '[draw]', 'executed_at = "2026-08-12"', f'unique = {len(seq)}',
       'sequence = 42   # 새 고유 38 + 재제시 4(E 2 · F 2). E·F 는 실행 중에 정해진다',
       'f_designated = "B1 의 3번째 · B2 의 3번째 — 결과와 무관하게 지금 지정된다"\n']
for i, c in enumerate(seq, 1):
    out += ['[[candidate]]', f'seq = {i}', f'block = "{c["block"]}"', f'stratum = "{c["stratum"]}"',
            f'file = "{c["file"]}"', f'line = {c["line"]}', f'symbol = "{c["symbol"]}"', '']
open('corpus/tasks/t3-candidates.toml', 'a').write('\n'.join(out))
print(f'{len(seq)}건 기록 — 블록별', collections.Counter(c['block'] for c in seq))
print('F 검산 대상:', [ (c['file'].split('/')[-1], c['line']) for c in seq if c['block'] in ('B1','B2') ][2::8])
