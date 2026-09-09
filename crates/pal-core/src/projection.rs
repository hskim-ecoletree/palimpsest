//! 2층에 있는 것 중 심볼이 아닌 것 — **파일 노드와 참조 엣지.**
//!
//! [`crate::SymbolNode`] 는 `touch` 가 먼저 필요로 해서 그쪽에 있다. 여기 있는 둘은
//! **F05 의 1패스 스티칭이 처음 만드는 것**이다(옛 F05 §4).
//!
//! # 저장 기술이 여기 없다
//!
//! 이 크레이트는 `redb` 를 모른다(stack §4.1). 여기 있는 것은 *"2층에 무엇이 있는가"*
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
use crate::file_graph::ImportedItem;
use crate::scope::{BoundSymbol, RefResolution, ScopeBinding, ScopeChain, ScopeIx, ScopeParent};
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
/// | `Ambiguous` | [`ambiguous`] — 후보가 둘 이상이라 **안 골랐다** |
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
/// 갈래의 합이 `refs` 의 길이와 같아야 한다 — [`Self::total`] 이 그것이다.
///
/// [`declarations`]: RefCounts::declarations
/// [`edges`]: RefCounts::edges
/// [`locals`]: RefCounts::locals
/// [`top_level`]: RefCounts::top_level
/// [`unresolved`]: RefCounts::unresolved
/// [`before_declaration`]: RefCounts::before_declaration
/// [`ambiguous`]: RefCounts::ambiguous
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
    /// 같은 스코프에 후보가 둘 이상이라 **안 고른 참조** — `cfg` 쌍둥이가 그 자리다.
    ///
    /// ⚠ **이 칸이 붙으면서 2층 행의 모양이 바뀌었다**(2026-09-08 · #130). 저장은
    /// postcard 라 자리 기반이고, 그래서 **옛 색인을 새 바이너리로 읽으면 디코드에서
    /// 죽는다.** 구제는 색인을 지우고 다시 세우는 것 하나다:
    ///
    /// ```text
    /// rm -rf .palimpsest/index.redb .palimpsest/cache
    /// pal query graph.dump --json > /dev/null
    /// ```
    ///
    /// 그 절차를 여기 적어 두지 않으면 화면이 원인 불명 오류만 내고 사람이 되짚을
    /// 자리가 없다.
    ///
    /// **미해소와 갈라 둔다.** 미해소는 F07 이 풀 수 있고 이것은 원리상 못 푼다 —
    /// 추출기가 `cfg` 를 해석하지 않기로 했기 때문이다.
    pub ambiguous: usize,
    /// `use` 로 들여온 이름을 가리킨 참조 — **파일 밖 심볼을 가리키므로 파일 안 엣지가
    /// 아니고, 지역 변수도 아니다.**
    ///
    /// # 왜 [`Self::locals`] 에서 갈랐나 (2026-09-09)
    ///
    /// 이 갈래가 생기기 전 임포트 참조는 전부 `locals` 로 샜다. Rust 추출기가 `use` 로
    /// 들어온 이름을 **모듈 스코프의 바인딩**으로 묶는데 그 바인딩은
    /// [`crate::BoundSymbol::NotASymbol`] 이라, `file_edges` 가 지역 변수와 같은 칸에
    /// 세었기 때문이다. 그래서 *"이 파일이 밖에서 이름을 몇 개나 쓰나"* 를 재는 자가
    /// 없었고, [`Self::unresolved`] 는 `use` 를 안 거친 이름만 담았다.
    ///
    /// **이 칸은 「아직 안 풀린 것」이 아니라 「2 층이 풀 것」이다.** 파일 하나만 보는
    /// 연산에서 대상 심볼은 원리상 안 보이고, 짝을 지어 줄 정보(어느 모듈의 어느
    /// 이름인가)는 [`crate::ImportSet::items`] 에 있다. 그 둘을 잇는 자리가 2 층이고,
    /// 이 갈래에 든 참조는 [`PendingImportRef`] 로 함께 나간다.
    ///
    /// ⚠ **[`Self::locals`] 로 되돌리면 파일 간 엣지의 분모가 사라진다** — 그러면
    /// *"몇 건을 풀었나"* 를 셀 모집단이 없다.
    pub imported: usize,
}

impl RefCounts {
    /// 일곱의 합. **`refs` 의 길이와 같아야 한다** — 다르면 갈래 하나가 샜다.
    #[must_use]
    pub const fn total(&self) -> usize {
        self.declarations
            + self.edges
            + self.locals
            + self.top_level
            + self.unresolved
            + self.before_declaration
            + self.ambiguous
            + self.imported
    }
}

/// 파일 하나 안에서 **풀리지 않고 2 층으로 넘어가는** 임포트 참조 하나.
///
/// # 왜 이름이 아니라 셋을 싣나
///
/// 2 층이 이 참조를 풀려면 *"어느 심볼이"*(`from`) *"무슨 이름을"*(`local`) 가리켰는지가
/// 둘 다 있어야 한다. `local` 하나만 실으면 엣지의 출발점이 없고, `from` 만 실으면
/// 무엇을 찾을지 모른다. `at` 은 같은 이름을 여러 번 부른 참조를 가르는 자리다 —
/// 없으면 세 번 부른 것이 한 건으로 뭉개진다.
///
/// ⚠ **`local` 은 이 파일이 부르는 이름이지 대상 모듈의 이름이 아니다.**
/// `use a::B as C;` 에서 여기 실리는 것은 `C` 이고, `B` 로 되돌리는 것은
/// [`crate::ImportedItem`] 이 진다. 그래서 이 타입은 모듈도 원본 이름도 안 싣는다 —
/// 두 곳에 적으면 별칭이 있는 자리에서 갈린다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PendingImportRef {
    /// 참조를 한 심볼 — 엣지의 출발점.
    pub from: SymbolId,
    /// 이 파일이 그 항목을 부르는 이름. [`crate::ImportedItem::local`] 과 짝짓는다.
    pub local: String,
    /// 참조가 일어난 바이트.
    pub at: usize,
}

/// [`file_edges`] 가 파일 하나에서 산출하는 것 셋.
///
/// 튜플이 아니라 이름을 붙인 까닭 하나 — 셋째가 붙으면서 `(edges, counts)` 의 자리
/// 기억이 안 통하게 됐고, 자리로 받는 코드는 셋째를 조용히 빠뜨린다.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FileRefs {
    /// 파일 **안**에서 해소된 엣지.
    pub edges: Vec<ReferenceEdge>,
    /// 갈래별 건수. 합이 `refs` 의 길이와 같아야 한다.
    pub counts: RefCounts,
    /// 2 층이 풀 임포트 참조. [`RefCounts::imported`] 와 **길이가 같아야 한다**.
    pub pending: Vec<PendingImportRef>,
}

/// 2층에 있는 파일 하나.
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
/// 이 엣지는 **스코프 해소로 후보가 유일할 때만** 성립한다
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
///
/// # `imports` 를 왜 받나
///
/// `use` 로 들여온 이름은 **모듈 스코프의 바인딩**이고 그 바인딩은
/// [`BoundSymbol::NotASymbol`] 이다. 그래서 그것을 가리킨 참조는 아래에서 지역 변수와
/// 같은 갈래로 떨어진다 — 이 인자가 없으면 [`RefCounts::locals`] 로 샌다.
///
/// **빈 슬라이스를 주면 오늘까지의 동작 그대로다.** 항목 축을 안 만드는 언어와, 이
/// 함수를 스코프 산출만 재려고 부르는 자리가 그것을 쓴다.
#[must_use]
pub fn file_edges(
    symbols: &[Symbol],
    nodes: &[SymbolNode],
    scopes: &ScopeChain,
    imports: &[ImportedItem],
    at: &Snapshot,
) -> FileRefs {
    let mut counts = RefCounts::default();
    if symbols.len() != nodes.len() {
        return FileRefs::default();
    }

    let mut pending = Vec::new();
    let mut edges = Vec::new();
    for r in &scopes.refs {
        match r.resolved {
            RefResolution::OutsideFile => counts.unresolved += 1,
            RefResolution::BeforeDeclaration => counts.before_declaration += 1,
            RefResolution::Ambiguous => counts.ambiguous += 1,
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
                    // **임포트가 여기로 온다.** `use` 로 들여온 이름은 모듈 스코프에
                    // 묶이고 심볼이 아니라, 이 갈래에서 지역 변수와 섞인다.
                    // 갈라 두지 않으면 파일 간 엣지의 분모가 사라진다.
                    //
                    // ⚠ **이름만으로 안 가른다** — 같은 이름의 지역 변수가 있으면
                    // 그것도 잡힌다. 바인딩이 **모듈 스코프**의 것이어야 한다.
                    if 임포트된_이름인가(scopes, scope, bound, imports) {
                        counts.imported += 1;
                        if let Some(from) = innermost(symbols, nodes, r.at) {
                            pending.push(PendingImportRef {
                                from,
                                local: r.name.clone(),
                                at: r.at,
                            });
                        } else {
                            // 어느 심볼 안에도 없는 임포트 참조 — 출발점이 없어 2 층도
                            // 엣지를 못 만든다. **건수는 `imported` 에 남기고 짝은 안
                            // 만든다** — 그래야 `pending` 의 길이와 이 칸이 갈린 이유가
                            // 하나뿐이 되고, 아래 불변식이 그것을 잰다.
                        }
                        continue;
                    }
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
    FileRefs { edges, counts, pending }
}

/// 이 바인딩이 `use` 로 들여온 이름인가.
///
/// # 둘을 함께 봐야 한다
///
/// **① 모듈 스코프의 바인딩인가** — `use` 는 파일 꼭대기에서만 이름을 묶는다
/// ([`ScopeParent::Root`]). 함수 안의 지역 변수는 여기서 걸러진다.
/// **② 그 이름이 [`ImportSet::items`] 에 있나** — 모듈 스코프에는 `use` 말고도
/// [`BoundSymbol::NotASymbol`] 인 것이 올 수 있다(타입 파라미터·매크로).
///
/// 하나만 보면 어느 쪽이든 샌다 — ① 만 보면 모듈 스코프의 다른 비-심볼이 임포트로
/// 세어지고, ② 만 보면 임포트와 같은 이름의 지역 변수가 임포트로 세어진다.
///
/// ⚠ **`items` 가 [`Slot::NotBuilt`] 인 언어는 빈 슬라이스를 받는다** — 그러면 이
/// 함수가 언제나 거짓이고 오늘까지의 동작 그대로다. *"항목 축을 안 만든다"* 와
/// *"임포트가 0 건이다"* 를 부르는 쪽이 갈라서 넘긴다.
fn 임포트된_이름인가(
    scopes: &ScopeChain,
    scope: ScopeIx,
    bound: Option<&ScopeBinding>,
    imports: &[ImportedItem],
) -> bool {
    if imports.is_empty() {
        return false;
    }
    let Some(binding) = bound else { return false };
    let Some(s) = scopes.scopes.get(scope.0 as usize) else { return false };
    if s.parent != ScopeParent::Root {
        return false;
    }
    imports.iter().any(|item| item.local == binding.name)
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
        let err = row.restore(&안만듦(), &안만듦()).expect_err("어긋남을 안 산출했다");
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
                visible_from: 50,
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
                visible_from: 1,
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

        let FileRefs { edges, counts: c, .. } =
            file_edges(&symbols, &nodes, &chain, &[], &스냅샷());

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
                visible_from: 100,
                hoisted: true,
                symbol: BoundSymbol::Symbol(crate::LocalIx(2)),
            },
        );
        chain.refs.push(참조("g", 20, RefResolution::Bound { scope: crate::ScopeIx(0), binding: 0 }));
        let edges = file_edges(&symbols, &nodes, &chain, &[], &스냅샷()).edges;
        assert_eq!(edges[0].from, nodes[1].id, "메서드가 아니라 클래스가 출발점이 됐다");
    }

    #[test]
    fn 자리가_어긋나면_엣지를_하나도_안_산출한다() {
        // **틀린 엣지가 없는 엣지보다 나쁘다**(C2). 길이가 다르면 `LocalIx` 가 가리키는
        // 자리가 다른 심볼이고, 그것은 조용한 오답이다.
        let (symbols, _, chain) = 판();
        let FileRefs { edges, counts: c, .. } = file_edges(&symbols, &[], &chain, &[], &스냅샷());
        assert!(edges.is_empty());
        assert_eq!(c.total(), 0);
    }

    #[test]
    fn 같은_쌍이_여러_번이면_참조는_여럿이고_쌍은_하나다() {
        let (symbols, nodes, mut chain) = 판();
        for at in [10, 12, 14] {
            chain.refs.push(참조("g", at, RefResolution::Bound { scope: crate::ScopeIx(0), binding: 0 }));
        }
        let FileRefs { edges, counts: c, .. } =
            file_edges(&symbols, &nodes, &chain, &[], &스냅샷());
        assert_eq!(c.edges, 3, "참조를 셋으로 안 셌다");
        assert_eq!(edges.len(), 3);
        let 쌍: std::collections::BTreeSet<(_, _)> =
            edges.iter().map(|e| (e.from, e.to)).collect();
        assert_eq!(쌍.len(), 1, "쌍이 하나가 아니다");
    }

    #[test]
    fn 여덟_갈래의_합이_전체다() {
        // 갈래 하나가 새면 이 합이 안 맞는다 — `[f05.2.pass]` ① 이 이것을 쓴다.
        //
        // ⚠ **갈래가 늘면 이 시험이 먼저 빨개져야 한다.** 값을 서로 다르게 두는 까닭도
        // 그것이다 — 같은 값을 쓰면 두 칸이 뒤바뀌어도 합이 안 움직인다.
        let c = RefCounts {
            declarations: 6,
            edges: 1,
            locals: 2,
            top_level: 3,
            unresolved: 4,
            before_declaration: 5,
            ambiguous: 7,
            imported: 8,
        };
        assert_eq!(c.total(), 36);
    }

    #[test]
    fn 모호한_참조는_엣지가_아니다() {
        // `cfg` 쌍둥이 — 둘째 선언 자리가 첫째를 가리키는 엣지가 되던 자리다(실측 13 건).
        let symbols = vec![
            심볼("f", SymbolKind::Function, 0, 50),
            심볼("t", SymbolKind::Function, 60, 90),
        ];
        let nodes = 좌표들(&symbols);
        let mut chain = ScopeChain::new();
        chain.refs.push(참조("t", 70, RefResolution::Ambiguous));
        let FileRefs { edges, counts: c, .. } =
            file_edges(&symbols, &nodes, &chain, &[], &스냅샷());
        assert!(edges.is_empty(), "모호한 참조가 엣지가 됐다");
        assert_eq!(c.ambiguous, 1);
        assert_eq!(c.total(), chain.refs.len());
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
                visible_from: 9,
                hoisted: true,
                symbol: BoundSymbol::Symbol(crate::LocalIx(0)),
            },
        );
        // 선언 자리(9)와 재귀 호출(30).
        chain.refs.push(참조("f", 9, RefResolution::Bound { scope: crate::ScopeIx(0), binding: 0 }));
        chain.refs.push(참조("f", 30, RefResolution::Bound { scope: crate::ScopeIx(0), binding: 0 }));
        let FileRefs { edges, counts: c, .. } =
            file_edges(&symbols, &nodes, &chain, &[], &스냅샷());
        assert_eq!(c.declarations, 1);
        assert_eq!(c.edges, 1, "재귀 호출이 사라졌다");
        assert_eq!(edges[0].from, edges[0].to, "재귀 엣지가 자기를 안 가리킨다");
    }
}
