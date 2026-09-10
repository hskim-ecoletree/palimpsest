//! 파일 경계를 넘는 참조 해소 — **F07 의 2 층 몫**.
//!
//! # 1 층이 못 하는 일이라 여기 있다
//!
//! [`file_edges`](crate::file_edges) 는 **파일 하나만** 본다. `use` 로 들어온 이름은
//! 그 파일 안에 정의가 없으므로 원리상 못 풀고, 그래서 그것을
//! [`PendingImportRef`] 로 내보낸다. 이 모듈이 그 짝을 받아 **다른 파일의 심볼**에
//! 잇는다.
//!
//! # 무엇을 잇고 무엇을 안 잇나 — 축2 의 두 갈래
//!
//! | [`PendingImportRef::tail`] | 찾는 것 | 갈래 |
//! |---|---|---|
//! | [`None`] | 임포트 항목 자체 — [`ImportedItem::name`] | **ⓐ** |
//! | [`Some`] | **대상 파일에서만** 그 꼬리 이름 | **ⓑ** |
//!
//! ⓑ 가 이 모듈의 요점이다. `S::foo()` 의 `foo` 는 **참조가 아니다** — 참조로 세면 같은
//! 파일의 동명 선언에 붙어 조용한 오답이 된다(실측 31 건). 꼬리는 머리를 푼 **뒤에**
//! 그 파일에서만 찾는다. 그것이 `A7` 갈래 (다2)이고, `경로_꼬리_배제` 를 안 끄는 까닭이다.
//!
//! # 안 따라가는 것
//!
//! **`pub use` 재수출.** 대상 파일의 **심볼**만 보므로 재수출은 자동으로 안 잡힌다 —
//! 재수출은 정의가 아니라 이름이고 [`SymbolNode`] 가 아니다. 잠근 축1 의 「밖」 칸이
//! 그것이고, `A5` 가 그 음성 대조다.
//!
//! # 후보가 유일하지 않으면 **엣지가 아니다**
//!
//! 모듈 경로가 여러 파일로 읽히거나 대상 파일에 같은 이름의 심볼이 둘이면
//! [`UnresolvedReason::Ambiguous`] 로 나간다. 하나를 고르면 그것이 조용한 오답이고,
//! `## 원문` 이 잠근 *"exact 만"* 이 정확히 이 자리다.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::{ImportedItem, PendingImportRef, RepoPath, Snapshot, SymbolId, SymbolNode};

/// 파일 **경계를 넘은** 참조 엣지 하나.
///
/// # 왜 [`crate::ReferenceEdge`] 와 다른 타입인가 (`C3`)
///
/// **등급이 다르다.** `REFERENCES` 는 `grade = "scoped"` 로 고정돼 있다 — 스코프
/// 체인이 후보를 하나로 좁혔다는 뜻이다. 파일 간 엣지는 그 자로 서지 않는다:
/// 임포트 경로를 파일로 펴고 그 파일 안에서 이름이 유일할 때 서므로 **`exact`** 다.
///
/// 같은 라벨에 두 등급을 실을 수는 없다. 등급 칸을 새로 만드는 것은 이 회차의 범위
/// 밖이고(`## 범위 밖` 이 등급 축을 `#133` 에 주었다), 고정값을 그대로 둔 채 파일 간
/// 엣지를 밀어 넣으면 **파생 문서가 거짓을 산출한다.** 남는 길이 하나뿐이라 그것을
/// 골랐다 — `C3` 이 그 사실을 문면으로 적고 `## 차선책` 이 같은 답을 등록해 두었다.
///
/// ⚠ **2 층의 저장 자리는 같다.** `EDGE_OUT`/`EDGE_IN` 은 라벨을 안 담고 `(출발, 도착)`
/// 쌍만 담는다. 라벨이 갈리는 자리는 **스키마와 그래프 뷰**이고, 뷰는 양 끝 심볼의
/// 파일이 다른가로 두 라벨을 가른다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrossFileEdge {
    /// 참조가 **일어난** 심볼 — 엣지의 출발점.
    pub from: SymbolId,
    /// 참조가 **가리키는** 다른 파일의 심볼.
    ///
    /// **[graph-edge] `REFERENCES_ACROSS`** — `schema/graph.toml`
    ///
    /// 표식이 필드에 붙는 것은 `xtask` 의 규약이다 — 엣지는 *"그것을 싣고 있는 자리"*
    /// 에 표시된다. **이 타입은 노드가 아니라 엣지의 행 자체이므로 자기가 자기
    /// 운반자다.**
    pub to: SymbolId,
    /// 공통 넷의 넷째 — 이 엣지가 선 스냅샷.
    pub at: Snapshot,
}

/// 파일 하나가 이 패스에 내놓는 것.
#[derive(Debug, Clone)]
pub struct CrossFileInput {
    pub path: RepoPath,
    /// 이 파일이 들여온 항목. **`Slot::NotBuilt` 인 언어는 빈 슬라이스를 준다** —
    /// 부르는 쪽이 *"항목 축을 안 만든다"* 와 *"임포트가 0 건"* 을 갈라서 넘긴다.
    pub imports: Vec<ImportedItem>,
    /// 1 층이 못 푼 임포트 참조.
    pub pending: Vec<PendingImportRef>,
}

/// 짝을 못 지은 까닭. **수만 세면 무엇이 막혔는지 모른다.**
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnresolvedReason {
    /// 참조한 이름이 이 파일의 임포트 항목에 없다 — 매크로가 만든 이름 따위.
    NoImport,
    /// 모듈 경로가 이 저장소 밖을 가리킨다 — `std`·외부 크레이트.
    OutsideRepo,
    /// 모듈 경로에 맞는 파일이 이 저장소에 없다.
    NoTargetFile,
    /// 대상 파일에 그 이름의 심볼이 없는데 **그 파일이 크레이트 뿌리**다.
    ///
    /// `use pal_core::Snapshot;` 뒤의 `Snapshot::single(…)` 이 그 형태다 —
    /// `crates/pal-core/src/lib.rs` 에는 `pub use` 만 있고 정의가 없다.
    /// **재수출 추적은 잠근 축1 의 「밖」**이므로 이 자리들은 설계대로 엣지가 아니다.
    ///
    /// ★ **[`NoSymbol`] 에서 갈라 둔 까닭은 가는 문이 다르기 때문이다** (2026-09-10 ·
    /// 판 4 의 이관표). 이쪽은 `A5`·`A5-a` 가 지는 **범위 밖**이고 저쪽은 `#133`(L2)의
    /// 잔여다. 한 이름으로 묶여 있으면 화면이 *"못 풀었다"* 만 말하고 **그것이 결함인지
    /// 경계인지**를 못 말한다 — 앞 판이 이 자리에 산문으로 적어 둔 비율(340 중 298)은
    /// 값이 움직이는 순간 거짓이 됐다. 그래서 산문을 **회계 열쇠로 바꾼다.**
    ///
    /// [`NoSymbol`]: Self::NoSymbol
    NoSymbolAtCrateRoot,
    /// 대상 파일에 그 이름의 심볼이 없고 **그 파일은 크레이트 뿌리가 아니다.**
    ///
    /// 꼬리가 **enum 변형**이거나 연관 상수라 그래프에 심볼이 안 선 자리가 여기 든다.
    /// 추출기가 변형 이름을 선언도 참조도 안 만들기로 이미 정했고(`rust_scopes.rs`),
    /// 소유자가 2026-09-09 에 그것을 **`#133`(L2) 잔여**로 보냈다.
    NoSymbol,
    /// 후보가 둘 이상이라 **안 골랐다.**
    Ambiguous,
}

impl UnresolvedReason {
    /// 회계의 열쇠로 쓰는 이름. **`serde` 표기와 같은 문자열이어야 한다** — 갈리면
    /// 저장된 회계와 화면이 서로 다른 이름을 쓴다.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoImport => "no_import",
            Self::OutsideRepo => "outside_repo",
            Self::NoTargetFile => "no_target_file",
            Self::NoSymbolAtCrateRoot => "no_symbol_at_crate_root",
            Self::NoSymbol => "no_symbol",
            Self::Ambiguous => "ambiguous",
        }
    }
}

/// 해소를 **실제로 지난 걸음** 하나.
///
/// # 왜 빈 배열이면 안 되나
///
/// 스키마가 `attempts` 를 `required = true` 로 적어 두고도 선행 구현은 그 자리를
/// 146 건 전부에서 비워 두었다(`schema/graph.toml` 의 `UnresolvedRef` 주석). 빈 배열은
/// *"시도를 안 했다"* 와 *"시도했는데 못 찾았다"* 를 같은 값으로 만든다 —
/// **어느 걸음에서 끊겼는지가 사라지면 「못 푼 참조」는 수일 뿐이고 고칠 자리를 안 준다.**
///
/// 그래서 이 타입은 걸음마다 **본 것**과 **그중 성립한 수**를 함께 진다. 마지막 걸음의
/// [`Self::found`] 가 0 인 자리가 끊긴 자리이고, [`UnresolvedRef::reason`] 이 그것을
/// 이름으로 말한다. 둘은 같은 사실의 두 표현이라 **어긋나면 그것이 결함이다.**
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Attempt {
    /// 어느 걸음인가.
    pub step: AttemptStep,
    /// 그 걸음이 **실제로 본 것** — 찾은 이름 · 편 모듈 경로 후보 · 대상 파일의 이름.
    pub tried: Vec<String>,
    /// 그중 성립한 수. **0 이면 여기서 끊겼다.**
    pub found: usize,
}

/// 해소가 지나는 걸음 셋. **순서가 있고, 앞이 끊기면 뒤는 안 돈다.**
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttemptStep {
    /// 참조한 지역 이름을 이 파일의 임포트 항목에서 찾는다.
    ImportItem,
    /// 임포트의 모듈 경로를 저장소의 파일로 편다.
    ModulePath,
    /// 펴진 파일 **안에서** 이름을 찾는다.
    TargetSymbol,
}

impl AttemptStep {
    /// 회계·화면이 쓰는 이름. **`serde` 표기와 같은 문자열이어야 한다.**
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ImportItem => "import_item",
            Self::ModulePath => "module_path",
            Self::TargetSymbol => "target_symbol",
        }
    }
}

/// 못 푼 참조 하나 — **F08 의 값.**
///
/// **[graph-node] `UnresolvedRef`** — `schema/graph.toml`
///
/// # 자리만 있던 것이 값이 된다
///
/// 이 타입은 2026-09-10 까지 **거주 불가한 빈 enum** 이었고 스키마가 그것을
/// `status = "not_built"` · `built_by = "F08"` 로 적었다. 그 짝은
/// *"안 만들었음"* 과 *"없음"* 이 같은 출력이 되는 것을 막는 장치였다 —
/// 자리만 두고 값을 만들 수 있으면 누군가 빈 값을 채워 넣기 때문이다.
///
/// **그 짝을 함께 뒤집는다.** 값을 만들 원천이 실제로 섰기 때문이다:
/// [`cross_file_edges`] 는 짝마다 임포트 항목 · 모듈 경로 · 대상 파일의 심볼을 순서대로
/// 훑고, 끊긴 자리와 그때까지 본 것을 이미 알고 있다. 그 앎을 버리지 않고 싣는 것이
/// 이 타입이다.
///
/// # 해소돼도 안 지운다
///
/// 스키마 주석이 그것을 못 박는다 — *"**해소돼도 지우지 않는다** — 정적 하한과 관측된
/// 해소를 겹쳐 보이는 것이 정직한 형태다."* 그러므로 이 목록은 「지금 못 푼 것」이 아니라
/// **「이 패스가 못 푼 것」**이고, 같은 자리에 엣지가 서 있을 수 있다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnresolvedRef {
    /// 참조가 **일어난** 심볼 — 열쇠의 앞 칸.
    ///
    /// **[graph-edge] `REFERS_UNRESOLVED`** — `schema/graph.toml`
    ///
    /// 표식이 필드에 붙는 것은 `xtask` 의 규약이다 — 엣지는 *"그것을 싣고 있는 자리"* 에
    /// 표시된다. `BOUND_TO` 가 `Binding::target` 에 실리는 것과 같은 자리다.
    ///
    /// ★ **엣지가 없으면 못 푼 참조는 떠 있는 행이다.** 어느 심볼이 무엇을 못 풀었는지가
    /// 그래프의 관계로 안 서면 *"여기를 바꾸면 무엇이 깨지나"* 의 역방향 색인에서
    /// **못 푼 몫이 통째로 안 보인다** — 그것이 `B4` 가 이름을 못 박은 까닭이다.
    pub site: SymbolId,
    /// 못 푼 이름 — 열쇠의 뒤 칸.
    ///
    /// ⓐ 는 임포트 항목의 **원본 이름**(별칭이 아니다), ⓑ 는 **경로 호출의 꼬리**다.
    pub name: String,
    /// 왜 못 풀었나.
    pub reason: UnresolvedReason,
    /// 실제로 지난 걸음들. **비어 있으면 형식 오류다** — [`Attempt`] 가 그 까닭을 진다.
    pub attempts: Vec<Attempt>,
    /// 공통 넷의 넷째 — 이 행이 선 스냅샷. `REFERS_UNRESOLVED` 가 이 값을 진다.
    ///
    /// ⚠ **사전 등록된 넷을 안 건드린다** — `site`·`name`·`reason`·`attempts` 는 그대로고
    /// 이 칸은 **엣지 등록이 요구하는 공통 넷**의 자리다(`B4`). 엣지를 세우려면 발생
    /// 스냅샷을 실을 자리가 있어야 하고, 없는 필드 이름을 스키마에 적는 것이 곧 거짓이다.
    pub at: Snapshot,
}

/// 이 패스의 회계. **[`crate::RefCounts`] 에 못 넣는 것을 여기서 헤아린다.**
///
/// ⚠ 그 타입은 `total() == refs.len()` 을 진다. ⓑ 는 참조 자리가 아니라 **머리 참조에
/// 실려 온 값**이라 그 합에 못 들어간다 — 넣으면 불변식이 깨진다. 그래서 갈래별 분모와
/// 산출을 이 구조가 진다.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CrossFileReport {
    /// ⓐ 의 시도 — **넘어온 짝 전부**다.
    ///
    /// ⚠ 꼬리가 있는 짝을 빼지 않는다. `Root::make()` 는 `Root` 로 가는 엣지도 낳으므로
    /// 그것도 ⓐ 의 시도이고, 빼면 그 시도가 어느 분모에도 안 든다.
    pub a_pending: usize,
    /// ⓐ 에서 엣지가 선 수.
    pub a_edges: usize,
    /// ⓑ 로 넘어온 짝 — **꼬리를 실은 것.** 이것이 ⓑ 의 분모다.
    pub b_pending: usize,
    /// ⓑ 에서 엣지가 선 수.
    pub b_edges: usize,
    /// ⓐ 가 못 선 까닭별 건수. **0 건인 까닭은 안 싣는다** — 없는 것과 안 헤아린 것을 가른다.
    pub unresolved: BTreeMap<String, usize>,
    /// ⓑ 가 못 선 까닭별 건수.
    ///
    /// ★ **ⓐ 와 갈라 두는 것이 요점이다.** 합쳐 두면 어느 갈래가 막혔는지 못 읽는다 —
    /// 실측(2026-09-09): 합계로는 `no_symbol` 이 가장 컸는데 그 대부분이 ⓐ 의 재수출
    /// 경유였고, ⓑ 가 막힌 자리는 다른 곳이었다.
    #[serde(default)]
    pub b_unresolved: BTreeMap<String, usize>,
}

impl CrossFileReport {
    /// 못 선 수 — ⓐ·ⓑ 합.
    #[must_use]
    pub fn misses(&self) -> usize {
        self.a_pending + self.b_pending - self.a_edges - self.b_edges
    }
}

/// 크레이트 하나의 뿌리 — `crates/pal-core/src`.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Crate {
    /// `use` 에 쓰이는 이름 — 디렉터리 이름의 `-` 를 `_` 로.
    name: String,
    /// `src` 까지의 접두 — `crates/pal-core/src`.
    root: String,
}

/// 저장소의 크레이트 목록을 **파일 목록에서** 뽑는다.
///
/// 매니페스트를 안 읽는 까닭 하나 — 이 패스의 입력은 이미 훑어진 파일 목록이고, 거기
/// 없는 크레이트는 어차피 대상이 못 된다. 매니페스트를 읽으면 **파일이 없는 크레이트가
/// 목록에 들어오고** 그 이름의 임포트가 `NoTargetFile` 이 아니라 조용히 사라진다.
fn crates_of(paths: &BTreeSet<&str>) -> Vec<Crate> {
    let mut out = Vec::new();
    for p in paths {
        // ★ **`main.rs` 도 크레이트 뿌리다** (2026-09-10 · 판 4 의 A7). 앞 판은
        //   `/lib.rs` 만 알아봤고, 그래서 `crates/pal-cli` 처럼 라이브러리가 없는
        //   바이너리 크레이트 안에서는 `crate::` 가 통째로 안 풀렸다(실측 20 건).
        let Some(root) = p.strip_suffix("/lib.rs").or_else(|| p.strip_suffix("/main.rs")) else {
            continue;
        };
        // `crates/pal-core/src/lib.rs` → root=`crates/pal-core/src`, dir=`pal-core`
        let Some(dir) = root.strip_suffix("/src").and_then(|d| d.rsplit('/').next()) else {
            continue;
        };
        out.push(Crate { name: dir.replace('-', "_"), root: root.to_owned() });
    }
    out.sort_by(|a, b| a.name.cmp(&b.name));
    out.dedup_by(|a, b| a.name == b.name);
    out
}

/// 이 파일이 속한 크레이트의 `src` 접두.
fn crate_root_of<'c>(path: &str, crates: &'c [Crate]) -> Option<&'c Crate> {
    crates
        .iter()
        .filter(|c| path.starts_with(&format!("{}/", c.root)))
        // 가장 긴 접두가 이긴다 — 중첩 크레이트에서 바깥이 이기면 틀린다.
        .max_by_key(|c| c.root.len())
}

/// 모듈 경로를 파일 후보로 — **찾는 것이 아니라 후보를 만드는 것이다.**
///
/// # 넷을 가른다 (잠근 축1)
///
/// | 첫 세그먼트 | 뿌리 |
/// |---|---|
/// | `crate` | 이 파일이 속한 크레이트의 `src` |
/// | `self` | 이 파일이 속한 모듈 디렉터리 |
/// | `super` (연속) | 그만큼 위 |
/// | 다른 크레이트 이름 | 그 크레이트의 `src` |
/// | 그 밖 | **이 파일과 같은 디렉터리의 형제 모듈** |
///
/// 마지막 줄이 `use inside::{Rel, Root}` 를 잡는다 — 같은 `mod` 안의 형제다.
///
/// # 돌려주는 것이 **두 층**이다
///
/// 앞이 그 모듈 자신의 파일(`a/b.rs`·`a/b/mod.rs`), 뒤가 부모 파일 안의 인라인
/// `mod`(`a.rs`·`a/mod.rs`). **앞에 맞는 것이 있으면 뒤는 안 본다.**
fn module_candidates(
    from: &str,
    module: &str,
    crates: &[Crate],
    item_inline_depth: usize,
) -> (Vec<String>, Vec<String>) {
    let mut segs: Vec<&str> = if module.is_empty() {
        Vec::new()
    } else {
        module.split("::").filter(|s| !s.is_empty()).collect()
    };

    // 형제 모듈 갈래는 **base 가 둘**이다 — 아래 `또다른` 이 그 둘째를 진다.
    let mut 또다른: Option<String> = None;
    let mut base = match segs.first().copied() {
        Some("crate") => {
            segs.remove(0);
            crate_root_or_dir(from, crates)
        }
        Some("self") => {
            segs.remove(0);
            모듈_자리(from)
        }
        Some("super") => {
            // ★★ **인라인 `mod` 의 겹만큼은 파일 안에서 소진된다** (2026-09-10 ·
            //    독립 리뷰 라운드 2). `super` 는 **쓴 자리의 모듈**을 기준으로 한 칸
            //    올라가는데, 이 함수는 모듈 자리를 **파일 경로로만** 계산한다.
            //    그래서 `mod tests` 안의 `use super::{…}` 가 파일의 **부모** 모듈로
            //    가고 — 한 칸 높다 — 거기서 꼬리가 **남의 사적 심볼**을 잡는다.
            //
            //    실측: `install/manifest.rs` 의 `mod tests` 안 `use super::{… Rel …}`
            //    가 `install.rs` 로 풀려 `Rel::new` 의 꼬리가 그 파일의 사적
            //    `Report::new` 를 잡았다 — **거짓 엣지 3 건**.
            let mut 남은겹 = item_inline_depth;
            let mut d = 모듈_자리(from);
            while segs.first().copied() == Some("super") {
                segs.remove(0);
                if 남은겹 > 0 {
                    // 이 `super` 는 **같은 파일 안**의 한 겹을 벗는다. 파일은 안 넘는다.
                    남은겹 -= 1;
                    continue;
                }
                d = d.rsplit_once('/').map_or(String::new(), |(p, _)| p.to_owned());
            }
            d
        }
        Some(first) => match crates.iter().find(|c| c.name == first) {
            Some(c) => {
                segs.remove(0);
                c.root.clone()
            }
            // ★ **크레이트가 아니면 자기 모듈 안의 형제다** (2026-09-10 · 판 4).
            //   Rust 의 균일 경로는 **현재 모듈** 기준이고, 앞 판이 쓴
            //   `module_dir_of`(형제 디렉터리)는 `a.rs` 꼴 모듈 파일에서 한 칸 어긋난다 —
            //   `crates/pal-cli/src/install.rs` 의 `use inside::{Rel, Root};` 가
            //   `src/inside.rs` 를 찾아 못 만났다. 그 줄은 `intent.md` 의 축1 「안」 표
            //   **첫째 칸에 예시로 적힌 바로 그 줄**이고, 실측 22 건이 통째로
            //   「저장소 밖」 통에 들어가 있었다.
            None => {
                // ⚠ **자리가 둘이라 둘 다 후보로 삼는다.** 형제 디렉터리 쪽을 버리면 실측으로
                //    ⓐ 가 103 건 줄었다 — `use common::{…}` 처럼 부모 디렉터리에 놓인
                //    형제가 그 자리다.
                또다른 = Some(module_dir_of(from));
                모듈_자리(from)
            }
        },
        None => {
            또다른 = Some(module_dir_of(from));
            모듈_자리(from)
        }
    };

    let 잇는다 = |mut b: String| {
        for s in &segs {
            b = if b.is_empty() { (*s).to_owned() } else { format!("{b}/{s}") };
        }
        b
    };
    let 또다른 = 또다른.map(&잇는다);
    base = 잇는다(base);
    // ★ **층이 둘이다.** 1 차는 그 모듈 자신의 파일이고, 2 차는 부모 파일 안의
    //   인라인 `mod` 다. **1 차에 맞는 파일이 있으면 2 차는 안 본다** — 둘을 한 벌로
    //   내면 `deep/leaf` 를 찾을 때 실재하는 `deep/mod.rs` 가 함께 잡혀 모호가 된다
    //   (2026-09-09 실측: `Leaf` 가 통째로 안 섰다).
    let mut 일차 = vec![format!("{base}.rs"), format!("{base}/mod.rs")];
    if segs.is_empty() {
        // ★ **뿌리는 둘이다** (2026-09-10 · `DL4-11`). `crates_of` 는 `lib.rs` 와
        //   `main.rs` 를 **둘 다** 크레이트 뿌리로 인정하는데(2026-09-10 · 판 4 의 `A7`)
        //   후보를 펴는 이 자리는 `lib.rs` 만 넣고 있었다. 그러면 라이브러리가 없는
        //   바이너리 크레이트를 이름으로 부른 임포트가 **`NoTargetFile` 통으로 조용히
        //   간다** — 인정한 자와 펴는 자가 갈리면 그 갈림은 회계에 안 나온다.
        일차.push(format!("{base}/lib.rs"));
        일차.push(format!("{base}/main.rs"));
    }
    let mut 이차 = base
        .rsplit_once('/')
        .map(|(p, _)| vec![format!("{p}.rs"), format!("{p}/mod.rs")])
        .unwrap_or_default();
    // 형제 모듈의 둘째 자리도 **1 차**다 — 같은 층의 다른 후보이지 부모 안의 인라인
    // `mod` 가 아니다.
    if let Some(b2) = 또다른 {
        일차.push(format!("{b2}.rs"));
        일차.push(format!("{b2}/mod.rs"));
        if let Some((p, _)) = b2.rsplit_once('/') {
            이차.push(format!("{p}.rs"));
            이차.push(format!("{p}/mod.rs"));
        }
    }
    (일차, 이차)
}

/// 이 파일의 **형제**가 놓이는 디렉터리.
///
/// `a/b.rs` 는 모듈 `b` 이고 그 형제는 `a/` 안에 있다. `a/mod.rs`·`lib.rs` 는 자기가
/// 디렉터리이고 그 형제도 같은 디렉터리다 — **둘이 같은 답으로 간다.**
fn module_dir_of(path: &str) -> String {
    path.rsplit_once('/').map_or(String::new(), |(d, _)| d.to_owned())
}

/// 이 파일이 **여는 모듈 자신**의 자리 — [`module_dir_of`] 와 한 칸 다르다.
///
/// # 왜 갈라야 하나
///
/// `super` 는 **자기 모듈의 부모**다. `deep/leaf.rs` 의 모듈은 `crate::deep::leaf` 이고
/// 그 부모는 `crate::deep`(= `src/deep`)인데, 형제 디렉터리로 재면 이미 `src/deep` 이라
/// 거기서 한 번 더 올라가 `src` 가 된다 — **`super` 하나가 두 칸을 간다.**
/// 실측(2026-09-09): 그래서 `super::super::inside::Root` 가 통째로 안 섰다.
///
/// | 파일 | 여는 모듈 | 이 함수 |
/// |---|---|---|
/// | `src/deep/leaf.rs` | `crate::deep::leaf` | `src/deep/leaf` |
/// | `src/deep/mod.rs` | `crate::deep` | `src/deep` |
/// | `src/lib.rs` | `crate` | `src` |
fn 모듈_자리(path: &str) -> String {
    let dir = path.rsplit_once('/').map_or("", |(d, _)| d);
    let file = path.rsplit('/').next().unwrap_or("");
    if matches!(file, "mod.rs" | "lib.rs") {
        dir.to_owned()
    } else {
        let stem = file.strip_suffix(".rs").unwrap_or(file);
        if dir.is_empty() { stem.to_owned() } else { format!("{dir}/{stem}") }
    }
}

/// 이 파일이 속한 크레이트의 `src`, 없으면 자기 디렉터리.
///
/// **크레이트를 못 찾는 것은 실패가 아니다** — 예제·시험처럼 `src/lib.rs` 밑이 아닌
/// 파일이 있고, 거기서 `crate::` 는 자기 자리에서 시작하는 것이 가장 가까운 답이다.
/// `None` 으로 끊으면 그 파일의 임포트가 통째로 `NoTargetFile` 이 된다.
fn crate_root_or_dir(from: &str, crates: &[Crate]) -> String {
    crate_root_of(from, crates).map_or_else(|| module_dir_of(from), |c| c.root.clone())
}

/// 파일 경계를 넘는 참조를 잇는다.
///
/// `symbols` 는 **저장소 전체**의 심볼이어야 한다 — 파일 하나치만 주면 이 패스가 원리상
/// 아무것도 못 잇는다.
///
/// # 후보가 유일하지 않으면 엣지가 아니다
///
/// 모듈 경로가 여러 파일로 읽히거나 대상 파일에 동명 심볼이 둘이면
/// [`UnresolvedReason::Ambiguous`] 다. **하나를 고르면 그것이 조용한 오답이다.**
///
/// # 셋째 반환값 — 못 푼 것도 산출이다 (`B1`~`B4`)
///
/// 이 함수는 짝마다 걸음 셋을 순서대로 지나고 끊긴 자리를 이미 안다. 그 앎을 회계의
/// **수**로만 남기면 *"몇 건이 막혔다"* 까지만 말하고 **어느 자리의 무엇이 막혔는지**를
/// 못 말한다 — [`UnresolvedRef`] 가 그 자리다. 회계([`CrossFileReport`])와 이 목록은
/// **같은 사실의 두 표현**이고, 까닭별 합이 서로 어긋나면 그것이 결함이다.
#[must_use]
pub fn cross_file_edges(
    files: &[CrossFileInput],
    symbols: &[SymbolNode],
    at: &Snapshot,
) -> (Vec<CrossFileEdge>, CrossFileReport, Vec<UnresolvedRef>) {
    let paths: BTreeSet<&str> = symbols.iter().map(|s| s.path.as_str()).collect();
    let crates = crates_of(&paths);
    // (파일, 이름) → 그 이름의 심볼들. 유일성 판정에 개수가 필요하고, **담은 것**이
    // 필요한 까닭은 아래 `주인` 이 진다.
    let mut by_name: BTreeMap<(&str, &str), Vec<(SymbolId, &[String])>> = BTreeMap::new();
    for s in symbols {
        by_name
            .entry((s.path.as_str(), s.name.as_str()))
            .or_default()
            .push((s.id, s.container.as_slice()));
    }

    let mut edges = Vec::new();
    let mut report = CrossFileReport::default();
    // 갈래 **하나만** 막힌 자리를 헤아린다 — 그 갈래의 통에만 들어간다.
    let miss = |r: &mut CrossFileReport, b: bool, why: UnresolvedReason| {
        let bucket = if b { &mut r.b_unresolved } else { &mut r.unresolved };
        *bucket.entry(why.as_str().to_owned()).or_default() += 1;
    };
    // ★ **공유 단계의 실패는 갈래 **둘 다**를 넘어뜨린다** (2026-09-10 · 판 4 라운드 2).
    //
    // 임포트 항목 찾기와 모듈 경로 펴기는 ⓐ·ⓑ 가 **같이** 지나는 자리다. 거기서 끊기면
    // 머리 엣지도 꼬리 엣지도 못 만든다. 그런데 앞 판은 꼬리가 있는 짝(`b`)의 공유 단계
    // 실패를 **ⓑ 통에만** 넣었고, 그래서 **ⓐ 의 까닭 합이 ⓐ 의 못 선 수보다 작았다** —
    // 실측 `5705 − 1460 = 4245` 인데 까닭의 합은 3929 였고 차 316 이 `282 + 20 + 14`
    // 로 딱 맞았다.
    //
    // ⚠ **수가 안 맞는 것보다 나쁜 것은 화면이 그것을 안 말하는 것이다.** 「못 선 까닭」
    // 줄이 갈래의 미달을 남김없이 가른다고 읽히는데 316 건이 어느 까닭에도 없었다 —
    // 그것이 거짓 신호다. **한 자리가 두 갈래를 넘어뜨리면 두 통에 다 헤아린다.**
    let miss_shared = |r: &mut CrossFileReport, b: bool, why: UnresolvedReason| {
        *r.unresolved.entry(why.as_str().to_owned()).or_default() += 1;
        if b {
            *r.b_unresolved.entry(why.as_str().to_owned()).or_default() += 1;
        }
    };
    // 「심볼이 없다」가 어느 문으로 가는지는 **대상이 크레이트 뿌리인가**로 갈린다 —
    // 뿌리면 재수출 경유(`A5`·`A5-a` · 축1 **밖**)이고 아니면 꼬리가 심볼로 안 선
    // 것(`#133` L2 잔여)이다. 문이 다르므로 회계도 갈라 헤아린다.
    //
    // ⚠ **`crates_of` 와 같은 자를 쓴다** — 그쪽이 뿌리로 인정한 파일만 뿌리다.
    // 접미사만 보면 크레이트가 아닌 디렉터리의 `lib.rs` 도 뿌리가 된다.
    let 심볼_없음 = |target: &str| {
        let 뿌리 = target
            .strip_suffix("/lib.rs")
            .or_else(|| target.strip_suffix("/main.rs"))
            .is_some_and(|root| crates.iter().any(|c| c.root == root));
        if 뿌리 { UnresolvedReason::NoSymbolAtCrateRoot } else { UnresolvedReason::NoSymbol }
    };

    let mut unresolved: Vec<UnresolvedRef> = Vec::new();

    for f in files {
        let from_path = f.path.as_str();
        for p in &f.pending {
            // ★ **모든 짝이 ⓐ 의 시도다.** 꼬리가 있는 짝도 머리 엣지를 낳으므로
            //   ⓐ 의 분모에서 빼면 그 시도가 어디에도 안 세어진다.
            let b = p.tail.is_call();
            report.a_pending += 1;
            if b {
                report.b_pending += 1;
            }

            // ★ **걸음을 지나면서 적는다 — 끊긴 뒤에 되짚어 만들지 않는다.**
            //   되짚은 것은 관측이 아니라 재구성이고, 재구성은 해소기가 실제로 무엇을
            //   봤는지를 못 말한다. `attempts` 가 요구하는 것은 **지난 것**이다.
            let mut attempts: Vec<Attempt> = Vec::new();

            let 항목 = f.imports.iter().find(|i| i.local == p.local);
            attempts.push(Attempt {
                step: AttemptStep::ImportItem,
                tried: vec![p.local.clone()],
                found: usize::from(항목.is_some()),
            });
            let Some(item) = 항목 else {
                miss_shared(&mut report, b, UnresolvedReason::NoImport);
                unresolved.push(UnresolvedRef {
                    site: p.from,
                    name: p.local.clone(),
                    reason: UnresolvedReason::NoImport,
                    attempts,
                    at: at.clone(),
                });
                continue;
            };

            // 저장소 밖인가 — 첫 세그먼트가 크레이트도 접두도 아니면서 형제 모듈로도
            // 안 읽히는 것은 아래 `NoTargetFile` 이 잡는다. 여기서 거르는 것은 **알려진
            // 밖**이다: 크레이트 목록에 없는 이름으로 시작하고 그 이름이 접두가 아닌 것.
            let head = item.module.split("::").next().unwrap_or("");
            let 접두 = matches!(head, "crate" | "self" | "super");
            let 우리것 = 접두 || crates.iter().any(|c| c.name == head) || head.is_empty();

            let (일차, 이차) =
                module_candidates(from_path, &item.module, &crates, item.inline_depth);
            let 훑는다 = |cs: &[String]| {
                let mut v: Vec<&str> =
                    cs.iter().filter_map(|c| paths.get(c.as_str()).copied()).collect();
                v.sort();
                v.dedup();
                v
            };
            let mut hit = 훑는다(&일차);
            let mut 본_후보 = 일차.clone();
            if hit.is_empty() {
                hit = 훑는다(&이차);
                본_후보.extend(이차.iter().cloned());
            }
            attempts.push(Attempt {
                step: AttemptStep::ModulePath,
                tried: 본_후보,
                found: hit.len(),
            });
            let target = match hit.as_slice() {
                [one] => *one,
                [] => {
                    let why =
                        if 우리것 { UnresolvedReason::NoTargetFile } else { UnresolvedReason::OutsideRepo };
                    miss_shared(&mut report, b, why);
                    unresolved.push(UnresolvedRef {
                        site: p.from,
                        name: item.name.clone(),
                        reason: why,
                        attempts,
                        at: at.clone(),
                    });
                    continue;
                }
                _ => {
                    miss_shared(&mut report, b, UnresolvedReason::Ambiguous);
                    unresolved.push(UnresolvedRef {
                        site: p.from,
                        name: item.name.clone(),
                        reason: UnresolvedReason::Ambiguous,
                        attempts,
                        at: at.clone(),
                    });
                    continue;
                }
            };

            // ── ⓐ **임포트한 이름 자체.** 별칭은 여기서 원본으로 돌아간다.
            //
            //    ★ **꼬리가 있어도 이 엣지는 만들어진다.** `Root::make()` 는 자리 하나이지만
            //    `Root` 를 가리키는 것도 `make` 를 가리키는 것도 참이다. 앞 판은 꼬리가
            //    머리를 **대체**했고, 그래서 `use` 로 들여온 타입이 경로 호출로만 쓰이면
            //    그 타입으로 가는 엣지가 통째로 사라졌다(2026-09-09 실측: `Root`·`Origin`).
            let a_hits = by_name.get(&(target, item.name.as_str()));
            let mut a_attempts = attempts.clone();
            a_attempts.push(Attempt {
                step: AttemptStep::TargetSymbol,
                tried: vec![format!("{target}#{}", item.name)],
                found: a_hits.map_or(0, Vec::len),
            });
            match a_hits.map(Vec::as_slice) {
                Some([(one, _)]) => {
                    edges.push(CrossFileEdge { from: p.from, to: *one, at: at.clone() });
                    report.a_edges += 1;
                }
                Some(_) => {
                    miss(&mut report, false, UnresolvedReason::Ambiguous);
                    unresolved.push(UnresolvedRef {
                        site: p.from,
                        name: item.name.clone(),
                        reason: UnresolvedReason::Ambiguous,
                        attempts: a_attempts,
                        at: at.clone(),
                    });
                }
                None => {
                    let why = 심볼_없음(target);
                    miss(&mut report, false, why);
                    unresolved.push(UnresolvedRef {
                        site: p.from,
                        name: item.name.clone(),
                        reason: why,
                        attempts: a_attempts,
                        at: at.clone(),
                    });
                }
            }

            // ── ⓑ **경로 호출의 꼬리.** 없으면 여기서 끝이다.
            let Some(want) = p.tail.name() else { continue };
            let b_hits = by_name.get(&(target, want));
            let mut b_attempts = attempts;
            b_attempts.push(Attempt {
                step: AttemptStep::TargetSymbol,
                tried: vec![format!("{target}#{want}")],
                found: b_hits.map_or(0, Vec::len),
            });
            let Some(hits) = b_hits else {
                let why = 심볼_없음(target);
                miss(&mut report, b, why);
                unresolved.push(UnresolvedRef {
                    site: p.from,
                    name: want.to_owned(),
                    reason: why,
                    attempts: b_attempts,
                    at: at.clone(),
                });
                continue;
            };
            // ★ **ⓑ 에서는 머리의 이름이 곧 담은 것이다** — `S::foo()` 의 `foo` 는
            //   `impl S` 안에 있다. 그러므로 대상 파일에 `foo` 가 여럿이어도
            //   담은 것이 `S` 인 하나가 있으면 그것이 답이고 모호가 아니다.
            //
            //   ⚠ **찾는 자리는 여전히 대상 파일 안이다** — 잠근 문면이 *"대상 파일에서만
            //   찾는다"* 이고, 저장소 전체에서 `impl S` 를 뒤지는 것은 그 밖이다.
            //   그래서 이 좁힘은 확대가 아니라 **모호를 가르는 자**다.
            let 주인: Vec<(SymbolId, &[String])> = hits
                .iter()
                .filter(|(_, c)| c.last().is_some_and(|last| last == &item.name))
                .copied()
                .collect();
            let 고른 = if 주인.len() == 1 { 주인.as_slice() } else { hits.as_slice() };
            match 고른 {
                [(one, _)] => {
                    edges.push(CrossFileEdge { from: p.from, to: *one, at: at.clone() });
                    report.b_edges += 1;
                }
                _ => {
                    miss(&mut report, b, UnresolvedReason::Ambiguous);
                    unresolved.push(UnresolvedRef {
                        site: p.from,
                        name: want.to_owned(),
                        reason: UnresolvedReason::Ambiguous,
                        attempts: b_attempts,
                        at: at.clone(),
                    });
                }
            }
        }
    }
    // ★★ **키가 키여야 한다** (2026-09-10 · 독립 리뷰 라운드 2).
    //
    // 스키마가 `key = ["site", "name"]` 으로 적는데 산출에 그 짝이 **여러 번** 있었다 —
    // 같은 자리에서 같은 이름이 여러 번 참조되면 그때마다 한 행이 났기 때문이다.
    // 실측: 행 4417 중 서로 다른 짝이 2883 · 중복 키 814 · 한 짝이 최대 **27** 번.
    //
    // ⚠ **바깥 도구에 실리면 엣지가 증식한다** — `pal export` 가 같은 키로 노드를 여러
    // 번 만들고, 이어지는 `MATCH (a:UnresolvedRef {site, name})` 이 **그 전부에 매치**한다.
    // 어떤 검사도 이 유일성을 안 재고 있었다: 불변식 ② 는 필수 속성만 보고 `xtask` 의
    // 스키마 정합은 이름과 형만 본다.
    //
    // **첫 것을 남긴다.** 같은 짝의 걸음은 같은 자리를 지나므로 첫 것이 그 자리를 진다.
    let mut 본것: BTreeSet<(SymbolId, String)> = BTreeSet::new();
    unresolved.retain(|u| 본것.insert((u.site, u.name.clone())));

    (edges, report, unresolved)
}
