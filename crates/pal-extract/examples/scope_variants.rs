//! **변형 대조** — 이 회차의 완수 증인 (`V1`~`V12`).
//!
//! 규칙을 하나씩 끈 판으로 저장소를 통째로 돌려 **엣지 집합이 달라지는지**를 잰다.
//! 안 달라지면 그 규칙은 아무것도 안 재고 있다.
//!
//! # 왜 수가 아니라 집합인가
//!
//! 사전부검 R2 가 옛 증인(「기여한 서로 다른 Rust 파일 ≥ 80」)을 **변형 13 개에서 전부
//! 126** 으로 관측했다. 수는 상수라 어떤 고장도 못 가른다. 그래서 증인이 수에서 변형
//! 대조로 옮겨졌다(소유자 답 2026-09-08).
//!
//! ⚠ **이 도구에는 단위시험이 없다.** `cargo xtask test` 는 `--all-targets` 라 예제를
//! 빌드만 하고 안 돌린다(`coord_collisions` 와 같은 자리 · #81). 그래서 자기 검산을
//! 안에 넣었다 — 스스로 센 엣지 수와 `pal_core::file_edges` 가 센 수가 갈리면 **`✗ 검산`
//! 을 찍고 종료 코드 2 로 죽는다.**
//!
//! 사용:
//!
//! - `cargo run -q -p pal-extract --example scope_variants -- <루트>` — 변형 대조(`V1`~`V12`)
//! - `cargo run -q -p pal-extract --example scope_variants -- <루트> --표본` — `E1` 의 층화 표본 50 건
//!
//! # 표본 규칙은 **뽑기 전에 정해져 있다** (`E1`)
//!
//! 잠긴 의도가 적은 것: *"기여 파일에서 파일별 층화로 뽑고, 갈래(타입 참조 / 호출·매크로 /
//! 그 밖)마다 최소 10 건을 채운다"* · 총 50 건. 이 코드가 고정하는 배분은
//! **타입 15 · 호출·매크로 20 · 그 밖 15** 이고, 갈래마다 `(경로, 바이트)` 로 정렬한 뒤
//! **균등 간격**으로 집는다 — 그래야 파일이 골고루 들어가고 다시 돌려도 같은 50 건이 나온다.
//! 난수를 안 쓰는 까닭: 표본을 다시 뽑아 유리한 것을 고르는 길을 막는다.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use pal_core::{
    BoundSymbol, Capable, Discriminator, ObjectName, RefResolution, RepoId, RepoPath, Snapshot,
    Symbol, SymbolId, SymbolNode, TreeRef,
};
use pal_extract::RustScopeRules;

/// 엣지 하나의 신원 — (파일, 출발 심볼 자리, 도착 심볼 자리).
///
/// 심볼 목록은 **변형과 무관하게 같다**(스코프만 갈린다). 그래서 자리로 대면
/// 좌표를 계산하지 않고도 집합이 대조된다.
type 엣지 = (String, u32, u32);

/// 엣지가 무엇이었나 — `V11` 이 이 분포를 게이트에 적는다.
#[derive(Default, Clone, Copy)]
struct 갈래 {
    타입: usize,
    호출: usize,
    그밖: usize,
}

fn 훑는다(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(rd) = std::fs::read_dir(dir) else { return };
    let mut 것들: Vec<PathBuf> = rd.flatten().map(|e| e.path()).collect();
    것들.sort();
    for p in 것들 {
        let name = p.file_name().and_then(|s| s.to_str()).unwrap_or("");
        if name.starts_with('.') || name == "target" || name == "node_modules" {
            continue;
        }
        if p.is_dir() {
            훑는다(&p, out);
        } else if p.extension().and_then(|s| s.to_str()) == Some("rs") {
            out.push(p);
        }
    }
}

/// 이 참조가 **호출·매크로**인가.
fn 호출인가(root: tree_sitter::Node<'_>, at: usize) -> bool {
    let Some(mut n) = root.named_descendant_for_byte_range(at, at) else { return false };
    loop {
        let Some(p) = n.parent() else { return false };
        match p.kind() {
            "call_expression" => {
                if p.child_by_field_name("function").is_some_and(|f| f.id() == n.id()) {
                    return true;
                }
                return false;
            }
            "macro_invocation" => {
                return p.child_by_field_name("macro").is_some_and(|f| f.id() == n.id());
            }
            // 매크로 토큰 열 안에서는 구조가 없다 — 뒤에 괄호 열이 오면 호출이다.
            "token_tree" => {
                return n.next_sibling().is_some_and(|s| s.kind() == "token_tree");
            }
            "scoped_identifier" | "generic_function" | "field_expression" => n = p,
            _ => return false,
        }
    }
}

/// 참조 일곱 갈래의 저장소 합 — **엣지 집합이 안 갈리는 규칙도 여기서는 갈릴 수 있다.**
///
/// 두 축을 따로 재는 까닭: 어떤 규칙은 엣지를 안 만들면서 **화면에 나가는 수**만
/// 움직인다 — `RefCounts::unresolved` 가 `pal query` 의 응답 묶음에서 **「범위 미해소」**
/// 로 화면에 닿는다(`pal-query/src/lib.rs` → `pal-cli/src/query.rs`).
///
/// ⚠ **`pal touch` 의 「내가 모르는 것」 칸은 이 값이 아니다**(정반합 판 2 의 정(正)이
/// 잡았다). 그 칸은 `pal-core/src/touch.rs` 의 `Capable<Vec<UnresolvedRef>>` 이고
/// `UnresolvedRef` 는 거주 불가 enum · F08 미구축이라 지금 *"능력이 없습니다"* 를 찍는다.
/// 두 자리를 같은 것으로 적으면 그것이 「모르는 것을 안다고 적는」 형태다.
/// 한 축만 재면 그 규칙이 아무것도 안 하는 것처럼 보인다.
#[derive(Default, Clone, Copy, PartialEq, Eq)]
struct 갈래합 {
    declarations: usize,
    edges: usize,
    locals: usize,
    top_level: usize,
    unresolved: usize,
    before_declaration: usize,
    ambiguous: usize,
}

impl 갈래합 {
    fn 더한다(&mut self, c: &pal_core::RefCounts) {
        self.declarations += c.declarations;
        self.edges += c.edges;
        self.locals += c.locals;
        self.top_level += c.top_level;
        self.unresolved += c.unresolved;
        self.before_declaration += c.before_declaration;
        self.ambiguous += c.ambiguous;
    }

    fn 줄(&self) -> String {
        format!(
            "선언 {} · 엣지 {} · 지역 {} · 최상위 {} · 미해소 {} · 선언전 {} · 모호 {}",
            self.declarations,
            self.edges,
            self.locals,
            self.top_level,
            self.unresolved,
            self.before_declaration,
            self.ambiguous
        )
    }
}

/// 한 판(변형 하나)이 저장소 전체에서 산출한 엣지 집합 · 갈래 분포 · 참조 일곱 갈래.
fn 한_판(
    files: &[PathBuf],
    root: &Path,
    rules: RustScopeRules,
) -> (BTreeSet<엣지>, 갈래, usize, 갈래합) {
    let language = tree_sitter::Language::new(tree_sitter_rust::LANGUAGE);
    let 스냅샷 = Snapshot::single(RepoId::new("pal"), TreeRef::Committed(ObjectName::from_bytes([0u8; 20])));
    let mut 집합 = BTreeSet::new();
    let mut 분포 = 갈래::default();
    let mut 기여_파일 = BTreeSet::new();
    let mut 합 = 갈래합::default();

    for f in files {
        let Ok(source) = std::fs::read(f) else { continue };
        let Ok(g) = pal_extract::rust_extract_with(&source, rules) else { continue };
        let Capable::Present(chain) = &g.scopes else { continue };
        let rel = f.strip_prefix(root).unwrap_or(f).to_string_lossy().replace('\\', "/");

        // **좌표는 자리로 만든다** — 변형끼리 대조하는 것이 목적이라 이름 충돌이
        // 없어야 하고, 심볼 자리는 변형과 무관하게 같다.
        let path = RepoPath::new(rel.clone());
        let nodes: Vec<SymbolNode> = g
            .symbols
            .iter()
            .enumerate()
            .map(|(i, s): (usize, &Symbol)| SymbolNode {
                id: SymbolId::compute(
                    &RepoId::new("pal"),
                    &path,
                    &[],
                    &format!("#{i}"),
                    &Discriminator::new(s.kind, 0),
                ),
                path: path.clone(),
                container: Vec::new(),
                name: s.name.clone(),
                kind: s.kind,
                body: s.body,
                span: s.span,
                identity: s.identity,
            })
            .collect();

        // ① 정본 — `pal_core` 가 센 것.
        let counts = pal_core::file_edges(&g.symbols, &nodes, chain, &[], &스냅샷).counts;
        합.더한다(&counts);

        // ② 자기 셈 — 같은 규칙을 자리로 다시 적용해 **바이트를 들고 있는다.**
        let mut parser = tree_sitter::Parser::new();
        if parser.set_language(&language).is_err() {
            continue;
        }
        let Some(tree) = parser.parse(&source, None) else { continue };
        let mut 이_파일 = 0usize;
        for r in &chain.refs {
            let RefResolution::Bound { scope, binding } = r.resolved else { continue };
            let Some(b) = chain.scopes.get(scope.0 as usize).and_then(|s| s.bindings.get(binding as usize))
            else {
                continue;
            };
            if b.declared_at == r.at {
                continue;
            }
            let BoundSymbol::Symbol(to) = b.symbol else { continue };
            let Some(from) = 가장_안쪽(&g.symbols, r.at) else { continue };
            이_파일 += 1;
            집합.insert((rel.clone(), from, to.0));
            기여_파일.insert(rel.clone());
            if r.namespace == pal_core::Namespace::Type {
                분포.타입 += 1;
            } else if 호출인가(tree.root_node(), r.at) {
                분포.호출 += 1;
            } else {
                분포.그밖 += 1;
            }
        }
        assert_eq!(
            이_파일, counts.edges,
            "✗ 검산 — {rel} 에서 자기 셈({이_파일})과 file_edges({}) 가 갈렸다",
            counts.edges
        );
    }
    (집합, 분포, 기여_파일.len(), 합)
}

/// 같은 스코프에 같은 이름·이름 공간의 **아이템 선언이 둘 이상인 자리** — `E3` 이 이 수를 본다.
///
/// 소유자의 답이 *"동명 선언 72건이 각기 다른 스코프에 서서 가짜 엣지가 사라진다"* 였다.
/// `impl` 이 스코프를 열면 그 72 건이 갈라진다.
///
/// ⚠ **남는 것이 `cfg` 쌍둥이뿐은 아니다**(정반합 판 1). 같은 이름의 값 아이템과 타입
/// 아이템도 든다 — 이 표가 타입 아이템을 `Namespace::Value` 에도 묶기 때문이다(`A10`).
/// 합(合)이 보기 12 개를 열어 **10 은 `cfg` 쌍둥이 · 2 는 아니다** 로 갈랐다.
fn 동명_재선언(
    files: &[PathBuf],
    root: &Path,
    rules: RustScopeRules,
) -> (usize, usize, Vec<String>) {
    let mut 자리 = 0usize;
    let mut 파일 = BTreeSet::new();
    let mut 보기 = Vec::new();
    for f in files {
        let Ok(source) = std::fs::read(f) else { continue };
        let Ok(g) = pal_extract::rust_extract_with(&source, rules) else { continue };
        let Capable::Present(chain) = &g.scopes else { continue };
        let mut 이_파일 = 0usize;
        for s in &chain.scopes {
            let mut 셈: BTreeMap<(&str, bool), usize> = BTreeMap::new();
            for b in &s.bindings {
                if b.hoisted {
                    *셈.entry((b.name.as_str(), b.namespace == pal_core::Namespace::Type))
                        .or_default() += 1;
                }
            }
            for ((name, 타입), n) in &셈 {
                if *n > 1 {
                    이_파일 += 1;
                    if 보기.len() < 12 {
                        let rel = f.strip_prefix(root).unwrap_or(f).to_string_lossy().to_string();
                        보기.push(format!(
                            "{rel}:{name}{}×{n}",
                            if *타입 { "(타입)" } else { "(값)" }
                        ));
                    }
                }
            }
        }
        if 이_파일 > 0 {
            자리 += 이_파일;
            파일.insert(f.clone());
        }
    }
    (자리, 파일.len(), 보기)
}

fn 가장_안쪽(symbols: &[Symbol], byte: usize) -> Option<u32> {
    let mut best: Option<(usize, u32)> = None;
    for (i, s) in symbols.iter().enumerate() {
        if s.span.byte_start > byte || byte >= s.span.byte_end {
            continue;
        }
        let width = s.span.byte_end - s.span.byte_start;
        let ix = u32::try_from(i).unwrap_or(u32::MAX);
        if best.is_none_or(|(w, _)| width < w) {
            best = Some((width, ix));
        }
    }
    best.map(|(_, ix)| ix)
}

/// 표본 한 줄 — 손으로 대조할 수 있게 **좌표와 소스 줄**을 함께 진다.
struct 표본줄 {
    갈래: &'static str,
    경로: String,
    줄: usize,
    이름: String,
    가는_곳: String,
    소스: String,
}

/// `E1` — 층화 표본을 뽑는다. **배분은 위 모듈 주석이 미리 고정했다.**
fn 표본(files: &[PathBuf], root: &Path) -> Vec<표본줄> {
    let language = tree_sitter::Language::new(tree_sitter_rust::LANGUAGE);
    let mut 갈래별: BTreeMap<&'static str, Vec<표본줄>> = BTreeMap::new();
    for f in files {
        let Ok(source) = std::fs::read(f) else { continue };
        let Ok(g) = pal_extract::rust_extract_with(&source, RustScopeRules::기준) else { continue };
        let Capable::Present(chain) = &g.scopes else { continue };
        let rel = f.strip_prefix(root).unwrap_or(f).to_string_lossy().replace('\\', "/");
        let mut parser = tree_sitter::Parser::new();
        if parser.set_language(&language).is_err() {
            continue;
        }
        let Some(tree) = parser.parse(&source, None) else { continue };
        for r in &chain.refs {
            let RefResolution::Bound { scope, binding } = r.resolved else { continue };
            let Some(b) =
                chain.scopes.get(scope.0 as usize).and_then(|s| s.bindings.get(binding as usize))
            else {
                continue;
            };
            if b.declared_at == r.at {
                continue;
            }
            let BoundSymbol::Symbol(to) = b.symbol else { continue };
            let Some(from) = 가장_안쪽(&g.symbols, r.at) else { continue };
            let 갈래 = if r.namespace == pal_core::Namespace::Type {
                "타입"
            } else if 호출인가(tree.root_node(), r.at) {
                "호출·매크로"
            } else {
                "그밖"
            };
            let 줄 = source[..r.at].iter().filter(|c| **c == b'\n').count() + 1;
            let 시작 = source[..r.at].iter().rposition(|c| *c == b'\n').map_or(0, |i| i + 1);
            let 끝 = source[r.at..].iter().position(|c| *c == b'\n').map_or(source.len(), |i| r.at + i);
            갈래별.entry(갈래).or_default().push(표본줄 {
                갈래,
                경로: rel.clone(),
                줄,
                이름: g.symbols[from as usize].name.clone(),
                가는_곳: g.symbols[to.0 as usize].name.clone(),
                소스: String::from_utf8_lossy(&source[시작..끝]).trim().to_owned(),
            });
        }
    }
    let 배분 = [("타입", 15usize), ("호출·매크로", 20), ("그밖", 15)];
    let mut out = Vec::new();
    for (갈래, 몫) in 배분 {
        let Some(v) = 갈래별.get_mut(갈래) else { continue };
        v.sort_by(|a, b| (&a.경로, a.줄).cmp(&(&b.경로, b.줄)));
        if v.is_empty() {
            continue;
        }
        let 간격 = v.len().div_ceil(몫).max(1);
        for (i, row) in v.drain(..).enumerate() {
            if i % 간격 == 0 && out.iter().filter(|r: &&표본줄| r.갈래 == 갈래).count() < 몫 {
                out.push(row);
            }
        }
    }
    out
}

fn main() {
    let root = PathBuf::from(std::env::args().nth(1).unwrap_or_else(|| ".".to_owned()));
    let 표본_모드 = std::env::args().any(|a| a == "--표본");
    let mut files = Vec::new();
    훑는다(&root, &mut files);
    if 표본_모드 {
        let rows = 표본(&files, &root);
        println!("E1 층화 표본 — {} 건 (타입 15 · 호출·매크로 20 · 그 밖 15 배분)", rows.len());
        for (i, r) in rows.iter().enumerate() {
            println!(
                "{:>3}  [{}] {}:{}  {} → {}\n      {}",
                i + 1,
                r.갈래,
                r.경로,
                r.줄,
                r.이름,
                r.가는_곳,
                r.소스
            );
        }
        return;
    }
    println!("코퍼스 — `.rs` {} 파일 · 뿌리 {}", files.len(), root.display());

    let 기준 = RustScopeRules::기준;
    let (기준_집합, 분포, 기여, 기준_합) = 한_판(&files, &root, 기준);
    println!(
        "\n기준 — 엣지 {} · 기여 파일 {기여}",
        기준_집합.len()
    );
    let 참조_합 = 분포.타입 + 분포.호출 + 분포.그밖;
    println!(
        "V11 갈래 분포 — 타입 참조 {} · 호출·매크로 {} · 그 밖 {} (참조 {참조_합})",
        분포.타입, 분포.호출, 분포.그밖
    );
    println!("기준 참조 갈래 — {}", 기준_합.줄());

    let 변형: Vec<(&str, &str, RustScopeRules)> = vec![
        ("V1", "파라미터 선언을 끈다", RustScopeRules { 파라미터_선언: false, ..기준 }),
        ("V2", "패턴 선언을 끈다", RustScopeRules { 패턴_선언: false, ..기준 }),
        ("V3", "impl 이 스코프를 안 연다", RustScopeRules { impl_이_연다: false, ..기준 }),
        (
            "V4",
            "아이템 선언을 hoist_home 으로 놓는다",
            RustScopeRules { 아이템을_hoist_home_에: true, ..기준 },
        ),
        ("V5", "속성 배제를 끈다", RustScopeRules { 속성_배제: false, ..기준 }),
        (
            "V6",
            "매크로 앞형제 거르기를 끈다",
            RustScopeRules { 매크로_앞형제_거르기: false, ..기준 },
        ),
        ("V7", "경로 꼬리 배제를 끈다", RustScopeRules { 경로_꼬리_배제: false, ..기준 }),
        (
            "V8",
            "field_identifier 배제를 끈다",
            RustScopeRules { 필드_식별자_배제: false, ..기준 },
        ),
        ("V9", "use 절 배제를 끈다", RustScopeRules { use_절_배제: false, ..기준 }),
        ("V10", "cfg 쌍둥이를 해소한다", RustScopeRules { cfg_쌍둥이_해소: true, ..기준 }),
    ];

    // `V4` 는 「`V3` 와 **같은 집합이 나오면 안 된다**」가 조건이라 따로 들고 있는다.
    let mut 집합들: BTreeMap<&str, BTreeSet<엣지>> = BTreeMap::new();
    let mut 실패 = 0usize;
    println!(
        "\n{:<5} {:<34} {:>7} {:>7} {:>7}  {:<10} 판정",
        "조건", "무엇을 껐나", "엣지", "가짜", "누락", "엣지축"
    );
    let mut 갈래로만 = Vec::new();
    for (id, 설명, rules) in 변형 {
        let (집합, _, _, 합) = 한_판(&files, &root, rules);
        let 가짜 = 집합.difference(&기준_집합).count();
        let 누락 = 기준_집합.difference(&집합).count();
        let 엣지가_다른가 = 가짜 + 누락 > 0;
        let 갈래가_다른가 = 합 != 기준_합;
        // ★★ **판정은 엣지 축 하나가 진다** (소유자 결정 2026-09-08 · 정반합 판 2).
        //
        //   앞 판은 `엣지가_다른가 || 갈래가_다른가` 로 판정했다. 그 축은 **등록된
        //   것이 아니다** — 잠긴 의도의 `V` 절은 엣지 집합을 증인으로 세운다. 그리고
        //   `V5`~`V9` 는 전부 「거르기를 끈다」라 끄면 참조가 늘기만 하고, `file_edges`
        //   는 참조를 정확히 한 갈래에 넣으므로 **갈래합은 언제나 갈린다.** 곧 넓힌
        //   축에서는 그 다섯 행이 원리상 `실패` 를 못 올린다 — 합격선을 넓힌 것이다.
        //
        //   갈래 축은 **보고로만 남긴다**(아래 `갈래로만`). 지우면 `V9` 가 참조를
        //   `미해소` 로 391 건 밀어 넣는 관측이 함께 사라진다.
        if !엣지가_다른가 {
            실패 += 1;
        }
        if !엣지가_다른가 && 갈래가_다른가 {
            갈래로만.push((id, 합));
        }
        println!(
            "{id:<5} {설명:<34} {:>7} {가짜:>7} {누락:>7}  {:<10} {}",
            집합.len(),
            if 엣지가_다른가 { "다르다" } else { "같다" },
            if 엣지가_다른가 {
                "다르다 ✓"
            } else {
                "같다 ✗ — 이 규칙은 등록된 축에서 아무것도 안 잰다"
            }
        );
        집합들.insert(id, 집합);
    }
    for (id, 합) in &갈래로만 {
        println!("  ⚠ {id} 은 **엣지 축이 아니라 참조 갈래 축**에서만 갈린다 — {}", 합.줄());
    }

    // `V4` — 두 변형의 엣지 집합이 **비트 단위로 같으면** 호이스팅이 impl 을 상쇄한 것이다.
    let 같나 = 집합들.get("V3") == 집합들.get("V4");
    println!(
        "\nV4 대 V3 — {}",
        if 같나 {
            실패 += 1;
            "같다 ✗ — hoist_home 이 impl 스코프를 정확히 상쇄한다"
        } else {
            "다르다 ✓"
        }
    );

    if 실패 > 0 {
        println!("\n✗ 변형 {실패} 개가 아무것도 안 갈랐다");
        std::process::exit(1);
    }
    let (기준_동명, 기준_파일, 보기) = 동명_재선언(&files, &root, 기준);
    let (열지_않으면, 안_연_파일, _) =
        동명_재선언(&files, &root, RustScopeRules { impl_이_연다: false, ..기준 });
    println!(
        "\nE3 동명 아이템 재선언 자리 — 기준 {기준_동명}(파일 {기준_파일}) · \
impl 을 안 열면 {열지_않으면}(파일 {안_연_파일})"
    );
    println!("   남은 자리 표본 — {}", 보기.join(" · "));
    // ⚠ **「`cfg` 쌍둥이뿐」이라고 적으면 안 된다** (정반합 판 1 의 반(反)이 잡았다).
    //   남은 자리에는 **같은 이름의 값 아이템과 타입 아이템**도 있다 —
    //   `install/blocks.rs` 의 `pub enum 상태` 와 `pub fn 상태`,
    //   `tests/install_boundary.rs` 의 `struct 방` 과 `fn 방`. 두 파일 다 `cfg` 가 없다.
    //   까닭은 이 표가 **타입 아이템을 `Namespace::Value` 에도 묶기** 때문이다
    //   (`rust_scopes.rs` 의 `declare_plain` · 조건 `A10`). 값 자리에서 타입 이름이
    //   쓰이는 358 건을 얻는 대가로 이 겹침이 생긴다.
    println!(
        "   이 자리는 엣지가 아니라 `Ambiguous` 다 — 해소하면 V10 만큼 엣지가 늘어난다"
    );
    println!(
        "   ⚠ 남은 것이 `cfg` 쌍둥이뿐은 **아니다** — 같은 이름의 값·타입 아이템도 든다"
    );

    // ★ **축을 뭉개지 않는다** (독립 리뷰 R1). 앞 판은 여기서 *"전부 기준과 갈렸다"*
    //   로 닫았는데, 그러면 **엣지 축에서 안 갈린 변형**이 그 한 줄에 삼켜진다.
    //   등록된 축은 엣지 집합이고 참조 갈래 축은 그것이 못 보는 것을 보는 둘째 자다.
    if 갈래로만.is_empty() {
        println!("\n✓ 변형 전부가 **엣지 집합**에서 기준과 갈렸다");
    } else {
        println!(
            "\n△ 변형 {} 개는 **엣지 집합에서 안 갈렸다** — 갈린 것은 참조 갈래 축뿐이다: {}",
            갈래로만.len(),
            갈래로만.iter().map(|(id, _)| *id).collect::<Vec<_>>().join(" · ")
        );
        println!("   등록된 축이 엣지 집합이므로 **이 줄을 통과로 읽지 마라.**");
    }
}
