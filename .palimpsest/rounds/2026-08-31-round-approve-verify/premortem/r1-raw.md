저장소·현재 구현·직전 회차 산출은 보지 않았다. 아래 좌표는 제공된 원 의도와 계획만으로 지정한 “반드시 조회할 계약 지점”이다.

| 번호 | 정확한 실패 시나리오 | 코드/계약 좌표 | 해악도 | 처분 권고 |
|---:|---|---|---|---|
| 1 | 승인이 command 문자열에만 결박되고 repository/worktree, projected digest, oracle digest, CWD가 빠져 승인 뒤 다른 내용에 같은 명령이 실행된다. | external approval store의 key/value schema, approval lookup predicate 조회 필요 | 금지역 | **계획 수정:** 승인을 `repo identity + CWD + canonical command + oracle digest + projected digest`에 결박. **완수 조건:** 어느 축 하나만 달라도 spawn 전 거부. |
| 2 | approval record가 위조·교체·부분 기록되거나 다른 사용자/프로세스가 만든 승인을 신뢰한다. store가 외부라는 이유만으로 신뢰 경계가 흐려진다. | approval store의 소유권, 권한, integrity 및 atomic write 계약 조회 필요 | 금지역 | **탐지 추가:** malformed/partial/untrusted-owner record는 fail-closed. **완수 조건:** 임의 fixture가 승인 파일을 주입해도 실행되지 않음. |
| 3 | `approve`와 `verify`가 command를 다르게 정규화한다. 같은 문자열이 POSIX shell, Windows quoting, argv 실행 여부에 따라 다른 프로그램·인수를 실행한다. | CLI parser → approval serializer → executor spawn API 사이 canonical-command 계약 조회 필요 | 금지역 | **계획 수정:** shell 문자열 대신 명시적 argv 계약을 우선하고 shell 사용 여부도 승인 자료에 포함. 세 OS quoting corpus를 완수 조건에 추가. |
| 4 | 승인 시점의 PATH/CWD/env와 실행 시점의 값이 달라 동일 command가 다른 executable 또는 config를 선택한다. | executor의 executable resolution, cwd/env capture 및 approval binding 조회 필요 | 금지역 | **완수 조건:** resolved executable identity와 CWD가 바뀌면 거부. PATH shadowing, relative executable, case 차이 fixture 추가. |
| 5 | approval 검사와 process spawn 사이 승인 폐기·교체 또는 content 변경이 일어나는 TOCTOU가 생긴다. | approval consume/validate/spawn 경계와 locking 또는 generation-token 계약 조회 필요 | 금지역 | **계획 수정:** 승인 generation을 spawn 직전 재검증하고 one-shot이면 원자적으로 consume. race fixture를 탐지 항목에 추가. |
| 6 | projected content-tree digest가 untracked file, symlink target/type, executable bit, 삭제 예정 파일, ignore 경계 중 일부를 빠뜨려 실제 실행 입력이 달라도 current로 판정한다. | projected-tree membership 및 canonical serialization 규칙 조회 필요 | 거짓신호 | **완수 조건:** 포함·제외 집합을 계약화하고 각 항목 단일 변이 시험 추가. 플랫폼별 표현이 달라도 의미가 같으면 같은 digest여야 함. |
| 7 | CRLF/LF, Unicode normalization, 경로 separator·case·reserved name 차이로 세 OS가 같은 tree에 다른 digest를 내거나 다른 tree에 같은 digest를 낸다. | projected digest의 path/content canonicalization 계약 조회 필요 | 실패 | **완수 조건:** 공통 golden vectors를 세 OS에서 동일 digest로 검증하고 collision성 경계 사례를 별도 시험. |
| 8 | 실행 시작 뒤 oracle 또는 snapshot이 바뀌었는데 시작 전 digest만 evidence에 남겨 결과가 현재 상태의 증거처럼 재사용된다. | executor pre/post digest capture, evidence record identity, reducer currentness predicate 조회 필요 | 거짓신호 | **계획 수정:** pre/post oracle·projected digest를 모두 기록하고 불일치면 condition met 금지. mid-run mutation fixture 필수. |
| 9 | 명령이 exit 0이지만 `EXPECT`를 실제로 관찰하지 못했는데 성공으로 기록된다. 빈 EXPECT, 잘못된 stream, output cap 이후 marker도 같은 구멍을 만든다. | executor outcome model과 reducer의 positive-control predicate 조회 필요 | 거짓신호 | **완수 조건:** exit 0은 필요조건일 뿐이며 non-empty EXPECT의 실제 관찰 없이는 met 불가. stdout/stderr 정책을 고정. |
| 10 | 명령이 nonzero인데 marker가 출력됐다는 이유로 성공하거나, 종료 직전 marker를 출력한 실패 프로세스가 통과한다. | marker matching과 exit-status 결합 규칙 조회 필요 | 거짓신호 | **완수 조건:** `exit == 0 AND expected marker observed AND no execution fault`를 불변식으로 고정. nonzero/marker fixture 추가. |
| 11 | marker가 부분 문자열, ANSI 처리 후 문자열, stderr, output-cap 경계에서 우연히 맞거나 fixture가 marker를 그대로 echo하여 실행 의미 없이 통과한다. | EXPECT framing/escaping/stream selection 계약 조회 필요 | 거짓신호 | **탐지 추가:** 구조화되거나 엄격히 framing된 관찰 규칙 사용. marker spoof·split-chunk·truncation 시험 추가. |
| 12 | negative control이 등록만 되고 실행되지 않았거나, 실행 결과가 누락됐거나, 대상 목록이 비어 있는데 vacuous truth로 condition met가 된다. | reducer의 negative-control evidence predicate 조회 필요 | 금지역 | **완수 조건:** 기대된 각 negative-control ID마다 현재 digest에 결박된 `executed` evidence가 있어야 하며 empty/unrun/unknown은 met 금지. |
| 13 | negative control이 positive와 실질적으로 같은 입력을 실행하거나 “실패해야 성공”의 극성이 뒤집혀 단순 실행 사실만으로 통과한다. | control identity, expected outcome, reducer polarity 계약 조회 필요 | 거짓신호 | **계획 수정:** evidence에 control role과 expected/observed outcome을 분리 기록. positive/negative 동일 digest·동일 invocation이면 거부. |
| 14 | 이전 round 또는 이전 snapshot의 성공 evidence를 replay해 새 실행 없이 current condition을 만족시킨다. rerun이 새 실행인지 중복 append인지도 구별 못 한다. | evidence identity: round ID, approval ID, attempt ID, oracle/projected digest 조회 필요 | 거짓신호 | **완수 조건:** stale/replayed evidence는 currentness에 기여하지 않음. attempt nonce와 digest 결박, duplicate/reorder fixture 추가. |
| 15 | append-only writer가 두 프로세스의 record를 섞거나 partial line을 남긴다. reducer가 잘린 마지막 줄을 무시하고 이전 성공만 사용하거나 손상 record 일부를 유효하게 읽는다. | writer atomic append, record framing/checksum, reader corruption policy 조회 필요 | 거짓신호 | **계획 수정:** 단일 record atomicity 또는 lock+framing 보장. **완수 조건:** torn/interleaved/truncated log는 명시적 residual/error이며 met 금지. |
| 16 | command는 실제 실행됐지만 evidence append가 실패한다. 자동 재시도가 command까지 다시 실행해 side effect가 중복되거나, 반대로 기록 없음이 단순 미실행처럼 보인다. | executor→writer transaction boundary와 retry/idempotency 계약 조회 필요 | 금지역 | **계획 수정:** 실행 attempt ID를 spawn 전에 정하고 “started/finished/persist-failed”를 구분. 기록 실패 시 임의 재실행 금지 및 수동 판단 가능한 상태 필요. |
| 17 | timeout/output cap 구현이 pipe backpressure로 멈추거나 부모만 종료한다. 특히 Windows에서 descendant가 살아남아 승인 범위 밖에서 계속 프로젝트를 변경한다. | process-group/job-object 생성과 stdout/stderr draining, kill/wait 계약 조회 필요 | 금지역 | **완수 조건:** 세 OS에서 descendant까지 종료·회수됨을 PID/side-effect fixture로 확인. cap 도달 후에도 deadlock 없이 종료 상태 기록. |
| 18 | 격리 fixture가 symlink, 절대 경로, inherited env, PATH, temp-dir 탈출 또는 child process로 실제 프로젝트를 건드린다. “fixture”라는 이름만으로 격리가 보장되지 않는다. | test harness의 cwd/env/filesystem/process isolation 경계 조회 필요 | 금지역 | **완수 조건:** sentinel을 둔 실제 workspace가 모든 시험 뒤 불변임을 검증하고 fixture 외 경로 접근을 의도적으로 시도하는 negative test 추가. |
| 19 | CLI 추가 과정에서 기존 schema 1 JSON에 필드 삭제·의미 변경·순서 의존이 생기거나 기존 exit code가 바뀌어 Python/dashboard가 조용히 오판한다. | 기존 status JSON schema 1, oracle digest, exit transition golden contract 및 Python/dashboard consumers 조회 필요 | 실패 | **완수 조건:** 기존 golden JSON/exit vectors를 byte 또는 semantic compatibility 기준으로 고정. 새 필드는 additive인지 명시하고 최소 두 consumer contract test 수행. |
| 20 | 공통 reducer/CLI 정리 중 hook unknown-input fail-open이 fail-closed로 바뀌거나 Stop 처리까지 암묵적으로 확장된다. 반대로 승인 검사 경로가 hook fail-open을 재사용해 command 실행까지 허용할 수도 있다. | hook transport unknown-input 분기와 command approval gate의 코드 공유 경계 조회 필요 | 금지역 | **계획 수정:** 두 정책을 별도 타입/entry point로 분리. **완수 조건:** unknown hook input은 기존 결과 유지, unknown command approval은 절대 spawn하지 않으며 Stop 동작은 diff 0. |
| 21 | 세 OS CI가 테스트 이름만 존재하고 Windows process-tree 시험이 skip/xpass되거나 shell-dependent fixture가 해당 플랫폼에서 실행되지 않아 “green”이 동일 계약을 증명하지 못한다. | CI matrix, skip/ignore 목록, 플랫폼별 test discovery 및 gate 소유자 조회 필요 | 거짓신호 | **완수 조건:** 핵심 attack population별 테스트 실행 건수를 OS마다 산출하고 skip 0을 gate로 설정. 원리상 측정 불가 항목만 명시적 residual 허용. |

가장 먼저 계획에 박아야 할 축은 네 가지다: 승인 레코드의 완전한 결박 키, spawn 전후 currentness 재검증, evidence 손상 시 fail-closed, negative-control의 비공허·현재성·극성 불변식이다. 이 넷이 불명확하면 executor와 CLI를 먼저 만들어도 금지역과 거짓 성공을 구분할 수 없다.

## 내가 기각한 것

없음.

원문 전문:

> 역할: pal-premortem-sweeper. 원 의도와 계획만 보고 실패 시나리오를 독립적으로 1라운드 생성하라. 대화 기록·직전 회차 산출·현재 구현 설명은 보지 마라.
>
> 원 의도: 읽기 전용 round status reducer 위에 사용자 승인된 command oracle 실행, projected content-tree currentness, append-only evidence 기록, 실행된 negative-control evidence 없이는 condition met 불가를 세운다. pal round approve/verify CLI를 만들고 세 OS에서 동일 계약을 지키며 기존 schema 1/oracle digest/status JSON/exit 전이를 임의 변경하지 않는다. 사용자 승인 없이 실제 프로젝트 명령/훅 실행 금지, 시험은 격리 fixture와 explicit approval fixture만 사용. hook transport의 unknown-input fail-open은 유지하고 Stop은 이번 범위 밖이다.
>
> 계획 순서: external approval store→projected digest→executor timeout/output/process-tree→append-only writer→CLI→negative-control reducer→최소 Python/dashboard 호환. 공격 모집단: unapproved, exit0/no EXPECT, nonzero/marker, PATH/CWD/shell drift, timeout/output cap, mid-run oracle/snapshot drift, append failures/partial line, rerun/stale, Windows tree cleanup, unrun negative-control false pass. 종료에는 actual isolated fixture effect, independent review, local full tests, three-OS CI, ADR/gate/graph binding/report/#97 close가 필요하다.
>
> 반환 형식: 번호 있는 시나리오 표. 각 행에 정확한 실패, 코드/계약 좌표(알 수 없으면 필요한 조회), 해악도(금지역/실패/거짓신호/미관), 처분 권고(계획 수정/탐지 추가/완수 조건/수용 사유). 억지로 없음이라 하지 말되 1라운드 상한에서 끝내라. 원문 전문을 최종 응답에 남겨라.
