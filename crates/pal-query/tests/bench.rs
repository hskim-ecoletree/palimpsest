//! 4종 벤치 + **선형성** — `[f05.3.pass]` ④.
//!
//! ```bash
//! cargo test -p pal-query --release -- --ignored --nocapture
//! ```
//!
//! # `criterion` 을 안 들인 이유가 이 파일의 형태다
//!
//! 등록(`[f05].criterion_decision`)이 적었다 — **합격선이 절대 시간이 아니라 비율과
//! 선형성**이고, 그 둘은 마이크로벤치의 통계 잡음에 둔감하다. 절대 시간은 기계에
//! 의존해 합격선이 될 수 없다(F04 `[f04.pass].bench_ratio` 가 이미 그렇게 등록했다).
//!
//! 대신 **회차 셋을 돌고 최솟값을 쓰고 분산을 함께 적는다.** 회차 간 분산이 커서
//! 선형성 판정이 뒤집히면 그때 `criterion` 이 필요하다는 뜻이고, 그 사실이 이 산출에
//! 보이게 해 두는 것이 여기서 지는 몫이다.
//!
//! # 두 규모로 재는 이유
//!
//! **규모가 하나면 선형성이 정의되지 않는다.** 노드 수를 4 배로 올리고 각 연산이
//! 어떻게 자라는지 본다. 등록한 선:
//!
//! | 연산 | 선 |
//! |---|---|
//! | ① 심볼 조회 | **log 이하** — B+tree 조회다 |
//! | ④ 전체 재구축 | **k 배의 2 배 이내** |
//! | ② 1홉 역방향 · ③ 3홉 BFS | **기록만** — 차수와 깊이에 매이지 노드 수의 함수가 아니다 |

use std::time::{Duration, Instant};

use pal_core::{
    BodyDigest, Budget, CANDIDATE_LIMIT, Discriminator, Elision, ExportDigest, ExtractGrade,
    FileRow, IdentityGrade, LanguageId, PROVISIONAL_PATH_PRODUCT_MAX, PROVISIONAL_TRAVERSAL_DEPTH,
    RefCounts, ReferenceEdge, RepoId, RepoPath, Slot, Snapshot, Span, Step, SymbolId, SymbolKind,
    SymbolNode, TreeRef, traverse,
};
use pal_store::{FileStitch, Projection};

/// 작은 규모. 큰 규모는 이것의 4 배다 — **비가 4 배 미만이면 선형성이 대조 불가다.**
const 작게: usize = 2_000;
const 배수: usize = 4;
/// 위와 같은 값의 실수판 — 캐스팅을 코드 한가운데 두지 않는다.
const 배수_실수: f64 = 4.0;
const 회차: usize = 3;
/// 파일 하나에 심볼 넷 — 그중 셋이 앞의 하나를 가리킨다(차수가 상수로 유지된다).
const 파일당: usize = 4;

fn 스냅샷() -> Snapshot {
    Snapshot::single(RepoId::new("r"), TreeRef::Committed(pal_core::ObjectName::from_bytes([5; 20])))
}

/// 심볼 `n` 개짜리 그래프. **사슬이 하나 있어 3홉 BFS 가 잴 것이 있다.**
fn 그래프(n: usize) -> Vec<FileStitch> {
    let repo = RepoId::new("r");
    let files = n / 파일당;
    let mut out = Vec::with_capacity(files);
    let mut 앞_파일_첫심볼: Option<SymbolId> = None;
    for f in 0..files {
        let path = RepoPath::new(format!("f{f}.ts"));
        let node = |i: usize| SymbolNode {
            id: SymbolId::compute(
                &repo,
                &path,
                &[],
                &format!("s{f}_{i}"),
                &Discriminator::new(SymbolKind::Function, 0),
            ),
            path: path.clone(),
            container: Vec::new(),
            name: format!("s{f}_{i}"),
            kind: SymbolKind::Function,
            body: BodyDigest::of_normalized(format!("s{f}_{i}").as_bytes()),
            span: Span { byte_start: i, byte_end: i + 1, line_start: 1, line_end: 1 },
            identity: IdentityGrade::Exact,
        };
        let symbols: Vec<SymbolNode> = (0..파일당).map(node).collect();
        let mut edges: Vec<ReferenceEdge> = (1..파일당)
            .map(|i| ReferenceEdge { from: symbols[i].id, to: symbols[0].id, at: 스냅샷() })
            .collect();
        // **파일을 잇는 사슬** — 이것이 없으면 BFS 가 1 홉에서 멈춘다.
        if let Some(prev) = 앞_파일_첫심볼 {
            edges.push(ReferenceEdge { from: symbols[0].id, to: prev, at: 스냅샷() });
        }
        앞_파일_첫심볼 = Some(symbols[0].id);
        out.push(FileStitch {
            file: FileRow {
                path: path.clone(),
                language: LanguageId::new("TypeScript"),
                grade: ExtractGrade::L2,
                export_digest: Slot::Built(ExportDigest::from_bytes([2; 32])),
                refs: Slot::Built(RefCounts { edges: edges.len(), ..RefCounts::default() }),
            },
            exports: vec![(symbols[0].name.clone(), symbols[0].id)],
            edges,
            symbols,
        });
    }
    out
}

/// 회차 셋을 돌고 **최솟값과 분산**을 낸다.
fn 잰다(name: &str, mut f: impl FnMut() -> usize) -> Duration {
    let mut times = Vec::with_capacity(회차);
    let mut 확인 = 0usize;
    for _ in 0..회차 {
        let t = Instant::now();
        확인 = f();
        times.push(t.elapsed());
    }
    // **하한이다** — 아무것도 안 하면 시간이 예쁘다.
    assert!(확인 > 0, "{name}: 잰 연산이 아무것도 안 냈다");
    let min = *times.iter().min().expect("회차");
    let max = *times.iter().max().expect("회차");
    let 분산 = if min.as_nanos() == 0 { 0.0 } else { max.as_secs_f64() / min.as_secs_f64() };
    println!("    {name:<24} min {min:>10.3?} · max/min {분산:.2}배 · 산출 {확인}");
    min
}

struct 회차값 {
    조회: Duration,
    역방향: Duration,
    bfs: Duration,
    재구축: Duration,
}

fn 한_규모(n: usize) -> 회차값 {
    let dir = std::env::temp_dir().join(format!("pal-f05-bench-{n}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let p = Projection::open(&dir.join("index.redb")).expect("2층");
    let files = 그래프(n);
    p.stitch("bench", &files, 1_000).expect("스티칭");

    let 대상: Vec<SymbolId> = files.iter().map(|f| f.symbols[0].id).collect();
    let budget = Budget::new(
        CANDIDATE_LIMIT,
        PROVISIONAL_PATH_PRODUCT_MAX,
        PROVISIONAL_TRAVERSAL_DEPTH,
        n * 2,
    );

    println!("  규모 {n} — 심볼 {} · 파일 {}", p.count().expect("심볼"), p.file_count().expect("파일"));
    let out = 회차값 {
        조회: 잰다("① 심볼 조회", || {
            대상.iter().filter(|id| p.symbol(**id).expect("조회").is_some()).count()
        }),
        역방향: 잰다("② 1홉 역방향", || {
            대상.iter().map(|id| p.callers(*id).expect("역방향").len()).sum()
        }),
        bfs: 잰다("③ 3홉 BFS", || {
            대상
                .iter()
                .take(200)
                .map(|id| {
                    let mut el = Elision::none();
                    traverse(id, &budget, &mut el, |x| {
                        p.callees(*x).unwrap_or_default().into_iter().map(Step::exact).collect()
                    })
                    .len()
                })
                .sum()
        }),
        재구축: 잰다("④ 전체 재구축", || p.stitch("bench", &files, 1_000).expect("스티칭").symbols),
    };
    let _ = std::fs::remove_dir_all(&dir);
    out
}

#[test]
#[ignore = "규모 벤치다 — `--release -- --ignored` 로 돈다"]
fn 사종_벤치와_선형성() {
    let 크게 = 작게 * 배수;
    println!();
    println!("F05 4종 벤치 · 회차 {회차} · 최솟값 · 규모 {작게} → {크게} ({배수}배)");
    println!();
    let a = 한_규모(작게);
    println!();
    let b = 한_규모(크게);
    println!();

    let 배 = |x: Duration, y: Duration| {
        if x.as_secs_f64() == 0.0 { f64::INFINITY } else { y.as_secs_f64() / x.as_secs_f64() }
    };
    let 조회배 = 배(a.조회, b.조회);
    let 역방향배 = 배(a.역방향, b.역방향);
    let bfs배 = 배(a.bfs, b.bfs);
    let 재구축배 = 배(a.재구축, b.재구축);

    println!("  선형성 — 노드 {배수}배에 대해");
    println!("    ① 심볼 조회   {조회배:.2}배   (선: 노드 수에 비례 이하 — B+tree 조회)");
    println!("    ② 1홉 역방향  {역방향배:.2}배   (기록만 — 차수의 함수다)");
    println!("    ③ 3홉 BFS     {bfs배:.2}배   (기록만 — 깊이와 차수의 함수다)");
    println!("    ④ 전체 재구축 {재구축배:.2}배   (선: {}배 이내)", 배수 * 2);
    println!();

    // ① 은 **일이 4 배로 늘어난다**(대상 수가 파일 수에 비례한다). 조회 하나가 log 라면
    //    전체는 4 배 남짓이어야 하고, 그보다 빨리 자라면 조회가 스캔이 된 것이다.
    assert!(
        조회배 <= 배수_실수 * 2.0,
        "심볼 조회가 {조회배:.2}배로 자랐다 — B+tree 조회가 아니라 스캔이다"
    );
    // ④ 는 일이 4 배다. 그보다 크게 자라면 배치나 색인이 초선형이다.
    assert!(
        재구축배 <= 배수_실수 * 2.0,
        "전체 재구축이 {재구축배:.2}배로 자랐다 (선 {}배)",
        배수 * 2
    );
}
