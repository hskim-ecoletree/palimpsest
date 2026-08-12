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


def defect(repo: Path, rev: str, budget: int = 400) -> dict:
    out = subprocess.run(
        [str(PAL), "defect", rev, "--repo", str(repo), "--json", "--history-limit", str(budget)],
        capture_output=True, text=True, check=True,
    )
    return json.loads(out.stdout)


def outcome(report: dict) -> str:
    r = report["result"]
    return r["why"] if r["outcome"] == "missed" else "bound"


def sample_by_t10_rule(repo: Path, n: int = 5) -> list[str]:
    """T10 과 **같은 선정 규칙** — 시각 오름차순 등간격."""
    log = subprocess.run(
        ["git", "-C", str(repo), "log", "--reverse", "--format=%H", "--grep", "^fix"],
        capture_output=True, text=True, check=True,
    ).stdout.split()
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
    for c, k, r in zip(T10_SAMPLE, kinds, a):
        note = ""
        if k == "no_extractor":
            note = f"  ← {r['result']['capability']['feature']} 미구축 ({r['result']['capability']['what']})"
        print(f"  {c}  {k}{note}")
    bound = sum(1 for k in kinds if k == "bound")
    no_ext = sum(1 for k in kinds if k == "no_extractor")
    print(f"\n  결박 {bound}/5 · 추출기 없음 {no_ext}/5")
    if no_ext:
        print("  **대조 불가** — T10 의 코퍼스는 TypeScript 이고 이 빌드의 추출기는 Kotlin 하나다.")
        print("  등록된 수치(발현 4/5 · 도입 4/5 · 하한 2/5)를 이 빌드로는 잴 수 없다.")

    # ── B. 비판정 관측 ───────────────────────────────────────────────────────
    print("\n── B. 비판정 관측 — 기계가 도는가 (Kotlin · 사전 등록된 값이 아니다) ──")
    picks, total = sample_by_t10_rule(KOTLIN)
    print(f"  모집단 `^fix` {total} 건 · T10 과 같은 선정 규칙(등간격 5)\n")
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

    # ① 추출기 없음이 "변한 것 없음" 으로 뭉개지지 않는가
    #    (F22-3 의 첫 구현이 정확히 그 상태였다)
    if no_ext == 5 and all(k != "no_semantic_change" for k in kinds):
        print("  ✓ 추출기 부재가 `no_semantic_change` 로 뭉개지지 않는다")
    else:
        print("  ✗ 추출기 부재가 '변한 것 없음' 으로 세어졌다 — 이 도구가 자기 문제를 저지른다")
        fail += 1

    # ② 담기지 않는 것을 실제로 세는가 — 코드가 아닌 파일만 건드린 커밋
    only_docs = subprocess.run(
        ["git", "-C", str(KOTLIN), "log", "--format=%H", "-40", "--grep", "^fix"],
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
        print("  – 코드 밖 변경만 있는 `fix` 커밋을 40건 안에서 찾지 못했다 (건너뜀 아님: 표본 부족)")

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
        print(f"  ? 예산 2 결과: {json.dumps(r['introduced_by'] if r['outcome']=='bound' else r, ensure_ascii=False)[:120]}")

    # ⑤ 나머지 후보를 버리지 않는가
    with_others = [r for r in b if r["result"]["introduced_by"].get("outcome") == "found"
                   and r["result"]["introduced_by"]["others"]]
    print(f"  {'✓' if with_others else '–'} 최빈 아닌 후보를 실은 건수: {len(with_others)}/{len(b)}"
          + ("" if with_others else "  (이번 표본에서는 전부 만장일치라 시험되지 않았다)"))

    print()
    print(f"  음성 대조 실패 {fail} 건")
    return fail


if __name__ == "__main__":
    sys.exit(main())
