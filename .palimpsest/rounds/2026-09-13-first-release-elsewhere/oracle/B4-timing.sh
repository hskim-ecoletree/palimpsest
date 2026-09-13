#!/usr/bin/env bash
# `B4` — 승인 대기를 더한 `pal touch` 의 질의 시간이 등록한 선 안인가.
#
# 선: `corpus/criteria.toml` `[f11.pass]` ⑦ — 질의 시간(`elapsed_micros`)의 p95 < 500 ms · 표본 50.
# 표본 규칙: `[f09.4].sample_selection` 규칙 4 — `symbol_id` 사전순 등간격.
#
#   표본 ① 전체 심볼에서 50
#   표본 ② **승인 대기가 1 건 이상 실리는 심볼만**에서 50 — 조립할 것이 없는 호출만 재면 전부 빠르다
#
# 부르는 것은 **지목 문자열**(짧은 해시)이다 — 이름만 주면 동명 심볼에서 후보 화면으로 끝나
# 대기 구역을 조립하지 않는 호출이 섞인다.
#
# 사용: B4-timing.sh <pal 바이너리> <저장소>
set -euo pipefail
PAL="$1"; REPO="$2"
cd "$REPO"

echo "# B4 — 승인 대기를 더한 \`pal touch\` 의 질의 시간"
echo
echo "- pal: \`$("$PAL" --version)\`"
echo "- 저장소: \`$(basename "$REPO")\` HEAD \`$(git rev-parse --short HEAD)\`"
echo "- 선: 질의 시간(\`elapsed_micros\`) p95 < 500 ms · 표본 50 · \`symbol_id\` 사전순 등간격"
echo

"$PAL" narrative > /dev/null
"$PAL" ledger --symbols > "$REPO/.palimpsest/b4-symbols.jsonl"

python3 - "$PAL" "$REPO" <<'PYEOF'
import json, subprocess, sys, statistics, re
pal, repo = sys.argv[1], sys.argv[2]
syms = [json.loads(l) for l in open(f"{repo}/.palimpsest/b4-symbols.jsonl", encoding="utf-8") if l.strip()]
syms.sort(key=lambda s: s["id"])
pending = json.load(open(f"{repo}/.palimpsest/narrative-pending.json", encoding="utf-8"))
targets = {t for e in pending["entries"] for t in e["targets"]}

def 등간격(xs, n=50):
    if len(xs) <= n:
        return list(xs)
    return [xs[(i * len(xs)) // n] for i in range(n)]

def 잰다(표본):
    rows = []
    for s in 표본:
        r = subprocess.run([pal, "touch", s["name"], "--pick", s["id"][:12], "--timing"],
                           cwd=repo, capture_output=True, text=True)
        m = re.search(r"elapsed_micros=(\S+) process_micros=(\S+)", r.stderr)
        q = m.group(1) if m else "없음"
        p = m.group(2) if m else "없음"
        대기 = re.search(r"■ 승인 대기 — 이 좌표를 가리키는 문서 조각 \((\d+)\)", r.stdout)
        rows.append({
            "q": None if q in ("none", "없음") else int(q),
            "p": None if p in ("none", "없음") else int(p),
            "대기": int(대기.group(1)) if 대기 else 0,
            "후보화면": "의 후보가" in r.stdout,
            "rc": r.returncode,
        })
    return rows

def p95(xs):
    xs = sorted(xs)
    return xs[max(0, int(round(0.95 * len(xs))) - 1)] if xs else None

전체 = 등간격(syms)
대기만 = 등간격([s for s in syms if s["id"] in targets])
합격 = True
for 이름, 표본 in (("① 전체 심볼", 전체), ("② 승인 대기가 실리는 심볼만", 대기만)):
    rows = 잰다(표본)
    qs = [r["q"] for r in rows if r["q"] is not None]
    ps = [r["p"] for r in rows if r["p"] is not None]
    qp95, pp95 = p95(qs), p95(ps)
    대기_호출 = sum(1 for r in rows if r["대기"] >= 1)
    후보 = sum(1 for r in rows if r["후보화면"])
    실패 = sum(1 for r in rows if r["rc"] != 0)
    print(f"## 표본 {이름}")
    print()
    print(f"- 모집단 {len(syms) if 이름.startswith('①') else len([s for s in syms if s['id'] in targets])} · 표본 {len(rows)} · 질의 시간이 찍힌 호출 {len(qs)}")
    print(f"- 질의 시간 p95 **{qp95/1000:.1f} ms** · 중앙 {statistics.median(qs)/1000:.1f} ms" if qs else "- 질의 시간이 안 찍혔다")
    print(f"- 프로세스 시간 p95 {pp95/1000:.1f} ms (기록만 · 합격선 아님)" if ps else "- 프로세스 시간이 안 찍혔다")
    print(f"- 대기 구역이 1 건 이상 실린 호출 {대기_호출} · 후보 화면으로 끝난 호출 {후보} · rc≠0 {실패}")
    선 = qs and qp95 < 500_000 and len(qs) == len(rows) and 후보 == 0 and 실패 == 0
    if 이름.startswith("②"):
        선 = 선 and 대기_호출 == len(rows)
    print(f"- 판정: {'선 안' if 선 else '선 밖'}")
    print()
    합격 = 합격 and bool(선)
print(f"## 합: {'통과' if 합격 else '반증'}")
sys.exit(0 if 합격 else 1)
PYEOF
