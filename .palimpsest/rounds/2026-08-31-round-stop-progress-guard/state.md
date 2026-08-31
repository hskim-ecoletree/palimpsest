# 교대 상태 — round-stop-progress-guard

## 지금 단계

의도 잠금과 사전부검. #85를 assign하고 `ready-for-agent`·`slice`로 정리했다. 기준 SHA와
원격은 `9831bdcb09b3db7be7ac614b8e2191867ee0112c`, 제한 diff는 없었다.

## 잠긴 계약

- Stop은 단일 catalog에 등록되지만 명시적 `pal round stop enable` 전에는 통과한다.
- activation과 counter/handoff는 repository 밖 private store에 두고 portable project/round
  identity로 결박한다.
- `stop_hook_active=true`가 최우선이며 Stop은 기존 reducer를 읽기만 한다.
- semantic state가 바뀌면 reset, 같은 의미 상태의 서로 다른 session 6회면 blocked handoff와
  함께 통과한다. complete/met는 쓰지 않는다.
- 실제 활성화는 임시 HOME/config/project에만 한다.

## 그래프 조회

`EVENTS`, `HOOK_EVENTS`, `decide`, `ConditionsReport`, `round::status` 등의 좌표는 찾았으나
cross-file consumption은 답하지 못했다. `stop_hook_active`는 symbol로 해소하지 못했다.
그래프가 스스로 밝힌 능력 부재는 cross-file resolution(F07), effects(F13), judgment(F15)다.
그 뒤 탐색을 `crates/pal-cli/src/{hook,install,round}`와 대응 시험으로 제한했다.

## 남은 것

사전부검 처분, RED test와 실패 보존, 구현, 전체 검증, 격리 Claude Code 효과 관측,
독립 리뷰와 발견 처분, ADR·게이트·결박·보고, push와 최종 CI, #85·#86 종료.

## 실패한 접근

없음.
