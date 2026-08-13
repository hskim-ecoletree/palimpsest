//! CI 검사 — **단계 1**(stack §4.3). 전부 S 규모이고 외부 의존을 늘리지 않는다.
//!
//! 여기 있는 일곱 중 둘은 계획 §2 가 *"되돌릴 수 없는 것"* 으로 분류한 것이다.
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

use std::collections::BTreeMap;
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
    match std::env::args().nth(1).as_deref() {
        None | Some("check") => check(&root),
        // 파생 ③ — 문서 표를 스키마에서 낸다. **손으로 쓰지 않는다.**
        Some("schema-doc") => {
            let text = std::fs::read_to_string(root.join("schema/graph.toml"))?;
            let schema = pal_core::GraphSchema::parse(&text).map_err(|e| anyhow::anyhow!("{e}"))?;
            let out = root.join("docs/graph-schema.md");
            std::fs::write(&out, render_schema_doc(&schema))?;
            println!("  냈다  {}", out.display());
            Ok(())
        }
        Some(other) => bail!("모르는 명령이다: {other} — `check` 또는 `schema-doc`"),
    }
}

fn check(root: &Path) -> Result<()> {
    let mut failures = Vec::new();

    let checks = [
        ("의존 방향", check_dependency_direction(root)),
        ("코어 어휘 금지", check_vocabulary(root)),
        ("의도 저장소 폐기 경로 부재", check_intent_untouched(root)),
        ("unsafe 금지", check_forbid_unsafe(root)),
        ("의존 정책", check_deny(root)),
        ("gix 격리", check_gix_isolation(root)),
        ("스키마 정합", check_schema(root)),
        ("선택 필드 금지 (1단계)", check_optional_fields(root)),
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

// ── 검사 8 — 선택 필드 금지 · 1단계 (stack §4.3 · F03-3) ────────────────────
//
// [stack §4.3](../../docs/plan/00-stack.md) 의 표가 이 검사의 소유를 **F03** 으로,
// 1 단계의 범위를 *"`pal-core` 의 `pub struct` 필드에 대한 문자열 스캔"* 으로 적었다.
// 2 단계(`syn` AST 승급)는 여기가 아니다.
//
// # 왜 금지인가
//
// stack §5.4: *"`Option<T>` — 선택 필드 금지 위반. 그리고 `None` 이 **「없음」인지
// 「안 만듦」인지 구별 안 됨**."* 이 저장소가 `Capable` · `UnresolvedReason` ·
// `Uncapturable` 로 일관되게 내린 판단이고 [ADR-0005](../../docs/adr/0005-absence-carries-its-kind.md)
// 가 *"부재는 종류를 싣는다"* 로 정본화했다.
//
// # 이 검사가 못 보는 것 — **적어 두지 않으면 1 단계가 2 단계인 척한다**
//
//   · `enum` 변형 안의 필드 (`Resolution::Candidates { demoted_to: Option<…> }`)
//   · 여러 줄에 걸쳐 쓰인 필드 선언
//   · 타입 별칭 뒤에 숨은 `Option`
//   · `pub` 이 아닌 필드 — **일부러 안 본다.** stack §5.4 가 구현 내부 자료구조를
//     허용 열에 두었다
//
// 허용되는 자리는 저장 포트 트레잇의 **반환값**인데, 그것은 `fn` 이라 이 스캔에
// 애초에 안 걸린다.

/// `pub struct` 안의 `pub` 필드에 `Option<` 이 있는가.
fn check_optional_fields(root: &Path) -> Result<String> {
    let src = root.join("crates/pal-core/src");
    let mut hits = Vec::new();
    let mut scanned = 0usize;
    for file in rust_sources(&src)? {
        let text = std::fs::read_to_string(&file)?;
        let mut in_struct = false;
        let mut depth = 0i32;
        for (n, line) in text.lines().enumerate() {
            let t = line.trim();
            if !in_struct && t.starts_with("pub struct ") && t.ends_with('{') {
                in_struct = true;
                depth = 1;
                scanned += 1;
                continue;
            }
            if !in_struct {
                continue;
            }
            depth += i32::try_from(t.matches('{').count()).unwrap_or(0);
            depth -= i32::try_from(t.matches('}').count()).unwrap_or(0);
            if depth <= 0 {
                in_struct = false;
                continue;
            }
            // 주석은 필드가 아니다 — 이 규칙을 설명하는 문장이 그 자리에 있다.
            if t.starts_with("//") || t.starts_with("/*") || t.starts_with('*') {
                continue;
            }
            if t.starts_with("pub ") && t.contains("Option<") {
                let rel = file.strip_prefix(root).unwrap_or(&file);
                hits.push(format!("{}:{}  {t}", rel.display(), n + 1));
            }
        }
    }
    if !hits.is_empty() {
        bail!(
            "`pal-core` 의 `pub struct` 에 선택 필드가 있다 — `None` 이 「없음」인지 \
             「안 만듦」인지 구별되지 않는다 (stack §5.4 · ADR-0005):\n    {}",
            hits.join("\n    ")
        );
    }
    Ok(format!("`pub struct` {scanned}개 · 선택 필드 0"))
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

// ── 검사 7 — 스키마 정합 (stack §4.3 단계 2 · DESIGN §1.2) ────────────────────

/// `schema/graph.toml` ↔ 코드. **양방향이다.**
///
/// | 방향 | 무엇을 막나 |
/// |---|---|
/// | 코드 → 스키마 | 급할 때 코드에만 노드를 만드는 것(F22 §4) |
/// | 스키마 → 코드 | **스키마가 만들 수 없는 것을 선언한 채 자라는 것** — 온톨로지의 팽창 |
///
/// 그리고 셋째 다리가 있다: 스키마가 적은 속성 이름과 **Rust 타입의 `pub` 필드**를
/// 대조한다. 이것이 없으면 필드를 하나 더 붙이고 스키마에 안 적는 경로가 열린 채로 남는다.
///
/// **스키마를 읽는 것은 `pal_core::GraphSchema::parse` 다** — 검사가 자기 파서를 들면
/// CI 를 통과한 스키마가 실행 시점에 거부될 수 있다.
fn check_schema(root: &Path) -> Result<String> {
    let path = root.join("schema/graph.toml");
    let text = std::fs::read_to_string(&path)
        .with_context(|| format!("스키마를 읽지 못했다: {}", path.display()))?;

    // **로딩 시점 거부가 여기서 CI 실패가 된다** (DESIGN §3.4).
    let schema = pal_core::GraphSchema::parse(&text).map_err(|e| anyhow::anyhow!("{e}"))?;

    let src = root.join("crates/pal-core/src");
    let marked = marked_types(&src)?;

    let mut problems = Vec::new();

    // ── 방향 1 — 코드에 표식이 있는데 스키마에 없다 ──────────────────────────
    for (label, (kind, rust_type, _)) in &marked {
        let found = match kind {
            Mark::Node => schema.nodes.get(label).map(|n| n.rust_type.clone()),
            Mark::Edge => schema
                .edges
                .get(label)
                .and_then(|e| e.carried_by.carrier().map(|c| c.rust_type.clone())),
        };
        match found {
            None => problems.push(format!(
                "코드가 `{label}` 을 선언했는데 스키마에 없다 ({rust_type})"
            )),
            Some(declared) if &declared != rust_type => problems.push(format!(
                "`{label}` 의 타입이 어긋난다 — 코드 `{rust_type}` · 스키마 `{declared}`"
            )),
            Some(_) => {}
        }
    }

    // ── 방향 2 — 스키마에 있는데 코드에 표식이 없다 ──────────────────────────
    for label in schema.nodes.keys() {
        if !marked.contains_key(label) {
            problems.push(format!(
                "스키마가 노드 `{label}` 을 선언했는데 코드에 `[graph-node]` 표식이 없다"
            ));
        }
    }
    for label in schema.edges.keys() {
        if !marked.contains_key(label) {
            problems.push(format!(
                "스키마가 엣지 `{label}` 을 선언했는데 코드에 `[graph-edge]` 표식이 없다"
            ));
        }
    }

    // ── 방향 3 — 속성 이름 ↔ `pub` 필드 ─────────────────────────────────────
    for (label, decl) in &schema.nodes {
        let Some((_, rust_type, span)) = marked.get(label) else { continue };
        match &decl.status {
            pal_core::NodeStatus::NotBuilt { by } => {
                // **자리만 만든 노드는 값을 만들 수 없어야 한다.**
                if !span.uninhabited {
                    problems.push(format!(
                        "`{label}` 은 `not_built`({by}) 인데 `{rust_type}` 에 값을 만들 수 있다 — \
                         자리만 두고 값을 만들 수 있으면 \"안 만들었음\"과 \"없음\"이 같아진다"
                    ));
                }
            }
            pal_core::NodeStatus::Built => {
                let mut declared = schema.field_names(label);
                declared.sort();
                let mut actual = span.fields.clone();
                actual.sort();
                if declared != actual {
                    let 없는: Vec<&String> =
                        actual.iter().filter(|f| !declared.contains(f)).collect();
                    let 남는: Vec<&String> =
                        declared.iter().filter(|f| !actual.contains(f)).collect();
                    if !없는.is_empty() {
                        problems.push(format!(
                            "`{rust_type}` 의 필드 {없는:?} 가 스키마에 없다"
                        ));
                    }
                    if !남는.is_empty() {
                        problems.push(format!(
                            "스키마가 `{label}` 에 적은 {남는:?} 가 `{rust_type}` 에 없다"
                        ));
                    }
                }
            }
        }
    }

    // ── 파생 — 문서 표가 스키마에서 나온 그대로인가 ──────────────────────────
    let doc_path = root.join("docs/graph-schema.md");
    let want = render_schema_doc(&schema);
    match std::fs::read_to_string(&doc_path) {
        Ok(have) if have == want => {}
        Ok(_) => problems.push(
            "docs/graph-schema.md 가 스키마와 다르다 — `cargo xtask schema-doc` 으로 다시 낸다"
                .to_owned(),
        ),
        Err(_) => problems.push("docs/graph-schema.md 가 없다 — `cargo xtask schema-doc`".to_owned()),
    }

    if !problems.is_empty() {
        bail!("스키마와 코드가 어긋난다:\n    {}", problems.join("\n    "));
    }
    Ok(format!(
        "노드 라벨 {}개 · 엣지 타입 {}개 · 양방향 0건",
        schema.nodes.len(),
        schema.edges.len()
    ))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mark {
    Node,
    Edge,
}

/// 표식이 붙은 타입 하나에서 읽어낸 것.
#[derive(Debug, Default, Clone)]
struct TypeSpan {
    fields: Vec<String>,
    /// 변형도 필드도 없어서 값을 만들 수 없는가.
    uninhabited: bool,
}

/// `pal-core` 소스에서 `[graph-node]`·`[graph-edge]` 표식을 걷는다.
///
/// **표식을 소스에 두는 이유**: 별도 목록에 두면 그 목록이 타입에서 멀어지고, 멀어진
/// 목록은 늦게 갱신된다. 표식은 타입 바로 위에 있어서 그 타입을 고치는 사람의 눈에 든다.
fn marked_types(src: &Path) -> Result<BTreeMap<String, (Mark, String, TypeSpan)>> {
    let mut out: BTreeMap<String, (Mark, String, TypeSpan)> = BTreeMap::new();

    for file in rust_sources(src)? {
        let text = std::fs::read_to_string(&file)?;
        let lines: Vec<&str> = text.lines().collect();

        for (i, line) in lines.iter().enumerate() {
            let Some(label) = marker(line, "[graph-node]") else { continue };
            let Some((rust_type, span)) = type_after(&lines, i) else {
                bail!("{}:{} 의 `[graph-node] {label}` 뒤에 타입이 없다", file.display(), i + 1);
            };
            out.insert(label, (Mark::Node, rust_type, span));
        }

        // 엣지 표식은 **필드**에 붙는다 — 그 엣지를 싣고 있는 자리이기 때문이다.
        for (i, line) in lines.iter().enumerate() {
            let Some(label) = marker(line, "[graph-edge]") else { continue };
            let Some(owner) = enclosing_type(&lines, i) else {
                bail!("{}:{} 의 `[graph-edge] {label}` 이 타입 밖에 있다", file.display(), i + 1);
            };
            out.insert(label, (Mark::Edge, owner, TypeSpan::default()));
        }
    }
    Ok(out)
}

/// `**[graph-node] `Symbol`**` 에서 `Symbol` 만 꺼낸다.
fn marker(line: &str, tag: &str) -> Option<String> {
    let rest = line.trim_start().strip_prefix("///")?.trim();
    let rest = rest.split_once(tag)?.1;
    let inner = rest.split_once('`')?.1;
    let (name, _) = inner.split_once('`')?;
    Some(name.to_owned())
}

/// 주석 뒤에 오는 첫 `pub struct`/`pub enum` 과 그 `pub` 필드들.
fn type_after(lines: &[&str], from: usize) -> Option<(String, TypeSpan)> {
    let mut i = from + 1;
    while i < lines.len() {
        let t = lines[i].trim_start();
        if t.starts_with("///") || t.starts_with("#[") || t.is_empty() {
            i += 1;
            continue;
        }
        let name = t
            .strip_prefix("pub struct ")
            .or_else(|| t.strip_prefix("pub enum "))?
            .split(|c: char| !c.is_alphanumeric() && c != '_')
            .next()?
            .to_owned();
        // 한 줄로 닫히는 거주 불가 열거 — `pub enum X {}`
        if t.ends_with("{}") {
            return Some((name, TypeSpan { fields: Vec::new(), uninhabited: true }));
        }
        let mut fields = Vec::new();
        let mut j = i + 1;
        let mut body = 0usize;
        while j < lines.len() {
            let l = lines[j].trim_start();
            if l == "}" {
                break;
            }
            if let Some(f) = l.strip_prefix("pub ") {
                if let Some((name, _)) = f.split_once(':') {
                    fields.push(name.trim().to_owned());
                }
            }
            if !l.starts_with("//") && !l.is_empty() {
                body += 1;
            }
            j += 1;
        }
        let uninhabited = body == 0;
        return Some((name, TypeSpan { fields, uninhabited }));
    }
    None
}

/// 이 줄을 감싸는 `pub struct`/`pub enum` 의 이름 — 위로 거슬러 찾는다.
fn enclosing_type(lines: &[&str], from: usize) -> Option<String> {
    for i in (0..from).rev() {
        let t = lines[i].trim_start();
        if let Some(rest) = t.strip_prefix("pub struct ").or_else(|| t.strip_prefix("pub enum ")) {
            return Some(
                rest.split(|c: char| !c.is_alphanumeric() && c != '_').next()?.to_owned(),
            );
        }
    }
    None
}

/// 파생 ③ — 문서 표. **손으로 쓰지 않는다.**
fn render_schema_doc(s: &pal_core::GraphSchema) -> String {
    use std::fmt::Write as _;
    let mut o = String::new();
    o.push_str("<!-- 이 파일은 `cargo xtask schema-doc` 이 낸다. 손으로 고치지 않는다. -->\n");
    o.push_str("<!-- 정본은 schema/graph.toml 이고 CI 가 둘의 일치를 센다. -->\n\n");
    o.push_str("# 그래프 스키마 v");
    let _ = writeln!(o, "{}\n", s.version);
    let _ = writeln!(
        o,
        "노드 라벨 **{}개** · 엣지 타입 **{}개**. \
         자라는 것 자체가 관측 대상이다([DESIGN §1.2](DESIGN.md)).\n",
        s.nodes.len(),
        s.edges.len()
    );

    o.push_str("## 노드\n\n| 라벨 | 출처 | Rust 타입 | 키 | 상태 |\n|---|---|---|---|---|\n");
    for n in s.nodes.values() {
        let status = match &n.status {
            pal_core::NodeStatus::Built => "값이 선다".to_owned(),
            pal_core::NodeStatus::NotBuilt { by } => format!("**자리만** — {by} 가 만든다"),
        };
        let _ = writeln!(
            o,
            "| `{}` | `{}` | `{}` | `{}` | {status} |",
            n.label,
            n.provenance.name(),
            n.rust_type,
            n.key.join("`, `")
        );
    }

    o.push_str("\n### 속성\n\n| 노드 | 속성 | 형 | 생산자 | 필수 |\n|---|---|---|---|---|\n");
    for n in s.nodes.values() {
        for a in &n.attrs {
            let req = match &a.required {
                pal_core::Requirement::Always => "예".to_owned(),
                pal_core::Requirement::IfProvenance(p) => format!("`{}` 일 때", p.name()),
            };
            let _ = writeln!(
                o,
                "| `{}` | `{}` | `{}` | `{}` | {req} |",
                n.label,
                a.name,
                a.value_type,
                a.producer.name()
            );
        }
    }

    o.push_str(
        "\n## 엣지\n\n**모든 엣지가 공통 넷을 진다** — 해소 등급 · 출처 · 근거 · 발생 `Snapshot`.\n\
         넷이 없는 엣지 타입은 등록되지 않는다.\n\n\
         | 엣지 | from | to | 카디널리티 | 등급 | 출처 | 근거 | Snapshot | 실린 자리 |\n\
         |---|---|---|---|---|---|---|---|---|\n",
    );
    for e in s.edges.values() {
        let grade = match &e.grade {
            pal_core::GradeRule::Fixed(g) => format!("`{}` (고정)", g.name()),
            pal_core::GradeRule::PerEdge => "엣지마다".to_owned(),
        };
        let ev = match &e.evidence {
            pal_core::EvidenceRule::NotApplicable => "해당 없음".to_owned(),
            pal_core::EvidenceRule::RequiredIfInferred { attr } => {
                format!("`{attr}` (`inferred` 일 때 필수)")
            }
        };
        let carrier = e
            .carried_by
            .carrier()
            .map_or_else(|| "—".to_owned(), |c| format!("`{}::{}`", c.rust_type, c.field));
        let _ = writeln!(
            o,
            "| `{}` | `{}` | `{}` | {} | {grade} | {} | {ev} | `{}` | {carrier} |",
            e.name,
            e.from,
            e.to.join("`, `"),
            e.cardinality.name(),
            e.provenance.iter().map(|p| format!("`{}`", p.name())).collect::<Vec<_>>().join(" · "),
            e.snapshot
        );
    }
    o
}
