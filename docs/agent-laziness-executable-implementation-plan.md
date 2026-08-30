# 실행 가능한 완수 원장 — 다음 세션 구현 계획

> **성격**: 조사 결과와 실행 순서를 잇는 착수 문서. 결정의 정본은 종료 시 발행할 ADR,
> 실행 상태의 정본은 GitHub 이슈다. 이 문서는 둘을 대신하지 않는다.
>
> **작성 기준**: palimpsest `2ea99a3ec15fb4f74c97d7541ad152127fdb2e5d`,
> unlazy `473d4b80421c36d733042434cd4b938f81a19ef1`, 2026-08-30.
>
> **입력**: [비교 보고서](agent-laziness-unlazy-comparison-and-implementation-plan.md).

이 문서의 목적은 비교를 한 번 더 하는 것이 아니다. 다음 세션은 §7만 따라 첫 구현 회차를
열고, §2의 수직 결과를 만들기 시작한다. 구현 중 새 사실이 나오면 `/round`의 정정·확대·승격
규칙으로 처리하되, 여기서 이미 대조한 범위를 다시 넓게 조사하지 않는다.

## 1. 조사 판정

### 1.1 유지되는 결론

비교 보고서의 중심 판정은 현재 자료와 맞는다. `/round`의 의도 잠금·접힘·정반합·독립 검토는
유지하고, 그 아래에 결정론적 조건의 실행·증거·재검증·Stop 차단 경로를 더한다.

외부 기준점도 그대로다. upstream `main`은 2026-08-30 조회에서 비교 문서가 고정한
`473d4b8`과 동일했다. upstream은 다음을 실제 코드 계약으로 둔다.

- gate 성공은 exit 0과 `EXPECT` 일치가 함께 있어야 한다.
- exact oracle 승인이 없으면 명령을 실행하지 않는다.
- `--reverify`는 이미 통과한 gate도 다시 실행한다.
- Stop은 검사를 실행하지 않고 저장된 상태만 축약한다.
- Stop의 무진척 guard는 메타데이터 수정이 아니라 semantic state 변화만 진척으로 센다.

근거는 upstream [README](https://github.com/Leonxlnx/unlazy/blob/473d4b80421c36d733042434cd4b938f81a19ef1/README.md),
[gate checker](https://github.com/Leonxlnx/unlazy/blob/473d4b80421c36d733042434cd4b938f81a19ef1/scripts/gate-check.mjs),
[Stop hook](https://github.com/Leonxlnx/unlazy/blob/473d4b80421c36d733042434cd4b938f81a19ef1/scripts/stop-hook.mjs),
[security boundary](https://github.com/Leonxlnx/unlazy/blob/473d4b80421c36d733042434cd4b938f81a19ef1/SECURITY.md)다.
Claude Code의 현재 공식 훅 계약도 `Stop`에 `stop_hook_active`와 `last_assistant_message`가
들어오고, exit 0의 최상위 `{"decision":"block","reason":"..."}`이 종료를 막는다고
명시한다([Hooks reference](https://code.claude.com/docs/en/hooks#stop)).

### 1.2 비교 문서에서 바로잡을 것

| 자리 | 조사 결과 | 계획에 반영한 판정 |
|---|---|---|
| 단계 0의 선행 ADR | [진행 규칙](plan/README.md#7-진행-규칙)은 ADR을 기능 종료 시 발행한다 | 선택은 첫 회차 `intent.md`에 잠그고 ADR은 그 회차 종료 때 발행한다 |
| Rust status가 `record.py conditions` 소비 | [stack](plan/00-stack.md)은 외부 런타임 없는 Rust 바이너리를 택했고, Python/Rust가 같은 문법을 해석하면 즉시 두 파서가 된다 | 조건 파서를 첫 회차에 Rust로 옮기고 Python은 그 결과의 소비자/호환 래퍼로 내린다 |
| active round 해소 | 현재 명시적 active pointer가 없고, 과거 `2026-08-18-round-protocol`도 `report.md`·`folded.md`가 없어 단순 스캔은 둘을 active로 오인한다 | 새 verification 원장이 있고 terminal marker가 없는 회차가 정확히 하나일 때만 자동 해소한다. 0개는 통과, 2개 이상은 구조 오류다 |
| 원장 확장자 | 현재 `xtask`는 회차 아래 `*.jsonl`을 발견 레코드 schema로 전수 해석한다 | `verification.log`에 JSON Lines를 담는다. 기존 `.log` 원자료 선례를 따르고 발견 레코드 모집단과 섞지 않는다 |
| snapshot 결박 | `HEAD + worktree digest`를 그대로 쓰면 evidence append와 커밋이 자기 증거를 낡게 만드는 재귀가 생긴다 | staleness는 HEAD가 아니라 content tree digest로 판정하고 현재 회차의 evidence 파일 하나만 digest 모집단에서 제외한다. HEAD는 provenance로만 기록한다 |
| hook 실패 정책 | 현재 훅은 “오작동으로 사람을 막지 않는다”는 fail-open이고, 새 상태 기계는 malformed ledger를 complete로 읽으면 안 된다 | transport/입력 미인식은 기존처럼 통과+진단, 선택된 active ledger의 형식 오류·미충족은 block으로 분리한다 |
| #92 상태 | `2bd9cd5 feat(#92)`가 추출 기반 내용 대조를 이미 구현했지만 이슈는 open이다 | 새 reducer에 다시 구현하지 않는다. 다음 세션 첫 이슈 정리에서 코드·게이트를 확인해 #92를 닫는다 |

### 1.3 현재 코드가 주는 경계

- [hook policy](../crates/pal-cli/src/hook/policy.rs)는 `EVENTS = ["SubagentStop"]`이고,
  `Stop`·`SessionStart`·`PreToolUse`를 명시적으로 통과시킨다.
- [hook transport](../crates/pal-cli/src/hook.rs)는 실패를 반환하지 않고 exit 0 + 구조화 JSON만
  차단으로 사용한다. 이 바이트 계약은 유지한다.
- [install hook](../crates/pal-cli/src/install/hooks.rs)과
  [layout](../crates/pal-cli/src/install/layout.rs)은 판정 목록에서 설치 목록을 렌더링한다.
  `Stop` 이름을 다른 파일에 손으로 복제하지 않는다.
- [CLI command enum](../crates/pal-cli/src/main.rs)은 아직 `round` 하위 명령이 없다.
- [condition parser](../.claude/skills/round/bin/record.py)는 Markdown 조건 문법의 현재 단일
  해석자다. 코드펜스·하위 조건·인라인 코드·판정 태그·중복 ID 함정을 이미 처리한다.
- [round script tests](../crates/pal-cli/tests/round_scripts_run.rs)는 설치된 Python 자산을
  실행하지만, 이는 `pal round status`가 Python을 런타임 의존으로 가져도 된다는 결정이 아니다.
- `pal touch`는 `decide`의 파일 내부 관계까지만 답했고 결박은 0건이었다. Markdown/Python
  하네스에는 좌표를 못 주었다. 이 계획은 그 능력 부재를 코드 영향 범위 조회인 척하지 않는다.

## 2. 첫 구현 회차

첫 회차의 이름은 `round-verification-status`로 한다. 목표는 **읽기 전용 판정자 한 자리**다.
명령 실행과 Stop을 섞지 않는다. 실행기보다 먼저 상태 축약기를 세우는 것은 수평 기반 공사가
아니라, 뒤의 실행기와 훅이 다른 완료 정의를 갖지 않게 하는 최소 초석이다.

### 2.1 잠긴 산출

한 회차에서 다음을 모두 만든다.

1. `pal-intent`의 Rust 조건 파서와 `ConditionId`, `Condition`; `pal-cli`의 비공개
   `Oracle`, `Evidence`, `ConditionState`, `VerificationState`.
2. `.palimpsest/rounds/<slug>/verification.log` 스키마 1 parser/reducer. 내용은 UTF-8 JSON Lines다.
3. `pal round conditions --file <intent.md> --json`과
   `pal round status --round <slug> [--json]`.
4. `--round`를 생략했을 때 새 verification 원장을 기준으로 active round를 해소하는 한 자리.
5. `record.py conditions`와 dashboard가 `PAL_BIN`(시험에서는
   `CARGO_BIN_EXE_pal`, 설치본에서는 PATH의 `pal`)을 호출하는 호환 래퍼가 되고, 같은
   fixture에서 Rust 출력과 같음을 보이는 전환 시험.
6. 저장소본·설치본 `/round`가 새 Rust 명령을 가리키게 하는 최소 문면 변경.
7. 회차 종료 때 이 상태 기계의 결정과 퇴로를 담은 ADR 한 편.

### 2.2 이번 회차의 소비 장면

다음 명령 하나가 실제 진행 중 fixture를 보고 조건별 상태와 전체 상태를 같은 값으로 낸다.

```bash
cargo run -q -p pal-cli -- round status \
  --round <fixture-round> --json
```

사람 출력과 JSON은 표현만 다르고 reducer 결과를 공유한다. 첫 회차는 `mode=command`만
지원한다. 원장이 없거나 condition에 oracle이 없으면 `unregistered`, evidence가 없으면
`pending`, oracle digest가 다르면 `stale`, 최신 evidence가 exit 0과 EXPECT를 함께
만족하면 `met`, 실행 결과가 실패하면 `unmet`이다.

첫 회차의 aggregate 이름은 round completion이 아니라 **verification state**다.
`unregistered | in_progress | met | invalid`의 닫힌 enum이며, `met`는 등록된 command-mode
조건의 증거가 현재라는 뜻일 뿐 `/round` 전체가 `complete`라는 뜻이 아니다. `report.md`와
`folded.md`는 각각 `reported`와 `folded`라는 별도 terminal observation으로 내고, 둘 다
있으면 `invalid`다. 전체 `RoundState`는 finding·정반합·Stop을 통합하는 회차에서만 만든다.

### 2.3 이번 회차에서 하지 않는 것

- shell 명령 실행과 사용자 승인 저장소 쓰기
- `Stop` 등록·차단
- timeout과 자식 프로세스 트리 정리
- 과거 회차 전량 이주
- `xtask`의 과거 원장 검사를 새 reducer로 전부 교체

이 범위 밖 항목들은 각각 #85·#95·#97을 실제로 닫는 다음 수직 경로의 일부다. 첫 회차가
끝날 때는 후속 구현 이슈를 새로 발행하는 대신 §6의 기존 이슈에 native blocking 관계와
판정 가능한 완료 조건을 넣는다.

### 2.4 첫 회차 합격선

- verification oracle ID는 `intent.md` 조건 ID의 부분집합이다. intent 밖 ID는 오류이고,
  oracle이 없는 intent 조건은 `unregistered`다.
- schema version, event kind, ID, mode 또는 필드가 알 수 없는 값이면 오류다.
- schema 중복과 oracle보다 앞선 evidence 같은 불가능한 전이는 오류다.
- 조건 문장을 verification 원장에 복제하지 않는다.
- `status`는 명령을 실행하거나 파일을 수정하지 않는다.
- current round 자동 해소는 후보 0개를 정상 통과, 2개 이상을 오류로 낸다.
- 과거 report 없는 회차는 verification 원장이 없으므로 active 후보가 아니다.
- Rust 파서가 전환 전에 보존한 Python golden의 코드펜스·들여쓰기·중복 ID·태그 순서 결과와 같다.
- JSON과 사람 출력이 같은 reducer 결과에서 렌더링된다.
- ubuntu·macOS·Windows가 같은 fixture에 같은 상태 enum을 낸다.

## 3. 잠글 결정

다음 값은 첫 회차 인터뷰에서 재탐색하지 않고 권고값으로 잠근다. 반증할 현재 코드 근거가
나올 때만 승격한다.

| 결정 | 권고값 | 이유 / 뒤집는 조건 |
|---|---|---|
| 원장 위치 | 회차 안 `verification.log` | JSON Lines이지만 기존 `*.jsonl` 발견 레코드 검사와 분리한다. 의도와 같은 수명·공유 범위 |
| 기록 형태 | append-only JSONL + schema 머리 행 | 부분 쓰기와 이력의 구분이 보인다. reducer는 마지막 유효 event를 계산 |
| digest | 기존 workspace `blake3` | 새 crypto 의존을 더하지 않고 내부 identity 계약과 맞춘다 |
| snapshot 판정 | approve+verify 회차에서 projected content tree digest로 확정 | 첫 status 회차는 oracle digest staleness만 판정한다. commit 전/후 재귀를 푸는 spike 뒤 편입 |
| HEAD | evidence provenance, staleness 입력 아님 | commit 자체가 증거를 낡게 하는 재귀를 막는다 |
| 조건 파서 | Rust 한 자리 | `pal` 단일 바이너리와 세 OS 계약. Python은 문법 소유자가 아니다 |
| active round | `verification.log` 존재 + terminal marker 부재의 유일 후보 | 새 pointer 진실원을 만들지 않고 과거 회차를 소급 이주하지 않는다 |
| 상태 오류 | CLI에서는 nonzero + JSON error, Stop에서는 active ledger 오류만 block | complete로 fail-open하지 않되 알 수 없는 hook payload로 사람을 막지 않는다 |
| raw 성공 출력 | 저장하지 않고 digest + byte count | 비밀 노출과 원장 팽창을 줄인다. 원문 필요 gate는 artifact 경로를 별도 등록 |
| CI | condition이 아니라 마지막 SHA의 외부 terminal observation | #95의 자기 참조를 만들지 않는다 |

첫 회차에서 결정하지 않을 값도 경계가 선명하다. 승인 저장소의 OS별 위치·권한, shell
허용 범위, timeout/output 상한, Windows process-tree cleanup, projected snapshot digest,
finding·정반합 통합, Stop 무진척 상한은 실행기와 Stop 회차의 결정이다. 첫 reducer의
타입에는 하드코딩하지 않는다.

### 3.1 schema 1의 닫힌 형태

첫 회차가 읽는 event는 셋뿐이다.

```json
{"kind":"schema","version":1,"round":"<slug>"}
{"kind":"oracle","id":"A1","mode":"command","check":"cargo test ...","expect":{"literal":"A1_OK"},"cwd":"."}
{"kind":"evidence","id":"A1","oracle_digest":"<blake3>","exit":0,"matched":true,"output_digest":"<blake3>","output_bytes":123}
```

- 파일은 최대 8 MiB, 한 행은 줄바꿈 제외 64 KiB다. 문자열은 UTF-8 최대 32 KiB다.
  빈 행과 trailing partial line은 오류다. 각 event는 아래에 적은 필드만 허용한다.
- `schema` 필드는 `kind="schema"`, `version=1`인 `u32`, `round`뿐이다. 첫 행에 정확히
  하나이며 round는 `[a-z0-9][a-z0-9-]*`이고 디렉터리 slug와 같아야 한다.
- `oracle` 필드는 `kind="oracle"`, 유효한 condition `id`, `mode="command"`, 비어 있지
  않은 `check`, 정확히 `{ "literal": <비어 있지 않은 문자열> }`인 `expect`, `cwd`뿐이다.
  `cwd`는 `/` 구분자의 정규화된 저장소 상대 경로이며 `.`은 허용하고 절대 경로와 `..`은
  거부한다.
- `evidence` 필드는 `kind="evidence"`, 유효한 condition `id`, 소문자 64자리 hex인
  `oracle_digest`, `i32` 범위의 `exit`, boolean `matched`, 소문자 64자리 hex인
  `output_digest`, `u64`인 `output_bytes`뿐이다. 시간은 판정 입력이 아니므로 schema 1에 싣지 않는다.
- `oracle`은 같은 ID에 여러 번 올 수 있고 파일 순서상 마지막 행이 현재 oracle이다. oracle
  변경도 append event이며 원장을 다시 쓰지 않는다.
- `evidence`도 같은 ID에 여러 번 올 수 있다. 현재 oracle digest를 가리키는 마지막 evidence가
  현재 관측이고, 새 oracle 뒤에 현재 digest의 evidence가 없으면 `stale`이다.
- 행 순서가 우선한다. 최신 oracle보다 앞선 evidence는 현재 관측이 아니며, evidence가 있던
  ID에 같은 digest의 oracle을 다시 append해도 새 evidence 전까지 `stale`이다. 최신 oracle
  뒤 마지막 evidence의 digest가 현재 oracle과 다르면, 그 앞에 현재 digest evidence가 있어도 `stale`이다.
- evidence ID에 oracle이 없거나, 한 condition에 mode가 둘이거나, 알 수 없는 필드·kind·mode가
  있으면 `invalid`다. unknown은 보존해 통과시키지 않는다.
- `oracle_digest` 입력 바이트는 ASCII `pal.round.oracle.v1` 뒤의 NUL 한 바이트인 domain 뒤에
  `[mode, check, "literal", expect.literal, cwd]`를 차례로 붙인다. 각 값은 UTF-8 바이트 앞에
  `u64` little-endian 길이를 붙인다. `command` / `cargo test -q` / `ROUND_OK` / `.`의 digest는
  `4cf3cb926ab8249a040632d0c1e694509ab40eee2eacc8da15d1353392b026dd`여야 한다.
  각 필드 한 글자 변경, 한국어 literal, 빈 값 거부, 32 KiB 경계가 단위 시험 벡터다.
- `mode=dialectic`과 `judgment` event는 첫 판에서 거부한다. 비결정론 조건은 oracle이 없는
  `unregistered`로 남고 verification aggregate가 `met`가 되지 않는다.

상태와 종료 코드는 다음처럼 고정한다.

| 입력 | condition state | status result | CLI exit |
|---|---|---|---:|
| 존재하는 round·intent에 원장 없음 | — | `unregistered` | 0 |
| 모든 condition에 oracle 없음 | `unregistered` | `unregistered` | 0 |
| 일부 condition에만 oracle 있음 | `unregistered` 포함 | `in_progress` | 0 |
| evidence 없음 | `pending` | `in_progress` | 0 |
| digest 불일치 | `stale` | `in_progress` | 0 |
| 최신 evidence 실패 | `unmet` | `in_progress` | 0 |
| 등록 condition 전부 met | `met` | `met` | 0 |
| schema/ID/전이 오류 | — | `invalid` + 해당 code | 2 |
| 자동 후보 0개 | — | outcome `no_active_round` | 0 |
| 자동 후보 2개 이상 | — | `invalid` + `resolve_error` | 2 |

현재 저장소의 terminal marker는 `report.md`와 `folded.md` 둘뿐이다. 막힘은 `/round`의
상태이지만 별도 기계 marker가 아직 없으므로 첫 status가 추측하지 않는다. 두 marker가 모두
있으면 오류, 하나가 있으면 active 후보에서 제외, 둘 다 없으면 verification 원장 존재 여부로
후보를 가른다.
명시한 round 디렉터리나 필요한 `intent.md`가 없으면 원장 유무보다 먼저 `resolve_error`다.
원장만 있고 intent가 없는 경우도 같다. malformed intent condition은 status에서
`invalid_schema`로 내고 새 code를 만들지 않는다.

### 3.2 CLI 경계

`pal round conditions --file <path> --json`은 현재 `record.py conditions`의 JSON 키를
그대로 낸다: 최상위 `파일`, `조건`, `열림`, `닫힘`, `형식오류`; 각 조건의 `id`, `상자`,
`판정`, `전사`, `줄`, `원문`, `형식오류`. 조건 형식 오류는 이 JSON을 출력하고 exit 1,
정상은 exit 0, 사용법·I/O·schema 오류는 exit 2다. 이것이 Python 래퍼 제거 전 parity 계약이다.

`pal round status --json`의 성공 형태는 다음과 같다.

```json
{"outcome":"status","round":"round-verification-status","verification":"met","terminal":"open","conditions":[{"id":"A1","state":"met","oracle_digest":"<64 hex>"}]}
```

등록되지 않은 condition은 `state="unregistered"`이고 `oracle_digest`를 생략한다. 오류는
`{"outcome":"invalid","code":"invalid_schema","message":"..."}`, 자동 후보 0개는
`{"outcome":"no_active_round"}`다. 안정된 `code` enum은 넷뿐이다.

| code | 포함하는 오류 |
|---|---|
| `invalid_schema` | malformed JSON, unknown/missing/extra field·version·kind·mode·type, 크기 제한, ID·round·cwd·digest 형식, schema 중복 |
| `invalid_transition` | oracle 없는 evidence 등 파일 순서상 불가능한 전이 |
| `resolve_error` | active 후보 복수, 명시 round와 directory/intent 불일치, 필요한 intent 부재 |
| `io_error` | intent·원장·회차 디렉터리 읽기 실패 |

구체 원인은 `message`가 지고 code를 늘리지 않는다. 사람 출력도 round, verification,
terminal, 모든 condition ID와 state를 반드시 보이며 같은 view model만 렌더링한다.

## 4. 코드 좌표

### 4.1 첫 회차가 소유할 파일

| 파일 | 변경 |
|---|---|
| `crates/pal-core/src/budget.rs`, `crates/pal-core/src/lib.rs` | schema 1의 잠긴 크기 상한 셋을 stack §5.5의 예산 단일 위치에서 소유 |
| `crates/pal-intent/Cargo.toml`, `crates/pal-intent/src/lib.rs`, `crates/pal-intent/src/round_condition.rs` | 조건 문법과 타입의 단일 Rust 정본과 공개 배선; intent 계층이라 CLI·xtask 모두 아래 방향으로 의존 가능 |
| `crates/pal-cli/Cargo.toml` | 기존 workspace `blake3` 직접 의존 배선 |
| `crates/pal-cli/src/round/mod.rs` | 상태 도메인 타입과 모듈 경계 |
| `crates/pal-cli/src/round/ledger.rs` | JSONL schema/parser와 transition 검증 |
| `crates/pal-cli/src/round/status.rs` | reducer, active round 해소, JSON/사람 view model |
| `crates/pal-cli/src/main.rs` | `Round` command와 `status` 배선만 |
| `crates/pal-cli/tests/round_status.rs` | CLI black-box와 RED fixture |
| `crates/pal-cli/tests/round_scripts_run.rs` | Python 호환 래퍼와 Rust 결과 parity |
| `.claude/skills/round/bin/record.py` | `conditions` 구현을 Rust 호출/호환 변환으로 내림 |
| `.claude/skills/round/bin/dashboard.py` | `조건들` import를 같은 `PAL_BIN` 호출로 내림 |
| `.claude/skills/round/SKILL.md` | 결정론 조건 등록과 status 사용의 짧은 계약 |
| `xtask/Cargo.toml`, `xtask/src/main.rs` | 원장 둘 대조의 intent-condition 읽기 한 자리만 `pal-intent` parser로 옮겨 Python과 이중 해석하지 않음; gate parser와 다른 Python 위임 이전은 범위 밖 |
| `docs/adr/00NN-*.md` | 회차 종료 때 실제 구현 결정을 기록 |

`pal-core`에는 넣지 않는다. [xtask의 코어 어휘 금지](../xtask/src/main.rs)는 거버넌스
어휘가 그래프 도메인으로 스며드는 것을 막는다. 조건 문법은 기존 `pal-intent`, 원장과 상태
기계는 Claude Code 하네스와 CLI의 제품 층에 둔다. `xtask → pal-cli`는 stack의 “어떤
크레이트도 `pal-cli`에 의존하지 않는다” 불변식을 깨므로 금지한다. 새 crate도 만들지 않는다.
수직 경로가 선 뒤 독립 컴파일 경계가 필요하다는 실측이 있을 때만 분리한다.
`main.rs`의 `mod round`가 바이너리에 배선하고 내부 단위 시험은 각 모듈에 둔다. CLI
black-box 시험은 기존 방식대로 `CARGO_BIN_EXE_pal`만 실행한다.

`record.py`와 dashboard는 `PAL_BIN`이 있으면 그 경로, 없으면 PATH의 `pal`을 실행한다.
`round_scripts_run.rs`는 설치 fixture 실행 때 `PAL_BIN=CARGO_BIN_EXE_pal`을 주입한다.
`install/layout.rs`와 `install/hooks.rs`는 소유 파일이 아니라, 바뀐 skill/script asset이 기존
파생 경로를 따라 설치되는지 확인하는 drift 검증 좌표다.

### 4.2 뒤 회차의 좌표

| 수직 경로 | 추가 좌표 |
|---|---|
| approve + verify | `round/approval.rs`, `round/verify.rs`, CLI tests, 사용자별 외부 승인 저장소 |
| 음성 대조 + judgment | ledger/reducer event, known-broken fixture, 정반합 evidence reference |
| Stop | `hook/policy.rs`, `hook.rs`, `install/hooks.rs`, hook/install 왕복 tests |
| 기존 검사 통합 | `xtask/src/main.rs`, `record.py`, dashboard, gate/report 생성 경로 |

## 5. RED와 검증

### 5.1 첫 회차 RED fixture

코드를 쓰기 전에 `crates/pal-cli/tests/round_status.rs`에 다음 실패를 등록한다.

1. 원장 없음 → verification `unregistered`, exit 0.
2. intent 조건 `A1 A2` 대 oracle `A1` → `A1=pending`, `A2=unregistered`; oracle `A3` 추가 → exit 2.
3. evidence 없음 → `pending`.
4. oracle 한 글자 변경 → 기존 evidence `stale`.
5. exit 0 + EXPECT 불일치 → `unmet`.
6. `folded.md` → terminal observation `folded`, verification state와 별개.
7. `report.md`와 `folded.md` 동시 존재 → exit 2.
8. 알 수 없는 schema version/kind/mode/field → 안정된 error code와 exit 2.
9. verification 원장이 있는 비종료 회차 둘 → active round 모호성 exit 2.
10. report 없는 과거 회차 + verification 원장 있는 새 회차 → 새 회차 하나를 선택.
11. 코드펜스 안 상자, 들여쓴 하위 조건, 중복 ID, 뒤집힌 전사 태그 → 전환 전 golden과 같음.
12. `mode=dialectic` 또는 `judgment` event → 첫 schema에서 조용히 통과하지 않고 exit 2.

음성 대조는 fixture 하나를 실제로 깨뜨려 같은 검사가 빨개지는지 먼저 본다. 특히 11번은
`record.py`를 고치기 **전에** 현 Python 파서의 입력과 JSON 출력을 golden fixture로 보존한다.
새 Rust 파서는 그 golden과 대고, wrapper 전환 뒤 Rust↔wrapper 동등성은 별도 배선 시험으로
둔다. 둘이 같은 새 구현을 호출한다는 사실을 의미론 대조로 세지 않는다.

### 5.2 첫 회차 검증 명령

```bash
cargo test -p pal-cli --test round_status
cargo test -p pal-cli --test round_scripts_run
cargo test -p pal-cli --test hook
cargo test -p pal-cli --test install_hooks
cargo xtask check
cargo test --workspace --all-targets
```

마지막 SHA의 GitHub CI는 외부 terminal observation으로 확인한다. 그 결론을 커밋되는
condition 상자에 미리 쓰지 않는다.

### 5.3 전체 경로의 후속 RED

approve/verify 회차는 미승인 실행, exit 0/no marker, exit 1/marker, PATH/CWD/shell 변경,
timeout, output cap, 실행 중 oracle 변경을 공격한다. Stop 회차는 pending/stale/unrun
negative control을 심어 실제 `decision:block`을 보고, semantic state 변화 없이 반복했을 때
상한이 풀려도 `complete`로 바뀌지 않음을 본다. 이 조건들이 서기 전에는 #85·#97을 닫지 않는다.

## 6. 이슈 처분

상태 변경은 이 문서가 아니라 다음 세션의 GitHub 작업으로 한다.

| 이슈 | 다음 세션 조치 | 완료 판정 |
|---|---|---|
| #88 | 기존 본문에 최신 비교가 대체했다는 댓글을 남기고, 제목·본문을 첫 회차 `round status reducer`로 갱신한 뒤 assign + `ready-for-agent` | §2.4와 세 OS CI |
| #92 | `2bd9cd5`와 현재 추출 parity를 확인하고 구현 근거 댓글과 함께 닫음 | 보존 원문 ID와 추출 결과 집합 같음이 실제 시험에서 통과 |
| #85 | **#88이 #85를 막는 방향**으로 #85에 native `blocked_by=#88`을 걸고 유지 | Stop 수직 경로와 실제 Claude Code block/pass 관측 후 닫음 |
| #97 | **#88이 #97을 막는 방향**으로 #97에 native `blocked_by=#88`을 걸고 유지 | negative control evidence 없이는 met가 불가능 |
| #95 | 기존 검사 통합 경로에 유지 | CI를 외부 terminal observation으로 옮긴 뒤 닫음 |
| #96 | verification event ledger가 실제 진행 원장을 대체하는지 효과 회차에서 판정 | 대체가 확인되면 흡수, 아니면 별도 실험으로 유지 |
| #90 | 첫 reducer에 억지로 흡수하지 않음 | legacy gate 모집단 문제라 status 원장과 별개 |
| #94 | Rust 조건/verification schema가 선 뒤 기존 죽은 `게이트파서` 선언을 제거 또는 사람용 요약으로 격하 | 기계 소비자 0인 정본 선언이 남지 않음 |

#84·#89·#93은 이 경로보다 먼저가 아니다. 이유는 status→verify→Stop 수직 경로의 선행
의존이 아니고, 사용자의 거짓 완료를 직접 막지 않기 때문이다. 새 실행 원장이 실제 효과를
낸 뒤에도 필요하면 그때 프론티어에서 다시 판정한다.

## 7. 다음 세션 착수 절차

다음 세션은 아래 순서로 바로 시작한다.

1. 현재 HEAD를 기록하고 아래 제한 diff를 본다. 출력이 있으면 그 경로만 다시 읽고 이 문서의
   좌표를 정정한다. 넓은 비교 조사는 다시 하지 않는다.

   ```bash
   git diff --name-only 2ea99a3ec15fb4f74c97d7541ad152127fdb2e5d..HEAD -- \
     crates/pal-cli/src/main.rs crates/pal-cli/src/hook.rs \
     crates/pal-cli/src/hook/policy.rs crates/pal-cli/src/install/hooks.rs \
     crates/pal-cli/src/install/layout.rs crates/pal-cli/tests/round_scripts_run.rs \
     .claude/skills/round/bin/record.py .claude/skills/round/bin/dashboard.py \
     crates/pal-intent/src/lib.rs crates/pal-intent/Cargo.toml \
     crates/pal-cli/Cargo.toml xtask/Cargo.toml xtask/src/main.rs
   ```

2. `/round`를 열고 이 문서의 §2.4를 완수 조건으로 옮긴다. 인터뷰 상한 1라운드,
   사전부검 상한 1라운드, 독립 리뷰 상한은 원 의도 모집단 2라운드로 등록한다.
3. `./scripts/frontier.sh`로 상태를 다시 보고 #88이 여전히 착수 가능하면 assign한다.
4. #88에 기존 13줄 문면 안을 최신 비교가 대체했다는 댓글을 남기고, §2·§5를 담은
   판정 가능한 이슈 본문으로 갱신한다. #85와 #97에 `blocked_by=#88`을 건다. GitHub
   쓰기가 실패하면 그 사실을 `state.md`에 적되 로컬 RED 착수를 막지 않는다. #94는 첫
   회차가 직접 흡수하므로 dependency를 만들지 않는다.
5. `pal touch`로 `Command`, `decide`, `HOOK_EVENTS`, `조건들`을 조회한다. 그래프가 파일
   경계를 답하지 못하면 능력 부재로 기록하고 그때만 `rg`로 내려간다.
6. `record.py` 수정 전에 §5.1 11번의 Python golden을 보존하고, 나머지 RED tests를 추가해
   현재 실패를 관측한다.
7. `pal-intent/round_condition.rs` → `round/ledger.rs` → `round/status.rs` → CLI 배선 → Python 호환
   래퍼 순으로 구현한다.
8. §5.2를 실행하고, 실제 진행 중 fixture에서 사람/JSON 출력을 둘 다 본다.
9. 독립 리뷰가 닫히면 ADR을 발행하고 #88을 닫는다. #92도 코드와 이슈의 drift를 해소한다.
10. 다음 프론티어는 approve+verify다. 그 회차를 실제로 즉시 열 수 있을 때만 #85·#97의
   blocking 관계를 따라 진행한다. 열지 않기로 판정하면 목표/우선순위 사유로 접는다.

projected snapshot digest spike는 첫 회차의 선행 조사가 아니다. approve+verify 회차를 열 때
`pal-git::WorktreeState.tree_digest`의 기존 API 확장으로 verification 원장 하나를 제외할 수
있는지 먼저 잰다. 성공하면 그 회차 소유 파일에 `crates/pal-git/src/lib.rs`를 명시하고,
실패하면 snapshot staleness를 `unmeasured`로 남겨 승격한다. HEAD-only를 판정으로 대체하지 않는다.

## 8. 전체 종료선

다음이 모두 실제 프로젝트 이슈 한 건에서 관측될 때 에이전트 게으름의 “증거 없는 완료”
경로를 닫는다.

1. 결정론적 조건은 승인된 실행 evidence 없이 met가 아니다.
2. 등록된 음성 대조를 실행하지 않으면 주 조건도 met가 아니다.
3. oracle 또는 projected content snapshot이 바뀌면 과거 evidence가 stale이다.
4. 종료 직전 재검증이 이미 met인 결정론 조건까지 다시 실행한다.
5. 비결정론 조건은 정반합 evidence로 같은 aggregate에 들어간다.
6. 열린 금지역·실패 finding은 complete를 막는다.
7. 메인 Stop이 incomplete를 실제로 차단한다.
8. blocked/folded와 무진척 guard release는 complete로 승격되지 않는다.
9. 체크박스만 켠 거짓 완료와 stale evidence를 실제 Stop이 각각 차단한다.
10. 세 OS CI와 마지막 SHA의 외부 CI 관측이 성공한다.
11. #85·#88·#95·#96·#97이 구현·흡수·명시적 접힘 중 하나로 실제 처분된다.

Depth Tree·모델 가격 라우팅·병렬 lease는 위 장면의 선행 조건이 아니다. 이 경로가 실제로
한 번 소비된 뒤 병렬 작업의 누락이 새 프론티어로 관측될 때만 별도 결정을 연다.
