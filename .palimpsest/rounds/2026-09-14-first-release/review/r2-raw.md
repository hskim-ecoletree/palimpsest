# 독립 리뷰 R2 — 원 반환문

> 회차 `2026-09-14-first-release` · 대상 `main` HEAD `3eee67c` · 2026-09-15 · 상한 2 중 2(마지막)
> 실행은 전부 스크래치의 `git clone --no-hardlinks` 사본에서 했다. 메인 체크아웃에서 쓴 것은 이 파일 하나다.
> ditto 원본은 `git clone` 의 원천으로만 읽었고 `git --no-optional-locks status` 로 셌다 — 리뷰 전후 HEAD `aded7ce` · porcelain 278 줄 · `.palimpsest` 항목 7612 가 같다.
> 쓴 바이너리 — 새 쪽은 **릴리스 `v0.1.1` 에서 받은 자산**(`pal 0.1.1+b56ef097f158` · `shasum -c` 넷 OK), 착수 쪽은 `ec92b89` 클론의 릴리스 빌드(`pal 0.0.0+ec92b899478d`).
> 모집단 확인 — `git log --diff-filter=A ec92b89..HEAD --name-only` 가 더한 파일은 회차 기록(`.palimpsest/rounds/…` · `docs/gates/first-release.md` · `docs/instructions/2026-09-14-owner-direction.md`) · 시험 다섯 · `README.md` · 라이선스 둘이다. 회차 장치(검사·스크립트)는 더해지지 않았다.

## 합격선 축

| 조건 | 판정 | 잰 수 | 근거 |
|---|---|---|---|
| A1 | 통과 | 시험 1 · 문자열 2 종 × 파일 2 | HEAD 클론 `cargo xtask test` 로그 `전_그래프_명령의_범위가_graph_dump_와_같은_값이다 ... ok` · `grep -c 'unresolved: 0,\|lowest_grade: ExtractGrade::L0,'` → `doctor.rs:0` · `export.rs:0` |
| A1-a | 통과 | 시험 2 · diff 1 | 같은 시험 ok · `범위는_질의마다_다른_값이다 ... ok` · `git diff --stat ec92b89..HEAD -- crates/pal-cli/tests/query_envelope.rs` 0 줄 |
| A2 | 통과 | 갈래 5 | `a2_1_derive_인자가_바뀌면_stale` · `a2_2_cfg_…` · `a2_3_serde_rename_…` · `a2_4_must_use_…` · `a2_5_수신자_참조_self_가_값_self_로_바뀌면_stale` 각각 ok. RED(착수 코드에서 다섯 `fresh`)는 `oracle/A2-red.txt` 판독 — 이번에 안 돌렸다 |
| A2-a | 통과 | ⑴ 시험 1 · ⑵ 판독 | ⑴ `a2a_1_속성_안의_주석과_공백만_바꾸면_fresh ... ok` ⑵ 이번에 안 돌렸다 — 미측정 목록 2. R1 이 HEAD `75e9ca4` 빌드로 재현했고 `git diff --stat 75e9ca4..HEAD -- crates xtask .github scripts Cargo.toml` 은 `Cargo.toml` 버전 한 줄뿐이다 |
| A2-b | 통과 | 결박 42 = 42 | **v0.1.1 릴리스 자산 ↔ 착수 빌드로 다시 쟀다** — 둘 다 `ec92b89` 클론 · 이름 `palimpsest` · porcelain 0 · `intent import` → `결박 42`. `query binding.status --json` 을 결박 ID 로 대 id 집합 같음 · 분포 `fresh 28 · stale 14` 양쪽 같음 · 판정 이동 0. ⑵ 모집단 20 은 오라클 판독(미측정 목록 1) |
| A3 | 통과 | 시험 1 | `a3_크레이트_경계를_넘는_use_가_사적_mod_선언에_엣지를_안_잇는다 ... ok`. RED 는 `oracle/A3-red.txt` 판독 |
| A3-a | 통과 | 노드 3475 · 엣지 6194 → 6192 | ⑴ `a3a_크레이트_안에서_traverse_모듈을_부르는_엣지는_남는다 ... ok` ⑵ **v0.1.1 자산 ↔ 착수 빌드** `query graph.dump --json` on `ec92b89`: 노드 3475 = 3475 · 파일 간 엣지 1338 → 1336 · 사라진 것 = `pal-query/src/lib.rs run → pal-core/src/lib.rs traverse(module)` · `pal-query/tests/bench.rs 한_규모 → 같은 선언` 둘 · 새로 생긴 것 0 |
| B1 | 통과 | 시험 1 | `b1_install_이_식별자를_선언하고_다른_이름의_클론이_같은_수를_보인다 ... ok` |
| B1-a | 통과 | 갈래 4 | `b1a_1_…` ~ `b1a_4_커밋_전이면_uninstall_이_되돌리고_커밋_뒤면_매니페스트가_남는다` 각각 ok |
| B2 | 통과 | 시험 2 | `b2_호출자가_상한보다_많으면_자리를_싣고_나머지를_그대로_펴는_명령을_안내한다 ... ok` · `b2_화면이_안내한_명령을_그대로_돌리면_통한다 ... ok` |
| C1 | 통과 | 파일 3 · 자산 4 · README 명령 줄 12 | 루트 README · 라이선스 둘 있음. `gh release download v0.1.1` → `shasum -a 256 -c SHA256SUMS` 넷 OK · 네 아카이브 전부 `README.md`·`LICENSE-MIT`·`LICENSE-APACHE` · 아카이브 README ↔ HEAD README `diff` rc 0. `git diff e80ca14 b56ef09 -- README.md` 가 산문 두 곳뿐이라 펜스 명령 줄 12 는 `oracle/C1-readme-vs-d1.txt` 때와 같다. 아래 D1 재실행에서 그 줄을 그대로 쳤다 |
| C2 | 통과 | 런 2 · 잡 16 · 자산 5 | `gh run list`: ⑴ `34859015942` 릴리스 `workflow_dispatch` `e80ca14` success ⑵ `34861114858` 태그 `v0.1.0` success(세운다 넷 · 툴체인 없이 셋 · 산출 — `oracle/C2-release.txt` 잡 목록과 같음) ⑶ `gh release view v0.1.0` 자산 넷 + `SHA256SUMS`. 교정 `v0.1.1` 런 `34937509696` 도 잡 8 전부 success · 받은 바이너리 `pal 0.1.1+b56ef097f158` |
| D1 | 통과 | 걸음 13 | **v0.1.1 자산으로 새 ditto 클론(`aded7ce` · `.palimpsest` 없음)에서 README 걸음을 다시 밟았다** — install · doctor --install · narrative · touch rc 0 → `(0)` + 승인 줄 · 승인 → `■ 이 좌표에 걸린 것 (1)` · `호출자 2 · 피호출자 1` · `호출자 자리 — 앞 2곳` · `mkdir -p` · export 2 줄 · README `git add` rc 0 · commit rc 0 · `mate-renamed` 로 클론 → import `결박 1` → `(1)` |
| D1-a | 통과 | ⑴ 걸음 13 · ⑵ 값 3 | ⑴ **착수 빌드로 다시 밟았다** — install 뒤 `.palimpsest` 없음 · 승인 뒤 `(1)` · README `git add` rc 128(`pathspec '.palimpsest/manifest.toml'`) → 개정 1 대로 있는 파일만 · 다른 이름 클론 import `결박 1` → **`(0)`** ⑵ 원본 HEAD · porcelain 줄 수 · `.palimpsest` 항목 수가 `effect/origin-after.txt` 와 같다(해시 두 칸은 미측정 목록 3) |
| E1 | 통과 | 이슈 64 | `gh issue list --state all` 을 `triage.md` 분류별 목록에 댐: R1 5 `CLOSED/COMPLETED` · R2 20 `OPEN` · R3 39 `CLOSED/NOT_PLANNED`. 코멘트 내용은 R1 이 실시간 대조했고 이번엔 상태만 셌다 |
| E2 | 통과 | 절 3 · 산출 줄 1 · 문서 줄 2 | `gh issue view 160` 본문에 「대상」 ditto · 세 칸(하려던 것 · 화면이 말한 것 · 달라진 것 — 없음·틀림·시끄러움) · 「판정 문장」 · 메인 체크아웃 `scripts/frontier.sh` 첫 항 `#160 … ← 순서표의 1 번` · `docs/plan/03-shortest-path.md:207`(끝의 정의) · `:212`(Kotlin · Java · JavaScript · Python) |
| F1 | 통과 | check 29 · test 1184 · CI 런 2 | HEAD `3eee67c` 클론 `cargo xtask check` → `검사 29/29 통과` rc 0 · `FAIL`·`error[E`·`panicked` 0 · 「최근 끝난 회차 `2026-09-14-first-release` 가 검사에 들었다」. `cargo xtask test` rc 0 · `test result` 합 통과 1184 · 실패 0 · 무시 3 · 「시험 통과」. `palimpsest` 클론 `doctor --full --json \| node scripts/check-round-doctor.mjs` → `MERGE_BLOCKER_DOCTOR_OK` rc 0. 코드를 바꾼 마지막 커밋 `ed4516a`(Cargo.*) · push `b56ef09` 런 `34936295023` success — ubuntu · macos · windows 각각 `cargo xtask check`·`cargo xtask test`·`pal doctor full 구조 판정` success · 놓는다 둘 · 받는다 둘. HEAD 런 `34937650914` 도 success |

**검산** — 통과 17 · 반증 0 · 대조불가 0 · 미측정 0 = 17. **게이트와 갈린 조건 0.**
음성 대조 — `A2-b` · `A3-a` ⑵ · `D1-a` ⑴ 을 이 리뷰가 착수 빌드와 **릴리스 자산**으로 다시 돌렸고 걸렸다(`D1-a` ⑴ 이 실제로 `(0)`, 새 바이너리가 같은 걸음에서 `(1)`).
RED — `D1-a` ⑴ 의 `(0)` 은 재실행 관측, `A2`·`A3` RED 는 기록 판독.

## 미측정 목록

| # | 안 잰 조건 | 모집단 | 유효 | 해악도 | 좌표(파일:줄) | 왜 못 쟀나 |
|---|---|---|---|---|---|---|
| 1 | `A2-b` ⑵ — 착수에서 `fresh` 이고 속성·`self` 수신자를 진 Rust 심볼을 감시하는 결박 **20 이상**. 42 = 42 · 이동 0 은 다시 쟀으나 모집단 20 은 오라클 판독이다 | 원의도 | 참 | 미관 | `.palimpsest/rounds/2026-09-14-first-release/oracle/A2b-verdicts.txt:25` | 오라클이 「원문 텍스트로 셌다」고 적은 스크립트가 파일로 안 남아 같은 규칙으로 다시 셀 절차가 없다(보고 「능력 부재」 1 과 같은 자리) |
| 2 | `A2-a` ⑵ — TS 픽스처·ditto 심볼 요약값 착수 ↔ 새 바이트 대조 | 원의도 | 참 | 미관 | `.palimpsest/rounds/2026-09-14-first-release/oracle/A2a-ts-digest.txt:16` | 이번 라운드에 안 돌렸다. R1 이 소스가 같은 `75e9ca4` 빌드로 재현했고 그 뒤 `crates/` 소스 변경 0 이라 우선순위를 뒤로 뒀다 |
| 3 | `D1-a` ⑵ 의 해시 두 칸(porcelain sha256 · `.palimpsest` 목록 sha256) | 원의도 | 참 | 미관 | `.palimpsest/rounds/2026-09-14-first-release/effect/origin-after.txt:3` | 해시를 뜬 명령(정렬·경로 표기)이 파일에 없어 같은 값을 재현할 절차가 없다 — HEAD · 줄 수 · 항목 수만 댔다 |

## 의도 축

### 빠진 것

없음 — 계획 1~8(#127 ① · #77 ①② · #139 · #159 · #156 · 받는 길 · 효과 · 처분)의 산출이 전부 있고, 개정 4 · 승격 1 의 원장 이관은 `.palimpsest/intent/bindings.jsonl` 47 줄(머리 + 42 + 4)과 결정 넷(`DecorBaseline` · `cross_file_edges_in` · `식별자를_정한다` · `whole_graph_coverage`)이 v0.1.1 바이너리로 들인 뒤 넷 다 `(1) fresh` 로 확인됐다.

### 요구되지 않은 것

없음 — 교정 릴리스 `v0.1.1` 은 원문에 없지만 R1 의 금지역 발견(아카이브에 실린 거짓 문장)을 닫는 수단이고, 원문 「CI 가 초록이면 릴리스 태그는 묻지 않고 push 한다」 안에 든다(아래 기각 1).

### 있는데 틀린 것

| # | 발견 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 근거(명령·출력) |
|---|---|---|---|---|---|---|---|
| 1 | R1 을 고친 README 의 Kotlin 줄이 **「심볼 좌표와 결정 결박까지입니다」** 라고만 적는데, Kotlin 추출기는 **파일 최상위 선언만** 심볼로 세운다 — 클래스 안 메서드 · `object` 안 함수 · 프로퍼티는 좌표가 없어 결박할 수 없다. Kotlin 저장소 사용자는 메서드에 결정을 걸 수 있다고 읽는다. 화면은 「찾지 못했습니다 · 없다는 뜻이 아닙니다」라고 정직하게 말하므로 도구가 거짓을 말하지는 않는다 — 그래서 금지역이 아니라 거짓신호로 두었다. 이 문장은 `v0.1.1` 네 아카이브에 실려 있다 | 원의도 | 참 | 거짓신호 | C1(조건이 문장의 참을 안 잰다) | `README.md:11` · `crates/pal-extract/src/kotlin.rs:1` | v0.1.1 자산 · 스크래치 `Main.kt`(`class Greeter { fun hi(): String {…} ; val name }` · `object Registry { fun register() {} }` · `fun top() {}`) → `pal symbols Main.kt` = `3 class Greeter · 10 object Registry · 14 fun top · 선언 3` — `hi` · `register` · `name` 없음 · `pal touch hi` → `` `hi` 을 이 스냅샷에서 찾지 못했습니다 `` · `kotlin.rs:1` `//! Kotlin 최상위 선언 추출.` |
| 2 | 사용 기록 이슈 #160 이 「대상」 절에서 **「바이너리: 릴리스 `v0.1.0` 의 자산」** 을 쓰라고 적는다. 그런데 `v0.1.0` 릴리스 노트는 이제 머리에 「교정 — `v0.1.1` 을 받으십시오 · 아카이브의 README 한 문장이 사실이 아닙니다」를 싣는다. 판정을 지는 기록이 교정 전 판(거짓 README 동봉)을 가리킨다. 바이너리 동작은 같아서(기각 2) 기록 값은 안 틀어진다 | 원의도 | 참 | 거짓신호 | E2(조건은 판을 안 잰다) | `gh issue view 160` 본문 `## 대상` 둘째 항 | `gh issue view 160 -R hskim-ecoletree/palimpsest` → `- 바이너리: 릴리스 \`v0.1.0\` 의 자산(빌드한 것이 아님).` · `gh release view v0.1.0` 머리 `⚠ **교정 — \`v0.1.1\` 을 받으십시오.**` · `gh release list` → `v0.1.1 Latest` |

## 이번 라운드의 새 발견

| # | 발견 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 근거(명령·출력) |
|---|---|---|---|---|---|---|---|
| 3 | README 가 가리키는 `releases/latest`(= `v0.1.1`) 의 노트가 **`Full Changelog` 링크 한 줄뿐**이다 — 무엇이 고쳐졌는지 · 바이너리가 `v0.1.0` 과 같다는 것 · 교정판이라는 것을 안 싣는다. 그 설명은 `v0.1.0` 노트에만 있다. 아무것도 안 깨지고 사람이 잘못 믿게 되지도 않는다 | 원의도 | 참 | 미관 | 어느 조건에도 안 걸림(C2 는 `v0.1.0` 을 잰다) | `README.md:18` | `gh release view v0.1.1 --json body --jq .body` → `**Full Changelog**: https://github.com/hskim-ecoletree/palimpsest/compare/v0.1.0...v0.1.1` 한 줄 |

## 자기 산출에 대한 발견

| # | 발견 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 근거(명령·출력) |
|---|---|---|---|---|---|---|---|
| 4 | 게이트·종료 보고·교대 상태가 **교정 릴리스 `v0.1.1` 이 이미 나간 것을 안 적는다.** 게이트 머리는 릴리스를 `v0.1.0` 하나로 적고 `:87` 은 「`v0.1.1` 로 다시 릴리스한다」(미래형 — 실제 공개 2026-09-15T06:44:10Z), 보고의 「첫 릴리스가 섰나」 · 「밖으로 나간 것」은 `v0.1.0` · 이슈 닫기 44 · 열기 7 만 세고 **태그 `v0.1.1` push · 릴리스 `v0.1.1` · `v0.1.0` 노트 편집**을 빠뜨린다, 「다음 회차가 받는 것」은 `v0.1.0` 을 쓰라고 적는다. 교대 상태 `:14` 도 「낸다」. 보고의 ⟨R2 뒤 채운다⟩ 자리와 함께 마감 때 고칠 자리다 | 회차기록 | 참 | 거짓신호 | 어느 조건에도 안 걸림 | `docs/gates/first-release.md:4` · `docs/gates/first-release.md:87` · `.palimpsest/rounds/2026-09-14-first-release/report.md:37` · `.palimpsest/rounds/2026-09-14-first-release/report.md:65` · `.palimpsest/rounds/2026-09-14-first-release/report.md:88` · `.palimpsest/rounds/2026-09-14-first-release/state.md:14` | `grep -n 'v0\.1\.[01]'` 산출 — 게이트 `:4` `**릴리스** [\`v0.1.0\`]` · `:87` `… **\`v0.1.1\`** 로 다시 릴리스한다.` · 보고 `:65` `공개 릴리스 \`v0.1.0\`(태그 \`e80ca14\`) · 이슈 닫기 44 · 이슈 열기 7` · `:88` `\`v0.1.0\` 을 쓰고` ↔ `gh release view v0.1.1` `published: 2026-09-15T06:44:10Z` |

## 내가 기각한 것

| # | 기각한 것 | 모집단 | 유효 | 해악도 | 좌표(파일:줄) | 왜 아니었나 |
|---|---|---|---|---|---|---|
| 1 | 태그 `v0.1.1` 이 CI 초록 전에 push 됐다 — 원문 「CI 가 초록이면 태그」 위반 | 원의도 | 거짓 | 거짓신호 | `.palimpsest/rounds/2026-09-14-first-release/intent.md:24` | `b56ef09` CI 런 `34936295023` success `updatedAt 2026-09-15T06:32:20Z` · 태그 객체 tagger 시각 `1789454035` = 06:33:55Z — 초록 뒤다 |
| 2 | `v0.1.0` 노트의 「바이너리의 동작은 `v0.1.1` 과 같고」가 사실이 아니다 | 원의도 | 거짓 | 금지역 | `gh release view v0.1.0` 본문 머리 인용 셋째 항 | `git diff --stat e80ca14..b56ef09 -- crates xtask .github Cargo.toml Cargo.lock scripts` → 시험 셋 · `scripts/interop-receive.sh` · `Cargo.*` 버전뿐, `crates/*/src` 변경 0 |
| 3 | R1 을 고친 README 의 「Kotlin — … 결정 결박까지」가 확인 없이 쓴 과대 진술이다(결박이 실제로 안 선다) | 원의도 | 거짓 | 금지역 | `README.md:11` | v0.1.1 자산 · 스크래치 Kotlin 저장소에서 README 걸음 — `pal touch Greeter` 승인 뒤 `■ 이 좌표에 걸린 것 (1) … 최신 상태(fresh)` · export → commit → 다른 이름 클론 import `결박 1` → `(1)`. 범위(최상위만)는 의도 축 1 로 남겼다 |
| 4 | README 의 「그 밖의 언어(Java · JavaScript · Python 등) — 「텍스트만 · 결박 불가」」가 틀렸다 — JavaScript 는 TS 추출기가 읽을 수 있다 | 원의도 | 거짓 | 금지역 | `README.md:12` | v0.1.1 `pal ledger` → `Java L0 unavailable … 텍스트만 · 결박 불가` · `JavaScript L0 unavailable … 텍스트만 · 결박 불가` · `Python L0 unavailable … 텍스트만 · 결박 불가` · `pal symbols src/a.js` → `이 빌드에 JavaScript 추출 능력이 없습니다` |
| 5 | `v0.1.1` 아카이브가 고치기 전 README 를 싣고 나갔다 | 원의도 | 거짓 | 금지역 | `.github/workflows/release.yml:108` | 네 아카이브 전부 `README.md` 동봉 · `shasum -c` OK · 받은 README ↔ HEAD README `diff` rc 0 |
| 6 | README 「`pal install` 은 그 프로젝트 안에만 씁니다」가 틀렸다 — `.claude/agents/`·`.claude/settings.json`·`.claude/pal/manifest.json` 을 안 적었고 홈 디렉터리에도 쓸 수 있다 | 원의도 | 거짓 | 거짓신호 | `README.md:45` | 그 파일들은 README 가 적은 「`.claude/` 아래」에 든다 · install 산출의 나머지는 `CLAUDE.md`·`.gitignore` 블록과 `.palimpsest/manifest.toml` 로 README 와 같다 · `find ~/.cache ~/.config ~/Library/Caches ~/Library/Application\ Support -maxdepth 2 -newer <픽스처> -iname '*pal*'` 0 건 |
| 7 | D1 은 `v0.1.0` 에서 쟀고 README 가 그 뒤 바뀌어 지금 나가는 판의 걸음은 확인되지 않았다 | 원의도 | 거짓 | 거짓신호 | `.palimpsest/rounds/2026-09-14-first-release/effect/d1-release/commands.txt:1` | README diff 는 산문 두 곳뿐(명령 줄 무변경) · v0.1.1 자산으로 새 ditto 클론에서 다시 밟아 `(1)` · 호출자 자리 2 · 다른 이름 클론 `(1)` |
| 8 | 판정을 다시 전사한 커밋 `3eee67c` 가 R1 때(`75e9ca4`)처럼 check 를 다시 빨갛게 했다 | 원의도 | 거짓 | 실패 | `docs/gates/first-release.md:87` | HEAD 클론 `cargo xtask check` 29/29 rc 0 · 「어색한 표현 부재 … 남은 것 0곳」 · CI 런 `34937650914`(`3eee67c`) success |
| 9 | 결박 커밋 메시지의 「넷 다 touch `(1)`」이 판 3 이관 뒤 지금 HEAD 에서는 서지 않는다 | 저장소 | 거짓 | 거짓신호 | `.palimpsest/intent/bindings.jsonl:1` | HEAD 클론에 v0.1.1 로 import(`결박 46`) → `DecorBaseline` · `cross_file_edges_in` · `식별자를_정한다` · `whole_graph_coverage` 넷 다 `■ 이 좌표에 걸린 것 (1)` · `최신 상태(fresh)` |
| 10 | 이 회차나 이 리뷰가 ditto 원본을 건드렸다(데이터_손실) | 원의도 | 거짓 | 금지역 | `.palimpsest/rounds/2026-09-14-first-release/effect/origin-after.txt:2` | 리뷰 전후 `git -C ~/dev/projects/ditto rev-parse HEAD` = `aded7ce…` · `--no-optional-locks status --porcelain` 278 줄 · `.palimpsest` 항목 7612 — 오라클 뒤 스냅샷과 같다 |
| 11 | 최단 경로의 4 단계 줄 「남이 받아 쓸 수 있는 `v0.1.0`」이 교정 뒤 낡았다 | 저장소 | 거짓 | 미관 | `docs/plan/03-shortest-path.md:204` | 단계의 정의(무엇을 세우는가)이지 「어느 판을 받으라」는 안내가 아니다 — `v0.1.0` 이 실제로 그 단계의 릴리스로 섰다 |

## 끝내도 되는가

된다 — 본 목록(의도 축 · 새 발견)에 금지역 0 · 실패 0 이 남았다(거짓신호 2 · 미관 1). 합격선은 17 전부 통과로 게이트와 안 갈렸고, 자기 산출 절의 거짓신호 1(릴리스 `v0.1.1` 미기재)은 마감 전사 때 처리할 자리다.
