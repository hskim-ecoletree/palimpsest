//! **승인 대기 목록** — `pal narrative` 가 만들고 `pal touch` 가 좌표마다 읽는다.
//!
//! # 왜 파일인가
//!
//! `pal touch` 는 인입을 안 돌린다 — 저장소 전체의 문서를 읽는 비용을 좌표 하나에 지우지
//! 않기 때문이다. 그런데 남의 저장소에 설치한 사용자는 후보 수백 건 중 **무엇을 승인할지**
//! 모른다. 그래서 인입이 끝날 때 「조각 → 가리키는 좌표들」을 남기고, `touch` 가 자기
//! 좌표를 가리키는 **승인 안 된** 조각을 싣는다.
//!
//! # 파생물이다 — 의도가 아니다
//!
//! 이 파일은 문서와 투영에서 **다시 계산된다.** 지워도 잃는 것이 없고(`pal narrative` 를
//! 다시 돌리면 다시 만들어진다), 사람이 한 일(승인·거부)은 여기 없고 의도 저장소에 있다(R-21).
//! 그래서 `.palimpsest/` 아래에 두고, 워킹트리 요약은 git 인덱스에 오른 파일만 보므로
//! 이 파일을 써도 스냅샷이 안 흔들린다.
//!
//! # 0 을 안 찍는다
//!
//! 목록이 없으면 **「아직 만들지 않았다」**, 다른 스냅샷에서 만들었으면 **「이 스냅샷의 것이
//! 아니다」** 다. 둘 다 「승인 대기 0 건」과 다른 답이다 — 섞으면 설치 직후 사용자가
//! 「이 좌표엔 결정이 없다」로 읽는다.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use pal_core::{Classification, EntityId, Proposal, SymbolId};
use pal_intent::IntentStore;
use serde::{Deserialize, Serialize};

/// 이 목록이 **어느 스냅샷의 것인가**를 가르는 열쇠.
///
/// ★ **화면 표기(`Display`)를 쓰지 않는다** (2026-09-13 · 시험 `b3` 가 잡았다). 화면 표기는
/// `repo@abc1234+worktree` 로 **워킹트리 요약을 버린다** — 추적 중인 파일을 고쳐도 같은
/// 문자열이라 옛 목록이 「이 스냅샷의 것」으로 실렸다. 직렬화 값은 `tree_digest` 를 싣는다.
///
/// # Errors
/// 직렬화가 실패하면.
pub fn 열쇠(snapshot: &pal_core::Snapshot) -> Result<String> {
    serde_json::to_string(snapshot).context("스냅샷을 열쇠로 못 만들었다")
}

/// 목록 파일의 자리.
fn 자리(repo: &Path) -> PathBuf {
    repo.join(".palimpsest/narrative-pending.json")
}

/// 파일에 적힌 목록.
#[derive(Debug, Serialize, Deserialize)]
struct 목록 {
    /// 이 목록을 만든 스냅샷 — 워킹트리면 트리 요약까지 든다.
    snapshot: String,
    entries: Vec<조각>,
}

/// 좌표 후보가 있는 문서 조각 하나.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct 조각 {
    item: EntityId,
    path: String,
    anchor: String,
    /// `bound` 또는 `candidates` — 인입의 분류다. **둘 다 승인 전이다.**
    class: String,
    targets: Vec<SymbolId>,
}

/// `pal narrative` 가 인입을 마친 뒤 부른다.
///
/// # Errors
/// 파일을 못 쓰면.
pub fn 쓴다(repo: &Path, snapshot: &str, proposals: &[Proposal]) -> Result<()> {
    let entries = proposals
        .iter()
        .filter_map(|p| {
            let class = match &p.class {
                Classification::Bound { .. } => "bound",
                Classification::Candidates { .. } => "candidates",
                Classification::Unbound => return None,
            };
            Some(조각 {
                item: p.item.clone(),
                path: p.fragment.path.as_str().to_owned(),
                anchor: p.fragment.anchor.clone(),
                class: class.to_owned(),
                targets: p.choices(),
            })
        })
        .collect();
    let file = 자리(repo);
    if let Some(dir) = file.parent() {
        std::fs::create_dir_all(dir).with_context(|| format!("{}", dir.display()))?;
    }
    let text = serde_json::to_string(&목록 { snapshot: snapshot.to_owned(), entries })?;
    std::fs::write(&file, text).with_context(|| format!("{}", file.display()))
}

/// 좌표 하나의 승인 대기 — 화면과 `--json` 이 함께 싣는다.
#[derive(Debug, Serialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum 대기 {
    /// `pal narrative` 가 목록을 만든 적이 없다.
    NotBuilt,
    /// 목록이 다른 스냅샷에서 만들어졌다 — 코드가 그 뒤에 바뀌었다.
    Stale,
    /// 이 좌표를 가리키는 승인 안 된 조각들.
    Present { entries: Vec<대기_조각> },
}

/// 화면에 실리는 조각 하나.
#[derive(Debug, Serialize)]
pub struct 대기_조각 {
    pub path: String,
    pub anchor: String,
    /// 이 조각이 가리키는 좌표 수 — 넓게 퍼진 조각은 이 좌표에 관한 것이 아닐 수 있다.
    pub spread: usize,
    pub class: String,
    /// 승인 명령이 부르는 개체 이름.
    pub approve_item: String,
    /// 승인 명령의 `--pick` — 이 좌표의 지목 문자열.
    pub approve_pick: String,
}

/// `pal touch` 가 찾은 좌표에 대해 부른다.
///
/// # Errors
/// 파일이 깨졌거나 의도 저장소를 못 읽으면.
pub fn 읽는다(repo: &Path, snapshot: &str, target: SymbolId, intent: &IntentStore) -> Result<대기> {
    let file = 자리(repo);
    let text = match std::fs::read_to_string(&file) {
        Ok(t) => t,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(대기::NotBuilt),
        Err(e) => return Err(e).with_context(|| format!("{}", file.display())),
    };
    let 목록: 목록 = serde_json::from_str(&text).with_context(|| format!("{} 이 깨졌다", file.display()))?;
    if 목록.snapshot != snapshot {
        return Ok(대기::Stale);
    }
    let 걸린: Vec<EntityId> = intent
        .bound_to(target)
        .context("의도 저장소를 읽지 못했다")?
        .into_iter()
        .map(|b| b.subject)
        .collect();
    let mut entries = Vec::new();
    for c in 목록.entries.into_iter().filter(|c| c.targets.contains(&target)) {
        // **이미 사람이 한 일은 뺀다** — 승인한 조각이 대기에 계속 나오면 그것이 거짓이다.
        if 걸린.contains(&c.item) || intent.refused(&c.item, target).context("거부 기록을 읽지 못했다")? {
            continue;
        }
        entries.push(대기_조각 {
            path: c.path,
            anchor: c.anchor,
            spread: c.targets.len(),
            class: c.class,
            approve_item: c.item.to_display(),
            approve_pick: target.short(),
        });
    }
    Ok(대기::Present { entries })
}

/// 사람 화면.
pub fn 화면(d: &대기) {
    match d {
        대기::NotBuilt => {
            println!("■ 승인 대기 — 이 좌표를 가리키는 문서 조각");
            println!("  아직 만들지 않았습니다 — `pal narrative` 를 먼저 돌리면 만들어집니다");
        }
        대기::Stale => {
            println!("■ 승인 대기 — 이 좌표를 가리키는 문서 조각");
            println!("  이 목록은 이 스냅샷의 것이 아닙니다 — `pal narrative` 를 다시 돌리십시오");
        }
        대기::Present { entries } => {
            println!("■ 승인 대기 — 이 좌표를 가리키는 문서 조각 ({})", entries.len());
            if entries.is_empty() {
                println!("  없습니다");
            }
            for e in entries {
                println!("  {} · {} · 후보 {}곳", e.path, e.anchor, e.spread);
                println!("    승인: pal narrative --approve {} --pick {}", e.approve_item, e.approve_pick);
            }
        }
    }
}
