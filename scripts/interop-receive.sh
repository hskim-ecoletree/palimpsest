#!/usr/bin/env bash
# **다른 플랫폼이 놓고 커밋한 저장소를 받아서 판정한다** — `[f24]` 의 마지막 축.
#
# 재는 것 넷. **셋째가 이 스크립트의 심장이다.**
#
# | # | 재는 것 | 안 서면 무슨 뜻인가 |
# |---|---|---|
# | 1 | `git clone` 한 워킹트리가 **깨끗하다** | 커밋된 바이트와 체크아웃 바이트가 갈렸다 — 산출물에 기계 고유의 줄바꿈이 실렸다 |
# | 2 | **아무것도 안 하고** `doctor` 여섯이 빨갛지 않다 | 그 저장소는 **받는 사람에게 고장 난 채로 도착**했다 |
# | 3 | `install`·`update` 뒤 **추적 파일 diff 0** | 받는 쪽이 돌리기만 해도 남이 안 만든 diff 가 난다 — 팀에서 핑퐁이 시작된다 |
# | 4 | `uninstall` 뒤 사용자 바이트가 **그대로** | 왕복이 이동을 견디지 못했다 |
#
# ⚠ **`doctor` 검사 4(`PATH` 에 `pal` 이 있는가)만 예외로 둔다.** 그것은 저장소의
# 성질이 아니라 **이 기계의 준비 상태**이고, `PATH` 를 얹어서 초록으로 만든다.
set -euo pipefail

GOT="${1:?받은 자리를 인자로 주십시오}"
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

pal="$ROOT/target/debug/pal"
[ -x "$pal" ] || pal="$ROOT/target/debug/pal.exe"
[ -x "$pal" ] || { echo "pal 을 못 찾았다: $ROOT/target/debug/" >&2; exit 1; }
export PATH="$(dirname "$pal"):$PATH"

GOT="$(cd "$GOT" && pwd)"
WORK="$ROOT/interop-work"
rm -rf "$WORK"

# ★ **clone 으로 받는다.** 파일 복사가 아니라 git 이 워킹트리 바이트를 정하게 한다 —
# 그것이 `core.autocrlf` 가 실제로 걸리는 자리이고 이 측정의 대상이다.
git clone -q "$GOT" "$WORK"
cd "$WORK"
git config user.email t@e
git config user.name t

FAIL=0
bad() { echo "✘ $*"; FAIL=1; }
good() { echo "✔ $*"; }

echo "── 받은 것 ──────────────────────────────────────────────────"
git log --oneline | sed 's/^/  /'
echo "  core.autocrlf = $(git config core.autocrlf || echo '(설정 없음)')"

# ── 1. clone 한 워킹트리가 깨끗하다 ─────────────────────────────────────────
if [ -z "$(git status --porcelain)" ]; then
  good "clone 직후 워킹트리가 깨끗하다"
else
  bad "clone 만 했는데 워킹트리가 더럽다 — 커밋된 바이트와 체크아웃 바이트가 갈린다:"
  git status --porcelain | sed 's/^/    /'
  git diff | head -40 | sed 's/^/    /'
fi

# ── 2. 아무것도 안 하고 doctor ──────────────────────────────────────────────
echo "── ★ 아무것도 안 하고 doctor ────────────────────────────────"
"$pal" doctor --install --json > 진단.json 2>진단.err || true
cat 진단.json
REDS=$(grep -c '"outcome": *"failed"' 진단.json || true)
REDS=$(echo "$REDS" | tr -d ' \r\n')
if [ "$REDS" = "0" ]; then
  good "clone 만 하고 여섯 검사에 빨강이 없다"
else
  bad "clone 만 했는데 검사 $REDS 개가 빨갛다 — 그 저장소는 고장 난 채로 도착했다"
  cat 진단.err | sed 's/^/    /'
fi
rm -f 진단.json 진단.err

# ── 3. install·update 뒤 추적 파일 diff 0 ──────────────────────────────────
echo "── install · update ─────────────────────────────────────────"
"$pal" install --target .
"$pal" update --target .
if [ -z "$(git status --porcelain)" ]; then
  good "install·update 뒤에도 추적 파일 diff 가 0 이다"
else
  bad "받는 쪽이 돌리기만 했는데 diff 가 났다 — 팀에서 핑퐁이 시작되는 자리다:"
  git status --porcelain | sed 's/^/    /'
  git diff | head -60 | sed 's/^/    /'
fi

# ── 4. uninstall 뒤 사용자 바이트가 그대로 ─────────────────────────────────
#
# ★ **대상이 둘로 갈린다 — 그리고 그것은 게이트 ① 이 이미 정한 갈래다.**
#
# | 대상 | 보증 | 왜 |
# |---|---|---|
# | `README.md`·`CLAUDE.md`·`.gitignore` | **바이트 동일** | 블록을 넣고 빼는 자리다. 사용자 바이트는 접두사로 그대로 남아야 한다 |
# | `.claude/settings.json` | **모든 키·값이 그대로**(바이트 동일 아님) | JSON 을 파싱해서 병합하고 다시 쓰므로 **포맷이 정규화된다.** 게이트 ① 이 이것을 유일한 예외로 적었다 |
#
# ⚠ 실측(2026-08-17)이 그 갈래를 여기서 확인했다: `{"env": {"A": "1"}}` 가 왕복 뒤
# 여러 줄로 다시 쓰였다. **키도 값도 안 바뀌었고 공백만 바뀌었다.**
echo "── uninstall ────────────────────────────────────────────────"
"$pal" uninstall --target .
# 설치 **전** 커밋이 사용자가 쓴 전부다.
BASE=$(git rev-list --max-parents=0 HEAD)

if git diff --quiet "$BASE" -- README.md CLAUDE.md .gitignore; then
  good "왕복 뒤 블록 파일 셋이 설치 전과 **바이트 동일**하다"
else
  bad "왕복이 블록 파일의 사용자 바이트를 바꿨다:"
  git diff "$BASE" -- README.md CLAUDE.md .gitignore | head -60 | sed 's/^/    /'
fi

# `settings.json` — **공백만 빼고** 댄다. `jq` 는 이 저장소가 안 믿는 의존이고
# (runner 마다 있고 없고가 갈린다) 우리 fixture 의 JSON 은 문자열 안에 공백이 없으므로
# 공백 제거가 이 자리에서는 정확한 정규화다. 값이 바뀌면 여기서 걸린다.
BEFORE=$(git show "$BASE:.claude/settings.json" | tr -d ' \t\r\n')
AFTER=$(tr -d ' \t\r\n' < .claude/settings.json)
if [ "$BEFORE" = "$AFTER" ]; then
  good ' 왕복 뒤 settings.json 의 키·값이 설치 전과 같다(포맷만 정규화됐다)'
else
  bad ' 왕복이 settings.json 의 키나 값을 바꿨다:'
  echo "    전: $BEFORE"
  echo "    후: $AFTER"
fi

cd "$ROOT"
rm -rf "$WORK"

if [ "$FAIL" = "0" ]; then
  echo
  echo "상호운용 통과 — 한쪽이 놓은 것을 다른 쪽이 그대로 쓴다"
else
  echo
  echo "상호운용 실패 — 위의 ✘ 를 보십시오" >&2
fi
exit "$FAIL"
