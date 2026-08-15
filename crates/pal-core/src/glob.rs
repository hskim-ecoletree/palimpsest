//! 경로 패턴 — **매니페스트의 제외 규칙과 `.gitattributes` 가 같은 매처를 쓴다.**
//!
//! # 왜 크레이트를 들이지 않는가
//!
//! `globset` 은 정확하지만 `aho-corasick` · `regex-automata` 를 끌고 온다. stack §3.4 는
//! P0 의 외부 크레이트 추가에 근거를 요구하고, 여기서 필요한 것은 gitignore 문법의
//! **부분집합**이다 — `*` · `**` · `?` 셋. 그 셋은 아래 백 줄이면 서고, 정확성은
//! 테스트가 진다.
//!
//! # 지원하지 않는 것을 조용히 넘기지 않는다
//!
//! 문자 클래스(`[abc]`)는 **거부한다**([`GlobError::Unsupported`]). 조용히 리터럴로
//! 취급하면 제외 규칙이 아무것도 안 걸러도 오류가 없고, 그러면 대장이
//! *"제외 0 건"* 을 정직한 답인 것처럼 낸다.

use std::fmt;

/// 패턴을 세울 수 없는 이유.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GlobError {
    /// 문자 클래스 등 이 매처가 다루지 않는 문법.
    Unsupported { pattern: String, what: &'static str },
    /// 빈 패턴.
    Empty,
}

impl fmt::Display for GlobError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unsupported { pattern, what } => {
                write!(f, "지원하지 않는 패턴 문법 `{what}`: {pattern}")
            }
            Self::Empty => f.write_str("빈 패턴"),
        }
    }
}

/// 경로 하나에 대는 패턴 — gitignore 문법의 부분집합.
///
/// | | |
/// |---|---|
/// | `*` | `/` 를 제외한 임의 문자열 |
/// | `**` | 세그먼트 경계를 넘는 임의 문자열 |
/// | `?` | `/` 를 제외한 한 문자 |
///
/// **`/` 가 (끝을 빼고) 없는 패턴은 파일 이름에 댄다** — git 과 같은 규칙이다.
/// `*.bat` 은 `a/b/c.bat` 에 걸리고 `vendor/**` 는 걸리지 않는다.
///
/// # 직렬화는 **원문 하나**다 (F12 가 처음 요구했다)
///
/// 파생 필드(`segments`·`name_only`·`dir_only`)를 그대로 실으면 되읽을 때
/// [`Glob::new`] 를 **안 지나간다** — 그러면 이 모듈이 *"검사하거나 거부하거나 둘 중
/// 하나만 한다"* 라고 세운 규율이 역직렬화 경로에서 조용히 꺼진다.
/// [`crate::ConfirmingSignal`] 이 같은 형태로 서 있다.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(into = "String", try_from = "String")]
pub struct Glob {
    /// 원문 — 오류·기록에 그대로 쓴다.
    source: String,
    /// `/` 로 나눈 세그먼트.
    segments: Vec<Segment>,
    /// 이름에만 대는가(패턴에 `/` 가 없었다).
    name_only: bool,
    /// 디렉터리만 가리키는가(패턴이 `/` 로 끝났다).
    dir_only: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Segment {
    /// `**` — 세그먼트 0 개 이상.
    AnyDepth,
    /// 그 밖의 세그먼트. 내부에 `*` · `?` 가 있을 수 있다.
    Pattern(String),
}

impl From<Glob> for String {
    fn from(g: Glob) -> Self {
        g.source
    }
}

impl TryFrom<String> for Glob {
    type Error = GlobError;

    /// **되읽기도 [`Glob::new`] 를 지난다** — 검사를 건너뛰는 문을 안 만든다.
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::new(&s)
    }
}

impl Glob {
    /// 패턴을 세운다.
    ///
    /// # Errors
    /// 비었거나 다루지 않는 문법이 있으면.
    pub fn new(pattern: &str) -> Result<Self, GlobError> {
        if pattern.is_empty() {
            return Err(GlobError::Empty);
        }
        if pattern.contains('[') {
            return Err(GlobError::Unsupported {
                pattern: pattern.to_owned(),
                what: "문자 클래스",
            });
        }
        let dir_only = pattern.ends_with('/');
        let trimmed = pattern.trim_end_matches('/');
        // 앞의 `/` 는 "루트 기준"을 뜻하고, 우리 경로는 이미 루트 기준이다.
        let body = trimmed.strip_prefix('/').unwrap_or(trimmed);
        let name_only = !body.contains('/');

        let segments = body
            .split('/')
            .map(|s| {
                if s == "**" { Segment::AnyDepth } else { Segment::Pattern(s.to_owned()) }
            })
            .collect();
        Ok(Self { source: pattern.to_owned(), segments, name_only, dir_only })
    }

    /// 원문.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.source
    }

    /// **디렉터리만 가리키는 패턴인가** — 파일에 대면 `matches` 가 언제나 거짓이다.
    #[must_use]
    pub const fn is_dir_only(&self) -> bool {
        self.dir_only
    }

    /// 이 경로가 걸리는가. `path` 는 저장소 루트 기준이고 구분자는 `/` 다.
    #[must_use]
    pub fn matches(&self, path: &str) -> bool {
        // `vendor/` 처럼 디렉터리를 가리키는 패턴은 그 **아래**의 파일에 건다.
        // git 은 디렉터리 자체를 무시 목록에 넣지만 대장이 세는 것은 파일이다.
        if self.dir_only {
            let mut expanded = Vec::with_capacity(self.segments.len() + 3);
            // 이름만 적힌 디렉터리 패턴(`build/`)은 **깊이를 가리지 않는다** — git 과 같다.
            if self.name_only {
                expanded.push(Segment::AnyDepth);
            }
            expanded.extend(self.segments.iter().cloned());
            // `*` 하나를 반드시 먹게 해 **`build` 라는 이름의 파일**이 걸리지 않게 한다.
            // 뒤에 `**` 만 두면 그것이 0 개를 먹어 파일 자신이 디렉터리인 척한다.
            expanded.push(Segment::Pattern("*".to_owned()));
            expanded.push(Segment::AnyDepth);
            let parts: Vec<&str> = path.split('/').collect();
            return match_segments(&expanded, &parts);
        }
        if self.name_only {
            let name = path.rsplit('/').next().unwrap_or(path);
            return match_segments(&self.segments, &[name]);
        }
        let parts: Vec<&str> = path.split('/').collect();
        match_segments(&self.segments, &parts)
    }
}

/// 세그먼트 목록을 경로 조각들에 댄다 — `**` 때문에 되돌아갈 수 있다.
fn match_segments(pattern: &[Segment], parts: &[&str]) -> bool {
    match pattern.first() {
        None => parts.is_empty(),
        Some(Segment::AnyDepth) => {
            // `**` 는 0 개부터 전부까지 먹는다.
            (0..=parts.len()).any(|take| match_segments(&pattern[1..], &parts[take..]))
        }
        Some(Segment::Pattern(p)) => {
            let Some((head, rest)) = parts.split_first() else { return false };
            match_one(p, head) && match_segments(&pattern[1..], rest)
        }
    }
}

/// 세그먼트 하나 — `*` 와 `?` 만 다룬다. `/` 는 여기 오지 않는다.
fn match_one(pattern: &str, text: &str) -> bool {
    let p: Vec<char> = pattern.chars().collect();
    let t: Vec<char> = text.chars().collect();
    let (mut pi, mut ti) = (0usize, 0usize);
    // `*` 를 만난 자리와 그때 소비한 위치 — 실패하면 여기로 되돌아온다.
    let mut star: Option<(usize, usize)> = None;

    while ti < t.len() {
        if pi < p.len() && (p[pi] == '?' || p[pi] == t[ti]) {
            pi += 1;
            ti += 1;
        } else if pi < p.len() && p[pi] == '*' {
            star = Some((pi, ti));
            pi += 1;
        } else if let Some((sp, st)) = star {
            // 별이 한 글자 더 먹는다.
            pi = sp + 1;
            ti = st + 1;
            star = Some((sp, st + 1));
        } else {
            return false;
        }
    }
    p[pi..].iter().all(|c| *c == '*')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 슬래시_없는_패턴은_이름에_댄다() {
        let g = Glob::new("*.bat").unwrap();
        assert!(g.matches("gradlew.bat"));
        // **git 과 같은 규칙이다** — 깊이에 상관없이 이름이 걸린다.
        assert!(g.matches("a/b/c.bat"));
        assert!(!g.matches("gradlew"));
    }

    #[test]
    fn 슬래시가_있으면_경로_전체에_댄다() {
        let g = Glob::new("vendor/**").unwrap();
        assert!(g.matches("vendor/a.kt"));
        assert!(g.matches("vendor/deep/nested/a.kt"));
        assert!(!g.matches("src/vendor.kt"));
        // `**` 는 0 개도 먹으므로 `vendor` 자신도 걸린다.
        assert!(g.matches("vendor"));
    }

    #[test]
    fn 가운데의_이중별은_깊이를_넘는다() {
        let g = Glob::new("**/__fixtures__/**").unwrap();
        assert!(g.matches("src/test/__fixtures__/a.json"));
        assert!(g.matches("__fixtures__/a.json"));
        assert!(!g.matches("src/__fixtures__x/a.json"));
    }

    #[test]
    fn 단일별은_세그먼트를_넘지_않는다() {
        let g = Glob::new("src/*.kt").unwrap();
        assert!(g.matches("src/A.kt"));
        // **이것이 `*` 와 `**` 를 가르는 자리다.**
        assert!(!g.matches("src/main/A.kt"));
    }

    #[test]
    fn 디렉터리_패턴은_그_아래를_건다() {
        let g = Glob::new("build/").unwrap();
        assert!(g.is_dir_only());
        assert!(g.matches("build/out.txt"));
        assert!(g.matches("build/a/b.txt"));
        // 이름만 적힌 디렉터리 패턴은 깊이를 가리지 않는다.
        assert!(g.matches("app/build/out.txt"));
        // **`build` 라는 이름의 파일은 디렉터리가 아니다.**
        assert!(!g.matches("build"));
        assert!(!g.matches("a/build"));
    }

    #[test]
    fn 물음표는_한_글자다() {
        let g = Glob::new("a?.kt").unwrap();
        assert!(g.matches("ab.kt"));
        assert!(!g.matches("abc.kt"));
        assert!(!g.matches("a.kt"));
    }

    #[test]
    fn 별이_여럿인_패턴이_되돌아간다() {
        // 순진한 좌우 매칭은 여기서 틀린다.
        let g = Glob::new("*a*b.kt").unwrap();
        assert!(g.matches("xaybzb.kt"));
        assert!(!g.matches("xbya.kt"));
    }

    #[test]
    fn 문자_클래스는_조용히_넘기지_않고_거부한다() {
        // 리터럴로 취급하면 규칙이 아무것도 안 걸러도 오류가 없다 — 그것이
        // "제외 0 건" 을 정직한 답인 것처럼 만든다.
        let e = Glob::new("a[bc].kt").unwrap_err();
        assert!(matches!(e, GlobError::Unsupported { .. }));
        assert!(matches!(Glob::new("").unwrap_err(), GlobError::Empty));
    }
}
