#!/usr/bin/env python3
"""t2-items/NN.md 를 고정 SHA의 코드까지 박아서 생성한다.

좌표만 주면 판정자가 코퍼스를 직접 열어야 한다 — 그것은 R-23 이 "도구가 흡수한다"고
적은 성분을 판정자에게 떠넘기는 것이다. 코드를 함께 제시하는 쪽이 설계에 충실하다.
"""
import tomllib, subprocess, os

B = os.path.expanduser('~/dev/projects/boxwood')
PIN = {'portal-backend': ('portal-backend', 'a29cad0bf6a8'),
       'boxwood-packages': ('boxwood-packages', '2e9198716796')}
BEFORE, AFTER = 8, 16

c = tomllib.load(open('corpus/tasks/label-candidates.toml', 'rb'))
order = c['sequence']['order']

for n, o in enumerate(order, 1):
    loc, sym = o.rsplit(' ', 1)
    path, line = loc.rsplit(':', 1)
    line = int(line)
    repo, sha = PIN[path.split('/')[0]]
    rel = path.split('/', 1)[1]
    src = subprocess.run(['git', '-C', os.path.join(B, repo), 'show', f'{sha}:{rel}'],
                         capture_output=True, text=True, check=True).stdout.split('\n')
    lo, hi = max(0, line - 1 - BEFORE), min(len(src), line + AFTER)
    snippet = '\n'.join(('%5d %s %s' % (i + 1, '▶' if i + 1 == line else ' ', src[i])).rstrip()
                        for i in range(lo, hi))
    nxt = ('다음 → [`%02d.md`](%02d.md)' % (n + 1, n + 1) if n < 33
           else '**마지막이다.** 판정표를 저장하고 `docs/gates/preflight.md` §T2 에 결과를 적는다.')
    open('corpus/tasks/t2-items/%02d.md' % n, 'w').write(f'''# {n} / 33

**{path}:{line}** · 심볼 `{sym}`

```kotlin
{snippet}
```

*`▶` 가 후보 줄이다. 앞뒤는 문맥이다. 더 봐야 하면 코퍼스를 열어도 된다 —
`git -C ~/dev/projects/boxwood/{repo} show {sha}:{rel}`*

---

1. **무엇인가?**
   · `가드(인가)` — 권한 보유 여부를 검사해 없으면 거부한다 (403)
   · `가드(인증)` — 신원·세션·토큰을 확립하거나 검증한다. 실패하면 거부한다 (401)
   · `아니오` — 둘 다 아니다 (형식 검증 · 로깅 · 트랜잭션 · 성능 …)
   · `모름` — 결론을 못 내리겠다. **이것도 1급 답이다**
2. **근거 좌표는?**  실제로 그 판정이 내려지는 파일·심볼, 또는 `없음`

답과 소요 초를 [`../label-judgment-sheet.md`](../label-judgment-sheet.md) 의 **{n}행**에 적는다.
앞 건으로 돌아가지 않는다.

{nxt}
''')
print('t2-items/01..33.md 재생성 — 코드 포함')
