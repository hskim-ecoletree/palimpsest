#!/usr/bin/env python3
"""F02-4(#49) 대조 — **결정적인가 · 선형인가. 그리고 그 상수가 무엇을 말하는가.**

합격선 정본은 `corpus/criteria.toml` `[f02.4]` 이고 판정은 `docs/gates/F02-4-parallel.md` 다.

이 스크립트가 재는 것:

    ①  병렬 결정성 — **회차 다섯**이 바이트 단위로 같은가. 그리고 **직렬과도** 같은가
    ②  차등 재추출 — 캐시 **미스 경로**와 **적중 경로**가 같은 답을 내는가
    ③  음성 대조 — 셋은 반드시 깨고 **하나는 반드시 안 깬다**(스레드 수)
    ④  선형성 — 규모 넷에서 제곱이 아닌가. **절대 시간은 합격선이 아니다**
    ⑤  트리 즉시 폐기 — 최대 상주 메모리가 **파일 수에 비례하지 않는가**

## ① 이 자기 산출을 자기가 확인하는 형태가 아닌 이유

*"두 번 돌려 같은가"* 는 바깥 표를 요구하지 않는다. 비교 대상이 **두 개의 독립한
실행**이고, 순서 의존 버그는 바로 그 둘을 갈라 놓는다. **한 번 더가 아니라 다섯 번이다**
— 경합은 간헐적이고 두 번으로는 안 드러난다.

그리고 **직렬과도 댄다.** 병렬끼리만 같으면 *"병렬 경로 전체가 일관되게 틀린 것"* 이
통과한다.

## ③ 은 소스를 변이시키고 다시 빌드한다

*"결정적이다"* 가 참인지 보려면 실제로 비결정적으로 만들어 봐야 한다. 산출 JSON 을
손보는 것으로는 **비교 함수가 diff 를 낸다**는 것만 보이지 *비결정성이 산출에 실린다*는
것을 보이지 않는다 — `f01-verify` ⑦ 과 같은 판단이다.

**끝나면 소스를 되돌리고 다시 빌드한다.** 도중에 죽으면 변이가 남으므로 `git status` 로
확인할 것.

사용:
    ./scripts/f02-4-verify.py --repo ~/dev/projects/boxwood/portal-backend

종료 코드:
    0  다섯 다 통과
    1  어긋난 것이 있다 · 또는 대조가 성립하지 않았다
"""

from __future__ import annotations

import argparse
import json
import os
import platform
import re
import shutil
import subprocess
import sys
import tempfile
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
AT = "a29cad0bf6a8"
RUNS = 5

LEDGER_RS = ROOT / "crates/pal-cli/src/ledger.rs"

# ── ③ 음성 대조 — **셋은 반드시 깬다** ────────────────────────────────────────
#
# 각각 다른 고장이다. 치환 대상이 소스에 없으면 `✓` 를 내는 대신 **멈춘다** — 리팩터가
# 이 자리를 지워도 대조가 조용히 계속되면 그 대조는 장식이다.
MUST_BREAK = [
    (
        "산출을 해시맵 순회 순서로 낸다",
        "        for ((path, _), outcome) in chunk.iter().zip(outcomes) {",
        "        let mut 뒤섞음: std::collections::HashMap<usize, _> =\n"
        "            chunk.iter().zip(outcomes).enumerate().collect();\n"
        "        for (_, ((path, _), outcome)) in 뒤섞음.drain() {",
    ),
    (
        "심볼 순서를 스레드 완료 순으로 낸다",
        "        let fresh: Vec<Result<FileOutcome>> = pending\n"
        "            .par_iter()\n"
        "            .map(|(i, source, declared)| {\n"
        "                let path = &chunk[*i].0;\n"
        "                pal_extract::classify(path, source, OVERSIZE_BYTES, declared.as_deref())\n"
        "                    .with_context(|| format!(\"분류 실패: {path}\"))\n"
        "            })\n"
        "            .collect();",
        "        let 통 = std::sync::Mutex::new(Vec::new());\n"
        "        pending.par_iter().for_each(|(i, source, declared)| {\n"
        "            let path = &chunk[*i].0;\n"
        "            let r = pal_extract::classify(path, source, OVERSIZE_BYTES, declared.as_deref())\n"
        "                .with_context(|| format!(\"분류 실패: {path}\"));\n"
        "            통.lock().expect(\"독점\").push(r);\n"
        "        });\n"
        "        let fresh: Vec<Result<FileOutcome>> = 통.into_inner().expect(\"독점\");",
    ),
    (
        "캐시 적중 경로에서 다른 값을 낸다",
        "                outcomes.push(Some(hit));",
        "                outcomes.push(Some(FileOutcome {\n"
        "                    state: pal_core::FileState::Unrecognized,\n"
        "                    ..hit\n"
        "                }));",
    ),
]


def die(msg: str) -> None:
    raise SystemExit(f"대조가 성립하지 않는다: {msg}")


def run(cmd, cwd=None, env=None, check=True):
    e = {**os.environ, **(env or {})}
    p = subprocess.run(cmd, cwd=cwd, capture_output=True, text=True, env=e, check=False)
    if check and p.returncode != 0:
        raise SystemExit(f"실패: {' '.join(map(str, cmd))}\n{p.stderr.strip()}")
    return p.stdout


def build() -> bool:
    p = subprocess.run(
        ["cargo", "build", "--release", "--quiet"], cwd=ROOT, capture_output=True, text=True, check=False
    )
    return p.returncode == 0


def ledger_text(pal: Path, repo: Path, cache: Path, threads: int | None = None,
                at: str = AT) -> str:
    """대장 **하나**를 JSON 문자열로 — 캐시 회계는 뺀다.

    두 회차의 캐시 수는 당연히 다르다(1회차 미스 · 2회차 적중). 비교 대상이 갈려야
    한다 — `[s1.pass]` 가 같은 자리에서 내린 판단이다.
    """
    env = {"RAYON_NUM_THREADS": str(threads)} if threads else {}
    out = run(
        [str(pal), "ledger", str(repo), "--at", at, "--cache-dir", str(cache), "--json"], env=env
    )
    return json.dumps(json.loads(out)["ledger"], sort_keys=True, ensure_ascii=False)


def max_rss_mb(pal: Path, repo: Path, cache: Path, at: str = AT) -> float:
    """최대 상주 메모리(MB). **`/usr/bin/time -l` 이 그것을 안다.**"""
    p = subprocess.run(
        ["/usr/bin/time", "-l", str(pal), "ledger", str(repo), "--at", at,
         "--cache-dir", str(cache), "--json"],
        capture_output=True, text=True, check=False,
    )
    m = re.search(r"(\d+)\s+maximum resident set size", p.stderr)
    if not m:
        die("최대 상주 메모리를 읽지 못했다 — `/usr/bin/time -l` 이 없는 플랫폼인가")
    return int(m.group(1)) / (1024 * 1024)


def synth_scale(tmp: Path, repo: Path, copies: list[int]):
    """같은 저장소를 복제해 **규모만** 바꾼다 — `f01-verify` ⑧ 과 같은 형태.

    실물 저장소들을 나란히 재면 그것은 파일 수의 함수가 아니라 **언어 구성의 함수**다.
    사본마다 고유 표식을 넣어 blob 을 다르게 만든다 — 안 그러면 캐시가 과도하게 적중해
    재는 것이 추출이 아니라 캐시 읽기가 된다.

    **이것은 "실물 10⁴ 저장소" 가 아니다.** 사본이므로 구문 다양성이 원본만큼이 아니고
    디렉터리 깊이가 하나 는다. 그 사실이 게이트에 적힌다.
    """
    synth = tmp / "scale"
    synth.mkdir()
    run(["git", "init", "--quiet", str(synth)])
    run(["git", "config", "user.email", "s@e"], cwd=synth)
    run(["git", "config", "user.name", "s"], cwd=synth)
    made = 0
    for k in copies:
        while made < k:
            root = synth / f"copy{made}"
            shutil.copytree(repo, root, ignore=shutil.ignore_patterns(".git", ".palimpsest"))
            for f in root.rglob("*"):
                if f.is_file() and not f.is_symlink():
                    with open(f, "ab") as fh:
                        fh.write(f"\n// palimpsest-scale-{made}\n".encode())
            made += 1
        run(["git", "add", "-A"], cwd=synth)
        run(["git", "commit", "--quiet", "-m", f"copies {k}", "--allow-empty"], cwd=synth)
        n = len(run(["git", "ls-tree", "-r", "HEAD", "--name-only"], cwd=synth).split("\n")) - 1
        yield k, synth, n


def main() -> int:  # noqa: PLR0915 — 다섯 검사가 한 흐름으로 읽혀야 한다
    ap = argparse.ArgumentParser()
    ap.add_argument("--repo", type=Path, default=Path("~/dev/projects/boxwood/portal-backend"))
    ap.add_argument("--bin", type=Path, default=ROOT / "target/release/pal")
    ap.add_argument("--skip-rebuild", action="store_true",
                    help="③ 을 건너뛴다 (소스를 변이시키고 여러 번 빌드한다)")
    a = ap.parse_args()
    pal, origin = a.bin.resolve(), a.repo.expanduser().resolve()
    if not pal.exists():
        die(f"바이너리가 없다: {pal} — `cargo build --release` 를 먼저 하라")

    failures: list[str] = []
    tmp = Path(tempfile.mkdtemp(prefix="f02-4-verify-"))
    try:
        repo = tmp / "pb"
        run(["git", "clone", "--quiet", "--local", "--no-hardlinks", str(origin), str(repo)])
        run(["git", "checkout", "--quiet", AT], cwd=repo)

        # ── ① 병렬 결정성 ──────────────────────────────────────────────────
        print("── ① 병렬 결정성 — **회차 다섯**, 그리고 직렬과도 ───────────────")
        outs = []
        for i in range(RUNS):
            # **회차마다 캐시를 비운다.** 안 그러면 2회차부터 캐시를 읽을 뿐이고,
            # 그것은 병렬 추출이 아니라 파일 읽기의 결정성을 재는 것이다.
            outs.append(ledger_text(pal, repo, tmp / f"c{i}"))
        same = len(set(outs)) == 1
        serial = ledger_text(pal, repo, tmp / "cs", threads=1)
        with_serial = serial == outs[0]
        print(f"  병렬 {RUNS} 회차가 바이트 단위로 {'같다' if same else '**다르다**'}"
              f"  (서로 다른 산출 {len(set(outs))} 종)")
        print(f"  직렬(RAYON_NUM_THREADS=1)과도 {'같다' if with_serial else '**다르다**'}")
        if not same:
            failures.append(f"① 회차마다 산출이 다르다 — 순서 의존이다. `symbol_id` 가 움직인다")
        if not with_serial:
            failures.append("① 병렬과 직렬이 다르다 — 병렬 경로 전체가 일관되게 틀렸을 수 있다")

        # ── ② 차등 재추출 ──────────────────────────────────────────────────
        print()
        print("── ② 차등 재추출 — 캐시 **미스 경로**와 **적중 경로** ───────────")
        warm = tmp / "warm"
        cold_out = ledger_text(pal, repo, warm)          # 전량 미스
        warm_out = ledger_text(pal, repo, warm)          # 전량 적중
        stats = json.loads(run([str(pal), "ledger", str(repo), "--at", AT,
                                "--cache-dir", str(warm), "--json"]))["cache"]
        agree = cold_out == warm_out
        print(f"  두 경로의 산출이 {'같다' if agree else '**다르다**'}"
              f"   (적중 {stats['hits']} · 빗나감 {stats['misses']})")
        if stats["hits"] == 0:
            failures.append("② 캐시가 한 번도 적중하지 않았다 — 적중 경로를 재지 못했다")
        if not agree:
            failures.append("② 캐시 적중 경로와 미스 경로가 다른 답을 냈다 — ADR-0004 의 키가 부족하다")

        # ── ③ 음성 대조 ────────────────────────────────────────────────────
        print()
        print("── ③ 음성 대조 — 셋은 반드시 깨고 **하나는 반드시 안 깬다** ─────")
        tested = 0
        if a.skip_rebuild:
            print("  **건너뛰었다** — `--skip-rebuild`. 이것은 통과가 아니다")
            failures.append("③ 음성 대조를 돌리지 않았다 — 「–」 는 통과가 아니다")
        else:
            saved = LEDGER_RS.read_text(encoding="utf-8")
            try:
                for idx, (label, find, repl) in enumerate(MUST_BREAK):
                    if find not in saved:
                        raise SystemExit(
                            f"변이 대상을 찾지 못했다 — 「{label}」\n  찾은 것: {find[:70]!r}…\n"
                            "  **리팩터가 이 자리를 옮겼다.** 고치지 않으면 이 대조가 조용히 꺼진다."
                        )
                    LEDGER_RS.write_text(saved.replace(find, repl, 1), encoding="utf-8")
                    if not build():
                        failures.append(f"③ 「{label}」 변이의 빌드가 실패했다 — 변이가 낡았다")
                        continue
                    # ⚠ **변이마다 캐시를 갈라야 한다.**
                    #
                    # 한 캐시를 돌려 쓰면 둘째 변이부터는 전량 적중이라 **병렬 구간이
                    # 아예 안 돈다** — 그러면 병렬을 깨뜨리는 변이가 조용히 통과한다.
                    # 실제로 처음에 그렇게 짰고 「스레드 완료 순」 변이가 통과했다.
                    # 자라는 값이 아니라 **공유 상태**에 묶여 꺼진 형태다.
                    a_out = ledger_text(pal, repo, tmp / f"m{idx}a")
                    b_out = ledger_text(pal, repo, tmp / f"m{idx}b")
                    # 셋째는 **일부러 같은 캐시**를 다시 쓴다 — 적중 경로를 밟는다.
                    c_out = ledger_text(pal, repo, tmp / f"m{idx}a")
                    broke = (a_out != b_out) or (a_out != c_out) or (a_out != outs[0])
                    tested += 1
                    print(f"  {'✓' if broke else '✗'} {label:<28} "
                          f"{'대조가 잡았다' if broke else '**놓쳤다**'}")
                    if not broke:
                        failures.append(f"③ 「{label}」 를 ①·② 가 못 잡았다 — 그 고장을 아무도 못 잡는다")
            finally:
                LEDGER_RS.write_text(saved, encoding="utf-8")
                if not build():
                    die("변이를 되돌린 뒤 빌드가 실패했다 — **소스가 변이 상태로 남았을 수 있다**")

        # **반대 방향** — 스레드 수를 바꿔도 산출이 같아야 한다.
        #
        # 다르면 그 추출은 스레드 수에 의존하고, 그러면 **기계마다 다른 `symbol_id`** 가
        # 나온다. 시간은 움직여도 되지만 좌표는 안 된다.
        by_threads = {t: ledger_text(pal, repo, tmp / f"t{t}", threads=t) for t in (1, 2, 4, 8)}
        stable = len(set(by_threads.values())) == 1
        tested += 1
        print(f"  {'✓' if stable else '✗'} {'스레드 수를 바꾼다':<28} "
              f"{'산출이 그대로다' if stable else '**바뀌었다**'}   (1 · 2 · 4 · 8)")
        if not stable:
            failures.append("③ 스레드 수에 따라 산출이 다르다 — 기계마다 다른 `symbol_id` 가 나온다")
        want = len(MUST_BREAK) + 1
        if tested != want and not a.skip_rebuild:
            failures.append("③ 시험되지 않은 대조가 있다 — 「–」 는 통과가 아니다")
        print(f"  시험한 대조 {tested}/{want}")

        # ── ④ 선형성 · ⑤ 메모리 ────────────────────────────────────────────
        print()
        print("── ④ 선형성 · ⑤ 트리 즉시 폐기 — **형태를 본다** ────────────────")
        print(f"  기계  {platform.platform()} · CPU {os.cpu_count()} · "
              f"rustc {run(['rustc', '--version']).strip()}")
        points = []
        for k, synth, n in synth_scale(tmp, repo, [1, 2, 4, 10]):
            cache = tmp / f"scale-c{k}"
            t0 = time.monotonic()
            ledger_text(pal, synth, cache, at="HEAD")
            secs = time.monotonic() - t0
            shutil.rmtree(cache, ignore_errors=True)
            rss = max_rss_mb(pal, synth, tmp / f"scale-r{k}", at="HEAD")
            points.append((n, secs, rss))
            print(f"  {n:>6} 파일  {secs:7.3f}s  ·  파일당 {secs / n * 1000:6.3f}ms  "
                  f"·  최대 상주 {rss:7.1f} MB")
        if len(points) < 4:
            failures.append(f"④ 규모 점 {len(points)}/4 — 대조 불가")
        else:
            (n0, t0, r0), (n1, t1, r1) = points[0], points[-1]
            per = (t1 / n1) / (t0 / n0)
            scale = n1 / n0
            print(f"  규모 {scale:.1f} 배에서 파일당 시간 {per:.2f} 배  "
                  f"(제곱이면 {scale:.1f} 배가 된다)")
            print(f"  규모 {scale:.1f} 배에서 최대 상주 {r1 / r0:.2f} 배  "
                  f"(트리를 안 버리면 {scale:.1f} 배가 된다)")
            if per >= scale:
                failures.append(f"④ 파일당 시간이 규모에 비례한다 — 제곱 비용이다 ({per:.2f} 배)")
            if r1 / r0 >= scale:
                failures.append(f"⑤ 최대 상주가 파일 수에 비례한다 — 트리를 안 버리고 있다 ({r1 / r0:.2f} 배)")
        print("  **절대 시간은 합격선이 아니다** — 값이 아니라 형태를 본다. "
              "10⁵ 판정은 R-24 에 따라 P1 종료 시점이다")

        print()
        if failures:
            print("반증 — 어긋난 것:")
            for f in failures:
                print(f"  · {f}")
            return 1
        print("다섯 다 통과")
        return 0
    finally:
        shutil.rmtree(tmp, ignore_errors=True)


if __name__ == "__main__":
    sys.exit(main())
