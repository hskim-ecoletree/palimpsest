# round-stop-progress-guard

> 착수 기준 `9831bdcb09b3db7be7ac614b8e2191867ee0112c` · 이슈 #85 · 2026-08-31
>
> 인터뷰 상한 1 · 사전부검 상한 2 · 독립 리뷰 상한 2

## 원문

> 그 뒤 `/round`를 열고 다음 수직 경로 `round-stop-progress-guard`에 바로 착수한다.
> 이번 회차의 착수 이슈는 #85
> 「Stop 훅 정책과 진행 인지형 자기 상한 — 관측만 하고 점등은 안 했다」다.
> 프론티어의 다른 정렬보다 소유자가 지정한 이 수직 소비 경로를 우선한다.

> 등록만으로 Stop 정책이 조용히 활성화되지 않아야 하고, 불완전한 회차의 종료 차단,
> 의미 있는 진행의 인식, 무진행 자기 상한, 복구 가능한 비활성화가 모두 실제 장치와 효과
> 관측으로 입증되어야 한다.

> 이 프롬프트는 #85의 구현과 격리 fixture 안에서의 Stop 점등 및 실제 block/pass 관측을
> 승인한다. 현재 저장소의 사용자용 hook 설정이나 사용자 전역 설치에서 Stop을 활성화하는
> 것은 승인하지 않는다. 실제 관측은 임시 HOME/config와 격리 프로젝트를 사용한다.
> 위험 신호가 나오면 Stop 비활성화 rollback은 별도 승인 없이 즉시 수행해도 된다.

> `pal hook Stop`은 읽기 전용 상태 소비자다. 승인 생성, oracle 실행, evidence append,
> 조건 변경이나 complete 승격을 해서는 안 된다. 직전 회차의 approve/verify 계약을
> 우회하거나 복제하지 않는다.

> 중간 구현, 초록 단위 시험, 설치 목록 변경만으로 멈추지 마라. #85의 핵심은 Stop을
> “추가했다”가 아니라 실제 소비 장면에서 불완전 종료를 막고, 진행을 인식하며, 무진행
> 루프에서는 진실을 보존한 채 빠져나오는 것이다. 독립 리뷰, 효과 관측, 최종 CI와 회차
> 종료까지 진행해라.

## 목적 기여

[00-goals.md](../../../docs/plan/00-goals.md)의 목표인 에이전트 하네스가 계획·실행·검증
루프를 실제로 강제하는 능력에 기여한다. 지금은 승인·검증 reducer 다음의 첫 실제 소비
경로이며, 이를 먼저 세워야 앞 수직 경로의 산출이 사용자 종료 행동을 바꾼다.

## 인터뷰에서 잠근 결정

- **경계** — 등록 catalog는 `SubagentStop`과 `Stop`을 함께 지지만 활성 상태는 지지 않는다.
  install/update는 둘을 등록할 뿐 Stop을 활성화하지 않는다. 네 표면 밖의 daemon이나 별도
  설정 정본은 만들지 않는다.
- **승인** — 프로젝트 소유자·운영자가 `pal round stop enable --round <slug>`를 직접 실행한
  것이 승인이다. 기록은 approve/verify와 같은 사용자별 private store에 둔다. repository의
  첫 commit identity, round slug, policy version, 고정 상한만 digest에 넣어 절대 경로·사용자명·
  비밀 없이 clone 간 같은 승인을 가리킬 수 있게 한다.
- **현재 회차** — enable 시 명시한 slug를 activation record에 결박한다. 자동 검색은 승인
  대상을 바꿀 수 있으므로 Stop 판정 때 하지 않는다. round 부재와 복수 terminal은 손상이다.
- **상태 판정** — activation 부재는 통과다. 활성 round의 open, unregistered, pending, stale,
  unmet, invalid는 차단한다. `verification=met && terminal=reported`와 명시적 `folded`는
  통과한다. folded는 완료가 아니라 `/round`의 별도 종료문이다.
- **재진입** — `stop_hook_active=true`는 cwd, activation, payload type, state read보다 먼저
  무조건 통과한다. false만 정책을 실행한다. 필드 누락·잘못된 타입은 activation이 확인된
  경우 차단하며, activation을 해소할 cwd조차 없는 미인식 입력은 기존 fail-open을 지킨다.
- **진행** — digest에는 round, aggregate verification, terminal, condition ID·state·oracle
  digest의 정렬된 집합만 넣는다. timestamp, JSON 공백·키 순서·줄바꿈, session metadata,
  같은 의미의 evidence 재기록은 제외한다. counter 초기화는 digest 변화만이 아니라 등록 조건
  수·met 조건 수·terminal의 단조 진척 순위가 이전 최고점을 넘을 때만 한다. regression과
  A→B→A 진동은 새 상태여도 진행이 아니며 거짓 완료가 되지 않는다.
- **무진행 상태** — activation record와 별도의 private operational record에 둔다. 사건
  identity는 project·round·session ID와 transcript 내용의 hash이며 raw transcript와 경로는
  저장하지 않는다. 같은 transcript replay는 중복 계수하지 않고 새 transcript 시도는 센다.
  원자 lock과 replace로 서로 다른 session을 직렬화하고 진척 순위가 최고점을 넘을 때만
  횟수를 초기화한다.
- **상한** — 선행 unlazy의 실측값 6회를 채택한다. Claude Code 자체 8회 차단 상한보다 먼저
  풀려 외부 강제 해제를 의존하지 않는다. 값을 사용자 옵션으로 열지 않는다.
- **truthful handoff** — 6회째에는 차단 JSON을 내지 않아 session 종료를 허용하되 operational
  record에 blocked handoff와 마지막 의미 상태를 남긴다. intent, verification, evidence,
  report/folded를 쓰지 않는다.
- **손상과 퇴로** — 명시적 activation record가 존재한 뒤의 malformed payload·round state·
  progress state는 fail-closed다. disable은 record 내용 파싱 없이 project identity 파일을
  제거해 손상 상태에서도 즉시 복구한다.
- **#86** — 등록 catalog를 가르는 순간 설치 목록과 판정 목록의 반복 자체를 없애야 하므로
  구조적으로 불가분이다. 같은 회차에서 양방향 구조 시험을 세우고 근거를 #86에 남긴다.

질문의 경계·의도·자율·종료·재고 범주를 모두 열었다. 소유자가 활성 범위, rollback 자율,
실제 효과 장면, 기존 reducer 재사용, #86 포함 판정까지 원문에서 지정했으므로 추가 선택을
요구하지 않는다.

## 완수 조건

- [ ] A1 단일 event catalog에서 Stop 등록이 렌더링되지만 activation 전 hook은 pass한다.
- [ ] A2 명시적 enable만 portable private activation record를 만들고 install/update는 만들지 않는다.
- [ ] B1 활성 Stop은 open round의 unregistered·pending·unmet·stale condition을 각각 block한다.
- [ ] B2 손상된 intent·부분/trailing 원장·없는 round·서로 충돌하는 terminal은 fail-closed한다.
- [ ] B3 current positive·negative evidence 전부가 met이고 terminal=reported인 round와 folded round만 pass한다.
- [ ] C1 `stop_hook_active=true`는 다른 모든 판정보다 먼저 무조건 pass한다.
- [ ] C2 Stop의 active payload에서 false는 정책을 실행하고 누락·잘못된 타입은 block하며 unknown event/input fail-open은 보존한다.
- [ ] D1 semantic digest는 실제 condition/aggregate/terminal 변화만 진행으로 세고 표현·timestamp·순서·동일 의미 evidence는 세지 않는다.
- [ ] D2 의미 진행만 counter를 reset하고 regression·A→B→A 진동은 reset하거나 complete/met로 취급하지 않는다.
- [ ] E1 같은 의미 상태의 서로 다른 session 6회에서 pass+blocked handoff가 나고 round 상태를 쓰지 않는다.
- [ ] E2 replay·restart·동시 session·stale/corrupt progress record와 원자 갱신을 Linux·macOS·Windows 계약으로 고정한다.
- [ ] F1 Stop 판정 중 command/oracle/approve/verify/evidence append/condition·terminal 쓰기가 일어나지 않는다.
- [ ] F2 disable은 정상·손상 activation에서 즉시 pass로 복구하고 update/uninstall과 갈리지 않는다.
- [ ] G1 기존 SubagentStop, hook transport 바이트, install/update/uninstall/doctor 계약이 회귀하지 않는다.
- [ ] G2 #86의 등록/settings drift는 단일 catalog와 양방향 음성 대조로 구조적으로 닫힌다.
- [ ] H1 격리된 실제 Claude Code에서 inactive pass, incomplete block, progress 인식, complete pass, cap handoff, re-entry pass, disable pass를 보존한다.
- [ ] H2 ADR·게이트·그래프 결박·구조화 발견 원장·종료 보고가 `/round` 계약대로 닫힌다.
- [ ] H3 지정된 국소 시험, `cargo xtask check`, workspace all-targets와 마지막 pushed SHA의 세 OS·양방향 상호운용 CI가 성공한다.

## 퇴로

위험 신호가 나오면 activation record를 즉시 지우는 `pal round stop disable`로 Stop 정책을
비활성화한다. registration은 남겨 기존 SubagentStop 관측을 보존하되 Stop은 activation 부재로
통과한다. 상한의 동시성·원자성을 세 OS에서 세울 수 없으면 정책은 비활성 상태로 남기고
회차를 완료로 선언하지 않는다.

## 범위 밖

- #96 진행 원장 복원 — 목표 안이지만 지금은 앞 reducer를 실제 Stop 소비 경로에 연결하는
  것이 더 먼저다. #85를 막는 측정이 나오지 않는 한 포함하지 않는다.
- 기존 schema 1/2, oracle/projected digest, condition/status JSON과 exit code 재설계.
- 현재 저장소의 사용자용 설정 또는 사용자 전역 Claude 설정에서 Stop 활성화.
- Claude Code 이외 호스트의 hard Stop enforcement.

## 개정

없음.

## 승격

없음.
