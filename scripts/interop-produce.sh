#!/usr/bin/env bash
# **한 플랫폼이 「설치하고 커밋한 저장소」를 하나 낸다** — 다른 플랫폼이 받을 것.
#
# 이 스크립트가 내는 것은 **git 저장소 하나**다. 받는 쪽은 그것을 `git clone` 으로
# 받고, 그러면 **워킹트리 바이트를 git 이 정한다** — `core.autocrlf` 가 실제로 걸리는
# 자리이고 이 측정의 대상이다. 파일을 그대로 복사하면 그 축이 통째로 사라진다.
#
# ⚠ **홈을 안 건드린다.** 여기서 만드는 것은 전부 작업 디렉터리 아래에 산다.
#
# ⚠ **식별자는 ASCII 다.** bash 의 변수 이름은 비ASCII 를 못 받는다(실측: `낼곳=…` 이
# `No such file or directory` 로 죽는다). 이 저장소의 다른 셸 스크립트도 같은 관습이다.
set -euo pipefail

OUT="${1:?낼 자리를 인자로 주십시오}"
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# 이 플랫폼에서 실제로 띄울 수 있는 이름 — Windows 는 `.exe` 가 붙는다.
pal="$ROOT/target/debug/pal"
[ -x "$pal" ] || pal="$ROOT/target/debug/pal.exe"
[ -x "$pal" ] || { echo "pal 을 못 찾았다: $ROOT/target/debug/" >&2; exit 1; }

rm -rf "$OUT"
mkdir -p "$OUT"
cd "$OUT"

# **살고 있는 프로젝트**를 만든다 — 사용자 바이트가 있어야 병합이 실제로 일어난다.
# 줄바꿈을 **일부러 LF 로** 적는다. Windows 의 `core.autocrlf=true` 가 체크아웃에서
# 그것을 CRLF 로 바꾸고, 그 상태가 이 측정이 재려는 것이다.
printf 'hello\n' > README.md
printf '# 내 규칙\n지키자\n' > CLAUDE.md
printf 'node_modules/\n' > .gitignore
mkdir -p .claude
printf '{\n  "env": {"A": "1"}\n}\n' > .claude/settings.json

git init -q .
git config user.email t@e
git config user.name t
git add -A
git commit -qm '첫 — 설치 전'

# ★ **`pal` 을 `PATH` 에 얹고 설치한다.** 훅 등록은 `PATH` 의 이름 하나이므로
# 그 이름이 안 풀리면 설치가 「아직 안 뜬다」를 말한다 — 그것은 이 측정의 대상이 아니다.
PATH="$(dirname "$pal"):$PATH" "$pal" install --target .

git add -A
git commit -qm '설치'

echo "── 놓는 쪽이 낸 것 ──────────────────────────────────────────"
git -C . log --oneline
git -C . ls-files | sed 's/^/  /'
echo "── 커밋된 바이트의 줄바꿈 ────────────────────────────────────"
# **index 의 바이트**를 본다(워킹트리가 아니라). 이것이 다른 기계로 가는 것이다.
for f in CLAUDE.md .gitignore .claude/settings.json .claude/pal/manifest.json; do
  if git cat-file -e ":$f" 2>/dev/null; then
    crlf=$(git cat-file blob ":$f" | tr -dc '\r' | wc -c | tr -d ' ')
    echo "  $f  CR=$crlf"
  fi
done
