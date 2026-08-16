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
    /// 절대 경로거나, `..` 가 들어 있거나, 비어 있으면.
    pub fn join(&self, rel: &Rel) -> Result<PathBuf> {
        if let Some(까닭) = 글자로_벗어나나(&rel.0) {
            bail!(
                "`{rel}` 은 **대상 밖**이다 ({까닭}) — 대상은 {self} 다.\n    \
                 매니페스트는 대상 프로젝트 안에 사는 파일이고, 그것이 적은 경로가 밖을 \
                 가리키면 **아무것도 건드리지 않는다.** 사람이 봐야 한다"
            );
        }
        Ok(self.0.join(&rel.0))
    }
}

impl fmt::Display for Root {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.display())
    }
}

/// 대상 안의 **상대 경로**.
///
/// ⚠ `AsRef<Path>` 도 `Deref` 도 **일부러 안 단다.** 달면 `target.join(entry.path)` 이
/// 다시 자라고, 그러면 이 파일이 막으려는 형태가 그대로 돌아온다.
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
