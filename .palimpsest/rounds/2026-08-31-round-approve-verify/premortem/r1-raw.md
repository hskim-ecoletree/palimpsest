### 승인 결박 축 누락

- 모집단: 원의도
- 유효성: 추정
- 해악도: 금지역
- 어디가 걸리나: `crates/pal-cli/src/round/`의 external approval store key/value schema 및 approval lookup predicate — 정확한 파일 조회 필요

승인이 command 문자열에만 결박되고 repository/worktree, projected digest, oracle digest, CWD가 빠지면 승인 뒤 다른 내용에 같은 명령이 실행된다. 승인을 `repo identity + CWD + canonical command + oracle digest + projected digest`에 결박하고, 어느 축 하나만 달라도 spawn 전에 거부해야 한다.

### 승인 레코드 위조·부분 기록

- 모집단: 자기장치
- 유효성: 추정
- 해악도: 금지역
- 어디가 걸리나: `crates/pal-cli/src/round/`의 approval store 소유권·권한·integrity·atomic-write 계약 — 정확한 파일 조회 필요

외부 store의 record가 위조·교체·부분 기록되거나 다른 사용자·프로세스가 만든 승인을 신뢰할 수 있다. malformed, partial, untrusted-owner record는 fail-closed해야 하며 임의 fixture가 승인 파일을 주입해도 실행되지 않아야 한다.

### approve와 verify의 command 해석 불일치

- 모집단: 자기장치
- 유효성: 추정
- 해악도: 금지역
- 어디가 걸리나: `crates/pal-cli/src/round/`의 CLI parser → approval serializer → executor spawn API — 정확한 파일 조회 필요

같은 command 문자열이 POSIX shell, Windows quoting, argv 실행 여부에 따라 다른 프로그램·인수로 해석될 수 있다. shell 문자열보다 명시적 argv 계약을 우선하고 shell 사용 여부도 승인 자료에 포함하며 세 OS quoting corpus를 검증해야 한다.

### PATH·CWD·환경 drift

- 모집단: 원의도
- 유효성: 추정
- 해악도: 금지역
- 어디가 걸리나: `crates/pal-cli/src/round/`의 executable resolution, CWD·environment capture 및 approval binding — 정확한 파일 조회 필요

승인 시점과 실행 시점의 PATH, CWD, environment가 달라 동일 command가 다른 executable이나 config를 선택할 수 있다. resolved executable identity와 CWD가 바뀌면 거부하고 PATH shadowing, relative executable, case 차이를 시험해야 한다.

### 승인 검사와 spawn 사이 TOCTOU

- 모집단: 자기장치
- 유효성: 추정
- 해악도: 금지역
- 어디가 걸리나: `crates/pal-cli/src/round/`의 approval consume·validate·spawn 경계 — 정확한 파일 조회 필요

승인 검사와 process spawn 사이 승인 폐기·교체 또는 content 변경이 일어날 수 있다. 승인 generation을 spawn 직전에 재검증하고 one-shot 승인은 원자적으로 consume하며 race fixture를 추가해야 한다.

### projected digest의 tree 구성요소 누락

- 모집단: 자기장치
- 유효성: 추정
- 해악도: 거짓신호
- 어디가 걸리나: `crates/pal-cli/src/round/`의 projected-tree membership 및 canonical serialization — 정확한 파일 조회 필요

untracked file, symlink target/type, executable bit, 삭제 예정 파일, ignore 경계 중 일부가 digest에서 빠지면 실제 실행 입력이 달라도 current로 판정한다. 포함·제외 집합을 계약화하고 각 항목의 단일 변이 시험을 추가해야 한다.

### 세 OS의 projected digest 불일치

- 모집단: 자기장치
- 유효성: 추정
- 해악도: 실패
- 어디가 걸리나: `crates/pal-cli/src/round/`의 path·content canonicalization 계약 — 정확한 파일 조회 필요

CRLF/LF, Unicode normalization, 경로 separator·case·reserved name 차이로 같은 tree에 다른 digest가 나오거나 다른 tree에 같은 digest가 나올 수 있다. 공통 golden vectors를 세 OS에서 검증하고 collision성 경계 사례를 시험해야 한다.

### 실행 중 oracle·snapshot drift

- 모집단: 원의도
- 유효성: 추정
- 해악도: 거짓신호
- 어디가 걸리나: `crates/pal-cli/src/round/`의 executor pre/post digest capture, evidence identity 및 reducer currentness predicate — 정확한 파일 조회 필요

실행 시작 뒤 oracle 또는 snapshot이 바뀌어도 시작 전 digest만 evidence에 남으면 결과가 현재 상태의 증거처럼 재사용된다. pre/post oracle·projected digest를 모두 기록하고 불일치 시 condition met를 금지하며 mid-run mutation fixture를 두어야 한다.

### exit 0이지만 EXPECT 미관찰

- 모집단: 원의도
- 유효성: 추정
- 해악도: 거짓신호
- 어디가 걸리나: `crates/pal-cli/src/round/`의 executor outcome model 및 positive-control reducer predicate — 정확한 파일 조회 필요

명령이 exit 0이어도 `EXPECT`를 실제로 관찰하지 못했거나 EXPECT가 비었거나 잘못된 stream을 봤다면 성공이 아니다. exit 0은 필요조건일 뿐이며 non-empty EXPECT의 실제 관찰과 명시적인 stdout/stderr 정책이 필요하다.

### nonzero인데 marker로 성공

- 모집단: 원의도
- 유효성: 추정
- 해악도: 거짓신호
- 어디가 걸리나: `crates/pal-cli/src/round/`의 marker matching과 exit-status 결합 규칙 — 정확한 파일 조회 필요

명령이 nonzero인데 marker가 출력됐다는 이유로 성공하거나 종료 직전 marker를 출력한 실패 프로세스가 통과할 수 있다. `exit == 0 AND expected marker observed AND no execution fault`를 불변식으로 고정하고 nonzero/marker fixture를 추가해야 한다.

### EXPECT marker spoof

- 모집단: 자기장치
- 유효성: 추정
- 해악도: 거짓신호
- 어디가 걸리나: `crates/pal-cli/src/round/`의 EXPECT framing·escaping·stream-selection 계약 — 정확한 파일 조회 필요

marker가 부분 문자열, ANSI 처리 뒤 문자열, stderr, output-cap 경계에서 우연히 맞거나 fixture가 marker를 그대로 echo하여 실행 의미 없이 통과할 수 있다. 엄격히 framing된 관찰 규칙과 marker spoof, split-chunk, truncation 시험이 필요하다.

### 미실행 negative control의 공허한 met

- 모집단: 원의도
- 유효성: 추정
- 해악도: 금지역
- 어디가 걸리나: `crates/pal-cli/src/round/status.rs`의 negative-control evidence predicate

negative control이 등록만 되고 실행되지 않았거나 결과가 누락됐거나 대상 목록이 비었는데 vacuous truth로 condition met가 될 수 있다. 각 negative-control ID마다 현재 digest에 결박된 `executed` evidence가 있어야 하며 empty, unrun, unknown은 met가 아니어야 한다.

### negative control의 동일 입력·극성 반전

- 모집단: 자기장치
- 유효성: 추정
- 해악도: 거짓신호
- 어디가 걸리나: `crates/pal-cli/src/round/`의 control identity, expected outcome 및 reducer polarity — 정확한 파일 조회 필요

negative control이 positive와 실질적으로 같은 입력을 실행하거나 “실패해야 성공”의 극성이 뒤집혀 실행 사실만으로 통과할 수 있다. evidence에 control role과 expected/observed outcome을 분리하고 동일 digest·동일 invocation이면 거부해야 한다.

### stale evidence replay와 rerun 혼동

- 모집단: 원의도
- 유효성: 추정
- 해악도: 거짓신호
- 어디가 걸리나: `crates/pal-cli/src/round/`의 evidence round ID, approval ID, attempt ID 및 oracle/projected digest — 정확한 파일 조회 필요

이전 round나 snapshot의 성공 evidence가 replay되어 새 실행 없이 current condition을 만족하거나 rerun과 중복 append를 구별하지 못할 수 있다. stale·replayed evidence는 currentness에 기여하지 않아야 하며 attempt nonce와 digest를 결박해야 한다.

### 동시 append와 partial line

- 모집단: 자기장치
- 유효성: 추정
- 해악도: 거짓신호
- 어디가 걸리나: `crates/pal-cli/src/round/`의 append-only writer atomicity, record framing/checksum 및 reader corruption policy — 정확한 파일 조회 필요

두 프로세스의 record가 섞이거나 partial line이 남았을 때 reducer가 잘린 행을 무시하고 이전 성공만 사용하거나 손상 record 일부를 유효하게 읽을 수 있다. torn, interleaved, truncated log는 명시적 residual/error이며 met를 금지해야 한다.

### 실행 뒤 evidence append 실패

- 모집단: 자기장치
- 유효성: 추정
- 해악도: 금지역
- 어디가 걸리나: `crates/pal-cli/src/round/`의 executor→writer transaction boundary 및 retry/idempotency 계약 — 정확한 파일 조회 필요

command는 실제 실행됐지만 evidence append가 실패할 수 있다. 자동 재시도가 command까지 다시 실행하면 side effect가 중복된다. attempt ID를 spawn 전에 정하고 `started`, `finished`, `persist-failed`를 구분하며 기록 실패 시 임의 재실행을 금지해야 한다.

### timeout·output cap 이후 descendant 잔존

- 모집단: 자기장치
- 유효성: 추정
- 해악도: 금지역
- 어디가 걸리나: `crates/pal-cli/src/round/`의 process-group/job-object 생성, stdout/stderr draining 및 kill/wait 계약 — 정확한 파일 조회 필요

timeout/output cap 구현이 pipe backpressure로 멈추거나 부모만 종료할 수 있다. 특히 Windows에서 descendant가 살아 승인 범위 밖에서 프로젝트를 계속 변경할 수 있다. 세 OS에서 descendant 종료·회수를 검증하고 cap 이후에도 deadlock 없이 종료 상태를 기록해야 한다.

### 격리 fixture의 실제 프로젝트 탈출

- 모집단: 규약
- 유효성: 추정
- 해악도: 금지역
- 어디가 걸리나: `crates/pal-cli/tests/`의 fixture CWD·environment·filesystem·process isolation 계약 — 정확한 fixture 파일 조회 필요

fixture가 symlink, 절대 경로, inherited environment, PATH, temp-directory 탈출 또는 child process로 실제 프로젝트를 건드릴 수 있다. 실제 workspace sentinel이 모든 시험 뒤 불변임을 검증하고 fixture 외 경로 접근을 의도적으로 시도하는 negative test가 필요하다.

### schema 1·exit·consumer 호환 파괴

- 모집단: 저장소
- 유효성: 추정
- 해악도: 실패
- 어디가 걸리나: `crates/pal-cli/tests/round_status.rs`, schema 1 status JSON·oracle digest·exit transition 계약 및 `.claude/skills/round/bin/` consumers

CLI 추가 과정에서 기존 schema 1 JSON의 필드·의미·순서 의존이나 exit code가 바뀌어 Python/dashboard가 조용히 오판할 수 있다. 기존 golden JSON/exit vectors를 byte 또는 semantic compatibility 기준으로 고정하고 최소 두 consumer contract test를 수행해야 한다.

### hook fail-open과 command approval 경계 혼합

- 모집단: 저장소
- 유효성: 추정
- 해악도: 금지역
- 어디가 걸리나: hook transport unknown-input entry point와 `crates/pal-cli/src/round/` command approval gate — hook 상대 경로 조회 필요

공통 reducer/CLI 정리 중 hook unknown-input fail-open이 fail-closed로 바뀌거나 Stop 처리까지 확장될 수 있다. 반대로 hook fail-open이 command 실행 승인에도 재사용될 수 있다. 두 정책을 별도 타입·entry point로 분리하고 unknown command approval은 절대 spawn하지 않으며 Stop 동작은 바꾸지 않아야 한다.

### 세 OS CI의 실행되지 않은 green

- 모집단: 규약
- 유효성: 추정
- 해악도: 거짓신호
- 어디가 걸리나: `.github/workflows/` CI matrix와 `crates/pal-cli/tests/`의 플랫폼별 test discovery·skip/ignore 계약 — 정확한 workflow 조회 필요

Windows process-tree 시험이 skip/xpass되거나 shell-dependent fixture가 해당 플랫폼에서 실행되지 않아도 세 OS CI가 green일 수 있다. 핵심 공격 모집단별 실제 테스트 실행 건수를 OS마다 산출하고 skip 0을 gate로 두며, 원리상 측정 불가한 항목만 residual로 허용해야 한다.

## 내가 기각한 것

없음.
