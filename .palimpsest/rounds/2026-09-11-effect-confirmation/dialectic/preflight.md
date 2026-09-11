# preflight — 기계가 먼저 돈 것 ⟨설계문 §기계가 먼저 돈다⟩

> 정(正)과 반(反)에게 **같은 바이트**로 준다. 셀 수 있는 것을 놓고 다투면 라운드가 탄다.

## 공통 — 기준 핀

```
$ git rev-parse HEAD
05f07ea995d534ec319426db2b301e1310b8a446

$ git status --porcelain
?? .palimpsest/rounds/2026-09-11-effect-confirmation/dialectic/preflight.md
(위가 비면 워킹트리가 깨끗하다)
```

**기준 커밋은 위 SHA 다. 그 뒤에 생긴 바이트는 증거가 아니다.**

### 증거 파일의 sha256

```
6ad3e4f4cae7ebd512e2779899af89080db042bbd158f9cd56348681ede09088  intent.md
49cadbdd4c309b792ec44650a646132479e7bf1054cc3c4d310d027ab849a3f5  findings.jsonl
628d11a82d3dcbb2141e7bfe101f875c589afd43c6ce31b99b49b1090dd645e9  plan/126-pre.md
cc1d0abf9e1e33da42b5a8d628633b94e947536009f7c4aa5108830c71acf3d2  plan/129-pre.md
f62783a58b19b55dddb28f34abbd3c7d0af0d68598160c9cbfd87f028ed9a0e4  plan/79-pre.md
08f9b674ab7ca8013440a80dbe21a04a6d3315c159b27607be14b6da333be917  touch/126.txt
3a6e24be123f1867164e8bee0755028602a9a686edc440e71a62e5cbd35a1f62  touch/129.txt
41f95666f3521928a3a1625cf410621bf948dfcf8c6d9e84e14dfa194437e88e  touch/79.txt
23bf96bea7cd7d2772a4ef06aad39e1ac3e1865ed0520bbbeebf74015217e59f  effect/126-delta.md
e9a673164f58b339201454a4fd916704c3a373ed950c48e5674969c2170d16dc  effect/126-judge-1.md
70d300b2e680c3b38f41874da5db674a5f297174098175888ac39649a5cb9ad8  effect/126-judge-2.md
7b4cf9b7e9a0af69f137648284eccc94756cf510e52e491cb1e36ada0980f518  effect/126-readnote.md
5254707a06f4c9dc7859e0020657601559af0d75ba69ebb34dbbe612e2dbb93f  effect/129-delta.md
a3d5e7a99b70022f50a9062cd93ecd689e55368d82ee8f4bef24766bf2ba3025  effect/129-judge-1.md
357f2c18cf13608014204b00979f15dcdc5367187d699a1991abbedcea815bc5  effect/129-judge-2.md
8c3b3d9ca21e4f3ee8d0038131a7dc42b6fce89a3bd594b26a282f02457bfd0d  effect/129-readnote.md
3387cb7fe6f9cfd8ac85915ce8f47abe0c0b764ad09bb4ae27971190bf6187d1  effect/79-delta.md
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
004949f56444d74edba03f8f2a45e8beb6a3dc64b7d2b6e740306281fd935b3f  dialectic/preflight.md
306ad7ec0c73692307f82de77ff2ae4286e4cdb041eaf9c288b610182ad88adf  docs/gates/effect-confirmation.md
```

## 판 1 — `B1`

```
$ git show --stat --format='%h %s' 398d233 9f993cc 1abc0e9 | grep -E '^[0-9a-f]{7} |files? changed'
398d233 round(effect-confirmation): ㉡ `#126` — 「최근」을 사전순에서 **커밋 시각**으로 옮긴다
 3 files changed, 180 insertions(+), 8 deletions(-)
9f993cc round(effect-confirmation): ㉠ `#79` — 정체성 상한을 버리지 않고 넷으로 갈라 산출한다
 4 files changed, 312 insertions(+), 9 deletions(-)
1abc0e9 round(effect-confirmation): ㉢ `#129` — 읽기 표면이 개체를 안 만든다
 6 files changed, 243 insertions(+), 8 deletions(-)
```

### 세 자리의 시험 이름 — 코드에서 뜬 것

```
$ grep -n 'fn 최근에_끝난_것은\|fn 커밋_시각이_없는\|fn 시각이_같으면' xtask/src/main.rs
6460:    fn 최근에_끝난_것은_사전순이_아니라_커밋_시각으로_고른다() {
6475:    fn 커밋_시각이_없는_회차는_가장_최근으로_본다() {
6484:    fn 시각이_같으면_사전순_최대를_고른다() {
$ grep -n 'fn 순서로_가린\|fn 합이_분모와\|fn check_ledger_pair_는' crates/pal-cli/src/ledger.rs
861:    fn 순서로_가린_것과_등급이_낮은_것이_갈린다() {
885:    fn 합이_분모와_같다() {
923:    fn check_ledger_pair_는_순서에_취약하지_않다() {
$ grep -n 'fn narrative_unbound_는\|fn 읽기_경로는' crates/pal-cli/tests/narrative_read_only.rs
51:fn narrative_unbound_는_읽기로도_돈다() {
73:fn 읽기_경로는_의도_저장소를_안_불린다() {
```

## 판 2 — `A5-c` · 「워킹트리」 문자열 전수

```
$ grep -rn '워킹트리' .palimpsest/rounds/2026-09-11-effect-confirmation docs/gates/effect-confirmation.md
conditions-audit/r1-raw.md:179:- 어떻게 실패하나: 손으로 한 줄 더한 산출을 잡는 조건이 없고, 재실행 대조도 원리상 좁다 — 실측한 산출 머리가 `palimpsest@4ab56d0+worktree#a9dd338ebdd7` 로 **워킹트리 다이제스트**를 실어 같은 SHA 로 다시 돌려도 바이트가 안 맞는다. `A1-b` 는 계획만 봉인하고 **산출 쪽에는 대응하는 봉인이 없다**
findings.jsonl:58:{"id": "PM2-04", "라운드": 2, "출처": "사전부검", "모집단": "저장소", "유효성": "참", "해악도": "금지역", "처분": "정정", "경로": "crates/pal-cli/src/touch.rs", "요약": "pal touch 가 더러운 워킹트리에서 「워킹트리 일치」를 찍는다", "승격됨": "아니오", "조건변경": "강화", "사전처분": "계획수정", "조건": "없음", "줄": null, "기준커밋": "834ba43", "상태": "닫힘", "닫은커밋": "834ba43", "처분자리": "intent.md"}
conditions-audit/r2-raw.md:44:- 어떻게 실패하나: 격리 사본에서 재현했다. 본문을 한 줄로 병합하고 `// AUDIT PROBE` 를 넣어도 `body c5cfb1f369bc` 가 그대로였고(인덱스·캐시를 **지우고 다시 세운 뒤에도** 같았다), `self.ordinal == 0` → `== 7` 로 바꾸자 `d6f47e7b25a7` 로 움직였다. `A5-b` 가 막으려는 공격(변경을 워킹트리에 먼저 써 놓고 touch)은 **포매팅·주석만 건드린 변경에서는 안 잡힌다**
conditions-audit/r2-raw.md:154:- 어떻게 실패하나: 열거할 방법도, 댈 코드 좌표도 조건에 없다. ⟨결정론적⟩ 태그인데 확인 행위는 코드 독해다. ⟨전제 자체는 실측으로 참이었다 — 더러운 워킹트리에서도 `워킹트리 일치`, `--at` 을 붙이면 `워킹트리 다름`⟩
conditions-audit/r2-raw.md:200:- 획득: 실측 — 두 번 돌려 `diff` 한 결과가 **바이트로 동일**했다. `#<hex>` 는 `crates/pal-core/src/coord.rs:294-297` 의 `Display` 가 `self.symbol.short()` 로 찍는 **심볼 ID** 이지 워킹트리 다이제스트가 아니다
dialectic/4-design.md:23:| **판 2** | `A5-c` | 「워킹트리 일치」 줄을 증거로 **안 썼고**, 그 줄이 `--at` 없는 산출에서 **구조적으로 참**이라는 코드 독해 논증이 서며, 그 사실이 게이트에 적혔나 | 제품 코드 · 회차 산출물 전량 · 게이트 |
dialectic/4-design.md:45:설계 시점(2026-09-12) 워킹트리는 **커밋 안 된 변경 넷**을 들고 있다:
dialectic/4-design.md:66:| `A5-c` | 회차 산출물·게이트에서 「워킹트리」 문자열 전수(`rg`) · 게이트 파일의 존재와 바이트 | ① *"증거로 **안 썼다**"* 는 인용의 **용법** 판정 ② *"`--at` 없이 부른 **모든 산출**에서 **구조적으로 참**"* 은 **코드 독해 논증** ⟨`CA2-14`⟩ |
dialectic/4-design.md:78:- **판 2** — 회차 디렉터리와 `docs/gates/effect-confirmation.md` 에 대한 `rg -n "워킹트리"` **전량** · 아래 코드 다섯 자리의 원문 · 같은 줄을 찍는 다른 표면 목록(`crates/pal-cli/src/export.rs:376` · `crates/pal-cli/src/doctor.rs:177` · `crates/pal-query/src/lib.rs:689`).
dialectic/4-design.md:104:- `crates/pal-cli/src/touch.rs:310-313` — 그 값이 `워킹트리  일치 / 다름` 으로 찍힌다
dialectic/4-design.md:131:| 판 2 | 코드 다섯 자리 원문 · `rg "워킹트리"` 전량 · 게이트 파일 바이트(또는 부재 산출) | 판 3·4 의 증거 묶음 |
dialectic/4-design.md:260:- **제품 결함의 수리** — 「워킹트리 일치」의 공허함(`A5-c`) · `크기` 줄(`MS-06`) ·
intent.md:182:- [ ] **A2-b** ⟨결정론적⟩ **재실행 대조가 선다** — 같은 HEAD·같은 워킹트리에서 그 명령을 다시 돌려 `diff` 가 **바이트로 동일**함을 보이고 산출을 `effect/rerun-<이슈>.txt` 로 남긴다. ⟨`PM2-10`·`PM3-07` — `CA1-17` 이 *"다시 돌려도 바이트가 안 맞는다"* 를 근거로 재실행 대조를 **포기**했는데 **그 근거가 거짓이다.** 두 번 돌려 `diff` 한 결과가 동일했고, `#<hex>` 는 워킹트리 다이제스트가 아니라 **심볼마다 다른 심볼 ID** 다(`crates/pal-core/src/coord.rs:296`). 더 강한 것이 가능한데 더 약한 것을 쓰면서 **강한 것이 불가능하다고 잘못 적었다**⟩
intent.md:188:- [ ] **A5-b** ⟨결정론적⟩ ★★ **커밋 순서만으로는 인과가 안 선다 — 바이트 앵커를 건다.** `touch/<이슈>.txt` 안의 `fun · <파일>:<줄> · … body <hash>` 줄이 **봉인 커밋 시점의 그 심볼 본문**에서 나온 값임을 보인다. ⟨`PM2-03`(격리 사본에서 재현)·`PM3-14`(유효성 **추정** — R3 은 사본을 안 돌렸다) — 재현된 것은 `PM2-03` 이다: 코드 변경을 **워킹트리에 먼저 써 놓고** touch 를 돌린 뒤 봉인→touch→변경 순으로 커밋하면 `A1`·`A1-b`·`A1-c`·`A5`·`A6` 이 **전부 통과**하면서 touch 는 변경을 이미 본 상태다⟩
intent.md:191:- [ ] **A5-c** ⟨정반합⟩ **`pal touch` 의 「워킹트리 일치」 줄을 증거로 안 쓴다** ⟨`CA2-14` — *"모든 산출에서 구조적으로 참"* 은 열거가 아니라 코드 독해 논증이라 ⟨결정론적⟩이 아니다⟩ — 그 줄이 `--at` 없이 부른 모든 산출에서 **구조적으로 참**임을 확인하고 그 사실을 게이트에 적는다. ⟨`PM2-04` — 더러운 워킹트리에서도 `일치` 를 찍는다. 사람은 그 줄을 *"touch 를 돌릴 때 고친 것이 없었다"* 로 읽는다. 제품의 결함이고 이 회차는 **관측만 하고 안 고친다**⟩
intent.md:192:- [ ] **A5-a** ⟨결정론적 · `A5`·`A5-b` 의 음성 대조⟩ 사본 **둘**에서 판정이 **뒤집히는지** 재고 `effect/negative-A5.md` 에 둘 다 남긴다 — ⓐ touch 커밋을 변경 커밋 뒤로 옮긴 사본에서 `A5` 가 빨개진다 · ⓑ 코드 변경을 **워킹트리에 먼저 써 놓고** touch 를 돌린 사본에서 `A5-b` 의 `body <hash>` 가 봉인 커밋 시점의 본문과 **안 맞는다**. **둘 중 하나라도 안 뒤집히면 그 축의 조건은 항등식이다** — ⓐ 가 안 뒤집히면 `A5`, ⓑ 면 `A5-b` 다. 한 축만 초록이어도 `A5-a` 는 **불통과**다 ⟨`PM2-03` 이 격리 사본에서 재현한 순서가 바로 ⓑ 인데 `A5-b` 에만 대조가 없었다. 조건을 늘리지 않으려고 `A5-a` 에 축을 하나 더 걸었다⟩
intent.md:313:  라 적어 **방향이 반대**다. `A5-c` 의 「워킹트리 일치」와 같은 부류 — **제품의 표시 결함이고
intent.md:338:| 2026-09-11 ⟨사전부검 R1⟩ | 착수 관측의 **죽은 링크를 고치고** 워킹트리 문장(`observations/red.md` 머리 인용구 — **절 이름이 아니다**)을 잰 시점에 묶었다 | 내 관측 파일이 `cargo xtask check` 를 **26/28 로 떨어뜨리고 있었다.** 실측으로 확인하고 고쳤다 ⟨`PM1-11`⟩ |
intent.md:350:| 2026-09-11 ⟨사전부검 R2·R3⟩ | 조건이 **43 → 46**. 넷을 세우고(`A5-b` 바이트 앵커 · `A2-b` 재실행 대조 · `A2-c` ㉢ 후보 목록 처분 · `A5-c` 워킹트리 줄 안 씀) `C2-c` 하나를 뺐다. `A6` 은 「보존본 하나」로 고쳤다 | 커밋 순서만으로는 인과가 안 선다(격리 사본 재현) · `CA1-17` 의 근거가 거짓이라 재실행 대조가 **실제로 가능**했다 · `A6` 의 「한 번만」은 §5.8 때문에 착수 전부터 불통과였다 |
intent.md:356:| 2026-09-11 ⟨기록 정합⟩ | `A5-a` 에 **축 ⓑ 를 더했다** — 워킹트리에 먼저 쓴 사본에서 `A5-b` 의 앵커가 빨개지는지 잰다. **조건 수는 50 그대로다** | ★★ 가 붙은 `A5-b` 에만 음성 대조가 없었다. `A5-d`·`A5-e` 는 대조가 아니라 적용 범위를 좁히는 단서다. 규약 `SKILL.md:605` 가 *"가르는 물음: 「사본을 부수면 정말 빨개지는가.」"* 로 못 박고, 이 회차 `state.md` 의 「걸리는 자리」 4 가 *"음성 대조 자신이 항등식일 수 있다 — 세운 뒤 끄면 정말 빨개지는지 돌려 봐라"* 로 받은 자리 |
touch/79.txt:9:# 종료값 0 · 표준오류 0 바이트 · 워킹트리 깨끗 · 산출 52 줄 · **4066 바이트**
touch/79.txt:57:  워킹트리  일치
dialectic/2-design.md:23:| **판 2** | `A5-c` | 「워킹트리 일치」 줄을 증거로 **안 썼고**, 그 줄이 `--at` 없는 산출에서 **구조적으로 참**이라는 코드 독해 논증이 서며, 그 사실이 게이트에 적혔나 | 제품 코드 · 회차 산출물 전량 · 게이트 |
dialectic/2-design.md:45:설계 시점(2026-09-12) 워킹트리는 **커밋 안 된 변경 넷**을 들고 있다:
dialectic/2-design.md:66:| `A5-c` | 회차 산출물·게이트에서 「워킹트리」 문자열 전수(`rg`) · 게이트 파일의 존재와 바이트 | ① *"증거로 **안 썼다**"* 는 인용의 **용법** 판정 ② *"`--at` 없이 부른 **모든 산출**에서 **구조적으로 참**"* 은 **코드 독해 논증** ⟨`CA2-14`⟩ |
dialectic/2-design.md:78:- **판 2** — 회차 디렉터리와 `docs/gates/effect-confirmation.md` 에 대한 `rg -n "워킹트리"` **전량** · 아래 코드 다섯 자리의 원문 · 같은 줄을 찍는 다른 표면 목록(`crates/pal-cli/src/export.rs:376` · `crates/pal-cli/src/doctor.rs:177` · `crates/pal-query/src/lib.rs:689`).
dialectic/2-design.md:104:- `crates/pal-cli/src/touch.rs:310-313` — 그 값이 `워킹트리  일치 / 다름` 으로 찍힌다
dialectic/2-design.md:131:| 판 2 | 코드 다섯 자리 원문 · `rg "워킹트리"` 전량 · 게이트 파일 바이트(또는 부재 산출) | 판 3·4 의 증거 묶음 |
dialectic/2-design.md:260:- **제품 결함의 수리** — 「워킹트리 일치」의 공허함(`A5-c`) · `크기` 줄(`MS-06`) ·
dialectic/3-design.md:23:| **판 2** | `A5-c` | 「워킹트리 일치」 줄을 증거로 **안 썼고**, 그 줄이 `--at` 없는 산출에서 **구조적으로 참**이라는 코드 독해 논증이 서며, 그 사실이 게이트에 적혔나 | 제품 코드 · 회차 산출물 전량 · 게이트 |
dialectic/3-design.md:45:설계 시점(2026-09-12) 워킹트리는 **커밋 안 된 변경 넷**을 들고 있다:
dialectic/3-design.md:66:| `A5-c` | 회차 산출물·게이트에서 「워킹트리」 문자열 전수(`rg`) · 게이트 파일의 존재와 바이트 | ① *"증거로 **안 썼다**"* 는 인용의 **용법** 판정 ② *"`--at` 없이 부른 **모든 산출**에서 **구조적으로 참**"* 은 **코드 독해 논증** ⟨`CA2-14`⟩ |
dialectic/3-design.md:78:- **판 2** — 회차 디렉터리와 `docs/gates/effect-confirmation.md` 에 대한 `rg -n "워킹트리"` **전량** · 아래 코드 다섯 자리의 원문 · 같은 줄을 찍는 다른 표면 목록(`crates/pal-cli/src/export.rs:376` · `crates/pal-cli/src/doctor.rs:177` · `crates/pal-query/src/lib.rs:689`).
dialectic/3-design.md:104:- `crates/pal-cli/src/touch.rs:310-313` — 그 값이 `워킹트리  일치 / 다름` 으로 찍힌다
dialectic/3-design.md:131:| 판 2 | 코드 다섯 자리 원문 · `rg "워킹트리"` 전량 · 게이트 파일 바이트(또는 부재 산출) | 판 3·4 의 증거 묶음 |
dialectic/3-design.md:260:- **제품 결함의 수리** — 「워킹트리 일치」의 공허함(`A5-c`) · `크기` 줄(`MS-06`) ·
dialectic/1-design.md:23:| **판 2** | `A5-c` | 「워킹트리 일치」 줄을 증거로 **안 썼고**, 그 줄이 `--at` 없는 산출에서 **구조적으로 참**이라는 코드 독해 논증이 서며, 그 사실이 게이트에 적혔나 | 제품 코드 · 회차 산출물 전량 · 게이트 |
dialectic/1-design.md:45:설계 시점(2026-09-12) 워킹트리는 **커밋 안 된 변경 넷**을 들고 있다:
dialectic/1-design.md:66:| `A5-c` | 회차 산출물·게이트에서 「워킹트리」 문자열 전수(`rg`) · 게이트 파일의 존재와 바이트 | ① *"증거로 **안 썼다**"* 는 인용의 **용법** 판정 ② *"`--at` 없이 부른 **모든 산출**에서 **구조적으로 참**"* 은 **코드 독해 논증** ⟨`CA2-14`⟩ |
dialectic/1-design.md:78:- **판 2** — 회차 디렉터리와 `docs/gates/effect-confirmation.md` 에 대한 `rg -n "워킹트리"` **전량** · 아래 코드 다섯 자리의 원문 · 같은 줄을 찍는 다른 표면 목록(`crates/pal-cli/src/export.rs:376` · `crates/pal-cli/src/doctor.rs:177` · `crates/pal-query/src/lib.rs:689`).
dialectic/1-design.md:104:- `crates/pal-cli/src/touch.rs:310-313` — 그 값이 `워킹트리  일치 / 다름` 으로 찍힌다
dialectic/1-design.md:131:| 판 2 | 코드 다섯 자리 원문 · `rg "워킹트리"` 전량 · 게이트 파일 바이트(또는 부재 산출) | 판 3·4 의 증거 묶음 |
dialectic/1-design.md:260:- **제품 결함의 수리** — 「워킹트리 일치」의 공허함(`A5-c`) · `크기` 줄(`MS-06`) ·
touch/126.txt:9:# 종료값 0 · 표준오류 0 바이트 · 워킹트리 깨끗(`git status --porcelain` 빈 출력)
touch/126.txt:52:  워킹트리  일치
dialectic/preflight.md:13:(위가 비면 워킹트리가 깨끗하다)
dialectic/preflight.md:91:## 판 2 — `A5-c` · 「워킹트리」 문자열 전수
dialectic/preflight.md:94:$ grep -rn '워킹트리' .palimpsest/rounds/2026-09-11-effect-confirmation docs/gates/effect-confirmation.md
dialectic/preflight.md:95:conditions-audit/r1-raw.md:179:- 어떻게 실패하나: 손으로 한 줄 더한 산출을 잡는 조건이 없고, 재실행 대조도 원리상 좁다 — 실측한 산출 머리가 `palimpsest@4ab56d0+worktree#a9dd338ebdd7` 로 **워킹트리 다이제스트**를 실어 같은 SHA 로 다시 돌려도 바이트가 안 맞는다. `A1-b` 는 계획만 봉인하고 **산출 쪽에는 대응하는 봉인이 없다**
dialectic/preflight.md:96:findings.jsonl:58:{"id": "PM2-04", "라운드": 2, "출처": "사전부검", "모집단": "저장소", "유효성": "참", "해악도": "금지역", "처분": "정정", "경로": "crates/pal-cli/src/touch.rs", "요약": "pal touch 가 더러운 워킹트리에서 「워킹트리 일치」를 찍는다", "승격됨": "아니오", "조건변경": "강화", "사전처분": "계획수정", "조건": "없음", "줄": null, "기준커밋": "834ba43", "상태": "닫힘", "닫은커밋": "834ba43", "처분자리": "intent.md"}
dialectic/preflight.md:97:conditions-audit/r2-raw.md:44:- 어떻게 실패하나: 격리 사본에서 재현했다. 본문을 한 줄로 병합하고 `// AUDIT PROBE` 를 넣어도 `body c5cfb1f369bc` 가 그대로였고(인덱스·캐시를 **지우고 다시 세운 뒤에도** 같았다), `self.ordinal == 0` → `== 7` 로 바꾸자 `d6f47e7b25a7` 로 움직였다. `A5-b` 가 막으려는 공격(변경을 워킹트리에 먼저 써 놓고 touch)은 **포매팅·주석만 건드린 변경에서는 안 잡힌다**
dialectic/preflight.md:98:conditions-audit/r2-raw.md:154:- 어떻게 실패하나: 열거할 방법도, 댈 코드 좌표도 조건에 없다. ⟨결정론적⟩ 태그인데 확인 행위는 코드 독해다. ⟨전제 자체는 실측으로 참이었다 — 더러운 워킹트리에서도 `워킹트리 일치`, `--at` 을 붙이면 `워킹트리 다름`⟩
dialectic/preflight.md:99:conditions-audit/r2-raw.md:200:- 획득: 실측 — 두 번 돌려 `diff` 한 결과가 **바이트로 동일**했다. `#<hex>` 는 `crates/pal-core/src/coord.rs:294-297` 의 `Display` 가 `self.symbol.short()` 로 찍는 **심볼 ID** 이지 워킹트리 다이제스트가 아니다
dialectic/preflight.md:100:dialectic/4-design.md:23:| **판 2** | `A5-c` | 「워킹트리 일치」 줄을 증거로 **안 썼고**, 그 줄이 `--at` 없는 산출에서 **구조적으로 참**이라는 코드 독해 논증이 서며, 그 사실이 게이트에 적혔나 | 제품 코드 · 회차 산출물 전량 · 게이트 |
dialectic/preflight.md:101:dialectic/4-design.md:45:설계 시점(2026-09-12) 워킹트리는 **커밋 안 된 변경 넷**을 들고 있다:
dialectic/preflight.md:102:dialectic/4-design.md:66:| `A5-c` | 회차 산출물·게이트에서 「워킹트리」 문자열 전수(`rg`) · 게이트 파일의 존재와 바이트 | ① *"증거로 **안 썼다**"* 는 인용의 **용법** 판정 ② *"`--at` 없이 부른 **모든 산출**에서 **구조적으로 참**"* 은 **코드 독해 논증** ⟨`CA2-14`⟩ |
dialectic/preflight.md:103:dialectic/4-design.md:78:- **판 2** — 회차 디렉터리와 `docs/gates/effect-confirmation.md` 에 대한 `rg -n "워킹트리"` **전량** · 아래 코드 다섯 자리의 원문 · 같은 줄을 찍는 다른 표면 목록(`crates/pal-cli/src/export.rs:376` · `crates/pal-cli/src/doctor.rs:177` · `crates/pal-query/src/lib.rs:689`).
dialectic/preflight.md:104:dialectic/4-design.md:104:- `crates/pal-cli/src/touch.rs:310-313` — 그 값이 `워킹트리  일치 / 다름` 으로 찍힌다
dialectic/preflight.md:105:dialectic/4-design.md:131:| 판 2 | 코드 다섯 자리 원문 · `rg "워킹트리"` 전량 · 게이트 파일 바이트(또는 부재 산출) | 판 3·4 의 증거 묶음 |
dialectic/preflight.md:106:dialectic/4-design.md:260:- **제품 결함의 수리** — 「워킹트리 일치」의 공허함(`A5-c`) · `크기` 줄(`MS-06`) ·
dialectic/preflight.md:107:intent.md:182:- [ ] **A2-b** ⟨결정론적⟩ **재실행 대조가 선다** — 같은 HEAD·같은 워킹트리에서 그 명령을 다시 돌려 `diff` 가 **바이트로 동일**함을 보이고 산출을 `effect/rerun-<이슈>.txt` 로 남긴다. ⟨`PM2-10`·`PM3-07` — `CA1-17` 이 *"다시 돌려도 바이트가 안 맞는다"* 를 근거로 재실행 대조를 **포기**했는데 **그 근거가 거짓이다.** 두 번 돌려 `diff` 한 결과가 동일했고, `#<hex>` 는 워킹트리 다이제스트가 아니라 **심볼마다 다른 심볼 ID** 다(`crates/pal-core/src/coord.rs:296`). 더 강한 것이 가능한데 더 약한 것을 쓰면서 **강한 것이 불가능하다고 잘못 적었다**⟩
dialectic/preflight.md:108:intent.md:188:- [ ] **A5-b** ⟨결정론적⟩ ★★ **커밋 순서만으로는 인과가 안 선다 — 바이트 앵커를 건다.** `touch/<이슈>.txt` 안의 `fun · <파일>:<줄> · … body <hash>` 줄이 **봉인 커밋 시점의 그 심볼 본문**에서 나온 값임을 보인다. ⟨`PM2-03`(격리 사본에서 재현)·`PM3-14`(유효성 **추정** — R3 은 사본을 안 돌렸다) — 재현된 것은 `PM2-03` 이다: 코드 변경을 **워킹트리에 먼저 써 놓고** touch 를 돌린 뒤 봉인→touch→변경 순으로 커밋하면 `A1`·`A1-b`·`A1-c`·`A5`·`A6` 이 **전부 통과**하면서 touch 는 변경을 이미 본 상태다⟩
dialectic/preflight.md:109:intent.md:191:- [ ] **A5-c** ⟨정반합⟩ **`pal touch` 의 「워킹트리 일치」 줄을 증거로 안 쓴다** ⟨`CA2-14` — *"모든 산출에서 구조적으로 참"* 은 열거가 아니라 코드 독해 논증이라 ⟨결정론적⟩이 아니다⟩ — 그 줄이 `--at` 없이 부른 모든 산출에서 **구조적으로 참**임을 확인하고 그 사실을 게이트에 적는다. ⟨`PM2-04` — 더러운 워킹트리에서도 `일치` 를 찍는다. 사람은 그 줄을 *"touch 를 돌릴 때 고친 것이 없었다"* 로 읽는다. 제품의 결함이고 이 회차는 **관측만 하고 안 고친다**⟩
dialectic/preflight.md:110:intent.md:192:- [ ] **A5-a** ⟨결정론적 · `A5`·`A5-b` 의 음성 대조⟩ 사본 **둘**에서 판정이 **뒤집히는지** 재고 `effect/negative-A5.md` 에 둘 다 남긴다 — ⓐ touch 커밋을 변경 커밋 뒤로 옮긴 사본에서 `A5` 가 빨개진다 · ⓑ 코드 변경을 **워킹트리에 먼저 써 놓고** touch 를 돌린 사본에서 `A5-b` 의 `body <hash>` 가 봉인 커밋 시점의 본문과 **안 맞는다**. **둘 중 하나라도 안 뒤집히면 그 축의 조건은 항등식이다** — ⓐ 가 안 뒤집히면 `A5`, ⓑ 면 `A5-b` 다. 한 축만 초록이어도 `A5-a` 는 **불통과**다 ⟨`PM2-03` 이 격리 사본에서 재현한 순서가 바로 ⓑ 인데 `A5-b` 에만 대조가 없었다. 조건을 늘리지 않으려고 `A5-a` 에 축을 하나 더 걸었다⟩
dialectic/preflight.md:111:intent.md:313:  라 적어 **방향이 반대**다. `A5-c` 의 「워킹트리 일치」와 같은 부류 — **제품의 표시 결함이고
dialectic/preflight.md:112:intent.md:338:| 2026-09-11 ⟨사전부검 R1⟩ | 착수 관측의 **죽은 링크를 고치고** 워킹트리 문장(`observations/red.md` 머리 인용구 — **절 이름이 아니다**)을 잰 시점에 묶었다 | 내 관측 파일이 `cargo xtask check` 를 **26/28 로 떨어뜨리고 있었다.** 실측으로 확인하고 고쳤다 ⟨`PM1-11`⟩ |
dialectic/preflight.md:113:intent.md:350:| 2026-09-11 ⟨사전부검 R2·R3⟩ | 조건이 **43 → 46**. 넷을 세우고(`A5-b` 바이트 앵커 · `A2-b` 재실행 대조 · `A2-c` ㉢ 후보 목록 처분 · `A5-c` 워킹트리 줄 안 씀) `C2-c` 하나를 뺐다. `A6` 은 「보존본 하나」로 고쳤다 | 커밋 순서만으로는 인과가 안 선다(격리 사본 재현) · `CA1-17` 의 근거가 거짓이라 재실행 대조가 **실제로 가능**했다 · `A6` 의 「한 번만」은 §5.8 때문에 착수 전부터 불통과였다 |
dialectic/preflight.md:114:intent.md:356:| 2026-09-11 ⟨기록 정합⟩ | `A5-a` 에 **축 ⓑ 를 더했다** — 워킹트리에 먼저 쓴 사본에서 `A5-b` 의 앵커가 빨개지는지 잰다. **조건 수는 50 그대로다** | ★★ 가 붙은 `A5-b` 에만 음성 대조가 없었다. `A5-d`·`A5-e` 는 대조가 아니라 적용 범위를 좁히는 단서다. 규약 `SKILL.md:605` 가 *"가르는 물음: 「사본을 부수면 정말 빨개지는가.」"* 로 못 박고, 이 회차 `state.md` 의 「걸리는 자리」 4 가 *"음성 대조 자신이 항등식일 수 있다 — 세운 뒤 끄면 정말 빨개지는지 돌려 봐라"* 로 받은 자리 |
dialectic/preflight.md:115:touch/79.txt:9:# 종료값 0 · 표준오류 0 바이트 · 워킹트리 깨끗 · 산출 52 줄 · **4066 바이트**
dialectic/preflight.md:116:touch/79.txt:57:  워킹트리  일치
dialectic/preflight.md:117:dialectic/2-design.md:23:| **판 2** | `A5-c` | 「워킹트리 일치」 줄을 증거로 **안 썼고**, 그 줄이 `--at` 없는 산출에서 **구조적으로 참**이라는 코드 독해 논증이 서며, 그 사실이 게이트에 적혔나 | 제품 코드 · 회차 산출물 전량 · 게이트 |
dialectic/preflight.md:118:dialectic/2-design.md:45:설계 시점(2026-09-12) 워킹트리는 **커밋 안 된 변경 넷**을 들고 있다:
dialectic/preflight.md:119:dialectic/2-design.md:66:| `A5-c` | 회차 산출물·게이트에서 「워킹트리」 문자열 전수(`rg`) · 게이트 파일의 존재와 바이트 | ① *"증거로 **안 썼다**"* 는 인용의 **용법** 판정 ② *"`--at` 없이 부른 **모든 산출**에서 **구조적으로 참**"* 은 **코드 독해 논증** ⟨`CA2-14`⟩ |
dialectic/preflight.md:120:dialectic/2-design.md:78:- **판 2** — 회차 디렉터리와 `docs/gates/effect-confirmation.md` 에 대한 `rg -n "워킹트리"` **전량** · 아래 코드 다섯 자리의 원문 · 같은 줄을 찍는 다른 표면 목록(`crates/pal-cli/src/export.rs:376` · `crates/pal-cli/src/doctor.rs:177` · `crates/pal-query/src/lib.rs:689`).
dialectic/preflight.md:121:dialectic/2-design.md:104:- `crates/pal-cli/src/touch.rs:310-313` — 그 값이 `워킹트리  일치 / 다름` 으로 찍힌다
dialectic/preflight.md:122:dialectic/2-design.md:131:| 판 2 | 코드 다섯 자리 원문 · `rg "워킹트리"` 전량 · 게이트 파일 바이트(또는 부재 산출) | 판 3·4 의 증거 묶음 |
dialectic/preflight.md:123:dialectic/2-design.md:260:- **제품 결함의 수리** — 「워킹트리 일치」의 공허함(`A5-c`) · `크기` 줄(`MS-06`) ·
dialectic/preflight.md:124:dialectic/3-design.md:23:| **판 2** | `A5-c` | 「워킹트리 일치」 줄을 증거로 **안 썼고**, 그 줄이 `--at` 없는 산출에서 **구조적으로 참**이라는 코드 독해 논증이 서며, 그 사실이 게이트에 적혔나 | 제품 코드 · 회차 산출물 전량 · 게이트 |
dialectic/preflight.md:125:dialectic/3-design.md:45:설계 시점(2026-09-12) 워킹트리는 **커밋 안 된 변경 넷**을 들고 있다:
dialectic/preflight.md:126:dialectic/3-design.md:66:| `A5-c` | 회차 산출물·게이트에서 「워킹트리」 문자열 전수(`rg`) · 게이트 파일의 존재와 바이트 | ① *"증거로 **안 썼다**"* 는 인용의 **용법** 판정 ② *"`--at` 없이 부른 **모든 산출**에서 **구조적으로 참**"* 은 **코드 독해 논증** ⟨`CA2-14`⟩ |
dialectic/preflight.md:127:dialectic/3-design.md:78:- **판 2** — 회차 디렉터리와 `docs/gates/effect-confirmation.md` 에 대한 `rg -n "워킹트리"` **전량** · 아래 코드 다섯 자리의 원문 · 같은 줄을 찍는 다른 표면 목록(`crates/pal-cli/src/export.rs:376` · `crates/pal-cli/src/doctor.rs:177` · `crates/pal-query/src/lib.rs:689`).
dialectic/preflight.md:128:dialectic/3-design.md:104:- `crates/pal-cli/src/touch.rs:310-313` — 그 값이 `워킹트리  일치 / 다름` 으로 찍힌다
dialectic/preflight.md:129:dialectic/3-design.md:131:| 판 2 | 코드 다섯 자리 원문 · `rg "워킹트리"` 전량 · 게이트 파일 바이트(또는 부재 산출) | 판 3·4 의 증거 묶음 |
dialectic/preflight.md:130:dialectic/3-design.md:260:- **제품 결함의 수리** — 「워킹트리 일치」의 공허함(`A5-c`) · `크기` 줄(`MS-06`) ·
dialectic/preflight.md:131:dialectic/1-design.md:23:| **판 2** | `A5-c` | 「워킹트리 일치」 줄을 증거로 **안 썼고**, 그 줄이 `--at` 없는 산출에서 **구조적으로 참**이라는 코드 독해 논증이 서며, 그 사실이 게이트에 적혔나 | 제품 코드 · 회차 산출물 전량 · 게이트 |
dialectic/preflight.md:132:dialectic/1-design.md:45:설계 시점(2026-09-12) 워킹트리는 **커밋 안 된 변경 넷**을 들고 있다:
dialectic/preflight.md:133:dialectic/1-design.md:66:| `A5-c` | 회차 산출물·게이트에서 「워킹트리」 문자열 전수(`rg`) · 게이트 파일의 존재와 바이트 | ① *"증거로 **안 썼다**"* 는 인용의 **용법** 판정 ② *"`--at` 없이 부른 **모든 산출**에서 **구조적으로 참**"* 은 **코드 독해 논증** ⟨`CA2-14`⟩ |
effect/rerun-129.txt:3:# 같은 HEAD(`22cf488`) · 같은 워킹트리(깨끗)에서 두 번째 실행. **바이트로 동일하다.**
effect/rerun-129.txt:20:  워킹트리  일치
touch/129.txt:11:# 종료값 0 · 표준오류 0 바이트 · 워킹트리 깨끗 · 산출 21 줄 · **1201 바이트**
touch/129.txt:30:  워킹트리  일치
plan/79-pre.md:91:**이 회차는 그것을 `## 범위 밖` 으로 처분한다** — `A5-c`(워킹트리 일치)·`MS-06`(크기 줄)과
effect/129-readnote.md:15:| `:30` | `워킹트리 일치` | `A5-c` 가 잡아 둔 자리 — 증거로 쓰지 않는다 |
effect/129-readnote.md:26:- 나머지 줄은 전부 근거(스냅숏·대장·워킹트리)이고 이미 아는 값이거나 `MS-06`·`A5-c` 로
effect/judge-prompt.md:185:# 종료값 0 · 표준오류 0 바이트 · 워킹트리 깨끗(`git status --porcelain` 빈 출력)
effect/judge-prompt.md:228:  워킹트리  일치
effect/judge-prompt.md:574:**이 회차는 그것을 `## 범위 밖` 으로 처분한다** — `A5-c`(워킹트리 일치)·`MS-06`(크기 줄)과
effect/judge-prompt.md:649:# 종료값 0 · 표준오류 0 바이트 · 워킹트리 깨끗 · 산출 52 줄 · **4066 바이트**
effect/judge-prompt.md:697:  워킹트리  일치
effect/judge-prompt.md:1287:# 종료값 0 · 표준오류 0 바이트 · 워킹트리 깨끗(`git status --porcelain` 빈 출력)
effect/judge-prompt.md:1330:  워킹트리  일치
effect/judge-prompt.md:1733:# 종료값 0 · 표준오류 0 바이트 · 워킹트리 깨끗 · 산출 21 줄 · **1201 바이트**
effect/judge-prompt.md:1752:  워킹트리  일치
effect/126-readnote.md:10:| `:16` | `check_ledger_pair · palimpsest@a74737b+worktree#076728deab4f` | 심볼 ID 가 **봉인 커밋의 트리**에 대해 섰다. `+worktree` 가 붙은 것은 워킹트리 스냅숏이라는 뜻이고 `:52`(`워킹트리  일치`)와 짝이다 |
effect/126-readnote.md:18:| `:51-53` | `대장 parsed 141 … / 1258 파일` · `2층 심볼 3308 색인됨` · `워킹트리 일치` | `A5-c` 가 잡아 둔 자리 — **더러운 워킹트리에서도 `일치` 를 찍는다.** 지금은 실제로 깨끗하므로 이 줄을 증거로 쓰지 않는다 |
effect/green-129.txt:3:# 잰 때: 2026-09-11T14:48:40Z · 코드는 워킹트리(커밋 직전)
effect/rerun-79.txt:3:# 같은 HEAD(`77dc977`) · 같은 워킹트리(깨끗)에서 같은 명령을 **두 번째로** 돌린 산출이다.
effect/rerun-79.txt:52:  워킹트리  일치
effect/rerun-126.txt:3:# 같은 HEAD(`a74737b`) · 같은 워킹트리(깨끗)에서 같은 명령을 **두 번째로** 돌린 산출이다.
effect/rerun-126.txt:51:  워킹트리  일치
premortem/r3-raw.md:77:- 어떻게 실패하나: `CA1-17` 은 *"워킹트리 다이제스트를 실어 **같은 SHA 로 다시 돌려도 바이트가 안 맞는다**"* 를 근거로 재실행 대조를 포기했다. 실측: `pal touch nodes_of` 두 번의 **전 출력이 `diff` 로 완전히 동일**하다. 게다가 같은 HEAD·같은 워킹트리에서 심볼마다 해시가 다르다 — **그 값은 워킹트리 다이제스트가 아니라 답마다 다른 해시**다
premortem/r3-raw.md:147:- 어떻게 실패하나: `CA1-02` 는 *"인과가 어디에서도 안 섰다"* 를 고치려고 `A5` 를 세웠다. 그러나 `git merge-base --is-ancestor` 가 대는 것은 **커밋 순서**뿐이다. 실제 변경을 워킹트리에서 먼저 다 쓰고, touch 를 먼저 커밋한 뒤 코드를 커밋하면 `A5` 가 통과한다. `A1-c` 는 사전 등록에 **바이트 앵커**를 걸었지만 `A5` 에는 대응하는 앵커가 **없다**
premortem/r3-raw.md:187:- 어떻게 실패하나: `pal touch` 출력의 1 줄은 **빈 줄**이고 다이제스트는 2 줄에 있다. `A2` 는 *"**산출 첫 줄의** 워킹트리 다이제스트"* 를 요구한다
premortem/r2-raw.md:31:- 어떻게 실패하나: 격리 사본에서 실측했다. `check_ledger_pair` 본문에 두 줄을 **커밋하지 않고** 끼운 뒤 touch 를 다시 돌렸더니 `body 1f1846428f55 → f86e990b516b` 로 바뀌었고 **머리는 한 글자도 안 변했다.** 즉 ①워킹트리에 코드 변경을 써 놓고 ②touch 를 돌리고 ③봉인 커밋 ④touch 커밋 ⑤변경 커밋 순으로 쌓으면 `A1`·`A1-b`·`A1-c`·`A5`·`A6` 이 **전부 통과**하면서 touch 는 변경을 이미 본 상태다. `A2` 가 요구하는 머리 넷 어느 것도 이것을 드러내지 않는다
premortem/r2-raw.md:40:### `pal touch` 가 더러운 워킹트리에서 「워킹트리 일치」를 찍는다
premortem/r2-raw.md:41:- 어떻게 실패하나: 같은 사본에서 `git status` 가 ` M xtask/src/main.rs` 인 상태로 touch 를 돌렸는데 근거 상자가 `워킹트리 일치`. `--at <sha>` 를 주면 `다름` 이 난다. 기본 경로는 워킹트리를 읽으므로 `matches_worktree` 가 **구조적으로 언제나 참**이다 — `--at` 을 안 준 모든 산출에서 죽은 가지다. `A2` 가 전 출력을 보존하고 그것이 인과의 증거로 원장에 들어가는데, 사람은 그 줄을 *"touch 를 돌릴 때 고친 것이 없었다"* 로 읽는다
premortem/r2-raw.md:101:- 어떻게 실패하나: 세 산출의 1 줄은 **빈 줄**이고 머리는 2 줄이다. `#<hex>` 는 같은 HEAD·같은 워킹트리에서 **심볼마다 다르다** — 실물은 `coord.rs:296` 의 `"{repo}@{tree}#{symbol.short()}"` 이고 **심볼 ID** 다. 그리고 `pal touch nodes_of` 를 두 번 돌려 `diff` 한 결과 **바이트로 동일**했다. 즉 `CA1-17` 의 근거(*"같은 SHA 로 다시 돌려도 바이트가 안 맞는다"*)가 거짓이고, 그 위에 세운 대체 논리 전체가 무너진다
premortem/r2-raw.md:151:- 어떻게 실패하나: `A1-c` 는 *"`C1`·`C3` 이 실제로 쓴 사전 등록이 그 blob 과 바이트로 같다"* 를 요구한다. 그런데 `C3` 이 보존하라는 것은 **프롬프트 전문**뿐이고, 프롬프트가 넷을 **경로로** 넘기면 전문 안에 사전 등록 바이트가 없다. 판정자가 읽은 것은 그 순간의 워킹트리이고 그것은 사후에 복원되지 않는다. `C1` 은 아예 보존 요구가 없다
observations/identity-after.txt:4:Snapshot  palimpsest@f60df1d+worktree  (워킹트리)
observations/identity-after.txt:8:워킹트리  f60df1d 와 다른 파일 2개  ·  인덱스 신뢰 1266 · 다시 잼 2
observations/issue-66-now.txt:51:Snapshot  palimpsest@49a6b6b+worktree  (워킹트리)
observations/red.md:4:> **착수를 잰 시점**(2026-09-11 · 이 회차 디렉터리를 만들기 전)에 워킹트리는 `8604d62` 와
observations/identity-before.txt:4:Snapshot  palimpsest@f60df1d+worktree  (워킹트리)
observations/identity-before.txt:8:워킹트리  f60df1d 와 같음  ·  인덱스 신뢰 1268 · 다시 잼 0
observations/premortem-artifacts/grade.txt:14:  워킹트리  일치
observations/premortem-artifacts/clone-after.txt:42:  워킹트리  일치
observations/premortem-artifacts/nodes_of2.txt:44:  워킹트리  일치
observations/premortem-artifacts/of_normalized.txt:35:  워킹트리  일치
observations/premortem-artifacts/identity_ceiling.txt:35:  워킹트리  일치
observations/premortem-artifacts/README.md:21:| `nodes_of.txt` · `nodes_of2.txt` | **`PM3-07`** — 같은 HEAD·같은 워킹트리에서 두 번 돌린 산출이 **바이트로 동일**하다. `CA1-17` 의 근거가 거짓임을 보여 `A2-b`(재실행 대조)가 실제로 가능해졌다 |
observations/premortem-artifacts/check_intent_untouched.txt:38:  워킹트리  일치
observations/premortem-artifacts/container_chains.txt:37:  워킹트리  일치
observations/premortem-artifacts/clone-before.txt:42:  워킹트리  일치
observations/premortem-artifacts/nodes_of.txt:44:  워킹트리  일치
observations/premortem-artifacts/check_ledger_pair.txt:38:  워킹트리  일치
```

### 그 줄이 서는 사슬 — 코드 다섯 자리

```
--- crates/pal-cli/src/main.rs:563 ---
        Command::Deviation(a) => plan::deviation(&계획_인자(&a)),
        Command::Touch { name, repo, at, cache_dir, index, intent, binding_max, timing, json } =>
            touch::run(touch::Args { repo: &repo, rev: at.as_deref(), cache_dir, index, intent,
                                     name: &name, binding_max, timing, json }),
        Command::Doctor { repo, at, cache_dir, index, intent, full, sample, install, json } => {
--- crates/pal-cli/src/touch.rs:159 ---
            // **F01 이 이 자리를 값으로 바꿨다.** 워킹트리를 재고 이 답이 선 트리와
            // 대므로 이제 *"모른다"* 가 아니라 *"같다 / 다르다"* 를 적을 수 있다.
            Capable::Present(report.worktree.matches(&report.ledger.snapshot_tree())),
            projection.rebuilding().unwrap_or(false),
            built_for_this,
--- crates/pal-cli/src/touch.rs:310 ---
    }
    println!("  2층       심볼 {} 색인됨", e.projection.symbols_indexed);
    match &e.projection.matches_worktree {
        Capable::Present(v) => println!("  워킹트리  {}", if *v { "일치" } else { "다름" }),
        Capable::NotBuilt { capability } => println!(
--- crates/pal-cli/src/ledger.rs:86-95 ---
    let worktree = repo.worktree_state().context("워킹트리를 읽지 못했다")?;

    let tree = match rev {
        Some(r) => TreeRef::Committed(
            repo.resolve_commit(r).with_context(|| format!("가리키는 것이 없다: {r}"))?,
        ),
        // **기본이 워킹트리다** (옛 F01 §3.2 · [R-06]). 이 제품의 1순위 사용 장면(적시 제시)은
        // 커밋 전 순간에 일어나고, 그 순간에 HEAD 를 보여주면 사용자가 방금 고친 것이
        // 답에서 사라진다. **커밋 축은 `--at` 을 준 사람이 명시적으로 고르는 것이다.**
        None => worktree.tree_ref(),
--- crates/pal-git/src/lib.rs:199-204 ---
    pub fn matches(&self, at: &TreeRef) -> bool {
        match at {
            TreeRef::Worktree { tree_digest, .. } => *tree_digest == self.tree_digest,
            TreeRef::Committed(c) => *c == self.base && self.dirty_paths.is_empty(),
        }
    }
```

### 같은 줄을 찍는 다른 표면

```
crates/pal-core/src/attributes.rs:5://! | `text` 속성 | 워킹트리 파일의 blob 이름을 git 과 같게 계산하려면 **CRLF→LF 를 먼저 되돌려야** 한다 |
crates/pal-core/src/attributes.rs:11://! F01 의 워킹트리 요약이 `gradlew.bat` 하나에서 git 과 다른 blob 이름을 산출했다.
crates/pal-core/src/attributes.rs:12://! 저장소의 blob 은 LF(2843바이트)이고 워킹트리 파일은 CRLF(2937바이트)다 —
crates/pal-core/src/attributes.rs:14://! (clean)에서 그것을 되돌리고, 그것을 안 하면 깨끗한 워킹트리가 dirty 로 보인다.**
crates/pal-core/src/attributes.rs:19://! `pal-git`(워킹트리·트리)의 일이다 — `pal-core` 가 I/O 를 하면 그것이 곧
crates/pal-core/src/envelope.rs:35:    /// 이 답이 선 트리가 워킹트리와 일치하는가.
crates/pal-core/src/envelope.rs:37:    /// **`bool` 이 아니라 [`Capable`] 이다.** 커밋 트리를 읽은 빌드는 워킹트리가 그것과
crates/pal-core/src/envelope.rs:38:    /// 같은지 **모른다** — 알려면 워킹트리 머클이 필요하고 그것은 옛 F01 §3.2 다.
crates/pal-core/src/binding.rs:71:/// 워킹트리에 건 결박에는 **커밋이 없으므로 시각도 없고**, 그것은 모르는 것이 아니다.
crates/pal-core/src/binding.rs:78:    /// 워킹트리에 걸었다 — **커밋이 없으므로 시각도 없다.** 「모른다」가 아니라 「없다」다.
```

## 판 3 — `effect/negative-C2.md` 가 있나

```
$ ls -la .palimpsest/rounds/2026-09-11-effect-confirmation/effect/negative-C2.md
ls: .palimpsest/rounds/2026-09-11-effect-confirmation/effect/negative-C2.md: No such file or directory
```

## 판 4 — 차이 절이 어디 있나(수는 정이 세고 반이 검산한다)

```
effect/129-delta.md:25:## 2. 차이 — 셋이고 **어느 것도 touch 가 원인이 아니다**
effect/129-delta.md:29:| ㉮ | `Ingested::비어_있다()` 를 새로 세웠다 | **컴파일러** — 질의 경로가 인입을 안 부를 때 `Ingested` 하나가 필요했다. `Vec::new()` 를 흩으면 *"안 물었다"* 와 *"물었는데 0"* 이 같은 글자가 된다 | `narrative.rs:110-124` |
effect/129-delta.md:30:| ㉯ | `touch.rs` 의 `QueryCtx` 에 `narrative_unminted: 0` 을 더했다 | **컴파일러**(`E0063`) — `pal touch` 는 인입을 안 부른다 | `touch.rs:153` |
effect/129-delta.md:31:| ㉰ | 세는 낱말이 `센다` → `헤아린다` | **저장소 검사** — 「어색한 표현 부재」가 `narrative.rs:85` 를 잡았다. ㉠ 에 이어 두 번째다 | `narrative.rs:85` |
effect/126-delta.md:20:## 2. 차이 — 넷이고 원인이 둘로 갈린다
effect/126-delta.md:24:| ㉮ | 사전 등록은 `최근에_끝난(&회차들, …)` 이라 적었는데 실제는 **`&끝난`**(`report.md` 로 먼저 거른 목록)을 넘긴다 | **touch 아님** — 사전 등록 §4 ⑵ 가 이미 *"호출자가 `report.md` 존재로 걸러 넘긴다"* 라고 적었다. ⑴ 의 표기가 그것과 어긋났던 것이고 실제는 ⑵ 를 따랐다 | `xtask/src/main.rs:6345-6350` |
effect/126-delta.md:25:| ㉯ | `%ct` 를 고른 근거(`%at` 은 rebase 가 옛 값을 들고 다닌다)를 코드 주석에 적었다 — 사전 등록엔 `%ct` 만 있고 까닭이 없었다 | **touch 아님** — 함수를 쓰다 나온 것 | `xtask/src/main.rs:6432-6433` |
effect/126-delta.md:26:| ㉰ | **새 함수를 `check_ledger_pair` 뒤에 두는 선택의 근거가 바뀌었다** — 「보기 좋다」 → 「이 심볼의 정체성이 선언 순서에 안 걸린다는 것을 확인했다」 | **`touch/126.txt:17`** (`identity ordinal`) → 따라가서 `crates/pal-core/src/coord.rs:246-266` 과 `crates/pal-cli/src/ledger.rs:334` | `xtask/src/main.rs:6407`(둔 자리 자체) |
effect/126-delta.md:27:| ㉱ | **호출자를 찾는 걸음이 없어졌다** — 등록 표를 안 건드리기로 확정했다 | **`touch/126.txt:24`** (`호출자 1 · 피호출자 12`) | `xtask/src/main.rs:643`(**안 건드린** 자리) |
effect/79-delta.md:23:## 2. 차이 — 넷이고 **어느 것도 touch 가 원인이 아니다**
effect/79-delta.md:27:| ㉮ | 세는 메서드 이름이 `센다` → **`헤아린다`** | **저장소 검사** — `cargo xtask check` 의 「어색한 표현 부재」가 내가 새로 넣은 **13 곳**을 잡았다(「~를 낸다」·「~ 센다」·「접다」) | `ledger.rs:337`·`:417` 외 11 곳 |
effect/79-delta.md:28:| ㉯ | 화면에 **「그룹마다 첫 선언은 빠진다」** 한 줄을 더했다 | **시험을 쓰다 나왔다** — `ordinal` 은 0 부터라 그룹의 첫 선언은 ②로 간다. 그러면 「순서에 취약 10」은 겹친 자리의 수가 아니라 **초과분**이다 | `ledger.rs:653-656` |
effect/79-delta.md:29:| ㉰ | 셋째 시험에 **짝**을 달았다(같은 이름 둘이면 ①) | **음성 대조 실측** — 사전 등록 §6 의 예상이 **틀렸다**(아래 §4) | `ledger.rs:934-941` |
effect/79-delta.md:30:| ㉱ | ⑶ 의 호출자 목록을 **`grep` 이 지었다** | ⚠ **`touch/79.txt:23`** 가 `호출자 3` 을 냈고 그 목록에 `defect.rs:326` 이 **없었다** | `defect.rs:326` |
```

## 판 1 — 이슈 본문 셋 인라인 ⟨원격은 변한다. 여기 박아 둔다⟩

### `#126` — OPEN · 「최근 끝난 회차」를 사전순으로 고르는 자리가 하나 더 있다 — check_ledger_pair

```
독립 리뷰 R1(회차 `2026-09-06-terrain-and-completion-scene`)이 낸 것이다.

## 무엇이 걸렸나

`cargo xtask check` 의 「원장 둘 대조」가 판정문에 `최근 끝난 회차 <슬러그> 가 검사에 들었다`
를 산출하는데, 그 「최근」이 **사전순 최대**다. 같은 날짜에 회차 둘이 서면 실제로 나중에 끝난
회차가 아니라 이름이 뒤인 회차가 뽑힌다.

같은 회차의 「어색한 표현 부재」가 이 코드를 본떴다가 **죽은 가지**가 났고(그 회차의 종결
문서가 한 번도 안 재어졌다), 그쪽은 하한 이후 회차를 전부 재는 것으로 고쳤다
(`docs/gates/README.md` 의 선언 「어색한 표현 교정 적용」).

## 왜 지금은 안 뒤집히나

「원장 둘 대조」에서는 **두 회차가 다 대상**이라 판정이 안 바뀐다. 바뀌는 것은 판정문의 그
한 줄뿐이다 — 읽는 사람이 「가장 최근에 끝난 회차」를 틀리게 읽는다.

## 무엇을 고치나

「최근」이 판정에 안 쓰이면 그 낱말을 판정문에서 빼거나, 쓰인다면 커밋 시각으로 고른다.
**사전순은 「최근」의 자가 아니다.**
```

### `#79` — OPEN · I3 — `identity_ceiling` 이 `min` 에 삼켜져 「순서에 취약한 것」과 「그냥 L1 인 것」이 같은 글자가 된다

```
**#66 회차 E 의 분할.** L1 축소가 만든 새 표면이다.

`nodes_of`(`ledger.rs:334`)가 `discriminator.identity_ceiling().min(s.identity)` 만
남기고 **ceiling 자체를 버린다.**

Rust 는 L1 이라 `ExtractGrade::L1.identity() == Ordinal` 이고, 그래서:

- **순서에 취약한 464 건**(`ordinal>0` · I2 참조)
- **그냥 L1 이라 `ordinal` 인 7,156 건**

이 대장에서 **같은 글자**가 된다. 앞의 것은 `impl` 순서를 바꾸면 좌표가 뒤바뀌고
뒤의 것은 아닌데, 산출이 둘을 안 가른다.

[ADR-0013] 이 이름 붙인 형태다 — **약한 것과 취약한 것이 같은 화면이 된다.**

**닫으려면 그 수를 내는 산출 경로가 필요하다.** 손으로 센 수를 판정 표에 실으면
다음 회차가 그것을 못 되짚는다.
```

### `#129` — OPEN · pal query narrative.unbound 가 읽기 트랜잭션에서 쓰려 해 실패한다

```
독립 리뷰 R5 가 금지역으로 잡았다. 회차 `2026-09-06-terrain-and-completion-scene` 의 순서표
§1 「못 잰 것」이 이 실패를 *"이미 결함으로 등록된 자리라 더 안 팠다"* 로 면제했는데 **그
등록이 없었다.**

## 재현

```
$ pal query narrative.unbound
Error: 개체를 남기지 못했다
       의도 저장소 트랜잭션이 실패했다: 읽기로 연 의도 저장소에 쓰려 했다
```

읽기 전용으로 연 의도 저장소에 쓰기를 시도한다. 광고된 질의 하나가 어떤 입력으로도 안 돈다.

## 왜 이슈로 세우나

R5 가 열린·닫힌 이슈 전부를 훑어 이 실패를 지는 것이 **없음**을 보였다. 순서표가 댄 근거는
다른 일(`F09` 표본기)을 지는 이슈였다. **잔여가 경계의 외양을 입은 세 번째 자리**이고,
앞 둘(Kotlin · 규모 선형성)은 같은 회차에서 이미 같은 형태로 잡혔다.
```

## 판 1 — RED·GREEN 산출

```
--- observations/red-129.txt ---
# ㉢ `#129` 의 RED — 작업 저장소에서 관측 ⟨`B1-a`⟩
#
# 잰 때: 2026-09-11T14:26:32Z · HEAD a359909
# ⚠ **`.palimpsest/intent.redb` 가 **있는데도** 실패하는 것을 요구한다** ⟨CA1-01⟩.
#    그 파일: 1351680 바이트 · 9월 8 07:49
#    (추적 안 됨 — `.gitignore` 가 막는 파생물이고 정본은 intent/bindings.jsonl 이다)
#
# 명령: ./target/release/pal query narrative.unbound
--- 표준출력 ---
--- 표준오류 ---
Error: 개체를 남기지 못했다

Caused by:
    의도 저장소 트랜잭션이 실패했다: 읽기로 연 의도 저장소에 쓰려 했다
--- 종료값 ---
rc=1

--- effect/green-129.txt (앞 20 줄) ---
# ㉢ `#129` 의 GREEN — 고친 뒤 ⟨`B1-d`⟩
#
# 잰 때: 2026-09-11T14:48:40Z · 코드는 워킹트리(커밋 직전)
# 명령: ./target/release/pal query narrative.unbound
#
# **종료값 0** · 표준출력 466960 바이트 · 표준오류 **0 바이트**
# ⚠ 전 출력이 466KB 다(미결박 1705 건의 목록). **앞 14 줄과 꼬리 6 줄만 싣는다** —
#   판정에 쓰는 것은 종료값·오류 부재·머리 두 줄이고, 목록 전량은 재실행으로 얻는다.
#
# ⚠ **줄 하나를 여기 적어 둔다**: zsh 의 MULTIOS 때문에 `2>&1 1>/dev/null` 이 표준출력을
#   표준오류인 것처럼 보여 준다. 처음 잴 때 그것에 속아 「오류가 있다」로 읽을 뻔했다.
#   **파일 둘로 갈라 받아야 안 속는다** — 위의 두 바이트 수가 그렇게 잰 것이다.

--- 표준출력 · 앞 14 줄 ---

■ narrative.unbound  

  결박됨 37 · 후보 있음 647 · **미결박 1705**
  이름이 아직 없어 뺀 조각 **4186** — `pal narrative` 를 한 번 돌리면 이름이 섭니다

```

## 판 1 — 시험 전량 요약

```
$ cargo xtask test  (마지막 줄)
시험 통과 — 이 플랫폼에는 안 재지는 것이 없다
$ cargo test --workspace 의 `^test result:` 합산 — 52 줄 · passed 합 1046
$ cargo xtask check — 검사 28/28 통과
```
