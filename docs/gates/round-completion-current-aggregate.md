# 게이트 — round completion current aggregate

> 회차 [`2026-09-02-agent-laziness-merge-blockers`](../../.palimpsest/rounds/2026-09-02-agent-laziness-merge-blockers/) ·
> 잠긴 의도 [`intent.md`](../../.palimpsest/rounds/2026-09-02-agent-laziness-merge-blockers/intent.md) ·
> 이슈 [#101](https://github.com/hskim-ecoletree/palimpsest/issues/101) ·
> 결정 [ADR-0031](../adr/0031-round-completion-requires-a-current-full-aggregate.md)

## 합격선

schema 3의 전수 재실행, 정반합 판단, current finding, 본문 있는 종료문, current projected·aggregate
checkpoint가 모두 서야 complete다. 구형 기록·tracked 변경·빈 문서·열린 해악·실제 depth-1
clone 실패는 각각 정상 경로와 한 시험 안에서 대조한다.

## 판정

| 판정 | 조건 |
|---|---|
| 통과 | A1 A2 B1 C1 D1 D2 D3 D4 E1 F1 F2 G2 |
| 반증 | G1 |
| 대조불가 | — |
| 미측정 | — |

**검산** — 통과 12 · 반증 1 · 대조불가 0 · 미측정 0 = 13

## 근거

- `round_status`, `round_approve_verify`, `round_stop`의 black-box 시험이 schema 1/3,
  tracked stale/current, 빈/유효 report·folded, shallow/full history, 전수 command 재실행,
  external seal 없는 직접 checkpoint, canonical/noncanonical profile, current/stale judgment,
  current/legacy/open/malformed finding을 정상·음성 쌍으로 잰다.
- `cargo xtask check`, `cargo xtask test`, `pal doctor --full --json`의 구조 검사와 고의 위반
  doctor fixture가 저장소 전수 경계를 잰다.
- #95는 외부 final-SHA CI 순서로 구현하고 #96은 더 먼저인 소비 가능한 Stop 경계를 지목해
  명시적으로 접는다. 독립 검토의 finding은 회차 원장에 같은 축으로 반영한다.
- 최종 SHA의 CI 일곱 job, PR 병합, `origin/main` 포함은 추적 판정문 밖에서 순서대로 관측한다.

## 효과

구형 성공 JSONL과 제목만 있는 종료문을 주어도 Stop이 complete로 읽지 않는다. 반대로 실제
shallow clone에서 승인·활성화하고 schema 3 전수 checkpoint를 만든 정상 fixture만 같은 소비
경로를 통과한다. 마지막 SHA CI는 자기 참조 없이 외부 종료문으로 남는다.

## 범위 밖

- untracked·ignored 전체 filesystem snapshot과 command 부작용 rollback
- 마지막 SHA CI를 repository ledger event로 복제하는 일
