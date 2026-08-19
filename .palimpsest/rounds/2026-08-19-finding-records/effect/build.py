#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""`## 효과` 를 **돌려서** 만든다 — 손으로 베끼지 않는다.

    사용: python3 build.py <회차디렉터리> > effect.md

★ 이 스크립트가 생긴 까닭: 이 회차가 효과의 수를 **세 번 손으로 적었고 세 번 다
커밋 시점에 이미 거짓**이었다(독립 리뷰 3 라운드 F5·F7·F10). 그중 하나는 **게이트**에
적혔다 — 가장 나쁜 자리다. 이 저장소의 규율이 그것을 이미 이름 붙였다:
**「세는 자리는 하나다. 베끼지 말고 돌려라.」**
"""
import sys, os, json, glob, subprocess

# ★ **출력 인코딩을 못 박는다.** `record.py`·`dashboard.py` 는 박는데 여기만 안 박아
# Windows 파이프에서 죽었다(독립 리뷰 5 라운드 S4). 게이트가 「세는 자리」로 가리키는
# 스크립트가 그 자리에서 죽으면 효과가 통째로 없어진다.
for _스트림 in (sys.stdout, sys.stderr):
    try:
        _스트림.reconfigure(encoding="utf-8")
    except (AttributeError, ValueError):
        pass

DITTO = os.path.expanduser("~/dev/projects/ditto")


def ditto_계수():
    """대조군을 센다. **admissible 이 없는 건도 세고 그 사실을 적는다.**

    ⚠ 앞 판은 `admissible` 이 있는 것만 세어 **총 83** 이라 적었는데 실제 objections 는
    **90** 이고 한 파일(7 건)이 그 필드를 안 가져 **말없이 빠졌다.** 「총 83」이라는
    표현이 그 배제를 감췄다(독립 리뷰 3 라운드 F10).
    """
    총 = 참 = 거짓 = 무판정 = 0
    파일 = 0
    for f in sorted(glob.glob(f"{DITTO}/reports/**/dialectic-*.json", recursive=True)):
        try:
            d = json.load(open(f, encoding="utf-8"))
        except Exception:
            continue
        objs = d.get("objections") or []
        if objs:
            파일 += 1
        for o in objs:
            총 += 1
            v = o.get("admissible")
            if v is True:
                참 += 1
            elif v is False:
                거짓 += 1
            else:
                무판정 += 1
    if 총 == 0:
        # ★ **대조군이 없으면 없다고 말한다.** 앞 판은 0 으로 나누며 죽었다 —
        #   이 저장소 밖의 경로에 기대므로 다른 기계에서는 정상적으로 없다.
        return None
    return dict(총=총, 참=참, 거짓=거짓, 무판정=무판정, 파일=파일)


def 우리_계수(회차):
    행 = []
    p = os.path.join(회차, "findings.jsonl")
    for i, line in enumerate(open(p, encoding="utf-8")):
        line = line.strip()
        if not line:
            continue
        o = json.loads(line)
        if i == 0 and "schema_version" in o:
            continue
        행.append(o)
    참 = sum(1 for o in 행 if o.get("유효성") == "참")
    거짓 = sum(1 for o in 행 if o.get("유효성") == "거짓")
    추정 = sum(1 for o in 행 if o.get("유효성") == "추정")
    return dict(총=len(행), 참=참, 거짓=거짓, 추정=추정)


def sh(*args):
    """돌리고 **rc 와 stderr 를 버리지 않는다.**

    ★ 이 회차가 `xtask` 에 대해 막 고친 결함(rc≠0 인데 아무 말 없으면 초록)이 **효과를
    만드는 자리에 그대로 있었다**(독립 리뷰 4 라운드 F1). 도구가 없거나 죽으면 빈
    코드블록을 내고 rc=0 으로 끝나는데, 게이트가 *"세는 자리는 하나다 — 돌려라"* 며
    가리키는 것이 이 스크립트다. **못 돌린 것을 조용히 빈칸으로 두면 그것이 거짓이다.**
    """
    r = subprocess.run(args, capture_output=True, text=True,
                       encoding="utf-8", errors="replace")
    out = (r.stdout or "").rstrip()
    if r.returncode != 0 or not out:
        err = (r.stderr or "").strip() or "(아무 말도 안 했다)"
        raise SystemExit(
            f"✗ 못 돌렸다 — rc={r.returncode}\n  명령: {' '.join(args)}\n  stderr: {err}\n"
            f"  ⚠ 효과를 빈칸으로 내지 않는다. 고치고 다시 돌려라."
        )
    return out


def main(회차):
    뿌리 = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(회차))))
    rec = os.path.join(뿌리, ".claude/skills/round/bin/record.py")
    dash = os.path.join(뿌리, ".claude/skills/round/bin/dashboard.py")
    우리 = 우리_계수(회차)
    d = ditto_계수()
    o = print
    o("## 효과 — 테스트가 아닌 것이 돌린 출력 (§8)")
    o("")
    o("> ⚠ **이 파일은 손으로 안 쓴다.** `python3 effect/build.py <회차> > effect/effect.md`")
    o("> 가 만든다 — 이 회차가 효과의 수를 세 번 손으로 적었고 **세 번 다 커밋 시점에**")
    o("> **이미 거짓**이었다. 세는 자리는 하나다.")
    o("")
    o("**물음**: *에이전트가 낸 발견 중 몇 %가 헛것인가?* — 앞 회차가 축 1 로 재려다")
    o("**참 109 · 거짓 0** 으로 반증된 그 물음이다.")
    o("")
    o(f"| | 총 | 참 | 거짓 | 헛것 |")
    o(f"|---|--:|--:|--:|--:|")
    잰것 = 우리["참"] + 우리["거짓"]
    o(f"| **palimpsest 회차 D** | {우리['총']} | {우리['참']} | {우리['거짓']} | "
      f"**{우리['거짓'] * 100 // 잰것}%** |")
    if d is None:
        o(f"| **ditto (대조군)** | — | — | — | **못 쟀다** |")
        d잰것 = 0
    else:
        d잰것 = d["참"] + d["거짓"]
        o(f"| **ditto (대조군)** | {d['총']} | {d['참']} | {d['거짓']} | "
          f"**{d['거짓'] * 100 // d잰것}%** |")
    o("")
    if 우리["추정"]:
        o(f"⚠ palimpsest 쪽 `추정` **{우리['추정']}** 건은 분모에서 뺐다 — 아직 안 갈렸다.")
    if d is None:
        o(f"⚠ 대조군을 **못 쟀다** — `{DITTO}` 에 `objections` 를 담은 파일이 없다."
          " 이 저장소 밖의 경로라 **다른 기계에서는 정상적으로 없다.**")
    elif d["무판정"]:
        o(f"⚠ ditto 쪽 **{d['무판정']}** 건은 `admissible` 필드가 없어 분모에서 빠졌다"
          f"(파일 {d['파일']} 개 중 하나가 그 필드를 안 쓴다). **총 {d['총']} 중 "
          f"{d잰것} 만 판정됐다** — 「총 {d잰것}」이라 적으면 그 배제가 감춰진다.")
    o("")
    o("⚠ **두 수는 같은 자로 잰 것이 아니다.** ditto 의 `admissible=false` 는 **심판이")
    o("각하한 것**이고 이쪽의 `유효성=거짓` 은 **발견자가 스스로 물린 것**이다.")
    o("같은 축에 놓았지만 같은 자가 아니다.")
    o("")
    o("### ① `record.py count`")
    o("```")
    o(sh(sys.executable, rec, "count", 회차))
    o("```")
    o("")
    o("### ② 계기판")
    o("```")
    o(sh(sys.executable, dash, "47a6770", os.path.join(회차, "intent.md")))
    o("```")
    o("")
    o("### ③ 대조군 상세")
    o("")
    o("전문은 [ditto-control.md](ditto-control.md).")


if __name__ == "__main__":
    main(sys.argv[1])
