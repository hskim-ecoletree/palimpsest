//! ditto·boxwood 표식 주석을 센다 — **회귀 관측용, 아무것도 안 쓴다.**
//! 사용: cargo run -q -p pal-extract --example count_marked -- <루트> <확장자…>
use std::path::Path;
const MARKERS: [&str; 2] = ["@decision:", "ADR-"];

fn walk(dir: &Path, exts: &[String], out: &mut Vec<std::path::PathBuf>) {
    let Ok(rd) = std::fs::read_dir(dir) else { return };
    for e in rd.flatten() {
        let p = e.path();
        let name = p.file_name().and_then(|s| s.to_str()).unwrap_or("");
        if name.starts_with('.') || name == "node_modules" || name == "target" { continue; }
        if p.is_dir() { walk(&p, exts, out); }
        else if p.extension().and_then(|s| s.to_str()).is_some_and(|x| exts.iter().any(|e| e == x)) {
            out.push(p);
        }
    }
}

fn main() {
    let mut a = std::env::args().skip(1);
    let root = a.next().expect("루트");
    let exts: Vec<String> = a.collect();
    let mut files = Vec::new();
    walk(Path::new(&root), &exts, &mut files);
    files.sort();
    let (mut n, mut attached, mut scanned) = (0usize, 0usize, 0usize);
    for f in &files {
        let Ok(src) = std::fs::read(f) else { continue };
        let Some(lang) = pal_core::Language::from_extension(
            f.extension().and_then(|s| s.to_str()).unwrap_or("")) else { continue };
        let pal_core::Capable::Present(e) = pal_extract::extractor_for(lang) else { continue };
        let Ok(cs) = e.marked_comments(&src, &MARKERS) else { continue };
        scanned += 1;
        n += cs.len();
        attached += cs.iter().filter(|c| c.attaches_to_byte.is_some()).count();
    }
    println!("파일 {scanned} · 표식 주석 {n} · 붙은 것 {attached}");
}
