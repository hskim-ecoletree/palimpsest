# 종료 보고 — round Stop progress guard

## 결론

#85의 수직 경로를 닫았다. Stop은 install catalog에 등록돼도 명시적 enable 전에는 통과하고,
활성화 뒤에는 current round의 불완전 종료를 실제 Claude Code transport에서 막는다. 의미 진행만
무진행 counter를 reset하며, 같은 의미 상태 6회에서는 round를 complete/met로 바꾸지 않은 채
blocked handoff를 남기고 세션 종료를 허용한다. disable은 즉시 inactive pass로 복구한다.

## 세운 장치

- `hook/catalog.rs`의 단일 typed catalog를 install settings와 dispatch가 함께 소비한다.
- `pal round stop enable|disable|status`가 portable private activation과 operational progress를
  관리하며 install/update는 activation을 만들지 않는다.
- Stop policy는 `stop_hook_active=true`를 최우선 통과시키고 기존 round status reducer를
  읽기만 한다. malformed active payload/state는 차단하고 unknown event/input fail-open은 보존한다.
- semantic digest/rank, replay-safe event hash, kernel advisory lock, atomic private record,
  6회 truthful handoff를 세웠다. nested cwd는 repository discovery로 해소하고 장기 transcript는
  regular file을 streaming hash한다.
- uninstall은 activation을 제거하고, 손상 activation도 body parse 없이 disable할 수 있다.
- ADR-0030과 게이트, portable graph export의 결박 `fc457053b9b88810`·`a09d4a147c335002`·
  `ca4855e1569b9f65`가 결정·승인·진행 판정을 코드 좌표에 건다.

## 검증과 효과

구현 전 RED는 새 black-box 14개 중 helper 9개 통과·기능 5개 실패로 보존했다. 최종 local
integration은 `round_stop` 20, `hook` 5, `install_hooks` 21, `round_status` 24,
`round_approve_verify` 22, `round_scripts_run` 15개가 전부 통과했다. `cargo xtask check`는
23/23, `cargo test --workspace --all-targets`는 성공했고 18개 schema 2 condition의 exact
approve/verify evidence가 모두 current `met`이다.

`pal doctor --full`의 그래프 전수는 위반 0·Residual 0이었다. 설치 진단은 이 source checkout에
manifest와 `.claude/settings.json`이 없고 빌드 산출물이 PATH에 설치되지 않았음을 3 Residual과
1 red로 냈다. 이번 승인 경계가 현재 저장소·사용자 전역 설치를 만들거나 활성화하지 말라고
정했으므로 설치 완료로 축약하지 않고 범위 밖 관측으로 남긴다.

격리된 임시 Git 프로젝트·HOME·config·private store에서 Claude Code 2.1.247로 inactive pass,
incomplete block, re-entry pass, approve→verify→evidence 진행, current positive/negative evidence와
report의 pass, 동일 의미 6회 handoff, incomplete 상태 보존, disable pass를 실제 관측했다. 원본은
[`effect/observation.md`](effect/observation.md)에 portable하게 보존했다.

구현·evidence SHA `f9691cc37f531ecfd50cae59f54cc3a052696bf9`의
[CI run 33361658434](https://github.com/hskim-ecoletree/palimpsest/actions/runs/33361658434)에서
ubuntu·macOS·Windows, 두 producer, 양방향 consumer 7개 job이 모두 성공했다. 종료 보고가 포함된
마지막 pushed SHA도 같은 workflow의 7개 job 성공을 종료의 외부 조건으로 삼는다.

## 발견 처분

사전부검 17건과 독립 리뷰 18건, 합계 35건은 정정 24·기각 11로 전부 닫혔다. 독립 리뷰가
찾은 종료문 골격 거짓 통과, progress off-by-one, 모순 operational record, stale-lock ABA,
nested cwd fail-open, 장기 transcript 상한은 `15af743`의 장치와 음성 대조로 정정했다. 열린
금지역·실패는 0이다.

## 남지 않은 것

회차 안에서 하겠다고 등록한 구현·리뷰·효과·결박·검증·CI·이슈 처분은 모두 닫혔다.

## 다음 회차가 받는 것

없음. #85의 소비 계약은 ADR-0030과 `docs/gates/round-stop-progress-guard.md`가 진다.

## 범위 밖

- #96 진행 원장 복원은 목표 안이지만 approve/verify reducer의 실제 Stop 소비 연결이 더 먼저라
  이번 회차에 포함하지 않았다. #85를 막는 측정은 없었다.
- 기존 verification schema 1/2, oracle/projected digest, condition/status JSON과 exit code 재설계.
- 현재 사용자 저장소·사용자 전역 설정의 Stop 활성화와 Claude Code 이외 host enforcement.
- 이 source checkout을 사용자 설치로 바꾸는 일. 실제 Stop 관측은 별도 임시 HOME/config와
  project-local settings에서 수행했다.

## 원리상 못 잰 것

없음. 실제 Claude Code 소비와 세 OS·양방향 artifact 소비를 각각 격리 관측과 CI로 쟀다.

## 능력 부재

그래프는 `stop_hook_active` field symbol과 cross-file consumption/effect/judgment를 직접 해소하지
못했다(F07·F13·F15). 해소 가능한 `EVENTS`, `command_enable`, `semantic_digest`에 직접 결박하고,
관계·효과·판정은 catalog 구조 시험, 실제 transport 관측, 게이트로 각각 보완했다.
