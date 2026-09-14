# 독립 리뷰 라운드 4 — 원문

> 회차 `2026-09-13-first-release-elsewhere` · 잰 날 2026-09-14 · HEAD `800308c`(로컬 · origin/main `067c7c4`) · 상한 4 의 마지막
> **읽기 전용이다.** 리뷰어는 저장소와 원본 ditto 에 쓰지 않았다. 빌드·복제는 세션 스크래치에서 했다(그 과정에서 스크래치의 `pal/` 을 지우고 `check.txt` 를 덮었다고 스스로 보고했다 — 회차 기록은 둘을 가리키지 않는다).
> **옮겨 적은 자:** 메인. `pal-independent-reviewer` 반환문의 마지막 ```markdown 블록 본문을 바이트 그대로 옮겼다 — 이 머리 넷만 더했다. 리뷰어가 안 받은 것: 대화 기록 · 메인의 사고 과정 · 앞 라운드(R1~R3) 결과.

## 합격선 축

| 조건 | 판정 | 잰 수 | 근거 |
|---|---|---|---|
| A1 | 통과 | CI 시험 1 · 두 OS | `067c7c4` 런 `34803197814` 의 ubuntu·windows 로그: `a1_지정자_여섯_꼴이_다른_파일의_심볼로_가는_엣지가_된다 ... ok`. RED 는 `oracle/A1-red.txt` 를 읽기만 했다(다시 안 돌렸다) |
| A1-a | 통과 | 위 시험 안 단언 · 실바이너리 화면 1 | HEAD `800308c` 바이너리로 ditto 새 복제본에 `touch codexHostAdapter` → `bare_specifier 2309` 가 `outside_repo 1494` 와 따로 적힌다. 까닭 줄은 「패키지 이름입니다 … 안 하기로 정한 자리입니다」 갈래이고 「원리상」 갈래가 아니다 |
| A2 | 통과 | CI 시험 6 · 두 OS | `a2_1` ~ `a2_6` 여섯이 전부 `ok` |
| A2-a | 통과 | CI 시험 1 · 두 OS | `a2a_패키지_extends_에서만_오는_별칭은_저장소_밖이_아니라_못_읽음이다 ... ok` |
| A3 | 미측정 | 0 | TypeScript 컴파일러 대조를 이번에 안 돌렸다. `oracle/A3-fixture.txt` 는 읽기만 했다 |
| A3-a | 미측정 | 0 | 같음 |
| A4 | 통과 | CI 시험 1 · 두 OS | `a4_재수출_배럴로_펴지는_임포트는_재수출을_지나는_이름이다 ... ok` |
| A5 | 통과(⑴ 만 직접) | 실바이너리 화면 1 | HEAD 바이너리 · 새 복제본 `aded7ce7` 에서 `cross-file-import 7199/11099`. ⑴ 선 것 > 0 은 섰다. ⑵⑶⑷ 는 `oracle/A5-ditto.txt` 와 #135 코멘트(모집단 2915 · 빠진 것 0 · 불일치 0)를 읽었고 컴파일러 대조는 다시 안 돌렸다 |
| A6 | 통과(⑴) · ⑵ 미측정 | CI 3 OS | ⑴ `067c7c4` 런에서 ubuntu·windows·macos 의 `cargo xtask test` step 이 전부 success. 그 뒤 `crates/` 변경은 0(`git diff --name-only 067c7c4 800308c` 에 crates 없음). ⑵ 엣지 집합 1319=1319 는 안 쟀다 |
| A7 | 통과 | 실바이너리 화면 2 | HEAD 바이너리 `touch` 두 화면에 「호출자 수는 하한입니다 — 파일 최상위의 참조와 문자열로 찾는 자리는 세지 않습니다」 |
| B1 | 통과 | CI 시험 1 · 실화면 2 | `b1_touch_가_승인_대기_조각을_경로_앵커_후보_수_승인_명령과_함께_싣는다 ... ok`. 실화면에 경로 · 앵커 · 「후보 3곳」 · 승인 명령이 실렸고 `걸린 것 (0)` |
| B1-a | 통과 | 위 시험 안 단언 | 음성 대조 `lonely` 는 시험 안의 단언이다(게이트 근거). 개별 출력은 안 셌다 |
| B2 | 통과 | CI 시험 1 · 실바이너리 1 회 | `b2_화면이_안내한_명령을_그대로_돌리면_통한다 ... ok`. 실측: 화면의 `승인:` 줄을 `shlex.split` 해서 고치지 않고 실행 → rc 0 → 다음 `touch` 가 `■ 이 좌표에 걸린 것 (1)` · 최신 상태(fresh) · 대기 (0) |
| B3 | 통과 | CI 시험 1 · 두 OS | `b3_목록이_없거나_낡으면_0_을_안_찍는다 ... ok` |
| B4 | 미측정 | 0 | p95 를 안 쟀다. `oracle/B4-timing.txt` 를 읽지 않았다 |
| C1 | 통과 | CI 시험 1 · 실화면 1 | `c1_… ... ok`. 실화면 `touch loadInstructions` 에 후보 2 · 후보마다 `지목 <hex>` · 「하나를 지목하려면 같은 명령에 --pick <지목> 을 붙이십시오」 |
| C1-a | 통과 | CI 시험 1 · 두 OS | `c1a_어느_후보에도_안_맞는_지목은_고르지_않는다 ... ok` |
| D1 | 통과 | CI 시험 1 · 실바이너리 출력 5 | `d1_장면_명령의_사람_화면에_작업_기록_어휘가_없다 ... ok`. 보조로 실출력 파일 다섯(install · narrative · touch 셋 · approve)을 조건의 패턴을 옮긴 파이썬 정규식으로 훑었다 → 걸린 것 1 곳뿐이고 사용자 경로 `.ditto/knowledge/adr/ADR-0003-toml-parser.md` 다(조건이 지우라고 한 사용자 내용). ⚠ 등록 함수(`PAL_VOCAB_SCAN`)가 아니다 |
| D1-a | 통과 | CI 시험 4 · 두 OS | `d1a_*` 넷이 `ok` |
| D2 | 통과(문자열) · 한 커밋 조건 미측정 | 실화면 1 | 실화면에 「안 하기로 정한 자리입니다」 · 「원리상 풀 수 없는 자리입니다」 둘 다. `9db40b7` 한 커밋 조건은 다시 안 쟀다 |
| E1 | 미측정 | 0 | `oracle/E1-order.sh` 를 안 돌렸다 |
| E2 | 통과(⑴⑶⑷ 를 HEAD 에서 재현) · ⑵ 산출 읽기 | 실바이너리 호출 4 | 효과는 `pal 0.0.0+db6c366`(`effect/00-seal.md`)으로 돌았다. HEAD `800308c` 로 다시 돌린 결과: ⑴ 결정 문서 조각 대기 → 안내 명령 승인 → `걸린 것 (1)` · fresh · 본문에 `mcpServersFromToml` 이 문자열로 있다 ⑶ 동명 후보 화면과 지목 문자열 ⑷ 위 D1 과 같음. ⑵ 증인 규칙은 다시 안 돌렸다(`호출자 2` 는 보았다) |
| E3 | 대조불가 | — | 정반합 판 뒤 소유자 결정(`intent.md` `## 승격` 4~6). 판을 다시 열지 않았다 |
| E4 | 미측정 | 0 | tsc 전후 대조를 안 돌렸다 |
| F1 | 통과 | 코멘트 1 | `gh issue view 135 --comments` 마지막 코멘트에 `7199/11099` · 모집단 2915 · 게이트 경로 `docs/gates/first-release-elsewhere.md` |
| F2 | 원문 문면으로는 반증 · 개정 읽기로는 통과 | 런 12 조회 | `067c7c4` 런 `34803197814` push success. 회차의 실제 마지막 커밋 `800308c`(그리고 `264d967` · `26ed32e`)은 push 되지 않아 런 0. 개정 줄(`intent.md:383`)은 있으나 까닭이 사실이 아니다(자기 산출 S3) |
| F4 | 통과 | report 줄 1 · API 1 | `report.md:66-68` 에 `G5` 와 `Actions` 가 한 절에 있다. `a7e5a05` 의 `total_count` 는 0 으로 조회했다. `6ee9eb3` 전체 SHA 조회는 `gh run list` 에 그 SHA 가 없는 것으로만 봤다 |
| G1 | 통과 | 더한 줄 전량 · 적중 0 | `git diff 6ee9eb3..800308c -- crates/ xtask/ \| grep '^+' \| grep -ciE 'ditto\|\.ditto\|~/\*\|bun-types'` → 0 (게이트는 `..264d967`, 그 뒤 크레이트 변경 없음). 음성 대조는 다시 안 돌렸다 |
| G2 | 반증 | 박제 항목 5 | 반증 자체는 `oracle/G2-violation.txt` · before/after 목록 해시가 진다. 이번에 원본을 git optional lock 없이 떠 보니 HEAD · porcelain 해시 · 278 줄 · `.palimpsest` 목록 해시 `0e07cb08…` · `node_modules` 16 이 `G2-ditto-origin-after.txt` 와 전부 같다. 박제 뒤로는 더 쓰인 것이 없다 |

## 미측정 목록

| # | 안 잰 조건 | 모집단 | 유효 | 해악도 | 좌표(파일:줄) | 왜 못 쟀나 |
|---|---|---|---|---|---|---|
| 1 | A3 · A3-a — TypeScript 컴파일러 대조 | 원의도 | 추정 | 거짓신호 | `.palimpsest/rounds/2026-09-13-first-release-elsewhere/intent.md:334` | 원본 `node_modules/typescript` 를 부르는 대조를 이번에 안 돌렸다. 산출 파일만 있다 |
| 2 | A5 ⑵⑶⑷ — ditto 전량의 정밀도·재현율 | 원의도 | 추정 | 거짓신호 | `.palimpsest/rounds/2026-09-13-first-release-elsewhere/intent.md:337` | 같음. ⑴ 과 선 것 수만 HEAD 바이너리로 쟀다 |
| 3 | A6 ⑵ — Rust 파일 간 엣지 집합 | 원의도 | 추정 | 거짓신호 | `.palimpsest/rounds/2026-09-13-first-release-elsewhere/intent.md:338` | 착수 워크트리 빌드가 한 번 더 필요해 안 했다. `5b4a37f` 뒤 크레이트 변경은 `radius.rs` 와 시험뿐이다 |
| 4 | B4 — p95 | 원의도 | 추정 | 거짓신호 | `.palimpsest/rounds/2026-09-13-first-release-elsewhere/intent.md:347` | 표본 100 호출을 안 돌렸다 |
| 5 | E1 — 효과 산출 커밋 순서·앵커 | 원의도 | 추정 | 거짓신호 | `.palimpsest/rounds/2026-09-13-first-release-elsewhere/intent.md:362` | 오라클 스크립트를 안 돌렸다. 효과 복제본이 세션 스크래치에 있다 |
| 6 | E2 ⑵ — 증인 심볼 규칙 | 원의도 | 추정 | 거짓신호 | `.palimpsest/rounds/2026-09-13-first-release-elsewhere/intent.md:363` | `symbol.callers` 줄 수 대조를 안 돌렸다 |
| 7 | E4 — tsc 전후 깨질 곳 대조 | 원의도 | 추정 | 거짓신호 | `.palimpsest/rounds/2026-09-13-first-release-elsewhere/intent.md:365` | 복제본 변경과 tsc 두 번이 필요해 안 했다 |
| 8 | D2 의 「한 커밋」 조건 · G1 음성 대조 | 원의도 | 추정 | 미관 | `.palimpsest/rounds/2026-09-13-first-release-elsewhere/intent.md:358` | `git log -p` 대조와 심은 입력을 다시 안 돌렸다 |

## 의도 축

### 빠진 것

없음. 원문 장면 네 줄은 HEAD 바이너리로 남의 저장소 복제본에서 다시 섰다(위 E2). 「깨질 곳의 자리」는 소유자 답(`## 승격` 7)으로 수가 의도로 정해졌고, 게이트 `### 첫 릴리스가 섰나` 가 그 사실을 적고 #156 으로 분할됐다.

### 요구되지 않은 것

없음 — 새로 낼 것이 없다. 장치 넷은 #158 이 이미 진다(자기장치). `radius.rs` Windows 수정은 소유자 답 `## 승격` 9 가 받았다.

### 있는데 틀린 것

없음.

## 이번 라운드의 새 발견

| # | 발견 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 근거(명령·출력) |
|---|---|---|---|---|---|---|---|
| 1 | 매니페스트가 저장소 식별자를 선언하지 않으면 식별자를 **디렉터리 이름**에서 유도한다. 그래서 같은 커밋·같은 결박 원장을 다른 이름의 디렉터리에 받은 사람의 `touch` 는 커밋된 결박이 있는데도 `■ 이 좌표에 걸린 것 (0) 아직 없습니다` 를 찍는다. `pal install` 뒤 ditto 도 선언이 없다(스냅샷 표기 `r4-ditto@aded7ce`). 원문 「남의 저장소에서 … 걸린 결정을 받는다」가 팀원이 클론한 자리에서 조용히 0 이 된다. 이 회차 이전 코드(`f8e93e9`)이고 R-08 이 경고한 형태로 문서화돼 있어 거짓신호로 적었다. 「아직 없습니다」를 사실 문장으로 찍는 점에서 금지역(사실이_아닌_것을_사실로)으로 올릴지는 메인이 판단할 몫이다 | 저장소 | 참 | 거짓신호 | 어느 조건에도 안 걸린다(의도 축) | `crates/pal-cli/src/ledger.rs:125` · `crates/pal-cli/src/ledger.rs:606` | 같은 `800308c` 클론·같은 바이너리·같은 `pal intent import`(결박 42)에서 디렉터리 이름 `pal` 이면 `touch TsProject` · `EXTRACTOR_REV` · `열쇠` 가 전부 `걸린 것 (0) 아직 없습니다`(스냅샷 `pal@800308c`, 지목 `eb6df4daf435`). 이름 `palimpsest` 이면 셋 다 `(1) 최신 상태(fresh)`(지목 `c059fff3b284` = `bindings.jsonl` 의 target 머리) |

## 자기 산출에 대한 발견

| # | 발견 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 근거(명령·출력) |
|---|---|---|---|---|---|---|---|
| S1 | 종료 보고의 상한 표가 독립 리뷰를 「R1 · R2」로 적고, 교대 문서도 「R1 · R2 처분」 · 「`96bfcb7` 뒤 R2 정정 반영」으로 적는다. R3 은 처분됐다(`26ed32e` · 원장 `IR3-*` 27 행 · `800308c`). 같은 보고 64 줄은 이미 「R3 · R4 처분」을 말해 한 문서 안에서 갈린다 | 회차기록 | 참 | 거짓신호 | — | `.palimpsest/rounds/2026-09-13-first-release-elsewhere/report.md:30` · `.palimpsest/rounds/2026-09-13-first-release-elsewhere/state.md:20` · `.palimpsest/rounds/2026-09-13-first-release-elsewhere/state.md:21` | `findings.jsonl` 에서 `id` 가 `IR3` 로 시작하는 행 27. `git log` 에 `26ed32e … 독립 리뷰 R3 을 고친다` |
| S2 | 「앞 회차의 착수 커밋 `6ee9eb3`」은 틀린 이름이다. 앞 회차의 착수 커밋은 `1276b8f` 이고, `6ee9eb3` 은 그 회차의 마지막 커밋(「push 를 기록한다」)이자 이 회차의 착수 커밋이다 | 회차기록 | 참 | 거짓신호 | F4 | `.palimpsest/rounds/2026-09-13-first-release-elsewhere/report.md:68` | `.palimpsest/rounds/2026-09-12-binding-radius-in-use/intent.md:3` 「착수 커밋 `1276b8f`」 · `git log -1 6ee9eb3` → `round(binding-radius-in-use): push 를 기록한다 — CI 가 안 떴고 G5 는 미측정이다` |
| S3 | F2 개정 줄의 까닭 「판정을 옮기는 커밋은 언제나 … 새 마지막 커밋이 되어 문면대로는 원리상 못 선다」가 사실이 아니다. 조건은 「마지막 SHA 에 success 런이 붙어 있다」는 세계의 상태이고, 전사 커밋을 push 하면 그 SHA 에 런이 붙는다. 이 회차 자신의 `067c7c4`(PR CI 초록을 기록한 회차기록 커밋)가 push 뒤 success 런을 받았다. 실제 까닭은 소유자 답 10 「push 안 함」이다. 또 이 개정은 조건을 「마지막 커밋」→「마지막 push」로 좁히므로 「정정」보다 축소에 가깝다 | 회차기록 | 참 | 거짓신호 | F2 | `.palimpsest/rounds/2026-09-13-first-release-elsewhere/intent.md:383` | `gh run list` → `067c7c4… push success 34803197814`. `git log -1 067c7c4` → `round(first-release-elsewhere): PR #157 의 Windows CI 초록을 남기고 보고 · state 에 push 이력을 적는다` |
| S4 | 종료 보고가 소유자 답 10(물음은 「CI 결과를 옮기는 전사 커밋을 push 하는가」)을 R3 · R4 처분 커밋까지 넓혀 「push 하지 않았다」를 정당화한다. 규약 §11 ⑥ 의 판정 문장(「회차의 마지막 커밋 SHA 에 success 런」)은 `800308c` 에서 서지 않는다. 원격 main 의 게이트는 여전히 `F2` 미측정 · 검산 26/1/1/1 이다 | 회차기록 | 참 | 거짓신호 | F2 · §11 ⑥ | `.palimpsest/rounds/2026-09-13-first-release-elsewhere/report.md:64` · `.palimpsest/rounds/2026-09-13-first-release-elsewhere/intent.md:400` · `.claude/skills/round/SKILL.md:961` | `git show 067c7c4:docs/gates/first-release-elsewhere.md` → `\| 미측정 \| F2 \|` · `검산 — 통과 26 · 반증 1 · 대조불가 1 · 미측정 1 = 29`. `origin/main` = `067c7c4` · 로컬 HEAD `800308c` 런 0 |

## 내가 기각한 것

| # | 기각한 것 | 모집단 | 유효 | 해악도 | 좌표(파일:줄) | 왜 아니었나 |
|---|---|---|---|---|---|---|
| 1 | 효과가 NUL 수정·언어 중립 문구·radius 수정 **전** 바이너리(`db6c366`)로 돌아, 최종 제품에서는 장면이 안 설 수 있다 | 원의도 | 거짓 | 거짓신호 | `.palimpsest/rounds/2026-09-13-first-release-elsewhere/effect/00-seal.md:13` | HEAD `800308c` 바이너리로 새 복제본에서 다시 돌렸다. install rc 0 → narrative rc 0(27.8 초) → 대기 1 → 안내 명령 rc 0 → `걸린 것 (1)` fresh · ADR-0003 본문 · 7199/11099 · 지목 안내. TS 화면의 「크레이트」·「연관 상수」도 사라졌다 |
| 2 | 이 회차의 결박 셋이 HEAD 에서 안 걸린다(`touch` 가 0) | 회차기록 | 거짓 | 금지역 | `.palimpsest/intent/bindings.jsonl:30` | 내 클론의 디렉터리 이름(`pal`)이 식별자를 바꾼 교란이었다. 이름 `palimpsest` 클론에서는 셋 다 `(1) 최신 상태(fresh)`. 교란 자체는 새 발견 1 로 옮겼다 |
| 3 | #158 이 자기장치 발견을 17 건으로 적어 원장 18 과 다르다 | 회차기록 | 거짓 | 미관 | `.palimpsest/rounds/2026-09-13-first-release-elsewhere/findings.jsonl` | 18 가운데 `CA1-20` 은 `거짓·기각` 이고 나머지 17 이 `정정` 이다. 「17 건 — 전부 닫혔다」는 정정된 것만 센 수로 읽힌다 |
| 4 | 「봉인 §4 는 그 노출을 적지 않았다」가 거짓이다 — §4 가 사전 승인 실험을 적는다 | 회차기록 | 거짓 | 거짓신호 | `.palimpsest/rounds/2026-09-13-first-release-elsewhere/effect/00-seal.md:48` | §4 는 「후보가 있다 · 한 번 승인해 봤다」만 적고 결정 본문(`smol-toml` · `shared.ts` wrapper)을 읽었다는 사실은 안 적는다. 보고 문장은 설 수 있고 소유자 답 5 가 받았다 |
| 5 | `## 원리상 못 잰 것` 의 `E3` 는 이 회차에서 다시 잴 수 있었으니 잔여다 | 원의도 | 거짓 | 거짓신호 | `.palimpsest/rounds/2026-09-13-first-release-elsewhere/report.md:111` | 못 되돌리는 것은 이미 커밋된 봉인의 입력(잠그기 전 실험의 원 출력)이다. 같은 세션에서 다시 돌려도 조율자가 ADR-0003 본문을 이미 알아 오염을 못 뺀다 |
| 6 | 종료 보고의 `## 하네스가 설계대로 안 돈 것` 이 금지 네 이름을 바꿔 단 잔여 목록이다 | 회차기록 | 거짓 | 거짓신호 | `.palimpsest/rounds/2026-09-13-first-release-elsewhere/report.md:72` | 항목 열이 전부 이미 일어난 대가이고 남은 일이 아니다. 금지 네 이름을 보고·게이트·교대 문서에 `grep` 하면 0 이다 |
| 7 | push 안 한 로컬 기록 커밋이 `cargo xtask check` 를 빨갛게 만든다 | 회차기록 | 거짓 | 실패 | `.palimpsest/rounds/2026-09-13-first-release-elsewhere/report.md:64` | `800308c` 클론에서 `검사 29/29 통과` · rc 0 |
| 8 | 「시험은 앞 회차 `ee93b35` 가 더했고 그 회차 push 에 런이 0 이라 아무도 못 봤다」가 거짓이다 — 9-12 에 success 런들이 있다 | 회차기록 | 거짓 | 거짓신호 | `.palimpsest/rounds/2026-09-13-first-release-elsewhere/report.md:58` | `git merge-base --is-ancestor ee93b35 1276b8f` → 아니다. 9-12 success 런은 전부 `ee93b35` 앞 커밋이고, 뒤의 push `a7e5a05` 는 `total_count` 0 |
| 9 | `radius.rs` 수정이 Windows 에서 새 잠금이나 회귀를 만든다 | 저장소 | 거짓 | 실패 | `crates/pal-cli/src/radius.rs:129` | 의도 저장소는 원장 둘을 다 센 뒤에만 다시 연다. `067c7c4` windows-latest 의 `cargo xtask test` success |
| 10 | 원본 ditto 가 종료 박제 뒤에도 더 바뀌었다 | 원의도 | 거짓 | 금지역 | `.palimpsest/rounds/2026-09-13-first-release-elsewhere/oracle/G2-ditto-origin-after.txt` | 지금 뜬 다섯 값이 after 파일과 한 글자도 다르지 않다 |
| 11 | 잠긴 의도 문서가 자기 안에서 어긋난다 — 체크박스 수 · 헤딩 중복 · 개정 수 | 회차기록 | 거짓 | 거짓신호 | `.palimpsest/rounds/2026-09-13-first-release-elsewhere/intent.md:378` | `- [x]` 29 · `- [ ]` 0 = 게이트 29 · `## ` 헤딩 중복 0 · `## 개정` 4 행 = 게이트 「개정 넷」 · `## 승격` 12 행 |

## 끝내도 되는가

된다 — 본 목록에 남은 것은 거짓신호 1(이 회차 이전 코드의 디렉터리 이름 식별자)뿐이고 금지역·실패는 없다. 다만 §11 ⑥ 을 문면대로 세울지(로컬 커밋 `264d967..800308c` push 와 런 확인), 소유자 답 10 의 범위를 다시 물을지는 메인이 정한다(S3 · S4).
