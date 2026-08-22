# 교대용 상태 — 에이전트 게으름

> 착수 `e45e822` · 브랜치 `round/agent-laziness`
> ⚠⚠ **이 파일에 수를 적지 마라.** 라운드 번호도 리뷰 횟수도 조건 수도 —
> 독립 리뷰가 **세 번 연달아** 「낡았다」로 잡았고, 세 번째는 *"매 라운드 갱신한다"* 를
> 자기 머리에 적은 채로 낡았다. **갱신 규율로는 안 막힌다. 수를 안 드는 것이 막는다.**
> 지금 상태는 아래 명령이 낸다.
> **이 파일과 [`intent.md`](intent.md) 전문이 다음 컨텍스트가 받는 전부다** (§5 교대).
> 앞 컨텍스트의 산출물·대화·라운드 서사를 시드로 받지 마라 — 의도가 변질되는 기제는
> 쪼개는 것 자체가 아니라 **다음 걸음의 입력이 원 목표가 아니라 직전 걸음의 산출물일 때**다.

## 어디까지 왔나

| 단계 | 상태 | 세는 자리 |
|---|---|---|
| 인터뷰 | **소진** | 상한 4 · [`intent.md`](intent.md) 머리 |
| 사전부검 | **소진** | 상한 3 · [`premortem/`](premortem/) |
| 승인 | **받았다** (2026-08-22) | [`intent.md`](intent.md) `## 승격` |
| 루프 | 도는 중 | `git log --oneline e45e822..HEAD` — `[R<n>]` 태그 |
| 독립 리뷰 | 도는 중 | **상한 8** · `ls review/*-raw.md` |

```bash
# 지금 상태를 다시 재는 명령 — 이 파일의 수를 믿지 말고 돌려라
python3 .claude/skills/round/bin/record.py conditions \
  .palimpsest/rounds/2026-08-22-agent-laziness/intent.md
python3 .claude/skills/round/bin/dashboard.py e45e822 \
  .palimpsest/rounds/2026-08-22-agent-laziness/intent.md HEAD
cargo xtask check
```

## 만든 것

| | |
|---|---|
| **21 번째 검사 「원장 둘 대조」** | 게이트 표준 표 ↔ `intent.md` 상자를 **양방향 집합 같음**으로 댄다 |
| **조건 파서 한 자리** | `record.py` 의 `조건들()`·`게이트판정()`. 계기판과 `xtask` 가 그것을 부른다 |
| **전사** | 죽어 있던 상자 90 개를 켰다 — 판정 내용은 안 바뀌었다 |
| **레코드 스키마 3** | 열림 축(`상태`·`닫은커밋`) + 계기판 **⑨ 해악 게이트** |
| **Stop 훅 관측** | 양성 대조와 함께. 정책은 안 세운다 |
| **unlazy 문면 전수 대조** | 규범 문장 전수 → 가져올 다섯 선정. 기입은 [#88] |

**라운드별로 무엇을 했는지는 커밋 메시지가 진다** — `git log e45e822..HEAD`.

## 21 번째 검사가 무엇을 하나 — 이 회차의 중심

모집단은 **회차 디렉터리 전부**이고, 게이트 본문의
`.palimpsest/rounds/<회차>/intent.md` 로 **역인덱스**를 걸어 짝을 찾는다.

| | |
|---|---|
| 대조 | ID 집합 **양방향** + 조건마다 상자와 판정 태그 |
| 검산 | 게이트가 적은 수 ↔ ID 목록 길이 |
| 형식 이전 | 표준 표가 없는 게이트는 **검사 밖.** 오류가 아니다 |
| 하한 | **끝난 회차 중 가장 최근 것**이 검사에 들었는가 |

## 실패한 접근 — 같은 벽에 다시 부딪히지 마라

1. **`^- \[` 로 상자를 세지 마라.** 코드펜스 안의 형식 예시와 `## 범위 밖` 불릿까지 센다
   (실측 `3/4`). 열림 쪽만 고치면 분모가 깨진다 (`2/3`). `record.py conditions` 를 불러라.
2. **절을 볼 때 깊이를 봐라.** 조건은 `## 완수 조건` 아래 `###` 하위 절에 산다.
3. **`intent.md` 에 새 frontmatter 를 만들지 마라.** 역인덱스는 **게이트 → intent** 한 방향이다.
   반대로 걸면 회차 내내 죽은 링크 검사가 빨개진다.
4. **게이트 `## 판정` 을 정규식으로 통째로 훑지 마라.** 잡음이 지배한다.
5. **기존 게이트 표를 갈아엎지 마라** — 표준 표를 위에 더한다.
6. **원 반환문의 기각 표 헤더는 `| # |` 로 시작해라.**
7. **레코드 좌표를 자동 추출하지 마라.**
8. **문서 안에 `](../../..` 꼴 문자열을 코드 스팬으로도 두지 마라** ([#87] 이 진다).
9. ★ **게이트 `## 판정` 절의 표 행을 전부 모으지 마라.** 헤더 뒤의 **잇닿은** 행만이
   그 표다 — `rust-extractor.md` 는 한 절에 표가 셋이다. 검산 줄도 표 **바로 뒤**에서 찾아라.
10. ★ **짝짓기를 마크다운 링크로만 하지 마라.** `inventory-disposal.md` 는 `intent.md`
    경로를 **평문**으로 적는다. 열쇠는 경로 **문자열**이다.
11. ★ **`SCHEMA_VERSION` 을 한 벌로 올리지 마라.** 행 형식이 안 바뀐 예외표가 빨개지고,
    초록으로 만드는 유일한 길이 **안 바뀐 파일을 손대는 것**이 된다. 버전은 종류마다 산다.
12. ★ **한 커밋에 처분과 레코드를 같이 싣지 마라.** 그 커밋의 SHA 를 `닫은커밋` 에
    원리상 못 적는다. **처분을 먼저 커밋하고 레코드가 그 SHA 를 받아 뒤따른다.**
13. ★ **`target/` 의 바이너리가 낡으면 `버전에_커밋이_실려_있다` 가 거짓 실패한다.**
    제품 결함이 아니다 — `touch crates/pal-cli/build.rs` 로 재빌드해라 (옛 `F06b` 판정-다).

## 걸린 것 — 안 바뀐 결정

- **`CHECK` 실행자·`verify.py`·허용 목록을 안 만든다** (승격 4). 강제는 21 번째 검사가 준다.
- **Stop 은 관측만** (승격 1). `EVENTS` 를 안 건드린다. 관측 결과는 [#85] 의 입력이다.
- **열림 축은 소급 안 함.** 과거 두 회차는 스키마 2 로 남고 「형식 이전」으로 뜬다.
- **금지역 좁힘의 축은 하나** — 「이 회차가 커밋한 산출물이 거짓을 말하는가」.
- **R1(문면)은 고르는 것까지** 하고 규약 기입은 [#88] 이 진다.

## 별도 목록 — §11 ③ 은 **안 비었다**

[#83] 게이트 절 이름 「남기는 빚」 · [#84] `completion-condition` 표준 표 승격 ·
[#85] Stop 정책과 자기 상한 · [#86] `HOOK_EVENTS` 반복 시험 ·
[#87] 죽은 링크 검사의 코드 스팬 오인 · [#88] 고른 다섯의 규약 기입 ·
[#89] 계기판 ③ 진자가 빌드 산출물에 지배됨 · [#90] 표 헤더 한 글자로 회차를 검사 밖에 뺄 수 있다 ·
[#92] 합계 검산이 행 수만 센다 · [#93] 「없음」 자리 행이 ⑦⑧ 을 오염시킨다 ·
[#94] `게이트파서` 선언이 코드 갈래를 못 따라간다.

## 남은 것

**K2 독립 리뷰 · K9 CI.** K9 는 CI 가 초록이 될 때까지 레코드에서 **`열림`** 이고,
그래서 계기판 ⑨ 가 **막힘**을 낸다. 그것이 옳다. **수는 여기 안 적는다** —
`dashboard.py` 를 돌려서 봐라.

⚠⚠ **`round/*` 브랜치는 `push` 로는 CI 가 안 돈다.** `ci.yml` 트리거는
`push: branches: [main]` 과 `pull_request` 뿐이다. **K9 의 닫는 길은 PR** 이고
소유자가 그것을 골랐다 — [#91](https://github.com/hskim-ecoletree/palimpsest/pull/91) 이 섰다.
**런 상태는 여기 안 적는다** — `gh run list --branch round/agent-laziness` 로 봐라. 그리고 **push 를 한 번**만 한다
(`cancel-in-progress: true` — 나눠 push 하면 앞 런이 취소된다).

[#83]: https://github.com/hskim-ecoletree/palimpsest/issues/83
[#84]: https://github.com/hskim-ecoletree/palimpsest/issues/84
[#85]: https://github.com/hskim-ecoletree/palimpsest/issues/85
[#86]: https://github.com/hskim-ecoletree/palimpsest/issues/86
[#87]: https://github.com/hskim-ecoletree/palimpsest/issues/87
[#88]: https://github.com/hskim-ecoletree/palimpsest/issues/88
[#89]: https://github.com/hskim-ecoletree/palimpsest/issues/89
[#90]: https://github.com/hskim-ecoletree/palimpsest/issues/90
[#92]: https://github.com/hskim-ecoletree/palimpsest/issues/92
[#93]: https://github.com/hskim-ecoletree/palimpsest/issues/93
[#94]: https://github.com/hskim-ecoletree/palimpsest/issues/94
