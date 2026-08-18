//! **캐시 위생** — 자라기만 하던 자리 둘에 처분이 생겼는가 (`[f05.5]`).
//!
//! 옛 F04 §12 가 둘을 F05 로 넘겼다: 격리 방(`cache/.corrupt/`)이 무한히 자란다 ·
//! 죽은 `.tmp` 를 아무도 안 지운다.
//!
//! # 이 조각이 지는 것은 「지운다」가 아니라 「지울 수 있게 하되 기본은 안 지운다」다
//!
//! 격리된 바이트는 **결함의 증거**이고 `.tmp` 는 **지금 도는 쓰기일 수 있다.**
//! 기본으로 지우면 ① 증거가 사라지고 ② 그 쓰기의 `rename` 이 깨진다.
//! **둘 다 되돌릴 수 없는 쪽이다.**

mod common;

use common::{pal, 저장소};
use std::path::Path;

fn 격리_파일(cache: &Path) -> Vec<std::path::PathBuf> {
    let dir = cache.join(".corrupt");
    let Ok(read) = std::fs::read_dir(&dir) else { return Vec::new() };
    read.flatten().map(|e| e.path()).filter(|p| p.is_file()).collect()
}

fn 임시_파일(cache: &Path) -> Vec<std::path::PathBuf> {
    let Ok(shards) = std::fs::read_dir(cache) else { return Vec::new() };
    let mut out = Vec::new();
    for shard in shards.flatten() {
        let p = shard.path();
        if !p.is_dir() || p.file_name().is_some_and(|n| n == ".corrupt") {
            continue;
        }
        for f in std::fs::read_dir(&p).into_iter().flatten().flatten() {
            if f.path().extension().is_some_and(|e| e == "tmp") {
                out.push(f.path());
            }
        }
    }
    out
}

/// 캐시 엔트리 `n` 개를 실제로 망가뜨려 **격리시킨다.**
fn 격리시킨다(repo: &Path, cache: &Path, n: usize) {
    let mut 깬것 = 0usize;
    'outer: for shard in std::fs::read_dir(cache).expect("캐시").flatten() {
        let p = shard.path();
        if !p.is_dir() || p.file_name().is_some_and(|x| x == ".corrupt") {
            continue;
        }
        for f in std::fs::read_dir(&p).expect("샤드").flatten() {
            if f.path().extension().is_some_and(|e| e == "tmp") {
                continue;
            }
            std::fs::write(f.path(), b"\x00\x01\x02 not zstd").expect("망가뜨리기");
            깬것 += 1;
            if 깬것 == n {
                break 'outer;
            }
        }
    }
    // **변형이 실제로 먹었는지 확인한다** — 대조가 꺼지는 열한 형태 중 하나다.
    assert_eq!(깬것, n, "망가뜨릴 캐시 파일이 {깬것} 개뿐이었다");
    // 다음 회차가 그것을 격리한다.
    let 대장 = pal(repo, &["ledger", ".", "--cache-dir", cache.to_str().expect("경로"), "--json"]);
    let v: serde_json::Value = serde_json::from_str(&대장).expect("대장 JSON");
    assert_eq!(v["cache"]["corrupt"].as_u64().expect("corrupt"), n as u64, "격리가 안 일어났다");
}

#[test]
fn 격리_방에_처분이_생기고_기본은_안_지운다() {
    let repo = 저장소("hygiene-q");
    let cache = repo.join("c");
    let c = cache.to_str().expect("경로");
    pal(&repo, &["ledger", ".", "--cache-dir", c, "--json"]);
    격리시킨다(&repo, &cache, 2);

    // **하한이다** — 격리 방이 비어 있으면 「줄었다」를 못 잰다.
    let 전 = 격리_파일(&cache);
    assert_eq!(전.len(), 2, "격리된 것이 둘이 아니다");
    let 격리_바이트: u64 = 전.iter().map(|p| std::fs::metadata(p).expect("크기").len()).sum();

    // ★ **손잡이를 안 주면 한 바이트도 안 지운다.**
    pal(&repo, &["cache", "prune", "--cache-dir", c, "--budget", "0"]);
    assert_eq!(
        격리_파일(&cache).len(),
        2,
        "`--sweep-quarantine` 없이 격리 방이 줄었다 — 결함의 증거가 사라졌다"
    );

    // 주면 줄어든다.
    let 회계 = pal(&repo, &[
        "cache", "prune", "--cache-dir", c, "--sweep-quarantine", "0", "--json",
    ]);
    let v: serde_json::Value = serde_json::from_str(&회계).expect("회계 JSON");
    assert_eq!(v["quarantine"]["scanned"], 2);
    assert_eq!(v["quarantine"]["removed"], 2);
    assert!(v["quarantine"]["freed_bytes"].as_u64().expect("freed") > 0);
    assert!(격리_파일(&cache).is_empty(), "격리 방이 안 줄었다");
    let _ = 격리_바이트;

    let _ = std::fs::remove_dir_all(&repo);
}

#[test]
fn 죽은_임시_파일만_지운다_어린_것은_안_지운다() {
    // ★ `[f05.5.pass]` ② — **나이가 「도는 쓰기」와 가르는 유일한 값이다.**
    let repo = 저장소("hygiene-t");
    let cache = repo.join("c");
    let c = cache.to_str().expect("경로");
    pal(&repo, &["ledger", ".", "--cache-dir", c, "--json"]);

    // 샤드 하나에 `.tmp` 를 둘 만든다 — **방금 만든 것이다.**
    let shard = std::fs::read_dir(&cache)
        .expect("캐시")
        .flatten()
        .map(|e| e.path())
        .find(|p| p.is_dir() && p.file_name().is_some_and(|n| n != ".corrupt"))
        .expect("샤드");
    std::fs::write(shard.join("도는-쓰기.tmp"), b"abc").expect("tmp");
    std::fs::write(shard.join("또-하나.tmp"), b"defg").expect("tmp");
    assert_eq!(임시_파일(&cache).len(), 2, "`.tmp` 가 둘이 아니다");

    // ★ 손잡이를 안 주면 한 개도 안 지운다.
    pal(&repo, &["cache", "prune", "--cache-dir", c, "--budget", "999999999"]);
    assert_eq!(임시_파일(&cache).len(), 2, "`--sweep-stray` 없이 `.tmp` 가 지워졌다");

    // ★ **주더라도 어린 것은 안 지운다.** 나이를 넉넉히 주면 방금 만든 둘이 남는다.
    let 회계 = pal(&repo, &[
        "cache", "prune", "--cache-dir", c, "--sweep-stray", "--stray-age", "3600", "--json",
    ]);
    let v: serde_json::Value = serde_json::from_str(&회계).expect("회계 JSON");
    assert_eq!(v["stray"]["scanned"], 2);
    assert_eq!(v["stray"]["removed"], 0, "방금 만든 `.tmp` 를 지웠다 — 도는 쓰기가 깨진다");
    assert_eq!(v["stray"]["too_young"], 2);
    assert_eq!(임시_파일(&cache).len(), 2);

    // 나이를 0 으로 낮추면 지운다. **상수였으면 이 시험이 한 시간을 기다린다.**
    let 회계 = pal(&repo, &[
        "cache", "prune", "--cache-dir", c, "--sweep-stray", "--stray-age", "0", "--json",
    ]);
    let v: serde_json::Value = serde_json::from_str(&회계).expect("회계 JSON");
    assert_eq!(v["stray"]["removed"], 2, "나이를 0 으로 줬는데 안 지웠다");
    assert_eq!(v["stray"]["freed_bytes"], 7);
    assert!(임시_파일(&cache).is_empty());

    // 그리고 **엔트리는 안 건드렸다** — `.tmp` 청소가 캐시를 지우면 안 된다.
    let 남은 = common::캐시_엔트리_수(&cache);
    assert!(남은 > 0, "`.tmp` 청소가 엔트리까지 지웠다");

    let _ = std::fs::remove_dir_all(&repo);
}
