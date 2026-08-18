#!/usr/bin/env python3
"""F10 대조 — 서술물 인입. **합격선은 재기 전에 등록됐다** (`corpus/criteria.toml` `[f10]`).

    ① 거짓 결박률 **< 5%** (표본 50 · 신호 종류로 층화)
    ② 그 짝 — 결박됨 **≥ 50** · 강 신호 부분모집단의 미결박 **≤ 10%**
    ③ 자연어 유사도를 안 쓴다 — **CI 가 진다**(`cargo xtask check` 14)
    ④ 승인 없이 `asserted` 가 되는 경로 부재 — **타입과 CI 가 진다**(검사 15)
    ⑤ 동점 후보를 확정하지 않는다 — **일부러 동점을 만들어** 센다
    ⑥ 조각화가 결정적이고 두 번째 인입이 개체를 안 만든다
    ⑦ 인입이 커밋 트리를 읽는다 — 워킹트리를 바꿔도 산출이 안 바뀐다
    ⑧ 일괄 승인이 걸리는 것을 거부한다 · 거부가 재구축을 넘어 살아남는다
    ★ ⑨ **F09 ⑤ 재측정** — 실제 문서 조각으로 결박을 걸고 거짓 양성률을 다시 잰다

**「문서를 좌표에 건다」는 말하기 가장 쉽다.** 아무것도 안 거는 인입기도, 전부 거는
인입기도 그 문장을 만족한다 — 그래서 ①과 ②가 서로를 막고, **②를 「미결박의 상한」으로
걸어** ⑤와 싸우지 않게 했다(`[f10.pass].resolution_floor_grounds`).

# 대조가 꺼지는 형태 — 이 스크립트가 막는 것 다섯

  · **열여섯·열일곱(변형이 대상을 안 건드림 · 종류를 안 봄)** — 문서 변형을 **종류별로**
    하고, **무엇이 바뀌어야 하는지를 먼저 적고** 그대로 나오는지 센다. 바뀐 파일 수와
    바이트 차이가 0 이면 멈춘다
  · **다섯째(도구가 무엇을 읽는지)** — ⑦이 워킹트리를 바꿔 놓고 같은 `--at` 으로 다시
    묻는다. 산출이 바뀌면 커밋 트리를 안 읽는 것이다
  · **둘째(공유 상태)** — 방마다 작업 사본과 캐시·2층·의도 저장소가 따로다
  · **편의 표본을 증거로 쓰기** — 코퍼스마다 **갈라 적고**, palimpsest 는 `--self` 로만
    돈다. 이 빌드에 Rust 추출기가 없어 **판정 대상이 비어 있기 때문이다**
  · **분모를 우리가 고르기** — 제외 규칙(`.claude/`·`.ditto/`·`.github/`)이
    **재기 전에 등록됐고**, 뺀 수를 함께 적는다

사용:
    ./scripts/f10-verify.py                # 전부 (약 3분)
    ./scripts/f10-verify.py --skip-f09     # ⑨(F09 재측정)를 건너뛴다 — **대조 불가로 적힌다**
    ./scripts/f10-verify.py --self         # palimpsest 자기 인입도 함께 낸다 (편의 표본)
"""

from __future__ import annotations

import argparse
import json
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
BIN = ROOT / "target/release/pal"

DITTO = Path.home() / "dev/projects/ditto"
DITTO_PIN = "aded7ce7f88f"
PORTAL = Path.home() / "dev/projects/boxwood/portal-backend"
PORTAL_PIN = "a29cad0bf6a8"

# ── `[f10.pass]` 가 등록한 값들. **여기서 정하지 않는다 — 옮겨 적을 뿐이다** ────────
FALSE_BINDING_MAX_PCT = 5
FALSE_BINDING_SAMPLE = 50
BOUND_COUNT_MIN = 50
UNBOUND_MAX_PCT_IN_STRONG = 10
BATCH_REFUSAL_MIN = 1
F09_FALSE_POSITIVE_MAX_PCT = 10
F09_SAMPLE_MIN = 20
# 코퍼스당 표본 — `[f09.4].sample_selection` 규칙 1 그대로(둘에서 각 10, 합 20).
SAMPLES_PER_CORPUS = 10

# ── 하한 — **시험되지 않은 대조는 `–` 가 아니라 실패다** (`2e2eb3f`) ──────────────
MIN_FRAGMENTS = 1
MIN_CHANGED_FILES = 1
MIN_CHANGED_BYTES = 1

# ⚠ **재기 전에 등록된 제외 규칙**(`[f10].input_quality`). 에이전트 지시·스킬 정의·
# 워크플로는 **그 저장소의 서술물이 아니라 도구 설정**이다. 넣으면 해소율의 분모가
# 코드와 무관한 것으로 채워져 이 값이 **저장소가 아니라 우리가 쓰는 도구를 잰다.**
TOOL_DIRS = (".claude/", ".ditto/", ".github/")

# **확정할 수 있는 신호** — 거리가 0 인 것(`ResolutionSignal::can_confirm_subject`).
#
# ⚠ **2026-08-15 에 넷에서 둘로 줄었다** (`[f10.5].signal_ruling` · #59).
# 앞선 판은 `fenced-path`·`span` 도 여기 담고 *"조회이지 판단이 아니므로 정의상 거짓일
# 수 없다"* 라고 적었는데, **실측이 그것을 반증했다**(`span` 48.9%) — [ADR-0015].
# **이 상수를 안 고치면 ①의 층화 판정이 없어진 함수의 뜻으로 계속 돈다.**
CONFIRMED = {"attached", "frontmatter"}

결과: list[tuple[str, str, str]] = []  # (표시, 이름, 값)


def ok(name: str, value: str) -> None:
    결과.append(("ok", name, value))


def fail(name: str, value: str) -> None:
    결과.append(("FAIL", name, value))


def skip(name: str, value: str) -> None:
    결과.append(("–", name, value))


def run(cmd: list[str], cwd: Path | None = None) -> subprocess.CompletedProcess:
    return subprocess.run(cmd, cwd=cwd, capture_output=True, text=True, check=False)


def pal(args: list[str], repo: Path, box: Path, at: str | None = None) -> dict:
    """`pal` 하나. **방마다 캐시·2층·의도가 따로다** — 공유 상태가 대조를 껐던 자리다."""
    cmd = [str(BIN), *args, "--repo", str(repo),
           "--cache-dir", str(box / "cache"), "--index", str(box / "index.redb"),
           "--intent", str(box / "intent.redb")]
    if at:
        cmd += ["--at", at]
    p = run(cmd)
    if p.returncode != 0:
        raise SystemExit(f"실패: pal {' '.join(args)}\n{p.stderr[-800:]}")
    return json.loads(p.stdout) if "--json" in args else {}


def pal_or_none(args: list[str], repo: Path, box: Path, at: str | None = None):
    """실패해도 안 멈춘다 — **실패 자체가 답인 자리**(거부·후보 밖 승인)에서 쓴다."""
    cmd = [str(BIN), *args, "--repo", str(repo),
           "--cache-dir", str(box / "cache"), "--index", str(box / "index.redb"),
           "--intent", str(box / "intent.redb")]
    if at:
        cmd += ["--at", at]
    return run(cmd)


def 사본(tmp: Path, tag: str, src: Path, pin: str) -> tuple[Path, Path]:
    """**이름을 고정한다** — 매니페스트가 없으면 `repo_id` 가 디렉터리 이름에서 온다.

    회차마다 이름이 달라지면 좌표가 전부 달라지고, 그러면 이 대조는 무엇을 재든
    「움직였다」를 낸다(F03-1 게이트 §4 에서 실제로 걸린 자리다).
    """
    box = tmp / tag
    box.mkdir(parents=True)
    repo = box / "corpus"
    p = run(["git", "clone", "--local", "--no-checkout", "-q", str(src), str(repo)])
    if p.returncode != 0:
        raise SystemExit(f"사본을 만들지 못했다: {p.stderr[-300:]}")
    run(["git", "-C", str(repo), "checkout", "-q", pin])
    return repo, box


def 인입(repo: Path, box: Path, at: str | None = None) -> dict:
    return pal(["narrative", "--json"], repo, box, at)


def 도구설정인가(path: str) -> bool:
    return path.startswith(TOOL_DIRS)


def 걸러낸다(got: dict) -> tuple[list[dict], int]:
    """도구 설정 아래를 뺀다. **뺀 수를 함께 낸다 — 조용한 절단 금지.**"""
    안 = [p for p in got["proposals"] if not 도구설정인가(p["fragment"]["path"])]
    return 안, len(got["proposals"]) - len(안)


def 분류(p: dict) -> str:
    return p["class"]["class"]


def 신호(p: dict) -> str | None:
    return p["class"].get("by")


# ═════════════════════════════════════════════════════════════════════════════
# ①② 해소 — **서로를 막는 짝**
# ═════════════════════════════════════════════════════════════════════════════

def 해소(tmp: Path, tag: str, src: Path, pin: str) -> dict:
    print(f"①② 해소 · {tag}")
    repo, box = 사본(tmp, f"resolve-{tag}", src, pin)
    got = 인입(repo, box, pin)
    쓸것, 뺀수 = 걸러낸다(got)

    if len(쓸것) < MIN_FRAGMENTS:
        fail(f"①② {tag}", f"조각이 {len(쓸것)}개다 — 이 대조는 아무것도 안 잰다")
        return {}

    분포 = {"bound": 0, "candidates": 0, "unbound": 0}
    for p in 쓸것:
        분포[분류(p)] += 1

    # ★ **강 신호 부분모집단** — 확인된 신호를 **낼 수 있었던** 조각들.
    #
    # 분모를 「확인된 신호로 걸린 것」으로 잡으면 그것은 **결과로 분모를 정하는 것**이다
    # (미결박은 정의상 안 들어간다). 그래서 **조각이 든 신호의 날것**으로 잡는다 —
    # 펜스 경로나 인라인 스팬을 하나라도 가진 조각.
    강 = [p for p in 쓸것
          if p["fragment"]["signals"]["fenced_paths"] or p["fragment"]["signals"]["spans"]
          or p["fragment"]["signals"]["grounds"] or p["fragment"]["signals"]["attached"]]
    강_미결박 = [p for p in 강 if 분류(p) == "unbound"]
    비율 = round(100 * len(강_미결박) / len(강), 1) if 강 else 0.0

    값 = (f"조각 {len(쓸것)} (도구 설정 {뺀수} 제외) · 결박됨 {분포['bound']} · "
          f"후보 {분포['candidates']} · 미결박 {분포['unbound']}")
    ok(f"①② {tag} 분포", 값)

    if not 강:
        skip(f"② {tag} 강 신호", "강 신호를 가진 조각이 0 개다 — **모집단 0 이라 대조 불가**")
    elif 비율 > UNBOUND_MAX_PCT_IN_STRONG:
        fail(f"② {tag} 강 신호 미결박",
             f"{len(강_미결박)}/{len(강)} = {비율}% (상한 {UNBOUND_MAX_PCT_IN_STRONG}%) — "
             f"**계단식이 확인된 신호를 쓰고도 못 걸었다.** R-09 의 실측이 아니라 구현의 반증이다")
    else:
        ok(f"② {tag} 강 신호 미결박",
           f"{len(강_미결박)}/{len(강)} = {비율}% (상한 {UNBOUND_MAX_PCT_IN_STRONG}%)")

    # **3분류 셋이 전부 산출되는가** — 하한이 없으면 전부 미결박으로 내는 인입기가 통과한다.
    없는 = [k for k, v in 분포.items() if v == 0]
    if 없는:
        fail(f"② {tag} 3분류", f"산출되지 않은 분류 {없는} — 하한이 없으면 ①이 공짜다")
    else:
        ok(f"② {tag} 3분류", f"셋 다 산출 {분포}")

    # ★ **후보를 좁혔는가** — 수만 내면 「후보 있음 N」이 「승인 대기 N」으로 읽힌다.
    퍼짐: dict[str, list[int]] = {}
    for p in 쓸것:
        if 분류(p) == "candidates":
            퍼짐.setdefault(신호(p), []).append(len(p["class"]["candidates"]))
    for by in sorted(퍼짐):
        s = sorted(퍼짐[by])
        ok(f"   {tag} 후보 퍼짐 {by}",
           f"조각 {len(s)} · 중앙 {s[len(s) // 2]} · 최대 {s[-1]} · 셋 이하 {sum(1 for x in s if x <= 3)}")

    return {"proposals": 쓸것, "counts": 분포, "repo": repo, "box": box, "excluded": 뺀수}


def 표본(전체: dict[str, dict]) -> None:  # noqa: C901
    """①의 표본 — **`[f10.2].sample_selection` 규칙 그대로.**

    판정은 사람(에이전트)이 하고 **조각마다 한 줄의 근거를 게이트에 적는다.**
    여기서는 **표본을 만들고 층화가 성립하는지**까지 센다 — 판정 자체는 게이트다.
    """
    print("① 거짓 결박률 표본 — 신호 종류로 층화한다")
    걸린 = [(t, p) for t, d in 전체.items() if d for p in d["proposals"] if 분류(p) == "bound"]
    if len(걸린) < BOUND_COUNT_MIN:
        fail("② 결박 하한",
             f"결박됨이 {len(걸린)}건이다 (하한 {BOUND_COUNT_MIN}) — "
             f"**①이 자기 정의(표본 {FALSE_BINDING_SAMPLE})대로 못 재어진다. 통과가 아니라 대조 불가다**")
        return
    ok("② 결박 하한", f"결박됨 {len(걸린)}건 ≥ {BOUND_COUNT_MIN}")

    # **`EntityId` 사전순 등간격** — 우리가 고르지 않는다(`[f10.2].sample_selection` 규칙 3).
    걸린.sort(key=lambda x: x[1]["item"]["id"])
    n = min(FALSE_BINDING_SAMPLE, len(걸린))
    간격 = max(1, len(걸린) // n)
    뽑은 = [걸린[i * 간격] for i in range(n) if i * 간격 < len(걸린)]

    판단드는것 = [x for x in 뽑은 if 신호(x[1]) not in CONFIRMED]
    값 = f"표본 {len(뽑은)} · 판단이 드는 신호로 걸린 것 {len(판단드는것)}"
    # ★ **확인된 신호만 담으면 ①이 0 으로 나오고 아무것도 안 잰다**(규칙 2).
    if len(판단드는것) * 2 < len(뽑은):
        skip("① 층화",
             f"{값} — **표본의 절반이 확인된 신호다.** 그만큼은 정의상 거짓일 수 없고 "
             f"**대조 불가로 센다**(`[f10.2].sample_selection` 규칙 2)")
    else:
        ok("① 층화", 값)

    # ★ **좌표의 이름과 경로를 함께 적는다.** 없으면 손 검토가 불가능하다 —
    # *"이 조각이 이 코드에 관한 것인가"* 는 좌표 해시를 보고 답할 수 없다.
    이름 = {}
    for tag, d in 전체.items():
        if not d:
            continue
        dump = pal(["query", "graph.dump", "--json"], d["repo"], d["box"],
                   DITTO_PIN if tag == "ditto" else PORTAL_PIN)
        for n in dump["answer"]["nodes"]:
            이름[n["id"]] = (n["name"], n["path"], n["kind"])

    out = ROOT / "corpus/tasks/f10-false-binding-sample.tsv"
    with out.open("w", encoding="utf-8") as f:
        f.write("코퍼스\t개체\t문서\t앵커\t걸린신호\t심볼\t종류\t코드경로\t문서첫줄\t판정\t근거\n")
        for tag, p in 뽑은:
            머리 = (p["fragment"]["body"].splitlines() or [""])[0][:90].replace("\t", " ")
            nm, pth, kind = 이름.get(p["class"]["target"], ("?", "?", "?"))
            f.write(f"{tag}\t{p['item']['id']}\t{p['fragment']['path']}\t{p['fragment']['anchor']}\t"
                    f"{신호(p)}\t{nm}\t{kind}\t{pth}\t{머리}\t\t\n")
    ok("① 표본 파일", f"{out.relative_to(ROOT)} — **판정은 게이트에 조각마다 한 줄로**")


# ═════════════════════════════════════════════════════════════════════════════
# ⑤ 동점 — **일부러 만든다.** 자연 발생을 기다리면 대조가 조용히 꺼진다
# ═════════════════════════════════════════════════════════════════════════════

def 동점(tmp: Path) -> None:
    print("⑤ ★ 동점 후보를 확정하지 않는다 — **셋을 일부러 만든다**")
    repo, box = 사본(tmp, "tie", DITTO, DITTO_PIN)

    # 코퍼스에서 **같은 이름이 둘 이상인 심볼**을 찾는다 — 우리가 만들지 않는다.
    dump = pal(["query", "graph.dump", "--json"], repo, box, DITTO_PIN)
    이름별: dict[str, list[dict]] = {}
    for n in dump["answer"]["nodes"]:
        이름별.setdefault(n["name"], []).append(n)
    중복 = sorted((k for k, v in 이름별.items() if len(v) >= 2 and k.isidentifier()),
                  key=lambda k: (-len(이름별[k]), k))
    두경로 = sorted({n["path"] for n in dump["answer"]["nodes"]})

    # ⚠ **「확정」 픽스처가 거리 0 인 신호로 바뀌었다** (2026-08-15 · `[f10.5].tie_grounds`).
    #
    # 앞선 판은 **유일한 스팬**으로 확정을 만들었는데, `span` 은 이제 거리가 있어서
    # **유일해도 확정하지 않는다.** 픽스처를 안 고치면 이 대조가 *"확정을 못 했다"* 로
    # 어긋나고, 그것은 동점 처리가 깨진 것이 아니라 **재료가 바뀐 것**이다.
    # **이것이 「대조가 꺼지는 열아홉째」가 이 자리에 실제로 걸린 모습이다** —
    # 등록(`[f10.pass]` ④)은 그대로이고 **픽스처가 그 등록을 따라간다.**
    #
    # 거리 0 인 신호로 확정을 만드는 법: 프론트매터에 `경로#이름` 을 적는다.
    확정파일, 확정이름 = None, None
    파일별: dict[str, list[dict]] = {}
    for n in dump["answer"]["nodes"]:
        파일별.setdefault(n["path"], []).append(n)
    for path, v in sorted(파일별.items()):
        나온 = [x["name"] for x in v]
        for nm in sorted(set(나온)):
            if 나온.count(nm) == 1:
                확정파일, 확정이름 = path, nm
                break
        if 확정파일:
            break

    if not 중복 or len(두경로) < 2 or not 확정파일:
        skip("⑤ 동점", "코퍼스에 동점을 만들 재료가 없다 — **모집단 0 이라 대조 불가**")
        return

    doc = repo / "docs" / "f10-tie.md"
    doc.parent.mkdir(parents=True, exist_ok=True)
    doc.write_text(
        # ③ 확정 — **거리 0 인 신호로.** 프론트매터는 **문서 첫 조각에만** 걸리므로
        #    맨 앞에 둔다. **이것이 없으면 「전부 후보로 낸다」가 통과한다.**
        f"---\ngrounds: [\"{확정파일}#{확정이름}\"]\n---\n\n"
        f"# 확정\n\n`{확정이름}` 하나를 가리킨다.\n\n"
        # ① 인라인 스팬 동점 — 같은 이름이 여럿
        f"# 스팬 동점\n\n`{중복[0]}` 를 가리킨다.\n\n"
        # ② 펜스 경로 동점 — 서로 다른 경로 둘
        f"# 경로 동점\n\n```\n{두경로[0]}\n{두경로[1]}\n```\n",
        encoding="utf-8",
    )
    run(["git", "-C", str(repo), "add", "-A"])
    run(["git", "-C", str(repo), "-c", "user.email=t@e", "-c", "user.name=t",
         "commit", "-q", "-m", "f10 동점 픽스처"])
    head = run(["git", "-C", str(repo), "rev-parse", "HEAD"]).stdout.strip()

    got = 인입(repo, box, head)
    내것 = {p["fragment"]["anchor"]: p for p in got["proposals"]
            if p["fragment"]["path"] == "docs/f10-tie.md"}
    if len(내것) < 3:
        fail("⑤ 동점", f"픽스처 조각이 {len(내것)}개다 — 변형이 안 먹었다")
        return

    걸림 = []
    for 앵커, 기대 in [("스팬-동점", "candidates"), ("경로-동점", "candidates"), ("확정", "bound")]:  # noqa: B007
        p = 내것.get(앵커)
        if p is None:
            걸림.append(f"{앵커}: 조각이 없다")
            continue
        났다 = 분류(p)
        if 앵커 == "확정":
            # ★ **반대 방향의 반대 방향** — 확정할 수 있는 것을 확정 못 하면 그것도 반증이다.
            if 났다 != "bound":
                걸림.append(f"{앵커}: {났다} — **유일한데 확정을 못 했다**")
        elif 났다 == "bound":
            걸림.append(f"{앵커}: bound — **동점을 확정했다. 이것이 거짓 결박이다**")
        elif 났다 == "unbound":
            # ★ 미결박으로 접어도 반증이다 — *"여럿이라 못 좁혔다"* 와 *"신호가 없다"* 는 다르다.
            걸림.append(f"{앵커}: unbound — **동점을 미결박으로 접었다**")

    if 걸림:
        fail("⑤ 동점", " · ".join(걸림))
    else:
        ok("⑤ 동점", f"스팬 동점 {len(내것['스팬-동점']['class']['candidates'])}후보 · "
                     f"경로 동점 {len(내것['경로-동점']['class']['candidates'])}후보 · 확정 1")


# ═════════════════════════════════════════════════════════════════════════════
# ⑥ 결정성 — **흔들리면 거부 기록이 아무것도 안 가린다**
# ═════════════════════════════════════════════════════════════════════════════

def 결정성(tmp: Path) -> None:
    print("⑥ 조각화가 결정적이고 두 번째 인입이 개체를 안 만든다")
    repo, box = 사본(tmp, "det", DITTO, DITTO_PIN)
    첫 = 인입(repo, box, DITTO_PIN)
    둘 = 인입(repo, box, DITTO_PIN)

    if 첫["fragments"] < MIN_FRAGMENTS:
        fail("⑥ 결정성", "조각이 0 개다 — 「같다」가 공짜다")
        return
    if 첫["minted"] < 1:
        fail("⑥ 결정성", "첫 인입에서 만들어진 개체가 0 이다 — 아래 단언이 아무것도 안 센다")
        return

    if 둘["minted"] != 0:
        fail("⑥ 개체 왕복",
             f"두 번째 인입이 개체 {둘['minted']}개를 새로 만들었다 — "
             f"**읽기가 더하기가 아니라 복제가 된다**(`[f05.4]` ②)")
    else:
        ok("⑥ 개체 왕복", f"첫 인입 {첫['minted']}개 · 두 번째 **0개**")

    가 = json.dumps(첫["proposals"], sort_keys=True, ensure_ascii=False)
    나 = json.dumps(둘["proposals"], sort_keys=True, ensure_ascii=False)
    if 가 != 나:
        fail("⑥ 조각화 결정성", "두 회차의 제안이 다르다 — 흔들리는 목록은 승인의 근거가 못 된다")
    else:
        ok("⑥ 조각화 결정성", f"조각 {첫['fragments']} · 두 회차가 **글자까지 같다**")


# ═════════════════════════════════════════════════════════════════════════════
# ⑦ 무엇을 읽는가 — **대조가 꺼지는 다섯째**
# ═════════════════════════════════════════════════════════════════════════════

def 읽는_트리(tmp: Path) -> None:
    print("⑦ ★ 인입이 커밋 트리를 읽는다 — 워킹트리를 바꿔도 산출이 안 바뀐다")
    repo, box = 사본(tmp, "tree", DITTO, DITTO_PIN)
    전 = 인입(repo, box, DITTO_PIN)

    # 워킹트리의 문서를 바꾼다 — **커밋하지 않는다.**
    문서 = [p["fragment"]["path"] for p in 전["proposals"]][:1]
    if not 문서:
        skip("⑦ 읽는 트리", "문서 조각이 없다 — **모집단 0 이라 대조 불가**")
        return
    f = repo / 문서[0]
    이전 = f.read_bytes()
    f.write_bytes(이전 + "\n\n# 워킹트리에만 있는 헤딩\n\n`없는이름` 을 가리킨다.\n".encode())
    바뀐바이트 = len(f.read_bytes()) - len(이전)
    if 바뀐바이트 < MIN_CHANGED_BYTES:
        fail("⑦ 읽는 트리", "워킹트리 변형이 파일을 안 바꿨다 — 「안 바뀌었다」가 공짜다")
        return

    후 = 인입(repo, box, DITTO_PIN)
    if 후["fragments"] != 전["fragments"]:
        fail("⑦ 읽는 트리",
             f"조각이 {전['fragments']} → {후['fragments']} 로 바뀌었다 — "
             f"**`--at` 을 줬는데 워킹트리를 읽는다**")
    else:
        ok("⑦ 읽는 트리",
           f"워킹트리 +{바뀐바이트}바이트 · 조각 {후['fragments']} **그대로** · 새 개체 {후['minted']}")


# ═════════════════════════════════════════════════════════════════════════════
# ④⑧ 승인·거부 — **세탁 금지와 재질문 제거**
# ═════════════════════════════════════════════════════════════════════════════

def 승인과_거부(tmp: Path) -> None:
    print("④⑧ 승인 · 거부 · 일괄")
    repo, box = 사본(tmp, "approve", DITTO, DITTO_PIN)
    got = 인입(repo, box, DITTO_PIN)
    걸린 = [p for p in got["proposals"] if 분류(p) == "bound"]
    여럿 = [p for p in got["proposals"] if 분류(p) == "candidates"]
    미결박 = [p for p in got["proposals"] if 분류(p) == "unbound"]
    if not 걸린 or not 여럿 or not 미결박:
        skip("④⑧", "세 분류가 다 안 나왔다 — **모집단이 모자라 대조 불가**")
        return

    한개 = 걸린[0]
    이름 = f"decision/{한개['item']['id']}"

    # ★ ④ **후보 밖의 좌표는 승인할 수 없다** — 세탁을 막는 자리.
    r = pal_or_none(["narrative", "--approve", 이름, "--pick", "0" * 64], repo, box, DITTO_PIN)
    if r.returncode == 0:
        fail("④ 후보 밖 승인", "후보 밖의 좌표를 승인했다 — **지어낸 결박이 `asserted` 가 됐다**")
    else:
        ok("④ 후보 밖 승인", "거부됐다 — " + r.stderr.strip().splitlines()[-1][:90])

    # ★ ④ **미결박은 승인할 것이 없다.**
    빈것 = f"decision/{미결박[0]['item']['id']}"
    r = pal_or_none(["narrative", "--approve", 빈것], repo, box, DITTO_PIN)
    if r.returncode == 0:
        fail("④ 미결박 승인", "후보가 없는데 승인됐다")
    else:
        ok("④ 미결박 승인", "거부됐다 — 승인할 것이 없다")

    # 승인 — **새 `asserted` 결박이 생기고 `promoted_by` 가 제안을 가리킨다.**
    r = pal_or_none(["narrative", "--approve", 이름], repo, box, DITTO_PIN)
    if r.returncode != 0:
        fail("④ 승인", f"승인이 실패했다: {r.stderr[-200:]}")
        return
    상태 = pal(["query", "binding.status", "--json"], repo, box, DITTO_PIN)
    묶음 = 상태["answer"]["bindings"]
    if len(묶음) != 1:
        fail("④ 승인", f"결박이 {len(묶음)}건이다 — 하나여야 한다")
        return
    ok("④ 승인", f"결박 1건 · 개체 {묶음[0]['subject']} · 상태 {묶음[0]['status']['code']['freshness']}")

    # ★ ⑧ **거부가 남고 다시 묻지 않는다.**
    거부할것 = 여럿[0]
    후보 = 거부할것["class"]["candidates"][0]
    이름2 = f"decision/{거부할것['item']['id']}"
    r = pal_or_none(["narrative", "--refuse", 이름2, "--pick", 후보, "--reason", "다른 것에 관한 문서다"],
                    repo, box, DITTO_PIN)
    if r.returncode != 0:
        fail("⑧ 거부", f"거부가 실패했다: {r.stderr[-200:]}")
        return
    # 같은 짝을 승인해 보면 **막혀야 한다.**
    r = pal_or_none(["narrative", "--approve", 이름2, "--pick", 후보], repo, box, DITTO_PIN)
    if r.returncode == 0:
        fail("⑧ 거부가 안 가린다", "거부한 짝이 승인됐다 — **재질문 제거가 성립하지 않는다**")
    else:
        ok("⑧ 거부", "거부한 짝의 승인이 막혔다 — " + r.stderr.strip().splitlines()[-1][:70])

    # ★ ⑧ **2층을 지워도 거부가 남는다** — R-21 의 자리.
    shutil.rmtree(box / "index.redb", ignore_errors=True)
    (box / "index.redb").unlink(missing_ok=True)
    r = pal_or_none(["narrative", "--approve", 이름2, "--pick", 후보], repo, box, DITTO_PIN)
    if r.returncode == 0:
        fail("⑧ 거부가 재구축을 못 넘는다", "2층을 지우자 거부가 사라졌다 — **R-21 의 자리다**")
    else:
        ok("⑧ 거부가 재구축을 넘는다", "2층을 지워도 거부한 짝이 여전히 막힌다")

    # ★ ⑧ **일괄 승인이 걸리는 것을 거부한다.**
    경로 = 여럿[0]["fragment"]["path"].rsplit("/", 1)[0] + "/"
    r = pal_or_none(["narrative", "--all-of", 경로], repo, box, DITTO_PIN)
    if r.returncode == 0:
        fail("⑧ 일괄", f"`{경로}` 에 후보 여럿인 조각이 있는데 일괄 승인이 통과했다")
    else:
        마지막 = [l for l in r.stderr.strip().splitlines() if l.strip()][0]
        ok("⑧ 일괄 거부", 마지막[:110])


# ═════════════════════════════════════════════════════════════════════════════
# ★ ⑨ F09 ⑤ 재측정 — **남의 게이트에 소급할 값이다**
# ═════════════════════════════════════════════════════════════════════════════

def f09_재측정(tmp: Path, tag: str, src: Path, pin: str, ext: str) -> None:
    """실제 문서 조각을 메모로 결박을 걸고 **앞으로 오면서** 켜지는 것을 센다.

    `[f09.4].sample_selection` 을 그대로 쓴다. **바꾸는 것은 하나뿐이다** —
    메모가 합성(`"계약 <심볼이름>"`)이 아니라 **F10 이 인입한 실제 문서 조각**이다.

    ⚠ **규칙 2 가 이 함수의 절반이다**: *"결박된 심볼이 실린 파일을 건드린 커밋만 센다.
    안 그러면 표본이 전부 「아무 일도 안 일어남」이 된다."* 처음 썼을 때 그것을 안 지켜
    **결박 14 · 켜진 것 2** 가 나왔다 — 등록이 미리 적어 둔 그대로였다.
    """
    print(f"⑨ ★ F09 ⑤ 재측정 · {tag} — **실제 문서 조각을 메모로**")
    repo, box = 사본(tmp, f"f09-{tag}", src, pin)

    커밋들 = run(["git", "-C", str(repo), "log", "--no-merges", "--format=%H", pin]).stdout.split()
    커밋들 = 커밋들[:120]
    if len(커밋들) < 20:
        skip(f"⑨ {tag}", f"머지 아닌 커밋이 {len(커밋들)}개뿐이다 — **대조 불가**")
        return
    시작 = 커밋들[-1]

    # ★ **규칙 2** — 이 창에서 실제로 건드려진 파일들. 안 걸면 표본이 전부
    #   「아무 일도 안 일어남」이 된다.
    건드린 = set(run(["git", "-C", str(repo), "log", "--no-merges", "--name-only",
                      "--format=", f"{시작}..{pin}"]).stdout.split())

    # 1) 인입해서 결박됨을 얻고, **좌표의 경로**를 붙인다.
    got = 인입(repo, box, pin)
    dump = pal(["query", "graph.dump", "--json"], repo, box, pin)
    경로 = {n["id"]: n["path"] for n in dump["answer"]["nodes"]}
    이름 = {n["id"]: n["name"] for n in dump["answer"]["nodes"]}

    걸린 = [p for p in got["proposals"]
            if 분류(p) == "bound"
            and not 도구설정인가(p["fragment"]["path"])
            # ★ 규칙 2 — 좌표가 실린 파일이 이 창에서 건드려졌는가.
            and 경로.get(p["class"]["target"]) in 건드린]
    if not 걸린:
        skip(f"⑨ {tag}", "창 안에서 건드려진 파일의 결박됨이 0 건이다 — **대조 불가**")
        return

    # 2) **`EntityId` 사전순 등간격** — 우리가 고르지 않는다(F09 규칙 4 와 같은 형태).
    걸린.sort(key=lambda p: p["item"]["id"])
    n = min(SAMPLES_PER_CORPUS, len(걸린))
    간격 = max(1, len(걸린) // n)
    뽑은 = [걸린[i * 간격] for i in range(n) if i * 간격 < len(걸린)]

    # 3) 시작 커밋에 건다. **좌표가 그때 없으면 건너뛴다** — 없는 것을 결박했다고
    #    적으면 그것이 곧 거짓이다(F09 가 방향을 거꾸로 했다가 전부 `orphaned` 였다).
    걸린수 = 0
    메모들: dict[str, str] = {}
    for p in 뽑은:
        후보이름 = [s for s in p["fragment"]["signals"]["spans"]
                    if s.replace(".", "").replace("#", "").isidentifier()]
        # **문서가 가리킨 이름**으로 건다 — 좌표로는 못 부른다.
        후보이름 = [이름.get(p["class"]["target"], "")] + 후보이름
        메모 = p["fragment"]["body"][:400]
        for nm in [x for x in 후보이름 if x]:
            r = pal_or_none(["bind", nm, "--note", 메모, "--radius", "symbol"], repo, box, 시작)
            if r.returncode == 0:
                걸린수 += 1
                메모들[nm] = p["fragment"]["path"] + "#" + p["fragment"]["anchor"]
                break

    if 걸린수 < 1:
        skip(f"⑨ {tag}", "시작 커밋에 걸린 결박이 0 건이다 — **대조 불가**")
        return

    # 4) 앞으로 오면서 처음 켜지는 자리를 모은다.
    표본: list[tuple] = []
    이미: set[str] = set()
    메모전체: dict[str, str] = {}
    심볼이름: dict[str, str] = {}
    for c in reversed(커밋들[:-1]):
        상태 = pal(["query", "binding.status", "--json"], repo, box, c)
        for b in 상태["answer"]["bindings"]:
            f = b["status"]["code"]["freshness"]
            if f in ("stale", "orphaned") and b["binding"] not in 이미:
                이미.add(b["binding"])
                제목 = run(["git", "-C", str(repo), "log", "-1", "--format=%s", c]).stdout.strip()
                짧은 = b["binding"][:8]
                메모전체[짧은] = b["note"]
                심볼이름[짧은] = 이름.get(b["target"], b["target"][:12])
                표본.append((c[:8], 짧은, f, b["note"].splitlines()[0][:56], 제목[:56]))

    print(f"      결박 {걸린수} · 커밋 {len(커밋들)} 훑음 · 켜진 것 {len(표본)}")
    for row in 표본:
        print("      " + " | ".join(str(x) for x in row))

    # ★ **판정 재료를 파일로 남긴다** — 옛 F09 §4-가 가 *"결박마다 한 줄"* 로 적은 그것이고,
    #   메모 전체가 없으면 *"이 커밋이 이 메모를 무효로 만드는가"* 를 판정할 수 없다.
    out = ROOT / f"corpus/tasks/f10-f09-remeasure-{tag}.tsv"
    with out.open("w", encoding="utf-8") as fh:
        fh.write("커밋\t결박\t갈래\t심볼\t커밋제목\t메모첫줄\t메모\t판정\t근거\n")
        for c, bid, fresh, 첫줄, 제목 in 표본:
            전체 = 메모전체.get(bid, "")
            fh.write(f"{c}\t{bid}\t{fresh}\t{심볼이름.get(bid, '?')}\t{제목}\t{첫줄}\t"
                     f"{전체[:600].replace(chr(9), ' ').replace(chr(10), ' / ')}\t\t\n")

    # ★ **등록된 하한** — 재측정의 표본이 모자라면 **대조 불가이고 통과가 아니다.**
    #   채우려고 편의 표본을 섞지 않는다(`[f10].f09_remeasurement`).
    if 걸린수 < SAMPLES_PER_CORPUS:
        skip(f"⑨ {tag}",
             f"결박이 {걸린수}건뿐이다 (코퍼스당 {SAMPLES_PER_CORPUS}) — **대조 불가**. "
             f"켜진 것 {len(표본)}")
    else:
        ok(f"⑨ {tag} 표본",
           f"결박 {걸린수} · 켜진 것 {len(표본)} — **거짓 양성 판정은 게이트에 결박마다 한 줄로**")


# ═════════════════════════════════════════════════════════════════════════════

def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--skip-f09", action="store_true",
                    help="⑨(F09 재측정)를 건너뛴다 — **대조 불가로 적힌다**")
    ap.add_argument("--self", dest="self_", action="store_true",
                    help="palimpsest 자기 인입도 낸다 — **편의 표본이고 판정 대상이 비어 있다**")
    a = ap.parse_args()

    if not BIN.exists():
        raise SystemExit(f"바이너리가 없다: {BIN} — `cargo build --release` 먼저")

    print("F10 — 서술물 인입")
    tmp = Path(tempfile.mkdtemp(prefix="pal-f10-"))
    전체 = {}
    try:
        for tag, src, pin in [("ditto", DITTO, DITTO_PIN), ("portal-backend", PORTAL, PORTAL_PIN)]:
            if not src.exists():
                skip(f"①② {tag}", f"코퍼스가 없다: {src} — **대조 불가**")
                continue
            전체[tag] = 해소(tmp, tag, src, pin)
        표본(전체)
        동점(tmp)
        결정성(tmp)
        읽는_트리(tmp)
        승인과_거부(tmp)

        if a.self_:
            # ⚠ **편의 표본이고 그것을 넘어 판정 대상이 비어 있다** — 이 빌드에 Rust
            # 추출기가 없어 palimpsest 의 2층 심볼이 한 자릿수다. 참고로만 낸다.
            got = 인입(ROOT, tmp / "self", None)
            skip("자기 저장소 인입 (R-19)",
                 f"문서 {got['docs']} · 조각 {got['fragments']} · {got['counts']} — "
                 f"⚠ **편의 표본이고 2층이 비어 있어 판정에 쓰지 않는다**")

        if a.skip_f09:
            skip("⑨ F09 ⑤ 재측정", "`--skip-f09` — **대조 불가**")
        else:
            for tag, src, pin, ext in [("ditto", DITTO, DITTO_PIN, ".ts"),
                                       ("portal-backend", PORTAL, PORTAL_PIN, ".kt")]:
                if src.exists():
                    f09_재측정(tmp, tag, src, pin, ext)
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
    if not 어긋남:
        print("어긋남 0")
    return 1 if 어긋남 else 0


if __name__ == "__main__":
    sys.exit(main())
