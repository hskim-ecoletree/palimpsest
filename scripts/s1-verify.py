#!/usr/bin/env python3
"""S1 대조 — `pal ledger` 의 산출을 등록된 합격선 다섯에 댄다.

합격선 정본은 `corpus/criteria.toml` `[s1]` 이다. 여기서 재는 것은 넷이고,
다섯째(gix 격리)는 구조의 합격선이라 `cargo xtask check` 가 센다.

  ① 경로 집합이 `git ls-tree -r` 와 같은가 — 양방향, 불일치 0
  ② 상태 분할이 전수인가 — 모든 파일이 정확히 한 칸, 합이 파일 수
  ③ 2회차 캐시 미스 0 · **대장 산출이 1회차와 완전히 같은가**
  ④ **음성 대조** — blob 하나를 바꾸면 미스가 정확히 1건

**④ 가 이 스크립트의 핵심이다.** ③ 만 보면 "언제나 적중"이라 거짓 보고하는 캐시가
만점을 받는다. S0 에서 대조 도구가 아무것도 안 하는 도구일 수 있다는 것이 문제였고,
캐시에서 같은 문제가 더 조용하게 일어난다.

사용:
    ./scripts/s1-verify.py --repo ~/dev/projects/boxwood/portal-backend --at a29cad0bf6a8

종료 코드:
    0  넷 다 통과
    1  하나라도 어긋났다 · 또는 대조가 성립하지 않았다
"""

from __future__ import annotations

import argparse
import json
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

BUCKETS = [
    "parsed", "partial", "unsupported", "unrecognized", "excluded", "binary", "generated",
]


def run(cmd: list[str], cwd: Path | None = None) -> str:
    p = subprocess.run(cmd, cwd=cwd, capture_output=True, text=True, check=False)
    if p.returncode != 0:
        raise SystemExit(f"실패: {' '.join(cmd)}\n{p.stderr.strip()}")
    return p.stdout


def ledger(pal: Path, repo: Path, at: str | None, cache: Path) -> dict:
    cmd = [str(pal), "ledger", str(repo), "--cache-dir", str(cache), "--json"]
    if at:
        cmd += ["--at", at]
    return json.loads(run(cmd))


def git_blob_paths(repo: Path, rev: str) -> set[str]:
    """`git ls-tree -r` 의 **blob 만**. 대장이 세는 단위와 같아야 한다."""
    out = run(["git", "ls-tree", "-r", rev], cwd=repo)
    paths = set()
    for line in out.splitlines():
        meta, _, path = line.partition("\t")
        # "<mode> <type> <hash>"
        if len(meta.split()) >= 2 and meta.split()[1] == "blob":
            paths.add(path)
    return paths


def bucket_of(state: dict | str) -> str:
    """`FileState` 의 외부 태그 표현에서 칸 이름을 꺼낸다."""
    if isinstance(state, str):  # unit variant — `unrecognized`
        return state
    if len(state) != 1:
        raise SystemExit(f"상태가 하나가 아니다: {state}")
    return next(iter(state))


def main() -> int:
    repo_root = Path(__file__).resolve().parent.parent
    ap = argparse.ArgumentParser()
    ap.add_argument("--repo", type=Path, required=True, help="대상 git 저장소")
    ap.add_argument("--at", help="커밋. 기본값은 HEAD")
    ap.add_argument("--bin", type=Path, default=repo_root / "target/release/pal")
    a = ap.parse_args()

    pal = a.bin.resolve()
    repo = a.repo.expanduser().resolve()
    if not pal.exists():
        print(f"바이너리가 없다: {pal} — `cargo build --release` 를 먼저 하라", file=sys.stderr)
        return 1

    failures: list[str] = []
    tmp = Path(tempfile.mkdtemp(prefix="s1-verify-"))
    try:
        # ── 1회차 ──────────────────────────────────────────────────────────
        cache = tmp / "cache"
        first = ledger(pal, repo, a.at, cache)
        entries = first["ledger"]["entries"]
        n = len(entries)
        print(f"1회차   파일 {n} · 적중 {first['cache']['hits']} · 빗나감 {first['cache']['misses']}")

        # ── ① 경로 집합 ────────────────────────────────────────────────────
        rev = a.at or "HEAD"
        want = git_blob_paths(repo, rev)
        got = {e["path"] for e in entries}
        only_git = sorted(want - got)
        only_ledger = sorted(got - want)
        print(f"① 경로  git {len(want)} · 대장 {len(got)} · "
              f"git에만 {len(only_git)} · 대장에만 {len(only_ledger)}")
        if only_git or only_ledger:
            failures.append("① 경로 집합 불일치")
            for p in only_git:
                print(f"    git에만: {p}")
            for p in only_ledger:
                print(f"    대장에만: {p}")
        if len(got) != n:
            failures.append(f"① 경로 중복 — 항목 {n} 인데 고유 경로 {len(got)}")

        # ── ② 상태 분할 ────────────────────────────────────────────────────
        counts = dict.fromkeys(BUCKETS, 0)
        for e in entries:
            counts[bucket_of(e["state"])] += 1
        total = sum(counts.values())
        print(f"② 분할  합 {total} / 파일 {n}  ·  " +
              " · ".join(f"{b} {counts[b]}" for b in BUCKETS if counts[b]))
        if total != n:
            failures.append(f"② 상태 분할이 전수가 아니다 — 합 {total} ≠ {n}")

        # ── ③ 2회차 ────────────────────────────────────────────────────────
        second = ledger(pal, repo, a.at, cache)
        same = json.dumps(first["ledger"], sort_keys=True) == json.dumps(
            second["ledger"], sort_keys=True)
        print(f"③ 캐시  2회차 적중 {second['cache']['hits']} · "
              f"빗나감 {second['cache']['misses']} · 산출 동일 {'예' if same else '아니오'}")
        if second["cache"]["misses"] != 0:
            failures.append(f"③ 2회차 미스 {second['cache']['misses']} — 0 이어야 한다")
        if not same:
            failures.append("③ 두 회차의 대장 산출이 다르다 — 캐시가 값을 바꿨다")

        # ── ④ 음성 대조 ────────────────────────────────────────────────────
        # **캐시가 변경을 보는지 확인한다.** 고정 SHA 는 바꿀 수 없으므로 작은 저장소를
        # 새로 만든다. 0 이면 캐시가 눈이 먼 것이고, 2 이상이면 키가 파일 경계를 넘는다.
        toy = tmp / "toy"
        toy.mkdir()
        run(["git", "init", "-q", str(toy)])
        run(["git", "config", "user.email", "s1@verify"], cwd=toy)
        run(["git", "config", "user.name", "s1"], cwd=toy)
        for i in range(5):
            (toy / f"F{i}.kt").write_text(f"class F{i}\nfun g{i}() {{}}\n", encoding="utf-8")
        run(["git", "add", "-A"], cwd=toy)
        run(["git", "commit", "-qm", "a"], cwd=toy)

        toy_cache = tmp / "toy-cache"
        r1 = ledger(pal, toy, None, toy_cache)
        r2 = ledger(pal, toy, None, toy_cache)

        (toy / "F2.kt").write_text("class F2\nfun g2() {}\nclass Added\n", encoding="utf-8")
        run(["git", "add", "-A"], cwd=toy)
        run(["git", "commit", "-qm", "b"], cwd=toy)
        r3 = ledger(pal, toy, None, toy_cache)

        m1, m2, m3 = (r["cache"]["misses"] for r in (r1, r2, r3))
        print(f"④ 음성  최초 {m1} · 재실행 {m2} · 1파일 변경 후 {m3}  (기대 5 · 0 · 1)")
        if m2 != 0:
            failures.append(f"④ 변경 없는 재실행에서 미스 {m2}")
        if m3 != 1:
            failures.append(
                f"④ 1파일 변경 후 미스가 {m3} — 0 이면 캐시가 변경을 못 보고, "
                "2 이상이면 키가 파일 경계를 넘는다")

        # 변경된 파일의 선언 수가 실제로 늘었는지 — 캐시가 옛 값을 준 것이 아님을 본다
        added = [e for e in r3["ledger"]["entries"] if e["path"] == "F2.kt"]
        if not added:
            failures.append("④ 변경한 파일이 대장에 없다")

    finally:
        shutil.rmtree(tmp, ignore_errors=True)

    print()
    if failures:
        print("어긋난 것:")
        for f in failures:
            print(f"  · {f}")
        print("\n반증이다")
        return 1
    print("넷 다 통과 — ⑤ gix 격리는 `cargo xtask check` 가 센다")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
