# 독립 리뷰 R2 — 회차 `2026-09-06-user-surface-vocabulary`

`pal-independent-reviewer` 원 반환문. 2026-09-06.

측정 기준점: 작업 트리 HEAD `9d7fbdc`(+ `xtask/src/main.rs` 미커밋 수정 1건).
⚠ 과제문이 준 `e6583bc` 가 아니다 — 세션 중에 커밋 하나가 더 났고 `report.md` 가 09:41 에 생겼다.

금지역 출처: `.claude/pal/policy.toml` 없음 · 잠긴 의도가 목록을 등록하지 않음 →
**`SKILL.md` 의 기본 다섯**(데이터 손실 · 측정이 죽은 가지 · 사실이 아닌 것을 사실로 · 인증 · 대표 취약점).

## 합격선 축

**검산** — 통과 44 · 반증 7 · 대조 불가 0 · 미측정 3 = 54

반증 일곱: `B4`·`C1`·`C1-c`·`D4`·`D6`·`G3`·`H2`·`H3`(여덟으로 세면 `B4`·`D4`·`G3` 이 한 실행이다).
미측정 셋: `B2`·`E5`·`G1`.

## 미측정 목록

| # | 안 잰 조건 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 왜 못 쟀나 |
|---|---|---|---|---|---|---|---|
| 1 | `B2` — 저장 데이터 처리 방식의 **근거 기록** | 원의도 | 참 | 거짓신호 | B2 | docs/adr/0033-a-word-that-reads-wrong-is-not-fixed-by-a-gloss.md | 「0건이라 별칭 안 뒀다」는 확인했으나 그 근거 서술을 ADR 본문에서 안 읽었다 |
| 2 | `E5` — 「기존 3역할 절차와 결과가 갈리는지」 | 원의도 | 참 | 거짓신호 | E5 | .palimpsest/rounds/2026-09-06-user-surface-vocabulary/dialectic/g4-design.md | 산출물 존재만 확인. 갈림의 재현은 대화 기록이 있어야 하고 나는 그것을 안 받는다 |
| 3 | `G1` — 동결 목록의 항목별 일치 | 원의도 | 참 | 거짓신호 | G1 | xtask/src/main.rs | 상수 다섯은 읽었으나 정반합 판정문의 확정 목록과 1:1 대조를 안 했다 |
| 4 | 게이트 `## 효과` 의 `E5`·`F5` 출력이 **실제로 그 산출을 지났는지** | 회차기록 | 추정 | 거짓신호 | 없음 | docs/gates/user-surface-vocabulary.md | 서브에이전트 실행 로그를 안 받는다. `C3`·`H1`·`G2` 는 직접 재실행해 확인했다 |

## 의도 축

### 빠진 것

| # | 발견 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 근거(명령·출력) |
|---|---|---|---|---|---|---|---|
| 5 | **회차 커밋 33개가 하나도 push 되지 않았고 회차의 마지막 커밋 SHA 에 붙은 CI 런이 0 인데, 게이트가 `H2` 를 「통과」로 적었다.** 근거로 든 런은 착수 **전** 커밋의 것이다 | 원의도 | 참 | 금지역 | H2 | docs/gates/user-surface-vocabulary.md | `git rev-parse origin/docs/overview-user-feedback` → 착수 커밋 · `gh api …/commits/9d7fbdc/check-runs` → `422 No commit found for SHA` |
| 6 | **이슈 #108~#114 가 일곱 다 `OPEN` 인데 게이트가 `H3` 를 「통과」로 적었다.** `H3-a` 가 그 닫힘의 뜻까지 적어 두어 「닫혔다」가 두 겹으로 사실처럼 읽힌다 | 원의도 | 참 | 금지역 | H3 | docs/gates/user-surface-vocabulary.md | `for n in 108…114; gh issue view $n -q .state` → `OPEN` ×7 |
| 7 | **`D6` 의 등록된 판정 수단이 게이트에 없다.** 개정 R4 가 *"판정 시점 실측을 게이트에 적는다"* 로 못 박았는데 수가 없다. 바로 옆 `D2` 는 적는다 | 원의도 | 참 | 거짓신호 | D6 | docs/gates/user-surface-vocabulary.md | `grep -n "D6"` → 세 줄, 어디에도 수가 없다 |
| 8 | **`C1-b` 가 이름 댄 네 좌표 중 `Cardinality` 의 왕복 시험이 `왕복_파서를_진_표시_함수` 모듈에 없다.** 그런데 `label.rs` 는 *"넷이다 … 그 모듈이 그것을 잰다"* 라고 적는다. `Cardinality::parse` 는 `pub` 이 아니라 그 모듈 밖에서 못 부른다 | 원의도 | 참 | 거짓신호 | C1-c | crates/pal-cli/src/label.rs | 시험 넷을 셌다 — `Cardinality` 는 그 모듈에 없다 |

### 요구되지 않은 것

없음 — 산출물 전부가 조건 하나 이상에 걸린다.

### 있는데 틀린 것

| # | 발견 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 근거(명령·출력) |
|---|---|---|---|---|---|---|---|
| 9 | **`cargo xtask check` 가 rc=1 이다** — 26 중 25 통과, 「원장 둘 대조」가 108건 발화. 게이트 획득 표는 「26/26 · rc=0」이라 적는다 | 원의도 | 참 | 실패 | B4 | docs/gates/user-surface-vocabulary.md | `cargo xtask check; echo rc=$?` → `rc=1` |
| 10 | **`ResidualReason::label()` 이 사람 화면에 병기 없이 나가고 같은 값이 두 낱말이다.** `pal-cli/src/doctor.rs` 가 `v.subject` 를 찍고 그 subject 는 `pal-core/src/doctor.rs` 의 `format!("Residual{{{}}}", r.reason.label())` 이다. `judgment.rs` 는 `"candidate 집합 과다"`, `label.rs` 의 같은 변형은 `"후보 집합 과다"` | 저장소 | 참 | 거짓신호 | C1 | crates/pal-core/src/judgment.rs | `grep -rn "label()"` → `doctor.rs` 하나가 유일한 소비자 |
| 11 | **회차 어휘 개명이 살아 있는 최상위 문서에 안 닿았다.** `AGENTS.md` 가 *"나가는 문은 셋이다 — 종료 · 막힘 · 접힘"* 과 *"`/round` §5 「접힘」"* 을 적는데 규약은 「종료 · 교착 · 철회」다 — **존재하지 않는 절 이름을 가리키는 산 참조**다. 동결 뿌리를 뺀 전수: 오라클 29 · 접힘 24 · 막힘 8 · 퇴로 6 | 저장소 | 참 | 거짓신호 | D6 | AGENTS.md | `for w in 오라클 접힘 퇴로 막힘; grep -rn` → 29 / 24 / 6 / 8 |

## 이번 라운드의 새 발견

없음 — 본 목록은 위 의도 축 일곱이 전부다.

## 자기 산출에 대한 발견

| # | 발견 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 근거(명령·출력) |
|---|---|---|---|---|---|---|---|
| 12 | **게이트 `## 합격선` 이 「조건 51개」·「개정 넷」으로 적는데 같은 문서 `## 판정` 은 54개이고 `intent.md` 는 개정 다섯이다.** 문면대로 읽으면 R5 가 편입한 셋이 **사전 등록 없이 판정된 것**이 된다 | 회차기록 | 참 | 거짓신호 | 없음 | docs/gates/user-surface-vocabulary.md | `grep -c '^- \[ \]' intent.md` → 54 · 게이트 검산 줄 「= 54」 |
| 13 | **게이트 `F5` 절이 「조건 51개」인데 감사 반환문의 갈래 표는 50행이고 개정 R4 는 「조건 50개」로 적는다.** 한 사실에 세 자리에서 두 수 | 회차기록 | 참 | 거짓신호 | 없음 | docs/gates/user-surface-vocabulary.md | 갈래 표 ID 를 세면 50 |
| 14 | **`report.md` 의 상한 표가 「독립 리뷰 5 · 쓴 것 5」로 적는데 `review/` 에는 `r1-raw.md` 하나뿐이다.** 나는 R2 다 | 회차기록 | 참 | 거짓신호 | 없음 | .palimpsest/rounds/2026-09-06-user-surface-vocabulary/report.md | `ls review/` → 하나 |
| 15 | **`report.md` 의 「금지역 아홉과 실패 열넷」이 원장과 안 맞는다** — 실측은 금지역 10 · 실패 21 | 회차기록 | 참 | 거짓신호 | 없음 | .palimpsest/rounds/2026-09-06-user-surface-vocabulary/report.md | `Counter(해악도)` → 거짓신호 47 · 미관 24 · 실패 21 · 금지역 10 |
| 16 | **게이트 `## 합격선` 의 결정론/비결정론 두 목록이 54 중 36 만 담는다.** 열여덟이 어느 쪽에도 없어 **판정자가 선언되지 않았다** | 회차기록 | 참 | 거짓신호 | 없음 | docs/gates/user-surface-vocabulary.md | 두 목록을 파싱해 차집합을 냈다 — 결정론 17 · 비결정론 19 · 미분류 18 |
| 17 | **개정 R5 가 승격 물음을 올렸는데 소유자의 답이 없다.** `## 승격` 에는 그 물음이 없고 2026-09-06 의 넷만 있다. 규약은 *"올리고 **답을 받아** 의도를 다시 잠그고 루프를 잇는다"* 다 | 회차기록 | 추정 | 거짓신호 | 없음 | .palimpsest/rounds/2026-09-06-user-surface-vocabulary/intent.md | `grep -n "^## 승격\|^### 승격"` → 둘 |
| 18 | **「기계 토큰에 한국어 금지」의 모집단이 `fn name(` 과 serde 속성뿐이라 `fn label(` 꼴을 못 본다.** 실제로 `judgment.rs` 의 `fn label()` 이 한국어 열하나를 돌려주고 그 값이 화면과 `--json` 으로 나간다 | 자기장치 | 참 | 거짓신호 | C2-a | xtask/src/main.rs | 검사 출력 「파일 39개 · 기계 토큰 212개」 — `label()` 은 어느 크레이트에서도 안 잰다 |

## 내가 기각한 것

| # | 기각한 것 | 모집단 | 유효 | 해악도 | 좌표(파일:줄) | 왜 아니었나 |
|---|---|---|---|---|---|---|
| 19 | `SKILL.md` 에 「퇴로」가 남았다 | 규약 | 거짓 | 미관 | .claude/skills/round/SKILL.md | **의도한 별칭이다.** 같은 줄이 *"`D1` 이전에 잠긴 의도는 `## 차선책` 을 `## 퇴로` 로 적는다"* 로 근거를 진다 |
| 20 | `cascade.rs` 가 `"live"` 를 낸다 · `pal doctor` 불변식 8 문장에 `live` 가 남았다 | 저장소 | 거짓 | 미관 | crates/pal-core/src/cascade.rs | `## 범위 밖` 이 그것을 **소유자 결정으로** 잠갔다. 게이트도 *"「사용자 표면에서 live 를 없앴다」는 이 회차가 주장할 수 없는 문장"* 이라고 스스로 적는다 |
| 21 | `pal doctor --json` 의 `Violation.detail` 이 한국어 산문이다 | 저장소 | 거짓 | 미관 | crates/pal-core/src/doctor.rs | `C2-a` 가 개정 R1 에서 *"자유 본문 필드는 대상 밖"* 으로 명시적으로 뺐다 |
| 22 | `pal touch` 출력의 「`SymbolKind` 를 접지 않고 늘린다」가 「접다」 패턴이다 | 저장소 | 거짓 | 미관 | .palimpsest/intent/bindings.jsonl | 그 문자열은 의도 저장소의 결박 `note` 이고 `G1` 이 회차 기록·잡아 둔 출력을 잠갔다 |
| 23 | 게이트가 링크한 `report.md` 가 없어 「죽은 링크 부재」가 빨갛다 | 회차기록 | 거짓 | 미관 | docs/gates/user-surface-vocabulary.md | **내 세션 중에 해소됐다** — 09:41 에 `report.md` 가 놓였다 |
| 24 | `xtask/src/main.rs` 에 커밋 안 된 수정이 있다 | 자기장치 | 거짓 | 미관 | xtask/src/main.rs | 「접힘 → 철회」 낱말 교체뿐이고 `cargo test` rc=0 이다. 진행 중인 작업이다 |
| 25 | `record.py` 의 검산 짝짓기에 `정반합` 출처 항이 이 회차에 안 잡힌다 | 자기장치 | 거짓 | 미관 | .claude/skills/round/bin/record.py | `record.py` 가 `정반합` 을 출처·기각 절·기계 칸에 다 등록한다. 원장 14행은 실제로 실려 있다 |

## 끝내도 되는가

**안 된다.** 본 목록에 금지역 둘(`H2` — 커밋 33개 미push · CI 런 0 · `H3` — 이슈 일곱 다 OPEN)과
실패 하나(`cargo xtask check` rc=1)가 남았고, 셋 다 게이트가 이미 「통과」로 적어 둔 조건이다.
절차상 곧 참이 될 수 있는 것이라도, **판정이 사실보다 먼저 쓰인 상태로 원장이 잠기면 아무도
다시 재지 않는다** — 그것이 이 저장소가 닫으려는 병이다.
