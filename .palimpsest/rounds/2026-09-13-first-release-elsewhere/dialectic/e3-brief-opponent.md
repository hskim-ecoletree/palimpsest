# 반(反) 에게 주는 발췌 — 판 `e3-effect`

> 설계문 표 3 행이 반에게 주라고 한 것만 **원문 그대로** 뽑았다(스크립트) — 기계 사실 표와 해악도 등급 표. 쟁점 목록(K0~K6)은 안 준다.

### 기계 사실 표 — 정 · 반 · 합이 **다시 재지 않고 받는다**

| # | 사실 | 값 | 재는 법 |
|--:|---|---|---|
| M1 | `E1` 오라클 | 통과. 여섯 산출이 서로 다른 커밋에 진조상 순서로 들어갔다. 변경 A 의 부모가 `touch` 때의 HEAD 다 | `oracle/E1-order.txt` |
| M2 | `E2` 오라클 | 통과(⑴~⑷) | `oracle/E2-scene.txt` |
| M3 | `E4` 산출 | P = {`src/cli/commands/setup.ts`, `src/core/hosts/index.ts`, `src/core/setup.ts`}<br>C = {`src/cli/commands/setup.ts`, `src/core/hosts/codex.ts`, `src/core/setup.ts`}<br>D = {`src/core/hosts/codex.ts`}<br>P∩C = {`src/cli/commands/setup.ts`, `src/core/setup.ts`}<br>P∖(C∪D) = {`src/core/hosts/index.ts`}. 오류 자리 `2:34 TS2307` 은 최상위이고, 선언 안의 오류는 0 이다 | `oracle/E4-breakage.txt` |
| M4 | 복제본 사슬 | `d0eff0d^` = `898a479`<br>`898a479^` = `5ec95eb` = `effect/03-touch/head.txt` 의 HEAD<br>`5ec95eb^` = `aded7ce` | `git -C <복제본> rev-parse` |
| M5 | 복제본의 지금 워킹트리 | `?? .palimpsest/` · `?? node_modules` 두 줄뿐이다. `src tests scripts` 에는 변경이 없다 | `git status --porcelain` |
| M6 | `tsc --noEmit` | 전 0 → A 뒤 4 → B 뒤 0 | `effect/05-change.txt` · `effect/05-tsc-*.txt` |
| M7 | A 뒤 `tsc` 오류 네 줄 | `src/cli/commands/setup.ts(8,34) TS2307`<br>`src/cli/commands/setup.ts(262,50) TS7006`<br>`src/core/hosts/index.ts(2,34) TS2307`<br>`src/core/setup.ts(7,34) TS2307`<br>**`src/core/hosts/shared.ts` 는 오류에 없다** | `effect/05-tsc-after-A.txt` |
| M8 | 편집 파일 수 | A 1 · B 5 · 합 6 | `git show --stat` |
| M9 | B 의 헝크 | `git show -U0 d0eff0d` 로 **11** 이다: ADR-0003 1 · `cli/commands/setup.ts` 3 · `hosts/index.ts` 3 · `hosts/shared.ts` 2 · `core/setup.ts` 2. `effect/06-delta.md` 의 변경 끝 좌표는 **`-U0` 헝크 머리와 맞는다.** 기본 문맥(3줄)의 헝크 머리와는 다르다 | `git show -U0 d0eff0d \| grep '^@@'` |
| M10 | 화면 끝 좌표가 풀리나 | `03-touch/01-mcpServersFromToml.txt:9-11` → 승인 대기(1), ADR-0003, 후보 3곳, 승인 명령: **맞다**<br>`03-touch/05-codexHostAdapter.txt:9-10` → 승인 대기(1), 후보 7곳: **맞다**<br>`03-touch/05-codexHostAdapter.txt:12` → 읽은 줄 기록은 이 줄을 「호출자 2」로 인용했다. 실제 `:12` 는 `■ 이 심볼이 하는 것` 이고 「호출자 2 · 피호출자 1」은 `:13` 이다: **한 줄 어긋난다**<br>`03-touch/05-codexHostAdapter.txt:15` → 읽은 줄 기록과 차이 문서가 이 줄을 하한 문구로 인용했다. 실제 `:15` 는 「`x.foo()` 는 아직 안 셉니다」이고 하한 문구는 `:16` 이다: **한 줄 어긋난다**<br>`04-touch-after-approve.txt:8-10` → ADR-0003 결정 본문(`parseToml` wrapper, 사용 지점 두 곳 `loadPermissions` · `mcpServersFromToml`): **맞다**<br>`03-touch/10-spawnRun-pick.txt:9` → 승인 대기(9): **맞다** | `awk 'NR==N'` |
| M11 | `effect/05-callers-B/*` 의 정체 | `pal touch` 화면이 **아니다.** `■ symbol.callers` 질의 출력이고 스냅샷은 `898a479+worktree`(A 뒤)다 | 각 파일 머리 |
| M12 | `effect/04-callers/05-codexHostAdapter.txt` | `discoverCodexAgents src/cli/commands/setup.ts:258` · `writeCodexSurfaceCatalog src/core/setup.ts:291` | 파일 |
| M13 | 행과 셈 문구 | 봉인 §5 걸음 5<br>`06-delta.md` §1 행 6(§5 다섯 + 「없음」 한 행) · §2 행 1 · §3 행 3<br>§4 는 「차이 넷」이라 적는다. **어느 행이 「차이」인지는 기계가 못 가른다 → `K1`** | 행 세기 |
| M14 | 사전 앎의 문면 | `00-seal.md` §4 ① 은 잠그기 전에 그 조각을 「한 번 승인해 봤다」고만 적는다<br>`intent.md` 「착수 시점 관측」 표의 「목표 1 이 코드에서 막히나」 행은 그 실험에서 `touch` 가 **ADR-0003 「결정」 본문을 찍었다**고 적는다<br>**두 문면의 관계를 해석하는 일은 `K3` 이 한다** | 두 파일 인용 |
| M15 | 읽은 줄 기록이 변경보다 앞서나 | 앞선다. 읽은 줄 기록이 `c43f159`, 변경이 `1c9e2f4` 로 진조상 관계다 | M1 과 같음 |

### 해악도 등급 — 반이 제 반론에 스스로 매긴다. 합은 근거를 달아서만 다시 매긴다

| 등급 | 이 판에서의 뜻 |
|---|---|
| **금지역** | 판정이 잠긴 의도를 바꾸거나 돌아간다. 이 판에서 이 꼴이 넷이다: 합격선을 원문에 없는 것으로 세운다 · **사전 앎의 몫을 화면의 몫으로 세어 효과를 부풀린다**(사전부검 R1 ① 이 이 꼴을 금지역으로 적었다) · 모집단에서 차이를 까닭 없이 뺀다 · 봉인 뒤의 기록을 고쳐 좌표를 맞춘다 |
| **실패** | 판정값이 틀린다. 판정을 떠받치는 행의 좌표 끝이 그 말을 담지 않는다 · 방향이 뒤집힌다 · 통과 / 반증 / 대조 불가가 바뀐다 |
| **거짓신호** | 판정값은 그대로인데 기록이 사실과 다른 것을 말한다. 예: 판정을 떠받치지 않는 좌표의 어긋남(M10) · 셈 문구가 표와 맞지 않음(M13) |
| **미관** | 표기 · 문장 |
