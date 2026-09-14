# 독립 리뷰 라운드 3 — 원문

> 회차 `2026-09-13-first-release-elsewhere` · 잰 날 2026-09-14 · HEAD `264d967`(로컬 · origin/main `067c7c4`)
> **읽기 전용이다.** 리뷰어는 저장소를 고치지 않았고 빌드·픽스처는 스크래치 클론에서 돌렸다. 원본 `~/dev/projects/ditto` 는 열지 않았다.
> **옮겨 적은 자:** 메인. `pal-independent-reviewer` 반환문의 마지막 ```markdown 블록 본문을 바이트 그대로 옮겼다 — 이 머리 넷만 더했다. 리뷰어가 안 받은 것: 대화 기록 · 메인의 사고 과정 · 앞 라운드(R1 · R2) 결과.

## 합격선 축

| 조건 | 판정 | 잰 수 | 근거 |
|---|---|---|---|
| A1 | 통과 | CI 시험 1 ok · 로컬 픽스처 `./a` 엣지 1/1 | 런 `34803197814`(`067c7c4`, crates·xtask 가 HEAD `264d967` 과 같다) ubuntu 잡 `a1_지정자_여섯_꼴이_다른_파일의_심볼로_가는_엣지가_된다 ... ok` · 로그 전체 `panicked`/`FAILED` 0. HEAD 바이너리 `pal 0.0.0+264d967fbc47` 로 TS 픽스처에서 `cross-file-import 1/1`. RED `oracle/A1-red.txt` 는 안 열었다 |
| A1-a | 통과 | 까닭 3/3 갈림 | HEAD 바이너리 픽스처(`node:fs` · `zod` · `./missing` 을 본문에서 씀): `cross-file-import 1/4` · `bare_specifier 1 · no_target_file 1 · outside_repo 1` · 맨 지정자 줄이 「안 하기로 정한 자리입니다」 갈래에 있다(「원리상 풀 수 없는」 아님) |
| A2 | 통과 | CI 시험 6/6 ok | `a2_1` ~ `a2_6` 여섯 줄 ok |
| A2-a | 통과 | CI 시험 1 ok | `a2a_패키지_extends_에서만_오는_별칭은_저장소_밖이_아니라_못_읽음이다 ... ok` |
| A3 | 미측정 | 0 | TypeScript 컴파일러 대조를 이번 라운드에 안 돌렸다 — 게이트 산출 `oracle/A3-fixture.txt` 만 있다 |
| A3-a | 미측정 | 0 | 같음 |
| A4 | 통과 | CI 시험 1 ok | `a4_재수출_배럴로_펴지는_임포트는_재수출을_지나는_이름이다 ... ok` |
| A5 | 미측정 | 0 | ditto 새 복제본 + TypeScript 대조를 안 돌렸다 |
| A6 | 미측정 | ⑴ CI `cargo xtask test` success · ⑵ 0 | ⑴ 은 `067c7c4` 런이 대신한다. ⑵ Rust 엣지 집합 1319=1319 는 다시 안 쟀다. 게이트 근거 줄이 `5b4a37f` 뒤 radius 수정 셋을 빠뜨린다(자기 산출 절) |
| A7 | 통과 | 화면 줄 1 · 단언 1 | HEAD 바이너리 화면 「※ 호출자 수는 하한입니다 — 파일 최상위의 참조와 문자열로 찾는 자리는 세지 않습니다」 · 커밋 `9db40b7` 이 그 단언을 더했다 |
| B1 | 통과 | CI 시험 1 ok | `b1_touch_가_승인_대기_조각을_경로_앵커_후보_수_승인_명령과_함께_싣는다 ... ok` · 단언 `안내된_명령(…).len() == 2` · 「걸린 것 (0)」 |
| B1-a | 통과 | 같은 시험 안 단언 1 | `crates/pal-cli/tests/pending_and_pick.rs:88-89` `lonely` 음성 대조 · 위 시험 ok |
| B2 | 통과 | CI 시험 1 ok | `b2_화면이_안내한_명령을_그대로_돌리면_통한다 ... ok`. RED `oracle/B2-red.txt` 는 안 열었다 |
| B3 | 통과 | CI 시험 1 ok · 로컬 ⑴ 1 | `b3_목록이_없거나_낡으면_0_을_안_찍는다 ... ok` · HEAD 바이너리 픽스처에서 「아직 만들지 않았습니다 — `pal narrative` 를 먼저 돌리면 만들어집니다」 |
| B4 | 미측정 | 0 | ditto 복제본 타이밍을 안 돌렸다 |
| C1 | 통과 | CI 시험 1 ok | `c1_후보_화면의_지목_문자열을_그대로_주면_하나가_나온다 ... ok` |
| C1-a | 통과 | CI 시험 1 ok | `c1a_어느_후보에도_안_맞는_지목은_고르지_않는다 ... ok` |
| D1 | 통과 | CI 시험 1 ok | `d1_장면_명령의_사람_화면에_작업_기록_어휘가_없다 ... ok` (`d1_scan` 은 설계대로 ignored). RED `oracle/D1-red.txt` 는 안 열었다 |
| D1-a | 통과 | CI 시험 4/4 ok | `d1a_*` 넷 ok |
| D2 | 통과 | 문자열 2/2 · 한 커밋 1/1 | HEAD 바이너리 화면에 두 문자열 · `git show 9db40b7 -- crates/pal-cli/tests/cross_file_references.rs` 에 `- 화면.contains("범위 밖")` 과 `+ …「안 하기로 정한 자리입니다」` · `+ …「원리상 풀 수 없는 자리입니다」` 가 한 커밋 |
| E1 | 미측정 | 0 | `oracle/E1-order.sh` 를 다시 안 돌렸다 |
| E2 | 미측정 | 0 | `oracle/E2-scene.py` 를 다시 안 돌렸다 |
| E3 | 대조 불가 | 산출 대조 1 | `intent.md ## 승격` 4~6 의 소유자 답이 있다. 판 `e3-effect` 의 논증은 검토하지 않았다 |
| E4 | 미측정 | 0 | `tsc` 전후 대조를 다시 안 돌렸다 |
| F1 | 통과 | 코멘트 1 | `gh issue view 135` — 코멘트 1, 본문에 `7199/11099` 와 `docs/gates/first-release-elsewhere.md` |
| F2 | 반증 (문면대로) — 게이트는 통과 | 런 조회 2 | 조건 문면 「회차의 마지막 커밋 SHA」: HEAD `264d967fbc47…` → `gh run list --commit` = `[]`. 게이트가 댄 `067c7c4` 런 `34803197814` 는 잡 7/7 success(확인). 읽기 전환이 `## 개정` 에 없다(자기 산출 절 1) |
| F4 | 통과 | total_count 1 · 보고 줄 1 | `oracle/F4-g5.txt` `{"runs":[],"total_count":0}` → `report.md:68` 한 줄에 `G5` 와 `Actions` |
| G1 | 통과 | 더한 줄 매치 0 (HEAD 까지) | `git diff 6ee9eb3..HEAD -- crates/ xtask/ \| grep '^+' \| grep -ciE 'ditto\|\.ditto\|~/\*\|bun-types'` → `0`. 음성 대조는 다시 안 돌렸다(게이트 RED 4 기록 있음) |
| G2 | 반증 | 목록 해시 1 쌍 | `oracle/G2-violation.txt` — `palimpsest_dir_listing_sha256` `cf6cd3d…` ≠ `0e07cb0…`. 원본은 다시 안 쟀다(불가침) |

## 미측정 목록

| # | 안 잰 조건 | 모집단 | 유효 | 해악도 | 좌표(파일:줄) | 왜 못 쟀나 |
|---|---|---|---|---|---|---|
| 1 | A3 — TS 해소 순서·모드 컴파일러 대조 | 원의도 | 추정 | 미관 | `.palimpsest/rounds/2026-09-13-first-release-elsewhere/intent.md:334` | ditto `node_modules/typescript` 를 부르는 대조를 이번 라운드에 안 돌렸다. 게이트 산출만 있다 |
| 2 | A3-a — 대조 스크립트 음성 대조 | 원의도 | 추정 | 미관 | `.palimpsest/rounds/2026-09-13-first-release-elsewhere/intent.md:335` | 같음 |
| 3 | A5 — ditto 정밀도·재현율 | 원의도 | 추정 | 미관 | `.palimpsest/rounds/2026-09-13-first-release-elsewhere/intent.md:337` | 새 복제본 + TypeScript `Program` 대조를 안 돌렸다 |
| 4 | A6 ⑵ — Rust 파일 간 엣지 집합 | 원의도 | 추정 | 미관 | `.palimpsest/rounds/2026-09-13-first-release-elsewhere/intent.md:338` | 착수 바이너리 워크트리 빌드를 안 했다. ⑴ 은 CI 가 대신한다 |
| 5 | B4 — ditto 표본 p95 | 원의도 | 추정 | 미관 | `.palimpsest/rounds/2026-09-13-first-release-elsewhere/intent.md:347` | ditto 복제본 타이밍을 안 돌렸다 |
| 6 | E1 — 효과 걸음의 커밋 순서·앵커 | 원의도 | 추정 | 미관 | `.palimpsest/rounds/2026-09-13-first-release-elsewhere/intent.md:362` | `oracle/E1-order.sh` 와 효과 복제본이 세션 스크래치에 있어 다시 안 돌렸다 |
| 7 | E2 — 장면 넷 | 원의도 | 추정 | 미관 | `.palimpsest/rounds/2026-09-13-first-release-elsewhere/intent.md:363` | `oracle/E2-scene.py` 를 다시 안 돌렸다 |
| 8 | E4 — 깨질 곳 대조 | 원의도 | 추정 | 미관 | `.palimpsest/rounds/2026-09-13-first-release-elsewhere/intent.md:365` | `tsc` 전후 대조를 다시 안 돌렸다 |

## 의도 축

### 빠진 것

| # | 발견 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 근거(명령·출력) |
|---|---|---|---|---|---|---|---|
| 1 | 원문 한 줄 「걸린 결정과 **깨질 곳**을 받는다」 가운데 깨질 곳의 자리가 `touch` 화면에 없다. 화면은 호출자 **수**만 싣는다. 소유자 답(`## 승격` 7 「수가 의도」)과 `#156` 분할로 게이트·보고에 적혀 있지만, 게이트 제목은 「첫 릴리스가 섰나 — 섰다」로 시작한다 | 원의도 | 참 | 거짓신호 | 어느 조건에도 안 걸림(E2 ⑵ 는 정정으로 수를 잰다) | `docs/gates/first-release-elsewhere.md:110` | HEAD 바이너리 TS 픽스처 `pal touch helper` → 「호출자 1 · 피호출자 0」만 있고 자리 줄이 없다 · `gh issue view 156` → OPEN |

### 요구되지 않은 것

| # | 발견 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 근거(명령·출력) |
|---|---|---|---|---|---|---|---|
| 2 | 레코드 추출기(규약 `.claude/`)의 좌표 규칙을 바꿨다. 원문은 「새 측정 장치 · 새 게이트 계열을 만들지 않는다. 규약이 요구하는 것만 만든다」다. 끝난 회차의 반환문에 옛·새 판을 대면 기계 칸 `경로` 가 3146 행 중 **50 행** 달라진다. 대부분 줄임·저장소 밖 좌표가 `(경로 없음)` 으로 떨어진다(`.../effect/build.py` → `(경로 없음)`, `NEXT-D-handoff.md` → `(경로 없음)`). 좋아진 것도 있다(`git diff … F11-touch.md F12.md` → `docs/gates/F11-touch.md`). `xtask` 「회차 레코드」 판정문의 「손으로 채운 칸 — 끝난 회차 229 [갈린 칸 225]」에 이 추출기 드리프트가 사람 전사로 섞여 세어지는지는 **추정**이다(검사는 보고만 하고 안 빨갛다) | 규약 | 참 | 거짓신호 | 어느 조건에도 안 걸림 | `.claude/skills/round/bin/extract.py:421` | 스크래치 클론에서 `git show 6ee9eb3:…/extract.py` 와 HEAD 판을 `.palimpsest/rounds/*/{review,premortem,dialectic,conditions-audit}/r*-raw.md` 전부에 돌림 → `rows 3146 경로 differ 50 fail 0` |
| 3 | `xtask` 「회차 레코드」 검사에 이력 면제(기준커밋 트리에 있던 좌표는 통과)를 더했다. 원문이 요구하지 않았고 보고도 그렇게 적는다(`report.md:78`). 음성 대조 시험이 있고 지금 면제는 5 행이다 | 저장소 | 참 | 미관 | 어느 조건에도 안 걸림 | `xtask/src/main.rs:4189` | HEAD 클론 `cargo xtask check` → `검사 29/29 통과` · 「5행 (기준커밋에는 있었고 뒤에 지워졌다)」 |

### 있는데 틀린 것

없음 — 원 의도 대상에서 잰 자리(A1-a·A7·B3 ⑴·D2 의 실제 바이너리 화면, TS 화면의 언어 중립 문구)에서 어긋남을 못 찾았다. 틀린 것은 회차 기록에서 나왔고 아래 자기 산출 절에 있다.

## 이번 라운드의 새 발견

| # | 발견 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 근거(명령·출력) |
|---|---|---|---|---|---|---|---|
| 4 | Windows 잠금 수정이 순서를 바꿨다. `pal radius` 가 **없는 결박**을 거부하기 전에 HEAD 원장 전체를 먼저 계산한다(앞 판은 결박 조회가 먼저였다). 그리고 의도 저장소를 읽고 닫았다가 기록 직전에 다시 연다. 그 사이 다른 프로세스가 같은 결박을 바꾸면 `옛` 기준의 `새` 가 덮어쓴다(단일 사용자 CLI 라 **추정**). CI 는 세 OS 초록이다 | 저장소 | 참 | 미관 | 어느 조건에도 안 걸림(`## 승격` 9·11 의 수정) | `crates/pal-cli/src/radius.rs:135` | `git diff 6ee9eb3 264d967 -- crates/pal-cli/src/radius.rs` — `let head = ledger::compute(…)` 가 `intent.get(&id)` 의 `bail!` 앞으로 옮겨졌고 `:232` 에서 `IntentStore::open` 을 다시 부른다 · 런 `34803197814` `windows-latest success` |

(의도 축 1~3 은 여기 다시 싣지 않는다.)

## 자기 산출에 대한 발견

| # | 발견 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 근거(명령·출력) |
|---|---|---|---|---|---|---|---|
| 5 | `F2` 의 상자와 게이트가 「통과」다. 그러나 조건 문면 「회차의 **마지막 커밋** SHA 에 success 런」의 마지막 커밋 `264d967` 에는 런이 0 이다. 게이트는 「main 에 올린 마지막 push `067c7c4`」로 읽었고 그 전환을 근거 줄에 드러냈다. 하지만 `## 개정` 표에는 줄이 없고(두 줄 p1·p2) 게이트는 「개정 둘」이라 적는다. `E2`/`E4` 는 같은 꼴의 문면 전환에 개정 줄과 소유자 답을 요구받았다. 규약 §11 ⑥ 「push 됐고」도 HEAD 에서 글자대로 서지 않는다(R3 기록 커밋도 로컬로 남는다). 로컬 HEAD 의 `cargo xtask check` 는 29/29 초록이라 안 올린 커밋이 CI 를 깰 근거는 못 찾았다 | 회차기록 | 참 | 거짓신호 | F2 | `.palimpsest/rounds/2026-09-13-first-release-elsewhere/intent.md:370` | `gh run list --commit 264d967fbc47b6b761d2bc096eeef9f2afb8dc0d --json databaseId,conclusion` → `[]` · `gh run view 34803197814` → headSha `067c7c4d…` success · `intent.md:382-383` 개정 두 줄 · `docs/gates/first-release-elsewhere.md:92` 「줄은 둘이다」 |
| 6 | 교대 문서가 PR #157 을 「draft」로 적는다. 실제로는 병합됐다 | 회차기록 | 참 | 거짓신호 | F2 | `.palimpsest/rounds/2026-09-13-first-release-elsewhere/state.md:23` | `gh pr view 157 --json state,isDraft,mergedAt` → `"state":"MERGED","mergedAt":"2026-09-14T03:36:40Z"` |
| 7 | 게이트 `A6` 근거 줄 「그 뒤 크레이트 변경(`c9e31a3`)은 시험 두 개뿐」이 낡았다. `5b4a37f` 뒤 크레이트를 고친 커밋은 넷이고 그중 셋(`1bcdc93` · `3f69ccd` · `51de0e8`)이 제품 코드 `radius.rs` 다. 그 줄은 `4693b65`(radius 수정 전)에 쓰였다. 「마지막 커밋의 시험 전량은 F2 의 CI 가 진다」가 실제로 덮어서 판정은 안 뒤집힌다. `G1` 줄의 범위 `6ee9eb3..c9e31a3` 도 같이 낡았지만 HEAD 까지 다시 재도 0 이다 | 회차기록 | 참 | 거짓신호 | A6 · G1 | `docs/gates/first-release-elsewhere.md:69` | `git log --format='%h %s' 5b4a37f..HEAD -- crates xtask` → 네 줄 · `git log -S'시험 두 개뿐' -- docs/gates/first-release-elsewhere.md` → `4693b65` · `git show c9e31a3 --stat` → `classify.rs` 는 `mod tests` 안 시험 하나 |
| 8 | 착수 때 §11 ③ 의 **(가) 장치를 회차 안에서 만들지 않는다**를 골랐다. 그런데 종료 보고는 「만든 것은 넷이다」(추출기 좌표 규칙 · `xtask` 이력 면제 · 회차 오라클 · 어휘 검사 함수)라 적고 「전부 이 회차에서 닫혔다 — 다음 회차가 받는 것 없다」로 끝낸다. 규약이 「(가)도 (나)도 아닌 것 — 이 회차 안에서 장치까지 완성하고 그 장치에 대한 발견도 다 닫는다 — 이 안 된다」고 한 길이다. 선택 번복의 기록도 없다 | 회차기록 | 참 | 거짓신호 | 어느 조건에도 안 걸림 | `.palimpsest/rounds/2026-09-13-first-release-elsewhere/report.md:85` | `intent.md:265` 「(가) 장치를 회차 안에서 만들지 않는다」 · `report.md:85-94` 표 넷 · `.claude/skills/round/SKILL.md:920-921` |
| 9 | 잠긴 의도의 `## 범위 밖` 에 승인 뒤 줄 둘이 더해졌다(기준선 빨강 `#155` · `touch` 호출자 자리 `#156`). 규약 §10 은 「범위 밖 — 착수 때 정한 것. 회차 중에 새로 생기지 않는다」다. 보고가 그 사실을 적었다(`report.md:98`) | 회차기록 | 참 | 미관 | 어느 조건에도 안 걸림 | `.palimpsest/rounds/2026-09-13-first-release-elsewhere/intent.md:291` | `git log -S'기준선의 기존 빨강'` → `d1f8ddb 2026-09-13 21:11` · 승인 `66fc201 2026-09-13 20:49` · `git log -S'#156'` → `403d589 2026-09-14 07:45` |

## 내가 기각한 것

| # | 기각한 것 | 모집단 | 유효 | 해악도 | 좌표(파일:줄) | 왜 아니었나 |
|---|---|---|---|---|---|---|
| 10 | CI 가 얕은 클론이면 추출기의 `git log --diff-filter=A` 와 `xtask` 이력 면제의 `git ls-tree <기준커밋>` 이 다른 답을 내 CI 가 빨개진다 | 규약 | 거짓 | 실패 | `.github/workflows/ci.yml:90` | CI 체크아웃이 `fetch-depth: 0` 이다 |
| 11 | `xtask` 이력 면제가 한 번도 없던 좌표까지 삼켜 「회차 레코드」 검사가 죽은 가지가 된다 | 저장소 | 거짓 | 금지역 | `xtask/src/main.rs:5943` | `기준커밋에_있었나` 는 git 실패·커밋 없음·빈 좌표에서 `false` 이고, 시험 `기준커밋에_있다가_지워진_좌표만_면제한다` 가 `never-there.md` 와 `0000000` 을 `false` 로 단언한다(코드 읽기 · 시험은 로컬에서 안 돌렸고 CI 초록) |
| 12 | push 안 한 전사 커밋 `264d967` 이 `cargo xtask check`(원장 둘 대조 등)를 빨갛게 만든다 | 회차기록 | 거짓 | 실패 | `docs/gates/first-release-elsewhere.md:48` | 스크래치 클론 HEAD `cargo xtask check` → `검사 29/29 통과` · 「원장 둘 대조 ok … 2026-09-13-first-release-elsewhere (29개)」. crates·xtask 는 `067c7c4` 와 같다 |
| 13 | TS 사용자 화면에 Rust 의 뜻(「크레이트 뿌리」·「연관 상수」)이 아직 실린다 | 원의도 | 거짓 | 거짓신호 | `crates/pal-cli/src/touch.rs:1` | 효과 산출 `effect/04-touch-after-approve.txt` 에는 있었으나, HEAD 바이너리 TS 픽스처 화면은 「패키지 뿌리 파일의 재수출」 · 「열거형 멤버처럼」이다 |
| 14 | radius 수정이 Windows 에서 임시 색인 `radius-base-*.redb` 를 못 지우는 회귀를 만들었다 | 저장소 | 거짓 | 미관 | `crates/pal-cli/src/radius.rs:243` | `let _ = std::fs::remove_file(&base_index);` 는 diff 의 문맥 줄로, 이 회차가 바꾸지 않았다 |
| 15 | 종료 보고의 `## 하네스가 설계대로 안 돈 것` 은 네 금지 이름을 새 이름으로 바꾼 잔여 목록이다 | 회차기록 | 거짓 | 거짓신호 | `.palimpsest/rounds/2026-09-13-first-release-elsewhere/report.md:72` | 항목이 미룬 일이 아니라 일어난 사고와 대가다. 앞 회차 `2026-09-11-effect-confirmation/report.md` 도 같은 절을 쓴다. 네 금지 이름은 `grep` 0 |
| 16 | 개정 p2 가 재측정 뒤에 쓰였는데 보고가 「개정 커밋은 재측정보다 앞선다」고 거짓으로 적는다 | 회차기록 | 거짓 | 금지역 | `.palimpsest/rounds/2026-09-13-first-release-elsewhere/intent.md:382` | `403d589 07:45:14` < `eb9c936 07:52:51`. 「두 결과를 다 본 뒤 썼다」도 함께 적혀 있다 |
| 17 | 「`pal doctor` 불변식 위반 0」이 사실이 아니다 | 회차기록 | 거짓 | 거짓신호 | `docs/gates/first-release-elsewhere.md:118` | HEAD 클론 `pal doctor` → 「■ 위반 (0)」 · 불변식 1~3 위반 0 |
| 18 | `radius.rs` Windows 수정은 원문에 없는 요구되지 않은 산출이다 | 저장소 | 거짓 | 미관 | `crates/pal-cli/src/radius.rs:129` | `## 승격` 9 「고치고 한 번 더 push」 · 11 「브랜치 PR 로 Windows CI 먼저」 — 소유자 답이 들였다 |
| 19 | 실제 바이너리에서 맨 지정자가 `outside_repo` 로 적힌다(A1-a 가 시험에서만 선다) | 원의도 | 거짓 | 금지역 | `crates/pal-core/src/cross_file.rs:1` | HEAD 바이너리 픽스처: `z — bare_specifier` · `readFile — outside_repo` · `gone — no_target_file` 로 갈린다 |

## 끝내도 되는가

된다 — 본 목록(원의도·저장소·규약)에 금지역·실패가 0 이다. 단 자기 산출 절 5(`F2` 를 「main 의 마지막 push」로 읽은 것이 `## 개정` 에 없고, HEAD 가 push 안 돼 §11 ⑥ 이 글자대로 안 선다)는 메인·소유자가 개정 줄로 받거나 판정을 되돌릴지 정해야 한다.
