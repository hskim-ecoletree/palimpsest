# F03-1 게이트 — 정체성: `symbol_id` 의 성분과 판별자 · 판정 기록

**판정: 통과 (2026-08-13).** 합격선 여섯이 전부 선다.

`[f03.1]` 이 물은 것:

> **같은 심볼인가 — 그리고 같은 것을 같다고 하는 쪽이 함께 서는가.**

**둘 다 선다.** 그리고 이 조각이 낸 가장 좋은 증거는 좌표가 움직였다는 사실이 아니라
**무엇이 움직였는지가 정확히 예측된 집합과 같았다는 것**이다 — 움직인 329 는
**컨테이너가 비지 않은 심볼 전부**이고, 다른 것은 하나도 안 움직였다.

이슈 [#51](https://github.com/hskim-ecoletree/palimpsest/issues/51) ·
합격선 정본 [`corpus/criteria.toml`](../../corpus/criteria.toml) `[f03.1]`
(등록 커밋 `3621f6d` — **첫 코드 커밋 이전**) ·
재현 `./scripts/f03-1-verify.py`

**합격선을 정한 것도 판정한 것도 에이전트다** — [R-18](../plan/00-risks.md#r-18)은 닫히지
않는다. 줄인 것: 합격선이 코드 이전에 등록됐고, 커밋 넷이 **구조와 동작을 갈랐으며**,
불변식 F 에 **음성 대조**가 붙었고(체인을 빼면 순서가 정체성을 흔든다는 것을 시험이
직접 보인다), **대조 스크립트 자신의 결함 셋을 잡아 고쳤다**(§4).

---

## 1. 합격선 여섯

| | 판정 | 관측 |
|---|---|---|
| ① 컨테이너 체인이 실린다 | **통과** | 체인 있는 심볼 **329** = `FileGraph.contains` **340** − 대장 밖 **11**. 최대 깊이 **1** |
| ② 불변식 E · F · G — **F 가 반대 방향 ★** | **통과** | E 충돌 대상 19 · F 는 메서드 **43** 개 · G 는 **69** 개 |
| ③ L0 결박 불가의 **타입 강제** | **통과** | `compile_fail` 문서 시험 **둘**이 실제로 돈다 |
| ④ 정체성 규칙 5종 | **통과** | 다섯 각각에 시험 하나. **익명 귀속의 뒤쪽 절반이 없었다**(§3) |
| ⑤ 오버로드 재정렬 빈도 | **측정했다 — 0 건** | 오버로드 자리 연 **2,345** · 커밋 쌍 **238** · 재정렬 **0** |
| ⑥ 결정성 | **통과** | 캐시를 따로 준 두 회차가 같다 (ditto 4,578 · portal-backend 1,296) |

---

## 2. 좌표가 움직였다 — **건수가 아니라 목록으로**

`[f03.1.pass].on_failure` 이 요구한 형태다. 「전」은 `860592b`(표면을 세운 커밋),
「후」는 `5d20d4e`.

### ditto @ `aded7ce7f88f` — 심볼 4,578 중 **329 이 움직였다**

**그 329 는 컨테이너가 비지 않은 심볼 전부와 정확히 같은 집합이다.** 사라진 심볼 0 ·
새로 생긴 심볼 0 · **`body_digest` 이동 0**.

| | |
|---|---|
| 종류 | `method` **321** · `function` **5** · `type_alias` **3** |
| 걸린 파일 | **63**. 상위 다섯 — `work-item-store.ts`(43) · `memory-store.ts`(28) · `icl/parser.ts`(19) · `coverage-store.ts`(15) · `handoff-ref-store.ts`(15) |
| 최대 깊이 | **1** — ditto 에 중첩 클래스가 없다 |

**`body_digest` 가 0 개 움직였다는 것이 이 표에서 가장 값진 줄이다.** F03 §2 의
*"둘을 분리한 것이 핵심이다"* 가 실물에서 처음 관측됐다 — 정체성 축을 통째로 바꿨는데
「변했는가」 축은 미동도 하지 않았다.

### 정체성 등급이 **34 개 올랐다** (`ordinal` → `exact`, 내려간 것 0)

**R-16 의 결함이 실물에서 어떤 모양이었는지가 이 목록이다.** 거의 전부가
`constructor` 다 — 한 파일에 예외 클래스가 여럿이면 **둘째 `constructor` 부터
`ordinal`** 이었고, 이제 각자의 클래스 안에서 유일하다.

```
rebuild/handoff/ref-store.ts   BatonExistsError.constructor
rebuild/handoff/ref-store.ts   HandoffCasExhaustedError.constructor
rebuild/record/intent.ts       IntentAlreadyLockedError.constructor
rebuild/record/intent.ts       IntentAfterVerdictError.constructor
rebuild/record/store.ts        WorkItemNotFoundError.constructor
rebuild/record/store.ts        TerminalStatusError.constructor
rebuild/record/store.ts        NotTerminalError.constructor
rebuild/record/store.ts        ParkingRequiresFinalizeError.constructor
rebuild/record/store.ts        UnknownCriterionError.constructor
rebuild/record/store.ts        IntentLockViolationError.constructor
rebuild/util/fs.ts             SchemaValidationError.constructor
src/acg/icl/parser.ts          Parser.constructor
src/core/cleanup-archive.ts    CleanupDeleteRefusedError.constructor
src/core/cleanup-archive.ts    CleanupDirtyRepoError.constructor
src/core/cleanup-store.ts      CleanupBasisRequiredError.constructor
src/core/cleanup-store.ts      CleanupStore.constructor
src/core/fs.ts                 SchemaValidationError.constructor
src/core/handoff-ref-store.ts  HandoffRefStore.constructor
src/core/journey-authoring/session.ts  JourneyReferenceNotFoundError.constructor
src/core/memory-project.ts     MemoryEventAlreadyDecidedError.constructor
src/core/memory-project.ts     MemorySelfApprovalError.constructor
src/core/memory-store.ts       MemorySourceStore.constructor
src/core/memory-store.ts       MemoryEventStore.constructor
src/core/memory-store.ts       MemoryGraphIrStore.constructor
src/core/memory-store.ts       MemoryProjectionStore.constructor
src/core/memory-store.ts       MemoryEventStore.dir
src/core/memory-store.ts       MemoryProjectionStore.dir
src/core/memory-store.ts       MemoryEventStore.get
src/core/memory-store.ts       MemoryEventStore.list
src/core/memory-store.ts       MemoryEventStore.path
src/core/memory-store.ts       MemoryGraphIrStore.path
src/core/memory-store.ts       MemoryGraphIrStore.write
src/core/run-with.ts           RunWithRuntimeError.constructor
src/core/work-item-store.ts    WorkItemStore.constructor
```

### boxwood/portal-backend @ `a29cad0bf6a8` — **0 개 움직였다**

Kotlin 추출기는 최상위 선언만 보므로 포함 관계가 없고 체인이 빈다. **빈 것이 정확한
값이다** — 담긴 심볼을 아예 안 뽑았으므로 담는 관계도 없다. 그 비대칭이
`[f03.1.does_not_prove].not_kotlin_containers` 에 등록된 그대로다.

---

## 3. 이 조각이 실제로 찾은 것 — **넷**

### ① `IdentityGrade` 가 두 사실을 겸한다 — 그런데 **실물 하중이 0 이 됐다**

`nodes_of` 는 `identity = min(판별자 상한, 추출기 실측)` 을 쓴다. 그런데 둘이 재는
것이 다르다:

| | 무엇을 말하나 |
|---|---|
| 판별자 상한 | *"이 좌표가 **선언 순서**에 의존하는가"* |
| 추출기 실측 | *"이 심볼에서 **지역 이름을 지워도 되는가**"* (ADR-0006) |

한 열거로 접으면 **상한 때문에 `ordinal` 로 적힌 심볼의 요약이 이미 지워져 있을 수
있다** — 등급이 말하는 정규형과 실제 정규형이 어긋나는 상태다.

**변경 전 그 상태가 34 건이었고, 지금은 0 건이다.** 실측했다 — ditto 4,578 중 대장이
`ordinal` 로 적은 **325** 개 전부가 추출기 실측도 `ordinal` 이다. **판별자 상한이 지금
실물에서 아무것도 내리지 않는다.**

**그러므로 갈라 두지 않는다.** 없는 하중에 타입을 늘리면 그것이 곧
*"만들 수 없는 변형을 미리 두는"* 형태다. **빚으로 적고 넘긴다** — F01 이 인식 ③ 에,
F02-3 이 값/타입 충돌에 한 것과 같은 처분이다.

### ② `.ts` 다섯이 **NUL 바이트로 `binary` 다** — 심볼 78 · 포함 관계 11 이 대장 밖

```
src/acg/boundary/codeql-edges.ts    `${from}\0${normTo}`
src/acg/icl/static-check.ts         `${s.kind}\0${s.ref}`
src/acg/impact/codeql-analyzer.ts   `${path}\0${pkg}`
src/core/memory-query.ts            `${from}\0${e.to}\0…`
src/core/mode-doctor.ts             h.update('\0')
```

**분류는 규칙대로 옳다** — `classify` ② 가 *"NUL 바이트. git 이 쓰는 것과 같은 판정"*
이라 적었고 git 도 이 파일들을 binary 로 본다. **그리고 대장은 거짓말하지 않는다** —
`binary{nul_byte}` 로 적히므로 *"보지 않음"* 이 산출에 남는다.

**그런데 이 다섯은 실코드다.** 합성 키의 구분자로 NUL 을 쓰는 관용구이고, 이 저장소
자신이 `SymbolId::compute` 에서 쓰는 것과 같은 기법이다. `binary{nul_byte}` 라는 이름은
사용자에게 *"이미지 파일"* 로 읽히지 *"NUL 을 담은 소스"* 로 읽히지 않는다.

**고치지 않고 넘긴다** — 범위를 늘리지 않는다. `[f03.1.pass]` ① 의 항등식은 그 다섯을
**빼고** 성립한다(340 − 11 = 329)는 사실을 함께 적는다.

### ③ 컨테이너가 **클래스만이 아니다** — 문서를 정정했다

F03 §3.2 는 컨테이너를 *"파일→클래스→(중첩)클래스"* 로 적었는데 실물은 더 넓다.
체인 있는 329 중 **여덟이 클래스가 아닌 것 안에 있다**:

```
function  rebuild/memory/reduce.ts          reduceEvents.effective
function  src/acg/fitness/injected-provider.ts  injectedProvider.load
function  src/acg/internal-packages.ts      scanLocalJars.walk
function  src/core/cleanup-scan.ts          walkDocs.walk
function  src/core/memory-scan.ts           walkFiles.walk
type_alias src/core/autopilot-loop.ts       executeTestBarrier.RunResult
type_alias src/core/lsp/client.ts           getDiagnostics.LspDiagnosticIn
type_alias src/core/lsp/client.ts           getDiagnostics.RpcIncoming
```

**좁히지 않고 문서를 고쳤다.** 좁히면 `walk` 라는 이름의 중첩 함수가 여럿인 파일에서
두 `walk` 가 **다시 선언 순서로만** 갈리고, 그것이 이 성분을 넣은 이유인 R-16 그
자체다. 클래스는 예시이지 규정이 아니었다 — F02 가 §4 를 정정한 것과 같은 형태다.

### ④ **익명 귀속의 뒤쪽 절반이 안 서 있었다**

F03 §3.4 는 *"익명 함수·클로저·람다는 독립 심볼이 아니고, 가장 가까운 이름 있는
조상에 귀속되고 그 `body_digest` 에 **포함된다**"* 라고 적는다. 앞쪽 절반
(*"심볼이 아니다"*)은 F02-1 이 시험했는데 **뒤쪽 절반은 아무도 안 봤다.**

안 서면 익명 안의 코드가 **통째로 감시 밖으로 사라진다** — F02 가 넘긴 「입자 부재」의
이웃이다. 시험을 세워 보니 **선다**: 화살표 함수의 본문을 바꾸면 조상의 요약이 바뀐다.

---

## 4. 대조 스크립트가 **자기를 세 번 속였다** — 그리고 셋 다 「대조가 꺼진 형태」였다

이 게이트에서 가장 값진 부분이다. `[f03].self_judged` 3 이 *"대조가 꺼지는 형태가
둘"* 이라고 적었는데, 실제로 걸린 셋은 **그 둘 중 어느 것도 아니었다.**

| | 무엇이 꺼졌나 | 어떻게 드러났나 |
|---|---|---|
| **저장소 이름** | 임시 저장소를 `mkdtemp` 로 만들어 회차마다 이름이 달랐다. 매니페스트가 없으면 `RepoId` 가 디렉터리 이름에서 온다 → **회차마다 좌표가 전부 다르다** | 불변식 F 가 *"움직였다"* 를 냈다. **무엇을 재든 언제나 실패하는 검사**였다 |
| **세는 단위** | 재정렬을 **바이트 자리**로 셌다. 줄 하나만 고쳐도 자리가 전부 움직인다 | 재정렬 **8 건**이 나왔고 그중 실제 재정렬은 **0** 이었다. `body_digest` 열로 고쳤다 |
| **자르는 단위** | 클래스 맞바꾸기를 **문자열 자리**로 잘랐다. `span` 은 바이트이고 이 코퍼스에는 한글 주석이 있다 | 조각이 어긋나 파일이 깨졌고, **깨진 파일에는 잴 것이 없어** *"어긋났다"* 가 아니라 *"메서드가 1 개다"* 가 됐다 |

**셋째가 특히 조용하다.** 변형이 소스를 망가뜨리면 산출이 비고, 빈 산출은 어긋남을
못 낸다 — *"통과"* 로 읽힐 수 있는 자리다. 그래서 스크립트에 하한 둘을 박았다:
**대조할 메서드가 둘 미만이면 실패**이고, **맞바꾸기가 자리를 하나도 안 움직이면 멈춘다.**

이것이 `[f22].self_judged` 의 *"음성 대조가 없으면 아무것도 세지 않는 검사가 만점을
받는다"* 가 **검사 자신에게** 적용된 네 번째 사례다(앞의 셋: `d1f9a09` F01 ·
`493f7e9` F22-3 · F02-4 의 공유 캐시).

---

## 5. 오버로드 재정렬 — **재고, 안 넣는다**

`[f03.1.pass]` ⑤ 는 수치가 아니라 둘을 요구했다: (1) 쟀다 (2) 채택 여부와 근거를 적었다.

| | |
|---|---|
| 세는 단위 | 한 커밋에서 같은 `(경로, 컨테이너 체인, 이름, 종류)` 심볼이 **둘 이상**이고, 그 **`body_digest` 열의 집합은 같은데 순서가 다른** 경우 |
| 모집단 | 두 코퍼스의 최근 **120 커밋씩** — 대조한 커밋 쌍 **238** |
| 오버로드 자리 | 연 **2,345 건** |
| **재정렬** | **0 건** |

**F03 §4.1 의 집합 digest 감시(`identity-shift` 사건)를 채택하지 않는다.**

근거: 그 대응이 막으려는 사건이 **238 커밋 쌍에서 한 번도 일어나지 않았다.** 문서
자신이 *"L1 언어에서 오버로드 재정렬이 실제로 얼마나 일어나는지 아무도 세어본 적이
없다. 이 기능에서 코퍼스로 세고, **유의미하면** 넣는다"* 라고 적었고, 이 값이 그
판단의 입력이다. 없는 사건에 스키마를 늘리면 그것이 곧 *"만들 수 없는 변형을 미리
두는"* 형태다(F02 가 `CodeFreshness` 의 세 변형을 안 만든 것과 같은 판단).

**단 이 값이 무엇을 재는지는 좁게 적는다** — 두 코퍼스의 **최근 이력**이고 전 이력이
아니다. 그리고 **재정렬이 일어나기 어려운 조건**이 두 코퍼스에 있다: TypeScript 는
같은 이름의 최상위 선언을 허용하지 않고(오버로드 자리 2,345 는 대부분 Kotlin 이다),
Kotlin 의 오버로드는 파라미터가 달라 사람이 손으로 순서를 바꿀 이유가 적다.
**「일어나지 않는다」가 아니라 「이 두 코퍼스의 이 구간에서 일어나지 않았다」이다.**

---

## 6. 이 조각이 **증명하지 않는 것** — 등록된 그대로

| | 관측 |
|---|---|
| **파일 간 정체성이 아니다** | 체인은 파일 **안**의 포함 관계다. 모듈을 넘는 이름은 F07 |
| **Kotlin 에 체인이 없다** | 심볼 1,296 전부가 빈 체인. 그 비대칭이 산출에 실린다 |
| **R-16 이 안 닫힌다** | 컨테이너가 **없는** 오버로드는 여전히 `ordinal` 로 갈린다. 단위 시험 `같은_컨테이너의_오버로드는_여전히_순서로_갈린다` 가 그 사실을 붙든다 |

---

## 7. 넘기는 빚 — **건수가 아니라 목록**

| 빚 | 자리 |
|---|---|
| `IdentityGrade` 가 **순서 의존**과 **지우기 안전성** 둘을 겸한다 (§3①) | 실물 하중 **0**. 하중이 생기면 가른다 |
| `.ts` 다섯이 NUL 바이트로 `binary` — 심볼 **78** · 포함 **11** 이 대장 밖 (§3②) | 열림. `binary{nul_byte}` 라는 이름이 실코드를 이미지처럼 읽히게 한다 |
| `pal touch` 가 컨테이너를 화면에 안 찍는다 | `SymbolNode.container` 는 실린다. 표시는 열림 |
| 오버로드 재정렬 측정이 **최근 120 커밋**이다 (§5) | 전 이력 측정은 비싸고, 지금 값이 0 이라 급하지 않다 |

---

## 8. 재현

```bash
cargo build --workspace --release
./scripts/f03-1-verify.py --history 120        # 여섯 다 통과 (약 12 분)
cargo test --workspace                          # 205
cargo test -p pal-core --doc                    # `compile_fail` 둘
```

### 커밋 다섯 — 순서가 규율이다

| | |
|---|---|
| `3621f6d` | **합격선** — 코드 이전 |
| `163294e` | 구조 — 포함 관계를 1층 캐시가 담는 값에 |
| `860592b` | 동작 — `pal ledger --symbols` 표면 (기존 산출 불변) |
| `5d20d4e` | 동작 — **컨테이너 체인을 좌표에.** 좌표가 329 움직였다 |
| `b477174` | 동작 — `SymbolIdentity` · 타입 강제 · 규칙 다섯 · 대조 스크립트 |

**구조 커밋에 「산출이 안 움직였다」는 증거를 메시지에 적었다** — `s0` 불일치 0 ·
골든 대장의 `entries`·`languages`·`repos_declared`·`snapshot` 이 바이트로 같다.
