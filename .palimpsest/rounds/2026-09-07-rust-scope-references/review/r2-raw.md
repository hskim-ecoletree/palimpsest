# 독립 리뷰 R2 — 원 반환문

> 잰 자: 독립 리뷰어 · 2026-09-08 · **값은 `be86499` 기준**(착수 `a774053` 의 15 커밋 뒤).
> 받은 것: 잠긴 의도 전문(원문 + 개정 + 승격) + 산출물. 회차의 대화 기록은 안 받았다.
> ⚠ **재는 동안 HEAD 가 세 번 움직였다** — `8318714` → `be86499` → `f4f7120`.
> 첫 `cargo xtask test` 는 메인이 커밋하는 **중간 워킹트리**에서 돌아 a14 둘이 빨갰다(기각 17).
> 그 뒤 값은 전부 `be86499` 에서 다시 쟀고, 핵심 다섯은 `f4f7120` 에서 재확인했다.
> 금지역 목록의 출처: `.claude/pal/policy.toml` **없음** · 잠긴 의도에 등록 목록 **없음** → **에이전트 정의의 기본 다섯**.

## 합격선 축

| 조건 | 판정 | 잰 수 | 근거 |
|---|---|---|---|
| A1 | 통과 | 시험 1 | `cargo test -p pal-extract` → `153 passed · 0 failed`. `a1_스코프_사슬이_성립한다 ... ok` |
| A2 | 통과 | 시험 1 | `a2_impl_이_스코프를_연다 ... ok` — 엣지 0 과 스코프 둘을 함께 단언 |
| A3 | 통과 | 시험 1 | `a3_평범한_호출이_엣지가_된다 ... ok` |
| A4 | 통과 | 시험 1 | `a4_매크로_안_호출이_엣지가_된다 ... ok` |
| A5 | 통과 | 시험 1 | `a5_매크로_안_멤버_이름은_참조가_아니다 ... ok` |
| A6 | 통과 | 시험 2 | `a6_매크로_안_경로_꼬리는_참조가_아니다 ... ok` · **`a6_매크로_안_구조체_리터럴_필드도_참조가_아니다 ... ok`**(R1 발견 11 이 심어 잡은 갈래가 시험으로 섰다) |
| A7 | 통과 | 시험 1 | `a7_매크로_밖_경로_꼬리도_참조가_아니다 ... ok` |
| A8 | 통과 | 시험 1 | `a8_use_절_안의_이름은_참조가_아니다 ... ok` |
| A9 | 통과 | 시험 1 | `a9_필드_식별자는_참조가_아니다 ... ok` |
| A10 | 통과 | 시험 1 | `a10_타입_이름이_값_자리에서도_해소된다 ... ok` |
| A11 | 통과 | 시험 1 · 갈래 4 | `a11_use_네_갈래가_각각_무엇을_담나 ... ok` |
| A12 | 통과 | 시험 1 | `a12_exports_는_정렬_중복제거_뒤에_요약된다 ... ok` · 음성 대조 발화 기록(`negative-controls.md`) |
| A13 | 통과 | 시험 1 | `a13_pub_의_뜻은_최상위_pub_하나다 ... ok` · `stitch_of` 인용이 게이트 §A 와 코드 주석 양쪽에 |
| A14 | 통과 | 시험 2 | `a14_지역_이름이_묶인다 ... ok` (자리 5) · `a14_지역이_자기_초기화식을_안_가린다 ... ok` |
| A15 | 통과 | 시험 1 | `a15_속성_안의_이름은_참조가_아니다 ... ok` |
| A16 | 통과 | 시험 1 | `a16_impl_메서드_이름이_모듈_스코프로_안_올라간다 ... ok` |
| A17 | 통과 | 시험 1 | `a17_cfg_쌍둥이는_엣지를_안_만든다 ... ok` — `Ambiguous` 까지 단언 |
| V1 | 통과 | 4306 (기준 4255) · 가짜 51 | `./target/release/examples/scope_variants .` (내가 다시 빌드해 다시 돌렸다) |
| V2 | 통과 | 4344 · 가짜 89 | 같은 명령 |
| V3 | 통과 | 4228 · 가짜 5 · 누락 32 | 같은 명령 |
| V4 | 통과 | 4243 · 가짜 5 · 누락 17 · `V4≠V3` | 같은 명령 — `V4 대 V3 — 다르다 ✓` |
| V5 | 통과 | 4268 · 가짜 13 | 같은 명령 |
| V6 | 통과 | 4289 · 가짜 34 | 같은 명령 |
| V7 | 통과 | 4342 · 가짜 87 | 같은 명령 |
| V8 | 통과 | 4508 · 가짜 253 | 같은 명령 |
| V9 | **정반합이 진다** | 엣지 4255 = 기준 · 가짜 0 · 누락 0 | 내가 본 사실만 적는다: 등록 문면은 「엣지 집합이 기준과 **다르다**」인데 장치의 `엣지축` 칸이 `같다` 다. 갈리는 것은 참조 갈래 축뿐(최상위 251→323 · 미해소 11889→12280 · 선언 13517→14971). 장치 마지막 줄이 *"등록된 축이 엣지 집합이므로 이 줄을 통과로 읽지 마라"* 로 스스로 지목한다 |
| V10 | 통과 | 4244 · 가짜 31 · 누락 42 | 같은 명령 |
| V11 | 통과 | 게이트 표 5 행 | `docs/gates/rust-scope-references.md:96-102` = 쌍 4,255 · 참조 6,639 · 1,198 / 3,843 / 1,598. **장치 출력과 한 자리도 안 갈린다**(내 재실행 값과 대조) |
| V12 | 통과 | 명령 1 · 바이너리 1 · 소스 1 | 게이트 `:83-87` 에 `cargo run … --example scope_variants -- .` · `target/release/examples/scope_variants` · `crates/pal-extract/examples/scope_variants.rs` |
| B1 | 통과 | 기여 `.rs` **132** (하한 80) | `./target/release/pal export --format cypher` → `REFERENCES` **4258** 줄. Symbol→path 조인으로 내가 다시 셌다: 기여 파일 133 · `.rs` **132** · `.ts` 1 |
| B2 | 통과 | 호출자 6 · 피호출자 3 | `./target/release/pal touch file_edges` |
| B3 | 통과 | 시험 1 | `b3_세_자리가_전부_present_이고_비지_않았다 ... ok` |
| C1 | 통과 | 시험 1 | `c1_rust_는_l1_로_잠겨_있다 ... ok` |
| C2 | **반증** | 결박 **30** · `fresh` **26** · `stale` **4** | `./target/release/pal query binding.status --json` 을 내가 세었다. 등록값은 `fresh 23 · stale 2`. **착수 25 는 안 움직였다**(fresh 23 · stale 2). 늘어난 stale 둘은 **이 회차가 건 결박**(`39b92bd51bbc82c6 ← 80cd88b02cc2` · `cbd70e3e8dfca3ba ← f24c2c6201a3`)이고 `be86499` 가 그 좌표를 고쳐서 낡았다. 게이트는 이 칸을 `통과 · fresh 23 · stale 2` 로 적는다(발견 4·9) |
| C3 | **반증** | `check` 27 중 **2 실패** · `test` **통과**(986 passed · 0 failed · 3 회 연속 rc=0) | `cargo xtask check` @`be86499` → `FAIL 죽은 링크 부재`(1건: 게이트 → 아직 없는 `report.md`) · `FAIL 회차 레코드`(추적 안 된 `dialectic/r1-raw.md` 17↔0). @`f4f7120` → `FAIL 죽은 링크 부재` · `FAIL 발견이 닫혔나`. **게이트가 적은 「검사 27 중 셋」과 그 셋의 표는 어느 시점과도 안 맞는다**(발견 5) |
| C4 | 통과 | ditto 4,578 행 · portal-backend 1,340 행 · **git diff 빈 출력** | `git diff --stat a774053..HEAD -- 'corpus/golden/*'` → 빈 출력 · rc 0. 게이트 `:136-149` 에 돌린 명령 두 줄과 `--bless` 안 씀이 적혀 있다. ⚠ 게이트가 댄 `git diff --stat corpus/golden/` 은 **워킹트리 대조**이고 조건이 요구한 「회차 전후」는 아니다 — 그 축은 내가 대신 쟀고 빈 출력이다 |
| C5 | **미측정** | CI 런 0 · 미푸시 커밋 15 | `gh run list --commit $(git rev-parse HEAD)` 빈 출력 · `git log origin/main..HEAD` 15 줄. 종료 시점 조건 |
| C6 | 통과 | 시험 2 | `cargo test -p pal-cli --test rust_references` → `rust_참조가_2층까지_도착하고_화면이_답한다 ... ok` · `rust_의_내보내기가_최상위_pub_만_담는다 ... ok` |
| C7 | 통과 | 골든 둘 바이트 동일 | 위 `git diff` 와 같은 근거. `ts_scopes.rs` 는 옮기기만 했다 |
| C8 | 통과 | `f03-2` → `f02-rust-scope` | `git show a774053:./crates/pal-extract/src/lib.rs` 대 현재(`:147`). 사유가 `lib.rs:120-146` 에 있고 R1 발견 10(`pal index`)도 고쳐졌다(`:143` 이 *"그 명령은 없다"* 로 적는다). ⚠ **승급 뒤 `be86499` 가 `ScopeBinding` 에 `visible_from` 을 더했는데 REV 를 다시 안 올렸다** — 격리 사본으로 재현을 시도했으나 **해가 안 났다**(기각 18) |
| C9 | 통과 | 자리 2 | `schema/graph.toml:422-427` · `crates/pal-core/src/graph.rs:166-175` 둘 다 *"L2 이상에서만"* 을 *"스코프 체인이 선 언어에서만"* 으로 다시 적었다 |
| D1 | 통과 | 이슈 1 | `gh issue view 133` → OPEN · *"Rust 추출 등급을 L1 에서 L2 로 올린다"* |
| D2 | **미측정** | 코드 주석 ✓ (`rust.rs:20-33`, 몫 셋 + 넷째 + 분모) · 종료 보고 ✗ | 조건이 **「코드 주석과 종료 보고에 있다」**인데 `report.md` 가 아직 없다 |
| D3 | 통과 | 자리 1 | `crates/pal-extract/src/classify.rs` 의 `grade_of` 옆 — 왜 `L1` 인지 · 언제 풀리는지(#133) |
| D4 | **반증** | 등록 검색어 4 중 **2 가 빠졌다** · 갱신 안 된 자리 2 | 잠긴 의도 `:128` 의 검색어는 「참조 엣지 3」·「기여 0」·**「아직 없다」**·**「스코프를 안 만든다」**인데 게이트 `:205-206` 은 「참조 엣지 3」·「기여 0」·「스코프 없음」·「REFERENCES 3」·「Rust 134 파일 기여」를 찾았다고 적는다(발견 3). 그리고 `doctor.rs:304`·`touch.rs:397` 의 수가 아직 낡았다(발견 7) |
| D5 | 통과 | 자리 1 | `crates/pal-cli/src/doctor.rs:290-318` — `.absent` 로 정했고 왜 참인지(뷰가 엣지를 안 읽는다)가 표로 있다. 실물 `pal doctor` 도 그렇게 산출한다 |
| D6 | 통과 | 자리 4 | `rust.rs:1` · `classify.rs` · `scope.rs` 의 `ScopeKind`(`:50-71`, `trait_item` 까지 적었다) · `ScopeChain`. R1 발견 14·15 가 둘 다 반영됐다 |
| D7 | 통과 | 절 1 | `docs/adr/0027-….md:140-176` 「개정」 절 — 금지역 둘을 각각 처분 |
| D8 | 통과 | 줄 2 | `./target/release/pal touch file_edges` 실물 화면에 ※ 두 줄이 뜬다 |
| E1 | **정반합이 진다** | 표본 50 · 재현 **바이트 동일** | `scope_variants . --표본` 산출이 `observations/e1-sample-50.txt` 와 `diff` rc=0. 내가 무작위로 연 둘(`install/inside.rs:61 join → 글자로_벗어나나` · `narrative.rs:97 fragment → 자리`)은 **참**이었다. 가짜 0/50 의 판정은 내가 안 한다 |
| E2 | **정반합이 진다** | — | 이슈 [#130] 과 A~D 의 대조는 문서 판정이고 정반합의 자리다 |
| E3 | **정반합이 진다** | 기준 21(파일 10) · `impl` 미개방 83(파일 30) | 장치가 그렇게 산출한다. 소유자 답의 「72」와도 「0」과도 다르다. 장치가 *"남은 것이 `cfg` 쌍둥이뿐은 아니다"* 로 스스로 적는다 |

## 미측정 목록

| # | 안 잰 조건 | 모집단 | 유효 | 해악도 | 좌표(파일:줄) | 왜 못 쟀나 |
|---|---|---|---|---|---|---|
| 1 | `C5` — 회차의 마지막 커밋 SHA 에 `conclusion=success` 런 | 원의도 | 참 | 실패 | .palimpsest/rounds/2026-09-07-rust-scope-references/intent.md:117 | 아직 push 안 됐다 — `gh run list --commit be86499…` 빈 출력 · `git log origin/main..HEAD` 15 줄. ⚠ 지금 상태로 push 하면 CI 의 `cargo xtask check` 단계가 빨개진다 |
| 2 | `D2` — 못 세는 몫 셋의 수가 **종료 보고**에 있다 | 원의도 | 참 | 거짓신호 | intent.md:126 | `report.md` 가 아직 없다. 코드 주석(`crates/pal-extract/src/rust.rs:20-33`) 쪽은 충족 |

## 의도 축

### 빠진 것

| # | 발견 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 근거(명령·출력) |
|---|---|---|---|---|---|---|---|
| 3 | **`D4` 가 등록한 검색어 넷 중 둘이 게이트의 검색 목록에서 빠지고 다른 셋으로 바뀌었다.** 잠긴 문면은 「참조 엣지 3」·「기여 0」·**「아직 없다」**·**「스코프를 안 만든다」**인데 게이트는 「참조 엣지 3」·「기여 0」·「스코프 없음」·「REFERENCES 3」·「Rust 134 파일 기여」를 찾았다고 적는다. **재는 모집단이 등록된 것과 다르다** | 원의도 | 참 | 거짓신호 | D4 | docs/gates/rust-scope-references.md:205-206 · .palimpsest/rounds/2026-09-07-rust-scope-references/intent.md:128 | `sed -n '128p' intent.md` 대 `sed -n '205,206p' docs/gates/rust-scope-references.md`. 빠진 검색어를 내가 돌려 보니 `grep -rn "아직 없다"` 는 `corpus/criteria.toml:874` 를 포함해 여덟 자리, `grep -rn "스코프를 안 만든다"` 는 `crates/pal-core/src/file_graph.rs:19` 한 자리를 낸다 |

### 요구되지 않은 것

없음 — `pal_core::ResolveRule` 과 `ScopeBinding::visible_from` 이 계획의 *"위층은 안 만진다"* 를 넘지만, 게이트 `:59-61` 이 그 사실과 까닭을 스스로 적었고 잠긴 의도의 `## 범위 밖` 어느 줄도 그것을 금하지 않는다.

### 있는데 틀린 것

| # | 발견 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 근거(명령·출력) |
|---|---|---|---|---|---|---|---|
| 4 | **게이트의 `C2` 행이 「통과 · `fresh 23 · stale 2` — 착수 값 그대로」라고 적는데 그 명령은 그 수를 안 낸다.** 실측은 결박 **30** · `fresh` **26** · `stale` **4** 다. 착수 25 는 정말 안 움직였지만, **늘어난 stale 둘은 이 회차가 §11 조건 4 로 건 결박 자신**이다 | 회차기록 | 참 | **금지역** (사실이 아닌 것을 사실로 적음) | C2 | docs/gates/rust-scope-references.md:124 · :25 · :56 | `./target/release/pal query binding.status --json` 을 파이썬으로 세었다 → `결박 30` · `Counter({'fresh': 26, 'stale': 4})`. stale 넷: `39b92bd51bbc82c6 ← 80cd88b02cc2`(이 회차) · `cbd70e3e8dfca3ba ← f24c2c6201a3`(이 회차) · `4065e80475cf1c72 ← bd26d1d8a3ee` · `49714573e0fa654c ← 55373aa24b6c` |
| 5 | **게이트 `C3` 절이 「검사 27 중 셋」이라 적고 그 셋의 원인 표를 다는데, 그 상태는 이미 끝났다.** 표의 세 줄 중 「회차 레코드」와 「발견이 닫혔나」는 `4d3bb76` 이후 **초록**이고, 「죽은 링크」의 원인도 표가 적은 「원 반환문의 뿌리 기준 링크」가 아니라 **게이트 자신이 가리키는 아직 없는 `report.md`** 다. *"앞의 둘은 원 반환문을 안 고치는 한 이 회차가 못 닫는다"* 는 문장은 **이미 닫힌 뒤에 쓰였다**(게이트를 만든 `8318714` 가 `4d3bb76` 보다 뒤다) | 회차기록 | 참 | **금지역** (사실이 아닌 것을 사실로 적음) | C3 | docs/gates/rust-scope-references.md:151-176 | `cargo xtask check` @`be86499` → `ok 회차 레코드 …` · `ok 발견이 닫혔나 …` · `죽은 링크 부재: 죽은 링크 1건: docs/gates/rust-scope-references.md → ../../.palimpsest/…/report.md` · `Error: 2개 검사가 실패했다`. `git log --oneline` 이 `4d3bb76`(장치 고침) → `8318714`(게이트 작성) 순서를 보인다 |
| 6 | **`measurements.md` 가 `4,258` 과 `4,255` 의 차이를 「`pal` 은 git 이 추적하는 **134** 개를 본다」로 설명하는데 그 수가 거짓이고, 같은 파일 안에서 자기모순이다.** `:78-80` 은 *"지금은 `git ls-files '*.rs'` 도 변형 대조 장치도 **둘 다 138**"* 이라 적는다 | 회차기록 | 참 | **금지역** (사실이 아닌 것을 사실로 적음) | D2 | .palimpsest/rounds/2026-09-07-rust-scope-references/observations/measurements.md:27-29 (대 :78-80) | `git ls-files '*.rs' \| wc -l` → **138** · `git ls-files --others --exclude-standard '*.rs' \| wc -l` → **0** · `pal export` 의 `language: "Rust"` File 노드 → **138**. 셋이 같으므로 「장치 138 대 pal 134」라는 갈림 자체가 없다 |
| 7 | **제품 코드 주석 두 자리의 수가 낡았고, 그중 하나는 자기 안에서도 안 맞는다.** `touch.rs` 는 *"Rust 4,168 엣지: 타입 참조 1,193 · 호출·매크로 3,752 · 그 밖 1,596"* 인데 **셋의 합이 6,541** 로 어느 계수기와도 안 맞고, `doctor.rs` 는 *"이제 4,168 건"* 이다. 실측은 엣지 4,258(`pal`)·4,255(장치) · 갈래 1,198 / 3,843 / 1,598(합 6,639) | 원의도 | 참 | 거짓신호 | D2 · D4 · D8 | crates/pal-cli/src/touch.rs:397-398 · crates/pal-cli/src/doctor.rs:304 | `sed -n '397,398p' crates/pal-cli/src/touch.rs` · `sed -n '304p' crates/pal-cli/src/doctor.rs` · `./target/release/pal export --format cypher \| grep -c REFERENCES` → `4258` · `./target/release/examples/scope_variants .` → `기준 — 엣지 4255` · `V11 갈래 분포 — 타입 참조 1198 · 호출·매크로 3843 · 그 밖 1598 (참조 6639)` |
| 8 | **`docs/plan/02-order.md` 의 실측표가 아직 두 동강이다** — `C6`·`U` 두 행이 삽입된 절 뒤에 남아 **표가 아니라 문단 글자로 렌더링된다.** 수는 갱신됐는데(4,258 · 132) 구조는 안 고쳐졌다 | 원의도 | 참 | 거짓신호 | D4 | docs/plan/02-order.md:68-69 | `sed -n '44,70p' docs/plan/02-order.md > frag.md` 뒤 `python3 -c "import markdown; …markdown(…, extensions=['tables'])"` → `table count: 2` 이고 마지막 `<p>` 안에 `| <strong>C6</strong> 시간축 \| … \|` 두 줄이 **문단 텍스트로** 들어 있다 |

## 이번 라운드의 새 발견

| # | 발견 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 근거(명령·출력) |
|---|---|---|---|---|---|---|---|
| 9 | **이 회차가 결정을 못 박은 결박 다섯 중 둘이 회차가 끝나기 전에 이미 `stale` 이 됐고, 어느 산출물도 그것을 안 적는다.** `be86499` 가 `resolve_shadowing` 과 패턴 거르기를 고치면서 바로 그 좌표들을 움직였다. `bindings.md` 는 아직 *"착수 시점 결박 스물다섯 — 안 움직였다"* 만 적고 자기 다섯의 낡음은 한 줄도 없다. **§11 조건 4 의 결박이 「낡음이 표시되는 자리」라는 것이 이 회차 안에서 실증됐는데 그 관측이 안 실렸다** | 원의도 | 참 | 거짓신호 | C2 | .palimpsest/rounds/2026-09-07-rust-scope-references/observations/bindings.md:19-26 · crates/pal-core/src/scope.rs:351 | `./target/release/pal touch resolve_shadowing` → `■ 이 좌표에 걸린 것 (1) / [39b92bd51bbc82c6] 최신 상태 아님(stale) ← 1 개가 변했습니다`. `pal query binding.status --json` 의 stale 넷 중 둘이 `bindings.md` 의 「이 회차가 건 다섯」 표에 있는 `39b92bd51bbc82c6` · `cbd70e3e8dfca3ba` 다 |
| 10 | **회차 기록 장치에 죽은 가지가 남았다** — `4d3bb76` 이 파싱 블록을 함수 둘로 빼면서 옛 블록을 지우지 않고 `if False:` 로 감쌌다. 세 줄이 영원히 안 도는 코드로 배포된다 | 규약 | 참 | 미관 | 없음 | .claude/skills/round/bin/extract.py:233-236 | `grep -n "if False:" -A 4 .claude/skills/round/bin/extract.py` → `233: if False:` / `234: m = None` / `235: if m:` / `236: pass`. `python3 -m py_compile` 통과(문법 오류는 아니다) |

## 자기 산출에 대한 발견

| # | 발견 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 근거(명령·출력) |
|---|---|---|---|---|---|---|---|
| 11 | **잠긴 의도의 붙여넣기 파손이 아직 안 고쳐졌다.** `⚠ **그래서 지금` 으로 시작하는 문장이 `cargo xtask check` 출력 27 줄을 통째로 삼켜 주어와 술어가 갈라져 있다. R1 발견 16 의 **거짓 진술 쪽은 ⚠⚠ 문단으로 정정됐지만 문서 파손 자체는 그대로**다. 그리고 삼켜진 출력은 착수 시점의 것도 지금의 것도 아닌 중간 상태라 그것을 현재로 읽을 여지가 남는다 | 회차기록 | 참 | 거짓신호 | 없음 | .palimpsest/rounds/2026-09-07-rust-scope-references/intent.md:242-268 | `sed -n '236,285p' …/intent.md` — `242` 행이 `⚠ **그래서 지금   ok    의존 방향  — 크레이트 8개, 규칙 4` 로 시작하고 `268` 행이 `  ok    완성 장면 형식  — 장면 5개 · 소제목 셋 전부 갖춤 의 「회차 레코드」가 빨갛다.**  은 종료 시점 조건이고` 로 끝난다 |
| 12 | **음성 대조 여섯을 최종 산출에서 다시 안 쟀고, 관측 파일의 머리말과 표의 출처가 갈린다.** 머리말은 *"산출 커밋 `11b6162`"* 인데 `V` 표의 기준값(4,255)은 `be86499` 의 것이다. `B1·B2`·`C1`·`C2`·`C6`·`C7`·`A12` 여섯 사본은 전부 `11b6162` 에서 떴고, 그 뒤 `be86499` 가 `scope.rs`·`rust_scopes.rs`·`scopes.rs` 를 고쳤다 — **발화가 최종 코드에서 재현되는지는 안 잰 것이다** | 회차기록 | 참 | 거짓신호 | 없음 | .palimpsest/rounds/2026-09-07-rust-scope-references/observations/negative-controls.md:3-4 · :26 | `sed -n '1,30p' negative-controls.md` — 머리말 `산출 커밋 11b6162` 대 `:26` 의 `기준은 엣지 쌍 **4,255**`. `git show be86499 --stat` 이 그 세 파일을 보인다 |
| 13 | **변형 대조 장치의 `판정` 칸이 아직 `V9` 를 `다르다 ✓` 로 적는다.** R1 발견 20 이 잡은 뒤 **요약 줄만** 고쳐졌다(`△ 변형 1 개는 엣지 집합에서 안 갈렸다 … 이 줄을 통과로 읽지 마라`). 행 단위로 표를 옮겨 적는 사람은 여전히 `엣지축 같다 · 판정 다르다 ✓` 라는 서로 반대인 두 칸을 본다 | 자기장치 | 참 | 거짓신호 | V9 | crates/pal-extract/examples/scope_variants.rs · observations/v-variants.txt | `./target/release/examples/scope_variants .` → `V9   use 절 배제를 끈다   4255   0   0   같다   다르다 ✓` 와 마지막의 `△ 변형 1 개는 **엣지 집합에서 안 갈렸다** … V9` |
| 14 | 게이트 `## 효과` 절이 붙인 원 출력의 **좌표 둘이 낡았다** — `scope.rs:155`·`scope.rs:325` 로 적혀 있는데 지금은 `:177`·`:351` 이다. 호출자·피호출자 수(17·1 · 0·4)는 지금도 재현된다 | 회차기록 | 참 | 미관 | 없음 | docs/gates/rust-scope-references.md:290 · :295 | `./target/release/pal touch RefResolution` → `enum · crates/pal-core/src/scope.rs:177` · `호출자 17 · 피호출자 1` · `./target/release/pal touch resolve_shadowing` → `fun · crates/pal-core/src/scope.rs:351` · `호출자 0 · 피호출자 4` |
| 15 | 게이트 `## 범위 밖` 의 `RefResolution::OutsideFile` **11,976** 이 낡았다 — 장치는 미해소 **11,889**, `pal` 은 범위 미해소 **11,911** 을 낸다 | 회차기록 | 참 | 거짓신호 | 없음 | docs/gates/rust-scope-references.md:315 | `./target/release/examples/scope_variants .` → `미해소 11889` · `./target/release/pal query graph.dump` → `범위 미해소 11911` |
| 16 | `state.md` 가 아직 *"지금 단계: **실행**"* 이고 「남은 것」이 `1. 계획 1~9 를 순서대로` 로 시작한다. 계획 아홉은 끝났고 게이트도 섰다 — 다음 컨텍스트가 이 파일을 먼저 읽으면 이미 한 일을 다시 한다 | 회차기록 | 참 | 미관 | 없음 | .palimpsest/rounds/2026-09-07-rust-scope-references/state.md:7-9 · :140-143 | `cat state.md` · 대조: `docs/gates/rust-scope-references.md` 가 `8318714` 에 섰고 `crates/pal-extract/src/rust_scopes.rs` 가 `11b6162` 에 섰다 |

## 내가 기각한 것

| # | 기각한 것 | 모집단 | 유효 | 해악도 | 좌표(파일:줄) | 왜 아니었나 |
|---|---|---|---|---|---|---|
| 17 | `cargo xtask test` 가 실패한다 — `a14_지역_이름이_묶인다` · `a14_지역이_자기_초기화식을_안_가린다` 가 **등록되지 않은 실패**로 걸린다 | 저장소 | 거짓 | — | crates/pal-extract/src/rust.rs:873,888 | 한 번 났고 재현이 안 됐다. 원인은 **내가 재는 동안 메인이 `be86499` 를 커밋하던 중간 워킹트리**였다(그 커밋이 정확히 그 두 시험을 만든 커밋이다). 그 뒤 세 번 연속 `rc=0` 이고 `cargo test --workspace` 는 `986 passed · 0 failed` 다 |
| 18 | `EXTRACTOR_REV` 를 승급한 **뒤에** `ScopeBinding` 에 `visible_from` 을 더했으므로 옛 1층 캐시 항목이 같은 키로 새 스키마에 읽혀 조용히 오독된다 (`C8` 이 정확히 이 형태를 등록했다) | 원의도 | 거짓 | — | crates/pal-core/src/scope.rs:115 · crates/pal-extract/src/lib.rs:147 | **격리 사본에서 재현을 시도했고 해가 안 났다.** `git clone --local` 로 뜬 사본에서 `8318714` 을 빌드해 1층 캐시를 채우고(항목 1,139), `be86499` 로 다시 빌드해 2층만 지우고 돌렸다 → `rc=0` · `노드 3205 · 엣지 4258` 로 **냉캐시와 같은 값** · 격리(`quarantine`) 파일 **0 건**. `crates/pal-store/src/cache.rs:368-385` 가 복호 실패를 `Lookup::Corrupt` 로 격리하고 재계산한다 — **조용한 오독 경로가 아니다.** 남은 것은 「REV 를 안 올려도 되는가」라는 규율 문제뿐이고 산출은 안 틀린다 |
| 19 | 이 회차의 게이트에 표준 판정 표가 없어 **원장 대조가 이 회차를 조용히 건너뛴다**(`형식 이전` 으로 분류돼 초록이 된다) | 규약 | 거짓 | — | xtask/src/main.rs:6058-6078 | 방어가 이미 있다. `report.md` 가 있는 회차는 표준 표가 없고 `docs/gates/README.md` 의 **닫힌** 선언 목록에도 없으면 **실패**로 떨어진다. 지금 초록인 것은 `report.md` 가 아직 없어서(진행 중)이고, 종료 보고를 쓰는 순간 이 검사가 표준 표를 강제한다. `check_declared_lists` 가 그 목록에 하한 `2026-08-19` 를 걸어 새 회차가 못 들어가게 한다 |
| 20 | `state.md` 의 *"`#[…]` 속성 안 이름이 참조가 된다 — 참조 **4,258**"* 이 최종 엣지 수(4,258)로 치환돼 사전부검 기록이 오염됐다 | 회차기록 | 거짓 | — | .palimpsest/rounds/2026-09-07-rust-scope-references/state.md:35 | 우연히 같은 수다. 1 차 출처가 그대로 그 수를 낸다 — `premortem/r2-raw.md:55` → *"실측: 이 저장소에서 속성 안 이름 **4,258 개**가 참조가 되고 그중 **60 개가 진짜 REFERENCES 엣지**"*. `git show 120e3ba:./…/state.md` 에도 같은 줄이 있고 그때는 최종 엣지 수가 존재하지도 않았다 |
| 21 | `corpus/criteria.toml:874` 의 *"파일 간 해소는커녕 파일 내 참조도 아직 없다"* 가 `D4` 누락이다 | 저장소 | 거짓 | — | corpus/criteria.toml:874 | **새 발견이 아니다.** R1 이 이미 냈고(`IR1-08`) 원장에 `처분=범위밖` 으로 실려 있다 — `python3` 로 `findings.jsonl` 을 읽어 확인했다. 문면이 그대로인 것은 처분과 어긋나지 않는다 |

## 끝내도 되는가

**안 된다.** 본 목록에 금지역 셋(발견 4 `C2` 행 · 5 `C3` 절 · 6 `measurements.md` 의 134)이 남았고, 합격선 축에 반증 넷(`C2`·`C3`·`D4` 그리고 `V9` 는 정반합의 자리)과 미측정 둘(`C5`·`D2` 종료 보고 몫)이 있다. 실패 등급은 `C5` 하나이고 그것은 push 로 닫힌다.
