//! 지원 언어. **다섯이 전부 1급이다** — 소유자 지시 2026-08-12 §1 ·
//! [2026-08-20 §1](../../../docs/instructions/2026-08-20-owner-direction.md) 이 다섯으로 갱신했다.

use serde::{Deserialize, Serialize};

/// 1급 지원 언어.
///
/// 다섯이 같은 층에 선다. 착수 순서만 다르고(Kotlin 이 S0), 그 순서는 선호가 아니라
/// **측정 가능성**이 정했다 — Kotlin 에만 사전 등록된 대조값이 있었다(T7 의 94.30%).
///
/// # 다섯째가 늘었다 — Rust (ADR-0027 · #66 · 2026-08-20)
///
/// **코어가 Rust 인데 추출기에 Rust 가 없어서** *"이 도구가 자기 자신을 큐레이션한다"*
/// 가 코퍼스 픽스처 한 파일 위에서만 섰다. R-19 는 자기적용을 「편향된 표본」이라
/// 불렀는데 거기서는 **편향 이전에 능력 부재**였다 — 표본이 치우친 것이 아니라
/// 잴 대상이 없었다.
///
/// `.svelte` 는 **1급이 아니다.** `<script>` 안의 js/ts 를 꺼내려면 injection 이
/// 필요하고 그것은 추출기 **구조**의 문제다. 소유 기능 미배정.
/// (앞 판은 이 문장을 *"다섯째 언어가 아니다"* 로 적었는데, Rust 가 다섯째가 되면서
/// 서수가 사실을 가리키지 않게 됐다.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    Kotlin,
    Java,
    JavaScript,
    TypeScript,
    Rust,
}

impl Language {
    /// 확장자로 언어를 정한다.
    ///
    /// `None` 은 *"이 확장자를 언어로 알지 못한다"* 이고
    /// **"언어는 아는데 추출기가 없다"와 다르다** — 후자는 [`crate::Capable`] 이 표현한다.
    /// 이 `Option` 은 조회 결과이지 도메인 값이 아니므로 stack §5.4 의 금지에 걸리지 않는다.
    #[must_use]
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext {
            "kt" | "kts" => Some(Self::Kotlin),
            "java" => Some(Self::Java),
            "js" | "mjs" | "cjs" | "jsx" => Some(Self::JavaScript),
            "ts" | "mts" | "cts" | "tsx" => Some(Self::TypeScript),
            "rs" => Some(Self::Rust),
            _ => None,
        }
    }

    /// 이름으로 언어를 정한다 — `.gitattributes` 의 `linguist-language` 가 쓴다.
    ///
    /// **대소문자를 가리지 않는다.** `linguist-language=kotlin` 도 같은 것을 뜻한다.
    #[must_use]
    pub fn from_name(name: &str) -> Option<Self> {
        [Self::Kotlin, Self::Java, Self::JavaScript, Self::TypeScript, Self::Rust]
            .into_iter()
            .find(|l| l.name().eq_ignore_ascii_case(name))
    }

    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Kotlin => "Kotlin",
            Self::Java => "Java",
            Self::JavaScript => "JavaScript",
            Self::TypeScript => "TypeScript",
            Self::Rust => "Rust",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 1급 언어 전수 — **네 번째 손 배열을 여기서 막는다.**
    ///
    /// ★ `Language` enum · `from_extension` 의 `match` · `FIRST_CLASS` · **`from_name`
    /// 의 배열** 넷이 함께 움직여야 하는데, 앞의 셋은 컴파일러나 타입이 잡고
    /// **`from_name` 만 안 잡혔다**(독립 리뷰 R5). 그 배열에서 언어가 빠져도
    /// 컴파일이 통과하고 `.gitattributes` 의 `linguist-language` 만 조용히 죽는다.
    const 전부: [Language; 5] = [
        Language::Kotlin,
        Language::Java,
        Language::JavaScript,
        Language::TypeScript,
        Language::Rust,
    ];

    #[test]
    fn 이름으로도_전부_찾힌다() {
        for l in 전부 {
            assert_eq!(from_name_or_panic(l.name()), l, "{} 가 from_name 배열에서 빠졌다", l.name());
        }
    }

    #[test]
    fn 이름은_대소문자를_안_가린다() {
        assert_eq!(Language::from_name("rust"), Some(Language::Rust));
        assert_eq!(Language::from_name("RUST"), Some(Language::Rust));
    }

    #[test]
    fn 확장자와_이름이_같은_집합을_본다() {
        // 확장자로 잡히는데 이름으로 안 잡히면 **같은 파일이 선언 유무에 따라
        // 다른 등급을 받는다** — 사전부검이 잡은 형태다.
        for (ext, l) in [("kt", Language::Kotlin), ("java", Language::Java),
                         ("js", Language::JavaScript), ("ts", Language::TypeScript),
                         ("rs", Language::Rust)] {
            assert_eq!(Language::from_extension(ext), Some(l));
            assert_eq!(Language::from_name(l.name()), Some(l));
        }
    }

    fn from_name_or_panic(n: &str) -> Language {
        Language::from_name(n).unwrap_or_else(|| panic!("from_name 이 {n} 를 모른다"))
    }
}
