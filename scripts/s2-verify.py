#!/usr/bin/env python3
"""S2 대조 — `pal touch` 의 산출을 등록된 합격선 여섯에 댄다.

합격선 정본은 `corpus/criteria.toml` `[s2]`.

  ① 봉투 필드 누락 0 + **`elision` 이 명시적 `none()`**
  ② **빈 자리가 `[]` 가 아니라 `not_built{기능번호}`**
  ③ 정규화 양방향 (포매팅에 불변 · 의미에 가변)
  ④ 정체성 (같은 심볼은 같은 id · 이동하면 id 는 바뀌고 digest 는 그대로)
  ⑤ 2층 재구축 등가성
  ⑥ 코퍼스에서 실제로 찾아진다

③④ 는 단위 테스트가 불변식으로 지고(`cargo test -p pal-extract -p pal-core`),
여기서는 **실제 코퍼스 위에서** 다시 확인한다 — 합성 입력만으로는 붙었다고 할 수 없다.

# ⚠ 재판정 (2026-08-14 · #55) — **등록의 수정이 아니라 이행의 기록이다**

이 스크립트는 `46b3153`(F01) 이후 한 줄도 안 움직였고, **F05 종료 시점에 이미 낡아
있었다.** 셋이 어긋났는데 **셋 다 F05 가 의도적으로 만든 것**이었다. 근거 전문은
`[s2].restated_after_f05`. 요지:

  · `elision` — 요구는 **「명시적 `none()`」이지 「`{dropped, reasons}`」가 아니다.**
    옛 검사가 요구를 **리터럴 하나로 굳혀** 놓아서 모양이 바뀌자 요구까지 못 읽었다.
    다시 쓴 형태: **자리가 있고 · 선언된 통이 1개 이상이고 · 전부 비어 있다.**
    `{}` 는 실패다 — 통이 0 이면 *"안 잘랐다"* 가 아니라 *"아무 말도 안 했다"* 다
  · `facts` — **이행.** F05-2 가 파일 안의 호출 관계를 값으로 만들었다.
    `bindings`(S3) · `matches_worktree`(F01) 와 글자 그대로 같은 일이다
  · `rebuild` — **이행.** 옛 주석이 뒤집는 조건을 스스로 적었다(*"**조용히** 채워지면
    관측 없이 단언된다"*). 조용히가 아니다 — ADR-0010 의 무대 테이블이 관측이다

**그리고 반대 방향을 넣는다** — 검사를 리터럴에서 요구로 옮기면 **아무것도 안 세는
검사**가 되기 쉽다. `--self-test` 가 방향마다 봉투를 망가뜨려 **잡히는지** 센다.
"""

from __future__ import annotations

import argparse
import copy
import json
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

ENVELOPE_FIELDS = ["answer", "snapshot", "projection", "coverage", "capabilities", "ledger", "elision"]

# **채워진 자리는 위반이 아니라 이행이다.** 넷이 이 길을 지나갔다:
#   bindings          S3 가 채웠다 (결박이 실제로 뜬다)
#   matches_worktree  F01 이 채웠다 (워킹트리 요약)
#   facts             F05-2 가 채웠다 (파일 **안**의 호출 관계)
#   rebuild           F05-2 가 채웠다 (ADR-0010 의 무대 테이블 = 관측 경로)
# S2 가 요구한 것은 *"빈 자리를 빈 배열로 내지 말 것"* 이고, 값이 생기면 값이 나가는
# 것이 맞다. **나머지 셋은 여전히 `NotBuilt` 여야 한다.**
NOT_BUILT_SLOTS = ["unresolved", "effects", "judgments"]
BUILT_SLOTS = ["bindings", "facts"]

# **하한** — 자리가 0 개면 ②가 공짜로 통과한다(`2e2eb3f`: 시험되지 않은 대조는 실패다).
MIN_NOT_BUILT = 3
MIN_BUILT = 2


def is_empty_bucket(v) -> bool:
    """비었다고 **말한** 통인가. `None` 은 비어 있는 것이 아니라 말을 안 한 것이다."""
    return v in ([], {}, 0)


def audit(env: dict) -> tuple[list[str], list[str]]:
    """봉투 하나를 합격선 ①②⑥ 에 댄다. **순수하다** — 그래야 반대 방향을 걸 수 있다.

    반환은 `(어긋난 것, 발견)`. **발견은 합격선이 아니다** — 판정에 안 쓰고 찍기만 한다.
    """
    failures: list[str] = []
    notes: list[str] = []

    # ── ① 봉투 필드
    missing = [f for f in ENVELOPE_FIELDS if f not in env]
    if missing:
        failures.append(f"① 봉투 필드 누락: {missing}")

    # ── ① `elision` 이 **명시적** `none()` 인가
    #
    # **모양이 아니라 요구를 잰다.** F05-1 이 `{dropped, reasons}` 를
    # `{truncated, limits_hit}` 로 바꿨고 그것은 F05 문서 §5.2 가 요구한 것이다.
    # 요구(stack §5.4 — 조용한 절단 금지)는 새 모양에서도 그대로 참이다.
    el = env.get("elision")
    if not isinstance(el, dict):
        failures.append(f"① elision 이 자리에 없다 — 조용한 절단 금지의 정면 위반: {el!r}")
    elif not el:
        # **`{}` 는 실패다.** 선언된 통이 0 이면 *"안 잘랐다"* 가 아니라
        # *"아무 말도 안 했다"* 이고, 그것이 stack §5.4 가 금한 바로 그것이다.
        failures.append("① elision 이 `{}` 다 — 통이 하나도 없으면 명시가 아니다")
    else:
        말한_것 = {k: v for k, v in el.items() if not is_empty_bucket(v)}
        if 말한_것:
            failures.append(f"① elision 이 `none()` 이 아니다 — 자른 것이 실려 있다: {말한_것}")

    # ── ⑥ 실물
    answer = env.get("answer") or {}
    if answer.get("outcome") != "found":
        failures.append(f"⑥ 심볼을 찾지 못했다: {answer.get('outcome')!r}")
    if (env.get("projection") or {}).get("symbols_indexed") == 0:
        failures.append("⑥ 2층이 비어 있다")

    # ── ② 빈 답의 정직성 — **이 게이트의 정체성이다**
    if len(NOT_BUILT_SLOTS) < MIN_NOT_BUILT or len(BUILT_SLOTS) < MIN_BUILT:
        failures.append(
            f"② 하한 미달 — 안 만든 자리 {len(NOT_BUILT_SLOTS)}/{MIN_NOT_BUILT} · "
            f"채워진 자리 {len(BUILT_SLOTS)}/{MIN_BUILT}. 자리가 없으면 ②가 공짜로 통과한다"
        )
    for slot in NOT_BUILT_SLOTS:
        v = answer.get(slot)
        if v in ([], {}, None):
            failures.append(f"② {slot}={v!r} — 빈 자리를 빈 값으로 냈다")
        elif not (isinstance(v, dict) and "not_built" in v):
            failures.append(f"② {slot} 이 not_built 가 아니다: {v!r}")
        elif not v["not_built"].get("capability", {}).get("feature"):
            failures.append(f"② {slot} 에 기능 번호가 없다")
    for slot in BUILT_SLOTS:
        v = answer.get(slot)
        if not (isinstance(v, dict) and "present" in v):
            failures.append(f"② {slot} 이 Present 가 아니다 (채워진 자리가 되돌아갔다): {v!r}")

    proj = env.get("projection") or {}
    # **F01 이 채운 자리** — 이제 값이어야 한다. 그 값이 참인지는 `f01-verify` 가 센다.
    mw = proj.get("matches_worktree")
    if not (isinstance(mw, dict) and "present" in mw):
        failures.append(f"② matches_worktree 가 값이 아니다 (F01 이 채운 자리): {mw!r}")
    # **F05-2 가 채운 자리** — ADR-0010 의 무대 테이블이 관측 경로다.
    rb = proj.get("rebuild")
    if not (isinstance(rb, dict) and "present" in rb):
        failures.append(f"② rebuild 가 값이 아니다 (F05 가 채운 자리): {rb!r}")

    # ── ⚠ 발견 — **[s2] 의 합격선이 아니다. 통과로도 세지 않는다.**
    #
    # `facts` 의 모집단. Kotlin 추출기는 스코프 체인을 안 만들어(`Capable::NotBuilt`)
    # 파일 내 엣지가 0 이다. **ADR-0002 가 가른 자리** — 「없음」이 아니라 「안 만듦」인데
    # `pal-cli/src/touch.rs` 가 `facts` 를 **무조건** `Capable::Present` 로 싣는다.
    # 소유자가 다르다(F07 · F02-3). **조용히 넘기지 않는 것이 여기서 할 수 있는 전부다.**
    f = (answer.get("facts") or {}).get("present")
    if isinstance(f, dict) and f.get("callers") == 0 and f.get("callees") == 0:
        notes.append(
            "facts 의 모집단이 0 이다 — Kotlin 은 스코프 체인을 안 만든다. "
            "「없음」이 아니라 「안 만듦」이고 그것이 ADR-0002 다 (소유자 F07·F02-3)"
        )

    return failures, notes


# ─────────────────────────────────────────────────────────────────────────────
# 음성 대조 — **방향마다 망가뜨려서 잡는다**
#
# 검사를 리터럴에서 요구로 옮기면 **아무것도 안 세는 검사**가 되기 쉽다. 여덟을 각각
# 깨뜨려 `audit` 이 잡는지 세고, **성한 봉투는 안 잡는지도** 함께 센다.
# ─────────────────────────────────────────────────────────────────────────────

def 망가뜨리기() -> list[tuple[str, callable]]:
    def 지우기(k):
        def f(e):
            e.pop(k, None)
        return f

    return [
        ("elision 자리를 통째로 지운다", 지우기("elision")),
        ("elision 을 `{}` 로 — 통이 하나도 없다", lambda e: e.__setitem__("elision", {})),
        ("elision 에 자른 것을 싣는다", lambda e: e["elision"].__setitem__(
            "truncated", [{"reason": "budget", "count": 1}])),
        ("봉투 필드 하나(coverage)를 지운다", 지우기("coverage")),
        ("안 만든 자리 하나를 `[]` 로", lambda e: e["answer"].__setitem__("unresolved", [])),
        ("안 만든 자리에서 기능 번호를 지운다", lambda e: e["answer"]["effects"]["not_built"]
            .__setitem__("capability", {})),
        ("채워진 자리(bindings)를 not_built 로 되돌린다", lambda e: e["answer"].__setitem__(
            "bindings", {"not_built": {"capability": {"feature": "F09", "name": "binding"}}})),
        ("rebuild 를 not_built 로 되돌린다", lambda e: e["projection"].__setitem__(
            "rebuild", {"not_built": {"capability": {"feature": "F05", "name": "rebuild"}}})),
    ]


def self_test(env: dict) -> list[str]:
    """**깨진 것을 잡고 성한 것을 잡지 않는다.**"""
    problems: list[str] = []

    성한것, _ = audit(env)
    print(f"  {'✓' if not 성한것 else '✗'} 성한 봉투는 안 잡는다{'':<22}"
          f"{'그대로' if not 성한것 else 성한것}")
    if 성한것:
        problems.append(f"성한 봉투에서 어긋남이 났다: {성한것}")

    for 이름, 깨기 in 망가뜨리기():
        상한것 = copy.deepcopy(env)
        try:
            깨기(상한것)
        except (KeyError, TypeError) as e:
            problems.append(f"「{이름}」 을 만들지 못했다 (봉투의 모양이 바뀌었다): {e}")
            print(f"  ✗ {이름:<44} 만들지 못했다")
            continue
        잡힌것, _ = audit(상한것)
        잡혔나 = bool(잡힌것)
        print(f"  {'✓' if 잡혔나 else '✗'} {이름:<44} {'잡힌다' if 잡혔나 else '안 잡힌다'}")
        if not 잡혔나:
            problems.append(f"「{이름}」 를 안 잡는다 — 이 검사는 문장일 뿐이다")

    return problems


def run(cmd, cwd=None):
    p = subprocess.run(cmd, cwd=cwd, capture_output=True, text=True, check=False)
    if p.returncode != 0:
        raise SystemExit(f"실패: {' '.join(map(str, cmd))}\n{p.stderr.strip()}")
    return p.stdout


def touch(pal, repo, at, cache, index, name):
    return json.loads(run([str(pal), "touch", name, "--repo", str(repo), "--at", at,
                           "--cache-dir", str(cache), "--index", str(index), "--json"]))


def main() -> int:
    root = Path(__file__).resolve().parent.parent
    ap = argparse.ArgumentParser()
    ap.add_argument("--repo", type=Path, required=True)
    ap.add_argument("--at", required=True)
    ap.add_argument("--symbol", default="ResultContext", help="코퍼스에 있는 심볼 이름")
    ap.add_argument("--bin", type=Path, default=root / "target/release/pal")
    ap.add_argument("--skip-self-test", action="store_true",
                    help="음성 대조를 건너뛴다 — **판정에 쓰지 마라**")
    a = ap.parse_args()

    pal, repo = a.bin.resolve(), a.repo.expanduser().resolve()
    if not pal.exists():
        print(f"바이너리가 없다: {pal}", file=sys.stderr)
        return 1

    failures: list[str] = []
    notes: list[str] = []
    tmp = Path(tempfile.mkdtemp(prefix="s2-verify-"))
    try:
        cache, index = tmp / "cache", tmp / "index.redb"
        env = touch(pal, repo, a.at, cache, index, a.symbol)

        f, n = audit(env)
        failures.extend(f)
        notes.extend(n)

        answer = env["answer"]
        el = env.get("elision") or {}
        print(f"① 봉투  필드 {len(ENVELOPE_FIELDS) - len([x for x in ENVELOPE_FIELDS if x not in env])}"
              f"/{len(ENVELOPE_FIELDS)} · elision 통 {len(el)} 개 · 전부 비었나 "
              f"{'예' if all(is_empty_bucket(v) for v in el.values()) else '아니오'}")
        print(f"⑥ 실물  outcome={answer.get('outcome')} · 2층 심볼 {env['projection']['symbols_indexed']}")
        shown = {s: answer[s]["not_built"]["capability"]["feature"] for s in NOT_BUILT_SLOTS
                 if isinstance(answer.get(s), dict) and "not_built" in answer[s]}
        채워진 = [s for s in BUILT_SLOTS
                if isinstance(answer.get(s), dict) and "present" in answer[s]]
        print(f"② 빈답  {len(shown)}/{len(NOT_BUILT_SLOTS)} 자리가 not_built · {shown}")
        print(f"② 이행  {len(채워진)}/{len(BUILT_SLOTS)} 자리가 채워졌다 · {채워진}"
              f"  · matches_worktree · rebuild")

        # ⑤ 2층 재구축 등가성 — 통째로 지우고 다시
        before = json.dumps(env, sort_keys=True)
        index.unlink(missing_ok=True)
        again = touch(pal, repo, a.at, cache, index, a.symbol)
        same = json.dumps(again, sort_keys=True) == before
        print(f"⑤ 2층  지우고 재구축 → {'동일' if same else '다름'}")
        if not same:
            failures.append("⑤ 2층을 지우고 재구축했더니 결과가 달랐다 — 2층에만 있는 상태가 있다")

        # ③④ 실제 코퍼스 위에서
        found = again["answer"]["symbol"]
        sid, body = found["id"], found["body"]
        third = touch(pal, repo, a.at, cache, index, a.symbol)["answer"]["symbol"]
        stable = third["id"] == sid and third["body"] == body
        print(f"④ 정체성  같은 심볼 재조회 동일 {'예' if stable else '아니오'} · id {sid[:12]} · body {body[:12]}")
        if not stable:
            failures.append("④ 같은 심볼을 다시 조회했더니 id/digest 가 달라졌다")
        if sid == body:
            failures.append("④ symbol_id 와 body_digest 가 같다 — 두 축이 분리되지 않았다")

        # ③ 은 단위 테스트가 불변식으로 진다. 여기서는 그것이 실제로 도는지 확인한다.
        run(["cargo", "test", "-p", "pal-extract", "-p", "pal-core", "--quiet"], cwd=root)
        print("③ 정규화  단위 불변식 통과 (포매팅 불변 · 의미 가변 · 변수명 보존)")

        # ★ 음성 대조 — **이 검사가 고장 났다면 어떻게 드러나는가**
        if not a.skip_self_test:
            print("\n★ 음성 대조 — 방향마다 망가뜨려서 잡는다")
            problems = self_test(env)
            print(f"  음성 대조 실패 {len(problems)} 건")
            failures.extend(f"★ {p}" for p in problems)

    finally:
        shutil.rmtree(tmp, ignore_errors=True)

    if notes:
        print("\n발견 (합격선이 아니다 · 통과로도 안 센다):")
        for n in notes:
            print(f"  · {n}")

    print()
    if failures:
        print("어긋난 것:")
        for f in failures:
            print(f"  · {f}")
        print("\n반증이다")
        return 1
    print("여섯 다 통과")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
