# preflight — 판 4 **라운드 3** ⟨설계문 §기계가 먼저 돈다⟩

> 정(正)과 반(反)에게 **같은 바이트**로 준다. 셀 수 있는 것을 놓고 다투면 라운드가 탄다.

★★ **이 파일이 `dialectic/preflight.md` 를 안 덮는다.** 앞 핀은 그대로 서 있고 여기가
새 핀이다 — 소유자 승격 **칸 9** 가 *"라운드마다 핀을 다시 박는 것"* 을 교훈으로 적었고
**칸 12 가 그것을 처음 집행한다.** 무엇이 어떻게 갈렸는지가 증거이므로 앞 것을 안 지운다.

## 왜 라운드 3 인가 — 라운드 2 는 **무효다**

설계 반증 **조항 8**(`dialectic/4-design.md:242-243`):

> **판정문이 기준 핀 밖의 바이트에 기댔다**(증거 `sha256` 불일치, 또는 판정 중 생긴 파일
> 인용) → 움직이는 표적을 쐈다. **그 판은 무효이고 다시 돌린다.**

그것이 판 4 라운드 2 에 대해 **문자 그대로** 발화했다 — `dialectic/preflight.md:60` 이
`docs/gates/effect-confirmation.md` 를 **이름으로** 핀에 싣는데 판정 시점 값이 달랐다.
**소유자가 2026-09-12 에 면제가 아니라 집행을 골랐다** ⟨`intent.md` `## 승격` 칸 12⟩.

⚠ **그러므로 `dialectic/4-{thesis,antithesis,synthesis}-r2.md` 는 이 라운드의 입력이 아니다.**
바이트로 보존되지만 **판정으로 안 쓰인다.** 규약 §5 가 정(正)에게 *"앞 라운드의 반론"* 을
안 주는 것과 같은 자다.

## 공통 — 기준 핀

```
$ git rev-parse HEAD
8457ac14fb1f56718a1c1702502fa6a216daa3f9

$ git status --porcelain
?? .palimpsest/rounds/2026-09-11-effect-confirmation/dialectic/preflight-r3.md
(위가 비면 워킹트리가 깨끗하다)
```

**기준 커밋은 위 SHA 다. 그 뒤에 생긴 바이트는 증거가 아니다.**

⚠ **이 파일 자신은 그 커밋에 없다 — 원리상 없을 수밖에 없다.** 핀을 자기를 담은 커밋으로
적으려면 커밋이 자기 SHA 를 자기 안에 담아야 한다. 한 번 시도해서 SHA 가 두 번 움직이는
것을 보고 되돌렸다. **핀은 마흔 항을 잰 그 나무의 SHA 이고, 그때 워킹트리에서 추적 안 되던
것은 이 파일 하나다.** 앞 핀(`dialectic/preflight.md`)도 같은 모양이었다.

⚠ **이 파일 자신은 핀 목록에 없다** — 자기 해시를 자기 안에 적을 수 없다. 앞 핀은
`dialectic/preflight.md` 를 자기 목록에 넣어 **구조상 만족 불가능한 항**을 하나 갖고
있었고, 그것이 칸 9 의 「다섯」을 헤아릴 때 셈을 흐렸다. 여기서 뺀다.

### 증거 파일의 sha256 — **마흔**

```
6177cf89d5978e5fef103c6e085825fd8b2194fa907a784ab5e49b3d156d54c7  intent.md
f5ce9bff89c3ffbcfdcc05b5c9e6e2535301b3faa2104e68bd27c438274da85c  findings.jsonl
628d11a82d3dcbb2141e7bfe101f875c589afd43c6ce31b99b49b1090dd645e9  plan/126-pre.md
cc1d0abf9e1e33da42b5a8d628633b94e947536009f7c4aa5108830c71acf3d2  plan/129-pre.md
f62783a58b19b55dddb28f34abbd3c7d0af0d68598160c9cbfd87f028ed9a0e4  plan/79-pre.md
08f9b674ab7ca8013440a80dbe21a04a6d3315c159b27607be14b6da333be917  touch/126.txt
116c29766b6b43698da01707dc9d52628b561b23992453528530d6810485bdaa  touch/129.txt
23063919c4aad9b0c1383167dec217afc8805f878056178b9704ae4781cf47e3  touch/79.txt
23bf96bea7cd7d2772a4ef06aad39e1ac3e1865ed0520bbbeebf74015217e59f  effect/126-delta.md
e9a673164f58b339201454a4fd916704c3a373ed950c48e5674969c2170d16dc  effect/126-judge-1.md
70d300b2e680c3b38f41874da5db674a5f297174098175888ac39649a5cb9ad8  effect/126-judge-2.md
7b4cf9b7e9a0af69f137648284eccc94756cf510e52e491cb1e36ada0980f518  effect/126-readnote.md
5254707a06f4c9dc7859e0020657601559af0d75ba69ebb34dbbe612e2dbb93f  effect/129-delta.md
a3d5e7a99b70022f50a9062cd93ecd689e55368d82ee8f4bef24766bf2ba3025  effect/129-judge-1.md
357f2c18cf13608014204b00979f15dcdc5367187d699a1991abbedcea815bc5  effect/129-judge-2.md
8c3b3d9ca21e4f3ee8d0038131a7dc42b6fce89a3bd594b26a282f02457bfd0d  effect/129-readnote.md
a8616bbf35196cecce42542af8e35a7c11bdcf0f9573ef9324e4ee090b04d194  effect/79-delta.md
5e36a9756beb7a4ef9c90c2217120c963b2a7df6355ba4e780eac71468804060  effect/79-judge-1.md
f9b3f3d4e42445df763d8d19fbbc9be94702e9b3955011c13798ec5b25ef5818  effect/79-readnote.md
f684c1cdab89a13af728de7b99cc6c7bf523492b082731214699622563aaaf38  effect/judge-prompt.md
1f9168a2bdca3065d401d3c7c29f3b4b2a5e82de859a940121090b348309b1f9  effect/judge-roster.md
60d5701ee182a9eb1077edd19a297050b0614c4ae0ba9418370b173efc0c94d4  effect/green-129.txt
b93df4dc5f6d4b4c4de042f4b3e4478af28c6378be67d9d3f7d3e6a6d519101a  effect/rerun-126.txt
70ce2ab3e0fefd098516d9641ef969468b7c64175f5a806c60f6ce96a49c3808  effect/rerun-129.txt
1af319a65b1bc570e07c8d7c4c16ff4d0ba6b5f0357dd139d53cde2dfc2d3006  effect/rerun-79.txt
90dd52b3c33a099c0483beab38f153ce24ccfca4360c983320cea441d905cc04  observations/identity-after.txt
936ea91a39e2a56363fb86862676660cdc06e3f57347fb40b881d05bd6f03331  observations/identity-before.txt
0d3bc888c1719d6925697e339e4c38155536a753a1bdd20dab4f2ddcd6b40980  observations/issue-66-now.txt
3d0d0021da488fcb27bb37eb97652c8178eae5d957c8c87299291485dc82b740  observations/ledger-pair-after.txt
c665f77bf7b683235fc207042b8470c53f556671d3e94129624faec99769a9d1  observations/ledger-pair-before.txt
24c8ea46fa126c8dc6828398c2eeb0132bed8acfd5c03c32aca40cc63f47fd39  observations/red-129.txt
d05a9c36d2e772a8315092b05d16de65be86dcc74984913277653a7660c46083  observations/red-survey.md
eba0d8b8d54879992e2e4929855d349f0a4fdd51e58fd0a66ce5cb27af4e385d  observations/red-verify.md
42fa1227785d777cd476a1076b687d76bbff08920ce5e02e6d13552062041fe0  observations/red.md
829871c1eabcb1d24d21e7b9f04be47b8a7d5afbca4ccad9e9439b287121703c  dialectic/1-design.md
829871c1eabcb1d24d21e7b9f04be47b8a7d5afbca4ccad9e9439b287121703c  dialectic/2-design.md
829871c1eabcb1d24d21e7b9f04be47b8a7d5afbca4ccad9e9439b287121703c  dialectic/3-design.md
829871c1eabcb1d24d21e7b9f04be47b8a7d5afbca4ccad9e9439b287121703c  dialectic/4-design.md
ececd3f8dc676b619fbbc59d5d8ea1b6e9fb1a1e70f361f4f5aa272ceb9ac84f  dialectic/preflight.md
cde538cd875488eb87823bd046c0719d11a4e1d77d22445db90d4c49ce36e329  docs/gates/effect-confirmation.md
```

## 라운드 2 의 핀 이후 무엇이 움직였나 — **전수**

앞 핀 `05f07ea` 의 마흔 항을 지금 값과 대면 어긋난 것이 **일곱**이고, 그중
`dialectic/preflight.md`(자기 핀 · 구조상 만족 불가)를 빼면 **여섯**이다.

| 파일 | 왜 움직였나 |
|---|---|
| `intent.md` | 이 회차가 자기 결정을 적었다 — `## 승격` 칸 6~12 · `## 상한` · `## 개정` |
| `findings.jsonl` | 원장이 늘었다 — `MS-10`~`MS-20` |
| `touch/79.txt` · `touch/129.txt` | ⓓ2 덧붙임 일곱 줄을 머리 끝에서 **전 출력 뒤**로 옮겼다. 핀 대비 **앞 65 줄·38 줄이 다시 바이트로 동일**하다 |
| `effect/79-delta.md` | 차이 행 ㉲·㉳ 가 들어왔다(`017e5f1`) |
| **`docs/gates/effect-confirmation.md`** | ★ **칸 9 가 빠뜨린 것.** 라운드 2 의 판정 대상 자신이다. `c807f32` 가 거짓 여섯을 고치고 절 셋을 세웠다 |

★ **라운드 3 이 재는 대상은 `c807f32` 뒤의 게이트다.** 라운드 2 가 든 반증 근거 셋 중
둘(모집단 12↔13 · 죽은 좌표 `effect/negative-79.txt`)이 그 커밋에서 **없어졌다.**

## 판 4 — 차이 절 전수 ⟨수는 정이 세고 반이 검산한다⟩

```
effect/126-delta.md:20:## 2. 차이 — 넷이고 원인이 둘로 갈린다
effect/126-delta.md:24:| ㉮ | 사전 등록은 `최근에_끝난(&회차들, …)` 이라 적었는데 실제는 **`&끝난`**(`report.md` 로 먼저 거른 목록)을 넘긴다 | **touch 아님** — 사전 등록 §4 ⑵ 가 이미 *"호출자가 `report.md` 존재로 걸러 넘긴다"* 라고 적었다. ⑴ 의 표기가 그것과 어긋났던 것이고 실제는 ⑵ 를 따랐다 | `xtask/src/main.rs:6345-6350` |
effect/126-delta.md:25:| ㉯ | `%ct` 를 고른 근거(`%at` 은 rebase 가 옛 값을 들고 다닌다)를 코드 주석에 적었다 — 사전 등록엔 `%ct` 만 있고 까닭이 없었다 | **touch 아님** — 함수를 쓰다 나온 것 | `xtask/src/main.rs:6432-6433` |
effect/126-delta.md:26:| ㉰ | **새 함수를 `check_ledger_pair` 뒤에 두는 선택의 근거가 바뀌었다** — 「보기 좋다」 → 「이 심볼의 정체성이 선언 순서에 안 걸린다는 것을 확인했다」 | **`touch/126.txt:17`** (`identity ordinal`) → 따라가서 `crates/pal-core/src/coord.rs:246-266` 과 `crates/pal-cli/src/ledger.rs:334` | `xtask/src/main.rs:6407`(둔 자리 자체) |
effect/126-delta.md:27:| ㉱ | **호출자를 찾는 걸음이 없어졌다** — 등록 표를 안 건드리기로 확정했다 | **`touch/126.txt:24`** (`호출자 1 · 피호출자 12`) | `xtask/src/main.rs:643`(**안 건드린** 자리) |
effect/79-delta.md:23:## 2. 차이 — **여섯**이고 그중 **하나는 touch 가 원인이다**
effect/79-delta.md:35:| ㉮ | 세는 메서드 이름이 `센다` → **`헤아린다`** | **저장소 검사** — `cargo xtask check` 의 「어색한 표현 부재」가 내가 새로 넣은 **13 곳**을 잡았다(「~를 낸다」·「~ 센다」·「접다」) | `ledger.rs:337`·`:417` 외 11 곳 |
effect/79-delta.md:36:| ㉯ | 화면에 **「그룹마다 첫 선언은 빠진다」** 한 줄을 더했다 | **시험을 쓰다 나왔다** — `ordinal` 은 0 부터라 그룹의 첫 선언은 ②로 간다. 그러면 「순서에 취약 10」은 겹친 자리의 수가 아니라 **초과분**이다 | `ledger.rs:653-656` |
effect/79-delta.md:37:| ㉰ | 셋째 시험에 **짝**을 달았다(같은 이름 둘이면 ①) | **음성 대조 실측** — 사전 등록 §6 의 예상이 **틀렸다**(아래 §4) | `ledger.rs:934-941` |
effect/79-delta.md:38:| ㉱ | ⑶ 의 호출자 목록을 **`grep` 이 지었다** | ⚠ **`touch/79.txt:23`** 가 `호출자 3` 을 냈고 그 목록에 `defect.rs:326` 이 **없었다** | `defect.rs:326` |
effect/79-delta.md:39:| ㉲ | 버킷 ① 의 술어가 **`ordinal > 0` → 「그 (체인·이름·종류) 그룹의 크기 > 1」**. `nodes_of` 가 2 패스가 되고 화면 문구 넷이 바뀐다. **`순서에 취약 10 → 20`** | **정반합** — 판 1 의 반(反) R1·R6 이 *"중복 그룹의 **첫 선언**도 순서가 바뀌면 `SymbolId` 가 움직인다"* 를 코드로 세웠고(`dialectic/1-synthesis.md:37`) 소유자가 **「그룹 단위로 다시 헤아린다」**를 골랐다. **touch 가 원인이 아니다** | `ledger.rs:334-379`(버킷 정의·`헤아린다`) · `:411-416`(1 패스) · `:436`(호출) · `:664-673`(화면) · `:883`·`:914`·`:938`(시험 셋) |
effect/79-delta.md:40:| ㉳ | `IdentityTally` 의 doc 주석 세 자리에서 **㉲ 뒤에 거짓이 된 문장**을 걷었다 — *"이 버킷은 「이름이 유일한 것」과 「그룹의 첫 선언」을 함께 담는다"* · *"셈법을 그룹 단위로 바꾸는 것은 … 이 자리에서 안 한다"* · 시험 주석의 *"둘째가 ①이다"* · *"판별자 상한은 `Exact` 이므로"* | **정반합** — 판 1 라운드 2 의 정(正) ⑦ 이 냈고 반(反)이 확인했으며 합(合)이 *"이번에 늘린다"* 로 처분했다(`dialectic/1-synthesis-r2.md`). `e44a87e`(R1-a 뒷정리)가 쓴 문단을 `06e8d63`(승격 집행)이 안 걷어낸 것이다. **touch 가 원인이 아니다** | `ledger.rs:310-317` · `:949` · `:966` |
effect/129-delta.md:25:## 2. 차이 — 셋이고 **어느 것도 touch 가 원인이 아니다**
effect/129-delta.md:29:| ㉮ | `Ingested::비어_있다()` 를 새로 세웠다 | **컴파일러** — 질의 경로가 인입을 안 부를 때 `Ingested` 하나가 필요했다. `Vec::new()` 를 흩으면 *"안 물었다"* 와 *"물었는데 0"* 이 같은 글자가 된다 | `narrative.rs:110-124` |
effect/129-delta.md:30:| ㉯ | `touch.rs` 의 `QueryCtx` 에 `narrative_unminted: 0` 을 더했다 | **컴파일러**(`E0063`) — `pal touch` 는 인입을 안 부른다 | `touch.rs:153` |
effect/129-delta.md:31:| ㉰ | 세는 낱말이 `센다` → `헤아린다` | **저장소 검사** — 「어색한 표현 부재」가 `narrative.rs:85` 를 잡았다. ㉠ 에 이어 두 번째다 | `narrative.rs:85` |
```

## 판 4 — 게이트 `## 효과` 의 차이 표 ⟨판정 대상⟩

```
182:| 자리 | # | 차이 | 원인이 touch 인가 | **방향** | **「touch 가 말한 것이 참이었나」** · 근거 |
184:| ㉡ `#126` | ㉮ | 사전 등록 표기(`&회차들`)와 실물(`&끝난`)이 어긋났다 | 아니다 | **나아졌다** — 사전 등록 §4 ⑴⑵ 의 내부 어긋남이 실물에서 ⑵ 쪽으로 풀렸다 | **해당 없음** — 대상이 없다 |
185:| ㉡ | ㉯ | `%ct` 를 고른 까닭을 주석에 적었다 | 아니다 | **나아졌다** — 근거가 코드에 섰다(`xtask/src/main.rs:6432-6433`) | **해당 없음** |
186:| ㉡ | ㉰ | 새 함수를 `check_ledger_pair` 뒤에 두는 **근거**가 바뀌었다 | **그렇다** ⟨`touch/126.txt:17`⟩ | **나아졌다** — 「보기 좋다」가 「정체성이 선언 순서에 안 걸린다」로 바뀌었다 | **참이었다.** `identity ordinal` 은 합친 값이고 그 값이 실제로 `ordinal` 이다. ⚠ **원인은 안 가른다** — 판별자 상한은 `Exact` 이고 낮춘 것은 추출기 등급이다(`MS-07`·`MS-09` · `ledger.rs` 의 `check_ledger_pair_는_순서에_취약하지_않다`) |
187:| ㉡ | ㉱ | **호출자를 찾는 걸음이 없어졌다** | **그렇다** ⟨`touch/126.txt:24`⟩ | **나아졌다** — 걸음 하나가 사라졌다 | **참이었다.** `effect/126-readnote.md:14` 가 `grep -n "check_ledger_pair" xtask/src/main.rs` 로 다시 확인했고 `:643` 한 자리뿐이었다 |
188:| ㉠ `#79` | ㉮ | 세는 메서드 이름 `센다` → `헤아린다` | 아니다 | **나아졌다** — 저장소 검사가 잡은 13 곳이 없어졌다 | **해당 없음** |
189:| ㉠ | ㉯ | 화면에 「그룹마다 첫 선언은 빠진다」를 더했다 | 아니다 | **나아졌다** — 그때 화면이 싣던 거짓이 줄었다. ⚠ **그 줄은 ㉲ 가 걷어냈다** — 셈법 자체가 바뀌어 더 안 참이 됐다 | **해당 없음** |
190:| ㉠ | ㉰ | 셋째 시험에 **짝**을 달았다 | 아니다 | **나아졌다** — 항등식이던 시험이 실제로 재게 됐다(`effect/79-delta.md:70-88` §4 가 1 차·2 차 산출을 나란히 싣는다 — 1 차의 끈 판에서 `check_ledger_pair_는_순서에_취약하지_않다` 가 **안 빨개졌고**, 짝을 단 2 차에서 셋 다 FAILED 다) | **해당 없음** |
191:| ㉠ | ㉱ | ⑶ 의 호출자 목록을 **`grep` 이 지었다** | **그렇다** ⟨`touch/79.txt:23`⟩ | **나빠졌다** — 제품이 이 물음에 **덜 답했다**. 사전 등록에 `grep` 결과를 먼저 안 적었으면 `defect.rs` 를 안 고쳤다 | **참이지만 덜 말했다.** 「호출자 3」은 **참조 엣지 기준**으로 참이고(`effect/79-readnote.md:39-41` — *"touch 가 거짓말을 한 것은 아니다"*), 파일 간 미해소 몫이 표시 없이 빠진다(`MS-08`) |
192:| ㉠ | ㉲ | 버킷 ① 의 술어가 `ordinal > 0` → **그룹 크기 > 1**(`순서에 취약 10 → 20`) | 아니다 ⟨**정반합**⟩ | **나아졌다** — 산출이 참이 됐다. 중복 그룹의 첫 선언도 순서가 바뀌면 `SymbolId` 가 움직인다 | **해당 없음** |
193:| ㉠ | ㉳ | `IdentityTally` 의 doc 주석 **세 자리**에서 ㉲ 뒤에 거짓이 된 문장을 걷었다 | 아니다 ⟨**정반합**⟩ | **나아졌다** — ㉲ 가 셈법을 바꾸면서 주석 넷(*"이 버킷은 「이름이 유일한 것」과 「그룹의 첫 선언」을 함께 담는다"* · *"셈법을 그룹 단위로 바꾸는 것은 … 이 자리에서 안 한다"* · *"둘째가 ①이다"* · *"판별자 상한은 `Exact` 이므로"*)이 거짓이 됐고 그것이 없어졌다. **㉲ 없이는 이 차이가 없다** ⟨`effect/79-delta.md:40`⟩ | **해당 없음** — 대상이 없다. 원인 끝이 `dialectic/1-synthesis-r2.md` 이고 `touch/79.txt:N` 이 아니다 |
194:| ㉢ `#129` | ㉮ | `Ingested::비어_있다()` 를 세웠다 | 아니다 ⟨컴파일러⟩ | **나아졌다** — *"안 물었다"* 와 *"물었는데 0"* 이 갈렸다 | **해당 없음** |
195:| ㉢ | ㉯ | `QueryCtx` 에 `narrative_unminted: 0` 을 더했다 | 아니다 ⟨컴파일러 `E0063`⟩ | **나아졌다** — 빠진 칸이 타입으로 드러났다 | **해당 없음** |
196:| ㉢ | ㉰ | 세는 낱말 `센다` → `헤아린다` | 아니다 ⟨저장소 검사⟩ | **나아졌다** — 「어색한 표현 부재」가 `narrative.rs:85` 를 잡았고 그 한 자리가 없어졌다. **㉠㉮ 에 이어 두 번째다** ⟨`effect/129-delta.md:31`⟩ | **해당 없음** |
198:**검산** — 차이 **13** = ㉡ 4 + ㉠ **6** + ㉢ 3. 원인이 touch 인 것 **3** · 아닌 것 **10**.
```

## 판 4 — 죽은 좌표가 정말 죽었나 ⟨라운드 2 의 반증 근거 ②⟩

```
$ ls .palimpsest/rounds/2026-09-11-effect-confirmation/effect/ | grep negative
negative-B5-group.txt

$ git log --all --oneline -- '*negative-79*'
(빈 출력)

$ git check-ignore -v effect/negative-79.txt ; echo "rc=$?"
rc=1

$ grep -c 'negative-79.txt' docs/gates/effect-confirmation.md
1
```

⚠ **남은 한 자리는 정정문 자신이다** — `### 정반합 산출물 — 정오표와 성격` 절이 그
좌표를 *"이 정정이 고친 바로 그 자리"* 로 인용한다. 근거로 세운 것이 아니라 **없어진
것으로 적은 것**이다.
