//! 사용자 언어 병기 — **사람이 보는 화면에만 쓴다.**
//!
//! [ADR-0033] §3·§4 가 정한 형식을 여기 한 곳에서 낸다: `사용자 언어(원 표기)` ·
//! 괄호 안은 소문자 · 원 표기는 `pal-core` 의 `name()` 이나 serde 토큰과 같은 문자열.
//!
//! # 왜 `pal-core` 가 아니라 여기인가 — 두 사고를 구조로 막는다
//!
//! **① 기계 출력 오염(`C1-a`).** `pal-core` 의 `name()` 은 화면 문자열이 아니라
//! **와이어 토큰**이다. `crate::export` 가 그것을 Cypher 속성 값으로 그대로 쓰고
//! (`identity: "…"`), `pal-query` 가 `BindingReport::watch_grades` 의 **키**로 쓴다.
//! 거기 병기를 얹으면 그래프 산출이 오염된다.
//!
//! **② 조용한 파서 죽음(`C1-b`).** `name()` 이 동시에 `parse` 의 열쇠인 자리가 넷이다 —
//! `ResolutionGrade` · `Provenance` · `QueryName` · `schema::Cardinality`.
//! 병기를 얹으면 `parse` 가 [`None`] 을 돌려주는데 그 반환형이 [`Option`] 이라
//! **아무 검사도 안 잡는다.** `pal-core` 의 `왕복_파서를_진_표시_함수` 모듈이 그것을 잰다.
//!
//! 병기를 크레이트 경계 밖에 두면 위 둘이 **구조로** 막힌다. `pal-core` 는 이 모듈을
//! 볼 수 없으므로 여기 있는 문자열이 직렬화 경로에 닿을 길이 없다.
//!
//! # 두 조각으로 돌려주는 까닭
//!
//! [ADR-0033] §3 이 *"고정폭 열에는 원 표기만 넣고 병기는 열 뒤에 붙인다"* 로 정했다.
//! `{:<N}` 은 char 수로 채우는데 한글은 터미널에서 두 열을 먹어 열 안에 넣으면 정렬이
//! 어긋난다. 그래서 조각 둘을 따로 내고, 고정폭이 아닌 자리는 [`Label::병기`] 로
//! 한 줄로 합친다. **두 조각이 한 줄에 같이 있으면 그것이 병기다** — 붙어 있어야
//! 하는 것이 아니다.
//!
//! [ADR-0033]: ../../../docs/adr/0033-a-word-that-reads-wrong-is-not-fixed-by-a-gloss.md

use pal_core::{Bucket, CodeFreshness, ExtractGrade, IdentityGrade, NearKind};

/// 값 하나의 두 표기.
pub struct Label {
    /// 정본. `pal-core` 의 토큰과 **글자까지 같다.**
    pub 원_표기: &'static str,
    /// 사용자 언어. **여기서만 산다** — 직렬화 경로에 닿지 않는다.
    pub 사용자_언어: &'static str,
}

impl Label {
    /// 고정폭이 아닌 자리에서 한 줄로 합친다 — `사용자 언어(원 표기)`.
    #[must_use]
    pub fn 병기(&self) -> String {
        format!("{}({})", self.사용자_언어, self.원_표기)
    }
}

/// 코드 신선도 넷. **[ADR-0033] §4 의 표가 정본이다.**
///
/// 원 표기는 `CodeFreshness` 의 serde 태그(`freshness`)가 내는 값과 같다 —
/// 아래 `토큰이_serde_와_같다` 가 그 같음을 잰다.
#[must_use]
pub const fn 신선도(c: &CodeFreshness) -> Label {
    match c {
        CodeFreshness::Fresh => Label { 원_표기: "fresh", 사용자_언어: "최신 상태" },
        CodeFreshness::Stale { .. } => Label { 원_표기: "stale", 사용자_언어: "최신 상태 아님" },
        CodeFreshness::Orphaned { .. } => Label { 원_표기: "orphaned", 사용자_언어: "좌표 없음" },
        CodeFreshness::Undeterminable { .. } => {
            Label { 원_표기: "undeterminable", 사용자_언어: "판정 불가" }
        }
    }
}

/// `pal ledger` 의 파일 상태 일곱.
#[must_use]
pub const fn 파일_상태(b: Bucket) -> Label {
    match b {
        Bucket::Parsed => Label { 원_표기: "parsed", 사용자_언어: "파싱됨" },
        Bucket::Partial => Label { 원_표기: "partial", 사용자_언어: "일부만 파싱됨" },
        Bucket::Unsupported => Label { 원_표기: "unsupported", 사용자_언어: "미지원" },
        Bucket::Unrecognized => Label { 원_표기: "unrecognized", 사용자_언어: "언어 미인식" },
        Bucket::Excluded => Label { 원_표기: "excluded", 사용자_언어: "설정으로 제외" },
        Bucket::Binary => Label { 원_표기: "binary", 사용자_언어: "이진 파일" },
        Bucket::Generated => Label { 원_표기: "generated", 사용자_언어: "생성물" },
    }
}

/// 심볼 정체성 등급 셋.
///
/// `Unavailable` 의 사용자 언어는 「결박 불가」다 — 대장 머리가 이미 그 말을 쓰고
/// (`pal ledger` 의 *"결박 불가 언어 N개"*), 같은 낱말이어야 사람이 두 줄을 같은 것으로
/// 읽는다.
#[must_use]
pub const fn 정체성_등급(g: IdentityGrade) -> Label {
    match g {
        IdentityGrade::Unavailable => Label { 원_표기: "unavailable", 사용자_언어: "결박 불가" },
        IdentityGrade::Ordinal => Label { 원_표기: "ordinal", 사용자_언어: "선언 순서" },
        IdentityGrade::Exact => Label { 원_표기: "exact", 사용자_언어: "정확" },
    }
}

/// 언어 추출 등급 다섯.
///
/// 원 표기 `L0`~`L4` 는 영어가 아니라 **코드**라, [ADR-0033] §3 의 「같은 언어권이면
/// 병기하지 않는다」 규칙이 걸리지 않는다. 병기가 그 코드의 뜻을 나른다.
#[must_use]
pub const fn 추출_등급(g: ExtractGrade) -> Label {
    match g {
        ExtractGrade::L0 => Label { 원_표기: "L0", 사용자_언어: "텍스트만" },
        ExtractGrade::L1 => Label { 원_표기: "L1", 사용자_언어: "구조" },
        ExtractGrade::L2 => Label { 원_표기: "L2", 사용자_언어: "참조 해소" },
        ExtractGrade::L3 => Label { 원_표기: "L3", 사용자_언어: "정의·사용" },
        ExtractGrade::L4 => Label { 원_표기: "L4", 사용자_언어: "제어흐름" },
    }
}

/// 「이것을 뜻했습니까」의 가까움 갈래 둘.
///
/// `NearKind::name()` 은 착수 시점에 「표기」·「부분」을 돌려주고 있었다 — `C2-a` 의
/// 검사가 찾은 셋째 위반이다(`C2-b` 는 둘만 셌다). 그 값은 serde 가 내는
/// `spelling`·`substring` 과 갈려 있었고, 그래서 같은 값이 화면과 `--json` 에서 다른
/// 낱말로 나갔다.
#[must_use]
pub const fn 가까움(k: NearKind) -> Label {
    match k {
        NearKind::Spelling => Label { 원_표기: "spelling", 사용자_언어: "표기만 다름" },
        NearKind::Substring => Label { 원_표기: "substring", 사용자_언어: "부분 일치" },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn 한국어가_있나(s: &str) -> bool {
        s.chars().any(|c| ('\u{AC00}'..='\u{D7A3}').contains(&c))
    }

    /// **원 표기가 `pal-core` 의 토큰과 글자까지 같아야 한다.**
    ///
    /// 갈리면 사람이 화면과 `--json` 을 눈으로 대조할 수 없다([ADR-0033] §3).
    #[test]
    fn 원_표기가_코어_토큰과_같다() {
        for b in Bucket::ALL {
            assert_eq!(파일_상태(b).원_표기, b.name());
        }
        for g in [IdentityGrade::Unavailable, IdentityGrade::Ordinal, IdentityGrade::Exact] {
            assert_eq!(정체성_등급(g).원_표기, g.name());
        }
        for g in
            [ExtractGrade::L0, ExtractGrade::L1, ExtractGrade::L2, ExtractGrade::L3, ExtractGrade::L4]
        {
            assert_eq!(추출_등급(g).원_표기, g.name());
        }
        for k in [NearKind::Spelling, NearKind::Substring] {
            assert_eq!(가까움(k).원_표기, k.name());
        }
    }

    /// 신선도만 `name()` 이 없으므로 **serde 가 내는 태그와 직접 대조한다.**
    #[test]
    fn 신선도_토큰이_serde_와_같다() {
        let 값 = [
            CodeFreshness::Fresh,
            CodeFreshness::Stale { triggered_by: Vec::new() },
            CodeFreshness::Orphaned { missing: Vec::new() },
            CodeFreshness::Undeterminable {
                reason: pal_core::UndeterminableReason::IdentityGrade,
                at: Vec::new(),
            },
        ];
        for c in &값 {
            let v = serde_json::to_value(c).expect("직렬화");
            let 태그 = v["freshness"].as_str().expect("태그");
            assert_eq!(신선도(c).원_표기, 태그, "병기의 원 표기가 와이어 토큰과 갈렸다");
        }
    }

    /// **원 표기에 한국어가 없고 사용자 언어에는 있다** — 두 조각이 뒤바뀌지 않았는지 잰다.
    #[test]
    fn 두_조각이_안_뒤바뀐다() {
        let 전부: Vec<Label> = Bucket::ALL
            .into_iter()
            .map(파일_상태)
            .chain([IdentityGrade::Unavailable, IdentityGrade::Ordinal, IdentityGrade::Exact]
                .into_iter()
                .map(정체성_등급))
            .chain([ExtractGrade::L0, ExtractGrade::L4].into_iter().map(추출_등급))
            .chain([신선도(&CodeFreshness::Fresh)])
            .chain([NearKind::Spelling, NearKind::Substring].into_iter().map(가까움))
            .collect();
        for l in &전부 {
            assert!(!한국어가_있나(l.원_표기), "원 표기 `{}` 에 한국어가 있다", l.원_표기);
            assert!(한국어가_있나(l.사용자_언어), "사용자 언어 `{}` 가 한국어가 아니다", l.사용자_언어);
            assert_eq!(l.병기(), format!("{}({})", l.사용자_언어, l.원_표기));
        }
        assert_eq!(전부.len(), 7 + 3 + 2 + 1 + 2);
    }
}
