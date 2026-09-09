//! tree-sitter 추출기.
//!
//! **쿼리 파일은 CLI 레퍼런스와 공유한다**(`queries/kotlin/top-level.scm`).
//! 대조(`corpus/criteria.toml` `[s0]`)가 *코드 경로*만의 차이가 되려면 그래야 한다 —
//! 한쪽만 고치면 그것은 대조를 사후 조정하는 일이다.

#![forbid(unsafe_code)]

mod cached;
mod classify;
mod extractor;
mod kotlin;
mod narrative;
mod parse;
mod plan;
mod recognize;
mod rust;
mod rust_scopes;
mod scopes;
mod shell;
mod ts_scopes;
mod typescript;

use pal_core::{Capable, ExtractorVersion, Language, Symbol};

pub use cached::{CachedGraph, RestoreError};
pub use classify::{Extraction, FileOutcome, classify, grade_of};
pub use extractor::{LanguageExtractor, extractor_for};
pub use kotlin::KotlinExtractor;
pub use narrative::fragment;
pub use parse::{ExtractError, MarkedComment, marked_comments};
// **계획 문서 인입** — 조각화는 [`fragment`] 를 그대로 쓰고 좌표 추출만 F12 가 세운다.
pub use plan::ingest_plan;
pub use recognize::{Recognition, recognize};
pub use rust::RustExtractor;
// **변형 대조 전용 표면** — `--example scope_variants` 하나가 부른다.
//
// ★ 이것이 이 회차가 세운 **장치**다(규약 §11 ③ (나) · 소유자가 대가를 보고 골랐다).
// 추출기 자신은 `RustScopeRules::기준` 만 쓰고, 다른 조합은 「그 규칙을 끄면 산출이
// 달라지는가」를 재는 자리에서만 만들어진다. 이 표면이 없으면 완수 증인이 없다.
#[doc(hidden)]
pub use rust::extract_with as rust_extract_with;
#[doc(hidden)]
pub use rust_scopes::RustScopeRules;
pub use shell::{FIRST_CLASS, GraphShell, capability_axis, shell_of};
pub use typescript::TypeScriptExtractor;

/// 판정용 문법의 고정 커밋 — `corpus/criteria.toml` `[g50]`.
///
/// # 축이 하나인데 언어가 둘이다 — `[g50.pass]` ⑥ 의 판단을 여기 적는다
///
/// 이 상수는 **Kotlin 문법의 rev 하나**인데 [`ExtractorVersion`] 의 `grammar` 축은
/// 두 언어가 함께 탄다. Kotlin 문법을 올리면:
///
///   · **1층 캐시가 두 언어 모두 전량 무효화된다**
///   · **`Coord.extractor` 가 두 언어 모두 움직인다** — 좌표의 성분이므로
///   · **그런데 TypeScript 의 `symbol_id`·`body_digest` 는 안 움직인다** —
///     그 값들은 Kotlin 문법에 의존하지 않는다
///
/// **축을 언어별로 가르지 않는다.** 셋을 재고 판단했다:
///
/// 1. **[ADR-0004] 가 요구하는 것은 「산출을 정하는 모든 입력이 키에 있다」이고,
///    지금 형태는 그것을 어기지 않는다.** 어기는 방향은 **덜 무효화하는 쪽**이고
///    지금은 **더** 무효화한다. 과잉 무효화는 느릴 뿐 틀리지 않는다
/// 2. **가르려면 캐시 키가 「이 블롭이 무슨 언어인가」를 알아야 한다.** 그런데 그것은
///    우리 코드가 내리는 **판정**이다(`recognize`). 판정을 키의 성분으로 쓰면
///    **판정이 틀린 파일이 틀린 키를 갖고, 그 틀림이 캐시 뒤로 숨는다.**
///    F03 이 실코드인 `.ts` 다섯을 `binary{nul_byte}` 로 잘못 읽은 것을 발견했는데,
///    축을 갈랐다면 그 다섯은 **재분류돼도 옛 항목을 그대로 돌려받았을 것이다**
/// 3. **비용이 일회성이고 F04 의 것이다.** 지금 무효화되는 것은 1층 캐시뿐이고
///    다시 채우는 값은 이미 재고 있다. 상시 비용이 아니다
///
/// **비대칭은 남고, 남는다는 사실을 적는 것이 여기서 지는 몫이다** —
/// `ditto` 골든(4,578 줄)이 **안 움직이는 것**이 그 비대칭의 관측 장치다
/// (`[g50.pass]` ④ · `scripts/f03-3-verify.py`).
///
/// [ADR-0004]: ../../../docs/adr/0004-cache-key-covers-every-input-that-decides-the-output.md
/// [`ExtractorVersion`]: pal_core::ExtractorVersion
pub const GRAMMAR_REV: &str = "acb96307d816618bd60e1e4d2fa3eaa793e97a2e";

/// 추출기 코드 버전. 문법과 **다른 축이다**(stack §5.1).
///
/// # 이 값을 올리는 것이 곧 1층 캐시 전량 무효화다
///
/// S2 에서 `Symbol` 에 `body_digest` 가 붙어 추출 산출이 바뀌었다. 값을 올리지 않았더니
/// **옛 캐시를 새 스키마로 읽으려다 실패했다** — 캐시 키에 이 값을 넣은 이유가 그
/// 자리에서 관측된 것이다. 올리면 키가 달라지므로 옛 항목은 조회되지 않고 조용히
/// 남았다가 `prune`(F04)이 걷어간다.
///
/// **문법 rev 는 그대로다.** 축이 둘인 이유가 이것이다 — 추출기 코드가 바뀌었다고
/// 문법이 바뀐 것은 아니다.
///
/// # `s2` → `f02-3` (2026-08-13 · #48)
///
/// 두 번째로 올린다. 이유는 첫 번째와 같은 형태다 — **추출 산출이 바뀌었다.**
/// `Symbol` 에 `identity` 가 붙었고 TypeScript 의 `body_digest` 가 지역 이름을 지우기
/// 시작했다. 올리지 않으면 **옛 캐시가 옛 요약을 새 스키마로 되돌려 준다** — 그러면
/// 같은 커밋이 캐시 상태에 따라 다른 답을 돌려준다.
///
/// # `f02-3` → `f03-1` (2026-08-13 · #51)
///
/// 세 번째로 올린다. **이번에는 산출이 아니라 캐시에 담기는 값의 모양이 먼저 바뀌었다** —
/// [`FileOutcome`] 이 포함 관계를 싣기 시작했다. 올리지 않으면 옛 항목을 새 스키마로
/// 읽으려다 실패한다(첫 승급 때 실제로 관측된 형태다).
///
/// **그리고 같은 값이 뒤이은 좌표 이동까지 덮는다.** 이 슬라이스의 동작 커밋이
/// `symbol_id` 에 컨테이너 체인을 넣어 좌표를 움직이는데, 두 커밋이 인접하고 그 사이
/// 상태는 배포되지 않는다. **슬라이스 하나가 좌표 이동 하나다** — 커밋마다 올리면
/// 승급이 관측되는 사건이 아니라 잡음이 된다(stack §5.1).
///
/// # `f03-1` → `f03-2` (2026-08-13 · #52)
///
/// 네 번째다. **정규형이 바뀌었다** — 옛 F03 §3.1 표의 남은 두 행(후행 쉼표 · 리터럴
/// 따옴표 종류)이 서고, 객체 리터럴의 축약 속성이 지우기에서 빠졌다.
///
/// 정규화가 바뀌면 **모든 `body_digest` 가 이동한다.** 안 올리면 옛 캐시가 옛 요약을
/// 새 스키마로 되돌려 주고, 그러면 같은 커밋이 캐시 상태에 따라 다른 답을 돌려준다.
/// 옛 F03 §3.1 이 *"승급은 관측되는 사건이다 — 조용히 바꾸면 전 결박이 이유 없이 `stale`
/// 이 된다"* 라고 적은 그 자리다.
///
/// # `f03-2` → `f02-rust-scope` (2026-09-08 · #130)
///
/// 다섯째다. **모양이 둘 바뀌었다.**
///
/// - Rust 추출기가 `scopes`·`imports`·`exports` 를 `Capable::Present` 로 산출한다 —
///   `FileGraph` 의 세 자리가 `not_built` 에서 값으로 바뀐다
/// - `ScopeKind` 에 `Impl` 이, `RefResolution` 에 `Ambiguous` 가 붙었다
///
/// 안 올리면 **옛 항목을 새 스키마로 읽으려다 실패한다** — 첫 승급 때 실제로 관측된
/// 형태다. 능력 축(`shell.rs`)이 캐시 키에 있어 Rust 항목은 어차피 미스가 되지만,
/// **`ScopeKind`·`RefResolution` 의 모양 변화는 TypeScript 항목에도 걸린다.**
/// 그래서 언어별이 아니라 이 축 하나를 올린다.
///
/// ⚠ **`body_digest` 는 안 움직인다.** Rust 심볼의 `identity` 는 여전히 `Ordinal` 이라
/// 정규화가 지역 이름을 안 지운다 — 결박 25 건이 `fresh` 그대로여야 한다(`C2`).
/// 움직이면 이 회차의 전제가 무너진 것이다.
///
/// ⚠ **2층 행의 모양도 함께 바뀌었다** — `RefCounts` 에 `ambiguous` 가 붙었다.
/// 그 행은 postcard 로 자리 기반 직렬화라 **옛 2층 행은 다시 세워야 한다.**
/// 이 축은 1층 캐시 키에만 들어가므로 2층은 재적재가 답이다.
///
/// 다시 세우는 명령은 **`pal query graph.dump`** 다 — 2층이 없거나 낡으면 그것이 세운다.
/// ⚠ **`pal index` 라는 명령은 없다**(실측: `error: unrecognized subcommand 'index'`).
/// 없는 구제 경로를 적으면 그것이 「모르는 것을 안다고 적는」 형태다.
///
/// # `f02-rust-scope` → `f07-import-items` (2026-09-09 · #134)
///
/// 여섯째다. **[`pal_core::ImportSet`] 의 모양이 바뀌었다** — `modules` 하나뿐이던 것에
/// [`pal_core::ImportSet::items`]([`pal_core::ImportedItem`] 의 [`pal_core::Slot`])가
/// 붙었다. 두 추출기가 다 그것을 채운다.
///
/// 안 올리면 **옛 캐시 항목이 새 필드 없이 되살아난다.** `FileGraph` 는 postcard 로
/// 저장되고 그것은 자리 기반 직렬화라, 옛 바이트를 새 타입으로 읽으면 실패하거나 —
/// 더 나쁘게 — 뒤 필드를 앞 필드로 읽는다. 능력 축(`shell.rs`)은 *"imports 자리를
/// 만드나"* 만 보고 **그 안의 항목 축은 안 본다**, 그래서 이 승급이 그 자리를 진다.
///
/// ⚠ **`body_digest` 는 안 움직인다.** 임포트 항목은 심볼 요약의 입력이 아니다 —
/// 결박 30 건이 `fresh` 26 · `stale` 4 그대로여야 한다(`D1`). 움직이면 **거기서 멈추고
/// 소유자에게 올린다.**
///
/// ⚠ **`scripts/f04-verify.py:188` 의 리터럴도 같이 움직인다**(`D3`). 그 스크립트는
/// 이 줄을 문자열로 찾아 변이시켜 「축을 움직였는데 캐시가 적중하나」를 재고, 찾을 것이
/// 없으면 `어긋남` 을 적고 **비-0 으로 끝난다.** 조용히 꺼지는 것이 아니라 빨개진다.
///
/// # `f07-import-items` → `f08-call-tails` (2026-09-09 · #134 · `A7` 갈래 (다2))
///
/// 일곱째다. **[`pal_core::ScopeChain`] 의 행 모양이 바뀌었다** —
/// [`pal_core::LocalRef`] 에 [`tail`](pal_core::LocalRef::tail) 이 붙었다. 경로 호출의
/// 꼬리(`S::foo()` 의 `foo`)를 **참조로 만들지 않고** 머리 참조에 실어 2 층으로 보내는
/// 자리다. 꼬리를 참조로 세면 같은 파일의 동명 선언에 붙어 조용한 오답이 되고(실측 31
/// 건), 그것이 `경로_꼬리_배제` 가 서 있는 까닭이다. **그 배제는 안 끈다.**
///
/// 안 올리면 **옛 캐시 항목이 꼬리 없이 되살아난다.** `ScopeChain` 은 `CachedGraph` 의
/// `scopes` 슬롯에 postcard 로 저장되고 그것은 자리 기반이라, 옛 바이트를 새 타입으로
/// 읽으면 실패하거나 — 더 나쁘게 — `Option` 자리를 뒤 필드로 읽는다. 그러면 파일 간
/// 엣지가 **캐시가 적중한 파일에서만 안 서고**, 그 어긋남은 화면 어디에도 안 나온다.
///
/// ⚠ **`body_digest` 는 안 움직인다.** 꼬리는 심볼 요약의 입력이 아니다 — 결박 30 건이
/// `fresh` 25 · `stale` 5 그대로여야 한다(`D1`). 움직이면 거기서 멈추고 올린다.
///
/// [`FileOutcome`]: crate::FileOutcome
pub const EXTRACTOR_REV: &str = "f08-call-tails";

#[must_use]
pub const fn version() -> ExtractorVersion {
    ExtractorVersion { grammar: GRAMMAR_REV, extractor: EXTRACTOR_REV }
}

/// 이 빌드가 그 언어를 추출할 수 있으면 시도한다.
///
/// **빌드되지 않은 언어에서 빈 `Vec` 을 돌려주지 않는다** — 그것이 거짓 안전이다.
/// 자리는 [`Capable`] 이 잡는다.
///
/// # Errors
/// 언어가 빌드되어 있고 파싱·쿼리가 실패하면 [`ExtractError`].
#[must_use]
pub fn extract(language: Language, source: &[u8]) -> Capable<Result<Vec<Symbol>, ExtractError>> {
    match extractor_for(language) {
        Capable::Present(e) => Capable::Present(e.extract(source).map(|g| g.symbols)),
        Capable::NotBuilt { capability } => Capable::NotBuilt { capability },
    }
}

/// 이 빌드가 그 언어를 추출할 수 있는가 — **소스 없이 묻는다.**
///
/// 대장은 파일을 읽기 전에 이것을 알아야 한다. [`extract`] 와 **같은 표를 본다** —
/// 둘이 갈리면 `pal symbols` 가 답하는 언어와 대장이 `parsed` 로 세는 언어가 달라진다.
/// 그 표가 [`extractor_for`] 이고, 셋이 전부 그것 하나를 탄다.
#[must_use]
pub fn capability(language: Language) -> Capable<()> {
    match extractor_for(language) {
        Capable::Present(_) => Capable::Present(()),
        Capable::NotBuilt { capability } => Capable::NotBuilt { capability },
    }
}
