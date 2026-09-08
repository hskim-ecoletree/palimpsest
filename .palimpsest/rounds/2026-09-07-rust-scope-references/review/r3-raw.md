# 독립 리뷰 R3 — 원 반환문

> 회차 `2026-09-07-rust-scope-references` · HEAD `ecd9dba` · 상한 3 중 **마지막 라운드**
> 이 파일은 에이전트가 낸 글을 한 글자도 안 고치고 옮긴 것이다.

## 내가 받은 것과 안 받은 것

**받은 것** — 잠긴 의도(`intent.md` 원문 · 개정 · 승격) · 게이트
(`docs/gates/rust-scope-references.md`) · `state.md` · `observations/` 여덟 · `dialectic/`
열셋 · `findings.jsonl` · `a774053..HEAD` 의 전 차분 · 저장소 전체와 실행 권한.

**안 받은 것** — 대화 기록 · 메인의 사고 과정 · 앞 라운드(R1·R2)의 결과.
`review/r1-raw.md` 와 `review/r2-raw.md` 는 **열지 않았다.**

⚠ **오염을 신고한다.** `grep -rn "스코프를 안 만든다"` 와 `grep -rn "아직 없다"` 를 저장소
전체에 돌렸을 때 두 파일의 줄 몇 개가 결과에 딸려 왔다(D4 검색어가 그 안에 인용돼 있다).
그 줄을 근거로 쓰지 않았고, `findings.jsonl` 은 회차 기록이라 정상적으로 읽었다.
그래서 아래 항 중 일부는 원장에 이미 열린 항과 겹칠 수 있다.

**환경** — 이 셸의 `grep`·`rg` 는 함수 래퍼다(`type grep` → shell function). 아래 수는
전부 어느 계수기로 잰 것인지 함께 적는다.

## 합격선 축

⚠ `E1`·`E2`·`E3`·`V9` 넷은 **정반합이 진다.** 근거만 댄다.

| 조건 | 판정 | 잰 수 | 근거 |
|---|---|---|---|
| A1 | 통과 | 시험 1 · `cargo test -p pal-extract` 153 통과 0 실패 | `rust.rs:736` `a1_스코프_사슬이_성립한다` — `scopes.len() >= 3` 과 `!refs.is_empty()` 를 둘 다 단언 |
| A2 | 통과 | 시험 1 | `rust.rs:743` — `엣지(src)` 가 빈 집합이고 `new` 를 담은 스코프가 정확히 둘이며 서로 다르다 |
| A3 | 통과 | 시험 1 | `rust.rs:760` — `[("b","a")]` 정확히 하나 |
| A4 | 통과 | 시험 1 | `rust.rs:765` — `assert_eq!(a(), 0)` 에서 `b → a` |
| A5 | 통과 | 시험 1 | `rust.rs:773` — `assert!(x.foo())` 의 `foo` 가 `refs` 에 없다 |
| A6 | 통과 | 시험 2 | `rust.rs:779` · `:944`(매크로 안 구조체 리터럴 필드) — 머리 `S` 는 남는 것까지 단언 |
| A7 | 통과 | 시험 1 | `rust.rs:786` — `Self::new()` 에서 `new` 의 실린 자리가 **1**(선언 자리)뿐 |
| A8 | 통과 | 시험 1 | `rust.rs:796` |
| A9 | 통과 | 시험 1 | `rust.rs:804` |
| A10 | 통과 | 시험 1 | `rust.rs:809` — `struct S; fn f(){ let _ = S; }` 에서 `f → S` |
| A11 | 통과 | 시험 1 · 갈래 4+1 · 이름 축 4 | `rust.rs:815` — `imports` 가 `Capable::Present` 이고 `modules` 와 스코프 `bindings` 두 축을 따로 단언 |
| A12 | 통과 | 시험 1 | `rust.rs:846` — 소스 순서를 뒤집어도 `names`·`export_digest` 가 같다 |
| A13 | 통과 | 시험 1 · 갈래 6 | `rust.rs:860` — 최상위 `pub` 만. 근거 인용은 있으나 좌표가 틀렸다(항 6) |
| A14 | 통과 | 시험 2 · 자리 5 | `rust.rs:881`(파라미터·`let`·`match_arm`·`for`·`if let` 다섯) · `:896`(자기 초기화식) |
| A15 | 통과 | 시험 1 | `rust.rs:908` |
| A16 | 통과 | 시험 1 | `rust.rs:915` — 모듈 스코프에 `m` 이 없고 `ScopeKind::Impl` 스코프에 있다 |
| A17 | 통과 | 시험 1 | `rust.rs:931` — 엣지 0 이고 `RefResolution::Ambiguous` 가 실제로 선다 |
| V1 | 통과 | 엣지 4306 · 가짜 51 · 누락 0 | 내가 `cargo run -q --release -p pal-extract --example scope_variants -- .` 를 다시 돌려 `v-variants.txt` 와 **완전 동일** 산출을 봤다 |
| V2 | 통과 | 4344 · 89 · 0 | 같은 실행 |
| V3 | 통과 | 4228 · 5 · 32 | 같은 실행 |
| V4 | 통과 | 4243 · 5 · 17 · `V4≠V3` 다르다 | 같은 실행 |
| V5 | 통과 | 4268 · 13 · 0 | 같은 실행 |
| V6 | 통과 | 4289 · 34 · 0 | 같은 실행 |
| V7 | 통과 | 4342 · 87 · 0 | 같은 실행 |
| V8 | 통과 | 4508 · 253 · 0 | 같은 실행 |
| V9 | 정반합이 진다 | 엣지 4255 · 가짜 0 · 누락 0 → **엣지 축에서 「같다」** | 등록된 축(V1 문면의 「엣지 집합이 기준과 다르다」)에서 안 갈린다. 장치가 마지막 줄에 *"이 줄을 통과로 읽지 마라"* 를 스스로 찍는다 |
| V10 | 통과 | 4244 · 31 · 42 | 같은 실행 |
| V11 | 통과 | 엣지 쌍 4255 · 참조 6639 · 타입 1198 · 호출·매크로 3843 · 그 밖 1598 | 게이트 `:94-106` 에 표가 있고 내 재실행이 같은 수를 냈다 |
| V12 | 통과 | 명령 1 · 바이너리 1 · 소스 1 | 게이트 `:83-87`. 그 명령을 그대로 돌려 재현했다 |
| B1 | 통과 | REFERENCES **4258** · 기여한 서로 다른 `.rs` **132** ≥ 80 | `./target/release/pal export --format cypher` 를 직접 돌려 `Symbol` 노드에서 `path` 를 이어 세었다(파이썬 조인). 기여 소스 파일 133 중 `.rs` 가 132 |
| B2 | 통과 | 호출자 **6** · 피호출자 **3** | `./target/release/pal touch file_edges` 직접 실행 |
| B3 | 통과 | 시험 1 · 자리 3 | `rust.rs:972` — `scopes`·`imports`·`exports` 셋이 `Capable::Present` 이고 값이 비지 않았다 |
| C1 | 통과 | 시험 1 | `rust.rs:962` — `grade_of(Rust)==L1` · `g.grade==L1` · `identity==Ordinal` |
| C2 | 통과 | 명령 산출 결박 **30**(fresh 26 · stale 4) · 이 회차가 건 다섯(ULID `01M1Z0…`)을 빼면 **25 = fresh 23 · stale 2** | `./target/release/pal query binding.status` 를 돌려 상태를 세고(`fresh 26 · stale 4`) 결정 ULID 로 회차 몫 다섯을 갈랐다. 착수 기준값이 그대로다 |
| **C3** | **반증** | `cargo test --workspace` **987 통과 · 0 실패 · rc=0** · `cargo xtask check` **26/27**(죽은 링크 1 건 — `report.md`) | 게이트의 판정과 같다. 남은 빨강 하나가 아직 안 쓴 종료 보고로 가는 링크다 |
| C4 | 통과 | ditto **4578 줄 · 움직인 것 0** · portal-backend **1340 줄 · 움직인 것 0** · `git diff a774053..HEAD -- corpus/golden/` **빈 출력** | `./scripts/f03-3-verify.py` 를 `--bless` 없이 통째로 돌려 ③ 두 줄을 직접 봤다 |
| C5 | **미측정** | 이 회차 커밋 24 개 중 런 **0** | `gh run list` 의 최신 런 `headSha` 가 착수 커밋 `a7740537…` 이다. 아직 push 전이라 원리상 못 잰다 |
| C6 | 통과 | 시험 2 · `cargo test -p pal-cli --test rust_references` 11 통과 0 실패 | `rust_참조가_2층까지_도착하고_화면이_답한다` · `rust_의_내보내기가_최상위_pub_만_담는다` |
| C7 | 통과 | `ditto.symbols.tsv` 움직인 것 0 · 회차 전후 바이트 동일 | 위 C4 와 같은 실행 + `git diff` |
| C8 | 통과 | `f03-2` → `f02-rust-scope` | `crates/pal-extract/src/lib.rs:147`. `git log -S` 로 전 이력을 훑어 그 값이 전에 쓰인 적 없음을 확인 |
| C9 | 통과 | 자리 2 | `schema/graph.toml:415-427` 과 `crates/pal-core/src/graph.rs:167-176` 양쪽이 *"자격을 지는 것은 등급 글자가 아니라 스코프 체인"* 으로 고쳐졌다 |
| D1 | 통과 | 이슈 1(#133 OPEN) · 닫힘 조건 4 | `gh issue view 133` |
| D2 | **미측정** | 코드 주석 몫 **충족**(몫 3 + 넷째) · **종료 보고 몫은 잴 자리가 없다** | 조건 문면은 *"코드 주석**과** 종료 보고에 있다"* 인데 `report.md` 가 없다. 게이트는 `통과` 로 적는다(항 5) |
| D3 | 통과 | 자리 1 | `classify.rs:224-247` — 왜 `L1` 인지 · 언제 풀리는지(#133) |
| D4 | **반증** | 등록 검색어 4 중 「아직 없다」의 산출이 재현 안 됨 — 게이트 **19** vs 실측 **115 줄 / 78 파일**(`git grep`), **48 줄 / 30 파일**(`rg` 기본) | 항 11 |
| D5 | 통과 | 후보 2 · 선택 1 | `doctor.rs:303-316` — `.absent` 유지, 사유를 「작아서」에서 「이 뷰가 엣지를 안 읽는다」로 바꿨고 `.holding` 이 왜 죽은 가지인지를 적었다 |
| D6 | 통과 | 자리 3 | `rust.rs:1`(제목이 「스코프 체인」으로) · `classify.rs` · `pal-core/src/scope.rs` 의 `ScopeKind`·`ScopeChain` 계약 |
| D7 | 통과 | 절 1 | `docs/adr/0027-…:3` 상태 줄 · `:140` 「개정」 절 |
| D8 | 통과 | 줄 2 | `pal touch file_edges` 실행 화면에 *"「호출자·피호출자」는 참조 엣지입니다"* 와 *"`x.foo()` 와 `S::foo()` 는 아직 안 셉니다"* 가 실제로 찍힌다 |
| E1 | 정반합이 진다 | 표본 50 줄 · 재현 시 **49 줄 동일 · 1 줄 좌표만 +1** | `--표본` 을 다시 돌려 `e1-sample-50.txt` 와 `diff` 했다. 갈린 것은 92 번째 줄의 `scope.rs:556→557` 하나뿐이다 |
| E2 | 정반합이 진다 | — | 게이트 `:263-265`. `V9` 를 덮개에서 뺐다 |
| E3 | 정반합이 진다 | 동명 재선언 자리 기준 **21**(파일 10) · `impl` 을 안 열면 **83**(파일 30) | 내 재실행이 같은 수를 냈다. 소유자 답의 「72」와 다른 것은 그 뒤 커밋 때문이라고 게이트가 적는다 |

**계수** — 통과 **44** · 반증 **2** · 대조 불가 **0** · 미측정 **2** · 정반합이 진다 **4** = **52**

**음성 대조** — 등록된 다섯 줄(B1·B2 / C1·C2 / C6 / C7 / A12)이 게이트 표에서 여섯 행으로
전부 ✓ 다. **그러나 그 여섯은 `11b6162` 에서만 쟀다**(항 12).
**RED 가 실제로 빨갰던 것** — 착수 시점 `REFERENCES 3` · 기여 `.rs` 0 · `호출자 0 · 피호출자 0`
이 `baseline-red.txt` 에 원 출력으로 있다. 0 개를 훑고 통과하는 검사가 아니다.

## 미측정 목록

| # | 안 잰 조건 | 모집단 | 유효 | 해악도 | 좌표(파일:줄) | 왜 못 쟀나 |
|---|---|---|---|---|---|---|
| 1 | `C5` — 회차의 마지막 커밋에 `conclusion=success` 런이 붙는다 | 원의도 | 참 | 실패 | .palimpsest/rounds/2026-09-07-rust-scope-references/intent.md:117 | 아직 push 전이라 런이 없다. `gh run list --limit 5 --json headSha,conclusion` 의 최신 런 `headSha` 가 착수 커밋 `a7740537a927…` 이고 이 회차 커밋 24 개 중 어느 것에도 런이 안 붙었다. 원리상 종료 뒤에만 잴 수 있다 |
| 2 | `D2` — 못 세는 몫 셋의 수가 **종료 보고에** 있다 | 원의도 | 참 | 거짓신호 | .palimpsest/rounds/2026-09-07-rust-scope-references/intent.md:126 | `report.md` 가 없다(`ls .palimpsest/rounds/2026-09-07-rust-scope-references/` 에 `report.md` 없음 · `cargo xtask check` 의 유일한 빨강이 그 죽은 링크다). 코드 주석 몫만 잴 수 있었다 |

## 의도 축

### 빠진 것

| # | 발견 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 근거(명령·출력) |
|---|---|---|---|---|---|---|---|
| 3 | **종료 보고 `report.md` 가 없다.** 그래서 `D2` 의 절반과 규약이 요구하는 네 이름(의도적으로 안 한 것 · 확인 못 한 것 · 추론(확인 아님) · 다음으로 넘기는 것)을 대조할 자리가 없고, 그 부재가 지금 `cargo xtask check` 의 **유일한 빨강**이다 | 원의도 | 참 | 실패 | D2·C3 | docs/gates/rust-scope-references.md:5 · .palimpsest/rounds/2026-09-07-rust-scope-references/ | `cargo xtask check` → `죽은 링크 부재: 죽은 링크 1건: docs/gates/rust-scope-references.md → ../../.palimpsest/rounds/2026-09-07-rust-scope-references/report.md` · `Error: 1개 검사가 실패했다` |

### 요구되지 않은 것

| # | 발견 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 근거(명령·출력) |
|---|---|---|---|---|---|---|---|
| 4 | **`trait_item` 도 스코프를 연다 — 등록된 목록에 없다.** 잠긴 의도와 `state.md:65` 의 문법 표는 여는 것으로 `impl_item` 만 적고, 같은 파일의 모듈 표(`rust_scopes.rs:7`)도 `impl_item` 만 적는데 구현은 `"impl_item" \| "trait_item"` 둘을 연다. 동작 자체는 Rust 의미상 옳지만(트레잇 연관 항목도 모듈로 안 올라간다) **등록 밖 확대이고 자기 파일의 표와 어긋난다** | 원의도 | 참 | 거짓신호 | 없음 | crates/pal-extract/src/rust_scopes.rs:169 ↔ crates/pal-extract/src/rust_scopes.rs:7 | `grep -n 'trait_item\|impl_item' crates/pal-extract/src/rust_scopes.rs` → `:169  "impl_item" \| "trait_item" if self.impl_이_연다 => Some(ScopeKind::Impl),` · `:7` 의 표는 `**impl_item**` 만 적는다. `pal-core/src/scope.rs` 의 `ScopeKind::Impl` 문서는 갱신됐고 이 표만 안 따라왔다 |

### 있는데 틀린 것

| # | 발견 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 근거(명령·출력) |
|---|---|---|---|---|---|---|---|
| 5 | **게이트가 `D2` 를 「통과」로 적는데 조건의 절반이 아직 없다.** 조건 문면은 *"못 세는 몫 셋의 수가 **코드 주석과 종료 보고에** 있다"* 이고 종료 보고가 없다. 게이트 근거 칸도 코드 주석만 댄다 — **못 잰 것을 잰 것으로 적었다** | 원의도 | 참 | 금지역 | D2 | docs/gates/rust-scope-references.md:191 · .palimpsest/rounds/2026-09-07-rust-scope-references/intent.md:126 | `sed -n '191p' docs/gates/rust-scope-references.md` → `\| \`D2\` \| 통과 \| 못 세는 몫 셋과 분모가 … 모듈 주석의 표에 있다` · `ls` 에 `report.md` 없음 |
| 6 | **`A13` 의 근거 좌표가 이 회차 안에서 이미 틀렸다.** `rust.rs:411` 과 `rust.rs:862` 가 `n.container.is_empty()` 를 `ledger.rs:377` 로 인용하는데 지금 그 줄은 **383** 이다. 착수 커밋에서는 `:377` 이 맞았고, 같은 회차의 `11b6162` 가 그 위에 주석 여섯 줄을 얹으면서 밀었다 | 원의도 | 참 | 거짓신호 | A13 | crates/pal-extract/src/rust.rs:411 · crates/pal-extract/src/rust.rs:862 | `grep -n 'container.is_empty' crates/pal-cli/src/ledger.rs` → `378`(주석) · `383`(코드). `git show a774053:crates/pal-cli/src/ledger.rs \| sed -n '377p'` → 그 코드 줄. 지금은 아니다 |
| 7 | **`D2` 의 넷째 몫이 게이트 안에서 두 값으로 병존한다 — `:191` 이 「58」, `:207` 이 「47」.** 최종 실측은 **47** 이고(장치의 `모호 47` · `rust.rs:38` 의 *"실측 참조 47 건"*), 58 은 `be86499` 이전 값이다. 같은 낡은 58 이 **사용자 표면에도 살아 있다** — `pal query binding.status` 의 결정 `01M1Z0ZCGY…` 가 *"모호 참조가 200 → 58 로 줄었다"* 를 찍는다 | 원의도 | 참 | 거짓신호 | D2 | docs/gates/rust-scope-references.md:191 ↔ :207 · decision/01M1Z0ZCGY2HYWSZP7C7XCT536 | `sed -n '191p;207p' docs/gates/rust-scope-references.md` → `참조 58` / `\| ⟨넷째⟩ … \| 47 \|`. `cargo run -q --release -p pal-extract --example scope_variants -- .` → `기준 참조 갈래 — … 모호 47`. `./target/release/pal query binding.status` 가 58 을 찍는다 |

## 이번 라운드의 새 발견

| # | 발견 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 근거(명령·출력) |
|---|---|---|---|---|---|---|---|
| 8 | **죽은 링크 검사의 전역 면제가 「보상 통제가 있다」는 거짓 진술 위에 서 있다.** `에이전트_원_반환문인가` 가 모든 회차의 `premortem`·`conditions-audit`·`review`·`dialectic` 네 디렉터리를 **영구히** 링크 검사 모집단에서 뺀다. 그 자리의 문서 주석은 *"원인은 따로 막는다 — 에이전트 정의가 **이제** 「저장소 안 문서는 링크가 아니라 코드 표기로 적어라」를 요구한다"* 라고 단언하는데 **그런 요구가 `.claude/` 어디에도 없다.** `.claude/agents/` 는 이 회차에서 한 글자도 안 바뀌었고(마지막 커밋 `213d3ee`, 착수 이전) `.claude/` 의 이 회차 차분은 `extract.py`·`record.py` 둘뿐이다. #134 의 닫힘 조건 3(*"새 회차의 에이전트 반환문이 그 규약을 지키는지 재는 자리가 선다"*)도 미이행이고 #134 는 OPEN 이다. **면제만 남고 원인 차단은 없다 — 그 모집단에서 측정이 죽은 가지가 됐다** | 저장소 | 참 | 금지역 | 없음 | xtask/src/main.rs:2635-2660 · xtask/src/main.rs:2721 · .claude/agents/ | `git diff --stat a774053..HEAD -- .claude/` → `extract.py \| 110 +++`, `record.py \| 8 ++` (agents 없음) · `git log --oneline -3 -- .claude/agents/` → `213d3ee`(착수 이전) · `git grep -n '코드 표기'` → xtask 셋 · 게이트 둘 · 회차기록 셋뿐, `.claude/agents/*.md` 0 건 · `gh issue view 134 --json state` → `OPEN` |
| 9 | **`D4` 가 게이트에 적으라고 한 「찾은 자리 수」가 재현되지 않는다.** 게이트는 「아직 없다」를 **19(`target/` 제외)** 로 적고 *"열여덟은 다른 기능의 것 · 이 회차가 낡게 만든 자리는 하나"* 라는 결론을 그 수 위에 세운다. 같은 범위를 다시 재면 **115 줄 / 78 파일**(`git grep`, 추적 파일 전부)이고 숨김 제외 `rg` 기본으로도 **48 줄 / 30 파일**이다. 회차기록과 게이트를 빼도 **36 줄 / 22 파일**이다. **19 를 내는 명령을 못 찾았다.** ⚠ 완화: 내가 그 36 줄을 전부 열어 봤고 이 회차가 낡게 만든 자리를 더 못 찾았다 — **결론은 살아 있고 수와 모집단만 거짓이다.** 조건이 요구한 산출물이 그 수 자체다 | 원의도 | 참 | 금지역 | D4 | docs/gates/rust-scope-references.md:223 · .palimpsest/rounds/2026-09-07-rust-scope-references/intent.md:128 | `git grep -n '아직 없다' \| wc -l` → `115` · `git grep -l … \| wc -l` → `78` · `rg -n '아직 없다' . \| wc -l` → `48` · `git grep -n '아직 없다' -- ':!.palimpsest' ':!docs/gates' \| wc -l` → `36`. 「스코프를 안 만든다」는 게이트의 **1** 이 재현된다(`git grep -n` 12 줄 중 코드 1 · 게이트 2 · 회차기록 9) |
| 10 | **음성 대조 여섯이 최종 코드에서 발화하는지 안 쟀는데 게이트 표는 무조건 ✓ 다.** 증거 파일이 스스로 *"여섯은 `11b6162` 에서만 쟀고 최종 코드에서 다시 안 쟀다 … 「대조가 발화했다」를 「최종 코드에서도 발화한다」로 읽지 마라"* 라고 적는데 **게이트의 음성 대조 표에는 그 한정이 한 글자도 없다.** 그 사이에 로직이 실제로 바뀌었다 — `be86499` 가 `ScopeBinding::visible_from` 을 새로 넣어 `resolve` 의 해소 순서를 바꾸고 매크로 안 구조체 리터럴 필드 거르기를 더했다. 게이트만 읽는 다음 컨텍스트는 「최종 코드에서 음성 대조가 섰다」로 읽는다 | 원의도 | 참 | 거짓신호 | 없음(음성 대조 절) | docs/gates/rust-scope-references.md:28-39 ↔ .palimpsest/rounds/2026-09-07-rust-scope-references/observations/negative-controls.md:6-11 | `sed -n '28,39p' docs/gates/rust-scope-references.md`(한정 없음) 대 `sed -n '6,11p' …/negative-controls.md` · `git diff --stat 11b6162..HEAD -- crates/` → `scope.rs \| 49 ++`, `rust_scopes.rs \| 31 ++`, `scopes.rs \| 18 ++`(주석만이 아니라 로직) |

## 자기 산출에 대한 발견

<메인이 처리 방침을 정한다. 이 회차가 반드시 닫아야 하는 것이 아니다.>

| # | 발견 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 근거(명령·출력) |
|---|---|---|---|---|---|---|---|
| 11 | **낡은 좌표를 고치려고 단 주석이 그 사이 다시 밀렸다.** 게이트가 *"`RefResolution` 은 `:177` 로, `resolve_shadowing` 은 `:351` 로 밀렸다"* 라고 적는데 지금 `pal touch` 가 찍는 것은 **`:178`·`:352`** 다. `b2050d8` 이 `scope.rs` 에 줄 하나를 더하면서 다시 밀었다 | 회차기록 | 참 | 거짓신호 | 없음 | docs/gates/rust-scope-references.md:316-318 | `./target/release/pal touch RefResolution` → `enum · crates/pal-core/src/scope.rs:178` · `./target/release/pal touch resolve_shadowing` → `fun · crates/pal-core/src/scope.rs:352` · `grep -n 'pub enum RefResolution\|fn resolve_shadowing' crates/pal-core/src/scope.rs` → `178` · `352` |
| 12 | **게이트의 `C4` 절이 지금 거짓인 사실을 현재형으로 적는다.** *"`scripts/f03-3-verify.py` 를 통째로 돌리면 ③ 에 못 닿는다"* 인데, **같은 커밋(`4d3bb76`)이 그 조기 반환을 없앴다.** 내가 통째로 돌려 ③ 두 줄을 봤다 | 회차기록 | 참 | 거짓신호 | C4 | docs/gates/rust-scope-references.md:146-149 · scripts/f03-3-verify.py:141-155 | `./scripts/f03-3-verify.py` → `FAIL ④ …` 뒤에 `ok ③ ditto 4578 줄 · 움직인 것 0` · `ok ③ portal-backend 1340 줄 · 움직인 것 0` · `✗ ④ 가 실패했다 … ③ 은 위에 그대로 산출했다` |
| 13 | **잠긴 의도 안에서 `E1` 표본 크기가 두 값으로 산다 — 조건은 50, 차선책과 범위 밖은 30.** 개정 표가 *"`E1` 의 표본을 30 균일에서 **50 층화**로 바꿨다"* 라고 적었는데 그 정정이 `## 차선책` 과 `## 범위 밖` 에 안 따라갔다. 게이트는 50 으로 통일돼 있다 | 회차기록 | 참 | 거짓신호 | E1 | .palimpsest/rounds/2026-09-07-rust-scope-references/intent.md:136 ↔ :160 ↔ :189 (개정 :235) | `sed -n '136p;160p;189p;235p' intent.md` → `:136` *"엣지 표본 **50 건**"* · `:160` *"`E1` 의 표본 30 건에서 가짜가 나오면"* · `:189` *"표본 30 건(`E1`)까지만 잰다"* · `:235` *"30 균일에서 **50 층화**로 바꾸고"* |
| 14 | **변형 대조 장치의 판정 칸이 `V9` 에 여전히 `다르다 ✓` 를 찍는다.** 등록된 축은 엣지 집합인데 판정 식이 `엣지가_다른가 \|\| 갈래가_다른가` 다. 표 아래 두 줄이 그것을 정정하지만 **표만 옮겨 적으면 통과로 읽힌다** | 자기장치 | 참 | 거짓신호 | V9 | crates/pal-extract/examples/scope_variants.rs:453-457 | `sed -n '449,458p' crates/pal-extract/examples/scope_variants.rs` · 실행 산출 `V9 use 절 배제를 끈다 4255 0 0 같다 다르다 ✓` |
| 15 | **음성 대조 문서가 「여섯」이라 적고 라벨 일곱을 나열한다** — `B1`·`B2`·`C1`·`C2`·`C6`·`C7`·`A12`. 게이트 표가 `B1`·`B2` 를 한 행으로 묶어 여섯 행인 것과 라벨 수를 섞었다 | 회차기록 | 참 | 미관 | 없음 | .palimpsest/rounds/2026-09-07-rust-scope-references/observations/negative-controls.md:6 | `sed -n '6p' …/negative-controls.md` → `⚠ **여섯(\`B1\`·\`B2\`·\`C1\`·\`C2\`·\`C6\`·\`C7\`·\`A12\`)은 …` — 괄호 안이 일곱이다 |
| 16 | **`E1` 표본 원 출력의 좌표 한 줄이 낡았다** — 92 번째 줄이 `scope.rs:556` 인데 지금 같은 명령을 돌리면 `:557` 이다. 나머지 49 줄은 바이트 동일이라 표본 규칙의 결정론은 살아 있다 | 회차기록 | 참 | 미관 | E1 | .palimpsest/rounds/2026-09-07-rust-scope-references/observations/e1-sample-50.txt:92 | `cargo run -q --release -p pal-extract --example scope_variants -- . --표본 > /tmp/s.txt; diff /tmp/s.txt …/e1-sample-50.txt` → `92c92` 한 줄만 |
| 17 | **게이트의 `## 판정` 이 아직 자리표시자다** — *"⟨정반합과 남은 독립 리뷰가 끝나면 여기 표준 표가 온다⟩"*. 조건별 판정이 `A`~`E` 하위 절에 흩어져 있고 그중 `V1`~`V10` 은 조건 단위 행이 없다(관측 파일로 위임) | 회차기록 | 참 | 미관 | 없음 | docs/gates/rust-scope-references.md:63-65 | `sed -n '63,65p' docs/gates/rust-scope-references.md` |

## 내가 기각한 것

| # | 기각한 것 | 모집단 | 유효 | 해악도 | 좌표(파일:줄) | 왜 아니었나 |
|---|---|---|---|---|---|---|
| 18 | 「`D2` 의 분모가 분자와 같아 무의미하다 — `state.md` 는 인라인 캡처 후보를 1,486 으로 적는다」 | 원의도 | 거짓 | 미관 | crates/pal-extract/src/rust.rs:24-31 · docs/gates/rust-scope-references.md:202-207 | 내가 다시 셌다(파이썬 · `git ls-files '*.rs'` 138 파일): 인라인 캡처꼴 **999** · 멤버 호출꼴 **14,329** · 경로 호출꼴 **4,284**. 게이트의 987 · 13,951 · 4,217 은 **134 파일** 기준이고 이 회차가 더한 네 파일 몫만큼 작다 — 규모가 맞는다. 그리고 셋 다 추출기가 원리상 전량을 못 보므로 **분자=분모가 참값**이다. 1,486 은 다른 세는 규칙의 값이다 |
| 19 | 「게이트가 *"합(合)이 열둘을 채택 · 둘을 기각했다"* 라고 적는데 원장은 채택 13 · 기각 2 다」 | 회차기록 | 거짓 | 미관 | docs/gates/rust-scope-references.md:254 | 원장에서 `DL1-01`~`DL1-14` 의 처분을 세면 정정 4 · 범위밖 9 · 기각 1 이고, 합(合)이 `R8` 과 `R13` 을 「사실만 채택」으로 반씩 갈랐다. **「완전 채택」을 세면 12** 가 된다. 어느 셈이 정본인지 문면이 안 정했을 뿐 틀린 수라고 못 박을 수 없다 |
| 20 | 「`ScopeChain::resolve` 가 프로덕션 호출자 0 인 `pub` stub 이다」 | 저장소 | 거짓 | 미관 | crates/pal-core/src/scope.rs:309-317 | `grep -rn '\.resolve(' crates/` 의 호출자 10 곳이 **전부 같은 파일의 시험**이다. 그러나 그 자리의 문서가 *"⚠⚠ 추출기는 이 함수를 안 부른다 … 이 자리는 기본값을 고정하는 껍데기와 시험의 입구로 남았다"* 라고 스스로 적는다. **모르고 남은 표면이 아니다** |
| 21 | 「`EXTRACTOR_REV` 를 `f03-2` 에서 `f02-rust-scope` 로 **내려서** 옛 캐시 항목과 충돌한다」 | 저장소 | 거짓 | 미관 | crates/pal-extract/src/lib.rs:147 | `git log -p --all -S'EXTRACTOR_REV: &str'` 로 전 이력을 훑었더니 그 값이 쓰인 적이 없다. 캐시 키는 순서가 아니라 **다름**만 요구한다 |
| 22 | 「`RefCounts` 에 칸을 더해 옛 2층 행이 조용히 어긋난다(데이터 손상)」 | 저장소 | 거짓 | 미관 | crates/pal-core/src/projection.rs:66-95 | `ambiguous` 가 `RefCounts` 의 **마지막** 칸이고 `RefCounts` 가 `FileRow` 의 **마지막** 칸이다. postcard 는 자리 기반이라 짧은 옛 바이트열은 입력 끝에서 죽는다 — 뒤 칸이 밀려 조용히 오독되는 경로가 아니다. `lib.rs:138-144` 가 구제 절차(`rm -rf .palimpsest/index.redb .palimpsest/cache` → `pal query graph.dump`)를 적어 두었다 |

## 끝내도 되는가

**안 된다.** 본 목록에 **금지역 셋**(항 5 · 8 · 9)과 **실패 하나**(항 3)가 남았다.
항 8 은 이 회차가 놓은 전역 면제의 유일한 사유가 거짓이라 다음 회차 전체를 지배하고,
항 9 는 조건 `D4` 가 요구한 산출물 그 자체가 재현되지 않으며, 항 5 는 못 잰 조건을
통과로 적었다. 항 3(종료 보고)은 이 회차가 어차피 써야 하는 것이라 함께 닫힌다.
