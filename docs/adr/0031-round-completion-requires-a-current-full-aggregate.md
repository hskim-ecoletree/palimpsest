# ADR-0031 — **회차 완료는 현재 전수 aggregate의 checkpoint다**

**상태**: 채택 (2026-09-02) · [#101](https://github.com/hskim-ecoletree/palimpsest/issues/101) ·
선행 [ADR-0028](0028-round-verification-is-an-append-only-observation-ledger.md) ·
[ADR-0029](0029-command-oracles-require-exact-external-approval-and-current-controls.md) ·
[ADR-0030](0030-stop-registration-does-not-authorize-enforcement.md) ·
판정 [round completion 게이트](../gates/round-completion-current-aggregate.md)

## 맥락

schema 1의 성공 evidence는 승인·실행이나 tracked projection 없이 손으로 쓸 수 있었고,
`verification=met`과 제목만 있는 종료문만으로 Stop을 통과했다. schema 2도 command 현재성은
재지만 비결정론 판단과 열린 finding을 같은 완료 모집단에 넣지 않았고, 종료 직전 이미 성공한
command를 전수 재실행했다는 표식이 없었다. depth-1 clone은 최초 commit object가 없어 approval과
Stop activation identity를 만들지 못했다.

## 결정

- status의 읽기 호환과 종료 자격을 분리한다. schema 1·2는 계속 읽지만 schema 3의 current
  checkpoint만 `completion=complete`가 될 수 있다. Stop은 `verification=met`이 아니라 이 값을
  소비한다.
- schema 3은 command `oracle`과 비결정론 `judgment`를 한 aggregate에 넣는다. judgment는
  thesis·antithesis·synthesis의 상대 경로와 현재 file digest, 판정을 모두 지닌다. 파일 부재,
  symlink, digest 변화는 조건을 stale로 만든다.
- `findings.jsonl`은 schema 3의 열린 축을 명시해야 한다. 구형·손상 원장은 current가 아니며,
  열린 `금지역`·`실패` finding 하나라도 complete를 막는다.
- report의 필수 heading마다 다음 `##` 전의 비어 있지 않은 본문을 요구한다. folded도 같은
  방식으로 접은 사유와 더 먼저인 일을 실제 본문으로 요구한다.
- `pal round verify --all`만 checkpoint를 쓴다. report가 선 뒤 승인된 command 전부를 이미
  met인 것까지 다시 실행하고, current judgment·finding·terminal을 다시 읽은 뒤 projected
  digest와 aggregate digest를 append한다. status는 둘을 현재 값과 대조한다.
- clone identity는 origin URL의 길이 결박 hash를 우선하고, origin 없는 저장소만 first-parent
  root를 쓴다. 따라서 shallow boundary나 뒤 deepen·local commit이 identity를 바꾸지 않는다.
  이 값은 외부 private 승인·activation namespace에만 있고 공유 산출물에 machine path를 싣지
  않는다.
- 마지막 SHA CI는 계속 원장 밖 terminal observation이다. 추적 파일이 자기 커밋의 CI 성공을
  주장하지 않고, 외부에서 일곱 job을 확인한 뒤에만 병합한다.

## 결과

**얻는 것.** 구형 성공 기록, stale tracked tree, 빈 종료문, 미실행 전수 재검증, 낡은 판단
파일, 열린 해악 finding 중 하나라도 있으면 Stop이 complete를 소비하지 않는다. 얕은 clone도
일반 clone과 같은 approve·activation 명령을 쓴다.

**잃는 것.** schema 1·2 회차는 자동으로 complete가 되지 않는다. schema 3 회차는 report 뒤
전수 command 비용을 다시 내며, judgment 근거 파일과 findings ledger를 명시적으로 유지해야
한다. origin URL 변경은 새 repository identity이므로 재승인·재활성화가 필요하다.

## 되돌리는 조건

checkpoint를 직접 쓴 구형 또는 stale 기록이 complete가 되거나, current judgment·finding이
aggregate 밖으로 빠지거나, 이미 met인 command가 `verify --all`에서 재실행되지 않거나, 실제
depth-1 clone의 approve·Stop identity가 deepen 또는 local commit으로 바뀌면 이 결정을 다시
연다. 추적 산출물이 자기 CI 성공을 완료 근거로 요구해 마지막 commit을 만들 수 없게 되어도
외부 terminal 경계를 다시 결정한다.

## 범위 밖

- untracked·ignored 전체 filesystem snapshot
- command oracle의 immutable sandbox나 부작용 rollback
- 마지막 SHA CI를 repository ledger event로 복제하는 일
