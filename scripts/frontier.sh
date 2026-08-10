#!/usr/bin/env bash
# 지금 착수 가능한 것을 센다 — 열려 있고, 열린 차단자가 없고, 아무도 잡지 않은 이슈.
#
# 상태를 복제하지 않는다. 이슈에서 매번 계산한다. 그래서 이 스크립트는
# 낡을 수 없다 (계획 README §7.4).
#
#   scripts/frontier.sh          착수 가능한 것만
#   scripts/frontier.sh --all    막힌 것까지, 무엇에 막혀 있는지와 함께
set -euo pipefail

REPO=$(gh repo view --json nameWithOwner --jq .nameWithOwner)
ALL=${1:-}

# 열린 이슈의 번호·제목·라벨·담당자를 한 번에 가져온다.
open=$(gh issue list --repo "$REPO" --state open --limit 200 \
  --json number,title,labels,assignees \
  --jq '.[] | [.number, (.labels|map(.name)|join(",")), (.assignees|length), .title] | @tsv')

# 이슈별 blocked_by 개수는 개별 조회만 가능하다. 병렬로 친다.
deps=$(echo "$open" | cut -f1 | xargs -P 8 -I{} sh -c \
  'printf "%s\t%s\n" "{}" "$(gh api repos/'"$REPO"'/issues/{} --jq ".issue_dependencies_summary.blocked_by // 0")"')

join_dep() { echo "$deps" | awk -F'\t' -v n="$1" '$1==n {print $2}'; }

ready=(); blocked=(); claimed=()
while IFS=$'\t' read -r num labels nassignee title; do
  [ -z "${num:-}" ] && continue
  b=$(join_dep "$num")
  mark=""
  case ",$labels," in *,epic,*) mark=" (에픽 — 하위 티켓의 지붕)";; esac
  if [ "${b:-0}" != "0" ]; then
    blocked+=("  #$num  $title  ← 차단자 $b")
  elif [ "$nassignee" != "0" ]; then
    claimed+=("  #$num  $title  ← 이미 잡힘")
  else
    ready+=("  #$num  $title$mark")
  fi
done <<< "$open"

echo "── 지금 착수 가능 ──────────────────────────────"
printf '%s\n' "${ready[@]:-  (없다)}"

if [ -n "${claimed[*]:-}" ]; then
  echo
  echo "── 진행 중 ────────────────────────────────────"
  printf '%s\n' "${claimed[@]}"
fi

if [ "$ALL" = "--all" ] && [ -n "${blocked[*]:-}" ]; then
  echo
  echo "── 막혀 있음 ──────────────────────────────────"
  printf '%s\n' "${blocked[@]}"
fi

echo
echo "열린 이슈 $(echo "$open" | wc -l | tr -d ' ')건 · 착수 가능 ${#ready[@]}건 · 막힘 ${#blocked[@]}건"
echo "지형은 docs/plan/README.md · 착수는 gh issue edit <번호> --add-assignee @me"
