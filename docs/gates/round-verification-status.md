# 게이트 — round verification status

> 회차 [`2026-08-30-round-verification-status`](../../.palimpsest/rounds/2026-08-30-round-verification-status/) ·
> 잠긴 의도 [`intent.md`](../../.palimpsest/rounds/2026-08-30-round-verification-status/intent.md) ·
> 이슈 [#88](https://github.com/hskim-ecoletree/palimpsest/issues/88) · 결정 [ADR-0028](../adr/0028-round-verification-is-an-append-only-observation-ledger.md)

## 합격선

읽기 전용 `pal round status`가 잠긴 schema 1과 상태 전이를 한 reducer로 판정하고,
Rust 조건 정본·CLI·Python 호환 표면이 같은 fixture를 소비해야 한다. 명령 실행이나 파일
수정, `xtask → pal-cli` 의존, `pal-cli` library target은 허용하지 않는다.

**RED** — 구현 전 `cargo test -p pal-cli --test round_status`의 13개 시험은 존재하지 않는
`round` subcommand 때문에 전부 실패했다. 기대 state 하나만 틀린 음성 대조도 exit 101로
빨개졌다. 원문은 [`red-observation.md`](../../.palimpsest/rounds/2026-08-30-round-verification-status/red-observation.md)가 진다.

## 판정

| 판정 | 조건 |
|---|---|
| 통과 | A1 A2 A3 A4 A5 A6 A7 A8 A10 |
| 반증 | — |
| 대조불가 | — |
| 미측정 | A9 |

**검산** — 통과 9 · 반증 0 · 대조불가 0 · 미측정 1 = 10

A9는 로컬 시험으로 닫지 않는다. 동일 checked-in fixture를 실행하는 마지막 SHA의
ubuntu·macOS·Windows CI가 모두 성공할 때만 통과로 옮긴다.

### 근거

- A1~A4: `round_status` black-box와 ledger unit test가 ID 부분집합, 닫힌 schema,
  불가능 전이, 고정 digest vector를 직접 공격한다. 잠긴 intent가 바이트 계약을 직접 싣는다.
- A5~A6: sentinel·입력 바이트 불변 시험과 0/1/복수 후보·terminal 충돌 fixture가 읽기 전용
  해소 경계를 잰다.
- A7: 전환 전 Python parser로 먼저 만든 golden과 Rust CLI, PAL_BIN wrapper, PATH fallback이
  같은 fixture에서 대조된다.
- A8: JSON과 사람 renderer가 하나의 `StatusView`만 받으며 두 실제 출력의 상태가 같다.
- A10: `cargo xtask check`의 의존 방향 검사와 Cargo manifest가 소유 경계를 잰다.

## 효과

시험이 아닌 빌드된 `pal`로 실제 진행 중 fixture를 소비했다. JSON과 사람 출력 모두
A1 `met`, A2 `pending`, aggregate `in_progress`, terminal `open`을 냈다. 입력과 원 출력은
[`effect/`](../../.palimpsest/rounds/2026-08-30-round-verification-status/effect/)에 보존했다.

이 장면은 뒤의 실행기와 Stop 훅이 별도 완료 정의를 만들지 않고 같은 reducer를 소비할 수
있는 최소 수직 경로가 실제로 섰음을 보인다.

## 범위 밖

- shell 명령 실행과 사용자 승인 저장소
- projected content snapshot digest
- Stop 등록·차단과 프로세스 정리
- 과거 회차 전량 이주
- finding·judgment를 포함한 전체 `RoundState`
- 기존 `xtask` 원장 검사의 전면 교체
