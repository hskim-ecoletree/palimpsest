# 격리된 실제 Stop 소비 관측

## 격리 경계

- 별도의 임시 Git 프로젝트, `HOME`, config, approval/progress 저장소를 사용했다.
- 저장소나 사용자 전역 hook 설정은 바꾸지 않았다.
- Claude Code 인증은 운영체제 자격 증명 저장소에서 실행 프로세스 환경으로만 주입했고,
  토큰·승인 digest·세션 식별자·절대 경로는 이 산출에 기록하지 않았다.
- Claude Code 2.1.247이 project-local `settings.json`의 `Stop` hook으로 실제
  `pal hook Stop`을 호출했다.

## 관측 장면

| 장면 | 실제 transport 관측 | 회차 상태 |
|---|---|---|
| 등록 후 비활성 | `Stop`이 등록돼 있어도 한 turn으로 종료했고 `Stop 정책이 활성화되지 않았다`고 통과했다 | open / in_progress |
| 명시적 활성화 | incomplete 회차에서 Claude Code가 hook의 `decision=block`을 소비했다 | open / in_progress, stale conditions |
| 재진입 | 같은 Claude Code 종료 재시도에서 `stop_hook_active=true` payload가 다시 block되지 않고 통과했다 | 바뀌지 않음 |
| 실제 진행 | `approve → verify → evidence` 뒤 두 condition이 met가 됐지만 report가 없을 때는 계속 block했다 | open / met |
| 완전한 종료 | 구조를 갖춘 report와 current positive/negative evidence가 함께 있을 때 한 turn으로 통과했다 | reported / met |
| 의미 변화 | report 추가로 projected digest가 바뀌자 기존 evidence가 stale이 됐고 counter가 reset됐다 | reported / in_progress |
| 무진행 상한 | 같은 의미 상태의 독립 Claude Code 세션 여섯 개에서 1/6부터 5/6까지 block하고, 6/6에서 종료를 허용하며 blocked handoff를 남겼다 | open / in_progress, stale conditions |
| truthful handoff | 상한 직후 `round status`는 여전히 `terminal=open`, `verification=in_progress`, 두 condition 모두 stale였다 | complete/met 승격 없음 |
| disable rollback | `round stop disable` 직후 status가 disabled가 됐고, 다음 실제 Stop은 한 turn으로 `Stop 정책이 활성화되지 않았다`며 통과했다 | 원래 종료 가능 상태 복구 |

## 음성 대조

- positive oracle만 current이고 negative control이 없을 때는 통과하지 않았다.
- report 파일의 존재만으로는 통과하지 않았고 필수 절을 갖춘 종료문만 인정했다.
- timestamp·파일 표현만 바꾼 반복은 의미 진행으로 세지 않았다.
- 상한 도달은 private blocked handoff만 기록했으며 round ledger와 evidence를 쓰지 않았다.
- hook 실행 중 `approve`, `verify`, oracle command는 실행되지 않았다.

## 보존 범위

fixture 원본은 `effect/template/`에 portable input만 보존한다. 임시 HOME/config, Claude
debug log, OAuth 자격 증명, approval/progress private record는 공유 산출물이 아니므로 보존하지
않는다. 위 표는 debug transport의 hook JSON, `pal round status --json`,
`pal round stop status --json`을 서로 대조해 얻었다.
