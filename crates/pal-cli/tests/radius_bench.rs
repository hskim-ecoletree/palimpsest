//! **반경별 비용 벤치** — `[f09.4.pass]` ④ 가 등록한 넷을 잰다.
//!
//! ```bash
//! cargo test -p pal-cli --release --test radius_bench -- --ignored --nocapture
//! ```
//!
//! # 등록이 요구한 것과 이 파일이 재는 것
//!
//! `corpus/criteria.toml` 의 `[f09.4.pass]` ④ (`bench_by_radius`):
//!
//! > **재는 것 넷**: `symbol` · `callers` · `closure(2)` · **증분 갱신**. **두 규모**에서
//! > 재고 **회차 3 의 최솟값과 회차 간 분산**을 함께 적는다.
//! > **합격선은 절대 시간이 아니라 이것이다**: **증분 갱신이 전체 재계산보다 빠르다.**
//!
//! ## ⚠⚠ 넷째 팔은 **대리물**이다 — 등록이 말한 것이 이 빌드에 없다
//!
//! *"증분으로 **상태를** 갱신하는 파이프라인"* 이 이 빌드에 없다. 저장소에서 「증분 갱신」이
//! 서는 자리는 `crates/pal-cli/src/touch.rs:18` 의 *"`WATCH` 색인이고 F09 가 증분 갱신을
//! **위해** 세웠다"* 뿐이다 — **색인은 있고 갱신 경로는 없다.**
//!
//! 그래서 넷째 팔은 그 색인이 사려던 것을 잰다:
//!
//! | | 무엇 |
//! |---|---|
//! | **색인 조회** | `IntentStore::bindings_watching(&changed)` — `WATCH` 다중맵을 타고 닿는 결박만 |
//! | **전수 훑기** | `IntentStore::all()` 뒤에 감시 집합을 손으로 걸러낸다 |
//!
//! **이 치환을 산출 머리에 찍는다.** 조용히 다른 것을 재면 「선 장치 · 안 닫힌 판정」이
//! 되고, 그것이 `PM2-15` 가 이름 붙인 자리다. 그러므로 **`[f09.4.pass]` ④ 는 이 벤치가
//! 돌아도 `대조불가` 다** — 등록된 합격선의 주어가 이 빌드에 없다.
//!
//! ## 그리고 `changed` 크기를 축에 넣는다
//!
//! 등록된 합격선이 **`changed` 의 크기를 안 적었다.** 그런데 그 값이 판정을 뒤집는다 —
//! 앞 회차가 곡선을 봤다(`k=1` 32.07× → `k=34` 0.926×). 하나만 골라 재면 **부하 선택이
//! 합격선을 정한다.** 그래서 작은 `changed` 와 큰 `changed` 를 **둘 다** 돌고, 두 비가
//! **1 을 건너뛰는지**를 산출에 적는다. 그것이 *"부하 미규정은 게이트에 결함으로 적는다"*
//! (소유자 ⟨2026-09-13⟩)의 기계 형태다.
//!
//! # 합격선은 절대 시간이 아니다
//!
//! `[f05].criterion_decision` 이 `criterion` 을 안 들이기로 했고 여기서 형태를 안 바꾼다.
//! `[f04.pass].bench_ratio` 가 *"기계에 의존하는 값은 합격선이 될 수 없다"* 를 적었다.
//! **비와 선형성만 적고, 회차 간 분산을 함께 실어** 분산이 커서 판정이 뒤집히면 그 사실이
//! 산출에 보이게 한다.

use std::collections::BTreeMap;
use std::time::{Duration, Instant};

use pal_core::{
    Binding, BodyDigest, BoundTime, Discriminator, EntityId, EntityKind, EntityOrigin,
    ExportDigest, ExtractGrade, FileRow, IdentityGrade, LanguageId, NewBinding, ObjectName, Radius,
    RefCounts, ReferenceEdge, RepoId, RepoPath, Slot, Snapshot, Span, SymbolId, SymbolKind,
    SymbolNode, TreeRef, WatchEntry, expand,
};
use pal_intent::IntentStore;
use pal_store::{FileStitch, Projection};

/// 작은 규모. 큰 규모는 이것의 4 배다 — **비가 4 배 미만이면 선형성이 대조 불가다.**
const 작게: usize = 2_000;
const 배수: usize = 4;
const 배수_실수: f64 = 4.0;
const 회차: usize = 3;
/// 파일 하나에 심볼 넷 — 그중 셋이 앞의 하나를 가리킨다(차수가 상수로 유지된다).
const 파일당: usize = 4;
/// 결박을 몇 건 걸고 재나 — `결박 수 × 폐포 크기` 가 예산의 축이다.
const 결박수: usize = 200;

fn 스냅샷() -> Snapshot {
    Snapshot::single(RepoId::new("r"), TreeRef::Committed(ObjectName::from_bytes([7; 20])))
}

/// 심볼 `n` 개짜리 그래프. **파일 안에 셋이 첫 심볼을 가리키고 파일을 잇는 사슬이 있다.**
///
/// `pal-query` 의 F05 벤치와 같은 모양이다 — 그래야 두 산출을 나란히 읽을 수 있다.
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
            imports: Slot::NotBuilt,
            pending: Vec::new(),
            symbols,
        });
    }
    out
}

/// 회차 셋을 돌고 **최솟값과 분산**을 산출한다. `pal-query` 의 F05 벤치와 같은 자다.
fn 잰다(name: &str, mut f: impl FnMut() -> usize) -> Duration {
    let mut times = Vec::with_capacity(회차);
    let mut 확인 = 0usize;
    for _ in 0..회차 {
        let t = Instant::now();
        확인 = f();
        times.push(t.elapsed());
    }
    // **하한이다** — 아무것도 안 하면 시간이 예쁘다.
    assert!(확인 > 0, "{name}: 잰 연산이 아무것도 안 산출했다");
    let min = *times.iter().min().expect("회차");
    let max = *times.iter().max().expect("회차");
    let 분산 = if min.as_nanos() == 0 { 0.0 } else { max.as_secs_f64() / min.as_secs_f64() };
    println!("    {name:<28} min {min:>10.3?} · max/min {분산:.2}배 · 산출 {확인}");
    min
}

struct 반경값 {
    symbol: Duration,
    callers: Duration,
    closure2: Duration,
    /// 감시 집합 크기의 합 — **시간이 아니라 부피다.** 반경이 무엇을 늘리는지가 여기 있다.
    감시: BTreeMap<&'static str, usize>,
}

fn 한_규모_반경(n: usize) -> 반경값 {
    let dir = std::env::temp_dir().join(format!("pal-f09-radius-{n}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("임시 자리");
    let p = Projection::open(&dir.join("index.redb")).expect("2층");
    let files = 그래프(n);
    p.stitch("bench", &files, 1_000, &스냅샷()).expect("스티칭");

    // **대상은 파일마다 첫 심볼**이다 — 그것이 같은 파일의 셋에게 불린다(차수 3).
    let 대상: Vec<SymbolId> = files.iter().map(|f| f.symbols[0].id).collect();

    println!(
        "  규모 {n} — 심볼 {} · 파일 {}",
        p.count().expect("심볼"),
        p.file_count().expect("파일")
    );

    let mut 감시 = BTreeMap::new();
    let 재기 = |r: Radius, 이름: &'static str, 감시: &mut BTreeMap<&'static str, usize>| {
        let mut 합 = 0usize;
        let d = 잰다(이름, || {
            합 = 대상.iter().map(|t| expand(*t, &r, &p).len()).sum();
            합
        });
        감시.insert(이름, 합);
        d
    };
    let out = 반경값 {
        symbol: 재기(Radius::Symbol, "① symbol 반경", &mut 감시),
        callers: 재기(Radius::Callers, "② callers 반경", &mut 감시),
        closure2: 재기(Radius::Closure { k: 2 }, "③ closure(2) 반경", &mut 감시),
        감시,
    };
    let _ = std::fs::remove_dir_all(&dir);
    out
}

/// 결박 `결박수` 건을 담은 의도 저장소와, 그 감시 집합에서 뽑은 좌표들.
fn 의도_저장소(dir: &std::path::Path, p: &Projection, 대상: &[SymbolId]) -> (IntentStore, Vec<SymbolId>) {
    let store = IntentStore::open(&dir.join("intent.redb")).expect("의도 저장소");
    let mut 감시된: Vec<SymbolId> = Vec::new();
    for (i, t) in 대상.iter().take(결박수).enumerate() {
        let watch: Vec<WatchEntry> = expand(*t, &Radius::Callers, p)
            .into_iter()
            .map(|s| {
                let body = p.symbol(s).expect("조회").map_or(BodyDigest::of_normalized(b""), |x| x.body);
                감시된.push(s);
                WatchEntry { symbol: s, digest: body }
            })
            .collect();
        let b = Binding::new(NewBinding {
            subject: EntityId::mint(EntityKind::new("decision"), EntityOrigin::Hand),
            target: *t,
            note: format!("벤치 결박 {i}"),
            bound_at: 스냅샷(),
            bound_at_time: BoundTime::Unrecorded,
            radius: Radius::Callers,
            watch,
        });
        store.record(&b).expect("결박");
    }
    감시된.sort_unstable();
    감시된.dedup();
    (store, 감시된)
}

/// ★ **넷째 팔** — `WATCH` 색인 조회 대 전수 훑기. **`changed` 크기를 축으로 둔다.**
fn 색인_대_전수(dir: &std::path::Path, store: &IntentStore, 감시된: &[SymbolId], k: usize) -> (Duration, Duration, usize) {
    let _ = dir;
    let changed: Vec<SymbolId> = 감시된.iter().copied().take(k).collect();
    let 색인 = 잰다(&format!("④-가 WATCH 색인 조회 (changed {k})"), || {
        store.bindings_watching(&changed).expect("색인").len()
    });
    let 전수 = 잰다(&format!("④-나 전수 훑기      (changed {k})"), || {
        // **같은 답이 나오는 다른 길이다** — 색인을 안 타고 전부 읽어 손으로 걸러낸다.
        // 이것이 없으면 「색인이 빠르다」가 무엇보다 빠른지 말할 수 없다.
        store
            .all()
            .expect("전수")
            .into_iter()
            .filter(|b| b.watch.iter().any(|w| changed.contains(&w.symbol)))
            .count()
    });
    let 답 = store.bindings_watching(&changed).expect("색인").len();
    (색인, 전수, 답)
}

fn 배(x: Duration, y: Duration) -> f64 {
    if x.as_secs_f64() == 0.0 { f64::INFINITY } else { y.as_secs_f64() / x.as_secs_f64() }
}

#[test]
#[ignore = "규모 벤치다 — `--release -- --ignored` 로 돈다"]
fn 반경별_벤치와_선형성() {
    let 크게 = 작게 * 배수;
    println!();
    println!("═══ F09 반경별 벤치 · `[f09.4.pass]` ④ ═══");
    println!();
    println!("  ⚠⚠ **넷째 팔은 대리물이다.** 등록된 합격선의 주어(*\"증분으로 상태를 갱신하는");
    println!("     파이프라인\"*)가 이 빌드에 없다. 재는 것은 `WATCH` 색인 조회 대 전수 훑기이고,");
    println!("     그래서 `[f09.4.pass]` ④ 는 이 벤치가 돌아도 **대조불가**다.");
    println!();
    println!("  ⚠ **합격선이 `changed` 크기를 안 적었다.** 그 값이 판정을 뒤집으므로 작은 것과");
    println!("     큰 것을 둘 다 돌고 두 비가 1 을 건너뛰는지 적는다.");
    println!();
    println!("  회차 {회차} · 최솟값 · 규모 {작게} → {크게} ({배수}배) · 결박 {결박수} 건");
    println!();

    let a = 한_규모_반경(작게);
    println!();
    let b = 한_규모_반경(크게);
    println!();

    println!("  반경이 무엇을 늘리나 — 감시 집합 크기의 합");
    println!("    규모 {작게}: {:?}", a.감시);
    println!("    규모 {크게}: {:?}", b.감시);
    println!();

    let symbol배 = 배(a.symbol, b.symbol);
    let callers배 = 배(a.callers, b.callers);
    let closure배 = 배(a.closure2, b.closure2);
    println!("  선형성 — 노드 {배수}배에 대해");
    println!("    ① symbol       {symbol배:.2}배   (선: {}배 이내 — 대상 수가 4 배다)", 배수 * 2);
    println!("    ② callers      {callers배:.2}배   (선: {}배 이내)", 배수 * 2);
    println!("    ③ closure(2)   {closure배:.2}배   (기록만 — 차수와 k 의 함수다)");
    println!();

    // ── ④ 넷째 팔 — 두 `changed` 크기에서 ────────────────────────────────
    let dir = std::env::temp_dir().join(format!("pal-f09-watch-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("임시 자리");
    let p = Projection::open(&dir.join("index.redb")).expect("2층");
    let files = 그래프(크게);
    p.stitch("bench", &files, 1_000, &스냅샷()).expect("스티칭");
    let 대상: Vec<SymbolId> = files.iter().map(|f| f.symbols[0].id).collect();
    let (store, 감시된) = 의도_저장소(&dir, &p, &대상);
    println!("  ④ WATCH 색인 대 전수 훑기 — 결박 {결박수} 건 · 감시된 좌표 {}", 감시된.len());
    println!();
    let 작은k = 1usize;
    let 큰k = 감시된.len();
    let (색인_작, 전수_작, 답_작) = 색인_대_전수(&dir, &store, &감시된, 작은k);
    let (색인_큰, 전수_큰, 답_큰) = 색인_대_전수(&dir, &store, &감시된, 큰k);
    println!();
    let 이득_작 = 배(색인_작, 전수_작);
    let 이득_큰 = 배(색인_큰, 전수_큰);
    println!("  ④ 색인이 전수보다 몇 배 빠른가 — **`changed` 크기가 판정을 뒤집는다**");
    println!("    changed {작은k:>5} (닿는 결박 {답_작:>4}) → {이득_작:.3}배");
    println!("    changed {큰k:>5} (닿는 결박 {답_큰:>4}) → {이득_큰:.3}배");
    let 건너뛰나 = (이득_작 > 1.0) != (이득_큰 > 1.0);
    println!(
        "    두 비가 1 을 건너뛰나 — **{}**{}",
        if 건너뛰나 { "그렇다" } else { "아니다" },
        if 건너뛰나 { "  ← 부하 선택이 합격선을 정한다. 게이트에 결함으로 적는다" } else { "" }
    );
    println!();
    let _ = std::fs::remove_dir_all(&dir);

    // ── 합격선 — **절대 시간이 아니다** ──────────────────────────────────
    //
    // ① 은 일이 4 배로 늘어난다(대상 수가 파일 수에 비례한다). 그보다 빨리 자라면
    //   `expand` 안에서 무언가가 초선형이다.
    assert!(
        symbol배 <= 배수_실수 * 2.0,
        "symbol 반경이 {symbol배:.2}배로 자랐다 (선 {}배) — 대상 수에 비례하지 않는다",
        배수 * 2
    );
    assert!(
        callers배 <= 배수_실수 * 2.0,
        "callers 반경이 {callers배:.2}배로 자랐다 (선 {}배) — 차수가 상수인 그래프에서 \
         초선형이면 역방향 조회가 스캔이다",
        배수 * 2
    );
    // **감시 집합은 반경이 넓어지면 커진다** — 안 커지면 이 벤치가 반경을 안 재고 있다.
    assert!(
        a.감시["② callers 반경"] > a.감시["① symbol 반경"],
        "callers 가 symbol 보다 안 컸다 — 엣지가 없다는 뜻이고 그러면 이 벤치가 \
         아무것도 안 잰다 ({:?})",
        a.감시
    );
    assert!(
        a.감시["③ closure(2) 반경"] >= a.감시["② callers 반경"],
        "closure(2) 가 callers 보다 작았다 ({:?})",
        a.감시
    );
    // ★ **④ 에는 합격선을 안 건다.** 등록된 합격선의 주어가 이 빌드에 없으므로
    //   그 값을 통과·반증으로 세면 **다른 것을 재고 그 이름을 쓰는 것**이 된다.
    //   `대조불가` 로 적고 두 비를 산출에 남긴다 — 그것이 이 팔이 지는 전부다.
}
