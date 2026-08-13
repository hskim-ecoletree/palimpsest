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
mod parse;
mod recognize;
mod scopes;
mod shell;
mod typescript;

use pal_core::{Capable, ExtractorVersion, Language, Symbol};

pub use cached::{CachedGraph, RestoreError, ShellMismatch, Slot};
pub use classify::{Extraction, FileOutcome, OVERSIZE_BYTES, classify, grade_of};
pub use extractor::{LanguageExtractor, extractor_for};
pub use kotlin::KotlinExtractor;
pub use parse::ExtractError;
pub use recognize::{Recognition, recognize};
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
/// 같은 커밋이 캐시 상태에 따라 다른 답을 낸다.
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
/// 네 번째다. **정규형이 바뀌었다** — F03 §3.1 표의 남은 두 행(후행 쉼표 · 리터럴
/// 따옴표 종류)이 서고, 객체 리터럴의 축약 속성이 지우기에서 빠졌다.
///
/// 정규화가 바뀌면 **모든 `body_digest` 가 이동한다.** 안 올리면 옛 캐시가 옛 요약을
/// 새 스키마로 되돌려 주고, 그러면 같은 커밋이 캐시 상태에 따라 다른 답을 낸다.
/// F03 §3.1 이 *"승급은 관측되는 사건이다 — 조용히 바꾸면 전 결박이 이유 없이 `stale`
/// 이 된다"* 라고 적은 그 자리다.
///
/// [`FileOutcome`]: crate::FileOutcome
pub const EXTRACTOR_REV: &str = "f03-2";

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
