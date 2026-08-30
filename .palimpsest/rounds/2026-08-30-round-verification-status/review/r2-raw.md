# 독립 리뷰 R2

검토 범위는 잠긴 `intent.md`와 지정된 산출물, 그리고 A9의 외부 CI 증거로 한정했다.
대화 기록과 기존 리뷰의 결론에는 의존하지 않았고 파일도 수정하지 않았다.

## 합격선 판정

| 조건 | 판정 | 근거 |
|---|---|---|
| A1 | 통과 | intent 조건 ID 밖의 oracle을 거부하고 미등록 조건을 `unregistered`로 환원한다. |
| A2 | 통과 | 닫힌 event 집합과 필드별 타입·값·경로 검증이 있다. |
| A3 | 통과 | schema 선두·단일성과 oracle 전 evidence 금지를 검증한다. |
| A4 | 대조불가 | 구현은 첫 oracle을 `pending`으로 두고 오류에는 별도 invalid outcome을 쓰지만 잠긴 의도 문구는 이를 다르게 말했다. |
| A5 | 통과 | status는 원장과 intent를 읽기만 하며 부작용 부재 시험이 있다. |
| A6 | 통과 | 비종료 원장 후보의 0·1·다수 분기를 구현하고 시험한다. |
| A7 | 통과 | Rust 조건 파서와 보존한 Python golden fixture가 바이트 동일하다. |
| A8 | 통과 | JSON과 사람 출력이 같은 `StatusView`를 렌더링한다. |
| A9 | 통과 | GitHub Actions run 33317281903에서 ubuntu·macOS·Windows와 상호운용 job이 모두 성공했다. |
| A10 | 통과 | 조건 문법은 `pal-intent`, 원장과 상태 기계는 `pal-cli`가 소유하며 `xtask`는 `pal-intent`에 직접 의존한다. |

## 발견

| # | 발견 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 근거(명령·출력) |
|---|---|---|---|---|---|---|---|
| R2-01 | 첫 oracle 뒤 evidence 이력이 없는 조건의 `pending` 전이가 잠긴 의도의 stale 축약문과 충돌한다. | 원의도 | 참 | 실패 | A4 | `.palimpsest/rounds/2026-08-30-round-verification-status/intent.md:66` | reducer와 시험은 첫 등록을 pending으로 고정하지만 의도 문장은 모든 oracle 뒤 무증거를 stale로 읽히게 했다. |
| R2-02 | 성공 상태 계약에 `invalid`가 들어 있고 코드에는 생성되지 않는 `VerificationState::Invalid`가 남아 오류 outcome과 상태를 이중으로 표현한다. | 저장소 | 참 | 거짓신호 | A4 | `crates/pal-cli/src/round/status.rs:31` | reducer의 오류는 별도 `StatusOutcome::Invalid`와 exit 2로 반환되고 성공 view는 이 variant를 만들지 않는다. |
| R2-03 | A9의 외부 CI는 통과했지만 잠긴 의도와 게이트 판정은 아직 미측정으로 기록돼 산출 증거와 회차 기록이 갈렸다. | 회차기록 | 참 | 거짓신호 | A9 | `docs/gates/round-verification-status.md:35` | run 33317281903의 세 OS 및 양방향 상호운용 job이 모두 success였다. |

## 내가 기각한 것

- stale 뒤 현재 digest의 evidence가 다시 `met`이 되는 것은 append-only 관측을 현재 oracle에 대조하는 계약과 맞는다.
- 원장에 조건 문장을 복제하지 않고 digest만 싣는 것은 drift를 막는 잠긴 schema와 맞는다.
- Python wrapper의 PATH fallback과 `PAL_BIN` 우선순위는 시험으로 고정됐다.
- `xtask`가 `pal-cli`를 거치지 않고 `pal-intent`를 직접 쓰므로 금지된 의존 방향은 생기지 않았다.
- 사람 출력과 JSON에 별도 reducer가 필요하다는 우려는 단일 `StatusView` 소비 구조로 기각했다.

## 미측정 목록

없음.

## 종료 가능 여부

R2-01~R2-03을 정정하고 그 처분을 커밋 좌표에 결박하면, 두 차례 독립 리뷰 상한과 모든
합격선을 충족하므로 종료 가능하다.
