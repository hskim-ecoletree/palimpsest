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

use super::inside::{Rel, Root};
use super::layout::{DERIVED, MANIFEST, SETTINGS};
use super::manifest::Manifest;
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

/// 이 자리에서 위로 올라가며 설치를 찾는다 — **경계를 넘지 않는다.**
///
/// # ★ `/` 까지 올라가면 `--repo` 가 경계가 아니다
///
/// 옛 탐색은 조상을 끝까지 훑었다. 그래서 조상 디렉터리에 매니페스트를 심어 두면
/// **아무 관계 없는 하위 디렉터리에서 `pal doctor --repo .` 를 돌려도** 그것을 찾았고,
/// 그 매니페스트가 적은 것을 진단이 그대로 믿었다. 매니페스트는 **남이 커밋해 보내는
/// 파일**이므로 그것을 어디까지 찾을지가 곧 신뢰 경계다.
fn 설치_루트(from: &Path) -> Option<PathBuf> {
    let 경계 = 경계(from);
    let mut here = Some(from);
    while let Some(dir) = here {
        if dir.join(MANIFEST).is_file() {
            return Some(dir.to_path_buf());
        }
        if dir == 경계 {
            return None;
        }
        here = dir.parent();
    }
    None
}

/// 탐색이 멈추는 자리 — **대상이 속한 worktree 의 뿌리**, 없으면 대상 자신.
///
/// 검사 3 이 *"하위 디렉터리에서 실행"* 을 지목하려면 위로 한 칸은 봐야 한다. 그
/// 「한 칸」의 상한을 프로젝트 경계로 못박는다 — 그 밖의 매니페스트는 **우리 대상의
/// 것이 아니다.**
fn 경계(from: &Path) -> PathBuf {
    let mut here = Some(from);
    while let Some(dir) = here {
        if dir.join(".git").exists() {
            return dir.to_path_buf();
        }
        here = dir.parent();
    }
    from.to_path_buf()
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
    // ★ **대조하기 전에 매니페스트가 우리 자리만 적었는지 본다.** 매니페스트는 남이
    // 커밋해 보낼 수 있는 파일이고, 진단이 그것을 그대로 믿으면 `uninstall` 이
    // 무엇을 할지도 화면에서 안 보인다.
    if let Err(e) = manifest::자리들(&뿌리, &m) {
        return Outcome::Failed(format!("{e:#}"));
    }
    let actual = match manifest::walk(&뿌리, &m.roots, &m.own_path) {
        Ok(a) => a,
        Err(e) => return Outcome::Failed(format!("실물을 훑지 못했다 — {e}")),
    };
    let d = manifest::diff(&m.files, &actual);
    // ★ **사각지대를 초록 안에서도 말한다.** 아래 [`남의_에이전트`] 를 볼 것.
    let 사각지대 = 남의_에이전트(&뿌리, &m);
    if d.is_clean() {
        // ★ **사용자 수정은 고장이 아니다.** 그런데 「이상 없음」으로 뭉개지도 않는다 —
        // 무엇이 왜 다른지를 초록 안에서 말한다(`[f24]` ④ 의 *"밟지 않는 것과 말하지
        // 않는 것은 다르다"* 를 진단 쪽에도 세운다).
        let mut 말 = if d.user_modified.is_empty() {
            format!("적힌 {}개가 실물과 sha256 까지 같다", m.files.len())
        } else {
            format!(
                "적힌 {}개가 전부 sha256 까지 같다. 그중 {}개는 **사용자 수정**이다 \
                 (`update` 가 밟지 않고 지나갔고, 그 시점의 sha 를 적어 두었다 — 그 뒤 또 \
                 바뀌면 여기가 빨개진다): {}",
                m.files.len(),
                d.user_modified.len(),
                d.user_modified.join(" · ")
            )
        };
        말.push_str(&사각지대);
        return Outcome::Ok(말);
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
    // ★ **「우리 것과 다르다」와 「무슨 내용인지 안 본다」는 다르다.** 사용자 수정으로
    // 적힌 자리라도 **우리가 마지막으로 본 뒤 또 바뀌면** 그 사실은 나온다.
    for (path, recorded, there) in &d.drifted {
        says.push(format!(
            "{path} 는 **사용자 수정**으로 적힌 자리인데 우리가 마지막으로 본 뒤 또 \
             바뀌었다 ({}… → {}…) — 사람이 또 고쳤으면 `pal update` 가 지금 내용을 \
             적는다",
            &recorded[..8],
            &there[..8]
        ));
    }
    Outcome::Failed(format!("{}{사각지대}", says.join(" / ")))
}

/// ★ **대조 밖에 무엇이 사는지 말한다** — `.claude/agents/` 는 **사각지대다.**
///
/// 그 디렉터리는 **남의 에이전트가 함께 사는 곳**이라 매니페스트가 그쪽만 「파일
/// 하나짜리 뿌리」로 잡는다([`manifest`] 머리말). 통째로 훑으면 남의 것을 우리 것으로
/// 세게 되므로 **그 설계는 그대로 둔다.**
///
/// 바꾸는 것은 **말하는가**뿐이다. 사각지대가 조용하면 사각지대인 줄 모르고, 그러면
/// 남이 그 자리에 무엇을 놓아도 진단은 초록만 낸다. 그 파일들은 하네스가 에이전트
/// 정의로 읽는 것들이다.
///
/// 판정은 **안 바꾼다** — 이 문장은 초록에도 빨강에도 똑같이 덧붙는다.
fn 남의_에이전트(뿌리: &Root, m: &Manifest) -> String {
    // 우리 것으로 적힌 자리는 뺀다. **이름을 손에 안 쥔다** — 매니페스트가 선언한
    // 뿌리에서 그 디렉터리를 유도한다.
    let mut 우리것: std::collections::BTreeSet<&str> = std::collections::BTreeSet::new();
    let mut 볼_곳: std::collections::BTreeSet<&str> = std::collections::BTreeSet::new();
    for rel in &m.roots.files {
        우리것.insert(rel.as_str());
        if let Some((dir, _)) = rel.as_str().rsplit_once('/') {
            볼_곳.insert(dir);
        }
    }
    let mut 남의것 = Vec::new();
    for dir in 볼_곳 {
        let Ok(path) = 뿌리.join(&Rel::new(dir)) else { continue };
        let Ok(entries) = std::fs::read_dir(&path) else { continue };
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().into_owned();
            // ⚠ **경로 구분자 가정**: `Rel` 은 언제나 `/` 로 갈린다.
            let rel = format!("{dir}/{name}");
            if !우리것.contains(rel.as_str()) {
                남의것.push(rel);
            }
        }
    }
    if 남의것.is_empty() {
        return String::new();
    }
    남의것.sort();
    format!(
        " / ★ **대조 밖**: 남의 에이전트가 함께 사는 자리라 이 검사가 안 보는 것이 \
         {}개 있다(설계대로다. 고장이 아니다): {}",
        남의것.len(),
        남의것.join(" · ")
    )
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

/// ★ **적힌 것은 대조하고, 실행은 우리가 아는 것만 한다.**
///
/// 세 겹이다 —
///
/// 1. 매니페스트가 적은 등록이 **설정에 그대로 있는가**(문자열 대조)
/// 2. 그 문자열이 **우리 형태인가**, 그리고 그 자리가 **실행될 수 있는가**(`stat`)
/// 3. **훅 규약이 실제로 서는가** — [`hooks::probe`] 가 **지금 도는 이 실행 파일**을
///    셸 없이 띄운다
///
/// ⚠ **2 에서 되읽은 경로를 3 이 안 쓴다.** 그 경로는 남이 커밋해 보낸 파일에서 왔고,
/// 옛 코드는 그것을 `/bin/sh -c` 로 돌려서 `pal doctor` 한 번이 임의 코드 실행이었다.
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
        let Some(등록된_자리) = hooks::되읽는다(&h.command, &h.event) else {
            return Outcome::Failed(format!(
                "{} 에 걸린 문자열이 우리 형태가 아니다 — **돌려보지 않는다.** \
                 매니페스트와 {SETTINGS} 는 대상 프로젝트 안의 평범한 파일이라 \
                 남이 커밋해 보낼 수 있다. `pal install` 을 다시 돌리십시오",
                h.event
            ));
        };
        if let Err(e) = hooks::실행할_수_있나(&등록된_자리) {
            return Outcome::Failed(format!("{} 이 안 돈다 — {e:#}", h.event));
        }
        if let Err(e) = hooks::probe(&h.event) {
            return Outcome::Failed(format!("{} 의 훅 규약이 안 선다 — {e:#}", h.event));
        }
    }
    Outcome::Ok(format!(
        "등록된 {}개가 설정과 맞고 그 자리가 실행될 수 있다. 훅 규약은 지금 이 실행 \
         파일로 확인했다 — **적힌 문자열은 안 돌린다**",
        적힌.len()
    ))
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
