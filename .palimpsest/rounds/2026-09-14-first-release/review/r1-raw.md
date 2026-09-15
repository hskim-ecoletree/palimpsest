# 독립 리뷰 R1 — 원 반환문

> 회차 `2026-09-14-first-release` · 대상 `main` HEAD `75e9ca4` · 2026-09-15 · 상한 2 중 1
> 실행은 전부 스크래치의 `git clone --no-hardlinks` 사본에서 했다. 메인 체크아웃에서 쓴 것은 이 파일 하나다.
> ditto 원본은 `git clone` 의 원천으로만 읽었다 — 리뷰 전후 HEAD `aded7ce` · porcelain 278 줄 · `.palimpsest` 항목 7612 가 같다.

## 합격선 축

| 조건 | 판정 | 잰 수 | 근거 |
|---|---|---|---|
| A1 | 통과 | 시험 1 · 문자열 2 종 × 파일 2 | HEAD 클론 `cargo xtask test` 에서 `전_그래프_명령의_범위가_graph_dump_와_같은_값이다 ... ok` · `grep 'unresolved: 0,\|lowest_grade: ExtractGrade::L0,' crates/pal-cli/src/doctor.rs crates/pal-cli/src/export.rs` rc=1(0 곳) |
| A1-a | 통과 | 시험 2 · 파일 diff 1 | 같은 시험 ok · `git diff --stat ec92b89..HEAD -- crates/pal-cli/tests/query_envelope.rs` 빈 산출(무수정) · `범위는_질의마다_다른_값이다 ... ok` |
| A2 | 통과 | 갈래 5 | HEAD 에서 `a2_1`~`a2_5` ok(시험 전량 1184 통과 · 0 실패 · 3 무시) · RED `oracle/A2-red.txt` 에서 다섯 다 `FAILED`(left "fresh") 관측 |
| A2-a | 통과 | ⑴ 시험 1 · ⑵ TS 파일 497 · 심볼 4663 | ⑴ `a2a_1_속성_안의_주석과_공백만_바꾸면_fresh ... ok` ⑵ 착수 바이너리(`pal 0.0.0+ec92b899478d`) ↔ HEAD 빌드(`0.1.0+75e9ca4`)의 `pal symbols --json` (name·kind·span·body) — `corpus/tasks/f03-normalize-seeds.ts` 심볼 7 다른 것 0 · ditto **클론** `.ts/.tsx` 496 파일 · 심볼 4656 다른 것 0 · 실패 0 |
| A2-b | 통과 | 결박 42 | **HEAD 바이너리로 다시 쟀다**(오라클은 병합 전 `fe70606` 워크트리 바이너리) — `ec92b89` 클론 · 이름 `palimpsest` · porcelain 0 → 결박 42 = baseline/05 의 42 · id 집합 같음 · `fresh 28 · stale 14` 양쪽 같음 · 판정 이동 0. 모집단 20 은 오라클 판독(재집계 안 함) |
| A3 | 통과 | 시험 5 | `a3_` 로 걸리는 시험 5 ok · RED `oracle/A3-red.txt`(엣지 1) 판독 |
| A3-a | 통과 | 엣지 6194 / 6192 | ⑴ `a3a_…` ok ⑵ **HEAD 바이너리로 다시 쟀다** — `ec92b89` 트리 `graph.dump --at ec92b89`: 노드 3475 = 3475 · 파일 간 엣지 1338 → 1336 · 사라진 것 = `pal-query/src/lib.rs run → pal-core/src/lib.rs traverse(module)` · `pal-query/tests/bench.rs 한_규모 → 같은 선언` 둘 · 새로 생긴 것 0 |
| B1 | 통과 | 시험 3 | `repo_identity.rs` 의 `b1_` 걸림 3 ok |
| B1-a | 통과 | 갈래 4 | `b1a_1`~`b1a_4` 각각 ok |
| B2 | 통과 | 시험 3 | `caller_places.rs` 의 `b2_` 걸림 3 ok — 시험이 `기대.len() > 표시_상한` 전제 · 안내 명령 그대로 실행 · 기대 집합 대조(`:127`·`:155`·`:163`) |
| C1 | 통과 | 파일 3 · 자산 4 · README 명령 12 | 루트 `README.md` · `LICENSE-MIT`(23 줄) · `LICENSE-APACHE`(176 줄) 있음 · `gh release download v0.1.0` → `shasum -c` 넷 OK · 네 아카이브 전부 `README.md`·`LICENSE-MIT`·`LICENSE-APACHE` 든다 · 아카이브 README 와 저장소 README `diff` rc=0 · README 명령 줄을 D1 재실행에서 그대로 쳤다. ⚠ README 의 사실 한 줄이 틀렸다 — 조건은 그 줄의 참을 안 잰다(의도 축 1) |
| C2 | 통과 | 런 3 · 자산 5 | ⑴ `34859015942` workflow_dispatch `e80ca14` success 2026-09-14T14:57:43Z ⑵ 태그 객체 날짜 15:16:45Z → 태그 런 `34861114858` success 15:16:48Z(툴체인 없이 셋 · 받은 바이너리 `touch` 걸음 `release.yml:274-290`) ⑶ 자산 넷 + `SHA256SUMS` · 받은 바이너리 `pal 0.1.0+e80ca144101d` |
| D1 | 통과 | 걸음 13 | **릴리스 자산 바이너리로 README 걸음을 새 ditto 클론에서 다시 밟았다** — 승인 뒤 `■ 이 좌표에 걸린 것 (1)` · 호출자 자리 2(`src/cli/commands/setup.ts:258` · `src/core/setup.ts:291`) · export 2 줄 · `git add` rc 0 · 다른 이름 클론 import `결박 1` → `(1)` |
| D1-a | 통과 | ⑴ 걸음 13 · ⑵ 값 3 | ⑴ **착수 바이너리로 다시 밟았다** — 매니페스트 없음 · README `git add` rc=128(`pathspec '.palimpsest/manifest.toml'`) → 있는 파일만 · 팀원 import `결박 1` → `(0)` ⑵ 원본 HEAD · porcelain 줄 수 · `.palimpsest` 항목 수가 `effect/origin-after.txt` 와 같다(해시는 안 댔다 — 미측정 목록) |
| E1 | 통과 | 이슈 64 | `gh issue list --state all` 실시간 대조: R1 5 CLOSED/COMPLETED · R2 20 OPEN · R3 39 CLOSED/NOT_PLANNED · #127 마지막 코멘트가 `baseline/06-doctor-full.txt` 와 ADR-0007 을 싣는다 |
| E2 | 통과 | 낱말 9 · 산출 줄 1 · 문서 줄 2 | #160 실시간 본문에 9 낱말 전부 · 메인 체크아웃에서 `scripts/frontier.sh` 첫 줄 `#160 … ← 순서표의 1 번` · `docs/plan/03-shortest-path.md:207`(끝의 정의) · `:212`(Kotlin · Java · JavaScript · Python) |
| **F1** | **반증** | check 29 · test 1184 · CI 잡 7 | ★ **게이트와 갈린다.** HEAD `75e9ca4` 클론 `cargo xtask check` **rc=1** — ok 28 · `FAIL 어색한 표현 부재` 4 곳(`report.md:1` · `report.md:81` · `docs/gates/first-release.md:1` · `:102` 「~를 낸다」). HEAD push 의 CI 런 `34864067820` **failure — ubuntu · macos · windows 셋 다 `cargo xtask check` 걸음**. `cargo xtask test` 는 rc=0 · 1184 통과 · 0 실패. 게이트가 댄 `b0e8bb2` 런 `34862470231` success 는 사실이나, 그 뒤 판정 전사 커밋이 check 를 빨갛게 했다 — 문면의 「`cargo xtask check` 가 로컬에서 통과」가 지금 트리에서 거짓이고 `main` 이 세 OS 빨강이다 |

**검산** — 통과 16 · 반증 1(F1) · 대조불가 0 · 미측정 0 = 17.
음성 대조 — `A1-a` · `A2-a` · `A2-b` · `A3-a` · `B1-a` · `D1-a` 가 등록됐고, 그중 `A2-b` · `A3-a` ⑵ · `D1-a` ⑴ 은 이 리뷰가 다시 돌려 걸렸다(`D1-a` ⑴ 이 실제로 `(0)` 을 냈다).
RED — `A2` 다섯 `FAILED` 는 기록 판독, `D1-a` ⑴ 의 `(0)` 은 재실행 관측.
CI 에만 있는 걸음 — `palimpsest` 이름 클론에서 `doctor --full --json | node scripts/check-round-doctor.mjs` rc 0 · `MERGE_BLOCKER_DOCTOR_OK`.

## 미측정 목록

| # | 안 잰 조건 | 모집단 | 유효 | 해악도 | 좌표(파일:줄) | 왜 못 쟀나 |
|---|---|---|---|---|---|---|
| 1 | `D1-a` ⑵ 의 **해시 두 칸**(porcelain sha256 · `.palimpsest` 목록 sha256) — HEAD · 줄 수 · 항목 수만 댔다 | 원의도 | 참 | 미관 | `.palimpsest/rounds/2026-09-14-first-release/effect/origin-after.txt:3` | 오라클이 해시를 뜬 정확한 명령(정렬·경로 표기)이 파일에 안 남아 같은 값을 재현할 절차가 없었다 |
| 2 | `A2-a` ⑵ 의 **pal-cli 시험 픽스처 TS 넷**(alpha · beta · delta · binding_status `a.ts`) | 원의도 | 참 | 미관 | `.palimpsest/rounds/2026-09-14-first-release/oracle/A2a-ts-digest.txt:8` | `crates/pal-cli/tests` 아래에 `.ts` 파일이 없다 — 시험이 실행 중에 만드는 픽스처라 파일로 못 집었다. ditto 클론 496 파일로 대신 쟀다 |

## 의도 축

### 빠진 것

없음 — 계획 1~8 의 산출(#127 ① · #77 ①② · #139 · #159 · #156 · 받는 길 · 효과 · 처분)이 전부 있고, 원문의 「릴리스까지, 사용은 분할」「R3 철회, R2 유지」「ditto」가 각각 `v0.1.0` · #160 · 이슈 상태 · D1 로 섰다.

### 요구되지 않은 것

없음 — 워크트리 병렬 짜임과 `.claude/worktrees/` 잔여는 종료 보고가 스스로 적었고 커밋되지 않았다.

### 있는데 틀린 것

| # | 발견 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 근거(명령·출력) |
|---|---|---|---|---|---|---|---|
| 1 | README 가 **「그 밖의 파일은 대장에 「결박 불가」로 적힙니다」** 라고 적는데, 릴리스 바이너리는 Kotlin 파일을 **파싱하고 좌표를 낸다** — 결박 불가로 안 적힌다. 이 문장은 `v0.1.0` 네 아카이브에 그대로 실려 나갔다(되돌리려면 재릴리스). 방향은 과소 진술이지만 사실이 아닌 것을 사실로 적은 문장이다. 원문 인터뷰 설명문 「첫 릴리스는 TypeScript·Rust만 지원합니다」와 `C1` 의 「지원 언어(TypeScript · Rust)」는 **품질 약속**으로 읽히나, README 둘째 문장은 **동작**을 진술한다 | 원의도 | 참 | 금지역 — 사실이_아닌_것을_사실로(`intent.md:146` 등록) | C1(조건이 그 줄의 참을 안 잰다) | `README.md:9` | 스크래치 저장소에 `src/Main.kt`(class Greeter · fun top) · 릴리스 자산 `pal 0.1.0+e80ca144101d`: `pal symbols src/Main.kt` → `Kotlin · 추출기 f11-decor · 3 class Greeter · 7 fun top · 선언 2` · `query ledger.snapshot --json` → `"parsed": 2, "unsupported": 0, "unbindable_languages": 0` · `pal touch Greeter` → `class · src/Main.kt:3 · identity ordinal` · 아카이브 README ↔ 저장소 README `diff` rc=0 |
| 2 | README 가 `pal install` 이 쓰는 것을 **「`.claude/` 아래의 훅·스킬과 `.palimpsest/manifest.toml`」** 으로 열거하는데, 실제로는 **`CLAUDE.md` 블록과 `.gitignore` 블록도 쓴다**(없으면 새로 만든다). 사용자는 자기 `CLAUDE.md` 가 바뀐다는 것을 README 에서 못 읽는다 | 원의도 | 참 | 거짓신호 | C1(조건 밖) | `README.md:42` | HEAD 빌드로 빈 저장소에 `pal install` → `블록 CLAUDE.md` · `블록 .gitignore` · `git status --porcelain` → `?? .claude/` `?? .gitignore` `?? .palimpsest/` `?? CLAUDE.md` · 효과 산출 `effect/d1-release/02-install.txt` 도 같은 두 줄 |
| 3 | 잠긴 의도의 금지역 절이 **「이 저장소의 `.palimpsest/intent/bindings.jsonl` 은 덧붙이기만 한다」** 로 등록했는데, 결박 커밋이 머리 줄(`schema_version` 2 → 3)과 옛 결박 42 줄을 **다시 썼다.** 커밋 메시지와 종료 보고는 그 사실을 적었으나 `## 개정` 에 행이 없고, 게이트·보고의 「문면을 고친 자리 셋」은 이것을 안 센다. 내용 손실은 없다(아래 기각 8) | 원의도 | 참 | 거짓신호 | 어느 조건에도 안 걸림(금지역 절) | `.palimpsest/rounds/2026-09-14-first-release/intent.md:149` · `.palimpsest/intent/bindings.jsonl:1` | `git show e7423e2 --stat` → `47 insertions(+), 43 deletions(-)` · 두 판을 파싱: 옛 43 줄(머리 `schema_version 2`) · 새 47 줄(`3`) · 옛 id 42 ⊂ 새 id 46 · `decor` 칸을 뺀 차이 0 |

## 이번 라운드의 새 발견

| # | 발견 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 근거(명령·출력) |
|---|---|---|---|---|---|---|---|
| 4 | **판정 전사 커밋(`75e9ca4`)이 `cargo xtask check` 를 빨갛게 했고 `main` CI 가 세 OS 에서 실패한다** — 그런데 같은 커밋이 게이트에 `F1 통과` · 보고에 「반증 · 대조불가 · 미측정이 없다」를 적었다. 원인 줄은 회차 기록(보고 `:1` · `:81` · 게이트 `:1` · `:102` 의 「~를 낸다」)이나, 깨진 것은 원 의도가 직접 요구한 `F1`(로컬 check 통과 · CI)이다. 게이트 줄은 `e97cc27` 에 이미 있었고 보고가 들어오며 회차가 「끝난 회차」로 검사 모집단에 들어 드러났다(check 산출 「최근 끝난 회차 `2026-09-14-first-release` 가 검사에 들었다」) | 원의도 | 참 | 실패 | F1 | `docs/gates/first-release.md:1` · `docs/gates/first-release.md:102` · `.palimpsest/rounds/2026-09-14-first-release/report.md:1` · `.palimpsest/rounds/2026-09-14-first-release/report.md:81` · `xtask/src/main.rs:4814` | HEAD 클론 `cargo xtask check` → `FAIL 어색한 표현 부재` · `Error: 1개 검사가 실패했다` · rc=1 · `gh run view 34864067820` → `macos-latest failure cargo xtask check` · `ubuntu-latest failure cargo xtask check` · `windows-latest failure cargo xtask check` · 로그 `report.md:1 「~를 낸다」 … docs/gates/first-release.md:102 「~를 낸다」` · `git blame` → 게이트 `:1`·`:102` 는 `e97cc27`, 보고 `:81` 은 `75e9ca4` |

## 자기 산출에 대한 발견

| # | 발견 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 근거(명령·출력) |
|---|---|---|---|---|---|---|---|
| 5 | 교대 상태가 낡았다 — 지금 단계 「구현」, 승인 「대기」, 「구현 · 릴리스 · 효과 · 처분 · 독립 리뷰 · 종료 — 남음」, 릴리스 수동 런 「push 대기」. 승인은 `approval.md` 에 있고 릴리스·효과·처분은 섰다. 새 컨텍스트가 이 파일을 요약으로 받는다 | 회차기록 | 참 | 거짓신호 | 어느 조건에도 안 걸림 | `.palimpsest/rounds/2026-09-14-first-release/state.md:8` · `:21` · `:34` · `:35` | `grep -n` 산출 `34:| 승인 | 대기 |` · `35:| 구현 · 릴리스 · 효과 · 처분 · 독립 리뷰 · 종료 | 남음 |` ↔ `approval.md` 「전부 승인」 · `gh release view v0.1.0` draft=false |
| 6 | `A2-b` · `A3-a` ⑵ · `A2-a` ⑵ 오라클이 **병합·릴리스 코드가 아닌 병합 전 워크트리 바이너리**(`pal 0.0.0+fe70606417be` · 「커밋 전 빌드」)로 쟀고, 게이트 근거 표는 그 사실을 안 적는다. 병합 뒤 `06f6b96` 등이 더해졌다 | 회차기록 | 참 | 미관 | A2-b · A3-a · A2-a | `.palimpsest/rounds/2026-09-14-first-release/oracle/A2b-verdicts.txt:13` · `.palimpsest/rounds/2026-09-14-first-release/oracle/A3-repo-diff.txt:12` · `.palimpsest/rounds/2026-09-14-first-release/oracle/A2a-ts-digest.txt:4` | 오라클 머리 줄 판독. HEAD 빌드로 셋 다 다시 재 같은 결론(합격선 축) — 그래서 미관 |

## 내가 기각한 것

| # | 기각한 것 | 모집단 | 유효 | 해악도 | 좌표(파일:줄) | 왜 아니었나 |
|---|---|---|---|---|---|---|
| 1 | `A2-a` ⑵ 가 ditto **원본**에서 `pal symbols` 를 돌렸고 `origin-before` 는 그 뒤(14:27Z)에 찍혀, 원본에 캐시를 쓴 흔적을 스냅샷이 흡수했다(데이터_손실) | 원의도 | 거짓 | 금지역 | `.palimpsest/rounds/2026-09-14-first-release/oracle/A2a-ts-digest.txt:12` | 원본 `.palimpsest` 의 최신 mtime 이 2026-09-14T08:47:51(KST) — 회차 개시 `88f15fb` 22:02 보다 앞 · 그날 바뀐 **파일** 0 · HEAD `aded7ce` · porcelain 278 · 항목 7612 가 before/after 와 같다 |
| 2 | `A2-b` 의 이동 0 이 병합 전 바이너리에서만 서고 병합·릴리스 코드에서는 다를 수 있다 | 원의도 | 거짓 | 금지역 | `.palimpsest/rounds/2026-09-14-first-release/oracle/A2b-verdicts.txt:72` | HEAD `0.1.0+75e9ca4` 로 재현 — 42 = 42 · 분포 같음 · 이동 0 |
| 3 | `A3-a` ⑵ 가 병합 코드에서 둘보다 많은 참인 엣지를 버린다 | 원의도 | 거짓 | 금지역 | `.palimpsest/rounds/2026-09-14-first-release/oracle/A3-repo-diff.txt:36` | HEAD 로 재현 — 사라진 것 = 고정 목록 둘 · 새로 생긴 것 0 · 노드 3475 = 3475 |
| 4 | 게이트의 「파일 간 엣지 1338 → 1336」이 틀렸다(내 첫 셈은 1314 → 1312) | 회차기록 | 거짓 | 거짓신호 | `docs/gates/first-release.md:64` | 내 셈이 (경로·이름·종류) 튜플의 **서로 다른 것**을 셌다. 엣지 다중집합으로 세면 1338 로 게이트와 같다 |
| 5 | #127 ② 를 근거 없이 「재현 안 됨」으로 닫았다 | 원의도 | 거짓 | 거짓신호 | `.palimpsest/rounds/2026-09-14-first-release/baseline/06-doctor-full.txt:1` | 닫는 코멘트가 `baseline/06-doctor-full.txt`(`검사 9302 · 위반 0`)와 ADR-0007 을 싣는다(`gh issue view 127 --comments`) |
| 6 | 태그 `v0.1.0` 이 CI 가 빨간 커밋에 찍혔다(`e80ca14` attempt 1 failure) — 원문 「CI 가 초록이면 태그」 위반 | 원의도 | 거짓 | 거짓신호 | `.palimpsest/rounds/2026-09-14-first-release/intent.md:24` | attempt 2 success 가 15:16:04Z 에 끝났고 태그 객체 날짜는 15:16:45Z — 초록 뒤에 태그했다 |
| 7 | `C2` ⑴ 태그 없는 런이 태그 뒤였다 | 원의도 | 거짓 | 실패 | `.palimpsest/rounds/2026-09-14-first-release/oracle/C2-release.txt:6` | dispatch `34859015942` 14:57:43Z success → 태그 런 15:16:48Z |
| 8 | 원장 판 2 → 3 이관이 옛 결박의 내용을 잃었다(데이터_손실) | 저장소 | 거짓 | 금지역 | `.palimpsest/intent/bindings.jsonl:2` | 옛 id 42 가 전부 남고 `decor` 칸을 뺀 차이 0 — 규칙 위반은 의도 축 3 으로 남겼다 |
| 9 | CI 가 check 에서 멈춰 `doctor --full` 구조 판정을 안 돌렸으니 HEAD 에서 그것도 빨갛다 | 저장소 | 거짓 | 실패 | `scripts/check-round-doctor.mjs:1` | `palimpsest` 이름 클론에서 `doctor rc=0` · `check-round-doctor rc=0` · `MERGE_BLOCKER_DOCTOR_OK` |
| 10 | E1 오라클은 14:58Z 스냅샷이라 뒤에 이슈 상태가 바뀌었을 수 있다 | 원의도 | 거짓 | 거짓신호 | `.palimpsest/rounds/2026-09-14-first-release/oracle/E1-issues.txt:68` | 실시간 `gh issue list --state all` 대조 — R1 5 CLOSED/COMPLETED · R2 20 OPEN · R3 39 CLOSED/NOT_PLANNED |

## 끝내도 되는가

안 된다 — 본 목록에 **금지역 1**(`README.md:9` 가 릴리스에 실린 채 Kotlin 을 「결박 불가」로 적는다)과 **실패 1**(`75e9ca4` 에서 `cargo xtask check` 빨강 · `main` CI 세 OS 실패 → `F1` 반증)이 남았다.
