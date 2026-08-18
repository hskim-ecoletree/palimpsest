//! `.gitattributes` — **세 소비자가 같은 파일을 읽는다.**
//!
//! | 무엇이 필요한가 | 왜 |
//! |---|---|
//! | `text` 속성 | 워킹트리 파일의 blob 이름을 git 과 같게 계산하려면 **CRLF→LF 를 먼저 되돌려야** 한다 |
//! | `linguist-language` | 언어 인식 4단계의 ③ (옛 F01 §3.3) |
//! | (같은 매처) | 매니페스트의 제외 규칙이 [`crate::Glob`] 을 공유한다 |
//!
//! # 이것이 실물 결함에서 나왔다
//!
//! F01 의 워킹트리 요약이 `gradlew.bat` 하나에서 git 과 다른 blob 이름을 냈다.
//! 저장소의 blob 은 LF(2843바이트)이고 워킹트리 파일은 CRLF(2937바이트)다 —
//! `*.bat text eol=crlf` 가 체크아웃에서 CRLF 를 넣었기 때문이다. **git 은 반대 방향
//! (clean)에서 그것을 되돌리고, 그것을 안 하면 깨끗한 워킹트리가 dirty 로 보인다.**
//!
//! # 이 크레이트는 파일을 읽지 않는다
//!
//! [`Attributes::parse`] 는 `(디렉터리, 내용)` 쌍을 받는다. 어디서 읽을지는
//! `pal-git`(워킹트리·트리)의 일이다 — `pal-core` 가 I/O 를 하면 그것이 곧
//! 도메인이 환경에 종속되는 자리다.

use crate::capable::Declared;
use crate::glob::{Glob, GlobError};
use crate::repo::RepoPath;

/// 파일 하나에 걸린 속성 — **미지정과 꺼짐은 다르다.**
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FileAttributes {
    /// `text` · `-text` · `binary`. [`Declared::Unspecified`] 면 **아무도 말하지
    /// 않은 것**이고 그때는 `core.autocrlf` 가 정한다 — 그 설정을 보는 것은 `pal-git` 이다.
    ///
    /// **`Option<bool>` 이 아니다** — 그러면 *"꺼져 있다"* 와 *"미지정"* 이 둘로 접히고,
    /// 그것이 stack §5.4 가 금한 자리다(`cargo xtask check` 가 잡는다).
    pub text: Declared<bool>,
    /// `linguist-language=<이름>`. 언어 인식 ③ 단계가 이것을 읽는다.
    pub language: Declared<String>,
}

impl FileAttributes {
    /// **줄 하나가 이긴 만큼만 덮어쓴다.** 미지정 속성은 앞의 판정을 지우지 않는다 —
    /// git 의 규칙이고, 그렇지 않으면 뒤에 오는 `*.kt diff` 같은 줄이 앞의 `text` 를
    /// 조용히 지운다.
    fn apply(&mut self, other: &Self) {
        if other.text.is_set() {
            self.text = other.text;
        }
        if other.language.is_set() {
            self.language.clone_from(&other.language);
        }
    }
}

/// 규칙 하나 — 어느 디렉터리의 `.gitattributes` 몇째 줄인가까지 안다.
#[derive(Debug, Clone)]
struct Rule {
    /// 이 규칙이 사는 디렉터리. 루트면 빈 문자열.
    dir: String,
    glob: Glob,
    attrs: FileAttributes,
}

/// 저장소 하나의 `.gitattributes` 전부.
#[derive(Debug, Clone, Default)]
pub struct Attributes {
    /// **적용 순서대로** 정렬돼 있다 — 얕은 디렉터리 먼저, 같은 파일 안에서는 줄 순서.
    /// 뒤에 오는 것이 이긴다.
    rules: Vec<Rule>,
    /// 세우지 못한 패턴 — **버리지 않고 센다.**
    skipped: Vec<String>,
}

impl Attributes {
    /// `(디렉터리, 파일 내용)` 들을 읽는다. 디렉터리는 저장소 루트 기준이고 루트는 `""`.
    ///
    /// **깊은 디렉터리의 규칙이 나중에 적용되도록 정렬한다** — git 의 우선순위다.
    #[must_use]
    pub fn parse(files: &[(String, String)]) -> Self {
        let mut ordered: Vec<(usize, String, &str)> = files
            .iter()
            .map(|(dir, body)| (dir.matches('/').count() + usize::from(!dir.is_empty()), dir.clone(), body.as_str()))
            .collect();
        ordered.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));

        let mut rules = Vec::new();
        let mut skipped = Vec::new();
        for (_, dir, body) in ordered {
            for line in body.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                let mut parts = line.split_whitespace();
                let Some(pattern) = parts.next() else { continue };
                match Glob::new(pattern) {
                    Ok(glob) => rules.push(Rule {
                        dir: dir.clone(),
                        glob,
                        attrs: parse_attrs(parts),
                    }),
                    // **조용히 넘기지 않는다.** 세지 않으면 규칙이 안 걸린 것과
                    // 애초에 없던 것이 같아 보인다([`GlobError`] 의 주석과 같은 이유).
                    Err(GlobError::Unsupported { pattern, .. }) => skipped.push(pattern),
                    Err(GlobError::Empty) => {}
                }
            }
        }
        Self { rules, skipped }
    }

    /// 이 경로에 걸린 속성.
    #[must_use]
    pub fn of(&self, path: &RepoPath) -> FileAttributes {
        let full = path.as_str();
        let mut out = FileAttributes::default();
        for rule in &self.rules {
            let relative = if rule.dir.is_empty() {
                Some(full)
            } else {
                full.strip_prefix(&rule.dir).and_then(|r| r.strip_prefix('/'))
            };
            let Some(relative) = relative else { continue };
            if rule.glob.matches(relative) {
                out.apply(&rule.attrs);
            }
        }
        out
    }

    /// 세우지 못한 패턴들. **비어 있지 않으면 산출에 실린다.**
    #[must_use]
    pub fn skipped(&self) -> &[String] {
        &self.skipped
    }

    /// 규칙 수 — 대장이 *"속성 규칙 N 개를 읽었다"* 를 적는다.
    #[must_use]
    pub fn len(&self) -> usize {
        self.rules.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }
}

/// 한 줄의 속성들 — **우리가 쓰는 둘만 읽고 나머지는 넘긴다.**
///
/// `diff` · `merge` · `filter` 같은 것들은 이 도구가 하는 일과 무관하다. 넘기는 것과
/// 못 읽는 것은 다르고, 못 읽는 것은 위의 `skipped` 가 센다.
fn parse_attrs<'a>(parts: impl Iterator<Item = &'a str>) -> FileAttributes {
    let mut out = FileAttributes::default();
    for token in parts {
        match token {
            // `binary` 는 `-text -diff -merge` 의 줄임이다.
            "binary" | "-text" => out.text = Declared::Set(false),
            // `text=auto` 는 "바이너리가 아니면 텍스트" 인데, 바이너리 판정은 내용을
            // 봐야 한다. 여기서는 텍스트로 두고 NUL 검사가 뒤에서 가른다.
            "text" | "text=auto" => out.text = Declared::Set(true),
            other => {
                if let Some(name) = other.strip_prefix("linguist-language=") {
                    out.language = Declared::Set(name.to_owned());
                }
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn 속성(내용: &str, path: &str) -> FileAttributes {
        Attributes::parse(&[(String::new(), 내용.to_owned())]).of(&RepoPath::new(path))
    }

    #[test]
    fn 코퍼스의_gitattributes_를_그대로_읽는다() {
        // `boxwood/portal-backend@a29cad0bf6a8` 의 실물 내용이다.
        let body = "/gradlew text eol=lf\n*.bat text eol=crlf\n*.jar binary\n";
        assert_eq!(속성(body, "gradlew.bat").text, Declared::Set(true));
        assert_eq!(속성(body, "gradlew").text, Declared::Set(true));
        assert_eq!(속성(body, "gradle/wrapper/gradle-wrapper.jar").text, Declared::Set(false));
        // 걸리지 않은 파일은 **미지정**이지 "텍스트 아님"이 아니다.
        assert_eq!(속성(body, "src/main/kotlin/A.kt").text, Declared::Unspecified);
    }

    #[test]
    fn 뒤_규칙이_이긴다() {
        let body = "*.kt text\n*.kt -text\n";
        assert_eq!(속성(body, "A.kt").text, Declared::Set(false));
    }

    #[test]
    fn 미지정_속성은_앞의_판정을_지우지_않는다() {
        // 뒤 줄이 `text` 를 말하지 않으므로 앞의 `text` 가 남아야 한다.
        let body = "*.kt text\n*.kt diff\n";
        assert_eq!(속성(body, "A.kt").text, Declared::Set(true));
    }

    #[test]
    fn 깊은_디렉터리가_이긴다() {
        let attrs = Attributes::parse(&[
            (String::new(), "*.kt text\n".to_owned()),
            ("src/gen".to_owned(), "*.kt -text\n".to_owned()),
        ]);
        assert_eq!(attrs.of(&RepoPath::new("src/A.kt")).text, Declared::Set(true));
        assert_eq!(attrs.of(&RepoPath::new("src/gen/A.kt")).text, Declared::Set(false));
    }

    #[test]
    fn 하위_gitattributes_는_자기_아래에만_건다() {
        let attrs = Attributes::parse(&[("src/gen".to_owned(), "*.kt -text\n".to_owned())]);
        assert_eq!(attrs.of(&RepoPath::new("other/A.kt")).text, Declared::Unspecified);
    }

    #[test]
    fn linguist_언어를_읽는다() {
        let body = "*.pde linguist-language=Java\n";
        assert_eq!(속성(body, "sketch.pde").language.as_deref(), Some("Java"));
    }

    #[test]
    fn 못_세운_패턴은_버리지_않고_센다() {
        let attrs = Attributes::parse(&[(String::new(), "a[bc].kt text\n*.kt text\n".to_owned())]);
        assert_eq!(attrs.skipped().len(), 1);
        assert_eq!(attrs.len(), 1);
    }

    #[test]
    fn 주석과_빈_줄은_규칙이_아니다() {
        let attrs = Attributes::parse(&[(String::new(), "# c\n\n*.kt text\n".to_owned())]);
        assert_eq!(attrs.len(), 1);
    }
}
