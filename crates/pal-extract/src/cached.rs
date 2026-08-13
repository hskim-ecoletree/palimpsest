//! 1층 캐시에 실리는 그래프 — **[`FileGraph`] 와 한 자리만 다르다.**
//!
//! 다른 한 자리는 **능력의 정체**다. `FileGraph` 의 네 자리는 [`Capable`] 이고
//! `Capable::NotBuilt` 는 [`pal_core::CapabilityId`] 를 담는데, 그 필드가
//! `&'static str` 이라 **애초에 역직렬화될 수 없다.** 그것이 우연이 아니다 —
//! *"이 빌드가 무엇을 만드는가"* 는 저장된 사실이 아니라 **빌드의 사실**이다.
//!
//! # 그래서 정체를 저장하지 않고 **되씌운다**
//!
//! 캐시에 실리는 것은 [`Slot`] 이고 그것은 *"이 자리가 만들어졌는가"* 만 담는다.
//! 되읽을 때 이 빌드의 껍데기([`crate::shell_of`])에서 정체를 가져와 씌운다.
//!
//! **그리고 씌우기가 어긋나면 오류다.** 캐시에는 `built` 인데 이 빌드가 안 만들거나
//! 그 반대이면, 그 항목은 다른 능력을 가진 빌드가 쓴 것이다 — 능력 축이 키에 있으므로
//! (`shell.rs`) **일어날 수 없고, 일어나면 키가 샌 것이다.** 조용히 넘기지 않는다.
//! 이것이 능력 축의 음성 대조다.

use pal_core::{
    Capable, Containment, ExportDigest, ExportSet, ExtractGrade, FileGraph, ImportSet, Language,
    LanguageId, RecoverySite, ScopeChain, Symbol,
};
use serde::{Deserialize, Serialize};

use crate::shell::{GraphShell, shell_of};

/// 캐시 안의 자리 하나. **[`Capable`] 이 아니다 — 능력의 정체를 담지 않는다.**
///
/// 담지 않는 것이 이 타입의 전부다. `Capable` 을 그대로 실으면 옛 빌드의
/// `CapabilityId` 가 새 빌드의 산출에 나가고, 그러면 *"F02 가 안 만들었다"* 가
/// F02 가 이미 만든 빌드에서도 그대로 보인다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Slot<T> {
    /// 이 빌드가 안 만든 자리였다. **어느 능력인지는 여기 없다.**
    NotBuilt,
    Built(T),
}

impl<T> Slot<T> {
    fn of(c: Capable<T>) -> Self {
        match c {
            Capable::Present(v) => Self::Built(v),
            Capable::NotBuilt { .. } => Self::NotBuilt,
        }
    }

    /// 이 빌드의 껍데기에서 정체를 가져와 씌운다.
    fn restore(self, shell: &Capable<()>, slot: &'static str) -> Result<Capable<T>, ShellMismatch> {
        match (self, shell) {
            (Self::Built(v), Capable::Present(())) => Ok(Capable::Present(v)),
            (Self::NotBuilt, Capable::NotBuilt { capability }) => {
                Ok(Capable::NotBuilt { capability: *capability })
            }
            (Self::Built(_), Capable::NotBuilt { .. }) => {
                Err(ShellMismatch { slot, cached_built: true })
            }
            (Self::NotBuilt, Capable::Present(())) => {
                Err(ShellMismatch { slot, cached_built: false })
            }
        }
    }
}

/// 캐시의 자리와 이 빌드의 능력이 어긋났다.
///
/// **능력 축이 키에 있으므로 일어날 수 없다.** 일어나면 키가 새는 것이고, 그 사실이
/// 조용한 오답이 되지 않게 오류로 낸다.
#[derive(Debug, thiserror::Error)]
pub enum RestoreError {
    #[error("캐시가 모르는 언어를 담았다: {0}")]
    UnknownLanguage(String),
    #[error("이 빌드에 {language} 추출기가 없는데 캐시에 그 언어의 그래프가 있다 — {capability}")]
    NoExtractor { language: String, capability: String },
    #[error(
        "캐시의 `{}` 자리가 이 빌드의 능력과 어긋난다 (캐시 {}, 빌드 {}) — \
         **능력 축이 키에 있으므로 일어날 수 없다. 키가 샜다**",
        .0.slot,
        if .0.cached_built { "만듦" } else { "안 만듦" },
        if .0.cached_built { "안 만듦" } else { "만듦" }
    )]
    Shell(ShellMismatch),
}

/// 어긋난 자리 하나.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShellMismatch {
    pub slot: &'static str,
    pub cached_built: bool,
}

/// 파일 하나의 추출 산출 — **캐시에 실리는 형태.**
///
/// [`FileGraph`] 의 필드를 그대로 담는다. 다른 것은 능력 자리 넷이 [`Slot`] 인 것뿐이다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CachedGraph {
    pub language: LanguageId,
    pub grade: ExtractGrade,
    pub symbols: Vec<Symbol>,
    pub contains: Vec<Containment>,
    /// **F02-2 가 F04 로 넘긴 자리다** — 전에는 개수만 실렸다.
    pub recovery_sites: Vec<RecoverySite>,
    exports: Slot<ExportSet>,
    imports: Slot<ImportSet>,
    export_digest: Slot<ExportDigest>,
    /// **F02-3 이 F04 로 넘긴 자리다** — 전에는 아예 안 실렸다.
    scopes: Slot<ScopeChain>,
}

impl CachedGraph {
    /// 그래프를 캐시의 형태로. **능력의 정체를 버린다.**
    #[must_use]
    pub fn of(graph: FileGraph) -> Self {
        Self {
            language: graph.language,
            grade: graph.grade,
            symbols: graph.symbols,
            contains: graph.contains,
            recovery_sites: graph.recovery_sites,
            exports: Slot::of(graph.exports),
            imports: Slot::of(graph.imports),
            export_digest: Slot::of(graph.export_digest),
            scopes: Slot::of(graph.scopes),
        }
    }

    /// 이 빌드의 능력을 씌워 그래프로 되돌린다.
    ///
    /// **이것이 F02-1 이 넘긴 빚의 답이다** — `FileGraph` 에 `Deserialize` 를 붙이는
    /// 대신 능력을 키로 보내고 여기서 되씌운다.
    ///
    /// # Errors
    /// 캐시의 자리와 이 빌드의 능력이 어긋나면 — **키가 샜다는 뜻이다.**
    pub fn restore(self) -> Result<FileGraph, RestoreError> {
        let name = self.language.as_str().to_owned();
        let language = Language::from_name(&name)
            .ok_or_else(|| RestoreError::UnknownLanguage(name.clone()))?;
        let shell: &GraphShell = match shell_of(language) {
            Capable::Present(s) => s,
            Capable::NotBuilt { capability } => {
                return Err(RestoreError::NoExtractor {
                    language: name,
                    capability: format!("{}/{}", capability.feature, capability.what),
                });
            }
        };

        let exports = self.exports.restore(&shell.exports, "exports").map_err(RestoreError::Shell)?;
        let imports = self.imports.restore(&shell.imports, "imports").map_err(RestoreError::Shell)?;
        let export_digest = self
            .export_digest
            .restore(&shell.export_digest, "export_digest")
            .map_err(RestoreError::Shell)?;
        let scopes = self.scopes.restore(&shell.scopes, "scopes").map_err(RestoreError::Shell)?;

        Ok(FileGraph {
            language: self.language,
            grade: self.grade,
            symbols: self.symbols,
            contains: self.contains,
            exports,
            imports,
            export_digest,
            scopes,
            recovery_sites: self.recovery_sites,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::extractor::extractor_for;

    fn 그래프(language: Language, source: &[u8]) -> FileGraph {
        let Capable::Present(e) = extractor_for(language) else {
            panic!("{} 추출기가 없다", language.name());
        };
        e.extract(source).expect("추출")
    }

    /// **왕복이 항등이어야 캐시가 재추출과 같다.** 이것이 재구축 등가성의 단위 시험이다.
    ///
    /// 직렬화까지 태우는 왕복은 `pal-cli` 의 통합 시험이 실제 캐시로 한다 — 여기서
    /// `postcard` 를 dev-의존으로 들이면 **의존을 늘리지 않는다**는 등록을 어긴다.
    fn 왕복(language: Language, source: &[u8]) {
        let 원본 = 그래프(language, source);
        let 되읽음 = CachedGraph::of(원본.clone()).restore().expect("되씌우기");
        assert_eq!(되읽음, 원본, "{} 왕복이 항등이 아니다", language.name());
    }

    #[test]
    fn typescript_그래프가_왕복한다() {
        // **스코프 체인과 export/import 가 실리는지가 여기서 보인다** — F02-2·F02-3 의 빚.
        왕복(Language::TypeScript, b"export class A { m() { const x = 1; return x } }\n");
    }

    #[test]
    fn kotlin_그래프가_왕복한다() {
        // 넷 다 `NotBuilt` 인 쪽. **빈 값으로 위장하지 않는다**는 것이 요점이다.
        왕복(Language::Kotlin, b"class A { fun m() {} }\n");
    }

    #[test]
    fn 회복_자리가_실린다() {
        // F02-2 의 빚: *"회복 자리가 1층 캐시에 안 실린다 — `FileOutcome` 은 개수만 담는다"*.
        let g = 그래프(Language::TypeScript, b"class A { \n");
        assert!(!g.recovery_sites.is_empty(), "회복 자리가 없으면 이 시험은 아무것도 재지 않는다");
        let 왕복 = CachedGraph::of(g.clone()).restore().expect("되씌우기");
        assert_eq!(왕복.recovery_sites, g.recovery_sites);
    }

    #[test]
    fn 자리가_어긋나면_조용히_넘기지_않는다() {
        // **★ 능력 축의 음성 대조다.** 다른 능력을 가진 빌드가 쓴 항목을 되읽으면
        // 그것이 오류로 나와야 한다 — 안 그러면 안 만든 능력이 값으로 위장한다.
        let kotlin = CachedGraph::of(그래프(Language::Kotlin, b"class A\n"));
        // Kotlin 그래프의 자리들을 TypeScript 껍데기에 씌우려 하면 넷 다 어긋난다.
        let Capable::Present(ts) = shell_of(Language::TypeScript) else { panic!() };
        let err = kotlin.scopes.restore(&ts.scopes, "scopes").expect_err("어긋남을 안 냈다");
        assert_eq!(err.slot, "scopes");
        assert!(!err.cached_built, "캐시가 안 만든 자리를 만들었다고 적었다");
    }
}
