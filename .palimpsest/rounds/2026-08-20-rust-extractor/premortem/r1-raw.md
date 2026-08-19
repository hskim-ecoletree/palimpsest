사전부검 라운드 1 — 저장소 `/Users/incognito/dev/projects/palimpsest`

실측 도구: `/tmp/rust-normalize-probe-20260820/` (격리 venv · `tree_sitter` 0.26 + `tree_sitter_rust`). `parse.rs` 의 `normalize_into`·`모은다`·`다음_선언`·`사이에_빈_줄` 과 `xtask` 의 `반환문_항_수` 를 **바이트 단위로 포팅**해 실제 저장소 117 개 `.rs` 와 `~/dev/projects/cargo`(rust-lang/cargo @ `514c56d`, 이미 클론돼 있다) 1,370 개 `.rs` 에 돌렸다.

---

### `&self` 와 `self` 가 같은 `body_digest` 를 갖는다 — R-22 충돌
- 어떻게 실패하나: `impl S { fn f(&self) -> u32 { self.x } }` 와 `impl S { fn f(self) -> u32 { self.x } }` 의 정규형이 **바이트로 같다**. `is_leading_separator` 가 `i==0` 인 리프 `&`·`|` 를 버리는데, tree-sitter-rust 의 `self_parameter: seq(optional('&'), optional(lifetime), optional(mutable_specifier), $.self)` 에서 `&` 가 정확히 그 자리다. `&mut self` ↔ `mut self` 도 같다. 즉 **빌림 메서드를 소비 메서드로 바꾸는 API 파괴가 결박을 `stale` 로 켜지 않는다.** 실측 산출: `b'\x1cfn\x1ff\x1f\x1c(\x1f\x1cself\x1f\x1a)…'` 가 두 소스에서 동일. palimpsest 자기 저장소에도, cargo 에도(`&self` 1,529 · `&mut self` 426 · `self` 138) 상시로 존재한다.
- 어디가 걸리나: `crates/pal-extract/src/parse.rs:248` `is_leading_separator` · `parse.rs:205` 의 `if is_leading_separator(&kids, i) { continue }`
- 획득: 조회 → **실측**. 상류 `grammar.js` 원문(`self_parameter`)과 `normalize_into` 를 포팅해 격리 사본에서 두 소스의 정규형 바이트를 비교. `*** SAME ***`
- 모집단: 저장소
- 유효성: 참
- 해악도: 금지역 (제품의 심장인 낡음 감지가 죽은 가지가 된다 · R-22 가 명시적으로 금한 형태)
- 대상: 계획대상
- 얼마나 아픈가: 되돌릴 수 있다(규칙에 언어 조건을 달면 된다). 걸리는 곳은 한 함수지만, 고치면 **TypeScript 의 `type X = | A | B` 정규화가 함께 움직이는지**를 다시 재야 한다 — ditto 골든 4,578 줄이 그 대조다.

### `;` 를 무조건 버리는 규칙이 `macro_rules!` 반복 분리자를 지운다
- 어떻게 실패하나: `macro_rules! m { ($($x:expr);*) => {…} }` 와 `macro_rules! m { ($($x:expr)*) => {…} }` 의 정규형이 **바이트로 같다**. 둘은 서로 다른 매크로다(하나는 인자 사이에 `;` 를 요구한다). `normalize_into` 의 `if kind == ";" { return }` 은 언어를 안 본다. 그 규칙의 근거로 적힌 문장이 `parse.rs:121` — *"선택적 세미콜론 | Kotlin 에서 스타일이고, TypeScript 에서도 ASI 가 있어 스타일이다"* — 이고, **Rust 에서는 참이 아니다.** 다섯째 언어가 들어오는 순간 그 표는 사실이 아닌 것을 사실로 적는다. cargo 에 `macro_rules!` 62 건, 이 저장소에 1 건.
- 어디가 걸리나: `crates/pal-extract/src/parse.rs:181` (`if kind == ";"`) · 근거 표는 `parse.rs:117-121`
- 획득: 조회 → **실측**. 포팅한 정규화로 두 매크로 정의를 비교. `*** SAME ***`
- 모집단: 저장소
- 유효성: 참
- 해악도: 금지역 (충돌 + 「사실이 아닌 것을 사실로 적음」이 겹친다)
- 얼마나 아픈가: 되돌릴 수 있다. 다만 `;` 규칙에 언어 조건을 달면 `parse.rs` 모듈 주석의 *"언어마다 따로 쓰지 않는다"* 라는 설계 문장이 처음으로 깨진다 — 그 문장을 어떻게 고칠지가 함께 걸린다.
- 대상: 계획대상

### `attribute_item` 이 주석-선언 인접을 끊는다 — 표식 130 중 **49** 가 좌표를 못 얻는다
- 어떻게 실패하나: tree-sitter-rust 에서 `#[must_use]` 는 `function_item` 의 **형제**이지 감싸는 마디가 아니다(실측 트리: `line_comment[0..33]` · `attribute_item[33..44]` · `function_item[45..58]`). `다음_선언` 은 첫 비-주석 형제를 잡아 `벗긴다` 를 태우는데, `벗긴다` 는 **포함 관계**를 벗기는 함수다 — `attribute_item` 을 래퍼 목록에 넣으면 그 **첫 이름 있는 자식**인 `attribute`(바이트 35)로 내려가고, 심볼이 서는 자리(45)와 여전히 다르다. 계획이 적은 처방(*"벗길 래퍼 목록을 Rust 용으로 정한다"*)은 **구조상 이 문제를 못 푼다.** 결과는 #62 가 TypeScript 에서 고친 것과 같은 실패(못 붙은 307 중 211)의 재발이다.
- 실측 분포(저장소 117 개 `.rs` · 표식 `["@decision:", "ADR-"]`): 선언에 붙는다 **45** · `attribute_item` 에 붙는다 **49** · 선언이 아닌 마디(`field_identifier`·`let_declaration`·`expression_statement`)에 붙는다 **33** · 없음 3. 즉 계획의 ⑨(*"이 회차의 ADR 을 실제로 결박해 본다"*)가 성립할 후보가 절반 이하다.
- 어디가 걸리나: `crates/pal-extract/src/parse.rs:563` `다음_선언` · `parse.rs:614` `벗긴다` · 계획 §추출기 셋째 항
- 획득: 조회 → **실측**. `모은다`/`다음_선언`/`사이에_빈_줄` 을 포팅해 117 파일 전수 실행.
- 모집단: 저장소
- 유효성: 참
- 해악도: 금지역 (계획의 종료 조건 ⑦·효과 ⑨ 가 이 경로 위에 서 있고, 안 고치면 「결박이 섰다」가 절반의 표본 위에서만 참이 된다)
- 대상: 계획대상
- 얼마나 아픈가: 되돌릴 수 있으나 **`벗긴다` 의 계약을 바꾸는 일**이다(포함 벗기기 → 앞 형제 건너뛰기). 그 함수는 Kotlin·TypeScript 도 탄다.

### `///` 는 줄마다 별개 노드다 — 조각 수가 다른 언어와 비교 불가가 된다
- 어떻게 실패하나: tree-sitter-rust 는 연속한 `///` 세 줄을 `line_comment` **세 개**로 낸다(실측). TypeScript 의 `/** … */` 는 하나다. 그래서 ADR 하나를 세 줄에 걸쳐 인용한 doc 주석은 **조각 3 개**가 되고, 같은 심볼에 같은 뜻의 결박이 3 건 생긴다. 계획의 RED 가 적은 「99 건」과 `marked_comments` 가 실제로 낼 수 **130 건**이 갈리며, ⑧(결박 ≥ N)·⑨(결박 목록)의 수가 언어에 따라 다른 자로 세어진다. 링크 참조 정의 줄(`/// [ADR-0023]: ../../..`)까지 표식으로 잡힌다.
- 어디가 걸리나: `crates/pal-extract/src/parse.rs:526` `모은다`(줄 단위 `line_comment` 를 각각 push) · 계획 §착수 시점 실측의 「99건」
- 획득: 조회 → **실측**. 다중 `///` 트리 덤프 + 전수 카운트(130).
- 모집단: 저장소
- 유효성: 참
- 해악도: 거짓신호 (수가 부풀고, 그 수가 종료 조건의 근거가 된다)
- 대상: 계획대상
- 얼마나 아픈가: 쉽다(인접한 `line_comment` 를 하나로 접는다). 그러나 접는 순간 `attaches_to_byte`·`span` 의 정의가 바뀌므로 계획 착수 **전에** 정해야 한다.

### `GRAMMAR_REV` 이 문법 **하나**뿐이라 Rust 핀을 옮겨도 캐시가 안 무효화된다
- 어떻게 실패하나: `GRAMMAR_REV` 는 Kotlin 의 rev 문자열 **한 개**다. TypeScript 의 핀(`75b3874`)은 이미 캐시 키에 없다. 그 자리의 doc 주석은 *"어기는 방향은 덜 무효화하는 쪽이고 지금은 더 무효화한다"* 라고 적는데, 그 문장은 **Kotlin 쪽 변경에 대해서만** 참이다 — TypeScript(그리고 앞으로 Rust) 문법 핀을 옮기면 키가 안 움직여 **옛 문법으로 계산한 항목을 새 문법 빌드가 그대로 되읽는다.** ADR-0004 가 금지한 「덜 무효화」다. 계획이 *"tree-sitter-rust 를 커밋 rev 로 핀(G50 이 Kotlin 에 한 형태와 같게)"* 라 적었지만, deny.toml:75 가 주장하는 *"핀이 곧 `ExtractorVersion` 의 문법 축"* 은 코드에서 **거짓**이다. 덤으로 `pal ledger` 의 `DetectorFreshness.grammar` 가 Rust 산출에 대해 Kotlin 의 rev 를 적는다.
- 어디가 걸리나: `crates/pal-extract/src/lib.rs:66` `GRAMMAR_REV` (근거 주석 `lib.rs:40-65`) · `deny.toml:75` · `corpus/golden/portal-backend.ledger.json` 의 `detector.grammar`
- 획득: 조회 (`grep -rn "75b3874\|acb96307"` → 코드에는 Kotlin rev 하나뿐 · `version()` 이 그것만 싣는다)
- 모집단: 저장소
- 유효성: 참
- 해악도: 금지역 (캐시가 틀린 좌표를 조용히 돌려준다 · 문서가 사실이 아닌 것을 사실로 적는다)
- 대상: 계획대상
- 얼마나 아픈가: 지금 고치면 값 하나를 합성 문자열로 바꾸는 일이다. 안 고치고 셋째 문법을 얹으면 **되돌리기 비용이 3 배가 된다**(초석 판별식에 걸린다).

### 컴파일러가 안 잡는 「넷」이 셋 있다 — 그중 하나는 실제 분기다
- 어떻게 실패하나: `Language` 에 변형을 더해도 아래 셋은 **컴파일도 시험도 안 운다.**
  1. `language.rs:41` `from_name` 의 배열 `[Kotlin, Java, JavaScript, TypeScript]` — 안 늘리면 `.gitattributes` 의 `linguist-language=Rust` 가 `Recognition::Known("Rust")` 로 떨어져 **선언한 파일은 L0, 확장자로 잡힌 파일은 L2** 라는 갈림이 생긴다. `recognize.rs` 의 시험은 `kt`·`ts` 만 본다.
  2. `extractor.rs:89` `const 일급: [Language; 4]` — 주석이 *"표가 늘 때 이 배열이 함께 늘어야 아래 시험들이 전수가 된다"* 라 적었지만 **강제 장치가 없다.** 안 늘리면 `능력과_등급이_같은_표를_본다`·`추출기는_자기_언어를_말한다` 가 Rust 를 건너뛰고 초록이 된다.
  3. `crates/pal-cli/src/main.rs:550` — 사용자에게 나가는 문자열에 네 언어 이름이 손으로 박혀 있다. 어떤 시험도 이 문자열을 안 본다(`grep` 결과 참조자 0). AGENTS.md 의 *"표면을 더할 때 목록을 더하지 않는다"*(ADR-0024)를 이미 어기고 있는 자리다.
  (`shell.rs:37` `FIRST_CLASS` 는 `type Shells = [_; 4]` 가 타입으로 잡는다 — 여기만 안전하다.)
- 어디가 걸리나: `crates/pal-core/src/language.rs:41-44` · `crates/pal-extract/src/extractor.rs:89` · `crates/pal-cli/src/main.rs:550`
- 획득: 조회 (`grep -rn "Language::\|\[Language; \|FIRST_CLASS"` 전수 · 각 자리 정독)
- 모집단: 저장소
- 유효성: 참
- 해악도: 금지역 (① 은 같은 파일이 선언 유무에 따라 다른 등급을 받는다 = 측정이 갈린다. ②③ 은 거짓신호)
- 대상: 계획대상
- 얼마나 아픈가: 되돌리기 쉽다. 다만 **세 자리를 다 찾아야 한다**는 것이 요점이고, ②는 「검사가 초록인 채로 안 재는」 형태라 사후에 발견되지 않는다.

### Rust 의 shadowing 이 `resolve` 의 「가장 앞선 것」 팔을 상시 경로로 만든다
- 어떻게 실패하나: `ScopeChain::resolve` 는 *"같은 스코프에 같은 이름이 여럿이면 **가장 앞선 것**을 쓴다"* 라 문서화돼 있고 구현도 그렇다(`bindings` 를 앞에서부터 훑어 첫 일치를 반환). TypeScript 에서 같은 스코프의 `let` 재선언은 문법 오류라 이 팔은 사실상 도달하지 않는다. **Rust 에서는 `let x = 1; let x = f(x);` 가 관용이다.** 그러면 두 번째 `x` 이후의 참조가 **첫 번째 바인딩으로 해소되고**, `normalized_of` 는 그 잘못된 `(scope, binding)` 으로 자리 번호를 매겨 지운다. `grade_of_symbol` 은 `unnameable`·TDZ 만 보므로 「해소는 됐는데 틀렸다」를 **`Ordinal` 로 떨어뜨리지 않는다** — 즉 R-22 가 요구한 *"모르면 지우면 안 된다"* 의 전제가 깨진 채로 지우기가 켜진다.
- 어디가 걸리나: `crates/pal-core/src/scope.rs:221-247` `resolve` · `crates/pal-extract/src/typescript.rs:454` `grade_of_symbol` · 계획 결정 #2(TS 급)
- 획득: 조회 (두 함수 정독 · Rust 의 shadowing 규칙). **바이트 충돌은 못 돌렸다** — Rust 추출기가 아직 없어 실물 정규형을 못 냈다.
- 모집단: 저장소
- 유효성: 추정 (「팔이 상시 경로가 된다」는 참 · 「충돌이 실제로 난다」는 미확인)
- 해악도: 금지역
- 대상: 계획대상
- 얼마나 아픈가: `resolve` 는 `pal-core` 의 공용 API 이고 TypeScript 가 쓴다. 고치려면 **「마지막 일치」와 「첫 일치」를 언어가 고르게** 하거나 Rust 스코프 빌더가 shadowing 마다 새 스코프를 열어야 한다 — 둘 다 초석이다.

### `Namespace` 가 둘 · `ScopeKind` 가 TypeScript 모양 — Rust 를 담을 자리가 없다
- 어떻게 실패하나: `Namespace = {Value, Type}` 인데 Rust 의 이름 공간은 셋(값·타입·**매크로**)이고 lifetime 이 또 따로다. `macro_rules! foo` 와 `fn foo` 는 Rust 에서 공존하는데 한 공간으로 뭉개면 `Namespace` 의 doc 이 직접 경고한 *"둘 중 하나가 다른 하나를 가리고 … 틀린 해소는 틀린 정규화이고 틀린 정규화는 서로 다른 코드가 같은 digest"* 가 그대로 성립한다. `ScopeKind = {Module, Function, Class, Braced}` 이고 `Module` 의 doc 은 *"파일 하나 = 모듈 하나"* 라 **파일 안의 `mod`** 를 담을 이름이 없다(`Braced` 로 밀면 `mod` 의 하이스팅 규칙 — Rust 의 아이템은 전부 앞뒤 무관 — 을 `Braced` 가 표현 못 한다). 그리고 `pal-core` 의 금지어에 `block` 이 있어 이름 선택이 이미 좁다. 계획의 「코어」 절에는 `Language`·`grade_of`·`FIRST_CLASS`·`shell_of`·`capability_axis` 만 적혀 있고 **`scope.rs` 가 없다.**
- 어디가 걸리나: `crates/pal-core/src/scope.rs:42-65` (`ScopeKind`·`Namespace`) · `crates/pal-extract/src/scopes.rs:1` (모듈 주석이 *"TypeScript 파일 하나의"* 로 시작 · 434 줄 전부 TS 노드 이름) · 계획 §코어
- 획득: 조회 (`scope.rs` 정독 · `scopes.rs` 의 상수 목록 `FUNCTION_LIKE`/`CLASS_LIKE`/`BRACED`/`UNNAMEABLE_PATTERN` 전수 확인)
- 모집단: 저장소
- 유효성: 참
- 해악도: 실패 (계획 범위가 조용히 좁혀져 있다 — 코어 열거 두 개와 `scopes.rs` 규모 434 줄이 계획에 안 적혀 있다)
- 대상: 계획대상
- 얼마나 아픈가: `ScopeKind`·`Namespace` 는 `Serialize`+`Deserialize` 이고 1층 캐시·2층에 실린다. 변형을 더하면 되돌리기가 비싸다.

### ★ TS 급(L2)이 이 회차의 물음에 필요한가 — 더 작은 표면으로 같은 답이 나온다
- 어떻게 실패하나: 원 의도(#66)가 묻는 것은 *"이 도구가 자기 자신을 큐레이션할 수 있는가"* 이고, 종료 조건 ⑦은 *"결박이 실제로 서고 자기 저장소가 피드백을 낸다"* 다. **결박이 서는 데 필요한 것은 좌표(경로+컨테이너 체인+이름)이지 스코프 해소가 아니다.** L2 가 사는 이유는 `body_digest` 가 리네임에 안 흔들리는 것이고, 그것은 *낡음의 정밀도* 문제이지 *결박의 존재* 문제가 아니다. 계획 결정 #2 의 근거는 *"Kotlin 급 최상위-쿼리로는 impl/mod 안의 표식 25 건을 놓친다"* 인데, 그 25 건을 잡는 데 필요한 것은 **중첩 순회**이지 **스코프 체인**이 아니다. 계획이 「쿼리 vs 순회」와 「L1 vs L2」를 한 결정으로 접었다. 갈라 보면 **「중첩 순회 + L1」** 이라는 선택지가 있고, 그것은 위 세 시나리오(shadowing · `Namespace` · `&self` 지우기)의 위험을 **전부 회피한다** — `normalized_of` 는 `identity != Exact` 이면 지우기를 안 한다. 규모도 `scopes.rs`(434) + `typescript.rs` 의 스코프 절반이 빠진다.
- 어디가 걸리나: 계획 §소유자가 정한 것 #2 · `crates/pal-extract/src/classify.rs:207` `grade_of`(언어별 등급이 이미 갈려 있다 — L1 을 고를 자리가 코드에 준비돼 있다) · `crates/pal-extract/src/typescript.rs:522` `normalized_of` 의 첫 줄
- 획득: 조회 (`grade_of` 가 언어별로 갈린 것 · `normalized_of` 의 조기 반환 · `scopes.rs` 434 줄이 TS 전용인 것). **소유자 인터뷰 4 라운드의 맥락은 못 물었다** — 이 결정이 이미 재검토된 것일 수 있다.
- 모집단: 원의도
- 유효성: 추정
- 해악도: 거짓신호 (L2 라고 적고 실제로는 틀린 해소 위에서 지우면 등급이 능력을 과장한다)
- 대상: 계획대상
- 얼마나 아픈가: 지금 갈라 두면 무료. 나중에 L2→L1 로 내리면 **좌표가 전부 이동한다**(`EXTRACTOR_REV` 승급 + 전 결박 `stale`).

### 「기존 세 언어 회귀」— 이 빌드의 추출기는 **둘**뿐이다
- 어떻게 실패하나: 계획 #6(판정 모집단 셋)과 #8(금지역에 「기존 세 언어 회귀」)이 존재하지 않는 모집단을 가리킨다. `extractor_for` 는 Kotlin·TypeScript 만 `Present` 이고 Java·JavaScript 는 `Capable::not_built(F02)` 다. `pal-extract/Cargo.toml` 의 문법 의존도 둘뿐이다. 금지역이 「셋」이라 적히면 판정 시점에 **둘을 재고 셋을 재었다고 적거나**, 없는 하나를 「대조 불가」로 처리하며 그 사실이 판정 표에 안 남는다. 이 저장소의 규율(*"대조 불가는 1급 판정이다"* · criteria.toml:151)에 정면으로 걸린다.
- 어디가 걸리나: `crates/pal-extract/src/extractor.rs:73-81` · `crates/pal-extract/Cargo.toml:11-16` · 계획 #6·#8
- 획득: 조회 (`extractor_for` 정독 · Cargo.toml 의존 목록)
- 모집단: 자기장치
- 유효성: 참
- 해악도: 거짓신호 (등록된 금지역이 실측 불가능한 대상을 지목한다)
- 대상: 계획자신
- 얼마나 아픈가: 계획 문서 한 줄. 지금 안 고치면 게이트 문서에 그대로 옮겨 적히고, 그 뒤에는 「어긋남」으로 기록해야 한다.

### 오라클의 모집단을 이 회차가 직접 편집한다 — 손 표본의 순환
- 어떻게 실패하나: 계획 #5 는 *"손 표본이 정본(추출기 코드보다 먼저 커밋)"*, #8 은 *"손 표본을 추출 결과에 맞춰 고치는 것"* 을 금지역으로 등록한다. 그런데 이 회차가 표본으로 삼을 자기 저장소의 `.rs` 파일들은 **이 회차가 실제로 고치는 파일들**이다 — `language.rs`·`extractor.rs`·`shell.rs`·`classify.rs`·`lib.rs`·`parse.rs`. 표본이 그 파일의 선언 목록이라면, 추출기를 만드는 커밋마다 표본이 낡는다. 그리고 그 순간 **「코드가 바뀌어서 갱신」과 「추출 결과에 맞추려고 갱신」이 산출로 구별되지 않는다.** 금지역이 관측 불가능해진다. 코퍼스 매니페스트는 이미 이 문제의 답을 갖고 있다 — *"경로가 아니라 (remote, SHA)가 코퍼스의 정체다"*(manifest.toml `[corpus.pin]`) — 인데 계획은 자기 저장소 표본에 SHA 핀을 걸지 않는다.
- 어디가 걸리나: 계획 §오라클 첫 항 · 계획 #8 · `corpus/manifest.toml:88-108` `[corpus.pin]` (있는 처방)
- 획득: 조회 (계획이 손댈 파일 목록 ↔ 저장소 `.rs` 목록 대조 · manifest 의 핀 규약 정독). 실측: 저장소 117 파일 · 선언 2,803(최상위 1,681 · 중첩 1,122).
- 모집단: 자기장치
- 유효성: 참
- 해악도: 금지역 (등록된 금지역 자체가 측정 불가능해진다 = 측정이 죽은 가지)
- 대상: 계획자신
- 얼마나 아픈가: 착수 전에 「표본은 커밋 `<SHA>` 기준」한 줄이면 끝난다. 착수 후에는 어느 커밋을 기준으로 셌는지 복원이 안 된다.

### cargo 코퍼스에서 1,370 중 **518** 이 파싱 오류를 낸다 — 문법 rev 선정 기준이 계획에 없다
- 어떻게 실패하나: 계획 #4 는 *"외부 코퍼스는 `rust-lang/cargo` @ 고정 SHA"*, 계획 §코어는 *"커밋 rev 로 핀(G50 이 Kotlin 에 한 형태와 같게)"* 라 적는다. 그런데 **G50 의 「형태」는 「핀한다」가 아니라 「후보 포크들을 1,122 파일 전수로 재고 사전 등록된 축(상류 추종)으로 갈랐다」**이다. 계획에는 무슨 rev 를, 무슨 측정으로 고르는지가 없다. 실측(현재 master 기준): cargo 1,370 개 `.rs` 중 **518** 이 `ERROR`/`MISSING` 을 낸다. 그중 24 는 오류 비율 30% 초과라 `PROVISIONAL_ERROR_RATIO_PERCENT` 에 걸려 `unsupported{GrammarDefeated}` 가 되고, 494 는 `partial` 이 된다. `tests/`·fixture 밖에서도 4 건이 난다 — 전부 `snapbox` 의 `str![[r#"…"#]]` 구문이다(`src/util/frontmatter.rs:429` · `crates/cargo-util-schemas/src/index.rs:315` 등). 즉 **판정 모집단 ②(외부 cargo · 추출이 정확한가)의 36% 가 `partial` 로 나오고**, 그것이 문법 탓인지 코퍼스 탓인지 가르는 규칙이 계획에 없다. 참고: 이 저장소 자신의 117 파일은 오류 0 이므로 자기 저장소만으로는 이 축이 **아예 안 보인다.**
- 어디가 걸리나: 계획 §코어 첫 항 · 계획 #4 · `crates/pal-core/src/budget.rs:56` `PROVISIONAL_ERROR_RATIO_PERCENT = 30` · `docs/gates/G50-kotlin-grammar-pin.md`(따라야 할 형태)
- 획득: 조회 → **실측**. `~/dev/projects/cargo` @ `514c56d` 전수 파싱 후 `ERROR` 바이트 비율 계산.
- 모집단: 저장소
- 유효성: 참
- 해악도: 실패 (게이트가 「추출이 정확한가」를 못 판정하거나, 판정하면 그 수가 무엇을 뜻하는지 모른 채 적힌다)
- 대상: 계획대상
- 얼마나 아픈가: 문법 rev 는 좌표의 성분이다. 재보지 않고 고르면 나중에 바꿀 때 **전 좌표가 이동한다** — G50 이 정확히 그 비용을 치른 자리다.

### CI 에 코퍼스 잡이 없다 — 외부 판정은 이 기계에서만 선다
- 어떻게 실패하나: `.github/workflows/ci.yml` 의 잡은 7 개(check ×3 · produce ×2 · receive ×2)이고 도는 명령은 `cargo xtask check` · `cargo xtask test` · `interop-*.sh` 뿐이다. `scripts/f0*-verify.py` 는 **하나도 CI 에 없다.** 계획의 판정 모집단 셋 중 ②(외부 cargo)와 ③(기존 언어 회귀)은 전부 손으로 도는 스크립트에 의존하고, 그러면 「재현성」은 이 맥북 한 대의 상태(클론 존재 · rustc 1.94.1 고정 · rustup 부재)에 묶인다. 그 사실이 계획의 제약 절에 「이 PC 에 rustup 이 없다」로만 적혀 있고 **판정의 재현성 문제로는 안 적혀 있다.**
- 어디가 걸리나: `.github/workflows/ci.yml:70-140` · `scripts/` (verify 스크립트 24 개) · 계획 §이 저장소의 제약
- 획득: 조회 (워크플로 전문 정독 · `scripts/` 목록 · 참조자 `grep`)
- 모집단: 저장소
- 유효성: 참
- 해악도: 거짓신호 (게이트가 「통과」라 적히는데 그 통과를 재현할 기계가 하나뿐이다)
- 대상: 계획대상
- 얼마나 아픈가: 되돌릴 수 있다. 다만 cargo 코퍼스(3,071 파일 · `.git` 78MB)를 CI 에서 클론하면 세 OS × 매 커밋이라 비용이 실질이다 — **그러니 「CI 에 안 넣는다」를 판정으로 적고 그 사실을 게이트에 남기는 것**이 처분이지, 안 적는 것이 처분이 아니다.

### 사전부검 반환문의 기각 표 헤더가 `| #` 이 아니면 합계 검산이 +1 로 어긋난다
- 어떻게 실패하나: `반환문_항_수` 는 사전부검 반환문에서 `^### ` 항 + `## 내가 기각한 것` 아래의 최상위 항을 센다. 표 분기에서 **제외하는 접두는 `| #`·`|-`·`| -`·`|:` 뿐**이므로, 기각 표의 헤더 행이 `| 제목 | … |` 로 시작하면 **헤더가 한 항으로 세어진다.** 그러면 `findings.jsonl` 의 `(회차, 출처=사전부검, 라운드)` 행 수와 1 차이가 나고 `합계 검산 어긋남` 으로 **검사 20(회차 레코드)이 실패**한다 → 세 OS 전부 빨강. 이 함정은 이번 회차가 처음 밟는다 — 앞 회차 세 라운드의 기각 절은 전부 **불릿**이었고(`r1~r3-raw.md` 확인), 표 분기는 그 뒤에 붙었다. 그리고 사전부검자 정의(`.claude/agents/pal-premortem-sweeper.md:108`)는 *"표로 낸다"* 만 적고 **헤더 형태를 지정하지 않는다.**
- 어디가 걸리나: `xtask/src/main.rs:3156` (표 분기의 제외 조건) · `xtask/src/main.rs:3403-3418` (합계 검산) · `.claude/agents/pal-premortem-sweeper.md:104-111`
- 획득: 조회 → **실측**. `반환문_항_수` 를 그대로 포팅해 두 형태의 모의 반환문에 실행: 헤더 `| 제목` → **5**, 헤더 `| #` → **4**(정답 4).
- 모집단: 자기장치
- 유효성: 참
- 해악도: 실패 (CI 가 빨개진다. 초록으로 만드는 가장 쉬운 길이 「레코드에서 기각 행을 지우기」인데 그것이 #72 가 고치려던 병 그 자체다)
- 대상: 계획자신
- 얼마나 아픈가: 되돌리기 쉽다 — 에이전트 정의에 헤더를 `| # |` 로 못 박거나 검사에서 첫 데이터 행 앞의 헤더를 인식하면 된다. 걸리는 곳은 두 곳(정의 + 검사).

### 아직 없는 파일을 좌표로 적으면 회차 레코드 검사가 실패한다
- 어떻게 실패하나: 이 회차의 계획이 만들 것 — `crates/pal-extract/src/rust.rs` · `corpus/tasks/<name>-rust-recall-sample.tsv` · `docs/gates/<기능>.md` · 새 ADR — 은 사전부검 시점에 **존재하지 않는다.** 그런데 검사 20 의 ③ 은 레코드 각 행의 `경로` 를 `좌표가_실재하는가` 로 해소하고, 그 함수는 실재하는 파일(또는 접미 일치)만 참으로 낸다. 앞으로 만들 파일을 좌표로 적으면 그 행이 즉시 빨강이다. 유일한 탈출구는 `"(경로 없음)"` 인데, 그러면 **계획이 만들 장치에 대한 발견은 전부 좌표를 잃는다** — 계기판의 좌표 해소율이 「발견이 어디에 걸리는지」를 못 세게 된다. 앞 회차들의 사전부검은 **이미 있는 장치**를 물었기 때문에 이 축이 안 보였다.
- 어디가 걸리나: `xtask/src/main.rs:3364-3367` (③ 좌표 해소) · `xtask/src/main.rs:3474-3509` `좌표가_실재하는가` · 계획 §만들 계획 전부
- 획득: 조회 (`좌표가_실재하는가` 정독 — 접미 일치는 열려 있으나 **없는 파일은 어떤 경로로도 안 맞는다**)
- 모집단: 자기장치
- 유효성: 참
- 해악도: 실패 (그리고 회피하면 거짓신호로 바뀐다)
- 대상: 계획자신
- 얼마나 아픈가: 되돌릴 수 있다. 처분은 셋 중 하나다 — ① 만들 파일의 발견은 **가장 가까운 기존 좌표**(예: `crates/pal-extract/src/typescript.rs`)에 건다 ② `경로` 에 「앞으로 생길 자리」를 표현할 값을 만든다 ③ `(경로 없음)` + 요약에 자리를 적고 그 비율을 판정에 싣는다.

### `#[cfg(test)]` 를 다 세면 ⑧ 의 분모가 시험 코드로 부푼다
- 어떻게 실패하나: 계획 결정 #3 은 *"`#[cfg(test)]` 를 건너뛰지 않는다. 선언은 전부 센다"* 다. 실측: 이 저장소 117 개 `.rs` 의 선언 2,803 중 **629(22%)** 가 `#[cfg(test)] mod` 안이고, 그 밖에 `tests/` 디렉터리의 통합 시험 파일이 **28 개** 더 있다. 계획의 종료 조건 ⑦과 원 의도의 ⑧(*결박 ≥ 1*, 그리고 이 회차가 늘리려는 그 수)은 **시험 함수만으로도 충족될 수 있다.** 그러면 「자기 저장소 실사용」이 참인데 그 실사용이 시험 코드 위에 서고, 그것은 지금의 「픽스처 한 파일 위에 서 있다」와 같은 종류의 거짓이다. 계획에는 ⑧ 의 결박이 **비-시험 심볼 위에 서야 한다**는 조건이 없다.
- 어디가 걸리나: 계획 #3·#7·원 의도 §⑧ · `crates/pal-cli/src/ledger.rs:301` `nodes_of`(시험 심볼도 그대로 2층에 간다)
- 획득: 조회 → **실측**. `#[cfg(test)]` 를 앞 형제 `attribute_item` 으로 판정해 전수 카운트(629 / 2,803).
- 모집단: 원의도
- 유효성: 참
- 해악도: 거짓신호
- 대상: 계획대상
- 얼마나 아픈가: 합격선에 한 줄(「⑧ 의 결박은 `#[cfg(test)]` 밖에 선다」)이면 끝난다. **측정 전에** 등록해야 사후 기입이 아니다.

### `#[cfg]` 로 갈린 동명 선언이 `identity_ceiling` 을 `Ordinal` 로 내린다
- 어떻게 실패하나: 추출기가 `cfg` 를 해석하지 않으므로 `#[cfg(unix)] fn 실행_권한` 과 `#[cfg(not(unix))] fn 실행_권한` 이 **같은 (경로, 컨테이너 체인, 이름, 종류)** 로 두 번 나온다. `nodes_of` 가 두 번째에 `ordinal = 1` 을 주고, `Discriminator::identity_ceiling` 이 그 심볼을 `Ordinal` 로 고정한다 — 언어 등급이 L2 여도 `body_digest` 지우기가 꺼진다. 그리고 좌표가 **소스 순서에 묶여서**, `#[cfg(unix)]` 팔과 `#[cfg(not(unix))]` 팔의 순서를 바꾸는 것만으로 두 결박이 서로 뒤바뀐다. 실측: 이 저장소에 그런 파일이 2 개(`crates/pal-cli/tests/hook.rs:191,197` `복사한_것에_실행_권한` · `crates/pal-cli/tests/install_hooks.rs:95,101` `실행_권한`). cargo 에서는 훨씬 흔하다.
- 어디가 걸리나: `crates/pal-cli/src/ledger.rs:313` (`Discriminator::new(s.kind, *slot)`) · `crates/pal-core/src/coord.rs:255-265` `identity_ceiling` · `crates/pal-cli/tests/hook.rs:191`
- 획득: 조회 → **실측**(스크립트로 파일별 `#[cfg]` 직후 동명 선언 중복을 전수 탐색 → 2 건).
- 모집단: 저장소
- 유효성: 참
- 해악도: 거짓신호 (등급 표는 L2 라 광고하는데 그 심볼들은 실제로 Ordinal — 다만 이 강등은 **설계대로 동작하는 것**이다)
- 대상: 계획대상
- 얼마나 아픈가: 안 고쳐도 된다. 다만 게이트의 「범위 밖」에 적어야 한다 — 안 적으면 판정 표의 Exact 비율이 왜 낮은지 설명되지 않는다.

### 소유자 지시 원문을 정정하는 방식이 계획에 없다 — `docs/instructions/` 는 원문이 사는 자리다
- 어떻게 실패하나: 계획 #1 은 *"기존 소유자 지시 2026-08-12 §1 「넷이 전부 1급」을 정정하는 결정"* 이라 적는다. 그런데 AGENTS.md 는 `docs/instructions/` 를 *"소유자가 실제로 무엇을 요구했나 — **원문이 산다**"* 로 정의한다. 원문을 고치면 **소유자가 그때 무엇을 말했는지의 기록이 사라진다**(2026-08-12 문서 §1 은 *"kotlin 지원은 1급이야. java, javascript, typescript 도 마찬가지로 말이야"* 라는 인용을 담고 있다). 고치지 않으면 `language.rs:1`·`shell.rs:33`·`docs/plan/00-stack.md:142`(*"문법 넷 … 넷이 전부 1급이다"*)가 **그 지시를 근거로 「넷」이라 주장하는 상태**로 남는다. 어느 쪽이든 사실이 아닌 것을 사실로 적게 되고, **계획에는 어느 쪽인지가 없다.** (다행히 그 문서를 가리키는 링크 넷은 앵커를 안 쓰므로 죽은 링크 검사는 안 걸린다.)
- 어디가 걸리나: `docs/instructions/2026-08-12-owner-direction.md:12,22,24` · `crates/pal-core/src/language.rs:1` · `crates/pal-extract/src/shell.rs:33` · `docs/plan/00-stack.md:142,182` · `crates/pal-core/src/capable.rs:11`(이미 낡았다 — *"추출기는 하나뿐이라"* 인데 지금 둘이다)
- 획득: 조회 (`grep -rn "2026-08-12-owner-direction"` 으로 참조자 4 곳 · 각 자리 정독 · AGENTS.md 의 「원문이 산다」)
- 모집단: 규약
- 유효성: 참
- 해악도: 금지역 (사실이 아닌 것을 사실로 적는 자리가 **어느 쪽을 골라도** 하나 생긴다)
- 대상: 계획자신
- 얼마나 아픈가: 새 지시 문서(2026-08-20) 하나 + 옛 문서에 「§1 은 2026-08-20 이 갱신했다」 한 줄이면 되돌릴 수 있다. 안 하면 다섯 자리가 조용히 낡는다.

### `deny.toml` 의 git 출처 예외가 셋으로 는다 — 계획의 제약문은 라이선스만 지킨다
- 어떻게 실패하나: 계획 §제약은 *"`cargo deny` 가 라이선스를 본다 — 새 의존의 라이선스가 예외를 늘리면 안 된다"* 라고만 적는다. `tree-sitter-rust` 는 MIT 라 **라이선스 예외는 안 는다**(확인함). 그러나 `[sources] allow-git` 은 2 → **3** 으로 는다. `deny.toml:6` 이 *"이 파일에 줄이 느는 것 자체가 관측 대상이다"* 라 못 박았고, `:53` 이 git 출처 예외를 *"이 파일에서 가장 중요한 줄"* 이라 부른다. 계획의 제약문이 **관측 대상 하나를 안 덮고 있다** — 안 적으면 게이트 판정에서 그 사실이 안 세어진다. (안 더하면 검사 5 「의존 정책」이 즉시 실패하므로 조용한 실패는 아니다.)
- 어디가 걸리나: `deny.toml:88-91` `allow-git` · `deny.toml:6` · 계획 §이 저장소의 제약 마지막 줄
- 획득: 조회 (deny.toml 전문 · `tree-sitter-rust` 상류 `Cargo.toml` 실물 조회 → `license = "MIT"` · 의존 `tree-sitter-language 0.1` + build-dep `cc`)
- 모집단: 규약
- 유효성: 참
- 해악도: 미관 (관측 대상 하나가 판정에서 빠진다)
- 대상: 계획자신
- 얼마나 아픈가: 계획 문서 한 줄.

### `recognize.rs` 의 `("rs", "Rust")` 가 죽은 가지가 된다
- 어떻게 실패하나: `BY_EXTENSION` 에 `("rs", "Rust")` 가 있고, 그 배열 머리에는 *"1급 넷 — `Language` 로도 잡힌다. 이름을 여기 한 번 더 적는 대신 아래에서 변환한다"* 라는 주석이 있다. `Language::from_extension("rs")` 가 `Some(Rust)` 를 내는 순간 `recognize` 는 `BY_EXTENSION` 에 닿기 전에 반환하므로 그 항목은 **도달 불가**가 된다. 안 지우면 「이름을 두 곳에 적지 않는다」는 그 주석이 스스로에 대해 거짓이 되고, 지우는 것을 잊어도 **어떤 검사도 안 운다**(`recognize.rs` 시험은 `kt`·`ts` 만 본다).
- 어디가 걸리나: `crates/pal-extract/src/recognize.rs:35` (주석) · `recognize.rs:58` (`("rs", "Rust")`) · `recognize.rs:129`(닿기 전에 반환하는 자리)
- 획득: 조회 (`recognize` 의 분기 순서 정독)
- 모집단: 저장소
- 유효성: 참
- 해악도: 미관
- 대상: 계획대상
- 얼마나 아픈가: 한 줄 삭제.

---

## 내가 기각한 것

| # | 제목 | 어떻게 실패한다고 봤나 | 어디가 걸리나 | 획득 | 모집단 | 유효성 | 해악도 | 대상 | 얼마나 아픈가 |
|---|---|---|---|---|---|---|---|---|---|
| 1 | `;` 삭제가 tail expression 과 statement 를 뭉갠다 | `fn f() -> i32 { … x }` 와 `{ … x; }` 가 같은 digest 가 될 것이라 봤다 | `crates/pal-extract/src/parse.rs:181` | 실측 — 포팅한 정규화로 비교 | 저장소 | 거짓 | 미관 | 계획대상 | **다르다.** `expression_statement` 마디가 `⟨⟩` 표식을 하나 더 내서 갈린다. `break`/`unsafe`/`if` 형태 넷 다 갈렸다 |
| 2 | `&x` 와 `x`, `&T` 와 `T` 가 같은 digest 를 갖는다 | `is_leading_separator` 가 `reference_expression`·`reference_type` 의 `&` 도 지운다 | `crates/pal-extract/src/parse.rs:248` | 실측 — 네 쌍 비교 | 저장소 | 거짓 | 미관 | 계획대상 | **다르다.** 두 경우 다 마디의 `⟨⟩` 가 남아 갈린다. 걸리는 것은 `self_parameter` 하나뿐이었다 |
| 3 | `[0; 10]` 배열 반복이 뭉개진다 | `array_expression` 의 `;` 가 지워져 다른 배열과 같아진다 | `crates/pal-extract/src/parse.rs:181` | 실측 | 저장소 | 거짓 | 미관 | 계획대상 | **다르다.** 짝이 될 유효한 Rust 가 없다(`[0 10]` 은 문법 오류) |
| 4 | `FIRST_CLASS` 를 안 늘려도 컴파일이 통과한다 | 손 유지 배열이라 컴파일러가 안 잡을 것이라 봤다 | `crates/pal-extract/src/shell.rs:37,86,113` | 조회 | 저장소 | 거짓 | 미관 | 계획대상 | **잡힌다.** `type Shells = [Capable<GraphShell>; 4]` 가 `FIRST_CLASS.map(probe)` 의 길이를 타입으로 강제한다. 컴파일러가 못 잡는 것은 `extractor.rs:89` 와 `language.rs:41` 이다 |
| 5 | `tree-sitter-rust` 가 `deny.toml` 의 라이선스 예외를 늘린다 | 새 문법 크레이트가 새 라이선스를 끌고 온다 | `deny.toml:14-40` | 조회 — 상류 `Cargo.toml` 실물 확인 | 규약 | 거짓 | 미관 | 계획대상 | **안 는다.** `license = "MIT"` · 의존은 `tree-sitter-language 0.1` 과 build-dep `cc` 뿐이고 둘 다 이미 트리에 있다 |
| 6 | 손 표본 TSV 가 회차 레코드 검사 ①의 「헤더 행」 규칙에 걸린다 | 기존 표본처럼 `#` 주석으로 시작하면 `첫줄.starts_with('#')` 에 걸린다 | `xtask/src/main.rs:3243-3250` | 조회 — 검사의 모집단 글롭 확인 | 자기장치 | 거짓 | 미관 | 계획자신 | **글롭 밖이다.** 검사는 `.palimpsest/rounds/**` 만 본다. 계획은 표본을 `corpus/tasks/` 에 둔다. ⚠ 회차 디렉터리로 옮기면 즉시 걸린다 |
| 7 | `tree-sitter` 0.26 과 `tree-sitter-rust` 의 ABI 가 안 맞는다 | 상류가 `tree-sitter = "0.25"` 를 적고 있다 | `Cargo.toml:37` | 조회 — 상류 `Cargo.toml` 실물 | 저장소 | 거짓 | 미관 | 계획대상 | **맞는다.** 그 0.25 는 `dev-dependencies` 다. 런타임 접점은 `tree-sitter-language = "0.1"` 이고 Kotlin·TypeScript 와 같다 |
| 8 | 골든 넷이 움직인다 | Rust 를 1급으로 올리면 대장 산출이 바뀐다 | `corpus/golden/portal-backend.ledger.json` · `ditto.symbols.tsv` | 실측 — 골든의 언어 분포 전수 | 저장소 | 거짓 | 미관 | 계획대상 | **안 움직인다.** 두 골든에 `.rs` 가 **0** 개다(Kotlin 672 · SQL 187 · Markdown 62 …). 그리고 그 사실이 곧 「골든이 Rust 회귀를 못 잡는다」는 뜻이기도 하다 |
| 9 | `&'a self` 와 `self` 도 같은 digest 를 갖는다 | lifetime 도 함께 사라질 것이라 봤다 | `crates/pal-extract/src/parse.rs:248` | 실측 | 저장소 | 거짓 | 미관 | 계획대상 | **다르다.** `lifetime` 노드가 남아 갈린다. 충돌은 lifetime 이 **없을 때**만 난다 — 그런데 실물의 `&self` 는 거의 전부 lifetime 이 없다 |

새 범주: **언어 중립으로 선언된 규칙이 새 언어에서 조용히 거짓이 되는 자리** — `parse.rs` 의 `;`·`&`·`|` 규칙은 각각 JS/Kotlin 의 의미로 정당화된 뒤 *"언어마다 따로 쓰지 않는다"* 는 규율로 전 언어에 걸려 있다. 다섯째 언어가 들어오는 순간 그 정당화 문장이 거짓이 되는데, 문장이 거짓이 된 것을 재는 장치가 없다. 시딩의 「기존 결정을 어기는 자리」와 다른 것은, **여기서는 결정을 어기지 않고 결정의 근거만 무너진다**는 점이다.
