//! **스냅샷의 정의** — 회차 `2026-09-15-clean-uninstall` 완수 조건이 쓰는 단 하나의 자.
//!
//! - **워킹트리**: 저장소 루트 아래 모든 파일·디렉터리의 경로 · 종류 · 모드 · 바이트.
//!   `.git/` 은 빼되 **`.git/config` · `.git/info/exclude` 는 바이트로 넣는다** — 시험이 부른 git 이
//!   `.git/index` 를 고쳐 쓰므로 나머지는 뺀다.
//! - **트리**(HOME 용): 그 디렉터리 아래 전부. 빼는 것이 없다.
//!
//! 조건마다 자를 따로 짜면 경계가 조건마다 갈린다(조건 설계 평가 R2). 여기 한 자리에서만 뜬다.

use std::collections::BTreeMap;
use std::path::Path;

/// 한 자리의 모습.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum 항목 {
    /// 디렉터리 — 모드만 댄다.
    디렉터리 { 모드: u32 },
    /// 파일 — 모드와 바이트.
    파일 { 모드: u32, 바이트: Vec<u8> },
    /// 심링크 — 가리키는 곳.
    링크 { 대상: String },
}

/// 워킹트리 스냅샷.
#[must_use]
pub fn 워킹트리(root: &Path) -> BTreeMap<String, 항목> {
    let mut out = BTreeMap::new();
    훑기(root, root, true, &mut out);
    for 넣을 in [".git/config", ".git/info/exclude"] {
        let p = root.join(넣을);
        if let Ok(meta) = std::fs::symlink_metadata(&p) {
            if meta.is_file() {
                out.insert(넣을.to_owned(), 항목::파일 {
                    모드: 모드(&meta),
                    바이트: std::fs::read(&p).expect("스냅샷: 읽기"),
                });
            }
        }
    }
    out
}

/// 트리 스냅샷 — 빼는 것 없이 전부(HOME · 저장소 자리).
#[must_use]
pub fn 트리(root: &Path) -> BTreeMap<String, 항목> {
    let mut out = BTreeMap::new();
    if root.exists() {
        훑기(root, root, false, &mut out);
    }
    out
}

/// 두 스냅샷이 갈린 경로 — 한쪽에만 있거나 모습이 다른 것.
#[must_use]
pub fn 갈린_경로(앞: &BTreeMap<String, 항목>, 뒤: &BTreeMap<String, 항목>) -> Vec<String> {
    let mut out: Vec<String> = 앞
        .keys()
        .chain(뒤.keys())
        .filter(|k| 앞.get(*k) != 뒤.get(*k))
        .cloned()
        .collect();
    out.sort();
    out.dedup();
    out
}

fn 훑기(root: &Path, dir: &Path, git_빼기: bool, out: &mut BTreeMap<String, 항목>) {
    let mut 자식: Vec<_> = std::fs::read_dir(dir)
        .expect("스냅샷: 디렉터리 읽기")
        .map(|e| e.expect("스냅샷: 항목").path())
        .collect();
    자식.sort();
    for p in 자식 {
        let rel = p
            .strip_prefix(root)
            .expect("스냅샷: 상대 경로")
            .to_string_lossy()
            .replace('\\', "/");
        if git_빼기 && (rel == ".git" || rel.starts_with(".git/")) {
            continue;
        }
        let meta = std::fs::symlink_metadata(&p).expect("스냅샷: 메타");
        if meta.file_type().is_symlink() {
            let 대상 = std::fs::read_link(&p).expect("스냅샷: 링크").to_string_lossy().into_owned();
            out.insert(rel, 항목::링크 { 대상 });
        } else if meta.is_dir() {
            out.insert(rel, 항목::디렉터리 { 모드: 모드(&meta) });
            훑기(root, &p, git_빼기, out);
        } else {
            out.insert(rel, 항목::파일 { 모드: 모드(&meta), 바이트: std::fs::read(&p).expect("스냅샷: 읽기") });
        }
    }
}

#[cfg(unix)]
fn 모드(meta: &std::fs::Metadata) -> u32 {
    use std::os::unix::fs::PermissionsExt;
    meta.permissions().mode() & 0o7777
}

/// Windows 에는 유닉스 모드가 없다 — 같은 물음(「쓸 수 있나」)을 읽기 전용 표시로 댄다.
#[cfg(not(unix))]
fn 모드(meta: &std::fs::Metadata) -> u32 {
    u32::from(meta.permissions().readonly())
}
