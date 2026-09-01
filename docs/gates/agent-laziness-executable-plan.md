# 게이트 — 에이전트 게으름 구현 착수 계획

> 회차 [`2026-08-30-agent-laziness-executable-plan`](../../.palimpsest/rounds/2026-08-30-agent-laziness-executable-plan/) ·
> 잠긴 의도 [`intent.md`](../../.palimpsest/rounds/2026-08-30-agent-laziness-executable-plan/intent.md) ·
> 산출 [`agent-laziness-executable-implementation-plan.md`](../agent-laziness-executable-implementation-plan.md)

## 합격선

계획은 다음 세션이 넓은 비교를 반복하지 않고 첫 RED를 쓸 수 있을 만큼 닫혀 있어야 한다.
그 판정은 계획의 §2~§7, 현재 저장소 좌표, upstream 일차 자료, 사전부검과 독립 검토를 함께 댄다.

### RED와 음성 대조

착수 전 비교 문서에는 첫 구현 회차의 단일 범위, event schema, 상태·exit 표, JSON 경계,
파일별 소유권과 정확한 RED fixture가 한 자리에 없었다. 계획에서 schema, CLI 경계, 코드 좌표,
RED 중 하나를 제거하면 다음 세션이 구현 전에 다시 설계해야 하므로 합격하지 않는다.

## 효과

테스트가 아닌 소비 경로로 `pal plan docs/agent-laziness-executable-implementation-plan.md --json`을
실행했고, 문서 headline과 §1~§8 항목이 계획 입력으로 실제 해석되는 것을 확인했다. 그래프는
Markdown·Python 코드 좌표를 답하지 못해 결박은 능력 부재로 남겼다.

## 판정

| 판정 | 조건 |
|---|---|
| 통과 | A1 A2 A3 A4 A5 A6 |
| 반증 | — |
| 대조불가 | — |
| 미측정 | — |

**검산** — 통과 6 · 반증 0 · 대조불가 0 · 미측정 0 = 6

### 근거

- A1: 현재 코드·ADR·이슈와 upstream 고정 커밋 및 Claude Code 공식 hook 문서를 대조했다.
- A2: 계획 §2·§4·§5·§7에 첫 회차 범위, 소유 좌표, RED, 검증, 착수 순서가 있다.
- A3: 계획 §3이 지금 잠글 값과 approve/verify·Stop 회차에서 실측할 값을 가른다.
- A4: 계획 §6이 기존 이슈별 조치·의존 방향·닫는 조건을 적고 실제 쓰기는 다음 세션으로 남긴다.
- A5: Rust·Python 구현은 바꾸지 않았고 문서와 회차 기록만 추가했다.
- A6: `cargo xtask check`가 `검사 23/23 통과`를 냈다.

## 퇴로

첫 구현 세션의 제한 diff가 기준 SHA 뒤의 관련 코드 변화를 보이면 그 경로만 다시 조사한다.
projected snapshot은 첫 회차에 억지로 넣지 않고 approve+verify 회차의 spike가 API 가능성을
보인 뒤 편입한다. 비결정론 condition과 Stop도 각 수직 경로의 원장이 서기 전에는 완료로 세지 않는다.

## 범위 밖

Rust·Python 구현, GitHub 이슈 변경, 과거 회차 이주, Depth Tree·병렬 lease·모델 가격
라우팅은 이 계획 회차에서 실행하지 않았다.
