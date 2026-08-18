//! 2층에 사는 것 중 심볼이 아닌 것 — **파일 노드와 참조 엣지.**
//!
//! [`crate::SymbolNode`] 는 `touch` 가 먼저 필요로 해서 그쪽에 있다. 여기 있는 둘은
//! **F05 의 1패스 스티칭이 처음 만드는 것**이다(옛 F05 §4).
//!
//! # 저장 기술이 여기 없다
//!
//! 이 크레이트는 `redb` 를 모른다(stack §4.1). 여기 있는 것은 *"2층에 무엇이 사는가"*
//! 이고 *"어떻게 담기는가"* 는 `pal-store` 다.
//!
//! # 왜 파일이 노드인가
//!
//! `schema/graph.toml` 이 적어 두었다 — *"`File` 노드 자체는 아직 없다. 그것을 만드는
//! 것은 F05 이고 지금 코드에 `FileNode` 가 없다. **없는 것을 미리 선언하지 않는다.**"*
//! 여기가 그 만기다.

use serde::{Deserialize, Serialize};

use crate::capable::Capable;
use crate::coord::{ExportDigest, SymbolId};
use crate::ledger::{ExtractGrade, LanguageId};
use crate::repo::{RepoPath, Snapshot};
use crate::scope::{BoundSymbol, RefResolution, ScopeChain};
use crate::slot::{ShellMismatch, Slot};
use crate::symbol::Symbol;
use crate::touch::SymbolNode;

/// 파일 하나의 참조가 **다섯 갈래 중 어디로 갔는가** (`[f05.2.pass]` ①).
///
/// [`crate::ScopeChain::refs`] 의 각 항목이 정확히 하나로 간다:
///
/// | 갈래 | 어디로 |
/// |---|---|
/// | `Bound` + **참조 자리 = 선언 자리** | [`declarations`] — **참조가 아니라 선언이다** |
/// | `Bound` + 바인딩이 심볼 + 담는 심볼이 있다 | **엣지** — [`edges`] |
/// | `Bound` + 바인딩이 `NotASymbol` | 아무 데도 — 지역 변수·파라미터다([`locals`]) |
/// | `Bound` + 담는 심볼이 없다 (최상위) | [`top_level`] — **엣지가 아니다** |
/// | `OutsideFile` | [`unresolved`] — import · 전역. **실패가 아니다** |
/// | `BeforeDeclaration` | [`before_declaration`] — TDZ |
///
/// # ⚠ 갈래가 다섯이 아니라 **여섯**이다 — 실물이 그렇게 말했다
///
/// `ScopeChain.refs` 는 **선언 자리의 이름도 참조로 싣는다.** `export function helper()`
/// 의 `helper`(바이트 16)가 `Bound{모듈 스코프, helper}` 로 들어 있다. 거르지 않으면
/// **모든 선언이 자기를 가리키는 엣지를 하나씩 낳고**, `pal touch helper` 가
/// *"부르는 것 1건"* 이라고 답한다 — 자기 자신이다.
///
/// 가르는 값은 [`crate::ScopeBinding::declared_at`] 이다. 참조 자리가 그것과 **정확히
/// 같으면** 그 참조는 선언이다. `from == to` 로 거르면 안 된다 — **재귀 호출**
/// (`function f() { return f() }`)이 함께 사라지고, 그것은 진짜 엣지다.
///
/// **뭉개면 미해소 수가 무엇을 세는지 알 수 없다.** 특히 TDZ 를 미해소에 넣으면
/// *"파일 밖에 있다"*(정상)와 *"언어의 오류"*가 한 숫자가 된다.
///
/// 다섯의 합이 `refs` 의 길이와 같아야 한다 — [`Self::total`] 이 그것이다.
///
/// [`declarations`]: RefCounts::declarations
/// [`edges`]: RefCounts::edges
/// [`locals`]: RefCounts::locals
/// [`top_level`]: RefCounts::top_level
/// [`unresolved`]: RefCounts::unresolved
/// [`before_declaration`]: RefCounts::before_declaration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct RefCounts {
    /// **선언 자리 그 자체.** 참조가 아니다 — 거르지 않으면 모든 선언이 자기 엣지를 낳는다.
    pub declarations: usize,
    /// 파일 **내** 엣지가 된 참조.
    pub edges: usize,
    /// 지역 변수·파라미터를 가리킨 참조 — 심볼이 아니라 엣지가 아니다.
    pub locals: usize,
    /// 어느 심볼 안에도 없는 최상위 참조 — 출발점이 없어 엣지가 아니다.
    pub top_level: usize,
    /// 파일 **밖**을 가리킨 참조 — F07 이 풀 것.
    pub unresolved: usize,
    /// 선언 전 참조(TDZ) — 엣지도 미해소도 아니다.
    pub before_declaration: usize,
}

impl RefCounts {
    /// 여섯의 합. **`refs` 의 길이와 같아야 한다** — 다르면 갈래 하나가 샜다.
    #[must_use]
    pub const fn total(&self) -> usize {
        self.declarations
            + self.edges
            + self.locals
            + self.top_level
            + self.unresolved
            + self.before_declaration
    }
}

/// 2층에 사는 파일 하나.
///
/// **[graph-node] `File`** — `schema/graph.toml`
///
/// # 두 자리가 왜 [`Capable`] 인가
///
/// Kotlin 추출기는 스코프 체인도 export 도 안 만든다(`FileGraph` 의 자리가 `NotBuilt`).
/// 그러므로 그 파일들에서 참조가 안 나오는 것은 **사실이 아니라 능력의 부재**다 —
/// [ADR-0002](../../../docs/adr/0002-empty-population-is-not-zero-violations.md) 의
/// *"모집단이 없으면 0 이 아니다"* 가 그대로 걸리는 자리이고, `0` 으로 적으면
/// *"참조가 없는 파일"* 과 *"참조를 안 보는 빌드"* 가 같은 출력이 된다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FileNode {
    pub path: RepoPath,
    pub language: LanguageId,
    /// **이 추출기가 실제로 도달한 등급.** 선언 상한이 아니다.
    pub grade: ExtractGrade,
    /// 이 파일의 표면 요약 — [R-05] 무효화 전파의 입력.
    ///
    /// **F04 가 1층 캐시에 실었고 쓰는 쪽이 없었다.** 여기가 그 소비자다. 쓰는 것은
    /// F07 이지만 **2층에 안 담기면 F07 이 다시 파싱해야 한다.**
    ///
    /// [R-05]: ../../../docs/plan/00-risks.md#r-05
    pub export_digest: Capable<ExportDigest>,
    /// 참조 다섯 갈래의 건수.
    pub refs: Capable<RefCounts>,
}

/// [`FileNode`] 의 **저장되는 형태** — 능력의 정체를 담지 않는다([`Slot`]).
///
/// `CachedGraph` 가 1층에서 하는 일을 2층에서 한다. 되읽을 때 이 빌드의 껍데기에서
/// 정체를 씌우고, **어긋나면 오류다.**
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileRow {
    pub path: RepoPath,
    pub language: LanguageId,
    pub grade: ExtractGrade,
    pub export_digest: Slot<ExportDigest>,
    pub refs: Slot<RefCounts>,
}

impl FileRow {
    /// 노드를 저장 형태로. **능력의 정체를 버린다.**
    #[must_use]
    pub fn of(node: FileNode) -> Self {
        Self {
            path: node.path,
            language: node.language,
            grade: node.grade,
            export_digest: Slot::of(node.export_digest),
            refs: Slot::of(node.refs),
        }
    }

    /// 이 빌드의 능력을 씌워 노드로 되돌린다.
    ///
    /// # Errors
    /// 저장의 자리와 이 빌드의 능력이 어긋나면 — **키가 샜다는 뜻이다.**
    pub fn restore(
        self,
        export_shell: &Capable<()>,
        scope_shell: &Capable<()>,
    ) -> Result<FileNode, ShellMismatch> {
        Ok(FileNode {
            path: self.path,
            language: self.language,
            grade: self.grade,
            export_digest: self.export_digest.restore(export_shell, "export_digest")?,
            refs: self.refs.restore(scope_shell, "refs")?,
        })
    }
}

/// 파일 **안**에서 해소된 참조 엣지 하나 — L2a 의 산물.
///
/// # 왜 `CALLS` 가 아닌가
///
/// [`crate::LocalRef`] 는 *"이 이름이 여기서 쓰였다"* 만 안다. **부르는 자리인지 타입
/// 주석인지 기록하지 않는다** — 그것을 알려면 참조를 만든 구문 노드의 종류가 필요하고
/// 그 값은 스코프 체인에 없다. `CALLS` 로 적으면 타입 참조가 호출로 둔갑한다.
/// **모르는 것을 안다고 하지 않는다** — 가르는 것은 추출기가 그 값을 실을 때다.
///
/// # 왜 등급이 `scoped` 로 고정인가
///
/// 이 엣지는 **스코프 해소로 후보가 유일할 때만** 선다
/// ([`crate::ResolutionGrade::Scoped`] 의 정의 그대로). 후보가 여럿인 경우가 이 층에
/// 없으므로 등급을 실을 자리가 필요 없다 — `BOUND_TO` 가 `exact` 하나뿐인 것과 같은 형태다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReferenceEdge {
    /// 참조가 **일어난** 심볼. 참조 바이트를 담는 가장 안쪽 심볼이다.
    pub from: SymbolId,
    /// 참조가 **가리키는** 심볼.
    ///
    /// **[graph-edge] `REFERENCES`** — `schema/graph.toml`
    ///
    /// 표식이 필드에 붙는 이유는 `xtask` 의 규약이다 — 엣지는 *"그것을 싣고 있는 자리"*
    /// 에 표시된다(`Binding::target` 과 같은 형태). **이 타입은 노드가 아니라 엣지의
    /// 행 자체이므로 자기가 자기 운반자다.**
    pub to: SymbolId,
    /// 공통 넷의 넷째 — 이 엣지가 선 스냅샷.
    pub at: Snapshot,
}

/// 파일 하나의 스코프 체인에서 **파일 내 엣지와 다섯 갈래의 건수**를 뽑는다 — 1패스의 심장.
///
/// # 무엇을 받는가
///
/// `symbols` 는 추출기의 소스 순서 목록이고 `nodes` 는 **같은 순서로** 좌표가 붙은 것이다
/// ([`crate::LocalIx`] 가 그 둘의 자리를 잇는다). 둘의 길이가 다르면 자리가 어긋난
/// 것이고, **그때는 엣지를 하나도 내지 않는다** — 틀린 엣지가 없는 엣지보다 나쁘다(C2).
///
/// # 출발점을 어떻게 찾는가 — **`LocalRef` 에 그 값이 없다**
///
/// [`crate::LocalRef`] 의 필드는 `name`·`namespace`·`at`·`resolved` 넷이고 *"어느 스코프에서
/// 일어났나"* 가 없다. 그래서 참조 바이트를 담는 **가장 안쪽 심볼**을 span 으로 찾는다.
/// 어느 심볼에도 안 담기면 최상위 참조이고 **엣지가 아니다** — 그 건수가 산출에 실린다.
///
/// # 같은 쌍이 여러 번 나올 수 있다
///
/// `a` 가 `b` 를 세 번 부르면 참조 셋이고 엣지 쌍은 하나다. [`RefCounts::edges`] 는
/// **참조**를 세고 저장은 **쌍**을 담는다 — 둘이 다른 것이 정상이다.
#[must_use]
pub fn file_edges(
    symbols: &[Symbol],
    nodes: &[SymbolNode],
    scopes: &ScopeChain,
    at: &Snapshot,
) -> (Vec<ReferenceEdge>, RefCounts) {
    let mut counts = RefCounts::default();
    if symbols.len() != nodes.len() {
        return (Vec::new(), counts);
    }

    let mut edges = Vec::new();
    for r in &scopes.refs {
        match r.resolved {
            RefResolution::OutsideFile => counts.unresolved += 1,
            RefResolution::BeforeDeclaration => counts.before_declaration += 1,
            RefResolution::Bound { scope, binding } => {
                let bound = scopes
                    .scopes
                    .get(scope.0 as usize)
                    .and_then(|s| s.bindings.get(binding as usize));
                // **선언 자리 그 자체는 참조가 아니다.** 거르지 않으면 모든 선언이
                // 자기를 가리키는 엣지를 낳는다 — 그리고 `from == to` 로 거르면
                // **재귀 호출**이 함께 사라진다.
                if bound.is_some_and(|b| b.declared_at == r.at) {
                    counts.declarations += 1;
                    continue;
                }
                let Some(BoundSymbol::Symbol(ix)) = bound.map(|b| b.symbol) else {
                    // 지역 변수·파라미터. **심볼이 아니므로 엣지가 아니다.**
                    counts.locals += 1;
                    continue;
                };
                let Some(to) = nodes.get(ix.0 as usize).map(|n| n.id) else {
                    counts.locals += 1;
                    continue;
                };
                let Some(from) = innermost(symbols, nodes, r.at) else {
                    // 어느 심볼 안에도 없다 — 출발점이 없어 엣지가 안 된다.
                    counts.top_level += 1;
                    continue;
                };
                counts.edges += 1;
                edges.push(ReferenceEdge { from, to, at: at.clone() });
            }
        }
    }
    (edges, counts)
}

/// `byte` 를 담는 **가장 안쪽** 심볼.
///
/// 가장 좁은 span 이 가장 안쪽이다. 겹치는 심볼이 없으면 그것이 유일한 답이고, 겹치면
/// (클래스 안의 메서드) 좁은 쪽이 옳다 — 메서드 안의 참조를 클래스가 한 것으로 적으면
/// 엣지가 한 층 위로 올라간다.
fn innermost(symbols: &[Symbol], nodes: &[SymbolNode], byte: usize) -> Option<SymbolId> {
    let mut best: Option<(usize, SymbolId)> = None;
    for (i, s) in symbols.iter().enumerate() {
        if s.span.byte_start > byte || byte >= s.span.byte_end {
            continue;
        }
        let width = s.span.byte_end - s.span.byte_start;
        if best.is_none_or(|(w, _)| width < w) {
            best = Some((width, nodes[i].id));
        }
    }
    best.map(|(_, id)| id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capable::CapabilityId;
    use crate::coord::BodyDigest;
    use crate::symbol::{Span, SymbolKind};

    fn 안만듦() -> Capable<()> {
        Capable::not_built(CapabilityId::new("F02", "kotlin-scopes"))
    }

    #[test]
    fn 안_만든_추출기의_참조_수는_0_이_아니라_notbuilt_다() {
        // **ADR-0002 가 걸리는 자리다.** `0` 으로 적으면 *"참조가 없는 파일"* 과
        // *"참조를 안 보는 빌드"* 가 같은 출력이 된다.
        let n = FileNode {
            path: RepoPath::new("A.kt"),
            language: LanguageId::new("Kotlin"),
            grade: ExtractGrade::L1,
            export_digest: Capable::not_built(CapabilityId::new("F02", "kotlin-exports")),
            refs: Capable::not_built(CapabilityId::new("F02", "kotlin-scopes")),
        };
        let json = serde_json::to_value(&n).unwrap();
        assert_eq!(json["refs"]["not_built"]["capability"]["feature"], "F02");
        assert!(!n.refs.is_present());
    }

    #[test]
    fn 저장_왕복이_항등이다() {
        let n = FileNode {
            path: RepoPath::new("a.ts"),
            language: LanguageId::new("TypeScript"),
            grade: ExtractGrade::L2,
            export_digest: Capable::Present(ExportDigest::from_bytes([3; 32])),
            refs: Capable::Present(RefCounts { edges: 2, ..RefCounts::default() }),
        };
        let 있음: Capable<()> = Capable::Present(());
        let 되읽음 = FileRow::of(n.clone()).restore(&있음, &있음).expect("되씌우기");
        assert_eq!(되읽음, n);
    }

    #[test]
    fn 자리가_어긋나면_조용히_넘기지_않는다() {
        // ★ 능력 축의 음성 대조 — 다른 능력을 가진 빌드가 쓴 행을 되읽으면 오류다.
        let row = FileRow::of(FileNode {
            path: RepoPath::new("a.ts"),
            language: LanguageId::new("TypeScript"),
            grade: ExtractGrade::L2,
            export_digest: Capable::Present(ExportDigest::from_bytes([3; 32])),
            refs: Capable::Present(RefCounts::default()),
        });
        let err = row.restore(&안만듦(), &안만듦()).expect_err("어긋남을 안 냈다");
        assert_eq!(err.slot, "export_digest");
        assert!(err.cached_built, "저장이 안 만든 자리를 만들었다고 적었다");
    }

    // ── `file_edges` — **다섯 갈래가 전부 1 건 이상 나오는 소스**로 잰다.
    //
    // `[f05.2.pass]` ①: *"다섯 갈래 중 하나만 있어도 「엣지가 생겼다」는 참이다.
    // 하나라도 0 이면 그 갈래는 시험되지 않은 것이다."*

    fn 심볼(name: &str, kind: SymbolKind, from: usize, to: usize) -> Symbol {
        Symbol {
            name: name.to_owned(),
            kind,
            span: Span { byte_start: from, byte_end: to, line_start: 1, line_end: 1 },
            body: BodyDigest::of_normalized(name.as_bytes()),
            identity: crate::IdentityGrade::Exact,
        }
    }

    fn 스냅샷() -> Snapshot {
        Snapshot::single(
            crate::RepoId::new("r"),
            crate::TreeRef::Committed(crate::ObjectName::from_bytes([1; 20])),
        )
    }

    fn 좌표들(symbols: &[Symbol]) -> Vec<SymbolNode> {
        symbols
            .iter()
            .enumerate()
            .map(|(i, s)| SymbolNode {
                id: crate::SymbolId::compute(
                    &crate::RepoId::new("r"),
                    &RepoPath::new("a.ts"),
                    &[],
                    &s.name,
                    &crate::Discriminator::new(s.kind, u32::try_from(i).unwrap()),
                ),
                path: RepoPath::new("a.ts"),
                container: Vec::new(),
                name: s.name.clone(),
                kind: s.kind,
                body: s.body,
                span: s.span,
                identity: s.identity,
            })
            .collect()
    }

    fn 참조(name: &str, at: usize, resolved: RefResolution) -> crate::LocalRef {
        crate::LocalRef {
            name: name.to_owned(),
            namespace: crate::Namespace::Value,
            at,
            resolved,
        }
    }

    /// `f`(0..50) 와 `g`(50..100) 두 함수. 스코프 하나에 `g`(심볼) 와 `x`(지역) 를 선언한다.
    fn 판() -> (Vec<Symbol>, Vec<SymbolNode>, ScopeChain) {
        let symbols = vec![
            심볼("f", SymbolKind::Function, 0, 50),
            심볼("g", SymbolKind::Function, 50, 100),
        ];
        let nodes = 좌표들(&symbols);
        let mut chain = ScopeChain::new();
        chain.declare(
            crate::ScopeIx(0),
            crate::ScopeBinding {
                name: "g".to_owned(),
                namespace: crate::Namespace::Value,
                declared_at: 50,
                hoisted: true,
                symbol: BoundSymbol::Symbol(crate::LocalIx(1)),
            },
        );
        chain.declare(
            crate::ScopeIx(0),
            crate::ScopeBinding {
                name: "x".to_owned(),
                namespace: crate::Namespace::Value,
                declared_at: 1,
                hoisted: false,
                symbol: BoundSymbol::NotASymbol,
            },
        );
        (symbols, nodes, chain)
    }

    #[test]
    fn 여섯_갈래가_전부_갈린다() {
        let (symbols, nodes, mut chain) = 판();
        // ① 엣지 — `f` 안(10)에서 심볼 `g` 를 부른다
        chain.refs.push(참조("g", 10, RefResolution::Bound { scope: crate::ScopeIx(0), binding: 0 }));
        // ② 지역 — `f` 안에서 지역 변수 `x`
        chain.refs.push(참조("x", 20, RefResolution::Bound { scope: crate::ScopeIx(0), binding: 1 }));
        // ③ 최상위 — 어느 심볼 span 에도 안 담기는 자리(200)에서 `g`
        chain.refs.push(참조("g", 200, RefResolution::Bound { scope: crate::ScopeIx(0), binding: 0 }));
        // ④ 파일 밖
        chain.refs.push(참조("console", 30, RefResolution::OutsideFile));
        // ⑤ TDZ
        chain.refs.push(참조("later", 40, RefResolution::BeforeDeclaration));

        // ⑥ 선언 자리 — `g` 의 선언 자리(50)에서 `g` 를 가리킨다
        chain.refs.push(참조("g", 50, RefResolution::Bound { scope: crate::ScopeIx(0), binding: 0 }));

        let (edges, c) = file_edges(&symbols, &nodes, &chain, &스냅샷());

        assert_eq!(c.declarations, 1, "선언 자리를 참조로 셌다");
        assert_eq!(c.edges, 1, "엣지가 된 참조");
        assert_eq!(c.locals, 1, "지역 변수 참조");
        assert_eq!(c.top_level, 1, "최상위 참조");
        assert_eq!(c.unresolved, 1, "파일 밖 참조");
        assert_eq!(c.before_declaration, 1, "TDZ");
        // **다섯의 합이 참조 수와 같다** — 갈래 하나가 새면 여기서 걸린다.
        assert_eq!(c.total(), chain.refs.len());

        assert_eq!(edges.len(), 1);
        assert_eq!(edges[0].from, nodes[0].id, "출발점이 `f` 가 아니다");
        assert_eq!(edges[0].to, nodes[1].id, "도착점이 `g` 가 아니다");
    }

    #[test]
    fn 겹치는_심볼에서는_가장_안쪽이_출발점이다() {
        // 클래스(0..100) 안의 메서드(10..40). 메서드 안의 참조를 클래스가 한 것으로 적으면
        // 엣지가 한 층 위로 올라간다.
        let symbols = vec![
            심볼("C", SymbolKind::Class, 0, 100),
            심볼("m", SymbolKind::Method, 10, 40),
            심볼("g", SymbolKind::Function, 100, 150),
        ];
        let nodes = 좌표들(&symbols);
        let mut chain = ScopeChain::new();
        chain.declare(
            crate::ScopeIx(0),
            crate::ScopeBinding {
                name: "g".to_owned(),
                namespace: crate::Namespace::Value,
                declared_at: 100,
                hoisted: true,
                symbol: BoundSymbol::Symbol(crate::LocalIx(2)),
            },
        );
        chain.refs.push(참조("g", 20, RefResolution::Bound { scope: crate::ScopeIx(0), binding: 0 }));
        let (edges, _) = file_edges(&symbols, &nodes, &chain, &스냅샷());
        assert_eq!(edges[0].from, nodes[1].id, "메서드가 아니라 클래스가 출발점이 됐다");
    }

    #[test]
    fn 자리가_어긋나면_엣지를_하나도_안_낸다() {
        // **틀린 엣지가 없는 엣지보다 나쁘다**(C2). 길이가 다르면 `LocalIx` 가 가리키는
        // 자리가 다른 심볼이고, 그것은 조용한 오답이다.
        let (symbols, _, chain) = 판();
        let (edges, c) = file_edges(&symbols, &[], &chain, &스냅샷());
        assert!(edges.is_empty());
        assert_eq!(c.total(), 0);
    }

    #[test]
    fn 같은_쌍이_여러_번이면_참조는_여럿이고_쌍은_하나다() {
        let (symbols, nodes, mut chain) = 판();
        for at in [10, 12, 14] {
            chain.refs.push(참조("g", at, RefResolution::Bound { scope: crate::ScopeIx(0), binding: 0 }));
        }
        let (edges, c) = file_edges(&symbols, &nodes, &chain, &스냅샷());
        assert_eq!(c.edges, 3, "참조를 셋으로 안 셌다");
        assert_eq!(edges.len(), 3);
        let 쌍: std::collections::BTreeSet<(_, _)> =
            edges.iter().map(|e| (e.from, e.to)).collect();
        assert_eq!(쌍.len(), 1, "쌍이 하나가 아니다");
    }

    #[test]
    fn 여섯_갈래의_합이_전체다() {
        // 갈래 하나가 새면 이 합이 안 맞는다 — `[f05.2.pass]` ① 이 이것을 쓴다.
        let c = RefCounts {
            declarations: 6,
            edges: 1,
            locals: 2,
            top_level: 3,
            unresolved: 4,
            before_declaration: 5,
        };
        assert_eq!(c.total(), 21);
    }

    #[test]
    fn 재귀는_선언_거르기에_안_걸린다() {
        // ★ **`from == to` 로 걸렀으면 이것이 사라진다.** 재귀 호출은 진짜 엣지다.
        let symbols = vec![심볼("f", SymbolKind::Function, 0, 50)];
        let nodes = 좌표들(&symbols);
        let mut chain = ScopeChain::new();
        chain.declare(
            crate::ScopeIx(0),
            crate::ScopeBinding {
                name: "f".to_owned(),
                namespace: crate::Namespace::Value,
                declared_at: 9,
                hoisted: true,
                symbol: BoundSymbol::Symbol(crate::LocalIx(0)),
            },
        );
        // 선언 자리(9)와 재귀 호출(30).
        chain.refs.push(참조("f", 9, RefResolution::Bound { scope: crate::ScopeIx(0), binding: 0 }));
        chain.refs.push(참조("f", 30, RefResolution::Bound { scope: crate::ScopeIx(0), binding: 0 }));
        let (edges, c) = file_edges(&symbols, &nodes, &chain, &스냅샷());
        assert_eq!(c.declarations, 1);
        assert_eq!(c.edges, 1, "재귀 호출이 사라졌다");
        assert_eq!(edges[0].from, edges[0].to, "재귀 엣지가 자기를 안 가리킨다");
    }
}
