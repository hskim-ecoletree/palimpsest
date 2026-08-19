사전부검 라운드 2 — 격리 사본(`scratchpad/exp`)에 `tree-sitter-rust@77a3747`(v0.24.2)을 실제로 붙여 빌드·파싱했고, ditto 496 파일 · cargo 378 파일 · 자기 저장소 117 파일에서 전수로 쟀다.

---

### Rust 속성(`#[...]`)이 심볼 마디 **밖**이라 `body_digest` 가 속성 변경을 못 본다
- 어떻게 실패하나: `#[derive(Debug)] struct S` → `#[derive(Debug, Clone, Serialize)] struct S` 를 정규화하면 **바이트가 같다.** `#[cfg(unix)] fn f` ↔ `#[cfg(windows)] fn f` 도 같고, `#[serde(rename="a")]` ↔ `"b"` 도, `#[must_use]` 유무도, `#[deprecated]` ↔ `#[inline]` 도 전부 같다. tree-sitter-rust 에서 `attribute_item` 은 `function_item`/`struct_item` 의 **형제**이므로 선언 마디를 정규화하면 속성이 애초에 포함되지 않는다. 결박이 이런 심볼을 지켜보면 derive 를 갈아도 `Live` 로 남는다 — R-22 가 금지한 *"서로 다른 코드가 같은 요약"* 이고 `&self`↔`self` 보다 **훨씬 흔하다**(cargo 378 파일에 `#[derive`·`#[cfg` 가 수천 건).
- 어디가 걸리나: `crates/pal-extract/src/parse.rs:169` `normalize_into` · 새 `rust.rs` 의 `Symbol.span.byte_start` 결정 · `crates/pal-core/src/symbol.rs:76` `Symbol::body`
- 획득: 조회 — 격리 사본에서 다섯 쌍을 심어 `normalize(function_item|struct_item)` 바이트 비교, 다섯 쌍 **전부 동일**
- 모집단: 저장소
- 유효성: 참
- 해악도: 금지역
- 대상: 계획대상
- 얼마나 아픈가: 되돌리기 비쌈 — 고치려면 심볼 span 을 앞 `attribute_item` 까지 넓혀야 하고 그러면 `symbol_id`(span 은 성분 아님, OK)는 안 움직이지만 **`body_digest` 전량이 이동**한다. 라운드 1 의 처방(「`다음_선언` 이 앞 형제 `attribute_item` 을 건너뛴다 · `벗긴다` 계약은 안 바꾼다」)은 **주석 축만 처분하고 span 축은 손대지 않아 이 선택을 굳힌다.** 걸리는 곳: Rust 전 심볼

### `is_leading_separator` 의 ditto 골든 대조는 **원리상 못 움직이는** 음성 대조다
- 어떻게 실패하나: 계획 처방 1 은 *"규칙에 노드 조건을 단다. `type X = | A | B` 정규화가 함께 움직이는지 ditto 골든 4,578 줄로 대조한다"* 인데, **ditto 496 파일 전수에서 `is_leading_separator` 가 참을 내는 자리가 0 건이다.** `union_type`/`intersection_type` 은 앞에 `|` 가 오면 이름 있는 자식이 정확히 하나라 `TRANSPARENT_IF_SINGLE`(`parse.rs:58`)이 먼저 잡고, 그 가지는 `transparent && !child.is_named()` 에서 이미 걸러진다(`parse.rs:210`). 즉 무슨 조건을 달아도 TS 골든은 안 움직이며, 대조가 초록인 것은 **처방이 옳다는 증거가 아니라 잴 것이 없다는 뜻**이다. 게이트에 「골든 4,578 줄 불변 확인」이라 적히면 그것이 곧 거짓 안심이다.
- 어디가 걸리나: `crates/pal-extract/src/parse.rs:248` `is_leading_separator` · 같은 파일 `239-247` 의 근거 주석(*"ditto 실측에서 그것이 마지막 남은 16 건이었다(파일 12)"*)
- 획득: 조회 — 격리 사본에 호출 지점과 동일한 가드 순서로 계수기를 심고 ditto `.ts`/`.tsx` 496 파일 전수 순회 → `파일 496 · is_leading_separator 참 0 건 · {}`. `TRANSPARENT_IF_SINGLE` 과 이 함수는 같은 커밋(`4532f48`)에서 함께 들어왔다
- 모집단: 자기장치
- 유효성: 참
- 해악도: 금지역 (측정이 죽은 가지가 됨)
- 대상: 계획자신
- 얼마나 아픈가: 되돌리기 쌈. ★ **더 작은 표면이 같은 답을 낸다** — 0 건이므로 이 규칙을 **삭제**해도 TS 산출은 바이트로 불변이고, 삭제하면 Rust 의 `&self`↔`self` 도 그것만으로 갈린다(`&` 가 남아 `self_parameter` 두 형태가 달라진다). 「조건을 단다」보다 「지운다」가 작고, 지우면 주석의 거짓 근거도 함께 사라진다

### 인접 `line_comment` 접기가 TypeScript 를 실제로 움직인다 — 단위시험 빨강 + ditto 330→327
- 어떻게 실패하나: 계획 처방 4 는 접기를 `parse.rs` 의 **공용** 수집기에 넣는다(모듈 주석이 *"언어마다 따로 쓰지 않는다"* 를 요구한다). 그러면 `parse.rs:766` `주석_블록_전체가_같은_선언에_붙는다` 가 `// @decision: 첫 줄\n// 이어지는 설명\n// ADR-0007\nexport class C {}` 에 대해 `assert_eq!(c.len(), 2)` 를 단언하는데 접으면 1 이 되어 **`cargo xtask test` 가 빨개진다.** 그리고 ditto 전수에서 표식 주석 **330 → 327** 로 3 건이 사라진다 — 이것은 `[f10]`·`[f11]` 결박 모집단의 이동이고, 계획이 스스로 등록한 금지역 「기존 **두** 언어 회귀」에 정면으로 걸린다.
- 어디가 걸리나: `crates/pal-extract/src/parse.rs:526` `모은다` · `parse.rs:766-771` · `corpus/tasks/f10-6-binding-sample.tsv` · `corpus/tasks/f10-5-binding-sample.tsv`
- 획득: 조회 — 격리 사본에서 `TypeScriptExtractor::marked_comments` 를 ditto 496 파일에 돌려 인접(사이에 빈 줄 없음·공백만) 그룹으로 접은 수를 셈 → `표식 주석 330 · 붙은 것 279 · 인접 접은 뒤 327`
- 모집단: 저장소
- 유효성: 참
- 해악도: 실패 (+ 금지역: 등록된 「두 언어 회귀」)
- 대상: 계획자신
- 얼마나 아픈가: 되돌릴 수 있으나 갈림길이 강제된다 — ⓐ 언어 조건부 접기(모듈 주석의 「언어 공통」 선언을 깬다) ⓑ 공용 접기 + TS 시험·표본 갱신(금지역 위반) ⓒ 안 접기(`///` 조각 폭증을 안는다). **계획에 셋 중 무엇인지가 없다**

### `attribute_item` 건너뛰기는 되지만, 표식 130 중 **59 건(45%)** 은 여전히 심볼 아닌 마디에 붙는다
- 어떻게 실패하나: 자기 저장소 `.rs` 117 파일의 표식 주석 마디는 **130 개**(계획 실측과 일치), 좌표 없음 **3 건**(일치), 현재 `attribute_item` 에 붙는 것 **49 건**(일치)이다. 처방대로 `attribute_item` 을 건너뛰면 선언 마디로 가는 것이 24 → **68** 로 오른다(처방은 실제로 듣는다). 그런데 나머지는 `use_declaration` **21** · `enum_variant` 8 · `field_declaration` 5 · `field_identifier` 3 · `let_declaration` 10 · `expression_statement` 7 · `match_arm` 1 · `identifier` 1 · `field_initializer` 1 · `inner_attribute_item` 1 · 없음 3 = **61 건**이고, 이것들의 `attaches_to_byte` 는 `pal-cli/src/narrative.rs:303` 의 **정확 일치 자리 맵**에 없다 → 조용히 미결박. 계획의 ⑧ 은 「결박이 선다」를 이 처방 하나에 걸었는데 **상한이 130 이 아니라 68**이고, 여기에 처방 4(접기)가 `///` 블록을 다시 접으면 더 준다.
- 어디가 걸리나: `crates/pal-extract/src/parse.rs:563` `다음_선언` · `crates/pal-cli/src/narrative.rs:287,303`
- 획득: 조회 — 격리 사본에서 `모은다` 를 복제해 `attribute_item` 을 건너뛰는 판/안 건너뛰는 판의 **도착 마디 종류 분포**를 자기 저장소 전수로 계수
- 모집단: 원의도
- 유효성: 참
- 해악도: 거짓신호 (⑧ 의 분자가 계획의 기대보다 작고, 게이트가 그것을 예고 없이 만난다)
- 대상: 계획대상
- 얼마나 아픈가: 되돌릴 수 있음. 걸리는 곳 하나(합격선 문장). **측정 전에** ⑧ 의 상한을 68 로 적어 두지 않으면, 재고 나서 낮은 수를 설명하기 위해 표본을 손보고 싶어지는 자리가 된다(계획이 스스로 금지역으로 등록한 「손 표본을 추출 결과에 맞춰 고침」)

### `tree-sitter-rust` 핀 SHA 가 계획 문서에서 **틀렸다** — `4f31efe` 는 v0.24.2 가 아니라 **v0.5.2**
- 어떻게 실패하나: 계획은 *"후보: `v0.24.2` = `4f31efe`… 계열"* 이라 적었다. 실제로 `refs/tags/v0.24.2` = `77a3747266f4d621d0757825e6b11edcbf991ca5` 이고, `4f31efefd36eaa2b6bf32efbc350699a8b4c7415` 는 **`refs/tags/v0.5.2` 의 태그 객체**다(peeled = `a608c1ca`). 이 값을 `Cargo.toml` 의 `rev` 와 `corpus/criteria.toml` 의 **사전 등록된 판정 축**에 그대로 옮기면 2019년 문법을 핀하게 되고, 「후보를 전수로 재고 사전 등록된 축으로 갈랐다」는 G50 형태의 주장 자체가 거짓 위에 선다.
- 어디가 걸리나: 계획 §만들 계획 「코어」 첫 줄 → 만들 자리 `Cargo.toml:119` 부근 · `deny.toml` 예외 6 · `corpus/criteria.toml` 새 절
- 획득: 조회 — `git ls-remote --tags https://github.com/tree-sitter/tree-sitter-rust`
- 모집단: 자기장치
- 유효성: 참
- 해악도: 금지역 (사실이 아닌 것을 사실로 적음 — 그것이 사전 등록 문서에 들어간다)
- 대상: 계획자신
- 얼마나 아픈가: 되돌리기 쌈(한 줄). 걸리는 곳 셋(`Cargo.toml`·`deny.toml`·`criteria.toml`). ⚠ 부수 실측: `77a3747`(v0.24.2)은 이 저장소의 `tree-sitter = "0.26"`(0.26.12)와 **실제로 빌드되고 파싱된다** — 계획의 *"파싱 확인됨"* 은 맞다. **틀린 것은 SHA 뿐이다**

### `GRAMMAR_REV` 합성이 골든 재축복을 강제하고, 그 재축복이 「두 언어 회귀」를 흡수한다
- 어떻게 실패하나: `corpus/golden/portal-backend.ledger.json:4` 에 `"grammar": "acb96307…"` 이 **박혀 있다.** `scripts/f01-verify.py:322` 는 골든 파일을 **본문 전체 문자열로** 비교하므로 합성 rev 로 바꾸는 순간 대조가 깨지고 `--bless` 로 덮어써야 한다. `--bless` 는 파일을 통째로 다시 쓴다(`f01-verify.py:316`) — 즉 **같은 커밋에서 Kotlin 산출이 우연히 움직였어도 그것까지 함께 축복된다.** 계획은 「기존 두 언어 회귀」를 금지역으로 등록해 놓고, 그 금지역을 관측하는 유일한 장치를 재베이스라인하도록 요구한다. 그리고 `scripts/f04-verify.py:194` 는 `'pub const GRAMMAR_REV: &str = "acb96307…";'` 라는 **리터럴 치환**으로 문법 축 음성 대조를 하는데, 합성으로 바꾸면 그 문자열이 사라져 `③ 문법 버전 — 치환 대상이 소스에 없다` 로 어긋남을 낸다(다행히 조용히 통과하진 않는다).
- 어디가 걸리나: `crates/pal-extract/src/lib.rs:66` · `corpus/golden/portal-backend.ledger.json:4` · `scripts/f01-verify.py:311-326` · `scripts/f04-verify.py:194,232-241`
- 획득: 조회 — `grep -rn "acb96307"` · `f01-verify.py`·`f04-verify.py` 해당 줄 읽음
- 모집단: 저장소
- 유효성: 참
- 해악도: 금지역 (등록된 금지역의 관측 장치가 같은 커밋에서 재베이스라인된다)
- 대상: 계획자신
- 얼마나 아픈가: 되돌릴 수 있으나 절차가 필요하다 — 「축복 전에 `diff_ledger` 산출이 `detector.grammar` **한 줄**뿐임을 확인하고 그 diff 를 게이트에 싣는다」가 계획에 없다. `f04-verify.py` 는 계획의 처방 목록(`deny.toml:75`·`lib.rs:40-65`)에 **아예 없다**

### 등록된 금지역 「기존 두 언어 회귀」에 **CI 관측 장치가 하나도 없다**
- 어떻게 실패하나: 두 언어 회귀를 잡는 것은 골든 셋(`portal-backend.ledger.json`·`ditto.symbols.tsv`·`portal-backend.symbols.tsv`)뿐이고, 그것을 대는 것은 `scripts/f01-verify.py`·`f03-3-verify.py` 뿐이며, 계획 스스로 *"`scripts/f0*-verify.py` 는 하나도 CI 에 없다 · 「CI 에 안 넣는다」를 판정으로 적는다"* 라 적었다. 그런데 CI 가 도는 `cargo xtask test` 안에서 TS/Kotlin 요약을 붙드는 것은 `crates/pal-extract/tests/normalize_props.rs` 의 **상대 불변식**(변형 전후가 같은가)뿐이라, `body_digest` 가 **전면 균일 이동**하면 양쪽이 함께 움직여 초록이다. 소스 어디에도 고정된 64자 16진 요약 기대값이 없다(실측: `crates/**` 의 64-hex 리터럴은 전부 `install/sha256.rs` 의 SHA-256 벡터). 즉 `;` 규칙이나 접기를 잘못 좁혀 TS 요약이 통째로 이동해도 **세 OS × 7 잡이 전부 초록**이다.
- 어디가 걸리나: `.github/workflows/ci.yml` (`xtask check`·`xtask test`·`interop-*.sh` 만) · `xtask/src/main.rs:300-312` 축 둘 · `crates/pal-extract/tests/normalize_props.rs`
- 획득: 조회 — `ci.yml` 전문 · `xtask::test` 의 축 배열 · `grep -rEn "\"[0-9a-f]{64}\"" crates`
- 모집단: 자기장치
- 유효성: 참
- 해악도: 금지역 (금지역이 등록됐는데 그것을 켜는 회로가 CI 에 없다)
- 대상: 계획자신
- 얼마나 아픈가: 되돌릴 수 있음. 계획은 이 사실을 「재현성이 이 기계 하나에 묶인다」로만 적었는데, **재현성 문제가 아니라 탐지 부재 문제**다. 최소 처방 하나면 닫힌다 — `pal-extract` 에 TS·Kotlin 씨앗 각 하나의 **절대 요약 고정 시험**을 넣으면 CI 가 전면 이동을 본다

### `impl` 컨테이너 체인이 트레잇 이름을 못 담아 cargo 심볼의 **6.1%** 가 선언 순서에 매인다 (R-16)
- 어떻게 실패하나: `container_chains`(`pal-cli/src/ledger.rs:269`)는 체인 성분으로 **컨테이너 심볼의 `name` 하나**만 쓴다. Rust 의 `impl Error`, `impl From<A> for Error`, `impl From<B> for Error` 는 셋 다 대상 타입 이름 `"Error"` 로 접히므로, 그 안의 `fn from` 셋이 `(체인=["Error"], 이름="from", 종류)` 로 **같은 열쇠**가 되어 `ordinal` 0·1·2 를 받는다. `impl` 블록 순서를 바꾸면 세 좌표가 서로의 심볼을 가리키고, 본문이 다르므로 `Orphaned` 가 아니라 **평범한 `Stale`** 로 위장한다 — R-16 이 적은 정확히 그 형태다. cargo 378 파일 7,620 선언 중 **464 건(6.1%)** 이 `ordinal>0` 이고(`impl_item` 374 · `function_item` 70 · `mod_item` 9 …), `#[cfg(unix)] mod imp` / `#[cfg(windows)] mod imp` 처럼 체인 성분 자체가 겹치면 그 안의 모든 심볼이 딸려 온다. 자기 저장소는 117 파일 2,778 선언 중 44 건(1.6%).
- 어디가 걸리나: `crates/pal-cli/src/ledger.rs:269-285` `container_chains` · `:307-337` `nodes_of` · `crates/pal-core/src/coord.rs:247` `Discriminator::identity_ceiling` · R-16
- 획득: 조회 — 격리 사본에서 `mod`/`impl`/`fn`/`struct`/… 중첩 순회를 짜고 `(체인, 이름, 종류)` 중복을 cargo 380 모집단과 자기 저장소 117 파일에서 전수 계수
- 모집단: 저장소
- 유효성: 참
- 해악도: 금지역 (조용한 재결박 — 좌표가 상대 심볼을 가리킨다)
- 대상: 계획대상
- 얼마나 아픈가: 되돌리기 중간. ★ R-16 의 「대응」이 *"F03에서 코퍼스로 세고, 유의미하면 넣는다"* 이고 **이 회차가 그 코퍼스를 처음 갖는다.** 계획에 그 측정 항목이 없다. 최소 처방: `impl_item` 의 체인 성분을 `trait::Type` 로 만들거나(문법이 `trait` 필드를 준다) 그 수를 판정 표에 싣기

### L1 로 내린 결정이 위 6.1% 를 **관측 불가능하게** 만든다
- 어떻게 실패하나: `nodes_of`(`ledger.rs:335`)는 `identity = discriminator.identity_ceiling().min(s.identity)` 다. L1 이면 `Symbol::identity` 가 항상 `Ordinal`(`ExtractGrade::L1.identity()`, `ledger.rs:464`)이므로 **`ordinal` 로 떨어진 464 건과 그냥 L1 이라 `ordinal` 인 7,156 건이 대장에서 같은 글자로 나온다.** 앞의 것은 순서에 취약하고 뒤의 것은 아닌데, 산출이 둘을 가르지 않는다. 이것은 라운드 1 이 「두 결정이 접혀 있다」며 L2→L1 로 축소하도록 처분한 **바로 그 처분이 만든 새 표면**이다 — L2 였다면 `identity_ceiling` 하나가 그 464 건만 `ordinal` 로 끌어내려 대장에 `exact/ordinal` 혼합으로 보였을 것이다.
- 어디가 걸리나: `crates/pal-cli/src/ledger.rs:325-336` · `crates/pal-core/src/ledger.rs:75-82` `ExtractGrade::identity`
- 획득: 조회 — 두 자리를 읽고 위 계수와 대조. `min()` 이 두 원인을 접는 것은 코드가 그대로 말한다
- 모집단: 자기장치
- 유효성: 참
- 해악도: 거짓신호 (약한 것과 취약한 것이 같은 화면이 된다 — [ADR-0013] 이 이름 붙인 형태)
- 대상: 계획자신
- 얼마나 아픈가: 되돌리기 쌈 — 판정 표에 「`identity_ceiling` 이 끌어내린 수」를 **따로** 싣기만 하면 닫힌다. 안 적으면 게이트의 Exact 비율 0% 를 「L1 이라서」로만 설명하게 되고 464 건이 그 설명 뒤로 숨는다

### `SymbolKind` 를 안 늘리면 충돌이 6.1% → **11.2%** 로 는다
- 어떻게 실패하나: `SymbolKind`(`pal-core/src/symbol.rs:19`)에는 `Struct`·`Trait`·`Module`·`Const`·`Static`·`Macro`·`Impl` 이 없다. Rust 를 기존 아홉에 접으면(struct/enum/trait/impl → `Class`) `struct Error` 와 그 `impl Error` 들이 `(체인=[], 이름="Error", 종류="class")` 로 **같은 열쇠**가 되어, cargo 에서 `ordinal>0` 이 464 → **852(11.2%)** 로 는다. `discriminator.kind.name()` 은 `SymbolId::compute` 의 성분이므로(`coord.rs:120`) 이 접힘은 정체성에 직접 실린다. 계획의 「만들 계획」에 `SymbolKind` 확장이 **없다.**
- 어디가 걸리나: `crates/pal-core/src/symbol.rs:19-34` · `crates/pal-core/src/coord.rs:108-127`
- 획득: 조회 — 같은 계수기를 「Rust 종류 그대로」와 「아홉으로 접음」 두 판으로 cargo 380 에 돌려 464 vs 852 를 실측
- 모집단: 저장소
- 유효성: 참
- 해악도: 거짓신호 (→ 금지역으로 번진다: 위 재결박 모집단이 1.8배)
- 대상: 계획대상
- 얼마나 아픈가: 되돌리기 쌈(변형 추가는 Kotlin 산출을 안 움직인다 — `symbol.rs:11-16` 이 이미 그 선례를 적었다). 걸리는 곳: `symbol.rs`·`schema.rs`·`surface/queries.toml` 표시 문자열

### `;` 규칙에 언어 축을 넣는 비용이 실측된 효용보다 크다
- 어떻게 실패하나: 계획 처방 2 는 `;` 삭제(`parse.rs:181`)에 언어/노드 조건을 달고 `parse.rs` 모듈 주석의 「언어마다 따로 쓰지 않는다」를 함께 고치라고 한다. 그런데 실측하면 Rust 에서 `;` 삭제가 실제로 뭉개는 것은 **`macro_rules!` 반복 분리자 하나뿐**이다: 꼬리 표현식 `fn f() -> i32 { x }` ↔ `{ x; }`, `{ g() }` ↔ `{ g(); }`, `loop { break }` ↔ `{ break; }`, `if c { a } else { b }` ↔ `{ a; } else { b; }` 는 **전부 요약이 다르다**(`expression_statement` 마디가 `NODE_OPEN/CLOSE` 를 남긴다). 그리고 `);*` 형태는 cargo @ `514c56d` 의 `.rs` 전체에서 **0 건**, 자기 저장소의 `macro_rules!` 는 1 개다. 즉 공용 정규화 함수에 언어 축을 넣고 모듈 계약 문장을 고치는 대가로 얻는 것이 **실물 0~1 건**이다.
- 어디가 걸리나: `crates/pal-extract/src/parse.rs:117-121`(근거 표) · `:181` · `:169` `normalize_into` 시그니처
- 획득: 조회 — 격리 사본에서 다섯 쌍 정규화 비교 + `git -C cargo grep ');\*' 514c56dd -- '*.rs'` → 0
- 모집단: 자기장치
- 유효성: 참
- 해악도: 미관 (실패는 안 나지만 표면이 는다)
- 대상: 계획자신
- 얼마나 아픈가: ★ **만들 필요가 없는 것을 만들고 있는 자리다.** 같은 답을 내는 더 작은 표면: 조건을 `token_repetition_pattern` **노드 한 종류**에만 달면 언어 축도 모듈 계약 문장도 안 건드린다. 부수 확인: `normalize`/`normalize_erasing` 은 `mod parse;` 가 비공개라 크레이트 밖으로 안 나가므로 시그니처 변경 자체의 파급은 3 자리(`kotlin.rs:135`·`typescript.rs:498,566`)뿐이다

### cargo 「프로덕션 380」에 시험 지원 코드 24 파일이 섞여 있다
- 어떻게 실패하나: `tests/testsuite/` 를 뺀 380 은 실측으로 맞다(1,372 → 380, 내가 SHA 에서 직접 셈). 그러나 그 380 안에 `crates/cargo-test-support/` **11** · `crates/resolver-tests/` **7** · `benches/benchsuite/` **5** · `tests/` **1** = **24 파일(6.3%)** 이 시험/벤치 지원 코드다. 판정 표에 「프로덕션 380」이라 적으면 그 문장이 6.3% 만큼 거짓이고, 계획이 같은 회차에서 등록하려는 「⑧ 의 결박은 `#[cfg(test)]` 밖에 선다」와 **모집단 정의가 어긋난다**(한쪽은 디렉터리로, 한쪽은 attribute 로 가른다).
- 어디가 걸리나: 계획 §결정 4 · 만들 자리 `corpus/manifest.toml` 새 `[[corpus]]` · `corpus/criteria.toml` 새 절
- 획득: 조회 — `git -C ~/dev/projects/cargo ls-tree -r --name-only 514c56dd | grep '\.rs$' | grep -v '^tests/testsuite/'` 를 디렉터리별로 집계
- 모집단: 자기장치
- 유효성: 참
- 해악도: 거짓신호
- 대상: 계획자신
- 얼마나 아픈가: 되돌리기 쌈 — 이름을 「`tests/testsuite/` 제외 380」으로 적고 그 안의 24 를 표에 싣기만 하면 된다. 걸리는 곳 둘

### 손 표본을 `56926aa` 에 못 박으면 오라클과 효과가 **다른 트리**에서 재어진다
- 어떻게 실패하나: 처방은 순환을 끊으려 표본을 `56926aa` 에 고정한다. 그런데 종료 조건 7 은 *"`ledger` 가 Rust 를 L1 로 보이고, 결박이 서고, 자기 저장소가 피드백을 낸다"* 이고 그것은 **워킹트리**(새 `rust.rs` 를 포함한 118 파일)에서 재어진다. 정확도는 117 파일 트리에서, 결박·효과는 118 파일 트리에서 나온다. 두 수가 같은 표에 나란히 놓이면 사람은 같은 모집단으로 읽는다. 게다가 이 회차가 고치는 `parse.rs`·`language.rs`·`shell.rs`·`main.rs`·`ledger.rs` 는 **표식 주석을 많이 든 파일들**이라(위 130 건의 상당수) 두 트리의 표식 집합 자체가 다르다.
- 어디가 걸리나: 만들 자리 `corpus/tasks/f-rust-recall-sample.tsv` · `docs/gates/<기능>.md` 「판정」 절
- 획득: 조회 — `pal ledger` 가 `--at` 을 받는 것 확인(`crates/pal-cli/src/main.rs:530` `ledger::compute(&path, at.as_deref(), …)`)이라 **읽는 방법 자체는 있다**. 갈리는 것은 모집단이지 접근이 아니다
- 모집단: 자기장치
- 유효성: 참
- 해악도: 거짓신호
- 대상: 계획자신
- 얼마나 아픈가: 되돌리기 쌈 — 판정 표에 두 열의 트리를 각각 적으면 닫힌다

### 동결된 판정 문서와 `corpus/` 가 *"추출기는 넷뿐이다"* 를 현재형으로 말한 채 남고, 어떤 검사도 안 본다
- 어떻게 실패하나: `docs/gates/F11-touch.md:316` 과 `docs/gates/F12.md:303` 이 *"추출기가 Kotlin·Java·JavaScript·TypeScript 넷뿐이다"* 를 현재형으로 적었는데, 이 둘은 `동결된_판정_문서인가`(`xtask/src/main.rs:2365`)가 참이라 죽은 링크 검사 모집단에서 빠지고, `check_stale_citation` 은 `docs/` 를 통째로 뺀다(`인용_모집단` 의 `밖`). `corpus/criteria.toml:10516` 도 같은 문장을 갖는데 `corpus/` 역시 두 검사 모두에서 빠진다(그리고 *"회차가 안 만진다 — 측정 장치다"* 가 소유자 판정으로 박혀 있어 고칠 수도 없다). 즉 다섯째 언어를 켠 뒤에도 저장소의 판정 문서들이 **넷이라고 말하는 상태로 초록**이다.
- 어디가 걸리나: `docs/gates/F11-touch.md:316` · `docs/gates/F12.md:303` · `corpus/criteria.toml:10516,2322,2403,2920` · `xtask/src/main.rs:2353-2372`
- 획득: 조회 — `grep -rn "넷뿐이다\|넷이 같은 층"` + `링크_모집단_밖`·`동결된_판정_문서인가`·`인용_모집단` 세 곳을 읽음
- 모집단: 저장소
- 유효성: 참
- 해악도: 거짓신호
- 대상: 계획대상
- 얼마나 아픈가: 되돌릴 수 없음(동결·범위 밖) — 그러므로 **게이트의 「범위 밖」 절에 적는 것 말고 길이 없고, 계획에 그 항목이 없다.** 계획은 코드 다섯 자리만 세었다

### 새 게이트 문서 이름의 **첫 글자**가 그 문서의 링크 검사 여부를 정한다
- 어떻게 실패하나: `동결된_판정_문서인가`(`xtask/src/main.rs:2365-2372`)는 `docs/gates/` 아래에서 파일명 첫 글자가 `F`·`G`·`S` 이거나 `preflight` 로 시작하면 **죽은 링크 검사 모집단에서 뺀다.** 계획의 「만들 계획」은 `docs/gates/<기능>.md` 라고만 적었다. `F-rust-extractor.md`·`G51-…` 로 지으면 그 문서 안의 모든 링크가 **한 번도 대어지지 않고**, `rust-extractor.md` 로 지으면 대어진다. 이 회차의 게이트 문서는 판정의 1차 증거를 담을 것이므로, 그 안의 좌표 인용이 낡아도 초록인 상태가 만들어진다.
- 어디가 걸리나: `xtask/src/main.rs:2365-2372` · `:2433`
- 획득: 조회 — 함수 본문 그대로
- 모집단: 규약
- 유효성: 참
- 해악도: 거짓신호
- 대상: 계획자신
- 얼마나 아픈가: 되돌리기 쌈 — 이름을 정하는 한 줄. 다만 이 규칙은 **어디에도 안 적혀 있고** 계획도 모른다

### `Cargo.lock` 이 「만들 계획」에 없고 CI 는 `--locked` 로 빌드한다
- 어떻게 실패하나: `.github/workflows/ci.yml` 의 `produce`·`receive` 잡이 `cargo build --locked -p pal-cli --bin pal` 을 쓴다. `tree-sitter-rust` 를 워크스페이스 의존에 더하고 `Cargo.lock` 을 같은 커밋에 안 담으면 그 두 잡이 `the lock file … needs to be updated but --locked was passed` 로 죽는다. 매트릭스 `check` 잡(`cargo xtask check`/`test`)은 `--locked` 가 아니라 통과하므로 **7 잡 중 4 잡만 빨개진다** — 「일부만 빨강」이 진단을 늦춘다. 그리고 `actions/cache` 키가 `hashFiles('**/Cargo.lock', 'deny.toml')` 이라 세 OS 캐시가 동시에 미스가 되어 회차 시간이 는다.
- 어디가 걸리나: `.github/workflows/ci.yml`(produce/receive 의 `pal 을 세운다` 단계) · `Cargo.lock`
- 획득: 조회 — `ci.yml` 전문
- 모집단: 저장소
- 유효성: 참
- 해악도: 실패
- 대상: 계획대상
- 얼마나 아픈가: 되돌리기 쌈. 걸리는 곳 하나. 계획의 「이 저장소의 제약」에 `deny.toml`·`EXTRACTOR_REV`·rustup 은 있는데 `--locked` 는 없다

---

## 내가 기각한 것

| # | 항목 | 어떻게 실패한다고 봤나 | 어디 | 획득 | 모집단 | 유효성 | 해악도 | 대상 | 왜 아니었나 |
|---|---|---|---|---|---|---|---|---|---|
| 1 | `&T` ↔ `T` 도 같은 요약이 된다 | `is_leading_separator` 가 `reference_type` 의 앞 `&` 를 버리므로 `fn g(x: &T)` 와 `fn g(x: T)` 가 뭉개진다 | `parse.rs:248` | 조회 — 격리 사본에서 두 쌍 정규화 비교 | 저장소 | 거짓 | — | 계획대상 | **갈린다.** `reference_type` 마디가 남긴 `NODE_OPEN/CLOSE` 가 갈라 놓는다. 뭉개지는 것은 `self_parameter` 뿐이다 |
| 2 | Rust 꼬리 표현식의 `;` 가 지워져 반환 의미가 사라진다 | `fn f() -> i32 { x }` 와 `{ x; }` 가 같은 요약이 되어 타입이 바뀌는 변경을 놓친다 | `parse.rs:181` | 조회 — 다섯 쌍 실측 | 저장소 | 거짓 | — | 계획대상 | **네 쌍 전부 갈린다.** `x;` 는 `expression_statement` 로 한 겹 더 싸이므로 마디 표식이 남는다 |
| 3 | `normalize` 시그니처에 언어를 넣으면 공개 API 가 깨진다 | `pal-extract` 밖 소비자가 있다 | `lib.rs:25-32` | 조회 — 재수출 목록·`grep normalize` | 저장소 | 거짓 | — | 계획대상 | `mod parse;` 는 비공개이고 `normalize`/`normalize_erasing` 은 재수출되지 않는다. 부르는 곳은 크레이트 안 3 자리뿐 |
| 4 | `shell.rs:37 FIRST_CLASS` 를 안 늘리면 캐시 능력 축이 조용히 낡는다 | 손 목록이라 컴파일러가 안 잡는다 | `shell.rs:37,89` | 조회 — `type Shells = [Capable<GraphShell>; 4]` 와 `const fn index_of` 의 전수 match | 저장소 | 거짓 | — | 계획대상 | 계획의 판단이 맞다. 컴파일이 선다. 손 목록으로 남는 것은 `extractor.rs:89` 와 `main.rs:550` 둘뿐 |
| 5 | `tree-sitter-rust` 가 `tree-sitter` 0.26 과 ABI 가 안 맞아 빌드가 죽는다 | 0.24.x 는 0.25 세대다 | `Cargo.toml:38` | 조회 — 격리 사본에 `rev=77a3747` 로 실제 추가·빌드·파싱 | 저장소 | 거짓 | — | 계획대상 | **붙고 파싱된다.** 이 보고서의 Rust 실측 전부가 그 빌드 위에서 나왔다 |
| 6 | `모은다` 가 `line_comment` 안의 `doc_comment` 자식을 중복으로 센다 | `///` 가 `line_comment > doc_comment` 두 겹이라 표식이 두 번 잡힌다 | `parse.rs:530-556` | 조회 — 코드 읽음 + 자기 저장소 전수 계수 130 | 저장소 | 거짓 | — | 계획대상 | 주석 마디에서 `continue` 하므로 재귀하지 않는다. 실측 130 은 `line_comment` **줄 수**와 맞고 두 배가 아니다 |
| 7 | `//!` 모듈 주석 28 건이 뒤따르는 `use`/`mod` 에 잘못 붙어 거짓 결박을 만든다 | 빈 줄이 없으면 `다음_선언` 이 아무 선언에나 붙인다 | `parse.rs:563` | 조회 — 자기 저장소 117 파일에서 표식 든 `//!` 블록 **바로 다음 줄이 비어 있지 않은 것 0 건** | 저장소 | 거짓 | — | 계획대상 | 전부 빈 줄로 갈려 `None` 이 된다. 다만 그 28 건이 **영구 미결박**이라는 사실은 위 「표식 130 중 59」 항의 분모에 이미 들어 있다 |
| 8 | `다음_선언` 에 `attribute_item` 건너뛰기를 넣으면 Kotlin·TypeScript 가 함께 움직인다 | 그 함수를 세 언어가 공유한다 | `parse.rs:563` | 추정 — 두 문법의 마디 이름 목록을 전수로 확인하지는 않았다 | 저장소 | 거짓 | — | 계획자신 | `attribute_item` 은 tree-sitter-rust 고유 이름이고 Kotlin 은 `annotation`, TypeScript 는 `decorator` 다 |
| 9 | `EXTRACTOR_REV` 승급이 자기 저장소의 기존 결박을 끊는다 | 좌표에 추출기 버전이 실린다(`Coord.extractor`) | `coord.rs:294` · `binding.rs` | 조회 — `Binding.target` 이 `Coord` 가 아니라 `SymbolId` 이고 `SymbolId::compute` 에 추출기 버전이 없다 | 저장소 | 거짓 | — | 계획대상 | 결박은 `SymbolId` 위에 서므로 승급으로 안 끊긴다. 끊는 것은 **정규화 변경**이고 그것은 `stale` 이지 `orphaned` 가 아니다 |

---

새 범주: **다축 노드의 부분 처분** — 한 구문 마디(`attribute_item`)가 서로 다른 두 축(주석 인접 · 심볼 span/요약)에서 동시에 하중을 지는데, 처방이 한 축만 닫으면 나머지 축의 잘못된 선택이 **처분된 것처럼 보이면서 굳는다.**
