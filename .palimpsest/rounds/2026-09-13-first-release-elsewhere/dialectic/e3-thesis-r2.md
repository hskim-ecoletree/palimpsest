# 정(正) — `E3` 는 통과인가 반증인가 대조 불가인가 · 판 `e3-effect` r2

> 판 `e3-effect` · 라운드 **r2** · 회차 `2026-09-13-first-release-elsewhere`
> **낸 자리:** 정(正). 읽기 전용 서브에이전트다. 본문을 반환문으로만 냈다.
> **받은 것:** 설계문 `dialectic/e3-design.md` 전문 · **r1 합의 판정문 `dialectic/e3-synthesis.md`(앞선 입장)** · `intent.md` 전문 · `effect/` 전부 · `oracle/E4-breakage.txt` · `oracle/E4-breakage.mjs` · `oracle/E4-literal-touch-c.txt`(새 기계 산출) · `oracle/E1-order.txt` · `oracle/E2-scene.txt` · 복제본 `ditto-effect`(HEAD `d0eff0d`)와 `ditto-effect-at-A`(HEAD `898a479`) 읽기 · `.claude/skills/round/SKILL.md`.
> **메인의 입장:** 없다고 받았다.
> **안 받은 것 · 안 연 것:** r1 반론표 `dialectic/e3-antithesis.md` · r1 정 `dialectic/e3-thesis.md` · 종료 판단 `dialectic/e3-referee.md` · `dialectic/e3-brief-*` · `dialectic/p1-*` · 대화 기록 · 메인의 사고 과정 · `state.md` · `findings.jsonl` · `premortem/` · `conditions-audit/` · 앞 회차 `2026-09-11-effect-confirmation` 의 산출 전부 · `docs/gates/effect-confirmation.md` · `docs/plan/03-shortest-path.md` · `docs/gates/first-release-elsewhere.md` · `oracle/F4-g5.txt`.
> **스스로 밝히는 오염.**
> ⑴ 환경 머리에 이 저장소의 최근 커밋 제목 다섯이 보였다. 그중 `3a33db6` 은 *"합(合) r1 — 조건부 통과, 금지역 1 이 남는다"*, `a2e9880` 은 *"반(反) r1 — 반론 아홉, 금지역 1 · 실패 1"*, `42af275` 는 *"r2 · E4 를 조건 문면대로 다시 쟀다"* 다. `git status` 의 추적 안 된 파일 이름 둘과 자동 메모리 색인도 보였다.
> ⑵ `ls dialectic/` 로 위 「안 연 것」의 **파일 이름**을 봤다. 열지는 않았다.
> ⑶ r1 합의 판정문이 r1 반론 1~9 의 요지와 채택 · 기각 사유를 싣고 있다. **r1 반론의 내용은 그 경로로 받았다.** 원문은 안 열었다.
> ⑷ `git log --all -S` 를 돌리다가 이 회차 커밋 제목 몇(`ae6f634` · `82ede5d` 등)을 더 봤다. 열지는 않았다.
> **복제본에서 연 것:** `git log` · `git show -U0 d0eff0d` · `git show --stat 898a479`. 그리고 아래 파일들이다.
> - `5ec95eb` 의 `src/cli/commands/setup.ts:258` · `:261` · `:262`, `src/core/setup.ts:291` · `:295`, `src/core/hosts/shared.ts:3` · `:32`
> - `5ec95eb` 의 `tsconfig.json` · `package.json` · `biome.json` · `.githooks/pre-commit`, ADR 두 문서(`ADR-0003-toml-parser.md` · `ADR-20260722-claude-code-only-host.md`), `scripts/adr-guard.ts`(grep 만)
> - `d0eff0d` 의 `src/core/hosts/types.ts:146-148` · `src/core/instruction-bridge.ts:155` · `src/cli/commands/doctor.ts:69` · `bin/ditto`(grep)
> - `git grep` 으로 `parseToml` · `smol-toml` · `codexHostAdapter` · `hosts/codex` · `getHostAdapter` · `'codex'` 를 찾았다
> 인용한 `effect/…:줄` · `oracle/…:줄` · `intent.md:줄` 은 전부 `awk 'NR==N'` 으로 직접 연 줄이다.
> **옮겨 적은 자:** 메인. 정(正) r2 에이전트는 읽기 전용이라 본문을 반환문으로만 냈다 — ```markdown 블록을 스크립트로 뽑아 한 글자도 안 바꾸고 옮겼다.

## 판정

**`E3` 는 통과다. r1 합이 붙인 조건(「봉인 §5 를 기준선으로 받지 않으면 대조 불가」)은 떼고, 판정값 셋 중 하나로 닫는다.**

## 근거 — 좌표와 함께

### ⑴ `K0` 선언 — 합격선은 원문에 결박된다. 읽기는 (나) 「참인 기입」이다

| 합격선의 부분 | 결박 인용 | 무엇을 요구하나 |
|---|---|---|
| 판정 단위 · 두 끝 · 세 물음 | `intent.md:362` *"「걸음을 없앴나 · 덜 말했나 · 안 닿았나」를 차이마다 **좌표 두 끝**(화면 줄 → 변경 줄)으로 판정한다"* | 행마다 두 끝이 풀리고, 세 물음 모두에 답한다 |
| 입력이 소비된다 | `intent.md:362` *"입력에 **과제 선택자의 사전 앎**과 `E4` 의 산출을 넣는다"* · `intent.md:167-168` *"숨기면 「덜 말했나」의 판정이 앞선 앎에 기운다"* | 사전 앎과 `E4` 가 어느 행의 어느 칸에서 쓰였는지 보인다. 사전 앎의 몫을 화면의 몫으로 세지 않는다 |
| 파손 자리 대조 | `intent.md:165` *"파일 최상위 참조와 문자열 레지스트리 조회는 안 든다 (…) 실제 파손 자리와의 대조는 `E3` 가 진다"* · `intent.md:178` | 계획과 실제가 같아도, 파손 자리이면 대조 행으로 든다 |
| 「그 화면」 | `intent.md:46` `pal touch <그 심볼>` · `intent.md:137` *"㈄ 의 `touch` 는 **이 회차가 만든 바이너리**로 부른다"* | `pal query` 출력과 잠그기 전 실험의 화면은 「그 화면」이 아니다 |
| 비교의 기준선 | `intent.md:135` *"계획을 먼저 봉인 → `touch` → 읽은 줄 기록 → 변경 → 차이"* · `intent.md:176` R1 ① 처리 *"㈄ 는 새 복제본 · 사전 앎을 봉인 문서에 적는다"* | 봉인 §5 는 잠긴 방법이 정한 반사실이다 |
| 방향 값은 게이트가 아니다 | `SKILL.md:714` *"틀린 답이어도 붙인다"* | 「없앴다」가 0 이어도 판정값이 떨어지지 않는다 |

(가) 「기입만」은 `intent.md:167-168` 이 요구하는 소비를 못 잰다. (다) 「효과가 보여야 통과」는 `SKILL.md:714` 와 부딪친다. 그래서 (나) 로 읽는다.

**판정값의 뜻.**
- **통과** — 모든 행의 두 끝이 풀린다. 세 물음에 답이 선다. 사전 앎과 `E4` 가 칸에서 소비됐다. 사전 앎의 몫을 화면의 몫으로 센 칸이 0 이다.
- **반증** — 끝이 그 답을 담지 않는 행이 있다.
- **대조 불가** — 합격선은 결박됐는데 그것을 잴 좌표가 없거나 풀리지 않는다(`dialectic/e3-design.md:90`).

**화면 끝의 뜻.** 그 차이에 대해 화면이 **말한 줄 또는 말하지 않았다고 스스로 밝힌 줄**이다. 화면이 원인이 아닌 행에도 이 끝은 선다. 그 경우 방향이 「안 닿았다」다.

### ⑵ 모집단

헝크: B 는 `git show -U0 d0eff0d` 로 11 개다. A 는 `git show --stat 898a479` 로 `codex.ts` 239 줄 삭제다.

**든 것 — 차이 행 셋 · 파손 대조 행 넷**

| 행 | 종류 | 변경 끝(`-U0` 헝크 머리) |
|---|---|---|
| **D1** | 차이 — 계획보다 넓어졌다 | `src/cli/commands/setup.ts` `@@ -251,29 +249,0` · `@@ -288,5 +258,5` · `src/core/setup.ts` `@@ -291,10 +290,3` |
| **D2** | 차이 — 봉인에 없던 편집 | `src/core/hosts/shared.ts` `@@ -3 +2,0` · `@@ -32,4 +30,0` |
| **D3** | 차이 — 봉인에 없던 편집 | `.ditto/knowledge/adr/ADR-0003-toml-parser.md` `@@ -3 +3` |
| **K1** | 파손 대조 — 계획 = 실제 | `src/core/hosts/index.ts` `@@ -2 +1,0` · `@@ -5 +3,0` · `@@ -9 +7` |
| **K2** | 파손 대조 — 계획 = 실제 | `src/cli/commands/setup.ts` `@@ -8 +7,0` · `src/core/setup.ts` `@@ -7 +6,0` |
| **K3** | 파손 대조 — 자기 헝크 없음 | D1 의 `@@ -251,29 +249,0` 안에서 사라졌다 |
| **K4** ★r2 에서 더함 | 파손 대조 — 계획 = 실제 · 헝크 없음. 문자열 `'codex'` 레지스트리 조회 | 헝크 없음. 봉인 걸음 5 가 안 건드리기로 했고(`effect/00-seal.md:65`) 실제도 같다 |

**K4 를 더한 까닭.** r1 합은 이 자리를 뺀 것 X1 로 두었다. 그러나 `intent.md:165` 는 **문자열 레지스트리 조회를 이름으로 들고, 같은 문장에서** 파손 대조를 `E3` 에 맡긴다. r1 합 스스로도 계획 = 실제인 K1 · K2 를 파손 대조로 넣었다. 같은 기준을 이 자리에 대면 든다.

**뺀 것과 까닭**

| # | 무엇 | 까닭 |
|---|---|---|
| X0 | A 의 `codex.ts` 239 줄 삭제 | 계획 1(`effect/00-seal.md:61`)과 실제가 같다. 과제 그 자체다(`effect/00-seal.md:11-12`). 파손 자리가 아니라 파손의 원인 D 다 |
| X1 | 13 개 이름마다 저장소를 훑는 걸음(`effect/06-delta.md:22`) — 쟁점 `K5` | 봉인 §5(`effect/00-seal.md:61` ~ `:65`)에 없는 걸음이다. **없던 걸음을 「없앴다」로 세면 기준선을 판정 뒤에 늘리는 것**이 된다. 그리고 「필요 없다」의 근거는 질의 출력이다(`effect/04-readnote.md:15` 가 `04-callers/…` 를 댄다). 「그 화면」이 아니다 |
| X2 | 줄 번호를 「고쳐 잡은」 것(`effect/04-readnote.md:25`) | 편집 헝크에 차이가 없다. 사전 앎의 `setup.ts:261` 은 참조 줄(`5ec95eb:src/cli/commands/setup.ts:261`)이고, 질의의 `:258` 은 선언 줄(`5ec95eb:src/cli/commands/setup.ts:258`)이다. 둘 다 맞다 |
| X3 | `bin/ditto` 가 B 뒤에도 codex 어댑터와 `parseToml` 을 싣는다 | 직접 확인했다: `d0eff0d:bin/ditto` 의 `:8804` `function parseToml(text) {` · `:17915` · `:18052`. blob 은 `aded7ce` · `5ec95eb` · `d0eff0d` 에서 모두 `611640a6…` 이고, 복제본 `core.hooksPath` 는 빈 값이다. **봉인에도 편집에도 없어 차이가 아니다.** 깨지지 않고 낡을 뿐이라 파손 자리도 아니다. 실험 환경의 한계로 적는다 |
| X4 | 화면의 결정 본문 두째 문장 *"`permission-inventory.ts`의 codex 분기는 wrapper를 통해 (…) traverse"*(`effect/04-touch-after-approve.txt:10`) | `git show 5ec95eb:src/core/permission-inventory.ts \| grep -c parseToml` 이 **0** 이다. 변경은 이 문장을 따르지 않았다. 헝크 0 · 파손 자리 아님이라 행이 아니다. ⚠ **화면이 「최신 상태(fresh)」(`:6`) 아래에 대상 코드와 다른 문장을 실었다**는 관찰로 남긴다. 세 물음 밖(「더 말했다」)이다. `pal` 을 고칠지는 이 판의 몫이 아니다 |
| X5 | `effect/06-delta.md` §1 「없음」 행 · §3 | D2 · D3 · K1 · K3 · K4 로 흡수했다 |

### ⑶ 차이별 표

| 행 | 차이 | 화면 끝 | 변경 끝 | 사전 앎 좌표 | 사전 앎을 뺀 반사실 | 방향 (세 물음) | 가장 강한 대안 원인과 그 반박 |
|---|---|---|---|---|---|---|---|
| **D1** | 봉인 걸음 3 은 *"참조 제거"*(`effect/00-seal.md:63`)였다. 실제로는 `discoverCodexAgents` 를 함수째 지웠고, `discoverProjectAgents` 의 codex·both 갈래와 `writeCodexSurfaceCatalog` 를 `throw` 로 바꿨다 | `effect/03-touch/05-codexHostAdapter.txt:13` *"호출자 2 · 피호출자 1"*. 수만 싣고 자리는 안 싣는다(`oracle/E4-literal-touch-c.txt:7` *"C(호출자 파일) 없음"*). 자리는 질의 `effect/04-callers/05-codexHostAdapter.txt:4` · `:5` 가 냈고, 사슬은 A 뒤 질의 `effect/05-callers-B/discoverCodexAgents.txt:2` · `:7` 이 냈다. **원인의 종류: 그 화면 밖(`pal query`) + 대상 저장소 결정문 + `tsc`** | 위 ⑵ D1 | `effect/00-seal.md:52-53`(두 파일의 참조 줄) · `effect/00-seal.md:65`(*"시끄러운 실패 (…) ADR-20260722 결정 2"*) | **안 바뀐다.** `tsc` 가 사전 앎 없이 두 파일을 댄다(`effect/05-tsc-after-A.txt:1` · `:4`). 편집 꼴 「시끄러운 실패」는 대상 저장소 결정문에 있다(`5ec95eb:.ditto/knowledge/adr/ADR-20260722-claude-code-only-host.md:17`) | 없앴나 **아니다** · 덜 말했나 **그렇다** — 셌지만 자리를 안 실었다. 화면이 밝힌 한계(`:15` 멤버 호출 · `:16` 최상위 · 문자열)에 「자리를 안 싣는다」는 없으니 **한계 밖**이다 · 안 닿았나 **그렇다**. 세 낱말이 이름하지 않는 꼴: **편집이 넓어졌다** | 대안: 원인이 `pal` 질의이니 도구가 닿았다(`effect/06-delta.md:13`). 반박: 질의는 「그 화면」이 아니다(`intent.md:46` · `:137`; `effect/05-callers-B/discoverCodexAgents.txt:2` 머리가 `■ symbol.callers`). 이 대안을 받아도 판정값은 안 바뀐다(`SKILL.md:714`) |
| **D2** | 봉인에 없던 편집: `shared.ts` 의 `smol-toml` import(`5ec95eb:src/core/hosts/shared.ts:3`)와 `parseToml` wrapper(`:32`)를 지웠다 | `effect/03-touch/01-mcpServersFromToml.txt:9` · `:10`(ADR-0003 · 결정 · 후보 3곳) · `:11`(승인 명령) → `effect/04-approve.txt:8`(그 명령 그대로) → `effect/04-touch-after-approve.txt:5`(걸린 것 1) · `:9`(wrapper 는 `shared.ts` 에 한 함수) · `:10`(사용 지점 두 곳이 `codex.ts`) | 위 ⑵ D2 | `effect/00-seal.md:48-50`(후보가 `codex.ts` 에 있었고, 다른 복제본에서 한 번 승인해 봤다) · `intent.md:119`(그 실험의 `touch` 가 ADR-0003 「결정」 본문을 찍었다) · `intent.md:167` | **안 바뀐다. 근거 넷.**<br>⑴ **기준선에 없다.** 노출을 가진 채 `touch` 전에 잠근 계획(`effect/00-seal.md:57` · `:59` · `:61` ~ `:65`)에 이 걸음이 없다. 순서는 봉인 → 설치 → `touch`(`oracle/E1-order.txt:11` · `:12`)다. `intent.md:119` 는 봉인보다 앞선 커밋 `e6ad3f2` 에 있다(`git log -L119,119`).<br>⑵ **도구가 안 닿는다.** `shared.ts` 는 A 뒤 `tsc` 오류(`effect/05-tsc-after-A.txt:1` ~ `:4`)에 없다. 이 대조는 두 `E4` 산출의 P(`oracle/E4-breakage.txt:5` · `oracle/E4-literal-touch-c.txt:6`)도 같다. ditto pre-commit 의 lint 규칙은 `5ec95eb:biome.json:40` `noUnusedVariables` · `:41` `noUnusedImports` 다. `scripts/adr-guard.ts` 에 `toml` · `0003` · `codex` 는 grep 0 줄이다. 과제 결정문의 제거 대상(`ADR-20260722-claude-code-only-host.md:18`)에 `parseToml` · `shared.ts` 는 없다.<br>⑶ **기록에 재료가 없다.** 이 저장소에서 `git log --all -S '사용 지점은 두 곳'` 은 읽은 줄 커밋 `c43f159` 가 처음이다. `-S 'parseToml'` 도 이 회차 커밋 중 `c43f159` 가 처음이다. 봉인 §4 의 사전 앎 넷(`effect/00-seal.md:48` ~ `:54`)에도 이 걸음의 재료가 문자로 없다.<br>⑷ **노출이 계획에 들지 않았다.** 노출은 봉인보다 앞섰지만 §5 에 들어가지 않았다. 반사실이 묻는 것은 「화면 없이 변경이 무엇이었나」이고, 그 답이 잠긴 기준선이다 | 없앴나 **아니다** · 덜 말했나 **아니다** — 결정 본문이 wrapper 자리와 사용 지점을 문자로 실었다(`:9` · `:10`). 사용자는 대상 저장소에서 사용처가 `codex.ts` 뿐임을 확인했다(`effect/04-readnote.md:11`). 내 grep 도 같다: `parseToml` 코드 참조는 `codex.ts` 와 `shared.ts:32` 뿐이다 · 안 닿았나 **아니다**. → **셋 밖: 화면이 걸음을 더했다.** 과제 문면 *"그 때문에 깨지는 곳을 고친다"*(`effect/00-seal.md:11-12`) 밖의 걸음이다 | 대안 ①: 원인은 잠그기 전 노출이다. 반박: 근거 ⑶ · ⑷. 그리고 변경 전에 커밋된 기록(`oracle/E1-order.txt:13` · `:14`)이 원인 칸에 이 화면 줄을 적었다(`effect/04-readnote.md:28`). 이것은 같은 저자의 기록이다.<br>대안 ②: 봉인 §5 가 과제 문면 때문에 좁게 쓰였다. 찬반 좌표가 없다. 그렇다 해도 「화면을 본 뒤 더해졌다」는 바뀌지 않는다 |
| **D3** | 봉인에 없던 편집: ADR-0003 상태 줄(`5ec95eb` 의 `- 상태: accepted`)에 사용 지점이 사라졌음을 적었다 | `effect/04-touch-after-approve.txt:10` | 위 ⑵ D3 | D2 와 같다 | D2 와 같다. 게다가 마크다운은 `tsc` 밖이고, `adr:check` 의 범위는 *"ADR 파일명 형식·식별자 중복·인덱스→파일 정합"*(`5ec95eb:.githooks/pre-commit:16-17`)이다 | D2 와 같다. **셋 밖: 걸음을 더했다** | 대안: ADR 정리는 과제의 일부다. 반박: 과제 결정문의 제거 대상(`ADR-20260722-claude-code-only-host.md:18`)에 ADR-0003 이 없다 |
| **K1** | `hosts/index.ts` 의 import · 등록 · 재수출. 계획 2(`effect/00-seal.md:62`)와 같다 | `effect/03-touch/05-codexHostAdapter.txt:16` *"호출자 수는 하한입니다 — 파일 최상위의 참조와 문자열로 찾는 자리는 세지 않습니다"* | 위 ⑵ K1 | `effect/00-seal.md:52-53` | 안 바뀐다. `effect/05-tsc-after-A.txt:3` 이 이 파일을 댄다 | 없앴나 **아니다** · 덜 말했나 **그렇다 — 한계 안**(오류 자리가 최상위뿐: `oracle/E4-breakage.txt:9` · `:12` · `oracle/E4-literal-touch-c.txt:17`) · 안 닿았나 **그렇다**(`effect/04-readnote.md:24` *"화면이 아니라 사전 앎이 근거다"*) | 대안: 하한 문구가 이 파일을 보게 했다. 반박: 변경 전 기록 `effect/04-readnote.md:24` 가 원인을 사전 앎으로 적었다 |
| **K2** | `setup` 두 파일의 import 줄. 계획 3(`effect/00-seal.md:63`)과 같다 | `effect/03-touch/05-codexHostAdapter.txt:13`(수만) | 위 ⑵ K2 | `effect/00-seal.md:53` | 안 바뀐다. `effect/05-tsc-after-A.txt:1` · `:4` | 없앴나 **아니다** · 덜 말했나 **그렇다**. 두 파일은 `touch` 기준 C 밖이다(`oracle/E4-literal-touch-c.txt:9` · `:10`). 오류 자리인 import 줄(`:13` · `:20`)은 최상위라 `:16` **한계 안**이다. 그러나 **파일 이름을 안 실은 것은 한계 밖**이다 · 안 닿았나 **그렇다**(`effect/04-readnote.md:25` *"같다"*) | 대안: 수 2 가 사전 앎을 확인해 주었으니 걸음을 줄였다. 반박: 봉인 §5 에 「확인」 걸음이 없다(`effect/00-seal.md:61` ~ `:65`). 파일 이름을 댄 것은 질의다(`oracle/E4-breakage.mjs:5` · `:42`) |
| **K3** | 멤버 호출 `codexHostAdapter.loadSurfaceInventory(projectRoot)`(`5ec95eb:src/cli/commands/setup.ts:261`). `tsc` 가 드러낸 자리는 그 다음 줄의 매개변수 `s`(`effect/05-tsc-after-A.txt:2` `(262,50) TS7006`)다 | `effect/03-touch/09-loadSurfaceInventory-pick.txt:15` *"호출자 0"* · `:17` *"`x.foo()` 는 아직 안 셉니다"* · `effect/03-touch/05-codexHostAdapter.txt:13` | 자기 헝크 없음 — D1 의 `@@ -251,29 +249,0` 안에서 사라졌다 | `effect/00-seal.md:53`(`setup.ts:261`) | 안 바뀐다. 줄은 D1 편집으로 사라졌고, 사전 앎은 이 메서드의 호출자를 말하지 않는다 | 없앴나 **아니다** · 덜 말했나 **그렇다**. 메서드 수준의 0 은 `:17` **한계 안**이다. 담는 함수 `discoverCodexAgents`(`5ec95eb:src/cli/commands/setup.ts:258`)는 수 2 에 들었지만 자리가 안 실렸다. 조건 문면대로 잰 `E4` 는 이 자리를 **「선언 안」** 으로 셌다(`oracle/E4-literal-touch-c.txt:14` · `:22`) · 안 닿았나 **그렇다** | 대안: 따로 선 파손이 아니라 import 실패에서 번진 것이다. 받는다(재실행은 안 했다). 이 행은 기록으로만 두고, 방향 분포의 강도 근거로 세지 않는다 |
| **K4** | 문자열 `'codex'` 로 레지스트리를 찾는 자리. B 뒤 한 예: `d0eff0d:src/core/instruction-bridge.ts:155` `getHostAdapter('codex')`. 등록이 빠지면 `d0eff0d:src/core/hosts/types.ts:148` 에서 `unknown host adapter` 로 던진다 | `effect/03-touch/05-codexHostAdapter.txt:16` *"문자열로 찾는 자리는 세지 않습니다"* | 헝크 없음. 봉인 걸음 5 `effect/00-seal.md:65` 가 *"안 건드린다"* 로 정했고 실제도 같다 | `effect/00-seal.md:54` · `:65` | 안 바뀐다. 이 행의 계획 자체가 사전 앎에서 왔다. `types.ts:148` 이 봉인 `:65` 의 *"시끄러운 실패로 이미 막혀 있다"* 를 이 한 자리에서 세운다 | 없앴나 **아니다** · 덜 말했나 **그렇다 — 한계 안**(`:16`) · 안 닿았나 **그렇다** | 대안: 던지는 것은 의도된 동작이니 파손 자리가 아니다. 반박: `intent.md:165` 가 이 자리를 파손 대조의 몫으로 이름 붙였다. 방향은 이 대안을 받아도 안 바뀐다 |

**사전 앎 반사실의 요약.** 일곱 행 모두 「안 바뀐다」이고, 칸마다 좌표가 붙었다. 효과가 화면에 귀속되는 행은 D2 · D3 둘이다. 그 귀속은 봉인 §5 에 기대는데, 그 기준선은 `intent.md:135` 와 `intent.md:176` 이 잠근 방법이다. 여기에 ⑵ 도구 반사실 · ⑶ 이력 좌표가 따로 받친다.

**방향 분포.** 기록이지 게이트가 아니다.
- 「없앴다」: 0 행
- 차이 행: D1 은 안 닿았다 · 덜 말했다(한계 밖), D2 · D3 은 셋 밖(걸음을 더했다)
- 파손 대조 행 K1 ~ K4: 넷 다 안 닿았다. 덜 말했다는 한계 안이 K1 · K4 이고, 한계 밖이 섞인 것이 K2 · K3 이다

### ⑷ `E4` 산출이 어느 행에 쓰였나

**산출이 둘이다. 둘 다 입력으로 썼고, `E4` 의 판정은 내리지 않는다.**

| 산출 | C 의 출처 | 쓰인 행 | 무엇의 근거 |
|---|---|---|---|
| `oracle/E4-breakage.txt` | `pal query symbol.callers`(`oracle/E4-breakage.mjs:5` · `:42`) | K1(`:9` · `:12` · `:14`), K2(`:8`) | 질의 기준으로는 최상위 밖의 누락이 0 이다. **질의의 몫**을 가른다 |
| `oracle/E4-literal-touch-c.txt` | 조건 문면 `intent.md:363` *"`touch` 가 호출자로 낸 파일 집합"* → 빈 집합(`:7`) | D1 · K2 · K3(`:9` · `:10` · `:13` · `:14` · `:20` · `:22`), K1(`:17`) | **「그 화면」의 몫**을 가른다. 화면은 파손 파일을 하나도 이름으로 대지 않았다. 셌지만 자리를 안 실은 선언 안 오류가 1 이다(`:14`). → D1 · K2 · K3 의 「덜 말했나」가 「그렇다 — 한계 밖」이 된다 |
| 두 산출의 P(`oracle/E4-breakage.txt:5` · `oracle/E4-literal-touch-c.txt:6`) | — | D2 · D3 | `shared.ts` 도 ADR 파일도 P 에 없다. `tsc` 반사실이 두 행에 닿지 않는다 |

**r1 합과 갈리는 자리.** r1 합은 K2 를 *"수 2 = 파손 호출자 함수 2 라서 덜 말하지 않았다"* 로 적었다. 조건 문면대로 잰 산출이 생겼으므로 E3 는 「그 화면」 기준(`intent.md:46` · `:137`)을 따른다. 수는 맞았지만 자리는 안 실렸다.

### ⑸ `E3` 판정값 — **통과**

- **두 끝이 풀린다.** 일곱 행 모두다. 화면 끝은 `awk` 로, 변경 끝은 `git show -U0 d0eff0d` 로 열었다. 헝크가 없는 K3 · K4 는 헝크 대신 D1 헝크 · 봉인 걸음 줄과 복제본 줄을 댔다.
- **세 물음에 답이 선다.** 일곱 행 모두다.
- **입력이 소비됐다.** 사전 앎은 일곱 행의 반사실 칸에서 좌표와 함께 쓰였다. `E4` 는 산출 둘이 모두 ⑷ 의 칸에서 쓰였다.
- **사전 앎의 몫을 화면의 몫으로 센 칸은 0 이다.** D2 · D3 의 귀속은 앞선 화면(`intent.md:119`)을 원인으로 들지 않는다. 사전 앎의 몫은 봉인 §5 로 떼어 두었고, 귀속은 이력 · 도구 반사실 좌표에 선다.

**r1 합의 조건을 떼는 까닭.**
1. **판정값은 셋 중 하나다.** `SKILL.md:111` · `SKILL.md:347` · `dialectic/e3-design.md:90-91`. 「조건부 통과」는 그 셋 어디에도 없다.
2. **조건이 묻는 것은 이미 잠겼다.** r1 합이 소유자에게 묻겠다고 한 물음(`dialectic/e3-synthesis.md:151`)은 「노출된 사람이 쓴 봉인 §5 를 기준선으로 받나」다. 방법은 봉인이 먼저라고 `intent.md:135` 가 잠갔다. 바로 그 노출(사전부검 R1 ①)에 대한 처방도 `intent.md:176` 과 `intent.md:167-168` 이 *"봉인 문서에 적고 효과 판정의 입력에 넣는다"* 로 잠갔다. **「노출 몫을 기록으로 따로 가를 수 있어야 한다」는 원문에 없다.** 그것을 합격선에 넣으면 합격선이 원문보다 높아진다.
3. **대조 불가의 정의에 닿지 않는다.** 대조 불가는 「합격선을 잴 좌표가 없다」이다(`dialectic/e3-design.md:90`). 이 판의 좌표 — 봉인 §5 · `E1` 순서 · 화면 줄 · 헝크 — 는 전부 있고 풀린다. r1 합이 없다고 한 것(`dialectic/e3-synthesis.md:106`, 잠그기 전 실험의 원 출력)은 합격선을 재는 좌표가 아니라, 기준선이 정직했는지를 재는 좌표다.

**판정과 따로 남기는 기록.**
- 효과가 가장 큰 두 행(D2 · D3)은 세 낱말 밖이다. 세 물음의 틀에는 「화면이 걸음을 더했다」가 없다.
- `effect/06-delta.md:35` 는 *"차이 넷"* 이라 적는다. 이 판정의 차이 행은 셋이고, 파손 대조 행이 넷이다.
- `effect/04-readnote.md:12` · `:13` 과 `effect/06-delta.md:12` · `:13` 은 화면 줄을 한 줄 어긋나게 인용했다: `:12` → 실제 `:13`, `:15` → 실제 `:16`. 이 판정의 행은 바로잡은 줄을 쓴다.
- X3(`bin/ditto` 가 낡았다) · X4(화면이 fresh 아래 코드와 다른 문장을 실었다).

## 대안과 그것이 빠지는 근거

| 대안 | 무엇인가 | 빠지는 근거 |
|---|---|---|
| **지금 그대로 — r1 합의 「조건부 통과」 유지** | D2 · D3 을 봉인 §5 수용 여부에 걸고 소유자에게 올린다 | 판정값 셋 밖이다(`SKILL.md:111`). 조건의 물음은 `intent.md:135` · `:176` 이 이미 답했다. 그리고 K4 누락과 K2 · D1 의 「덜 말했나」가 새 `E4` 산출(`oracle/E4-literal-touch-c.txt:7` · `:14`)과 맞지 않는다 |
| **대조 불가** | 잠그기 전 노출의 몫이 기록으로 안 갈리니 D2 · D3 을 못 판다 | 합격선(⑴)에 「노출 몫의 기록 분리」가 없다. `intent.md:167-168` 의 요구는 공개와 입력이고, 둘 다 됐다(`effect/00-seal.md:48-50` · `intent.md:119`). 반사실에는 좌표 넷(D2 칸)이 있다 |
| **반증** | 끝이 답을 담지 않는 행이 있다 | 일곱 행의 끝을 직접 열었다. 한 줄 어긋난 인용은 기록 문서(`effect/04-readnote.md` · `effect/06-delta.md`)의 것이고, 이 판정의 끝이 아니다 |
| **승격(`K0` 결박 불가)** | 원문에 합격선이 없다 | `intent.md:362` · `:165` · `:167-168` · `:46` · `:137` 과 `SKILL.md:714` 로 결박된다(⑴) |
| **통과 — r1 합의 행 여섯 그대로** | K4 를 빼고, K2 · D1 의 「덜 말했나」를 「아니다」로 둔다 | K4 는 `intent.md:165` 가 이름으로 든 자리다. 빼는 까닭이 서지 않는다. 「덜 말했나」는 조건 문면대로 잰 `E4` 산출과 어긋난다 |

## 이 초안이 틀렸다면 무엇이 먼저 드러나나

1. **봉인 커밋 `fc974c8` 앞에 이 걸음의 재료가 문자로 있으면.** 잠그기 전 실험의 원 출력이나 그 세션의 메모에 `parseToml` 제거나 ADR-0003 정리가 **계획으로** 적혀 있으면 D2 · D3 의 귀속이 무너진다. 사전 앎의 몫을 화면에 센 것이 된다(금지역). 이 저장소 안의 `git log --all -S` 는 0 이었다. 찾을 곳은 저장소 밖(세션 스크래치 · 대화 기록)이다.
2. **ditto 자신의 검사가 A 뒤 `shared.ts` 를 잡으면.** `898a479` 에서 `bun run lint` 나 다른 검사가 쓰이지 않는 `parseToml` 이나 `smol-toml` import 를 빨갛게 내면, D2 반사실 ⑵ 가 무너진다. 나는 biome 를 돌리지 않고 규칙 이름만 읽었다.
3. **「그 화면」에 `pal query` 를 넣어 읽는 원문 좌표가 있으면.** D1 · K2 의 「안 닿았나」와 「덜 말했나」가 뒤집힌다. 판정값은 `SKILL.md:714` 로 그대로다.
4. **`listHostAdapters()` 를 쓰는 자리가 codex 가 목록에 있음을 조용히 가정하면.** 예: `d0eff0d:src/cli/commands/doctor.ts:69`. 문자열 조회처럼 던지지 않고 조용히 줄어드는 파손 자리가 모집단에 더 있다는 뜻이고, K4 가 그것을 못 덮는다.
5. **`dialectic/e3-design.md:90` 의 「대조 불가」를 「행의 원인 귀속을 가를 좌표가 없다」까지 넓혀 읽으면.** 판정은 대조 불가로 간다. 이 초안은 그 정의를 문면대로 좁게 읽었다.
6. **봉인 §5 가 과제 문면(`effect/00-seal.md:11-12` *"깨지는 곳"*) 때문에 의도적으로 좁았다는 좌표가 나오면.** 판정값은 안 바뀐다. D2 · D3 의 효과가 「놓친 걸음을 잡았다」가 아니라 「과제 밖 걸음을 더하게 했다」로 성격이 바뀐다.

## 내가 확인 못 한 것

| 무엇 | 왜 못 했나 |
|---|---|
| A 뒤 biome · `adr:guard` · `adr:check` 의 실제 출력 | 복제본에서 실행하지 않았다. 규칙 이름(`5ec95eb:biome.json:40-41`)과 grep 만 봤다. export 함수가 `noUnusedVariables` 에 안 걸린다는 것은 그 도구에 대한 내 앎이고, 관측이 아니다 |
| 잠그기 전 실험(`intent.md:119`)의 화면이 이번 `effect/04-touch-after-approve.txt:9-10` 과 같은 글자였나 | 원 출력이 `baseline/` 에 없다(목록: `01-install.txt` · `02-ledger.txt` · `03-narrative.txt` · `04-touch-*.txt` 셋 · `runner-pre.md`). grep 0 줄. 바이너리도 다르다(`6ee9eb3` 대 `db6c366`) |
| `TS7006`(`262,50`)이 import 실패에서 번진 것인가 | `tsc` 를 다시 돌리지 않았다 |
| 문자열 `'codex'` 레지스트리 조회 94 줄(`effect/00-seal.md:54`)의 전수, 그리고 전부가 `getHostAdapter` 를 거쳐 던지는가 | 한 자리(`d0eff0d:src/core/instruction-bridge.ts:155`)만 확인했다. 94 는 다시 세지 않았다 |
| `listHostAdapters()` 호출자들이 목록에 codex 가 있다고 가정하나 | 호출 자리를 grep 으로만 봤다(`doctor.ts:69` 외). 본문은 안 읽었다 |
| 「최신 상태(fresh)」(`effect/04-touch-after-approve.txt:6`)가 무엇에 대한 판정인가, 그리고 X4 와의 관계 | `pal` 코드를 읽지 않았다 |
| `git log -S 'parseToml'` 에 걸린 옛 커밋 `cae1742`(2026-08-13)의 내용 | 열지 않았다. 이 회차 커밋 중 첫 등장만 근거로 썼다 |
| 봉인 §5 를 쓴 자가 과제 문면 때문에 이 걸음을 뺐는지, 몰라서 뺐는지 | 의도는 기록으로 못 잰다 |
