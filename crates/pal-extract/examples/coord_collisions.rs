//! 좌표 충돌을 센다 — **R-16 이 「코퍼스로 세라」고 남긴 그 측정**.
//!
//! `(컨테이너 체인, 이름, 종류)` 가 겹치는 심볼은 `ordinal` 을 받고, 그러면
//! **선언 순서를 바꾸는 것만으로 좌표가 서로를 가리킨다.** 본문이 다르므로
//! `Orphaned` 가 아니라 평범한 `Stale` 로 위장한다.
//!
//! ⚠ **이 도구가 저장소에 사는 것이 요점이다.** 앞서 같은 수를 격리 스파이크로
//! 재고 안 남겼더니 판정·이슈에 실린 값이 **어떤 모델로도 재현되지 않았다**
//! (독립 리뷰 R3). **세는 자리는 하나다.**
//!
//! 사용: cargo run -q -p pal-extract --example coord_collisions -- <루트> <확장자…> [--제외 <조각>]
//!
//! `--제외` 는 경로에 그 조각이 들어가면 뺀다 — cargo 코퍼스의 `tests/testsuite/`
//! 992 파일이 그 자리다(손 표본 모집단이 그것을 뺀 380 이다).
use std::collections::HashMap;
use std::path::{Path, PathBuf};

fn walk(dir: &Path, exts: &[String], 제외: &[String], out: &mut Vec<PathBuf>) {
    let Ok(rd) = std::fs::read_dir(dir) else { return };
    for e in rd.flatten() {
        let p = e.path();
        let name = p.file_name().and_then(|s| s.to_str()).unwrap_or("");
        if name.starts_with('.') || name == "node_modules" || name == "target" {
            continue;
        }
        let 경로 = p.to_string_lossy().replace('\\', "/");
        if 제외.iter().any(|x| 경로.contains(x.as_str())) {
            continue;
        }
        if p.is_dir() {
            walk(&p, exts, 제외, out);
        } else if p.extension().and_then(|s| s.to_str()).is_some_and(|x| exts.iter().any(|e| e == x))
        {
            out.push(p);
        }
    }
}

/// 컨테이너 체인 — `contains` 를 거슬러 조상 이름을 `.` 로 잇는다.
fn 체인(g: &pal_core::FileGraph, ix: usize, memo: &mut HashMap<usize, String>) -> String {
    if let Some(v) = memo.get(&ix) {
        return v.clone();
    }
    let 부모 = g.contains.iter().find(|c| c.child.0 as usize == ix).map(|c| c.parent.0 as usize);
    let v = match 부모 {
        None => "-".to_owned(),
        Some(p) => {
            let 위 = 체인(g, p, memo);
            let 이름 = &g.symbols[p].name;
            if 위 == "-" { 이름.clone() } else { format!("{위}.{이름}") }
        }
    };
    memo.insert(ix, v.clone());
    v
}

fn main() {
    let mut a = std::env::args().skip(1);
    let root = a.next().expect("루트");
    let (mut exts, mut 제외, mut 제외_모드) = (Vec::new(), Vec::new(), false);
    for x in a {
        if x == "--제외" {
            제외_모드 = true;
        } else if 제외_모드 {
            제외.push(x);
        } else {
            exts.push(x);
        }
    }
    assert!(!exts.is_empty(), "확장자를 하나 이상 줘라 — 빈 모집단은 0 이 아니라 「안 봤다」다");

    let mut files = Vec::new();
    walk(Path::new(&root), &exts, &제외, &mut files);
    files.sort();

    let (mut 파일, mut 선언, mut 충돌_심볼, mut 충돌_열쇠) = (0usize, 0usize, 0usize, 0usize);
    let (mut 접음_충돌, mut 접음_열쇠) = (0usize, 0usize);
    for f in &files {
        let Ok(src) = std::fs::read(f) else { continue };
        let Some(lang) = pal_core::Language::from_extension(
            f.extension().and_then(|s| s.to_str()).unwrap_or("").to_ascii_lowercase().as_str(),
        ) else {
            continue;
        };
        let pal_core::Capable::Present(e) = pal_extract::extractor_for(lang) else { continue };
        let Ok(g) = e.extract(&src) else { continue };
        파일 += 1;
        선언 += g.symbols.len();

        let mut memo = HashMap::new();
        // ① 지금의 `SymbolKind` — 이 회차가 일곱을 더한 상태
        let mut 셈: HashMap<(String, String, &'static str), usize> = HashMap::new();
        // ② 접은 모델 — struct/enum/trait/union 을 전부 `class` 로
        let mut 접음: HashMap<(String, String, &'static str), usize> = HashMap::new();
        for (i, s) in g.symbols.iter().enumerate() {
            let c = 체인(&g, i, &mut memo);
            *셈.entry((c.clone(), s.name.clone(), s.kind.name())).or_default() += 1;
            let 접은종류 = match s.kind {
                pal_core::SymbolKind::Struct
                | pal_core::SymbolKind::Enum
                | pal_core::SymbolKind::Trait
                | pal_core::SymbolKind::Union => "class",
                k => k.name(),
            };
            *접음.entry((c, s.name.clone(), 접은종류)).or_default() += 1;
        }
        for (m, (열쇠, 심볼)) in
            [(&셈, (&mut 충돌_열쇠, &mut 충돌_심볼)), (&접음, (&mut 접음_열쇠, &mut 접음_충돌))]
        {
            for n in m.values().filter(|n| **n > 1) {
                *열쇠 += 1;
                *심볼 += *n;
            }
        }
    }
    let 비율 = |x: usize| if 선언 == 0 { 0.0 } else { x as f64 * 100.0 / 선언 as f64 };
    println!("파일 {파일} · 선언 {선언}");
    println!(
        "지금 종류    충돌 열쇠 {충돌_열쇠} · 충돌에 낀 심볼 {충돌_심볼} ({:.1}%)",
        비율(충돌_심볼)
    );
    println!(
        "접은 모델    충돌 열쇠 {접음_열쇠} · 충돌에 낀 심볼 {접음_충돌} ({:.1}%)",
        비율(접음_충돌)
    );
}
