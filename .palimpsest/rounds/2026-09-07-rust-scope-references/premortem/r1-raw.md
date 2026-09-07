# 사전부검 — 라운드 1 원 반환문

> 낸 자: `pal-premortem-sweeper` · 2026-09-07 · 착수 커밋 `a774053`
> **안 받은 것**: 대화 기록 · 앞 라운드 결과 · 조건 설계 평가 산출물 · 이 회차의 `grammar-probe.md`.
> 받은 것은 원 의도와 계획, 그리고 저장소뿐이다.
> **아래는 그 에이전트가 낸 것을 한 글자도 안 고치고 옮긴 것이다.**

---

# 사전부검 라운드 1 — 이슈 #130 (Rust 스코프·임포트·익스포트)

격리 사본: `/tmp/pal-copy-20260907/`(`git archive HEAD`) · 문법 스파이크 `/tmp/rs-scope-spike-20260907/`(Cargo.lock 과 같은 `tree-sitter-rust` rev `77a3747`).

---

### `impl` 이 스코프를 안 열면 동명 선언 둘이 서로를 가리키는 가짜 엣지를 낳는다

- 어떻게 실패하나: 계획 A6 이 `impl` 을 스코프에서 빼면 `impl A { fn new() }` 와 `impl B { fn new() }` 의 `new` 두 개가 **같은 모듈 스코프**에 산다. `ScopeChain::resolve` 는 *"가장 앞선 것"* 을 돌려주므로(`scope.rs:221`) 둘째 `new` 의 **선언 이름 토큰**이 첫째 `new` 로 해소되고, `file_edges` 의 선언 자리 필터는 `bound.declared_at == r.at` 하나뿐이라(`projection.rs:238`) 그것을 못 거른다 → `new#2 → new#1` 엣지가 선다. 이 저장소 Rust 134 파일에서 **같은 모듈 스코프의 동명 재선언이 72 건 · 파일 22 개**(`crates/pal-core/src/chain.rs` 의 `new` 는 3 개). `pal touch new` 가 있지도 않은 호출자를 산출한다.
- 어디가 걸리나: `crates/pal-core/src/scope.rs:221-223` · `crates/pal-core/src/projection.rs:238-239` · 계획 A6
- 획득: **조회** — ① 격리 사본에 `crates/pal-core/tests/premortem_dup_decl.rs` 를 심어 실제 `pal_core::file_edges` + `ScopeChain::resolve` 로 돌렸다. 산출: `RefCounts{declarations:1, edges:1}` · `4b8081bf0dd2 -> cfb353269b7b`(둘째→첫째). ② 72 건은 문법 스파이크가 134 파일을 파싱해 센 것. `hoisted` 가 참이든 거짓이든 같은 결과다(`declared_at <= at` 팔로도 첫째가 이긴다).
- 모집단: 저장소
- 유효성: 참
- 해악도: **금지역** — 그래프에 사실이 아닌 관계가 저장되고 `pal touch` 가 그것을 호출자로 산출한다
- 대상: 계획대상
- 얼마나 아픈가: 되돌릴 수 있다(재구축). 걸리는 곳은 22 파일 · 72 자리이고, 고치려면 `impl` 스코프(#78, 계획이 명시적으로 분리한 것)를 다시 열거나 선언-자리 필터를 「같은 이름의 어떤 바인딩의 declared_at 과도 같으면」으로 넓혀야 한다 — 후자는 `pal-core` 를 만지는 것이라 계획 5(「위층은 안 만진다」)를 깬다

---

### 매크로 `token_tree` 안에는 `scoped_identifier` 도 `field_expression` 도 없다 — A4 와 A5 가 서로를 무효화한다

- 어떻게 실패하나: 계획의 표는 「`macro_invocation` 의 `token_tree` 안 `identifier`」를 세고 「`field_expression` 의 필드」와 「`scoped_identifier` 의 첫 세그먼트 아닌 것」은 안 센다고 적었다. 그런데 tree-sitter-rust 는 `token_tree` 안을 **구조 없이 토큰으로** 낸다: `assert!(x.foo().bar)` 는 `identifier "x"` · `. ` · `identifier "foo"` · `. ` · `identifier "bar"` 이고, `matches!(k, SymbolKind::Struct)` 는 `identifier "SymbolKind"` · `::` · `identifier "Struct"` 다. **`field_identifier` 도 `scoped_identifier` 도 생기지 않는다.** 즉 A5 의 시험(`x.foo()`)은 초록인데 매크로 안에서는 정확히 반대로 동작한다. 실측: 이 저장소의 `token_tree` 안 `identifier` **13,579 건**, 그중 `.` 바로 뒤(멤버 이름) **3,616 건** · `::` 바로 뒤(경로 꼬리) **1,138 건**. 그중 **같은 파일의 아이템 이름과 이름이 겹치는 것이 509 건 · 파일 47 개**(`is_empty`·`종류`·`상태`·`capabilities`…) — 이것이 그대로 가짜 REFERENCES 엣지가 된다.
- 어디가 걸리나: 계획의 「Rust 에서 TypeScript 와 갈리는 자리」 표 5·6 행 · A4 · A5 · `crates/pal-extract/src/scopes.rs:428` (`reference_namespace` 참조 구현)
- 획득: **조회** — 문법 스파이크로 AST 를 덤프하고(`spike`), 134 파일을 훑어 셋을 셌다(`mac`·`dotted`·`collide`)
- 모집단: 저장소
- 유효성: 참
- 해악도: **금지역** — 509 건이 「A 가 B 를 참조한다」는 거짓 사실로 2층에 앉는다
- 얼마나 아픈가: 되돌릴 수 있다. 다만 방어하려면 `token_tree` 안에서 **앞 형제 토큰(`.`·`::`)을 보고 거르는** 규칙이 필요하고 그 규칙이 계획 어디에도 없다. 그리고 남는 8,800 건(패턴 이름·클로저 파라미터·`macro_rules!` 틀 안의 이름 — `macro_definition` 의 `token_tree` 안 `fn generated` 의 `generated` 도 그냥 `identifier` 다)은 여전히 규칙이 없다
- 대상: 계획대상

---

### `REFERENCES ≥ 100` 이 아무것도 가르지 않는다 — 참조 후보가 76,416 이다

- 어떻게 실패하나: E1 이 이미 물은 것에 수를 댄다. 이 저장소 Rust 134 파일의 `identifier + type_identifier` 는 **76,416 건**이다(비교: F02-3 게이트가 적은 ditto TypeScript 496 파일의 파일 안 이름 참조가 76,048 건 — 파일당 4 배 밀도). 위 두 시나리오의 가짜 엣지만으로도 **72 + 509 = 581 건**이라 B1 은 **스코프 해소가 통째로 틀려도 통과한다.** 임계 100 은 실측 분포와 두 자릿수 어긋나 있고, 음성 대조(「`not_built` 로 되돌리면 3 으로 돌아간다」)는 *능력 자리를 껐는가*만 재지 *해소가 옳은가*는 안 잰다.
- 어디가 걸리나: 완수 조건 B1 · E1 · 음성 대조 절 1 행
- 획득: **조회** — 스파이크로 `identifier`(68,897) · `type_identifier`(7,519) 를 셌다. 「581 건이 전부 가짜여도 통과」는 위 두 시나리오의 실측값을 더한 것
- 모집단: 자기장치
- 유효성: 참
- 해악도: **금지역** — 완수의 증인이 죽은 가지다. 통과가 「Rust 134 파일이 참조 그래프에 기여한다」를 뜻하지 않는다
- 대상: 계획자신
- 얼마나 아픈가: 지금 고치면 싸다(임계를 「해소 정확도 표본」이나 「가짜 엣지 0 건인 부분집합」으로 바꾸는 것). 회차가 끝난 뒤엔 「세 자리를 넘겼다」가 게이트에 사실로 적히고 그 문장을 되돌리는 비용이 붙는다

---

### Rust 를 지나는 통합 시험이 하나도 없다 — B1·B2 는 CI 에 안 실린다

- 어떻게 실패하나: `crates/pal-cli/tests/` 의 픽스처는 전부 `.ts` 와 `.kt` 다. `stitching.rs:39,49` 는 `a.ts`·`gamma.kt` 를 쓰고 `참조_수("gamma.kt")` 가 `NotBuilt` 임을 단언하며 **Rust 팔이 아예 없다.** `host_free.rs` 의 REFERENCES 대조도 `a.ts` 하나짜리 픽스처다. 그러므로 이 회차가 세우는 경로(추출 → 1층 캐시 → `stitch_of` → EDGE_IN/OUT → `pal touch` 호출자)는 **A1~A6 의 추출기 단위 시험까지만** 자동으로 관측되고, 그 위는 사람이 한 번 눈으로 본 B1·B2 뿐이다. 다음 회차가 Rust 스코프를 깨뜨려도 `cargo xtask check` 와 `cargo xtask test` 는 초록이다.
- 어디가 걸리나: `crates/pal-cli/tests/stitching.rs:39-121` · `crates/pal-cli/tests/host_free.rs:76-80` · 완수 조건 B1·B2·C3
- 획득: **조회** — `crates/pal-cli/tests` 전수 grep(`.rs` 픽스처 0 건) · 각 시험의 픽스처 생성부를 읽었다
- 모집단: 저장소
- 유효성: 참
- 해악도: **금지역** — 「검사가 있는데 그 경로를 안 지난다」. C3(`cargo xtask check` 전량 통과)이 이 회차의 산물에 대해 아무것도 안 잰다
- 대상: 계획자신 (완수 조건 B 의 증인 방식)
- 얼마나 아픈가: 지금은 `.rs` 픽스처 한 줄이면 닫힌다. 나중엔 「초록인데 기능이 없다」를 발견하는 비용이 붙는다

---

### `doctor` 가 REFERENCES 를 능력 부재로 선언한 **사유**가 이 회차에 무너지는데 계획 6 이 그것을 안 만진다

- 어떻게 실패하나: `doctor.rs:301` 이 `REFERENCES` 를 `F07/graph-view-stitched-nodes` 부재로 선언하고, 그 사유가 코드 주석에 이렇게 적혀 있다 — *"지금 참조 엣지는 전부 `scoped` 이고 … 불변식 넷(공통 넷 · 출처 동질성 · 등급 규칙 · 근거 규칙)이 구조상 어긋날 수 없고, 그것을 「실물에서 통과」로 세는 것은 **작아서 안 걸린 것**을 성해서 안 걸린 것으로 읽는 것이다."* 이 회차 뒤 REFERENCES 는 3 → 세 자리 이상이 되어 「작아서」가 거짓이 되는데, 계획 6 은 *"화면 표시와 능력 선언도 안 만진다"* 로 그 선언을 그대로 둔다. 결과: `pal export` 는 REFERENCES 를 수백~수천 건 내는데 `pal doctor --full` 은 그 엣지에 **불변식 넷을 한 번도 안 돌리고** 「이 빌드가 안 담는다」로 답한다. `scripts/check-round-doctor.mjs` 는 `coverage_gaps` 가 비었는지만 보므로 선언이 있는 한 CI 는 초록이다.
- 어디가 걸리나: `crates/pal-cli/src/doctor.rs:290-301` · `scripts/check-round-doctor.mjs:16-23` · 계획 6
- 획득: **조회** — `doctor.rs` 의 `coverage()` 와 CI 스크립트를 읽었다
- 모집단: 저장소
- 유효성: 참
- 해악도: **금지역** — 측정이 죽은 가지다. 새로 선 수백 건의 엣지가 어떤 불변식도 안 지난다
- 대상: 계획대상
- 얼마나 아픈가: 되돌릴 수 있다. 걸리는 곳은 한 줄(`.absent("REFERENCES", …)`)이지만 그것을 `holding` 으로 옮기면 뷰가 EDGE 테이블을 싣게 되고 그 순간 계획 6 의 「위층은 안 만진다」가 깨진다 — **계획이 스스로 만든 딜레마다**

---

### `L2 = 스코프 해소된 참조` 라는 정의와 `grade_of(Rust) = L1` 을 시험으로 잠그는 것이 정면충돌한다

- 어떻게 실패하나: `ExtractGrade` 의 정본 정의가 `L1 = 구조(선언·포함 관계)` · `L2 = 스코프 해소된 참조` 다(`crates/pal-core/src/ledger.rs:50-53`). 이 회차 뒤 Rust 는 스코프 해소된 참조를 산출한다. 그런데 C1 이 `grade_of(Rust) == L1` 을 **시험으로 잠근다.** 그러면 `pal ledger` 의 언어 능력 행이 「Rust: L1 · ordinal」로 나가고 같은 산출의 `[edge.REFERENCES] grade = "scoped"`(`schema/graph.toml:419`) 와 나란히 선다. 소유자의 결정(「엣지만 · L1 유지」)은 정당하지만, **그 어긋남을 어디에도 안 적는다** — D2 는 `println!` 포맷 문자열만 적는다.
- 어디가 걸리나: `crates/pal-core/src/ledger.rs:44-58` · `crates/pal-extract/src/classify.rs:207-238` · 완수 조건 C1 · D2
- 획득: **조회** — `ExtractGrade` 정의와 `grade_of` 주석을 읽었다
- 모집단: 규약
- 유효성: 참
- 해악도: **거짓신호** — 아무것도 안 깨지지만 대장을 읽는 사람이 「Rust 는 구조만 본다」로 읽는다
- 대상: 계획자신 (C1 이 이 회차가 만드는 장치다)
- 얼마나 아픈가: D 항목 하나(「L1 인데 스코프가 있다 — 왜, 그리고 언제 풀리나」)를 더하면 닫힌다. 안 더하면 `grade_of` 옆 주석과 대장 산출이 각각 다른 말을 하는 상태가 굳는다

---

### 「스코프를 안 만든다」고 적은 주석 넷이 계획 4·6 에 의해 그대로 남는다

- 어떻게 실패하나: 계획 4 는 *"`grade_of(Rust)` 는 안 만진다"*, 계획 3 은 `not_built` **호출 셋만** 바꾼다. 그러면 다음이 그대로 남아 거짓이 된다 — `rust.rs:1` *"Rust 선언 추출 — **중첩 순회 · 스코프 없음**"* · `rust.rs:298` *"**L1 이라 심볼 단위로도 `ordinal` 이다.** 스코프가 없으므로 어느 이름이 지역인지 모르고"* · `rust.rs:326` *"**스코프를 안 만든다** — L1 을 고른 것이 이 자리다"* · `classify.rs:226` *"순회는 `impl`·`mod` 안까지 들어가지만 **스코프 해소는 하지 않는다.**"* `cargo xtask check` 의 「사라진 문서를 현재형으로 안 부른다」는 **문서 링크**만 보므로 이 넷을 안 잡는다.
- 어디가 걸리나: `crates/pal-extract/src/rust.rs:1,298,326` · `crates/pal-extract/src/classify.rs:226` · `xtask/src/main.rs:639`(`check_stale_citation`)
- 획득: **조회** — 원문 인용은 위 좌표에서 그대로 옮겼고, `check_stale_citation` 의 모집단이 문서라는 것은 `check()` 목록과 함수 이름으로 확인했다(함수 본문은 안 읽었다 — 그 한 겹은 추정이다)
- 모집단: 저장소
- 유효성: 참
- 해악도: **거짓신호**
- 대상: 계획대상
- 얼마나 아픈가: 네 줄. 되돌리기 쉽다

---

### 매크로 **밖**에서도 경로 꼬리 6,961 건과 `use` 안 이름 2,601 건을 거르는 규칙이 계획에 없다

- 어떻게 실패하나: 계획의 표는 참조로 세는 것에 「`identifier`」와 「`scoped_identifier` 의 첫 세그먼트」를 **둘 다** 적었다. 그런데 꼬리 세그먼트도 `identifier` 노드다 — `Self::new` 는 `identifier "Self"` · `::` · `identifier "new"` 이고, `crate::a::B` 는 중첩 `scoped_identifier` 안에 `identifier "a"`·`identifier "B"` 를 담는다. 두 규칙은 양립 불가능하고 계획은 어느 쪽이 이기는지 안 정했다. 실측: 매크로 밖 꼬리 세그먼트 **6,961 건**. 그리고 `use` 선언 안의 `identifier` **2,601 건** — TypeScript 빌더는 `in_module_clause`(`scopes.rs:414`)로 이 자리를 명시적으로 뺐고 그 이유를 *"이 파일 어딘가의 같은 이름으로 해소되고, 실물에서 그것이 「선언 전 참조」 거짓 양성이 됐다"* 로 적어 두었는데, 계획의 Rust 표에는 대응하는 행이 없다. `mod tests { use super::*; }` 처럼 **`use` 가 심볼 `mod` 의 span 안에 있으면** 그 이름은 `top_level` 이 아니라 엣지의 출발점을 갖는다.
- 어디가 걸리나: 계획의 「갈리는 자리」 표 5·6 행 · `crates/pal-extract/src/scopes.rs:405-426`(참조 구현이 이미 답을 갖고 있다)
- 획득: **조회** — 문법 덤프로 노드 종류를 확인하고 스파이크(`tails`)로 134 파일을 셌다
- 모집단: 저장소
- 유효성: 참
- 해악도: **금지역** — `Self::new()` 의 `new` 가 파일 안 **첫** `new` 로 해소되므로(시나리오 1 과 같은 기계) 「그럴듯하지만 틀린」 엣지를 대량으로 만든다
- 대상: 계획대상
- 얼마나 아픈가: 되돌릴 수 있다. 규칙 두 줄이면 닫히지만 **어느 쪽이 이기는지 계획이 안 정해 놓아** 구현자가 임의로 고르고 A1~A6 이 그 선택을 검증하지 않는다

---

### 타입 이름은 `type_identifier` 로 선언되고 값 자리에서는 `identifier` 로 쓰인다 — 이름 공간을 한쪽만 두면 358 건이 조용히 안 잡힌다

- 어떻게 실패하나: `struct S(u8);` 의 이름은 `type_identifier` 인데 `S(1)` 의 `S` 와 단위 구조체 `T` 의 값 자리 쓰임은 `identifier` 다. `Namespace::Value`/`Type` 를 그대로 쓰면서 `struct_item`·`enum_item`·`union_item` 을 Type 에만 선언하면, 생성자 호출과 단위 구조체 값이 전부 `OutsideFile` 로 샌다. TypeScript 빌더는 이 자리를 알고 있어 클래스를 **두 공간에 다 넣는다**(`scopes.rs:264-268`, *"클래스는 두 이름 공간에 다 있다"*). 계획의 표에는 이름 공간 열이 아예 없다. 실측: 같은 파일의 타입 이름을 값 자리에서 쓰는 `identifier` **358 건**.
- 어디가 걸리나: 계획의 「갈리는 자리」 표(이름 공간 행 없음) · `crates/pal-core/src/scope.rs:236-244`(`Namespace` 가 TypeScript 의 둘로 정의돼 있다) · 완수 조건 A1~A6(이 경우를 안 잰다)
- 획득: **조회** — 문법 덤프 + 스파이크(`ctor`)로 358 건을 셌다
- 모집단: 저장소
- 유효성: 참
- 해악도: **거짓신호** — 안 깨진다. 리콜만 조용히 준다(엣지가 안 서는 것은 관측 불가능하다)
- 대상: 계획대상
- 얼마나 아픈가: 되돌릴 수 있다. 다만 `Namespace` 는 문서 주석이 *"TypeScript 의 두 이름 공간"* 이라고 못 박은 `pal-core` 타입이라, Rust 의 (값·타입·매크로·수명) 넷을 어떻게 접을지가 초석 성격이다

---

### `ExportSet` 의 「정렬·중복 제거」는 강제되지 않는 호출자 계약이고 A3 이 그것을 안 잰다

- 어떻게 실패하나: `ExportSet::digest` 는 `names` 를 **정렬하지 않는다**(`file_graph.rs:86-104`). 계약은 주석에만 있다 — *"**정렬·중복 제거된 뒤에 불러야 한다**"*. TypeScript 추출기는 그 계약을 명시적으로 지키고 이유까지 적는다(`typescript.rs:160-165`, *"소스 순서에 의존하면 … 포매터가 export 를 재배열할 때 산출이 움직인다 — R-05"*). 계획 2(「`pub` 를 `ExportSet` 으로 산출한다」)와 A3(「`pub fn f()` 를 담고 `fn g()` 는 안 담는다」)은 **정렬을 요구하지도 재지도 않는다.** 안 지키면 `pub fn` 두 개의 순서만 바꿔도 `export_digest` 가 움직이고, 그것이 R-05 무효화 전파의 입력이다.
- 어디가 걸리나: `crates/pal-core/src/file_graph.rs:76,84-104` · `crates/pal-extract/src/typescript.rs:158-165` · 완수 조건 A3
- 획득: **조회** — `ExportSet::digest` 본문에 정렬이 없음을 읽었고, TypeScript 쪽이 호출 전에 `sort_unstable`·`dedup` 하는 것을 확인했다
- 모집단: 저장소
- 유효성: 참
- 해악도: **거짓신호** — 지금은 `export_digest` 의 소비자가 없어(전수 grep: `pal-cli/src/ledger.rs:390` 에서 실어 나르기만 한다) 아무것도 안 깨진다. F07 이 그것을 읽는 순간 거짓 무효화가 된다
- 대상: 계획대상
- 얼마나 아픈가: 두 줄. 지금이 가장 싸다

---

### `stitch_of` 는 최상위 이름만 EXPORTS 로 만든다 — 중첩 `pub` 365 건이 digest 는 움직이고 엣지는 안 된다

- 어떻게 실패하나: `crates/pal-cli/src/ledger.rs:377` 이 `n.container.is_empty() && &n.name == name` 으로 거른다. Rust 에서 `pub` 은 `impl`·`mod` 안에도 산다. 실측: 이 저장소의 `pub` 아이템 **최상위 592 · 중첩 365(38%)**. A3 이 「`pub fn f()` 를 exports 에 담는다」만 요구하므로 구현자가 중첩된 것까지 담으면 그 365 개는 `ExportSet.names` 와 `export_digest` 에는 들어가고 `EXPORTS` 엣지로는 **조용히 사라진다.** 그리고 Rust 의 `pub` 은 애초에 「밖에서 보인다」가 아니다 — 비공개 `mod` 안의 `pub fn` 은 크레이트 밖에서 안 보인다. 즉 `ExportSet` 의 뜻이 언어마다 갈리는데 계획이 그 뜻을 안 정한다.
- 어디가 걸리나: `crates/pal-cli/src/ledger.rs:370-385` · 완수 조건 A3
- 획득: **조회** — `stitch_of` 의 필터를 읽었고 스파이크(`pubs`)로 592/365 를 셌다
- 모집단: 저장소
- 유효성: 참
- 해악도: **거짓신호**
- 대상: 계획대상
- 얼마나 아픈가: 되돌릴 수 있으나, `export_digest` 는 F07 이 읽을 값이라 뜻을 잘못 굳히면 그 위가 움직인다

---

### `rust_scopes.rs` 를 새로 쓰는 것 — `scopes.rs` 가 이미 같은 기계이고 이 저장소는 「두 벌이면 한쪽만 고치는 경로가 생긴다」를 명문화했다

- 어떻게 실패하나: 계획 1 이 **새 파일**을 세우고 범위 밖 절이 *"`scopes.rs` 를 언어 공용 모듈로 리팩터링하는 것"* 을 뺀다. 그러면 `declare_pass`/`reference_pass` 2 패스 · `scope_at` 을 `Node::id` 로 잡기 · `hoist_home` · `symbol_at` 을 시작 바이트로 잇기 — 이 뼈대가 **두 벌**이 된다. 그리고 TypeScript 판이 실물에서 사서 넣은 방어 넷이 새 판에는 없다: ① *"주석은 파라미터가 아니다 … ditto 실측에서 그런 바인딩이 **32 개**(파일 5)"* ② `in_module_clause` ③ *"선언의 자리는 이름 토큰의 자리다"* ④ `protected`. 이 저장소의 `slot.rs` 모듈 주석이 같은 상황에서 내린 판단이 정본이다 — *"두 벌을 만들거나 여기로 오거나 둘 중 하나이고, **두 벌이면 한쪽만 고치는 경로가 생긴다**"* — 그리고 [ADR-0024](docs/adr/0024-an-adapter-that-can-diverge-is-a-second-core.md) 가 같은 축이다.
- 어디가 걸리나: `crates/pal-extract/src/scopes.rs`(434 줄, 전체) · `crates/pal-core/src/slot.rs:16-21` · 계획 1 · 범위 밖 절
- 획득: **조회** — `scopes.rs` 전문을 읽고 방어 넷의 주석을 확인했다. *"더 작은 표면으로 같은 답이 나오는가"* — 갈리는 것은 **노드 종류 표 여섯 줄**(스코프를 여는 것 · 참조로 세는 것 · 안 세는 것 · 호이스팅 규칙)이고 뼈대는 같다. 그것이 「표로 뽑고 뼈대를 공유한다」로 되는지는 **추정**이다(리팩터링을 안 해 봤다)
- 모집단: 규약
- 유효성: 참 (뼈대가 같다는 사실 · 방어 넷의 존재는 조회) / 「표만 갈라도 된다」는 부분은 추정
- 해악도: **거짓신호** — 당장 안 깨진다. 위 시나리오 8·9 가 바로 「TypeScript 판이 이미 답을 갖고 있는데 Rust 판에 없는 것」의 실물 두 건이다
- 대상: 계획자신
- 얼마나 아픈가: 지금 정하면 파일 하나의 구조 결정이다. 두 벌이 선 뒤엔 Kotlin(#131)이 셋째 벌이 되고 그때 합치는 비용이 세 배다

---

### 「참조 엣지는 아직 없다」고 적힌 자리 둘이 이 회차 뒤 거짓이 되는데 갱신이 계획에 없다

- 어떻게 실패하나: `schema/graph.toml:209` — *"참조 엣지(`CALLS` · `REFERENCES`)는 옛 F02 §3.5 와 F07 이 만들고 **아직 없다**."* · `docs/plan/02-order.md:49` 의 `C2` 행 — *"심볼 3,100 에 참조 엣지 **3 개**, 전부 TypeScript 파일 하나 … Rust 134 파일 기여 **0**"*. 계획의 어느 항목도 이 둘을 갱신하지 않는다. `03-shortest-path.md:152` 는 *"`REFERENCES` 가 세 자리에 못 미치면 순서표 §1 의 `C2` 행을 갱신한다"* 로 **실패했을 때만** 갱신을 걸어 두었다 — 성공했을 때의 갱신이 없다.
- 어디가 걸리나: `schema/graph.toml:207-210` · `docs/plan/02-order.md:49` · `docs/plan/03-shortest-path.md:152`
- 획득: **조회** — 세 자리를 직접 읽었다
- 모집단: 저장소
- 유효성: 참
- 해악도: **거짓신호**
- 대상: 계획대상
- 얼마나 아픈가: 두 줄. 다만 `02-order.md` §1 은 이 저장소가 「지금 어디에 서 있는가」를 읽는 표라 낡으면 다음 회차의 착수 판단이 그 위에 선다

---

### 1층 캐시가 파일마다 수백~수천 개의 `LocalRef` 를 싣게 되고 그 비용을 아무도 안 잰다

- 어떻게 실패하나: `CachedGraph.scopes: Slot<ScopeChain>`(`cached.rs:66`)이 `refs: Vec<LocalRef>` 를 통째로 직렬화한다. 각 `LocalRef` 는 `String` 이름을 소유한다. 이 저장소의 참조 후보가 76,416 건이므로 Rust 캐시가 지금보다 크게 는다. `budget.rs` 를 훑었지만 캐시 항목 크기나 추출 시간의 상한은 없다.
- 어디가 걸리나: `crates/pal-extract/src/cached.rs:60-67` · `crates/pal-core/src/budget.rs`
- 획득: **추정** — 후보 수(76,416)만 실측이고 **실제 캐시 크기·시간은 못 쟀다**(전체 빌드를 안 돌렸다). `pal ledger` 를 이 저장소에 돌려 전후를 비교하지 않았다
- 모집단: 저장소
- 유효성: 추정
- 해악도: **미관**
- 대상: 계획대상
- 얼마나 아픈가: 되돌릴 수 있다. 걸리는 곳 하나

---

## 내가 기각한 것

| 무엇 | 왜 기각했나 |
|---|---|
| — | 아래 표가 진다 |

### `FileRow::restore` 의 껍데기 어긋남으로 2층 되읽기가 깨진다
- 어떻게 실패하나: Rust 만 `scopes` 를 만들고 Kotlin 은 안 만드는데 `FileRow::restore` 는 껍데기를 **필드당 하나씩** 받으므로 언어가 섞인 저장소에서 `ShellMismatch` 가 난다 — 고 적었다.
- 어디가 걸리나: `crates/pal-core/src/projection.rs:150-162`
- 획득: **조회** — 전수 grep 결과 `FileRow::restore` 의 호출자는 같은 파일의 시험 둘(`projection.rs:320,334`)뿐이고 실운영 경로에 없다
- 모집단: 저장소
- 유효성: 거짓
- 해악도: 미관
- 대상: 계획대상
- 얼마나 아픈가: 해당 없음

### `export_digest` 가 Present 로 바뀌면서 낡음 전파가 결박 25 건을 stale 로 만든다 (C2 위반)
- 어떻게 실패하나: Rust 파일에 `export_digest` 가 처음 생기면 R-05 무효화가 돌아 결박이 움직인다 — 고 적었다.
- 어디가 걸리나: `crates/pal-core/src/projection.rs:115` · `crates/pal-core/src/cascade.rs`
- 획득: **조회** — `export_digest` 전수 grep 결과 소비자가 없다. `pal-cli/src/ledger.rs:390` 이 실어 나르고 `cascade.rs` 는 그 값을 안 본다. 주석도 *"쓰는 것은 F07"* 로 적는다
- 모집단: 저장소
- 유효성: 거짓
- 해악도: 미관
- 대상: 계획대상
- 얼마나 아픈가: 해당 없음

### `cargo xtask check` 의 「코어 어휘 금지」·「기계 토큰에 한국어 금지」가 새 파일을 잡는다
- 어떻게 실패하나: `rust_scopes.rs` 가 한국어 식별자(`순회`·`이름`)를 쓰면 CI 가 빨개진다 — 고 적었다.
- 어디가 걸리나: `xtask/src/main.rs:806-830` · `854-884`
- 획득: **조회** — 두 검사의 모집단이 각각 `crates/pal-core/src` 와 `pal-core`·`pal-query`·`pal-store` 셋이다. 새 파일은 `pal-extract` 라 **모집단 밖**이고, `rust.rs` 가 이미 한국어 식별자를 쓰고 있다
- 모집단: 저장소
- 유효성: 거짓
- 해악도: 미관
- 대상: 계획대상
- 얼마나 아픈가: 해당 없음

### 능력 축이 바뀌어 `shell.rs`·`cache.rs`·`host_free.rs` 의 시험이 빨개진다
- 어떻게 실패하나: `capability_axis()` 가 Rust 자리 넷을 `built` 로 바꾸므로 축을 단언하는 시험이 깨진다 — 고 적었다.
- 어디가 걸리나: `crates/pal-extract/src/shell.rs:174-208` · `crates/pal-store/src/cache.rs:674` · `crates/pal-cli/tests/host_free.rs:444-520`
- 획득: **조회** — 시험 본문을 읽었다. `shell.rs` 는 Kotlin(전부 부재)과 TypeScript(전부 있음)만 단언하고 Rust 팔이 없다. `cache.rs:674` 의 상수는 Kotlin 문자열이다. `host_free.rs` 는 `a.ts` 하나짜리 픽스처라 사유 갈래 둘(`not_built`·`not_stored`)이 그대로 성립한다
- 모집단: 저장소
- 유효성: 거짓
- 해악도: 미관
- 대상: 계획대상
- 얼마나 아픈가: 해당 없음

### C4(골든 `symbols.tsv` 대조)가 아무것도 안 재는 죽은 가지다
- 어떻게 실패하나: 골든 셋에 `.rs` 가 0 건이므로 회귀 방어가 이 회차의 산물을 못 본다 — 고 적었다. 확인해 보니 절반만 맞다: 골든에 `.rs` 는 실제로 **0 건**(`ditto.symbols.tsv` 4,578 줄 전부 `.ts` · `portal-backend.*` 는 `.kt`)이지만, 능력 축이 바뀌면 **캐시 키가 바뀌어 두 코퍼스가 전면 재추출된다.** 그 경로가 깨지면 골든이 움직이므로 C4 는 공짜 통과가 아니다 — 다만 **Rust 산출의 정확도에 대해서는 아무것도 안 말한다.**
- 어디가 걸리나: `corpus/golden/ditto.symbols.tsv` · `corpus/golden/portal-backend.symbols.tsv` · `scripts/f03-3-verify.py:40-45` · 완수 조건 C4
- 획득: **조회** — 골든 파일의 확장자 분포를 셌고(`.ts` 4,578 · `.rs` 0) `f03-3-verify.py` 의 코퍼스 둘을 읽었다
- 모집단: 저장소
- 유효성: 거짓 (「죽은 가지」로는 거짓 — 재는 것이 있다. 다만 **Rust 를 안 잰다**는 사실은 시나리오 4 가 진다)
- 해악도: 거짓신호
- 대상: 계획자신
- 얼마나 아픈가: 해당 없음

---

새 범주: **문법 표면의 비균질성** — 같은 구문이 매크로 안에서 구조를 잃어(`field_identifier`·`scoped_identifier` 가 사라져 전부 `identifier` 가 된다) 같은 규칙이 위치에 따라 반대로 동작하는 자리.
