//! CI 검사 — **단계 1**(stack §4.3). 전부 S 규모이고 외부 의존을 늘리지 않는다.
//!
//! 여기 있는 여섯 중 둘은 계획 §2 가 *"되돌릴 수 없는 것"* 으로 분류한 것이다.
//! **그 둘의 처분은 게이트가 아니라 빌드 실패다.**
//!
//! 앞의 다섯이 stack §4.3 **단계 1** 의 전부다 — F01 완료 체크리스트가 *"CI 1단계 켜기"* 로
//! 세는 그 목록이고, 다섯째(`cargo-deny`)가 S0 이 남긴 빚이었다.
//! 여섯째(gix 격리)는 단계 1 이 아니라 **S1 의 합격선 ⑤** 다 — 산출이 아니라 구조를
//! 재는 합격선이라 게이트가 아니라 여기 산다(`corpus/criteria.toml` `[s1.pass]`).
//!
//! ```text
//! cargo xtask check
//! ```

#![forbid(unsafe_code)]

use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result, bail};

/// stack §4.2 의 금지 어휘. 정본은 그 문서이고 여기는 그것을 옮겨 담는다.
const BANNED_HOST: &[&str] = &["claude", "mcp", "tool_call", "session", "prompt"];
const BANNED_GOVERNANCE: &[&str] =
    &["gate", "risk_level", "block", "approve_and_merge", "completion", "change_contract"];
const BANNED_STORAGE: &[&str] = &["cypher", "sql", "redb", "table", "node_label"];

/// `pal-core` 가 의존해서는 안 되는 기술 크레이트 — stack §4.1.
const CORE_FORBIDDEN_DEPS: &[&str] = &["tree-sitter", "redb", "gix"];

/// 의도를 지우는 경로. `pal-store` 소스에 나타나면 실패 — R-21.
const INTENT_DELETE_MARKERS: &[&str] = &["pal_intent", "pal-intent", "intent.redb", "intent/"];

fn main() -> Result<()> {
    let root = repo_root()?;
    let mut failures = Vec::new();

    let checks = [
        ("의존 방향", check_dependency_direction(&root)),
        ("코어 어휘 금지", check_vocabulary(&root)),
        ("의도 저장소 폐기 경로 부재", check_intent_untouched(&root)),
        ("unsafe 금지", check_forbid_unsafe(&root)),
        ("의존 정책", check_deny(&root)),
        ("gix 격리", check_gix_isolation(&root)),
    ];
    let total = checks.len();

    for (name, result) in checks {
        match result {
            Ok(note) => println!("  ok    {name}  — {note}"),
            Err(e) => {
                println!("  FAIL  {name}");
                failures.push(format!("{name}: {e:#}"));
            }
        }
    }

    if failures.is_empty() {
        println!("\n검사 {total}/{total} 통과");
        Ok(())
    } else {
        eprintln!();
        for f in &failures {
            eprintln!("{f}");
        }
        bail!("{}개 검사가 실패했다", failures.len())
    }
}

fn repo_root() -> Result<PathBuf> {
    Ok(Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .context("워크스페이스 루트를 찾지 못했다")?
        .to_path_buf())
}

// ── 검사 1 — 의존 방향 (stack §4.1) ─────────────────────────────────────────

fn check_dependency_direction(root: &Path) -> Result<String> {
    let out = Command::new(env!("CARGO"))
        .args(["metadata", "--format-version", "1", "--no-deps"])
        .current_dir(root)
        .output()
        .context("cargo metadata 를 돌리지 못했다")?;
    let meta: serde_json::Value = serde_json::from_slice(&out.stdout)?;

    let packages = meta["packages"].as_array().context("packages 가 없다")?;
    let workspace: Vec<&str> =
        packages.iter().filter_map(|p| p["name"].as_str()).collect();

    let deps_of = |name: &str| -> Vec<String> {
        packages
            .iter()
            .find(|p| p["name"].as_str() == Some(name))
            .and_then(|p| p["dependencies"].as_array())
            .map(|d| d.iter().filter_map(|x| x["name"].as_str()).map(str::to_owned).collect())
            .unwrap_or_default()
    };

    // (1) pal-core 는 워크스페이스 내 어떤 크레이트에도 의존하지 않는다
    let core = deps_of("pal-core");
    let leaked: Vec<&String> = core.iter().filter(|d| workspace.contains(&d.as_str())).collect();
    if !leaked.is_empty() {
        bail!("pal-core 가 워크스페이스 크레이트에 의존한다: {leaked:?}");
    }

    // (2) pal-core 는 파서·저장 기술에 의존하지 않는다
    let tech: Vec<&String> = core
        .iter()
        .filter(|d| CORE_FORBIDDEN_DEPS.iter().any(|f| d.starts_with(f)))
        .collect();
    if !tech.is_empty() {
        bail!("pal-core 가 기술 크레이트에 의존한다: {tech:?}");
    }

    // (3) 어떤 크레이트도 표면(pal-cli)에 의존하지 않는다
    for p in &workspace {
        if *p != "pal-cli" && deps_of(p).iter().any(|d| d == "pal-cli") {
            bail!("{p} 가 pal-cli 에 의존한다 — 소비자 어휘의 역류");
        }
    }

    // (4) **R-21** — pal-store 는 pal-intent 에 의존하지 않는다
    if deps_of("pal-store").iter().any(|d| d == "pal-intent") {
        bail!("pal-store 가 pal-intent 에 의존한다 — 캐시 폐기 경로가 의도에 닿는다 (R-21)");
    }

    Ok(format!("크레이트 {}개, 규칙 4", workspace.len()))
}

// ── 검사 2 — 코어 어휘 금지 (stack §4.2) ────────────────────────────────────

fn check_vocabulary(root: &Path) -> Result<String> {
    let allow = read_allowlist(&root.join("xtask/vocab.toml"))?;
    let banned: Vec<&str> = BANNED_HOST
        .iter()
        .chain(BANNED_GOVERNANCE)
        .chain(BANNED_STORAGE)
        .copied()
        .filter(|w| !allow.iter().any(|a| a == w))
        .collect();

    let mut hits = Vec::new();
    for file in rust_sources(&root.join("crates/pal-core/src"))? {
        let text = std::fs::read_to_string(&file)?;
        for (n, line) in text.lines().enumerate() {
            // 주석은 산문이라 검사하지 않는다 — 금지 대상은 **코드의 어휘**다.
            let code = line.split("//").next().unwrap_or("");
            for w in &banned {
                if code.to_lowercase().contains(w) {
                    hits.push(format!("{}:{} `{w}`", file.display(), n + 1));
                }
            }
        }
    }
    if !hits.is_empty() {
        bail!("pal-core 에 금지 어휘가 있다:\n    {}", hits.join("\n    "));
    }
    Ok(format!("금지어 {}개 · 허용 예외 {}개", banned.len(), allow.len()))
}

/// `vocab.toml` 의 `allow = [...]` 에서 따옴표 안의 것만 걷는다.
/// **toml 크레이트를 들이지 않는다** — 이 한 줄을 읽자고 의존을 늘리지 않는다(stack §3.4).
fn read_allowlist(path: &Path) -> Result<Vec<String>> {
    let text = std::fs::read_to_string(path)
        .with_context(|| format!("허용 목록을 읽지 못했다: {}", path.display()))?;
    let body = text
        .split_once("allow")
        .and_then(|(_, rest)| rest.split_once('['))
        .and_then(|(_, rest)| rest.split_once(']'))
        .map(|(inside, _)| inside.to_owned())
        .unwrap_or_default();
    Ok(body.split('"').skip(1).step_by(2).map(str::to_owned).collect())
}

// ── 검사 3 — 의도 저장소 폐기 경로 부재 (R-21) ──────────────────────────────

fn check_intent_untouched(root: &Path) -> Result<String> {
    let dir = root.join("crates/pal-store/src");
    let mut hits = Vec::new();
    for file in rust_sources(&dir)? {
        let text = std::fs::read_to_string(&file)?;
        for (n, line) in text.lines().enumerate() {
            let code = line.split("//").next().unwrap_or("");
            for m in INTENT_DELETE_MARKERS {
                if code.contains(m) {
                    hits.push(format!("{}:{} `{m}`", file.display(), n + 1));
                }
            }
        }
    }
    if !hits.is_empty() {
        bail!(
            "pal-store 가 의도 저장소를 언급한다 — 지우는 경로가 생길 자리다 (R-21):\n    {}",
            hits.join("\n    ")
        );
    }
    Ok("pal-store 소스에 의도 경로 언급 0건".to_owned())
}

// ── 검사 4 — unsafe 금지 (stack §3.4) ───────────────────────────────────────

fn check_forbid_unsafe(root: &Path) -> Result<String> {
    let mut missing = Vec::new();
    let mut checked = 0;
    for entry in std::fs::read_dir(root.join("crates"))? {
        let dir = entry?.path();
        for name in ["lib.rs", "main.rs"] {
            let f = dir.join("src").join(name);
            if f.exists() {
                checked += 1;
                if !std::fs::read_to_string(&f)?.contains("#![forbid(unsafe_code)]") {
                    missing.push(f.display().to_string());
                }
            }
        }
    }
    if !missing.is_empty() {
        bail!("`#![forbid(unsafe_code)]` 가 없다:\n    {}", missing.join("\n    "));
    }
    Ok(format!("크레이트 루트 {checked}개"))
}

// ── 검사 6 — gix 격리 (R-15 · criteria [s1.pass].gix_direct_dependents) ─────

/// `gix` 에 직접 의존하는 워크스페이스 크레이트는 **`pal-git` 하나뿐이어야 한다.**
///
/// `gix` 는 API 가 아직 진화 중이다(stack §3.1). 접촉면이 퍼지면 상류가 시그니처를 바꿀 때
/// 고칠 자리가 한 곳이 아니게 되고, [R-15] 의 대응 *"깨지면 그 모듈만 고친다"* 가
/// 성립하지 않는다. **이것은 산출이 아니라 구조의 합격선이고 그래서 기계가 센다.**
fn check_gix_isolation(root: &Path) -> Result<String> {
    const ALLOWED: &str = "pal-git";

    let out = Command::new(env!("CARGO"))
        .args(["metadata", "--format-version", "1", "--no-deps"])
        .current_dir(root)
        .output()
        .context("cargo metadata 를 돌리지 못했다")?;
    let meta: serde_json::Value = serde_json::from_slice(&out.stdout)?;
    let packages = meta["packages"].as_array().context("packages 가 없다")?;

    let mut leaked = Vec::new();
    for p in packages {
        let Some(name) = p["name"].as_str() else { continue };
        if name == ALLOWED {
            continue;
        }
        let deps = p["dependencies"].as_array().map_or(&[][..], Vec::as_slice);
        for d in deps {
            let Some(dep) = d["name"].as_str() else { continue };
            // `gix` 와 그 하위 크레이트(`gix-*`) 전부. 우회 경로를 막는다.
            if dep == "gix" || dep.starts_with("gix-") {
                leaked.push(format!("{name} → {dep}"));
            }
        }
    }
    if !leaked.is_empty() {
        bail!(
            "gix 가 {ALLOWED} 밖으로 샜다 — R-15 의 대응이 성립하지 않는다:\n    {}",
            leaked.join("\n    ")
        );
    }
    Ok(format!("gix 직접 의존은 {ALLOWED} 하나"))
}

// ── 검사 5 — 의존 정책 (stack §3.4 · §4.3 단계 1) ────────────────────────────

/// `cargo deny check` 를 부른다 — 라이선스 · 보안 권고 · 출처 · 금지 크레이트.
///
/// **미설치일 때 건너뛰지 않는다.** 건너뛴 검사는 켜지지 않은 검사이고, 이 검사는
/// F01 완료 체크리스트가 *"CI 1단계 켜기"* 로 세는 다섯 중 하나다. 정책 정본은
/// 저장소 루트의 `deny.toml` 이며 **거기에 줄이 느는 것 자체가 관측 대상이다.**
///
/// 여기가 검사가 저장소 밖 도구에 기대는 유일한 자리다. xtask 의 Cargo 의존은
/// 늘지 않는다(stack §3.3) — 서브프로세스로 부른다.
fn check_deny(root: &Path) -> Result<String> {
    let policy = root.join("deny.toml");
    if !policy.exists() {
        bail!("deny.toml 이 없다 — 정책 없이 통과시키지 않는다");
    }

    let out = Command::new(env!("CARGO"))
        .args(["deny", "--all-features", "check"])
        .current_dir(root)
        .output()
        .context("cargo 를 실행하지 못했다")?;

    let stderr = String::from_utf8_lossy(&out.stderr);
    if !out.status.success() {
        // 미설치와 위반은 다른 사건이다. 뭉개면 "설치 안 됨"이 "정책 위반"으로 보고된다.
        if stderr.contains("no such command") || stderr.contains("no such subcommand") {
            bail!(
                "cargo-deny 가 설치되어 있지 않다 — `cargo install --locked cargo-deny` \
                 또는 `brew install cargo-deny`.\n    \
                 이 검사는 stack §4.3 단계 1 에 등록돼 있으므로 건너뛰지 않는다"
            );
        }
        bail!("{}", stderr.trim());
    }

    // 요약은 "advisories ok, bans ok, licenses ok, sources ok" 형태다.
    // **어느 스트림으로 나오는지에 기대지 않는다** — 파이프로 잡으면 터미널일 때와 다르다.
    let stdout = String::from_utf8_lossy(&out.stdout);
    let summary = stderr
        .lines()
        .chain(stdout.lines())
        .map(str::trim)
        .rfind(|l| l.contains("advisories") && l.contains("licenses"))
        .unwrap_or("");
    Ok(if summary.is_empty() { "통과 (요약 없음)".to_owned() } else { summary.to_owned() })
}

fn rust_sources(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        for entry in std::fs::read_dir(&d)
            .with_context(|| format!("읽지 못했다: {}", d.display()))?
        {
            let p = entry?.path();
            if p.is_dir() {
                stack.push(p);
            } else if p.extension().is_some_and(|e| e == "rs") {
                out.push(p);
            }
        }
    }
    out.sort();
    Ok(out)
}
