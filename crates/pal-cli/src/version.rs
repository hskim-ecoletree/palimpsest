//! 이 빌드가 무엇인지 — **`[f24]` ⑨ 의 자리.**
//!
//! 게이트는 *"버전이 무엇이어야 하는지 안 정한다 — 두 빌드가 갈리는가만 잰다"* 고 적었다.
//! 그래서 여기서 고르는 것은 하나다: **패키지 버전 하나로는 두 커밋이 안 갈리므로
//! 커밋을 함께 싣는다.**
//!
//! 커밋은 `build.rs` 가 `PAL_COMMIT` 으로 넣는다. 그것이 없는 빌드(git 없는 tarball)
//! 에서도 서야 하고, 그때는 패키지 버전 하나로 선다 — **없는 것을 지어내지 않는다.**

use std::sync::LazyLock;

/// 이 빌드의 커밋. 릴리스 tarball 처럼 git 이 없는 곳에서 빌드하면 `None`.
const COMMIT: Option<&str> = option_env!("PAL_COMMIT");

/// `pal --version` 이 내는 문자열.
///
/// **`&'static str` 이다** — clap 의 `version` 이 그것을 요구하고, 그것 하나 때문에
/// clap 의 feature 를 늘리지 않는다(stack §3.4: 의존을 늘리기 전에 줄인다).
pub fn describe() -> &'static str {
    static VERSION: LazyLock<String> =
        LazyLock::new(|| compose(env!("CARGO_PKG_VERSION"), COMMIT));
    VERSION.as_str()
}

/// **순수 함수다** — 그래서 *"두 빌드가 갈리는가"* 를 시험으로 잴 수 있다.
///
/// `SemVer` 의 빌드 메타데이터 형태(`+`)를 쓴다. 비교에 안 쓰이는 자리이고, 우리가 그것을
/// 비교에 쓰지도 않는다 — 설치 경로는 이 문자열을 **같다/다르다**로만 본다.
fn compose(package: &str, commit: Option<&str>) -> String {
    match commit {
        Some(sha) if !sha.is_empty() => format!("{package}+{sha}"),
        _ => package.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::compose;

    /// **⑨ 가 재는 문장 그대로다** — 두 커밋이 갈린다.
    #[test]
    fn 커밋이_다르면_버전이_갈린다() {
        assert_ne!(compose("0.0.0", Some("aaaaaaaaaaaa")), compose("0.0.0", Some("bbbbbbbbbbbb")));
    }

    /// 그리고 **커밋이 없으면 안 갈린다** — 그 사실을 숨기지 않는다.
    /// 이 자리가 곧 옛 상태(`pal 0.0.0` 고정)이고, 반증 형태를 시험이 들고 있는다.
    #[test]
    fn 커밋이_없으면_패키지_버전_하나다() {
        assert_eq!(compose("0.0.0", None), "0.0.0");
        assert_eq!(compose("0.0.0", Some("")), "0.0.0");
    }

    #[test]
    fn 커밋이_있으면_패키지_버전이_남는다() {
        assert!(compose("1.2.3", Some("abcdef")).starts_with("1.2.3"));
    }
}
