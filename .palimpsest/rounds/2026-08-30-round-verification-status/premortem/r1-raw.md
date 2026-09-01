# 사전부검 R1 원 반환문

## 발견

### `pal-intent/src/lib.rs` 소유 좌표 누락
- 어떻게 실패하나: 새 모듈이 노출되지 않아 compile 실패 가능
- 어디가 걸리나: `crates/pal-intent/src/lib.rs`
- 획득: 조회
- 모집단: 저장소
- 유효성: 참
- 해악도: 실패
- 대상: 계획대상
- 얼마나 아픈가: 한 공개 좌표와 compile gate가 걸린다

### 원장 없음과 intent 없음 판정 우선순위 충돌
- 어떻게 실패하나: round 없음·디렉터리만·intent만·원장만 입력이 서로 다른 code를 낼 수 있다
- 어디가 걸리나: `crates/pal-cli/src/round/status.rs`
- 획득: 추정
- 모집단: 원의도
- 유효성: 참
- 해악도: 실패
- 대상: 계획대상
- 얼마나 아픈가: resolver와 오류 code 계약이 걸린다

### 같은 내용 oracle 재등록이 과거 evidence를 되살림
- 어떻게 실패하나: 같은 digest의 oracle을 append하면 앞 evidence가 다시 current가 될 수 있다
- 어디가 걸리나: `crates/pal-cli/src/round/ledger.rs`
- 획득: 추정
- 모집단: 원의도
- 유효성: 참
- 해악도: 실패
- 대상: 계획대상
- 얼마나 아픈가: stale 판정의 시간 순서가 걸린다

### current oracle과 다른 digest의 후행 evidence 규칙이 갈림
- 어떻게 실패하나: 현재 digest evidence 뒤에 old digest evidence가 오면 met과 stale 구현이 갈릴 수 있다
- 어디가 걸리나: `crates/pal-cli/src/round/ledger.rs`
- 획득: 추정
- 모집단: 원의도
- 유효성: 참
- 해악도: 실패
- 대상: 계획대상
- 얼마나 아픈가: 마지막 event 규칙 한 자리가 걸린다

### terminal 충돌을 자동 해소가 조용히 숨김
- 어떻게 실패하나: report와 folded가 함께 있는 round가 후보에서 빠져 invalid를 숨길 수 있다
- 어디가 걸리나: `crates/pal-cli/src/round/status.rs`
- 획득: 추정
- 모집단: 원의도
- 유효성: 참
- 해악도: 실패
- 대상: 계획대상
- 얼마나 아픈가: 명시·자동 resolver 둘이 걸린다

### 읽기 전용 계약이 직접 공격되지 않음
- 어떻게 실패하나: oracle check의 부작용 실행이나 입력 파일 수정이 생겨도 상태 값 시험만 통과할 수 있다
- 어디가 걸리나: `crates/pal-cli/tests/round_status.rs`
- 획득: 추정
- 모집단: 자기장치
- 유효성: 참
- 해악도: 금지역
- 대상: 계획자신
- 얼마나 아픈가: status 안전 경계 전체가 걸린다

### 크기와 구문 경계가 RED에서 빠짐
- 어떻게 실패하나: 8 MiB 파일·64 KiB 행·32 KiB 문자열·빈 행·partial line·숫자 경계·duplicate key가 조용히 통과할 수 있다
- 어디가 걸리나: `crates/pal-cli/src/round/ledger.rs`
- 획득: 추정
- 모집단: 원의도
- 유효성: 참
- 해악도: 실패
- 대상: 계획대상
- 얼마나 아픈가: schema parser 경계 전반이 걸린다

### 사람 출력과 JSON의 단일 reducer가 수동 관찰에 의존함
- 어떻게 실패하나: renderer가 각자 상태를 다시 계산해도 일부 fixture만 같을 수 있다
- 어디가 걸리나: `crates/pal-cli/src/round/status.rs`
- 획득: 추정
- 모집단: 자기장치
- 유효성: 참
- 해악도: 거짓신호
- 대상: 계획자신
- 얼마나 아픈가: 출력 표면 둘과 view model이 걸린다

### 세 OS 합격선이 단순 CI 성공으로 축소됨
- 어떻게 실패하나: 다른 시험이 세 job에서 초록이어도 동일 fixture의 enum이 다를 수 있다
- 어디가 걸리나: `crates/pal-cli/tests/round_status.rs`
- 획득: 추정
- 모집단: 자기장치
- 유효성: 참
- 해악도: 거짓신호
- 대상: 계획자신
- 얼마나 아픈가: 마지막 SHA의 세 named checks가 걸린다

### Python wrapper가 PATH fallback을 놓침
- 어떻게 실패하나: 시험용 PAL_BIN만 통과하고 설치본 PATH 해소가 실패할 수 있다
- 어디가 걸리나: `.claude/skills/round/bin/record.py`
- 획득: 추정
- 모집단: 저장소
- 유효성: 참
- 해악도: 실패
- 대상: 계획대상
- 얼마나 아픈가: 저장소본과 설치본 소비 경로가 걸린다

### malformed intent의 status code가 잠기지 않음
- 어떻게 실패하나: 같은 오류가 구현마다 resolve_error나 새 code로 갈릴 수 있다
- 어디가 걸리나: `crates/pal-cli/src/round/status.rs`
- 획득: 추정
- 모집단: 원의도
- 유효성: 참
- 해악도: 실패
- 대상: 계획대상
- 얼마나 아픈가: JSON 소비자 계약이 걸린다

### xtask 변경 범위가 모호함
- 어떻게 실패하나: 조건 parser 전환이 gate parser까지 번져 불필요한 회귀를 만들 수 있다
- 어디가 걸리나: `xtask/src/main.rs`
- 획득: 조회
- 모집단: 원의도
- 유효성: 참
- 해악도: 실패
- 대상: 계획대상
- 얼마나 아픈가: 원장 둘 대조의 condition read 한 자리만 옮겨야 한다

## 내가 기각한 것

없음.

새 범주: 없음
