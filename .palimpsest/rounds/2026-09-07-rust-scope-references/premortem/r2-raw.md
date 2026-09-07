# 사전부검 — 라운드 2 원 반환문

> 낸 자: `pal-premortem-sweeper`(새 실행) · 2026-09-08 · HEAD `a8b63ad`
> **안 받은 것**: 대화 기록 · 앞 라운드 결과 · 조건 설계 평가 산출물.
> 라운드 1 이 방어한 것 열넷을 목록으로 주고 「다시 내지 마라」를 걸었다.
> **아래는 그 에이전트가 낸 것을 한 글자도 안 고치고 옮긴 것이다.**

---

프로토타입을 격리 사본(`/tmp/rustscope-20260908/`)에 세웠다 — `scopes.rs`의 2패스·`scope_at`·`hoist_home`·`pal_core::ScopeChain::resolve`·`projection::file_edges`를 그대로 이식하고 계획의 Rust 표(여는 것 6종 · 세는 것 · 안 세는 것)를 넣어, 이 저장소의 `.rs` 134 파일에 13개 변형으로 돌렸다. 기준선 일치 신호: 내 이식이 심볼 3,103개(관측치 3,110)를 낸다.

---

### ADR-0027 이 **등록한 금지역 둘**이 그대로 되살아나는데 계획이 그 ADR 을 안 만진다

- 어떻게 실패하나: 계획대로 `ScopeChain`을 채우면 `docs/adr/0027:54` 의 *"L1 을 고른 것이 **금지역 둘**을 구조적으로 없앴다 — ① Rust shadowing 이 `ScopeChain::resolve` 의 「가장 앞선 것」 팔을 **상시 경로**로 만드는 것 ② `Namespace` 가 둘뿐이라 Rust 의 **매크로 이름 공간**을 못 담는 것"* 이 **거짓 문장이 된다.** 등급 글자만 `L1`로 남기는 것은 이 ADR 의 근거를 하나도 보존하지 않는다 — ADR 이 말한 L1 은 *스코프 체인을 안 만든다*는 뜻이었다(§②, 그리고 같은 ADR 의 대안 표: *"L2 까지 | 금지역 둘을 이 회차가 지고 가야 하고, **`ScopeKind`·`Namespace` 는 1층 캐시에 실려 되돌리기가 비싸다**"*). 계획은 `ScopeKind`·`Namespace`를 Rust 파일마다 1층 캐시에 싣는데 그 대가는 등급 글자와 무관하게 발생한다.
- 어디가 걸리나: `docs/adr/0027-the-instrument-must-reach-its-own-floor.md:54,134` · 계획 9 의 낡는 문면 다섯(`rust.rs:1,298,326`·`classify.rs:226`·`schema/graph.toml:207-210`·`02-order.md:49`·`03-shortest-path.md:79-81`)에 **ADR-0027 도 `docs/gates/rust-extractor.md` 도 없다**
- 획득: 조회 — `grep -rn 금지역 docs/adr/0027*` · ADR 전문 확인 · 계획의 9 항 목록과 대조
- 모집단: 규약
- 유효성: 참
- 해악도: **금지역** — 출처는 내 판단이 아니라 ADR-0027:54 의 등록 목록이다(사실이 아닌 것을 사실로 적음)
- 대상: 계획대상
- 얼마나 아픈가: ADR 개정(또는 대체 ADR)이 필요하다. 되돌리기는 문서만이면 싸지만, 캐시에 실린 `ScopeKind`/`Namespace` 값은 ADR 이 「되돌리기가 비싸다」고 적은 그 자리다. 걸리는 곳 3(ADR·gate·D3 이 가리키는 `classify.rs:226`)

---

### 공용 뼈대의 `hoist_home` 이 「`impl` 이 스코프를 연다」를 **정확히 상쇄한다**

- 어떻게 실패하나: 계획 1 이 뼈대에 **`hoist_home` 을 명시적으로 남긴다.** TypeScript 에서 `function_declaration` 의 이름은 `hoist_home(outer)`(가장 가까운 Function/Module)에 놓인다(`scopes.rs:240`). Rust 표가 「아이템은 순서 무관」을 그 기존 장치로 표현하면 — 이것이 자연스러운 이식이다 — `impl_item` 이 연 스코프(`ScopeKind::Class`)는 `hoist_home` 이 **건너뛰는 종류**라 메서드 이름이 전부 모듈 스코프로 올라간다. 실측: 「hoist_home 으로 아이템 선언」 변형의 엣지 집합이 「impl 이 스코프를 안 연다」 변형과 **비트 단위로 같다**(`plan_hoisthome == plan_noimpl → true`). 옳은 표 대비 **가짜 엣지 78 · 사라진 엣지 17**, 그리고 `new@12 → new@1`·`fmt@19 → fmt@3`·`as_str@8 → as_str@2` 같은 impl 사이 오배선이 생긴다. A2(`impl` 이 스코프를 연다)가 사슬 **모양**만 보면 이 상태에서도 초록이다.
- 어디가 걸리나: `crates/pal-extract/src/scopes.rs:148-159`(`hoist_home`) · `:236-241`(`declare_own` 의 함수 이름 배치) · `crates/pal-core/src/scope.rs:40-51`(`ScopeKind` 에 `Impl` 이 없다)
- 획득: 조회 — 격리 사본 실측(변형 `plan_table` vs `plan_hoisthome`, 134 파일)
- 모집단: 자기장치
- 유효성: 참
- 해악도: **금지역** — 사실이 아닌 REFERENCES 95 건(3,751 쌍의 2.5%)
- 대상: **계획자신** (계획 1 이 만드는 공용 뼈대)
- 얼마나 아픈가: 되돌릴 수 있다(표 한 줄). 그러나 **A1~A13 중 어느 것도 이것을 안 잰다** — 잡으려면 「서로 다른 impl 의 동명 메서드가 서로를 안 가리킨다」는 **해소** 시험이 필요하고 계획에 없다

---

### 언어 표에 **「무엇이 이름을 선언하는가」 축이 없다**

- 어떻게 실패하나: 계획 2 의 열은 다섯이다 — *"스코프를 여는 것 · 참조로 세는 것 · 안 세는 것 · 호이스팅 규칙 · 이름 공간"*. **선언 쪽이 없다.** 그런데 TypeScript 의 선언 쪽은 뼈대에 하드코딩돼 있다: `declare_parameters` 는 마디 이름 `formal_parameters` 와 필드 `pattern` 을 본다(`scopes.rs:269,284-297`), `declare_plain` 은 `lexical_declaration`·`variable_declaration`·`import_statement` 를 본다(`:313-330`). Rust 는 `parameters`/`parameter`·`closure_parameters`·`let_declaration`·`match_arm` 의 `tuple_struct_pattern`·`for_expression`·`let_condition` 이고 **하나도 안 겹친다.** 표에 축이 없으니 그대로 두면 Rust 지역 이름이 하나도 안 묶이고, 파라미터·패턴 이름의 참조가 밖으로 새어 **동명 최상위 심볼로 해소된다.** 실측 가짜 엣지: 파라미터만 빠뜨려도 **52**, 패턴까지 빠뜨리면 **58** (`main@0 → commit@4`, `print@8 → report@4`, `apply@26 → entry@12`, `plan@22 → desired@7`).
- 어디가 걸리나: `crates/pal-extract/src/scopes.rs:284-297`(`formal_parameters`) · `:313-341` · 계획 2 의 열 목록
- 획득: 조회 — 격리 사본 실측(`plan_table` vs `no_params`/`plan_nolocal`) + 문법 덤프로 Rust 마디 이름 확인
- 모집단: 자기장치
- 유효성: 참
- 해악도: **금지역** — 사실이 아닌 REFERENCES 최대 58 건
- 대상: **계획자신**
- 얼마나 아픈가: 되돌릴 수 있다. 곁가지: 계획의 여는 목록에 `for_expression`·`let_condition` 이 없어서, 선언 축을 더해도 `for x in …` 과 `if let Some(x)` 의 `x` 가 **바깥 블록에 남는다**(그 블록이 끝난 뒤의 `x` 까지 그 바인딩으로 해소된다 — 실측 지역 판정 346 건 이동)

---

### `#[…]` 속성 안의 이름이 참조로 세어진다 — 「안 세는 것」 표에 속성이 없다

- 어떻게 실패하나: 계획의 배제 목록은 `field_identifier`·`primitive_type`·라이프타임·경로 꼬리·`use` 절이고, 매크로 처방은 *"`token_tree` … 앞 형제 토큰(`.`·`::`)을 보고 걸러야 한다"* 다. 그런데 **속성도 `token_tree` 로 파싱된다**: `#[cfg(test)]` → `attribute_item > attribute > identifier(cfg) + token_tree > identifier(test)`. `test` 의 앞 형제는 `(` 라 앞형제 규칙이 안 걸린다. 실측: 이 저장소에서 속성 안 이름 **4,258 개**가 참조가 되고 그중 **60 개가 진짜 REFERENCES 엣지**가 된다 — 59 개는 `#[cfg(test)]` 의 `test` 가 `xtask/src/main.rs:363` 의 `fn test` 로 해소된 것이다. 즉 **`pal touch test` 가 「호출자 59」를 산출하는데 전부 존재하지 않는 호출이다.** 나머지 4,198 개는 `outside`(17,691) 를 **24% 부풀린다** — 그 수가 `coverage.unresolved` 로 화면에 나간다.
- 어디가 걸리나: 계획의 「안 세는 것」 열 · `crates/pal-extract/src/scopes.rs:428-434`(`reference_namespace`) · `crates/pal-core/src/scope.rs:145`(`OutsideFile` 의 뜻)
- 획득: 조회 — 격리 사본 실측(속성참조 4,258 · 속성엣지 60) + tree-sitter AST 덤프
- 모집단: 자기장치
- 유효성: 참
- 해악도: **금지역** — 사실이 아닌 것을 사실로(엣지 60) · 미해소 수의 24% 오염
- 대상: 계획대상
- 얼마나 아픈가: 되돌릴 수 있다(배제 규칙 한 줄: `attribute_item` 조상 배제). ADR-0027 §⑤ 가 이미 *"`attribute_item` 은 tree-sitter-rust 고유 마디"* 라고 그 열쇠를 적어 두었는데 계획이 그것을 참조 축에 안 썼다

---

### `cfg` 쌍둥이 선언에서 `resolve` 가 **조용히 하나를 고르고**, 둘째 선언 자리가 **가짜 엣지**가 된다

- 어떻게 실패하나: 추출기는 `cfg` 를 해석하지 않는다(`rust.rs:386` 시험이 그 결정이다). 그래서 `#[cfg(unix)] fn spawn_child` 와 `#[cfg(windows)] fn spawn_child`(`crates/pal-cli/src/round/verify.rs:546,553`)가 **같은 스코프에 같은 이름으로 둘** 선다. `ScopeChain::resolve` 는 *"같은 스코프에 같은 이름이 여럿이면 **가장 앞선 것**을 쓴다"*(`scope.rs:221,231-240`). 결과 둘:
  ① 실측 **39 건**의 엣지가 후보 둘 이상 중 하나를 임의로 가리킨다 — Windows 에서 도는 코드가 unix 선언을 가리킨다(ADR-0023 축).
  ② 실측 **13 건**은 *둘째 쌍둥이의 선언 이름 자체*가 첫째를 가리키는 엣지가 된다 — `projection.rs:238` 의 `bound.declared_at == r.at` 방어가 **다른 바인딩이 뽑히면 안 걸리기 때문**이다. 그 주석이 막으려던 것(*"`pal touch helper` 가 「부르는 것 1건」이라고 답한다 — 자기 자신이다"*)이 정확히 재현된다.
  대조: `stitch_of` 는 같은 모호성에서 *"둘 이상이면 담지 않는다 — 하나를 고르면 그것이 조용한 오답이다"*(`ledger.rs:378-380`)로 **거부**한다. 같은 저장소가 EXPORTS 에서는 거부하고 REFERENCES 에서는 고른다.
- 어디가 걸리나: `crates/pal-core/src/scope.rs:221-249` · `crates/pal-core/src/projection.rs:230-240` · `crates/pal-cli/src/ledger.rs:373-382`
- 획득: 조회 — 격리 사본 실측(모호엣지 39 · 선언자리엣지 13, 「가장 뒤」 팔로 바꾸면 13 → 0)
- 모집단: 저장소
- 유효성: 참
- 해악도: **금지역** — ADR-0027:54 가 등록한 금지역 ①의 실물 형태 · 플랫폼 축(ADR-0023)
- 얼마나 아픈가: 되돌릴 수 있으나 **고칠 자리가 `pal-core`(계획이 "안 만진다"고 한 위층)** 이고, 고치면 TypeScript 해소도 함께 움직인다. 관련 실측: 「가장 앞선 것」 팔은 이 저장소에서 **선언 판정 214 건**을 참조로 오분류한다(`declarations` 12,404 vs 12,618)

---

### `ResolutionGrade::Scoped` 는 「L2 이상에서만」인데 L1 언어가 `scoped` 엣지를 3,752 개 낸다

- 어떻게 실패하나: `schema/graph.toml:418` 이 `[edge.REFERENCES] grade = "scoped"` 로 **고정**하고, `pal-core/src/graph.rs:166` 이 `Scoped` 를 *"스코프 해소로 후보가 유일 — **L2 이상에서만.**"* 으로 정의한다. 계획은 `grade_of(Rust) == L1` 을 **시험으로 잠그면서**(C1) 그 L1 언어에서 `scoped` 엣지를 산출한다. 소비 규칙이 이 등급 위에 서 있다 — *"컨설팅(정밀도 우선)은 `Exact|Scoped` 만 쓴다"*(`graph.rs:157-160`). 즉 정밀도 소비자가 L1 산출을 L2 자격으로 받는다. 실행 중 검사가 없다(`doctor` 의 불변식 여덟에 등급 정합 항목이 없다 — `pal-core/src/doctor.rs:59-77`).
- 어디가 걸리나: `schema/graph.toml:405-420` · `crates/pal-core/src/graph.rs:166` · `crates/pal-core/src/projection.rs:174-178`
- 획득: 조회 — `grep -rn ResolutionGrade::Scoped` · 스키마 원문 · `InvariantId` 전수
- 모집단: 규약
- 유효성: 참
- 해악도: **금지역** — 사실이 아닌 자격을 3,752 엣지에 박는다. ⚠ 계획 9 는 `schema/graph.toml:207-210`(「참조 엣지는 아직 없다」)만 갱신 대상으로 적었고 **418 줄의 등급 선언은 목록에 없다**
- 대상: 계획대상
- 얼마나 아픈가: 되돌릴 수 있다. 걸리는 곳 3(스키마 · `ResolutionGrade` 문서 · `projection.rs` 의 「왜 `scoped` 고정인가」 절)

---

### `doctor` 의 REFERENCES 선언 — **양쪽 다 틀리고**, 한쪽은 측정을 죽은 가지로 만든다

- 어떻게 실패하나: 계획 8/D5 는 *"부재 선언이 아직 참인지 **판정한다**"* 로만 적혀 있다. 두 갈래 다 문제다.
 ① `.holding("REFERENCES")` 로 뒤집으면 — `build_view`(`doctor.rs:192-263`)는 인자로 `symbols` 와 `bindings` 만 받고 2층의 엣지를 **아예 안 읽는다.** 선언만 뒤집으면 불변식 ①②가 REFERENCES 에 대해 **모집단 0 · 위반 0** 으로 초록이 된다. `doctor.rs:315-320` 의 주석이 정반대 방향(선언 누락 = 구멍)만 경고하고 이 방향은 안 본다.
 ② `.absent(...)` 로 두면 — 사유가 `F07/graph-view-stitched-nodes`(*"F07 이 후보 엣지를 만들 때 이 자리가 하중을 진다"*)인데, 그 근거 문장(*"지금 참조 엣지는 … 구조상 어긋날 수 없고 … **작아서 안 걸린 것**"*)이 3 건에서 3,752 건이 되는 순간 무효가 된다. 즉 사유가 거짓인 채로 남는다.
 계획이 "위층은 안 만진다"고 못 박았으므로 옳은 셋째 길(뷰에 엣지를 싣기)은 범위 밖이다.
- 어디가 걸리나: `crates/pal-cli/src/doctor.rs:290-301`(선언) · `:192-263`(`build_view` 의 입력) · `crates/pal-core/src/doctor.rs:59-77`
- 획득: 조회 — `build_view` 시그니처와 `coverage()` 의 관계를 직접 읽음
- 모집단: 자기장치
- 유효성: 참
- 해악도: **금지역**(①의 경우 측정이 죽은 가지) / **거짓신호**(②의 경우)
- 대상: 계획대상
- 얼마나 아픈가: 되돌릴 수 있다. D5 가 「판정」만 요구하고 **어느 답이 나와야 하는지·무엇이 그 답을 지키는지**를 안 정한다

---

### 완수 증인 B1 이 **어떤 고장도 못 가른다** — 옛 증인(`REFERENCES ≥ 100`)과 같은 병

- 어떻게 실패하나: 계획이 「참조 후보 76,416 앞에서 안 가른다」는 이유로 임계를 **기여 파일 수 80** 으로 바꿨다. 실측: 내가 돌린 **13 개 변형 전부**(파라미터 미선언 · 패턴 미선언 · impl 스코프 없음 · hoist_home 오배치 · 지역 선언 전무)가 **기여 파일 126** 을 낸다. 상수다. 가짜 엣지 95 건짜리 구현도 B1 을 46 파일의 여유로 통과한다. 곁들여 E1(표본 30 에 가짜 0)의 검출력을 실측 오류율로 계산하면: 속성 엣지 60/3,751 → 놓칠 확률 62%, hoist_home 95/3,751 → 47%, 파라미터 52/3,751 → 66%. **세 고장 각각이 완수 조건 전체를 절반 넘게 통과한다.**
- 어디가 걸리나: 완수 조건 B1 · E1 (계획 문서)
- 획득: 조회 — 13 변형의 기여 파일 수 실측 + 이항 계산
- 모집단: 자기장치
- 유효성: 참
- 해악도: **거짓신호** (금지역 경계 — 증인이 고장을 못 가르면 「완수」가 사실이 아닌 것을 사실로 만든다)
- 대상: **계획자신**
- 얼마나 아픈가: 싸게 고칠 수 있다 — 임계를 「기여 파일」이 아니라 **음성 대조**로 바꾸면 된다(예: 「파라미터 선언을 끄면 이 시험이 빨개진다」를 사전 등록). 지금 계획에 음성 대조가 하나도 없다

---

### `pal touch` 가 「호출자·피호출자」라고 적는데 Rust 엣지의 **62% 는 호출이 아니다**

- 어떻게 실패하나: 화면은 `호출자 N · 피호출자 M` 을 찍는다(`touch.rs:396`). 그런데 스키마는 *"**`CALLS` 가 아니다** … `CALLS` 로 적으면 타입 참조가 호출로 둔갑한다"*(`graph.toml:407-409` · `projection.rs:167-172`)라고 못 박았다. Rust 에서 그 괴리가 TypeScript 보다 훨씬 크다: 메서드 호출은 `field_identifier` 라 배제되고 `S::new()` 의 꼬리도 배제되므로, 남는 엣지는 **타입 참조 1,182(19.6%) · 호출/매크로 2,305(38.3%) · 그 밖(경로 머리·구조체 리터럴·값) 2,529(42.0%)** 다. 실측 예: `pal touch file_edges` 의 피호출자는 `innermost` 하나만 호출이고 나머지 넷은 `RefCounts`·`ReferenceEdge` 타입 참조다. **B2 가 이 숫자를 완수의 증인으로 삼는다.**
- 어디가 걸리나: `crates/pal-cli/src/touch.rs:396` · `crates/pal-core/src/projection.rs:165-178`
- 획득: 조회 — 격리 사본에서 엣지를 세 갈래로 분류 실측
- 모집단: 저장소
- 유효성: 참
- 해악도: **거짓신호** (기존 결함이지만 계획이 그것을 완수 증인으로 승격시킨다)
- 대상: 계획대상
- 얼마나 아픈가: 되돌릴 수 있다(라벨 한 줄). 계획의 D2 는 「포맷 문자열 못 보는 몫」만 등록하고 이 몫을 등록하지 않는다

---

### `ImportSet` 은 **읽는 쪽이 하나도 없다** — 만들고 아무 데도 안 닿는 표면

- 어떻게 실패하나: `grep` 전수: `imports` 를 만지는 코드는 `file_graph.rs`(정의) · `cached.rs`(캐시 왕복) · `shell.rs`(능력 축) · `typescript.rs`(생산)뿐이다. **어떤 질의·투영·화면·게이트도 `ImportSet` 을 읽지 않는다**(`FileNode` 조차 `export_digest` 와 `refs` 만 싣는다 — `projection.rs:99-108`). 계획 4 와 A11(imports 네 갈래)·B3(`Capable::Present` 가 빈 값이 아니다)은 **자기가 만든 값을 자기가 확인하는 닫힌 고리**이고, 통과해도 관측 가능한 동작이 하나도 안 생긴다.
- 어디가 걸리나: `crates/pal-core/src/file_graph.rs:108-115` · 소비자 부재 · 완수 조건 A11·B3
- 획득: 조회 — `grep -rn "\.imports\b" crates`
- 모집단: 저장소
- 유효성: 참 (원 의도 #130 이 imports 를 명시하므로 「범위 밖」이라는 뜻은 아니다 — **증인이 기능을 안 잰다**는 뜻이다)
- 해악도: **거짓신호** — B3 초록이 「Rust 임포트가 선다」로 읽힌다
- 대상: 계획대상
- 얼마나 아픈가: 아프지 않다. 다만 「더 작은 표면으로 같은 답이 나오는가」의 답이 있다 — exports 는 `stitch_of`→EXPORTS·`export_digest`→`FileNode` 로 소비자가 있고 imports 는 없다. 둘을 같은 무게로 잡을 이유가 이 회차에는 없다

---

### `ScopeKind`·`ScopeChain` 의 문서 계약이 TypeScript 를 전제한다 — Rust 가 그 뜻을 조용히 넓힌다

- 어떻게 실패하나: `ScopeKind` 는 넷뿐이고 `Class` 는 *"클래스 본문 — 타입 파라미터와 멤버"* 로 정의돼 있다(`scope.rs:45`). Rust `impl` 을 여기 얹으면 1층 캐시에 직렬화되는 enum 값의 뜻이 **언어마다 갈린다.** 또 `ScopeChain` 은 *"**`scopes[0]` 이 모듈 스코프다**"*(`:154`)로 적혀 있는데 Rust 의 중첩 `mod` 를 `Module` 로 열면 한 파일에 Module 스코프가 여럿 선다 — `hoist_home` 이 그 값으로 분기하므로(`scopes.rs:151`) 뜻이 바뀌면 배치가 바뀐다. 어느 쪽을 고르든 문서와 코드 중 하나가 거짓이 되는데, 계획의 낡는 문면 목록(9)에 `pal-core/src/scope.rs` 가 **없다.**
- 어디가 걸리나: `crates/pal-core/src/scope.rs:40-51,152-156` · `crates/pal-extract/src/scopes.rs:134-145`
- 획득: 조회 — 타입 정의와 `hoist_home` 의 분기를 직접 읽음
- 모집단: 저장소
- 유효성: 참
- 해악도: **거짓신호** (되돌리기 비용은 ADR-0027 이 이미 「비싸다」고 적은 축이다 — 캐시에 실린다)
- 대상: 계획대상
- 얼마나 아픈가: 문서만이면 싸다. `ScopeKind::Impl` 을 더하면 직렬화 형식이 바뀌어 캐시 전량 무효(능력 축이 키에 있으므로 어차피 이번 커밋에 한 번 일어난다)

---

## 내가 기각한 것

| 무엇 | 어디 | 획득 | 모집단 | 유효성 | 해악도 | 대상 | 왜 기각했나 |
|---|---|---|---|---|---|---|---|
| 옛 캐시 항목이 `RestoreError::Shell`(키가 샜다)로 터진다 | `pal-extract/src/cached.rs:93-113` | 조회 — `shell.rs:148` `capability_axis()` 가 언어×자리를 전수 재현하고 `ledger.rs:104` 가 그것을 키 성분으로 넘김 | 저장소 | 거짓 | 실패 | 계획대상 | 능력 축이 바뀌면 **키 자체가 달라져** 옛 항목을 조회조차 안 한다. 대신 전 언어 캐시가 한 번 미스가 되는 비용만 남고 그것은 계획 14 가 수용했다 |
| 골든 대장(`corpus/golden/portal-backend.ledger.json`)이 능력 축 때문에 흔들린다 | `scripts/f01-verify.py:50` | 조회 — 골든 JSON 전수 확인(키: detector·entries·languages·repos_declared·scope·snapshot), `not-built`·`capab` 문자열 없음 | 저장소 | 거짓 | 실패 | 계획대상 | 능력 축이 골든에 안 실린다. Rust 등급도 `l1` 그대로라 `languages` 도 안 움직인다 |
| `crates/pal-cli/tests` 의 `.rs` 픽스처가 `cargo xtask check` 의 코어 어휘 금지에 걸린다 | `xtask/src/main.rs:806-832` | 조회 — `check_vocabulary` 가 `rust_sources(root/"crates/pal-core/src")` 만 훑는다 | 자기장치 | 거짓 | 실패 | 계획대상 | 모집단이 `pal-core/src` 뿐이다. C6 픽스처는 안 닿는다 |
| B1(기여 파일 80)이 도달 불가라 회차가 못 닫힌다 | 완수 조건 B1 | 조회 — 격리 사본 실측 126/134 | 자기장치 | 거짓 | 실패 | 계획자신 | 넉넉히 넘는다. **반대 방향이 문제였고**(위 B1 시나리오) 그 쪽으로 다시 적었다 |
| 엣지 0→3,752 로 `pal-query` 벤치의 선형성 합격선이 깨진다 | `crates/pal-query/tests/bench.rs:38-45` | 조회 — 벤치가 합성 그래프(`그래프(n)`)를 만들고 `--ignored` 다 | 저장소 | 거짓 | 실패 | 계획대상 | 실물 저장소를 안 읽는다. 규모도 스스로 만든다 |
| Rust 의 셋째 이름 공간(매크로)이 `Namespace{Value,Type}` 에 뭉개져 이 저장소에서 오해소를 낸다 | `pal-core/src/scope.rs:58-65` | 조회 — 134 파일에서 `macro_rules!` 이름과 동명 `fn`/`mod`/`struct`/`const` 를 전수 대조 → **0 건** | 저장소 | 거짓 | 금지역 | 계획대상 | 이 저장소에서는 **모집단이 0** 이라 실측으로 못 세운다. ⚠ 위험 자체는 ADR-0027:54 가 금지역으로 **등록**했고 첫 시나리오가 그 등록을 진다 — 여기서 기각한 것은 *"이 코퍼스에서 관측된다"* 는 주장뿐이다 |

새 범주: **등록된 금지역 목록을 되살리면서 그 등록 문서를 갱신 대상에 안 넣는 것** (ADR 이 「이 선택이 금지역 둘을 없앴다」고 적었는데 계획이 그 선택을 반쯤 뒤집고 ADR 을 낡는 문면 목록에서 뺀 자리)
