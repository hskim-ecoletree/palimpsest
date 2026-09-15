#!/usr/bin/env bash
# **이 저장소 안에 릴리스 `pal` 을 받는다** — 첫 릴리스 사용 기록 기간(#160 · 소유자 `U27`)의 실행 파일.
#
# 전역(`PATH` · 홈 디렉터리)에 두지 않는다 — 소유자 지시(2026-09-15):
# *「이 프로젝트에 해. 전역에 만들면 쓰레기가 될 수도 있잖아」*.
# 받는 자리는 `.palimpsest/bin/` 이고 무시 목록에 있다. **정본은 릴리스 자산**이라 바이너리를 커밋하지 않는다 —
# 새 세션이나 다른 기계에서는 이 스크립트를 한 번 부르면 같은 바이너리가 온다.
#
#   scripts/pal-release.sh            최신 릴리스
#   scripts/pal-release.sh v0.1.1     그 태그
#
# 받은 자산은 같은 릴리스의 `SHA256SUMS` 로 대조한다(서명이 없다). 해시가 어긋나면 아무것도 안 두고 멈춘다.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPO=hskim-ecoletree/palimpsest
TAG="${1:-$(gh release view -R "$REPO" --json tagName --jq .tagName)}"

case "$(uname -s)-$(uname -m)" in
  Darwin-arm64)  target=aarch64-apple-darwin;     ext=tar.gz; bin=pal ;;
  Darwin-x86_64) target=x86_64-apple-darwin;      ext=tar.gz; bin=pal ;;
  Linux-x86_64)  target=x86_64-unknown-linux-gnu; ext=tar.gz; bin=pal ;;
  MINGW*-x86_64|MSYS*-x86_64|CYGWIN*-x86_64)
                 target=x86_64-pc-windows-msvc;   ext=zip;    bin=pal.exe ;;
  *) echo "이 플랫폼의 릴리스 자산이 없다: $(uname -s)-$(uname -m)" >&2; exit 1 ;;
esac
asset="pal-$target.$ext"

tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT
gh release download "$TAG" -R "$REPO" -p "$asset" -p SHA256SUMS -D "$tmp"

# ⚠ 자산 이름 한 줄을 `awk` 로 뽑는다 — macOS 의 BSD `grep` 은 `\|` 를 몰라 조용히 0 줄을 돌려줬다(회차 `2026-09-14-first-release`).
line="$(awk -v a="$asset" '$2 == a || $2 == "*" a' "$tmp/SHA256SUMS")"
[ -n "$line" ] || { echo "SHA256SUMS 에 $asset 줄이 없다" >&2; exit 1; }
want="${line%% *}"
if command -v sha256sum >/dev/null 2>&1; then
  got="$(sha256sum "$tmp/$asset" | cut -d' ' -f1)"
else
  got="$(shasum -a 256 "$tmp/$asset" | cut -d' ' -f1)"
fi
[ "$want" = "$got" ] || { echo "해시가 어긋난다: $asset (기대 $want · 받은 것 $got)" >&2; exit 1; }

case "$ext" in
  zip) (cd "$tmp" && unzip -q "$asset") ;;
  *)   tar xzf "$tmp/$asset" -C "$tmp" ;;
esac
mkdir -p "$ROOT/.palimpsest/bin"
mv -f "$tmp/pal-$target/$bin" "$ROOT/.palimpsest/bin/$bin"
chmod +x "$ROOT/.palimpsest/bin/$bin" 2>/dev/null || true

echo "$TAG · $asset · sha256 대조 일치 → .palimpsest/bin/$bin"
"$ROOT/.palimpsest/bin/$bin" --version
