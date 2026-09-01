### 로컬 HEAD와 PR 최종 SHA가 다른데 로컬 검사만 통과한다

- 모집단: 회차기록
- 유효성: 참
- 해악도: 금지역
- 어디가 걸리나: `.palimpsest/rounds/2026-09-01-agent-laziness-merge-evaluation/GATES.md`

로컬 HEAD·PR `headRefOid`·dirty 상태를 함께 고정해야 한다.

### 최신 main과 합친 결과는 실패하지만 오래된 base 위 브랜치만 검사한다

- 모집단: 저장소
- 유효성: 참
- 해악도: 금지역
- 어디가 걸리나: `.palimpsest/rounds/2026-09-01-agent-laziness-merge-evaluation/state.md`

`origin/main`을 먼저 합친 결과에서 전 검사를 다시 해야 한다.

### doctor JSON의 빈 배열 하나만 찾아 다른 위반을 놓친다

- 모집단: 자기장치
- 유효성: 참
- 해악도: 실패
- 어디가 걸리나: `.palimpsest/rounds/2026-09-01-agent-laziness-merge-evaluation/GATES.md`

`answer`의 네 오류 배열과 checked invariant를 구조적으로 순회해야 한다.

### 등록·승인·실행·현재성·중단 중 하나가 직접 자극되지 않는다

- 모집단: 원의도
- 유효성: 참
- 해악도: 실패
- 어디가 걸리나: `.palimpsest/rounds/2026-09-01-agent-laziness-merge-evaluation/intent.md`

각 경계의 정상·우회·낡음·중단 시험 이름과 결과를 대조해야 한다.

### 기존 실행 원장을 실제로 재실행하지 않는다

- 모집단: 회차기록
- 유효성: 참
- 해악도: 실패
- 어디가 걸리나: `.palimpsest/rounds/2026-09-01-agent-laziness-merge-evaluation/GATES.md`

실행 계획 원장의 명령과 현재 저장소 harness를 다시 실행해야 한다.

### 독립 검토가 대상 SHA·근거·해악도·병합 차단 없이 끝난다

- 모집단: 회차기록
- 유효성: 참
- 해악도: 거짓신호
- 어디가 걸리나: `.palimpsest/rounds/2026-09-01-agent-laziness-merge-evaluation/review/r1-raw.md`

독립 반환문에 대상·좌표·근거·해악도·병합 차단 여부를 보존해야 한다.

### 이전 SHA나 일부 플랫폼 CI를 최종 CI로 대체한다

- 모집단: 저장소
- 유효성: 참
- 해악도: 금지역
- 어디가 걸리나: `.palimpsest/rounds/2026-09-01-agent-laziness-merge-evaluation/GATES.md`

PR 최종 `headRefOid`에 붙은 정확한 일곱 check의 완료·성공을 API로 확인해야 한다.

### PR 상태만 merged이고 원격 main이 실제 결과를 포함하지 않는다

- 모집단: 저장소
- 유효성: 참
- 해악도: 금지역
- 어디가 걸리나: `.palimpsest/rounds/2026-09-01-agent-laziness-merge-evaluation/GATES.md`

merge commit과 `origin/main`의 조상 관계를 확인해야 한다.

## 내가 기각한 것

없음. 잘못 합격하는 경로로서 전부 유효했다.
