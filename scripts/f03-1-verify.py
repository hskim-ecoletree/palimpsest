#!/usr/bin/env python3
"""F03-1 대조 — 정체성: `symbol_id` 의 성분과 판별자.

합격선은 `corpus/criteria.toml` `[f03.1.pass]` 에 있고 **코드보다 먼저 등록됐다**
(커밋 `3621f6d`).

여섯을 잰다:

    ① 컨테이너 체인이 실제로 실린다 — 그리고 그 수가 `FileGraph.contains` 와 같다
    ② 불변식 E · F · G — **F 가 반대 방향이다 ★**
    ③ `SymbolIdentity` 3변형 + L0 결박 불가의 컴파일 강제 (`cargo test --doc`)
    ④ 정체성 규칙 5종 (§3.4)
    ⑤ 오버로드 재정렬 빈도 — **측정만이다**
    ⑥ 같은 커밋 두 번 추출 시 `symbol_id` 일치 100%

**대조가 꺼지는 형태 둘을 막는다** (`[f03].self_judged` 3):

  · **자라는 값에 묶지 않는다** — 변형 대상이 소스에 없으면 **멈춘다.**
    파일 수·심볼 수에 묶으면 코퍼스가 자랄 때 조용히 꺼진다
  · **공유 상태를 안 쓴다** — 회차마다 캐시 디렉터리를 새로 만든다.
    F02-4 에서 변이 셋이 캐시를 돌려 써 병렬 구간이 아예 안 돌았다

사용:
    ./scripts/f03-1-verify.py
    ./scripts/f03-1-verify.py --ditto ~/dev/projects/ditto

종료 코드:
    0  여섯 다 통과
    1  하나라도 어긋났다 · 또는 대조가 성립하지 않았다
"""

from __future__ import annotations

import argparse
import collections
import json
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
BIN = ROOT / "target/release/pal"

DITTO_PIN = "aded7ce7f88f"
PORTAL_PIN = "a29cad0bf6a8"


def run(args: list[str], **kw) -> subprocess.CompletedProcess:
    return subprocess.run(args, capture_output=True, text=True, check=False, **kw)


def symbols_of(repo: Path, at: str, cache: Path) -> list[dict]:
    """**캐시를 새로 만든다** — 회차끼리 상태를 물려주지 않는다."""
    p = run([str(BIN), "ledger", str(repo), "--at", at, "--cache-dir", str(cache), "--symbols"])
    if p.returncode != 0:
        raise SystemExit(f"대장을 내지 못했다: {p.stderr[-400:]}")
    return [json.loads(l) for l in p.stdout.splitlines() if l]


def graph_contains(worktree: Path) -> int:
    """그 트리의 `.ts` 전부에서 `FileGraph.contains` 항목 수의 합."""
    total = 0
    for f in sorted(worktree.rglob("*.ts")):
        if "node_modules" in f.parts:
            continue
        p = run([str(BIN), "symbols", str(f), "--graph"])
        if p.returncode != 0:
            continue
        try:
            g = json.loads(p.stdout)
        except json.JSONDecodeError:
            continue
        if isinstance(g, dict):
            total += len(g.get("contains", []))
    return total


def key(s: dict) -> tuple:
    return (s["path"], tuple(s["container"]), s["name"], s["kind"], s["span"]["byte_start"])


# ═════════════════════════════════════════════════════════════════════════════
# ① 컨테이너 체인이 실린다 — 그리고 세는 쪽과 좌표를 만드는 쪽이 같은 것을 본다
# ═════════════════════════════════════════════════════════════════════════════


def check_1(ditto: Path, tmp: Path) -> tuple[bool, str]:
    syms = symbols_of(ditto, DITTO_PIN, tmp / "c1")
    with_chain = [s for s in syms if s["container"]]
    if not with_chain:
        return False, "체인이 비지 않은 심볼이 0 이다 — 성분이 여전히 빈 배열이다"

    # `FileGraph.contains` 와 대 본다. **대장이 제외한 파일은 빼고 센다** — 그 차이가
    # 곧 "대장이 안 본 파일" 이고, 그것을 안 빼면 이 항등식이 거짓말을 한다.
    tree = tmp / "ditto-tree"
    tree.mkdir()
    tar = subprocess.run(["git", "-C", str(ditto), "archive", DITTO_PIN],
                         stdout=subprocess.PIPE, check=True)
    subprocess.run(["tar", "-x", "-C", str(tree)], input=tar.stdout, check=True)
    total = graph_contains(tree)

    seen_paths = {s["path"] for s in syms}
    dropped = 0
    for f in sorted(tree.rglob("*.ts")):
        rel = str(f.relative_to(tree))
        if "node_modules" in f.parts or rel in seen_paths:
            continue
        p = run([str(BIN), "symbols", str(f), "--graph"])
        if p.returncode == 0:
            try:
                g = json.loads(p.stdout)
                if isinstance(g, dict):
                    dropped += len(g.get("contains", []))
            except json.JSONDecodeError:
                pass

    depth = max(len(s["container"]) for s in syms)
    kinds = collections.Counter(s["kind"] for s in with_chain)
    ok = len(with_chain) == total - dropped
    note = (f"체인 있는 심볼 {len(with_chain)} · contains 합계 {total} − 대장 밖 {dropped} "
            f"= {total - dropped} · 최대 깊이 {depth} · {dict(kinds)}")
    return ok, note


# ═════════════════════════════════════════════════════════════════════════════
# ② 불변식 E · F · G — **F 가 반대 방향이다 ★**
#
# **실물 소스를 변형한다.** 합성 픽스처로 대신하지 않는다(`[f03].self_judged` 4).
# 그리고 **변형 대상이 없으면 멈춘다** — 자라는 값에 묶지 않는다.
# ═════════════════════════════════════════════════════════════════════════════


def check_2(ditto: Path, tmp: Path) -> tuple[bool, str]:
    syms = symbols_of(ditto, DITTO_PIN, tmp / "c2")
    by_path: dict[str, list[dict]] = collections.defaultdict(list)
    for s in syms:
        by_path[s["path"]].append(s)

    # **E** — 같은 파일 안 서로 다른 컨테이너의 같은 이름
    collisions = 0
    for path, ss in by_path.items():
        names = collections.defaultdict(set)
        for s in ss:
            if s["container"]:
                names[(s["name"], s["kind"])].add(tuple(s["container"]))
        for (_n, _k), containers in names.items():
            if len(containers) > 1:
                collisions += 1
    if collisions == 0:
        return False, ("불변식 E 의 변형 대상이 코퍼스에 없다 — 서로 다른 컨테이너의 "
                       "같은 이름이 0 건이다. **대조가 성립하지 않는다**")

    ids = collections.Counter(s["id"] for s in syms)
    dup = [i for i, n in ids.items() if n > 1]
    if dup:
        return False, f"불변식 E 위반 — 좌표가 겹치는 심볼 {len(dup)} 건"

    # **F ★** — 컨테이너 순서를 바꾼다. 실물 파일 하나를 고른다.
    # **가장 두꺼운 파일을 고른다.** 아무거나 고르면 메서드 둘짜리 파일이 걸리고,
    # 그러면 F 는 통과하되 두 줄만 시험한 것이 된다.
    target = None
    best = 0
    for path, ss in sorted(by_path.items()):
        tops = [s for s in ss if not s["container"] and s["kind"] == "class"]
        inner = sum(1 for s in ss if s["container"])
        if len(tops) >= 2 and inner > best:
            best, target = inner, (path, tops)
    if target is None:
        return False, ("불변식 F 의 변형 대조가 성립하지 않는다 — 최상위 클래스가 둘 이상인 "
                       "`.ts` 파일이 코퍼스에 없다. **시험되지 않은 대조는 실패다**")

    path, tops = target
    work = tmp / "f-work"
    work.mkdir()
    # **바이트로 다룬다.** `span` 은 바이트 자리이고 이 코퍼스에는 한글 주석이 있다 —
    # 문자열 자리로 자르면 조각이 어긋나 파일이 깨지고, **깨진 파일에서 심볼이 사라지면
    # 대조는 「어긋났다」가 아니라 「잴 것이 없다」가 된다.** 첫 실행이 그렇게 걸렸다.
    src = subprocess.run(["git", "-C", str(ditto), "show", f"{DITTO_PIN}:{path}"],
                         stdout=subprocess.PIPE, check=True).stdout
    a, b = tops[0], tops[1]
    swapped = (src[: a["span"]["byte_start"]]
               + src[b["span"]["byte_start"]: b["span"]["byte_end"]]
               + src[a["span"]["byte_end"]: b["span"]["byte_start"]]
               + src[a["span"]["byte_start"]: a["span"]["byte_end"]]
               + src[b["span"]["byte_end"]:])
    if swapped == src:
        raise SystemExit("변형이 소스를 안 바꿨다 — 대조가 꺼졌다")

    # **경로가 같아야 한다.** `symbol_id` 의 성분에 경로가 있으므로 파일 이름을 달리
    # 두면 두 산출이 무조건 달라지고, 그러면 이 대조는 무엇을 재든 「움직였다」를 낸다.
    # 첫 실행이 그렇게 어긋났다 — **검사 자신이 반대 방향을 안 지킨 자리다.**
    before_f = symbols_of_file(tmp, "a.ts", src)
    after_f = symbols_of_file(tmp, "a.ts", swapped)
    # 컨테이너 안의 메서드만 본다 — 클래스 자신은 순서와 무관하게 이름이 열쇠다.
    def methods(v: list[dict]) -> dict:
        return {(tuple(s["container"]), s["name"], s["kind"]): s["id"]
                for s in v if s["container"]}
    m_before, m_after = methods(before_f), methods(after_f)
    common = set(m_before) & set(m_after)
    if len(common) < 2:
        return False, f"불변식 F — 대조할 메서드가 {len(common)} 개다. 시험되지 않았다"
    # **변형이 실제로 산출을 움직였는가.** 자리가 하나도 안 움직였으면 맞바꾸기가
    # 헛돌았다는 뜻이고, 그러면 F 는 아무것도 안 재고 통과한다.
    spans_before = sorted((s["name"], s["span"]["byte_start"]) for s in before_f)
    spans_after = sorted((s["name"], s["span"]["byte_start"]) for s in after_f)
    if spans_before == spans_after:
        raise SystemExit("맞바꾸기가 자리를 하나도 안 움직였다 — 대조가 꺼졌다")
    shifted = [k for k in common if m_before[k] != m_after[k]]
    if shifted:
        return False, f"불변식 F 위반 — 클래스 순서를 바꿨더니 좌표가 움직였다: {shifted[:5]}"

    # **G** — 옮기면 정체성만 바뀐다
    # G 는 **일부러** 경로를 바꾼다 — 그것이 이 불변식이 재는 사건이다.
    moved = symbols_of_file(tmp, "sub/a.ts", src)
    by_key_before = {(s["name"], s["kind"], s["span"]["byte_start"]): s for s in before_f}
    g_bad = []
    for s in moved:
        k = (s["name"], s["kind"], s["span"]["byte_start"])
        o = by_key_before.get(k)
        if o is None:
            continue
        if o["id"] == s["id"] or o["body"] != s["body"]:
            g_bad.append(k)
    if g_bad:
        return False, f"불변식 G 위반 — 옮겼는데 정체성이 그대로거나 본문이 움직였다: {g_bad[:5]}"

    return True, (f"E 충돌 대상 {collisions} 건 · F 는 `{path}` 의 클래스 둘을 맞바꿔 "
                  f"메서드 {len(common)} 개 · G 는 같은 파일을 옮겨 {len(moved)} 개")


# 임시 저장소를 만든 횟수 — **이름이 아니라 부모 디렉터리만 바뀐다.**
_MADE = [0]


def symbols_of_file(tmp: Path, rel: str, source: bytes) -> list[dict]:
    """소스 하나를 **새 git 저장소**에 넣고 좌표를 낸다.

    저장소를 매번 새로 만든다 — 회차끼리 상태를 물려주면 그것이 F02-4 에서 대조를
    끈 형태다.

    # ⚠ 저장소의 **이름**은 고정한다

    매니페스트가 없으면 `RepoId` 가 디렉터리 이름에서 온다(`ledger::repo_name`).
    임시 디렉터리 이름을 그대로 쓰면 **회차마다 `repo_id` 가 달라 좌표가 전부
    달라지고**, 그러면 이 대조는 무엇을 재든 언제나 「움직였다」를 낸다.
    첫 실행에서 실제로 그렇게 어긋났다.
    """
    _MADE[0] += 1
    repo = tmp / f"rep{_MADE[0]}" / "corpus"
    repo.mkdir(parents=True)
    f = repo / rel
    f.parent.mkdir(parents=True, exist_ok=True)
    f.write_bytes(source)
    for args in (["init", "-q"], ["add", "-A"], ["-c", "user.email=t@t", "-c", "user.name=t",
                                                 "commit", "-qm", "t"]):
        p = run(["git", "-C", str(repo), *args])
        if p.returncode != 0:
            raise SystemExit(f"임시 저장소를 못 만들었다: {p.stderr[-300:]}")
    return symbols_of(repo, "HEAD", repo / ".cache")


# ═════════════════════════════════════════════════════════════════════════════
# ③ 타입 강제 — `compile_fail` 문서 시험이 실제로 돈다
# ═════════════════════════════════════════════════════════════════════════════


def check_3() -> tuple[bool, str]:
    p = run(["cargo", "test", "-p", "pal-core", "--doc"], cwd=ROOT)
    out = p.stdout + p.stderr
    if p.returncode != 0:
        return False, f"문서 시험이 실패했다: {out[-300:]}"
    n = out.count("compile fail")
    if n < 2:
        return False, (f"`compile_fail` 시험이 {n} 개다 — 둘이어야 한다 "
                       "(L0 결박 · 리터럴 생성). **시험되지 않은 대조는 실패다**")
    return True, f"`compile_fail` 문서 시험 {n} 개가 돌았다"


# ═════════════════════════════════════════════════════════════════════════════
# ④ 정체성 규칙 5종 — 단위 시험이 각각 서 있는가
# ═════════════════════════════════════════════════════════════════════════════

RULE_TESTS = {
    "오버로드": "같은_컨테이너의_오버로드는_여전히_순서로_갈린다",
    "익명 귀속": "익명은_가장_가까운_조상의_요약에_포함된다",
    "제네릭": "제네릭은_선언_하나가_심볼_하나고_인스턴스화는_심볼이_아니다",
    "생성 심볼": "생성물은_증거가_둘일_때만이다",
    "재선언": "재선언은_한_좌표로_뭉개지지_않고_후보로_남는다",
}


def check_4() -> tuple[bool, str]:
    p = run(["cargo", "test", "--workspace", "--", "--list"], cwd=ROOT)
    listed = p.stdout
    missing = [name for name, t in RULE_TESTS.items() if t not in listed]
    if missing:
        return False, f"규칙 {missing} 의 시험이 없다"
    return True, f"다섯 규칙의 시험이 각각 선다 — {' · '.join(RULE_TESTS)}"


# ═════════════════════════════════════════════════════════════════════════════
# ⑤ 오버로드 재정렬 빈도 — **측정만이다**
#
# 세는 단위: 한 커밋에서 같은 컨테이너 안 같은 (이름, 종류) 심볼이 **둘 이상**이고
# 그 선언 순서가 부모 커밋과 다른 경우.
# ═════════════════════════════════════════════════════════════════════════════


def overload_order(syms: list[dict]) -> dict[tuple, list[str]]:
    """`(경로, 체인, 이름, 종류)` → **선언 순서의 `body_digest` 열.** 둘 이상인 것만.

    # 왜 자리(byte)가 아니라 요약인가

    같은 자리의 같은 이름·같은 종류인 심볼들은 **순서 말고 서로를 가를 것이 없다** —
    그것이 R-16 이 경고한 상태 그 자체다. 그래서 「재정렬」을 재려면 각 항목을
    순서와 **무관하게** 알아보는 값이 있어야 하고, 그것이 `body_digest` 다.

    자리(byte offset)로 재면 **줄 하나만 고쳐도 전부 움직인 것으로 세어진다** —
    첫 실행이 그렇게 8 건을 냈고 그중 재정렬은 없었다.
    """
    groups: dict[tuple, list[str]] = collections.defaultdict(list)
    for s in syms:
        groups[(s["path"], tuple(s["container"]), s["name"], s["kind"])].append(s["body"])
    return {k: v for k, v in groups.items() if len(v) > 1}


def check_5(repos: list[tuple[Path, str]], tmp: Path, limit: int) -> tuple[bool, str]:
    total_commits = 0
    reordered = 0
    examples: list[str] = []
    overload_sites = 0
    for repo, pin in repos:
        revs = run(["git", "-C", str(repo), "log", "--format=%H", f"-n{limit}", pin]).stdout.split()
        prev = None
        for i, rev in enumerate(revs):
            cache = tmp / f"ov-{repo.name}-{i}"
            try:
                syms = symbols_of(repo, rev, cache)
            except SystemExit:
                continue
            groups = overload_order(syms)
            overload_sites += len(groups)
            if prev is not None:
                total_commits += 1
                # 같은 자리에서 **순서만** 바뀐 것을 센다 — 자리가 늘거나 줄면
                # 그것은 재정렬이 아니라 선언의 추가·삭제다.
                for k, now in groups.items():
                    was = prev.get(k)
                    if not was:
                        continue
                    # **같은 것들이 순서만 바뀌었는가.** 집합이 같은데 열이 다르면
                    # 재정렬이고, 집합이 다르면 그것은 선언의 추가·삭제·수정이다.
                    if sorted(was) == sorted(now) and was != now:
                        reordered += 1
                        if len(examples) < 5:
                            examples.append(f"{repo.name}@{rev[:8]} {k[0]}::{k[2]}")
            prev = groups
    note = (f"오버로드 자리 연 {overload_sites} 건 · 대조한 커밋 쌍 {total_commits} · "
            f"재정렬 {reordered} 건")
    if examples:
        note += " — " + " · ".join(examples)
    return True, note  # **측정만이다. 수치가 합격선이 아니다**


# ═════════════════════════════════════════════════════════════════════════════
# ⑥ 결정성 — 같은 커밋 두 번
# ═════════════════════════════════════════════════════════════════════════════


def check_6(repos: list[tuple[Path, str]], tmp: Path) -> tuple[bool, str]:
    lines = []
    for repo, pin in repos:
        # **캐시를 따로 준다** — 같은 캐시로 두 번 돌면 둘째는 캐시를 재는 것이지
        # 추출을 재는 것이 아니다(F02-4 가 그 자리에서 대조를 잃었다).
        a = symbols_of(repo, pin, tmp / f"det-{repo.name}-a")
        b = symbols_of(repo, pin, tmp / f"det-{repo.name}-b")
        if [s["id"] for s in a] != [s["id"] for s in b]:
            return False, f"{repo.name} — 두 회차의 `symbol_id` 가 다르다"
        lines.append(f"{repo.name} {len(a)}")
    return True, "회차 둘이 같다 — " + " · ".join(lines)


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--ditto", type=Path, default=Path.home() / "dev/projects/ditto")
    ap.add_argument("--portal", type=Path,
                    default=Path.home() / "dev/projects/boxwood/portal-backend")
    ap.add_argument("--history", type=int, default=120,
                    help="⑤ 가 볼 커밋 수. 저장소마다")
    a = ap.parse_args()

    if not BIN.exists():
        print(f"  {BIN} 이 없다 — `cargo build --workspace --release`", file=sys.stderr)
        return 1
    for r in (a.ditto, a.portal):
        if not (r / ".git").exists():
            print(f"  코퍼스가 없다: {r}", file=sys.stderr)
            return 1

    repos = [(a.ditto, DITTO_PIN), (a.portal, PORTAL_PIN)]
    results: list[tuple[str, bool, str]] = []

    with tempfile.TemporaryDirectory() as td:
        tmp = Path(td)
        print("F03-1 — 정체성: `symbol_id` 의 성분과 판별자")
        print()
        for name, fn in (
            ("① 컨테이너 체인이 실린다", lambda: check_1(a.ditto, tmp)),
            ("② 불변식 E·F·G  ★F", lambda: check_2(a.ditto, tmp)),
            ("③ L0 결박 불가의 타입 강제", check_3),
            ("④ 정체성 규칙 5종", check_4),
            ("⑤ 오버로드 재정렬 빈도(측정만)", lambda: check_5(repos, tmp, a.history)),
            ("⑥ 결정성 — 같은 커밋 두 번", lambda: check_6(repos, tmp)),
        ):
            ok, note = fn()
            results.append((name, ok, note))
            print(f"  {'ok  ' if ok else 'FAIL'}  {name}")
            print(f"        {note}")
            print()

    bad = [n for n, ok, _ in results if not ok]
    if bad:
        print(f"어긋난 것 {len(bad)}: {' · '.join(bad)}")
        return 1
    print("여섯 다 통과")
    return 0


if __name__ == "__main__":
    sys.exit(main())
