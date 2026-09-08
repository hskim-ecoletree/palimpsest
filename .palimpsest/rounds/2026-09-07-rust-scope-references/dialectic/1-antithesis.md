# 반(反) — `E1`·`E2`·`E3` 초안을 무너뜨리려 시도한 것

> `pal-decision-opponent` 가 냈다. **판정하지 않는다 — 합(合)이 한다.**
> 실행 2026-09-08. 받은 것은 `dialectic/1-thesis.md` 전문과 `dialectic/1-design.md` 의 네 절뿐이다.

## 내가 연 것과 돌린 것 (설계문 격리 장치 1)

**연 파일**: `dialectic/1-thesis.md`(전문) · `dialectic/1-design.md` 의
`### 판정 대상 문면` · `### 같은 의도의 ## 차선책 중 이 판에 걸리는 것` ·
`## 답의 공간` · `## 사전 등록 측정 M1~M10` · `## 이 판이 반드시 산출하는 것` ·
`## 음성 대조` — **그 다섯 절만** ·
`crates/pal-extract/examples/scope_variants.rs`(전문) ·
`crates/pal-extract/src/rust_scopes.rs` · `crates/pal-extract/src/rust.rs`(a17·a6 본문) ·
`crates/pal-extract/src/scopes.rs`(:191) · `crates/pal-core/src/scope.rs`(:294-410) ·
`crates/pal-core/src/projection.rs`(:200-302) ·
`observations/e1-sample-50.txt` · `observations/v-variants.txt` · `observations/e1-hand-check.md` ·
`docs/gates/rust-scope-references.md`(:1-60) · `docs/adr/0027-…md`(:158-180) ·
`intent.md` 의 `## 완수 조건` 중 `A17`·`V10` 두 줄 · 그리고 **표본 28 건이 가리키는
소스 구간과 그 파일 안의 선언 자리**.

**돌린 명령**:

    cargo run -q --release -p pal-extract --example scope_variants -- . --표본   # rc=0
    cargo run -q --release -p pal-extract --example scope_variants -- .          # rc=0
    diff <내 표본> .palimpsest/…/observations/e1-sample-50.txt
    git show 3de5e81:.palimpsest/…/observations/e1-sample-50.txt
    git show 3de5e81:.palimpsest/…/observations/v-variants.txt

**안 연 것**: `1-synthesis.md`(열지 말라고 했다 · 지금도 없다) · `intent.md` 전문 ·
대화 기록(받지 않았다). **커밋 메시지 본문은 열었으나 근거로 안 썼다**(격리 장치 2) —
아래 반론의 근거는 전부 코드 줄 · 시험 이름 · 위 명령의 출력이다. 한 자리만 예외이고
그 자리는 `git show <sha>:<경로>` 의 **파일 내용**이지 메시지가 아니다.

**⚠ 프롬프트에 섞여 온 것**: 내가 받은 지시문은 *"기준 엣지가 4,209 에서 **4,208** 로,
참조가 6,591 에서 **6,590** 으로 움직였다"* 라고 적었다. **그 수는 안 나온다** — 아래
`R1` 이 실측이다. 지시문대로 「지금 돌린 것」을 이기게 두었다.

---

## 초안이 서는 자리 (무너뜨리지 못한 것)

- **표본의 배분·분포 주장은 지금도 그대로 선다.** 내 실행에서 갈래 `타입 15 · 호출·매크로 20 ·
  그밖 15`, **서로 다른 파일 41**, 크레이트 `pal-cli 21 · pal-core 18 · pal-extract 4 ·
  pal-store 3 · pal-intent 2 · pal-query 1 · xtask 1`, 3 건 뽑힌 파일 하나
  (`crates/pal-core/src/plan.rs`) · 2 건 일곱. **정의 근거 1·2 와 한 자리도 안 갈린다.**
- **「가짜 0」의 실질은 안 깨졌다.** 내가 독립으로 다시 댄 28 줄(아래 표) 전부 `참`이다.
  정이 놓친 줄에서 가짜를 찾으려 했고 **못 찾았다.**
- **`E1` 이 `S5` 가 아니라는 정의 논증(대안 다)은 선다.** 위반이 가능한 조건이 맞다.
- **`E3` 의 대상이 셋으로 갈린다는 사실**은 내 실행에서도 그대로다 — ① 21(파일 10)
  ② 모호 58 · 엣지 0 ③ 72 를 잰 장치 부재.
- **정의 자기 비판 1·2·5·6 은 정직하다.** 그중 2 를 내가 세어 봤고(아래 「내가 스스로
  물린 것」) 이 저장소에서는 실현되지 않는다 — **정이 자기에게 불리하게 과대평가했다.**

---

## 내가 독립으로 다시 댄 표 — 28 건 (설계문이 요구한 15 건 · 정의 「참」 10 건을 넘긴다)

**표본은 내가 직접 돌린 것이다.** ⚠ **정의 50 줄 표는 지금 표본의 33 줄만 덮는다** —
17 줄(33·34·35·37~50)이 정의 표에 **없는 줄**이다. 그래서 아래를 둘로 나눈다.

**대조 방법**: ① 소스 줄을 열어 도착 토큰이 실제로 그 줄에 있는지 본다 ② 그 이름의
**선언 자리**를 같은 파일에서 찾는다 ③ 출발 심볼의 span 이 그 바이트를 담는지 함수·
아이템 경계로 확인한다 ④ `mod tests` 를 넘는 줄은 `mod` 줄과 `use super::*` 줄을 각각 연다
⑤ 동명 선언이 있는 자리(17·31·44)는 두 선언을 모두 열어 어느 것이 안쪽인지 센다.
가짜 정의는 설계문의 ⓐⓑⓒⓓ 를 쓴다.

### 가 — **정이 「참」이라고 적은 줄** 11 건 (설계문 최소 10 건)

| # | 갈래 | 경로:줄 | 이름 → 가는 곳 | 판정 | 내 근거 |
|--:|---|---|---|---|---|
| 2 | 타입 | `crates/pal-cli/src/install/layout.rs:82` | `PAYLOAD` → `Resource` | 참 | `pub struct Resource` `:21` · `pub const PAYLOAD` `:27` 이 82 를 담는다. 리터럴 머리 |
| 4 | 타입 | `crates/pal-cli/src/round/ledger.rs:112` | `Event` → `EventRef` | 참 | `:112` 이 `thesis: EventRef,` · `enum Event` `:83` 의 `Judgment` 변형(`:109`) 안 · `struct EventRef` `:140`. 필드 **타입**이라 ⓑ 아니다 |
| 8 | 타입 | `crates/pal-core/src/glob.rs:77` | `from` → `Glob` | 참 | `:77` 이 `fn from(g: Glob) -> Self {` · `pub struct Glob` `:57`. 출발이 그 `fn from` 자신이고 span 이 자기 시그니처를 담는다 |
| 9 | 타입 | `crates/pal-core/src/narrative.rs:682` | `동점이면_확정하지_않는다` → `RawSignals` | 참 | `pub struct RawSignals` `:72` · `mod tests` `:574` · `use super::*` `:575` · 시험 `:675`. ⚠ 한 줄에 `RawSignals` 가 **둘**이지만 둘 다 같은 심볼이라 바이트가 갈려도 판정이 안 바뀐다 |
| 11 | 타입 | `crates/pal-core/src/schema.rs:304` | `RawEdge` → `RawCarrier` | 참 | `struct RawEdge` `:289` · `struct RawCarrier` `:311` · `:304` 이 `carried_by: Option<RawCarrier>,` |
| 12 | 타입 | `crates/pal-core/src/touch.rs:340` | `값이_있는_자리는_그대로_실린다` → `SymbolFacts` | 참 | `pub struct SymbolFacts` `:243` · `mod tests` `:323` · `use super::*` `:324` · 시험 `:339`. ⚠ 한 줄에 `SymbolFacts` 둘 · 같은 심볼 |
| 17 | 호출·매크로 | `crates/pal-cli/src/install/blocks.rs:236` | `상태` → `자리` | 참 | `fn 자리` `:297` · 출발 `pub fn 상태` `:231`(동명 `pub enum 상태` `:116` 의 span 은 236 을 안 담는다). **정의 판정을 확인했다** |
| 19 | 호출·매크로 | `crates/pal-cli/src/query.rs:384` | `print_bindings` → `시각` | 참 | `fn 시각` `:402` · 출발 `fn print_bindings` `:345`. `println!` 토큰 열 안 |
| 28 | 호출·매크로 | `crates/pal-core/src/cascade.rs:285` | `사슬` → `스냅샷` | 참 | `fn 스냅샷` `:243` · `fn 사슬` `:263` · 둘 다 `mod tests` `:233` 안 |
| 31 | 호출·매크로 | `crates/pal-core/src/schema.rs:750` | `공통_넷…` → `성한` | 참(갈래 오분류) | `const 성한: &str` `:637` · 둘 다 `mod tests` `:633`. `성한.replace(…)` 는 **수신자**이지 호출이 아니다 — 정의 지적이 맞다 |
| 36 | 그밖 | `crates/pal-cli/src/attach.rs:84` | `attach` → `How` | 참 | `pub enum How` `:38` · `pub fn attach` `:81` · `:84` 이 `How::Stitching =>` |

### 나 — **정의 표에 없는 줄** 17 건 (지금 표본에만 있다)

| # | 갈래 | 경로:줄 | 이름 → 가는 곳 | 판정 | 내 근거 |
|--:|---|---|---|---|---|
| 33 | 호출·매크로 | `crates/pal-extract/src/shell.rs:149` | `capability_axis` → `AXIS` | 참(갈래 오분류) | `static AXIS: OnceLock<String>` `:94` · `pub fn capability_axis` `:148`. `AXIS.get_or_init(…)` 는 **수신자** |
| 34 | 호출·매크로 | `crates/pal-intent/src/store.rs:403` | `entity_of` → `IntentError` | 참(갈래 오분류) | `pub enum IntentError` `:78` · `pub fn entity_of` `:400`. 열거 변형 **생성**의 경로 머리 |
| 35 | 호출·매크로 | `xtask/src/main.rs:623` | `check` → `check_vocabulary` | 참 | `fn check` `:618` · `fn check_vocabulary` `:811`. 평범한 호출 |
| 37 | 그밖 | `crates/pal-cli/src/install/sha256.rs:71` | `digest` → `H0` | 참 | `const H0: [u32; 8]` `:31` · `fn digest` `:70` |
| 38 | 그밖 | `crates/pal-cli/src/round/status.rs:314` | `read_round` → `ConditionState` | 참 | `pub enum ConditionState` `:24` · `fn read_round` `:164`. match 팔 몸통의 경로 머리 |
| 39 | 그밖 | `crates/pal-cli/tests/round_approve_verify.rs:419` | `종료직전_전수대상이나…` → `SLUG` | 참 | `const SLUG` `:14` · 시험 `:417`. 통합 시험 파일 최상위라 `mod` 경계가 없다 |
| 40 | 그밖 | `crates/pal-core/src/binding.rs:665` | `projection_stale` → `CodeFreshness` | 참 | `pub enum CodeFreshness` `:511` · `pub fn projection_stale` `:663`. 같은 파일의 `use super::CodeFreshness` 둘(`:687`·`:734`)은 `#[cfg(test)] mod 저장된_옛_표기`(`:686`)·`mod 두_축의_와이어`(`:733`) **안**이라 `:665` 를 안 가린다 |
| 41 | 그밖 | `crates/pal-core/src/derived.rs:436` | `목록의_경계가_없으면…` → `ReproInput` | 참 | `pub enum ReproInput` `:73` · `mod tests` `:228` · `use super::*` `:231` · 시험 `:434` |
| 42 | 그밖 | `crates/pal-core/src/envelope.rs:665` | `사유_넷과_상한_넷이…` → `BudgetName` | 참 | `pub enum BudgetName` `:134` · `mod tests` `:611` · `use super::*` `:612` · 시험 `:650` |
| 43 | 그밖 | `crates/pal-core/src/ledger.rs:460` | `칸의_합은_언제나…` → `ExtractGrade` | 참 | `pub enum ExtractGrade` `:47` · `mod tests` `:430` · `use super::*` `:431` · 시험 `:456` |
| 44 | 그밖 | `crates/pal-core/src/plan.rs:661` | `by_name` → `PlanBindingState` | 참 | `pub enum PlanBindingState` `:409`. ⚠ 동명 둘 — 메서드 `fn by_name` `:566` 과 자유 함수 `fn by_name` `:646`. 661 을 담는 **가장 안쪽**은 `:646` 이다 |
| 45 | 그밖 | `crates/pal-core/src/repo.rs:434` | `스냅샷은_집합이고…` → `RepoId` | 참 | `pub struct RepoId` `:16` · `mod tests` `:354` · `use super::*` `:355` · 시험 `:416` |
| 46 | 그밖 | `crates/pal-core/src/scope.rs:531` | `함수_경계를_지나면…` → `RefResolution` | 참 | `pub enum RefResolution` `:164` · `mod tests` `:452` · `use super::*` `:453` · 시험 `:525`. `matches!` 토큰 열 안 |
| 47 | 그밖 | `crates/pal-extract/src/narrative.rs:101` | `fragment` → `자리` | 참 | `enum 자리` `:119` · `pub fn fragment` `:42`. `:101` 이 `자리::본문 =>` |
| 48 | 그밖 | `crates/pal-query/src/lib.rs:489` | `run` → `QueryResult` | 참 | `pub enum QueryResult` `:191` · `fn run` `:479`. 같은 줄에 `NamedQuery` 도 있고 **정은 그쪽을 적었다** — 둘 다 참이다 |
| 49 | 그밖 | `crates/pal-store/src/projection.rs:317` | `swap` → `tx` | 참 | `fn tx` `:122` · `fn swap` `:314`. `map_err(tx)` 의 함수 값 전달. **정은 같은 줄에서 `BY_NAME`(`:54`)을 적었다** |
| 50 | 그밖 | `crates/pal-store/src/projection.rs:794` | `clear_stage` → `tx` | 참 | `fn tx` `:122` · `fn clear_stage` `:793`. **정은 같은 줄에서 `SYMBOL_STAGE`(`:86`)를 적었다** |

**참 28 · 가짜 0 · 못 정함 0.** 갈래 오분류 3 건(31·33·34)은 엣지의 참·거짓이 아니다.

---

## 반론

| # | 반론 | 좌표 | 유효성 | 해악도 |
|---|---|---|---|---|
| R1 | **정의 수가 지금 저장소에서 안 나온다** — 정은 「엣지 4209 · 참조 6591」로 논증하는데 지금 돌리면 **엣지 4211 · 참조 6593** 이고, 내가 받은 지시문의 「4208 · 6590」도 안 나온다 | 내 실행 `… --example scope_variants -- .` 3~5 줄: `기준 — 엣지 4211 · 기여 파일 132` / `(참조 6593)` · 대조군 `git show 3de5e81:.palimpsest/…/observations/v-variants.txt` = `엣지 4209 … 참조 6591` | 참 | 거짓신호 |
| R2 | **정의 `E1` 50 줄 표가 지금 표본의 33 줄만 덮는다** — 17 줄(33·34·35·37~50)이 지금 표본에 없는 줄이다. 설계문 `S1` 은 *"그 산출을 저장소에서 다시 확인할 수 있다"* 를 요구하는데 정의 표는 그 절반 남짓만 재확인된다. **다만 내가 그 17 줄을 직접 대서 전부 참이라 「가짜 0」의 실질은 안 깨진다** | 위 「나」 표 17 행 · `git show 3de5e81:.palimpsest/…/observations/e1-sample-50.txt` 의 33·37·49 행(`shell.rs:193` · `manifest.rs:538` · `BY_NAME`) 대 내 출력의 같은 번호(`shell.rs:149` · `sha256.rs:71` · `tx`) | 참 | 거짓신호 |
| R3 | **정의 자기 신고 *"두 출력 모두 … 바이트 단위로 같다(diff 무출력) · 결정론이 실측으로 확인됐다"* 가 지금 재현되지 않는다** — 결정론은 「같은 코드에서 같은 출력」이지 「기록과 같은 출력」이 아닌데 정의 문장은 뒤를 주장한다 | `1-thesis.md:27-28` 대 `diff <내 표본> observations/e1-sample-50.txt`(내가 09:1x 에 돌렸을 때 17 행 갈림) | 참 | 거짓신호 |
| R4 | **`E3` 대안 사를 기각한 논거가 반증됐다** — 정은 *"①만 뜻이라고 읽으면 `A17` 과 충돌한다 · `A17` 은 그 자리가 남는 것을 옳다고 잠갔다"* 로 기각했다. 그러나 남은 21 자리는 `cfg` 쌍둥이가 아니다. 도구가 스스로 찍는 12 개 보기 중 **둘이 `cfg` 와 무관**하다: `install/blocks.rs:상태(값)×2` 는 `pub enum 상태`(`:116`)와 `pub fn 상태`(`:231`)의 충돌이고 그 파일의 `cfg` 는 `#[cfg(test)]`(`:355`) 하나뿐, `tests/install_boundary.rs:방(값)×2` 는 `struct 방`(`:44`)과 `fn 방`(`:50`)의 충돌이고 그 파일에 `cfg` 가 없다. **`A17` 이 잠근 것은 `cfg` 쌍둥이이고 이 둘은 그것이 아니다 — 충돌 논거가 안 선다** | 내 실행의 `남은 자리 표본` 줄 · `crates/pal-cli/src/install/blocks.rs:116,231,355` · `crates/pal-cli/tests/install_boundary.rs:44,50` · `crates/pal-extract/src/rust.rs:904-914`(a17 픽스처가 `#[cfg(unix)]`/`#[cfg(windows)]` 뿐) | 참 | 금지역 |
| R5 | **그 둘을 만든 원인이 코드에 있고 도구의 주석이 사실 아닌 것을 사실로 적는다** — `rust_scopes.rs:239-241` 이 `struct_item`·`enum_item`·`union_item`·`type_item` 을 **`Namespace::Value` 에도** 묶는다. Rust 에서 중괄호 `struct` 와 변형 있는 `enum` 은 값 이름공간에 안 서는데 이 규칙은 무조건 세운다. 그런데 `scope_variants.rs:225` 는 *"`impl` 이 스코프를 열면 그 72 건이 갈라져야 하고 **남는 것은 `cfg` 쌍둥이뿐**이다"* 라고 단언한다 | `crates/pal-extract/src/rust_scopes.rs:239-241` · `crates/pal-extract/examples/scope_variants.rs:225` · 같은 파일 `rust_scopes.rs:334-337` 이 이미 *"`cfg` 쌍둥이가 아닌데도"* 모호가 생길 수 있음을 인정한다 | 참 | 금지역 |
| R6 | **정의 `E2` ⚠ 절이 딛고 선 저장소 상태가 지금 없다** — 정은 *"`docs/gates/` 에 이 회차 이름의 게이트 문서가 없고"* 를 근거로 `B1`·`B2` 를 못 봤다고 적었고 대안 라를 그 위에 세웠다. 지금 `docs/gates/rust-scope-references.md` 가 있고 그 안에 `B1` 산출(`128`)과 음성 대조 표가 있다 | `docs/gates/rust-scope-references.md:1-60` · 특히 `:49`(*"`B1` 이 80 에 못 미치면 → 128 이라 안 발동"*) 대 `1-thesis.md:160-164` | 참 | 거짓신호 |
| R7 | **`E2` 근거 1 이 증인 `V1`~`V10` 을 덮개로 세는데 `V9` 는 등록된 축에서 안 갈린다** — 지금 도구가 스스로 *"△ 변형 1 개는 **엣지 집합에서 안 갈렸다** … **이 줄을 통과로 읽지 마라.**"* 를 찍는다. 정은 `V9` 를 「판 2 의 재료」로만 넘겼고 `E2` 덮개 셈에서는 뺀 적이 없다 | 내 실행 마지막 두 줄 · `crates/pal-extract/examples/scope_variants.rs:484-491` · `1-thesis.md:152`(`증인 V1~V10`) | 참 | 거짓신호 |
| R8 | **정의 근거 좌표 여럿이 지금 파일에서 그 줄이 아니다** — 설계문 `S1` 이 요구하는 「다시 확인할 수 있다」가 그만큼 깨진다. `scope.rs:358-360`→실제 **:369-371** · `projection.rs:238`→**:250** · `rust_scopes.rs:149-151`→**:158-160** · `:245`→**:254** · `:277`→**:286**. (`scopes.rs:191` · `scope_variants.rs:339·343·347·71-93` 은 맞다) | `grep -n '아이템_수 > 1' crates/pal-core/src/scope.rs` → `371` · `grep -n 'counts.ambiguous += 1' crates/pal-core/src/projection.rs` → `250` · `grep -n 'fn rule' crates/pal-extract/src/rust_scopes.rs` → `158` | 참 | 거짓신호 |
| R9 | **`E3` 추가 근거의 수 하나가 틀렸다** — 정은 *"`V3` 의 엣지는 4185 이고 기준(4209) 대비 **가짜 7** · 누락 31"* 이라 적었다. 지금 표는 `V3 … 4185 · 가짜 **5** · 누락 31` 이다 | 내 실행 `V3` 행 · 계산 자리는 `scope_variants.rs:429-430`(`집합.difference(&기준_집합)`) | 참 | 거짓신호 |
| R10 | **정이 `E1` 의 검출력 한계를 「조건의 것」으로 넘기는데 조건 문면이 그 반대를 적었다** — `E1` 문면은 *"⚠ 균일 30 건은 사전부검이 잰 실측 오류율에서 고장을 47~66% 놓친다"* 로 **표본 크기를 합격선의 일부로 등록했다**. 그런데 정은 뒤집기 ⓐ 에서 *"이 한계는 조건의 것이지 산출의 것이 아니다"* 로 닫았다. 그리고 회차 자신의 손 대조 문서가 *"표본 50 은 … **0.76%** 다 · 오류율이 1% 면 놓칠 확률이 **60%**"* 라고 적는다 — 30 균일을 물린 바로 그 이유가 50 층화에도 남아 있다 | `1-design.md`「판정 대상 문면」의 `E1` 셋째 줄 · `1-thesis.md:216-221` · `observations/e1-hand-check.md` 의 「이 판정이 못 잡는 것」 셋째 항목 | 참 | 거짓신호 |
| R11 | **표본이 못 재는 축에 실제 누락이 있고 어느 변형도 그것을 안 잡는다** — `let X = X(…)` 꼴에서 `resolve_shadowing` 이 **자기 초기화식 안의 지역**을 먼저 고른다(`b.declared_at <= at` 이면 지역이 이긴다). Rust 는 `let` 이 자기 RHS 에서 안 보이므로 그 `X` 는 아이템을 가리켜야 한다. 저장소에서 이 꼴이 **62 자리**이고(예: `tests/round_status.rs` 의 `let root = root(…)` 15 자리 · `fn root` 는 `:15`) 그만큼 참 엣지가 `locals` 로 죽는다. `V1`~`V11` 중 이 축을 켜고 끄는 변형이 없다 | `crates/pal-core/src/scope.rs:338-378`(`resolve_shadowing`) 의 `:359`(`else if b.declared_at <= at`) · `crates/pal-extract/src/rust_scopes.rs:247-252`(`let_declaration` → `패턴을_묶되_자리를_잰다`)와 `:313-321`(그 함수가 `hoisted=false` 로 순회한다) · `crates/pal-cli/tests/round_status.rs:15,94,105,121,145,179,198,223,234,292,305,322,331,345,356` · 세기: 정규식 `let (mut )?(\w+) = \2\(` 전수 **62** | 참 | 거짓신호 |
| R12 | **「기여 파일」 수가 세 값으로 병존하고 어느 것이 `E1` 문면의 126 인지 아무 데도 안 적혔다** — 조건 문면 **126** · 도구 출력 **132** · 게이트 문서 **128**. 정은 「126 과 132 의 차를 모른다」까지만 적었고 128 은 못 봤다 | `1-design.md`「판정 대상 문면」의 `E1` · 내 실행 3 줄(`기여 파일 132`) · `docs/gates/rust-scope-references.md:49`(`128`) | 참 | 거짓신호 |
| R13 | **`E3` 대안 아(→`S4`)를 기각한 논거가 되돌아온다** — 정은 *"`S4` 로 적으면 다시 잴 것이 무엇인지가 정해져 있지 않다"* 를 `S5` 의 이유로 들었다. 그러나 `R4`·`R5` 가 보이듯 **①이 무엇으로 이루어졌는지조차 아직 안 갈렸다**(21 중 몇이 `cfg` 인지 회차 어디에도 없다). *"무엇을 다시 잴지 모른다"* 는 조건이 안 재는 것의 증거가 아니라 **아직 안 잰 것**의 증거로도 똑같이 읽힌다 | `1-thesis.md:210` · 내 실행의 `남은 자리 표본` 은 21 중 **12 만** 찍는다(`scope_variants.rs:250`, `if 보기.len() < 12`) | 참 | 거짓신호 |
| R14 | **정이 「참」의 기계적 바닥으로 든 불변이 갈래 하나를 안 덮는다** — 정은 *"임포트된 이름은 원리상 엣지가 안 된다"* 로 도착지의 안전을 논증했다. 그런데 표본 28 줄 중 **아홉**(9·12·41·42·43·45·46 과 정 표의 10·33)은 `use` 가 아니라 **중첩 `mod` 에서 바깥으로 걸어 나가는 경로**로 해소된다(`scope.rs:376-378`). 그 경로에는 정이 든 불변이 안 걸린다 — 정도 자기 비판 2 에서 인정했지만 **근거 4 의 결론문(*"표본 50 줄의 도착지가 전부 … 이 불변과 맞는다"*)은 고치지 않았다** | `crates/pal-core/src/scope.rs:376-378`(`ScopeParent::Enclosing(next) => cursor = next`) · `1-thesis.md:130-135` 대 `:247-252` | 참 | 미관 |

---

## 음성 대조 — 설계문이 요구한 셋

**뒤집기 ⓐ — 내 반론을 표본 5 건으로 깎으면 살아 있나.** `R2`·`R11` 은 **안 산다** —
둘 다 세기(17 행 · 62 자리)가 근거라 표본을 깎으면 근거가 없어진다. 반대로
`R4`·`R5`·`R7`·`R8` 은 **표본과 무관**하다(코드 줄과 도구 출력 한 줄이 근거다).
그러므로 이 산출에서 표본에 매인 반론은 둘뿐이고 나머지는 표본 크기와 독립이다.

**뒤집기 ⓑ — 무엇이 있었으면 내 반론이 안 섰나.**

- `R4`·`R5` → `install/blocks.rs` 의 `상태` 나 `install_boundary.rs` 의 `방` 중 **하나라도
  `#[cfg(…)]` 가 붙어 있었으면** 안 섰다. 둘 다 열어서 `cfg` 가 없는 것을 확인했다.
- `R11` → `resolve_shadowing` 이 지역 바인딩에 `declared_at < at` 이 아니라 **`let` 문 끝
  바이트**를 기준으로 삼았으면 안 섰다. 지금은 패턴 토큰의 바이트다.
- `R2`·`R9` → 정의 산출문과 내 실행이 같은 코드 상태였으면 안 섰다.
- `R7` → 도구가 `V9` 을 엣지 축에서 갈리게 만들었으면 안 섰다. 지금은 `엣지축 같다` 다.

**뒤집기 ⓒ — `E3` 의 0 은 무엇의 0 인가.** 정과 같은 셋을 내가 다시 재면
① **21**(파일 10) ② **0**(모호 58 이 엣지를 안 만든다 · `scope.rs:371` + `projection.rs:250`)
③ **재현 불가**. **셋이 갈리는 사실은 내 실행에서도 그대로다.** 다만 `R4`·`R13` 이 ①의
**구성**까지 미확정임을 더한다 — 즉 갈리는 것이 셋이 아니라 「셋 중 하나는 아직 정의도 안
됐다」에 가깝다. 이 차이가 `S5` 와 `S4` 중 어느 쪽을 밀지는 **합이 정한다.**

---

## 내가 스스로 물린 것 (산출했다가 근거를 못 댄 것)

- **「`use super::*` 없는 중첩 `mod` 가 가짜를 만든다」를 세어 봤고 이 저장소에서 0 이다.**
  ⚠ **처음 센 것이 틀렸다** — 정규식이 `[A-Za-z_]` 로 시작해 한글 이름 모듈을 통째로
  빠뜨렸다. 다시 세니 인라인 `mod` 가 **80 개**이고 뒤 12 줄에 `use super` 가 없는 것은
  **하나**다: `crates/pal-core/src/graph.rs:339`(`#[cfg(test)] mod 왕복_파서를_진_표시_함수` —
  같은 파일의 `use super::*` `:246` 은 다른 모듈(`mod tests` `:245`) 것이라 여기 안 걸린다). 그 몸통을 열어 보니 바깥 아이템 접근이 전부
  `crate::ResolutionGrade`·`crate::Provenance`·`crate::QueryName` 꼴의 **경로**이고
  벗은 이름은 모듈 안에 선 `한국어가_있나`·지역 변수뿐이라 경계를 넘는 벗은 이름이 없다.
  **정의 자기 비판 2 는 이 코퍼스에서 실현되지 않는다** — 정이 자기에게 불리하게 적었고
  나는 그것을 반론으로 못 세운다.
- **`R11` 의 62 자리가 실제로 엣지에서 빠지는 것을 엣지 덤프로 확인 못 했다.**
  근거는 `scope.rs:338-378` 의 해소 경로와 `rust_scopes.rs:247-252`·`:313-321` 의 `let` 패턴 묶기
  뿐이고, 그 62 자리 중 몇이 다른 이유(예: 도착이 심볼이 아님)로 이미 안 서는지는 안 갈랐다.
  **수는 「후보 자리」이지 「확인된 누락」이 아니다.**
- **남은 모호 21 자리 중 몇이 `cfg` 인지 못 셌다.** 도구가 12 개만 찍고(`scope_variants.rs:249`)
  나는 코드를 못 고친다. `R4` 는 *"둘은 `cfg` 가 아니다"* 까지만 말하고 *"몇이 아니다"* 는 안 말한다.
- **`pal export --format cypher` 를 안 돌렸다.** `REFERENCES` 도 게이트 문서의 `128` 도
  내 눈으로 재확인 안 했다. `R12` 는 세 수가 병존한다는 사실만 대고 어느 것이 맞는지 안 정한다.
- **`cargo xtask check`·`cargo xtask test` 를 안 돌렸다.** `A`~`D` 통과 여부는 이 판 밖이다.
- **표본의 바이트 자리를 못 봤다.** 도구가 줄만 찍는다 — 그래서 한 줄에 후보가 둘인 자리
  (9·12·48·49·50)는 「어느 토큰이든 판정이 같다」로만 닫았고 바이트로는 못 갈랐다.
  49·50 은 정과 내가 **같은 줄에서 다른 도착지**를 적은 자리이고, 그것이 줄 좌표만으로는
  참조 하나를 유일하게 못 집는다는 실증이다.
- **판 2(`V9` 의 축이 정정인가 완화인가)는 안 정했다.** `R7` 은 `E2` 덮개 셈만 문제 삼는다.
- **회차 산출물이 내 검토 중에 움직였다.** 내가 처음 잰 `observations/e1-sample-50.txt` 는
  `3de5e81` 판이었고 뒤에 다시 여니 내 실행과 같아져 있었다. `git show 3de5e81:<경로>` 로
  둘 다 재현되므로 `R2`·`R3` 은 그대로 서지만, **내가 본 저장소 상태가 한 시점이 아니었다**
  는 것을 적어 둔다.
