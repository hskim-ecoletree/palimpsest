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
//! # 인식은 셋이고, F01 §3.3 의 넷과 순서가 다르다
//!
//! [옛 F01 §3.3](../../../docs/plan/disposal-map.md) 이 정한 순서는
//! ① 확장자 ② 셔뱅 ③ `.gitattributes` 의 `linguist-language` ④ 내용 휴리스틱 이다.
//! **F01 이 ②③ 을 세우면서 두 자리에서 그 순서를 정정했다.**
//!
//! **③ 이 ① 보다 앞선다.** `linguist-language` 는 사람이 그 파일에 대해 **선언한** 것이고
//! 확장자는 규약일 뿐이다. 순서를 문서대로 두면 선언이 규약에 져서 *"사용자가 덮어쓸 수
//! 있게"* (F01 §5)가 성립하지 않는다 — 덮어쓸 수 없는 것은 덮어쓰기가 아니다.
//!
//! **④ 는 세우지 않았다.** 실물 코퍼스에서 ④ 가 켤 수 있는 것은
//! `…Test.kt.bak` 하나뿐이고, 그것을 Kotlin 으로 인식하면 **백업 파일이 추출 대상이
//! 된다.** 아래가 이미 적고 있는 원칙 — *"틀리게 인식하는 것보다 모른다고 적는 것이
//! 낫다"* — 이 그 자리에 그대로 걸린다. 근거는 `docs/gates/F01.md` 에 목록으로 있다.

use pal_core::{Language, LanguageId, SHEBANG_SCAN_BYTES};

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

/// 셔뱅의 인터프리터 → 언어. **경로가 아니라 이름만 본다** (`/usr/bin/env python3` 도).
const BY_INTERPRETER: &[(&str, &str)] = &[
    ("sh", "Shell"),
    ("bash", "Shell"),
    ("zsh", "Shell"),
    ("dash", "Shell"),
    ("ksh", "Shell"),
    ("python", "Python"),
    ("python2", "Python"),
    ("python3", "Python"),
    ("node", "JavaScript"),
    ("ruby", "Ruby"),
    ("perl", "Perl"),
];

// **셔뱅 스캔 길이는 여기 없다** — `pal-core::budget` 한 곳이다(stack §5.5).

/// 이 파일이 무슨 언어인가 — **선언 → 확장자·이름 → 셔뱅** 순서로 묻는다.
///
/// `declared` 는 `.gitattributes` 의 `linguist-language` 다(F01 §3.3 의 ③).
/// **그것이 가장 앞선다** — 이유는 이 모듈의 머리 주석에 있다.
#[must_use]
pub fn recognize(
    extension: &str,
    file_name: &str,
    declared: Option<&str>,
    head: &[u8],
) -> Recognition {
    // ③ 선언 — 사람이 그 파일에 대해 적어 둔 것.
    if let Some(name) = declared {
        return match Language::from_name(name) {
            Some(l) => Recognition::FirstClass(l),
            None => Recognition::Known(LanguageId::new(name)),
        };
    }
    // ① 확장자와 이름.
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
    // ② 셔뱅 — 확장자가 없는 실행 스크립트의 자리다(`gradlew`).
    if let Some(name) = shebang_language(head) {
        return Recognition::Known(LanguageId::new(name));
    }
    Recognition::Unknown
}

/// 첫 줄이 `#!` 이면 인터프리터 이름으로 언어를 정한다.
///
/// `#!/usr/bin/env python3` 처럼 `env` 를 거치는 형태가 흔하므로 **마지막 인자**까지
/// 본다. 모르는 인터프리터는 **모른다** — 목록을 넓히는 것이 대장을 더 정직하게
/// 만들지 않는다.
fn shebang_language(head: &[u8]) -> Option<&'static str> {
    let head = &head[..head.len().min(SHEBANG_SCAN_BYTES)];
    let line = head.split(|b| *b == b'\n').next()?;
    let line = std::str::from_utf8(line).ok()?.trim_end_matches('\r');
    let rest = line.strip_prefix("#!")?;

    for token in rest.split_whitespace() {
        // 옵션(`-u`)은 인터프리터가 아니다.
        if token.starts_with('-') {
            continue;
        }
        let name = token.rsplit('/').next().unwrap_or(token);
        if name == "env" {
            continue;
        }
        if let Some((_, language)) = BY_INTERPRETER.iter().find(|(i, _)| *i == name) {
            return Some(language);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn 인식(ext: &str, name: &str) -> Recognition {
        recognize(ext, name, None, b"")
    }

    #[test]
    fn 일급_넷은_따로_잡힌다() {
        assert_eq!(인식("kt", "A.kt"), Recognition::FirstClass(Language::Kotlin));
        assert_eq!(인식("ts", "a.ts"), Recognition::FirstClass(Language::TypeScript));
    }

    #[test]
    fn 셔뱅이_확장자_없는_스크립트를_잡는다() {
        // **실물에서 이 단계가 켜는 것은 `gradlew` 하나다**(코퍼스 실측).
        let r = recognize("", "gradlew", None, b"#!/bin/sh\n\n# gradle wrapper\n");
        assert_eq!(r.language().unwrap().as_str(), "Shell");
        // `env` 를 거치는 형태도 본다.
        let r = recognize("", "run", None, b"#!/usr/bin/env python3\n");
        assert_eq!(r.language().unwrap().as_str(), "Python");
        // 옵션은 인터프리터가 아니다.
        let r = recognize("", "run", None, b"#!/usr/bin/env -S node --experimental\n");
        assert_eq!(r.language().unwrap().as_str(), "JavaScript");
    }

    #[test]
    fn 모르는_인터프리터는_모른다() {
        assert_eq!(recognize("", "run", None, b"#!/usr/bin/awk -f\n"), Recognition::Unknown);
        // 셔뱅이 아닌 첫 줄에 반응하지 않는다.
        assert_eq!(recognize("", "notes", None, b"# hello\n"), Recognition::Unknown);
    }

    #[test]
    fn 선언이_확장자를_이긴다() {
        // `.gitattributes` 의 `linguist-language` 는 사람이 선언한 것이고 확장자는
        // 규약일 뿐이다. 지지 않아야 "사용자가 덮어쓸 수 있게" 가 성립한다.
        let r = recognize("txt", "a.txt", Some("Kotlin"), b"");
        assert_eq!(r, Recognition::FirstClass(Language::Kotlin));
        let r = recognize("kt", "A.kt", Some("Markdown"), b"");
        assert_eq!(r.language().unwrap().as_str(), "Markdown");
    }

    #[test]
    fn 선언이_셔뱅도_이긴다() {
        let r = recognize("", "gradlew", Some("Groovy"), b"#!/bin/sh\n");
        assert_eq!(r.language().unwrap().as_str(), "Groovy");
    }

    #[test]
    fn 추출기가_없어도_언어는_인식된다() {
        // **이것이 `unsupported` 와 `unrecognized` 를 가르는 자리다.**
        let r = 인식("sql", "V1__init.sql");
        assert_eq!(r.language().unwrap().as_str(), "SQL");
        assert!(matches!(r, Recognition::Known(_)));
    }

    #[test]
    fn 이름으로만_아는_것도_있다() {
        assert_eq!(인식("", "Dockerfile").language().unwrap().as_str(), "Dockerfile");
    }

    #[test]
    fn 모르는_것은_모른다고_한다() {
        // 목록을 넓히는 것이 대장을 더 정직하게 만들지 않는다.
        assert_eq!(인식("bak", "a.bak"), Recognition::Unknown);
        // **`.bak` 은 내용 휴리스틱(④)의 자리이고 그것을 세우지 않았다** — 백업 파일을
        // Kotlin 으로 인식하면 실코드가 아닌 것이 추출 대상이 된다.
        assert_eq!(인식("bak", "TenantTest.kt.bak"), Recognition::Unknown);
    }
}
