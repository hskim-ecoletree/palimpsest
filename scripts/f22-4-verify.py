#!/usr/bin/env python3
"""[f22.4] 의 대조 — `pal doctor` 의 불변식 여덟이 깨진 것을 잡고 성한 것을 잡지 않는가.

합격선 정본은 `corpus/criteria.toml` `[f22.4]`.

  ① 여덟 각각에 **그 불변식 하나만** 어긋난 픽스처가 있고 여덟이 전부 잡힌다
  ② **음성 대조** — 성한 그래프에서 위반 0 건. 그리고 그 성한 그래프는 픽스처가 아니라
     **코퍼스에서 실제로 만든 2층**이다
  ③ 표본이었다는 사실이 `Residual` 로 산출에 실린다
  ④ 산출이 `Envelope` 이고 필드 일곱이 전부 실린다
  ⑤ `stale` / `stale-derived` 가 갈리고 전파 예산 초과가 `Residual` 이다
  ⑥ 스냅샷 격리 100 회에서 부분 갱신 관측 0 회

**이 스크립트의 절반은 E 부다.** ①의 8/8 은 *"무엇이든 위반이라고 말하는"* 검사로도
만점을 받고, ②의 0 건은 *"아무것도 세지 않는"* 검사로도 만점을 받는다. 그래서 검사를
하나씩 지우고 **그 픽스처가 통과해 버리는지**를 본다.

변이는 **자라는 값에 묶지 않는다** — 규모·개수·라벨 수에 걸면 그것이 자랄 때 변이가
아무것도 바꾸지 않게 되고 그 자리가 조용히 꺼진다(F22-1 에서 실제로 그렇게 됐다:
`7fe6b62`). 여기서는 조건식을 `false` 로 바꾼다. 그리고 **치환 대상이 소스에 없으면
그 자체를 오류로 낸다** — 변이가 낡으면 조용히 넘어가는 대신 소리를 낸다.
"""

from __future__ import annotations

import argparse
import json
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent

ENVELOPE_FIELDS = [
    "answer",
    "snapshot",
    "projection",
    "coverage",
    "capabilities",
    "ledger",
    "elision",
]

DOCTOR = ROOT / "crates/pal-core/src/doctor.rs"
CASCADE = ROOT / "crates/pal-core/src/cascade.rs"
ISOLATION = ROOT / "crates/pal-store/tests/isolation.rs"

# 여덟 픽스처의 시험 이름. **여덟이고 여덟이어야 한다.**
여덟 = [
    (1, "doctor::tests::불변식_1_엣지의_양_끝_노드가_존재한다"),
    (2, "doctor::tests::불변식_2_필수_속성이_있다"),
    (3, "doctor::tests::불변식_3_생산자가_출처와_정합한다"),
    (4, "doctor::tests::불변식_4_inferred_는_근거를_싣는다"),
    (5, "doctor::tests::불변식_5_잘린_후보에는_미해소_참조가_붙는다"),
    (6, "doctor::tests::불변식_6_잔여가_실재하는_좌표에_결박된다"),
    (7, "doctor::tests::불변식_7_색인이_가리키는_결박이_의도_저장소에_있다"),
    (8, "doctor::tests::불변식_8_live_노드의_입력에_stale_이_없다"),
]

# ── 음성 대조의 변이 ─────────────────────────────────────────────────────────
#
# (이름, 파일, 찾을 것, 바꿀 것, 무너져야 하는 시험)
변이 = [
    (
        "불변식 1 의 끝점 대조를 지움",
        DOCTOR,
        "            if !missing.is_empty() {",
        "            if false {",
        "doctor::tests::불변식_1_엣지의_양_끝_노드가_존재한다",
    ),
    (
        "불변식 2 의 필수 속성 검사를 지움",
        DOCTOR,
        "            if required && !n.attrs.contains_key(&a.name) {",
        "            if false {",
        "doctor::tests::불변식_2_필수_속성이_있다",
    ),
    (
        "불변식 3 의 생산자 대조를 지움",
        DOCTOR,
        "                    if &a.producer != producer {",
        "                    if false {",
        "doctor::tests::불변식_3_생산자가_출처와_정합한다",
    ),
    (
        "불변식 3 의 출처 대조를 지움",
        DOCTOR,
        "            if decl.provenance != n.provenance {",
        "            if false {",
        "doctor::tests::인스턴스_출처가_선언과_다르면_잡힌다",
    ),
    (
        "불변식 4 의 근거 검사를 지움",
        DOCTOR,
        "            if n.evidence_refs.is_empty() {",
        "            if false {",
        "doctor::tests::불변식_4_inferred_는_근거를_싣는다",
    ),
    (
        "불변식 5 의 강등 기록 검사를 지움",
        DOCTOR,
        "            if *total > kept.len() && demoted_to.is_none() {",
        "            if false {",
        "doctor::tests::불변식_5_잘린_후보에는_미해소_참조가_붙는다",
    ),
    (
        "불변식 5 의 후보 상한을 지움",
        DOCTOR,
        "            if kept.len() > CANDIDATE_LIMIT {",
        "            if false {",
        "doctor::tests::후보가_상한을_넘으면_잡힌다",
    ),
    (
        "불변식 6 의 유령 좌표 검사를 지움",
        DOCTOR,
        "            if !ghosts.is_empty() {",
        "            if false {",
        "doctor::tests::불변식_6_잔여가_실재하는_좌표에_결박된다",
    ),
    (
        "불변식 7 의 실체 대조를 지움",
        DOCTOR,
        "            if !self.view.intent_entities().contains(&e.binding) {",
        "            if false {",
        "doctor::tests::불변식_7_색인이_가리키는_결박이_의도_저장소에_있다",
    ),
    (
        "불변식 8 의 등급 대조를 지움",
        DOCTOR,
        "            if want != &n.freshness {",
        "            if false {",
        "doctor::tests::불변식_8_live_노드의_입력에_stale_이_없다",
    ),
    (
        "능력 부재를 위반 0 으로 접음",
        DOCTOR,
        "        let outcome = if report.checked == 0 && report.skipped == 0 && !absent.is_empty() {",
        "        let outcome = if false {",
        "doctor::tests::담을_수_없는_불변식은_위반_0_이_아니라_능력_부재다",
    ),
    (
        "표본 밖을 잔여로 내지 않음",
        DOCTOR,
        "        if skipped.is_empty() {\n            return None;\n        }",
        "        if !skipped.is_empty() {\n            return None;\n        }",
        "doctor::tests::표본_밖은_이상_없음이_아니라_잔여다",
    ),
    (
        "덮개의 구멍을 세지 않음",
        DOCTOR,
        "            if !declared.contains(label.as_str()) {",
        "            if false {",
        "doctor::tests::스키마가_자랐는데_뷰가_말하지_않으면_구멍으로_실린다",
    ),
    (
        "이어달리기 예산을 무한으로",
        CASCADE,
        "            if depth + 1 > depth_budget {",
        "            if false {",
        "cascade::tests::예산에_걸리면_멈추지_않고_잔여를_낸다",
    ),
    (
        "계산하지 못한 자리를 live 로 적음",
        CASCADE,
        "        if cutoff.contains(&&n.key) {\n            continue;\n        }\n        grades.entry",
        "        grades.entry",
        "cascade::tests::예산에_걸리면_멈추지_않고_잔여를_낸다",
    ),
]


def run(cmd, cwd=None, check=True):
    p = subprocess.run(cmd, cwd=cwd, capture_output=True, text=True, check=False)
    if check and p.returncode != 0:
        raise SystemExit(f"실패: {' '.join(map(str, cmd))}\n{p.stderr.strip()}")
    return p.stdout


def 시험(name: str) -> bool:
    p = subprocess.run(
        ["cargo", "test", "-p", "pal-core", "--lib", "--", name, "--exact"],
        cwd=ROOT,
        capture_output=True,
        text=True,
        check=False,
    )
    return p.returncode == 0


def mutate(path: Path, old: str, new: str) -> None:
    """치환한다. **없으면 오류다** — 변이가 낡으면 조용히 넘어가는 대신 소리를 낸다."""
    text = path.read_text(encoding="utf-8")
    if old not in text:
        raise SystemExit(
            f"변이 대상을 찾지 못했다: {path.name}\n  찾은 것: {old!r}\n"
            "  **소스가 바뀌어 변이가 낡았다.** 변이를 고치지 않으면 이 자리가 조용히 꺼진다."
        )
    path.write_text(text.replace(old, new, 1), encoding="utf-8")


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--repo", type=Path, default=Path("~/dev/projects/boxwood/portal-backend"))
    ap.add_argument("--at", default="a29cad0bf6a8")
    ap.add_argument("--early", default="f84e9d40", help="결박 대상이 아직 없는 커밋")
    ap.add_argument("--symbol", default="ResultContext", help="코퍼스에 있는 심볼")
    ap.add_argument("--bin", type=Path, default=ROOT / "target/release/pal")
    a = ap.parse_args()

    pal, repo = a.bin.expanduser().resolve(), a.repo.expanduser().resolve()
    if not pal.exists():
        print(f"바이너리가 없다: {pal}  (cargo build --release)", file=sys.stderr)
        return 1

    failures: list[str] = []

    # ── A. 깨진 픽스처 여덟 ──────────────────────────────────────────────────
    print()
    print("── A. 깨진 픽스처 여덟 — 각각 **그 불변식 하나만** 어긋난다 ─────────────")
    for n, name in 여덟:
        ok = 시험(name)
        print(f"  {'✓' if ok else '✗'} 불변식 {n}  {name.split('::')[-1]}")
        if not ok:
            failures.append(f"① 불변식 {n} 의 픽스처가 잡히지 않았다")
    print(f"\n  잡힌 픽스처 {sum(1 for n, name in 여덟 if 시험(name))}/8")

    for name in [
        "doctor::tests::성한_그래프에서는_위반이_0_이다",
        "doctor::tests::성한_그래프에서_여덟이_전부_모집단을_갖는다",
        "doctor::tests::인스턴스_출처가_선언과_다르면_잡힌다",
        "doctor::tests::후보가_상한을_넘으면_잡힌다",
    ]:
        ok = 시험(name)
        print(f"  {'✓' if ok else '✗'} {name.split('::')[-1]}")
        if not ok:
            failures.append(f"① {name} 실패")

    # ── ⑤ 낡음 전파 — 둘을 가르고 예산 초과는 잔여다 ─────────────────────────
    print()
    print("── A2. ⑤ 낡음 전파 — `stale` 과 `stale-derived` 를 가른다 ───────────────")
    for name in [
        "cascade::tests::자기가_낡은_것과_입력이_낡은_것은_다른_등급이다",
        "cascade::tests::입력이_낡으면_파생물은_stale_derived_다",
        "cascade::tests::예산에_걸리면_멈추지_않고_잔여를_낸다",
        "cascade::tests::낡은_것이_없으면_전부_live_다",
    ]:
        ok = 시험(name)
        print(f"  {'✓' if ok else '✗'} {name.split('::')[-1]}")
        if not ok:
            failures.append(f"⑤ {name} 실패")

    # ── B. 성한 그래프는 코퍼스에서 실제로 만든 2층이다 ──────────────────────
    print()
    print("── B. 음성 대조 — **코퍼스에서 실제로 만든 2층**에서 위반 0 건 ─────────")
    tmp = Path(tempfile.mkdtemp(prefix="f22-4-"))
    try:
        base = [
            "--repo", str(repo),
            "--cache-dir", str(tmp / "cache"),
            "--index", str(tmp / "index.redb"),
            "--intent", str(tmp / "intent.redb"),
        ]
        # 결박을 하나 건다 — **엣지가 없으면 불변식 1 의 모집단이 0 이다.**
        run([str(pal), "bind", a.symbol, "--note", "이 클래스는 계약이다", *base, "--at", a.at])

        env = json.loads(run([str(pal), "doctor", *base, "--at", a.at, "--full", "--json"]))
        d = env["answer"]

        빠진 = [f for f in ENVELOPE_FIELDS if f not in env]
        print(f"  {'✓' if not 빠진 else '✗'} ④ 봉투 필드 일곱  누락 {len(빠진)}")
        if 빠진:
            failures.append(f"④ 봉투 필드 누락: {빠진}")

        위반 = d["violations"]
        print(f"  {'✓' if not 위반 else '✗'} ② 전수에서 위반 {len(위반)} 건")
        if 위반:
            for v in 위반:
                print(f"      [{v['invariant']}] {v['subject']} — {v['detail']}")
            failures.append(f"② 성한 코퍼스 2층에서 위반 {len(위반)} 건")

        구멍 = d["coverage_gaps"]
        print(f"  {'✓' if not 구멍 else '✗'} 이 검사의 구멍 {len(구멍)} 건")
        if 구멍:
            failures.append(f"덮개 구멍: {구멍}")

        미결박 = d["unanchored_cutoff"]
        print(f"  {'✓' if not 미결박 else '✗'} 결박하지 못한 예산 초과 {len(미결박)} 건")
        if 미결박:
            failures.append(f"결박하지 못한 예산 초과: {미결박}")

        전수_잔여 = d["residuals"]
        print(f"  {'✓' if not 전수_잔여 else '✗'} 전수에는 표본 잔여가 없다 ({len(전수_잔여)})")
        if 전수_잔여:
            failures.append("전수인데 잔여가 남았다")

        # ③ 표본이었다는 사실
        표본 = json.loads(
            run([str(pal), "doctor", *base, "--at", a.at, "--sample", "4", "--json"])
        )["answer"]
        표본_잔여 = [r for r in 표본["residuals"] if r["reason"] == "outside-sample"]
        결박됨 = all(r["bound_to"] for r in 표본_잔여)
        ok3 = bool(표본_잔여) and 결박됨 and not 표본["violations"]
        print(
            f"  {'✓' if ok3 else '✗'} ③ 표본 밖이 잔여로 실린다 — {len(표본_잔여)} 건 · "
            f"전부 좌표에 결박됨 {결박됨}"
        )
        if not ok3:
            failures.append("③ 표본이었다는 사실이 잔여로 실리지 않았다")

        # **성한 것을 잡지 않는 것만으로는 부족하다** — 실물에서 실제로 깨면 잡는가.
        # 결박은 `a29cad0` 의 심볼에 걸렸고, 그 심볼이 아직 없던 커밋에서 2층을 세우면
        # `BOUND_TO` 가 매달린다. **코퍼스 위에서 위반이 실제로 나오는 유일한 자리다.**
        깨진 = json.loads(
            run([str(pal), "doctor", *base, "--at", a.early, "--full", "--json"])
        )["answer"]
        매달림 = [v for v in 깨진["violations"] if v["invariant"] == "edge-ends-exist"]
        print(
            f"  {'✓' if 매달림 else '✗'} 실물을 깨면 잡는다 — 결박 대상이 없던 커밋 "
            f"{a.early} 에서 불변식 1 위반 {len(매달림)} 건"
        )
        if not 매달림:
            failures.append("실물에서 매달린 엣지를 잡지 못했다 — 위반 0 이 장식이다")

        # ── C. 실물에서 시험된 것 / 픽스처에서만 시험된 것 ───────────────────
        print()
        print("── C. 실물에서 시험된 것 / 픽스처에서만 시험된 것 ──────────────────────")
        실물, 픽스처만 = [], []
        for r in d["invariants"]:
            outcome = r["outcome"]
            if isinstance(outcome, dict) and "checked" in outcome:
                o = outcome["checked"]
                실물.append((r["number"], o["checked"]))
                print(f"  {r['number']}  실물     모집단 {o['checked']} · 위반 {o['violations']}")
            else:
                by = " · ".join(f"{x['label']}({x['built_by']})" for x in r["absent"])
                픽스처만.append(r["number"])
                print(f"  {r['number']}  픽스처만  담을 자리가 없다 — {by}")
        print(f"\n  실물 {len(실물)} · 픽스처만 {len(픽스처만)}")
        for n in (1, 2, 3):
            if n not in [x[0] for x in 실물]:
                failures.append(f"불변식 {n} 이 실물에서 검사되지 않았다")
    finally:
        shutil.rmtree(tmp, ignore_errors=True)

    # ── D. 스냅샷 격리 ──────────────────────────────────────────────────────
    print()
    print("── D. 스냅샷 격리 100 회 — 부분 갱신이 보이는 창이 있는가 ───────────────")
    p = subprocess.run(
        ["cargo", "test", "-p", "pal-store", "--test", "isolation", "--", "--nocapture"],
        cwd=ROOT,
        capture_output=True,
        text=True,
        check=False,
    )
    줄 = next((l.strip() for l in p.stdout.splitlines() if l.startswith("격리")), "")
    print(f"  {'✓' if p.returncode == 0 else '✗'} {줄 or '측정값을 찾지 못했다'}")
    if p.returncode != 0:
        failures.append("⑥ 격리 시험 실패 — **저장 계약의 실패다**")

    # ── E. 음성 대조 — 검사를 지우면 픽스처가 통과해 버리는가 ────────────────
    print()
    print("── E. 음성 대조 — 검사를 지우면 그 픽스처가 통과해 버리는가 ────────────")
    backups = {p: p.read_text(encoding="utf-8") for p in {DOCTOR, CASCADE, ISOLATION}}
    try:
        for name, path, old, new, test in 변이:
            mutate(path, old, new)
            여전히 = 시험(test)
            path.write_text(backups[path], encoding="utf-8")
            if 여전히:
                print(f"  ✗ {name:<38} {test.split('::')[-1]} 가 여전히 통과한다")
                failures.append(f"음성 대조: {name} — 검사가 아니라 장식이다")
            else:
                print(f"  ✓ {name:<38} 무너졌다")

        # 격리 시험의 음성 대조 — **쓰기가 안 돌면 0/100 이 아무것도 말하지 않는다.**
        mutate(
            ISOLATION,
            "            while !done.load(Ordering::Relaxed) {",
            "            while false {",
        )
        p = subprocess.run(
            ["cargo", "test", "-p", "pal-store", "--test", "isolation"],
            cwd=ROOT,
            capture_output=True,
            text=True,
            check=False,
        )
        ISOLATION.write_text(backups[ISOLATION], encoding="utf-8")
        if p.returncode == 0:
            print("  ✗ 격리 시험에서 쓰기를 끔                    여전히 통과한다")
            failures.append("음성 대조: 쓰기 없이도 격리 시험이 통과한다 — 0/100 이 장식이다")
        else:
            print("  ✓ 격리 시험에서 쓰기를 끔                    무너졌다")
    finally:
        for path, text in backups.items():
            path.write_text(text, encoding="utf-8")

    print()
    if failures:
        print("── 어긋난 것 ───────────────────────────────────────────────────────────")
        for f in failures:
            print(f"  ✗ {f}")
        print(f"\n  **{len(failures)} 건**. 건수가 아니라 목록을 게이트에 적는다.")
        return 1
    print("  음성 대조 실패 0 건 — **깨진 것을 잡고 성한 것을 잡지 않는다**")
    return 0


if __name__ == "__main__":
    sys.exit(main())
