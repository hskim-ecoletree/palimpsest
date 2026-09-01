# 상태 — 에이전트 게으름 구현 착수 계획을 잠근다

## 지금 단계

종료. Markdown 계획, 게이트, 종료 보고가 섰다. 구현과 이슈 변경은 잠긴 범위대로 하지 않았다.

## 인터뷰

- 상한: 1라운드.
- 경계: 계획 문서와 필요한 조사까지, 구현은 다음 세션.
- 의도: 다음 세션이 추가 설계 탐색 없이 첫 구현 회차를 열 수 있는 계획.
- 자율: 저장소 읽기와 외부 일차 자료 조사는 허용, 이슈 상태 변경과 구현은 하지 않음.
- 종료: 코드 좌표·미결 결정·첫 RED·검증 명령이 있는 Markdown과 통과한 검산.
- 재고: 비교 문서, #85·#88·#90·#92·#94·#95·#96·#97, 현재 round/hook/xtask 산출.

## 사전부검

- 상한: 1라운드. 계획 초안과 잠긴 의도만 독립 검토한다.
- 완료: 1라운드. 발견 9건을 전부 처분했다.
  - R-01 projected snapshot spike가 첫 회차 범위를 흔듦 → approve+verify 회차로 이동.
  - R-02 첫 reducer의 finding 입력이 정의되지 않음 → 전체 `RoundState` 통합 회차로 이동.
  - R-03 Rust↔Python 자기 대조 → Python 변경 전 golden을 독립 원천으로 등록.
  - R-04 terminal marker가 닫혀 있지 않음 → `report.md`·`folded.md` 둘과 충돌 규칙을 명시.
  - R-05 dependency 방향이 모호함 → #88이 #85·#97을 막는 방향으로 명시.
  - R-06 기준 SHA drift → 관련 경로 제한 diff를 착수 첫 단계에 추가.
  - P-01 상태 enum과 exit가 모호함 → condition/verification/terminal observation 표를 추가.
  - P-02 schema와 전이가 비어 있음 → schema 1 event·필드·전이 규칙을 추가.
  - P-03 결정론/비결정론 분류가 비어 있음 → 첫 판은 command-only, 나머지는 fail closed.

## 실패한 접근

- 독립 리뷰의 첫 제안대로 `xtask → pal-cli` library 의존을 두려 했다. 현재 stack과
  `xtask` 검사는 어떤 크레이트도 제품 표면인 `pal-cli`에 의존하지 못하게 하므로 폐기했다.
  조건 문법은 기존 아래 계층인 `pal-intent`, 원장 reducer는 `pal-cli`로 갈랐다.

## 독립 검토

- 1라운드 완료. A1·A2는 초안에서 반증, A3~A6은 통과였다.
- IR-01 `verification.jsonl`이 기존 finding 전수 검사와 충돌 → `verification.log`로 분리.
- IR-02 oracle ID 양방향 일치와 부분 등록이 충돌 → oracle은 intent ID의 부분집합으로 고정.
- IR-03 Python/Rust 호출 경계가 없음 → `PAL_BIN`, conditions JSON/exit 호환, Python golden을 명시.
- IR-04 `blake3` 직접 의존 좌표 누락 → `pal-cli/Cargo.toml` 소유 파일에 추가.
- IR-05 schema·canonical digest가 불충분 → 크기·필드·타입·전이·바이트 직렬화와 test vector를 잠금.
- IR-06 status JSON 계약이 없음 → 성공·오류·no-active 형태와 안정된 error code를 잠금.
- IR-07 파생 asset 파일을 소유 파일로 오인 → `layout.rs`를 변경 대상에서 drift 검증 좌표로 내림.
- 검토 제안 중 `xtask → pal-cli`는 저장소 불변식과 충돌해 채택하지 않고 `pal-intent` 공유로 정정.
- 2라운드에서 앞 발견 여섯은 닫힘, CLI error code는 부분 닫힘으로 판정됐다. code를
  `invalid_schema | invalid_transition | resolve_error | io_error` 넷으로 완전히 닫았다.
- 불필요한 `pal-cli` lib target이 기존 doctest 설명을 낡게 만든다는 발견을 받아
  `src/lib.rs`를 소유 좌표에서 제거하고 binary `mod round` + black-box 실행 경계를 유지했다.

## 남은 것

- 없음.
