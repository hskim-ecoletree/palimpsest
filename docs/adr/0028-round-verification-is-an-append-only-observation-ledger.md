# ADR-0028 — **회차 검증은 명령 실행기가 아니라 append-only 관측 원장이다**

**상태**: 채택 (2026-08-30) · [#88](https://github.com/hskim-ecoletree/palimpsest/issues/88) 종료 시점에 발행 ·
근거 [ADR-0023](0023-consistent-method-and-result-across-platforms.md) ·
[ADR-0024](0024-an-adapter-that-can-diverge-is-a-second-core.md) ·
[ADR-0025](0025-the-harness-that-reads-the-graph-is-the-same-product.md) ·
판정 [docs/gates/round-verification-status.md](../gates/round-verification-status.md)

## 맥락

`/round`는 완수 조건을 잠그고 검증을 요구했지만, 그 증거의 현재성을 한 자리에서 읽는
기계가 없었다. Python은 Markdown 조건을 읽었고 `xtask`는 종료된 회차의 문서를 대조했지만,
진행 중 회차에서 어느 조건이 미등록·대기·낡음·충족·불충족인지 같은 정의로 답하지 못했다.

실행기나 Stop 훅부터 만들면 각 표면이 자기 완료 정의를 갖는다. 반대로 status가 명령을
실행하면 읽기와 쓰기, 판정과 부작용이 한 경계에 섞인다. 세 OS가 같은 방법과 결과를 내야
하므로 Python parser를 계속 정본으로 둘 수도 없다.

## 결정

회차 검증을 **읽기 전용 reducer가 소비하는 append-only JSONL 원장**으로 둔다.

- 조건 문법과 `ConditionId`의 단일 Rust 정본은 `pal-intent`가 소유한다. Python
  `record.py conditions`와 dashboard는 `PAL_BIN` 또는 PATH의 `pal`을 부르는 호환 표면이다.
- schema 1 원장과 상태 기계는 `pal-cli`의 private module이 소유한다. `pal-cli` library
  target을 만들지 않고 `xtask`는 `pal-intent`에 직접 의존한다.
- 원장은 조건 문장을 복제하지 않는다. oracle의 command·literal·cwd를 닫힌 바이트
  직렬화로 digest하고 evidence는 그 digest를 가리킨다. 시간과 HEAD는 상태 입력이 아니다.
- oracle과 evidence는 덮어쓰지 않고 append한다. 첫 oracle에 evidence 이력이 없으면
  `pending`이고, evidence 뒤 oracle을 재등록하면 같은 digest라도 새 evidence 전까지
  `stale`이다. 마지막 oracle 뒤의 마지막 evidence만 현재 관측이다.
- `pal round status`는 파일을 읽어 하나의 `StatusView`를 만들고 JSON과 사람 출력이 이를
  함께 소비한다. 명령을 실행하거나 원장을 쓰지 않는다.
- 성공 status의 verification aggregate는 `unregistered|in_progress|met`다. schema·전이·
  해소·I/O 실패의 `invalid`는 verification 값이 아니라 별도 오류 outcome이다. 어느 쪽도
  전체 `/round` 완료가 아니다. report/folded는 별도 terminal observation이다.
- CI는 원장 condition으로 넣지 않는다. 마지막 SHA의 세 플랫폼 결과를 외부 terminal
  observation으로 확인해 자기 참조를 피한다.

schema 1의 필드·크기·digest vector·상태·JSON·exit 계약은 회차의 잠긴
[`intent.md`](../../.palimpsest/rounds/2026-08-30-round-verification-status/intent.md)가 진다.

## 결과

**얻는 것.** 실행기와 훅은 같은 상태 enum을 소비할 수 있고, 진행 중 회차의 미등록·대기·
낡은 증거를 완료로 읽지 않는다. parser와 reducer는 같은 Rust 바이너리로 세 플랫폼에서
돈다. 사람 출력과 JSON이 독립적으로 상태를 다시 계산하지 않는다.

**잃는 것.** 원장 작성자는 oracle digest와 event 순서를 정확히 지켜야 한다. schema 1은
`mode=command`만 담으므로 dialectic·judgment 조건은 `unregistered`로 남는다. 과거 회차는
verification 원장이 없어 자동 후보가 되지 않는다.

**다음 결정에 남기는 것.** 명령 실행·승인, projected content digest, Stop 차단,
finding·judgment를 포함한 전체 `RoundState`는 이 ADR이 결정하지 않는다.

## 되돌리는 조건

append-only event 순서만으로 현재성을 재현할 수 없거나, 실제 세 플랫폼에서 같은 fixture가
다른 상태 enum을 내거나, 읽기 전용 status가 명령 실행·파일 수정을 피할 수 없으면 이 결정을
다시 연다. Python wrapper가 Rust와 독립된 조건 문법을 되찾거나 `xtask → pal-cli` 의존이
필요해져도 소유 경계가 무너진 것이므로 재검토한다.
