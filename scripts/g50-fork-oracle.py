#!/usr/bin/env python3
"""G50 포크 오라클 — **어느 Kotlin 문법인가를 무엇으로 가르는가.**

합격선은 `corpus/criteria.toml` `[g50.oracle]` 에 있고 **측정보다 먼저 등록됐다**
(커밋 `af157e7` · `registered_before_any_measurement = true`).

    축 A  두 후보의 산출 차분 — 파싱 성패 · 선언 수 · **트리 S-표현식**
    축 B  트리가 옳은가 — 문법 밖의 독립 계수기로 후보를 뜨고 손으로 읽는다
    축 C  동률일 때만 — rev 핀 가능성 → 라이선스 → 상류 추종 → 링크

# 팔이 셋인 이유 — **현행 핀이 음성 대조다** ★

`ng @ 3dea6df` 를 같이 돌린다. **깨짐 56 · 강등 27 중 성한 것 0** 을 재현해야 하고,
안 하면 두 후보의 값도 못 믿는다. F02-2 가 정확히 그 형태로 한 번 틀렸다 — 여섯
문법을 한 바이너리에 링크하니 링커가 하나만 골라 **여섯 행이 글자까지 같았다.**

# ⚠ 별도 클론으로는 부족하다 — **CLI 가 문법 이름으로 캐시를 공유한다**

**이 스크립트를 처음 돌렸을 때 그 사고가 다시 났다.** 클론 셋을 따로 두고 각자의
디렉터리에서 `tree-sitter build` 를 했는데도 **세 팔이 같은 파서를 실었다** —
CLI 가 컴파일 결과를 `~/.cache/tree-sitter/lib/<문법이름>.dylib` 에 넣고, 세 문법의
이름이 전부 `kotlin` 이기 때문이다. **F02-2 의 링커 사고와 같은 병의 두 번째 형태이고,
「별도 바이너리」라는 그때의 처방이 여기서는 듣지 않는다.**

드러난 방식도 그때와 같다 — `ng` 가 자기 값을 못 냈다(선언 총수 0). 그래서 팔마다
**`HOME` 과 `XDG_CACHE_HOME` 을 따로 준다.** 캐시가 갈리는 것을 산출로도 확인한다.

# 대조가 꺼지는 형태 — `[g50].self_judged` 4 가 박아 둔 셋

    · 세 팔의 산출이 **전부 같으면 실패**  (장치가 꺼진 것이다)
    · 하한 — 파일 1,122 · 팔 3 · 독립 계수기 선언 2,000
    · 팔마다 **선언 총수가 0 보다 큰지 먼저** 확인 (빈 산출은 어긋남을 못 낸다)

사용:
    ./scripts/g50-fork-oracle.py --arms <클론들의_부모> --corpus /tmp/s0-corpus
전제:
    <부모>/ng · <부모>/sg · <부모>/brokk 가 클론돼 있다. tree-sitter CLI 0.26.12.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
QUERY = ROOT / "crates/pal-extract/queries/kotlin/top-level.scm"
VECTOR = ROOT / "corpus/tasks/s0-reference-vector.tsv"

N_FILES = 1122
MIN_INDEPENDENT_DECLS = 2000  # [g50.oracle].axis_b 의 하한
CONTROL = "ng"
CANDIDATES = ("sg", "brokk")

# 축 C ③(상류 추종)이 대는 정본. **두 후보 다 이것의 포크다**(GitHub `parent`).
UPSTREAM = "https://github.com/fwcd/tree-sitter-kotlin"

# ── 팔마다 쿼리가 다르다. **그것 자체가 이 측정의 첫 발견이다** ────────────────
#
# 두 후보는 fwcd 계열이라 이름 있는 마디가 `simple_identifier`·`type_identifier` 이고
# **`name:` 필드가 아예 없다.** `ng` 는 amaanq 의 다시 쓰기라 `identifier` + `name:` 다.
# 그래서 공유 쿼리를 그대로는 못 쓴다 — 컴파일이 안 된다(`Invalid field name "name"`).
#
# `[g50.pass]` ③ 이 미리 정한 규칙: **패턴 다섯 유지 · 이름 치환만 · 술어 금지 ·
# `source_file` 직계 자식이라는 단위 유지.** 아래가 그 규칙 안에서 옮긴 것이고,
# **필드 제약이 사라진 만큼 느슨해졌으므로** 반대 방향을 같이 잰다(`FORK_STRUCT_ONLY`).
FORK_QUERY = """\
(source_file (class_declaration    (type_identifier) @name) @decl)
(source_file (function_declaration (simple_identifier) @name) @decl)
(source_file (object_declaration   (type_identifier) @name) @decl)
(source_file (type_alias           (type_identifier) @name) @decl)
(source_file (property_declaration (variable_declaration (simple_identifier) @name)) @decl)
"""

# ★ 반대 방향 — **이름을 아예 안 보는 쿼리.** 매치 수가 위와 같아야 한다.
# 다르면 이름 마디가 매치를 **늘렸거나 줄였다**는 뜻이고, 그러면 늘어난 선언에
# 우리 손이 섞인 것이다. 팔마다 어휘가 다르므로 이것도 팔마다 다르다.
STRUCT_ONLY = {
    "fork": """\
(source_file (class_declaration) @decl)
(source_file (function_declaration) @decl)
(source_file (object_declaration) @decl)
(source_file (type_alias) @decl)
(source_file (property_declaration) @decl)
""",
    "ng": """\
(source_file (class_declaration) @decl)
(source_file (function_declaration) @decl)
(source_file (object_declaration) @decl)
(source_file (type_alias) @decl)
(source_file (property_declaration) @decl)
""",
}

# **현행 핀이 내야 하는 값** — [g50.oracle].why_a_control_arm ★
CONTROL_EXPECT_FAILS = 56
CONTROL_EXPECT_RECOVERED = 0

# F02-2 게이트 §1 의 **강등 27**. `…/` 를 편 것이고 코퍼스 접두어가 붙는다.
_P = "portal-backend/src/main/kotlin/kr/co/ecoletree/boxwood/"
DEGRADED_27 = [_P + s for s in (
    "auth/pat/repository/ExposedPatTokenRepository.kt",
    "auth/repository/impl/UserTokenRepositoryImpl.kt",
    "auth/systempat/repository/SystemPatTokenRepository.kt",
    "automation/connector/repository/impl/ConnectorDslRepository.kt",
    "automation/email/templates/repository/impl/EmailTemplateDslRepository.kt",
    "automation/llm/repository/impl/LlmPromptTemplateDslRepository.kt",
    "automation/llm/repository/impl/LlmTaskMetaDslRepository.kt",
    "automation/llm/repository/impl/LlmTaskVersionDslRepository.kt",
    "automation/llm/repository/impl/McpToolConfigDslRepository.kt",
    "automation/process/repository/impl/BpmnDslRepository.kt",
    "automation/process/repository/impl/ProcessDslRepository.kt",
    "automation/process/repository/impl/ProcessGlobalVariableDslRepository.kt",
    "automation/process/repository/impl/ProcessTriggerDslRepository.kt",
    "automation/process/repository/impl/ProcessTriggerEventVariableDslRepository.kt",
    "automation/servicetask/repository/impl/ServiceTaskDslRepository.kt",
    "group/base/repository/impl/MembershipTypeDslRepository.kt",
    "organization/repository/impl/OrganizationGroupDslRepository.kt",
    "organization/repository/impl/OrganizationMembershipDslRepository.kt",
    "permission/repository/impl/ResourcePermissionMappingDslRepository.kt",
    "permission/repository/impl/RolePermissionMappingDslRepository.kt",
    "role/repository/impl/RoleGroupDslRepository.kt",
    "role/repository/impl/RoleGroupMembershipDslRepository.kt",
    "user/repository/impl/TenantUserDslRepository.kt",
    "auth/repository/impl/TokenBlacklistRepositoryImpl.kt",
    "auth/repository/impl/RefreshTokenFamilyDslRepository.kt",
    "automation/llm/repository/impl/McpToolDslRepository.kt",
    "permission/annotation/RequiresIntegratedPermission.kt",
)]


def home_of(arm: Path) -> Path:
    """팔마다 다른 `HOME`. **이것이 파서 캐시를 가르는 유일한 장치다.**"""
    h = arm.parent / f".home-{arm.name}"
    h.mkdir(parents=True, exist_ok=True)
    return h


def env_of(arm: Path) -> dict:
    h = home_of(arm)
    return {**os.environ, "HOME": str(h), "XDG_CACHE_HOME": str(h / ".cache")}


def run(args: list[str], cwd: Path) -> str:
    p = subprocess.run(args, cwd=cwd, capture_output=True, text=True, check=False,
                       env=env_of(cwd) if args and args[0] == "tree-sitter" else None)
    return p.stdout


def silent_misparse_17() -> list[str]:
    """T7 의 **조용한 오파싱 17** — 지금 커밋된 레퍼런스 벡터에서 뜬다.

    새로 재지 않는다. **이미 등록된 산출**이라 우리가 고를 여지가 없다.
    """
    rows = []
    for line in VECTOR.read_text(encoding="utf-8").splitlines():
        if line.startswith("#") or line.startswith("path\t"):
            continue
        path, parse, n = line.split("\t")
        if parse == "ok" and n == "0":
            rows.append(path)
    return rows


def query_counts(arm: Path, query: Path, files: list[Path]) -> dict[str, int]:
    counts: dict[str, int] = {}
    for i in range(0, len(files), 160):
        batch = files[i : i + 160]
        out = run(["tree-sitter", "query", str(query), *[str(f) for f in batch]], arm)
        cur = None
        for line in out.splitlines():
            if not line.startswith((" ", "\t")) and line.strip().endswith(".kt"):
                cur = line.strip()
            elif cur is not None and line.lstrip().startswith("pattern:"):
                counts[cur] = counts.get(cur, 0) + 1
    return counts


def measure(arm: Path, files: list[Path], corpus: Path, query: Path, struct: Path) -> dict:
    """한 팔. **다른 팔과 디렉터리를 공유하지 않는다.**"""
    listing = arm / ".g50-paths.txt"
    listing.write_text("\n".join(str(f) for f in files), encoding="utf-8")
    try:
        quiet = run(["tree-sitter", "parse", "--quiet", "--paths", str(listing)], arm)
        full = run(["tree-sitter", "parse", "--paths", str(listing)], arm)
    finally:
        listing.unlink(missing_ok=True)

    failed = set()
    for line in quiet.splitlines():
        head = line.split("\t", 1)[0].strip()
        if head.endswith(".kt"):
            failed.add(head)

    # 트리 하나가 **0 열의 `(`** 로 시작한다. 그 외 트리 줄은 전부 들여쓰기돼 있고,
    # 실패 파일의 진단 줄은 절대 경로라 `/` 로 시작한다.
    #
    # **뿌리가 `source_file` 이라고 가정하면 안 된다** — 파일이 통째로 안 읽히면
    # 뿌리가 `(ERROR` 다. `ng` 에서 42 개가 그렇고, 그렇게 세면 트리 수가 조용히 모자란다.
    trees: list[list[str]] = []
    for line in full.splitlines():
        if line.startswith("("):
            trees.append([line])
        elif trees and not line.startswith("/"):
            trees[-1].append(line)

    counts = query_counts(arm, query, files)
    structs = query_counts(arm, struct, files)

    rel = [f.relative_to(corpus).as_posix() for f in files]
    return {
        "rev": run(["git", "rev-parse", "HEAD"], arm).strip(),
        "dir": str(arm),
        "fail": {r for r, f in zip(rel, files) if str(f) in failed},
        "count": {r: counts.get(str(f), 0) for r, f in zip(rel, files)},
        "struct": {r: structs.get(str(f), 0) for r, f in zip(rel, files)},
        "tree": (
            {r: hashlib.blake2b("\n".join(t).encode(), digest_size=16).hexdigest()
             for r, t in zip(rel, trees)}
            if len(trees) == len(files) else None
        ),
        "n_trees": len(trees),
    }


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--arms", type=Path, required=True)
    ap.add_argument("--corpus", type=Path, required=True)
    ap.add_argument("--out", type=Path)
    a = ap.parse_args()
    a.corpus = a.corpus.resolve()

    files = sorted(a.corpus.rglob("*.kt"))
    print("G50 포크 오라클")
    print()

    # ── 하한 셋 ────────────────────────────────────────────────────────────
    if len(files) != N_FILES:
        print(f"  FAIL  파일 수가 {N_FILES} 가 아니다: {len(files)} — 코퍼스가 넷이 아니다")
        return 1
    names = [CONTROL, *CANDIDATES]
    dirs = {n: (a.arms / n).resolve() for n in names}
    if len(set(dirs.values())) != 3:
        print(f"  FAIL  팔이 디렉터리를 공유한다: {dirs}")
        return 1
    for n, d in dirs.items():
        if not (d / "grammar.js").exists():
            print(f"  FAIL  팔 {n} 의 클론이 없다: {d}")
            return 1
    print(f"  ok    하한 — 파일 {len(files)} · 팔 {len(names)} · 디렉터리 셋이 서로 다르다")

    # ── 팔마다 **따로 빌드하고 캐시를 가른다** ───────────────────────────────
    libs = {}
    for n, d in dirs.items():
        p = subprocess.run(["tree-sitter", "build"], cwd=d, capture_output=True,
                           text=True, env=env_of(d))
        if p.returncode != 0:
            print(f"  FAIL  팔 {n} 이 빌드되지 않는다 — 대조 불가\n{p.stderr[-400:]}")
            return 1
        libs[n] = sorted(home_of(d).rglob("tree-sitter/lib/*.dylib")) + \
            sorted(home_of(d).rglob("tree-sitter/lib/*.so"))
    if len({str(v) for v in libs.values()}) != 3 or any(not v for v in libs.values()):
        print(f"  FAIL  팔들이 파서 캐시를 공유한다 — CLI 가 문법 이름으로 캐시한다: {libs}")
        return 1
    print("  ok    ★ 세 팔이 각자의 디렉터리에서 빌드되고 **파서 캐시가 셋으로 갈렸다**")

    # ── 팔마다 쿼리를 놓는다. **`ng` 것만 저장소에 있다** ────────────────────
    qdir = a.arms / ".queries"
    qdir.mkdir(exist_ok=True)
    (qdir / "fork.scm").write_text(FORK_QUERY, encoding="utf-8")
    (qdir / "fork-struct.scm").write_text(STRUCT_ONLY["fork"], encoding="utf-8")
    (qdir / "ng-struct.scm").write_text(STRUCT_ONLY["ng"], encoding="utf-8")
    q = {CONTROL: (QUERY, qdir / "ng-struct.scm")}
    for n in CANDIDATES:
        q[n] = (qdir / "fork.scm", qdir / "fork-struct.scm")

    m = {n: measure(dirs[n], files, a.corpus, *q[n]) for n in names}

    # 침묵으로 드러나는 실패 — 빈 산출은 어긋남을 못 낸다
    for n in names:
        tot = sum(m[n]["count"].values())
        if tot == 0:
            print(f"  FAIL  팔 {n} 의 선언 총수가 0 이다 — 산출이 비면 어긋남을 못 낸다")
            return 1
        # ★ 이름을 안 보는 쿼리와 매치 수가 같아야 한다 — 이름 마디가 매치를 늘리거나
        #    줄이면 늘어난 선언에 우리 손이 섞인다([g50.pass] ③).
        bad = sorted(p for p in m[n]["count"] if m[n]["count"][p] != m[n]["struct"][p])
        if bad:
            print(f"  FAIL  ★ 팔 {n}: 이름 있는 쿼리와 구조만 보는 쿼리의 매치 수가 다르다 — {len(bad)} 파일")
            for p in bad[:10]:
                print(f"          {p}  이름 {m[n]['count'][p]} ↔ 구조 {m[n]['struct'][p]}")
            return 1
        if m[n]["tree"] is None:
            print(f"  FAIL  팔 {n} 의 트리 수가 파일 수와 다르다: {m[n]['n_trees']}")
            return 1
    print("  ok    세 팔 다 선언 총수 > 0 · 트리 수 == 파일 수")
    print("  ok    ★ 세 팔 다 **이름 있는 쿼리와 구조만 보는 쿼리의 매치 수가 같다**")

    # 세 팔의 산출이 전부 같으면 장치가 꺼진 것이다
    sig = {n: (sorted(m[n]["fail"]), m[n]["count"], m[n]["tree"]) for n in names}
    if sig[names[0]] == sig[names[1]] == sig[names[2]]:
        print("  FAIL  세 팔의 산출이 **글자까지 같다** — 측정 장치가 꺼졌다 (F02-2 의 링커 사고)")
        return 1
    print("  ok    ★ 세 팔의 산출이 같지 않다 — 장치가 켜져 있다")
    print()

    # ── 음성 대조 — 현행 핀이 자기 값을 재현하는가 ★ ─────────────────────────
    c = m[CONTROL]
    c_fail = len(c["fail"])
    c_rec = sum(1 for p in DEGRADED_27 if p not in c["fail"])
    ok_control = c_fail == CONTROL_EXPECT_FAILS and c_rec == CONTROL_EXPECT_RECOVERED
    print(f"  {'ok  ' if ok_control else 'FAIL'}  ★ 음성 대조 — {CONTROL} @ {c['rev'][:7]}: "
          f"깨짐 {c_fail} (기대 {CONTROL_EXPECT_FAILS}) · "
          f"강등 27 중 성함 {c_rec} (기대 {CONTROL_EXPECT_RECOVERED})")
    if not ok_control:
        print("        측정 장치가 F02-2 의 값을 재현하지 못한다 — 후보의 값도 못 믿는다")
        return 1
    print()

    # ── 표 ─────────────────────────────────────────────────────────────────
    print("  팔          rev        깨짐   선언 총수   강등 27 중 성함   조용한 오파싱 후보")
    silent = silent_misparse_17()
    # 독립 계수기는 **같은 파일 하나**를 쓴다 — 따로 세면 같은 결함이 여기서 다시 난다.
    # 파일명에 하이픈이 있어 `import` 가 안 되므로 소스를 그대로 실행한다.
    indep = {}
    ns: dict = {"__name__": "g50_kotlin_scan"}
    exec(compile((ROOT / "scripts/g50-kotlin-scan.py").read_text(encoding="utf-8"),
                 "g50-kotlin-scan.py", "exec"), ns)
    for f in files:
        indep[f.relative_to(a.corpus).as_posix()] = ns["count_file"](f)
    if sum(indep.values()) < MIN_INDEPENDENT_DECLS:
        print(f"  FAIL  독립 계수기가 {sum(indep.values())} 를 냈다 — {MIN_INDEPENDENT_DECLS} 미만이면 계수기가 고장 난 것이다")
        return 1

    report: dict = {"independent_total": sum(indep.values()), "arms": {}}
    for n in names:
        rec = sum(1 for p in DEGRADED_27 if p not in m[n]["fail"])
        cand = sorted(p for p in indep if indep[p] >= 1 and m[n]["count"][p] == 0)
        print(f"  {n:<10}  {m[n]['rev'][:7]}  {len(m[n]['fail']):>4}   "
              f"{sum(m[n]['count'].values()):>8}   {rec:>13}   {len(cand):>16}")
        report["arms"][n] = {
            "rev": m[n]["rev"], "fail": len(m[n]["fail"]),
            "declarations": sum(m[n]["count"].values()), "recovered_of_27": rec,
            "silent_suspects": cand,
        }
    print(f"  (독립 계수기 — tree-sitter 없이 센 선언 총수 {sum(indep.values())})")
    print()

    # ── 축 A — 두 후보의 차분 ───────────────────────────────────────────────
    x, y = CANDIDATES
    d_fail = sorted(m[x]["fail"] ^ m[y]["fail"])
    d_count = sorted(p for p in indep if m[x]["count"][p] != m[y]["count"][p])
    d_tree = sorted(p for p in indep if m[x]["tree"][p] != m[y]["tree"][p])
    print(f"  축 A — {x} ↔ {y} 차분:  파싱 성패 {len(d_fail)} · 선언 수 {len(d_count)} · 트리 {len(d_tree)}")
    for 이름, lst in (("파싱 성패", d_fail), ("선언 수", d_count), ("트리", d_tree)):
        for p in lst[:20]:
            extra = ""
            if 이름 == "선언 수":
                extra = f"  {m[x]['count'][p]} ↔ {m[y]['count'][p]}"
            print(f"        [{이름}] {p}{extra}")
        if len(lst) > 20:
            print(f"        … {len(lst) - 20} 더")
    report["axis_a"] = {"fail": d_fail, "count": d_count, "tree": d_tree}
    if not (d_fail or d_count or d_tree):
        print("        **차분 0 — 측정으로 가를 수 없다. 그것이 판정이다.** 축 C 로 내려간다")
    print()

    # ── 축 B — 손으로 읽을 후보 ─────────────────────────────────────────────
    sample = sorted(set(silent) | set(DEGRADED_27) | set(d_fail) | set(d_count) | set(d_tree))
    print(f"  축 B — 표본 {len(sample)} (조용한 오파싱 {len(silent)} · 강등 {len(DEGRADED_27)} · 축 A 차분)")
    for n in CANDIDATES:
        sus = [p for p in sample if indep[p] >= 1 and m[n]["count"][p] == 0]
        print(f"        {n}: 손으로 읽을 후보 {len(sus)}")
        for p in sus:
            print(f"          {p}  독립 {indep[p]} · 문법 0 · {'깨짐' if p in m[n]['fail'] else '성함'}")
        report["arms"][n]["axis_b_suspects"] = sus
    report["axis_b_sample"] = sample
    report["silent_17"] = silent

    # ── 축 C — **축 A 가 못 갈랐을 때만 돈다** ───────────────────────────────
    #
    # 돌리는 조건을 코드로 박는다. 축 A 가 갈랐는데도 이것을 보면 그것이 곧
    # 「고른 뒤에 근거를 만드는 일」이다([g50.oracle].axis_c).
    if d_fail or d_count or d_tree:
        print("\n  축 C — **돌지 않는다.** 축 A 가 갈랐다")
        return 0

    print(f"\n  축 C — 축 A 가 못 갈랐으므로 순서대로 댄다 (upstream = {UPSTREAM})")
    axis_c: dict = {}
    for n in CANDIDATES:
        d = dirs[n]
        subprocess.run(["git", "remote", "add", "g50-upstream", UPSTREAM], cwd=d,
                       capture_output=True, text=True)
        subprocess.run(["git", "fetch", "-q", "g50-upstream"], cwd=d,
                       capture_output=True, text=True)
        up = run(["git", "rev-parse", "g50-upstream/HEAD"], d).strip()
        behind = run(["git", "rev-list", "--count", f"HEAD..{up}"], d).strip()
        behind_grammar = [l for l in run(
            ["git", "log", "--format=%ad %h %s", "--date=short",
             f"HEAD..{up}", "--", "grammar.js", "src/scanner.c"], d).splitlines() if l]
        lic = ""
        for line in (d / "Cargo.toml").read_text(encoding="utf-8").splitlines():
            if line.startswith("license"):
                lic = line.split("=", 1)[1].strip().strip('"')
        axis_c[n] = {
            "rev": m[n]["rev"], "license": lic, "upstream": up,
            "behind_upstream": int(behind or 0),
            "behind_upstream_grammar": behind_grammar,
            "last_commit": run(["git", "log", "-1", "--format=%ad", "--date=short"], d).strip(),
        }
        print(f"        {n:<6} rev {m[n]['rev'][:7]} · 라이선스 {lic} · 마지막 커밋 "
              f"{axis_c[n]['last_commit']} · **upstream 보다 뒤진 커밋 {behind}** "
              f"(그중 문법 {len(behind_grammar)})")
        for l in behind_grammar:
            print(f"            뒤진 문법 커밋: {l}")
    report["axis_c"] = axis_c

    ranked = sorted(CANDIDATES, key=lambda n: (axis_c[n]["behind_upstream"], n))
    if axis_c[ranked[0]]["behind_upstream"] == axis_c[ranked[1]]["behind_upstream"]:
        print("        넷이 다 같다 — **사전순으로 고르고 그렇게 골랐다고 적는다**")
    else:
        print(f"        → **{ranked[0]}** — 축 C ③(상류 추종)이 갈랐다. "
              f"①②④ 는 동률이었다")
    report["chosen"] = ranked[0]

    if a.out:
        a.out.write_text(json.dumps(report, ensure_ascii=False, indent=2), encoding="utf-8")
        print(f"\n→ {a.out}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
