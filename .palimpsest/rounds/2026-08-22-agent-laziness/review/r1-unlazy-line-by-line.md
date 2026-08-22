# R1 · unlazy SKILL.md ↔ `/round` SKILL.md 전수 대조

- **A(외부)** — Leonxlnx/unlazy v2.0.0 (`ed9e8d2`) `SKILL.md`, 104 행.
  사본: `scratchpad/unlazy/SKILL.md`
- **B(우리)** — `/Users/incognito/dev/projects/palimpsest-agent-laziness/.claude/skills/round/SKILL.md`, 549 행.
- 작성 2026-08-23. **B 의 문면은 고치지 않았다.** 이 문서만 썼다.

⚠ **대조 범위의 한계** — 받은 사본에는 `SKILL.md` **한 파일뿐**이다.
A 가 가리키는 `references/method.md` · `references/orchestration.md` ·
`references/token-economy.md` · `templates/gates-leaf.md` · `scripts/gate-check.mjs` ·
`scripts/install-hooks.mjs` 는 **없다.** 그래서 이 대조는 **A 의 SKILL.md 본문만**을
모집단으로 한다. 참조 파일에만 있는 규범은 이 표에 **안 들어온다.**

---

## 0. 모집단 — 어떻게 갈랐나

**단위는 문장이다.** 마침표로 끊는다. 불릿 한 개가 두 문장이면 두 단위다.
굵은 글씨 선두 조각(`**Contracts before fan-out.**` 꼴)은 그 자체가 규범 조각이라 한 단위로 센다.

**규범 문장** = 행동을 지시하거나 · 금지하거나 · 기준을 정하는 문장.

**빼는 것과 그 수:**

| 뺀 것 | 수 | 비고 |
|---|--:|---|
| YAML frontmatter | 1 블록 (9 행) | `name`·`description`·`license`·`metadata` |
| 제목·절 헤딩 | 10 | `# Unlazy` 포함 |
| 코드펜스 | 2 | 25–27 · 96–98 |
| 순수 서술·근거·라벨·리스트 도입 문장 | 29 | 아래 §0.1 에 전부 나열 |

**본문 문장 98 개 중 규범 문장 M = 69, 비규범 29.** (98 = 69 + 29)

### 0.1 비규범으로 뺀 29 문장 (전수)

| 행 | 문장 | 왜 뺐나 |
|--:|---|---|
| 13 | "You are running under anti-laziness discipline." | 프레이밍. 행동·금지·기준 아님 |
| 13 | "The failure this skill exists to kill is output that is technically responsive but quietly incomplete: …" | 목적 서술 |
| 15 | "v1 of this skill fought these with instructions." | 역사 |
| 15 | "A controlled six-run test showed the limit of that: …" | 역사·측정 보고 |
| 15 | "So v2 moves enforcement out of your goodwill and into files and checks." | 설계 서술 |
| 21 | "Why a file: your intentions do not survive a long context, files do." | 근거 |
| 21 | "A checklist you wrote at minute 2 is still exactly as sharp at minute 90 …" | 근거 |
| 31 | "A clean, visible handover beats silent degradation, and the enforcement tooling treats an ABANDON line as an honest exit, not a failure." | 근거 |
| 35 | "**Solo** (default)." | 라벨 |
| 37 | "**Orchestrated**." | 라벨 |
| 39 | "The reason orchestrated mode exists: the stall-at-80-percent failure is an end-of-long-context disease." | 근거 |
| 39 | "A fresh context per leaf means every leaf starts with full attention." | 근거 |
| 39 | "That is the honest version of \"every leaf gets the full budget\" …" | 근거 |
| 43 | "Created by Leonxlnx." | 서술 |
| 43 | "In v2 the tree is a decomposition tool, not an effort multiplier; measured runs showed …" | v1→v2 역사 서술 |
| 43 | "What depth buys you is structure:" | 리스트 도입 |
| 45 | "Layer 1 is the task." | 정의 |
| 45 | "Leaves are where work happens." | 정의 |
| 47 | "Deep effort that does not integrate is waste." | 근거 |
| 48 | "Thirty-two finished leaves can still be a broken product; branch gates are where that is caught." | 근거 |
| 64 | "The single most reproducible failure in tested runs: final reports whose numbers were wrong while their substance was right." | 측정 보고 |
| 64 | "Confident claims like \"34 stat rows\" where 17 exist, written from memory instead of measurement." | 예시 서술 |
| 70 | "The keepers from v1, still true, now backed by structure:" | 리스트 도입 |
| 72 | "If you notice yourself composing a status summary while boxes are unchecked, that is the laziness reflex firing." | 진단 서술 |
| 73 | "This is continuation forcing made mechanical." | 서술 |
| 81 | "The rules that keep this skill cheap, expanded in references/token-economy.md:" | 리스트 도입 |
| 92 | "If the harness is Claude Code, this skill ships a Stop hook that structurally blocks ending the turn …" | 도구 서술 |
| 92 | "It converts \"no report until done\" from a rule into a wall." | 서술 |
| 100 | "Everything else in this skill works without it, in any harness that can read a markdown file." | 서술 |

---

## 1. 대조 표 — 규범 문장 69 전수

판정: `있음` · `더 강함` · `약함` · `없음`.
**B 를 원문으로 인용 못 하면 `없음` 이다.**

| # | A 의 문장 (원문) | B 의 대응물 (절 + 원문 인용) | 판정 |
|--:|---|---|---|
| 1 | "You do not promise you are done." | §11 조건 1 — "등록된 완수 조건을 재고 판정을 커밋했다." | 더 강함 |
| 2 | "You prove it against a ledger." | §9 — "★ **원장이 둘이라는 것이 이 표의 존재 이유다.** 판정은 `intent.md` 의 상자에도 있고 게이트의 이 표에도 있다. 둘이 갈리는 것을 **`cargo xtask check` 의 「원장 둘 대조」** 가 댄다" | 더 강함 |
| 3 | "Before starting real work, write the acceptance gates to a file." | §9 — "## 합격선     측정 전에 등록한다. RED 관측 · 음성 대조 · 퇴로를 함께." | 더 강함 |
| 4 | "Not in your head, not in prose, in a file: `GATES.md` in the working directory, using the format in templates/gates-leaf.md." | 머리말 — "회차 하나가 디렉터리 하나다 — `.palimpsest/rounds/<slug>/` 에 `intent.md`(잠긴 의도)와 `state.md`(교대용 상태)가 산다. 둘 다 커밋한다." + §3 의 절 이름 여섯 고정 | 더 강함 |
| 5 | "One checkbox per outcome the task requires, and wherever an outcome can be checked by a command, give it a `CHECK:` line and an `EXPECT:` line so the check is runnable rather than a matter of opinion." | §3 — "**완수 조건    체크 가능한 것만. [ ] 로 적는다.**" / 조건 한 줄 형식 `- [x] <ID> <조건>  · <판정> ⟨전사 YYYY-MM-DD⟩` | 약함 — 상자 하나=조건 하나와 「체크 가능한 것만」은 있으나, **조건마다 실행 가능한 명령(`CHECK:`)과 기대값(`EXPECT:`)을 다는 칸이 없다** |
| 6 | "Done means every box is checked with evidence recorded." | §3 — "\| **상자 켜짐** \| **판정이 났다** — 통과 · 반증 · 대조불가 \|" + "`· <판정>` … **켜졌으면 반드시 있고, 안 켜졌으면 없어야 한다**" | 더 강함 |
| 7 | "Run the bundled checker to execute the checks and record evidence for you:" | §3 — "**세는 자리는 하나다** — `python3 .claude/skills/round/bin/record.py conditions <intent.md>`" ; §7 — "`cargo xtask check` 의 **회차 레코드** 검사가 스키마·enum·좌표·합계 검산을 잰다." | 있음 |
| 8 | "Manual gates (no CHECK possible) are checked by hand, but only with the `EVIDENCE:` line replaced by actual proof: a measurement, a quote of output, a file path with the relevant line." | §3 — "`⟨전사 …⟩` \| 게이트에서 옮겨 적었다는 표시." ; §7 — "전 출력을 파일로 받고 거기서 센다." | 약함 — **조건마다 「증거로 교체」를 요구하는 칸이 없다.** `⟨전사⟩` 는 출처 표시이지 증거가 아니다 |
| 9 | "An evidence line still reading `pending` is an unmet gate, whatever the checkbox says." | §3 — "\| **상자 안 켜짐** \| **미측정.** 아직 안 쟀다 \|" + "★ **켜짐이 「통과」가 아니다.**" | 있음 |
| 10 | "If a gate becomes genuinely impossible, do not quietly drop it." | §5 막힘 — "**막혔다고 적고 올린다.** 축소로 위장해 완료를 선언하지 않는다." ; §10 — "이 네 절은 종료 보고에 **없다**. 하나라도 나타나면 회차가 안 끝난 것이다." | 더 강함 |
| 11 | "Add a line `ABANDON: <gate id> <reason>` to the gates file and say so in your report." | §10 — "## 원리상 못 잰 것    왜 원리상인지 + 무엇이 갖춰지면 잴 수 있는지. 조건이 없으면 그것은 경계가 아니라 잔여다." ; §9 — "## 판정       통과 · 반증 · 대조 불가 · **미측정**." | 더 강함 |
| 12 | "The task fits one focused stretch: roughly under half an hour of real work, tree depth 3 or less." (Solo 선택 기준) | — | 없음 (⚠ B 에 **반대 규범**이 있다: §5 교대 "한 회차를 한 컨텍스트에 맞추지 않는다. **의도에 맞춘다.**") |
| 13 | "One `GATES.md`, work until it is fully checked, report with the ledger pasted." | 머리말 — "한 회차는 **의도를 잠그고, 끝날 때까지 루프를 돌고, 실제로 쓰이는 것을 보고 끝난다.**" ; §9 표준 표 + "**검산** — 통과 3 · 반증 0 · 대조불가 2 · 미측정 0 = 5" | 있음 |
| 14 | "The task is a build: tree depth 4 or more, or clearly beyond one sitting." (Orchestrated 선택 기준) | — | 없음 |
| 15 | "Decompose per references/method.md, write `PLAN.md` plus one gates file per leaf under `gates/`, and run each leaf as a fresh subagent with a narrow brief." | 오케스트레이션 — "메인은 **얇은 경계**다 — 소유자와 대화하고, 상태를 갱신하고, 위임한다." | 약함 — 위임은 있으나 **리프 단위 분해도, 리프마다 게이트 파일도 없다.** B 의 게이트 단위는 회차/기능이다 |
| 16 | "Read references/orchestration.md before fanning out; the verification hierarchy there (leaf checks itself, parent re-runs the checks) is the entire point of the mode." | §10 — "**커밋한다.** 그래야 다음 회차와 `gate-verifier` 가 원문으로 대조한다." | 약함 — 사후 대조자는 있으나 **「부모가 자식의 검사를 다시 돌린다」는 계층 규칙이 없다** |
| 17 | "**Split at natural joints, N layers deep.**" | — | 없음 |
| 18 | "**A leaf is a real unit of work**: ten or more minutes of focused effort, one coherent deliverable." | — | 없음 |
| 19 | "If your leaves come out smaller, you went one layer too deep; back off." | — | 없음 |
| 20 | "**Contracts before fan-out.**" | — | 없음 |
| 21 | "If leaves touch shared surfaces, write the interfaces, data ownership and naming into `PLAN.md` first." | — | 없음 |
| 22 | "**Branches get gates too.**" | — | 없음 |
| 23 | "Every internal node gets an integration gates file: children merged, interfaces match, cross-checks pass." | — | 없음 |
| 24 | "**Effort per leaf comes from its gates**, not from N." | §7 — "**등록된 합격선을 재고 판정하면 끝이다.** 추가 측정도 상향도 하지 않는다." | 있음 |
| 25 | "A leaf is finished when its gates file is fully checked with evidence, or a full improvement pass finds nothing, whichever is later." | §11 — "전부 참일 때 끝난다:" (조건 여섯) + "#### ① 상한 … ② 해악 게이트 … ③ 모집단 분리" | 있음 (⚠ 뒷절 "a full improvement pass finds nothing" 은 B 가 **반증**했다 — §11 "★ **앞 판은 「소진 = 새 발견이 연속 두 라운드 0」이었고 반증됐다.**") |
| 26 | "Scale guidance: tree 2 or 3 for a feature, a bug hunt, a document, solo mode." | — | 없음 |
| 27 | "Tree 4 or 5 for a subsystem or serious refactor." | — | 없음 |
| 28 | "Tree 6 or 7 for an entire project built to a high bar, orchestrated, with leaves mapped to disjoint work units and parallelized where the harness allows." | — | 없음 |
| 29 | "**Implement completely.**" | 머리말 — "**이 회차에서 나온 것은 이 회차에서 없어진다.** 「나중에」 칸이 없다." ; §10 — "가르는 문장: **「이 회차에서 그것을 할 수 있었는가.」** 할 수 있었으면 한다." | 더 강함 |
| 30 | "No placeholders, no TODO, no \"rest as exercise\"." | §5 — "이월 사유가 **아닌** 것: 어렵다 · 시간이 없다 · 컨텍스트가 모자란다 · 생각보다 넓다." ; §10 — "「의도적으로 안 한 것」 · 「확인 못 한 것」 · 「추론(확인 아님)」 · 「다음으로 넘기는 것」 — 이 네 절은 종료 보고에 **없다**." | 더 강함 |
| 31 | "**Re-read as a domain expert.**" | §7 — "`pal-independent-reviewer` 를 띄운다. … **의도 축** — 빠진 것 · 요구되지 않은 것 · 있는데 틀린 것." | 약함 — 자리는 **종료 전 독립 리뷰**뿐이고, **작업 중 자기 산출을 전문가 눈으로 다시 읽는 패스가 없다** |
| 32 | "Name the cheap version of each part, replace it with the good version." | — | 없음 |
| 33 | "**Hunt defects.**" | §7 — "**의도 축** — 빠진 것 · 요구되지 않은 것 · 있는데 틀린 것." ; §2 — "`pal-premortem-sweeper` 를 띄운다." | 있음 |
| 34 | "Edge cases, correctness, performance, the tells that something is fake." | §11 금지역 — "측정이_죽은_가지 = \"검사·시험이 있는데 실제로는 아무것도 재지 않는다. 통과가 거짓이 된다\"" | 약함 — 「가짜의 낌새」만 대응하고 **엣지 케이스·성능은 B 어디에도 없다** |
| 35 | "Fix what you find." | §5 — "회차 중에 나온 것은 **그 자리에서** 없어진다." | 있음 (B 는 「없앤다」에 범위 밖·승격도 포함하므로 「고친다」보다 넓다) |
| 36 | "**Polish that costs nothing.**" | — | 없음 (⚠ B 에 **반대 규범**: §11② "\| **미관** \| **닫을 수 있다** \|") |
| 37 | "Tuned constants beat new features." | — | 없음 |
| 38 | "A pass that produces no improvement, plus a fully checked gates file, is the only finish line." | — | 없음 (⚠ B 가 **명시적으로 반증**: §11 "**네 계열 어디서도 dry 가 안 났다.**") |
| 39 | "So: at report time, re-measure every number you are about to state, or label it unverified." | §9 — "> 수를 안 적고 조건 ID를 적는다. ID를 적으면 수는 세면 나온다. 캐시가 아니다." ; §7 — "★ **에이전트의 원 반환문을 파일로 보존한다** … 자기가 쓴 것을 자기가 세면 그것은 검산이 아니라 항등식이다." | 더 강함 |
| 40 | "Paste the gates ledger with its count, N of N checked." | §9 — "**검산** — 통과 3 · 반증 0 · 대조불가 2 · 미측정 0 = 5" + "★ 넷의 합이 등록된 조건 수와 같아야 한다 — 검산하고 적는다." | 있음 |
| 41 | "A report is a set of claims backed by a ledger, never a vibe of completion." | §10 — "**커밋한다.** 그래야 다음 회차와 `gate-verifier` 가 원문으로 대조한다." ; §11 조건 1 | 있음 |
| 42 | "**No report until the ledger is full.**" | §10 — "하나라도 나타나면 회차가 안 끝난 것이다." ; §11 — "전부 참일 때 끝난다" | 있음 |
| 43 | "Open the gates file and pick the next unchecked box." | §5 계기판 — "여덟 칸이 나온다 — 자기 비율 · **미판정 잔액** · 진자 · …" | 약함 — 잔액을 **보이는** 자리는 있으나 B 가 못 박기를 "★ **이것은 판정 도구가 아니다.** 통과·실패를 내지 않고 아무것도 막지 않는다." — **「다음 미체크 상자를 집어라」라는 행동이 없다** |
| 44 | "**When you feel finished, check instead of concluding.**" | §5 막힘 — "시간·컨텍스트 상한에 닿은 것도 완료가 아니라 막힘이다." ; §11① — "★ **상한에 닿는 것은 완료가 아니다.** ②가 통과해야 닫힌다." | 약함 — 「상한 도달 ≠ 완료」만 있고, **「끝난 느낌」이라는 방아쇠와 그때 할 행동이 없다** |
| 45 | "Run gate-check, then re-read one passed gate adversarially and try to refute its evidence." | §3 — "**음성 대조** — *\"검사가 고장이면 드러나는 조건\"*." | 약함 — 음성 대조는 **측정 전 사전 등록**이지 **통과한 게이트를 사후에 적대적으로 반박하는 행동이 아니다** (그 행동은 B 본문 밖 `gate-verifier` 에이전트 정의에만 있다) |
| 46 | "**Finish one line of attack.**" | §5 막힘 — "같은 실패가 **3 회** 반복되거나, 재현은 되는데 원인을 못 대면 막힌 것이다." | 약함 — **언제 그만둘지의 상한**만 있고 **「바꾸기 전에 지금 것을 끝내라」가 없다** |
| 47 | "Before switching approach, state what the current one still has to give and why switching wins." | — | 없음 |
| 48 | "If you cannot, keep going." | — | 없음 |
| 49 | "**Do not simulate work you can do.**" | §6 — "**추정을 조회인 척하지 않는다.** *\"모르는 것을 안다고 하지 않는다\"* 가 이 제품의 첫 번째 원칙이다." ; §8 — "`pal` 을 직접 부르거나 스킬을 실제로 태워서 **물음 하나에 답을 받는다.** 검사가 통과했다는 것은 효과를 봤다는 뜻이 아니다." | 더 강함 |
| 50 | "If an action is cheap and reversible, take it and observe rather than reasoning about what it would probably do." | §2 — "**영향 범위는 상상하지 말고 조회한다** — §6 「그래프」." ; §6 — "★ **가장 자주 걸리는 자리: `grep` 을 집으려는 순간.** … **그 자리에서 그래프에 먼저 묻는다.**" | 있음 |
| 51 | "**Ignore resource anxiety.**" | §5 — "이월 사유가 **아닌** 것: 어렵다 · 시간이 없다 · 컨텍스트가 모자란다 · 생각보다 넓다." | 더 강함 |
| 52 | "Never compress, summarize or stub because the end feels near." | §5 막힘 — "축소로 위장해 완료를 선언하지 않는다." ; §5 — "**의도를 향해 가는 변경은 자유, 의도에서 멀어지는 변경은 승인.**" (축소는 승격) | 더 강함 |
| 53 | "If a real limit approaches, write remaining work into the gates file and hand over cleanly with ABANDON lines and reasons." | §5 교대 — "모자라면 교대한다." + "`state.md` 에는 현재 단계·남은 것과 함께 **실패한 접근**을 적는다 — 다음 컨텍스트가 같은 벽에 다시 부딪히지 않도록." | 더 강함 |
| 54 | "**Full files, full lists, full sweeps.**" | §7 — "전 출력을 파일로 받고 거기서 센다. 파이프로 잘라 `error[E` · `panicked` 를 날리지 않는다." ; §7 — "**모집단이 비면 실패다** — 0 건은 「안 부른다」가 아니라 「안 봤다」다." | 약함 — **측정 출력의 절단 금지**와 **빈 모집단 금지**는 있으나, **작업 대상 전체를 훑으라는 요구가 없다** |
| 55 | "If the task says all 80 files, the count opened must be 80, and you state that count." | §9 — "★ 넷의 합이 등록된 조건 수와 같아야 한다 — 검산하고 적는다." | 약함 — 검산은 **판정 수 ↔ 등록 조건 수**에만 걸리고, **훑은 수 ↔ 선언한 모집단 수**를 대는 자리가 없다 |
| 56 | "Sampling is only acceptable when declared." | §3 — "## 범위 밖      이 의도가 답하지 않기로 한 것. 한 줄씩." | 약함 — 「안 답하기로 한 것」을 선언하는 자리는 있으나 **「표본을 뽑았다」를 선언하는 행동이 없다** |
| 57 | "Discipline is not maximalism, and enforcement should be nearly free." | §5 계기판 — "**읽기를 강요하지 않으므로 인지비용이 사람의 통제 아래** 있다." ; §9 — "`docs/gates/<기능>.md` 는 **그 회차가 건드린 코드보다 작다.**" | 있음 |
| 58 | "Checks run as shell commands, not as you re-reading everything you wrote." | §3 — "**세는 자리는 하나다** — `python3 .claude/skills/round/bin/record.py conditions <intent.md>`" + "`^- \\[` 로 세면 … **`3/4`** 가 나온다. 실측이다." ; §11 — "⚠ **수를 여기 안 적는다** — 세는 자리는 `xtask` 의 `checks.len()` 이고, 손으로 베면 검사가 하나 늘어나는 날 갈린다" | 더 강함 |
| 59 | "Evidence is capped: the deciding lines of output, never full logs." | 오케스트레이션 — "\| **컨텍스트 절약** \| … \| 필요한 것 전부. 큰 출력은 파일에 적고 요약만 돌려준다 \|" | 있음 (B 는 「상한」이 아니라 **자리를 가른다** — 파일엔 전량, 메인엔 요약) |
| 60 | "In orchestrated mode, a leaf brief is the contract plus its gates file, never the parent's history." | §5 교대 — "★ **새 컨텍스트에 주는 것은 「잠긴 의도 전문 + `state.md` 요약」이다. 직전 산출물을 시드로 주지 않는다.**" ; 오케스트레이션 — "\| **판단 독립** \| … \| **원 의도와 검토 대상만.** 대화 기록도 앞 라운드 서사도 주지 않는다 \|" | 더 강함 |
| 61 | "Append to `PLAN.md`'s status log, do not rewrite the file." | §3 — "## 원문        ← 절대 안 바뀐다" + "## 개정        루프가 적는다. 정정·확대를 무엇을 왜 바꿨는지." | 있음 |
| 62 | "Mechanical leaves go to a cheaper model or lower effort where the harness allows it." | — | 없음 |
| 63 | "Below roughly half an hour of work, stay solo; subagent overhead only pays for itself on real builds." | — | 없음 |
| 64 | "It changes harness behavior, so never install it silently." | §11 금지역 — "★ **이 파일을 `pal install` 이 놓지 않는다.** … **우리가 소유한 자리에 사람이 고치는 것이 정상인 내용을 두는 것 자체가 모순이다.** 그래서 **「있으면 읽는다」**로 적는다." | 더 강함 (같은 축에서 B 는 아예 안 놓는다) |
| 65 | "When a task would clearly benefit, offer it once:" | §5 승격 — "★ **승격은 물음 하나에 칸 하나다.** 둘을 묶어 올리면 답 하나가 두 물음을 닫은 것처럼 보이고" | 있음 |
| 66 | "and tell the user what it does and how to remove it (`--uninstall`)." | — | 없음 |
| 67 | "Conversational replies, trivial edits and factual questions get normal effort." | frontmatter `description` — "새 일을 시작할 때 · 이슈를 집을 때 · 소유자가 무언가를 요구할 때 쓴다." | 약함 — **부를 자리**는 정하나 **「사소한 일에는 규약을 안 태운다」는 면제 문장이 본문에 없다** |
| 68 | "No gates file for a one-line fix." | — | 없음 |
| 69 | "The tree is for work the user wants DONE WELL, and the discipline exists to make \"done well\" the only kind of done you produce." | 머리말 — "한 회차는 **의도를 잠그고, 끝날 때까지 루프를 돌고, 실제로 쓰이는 것을 보고 끝난다.**" | 있음 |

### 검산

| 판정 | # | 수 |
|---|---|--:|
| **있음** | 7 9 13 24 25 33 35 40 41 42 50 57 59 61 65 69 | 16 |
| **더 강함** | 1 2 3 4 6 10 11 29 30 39 49 51 52 53 58 60 64 | 17 |
| **약함** | 5 8 15 16 31 34 43 44 45 46 54 55 56 67 | 14 |
| **없음** | 12 14 17 18 19 20 21 22 23 26 27 28 32 36 37 38 47 48 62 63 66 68 | 22 |

**검산** — 있음 16 · 더 강함 17 · 약함 14 · 없음 22 = **69 = M.**
**표 행 수 69 = M 69.** 빠진 문장 없다.

기계로 검산했다 (손으로 안 셌다):

```
$ awk '/^## 1\. 대조 표/,/^### 검산/' r1-unlazy-line-by-line.md \
  | grep -oE '^\| *[0-9]+ \|' | grep -oE '[0-9]+' | sort -n | diff - <(seq 1 69)
# 출력 없음 — 1..69 전수, 중복·결번 없다
$ grep -cE '^\| *[0-9]+ \| "' <위 구간>        →  69
$ 0.1 제외표 행 수                              →  29     (69 + 29 = 98)
$ 판정별                더 강함 17 · 약함 14 · 없음 22 · 있음 16  →  69
```

⚠ 판정 분포를 세는 `awk` 한 줄은 **#36 을 못 읽는다** — 그 행의 판정 칸에 `\|`
(이스케이프한 파이프)가 들어 있어 필드가 밀린다. **`없음` 으로 손으로 확인했고,
그래서 없음은 21 이 아니라 22 다.** 자동 계수를 그대로 믿으면 하나가 샌다.

---

## 2. 「대응물 없음」 목록 — 규범 문장 69 개 중 **없음 22 · 약함 14** (합 36)

비용은 **B 에 넣었을 때 늘어나는 줄 수**의 추정이다. B 는 549 행이다.

### 2.1 `없음` 22 건

#### (a) Depth Tree — A 의 중심 장치 전체 · 10 건

> 17 "**Split at natural joints, N layers deep.**"
> 18 "**A leaf is a real unit of work**: ten or more minutes of focused effort, one coherent deliverable."
> 19 "If your leaves come out smaller, you went one layer too deep; back off."
> 20 "**Contracts before fan-out.**"
> 21 "If leaves touch shared surfaces, write the interfaces, data ownership and naming into `PLAN.md` first."
> 22 "**Branches get gates too.**"
> 23 "Every internal node gets an integration gates file: children merged, interfaces match, cross-checks pass."
> 26 "Scale guidance: tree 2 or 3 for a feature, a bug hunt, a document, solo mode."
> 27 "Tree 4 or 5 for a subsystem or serious refactor."
> 28 "Tree 6 or 7 for an entire project built to a high bar, orchestrated, with leaves mapped to disjoint work units and parallelized where the harness allows."

- **행동**: 일을 리프까지 층으로 쪼개고, 리프마다 게이트 파일을 두고, 내부 노드마다 통합 게이트를 두고, 팬아웃 전에 인터페이스 계약을 적는다.
- **어느 절**: 새 절(§5 와 §7 사이) 또는 「오케스트레이션」 절 확장.
- **비용**: 표 하나 + 규칙 다섯 + 규모 지침 셋 → **25–40 줄.** B 에서 가장 비싼 후보다.
- ⚠ **B 는 회차 단위로 게이트를 두고 리프 개념이 없다.** 도입하면 §9 의 「원장 둘 대조」·「검산」이 리프마다 서야 한다 — 파급이 문면보다 크다.

#### (b) 모드 선택 기준 · 3 건

> 12 "The task fits one focused stretch: roughly under half an hour of real work, tree depth 3 or less."
> 14 "The task is a build: tree depth 4 or more, or clearly beyond one sitting."
> 63 "Below roughly half an hour of work, stay solo; subagent overhead only pays for itself on real builds."

- **행동**: 일의 크기로 solo/orchestrated 를 고른다. 작으면 위임하지 않는다.
- **어느 절**: 머리말 또는 「오케스트레이션」.
- **비용**: **4–6 줄.**
- ⚠ **B 에 반대 규범이 있다** — §5 교대 "한 회차를 한 컨텍스트에 맞추지 않는다. **의도에 맞춘다.**" 가져오면 정면으로 부딪힌다.

#### (c) 품질 패스 둘 — 전문가 상향 · 무료 광택 · 3 건

> 32 "Name the cheap version of each part, replace it with the good version."
> 36 "**Polish that costs nothing.**"
> 37 "Tuned constants beat new features."

- **행동**: 구현 후 각 부분의 싼 판을 이름 붙여 좋은 판으로 갈고, 비용 0 인 광택을 낸다.
- **어느 절**: §7 검증 앞 새 소절, 또는 §5 「발견의 처분」 옆.
- **비용**: **3–5 줄.**
- ⚠ **§11② 가 「미관 → 닫을 수 있다」로 명시**한다. 「무료 광택」은 그 칸과 부딪힌다.

#### (d) 개선 없는 패스를 종료선으로 · 1 건

> 38 "A pass that produces no improvement, plus a fully checked gates file, is the only finish line."

- **행동**: 개선이 안 나오는 패스가 한 번 나와야 끝낸다.
- **어느 절**: §11.
- **비용**: 0 줄 — **가져오면 안 된다.**
- ★ **B 가 이미 실측으로 반증했다** (§11 「끝을 정하는 자」, 네 계열 dry 0 회). 이것은 「빠진 것」이 아니라 **B 가 더 앞서 있는 자리**다.

#### (e) 공격선 전환의 문턱 · 2 건

> 47 "Before switching approach, state what the current one still has to give and why switching wins."
> 48 "If you cannot, keep going."

- **행동**: 접근을 바꾸기 전에 «지금 것이 아직 낼 것» + «바꾸는 편이 나은 까닭»을 말한다. 못 대면 계속한다.
- **어느 절**: §5 「막힘」 바로 앞.
- **비용**: **2–3 줄.** 싸다.
- B 의 §5 막힘(3 회)은 **그만둘 상한**이고 이것은 **그만두기 위한 조건**이라 겹치지 않는다.

#### (f) 값싼 모델 라우팅 · 1 건

> 62 "Mechanical leaves go to a cheaper model or lower effort where the harness allows it."

- **행동**: 기계적인 일은 싼 모델/낮은 노력으로 돌린다.
- **어느 절**: 「오케스트레이션」 표에 열 하나.
- **비용**: **1–2 줄.**

#### (g) 하네스 개입의 고지 · 1 건

> 66 "and tell the user what it does and how to remove it (`--uninstall`)."

- **행동**: 하네스를 바꾸는 설치물은 무엇을 하고 어떻게 지우는지 말한다.
- **어느 절**: §11 「금지역은 어디서 오나」의 `pal install` 각주.
- **비용**: **1 줄.**
- B 는 「아예 안 놓는다」로 회피했으므로 **지금은 필요 없을 수 있다.**

#### (h) 사소한 일 면제 · 1 건

> 68 "No gates file for a one-line fix."

- **행동**: 한 줄 수정에는 게이트 파일을 만들지 않는다.
- **어느 절**: 머리말 또는 §3.
- **비용**: **1–2 줄.**

### 2.2 `약함` 14 건

| # | A 원문 | 요구하는 행동 | B 의 어느 절 | 비용 |
|--:|---|---|---|--:|
| 5 | "give it a `CHECK:` line and an `EXPECT:` line so the check is runnable rather than a matter of opinion" | 조건마다 **실행 가능한 명령과 기대값**을 단다 | §3 「조건 한 줄의 형식」 — 토큰 둘 추가 | 4–6 줄 (+ `record.py`·`xtask` 파서 변경) |
| 8 | "only with the `EVIDENCE:` line replaced by actual proof: a measurement, a quote of output, a file path with the relevant line" | 판정마다 **증거**(측정치·출력 인용·파일:행)를 단다 | §3 조건 형식 · §9 `## 판정` | 3–5 줄 |
| 15 | "write `PLAN.md` plus one gates file per leaf under `gates/`, and run each leaf as a fresh subagent with a narrow brief" | 일을 리프로 갈라 리프마다 게이트를 두고 새 컨텍스트로 돌린다 | 오케스트레이션 (2.1(a) 와 한 덩어리) | (a) 에 포함 |
| 16 | "the verification hierarchy there (leaf checks itself, parent re-runs the checks)" | **부모가 자식의 검사를 다시 돌린다** | §7 검증 | 2–3 줄 |
| 31 | "**Re-read as a domain expert.**" | 작업 중 **자기 산출을 전문가 눈으로 재독**한다 (독립 리뷰 전에) | §7 앞 새 소절 | 2–3 줄 |
| 34 | "Edge cases, correctness, performance, the tells that something is fake." | 결함 사냥의 **모집단을 넷으로 명시** | §11② 해악 게이트 표 또는 §7 의도 축 | 1–2 줄 |
| 43 | "Open the gates file and pick the next unchecked box." | 잔액을 본 뒤 **다음 미판정 조건을 집는다** | §5 계기판 | 1–2 줄 (⚠ 계기판의 「아무것도 안 막는다」와 충돌 검토 필요) |
| 44 | "**When you feel finished, check instead of concluding.**" | **끝난 느낌**을 방아쇠로 삼아 결론 대신 검사한다 | §11 「끝을 정하는 자」 | 2 줄 |
| 45 | "re-read one passed gate adversarially and try to refute its evidence" | **통과한 게이트 하나를 골라 반박을 시도**한다 | §7 검증 (또는 §9 판정 뒤) | 2–3 줄 |
| 46 | "**Finish one line of attack.**" | 접근을 **바꾸기 전에** 지금 것을 끝낸다 | §5 막힘 앞 (2.1(e) 와 한 덩어리) | (e) 에 포함 |
| 54 | "**Full files, full lists, full sweeps.**" | 작업 대상 **전수**를 훑는다 (측정 출력만이 아니라) | §7 「측정할 때 걸리는 것 넷」 | 2 줄 |
| 55 | "the count opened must be 80, and you state that count" | **훑은 수 ↔ 선언한 모집단 수**를 검산한다 | §9 「검산」 확장 | 2–3 줄 |
| 56 | "Sampling is only acceptable when declared." | 표본을 뽑았으면 **선언**한다 | §3 `## 범위 밖` 또는 §9 | 1–2 줄 |
| 67 | "Conversational replies, trivial edits and factual questions get normal effort." | 사소한 일에는 규약을 **안 태운다** | 머리말 (2.1(h) 와 한 덩어리) | (h) 에 포함 |

---

## 3. 앞 세션 초안 일곱과의 차이

### 3.0 초안 일곱이 A 의 SKILL.md 에 실제로 있는가

**일곱 전부 SKILL.md 본문에 문장으로 있다.** `references/` 전용인 것은 없다.
다만 **상세는 참조 파일에 있고 이 대조는 그것을 못 봤다** —
`references/method.md`(분해), `references/orchestration.md`(검증 계층),
`references/token-economy.md`(토큰 경제), `templates/gates-leaf.md`(게이트 형식),
`scripts/gate-check.mjs`(체커), `scripts/install-hooks.mjs`(Stop 훅).
초안 항 1·5·7 의 세부는 그 파일들에 더 있을 수 있다. **못 봤으므로 못 셌다.**

### 3.1 초안에 있는데 전수에서 **대응물이 있다고 판명된 것** — 일곱 중 넷

| 초안 항 | 전수 판정 | B 의 대응물 |
|---|---|---|
| **3. "Do not simulate work you can do"** | **더 강함** (#49) · **있음** (#50) | §2 "**영향 범위는 상상하지 말고 조회한다**" · §6 "**추정을 조회인 척하지 않는다.**" · §8 "`pal` 을 직접 부르거나 스킬을 실제로 태워서 **물음 하나에 답을 받는다.**" |
| **4. "Ignore resource anxiety" + 정직한 인계** | **더 강함 ×3** (#51·52·53) | §5 "이월 사유가 **아닌** 것: 어렵다 · 시간이 없다 · 컨텍스트가 모자란다 · 생각보다 넓다." · §5 교대 "실패한 접근을 적는다" · §5 막힘 "축소로 위장해 완료를 선언하지 않는다." |
| **6. Report audit** | **더 강함** (#39), **있음** (#40·41) | §9 "수를 안 적고 조건 ID를 적는다" · §7 "자기가 쓴 것을 자기가 세면 그것은 검산이 아니라 항등식이다" — **B 가 A 보다 세다** |
| **7. 토큰 경제** | **더 강함** (#58·60), **있음** (#57·59·61) — 셋 중 **없음은 값싼 모델(#62)·30분 미만 solo(#63) 둘뿐** | §3 "세는 자리는 하나다" · 오케스트레이션 "큰 출력은 파일에 적고 요약만 돌려준다" · §5 교대 "직전 산출물을 시드로 주지 않는다" |

**초안 일곱 중 넷이 「이미 있다」 또는 「B 가 더 세다」로 뒤집혔다.**

### 3.2 초안 항이 **부분만** 맞은 것 — 셋

- **1. 네 패스** — 넷 중 **둘은 대응물이 있다**: "Implement completely"(#29 더 강함) · "No placeholders"(#30 더 강함) · "Hunt defects"(#33 있음) · "Fix what you find"(#35 있음). **빈 것은 「전문가 재독」(#31 약함) · 「싼 판→좋은 판」(#32 없음) · 「무료 광택」(#36·37 없음)** 이다. 「네 패스가 통째로 없다」는 틀렸다.
- **2. "When you feel finished, check instead of concluding"** — **약함**(#44·45). §11① "★ **상한에 닿는 것은 완료가 아니다.**" 와 §5 막힘 "시간·컨텍스트 상한에 닿은 것도 완료가 아니라 막힘이다." 가 **부분 대응**한다. 빈 것은 **「끝난 느낌」이라는 방아쇠**와 **통과한 게이트를 사후에 반박해 보는 행동**이다.
- **5. Full files / 80 이면 80 이고 그 수를 말한다** — **약함**(#54·55·56). §7 "전 출력을 파일로 받고 거기서 센다" · "**모집단이 비면 실패다**" · §9 "검산하고 적는다" 가 부분 대응한다. 빈 것은 **작업 대상 전수 훑기**와 **훑은 수 ↔ 선언 모집단 수의 검산**과 **표본 선언**이다.

### 3.3 초안에 **없는데** 전수가 새로 찾은 것

- **Depth Tree 장치 전체 10 건** (#17–23, 26–28) — 초안 1 번은 「네 패스」만 집었고 **트리·리프·계약·통합 게이트·규모 지침은 안 들어 있었다.** 없음 22 건의 **절반 가까이(10)** 가 여기다.
- **모드 선택 기준 3 건** (#12·14·63) — solo/orchestrated 를 일의 크기로 고르는 규칙. **B 의 §5 교대와 정면으로 부딪힌다**는 것도 새 발견이다.
- **공격선 전환 문턱 2 건** (#47·48) — 싸고(2–3 줄) B 와 안 부딪힌다. **가장 싼 후보.**
- **개선 없는 패스 = 종료선 (#38)** — 초안에 없었고, **B 가 이미 실측으로 반증한 자리**다. 「가져올 것」이 아니라 「A 가 뒤처진 것」으로 적어야 한다.
- **사소한 일 면제 2 건** (#67 약함·#68 없음) — B 본문에 규약을 안 태우는 조건이 없다.
- **`CHECK:`/`EXPECT:` 와 `EVIDENCE:` 두 칸** (#5·#8 약함) — 초안 어디에도 없었는데, **B 의 조건 한 줄 형식에 실행 가능한 검사와 증거를 다는 칸이 없다**는 것은 구조적 공백이다.
- **부모가 자식 검사를 재실행 (#16 약함)** · **값싼 모델 라우팅 (#62)** · **하네스 개입 고지 (#66)**.

### 3.4 한 줄 요약

**초안 일곱 중 넷은 이미 B 에 있거나 B 가 더 세고(3·4·6·7), 둘은 부분만 맞고(2·5),
하나는 절반만 맞다(1). 대신 전수가 초안이 못 본 것을 22 건 중 최소 18 건 새로 찾았고,
그 절반은 A 의 중심 장치인 Depth Tree 다.**
