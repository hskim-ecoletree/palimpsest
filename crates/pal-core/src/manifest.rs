//! 프로젝트 매니페스트 — **범위는 선언에서 온다** (옛 F01 §3.5 · 옛 DESIGN §4.3).
//!
//! > *"어떤 저장소들이 한 프로젝트인가"는 코드에 없다.* 그러므로 매니페스트는
//! > `asserted` 이고, **대장은 항상 "선언된 저장소 N개"를 머리에 적는다.**
//!
//! # 없으면 없다고 적는다
//!
//! 매니페스트가 없을 때 조용히 경로에서 이름을 유도하면 **선언과 추정이 같아 보인다.**
//! [`ScopeSource`] 가 그 둘을 가르고, 그 값이 대장에 실린다 — `Capable` 이 산출에
//! 하는 일을 이것이 범위에 한다.

use serde::Deserialize;

use crate::glob::{Glob, GlobError};
use crate::ledger::ExclusionRuleId;
use crate::repo::{RepoId, RepoPath};

/// 매니페스트를 세울 수 없는 이유. **조용히 넘어가지 않는다.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ManifestError {
    /// TOML 이 아니거나 형태가 다르다.
    Syntax(String),
    /// 선언된 저장소가 없다. **0 개는 프로젝트가 아니다.**
    NoRepos,
    /// 제외 규칙의 패턴을 세우지 못했다.
    BadGlob { rule: String, source: GlobError },
    /// 저장소 식별자가 비었다 — [R-08] 이 요구하는 것은 **안정 식별자**다.
    EmptyRepoId,
}

impl std::fmt::Display for ManifestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Syntax(e) => write!(f, "매니페스트를 읽지 못했다: {e}"),
            Self::NoRepos => f.write_str("`[[repo]]` 가 하나도 없다 — 선언된 저장소 0 개는 프로젝트가 아니다"),
            Self::BadGlob { rule, source } => write!(f, "제외 규칙 `{rule}`: {source}"),
            Self::EmptyRepoId => f.write_str("저장소 `id` 가 비었다 — 경로도 URL 도 아닌 안정 식별자여야 한다 (R-08)"),
        }
    }
}

/// 제외 규칙 하나 — **ID 가 필수다.**
///
/// 제외 규칙을 넓히면 판정 대상이 줄고 *"잔여가 줄었다"* 로 보인다. 규칙 ID 가 있어야
/// **"범위가 줄어서 사라진 것"** 과 **"판정되어 사라진 것"** 을 나중에 구별할 수 있고,
/// 그때 이 ID 가 `ScopeReduction` 이 된다(옛 F01 §3.3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExclusionRule {
    pub id: ExclusionRuleId,
    pub glob: Glob,
}

/// 저장소 하나의 선언.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepoDecl {
    /// **안정 식별자.** 경로도 원격 URL 도 아니다 — 둘 다 움직인다([R-08]).
    pub id: RepoId,
    /// 매니페스트 파일 기준 상대 경로.
    pub path: String,
    pub exclude: Vec<ExclusionRule>,
}

/// `.palimpsest/manifest.toml`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Manifest {
    pub repos: Vec<RepoDecl>,
}

impl Manifest {
    /// 매니페스트를 읽는다.
    ///
    /// # Errors
    /// TOML 이 아니거나, 저장소가 없거나, 제외 규칙의 패턴을 세우지 못하면.
    pub fn parse(text: &str) -> Result<Self, ManifestError> {
        let raw: RawManifest =
            toml::from_str(text).map_err(|e| ManifestError::Syntax(e.to_string()))?;
        if raw.repo.is_empty() {
            return Err(ManifestError::NoRepos);
        }
        let mut repos = Vec::with_capacity(raw.repo.len());
        for r in raw.repo {
            if r.id.trim().is_empty() {
                return Err(ManifestError::EmptyRepoId);
            }
            let mut exclude = Vec::new();
            for rule in r.exclude.map(|e| e.rules).unwrap_or_default() {
                let glob = Glob::new(&rule.glob)
                    .map_err(|source| ManifestError::BadGlob { rule: rule.id.clone(), source })?;
                exclude.push(ExclusionRule { id: ExclusionRuleId::new(rule.id), glob });
            }
            repos.push(RepoDecl { id: RepoId::new(r.id), path: r.path, exclude });
        }
        Ok(Self { repos })
    }

    /// 이 경로를 제외하는 첫 규칙. **없으면 범위 안이다.**
    ///
    /// 여럿이 걸려도 **첫 것**이 이긴다 — 그래야 같은 대장이 두 번 같게 나온다.
    /// 규칙 순서가 곧 우선순위이고 그것은 매니페스트가 정한다.
    #[must_use]
    pub fn excluded_by(&self, repo: &RepoId, path: &RepoPath) -> Option<&ExclusionRule> {
        self.repos
            .iter()
            .find(|r| &r.id == repo)?
            .exclude
            .iter()
            .find(|rule| rule.glob.matches(path.as_str()))
    }

    /// 제외 규칙 총수 — 대장이 *"규칙 N 개를 선언받았다"* 를 적는다.
    #[must_use]
    pub fn rule_count(&self) -> usize {
        self.repos.iter().map(|r| r.exclude.len()).sum()
    }
}

/// 이 대장의 범위가 어디서 왔는가. **선언과 추정을 가른다.**
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeSource {
    /// `.palimpsest/manifest.toml` 이 선언했다 — 출처는 `asserted`(옛 DESIGN §4.3).
    Declared {
        /// 선언된 저장소 수.
        repos: usize,
        /// 제외 규칙 수.
        rules: usize,
    },
    /// 매니페스트가 없어 경로에서 유도했다.
    ///
    /// **선언이 아니라 추정이다.** 저장소 식별자가 디렉터리 이름이므로 저장소를 옮기면
    /// 바뀌고([R-08]), 제외 규칙은 하나도 없다. 그 사실이 산출에 실려야 사용자가
    /// *"제외 0 건"* 을 정직한 답으로 읽지 않는다.
    InferredFromPath,
}

impl ScopeSource {
    /// 사람이 읽는 한 줄.
    #[must_use]
    pub fn describe(&self) -> String {
        match self {
            Self::Declared { repos, rules } => {
                format!("매니페스트가 선언 (저장소 {repos} · 제외 규칙 {rules})")
            }
            Self::InferredFromPath => {
                "매니페스트 없음 — 경로에서 유도했습니다 (선언이 아니라 추정)".to_owned()
            }
        }
    }
}

// ── TOML 표현 ────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct RawManifest {
    #[serde(default)]
    repo: Vec<RawRepo>,
}

#[derive(Deserialize)]
struct RawRepo {
    id: String,
    #[serde(default = "dot")]
    path: String,
    exclude: Option<RawExclude>,
}

fn dot() -> String {
    ".".to_owned()
}

#[derive(Deserialize)]
struct RawExclude {
    #[serde(default)]
    rules: Vec<RawRule>,
}

#[derive(Deserialize)]
struct RawRule {
    id: String,
    glob: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    const 예시: &str = r#"
[[repo]]
id     = "order-svc"
path   = "."
[repo.exclude]
rules  = [{ id = "vendor",   glob = "vendor/**" },
          { id = "fixtures", glob = "**/__fixtures__/**" }]
"#;

    #[test]
    fn f01_문서의_예시를_그대로_읽는다() {
        let m = Manifest::parse(예시).unwrap();
        assert_eq!(m.repos.len(), 1);
        assert_eq!(m.repos[0].id, RepoId::new("order-svc"));
        assert_eq!(m.rule_count(), 2);
    }

    #[test]
    fn 규칙_id_와_함께_제외된다() {
        let m = Manifest::parse(예시).unwrap();
        let id = RepoId::new("order-svc");
        let hit = m.excluded_by(&id, &RepoPath::new("vendor/a.kt")).unwrap();
        assert_eq!(hit.id.as_str(), "vendor");
        let hit = m.excluded_by(&id, &RepoPath::new("src/__fixtures__/x.json")).unwrap();
        assert_eq!(hit.id.as_str(), "fixtures");
    }

    #[test]
    fn 걸리지_않는_파일은_범위_안이다() {
        // **성한 것을 잡지 않는지가 합격선의 절반이다.**
        let m = Manifest::parse(예시).unwrap();
        let id = RepoId::new("order-svc");
        assert!(m.excluded_by(&id, &RepoPath::new("src/main/A.kt")).is_none());
        // 이름에 vendor 가 들어갈 뿐인 파일은 제외되지 않는다.
        assert!(m.excluded_by(&id, &RepoPath::new("src/vendor.kt")).is_none());
    }

    #[test]
    fn 다른_저장소의_규칙은_걸리지_않는다() {
        let m = Manifest::parse(예시).unwrap();
        assert!(m.excluded_by(&RepoId::new("other"), &RepoPath::new("vendor/a.kt")).is_none());
    }

    #[test]
    fn 저장소가_없으면_거부한다() {
        assert_eq!(Manifest::parse("").unwrap_err(), ManifestError::NoRepos);
    }

    #[test]
    fn 빈_식별자를_거부한다() {
        let e = Manifest::parse("[[repo]]\nid = \"  \"\n").unwrap_err();
        assert_eq!(e, ManifestError::EmptyRepoId);
    }

    #[test]
    fn 세우지_못하는_패턴은_거부한다() {
        // **조용히 넘기면 그 규칙은 아무것도 안 거르고 오류도 없다.**
        let text = "[[repo]]\nid=\"a\"\n[repo.exclude]\nrules=[{id=\"x\", glob=\"a[bc]/**\"}]\n";
        let e = Manifest::parse(text).unwrap_err();
        assert!(matches!(e, ManifestError::BadGlob { .. }));
    }

    #[test]
    fn 제외_규칙이_없어도_선언은_선언이다() {
        let m = Manifest::parse("[[repo]]\nid = \"a\"\n").unwrap();
        assert_eq!(m.rule_count(), 0);
        assert_eq!(m.repos[0].path, ".");
    }
}
