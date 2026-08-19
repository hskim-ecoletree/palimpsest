사전부검 라운드 3 — 격리 사본(`scratchpad/probe`)에 `tree-sitter-rust @ 77a3747` + `tree-sitter-typescript @ 75b3874` + `brokk-tree-sitter-kotlin @ acb9630` 을 붙여 `parse.rs` 의 정규화·주석수집 규칙을 그대로 옮겨 심고, 자기 저장소 117 `.rs` · cargo `514c56d` 1,372 `.rs` · ditto 544 TS · boxwood 2,839 Kotlin 에 실제로 돌렸다. 계획이 적은 수(117 파일 · 표식 130 · cargo 1,372→380 · ditto 330→327)는 **전부 재현됐다** — 그래서 안 맞는 자리들이 의미가 있다.

---

### ④ 와 「`다음_선언` 의 `attribute_item` 건너뛰기」가 서로를 무효화한다 — 결박 43 → 16
- 어떻게 실패하나: `crates/pal-cli/src/narrative.rs:297` 이 `자리: BTreeMap<span.byte_start, SymbolId>` 를 만들고 `:303` 이 `attaches_to_byte` 로 **정확 일치** 조회한다. ④ 는 Rust 심볼의 `span.byte_start` 를 앞 `attribute_item` 시작으로 **뒤로** 옮기고, 같은 계획의 「`다음_선언` 의 `attribute_item` 건너뛰기」는 `attaches_to_byte` 를 **앞으로**(선언 마디) 옮긴다. 두 값이 서로 반대 방향으로 갈라져 **키가 안 맞는다.** 실측: 표식 주석 130 중 **49 개**가 속성을 사이에 두고 있고, ③ 적용 후 선언에 도달하는 43 중 **29** 가 그 형태다 → 43 → **16**. 둘 중 **하나만** 하면 43 이 그대로 선다(속성 시작 ↔ 속성 시작, 또는 선언 시작 ↔ 선언 시작).
- 어디가 걸리나: `crates/pal-cli/src/narrative.rs:297-303` · `crates/pal-extract/src/parse.rs:563-580` `다음_선언` · `crates/pal-extract/src/typescript.rs:569` `span_of` 의 Rust 대응물
- 획득: 조회 + 격리 사본 실측 — `모은다`/`다음_선언`/`사이에_빈_줄` 을 그대로 이식해 117 파일 전수로 세었다(`attribute_item 에 붙는 것 49 · 두 방식이 갈리는 것 49`)
- 모집단: 자기장치
- 유효성: 참
- 해악도: 실패 — ⑧ 의 완수 조건(「결박이 선다」·상한 68)이 못 선다
- 대상: 계획자신
- 얼마나 아픈가: 되돌리기 쉽다(둘 중 하나를 빼면 된다). 걸리는 곳 둘 · 영향 받는 좌표 29

### ③ 접기가 ⑧ 이 **측정 전에 못 박은 수** 130·68 을 그 자리에서 무너뜨린다
- 어떻게 실패하나: ③ 은 `doc_comment` 자식을 가진 `line_comment` 를 접는다. 자기 저장소 표식 주석 130 중 **104 가 `///`** 다. 접으면 조각이 130 → **85**, 선언 마디에 도달하는 것이 **43** 이 된다. 그런데 ⑧ 은 접기를 안 넣은 셈으로 「마디 130 · 상한 68」을 사전 등록한다. 게다가 접기를 빼고 재어도 도달 수는 **69** 이지 68 이 아니다(내 분할: enum_item 23 · function_item 23 · const_item 12 · struct_item 5 · mod_item 4 · trait_item 2 = 69, 나머지 61 = use 21·let 10·enum_variant 8·expression_statement 7·field_declaration 5·기타 7 + 좌표없음 3 — **「나머지 61」은 정확히 맞는다**). 즉 계획이 스스로 등록한 금지역(*"낮은 수를 설명하려고 표본을 손보고 싶어지는 자리"*)에 **착수 전부터 들어가 있다.**
- 어디가 걸리나: 계획 §「완수 조건으로 전환」 · `crates/pal-extract/src/parse.rs:513-560` `marked_comments`/`모은다` · `corpus/criteria.toml` 새 절(아직 없음)
- 획득: 조회 + 실측 — ③ 규칙(`line_comment` + `doc_comment` 자식 + 빈 줄 없는 연속)을 그대로 구현해 117 파일 전수 재계산
- 모집단: 자기장치
- 유효성: 참
- 해악도: 실패
- 대상: 계획자신
- 얼마나 아픈가: 착수 전에 수를 고치면 무료. 놔두면 **측정 뒤에 수를 고치는 것 말고 길이 없다**

### ② 는 아무것도 안 한다 — `;` 반복 구분자는 구문 트리에 **마디가 없다**
- 어떻게 실패하나: 격리 사본에서 CST 를 통째로 찍었다. `macro_rules! m { ($($x:expr);*) => { $($x);* }; }` 의 `token_repetition_pattern [18..30]` 자식은 `$` `(` `token_binding_pattern` `)` `*` 뿐이고 **바이트 28 의 `;` 에 대응하는 마디가 없다**(문법 `grammar.js:209` 의 `optional(/[^+*?]+/)` 가 숨은 토큰이다). 그래서 `;` 를 **전혀 안 지워도** `$($x);*` ≡ `$($x),*` ≡ `$($x)*` 가 바이트로 같다. ② 처방(`;` 조건을 `token_repetition_pattern` 에만)은 **살릴 `;` 마디가 없어** 산출을 한 바이트도 못 움직인다. 덤으로 그 노드는 **자기 저장소에 0 개**다(cargo 에 35). 계획이 근거로 든 *"자기 저장소 1 건"* 은 이 노드로 설명되지 않는다.
- 어디가 걸리나: `crates/pal-extract/src/parse.rs:181` `if kind == ";"` · `parse.rs:117-121` 근거 표
- 획득: 조회 + 실측 — CST 덤프 · `token_repetition_pattern` 노드 수 전수(palimpsest 0 · cargo 35)
- 모집단: 자기장치
- 유효성: 참
- 해악도: 거짓신호 — 게이트에 「충돌을 닫았다」가 실리지만 닫힌 것이 없다(죽은 가지 하나가 새로 생긴다)
- 대상: 계획자신
- 얼마나 아픈가: 되돌리기 쉽다. 그러나 **실제 1 건이 무엇인지 다시 재야 한다** — 재기 전에는 규칙을 못 정한다

### ④ 의 처방이 ④ 가 인용한 충돌을 못 닫는다 — `span` 은 `body_digest` 의 성분이 아니다
- 어떻게 실패하나: ④ 의 근거는 *"`#[derive(Debug)] struct S` ↔ `#[derive(Debug, Clone, Serialize)] struct S` 의 정규형이 바이트로 같다"* 이고 그것은 참이다(실측: 두 소스의 `struct_item` 정규형이 동일). 그런데 처방은 **`span.byte_start` 를 넓히는 것**이다. `body_digest` 는 `digest_of(node)`(`typescript.rs:482-484`) 가 **마디**를 정규화해 만들고 `span_of(node)`(`:569`) 와 **서로 독립**이다. `attribute_item` 은 선언 마디의 **형제**라 둘을 함께 덮는 마디가 없으므로(CST 확인), span 을 넓혀도 정규형은 그대로다. → `#[cfg(unix)]`↔`#[cfg(windows)]` · `#[serde(rename)]` 값 변경 · `#[must_use]` 유무가 **여전히 낡음을 안 켠다.** 닫으려면 `normalize` 가 **마디 여럿을 받는 새 진입점**을 가져야 하는데 그것은 만들 계획에 없다.
- 어디가 걸리나: `crates/pal-extract/src/typescript.rs:482`·`491`·`569` · `crates/pal-extract/src/parse.rs:126-176` `normalize`/`normalize_erasing`
- 획득: 조회 + 실측 — 두 소스의 `struct_item` 정규형 바이트 대조 · `digest_of`/`span_of` 호출 경로 확인
- 모집단: 자기장치
- 유효성: 참
- 해악도: 금지역 — 게이트에 *"속성 축을 닫았다"* 가 사실이 아닌 채로 실린다
- 대상: 계획자신
- 얼마나 아픈가: `parse.rs` 의 공개 API 를 하나 늘려야 한다(TS·Kotlin 도 지나는 자리). ①③ 과 같은 파일이라 회귀 표면이 겹친다

### ⑩ 의 「절대 요약 고정 시험」이 ③④⑤⑥ 을 **원리상 못 본다**
- 어떻게 실패하나: ⑩ 은 *"이것이 위 ①②③④ 전부의 회로가 된다"* 라고 적는다. 그런데 고정되는 값은 `body_digest` 다. ③ 이 바꾸는 것은 `marked_comments` 의 **개수**이고, 그것이 요약을 안 건드린다는 것은 이 저장소가 이미 시험으로 못 박았다(`parse.rs:770-779` `주석_수집이_요약을_안_건드린다`). ④ 가 바꾸는 것은 `span`, ⑤⑥ 이 바꾸는 것은 `SymbolId` 다 — 셋 다 `body_digest` 밖이다. 실제로 ⑩ 이 덮는 것은 ① 하나뿐이고 ② 는 무효다. 반대로 이미 있는 `corpus/golden/{ditto,portal-backend}.symbols.tsv` 는 `symbol_id`·`body_digest`·`kind`·`identity` 를 **줄마다** 담고 있어 ③ 을 뺀 나머지를 다 본다 — 다만 `scripts/f03-3-verify.py` 로만 돌고 **CI 에 없다.** 즉 진짜 공백은 「장치가 없다」가 아니라 「있는 장치가 CI 밖이다」다.
- 어디가 걸리나: `crates/pal-extract/tests/normalize_props.rs`(TypeScript 전용 · Kotlin 0 건) · `corpus/golden/ditto.symbols.tsv`(4,578) · `corpus/golden/portal-backend.symbols.tsv`(1,340) · `scripts/f03-3-verify.py:42-44` · `.github/workflows/ci.yml`
- 획득: 조회 — 골든 헤더 확인 · CI 잡 전수 확인
- 모집단: 자기장치
- 유효성: 참
- 해악도: 거짓신호 — 「등록한 금지역에 회로를 달았다」가 절반만 참인 채로 판정에 실린다
- 대상: 계획자신
- 얼마나 아픈가: 되돌리기 쉽다. ③ 용 회로는 **따로** 필요하다(표식 수 331 고정 같은 것)

### ⑤ 가 `container_chains` 에서 설 수 없다 — `Symbol` 에 `trait` 를 실을 자리가 없다
- 어떻게 실패하나: `container_chains`(`crates/pal-cli/src/ledger.rs:269`)가 받는 것은 `&[pal_core::Symbol]` 이고 그 타입에는 `name`·`kind`·`span`·`body`·`identity` 뿐이다(`crates/pal-core/src/symbol.rs:70-98`). 체인 성분은 **부모 심볼의 `name` 문자열 그대로**다(골든의 `container` 열이 그 증거). 따라서 `trait::Type` 을 얻는 길은 둘뿐이다 — ㉮ 추출기가 `impl` 을 **심볼로 내고 그 `name` 을 `From<A>::Error` 로 짓는다**(그러면 `container_chains` 는 손댈 것이 없고, 대신 `impl` 이 `pal symbols`·`자리` 맵·선언 수·`Containment` 에 전부 들어와 2,778 이라는 기준선이 움직인다), ㉯ `Symbol` 에 새 `pub` 필드를 단다(그러면 `schema/graph.toml:45-67` 과 `docs/graph-schema.md` 가 함께 움직이고 `check_schema` 방향 3 이 빨개진다 — 만들 계획에 그 둘이 없다). 계획은 ㉯ 를 적었지만 그것을 지탱할 데이터 경로를 안 만든다. 그리고 어느 쪽이든 **잔차가 안 적혀 있다** — `impl<T: Display> From<T> for E` ↔ `impl<T: Debug> From<T> for E` 는 여전히 같은 체인이다.
- 어디가 걸리나: `crates/pal-cli/src/ledger.rs:269-287`·`307-338` · `crates/pal-core/src/symbol.rs:70-98` · `crates/pal-core/src/coord.rs:100-125` `SymbolId::compute` · `schema/graph.toml:45-67`
- 획득: 조회 — 함수 시그니처·타입 필드·스키마 속성 목록 · 골든 `container` 열 실물
- 모집단: 저장소
- 유효성: 참
- 해악도: 실패 — ㉯ 로 가면 `cargo xtask check` 의 「스키마 정합」이 빨개진다. ㉮ 로 가면 안 빨개지지만 **선언 수 기준선이 조용히 움직인다**
- 대상: 계획대상
- 얼마나 아픈가: 초석이다(`SymbolId` 성분). 되돌리려면 1층 캐시 전량 무효화 + 좌표 이동

### ⑦ 이 코드 경로 없이 문서 표에만 산다 — `identity_ceiling` 은 `min` 에 삼켜져 되찾을 수 없다
- 어떻게 실패하나: `identity_ceiling()` 은 `ordinal == 0 ? Exact : Ordinal`(`crates/pal-core/src/coord.rs:259-265`)이고 `nodes_of` 는 `discriminator.identity_ceiling().min(s.identity)`(`crates/pal-cli/src/ledger.rs:334`) 만 남기고 **ceiling 자체를 버린다.** Rust 는 L1 → `ExtractGrade::L1.identity() == Ordinal`(`crates/pal-core/src/ledger.rs:79`)이라 **모든** Rust 심볼이 `ordinal` 이 된다. 즉 「순서에 취약한 464」와 「그냥 L1 인 7,156」은 `SymbolNode.identity` 로 **되찾을 수 없다.** ⑦ 을 판정 표에 실으려면 그 수를 내는 산출 경로를 새로 만들어야 하는데 만들 계획에 없다 — 손으로 센 수가 표에 들어가면 다음 회차에 검증 불가한 수가 하나 는다.
- 어디가 걸리나: `crates/pal-cli/src/ledger.rs:334` · `crates/pal-core/src/coord.rs:259-265` · `crates/pal-core/src/ledger.rs:76-82`
- 획득: 조회 — 세 자리의 실제 코드
- 모집단: 저장소
- 유효성: 참
- 해악도: 거짓신호
- 대상: 계획대상
- 얼마나 아픈가: 작다. 그러나 「⑦ 을 실었다」가 판정에 들어가면 다음 회차가 그 수를 못 되짚는다

### 코드 doc 주석 **아홉** 자리가 「넷」을 현재형으로 말한 채 남는다 (계획은 「다섯」이라 적었다)
- 어떻게 실패하나: 계획은 *"코드 다섯 자리가 새 문서를 가리키게 고친다"* 라고 적는데, 실제로 「1급 언어는 넷」을 현재형으로 말하는 `.rs` 자리는 아홉이다: `pal-core/src/language.rs:1`·`:7`, `pal-core/src/capable.rs:12`, `pal-extract/src/shell.rs:33`·`:129`, `pal-extract/src/extractor.rs:88`, `pal-extract/src/recognize.rs:35`·`:73`, `pal-cli/src/main.rs:550`. 그중 「지시 2026-08-12 §1」을 인용하는 것은 셋뿐이라 「다섯」은 어느 셈으로도 안 맞는다. 게다가 `language.rs:11` 이 *"`.svelte` 는 **다섯째 언어가 아니다**"* 라고 적어 두었는데 Rust 가 다섯째가 되면 그 문장이 오독을 부른다. `check_stale_citation` 은 **사라진 문서**만 보고 「수」는 안 본다 — 잡는 검사가 없다.
- 어디가 걸리나: 위 아홉 자리 · `xtask/src/main.rs:2788` `check_stale_citation`
- 획득: 조회 — `grep` 전수 후 손으로 분류
- 모집단: 저장소
- 유효성: 참
- 해악도: 거짓신호
- 대상: 계획대상
- 얼마나 아픈가: 되돌리기 쉽다. 아홉 곳

### ⑧ 이 `GRAMMAR_REV` 소비자 **셋 중 둘만** 든다 — `scripts/s0-compare.py` 가 빠졌다
- 어떻게 실패하나: ⑧ 은 `scripts/f01-verify.py:316` 과 `scripts/f04-verify.py:194` 를 든다. 그런데 `scripts/s0-compare.py:11` 이 *"정본은 둘이고 서로를 확인한다"* 라고 못 박고, `corpus/tasks/s0-reference-vector.tsv:2` 가 `# 문법 acb9630` 을 담는다. 합성으로 바꾸면 이 짝이 깨진다. ⚠ 더 나쁜 것은 **그 대조가 코드로는 구현돼 있지 않다**는 것이다(`s0-compare.py` 본문에 `grammar` 참조 0 건) — 그래서 깨져도 아무 검사도 안 울고, S0 재검증을 하는 사람이 손으로 대다가 **어긋남이 아닌 것을 어긋남으로 적는다.**
- 어디가 걸리나: `scripts/s0-compare.py:1-31` · `corpus/tasks/s0-reference-vector.tsv:2` · `crates/pal-extract/src/lib.rs:66` · `deny.toml:75`
- 획득: 조회 — `GRAMMAR_REV` 전수 grep + 두 스크립트 본문 확인
- 모집단: 저장소
- 유효성: 참
- 해악도: 거짓신호
- 대상: 계획대상
- 얼마나 아픈가: 작다. 한 줄 + 문서 한 문장

### cargo 380 모집단에 파싱 오류 6 파일 · ERROR 43 이 있는데 계획이 파싱 실패 축을 한 번도 안 적는다
- 어떻게 실패하나: `514c56d` 의 `git ls-files '*.rs'` = 1,372, `tests/testsuite/` 제외 = **380**(계획과 일치). 그 380 에 `77a3747` 문법으로 파싱 오류가 있는 파일이 **6** 개, ERROR 마디 **43** 개다(`src/util/frontmatter.rs` 19 · `cargo-util-schemas/src/core/partial_version.rs` 8 · `manifest/rust_version.rs` 7 · `tests/build-std/main.rs` 6 · `index.rs` 2 · `rustfix/tests/everything/handle-insert-only.rs` 1). `classify` 는 회복 지점이 있는 파일을 `partial` 로 보내므로 **「정확도」의 분모가 380 이 아니다.** 계획의 모집단 이름은 「`tests/testsuite/` 제외 380」 하나뿐이고 `partial` 을 어디에 셀지가 안 적혀 있다. (참고로 제외된 `tests/testsuite/` 992 파일 중 512 가 오류를 낸다 — 제외 결정 자체는 결과적으로 옳았지만 계획은 그 이유를 모른 채 제외했다.)
- 어디가 걸리나: `crates/pal-extract/src/classify.rs` · `crates/pal-extract/src/parse.rs:360-390` `recovery_sites` · 계획 §「결정된 것」 4
- 획득: 실측 — 380 파일 전수 파싱 후 `is_error()`/`is_missing()` 재귀 계수
- 모집단: 원의도
- 유효성: 참
- 해악도: 거짓신호 — 「정확도 N%」의 분모가 어디까지인지 판정 표가 못 말한다
- 대상: 계획대상
- 얼마나 아픈가: 착수 전에 모집단 정의에 한 줄 더하면 무료

### 초석 변경(⑤⑥)·두 언어 회귀 위험(①②③④)·골든 재베이스라인(⑧)이 **한 push** 에 들어간다
- 어떻게 실패하나: CI 는 `cancel-in-progress: true` 라 push 는 한 번이고, 잡은 7 개다(`check` ×3 · `produce` ×2 · `receive` ×2 — 실측). ⑧ 은 `f01-verify.py --bless`(`:316`)로 골든을 **통째로 다시 쓰는** 절차를 요구하는데, 같은 push 에 ①③④ 가 함께 들어 있으면 그 재축복이 **Kotlin·TS 산출 이동까지 흡수한다.** 계획이 스스로 그 위험을 적고 *"diff 가 `detector.grammar` 한 줄뿐임을 확인"* 을 처방하지만, 그 확인은 `f01-verify.py` 를 **로컬에서** 돌려야 하고 CI 에는 그 잡이 없다. 게다가 ⑤⑥ 은 `SymbolId::compute` 의 성분(`coord.rs:100-125`)이라 AGENTS.md 가 *"한 번에 제대로"* 로 분류한 초석이고, ①③④ 는 *"완성이 먼저"* 쪽이다. 두 규율이 한 커밋에서 충돌한다.
- 어디가 걸리나: `.github/workflows/ci.yml:47-52`(concurrency)·`:60-131`(check ×3)·`:153-218`(produce/receive) · `scripts/f01-verify.py:307-330` · `AGENTS.md` §「순서」
- 획득: 조회 — 워크플로 전문 · `--bless` 코드 경로 · 잡 수 계수
- 모집단: 규약
- 유효성: 참
- 해악도: 거짓신호 — CI 가 초록인데 그 초록이 「회귀 없음」이 아니라 「회귀를 흡수함」일 수 있다
- 대상: 계획자신
- 얼마나 아픈가: 갈라야 한다면 자연스러운 절단면은 **①③④ + 새 추출기 + 오라클**(회차 E) / **⑤⑥⑦ + ⑧**(다음 회차). ⑧ 은 ⑨(핀) 없이는 못 하고 ⑨ 는 추출기 없이 못 한다 — 즉 ⑧ 을 뒤로 미는 것이 유일하게 되돌릴 수 있는 절단이다

### ⑪ 의 「`FIRST_CLASS` 는 타입이 잡는다」가 반만 맞다 — 남는 것은 컴파일 오류가 아니라 **런타임 패닉**
- 어떻게 실패하나: `Language` 에 `Rust` 를 더하면 `crates/pal-extract/src/shell.rs:91` `index_of` 의 전수 `match` 가 컴파일 오류를 낸다 — 여기까지는 맞다. 그런데 자연스러운 최소 수정(`Language::Rust => 4`)을 하고 `FIRST_CLASS: [Language; 4]`(`:37`)와 `type Shells = [Capable<GraphShell>; 4]`(`:86`)를 그대로 두면 **컴파일이 통과하고** `shell_of(Language::Rust)` 가 `shells()[4]` 로 **인덱스 범위 초과 패닉**을 낸다. 즉 「타입이 잡는다」는 *편집을 강제한다*는 뜻일 뿐 *올바른 편집을 강제한다*는 뜻이 아니고, ⑪ 이 이 자리를 탐지 장치 목록에서 뺀 근거가 그 차이를 안 가른다. 그리고 `FIRST_CLASS` 를 안 늘리면 `capability_axis()`(`:142`, 캐시 키 성분)에 Rust 능력이 안 실려 ADR-0004 를 어긴다.
- 어디가 걸리나: `crates/pal-extract/src/shell.rs:37`·`:86`·`:91`·`:113`·`:120`·`:142-160`
- 획득: 조회 — 세 자리의 타입과 인덱싱 경로
- 모집단: 저장소
- 유효성: 참
- 해악도: 실패 — `pal ledger`·`pal cache` 가 패닉한다(시험이 즉시 잡을 가능성은 높다)
- 대상: 계획대상
- 얼마나 아픈가: 작다. 세 줄. 다만 ⑪ 의 「기각됨」 판단을 판정에 그대로 실으면 다음 사람이 이 자리를 안 본다

---

## 내가 기각한 것

| # | 제목 | 어떻게 실패한다고 봤나 | 어디가 걸리나 | 획득 | 모집단 | 유효성 | 해악도 | 대상 | 왜 기각했나 |
|---|---|---|---|---|---|---|---|---|---|
| 1 | ① `is_leading_separator` 삭제가 TS 산출을 움직인다 | 그 함수의 doc 이 *"ditto 실측 마지막 16 건"* 이라 적었으니 삭제하면 그 16 이 되살아난다 | `crates/pal-extract/src/parse.rs:214`·`:248` | 실측 | 자기장치 | 거짓 | 실패 | 계획자신 | ditto `.ts`/`.tsx` **544 파일 전수**에서 참 **0 건** · 삭제 전후 정규형 **바이트 동일**. `TRANSPARENT_IF_SINGLE` 가지가 언제나 먼저 `continue` 한다 |
| 2 | ① 삭제가 Kotlin 산출을 움직인다 | 계획이 TS 만 재고 Kotlin 을 안 쟀다 — 금지역 「두 언어 회귀」의 다른 쪽 | 같은 자리 · `brokk-tree-sitter-kotlin @ acb9630` | 실측 | 자기장치 | 거짓 | 실패 | 계획자신 | boxwood `.kt` **2,839 파일 전수**에서 참 **0 건** · 바이트 동일 |
| 3 | ⑨ 의 핀 `77a3747` 이 태그 객체이거나 v0.24.2 가 아니다 | 2 판이 `4f31efe`(v0.5.2 **태그 객체**)를 적었던 전례 | `Cargo.toml` 새 핀 | 조회 | 자기장치 | 거짓 | 실패 | 계획자신 | `77a3747…` = `refs/tags/v0.24.2` **직접 커밋** = `refs/heads/master` = `HEAD`. 계획의 정정이 맞다. tree-sitter 0.26 과 실제로 빌드·파싱됨 |
| 4 | tree-sitter-rust 가 `deny.toml` 라이선스 예외를 늘린다 | Kotlin 포크 때 `encoding_rs(BSD-3)` 가 물었던 전례 | `deny.toml:88-91` | 조회 | 저장소 | 거짓 | 실패 | 계획대상 | `license = "MIT"` · deps 는 `tree-sitter-language 0.1` + build-dep `cc 1.1` 뿐. `allow-git` 만 2→3 (계획과 일치) |
| 5 | 자기 저장소 `.rs` 117 파일에 파싱 실패가 있어 `parsed` 가 아니라 `partial` 로 간다 | edition 2024 · rustc 1.94 문법이 2026-03 문법을 넘어설 수 있다 | `crates/pal-extract/src/classify.rs` | 실측 | 원의도 | 거짓 | 실패 | 계획대상 | 117 파일 전수: **ERROR 0 · MISSING 0**. `pal ledger` 도 실제로 돌려 재현했다 |
| 6 | ⑧ 의 결박 좌표가 `#[cfg(test)]` 안에 몰려 완수 조건이 못 선다 | 선언의 22% 가 시험 안이라는 계획 자신의 수 | 계획 §「완수 조건으로 전환」 | 실측 | 자기장치 | 거짓 | 실패 | 계획자신 | 선언 마디에 도달하는 **69 중 `#[cfg(test)]` 안은 1 건**뿐. 이 조건은 여유롭게 선다 |
| 7 | ⑥ `SymbolKind` 확장이 `surface/queries.toml`·`check_schema`·골든을 움직인다 | 종류가 `SymbolId::compute` 의 성분이라 파급이 넓어 보인다 | `schema/graph.toml:60` · `surface/queries.toml` · `xtask/src/main.rs:953` | 조회 | 저장소 | 거짓 | 실패 | 계획대상 | `queries.toml` 에 `kind` **0 건**. 스키마는 `type = "enum:SymbolKind"` 로 **타입 이름만** 담는다. `SymbolId::compute` 는 `kind.name()` 문자열을 쓰므로 **뒤에 더하는 변형은 기존 이름을 안 움직인다** |
| 8 | `Cargo.lock` 을 안 담으면 CI 7 잡 중 4 가 빨개진다 | 계획의 서술 그대로 | `.github/workflows/ci.yml:178`·`:229` | 조회 | 자기장치 | 거짓 | 미관 | 계획자신 | `--locked` 는 `produce`(2) 와 `receive`(2) 에만 있고 `receive` 는 `needs: produce` 라 **실패가 아니라 스킵**된다. 실제로는 **2 실패 · 2 스킵 · 3 초록**. 처방 자체는 유효하고 근거 문장만 틀렸다 |
| 9 | `check_round_records` 가 `| # |` 표를 못 세어 기각 행이 0 이 된다 | 계획이 그 헤더를 못 박은 것 자체가 불안해 보였다 | `xtask/src/main.rs:3155-3163` | 조회 | 회차기록 | 거짓 | 실패 | 계획자신 | 코드가 이미 `| #`·`|-`·`| -`·`|:` 를 건너뛰고 나머지 `|` 행을 센다. ⚠ 다만 **데이터 행을 `| #1 |` 로 쓰면 헤더로 오인돼 안 세어진다** — 행 번호는 `| 1 |` 로 써야 한다 |
| 10 | ④ 의 span 확대가 TS·Kotlin 의 span 까지 움직인다 | `parse.rs:393` 에 공용 `span_of` 가 있다 | `crates/pal-extract/src/parse.rs:393` · `typescript.rs:569` | 조회 | 저장소 | 거짓 | 실패 | 계획대상 | 두 `span_of` 는 **다른 함수**다 — `parse.rs` 의 것은 `RecoverySite` 전용이고 심볼 span 은 추출기마다 자기 것을 쓴다. Rust 전용 확대가 구조적으로 가능하다 |

---

새 범주: **사전 등록된 수가 같은 계획의 다른 처방에 의해 무효화되는 자리** (③ 이 ⑧ 의 130·68 을 재기 전에 무너뜨리는 형태 — 「측정 전에 적는다」 규율이 **처방들 사이의 순서**를 안 재면 그 자리에서 깨진다)
