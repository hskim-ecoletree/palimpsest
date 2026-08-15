#!/usr/bin/env python3
"""F10-6 대조 — **「다음에 오는 선언」을 「붙어 있음」으로 만든다** (#62 · #63).

합격선은 `corpus/criteria.toml` `[f10.6]` 에 있고 **첫 코드 커밋보다 먼저, 별도
커밋으로** 등록됐다. ⚠ **그러나 `registered_before_any_measurement = false` 다** —
[#62] 의 진단을 이미 본 뒤에 등록됐고, 그 사실과 줄이는 방법 다섯이
`[f10.6].measurement_already_seen` 에 있다.

    ① 거짓 결박률 **≤ 5%** (표본 50) — `[f10.pass]` 에서 옮긴 값
    ② 결박됨 **≥ 50** — ①의 짝이자 ★ **재현율의 하한**. 23 에서 넘으려면 래퍼를 벗기는
      수밖에 없다
    ③ `attached` 표본 **≥ 20**
    ④ ★ **안 붙여야 할 것을 계속 안 붙이는가** — 빈 줄로 갈린 것 · 붙을 선언이 없는 것.
      **결박 0 · 모집단 각 ≥ 1**. ⚠ **소스에서 줄로 센다** — 산출로 세면 순환한다
    ⑤ 골든 넷 — **이 스크립트가 아니라 기준선이 진다**(`f03-3` ③ · `f01` · `s0-compare`).
      ⚠ **이번엔 `pal-extract` 를 고치므로 앞 회차들과 달리 「당연히 안 움직인다」가 아니다**
    ⑥ ★ **Kotlin** — 실 코퍼스에 표식 주석이 **0 건**이라 **대조 불가**다. 픽스처로 세운다
    ⑦ **F09 ⑤ 재측정** ([#63]) — `f10-verify.py` 의 ⑨ 를 **그대로 부른다.**
      복제하면 그것이 곧 두 곳에 적힌 같은 규칙이다
    ⑧ 후보 여럿의 모집단 — ★ **열아홉째를 이 절이 만들 수 있다**

**제외 규칙·코퍼스 핀·인입 절차는 `f10-verify.py` 에서 빌려 쓴다.**

사용:
    ./scripts/f10-6-verify.py
"""

from __future__ import annotations

import importlib.util
import re
import shutil
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent

_spec = importlib.util.spec_from_file_location("f10_verify", ROOT / "scripts/f10-verify.py")
F10 = importlib.util.module_from_spec(_spec)
assert _spec.loader is not None
_spec.loader.exec_module(F10)

# ── `[f10.6.pass]` 가 등록한 값들. **여기서 정하지 않는다 — 옮겨 적을 뿐이다** ────────
FALSE_BINDING_MAX_PCT = 5          # `[f10.pass]` → `[f10.5.pass]` → 여기
FALSE_BINDING_SAMPLE = 50          # 같은 자리
BOUND_COUNT_MIN = 50               # 같은 자리. ★ 이 절에서는 **재현율의 하한**도 진다
ATTACHED_SAMPLE_MIN = 20           # `[f10.4.pass].f09_remeasure_sample_min` 에서
MARKED_COMMENT_POPULATION_MIN = 1  # ★ 0 이면 「미결박이 줄었다」가 산술로 공짜가 된다
NEGATIVE_BOUND_MAX = 0             # 빈 줄로 갈린 것 · 붙을 선언 없는 것
NEGATIVE_POPULATION_MIN = 1        # ★ 없으면 「그런 주석이 없어서 0」이 만점을 받는다
KOTLIN_FIXTURE_BOUND_MIN = 1
KOTLIN_FIXTURE_UNBOUND_MIN = 1
CANDIDATES_POPULATION_MIN = 1

ZERO_DISTANCE = {"attached", "frontmatter"}

결과 = F10.결과
ok, fail, skip = F10.ok, F10.fail, F10.skip
run, pal, 사본, 인입, 걸러낸다, 분류, 신호 = (
    F10.run, F10.pal, F10.사본, F10.인입, F10.걸러낸다, F10.분류, F10.신호
)
DITTO, DITTO_PIN, PORTAL, PORTAL_PIN = F10.DITTO, F10.DITTO_PIN, F10.PORTAL, F10.PORTAL_PIN

표식주석앵커 = re.compile(r"^L\d+$")


# ═════════════════════════════════════════════════════════════════════════════
# ①②③ — 분포 · 결박 하한(재현율) · attached 표본
# ═════════════════════════════════════════════════════════════════════════════

def 해소(tmp: Path, tag: str, src: Path, pin: str) -> dict:
    print(f"①②③ 해소 · {tag}")
    repo, box = 사본(tmp, f"resolve6-{tag}", src, pin)
    got = 인입(repo, box, pin)
    쓸것, 뺀수 = 걸러낸다(got)
    if not 쓸것:
        fail(f"①② {tag}", "조각이 0 개다 — 이 대조는 아무것도 안 잰다")
        return {}

    분포 = {"bound": 0, "candidates": 0, "unbound": 0}
    for p in 쓸것:
        분포[분류(p)] += 1
    ok(f"①② {tag} 분포",
       f"조각 {len(쓸것)} (도구 설정 {뺀수} 제외) · 결박됨 {분포['bound']} · "
       f"후보 {분포['candidates']} · 미결박 {분포['unbound']}")

    # ★ **표식 주석만 따로 센다** — 이 절이 고친 것이 그 경로다.
    주석 = [p for p in 쓸것 if 표식주석앵커.match(p["fragment"]["anchor"])]
    주석분포 = {"bound": 0, "candidates": 0, "unbound": 0}
    for p in 주석:
        주석분포[분류(p)] += 1
    if not 주석:
        # ⚠ **코퍼스 하나의 0 은 실패가 아니라 대조 불가다**([ADR-0002]) —
        #   portal-backend 는 `.kt` 에 `ADR-` 표식 주석이 **0 건**이고 그것이 실물이다.
        #   **합계가 0 인 것만 실패**이고, 그것은 아래 `모집단()` 이 진다.
        skip(f"② {tag} 표식 주석", "표식 주석 조각이 0 개다 — **모집단 0 이라 대조 불가**")
    else:
        ok(f"② {tag} 표식 주석",
           f"조각 {len(주석)} · 결박됨 {주석분포['bound']} · 후보 {주석분포['candidates']} · "
           f"미결박 {주석분포['unbound']} — **셋을 기록으로 낸다**")

    return {"proposals": 쓸것, "주석": 주석, "counts": 분포, "주석분포": 주석분포,
            "repo": repo, "box": box, "pin": pin}


# ═════════════════════════════════════════════════════════════════════════════
# ④ ★ 음성 대조 — **안 붙여야 할 것을 계속 안 붙이는가.** 소스에서 줄로 센다
# ═════════════════════════════════════════════════════════════════════════════

주석줄 = re.compile(r"^\s*(//|/\*|\*|#)")


def 갈래(내용: str, 시작줄: int, 본문: str) -> tuple[bool, str]:
    """**이 검사가 계산하는 문장**(`[f10.6.pass].negative_binding_grounds` 의 표):

      · **빈 줄로 갈림** — 주석이 끝난 줄과 그 뒤 첫 비주석·비공백 줄 사이에
        **공백만 있는 줄이 하나 이상** 있다
      · **붙을 선언이 없음** — 그 첫 줄이 `import` 로 시작하거나 **그런 줄이 없다**

    ⚠ **`다음_선언` 과 다른 방법으로 센다.** 같은 방법으로 세면 구현이 자기를 채점한다.
    """
    줄들 = 내용.split("\n")
    i = 시작줄 + 본문.count("\n")          # 주석의 **마지막 줄** 다음(1-기반 → 0-기반 겸함)
    빈줄있음 = False
    while i < len(줄들):
        s = 줄들[i].strip()
        if s == "":
            빈줄있음 = True
        elif 주석줄.match(줄들[i]):
            pass
        else:
            break
        i += 1
    if i >= len(줄들):
        return 빈줄있음, "없음"
    첫줄 = 줄들[i].lstrip()
    if 첫줄.startswith("import"):
        return 빈줄있음, "import"
    return 빈줄있음, "선언"


def 모집단(전체: dict[str, dict]) -> None:
    """★ **표식 주석 조각의 합계 하한** — 0 이면 「미결박이 줄었다」가 0−0−0 으로 공짜다."""
    합 = sum(len(d["주석"]) for d in 전체.values() if d)
    if 합 < MARKED_COMMENT_POPULATION_MIN:
        fail("② 표식 주석 모집단",
             f"두 코퍼스를 합쳐 {합}개다 (하한 {MARKED_COMMENT_POPULATION_MIN}) — "
             f"★ **이 절이 재는 것이 통째로 사라졌다**")
    else:
        ok("② 표식 주석 모집단", f"두 코퍼스 합계 {합}개 ≥ {MARKED_COMMENT_POPULATION_MIN}")


def 음성대조(전체: dict[str, dict]) -> None:
    print("④ ★ **안 붙여야 할 것을 계속 안 붙이는가** — 소스에서 줄로 센다")
    빈줄모집단 = 빈줄결박 = 없음모집단 = 없음결박 = 0
    걸린것: list[str] = []
    내용캐시: dict[tuple[str, str], str] = {}

    for tag, d in 전체.items():
        if not d:
            continue
        for p in d["주석"]:
            path = p["fragment"]["path"]
            열쇠 = (tag, path)
            if 열쇠 not in 내용캐시:
                r = run(["git", "-C", str(d["repo"]), "show", f"{d['pin']}:{path}"])
                내용캐시[열쇠] = r.stdout if r.returncode == 0 else ""
            내용 = 내용캐시[열쇠]
            if not 내용:
                continue
            시작 = int(p["fragment"]["anchor"][1:])
            빈줄, 종류 = 갈래(내용, 시작, p["fragment"]["body"])
            결박 = 분류(p) == "bound"
            if 빈줄:
                빈줄모집단 += 1
                if 결박:
                    빈줄결박 += 1
                    걸린것.append(f"빈줄: {tag} {path}:{p['fragment']['anchor']}")
            if 종류 in ("없음", "import"):
                없음모집단 += 1
                if 결박:
                    없음결박 += 1
                    걸린것.append(f"선언없음({종류}): {tag} {path}:{p['fragment']['anchor']}")

    # 모집단 하한이 **먼저**다 — 없으면 「막아서 0」과 「없어서 0」이 같은 값이 된다.
    if 빈줄모집단 < NEGATIVE_POPULATION_MIN:
        skip("④ 빈 줄로 갈림",
             f"모집단이 {빈줄모집단} 이다 (하한 {NEGATIVE_POPULATION_MIN}) — **대조 불가.** "
             f"⚠ 「막아서 0」과 「없어서 0」을 가를 수 없다")
    elif 빈줄결박 > NEGATIVE_BOUND_MAX:
        fail("④ 빈 줄로 갈림",
             f"모집단 {빈줄모집단} 중 **{빈줄결박} 건이 결박됐다** — "
             f"**처분 (나)가 코드에 안 박혔다**")
    else:
        ok("④ 빈 줄로 갈림",
           f"모집단 {빈줄모집단} · 결박 **0** — 빈 줄은 경계다")

    if 없음모집단 < NEGATIVE_POPULATION_MIN:
        skip("④ 붙을 선언이 없음",
             f"모집단이 {없음모집단} 이다 (하한 {NEGATIVE_POPULATION_MIN}) — **대조 불가**")
    elif 없음결박 > NEGATIVE_BOUND_MAX:
        fail("④ 붙을 선언이 없음",
             f"모집단 {없음모집단} 중 **{없음결박} 건이 결박됐다** — "
             f"★ **거짓 결박의 반대편이 열렸다**")
    else:
        ok("④ 붙을 선언이 없음",
           f"모집단 {없음모집단} · 결박 **0** — 없는 좌표를 지어내지 않는다")

    for x in 걸린것[:20]:
        print(f"      · {x}")


# ═════════════════════════════════════════════════════════════════════════════
# ①③ 표본 — **새로 뽑는다. 옛 판정을 옮겨 붙이지 않는다**
# ═════════════════════════════════════════════════════════════════════════════

def 표본(전체: dict[str, dict]) -> None:
    print("①③ 표본 — **새 파일에 적는다.** F10-5 의 판정 기록을 덮지 않는다")
    걸린 = [(t, p) for t, d in 전체.items() if d for p in d["proposals"] if 분류(p) == "bound"]

    # ② 결박 하한 — ★ **재현율의 하한을 겸한다.** 23 에서 넘으려면 래퍼를 벗기는 수밖에 없다.
    if len(걸린) < BOUND_COUNT_MIN:
        skip("② 결박 하한(재현율)",
             f"결박됨이 {len(걸린)}건이다 (하한 {BOUND_COUNT_MIN}) — **①이 자기 정의"
             f"(표본 {FALSE_BINDING_SAMPLE})대로 못 재어진다. 통과가 아니라 대조 불가다**")
    else:
        ok("② 결박 하한(재현율)",
           f"결박됨 {len(걸린)}건 ≥ {BOUND_COUNT_MIN} — **래퍼를 벗긴 것이 산출에 남았다**")

    attached = [(t, p) for t, p in 걸린 if 신호(p) == "attached"]
    if len(attached) < ATTACHED_SAMPLE_MIN:
        skip("③ attached 표본",
             f"`attached` 로 걸린 것이 {len(attached)}건이다 (하한 {ATTACHED_SAMPLE_MIN}) — "
             f"**대조 불가**")
    else:
        ok("③ attached 표본",
           f"`attached` 결박 {len(attached)}건 ≥ {ATTACHED_SAMPLE_MIN} — "
           f"**{'전수' if len(attached) <= FALSE_BINDING_SAMPLE else '등간격 표본'}으로 판정한다**")

    잘못 = sorted({신호(p) for _, p in 걸린 if 신호(p) not in ZERO_DISTANCE})
    if 잘못:
        fail("① 거리 있는 신호가 확정했다",
             f"{잘못} 가 `결박됨` 을 냈다 — **처분이 코드에 안 박혔다. 구현의 반증이다**")
    elif 걸린:
        ok("① 확정한 신호", f"전부 거리 0 이다 — {sorted({신호(p) for _, p in 걸린})}")

    if not 걸린:
        skip("① 표본 파일", "결박이 0 건이다 — **판정할 것이 없다. 대조 불가**")
        return

    # **`EntityId` 사전순 등간격** — 우리가 고르지 않는다(`[f10.2].sample_selection` 규칙 3).
    걸린.sort(key=lambda x: x[1]["item"]["id"])
    n = min(FALSE_BINDING_SAMPLE, len(걸린))
    간격 = max(1, len(걸린) // n)
    뽑은 = [걸린[i * 간격] for i in range(n) if i * 간격 < len(걸린)]

    이름 = {}
    for tag, d in 전체.items():
        if not d:
            continue
        dump = pal(["query", "graph.dump", "--json"], d["repo"], d["box"], d["pin"])
        for nd in dump["answer"]["nodes"]:
            이름[nd["id"]] = (nd["name"], nd["path"], nd["kind"])

    # ★ **새 파일이다** — `f10-5-binding-sample.tsv` 를 덮으면 F10-5 의 반증 근거가 사라진다
    #   (`[f10.6].sample_selection` 규칙 10).
    out = ROOT / "corpus/tasks/f10-6-binding-sample.tsv"
    with out.open("w", encoding="utf-8") as f:
        f.write("코퍼스\t개체\t문서\t앵커\t걸린신호\t심볼\t종류\t코드경로\t조각머리\t판정\t근거\n")
        for tag, p in 뽑은:
            머리 = (p["fragment"]["body"].splitlines() or [""])[0][:90].replace("\t", " ")
            nm, pth, kind = 이름.get(p["class"]["target"], ("?", "?", "?"))
            f.write(f"{tag}\t{p['item']['id']}\t{p['fragment']['path']}\t{p['fragment']['anchor']}\t"
                    f"{신호(p)}\t{nm}\t{kind}\t{pth}\t{머리}\t\t\n")
    ok("①③ 표본 파일",
       f"{out.relative_to(ROOT)} — {len(뽑은)}건 · **판정은 게이트에 조각마다 한 줄로**")


# ═════════════════════════════════════════════════════════════════════════════
# ⑥ ★ Kotlin — **실 코퍼스는 대조 불가다. 픽스처로 세운다**
# ═════════════════════════════════════════════════════════════════════════════

코틀린픽스처 = """// ADR-0099 이 클래스가 왜 이렇게 결정됐는지 — 붙어 있다
class F106Bound(val a: Int)

// ADR-0099 이것은 파일 머리 주석처럼 빈 줄로 갈려 있다 — 붙어 있지 않다

class F106Unbound(val b: Int)
"""


def 코틀린(tmp: Path) -> None:
    print("⑥ ★ Kotlin — 실 코퍼스 모집단 **0**. 픽스처로 세운다")
    if not PORTAL.exists():
        skip("⑥ Kotlin", f"코퍼스가 없다: {PORTAL} — **대조 불가**")
        return
    repo, box = 사본(tmp, "kt6", PORTAL, PORTAL_PIN)

    # ⚠ **실 코퍼스에 표식 주석이 몇 건인지 먼저 센다** — 0 이면 그것이 [ADR-0002] 다.
    r = run(["git", "-C", str(repo), "grep", "-l", "ADR-", PORTAL_PIN, "--", "*.kt"])
    실물 = len([x for x in r.stdout.split("\n") if x.strip()])

    파일 = repo / "f10-6-kotlin-fixture.kt"
    파일.write_text(코틀린픽스처, encoding="utf-8")
    run(["git", "-C", str(repo), "add", "-A"])
    run(["git", "-C", str(repo), "-c", "user.email=t@e", "-c", "user.name=t",
         "commit", "-q", "-m", "f10-6 Kotlin 픽스처"])
    head = run(["git", "-C", str(repo), "rev-parse", "HEAD"]).stdout.strip()

    got = 인입(repo, box, head)
    내것 = [p for p in got["proposals"]
            if p["fragment"]["path"] == "f10-6-kotlin-fixture.kt"]
    if len(내것) != 2:
        fail("⑥ Kotlin 픽스처",
             f"조각이 {len(내것)}개다 — 둘을 기대했다. **변형이 안 먹었다**")
        return

    내것.sort(key=lambda p: int(p["fragment"]["anchor"][1:]))
    첫, 둘 = 내것
    걸림 = []
    if 분류(첫) != "bound":
        걸림.append(f"인접한 주석이 `{분류(첫)}` 다 — **Kotlin 경로가 안 붙는다. "
                    f"이 수정이 한 언어만 고쳤다**")
    elif 신호(첫) != "attached":
        걸림.append(f"인접한 주석을 `{신호(첫)}` 가 걸었다 — `attached` 를 기대했다")
    if 분류(둘) == "bound":
        걸림.append("빈 줄로 갈린 주석이 **결박됐다** — 처분 (나)가 Kotlin 경로에 안 박혔다")

    if 걸림:
        fail("⑥ Kotlin 픽스처", " · ".join(걸림))
    else:
        ok("⑥ Kotlin 픽스처",
           f"인접 **결박 1** ≥ {KOTLIN_FIXTURE_BOUND_MIN} · 빈 줄 **미결박 1** ≥ "
           f"{KOTLIN_FIXTURE_UNBOUND_MIN} — **래퍼 없는 언어에서도 두 방향이 산다**")

    skip("⑥ Kotlin 실 코퍼스",
         f"portal-backend 의 `.kt` 중 `ADR-` 를 가진 파일이 **{실물}** 개다 — "
         f"**모집단 0 이므로 대조 불가.** ⚠ **픽스처의 통과를 실 코퍼스의 통과로 "
         f"적지 않는다**([ADR-0002])")


# ═════════════════════════════════════════════════════════════════════════════
# ⑧ 후보 여럿의 모집단 — ★ **열아홉째를 이 절이 만들 수 있다**
# ═════════════════════════════════════════════════════════════════════════════

def 재측정_모집단(전체: dict[str, dict]) -> None:
    """★ **[#63] 의 물음 2 를 재는 자리** — 표본이 안 차는 것이 **희소해서인가,
    표본기가 잘라서인가.**

    `f10-verify.py` 의 ⑨ 는 코퍼스당 **최대 10** 을 뽑는다(`SAMPLES_PER_CORPUS`).
    그런데 등록된 하한은 **20**(`[f10.4.pass].f09_remeasure_sample_min`)이고, 코퍼스는
    둘이며 그중 **portal-backend 는 표식 주석이 0 건이라 구조적으로 0** 이다.
    **그러므로 이 축에서 도달 가능한 최대가 10 이다** — 결박이 얼마나 늘든.

    ⚠ **그 사실은 결박을 세어야 확인된다.** 모집단이 크면 자르는 것이고, 모집단이
    작으면 희소한 것이다. **둘은 다른 답이고 처방도 다르다.**
    """
    print("⑦ 짝 — **재측정의 모집단을 센다.** 잘라서인가 희소해서인가")
    for tag, d in 전체.items():
        if not d:
            continue
        커밋들 = run(["git", "-C", str(d["repo"]), "log", "--no-merges", "--format=%H",
                     d["pin"]]).stdout.split()[:120]
        if len(커밋들) < 20:
            skip(f"⑦ 모집단 {tag}", f"머지 아닌 커밋이 {len(커밋들)}개뿐이다 — **대조 불가**")
            continue
        건드린 = set(run(["git", "-C", str(d["repo"]), "log", "--no-merges", "--name-only",
                         "--format=", f"{커밋들[-1]}..{d['pin']}"]).stdout.split())
        dump = pal(["query", "graph.dump", "--json"], d["repo"], d["box"], d["pin"])
        경로 = {n["id"]: n["path"] for n in dump["answer"]["nodes"]}
        결박 = [p for p in d["proposals"] if 분류(p) == "bound"]
        창안 = [p for p in 결박 if 경로.get(p["class"]["target"]) in 건드린]
        ok(f"⑦ 모집단 {tag}",
           f"결박 {len(결박)} · 그중 창(커밋 {len(커밋들)}) 안에서 건드려진 파일의 것 "
           f"**{len(창안)}** · ⑨ 가 뽑는 상한 **{F10.SAMPLES_PER_CORPUS}**")


def 후보모집단(전체: dict[str, dict]) -> None:
    print("⑧ 후보 여럿의 모집단 — **결박이 늘면 이것이 준다**")
    후보있음 = sum(1 for d in 전체.values() if d
                 for p in d["proposals"] if 분류(p) == "candidates")
    if 후보있음 < CANDIDATES_POPULATION_MIN:
        fail("⑧ 후보 여럿 모집단",
             f"후보 여럿인 제안이 {후보있음}건이다 (하한 {CANDIDATES_POPULATION_MIN}) — "
             f"★ **`[f10.5.pass]` ⑦(일괄 승인의 거부)이 이번 회차에 꺼졌다. "
             f"열아홉째의 셋째 실물이다**")
    else:
        ok("⑧ 후보 여럿 모집단",
           f"후보 여럿인 제안 {후보있음}건 ≥ {CANDIDATES_POPULATION_MIN} — "
           f"**`[f10.5.pass]` ⑦의 하한이 계속 선다**")


# ═════════════════════════════════════════════════════════════════════════════

def main() -> int:
    if not F10.BIN.exists():
        raise SystemExit(f"바이너리가 없다: {F10.BIN} — `cargo build --release` 먼저")

    print("F10-6 — 「다음에 오는 선언」을 「붙어 있음」으로 (#62 · #63)")
    tmp = Path(tempfile.mkdtemp(prefix="pal-f10-6-"))
    전체 = {}
    try:
        for tag, src, pin in [("ditto", DITTO, DITTO_PIN), ("portal-backend", PORTAL, PORTAL_PIN)]:
            if not src.exists():
                skip(f"①②③ {tag}", f"코퍼스가 없다: {src} — **대조 불가**")
                continue
            전체[tag] = 해소(tmp, tag, src, pin)
        모집단(전체)
        음성대조(전체)
        표본(전체)
        코틀린(tmp)
        후보모집단(전체)
        재측정_모집단(전체)

        # ⑦ [#63] — **`f10-verify.py` 의 ⑨ 를 그대로 부른다.** 복제하지 않는다.
        #   ⚠ 태그에 `-6` 을 붙여 **F10 회차의 판정 파일을 안 덮는다**
        #   (`[f10.6].sample_selection` 규칙 10).
        print("⑦ [#63] F09 ⑤ 재측정 — `f10-verify.py` ⑨ 를 그대로 부른다")
        for tag, src, pin, ext in [("ditto", DITTO, DITTO_PIN, ".ts"),
                                   ("portal-backend", PORTAL, PORTAL_PIN, ".kt")]:
            if not src.exists():
                skip(f"⑦ {tag}", f"코퍼스가 없다: {src} — **대조 불가**")
                continue
            F10.f09_재측정(tmp, f"{tag}-6", src, pin, ext)
    finally:
        shutil.rmtree(tmp, ignore_errors=True)

    print()
    for 표시, 이름, 값 in 결과:
        print(f"  {표시:<5} {이름}  — {값}")
    어긋남 = [f"{n}: {v}" for m, n, v in 결과 if m == "FAIL"]
    불가 = [f"{n}: {v}" for m, n, v in 결과 if m == "–"]
    print()
    if 어긋남:
        print(f"어긋난 것 {len(어긋남)}:")
        for x in 어긋남:
            print(f"   · {x}")
    if 불가:
        print(f"대조 불가 {len(불가)}건 — **통과로 세지 않는다**")
        for x in 불가:
            print(f"   – {x}")
    if not 어긋남:
        print("어긋남 0")
    print()
    print("⑤ 골든 넷은 **기준선이 진다** — `f03-3` ③(symbols 둘) · `f01`(ledger 997) · "
          "`s0-compare`(reference-vector). ⚠ **이번엔 `pal-extract` 를 고쳤다**")
    return 1 if 어긋남 else 0


if __name__ == "__main__":
    sys.exit(main())
