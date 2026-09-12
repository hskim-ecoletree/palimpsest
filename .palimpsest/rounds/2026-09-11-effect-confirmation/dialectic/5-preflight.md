# preflight — 판 5 `D2` ⟨기계가 먼저 돈다⟩

> 메인이 발동 **전에** 돌려 정과 반에게 **같은 바이트**로 준다.
> 설계문 `dialectic/5-design.md` 의 `### 기계가 먼저 돈다` 가 요구한 일곱을 싣는다.

## 기준 핀

```
$ git rev-parse HEAD
a1493b50fb0185840df7cc1cc973cb0b31377f83

$ git status --porcelain
?? .palimpsest/rounds/2026-09-11-effect-confirmation/dialectic/5-design.md
?? .palimpsest/rounds/2026-09-11-effect-confirmation/dialectic/5-preflight.md
  ← 위가 비었으면 워킹트리가 깨끗하다
```

**핀 뒤에 생긴 바이트는 증거가 아니다.** 판정문이 그것에 기댔으면 그 판은 무효다.

### 증거 파일의 sha256

```
fb7e5a13ad0ee8ab7c8944589deab5551058acfb9b90255c81561f59c69554c5  docs/gates/effect-confirmation.md
4c3b065dd0cfd235689005a06369f1012fd9b95d8a9479d2b7f9552a2c780145  docs/instructions/2026-08-20-owner-direction.md
1e15e45ae8322720fd3f06d3443ea3bb21a39aae00e43609854bb9311fdf63bc  docs/adr/0027-the-instrument-must-reach-its-own-floor.md
47b35dbbc27b17a09eeb53cf3a0b1a395e09d4ed889a1c246fb74d4d9a614e5a  docs/adr/0017-an-instrument-that-cannot-reach-the-floor-is-not-an-empty-population.md
b4dabe8f6e9b76b01be228b48b4f41643ab632e3a2a78103f7de085263ebf055  docs/adr/0025-the-harness-that-reads-the-graph-is-the-same-product.md
685683660b77798545537f570a49c472f49389de590cb64475a11394b5f98ba9  docs/plan/00-goals.md
07569ebad8f1e0af36917ae24872f72299d86464519eecd20d00837b945db8d6  docs/plan/00-risks.md
1aff53c18b0ad300c749c17988186afef96f7c2e30c28b301d672d267507f0c7  AGENTS.md
88f4c35ecb798f9e0613f6bc196400fae10c6d8200b1fdcce3776f7a6084724c  scripts/f12-verify.py
038b06cee91a3d3e78e1505732f791c88b61a121993d1de4a82611026107a3fc  docs/gates/F12.md
ed5fcf60f11dd8b2e6fef15b0719a3596f65748e4ec62846ae3dc0ee7f426dce  corpus/criteria.toml
b352093f2293affb918f5b6111042f3fd2b4e3d632e4844eb50081a15c8b6939  .palimpsest/rounds/2026-09-11-effect-confirmation/intent.md
7be00e1ac33201125a2e86689f10afe1d4e6df6b9ede4835e007a2ca937e6ce9  .palimpsest/rounds/2026-09-11-effect-confirmation/effect/e2-run.txt
0d3bc888c1719d6925697e339e4c38155536a753a1bdd20dab4f2ddcd6b40980  .palimpsest/rounds/2026-09-11-effect-confirmation/observations/issue-66-now.txt
936ea91a39e2a56363fb86862676660cdc06e3f57347fb40b881d05bd6f03331  .palimpsest/rounds/2026-09-11-effect-confirmation/observations/identity-before.txt
90dd52b3c33a099c0483beab38f153ce24ccfca4360c983320cea441d905cc04  .palimpsest/rounds/2026-09-11-effect-confirmation/observations/identity-after.txt
c53ded189fea2571d2ca64cf85d42f525c88eedede662142b8a2eee6a6edeb71  .palimpsest/rounds/2026-09-11-effect-confirmation/touch/79.txt
08f9b674ab7ca8013440a80dbe21a04a6d3315c159b27607be14b6da333be917  .palimpsest/rounds/2026-09-11-effect-confirmation/touch/126.txt
15a2acd99d890996ac3c61eff2c7e7a8f7bcf150311790252cbc0bdc4a73cdcc  .palimpsest/rounds/2026-09-11-effect-confirmation/touch/129.txt
fa3b92377d2783442489b4dc49289638c68c0448b2a21e1c5b3088223be06e23  .palimpsest/rounds/2026-09-11-effect-confirmation/effect/79-delta.md
23bf96bea7cd7d2772a4ef06aad39e1ac3e1865ed0520bbbeebf74015217e59f  .palimpsest/rounds/2026-09-11-effect-confirmation/effect/126-delta.md
5254707a06f4c9dc7859e0020657601559af0d75ba69ebb34dbbe612e2dbb93f  .palimpsest/rounds/2026-09-11-effect-confirmation/effect/129-delta.md
```

## 1. `gh issue view 66 --comments` — 전문

```
author:	hskim-ecoletree
association:	owner
edited:	false
status:	none
--
**남긴다 — 2026-08-18 재고 처분 회차가 판정했다.**

셋을 물었다: ① 낡았는가 ② 방향을 잘못 가리키는가 ③ 흡수됐는가. **셋 다 아니오.**

- **자족적이다.** 이 이슈는 측정치를 본문에 담고 있고, 삭제 대상(`docs/plan/features/` · `DESIGN.md` · `WHITEPAPER.md` · `how-it-works.md`)을 **하나도 안 가리킨다** (실측: 결함 이슈 9 건 전부 0 건). 가리키는 것은 게이트(동결)·ADR(남김)·`00-risks.md`(남김)뿐이라 전부 살아 있다.
- **그래프는 하네스의 입력이다.** [ADR-0025](https://github.com/hskim-ecoletree/palimpsest/blob/main/docs/adr/0025-the-harness-that-reads-the-graph-is-the-same-product.md) 가 방향을 튼 것은 *"그래프를 읽는 하네스가 같은 제품이다"* 이지 그래프를 버린 것이 아니다. 추출·결박·낡음의 품질 결함은 그대로 유효하다.

⚠ **다만 프론티어의 구성이 바뀌었다.** 에픽 12 와 작업 항목 하나를 닫아서 지금 착수 가능한 것은 **전부 「고칠 것」**이다. 새 지형(「만들 것」)은 다음 회차가 세운다.

판정 전문: [`docs/gates/inventory-disposal.md`](https://github.com/hskim-ecoletree/palimpsest/blob/main/docs/gates/inventory-disposal.md)

--
author:	hskim-ecoletree
association:	owner
edited:	false
status:	none
--
## 관측 — 세 회차 연속 「능력 부재」로 나왔고 **수가 커지고 있다** (회차 `2026-08-19-finding-records`)

이 회차도 §11 조건 4(결박·그래프 갱신)를 시도했고 **또 여기로 왔다.**
산출은 [`effect/binding-attempt.txt`](https://github.com/hskim-ecoletree/palimpsest/blob/main/.palimpsest/rounds/2026-08-19-finding-records/effect/binding-attempt.txt).

⚠ **수를 여기 안 적는다** — 커밋마다 변하고, 이 회차가 「손으로 벤 거울」에 다섯 번
물렸다. **돌려라:**

```bash
cargo run -q -p pal-cli -- ledger      # 결박 불가 언어와 파일
cargo run -q -p pal-cli -- narrative   # 문서 조각과 결박된 수
```

### 그날 본 것의 모양 (수가 아니라 **꼴**)

- **결박 불가 언어가 일곱**이고 그중 **Markdown 과 Rust 가 압도적**이다 —
  이 저장소의 문서와 코어가 전부 거기 있다
- **결박 가능한 것은 TypeScript 한 파일**뿐이다. 코퍼스 픽스처다
- `pal narrative` 는 **조각이 이천이 넘는데 결박됨 0** 을 낸다
- **회차마다 커진다** — 회차가 산출을 더할수록 미결박이 늘어난다.
  세 회차가 각각 「능력 부재」를 적었고 그때마다 수가 컸다

### 왜 이것이 다음인가

회차 `2026-08-19-finding-records` 의 종료 보고가 스스로 적은 미흡이다 —
발견의 **대부분이 하네스 자기 자신과 회차 기록**을 향했고 **제품의 원 의도는 한 걸음도
안 나갔다.** 그리고 이 회차도 「부르는 자리」를 전부 `grep` 으로 셌다.

★ AGENTS.md 가 정한 것: *"`grep` 을 집으려는 순간에 그래프에 먼저 묻는다.
`grep` 은 문자열을 맞히고 그래프는 **관계를 안다**."* **그 문장이 이 저장소에서
아직 한 번도 참이 된 적 없다** — Markdown·Rust 추출기가 서야 처음 참이 된다.

### 이 이슈가 묻는 것은 그대로다

1. **이 도구가 자기 자신을 큐레이션할 수 있어야 하는가**
2. 그렇다면 **Rust 추출기**인가, 아니면 ⑧을 **다른 저장소의 실사용**으로 다시 정의하는가

⚠ 회차 `2026-08-19-finding-records` 는 **Markdown 도 함께 걸린다**는 것을 더한다 —
`pal narrative` 가 문서를 다루는데 그 문서들이 전부 미결박이라, **문서 결박(#69)과
같은 벽**이다.
--
```

## 2. `docs/gates/effect-confirmation.md` — 절 목록과 `## 효과` 의 하위 절

```
$ grep -n '^## \|^### ' docs/gates/effect-confirmation.md
7:## 합격선
20:## 판정
34:## 효과
40:### 이 회차의 산출이 실제로 돌았다
49:### `#66` 의 낡은 전제 둘 — 정정문 ⟨`D1`⟩
62:### `#66` 의 물음 2 — 두 갈래의 현재 상태 ⟨`D3`⟩
75:### `#66` 을 **현재형으로** 인용하는 다섯 자리의 처분 ⟨`D4-a`⟩
98:### 차이마다 — 방향과 「touch 가 말한 것이 참이었나」 ⟨`E2`·`C4`⟩
125:## 범위 밖

$ wc -l docs/gates/effect-confirmation.md
     128
```

⚠ **`D2` 절이 없다.** `## 효과` 의 하위 절은 다섯이고 그중 `D1`·`D3`·`D4-a` 는 이름으로 있다.
그 부재가 무엇을 뜻하는지는 **판이 정한다** — 이 preflight 는 사실만 적는다.

전문은 저장소에 있다(위 sha256 표가 바이트를 못 박는다).

## 3. `자기 자신을 큐레이션 | 자기적용 | 자기 저장소` 전수

⚠ **앞 판 넷의 산출(`dialectic/[1-4]-*` · `dialectic/r1-*` · `r2-*`)은 잘라냈다.**

```
$ grep -rn '자기 자신을 큐레이션\|자기적용\|자기 저장소' .palimpsest/rounds/2026-09-11-effect-confirmation docs corpus AGENTS.md
.palimpsest/rounds/2026-09-11-effect-confirmation/intent.md:244:- [ ] **D2** ⟨정반합⟩ `#66` 의 물음 1 — *"이 도구가 자기 자신을 큐레이션할 수 있어야 하는가"* — 에 답했다. ⚠ **규범 명제라 측정이 혼자 못 답한다** — 이 회차의 측정을 근거로 대고 판정은 정반합이 한다
.palimpsest/rounds/2026-09-11-effect-confirmation/intent.md:288:- **R-19 — 자기 저장소 자기적용의 편향.** 이 회차는 **「이 저장소에서 섬」까지만** 주장한다.
.palimpsest/rounds/2026-09-11-effect-confirmation/dialectic/5-design.md:1:# 토론 설계 — `#66` 의 물음 1(*"이 도구가 자기 자신을 큐레이션할 수 있어야 하는가"*)에 답했나
.palimpsest/rounds/2026-09-11-effect-confirmation/dialectic/5-design.md:15:> **규범 명제 「이 도구(palimpsest)는 자기 저장소를 스스로 큐레이션할 수 있어야 한다」에
.palimpsest/rounds/2026-09-11-effect-confirmation/dialectic/5-design.md:80:3. `rg -n '자기 자신을 큐레이션|자기적용|자기 저장소' .palimpsest/rounds/2026-09-11-effect-confirmation docs corpus AGENTS.md` **전량**.
.palimpsest/rounds/2026-09-11-effect-confirmation/dialectic/5-design.md:123:3. **물음의 치환.** *"할 수 **있는가**"*(능력) · *"자기 저장소 **에서** 쓰나"*(사용) ·
.palimpsest/rounds/2026-09-11-effect-confirmation/dialectic/5-design.md:126:   *"자기 자신을 큐레이션할 수 **있는가**"* 로 인용한다. 정·반 둘 다 이 치환을 감시한다.
.palimpsest/rounds/2026-09-11-effect-confirmation/dialectic/5-design.md:138:  **그 프로젝트**의 온톨로지를 만든다. **자기 저장소는 첫 시험대일 뿐이다**"*
.palimpsest/rounds/2026-09-11-effect-confirmation/dialectic/5-design.md:139:- `docs/plan/00-risks.md` R-19(`:407-410`) — *"자기적용은 개발용 피드백 루프이지 **증거가
.palimpsest/rounds/2026-09-11-effect-confirmation/dialectic/5-design.md:156:*"이 도구는 자기 저장소를 큐레이션할 수 있어야 한다"* 가 **이 저장소의 규범으로 선다.**
.palimpsest/rounds/2026-09-11-effect-confirmation/dialectic/5-design.md:167:자기 큐레이션은 **요구가 아니다** — 자기 저장소는 시험대일 뿐이고 `⑧`이 그 위에 서야 할
.palimpsest/rounds/2026-09-11-effect-confirmation/dialectic/5-design.md:368:- **R-19 를 없애는 것.** 자기 저장소 자기적용의 편향은 이 회차가 **원리상 못 없앤다.**
.palimpsest/rounds/2026-09-11-effect-confirmation/premortem/r1-raw.md:168:- 어떻게 실패하나: D3 은 `#66` 의 물음 2 — *"Rust 추출기인가, 아니면 ⑧을 다른 저장소의 실사용으로 다시 정의하는가"* — 에 답하라 한다. 앞 갈래는 이미 섰다(`#130`·ADR-0027, 결박 가능 파일 Rust 140). 뒤 갈래는 이 회차가 `## 범위 밖` 에 *"남의 저장소 실사용 — 코퍼스 저장소에는 「실제 작업」이 없어 합격선 문면을 벗어난다"* 로 스스로 뺐다. 그러면 D3 은 측정이 아니라 선언으로만 채워지고, D2(*"자기 자신을 큐레이션할 수 있어야 하는가"*)는 애초에 규범 명제라 측정이 못 답한다
docs/agent-laziness-unlazy-comparison-and-implementation-plan.md:179:### 4.5 자기 저장소 하네스에 대한 그래프의 관측 범위
docs/overview.md:716:세 번째가 가장 심각하다. 이 제품이 지적한 문제를 **자기 저장소의 판정 기록이 그대로 저지르고
docs/sunset.toml:15:# 소유자가 정했다: **`pal` 이 자기 저장소에서 회차를 한 번 돌렸을 때.**
docs/instructions/2026-08-20-owner-direction.md:7:> *"자기 저장소 실사용이 능력 부재 위에 섰다."* 회차 기록은
docs/instructions/2026-08-20-owner-direction.md:16:| **§5** | 외부 코퍼스 | **`rust-lang/cargo`** — 자기 저장소 편향(R-19)을 피한다 |
docs/instructions/2026-08-20-owner-direction.md:30:### 왜 갱신하나 — 능력 부재가 자기적용을 막았다
docs/instructions/2026-08-20-owner-direction.md:42:[R-19](../plan/00-risks.md) 는 자기적용을 *"**편향된** 표본"* 이라 불렀는데,
docs/instructions/2026-08-20-owner-direction.md:86:선언의 **22%**(자기 저장소 2,778 중 629)가 시험 안이라 **분모가 부푼다.**
docs/instructions/2026-08-20-owner-direction.md:106:자기 저장소 117 파일만으로는 [R-19](../plan/00-risks.md) 의 편향을 못 피하고,
docs/instructions/2026-08-10-owner-direction.md:70:| U18-a | palimpsest 자신의 방법론 | **정합** | `plan README §7` 진행 규칙 | 게이트 기록·ADR 종료 시 발행·사전 등록이 이미 방법론이다. 빠진 것은 **브랜치·검토 규약**과 **자기적용** |
docs/gates/F11-touch.md:52:| **⑧ 자기 저장소 결박** | ≥ **1** | **1** | **통과** ⚠ (§10) |
docs/gates/F11-touch.md:311:## 10. ⑧ 자기 저장소 — ⚠ **편의 표본이자 능력 부재**
docs/gates/F11-touch.md:321:그러므로 *"자기 저장소에서 실사용"* 은 이 빌드에서 **코퍼스 픽스처 한 파일 위에서만**
docs/gates/F11-touch.md:322:선다. [R-19]가 *"자기 저장소 자기적용은 편향된 표본"* 이라고 이미 적었는데,
docs/gates/F11-touch.md:418:| **자기 저장소에 결박 가능한 좌표가 7개** — 코어가 Rust다 | **F16 / 추출기 확장** |
docs/gates/F10.md:465:**가. 자기 저장소로는 아무것도 못 잰다.** 문서 §3.5 가 *"첫 소비자는 이 저장소 자신"*
docs/gates/rust-extractor.md:207:기준 SHA 명시: 자기 저장소 `56926aa` · cargo `514c56dd`.
docs/gates/rust-extractor.md:387:| 자기 저장소 표식 (변한다) | `cargo run -q -p pal-extract --example count_marked -- . rs` |
docs/gates/inventory-disposal.md:146:| `#66` 자기 저장소 실사용 | `needs-triage` | **남김** | 같음. `gates/F11-touch`·`ADR-0017`·`00-risks`. ★ 이 회차의 **능력 부재 ①②를 지는 이슈**다 |
docs/gates/inventory-disposal.md:274:#### 결박 — 하네스가 자기 저장소에서 돈 첫 관측
docs/gates/effect-confirmation.md:69:| ⓐ **Rust 추출기** | **섰다.** 자기 저장소가 L1 로 파싱되고 심볼이 3332 개 선다 | `pal ledger` | `Rust L1 ordinal 141 파일` · `정체성 심볼 3332` |
docs/gates/effect-confirmation.md:83:| `docs/gates/F12.md:398` | 같음(결함 표의 행선지 칸) | **안 고친다** | *"자기 적용이 구조적으로 못 선다 — 코어가 Rust 다 → **[#66]**(소유자에게 묻는 중)"* — **「구조적으로 못 선다」가 거짓이다.** 자기 저장소가 파싱되고 심볼 3332 이 선다 |
docs/gates/F12.md:315:**⚠ R-19**: 자기 저장소는 편의 표본이다. 성립했더라도 ①의 근거가 아니다.
docs/gates/F24-install-distribute.md:321:| **이 빌드가 심볼을 내는 언어에 Rust 가 없다** | `pal symbols crates/pal-core/src/language.rs` → *"확장자 `.rs` 를 언어로 알지 못한다 — 아는 것은 Kotlin · Java · JavaScript · TypeScript 넷이다"*. `crates/pal-core/src/language.rs:14` 의 `enum Language` 가 넷이다. **자기 저장소로는 못 잰다** |
docs/adr/0027-the-instrument-must-reach-its-own-floor.md:14:`[f11.pass]` ⑧ 은 *"자기 저장소 실사용 · 결박 ≥ 1"* 을 요구했고 그 하한은 넘었다.
docs/adr/0027-the-instrument-must-reach-its-own-floor.md:25:[R-19](../plan/00-risks.md) 는 자기적용을 *"**편향된** 표본"* 이라 불렀다. 그러나 여기서는
docs/adr/0027-the-instrument-must-reach-its-own-floor.md:41:그것을 **갱신한다.** 별도 「자기적용 전용 층」을 두는 길도 있었으나 안 골랐다 —
docs/adr/0027-the-instrument-must-reach-its-own-floor.md:110:- 자기 저장소가 심볼로 색인된다. `symbol.resolve`·`symbol.contains` 가
docs/adr/0027-the-instrument-must-reach-its-own-floor.md:134:| Rust 를 자기적용 전용 층으로 | 능력표·등급·레지스트리가 두 모집단을 갖는다 — 갈림이 곧 drift 다 |
docs/adr/0025-the-harness-that-reads-the-graph-is-the-same-product.md:24:세 번째가 특히 이 제품 자신의 논지에 반한다. [백서 §1](../../WHITEPAPER.md) 이 *"낡은 문서는 거짓 신호가 된다"* 를 논증하는데, **자기 저장소의 판정 기록이 정확히 그 상태**다.
docs/plan/disposal-map.md:251:| 7 | **D28** | "**`provider`는 프로젝트가 자기 저장소에 두는 실행 가능한 어댑터이고, 코어는 그 산물의 스키마만 안다.**" 산물은 `observed`, "코어는 실행하지 않는다" | docs/DESIGN.md:192 · 920 · 924 | **B** — F16·F21 |
docs/plan/00-stack.md:461:| 릴리스 | **P2까지 릴리스 아티팩트를 만들지 않는다.** 자기 저장소와 코퍼스에서만 돈다 |
docs/plan/00-risks.md:27:| [R-19](#r-19) | 자기 저장소 자기적용은 편향된 표본이다 | 영구 | — |
docs/plan/00-risks.md:245:**판정** — F10. 자기 저장소 + 코퍼스에서 해소율 측정.
docs/plan/00-risks.md:408:### 자기 저장소 자기적용은 편향된 표본이다 · **영구**
docs/plan/00-risks.md:410:첫 실사용을 이 저장소 자신으로 하는데(문서를 코드 좌표에 걸기), palimpsest 저장소는 문서 대 코드 비율이 극단적이라 서술물 인입의 성적이 실제 프로젝트와 다를 것이 거의 확실하다. **편의 표본임을 게이트 기록에 명시한다.** 자기적용은 개발용 피드백 루프이지 증거가 아니다.
docs/plan/00-risks.md:621:**판정** — F23. 자기적용과 코퍼스에서 충돌률을 잰다.
docs/plan/00-goals.md:316:| **자기 자신의 온톨로지를 만드는 도구** | 설치된 **그 프로젝트**의 온톨로지를 만든다. 자기 저장소는 첫 시험대일 뿐이다 |
corpus/criteria.toml:7467:     가 실측했다). 자기 저장소만으로 판정하지 않는다
corpus/criteria.toml:7496:| 자기 저장소 인입 + 편의 표본 명시 | **없다**(`input_quality`) |
corpus/criteria.toml:7784:**전부 실물이다. 우리가 만든 문서가 하나도 없다.** 그리고 **자기 저장소만으로 판정하지
corpus/criteria.toml:8518:# ② 자기 저장소 인입 — **편의 표본임을 명시한다**
corpus/criteria.toml:8521:**★ 자기 저장소는 증거가 아니라 피드백 루프다**([R-19] · 문서 §3.5).
corpus/criteria.toml:9650:| 자기 저장소 실사용 | **없다** | **⑧** |
corpus/criteria.toml:10034:# ⑧ 자기 저장소 실사용 — ⚠ **편의 표본이다** (R-19)
corpus/criteria.toml:10037:[F11 §7] 의 마지막 줄이다 — *"자기 저장소에서 실사용"*.
corpus/criteria.toml:10041:⚠ **[R-19] 가 이미 이름 붙였다** — *"자기 저장소 자기적용은 편향된 표본이다.
corpus/criteria.toml:10042:자기적용은 개발용 피드백 루프이지 증거가 아니다."*
corpus/criteria.toml:10808:**⚠ R-19**: 자기 저장소는 **편의 표본**이다. 성립했더라도 ①의 근거가 아니다.
corpus/criteria.toml:11118:  2. **자기 저장소 표식 실측** — 표식 주석 마디 **130** · 선언 **2,778**
corpus/criteria.toml:11158:자기 저장소 `56926aa`. 표본이 될 파일을 이 회차가 고치므로, SHA 를 안 박으면
corpus/criteria.toml:11221:**③ 자기 저장소에서 결박이 실제로 선다.**
corpus/tasks/rust-recall-sample.tsv:28:# ★ **자기 저장소 기준 SHA 는 `56926aa` 다**(회차 E 착수 커밋). 이 표본은 cargo 만
corpus/tasks/rust-recall-sample.tsv:29:#   담지만, 같은 회차의 결박 측정이 **자기 저장소**에서 나오고 그쪽 파일들은
corpus/tasks/rust-recall-sample.tsv:114:#      **자기 저장소에서는 다르다** — 선언 2,778 중 **629(22%)** 가 그 안이다.
corpus/tasks/rust-recall-sample.tsv:115:#      그러므로 이 규칙의 음성 대조는 **cargo 표본이 아니라 자기 저장소**가 진다
```

## 4. 권위 계열 — 경로와 sha256 은 위 표가 진다

전문을 여기 복사하지 않는다(합쳐서 수만 줄이다). **바이트는 위 sha256 표가 못 박았고
정·반 둘 다 같은 핀에서 읽는다.** 아래는 설계문이 지목한 자리의 좌표다.

| 문서 | 지목된 자리 |
|---|---|
| `docs/instructions/2026-08-20-owner-direction.md` | 전문 |
| `docs/adr/0027-the-instrument-must-reach-its-own-floor.md` | 전문 — 특히 `## 결정` 과 대안 표 |
| `docs/adr/0017-an-instrument-that-cannot-reach-the-floor-is-not-an-empty-population.md` | 전문 |
| `docs/adr/0025-the-harness-that-reads-the-graph-is-the-same-product.md` | 전문 |
| `docs/plan/00-goals.md` | §0.1 · §4 의 비목표 표 |
| `docs/plan/00-risks.md` | R-19 |
| `AGENTS.md` | `:7` |

### 설계문이 「반대 방향 근거」로 지목한 넷 — 원문 그대로

```
$ grep -n '자기 자신의 온톨로지' docs/plan/00-goals.md
316:| **자기 자신의 온톨로지를 만드는 도구** | 설치된 **그 프로젝트**의 온톨로지를 만든다. 자기 저장소는 첫 시험대일 뿐이다 |

$ grep -n -A 4 'R-19' docs/plan/00-risks.md | head -20
27:| [R-19](#r-19) | 자기 저장소 자기적용은 편향된 표본이다 | 영구 | — |
28-| [R-20](#r-20) | 평가 코퍼스가 아직 확보되지 않았다 | 높음 | P0-preflight |
29-| [R-21](#r-21) | **의도가 파생층에 섞여 있다 — 캐시 폐기가 승인 노동을 지운다** | 치명 | F05 (지금 결정) |
30-| [R-22](#r-22) | **정규화 등급이 오르면 좌표가 전면 이동한다** | 치명 | F03 (지금 결정) |
31-| [R-23](#r-23) | preflight이 재는 단가가 실제 단가를 대표하지 못한다 | 높음 | P0-preflight 설계 시점 |
--
407:## R-19 {#r-19}
408-### 자기 저장소 자기적용은 편향된 표본이다 · **영구**
409-
410-첫 실사용을 이 저장소 자신으로 하는데(문서를 코드 좌표에 걸기), palimpsest 저장소는 문서 대 코드 비율이 극단적이라 서술물 인입의 성적이 실제 프로젝트와 다를 것이 거의 확실하다. **편의 표본임을 게이트 기록에 명시한다.** 자기적용은 개발용 피드백 루프이지 증거가 아니다.
411-

$ sed -n '7p' AGENTS.md
★ **이 역할들은 단계에 배치되지 않는다 — 필요하다고 판단되는 어느 자리에서든 부르는 도구다.** 가장 자주 걸리는 자리 하나: **`grep` 을 집으려는 순간에 그래프에 먼저 묻는다.** `grep` 은 문자열을 맞히고 그래프는 **관계를 안다** — 부르는 자리 · 흘러드는 데이터 · 걸린 결정 · 낡은 것. 구현 계획을 세울 때 · 리뷰할 때 · 질문을 만들 때 · 의도의 진위를 볼 때 전부 같은 자리다.
```

## 5. 이 회차의 측정 계열 — 경로와 sha256 은 위 표가 진다

`effect/e2-run.txt` · `observations/issue-66-now.txt` · `observations/identity-before.txt` ·
`observations/identity-after.txt` · `touch/{79,126,129}.txt` · `effect/{79,126,129}-delta.md`.
**전문을 여기 복사하지 않는다** — 위 sha256 표가 바이트를 못 박았고 정·반 둘 다 같은 핀에서
읽는다. ⚠ `observations/identity-group.txt` 는 핀 표에 없다(㉲ 뒤에 생겼다) — **증거로 쓰려면
그 사실을 적어야 한다.**

## 6. `intent.md` 의 지목된 절

```
$ grep -n '^## ' .palimpsest/rounds/2026-09-11-effect-confirmation/intent.md
5:## 원문
38:## 목적 기여
67:## 상한
83:## 금지역
95:## 재는 자리 셋
121:## 순서 — 못 박는다 ⟨사전부검 R1 `PM1-17` · **R2·R3 가 근거를 정정했다**⟩
162:## 완수 조건
260:## 차선책
286:## 범위 밖
327:## 개정
368:## 승인
381:## 승격
495:## 착수 시점 관측

$ grep -n 'Markdown\|MS-08' .palimpsest/rounds/2026-09-11-effect-confirmation/intent.md | head
302:- **Markdown 결박(`#69`).** 이 저장소 문서가 미결박인 것은 이 회차가 안 고친다.
315:- **`pal touch` 의 「호출자 N」이 파일 간 미해소 몫을 표시 없이 뺀다** ⟨`MS-08` · 루프 ㉠⟩.
```

## 7. `#66` 을 현재형으로 되묻는 문면 다섯 — 원문 그대로

⚠ **`D4-a` 의 처분은 이 판의 것이 아니다.** 그 문면이 저장소에 살아 있다는 **사실만** 준다.

```
$ sed -n '556,570p' scripts/f12-verify.py
    걸린 = sum(1 for row in v["states"] for s in row if s.get("state") == "bound")
    # ⚠ **이 문면의 전제가 2026-09-12 에 깨졌다** — 회차 `2026-09-11-effect-confirmation`
    #   의 `D1` 이 명령으로 다시 쟀다. 앞 판은 *"추출기는 Kotlin·Java·JS·TS 넷뿐이다"* 를
    #   **현재형으로** 주장했는데 Rust 추출기가 그 뒤에 들어와 지금은 **다섯**이다.
    #   **돌아가는 코드가 거짓을 주장하는 자리라 고쳤다** — 회차의 금지역
    #   「사실이 아닌 것을 사실로」에 든다. ⑧ 합산 여부는 F12 판정 시점의 결정이므로
    #   그대로 두고, **그 결정이 선 전제가 지금은 다르다는 것**을 같은 줄에 적는다.
    skip("⑪ 자기 적용", f"계획 항목 **{항목}** 이 실재하고 좌표로 걸린 것이 **{걸린}** — "
                        f"⚠ **모집단 0 이 아니다.** 항목이 지목하는 것은 Rust 식별자이고, "
                        f"F12 판정 시점에는 추출기가 Kotlin·Java·JS·TS 넷이라 "
                        f"[ADR-0017] 의 **「자가 짧다」**로 ⑧에 합산하지 않았다. "
                        f"⚠ **그 전제는 지금 거짓이다** — Rust 추출기가 들어와 **다섯**이다"
                        f"(`pal symbols` 의 오류 문면이 그 목록을 적는다). 지금 값과 처분은 "
                        f"`docs/gates/effect-confirmation.md` 가 진다 (#66)")


$ sed -n '312p;398p' docs/gates/F12.md
⚠ **처분은 이 게이트가 정하지 않는다** — [#66] 이 소유자에게 묻고 있다
| **자기 적용이 구조적으로 못 선다** — 코어가 Rust 다 | **[#66]** (소유자에게 묻는 중) |

$ sed -n '10530p;10886p' corpus/criteria.toml
  · **이 자리의 처분은 [#66] 이 소유자에게 묻고 있다** — 이 절이 정하지 않는다
     **Rust 추출기가 선 뒤**의 문장이고, 그 처분은 [#66] 이 소유자에게 묻고 있다
```
