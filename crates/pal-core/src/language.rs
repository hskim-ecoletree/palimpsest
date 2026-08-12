//! 지원 언어. **넷이 전부 1급이다** — 소유자 지시 2026-08-12 §1.

use serde::Serialize;

/// 1급 지원 언어.
///
/// 넷이 같은 층에 선다. 착수 순서만 다르고(Kotlin 이 S0), 그 순서는 선호가 아니라
/// **측정 가능성**이 정했다 — Kotlin 에만 사전 등록된 대조값이 있다(T7 의 94.30%).
///
/// `.svelte` 는 다섯째 언어가 아니다. `<script>` 안의 js/ts 를 꺼내려면
/// injection 이 필요하고 그것은 추출기 **구조**의 문제다. 소유 기능 미배정.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    Kotlin,
    Java,
    JavaScript,
    TypeScript,
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
            _ => None,
        }
    }

    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Kotlin => "Kotlin",
            Self::Java => "Java",
            Self::JavaScript => "JavaScript",
            Self::TypeScript => "TypeScript",
        }
    }
}
