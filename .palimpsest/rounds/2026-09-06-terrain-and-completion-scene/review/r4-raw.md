# 독립 리뷰 R4 — 원 반환문

> `pal-independent-reviewer` 가 낸 것을 그대로 보존한다. 요약하지 않는다.
> 받은 것: 잠긴 의도와 산출물. 안 받은 것: 대화 기록 · 앞 라운드의 결과 · 설명.
> 실행 2026-09-07 · HEAD `07efe73`

---

모든 판정을 직접 돌려서 냈다. 금지역 목록의 출처는 **잠긴 의도 `## 금지역`**(`.palimpsest/rounds/2026-09-06-terrain-and-completion-scene/intent.md:105-111`) 이다 — `.claude/pal/policy.toml` 은 없다(`ls` 로 확인).

## 합격선 축

| 조건 | 판정 | 잰 수 | 근거 |
|---|---|---|---|
| A1 | 통과 | 장면 5 · 소제목 15 | `cargo xtask check` → `완성 장면 형식 — 장면 5개 · 소제목 셋 전부 갖춤` (27/27 · rc=0) |
| A1-a | 통과 | 장면 5 ≥ 하한 3 | 같은 검사. `xtask/src/main.rs:4544` 의 `if 장면 < 장면_하한 { bail! }` 을 소스로 확인 |
| A2 | 통과(정반합 판 2 R1) | 바깥쪽 2(§1·§2) · 안쪽 2(§3·§4) | `docs/plan/01-completion-scenes.md` 절머리 실측. 문면(「둘 다」)을 실제로 잰다 |
| A3 | 통과(정반합 판 2 R1) | `P8` 장면 1(§5) | 같은 문서 `:139-208`. 차선책 부속 요구 ⓑ 미이행을 게이트가 `A3` 행에 적는다 |
| A4 | 통과 | 문서 487 · 링크 648 · 죽은 것 0 | `cargo xtask check` 「죽은 링크 부재」 · `docs/plan/02-order.md:4` 가 `01-completion-scenes.md` 를 건다 |
| A5 | 통과 | 양쪽 2곳 | `docs/plan/00-goals.md:137` ★ 「이 절이 정본이다」 · `docs/plan/01-completion-scenes.md:6` 「00-goals §1 이 정본이다」 |
| B1 | 통과 | 능력 7행 · 측정 커밋 7/7 | `docs/plan/02-order.md:46-54` 전 행에 `19b5877`. ⚠ 근거 열의 값 전부를 그 커밋에서 재현하진 않았다(미측정 목록 참조) |
| B1-a | 통과 | 정의 1절 | `docs/plan/02-order.md:38-44` 「「서 있다」가 무엇인가」 |
| B2 | 통과 | 항목 6/6 이 능력 표기 | `:96-103` — `C2`·`C2`·`C6`·`C3`·`C4`·`C5` |
| B2-a | 통과 | 근거 6/6 | 같은 표 「왜 그 자리인가 — §1 의 어느 행」 열 |
| B3 | 미측정(교착) | — | 게이트대로. 소유자 답(축 문장의 읽기 강도) 없이는 안 선다. 이슈 #122 |
| B3-a | 미측정(교착) | 분모 4 확정 · 문턱 미정 | 게이트 `:67`. 분모 넷의 눈금(0·25·50·75)이 선택을 못 낳는다 |
| B4 | 통과 | 이슈 1 | `gh issue view 77` → `state=OPEN · createdAt 2026-08-19T22:58:57Z`, 착수 `7c41b81` = `2026-09-06T14:39:03+09:00` 이전 |
| B4-a | 통과 | 첫 항목 1 | `./scripts/frontier.sh` 실행 → `#77 … ← 순서표의 1 번` · 마지막 줄이 두 문서를 가리킨다 |
| B5 | 반증 | `grep -c '§1'` 절 전체 3 · 원 논증 두 문단 0 | 재현: `awk '/^### 음성 대조 — 역순으로/,/^### \`U\`/' docs/plan/02-order.md \| grep -c '§1'` → `3`; `:110-116` 두 문단에 `§1` 0 |
| B6 | 반증 | 도출 출처 3갈래 · 자리를 움직이는 행 1(`C2`) | 게이트 `:78`(정반합 판 3). 조건 문면(「실측에서 도출됐다」)을 실제로 잰다 |
| C1 | 통과 | ⏳ 0건 | `grep -n '⏳' AGENTS.md` → 0. `AGENTS.md:39,40` 이 두 새 문서를 링크 |
| D1 | 통과(정반합) | 승계 3 · 이미있다 5 · 안승계 3 · 안본절 2 처분 | `docs/plan/02-order.md:283-345`. ⚠ `fabricate` 원문은 저장소 밖이라 내가 대조 안 했다 |
| D2 | 통과(정반합) | §1.2 열셋 중 승계 5 · 지목 3 | `docs/plan/02-order.md:249-282`. ⚠ 그 산술이 처분표와 어긋난다 — 아래 「있는데 틀린 것」 3·4 |
| D3 | 통과 | 1건 | `git ls-tree 7c41b81 \| grep ^NEXT` → 3, 지금 뿌리 → 2. `NEXT-B-plan-for-harness.md` 없음(지운 커밋 `3b5762a`) |
| D3-a | 통과 | 모집단 3 · 지움 1 · 남김 2 · 붙드는 행 3/0 | 관측 재현: `grep -c 'NEXT-E-prompt.md' .palimpsest/rounds/2026-08-23-check-verifies-work/findings.jsonl` → `3`, 다른 파일 → `0` |
| E1 | 통과 | 검사 27/27 · rc=0 | `cargo xtask check` 전 출력 확인 |
| E2 | 통과 | 950 통과 · 0 실패 · 시험 바이너리 46 | `cargo test --workspace` 단독 실행 rc=0 (전 출력에서 `error[`·`panicked`·`FAILED` 0건) |
| E3 | 통과 | 209행 · 닫힘 208 · 열림 0 | `findings.jsonl` 파싱 · `cargo xtask check` 「발견이 닫혔나 — 열림 0행」 |
| E4 | 통과 | 금지 이름 0/4 · `## 원리상 못 잰 것` 3행 | `report.md` 절머리 5개(`남지 않은 것`·`다음 회차가 받는 것`·`범위 밖`·`원리상 못 잰 것`·`능력 부재`) |
| E5 | 통과 | 26+2+0+4=32 · 상자 32 | 게이트 `:57-64` · `grep -c '^- \[' intent.md` → `32` · 「원장 둘 대조」가 `(32개)` 로 센다 |
| E5-a | 통과 | 회차 이름 상수 0 · 종결 문서 4 | `이_회차_종결문서`(`xtask/src/main.rs:4662-4716`) 가 `docs/gates/README.md:140-152` 의 하한을 읽는다. 음성 대조 둘이 실제 RED 로 관측됨 |
| E6 | 미측정 | 미push 커밋 41 | `git status -sb` → `[ahead 41]` · `gh run list` 최신 런 head `7c41b81`(착수 커밋) |
| E6-a | 미측정 | 같음 | 같은 근거 |
| E7 | 통과 | `dialectic/` 21 파일 · 판 3 | 설계·정·반·합·종료 판단·취합 보고가 판마다 있다 |
| F1 | 통과 | 출력 1블록 | 게이트 `:139-155` 에 `frontier.sh` 출력. ⚠ 착수 블록이 바이트 그대로가 아니다 — 자기 산출 절 2 |
| F2 | 통과 | 착수 31 평평 → 지금 첫 항목 표시 1 | `./scripts/frontier.sh` 실행에서 `← 순서표의 1 번` 확인. 번호는 스크립트에 없고 `02-order.md:181` 에서 정규식으로 읽는다 |

음성 대조는 두 자리 다 **실제로 빨간 것이 관측됐다**(`observations/negative-control-scenes.txt` · `negative-control-selector.txt`, RED 출력 원문 포함).

## 미측정 목록

| # | 안 잰 조건 | 모집단 | 유효 | 해악도 | 좌표(파일:줄) | 왜 못 쟀나 |
|---|---|---|---|---|---|---|
| 1 | `B3` | 원의도 | 참 | 거짓신호 | `docs/gates/terrain-and-completion-scene.md:66` | 「소비자다」의 자가 없어 판정 입력이 안 선다. 소유자 답(#122) 없이는 나도 못 잰다 |
| 2 | `B3-a` | 원의도 | 참 | 거짓신호 | `docs/gates/terrain-and-completion-scene.md:67` | 문턱이 실측이 아니라 축 문장의 읽기 강도에서 나온다 |
| 3 | `E6` | 원의도 | 참 | 거짓신호 | `docs/gates/terrain-and-completion-scene.md:121` | 41 커밋이 미push 라 이 회차 커밋에 붙은 런이 없다 |
| 4 | `E6-a` | 원의도 | 참 | 거짓신호 | `docs/gates/terrain-and-completion-scene.md:122` | 같음 |
| 5 | `D1` 의 원문 대조 | 원의도 | 참 | 미관 | `docs/plan/02-order.md:283` | `~/dev/projects/fabricate/` 는 저장소 밖이라 내가 열지 않았다 — 인용 줄 번호를 원문과 안 맞춰 봤다 |
| 6 | `B1` 근거 열의 값 재현 | 원의도 | 참 | 미관 | `docs/plan/02-order.md:48` | `19b5877` 로 체크아웃해 `pal export --format cypher` 를 다시 돌리지 않았다. 측정 커밋이 행마다 박혀 있는 것만 확인했다 |

## 의도 축

### 빠진 것

| # | 발견 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 근거(명령·출력) |
|---|---|---|---|---|---|---|---|
| 1 | 순서표 §1 「못 잰 것」의 **규모 선형성**은 *"시간이 모자랐다"* 로 적힌 **잔여**인데, 종료 보고의 어느 절에도 없고 이슈로도 안 섰다. `/round` §10 의 가르는 문장(*"이 회차에서 그것을 할 수 있었는가"*)에 걸린다 — 같은 저장소를 복제해 재는 절차는 이 회차에서 돌릴 수 있었다 | 원의도 | 참 | 미관 | `E4` | `docs/plan/02-order.md:79` | `grep -n '규모 선형성' docs/plan/02-order.md` → `79:| 규모 선형성 | 시간이 모자랐다. …`; `grep -n '규모' report.md` → 0건; `./scripts/frontier.sh` 열린 이슈 39건에 해당 항목 없음 |

### 요구되지 않은 것

| # | 발견 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 근거(명령·출력) |
|---|---|---|---|---|---|---|---|
| 2 | 원문이 산출을 *"`docs/plan/` 새 문서 + `AGENTS.md` 두 칸은 가리키기만"* 으로 잠갔는데 `docs/plan/README.md` 와 `docs/plan/disposal-map.md` 도 고쳐졌다. 회차가 이 사실을 `state.md:59` 에 「미관·수용」으로 이미 적었으므로 숨긴 것은 아니다 — 다만 그 편집이 아래 3·4 의 결함을 새로 낳았다 | 저장소 | 참 | 미관 | 없음(의도 축) | `docs/plan/README.md:1`, `docs/plan/disposal-map.md:6` | `git diff --stat 7c41b81..HEAD` → `docs/plan/README.md \| 16 +-` · `docs/plan/disposal-map.md \| 20 +-` |

### 있는데 틀린 것

| # | 발견 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 근거(명령·출력) |
|---|---|---|---|---|---|---|---|
| 3 | **`AGENTS.md` 의 새 칸이 *"그중 둘은 목표 상태다"* 로 남아 있다 — 회차 자신이 넷이라고 재고 적었는데 진입점만 안 고쳤다.** 독립 리뷰 R3 의 레코드 `IR3-06`(유효성=참 · 처분=정정 · 조건 `C1`·`A2`)이 *"그래서 AGENTS.md 의 새 칸이 「그중 둘은 목표 상태다」로 적는다 — 실제로 안 도는 장면은 넷이다"* 라고 이름까지 대고 닫혔는데, `AGENTS.md` 는 착수 뒤 `3b5762a` 이후 한 번도 안 만졌다 | 원의도 | 참 | **금지역**(사실이 아닌 것을 사실로 · 잠긴 의도 `intent.md:109`) | `C1` | `AGENTS.md:40` | `grep -n '목표 상태' AGENTS.md` → `40: … 그중 둘은 목표 상태다 …`; `git log --oneline 7c41b81..HEAD -- AGENTS.md` → `3b5762a` 하나; `grep -c '목표 상태\|아직 안 돈다\|원리상 안 돈다' docs/plan/01-completion-scenes.md` 절별 확인 → §2·§3·§4·§5 넷에 표시 |
| 4 | **완성 장면이 *"도는 것은 §1 하나다"* 로 적는데 §1 도 안 돈다.** §1 의 화면 전문은 정본인 `00-goals.md` §1 이고 그 문서가 바로 다음 줄에서 *"이것은 전부 다 만들었을 때의 화면이다"* 라고 적는다. 실측: `pal touch` 는 돌지만 그 화면의 일곱 상자 중 넷(`내가 모르는 것`·`효과`·`판정`·`호출자`)이 「능력 없음/미구축」을 찍는다. R3 이 §2 를 고치면서 새로 쓴 산술이 §1 을 잘못 셌다 | 원의도 | 참 | 거짓신호 | `A5` | `docs/plan/01-completion-scenes.md:97` | `cargo run -q --bin pal -- touch check_completion_scenes` → `■ 내가 모르는 것 (이 빌드에는 unresolved-refs 능력이 없습니다 — F08 미구축)` · `■ 효과 (… effects 능력이 없습니다 — F13)` · `■ 판정 (… judgment 능력이 없습니다 — F15)` · `호출자 0 · 피호출자 0`; `docs/plan/00-goals.md:145` |
| 5 | 처분표 `B` 행이 *"열셋 가운데 다섯이 §5.1 로 승계됐고 **셋이 지목됐다**"* 로 적는데, 같은 문서 `:66` 은 R3 이 고치면서 *"**`F23` 하나**를 §2 의 빈자리로 지목했다 … `F02`·`F09` 는 §1.1 소속이라 이 열셋에 안 든다"* 로 적는다. R3 이 한 자리에서 고친 산술이 다른 자리에 그대로 남았다 | 저장소 | 참 | 거짓신호 | `D2` | `docs/plan/disposal-map.md:128` | `grep -n '아직 안 만든 기능의 결정' docs/plan/disposal-map.md` → `128: … 열셋 가운데 다섯이 §5.1 로 승계됐고 셋이 지목됐다`; `sed -n '66p'` → `… F23 하나를 §2 의 빈자리로 지목했다 — 여섯이 다뤄졌고 일곱이 남았다` |
| 6 | 처분표 머리가 *"**§2 의** `B` 처분 35 건"* 이라 적는데 그 행은 **§3**(`docs/DESIGN.md` 의 결정 99 개)에 있고 §2(최상위 문서)에는 `B` 가 한 글자도 없다. 그리고 같은 문장이 **DESIGN 결정 35 건**과 **§1.2 의 기능 열셋**을 한 모집단처럼 잇는다 — 둘은 다른 것을 헤아린다 | 저장소 | 참 | 거짓신호 | `D2` | `docs/plan/disposal-map.md:13` | `grep -n '^#' docs/plan/disposal-map.md` → `97:## 2. 최상위 문서` · `118:## 3. docs/DESIGN.md 의 결정 99 개` · `B`=35 행은 `:128`; `sed -n '97,117p' … \| grep -n 'B'` → 출력 없음 |

## 이번 라운드의 새 발견

위 「있는데 틀린 것」 3~6 과 「빠진 것」 1 이 이번 라운드의 새 발견 전부다. 그 밖에 본 목록에 넣을 것은 없다 — `.claude/` 는 이 회차가 한 글자도 안 고쳤고(`git diff --stat 7c41b81..HEAD -- .claude/` → 빈 출력), 따라서 `규약` 모집단의 발견은 없다.

## 자기 산출에 대한 발견

| # | 발견 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 근거(명령·출력) |
|---|---|---|---|---|---|---|---|
| 7 | 게이트 `## 효과` 가 `$ pal touch check_completion_scenes` 블록을 리터럴 출력처럼 싣는데 **바이트 그대로가 아니다** — 실제 출력은 `symbol 반경` 이고 게이트는 `반경 symbol` 로 어순을 뒤집었다(공백 수도 다르다). R3 이 `effect-frontier.txt` 에서 잡은 것과 같은 갈래가 게이트에 새로 났다 | 회차기록 | 참 | 거짓신호 | `F1` | `docs/gates/terrain-and-completion-scene.md:169` | 실제: `cargo run -q --bin pal -- touch check_completion_scenes \| sed -n '4p'` → `  [e64fd4f5378c5f20] 최신 상태(fresh)  ·  symbol 반경 · 감시 1`; 게이트: `grep -n '최신 상태(fresh)'` → `169:  [e64fd4f5378c5f20]  최신 상태(fresh)  · 반경 symbol · 감시 1` |
| 8 | 게이트 `## 효과` 의 「착수 시점」 블록은 관측 파일이 *"바이트 그대로가 아니다(⏳ 가 빠졌다)"* 라고 명시한 그 줄을 **그 경고 없이** 옮겨 실었다. 관측만 R3 이 고쳤고(`8554b85`) 그것을 인용하는 게이트는 안 고쳤다 | 회차기록 | 참 | 거짓신호 | `F2` | `docs/gates/terrain-and-completion-scene.md:147` | `git show 7c41b81:scripts/frontier.sh \| tail -1` → `… (지형은 ⏳ 비었다 — docs/plan/disposal-map.md)`; 게이트 `:147` 에는 `⏳` 없음. 경고는 `observations/effect-frontier.txt:5-8` 에만 있다 |
| 9 | 음성 대조 관측 둘이 **자기가 증명하는 동작이 존재하지 않는 커밋**을 좌표로 적는다. `negative-control-selector.txt` 는 `HEAD ced21fd` 라 적는데 fail-open 을 닫은 것은 그 다음 커밋 `e3de366` 이고, `negative-control-scenes.txt` 는 `HEAD 2cc703c` 라 적는데 「완성 장면 형식」 검사 자체가 그 다음 `844427e` 에서 생겼다. 적힌 좌표로 되짚으면 기록된 RED 가 안 재현된다 | 회차기록 | 참 | 거짓신호 | `E5-a`·`A1` | `.palimpsest/rounds/2026-09-06-terrain-and-completion-scene/observations/negative-control-selector.txt:2` | `git log --diff-filter=A -- …/negative-control-selector.txt` → `e3de366`; `git show e3de366 -- xtask/src/main.rs` 가 `fn 이_회차_종결문서(…) -> Vec<PathBuf>` → `-> Result<Vec<PathBuf>>` 로 바꾼다. 같은 명령이 `negative-control-scenes.txt` → `844427e` |
| 10 | 이 회차가 건 결박 둘 중 **`이_회차_종결문서` 가 지금 `stale`** 이다 — `ef93f6b` 에서 걸고 `e3de366` 이 그 함수 본문을 고쳤다. 게이트 `## 효과` 와 종료 보고는 fresh 인 쪽만 싣고 다른 쪽이 stale 인 것을 안 적는다. `/round` §11 조건 4 는 *"결박하고 그래프를 갱신했다"* 다 | 회차기록 | 참 | 거짓신호 | 없음(규약 §11-4) | `.palimpsest/rounds/2026-09-06-terrain-and-completion-scene/report.md:74` | `cargo run -q --bin pal -- touch 이_회차_종결문서` → `[49714573e0fa654c] 최신 상태 아님(stale) ← 1 개가 변했습니다  ·  symbol 반경 · 감시 1` |

## 내가 기각한 것

| # | 기각한 것 | 모집단 | 유효 | 해악도 | 좌표(파일:줄) | 왜 아니었나 |
|---|---|---|---|---|---|---|
| 11 | `cargo test --workspace` 가 `진단이_등록된_훅을_실제로_돌려본다` 에서 실패한다(20 통과 · 1 실패) — `E2` 반증 | 저장소 | 거짓 | 실패 | `crates/pal-cli/tests/install_hooks.rs:580` | **내가 만든 실패다.** 시험을 백그라운드로 돌리는 중에 `cargo run --bin pal` 을 쳐서 `target/debug/pal` 이 재빌드됐고, 그 시험이 등록본과 지금 실행 파일의 **바이트 크기**를 대므로 깨졌다(`53240744 vs 53271288`). 단독 재실행 → `950 통과 · 0 실패 · rc=0` |
| 12 | `effect-frontier.txt` 가 「열린 이슈 34건」인데 지금 39건이라 관측이 낡았다 | 회차기록 | 거짓 | 거짓신호 | `.palimpsest/rounds/2026-09-06-terrain-and-completion-scene/observations/effect-frontier.txt:22` | 관측은 `커밋 59801f7 뒤 · 2026-09-06` 으로 시점이 박혀 있고, 그 뒤 이 회차가 이슈 다섯을 열었다. 시점이 박힌 관측이 나중 값과 다른 것은 낡음이 아니다 |
| 13 | `E5-a` 가 「회차 이름을 손으로 안 적는다」인데 `docs/gates/README.md` 에 하한 `2026-09-06` 을 손으로 적는다 | 자기장치 | 거짓 | 거짓신호 | `docs/gates/README.md:140` | 조건 문면이 금한 것은 **회차 이름**이다. 하한은 날짜 하나이고 회차가 늘어도 안 고쳐도 되므로 R1 이 잡은 죽은 가지(사전순 하나)가 재발하지 않는다. `grep -rn 'terrain-and-completion-scene' xtask/src/main.rs` 의 두 건은 전부 주석이다 |
| 14 | 순서표 §3 이 스킬의 `pal` 호출을 **여섯**으로 세는데 `.claude/skills/round/bin/record.py:9` 에도 `pal round` 가 있어 계수가 틀렸다 | 원의도 | 거짓 | 미관 | `docs/plan/02-order.md:157` | 그 행이 세는 대상은 `SKILL.md` 이고, 일곱째를 넣어도 전부 `pal round …` 라 그 행의 판정(「없음」)이 안 바뀐다 |
| 15 | 게이트가 개정을 「46 까지」로 적는데 개정 표 행 수가 27 이라 어긋난다 | 회차기록 | 거짓 | 거짓신호 | `docs/gates/terrain-and-completion-scene.md:11` | 첫 행이 `1~20` 을 한 줄로 묶는다(`intent.md:264`). 20 + 26 = 46 으로 맞는다. 헤딩 중복도 여섯 문서 전부 0건이었다 |

## 끝내도 되는가

**안 된다** — 본 목록에 **금지역 하나**(#3 · `AGENTS.md:40` 이 「둘은 목표 상태」로 남았고, 그것을 지목한 R3 레코드 `IR3-06` 은 「정정·닫힘」으로 실려 있다)가 남았다. 나머지 넷(#4·#5·#6·#1)은 거짓신호·미관이라 상한과 함께 메인이 판단할 자리다.
