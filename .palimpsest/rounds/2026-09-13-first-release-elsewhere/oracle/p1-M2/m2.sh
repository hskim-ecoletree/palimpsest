#!/usr/bin/env bash
# M2 측정 — 분류 규칙 두 변형에서 A5 ⑷ 재현율 대조를 다시 돈다. 커밋하지 않는다.
set -uo pipefail
S=<scratch>
REPO=~/dev/projects/palimpsest
W=$S/wt-m2
FIVE="src/core/codeql-edges.ts src/core/static-check.ts src/core/codeql-analyzer.ts src/core/memory-query.ts src/core/mode-doctor.ts"
for v in git8000 textsrc; do
  echo "=== $v ==="
  git -C $W checkout -q -- crates/pal-extract/src/classify.rs
  python3 - "$W/crates/pal-extract/src/classify.rs" "$v" <<'PY'
import sys
p,v=sys.argv[1],sys.argv[2]; t=open(p,encoding='utf-8').read()
old="    if source.contains(&0) {\n        return plain(FileState::Binary"
new={"git8000":"    if source[..source.len().min(8000)].contains(&0) {\n        return plain(FileState::Binary",
     "textsrc":"    if source.contains(&0)\n        && !matches!(recognize(path.extension(), path.file_name(), declared, source), Recognition::FirstClass(_))\n    {\n        return plain(FileState::Binary"}[v]
assert t.count(old)==1; open(p,'w',encoding='utf-8').write(t.replace(old,new))
PY
  (cd $W && cargo build --release -q -p pal-cli) || { echo "빌드 실패 $v"; continue; }
  PAL=$W/target/release/pal
  C=$S/ditto-m2-$v; rm -rf "$C"; git clone -q ~/dev/projects/ditto "$C"; git -C "$C" checkout -q aded7ce
  (cd "$C" && $PAL touch resolveRepoRootForCreate > $S/m2-$v-touch.txt 2>&1)
  for f in $FIVE; do echo "symbols $f: $(cd "$C" && $PAL symbols $f 2>&1 | grep -c .)"; done
  node $REPO/.palimpsest/rounds/2026-09-13-first-release-elsewhere/oracle/ts-oracle.mjs --repo="$C" --pal=$PAL --unresolved --recall > $S/m2-$v.txt 2>&1; echo "oracle rc=$?"
  grep -E '불일치|모집단|빠진|outside_repo' $S/m2-$v.txt | head -8
done
git -C $W checkout -q -- crates/pal-extract/src/classify.rs
echo done
