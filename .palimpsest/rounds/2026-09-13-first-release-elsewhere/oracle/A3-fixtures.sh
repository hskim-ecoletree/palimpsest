#!/usr/bin/env bash
# `A3` · `A3-a` — TS 픽스처에서 해소 순서와 모드가 TypeScript 와 같은가.
#
# 픽스처는 `crates/pal-cli/tests/ts_cross_file.rs` 의 `A1` · `A2` 칸(⑴~⑷ · ⑹)을 그대로 다시 뜨고,
# 확장자·디렉터리 충돌과 `nodenext` ESM 픽스처를 더한다. 각 픽스처에서 `pal touch 씀` 으로
# 2층을 세운 뒤 `ts-oracle.mjs` 로 댄다. 가지별 표본은 픽스처 전부를 합쳐 본다.
#
# 사용: A3-fixtures.sh <pal 바이너리> <작업 디렉터리>
set -euo pipefail
PAL="$1"; WORK="$2"
HERE="$(cd "$(dirname "$0")" && pwd)"
rm -rf "$WORK"; mkdir -p "$WORK"

fixture() { # fixture <이름> then files via heredoc lines "경로<TAB>본문(\n 이스케이프)"
  local d="$WORK/$1"; mkdir -p "$d"
  while IFS=$'\t' read -r p body; do
    [ -z "$p" ] && continue
    mkdir -p "$d/$(dirname "$p")"
    printf '%b' "$body" > "$d/$p"
  done
  (cd "$d" && git init -q . && git add -A && git -c user.email=t@example.com -c user.name=t commit -qm init)
}

fixture a1 <<'EOF'
tsconfig.json	{"compilerOptions":{"moduleResolution":"bundler","baseUrl":".","paths":{"~/*":["./src/*"]}}}
src/a.ts	export function fromA() { return 1; }\n
src/b/c.ts	export function fromC() { return 2; }\n
src/x.ts	export function fromX() { return 3; }\n
src/y.ts	export function fromY() { return 4; }\n
src/dir/index.ts	export function fromDir() { return 5; }\n
src/alias/target.ts	export function fromAlias() { return 6; }\n
src/user/sib.ts	export function fromSib() { return 7; }\n
src/user/use.ts	import { fromSib } from './sib';\nimport { fromA } from '../a';\nimport { fromC } from '../b/c';\nimport { fromX } from '../x.js';\nimport { fromY } from '../y.ts';\nimport { fromDir } from '../dir';\nimport { fromAlias } from '~/alias/target';\nexport function 씀() { return fromSib() + fromA() + fromC() + fromX() + fromY() + fromDir() + fromAlias(); }\n
EOF

fixture a2-1 <<'EOF'
tsconfig.json	{\n  // 주석\n  "compilerOptions": {\n    /* 블록 */ "moduleResolution": "bundler", "baseUrl": ".",\n    "paths": { "@app/*": ["src/*",], },\n  },\n}\n
src/m.ts	export function 모듈() { return 1; }\n
use.ts	import { 모듈 } from '@app/m';\nexport function 씀() { return 모듈(); }\n
EOF

fixture a2-2 <<'EOF'
tsconfig.json	{"extends":"./configs/mid.json"}
configs/mid.json	{"extends":"./base"}
configs/base.json	{"compilerOptions":{"moduleResolution":"bundler","baseUrl":"..","paths":{"@lib/*":["lib/*"]}}}
lib/util.ts	export function 유틸() { return 1; }\n
use.ts	import { 유틸 } from '@lib/util';\nexport function 씀() { return 유틸(); }\n
EOF

fixture a2-3 <<'EOF'
tsconfig.json	{"compilerOptions":{"moduleResolution":"bundler","paths":{"#src/*":["./src/*"]}}}
src/m.ts	export function 모듈() { return 1; }\n
other/use.ts	import { 모듈 } from '#src/m';\nexport function 씀() { return 모듈(); }\n
EOF

fixture a2-4 <<'EOF'
tsconfig.json	{"compilerOptions":{"moduleResolution":"bundler","paths":{"@/*":["./outer/*"]}}}
pkg/tsconfig.json	{"compilerOptions":{"moduleResolution":"bundler","paths":{"@/*":["./inner/*"]}}}
outer/a.ts	export function 둘() { return 1; }\n
pkg/inner/a.ts	export function 둘() { return 2; }\n
pkg/src/use.ts	import { 둘 } from '@/a';\nexport function 씀() { return 둘(); }\n
EOF

fixture a2-6 <<'EOF'
configs/base.json	{"compilerOptions":{"moduleResolution":"bundler","paths":{"@x/*":["./x/*"]}}}
app/tsconfig.json	{"extends":"../configs/base.json"}
configs/x/m.ts	export function 모듈() { return 1; }\n
app/x/m.ts	export function 모듈() { return 2; }\n
app/use.ts	import { 모듈 } from '@x/m';\nexport function 씀() { return 모듈(); }\n
EOF

# 충돌 — 같은 이름의 파일·디렉터리·선언 파일·JSX 파일이 함께 있다.
fixture collide <<'EOF'
tsconfig.json	{"compilerOptions":{"moduleResolution":"bundler","jsx":"preserve"}}
x.ts	export function 엑스() { return 1; }\n
x/index.ts	export function 엑스() { return 2; }\n
y.ts	export function 와이() { return 1; }\n
y.d.ts	export declare function 와이(): number;\n
z.ts	export function 제트() { return 1; }\n
z.tsx	export function 제트() { return 2; }\n
w.ts	export function 더블유() { return 1; }\n
onlydir/index.ts	export function 디렉터리() { return 1; }\n
comp.tsx	export function 컴포넌트() { return 1; }\n
types.d.ts	export declare function 선언만(): number;\n
use.ts	import { 엑스 } from './x';\nimport { 와이 } from './y';\nimport { 제트 } from './z';\nimport { 더블유 } from './w.js';\nimport { 디렉터리 } from './onlydir';\nimport { 컴포넌트 } from './comp';\nimport { 선언만 } from './types';\nexport function 씀() { return 엑스() + 와이() + 제트() + 더블유() + 디렉터리() + 컴포넌트() + 선언만(); }\n
EOF

# nodenext ESM — 확장자 없는 상대 경로는 TypeScript 가 안 푼다.
fixture nodenext <<'EOF'
tsconfig.json	{"compilerOptions":{"module":"nodenext","moduleResolution":"nodenext"}}
package.json	{"type":"module"}
src/x.ts	export function 붙임() { return 1; }\nexport function 안붙임() { return 2; }\n
src/use.ts	import { 붙임 } from './x.js';\nimport { 안붙임 } from './x';\nexport function 씀() { return 붙임() + 안붙임(); }\n
EOF

total=0
: > "$WORK/branches.txt"
for d in a1 a2-1 a2-2 a2-3 a2-4 a2-6 collide nodenext; do
  (cd "$WORK/$d" && "$PAL" touch 씀 >/dev/null)
  echo
  echo "################ 픽스처 $d"
  set +e
  node "$HERE/ts-oracle.mjs" --repo="$WORK/$d" --pal="$PAL" | tee "$WORK/$d.report"
  rc=${PIPESTATUS[0]}; set -e
  # 가지 표본 줄만 — 머리의 「- 모드: 정밀도만」 도 같은 머리라 숫자로 끝나는 줄만 집는다.
  grep -E "^- (지정자|대상 확장자|모드): .* [0-9]+$" "$WORK/$d.report" >> "$WORK/branches.txt" || true
  echo "rc=$rc"; total=$((total + rc))
done

# `A3` 는 **픽스처 전체를 합친** 가지별 표본을 요구한다 — 픽스처마다 0 인 가지는 판정이 아니다.
echo
echo "################ 합산 — 가지별 표본(픽스처 전체)"
python3 - "$WORK/branches.txt" <<'PYEOF'
import sys, re, collections
합 = collections.Counter(); 순서 = []
for line in open(sys.argv[1], encoding="utf-8"):
    _, body = line.split(": ", 1)
    for part in body.strip().split(" · "):
        name, n = part.rsplit(" ", 1)
        if name not in 순서: 순서.append(name)
        합[name] += int(n)
print("- " + " · ".join(f"{k} {합[k]}" for k in 순서))
영 = [k for k in 순서 if 합[k] == 0]
print(f"- **합산 표본 0 인 가지**: {' · '.join(영) if 영 else '없음'}")
PYEOF

echo
echo "################ A3-a — 음성 대조(엣지 하나의 대상 파일을 바꿔 넣는다 · 불일치 ≥ 1 이어야 한다)"
set +e
node "$HERE/ts-oracle.mjs" --repo="$WORK/a1" --pal="$PAL" --corrupt | grep -E "잰 엣지|불일치|^  - "
echo "rc=$?"
set -e
echo
echo "합계 rc(불일치 있는 픽스처 수) = $total"
