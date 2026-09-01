# 독립 리뷰 R1

검토 범위는 잠긴 `intent.md`와 지정된 산출물로 한정했다. 대화 기록, `state.md`, 계획 문서,
사전부검 및 기존 리뷰는 읽지 않았고 파일도 수정하지 않았다.

## 합격선 판정

| 조건 | 판정 | 근거 |
|---|---|---|
| A1 | 통과 | oracle ID를 intent ID 집합과 대조하고 밖의 ID를 거부한다. 미등록 조건은 `unregistered`다. |
| A2 | 통과 | 닫힌 event와 타입·ID·digest·문자열·cwd 검증이 있다. |
| A3 | 통과 | schema는 첫 행에 정확히 하나여야 하고 oracle 없는 evidence는 transition 오류다. |
| A4 | 대조불가 | 원장에는 조건 문장이 없고 digest 구현과 고정 벡터는 있으나 잠긴 의도가 실제 바이트 계약을 싣지 않았다. |
| A5 | 통과 | status 경로는 읽기만 하고 부작용 시험이 통과했다. |
| A6 | 통과 | 비종료 원장 디렉터리만 후보이며 0/1/다수 분기가 명시됐다. |
| A7 | 통과 | Rust parser와 전환 전 Python parser 출력이 바이트 동일했다. |
| A8 | 통과 | JSON과 사람 출력이 하나의 `StatusView`를 받는다. |
| A9 | 미측정 | 로컬 macOS 실행만으로 ubuntu·Windows 결과를 승인할 수 없다. |
| A10 | 통과 | 소유권과 의존 방향이 잠긴 계약과 같다. |

## 발견

| # | 발견 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 근거(명령·출력) |
|---|---|---|---|---|---|---|---|
| R1-01 | A4가 참조하는 잠긴 schema와 digest 직렬화가 잠긴 의도 자체에 없어 독립 대조가 불가능하다. | 원의도 | 참 | 거짓신호 | A4 | `.palimpsest/rounds/2026-08-30-round-verification-status/intent.md:37` | 허용된 유일한 의도 문서에서 구체 계약을 찾을 수 없었다. |
| R1-02 | A9의 세 플랫폼 동등성을 현재 산출물이 직접 측정하지 않는다. | 자기장치 | 참 | 거짓신호 | A9 | `crates/pal-cli/tests/round_status.rs:92` | 로컬 시험은 통과했으나 ubuntu·Windows 실행 증거가 없다. |
| R1-03 | 구현 전 RED 관측은 역사적 요구인데 현재 산출물에는 그 실행 결과가 없다. | 회차기록 | 참 | 거짓신호 | A7 | `.palimpsest/rounds/2026-08-30-round-verification-status/intent.md:45` | 현재 초록 시험만으로 구현 전 실패를 복원할 수 없다. |

## 내가 기각한 것

- `VerificationState::Invalid`가 reducer에서 생성되지 않는 점은 오류 봉투와 exit 2가 별도로 진다.
- stale 이후 현재 digest의 새 evidence가 met으로 회복되는 것은 append-only 전이와 맞는다.
- Python golden은 전환 전 parser와 직접 대조해 같았다.
- 두 renderer가 같은 `StatusView`를 소비하므로 사람 출력 시험의 별도 JSON 문자열 대조는 필요 없다.

## 미측정 목록

없음 — 미측정과 대조불가는 발견 R1-01~R1-03에 이미 열거했다.

## 종료 가능 여부

현재 증거만으로는 종료 불가다. R1-01과 R1-03은 회차 산출물로 닫고 R1-02는 마지막 SHA의
세 플랫폼 CI로만 닫아야 한다.
