# 게이트 — round Stop progress guard

> 회차 [`2026-08-31-round-stop-progress-guard`](../../.palimpsest/rounds/2026-08-31-round-stop-progress-guard/) ·
> 잠긴 의도 [`intent.md`](../../.palimpsest/rounds/2026-08-31-round-stop-progress-guard/intent.md) ·
> 이슈 [#85](https://github.com/hskim-ecoletree/palimpsest/issues/85) ·
> 결정 [ADR-0030](../adr/0030-stop-registration-does-not-authorize-enforcement.md)

## 합격선

Stop registration과 activation이 분리돼 설치만으로 조용히 점등되지 않아야 한다. 명시적으로
활성화한 Stop은 current round의 불완전 종료를 실제 transport에서 막고 의미 진행만 counter를
reset해야 한다. 같은 의미 상태의 자기 상한은 세션을 끝내되 complete/met를 위조하지 않고,
disable은 즉시 원래 종료 가능 상태를 복구해야 한다. 마지막 pushed SHA의 세 OS가 같은 공격
모집단을 실행해야 한다.

**RED** — 기준 구현 `9831bdc`에서 새 `round_stop` black-box 14개 중 helper 9개만 통과했고,
기능 5개는 존재하지 않는 `pal round stop` subcommand와 clap exit 2로 실패했다. 원문은
[`red/red-observation.md`](../../.palimpsest/rounds/2026-08-31-round-stop-progress-guard/red/red-observation.md)가 진다.

## 판정

| 판정 | 조건 |
|---|---|
| 통과 | — |
| 반증 | — |
| 대조불가 | — |
| 미측정 | A1 A2 B1 B2 B3 C1 C2 D1 D2 E1 E2 F1 F2 G1 G2 H1 H2 H3 |

**검산** — 통과 0 · 반증 0 · 대조불가 0 · 미측정 18 = 18

### 현재 근거

- A1·A2·F2·G1·G2: typed hook catalog와 install/update/uninstall black-box가 registration,
  settings, dispatch, activation, rollback을 양방향으로 잰다.
- B1~C2: `round_stop`과 `hook` black-box가 aggregate 상태, malformed/partial 원장,
  terminal 구조, active payload, re-entry 우선순위와 unknown fail-open을 잰다.
- D1~E2: semantic digest/rank, replay, restart, concurrent session, corrupt progress, kernel lock,
  streaming transcript와 6회 truthful handoff를 공격한다.
- F1: Stop 전후 tracked round tree와 ledger를 대조해 command/oracle/approve/verify/evidence 및
  condition/terminal write가 없음을 잰다.
- H1: 시험이 아닌 실제 Claude Code 소비 장면은
  [`effect/observation.md`](../../.palimpsest/rounds/2026-08-31-round-stop-progress-guard/effect/observation.md)에 보존했다.
- H2·H3: ADR, 발견 원장, 그래프 결박, 종료 보고와 전체 검증·최종 CI는 종료 판정에서 갱신한다.

## 효과

별도 임시 Git 프로젝트·HOME·config·private store에서 Claude Code 2.1.247이 project-local Stop
hook을 실제 소비했다. inactive pass, incomplete block, `stop_hook_active` re-entry pass,
approve→verify→evidence 진행, current positive/negative evidence와 report의 complete pass,
동일 의미 6회 blocked handoff, incomplete 상태 보존, disable pass를 관측했다. 공유 산출에는
절대 경로·토큰·승인 digest·세션 식별자를 싣지 않았다.

## 범위 밖

- #96 진행 원장 복원 — 목표 안이지만 실제 Stop 소비 경로가 먼저이고 blocking 측정이 없었다.
- 기존 verification schema와 status/exit 계약 재설계.
- 사용자 저장소·전역 설정의 Stop 활성화와 Claude Code 밖 host enforcement.
