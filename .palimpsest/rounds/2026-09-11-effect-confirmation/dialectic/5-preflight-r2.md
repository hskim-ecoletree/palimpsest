# preflight — 판 5 **라운드 2** ⟨설계문 §기계가 먼저 돈다⟩

> 정(正)과 반(反)에게 **같은 바이트**로 준다.

★★ **앞 핀(`dialectic/5-preflight.md`)을 안 덮는다.** 소유자 승격 **칸 9** 가
*"라운드마다 핀을 다시 박는 것"* 을 교훈으로 적었고 이 라운드가 그것을 집행한다.

## 공통 — 기준 핀

```
$ git rev-parse HEAD
8966f5b0954460df91ec3cab84bc079fc32613a2

$ git status --porcelain
?? .palimpsest/rounds/2026-09-11-effect-confirmation/dialectic/5-preflight-r2.md
(위가 비면 워킹트리가 깨끗하다)
```

**기준 커밋은 위 SHA 다. 그 뒤에 생긴 바이트는 증거가 아니다.**

⚠ **이 파일 자신은 목록에 없다** — 커밋이 자기 SHA 를 자기 안에 담을 수 없다.
핀은 아래 마흔 항을 잰 그 나무의 SHA 이고, 그때 추적 안 되던 것은 이 파일 하나다.

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

★ **판 4 라운드 3 의 핀(`dialectic/preflight-r3.md`)과 같은 마흔 항이고 값도 같다.**
두 라운드 사이에 움직인 것은 `docs/plan/03-shortest-path.md`·`02-order.md` 둘이고
그 둘은 핀 목록에 없다(`E6` 의 자리이지 이 판들의 증거가 아니다).

## 판 5 라운드 2 의 의제 — `AT5-14` 하나

종료 판단(`dialectic/5-referee.md`)이 **의제를 하나로 한정했다**:
*"라운드 2 의 의제는 `AT5-14` 하나로 한정된다. 나머지 15 건은 라운드 2 의 대상이 아니다."*

### 그 뒤에 무엇이 섰나 — 처분의 좌표

```
$ grep -n "AT5-14" .palimpsest/rounds/2026-09-11-effect-confirmation/intent.md
371:| 2026-09-12 ⟨정반합 R2 · 승격⟩ | **조건 문면은 한 글자도 안 바꿨다.** 칸 6~11 여섯 — ⑴ 조항 8 **집행 면제** 둘(칸 6·9) ⑵ `#140` ② 문면 교정(칸 7) ⑶ `A6` 을 기록으로 닫고 판정문에 **성격**을 적는다(칸 8) ⑷ `D2` 를 **답 없이 차선책 4 로** 닫고 `D4` 는 **반증**(칸 10) ⑸ `AT5-14` 는 정정문을 옆에 붙이고 원문 보존(칸 11). 원문은 **`## 승격` 이 진다** | 판 3 라운드 2 의 합·판 5 의 합·취합 보고가 각각 올렸다. 조건 수 50 · 문면 `36f0507` 그대로라 §4 의 재승인을 안 부른다 |
558:### 칸 11 ⟨판 5 · `D2` · 합의 채택 `AT5-14` · **금지역**⟩
570:정정은 ⑴ 원장(`출처=정반합` 으로 `AT5-14` 가 이미 실린다) ⑵ 게이트에 적어 **독자가 만나는

$ grep -n "정오표" docs/gates/effect-confirmation.md
211:### 정반합 산출물 — 정오표와 성격 ⟨칸 8·11⟩
214:*"판정문의 사후 수정은 증거 위조다"* 로 못 박는다. 그래서 **정오표를 옆에 붙인다.**
217:#### 정오표 — `dialectic/5-thesis.md` 의 부수 발견 ③ ⟨`AT5-14` · 금지역⟩

$ shasum -a 256 dialectic/5-thesis.md   # 원문이 안 고쳐졌나
6e22d239c35bf743557b49befe19140d6999e7ae374d64982bd0186b0fd925f2  dialectic/5-thesis.md

$ git log --oneline -1 -- .palimpsest/rounds/2026-09-11-effect-confirmation/dialectic/5-thesis.md
e14157c round(effect-confirmation): 판 5 의 정(正)을 보존하고 그것이 잡은 내 거짓 하나를 적는다
```

### `AT5-14` 가 다툰 두 수 — 지금 값

```
$ sed -n "22,24p" docs/adr/0027-the-instrument-must-reach-its-own-floor.md
**코어가 Rust 인데 추출기는 Kotlin·TypeScript 둘뿐이었다.** 그래서 *"이 도구가 자기
자신을 큐레이션한다"* 가 **코퍼스 픽스처 한 파일 위에서만** 섰다.


$ git ls-tree a1493b5 crates/pal-extract/src/ | awk "{print \$NF}"
crates/pal-extract/src/cached.rs
crates/pal-extract/src/classify.rs
crates/pal-extract/src/extractor.rs
crates/pal-extract/src/kotlin.rs
crates/pal-extract/src/lib.rs
crates/pal-extract/src/narrative.rs
crates/pal-extract/src/parse.rs
crates/pal-extract/src/plan.rs
crates/pal-extract/src/recognize.rs
crates/pal-extract/src/rust.rs
crates/pal-extract/src/rust_scopes.rs
crates/pal-extract/src/scopes.rs
crates/pal-extract/src/shell.rs
crates/pal-extract/src/ts_scopes.rs
crates/pal-extract/src/typescript.rs

$ sed -n "24,30p" crates/pal-core/src/language.rs
pub enum Language {
    Kotlin,
    Java,
    JavaScript,
    TypeScript,
    Rust,
}
```
