//! 파일 하나에서 나온 전부.
//!
//! **핵심 성질은 파일 하나에만 의존한다는 것이다**(F02 §2). 다른 파일을 보지 않으므로
//! 완전 병렬이고(#49), 1층 콘텐츠 주소 캐시(F04)의 값이 될 수 있다(stack §5.2의
//! `ExtractCache` 포트). 그 성질이 깨지면 **둘 다 무너진다.**
//!
//! # `Coord` 가 여기 없다
//!
//! [`crate::Symbol`] 이 이미 그렇게 서 있다 — *"좌표는 저장소·트리·추출기 버전을 알아야
//! 하고 그것들은 파일 하나 바깥의 사실이다."* `FileGraph` 가 그 규율을 이어받는다.
//! 그래서 **같은 blob 을 다른 저장소·다른 경로에 두고 추출하면 바이트 단위로 같다** —
//! `corpus/criteria.toml` `[f02.1.pass]` ② 가 그것을 판정한다.
//!
//! # 지금 없는 자리 — **비어 있는 것이 아니라 아직 필드가 아니다**
//!
//! F02 §2 의 `FileGraph` 는 여기에 더해 `scopes`·`local_refs`·`raw_refs`·`export_digest`
//! 를 갖는다. **그것들을 빈 값으로 미리 세우지 않는다** — 빈 스코프 목록은 *"스코프가
//! 없는 파일"* 과 *"이 빌드가 스코프를 안 만든다"* 를 같은 출력으로 만들고, 그것이
//! [`crate::Capable`] 이 존재하는 이유의 정면 위반이다.
//!
//! 자리와 소유자: 스코프 체인은 **#48**(L2a · R-22), 파일 간 참조 해소는 **F07**.
//!
//! # 왜 `Deserialize` 가 없는가
//!
//! [`FileGraph::exports`]·[`FileGraph::imports`] 가 [`Capable`] 이고, `Capable` 은
//! **역직렬화되지 않는다** — 능력은 **빌드의 사실**이지 저장된 사실이 아니다.
//! 옛 캐시에서 `Present` 를 읽어 오면 그 빌드가 만들지 않는 능력을 있다고 답하게 된다.
//! 그래서 1층 캐시가 이 타입을 값으로 가질 때(F04) 능력 축은 키로 가야 하고,
//! 그 판단은 **이 조각이 하지 않는다.** 지금은 쓰기만 한다.

use serde::Serialize;

use crate::capable::Capable;
use crate::ledger::{ExtractGrade, LanguageId};
use crate::symbol::Symbol;

/// [`FileGraph::symbols`] 안의 자리. **파일 안에서만 뜻이 있다.**
///
/// 파일 밖에서 심볼을 가리키는 것은 `SymbolId` 이고 그것은 좌표를 요구한다(F03).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct LocalIx(pub u32);

/// 포함 관계 하나 — F02 의 **C1**.
///
/// # 왜 `(LocalIx, LocalIx)` 가 아닌가
///
/// F02 §2 는 `contains: Vec<(LocalIx, LocalIx)>` 로 적었다. **이름을 붙여 갈랐다** —
/// 벌거벗은 쌍은 인자가 뒤바뀌어도 타입이 잡지 못하고, 뒤바뀌면 *"메서드가 클래스를
/// 담는다"* 가 조용히 참이 된다. 이 저장소가 막으려는 것이 정확히 그 형태다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Containment {
    /// 담는 쪽 — 예: 클래스.
    pub parent: LocalIx,
    /// 담기는 쪽 — 예: 그 클래스의 메서드.
    pub child: LocalIx,
}

/// 이 파일이 밖에 노출하는 것.
///
/// **파일 하나만 보고 알 수 있는 데까지다.** `export * from '…'` 이 무슨 이름을
/// 내보내는지는 그 모듈을 읽어야 알고, 그것은 F07(스티칭)이다. 그래서 별 재수출은
/// **이름이 아니라 대상 모듈로** 남는다 — 모르는 것을 안다고 하지 않는다.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct ExportSet {
    /// 이름으로 내보내는 것. **정렬·중복 제거** — 소스 순서에 의존하지 않는 집합이다.
    pub names: Vec<String>,
    /// `export * from '…'` 의 대상 모듈 지정자. 정렬·중복 제거.
    pub star_from: Vec<String>,
    /// 기본 내보내기가 있는가.
    pub has_default: bool,
}

/// 이 파일이 참조하는 외부 모듈.
///
/// **지정자만이다. 그것이 어느 파일인지는 이 조각이 답하지 않는다**(F07).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct ImportSet {
    /// 모듈 지정자. 정렬·중복 제거. 동적 `import()` 는 **리터럴 인자만** 담는다.
    pub modules: Vec<String>,
}

/// 파일 하나의 추출 산출.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FileGraph {
    /// 이 파일의 언어. 추출기가 실제로 붙은 언어다.
    pub language: LanguageId,
    /// **이 추출기가 실제로 도달한 등급.** 선언 상한이 아니다.
    pub grade: ExtractGrade,
    /// 이 파일이 정의하는 것 — **소스 순서**.
    pub symbols: Vec<Symbol>,
    /// 포함 관계(C1). 최상위만 보는 추출기에서는 비고, **그것이 정확한 값이다** —
    /// 담긴 심볼을 아예 뽑지 않았으므로 담는 관계도 없다.
    pub contains: Vec<Containment>,
    /// 밖에 노출하는 것 — **만들지 않는 추출기에서는 빈 집합이 아니라 `NotBuilt` 다.**
    ///
    /// 빈 [`ExportSet`] 은 *"아무것도 안 내보내는 파일"* 이라는 뜻이고, 그것은 Kotlin
    /// 최상위 선언에 대해 **거짓이다**(기본이 `public`). 안 만든 것을 안 만들었다고
    /// 적는 자리가 [`Capable`] 이다.
    pub exports: Capable<ExportSet>,
    /// 참조하는 외부 모듈 — 위와 같은 이유로 [`Capable`].
    pub imports: Capable<ImportSet>,
    /// tree-sitter 가 오류 회복한 지점의 수. **0 이면 `parsed`, 아니면 `partial`.**
    ///
    /// 회복 **지점**의 좌표(`Site`)와 그 회복을 1급으로 다루는 것은 **#47** 이다.
    pub recovery_sites: usize,
}

impl FileGraph {
    /// 포함 관계도 export/import 도 없이 — **최상위 선언만** 뽑는 추출기의 산출.
    ///
    /// `capability` 는 그 추출기가 **안 만드는** 것의 정체다.
    #[must_use]
    pub fn flat(
        language: LanguageId,
        grade: ExtractGrade,
        symbols: Vec<Symbol>,
        recovery_sites: usize,
        exports: Capable<ExportSet>,
        imports: Capable<ImportSet>,
    ) -> Self {
        Self {
            language,
            grade,
            symbols,
            contains: Vec::new(),
            exports,
            imports,
            recovery_sites,
        }
    }

    /// 이 파일이 성하게 파싱됐는가.
    #[must_use]
    pub const fn is_whole(&self) -> bool {
        self.recovery_sites == 0
    }

    /// `child` 를 직접 담는 심볼.
    ///
    /// 포함 관계는 **한 부모만** 갖는다. 여럿이면 그것은 추출기의 결함이고, 여기서는
    /// 처음 것을 낸다 — 세는 쪽(`contains`)이 그 불변식을 지킬 책임을 진다.
    #[must_use]
    pub fn parent_of(&self, child: LocalIx) -> Option<LocalIx> {
        self.contains.iter().find(|c| c.child == child).map(|c| c.parent)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coord::BodyDigest;
    use crate::symbol::{Span, SymbolKind};

    fn 심볼(name: &str) -> Symbol {
        Symbol {
            name: name.to_owned(),
            kind: SymbolKind::Class,
            span: Span { byte_start: 0, byte_end: 1, line_start: 1, line_end: 1 },
            body: BodyDigest::of_normalized(name.as_bytes()),
        }
    }

    fn 안만듦() -> Capable<ExportSet> {
        Capable::not_built(crate::CapabilityId::new("F02", "kotlin-exports"))
    }

    fn 평평한(symbols: Vec<Symbol>, recovery_sites: usize) -> FileGraph {
        FileGraph::flat(
            LanguageId::new("Kotlin"),
            ExtractGrade::L1,
            symbols,
            recovery_sites,
            안만듦(),
            Capable::not_built(crate::CapabilityId::new("F02", "kotlin-imports")),
        )
    }

    #[test]
    fn 최상위만_뽑은_그래프는_포함_관계가_없다() {
        // **비어 있는 것이 정확한 값이다** — 담긴 심볼을 안 뽑았으므로 담는 관계도 없다.
        let g = 평평한(vec![심볼("A")], 0);
        assert!(g.contains.is_empty());
        assert_eq!(g.parent_of(LocalIx(0)), None);
    }

    #[test]
    fn 안_만든_export_는_빈_집합이_아니라_notbuilt_다() {
        // **거짓 안전이 죽는 자리다.** 빈 `ExportSet` 은 *"아무것도 안 내보낸다"* 이고
        // Kotlin 최상위 선언에 대해 그것은 거짓이다(기본이 public).
        let g = 평평한(vec![심볼("A")], 0);
        assert!(!g.exports.is_present(), "안 만든 능력이 값으로 위장했다");
        assert!(!g.imports.is_present());
    }

    #[test]
    fn 회복_지점이_없으면_성한_파일이다() {
        assert!(평평한(vec![], 0).is_whole());
        assert!(!평평한(vec![], 2).is_whole());
    }

    #[test]
    fn 포함_관계는_부모를_가리킨다() {
        // 이름을 붙여 가른 이유가 이 시험이다 — 벌거벗은 쌍이면 뒤바뀌어도 통과한다.
        let mut g = 평평한(vec![심볼("C"), 심볼("m")], 0);
        g.contains.push(Containment { parent: LocalIx(0), child: LocalIx(1) });
        assert_eq!(g.parent_of(LocalIx(1)), Some(LocalIx(0)));
        assert_eq!(g.parent_of(LocalIx(0)), None, "부모와 자식이 뒤바뀌었다");
    }
}
