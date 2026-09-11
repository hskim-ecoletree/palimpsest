# 실측 — 다섯 항목 · `/Users/incognito/dev/projects/palimpsest` · HEAD `8604d62` · 2026-09-11 10:2x

저장소는 **읽기만** 했다. `git status --porcelain` 이 측정 전후 동일(`?? .palimpsest/rounds/2026-09-11-effect-confirmation/` 한 줄, 내가 만든 것 아님).
⚠ **단 하나의 부작용**: 항목 3 의 교차검증으로 `pal touch` 를 돌리면서 **gitignore 된 파생 저장소
`.palimpsest/index.redb` 가 재빌드됐다**(mtime 10:25). `pal ledger` 는 `--cache-dir` 를 스크래치패드로
돌려 1층 캐시를 저장소 밖에 뒀지만, `pal touch` 에는 그 플래그가 없다. git 추적 파일은 **0 바이트도 안 변했다**.

도구: `grep` = **ugrep 7.8.4**(별칭). `rg` = ripgrep 14.1.1. `python3` 3.13. `find`/`git ls-files`.
긴 교차 정규식은 ugrep 에서 죽을 수 있어 **모든 탐색을 `rg` 또는 `python3` 로 했다.** ugrep 을 1차 근거로 쓴 자리는 없다.

---

## 0. 모집단 — 두 셈이 갈린다

| 세는 법 (명령) | 회차 수 | 파일 수 |
|---|---:|---:|
| `find .palimpsest/rounds -mindepth 1 -maxdepth 1 -type d \| wc -l` / `find … -type f \| wc -l` | **22** | **775** |
| `git ls-files .palimpsest/rounds/ \| cut -d/ -f3 \| sort -u \| wc -l` / `git ls-files … \| wc -l` | **21** | **771** |

**갈리는 까닭**: 스물둘째 `2026-09-11-effect-confirmation/` 이 아직 커밋 안 됐고(`?? `),
그 디렉터리는 **측정 중에도 자란다**(다른 에이전트가 동시에 쓰고 있다 — 처음 셀 때 775, 재실행 시 값이 움직였다).

**아래 1 번은 디스크 22 회차 · 775 파일을 모집단으로 썼다**(질문이 「전부」라고 했다).
확장자 내역(`find … -type f | sed 's/.*\.//' | sort | uniq -c`): `md` 348 · `log` 208 · `py` 125 · `txt` 74 · `jsonl` 19 · `tsv` 1.

---

## 1. 회차 기록의 「사전 등록」 — 두 수를 갈라서

도구: `rg` (모든 질의) + `python3` (표 열 교차 · 절 추출). ugrep 미사용.

### 1-A. 「작업 전에 **무엇을 바꿀 것인지** 적어 두고 나중에 **실제 변경과 대조**한 기록」 — **0 건**

**내가 쓴 기준(꼴)** — 둘을 다 만족해야 1 건으로 센다:
- ⓐ 착수 전에 쓰였고, **변경 대상의 열거**(파일/좌표/심볼)를 담는다.
- ⓑ 같은 회차 안에서 그 열거를 **실제 변경(diff·산출)과 항목별로 대어** 맞았는지/틀렸는지를 적는다.

**ⓑ 를 만족하는 기록이 0 건이다.** ⓐ 만 만족하는 자리는 **넷**을 찾았고 전부 좌표를 적는다:

| # | 좌표 | 무엇 | ⓑ 있나 |
|---|---|---|---|
| A-1 | `docs/agent-laziness-executable-implementation-plan.md:249-277` §4.1 「첫 회차가 소유할 파일」 (14 행 표 · `파일`×`변경` 두 열) | 회차 `2026-08-30-agent-laziness-executable-plan` 이 **구현 착수 전에** 낸 변경 대상 표 | **부분.** 대조표는 없고 **사후 정정 메모 셋**만 있다 — `.palimpsest/rounds/2026-08-30-round-verification-status/state.md:23` · `:55` · `:68`. 예: `:55` *"P1 `pal-intent/src/lib.rs` 누락 → 계획 §4.1 좌표 정정 + compile check."* · `:68` *"값은 바꾸지 않고 `pal-core::budget` 단일 위치로 옮겼으며 계획 §4.1 소유 좌표를 정정했다."* → **어긋난 것을 고쳤다는 사실은 있으나 「몇 개 중 몇 개가 맞았나」가 없다** |
| A-2 | `.palimpsest/rounds/2026-08-23-check-verifies-work/state.md:78` `## (접기 전 기록) 만질 자리` (6 행 표) | 만질 파일 여섯과 각각 무엇을 할지 | **없다.** 회차가 접혔다(`folded.md`). 그 표를 실제 변경에 댄 기록이 없다 |
| A-3 | `.palimpsest/rounds/2026-09-02-agent-laziness-merge-blockers/GATES.md:3` `OWNS: crates/pal-cli/src/round/**, …` (10 개 글롭) | unlazy 스킬 꼴의 **변경 범위 선언** | **없다.** `OWNS` 를 읽는 코드가 저장소에 **없다**(`rg -n 'OWNS' --glob '!target' .` → GATES.md 둘 + 비교 문서 둘뿐) |
| A-4 | `.palimpsest/rounds/2026-09-01-agent-laziness-merge-evaluation/GATES.md:3` `OWNS: .palimpsest/rounds/2026-09-01-…/**` | 같음 | **없다** |

**경계 사례 둘 — 「바꿀 대상」이 아니라 「바뀐 뒤의 값」을 예고하고 나중에 잰 것.** 기준 ⓐ 를 엄밀히는 안 만족해서 1-A 에 안 셌다. 좌표를 적어 둔다:
- `.palimpsest/rounds/2026-08-18-inventory-disposal/intent.md:175` = `state.md:90-91`:
  > **★ `docs/gates/F06b.md:38` 이 MCP 없는 세계를 이미 예측해 뒀다** — `755 · 41줄` (`774 = 755 + 19` · `pal-mcp` 단위 5 + 세션 바이너리 14). **삭제 후 이 예측을 잰다.**
  → 실제: `state.md:181` *"`cargo build -p pal-cli` 뒤에 다시 돌리면 **755/755 rc=0**."* **예고 ↔ 실측이 대어졌다.**
- `.palimpsest/rounds/2026-08-20-rust-extractor/report.md:34-37`:
  > `### ★ 사전부검이 코드 없이 예측한 수 셋이 전부 맞았다` / *"처방 전 **24** · `attribute_item` 건너뛰기만 **69** · 접기까지 **43**. `[rust].what_was_already_known` ③ 에 *"이미 안다"* 를 적어 두었으므로 사후 기입이 아니다."*
  예고 출처: `premortem/r3-raw.md:16`(착수 전) · 등록처 `corpus/criteria.toml:11120` · 대조처 `corpus/criteria.toml:11226` *"(사전 등록 기대값: 접음 **43** · 안 접음 **69** …)"*

**거울 꼴 — 「무엇이 **안** 바뀔 것인가」를 등록하고 실제로 대조한 기록은 실재한다.** (묻지 않은 것이나 가장 가까운 꼴이라 적는다.)
`## 금지역` 절이 `intent.md`/`state.md`/`report.md` 에 **7 회차**(rg `'^#{1,4} .*금지역'`). 그중 실제 대조가 붙은 대표:
- `.palimpsest/rounds/2026-08-20-rust-extractor/intent.md:60-68`(등록) → `:111-117` 조건 F1~F5 전부 `통과 ⟨전사 2026-08-23⟩` → `report.md:45-48`:
  > `### 등록된 금지역이 지켜졌다` / *"Kotlin 골든 **대조 동일**(여덟 다 통과) · ditto 골든 **4,578 줄 움직인 것 0** · ditto 표식 **330 불변** · `xtask check` 20 · `xtask test` 전량."*

### 1-B. 「측정 규칙 · 분모 · 판정 기준을 **측정 전에** 적어 둔 기록」 — **12 건 / 7 회차**

**내가 쓴 기준**: 문서 또는 명확히 구획된 절이 ⓘ 측정 규칙·분모·표본 규칙·합격선·채점 규칙 중 하나를 고정하고, ⓙ **측정/채점/선정 전에 쓰였다는 표시**(문면 또는 커밋 해시)를 자기 안에 지닌다.

| # | 좌표 | 봉인 대상 | 「먼저」의 증거 (원문) |
|---|---|---|---|
| B1 | `…/2026-08-23-agent-laziness-behavior/exp/prereg/` (파일 12 + `oracle/`) + `exp/PREREG-SHA.log` (전문 한 줄 `1843feb`) | 합격선·축 방향·3×3 격자·눈가림·탐지기 모집단·짝짓기(`rules.log`) | `plan.md:441` *"★ 사전 등록 커밋 (이 커밋의 **해시를 판정문에 적는다**)"* · `plan.md:472` *"`git diff <해시>..HEAD -- <사전등록물>` 을 첨부한다."* |
| B2 | `…/2026-08-23-agent-laziness-behavior/intent.md:109` | 합격선 ①②③ | 절 제목 자체: `## 합격선 — **측정 전에 등록한다**` |
| B3 | `…/2026-08-23-agent-laziness-behavior/plan.md:96` `### ★ 탐지기의 모집단을 **여기서 등록한다** (R2-03)` | 탐지기 모집단 | 제목 |
| B4 | `…/2026-08-23-agent-laziness-behavior/plan.md:209` `## 오라클 다섯 축 — 방향 · **분모** · **동점** · **대조 불가**를 다 등록한다` | 오라클 분모·동점 규칙 | 제목 |
| B5 | `…/2026-09-08-cross-file-references/observations/a7-denominator-preregistration.md:1` | 분모 정의 `M1~M4` · 모집단 두 겹 · 음성 대조 셋 | `:10` *"⚠ **이 문서는 측정 전에 쓴다.** 값이 나온 뒤 규칙을 손보면 여러 번 뽑고 유리한 판을 남기는 것과 같은 형태가 된다."* + 머리에 커밋 `553a76c` |
| B6 | `…/2026-09-08-cross-file-references/observations/a7-call-preregistration.md:1` | 「경로 호출」 판정 규칙 `C-1~C-4` · 음성 대조 넷 | 같은 문장 + 커밋 `5bb5995` |
| B7 | `…/2026-09-08-cross-file-references/dialectic/1-design.md:55` `### 사전 등록 명령표 — 논증이 아니라 실행이 답하는 것` | 측정 명령 · 기준 커밋 `dab1743` · 산출물 네 칸 | `:53` *"논증하지 말고 돌려라. 아래를 **사전 등록 명령표**로 잠근다."* |
| B8 | `…/2026-08-18-completion-condition/retro/06-pendulum-metrics.md:1` | 진자 지표 후보 셋 | `:3` *"★ **알려진 셋을 보기 전에 등록한다.** 이 문서가 커밋된 뒤에야 처치군에 돌린다."* · `:54` `## 홀드아웃 — 처치군에 돌리기 전에 (C5 단계 2)` |
| B9 | `…/2026-08-18-completion-condition/retro/15-dialectic-sample.md:6` `## 표본 규칙 — 뽑기 전에 적는다` | 양성 3 · 음성 5 표본 규칙 | 제목 |
| B10 | `…/2026-08-18-completion-condition/retro/17-dialectic-scoring.md:1` | 채점 규칙 · 판정 규칙 C10 | `:3` *"**채점 전에 등록한다.** 결과를 보고 규칙을 고치면 실증이 아니다."* |
| B11 | `…/2026-09-05-openmetadata-decision/selection-rules.md:1` (+ `intent.md:29` `### A — 선정 (사전 등록)`) | 후보 선정 규칙 · 모집단 | `:1` `# 후보 선정 규칙 — **판정보다 먼저 등록한다**` · `:3` *"이 파일은 **후보를 고르기 전에** 커밋된다. … **여기에는 판정 결과가 없다** — 있으면 규칙이 결과를 따라간 것이다."* |
| B12 | `…/2026-09-07-rust-scope-references/observations/e1-hand-check.md:6` `## 표본 규칙 — **뽑기 전에 고정했다**` (+ `dialectic/1-design.md:121` `## 사전 등록 측정 — M1~M10`) | 층화 표본 50 의 배분과 집는 법 | *"난수를 안 쓴다 — 표본을 다시 뽑아 유리한 것을 고르는 길을 막는다."* |

**세는 법에 따라 흔들리는 자리**: `## 승격 문턱 — 결과를 보기 전에 등록한다` 가
`2026-09-06-terrain-and-completion-scene/dialectic/2-design.md:219` 와 `3-design.md:164` **둘**에 있다
(`1-design.md:174` 는 `## 승격 문턱 — 어느 결과가 소유자에게 올라가나` 로 **「먼저」 표시가 없다** — rg 로 재확인했다.
`2-synthesis.md:156` 은 등록문이 아니라 *"설계문이 결과를 보기 전에 등록한 승격 문턱 표를 그대로 적용한다"* 는 **적용문**이다).
「회차 1 건」으로 세면 12 → **13**, 「문서마다 1 건」으로 세면 12 → **14**. 위 표는 **그 둘을 아예 안 셌다**(승격 문턱은 측정 규칙이 아니라 **처분 규칙**이라 판단).

### ★ 1-A 와 1-B 의 관계 — 봉인은 있는데 **봉인의 대상이 다르다**
12 건 전부가 **「어떻게 잴 것인가」**를 봉인했고, **「무엇이 바뀔 것인가」**를 봉인한 것은 넷(A-1~A-4)뿐인데
그 넷 중 실제 변경과 대조된 것은 **없다**(A-1 만 부분).

### ★★ 갈린 자리 — 남이 낸 같은 조사와 모집단이 다르다
`.palimpsest/rounds/2026-09-11-effect-confirmation/observations/red-survey.md:1-96` 이 **같은 물음**을 먼저 쟀고
모집단을 **21 회차 · 771 파일**(git 추적)로 고정했다. 나는 **22 회차 · 775 파일**(디스크)로 쟀다.
결론(1-A = 0)은 같으나 **분모가 다르다.** 그 파일은 꼴 D(계획↔실제 대조표) 를 `grep` 으로 0 히트라 적는데,
나는 `python3` 로 표 행의 「예측/예상/계획」 열 × 「실제/실측/관측/결과」 열 교차를 다시 세어 **120 행**을 얻었고
전수 육안 판정 결과 **전부 산문 언급이지 계획↔실제 대조표가 아니었다.** 두 방법이 다른 중간값(0 vs 120)을 내고 **최종 판정만 같다.**

---

## 2. `docs/gates/` — 효과 절

도구: `find`/`ls` (모집단) · `rg -n '^#{1,4} .*효과'` (절 탐지) · `python3` (절 본문 추출 후 `'pal touch' in body`).

### (a) 게이트 문서 총 개수
- `ls -1 docs/gates/*.md | wc -l` → **53**
- `git ls-files docs/gates/ | wc -l` → **53** (두 셈 일치)
- `README.md` 를 「게이트」로 안 치면 → **52**. 아래 (b)(c)(d) 는 53 을 분모로 썼다.

### (b) `## 효과` 절이 있는 게이트 — **18 / 53**
`rg -n '^#{1,4} .*효과' docs/gates/*.md` 는 **19 줄**을 낸다. 그중 `docs/gates/preflight.md:1882` 은
`#### 1. M1은 효과 추정치가 아니라 검출률이다` 로 **「효과」 절이 아니다.** 빼면 18.
`rg -n '^## 효과' docs/gates/*.md | wc -l` → **18** (일치).

목록(파일:줄 · 절 길이): agent-laziness-behavior.md:251(77줄) · agent-laziness-executable-plan.md:18(6) ·
agent-laziness.md:196(54) · cross-file-references.md:342(56) · gate-parser-schema.md:117(21) ·
inventory-disposal.md:226(234) · legacy-round-debt.md:55(5) · openmetadata-decision.md:127(48) ·
round-approve-verify.md:55(7) · round-completion-condition.md:63(23) · round-completion-current-aggregate.md:37(6) ·
round-finding-records.md:134(32) · round-stop-progress-guard.md:54(8) · round-verification-status.md:43(9) ·
rust-extractor.md:412(16) · rust-scope-references.md:329(34) · terrain-and-completion-scene.md:147(49) ·
user-surface-vocabulary.md:273(195)

### (c) 그 절 안에 `pal touch` 의 출력이 붙은 게이트 — **4 / 18**
(`'pal touch'` 문자열이 절 본문에 있는 게이트가 정확히 이 넷이고, 넷 다 **실행 출력 블록**이 붙어 있다 — 명령만 적힌 자리는 없었다.)

| 좌표 | 붙은 출력 |
|---|---|
| `docs/gates/rust-scope-references.md:329` | `$ ./target/release/pal touch RefResolution` · `$ ./target/release/pal touch resolve_shadowing` — `호출자 17 · 피호출자 1` / `호출자 0 · 피호출자 4` |
| `docs/gates/cross-file-references.md:342` | `./target/release/pal touch print_facts` 의 **첫 답**(`■ 이 좌표에 걸린 것 (0)`)과 **고친 뒤의 답**과 **종료 시점의 답**(`(1)`) 셋 |
| `docs/gates/terrain-and-completion-scene.md:147` | `$ pal touch check_completion_scenes` — `■ 이 좌표에 걸린 것 (1)` |
| `docs/gates/user-surface-vocabulary.md:273` | `$ pal touch SymbolKind` — 전문(근거 상자 포함) |

참고로 `pal touch` 라는 문자열은 `docs/gates/` **15 파일**에 있다(`rg -l 'pal touch' docs/gates/ | wc -l`) — 절 밖이 대부분이다.

### (d) 「그 산출을 **보고 무엇이 달라졌다**」를 적은 것 — **2 / 4**

**내가 쓴 기준(먼저 적는다)**: 절 문면이 ⓧ 그 출력을 본 **뒤에** ⓨ **구체적 산출물이 실제로 바뀌었다**는 것을
ⓩ **인과로** 적어야 한다. 「출력이 이렇게 나왔다」·「착수 시점과 지금이 이렇게 다르다」(전후 관측)는 **안 센다** —
그 차이는 회차의 작업이 만든 것이지 **출력을 봐서** 만든 것이 아니다.

| 좌표 | 판정 | 원문 인용 |
|---|---|---|
| `docs/gates/rust-scope-references.md:356-358` | **센다** | *"**그 0 을 그대로 두면 「아무도 안 부른다」로 읽힌다.** 그래서 `pal touch` 의 라벨에 줄을 하나 더 넣었다 — *"`x.foo()` 와 `S::foo()` 는 아직 안 셉니다 … 그래서 0 은 「안 부른다」가 아니라 「이 층이 못 본다」일 수 있습니다"*. **효과 관측이 화면을 고치게 한 자리다.**"* |
| `docs/gates/cross-file-references.md:365-366, 379` | **센다** | `:365-366` *"★ **가르는 타입이 이미 있었다.** … **`pal touch` 의 답만 그것을 안 싣고 있었다.** 고친 뒤의 답:"* / `:379` *"★★ **그리고 고친 화면이 종료 시점에 실제로 답을 바꿨다.**"* |
| `docs/gates/terrain-and-completion-scene.md:172-176, 193` | **안 센다 (경계)** | *"…개정 38 이 그것을 `frontier.sh` 로 전환하며 *"`pal` 조회는 결박 대상 확장이 서야 성립한다"* 를 사유로 댔는데, 이 회차가 코드 심볼 둘에 실제로 결박해서 **그 사유가 사후에 반증됐다**"* · `:193` *"효과는 산출됐고, 그 효과가 가리키는 것이 틀렸다는 것을 정반합 판 3 이 잡았다."* → **바뀐 것은 판정이지 산출물이 아니다.** 기준 ⓨ 미충족 |
| `docs/gates/user-surface-vocabulary.md:312` | **안 센다** | *"착수 시점의 같은 자리는 「절단 없음(명시)」·「접힘 1049건이 다른 질의로 옮겨졌습니다」·「`ledger.snapshot` 가 폅니다」였다"* → **전후 관측**이지 「보고 바꿨다」가 아니다 |

⚠ **기준을 느슨히 하면 (d) 는 4 까지 간다**: 「출력이 무언가를 정정하게 했다」까지 세면 terrain 이 들어와 **3**,
「착수 대비 출력이 달라졌다」까지 세면 user-surface-vocabulary 도 들어와 **4**.
그리고 ★ **(d)=2 의 둘 다 「달라진 것」이 `pal touch` **자신의 화면**이다** — 「그 산출을 보고 **하려던 작업**이 달라졌다」는 자리는 **0 건**이다.

---

## 3. 결박 가능한 좌표 수 · 그 좌표가 사는 파일 수

캐시는 `--cache-dir /…/scratchpad/palcache` 로 저장소 밖에 뒀다.

| 방법 | 명령 | 심볼 | 파일 |
|---|---|---:|---:|
| **1** | `./target/release/pal ledger . --cache-dir <SC>/palcache --symbols` → 줄 수 / `id` 중복 제거 / `path` 유일값 | **3,306** (줄 3,306 · distinct `id` 3,306) | **141** |
| **2** | `pal ledger … --json` 에서 `parsed` 인 141 파일을 뽑아 각각 `./target/release/pal symbols --json <p>` 의 배열 길이를 합산 | **3,306** | **141** (오류 0 · 심볼 0 인 파일 0) |
| **3** | `./target/release/pal touch SymbolKind` 의 `■ 이 답의 근거` 상자 `:44` | `2층  심볼 **3306** 색인됨` | `대장 parsed **141** … / 1229 파일` |

**세 방법이 전부 3,306 / 141 로 같다.** 갈린 값 없음.

- 파일 141 의 내역: `.rs` **140** · `.ts` **1** (`corpus/tasks/f03-normalize-seeds.ts`).
- 심볼 종류 분포(`kind`): function 2400 · struct 267 · const 261 · module 202 · enum 148 · type_alias 11 · trait 7 · static 6 · class 1 · method 1 · interface 1 · macro 1.
- 분모: 저장소 전체 **1,229 파일**. 그중 `parsed` 141 · `unsupported` 763 · `unrecognized` 324 · `binary` 1.
  → **결박 가능한 파일은 1,229 중 141(11.5%)이다.**
- ⚠ `pal symbols --help` 는 *"파일 하나의 **최상위** 심볼을 산출한다"* 라고 적지만 실측 합이 `--symbols`(중첩 포함)와 **정확히 같다** — 문면과 산출이 어긋나는 자리다.

---

## 4. `Language` enum ↔ `pal ledger` 가 등급을 매기는 언어

### (가) `crates/pal-core/src/language.rs:24` `pub enum Language` — **변종 다섯**
```
pub enum Language {
    Kotlin,
    Java,
    JavaScript,
    TypeScript,
    Rust,
}
```
(명령: `awk '/pub enum Language/,/^}/' crates/pal-core/src/language.rs`. 같은 파일 `:83-87` 의 `ALL` 배열도 같은 다섯.)

### (나) 이 저장소에서 `pal ledger` 가 실제로 등급을 매기는 언어 — **아홉**
(명령: `./target/release/pal ledger . --cache-dir <SC>/palcache`)

| 언어 | 등급 | 파일 수 | enum 에 있나 |
|---|---|---:|---|
| Markdown | `L0 unavailable` (결박 불가) | 557 | ✗ |
| Python | `L0 unavailable` | 165 | ✗ |
| **Rust** | **`L1 ordinal`** (구조 · 선언 순서) | **140** | ✓ |
| TOML | `L0 unavailable` | 29 | ✗ |
| Shell | `L0 unavailable` | 6 | ✗ |
| JSON | `L0 unavailable` | 3 | ✗ |
| YAML | `L0 unavailable` | 2 | ✗ |
| **JavaScript** | `L0 unavailable` | 1 | ✓ |
| **TypeScript** | **`L2 exact`** (참조 해소 · 정확) | **1** | ✓ |
| (언어 미인식 `unrecognized`) | — | 324 | — |
| (이진) | — | 1 | — |
| **합** | | **1,229** | |

### (다) 둘이 같은가 — **다르다**
- **enum 5 ⊄ 등급 9, 등급 9 ⊄ enum 5.** 교집합은 **셋**(Rust · TypeScript · JavaScript).
- enum 에 있는데 이 저장소에 파일이 **0** 인 것: **Kotlin · Java**.
- 등급이 매겨지는데 enum 에 **없는** 것: **여섯**(Markdown · Python · TOML · Shell · JSON · YAML).
- **까닭(실측)**: 언어 이름 표는 enum 이 아니라 `crates/pal-extract/src/recognize.rs:34` `BY_EXTENSION`
  (24 개 확장자 → SQL·Markdown·JSON·YAML·XML·TOML·Java Properties·Gradle·Shell·Batch·HTML·CSS·SCSS·Python·Go·Ruby·Svelte·Vue)
  + `:69` `BY_FILE_NAME`(Dockerfile·Make·Groovy)가 진다. 그 파일 머리 주석이 스스로 적는다 —
  *"여기 있는 동안은 언어를 늘리는 일이 코드를 고치는 일이다 — 그 사실을 숨기지 않는다."*
  enum 은 **「1급 다섯」**이고 등급표는 **「인식하는 전부」**라 두 목록은 애초에 다른 것을 센다.

---

## 5. 「추출기가 몇 개였나/몇 개인가」를 수로 적은 자리 — **어느 것이 맞는지는 판정 안 한다**

도구: `python3` (정규식 넷 — `추출기(는|가|…)\s*N\s*(뿐|이다|…)` · `N\s*(개|종)?\s*추출기` · 언어이름 나열 × 수 낱말의 근접 교차). `rg` 로 교차검증.
모집단: `target/` · `.git/` · `node_modules/` 를 뺀 저장소 전부(`.md .rs .py .toml .sh .txt .json .yml .yaml .jsonl`).

### 「지금 몇이다」를 **현재형**으로 적은 자리 — **수가 셋으로 갈린다 (넷 · 셋 · 둘 · 하나)**

| 좌표 | 적은 수 | 원문 인용 |
|---|---:|---|
| `crates/pal-core/src/capable.rs:13` | **셋** | *"지원 언어가 다섯인데 (Kotlin·Java·JavaScript·TypeScript·Rust — 지시 2026-08-12 §1 · 2026-08-20 §1) **추출기는 셋뿐이라**,"* |
| `docs/overview.md:1026` | **셋(3종)** | *"1급 언어 \| 5종 (Kotlin·Java·JavaScript·TypeScript·Rust) — 그러나 **추출기는 3종**(Kotlin·TypeScript·Rust). 나머지 둘은 `NotBuilt`"* |
| `docs/gates/F11-touch.md:318` (각주) | **셋** | *"↳ **2026-08-20 이 갱신했다 — Rust 가 다섯째 1급이 됐고 추출기는 셋이다**"* |
| `docs/gates/F12.md:305` (각주) | **셋** | 같은 문장 |
| `docs/plan/03-shortest-path.md:82` | **셋** | *"### 3.1 막힌 자리는 **추출기 셋이다**"* |
| `docs/gates/F11-touch.md:316` | **넷** | *"★ **코어가 Rust이고 추출기가 Kotlin·Java·JavaScript·TypeScript 넷뿐이다.**"* |
| `scripts/f12-verify.py:559` | **넷** | *"추출기는 Kotlin·Java·JS·TS **넷뿐이다.** [ADR-0017] 의 **「자가 짧다」**이고 "* |
| `corpus/criteria.toml:10520` | **넷** | *"식별자**이고, 이 빌드의 추출기는 Kotlin·Java·JavaScript·TypeScript **넷뿐이다.**"* |
| `corpus/criteria.toml:10799` | **넷** | *"그것이 지목하는 것은 **Rust 식별자**이고 추출기는 Kotlin·Java·JS·TS **넷뿐**이다"* |
| `docs/gates/F02-1-extractor.md:179` | **넷** | *"추출기는 **넷**에 대한 규칙을 갖고 있고 단위 시험이 그것을 센다."* |
| `docs/gates/inventory-disposal.md:450` | **넷** | *"**Rust 추출기** — `pal_core::Language` 가 Kotlin·Java·JS·TS **넷뿐**이고 `pal ledger` 가 `Rust L0 결박 불가 116 파일` 을 낸다"* |
| `scripts/f22-3-verify.py:171` | **하나** | *"**대조 불가** — T10 의 코퍼스는 TypeScript 이고 **이 빌드의 추출기는 Kotlin 하나다.**"* |
| `crates/pal-core/src/file_graph.rs:123` · `:158` · `scope.rs:369` · `parse.rs:124` · `lib.rs:150` | **둘** | 예: `file_graph.rs:158` *"⚠ **오늘 두 추출기가 다 [`Slot::Built`] 로 산출한다**"* · `parse.rs:124` *"지금 **두 추출기** 다 L1(구조)이라 스코프가 없다"* |
| `crates/pal-extract/src/classify.rs:260` | **셋**(여집합으로) | *"**남은 둘(Java · JavaScript)은 추출기가 없으므로** 이 함수에 도달하지 않는다."* |

### 「그때 몇이었다」를 **과거형**으로 적은 자리

| 좌표 | 적은 수 | 원문 인용 |
|---|---:|---|
| `docs/adr/0027-the-instrument-must-reach-its-own-floor.md:22` | **둘** | *"**코어가 Rust 인데 추출기는 Kotlin·TypeScript 둘뿐이었다.** 그래서 *"이 도구가 자기…"* |
| `.palimpsest/rounds/2026-08-20-rust-extractor/intent.md:174` | **둘** | *"\| 사전부검 R1 \| 「기존 **세** 언어 회귀」 → **두** 언어. **이 빌드의 추출기는 둘이다** \|"* |
| `.palimpsest/rounds/2026-08-20-rust-extractor/findings.jsonl:11` (`PM1-S10`) | **둘** | `"요약": "「기존 세 언어 회귀」— 이 빌드의 추출기는 둘뿐이다"` · `"해악도": "금지역"` · `"처분": "정정"` |
| `corpus/criteria.toml:11237` | **둘** | *"⚠ **「세 언어」가 아니다.** 이 빌드의 추출기는 Kotlin·TypeScript **둘**이고"* |
| `.palimpsest/rounds/2026-08-18-inventory-disposal/state.md:307` | **넷** | *"★ **원리상 못 한다.** 이 저장소는 Rust 이고 **추출기는 넷(Kotlin·Java·JS·TS)뿐이다.**"* |

### 이 갈림을 **저장소 자신이 이미 적어 둔** 자리 (판정 아님 · 기록)

| 좌표 | 원문 |
|---|---|
| `.palimpsest/rounds/2026-08-20-rust-extractor/findings.jsonl:105` (`IR1-R06`) | `"요약": "capable.rs 가 「추출기는 셋뿐」이라 적는데 다섯이다"` · `"유효성": "거짓"` · `"처분": "기각"` |
| `.palimpsest/rounds/2026-08-20-rust-extractor/review/r1-raw.md:82` | *"\| 6 \| `capable.rs` 가 「추출기는 셋뿐」이라 적는데 다섯이다 \| 저장소 \| 거짓 \| — \| `capable.rs:11-13` \| **「선언된 1급 다섯 · 만들어진 추출기 셋 · 나머지 둘」이 정확하다** \|"* |
| `docs/gates/rust-extractor.md:444` | *"- **`corpus/criteria.toml:10516` 이 「추출기는 넷뿐」을 현재형으로 말한다** —"* |
| `.palimpsest/rounds/2026-08-20-rust-extractor/intent.md:163` · `premortem/r2-raw.md:135` · `findings.jsonl:44`(`PM2-S14`) | *"동결된 판정 문서와 `corpus/` 가 *"추출기는 넷뿐이다"* 를 현재형으로 말한 채 남고, **어떤 검사도 안 본다**"* · `"처분": "범위밖"` |
| `corpus/criteria.toml:11282` | *"· **동결된 판정 문서와 이 파일 자신**이 「추출기는 넷뿐」을 현재형으로 말하는 것 —"* |
| `.palimpsest/rounds/2026-09-11-effect-confirmation/intent.md:137` (미커밋 회차) | *"- [ ] **D1** `#66` 본문의 낡은 전제 둘 — 「결박 가능한 좌표 7 개 · 파일 1 개」 · 「추출기는 Kotlin·Java·JavaScript·TypeScript 넷뿐」 — 을 **실측값으로 정정했다.**"* (아직 미체크) |

### 오늘 HEAD `8604d62` 의 코드가 내는 수 — **셋** (판정이 아니라 측정)
`crates/pal-extract/src/extractor.rs:74-80` 의 `extractor_for` 가 언어 다섯에 대해 내는 것:
```
Language::Kotlin     => Capable::Present(&crate::kotlin::KOTLIN)
Language::Java       => Capable::not_built(CapabilityId::new("F02", "java-extraction"))
Language::JavaScript => { … not_built … }
Language::TypeScript => Capable::Present(&crate::typescript::TYPESCRIPT)
Language::Rust       => Capable::Present(&crate::rust::RUST)
```
→ `Capable::Present` 가 **셋**(Kotlin · TypeScript · Rust), `not_built` 가 **둘**(Java · JavaScript).
소스 모듈로 세도 `kotlin.rs` · `typescript.rs` · `rust.rs` **셋**.
**이 수와 위 표의 「넷」·「둘」·「하나」가 어긋난다는 사실만 적는다. 어느 것이 맞는지는 판정하지 않는다.**
