# 독립 리뷰 R1 — 원 반환문

> 잰 자: 독립 리뷰어 · 2026-09-08 · HEAD `3de5e81` (착수 `a774053` 의 12 커밋 뒤)
> 받은 것: 잠긴 의도 전문 + 산출물. 회차의 대화 기록은 안 받았다.
> ⚠ 이 회차는 **진행 중**이다 — 게이트 문서(`docs/gates/rust-scope-references.md`)와
> 종료 보고가 아직 없고, `dialectic/1-thesis.md` 가 추적 안 된 채 워킹트리에 있다.
> ⚠ 재는 동안 메인이 커밋 셋을 더 얹었다(`7dbc1d2` → `3de5e81`). 값은 `3de5e81` 기준이다.

## 합격선 축

| 조건 | 판정 | 잰 수 | 근거 |
|---|---|---|---|
| A1 | 통과 | 시험 1 | `cargo test -p pal-extract` → `rust::tests::a1_스코프_사슬이_성립한다 ... ok` |
| A2 | 통과 | 시험 1 | `a2_impl_이_스코프를_연다 ... ok` — 엣지 0 · `new` 가 서로 다른 스코프 둘 |
| A3 | 통과 | 시험 1 | `a3_평범한_호출이_엣지가_된다 ... ok` |
| A4 | 통과 | 시험 1 | `a4_매크로_안_호출이_엣지가_된다 ... ok` |
| A5 | 통과 | 시험 1 | `a5_매크로_안_멤버_이름은_참조가_아니다 ... ok` ⟨발견 11 이 이 조건의 **경계**를 짚는다⟩ |
| A6 | 통과 | 시험 1 | `a6_매크로_안_경로_꼬리는_참조가_아니다 ... ok` (머리 `S` 는 참조로 남는 것까지 단언) |
| A7 | 통과 | 시험 1 | `a7_매크로_밖_경로_꼬리도_참조가_아니다 ... ok` — 실린 자리 수로 잰다 |
| A8 | 통과 | 시험 1 | `a8_use_절_안의_이름은_참조가_아니다 ... ok` |
| A9 | 통과 | 시험 1 | `a9_필드_식별자는_참조가_아니다 ... ok` |
| A10 | 통과 | 시험 1 | `a10_타입_이름이_값_자리에서도_해소된다 ... ok` |
| A11 | 통과 | 시험 1 · 갈래 5 | `a11_use_네_갈래가_각각_무엇을_담나 ... ok` — `modules` 축과 스코프 이름 축을 따로 단언 |
| A12 | 통과 | 시험 1 | `a12_exports_는_정렬_중복제거_뒤에_요약된다 ... ok` · 음성 대조 발화 확인(`negative-controls.md`) |
| A13 | 통과 | 시험 1 · 갈래 6 | `a13_pub_의_뜻은_최상위_pub_하나다 ... ok` · 근거 인용 `crates/pal-cli/src/ledger.rs:377` 이 코드 주석에도 있다 |
| A14 | 통과 | 시험 1 · 자리 5 | `a14_지역_이름이_묶인다 ... ok` — 파라미터·`let`·`match` 팔·`for`·`if let` 다섯 |
| A15 | 통과 | 시험 1 | `a15_속성_안의_이름은_참조가_아니다 ... ok` |
| A16 | 통과 | 시험 1 | `a16_impl_메서드_이름이_모듈_스코프로_안_올라간다 ... ok` |
| A17 | 통과 | 시험 1 | `a17_cfg_쌍둥이는_엣지를_안_만든다 ... ok` — `Ambiguous` 로 적히는 것까지 단언 |
| V1 | 통과 | 엣지 4260 (기준 4209) · 가짜 51 | `cargo run -q --release -p pal-extract --example scope_variants -- .` |
| V2 | 통과 | 4340 · 가짜 131 | 같은 명령 |
| V3 | 통과 | 4185 · 가짜 7 · 누락 31 | 같은 명령 |
| V4 | 통과 | 4199 · 가짜 7 · 누락 17 · `V4≠V3` 다르다 | 같은 명령 — 「`V3` 과 같은 집합이 나오면 안 된다」가 성립 |
| V5 | 통과 | 4222 · 가짜 13 | 같은 명령 |
| V6 | 통과 | 4242 · 가짜 33 | 같은 명령 |
| V7 | 통과 | 4296 · 가짜 87 | 같은 명령 |
| V8 | 통과 | 4426 · 가짜 217 | 같은 명령 |
| V9 | **반증** | 엣지 4209 = 기준 · 가짜 0 · 누락 0 | 등록 문면은 「엣지 집합이 기준과 **다르다**」(`V1` 이 그 축을 정의한다). 장치도 *"⚠ V9 은 엣지 축이 아니라 참조 갈래 축에서만 갈린다"* 로 적고, 그 다른 축(최상위 251→323 · 미해소 12001→12392 · 선언 13461→14910)에서만 갈린다. **등록된 축에서는 안 갈렸다** |
| V10 | 통과 | 4240 · 가짜 31 | 같은 명령 |
| V11 | 미측정 | — | 「**게이트에 적는다**」인데 `docs/gates/rust-scope-references.md` 가 없다. 값 자체는 `observations/measurements.md:26-31` 에 있다(1,197 / 3,796 / 1,598 · 참조 6,591 · 쌍 4,209) |
| V12 | 미측정 | — | 같은 이유. 명령·바이너리 경로는 `observations/negative-controls.md:7-9` 에 있다 |
| B1 | 통과 | 기여 `.rs` **132** (하한 80) | `./target/release/pal export --format cypher` → `REFERENCES 4212건`. Symbol→path 로 조인해 셌다: 기여 파일 전체 133 · `.rs` **132** · `.ts` 1 |
| B2 | 통과 | 호출자 6 · 피호출자 3 | `./target/release/pal touch file_edges` |
| B3 | 통과 | 시험 1 · 자리 3 | `b3_세_자리가_전부_present_이고_비지_않았다 ... ok` |
| C1 | 통과 | 시험 1 | `c1_rust_는_l1_로_잠겨_있다 ... ok` · 음성 대조 발화 확인 |
| C2 | **반증** | `fresh` **28** · `stale` 2 · 결박 **30** | `./target/release/pal query binding.status --json` 을 세었다. 등록값은 `fresh 23 · stale 2`(결박 25). **착수 25 건은 안 움직였고** 이 회차가 `pal bind` 로 다섯을 새로 걸어 `fresh` 가 늘었다(`observations/bindings.md`). ⚠ **결박 다섯을 지워 수를 맞추는 것은 고침이 아니다** — 사실을 적는 것이 답이다 |
| C3 | **반증** | `check` 27 중 **2 실패** · `test` **985 통과 0 실패** | `cargo xtask check` → `FAIL 죽은 링크 부재`(5건) · `FAIL 회차 레코드` · `Error: 2개 검사가 실패했다`. `cargo xtask test` → rc 0 · passed 985 · failed 0 |
| C4 | 미측정 | 골든 ditto 4578 줄 · portal-backend 1340 줄 · **둘 다 바이트 동일** · `git diff a774053..HEAD -- '*symbols.tsv'` 빈 출력 | 실질은 내가 쟀다(`f03-3-verify.py` 의 `snapshot`·`diff` 를 직접 불러 대조). **다만 조건이 요구한 「돌린 명령을 게이트에 적고」가 게이트 부재로 미충족**이고, 등록된 러너를 그대로 돌리면 ③ 에 못 간다(발견 17) |
| C5 | 미측정 | CI 런 0 | `gh run list --commit 3de5e813...` 빈 출력. 회차 커밋이 아직 push 되지 않았다 — 종료 시점 조건 |
| C6 | 통과 | 시험 2 | `cargo test -p pal-cli --test rust_references` → `rust_참조가_2층까지_도착하고_화면이_답한다 ... ok` · `rust_의_내보내기가_최상위_pub_만_담는다 ... ok`. 추출 → 1층 → `stitch_of` → `pal touch` 를 실제로 지난다 |
| C7 | 통과 | ditto 4578 줄 **바이트 동일 True** · portal-backend 1340 줄 **True** | `f03-3-verify.py` 의 `snapshot(repo, pin, tmp)` 를 직접 불러 골든과 대조했다. `--bless` 안 썼다. `git diff` 도 빈 출력 |
| C8 | 통과 | `f03-2` → `f02-rust-scope` | `git show a774053:crates/pal-extract/src/lib.rs` 대 현재. 사유가 `lib.rs:120-142` 에 적혀 있다 ⟨발견 10·18·19 가 그 승급의 대가를 짚는다⟩ |
| C9 | 통과 | 자리 2 | `schema/graph.toml:421-426` 과 `crates/pal-core/src/graph.rs:166-175` 양쪽이 *"L2 이상에서만"* 을 *"스코프 체인이 선 언어에서만"* 으로 다시 적었다 |
| D1 | 통과 | 이슈 1 | `gh issue view 133` → OPEN · *"Rust 추출 등급을 L1 에서 L2 로 올린다"* |
| D2 | 미측정 | 코드 주석 ✓ (`rust.rs:20-26`, 몫 셋 + 분모) · 종료 보고 ✗ | 조건이 **「코드 주석과 종료 보고에 있다」**인데 종료 보고가 아직 없다 |
| D3 | 통과 | 자리 1 | `crates/pal-extract/src/classify.rs:223-258` — 왜 `L1` 인지 · 언제 풀리는지(#133) |
| D4 | **반증** | 안 갱신된 자리 **2** · 게이트 기재 ✗ | 「저장소 전체에서 찾아 **전부** 갱신」인데 `crates/pal-core/src/file_graph.rs:14-21` 과 `corpus/criteria.toml:874` 가 남았다(발견 7·8). 「찾은 자리 수를 게이트에 적는다」도 게이트 부재로 미충족 |
| D5 | 통과 | 자리 1 | `crates/pal-cli/src/doctor.rs:300-314` — `.absent` 로 정했고 왜 참인지(뷰가 엣지를 안 읽는다)가 표로 있다. 실물 확인: `pal doctor` 불변식 ① 이 **검사 30 · 담지 못하는 자리에 REFERENCES(F07)** 로 나온다 |
| D6 | 통과 | 자리 4 | `rust.rs:1` · `classify.rs` 의 `grade_of` 옆 · `scope.rs` 의 `ScopeKind`(`:50-62`)·`ScopeChain`(`:207-217`) ⟨발견 14·15 가 남은 흠을 짚는다⟩ |
| D7 | 통과 | 절 1 | `docs/adr/0027-...md:139-176` 「개정」 절 — 금지역 둘을 각각 「그 팔을 안 쓴다」·「남아 있다(모집단 0)」로 처분 |
| D8 | 통과 | 줄 2 | `crates/pal-cli/src/touch.rs:399-412` · 실물 화면에서 두 ※ 줄 확인 |
| E1 | 정반합이 진다 | 표본 50 · 재현 **바이트 동일** | `--표본` 산출이 `observations/e1-sample-50.txt` 와 `diff` rc=0. 내가 무작위로 연 둘(`install/inside.rs:69 join → 실제_경로` · `install/blocks.rs:236 상태 → 자리`)은 참이었다. **가짜 0/50 의 판정은 내가 안 한다** |
| E2 | 정반합이 진다 | — | 내가 본 사실: 이슈 [#130] 의 이 회차 몫이 A~D 로 덮이는가는 문서 대조이고 정반합의 자리다 |
| E3 | 정반합이 진다 | 기준 21(파일 10) · `impl` 미개방 83(파일 30) | 장치가 산출한다. **소유자 답의 「72」와도 다르고 「0」과도 다르다.** 장치는 *"이 자리는 엣지가 아니라 `Ambiguous` 다"* 로 적는다 — 그 재해석이 옳은지는 정반합의 자리다 |

## 미측정 목록

| # | 안 잰 조건 | 모집단 | 유효 | 해악도 | 좌표(파일:줄) | 왜 못 쟀나 |
|---|---|---|---|---|---|---|
| 1 | `V11` — 기준 엣지 수와 갈래 분포를 게이트에 적는다 | 원의도 | 참 | 거짓신호 | .palimpsest/rounds/2026-09-07-rust-scope-references/intent.md:102 | 게이트 문서가 아직 없다. 값 자체는 `observations/measurements.md:26-31` 에 있다 |
| 2 | `V12` — 돌린 명령과 바이너리 경로가 게이트에 있다 | 원의도 | 참 | 거짓신호 | intent.md:103 | 같은 이유. `observations/negative-controls.md:7-9` 에 있다 |
| 3 | `C4` — 골든 대조를 돌린 명령을 게이트에 적는다 | 원의도 | 참 | 거짓신호 | intent.md:116 | 게이트 부재. 골든 불변 자체는 내가 쟀다(둘 다 바이트 동일) |
| 4 | `C5` — 마지막 커밋 SHA 에 `conclusion=success` 런 | 원의도 | 참 | 실패 | intent.md:117 | 아직 push 안 됨 · `gh run list --commit` 빈 출력. ⚠ 지금 상태로 push 하면 CI 의 `cargo xtask check` 단계가 빨개진다(`.github/workflows/ci.yml:121`) |
| 5 | `D2` — 못 세는 몫 셋의 수가 **종료 보고**에 있다 | 원의도 | 참 | 거짓신호 | intent.md:126 | 종료 보고가 아직 없다. 코드 주석 쪽은 충족 |

## 의도 축

### 빠진 것

| # | 발견 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 근거(명령·출력) |
|---|---|---|---|---|---|---|---|
| 6 | `cargo xtask check` 가 통과하지 않는다 — 죽은 링크 5 · 회차 레코드 1. 그중 **둘은 이 회차의 계획 문서가 아직 없는 게이트를 가리켜서** 생겼다 | 원의도 | 참 | 실패 | C3 | docs/plan/02-order.md:67 · docs/plan/03-shortest-path.md:92 | `cargo xtask check` → `죽은 링크 부재: 죽은 링크 5건: … docs/plan/02-order.md → ../gates/rust-scope-references.md · docs/plan/03-shortest-path.md → ../gates/rust-scope-references.md` · `Error: 2개 검사가 실패했다` |
| 7 | `D4` 가 요구한 「스코프를 안 만든다」 자리 하나가 안 갱신됐다. 그 문단은 *"지금 없는 자리 … `scopes`·`export_digest` 를 빈 값으로 미리 세우지 않는다"* 인데 **둘 다 이미 필드다**(`:215`·`:223`) | 저장소 | 참 | 거짓신호 | D4 | crates/pal-core/src/file_graph.rs:14-21 | `grep -rn "스코프를 안 만든다"` → `crates/pal-core/src/file_graph.rs:18`. `git show a774053:…file_graph.rs` 로 착수 전부터 있던 문면임을 확인 |
| 8 | `D4` 의 검색어 「아직 없다」가 걸리는 자리 하나가 안 갱신됐다 — *"파일 간 해소는커녕 **파일 내 참조도 아직 없다**"* | 저장소 | 참 | 거짓신호 | D4 | corpus/criteria.toml:874 | `grep -rn "아직 없다" --exclude-dir=.palimpsest` → 두 자리 중 하나. 다른 하나(`docs/overview.md:1028`)는 지금도 참이라 기각했다(발견 30) |

### 요구되지 않은 것

| # | 발견 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 근거(명령·출력) |
|---|---|---|---|---|---|---|---|
| 9 | `ScopeChain::resolve` 가 **생산 호출자 0** 이 됐다 — 이 회차가 두 추출기를 `resolve_with` 로 옮기면서 시험 전용 껍데기가 됐는데 문서는 여전히 *"다른 규칙이 필요한 언어는 `resolve_with` 를 부른다"* 로 자기가 상시 경로인 듯 적는다 | 원의도 | 참 | 미관 | 없음 | crates/pal-core/src/scope.rs:289 | `grep -rn "\.resolve(" crates --include='*.rs'` 에서 `resolve_name`·`resolve_with` 를 뺀 10 자리가 전부 `crates/pal-core/src/scope.rs` 의 `mod tests` 안(`:461`~`:590`) |

### 있는데 틀린 것

| # | 발견 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 근거(명령·출력) |
|---|---|---|---|---|---|---|---|
| 10 | **`pal index` 는 존재하지 않는 명령이다.** `EXTRACTOR_REV` 승급 주석이 *"옛 2층 행은 다시 세워야 한다(`pal index`)"* 로 없는 구제 경로를 사실처럼 적는다. 저장소 전체에서 이 문자열이 나오는 자리는 이 한 줄뿐이다 | 원의도 | 참 | **금지역** (사실이 아닌 것을 사실로 적음) | C8 | crates/pal-extract/src/lib.rs:139 | `./target/release/pal index` → `error: unrecognized subcommand 'index'` · `pal --help` 의 서브커맨드 19 개에 `index` 없음 · <code>grep -rn '`pal index`' crates docs</code> → 1 건(그 줄) |
| 11 | **매크로 토큰 열 안의 구조체 리터럴 필드명이 참조로 세어져 가짜 엣지가 선다.** 거르기가 「앞 형제가 `.`·`::`」 하나뿐이라 `Foo { bar: 2 }` 의 `bar` 는 앞 형제가 `{`·`,` 여서 안 걸린다. 같은 구문이 **매크로 밖에서는** `field_identifier` 라 옳게 걸러진다 — 이 회차가 *"가장 조심할 자리"* 로 등록한 「같은 규칙이 매크로 안팎에서 반대로 돈다」의 미처리 갈래다 | 원의도 | 참 | 거짓신호 | A5(경계) | crates/pal-extract/src/rust_scopes.rs:130-135 | 격리 저장소에 심어 재현: `pub struct Foo{pub bar:u32}` / `pub fn bar()->u32{7}` / `pub fn zed(){let f=Foo{bar:1}; assert_eq!(f.bar,1);}` / `pub fn mac(){let g=Foo{bar:2}; assert_eq!(g, Foo{bar:2});}` → `pal export --format cypher` 가 `mac → bar` 엣지를 낸다. **`zed`(매크로 밖 같은 구문)는 안 낸다.** ⚠ **이 저장소에서의 모집단은 좁힌 정규식 근사로 0 이다** (매크로 열셋 안의 `Ident { field: … }` 를 같은 파일 아이템 이름과 대조) — **기전은 참이고 이 코퍼스에서 안 걸린다.** 느슨한 첫 근사가 낸 6 은 내 정규식의 오탐이었다(발견 32) |
| 12 | **회차 산출물 안의 수가 최종 상태와 갈린다.** `e9402bc` 가 관측 넷만 다시 쟀고 코드 주석·계획 문서·이슈는 그 전 값 그대로다. 실측 최종: `pal export` **4,212** · 장치 쌍 **4,209** · 참조 **6,591** · 갈래 **1,197/3,796/1,598** · 기여 `.rs` **132** · `git ls-files '*.rs'` **138** | 원의도 | 참 | 거짓신호 | D4 | crates/pal-cli/src/touch.rs:397 · crates/pal-cli/src/doctor.rs:304 · crates/pal-extract/src/rust.rs:20,28 · crates/pal-extract/src/cached.rs:6 · docs/plan/02-order.md:61 · docs/plan/03-shortest-path.md:92,122,163 | `touch.rs:397` = `4,168 엣지 · 1,193 · 3,752 · 1,596` ⟨셋의 합 6,541 로 **자기 안에서도 안 맞는다**⟩ · `doctor.rs:304` = `4,168` · `rust.rs:20` = `134 파일` · `rust.rs:28` = `6,584 · 4,179` · `cached.rs:6` = `6,584 · 134 파일` · `02-order.md:61` = `4,179 · 128` · `03-shortest-path.md` = `4,179 · 128`(세 자리). 실측은 위 칸 |
| 13 | **`docs/plan/02-order.md` 의 실측표가 삽입된 절에 의해 두 동강 났다.** `C1`~`C5` 행(`:48-52`) 뒤에 `### 그 뒤 무엇이 바뀌었나` 절과 새 표(`:54-66`)가 끼어들고, `C6`·`U` 행(`:68-69`)이 원래 표 밖으로 밀렸다. 렌더링하면 두 행이 표에서 떨어진다 | 원의도 | 참 | 거짓신호 | D4 | docs/plan/02-order.md:53-69 | <code>grep -n "^\| \*\*C[0-9]\|^\| \*\*U\|^### " docs/plan/02-order.md</code> → `52:| **C5** …` · `54:### 그 뒤 무엇이 바뀌었나` · `68:| **C6** …` · `69:| **U** …` · `71:### 실측이 갈린 자리 셋` |
| 14 | `ScopeKind::Impl` 문서가 *"`impl` 블록 하나 — **Rust 전용**"* 인데 구현은 `trait_item` 도 같은 값으로 연다 | 원의도 | 참 | 미관 | D6 | crates/pal-core/src/scope.rs:50 · crates/pal-extract/src/rust_scopes.rs:160 | `rust_scopes.rs:160` → `"impl_item" \| "trait_item" if self.impl_이_연다 => Some(ScopeKind::Impl)`. `scope.rs:50-62` 의 문서 계약에 `trait` 이 없다 |
| 15 | `Namespace` 의 문서 머리가 *"**TypeScript 의** 두 이름 공간"* 그대로다. 이 회차의 ADR-0027 개정이 *"`Namespace` 가 매크로 이름 공간을 못 담는 것 — **남아 있다**"* 로 그것을 살아 있는 금지역으로 다시 등록했는데, 정의 옆에는 그 사실도 Rust 가 쓴다는 사실도 없다 | 원의도 | 참 | 거짓신호 | D6 · D7 | crates/pal-core/src/scope.rs:69-72 | `sed -n '69,82p' crates/pal-core/src/scope.rs` · `docs/adr/0027-...md` 개정 절의 금지역 표 둘째 행 · `rust_scopes.rs:181-186` 이 `Namespace::Value`·`Type` 을 쓴다 |

## 이번 라운드의 새 발견

| # | 발견 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 근거(명령·출력) |
|---|---|---|---|---|---|---|---|
| 16 | **잠긴 의도가 적은 「그 빨강의 원인이 이 회차의 산출이 아니라 장치다」가 반증됐다.** 착수 커밋에서 `cargo xtask check` 는 **27/27 통과**였다. 지금 빨간 둘은 전부 착수 뒤에 생긴 파일이 원인이고, 죽은 링크 5 중 **2 는 이 회차의 계획 문서**가 아직 없는 게이트를 가리켜서 생겼다. 그리고 그 문단 자체가 `cargo xtask check` 출력의 통째 붙여넣기에 삼켜져 주어 둘(`cargo xtask check` · `C3`)이 사라졌다 | 회차기록 | 참 | **금지역** (사실이 아닌 것을 사실로 적음) | C3 | .palimpsest/rounds/2026-09-07-rust-scope-references/intent.md:241-269 | `git clone --no-checkout` 으로 `a774053` 사본을 뜨고 **그 사본에서 다시 빌드**해 `cargo xtask check` → `죽은 링크 부재 — 문서 490개 · 링크 671건 · 죽은 것 0건` · `회차 레코드 — 산출 18개 …` · **`검사 27/27 통과`** · exit 0. 삽입 시점은 `git show 17c99db -- …/intent.md` |
| 17 | **`scripts/f03-3-verify.py` 의 ③(골든 스냅샷 대조)이 지금 상태에서 도달 불가다.** ④ 가 `cargo xtask check` 를 먼저 부르고 실패하면 `return 1` 하므로, 이 회차가 `check` 를 빨갛게 만든 순간 골든 회귀 검사가 한 줄도 안 돈다. **초록으로 뒤집히지는 않으니 금지역으로 안 올린다** | 저장소 | 참 | 실패 | C4 · C7 | scripts/f03-3-verify.py:141-147 | `./scripts/f03-3-verify.py` → `ok ① · ② 의 시험 5 개가 성립한다` → `FAIL ④ cargo xtask check 가 실패했다` 로 끝. **③ 이 한 줄도 안 찍힌다.** 나는 `snapshot`·`diff` 를 직접 불러 우회해서 쟀다 |
| 18 | **`EXTRACTOR_REV` 승급이 `scripts/f04-verify.py` ③ 의 치환 대상을 소스에서 없앴다.** 그 스크립트는 `'pub const EXTRACTOR_REV: &str = "f03-2";'` 를 찾아 변이시키는데 값이 `"f02-rust-scope"` 로 바뀌었다. 조용히 넘어가지는 않고 `어긋남` 으로 시끄럽게 멈춘다 | 저장소 | 참 | 실패 | C8 | scripts/f04-verify.py:188 | `grep -n EXTRACTOR_REV scripts/f04-verify.py` → `188: 'pub const EXTRACTOR_REV: &str = "f03-2";'`. 현재 값은 `crates/pal-extract/src/lib.rs:143` 의 `"f02-rust-scope"`. 방어는 `scripts/f04-verify.py:236-240` |
| 19 | **옛 2층 색인을 새 바이너리로 읽으면 `pal export` 가 원인 불명 오류로 죽고, 화면에 구제 방법이 없다.** `RefCounts` 에 `ambiguous` 가 붙어 postcard 자리 기반 행이 갈렸는데 2층에는 스키마 버전 축이 없다(`META` 는 `built_for` 하나뿐). ⚠ **조용한 오독은 아니다** — rc 1 로 시끄럽게 멎는다 | 원의도 | 참 | 거짓신호 | C8 | crates/pal-core/src/projection.rs:83 · crates/pal-store/src/projection.rs:80,103 | 격리 저장소에서 `a774053` 사본의 `pal` 로 2층을 세운 뒤 새 `pal export --format cypher` → `Error: 파일을 읽지 못했다 / Caused by: 2층 값을 풀지 못했다: Hit the end of buffer, expected more data` · `rc=1`. 같은 저장소를 옛 바이너리로는 `rc=0`. 「무엇을 하면 되는지」가 출력에 없고, 코드가 가리키는 `pal index` 는 없는 명령이다(발견 10) |

## 자기 산출에 대한 발견

| # | 발견 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 근거(명령·출력) |
|---|---|---|---|---|---|---|---|
| 20 | 변형 대조 장치가 `V9` 를 **`판정: 다르다 ✓`** 로 산출하고 마지막 줄을 *"✓ 변형 열하나가 전부 기준과 갈렸다"* 로 닫는다. 그런데 등록된 축(엣지 집합)에서는 **안 갈렸다** — 장치는 바로 아래 줄에서 그 사실을 정직하게 적으므로 은폐는 아니지만, **요약 줄과 판정 칸이 축 이동을 흡수한다** | 자기장치 | 참 | 거짓신호 | V9 | crates/pal-extract/examples/scope_variants.rs · observations/v-variants.txt | `cargo run -q --release -p pal-extract --example scope_variants -- .` → `V9 use 절 배제를 끈다 4209 0 0 같다 다르다 ✓` · `⚠ V9 은 **엣지 축이 아니라 참조 갈래 축**에서만 갈린다` · `✓ 변형 열하나가 전부 기준과 갈렸다` |
| 21 | 잠긴 의도의 개정표가 *"음성 대조를 **다섯 줄에서 일곱 줄로 늘렸다**"* 로 적는데 실제 표는 **다섯 줄**이다. 라운드 2 가 `A2` 와 `A5~A9` 행을 `V` 절로 흡수하며 7→5 로 줄인 것이 개정표에 한 줄도 없다 | 회차기록 | 참 | 거짓신호 | 없음 | intent.md:220 · intent.md:144-150 | `git show 4e36e5a:…/intent.md` 의 음성 대조 표는 데이터 행 **7**, 현재는 **5**. 개정표에는 늘린 기록만 있고 줄인 기록이 없다 |
| 22 | 회차 레코드 검산이 두 자리에서 어긋난다(사전부검 R1 20↔19 · R2 17↔11) 그리고 조건평가 R1·R2 의 기계 칸이 추출기 산출과 갈린다(19건 · 20건). 이슈 [#132] 가 이미 이 형태를 진다 | 회차기록 | 참 | 실패 | C3 | .palimpsest/rounds/2026-09-07-rust-scope-references/premortem/r1-raw.md · r2-raw.md · conditions-audit/r1-raw.md · r2-raw.md | `cargo xtask check` → `회차 레코드: 합계 검산 어긋남 … 원 반환문의 항이 20 인데 … 19 행이다 (빠진 행 1)` 외 3 건 |
| 23 | 앞 라운드들의 원 반환문이 ADR 을 **뿌리 기준 상대 경로**(`docs/adr/…`)로 걸어 죽은 링크 셋이 됐다. 파일 자체는 존재한다. 이슈 [#134] 가 이 형태를 진다 | 회차기록 | 참 | 실패 | C3 | conditions-audit/r2-raw.md:66,134 · premortem/r1-raw.md:161 | `cargo xtask check` 의 죽은 링크 5 중 3 · `ls docs/adr/` 에 세 파일 다 있다 |
| 24 | `e1-hand-check.md` 가 *"표본 50 은 엣지 **4,179** 의 1.2% 다"* 로 남았다. `e9402bc` 가 관측 넷을 다시 쟀는데 이 파일의 이 줄은 안 움직였다. 최종은 4,209(장치)·4,212(`pal`) | 회차기록 | 참 | 거짓신호 | E1 | observations/e1-hand-check.md:46 | `grep -n "4,179\|4,209\|4,212" observations/*` — `e1-hand-check.md:46` 만 4,179 |
| 25 | `measurements.md` 머리말이 *"지금 값은 … 산출 커밋 `11b6162` 뒤 워킹트리에서 잰 것"* 인데, 표의 값은 `e9402bc` 에서 그 뒤 상태로 **다시 잰 것**이다(4,179→4,212 등). 머리말이 값보다 두 커밋 뒤처졌다 | 회차기록 | 참 | 거짓신호 | 없음 | observations/measurements.md:4 | `git log --oneline -- observations/measurements.md` → `e9402bc` · `7dbc1d2`. `git diff 7dbc1d2..HEAD -- observations/measurements.md` 가 값만 바꾸고 머리말은 그대로 둔다 |
| 26 | `measurements.md` 의 `D2` 절 제목이 모집단을 *"`git ls-files '*.rs'` **134** 파일"* 로 적는데 실측 **138** 이다 | 회차기록 | 참 | 거짓신호 | D2 | observations/measurements.md:47 | `git ls-files '*.rs' \| wc -l` → `138` |

## 내가 기각한 것

| # | 기각한 것 | 모집단 | 유효 | 해악도 | 좌표(파일:줄) | 왜 아니었나 |
|---|---|---|---|---|---|---|
| 27 | `cargo test --workspace` 가 실패한다 — `버전에_커밋이_실려_있다 ... FAILED` | 저장소 | 거짓 | — | crates/pal-cli/tests/version_is_in_the_binary.rs:40 | 두 번 재현됐다가 재빌드 뒤 통과했다. 원인은 **내가 재는 동안 메인이 커밋을 셋 더 얹은 것**이다(리뷰 시작 시 HEAD `7dbc1d2` → 지금 `3de5e81`). 박힌 SHA 와 `git HEAD` 가 순간적으로 갈렸을 뿐이고 지금은 `985 passed · 0 failed` |
| 28 | 새 `RefCounts` 가 옛 2층 행을 **조용히** 오독한다 | 원의도 | 거짓 | — | crates/pal-core/src/projection.rs:83 | 격리 실측: `rc=1` 로 `Hit the end of buffer` 를 내며 멎는다. 조용하지 않다. **남은 흠은 「구제 방법이 화면에 없다」쪽**이고 그것은 발견 19 로 따로 냈다 |
| 29 | `pal doctor` 의 `.absent("REFERENCES", …)` 가 이제 거짓이다 — 엣지가 4,212 건 있는데 「없다」고 선언한다 | 원의도 | 거짓 | — | crates/pal-cli/src/doctor.rs:316 | `pal doctor` 실물이 불변식 ① 을 **검사 30 · 담지 못하는 자리에 REFERENCES(F07)** 로 산출한다 — `build_view` 가 2층 엣지를 아예 안 읽으므로 선언은 뷰의 사실로서 참이다. `D5` 의 표가 그 판단과 대가를 이미 적어 뒀다 |
| 30 | `docs/overview.md:1028` 의 *"파일 경계를 넘는 참조는 아직 없다"* 가 `D4` 누락이다 | 저장소 | 거짓 | — | docs/overview.md:1028 | 지금도 참이다. 이 회차의 `## 범위 밖` 첫 줄이 파일 경계 넘기를 명시적으로 뺐다 |
| 32 | 발견 11 의 가짜 엣지가 이 저장소에 **6 자리** 있다 | 원의도 | 거짓 | — | crates/pal-cli/src/install.rs:24 | 내 첫 정규식이 매크로 인자 범위를 `bail!` 등으로 과대하게 잡아 `blocks: …` 같은 자리를 주웠다. 실제 export 를 열어 보니 `blocks` 로 가는 엣지 다섯은 전부 `blocks::상태(…)` 의 **경로 머리**라 옳은 엣지였다. 좁힌 근사로 다시 재니 **0** 이다. ⚠ **기전 자체는 격리 재현으로 참이다** — 기각한 것은 「이 코퍼스에 6 자리 있다」는 수뿐이다 |
| 31 | 골든 `symbols.tsv` 가 움직였다 | 원의도 | 거짓 | — | corpus/golden/ditto.symbols.tsv · corpus/golden/portal-backend.symbols.tsv | 직접 재생성해 대조: ditto 4,578 줄 **동일** · portal-backend 1,340 줄 **동일**. `git diff a774053..HEAD -- '*symbols.tsv'` 도 빈 출력 |

## 끝내도 되는가

**안 된다.** 본 목록에 금지역 둘(발견 10 `pal index` · 발견 16 착수 시점 진술 반증)과 실패 셋(발견 6 = `C3` · 17 · 18)이 남았고, 합격선 축에 반증 넷(`V9`·`C2`·`C3`·`D4`)과 미측정 다섯이 있다.
