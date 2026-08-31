# 교대 상태 — round-stop-progress-guard

## 지금 단계

구현·독립 리뷰·격리된 실제 Claude Code 효과 관측까지 끝났다. 기준 SHA
`9831bdcb09b3db7be7ac614b8e2191867ee0112c` 뒤 Stop catalog/activation/status/progress guard와
리뷰 수정이 `4ca7bc0`·`15af743`에 섰다. 현재 사용자 저장소·전역 설정의 Stop은 활성화하지
않았다.

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

완수 조건 oracle 승인·실행, 지정 국소 시험과 workspace 전체 검증, 그래프 결박, 종료 보고,
push와 마지막 SHA의 CI, #85·#86 종료.

## 실패한 접근

- 독립 리뷰가 종료문 파일 존재만 믿는 거짓 통과, progress counter off-by-one, 모순 record,
  stale-lock ABA, nested cwd fail-open, 장기 transcript 상한을 찾았다. `15af743`에서 종료문 구조
  검증, counter=0, semantic invariant, kernel advisory lock, repository discovery, streaming
  hash로 닫았다.
- 격리 Claude Code 비활성 재관측 첫 시도는 임시 HOME의 PATH에 `pal`이 없어 transport
  fail-open했다. 그 결과는 버리고 빌드 디렉터리를 PATH에 명시해 실제 `pal hook Stop`의
  inactive pass를 다시 관측했다.
