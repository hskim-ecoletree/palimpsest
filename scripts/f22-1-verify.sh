#!/usr/bin/env bash
# ═════════════════════════════════════════════════════════════════════════════
# [f22.1] 의 음성 대조 — **이 검사가 고장 났다면 어떻게 드러나는가.**
#
# `cargo xtask check` 의 "스키마 정합" 은 정상 상태에서 `양방향 0건` 을 낸다. 그런데
# **아무것도 세지 않는 검사도 0 을 낸다.** 그래서 스키마를 한 자리씩 망가뜨리고
# 검사가 실제로 실패하는지를 본다. 하나라도 통과하면 그 항목은 측정이 아니라 장식이다.
#
# 판정 기록: docs/gates/F22-1-schema.md · 합격선 corpus/criteria.toml [f22.1]
# ═════════════════════════════════════════════════════════════════════════════
set -uo pipefail
cd "$(dirname "$0")/.."

SCHEMA=schema/graph.toml
DOC=docs/graph-schema.md
BACKUP=$(mktemp)
DOC_BACKUP=$(mktemp)
cp "$SCHEMA" "$BACKUP"
cp "$DOC" "$DOC_BACKUP"
restore() { cp "$BACKUP" "$SCHEMA"; cp "$DOC_BACKUP" "$DOC"; rm -f "$BACKUP" "$DOC_BACKUP"; }
trap restore EXIT

pass=0; fail=0

# 검사가 실패해야 정상인 경우
must_fail() {
  local name="$1"
  if cargo xtask check >/dev/null 2>&1; then
    printf '  ✗ %-44s 검사가 통과했다 — 이 자리는 세어지지 않는다\n' "$name"
    fail=$((fail+1))
  else
    printf '  ✓ %-44s 검사가 잡았다\n' "$name"
    pass=$((pass+1))
  fi
  cp "$BACKUP" "$SCHEMA"; cp "$DOC_BACKUP" "$DOC"
}

must_pass() {
  local name="$1"
  if cargo xtask check >/dev/null 2>&1; then
    printf '  ✓ %-44s 통과\n' "$name"
    pass=$((pass+1))
  else
    printf '  ✗ %-44s 성한 상태인데 실패했다\n' "$name"
    fail=$((fail+1))
  fi
}

echo
echo "── 성한 상태 ────────────────────────────────────────────────────────────"
must_pass "손대지 않은 스키마"

echo
echo "── 음성 대조 — 망가뜨리면 잡히는가 ──────────────────────────────────────"

# ① 노드 라벨 하나를 스키마에서 지운다 (코드에는 남아 있다)
python3 - <<'PY'
s=open('schema/graph.toml',encoding='utf-8').read()
i=s.index('[node.Binding]'); j=s.index('# ─────',i)
open('schema/graph.toml','w',encoding='utf-8').write(s[:i]+s[j:])
PY
must_fail "노드 \`Binding\` 을 스키마에서 지움"

# ② 엣지 타입 하나를 지운다
python3 - <<'PY'
s=open('schema/graph.toml',encoding='utf-8').read()
open('schema/graph.toml','w',encoding='utf-8').write(s[:s.index('[edge.BOUND_TO]')])
PY
must_fail "엣지 \`BOUND_TO\` 를 스키마에서 지움"

# ③ 코드에 없는 속성을 스키마에 더한다 (스키마 → 코드 방향)
python3 - <<'PY'
s=open('schema/graph.toml',encoding='utf-8').read()
s=s.replace('  { name = "span",     type = "span",               producer = "extractor", required = true },',
            '  { name = "span",     type = "span",               producer = "extractor", required = true },\n  { name = "owner",    type = "string",             producer = "extractor", required = true },')
open('schema/graph.toml','w',encoding='utf-8').write(s)
PY
must_fail "코드에 없는 속성 \`Symbol.owner\` 를 더함"

# ④ 코드에 있는 속성을 스키마에서 뺀다 (코드 → 스키마 방향)
python3 - <<'PY'
s=open('schema/graph.toml',encoding='utf-8').read()
s=s.replace('  { name = "span",     type = "span",               producer = "extractor", required = true },\n','')
open('schema/graph.toml','w',encoding='utf-8').write(s)
PY
must_fail "코드에 있는 \`Symbol.span\` 을 스키마에서 뺌"

# ⑤ 속성 출처 동질성을 깨뜨린다 — 로딩 시점 거부여야 한다
python3 - <<'PY'
s=open('schema/graph.toml',encoding='utf-8').read()
s=s.replace('{ name = "note",     type = "string",        producer = "human",          required = true }',
            '{ name = "note",     type = "string",        producer = "agent",          required = true }')
open('schema/graph.toml','w',encoding='utf-8').write(s)
PY
must_fail "asserted 노드에 agent 생산자를 섞음"

# ⑥ 엣지 공통 넷 중 하나를 뺀다
python3 - <<'PY'
s=open('schema/graph.toml',encoding='utf-8').read()
s=s.replace('snapshot    = "bound_at"\n','')
open('schema/graph.toml','w',encoding='utf-8').write(s)
PY
must_fail "엣지에서 공통 넷의 \`snapshot\` 을 뺌"

# ⑦ 자리만 만든 노드를 "값이 선다"고 적는다
python3 - <<'PY'
s=open('schema/graph.toml',encoding='utf-8').read()
s=s.replace('status     = "not_built"\nbuilt_by   = "F08"\n','')
open('schema/graph.toml','w',encoding='utf-8').write(s)
PY
must_fail "거주 불가 타입을 \`built\` 로 적음"

# ⑧ 파생된 문서 표를 손으로 고친다
python3 - <<'PY'
s=open('docs/graph-schema.md',encoding='utf-8').read()
open('docs/graph-schema.md','w',encoding='utf-8').write(s.replace('노드 라벨 **3개**','노드 라벨 **9개**'))
PY
must_fail "파생된 문서 표를 손으로 고침"

echo
echo "── 스키마 규모 ──────────────────────────────────────────────────────────"
python3 - <<'PY'
import tomllib
d = tomllib.load(open('schema/graph.toml','rb'))
n, e = d.get('node', {}), d.get('edge', {})
built = [k for k,v in n.items() if v.get('status','built') == 'built']
print(f"  노드 라벨 {len(n)}개 (값이 서는 것 {len(built)} · 자리만 {len(n)-len(built)})")
print(f"  엣지 타입 {len(e)}개")
print(f"  속성 {sum(len(v.get('attrs',[])) for v in n.values())}개")
PY

echo
if [ "$fail" -eq 0 ]; then
  echo "  음성 대조 $pass/$((pass+fail)) — **망가뜨린 자리가 전부 잡혔다**"
else
  echo "  음성 대조 실패 $fail 건 — 그 자리는 세어지지 않는다"
fi
exit "$fail"
