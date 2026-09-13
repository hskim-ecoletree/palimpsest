#!/usr/bin/env bash
# `E1` — 효과 확인의 걸음 순서가 커밋으로 서는가.
#
# ⑴ 봉인 ⑵ 설치·인입 ⑶ touch ⑷ 읽은 줄 ⑸ 변경 ⑹ 차이 — 각 산출이 **서로 다른 커밋**에 처음
# 들어가고 앞 것이 뒤 것의 **진조상**이다. 그리고 **변경을 touch 뒤에 했다는 앵커**:
# ⑶ 의 `head.txt` 가 적은 복제본 HEAD 가 변경 A 의 부모이고, 그때 `src tests scripts` 변경이 0 이다.
#
# 사용: E1-order.sh <회차 디렉터리(저장소 상대)> <복제본>
#       E1-order.sh --self-test     음성 대조 — 두 산출이 한 커밋 · porcelain 이 비지 않음 · 부모가 다름
set -uo pipefail

check_pairs() { # check_pairs sha1 sha2 ... — 이웃 쌍마다 SHA 다름 + 진조상
  local fail=0 prev=""
  for sha in "$@"; do
    if [ -n "$prev" ]; then
      if [ "$prev" = "$sha" ]; then echo "  ✗ $prev = $sha — 한 커밋에 들었다"; fail=1
      elif ! git merge-base --is-ancestor "$prev" "$sha" 2>/dev/null; then echo "  ✗ $prev 가 $sha 의 조상이 아니다"; fail=1
      else echo "  ✓ $prev → $sha"; fi
    fi
    prev="$sha"
  done
  return $fail
}

check_anchor() { # check_anchor <touch 때 HEAD> <porcelain 줄 수> <변경 A 의 부모>
  local fail=0
  [ "$2" = "0" ] || { echo "  ✗ touch 때 src·tests·scripts 변경이 $2 줄이다"; fail=1; }
  [ "$1" = "$3" ] || { echo "  ✗ 변경 A 의 부모 $3 ≠ touch 때 HEAD $1"; fail=1; }
  [ $fail -eq 0 ] && echo "  ✓ 변경 A 의 부모가 touch 때 HEAD 이고 그때 변경이 0 이다"
  return $fail
}

if [ "${1:-}" = "--self-test" ]; then
  head=$(git rev-parse HEAD); parent=$(git rev-parse HEAD^)
  ok=0
  echo "## 음성 대조 ① 두 산출이 한 커밋"; check_pairs "$parent" "$head" "$head" && { echo "  ✗ 안 걸렸다"; ok=1; } || echo "  → 걸렸다"
  echo "## 음성 대조 ② porcelain 이 비지 않음"; check_anchor aaa 1 aaa && { echo "  ✗ 안 걸렸다"; ok=1; } || echo "  → 걸렸다"
  echo "## 음성 대조 ③ 부모가 다름"; check_anchor aaa 0 bbb && { echo "  ✗ 안 걸렸다"; ok=1; } || echo "  → 걸렸다"
  exit $ok
fi

R="$1"; CLONE="$2"
first_add() { git log --diff-filter=A --format=%H -- "$1" | tail -1; }
echo "# E1 — 걸음 순서"
echo
declare -a names=(seal install touch readnote change delta)
declare -a paths=("$R/effect/00-seal.md" "$R/effect/01-install.txt" "$R/effect/03-touch/head.txt" "$R/effect/04-readnote.md" "$R/effect/05-change.txt" "$R/effect/06-delta.md")
shas=()
for i in "${!paths[@]}"; do
  s=$(first_add "${paths[$i]}")
  echo "- ${names[$i]} \`${paths[$i]#$R/}\` → \`${s:0:12}\`"
  shas+=("$s")
done
echo
echo "## 이웃 쌍"
check_pairs "${shas[@]}"; r1=$?
echo
echo "## 앵커"
touch_head=$(sed -n 1p "$R/effect/03-touch/head.txt" | awk '{print $2}')
porcelain=$(sed -n 2p "$R/effect/03-touch/head.txt" | awk '{print $NF}')
a=$(grep -m1 "^- A " "$R/effect/05-change.txt" | grep -o '[0-9a-f]\{40\}' | head -1)
a_parent=$(git -C "$CLONE" rev-parse "$a^")
check_anchor "$touch_head" "$porcelain" "$a_parent"; r2=$?
echo
echo "## 판정: $( [ $r1 -eq 0 ] && [ $r2 -eq 0 ] && echo 통과 || echo 반증)"
exit $(( r1 + r2 ))
