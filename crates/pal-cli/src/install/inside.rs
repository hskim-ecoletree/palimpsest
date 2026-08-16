//! **대상 안인가** — 경로가 파일시스템에 닿기 전에 지나는 **단 하나의 문**.
//!
//! # 왜 문을 하나로 모으는가
//!
//! 매니페스트는 **대상 프로젝트 안에 사는 파일**이다. 남의 저장소에
//! `.claude/pal/manifest.json` 이 커밋돼 있으면 `pal uninstall` 한 번이 **그 파일이 적은
//! 아무 경로나** 지운다. 검사를 부르는 자리마다 흩어 놓으면 **다음에 필드를 더하는
//! 사람이 한 자리를 빠뜨리고, 그 자리는 조용히 통과한다.**
//!
//! 그래서 경로는 [`Rel`] 이라는 **타입**으로 산다. `Rel` 은 문자열로만 보이고,
//! **파일시스템 경로가 되는 길이 [`Root::join`] 하나뿐**이다. 매니페스트에 `Rel` 필드를
//! 더하면 그 필드도 자동으로 이 문을 지난다 — 검사를 잊을 자리가 없다.
//!
//! # F04 가 이미 적은 문장
//!
//! *"낱말 없이도 상위 디렉터리를 지울 수 있고 **`..` 하나면 경계가 사라진다**."*
//! `Path::join` 은 **절대 경로를 받으면 base 를 통째로 버린다** — `join` 만으로는
//! 경계가 아니다.

use std::fmt;
use std::path::{Component, Path, PathBuf};

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

/// 대상 프로젝트의 뿌리. **정규화된 절대 경로다.**
#[derive(Clone)]
pub struct Root(PathBuf);

impl Root {
    /// 대상 경로를 확정한다. **없으면 만들지 않는다** — 오타 하나로 남의 자리에 트리를
    /// 세우는 것이 이 명령의 가장 조용한 실패다.
    ///
    /// # Errors
    /// 디렉터리가 아니거나 정규화하지 못하면.
    pub fn 세운다(target: &Path) -> Result<Self> {
        if !target.is_dir() {
            bail!("대상이 디렉터리가 아니다: {}", target.display());
        }
        let real = target
            .canonicalize()
            .with_context(|| format!("대상 경로를 확정하지 못했다: {}", target.display()))?;
        Ok(Self(real))
    }

    #[must_use]
    pub fn path(&self) -> &Path {
        &self.0
    }

    /// 대상 안의 자리 하나.
    ///
    /// **밖으로 나가면 실패한다.** 나가는 항목은 **건드리지 않고** 그 사실을 낸다 —
    /// 「그 항목만 건너뛰고 나머지는 계속」이 아니다. 매니페스트 하나가 남의 자리를
    /// 가리키고 있으면 그 매니페스트 전체를 사람이 봐야 한다.
    ///
    /// # Errors
    /// 절대 경로거나, `..` 가 들어 있거나, 비어 있거나, **심링크를 따라가면 대상
    /// 밖으로 나가면.**
    pub fn join(&self, rel: &Rel) -> Result<PathBuf> {
        if let Some(까닭) = 글자로_벗어나나(&rel.0) {
            bail!(
                "`{rel}` 은 **대상 밖**이다 ({까닭}) — 대상은 {self} 다.\n    \
                 매니페스트는 대상 프로젝트 안에 사는 파일이고, 그것이 적은 경로가 밖을 \
                 가리키면 **아무것도 건드리지 않는다.** 사람이 봐야 한다"
            );
        }
        let candidate = self.0.join(&rel.0);
        let real = 실제_경로(&candidate, 0)?;
        if !real.starts_with(&self.0) {
            bail!(
                "`{rel}` 은 심링크를 따라가면 **대상 밖**으로 나간다 — {} 다. 대상은 \
                 {self} 다.\n    소유자의 문장은 **\"`~/.claude/` 하위에 기대는 구조는 \
                 절대 있어서는 안 돼\"** 였다. **안을 가리키는 심링크는 살리고 밖으로 \
                 나가는 것은 막는다** — 여기서 멈춘다",
                real.display()
            );
        }
        // ★ **확정한 것이 아니라 원래 경로를 낸다.** 확정한 것을 쓰면 대상 **안**을
        // 가리키는 심링크가 일반 파일로 바뀌고 모드·하드링크가 함께 소실된다.
        Ok(candidate)
    }
}

impl fmt::Display for Root {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.display())
    }
}

/// 대상 안의 **상대 경로**. **구분자는 언제나 `/` 다.**
///
/// ⚠ `AsRef<Path>` 도 `Deref` 도 **일부러 안 단다.** 달면 `target.join(entry.path)` 이
/// 다시 자라고, 그러면 이 파일이 막으려는 형태가 그대로 돌아온다.
///
/// # ⚠ 경로 구분자 가정 — **매니페스트는 기계 사이를 오간다**
///
/// 소유자 결정(2026-08-16): *"windows 를 대응한다는 가정하에 앞으로 모든 설계와 개발이
/// 되어야 해."* 이 타입이 지는 문자열은 매니페스트에 **그대로 실려** 커밋과 함께
/// 움직인다. 그래서 [`pal_core::RepoPath`] 와 같은 불변식을 진다 — *"Windows 에서 만든
/// 대장과 macOS 에서 만든 대장이 달라지면 대조가 성립하지 않는다"*.
///
/// 지금 이 불변식이 서는 근거는 **`Rel` 이 태어나는 자리가 둘뿐**이라는 것이다:
/// [`super::layout`] 의 컴파일된 `/` 상수와, 매니페스트에서 읽은 문자열. 파일시스템을
/// 훑어 이름을 만드는 유일한 자리(`manifest::walk`)는 문자열 쪽에서 `\` → `/` 를
/// **이미 하고 있고**, 그래서 Windows 빌드도 `\` 가 든 `Rel` 을 못 만든다.
///
/// **파일시스템 경로에서 `Rel` 을 만드는 길을 새로 여는 사람은 그 자리에서 `\` → `/`
/// 를 해야 한다.** 안 하면 두 플랫폼의 매니페스트가 갈리고, 그때 깨지는 것은 설치가
/// 아니라 **대조**다.
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[serde(transparent)]
pub struct Rel(String);

impl Rel {
    #[must_use]
    pub fn new(s: &str) -> Self {
        Self(s.to_owned())
    }

    /// **보고와 비교에만 쓴다.** 경로가 아니다.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Rel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// 이 상대 경로가 **글자만으로** 대상을 벗어나는가.
///
/// 절대 경로 · `..` · 빈 경로 셋이다. 파일시스템에 물어보기도 전에 답이 나오는 것들이라
/// 여기서 먼저 끊는다.
fn 글자로_벗어나나(rel: &str) -> Option<&'static str> {
    if rel.is_empty() {
        return Some("빈 경로다");
    }
    let path = Path::new(rel);
    if path.is_absolute() {
        return Some("절대 경로다 — `Path::join` 은 절대 경로를 받으면 base 를 통째로 버린다");
    }
    for c in path.components() {
        match c {
            Component::ParentDir => return Some("`..` 가 들어 있다"),
            Component::RootDir | Component::Prefix(_) => return Some("뿌리에서 시작한다"),
            Component::CurDir | Component::Normal(_) => {}
        }
    }
    None
}

/// 심링크를 몇 겹까지 따라가나. **고리가 있으면 여기서 멈춘다.**
const 심링크_한도: u32 = 40;

/// 이 경로가 **실제로 앉는 자리.** 없는 자리는 있는 조상까지 확정하고 이름을 잇는다.
///
/// ⚠ **끊긴 심링크를 손으로 푼다.** `canonicalize` 는 끊긴 링크에서 실패하는데, 거기서
/// 「없는 자리」로 넘기면 `settings.json → 밖/없는파일` 같은 형태가 **밖에 파일을
/// 만든다** — 쓰기는 링크를 따라가기 때문이다.
fn 실제_경로(p: &Path, 깊이: u32) -> Result<PathBuf> {
    if 깊이 > 심링크_한도 {
        bail!("심링크가 {심링크_한도}겹을 넘는다 — 고리일 수 있다: {}", p.display());
    }
    if let Ok(real) = p.canonicalize() {
        return Ok(real);
    }
    if p.symlink_metadata().is_ok_and(|m| m.file_type().is_symlink()) {
        let target = std::fs::read_link(p)
            .with_context(|| format!("심링크를 읽지 못했다: {}", p.display()))?;
        let joined = if target.is_absolute() {
            target
        } else {
            p.parent().unwrap_or(Path::new("")).join(target)
        };
        return 실제_경로(&joined, 깊이 + 1);
    }
    match (p.parent(), p.file_name()) {
        (Some(parent), Some(name)) => Ok(실제_경로(parent, 깊이 + 1)?.join(name)),
        _ => bail!("경로를 확정하지 못했다: {}", p.display()),
    }
}

#[cfg(test)]
mod tests {
    use super::글자로_벗어나나;

    #[test]
    fn 절대_경로와_상위_참조를_글자로_끊는다() {
        for 나쁜 in ["/etc/passwd", "../밖", "a/../../밖", "a/..", ""] {
            assert!(글자로_벗어나나(나쁜).is_some(), "`{나쁜}` 를 안 끊었다");
        }
    }

    #[test]
    fn 안쪽_경로는_안_끊는다() {
        for 좋은 in [".claude/pal/manifest.json", "CLAUDE.md", "./a/b"] {
            assert!(글자로_벗어나나(좋은).is_none(), "`{좋은}` 를 끊었다");
        }
    }
}
