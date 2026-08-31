### session ID만으로 새 시도와 replay를 구분할 수 없다

- 모집단: 원의도
- 유효성: 참
- 해악도: 금지역
- 어디가 걸리나: `crates/pal-cli/src/install/hooks.rs`

session ID만으로는 같은 세션의 새 시도와 stale replay를 못 가른다. transcript의 내용 hash를
event identity에 포함하되 원문과 절대 경로는 보존하지 않아야 한다.

### create_new lock 잔해가 crash 뒤 영구 잠금을 만든다

- 모집단: 자기장치
- 유효성: 참
- 해악도: 금지역
- 어디가 걸리나: `crates/pal-cli/src/round/verify.rs`

create_new lock 잔해는 crash 뒤 영구 잠금이 될 수 있다. bounded wait와 나이로 판정하는 stale
recovery, 그리고 원자 replace를 함께 둬야 한다.

### counter key가 project round session을 모두 가르지 못할 수 있다

- 모집단: 자기장치
- 유효성: 참
- 해악도: 실패
- 어디가 걸리나: `crates/pal-cli/src/round/stop.rs`

상태 key가 round만이면 프로젝트가 충돌하고 session만이면 같은 session이 충돌한다. portable
project+round identity로 store를 가르고 event hash로 replay를 가려야 한다.

### verify와 status 동시 읽기가 혼합 snapshot을 만들 수 있다

- 모집단: 저장소
- 유효성: 추정
- 해악도: 실패
- 어디가 걸리나: `crates/pal-cli/src/round/status.rs`

status read 중 verify가 append하면 혼합 snapshot을 볼 수 있다. bounded reread로 동일한 semantic
identity가 연속 관찰됐을 때만 판정해야 한다.

### malformed Stop과 기존 unknown fail-open의 dispatch 순서가 충돌한다

- 모집단: 저장소
- 유효성: 참
- 해악도: 금지역
- 어디가 걸리나: `crates/pal-cli/src/hook.rs`

malformed를 전역 fail-closed로 바꾸면 기존 unknown/SubagentStop 계약을 깨고, 전역 fail-open이면
활성 Stop이 빠져나간다. event→reentry→activation→active payload/state 순서로 dispatch해야 한다.

### reentry guard가 상태 조회 뒤면 손상 상태에서 무한 루프가 난다

- 모집단: 원의도
- 유효성: 참
- 해악도: 금지역
- 어디가 걸리나: `crates/pal-cli/src/hook/policy.rs`

reentry guard가 config/status/lock보다 뒤면 손상 상태에서 무한 루프가 난다. event를 안 뒤에는
`stop_hook_active=true`를 다른 판정보다 먼저 처리해야 한다.

### cap handoff가 round 정본을 쓰면 거짓 종료가 된다

- 모집단: 원의도
- 유효성: 참
- 해악도: 금지역
- 어디가 걸리나: `crates/pal-cli/src/round/stop.rs`

자기 상한이 intent/verification/report/folded를 건드리거나 기록 실패 후 pass하면 거짓 종료다.
operational state에 truthful blocked handoff만 쓰고 기록 실패는 계속 block해야 한다.

### doctor probe는 active block pass를 재지 않는다

- 모집단: 저장소
- 유효성: 참
- 해악도: 실패
- 어디가 걸리나: `crates/pal-cli/src/install/hooks.rs`

doctor probe는 transport만 재므로 실제 active block/pass를 보이지 못한다. 실제 Claude Code의
격리 세션에서 activation 전후 block/pass와 reentry를 관측해야 한다.

### catalog 밖 이벤트 목록이 남으면 #86이 재발한다

- 모집단: 저장소
- 유효성: 참
- 해악도: 실패
- 어디가 걸리나: `crates/pal-cli/src/install/layout.rs`

catalog 밖에 이벤트 문자열 목록이 남으면 #86의 drift가 재발한다. dispatch와 install 렌더링이
한 catalog를 소비하고 집합 동치 구조 시험이 있어야 한다.

### block reason이 절대 경로 command raw output을 누출할 수 있다

- 모집단: 자기장치
- 유효성: 추정
- 해악도: 거짓신호
- 어디가 걸리나: `crates/pal-cli/src/hook.rs`

block reason에 절대 경로·oracle command·raw output을 넣으면 정보가 샌다. slug와 bounded 상태
요약만 내고 비밀·경로 음성 대조를 둬야 한다.

## 내가 기각한 것

없음.
