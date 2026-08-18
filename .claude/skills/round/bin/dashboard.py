#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""회차 계기판 — 잔액을 수로 낸다.

**이 자리**: 규약 §5 루프의 「검증 → 수정 착수」 사이. 에이전트가 부른다.
사용자가 수시로 부르는 물건이 아니다.

**이 물건이 하는 일은 하나다: 당긴다.** 판정하지 않고, 읽기를 강요하지 않는다.
수가 이상하면 사람이 들여다보거나 설명을 요구한다 — 그 요구가 이 장치의 산출이다.

카테고리 여섯은 **enum** 이고 회고에서 도출됐다:
`.palimpsest/rounds/2026-08-18-completion-condition/retro/09-categories.md`

    사용: dashboard.py <착수커밋> [의도파일] [종료커밋=HEAD]
"""
import subprocess, sys, re, collections, os

자기_접두 = ('.palimpsest/', 'docs/gates/', '.claude/', 'xtask/', 'scripts/')

def sh(c):
    return subprocess.run(c, shell=True, capture_output=True, text=True).stdout

def 자기인가(p):
    return p.startswith(자기_접두)

def 라운드번호(h):
    subj = sh(f"git show -s --format=%s {h}").strip()
    # 규율 C0-b: 라운드 번호는 커밋 제목에 `[R<n>]` 로 남긴다.
    # 옛 회차는 「독립 리뷰 N 라운드」 표기를 쓴다 — 그것만 받는다.
    # (「사전부검 3 라운드」 같은 다른 라운드를 오독하지 않기 위해서다.)
    m = re.search(r'\[R(\d+)\]', subj) or re.search(r'독립 리뷰\s*(\d+)\s*라운드', subj)
    return int(m.group(1)) if m else None

def main(착수, 의도파일=None, 종료="HEAD"):
    rng = f"{착수}..{종료}"
    커밋 = [h for h in sh(f"git log --format=%h {rng}").split() if h]

    # 경로별 만진 횟수 · 라운드
    만짐 = collections.Counter()
    경로라운드 = collections.defaultdict(set)
    라운드셋 = set()
    이번라운드경로 = []
    최신라운드 = None
    for h in 커밋:
        r = 라운드번호(h)
        if r is not None:
            라운드셋.add(r)
            if 최신라운드 is None or r > 최신라운드:
                최신라운드 = r
        ps = [x.strip() for x in sh(f"git show --name-only --format= {h}").split('\n') if x.strip()]
        for p in ps:
            만짐[p] += 1
            if r is not None:
                경로라운드[p].add(r)
        if r is not None and r == 최신라운드:
            이번라운드경로 += ps

    print(f"── 회차 계기판 ── {rng} · 커밋 {len(커밋)}")
    print()

    # ① 자기 비율
    def 비율(ps):
        if not ps: return None, 0, 0
        s = sum(1 for p in set(ps) if 자기인가(p))
        n = len(set(ps))
        return 100.0*s/n, s, n
    전체, s_a, n_a = 비율(list(만짐))
    이번, s_b, n_b = 비율(이번라운드경로)
    print(f"① 자기 비율      회차 전체 {전체:>3.0f}%  ({s_a}/{n_a} 경로)")
    if 이번 is not None and 이번라운드경로:
        print(f"                  이번 라운드 {이번:>3.0f}%  ({s_b}/{n_b})"
              + ("   ← 저장소를 안 만졌다" if 이번 >= 99 else ""))

    # ② 미판정 잔액
    if 의도파일 and os.path.exists(의도파일):
        t = open(의도파일, encoding='utf-8').read()
        열림 = len(re.findall(r'^- \[ \]', t, re.M))
        닫힘 = len(re.findall(r'^- \[[xX]\]', t, re.M))
        print(f"② 미판정 잔액    {열림} / {열림+닫힘}")
    else:
        print("② 미판정 잔액    — (의도 파일 없음)")

    # ③ 진자 P1
    진자 = [p for p, k in 만짐.items() if k >= 2 and not sh(f"git diff {착수} {종료} -- '{p}'").strip()]
    print(f"③ 진자 (P1)      {len(진자)}"
          + ("   ← 고쳤다 되돌린 자리" if 진자 else ""))
    for p in sorted(진자): print(f"                    {만짐[p]}회  {p}")
    if 진자: print("                  ⚠ 재현율 1/3 — 순변경 있는 부분 진자와 토큰 왕복은 못 잡는다")

    # ④ 연쇄 깊이
    연쇄 = sorted(((len(rs), p) for p, rs in 경로라운드.items() if len(rs) >= 2), reverse=True)
    print(f"④ 연쇄 깊이      최대 {연쇄[0][0] if 연쇄 else 0} 라운드")
    for d, p in 연쇄[:3]: print(f"                    {d} 라운드에 걸쳐  {p}")

    # ⑤ 라운드
    print(f"⑤ 라운드         {최신라운드 if 최신라운드 else '—'} (라운드가 붙은 커밋 {len(라운드셋)})")

    # ⑥ 승격
    # 규율: 승격 커밋은 제목에 `[승격]` 을 단다.
    승격 = len([h for h in 커밋 if '[승격]' in sh(f"git show -s --format=%s {h}")])
    print(f"⑥ 승격 횟수      {승격}")

    print()
    print("⚠ 이 계기판은 **지난 라운드들의 잔액**을 낸다.")
    print("  지금 착수할 수정이 진자를 만드는지는 원리상 못 말한다.")

if __name__ == '__main__':
    main(sys.argv[1],
         sys.argv[2] if len(sys.argv) > 2 else None,
         sys.argv[3] if len(sys.argv) > 3 else 'HEAD')
