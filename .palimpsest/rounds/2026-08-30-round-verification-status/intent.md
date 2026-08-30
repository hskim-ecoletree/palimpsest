# 회차 — round verification status

## 원문

> AGENTS.md와 `/round` 절차를 따른다.
>
> 먼저 `docs/agent-laziness-executable-implementation-plan.md`를 읽고, §7 「다음 세션 착수 절차」에 따라 바로 구현에 착수해라.
>
> 목표는 §2의 첫 구현 회차 `round-verification-status`를 완수하는 것이다. §2.4를 완수 조건으로 옮기고 `/round`를 연 뒤 다음 순서로 진행한다.
>
> 1. 계획서 기준 SHA 이후의 제한 diff를 확인한다.
> 2. `./scripts/frontier.sh`로 현재 이슈 상태를 확인하고 #88을 착수 대상으로 정리한다.
> 3. 그래프에 먼저 코드 관계를 묻고, 답하지 못한 범위에서만 `rg`를 사용한다.
> 4. `record.py`를 변경하기 전에 기존 Python 조건 파서의 golden fixture를 보존한다.
> 5. 계획서 §5.1의 RED tests를 먼저 추가하고 실제 실패를 확인한다.
> 6. `pal-intent/round_condition.rs` → `pal-cli`의 ledger/status → CLI 배선 → Python 호환 래퍼 순으로 구현한다.
> 7. §5.2의 검증을 모두 실행한다.
> 8. 사전부검과 독립 리뷰의 발견을 전부 처분하고, ADR·게이트·효과 관측·종료 보고까지 회차를 완결한다.
>
> 계획서에서 잠근 schema, digest 직렬화, 상태 전이, JSON 형식, exit code, 의존 방향을 임의로 다시 설계하지 마라. 현재 코드가 기준 SHA 이후 바뀌어 충돌할 때만 관련 경로로 조사를 제한하고 정정 근거를 `state.md`에 기록한다.
>
> 구현 중 `xtask → pal-cli` 의존이나 불필요한 `pal-cli` library target을 만들지 마라. 조건 문법의 단일 Rust 정본은 `pal-intent`, verification 원장과 상태 기계는 `pal-cli`가 소유한다.
>
> 중간 단계에서 멈추지 말고, 첫 회차의 완수 조건과 검증이 실제로 충족될 때까지 진행해라.

## 목적 기여

[00-goals.md](../../../docs/plan/00-goals.md)의 첫 릴리스 목표와 하네스 목표에 기여한다.
증거 없는 완료를 막는 소비 경로 `status`가 #85·#97의 선행 조건이고, 현재 프론티어 #88이
직접 착수 가능하므로 소비 가능한 수직 경로에 가장 가깝다.

## 완수 조건

- [ ] A1 verification oracle ID는 intent 조건 ID의 부분집합이며, intent 밖 ID는 오류이고 oracle 없는 intent 조건은 `unregistered`다
- [ ] A2 알 수 없는 schema version·event kind·ID·mode·필드와 잘못된 타입은 오류다
- [ ] A3 schema 중복과 oracle보다 앞선 evidence를 포함한 불가능한 상태 전이는 오류다
- [ ] A4 verification 원장은 조건 문장을 복제하지 않고 잠긴 schema와 digest 직렬화를 그대로 지킨다
- [ ] A5 `status`는 명령을 실행하거나 파일을 수정하지 않는다
- [ ] A6 current round 자동 해소는 후보 0개를 정상 통과, 후보 2개 이상을 오류로 내며 verification 원장 없는 과거 회차를 후보로 삼지 않는다
- [ ] A7 Rust 조건 파서는 전환 전에 보존한 Python golden의 코드펜스·들여쓰기·중복 ID·태그 순서 결과와 같다
- [ ] A8 JSON과 사람 출력은 같은 reducer 결과에서 렌더링된다
- [ ] A9 ubuntu·macOS·Windows가 같은 fixture에 같은 상태 enum을 낸다
- [ ] A10 `xtask → pal-cli` 의존과 `pal-cli` library target 없이 조건 문법은 `pal-intent`, verification 원장과 상태 기계는 `pal-cli`가 소유한다

**RED 관측**: `pal round` 명령과 Rust 조건 파서·verification reducer가 아직 없으므로 §5.1의
black-box 시험이 구현 전에 실패해야 한다.

**음성 대조**: §5.1 fixture 하나의 기대 상태를 의도적으로 틀리게 한 동일 시험이 빨개져야
하며, Python golden은 기존 parser 결과를 먼저 고정해 새 Rust 구현과 독립된 원천이어야 한다.

### A4 대조 계약 — schema 1과 digest 직렬화

독립 리뷰가 외부 계획 문서 없이도 A4를 대조할 수 있도록 이 회차가 지켜야 할 바이트 계약을
여기에도 잠근다.

- 원장은 UTF-8 JSON Lines이고 8 MiB 이하, 각 행은 줄바꿈 제외 64 KiB 이하, 문자열은
  UTF-8 32 KiB 이하다. 빈 행과 trailing partial line은 오류다.
- 첫 행은 정확히 하나의 `{"kind":"schema","version":1,"round":"<slug>"}`다.
  `<slug>`는 `[a-z0-9][a-z0-9-]*`이며 디렉터리 이름과 같아야 한다.
- oracle은 정확히 `kind,id,mode,check,expect,cwd` 필드만 갖는다. `mode`는 `command`만,
  `expect`는 비어 있지 않은 `literal` 하나만 허용하고, `cwd`는 `.` 또는 정규화된 저장소
  상대 경로다. 조건 문장은 원장에 싣지 않는다.
- evidence는 정확히 `kind,id,oracle_digest,exit,matched,output_digest,output_bytes` 필드만
  갖는다. digest는 소문자 64자리 hex, `exit`는 i32, `output_bytes`는 u64다.
- oracle digest 입력은 ASCII `pal.round.oracle.v1` 뒤의 NUL 한 바이트에 이어
  `[mode, check, "literal", expect.literal, cwd]`를 붙인다. 각 값은 UTF-8 바이트 앞에
  u64 little-endian 길이를 붙인다. `command` / `cargo test -q` / `ROUND_OK` / `.`의
  digest는 `4cf3cb926ab8249a040632d0c1e694509ab40eee2eacc8da15d1353392b026dd`다.
- oracle과 evidence는 append-only이며 마지막 oracle이 현재다. 최신 oracle 뒤에 그
  digest의 evidence가 없거나, 최신 evidence digest가 다르면 `stale`이다. oracle 없는
  evidence는 `invalid_transition`, 알 수 없는 필드·kind·mode·type과 중복 schema는
  `invalid_schema`다.
- 조건 상태는 `unregistered|pending|stale|met|unmet`, aggregate는
  `unregistered|in_progress|met|invalid`, terminal은 `open|reported|folded`다.
  오류 code는 `invalid_schema|invalid_transition|resolve_error|io_error`로 닫는다.
- 성공과 `no_active_round`는 exit 0, 조건 parser 형식 오류는 exit 1, status의 schema·전이·
  해소·I/O 오류는 exit 2다. JSON과 사람 출력은 같은 `StatusView`에서 렌더링한다.

구현 전 RED와 음성 대조의 관측 원문은 [`red-observation.md`](red-observation.md)가 진다.

## 퇴로

- 기준 SHA 이후 관련 코드가 바뀌어 잠긴 계약과 충돌하면 관련 경로만 조사하고 근거를
  `state.md`에 기록한 뒤 정정한다.
- 잠긴 계약을 코드가 원리상 수용하지 못하면 더 약한 계약으로 조용히 바꾸지 않고 승격한다.

## 범위 밖

- shell 명령 실행과 사용자 승인 저장소
- Stop 등록·차단과 프로세스 정리
- projected content snapshot digest
- 과거 회차 전량 이주
- finding·정반합을 포함한 전체 `RoundState`
- 기존 `xtask` 원장 검사의 전면 교체

## 개정

- 없음.

## 승격

- 없음.
