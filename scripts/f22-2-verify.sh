#!/usr/bin/env bash
# ═════════════════════════════════════════════════════════════════════════════
# [f22.2] 의 음성 대조 — **붕괴 픽스처가 실제로 하중을 지는가.**
#
# 픽스처가 통과한다는 사실만으로는 부족하다. 선행 구현이 무너진 방식(생산자를 성분에서
# 뺀 것)을 **그대로 재현**해서 픽스처가 실패하는지를 본다. 실패하지 않으면 그 픽스처는
# 무엇도 지키지 않는 것이다.
#
# 판정 기록: docs/gates/F22-2-derived-id.md · 합격선 corpus/criteria.toml [f22.2]
# ═════════════════════════════════════════════════════════════════════════════
set -uo pipefail
cd "$(dirname "$0")/.."

SRC=crates/pal-core/src/derived.rs
BACKUP=$(mktemp)
cp "$SRC" "$BACKUP"
trap 'cp "$BACKUP" "$SRC"; rm -f "$BACKUP"' EXIT

pass=0; fail=0

run_test() { cargo test -p pal-core --lib -- "$1" --exact >/dev/null 2>&1; }

expect_break() {   # 성분을 뺐을 때 그 시험이 실패해야 한다
  local name="$1" test="$2"
  if run_test "$test"; then
    printf '  ✗ %-40s %s 가 여전히 통과한다 — 성분이 하중을 지지 않는다\n' "$name" "$test"
    fail=$((fail+1))
  else
    printf '  ✓ %-40s %s 가 무너졌다\n' "$name" "$test"
    pass=$((pass+1))
  fi
  cp "$BACKUP" "$SRC"
}

echo
echo "── 성한 상태 ────────────────────────────────────────────────────────────"
if cargo test -p pal-core --lib -- derived:: >/dev/null 2>&1; then
  n=$(cargo test -p pal-core --lib -- derived:: 2>/dev/null | grep -cE '^test derived::')
  printf '  ✓ 파생 정체성 시험 %s 건 전부 통과\n' "$n"
  pass=$((pass+1))
else
  printf '  ✗ 성한 상태에서 실패했다\n'; fail=$((fail+1))
fi

echo
echo "── 음성 대조 — 성분을 빼면 픽스처가 무너지는가 ──────────────────────────"

# M1 — **선행 구현이 무너진 그 방식.** 생산자를 성분에서 뺀다.
python3 - <<'PY'
s=open('crates/pal-core/src/derived.rs',encoding='utf-8').read()
s=s.replace('        field(&mut h, producer.name().as_bytes());\n','')
s=s.replace('''        match producer {
            Producer::Rule { at } => field(&mut h, at.as_bytes()),
            Producer::Provider { id } => field(&mut h, id.as_bytes()),
            _ => field(&mut h, b""),
        }
''','        let _ = producer;\n')
open('crates/pal-core/src/derived.rs','w',encoding='utf-8').write(s)
PY
expect_break "생산자를 성분에서 뺌 (연구 G §2 의 재현)" "derived::tests::같은_대상의_판정_셋이_세_노드로_선다"

# M2 — 출처를 성분에서 뺀다
python3 - <<'PY'
s=open('crates/pal-core/src/derived.rs',encoding='utf-8').read()
s=s.replace('        field(&mut h, provenance.name().as_bytes());\n','        let _ = provenance;\n')
open('crates/pal-core/src/derived.rs','w',encoding='utf-8').write(s)
PY
expect_break "출처를 성분에서 뺌" "derived::tests::출처만_달라도_다른_노드다"

# M3 — 재현 입력을 성분에서 뺀다
python3 - <<'PY'
s=open('crates/pal-core/src/derived.rs',encoding='utf-8').read()
s=s.replace('        repro_into(&mut h, repro);\n','        let _ = repro;\n')
open('crates/pal-core/src/derived.rs','w',encoding='utf-8').write(s)
PY
expect_break "재현 입력을 성분에서 뺌" "derived::tests::재현_입력만_달라도_다른_노드다"

# M4 — 대상 정렬을 없앤다 (붕괴의 **반대편** 고장)
python3 - <<'PY'
s=open('crates/pal-core/src/derived.rs',encoding='utf-8').read()
s=s.replace('        sorted.sort();\n        h.update(&u32::try_from(sorted.len())','        h.update(&u32::try_from(sorted.len())')
open('crates/pal-core/src/derived.rs','w',encoding='utf-8').write(s)
PY
expect_break "대상 정렬을 없앰" "derived::tests::대상의_순서는_정체성을_바꾸지_않는다"

# M5 — 길이 접두어를 없앤다
python3 - <<'PY'
s=open('crates/pal-core/src/derived.rs',encoding='utf-8').read()
s=s.replace('''fn field(h: &mut blake3::Hasher, bytes: &[u8]) {
    h.update(&u32::try_from(bytes.len()).unwrap_or(u32::MAX).to_le_bytes());
    h.update(bytes);
}''','''fn field(h: &mut blake3::Hasher, bytes: &[u8]) {
    h.update(bytes);
}''')
s=s.replace('''            for p in sorted {
                field(h, p.as_str().as_bytes());
            }''','''            for p in sorted {
                h.update(p.as_str().as_bytes());
            }''')
s=s.replace('            h.update(&u32::try_from(sorted.len()).unwrap_or(u32::MAX).to_le_bytes());\n            for p in sorted {','            for p in sorted {')
open('crates/pal-core/src/derived.rs','w',encoding='utf-8').write(s)
PY
expect_break "길이 접두어를 없앰" "derived::tests::목록의_경계가_없으면_다른_집합이_하나가_된다"

echo
echo "── 본문이 성분이 아니라는 것 — **구조로 강제된다** ──────────────────────"
# 본문 방향은 변이로 시험할 수 없다. 넣을 자리 자체가 없기 때문이고, 그것이 강제의 형태다.
if grep -A6 'pub fn compute(' "$SRC" | grep -qiE '\bbody\b'; then
  echo "  ✗ compute 의 시그니처에 본문이 있다"
  fail=$((fail+1))
else
  echo "  ✓ compute 의 인자에 본문이 없다 — 넣을 자리가 없는 것이 강제의 형태다"
  echo '    (같은 인자로 두 번 부르면 같은 id 라는 사실은 본문만_다른_둘은_한_노드다 가 센다)'
  pass=$((pass+1))
fi

echo
if [ "$fail" -eq 0 ]; then
  echo "  음성 대조 $pass/$((pass+fail)) — **성분 다섯이 전부 하중을 진다**"
else
  echo "  음성 대조 실패 $fail 건"
fi
exit "$fail"
