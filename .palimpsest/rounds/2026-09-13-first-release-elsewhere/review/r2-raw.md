# 독립 리뷰 라운드 2 — 원문

> 회차 `2026-09-13-first-release-elsewhere` · 잰 날 2026-09-14 · HEAD `5b4a37f`
> **읽기 전용이다.** 리뷰어는 저장소를 고치지 않았고 빌드·복제는 스크래치에만 했다. 원본 `~/dev/projects/ditto` 에는 읽기만 댔다.
> **옮겨 적은 자:** 메인. `pal-independent-reviewer` 반환문의 마지막 ```markdown 블록 본문을 바이트 그대로 옮겼다 — 이 머리 넷만 더했다. 리뷰어가 안 받은 것: 대화 기록 · 메인의 사고 과정 · 앞 라운드(R1) 결과.

## 합격선 축

| 조건 | 판정 | 잰 수 | 근거 |
|---|---|---|---|
| A1 | 통과 | 재실행 — `ts_cross_file` 19 통과 · 0 실패 | 시험 파일 재실행(스크래치 target). RED `oracle/A1-red.txt` 끝 「파일 간 엣지 0 []」 읽음 |
| A1-a | 통과 | 같은 시험 파일 | 같음 |
| A2 | 통과 | 같은 시험 파일 | 같음 |
| A2-a | 통과 | 같은 시험 파일 | 같음 |
| A3 | 통과 | 산출 대조(재실행 아님) | `oracle/A3-fixture.txt:172` 「합산 표본 0 인 가지: 없음」 · 픽스처마다 불일치 0 · `:179` rc 0 |
| A3-a | 통과 | 산출 대조 | `oracle/A3-fixture.txt:175` 잰 엣지 7 · 불일치 1 |
| A4 | 통과 | 재실행 — `ts_cross_file` | 같음 |
| A5 | 통과 | ⑴ 재실행 1 · ⑶⑷ 산출 대조 · ⑵ 산출만 | HEAD 바이너리 `pal 0.0.0+5b4a37f08e8b` · 새 복제본 `aded7ce` 의 `touch codexHostAdapter` 가 `cross-file-import 7199/11099`. `oracle/A5-ditto.txt:46` 잰 엣지 5082 · 불일치 0 · `:55` 모집단 2915 · 빠진 것 0 |
| A6 | 미측정 | 0 | 종료 걸음에서 잰다(메인 지시) |
| A7 | 통과 | 재실행 — `cross_file_references` 25 통과 · HEAD 화면 1 | HEAD 화면에 「호출자 수는 하한입니다 — 파일 최상위의 참조와 문자열로 찾는 자리는 세지 않습니다」 |
| B1 | 통과 | 재실행 — `pending_and_pick` 14 통과 · HEAD 화면 2 | 새 복제본 `touch codexHostAdapter` 에 대기 구역 (1): 경로 · 앵커 · 「후보 7곳」 · 승인 명령 |
| B1-a | 통과 | 시험 | 같음 |
| B2 | 통과 | 시험 + HEAD 실측 1 | 화면 명령 `pal narrative --approve decision/01M2ENFEKS1GPTWMHFBZS5A8MK --pick d728cbcee5e3` 을 그대로 실행하니 rc 0 → 이어진 `touch` 가 `■ 이 좌표에 걸린 것 (1)` · 「최신 상태(fresh)」. RED `oracle/B2-red.txt` rc=2 |
| B3 | 통과 | 시험 | `pending_and_pick` 재실행 · HEAD 새 복제본(인입 전) 화면에 「아직 만들지 않았습니다」는 `oracle/A5-touch-final.txt` 에서만 봤다 |
| B4 | 통과 | 산출 대조(재실행 아님) | `oracle/B4-timing.txt:10,18` p95 2.2 ms 두 표본 · `:20` 표본 ② 대기 50 · 후보 화면 0 |
| C1 | 통과 | 시험 + HEAD 실측 2 | 후보 화면 지목 `5bd759951084` → `touch loadInstructions --pick 5bd759951084` rc 0 · 한 심볼 답. `query symbol.callers loadInstructions` 후보 화면이 같은 지목 문자열을 싣는다 |
| C1-a | 통과 | 시험 | 같음 |
| D1 | 통과 | 시험 13 통과 + HEAD 화면 17 스캔 | `PAL_VOCAB_SCAN` 으로 새 복제본의 install · narrative · touch(찾음·후보·못 찾음) · query · help 셋 · 승인 · 지목 화면. `pal` 문구에서 걸린 곳 0 — 걸린 10 은 전부 ditto 문서 본문(기각 표 #13) |
| D1-a | 통과 | 시험 | 같음 |
| D2 | 통과 | 시험 + 산출 대조 | `oracle/D2-commit.txt` 「9db40b7 한 커밋」 |
| E1 | 통과 | 산출 대조 + 음성 대조 재실행 3 | `oracle/E1-order.txt` 이웃 쌍 다섯 ✓ · 앵커 ✓. `E1-order.sh --self-test` 를 다시 돌렸다: ①②③ 전부 「걸렸다」 · rc 0 |
| E2 | 통과 | 산출 대조 · ⑴⑶ 은 HEAD 에서 같은 꼴 재현 | `oracle/E2-scene.txt` ⑴~⑷ ✓ · 증인 `05 codexHostAdapter` |
| E3 | 대조불가 | — | `intent.md ## 승격` 4~6 의 소유자 답과 게이트:79 · 보고:14 가 같다 |
| E4 | 통과 | 산출 대조 | `oracle/E4-breakage.txt` P∖(C∪D) = `src/core/hosts/index.ts` · 오류 자리 최상위 · 선언 안 0 · rc 0 |
| F1 | 통과 | `gh` 조회 1 | `#135` 마지막 코멘트가 `7199/11099` 와 `docs/gates/first-release-elsewhere.md` 를 싣는다. 수는 HEAD 재실행과 같다 |
| F2 | 미측정 | 0 | `origin/main` = `6ee9eb3` — 회차 커밋 90 이 아직 push 전이다 |
| F4 | 통과 | `gh` 재조회 2 | `actions/runs?head_sha=6ee9eb3…` total_count 0 · `check-runs` total_count 0 · `report.md:56-58` 에 `G5`·`Actions` 줄이 있다. ★ 그 줄의 원인 문장은 거짓이다 — 의도 축 #3 |
| G1 | 통과 | 재측정 — 더한 줄 0 · 음성 대조 1 | `git diff 6ee9eb3..5b4a37f -- crates/ xtask/` 더한 줄 0 · `+ // ditto` 심은 입력 1 |
| G2 | 반증 | 재박제 1 | 지금 원본: HEAD `aded7ce` · porcelain `3750611…` · 목록 해시 `0e07cb0…` · node_modules 16. 뒤 박제와 같고 앞 박제 `cf6cd3d…` 와 다르다 |

검산 — 통과 25 · 반증 1 · 대조불가 1 · 미측정 2 = 29. 게이트의 표준 표와 ID 집합이 같다. 음성 대조가 등록되고 확인된 것: A1-a · A2-a · A3-a · B1-a · C1-a · D1-a(시험) · E1(재실행) · G1(재실행). RED 가 실제로 빨갰던 관측은 A1 · B2 · D1 산출에서 읽었다.

## 미측정 목록

| # | 안 잰 조건 | 모집단 | 유효 | 해악도 | 좌표(파일:줄) | 왜 못 쟀나 |
|---|---|---|---|---|---|---|
| 1 | A6 — 최종 커밋 시험 전량 · Rust 파일 간 엣지 집합 | 원의도 | 참 | 실패 | `.palimpsest/rounds/2026-09-13-first-release-elsewhere/intent.md:338` | 종료 걸음에서 잰다. 리뷰 중 워킹트리에 `oracle/A6-rust.txt` 수정과 `oracle/A6-xtask-test.txt` 새 파일이 생겼다 — 이 리뷰의 명령은 스크래치에만 썼다 |
| 2 | F2 — 마지막 커밋 SHA 의 CI `conclusion=success` | 원의도 | 참 | 실패 | `.palimpsest/rounds/2026-09-13-first-release-elsewhere/intent.md:370` | push 전이다(`origin/main` = `6ee9eb3`) |
| 3 | A3 · A5 ⑵ · B4 · E2 ⑵ · E4 의 재실행 | 원의도 | 참 | 거짓신호 | `.palimpsest/rounds/2026-09-13-first-release-elsewhere/oracle/A5-ditto.txt` | TypeScript 대조 · 타이밍 · 효과 복제본 재구성을 이 라운드는 안 돌렸다. 커밋된 산출만 읽었다 |

## 의도 축

### 빠진 것

없음 — 원문 장면 네 줄(걸린 것 ≥ 1 · 호출자가 파일 경계를 넘은 수 · 동명 고르기 · 내부 어휘 없음)이 HEAD 바이너리와 새 복제본에서 다시 섰다. 「깨질 곳의 자리」는 `## 승격` 7 이 좁혔고 `#156` 으로 섰다. A6 · F2 는 위 미측정이다.

### 요구되지 않은 것

| # | 발견 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 근거(명령·출력) |
|---|---|---|---|---|---|---|---|
| 1 | 레코드 추출기의 좌표 규칙을 바꿨다(반환문이 들어온 커밋 트리에 대 고르기 · 절대경로 갈래). 원문 「새 측정 장치 · 새 게이트 계열을 만들지 않는다. 규약이 요구하는 것만 만든다」 밖이다. 보고:68 이 인정한다 | 규약 | 참 | 미관 | 조건 없음 | `.claude/skills/round/bin/extract.py:421` | `git show --stat eea73db 4abfb96` — extract.py +63/+12. `cargo xtask check` 29/29 통과 |
| 2 | `xtask` 「회차 레코드」 검사에 새 면제 `이력_면제`(기준커밋 트리에 있었던 좌표)와 그 시험을 더했다. 원문의 같은 줄 밖이고, 보고:68 은 「추출기」만 이름하며 검사 면제 신설은 이름하지 않는다 | 저장소 | 참 | 미관 | 조건 없음 | `xtask/src/main.rs:4186` | `git diff 6ee9eb3 5b4a37f -- xtask/src/main.rs` — `기준커밋에_있었나` · 판정문에 「기준커밋에는 있었고 뒤에 지워졌다」 수를 싣는다 |

### 있는데 틀린 것

| # | 발견 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 근거(명령·출력) |
|---|---|---|---|---|---|---|---|
| 3 | 종료 보고가 앞 회차 `G5` 의 원인을 「설정이 막은 것이 아니라 그 커밋이 push 로 올라간 적이 없어서다」로 사실처럼 적었다. 거짓이다 — `6ee9eb3` 은 push 됐고 런이 0 이다. 원문 착수 첫 걸음 1 「소유자가 어디를 봐야 하는지 한 줄로」가 소유자를 틀린 원인으로 보낸다. 같은 회차의 `intent.md:124` 와 차선책 `:307`(「push 이벤트는 있는데 런이 0」)과도 모순이다 | 회차기록(금지역 예외) | 참 | 금지역 | F4 | `.palimpsest/rounds/2026-09-13-first-release-elsewhere/report.md:58` | `gh api repos/hskim-ecoletree/palimpsest/events` → `PushEvent 2026-09-13T09:12:03Z head=6ee9eb30f2dc… ref=refs/heads/main` · `git rev-parse origin/main` = `6ee9eb3…` · `.github/workflows/ci.yml:46-48` `on: push: branches: [main]` · 경로 필터 없음 · 앞 회차 `observations/G5-push.txt` 가 `a7e5a05` push 와 런 0 을 적었다. 금지역 출처: `.claude/pal/policy.toml` 없음 → `intent.md ## 금지역` = 기본 다섯의 「사실이_아닌_것을_사실로」 |

## 이번 라운드의 새 발견

| # | 발견 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 근거(명령·출력) |
|---|---|---|---|---|---|---|---|
| 4 | 개정 p1 의 NUL 규칙 뒤로 확장자만 `.ts` 인 이진 파일은 `binary` 가 아니라 `unsupported · 문법이 못 읽음` 으로 적히고, 대장의 언어 줄이 그 파일들을 TypeScript 파일로 센다. 거짓 심볼은 없다(선언 0). 보고:86 은 이것을 「원리상 못 잰 것 — 표본이 없다」로 적었다 | 원의도 | 참 | 거짓신호 | A5 ⑷(개정 p1 확대) | `crates/pal-extract/src/classify.rs:137` | 스크래치 픽스처(`ok.ts` + NUL 이 든 이진 `.ts` 둘) → `pal ledger`: 「parsed 1 · unsupported 2 · 문법이 못 읽음 2 · binary 0」 · 「TypeScript L2 exact 3 파일」. `pal symbols src/stream.ts` → 「선언 0」 |

## 자기 산출에 대한 발견

| # | 발견 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 근거(명령·출력) |
|---|---|---|---|---|---|---|---|
| 5 | 게이트와 보고가 `oracle/A5-touch-final.txt` 를 「최종 바이너리」 산출이라 부른다. 그 파일은 `c874d9f` 가 지운 설명문(「크레이트 뿌리」 · 「열거형 변형 · 연관 상수」)을 싣고 버전 머리가 없다 — 수정 전 바이너리가 뜬 것이다. 수 `7199/11099` 는 참이다 | 회차기록 | 참 | 거짓신호 | F1 | `docs/gates/first-release-elsewhere.md:81` | `grep -n 크레이트 oracle/A5-touch-final.txt` → `:22` 「크레이트 뿌리의 재수출」 · `git log -- oracle/A5-touch-final.txt` = `720e6ff`(`c874d9f` 뒤 커밋). HEAD 바이너리 화면은 「패키지 뿌리 파일」 · 수 7199/11099 같음 |
| 6 | 「원리상 못 잰 것」 두 줄이 잔여다. ② 1급 확장자를 단 비소스 파일은 픽스처로 1 분 안에 쟀다(#4). ③ 「`touch` 수 = `symbol.callers` 줄 수」의 일반성은 까닭이 「코드로 추적하지 않았다」로, 할 수 있었던 일이다. §10 「이 회차에서 그것을 할 수 있었는가」 | 회차기록 | 참 | 거짓신호 | 조건 없음 | `.palimpsest/rounds/2026-09-13-first-release-elsewhere/report.md:86` | 픽스처 실행 출력(#4). `crates/pal-query/src/lib.rs:839` `callers: p.callers(symbol.id)?.len()` · `:576` `for id in p.callers(start.id)?` — 두 자리가 같은 원천이라 추적 가능하다 |
| 7 | 교대 문서가 낡았다 — 「독립 리뷰 R1 진행 중」 · 「미측정 3(A6 · F1 · F2)」 · 「남은 것: report.md 초안 · #135 코멘트 초안 · 스크립트 수정 커밋」. 전부 이미 됐다. 새 컨텍스트는 이 파일을 받는다 | 회차기록 | 참 | 거짓신호 | 조건 없음 | `.palimpsest/rounds/2026-09-13-first-release-elsewhere/state.md:8` | `cat state.md` 대 `git log` — `be2ccb5` · `ed27c9b` · `96bfcb7` · `5b4a37f` 가 이미 섰다 |
| 8 | 어휘 검사 함수의 사용자 내용 지우기가 여러 줄 `body` 를 못 지운다 — 통째 문자열 `replace` 인데 화면은 본문을 들여쓰기로 줄마다 찍는다. 남의 저장소에서 결정 본문에 `M5` · `§` · `#7` 이 있으면 `pal` 문구가 아닌데도 빨갛다. `E2` ⑷ 는 ADR-0003 본문에 패턴이 없어 초록이었다. 거짓 초록 방향은 아니다 | 자기장치 | 참 | 거짓신호 | D1 · E2 ⑷ | `crates/pal-cli/tests/user_vocabulary.rs:55` | `PAL_VOCAB_SCAN=…/10-touch-after.out`(짝 `.out.json` 있음) → 「걸린 자리 10」 전부 ditto 본문 줄. 같은 JSON 에 `M5 source-mode` · `(#7)` · `§3 현재상태표` 가 있다 → True True True |
| 9 | `G1` 산출의 범위가 `6ee9eb3..6145849` 다. 그 뒤 `crates/` 커밋(`d734c04` · `c874d9f`)이 더 있는데 게이트는 그 산출로 통과를 적는다 | 회차기록 | 참 | 미관 | G1 | `.palimpsest/rounds/2026-09-13-first-release-elsewhere/oracle/G1-coupling.txt` | 산출 머리 「범위: 6ee9eb3..6145849」. HEAD 까지 재측정 0 — 판정은 선다 |
| 10 | 게이트가 E1 음성 대조를 「`--self-test` 셋」으로 인용하는데 그 실행 출력이 커밋되지 않았다(커밋 메시지 `1d7b3b8` 만 말한다) | 회차기록 | 참 | 미관 | E1 | `docs/gates/first-release-elsewhere.md:77` | `grep -rln self-test` 회차 디렉터리 → `E1-order.sh` 만. 재실행: ①②③ 「걸렸다」 · rc 0 |
| 11 | 보고가 「(가) 장치를 회차 안에서 만들지 않는다를 골랐다. 이 회차가 만든 장치에 대한 별도 목록이 없다」로 적는다. 그러나 회차는 검사 면제(#2) · 추출기(#1) · 어휘 검사 시험을 만들었고 원장에 자기장치 발견 17 이 있다(전부 닫힘). §11 ③ 음성 대조 「별도 목록이 매 회차 비어 있으면」에 그대로 걸리는 문장이다 | 회차기록 | 참 | 거짓신호 | 조건 없음 | `.palimpsest/rounds/2026-09-13-first-release-elsewhere/report.md:75` | `PATH=<스크래치 pal> python3 .claude/skills/round/bin/dashboard.py 6ee9eb3 …intent.md` → ⑦ 「자기장치 17」 · ⑨ 「닫힘 188 / 188」 |

## 내가 기각한 것

| # | 기각한 것 | 모집단 | 유효 | 해악도 | 좌표(파일:줄) | 왜 아니었나 |
|---|---|---|---|---|---|---|
| 12 | TS 화면에 Rust 어휘(「크레이트 뿌리」 · 「연관 상수」)가 아직 실린다 | 원의도 | 거짓 | 거짓신호 | `crates/pal-cli/src/touch.rs:593` | 효과 산출과 `A5-touch-final.txt` 에만 있다. HEAD 바이너리의 새 복제본 화면은 「패키지 뿌리 파일」 · 「열거형 멤버」다(`c874d9f`) |
| 13 | HEAD 화면 스캔의 걸린 10 이 `pal` 의 어휘 누출이다 | 원의도 | 거짓 | 금지역 | `crates/pal-cli/tests/user_vocabulary.rs:246` | 10 전부 승인한 ditto 설계 문서의 본문 줄이다(`M1~M4` · `M5` · `§3` · `(#7)`). `pal` 이 쓴 줄에서는 0. 도구 결함은 #8 로 따로 적었다 |
| 14 | `pal export` 화면의 `F19` · `R-21` 이 원문 「화면에 내부 어휘가 없다」를 어긴다 | 원의도 | 거짓 | 거짓신호 | `.palimpsest/rounds/2026-09-13-first-release-elsewhere/oracle/A5-ditto.txt:1` | 원문은 `pal touch` 장면이고, `D1` 모집단(인터뷰 · 계획 ㈃ 「장면 네 명령」)에 `export` 는 없다 |
| 15 | 보고의 `## 하네스가 설계대로 안 돈 것` 이 금지된 네 이름을 바꿔 단 목록이다 | 회차기록 | 거짓 | 거짓신호 | `.palimpsest/rounds/2026-09-13-first-release-elsewhere/report.md:62` | 항목이 전부 이미 고친 사건과 대가다 — 넘기는 일이 아니다. 앞 회차 `2026-09-11-effect-confirmation/report.md` 에도 같은 절이 있다. 네 이름 grep 0 |
| 16 | 뒤 박제 이후 ditto 원본에 또 쓰였다 | 원의도 | 거짓 | 금지역 | `.palimpsest/rounds/2026-09-13-first-release-elsewhere/oracle/G2-ditto-origin-after.txt` | `GIT_OPTIONAL_LOCKS=0` 으로 다시 뜬 네 값이 뒤 박제와 전부 같다 |
| 17 | HEAD 에서 시험 또는 `cargo xtask check` 가 빨갛다(`IR1-14` 재발) | 원의도 | 거짓 | 실패 | `xtask/src/main.rs` | 시험 파일 넷 71 통과 · 0 실패. `cargo xtask check` 「검사 29/29 통과」 · RC=0 |
| 18 | 보고의 「열린 발견 0 · 금지역 0 · 실패 0」이 원장과 다르다 | 회차기록 | 거짓 | 거짓신호 | `.palimpsest/rounds/2026-09-13-first-release-elsewhere/report.md:32` | 계기판 ⑨ 「닫을 수 있다 — 열린 발견 0 · 닫힘 188 / 188」. 이 R2 의 발견은 아직 원장에 없다 |
| 19 | 조건 수가 갈린다(본문 29 · 상자 · 판정 표) | 회차기록 | 거짓 | 거짓신호 | `docs/gates/first-release-elsewhere.md:57` | 상자를 셌다: A 10 · B 5 · C 2 · D 3 · E 4 · F 3 · G 2 = 29. 결정론적 28 + E3 = 29. 판정 표 25+1+1+2 = 29 |
| 20 | 착수 재료 `NEXT-E-*.md` 가 남았다 | 원의도 | 거짓 | 미관 | `NEXT-E-prompt.md` | `ls NEXT-E*` → 없음 |
| 21 | CI 경로 필터가 앞 회차 G5 의 런 0 을 설명한다(그러면 #3 이 부분적으로 옳다) | 저장소 | 거짓 | 거짓신호 | `.github/workflows/ci.yml:46` | `on: push: branches: [main]` · `pull_request` 뿐 · 경로 필터 없음 |

## 끝내도 되는가

안 된다 — 본 목록에 금지역 1(#3 `report.md:58` 의 거짓 원인 문장)이 남았고, A6 · F2 가 미측정이다. #3 을 고치고 A6 · F2 가 서면 본 목록에 금지역·실패는 남지 않는다. 나머지(#1 · #2 · #4 와 자기 산출 #5~#11)는 거짓신호·미관이다.
