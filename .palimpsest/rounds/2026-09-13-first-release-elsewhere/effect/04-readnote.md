# 읽은 줄 — ㈄ `touch` 13 과 호출자 목록

> 회차 `2026-09-13-first-release-elsewhere` · `E1` ④ · **변경 전에 커밋한다.**
> 좌표는 `effect/03-touch/NN-이름.txt:줄` · `effect/04-callers/NN-이름.txt` · `effect/04-touch-after-approve.txt:줄` 이다.

## 1. 읽은 줄과 알게 된 것

| 좌표 | 읽은 것 | 알게 된 것 |
|---|---|---|
| `03-touch/01-mcpServersFromToml.txt:9-11` | `■ 승인 대기 — … (1)` · `.ditto/knowledge/adr/ADR-0003-toml-parser.md · 결정 · 후보 3곳` · 승인 명령 | 이 함수에 **결정 문서 한 절이 걸리려 한다.** 후보가 3곳뿐이라 이 좌표에 관한 조각일 가능성이 크다 → 승인했다(`04-approve.txt`) |
| `04-touch-after-approve.txt:5-10` | `■ 이 좌표에 걸린 것 (1)` · `최신 상태(fresh)` · 본문 「`parseToml` wrapper 한 함수만 유지 · 사용 지점은 두 곳: `codex.ts` 의 `loadPermissions` 와 `mcpServersFromToml`」 | ★ **그 결정의 사용 지점 둘이 전부 지우려는 파일 안에 있다.** 대상 저장소를 확인했다 — `src/core/hosts/shared.ts:32` 의 `parseToml` 을 쓰는 곳은 `codex.ts` 뿐이고 `smol-toml` 도 그 래퍼만 쓴다. **`codex.ts` 를 지우면 ADR-0003 의 결정과 그 래퍼가 주인을 잃는다** |
| `03-touch/05-codexHostAdapter.txt:12` · `04-callers/05-codexHostAdapter.txt` | `호출자 2` → `src/cli/commands/setup.ts:258` · `src/core/setup.ts:291` | 파일 경계를 넘어 이 파일에 기대는 자리는 이 둘이다. 봉인 §4 의 사전 앎(`setup.ts:261` · `core/setup.ts:295`)과 **같은 두 파일**이다 — 줄 번호만 다르다(사전부검 반환문의 줄이 어긋나 있었다) |
| `03-touch/05-codexHostAdapter.txt:15` | `※ 호출자 수는 하한입니다 — 파일 최상위의 참조와 문자열로 찾는 자리는 세지 않습니다` | 등록 줄이 목록에 없다는 뜻이다. **`src/core/hosts/index.ts` 는 화면이 이름으로 안 댔다** — 봉인 §4 의 사전 앎으로만 안다 |
| `03-touch/05-codexHostAdapter.txt:9-10` | 승인 대기 1 · `reports/design/dual-host-surface-adapter-plan.md` §9 · **후보 7곳** | 넓게 퍼진 조각이다(좌표 7곳에 같이 걸리려 한다). 이 좌표에 관한 결정이라고 볼 근거가 없어 **승인하지 않았다** |
| `04-callers/01·02·03·04·11·12·13` | 호출자가 전부 `src/core/hosts/codex.ts` 안이다(`:162` · `:185` · `:231` · `:202`) | 함수·상수 여덟은 **파일 밖에서 안 불린다** — 이름마다 저장소를 훑어 확인할 걸음이 필요 없다 |
| `03-touch/06~10` 의 `-pick` 화면 · `04-callers/06~10` | 메서드 다섯은 호출자 0 · `※ x.foo() 는 아직 안 셉니다` | 메서드는 멤버 호출로만 불려서 **0 이 「안 쓰인다」가 아니다.** 이 다섯에 대해서는 화면이 답하지 않는다 — `tsc` 가 진다 |
| `03-touch/10-spawnRun-pick.txt` | 승인 대기 **9** | 흔한 이름이라 문서 여러 곳이 후보로 댄다. 승인하지 않았다 |

## 2. 그래서 계획이 바뀌었나 — 봉인 §5 와 대 본다

| 봉인 §5 의 걸음 | 지금 | 까닭(좌표) |
|---|---|---|
| 1 `codex.ts` 삭제 | 같다 | — |
| 2 `hosts/index.ts` 의 import · 등록 · 재수출 제거 | 같다 — **화면이 아니라 사전 앎이 근거다** | `05:15` 는 최상위 참조가 빠진다고만 말했다 |
| 3 `setup.ts` 둘의 참조 제거 | 같다 · 줄 번호를 화면 것으로 고쳐 잡았다 | `04-callers/05` |
| 4 `tsc --noEmit` 을 따라 고친다 | 같다 · **메서드 다섯은 여전히 `tsc` 에 맡긴다** | `03-touch/06~10` |
| 5 문자열 레지스트리 94 줄은 안 건드린다 | 같다 | — |
| **(새) 6** | **`shared.ts` 의 `parseToml` 이 주인을 잃는다 — 지우고, ADR-0003 의 상태 줄에 사용 지점이 사라졌음을 적는다.** `smol-toml` 의존 제거는 잠금 파일을 움직여 이번 조각에서 안 한다 | ★ `04-touch-after-approve.txt:8-10` 의 결정 본문 |
| 이름마다 저장소를 훑어 밖에서 쓰이는지 볼 걸음 | **안 한다** | `04-callers/01·02·03·04·11·12·13` 이 파일 안이라고 답했다 |

## 3. 걸음 ⑤ 의 순서

- **A** — `src/core/hosts/codex.ts` 만 지우고 커밋한다(부모 = `03-touch/head.txt` 의 HEAD). `tsc --noEmit` 을 A 전후로 떠 새 오류 파일 집합을 적는다(`E4`).
- **B** — 위 표의 2 · 3 · 4 · 6 을 하고 커밋한다. `tsc --noEmit` 이 오류 0 으로 돌아와야 한다.

## 4. 실행 하네스의 결함 — 사실대로 적는다

첫 승인 시도는 `pal` 이 아니라 **내 실행 스크립트의 결함**으로 실패했다(`04-approve-harness-error.txt`): 이 셸은 zsh 라서 따옴표 없는 변수를 낱말로 안 쪼갠다. 명령 한 줄이 인자 하나가 됐고 `pal` 이 인자 없이 불려 도움말을 찍었다. 같은 줄을 낱말로 갈라 다시 돌린 것이 `04-approve.txt` 다 — **고친 것은 낱말 가르기뿐이고 명령 문자열은 화면 그대로다.**
