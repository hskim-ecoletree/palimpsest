//! 언어마다 하나. **코어는 이 트레잇만 안다** — F02 §3.1.
//!
//! # 왜 자유 함수에서 트레잇으로 옮기는가
//!
//! S0 이 세운 것은 `extract(language, source)` 하나에 `match` 로 Kotlin 을 직접 부르는
//! 형태였다. 언어가 둘이 되는 순간 그 `match` 가 세 자리로 갈라진다 — 추출·능력·등급.
//! **셋이 갈리면 `pal symbols` 가 답하는 언어와 대장이 `parsed` 로 세는 언어가
//! 달라진다**(`corpus/criteria.toml` `[f02.1.pass]` ⑤).
//!
//! 그래서 **레지스트리 하나가 셋의 단일 진실이다.** [`extractor_for`] 가 그것이고,
//! [`crate::capability`] 와 [`crate::extract`] 와 [`crate::classify`] 가 전부 그것을 탄다.
//!
//! # 이 트레잇이 F02 §3.1 과 다른 한 자리
//!
//! 기능 문서는 `fn extract(&self, tree: &Tree, src: &[u8])` 로 적었다 — 파싱을 밖에
//! 두고 파서를 **스레드당 재사용**하기 위해서다. 여기서는 `fn extract(&self, source)`
//! 이고 파싱이 안에 있다.
//!
//! **이 조각이 구조적 변경이기 때문이다**(Tidy First). 파싱을 밖으로 빼면 Kotlin 의
//! 파싱 경로가 바뀌고, 그러면 이 커밋이 *"산출이 안 바뀐다"* 를 더는 값싸게 주장할 수
//! 없다. 파서 재사용은 **#49**(병렬)의 몫이고 그때 이 시그니처가 문서 형태로 간다.
//! **빚으로 적고 넘긴다.**

use pal_core::{Capable, CapabilityId, ExtractGrade, FileGraph, Language};

use crate::ExtractError;

/// 언어 하나를 읽는 능력.
///
/// `Send + Sync` 인 이유는 **#49 가 이것을 `par_iter` 안에서 부르기 때문이다.** 상태를
/// 들면 병렬이 비결정적이 되므로 구현은 무상태여야 한다 — 그래서 전부 유닛 구조체다.
pub trait LanguageExtractor: Send + Sync {
    /// 이 추출기가 읽는 언어.
    fn language(&self) -> Language;

    /// **실제로 도달하는 등급.** 선언 상한이 아니라 실측이다.
    ///
    /// 등급 표의 단일 진실은 [`crate::grade_of`] 이고 구현은 그것을 부른다. 두 자리에
    /// 적으면 갈린다.
    fn grade(&self) -> ExtractGrade;

    /// 파일 하나를 읽는다.
    ///
    /// # Errors
    /// 문법을 붙이지 못하거나 파싱이 중단되면 [`ExtractError`]. **깨진 소스는 오류가
    /// 아니다** — 회복 지점과 함께 부분 결과가 나온다(`recovery_sites`).
    fn extract(&self, source: &[u8]) -> Result<FileGraph, ExtractError>;
}

/// 이 빌드에 그 언어의 추출기가 있는가 — **소스 없이 묻는다.**
///
/// **이것이 능력 표의 단일 진실이다.** 빌드되지 않은 언어에서 빈 `Vec` 을 돌려주지
/// 않는다 — 그것이 거짓 안전이고, *"선언이 없는 파일"* 과 *"이 빌드가 그 언어를
/// 모른다"* 를 같은 출력으로 만든다(stack §5.3).
#[must_use]
pub fn extractor_for(language: Language) -> Capable<&'static dyn LanguageExtractor> {
    match language {
        Language::Kotlin => Capable::Present(&crate::kotlin::KOTLIN),
        Language::Java => Capable::not_built(CapabilityId::new("F02", "java-extraction")),
        Language::JavaScript => {
            Capable::not_built(CapabilityId::new("F02", "javascript-extraction"))
        }
        Language::TypeScript => Capable::Present(&crate::typescript::TYPESCRIPT),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grade_of;

    /// 1급 언어 넷 — 표가 늘 때 이 배열이 함께 늘어야 아래 시험들이 전수가 된다.
    const 일급: [Language; 4] =
        [Language::Kotlin, Language::Java, Language::JavaScript, Language::TypeScript];

    #[test]
    fn 능력과_등급이_같은_표를_본다() {
        // **`[f02.1.pass]` ⑤ 가 판정하는 자리다.** 추출기가 있는데 등급이 L0 이면
        // 대장이 없는 능력을 광고하거나 있는 능력을 숨긴다.
        for language in 일급 {
            let 있다 = extractor_for(language).is_present();
            let 등급 = grade_of(language);
            assert_eq!(
                있다,
                등급 != ExtractGrade::L0,
                "{} — 추출기 유무와 등급이 갈렸다",
                language.name()
            );
        }
    }

    #[test]
    fn 추출기는_자기_언어를_말한다() {
        // 레지스트리의 열쇠와 구현이 갈리면 `pal symbols` 가 엉뚱한 언어로 답한다.
        for language in 일급 {
            if let Capable::Present(e) = extractor_for(language) {
                assert_eq!(e.language(), language);
                assert_eq!(e.grade(), grade_of(language));
            }
        }
    }

    #[test]
    fn 미구축_언어는_자기_기능_번호를_싣는다() {
        let Capable::NotBuilt { capability } = extractor_for(Language::Java) else {
            panic!("Java 추출기가 생겼다면 이 시험을 고쳐라");
        };
        assert_eq!(capability.feature, "F02");
        assert_eq!(capability.what, "java-extraction");
    }
}
