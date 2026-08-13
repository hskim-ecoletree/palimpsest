# F02-1 게이트 — 추출기 골격 + TypeScript 선언 · 판정 기록

**판정: 통과 (2026-08-13).**

**선언을 빠짐없이 뽑는다. 그리고 그것을 골든이 아닌 것으로 쟀다.**

표본 20 파일에서 손으로 센 선언 **172 개**와 추출 목록이 **집합으로 같다** — 빠뜨린 것
0 · 잘못 잡은 것 0. 손 목록은 추출기 코드보다 **먼저 커밋됐다**(`d77c04f`).

| | 등록값 | 관측 |
|---|---|---|
| ① 심볼 리콜 — 표본 파일 수 | 20 | **통과** — 20 |
| ① 손 목록이 추출 이전에 커밋됨 | true | **통과** — `d77c04f` → `25b812d` |
| ① 빠뜨린 심볼 | **0** | **통과** — 0 |
| ① 잘못 잡은 심볼 | **0** | **통과** — 0 |
| ② `FileGraph` 가 파일 하나에만 의존 | true | **통과** — 다른 저장소·다른 경로·다른 이름에서 **바이트 단위로 동일** |
| ② `Coord` 가 `FileGraph` 에 없다 | true | **통과** — `repo`·`tree`·`coord`·`path` 0 개 |
| ③ 반드시 바꾸는 변이 넷 | 4/4 | **통과** — 4/4 |
| ③ 반드시 안 바꾸는 변이 둘 | 2/2 | **통과** — 2/2 |
| ③ 변이 대상 부재 시 멈춤 | true | **통과** — 여섯 다 고정 SHA 의 실재 경로·실재 식별자에 묶임 |
| ④ S0 레퍼런스 벡터 불변 | 불일치 0 | **통과** — 1,122 파일 · 선언 2,241 · 불일치 0 |
| ④ F01 골든 대장 불변 | 997 항목 동일 | **통과** — `f01-verify` ⑦ 「대조 동일」 |
| ④ `grade_of(Kotlin)` | L1 | **통과** — L1 |
| ⑤ TypeScript 능력이 뒤집힌다 | true | **통과** — `NotBuilt` → `Present` |
| ⑤ Java · JavaScript 는 `NotBuilt` | true | **통과** — `javascript-extraction` 실물 확인 |
| ⑥ `f22-3` 재실행 | true | **통과** — [F22-3 재판정](F22-3-chain-nodes.md) |
| ⑥ 산출을 목록으로 | true | **통과** — 커밋마다 발현·도입·신뢰도·나머지 후보 |
| ⑥ F22-3 게이트 갱신 | true | **통과** — 대조 불가 → 통과, 정정 하나 포함 |
| ⑥ 낡은 음성 대조를 먼저 고침 | true | **통과** — `2e2eb3f`. **고치기 전 수치는 쓰지 않았다** |
| ⑦ 테스트/실코드 구별 부재가 선언됨 | true | **통과** — 아래 「구별하지 않는다」 |

이슈 [#46](https://github.com/hskim-ecoletree/palimpsest/issues/46) (부모 [#5](https://github.com/hskim-ecoletree/palimpsest/issues/5)) ·
합격선 정본 [`corpus/criteria.toml`](../../corpus/criteria.toml) `[f02.1]` ·
오라클 [`corpus/tasks/f02-recall-sample.tsv`](../../corpus/tasks/f02-recall-sample.tsv) ·
대조 [`scripts/f02-1-verify.py`](../../scripts/f02-1-verify.py)

**합격선을 정한 것도 판정하는 것도 에이전트다** — [R-18](../plan/00-risks.md#r-18)은
닫히지 않는다. 줄일 수 있었던 것만 줄였다: 합격선이 **코드 이전에**(`f22017e`·`a5a5b13`)
등록됐고, 손 목록이 **추출기 이전에**(`d77c04f`) 커밋됐으며, 대조가 **자기 자신을**
음성 대조한다(아래).

---

## 1. 오라클 — 손으로 센 목록. **개수가 아니다**

`corpus/tasks/f02-recall-sample.tsv`. ditto @ `aded7ce7f88feb3c03238c5f9760f3a2ade4a6c1`
의 `.ts` **496** 중 **LC_ALL=C 경로 정렬 후 등간격** 20 파일 — T10 과 같은 규칙
(`g = floor(496/20) = 24` → 인덱스 0·24·…·456).

**표본 전부를 적는다**(합격선의 요구):

| # | 파일 | 손으로 센 선언 |
|---:|---|---:|
| 1 | `rebuild/drive/backstop-turns.frozen.test.ts` | **0** |
| 2 | `rebuild/hook/stop-gate.ts` | 3 |
| 3 | `rebuild/memory/query.ts` | 1 |
| 4 | `rebuild/schemas/completion-contract.test.ts` | 1 |
| 5 | `rebuild/seam/hook-fire-smoke.ts` | 4 |
| 6 | `rebuild/state/relock.ts` | 3 |
| 7 | `scripts/adr-guard.ts` | 9 |
| 8 | `src/acg/fitness/fitness-runner.ts` | 13 |
| 9 | `src/acg/tidy/subgraph.ts` | 3 |
| 10 | `src/cli/commands/impact.ts` | 1 |
| 11 | `src/cli/wizard/prompt.ts` | 7 |
| 12 | `src/core/change-contract-store.ts` | 6 |
| 13 | `src/core/coverage-manager.ts` | 47 |
| 14 | `src/core/e2e/lifecycle.ts` | 7 |
| 15 | `src/core/handoff-store.test.ts` | 5 |
| 16 | `src/core/land-commit.ts` | 8 |
| 17 | `src/core/prism/opponent.ts` | 22 |
| 18 | `src/core/teardown.ts` | 12 |
| 19 | `src/hooks/runtime.characterization.test.ts` | 3 |
| 20 | `src/schemas/common.ts` | 17 |
| | **합계** | **172** |

**1 번이 0 인 것이 이 표에서 가장 중요한 줄이다.** `describe`/`test` 콜백이 전부 익명
화살표라 심볼이 아니다. TSV 는 그것을 **`ordinal 0` 한 줄**로 적는다 — *"선언이 없는
파일"* 과 *"빠뜨린 파일"* 이 구별되어야 한다.

### F02 §3.3 이 답하지 않은 다섯을 판단했고, **코드보다 먼저 적었다**

세는 규칙은 TSV 파일 머리에 있다. 표가 답하지 않은 자리에서 판단한 것 다섯:

| | 판단 | 표본에서 하중을 지는가 |
|---|---|---|
| 익명 함수 | 독립 심볼이 아니다. 가장 가까운 이름 있는 조상에 귀속 | **진다** — 1 번이 0 인 이유 |
| 함수 내부 지역 변수 | 심볼이 아니다(폭발) | **진다** — 거의 모든 파일 |
| 블록 스코프의 `const` | 모듈 스코프가 아니다 | **진다** — 7 번의 `if (import.meta.main)` 안 둘 |
| 객체 리터럴 속성 | `variable_declarator` 가 아니다 | **진다** — 10 번 `run:` · 13 번 `enforce:` 여섯 |
| 인터페이스 멤버 시그니처 | 표에 없다 | **진다** — 8 번 `EvaluatorProvider.evaluate` |

그리고 **`constructor` 를 `method_definition` 으로 센다**(12 번). 표의 문자 그대로다.

---

## 2. `0 · 0` 이 "비교를 안 했다"가 아니라는 것 — **대조가 자기를 음성 대조한다**

리콜이 0·0 으로 나오면 그것이 *"둘이 같다"* 인지 *"비교가 안 돌았다"* 인지 산출만으로는
갈리지 않는다. 그래서 **같은 비교 함수에** 일부러 어긋난 입력을 먹인다:

```
자기 대조   한 줄 빼면 잘못 잡음 1(기대 1) · 없는 줄 넣으면 빠뜨림 1(기대 1)
            (src/core/coverage-manager.ts, 손 47)
```

양방향으로 정확히 1 건씩 잡힌다. 이것이 없으면 `0 · 0` 은 아무것도 뜻하지 않는다.

---

## 3. 음성 대조 — **넷은 바꾸고 둘은 안 바꾼다. 무거운 쪽은 둘이다**

```
✓ 선언 하나를 지운다            산출이 바뀌었다   src/core/land-commit.ts
✓ 선언 하나의 이름을 바꾼다        산출이 바뀌었다   src/core/teardown.ts
✓ 컨테이너 안으로 선언을 옮긴다      산출이 바뀌었다   rebuild/state/relock.ts
✓ export 를 뗀다               산출이 바뀌었다   src/core/land-commit.ts
✓ 주석만 고친다                 산출이 그대로다   rebuild/memory/query.ts
✓ 포매팅만 바꾼다                산출이 그대로다   rebuild/memory/query.ts
```

여섯 다 **고정 SHA 의 실재 경로와 실재 식별자**에 묶여 있고, 치환 대상이 소스에 없으면
`✓` 를 내는 대신 **멈춘다**. 자라는 값에 묶지 않았다.

### 무엇을 비교하는가 — **좌표는 빼고 본다. 이것이 판단이다**

`span` 은 바이트 오프셋이라 **어떤 편집이든 움직인다.** 주석 한 줄을 더하면 뒤의 모든
선언이 밀린다. span 을 넣고 비교하면 *"주석만 바꾸면 산출이 안 바뀐다"* 가 **성립할 수
없는 명제**가 되고, 반대로 *"바뀌어야 한다"* 넷은 아무 편집으로나 통과한다 — 둘 다
검사가 아니게 된다.

그래서 비교 대상은 **낡음을 판정하는 축**이다: 이름 · 종류 · `body_digest` · 포함 관계 ·
export · import. `body_digest` 를 span 과 **다른 축**에 둔 이유가 정확히 이것이다
(DESIGN §2.2 · F03 §2). 좌표가 움직이는 것은 정상이고, `stale` 을 켜는 것은 digest 다.

**이 해석은 합격선 문면에 없다. 여기 적어 판정에 싣는다.**

---

## 4. Kotlin 이 움직이지 않았다 — 그리고 **그 검사의 한계**

```
S0 전수 대조   1,122 파일 · 선언 2,241 · 선언 ≥1 파일 1,058 · 불일치 0
골든 대장      corpus/golden/portal-backend.ledger.json — 대조 동일 (997 항목)
등급           grade_of(Kotlin) == L1
음성           L1→L0 으로 떨어뜨리면 골든이 잡는다 (672 곳) — 리팩터가 이 변이를 지우지 않았다
```

트레잇 도입(`2e4934b`)과 정규형 이동(`47d90c3`)을 **구조적 변경으로 따로 커밋**했고
둘 다 산출을 안 움직였다. `s2` 의 정체성 값이 리팩터 전후로 같다 —
`id 3b24ed45c5f2 · body b9fd0caa81de`.

### ⚠ 그러나 **골든은 TypeScript 를 하나도 덮지 않는다**

`portal-backend@a29cad0` 에는 `.ts`·`.tsx`·`.js`·`.java` 가 **0 개**다(확장자를 세어
확인했다). 그래서 ④ 가 증명한 것은 **Kotlin 이 안 움직였다**이지 *"이 조각의 산출이
골든에 잡힌다"* 가 아니다. **TypeScript 쪽에는 골든이 없다** — 그 자리를 진 것이 ① 의
손 목록이고, 그것이 이 조각을 쪼갠 이유이기도 하다.

---

## 5. 이 게이트가 **닿지 않은** 자리 — 알면서 남긴 것

### ⚠ `.tsx` 는 이 문법이 아니다 — **`partial` 이 문법 부재를 가린다**

`tree-sitter-typescript` 는 `LANGUAGE_TYPESCRIPT` 와 `LANGUAGE_TSX` 를 **따로** 낸다.
붙인 것은 앞쪽 하나다. 그런데 `Language::from_extension` 은 `tsx` 를 `TypeScript` 로
접는다(`language.rs`). 그러면 `.tsx` 파일은 JSX 가 통째로 `ERROR` 가 되어 **`partial`
로 분류되고**, *"깨진 파일"* 과 *"이 빌드가 그 방언을 모른다"* 가 같은 출력이 된다 —
`Capable` 이 막으려는 바로 그 형태다.

**두 코퍼스에 `.tsx` 가 0 개라 이 조각의 판정에는 닿지 않았다.** 닿지 않았다는 사실을
여기 적는다. 처분은 `[f02.1.grammar].consequence` 에 등록했고 열려 있다 — 방언 축을
`LanguageId` 에 둘지, 확장자를 갈라 셀지.

### 규칙 넷이 **코퍼스가 아니라 단위 시험만** 덮는다

표본 20 파일에 모듈 스코프의 **구조 분해**(`const {a,b} = x`) · **다중 선언자**
(`const a = 1, b = 2`) · **`declare`** · **`namespace`** 가 **하나도 없다**(세어 봤다).
추출기는 넷에 대한 규칙을 갖고 있고 단위 시험이 그것을 센다. **코퍼스 실물에서는
시험되지 않았다.**

### 포함 관계가 **한 층에서만** 시험됐다

표본에서 `contains` 가 서는 곳은 12 번(클래스 하나 · 메서드 다섯)뿐이다. **중첩
클래스 → 메서드 같은 두 층 이상의 체인은 코퍼스에 없었다.** 단위 시험이 한 층을 센다.

### 발현 좌표를 파일로 되짚지 못한다

`manifests_at` 은 `SymbolId` 목록이고 경로가 실려 있지 않다. F22-3 재판정의 「테스트
파일」 표시는 **커밋이 만진 파일**로 센 상한이지 좌표에서 되짚은 값이 아니다.

---

## 6. 이 조각이 갚은 빚 · 새로 만든 빚

### 갚았다

| 빚 | 어디서 왔나 | 어떻게 닫혔나 |
|---|---|---|
| 범위 반증 테스트의 오라클이 없다 | F01 게이트 | `f02-recall-sample.tsv` — **수가 아니라 목록** |
| T10 의 수치가 재어지지 않는다 | F22-3 대조 불가 | 재실행 → 4/5 · 4/5 · 2/5. [F22-3 재판정](F22-3-chain-nodes.md) |
| `f22-3` 의 음성 대조가 조용히 꺼져 있다 | 2026-08-13 발견 | `2e2eb3f`. **재실행보다 먼저 고쳤다** |

### 새로 만들었다

| 빚 | 소유 |
|---|---|
| **입자 부재** — 조상 없는 익명 함수의 변경이 `no_semantic_change` 로 사라진다 | 아래 |
| `.tsx` 가 잘못된 문법으로 읽힌다 | 미배정 (`[f02.1.grammar]`) |
| 파서 스레드당 재사용 (F02 §3.1 시그니처) | **#49** |
| Kotlin 의 `exports`·`imports` 가 `NotBuilt` | 열림 |
| Kotlin 을 `FileGraph`(포함 관계)로 올리기 | 열림 — `ExtractorVersion` 이 좌표 이동을 진다 |
| `FileGraph` 에 `Deserialize` 가 없다 → 1층 캐시의 값으로 쓰려면 능력 축을 키로 | **F04** |

### **입자 부재** — 이 조각에서 가장 무거운 발견

`afcfefab`(T10 표본)이 `describe(…)` 콜백 **안**에 새 `test.each(…)` 블록을 통째로
더했는데 산출은 **`no_semantic_change`** 다 — *"코드는 변했는데 의미가 변한 심볼이
없다."*

**이 문장은 이 경우에 거짓이다.** 그 파일에는 심볼이 4 개 있지만, 변경이 일어난 곳은
익명 콜백 안이고 **담아 줄 이름 있는 조상이 위에 없다.** 규칙(*"가장 가까운 이름 있는
조상에 귀속"*)에 **조상이 없는 경우**가 빠져 있고, 그 코드는 어느 심볼에도 속하지
않는다.

이것은 F22-3 이 첫 구현에서 고발한 병의 **다른 형태다** — 그때는 *능력 부재*였고
(`Capable` 로 고쳤다) 이번은 **입자 부재**다. 둘 다 *"우리가 못 읽었다"* 를 *"변한 것이
없다"* 로 적는다.

**고치지 않았다.** 세는 규칙은 코드보다 먼저 등록됐고, 이 발견을 보고 규칙을 바꾸면
그것이 대조를 사후 조정하는 일이다. 지목만 해 둔다:

- `no_semantic_change` 를 **하나로 두지 않는다** — *"변한 심볼이 없다"* 와 *"변경이
  어느 심볼에도 담기지 않았다"* 는 다른 사실이고 후자는 `Uncapturable` 의 자리다
- 조상 없는 익명 함수를 파일 단위 잔여로 남길지 — **#47**(`partial` 회복)의 이웃

---

## 7. 표면이 하나 늘었다 — `pal symbols --graph`

②(파일 격리)를 **밖에서 잴 창**이 필요했다. `--json` 의 형태는 건드리지 않았다 —
`s0-compare.py` 가 그것을 JSON **배열**로 파싱하고 배열 길이가 S0 대조의 선언 수다.
형태를 바꿨으면 1,122 파일 대조가 깨진다.

`--graph` 는 **경로를 찍지 않는다.** 찍으면 ② 가 성립할 수 없다.

`surface/queries.toml`(stack §6)은 아직 없으므로 카탈로그 동기 검사에 걸리지 않는다.
**그 파일이 생기는 날 이 플래그가 거기 실려야 한다** — F06 의 몫으로 적어 둔다.

---

## 8. 재현

```bash
cargo build --workspace --release && cargo build --workspace
./scripts/s0-corpus.sh /tmp/s0-corpus ~/dev/projects/boxwood     # 1,122 파일
./scripts/f02-1-verify.py --s0-corpus /tmp/s0-corpus             # 다섯 다 통과
./scripts/f22-3-verify.py                                        # 음성 대조 실패 0 건
./scripts/f01-verify.py --repo ~/dev/projects/boxwood/portal-backend   # 여덟 다 통과
```

`s0-corpus.sh` 는 저장소 **셋**을 요구한다 — `portal-backend` · `portal-backend-aa-task` ·
**`boxwood-packages`**. 셋째가 없으면 1,105/1,122 에서 멈춘다(이 세션에서 실제로 걸렸다).

기준선: `cargo xtask check` 7/7 · 테스트 **161**(137 → 새 시험 24) ·
clippy 경고 **2**(`chain.rs:224` · `ledger.rs:357` — 둘 다 1.97 의 새 린트이고 회귀가
아니다. 이 조각은 하나도 더하지 않았다).
