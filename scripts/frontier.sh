#!/usr/bin/env bash
# 지금 착수 가능한 것을 잰다 — 열려 있고, 열린 차단자가 없고, 아무도 잡지 않은 이슈.
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
#
# **빈 필드를 내보내지 않는다.** 탭은 IFS 공백이라 `read` 가 연속된 구분자를 하나로
# 합친다 — 라벨 없는 이슈에서 필드가 한 칸씩 밀려 담당자 칸에 제목이 들어가고,
# 그러면 **아무도 잡지 않은 이슈가 "이미 잡힘" 으로 나온다.** 이 스크립트는
# *"지금 어디에 서 있는가"* 에 답하는 자리이므로(AGENTS.md), 그 오보는 착수 가능한
# 것을 0 건으로 보이게 한다. `-` 는 라벨 없음을 뜻하고 아래 `case` 와 겹치지 않는다.
open=$(gh issue list --repo "$REPO" --state open --limit 200 \
  --json number,title,labels,assignees \
  --jq '.[] | [.number, ((.labels|map(.name)|join(","))|if . == "" then "-" else . end), (.assignees|length), .title] | @tsv')

# 이슈별 blocked_by 개수는 개별 조회만 가능하다. 병렬로 친다.
deps=$(echo "$open" | cut -f1 | xargs -P 8 -I{} sh -c \
  'printf "%s\t%s\n" "{}" "$(gh api repos/'"$REPO"'/issues/{} --jq ".issue_dependencies_summary.blocked_by // 0")"')

join_dep() { echo "$deps" | awk -F'\t' -v n="$1" '$1==n {print $2}'; }

# 순서표가 첫 항목으로 지목한 이슈. **번호를 여기 안 적는다** — 문서가 진다.
#
# 이 스크립트는 지금까지 열린 이슈를 전부 「착수 가능」으로 산출했다. 무엇이 먼저인지는
# 저장소가 답하지 않았고, 그것이 회차 2026-09-06-terrain-and-completion-scene 을 연
# 근거다. 이제 docs/plan/02-order.md 가 능력 실측에서 순서를 도출했고, 그 문서의
# §4 가 첫 항목을 이슈 번호로 지목한다. 여기서는 그 번호를 읽기만 한다.
ORDER_DOC="$(dirname "$0")/../docs/plan/02-order.md"
first=""
if [ -f "$ORDER_DOC" ]; then
  first=$(grep -oE '순서표의 1 번은 이슈 \[#[0-9]+\]' "$ORDER_DOC" \
          | grep -oE '#[0-9]+' | tr -d '#' | head -1) || true
fi

ready=(); blocked=(); claimed=(); head_line=""
while IFS=$'\t' read -r num labels nassignee title; do
  [ -z "${num:-}" ] && continue
  b=$(join_dep "$num")
  mark=""
  case ",$labels," in *,epic,*) mark=" (에픽 — 하위 티켓의 지붕)";; esac
  if [ "${b:-0}" != "0" ]; then
    blocked+=("  #$num  $title  ← 차단자 $b")
  elif [ "$nassignee" != "0" ]; then
    claimed+=("  #$num  $title  ← 이미 잡힘")
  elif [ -n "$first" ] && [ "$num" = "$first" ]; then
    head_line="  #$num  $title$mark  ← 순서표의 1 번"
  else
    ready+=("  #$num  $title$mark")
  fi
done <<< "$open"

echo "── 지금 착수 가능 ──────────────────────────────"
if [ -n "$head_line" ]; then
  printf '%s\n' "$head_line"
fi
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
n_ready=${#ready[@]}
[ -n "$head_line" ] && n_ready=$((n_ready + 1))
echo "열린 이슈 $(echo "$open" | wc -l | tr -d ' ')건 · 착수 가능 ${n_ready}건 · 교착 ${#blocked[@]}건"
if [ -z "$first" ]; then
  echo "⚠ 순서표가 첫 항목을 안 댄다 — docs/plan/02-order.md §4 를 본다"
elif [ -z "$head_line" ]; then
  # ★ **번호를 대는데 그 이슈가 목록에 없으면 그것도 신호다** (독립 리뷰 R3).
  #   앞 판은 이 자리에서 아무 말도 안 했다 — 첫 항목이 닫히거나 담당자가 붙는 순간
  #   프론티어가 착수 전 상태(전부 평평)로 **조용히** 돌아갔다.
  echo "⚠ 순서표의 첫 항목 #${first} 이 착수 가능 목록에 없다 — 닫혔거나 이미 잡혔다. docs/plan/02-order.md §4 를 다시 본다"
fi
echo "착수는 gh issue edit <번호> --add-assignee @me   (순서는 docs/plan/02-order.md · 완성 장면은 docs/plan/01-completion-scenes.md)"
