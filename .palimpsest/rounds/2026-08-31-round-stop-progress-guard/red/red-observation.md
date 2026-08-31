# RED 관측 — 2026-08-31

명령:

```text
cargo test -p pal-cli --test round_stop -- --nocapture
```

관측:

```text
running 14 tests
9 passed; 5 failed
error: unrecognized subcommand 'stop'
Usage: pal round <COMMAND>
```

실패한 다섯 공격은 activation/disable, reentry·malformed dispatch, semantic progress,
replay·6회 handoff, corrupt state다. 공통 실패 원인은 `pal round stop` 표면과 Stop 정책이
아직 없다는 것이다. 기존 공용 시험 아홉은 통과해 RED fixture 자체의 기반은 살아 있다.

## 음성 대조

- 새 CLI를 no-op으로만 추가하면 pending block assertion이 실패한다.
- 모든 Stop을 무조건 block하면 activation 전·disable·reentry pass assertion이 실패한다.
- raw 파일 digest를 쓰면 JSON 표현만 바꾼 공격이 counter reset을 드러낸다.
- session ID만 dedupe하면 같은 session의 새 transcript 여섯 개가 상한에 도달하지 못한다.
- cap에서 round 파일을 쓰면 verification byte equality와 report 부재 assertion이 실패한다.
