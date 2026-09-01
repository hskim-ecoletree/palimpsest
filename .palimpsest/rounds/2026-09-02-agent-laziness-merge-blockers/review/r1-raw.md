## 이번 라운드의 새 발견

| # | 발견 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 근거 |
|---|---|---|---|---|---|---|---|
| N1 | checkpoint를 원장에 직접 추가하면 `verify --all` 없이도 complete가 된다 | 원의도 | 참 | 금지역 | D1 F1 G1 | `crates/pal-cli/src/round/ledger.rs:115`, `crates/pal-cli/src/round/status.rs:346`, `crates/pal-cli/src/round/verify.rs:278` | checkpoint는 공개된 projected/aggregate digest 둘만 지닌다. status는 생산 경로를 확인하지 않고 두 digest의 동등성만으로 complete를 판정한다. |
| N2 | 빈 report를 `verify --all`과 status는 complete로 인정한다 | 원의도 | 참 | 금지역 | B1 D1 | `crates/pal-cli/src/round/status.rs:346`, `crates/pal-cli/src/round/verify.rs:243`, `crates/pal-cli/src/round/stop.rs:403` | 본문 검증은 Stop 경로에만 있다. finalize와 status는 report 파일 존재만 확인하므로 heading-only report를 complete로 기록한 뒤 Stop에서야 차단할 수 있다. |
| N3 | 불완전한 findings 행을 current 원장으로 인정한다 | 원의도 | 참 | 금지역 | D3 F1 | `crates/pal-cli/src/round/status.rs:420`, `crates/pal-cli/tests/round_approve_verify.rs:327`, `.claude/skills/round/bin/record.py:98` | completion 파서는 상태·해악도·닫은커밋만 검사한다. 정상 finalize 시험도 정본 스키마의 여러 필수 필드가 빠진 행으로 성공한다. |
| N4 | `verify --all`은 조건별 승인 profile을 복원하지 못한다 | 원의도 | 참 | 실패 | D1 F1 | `crates/pal-cli/src/round/approval.rs:38`, `crates/pal-cli/src/round/verify.rs:212` | 승인 digest는 shell·PATH·timeout·output limit을 포함하지만 복원 가능한 profile은 없다. finalize는 profile 하나를 모든 condition에 재사용하므로 서로 다른 profile로 승인된 조건들을 함께 재실행할 수 없다. |
| N5 | 새 gate가 원장 대조에서 빠진 채 전 조건 통과를 주장한다 | 회차기록 | 참 | 금지역 | F1 G1 | `docs/gates/round-completion-current-aggregate.md:3`, `docs/gates/round-completion-current-aggregate.md:17` | gate가 intent 파일이 아닌 회차 디렉터리를 링크해 `cargo xtask check`가 해당 회차를 `게이트 없음`으로 냈다. 필수 `## 범위 밖`도 없고, intent 상자는 전부 미측정인데 gate는 13개 전부 통과로 기록한다. |
| N6 | doctor checker가 `not_built` invariant를 전수 통과로 계산한다 | 자기장치 | 참 | 거짓신호 | F2 G1 | `scripts/check-round-doctor.mjs:19`, `.palimpsest/rounds/2026-09-02-agent-laziness-merge-blockers/GATES.md:22` | full doctor의 invariant 4~8은 `not_built`지만 checker는 `outcome.checked`가 있는 항목만 검사한다. `outcome: {}`도 통과하며 checker와 portable ledger 대조는 CI/xtask에 연결되지 않았다. |

## 내가 기각한 것

| # | 기각한 것 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 왜 아니었나 |
|---|---|---|---|---|---|---|---|
| X1 | shallow fixture가 실제 depth-1이 아니라는 의심 | 자기장치 | 거짓 | 미관 | C1 | `crates/pal-cli/tests/round_approve_verify.rs:406` | `--is-shallow-repository=true`, commit 수 1, `HEAD^` 부재를 확인하며 deepen과 local commit 뒤 Stop identity도 확인한다. |
| X2 | #101 항목이나 native blocker 관계가 조용히 줄었다는 의심 | 원의도 | 거짓 | 미관 | D4 | GitHub issues #85, #88, #95, #96, #97, #101 | #85·#88·#97은 완료됐고 #95는 구현 종료, #96은 우선순위를 명시한 NOT_PLANNED 처분이다. #101의 blocker 관계도 active 0/total 2로 보존됐다. |
| X3 | 기존 portable GATES 수리가 절대 경로나 사라진 도구를 남겼다는 의심 | 저장소 | 거짓 | 미관 | E1 | `.palimpsest/rounds/2026-08-30-agent-laziness-executable-plan/GATES.md:5` | 현재 명령은 저장소 상대 경로만 사용하며 macOS 절대 경로와 `gate-lint.mjs` 의존을 제거했다. |
| X4 | 현재 로컬 정책·시험 하네스 자체가 실패한다는 의심 | 저장소 | 거짓 | 미관 | F1 | `.palimpsest/rounds/2026-09-02-agent-laziness-merge-blockers/GATES.md:7` | focused suites는 25/25/22, `cargo xtask check`는 23/23, `cargo xtask test`는 exit 0으로 재확인했다. |

## 미측정 목록

- G2는 미측정이다. PR #91의 원격 head는 아직 기준 SHA `979e2433...`이므로 현재 `1064ce3`과 워크트리 수정에 대한 CI 7개, 병합, `origin/main` 포함을 아직 확인할 수 없다.

## 끝내도 되는가

안 된다 — 금지역 4건과 실패 1건이 남았고 최종 PR SHA의 CI도 미측정이다.
