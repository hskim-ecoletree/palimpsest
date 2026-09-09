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
//! [`Unresolved::Ambiguous`] 로 나간다. 하나를 고르면 그것이 조용한 오답이고,
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
pub enum Unresolved {
    /// 참조한 이름이 이 파일의 임포트 항목에 없다 — 매크로가 만든 이름 따위.
    NoImport,
    /// 모듈 경로가 이 저장소 밖을 가리킨다 — `std`·외부 크레이트.
    OutsideRepo,
    /// 모듈 경로에 맞는 파일이 이 저장소에 없다.
    NoTargetFile,
    /// 대상 파일에 그 이름의 심볼이 없다.
    ///
    /// # 이 갈래는 대부분 **결함이 아니라 경계다** (2026-09-09 실측)
    ///
    /// ⓑ 의 340 건을 뜯어보니 **298 건(87.6%)이 크레이트 루트 `lib.rs`** 를 대상으로
    /// 한다 — `use pal_core::Snapshot;` 뒤의 `Snapshot::single(…)` 이 그 형태다. 그
    /// 파일에는 `pub use` 만 있고 정의가 없다. **재수출 추적은 잠근 축1 의 「밖」**
    /// 이므로 이 자리들은 설계대로 안 서는 것이다(`A5`·`A5-a`).
    ///
    /// 남은 42 건의 상당수는 꼬리가 **enum 변형**이다(`Present` 34 · `Committed` 15 ·
    /// `At` 11). 추출기가 변형 이름을 선언도 참조도 안 만들기로 이미 정했고, 소유자가
    /// 2026-09-09 에 그것을 `#133`(L2) 잔여로 보냈다.
    NoSymbol,
    /// 후보가 둘 이상이라 **안 골랐다.**
    Ambiguous,
}

impl Unresolved {
    /// 회계의 열쇠로 쓰는 이름. **`serde` 표기와 같은 문자열이어야 한다** — 갈리면
    /// 저장된 회계와 화면이 서로 다른 이름을 쓴다.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoImport => "no_import",
            Self::OutsideRepo => "outside_repo",
            Self::NoTargetFile => "no_target_file",
            Self::NoSymbol => "no_symbol",
            Self::Ambiguous => "ambiguous",
        }
    }
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
        let Some(root) = p.strip_suffix("/lib.rs") else { continue };
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
fn module_candidates(from: &str, module: &str, crates: &[Crate]) -> (Vec<String>, Vec<String>) {
    let mut segs: Vec<&str> = if module.is_empty() {
        Vec::new()
    } else {
        module.split("::").filter(|s| !s.is_empty()).collect()
    };

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
            let mut d = 모듈_자리(from);
            while segs.first().copied() == Some("super") {
                segs.remove(0);
                d = d.rsplit_once('/').map_or(String::new(), |(p, _)| p.to_owned());
            }
            d
        }
        Some(first) => match crates.iter().find(|c| c.name == first) {
            Some(c) => {
                segs.remove(0);
                c.root.clone()
            }
            // 크레이트가 아니면 형제 모듈이다 — 세그먼트를 안 걷어낸다.
            None => module_dir_of(from),
        },
        None => module_dir_of(from),
    };

    for s in &segs {
        base = if base.is_empty() { (*s).to_owned() } else { format!("{base}/{s}") };
    }
    // ★ **층이 둘이다.** 1 차는 그 모듈 자신의 파일이고, 2 차는 부모 파일 안의
    //   인라인 `mod` 다. **1 차에 맞는 파일이 있으면 2 차는 안 본다** — 둘을 한 벌로
    //   내면 `deep/leaf` 를 찾을 때 실재하는 `deep/mod.rs` 가 함께 잡혀 모호가 된다
    //   (2026-09-09 실측: `Leaf` 가 통째로 안 섰다).
    let mut 일차 = vec![format!("{base}.rs"), format!("{base}/mod.rs")];
    if segs.is_empty() {
        일차.push(format!("{base}/lib.rs"));
    }
    let 이차 = base
        .rsplit_once('/')
        .map(|(p, _)| vec![format!("{p}.rs"), format!("{p}/mod.rs")])
        .unwrap_or_default();
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
/// [`Unresolved::Ambiguous`] 다. **하나를 고르면 그것이 조용한 오답이다.**
#[must_use]
pub fn cross_file_edges(
    files: &[CrossFileInput],
    symbols: &[SymbolNode],
    at: &Snapshot,
) -> (Vec<CrossFileEdge>, CrossFileReport) {
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
    let miss = |r: &mut CrossFileReport, b: bool, why: Unresolved| {
        let bucket = if b { &mut r.b_unresolved } else { &mut r.unresolved };
        *bucket.entry(why.as_str().to_owned()).or_default() += 1;
    };

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

            let Some(item) = f.imports.iter().find(|i| i.local == p.local) else {
                miss(&mut report, b, Unresolved::NoImport);
                continue;
            };

            // 저장소 밖인가 — 첫 세그먼트가 크레이트도 접두도 아니면서 형제 모듈로도
            // 안 읽히는 것은 아래 `NoTargetFile` 이 잡는다. 여기서 거르는 것은 **알려진
            // 밖**이다: 크레이트 목록에 없는 이름으로 시작하고 그 이름이 접두가 아닌 것.
            let head = item.module.split("::").next().unwrap_or("");
            let 접두 = matches!(head, "crate" | "self" | "super");
            let 우리것 = 접두 || crates.iter().any(|c| c.name == head) || head.is_empty();

            let (일차, 이차) = module_candidates(from_path, &item.module, &crates);
            let 훑는다 = |cs: &[String]| {
                let mut v: Vec<&str> =
                    cs.iter().filter_map(|c| paths.get(c.as_str()).copied()).collect();
                v.sort();
                v.dedup();
                v
            };
            let mut hit = 훑는다(&일차);
            if hit.is_empty() {
                hit = 훑는다(&이차);
            }
            let target = match hit.as_slice() {
                [one] => *one,
                [] => {
                    miss(
                        &mut report,
                        b,
                        if 우리것 { Unresolved::NoTargetFile } else { Unresolved::OutsideRepo },
                    );
                    continue;
                }
                _ => {
                    miss(&mut report, b, Unresolved::Ambiguous);
                    continue;
                }
            };

            // ── ⓐ **임포트한 이름 자체.** 별칭은 여기서 원본으로 돌아간다.
            //
            //    ★ **꼬리가 있어도 이 엣지는 만들어진다.** `Root::make()` 는 자리 하나이지만
            //    `Root` 를 가리키는 것도 `make` 를 가리키는 것도 참이다. 앞 판은 꼬리가
            //    머리를 **대체**했고, 그래서 `use` 로 들여온 타입이 경로 호출로만 쓰이면
            //    그 타입으로 가는 엣지가 통째로 사라졌다(2026-09-09 실측: `Root`·`Origin`).
            match by_name.get(&(target, item.name.as_str())).map(Vec::as_slice) {
                Some([(one, _)]) => {
                    edges.push(CrossFileEdge { from: p.from, to: *one, at: at.clone() });
                    report.a_edges += 1;
                }
                Some(_) => miss(&mut report, false, Unresolved::Ambiguous),
                None => miss(&mut report, false, Unresolved::NoSymbol),
            }

            // ── ⓑ **경로 호출의 꼬리.** 없으면 여기서 끝이다.
            let Some(want) = p.tail.name() else { continue };
            let Some(hits) = by_name.get(&(target, want)) else {
                miss(&mut report, b, Unresolved::NoSymbol);
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
                _ => miss(&mut report, b, Unresolved::Ambiguous),
            }
        }
    }
    (edges, report)
}
