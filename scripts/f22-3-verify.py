#!/usr/bin/env python3
# ═════════════════════════════════════════════════════════════════════════════
# [f22.3] 의 판정 — 사슬의 마디 · 소급 결박
#
# 세 부분이다.
#   A. **등록된 오라클** — T10 이 손으로 잰 ditto 표본 5 건을 코드가 재현하는가
#   B. **비판정 관측** — 기계가 실제로 도는가(Kotlin 저장소). 사전 등록된 값이 아니다
#   C. **음성 대조** — 이 검사가 고장 났다면 어떻게 드러나는가
#
# 판정 기록: docs/gates/F22-3-chain-nodes.md · 합격선 corpus/criteria.toml [f22.3]
# ═════════════════════════════════════════════════════════════════════════════
import json
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
PAL = ROOT / "target" / "debug" / "pal"
DITTO = Path.home() / "dev" / "projects" / "ditto"
KOTLIN = Path.home() / "dev" / "projects" / "boxwood" / "portal-backend"

# T10 이 등록한 표본 — 시각 오름차순 등간격, g = floor(149/5) = 29
T10_SAMPLE = ["add91871", "2550118a", "afcfefab", "7f6b0a58", "9dc1af1e"]

# ditto 의 코퍼스 핀. **팁(e168ccdd)이 아니다** — corpus/manifest.toml `[[corpus]] id="ditto"`.
T10_AT = "aded7ce7f88feb3c03238c5f9760f3a2ade4a6c1"

# ── B 부의 모집단을 **고정 SHA 에 묶는다** (2026-08-13 · F02-1 이 고침) ────────
#
# # 무엇이 고장 나 있었나
#
# `sample_by_t10_rule` 이 `git log --grep '^fix'` 를 **rev 없이** 불렀다. 즉 모집단이
# 코퍼스의 **현재 HEAD** 에 달려 있었고, HEAD 가 전진하면서 `^fix` 가 **477 → 654** 로
# 자라 표본이 통째로 바뀌었다. `7fe6b62` 가 잡은 것과 같은 형태다 —
# **변이를 자라는 값에 묶으면 코퍼스가 자랄 때 조용히 꺼진다.**
#
# 그 결과 아래 음성 대조 ⑤ 가 `–`(시험되지 않음)를 찍는데도 스크립트는
# *"음성 대조 실패 0 건"* 이라고 적었고, 게이트는 그것을 **1/3** 으로 기록했다.
# **검사가 자기에 대해 거짓말하는 형태다.**
#
# 그래서 셋을 바꿨다:
#   ① 모집단을 `corpus/manifest.toml` 의 핀(`a29cad0bf6a8`)에 묶는다
#   ② **시험되지 않은 대조는 `–` 가 아니라 실패다.** 건너뛰면 0 건이 거짓이 된다
#   ③ 잘라낸 것이 있으면 수를 찍는다 — 조용한 절단 금지
KOTLIN_AT = "a29cad0bf6a8"

# ⑤(나머지 후보)를 시험할 커밋을 찾는 데 쓰는 상한. **조용히 자르지 않는다** —
# 이 수만큼 보고도 못 찾으면 `–` 가 아니라 **실패**로 적힌다.
OTHERS_PROBE_LIMIT = 30


def defect(repo: Path, rev: str, budget: int = 400) -> dict:
    out = subprocess.run(
        [str(PAL), "defect", rev, "--repo", str(repo), "--json", "--history-limit", str(budget)],
        capture_output=True, text=True, check=True,
    )
    return json.loads(out.stdout)


def outcome(report: dict) -> str:
    r = report["result"]
    return r["why"] if r["outcome"] == "missed" else "bound"


# 확장자 → 이 빌드에 추출기가 있는가. `pal_extract::capability` 와 **같은 표여야 한다**.
EXTRACTABLE = {"kt", "kts", "ts", "mts", "cts", "tsx"}
NOT_BUILT = {"java", "js", "mjs", "cjs", "jsx"}


def touched(repo: Path, rev: str) -> list[str]:
    return subprocess.run(
        ["git", "-C", str(repo), "diff-tree", "--no-commit-id", "--name-only", "-r", rev],
        capture_output=True, text=True, check=True).stdout.split()


def is_test_path(p: str) -> bool:
    """**규약이다. 판정이 아니다.** 무엇이 테스트인가를 정하는 것은 F15 다."""
    name = p.rsplit("/", 1)[-1]
    return (".test." in name or ".spec." in name
            or p.startswith("tests/") or "/tests/" in p or "/__tests__/" in p)


def code_files(repo: Path, rev: str) -> tuple[list[str], list[str]]:
    """이 커밋이 만진 **추출 가능한** 코드 파일 — (테스트, 실코드)."""
    files = [f for f in touched(repo, rev) if f.rsplit(".", 1)[-1] in EXTRACTABLE]
    return ([f for f in files if is_test_path(f)], [f for f in files if not is_test_path(f)])


def fix_population(repo: Path) -> list[str]:
    """`^fix` 모집단 — **고정 SHA 기준**. 시각 오름차순.

    `KOTLIN_AT` 을 빼면 HEAD 가 전진할 때마다 모집단이 자라고 표본이 통째로 바뀐다.
    """
    return subprocess.run(
        ["git", "-C", str(repo), "log", "--reverse", "--format=%H", "--grep", "^fix", KOTLIN_AT],
        capture_output=True, text=True, check=True,
    ).stdout.split()


def ditto_fix_population() -> list[str]:
    """ditto 의 `^fix` 모집단 — **T10 이 등록한 핀 기준.** 시각 오름차순.

    `recurrence.toml` `[selection].universe` 와 같은 규칙이 아니다. 여기서 필요한 것은
    *"추출기가 없는 언어만 만진 커밋"* 하나이고, 그것을 **결정적으로** 고르기만 하면 된다.
    """
    return subprocess.run(
        ["git", "-C", str(DITTO), "log", "--reverse", "--format=%H", "--grep", "^fix",
         T10_AT],
        capture_output=True, text=True, check=True,
    ).stdout.split()


def sample_by_t10_rule(log: list[str], n: int = 5) -> tuple[list[str], int]:
    """T10 과 **같은 선정 규칙** — 시각 오름차순 등간격."""
    if len(log) < n:
        raise SystemExit(
            f"모집단이 {len(log)} 건이라 등간격 {n} 을 뽑을 수 없다 — 코퍼스 핀({KOTLIN_AT})이 "
            "도달 가능한가. **표본을 줄여 넘어가지 않는다.**"
        )
    g = len(log) // n
    return [log[i * g] for i in range(n)], len(log)


def main() -> int:
    if not PAL.exists():
        print("먼저 `cargo build --workspace` 를 돌려야 한다"); return 1
    fail = 0

    # ── A. 등록된 오라클 ─────────────────────────────────────────────────────
    print("\n── A. 등록된 오라클 — T10 표본 5 건 (ditto @ aded7ce7) ──────────────")
    a = [defect(DITTO, c) for c in T10_SAMPLE]
    kinds = [outcome(r) for r in a]
    a_manifest = a_major = a_strict = 0
    test_only: list[str] = []
    for c, k, r in zip(T10_SAMPLE, kinds, a):
        line = f"  {c}  {k:20s}"
        if k == "no_extractor":
            cap = r["result"]["capability"]
            line += f" ← {cap['feature']} 미구축 ({cap['what']})"
        elif k == "bound":
            d = r["result"]
            intro = d["introduced_by"]
            a_manifest += 1
            line += f" 발현 {len(d['manifests_at']):3d}"
            if intro["outcome"] == "found":
                cf = intro["confidence"]
                pct = cf["agreeing"] * 100 // cf["total"] if cf["total"] else 0
                a_major += 1 if cf["agreeing"] * 2 >= cf["total"] else 0
                a_strict += 1 if cf["agreeing"] * 2 > cf["total"] else 0
                line += f" · 도입 {intro['change'][:8]} ({cf['agreeing']}/{cf['total']} = {pct}%)"
                if intro["others"]:
                    line += f" · 나머지 후보 {len(intro['others'])}"
            else:
                line += f" · 도입 지목 못 함 ({intro['reason']})"
        # ⑦ — **구별하지 않는다는 사실을 값으로 남긴다.** 무엇이 테스트인가의 판정은
        # F15 이고, 여기서 지는 것은 발현 좌표가 테스트 파일에 앉았다는 **사실의 기록**이다.
        tests, real = code_files(DITTO, c)
        if tests and not real:
            test_only.append(c)
            line += "   ⚠ 만진 코드가 **테스트뿐이다**"
        elif tests:
            line += f"   (테스트 {len(tests)} · 실코드 {len(real)})"
        print(line)

    bound = sum(1 for k in kinds if k == "bound")
    no_ext = sum(1 for k in kinds if k == "no_extractor")
    print(f"\n  결박 {bound}/5 · 추출기 없음 {no_ext}/5")
    print(f"  발현 좌표 {a_manifest}/5 · 도입(과반) {a_major}/5 · 도입(엄격) {a_strict}/5"
          f"   [T10 등록값: 4/5 · 4/5 · 2/5]")
    if no_ext:
        print("  **대조 불가** — T10 의 코퍼스는 TypeScript 이고 이 빌드의 추출기는 Kotlin 하나다.")
        print("  등록된 수치(발현 4/5 · 도입 4/5 · 하한 2/5)를 이 빌드로는 잴 수 없다.")
    else:
        print("  **재어졌다.** 위 수치는 F22-3 의 판정이지 F02 의 합격선이 아니다")
        print("  — T10 은 삭제된 줄을 blame 했고 이 구현은 심볼을 본다(같은 질문, 다른 도구).")
    if test_only:
        print(f"  ⚠ **이 빌드는 테스트와 실코드를 구별하지 않는다**(구별은 F15). "
              f"코드가 테스트뿐인 커밋: {', '.join(test_only)}")

    # ── B. 비판정 관측 ───────────────────────────────────────────────────────
    print("\n── B. 비판정 관측 — 기계가 도는가 (Kotlin · 사전 등록된 값이 아니다) ──")
    population = fix_population(KOTLIN)
    picks, total = sample_by_t10_rule(population)
    print(f"  모집단 `^fix` {total} 건 @ {KOTLIN_AT} · T10 과 같은 선정 규칙(등간격 5)")
    print("  **고정 SHA 에 묶여 있다** — HEAD 를 쓰면 코퍼스가 자랄 때 표본이 통째로 바뀐다\n")
    reports = []
    for c in picks:
        r = defect(KOTLIN, c)
        reports.append(r)
        k = outcome(r)
        line = f"  {c[:8]}  {k:20s}"
        if k == "bound":
            d = r["result"]
            intro = d["introduced_by"]
            line += f" 발현 {len(d['manifests_at']):3d}"
            if intro["outcome"] == "found":
                cf = intro["confidence"]
                pct = cf["agreeing"] * 100 // cf["total"] if cf["total"] else 0
                line += f" · 도입 {intro['change'][:8]} ({cf['agreeing']}/{cf['total']} = {pct}%)"
                if intro["others"]:
                    line += f" · 나머지 후보 {len(intro['others'])}"
            else:
                line += f" · 도입 지목 못 함 ({intro['reason']})"
        print(line)

    b = [r for r in reports if outcome(r) == "bound"]
    manifest_ok = len(b)
    major = sum(1 for r in b
                if r["result"]["introduced_by"]["outcome"] == "found"
                and r["result"]["introduced_by"]["confidence"]["agreeing"] * 2
                    >= r["result"]["introduced_by"]["confidence"]["total"])
    strict = sum(1 for r in b
                 if r["result"]["introduced_by"]["outcome"] == "found"
                 and r["result"]["introduced_by"]["confidence"]["agreeing"] * 2
                     > r["result"]["introduced_by"]["confidence"]["total"])
    missed = len(reports) - len(b)
    print(f"\n  발현 좌표 {manifest_ok}/5 · 도입(과반) {major}/5 · 도입(엄격) {strict}/5 · 담기지 않음 {missed}/5")
    print("  **이 값은 합격선이 아니다.** T10 의 표는 ditto 의 것이고 이것은 다른 모집단이다.")

    # ── C. 음성 대조 ─────────────────────────────────────────────────────────
    print("\n── C. 음성 대조 — 이 검사가 고장 났다면 ──────────────────────────────")

    # ── ① 추출기 없음이 "변한 것 없음" 으로 뭉개지지 않는가 ──────────────────
    #
    # F22-3 의 첫 구현이 정확히 그 상태였다. **이 대조가 2026-08-13 에 낡았다** —
    # 원래는 *"T10 표본 5 건이 전부 `no_extractor` 인가"* 를 물었고, TypeScript 추출기가
    # 서면서 그 전제(추출기가 없다)가 사라져 `✗` 를 찍기 시작했다. **결함이 아니라 대조가
    # 낡은 것이다.**
    #
    # **지우지 않는다.** 명제는 여전히 참이어야 하고, 지우면 이 도구가 자기 문제를 다시
    # 저지를 때 아무도 못 잡는다. 그래서 **추출기가 실제로 없는 언어**로 옮긴다 —
    # JavaScript 는 `NotBuilt{F02, javascript-extraction}` 이고 ditto 에 `.mjs` 가 있다.
    js_probe = None
    scanned = 0
    for c in ditto_fix_population()[:OTHERS_PROBE_LIMIT]:
        scanned += 1
        files = touched(DITTO, c)
        exts = {f.rsplit(".", 1)[-1] for f in files}
        if exts & NOT_BUILT and not (exts & EXTRACTABLE):
            js_probe = c
            break
    if js_probe is None:
        print(f"  ✗ 추출기 없는 언어만 만진 `fix` 커밋을 앞 {scanned} 건에서 찾지 못했다 "
              "— **시험되지 않았다.** 상한을 늘리거나 대조를 고쳐라")
        fail += 1
    else:
        k = outcome(defect(DITTO, js_probe))
        ok = k == "no_extractor"
        print(f"  {'✓' if ok else '✗'} 추출기 부재가 `no_semantic_change` 로 뭉개지지 않는다 "
              f"— {js_probe[:8]} → {k}  (모집단 {scanned} 건까지 훑었다)")
        if not ok:
            fail += 1

    # ② 담기지 않는 것을 실제로 세는가 — 코드가 아닌 파일만 건드린 커밋
    only_docs = subprocess.run(
        ["git", "-C", str(KOTLIN), "log", "--format=%H", "-40", "--grep", "^fix", KOTLIN_AT],
        capture_output=True, text=True, check=True).stdout.split()
    outside = None
    for c in only_docs:
        files = subprocess.run(
            ["git", "-C", str(KOTLIN), "diff-tree", "--no-commit-id", "--name-only", "-r", c],
            capture_output=True, text=True, check=True).stdout.split()
        if files and not any(f.rsplit(".", 1)[-1] in {"kt", "kts", "java", "ts", "js"} for f in files):
            outside = c
            break
    if outside:
        r = defect(KOTLIN, outside)
        ok = outcome(r) == "outside_code"
        print(f"  {'✓' if ok else '✗'} 코드 밖 변경만 있는 커밋 {outside[:8]} → {outcome(r)}")
        fail += 0 if ok else 1
    else:
        # **`–` 를 찍고 넘어가지 않는다.** 시험되지 않은 대조가 있는데 "실패 0 건" 이라고
        # 적으면 검사가 자기에 대해 거짓말한다.
        print(f"  ✗ 코드 밖 변경만 있는 `fix` 커밋을 {len(only_docs)}건 안에서 찾지 못했다 "
              "— **시험되지 않았다.** 표본을 넓히거나 대조를 고쳐라")
        fail += 1

    # ③ 결정론 — 같은 입력이 같은 답을 낸다 (출처 배정 규칙 1 의 조건)
    twice = [defect(KOTLIN, picks[0]), defect(KOTLIN, picks[0])]
    if twice[0] == twice[1]:
        print("  ✓ 같은 커밋을 두 번 → 산출 동일 (`Change`·`Actor` 가 `extracted` 인 근거)")
    else:
        print("  ✗ 같은 커밋이 다른 답을 냈다 — 출처 배정이 `extracted` 일 수 없다")
        fail += 1

    # ④ 예산이 걸린 것과 정말 없는 것이 갈리는가
    tight = defect(KOTLIN, picks[0], budget=2)
    r = tight["result"]
    if r["outcome"] == "bound" and r["introduced_by"]["outcome"] == "not_found" \
            and "history_budget" in json.dumps(r["introduced_by"]["reason"]).lower().replace("historybudget", "history_budget"):
        print("  ✓ 예산 2 로 좁히면 `HistoryBudget` 으로 갈린다 — 조용히 멈추지 않는다")
    elif r["outcome"] == "bound" and r["introduced_by"]["outcome"] == "found":
        cf = r["introduced_by"]["confidence"]
        print(f"  ✓ 예산 2 에서도 지목됨 ({cf['agreeing']}/{cf['total']}) — 도입이 바로 앞 커밋이다")
    else:
        # `?` 도 `–` 와 같은 병이다 — 판정하지 않고 넘어가면 0 건이 거짓이 된다.
        print(f"  ✗ 예산 2 가 갈리지도 지목하지도 않았다 — **시험되지 않았다**: "
              f"{json.dumps(r['introduced_by'] if r['outcome'] == 'bound' else r, ensure_ascii=False)[:120]}")
        fail += 1

    # ── ⑤ 나머지 후보를 버리지 않는가 ────────────────────────────────────────
    #
    # **이 자리가 2026-08-13 에 조용히 꺼져 있던 곳이다.** 등간격 표본 5 건이 전부
    # 만장일치라 `–` 를 찍었고, 그런데도 아래에서 *"실패 0 건"* 이라고 적었다.
    #
    # 고친 방식: **표본이 우연히 시험해 주기를 기다리지 않고 모집단을 뒤진다.**
    # 고정 SHA 모집단을 시각 오름차순으로 훑어 `others` 가 실린 첫 커밋을 찾는다.
    # 결정적이고, 못 찾으면 `–` 가 아니라 **실패**다.
    with_others = [r for r in b if r["result"]["introduced_by"].get("outcome") == "found"
                   and r["result"]["introduced_by"]["others"]]
    probed = 0
    found_at = None
    if with_others:
        found_at = "등간격 표본 안"
    else:
        for c in population[:OTHERS_PROBE_LIMIT]:
            probed += 1
            intro = defect(KOTLIN, c)["result"]
            if intro.get("outcome") == "bound" and intro["introduced_by"].get("outcome") == "found" \
                    and intro["introduced_by"]["others"]:
                found_at = c[:8]
                break
    if found_at:
        print(f"  ✓ 최빈 아닌 후보를 실은 자리를 찾았다 — {found_at}"
              + (f" (모집단 {probed}/{len(population)} 건까지 훑었다)" if probed else ""))
    else:
        print(f"  ✗ 최빈 아닌 후보를 실은 커밋을 모집단 앞 {probed}/{len(population)} 건에서 찾지 못했다 "
              "— **이 대조는 시험되지 않았다.** 상한을 늘리거나 대조를 고쳐라")
        fail += 1

    print()
    print(f"  음성 대조 실패 {fail} 건"
          + ("" if fail else " — **다섯이 전부 시험됐다**"))
    return fail


if __name__ == "__main__":
    sys.exit(main())
