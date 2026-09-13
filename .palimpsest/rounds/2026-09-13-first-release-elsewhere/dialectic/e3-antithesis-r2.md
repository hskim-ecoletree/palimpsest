# 반(反) — `E3` r2 초안에 대한 반론표 · 판 `e3-effect`

> 판 `e3-effect` · 라운드 **r2** · 회차 `2026-09-13-first-release-elsewhere`
> **낸 자리:** 반(反). 읽기 전용 서브에이전트다. 저장소 파일과 복제본 파일은 하나도 안 바꿨다. 본문은 반환문으로만 냈다.
> **받은 것:** 초안 `dialectic/e3-thesis-r2.md` · 발췌 `dialectic/e3-brief-opponent.md`(M1~M15 · 해악도 표) · `effect/00-seal.md` · `oracle/E4-breakage.txt` · `oracle/E4-literal-touch-c.txt` · 근거 원문 열람(`effect/` · `intent.md` · `oracle/E1-order.txt` · `oracle/E2-scene.txt` · `oracle/E2-scene.py` · `oracle/E4-breakage.mjs` · `dialectic/e3-synthesis.md` · 복제본 `ditto-effect` · `ditto-effect-at-A` · `.claude/skills/round/SKILL.md` 의 인용 줄과 그 앞뒤).
> **안 받은 것 · 안 연 것:** `dialectic/e3-design.md`(초안이 인용한 `:90-91` 도 안 열었다) · r1 반론표 `dialectic/e3-antithesis.md` · r1 정 `dialectic/e3-thesis.md` · `dialectic/e3-referee.md` · `dialectic/p1-*` · `dialectic/r1-raw.md` · `state.md` · `findings.jsonl` · `premortem/` · `conditions-audit/` · 앞 회차 `2026-09-11-effect-confirmation` 의 산출 · `docs/gates/effect-confirmation.md` · `docs/plan/03-shortest-path.md:140-146` · 대화 기록.
> **메인의 사고 과정:** 프롬프트에 섞여 오지 않았다. 과제 지시만 받았다.
> **스스로 밝히는 오염.**
> ⑴ 환경 머리에 최근 커밋 제목 다섯이 보였다. `3a33db6` *"합(合) r1 — 조건부 통과, 금지역 1 이 남는다"* · `42af275` *"r2 · E4 를 조건 문면대로 다시 쟀다"* 등이다. `git status` 의 추적 안 된 파일 이름과 자동 메모리 색인도 보였다.
> ⑵ `ls dialectic/` 로 위 「안 연 것」의 **파일 이름**을 봤다. 열지는 않았다.
> ⑶ `git log --all -S` 출력에서 이 회차 커밋 제목을 더 봤다. `a2e9880` *"반(反) r1 — 반론 아홉, 금지역 1 · 실패 1"* · `82ede5d` · `8852754` · `ae6f634` 다. 열지는 않았다.
> ⑷ r1 반론의 요지는 r1 합 `dialectic/e3-synthesis.md` 의 채택 · 기각 표로 받았다.
> **복제본에서 돌린 것(전부 읽기):** `git log` · `git show` · `git grep`. 그리고 `ditto-effect-at-A` 에서 `…/ditto-effect/node_modules/.bin/biome lint src/core/hosts/shared.ts` 를 돌렸다(`--write` 없음). 스크래치패드 뿌리에 `codex94.txt` 한 파일을 썼다. 복제본 밖이다.
> 인용한 `effect/…:줄` · `intent.md:줄` · `dialectic/e3-thesis-r2.md:줄` 은 `awk 'NR==N'` 이나 `cat -n` 으로 직접 연 줄이다.
> **옮겨 적은 자:** 메인. 반(反) r2 에이전트는 읽기 전용이라 본문을 반환문으로만 냈다 — ```markdown 블록을 스크립트로 뽑아 한 글자도 안 바꾸고 옮겼다.

## 초안이 서는 자리

- **두 끝이 풀린다.** 초안이 D1 · D2 · K1 ~ K4 에 댄 화면 줄은 원문과 같다. `effect/03-touch/05-codexHostAdapter.txt:13` *"호출자 2 · 피호출자 1"* · `:16` 하한 문구 · `effect/04-touch-after-approve.txt:9` · `:10` · `effect/03-touch/09-loadSurfaceInventory-pick.txt:15` · `:17` 을 열어 봤다. `oracle/E4-breakage.txt:5` · `:8` · `:9` · `:12` · `:14` 와 `oracle/E4-literal-touch-c.txt:6` · `:7` · `:9` · `:10` · `:13` · `:14` · `:17` · `:20` · `:22` 의 인용 줄 번호도 맞다.
- **D2 반사실 ⑵(도구가 안 닿는다) 가 선다.** `ditto-effect-at-A`(HEAD `898a479`)에서 `biome lint src/core/hosts/shared.ts` 를 돌렸다. 출력은 *"Checked 1 file in 16ms. No fixes applied."* 이고 진단은 0 줄이다. 찍힌 `rc` 는 파이프 끝 `tail` 의 것이라 판정에 쓰지 않았다. 초안이 「관측이 아니다」(`dialectic/e3-thesis-r2.md:148`)라고 남긴 자리가 관측으로 선다.
- **B 뒤 테스트 파손을 가리키는 좌표가 없다.** `git grep -n -E "getHostAdapter\(['\"]codex|writeCodexSurfaceCatalog|discoverProjectAgents|installCodexSurface|discoverCodexAgents|loadSource\(|listHostAdapters" d0eff0d -- '*.test.ts'` 가 0 줄이다.
- **시간 순서가 선다.** `intent.md:119` 의 첫 커밋은 `e6ad3f2`(2026-09-13 19:58)다. 봉인 `fc974c8`(22:48)보다 앞선다. 이 저장소에서 `git log --all -S '사용 지점은 두 곳'` 의 가장 이른 커밋은 `c43f159` 다.
- **K4 · X2 · X4 의 복제본 좌표가 맞다.** `d0eff0d:src/core/hosts/types.ts:148` 이 `unknown host adapter` 로 던진다. `d0eff0d:src/core/instruction-bridge.ts:155` 는 `getHostAdapter('codex')` 다. `5ec95eb:src/cli/commands/setup.ts:258` 은 선언 줄이고 `:261` 은 참조 줄이다. `5ec95eb:src/core/permission-inventory.ts` 에는 `parseToml` 이 없다.
- **K4 를 모집단에 넣은 근거가 선다.** `intent.md:165` 가 *"문자열 레지스트리 조회"* 를 이름으로 들고, 같은 문장에서 파손 대조를 `E3` 에 맡긴다.

## 반론

| # | 반론 | 좌표 | 유효성 | 해악도 |
|---|---|---|---|---|
| 1 | D2 반사실 ⑶ 「기록에 재료가 없다」는 **palimpsest 저장소만** 뒤졌다. 그런데 새로 넣은 걸음(`parseToml` 제거 · ADR-0003 정리)의 재료는 봉인 전부터 **대상 저장소에 글자 그대로** 있었다. 그 본문은 잠그기 전 실험이 찍은 바로 그 본문이다. `-S` 로 palimpsest 저장소를 뒤져서는 원리상 안 걸린다. r1 합이 지운 ⑶ 을 대신한 근거가 증거 구실을 못 한다. 이것을 빼면 D2 · D3 의 화면 귀속은 ⑴ · ⑷ 에만 기댄다. 둘 다 봉인 §5 를 쓴 사람의 기록이고, r1 합이 조건을 붙인 바로 그 자리다 | `dialectic/e3-thesis-r2.md:81`(⑶ *"`git log --all -S '사용 지점은 두 곳'` 은 읽은 줄 커밋 `c43f159` 가 처음이다"*) · `:112` · `git -C ditto-effect show aded7ce:.ditto/knowledge/adr/ADR-0003-toml-parser.md`(「결정」 절에 *"`parseToml(text)` wrapper 한 함수만 유지"* · *"사용 지점은 두 곳: `src/core/hosts/codex.ts`의 `loadPermissions`와 `mcpServersFromToml`"*) · `git grep -n 'hosts/codex' 5ec95eb` → `5ec95eb:.ditto/knowledge/adr/ADR-0003-toml-parser.md:26` · `5ec95eb:.ditto/memory/events/memevt_db96d410b501.json:9` · `intent.md:119`(그 실험의 `touch` 가 이 「결정」 본문을 찍었다) | 참 | 금지역 |
| 2 | 봉인 §5 는 자기 입력을 스스로 **§4 로 한정**했다(*"위 사전 앎만으로"*). 그런데 §4 ① 에는 「후보가 있다 · 한 번 승인해 봤다」만 있다. 「결정 본문(사용 지점 두 곳이 `codex.ts`)을 봤다」는 없다. 따라서 §5 는 선택자가 실제로 가진 사전 앎보다 **좁은 집합**에 대한 반사실이다. `intent.md:119` 의 노출을 가르는 기준선이 못 된다. 초안이 정한 통과의 넷째 칸 「사전 앎의 몫을 화면의 몫으로 센 칸 0」을 D2 · D3 에서 잴 좌표가 없다. 그러면 초안 자신의 대조 불가 정의에 닿는다. `intent.md:362` 가 입력으로 이름한 것은 「과제 선택자의 사전 앎」이다. 「봉인 §4」가 아니다. 그래서 초안의 조건 떼기 사유 2(`:116` *"처방이 잠겼다"* · `:130` *"둘 다 됐다"*)는 처방이 **덜 이행된** 자리에 선다. `intent.md:167` 은 그 사실을 *"봉인 문서 머리에 적고"* 라 요구했다 | `effect/00-seal.md:59` · `effect/00-seal.md:48-50` · `intent.md:119` · `intent.md:167` · `intent.md:362` · `dialectic/e3-thesis-r2.md:41` · `:43` · `:81`(⑷) · `:116` · `:117` · `:130` | 참 | 실패 |
| 3 | X1(13 개 이름을 저장소에서 훑는 걸음)을 「봉인 §5 에 없다」는 까닭으로 뺐다. 그러나 그 걸음은 봉인 저자의 차이 문서가 스스로 *"`tsc` 전에 할 법한 걸음"* 이라 적었다. 그리고 그 걸음은 **D2 · D3 의 재료에 곧장 닿는다.** `mcpServersFromToml` 을 `git grep` 하면 bin · 측정 번들 밖에서 네 곳이 걸리고, 첫 줄이 ADR-0003 결정 본문이다. 화면 없이도 평범한 grep 한 번이 같은 문장에 닿는다. 그런데 초안의 도구 반사실 ⑵ 는 `tsc` · biome · `adr-guard` 만 봤다. X1 을 모집단에서 뺀 것이 D2 의 「안 닿았나 — 아니다」를 떠받친다. (grep 을 실제로 했을지는 기록으로 못 잰다. 그러나 「할 법하다」는 기록이 저자 자신의 것이다) | `effect/06-delta.md:22` · `dialectic/e3-thesis-r2.md:70`(X1) · `:81`(⑵) · `git grep -n mcpServersFromToml 5ec95eb` → `.ditto/knowledge/adr/ADR-0003-toml-parser.md:26` · `.ditto/memory/events/memevt_db96d410b501.json:9` · `src/core/hosts/codex.ts:25` · `:166` · `git grep -n loadPermissions 5ec95eb` 도 같은 `:26` 이 걸린다 | 참 | 실패 |
| 4 | 초안은 r1 합이 **소유자에게 올린 물음**을 정(正)의 추론으로 닫았다. 규약은 *"살아남은 반론이 금지역·실패에 닿으면 → 소유자에게 올린다"* 이다. r1 합은 살아남은 금지역을 1 로 세고 올렸다. 초안이 조건을 떼는 사유 1(`SKILL.md:111` — 값은 셋)은 판정값이 몇 개인지를 말할 뿐이다. 올림 규칙 `:447` 에 답하지 않는다. 올림은 넷째 판정값이 아니라 규약의 절차다. r1 합이 올린 까닭(*"자기 기준선을 자기가 판정하는 셈"*)도 초안에서 반박되지 않았다 | `.claude/skills/round/SKILL.md:447` · `dialectic/e3-synthesis.md:149` · `:151` · `:165`(금지역 1) · `dialectic/e3-thesis-r2.md:23` · `:115` | 참 | 실패 |
| 5 | D2 의 원인 줄 `effect/04-touch-after-approve.txt:10` 은 **같은 줄 안에서** 셋째 사용처가 있다고 말한다(*"`permission-inventory.ts`의 codex 분기는 wrapper를 통해 … traverse"*). `5ec95eb` 에서는 거짓이다. 그래서 화면을 말한 그대로 받으면 「wrapper 를 지워도 된다」가 아니라 「사용처가 더 있다」가 된다. 걸음을 결정한 것은 읽은 줄 기록 `:11` 의 **대상 저장소 grep** 이다. 그 grep 이 화면을 뒤집었다. 초안은 이 줄의 첫 문장을 D2 의 원인으로 쓴다(`:81` 덜 말했나 **아니다**). 둘째 문장은 X4 로 「행이 아니다」라며 뺀다(`:73`). 한 줄을 갈라 쓴 것이다. 짐을 지는 행에서 「틀리게 말했다」가 방향 칸에 없어 화면 몫이 부푼다 | `effect/04-touch-after-approve.txt:10` · `effect/04-readnote.md:11` · `git grep -n -i 'codex\|toml' 5ec95eb -- src/core/permission-inventory.ts`(`:48` ~ `:85` 이 `inv.raw` 만 읽는다. TOML import 없음) · `git grep -n 'parseToml\|smol-toml' 5ec95eb -- src tests scripts`(`codex.ts` · `shared.ts` 뿐) · `dialectic/e3-thesis-r2.md:73` · `:81` | 참 | 거짓신호 |
| 6 | 초안은 「그 화면 = `touch` 뿐, 질의는 아니다」를 `E4` 에는 댄다(C = ∅). 그런데 받아들인 기계 사실 M2(`E2` 통과)의 ⑵ 는 **질의 출력을 `touch` 출력으로 세서** 선 것이다. 같은 입력을 두 기준으로 읽었다. 초안의 좁은 읽기가 옳으면 `E2` ⑵ 가 조건 문면(*"그 출력들에서 … `호출자` 에 다른 파일의 참조"*)대로는 안 선다. 넓은 읽기가 옳으면 초안의 반증 조건 3(`:139`)이 발동해 D1 · K2 의 방향이 뒤집힌다. 초안은 그 조건을 가정으로만 적고 좌표를 못 찾았다 | `oracle/E2-scene.py:55`(`for f in sorted((fx / "04-callers").glob("*.txt"))`) · `oracle/E2-scene.txt`(⑵ 2 건이 `04-callers` 의 줄이다) · `intent.md:361` · `dialectic/e3-thesis-r2.md:34` · `:102` · `:139` | 참 | 거짓신호 |
| 7 | ⑷ 는 `oracle/E4-literal-touch-c.txt:14` 의 「선언 안 1」로 D1 · K2 · K3 의 「덜 말했나」를 「한계 밖」으로 옮긴다. 그런데 K3 행은 그 `TS7006` 이 import 실패에서 **번진 것**이라는 대안을 **받았다.** 받았다면 `:14` 는 `pal` 이 빠뜨린 호출자 자리가 아니다. C = ∅ 과 번짐이 겹쳐 생긴 산물이다. 그 줄을 담는 `discoverCodexAgents` 는 이미 셌던 호출자 2 에 들어 있다 | `dialectic/e3-thesis-r2.md:85`(*"받는다"*) · `:102` · `oracle/E4-literal-touch-c.txt:14` · `effect/04-callers/05-codexHostAdapter.txt:4` · `effect/05-tsc-after-A.txt:2` | 참 | 거짓신호 |
| 8 | K4 는 사전 앎 좌표로 봉인 §4 ③ 의 *"레지스트리를 문자열 `'codex'` 로 찾는 자리가 … 94 줄"* 을 물려받는다. 실측하면 94 줄 가운데 `getHostAdapter('codex')` 는 **1 줄**이다. 나머지는 비교 `=== 'codex'` 32 · `host: 'codex'` 24 · 타입 합집합 24 등이다. 반대로 변수로 레지스트리를 찾는 자리는 K4 에 없다. `run-with.ts:113` 이 `'codex'` 를 받고 `:169` 에서 `getHostAdapter(provider)` 를 부른다. `instruction-bridge.ts:346` 도 그렇다. 둘 다 결국 `types.ts:148` 에서 던지므로 「시끄러운 실패」는 선다. 그러나 모집단의 이름과 수가 사실과 다르다. 초안은 이 셈을 「확인 못 한 것」(`:151`)에 두었다 | `effect/00-seal.md:54` · `effect/00-seal.md:65` · `git grep -h "'codex'" 5ec95eb -- src tests`(94 줄 · `getHostAdapter('codex')` 1) · `d0eff0d:src/core/run-with.ts:113` · `:169` · `d0eff0d:src/core/instruction-bridge.ts:346` · `dialectic/e3-thesis-r2.md:86` · `:151` | 참 | 거짓신호 |
| 9 | 초안은 (다) 「효과가 보여야 통과」를 `SKILL.md:714` 한 줄로 기각한다. 그 줄은 `## 8. 효과` 절 안에 있고, 게이트 문서에 **산출을 붙이는 일**을 말한다. ⟨정반합⟩ 조건이 방향과 무관하게 통과한다는 말은 없다. 한편 초안 자신이 「없앴다 0」, 가장 큰 효과 행 둘은 **세 낱말 밖**이며 **과제 문면 밖의 걸음**이라고 적는다. 이 상태에서 게이트에 「`E3` 통과」만 남으면 읽는 사람은 「효과가 확인됐다」로 받는다. 규약은 조건이 의도를 못 재고 있으면 정정하라고 한다 | `.claude/skills/round/SKILL.md:710` · `:712` · `:714` · `intent.md:135`(㈄ 「효과 확인」) · `dialectic/e3-thesis-r2.md:36` · `:38` · `:81`(*"과제 문면 … 밖의 걸음이다"*) · `:91` · `:120` | 추정 | 거짓신호 |
| 10 | 초안이 줄 어긋난 인용을 모은 목록에 `effect/06-delta.md:28` 이 빠졌다. 그 줄도 하한 문구를 `05-codexHostAdapter.txt:15` 로 인용한다. 실제는 `:16` 이다 | `effect/06-delta.md:28` · `effect/03-touch/05-codexHostAdapter.txt:16` · `dialectic/e3-thesis-r2.md:122` | 참 | 미관 |

## 초안이 버린 갈래를 받치는 가장 강한 근거

| 갈래 | 가장 강한 근거 | 좌표 | 강도(내 매김) |
|---|---|---|---|
| **r1 합의 조건부 통과(소유자에게 올림)** | 규약이 살아남은 금지역을 소유자에게 올리라고 한다. r1 합이 그 금지역을 1 로 세고 올렸다. 초안은 올림 규칙에 답하지 않았다(반론 4). 게다가 기준선을 쓴 자가 그 기준선의 정직성을 스스로 판정하는 꼴이 그대로다 | `.claude/skills/round/SKILL.md:447` · `dialectic/e3-synthesis.md:149` · `:165` | 강하다 |
| **대조 불가** | 봉인 §5 는 입력을 §4 로 한정했다. §4 는 결정 본문 노출을 적지 않았다. 그 본문은 봉인 전 대상 저장소에 글자 그대로 있었고, 잠그기 전 실험이 찍었다. 그래서 D2 · D3 에서 「사전 앎의 몫 0」을 잴 좌표가 없다(반론 1 · 2 · 3) | `effect/00-seal.md:59` · `effect/00-seal.md:48-50` · `intent.md:119` · `5ec95eb:.ditto/knowledge/adr/ADR-0003-toml-parser.md:26` · `effect/06-delta.md:22` | 강하다. 초안 자신의 정의(`dialectic/e3-thesis-r2.md:43`)로도 닿는다 |
| **반증** | D2 의 화면 끝 `:10` 은 「wrapper 에 다른 사용처가 없다」를 담지 않는다. 오히려 셋째 사용처가 있다고 말한다. 제거는 대상 저장소 grep 이 정했다(반론 5) | `effect/04-touch-after-approve.txt:10` · `effect/04-readnote.md:11` · `5ec95eb:src/core/permission-inventory.ts:48` | 약하다. `:10` 은 「사용 지점 두 곳」이라는 글자도 싣고 있어서, 초안의 반증 정의(끝이 답을 담지 않는다)를 문면대로는 못 채운다 |
| **승격(`K0` 결박 불가 · 정정)** | 합격선인 세 물음이 이 판에서 가장 큰 효과(「화면이 과제 밖 걸음을 더했다」)를 이름하지 못한다. 초안이 스스로 그렇게 기록했다. 조건이 의도(㈄ 효과 확인)를 못 재고 있으면 규약상 정정 대상이다 | `dialectic/e3-thesis-r2.md:120` · `intent.md:362` · `intent.md:135` · `.claude/skills/round/SKILL.md:710` | 중간. 세 낱말은 원문에 결박돼 있다. 「못 잰다」는 해석이다 |

## 내가 스스로 물린 것

| 산출했던 반론 | 물린 까닭 |
|---|---|
| `d0eff0d:src/cli/commands/doctor.ts:69` 의 `listHostAdapters()` 가 codex 를 조용히 목록에서 빼니, 조용한 파손 자리가 모집단에 없다(초안 반증 조건 4) | 좌표는 있다(`:69`, `:75` 의 `id === 'codex'` 걸러내기). 그러나 codex 를 목록에서 빼는 것은 과제의 결정(`ADR-20260722-claude-code-only-host`)이 뜻한 동작이다. 의도 밖의 **파손**이라는 좌표를 못 댔다 |
| 테스트가 B 뒤 런타임에 깨지는데 모집단이 `tsc` 만 본다 | `*.test.ts` 에서 codex 설정 경로 · 레지스트리를 찾는 `git grep` 이 0 줄이다. 파손 좌표가 없다. 테스트는 안 돌렸다 |
| biome 가 A 뒤 쓰이지 않는 `parseToml` export 를 잡으니 D2 반사실 ⑵ 가 무너진다 | 직접 돌렸더니 진단이 0 줄이다. 오히려 초안이 서는 자리로 옮겼다 |
| 「`-S 'parseToml'` 이 이 회차에서 `c43f159` 가 처음」은 거짓이다 — `cae1742`(2026-08-13)에 있다 | `cae1742` 에 걸린 줄은 골든 산출의 심볼 목록 한 줄(`src/core/hosts/shared.ts parseToml function …`)이다. 회차 밖이고 계획 재료도 아니다. 초안은 「이 회차 커밋 중」으로 한정해 적었다 |
| 초안이 `SKILL.md:347` 을 「판정값 셋」의 근거로 댔는데, 그 줄은 넷(미측정 포함)을 싣는다 | 문면상 어긋남은 있다. 그러나 미측정은 상자가 안 켜진 상태라 판정값 셋이라는 요지를 바꾸지 않는다. 미관에도 못 미친다 |
| 초안이 읽은 대조 불가 정의 `dialectic/e3-design.md:90` 가 너무 좁다 | 그 파일은 받지 않았고 열지 않았다. 원문을 대지 못한다 |
| 봉인 전에 대상 저장소를 grep 하는 사람이면 누구든 ADR-0003 에 닿는다 — 일반 반사실 | 봉인 저자가 그 걸음을 할 법하다고 **기록한 좌표**(`effect/06-delta.md:22`)가 있는 범위만 반론 3 에 남겼다. 그 밖의 「누구든 했을 것」은 좌표가 없다 |
