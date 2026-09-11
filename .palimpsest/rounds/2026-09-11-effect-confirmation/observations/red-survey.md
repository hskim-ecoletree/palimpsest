# 실측 — 여섯 항목 · 저장소 `/Users/incognito/dev/projects/palimpsest` · HEAD `8604d62`

측정 시각 2026-09-11. 저장소는 읽기만 했다. `pal ledger` 는 `--cache-dir` 를 스크래치패드로
돌려 저장소에 한 바이트도 안 썼다 (`git status --porcelain` 이 측정 전후 같다).

## 0. 모집단 확정 — ⚠ 「스물한 회차」가 지금은 스물둘이다

| 세는 법 | 값 |
|---|---|
| `ls -1 .palimpsest/rounds/ \| wc -l` | **22** |
| `git ls-files .palimpsest/rounds/ \| wc -l` | **771** 파일 |
| `find .palimpsest/rounds -type f \| wc -l` (측정 시작) | **771** |
| `find .palimpsest/rounds -type f \| wc -l` (측정 중반) | **772** |

스물둘째 `2026-09-11-effect-confirmation/` 은 **git 에 없다**(`git status --porcelain` → `?? `).
측정 시작 시점(09:55)에는 빈 `observations/` 하나였고, 측정 중에 `observations/red.md`(10:09) ·
`intent.md` · `state.md` 가 **생겼다**. 다른 에이전트가 동시에 쓰고 있다.

**그러므로 아래 1·2 의 모집단은 「HEAD `8604d62` 에 커밋된 회차 21 개 · 파일 771 개」로 고정한다.**
확장자 내역: `.md` 344 · `.log` 208 · `.py` 125 · `.txt` 74 · `.jsonl` 19 · `.tsv` 1.

---

## 1. 「작업 전 계획 봉인 → 도구 산출 → 실제 변경과 대조」 — **0 건**

### 세는 법 — 여섯 개의 독립된 꼴 질의

| # | 꼴 | 정규식 / 명령 | 히트 |
|---|---|---|---|
| A | 낱말 | `grep -rniE '사전[ ]?등록\|pre-?regist\|preregist\|봉인\|seal(ed)?[ -]?(plan\|prediction)'` | **112 줄** |
| B | 보기 전 / 본 뒤 | `grep -rniE '보기[ ]?전\|돌리기[ ]?전\|본[ ]?뒤\|본[ ]?후\|before[ ]+(running\|seeing\|looking)'` | **47 줄** |
| C | 파일명 | `find .palimpsest/rounds -iname '*prereg*' -o -iname '*pre-reg*' -o -iname '*holdout*' -o -iname '*seal*'` | **4 경로** |
| D | 계획↔실제 대조표 | `grep -rniE '예측[^가-힣]{0,3}(↔\|대\|vs\|대조\|와 실제)\|계획[ ]?↔[ ]?실제\|예상[ ]?↔[ ]?실제\|\| *예측 *\|\|\| *예상 *\|\|predicted.*actual'` | **0** |
| E | 산출을 보고 바뀜 | `grep -rniE '산출을 보고\|출력을 보고\|화면을 보고' --include='*.md'` | **3 줄** |
| F | 작업 앞에 부름 | `grep -rn 'pal touch' .palimpsest/rounds/ --include='*.md' \| grep -E '전에\|앞에\|먼저'` | **5 줄**(그중 3 줄이 미커밋 회차) |

⚠ `grep -E` 가 ugrep 로 별칭돼 있어 한글 클래스가 섞인 긴 교차 정규식 둘이 `exceeds complexity
limits` 로 **에러**를 냈다. 그 둘은 더 짧은 질의로 쪼개 다시 돌렸다 (위 D·E).

### 무엇이 있었나 — 봉인은 **있다.** 봉인의 **대상이 다르다**

봉인 기록은 실재한다. 다만 전부 **「측정 규칙·판정 규칙」을 봉인**한 것이고, **「무엇이
바뀔 것인가」를 봉인한 것은 하나도 없다.**

1. **`2026-08-23-agent-laziness-behavior/exp/prereg/`** — 파일 12 개 + `oracle/` 디렉터리.
   `exp/PREREG-SHA.log` 전문은 한 줄 `1843feb`.
   - `plan.md:441` — *"★ 사전 등록 커밋 (이 커밋의 **해시를 판정문에 적는다**)"*
   - `plan.md:472` — *"`git diff <해시>..HEAD -- <사전등록물>` 을 첨부한다."*
   - `review/r7-raw.md:40` — *"`git log` — 사전 등록 `1843feb` 16:23 · 첫 원자료 `ccfc49c` 16:33. 앞선다"*
   - 봉인 대상 = 합격선 · 축 방향 · 3×3 격자 · 눈가림 · 탐지기 모집단 · 짝짓기(`rules.log`).
     **실험 결과를 보기 전에 채점 규칙을 잠근 것**이지, 코드 변경 예측이 아니다.
   - ★ 그리고 그 봉인은 **깨졌다** — `findings.jsonl:177` `IR5-1`:
     *"A10 이 잡으라고 세운 바로 그 사건이 일어났고, 증인이 그것을 못 봤다고 적었다"*
     (`control-saturation.log` 가 R4 처분에서 25 줄 늘고 1 줄 지워졌다).

2. **`2026-09-08-cross-file-references/observations/a7-denominator-preregistration.md`** ·
   **`…/a7-call-preregistration.md`** — 둘 다 머리에 같은 문장:
   > ⚠ **이 문서는 측정 전에 쓴다.** 값이 나온 뒤 규칙을 손보면 여러 번 뽑고 유리한 판을
   > 남기는 것과 같은 형태가 된다.
   봉인 대상 = 분모의 정의(M1~M4) · 「무엇이 경로 호출인가」의 판정 규칙 넷(C-1~C-4) ·
   음성 대조 조건 셋/넷. **역시 분모이지 변경 예측이 아니다.**

3. **`2026-08-18-completion-condition/retro/06-pendulum-metrics.md:3`** —
   > ★ **알려진 셋을 보기 전에 등록한다.** 이 문서가 커밋된 뒤에야 처치군에 돌린다.
   `:54` `## 홀드아웃 — 처치군에 돌리기 전에 (C5 단계 2)` · `:65` *"사후 맞춤을 막기 위해
   **처치군에 돌리기 전에** 후보를 적는다."* 봉인 대상 = 지표 후보 목록.

4. **`2026-09-06-terrain-and-completion-scene/dialectic/2-design.md:219`**
   `## 승격 문턱 — 결과를 보기 전에 등록한다` (`:190` · `2-synthesis.md:156` · `3-design.md:164`).
   봉인 대상 = 승격 문턱.

### 「없다」인가 「못 찾았다」인가 — **없다** (근거 셋)

- ⓐ **꼴 D(계획↔실제 대조표)가 771 파일에서 0 히트다.** 표 머리에 「예측」/「예상」 열을 둔
  마크다운 표가 회차 기록 전체에 **하나도 없다.**
- ⓑ **저장소가 자기 입으로 그렇게 적는다.** `docs/plan/03-shortest-path.md:139`:
  > **3** | **효과 확인** — 이 저장소에서 `pal touch` 를 실제 작업 앞에 부르고, 그 산출을 본
  > 뒤의 변경이 달라졌는지 잰다 | … **그리고 이 단계는 한 번도 실행된 적이 없다** | 그 산출을
  > 보고 달라진 변경 **1 건 이상**, 좌표와 함께
  (같은 파일 `:75` 도 *"남은 것은 「그 하나」가 아니라 **효과 확인**"*)
- ⓒ **가장 가까운 지시조차 「닫기 전」이지 「작업 앞」이 아니다.**
  `2026-09-06-user-surface-vocabulary/intent.md:17-18`(소유자 잠근 의도 인용 블록):
  > 이슈마다 커밋을 나누고 제목에 이슈 번호를 단다. **회차를 닫기 전에** `pal touch` 를
  > 실제로 한 번 실행한 출력을 `## 효과` 에 붙인다.
  = 사후 증거 부착이지 사전 봉인이 아니다.

⚠ **경계 사례 하나 — 판정은 안 한다, 문면만 싣는다.**
`docs/gates/rust-scope-references.md` §효과(`:334-336`, `:352-356`)는 **산출을 본 뒤에 코드를
고친 것**을 적는다. 다만 **그 앞에 봉인된 계획이 없다** — 「무엇이 나올 것인가」를 미리 적은
문서가 그 회차에 없다. 위 정의의 세 요소 중 ②③ 만 있고 ① 이 없다.

---

## 2. 게이트 `## 효과` 절 — 18 개 중 `pal touch` 출력은 **4 개**

### 세는 법
```
grep -nE '^#{1,4} *효과' docs/gates/*.md          # 절 머리 찾기
python3: 그 줄부터 다음 '^## ' 전까지를 잘라 본문으로 삼고 'pal touch' 포함 여부
```
`docs/gates/*.md` 전체 = **53 개**. `## 효과` 절이 있는 것 = **18 개**.
`^#+ .*효과` 로 넓혀도 18 개 + 오탐 1(`preflight.md:1882` `#### 1. M1은 효과 추정치가…`).

**회차↔게이트 대응**: 회차 슬러그로 게이트 이름을 맞추면 21 중 **17** 이 대응한다. 대응이
없는 회차 넷 = `round-protocol` · `check-verifies-work` · `agent-laziness-merge-evaluation` ·
`agent-laziness-merge-blockers`. 대응하는 17 게이트는 **전부** `## 효과` 를 가졌고, 여기에 회차
대응이 없는 `round-completion-current-aggregate.md` 하나가 더해져 18 이 된다.

### 18 개 전수 — 좌표와 길이

| 게이트 (`docs/gates/`) | `## 효과` 절 범위 | 줄수 | `pal touch` |
|---|---|---|---|
| agent-laziness-behavior.md | 251–327 | 77 | – |
| agent-laziness-executable-plan.md | 18–23 | 6 | – |
| agent-laziness.md | 196–249 | 54 | – |
| **cross-file-references.md** | **342–397** | 56 | **있음** |
| gate-parser-schema.md | 117–137 | 21 | – |
| inventory-disposal.md | 226–459 | 234 | – |
| legacy-round-debt.md | 55–59 | 5 | – |
| openmetadata-decision.md | 127–174 | 48 | – |
| round-approve-verify.md | 55–61 | 7 | – |
| round-completion-condition.md | 63–85 | 23 | – |
| round-completion-current-aggregate.md | 37–42 | 6 | – |
| round-finding-records.md | 134–165 | 32 | – |
| round-stop-progress-guard.md | 54–61 | 8 | – |
| round-verification-status.md | 43–51 | 9 | – |
| rust-extractor.md | 412–427 | 16 | – |
| **rust-scope-references.md** | **329–362** | 34 | **있음** |
| **terrain-and-completion-scene.md** | **147–195** | 49 | **있음** |
| **user-surface-vocabulary.md** | **273–467** | 195 | **있음** |

**4 / 18.** 넷 다 실행 출력을 펜스(```) 또는 들여쓰기 블록으로 **붙였고** 회차 관측 파일 전문을 링크한다.

| 게이트 | 실행한 명령 | 출력 좌표 | 전문 |
|---|---|---|---|
| cross-file-references.md:346 | `./target/release/pal touch print_facts` (2026-09-10) | `:349-354` 첫 답 · `:369-374` 고친 뒤 · `:381-385` 종료 시점 | `observations/effect-touch-print-facts.txt` · `…-after.txt` |
| rust-scope-references.md:337,342 | `./target/release/pal touch RefResolution` · `resolve_shadowing` | `:337-344` | `observations/effect-touch.txt` |
| terrain-and-completion-scene.md:179 | `pal touch check_completion_scenes` | `:178-190` | (본문 인용) |
| user-surface-vocabulary.md:281 | `pal touch SymbolKind` | `:279-306` | `observations/h1-touch.txt` |

### 그중 「그 산출을 보고 무엇이 달라졌다」를 적은 것 — **2 개** (판정 기준 명시)

**기준**: 절 안에 *산출을 본 것이 원인이 되어 무엇을 고쳤다* 는 인과 문장이 있는가
(표지: 「그래서 … 고쳤다/넣었다」 · 「…하게 한 자리다」 · 「고친 뒤의 답」 · 「요구했기 때문이다」).

**① `docs/gates/rust-scope-references.md:355-356`** — 가장 명시적이다:
> **그 0 을 그대로 두면 「아무도 안 부른다」로 읽힌다.** 그래서 `pal touch` 의 라벨에 줄을
> 하나 더 넣었다 … **효과 관측이 화면을 고치게 한 자리다.**

**② `docs/gates/cross-file-references.md:366-374`**:
> **`pal touch` 의 답만 그것을 안 싣고 있었다.** 고친 뒤의 답:
> ```text
> ■ 이 좌표에 걸린 것 (0)
>   **못 읽었습니다 — 「0 건」이 아닙니다.** 파생 저장소가 없습니다: ./.palimpsest/intent.redb
> ```
그리고 `:376-385` — *"★★ **그리고 고친 화면이 종료 시점에 실제로 답을 바꿨다.**"* →
`■ 이 좌표에 걸린 것 (1)`. *"**「0 건」이었던 자리에 결박이 나타났다.**"*

**경계 둘 — 안 세었다. 이유를 적는다.**
- `terrain-and-completion-scene.md:194-195`: 출력이 **주장을 반증**했으나 그 때문에 무엇을
  고쳤다는 문장이 없다. 반대로 *"그 효과가 가리키는 것이 **틀렸다**"* 로 끝난다:
  > ⚠ **그 출력이 지금 가리키는 첫 항목은 §2 의 1 번과 다른 축이다** — 효과는 산출됐고, 그
  > 효과가 가리키는 것이 틀렸다는 것을 정반합 판 3 이 잡았다. **효과가 없는 것과 효과가 틀린
  > 것은 다르다.**
- `user-surface-vocabulary.md:279-311`: 순서가 **반대다** — 먼저 고치고 나서 목표 상태 ⓐⓑⓒ 를
  출력으로 **확인**한다. 다만 같은 절 `:454-459` 에 인과 문장이 하나 있다:
  > ★ **펜스 안은 뺐다 — 검사가 증거를 고치라고 요구했기 때문이다.** … **검출 수단이 증거를
  > 고치라고 요구하면 그 수단이 틀린 것이다**
  고친 대상이 「`pal touch` 산출」이 아니라 「검사기」이므로 위 기준의 인과와 대상이 다르다.
  ⚠ **넓은 기준(대상 무관)으로 세면 3 이고, 좁은 기준(산출을 보고 산출/코드를 고침)으로 세면 2 다.**

---

## 3. `.claude/pal/policy.toml` — **없다**

```
$ ls -la .claude/pal/policy.toml
ls: .claude/pal/policy.toml: No such file or directory
$ ls -la .claude/pal/
ls: .claude/pal/: No such file or directory
$ find . -name 'policy.toml' -not -path './target/*'
(빈 출력)
```
디렉터리 `.claude/pal/` 자체가 없다. 저장소 어디에도 `policy.toml` 이라는 이름의 파일이 없다.
전문을 적을 것이 없다.

---

## 4. 이슈 #79 · #126 · #129

세는 법: `gh issue view <n> --json state,assignees,labels` · `gh issue view <n> --json body -q .body` 로
본문을 받아 좌표를 뽑고, 각 좌표를 `grep -rn` 으로 HEAD 에 대고 해소했다.

### (a) 상태 — 셋 다 열림 · 담당자 없음

| 이슈 | state | assignees | labels |
|---|---|---|---|
| #79 | **OPEN** | `[]` | `needs-triage` |
| #126 | **OPEN** | `[]` | `[]` (없음) |
| #129 | **OPEN** | `[]` | `[]` (없음) |
| (#66) | **OPEN** | `[]` | `needs-triage` |

### (b)(c) #79 — 「`identity_ceiling` 이 `min` 에 삼켜진다」 · 좌표 **전부 실재**

본문 원문: *"`nodes_of`(`ledger.rs:334`)가 `discriminator.identity_ceiling().min(s.identity)` 만
남기고 **ceiling 자체를 버린다.**"*

| 본문의 좌표 | 지금 | 어긋남 |
|---|---|---|
| `ledger.rs:334` | `crates/pal-cli/src/ledger.rs:334` | **줄 번호 정확.** 단 본문이 **크레이트 경로를 안 적었고** `ledger.rs` 는 저장소에 **셋**이다 (`crates/pal-core/src/ledger.rs` · `crates/pal-cli/src/ledger.rs` · `crates/pal-cli/src/round/ledger.rs`) — 애매하다 |
| `nodes_of` | `crates/pal-cli/src/ledger.rs:301` `pub(crate) fn nodes_of(` | 실재 |
| `discriminator.identity_ceiling().min(s.identity)` | `crates/pal-cli/src/ledger.rs:334` `identity: discriminator.identity_ceiling().min(s.identity),` | **원문 그대로 실재 · 안 고쳐졌다** |
| `identity_ceiling` 정의 | `crates/pal-core/src/coord.rs:259` `pub const fn identity_ceiling(&self) -> IdentityGrade` | 실재 |
| (인용) `crates/pal-cli/src/ledger.rs:294` | `/// [`Discriminator::identity_ceiling`] 이 그것을 강제한다.` | 실재 |
| `ExtractGrade::L1.identity() == Ordinal` | `crates/pal-core/src/coord.rs:342,346` 에 시험 있음 | 실재 |
| 수 「464 건」 · 「7,156 건」 | 본문에 산출 명령이 없다 — **되짚을 수단 없음**. 본문 자신이 *"손으로 센 수를 판정 표에 실으면 다음 회차가 그것을 못 되짚는다"* 라고 적는다 | 미검증 |

### (b)(c) #126 — 「`check_ledger_pair` 가 최근을 사전순으로 고른다」 · 좌표 **전부 실재 · 안 고쳐졌다**

본문이 준 심볼: `check_ledger_pair` · `cargo xtask check` · 판정문 문자열
*"최근 끝난 회차 `<슬러그>` 가 검사에 들었다"* · `docs/gates/README.md` 의 선언 「어색한 표현 교정 적용」.

| 본문의 좌표 | 지금 |
|---|---|
| `check_ledger_pair` 정의 | `xtask/src/main.rs:5851` `fn check_ledger_pair(root: &Path) -> Result<String> {` |
| 그 호출 | `xtask/src/main.rs:643` `("원장 둘 대조", check_ledger_pair(root)),` |
| 판정문 문자열 | `xtask/src/main.rs:6263` `format!("최근 끝난 회차 \`{회차}\` 가 검사에 들었다")` · `:6270` `… 가 검사 밖이다` |
| **결함 자체 (사전순)** | `xtask/src/main.rs:5865` `회차들.sort();` → `:6252-6258` `// ⑥ **하한 — 끝난 회차 중 가장 최근 것이 검사에 들었는가.**` / `let 최근_끝난 = 회차들 .iter() .rev() .find(\|회차\| 뿌리.join(회차).join("report.md").is_file()) .cloned();` |
| `docs/gates/README.md` 선언 | `docs/gates/README.md:140` `### 어색한 표현 교정 적용 — **이 목록도 방향이 반대다**` |

**`sort()` + `.rev().find()` = 사전순 최대**가 `5865`/`6254-6258` 에 그대로 있다. 시각 비교 없음.
⚠ 이 결함이 **실제로 발화한 적이 있다**: `2026-09-06-terrain-and-completion-scene/review/r1-raw.md:114`
가 같은 형태를 다른 함수(`xtask/src/main.rs:4666`)에서 **금지역**으로 잡았다.

### (b)(c) #129 — 「`narrative.unbound` 가 읽기 트랜잭션에서 쓴다」 · 좌표 **전부 실재**

본문에 `파일:줄` 이 하나도 없다 — 재현 명령과 에러 문자열뿐이다. 그 문자열로 해소했다.

| 본문이 준 것 | 지금 |
|---|---|
| `pal query narrative.unbound` (질의 이름) | `crates/pal-core/src/query_log.rs:73` `#[serde(rename = "narrative.unbound")]` · `:115` `Self::NarrativeUnbound => "narrative.unbound"` |
| 에러 `개체를 남기지 못했다` | `crates/pal-cli/src/narrative.rs:200` `intent.keep_entity(&origin, &id).context("개체를 남기지 못했다")?;` |
| 에러 `읽기로 연 의도 저장소에 쓰려 했다` | `crates/pal-intent/src/store.rs:169` `"읽기로 연 의도 저장소에 쓰려 했다".to_owned(),` |
| 쓰기 경로 | `crates/pal-intent/src/store.rs:163-171` `fn write(&self)` — `Handle::Absent \| Handle::Reading(_) => Err(IntentError::Transaction(…))` |
| 그 쓰기를 부르는 자 | `crates/pal-intent/src/store.rs:419-424` `pub fn keep_entity(&self, …)` 안 `let write = self.write()?;` |
| 결과 출력 자리 | `crates/pal-cli/src/query.rs:306` `print_narrative(unbound, *candidates, *bound, candidate_sizes);` |

**경로가 끊긴 데 없이 이어진다**: `query narrative.unbound` → `narrative.rs:200 keep_entity` →
`store.rs:424 self.write()` → `store.rs:167-171` 읽기 핸들이면 에러. **안 고쳐졌다.**
⚠ **재현은 안 돌렸다** — 읽기 전용 지시 때문. 코드 경로만 해소했다.
독립 증인 하나가 저장소 안에 있다: `crates/pal-cli/tests/query_envelope.rs:68`
`// 읽기로 연 의도 저장소에 쓰려 한다(F10 표면의 결함 · \`docs/gates/F12.md\`).`

---

## 5. #66 의 수 둘을 다시 잰다 — **7 → 3,306** · **1 파일 → 141 파일**

### 먼저: 「무슨 명령으로 그 수가 나왔나」 — 지목된 두 근거에 **없다**

#66 이 댄 근거 둘을 전문으로 읽었다:
- **`docs/gates/F11-touch.md` §10** (`:311-323`) — 전문에 **명령이 한 줄도 없다.**
  > `isAtOrAboveHome`에 결박 1(하한 1). **그런데 이 저장소에서 결박 가능한 좌표가
  > 7개뿐이고 파일이 1개다** — `corpus/tasks/f03-normalize-seeds.ts` 하나.
- **`corpus/criteria.toml` `[f11.pass].self_repo_grounds`** (`:10036-10046`) — 역시 명령이 없다.
  요구는 *"· **결박 ≥ 1** — palimpsest 자신에서 `pal touch` 가 결박을 하나 이상 낸다"* 뿐이고,
  **7 과 1 이라는 수는 이 합격선에 아예 안 적혀 있다.** 인접한 `self_repo_bound_min = 1` 만 있다.

**그러므로 「방법은 두 근거에서 못 찾았다」가 1 차 답이다.** 더 찾아본 것:
`grep -rn '결박 가능한 좌표' --include='*.md' .` → 같은 수를 쓰는 자리 **다섯**
(`docs/instructions/2026-08-20-owner-direction.md:35` · `docs/gates/F11-touch.md:313,418` ·
`docs/adr/0027-…:18` · `.palimpsest/rounds/2026-09-05-openmetadata-decision/premortem/r1-raw.md:38`)
— **다섯 다 명령을 안 적고 서로를 인용한다.** F11 이름의 회차 디렉터리는 없다(회차 21 중 0).

### 그래서 재구성했다 — 그리고 **옛 수를 정확히 재현한다**

```
$ ./target/release/pal ledger --symbols --cache-dir <스크래치>/cache .   # 3,306 줄
$ python3: 줄마다 json.loads → path 로 Counter
```
`pal ledger --symbols` 의 도움말이 *"대장이 아니라 **좌표를 붙인 심볼 전부**"* 라 적는다 =
「결박 가능한 좌표」의 조작적 정의.

**재현 검증**: 이 명령으로 `corpus/tasks/f03-normalize-seeds.ts` 만 세면 **정확히 7** 이다.
⇒ #66 이 쓴 방법이 이것(또는 이것과 동치)임을 **수가 증언한다**. (최상위만 세면 5 라 안 맞는다 —
**중첩 포함 전수**가 맞는 자다.)

### 그때 ↔ 지금

| | #66 (2026-08-20 이전) | **지금 `8604d62`** | 배수 |
|---|---|---|---|
| 결박 가능한 좌표 | **7** | **3,306** | ×472 |
| 그 좌표가 사는 파일 | **1** (`corpus/tasks/f03-normalize-seeds.ts`) | **141** | ×141 |

지금 141 파일의 내역: **`.rs` 140 · `.ts` 1**. 그 `.ts` 하나가 #66 이 적은 바로 그 파일이고,
오늘도 **7 개** 그대로다(파일은 `8월 13 13:13` 이후 안 바뀌었다). 즉 **더해진 것이 3,299 이고
전부 Rust 다.**

상위 파일: `xtask/src/main.rs` 223 · `crates/pal-store/src/projection.rs` 82 ·
`crates/pal-extract/src/rust.rs` 79 · `crates/pal-extract/src/typescript.rs` 79 ·
`crates/pal-core/src/plan.rs` 78.

### 둘째 방법으로 대조 — 값이 **안 갈린다**

```
$ ./target/release/pal ledger --json --cache-dir <스크래치>/cache .
```
`ledger.entries` **1,229** 파일 · `ledger.languages` **9 종**:

| 언어 | grade | identity | files |
|---|---|---|---|
| Markdown | l0 | unavailable | 557 |
| Python | l0 | unavailable | 165 |
| **Rust** | **l1** | ordinal | **140** |
| TOML | l0 | unavailable | 29 |
| Shell | l0 | unavailable | 6 |
| JSON | l0 | unavailable | 3 |
| YAML | l0 | unavailable | 2 |
| JavaScript | **l0** | **unavailable** | 1 |
| **TypeScript** | **l2** | exact | **1** |

**l0 아닌 것 = Rust 140 + TypeScript 1 = 141.** `--symbols` 의 파일 수와 **정확히 일치**한다.
캐시는 `hits 1229 · misses 0 · corrupt 0`, 워크트리 `trusted_from_index 1229 · rehashed 0`,
`detector.head_now = 8604d628…`.

⚠ **덤으로 드러난 것**: 이 저장소에 **JavaScript 파일이 1 개 있는데 grade 가 `l0`(unavailable)**
이다. Kotlin·Java 는 파일이 **0** 이라 언어 목록에 아예 안 뜬다.

---

## 6. `pal_core::Language` 는 지금 **다섯**을 담는다 — #66 은 **넷**이라 적는다

세는 법: `grep -rn 'pub enum Language' --include='*.rs' crates/` → 정의는 **한 자리**뿐.

**`crates/pal-core/src/language.rs:24-30`** — 전문:
```rust
pub enum Language {
    Kotlin,
    Java,
    JavaScript,
    TypeScript,
    Rust,
}
```

세 자리가 같은 다섯을 진다:
- `language.rs:41-45` `from_extension` — `"kt"|"kts"` · `"java"` · `"js"|"mjs"|"cjs"|"jsx"` ·
  `"ts"|"mts"|"cts"|"tsx"` · `"rs"`
- `language.rs:55` `from_name` — `[Self::Kotlin, Self::Java, Self::JavaScript, Self::TypeScript, Self::Rust]`
- `language.rs:61-67` `name()`

**#66 본문**: *"코어가 Rust이고 추출기는 Kotlin·Java·JavaScript·TypeScript 넷뿐이다(`pal_core::Language`)"*
→ **다섯째 `Rust` 가 빠졌다.** `Rust` 는 `docs/adr/0027-the-instrument-must-reach-its-own-floor.md`
(*"**Rust 를 다섯째 1급 언어로 올린다.**"*)로 들어왔고, 그 근거가
`docs/instructions/2026-08-20-owner-direction.md:34-45` 다.

⚠ **`Language` 의 다섯 ≠ 추출기의 수.** 위 5-③ 의 ledger 가 이 저장소에서 실제로 grade 를
받는 언어를 **둘**(Rust l1 · TypeScript l2)로 적는다.

---

## 갈린 것 (한 자리에 모음)

1. ⚠ **회차 수** — `ls` 는 22, `git ls-files` 는 21 개 회차. 스물둘째는 미커밋이고 **측정 중에도 자라고 있었다**(771 → 772 파일).
2. ⚠ **「그때 추출기가 몇이었나」가 문서 셋에서 셋 다 다르다.**
   - `#66` 본문 · `docs/gates/F11-touch.md:315` — *"추출기가 Kotlin·Java·JavaScript·TypeScript **넷**뿐"*
   - `docs/gates/F11-touch.md:318-319`(2026-08-20 각주) — *"Rust 가 다섯째 1급이 됐고 추출기는 **셋**이다"*
   - `docs/adr/0027-…:24` — *"코어가 Rust 인데 추출기는 Kotlin·TypeScript **둘**뿐이었다"*
   같은 시점의 같은 것을 **넷 · 셋 · 둘**로 적는다. 어느 하나를 고르지 않는다.
3. ⚠ **항목 2 의 「무엇이 달라졌다」 수가 기준에 따라 갈린다** — 좁은 기준 **2**, 넓은 기준 **3**. 위 §2 에 기준 둘을 다 적었다.
4. ⚠ **#79 의 `ledger.rs` 가 세 파일 중 어느 것인지 본문이 안 적는다.** 줄 번호(334)와 내용으로 `crates/pal-cli/src/ledger.rs` 로 해소했다.
5. **안 갈린 것**: #66 의 파일 수는 두 방법(`--symbols` 의 distinct path=141 · `--json` 의 l0-아닌 언어 파일 합=141)이 **같은 값**을 냈다.
