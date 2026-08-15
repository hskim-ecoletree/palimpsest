#!/usr/bin/env python3
"""F12 대조 — **계획-구현 결박·이탈률** (#16).

합격선은 `corpus/criteria.toml` `[f12]` 에 있고 **첫 코드 커밋보다 먼저, 별도 커밋으로**
등록됐다. ⚠ **`registered_before_any_measurement = false` 다** — 착수 조사가 코퍼스를
이미 쟀고, 본 것 전부가 `[f12].measurement_already_seen` 에 있다.

    ① **3 분류가 각각 모집단 ≥ 1** — 전부 `unplanned` / 전부 `unmeasurable` 을 둘 다 막는다
    ② **`unmeasurable` 분리** — 넷이 각각 실리고 미구현에 합산되지 않는다
    ③ ★ **음성 대조 — 짝을 섞으면 `as_planned` 가 떨어진다.** 하한 없이 부등호 하나
    ④ **`pending` → `live` 전이** 모집단 ≥ 1 · 그리고 **결박을 하나도 안 만든다**(구조)
    ⑤ **심볼 단위 diff** — 포매팅 0 · 본문 ≥ 1 (양쪽)
    ⑥ **경로 패턴 상한** — 초과하면 거부한다
    ⑦ **`plan.deviation` 카탈로그 정합** + 질의 로그
    ⑧ **좌표 해소율** — ★ 값이다. **하한이 없다**
    ⑨ **기획→결정 해소율** — ★ 값이다. **하한이 없다**
    ⑩ **계획 선행성** — ★ 0 이면 ①③이 **대조 불가**다
    ⑪ **자기 적용** — 구조적 **대조 불가**(Rust 추출기 부재 · ADR-0017)
    ⑫ **골든 넷이 안 움직인다**
    ⑬ **경계** — `--base` 를 안 만든다(F23)

★ **소급 대조의 입력은 `[f12].plan_projection` 이 재기 전에 기계 규칙으로 고정했다.**
여기서 work item 이나 좌표를 고르지 않는다 — 고르면 이 대조는 이탈률이 아니라
**우리의 선별 솜씨**를 잰다.

⚠ **종료 코드는 「어긋남」만 센다. 「대조 불가」는 안 센다** — 판정은 이 스크립트가
아니라 `docs/gates/F12.md` 가 한다(F11 이 실물 사례다).

사용:
    ./scripts/f12-verify.py
    ./scripts/f12-verify.py --limit 20     # 소급 대조를 앞 20 건만 (개발용 · 게이트 아님)
"""

from __future__ import annotations

import argparse
import importlib.util
import json
import re
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
BIN = ROOT / "target/release/pal"

_spec = importlib.util.spec_from_file_location("f10_verify", ROOT / "scripts/f10-verify.py")
F10 = importlib.util.module_from_spec(_spec)
assert _spec.loader is not None
_spec.loader.exec_module(F10)

결과 = F10.결과
ok, fail, skip = F10.ok, F10.fail, F10.skip
run, 사본 = F10.run, F10.사본
DITTO, DITTO_PIN = F10.DITTO, F10.DITTO_PIN

# ── `[f12.pass]` 가 등록한 값들. **여기서 정하지 않는다 — 옮겨 적을 뿐이다** ────────
CLASSIFICATION_POPULATION_MIN = 1     # 이 저장소의 하한 관례
PENDING_POPULATION_MIN = 1
PATTERN_REFUSAL_POPULATION_MIN = 1
SYMBOL_DIFF_FORMATTING_MAX = 0
SYMBOL_DIFF_BODY_MIN = 1
PATTERN_FILE_MAX = 32                 # `CANDIDATE_LIMIT` 을 그대로 쓴다 — 새 숫자가 아니다

CATALOG = ROOT / "surface/queries.toml"
QUERY_NAME = "plan.deviation"

# `plan_projection` — 추출기가 보는 확장자. `t_code` 의 자격이다.
코드_확장자 = (".ts", ".tsx", ".js", ".jsx", ".mjs", ".cjs", ".kt", ".java")

WI = re.compile(r"\bwi_[a-z0-9]+\b")


def pal_raw(args: list[str], repo: Path, box: Path):
    cmd = [str(BIN), *args, "--repo", str(repo), "--cache-dir", str(box / "cache")]
    return run(cmd)


def git(repo: Path, args: list[str]) -> str:
    p = run(["git", "-C", str(repo), *args])
    if p.returncode != 0:
        raise SystemExit(f"git {args}: {p.stderr[-400:]}")
    return p.stdout


# ═════════════════════════════════════════════════════════════════════════════
# `[f12].plan_projection` — **재기 전에 고정된 규칙. 여기서 고르지 않는다**
# ═════════════════════════════════════════════════════════════════════════════

def 커밋_지도(repo: Path) -> dict[str, list[tuple[str, int, bool]]]:
    """work item → [(sha, 시각, 코드를_바꿨나)] · 시각 오름차순.

    **규칙**: 커밋 메시지 전문에 `wi_<id>` 가 나타나면 그 커밋은 그 work item 의 것이다.
    ⚠ ditto 에 git 트레일러 관례가 없다 — 실제 표기는 제목의 `wi_...` 다.
    """
    raw = git(repo, ["log", DITTO_PIN, "--format=%H%x1f%ct%x1f%B%x1e"])
    out: dict[str, list[tuple[str, int, bool]]] = {}
    for rec in raw.split("\x1e"):
        rec = rec.strip("\n")
        if not rec:
            continue
        sha, ts, body = rec.split("\x1f", 2)
        ids = set(WI.findall(body))
        if not ids:
            continue
        files = git(repo, ["show", "--name-only", "--format=", sha]).split()
        코드 = any(f.endswith(코드_확장자) for f in files)
        for wi in ids:
            out.setdefault(wi, []).append((sha, int(ts), 코드))
    for v in out.values():
        v.sort(key=lambda t: t[1])
    return out


def 레코드들(repo: Path) -> dict[str, dict]:
    """추적되는 `.ditto/work-items/<wi>/record.json` 전부."""
    names = git(repo, ["ls-tree", "-r", "--name-only", DITTO_PIN, ".ditto/work-items"]).splitlines()
    out: dict[str, dict] = {}
    for n in names:
        if not n.endswith("/record.json"):
            continue
        wi = n.split("/")[2]
        try:
            out[wi] = json.loads(git(repo, ["show", f"{DITTO_PIN}:{n}"]))
        except (json.JSONDecodeError, SystemExit):
            continue
    return out


def 계획_문서(rec: dict, baseline: str) -> str:
    """`[f12].plan_projection` 의 렌더링 규칙 — **한 글자도 고치지 않는다.**"""
    머리 = [f"---", f"baseline: {baseline}", "---", "",
            f"# {rec.get('title', rec.get('id', ''))}", ""]
    for k in ("source_request", "goal"):
        v = (rec.get(k) or "").strip()
        if v:
            머리.append(v)
            머리.append("")
    본문 = []
    for ac in rec.get("acceptance_criteria") or []:
        본문.append(f"## {ac.get('id', '')}")
        본문.append((ac.get("statement") or "").strip())
        for ev in ac.get("evidence") or []:
            cmd = (ev.get("command") or "").strip()
            if cmd:
                본문.append(f"검증: {cmd}")
        본문.append("")
    return "\n".join(머리 + 본문) + "\n"


def 선행성(repo: Path, wi: str, commits: list[tuple[str, int, bool]]) -> tuple[int, int] | None:
    """`(t_plan, t_code)` — `[f12].deviation_rule` ⑩ 그대로.

    `t_plan` = `record.json` 에 **비지 않은 `acceptance_criteria`** 가 처음 나타난 커밋의 시각.
    `t_code` = **코드 파일을 바꾼** 가장 이른 wi 커밋의 시각.
    """
    코드 = [ts for _, ts, c in commits if c]
    if not 코드:
        return None
    path = f".ditto/work-items/{wi}/record.json"
    hist = git(repo, ["log", DITTO_PIN, "--format=%H%x09%ct", "--reverse", "--", path])
    for line in hist.splitlines():
        sha, ts = line.split("\t")
        try:
            rec = json.loads(git(repo, ["show", f"{sha}:{path}"]))
        except (json.JSONDecodeError, SystemExit):
            continue
        if rec.get("acceptance_criteria"):
            return int(ts), min(코드)
    return None


# ═════════════════════════════════════════════════════════════════════════════
# 소급 대조 — ①②③⑧⑨⑩
# ═════════════════════════════════════════════════════════════════════════════

def 소급(tmp: Path, limit: int | None) -> dict | None:
    print("소급 대조 — ditto @", DITTO_PIN)
    repo, box = 사본(tmp, "f12", DITTO, DITTO_PIN)
    plans = tmp / "plans"
    plans.mkdir()

    recs = 레코드들(repo)
    cmap = 커밋_지도(repo)
    # ★ **모집단을 이 스크립트가 고르지 않는다** — 규칙 둘의 교집합이다.
    쌍 = sorted(wi for wi in recs if wi in cmap)
    print(f"  record.json {len(recs)} · wi 커밋을 가진 work item {len(cmap)} · **둘 다 {len(쌍)}**")

    # ⑨ 기획 → 결정. **분모는 record.json 전부다** — `Plan` 이 선 것만 세면 1.0 이 된다.
    ac_있음 = sum(1 for r in recs.values() if r.get("acceptance_criteria"))
    ok("⑨ 기획→결정 해소율", f"**{ac_있음} / {len(recs)}** — AC 가 1 건 이상인 work item / "
                              f"추적되는 record.json 전부. **하한 없음**(F12 §6 이 이 값의 "
                              f"소비자를 F18 의 판단으로 적었다)")

    # ⑩ 선행성 — ①③의 자격이다.
    선행, 사후, 코드없음 = [], [], []
    for wi in 쌍:
        r = 선행성(repo, wi, cmap[wi])
        if r is None:
            코드없음.append(wi)
        elif r[0] <= r[1]:
            선행.append(wi)
        else:
            사후.append(wi)
    ok("⑩ 계획 선행성", f"선행 **{len(선행)}** · 사후 기입 {len(사후)} · 코드 커밋 없음 "
                        f"{len(코드없음)} / 짝 {len(쌍)} — **하한 없음.** ⚠ 0 이면 ①③이 대조 불가")
    if not 선행:
        skip("① 3 분류 모집단", "계획이 코드보다 먼저 적힌 짝이 **0** — 사후 기입된 계획으로 "
                                "잰 이탈률은 아무것도 재지 않는다 (`plan-drift-pairs.toml` 의 선결 조건)")
        skip("③ 섞음 대조", "①과 같은 이유 — **대조 불가**")
        return None

    표본 = 선행[:limit] if limit else 선행
    if limit:
        print(f"  ⚠ `--limit {limit}` — 게이트 판정용이 아니다")

    # ── 두 회차: 짝지음과 회전 섞음 ──────────────────────────────────────────
    #
    # ★ **순열을 결정적으로 정한다** — `wi` 사전순 정렬 후 인덱스 +1 회전 하나뿐이다.
    #    여러 순열을 돌려 좋은 것을 고르면 그것이 곧 눈금을 옮긴 것이다.
    짝 = {wi: wi for wi in 표본}
    섞 = {wi: 표본[(i + 1) % len(표본)] for i, wi in enumerate(표본)}

    def 회차(맵: dict[str, str], tag: str) -> list[dict]:
        out = []
        for wi, 구간 in 맵.items():
            cs = cmap[구간]
            base = f"{cs[0][0]}^"
            head = cs[-1][0]
            doc = plans / f"{tag}-{wi}.md"
            doc.write_text(계획_문서(recs[wi], base), encoding="utf-8")
            p = pal_raw(["deviation", str(doc), "--at", head, "--json"], repo, box)
            if p.returncode != 0:
                continue
            d = json.loads(p.stdout)
            d["_wi"] = wi
            d["_foreign"] = 남의_커밋(repo, base, head, 구간, cmap)
            out.append(d)
        return out

    A = 회차(짝, "paired")
    B = 회차(섞, "shuffled")
    if not A:
        skip("① 3 분류 모집단", "짝지은 회차가 하나도 안 돌았다 — **대조 불가**")
        return None

    총 = 합산(A)
    # ① 3 분류 모집단
    부족 = [k for k in ("as_planned", "unplanned", "unimplemented")
            if 총[k] < CLASSIFICATION_POPULATION_MIN]
    줄 = (f"계획대로 **{총['as_planned']}** · 계획에 없던 **{총['unplanned']}** · "
          f"미구현 **{총['unimplemented']}** · (못 잼 {총['unmeasurable']}) / 계획 {len(A)}")
    if 부족:
        fail("① 3 분류 모집단", f"{줄} — {부족} 가 하한 {CLASSIFICATION_POPULATION_MIN} 미만")
    else:
        ok("① 3 분류 모집단", 줄)

    # ② `unmeasurable` 분리 — 산출에 넷이 각각 있고, 합산되지 않는다.
    넷 = all(k in A[0]["deviation"] for k in
             ("as_planned", "unplanned", "unimplemented", "unmeasurable"))
    if 넷 and 총["unmeasurable"] > 0:
        ok("② `unmeasurable` 분리", f"산출에 넷이 각각 있다 · 못 잼 **{총['unmeasurable']}** 이 "
                                     f"미구현 {총['unimplemented']} 과 다른 줄이다")
    elif not 넷:
        fail("② `unmeasurable` 분리", "산출에 넷이 다 있지 않다")
    else:
        skip("② `unmeasurable` 분리", "못 잰 항목이 0 이라 분리가 시험되지 않았다 — **대조 불가**")

    # ③ ★ 음성 대조 — 짝을 섞으면 떨어진다. **하한 없이 부등호 하나다.**
    a, b = 총["as_planned"], 합산(B)["as_planned"]
    if a > b:
        ok("③ 섞음 대조", f"짝지음 **{a}** > 섞음 **{b}** — 이탈이 계획에 반응한다. "
                          f"⚠ **차이가 곧 이 지표의 두께다**")
    else:
        fail("③ 섞음 대조", f"짝지음 {a} ≤ 섞음 {b} — **이탈률이 계획을 안 재고 있다**. "
                            f"그 파일이 자주 바뀐다는 사실을 재는 것이다")

    # ④ pending → live
    pending = sum(d["deviation"]["promoted_from_pending"] for d in A)
    p상태 = sum(1 for d in A for r in d["resolutions"]
                for x in r["at_baseline"] if x["state"]["state"] == "pending")
    if p상태 >= PENDING_POPULATION_MIN and pending >= PENDING_POPULATION_MIN:
        ok("④ `pending` → `live`", f"기준선에서 `pending` **{p상태}** · 전이 **{pending}** "
                                    f"(하한 {PENDING_POPULATION_MIN})")
    elif p상태 < PENDING_POPULATION_MIN:
        skip("④ `pending` → `live`", f"실 코퍼스에서 `pending` 모집단이 {p상태} — 픽스처는 "
                                      f"`cargo test` 가 진다. **픽스처의 통과를 실 코퍼스의 "
                                      f"통과로 적지 않는다**")
    else:
        fail("④ `pending` → `live`", f"`pending` {p상태} 인데 전이 {pending} — 「핵심 상태」가 "
                                      f"영원히 안 풀린다")

    # ⑧ 좌표 해소율 — ★ **하한 없음.**
    해소 = sum(d["deviation"]["as_planned"] and 1 or 0 for d in A)
    항목 = sum(len(d["plan"]["items"]) for d in A)
    잰것 = 항목 - 총["unmeasurable"]
    사유 = 합산_사유(A)
    신호 = 합산_신호(A)
    ok("⑧ 좌표 해소율", f"**{잰것} / {항목}** 항목 · 못 잼 사유 {사유} · "
                        f"`as_planned` 의 신호별 {신호} — ★ **하한 없음**")

    # ★ **지표 자신의 값** — `[outcome]` M2. 합격선이 아니라 산출이고,
    #   `[outcome].step_4_report` 가 *"값 · n · 제외 건수 · caveat 을 함께 낸다"* 를 요구한다.
    율 = sorted(r["value"] for r in (d["deviation"] and 이탈률(d) for d in A) if r)
    정의안됨 = len(A) - len(율)
    if 율:
        중앙율 = 율[len(율) // 2]
        ok("· M2 이탈률", f"중앙값 **{중앙율:.3f}** · 분포 [{율[0]:.2f}, {율[-1]:.2f}] · "
                          f"n **{len(율)}** · 정의되지 않음 {정의안됨}(실제 변경 0) · "
                          f"제외 {len(쌍) - len(표본)}(선행성·코드 커밋 없음) — "
                          f"⚠ **caveat 은 아래 「구간 오염」과 ⑧의 신호별 층화다**")
    else:
        skip("· M2 이탈률", "정의된 회차가 없다 — 실제 변경이 전부 0")

    # 확인 — 남의 커밋이 섞인 정도(구간의 오염). **값이지 합격선이 아니다.**
    f = sorted(d["_foreign"] for d in A)
    중앙 = f[len(f) // 2] if f else 0
    ok("· 구간 오염", f"B..H 안에서 **그 work item 이 아닌 커밋**의 비율 — 중앙값 "
                      f"**{중앙:.2f}** · 최대 **{f[-1]:.2f}** · 0 인 회차 "
                      f"**{sum(1 for x in f if x == 0)}/{len(f)}** — ⚠ 섞인 만큼 "
                      f"이탈률의 분자가 부푼다. ③이 그 위에서도 서는지를 잰다")
    return {"repo": repo, "box": box, "A": A}


def 이탈률(d: dict) -> dict | None:
    """`|A ∖ D| / |A|` — **실제 변경이 0 이면 정의되지 않는다.**

    *"하나도 안 벗어났다"* 와 *"잴 것이 없었다"* 는 다른 답이고, 코어가 그것을
    `DeviationRate::Undefined` 로 가른다.
    """
    dev = d["deviation"]
    a = len(dev["delta"]["changed"]) + len(dev["delta"]["added"]) + len(dev["delta"]["removed"])
    if a == 0:
        return None
    return {"value": len(dev["unplanned"]) / a}


def 남의_커밋(repo: Path, base: str, head: str, wi: str, cmap: dict) -> float:
    """`B..H` 안에서 그 work item 이 아닌 커밋의 비율."""
    shas = git(repo, ["rev-list", f"{base}..{head}"]).split()
    if not shas:
        return 0.0
    mine = {s for s, _, _ in cmap[wi]}
    return sum(1 for s in shas if s not in mine) / len(shas)


def 합산(rows: list[dict]) -> dict[str, int]:
    out = {k: 0 for k in ("as_planned", "unplanned", "unimplemented", "unmeasurable")}
    for r in rows:
        for k in out:
            out[k] += len(r["deviation"][k])
    return out


def 합산_사유(rows: list[dict]) -> str:
    acc: dict[str, int] = {}
    for r in rows:
        for u in r["deviation"]["unmeasurable"]:
            acc[u["why"]] = acc.get(u["why"], 0) + 1
    return " · ".join(f"{k} {v}" for k, v in sorted(acc.items())) or "없음"


def 합산_신호(rows: list[dict]) -> str:
    acc: dict[str, int] = {}
    for r in rows:
        for p in r["deviation"]["as_planned"]:
            acc[p["by"]] = acc.get(p["by"], 0) + 1
    return " · ".join(f"{k} {v}" for k, v in sorted(acc.items())) or "없음"


# ═════════════════════════════════════════════════════════════════════════════
# ⑤ 심볼 단위 diff — **양방향이다**
# ═════════════════════════════════════════════════════════════════════════════

def 심볼_diff(tmp: Path) -> None:
    print("⑤ 심볼 단위 diff — 포매팅과 본문")
    repo, box = 사본(tmp, "f12-diff", DITTO, DITTO_PIN)
    src = repo / "src/core/git.ts"
    if not src.exists():
        skip("⑤ 심볼 단위 diff", "표본 파일이 없다 — **대조 불가**")
        return

    def 심볼들(rev: str | None) -> dict[str, str]:
        args = ["ledger", str(repo), "--symbols", "--cache-dir", str(box / "cache")]
        if rev:
            args += ["--at", rev]
        p = run([str(BIN), *args])
        if p.returncode != 0:
            raise SystemExit(f"ledger --symbols 실패: {p.stderr[-400:]}")
        out = {}
        for line in p.stdout.splitlines():
            if not line.strip():
                continue
            s = json.loads(line)
            out[s["id"]] = s["body"]
        return out

    기준 = 심볼들(DITTO_PIN)

    # ── 가 — 포매팅만 바꾼다. **변경 심볼이 0 이어야 한다** ──────────────────
    원본 = src.read_text(encoding="utf-8")
    src.write_text(포매팅만(원본), encoding="utf-8")
    포매팅후 = 심볼들(None)
    바뀐_포매팅 = [k for k, v in 포매팅후.items() if 기준.get(k) not in (None, v)]

    # ── 나 — 본문을 바꾼다. **≥ 1 이어야 한다** ─────────────────────────────
    src.write_text(원본 + "\nexport function palF12ProbeChanged(): number { return 41; }\n",
                   encoding="utf-8")
    본문후 = 심볼들(None)
    새것 = [k for k in 본문후 if k not in 기준]
    src.write_text(원본, encoding="utf-8")

    if len(바뀐_포매팅) <= SYMBOL_DIFF_FORMATTING_MAX and len(새것) >= SYMBOL_DIFF_BODY_MIN:
        ok("⑤ 심볼 단위 diff", f"포매팅만 바꿈 → 변경 **{len(바뀐_포매팅)}** "
                               f"(상한 {SYMBOL_DIFF_FORMATTING_MAX}) · "
                               f"본문 바꿈 → 새 심볼 **{len(새것)}** (하한 {SYMBOL_DIFF_BODY_MIN})")
    else:
        fail("⑤ 심볼 단위 diff", f"포매팅 {len(바뀐_포매팅)} · 본문 {len(새것)} — "
                                 f"한쪽만 서면 반쪽이 만점을 받는다")


def 포매팅만(src: str) -> str:
    """공백과 후행 쉼표만 건드린다 — **선언을 안 만든다.**"""
    return "\n".join(l.rstrip() + "  " if l.strip() else l for l in src.splitlines()) + "\n"


# ═════════════════════════════════════════════════════════════════════════════
# ⑥⑦⑬ — 상한 · 카탈로그 · 경계
# ═════════════════════════════════════════════════════════════════════════════

def 상한과_카탈로그(tmp: Path) -> None:
    repo, box = 사본(tmp, "f12-cat", DITTO, DITTO_PIN)

    # ⑥ 경로 패턴의 상한
    doc = tmp / "wide.md"
    doc.write_text(
        f"---\nbaseline: {DITTO_PIN}\n---\n# 넓은 계획\n무엇\n\n## a-1\n좌표: src/**\n",
        encoding="utf-8",
    )
    p = pal_raw(["plan", str(doc), "--at", DITTO_PIN, "--json"], repo, box)
    if p.returncode != 0:
        fail("⑥ 경로 패턴 상한", f"`pal plan` 이 실패했다: {p.stderr[-200:]}")
    else:
        states = json.loads(p.stdout)["states"]
        거부 = sum(1 for row in states for s in row
                   if s.get("state") == "unresolved" and s.get("why") == "pattern-too-broad")
        if 거부 >= PATTERN_REFUSAL_POPULATION_MIN:
            ok("⑥ 경로 패턴 상한", f"`src/**` 가 거부됐다 **{거부}** 건 (상한 {PATTERN_FILE_MAX} 파일) — "
                                   f"⚠ **픽스처다.** 실 코퍼스의 통과로 안 적는다")
        else:
            fail("⑥ 경로 패턴 상한", f"`src/**` 가 안 거부됐다 — 상한이 안 걸린다")

    # ⑦ 카탈로그 정합 + 질의 로그
    catalog = CATALOG.read_text(encoding="utf-8")
    코드 = (ROOT / "crates/pal-core/src/query_log.rs").read_text(encoding="utf-8")
    실행기 = (ROOT / "crates/pal-query/src/lib.rs").read_text(encoding="utf-8")
    네자리 = (f'[query."{QUERY_NAME}"]' in catalog
              and f'rename = "{QUERY_NAME}"' in 코드
              and "PlanDeviation" in 실행기)
    ok_doc = tmp / "one.md"
    ok_doc.write_text(
        f"---\nbaseline: {DITTO_PIN}~1\n---\n# 하나\n무엇\n\n## a-1\n`resolveClaimBranch` 를 고친다\n",
        encoding="utf-8",
    )
    q = pal_raw(["query", QUERY_NAME, str(ok_doc), "--at", DITTO_PIN, "--json"], repo, box)
    로그 = False
    if q.returncode == 0:
        env = json.loads(q.stdout)
        로그 = env.get("log", {}).get("status") == "recorded"
    if 네자리 and q.returncode == 0 and 로그:
        ok("⑦ 카탈로그 정합", f"`{QUERY_NAME}` 이 네 자리에 함께 서고 **질의 로그가 남는다**")
    elif 네자리 and q.returncode == 0:
        fail("⑦ 카탈로그 정합", "네 자리는 섰는데 **질의 로그가 안 남는다** — F17 이 미조회를 과대 계상한다")
    else:
        fail("⑦ 카탈로그 정합", f"네 자리 {네자리} · 질의 종료 {q.returncode}: {q.stderr[-200:]}")
    skip("⑦ MCP 경로", "`crates/pal-mcp` 가 없다 (F06b) — **「CLI 만 섰으므로 절반 통과」로 "
                       "안 적는다**")

    # ⑬ 경계 — `--base` 를 안 만든다
    #
    # ⚠ **문자열 하나로 세지 않는다.** 이 명령의 도움말은 *"`--base <ref>` 가 없다"* 를
    # 문장으로 적고 있어서, 본문을 훑으면 **자기 설명 때문에 실패한다**(실제로 밟았다).
    # 손잡이는 **`Options` 의 줄**이고 그 줄은 공백으로 시작한다.
    도움 = run([str(BIN), "deviation", "--help"])
    손잡이 = re.compile(r"^\s+--([a-z0-9-]+)", re.M)
    이름들 = set(손잡이.findall(도움.stdout))
    if "repo" not in 이름들:
        # **음성 대조** — 있는 손잡이를 못 찾으면 이 검사는 아무것도 안 센다.
        fail("⑬ 경계", f"손잡이를 하나도 못 읽었다({sorted(이름들)}) — 검사가 고장 났다")
    elif "base" in 이름들:
        fail("⑬ 경계", "`pal deviation` 에 `--base` 가 있다 — **F23 을 당겨왔다**")
    else:
        ok("⑬ 경계", f"손잡이 {sorted(이름들)} — `--base` 가 **없다.** 기준선은 계획 "
                      f"문서가 진다(F12 §4)")

    # ④의 구조 — **이 기능은 결박을 하나도 안 만든다**
    소스 = "\n".join(
        (ROOT / p).read_text(encoding="utf-8")
        for p in ("crates/pal-core/src/plan.rs", "crates/pal-extract/src/plan.rs",
                  "crates/pal-cli/src/plan.rs")
    )
    금지 = [m for m in ("Binding::new", "Binding::promote", "IntentStore::open(") if m in 소스]
    if 금지:
        fail("④ 결박 부재(구조)", f"계획 경로가 결박을 만든다: {금지} — ADR-0015 가 금한 형태다")
    else:
        ok("④ 결박 부재(구조)", "계획 경로에 `Binding::new`·`promote`·의도 저장소 쓰기가 **0** — "
                                 "거리 있는 신호로 `asserted` 를 만들지 않는다(ADR-0015)")


# ═════════════════════════════════════════════════════════════════════════════
# ⑪ 자기 적용 — ⚠ **구조적 대조 불가**(ADR-0017)
# ═════════════════════════════════════════════════════════════════════════════

def 자기_적용(tmp: Path) -> None:
    box = tmp / "self"
    box.mkdir()
    doc = ROOT / "docs/plan/features/F12-plan-binding.md"
    p = pal_raw(["plan", str(doc), "--json"], ROOT, box)
    if p.returncode != 0:
        skip("⑪ 자기 적용", f"자기 계획 문서를 못 읽었다: {p.stderr[-200:]} — **대조 불가**")
        return
    v = json.loads(p.stdout)
    항목 = len(v["plan"]["items"])
    걸린 = sum(1 for row in v["states"] for s in row if s.get("state") == "bound")
    skip("⑪ 자기 적용", f"계획 항목 **{항목}** 이 실재하고 좌표로 걸린 것이 **{걸린}** — "
                        f"⚠ **모집단 0 이 아니다.** 항목이 지목하는 것은 Rust 식별자이고 "
                        f"추출기는 Kotlin·Java·JS·TS 넷뿐이다. [ADR-0017] 의 **「자가 짧다」**이고 "
                        f"⑧에 합산하지 않는다 (#66)")


# ═════════════════════════════════════════════════════════════════════════════
# ⑫ 골든 넷
# ═════════════════════════════════════════════════════════════════════════════

def 골든() -> None:
    p = run(["git", "-C", str(ROOT), "status", "--porcelain", "--",
             "corpus/golden", "corpus/tasks/s0-reference-vector.tsv"])
    if p.stdout.strip():
        fail("⑫ 골든 넷", f"움직였다:\n    {p.stdout.strip()}")
    else:
        ok("⑫ 골든 넷", "안 움직였다 — ⚠ **이 절은 `pal-extract` 를 안 건드리므로 구조적으로 "
                         "당연하다. 당연한 것을 통과로 세지 않는다**")


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--limit", type=int, default=None, help="소급 대조를 앞 N 건만 (개발용)")
    a = ap.parse_args()

    if not BIN.exists():
        raise SystemExit(f"바이너리가 없다: {BIN} — `cargo build --release`")
    if not DITTO.exists():
        raise SystemExit(f"코퍼스가 없다: {DITTO}")

    with tempfile.TemporaryDirectory(prefix="pal-f12-") as td:
        tmp = Path(td)
        소급(tmp, a.limit)
        심볼_diff(tmp)
        상한과_카탈로그(tmp)
        자기_적용(tmp)
    골든()

    print()
    for 표시, 이름, 값 in 결과:
        print(f"  {표시:<5} {이름}  — {값}")

    print()
    어긋남 = [r for r in 결과 if r[0] == "FAIL"]
    대조불가 = [r for r in 결과 if r[0] == "–"]
    for 표시, 이름, 값 in 어긋남:
        print(f"  FAIL  {이름}: {값}")
    if 대조불가:
        print(f"대조 불가 {len(대조불가)}건 — **통과로 세지 않는다**")
        for _, 이름, 값 in 대조불가:
            print(f"   – {이름}: {값}")
    print(f"어긋남 {len(어긋남)}")
    return 1 if 어긋남 else 0


if __name__ == "__main__":
    sys.exit(main())
