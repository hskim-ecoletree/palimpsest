#!/usr/bin/env python3
"""E1 — 세 OS CI 잡 로그에서 시험 파일마다 돈 수가 그 OS 에서 컴파일되는 `#[test]` 수와 같은가.

회차 `2026-09-15-clean-uninstall` 완수 조건 E1 의 셈 스크립트.

    python3 e1-count.py --run <CI 런 id> [--pattern 'clean_uninstall_*.rs']
    python3 e1-count.py --logs <디렉터리> [--pattern …]   # <잡 이름>.log (ubuntu-latest.log …) 를 읽는다

rc: 0 전부 같다 · 1 어긋났다 · 2 셀 수 없었다(짐작하지 않고 멈춘다).

# 로그에서 「어느 결과가 어느 파일의 것인가」를 되살리는 규칙

1. **시험 걸음만 자른다** — `cargo xtask test` 가 처음 찍는 `■ 시험` 줄부터 `시험 통과` 또는
   `시험 결과가 등록과 다르다` 줄까지. 앞 걸음(`cargo xtask check` · `cargo run`)도 `Running` 을 찍으므로
   로그 전체에서 세면 순번이 밀린다(이 스크립트의 첫 판이 그렇게 틀렸다).
2. `xtask` 의 `돌린다` 는 cargo 를 `.output()` 으로 받아 **stdout 을 다 찍은 뒤 stderr 를 찍는다.** 그래서
   `test result:`(stdout) 묶음과 `Running …`(stderr) 묶음이 떨어져 있지만 **각 묶음 안의 순서는 cargo 의
   실행 순서 그대로**다. 첫 축은 `--all-targets` 이고(`축과_댄다` 가 지킨다) doctest 축은 `Running` 이 아니라
   `Doc-tests` 를 찍는다. 따라서 구간 안의 **N 번째 시험 바이너리 `Running` 줄 ↔ N 번째 `test result:`** 가 짝이다.

# 소스에서 OS 별 기대 수를 세는 규칙

- `#[test]` 가 붙은 함수를 헤아린다. 그 함수 위 속성 묶음 · 감싼 인라인 `mod {}` · 그 파일을 끌어온 `mod x;` 선언에 붙은
  `#[cfg(...)]` 를 OS 셋(linux · macos · windows)에 누적해 댄다.
- **`mod x;` 를 따라간다** — `#[path = "…"]` 가 있으면 선언한 파일의 디렉터리 기준, 없으면 `x.rs` · `x/mod.rs`.
  통합 시험 바이너리는 `mod common;` 으로 `tests/common/mod.rs` 를, 그것은 `#[path]` 로 제품 소스(`install/sha256.rs` 등)를
  끌어오고, 그 안의 `#[cfg(test)] mod tests` 도 그 바이너리에서 돈다(첫 판이 이것을 못 세어 9 개씩 모자랐다).
- 아는 cfg 식: 원자 `test` · `unix` · `windows` · `target_os = "linux|macos|windows"` 와 그것들을 묶는 `not(…)` · `any(…)` · `all(…)`
  (기존 시험에 `not(any(unix, windows))` 가 있다). **모르는 원자나 모양을 만나면 멈춘다.** `#[ignore]` 가 붙은 시험은 E1 이 0 을 요구하므로 어긋남으로 적는다.
"""
import argparse
import fnmatch
import json
import os
import re
import subprocess
import sys

OS_JOBS = {"ubuntu-latest": "linux", "macos-latest": "macos", "windows-latest": "windows"}
ALL = frozenset({"linux", "macos", "windows"})
REPO = "hskim-ecoletree/palimpsest"
ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "..", ".."))
TESTS = os.path.join(ROOT, "crates", "pal-cli", "tests")


class 셀수없다(Exception):
    pass


_원자 = {
    "test": ALL,
    "unix": frozenset({"linux", "macos"}),
    "windows": frozenset({"windows"}),
}


def _식(e, 원문):
    """cfg 식 하나를 OS 집합으로 — 원자와 `not` · `any` · `all` 만 안다."""
    t = re.fullmatch(r'target_os="(linux|macos|windows)"', e)
    if t:
        return frozenset({t.group(1)})
    if e in _원자:
        return _원자[e]
    m = re.fullmatch(r"(not|any|all)\((.*)\)", e)
    if not m:
        raise 셀수없다(f"모르는 cfg 모양이다: {원문}")
    인자 = []
    깊이, 앞 = 0, 0
    for i, c in enumerate(m.group(2)):
        if c == "(":
            깊이 += 1
        elif c == ")":
            깊이 -= 1
        elif c == "," and 깊이 == 0:
            인자.append(m.group(2)[앞:i])
            앞 = i + 1
    인자.append(m.group(2)[앞:])
    인자 = [a for a in 인자 if a]
    집합들 = [_식(a, 원문) for a in 인자]
    if m.group(1) == "not":
        if len(집합들) != 1:
            raise 셀수없다(f"not 의 인자가 하나가 아니다: {원문}")
        return ALL - 집합들[0]
    if m.group(1) == "any":
        return frozenset().union(*집합들)
    return frozenset(ALL.intersection(*집합들))


def cfg_집합(속성):
    s = 속성.strip()
    m = re.fullmatch(r"#\[cfg\((.*)\)\]", s)
    if not m:
        return ALL
    return _식(m.group(1).replace(" ", ""), s)


def _코드만(line):
    """문자열·문자 리터럴을 비우고 `//` 주석을 뗀다 — 중괄호와 선언만 보려고."""
    s = re.sub(r'"(?:\\.|[^"\\])*"', '""', line)
    s = re.sub(r"'(?:\\.|[^'\\])'", "''", s)
    i = s.find("//")
    return s if i < 0 else s[:i]


def 기대(path, 위_cfg=ALL, 방문=None):
    """파일 하나(와 그것이 끌어온 모듈)의 OS 별 `#[test]` 수와 `#[ignore]` 이름."""
    방문 = set() if 방문 is None else 방문
    real = os.path.realpath(path)
    if real in 방문:
        return {o: 0 for o in ALL}, []
    방문.add(real)
    수 = {o: 0 for o in ALL}
    무시 = []
    속성 = []
    mod_스택 = []  # (여는 깊이, 집합)
    깊이 = 0
    here = os.path.dirname(path)
    for raw in open(path, encoding="utf-8"):
        s = raw.strip()
        if s.startswith("#["):
            속성.append(s)
            continue
        if s.startswith("//") or not s:
            continue
        code = _코드만(raw).strip()
        현재 = 위_cfg
        for _, m in mod_스택:
            현재 = 현재 & m
        선언_cfg = 현재
        for a in 속성:
            if a.startswith("#[cfg("):
                선언_cfg = 선언_cfg & cfg_집합(a)
        md = re.match(r"(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*;", code)
        mi = re.match(r"(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*\{", code)
        mf = re.match(r"(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?fn\s+(\w+)", code)
        if md:
            p = next((re.fullmatch(r'#\[path\s*=\s*"([^"]+)"\]', a).group(1) for a in 속성
                      if re.fullmatch(r'#\[path\s*=\s*"([^"]+)"\]', a)), None)
            name = md.group(1)
            cands = ([os.path.join(here, p)] if p else
                     [os.path.join(here, name + ".rs"), os.path.join(here, name, "mod.rs")])
            target = next((c for c in cands if os.path.isfile(c)), None)
            if target is None:
                raise 셀수없다(f"`mod {name};` 의 파일을 못 찾았다: {cands}")
            if 선언_cfg:
                sub, subi = 기대(target, 선언_cfg, 방문)
                for o in ALL:
                    수[o] += sub[o]
                무시 += subi
        elif mi:
            mod_스택.append((깊이, 선언_cfg))
        elif mf and "#[test]" in 속성:
            for o in 선언_cfg:
                수[o] += 1
            if any(a.startswith("#[ignore") for a in 속성):
                무시.append(mf.group(1))
        속성 = []
        깊이 += code.count("{") - code.count("}")
        while mod_스택 and 깊이 <= mod_스택[-1][0]:
            mod_스택.pop()
    return 수, 무시


_달림 = re.compile(r"Running\S*\s+(unittests\s+\S+|tests[\\/](\S+?\.rs))\s")
_결과 = re.compile(r"test result: \w+\. (\d+) passed; (\d+) failed; (\d+) ignored")


def 로그_짝(text):
    """시험 걸음 구간에서 (시험 파일 이름 → (passed, failed, ignored))."""
    시작 = text.find("■ 시험")
    if 시작 < 0:
        raise 셀수없다("로그에 `■ 시험` 걸음이 없다")
    끝 = min([i for i in (text.find("시험 통과", 시작), text.find("시험 결과가 등록과 다르다", 시작)) if i >= 0] or [len(text)])
    구간 = text[시작:끝]
    달린 = list(_달림.finditer(구간))
    결과 = [tuple(int(x) for x in m.groups()) for m in _결과.finditer(구간)]
    if not 달린:
        raise 셀수없다("시험 걸음 안에 `Running` 줄이 없다")
    if len(결과) < len(달린):
        raise 셀수없다(f"`Running` {len(달린)} 줄인데 `test result:` 가 {len(결과)} 줄뿐이다")
    out = {}
    for i, m in enumerate(달린):
        if m.group(2):
            if m.group(2) in out:
                raise 셀수없다(f"같은 시험 파일이 두 번 돌았다: {m.group(2)}")
            out[m.group(2)] = 결과[i]
    return out


def 잡_로그(run):
    jobs = json.loads(subprocess.check_output(
        ["gh", "run", "view", str(run), "-R", REPO, "--json", "jobs"], text=True))["jobs"]
    out = {}
    for j in jobs:
        if j["name"] in OS_JOBS:
            out[j["name"]] = subprocess.check_output(
                ["gh", "run", "view", "-R", REPO, "--job", str(j["databaseId"]), "--log"], text=True, errors="replace")
    missing = set(OS_JOBS) - set(out)
    if missing:
        raise 셀수없다(f"런에 시험 잡이 없다: {sorted(missing)}")
    return out


def main():
    ap = argparse.ArgumentParser()
    g = ap.add_mutually_exclusive_group(required=True)
    g.add_argument("--run")
    g.add_argument("--logs")
    ap.add_argument("--pattern", default="clean_uninstall_*.rs")
    a = ap.parse_args()
    try:
        files = sorted(f for f in os.listdir(TESTS) if fnmatch.fnmatch(f, a.pattern))
        if not files:
            raise 셀수없다(f"`{a.pattern}` 에 드는 시험 파일이 0 개다 — 0 을 훑고 통과하지 않는다")
        logs = (잡_로그(a.run) if a.run else
                {job: open(os.path.join(a.logs, job + ".log"), encoding="utf-8", errors="replace").read()
                 for job in OS_JOBS})
        짝 = {job: 로그_짝(t) for job, t in logs.items()}
        어긋남 = 0
        print(f"{'파일':<42} {'OS':<15} {'기대':>4} {'passed':>6} {'failed':>6} {'ignored':>7}  판정")
        for f in files:
            수, 무시 = 기대(os.path.join(TESTS, f))
            for n in 무시:
                print(f"{f:<42} {'-':<15} {'':>4} {'':>6} {'':>6} {'':>7}  ✗ #[ignore] {n}")
                어긋남 += 1
            for job, os_ in OS_JOBS.items():
                got = 짝[job].get(f)
                if got is None:
                    print(f"{f:<42} {job:<15} {수[os_]:>4} {'-':>6} {'-':>6} {'-':>7}  ✗ 로그에 없다")
                    어긋남 += 1
                    continue
                ok = got[0] == 수[os_] and got[1] == 0 and got[2] == 0
                어긋남 += 0 if ok else 1
                print(f"{f:<42} {job:<15} {수[os_]:>4} {got[0]:>6} {got[1]:>6} {got[2]:>7}  {'✓' if ok else '✗'}")
        print(f"\n파일 {len(files)} · OS {len(OS_JOBS)} · 어긋남 {어긋남}")
        return 0 if 어긋남 == 0 else 1
    except 셀수없다 as e:
        print(f"셀 수 없었다: {e}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    sys.exit(main())
