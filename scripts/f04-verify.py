#!/usr/bin/env python3
"""F04(#7) 대조 — **「적중했다」가 참인가, 그리고 지우는 명령이 경계를 지키는가.**

합격선 정본은 `corpus/criteria.toml` `[f04]` 이고 판정은 `docs/gates/F04.md` 다.

이 스크립트가 재는 것 일곱 (열하나 중 ②⑥ 은 `cargo test` 가 상시로 재고 ⑩ 은 기권이다):

    ①  캐시 유/무 산출이 **바이트로 같은가**
    ③  ★ **적중이 0 이어야 하는 회차**에서 0 인가 — 축 셋을 실제로 움직여 본다
    ④  ★ **축출이 실제로 축출했는가** — 그리고 예산이 넉넉하면 **0 건**인가
    ⑤  ★ **손상은 사건이고 부재는 정상인가**
    ⑦  적중률 시나리오 넷 — 목표값이 아니라 **등식**이다
    ⑧  파일당 압축 바이트 (합격선 2KB)
    ⑨  벤치 — **합격선은 비율**이고 절대 시간은 기록이다. G50 이 남긴 비용도 여기서

## ③ 은 소스를 변이시키고 다시 빌드한다

*"키가 안 샌다"* 가 참인지 보려면 **축을 실제로 움직여야** 한다. 키를 손으로 만들어
비교하는 것은 `cargo test` 가 이미 하고 있고, 그것은 *"`CacheKey::new` 가 성분을
섞는다"* 만 말한다 — **그 성분이 실물 회차에 실제로 실리는지**는 말하지 않는다.
`f01-verify` ⑦ · `f02-4-verify` ③ 과 같은 판단이다.

**변이 셋은 서로 다른 축이다**: 추출기 코드 버전 · 문법 버전 · **능력 축**.
셋째가 이 기능이 새로 만든 축이고, 그래서 셋째가 가장 중요하다.

**끝나면 소스를 되돌리고 다시 빌드한다.** 도중에 죽으면 변이가 남으므로 `git status`
로 확인할 것.

## 회차마다 캐시 디렉터리를 새로 만든다

F02-4 가 변이 셋에 캐시 디렉터리를 **돌려 써서** 병렬 구간이 아예 안 돌았다
(`[f04].self_judged` ③). **캐시 기능의 대조는 그 함정 위에 통째로 서 있다.**
그리고 `--cache-dir` 을 **언제나 명시한다** — 안 주면 `<저장소>/.palimpsest/cache` 로
가고 그것이 곧 회차 사이의 공유 상태다.

사용:
    ./scripts/f04-verify.py
    ./scripts/f04-verify.py --skip-mutation      # ③ 을 건너뛴다 (빌드 넷을 아낀다)

종료 코드:
    0  일곱 다 통과
    1  어긋난 것이 있다 · 또는 대조가 성립하지 않았다
"""

from __future__ import annotations

import argparse
import json
import os
import platform
import shutil
import subprocess
import sys
import tempfile
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
BIN = ROOT / "target/release/pal"

DITTO = (Path.home() / "dev/projects/ditto", "aded7ce7f88f", "ditto")
PORTAL = (Path.home() / "dev/projects/boxwood/portal-backend", "a29cad0bf6a8", "portal-backend")

# **하한이다.** 이보다 적으면 시험되지 않은 것이고, 시험되지 않은 대조는 `–` 가 아니라
# 실패다(`2e2eb3f`).
최소_파일 = {"ditto": 400, "portal-backend": 900}

LIB_RS = ROOT / "crates/pal-extract/src/lib.rs"
KOTLIN_RS = ROOT / "crates/pal-extract/src/kotlin.rs"

실패: list[str] = []
기록: list[str] = []


def 적음(line: str = "") -> None:
    print(line)


def ok(line: str) -> None:
    적음(f"  ok    {line}")


def 어긋남(line: str) -> None:
    적음(f"  FAIL  {line}")
    실패.append(line)


def run(args: list[str], cwd: Path | None = None) -> subprocess.CompletedProcess:
    return subprocess.run(args, cwd=cwd, capture_output=True, text=True, check=False)


def pal(args: list[str], cwd: Path | None = None) -> str:
    p = run([str(BIN), *args], cwd=cwd)
    if p.returncode != 0:
        raise SystemExit(f"pal {args} 실패:\n{p.stderr[-800:]}")
    return p.stdout


def 대장(repo: Path, at: str | None, cache: Path, symbols: bool = False) -> str:
    args = ["ledger", str(repo), "--cache-dir", str(cache)]
    if at:
        args += ["--at", at]
    args += ["--symbols"] if symbols else ["--json"]
    return pal(args)


def 회계(repo: Path, at: str | None, cache: Path) -> tuple[int, int, int]:
    d = json.loads(대장(repo, at, cache))
    c = d["cache"]
    return c["hits"], c["misses"], c["corrupt"]


def 산출만(json_text: str) -> str:
    """**회계를 뺀 산출.** 캐시 수는 회차마다 다른 것이 정상이다(`[s1.pass]`)."""
    d = json.loads(json_text)
    d.pop("cache", None)
    d.pop("corrupt", None)
    return json.dumps(d, sort_keys=True, ensure_ascii=False)


def 새_캐시(tag: str) -> Path:
    p = Path(tempfile.mkdtemp(prefix=f"f04-{tag}-"))
    return p / "cache"


def 캐시_파일들(cache: Path) -> list[Path]:
    if not cache.exists():
        return []
    return [
        f
        for shard in cache.iterdir()
        if shard.is_dir() and shard.name != ".corrupt"
        for f in shard.iterdir()
        if f.is_file() and not f.name.endswith(".tmp")
    ]


# ═════════════════════════════════════════════════════════════════════════════
# ① 캐시 유/무 산출이 바이트로 같다
# ═════════════════════════════════════════════════════════════════════════════


def 하나(name: str, repo: Path, at: str) -> None:
    차가운 = 새_캐시(f"{name}-cold")
    더운 = 새_캐시(f"{name}-warm")

    없이 = 대장(repo, at, 차가운)
    파일수 = len(json.loads(없이)["ledger"]["entries"])
    if 파일수 < 최소_파일[name]:
        어긋남(f"① {name} — 파일이 {파일수} 개다. 하한 {최소_파일[name]} 미만이라 시험되지 않았다")
        return

    대장(repo, at, 더운)  # 채운다
    있이 = 대장(repo, at, 더운)  # 전부 적중일 회차
    적중, 빗나감, _ = 회계(repo, at, 더운)

    if 적중 == 0:
        어긋남(f"① {name} — 둘째 회차의 적중이 0 이다. 캐시가 아예 안 쓰였다")
        return
    if 산출만(없이) != 산출만(있이):
        어긋남(f"① {name} — 캐시 유/무의 산출이 다르다")
        return

    심볼_없이 = 대장(repo, at, 새_캐시(f"{name}-s1"), symbols=True)
    심볼_있이 = 대장(repo, at, 더운, symbols=True)
    if 심볼_없이 != 심볼_있이:
        어긋남(f"① {name} — 캐시 유/무의 심볼 산출이 다르다")
        return

    ok(f"① {name}  파일 {파일수} · 적중 {적중} · 빗나감 {빗나감} · 산출 동일(대장·심볼 둘 다)")


def 검사1() -> None:
    적음("① 캐시 유/무 산출이 바이트로 같은가")
    for repo, at, name in (DITTO, PORTAL):
        하나(name, repo, at)
    적음()


# ═════════════════════════════════════════════════════════════════════════════
# ③ ★ 적중이 0 이어야 하는 회차 — **축을 실제로 움직인다**
# ═════════════════════════════════════════════════════════════════════════════

변이 = [
    (
        "추출기 코드 버전",
        LIB_RS,
        'pub const EXTRACTOR_REV: &str = "f03-2";',
        'pub const EXTRACTOR_REV: &str = "f04-mutant";',
    ),
    (
        "문법 버전",
        LIB_RS,
        'pub const GRAMMAR_REV: &str = "acb96307d816618bd60e1e4d2fa3eaa793e97a2e";',
        'pub const GRAMMAR_REV: &str = "0000000000000000000000000000000000000000";',
    ),
    (
        "능력 축 — Kotlin 이 exports 를 만들기 시작한다",
        KOTLIN_RS,
        'Capable::not_built(CapabilityId::new("F02", "kotlin-exports")),',
        "Capable::Present(pal_core::ExportSet::default()),",
    ),
]


def 빌드() -> None:
    p = run(["cargo", "build", "--workspace", "--release"], cwd=ROOT)
    if p.returncode != 0:
        raise SystemExit(f"빌드 실패:\n{p.stderr[-2000:]}")


def 검사3(skip: bool) -> None:
    적음("③ ★ 적중이 0 이어야 하는 회차 — 축 셋을 실제로 움직인다")
    if skip:
        적음("  –     `--skip-mutation` — **건너뛴 것은 통과가 아니다**")
        실패.append("③ 을 건너뛰었다")
        적음()
        return

    repo, at, name = PORTAL
    cache = 새_캐시("axis")
    대장(repo, at, cache)
    적중_전, _, _ = 회계(repo, at, cache)
    # **하한이다.** 캐시가 안 찼으면 아래 셋이 공짜로 통과한다.
    if 적중_전 == 0:
        어긋남("③ 채우기 회차의 적중이 0 이다 — 아래 셋이 아무것도 재지 않는다")
        적음()
        return
    ok(f"③ 채움 — 변이 전 적중 {적중_전}")

    원본 = {LIB_RS: LIB_RS.read_text(), KOTLIN_RS: KOTLIN_RS.read_text()}
    try:
        for 이름, 파일, 찾을, 바꿀 in 변이:
            text = 파일.read_text()
            if 찾을 not in text:
                # **치환 대상이 없으면 멈춘다.** 리팩터가 이 자리를 지워도 대조가 조용히
                # 계속되면 그 대조는 장식이다(`[f04].self_judged` ④).
                어긋남(f"③ {이름} — 치환 대상이 소스에 없다: {찾을[:50]}")
                continue
            파일.write_text(text.replace(찾을, 바꿀, 1))
            빌드()
            적중, 빗나감, _ = 회계(repo, at, cache)
            파일.write_text(원본[파일])
            if 적중 != 0:
                어긋남(f"③ {이름} — 축을 움직였는데 적중이 {적중} 이다. **키가 샌다**")
            else:
                ok(f"③ {이름} — 적중 0 · 빗나감 {빗나감} ★")
    finally:
        for 파일, text in 원본.items():
            파일.write_text(text)
        빌드()
    적음()


# ═════════════════════════════════════════════════════════════════════════════
# ④ ★ 축출이 실제로 축출했다
# ═════════════════════════════════════════════════════════════════════════════


def 검사4() -> None:
    적음("④ ★ 축출이 실제로 줄이고, 예산이 넉넉하면 한 건도 안 지운다")
    repo, at, name = PORTAL
    cache = 새_캐시("evict")
    대장(repo, at, cache)

    전_파일 = len(캐시_파일들(cache))
    if 전_파일 < 100:
        어긋남(f"④ 캐시 엔트리가 {전_파일} 개다 — 100 미만이면 시험되지 않았다")
        적음()
        return
    전_바이트 = sum(f.stat().st_size for f in 캐시_파일들(cache))

    # ★ 반대 방향 먼저 — 넉넉하면 0 건.
    보고 = json.loads(pal(["cache", "prune", "--repo", str(repo), "--cache-dir", str(cache),
                          "--budget", str(전_바이트 * 10), "--json"]))
    if 보고["report"]["removed"] != 0 or len(캐시_파일들(cache)) != 전_파일:
        어긋남(f"④ 예산이 넉넉한데 {보고['report']['removed']} 건을 지웠다")
    else:
        ok(f"④ 예산 넉넉 — 지움 0 · 엔트리 {전_파일} 그대로 ★")

    예산 = 전_바이트 // 10
    보고 = json.loads(pal(["cache", "prune", "--repo", str(repo), "--cache-dir", str(cache),
                          "--budget", str(예산), "--json"]))
    r = 보고["report"]
    후_파일 = 캐시_파일들(cache)
    후_바이트 = sum(f.stat().st_size for f in 후_파일)

    if r["removed"] == 0:
        어긋남("④ 예산을 1/10 로 줬는데 한 건도 안 지웠다")
    elif len(후_파일) != 전_파일 - r["removed"]:
        어긋남(f"④ 보고 {r['removed']} 건과 실제 감소 {전_파일 - len(후_파일)} 건이 다르다")
    elif 후_바이트 > 예산:
        어긋남(f"④ 지우고도 예산을 넘는다 — {후_바이트} > {예산}")
    else:
        ok(f"④ 축출 — 훑음 {r['scanned']} · 지움 {r['removed']} · "
           f"파일 {전_파일} → {len(후_파일)} · 바이트 {전_바이트} → {후_바이트} (예산 {예산})")

    # 축출 뒤의 회차는 **미스가 늘고 산출은 그대로**여야 한다.
    적중, 빗나감, 손상 = 회계(repo, at, cache)
    if 손상 != 0:
        어긋남(f"④ 축출 뒤에 손상이 {손상} 건 나왔다 — 축출은 손상이 아니다")
    elif 빗나감 == 0:
        어긋남("④ 축출했는데 빗나감이 0 이다")
    else:
        ok(f"④ 축출 뒤 회차 — 적중 {적중} · 빗나감 {빗나감} · 손상 0 ★")
    적음()


# ═════════════════════════════════════════════════════════════════════════════
# ⑤ ★ 손상은 사건이고 부재는 정상이다
# ═════════════════════════════════════════════════════════════════════════════


def 검사5() -> None:
    적음("⑤ ★ 손상은 사건이고 부재는 정상이다")
    repo, at, name = PORTAL

    # (가) 하나를 망가뜨린다 → 손상 1 · 산출은 그대로.
    cache = 새_캐시("corrupt")
    기준 = 대장(repo, at, cache)
    파일들 = sorted(캐시_파일들(cache))
    if not 파일들:
        어긋남("⑤ 캐시가 비었다 — 망가뜨릴 것이 없다")
        적음()
        return
    파일들[0].write_bytes("zstd 가 아니다".encode())
    적중, 빗나감, 손상 = 회계(repo, at, cache)
    산출 = 대장(repo, at, cache)
    격리 = list((cache / ".corrupt").iterdir()) if (cache / ".corrupt").exists() else []

    if 손상 != 1:
        어긋남(f"⑤ 하나를 망가뜨렸는데 손상이 {손상} 건이다")
    elif 산출만(산출) != 산출만(기준):
        어긋남("⑤ 손상 뒤의 산출이 달라졌다 — 재계산이 안 됐다")
    elif len(격리) != 1:
        어긋남(f"⑤ 격리된 파일이 {len(격리)} 개다 — 지웠거나 안 옮겼다")
    else:
        ok(f"⑤ 손상 1 · 격리 1(바이트 남음) · 산출 동일 · 적중 {적중} · 빗나감 {빗나감}")

    # (나) ★ 반대 방향 — 하나를 **지우면** 미스만 늘고 손상은 0.
    cache2 = 새_캐시("absent")
    대장(repo, at, cache2)
    파일들2 = sorted(캐시_파일들(cache2))
    파일들2[0].unlink()
    적중2, 빗나감2, 손상2 = 회계(repo, at, cache2)
    if 손상2 != 0:
        어긋남(f"⑤ ★ 지운 것을 손상 {손상2} 건으로 셌다 — 축출 뒤의 미스가 사건이 된다")
    elif 빗나감2 != 1:
        어긋남(f"⑤ ★ 하나를 지웠는데 빗나감이 {빗나감2} 건이다")
    else:
        ok(f"⑤ ★ 지운 것 — 빗나감 1 · 손상 0 (적중 {적중2})")

    # (다) ★ 멀쩡한 캐시에서 손상 0.
    cache3 = 새_캐시("clean")
    대장(repo, at, cache3)
    _, _, 손상3 = 회계(repo, at, cache3)
    if 손상3 != 0:
        어긋남(f"⑤ ★ 멀쩡한 캐시에서 손상이 {손상3} 건이다 — 계수기가 아무거나 센다")
    else:
        ok("⑤ ★ 멀쩡한 캐시 — 손상 0")
    적음()


# ═════════════════════════════════════════════════════════════════════════════
# ⑦ 적중률 시나리오 넷 — 등식이다
# ═════════════════════════════════════════════════════════════════════════════


def 검사7() -> None:
    적음("⑦ 적중률 시나리오 넷 — 목표값이 아니라 등식이다")
    repo, at, name = PORTAL
    작업 = Path(tempfile.mkdtemp(prefix="f04-scenario-"))
    사본 = 작업 / "repo"
    p = run(["git", "clone", "--quiet", "--local", str(repo), str(사본)])
    if p.returncode != 0:
        어긋남(f"⑦ 코퍼스 사본을 못 만들었다: {p.stderr[-300:]}")
        적음()
        return
    run(["git", "checkout", "--quiet", at], cwd=사본)

    cache = 새_캐시("scenario")
    대장(사본, at, cache)

    # 가 — 2회차: 미스 0
    적중, 빗나감, _ = 회계(사본, at, cache)
    (ok if 빗나감 == 0 else 어긋남)(
        f"⑦가 2회차 — 적중 {적중} · 빗나감 {빗나감} (기대 0)"
    )

    # 나 — 워킹트리 편집: 미스 = 편집한 파일 수
    kt = sorted(사본.rglob("*.kt"))
    if len(kt) < 2:
        어긋남("⑦나 — 편집할 `.kt` 가 둘 미만이다")
    else:
        대장(사본, None, cache)  # 워킹트리 회차를 한 번 채운다
        kt[0].write_text(kt[0].read_text() + "\n// 이 줄이 blob 을 바꾼다\n")
        적중, 빗나감, _ = 회계(사본, None, cache)
        (ok if 빗나감 == 1 else 어긋남)(
            f"⑦나 워킹트리 편집 1 파일 — 빗나감 {빗나감} (기대 1) · 적중 {적중}"
        )
        run(["git", "checkout", "--quiet", "--", "."], cwd=사본)

    # 다 — 과거 커밋: 미스 = 두 트리 사이에 blob 이 다른 파일 수
    이전 = run(["git", "rev-parse", "--short", f"{at}~3"], cwd=사본).stdout.strip()
    if not 이전:
        어긋남("⑦다 — 3커밋 전을 못 찾았다")
    else:
        diff = run(["git", "diff", "--name-only", 이전, at], cwd=사본).stdout.split()
        cache2 = 새_캐시("scenario-past")
        대장(사본, at, cache2)
        적중, 빗나감, _ = 회계(사본, 이전, cache2)
        # **삭제된 파일은 옛 트리에만 있다** — 그쪽이 미스가 된다. 이름이 같아도 내용이
        # 바뀌었으면 미스다. 그래서 기대는 `git diff` 의 파일 수 이하이고 0 보다 크다.
        if 0 < 빗나감 <= len(diff):
            ok(f"⑦다 3커밋 전 — 빗나감 {빗나감} ≤ 바뀐 파일 {len(diff)} · 적중 {적중}")
        else:
            어긋남(f"⑦다 3커밋 전 — 빗나감 {빗나감} 이 바뀐 파일 {len(diff)} 과 안 맞는다")

    # 라 — ★ 파일 이동: 옮긴 파일이 **전부 미스**. ADR-0004 가 치른 값이다.
    cache3 = 새_캐시("scenario-move")
    run(["git", "checkout", "--quiet", "--", "."], cwd=사본)
    대장(사본, None, cache3)
    옮길 = sorted(사본.rglob("*.kt"))[:5]
    if len(옮길) < 5:
        어긋남("⑦라 — 옮길 `.kt` 가 다섯 미만이다")
    else:
        새_방 = 사본 / "옮긴자리"
        새_방.mkdir(exist_ok=True)
        # **`git mv` 다.** 손으로 옮기면 색인이 옛 경로를 그대로 들고 있어서 대장이
        # **옮긴 것을 보지 못한다** — 그러면 빗나감 0 이 나오고, 그 0 은 *"키에 경로가
        # 없다"* 가 아니라 *"변형이 아무것도 안 바꿨다"* 다. 이 대조가 꺼지는 형태의
        # 하나이고(F03 지붕 §3 의 아홉 중 마지막), 처음 쓸 때 실제로 걸렸다.
        for f in 옮길:
            run(["git", "mv", str(f.relative_to(사본)), f"옮긴자리/{f.name}"], cwd=사본)
        옮긴_수 = len(list(새_방.iterdir()))
        if 옮긴_수 != 5:
            어긋남(f"⑦라 ★ 변형이 안 먹었다 — 옮겨진 파일이 {옮긴_수} 개다")
            shutil.rmtree(작업, ignore_errors=True)
            적음()
            return
        적중, 빗나감, _ = 회계(사본, None, cache3)
        if 빗나감 == 5:
            ok(f"⑦라 ★ 파일 이동 5 — 빗나감 5 (내용은 같다) · 적중 {적중} — "
               f"**ADR-0004 가 치른 값이 여기 있다**")
        else:
            어긋남(f"⑦라 ★ 파일 이동 5 — 빗나감 {빗나감} (기대 5). "
                  f"0 이면 키에 경로가 안 들어간 것이고 F01 이 고친 버그의 재발이다")

    shutil.rmtree(작업, ignore_errors=True)
    적음()


# ═════════════════════════════════════════════════════════════════════════════
# ⑧ 크기 — 파일당 압축 바이트
# ═════════════════════════════════════════════════════════════════════════════

목표_바이트 = 2048


def 검사8() -> None:
    적음(f"⑧ 크기 — 파일당 평균 압축 바이트 (합격선 {목표_바이트} B)")
    for repo, at, name in (DITTO, PORTAL):
        cache = 새_캐시(f"size-{name}")
        대장(repo, at, cache)
        파일들 = 캐시_파일들(cache)
        if len(파일들) < 최소_파일[name]:
            어긋남(f"⑧ {name} — 엔트리가 {len(파일들)} 개다. 하한 미만")
            continue
        총 = sum(f.stat().st_size for f in 파일들)
        평균 = 총 // len(파일들)
        기록.append(f"{name}: 엔트리 {len(파일들)} · 총 {총:,} B · 평균 {평균:,} B")
        (ok if 평균 <= 목표_바이트 else 어긋남)(
            f"⑧ {name}  엔트리 {len(파일들)} · 총 {총 / 1024 / 1024:.1f} MiB · "
            f"평균 {평균:,} B"
        )
    적음()


# ═════════════════════════════════════════════════════════════════════════════
# ⑨ 벤치 — 합격선은 비율, 절대 시간은 기록
# ═════════════════════════════════════════════════════════════════════════════

비율_하한 = 10.0


def 잰다(fn) -> float:
    t = time.monotonic()
    fn()
    return time.monotonic() - t


def 검사9() -> None:
    적음(f"⑨ 벤치 — 합격선은 **비율**(증분이 콜드보다 {비율_하한:.0f}배 이상 빠르다)")
    적음(f"      기계: {platform.platform()} · CPU {os.cpu_count()}")
    for repo, at, name in (DITTO, PORTAL):
        cache = 새_캐시(f"bench-{name}")
        콜드 = 잰다(lambda: 대장(repo, at, cache))
        파일수 = len(캐시_파일들(cache))
        증분 = 잰다(lambda: 대장(repo, at, cache))
        적중, 빗나감, _ = 회계(repo, at, cache)
        비율 = 콜드 / 증분 if 증분 > 0 else float("inf")

        기록.append(
            f"{name}: 콜드 {콜드:.2f}s ({파일수} 파일) · 증분 {증분:.2f}s · 비율 {비율:.1f}배"
        )
        # ★ 증분이 실제로 캐시를 썼는가 — 안 읽고 빠르면 비율이 예뻐진다.
        if 빗나감 != 0:
            어긋남(f"⑨ {name} — 증분 회차의 빗나감이 {빗나감} 이다. 캐시를 안 썼다")
        elif 비율 < 비율_하한:
            어긋남(f"⑨ {name} — 비율 {비율:.1f}배 (하한 {비율_하한:.0f})")
        else:
            ok(f"⑨ {name}  콜드 {콜드:.2f}s · 증분 {증분:.2f}s · **{비율:.1f}배** · 적중 {적중}")
    적음()
    적음("   G50 이 남긴 비용 — 문법 축이 하나라 Kotlin 문법을 올리면 TypeScript 캐시도")
    적음("   전량 무효화된다. **그 비용이 위 `ditto` 콜드 시간**이다.")
    적음()


# ═════════════════════════════════════════════════════════════════════════════
# ②⑥ 은 `cargo test` 가 상시로 잰다 — **여기서는 그것이 도는지 확인한다**
# ═════════════════════════════════════════════════════════════════════════════


def 검사_ci() -> None:
    적음("②⑥ CI 상시 — `cargo test` 가 재는 둘이 실제로 도는가")
    p = run(["cargo", "test", "-p", "pal-cli", "--test", "prune_boundary",
             "--test", "rebuild_equivalence"], cwd=ROOT)
    if p.returncode != 0:
        어긋남("②⑥ 통합 시험이 실패했다\n" + p.stdout[-1500:])
    elif "2 passed" not in p.stdout and p.stdout.count("1 passed") < 2:
        어긋남("②⑥ 시험이 돌지 않았다 — 이름이 바뀌었는가")
    else:
        ok("②⑥ `prune_boundary` · `rebuild_equivalence` 둘 다 돈다")
    적음()


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--skip-mutation", action="store_true")
    a = ap.parse_args()

    if not BIN.exists():
        print(f"  {BIN} 이 없다 — `cargo build --workspace --release`", file=sys.stderr)
        return 1
    for repo, _, name in (DITTO, PORTAL):
        if not repo.exists():
            print(f"  코퍼스가 없다: {repo} — 대조 불가", file=sys.stderr)
            return 1

    적음("F04 — 추출 캐시 (1층)")
    적음()
    검사1()
    검사3(a.skip_mutation)
    검사4()
    검사5()
    검사7()
    검사8()
    검사9()
    검사_ci()

    if 기록:
        적음("── 기록 (합격선이 아니다) ──")
        for line in 기록:
            적음(f"   {line}")
        적음()

    if 실패:
        적음(f"어긋난 것 {len(실패)}:")
        for f in 실패:
            적음(f"   · {f}")
        return 1
    적음("일곱 다 통과 — ②⑥ 은 `cargo test` 가 상시로 잰다 · ⑩ 은 기권(모집단 0)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
