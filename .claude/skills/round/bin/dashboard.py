#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""회차 계기판 — 잔액을 수로 낸다.

**이 자리**: 규약 §5 루프의 「검증 → 수정 착수」 사이. 에이전트가 부른다.
사용자가 수시로 부르는 물건이 아니다.

**이 물건이 하는 일은 하나다: 당긴다.** 판정하지 않고, 읽기를 강요하지 않는다.
수가 이상하면 사람이 들여다보거나 설명을 요구한다 — 그 요구가 이 장치의 산출이다.

카테고리는 **enum** 이고 회고에서 도출됐다:
`.palimpsest/rounds/2026-08-18-completion-condition/retro/09-categories.md`

★ **⑦⑧ 은 원천이 다르다.** ①~⑥ 은 git 이 답하고 ⑦⑧ 은 **기록된 판정**이 답한다
(`findings.jsonl`). 그래서 규약 §5 의 「원천이 산문이면 못 더한다」를 **충족했다고
주장하지 않는다** — 형식이 기계 판독이 된 것이지 원천이 기계가 된 것이 아니다.

⚠ **호출은 `python3 <경로>` 다.** 설치본은 파일 모드를 0644 로 놓아 직접 실행이
안 되고, Windows 에서는 모드로도 안 풀린다(옛 ADR-0023).

    사용: python3 dashboard.py <착수커밋> [의도파일] [종료커밋=HEAD]
"""
import subprocess, sys, re, collections, os

# ★ **출력 인코딩을 못 박는다.** 이 스크립트의 출력은 전부 한국어이고, Windows 의
# 파이썬은 **비-tty stdout 에 로케일 인코딩(보통 cp1252)** 을 쓴다. 그러면 파이프로
# 받는 순간 `UnicodeEncodeError` 로 죽는다 — `xtask` 의 회차 레코드 검사와 설치본
# 시험이 정확히 그렇게 부른다(독립 리뷰 2026-08-19 · `PYTHONIOENCODING=cp1252` 로 재현).
# 옛 ADR-0023: 고를 축은 「볼 수 있는 쪽」이 아니라 **양쪽이 할 수 있는 것**이다.
for _스트림 in (sys.stdout, sys.stderr):
    try:
        _스트림.reconfigure(encoding="utf-8")
    except (AttributeError, ValueError):
        pass


자기_접두 = ('.palimpsest/', 'docs/gates/', '.claude/', 'xtask/', 'scripts/')

def sh(c):
    # ★ **입력 디코드도 못 박는다.** (2026-08-19 · 독립 리뷰 2 라운드)
    #   앞 판은 `sys.stdout.reconfigure` 로 **출력**만 막았는데, 죽는 자리는 여기였다 —
    #   `text=True` 는 로케일 인코딩으로 git 출력을 읽고, 이 저장소의 커밋 제목은
    #   한국어라 cp1252·cp949 에서 **디코드 불가**다. 그러면 Windows 에서 ⑦⑧ 이 영영
    #   안 뜬다. `errors="replace"` 를 함께 두는 까닭: 계기판이 죽으면 그 자리에서
    #   회차가 멈추므로, **못 읽은 글자 하나 때문에 수 전체를 잃지 않는다.**
    return subprocess.run(c, shell=True, capture_output=True, text=True,
                          encoding="utf-8", errors="replace").stdout

def 자기인가(p):
    return p.startswith(자기_접두)

def 라운드번호(h):
    subj = sh(f"git show -s --format=%s {h}").strip()
    # 라운드 번호는 커밋 제목의 `[R<n>]` 로 읽는다 — **규약 §5 「교대」가 그 표기를 진다.**
    #
    # ⚠ **제목 서사에서 번호를 캐내지 않는다.** 앞 회차의 커밋 제목은
    # 「독립 리뷰 N 라운드」 표기를 썼는데 **그 표기가 회차 안에서 갈렸다** —
    # 라운드 1(`a308602` *"독립 리뷰의 반증 하나와 발견 열넷"*)에는 번호가 없고,
    # 라운드 6(`a1553d7 feat(xtask)`)은 접두사부터 다르다. 그 둘을 놓치면
    # 연쇄 깊이가 **13 → 11** 로 틀린 수를 낸다(실측 2026-08-19 · 독립 리뷰 1 라운드가 잡았다).
    #
    # ★ **이것이 회고가 C1-b 에서 실측한 병이 장치 안에서 재발한 것이다** —
    # *"축은 커밋 접두사에 걸지 않는다. 표기 규약이 회차 안에서 태어나면 갈린다."*
    # 그래서 **서사가 아니라 기계 표기 하나만** 읽고, 없으면 `None` 을 낸다.
    # 없는 것을 추측으로 메우지 않는다.
    m = re.search(r'\[R(\d+)\]', subj)
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
    # 빈 범위에서 크래시하지 않는다. **못 잰 것은 못 잰다고 말한다** — 0% 라고 말하지 않는다.
    # (실측 2026-08-19: 잘못된 인자로 빈 범위가 들어와 `NoneType.__format__` 로 죽었다.
    #  계기판이 죽으면 그 자리에서 회차가 멈춘다.)
    if 전체 is None:
        print(f"① 자기 비율      — (범위 `{rng}` 에 만진 경로가 없다)")
        # ★ **⑦⑧ 은 범위와 무관하다** — 파일 전체를 잰다. 그러니 ① 이 못 재도 낸다.
        #   (실측 2026-08-19: 조기 return 이 ⑦⑧ 까지 죽였고, 설치본은 갓 설치한 상태에서
        #    언제나 빈 범위라 **그 둘이 영영 안 보였다** — 태어나면서 죽은 가지다.)
        발견칸(의도파일, set())
        print()
        print("⚠ 범위가 비었다 — 인자를 확인하라: python3 dashboard.py <착수커밋> [의도파일] [종료커밋]")
        print("  ⑦⑧ 은 커밋 범위를 안 쓰므로 위에 그대로 났다.")
        return
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
    # 라운드 표기가 붙은 커밋이 있으면 라운드로, 하나도 없으면 **커밋 수로** 낸다.
    # 어느 쪽으로 쟀는지 반드시 적는다 — 두 수는 다른 것이고 섞이면 거짓이 된다.
    if 라운드셋:
        연쇄 = sorted(((len(rs), p) for p, rs in 경로라운드.items() if len(rs) >= 2), reverse=True)
        단위 = "라운드"
        덧말 = ""
    else:
        연쇄 = sorted(((k, p) for p, k in 만짐.items() if k >= 2), reverse=True)
        단위 = "커밋"
        덧말 = "   ⚠ 라운드 표기(`[R<n>]`)가 없어 커밋 수로 쟀다"
    print(f"④ 연쇄 깊이      최대 {연쇄[0][0] if 연쇄 else 0} {단위}{덧말}")
    for d, p in 연쇄[:3]: print(f"                    {d} {단위}에 걸쳐  {p}")

    # ⑤ 라운드
    # ⚠ `len(라운드셋)` 은 **서로 다른 라운드 번호의 수**이지 커밋 수가 아니다.
    #   앞 판이 *"라운드가 붙은 커밋 N"* 이라 적어 거짓 신호를 냈다(독립 리뷰 2 라운드).
    표기달린커밋 = sum(1 for h in 커밋 if 라운드번호(h) is not None)
    print(f"⑤ 라운드         {최신라운드 if 최신라운드 else '—'}"
          f"  (서로 다른 라운드 {len(라운드셋)} · 표기 달린 커밋 {표기달린커밋})")

    # ⑥ 승격
    # ⑥ 승격 — 커밋 제목의 `[승격]` 표기. **규약 §5 「승격」이 그 표기를 진다.**
    # ⚠ 그 표기가 이 회차에서 태어났으므로 **앞 회차들에 대해서는 구조적으로 0** 이다.
    #   0 을 「승격이 없었다」로 읽으면 안 된다 — 표기가 없었을 뿐이다.
    승격 = len([h for h in 커밋 if '[승격]' in sh(f"git show -s --format=%s {h}")])
    표기있음 = any('[승격]' in sh(f"git show -s --format=%s {h}") or '[R' in sh(f"git show -s --format=%s {h}") for h in 커밋)
    print(f"⑥ 승격 횟수      {승격}"
          + ("" if 표기있음 else "   ⚠ 표기가 없는 회차 — 0 은 「없었다」가 아니라 「못 셌다」다"))

    # ⑦⑧ — **원천이 git 이 아니다.**
    발견칸(의도파일, 라운드셋)

    print()
    print("⚠ 이 계기판은 **지난 라운드들의 잔액**을 낸다.")
    print("  지금 착수할 수정이 진자를 만드는지는 원리상 못 말한다.")
    print("⚠ ①~⑥ 은 `<착수>..<종료>` **커밋 범위**를 재고, ⑦⑧ 은 **레코드 파일 전체**를 잰다.")
    print("  같은 회차의 두 수지만 **같은 범위가 아니다**.")


def 발견칸(의도파일, 라운드셋):
    """⑦ 원 의도 비율 · ⑧ 발견 유효성 — `findings.jsonl` 이 원천이다.

    ★ **레코드가 없으면 「못 셌다」다. 0 이라고 말하지 않는다** — ⑥ 이 표기 없는 회차에
    대해 그렇게 하는 것과 같은 자다.
    """
    import json
    if not 의도파일:
        print("⑦ 원 의도 비율    — (의도 파일 없음 → 레코드 자리를 못 찾는다)")
        print("⑧ 발견 유효성    — (같음)")
        return
    path = os.path.join(os.path.dirname(os.path.abspath(의도파일)), 'findings.jsonl')
    if not os.path.exists(path):
        print("⑦ 원 의도 비율    — **못 셌다** (레코드가 없다. 0 이 아니다)")
        print("⑧ 발견 유효성    — **못 셌다**")
        return
    행 = []
    for i, line in enumerate(io_open(path)):
        line = line.strip()
        if not line:
            continue
        o = json.loads(line)
        if i == 0 and 'schema_version' in o:
            continue
        행.append(o)
    if not 행:
        print("⑦ 원 의도 비율    — **못 셌다** (레코드가 비었다)")
        print("⑧ 발견 유효성    — **못 셌다**")
        return

    n = len(행)
    원의도 = sum(1 for o in 행 if o.get('모집단') == '원의도')
    print(f"⑦ 원 의도 비율   {원의도 * 100 // n:>3}%  ({원의도}/{n} 발견)")
    분포 = collections.Counter(o.get('모집단') for o in 행)
    print("                  " + " · ".join(f"{k} {v}" for k, v in 분포.most_common()))

    유효 = collections.Counter(o.get('유효성') for o in 행)
    참, 거짓 = 유효.get('참', 0), 유효.get('거짓', 0)
    잰것 = 참 + 거짓
    if 잰것:
        print(f"⑧ 발견 유효성    참 {참} · 거짓 {거짓}  →  {거짓 * 100 // 잰것}% 가 헛것")
    else:
        print("⑧ 발견 유효성    — **못 셌다** (참·거짓이 하나도 없다)")
    if 유효.get('추정'):
        print(f"                  추정 {유효['추정']} 은 분모에서 뺐다 — 아직 안 갈렸다")

    # ★ **라운드는 출처 안에서의 셈이다.** (정정 2026-08-19 · 독립 리뷰가 잡았다)
    #   앞 판은 레코드의 `라운드` 를 커밋 태그 `[R<n>]` 과 **댔는데 전제가 틀렸다** —
    #   사전부검 R1~R3 · 독립 리뷰 R1~R5 · 커밋 R1~R7 은 **서로 다른 셈**이다.
    #   사전부검 R1 이 커밋 R1 과 우연히 겹쳐서 그 틀림이 안 보였다.
    #   그러니 대지 않고 **출처별로 그대로 보인다** — 대조가 아니라 분포다.
    출처별 = collections.defaultdict(set)
    for o in 행:
        if o.get('라운드') is not None:
            출처별[o.get('출처')].add(o['라운드'])
    if 출처별:
        print("                  라운드 " + " · ".join(
            f"{k} R{min(v)}~R{max(v)}" if len(v) > 1 else f"{k} R{min(v)}"
            for k, v in sorted(출처별.items())))
        print(f"                  (커밋 태그는 {sorted(라운드셋) if 라운드셋 else '—'} — "
              "**다른 셈이다. 대지 않는다**)")

    print("                  ★ ⑦⑧ 의 원천은 **기록된 판정**이지 git 이 아니다.")


def io_open(p):
    import io
    return io.open(p, encoding='utf-8')

if __name__ == '__main__':
    main(sys.argv[1],
         sys.argv[2] if len(sys.argv) > 2 else None,
         sys.argv[3] if len(sys.argv) > 3 else 'HEAD')
