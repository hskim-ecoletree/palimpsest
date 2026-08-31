### Stop을 기본 등록에서 빼야 한다

- 모집단: 원의도
- 유효성: 거짓
- 해악도: 금지역
- 어디가 걸리나: `crates/pal-cli/src/hook/policy.rs`

등록이 곧 활성화라는 위험을 피하려고 Stop을 기본 등록에서 빼면 소유자가 요구한 등록과
정책 활성화의 분리를 검증할 수 없다. 단일 catalog에는 등록하되 private activation이 없으면
반드시 통과시켜야 한다.

### ledger 없는 열린 회차가 자동 해소에서 빠진다

- 모집단: 원의도
- 유효성: 참
- 해악도: 금지역
- 어디가 걸리나: `crates/pal-cli/src/round/status.rs`

`verification.log`가 없는 열린 회차를 active 검색이 놓치면 Stop이 통과한다. 활성화가 명시한
slug를 읽고 원장이 없는 open round도 불완전으로 차단해야 한다.

### 프로젝트 안 counter가 projected digest를 오염시킨다

- 모집단: 자기장치
- 유효성: 참
- 해악도: 금지역
- 어디가 걸리나: `crates/pal-cli/src/round/verify.rs`

무진행 counter를 프로젝트 안에 쓰면 schema 2 projected digest가 바뀌어 현재 evidence를
스스로 stale로 만든다. operational state는 projected tree 밖 private store에 둬야 한다.

### raw ledger와 StatusView 어느 한쪽만 digest하면 의미 또는 표현을 잘못 센다

- 모집단: 자기장치
- 유효성: 참
- 해악도: 실패
- 어디가 걸리나: `crates/pal-cli/src/round/status.rs`

raw ledger digest는 순서·timestamp를 진행으로 오인하고 StatusView만 쓰면 evidence 의미 일부가
사라진다. typed semantic projection을 두고 표현 변화와 의미 변화를 분리해야 한다.

### digest 변화만 진행으로 보면 regression과 진동이 상한을 피한다

- 모집단: 자기장치
- 유효성: 참
- 해악도: 실패
- 어디가 걸리나: `crates/pal-cli/src/round/stop.rs`

digest가 달라졌다는 사실만 진행으로 보면 A→B→A 진동과 regression이 자기 상한을 영원히
피한다. 단조 진척 순위와 bounded seen state가 필요하다.

### settings 등록 존재를 activation으로 읽으면 상태가 갈린다

- 모집단: 원의도
- 유효성: 참
- 해악도: 실패
- 어디가 걸리나: `crates/pal-cli/src/install/manifest.rs`

settings hook 존재로 activation을 추론하면 매니페스트·설정 손상과 사용자 항목이 섞인다.
등록과 별개인 portable private activation record를 둬야 한다.

### enable update disable 왕복에서 등록 또는 사용자 hook을 잃을 수 있다

- 모집단: 자기장치
- 유효성: 추정
- 해악도: 실패
- 어디가 걸리나: `crates/pal-cli/src/install.rs`

disable/update가 exact owned entry 규율을 우회하면 사용자 hook을 지우거나 activation을 잃는다.
enable→update→disable 왕복에서 등록과 사용자 항목을 함께 검증해야 한다.

## 내가 기각한 것

없음 — 위 첫 발견이 검토한 대안의 기각까지 포함하며, 원 반환에서 숨긴 별도 항은 없다.
