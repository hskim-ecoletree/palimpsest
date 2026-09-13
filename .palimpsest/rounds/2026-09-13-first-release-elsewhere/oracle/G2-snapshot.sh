#!/usr/bin/env bash
# `G2` — ditto 원본 상태 박제. `G2-ditto-origin-before.txt` 를 뜬 명령을 세션 전사에서 그대로 옮겼다.
# 사용: G2-snapshot.sh <머리 문구>   예) G2-snapshot.sh "회차 종료 쪽 박제" > oracle/G2-ditto-origin-after.txt
set -uo pipefail
O=~/dev/projects/ditto
echo "# ditto 원본 상태 — ${1:-박제} ($(date -u +%Y-%m-%dT%H:%M:%SZ))"
echo "HEAD $(git -C $O rev-parse HEAD)"
echo "porcelain_sha256 $(git -C $O status --porcelain | shasum -a 256 | cut -d' ' -f1)"
echo "porcelain_lines $(git -C $O status --porcelain | wc -l | tr -d ' ')"
echo "palimpsest_dir_listing_sha256 $(ls -laR $O/.palimpsest 2>/dev/null | shasum -a 256 | cut -d' ' -f1)"
echo "node_modules_entries $(ls $O/node_modules | wc -l | tr -d ' ')"
