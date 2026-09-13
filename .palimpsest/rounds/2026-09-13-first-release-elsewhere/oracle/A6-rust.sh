#!/usr/bin/env bash
# `A6` ⑵ — Rust 회귀 0: 착수 바이너리와 새 바이너리가 같은 트리에서 **같은 파일 간 엣지 집합**을 내는가.
#
# 둘에게 캐시·인덱스 자리를 **따로** 준다 — 같은 자리를 공유하면 뒤의 것이 앞의 산출을 읽는다.
# 수가 아니라 `(from, to)` 쌍의 집합을 댄다 — 하나를 잃고 하나를 얻어도 수는 같다.
#
# 음성 대조: 한쪽만 파일 간 엣지 집합이 다른 앞 커밋을 보게 하면 **달라야** 한다.
#
# 사용: A6-rust.sh <착수 pal> <새 pal> <저장소> <작업 디렉터리> <기준 커밋> <음성 대조 커밋>
set -euo pipefail
START="$1"; NEW="$2"; REPO="$3"; WORK="$4"; AT="$5"; OTHER="$6"
rm -rf "$WORK"; mkdir -p "$WORK"

dump() { # dump <바이너리> <자리 이름> <커밋>
  "$1" query graph.dump --json --repo "$REPO" --at "$3" \
    --cache-dir "$WORK/$2/cache" --index "$WORK/$2/index.redb" --intent "$WORK/$2/intent.redb" \
    > "$WORK/$2.json"
}

echo "# A6 ⑵ — 파일 간 엣지 집합 대조"
echo
echo "- 착수: \`$("$START" --version)\` · 새: \`$("$NEW" --version)\`"
echo "- 저장소: \`$(basename "$REPO")\` · 기준 커밋 \`$AT\` · 음성 대조 커밋 \`$OTHER\`"
echo

dump "$START" start "$AT"
dump "$NEW" new "$AT"
dump "$NEW" negative "$OTHER"

python3 - "$WORK" <<'PYEOF'
import json, sys
work = sys.argv[1]
def 집합(name):
    d = json.load(open(f"{work}/{name}.json", encoding="utf-8"))["answer"]
    paths = {n["id"]: n["path"] for n in d["nodes"]}
    return {(e["from"], e["to"]) for e in d["edges"] if e["from"] in paths and e["to"] in paths and paths[e["from"]] != paths[e["to"]]}
s, n, neg = 집합("start"), 집합("new"), 집합("negative")
print(f"- 착수 바이너리 파일 간 엣지 {len(s)} · 새 바이너리 {len(n)} · 착수에만 {len(s - n)} · 새에만 {len(n - s)}")
print(f"- 판정 ⑵: {'같다' if s == n else '다르다'}")
print(f"- 음성 대조(새 바이너리 · 다른 커밋) 엣지 {len(neg)} · 기준과 {'같다 — 비교가 차이를 못 잡는다' if neg == n else '다르다 — 비교가 차이를 잡는다'}")
sys.exit(0 if s == n and neg != n else 1)
PYEOF
