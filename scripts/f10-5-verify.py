#!/usr/bin/env python3
"""F10-5 대조 — **신호를 줄인다** (#59 · #60). 합격선은 재기 전에 등록됐다
(`corpus/criteria.toml` `[f10.5]` · 등록 커밋 `6218675` — **첫 코드 커밋 이전**).

    ① 거짓 결박률 **≤ 5%** (표본 50) — `[f10.pass]` 에서 **글자 그대로 옮긴 값**
    ② 그 짝 — 결박됨 **≥ 50**. 미만이면 **통과가 아니라 대조 불가**
    ③ ★ **미결박의 증가분** — 정직해진 양이 산출에 남는가. 그리고 **뺀 것이 정말 빠졌는가**
    ④ 후보 퍼짐 — 뺀 신호 둘이 **산출에서 사라졌는가** · 남은 것이 **좁히는가**
    ⑤ ★ **동점 미확정이 여전히 서는가** — ⚠ **거리 0 인 신호로 다시 만든다**
    ⑥ 문서 변형 다섯 — `[f10.1.pass]` 가 등록했는데 `f10-verify` 가 안 세우던 것
    ⑦ ★ **`attached` 를 다시 잰다** — F10 의 0% 는 **표본이 둘**이었다
    ⑧ 일괄 승인의 거부 — 사유 하나가 **도달 불가**가 됐다. 남은 사유가 하한을 지는가

# 이 스크립트가 막는 「대조가 꺼지는 형태」 넷

  · ★ **열아홉째(한 대조의 개선이 다른 대조의 하한을 끈다)** — 이 이슈에 **직접** 걸린다.
    신호를 빼면 `[f10.pass]` ④(동점)의 재료가 준다. **옛 픽스처는 스팬·경로 동점이었고
    둘 다 이제 확정할 수 없는 신호**라 *"동점을 확정 안 했다"* 가 **공짜로 참**이 된다.
    그래서 ⑤가 동점을 **거리 0 인 신호(`frontmatter`)로 만들고**, 그 짝으로
    **거리 있는 신호가 유일해도 확정 안 하는지**를 함께 센다
  · **첫째(변형이 아무것도 안 바꿈)** — 신호를 빼는 것은 **코드 변경**이다. ③이
    **뺀 신호의 날것이 실려 있는데도 후보를 하나도 안 내는지**를 직접 센다.
    날것이 0 이면 그 검사가 아무것도 안 세는 것이므로 **그것도 잡는다**
  · **열여섯·열일곱(변형이 대상을 안 건드림 · 종류를 안 봄)** — ⑥이 변형을 **종류별로**
    하고 **무엇이 바뀌어야 하는지를 먼저 적고** 그대로 나오는지 센다. 바뀐 파일 수와
    바이트 차이가 0 이면 멈춘다
  · **편의 표본** — palimpsest 는 여기서 **안 돈다**. 이 빌드에 Rust 추출기가 없어
    판정 대상이 비어 있다(`[f10].input_quality`)

**제외 규칙·코퍼스 핀·인입 절차는 `f10-verify.py` 에서 빌려 쓴다** — 복제하면 그것이
곧 두 곳에 적힌 같은 규칙이다(옛 계획 §7 의 넷째).

사용:
    ./scripts/f10-5-verify.py
"""

from __future__ import annotations

import importlib.util
import shutil
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent

_spec = importlib.util.spec_from_file_location("f10_verify", ROOT / "scripts/f10-verify.py")
F10 = importlib.util.module_from_spec(_spec)
assert _spec.loader is not None
_spec.loader.exec_module(F10)

# ── `[f10.5.pass]` 가 등록한 값들. **여기서 정하지 않는다 — 옮겨 적을 뿐이다** ────────
FALSE_BINDING_MAX_PCT = 5          # `[f10.pass]` 에서 그대로
FALSE_BINDING_SAMPLE = 50          # `[f10.pass]` 에서 그대로
BOUND_COUNT_MIN = 50               # `[f10.pass]` 에서 그대로
ATTACHED_SAMPLE_MIN = 20           # `[f10.4.pass].f09_remeasure_sample_min` 에서 그대로
NARROWING_MIN_WITHIN_THREE = 1
DISTRIBUTION_CHANGED_MIN = 1
BATCH_REFUSAL_MIN = 1
MUTATION_KINDS = 5
MIN_CHANGED_FILES = 1
MIN_CHANGED_BYTES = 1

# **거리 0 인 신호** — `ResolutionSignal::can_confirm_subject`.
ZERO_DISTANCE = {"attached", "frontmatter"}
# **계단식에서 뺀 신호** — `[f10.5].signal_ruling`. 산출에 한 줄도 나오면 안 뺀 것이다.
REMOVED = {"same-commit", "directory-proximity"}

결과 = F10.결과  # 같은 목록에 쌓는다 — 표시 규약이 하나여야 한다
ok, fail, skip = F10.ok, F10.fail, F10.skip
run, pal, 사본, 인입, 걸러낸다, 분류, 신호 = (
    F10.run, F10.pal, F10.사본, F10.인입, F10.걸러낸다, F10.분류, F10.신호
)
DITTO, DITTO_PIN, PORTAL, PORTAL_PIN = F10.DITTO, F10.DITTO_PIN, F10.PORTAL, F10.PORTAL_PIN


# ═════════════════════════════════════════════════════════════════════════════
# ①②③④ — 분포 · 결박 하한 · 뺀 것이 정말 빠졌나 · 후보 퍼짐
# ═════════════════════════════════════════════════════════════════════════════

def 해소(tmp: Path, tag: str, src: Path, pin: str) -> dict:
    print(f"①②③④ 해소 · {tag}")
    repo, box = 사본(tmp, f"resolve-{tag}", src, pin)
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

    # ★ ③ **뺀 것이 정말 빠졌는가** — 「변형이 아무것도 안 바꿈」을 막는 자리.
    #
    #   `co_changed` 는 **날것으로 남아 있다**([ADR-0002] · 지우면 모집단이 사라진다).
    #   그러므로 *"날것은 실렸는데 그 신호로 걸린 것이 0"* 이 **뺐다는 실측**이다.
    #   ⚠ **날것이 0 이면 이 검사가 아무것도 안 센다** — 그것도 잡는다.
    날것있음 = [p for p in 쓸것 if p["fragment"]["signals"]["co_changed"]]
    뺀신호로걸린것 = [p for p in 쓸것 if 신호(p) in REMOVED]
    if not 날것있음:
        skip(f"③ {tag} 뺀 신호",
             "`co_changed` 날것을 가진 조각이 0 개다 — **모집단 0 이라 이 검사가 "
             "아무것도 안 센다**")
    elif 뺀신호로걸린것:
        fail(f"③ {tag} 뺀 신호",
             f"뺀 신호로 걸린 조각이 {len(뺀신호로걸린것)}개다 — **안 뺀 것이다**")
    else:
        ok(f"③ {tag} 뺀 신호",
           f"날것을 가진 조각 {len(날것있음)}개 · 그중 뺀 신호로 걸린 것 **0** — "
           f"**날것은 남고 해소에는 안 쓰인다**")

    # ★ ④ **후보 퍼짐** — 뺀 신호가 표에 안 나오는가 · 남은 것이 좁히는가.
    퍼짐: dict[str, list[int]] = {}
    for p in 쓸것:
        if 분류(p) == "candidates":
            퍼짐.setdefault(신호(p), []).append(len(p["class"]["candidates"]))
    if not 퍼짐:
        skip(f"④ {tag} 후보 퍼짐", "후보 있음이 0 건이다 — **모집단 0 이라 대조 불가**")
    else:
        남은것 = sorted(set(퍼짐) & REMOVED)
        if 남은것:
            fail(f"④ {tag} 뺀 신호가 산출에", f"{남은것} 가 퍼짐 표에 있다 — **안 뺀 것이다**")
        else:
            ok(f"④ {tag} 뺀 신호가 산출에", f"{sorted(REMOVED)} 가 한 줄도 없다")
        셋이하합 = 0
        for by in sorted(퍼짐):
            s = sorted(퍼짐[by])
            셋이하 = sum(1 for x in s if x <= 3)
            셋이하합 += 셋이하
            ok(f"   {tag} 후보 퍼짐 {by}",
               f"조각 {len(s)} · 중앙 {s[len(s) // 2]} · 최대 {s[-1]} · 셋 이하 {셋이하}")
        if 셋이하합 < NARROWING_MIN_WITHIN_THREE:
            fail(f"④ {tag} 좁히는가",
                 f"후보 셋 이하인 제안이 {셋이하합}건이다 (하한 {NARROWING_MIN_WITHIN_THREE}) — "
                 f"**남은 신호도 좁히지 못한다. #60 이 안 닫힌다**")
        else:
            ok(f"④ {tag} 좁히는가", f"후보 셋 이하인 제안 {셋이하합}건 ≥ {NARROWING_MIN_WITHIN_THREE}")

    return {"proposals": 쓸것, "counts": 분포, "repo": repo, "box": box}


# ═════════════════════════════════════════════════════════════════════════════
# ①② 표본 · ⑦ attached 를 다시 잰다
# ═════════════════════════════════════════════════════════════════════════════

def 표본(전체: dict[str, dict]) -> None:
    print("①⑦ 표본 — **옛 판정을 옮겨 붙이지 않는다. 새로 뽑는다**")
    걸린 = [(t, p) for t, d in 전체.items() if d for p in d["proposals"] if 분류(p) == "bound"]

    # ② 결박 하한 — **미만이면 통과가 아니라 대조 불가다**(`[f10.5.pass]` ②).
    if len(걸린) < BOUND_COUNT_MIN:
        skip("② 결박 하한",
             f"결박됨이 {len(걸린)}건이다 (하한 {BOUND_COUNT_MIN}) — **①이 자기 정의"
             f"(표본 {FALSE_BINDING_SAMPLE})대로 못 재어진다. 통과가 아니라 대조 불가다**")
    else:
        ok("② 결박 하한", f"결박됨 {len(걸린)}건 ≥ {BOUND_COUNT_MIN}")

    # ★ ⑦ **`attached` 를 따로 잰다** — F10 의 0% 는 표본이 둘이었다.
    attached = [(t, p) for t, p in 걸린 if 신호(p) == "attached"]
    if len(attached) < ATTACHED_SAMPLE_MIN:
        skip("⑦ attached 표본",
             f"`attached` 로 걸린 것이 {len(attached)}건이다 (하한 {ATTACHED_SAMPLE_MIN}) — "
             f"**대조 불가.** ⚠ **「0% 를 유지했다」로 적으면 [ADR-0002] 위반이다**")
    else:
        ok("⑦ attached 표본",
           f"`attached` 결박 {len(attached)}건 ≥ {ATTACHED_SAMPLE_MIN} — "
           f"**{'전수' if len(attached) <= FALSE_BINDING_SAMPLE else '등간격 표본'}으로 판정한다**")

    # 거리 0 이 아닌 신호가 확정을 냈으면 **구현의 반증이다**(`falsified_if`).
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
        dump = pal(["query", "graph.dump", "--json"], d["repo"], d["box"],
                   DITTO_PIN if tag == "ditto" else PORTAL_PIN)
        for nd in dump["answer"]["nodes"]:
            이름[nd["id"]] = (nd["name"], nd["path"], nd["kind"])

    out = ROOT / "corpus/tasks/f10-5-binding-sample.tsv"
    with out.open("w", encoding="utf-8") as f:
        f.write("코퍼스\t개체\t문서\t앵커\t걸린신호\t심볼\t종류\t코드경로\t조각머리\t판정\t근거\n")
        for tag, p in 뽑은:
            머리 = (p["fragment"]["body"].splitlines() or [""])[0][:90].replace("\t", " ")
            nm, pth, kind = 이름.get(p["class"]["target"], ("?", "?", "?"))
            f.write(f"{tag}\t{p['item']['id']}\t{p['fragment']['path']}\t{p['fragment']['anchor']}\t"
                    f"{신호(p)}\t{nm}\t{kind}\t{pth}\t{머리}\t\t\n")
    ok("①⑦ 표본 파일",
       f"{out.relative_to(ROOT)} — {len(뽑은)}건 · **판정은 게이트에 조각마다 한 줄로**")


# ═════════════════════════════════════════════════════════════════════════════
# ⑤ 동점 — ★ **거리 0 인 신호로 다시 만든다.** 열아홉째가 여기 걸린다
# ═════════════════════════════════════════════════════════════════════════════

def 동점(tmp: Path) -> None:
    print("⑤ ★ 동점 — **거리 0 인 신호로 만든다** (옛 픽스처는 무력해졌다)")
    repo, box = 사본(tmp, "tie5", DITTO, DITTO_PIN)

    dump = pal(["query", "graph.dump", "--json"], repo, box, DITTO_PIN)
    파일별: dict[str, list[dict]] = {}
    for n in dump["answer"]["nodes"]:
        파일별.setdefault(n["path"], []).append(n)
    # 심볼이 **둘 이상**인 파일 — `grounds:` 에 `#` 없이 파일만 적으면 그 파일의 심볼
    # 전부가 후보다(`by_ground`). **그것이 거리 0 인 신호로 만든 동점이다.**
    여럿 = sorted((p for p, v in 파일별.items() if len(v) >= 2), key=lambda p: (-len(파일별[p]), p))
    # 그리고 **확정**할 자리 하나 — 같은 파일의 심볼 이름 하나를 `#` 뒤에 적는다.
    if not 여럿:
        skip("⑤ 동점", "심볼이 둘 이상인 파일이 없다 — **모집단 0 이라 대조 불가**")
        return
    동점파일 = 여럿[0]
    이름들 = sorted({n["name"] for n in 파일별[동점파일]})
    확정파일, 확정이름 = None, None
    for p, v in sorted(파일별.items()):
        나온이름 = [n["name"] for n in v]
        for nm in sorted(set(나온이름)):
            if 나온이름.count(nm) == 1:
                확정파일, 확정이름 = p, nm
                break
        if 확정파일:
            break
    if not 확정파일:
        skip("⑤ 동점", "유일하게 해소될 좌표가 없다 — **모집단 0 이라 대조 불가**")
        return

    # 그리고 **거리 있는 신호가 유일해도 확정 안 하는지** — 이 절의 핵심 단언.
    이름별: dict[str, list[dict]] = {}
    for n in dump["answer"]["nodes"]:
        이름별.setdefault(n["name"], []).append(n)
    유일한이름 = sorted(k for k, v in 이름별.items() if len(v) == 1 and k.isidentifier())
    if not 유일한이름:
        skip("⑤ 거리 있는 신호", "유일한 이름이 없다 — **모집단 0**")
        return

    doc = repo / "docs" / "f10-5-tie.md"
    doc.parent.mkdir(parents=True, exist_ok=True)
    doc.write_text(
        # ① **프론트매터 동점** — 거리 0 인데 파일만 적어 후보가 여럿이다.
        f"---\ngrounds: [\"{동점파일}\"]\n---\n\n"
        f"# 프론트매터-동점\n\n`{동점파일}` 의 것들을 말한다.\n\n"
        # ② **확정** — 거리 0 이고 유일하다. 없으면 「전부 후보로 낸다」가 만점을 받는다.
        f"# 확정\n\n확정할 자리다.\n\n"
        # ③ ★ **거리 있는 신호는 유일해도 확정 안 한다** — 이 절이 바꾼 것.
        f"# 유일한-스팬\n\n`{유일한이름[0]}` 하나뿐이다.\n",
        encoding="utf-8",
    )
    # 「확정」 조각에 프론트매터가 아니라 **그 조각만의** 좌표를 줘야 하는데,
    # 프론트매터는 문서 첫 조각에만 걸린다. 그래서 **문서를 하나 더 만든다.**
    doc2 = repo / "docs" / "f10-5-확정.md"
    doc2.write_text(
        f"---\ngrounds: [\"{확정파일}#{확정이름}\"]\n---\n\n"
        f"# 확정\n\n`{확정이름}` 하나를 가리킨다.\n",
        encoding="utf-8",
    )
    run(["git", "-C", str(repo), "add", "-A"])
    run(["git", "-C", str(repo), "-c", "user.email=t@e", "-c", "user.name=t",
         "commit", "-q", "-m", "f10-5 동점 픽스처"])
    head = run(["git", "-C", str(repo), "rev-parse", "HEAD"]).stdout.strip()

    got = 인입(repo, box, head)
    내것 = {(p["fragment"]["path"], p["fragment"]["anchor"]): p for p in got["proposals"]}
    걸림 = []

    기대 = [
        ("docs/f10-5-tie.md", "프론트매터-동점", "candidates", "frontmatter"),
        ("docs/f10-5-확정.md", "확정", "bound", "frontmatter"),
        ("docs/f10-5-tie.md", "유일한-스팬", "candidates", "span"),
    ]
    for path, 앵커, 기대분류, 기대신호 in 기대:
        p = 내것.get((path, 앵커))
        if p is None:
            걸림.append(f"{앵커}: 조각이 없다 — 변형이 안 먹었다")
            continue
        났다, 난신호 = 분류(p), 신호(p)
        if 났다 != 기대분류:
            사유 = {
                ("candidates", "bound"): "**동점을 확정했다. 이것이 거짓 결박이다**",
                ("candidates", "unbound"): "**동점을 미결박으로 접었다**",
                ("bound", "candidates"): "**거리 0 인데 유일한 것을 확정 못 했다**",
                ("bound", "unbound"): "**거리 0 인데 아무것도 못 걸었다**",
            }.get((기대분류, 났다), f"{기대분류} 를 기대했는데 {났다} 다")
            걸림.append(f"{앵커}: {났다} — {사유}")
        elif 난신호 != 기대신호:
            걸림.append(f"{앵커}: `{난신호}` 가 걸었다 — `{기대신호}` 를 기대했다")

    if 걸림:
        fail("⑤ 동점", " · ".join(걸림))
    else:
        ok("⑤ 동점",
           f"프론트매터 동점 {len(내것[('docs/f10-5-tie.md', '프론트매터-동점')]['class']['candidates'])}후보 · "
           f"확정 1 · **유일한 스팬도 후보로 나갔다** — 거리 있는 신호는 확정을 못 한다")


# ═════════════════════════════════════════════════════════════════════════════
# ⑥ 문서 변형 다섯 — **`[f10.1.pass]` 가 등록했는데 아무도 안 세우던 것**
# ═════════════════════════════════════════════════════════════════════════════

def 변형(tmp: Path) -> None:  # noqa: C901
    print("⑥ 문서 변형 다섯 — **종류별로. 그리고 「안 바뀌어야 하는 것」이 절반이다**")
    repo, box = 사본(tmp, "mut5", DITTO, DITTO_PIN)

    dump = pal(["query", "graph.dump", "--json"], repo, box, DITTO_PIN)
    이름별: dict[str, list[dict]] = {}
    for n in dump["answer"]["nodes"]:
        이름별.setdefault(n["name"], []).append(n)
    유일 = sorted(k for k, v in 이름별.items() if len(v) == 1 and k.isidentifier())
    경로들 = sorted({n["path"] for n in dump["answer"]["nodes"]})
    if len(유일) < 2 or len(경로들) < 2:
        skip("⑥ 변형", "변형에 쓸 재료가 없다 — **모집단 0 이라 대조 불가**")
        return

    # ⚠ **문서를 둘로 가른다 — 첫 회차가 여기서 걸렸다.**
    #
    #   처음에는 한 문서에 프론트매터와 코드 블록을 함께 뒀다. 그러자 계단식에서
    #   `frontmatter` 가 `fenced-path` 를 **가려서**, 경로를 바꿔도 후보가 안 움직였다 —
    #   그리고 이 검사가 그것을 *"변형이 대상을 안 건드렸다"* 로 잡았다.
    #   **열여섯째 형태를 이 픽스처가 실제로 밟았고 검사가 옳게 일했다.**
    #   프론트매터는 **문서 첫 조각에만** 걸리므로 그 변형만 따로 문서를 쓴다.
    doc = repo / "docs" / "f10-5-mut.md"
    doc.parent.mkdir(parents=True, exist_ok=True)
    원본 = (
        f"# 머리-하나\n\n산문 한 줄이다.\n\n```\n{경로들[0]}\n```\n\n"
        f"# 머리-둘\n\n`{유일[0]}` 를 말한다.\n"
    )
    doc.write_text(원본, encoding="utf-8")
    docfm = repo / "docs" / "f10-5-mut-fm.md"
    원본fm = f"---\ngrounds: [\"{경로들[0]}\"]\n---\n\n# 머리-fm\n\n프론트매터가 걸린 조각이다.\n"
    docfm.write_text(원본fm, encoding="utf-8")
    run(["git", "-C", str(repo), "add", "-A"])
    run(["git", "-C", str(repo), "-c", "user.email=t@e", "-c", "user.name=t",
         "commit", "-q", "-m", "f10-5 변형 원본"])
    기준 = 인입(repo, box, run(["git", "-C", str(repo), "rev-parse", "HEAD"]).stdout.strip())

    def 상태(got: dict, path: str = "docs/f10-5-mut.md") -> dict:
        out = {}
        for p in got["proposals"]:
            if p["fragment"]["path"] != path:
                continue
            out[p["fragment"]["anchor"]] = {
                "개체": p["item"]["id"],
                "본문": p["fragment"]["body"],
                "분류": 분류(p),
                "후보": sorted(p["class"].get("candidates", [])
                             or ([p["class"]["target"]] if 분류(p) == "bound" else [])),
            }
        return out

    앞 = 상태(기준)
    앞fm = 상태(기준, "docs/f10-5-mut-fm.md")
    if len(앞) < 2 or len(앞fm) < 1:
        fail("⑥ 변형", f"원본 조각이 {len(앞)}·{len(앞fm)}개다 — 이 대조는 아무것도 안 잰다")
        return

    # **무엇이 바뀌어야 하고 무엇이 안 바뀌어야 하는가 — 재기 전에 적는다**
    # (`[f10.5.pass].mutation_grounds` 의 표 그대로).
    변형들 = [
        ("헤딩 텍스트", 원본.replace("# 머리-둘", "# 머리-셋"),
         {"바뀜": ["앵커"], "안바뀜": []}),
        ("산문 한 줄", 원본.replace("산문 한 줄이다.", "산문을 고쳤다. 아주 다르게."),
         {"바뀜": ["본문"], "안바뀜": ["앵커", "개체", "후보"]}),
        ("코드 블록 안의 경로", 원본.replace(f"```\n{경로들[0]}\n```", f"```\n{경로들[1]}\n```"),
         {"바뀜": ["후보"], "안바뀜": ["앵커", "개체"]}),
        ("인라인 스팬의 이름", 원본.replace(f"`{유일[0]}`", f"`{유일[1]}`"),
         {"바뀜": ["후보"], "안바뀜": ["앵커", "개체"]}),
        ("프론트매터의 grounds", 원본fm.replace(f"grounds: [\"{경로들[0]}\"]",
                                              f"grounds: [\"{경로들[1]}\"]"),
         {"바뀜": ["후보"], "안바뀜": ["앵커", "개체"]}),
    ]
    if len(변형들) != MUTATION_KINDS:
        fail("⑥ 변형", f"변형이 {len(변형들)}종이다 — 등록은 {MUTATION_KINDS} 이다")
        return

    걸림 = []
    for 이름, 새본문, 기대 in 변형들:
        프론트 = 이름 == "프론트매터의 grounds"
        대상, 기준본문, 기준상태, 경로 = (
            (docfm, 원본fm, 앞fm, "docs/f10-5-mut-fm.md") if 프론트
            else (doc, 원본, 앞, "docs/f10-5-mut.md")
        )
        전바이트 = 대상.read_bytes()
        대상.write_text(새본문, encoding="utf-8")
        후바이트 = 대상.read_bytes()
        # **하한** — 변형이 실제로 먹었는가. 0 이면 그 자리에서 멈춘다.
        차이 = sum(1 for a, b in zip(전바이트, 후바이트) if a != b) + abs(len(전바이트) - len(후바이트))
        if 차이 < MIN_CHANGED_BYTES:
            걸림.append(f"{이름}: 바이트 차이 0 — **변형이 안 먹었다**")
            대상.write_text(기준본문, encoding="utf-8")
            continue
        run(["git", "-C", str(repo), "add", "-A"])
        run(["git", "-C", str(repo), "-c", "user.email=t@e", "-c", "user.name=t",
             "commit", "-q", "-m", f"f10-5 변형 {이름}"])
        head = run(["git", "-C", str(repo), "rev-parse", "HEAD"]).stdout.strip()
        뒤 = 상태(인입(repo, box, head), 경로)

        # 「앵커」 변형은 조각의 이름 자체가 바뀌므로 짝을 앵커로 못 맞춘다 — 수로 본다.
        if 이름 == "헤딩 텍스트":
            if set(기준상태) == set(뒤):
                걸림.append(f"{이름}: 앵커 집합이 그대로다 — **바뀌어야 하는 것이 안 바뀌었다**")
            elif len(뒤) != len(기준상태):
                걸림.append(f"{이름}: 조각 수가 {len(기준상태)} → {len(뒤)} 다 — 다른 것도 바뀌었다")
        else:
            공통 = set(기준상태) & set(뒤)
            if len(공통) != len(기준상태):
                걸림.append(f"{이름}: 앵커가 바뀌었다 — **안 바뀌어야 하는 것이 바뀌었다**")
            바뀐필드 = {k for a in 공통 for k in ("개체", "본문", "후보")
                      if 기준상태[a][k] != 뒤[a][k]}
            for k in 기대["바뀜"]:
                if k not in 바뀐필드:
                    걸림.append(f"{이름}: `{k}` 가 안 바뀌었다 — **변형이 대상을 안 건드렸다**")
            for k in 기대["안바뀜"]:
                if k in 바뀐필드:
                    걸림.append(
                        f"{이름}: `{k}` 가 바뀌었다 — **안 바뀌어야 하는 것이 바뀌었다**"
                        + ("  ★ **자연어를 보고 있다는 뜻이다**" if 이름 == "산문 한 줄" else "")
                    )
        대상.write_text(기준본문, encoding="utf-8")
        run(["git", "-C", str(repo), "add", "-A"])
        run(["git", "-C", str(repo), "-c", "user.email=t@e", "-c", "user.name=t",
             "commit", "-q", "-m", "f10-5 원본 복귀"])

    if 걸림:
        fail("⑥ 변형 다섯", " · ".join(걸림))
    else:
        ok("⑥ 변형 다섯",
           f"{MUTATION_KINDS}종 전부 — **바뀔 것이 바뀌고 안 바뀔 것이 안 바뀌었다.** "
           f"★ 산문을 고쳐도 좌표 후보가 안 움직였다")


# ═════════════════════════════════════════════════════════════════════════════
# ⑧ 일괄 승인의 거부 — **사유 하나가 도달 불가가 됐다**
# ═════════════════════════════════════════════════════════════════════════════

def 일괄(전체: dict[str, dict]) -> None:
    print("⑧ 일괄 승인의 거부 — **남은 사유가 하한을 지는가**")
    후보있음 = sum(1 for d in 전체.values() if d
                 for p in d["proposals"] if 분류(p) == "candidates")
    if 후보있음 < BATCH_REFUSAL_MIN:
        fail("⑧ 거부 하한",
             f"후보 여럿인 제안이 {후보있음}건이다 (하한 {BATCH_REFUSAL_MIN}) — "
             f"**남은 거부 사유의 모집단이 0 이고, 그러면 이 하한이 꺼진다**")
    else:
        ok("⑧ 거부 하한",
           f"후보 여럿인 제안 {후보있음}건 ≥ {BATCH_REFUSAL_MIN} — **남은 사유가 하한을 진다**")

    # ⚠ **도달 불가가 된 사유를 세어 둔다** — *"안 켜진다"* 와 *"없다"* 를 가른다.
    거리있는확정 = sum(1 for d in 전체.values() if d for p in d["proposals"]
                   if 분류(p) == "bound" and 신호(p) not in ZERO_DISTANCE)
    skip("⑧ 도달 불가가 된 사유",
         f"*\"판단이 드는 신호로 걸렸다\"* 의 모집단이 **{거리있는확정}** 이다 — "
         f"`Bound` 가 `ConfirmingSignal` 을 지므로 **구조적으로 0**. "
         f"⚠ **자리는 남긴다** — 거리 있는 신호가 확정을 내게 되면 다시 켜져야 한다")


# ═════════════════════════════════════════════════════════════════════════════

def main() -> int:
    if not F10.BIN.exists():
        raise SystemExit(f"바이너리가 없다: {F10.BIN} — `cargo build --release` 먼저")

    print("F10-5 — 신호를 줄인다 (#59 · #60)")
    tmp = Path(tempfile.mkdtemp(prefix="pal-f10-5-"))
    전체 = {}
    try:
        for tag, src, pin in [("ditto", DITTO, DITTO_PIN), ("portal-backend", PORTAL, PORTAL_PIN)]:
            if not src.exists():
                skip(f"①②③④ {tag}", f"코퍼스가 없다: {src} — **대조 불가**")
                continue
            전체[tag] = 해소(tmp, tag, src, pin)
        표본(전체)
        동점(tmp)
        변형(tmp)
        일괄(전체)
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
    return 1 if 어긋남 else 0


if __name__ == "__main__":
    sys.exit(main())
