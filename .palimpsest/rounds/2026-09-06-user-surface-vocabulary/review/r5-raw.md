# 독립 리뷰 R5 — 회차 `2026-09-06-user-surface-vocabulary`

`pal-independent-reviewer` 원 반환문. 2026-09-06. **마지막 라운드다.**

> ⚠ **좌표 칸을 기계가 읽을 수 있게 정규화했다** — 글로브·줄임표·중괄호 묶음을 실제 경로
> 하나로 폈고, 조건 칸의 여러 ID 는 첫 하나만, 괄호 주석은 지웠다. **문장은 하나도 안 바꿨다.**
>
> ⚠ **리뷰가 도는 동안 메인이 여덟 번 커밋했다.** 리뷰어가 그것을 스스로 적었고
> (`## 끝내도 되는가` 의 덧붙임), 그 사실이 발견 #17·#18 을 기각으로 뒤집었다.
> **그 덧붙임을 이 회차의 「실패한 접근」에 옮겨 적었다** — 잠긴 의도가 리뷰 도중 바뀌면
> 리뷰가 무엇을 잰 것인지 사후에 대조할 수 없다.

모든 측정을 마쳤다. 측정 중 HEAD가 `5f95f79 → 06313e0 → 686ba33 → f40c16c → 3815ed1 → a306f16 → a97958e → d8381da` 로 여덟 번 움직였다. **아래 판정은 전부 `d8381da` 에서 다시 잰 값이다.**

---

## 합격선 축

| 조건 | 판정 | 잰 수 | 근거 |
|---|---|---|---|
| A1 | 통과 | ADR 1 · 합(合) 산출물 1 | `docs/adr/0033-…md` · `dialectic/a1-synthesis.md` |
| A2 | 통과 | 충돌 명시 1곳 | ADR-0033 「제약 하나가 미리 걸려 있었다 … `Lineage`(binding.rs:566)가 이미 `Current` 를 쓴다」 |
| A2-a | 통과 | 시험 2 · RED 1 | `cargo test -p pal-core 두_축의_와이어` 2 passed; `observations/a2a-red.txt` 가 `rename="current"` 로 `["current"]` 발화를 잡았다 |
| A3 | 통과 | 형식 3 | ADR-0033 §3 표 — 소문자 · 줄 끝 병기 · 영어권 생략 미구현 |
| A4 | 통과 | ADR 1 | `docs/adr/0033-…md` |
| B1 | 통과 | `Fresh` 개명 적용 | `binding.rs` 직렬화 토큰 `fresh` (`pal touch --json` 실측) |
| B2 | 통과 | 근거 1곳 | ADR-0033:131 「별칭을 두지 않았다 — 저장된 데이터가 0 건」 |
| B2-a | 통과 | 양성 1 · 대조 1 | `옛_토큰은_읽기에서_거부된다` + `새_토큰은_읽힌다` 둘 다 초록 |
| B3 | 통과 | **945 passed · 0 failed · rc=0** | `cargo test --workspace` |
| B4 | 통과 | **26/26 · rc=0** | `cargo xtask check` |
| C1 | **대조 불가** | 목록 밖 누락 **3종 실측** | `pal plan docs/plan/00-goals.md` 가 `unresolved (not-at-baseline · 후보 0)` · `pending (PathAbsent)` · `bound 1` 을 병기 없이 찍는다. 게이트 자신이 *"`C1` 의 「전부」에는 모집단 오라클이 없다"* 를 적었다(`a97958e`) |
| C1-a | 통과 | 시험 1 | `crates/pal-cli/tests/export_ascii.rs` |
| C1-b | 통과 | 왕복 자리 4 | `graph.rs` 셋 + `schema.rs` `Cardinality` |
| C1-c | 통과 | 시험 6 · RED 1 | `observations/c1c-red.txt` — 병기를 얹자 2 FAILED |
| C2 | 통과 | 한국어 **2건 · 둘 다 자유 본문** | `pal touch SymbolKind --json` 전수 워크: `/…/note` 와 그 앵커뿐. 병기 꼴 `"…(…)"` **0건** |
| C2-a | 통과 | 파일 39 · 토큰 223 · 한국어 0 | `cargo xtask check` + 순수 함수 음성 대조 |
| C2-b | 통과 | 위반 3 전부 닫힘 | 검사 초록. 전 크레이트 재주사에서 남은 것은 `pal-intent/round_condition.rs`(게이트 파서 계약, 명시적 제외)뿐 |
| C3 | 통과 | 이슈 댓글 2 | `gh issue view 110 --comments` — `pal touch` 전문이 붙어 있다 |
| D1 | 통과 | 일곱 확정 | ADR-0034 표 |
| D2 | 통과 | 좌표 3 · 예외표 89/125 | 예외표 82+7=89 · 전체 83+42=125 재현 ✓ (⚠ 레코드 수는 아래 발견 9) |
| D3 | 통과 | 넷 다 적용 | `touch.rs:296` 「생략」 · `evidence.rs:33` 「이관」 · `surface/queries.toml` 「오라클」 0건 · `folded.md`/철회 |
| D3-a | 통과 | 대조 3단(①②③)+시험 | `observations/d3a-negative-control.txt` — `git clone`+`--root`, 양쪽 아닌 표기에서 발화 |
| D3-b | 통과 | 별칭 1 · 픽스처 2 | `status.rs:649` `contains("철회")‖contains("접힘")` |
| D4 | 통과 | 26/26 rc=0 | 같음 |
| D4-a | 통과 | 945 passed | 같음 |
| D5 | 통과 | ADR 1 · 이슈 인용 0 | ADR-0034:29-31 (⚠ 근거 **좌표**는 틀렸다 — 발견 6) |
| D6 | 통과 | **1·0·0·0·17·7·123** | `grep -rn <낱말> .claude crates xtask scripts \| wc -l` — 게이트 표 일곱 수 **전부 재현** |
| E1 | 통과 | 에이전트 정의 5 신설 | `.claude/agents/pal-{debate-designer,decision-proposer,debate-referee,debate-reporter,condition-auditor}.md` |
| E2 | 통과 | 표 6행 · 「안 받는 것」 6칸 | `SKILL.md §5` |
| E3 | 통과 | 강제/비강제 둘 다 적힘 | `SKILL.md` 「강제되지 않는 것 … 저작이다」 |
| E3-a | 통과 | 인용 보존 + 자리 이동 | `SKILL.md` 「앞선 소유자 지시와 부딪히지 않는다」 |
| E4 | 통과 | PAYLOAD 6 · OWNED_FILES 6 | `install/layout.rs:87-115,193-198` |
| E4-a | 통과 | 출처 enum 6 · 필수절 매핑 | `record.py:70,543` (`독립리뷰`→`["내가 기각한 것","미측정 목록"]`) |
| E5 | 통과 | 산출물 6 · 갈린 지점 2 | `dialectic/g4-{design,thesis,antithesis,synthesis,referee}.md` + `r1-raw.md`. 각 머리에 「안 받은 것」 고지 |
| E6 | 통과 | 945 passed | `놓는_것은_전부_되돌릴_수_있다` 포함 |
| F1 | 통과 | §3.5 신설 | `SKILL.md:160` |
| F2 | 통과 | 세 갈래 + §5 겹침 | `SKILL.md:184-203` |
| F3 | 통과 | 상한 2 | `SKILL.md:213` |
| F4 | 통과 | 평가자 분리 명시 | `SKILL.md:219` |
| F5 | 통과 | 조건 50 · 발견 14 · 기각 7 | `conditions-audit/r1-raw.md` · 검사 「완수 조건 설계 평가」 초록 |
| F6 | 통과 | 강제/비강제 둘 다 | `SKILL.md:224-236` |
| F7 | 통과 | 945 passed | 같음 |
| G1 | 통과 | 동결 5뿌리 | `동결_경로 = ["docs/adr","docs/gates","docs/instructions","corpus","target"]` + `g1-synthesis.md` |
| G2 | **반증** | 「접다」 **35곳 잔존** | 착수 사본 1350 재현 ✓ / 현재 검사 0곳. 그러나 활용형 `접기·접혔·접는` 을 넣으면 **35곳**(AGENTS.md:29 포함)이 선언된 뿌리 안에 남는다 |
| G2-a | 통과 | 시험 8 · 패턴 8 | 발화 8 · 경계 · 표식 · 일반 한국어 · 표지 목록 · 한 줄 표지 · 침묵 |
| G3 | 통과 | 26/26 rc=0 · 945 passed | 같음 |
| G4 | 통과 | 문서 216행 · 절 5 | `~/dev/projects/cut-the-bullshit/KO-SENTENCE-REGISTER.md` |
| G4-b | 통과 | 좌표 5 전부 재현 | `e7b3684` · `merge-base --is-ancestor … origin/main` rc=0 · 216행 · 4.1~4.4 = 159·172·180·189행 |
| G4-c | 통과 | 명령 5 · 관측일 1 | 게이트 `## 효과` 의 `G4-c` 블록 |
| H1 | 통과 | ⓐ 0·0·0 · ⓑ `최신 상태(fresh)` · ⓒ 블록 1 | `h1-touch.txt` 전수: `절단`0 `접힘`0 `가 폅니다`0 (⚠ 인용 바이트 주장은 발견 8) |
| H2 | **미측정** | origin 대비 **49 커밋 ahead** | `git status -sb`; 이 브랜치의 마지막 CI 런은 착수 전 커밋의 것이다 |
| H3 | 통과 | #108~#114 **7/7 CLOSED** | `gh issue view` |
| H3-a | 통과 | 그 줄 1 | 게이트 `### H3-a` |
| H4 | 통과 | 일곱 **전부 덮임** | `git log --format=%s b165445..HEAD` → #108×1 #109×6 #110×5 #111×2 #112×6 #113×1 #114×4 |

---

## 미측정 목록

| # | 안 잰 조건 | 모집단 | 유효 | 해악도 | 좌표(파일:줄) | 왜 못 쟀나 |
|---|---|---|---|---|---|---|
| 1 | `H2` — 마지막 커밋 SHA 에 `conclusion=success` 런 | 원의도 | 참 | 거짓신호 | `.palimpsest/rounds/2026-09-06-user-surface-vocabulary/intent.md` | push 가 0 회다(49 커밋 ahead). 원리상 push 전에는 못 잰다. 게이트가 「미측정」으로 적은 것은 옳다 |
| 2 | `E5` 의 「기존 3역할 절차와 갈리는가」 재현 | 회차기록 | 참 | 거짓신호 | `docs/gates/user-surface-vocabulary.md` | 다시 돌리면 앞 절차 결과가 컨텍스트에 들어와 격리가 깨진다. 나도 못 깬다 |
| 3 | `A1`·`D1`·`G1` 정반합 **판정문의 내용 타당성** | 회차기록 | 추정 | 미관 | `.palimpsest/rounds/2026-09-06-user-surface-vocabulary/dialectic/a1-synthesis.md` | 나는 산출물 존재와 ADR 반영만 쟀다. 판정 자체를 다시 논증하는 것은 이 자리의 일이 아니라고 봤다 |

---

## 의도 축

### 빠진 것

| # | 발견 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 근거(명령·출력) |
|---|---|---|---|---|---|---|---|
| 4 | **`pal plan` 화면이 상태값 셋을 병기 없이 찍는다 — `C1` 의 네 번째 누락이다.** `UnresolvedWhy` 는 영어 토큰만, `PlanBindingState::Pending` 은 **Rust `Debug` 를 그대로** 사람 화면에 낸다 | 원의도 | 참 | 거짓신호 | C1 | `crates/pal-cli/src/plan.rs` | `./target/debug/pal plan docs/plan/00-goals.md` → `예상 좌표  F16b (identifier)  →  unresolved (not-at-baseline · 후보 0)` · `예상 좌표  docs/features/<name>.md (경로)  →  pending (PathAbsent)`. `grep -rn "println!\|format!" crates/pal-cli/src \| grep "\.name()" \| grep -v "label::"` 가 13곳을 낸다 |
| 5 | **`G2` 가 지목한 「접다」가 선언된 아홉 뿌리 안에 35곳 남았다.** `AGENTS.md:29` 「접는 사유는 둘이다」 · `HANDOFF.md:5` · `HANDOFF-check-verifies-work.md` 6곳 · `crates` 20곳 · `xtask/src/main.rs:5679,5754,5758`(`접혔나`) · `scripts/f09-verify.py:509` | 원의도 | 참 | 거짓신호 | G2 | `xtask/src/main.rs` | `d8381da` 격리 클론에서 `접다` 활용형에 `"접기","접혔","접는"` 셋만 더해 `cargo xtask check --root <클론>` → `소유자가 지목한 표현 패턴이 35곳 남았다`. 넣기 전 같은 클론은 0곳 |

### 요구되지 않은 것

없음 — 산출물 전부가 조건에 대응한다.

### 있는데 틀린 것

| # | 발견 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 근거(명령·출력) |
|---|---|---|---|---|---|---|---|
| 6 | **ADR 둘과 게이트가 이름의 근거로 댄 「1차 원문 10행·12행」이 그 문서의 그 줄이 아니다.** 인용문 자체는 정확하나 좌표가 틀렸다 — 원문에서 on-air 문장은 **4행**, 「코드성 데이터는 영어 우선」은 **5행**이다. 10행은 「어색한 표현 예시」 머리, 12행은 「'~ 에 산다'」다. 같은 ADR 의 `23행`(접다)·`31행`(절단)·`32행`(봉투)은 정확해서, 두 좌표만 어긋난다 | 원의도 | 참 | 거짓신호 | D5 | `docs/adr/0034-round-vocabulary-answers-to-the-reader-not-to-the-history.md` | `git show 9eec834:temp-comment.md \| sed -n '4p;5p;10p;12p'` → 4행=on-air · 5행=코드성 · 10행=「한국어권 사용자들에게 어색하거나 잘못된 표현 예시」 · 12행=「'~ 에 산다'」 |

---

## 이번 라운드의 새 발견

| # | 발견 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 근거(명령·출력) |
|---|---|---|---|---|---|---|---|
| 7 | **종료 보고의 상한 표가 「독립 리뷰 5 / 쓴 것 1」로 적는다 — 실제로 네 라운드가 돌았고 그 레코드 103행이 같은 커밋의 원장에 있다.** 이번이 다섯째다. 읽는 사람은 상한을 거의 안 썼다고 믿는다 | 회차기록 | 참 | **금지역** (사실이 아닌 것을 사실로) | 없음 | `.palimpsest/rounds/2026-09-06-user-surface-vocabulary/report.md` | `python3` 로 `findings.jsonl` 집계 → `('독립리뷰',1) 28 · (…,2) 25 · (…,3) 23 · (…,4) 27`. `cargo xtask check` 도 `독립리뷰R1 28↔28 · R3 23↔23 · R4 27↔27 · R2 25↔25` 를 찍는다 |
| 8 | **게이트 `## 효과` 가 `pal touch` 인용 블록을 *"잡아 둔 전문에서 그대로 잘라 온 것 … 같은 실행의 같은 바이트다"* 라고 적는데, 그 블록의 한 줄에 원 출력에 없는 `<!-- 검출-수단-자기-제외-시작 검출-수단-자기-제외-끝 -->` 가 삽입돼 있다.** 검출 수단을 통과시키려고 증거 바이트를 고친 것이고, 같은 자리가 이미 두 번(R3·R4) 어긋났던 곳이다 | 회차기록 | 참 | **금지역** (사실이 아닌 것을 사실로) | H1 | `docs/gates/user-surface-vocabulary.md` | 게이트 코드블록 17줄을 전문과 대조 → `mismatched lines: 1` · `'      ③ \`SymbolKind\` 를 접지 않고 늘린다 <!-- 검출-수단-자기-제외-시작 검출-수단-자기-제외-끝 -->'` |

---

## 자기 산출에 대한 발견

| # | 발견 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 근거(명령·출력) |
|---|---|---|---|---|---|---|---|
| 9 | **게이트 `D2` 의 「판정 시점에 다시 센」 레코드 수 1564 가 지금 1639 다.** 그 문장 바로 위가 *"앞 판은 1502 라 적었는데 어느 시점에도 안 맞는 수였다"* 를 적고 있어, 같은 실패의 세 번째 판이다. 회차가 레코드를 더할 때마다 낡으므로 구조적이다 | 회차기록 | 참 | 거짓신호 | 없음 | `docs/gates/user-surface-vocabulary.md` | `cargo xtask check` → `회차 레코드 — … 레코드 1639행`; 독립 집계도 1639 |
| 10 | **커밋된 종료 보고에 미치환 자리표시자 여섯이 남아 있다** — `HAZ_금지역`·`HAZ_실패`·`HAZ_거짓신호`·`HAZ_미관`·`REC_행`·`SRC_사전부검`·`SRC_독립리뷰`·`SRC_정반합`·`SRC_조건평가`·`SRC_실측`·`PUSH_OBSERVATION`. 그 문장은 *"회차 전체의 해악도 분포는 … 이고 전부 처리 방침이 실행됐다"* 라고 단언하는데 수가 하나도 없다. `PUSH_OBSERVATION` 만 push 뒤에 채울 것이고 나머지는 지금 채울 수 있다 | 회차기록 | 참 | 거짓신호 | 없음 | `.palimpsest/rounds/2026-09-06-user-surface-vocabulary/report.md` | `grep -n "HAZ_ REC_ SRC_ PUSH_OBSERVATION" report.md` → 4행 |
| 11 | **`H4` 의 「원문 12행의 커밋 규율」이 `## 원문` 12행이 아니다.** 그 줄은 「비싼 결정이므로 정반합을 걸고 ADR 로 발행한다」이고, 커밋 규율은 **17행**이다. 착수 커밋 `f91bea2` 부터 줄 오프셋이 안 변했으므로 낡음이 아니라 처음부터 틀렸다 | 회차기록 | 참 | 거짓신호 | 없음 | `.palimpsest/rounds/2026-09-06-user-surface-vocabulary/intent.md` | `git show f91bea2:…/intent.md` 의 12행·17행 대조 |
| 12 | **「어색한 표현 부재」의 「접다」 활용형 목록이 조건 문면의 「꼴」보다 아직 좁다** — `접기`·`접혔`·`접는`(단독)이 없다. `R4` 가 같은 결함(390곳)을 잡아 목록을 넓힌 **직후의 두 번째 잔여**이고, 넓힌 목록 자신이 「검사가 0곳을 낸다」를 합격선으로 만든 상태다 | 자기장치 | 참 | 거짓신호 | 없음 | `xtask/src/main.rs` | 발견 5 와 같은 명령. 세 형태만 더해 35곳 |
| 13 | **`observations/g2a-red.txt` 가 낡았다** — ⑤가 `패턴 7개 · 파일 235개 · 줄 81799개` 로 적고 ④가 시험 이름을 `일곱_패턴이_각각_발화한다` 로 적는데, 지금 값은 `패턴 8개 · 파일 246개 · 줄 85688개` 이고 시험 이름은 `여덟_패턴이_각각_발화한다` 다. 게이트 RED 표가 이 파일을 `G2`·`G2-a` 의 기록으로 링크한다 | 회차기록 | 참 | 미관 | 없음 | `.palimpsest/rounds/2026-09-06-user-surface-vocabulary/observations/g2a-red.txt` | `cargo xtask check` 마지막 줄 ↔ 파일 ⑤; `grep -n "fn 여덟_패턴" xtask/src/main.rs` → 4698 |
| 14 | **개정 R5 가 `G4` 판의 산출물을 넷으로 적는다** — 실제로는 여섯이다(`g4-referee.md` · `dialectic/r1-raw.md` 누락). `SKILL.md §5` 가 *"판 하나가 산출물 **여섯**을 남긴다"* 로 그 수를 규약에 못 박았다 | 회차기록 | 참 | 미관 | 없음 | `.palimpsest/rounds/2026-09-06-user-surface-vocabulary/intent.md` | `ls dialectic/g4-*` → design·thesis·antithesis·synthesis·**referee** 다섯 + `r1-raw.md` |

---

## 내가 기각한 것

| # | 기각한 것 | 모집단 | 유효 | 해악도 | 좌표(파일:줄) | 왜 아니었나 |
|---|---|---|---|---|---|---|
| 15 | 게이트가 *"palimpsest 도메인 용어 제외 **명시**"* 라고 적었는데 그 문서에 `palimpsest`·`도메인`·`제외` 가 0건이다 | 거짓 | 거짓 | 거짓신호 | `docs/gates/user-surface-vocabulary.md` | 문서 157행이 **"특정 프로젝트에서만 쓰는 용어는 뺐다."** 로 적는다. 낱말만 다르고 명시는 있다 |
| 16 | `cargo test` 가 `버전에_커밋이_실려_있다` 에서 FAILED — 「초록이 거짓」의 재발인가 | 거짓 | 거짓 | 실패 | `crates/pal-cli/tests/version_is_in_the_binary.rs` | 측정 중 메인이 커밋해 HEAD 가 움직여 생긴 낡은 빌드 산물이다(`산출했다: pal 0.0.0+06313e0` / `HEAD: 686ba33`). 재빌드 후 통과, 전량 945 passed |
| 17 | 게이트 `D6` 의 「오라클 0 · 접힘 2 · 처분 94」가 **어느 단일 모집단으로도 재현되지 않는다** — 처분 94 를 내는 모집단은 접힘 3 · 오라클 1 을 낸다 | 거짓 | 거짓 | 거짓신호 | `docs/gates/user-surface-vocabulary.md` | 내가 재는 동안 커밋 `a306f16` 이 모집단을 `.claude·crates·xtask·scripts` 로 못 박고 수를 갈래와 함께 다시 적었다. 그 값 일곱을 `grep -rn … \| wc -l` 로 **전부 재현했다**(1·0·0·0·17·7·123) |
| 18 | 게이트 RED 표가 `G2` 의 교정 전 수를 **931** 로 적는데 합격선 산출은 **1350** 이라 한 사실에 두 수다 | 거짓 | 거짓 | 거짓신호 | `docs/gates/user-surface-vocabulary.md` | 측정 중 커밋 `d8381da` 가 그 칸에 *"⚠ 그 뒤에 활용형이 넓어졌다 — 같은 사본을 넓힌 수단으로 다시 재면 1350 이다"* 를 붙였다 |
| 19 | 「기계 토큰에 한국어 금지」의 모집단이 `pal-core`·`pal-query`·`pal-store` 셋뿐이라 `pal-cli`·`pal-extract`·`pal-git`·`pal-intent` 의 위반을 놓친다 | 거짓 | 거짓 | 실패 | `xtask/src/main.rs` | 네 크레이트를 같은 규칙으로 재주사한 결과 걸리는 것은 `pal-intent/src/round_condition.rs:41-90`(`"통과"`·`"반증"` 등) 뿐이고, 그것은 게이트 파서 계약이라 한국어가 정본이다. 검사가 그 제외를 주석으로 명시한다 |
| 20 | `crates/pal-cli/src/label.rs` 가 연결 안 된 표면(stub)이다 | 거짓 | 거짓 | 실패 | `crates/pal-cli/src/label.rs` | `pub fn` 여덟 전부에 호출자가 있다 — `ledger.rs` 3 · `query.rs` 3 · `touch.rs` 3 · `doctor.rs` 2 (총 11곳, 4파일) |

---

## 끝내도 되는가

**안 된다.** 본 목록에 **금지역 둘**(#7 종료 보고의 「독립 리뷰 1」 · #8 게이트가 「같은 바이트」라 적은 인용 블록의 삽입된 표지)과 **원의도 거짓신호 셋**(#4 `pal plan` 의 병기 누락 · #5 「접다」 35곳 · #6 ADR 의 원문 좌표)이 남았다. `H2` 는 push 전이라 원리상 미측정이고 그것은 정당하다.

⚠ **덧붙임 — 이 리뷰의 기반이 흔들렸다.** 측정 중 `intent.md` 가 두 번(`06313e0` 상자 전사 · `686ba33` 범위 밖 정정과 「답 안 온 물음」 신설), 게이트가 세 번(`a306f16`·`a97958e`·`d8381da`) 바뀌었다. 위 판정은 전부 `d8381da` 에서 다시 잰 값이지만, **잠긴 의도가 독립 리뷰가 도는 동안 바뀌면 리뷰가 무엇을 잰 것인지 사후에 대조할 수 없다.** #17·#18 이 그 때문에 기각으로 뒤집혔다 — 운이 좋았던 것이지 구조가 막은 것이 아니다.

주요 파일 경로:
- `/Users/incognito/dev/projects/palimpsest/.palimpsest/rounds/2026-09-06-user-surface-vocabulary/intent.md`
- `/Users/incognito/dev/projects/palimpsest/.palimpsest/rounds/2026-09-06-user-surface-vocabulary/report.md`
- `/Users/incognito/dev/projects/palimpsest/docs/gates/user-surface-vocabulary.md`
- `/Users/incognito/dev/projects/palimpsest/xtask/src/main.rs`
- `/Users/incognito/dev/projects/palimpsest/crates/pal-cli/src/plan.rs`
- `/Users/incognito/dev/projects/palimpsest/crates/pal-core/src/plan.rs`
- `/Users/incognito/dev/projects/palimpsest/docs/adr/0033-a-word-that-reads-wrong-is-not-fixed-by-a-gloss.md`
- `/Users/incognito/dev/projects/palimpsest/docs/adr/0034-round-vocabulary-answers-to-the-reader-not-to-the-history.md`