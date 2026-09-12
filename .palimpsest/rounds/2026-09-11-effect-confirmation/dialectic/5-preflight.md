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
