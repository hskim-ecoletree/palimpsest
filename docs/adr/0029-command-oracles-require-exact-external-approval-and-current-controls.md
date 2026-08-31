# ADR-0029 — **command oracle는 외부 exact 승인과 현재 음성 대조가 함께 있어야 한다**

**상태**: 채택 (2026-08-31) · [#97](https://github.com/hskim-ecoletree/palimpsest/issues/97) ·
선행 [ADR-0028](0028-round-verification-is-an-append-only-observation-ledger.md) ·
판정 [round approve/verify 게이트](../gates/round-approve-verify.md)

## 맥락

schema 1의 읽기 전용 reducer는 관측을 실행하지 않는다. 따라서 누가 어떤 command를 어떤
workspace에서 실행하도록 허용했는지, 실행 뒤 입력이 그대로인지, 등록된 음성 대조가 실제로
돌았는지는 답하지 못했다. 단순히 command 문자열이나 성공 exit만 승인하면 PATH·CWD·shell·
projected tree가 바뀐 뒤에도 권한과 evidence를 재사용할 수 있고, 음성 대조 event만 등록해도
주 조건을 거짓으로 닫을 수 있다.

## 결정

`pal round approve`와 `pal round verify`를 schema 2의 새 회차에만 연다.

- 승인은 repository 밖 사용자별 private 저장소에 둔다. 승인 identity는 portable repository
  root identity, round·condition, oracle digest와 negative-control 역할, 상대 CWD, canonical
  shell path와 shell bytes, PATH, timeout·output budget, 현재 projected digest를 모두 결박한다.
  malformed·symlink·권한 또는 identity 불일치는 spawn 전에 fail-closed한다.
- command는 PATH에서 shell을 찾지 않는다. Unix는 `/bin/sh`, Windows는 환경 문자열이 아니라
  OS Known Folders API의 `System/cmd.exe`를 기본으로 쓰며 선택된 shell 자체도 승인한다. 다른
  shell은 승인하지 않는다. 기본 실행 예산은 120초와 stdout·stderr 합계 1 MiB다.
- projected digest는 `pal-git`의 정렬된 tracked worktree projection을 쓰되 Git index stat
  cache를 믿지 않고 매번 tracked blob bytes를 읽는다. 현재 round의 `verification.log`만
  제외한다. 절대 경로, HEAD, untracked·ignored 파일은 모집단이 아니다.
- verify는 실행 전 승인·oracle·projection을 확인하고 종료 뒤 append lock을 잡은 채 모두
  다시 계산한다. 하나라도 바뀌면 결과를 폐기한다. evidence append는 기존 완전 원장과 새 JSON
  한 행을 같은 디렉터리의 임시 파일에 쓰고 sync한 뒤 atomic replace한다. reader의 파일·행
  상한을 넘길 append는 교체 전에 거부한다. 여기서 보장하는 것은 성공 뒤 완전 원장 하나가
  보인다는 것이며 전원 손실 뒤 directory metadata persistence는 이 원장의 계약이 아니다.
  실패 뒤 command를 자동 재실행하지 않는다.
- positive command는 `exit == 0`, non-empty EXPECT의 combined output 관측, execution fault 없음이
  함께 있어야 성공이다. `negative_for`가 가리키는 각 control도 자기 oracle과 현재 projection에
  결박된 실제 성공 evidence가 있어야 주 조건이 `met`이다. 미실행·실패·stale control은 주
  조건을 `pending|unmet|stale`로 낮춘다. schema 2 oracle digest는 `negative_for` 역할도 포함해
  원장 편집만으로 과거 evidence를 control 성공으로 재분류하지 못한다.
- 부모가 먼저 끝나도 stdout·stderr EOF까지 기다리며 drain thread를 회수하고 read 오류를
  정상 EOF로 축약하지 않는다. Unix는 새 process group을 쓴다. Windows는 root를 suspended로
  spawn해 Job Object에 넣은 뒤에만 resume하며 timeout·output overflow 때 root PID가 사라졌어도
  job handle로 전체 tree를 종료한다. cleanup 실패는 성공 evidence가 아니다.
- Unix approval store는 owner·mode·link count를 요구한다. Windows는 OS Known Folders API의
  LocalAppData를 쓰고 owner가 current process token SID인지 handle에서 먼저 확인한다. 그 SID를
  상속을 끊은 단일 full-control DACL의 유일한 principal로 설정한 뒤 다시 읽어 일치시킨다.
  Windows는 [ADR-0023](0023-consistent-method-and-result-across-platforms.md)의 기존 판정대로
  stable safe API로 link count를 보려 하지 않는다. 쓰기는 임시 파일의 rename으로 기존 link를
  끊고, 읽기는 digest 이름과 exact record 내용 및 공유 없는 handle의 owner·DACL을 함께 확인한다.
  private user directory를 정할 수 없으면 temp로 후퇴하지 않는다.

Windows 전용 OS 표면은 `known-folders`, `windows-token`, `windows-permissions`, `process-wrap`의
safe API에 격리한다. 이 네 의존은 각각 환경에 기대지 않는 System/LocalAppData, 현재 token SID,
owner·DACL 설정/재검증, suspended-spawn Job Object를 제공하며 제품 크레이트의 `unsafe` 금지를
깨지 않는다.

schema 1의 직렬화, oracle digest, status JSON과 상태·exit 계약은 바꾸지 않는다. reader는
schema 1과 2를 version별로 읽고 schema 1에서 schema 2 필드를, schema 2 evidence에서 projected
digest 누락을 거부한다. 조건 문법은 계속 `pal-intent`, 원장과 실행 상태 기계는 private
`pal-cli` 모듈이 소유한다.

## 결과

**얻는 것.** 승인받지 않은 프로젝트 명령은 실행되지 않고, 승인 뒤 입력·환경이 바뀌면 다시
승인해야 한다. 실행된 현재 음성 대조 없이 positive 성공 하나만으로 조건이 닫히지 않는다.
status는 계속 읽기 전용이며 Python/dashboard는 기존 JSON을 그대로 소비한다.

**잃는 것.** PATH나 shell bytes, tracked projection, 예산이 달라져도 재승인이 필요하고,
platform default가 아닌 shell은 승인할 수 없다.
untracked·ignored 입력을 읽는 oracle은 이 projection만으로 현재성을 증명하지 못하므로 oracle
자체가 별도 결박 입력을 만들거나 뒤 결정이 모집단을 넓혀야 한다. command 실행과 evidence
append는 하나의 filesystem transaction이 아니므로 append 실패는 “실행됐지만 증거 없음”으로
남고 자동 재실행하지 않는다.

executor는 현재 workspace를 읽는 oracle이지 immutable checkout sandbox가 아니다. 동시 tracked
변경은 사후 currentness 검사에서 evidence를 폐기하지만 이미 실행된 command의 부작용을
되돌리지는 않는다. 그래서 approve/verify의 허용 모집단은 검증 command이고, mutable build나
배포 command를 transaction처럼 실행하는 표면이 아니다. 외부 approval 파일 삭제도 취소 API가
아니며 실행 뒤 재검사에서 evidence를 폐기하는 입력이다.

## 되돌리는 조건

세 OS에서 같은 tracked tree가 다른 projected digest를 내거나 descendant cleanup이 새 파일을
남기거나, exact 승인이 없는 spawn이 관측되거나, current negative-control 없이 주 조건이
`met`이 되면 결정을 즉시 다시 연다. untracked 입력이 실제 oracle의 주 모집단이라는 효과가
관측되면 projected membership도 다시 결정한다.

## 범위 밖

Stop 등록·차단과 진행 인지형 자기 상한은 #85의 뒤 수직 경로다. hook transport의 unknown-input
fail-open 계약과 finding·judgment 통합은 이 결정이 바꾸지 않는다.
