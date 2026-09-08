# 정(正) — `E1`·`E2`·`E3` 각각에 대해 지금 저장소에 서 있는 것이 그 조건 문면이 요구한 것을 냈나

> `pal-decision-proposer` 가 냈다. **초안이다 — 판정은 합(合)이 한다.**
> 실행 2026-09-08. 설계문 `dialectic/1-design.md` 전문을 받았고 그 형식에 맞춘다.

## 내가 연 것과 돌린 것 — 자기 신고 (설계문 격리 장치 1)

**연 파일**: `dialectic/1-design.md`(전문) · `intent.md` 의 `## 완수 조건`(:66-139) ·
`## 차선책`(:154-171) · `## 범위 밖`(:172-186) · `## 승격`(:271-286) · `## 개정`(:205-233) ·
`## 상한`(:187-204) · `## 착수 전에 연 이슈`(:235-…) · `observations/e1-sample-50.txt` ·
`observations/e1-hand-check.md` · `observations/v-variants.txt` ·
`premortem/r1-raw.md` · `conditions-audit/r2-raw.md` · `state.md` ·
`crates/pal-core/src/scope.rs` · `crates/pal-core/src/projection.rs` ·
`crates/pal-extract/src/scopes.rs` · `rust_scopes.rs` · `rust.rs`(시험 이름 목록) ·
`crates/pal-extract/examples/scope_variants.rs`(전문) ·
`crates/pal-cli/tests/rust_references.rs`(시험 이름) · `crates/pal-cli/src/touch.rs`(:395-407) ·
`crates/pal-cli/src/doctor.rs`(:303-316) · `docs/adr/0027-…md` 의 `## 개정`(:140-154) ·
`docs/gates/rust-extractor.md`(머리 5 줄) · 그리고 **표본 50 건이 가리키는 41 개 파일의 해당
구간과 그 파일 안의 선언 자리**.

**돌린 명령**:

    gh issue view 130
    cargo run -q --release -p pal-extract --example scope_variants -- . --표본   # rc=0
    cargo run -q --release -p pal-extract --example scope_variants -- .          # rc=0

두 출력 모두 회차 기록의 `observations/e1-sample-50.txt` · `observations/v-variants.txt` 와
**바이트 단위로 같다**(`diff` 무출력). 결정론이 실측으로 확인됐다.

**안 연 것**: `dialectic/1-antithesis.md` · `1-synthesis.md`(둘 다 아직 없다) ·
대화 기록(받지 않았다). **`git log` 과 커밋 메시지 본문은 안 열었다** — 이 초안의 근거는
전부 코드 줄 · 시험 이름 · 위 두 명령의 출력이다(격리 장치 2).

**받은 「메인의 입장」은 입력으로만 다뤘다.** 아래 `E1` 의 50 줄은 내가 다시 세운 것이고,
`E3` 에서는 메인의 입장과 **갈라선다**(그 자리를 명시했다).

**판 2 는 안 건드렸다** — `V9` 의 축이 정정인가 완화인가에 대한 판정은 이 산출에 없다(`N6`).

---

## 판정

**`E1` 을 `S1`(통과)로, `E2` 를 `S1`(통과)로, `E3` 을 `S5`(조건이 그 물음을 안 잰다)로 간다.**

---

## 근거 — 좌표와 함께

### `E1` — `S1`(통과)

조건 문면(`intent.md:136`): *"엣지 표본 **50 건**을 손으로 대조했을 때 가짜가 0 이다.
**표본 규칙**: 기여 파일 126 개에서 파일별 층화로 뽑고, 갈래마다 최소 10 건을 채운다."*

**근거 1 — 표본이 조건의 배분을 낸다.** 내 실행 출력 머리줄이
`E1 층화 표본 — 50 건 (타입 15 · 호출·매크로 20 · 그 밖 15 배분)` 이고, 실제 행을 세면
타입 15 · 호출·매크로 20 · 그밖 15 다(내가 다시 셌다). 배분을 고정하는 줄은
`crates/pal-extract/examples/scope_variants.rs:339` 이고, 뽑기는 `:343` 의
`v.sort_by(|a, b| (&a.경로, a.줄).cmp(&(&b.경로, b.줄)))` 뒤 `:347` 의
`간격 = v.len().div_ceil(몫).max(1)` 로 도는 **경로순 등간격**이다. 난수가 없다.

**근거 2 — 50 건이 41 개 파일 · 7 개 크레이트를 덮는다.** 내가 출력에서 센 분포:
`pal-cli` 21 · `pal-core` 18 · `pal-extract` 4 · `pal-store` 3 · `pal-intent` 2 ·
`pal-query` 1 · `xtask` 1. 한 파일에서 3 건이 하나(`crates/pal-core/src/plan.rs`), 2 건이 일곱.

**근거 3 — 50 줄을 내가 다시 대조했고 가짜가 0 이다.** 아래 표가 그것이다.
가짜의 정의는 설계문이 고정한 ⓐⓑⓒⓓ 를 쓴다.

**대조 방법**(반이 재현할 수 있게 적는다): ① 도착 이름이 소스 줄에 실제로 있는지 기계로
확인했다(50/50 있다) ② 그 이름의 **선언 자리**를 같은 파일에서 찾아 좌표를 달았다
③ 참조 자리와 선언 자리의 **모듈 사슬**을 중괄호 셈으로 뽑아 선언이 참조의 스코프에서
보이는지 확인했다 ④ `mod tests` 경계를 넘는 아홉 줄은 그 모듈의 `use super::*` 줄을
직접 확인했다 ⑤ 출발 심볼이 의심스러운 넷(17·18·4·2)은 파일을 열어 함수 경계를 눈으로 셌다.

| # | 갈래 | 경로:줄 | 이름 → 가는 곳 | 판정 | 근거 |
|--:|---|---|---|---|---|
| 1 | 타입 | `crates/pal-cli/src/attach.rs:81` | `attach` → `How` | 참 | `pub enum How` 가 같은 파일 `:38`. 파라미터 타입 자리다 |
| 2 | 타입 | `crates/pal-cli/src/install/layout.rs:82` | `PAYLOAD` → `Resource` | 참 | `pub struct Resource` `:21`. 구조체 리터럴 머리 |
| 3 | 타입 | `crates/pal-cli/src/label.rs:207` | `잔여_사유` → `Label` | 참 | `pub struct Label` `:45`. 리터럴 머리 |
| 4 | 타입 | `crates/pal-cli/src/round/ledger.rs:112` | `Event` → `EventRef` | 참 | `struct EventRef` `:140`. `enum Event` 의 `Judgment` 변형 필드 타입이고 출발 심볼이 그 enum 인 것이 맞다 |
| 5 | 타입 | `crates/pal-cli/src/round/verify.rs:204` | `finalize` → `FinalizeView` | 참 | `pub struct FinalizeView` `:56`. 반환 타입 |
| 6 | 타입 | `crates/pal-core/src/cascade.rs:101` | `cascade` → `Cascade` | 참 | `pub struct Cascade` `:83`. 반환 타입 |
| 7 | 타입 | `crates/pal-core/src/doctor.rs:739` | `inferred_carries_evidence` → `Outcome` | 참 | `pub struct Outcome` `:171`. 반환 타입 |
| 8 | 타입 | `crates/pal-core/src/glob.rs:77` | `from` → `Glob` | 참 | `pub struct Glob` `:57`. `impl From<Glob> for String` 의 `fn from(g: Glob)` 파라미터 |
| 9 | 타입 | `crates/pal-core/src/narrative.rs:682` | `동점이면_확정하지_않는다` → `RawSignals` | 참 | `pub struct RawSignals` `:72`. `mod tests`(`:574`)의 `use super::*`(`:575`)로 실제 Rust 에서도 같은 심볼 |
| 10 | 타입 | `crates/pal-core/src/plan.rs:1185` | `넷이_각각_성립한다` → `SnapshotView` | 참 | `pub struct SnapshotView<'a>` `:560`. `mod tests` `:980` · `use super::*` `:981` |
| 11 | 타입 | `crates/pal-core/src/schema.rs:304` | `RawEdge` → `RawCarrier` | 참 | `struct RawCarrier` `:311`. `Option<RawCarrier>` 필드 타입 |
| 12 | 타입 | `crates/pal-core/src/touch.rs:340` | `값이_있는_자리는_그대로_실린다` → `SymbolFacts` | 참 | `pub struct SymbolFacts` `:243`. `mod tests` `:323` · `use super::*` `:324` |
| 13 | 타입 | `crates/pal-extract/src/scopes.rs:246` | `declare_pass` → `ScopeRules` | 참 | `pub(crate) trait ScopeRules` `:62`. `&dyn ScopeRules` |
| 14 | 타입 | `crates/pal-intent/src/store.rs:449` | `keep_refusal` → `IntentError` | 참 | `pub enum IntentError` `:78`. `Result<(), IntentError>` |
| 15 | 타입 | `crates/pal-store/src/cache.rs:368` | `lookup` → `CacheKey` | 참 | `pub struct CacheKey` `:57`. `&CacheKey` |
| 16 | 호출·매크로 | `crates/pal-cli/build.rs:27` | `main` → `git_dir` | 참 | `fn git_dir` `:51`. 평범한 호출 |
| 17 | 호출·매크로 | `crates/pal-cli/src/install/blocks.rs:236` | `상태` → `자리` | 참 | `fn 자리` `:297`. 출발은 `pub fn 상태` `:231` 이고 동명 `pub enum 상태` `:116` 과 **다른 심볼**이다 — 그 바이트를 담는 가장 안쪽 심볼은 함수다 |
| 18 | 호출·매크로 | `crates/pal-cli/src/install/inside.rs:69` | `join` → `실제_경로` | 참 | `fn 실제_경로` `:186`. 출발 `pub fn join` `:60` |
| 19 | 호출·매크로 | `crates/pal-cli/src/query.rs:384` | `print_bindings` → `시각` | 참 | `fn 시각` `:402`. `println!` 안 호출 — `A4` 가 재는 형태 |
| 20 | 호출·매크로 | `crates/pal-cli/src/round/stop.rs:135` | `command_enable` → `print_view` | 참 | `fn print_view` `:672` |
| 21 | 호출·매크로 | `crates/pal-cli/tests/common/mod.rs:116` | `저장소` → `git` | 참 | `pub fn git` `:122` |
| 22 | 호출·매크로 | `crates/pal-cli/tests/install.rs:539` | `리소스를_하나도_못_찾으면_실패한다` → `값` | 참 | `fn 값` `:135` |
| 23 | 호출·매크로 | `crates/pal-cli/tests/install_hooks.rs:270` | `등록된_명령이_돌고_차단을_산출한다` → `프로젝트` | 참 | `fn 프로젝트` `:66` |
| 24 | 호출·매크로 | `crates/pal-cli/tests/install_recovery.rs:201` | `못_지운_디렉터리를_말한다` → `프로젝트` | 참 | `fn 프로젝트` `:20` |
| 25 | 호출·매크로 | `crates/pal-cli/tests/round_approve_verify.rs:763` | `실행된_현재_negative_control없이는_주조건도_met이_아니다` → `verify` | 참 | `fn verify` `:144` |
| 26 | 호출·매크로 | `crates/pal-cli/tests/round_status.rs:332` | `명시_round의_디렉터리와_intent_부재는_resolve_error다` → `status` | 참 | `fn status` `:51` |
| 27 | 호출·매크로 | `crates/pal-cli/tests/round_stop.rs:788` | `corrupt_progress와_trailing_partial_crlf를_보수적으로_판정한다` → `enable` | 참 | `fn enable` `:156` |
| 28 | 호출·매크로 | `crates/pal-core/src/cascade.rs:285` | `사슬` → `스냅샷` | 참 | `fn 스냅샷` `:243`. 참조와 선언이 같은 `mod tests` 안 |
| 29 | 호출·매크로 | `crates/pal-core/src/doctor.rs:1391` | `담을_수_없는_불변식은_위반_0_이_아니라_능력_부재다` → `성한` | 참 | `fn 성한` `:1103`. 둘 다 `mod tests` |
| 30 | 호출·매크로 | `crates/pal-core/src/plan.rs:1045` | `포매팅만_바뀌면_변경_심볼이_0_이고_본문이_바뀌면_1_이다` → `심볼` | 참 | `fn 심볼` `:987`. 둘 다 `mod tests` |
| 31 | 호출·매크로 | `crates/pal-core/src/schema.rs:750` | `공통_넷_각각이_없으면_엣지가_등록되지_않는다` → `성한` | 참 | `const 성한: &str` `:637`(둘 다 `mod tests`). ⚠ **갈래는 틀렸다** — `성한.replace(…)` 는 호출이 아니라 **수신자**이고 `scope_variants.rs:89` 의 `field_expression` 팔이 호출로 센다. 엣지 자체는 참이다 |
| 32 | 호출·매크로 | `crates/pal-extract/src/parse.rs:608` | `다음_선언` → `벗긴다` | 참 | `fn 벗긴다` `:691`. 둘 다 `mod tests` |
| 33 | 호출·매크로 | `crates/pal-extract/src/shell.rs:193` | `껍데기는_추출기의_상수와_같다` → `shell_of` | 참 | `pub fn shell_of` `:126`. `mod tests` `:186` · `use super::*` `:187` |
| 34 | 호출·매크로 | `crates/pal-intent/src/store.rs:423` | `keep_entity` → `IntentError` | 참 | `pub enum IntentError` `:78`. `IntentError::Decode(…)` 의 **경로 머리**이고 그 머리는 enum 을 가리킨다. ⚠ 갈래는 「호출」로 셌지만 열거 변형 생성이다 |
| 35 | 호출·매크로 | `xtask/src/main.rs:625` | `check` → `check_intent_untouched` | 참 | `fn check_intent_untouched` `:1021` |
| 36 | 그밖 | `crates/pal-cli/src/attach.rs:84` | `attach` → `How` | 참 | `pub enum How` `:38`. `How::Stitching` 의 머리 |
| 37 | 그밖 | `crates/pal-cli/src/install/manifest.rs:538` | `표식_모으기` → `표식` | 참 | `const 표식` `:487`. 둘 다 `mod tests` |
| 38 | 그밖 | `crates/pal-cli/src/round/status.rs:313` | `read_round` → `ConditionState` | 참 | `pub enum ConditionState` `:24`. match 팔 경로 머리 |
| 39 | 그밖 | `crates/pal-cli/tests/round_approve_verify.rs:350` | `종료직전_전수재실행은_이미_met인_command와_정반합_finding을_같이_닫는다` → `SLUG` | 참 | `const SLUG` `:14`. `json!` 매크로 토큰 열 안의 값 참조 |
| 40 | 그밖 | `crates/pal-core/src/binding.rs:649` | `evaluate` → `CodeFreshness` | 참 | `pub enum CodeFreshness` `:511`. `use super::CodeFreshness` 는 `:687`·`:734` 의 **시험 모듈 안**이라 이 자리를 안 가린다 |
| 41 | 그밖 | `crates/pal-core/src/derived.rs:429` | `대상_집합이_다르면_다른_노드다` → `DerivedId` | 참 | `pub struct DerivedId` `:90`. `mod tests` `:228` · `use super::*` `:231` |
| 42 | 그밖 | `crates/pal-core/src/envelope.rs:664` | `사유_넷과_상한_넷이_서로_다른_값이다` → `BudgetName` | 참 | `pub enum BudgetName` `:134`. `mod tests` `:611` · `use super::*` `:612` |
| 43 | 그밖 | `crates/pal-core/src/ledger.rs:459` | `칸의_합은_언제나_파일_총수다` → `LanguageId` | 참 | `pub struct LanguageId` `:30`. `mod tests` `:430` · `use super::*` `:431` |
| 44 | 그밖 | `crates/pal-core/src/plan.rs:658` | `by_name` → `UnresolvedWhy` | 참 | `pub enum UnresolvedWhy` `:374` |
| 45 | 그밖 | `crates/pal-core/src/repo.rs:425` | `스냅샷은_집합이고_비어_있을_수_없다` → `Snapshot` | 참 | `pub struct Snapshot` `:288`. `mod tests` `:354` · `use super::*` `:355` |
| 46 | 그밖 | `crates/pal-core/src/scope.rs:518` | `함수_경계를_지나면_뒤에_선_이름도_해소된다` → `Namespace` | 참 | `pub enum Namespace` `:76`. `matches!` 안 · `mod tests` `:439` · `use super::*` `:440` |
| 47 | 그밖 | `crates/pal-extract/src/narrative.rs:97` | `fragment` → `자리` | 참 | `enum 자리` `:119`. match 팔 머리 |
| 48 | 그밖 | `crates/pal-query/src/lib.rs:489` | `run` → `NamedQuery` | 참 | `pub enum NamedQuery` `:67`. match 팔 머리 |
| 49 | 그밖 | `crates/pal-store/src/projection.rs:317` | `swap` → `BY_NAME` | 참 | `const BY_NAME` `:54` |
| 50 | 그밖 | `crates/pal-store/src/projection.rs:794` | `clear_stage` → `SYMBOL_STAGE` | 참 | `const SYMBOL_STAGE` `:86` |

**참 50 · 가짜 0 · 못 정함 0.** 메인의 손 대조 표(`observations/e1-hand-check.md:37`)와 **수가
같다.** 다만 그 표는 갈래별로 뭉쳐 세 줄로 적혔고 이 표는 50 줄이 각각 선다.

**근거 4 — 왜 「참」이 참인가의 기계적 바닥.** 엣지는 `RefResolution::Bound` 이고 그 바인딩이
`BoundSymbol::Symbol` 일 때만 선다(`crates/pal-core/src/projection.rs:239-256`). `use` 로
들어온 이름은 `crates/pal-extract/src/rust_scopes.rs:443-444` 에서 묶이는데 그 자리는 심볼이
아니라 `BoundSymbol::NotASymbol` 이 되고(`crates/pal-extract/src/scopes.rs:190-192`),
그래서 **임포트된 이름은 원리상 엣지가 안 된다**(`projection.rs:252-255` 가 `locals` 로 센다).
표본 50 줄의 도착지가 전부 그 파일 안의 실제 선언인 것이 이 불변과 맞는다.

**근거 5 — 차선책이 발화하지 않는다.** `intent.md:160-163` 의 *"`E1` 의 표본 30 건에서
가짜가 나오면"* 은 가짜가 나올 때의 절차다. 가짜가 0 이므로 **문면의 「30 건」과 `E1` 의
「50 건」 불일치는 이 판정을 안 움직인다.** 불일치 자체는 사실이고 거짓신호 등급으로 적는다.

### `E2` — `S1`(통과)

조건 문면(`intent.md:137`): *"A~D 의 합이 이슈 #130 의 「무엇이 참이면 닫히나」 중 이 회차
몫(1·2·4)을 덮나"*. **내가 고른 읽기: 「조건 문면의 합이 이슈 문면을 덮나」**(덮개이지
통과 상태가 아니다). 이유는 `E2` 의 동사가 「덮나」이고, 통과 여부는 규약 §5 ①이 결정론적
자리에 두었기 때문이다.

`gh issue view 130` 의 원문 넷 중 이 회차 몫 셋:

| #130 | 문면 | 무엇이 덮나 | 좌표 |
|---|---|---|---|
| 1 | *"Rust 추출기가 한 파일 안의 스코프 사슬을 산출한다"* | `A1`(사슬 셋 이상 · `refs` 비지 않음) · `A2`(`impl` 이 스코프를 연다) · `A14`~`A17` · `B3` · 증인 `V1`~`V10` | 시험 실재: `crates/pal-extract/src/rust.rs:721`(`a1_스코프_사슬이_성립한다`) · `:728`(`a2_impl_이_스코프를_연다`) · `:866`(`a14`) · `:881`(`a15`) · `:888`(`a16`) · `:904`(`a17`) · `:927`(`b3_세_자리가_전부_present_이고_비지_않았다`) |
| 2 | *"임포트·익스포트를 산출한다"* | `A11`(`use` 네 갈래) · `A12`(정렬·중복제거 뒤 요약) · `A13`(`pub` 의 뜻) · `B3` · 통합 시험 | `rust.rs:800`·`:831`·`:845`·`:927` · `crates/pal-cli/tests/rust_references.rs:131`(`rust_의_내보내기가_최상위_pub_만_담는다`) |
| 4 | *"`pal export --format cypher` 를 돌렸을 때 `REFERENCES` 가 3 보다 크다"* | `B1` — *"재적재한 뒤 `pal export --format cypher` 의 `REFERENCES` 에 기여한 서로 다른 Rust 파일이 80 개 이상"*(`intent.md:107`) · 보강 `B2` | **문면이 이슈보다 강하다**: 서로 다른 파일 80 개가 각각 최소 한 엣지를 내면 `REFERENCES` ≥ 80 > 3 이다 |

**덮개에 구멍이 없다.** 셋 중 어느 것도 A~D 바깥에 남지 않고, 4 번은 조건 쪽이 이슈보다
엄격하다. #130 의 3 번(Kotlin)은 `intent.md:175` 의 `## 범위 밖` 이 `#131` 로 갈랐고
이슈 본문의 *"3 번은 `#131` 로 갈렸다"* 와 어긋나지 않는다.

⚠ **이 판정에 안 들어간 것**: 그 조건들이 실제로 통과했는가. `docs/gates/` 에 이 회차
이름의 게이트 문서가 없고(있는 것은 `docs/gates/rust-extractor.md` 이며 그 머리가
`회차: .palimpsest/rounds/2026-08-20-rust-extractor/` 로 **다른 회차**를 가리킨다),
회차 디렉터리의 `observations/` 에는 표본·변형 출력 셋뿐이다. **`B1`·`B2` 의 산출을 나는
못 봤다.** 그것이 `E2` 를 「통과 상태」로 읽었을 때의 값이고, 그 읽기에서는 `S4` 다.

### `E3` — `S5`(조건이 그 물음을 안 잰다)

조건 문면(`intent.md:138`): *"**소유자 답의 수가 회차 뒤에 참인가** — "동명 선언 72건이
각기 다른 스코프에 서서 가짜 엣지가 사라진다" 를 저장소 전체에서 다시 재서 0 인지 본다"*.

**설계문의 뒤집기 ⓒ 가 요구한 대로 「0 이 걸리는 대상 셋」을 각각 잰다.**

| 대상 | 재서 나온 값 | 그 대상이 조건의 뜻이면 이름 |
|---|---|---|
| ① 동명 아이템 재선언 **자리 수** | **기준 21(파일 10)** — 내 실행 출력의 `E3` 줄 | `S3` (0 이 아니다) |
| ② 그 자리가 만든 **가짜 엣지** 수 | **0** — `crates/pal-core/src/scope.rs:358-360` 이 `hoisted && 아이템_수 > 1` 에서 `RefResolution::Ambiguous` 를 돌려주고 `crates/pal-core/src/projection.rs:238` 이 그것을 세기만 하고 엣지를 안 만든다. 기준 실측 `모호 58` | `S1` |
| ③ **소유자가 말한 72 건이 갈라졌나** | **재현 안 된다** — 72 의 1 차 출처는 `premortem/r1-raw.md:18`(*"같은 모듈 스코프의 동명 재선언이 72 건 · 파일 22 개"*)이고 `:20` 이 그것을 잰 장치를 *"문법 스파이크가 134 파일을 파싱해 센 것"* 이라고 적는다. 지금 장치로 같은 형태(`impl` 을 안 여는 판)를 재면 **83(파일 30)** 이다. 코퍼스도 `.rs` **138 파일**로 달라졌다 | `S4` |

**셋이 서로 다른 답을 낸다. 설계문이 그 경우를 미리 `S5` 의 근거로 등록했다**(「음성 대조
ⓒ」). 그러므로 `E3` 은 어느 산출이 나오든 세 값 중 하나를 골라 통과로도 반증으로도 적을 수
있는 조건이고, 그 고름이 측정 뒤에 일어난다.

**추가 근거 — 조건 문면이 원인까지 함께 잠갔는데 그 원인이 실측과 다르다.** 인용문의 인과는
*"각기 다른 스코프에 서서 가짜 엣지가 사라진다"* 다. 그런데 `impl` 을 **안 여는** 판(`V3`)
에서도 그 자리들은 엣지를 안 만든다 — 같은 모듈 스코프에 동명 아이템 둘이 서면
`scope.rs:358` 이 `Ambiguous` 를 돌려주기 때문이다. 실측이 그것을 보인다: `V3` 의 엣지는
4185 이고 기준(4209) 대비 **가짜 7 · 누락 31** 이지, 「72 건이 되살아난다」가 아니다.
`impl` 이 스코프를 여는 것이 지우는 것은 **엣지가 아니라 모호 자리**(83 → 21)다.
가짜 엣지를 없앤 것은 `ResolveRule::Shadowing` 의 `Ambiguous` 팔이다
(`crates/pal-extract/src/rust_scopes.rs:149-151` 이 기준에서 그 규칙을 고른다).

★ **여기서 메인의 입장과 갈라선다.** 메인은 *"「가짜 엣지가 0 인가」와 「동명 선언 자리가
0 인가」는 다른 물음"* 이라고 적었고 나도 그 사실에는 동의한다. 갈리는 곳은 **처분**이다 —
메인은 그 갈림을 `E3` 판정의 재료로 두었고, 나는 그 갈림이 곧 `E3` 이 물음을 안 재고
있다는 뜻(`S5`)이라고 본다.

---

## 대안과 그것이 빠지는 근거 — 「지금 그대로」를 포함한다

| # | 대안 | 왜 안 골랐나 |
|---|---|---|
| 가 | **지금 그대로 — 셋 다 통과로 적고 회차를 닫는다** | `E1`·`E2` 는 그대로여도 서지만 `E3` 이 안 선다. 「0 인지 본다」의 대상 셋이 21 · 0 · 재현불가로 갈리는데 하나를 골라 통과로 적으면 그것이 금지역 ①(*"안 쟀거나 증거가 없는 것을 통과로"*)의 정확한 형태다 |
| 나 | `E1` 을 `S3`(반증)으로 — 표본 규칙이 문면과 다르다 | 문면은 *"기여 파일 126 개에서 파일별 층화"* 인데 구현은 **갈래 층화 + 경로순 등간격**이고 뽑힌 모집단도 도구의 **기여 파일 132** 다. 그러나 ⓐ 갈래마다 최소 10 건은 충족했고(15·20·15) ⓑ 경로순 등간격은 파일을 골고루 태우는 계통 추출이라 「파일별 층화」의 한 구현으로 읽힌다(41 파일) ⓒ 더 넓은 모집단에서 뽑는 것은 유리하게 고르는 방향이 아니다. **어긋남은 적되 반증으로는 안 간다** |
| 다 | `E1` 을 `S5` 로 — 표본 크기가 검출력을 안 잠근다 | `E1` 은 *"50 건에 가짜 0"* 이라 **위반이 가능하다**(가짜가 하나만 나와도 깨진다). 어떤 산출로도 위반을 못 내는 조건이 아니므로 `S5` 의 정의에 안 맞는다. 검출력 문제는 아래 「틀렸다면」에 넘긴다 |
| 라 | `E2` 를 `S4`(대조 불가)로 — `B1`·`B2` 의 산출이 저장소에 없다 | 그 읽기는 `E2` 의 동사를 「덮나」에서 「통과했나」로 바꾼다. 통과 여부는 규약 §5 ①의 결정론적 자리이고 `E2` 가 그 자리를 겹쳐 지면 `A`~`D` 를 두 번 세는 것이다. **다만 이 대안은 살아 있다** — 읽기가 갈리는 것 자체가 반의 재료다 |
| 마 | `E2` 를 `S5` 로 — 회차의 산출을 안 재고 잠긴 의도를 잰다 | 참인 지적이지만 `E2` 는 **거짓일 수 있다**(덮개에 구멍이 있으면 깨진다). 구멍을 찾았는데 없었다. 위반 가능성이 있으면 `S5` 가 아니다 |
| 바 | `E3` 을 `S1` 로 — ②만 뜻이라고 읽는다 | 인용문의 주어가 *"동명 선언 72건"* 이고 `## 개정`(`intent.md:226`)이 이 조건을 *"소유자 답의 수 72 를 다시 잰다"* 로 이름 붙였다. **수가 조건의 일부다.** ②만 뜻이라고 읽으면 「72」와 「저장소 전체에서 다시 재서」가 남는 낱말이 된다 |
| 사 | `E3` 을 `S3`(반증)으로 — 21 ≠ 0 | ①만 뜻이라고 읽는 것이고, 그러면 `A17`(`cfg` 쌍둥이는 엣지를 안 만든다)이 **조건끼리 충돌한다** — `A17` 은 그 자리가 남는 것을 옳다고 잠갔다. 한 회차의 조건 둘이 같은 자리에 반대 값을 요구하면 그것은 산출의 실패가 아니라 조건의 실패다 |
| 아 | `E3` 을 `S4`(대조 불가)로 — 72 를 잰 장치가 없다 | **가장 가까운 경쟁 이름이다.** ③ 하나만 보면 `S4` 가 맞다. 그런데 `S4` 로 적으면 *"표본 출력과 대조 표를 남기고 다시 재면 닫힌다"*(`P-d`)가 뒤따르는데, **다시 잴 것이 무엇인지가 정해져 있지 않다** — 그것이 `S5` 다. 그래서 `S5` 를 골랐고, 이 갈림은 합이 뒤집을 수 있다 |

---

## 음성 대조 — 설계문이 요구한 셋

**뒤집기 ⓐ — 표본을 깎아도 근거가 살아 있나.** 이 근거를 **표본 5 건**(위 표의 1·11·21·
36·46)에 그대로 적용하면 *"다섯 줄 다 참이다"* 가 같은 방식으로 성립한다. 즉 **줄 단위
논증은 n 에 안 매인다.** 50 이 5 보다 사는 것은 ⓐ 갈래 셋을 각각 10 건 이상 덮는다는 것과
ⓑ 41 파일 · 7 크레이트에 걸친다는 것뿐이고, **`E1` 은 그 두 가지를 합격선으로 안 적었다** —
적은 것은 「50 건에 가짜 0」이다. **그러므로 내 `S1` 은 「가짜가 없다」가 아니라 「이 50 줄에
가짜가 없다」를 말한다.** 이 한계는 조건의 것이지 산출의 것이 아니다.

**뒤집기 ⓑ — 무엇이 있었으면 반대 이름이었나.**

- `E1`(`S1`) → **한 줄이라도** 다음이면 `S3` 이었다: 도착 선언이 그 파일에 없거나(ⓐ),
  그 토큰이 선언 이름·필드·경로 꼬리·속성 안이거나(ⓑ), 출발 심볼이 그 바이트를 안 담거나(ⓒ),
  `use` 로 들어온 이름인데 동명 지역 선언으로 갔거나(ⓓ). 가장 위험했던 자리는 40 번
  (`binding.rs` 의 `CodeFreshness` — 같은 파일에 `use super::CodeFreshness` 가 둘 있다)이고,
  그 `use` 가 **시험 모듈 밖**에 있었으면 나는 그 줄을 「못 정함」으로 적었을 것이다.
- `E2`(`S1`) → `A`~`D` 어디에도 *"임포트·익스포트를 산출한다"* 를 재는 조건이 없었으면 `S3`.
  실제로 `A11`·`A12`·`A13` 이 있고 시험 이름이 `rust.rs:800`·`:831`·`:845` 에 선다.
- `E3`(`S5`) → 세 대상이 **같은 답**을 냈으면 `S5` 가 아니었다. 구체적으로 ① 이 0 이었으면
  `S1` 이고, 72 를 잰 문법 스파이크가 저장소에 남아 다시 돌 수 있었으면 `S1`/`S3` 중 하나로
  갈렸을 것이다.

**뒤집기 ⓒ — `E3` 의 0 은 무엇의 0 인가.** 위 `E3` 절의 표가 그것이고, 셋이 21 · 0 ·
재현불가로 갈린다.

---

## 이 초안이 틀렸다면 무엇이 먼저 드러나나

1. **`E1` 의 손 대조 방법이 얕다.** 나는 도착 이름을 소스 줄에서 찾고 그 이름의 선언을
   같은 파일에서 grep 으로 찾았을 뿐, **`r.at` 바이트를 직접 열지 않았다.** 한 줄에 같은
   이름이 둘 이상 있으면 내가 다른 토큰을 보고 「참」이라고 적었을 수 있다. 반이 15 건을
   다시 뽑을 때 **바이트 자리로** 확인하면 이 초안이 먼저 깨진다.
2. **`mod tests` 경계를 넘는 아홉 줄(9·10·12·33·41·42·43·45·46)의 해소 기제가 Rust 규칙이
   아니다.** 도구는 중첩 `mod` 스코프에서 **바깥으로 걸어 나가** 이름을 찾는데
   (`crates/pal-core/src/scope.rs:363-366`), Rust 의 모듈은 렉시컬 상속을 안 한다.
   이 아홉 줄은 `use super::*` 가 있어서 **결과가 우연히 같았다.** `use super::*` 가 없는
   중첩 `mod` 와 바깥의 동명 아이템이 저장소 어딘가에 함께 있으면 그것은 ⓓ 가짜다 —
   **나는 그 자리를 저장소 전체에서 세지 않았다.** 반이 세면 `E1` 이 흔들린다.
3. **`E2` 가 회차의 산출을 하나도 안 잰다.** `E2` 의 참·거짓은 잠긴 의도가 잠긴 순간
   정해졌고 그 뒤 어떤 코드도 그것을 못 바꾼다. 「회차가 의도를 냈나」를 묻는 자리에
   착수 시점 문서 감사가 들어와 있다는 지적이 서면 내 `S1` 은 이름이 틀린 것이 된다.
4. **`E3` 의 `S5` 가 `S4` 에 질 수 있다.** 「대상이 셋이다」는 내 읽기이고, 조건 문면의
   마지막 절이 *"0 인지 본다"* 라 ②만 가리킨다고 읽을 여지가 있다. 그 읽기가 서면 `E3` 은
   `S1` 이고 내 초안의 셋 중 하나가 통째로 뒤집힌다.
5. **`V3` 실측의 해석이 틀렸을 수 있다.** 나는 *"`impl` 이 여는 것은 엣지가 아니라 모호
   자리를 지운다"* 를 `V3` 의 「가짜 7 · 누락 31」에서 읽었는데, 그 7 과 31 이 동명 재선언
   자리에서 온 것인지는 **안 갈랐다.** 갈라 보면 다른 그림이 나올 수 있다.
6. **갈래 분류가 이름대로가 아니다**(31 · 34 번 행). `scope_variants.rs:71-93` 의 `호출인가`
   는 메서드 **수신자**와 열거 변형 **생성**을 호출로 센다. `E1` 의 층화 축이 그 분류라
   *"갈래마다 최소 10 건"* 이 무엇을 10 건 채운 것인지가 흔들린다. 이것이 `E1` 을 `S2` 나
   `S5` 쪽으로 밀 수 있다.

---

## 내가 확인 못 한 것

- **`pal export --format cypher` 를 안 돌렸다.** `REFERENCES` 4,179 도 기여 파일 128 도
  126 도 내 눈으로 안 봤다. 내가 본 수는 도구 출력의 **엣지 4209 · 기여 파일 132** 뿐이고,
  `E1` 문면의 「기여 파일 126 개」와 다르다. 그 차의 까닭을 나는 모른다.
- **표본의 모집단은 「엣지」가 아니라 「참조」다.** `scope_variants.rs:307-337` 의 `표본()`
  은 `chain.refs` 를 훑어 중복 제거 없이 담고, 그 총수가 출력의 `참조 6591` 이다.
  같은 실행의 엣지 집합은 4209(중복 제거된 세 쌍)이다. 그러므로 50 건은 **6,591 중 50**
  (0.76%)이고 `observations/e1-hand-check.md:46` 이 적은 *"엣지 4,179 의 1.2%"* 와 분모가
  다르다. 어느 분모가 `E1` 의 뜻인지 나는 못 정했다.
- **`A`~`D` 의 통과 여부를 안 쟀다.** `cargo xtask check` · `cargo xtask test` 를 안 돌렸다.
  설계문이 그 자리를 이 판 밖에 두었다.
- **`V10` 의 엣지 +31 이 어느 몫인지 안 갈랐다.** `rust_scopes.rs:149-151` 이 `V10` 에서
  해소 규칙 전체를 `Tdz` 로 바꾸므로 그 31 은 `cfg` 쌍둥이 몫과 규칙 교체 몫이 섞여 있다.
- **저장소 전체의 가짜 엣지율을 안 쟀다.** `E1` 이 표본까지만 요구하고 `intent.md:180` 의
  `## 범위 밖` 이 전수 측정을 뺐다 — 그 줄이 `E1` 을 여전히 **「표본 30 건」**으로 부른다.
- **`docs/gates/` 에 이 회차 게이트 문서가 없는 것**은 확인했지만, 그것이 회차 종료 조건에
  무엇을 하는지는 이 판이 지는 물음이 아니다.
- **판 2(`V9` 의 축)를 안 봤다.** `rust_scopes.rs:245` 와 `:277` 이 `use` 를 선언과 배제
  양쪽에 두는 것은 위에서 근거로만 인용했고 처분을 안 정했다.
