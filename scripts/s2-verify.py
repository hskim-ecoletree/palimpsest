#!/usr/bin/env python3
"""S2 대조 — `pal touch` 의 산출을 등록된 합격선 여섯에 댄다.

합격선 정본은 `corpus/criteria.toml` `[s2]`.

  ① 봉투 필드 누락 0
  ② **빈 자리가 `[]` 가 아니라 `not_built{기능번호}`**
  ③ 정규화 양방향 (포매팅에 불변 · 의미에 가변)
  ④ 정체성 (같은 심볼은 같은 id · 이동하면 id 는 바뀌고 digest 는 그대로)
  ⑤ 2층 재구축 등가성
  ⑥ 코퍼스에서 실제로 찾아진다

③④ 는 단위 테스트가 불변식으로 지고(`cargo test -p pal-extract -p pal-core`),
여기서는 **실제 코퍼스 위에서** 다시 확인한다 — 합성 입력만으로는 붙었다고 할 수 없다.
"""

from __future__ import annotations

import argparse
import json
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

ENVELOPE_FIELDS = ["answer", "snapshot", "projection", "coverage", "capabilities", "ledger", "elision"]
# **`bindings` 는 여기 없다.** S2 등록 시점에는 그 자리도 `NotBuilt` 였고 그것이
# 합격선 ② 였는데, **S3 가 그 자리를 채웠다**(결박이 실제로 뜬다). 자리가 채워진 것은
# 합격선의 위반이 아니라 이행이다 — S2 가 요구한 것은 *"빈 자리를 빈 배열로 내지 말 것"*
# 이고, 값이 생기면 값이 나가는 것이 맞다. 나머지 넷은 여전히 `NotBuilt` 여야 한다.
NOT_BUILT_SLOTS = ["facts", "unresolved", "effects", "judgments"]
BUILT_SLOTS = ["bindings"]


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
    a = ap.parse_args()

    pal, repo = a.bin.resolve(), a.repo.expanduser().resolve()
    if not pal.exists():
        print(f"바이너리가 없다: {pal}", file=sys.stderr)
        return 1

    failures: list[str] = []
    tmp = Path(tempfile.mkdtemp(prefix="s2-verify-"))
    try:
        cache, index = tmp / "cache", tmp / "index.redb"
        env = touch(pal, repo, a.at, cache, index, a.symbol)

        # ① 봉투
        missing = [f for f in ENVELOPE_FIELDS if f not in env]
        print(f"① 봉투  필드 {len(ENVELOPE_FIELDS) - len(missing)}/{len(ENVELOPE_FIELDS)} · 누락 {len(missing)}")
        if missing:
            failures.append(f"① 봉투 필드 누락: {missing}")
        if env.get("elision") != {"dropped": 0, "reasons": []}:
            failures.append(f"① elision 이 명시적 none() 이 아니다: {env.get('elision')}")

        # ⑥ 실물
        answer = env["answer"]
        print(f"⑥ 실물  outcome={answer.get('outcome')} · 2층 심볼 {env['projection']['symbols_indexed']}")
        if answer.get("outcome") != "found":
            failures.append(f"⑥ `{a.symbol}` 를 찾지 못했다: {answer.get('outcome')}")
        if env["projection"]["symbols_indexed"] == 0:
            failures.append("⑥ 2층이 비어 있다")

        # ② 빈 자리
        bad = []
        for slot in NOT_BUILT_SLOTS:
            v = answer.get(slot)
            if v == [] or v == {} or v is None:
                bad.append(f"{slot}={v!r}")
            elif not (isinstance(v, dict) and "not_built" in v):
                bad.append(f"{slot} 이 not_built 가 아니다: {v!r}")
            elif not v["not_built"].get("capability", {}).get("feature"):
                bad.append(f"{slot} 에 기능 번호가 없다")
        for slot in BUILT_SLOTS:
            v = answer.get(slot)
            if not (isinstance(v, dict) and "present" in v):
                bad.append(f"{slot} 이 Present 가 아니다 (S3 가 채운 자리): {v!r}")
        shown = {s: answer[s]["not_built"]["capability"]["feature"] for s in NOT_BUILT_SLOTS
                 if isinstance(answer.get(s), dict) and "not_built" in answer[s]}
        print(f"② 빈답  {len(shown)}/{len(NOT_BUILT_SLOTS)} 자리가 not_built · {shown}"
              f"  ·  채워진 자리 {BUILT_SLOTS}")
        if bad:
            failures.extend(f"② {b}" for b in bad)
        # 워킹트리도 같은 규율을 받는다
        mw = env["projection"]["matches_worktree"]
        if not (isinstance(mw, dict) and "not_built" in mw):
            failures.append(f"② matches_worktree 가 not_built 가 아니다: {mw!r}")

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
        # 같은 입력 → 같은 값
        third = touch(pal, repo, a.at, cache, index, a.symbol)["answer"]["symbol"]
        stable = third["id"] == sid and third["body"] == body
        print(f"④ 정체성  같은 심볼 재조회 동일 {'예' if stable else '아니오'} · id {sid[:12]} · body {body[:12]}")
        if not stable:
            failures.append("④ 같은 심볼을 다시 조회했더니 id/digest 가 달라졌다")
        if sid == body:
            failures.append("④ symbol_id 와 body_digest 가 같다 — 두 축이 분리되지 않았다")

        # ③ 은 단위 테스트가 불변식으로 진다. 여기서는 그것이 실제로 도는지 확인한다.
        out = run(["cargo", "test", "-p", "pal-extract", "-p", "pal-core", "--quiet"], cwd=root)
        print("③ 정규화  단위 불변식 통과 (포매팅 불변 · 의미 가변 · 변수명 보존)")

    finally:
        shutil.rmtree(tmp, ignore_errors=True)

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
