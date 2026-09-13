# 합(合) — `E3` 판정 · 판 `e3-effect` r2

> 판 `e3-effect` · 라운드 **r2(상한)** · 회차 `2026-09-13-first-release-elsewhere`
> **낸 자리:** 합(合). 읽기 전용 서브에이전트다. 정도 반도 아니다. 판정문 본문을 반환문으로만 냈다. 파일은 하나도 안 바꿨고 커밋하지 않았다.
> **받은 것:** 초안 `dialectic/e3-thesis-r2.md` · 반론표 `dialectic/e3-antithesis-r2.md` · 발췌 `dialectic/e3-brief-opponent.md` · r1 합 `dialectic/e3-synthesis.md`(초안과 반론이 인용한 앞선 판정) · 인용 좌표를 열어 볼 원문(`effect/` · `intent.md` · `oracle/E1-order.txt` · `oracle/E2-scene.txt` · `oracle/E2-scene.py` · `oracle/E4-breakage.txt` · `oracle/E4-breakage.mjs` · `oracle/E4-literal-touch-c.txt`) · 복제본 `ditto-effect` 읽기 · `.claude/skills/round/SKILL.md` 의 인용 줄.
> **안 받은 것 · 안 연 것:** 토론 설계문 `dialectic/e3-design.md`(초안이 인용한 `:90-91` 도 안 열었다) · r1 정 `dialectic/e3-thesis.md` · r1 반 `dialectic/e3-antithesis.md` · 종료 판단 `dialectic/e3-referee.md` · `dialectic/p1-*` · `r1-raw.md` · 정과 반의 사고 과정 · 대화 기록 · `state.md` · `findings.jsonl` · `premortem/` · `conditions-audit/` · 앞 회차 `2026-09-11-effect-confirmation` 의 산출 · `docs/gates/effect-confirmation.md` · `docs/plan/03-shortest-path.md:140-146`. 복제본 `ditto-effect-at-A` 는 열지 않았다.
> **스스로 밝히는 오염.**
> ⑴ 환경 머리에 이 저장소의 최근 커밋 제목 다섯이 보였다. 그중 `f6a623d` 는 *"판 e3 반(反) r2 — 반론 열, 금지역 1 · 실패 3"* 이다. 추적 안 된 파일 이름 `docs/gates/first-release-elsewhere.md` 와 자동 메모리 색인도 보였다.
> ⑵ `git log -L119,119` 를 `intent.md` 에 돌려 첫 커밋 `e6ad3f2` 를 봤다. `git log -1` 로 `fc974c8` · `c43f159` 의 제목과 시각도 봤다.
> ⑶ `baseline/` 은 파일 이름 목록을 봤다. 그리고 `grep -l '사용 지점\|parseToml'` 이 걸린 파일 수(0)만 봤다. 내용은 안 열었다.
> **복제본에서 돌린 것(전부 읽기):** `git log` · `git show -U0 d0eff0d`(헝크 머리) · `git show aded7ce:.ditto/knowledge/adr/ADR-0003-toml-parser.md` · `git diff --stat aded7ce 5ec95eb -- .ditto/knowledge/adr/ADR-0003-toml-parser.md`(0 줄) · `git grep -n mcpServersFromToml 5ec95eb -- . ':!bin'` · `git grep -n 'parseToml\|smol-toml' 5ec95eb -- src tests scripts` · `git grep -n -i 'toml\|codex' 5ec95eb -- src/core/permission-inventory.ts` · `git grep -h "'codex'" 5ec95eb -- src tests | wc -l` · `git grep -n "getHostAdapter('codex')" 5ec95eb -- src tests`. 그리고 `5ec95eb:src/cli/commands/setup.ts:256-264` · `d0eff0d:src/core/run-with.ts:113` · `:169` · `d0eff0d:src/core/instruction-bridge.ts:155` · `:346` · `d0eff0d:src/core/hosts/types.ts:145-149` · `5ec95eb:.ditto/knowledge/adr/ADR-20260722-claude-code-only-host.md:15-19` · `d0eff0d:.ditto/knowledge/adr/ADR-0003-toml-parser.md:3` 를 열었다.
> 인용한 `effect/…:줄` · `oracle/…:줄` · `intent.md:줄` 은 전부 `awk 'NR==N'` 이나 `cat -n` 으로 직접 연 줄이다.
> **옮겨 적은 자:** 메인. 합(合) r2 에이전트는 읽기 전용이라 본문을 반환문으로만 냈다 — ```markdown 블록을 스크립트로 뽑아 한 글자도 안 바꾸고 옮겼다.

## 판정

**수정.** 초안의 판정값 「통과」는 서지 않는다. 초안이 「사전 앎의 몫을 화면의 몫으로 센 칸 0」(`dialectic/e3-thesis-r2.md:112`)을 세운 이력 근거 ⑶ 이 증거 구실을 못 한다(반론 1). 그 근거를 빼면 효과가 가장 큰 두 행 D2 · D3 에서 「안 닿았나」를 가를 좌표가 없다(반론 2 · 3). 그래서 이 합의 판정값은 **대조 불가**다(⑸). 살아남은 반론에 금지역이 있어서 **소유자에게 올린다.**

### ⑴ 합격선 읽기와 `intent.md` 결박

**합격선은 원문에 결박된다.**

| 합격선의 부분 | 결박 인용 | 읽기 |
|---|---|---|
| 판정 단위 · 두 끝 · 입력 | `intent.md:362` *"「걸음을 없앴나 · 덜 말했나 · 안 닿았나」를 차이마다 **좌표 두 끝**(화면 줄 → 변경 줄)으로 판정한다. 입력에 **과제 선택자의 사전 앎**과 `E4` 의 산출을 넣는다."* | 행마다 두 끝이 풀리고, 세 물음 모두에 답이 선다. 사전 앎과 `E4` 가 어느 칸에서 쓰였는지 보인다 |
| 세 낱말은 물음이다 | `intent.md:53` *"그 화면을 본 뒤 그 변경에서 무엇이 달라졌는지다(걸음을 없앴나 · 덜 말했나 · 안 닿았나)."* | 「안 닿았다」도 답의 하나다. 그래서 효과가 0 이어도 판정값은 떨어지지 않는다. 초안은 이 읽기를 `SKILL.md:714` 로 받쳤다. 그러나 그 줄은 게이트 문서에 산출을 붙이는 일을 말한다(반론 9). 이 합은 `intent.md:53` · `:362` 의 문면으로 받친다 |
| 파손 자리 대조 | `intent.md:165` *"파일 최상위 참조와 문자열 레지스트리 조회는 안 든다 (…) 실제 파손 자리와의 대조는 `E3` 가 진다."* | 계획과 실제가 같은 자리도 파손 대조 행으로 든다. 차이 행과 구별해서 센다 |
| 사전 앎의 몫 | `intent.md:167-168` *"그 사실을 봉인 문서 머리에 적고 효과 판정의 입력에 넣는다 — 숨기면 「덜 말했나」의 판정이 앞선 앎에 기운다."* · `intent.md:176`(잠그기 전 실험의 오염을 금지역으로 적었다) | 사전 앎의 몫을 화면의 몫으로 세지 않는다. **기록으로 안 갈리는 몫은 반박으로 지우지 않고, 안 갈린다고 적는다** |
| 「그 화면」 | `intent.md:46` `pal touch <그 심볼>` · `intent.md:137` *"㈄ 의 `touch` 는 **이 회차가 만든 바이너리**로 부른다"* | `pal query` 출력은 「그 화면」이 아니다. 착수 시점 바이너리로 찍은 잠그기 전 실험의 화면(`intent.md:119`)도 아니다 |

**판정값의 뜻.**
- **통과** — 모든 행의 두 끝이 풀린다. 세 물음 모두에 좌표로 답이 선다. 사전 앎과 `E4` 가 칸에서 쓰였다. 사전 앎의 몫을 화면의 몫으로 센 칸이 0 이다.
- **반증** — 좌표 끝이 그 답을 담지 않는 행이 있다.
- **대조 불가** — 합격선은 결박됐는데, 그것을 잴 좌표가 없거나 풀리지 않는다. 이 판에서는 **한 차이 행의 물음 하나를 가를 좌표가 없는 경우**가 여기에 든다.

### ⑵ 모집단

헝크: B 는 `git show -U0 d0eff0d` 로 11 개다(직접 쟀다). A 는 `codex.ts` 삭제 하나다.

**든 것 — 차이 행 넷 · 파손 대조 행 넷**

| 행 | 종류 | 변경 끝 |
|---|---|---|
| **D1** | 차이 — 계획보다 넓어졌다 | `src/cli/commands/setup.ts` `@@ -251,29 +249,0` · `@@ -288,5 +258,5` · `src/core/setup.ts` `@@ -291,10 +290,3` |
| **D2** | 차이 — 봉인에 없던 편집 | `src/core/hosts/shared.ts` `@@ -3 +2,0` · `@@ -32,4 +30,0` |
| **D3** | 차이 — 봉인에 없던 편집 | `.ditto/knowledge/adr/ADR-0003-toml-parser.md` `@@ -3 +3` |
| **D4** ★이 합이 더했다 | 차이 — 하지 않은 걸음: 13 개 이름마다 저장소를 훑는 걸음 | 헝크 없음. 차이 문서가 `effect/06-delta.md:18` *"## 2. 계획에 있었는데 안 한 걸음"* 아래 `:22` 에 이 걸음을 적었다 |
| **K1** | 파손 대조 — 계획 = 실제 | `src/core/hosts/index.ts` `@@ -2 +1,0` · `@@ -5 +3,0` · `@@ -9 +7` |
| **K2** | 파손 대조 — 계획 = 실제 | `src/cli/commands/setup.ts` `@@ -8 +7,0` · `src/core/setup.ts` `@@ -7 +6,0` |
| **K3** | 파손 대조 — 자기 헝크 없음 | D1 의 `@@ -251,29 +249,0` 안에서 사라졌다 |
| **K4** | 파손 대조 — 계획 = 실제 · 헝크 없음. 레지스트리 조회 | 봉인 걸음 5(`effect/00-seal.md:65`)가 안 건드리기로 했고 실제도 같다 |

**D4 를 더한 까닭.** 초안은 이 걸음을 X1 로 뺐고, 까닭으로 「봉인 §5 에 없다」와 「근거가 질의 출력이다」를 댔다(`dialectic/e3-thesis-r2.md:70`).
- 첫째 까닭은 봉인 저자의 차이 문서와 어긋난다. 그 문서는 이 걸음을 「계획에 있었는데 안 한 걸음」(`effect/06-delta.md:18`)으로 분류했고, *"`tsc` 전에 할 법한 걸음"*(`:22`)이라 적었다.
- 둘째 까닭은 방향의 답이지 모집단에서 뺄 까닭이 아니다.
- 이 걸음은 D2 · D3 의 반사실에 곧장 닿는다(반론 3).

**뺀 것과 까닭**

| # | 무엇 | 까닭 |
|---|---|---|
| X0 | A 의 `codex.ts` 삭제 | 계획 1(`effect/00-seal.md:61`)과 같다. 과제 그 자체다(`effect/00-seal.md:11-12`). 파손 자리가 아니라 파손의 원인이다 |
| X2 | 줄 번호를 「고쳐 잡은」 것(`effect/04-readnote.md:25`) | 편집 헝크에 차이가 없다. `5ec95eb:src/cli/commands/setup.ts:258` 은 선언 줄이고 `:261` 은 참조 줄이다(직접 열었다) |
| X3 | `bin/ditto` 가 B 뒤에도 낡은 채 남았다 | 봉인에도 편집에도 없어 차이가 아니다. 깨지지 않고 낡을 뿐이라 파손 자리도 아니다. 초안 · r1 합과 같다 |
| X4 | 화면 `effect/04-touch-after-approve.txt:10` 의 둘째 문장(셋째 사용처) | 행으로 세우지 않는다. 대신 **D2 의 방향 칸에 적는다**(반론 5) |
| X5 | `effect/06-delta.md` §1 「없음」 행 · §3 | D2 · D3 · K1 · K3 · K4 로 흡수했다 |

### ⑶ 차이별 표

| 행 | 차이 | 화면 끝 | 변경 끝 | 사전 앎 좌표 | 사전 앎을 뺀 반사실 | 방향 (세 물음) | 가장 강한 대안 원인과 반박 |
|---|---|---|---|---|---|---|---|
| **D1** | 봉인 걸음 3 은 *"참조 제거"*(`effect/00-seal.md:63`)였다. 실제로는 `discoverCodexAgents` 를 함수째 지웠고, codex 갈래 둘을 `throw` 로 바꿨다 | `effect/03-touch/05-codexHostAdapter.txt:13` *"호출자 2 · 피호출자 1"*. 수만 싣는다(`oracle/E4-literal-touch-c.txt:7` *"C(호출자 파일) 없음"*) | ⑵ D1 | `effect/00-seal.md:52-53` · `:65` | 안 바뀐다. `tsc` 가 두 파일을 댄다(`effect/05-tsc-after-A.txt:1` · `:4`). 「시끄러운 실패」라는 편집 꼴은 대상 저장소 결정문에 있다(`5ec95eb:.ditto/knowledge/adr/ADR-20260722-claude-code-only-host.md:17`) | 없앴나 **아니다** · 덜 말했나 **그렇다**. 수는 댔지만 자리를 안 실었다. 화면이 밝힌 한계(`:14-16`)에 이 꼴은 없다 · 안 닿았나 **그렇다**. 원인은 질의다(`effect/06-delta.md:13`) | 대안: 질의도 `pal` 이니 도구가 닿았다. 반박: 「그 화면」은 `touch` 다(`intent.md:46` · `:137`). `effect/04-callers/05-codexHostAdapter.txt:2` 의 머리는 `■ symbol.callers` 다 |
| **D2** | 봉인에 없던 편집: `shared.ts` 의 `smol-toml` import 와 `parseToml` wrapper 를 지웠다 | `effect/03-touch/01-mcpServersFromToml.txt:10` · `:11` → 승인 → `effect/04-touch-after-approve.txt:9` · `:10` | ⑵ D2 | `effect/00-seal.md:48-50`(후보를 알았고 한 번 승인해 봤다) · `intent.md:119`(그 실험의 `touch` 가 ADR-0003 「결정」 본문을 찍었다. 첫 커밋 `e6ad3f2` 2026-09-13 19:58 은 봉인 `fc974c8` 22:48 보다 앞선다) · 그 본문은 대상 저장소에 글자 그대로 있다: `aded7ce:.ditto/knowledge/adr/ADR-0003-toml-parser.md:25` · `:26`. `aded7ce` → `5ec95eb` 사이 diff 는 0 줄이다 | **안 갈린다.**<br>⑴ 봉인 §5 에 이 걸음이 없다(`effect/00-seal.md:61-65`). 그러나 §5 는 스스로 입력을 *"위 사전 앎만으로"*(`:59`)라 한정했다. 그 §4 ① 에는 본문 노출이 없다(`:48-50`).<br>⑵ `tsc` 는 `shared.ts` 에 닿지 않는다(`effect/05-tsc-after-A.txt:1-4`). 그러나 차이 문서가 *"할 법한 걸음"* 으로 적은 grep(`effect/06-delta.md:22`)은 닿는다. `git grep -n mcpServersFromToml 5ec95eb` 의 첫 줄이 `.ditto/knowledge/adr/ADR-0003-toml-parser.md:26` 이다.<br>⑶ 초안의 이력 근거는 **지운다**(반론 1). 잠그기 전 실험의 원 출력은 `baseline/` 에 없다(목록 일곱 파일에서 grep 0) | 없앴나 **아니다**.<br>덜 말했나 **아니다 — 단 틀리게도 말했다.** 같은 줄 `:10` 이 `permission-inventory.ts` 의 셋째 사용처를 말한다. 그러나 `5ec95eb` 에서 `parseToml` · `smol-toml` 을 쓰는 곳은 `codex.ts` · `shared.ts` 뿐이다(`git grep` 출력). 제거는 대상 저장소를 확인한 뒤 정해졌다(`effect/04-readnote.md:11`).<br>안 닿았나 **좌표로 안 갈린다.** 변경 전 기록(`effect/04-readnote.md:28`, 커밋 순서 `oracle/E1-order.txt:13-14`)은 원인을 이 화면 줄로 적었다. 그러나 「화면이 없었으면 이 걸음이 없었다」를 받칠 좌표가 없다 | 대안 ①: 잠그기 전 노출(`intent.md:119`)이 원인이다.<br>대안 ②: 도구 없는 세계의 grep 걸음(`effect/06-delta.md:22`)이 같은 본문에 닿는다.<br>**둘 다 반박할 좌표가 없다.** 반대편 좌표 `effect/04-readnote.md:11` · `:28` 은 봉인과 같은 저자의 기록이다 |
| **D3** | 봉인에 없던 편집: ADR-0003 상태 줄에 사용 지점이 사라졌음을 적었다(`d0eff0d:.ditto/knowledge/adr/ADR-0003-toml-parser.md:3`) | `effect/04-touch-after-approve.txt:10` | ⑵ D3 | D2 와 같다 | D2 와 같다. 게다가 마크다운은 `tsc` 밖이다 | D2 와 같다. 안 닿았나 **좌표로 안 갈린다** | D2 와 같다. 과제 결정문의 제거 대상(`5ec95eb:.ditto/knowledge/adr/ADR-20260722-claude-code-only-host.md:18`)에 ADR-0003 은 없다 |
| **D4** | 13 개 이름마다 저장소를 훑는 걸음을 하지 않았다 | `effect/03-touch/01-mcpServersFromToml.txt:13` *"호출자 1 · 피호출자 5"* 처럼, 화면은 수만 싣는다 | ⑵ D4 | `effect/00-seal.md:52-54` | 안 바뀐다. 이 걸음을 없앤 근거는 질의 출력이다(`effect/04-readnote.md:15` → `04-callers/01·02·03·04·11·12·13`) | 없앴나 **그렇다 — 질의가 없앴다. 화면 몫은 아니다** · 덜 말했나 **그렇다**(수만 실었다) · 안 닿았나 **그렇다**(화면으로는) | 대안: 이 걸음은 기준선에 없었으니 차이가 아니다. 반박: 봉인 저자의 `effect/06-delta.md:18` · `:22` 가 차이로 분류했다. 다만 기준선에 드는지를 두고 `:18` 과 `:22` 의 문면이 서로 어긋난다 |
| **K1** | `hosts/index.ts` 의 import · 등록 · 재수출. 계획 2 와 같다 | `effect/03-touch/05-codexHostAdapter.txt:16`(하한 문구) | ⑵ K1 | `effect/00-seal.md:52-53` | 안 바뀐다. `effect/05-tsc-after-A.txt:3` | 없앴나 **아니다** · 덜 말했나 **그렇다 — 한계 안**(`oracle/E4-breakage.txt:9` · `:12`) · 안 닿았나 **그렇다**(`effect/04-readnote.md:24`) | 대안: 하한 문구가 이 파일을 보게 했다. 반박: 변경 전 기록 `effect/04-readnote.md:24` 가 근거를 사전 앎으로 적었다 |
| **K2** | `setup` 두 파일의 import 줄. 계획 3 과 같다 | `effect/03-touch/05-codexHostAdapter.txt:13`(수만) | ⑵ K2 | `effect/00-seal.md:53` | 안 바뀐다. `effect/05-tsc-after-A.txt:1` · `:4` | 없앴나 **아니다** · 덜 말했나 **그렇다.** 오류 자리가 최상위라는 점은 한계 안이다(`oracle/E4-literal-touch-c.txt:13` · `:20`). 파일 이름을 안 실은 것(`:7`)은 한계 밖이다 · 안 닿았나 **그렇다** | 대안: 수 2 가 사전 앎을 확인해 주었다. 반박: 봉인 §5 에 「확인」 걸음이 없다(`effect/00-seal.md:61-65`) |
| **K3** | 멤버 호출 `5ec95eb:src/cli/commands/setup.ts:261`. `tsc` 가 드러낸 자리는 `:262` 의 매개변수 `s` 다(`effect/05-tsc-after-A.txt:2`) | `effect/03-touch/09-loadSurfaceInventory-pick.txt:15` *"호출자 0"* · `:17` | D1 의 `@@ -251,29 +249,0` | `effect/00-seal.md:53` | 안 바뀐다 | 없앴나 **아니다** · 덜 말했나 **그렇다 — 한계 안**(`:17`). `oracle/E4-literal-touch-c.txt:14` 의 「선언 안」은 **import 실패에서 번진 것으로 읽는다**. 따로 빠뜨린 호출자 자리로 세지 않는다(반론 7) · 안 닿았나 **그렇다** | 대안: 번짐이다. 받는다. 추정이고 재실행은 하지 않았다 |
| **K4** | 레지스트리 조회. `getHostAdapter('codex')` 는 `src`·`tests` 에 한 줄이다(`5ec95eb:src/core/instruction-bridge.ts:155`). 변수로 찾는 자리는 `d0eff0d:src/core/run-with.ts:113` → `:169` 와 `d0eff0d:src/core/instruction-bridge.ts:346` 이다. 모두 `d0eff0d:src/core/hosts/types.ts:148` 에서 던진다 | `effect/03-touch/05-codexHostAdapter.txt:16` | ⑵ K4 | `effect/00-seal.md:54` · `:65`. 사전 앎의 「94 줄」은 `'codex'` 문자열이 든 줄 전체의 수다(`git grep -h` 94). 레지스트리 조회의 수가 아니다 | 안 바뀐다 | 없앴나 **아니다** · 덜 말했나 **그렇다 — 한계 안**(`:16`) · 안 닿았나 **그렇다** | 대안: 던지는 것은 의도된 동작이다. 반박: `intent.md:165` 가 이 자리를 파손 대조의 몫으로 이름했다 |

**반사실 요약.** D1 · D4 · K1 ~ K4 는 좌표와 함께 「안 바뀐다」다. **D2 · D3 은 「안 갈린다」다.** 두 행을 화면의 몫으로 셀 근거는 봉인 §5 와 같은 저자의 읽은 줄 기록뿐이다. 그 §5 는 본문 노출을 제 입력에서 뺐고(`effect/00-seal.md:59` · `:48-50`), 차이 문서는 §5 가 할 법한 걸음 하나를 빠뜨렸다고 적는다(`effect/06-delta.md:22`).

**방향 분포(기록이지 게이트가 아니다).**
- 「없앴다」: 화면 몫으로는 0 행이다. 질의 몫으로 D4 한 행이 있다.
- 차이 행: D1 · D4 는 화면이 안 닿았다. D2 · D3 은 셋 밖(걸음이 더해졌다)이고, 화면이 닿았는지가 안 갈린다.
- 파손 대조 행 K1 ~ K4: 넷 다 화면이 안 닿았고, 넷 다 덜 말했다. 한계 안이 K1 · K3 · K4, 한계 밖이 섞인 것이 K2 다.

### ⑷ `E4` 산출이 어느 행에 쓰였나

| 산출 | 쓰인 행 | 무엇의 근거 |
|---|---|---|
| `oracle/E4-breakage.txt`(C 는 질의에서 뽑았다: `oracle/E4-breakage.mjs:5` · `:42`) | K1(`:9` · `:12`) · K2(`:8`) | 질의 기준으로는 최상위 밖의 누락이 0 이다. 질의의 몫을 가른다 |
| `oracle/E4-literal-touch-c.txt`(조건 문면 `intent.md:363` 대로 C = ∅) | D1 · K2(`:7` · `:13` · `:20`) · K3(`:14` — 번짐의 기록으로만) | 「그 화면」은 파손 파일을 이름으로 하나도 대지 않았다(`:7`). **`:14` 를 「한계 밖」의 근거로 쓰지 않는다**(반론 7) |
| 두 산출의 P(`oracle/E4-breakage.txt:5` · `oracle/E4-literal-touch-c.txt:6`) | D2 · D3 | `shared.ts` 도 ADR 파일도 P 에 없다. `tsc` 반사실은 두 행에 닿지 않는다. 그러나 그것만으로 「도구 없이 이 걸음이 없었다」는 안 선다(D2 반사실 ⑵) |

### ⑸ `E3` 판정값 — **대조 불가**

- **합격선은 결박된다**(⑴).
- **반증은 아니다.** 여덟 행 모두 두 끝이 풀린다. D2 의 끝 `effect/04-touch-after-approve.txt:10` 은 *"사용 지점은 두 곳: `src/core/hosts/codex.ts`의 …"* 를 글자로 담는다. 같은 줄의 거짓 문장은 방향 칸에 적었다.
- **통과도 아니다.** D2 · D3 의 「안 닿았나」를 가를 좌표가 없다. 초안이 그 칸을 세운 근거는 셋이었다.
  - ⑶ 이력은 증거 구실을 못 한다(반론 1).
  - ⑴ 기준선은 스스로 입력을 §4 로 한정했고, §4 에 본문 노출이 없다(반론 2).
  - ⑵ 도구 반사실은 봉인 저자가 적은 grep 걸음을 빠뜨렸다(반론 3).
  - 가를 좌표가 될 수 있었던 잠그기 전 실험의 원 출력은 `baseline/` 에 없다.
  그러니 「사전 앎의 몫을 화면의 몫으로 센 칸 0」은 재지 않았다. 그것을 통과로 세면 안 잰 것을 잰 것으로 세는 셈이다.
- 이 값을 뒤집을 수 있는 것은 소유자의 결정 하나다. 봉인 §5 와 변경 전 읽은 줄 기록을 D2 · D3 의 귀속 좌표로 받으면 D2 · D3 의 「안 닿았나」가 「아니다」로 서고, `E3` 는 통과다. 그 결정은 합이 대신 못 한다(아래 「소유자에게」).

**판정과 따로 남기는 기록.**
- 화면이 가장 크게 움직인 두 행(D2 · D3)은 세 낱말 밖이다. 세 물음의 틀에는 「걸음을 더했다」가 없다.
- 게이트에 `E3` 판정값을 적을 때는 방향 분포를 함께 적는다. 값만 남기면 효과가 확인됐다고 읽힌다(반론 9).
- `effect/06-delta.md:35` 는 *"차이 넷"* 이라 적는다. 이 판정의 차이 행은 넷이지만 구성이 다르다(D4 가 들고 K1 은 파손 대조다).
- 화면 줄을 한 줄 어긋나게 인용한 곳: `effect/04-readnote.md:12` · `:13` · `effect/06-delta.md:12` · `:13` · `:28`. 셋이 `:15` 로 적은 하한 문구는 실제로 `:16` 이다.
- X3(`bin/ditto` 가 낡았다).
- `oracle/E2-scene.py:55` 는 `E2` ⑵ 를 `04-callers`(질의 출력)에서 뽑는다. 이 판이 쓰는 「그 화면 = `touch`」 읽기(`intent.md:46` · `:137`)와 어긋난다. `E2` 의 몫이라 여기서 판정하지 않는다(반론 6).

## 채택한 반론

| # | 반론 | 왜 채택했나 | 해악도 |
|---|---|---|---|
| 1 | D2 반사실 ⑶ 「기록에 재료가 없다」는 palimpsest 저장소만 뒤졌다. 재료는 대상 저장소에 글자 그대로 있었다 | 직접 확인했다. `aded7ce:.ditto/knowledge/adr/ADR-0003-toml-parser.md:25` · `:26` 이 *"wrapper 한 함수만 유지"* · *"사용 지점은 두 곳: `src/core/hosts/codex.ts`의 `loadPermissions`와 `mcpServersFromToml`"* 이다. `aded7ce` → `5ec95eb` diff 는 0 줄이다. `intent.md:119` 는 봉인보다 앞선 커밋(`e6ad3f2` 19:58 < `fc974c8` 22:48)에서, 그 실험의 `touch` 가 바로 이 「결정」 본문을 찍었다고 적는다. 초안은 같은 D2 행의 사전 앎 칸에 `intent.md:119` 를 대 놓고(`dialectic/e3-thesis-r2.md:81`), 반사실 칸에서는 「기록에 재료가 없다」고 적었다. 노출은 화면을 본 것이지 문장을 적은 것이 아니다. 그래서 palimpsest 저장소에 `-S` 를 돌려서는 원리상 안 걸린다. 초안은 이 근거로 조건을 떼고 「사전 앎의 몫을 화면의 몫으로 센 칸 0」(`:112`)을 세웠다. 짐을 지는 근거이므로 발췌 `dialectic/e3-brief-opponent.md:29` 의 금지역 꼴(사전 앎의 몫을 화면의 몫으로 세어 효과를 부풀린다)에 든다. 등급을 내릴 근거가 없다 | 금지역 |
| 2 | 봉인 §5 는 입력을 §4 로 한정했고, §4 에는 본문 노출이 없다. 그래서 D2 · D3 에서 「사전 앎 몫 0」을 잴 좌표가 없다 | `effect/00-seal.md:59` *"(위 사전 앎만으로)"* · `:48-50` 은 「후보가 있다 · 한 번 승인해 봤다」까지만 적는다. `intent.md:119` 는 본문을 찍었다고 적는다. 초안의 조건 떼기 사유 2(`dialectic/e3-thesis-r2.md:116`)와 대조 불가 기각(`:130` *"둘 다 됐다"*)은 §5 를 노출 전체에 대한 기준선으로 전제한다. 원문은 그 전제를 받치지 않는다. ⚠ 한 부분은 받지 않는다. *"`intent.md:167` 의 처방이 덜 이행됐다"* 는 주장이다. `intent.md:167` 이 봉인 머리에 적으라고 한 사실은 「후보로 걸린다는 것」이고, `effect/00-seal.md:48-49` 가 그것을 적었다. 등급은 판정값이 바뀐다는 주장대로 둔다. ⑸ 가 그렇게 바뀌었다 | 실패 |
| 3 | X1(이름마다 저장소를 훑는 걸음)을 빼서, 같은 본문에 닿는 grep 반사실이 도구 반사실 ⑵ 에서 빠졌다 | `effect/06-delta.md:22` *"`tsc` 전에 할 법한 걸음"* 과 `:18` 의 분류 *"계획에 있었는데 안 한 걸음"* 이 봉인 저자의 기록이다. `git grep -n mcpServersFromToml 5ec95eb -- . ':!bin'` 의 첫 줄이 `.ditto/knowledge/adr/ADR-0003-toml-parser.md:26` 이다(직접 돌렸다). 초안 ⑵(`dialectic/e3-thesis-r2.md:81`)는 `tsc` · biome · `adr-guard` 만 봤다. grep 을 실제로 했을지는 기록으로 못 잰다. 그러나 이 반론은 「안 닿았나 — 아니다」를 받칠 좌표가 없다는 데서 선다. 그 칸이 D2 · D3 의 방향과 판정값을 떠받친다. 그래서 등급을 내리지 않는다. 모집단에 D4 를 더한 것도 이 반론에서 나왔다 | 실패 |
| 5 | D2 의 원인 줄 `:10` 은 같은 줄에서 거짓인 셋째 사용처를 말한다. 초안은 한 줄을 갈라 첫 문장만 원인으로 썼다 | `effect/04-touch-after-approve.txt:10` 에 *"`permission-inventory.ts`의 codex 분기는 wrapper를 통해 nested section을 직접 traverse"* 가 있다. `git grep -n 'parseToml\|smol-toml' 5ec95eb -- src tests scripts` 에서 사용처는 `codex.ts` · `shared.ts` 뿐이다. `permission-inventory.ts` 의 `toml` · `codex` grep 에는 TOML import 가 없다. 제거는 대상 저장소 확인(`effect/04-readnote.md:11`) 뒤에 정해졌다. 판정값은 안 바뀐다. 끝 `:10` 이 「사용 지점 두 곳」을 글자로 담기 때문이다 | 거짓신호 |
| 6 | 「그 화면 = `touch`」 읽기와 `E2` ⑵ 의 오라클이 서로 어긋난다 | `oracle/E2-scene.py:55` 가 `04-callers` 를 읽는다. `intent.md:361` 은 *"그 출력들에서"*, 곧 `touch` 출력이라 적는다. `E3` 의 행과 판정값은 안 바뀐다(초안은 `E2` ⑵ 를 어느 행에도 쓰지 않았다). 그러나 같은 입력을 두 기준으로 읽는 기록이 남는다 | 거짓신호 |
| 7 | `oracle/E4-literal-touch-c.txt:14` 의 「선언 안 1」은 번짐의 산물이라 「한계 밖」의 근거가 못 된다 | 초안 K3 가 번짐 대안을 받았다(`dialectic/e3-thesis-r2.md:85`). `5ec95eb:src/cli/commands/setup.ts:262` 는 `inventory.localSurfaces.filter((s) => …)` 이고, `inventory` 의 타입은 `:261` 의 `codexHostAdapter` 에서 온다. 그 import 가 `effect/05-tsc-after-A.txt:1` 에서 실패했다. 담는 함수 `discoverCodexAgents` 는 질의가 이미 낸 호출자다(`effect/04-callers/05-codexHostAdapter.txt:4`). D1 · K2 의 「한계 밖」은 `:7`(이름 없음)만으로도 서니 판정값은 그대로다 | 거짓신호 |
| 8 | K4 가 물려받은 「94 줄」은 레지스트리 조회의 수가 아니고, 변수로 찾는 조회가 빠졌다 | `git grep -h "'codex'" 5ec95eb -- src tests` 는 94 줄이다. 그중 `getHostAdapter('codex')` 는 `5ec95eb:src/core/instruction-bridge.ts:155` 한 줄이다. `d0eff0d:src/core/run-with.ts:113` 이 `'codex'` 를 돌려주고 `:169` 가 `getHostAdapter(provider)` 를 부른다. `d0eff0d:src/core/instruction-bridge.ts:346` 도 `getHostAdapter(host)` 다. 모두 `d0eff0d:src/core/hosts/types.ts:148` 에서 던지므로 「시끄러운 실패」는 선다. 반이 댄 세부 셈 가운데 `=== 'codex'` 는 내 `grep \| wc -l` 로 30 줄이라 32 와 다르다. 반론의 요지는 바꾸지 않는다 | 거짓신호 |
| 9 | (다) 「효과가 보여야 통과」를 `SKILL.md:714` 로 기각한 것은 인용이 맞지 않는다. 판정값만 남으면 효과 확인으로 읽힌다 | `SKILL.md:712` 는 *"## 8. 효과 — 산출이 있어야 지난 것이다"* 이고, `:714` 는 게이트 문서에 산출을 붙이는 일이다. ⟨정반합⟩ 조건의 통과 뜻을 말하지 않는다. (다) 를 기각하는 결론은 `intent.md:53` · `:362` 의 문면(세 낱말은 부정의 답을 포함한 물음이다)으로 선다. 그래서 판정값은 안 바뀌고, 근거 줄과 게이트 기록만 고친다. 「읽는 사람이 오해한다」는 부분은 반 스스로 추정으로 매겼다 | 거짓신호 |
| 10 | 줄 어긋난 인용 목록에 `effect/06-delta.md:28` 이 빠졌다 | `effect/06-delta.md:28` 은 `03-touch/05-codexHostAdapter.txt:15` 를 하한 문구로 적는다. 실제 하한 문구는 `:16` 이다(`cat -n` 으로 확인했다) | 미관 |

## 기각한 반론

| # | 반론 | 왜 기각했나 (근거 필수) |
|---|---|---|
| 4 | 초안은 r1 합이 소유자에게 올린 물음을 정(正)의 추론으로 닫았다. 올림 규칙에 답하지 않았고, r1 합이 올린 까닭도 반박하지 않았다 (실패) | ⑴ 올림 규칙 `SKILL.md:447` *"살아남은 반론이 금지역·실패에 닿으면 → 소유자에게 올린다"* 가 거는 대상은 **이번 합이 채택한 반론**이다. 정이 조건을 뗀다고 적은 것은 초안의 주장이고, 올림을 우회하는 절차가 아니다. 이 판정문의 「소유자에게」가 그 규칙을 이번 라운드에서 다시 건다. 이 반론을 받든 안 받든 올림은 일어난다.<br>⑵ *"r1 합이 올린 까닭이 반박되지 않았다"* 는 문면과 다르다. 초안 `dialectic/e3-thesis-r2.md:116` 이 r1 합의 물음(`dialectic/e3-synthesis.md:151`)을 이름으로 들고 `intent.md:135` · `:176` · `:167-168` 로 답하려 했다. 그 답이 서지 않는다는 것은 반론 1 · 2 · 3 이 채택으로 이미 담았다.<br>⑶ 이 반론만의 몫(「정이 올림을 닫았다」)은 판정값을 바꾸지 않는다. 판정값은 ⑸ 가 반론 1 · 2 · 3 의 좌표로 정했다 |

## 초안을 어떻게 고치나

1. **D2 · D3 의 반사실 ⑶(이력) 을 지운다**(반론 1). 반사실 칸의 「안 바뀐다」를 「안 갈린다」로 바꾸고, 대상 저장소 좌표 `aded7ce:.ditto/knowledge/adr/ADR-0003-toml-parser.md:25-26` 과 diff 0 줄을 사전 앎 칸에 더한다.
2. **판정값의 근거 `dialectic/e3-thesis-r2.md:112` 의 「센 칸은 0 이다」를 지운다.** D2 · D3 은 「기록으로 안 갈린다 — 화면 몫으로 세지 않는다」로 적는다(반론 1 · 2).
3. **조건 떼기 사유 2 · 3(`:116` · `:117`)과 대조 불가 기각(`:130`)을 거둔다.** §5 가 입력을 §4 로 한정했고(`effect/00-seal.md:59`), §4 에 본문 노출이 없다는 사실(`:48-50` 대 `intent.md:119`)을 적는다(반론 2).
4. **X1 을 모집단의 차이 행 D4 로 옮긴다.** D2 반사실 ⑵ 에 grep 경로(`effect/06-delta.md:22` · `git grep -n mcpServersFromToml 5ec95eb` 첫 줄)를 더한다(반론 3).
5. **D2 방향 칸에 「틀리게도 말했다」를 적는다.** `:10` 둘째 문장과 `5ec95eb` 의 grep 결과를 댄다. X4 는 행 밖으로 빼지 않고 이 칸으로 흡수한다(반론 5).
6. **`E2` ⑵ 의 오라클 출처(`oracle/E2-scene.py:55`)가 「그 화면」 읽기와 어긋난다는 기록**을 판정과 따로 남기고, `E2` 쪽으로 넘긴다(반론 6).
7. **⑷ 에서 `oracle/E4-literal-touch-c.txt:14` 를 「한계 밖」의 근거로 쓰지 않는다.** D1 · K2 는 `:7` 로 받친다. K3 의 `:14` 는 번짐의 기록으로만 둔다(반론 7).
8. **K4 의 서술을 고친다.** 조회는 `getHostAdapter('codex')` 1 줄과 변수 경유 조회(`d0eff0d:src/core/run-with.ts:169` · `d0eff0d:src/core/instruction-bridge.ts:346`)다. 「94」는 `'codex'` 문자열이 든 줄 전체의 수로 적는다(반론 8).
9. **(다) 기각의 근거를 `SKILL.md:714` 에서 `intent.md:53` · `:362` 로 바꾼다.** 게이트에 판정값을 적을 때 방향 분포를 함께 싣는다(반론 9).
10. **줄 어긋난 인용 목록에 `effect/06-delta.md:28` 을 더한다**(반론 10).
11. **⑸ 의 판정값을 「대조 불가」로 적는다.** 뒤집을 수 있는 것은 아래 소유자의 결정 하나라고 적는다.

## 소유자에게

**올린다.**

**왜.** 살아남은 반론 중 가장 무거운 것이 **금지역**(반론 1)이고, **실패**가 둘(반론 2 · 3) 있다. 셋 다 한 자리를 가리킨다. 효과가 가장 큰 D2 · D3 을 화면의 몫으로 셀 수 있는지다. 그 근거로 남는 것은 봉인 §5 와 읽은 줄 기록뿐이다. 둘 다 잠그기 전에 같은 ADR-0003 결정 본문을 본 사람(`intent.md:119`)의 기록이다. 그 저자의 차이 문서는 §5 가 할 법한 걸음 하나를 빠뜨렸다고도 적는다(`effect/06-delta.md:22`). 이 기준선을 받을지를 그 기록을 쓴 메인이 정하면, 자기 기준선을 자기가 판정하게 된다. 상한 r2 에 이르렀으니 합이 더 돌리지 않는다.

| 물음 | 칸 |
|---|---|
| 잠그기 전 실험에서 ADR-0003 결정 본문을 이미 본 저자가 쓴 봉인 §5(`effect/00-seal.md:57-65`)와 변경 전 읽은 줄 기록(`effect/04-readnote.md:11` · `:28`)을, D2 · D3(`shared.ts` wrapper 제거 · ADR-0003 상태 줄)의 원인을 이번 `pal touch` 화면으로 가르는 좌표로 받아 `E3` 를 **통과**로 닫나, 아니면 이 합대로 **대조 불가**로 두나? | ⟨소유자⟩ |

## 내가 못 정한 것

| 무엇 | 왜 못 정했나 | 무엇이 있어야 정하나 |
|---|---|---|
| D2 · D3 에서 화면 · 잠그기 전 노출 · grep 경로의 몫 | 잠그기 전 실험의 원 출력이 `baseline/` 에 없다(목록 일곱 파일에서 grep 0). 봉인 §4 는 본문 노출을 적지 않았다. 반대편 좌표는 같은 저자의 기록이다 | 잠그기 전 실험의 `touch` 원 출력과 그 세션의 기록. 그것이 있어도 「§5 에서 일부러 뺐나」라는 의도는 기록으로 못 잰다 → 소유자 |
| `dialectic/e3-design.md:90` 의 「대조 불가」 정의가 행 하나의 물음을 가를 좌표가 없는 경우까지 덮나 | 그 파일을 받지 않았고 열지 않았다. 이 합은 ⑴ 의 원문 결박과 `SKILL.md:111` 의 값 셋 안에서 읽었다 | 설계문 `:90` 원문 |
| `TS7006`(`262,50`)이 import 실패에서 번졌나(K3 · 반론 7) | `tsc` 를 다시 돌리지 않았다. 코드 줄(`5ec95eb:src/cli/commands/setup.ts:261-262`)에서 읽었을 뿐이다 | `898a479` 사본에서 `setup.ts:8` 의 import 를 스텁으로 바꾸고 돌린 `tsc --noEmit` 출력 |
| `E2` ⑵ 가 조건 문면대로 서나(반론 6) | `E2` 판정의 몫이다. 이 판은 `E3` 만 판정한다 | `touch` 출력만으로 ⑵ 를 다시 잰 `E2` 오라클 출력 |
| D4 가 기준선에 드는 걸음인가 | `effect/06-delta.md:18`(「계획에 있었는데」)과 `:22`(「명시하지 않았으나」)의 문면이 서로 어긋난다 | 봉인 저자의 판단. 어느 쪽이든 D4 의 화면 몫 방향(안 닿았다)은 안 바뀐다 |

### 살아남은 반론의 해악도 분포 · 근거 없는 기각 · 결박

| 항목 | 값 |
|---|---|
| 금지역 | 1 |
| 실패 | 2 |
| 거짓신호 | 5 |
| 미관 | 1 |
| 근거 없는 기각 | 0 |
| 합격선을 원문에 결박 못 했나 | 아니오 |
