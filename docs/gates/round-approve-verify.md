# 게이트 — round approve/verify

> 회차 [`2026-08-31-round-approve-verify`](../../.palimpsest/rounds/2026-08-31-round-approve-verify/) ·
> 잠긴 의도 [`intent.md`](../../.palimpsest/rounds/2026-08-31-round-approve-verify/intent.md) ·
> 이슈 [#97](https://github.com/hskim-ecoletree/palimpsest/issues/97) ·
> 결정 [ADR-0029](../adr/0029-command-oracles-require-exact-external-approval-and-current-controls.md)

## 합격선

외부 exact 승인 없이는 command가 spawn되지 않고, 실행 전후 oracle·tracked projection이 현재인
완전한 evidence만 append되어야 한다. 연결된 음성 대조의 실제 현재 성공 evidence가 없으면
positive 성공만으로 주 조건을 `met`으로 닫을 수 없다. schema 1과 기존 소비·hook 계약은
그대로이고 마지막 pushed SHA의 세 OS가 같은 공격 모집단을 실행해야 한다.

**RED** — baseline `e9a1da7`에서 새 black-box 18개 중 공통 helper 10개만 통과하고 기능 시험
8개는 존재하지 않는 approve/verify subcommand와 clap exit 2로 실패했다. 원문은
[`red-observation.md`](../../.palimpsest/rounds/2026-08-31-round-approve-verify/red-observation.md)가 진다.

## 판정

| 판정 | 조건 |
|---|---|
| 통과 | A1 A2 A3 A4 A5 A6 A7 A8 A9 A10 A11 A12 A13 |
| 반증 | — |
| 대조불가 | — |
| 미측정 | A14 A15 |

**검산** — 통과 13 · 반증 0 · 대조불가 0 · 미측정 2 = 15

### 현재 근거

- A1~A6, A8~A10: `round_approve_verify` black-box 21개가 승인 identity drift, malformed record,
  timeout/output cap, process tree, 중간 oracle/projection 변경, append lock·partial line, stale·rerun,
  EXPECT 극성, 미실행 음성 대조, racy stat, control-role replay, 부모 선종료 descendant를 직접
  공격한다.
- A4: `pal-git::GitAccess::worktree_digest_excluding`가 stat cache 없이 tracked bytes를 다시
  읽고 현재 ledger 하나만 제외한다. evidence는 projected digest를 지니고 reducer가 현재 값과
  대조한다.
- A7·A11: `round_status`의 schema 1 golden과 Python/dashboard 호환은 바뀌지 않았고 schema 2
  version test가 schema 1 read compatibility와 version별 unknown 거부를 잰다. hook/install 회귀와
  의존 방향 검사가 기존 경계를 지킨다.
- A12: 시험이 아닌 빌드된 `pal`의 승인 전 exit 3, positive 뒤 pending, 음성 대조 뒤 met 전이는
  [`effect/observation.md`](../../.palimpsest/rounds/2026-08-31-round-approve-verify/effect/observation.md)에 보존했다.
- A13: 요구된 다섯 integration suite, `cargo xtask check` 23/23, workspace all-targets가
  통과했다. workspace의 기존 release 규모 benchmark 하나만 선언대로 ignored이고 새 공격
  모집단은 19/19 실행됐다.

## 효과

시험이 아닌 새 임시 Git 저장소에서 빌드된 `pal`을 실행했다. 승인 전 verify는 exit 3으로
차단됐고, positive evidence만 append한 뒤에는 A1과 연결 control이 모두 `pending`이었다. 실제
known-broken control을 별도 승인·실행해 두 번째 current evidence가 생긴 뒤에만 둘과 aggregate가
`met`이 됐다. 공유 산출물에는 임시 절대 경로나 approval identity를 싣지 않았다.

## #85 소비 계약

뒤 Stop 경로는 `pal round status`의 기존 상태와 approve/verify의 fail-closed exit를 소비할 수
있다. 이번 diff는 hook을 점등하지 않고 `hook`·`install` 실행 경계를 바꾸지 않는다.

## 범위 밖

- Stop 등록·차단과 진행 인지형 자기 상한(#85)
- untracked·ignored filesystem 전체 snapshot
- finding·judgment 조건 실행
- 과거 schema 1 회차의 소급 migration
