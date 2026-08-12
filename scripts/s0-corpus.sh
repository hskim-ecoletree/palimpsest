#!/usr/bin/env bash
# S0 대조 코퍼스를 고정 SHA 에서 꺼낸다 — 1,122 개의 `.kt`
#
# **경로가 아니라 (remote, SHA) 가 코퍼스의 정체다**(corpus/manifest.toml [corpus.pin]).
# 이 스크립트는 그 SHA 에서 작업본을 만든다. 파서가 파일을 읽어야 하기 때문이다.
#
# 사용:  ./scripts/s0-corpus.sh <출력_디렉터리> [클론_루트]
#   클론_디렉터리 기본값은 ~/dev/projects/boxwood-workspace
#   SHA 를 가진 클론이 없으면 manifest 의 remote 에서 먼저 클론해야 한다:
#     git clone --no-single-branch <remote>
set -euo pipefail

OUT="${1:?사용: $0 <출력_디렉터리> [클론_루트]}"
ROOT="${2:-$HOME/dev/projects/boxwood-workspace}"

# corpus/manifest.toml [corpus.pin].kt_bearing 의 셋. 합이 1,122 이다.
#   저장소_별칭 : 클론_경로 : SHA : 기대_파일수
PINS=(
  "portal-backend:portal-backend:a29cad0bf6a8:671"
  "portal-backend-aa-task:portal-backend:10185f804ad8:434"
  "boxwood-packages:boxwood-packages:2e9198716796:17"
)

rm -rf "$OUT"
mkdir -p "$OUT"

total=0
for pin in "${PINS[@]}"; do
  IFS=: read -r alias clone sha want <<<"$pin"
  repo="$ROOT/$clone"
  [ -d "$repo/.git" ] || { echo "없다: $repo" >&2; exit 1; }
  git -C "$repo" cat-file -e "${sha}^{commit}" 2>/dev/null \
    || { echo "SHA 도달 불가: $clone @ $sha" >&2; exit 1; }

  mkdir -p "$OUT/$alias"
  git -C "$repo" archive "$sha" | tar -x -C "$OUT/$alias" --include='*.kt'

  got=$(find "$OUT/$alias" -name '*.kt' | wc -l | tr -d ' ')
  [ "$got" = "$want" ] || { echo "$alias: $got 개 — 기대 $want" >&2; exit 1; }
  printf '%-24s %4s  @%s\n' "$alias" "$got" "$sha"
  total=$((total + got))
done

echo "─────────────────────────────────"
printf '%-24s %4s  (기대 1122)\n' "합계" "$total"
[ "$total" = "1122" ] || { echo "합계 불일치" >&2; exit 1; }
