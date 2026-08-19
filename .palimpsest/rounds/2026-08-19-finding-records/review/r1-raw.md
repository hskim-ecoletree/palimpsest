I'll start by reading the locked intent and establishing the round's actual output surface.
독립 리뷰를 마쳤다. 잠긴 의도(`/Users/incognito/dev/projects/palimpsest/.palimpsest/rounds/2026-08-19-finding-records/intent.md`, 체크박스 **46** = 라벨 46, 헤딩 중복 0, `## 개정`·`## 승격` 은 빈 채)를 기준으로 잰다.

## 합격선 축

| 조건 | 판정 | 잰 수 | 근거 |
|---|---|---|---|
| C1-a | 통과 | 머리 줄 1 | `head -1 findings.jsonl` → `{"schema_version": 1, "회차": "..."}` |
| C1-b | 통과 | 필드 16 · enum 축 9 | `record.py --schema` ↔ `plan-v4.md:22-38` 표 전 항목 일치 |
| C1-c | 통과 | 대응표 10 항 | `record.py:54-65`; §5 어휘 9(안개·범위밖·정정·확대·축소·전환·완화·승격·막힘) 전부 있고 `막힘`·`승격` 은 `ENUM["처분"]`(6값) 밖 |
| C1-d | 통과 | `조건변경` 3값 | `record.py:38` `["강화","완화","없음"]`; `검증()` 이 `완화`↛`축소` 를 거부(`:96-97`) |
| C1-e | 통과 | `획득` 0회 | `FIELDS` 16개에 `획득` 없음 |
| C1-f | 통과 | 3 | `경로`·`줄`·`기준커밋` 모두 `FIELDS` 에 있고 66행 중 줄 24 · 기준커밋 66 채움 |
| C2-a | 통과 | 66행 검증 | `record.py check …` → `합계 66행 · 문제 없음` rc=0; `add` 는 설치본 시험이 실행 |
| C2-b | 통과 | enum 축 9 | `xtask/src/main.rs:3151-3175` 가 `Command::new(파이썬).arg(원천).arg("--schema")` 로 **호출**; 정규식 없음. 출력에 ``파이썬 `python3` `` |
| C2-c | 통과 | 살아 있는 자리 10 | `SKILL.md:96,98,235,237` · `dashboard.py:21,95` · `record.py:5-8` 전부 `python3 <경로>`. ⚠ 의도가 센 다섯 중 `NEXT-D-handoff.md` 는 이 회차가 지웠다(아래 발견 1) |
| **C2-d** | **반증** | Windows 실행 0회 | `git status -sb` → `ahead 9`, **push 안 됨**. `gh run list` 최신 런은 착수 커밋 `47a6770`. 이 회차 코드가 Windows 에서 돈 적 0. 조건은 "결과가 아니라 **잰 것**" 인데 잰 것이 없다 |
| C3-a | 통과 | 산출 3 | `cargo xtask check` → `산출 3개`; `회차_산출()` 이 `.palimpsest/rounds/**` 의 `jsonl`+`tsv` |
| C3-b | 통과 | 1 | 산출 3 = `findings.jsonl` · `premortem/disposal-overrides.jsonl` · `2026-08-18-completion-condition/retro/02-classification.tsv` |
| C3-c | 통과 | rc=1 | **격리 사본에서 재빌드해 직접 재현**: `mv rounds rounds.off` → `FAIL 회차 레코드 … 하나도 없다` |
| C3-d | 통과(부분) | 3 회차분 | `검산 R1 23↔23 · R2 17↔17 · R3 19↔19`. ⚠ 66행 중 **59행만** 검산에 걸린다(발견 5) |
| C3-e | 통과 | 좌표 66 | `:3245` 가 `경로` 만 해소, `줄` 미측정 — 선언(`:3239-3242`)과 구현 일치 |
| C3-f | 통과 | 7 기록 + 내가 3 재현 | `effect/negative-control.txt` 의 일곱. 나는 격리 사본(`scratchpad/neg/repo`, `cargo build -p xtask` 재빌드)에서 ③빈 모집단·④없는 경로·⑤합계 검산 어긋남을 rc=1 로 재현 |
| C4-a | 통과 | 2칸 | 계기판 출력에 `⑦ 원 의도 비율 21% (14/66)` · `⑧ 발견 유효성 참 49 · 거짓 17 → 25%` |
| C4-b | 통과 | 2상태 | `계기판이_레코드가_없으면_못_셌다고_말한다` ok — 레코드 전 「못 셌다」, 후 `1/1` |
| C4-c | 통과 | 1줄 | `⚠ ①~⑥ 은 <착수>..<종료> 커밋 범위를 재고, ⑦⑧ 은 레코드 파일 전체를 잰다` |
| C4-d | 통과 | 1줄 | `★ ⑦⑧ 의 원천은 **기록된 판정**이지 git 이 아니다.` |
| C4-e | 통과 | 1회 발화 | `⚠ 라운드가 어긋난다 — 커밋 태그 [1..7] ↔ 레코드 [1,2,3,4,5,7]` — 실제로 떴다 |
| C5-a | 통과 | 3/3 | `layout.rs:92-94`(PAYLOAD) · `:156`(OWNED_FILES) · `:169`(DIRS `.claude/skills/pal-round/bin`) |
| C5-b | 통과 | 2 시험 | `설치본의_record_가_스키마를_낸다` · `설치본의_계기판이_돈다` ok |
| C5-c | 통과 | 1 시험 | `round_scripts_run.rs:94-145` 가 「놓기 전/후」 둘 다 잰다 |
| C6-a | 통과 | 2필드 | `pal-premortem-sweeper.md` 반환에 `모집단`·`유효성` 추가됨 |
| **C6-b** | **반증** | 4/5 | 표가 붙은 절은 **넷**(빠진 것·요구되지 않은 것·있는데 틀린 것·내가 기각한 것). **`## 미측정 목록`(`:138`)은 여전히 산문**인데 바로 아래 `:167` 이 그것을 「형식 없이 두었다」고 자기 지목한다 |
| C6-c | 통과 | 1 | `근거: 조회|추정` → `획득: 조회|추정` |
| C7-a | 통과 | 59행 | 사전부검 R1 23 · R2 17 · R3 19; 검산 세 줄 전부 일치 |
| **C7-b** | **반증** | 0행 | `출처` 분포 = `사전부검 59 · 실측 7`. **독립리뷰 0**. `합격선판정` 66행 전부 `해당없음` |
| **C7-c** | **반증** | 0행 | **인터뷰 0행**. 의도가 인터뷰 3 라운드를 썼다고 적었는데(`:11-28`) 레코드에 없다 |
| **C7-d** | **반증** | 3/4 | `premortem/r1·r2·r3-raw.md` 뿐. 독립리뷰·인터뷰·실측의 원 반환문 보존 자리는 규약에도 없다 |
| C7-e | 통과 | 48행 | `모집단=자기장치` 48행이 실제로 적혔다 |
| C8-a | 통과 | 판별식 7갈래 | `SKILL.md:459-467` 에 `^\.claude/`·`^schema/`·`^surface/`·`^corpus/tasks/` 추가 + *"이 목록도 손으로 벤 거울이다"* 명시. `grep include_str!` 로 확인: 실제 컴파일 입력이 전부 덮인다 |
| C8-b | 통과 | 2/2 | `:105` 여덟 칸 · `:455` 검사 20 개. ⚠ 일곱 줄 아래 `:112` 가 아직 「여섯이 전부다」(발견 2) |
| C8-c | 통과 | 4줄 | `SKILL.md:96,98` |
| **C9-a** | **대조 불가** | — | 소유자만 관측자다. 이 자리에 관측 장치가 없고 회차 산출에 기록도 없다 |
| **C9-b** | **대조 불가** | — | 위와 같음 |
| **C10-a** | **반증** | 0/46 | 계기판 `② 미판정 잔액 46 / 46`; intent.md 체크박스 46개 전부 `[ ]`; 판정 문서 없음 |
| **C10-b** | **반증** | 0라운드 | 독립리뷰 레코드 0 — 상한 5 의 셈이 시작도 안 됐다 |
| **C10-c** | **반증** | — | `effect/effect.md` 는 **untracked**(`git status` `??`)이고 제목이 `# 효과`(h1)라 §9 의 `## 효과` 절이 아니다. 게이트 문서도 없다 |
| **C10-d** | **미측정** | — | 산출에 결박·그래프 흔적이 없으나(`git diff --stat` 에 schema/graph 없음, report 없음) 실행 여부를 잴 장치가 내게 없다 |
| **C10-e** | **반증** | 0 | `find … -name report.md` → 이 회차만 없다(앞 두 회차엔 있다). `state.md` 도 없다 |
| **C10-f** | **반증** | 0 런 | `ahead 9`, 이 회차 커밋에 붙은 CI 런 0건 |
| **C11-a** | **반증** | 0 | 게이트 문서 자체가 없다. 정직한 문장은 `plan-v4.md:148-153` 에만 있고, 그 문장이 *"게이트 `## 판정` 에 … 적는다"* 라고 자기 미이행을 선언한다 |
| **C12-a** | **반증** | 0 | `gh issue list --state all` — 2026-08-19 이후 생성 이슈 `[]`. 최신은 #71·#72(2026-08-18T21:12) |
| **C12-b** | **대조 불가** | — | C12-a 가 반증이라 「비어 있지 않은 목록」이 존재하지 않는다 |

**합: 통과 30 · 반증 12 · 대조 불가 3 · 미측정 1 = 46** ✓

## 미측정 목록

- **C10-d** — 결박·그래프 갱신. 산출에 흔적 0이지만 `.palimpsest/index.redb`(파생·gitignore)에 들어갔을 가능성을 이 자리에서 배제 못 한다.

## 의도 축

### 빠진 것

| # | 발견 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 근거(명령·출력) |
|---|---|---|---|---|---|---|---|
| M1 | 원문 라운드 3 의 **「ditto 를 다 만든 뒤에 대 보아 저쪽이 품고 이쪽이 빠뜨린 필드를 발견으로 낸다」** 가 산출에 없다. 실제로 대 보면 갈린다 — ditto `objections[]` 는 `evidence[]`(kind/path/command/sha256/lines/summary) · `required_fix` · `failure_mode` · `run`(provider/model/timestamp)을 품는데 palimpsest 레코드엔 **하나도 없다.** 특히 리뷰어 표의 `근거(명령·출력)` 열이 **레코드에 착지할 칸이 없어** 전사 때 통째로 버려진다 | 원의도 | 참 | 거짓신호 | 없음(의도 축) | `intent.md:25-26` ↔ `record.py:45-51` | `python3` 로 `dialectic.schema.json` 파싱 → `opponent.objections.items.properties` = severity·id·claim·**evidence**·maps_to·**failure_mode**·**required_fix**. `record.py` `FIELDS` 16개에 근거/수정요구 칸 0. `grep -rn "dialectic\|ditto" .palimpsest/rounds/2026-08-19-finding-records/` → 대조 결과 문서 0건 |
| M2 | **독립리뷰·인터뷰 발견을 담을 기계 경로가 없다.** `premortem/extract.py:11-16` 의 매핑은 사전부검 전용(`대상 계획대상/계획자신`)이고, 원 반환문 보존 규약(`SKILL.md:243`)도 `premortem/r<n>-raw.md` 꼴만 정한다 | 원의도 | 참 | 실패 | C7-b·C7-c·C7-d | `SKILL.md:243` · `premortem/extract.py:11-16` | `출처` 분포에 독립리뷰·인터뷰 0. `合격선판정` 66행 전부 `해당없음` |
| M3 | 종료 산출 일체 없음 — `report.md` · `state.md` · 게이트 문서 · 커밋된 `## 효과` | 원의도 | 참 | 실패 | C10-c·e·f, C11-a, C12-a | 없음 | `find .palimpsest/rounds -name 'state.md' -o -name 'report.md'` → 이 회차 0건; `git log 47a6770..HEAD -- docs/gates/` 빈 출력 |

### 요구되지 않은 것

| # | 발견 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 근거(명령·출력) |
|---|---|---|---|---|---|---|---|
| U1 | `NEXT-D-handoff.md`(118줄, 추적됨) **삭제**. 의도의 범위 밖·완수 조건 어디에도 삭제가 없고 오히려 C2-c 가 그것을 다섯 자리 중 하나로 **센다**. `## 개정` 은 비어 있어 이 축소가 기록되지 않았다 | 원의도 | 참 | 거짓신호 | C2-c | `intent.md:95`, `intent.md:155-157` | `git log --diff-filter=D --name-only 47a6770..HEAD` → `NEXT-D-handoff.md` (커밋 `433fcf9`) |

### 있는데 틀린 것

| # | 발견 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 근거(명령·출력) |
|---|---|---|---|---|---|---|---|
| W1 | 리뷰어 반환 표에 **`처분` 칸이 없다.** `처분` 은 레코드 `REQUIRED` 9개 중 하나라, 독립리뷰 발견을 적으려면 **기록자가 추측**해야 한다. 사전부검에는 `premortem/disposal-overrides.jsonl` 이라는 자리가 있지만 독립리뷰·인터뷰에는 없다 — 이 회차가 사전부검에 대해 *"추측이 들어가면 ⑦⑧ 의 분모가 기록자 판단에 좌우된다"* 며 고친 바로 그 병이 다른 출처에 그대로 남았다 | 규약 | 참 | 거짓신호 | C6-b·C7-b | `.claude/agents/pal-independent-reviewer.md:143-160` ↔ `record.py:45` | `record.py --schema` `필수` = id·라운드·출처·모집단·유효성·해악도·**처분**·경로·요약. 리뷰어 표 열 = #·발견·모집단·유효·해악도·조건·좌표·근거 |

## 이번 라운드의 새 발견

| # | 발견 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 근거(명령·출력) |
|---|---|---|---|---|---|---|---|
| 1 | **설치본에 실려 나가는 파일이 이 회차가 지운 문서를 현재형으로 부른다.** `record.py:13` 이 *"다섯 자리가 전부 이 형태로 적힌다 — … `NEXT-D-handoff.md`"* 라고 단언하는데 그 파일은 같은 회차 커밋 `433fcf9` 가 지웠다. 이것을 잡으라고 있는 검사 19(`사라진 문서를 현재형으로 안 부른다`)는 **손으로 적은 8개 토큰 목록**만 보고, 이 회차가 지운 파일을 거기 안 넣었다 — 삭제와 목록 갱신을 잇는 장치가 없다 | 저장소 | 참 | 거짓신호 | C2-c | `xtask/src/main.rs:2771-2781` · `.claude/skills/round/bin/record.py:13` | `ls NEXT-D-handoff.md` → No such file. `grep -rn NEXT-D-handoff` → `record.py:13` 만 남음. `cargo xtask check` → `사라진 문서 인용 490곳 · 전부 「옛」 표기` **ok**(못 잡았다) |
| 2 | **규약이 일곱 줄 사이에서 자기모순.** `SKILL.md:105` *"여덟 칸이 나온다"* ↔ `:112` *"카테고리는 **enum 이고 여섯이 전부다.**"* C8-b 가 고치라 한 두 문자열은 고쳐졌지만 같은 수의 **세 번째 사본**이 남았다. 이 파일은 `layout.rs` 가 `include_str!` 로 실어 사용자 프로젝트에 나가고 **다음 회차 전체를 지배한다** | 규약 | 참 | 거짓신호 | C8-b | `.claude/skills/round/SKILL.md:105,112` | `grep -n "여섯이 전부\|여덟 칸" SKILL.md` → `105:여덟 칸이 나온다` / `112:카테고리는 **enum 이고 여섯이 전부다.**` |
| 3 | **`## 미측정 목록` 만 표가 안 붙었다.** 그리고 `:166-169` 의 산문이 *"앞 판은 … `## 미측정 목록` … 을 **형식 없이** 두었고, 그 절의 발견들은 필수 필드를 못 채워 **레코드에 안 실렸다**"* 라고 그 절을 **직접 지목한 뒤 고치지 않았다.** 게다가 그 산문은 표가 붙은 절을 「여섯」이라 세는데, 자기가 나열한 것은 다섯이고 실제로 고쳐진 것은 넷이다 | 규약 | 참 | 거짓신호 | C6-b | `.claude/agents/pal-independent-reviewer.md:138,166-169` | `sed -n 136,170p` — `## 미측정 목록` 아래에 `<이번 라운드에 안 잰 조건. 없으면 "없음".>` 산문만 |
| 4 | **Windows 를 안 쟀고 CI 가 한 번도 안 돌았다.** 9 커밋이 push 안 됐다. C2-d 는 *"결과가 아니라 **잰 것**이 조건"* 인데 잰 것이 0이다. 그리고 안 잰 자리에 실제 실패 후보가 있다(자기 산출 1번) | 원의도 | 참 | 실패 | C2-d·C10-f | 없음 | `git status -sb` → `## main...origin/main [ahead 9]`; `gh run list --limit 8` 최신 = 착수 커밋 `47a6770` |
| 5 | **합계 검산이 사전부검 밖에서는 죽은 가지다 — 그리고 라운드 번호가 겹치면 거짓 실패한다.** 검산 루프는 `premortem/r<n>-raw.md` 만 훑고(`:3258-3268`) 레코드 쪽은 `라운드==n` 인 **모든 출처**를 센다. ①C7-b/C7-c 가 요구하는 독립리뷰·인터뷰 행은 대응 raw 파일이 없어 **아무 검산도 안 받는다**(현재도 실측 7행이 무검산). ②그 행의 `라운드` 가 사전부검 라운드와 겹치면 **멀쩡한 레코드가 FAIL 을 낸다** | 자기장치 | 참 | **금지역**(측정이 죽은 가지) | C3-d·C7-b·C7-c·C7-d | `xtask/src/main.rs:3255-3285` | 격리 사본 재빌드 후 실측 — **NC-A**: `{"라운드":8,"출처":"독립리뷰",…}` 추가 → `rc=0 · 레코드 67행 · 검산 R1/R2/R3 그대로`(새 행 무검산). **NC-B**: 같은 행을 `"라운드":1` 로 → `rc=1 · 합계 검산 어긋남 — r1-raw.md: 원 반환문의 항이 23 인데 레코드는 24 행이다` |

## 자기 산출에 대한 발견

| # | 발견 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 근거(명령·출력) |
|---|---|---|---|---|---|---|---|
| S1 | **Windows 파이프에서 `record.py --schema` 와 `dashboard.py` 가 `UnicodeEncodeError` 로 죽을 개연이 크다.** 스키마 키·계기판 출력이 전부 한글이고 `ensure_ascii=False`. Windows Python 은 비-tty stdout 에 `locale.getpreferredencoding()`(보통 cp1252)를 쓴다. 그러면 CI 의 20번째 검사와 새 시험 셋이 Windows 에서만 빨개진다 | 자기장치 | **추정**(기제는 재현, 실 Windows 미관측) | 실패 | C2-d | `record.py:195` · `dashboard.py:95` | `PYTHONIOENCODING=cp1252 python3 record.py --schema` → `rc=1 … UnicodeEncodeError: 'charmap' codec can't encode characters in position 28-29`. 계기판도 동일 rc=1 |
| S2 | **좌표 해소가 접미 매칭이라 거의 아무것이나 통과한다.** `경로=".md"` · `"s.rs"` 가 실재로 판정된다 | 자기장치 | 참 | 거짓신호 | C3-e | `xtask/src/main.rs:3303-3330` | 격리 사본에서 심어 확인 — `.md` rc=0 · `s.rs` rc=0 · `main.rs` rc=0 / `완전히없는파일.xyz` rc=1 · `x` rc=1 |
| S3 | **「0 건은 안 봤다일 수 있다」가 회차 단위로는 안 선다.** 새 회차에 머리 줄만 있는 `findings.jsonl` 을 두면 통과한다 — 사전부검이 없는 회차는 발견 0 으로 조용히 지난다 | 자기장치 | 참 | 거짓신호 | C3-c | `xtask/src/main.rs:3183-3189` | NC-C: `2026-08-20-빈회차/findings.jsonl`(헤더만) → `rc=0 · 산출 4개 · 레코드 66행` |
| S4 | `(경로 없음)` 센티널이 `xtask` 에만 살고 `record.py --schema`·대응표·계획 어디에도 없다. 이 회차가 *"두 곳에 적으면 갈리고 갈린 것을 대는 장치가 없다"* 며 enum 을 한 자리에 모은 것과 같은 자 | 자기장치 | 참 | 미관 | C1-b | `xtask/src/main.rs:3245` | `record.py --schema | grep 경로 없음` → 0건; 레코드 4행이 그 값을 쓴다 |
| S5 | 계기판 ⑦⑧ 의 **라운드 대조(C4-e)가 설치본에서는 원리상 안 뜬다** — 빈 범위 분기가 `발견칸(의도파일, set())` 로 부르고, 대조는 `라운드셋` 이 비면 건너뛴다. 갓 설치한 프로젝트는 언제나 빈 범위다 | 자기장치 | 참 | 미관 | C4-e | `.claude/skills/round/bin/dashboard.py:93`, `:207-210` | 코드 읽음 + `설치본의_계기판이_돈다` 가 빈 범위 경로를 탄다 |
| S6 | `effect/effect.md` **② 의 「파일 9 개」가 틀렸다 — 7 이다.** ditto 쪽 총 83·참 50·거짓 33 은 정확 | 회차기록 | 참 | 거짓신호 | C10-c | `.palimpsest/rounds/2026-08-19-finding-records/effect/effect.md:19` | `grep -rc '"admissible": \(true\|false\)'` → dialectic-1,2,3,4,5,8,9 **7개** 파일, 14+10+6+20+9+13+11 = 83 |
| S7 | `effect/effect.md` 가 **커밋되지 않았다**(untracked) 그리고 `# 효과`(h1)라 §9 가 정한 `## 효과` 절이 아니다 | 회차기록 | 참 | 거짓신호 | C10-c | 위 파일 :1 | `git status --short` → `?? .palimpsest/rounds/2026-08-19-finding-records/effect/effect.md` |
| S8 | `intent.md` 의 `## 개정`·`## 승격` 이 **둘 다 플레이스홀더 그대로**인데 계기판 `⑥ 승격 횟수 1` 이고 커밋 `48e5264` 에 `[승격]` 표기가 있으며 소유자 답(*"안 만지고 #68 에 관측만"*)은 `plan-v4.md:148` 에만 있다. §3 이 정한 자리가 비었다 | 회차기록 | 참 | 거짓신호 | 없음 | `intent.md:155-161` · `plan-v4.md:148` | `sed -n 155,161p intent.md` → `(루프가 적는다.)` / `(축소·전환으로 올린 것과 소유자의 답을 적는다.)` |
| S9 | 레코드 한 행에 **기계 고유 절대 경로**가 박혔다(`/Users/incognito/dev/projects/palimpsest/…`). 다른 기계에서는 `root.join()` 도 접미 매칭도 안 맞아 검사가 빨개진다 | 회차기록 | 참 | 미관 | C3-e | `findings.jsonl` 의 해당 행 | 좌표 분포 출력 — `EXACT 1 '/Users/incognito/dev/projects/palimpsest/.palimpsest/rounds/2026-08-19-finding-records/premortem/r1-raw.md'` |
| S10 | 이 회차에만 **`state.md` 가 없다**(앞 두 회차엔 있다). §5 「교대」가 새 컨텍스트에 주기로 한 둘 중 하나가 없다 | 회차기록 | 참 | 미관 | 없음 | 없음 | `find .palimpsest/rounds -name 'state.md'` → 2026-08-18 계열 셋만 |

## 내가 기각한 것

| # | 기각한 것 | 모집단 | 유효 | 해악도 | 좌표(파일:줄) | 왜 아니었나 |
|---|---|---|---|---|---|---|
| X1 | `docs/sunset.toml:31` 의 *"이 파일도 **검사 19** 의 모집단이다"* 가 검사 20개가 되면서 낡았다 | 저장소 | 거짓 | — | `docs/sunset.toml:31` | 세어 봤다 — 회차 레코드가 **20번째**로 append 됐고 19번째는 여전히 「사라진 문서를 현재형으로 안 부른다」다. 인용이 맞다 |
| X2 | `record.py` 의 `읽기()` 는 `i == 1`, `xtask` 는 `i == 0` 으로 머리 줄을 판별한다 — 앞에 빈 줄이 있으면 두 구현이 **조용히** 갈린다 | 자기장치 | 거짓 | — | `record.py:109` · `xtask/src/main.rs:3213` | 심어서 확인 — 선행 빈 줄을 넣으니 `record.py check` 가 **rc=1 로 시끄럽게** 실패(`머리 줄에 schema_version 이 없다` + 필수 필드 9건). 조용한 갈림이 아니다 |
| X3 | 새 검사가 `cargo xtask check` 에 **파이썬 의존**을 들여, 파이썬 없는 기계에서 게이트 전체가 빨개진다 | 자기장치 | 거짓 | — | `xtask/src/main.rs:3122-3145` | 세 러너 전부 파이썬을 품고(`ubuntu/macos/windows-latest`) `python3`→`python` 폴백이 있으며 실패 시 사유를 정확히 낸다. 실측 하한도 아니고 가정 위의 가정이다 |
| X4 | 설치본 파일이 0644 라 스크립트가 안 돌 것이다 | 자기장치 | 거짓 | — | `round_scripts_run.rs:59-86` | 이미 `python3 <경로>` 로 축을 옮겼고 시험 셋이 실제로 설치본에서 돌아 ok |
| X5 | `sunset` 트리거가 `findings.jsonl` 로 발화해 CI 가 빨개진다 | 저장소 | 거짓 | — | `docs/sunset.toml:29` | `cargo xtask check` → `sunset 선언 — 트리거 .palimpsest/rounds/*/*.json 는 아직 0건`. `.jsonl` 은 안 걸린다(그 비껴감 자체는 `disposal-overrides.jsonl:1` 의 `_왜` 와 `plan-v4.md:146-153` 에 정직하게 적혀 있다 — 다만 게이트에는 아직 안 적혔다 = C11-a) |

기각 5 / 총 제기 25 → **20% 가 헛것**.

## 끝내도 되는가

**안 된다.** 본 목록에 금지역 1(발견 5 — 합계 검산이 사전부검 밖에서 죽은 가지이고 라운드 충돌 시 거짓 실패)과 실패 2(발견 4 · M2/M3)가 남았고, 합격선 축이 **반증 12 · 대조 불가 3 · 미측정 1** 이다. 특히 C7-b·C7-c·C7-d 는 이 회차가 스스로 *"유일한 실사용 증인"* 이라 이름 붙인 조건인데 **독립리뷰 0행 · 인터뷰 0행**이다 — 장치는 섰지만 그 장치가 담기로 한 네 출처 중 둘이 비어 있고, 담으려 하면 발견 5 가 발화한다.

주요 좌표: `/Users/incognito/dev/projects/palimpsest/xtask/src/main.rs:3255-3285`(합계 검산) · `:3303-3330`(좌표 해소) · `/Users/incognito/dev/projects/palimpsest/.claude/skills/round/bin/record.py:13`(사라진 문서 인용) · `/Users/incognito/dev/projects/palimpsest/.claude/skills/round/SKILL.md:105,112`(자기모순) · `/Users/incognito/dev/projects/palimpsest/.claude/agents/pal-independent-reviewer.md:138,166-169`(미측정 목록).
