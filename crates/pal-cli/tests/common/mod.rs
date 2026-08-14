//! 통합 시험이 함께 쓰는 저장소 하나.
//!
//! `dead_code` 를 끈다 — 시험 바이너리마다 쓰는 함수가 다르고, 안 쓰는 쪽에서
//! 경고가 나면 clippy 기준선이 시험을 추가할 때마다 움직인다.
#![allow(dead_code)]
//!
//! # 왜 실물 git 저장소인가
//!
//! 여기서 재는 것 둘(**캐시 폐기 격리** · **재구축 등가성**)은 **파일시스템 위의
//! 사실**이다. `pal cache prune` 이 `cache/` 밖을 안 건드리는지는 API 를 불러서
//! 알 수 없고 — 그것은 *"이 함수가 저 함수를 안 부른다"* 일 뿐이다 — **바이트를 떠서
//! 대야** 안다. 그래서 바이너리를 실제로 돌린다.
//!
//! # 시험마다 다른 방
//!
//! 같은 디렉터리를 돌려 쓰면 한 시험이 다른 시험의 캐시를 본다. F02-4 에서 병렬
//! 대조를 통째로 꺼뜨린 형태가 그것이고(`corpus/criteria.toml` `[f04].self_judged` ③),
//! **캐시 기능의 대조는 그 함정 위에 통째로 서 있다.**

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;

pub const PAL: &str = env!("CARGO_BIN_EXE_pal");

/// 저장소 하나를 세운다 — TypeScript 셋 · Kotlin 하나.
///
/// 이름은 **저장소 안에서 유일**해야 한다. `pal bind` 가 후보가 여럿이면 멈추고,
/// 그 멈춤은 이 시험이 재려는 것과 무관한 실패다.
pub fn 저장소(tag: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("pal-f04-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("임시 저장소");

    std::fs::write(root.join("alpha.ts"), "export class 알파 { 메서드() { const x = 1; return x } }\n")
        .expect("alpha.ts");
    std::fs::write(root.join("beta.ts"), "export function 베타(n: number) { return n + 1 }\n")
        .expect("beta.ts");
    // **Kotlin 쪽 이름만 ASCII 다.** 지금 문법 핀(`brokk @ acb9630`)이 비ASCII 식별자를
    // 선언으로 못 읽는다 — 이 세션의 발견이고 `docs/gates/F04.md` §발견 에 적혀 있다.
    // 여기서 고치지 않는다(범위 밖). **TypeScript 쪽 둘은 일부러 한글로 둔다** — 그쪽은
    // 읽으므로, 두 언어의 차이가 이 시험 안에 남는다.
    std::fs::write(root.join("gamma.kt"), "class Gamma { fun method() {} }\n").expect("gamma.kt");
    // **F05 가 더한 파일** — 파일 **안**의 참조가 실제로 생기는 자리다.
    // 앞의 셋에는 심볼→심볼 참조가 하나도 없고, 그러면 재구축 등가성이 엣지에 대해
    // **공짜로 통과한다**(`[f05.2.pass]` ④ 의 하한이 이것을 막는다).
    std::fs::write(
        root.join("delta.ts"),
        "export function 도움() { return 1 }\nexport function 부름() { return 도움() }\n",
    )
    .expect("delta.ts");

    git(&root, &["init", "-q", "."]);
    git(&root, &["add", "-A"]);
    git(&root, &["-c", "user.email=t@example.com", "-c", "user.name=t", "commit", "-qm", "첫 커밋"]);
    root
}

pub fn git(cwd: &Path, args: &[&str]) {
    let out = Command::new("git").args(args).current_dir(cwd).output().expect("git 을 못 돌렸다");
    assert!(out.status.success(), "git {args:?}: {}", String::from_utf8_lossy(&out.stderr));
}

/// `pal` 을 돌리고 표준출력을 낸다. **실패하면 멈춘다** — 조용한 성공이 없어야 한다.
pub fn pal(cwd: &Path, args: &[&str]) -> String {
    let out = Command::new(PAL).args(args).current_dir(cwd).output().expect("pal 을 못 돌렸다");
    assert!(
        out.status.success(),
        "pal {args:?}\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).expect("UTF-8")
}

/// 디렉터리 아래 파일 **전부**의 `(상대 경로, 바이트)`.
///
/// # 이름으로 세지 않는다
///
/// `intent.redb` · `index.redb` 를 이름으로 적으면 **다음에 생기는 파일이 빠진다.**
/// 검사가 낡는 방식이 그것이고, 낡은 검사는 통과한다.
pub fn 바이트_전부(root: &Path, 빼는_이름: &str) -> BTreeMap<String, Vec<u8>> {
    let mut out = BTreeMap::new();
    훑기(root, root, 빼는_이름, &mut out);
    out
}

fn 훑기(root: &Path, dir: &Path, 빼는_이름: &str, out: &mut BTreeMap<String, Vec<u8>>) {
    let Ok(read) = std::fs::read_dir(dir) else { return };
    for entry in read.flatten() {
        let path = entry.path();
        if path.file_name().is_some_and(|n| n == 빼는_이름) {
            continue;
        }
        if path.is_dir() {
            훑기(root, &path, 빼는_이름, out);
        } else if path.is_file() {
            let rel = path.strip_prefix(root).unwrap_or(&path).display().to_string();
            out.insert(rel, std::fs::read(&path).expect("읽기"));
        }
    }
}

/// 캐시 디렉터리 안의 엔트리 파일 수 — **격리 방은 빼고 센다.**
pub fn 캐시_엔트리_수(cache: &Path) -> usize {
    let mut n = 0;
    let Ok(shards) = std::fs::read_dir(cache) else { return 0 };
    for shard in shards.flatten() {
        let p = shard.path();
        if !p.is_dir() || p.file_name().is_some_and(|f| f == ".corrupt") {
            continue;
        }
        n += std::fs::read_dir(&p).into_iter().flatten().flatten().count();
    }
    n
}
