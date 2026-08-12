//! 파일 → 언어. **인식과 추출 가능성은 다른 질문이다.**
//!
//! `.java` 는 언어가 인식되지만 이 빌드에 추출기가 없다. `.txt` 는 언어 자체를 모른다.
//! 대장은 둘을 다르게 적어야 한다 — 전자는 **로드맵**이고 후자는 **설정**이다.
//!
//! # 이 표가 코드에 있는 것은 S1 한정이다
//!
//! 최종적으로는 데이터여야 한다(stack §4 의 "단일 진실 파일"과 같은 성격). 지금 코드에
//! 두는 이유는 그것을 읽을 TOML 파서가 P0 의존 목록에 없기 때문이고, 매니페스트 로딩과
//! 함께 **F01 이 데이터로 옮긴다.** 여기 있는 동안은 언어를 늘리는 일이 코드를 고치는
//! 일이다 — 그 사실을 숨기지 않는다.
//!
//! # S1 의 인식은 4단계 중 **첫째뿐이다**
//!
//! [F01 §3.3](../../../docs/plan/features/F01-repo-ledger.md) 이 정한 순서는
//! ① 확장자 ② 셔뱅 ③ `.gitattributes` 의 `linguist-language` ④ 내용 휴리스틱 이다.
//! S1 은 ① 과 파일명만 본다. 그래서 `gradlew`(셔뱅으로만 알 수 있다) 같은 파일은
//! `Unrecognized` 로 남고, **그것이 이 슬라이스의 정직한 답이다** — 모르는 것을
//! 안다고 하지 않는다.

use pal_core::{Language, LanguageId};

/// 확장자로 아는 언어. **추출 가능 여부와 무관하다.**
///
/// 여기 없는 확장자는 `Unrecognized` 다. 목록을 넓히는 것이 대장을 더 정직하게 만들지는
/// 않는다 — 틀리게 인식하는 것보다 모른다고 적는 것이 낫다.
const BY_EXTENSION: &[(&str, &str)] = &[
    // 1급 넷 — `Language` 로도 잡힌다. 이름을 여기 한 번 더 적는 대신 아래에서 변환한다.
    // 그 밖의 언어들:
    ("sql", "SQL"),
    ("md", "Markdown"),
    ("markdown", "Markdown"),
    ("json", "JSON"),
    ("yml", "YAML"),
    ("yaml", "YAML"),
    ("xml", "XML"),
    ("toml", "TOML"),
    ("properties", "Java Properties"),
    ("gradle", "Gradle"),
    ("sh", "Shell"),
    ("bash", "Shell"),
    ("bat", "Batch"),
    ("cmd", "Batch"),
    ("html", "HTML"),
    ("htm", "HTML"),
    ("css", "CSS"),
    ("scss", "SCSS"),
    ("py", "Python"),
    ("go", "Go"),
    ("rs", "Rust"),
    ("rb", "Ruby"),
    ("svelte", "Svelte"),
    ("vue", "Vue"),
];

/// 확장자가 없는데 이름으로 아는 것.
const BY_FILE_NAME: &[(&str, &str)] = &[
    ("Dockerfile", "Dockerfile"),
    ("Makefile", "Make"),
    ("Jenkinsfile", "Groovy"),
];

/// 이 파일이 무슨 언어인가.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Recognition {
    /// 1급 넷 중 하나다. **추출기가 있는지는 별도 질문이다**(`pal_extract::extract`).
    FirstClass(Language),
    /// 언어는 알지만 우리 추출 대상이 아니다.
    Known(LanguageId),
    /// 모른다.
    Unknown,
}

impl Recognition {
    /// 인식된 이름. `Unknown` 이면 없다.
    #[must_use]
    pub fn language(&self) -> Option<LanguageId> {
        match self {
            Self::FirstClass(l) => Some(LanguageId::new(l.name())),
            Self::Known(id) => Some(id.clone()),
            Self::Unknown => None,
        }
    }
}

/// 확장자와 파일 이름으로 언어를 정한다.
///
/// **내용을 보지 않는다.** 내용 기반 판정은 F01 의 ②~④ 단계다.
#[must_use]
pub fn recognize(extension: &str, file_name: &str) -> Recognition {
    if let Some(l) = Language::from_extension(extension) {
        return Recognition::FirstClass(l);
    }
    let lower = extension.to_ascii_lowercase();
    if let Some((_, name)) = BY_EXTENSION.iter().find(|(e, _)| *e == lower) {
        return Recognition::Known(LanguageId::new(*name));
    }
    if let Some((_, name)) = BY_FILE_NAME.iter().find(|(n, _)| *n == file_name) {
        return Recognition::Known(LanguageId::new(*name));
    }
    Recognition::Unknown
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 일급_넷은_따로_잡힌다() {
        assert_eq!(recognize("kt", "A.kt"), Recognition::FirstClass(Language::Kotlin));
        assert_eq!(recognize("ts", "a.ts"), Recognition::FirstClass(Language::TypeScript));
    }

    #[test]
    fn 추출기가_없어도_언어는_인식된다() {
        // **이것이 `unsupported` 와 `unrecognized` 를 가르는 자리다.**
        let r = recognize("sql", "V1__init.sql");
        assert_eq!(r.language().unwrap().as_str(), "SQL");
        assert!(matches!(r, Recognition::Known(_)));
    }

    #[test]
    fn 이름으로만_아는_것도_있다() {
        assert_eq!(recognize("", "Dockerfile").language().unwrap().as_str(), "Dockerfile");
    }

    #[test]
    fn 모르는_것은_모른다고_한다() {
        // 목록을 넓히는 것이 대장을 더 정직하게 만들지 않는다.
        assert_eq!(recognize("bak", "a.bak"), Recognition::Unknown);
        assert_eq!(recognize("", "gradlew"), Recognition::Unknown);
    }
}
