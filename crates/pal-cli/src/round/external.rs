//! **프로젝트 밖 저장소의 이 프로젝트 몫** — uninstall 이 부르는 단일 접점.
//!
//! 승인 · 종료 봉인 · Stop 활성화 · 진행 파일은 저장소 **밖**(`approval::store_location`)에 있다.
//! 그 자리를 무엇으로 가르고 무엇을 걷는지는 이 모듈 한 자리가 진다 — `install.rs` 는 이것을 부르고
//! 보고를 출력할 뿐이다(회차 `2026-09-15-clean-uninstall` 계획 1 · 4 · 5).
//!
//! # 계약
//!
//! - uninstall 은 **안의 정본을 지우기 전에** 이것을 부른다. `--purge` 가 봉인을 가르려면 회차 원장이
//!   아직 워킹트리에 있어야 한다.
//! - `worktree_거부` 가 비어 있지 않으면 **밖의 기록을 하나도 안 건드렸다**는 뜻이다.
//!
//! # 가르는 규칙
//!
//! | 기록 | 이 프로젝트 몫인가 |
//! |---|---|
//! | `round-stop-activation-<식별자>.json` | 이름의 식별자가 이 프로젝트면 |
//! | `round-stop-progress-<d>.{json,lock,project}` | 표시 파일 `<d>.project` 가 있으면 그 식별자로 · 없으면 `d` 가 이 체크아웃의 회차 slug 로 만든 활성화 digest 이면 · 둘 다 아니면 **가를 수 없다** |
//! | `<digest>.{json,project}`(명령 승인 · 종료 봉인) | 표시 파일이 있으면 그 식별자로 · 없으면 이 체크아웃의 `rounds/*/verification.log` checkpoint 에 적힌 봉인이면 · 둘 다 아니면 **가를 수 없다** |
//!
//! 표시 파일이 다른 식별자를 말하면 남의 기록이다 — 세지도 걷지도 않는다.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use pal_git::GixRepo;

use super::approval::{self, ProjectMarker};
use super::stop;

/// 어디까지 걷나.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum 걷기 {
    /// 운영 상태(Stop 활성화 · 진행 파일 · 잠금)만 걷는다. 승인 · 봉인은 정본이라 남긴다.
    기본,
    /// `--purge` — 이 프로젝트의 승인 · 봉인까지 걷는다. 추적 중인 회차 원장에 적힌 봉인은 남긴다.
    // ⚠ 만드는 자리(`uninstall --purge`)가 아직 없다 — 그 플래그를 세우는 커밋이 이 줄을 지운다.
    #[allow(dead_code)]
    전부,
}

/// 밖에서 한 일과 안 한 일. uninstall 이 이것을 그대로 출력한다.
#[derive(Debug, Default)]
pub struct 밖의_보고 {
    /// 들여다본 저장소 자리. 자리를 정하지 못했으면 `None`.
    pub 들여다본_자리: Option<PathBuf>,
    /// 지운 파일 · 디렉터리.
    pub 지운: Vec<PathBuf>,
    /// 이 프로젝트 몫인데 남긴 파일과 그 까닭.
    pub 남긴: Vec<(PathBuf, String)>,
    /// 어느 프로젝트 몫인지 가를 수 없어 안 지운 파일.
    pub 가를_수_없는: Vec<PathBuf>,
    /// 같은 저장소의 다른 git worktree. 비어 있지 않으면 밖을 하나도 안 건드렸다.
    pub worktree_거부: Vec<PathBuf>,
    /// 같은 원격을 쓰는 다른 클론의 기록도 함께 지웠다(`--purge` · origin 식별자).
    pub 클론_경고: bool,
    /// 이 프로젝트의 Stop 활성화를 걷었다 — 같은 원격의 다른 체크아웃도 Stop 정책이 비활성화된다.
    pub 다른_체크아웃_stop_비활성화: bool,
}

/// 이 프로젝트 몫을 `방식` 만큼 걷는다.
///
/// # Errors
/// 저장소 자리를 못 정하거나, 회차 원장 · worktree 목록을 못 읽거나, 파일을 못 지우면.
pub fn 걷는다(repo: &Path, 방식: 걷기) -> Result<밖의_보고> {
    let store = approval::store_location(None).map_err(anyhow::Error::from)?;
    걷는다_자리에서(repo, &store, 방식)
}

/// 기록 하나의 몫.
enum 몫 {
    이_프로젝트,
    남의,
    가를_수_없다,
}

/// 명령 승인인가 종료 봉인인가 — 남기는 까닭이 다르다.
enum 정본 {
    승인,
    봉인 { slug: String, 추적: bool },
}

fn 걷는다_자리에서(repo: &Path, store: &Path, 방식: 걷기) -> Result<밖의_보고> {
    let mut 보고 = 밖의_보고 {
        들여다본_자리: Some(store.to_path_buf()),
        ..밖의_보고::default()
    };
    let repo = repo
        .canonicalize()
        .with_context(|| format!("repo `{}`", repo.display()))?;
    // 식별자가 없는 자리(git 저장소가 아니거나 커밋이 없다)는 밖에 이 프로젝트 몫을 쓴 적이 없다.
    let Ok(git) = GixRepo::open(&repo) else {
        return Ok(보고);
    };
    let Ok(project) = git.stable_repository_identity() else {
        return Ok(보고);
    };
    // ★ 소유자 승격 5 — 같은 저장소의 다른 worktree 가 있으면 기본이든 전부든 밖을 하나도 안 건드린다.
    보고.worktree_거부 = git
        .other_worktrees()
        .map_err(anyhow::Error::from)
        .context("같은 저장소의 다른 git worktree 를 확인하지 못했다")?;
    if !보고.worktree_거부.is_empty() || !store.is_dir() {
        return Ok(보고);
    }

    let (slugs, 봉인들) = 체크아웃의_회차(&repo, &git)?;
    let 이_프로젝트_진행: BTreeSet<String> = slugs
        .iter()
        .map(|slug| stop::activation_digest(&project, slug))
        .collect();
    let 활성화 = stop::activation_path(store, &project);

    let mut 지울 = Vec::new();
    for name in 파일_이름들(store)? {
        let path = store.join(&name);
        if path == 활성화 {
            보고.다른_체크아웃_stop_비활성화 = true;
            지울.push(path);
        } else if let Some((digest, _)) = name
            .strip_prefix(stop::PROGRESS_PREFIX)
            .and_then(|rest| rest.split_once('.'))
            .filter(|(digest, ext)| is_hex(digest) && matches!(*ext, "json" | "lock" | "project"))
        {
            let record = stop::progress_path(store, digest);
            match 표시로_가른다(&record, &project) {
                Some(몫::이_프로젝트) => 지울.push(path),
                Some(몫::남의) => {}
                None if 이_프로젝트_진행.contains(digest) => 지울.push(path),
                Some(몫::가를_수_없다) | None => 보고.가를_수_없는.push(path),
            }
        } else if let Some((digest, _)) = name
            .split_once('.')
            .filter(|(digest, ext)| digest.len() == 64 && is_hex(digest) && matches!(*ext, "json" | "project"))
        {
            let record = store.join(format!("{digest}.json"));
            let 몫 = match 표시로_가른다(&record, &project) {
                Some(몫) => 몫,
                None if 봉인들.contains_key(digest) => 몫::이_프로젝트,
                None => 몫::가를_수_없다,
            };
            match 몫 {
                몫::남의 => {}
                몫::가를_수_없다 => 보고.가를_수_없는.push(path),
                몫::이_프로젝트 => {
                    let 종류 = 봉인들.get(digest).map_or(정본::승인, |(slug, 추적)| 정본::봉인 {
                        slug: slug.clone(),
                        추적: *추적,
                    });
                    match (방식, 종류) {
                        (걷기::전부, 정본::봉인 { slug, 추적: true }) => 보고.남긴.push((
                            path,
                            format!(
                                "종료 봉인 — git 이 추적 중인 회차 원장 `.palimpsest/rounds/{slug}/verification.log` 에 적혀 있어 남긴다"
                            ),
                        )),
                        (걷기::전부, _) => 지울.push(path),
                        (걷기::기본, 정본::승인) => 보고.남긴.push((
                            path,
                            "명령 승인 — 정본이라 기본 uninstall 은 남긴다 · `pal uninstall --purge` 가 걷는다".to_owned(),
                        )),
                        (걷기::기본, 정본::봉인 { .. }) => 보고.남긴.push((
                            path,
                            "종료 봉인 — 정본이라 기본 uninstall 은 남긴다 · `pal uninstall --purge` 가 걷는다".to_owned(),
                        )),
                    }
                }
            }
        }
    }

    // 표시 파일은 마지막에 지운다 — 중간에 멈추면 남은 기록이 여전히 갈린다.
    지울.sort_by_key(|path| {
        path.extension()
            .is_some_and(|ext| ext == approval::PROJECT_MARKER_EXTENSION)
    });
    for path in 지울 {
        match std::fs::remove_file(&path) {
            Ok(()) => 보고.지운.push(path),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => {
                return Err(error).with_context(|| format!("밖의 기록을 지우지 못했다: {}", path.display()));
            }
        }
    }
    보고.클론_경고 = 방식 == 걷기::전부 && git.has_origin() && !보고.지운.is_empty();
    빈_자리를_걷는다(store, &mut 보고)?;
    Ok(보고)
}

/// 표시 파일이 있으면 그것으로 가른다. 없으면 `None` — 부르는 쪽이 원장 · slug 로 가른다.
fn 표시로_가른다(record: &Path, project: &str) -> Option<몫> {
    match approval::read_project_marker(record) {
        ProjectMarker::Absent => None,
        ProjectMarker::Project(owner) if owner == project => Some(몫::이_프로젝트),
        ProjectMarker::Project(_) => Some(몫::남의),
        ProjectMarker::Unreadable => Some(몫::가를_수_없다),
    }
}

/// 이 체크아웃의 회차 slug 와, 원장 checkpoint 에 적힌 봉인 → (slug, 그 원장을 git 이 추적 중인가).
type 봉인_목록 = BTreeMap<String, (String, bool)>;

fn 체크아웃의_회차(repo: &Path, git: &GixRepo) -> Result<(Vec<String>, 봉인_목록)> {
    let rounds = repo.join(".palimpsest/rounds");
    let mut slugs = Vec::new();
    let mut 봉인들 = 봉인_목록::new();
    let entries = match std::fs::read_dir(&rounds) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok((slugs, 봉인들)),
        Err(error) => return Err(error).with_context(|| format!("{}", rounds.display())),
    };
    for entry in entries {
        let entry = entry.with_context(|| format!("{}", rounds.display()))?;
        let Ok(slug) = entry.file_name().into_string() else { continue };
        if !entry.file_type().is_ok_and(|kind| kind.is_dir()) {
            continue;
        }
        let ledger = entry.path().join("verification.log");
        if std::fs::symlink_metadata(&ledger).is_ok_and(|meta| meta.is_file()) {
            let text = std::fs::read(&ledger)
                .with_context(|| format!("회차 원장을 읽지 못했다: {}", ledger.display()))?;
            let 추적 = git
                .is_tracked(&format!(".palimpsest/rounds/{slug}/verification.log"))
                .map_err(anyhow::Error::from)?;
            for line in String::from_utf8_lossy(&text).lines() {
                let Ok(event) = serde_json::from_str::<serde_json::Value>(line) else { continue };
                if event["kind"] != "checkpoint" {
                    continue;
                }
                if let Some(seal) = event["finalization_seal"].as_str() {
                    let 자리 = 봉인들.entry(seal.to_owned()).or_insert((slug.clone(), false));
                    자리.1 |= 추적;
                }
            }
        }
        slugs.push(slug);
    }
    slugs.sort();
    Ok((slugs, 봉인들))
}

fn 파일_이름들(store: &Path) -> Result<Vec<String>> {
    let mut names = Vec::new();
    for entry in std::fs::read_dir(store).with_context(|| format!("{}", store.display()))? {
        let entry = entry.with_context(|| format!("{}", store.display()))?;
        if entry.file_type().is_ok_and(|kind| kind.is_dir()) {
            continue;
        }
        if let Ok(name) = entry.file_name().into_string() {
            names.push(name);
        }
    }
    names.sort();
    Ok(names)
}

/// 걷은 뒤 비면 `approvals` · `palimpsest` 와 **적힌 조상만** 지운다 — 미리 있던 빈 조상은 남긴다.
fn 빈_자리를_걷는다(store: &Path, 보고: &mut 밖의_보고) -> Result<()> {
    let Some(root) = approval::store_root(store) else {
        return Ok(());
    };
    if !비었다(store)? {
        return Ok(());
    }
    remove_dir(store, 보고)?;
    let 남은: Vec<String> = std::fs::read_dir(root)
        .with_context(|| format!("{}", root.display()))?
        .filter_map(|entry| entry.ok().and_then(|entry| entry.file_name().into_string().ok()))
        .collect();
    if 남은.iter().any(|name| name != approval::CREATED_ANCESTORS) {
        return Ok(());
    }
    let 조상 = approval::created_ancestors(root);
    if !남은.is_empty() {
        let record = root.join(approval::CREATED_ANCESTORS);
        std::fs::remove_file(&record).with_context(|| format!("{}", record.display()))?;
        보고.지운.push(record);
    }
    remove_dir(root, 보고)?;
    let mut cursor = root.parent();
    for name in 조상 {
        let Some(dir) = cursor else { break };
        if dir.file_name().is_none_or(|actual| actual.to_string_lossy() != name) || !비었다(dir)? {
            break;
        }
        remove_dir(dir, 보고)?;
        cursor = dir.parent();
    }
    Ok(())
}

fn 비었다(dir: &Path) -> Result<bool> {
    Ok(std::fs::read_dir(dir)
        .with_context(|| format!("{}", dir.display()))?
        .next()
        .is_none())
}

fn remove_dir(dir: &Path, 보고: &mut 밖의_보고) -> Result<()> {
    std::fs::remove_dir(dir).with_context(|| format!("빈 디렉터리를 지우지 못했다: {}", dir.display()))?;
    보고.지운.push(dir.to_path_buf());
    Ok(())
}

fn is_hex(value: &str) -> bool {
    !value.is_empty() && value.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}
