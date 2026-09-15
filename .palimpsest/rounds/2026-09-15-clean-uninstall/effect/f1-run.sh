#!/usr/bin/env bash
# ═════════════════════════════════════════════════════════════════════════════
# F1 — **효과**: 남의 저장소에서 설치 → 사용 → 제거가 흔적을 남기는가.
#
# 회차 `2026-09-15-clean-uninstall` 완수 조건 F1(계획 7). 대상은 이 저장소가 아니라
# 실제 저장소 하나(`ditto` · TypeScript · ADR 있음)의 **새 클론**이고, 걸음은 둘이다 —
# 기본 uninstall 과 `--purge`. 걸음마다 새로 클론한다.
#
#   f1-run.sh <pal 바이너리> [<바이너리 꼬리표>]
#
#   f1-run.sh /path/to/target/release/pal round      이 회차가 빌드한 바이너리
#   f1-run.sh ./.palimpsest/bin/pal          start    착수 바이너리(v0.1.1) — RED
#
# rc: 0 통과 · 1 어긋남 · 2 못 쟀다(짐작하지 않고 멈춘다).
#
# 산출은 이 스크립트 옆에 `f1-<걸음>-<꼬리표>.txt` 로 붙는다 — 스냅샷 diff · uninstall 화면 · 판정.
#
# ── 스냅샷은 **조건의 정의 그대로** ────────────────────────────────────────────
#
# - 워킹트리: 저장소 루트 아래 전부의 경로 · 종류 · 모드 · 바이트.
#   `.git/` 은 빼되 **`.git/config` · `.git/info/exclude` 는 넣는다**(시험이 부른 git 이 `.git/index` 를 고쳐 쓴다).
# - HOME: 격리 `HOME` **전체**.
#
# ⚠ **`python3` 를 HOME 안에서 부르지 않는다.** 그 캐시(`Library/Caches/com.apple.python`)가
#   HOME 스냅샷에 끼어 걸음마다 거짓 갈림이 됐다(회차 `state.md` 의 실패한 접근).
#   그래서 이 스크립트는 `find` · `stat` · `shasum` 만 쓴다. 그 셋은 격리 HOME 을 안 받는다 —
#   `HOME` 을 바꿔 부르는 것은 `pal` 뿐이다.
#
# ⚠ **셸 변수 이름은 ASCII 다.** bash 는 비ASCII 식별자를 대입으로 안 읽고
#   `command not found` 로 넘어간다(이 회차 실측).
#
# ⚠ 경로에 줄바꿈이 있으면 스냅샷이 어긋난다 — 대상 저장소에는 없다(확인함).
# ═════════════════════════════════════════════════════════════════════════════
set -uo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

PAL="${1:-}"
TAG="${2:-}"
if [ -z "$PAL" ]; then
  echo "사용법: f1-run.sh <pal 바이너리> [<바이너리 꼬리표>]" >&2
  exit 2
fi
case "$PAL" in
  /*) ;;
  *) PAL="$(cd "$(dirname "$PAL")" && pwd)/$(basename "$PAL")" ;;
esac
[ -x "$PAL" ] || { echo "바이너리를 못 찾았다: $PAL" >&2; exit 2; }
[ -n "$TAG" ] || TAG="$("$PAL" --version 2>/dev/null | tr ' +' '--' | tr -cd 'A-Za-z0-9.-')"
[ -n "$TAG" ] || TAG=unknown

# ── 대상 저장소 ──────────────────────────────────────────────────────────────
#
# 로컬 경로에서 **새로 클론**한 뒤 origin 을 GitHub 주소로 맞춘다 — 프로젝트 식별자는
# `remote.origin.url` 의 해시라, 이렇게 해야 실사용과 같은 식별자가 된다.
SOURCE="${PAL_F1_SOURCE:-$HOME/dev/projects/ditto}"
ORIGIN="${PAL_F1_ORIGIN:-https://github.com/incognito050924/ditto.git}"
WANT_HEAD="${PAL_F1_HEAD:-aded7ce7}"

[ -d "$SOURCE/.git" ] || { echo "대상 저장소가 없다: $SOURCE" >&2; exit 2; }

SCRATCH="${TMPDIR:-/tmp}/pal-f1-$$"
mkdir -p "$SCRATCH" || exit 2
trap 'rm -rf "$SCRATCH"' EXIT

# ═════════════════════════════════════════════════════════════════════════════
# 스냅샷
# ═════════════════════════════════════════════════════════════════════════════

# snap <뿌리> <w|h> <산출 파일>
#   한 줄에 하나: `<상대 경로>\t<종류>\t<모드>\t<바이트 해시 또는 링크 대상>`
snap() {
  local root="$1" kind="$2" out="$3"
  local w="$SCRATCH/snapwork"
  rm -f "$w.paths" "$w.sorted" "$w.meta" "$w.hash"
  : > "$w.paths"
  if [ ! -e "$root" ]; then
    : > "$out"
    return 0
  fi
  if [ "$kind" = w ]; then
    find "$root" -path "$root/.git" -prune -o -print >> "$w.paths" 2>/dev/null
    local extra
    for extra in .git/config .git/info/exclude; do
      [ -f "$root/$extra" ] && printf '%s\n' "$root/$extra" >> "$w.paths"
    done
  else
    find "$root" -print >> "$w.paths" 2>/dev/null
  fi
  LC_ALL=C sort -u "$w.paths" > "$w.sorted"
  : > "$w.meta"
  tr '\n' '\0' < "$w.sorted" | xargs -0 stat -f '%N	%OLp	%HT	%Y' >> "$w.meta" 2>/dev/null
  : > "$w.hash"
  awk -F'\t' '$3 == "Regular File" {print $1}' "$w.meta" | tr '\n' '\0' \
    | xargs -0 shasum -a 256 >> "$w.hash" 2>/dev/null
  awk -F'\t' -v root="$root" '
    NR == FNR {
      h = substr($0, 1, 64); p = substr($0, 67); H[p] = h; next
    }
    {
      path = $1
      if (substr(path, 1, length(root)) != root) next
      rel = substr(path, length(root) + 2)
      if (rel == "") next
      if ($3 == "Directory")          { print rel "\tD\t" $2 "\t-" }
      else if ($3 == "Symbolic Link") { print rel "\tL\t-\t" $4 }
      else                            { print rel "\tF\t" $2 "\t" (H[path] == "" ? "?" : H[path]) }
    }
  ' "$w.hash" "$w.meta" | LC_ALL=C sort > "$out"
}

# 갈린 경로 — 한쪽에만 있거나 줄이 다른 것.
diff_paths() {
  awk -F'\t' '
    NR == FNR { A[$1] = $0; next }
    { if (!($1 in A) || A[$1] != $0) print $1; delete A[$1] }
    END { for (k in A) print k }
  ' "$1" "$2" | LC_ALL=C sort -u
}

# 한 경로의 종류(D · F · L) — 두 스냅샷 가운데 있는 쪽에서 읽는다.
type_of() {
  awk -F'\t' -v p="$1" '$1 == p { print $2; found = 1; exit } END { if (!found) print "?" }' "$2" "$3"
}

# ═════════════════════════════════════════════════════════════════════════════
# 허용 목록 `L`
# ═════════════════════════════════════════════════════════════════════════════
#
# 조건 머리말의 목록 그대로. 워킹트리 쪽에서 `.palimpsest` 자신은 **적힌 자리들의 부모
# 디렉터리**다 — 그것이 없으면 안의 자리가 남을 수 없다.
in_l_worktree() {
  case "$1" in
    .palimpsest|.palimpsest/.gitignore|.palimpsest/intent.redb) return 0 ;;
    .palimpsest/intent|.palimpsest/intent/*) return 0 ;;
    .palimpsest/rounds|.palimpsest/rounds/*) return 0 ;;
    *) return 1 ;;
  esac
}

# 밖의 `L` — 승인 · 봉인 `<digest>.json` 과 `<digest>.project` · 조상 기록 · 그 자리의 부모 디렉터리.
# `STORE_REL` 은 HOME 기준 상대 경로(macOS: `Library/Application Support/palimpsest/approvals`).
in_l_home() {
  local p="$1"
  local base="${STORE_REL%/approvals}"
  case "$p" in
    "$base/created-ancestors.json") return 0 ;;
    "$STORE_REL"/*.json|"$STORE_REL"/*.project) return 0 ;;
  esac
  # 부모 디렉터리 — 저장소 자리부터 HOME 까지.
  local q="$STORE_REL"
  while [ -n "$q" ] && [ "$q" != "." ] && [ "$q" != "/" ]; do
    [ "$p" = "$q" ] && return 0
    q="$(dirname "$q")"
  done
  return 1
}

# `L` 의 **정본 자리**인가 — 남으면 uninstall 직전 바이트와 같아야 하는 것.
# `.palimpsest/.gitignore` 는 여기 없다: uninstall 이 그때 두는 가림 파일이라 직전 바이트가 없다.
is_canon_worktree() {
  case "$1" in
    .palimpsest/intent.redb) return 0 ;;
    .palimpsest/intent/*|.palimpsest/rounds/*) return 0 ;;
    *) return 1 ;;
  esac
}
is_canon_home() {
  case "$1" in
    # 운영 상태(Stop 활성화 · 진행 파일 · 잠금)는 정본이 아니다 — 기본 uninstall 이 **걷는** 자리라
    # 직전 바이트와 같으면 오히려 어긋남이다. 밖의 정본은 명령 승인과 종료 봉인 둘뿐이다.
    "$STORE_REL"/round-stop-*) return 1 ;;
    "$STORE_REL"/*.json|"$STORE_REL"/*.project) return 0 ;;
    *) return 1 ;;
  esac
}

# uninstall 화면이 그 경로를 말했는가.
#   - 그 경로 자체가 화면에 있거나
#   - **자리**로 말한 윗 디렉터리(`.palimpsest/rounds/` 처럼 끝이 `/`)가 있거나.
#     제품은 남긴 정본을 자리와 파일 수로 말한다 — 그 말이 그 아래를 덮는다.
covered() {
  local p="$1" screen="$2"
  grep -qF -- "$p" "$screen" && return 0
  local q
  q="$(dirname "$p")"
  while [ "$q" != "." ] && [ "$q" != "/" ] && [ -n "$q" ]; do
    grep -qF -- "$q/" "$screen" && return 0
    q="$(dirname "$q")"
  done
  return 1
}

# ═════════════════════════════════════════════════════════════════════════════
# 한 걸음
# ═════════════════════════════════════════════════════════════════════════════

RC=0

# JSON 한 줄에서 문자열 값 하나.
jval() { sed -n "s/.*\"$1\":\"\([^\"]*\)\".*/\1/p" | head -1; }

step() {
  local mode="$1"                       # default · purge
  local out="$HERE/f1-$mode-$TAG.txt"
  local work="$SCRATCH/$mode"
  local repo="$work/repo" home="$work/home"
  local step_rc=0
  local note=""

  rm -rf "$work"
  mkdir -p "$home" || return 2

  {
    echo "═══════════════════════════════════════════════════════════════════════"
    echo "F1 — 효과 · 걸음 「${mode}」 · 바이너리 꼬리표 「${TAG}」"
    echo "═══════════════════════════════════════════════════════════════════════"
    echo
    echo "바이너리   : $PAL"
    echo "판              : $("$PAL" --version 2>&1 | head -1)"
    echo "대상 저장소     : $SOURCE  →  origin $ORIGIN"
    echo "돌린 때         : $(date -u '+%Y-%m-%dT%H:%M:%SZ')"
    echo "기계            : $(uname -s) $(uname -m)"
    echo
  } > "$out"

  # ── 새 클론 ────────────────────────────────────────────────────────────────
  if ! git clone -q "$SOURCE" "$repo" 2>>"$out"; then
    echo "못 쟀다 — 클론이 안 됐다" >> "$out"
    return 2
  fi
  git -C "$repo" remote set-url origin "$ORIGIN" 2>>"$out"
  git -C "$repo" config user.email "f1@example.invalid"
  git -C "$repo" config user.name "F1"
  local head
  head="$(git -C "$repo" rev-parse --short=8 HEAD)"
  {
    echo "클론 HEAD       : $head (기대 $WANT_HEAD)"
    echo "origin          : $(git -C "$repo" config --get remote.origin.url)"
    echo
  } >> "$out"
  case "$head" in
    "$WANT_HEAD"*) ;;
    *) echo "못 쟀다 — HEAD 가 기대와 다르다" >> "$out"; return 2 ;;
  esac

  # 저장소 자리(HOME 기준 상대) — macOS.
  STORE_REL="Library/Application Support/palimpsest/approvals"
  local store="$home/$STORE_REL"

  # `pal` 만 격리 HOME 을 받는다.
  p() { ( cd "$repo" && env -u PAL_APPROVAL_DIR HOME="$home" "$PAL" "$@" ); }

  # ── 설치 전 스냅샷 ─────────────────────────────────────────────────────────
  snap "$repo" w "$work/s0"
  snap "$home" h "$work/h0"
  echo "설치 전 스냅샷  : 워킹트리 $(wc -l < "$work/s0" | tr -d ' ') 자리 · HOME $(wc -l < "$work/h0" | tr -d ' ') 자리" >> "$out"
  echo >> "$out"

  # ── 흐름 ───────────────────────────────────────────────────────────────────
  echo "── 흐름 ──────────────────────────────────────────────────────────────" >> "$out"
  if ! p install > "$work/install.txt" 2>&1; then
    { echo "못 쟀다 — install rc≠0"; cat "$work/install.txt"; } >> "$out"
    return 2
  fi
  [ -f "$repo/.claude/pal/manifest.json" ] || { echo "못 쟀다 — 매니페스트가 안 생겼다" >> "$out"; return 2; }
  echo "  install         ok" >> "$out"

  # 이름이 하나뿐인 `src/` 아래 함수 — 결정론적으로 고른다(`pal bind` 는 후보가 여럿이면 멈춘다).
  p ledger "$repo" --symbols > "$work/symbols.jsonl" 2>"$work/symbols.err"
  local symbol
  symbol="$(sed -E 's/.*"path":"([^"]*)","container":\[[^]]*\],"name":"([^"]*)","kind":"([^"]*)".*/\3 \1 \2/' "$work/symbols.jsonl" \
    | awk '$1 == "function" && $2 ~ /^src\// {print $3}' | LC_ALL=C sort | uniq -c | awk '$1 == 1 {print $2}' | head -1)"
  [ -n "$symbol" ] || { echo "못 쟀다 — 이름이 하나뿐인 심볼을 못 골랐다" >> "$out"; return 2; }
  echo "  고른 좌표       $symbol" >> "$out"

  p touch "$symbol" > "$work/touch.txt" 2>&1 || { echo "못 쟀다 — touch rc≠0" >> "$out"; cat "$work/touch.txt" >> "$out"; return 2; }
  [ -f "$repo/.palimpsest/index.redb" ] || { echo "못 쟀다 — index.redb 가 안 생겼다" >> "$out"; return 2; }
  echo "  touch           ok" >> "$out"

  p narrative > "$work/narrative.txt" 2>&1 || { echo "못 쟀다 — narrative rc≠0" >> "$out"; cat "$work/narrative.txt" >> "$out"; return 2; }
  [ -f "$repo/.palimpsest/narrative-pending.json" ] || { echo "못 쟀다 — narrative-pending.json 이 안 생겼다" >> "$out"; return 2; }
  echo "  narrative       ok · $(sed -n 's/^  문서 \(.*\)$/\1/p' "$work/narrative.txt" | head -1)" >> "$out"

  p bind "$symbol" --note "F1 효과 실측이 건 조각 — 이 좌표에 결정을 결박한다" > "$work/bind.txt" 2>&1 \
    || { echo "못 쟀다 — bind rc≠0" >> "$out"; cat "$work/bind.txt" >> "$out"; return 2; }
  [ -f "$repo/.palimpsest/intent.redb" ] || { echo "못 쟀다 — intent.redb 가 안 생겼다" >> "$out"; return 2; }
  echo "  bind            ok · 결박 하나" >> "$out"

  # 회차 둘 — `report.md` 가 있는 회차는 Reported 라 Stop 활성화가 거부된다.
  # 그래서 봉인할 회차와 Stop 을 거는 열린 회차를 갈라 둔다.
  local slug_open=open-round slug_sealed=sealed-round
  local d
  for d in "$slug_open" "$slug_sealed"; do
    mkdir -p "$repo/.palimpsest/rounds/$d"
    printf '# fixture\n\n## 완수 조건\n\n- [ ] A1 condition A1\n' > "$repo/.palimpsest/rounds/$d/intent.md"
    {
      printf '{"kind":"schema","version":3,"round":"%s"}\n' "$d"
      printf '{"kind":"oracle","id":"A1","mode":"command","check":"echo ROUND_OK","expect":{"literal":"ROUND_OK"},"cwd":"."}\n'
    } > "$repo/.palimpsest/rounds/$d/verification.log"
  done
  printf '# report\n\n## 남지 않은 것\n없음.\n\n## 다음 회차가 받는 것\n없음.\n\n## 범위 밖\n없음.\n\n## 원리상 못 잰 것\n없음.\n\n## 능력 부재\n없음.\n' \
    > "$repo/.palimpsest/rounds/$slug_sealed/report.md"
  printf '{"schema_version":3,"종류":"레코드","회차":"%s"}\n' "$slug_sealed" \
    > "$repo/.palimpsest/rounds/$slug_sealed/findings.jsonl"

  local approve_json verify_json final_json enable_json
  approve_json="$(p round approve --round "$slug_sealed" --id A1 --json 2>&1)"
  local approval_digest
  approval_digest="$(printf '%s' "$approve_json" | jval approval_digest)"
  [ -n "$approval_digest" ] || { echo "못 쟀다 — approve 가 승인 digest 를 안 돌려줬다: $approve_json" >> "$out"; return 2; }
  echo "  round approve   ok · $approval_digest" >> "$out"

  verify_json="$(p round verify --round "$slug_sealed" --id A1 --json 2>&1)"
  case "$verify_json" in
    *'"met":true'*) echo "  round verify    met" >> "$out" ;;
    *) echo "못 쟀다 — verify 가 met 이 아니다: $verify_json" >> "$out"; return 2 ;;
  esac

  final_json="$(p round verify --round "$slug_sealed" --all --json 2>&1)"
  case "$final_json" in
    *'"completion":"complete"'*) echo "  종료 봉인       complete" >> "$out" ;;
    *) echo "못 쟀다 — 종료 봉인이 안 됐다: $final_json" >> "$out"; return 2 ;;
  esac
  local seal_digest
  seal_digest="$(grep -o '"finalization_seal":"[0-9a-f]*"' "$repo/.palimpsest/rounds/$slug_sealed/verification.log" \
    | tail -1 | sed 's/.*:"//; s/"//')"
  [ -n "$seal_digest" ] || { echo "못 쟀다 — 봉인 digest 를 원장에서 못 읽었다" >> "$out"; return 2; }

  enable_json="$(p round stop enable --round "$slug_open" --json 2>&1)"
  local activation_digest
  activation_digest="$(printf '%s' "$enable_json" | jval activation_digest)"
  [ -n "$activation_digest" ] || { echo "못 쟀다 — stop enable 이 활성화 digest 를 안 돌려줬다: $enable_json" >> "$out"; return 2; }
  echo "  stop enable     ok · $activation_digest" >> "$out"

  # 읽을 수 있는 transcript 로 Stop 훅 — 진행 파일을 쓰는 자리(차단)까지 간다.
  printf 'F1 효과 실측\n' > "$work/transcript.jsonl"
  local hook_out
  hook_out="$(printf '{"session_id":"f1","transcript_path":"%s","cwd":"%s","hook_event_name":"Stop","stop_hook_active":false,"last_assistant_message":"done"}' \
    "$work/transcript.jsonl" "$repo" | p hook Stop 2>&1)"
  case "$hook_out" in
    *'"block"'*) echo "  Stop 훅         차단(진행 파일을 쓰는 자리까지)" >> "$out" ;;
    *) echo "못 쟀다 — Stop 훅이 차단까지 가지 않았다: $hook_out" >> "$out"; return 2 ;;
  esac

  # ── 걸음마다 대상이 생겼음을 uninstall 전에 단언한다 ───────────────────────
  # **어느 판이든 써야 하는 것** — 없으면 흐름이 안 돌았다는 뜻이라 못 쟀다.
  local missing=""
  local f
  for f in "$store/$approval_digest.json" \
           "$store/$seal_digest.json" \
           "$store/round-stop-progress-$activation_digest.json" \
           "$store/round-stop-progress-$activation_digest.lock"; do
    [ -e "$f" ] || missing="$missing
    $f"
  done
  if [ -n "$missing" ]; then
    echo "못 쟀다 — 생겼어야 할 밖의 기록이 없다:$missing" >> "$out"
    return 2
  fi
  echo "  밖의 기록       승인 · 봉인 · 진행 · 잠금 생겼다" >> "$out"
  # **이 회차가 더한 것** — 표시 파일과 조상 기록은 착수 바이너리가 안 쓴다.
  # 그것을 전제로 걸면 착수 바이너리가 RED 를 보이기 전에 멈춘다(실측). 있고 없음을 적기만 한다.
  local extra
  for extra in "$store/$approval_digest.project" "$store/$seal_digest.project" \
               "$store/round-stop-progress-$activation_digest.project" \
               "$home/Library/Application Support/palimpsest/created-ancestors.json"; do
    if [ -e "$extra" ]; then
      echo "  표시 · 조상     있다   ${extra#"$home/"}" >> "$out"
    else
      echo "  표시 · 조상     없다   ${extra#"$home/"}   (옛 판은 안 쓴다)" >> "$out"
    fi
  done
  echo >> "$out"

  # ── uninstall 직전 스냅샷 ─────────────────────────────────────────────────
  snap "$repo" w "$work/s1"
  snap "$home" h "$work/h1"

  # ── uninstall ─────────────────────────────────────────────────────────────
  local screen="$work/uninstall.txt"
  if [ "$mode" = purge ]; then
    p uninstall --purge > "$screen" 2>&1
  else
    p uninstall > "$screen" 2>&1
  fi
  local urc=$?
  snap "$repo" w "$work/s2"
  snap "$home" h "$work/h2"

  {
    echo "── uninstall 화면 ────────────────────────────────────────────────────"
    echo "명령: pal uninstall$([ "$mode" = purge ] && echo ' --purge')   rc=$urc"
    echo
    cat "$screen"
    echo
  } >> "$out"

  # ── 스냅샷 diff ───────────────────────────────────────────────────────────
  diff_paths "$work/s0" "$work/s2" > "$work/dw"
  diff_paths "$work/h0" "$work/h2" > "$work/dh"
  local nw nh
  nw="$(wc -l < "$work/dw" | tr -d ' ')"
  nh="$(wc -l < "$work/dh" | tr -d ' ')"
  # 목록은 한쪽에 이만큼까지만 적는다 — 착수 바이너리는 `.palimpsest/cache/` 아래만 2700 줄을
  # 남겨서, 다 적으면 산출물이 갈림의 **수**보다 목록으로 뒤덮인다. 수는 위가 정본이다.
  local cap=200
  {
    echo "── 스냅샷 diff (설치 전 ↔ uninstall 뒤) ──────────────────────────────"
    echo "워킹트리 갈림 $nw · HOME 갈림 $nh"
    echo "(목록은 한쪽에 $cap 줄까지 적는다 — 수는 위가 정본이다)"
    echo
    echo "  [워킹트리]"
    if [ "$nw" = 0 ]; then
      echo "    (없다)"
    else
      head -n "$cap" "$work/dw" | sed 's/^/    /'
      [ "$nw" -gt "$cap" ] && echo "    …$((nw - cap)) 줄 더"
    fi
    echo
    echo "  [HOME]"
    if [ "$nh" = 0 ]; then
      echo "    (없다)"
    else
      head -n "$cap" "$work/dh" | sed 's/^/    /'
      [ "$nh" -gt "$cap" ] && echo "    …$((nh - cap)) 줄 더"
    fi
    echo
  } >> "$out"

  # ── 판정 ──────────────────────────────────────────────────────────────────
  echo "── 판정 ──────────────────────────────────────────────────────────────" >> "$out"
  if [ "$mode" = purge ]; then
    # `--purge` 걸음 — diff 가 비었다.
    if [ "$nw" = 0 ] && [ "$nh" = 0 ]; then
      echo "  ✓ --purge 뒤 워킹트리 · HOME 스냅샷이 설치 전과 같다" >> "$out"
    else
      # 갈림이 있으면 **그 가운데 `L` 밖이 몇인지**까지 적는다 — 착수 바이너리의 RED 가
      # 「남았다」가 아니라 「허용 목록 밖이 남았다」라는 것을 이 수가 말한다.
      local out_l=0 rel
      while IFS= read -r rel; do
        [ -n "$rel" ] || continue
        in_l_worktree "$rel" || out_l=$((out_l + 1))
      done < "$work/dw"
      while IFS= read -r rel; do
        [ -n "$rel" ] || continue
        in_l_home "$rel" || out_l=$((out_l + 1))
      done < "$work/dh"
      echo "  ✗ --purge 뒤에도 갈림이 있다 — 워킹트리 $nw · HOME $nh (그 가운데 L 밖 $out_l)" >> "$out"
      step_rc=1
    fi
  else
    # 기본 걸음 — ① 갈림이 전부 `L` 안 ② 각 경로가 화면에 ③ `L` 의 정본이 직전 바이트.
    local outside=0 uncovered=0 changed=0 dirs=0
    local rel t
    while IFS= read -r rel; do
      [ -n "$rel" ] || continue
      if in_l_worktree "$rel"; then :; else
        echo "  ✗ [L 밖] 워킹트리 $rel" >> "$out"
        outside=$((outside+1))
      fi
      t="$(type_of "$rel" "$work/s2" "$work/s1")"
      if [ "$t" = D ]; then
        dirs=$((dirs+1))
      elif ! covered "$rel" "$screen"; then
        echo "  ✗ [화면에 없다] 워킹트리 $rel" >> "$out"
        uncovered=$((uncovered+1))
      fi
    done < "$work/dw"
    while IFS= read -r rel; do
      [ -n "$rel" ] || continue
      if in_l_home "$rel"; then :; else
        echo "  ✗ [L 밖] HOME $rel" >> "$out"
        outside=$((outside+1))
      fi
      t="$(type_of "$rel" "$work/h2" "$work/h1")"
      if [ "$t" = D ]; then
        dirs=$((dirs+1))
      elif ! covered "$home/$rel" "$screen"; then
        echo "  ✗ [화면에 없다] HOME $rel" >> "$out"
        uncovered=$((uncovered+1))
      fi
    done < "$work/dh"
    changed=$((nw+nh))

    # ③ `L` 의 정본 자리가 uninstall 직전 바이트와 같은가.
    local canon=0 drifted=0 line before after
    while IFS= read -r line; do
      rel="${line%%	*}"
      if is_canon_worktree "$rel"; then
        canon=$((canon+1))
        before="$line"
        after="$(awk -F'\t' -v p="$rel" '$1 == p { print; exit }' "$work/s2")"
        if [ "$before" != "$after" ]; then
          echo "  ✗ [정본이 직전 바이트와 다르다] 워킹트리 $rel" >> "$out"
          drifted=$((drifted+1))
        fi
      fi
    done < "$work/s1"
    while IFS= read -r line; do
      rel="${line%%	*}"
      if is_canon_home "$rel"; then
        canon=$((canon+1))
        before="$line"
        after="$(awk -F'\t' -v p="$rel" '$1 == p { print; exit }' "$work/h2")"
        if [ "$before" != "$after" ]; then
          echo "  ✗ [정본이 직전 바이트와 다르다] HOME $rel" >> "$out"
          drifted=$((drifted+1))
        fi
      fi
    done < "$work/h1"

    {
      echo "  갈림 $changed (워킹트리 $nw · HOME $nh) · 그 가운데 디렉터리 $dirs"
      echo "  ① L 밖 갈림            $outside"
      echo "  ② 화면에 없는 갈림       $uncovered   (디렉터리 엔트리는 뺀다 — L 이 「그 자리의 부모 디렉터리」로 이미 허용한 자리다)"
      echo "  ③ 잰 정본 자리 $canon · 직전 바이트와 갈린 것 $drifted"
    } >> "$out"
    if [ "$canon" = 0 ]; then
      echo "  ✗ 잰 정본 자리가 0 이다 — 아무것도 안 재고 통과할 뻔했다" >> "$out"
      step_rc=1
    fi
    if [ $((outside+uncovered+drifted)) -eq 0 ] && [ "$canon" -gt 0 ]; then
      echo "  ✓ 갈림이 전부 L 안이고 · 각 경로가 화면에 나오고 · L 의 정본이 직전 바이트다" >> "$out"
    else
      step_rc=1
    fi
  fi

  if [ "$mode" = purge ] && [ "$urc" != 0 ]; then
    echo "  ⚠ uninstall rc=$urc" >> "$out"
  fi
  echo >> "$out"
  echo "판정: $([ $step_rc -eq 0 ] && echo 통과 || echo 어긋남)  (rc=$step_rc)" >> "$out"
  echo "산출: $out"
  return $step_rc
}

for m in default purge; do
  step "$m"
  rc=$?
  if [ $rc -eq 2 ]; then
    echo "걸음 $m — 못 쟀다" >&2
    exit 2
  fi
  [ $rc -eq 0 ] || RC=1
  echo "걸음 $m — $([ $rc -eq 0 ] && echo 통과 || echo 어긋남)"
done

exit $RC
