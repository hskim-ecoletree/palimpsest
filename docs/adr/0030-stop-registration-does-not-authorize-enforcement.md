# ADR-0030 — **Stop 등록은 정책 활성화를 승인하지 않는다**

**상태**: 채택 (2026-08-31) · [#85](https://github.com/hskim-ecoletree/palimpsest/issues/85) ·
선행 [ADR-0029](0029-command-oracles-require-exact-external-approval-and-current-controls.md) ·
판정 [Stop progress guard 게이트](../gates/round-stop-progress-guard.md)

## 맥락

hook event를 install settings에 등록하는 일과 그 event가 사용자의 종료를 막도록 승인하는 일은
다르다. 둘을 같은 boolean이나 중복 목록에 넣으면 update만으로 정책이 조용히 켜지거나,
registration과 dispatch가 갈라진다. 한편 Stop을 무한히 block하면 완료를 강제하는 장치가
세션 종료 자체를 막는다. 종료를 풀기 위해 round를 complete로 쓰면 더 큰 거짓 통과가 된다.

Claude Code 공식 hook 계약은 Stop input의 `stop_hook_active=true`를 이미 Stop hook 때문에
계속 중인 재진입으로 정의하고, 이 값을 검사해 풀리지 않을 조건의 반복 차단을 피하라고 한다.
host는 연속 8회 block 뒤 종료를 강제한다. 이 수와 payload는 외부 transport의 근거이며 저장소
상태의 근거는 아니다. 비교 구현 unlazy는 semantic gate progress가 없을 때 6회에서 경고와 함께
해제한다. pal은 이 두 근거보다 강한 자기 관측 없이 새 숫자를 만들지 않고 6을 고정한다.

## 결정

- `SubagentStop`과 `Stop`의 등록·command rendering은 하나의 typed event catalog가 소유한다.
  install/update는 catalog를 settings에 렌더링할 뿐 activation을 만들지 않는다. dispatch도 같은
  catalog를 소비하므로 별도 `HOOK_EVENTS` mirror는 없다.
- 정책은 프로젝트 소유자·운영자가 `pal round stop enable --round <slug>`를 직접 실행해야만
  켜진다. activation은 repository 밖 사용자별 private store에 두고 repository 첫 commit
  identity, round slug, policy version, 고정 상한을 digest한다. 절대 경로·사용자명·비밀은
  portable identity에 넣지 않는다.
- enable이 지정한 round만 읽는다. Stop은 `round::status` reducer와 terminal 문서를 읽는
  소비자이며 approve, verify, oracle, evidence append, intent·condition·terminal 변경을 하지
  않는다. `verification=met`인 current positive/negative evidence와 구조를 갖춘 report가 함께
  있거나, 사유를 갖춘 folded 종료문일 때만 정상 통과한다. 부재·pending·unmet·stale·invalid·
  partial·모순은 활성 정책에서 block한다.
- `stop_hook_active=true`는 cwd 해석과 activation/state I/O보다 먼저 무조건 통과한다. false는
  정책을 실행하고, 활성 Stop의 누락·잘못된 타입은 block한다. catalog 밖 event/input의 기존
  fail-open은 바꾸지 않는다.
- semantic progress는 round, aggregate verification, terminal, 정렬된 condition ID·state·oracle
  digest와 단조 rank로 정한다. timestamp, JSON 표현·순서·줄바꿈, session metadata, 같은 의미의
  evidence 재기록은 제외한다. 등록 수·관측 수·met 수·terminal rank가 이전 최고점을 넘을 때만
  counter를 0으로 reset한다. regression과 A→B→A 진동은 progress가 아니다.
- operational progress는 activation과 분리된 private record다. session ID와 transcript bytes를
  streaming hash한 event identity만 보존해 replay를 중복 계수하지 않는다. progress의 digest,
  rank, history, counter, handoff 불변식은 매번 검증한다. 안정된 lock inode에 kernel advisory
  lock을 잡고 bounded wait한 뒤 atomic replace하므로 stale-file 삭제 ABA를 만들지 않는다.
- 같은 의미 상태의 독립 사건 6회째에는 block JSON을 내지 않고 `blocked` handoff를 private
  record에 남긴다. round status, condition, evidence, report/folded는 쓰지 않는다. 따라서 세션은
  끝나도 회차는 incomplete인 진실을 보존한다.
- `pal round stop disable`은 activation body가 손상돼도 portable project identity의 record를
  제거한다. uninstall도 먼저 disable한다. registration은 남아도 activation 부재이므로 Stop은
  즉시 통과한다.
- hook payload의 cwd는 저장소 root라는 보장이 없으므로 `pal-git`의 repository discovery로
  worktree root를 찾는다. transcript는 크기 상한으로 정상 장기 session을 버리지 않고 regular
  file을 끝까지 streaming hash한다. 플랫폼 차이는 private-store/atomic replace의 기존 한
  자리에만 남긴다.

## 결과

**얻는 것.** 설치만으로 사용자 종료가 바뀌지 않는다. 명시적으로 켠 프로젝트에서는 current
round state가 종료 허용을 결정하고, 진행은 반복 상한을 reset하지만 표현 변화는 그러지 않는다.
무진행 루프는 host의 8회 강제 해제 전에 6회에서 truthful handoff로 끝난다. registration,
settings, dispatch 목록은 한 catalog라 #86의 drift 경로도 사라진다.

**잃는 것.** activation은 자동으로 현재 회차를 추측하지 않으므로 round를 명시해야 한다.
private progress가 손상되면 정책은 block하며 disable 전까지 자동 복구하지 않는다. transcript
전체 hashing 비용은 크기에 비례하지만 내용을 보존하지 않는다. 상한 6은 설정 옵션이 아니므로
바꾸려면 이 결정과 attack population을 다시 열어야 한다.

## 되돌리는 조건

등록만으로 activation이 생기거나, 의미 없는 변화가 counter를 reset하거나, 의미 진행이 reset되지
않거나, 6회 handoff가 complete/met를 쓰거나, re-entry가 다시 block되거나, disable 뒤 실제
transport가 계속 block하면 즉시 Stop을 disable하고 이 결정을 다시 연다. 세 OS에서 같은 portable
input이 다른 판정을 내거나 lock/replace가 부분 operational record를 노출해도 같다.

## 범위 밖

- #96 진행 원장 복원. 목표 안이지만 approve/verify reducer를 실제 Stop 소비에 연결하는 #85가
  먼저이며, #85를 막는 측정은 없었다.
- schema 1/2, oracle/projected digest, condition/status JSON과 exit code의 재설계.
- Claude Code 이외 host의 hard Stop enforcement와 사용자 전역 Stop 활성화.

## 외부 근거

- [Claude Code Hooks reference — Stop input과 decision control](https://code.claude.com/docs/en/hooks)
- [unlazy — optional Claude Code Stop hook](https://github.com/Leonxlnx/unlazy/blob/main/README.md#optional-claude-code-stop-hook)
