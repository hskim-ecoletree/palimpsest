//! 설치 검사 — `[f24]` ⑤ 의 **다섯**과, 훅이 실제로 도는지 보는 **여섯째**.
//!
//! # 여섯째가 왜 여기 붙는가
//!
//! ⑤ 가 등록한 다섯은 그대로 있고 하나가 **더해졌다.** ⑧ 이 *"등록된 훅 명령이
//! 발화한다"* 를 요구하는데, 「`settings.json` 에 적혀 있다」로는 그것을 못 보증한다 —
//! 실행 파일이 사라지면 exit **127**, 권한을 잃으면 exit **126** 이고 하네스는 그 실패를
//! **완전히 삼킨다.** 그래서 여섯째는 **등록된 명령을 실제로 실행해서** 대답을 본다.
//!
//! # 왜 이 다섯이 `pal doctor` 에 합류하는가
//!
//! **실측**: 깨진 `settings.json` 은 하네스의 `-p` 에서 **완전히 침묵한다**(exit 0 ·
//! stderr 0 바이트). 오직 `claude doctor` 와 대화형 다이얼로그만 말한다. 그래서
//! **우리 `doctor` 가 유일한 문이다.**
//!
//! # ★ 정상 fixture 에서 다섯이 전부 초록인가
//!
//! 이 줄이 없으면 **항상 빨간 `doctor`** 가 고장 다섯을 공짜로 통과한다. F04 가
//! `prune` 에서 쓴 것과 같은 형태의 대칭 검사다(*"★ 그런데 실제로 지우기는 했는가"*).
//!
//! # 검사할 수 없는 것은 `Residual`
//!
//! `pal doctor` 가 이미 쓰는 어휘이고 **새로 만들지 않는다.** *"검사하지 못한 것은
//! 「이상 없음」이 아니다."*

use std::path::{Path, PathBuf};

use serde::Serialize;

use super::inside::Root;
use super::layout::{DERIVED, MANIFEST, SETTINGS};
use super::{hooks, ignore, manifest, settings};

/// 검사 하나의 결말.
#[derive(Serialize)]
#[serde(tag = "outcome", content = "detail", rename_all = "snake_case")]
pub enum Outcome {
    /// 초록.
    Ok(String),
    /// 빨강 — **무엇이 왜.**
    Failed(String),
    /// **검사하지 못했다.** 「이상 없음」이 아니다.
    Residual(String),
}

impl Outcome {
    #[must_use]
    pub fn mark(&self) -> &'static str {
        match self {
            Self::Ok(_) => "ok  ",
            Self::Failed(_) => "빨강",
            Self::Residual(_) => "잔여",
        }
    }
    fn detail(&self) -> &str {
        match self {
            Self::Ok(s) | Self::Failed(s) | Self::Residual(s) => s,
        }
    }
}

#[derive(Serialize)]
pub struct Check {
    pub number: u8,
    pub name: &'static str,
    #[serde(flatten)]
    pub outcome: Outcome,
}

/// 센다.
#[must_use]
pub fn checks(target: &Path) -> Vec<Check> {
    let root = 설치_루트(target);
    vec![
        Check { number: 1, name: "설정 파일이 유효한 JSON 인가", outcome: 설정(target) },
        Check { number: 2, name: "매니페스트가 실물과 맞는가", outcome: 매니페스트(root.as_deref()) },
        Check { number: 3, name: "여기가 설치 루트인가", outcome: 루트(target, root.as_deref()) },
        Check { number: 4, name: "`pal` 실행 파일을 찾을 수 있는가", outcome: 실행_파일() },
        Check { number: 5, name: "`.gitignore` 에 파생이 등재됐는가", outcome: 등재(target) },
        Check { number: 6, name: "등록된 훅이 실제로 도는가", outcome: 훅(root.as_deref()) },
    ]
}

/// 이 자리에서 위로 올라가며 설치를 찾는다.
fn 설치_루트(from: &Path) -> Option<PathBuf> {
    let mut here = Some(from);
    while let Some(dir) = here {
        if dir.join(MANIFEST).is_file() {
            return Some(dir.to_path_buf());
        }
        here = dir.parent();
    }
    None
}

fn 설정(target: &Path) -> Outcome {
    let path = target.join(SETTINGS);
    if !path.exists() {
        return Outcome::Residual(format!("{SETTINGS} 가 없다 — 읽을 것이 없다"));
    }
    match settings::read(&path) {
        Ok(_) => Outcome::Ok(format!("{SETTINGS} 를 읽었다")),
        Err(e) => Outcome::Failed(format!("{e}")),
    }
}

fn 매니페스트(root: Option<&Path>) -> Outcome {
    let Some(root) = root else {
        return Outcome::Residual("설치를 찾지 못했다 — 대조할 상대가 없다".to_owned());
    };
    let m = match manifest::read(&root.join(MANIFEST)) {
        Ok(m) => m,
        Err(e) => return Outcome::Failed(format!("{e}")),
    };
    let 뿌리 = match Root::세운다(root) {
        Ok(r) => r,
        Err(e) => return Outcome::Failed(format!("{e}")),
    };
    let actual = match manifest::walk(&뿌리, &m.roots, &m.own_path) {
        Ok(a) => a,
        Err(e) => return Outcome::Failed(format!("실물을 훑지 못했다 — {e}")),
    };
    let d = manifest::diff(&m.files, &actual);
    if d.is_clean() {
        // ★ **사용자 수정은 고장이 아니다.** 그런데 「이상 없음」으로 뭉개지도 않는다 —
        // 무엇이 왜 다른지를 초록 안에서 말한다(`[f24]` ④ 의 *"밟지 않는 것과 말하지
        // 않는 것은 다르다"* 를 진단 쪽에도 세운다).
        if d.user_modified.is_empty() {
            return Outcome::Ok(format!("적힌 {}개가 실물과 sha256 까지 같다", m.files.len()));
        }
        return Outcome::Ok(format!(
            "적힌 {}개 중 {}개가 sha256 까지 같고, 나머지는 **사용자 수정**이다 (`update` 가 \
             밟지 않고 지나갔다): {}",
            m.files.len(),
            m.files.len() - d.user_modified.len(),
            d.user_modified.join(" · ")
        ));
    }
    let mut says = Vec::new();
    if !d.missing.is_empty() {
        says.push(format!("적혔는데 없다: {}", d.missing.join(" · ")));
    }
    if !d.unrecorded.is_empty() {
        says.push(format!("생겼는데 안 적혔다: {}", d.unrecorded.join(" · ")));
    }
    for (path, recorded, there) in &d.changed {
        says.push(format!("{path} 의 sha256 이 다르다 ({}… → {}…)", &recorded[..8], &there[..8]));
    }
    Outcome::Failed(says.join(" / "))
}

fn 루트(target: &Path, root: Option<&Path>) -> Outcome {
    match root {
        None => Outcome::Residual("설치를 찾지 못했다".to_owned()),
        Some(r) if r == target => Outcome::Ok("여기가 설치 루트다".to_owned()),
        Some(r) => Outcome::Failed(format!(
            "여기는 설치 루트가 아니다 — 설치는 {} 에 있다. 거기서 돌리십시오",
            r.display()
        )),
    }
}

fn 실행_파일() -> Outcome {
    // ⚠ **홈을 안 읽는다.** `PATH` 만 본다 — `[f24]` ⑦.
    let Some(path) = std::env::var_os("PATH") else {
        return Outcome::Residual("`PATH` 가 없다".to_owned());
    };
    for dir in std::env::split_paths(&path) {
        let candidate = dir.join("pal");
        if candidate.is_file() {
            return Outcome::Ok(format!("{}", candidate.display()));
        }
    }
    Outcome::Failed("`PATH` 어디에도 `pal` 이 없다 — 설치된 커맨드가 못 돈다".to_owned())
}

fn 등재(target: &Path) -> Outcome {
    let mut 빠진 = Vec::new();
    for path in DERIVED {
        match ignore::verdict(target, path) {
            Ok(ignore::Verdict::Covered) => {}
            Ok(ignore::Verdict::NotAWorktree) => {
                return Outcome::Residual("git worktree 가 아니다 — 등재를 물을 수 없다".to_owned());
            }
            Ok(ignore::Verdict::Revived { pattern }) => {
                빠진.push(format!("{path} (사용자가 `{pattern}` 로 되살렸다)"));
            }
            Ok(ignore::Verdict::Uncovered) => 빠진.push((*path).to_owned()),
            Err(e) => return Outcome::Residual(format!("git 에게 못 물었다 — {e}")),
        }
    }
    if 빠진.is_empty() {
        Outcome::Ok(format!("파생 {}개가 전부 등재됐다", DERIVED.len()))
    } else {
        Outcome::Failed(format!("등재 안 됨: {}", 빠진.join(" · ")))
    }
}

/// ★ **등록된 명령을 실제로 실행해서 대답을 본다.**
///
/// 두 겹이다 — 매니페스트가 적은 등록이 **설정에 그대로 있는가**, 그리고 그 문자열이
/// **실제로 도는가.** 첫째만 보면 실행 파일이 사라진 자리를 못 보고, 둘째만 보면
/// 사용자가 등록을 지운 자리를 못 본다.
fn 훅(root: Option<&Path>) -> Outcome {
    let Some(root) = root else {
        return Outcome::Residual("설치를 찾지 못했다 — 등록된 훅이 없다".to_owned());
    };
    let m = match manifest::read(&root.join(MANIFEST)) {
        Ok(m) => m,
        Err(e) => return Outcome::Failed(format!("{e}")),
    };
    let 적힌: &[manifest::HookEntry] = match &m.settings {
        Some(s) if !s.hooks.is_empty() => &s.hooks,
        _ => return Outcome::Residual("매니페스트에 등록된 훅이 없다".to_owned()),
    };

    let read = match settings::read(&root.join(SETTINGS)) {
        Ok(r) => r,
        Err(e) => return Outcome::Failed(format!("설정을 못 읽어 등록을 확인할 수 없다 — {e}")),
    };
    for h in 적힌 {
        if !hooks::registered(read.current.as_ref(), &h.event, &h.command) {
            return Outcome::Failed(format!(
                "{} 의 등록이 {SETTINGS} 에서 사라졌다 — `pal install` 을 다시 돌리십시오",
                h.event
            ));
        }
        if let Err(e) = hooks::probe(&h.event, &h.command) {
            return Outcome::Failed(format!("{} 이 안 돈다 — {e:#}", h.event));
        }
    }
    Outcome::Ok(format!("등록된 {}개가 실제로 돌고 우리 표식으로 대답했다", 적힌.len()))
}

/// 사람이 읽는 화면.
pub fn print(checks: &[Check]) {
    println!();
    println!("■ 설치 검사 {}개", checks.len());
    for c in checks {
        println!("  {} {}  {}", c.outcome.mark(), c.number, c.name);
        println!("      {}", c.outcome.detail());
    }
}
