//! **배치 커밋이 실제로 나뉘는가** — `[f05.2.pass]` ③ 의 앞 절.
//!
//! F05 §4: *"redb 쓰기 트랜잭션은 커밋마다 fsync 비용이 있다. 파일 1,000 개 단위로 묶어
//! 커밋한다. 중단되면 그 배치만 잃고 다시 스티칭한다."*
//!
//! # 이 시험이 없으면 「배치를 넣었다」는 주장뿐이다
//!
//! 파일이 배치 크기보다 적으면 배치가 **한 번도 안 나뉜다.** 그래서 배치 크기가 상수가
//! 아니라 **인자**이고, 여기서 낮춰서 커밋이 실제로 여럿인지 **센다.**
//!
//! 뒤 절(*"그런데도 부분 갱신이 안 보인다"*)은 `isolation.rs` 가 진다 — 그 시험을
//! 고치지 않는 것이 `[f05.2.pass]` ③ 의 등록 내용이다.

use pal_core::{
    BodyDigest, Discriminator, ExtractGrade, FileRow, IdentityGrade,
    LanguageId, RefCounts, ReferenceEdge, RepoId, RepoPath, Slot, Snapshot, Span, SymbolId,
    SymbolKind, SymbolNode, TreeRef,
};
use pal_store::{FileStitch, Projection};

fn 스냅샷() -> Snapshot {
    Snapshot::single(RepoId::new("r"), TreeRef::Committed(pal_core::ObjectName::from_bytes([9; 20])))
}

/// 파일 하나 — 심볼 둘과 그 사이의 엣지 하나.
fn 파일치(i: usize) -> FileStitch {
    let repo = RepoId::new("r");
    let path = RepoPath::new(format!("f{i}.ts"));
    let node = |name: &str, slot: u32| SymbolNode {
        id: SymbolId::compute(
            &repo,
            &path,
            &[],
            name,
            &Discriminator::new(SymbolKind::Function, slot),
        ),
        path: path.clone(),
        container: Vec::new(),
        name: name.to_owned(),
        kind: SymbolKind::Function,
        body: BodyDigest::of_normalized(name.as_bytes()),
        span: Span { byte_start: 0, byte_end: 1, line_start: 1, line_end: 1 },
        identity: IdentityGrade::Exact,
    };
    let a = node(&format!("a{i}"), 0);
    let b = node(&format!("b{i}"), 0);
    FileStitch {
        file: FileRow {
            path: path.clone(),
            language: LanguageId::new("TypeScript"),
            grade: ExtractGrade::L2,
            export_digest: Slot::Built(pal_core::ExportDigest::from_bytes([1; 32])),
            refs: Slot::Built(RefCounts { edges: 1, ..RefCounts::default() }),
        },
        exports: vec![(a.name.clone(), a.id)],
        edges: vec![ReferenceEdge { from: a.id, to: b.id, at: 스냅샷() }],
        symbols: vec![a, b],
    }
}

fn 방(tag: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("pal-f05-batch-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    dir.join("index.redb")
}

#[test]
fn 배치가_실제로_나뉜다() {
    let path = 방("split");
    let p = Projection::open(&path).expect("2층");
    let files: Vec<FileStitch> = (0..5).map(파일치).collect();

    let 나뉨 = p.stitch("s", &files, 2).expect("스티칭");
    // 파일 5 · 배치 2 → **배치 커밋 셋**(2+2+1). 그리고 무대 준비 1 + 교체 1.
    assert_eq!(나뉨.batch_commits, 3, "배치가 안 나뉘었다");
    assert_eq!(나뉨.commits, 5, "무대 준비와 교체를 안 셌다");
    assert_eq!(나뉨.files, 5);
    assert_eq!(나뉨.symbols, 10);
    assert_eq!(나뉨.edges, 5);

    // ── 음성 대조 — **배치를 안 나누면 커밋이 하나여야 한다.**
    // 늘 여럿이면 위의 3 이 배치의 증거가 아니다.
    let 안나뉨 = p.stitch("s", &files, 1_000).expect("스티칭");
    assert_eq!(안나뉨.batch_commits, 1, "배치 크기를 무시하고 나눴다");

    // 그리고 **값이 같다** — 배치 크기는 산출을 안 바꾼다.
    assert_eq!(안나뉨.files, 나뉨.files);
    assert_eq!(안나뉨.symbols, 나뉨.symbols);

    assert_eq!(p.file_count().expect("파일"), 5);
    assert_eq!(p.count().expect("심볼"), 10);
    assert_eq!(p.edge_count().expect("정방향"), 5);
    assert_eq!(p.reverse_edge_count().expect("역방향"), 5);
    assert_eq!(p.export_count().expect("내보내기"), 5);
    assert_eq!(p.built_for().expect("메타"), Some("s".to_owned()));
    // 끝났으면 무대가 없다 — **있으면 재구축 중이라고 답하게 된다.**
    assert!(!p.rebuilding().expect("무대"), "교체 뒤에도 무대가 남았다");

    let _ = std::fs::remove_dir_all(path.parent().expect("방"));
}

#[test]
fn 스티칭은_옛_세대를_남기지_않는다() {
    // **교체는 지우고 갈아 끼우는 것이다.** 남으면 *"재구축했는데 옛 값이 나오는"*
    // 상태가 되고, 그 순간 재구축 등가성이 성립하지 않는다.
    let path = 방("replace");
    let p = Projection::open(&path).expect("2층");
    let 다섯: Vec<FileStitch> = (0..5).map(파일치).collect();
    p.stitch("s1", &다섯, 2).expect("첫 회");
    assert_eq!(p.file_count().expect("파일"), 5);

    let 둘: Vec<FileStitch> = (0..2).map(파일치).collect();
    p.stitch("s2", &둘, 2).expect("둘째 회");
    assert_eq!(p.file_count().expect("파일"), 2, "옛 세대가 남았다");
    assert_eq!(p.count().expect("심볼"), 4, "옛 심볼이 남았다");
    assert_eq!(p.edge_count().expect("엣지"), 2, "옛 엣지가 남았다");
    assert_eq!(p.built_for().expect("메타"), Some("s2".to_owned()));

    let _ = std::fs::remove_dir_all(path.parent().expect("방"));
}

#[test]
fn 파일이_없어도_교체가_성립한다() {
    // 무대 자리를 미리 안 만들면 이름 바꾸기가 실패하고, 그러면 **빈 저장소에서
    // 첫 스티칭이 터진다.**
    let path = 방("empty");
    let p = Projection::open(&path).expect("2층");
    let r = p.stitch("s", &[], 1_000).expect("빈 스티칭");
    assert_eq!(r.batch_commits, 0);
    assert_eq!(p.file_count().expect("파일"), 0);
    assert!(!p.rebuilding().expect("무대"));

    let _ = std::fs::remove_dir_all(path.parent().expect("방"));
}
