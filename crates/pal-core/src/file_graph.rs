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

use serde::{Deserialize, Serialize};

use crate::capable::Capable;
use crate::coord::ExportDigest;
use crate::ledger::{ExtractGrade, LanguageId};
use crate::scope::ScopeChain;
use crate::symbol::{Span, Symbol};

/// [`FileGraph::symbols`] 안의 자리. **파일 안에서만 뜻이 있다.**
///
/// 파일 밖에서 심볼을 가리키는 것은 `SymbolId` 이고 그것은 좌표를 요구한다(F03).
///
/// # 되읽을 수 있다 — 그리고 그것이 [`Coord`] 와 다른 점이다
///
/// [`Coord`] 는 `Deserialize` 가 없다. 추출기 버전이 이 빌드에 박힌 상수라 밖에서 온
/// 좌표를 되읽으면 서로 다른 추출기의 산출이 같은 좌표계에 있는 것처럼 보이기 때문이다.
/// **이 값에는 그 문제가 없다** — 파일 하나 안의 자리이고, 1층 캐시의 키가 이미
/// `ExtractorVersion` 을 성분으로 갖는다(F03-1 · [`Containment`] 가 캐시에 실린다).
///
/// [`Coord`]: crate::Coord
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct LocalIx(pub u32);

/// 포함 관계 하나 — F02 의 **C1**.
///
/// # 왜 `(LocalIx, LocalIx)` 가 아닌가
///
/// F02 §2 는 `contains: Vec<(LocalIx, LocalIx)>` 로 적었다. **이름을 붙여 갈랐다** —
/// 벌거벗은 쌍은 인자가 뒤바뀌어도 타입이 잡지 못하고, 뒤바뀌면 *"메서드가 클래스를
/// 담는다"* 가 조용히 참이 된다. 이 저장소가 막으려는 것이 정확히 그 형태다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

impl ExportSet {
    /// 이 집합의 요약. **정렬·중복 제거된 뒤에 불러야 한다.**
    ///
    /// 성분 사이에 `\0` 을 넣는다 — 넣지 않으면 `["ab","c"]` 와 `["a","bc"]` 가 같은
    /// 값이 되고, 그것이 **서로 다른 표면을 하나로 만드는** 형태다(`SymbolId::compute`
    /// 와 같은 자리).
    #[must_use]
    pub fn digest(&self) -> ExportDigest {
        let mut h = blake3::Hasher::new();
        h.update(b"pal-exports-v1\0");
        h.update(if self.has_default { b"default\0" } else { b"\0" });
        for n in &self.names {
            h.update(n.as_bytes());
            h.update(b"\0");
        }
        h.update(b"star\0");
        for m in &self.star_from {
            h.update(m.as_bytes());
            h.update(b"\0");
        }
        ExportDigest::from_bytes(*h.finalize().as_bytes())
    }
}

/// 이 파일이 참조하는 외부 모듈.
///
/// **지정자만이다. 그것이 어느 파일인지는 이 조각이 답하지 않는다**(F07).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct ImportSet {
    /// 모듈 지정자. 정렬·중복 제거. 동적 `import()` 는 **리터럴 인자만** 담는다.
    pub modules: Vec<String>,
}

/// 파서가 회복한 자리 하나가 **무엇이었나**.
///
/// 둘을 가르지 않으면 `MISSING` 이 구별되지 않는다 — 그것은 **너비가 0 인 자리**이고,
/// span 만 보면 *"아무 데도 아닌 곳"* 과 같은 값이 된다. 사용자가 보아야 하는 것은
/// *"여기에 무엇이 빠졌다"* 이지 빈 범위가 아니다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryKind {
    /// `ERROR` — 문법에 맞지 않는 토큰이 있었다. 그 범위를 삼킨다.
    Error,
    /// `MISSING` — 문법이 요구하는 토큰이 없어 파서가 **지어 넣었다.** 너비가 0 이다.
    Missing,
}

/// 파서가 회복한 자리 하나 — **개수가 아니라 자리다.**
///
/// # `Coord` 가 아니라 `Span` 인 이유
///
/// 옛 코드는 *"회복 지점의 좌표(`Site`)는 F03 이후다 — 좌표에 `symbol` 성분이 필요하다"*
/// 라고 적고 개수만 셌다. **그 이유가 성립하지 않는다.** 파일 안의 바이트 범위는 파일
/// 하나만 보면 알고, [`FileGraph`] 는 `Coord` 를 싣지 않는다(`[f02.1.pass]` ②).
/// 여기서 필요한 것은 `span` 이지 `Coord` 가 아니다.
///
/// **개수만으로는 사용자가 어디를 못 읽었는지 모른다.** 그것이 *"공백이 순위를 갖는다"*
/// (DESIGN §5.3)가 성립하는 조건이다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct RecoverySite {
    pub kind: RecoveryKind,
    pub span: Span,
}

impl RecoverySite {
    /// 이 자리가 삼킨 바이트 수. **`MISSING` 은 0 이다.**
    #[must_use]
    pub const fn width(&self) -> usize {
        self.span.byte_end.saturating_sub(self.span.byte_start)
    }

    /// `byte` 가 이 자리 **안**인가 — 끝은 포함하지 않는다.
    #[must_use]
    pub const fn contains_byte(&self, byte: usize) -> bool {
        self.span.byte_start <= byte && byte < self.span.byte_end
    }
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
    /// [`exports`] 의 요약 — **[R-05] 의 무효화 전파용.**
    ///
    /// # 왜 `exports` 가 있는데 따로 두는가
    ///
    /// 무효화 전파는 *"이 파일의 **표면**이 변했는가"* 를 묻고, 그 답은 집합 비교가 아니라
    /// **한 값의 비교**여야 한다 — 의존 파일이 많을수록 그 비교가 자주 일어난다.
    ///
    /// **소유가 이 조각인 근거**는 `docs/gates/F02-3-scope.md` 에 있다. 쓰는 쪽(무효화
    /// 전파)은 **F05·F07** 이고 여기서 판정하지 않는다 — 이 조각이 지는 것은 *"값이
    /// 있고, 집합이 같으면 같고 다르면 다르다"* 까지다.
    ///
    /// [`exports`]: FileGraph::exports
    /// [R-05]: ../../../docs/plan/00-risks.md#r-05
    pub export_digest: Capable<ExportDigest>,
    /// 파일 안의 스코프 체인 — **L2a**([R-22]). 만들지 않는 추출기에서는 `NotBuilt` 다.
    ///
    /// 빈 [`ScopeChain`] 은 *"스코프가 없는 파일"* 이라는 뜻이고 그것은 어떤 파일에
    /// 대해서도 참이 아니다 — 모듈 스코프는 언제나 있다. Kotlin 추출기는 이것을 안 만들고,
    /// **안 만들었다고 적는 자리가 [`Capable`] 이다.**
    ///
    /// [R-22]: ../../../docs/plan/00-risks.md#r-22
    pub scopes: Capable<ScopeChain>,
    /// tree-sitter 가 오류 회복한 **자리들**. 비면 `parsed`, 아니면 `partial`.
    ///
    /// **소스 순서다.** 정렬하지 않는다 — 순회가 소스 순서로 내는 것이 곧 사용자가 파일을
    /// 읽는 순서이고, 그것을 바꾸면 *"첫 번째 공백"* 이 뜻을 잃는다.
    pub recovery_sites: Vec<RecoverySite>,
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
        recovery_sites: Vec<RecoverySite>,
        exports: Capable<ExportSet>,
        imports: Capable<ImportSet>,
        scopes: Capable<ScopeChain>,
    ) -> Self {
        let export_digest = match &exports {
            Capable::Present(e) => Capable::Present(ExportSet::digest(e)),
            Capable::NotBuilt { capability } => Capable::NotBuilt { capability: *capability },
        };
        Self {
            language,
            grade,
            symbols,
            contains: Vec::new(),
            exports,
            imports,
            export_digest,
            scopes,
            recovery_sites,
        }
    }

    /// 이 파일이 성하게 파싱됐는가.
    #[must_use]
    // **`const` 가 아니다** — `Vec::is_empty` 는 1.87 부터 const 이고 MSRV 는 1.85 다
    // (stack §7 의 *"최신 stable − 2"*). 붙이면 `clippy::incompatible_msrv` 가 잡는다.
    pub fn is_whole(&self) -> bool {
        self.recovery_sites.is_empty()
    }

    /// 회복 지점의 **수**. 대장이 싣는 값이 이것이다.
    ///
    /// **대장은 자리를 싣지 않는다.** 자리는 파일 하나의 사실이고 대장은 저장소 하나의
    /// 표다 — 997 줄에 span 을 실으면 대장이 읽히지 않는다. 자리를 보는 창은
    /// `pal symbols --graph` 다. 그 판단의 근거는 `docs/gates/F02-2-partial.md`.
    #[must_use]
    pub fn recovery_count(&self) -> usize {
        self.recovery_sites.len()
    }

    /// 회복 자리가 삼킨 바이트의 합 ÷ 소스 길이 — **백분율, 내림.**
    ///
    /// 자리들은 서로 겹치지 않는다(순회가 `ERROR` 안쪽으로 내려가지 않는다). 그래서
    /// 단순 합이 곧 덮인 넓이다 — 겹치면 이 값이 100 을 넘고, 그것이 곧 세는 단위가
    /// 무너졌다는 신호다.
    ///
    /// 빈 소스는 0 이다. **`MISSING` 은 너비가 0 이라 이 비율을 올리지 않는다** —
    /// 세미콜론 하나가 빠진 파일이 `Unsupported` 로 강등되면 그것이 거짓말이다.
    #[must_use]
    pub fn error_ratio_percent(&self, source_len: usize) -> usize {
        if source_len == 0 {
            return 0;
        }
        let covered: usize = self.recovery_sites.iter().map(RecoverySite::width).sum();
        covered.saturating_mul(100) / source_len
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
            identity: crate::IdentityGrade::Ordinal,
        }
    }

    fn 안만듦() -> Capable<ExportSet> {
        Capable::not_built(crate::CapabilityId::new("F02", "kotlin-exports"))
    }

    fn 자리(byte_start: usize, byte_end: usize) -> RecoverySite {
        RecoverySite {
            kind: RecoveryKind::Error,
            span: Span { byte_start, byte_end, line_start: 1, line_end: 1 },
        }
    }

    fn 평평한(symbols: Vec<Symbol>, recovery_sites: Vec<RecoverySite>) -> FileGraph {
        FileGraph::flat(
            LanguageId::new("Kotlin"),
            ExtractGrade::L1,
            symbols,
            recovery_sites,
            안만듦(),
            Capable::not_built(crate::CapabilityId::new("F02", "kotlin-imports")),
            Capable::not_built(crate::CapabilityId::new("F02", "kotlin-scopes")),
        )
    }

    #[test]
    fn 최상위만_뽑은_그래프는_포함_관계가_없다() {
        // **비어 있는 것이 정확한 값이다** — 담긴 심볼을 안 뽑았으므로 담는 관계도 없다.
        let g = 평평한(vec![심볼("A")], vec![]);
        assert!(g.contains.is_empty());
        assert_eq!(g.parent_of(LocalIx(0)), None);
    }

    #[test]
    fn 안_만든_export_는_빈_집합이_아니라_notbuilt_다() {
        // **거짓 안전이 죽는 자리다.** 빈 `ExportSet` 은 *"아무것도 안 내보낸다"* 이고
        // Kotlin 최상위 선언에 대해 그것은 거짓이다(기본이 public).
        let g = 평평한(vec![심볼("A")], vec![]);
        assert!(!g.exports.is_present(), "안 만든 능력이 값으로 위장했다");
        assert!(!g.imports.is_present());
    }

    #[test]
    fn 회복_지점이_없으면_성한_파일이다() {
        assert!(평평한(vec![], vec![]).is_whole());
        assert!(!평평한(vec![], vec![자리(0, 1), 자리(4, 6)]).is_whole());
        assert_eq!(평평한(vec![], vec![자리(0, 1), 자리(4, 6)]).recovery_count(), 2);
    }

    #[test]
    fn 비율은_삼킨_넓이지_자리의_수가_아니다() {
        // **자리 열 개보다 넓은 자리 하나가 더 나쁘다.** 수로 재면 그 둘이 뒤집힌다.
        let 넓은 = 평평한(vec![], vec![자리(0, 40)]);
        let 좁은_여럿 = 평평한(vec![], (0..10).map(|i| 자리(i * 2, i * 2 + 1)).collect());
        assert_eq!(넓은.error_ratio_percent(100), 40);
        assert_eq!(좁은_여럿.error_ratio_percent(100), 10);
        assert!(좁은_여럿.recovery_count() > 넓은.recovery_count());
    }

    #[test]
    fn missing_은_비율을_올리지_않는다() {
        // 너비가 0 이다. 세미콜론 하나가 빠진 파일이 강등되면 그것이 거짓말이다.
        let g = 평평한(
            vec![],
            vec![RecoverySite {
                kind: RecoveryKind::Missing,
                span: Span { byte_start: 7, byte_end: 7, line_start: 1, line_end: 1 },
            }],
        );
        assert_eq!(g.error_ratio_percent(100), 0);
        assert!(!g.is_whole(), "너비가 0 이라고 회복이 없었던 것은 아니다");
    }

    #[test]
    fn 빈_소스는_비율이_0_이다() {
        assert_eq!(평평한(vec![], vec![]).error_ratio_percent(0), 0);
    }

    #[test]
    fn 포함_관계는_부모를_가리킨다() {
        // 이름을 붙여 가른 이유가 이 시험이다 — 벌거벗은 쌍이면 뒤바뀌어도 통과한다.
        let mut g = 평평한(vec![심볼("C"), 심볼("m")], vec![]);
        g.contains.push(Containment { parent: LocalIx(0), child: LocalIx(1) });
        assert_eq!(g.parent_of(LocalIx(1)), Some(LocalIx(0)));
        assert_eq!(g.parent_of(LocalIx(0)), None, "부모와 자식이 뒤바뀌었다");
    }
}
