# RED와 음성 대조 관측

기준 SHA `2ea99a3ec15fb4f74c97d7541ad152127fdb2e5d`의 구현 전 상태에서 보존했다.

## Python golden 선행 보존

`record.py`를 바꾸기 전에 기존 parser를
`crates/pal-cli/tests/fixtures/round_conditions_traps.md`에 실행했다. 종료 코드는 `1`이었고,
stdout을 `round_conditions_traps.golden.json`에 먼저 고정했다. fixture에는 코드펜스,
중첩 들여쓰기, 중복 ID, 뒤집힌 전사 태그가 함께 있다. 이후 Rust 출력은 이 파일과 바이트
단위로 같아야 했다.

## 구현 전 RED

계획 §5.1의 black-box 시험을 먼저 추가한 뒤 다음을 실행했다.

```text
$ cargo test -p pal-cli --test round_status
running 13 tests
...
error: unrecognized subcommand 'round'
test result: FAILED. 0 passed; 13 failed
exit: 101
```

실패 원인은 새 표면이 아직 없다는 계획의 RED 예측과 같았다.

## 음성 대조

구현 뒤 `met` fixture 하나의 기대 상태만 존재하지 않는 값으로 바꾸고 같은 시험을 다시
실행했다.

```text
$ cargo test -p pal-cli --test round_status
assertion failed
left: "met"
right: "unregistered"
test result: FAILED
exit: 101
```

기대값을 복구한 뒤 같은 시험 24개가 모두 통과했다. 이 관측은 현재 초록 시험이 구현 전
실패를 거쳤다는 역사 증거이며, 최종 검증 결과와 섞지 않는다.
