//! **옛 `intent.redb` 의 결박이 새 빌드에서 읽힌다** — #77 · [R-21].
//!
//! # 왜 이 시험이 있나
//!
//! 감시 원소에 장식 기준값(`decor`)이 붙었다. 저장은 postcard 라 **자리 기반**이고, 옛 바이트를
//! 새 타입으로 풀면 멎거나 어긋난 채 풀린다. 그런데 이 파일은 재생 불가능한 유일한 데이터라
//! *"지우고 JSONL 에서 다시"* 가 답이 아니다 — 내보내기 뒤에 건 결박이 여기에만 있을 수 있다.
//!
//! 그래서 모양마다 표가 따로다(`binding` = 판 2 · `binding.v3` = 판 3). 여기서는 **옛 빌드가
//! 쓰던 바이트를 옛 표에 그대로 넣고** 새 빌드의 읽기 API 전부가 그것을 보는지 잰다.
//!
//! [R-21]: ../../../docs/plan/00-risks.md#r-21

use std::path::PathBuf;

use pal_core::{
    Binding, BindingId, BodyDigest, BoundTime, DecorBaseline, Discriminator, EntityId, EntityKind,
    EntityOrigin, NewBinding, ObjectName, PromotedBy, Radius, RepoId, RepoPath, Snapshot,
    SymbolId, SymbolKind, TreeRef, WatchEntry,
};
use pal_intent::IntentStore;
use redb::{Database, MultimapTableDefinition, TableDefinition};
use serde::Serialize;

/// 옛 빌드(판 2)의 표 — 이름이 곧 그 모양의 표식이다.
const 옛_표: TableDefinition<&str, Vec<u8>> = TableDefinition::new("binding");
const 옛_대상_색인: MultimapTableDefinition<&[u8], &str> = MultimapTableDefinition::new("bound_by");

/// 옛 빌드의 감시 원소 — **`decor` 가 없다.**
#[derive(Serialize)]
struct 옛감시 {
    symbol: SymbolId,
    digest: BodyDigest,
}

/// 옛 빌드의 결박 — 필드 순서가 곧 postcard 의 모양이다.
#[derive(Serialize)]
struct 옛결박<'a> {
    id: &'a BindingId,
    subject: &'a EntityId,
    target: SymbolId,
    note: &'a str,
    bound_at: &'a Snapshot,
    bound_at_time: BoundTime,
    radius: &'a Radius,
    watch: Vec<옛감시>,
    promoted_by: &'a PromotedBy,
}

fn 파일(tag: &str) -> (PathBuf, PathBuf) {
    let root = std::env::temp_dir().join(format!("pal-intent-legacy-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("임시 디렉터리");
    let file = root.join("intent.redb");
    (root, file)
}

fn 결박(name: &str, 장식: DecorBaseline) -> Binding {
    let target = SymbolId::compute(
        &RepoId::new("r"),
        &RepoPath::new("lib.rs"),
        &[],
        name,
        &Discriminator::new(SymbolKind::Struct, 0),
    );
    Binding::new(NewBinding {
        subject: EntityId::mint(EntityKind::new("decision"), EntityOrigin::Hand),
        target,
        note: format!("{name} 의 계약"),
        bound_at: Snapshot::single(RepoId::new("r"), TreeRef::Committed(ObjectName::from_bytes([7; 20]))),
        bound_at_time: BoundTime::Committed { epoch_secs: 1_700_000_000 },
        radius: Radius::Symbol,
        watch: vec![WatchEntry { symbol: target, digest: BodyDigest::of_normalized(name.as_bytes()), decor: 장식 }],
    })
}

/// 옛 빌드가 쓰던 그대로 — 옛 표에 옛 바이트를 넣는다.
fn 옛_빌드가_쓴다(file: &std::path::Path, b: &Binding) {
    let 바이트 = postcard::to_allocvec(&옛결박 {
        id: &b.id,
        subject: &b.subject,
        target: b.target,
        note: &b.note,
        bound_at: &b.bound_at,
        bound_at_time: b.bound_at_time,
        radius: &b.radius,
        watch: b.watch.iter().map(|w| 옛감시 { symbol: w.symbol, digest: w.digest }).collect(),
        promoted_by: &b.promoted_by,
    })
    .expect("직렬화");
    let db = Database::create(file).expect("옛 저장소");
    let w = db.begin_write().expect("쓰기");
    {
        let mut t = w.open_table(옛_표).expect("옛 표");
        t.insert(b.id.as_str(), 바이트).expect("옛 행");
        let mut idx = w.open_multimap_table(옛_대상_색인).expect("색인");
        idx.insert(b.target.as_bytes().as_slice(), b.id.as_str()).expect("색인 행");
    }
    w.commit().expect("커밋");
}

#[test]
fn 옛_표의_결박이_읽기_api_전부에서_보인다() {
    let (root, file) = 파일("read");
    let 옛 = 결박("Old", DecorBaseline::Unrecorded);
    옛_빌드가_쓴다(&file, &옛);

    let store = IntentStore::open_read_only(&file).expect("읽기로 연다");
    assert_eq!(store.get(&옛.id).expect("get"), Some(옛.clone()), "옛 표의 결박을 get 이 못 올렸다");
    assert_eq!(store.all().expect("all"), vec![옛.clone()]);
    assert_eq!(store.count().expect("count"), 1, "옛 표의 결박이 건수에서 빠졌다 — 조용한 유실이다");
    assert_eq!(store.bound_to(옛.target).expect("bound_to"), vec![옛.clone()]);
    let 올린 = store.get(&옛.id).expect("get").expect("있다");
    assert!(올린.watch.iter().all(|w| w.decor.unwatched()), "옛 판 결박에 장식 기준값을 지어냈다");
    drop(store);

    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn 새_쓰기는_새_표에_가고_같은_id_는_새_표가_이긴다() {
    let (root, file) = 파일("write");
    let 옛 = 결박("Old", DecorBaseline::Unrecorded);
    옛_빌드가_쓴다(&file, &옛);

    let store = IntentStore::open(&file).expect("쓰기로 연다");
    // ── 다른 결박을 새로 건다 — 건수가 둘이다(옛 것이 안 사라진다) ──────────────
    let 새 = 결박("New", DecorBaseline::Recorded(BodyDigest::of_normalized(b"#[derive(Debug)]")));
    store.record(&새).expect("새 결박");
    assert_eq!(store.count().expect("count"), 2, "새 결박을 쓰자 옛 결박이 건수에서 빠졌다");
    assert_eq!(store.get(&새.id).expect("get"), Some(새.clone()), "새 모양의 장식 기준값이 왕복하지 않는다");

    // ── 같은 id 를 새 모양으로 다시 쓴다 — 하나이고 새 값이 이긴다 ────────────────
    let mut 다시 = 옛.clone();
    다시.watch[0].decor = DecorBaseline::Recorded(BodyDigest::of_normalized(b"#[must_use]"));
    store.record(&다시).expect("재기록");
    assert_eq!(store.count().expect("count"), 2, "같은 id 가 두 표에 있어 두 번 셌다");
    assert_eq!(store.get(&옛.id).expect("get"), Some(다시.clone()), "새 표가 옛 표를 이기지 못했다");
    let 전부 = store.all().expect("all");
    assert_eq!(전부.len(), 2);
    assert!(전부.windows(2).all(|w| w[0].id < w[1].id), "결박 id 순이 아니다");

    // ── 내보내기는 판 3 이고 장식 기준값을 싣는다 ─────────────────────────────
    let jsonl = store.export_jsonl().expect("내보내기");
    assert!(jsonl.lines().next().is_some_and(|h| h.contains("\"schema_version\":3")), "내보내기 판이 3 이 아니다:\n{jsonl}");
    assert!(jsonl.contains("\"decor\":{\"recorded\""), "내보내기가 장식 기준값을 안 실었다:\n{jsonl}");
    drop(store);

    let _ = std::fs::remove_dir_all(&root);
}

/// **판 2 JSONL 의 감시 원소는 장식 기준값이 없는 채로 들어온다** — 지어내지 않는다.
#[test]
fn 판_2_jsonl_은_장식을_기록_안_됨으로_읽는다() {
    let (root, file) = 파일("jsonl2");
    let 원본 = 결박("Old", DecorBaseline::Recorded(BodyDigest::of_normalized(b"x")));
    let 쓰는곳 = IntentStore::open(&file).expect("열기");
    쓰는곳.record(&원본).expect("기록");
    let 판3 = 쓰는곳.export_jsonl().expect("내보내기");
    drop(쓰는곳);

    // 판 3 을 판 2 모양으로 — 머리의 판을 내리고 `decor` 를 지운다.
    let mut 줄들 = Vec::new();
    for line in 판3.lines() {
        let mut v: serde_json::Value = serde_json::from_str(line).expect("줄");
        if v["kind"] == "header" {
            v["schema_version"] = serde_json::json!(2);
        }
        if let Some(ws) = v.get_mut("watch").and_then(|w| w.as_array_mut()) {
            for w in ws {
                assert!(w.as_object_mut().expect("원소").remove("decor").is_some(), "판 3 이 decor 를 안 실었다");
            }
        }
        줄들.push(serde_json::to_string(&v).expect("직렬화"));
    }

    let (root2, file2) = 파일("jsonl2-in");
    let 받는곳 = IntentStore::open(&file2).expect("열기");
    let 보고 = 받는곳.import_jsonl(&줄들.join("\n")).expect("판 2 를 읽는다");
    assert_eq!(보고.schema_version, 2);
    let 들어온 = 받는곳.get(&원본.id).expect("get").expect("있다");
    assert_eq!(들어온.watch[0].digest, 원본.watch[0].digest, "본문 기준값이 안 왔다");
    assert_eq!(들어온.watch[0].decor, DecorBaseline::Unrecorded, "판 2 에 없는 장식 기준값을 지어냈다");
    drop(받는곳);

    let _ = std::fs::remove_dir_all(&root);
    let _ = std::fs::remove_dir_all(&root2);
}
