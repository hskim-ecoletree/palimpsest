# 독립 리뷰 라운드 1 — 원 반환문

> `pal-independent-reviewer` 가 돌려준 것을 그대로 보존한다. 합계 검산의 둘째 원천이다.
> 잠긴 의도와 산출물만 줬고 대화 기록·앞 라운드 논리는 안 줬다.

실측은 격리 클론(`git clone --no-local` → `92cd404`)과 `cargo xtask check --root <사본>` 으로 했고, 작업 트리는 안 건드렸다.

---

## 조건 판정

| 조건 | 판정 | 잰 수 | 근거 |
|---|---|---|---|
| A1 | **통과** | 사전 2/2 · 뺀 키 4 (조건파서 2 · 게이트파서 2) | `git show 92cd404 -- .claude/skills/round/bin/record.py` 가 두 사전 모두를 만졌고 `verification/mutate.py` 의 `B1-조건파서`·`B1-게이트파서` 가 둘 다 판정 대상에 든다 |
| A2 | **반증** (판정 맞다) | `설명` 밖 소비자 0 = 2 자리 | `mutate-result.txt` 의 `A2-설명밖-닫힘축뜻`·`A2-설명밖-역사형식` 둘 다 `검사_rc 0`. 그리고 `extract.py` 의 `자동프로필()` 은 `legacy-2020` 을 **원리상 못 낸다** → `반환형식.역사형식.legacy-2020` 은 이미 갈린 죽은 값 |
| B1 | **통과** (표본이 아니라 **전수**로 재확인했다) | 조건파서 **4/4** · 게이트파서 **10/10** · `설명` 서브트리 문자열 리프 **28/28** | 클론에서 각 사전의 손으로 쓴 문자열을 **전량** 치환 → `--schema` 도달 확인 · `cargo xtask check --root` = **23/23 통과** 각각. 28개 전량 동시 변이에서도 23/23 통과 + `extract.py` 산출 **바이트 동일** |
| B2 | **반증** (판정 맞다) | 정본 키 11 중 초록 2 | `mutate-result.txt` `B2-열별칭`: `검사_rc 0` · `추출_바뀜 true` |
| B3 | **통과** (판정 맞다) | 제품 diff 2 파일 · 새 손 열거 **0** | 새 검산 방어는 `[k2 for k2 in 판정값 if k2 not in _검산.groupindex]` 로 **파생**한다 — 넷을 다시 안 적었다 |
| C1 | **통과** | 재실행 산출물 5개 | RED 셋(`기준 0` / `변이 0` / `대조 1`)이 명령·입력·종료값·출력을 다 진다 |
| C2 | **통과** (환경 의존이다) | `cargo xtask check` **23/23** · `cargo xtask test` 전량 · `round_scripts_run` **15 passed** | ⚠ **23/23 은 `.gitignore` 된 `__pycache__/` 가 로컬에 있어서다** — 신선한 체크아웃에서는 `FAIL 회차 레코드` |
| D1 | **미측정 → 지금 반증** | 본 목록 금지역 **2** | 아래 「있는데 틀린 것」 6·7 |
| D2 | **미측정** | 게이트 문서 **0** · `report.md` **0** | `cargo xtask check` 판정문이 *"게이트 없음 2026-09-03-gate-parser-schema"* |

### 음성 대조 · RED

- 등록된 음성 대조 중 **①「비정본 변이는 결과를 안 바꾼다」와 ②「정본 키 변이는 실패시킨다」는 걸렸고 실제로 빨간 것이 관측됐다**.
- **③「두 변이 모두 새 출력에 도달했음을 먼저 확인한다」는 안 걸렸다** — `mutate.py` 의 `잰다()` 는 `--schema` 를 한 번도 안 부른다.

---

## 미측정 목록

| # | 안 잰 조건 | 모집단 | 유효성 | 해악도 | 조건 | 좌표(파일:줄) | 왜 못 쟀나 |
|---|---|---|---|---|---|---|---|
| M1 | D2 (CI 성공 · 게이트 커밋 · #94 종료) | 회차기록 | 참 | 실패 | D2 | `.palimpsest/rounds/2026-09-03-gate-parser-schema/intent.md` | 최종 커밋도 게이트도 아직 없다 |
| M2 | Windows/Linux 판 `cargo xtask check` | 저장소 | 추정 | 실패 | C2 | `.github/workflows/ci.yml` | 이 PC 는 darwin 하나뿐 |

## 빠진 것

| # | 발견 | 모집단 | 유효성 | 해악도 | 조건 | 좌표(파일:줄) | 근거 |
|---|---|---|---|---|---|---|---|
| 1 | 게이트 문서가 없다 — D2 가 *"회차 기록과 게이트를 커밋하고"* 를 요구하는데 `docs/gates/` 에 이 회차 짝이 0 이다 | 회차기록 | 참 | 실패 | D2 | `docs/gates/README.md` | `cargo xtask check` 판정문: *"게이트 없음 … 2026-09-03-gate-parser-schema"* |
| 2 | 종료 보고가 없다 — 네 이름을 안 쓴 것을 대조할 대상 자체가 없다 | 회차기록 | 참 | 거짓신호 | D2 | `.palimpsest/rounds/2026-09-03-gate-parser-schema/state.md` | `find` → 15 파일, `report.md` 없음 |
| 3 | 제품 수정 ⓒ(검산 갈래 크래시→진단)에 등록된 대조가 없다 — `mutate.py` 에 `판정값` 변이가 없고 `게이트판정()` 단위 시험이 저장소 전체에 0 이다 | 자기장치 | 참 | 거짓신호 | C1 | `.palimpsest/rounds/2026-09-03-gate-parser-schema/verification/mutate.py` | 클론에서 재현: 수정 전 `IndexError`, 수정 후 진단. 고침은 맞는데 산출물이 그것을 안 남겼다 |

## 요구되지 않은 것

| # | 발견 | 모집단 | 유효성 | 해악도 | 조건 | 좌표(파일:줄) | 근거 |
|---|---|---|---|---|---|---|---|
| 4 | `## 개정` 의 「한 것은 셋이다 · ⓐⓑⓒ」가 제품 변경 넷과 안 맞는다 — `docs/gates/README.md` 새 절이 어디에도 없다 | 회차기록 | 참 | 거짓신호 | D2 | `.palimpsest/rounds/2026-09-03-gate-parser-schema/intent.md` | 개정 본문에 `README` 문자열 0건 |
| 5 | `## 범위 밖` 에 네 항목이 잠금 뒤 직접 추가됐는데 개정이 그 추가를 안 진다. 그리고 `PM2-16` 은 그 넷 어디에도 안 걸리는데 `범위밖` 으로 닫혔다 | 회차기록 | 참 | 거짓신호 | D2 | `.palimpsest/rounds/2026-09-03-gate-parser-schema/findings.jsonl` | `git diff 7af5ddc..HEAD` 의 `## 범위 밖` hunk |

## 있는데 틀린 것

| # | 발견 | 모집단 | 유효성 | 해악도 | 조건 | 좌표(파일:줄) | 근거 |
|---|---|---|---|---|---|---|---|
| 6 | `docs/gates/README.md` 가 인용한 조건 라벨 `C5` 가 `rust-extractor.md` 에 없다 — 낡은 인용을 정정하려던 절이 좌표를 지어냈다 | 저장소 | 참 | 금지역 | B3 | `docs/gates/README.md` | `grep -c "C5" docs/gates/rust-extractor.md` → **0** |
| 7 | 회차가 「거짓이었다」고 선언한 문장이 같은 파일 주석에 그대로 있다 — 고친 것은 면제된 값 쪽이고 안 고친 주석이 면제 안 된 쪽이다 | 원의도 | 참 | 금지역 | A2 | `.claude/skills/round/bin/record.py` | `grep -n "어느 코드도 안 읽는다"` → 주석이 `7af5ddc` 판과 한 글자도 안 바뀌었다 |
| 8 | `xtask` 의 *"여기를 고치면 검사 동작이 바뀐다 — 그것이 이 선언이 정본이라는 증인이다"* 가 키 삭제에 대해 거짓이다 | 저장소 | 참 | 거짓신호 | A2 | `xtask/src/main.rs` | `mutate-result.txt` `연산자-키삭제` → `검사_rc 0` |
| 9 | `설명.정본아님` 이 자기 자신에 대한 사실 주장을 「거짓일 수 없는 구역」 안에 넣었다 — 원리상 반증 불가다 | 원의도 | 참 | 거짓신호 | A2 | `.claude/skills/round/bin/record.py` | 클론에서 `설명` 문자열 28개 전량 변이 → 23/23 통과 |

## 이번 라운드의 새 발견

| # | 발견 | 모집단 | 유효성 | 해악도 | 조건 | 좌표(파일:줄) | 근거 |
|---|---|---|---|---|---|---|---|
| 10 | 레코드 `출처` enum 에 정반합 자리가 없다 — 판정을 둘 뒤집은 반론 15 건이 원장 밖이라 A 축·계기판의 모집단이 아니다 | 규약 | 참 | 거짓신호 | D2 | `.claude/skills/round/SKILL.md` | `--schema` 의 `enum.출처` → 넷뿐 · `findings.jsonl` 43행이 전부 `사전부검` |

## 자기 산출에 대한 발견

| # | 발견 | 모집단 | 유효성 | 해악도 | 조건 | 좌표(파일:줄) | 근거 |
|---|---|---|---|---|---|---|---|
| 11 | `findings.jsonl` 의 좌표 하나가 `.gitignore` 된 빌드 산출물이라 신선한 체크아웃에서 `cargo xtask check` 가 빨개진다 — D2 의 CI 성공을 지금 상태로 막는다 | 회차기록 | 참 | 실패 | C2 | `.palimpsest/rounds/2026-09-03-gate-parser-schema/findings.jsonl` | 신선 클론에서 `FAIL 회차 레코드` · `git check-ignore -v` → `.gitignore` |
| 12 | 판정을 둘 뒤집은 근거 파일이 저장소에 없다 — `antithesis.md` 가 대는 측정 산출물이 전부 부재라 재실행도 열람도 불가다 | 회차기록 | 참 | 거짓신호 | C1 | `.palimpsest/rounds/2026-09-03-gate-parser-schema/decision/antithesis.md` | `find . -name "b1b2*"` → 0건. 내가 독립으로 재서 결론은 참임을 확인했다 |
| 13 | `mutate.py` 의 복원 대조가 항등식이다 — 쓰고 곧바로 읽어 비교하므로 언제나 참이다 | 자기장치 | 참 | 미관 | C1 | `.palimpsest/rounds/2026-09-03-gate-parser-schema/verification/mutate.py` | 코드 판독 |
| 14 | `mutate.py` 는 기대와 실측을 대조하지 않고 항상 종료값 0 이며, 재실행이 산출물을 덮어써 드리프트가 원리상 안 보인다 | 자기장치 | 참 | 거짓신호 | C1 | `.palimpsest/rounds/2026-09-03-gate-parser-schema/verification/mutate.py` | 코드 판독 — `기대` 는 출력 필드일 뿐 대조가 없다 |
| 15 | 등록된 음성 대조의 셋째 절(「도달했음을 먼저 확인한다」)이 장치에 없다 — `잰다()` 가 `--schema` 를 안 부른다 | 자기장치 | 참 | 거짓신호 | B1 | `.palimpsest/rounds/2026-09-03-gate-parser-schema/verification/mutate.py` | `grep -n "schema" mutate.py` → 도움말 줄 하나뿐 |
| 16 | `findings.jsonl` PM1-10 의 `조건변경=강화` 가 개정과 정면으로 어긋난다 — 개정은 B3 정정을 **철회**했다 | 회차기록 | 참 | 거짓신호 | B3 | `.palimpsest/rounds/2026-09-03-gate-parser-schema/findings.jsonl` | 레코드 판독 vs `intent.md` 개정 |
| 17 | 잠긴 `intent.md` 의 「RED 관측」 절이 아직 *"아직 미측정이다"* 다 — `red/` 가 관측을 남겼는데 본문이 안 따라왔다 | 회차기록 | 참 | 거짓신호 | C1 | `.palimpsest/rounds/2026-09-03-gate-parser-schema/intent.md` | `git diff` 에 그 hunk 없음 |
| 18 | `state.md` 「지금 단계」가 낡았다 — 정반합·효과·제품 커밋이 이미 끝났는데 「사전부검 → RED 관측」 순서를 적는다 | 회차기록 | 참 | 미관 | D2 | `.palimpsest/rounds/2026-09-03-gate-parser-schema/state.md` | 파일 판독 vs `git log` |
| 19 | `effect/observation.md` 의 *"값을 바꾸면 빨개지는 바로 그 키들이다"* 는 회차 산출물이 안 잰 주장이다 | 회차기록 | 참 | 미관 | D1 | `.palimpsest/rounds/2026-09-03-gate-parser-schema/effect/observation.md` | `mutate.py` 에 그 일곱의 값 변이가 없다. 내가 재서 참임을 확인했다 |

## 내가 기각한 것

| # | 기각한 것 | 모집단 | 유효성 | 해악도 | 조건 | 좌표(파일:줄) | 왜 아니었나 |
|---|---|---|---|---|---|---|---|
| 20 | 지운 상수 `조건절` 이나 뺀 키 넷을 읽는 자가 남아 회귀가 난다 | 저장소 | 거짓 | 실패 | B3 | `.claude/skills/round/bin/record.py` | 저장소 전체 참조 0 · `py_compile` 3 파일 통과 |
| 21 | 새 방어의 조기 반환이 다른 갈래를 조용히 죽인다 | 저장소 | 거짓 | 실패 | B3 | `.claude/skills/round/bin/record.py` | 게이트 47 전량을 `7af5ddc` 판과 대조: 출력·종료값 다른 것 **0** |
| 22 | `열림값` 은 값을 바꿔도 초록이라 소비자 0 이고 효과 문서가 거짓이다 | 저장소 | 거짓 | 거짓신호 | D1 | `.claude/skills/round/bin/record.py` | 내 첫 관측이 **모집단 공백** 때문이었다 — 클론에 이 회차 레코드가 없어 열린 행이 0 이었다. 넣고 다시 재니 빨개졌다 |
| 23 | README 새 절이 동결 규율을 어겼다 | 저장소 | 거짓 | 미관 | B3 | `docs/gates/README.md` | `README.md` 는 판정 문서가 아니고 선례가 이미 있다. 두 게이트 본문은 한 글자도 안 바뀌었다 |
| 24 | README 의 동결 함수 서술이 거짓이다 — 다섯째 규칙이 있다 | 저장소 | 거짓 | 미관 | B3 | `xtask/src/main.rs` | `docs/gates/` 에 하위 디렉터리가 없어 그 갈래는 오늘 발화하지 않는다 |
| 25 | `mutate.py` 가 저장소 밖 절대경로를 발견 좌표에 심는다 | 자기장치 | 거짓 | 금지역 | C1 | `.palimpsest/rounds/2026-09-03-gate-parser-schema/verification/mutate.py` | 뿌리를 `git rev-parse` 로 잡고 산출물에 절대경로를 안 쓴다 |
| 26 | `_검산` 이 `판정값` 넷을 손으로 다시 열거한 것이 B3 위반이다 | 저장소 | 거짓 | 거짓신호 | B3 | `.claude/skills/round/bin/record.py` | 그 열거는 이번 회차가 만든 것이 아니다. B3 는 diff 로 판정한다 |

## 끝내도 되는가

**안 된다.** 본 목록에 **금지역 2** 가 열려 있다(6·7). 둘 다 이 회차가 닫으려던 병 그 자체이고 각각 한 줄 편집으로 닫힌다. 그리고 자기 산출 쪽 11(`__pycache__` 좌표)은 **D2 의 CI 성공을 지금 상태로 막는다**.
