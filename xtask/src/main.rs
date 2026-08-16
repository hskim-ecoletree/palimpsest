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
        // 파생 — 질의 표를 카탈로그에서 낸다. **손으로 쓰지 않는다.**
        Some("query-doc") => {
            let text = std::fs::read_to_string(root.join("surface/queries.toml"))?;
            let catalog =
                pal_core::QueryCatalog::parse(&text).map_err(|e| anyhow::anyhow!("{e}"))?;
            let out = root.join("docs/query-catalog.md");
            std::fs::write(&out, render_catalog_doc(&catalog))?;
            println!("  냈다  {}", out.display());
            Ok(())
        }
        // ★ 시험을 돌리고 **남는 실패가 등록된 외침과 정확히 같은지** 판정한다.
        Some("test") => test(&root),
        Some(other) => {
            bail!("모르는 명령이다: {other} — `check` · `test` · `schema-doc` · `query-doc`")
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// `cargo xtask test` — **외침을 세는 자리**
// ─────────────────────────────────────────────────────────────────────────────
//
// # 왜 `cargo test` 를 그냥 안 부르나
//
// 이 저장소에는 **일부러 실패하는 시험**이 있다. 짝 없는 `#[cfg(unix)]` 시험은 다른
// 플랫폼에서 조용히 사라지고, 그러면 방어가 사라진 줄도 모른다 — 그래서 그 자리마다
// **시끄럽게 실패하는 짝**을 단다(AGENTS.md · 소유자 지시 2026-08-16).
//
// 그 규율과 CI 는 정면으로 부딪힌다. CI 가 `cargo test` 를 그대로 돌리면 Windows 는
// **영구히 빨갛고**, 그러면 사람이 **CI 의 빨강을 무시하는 법을 배운다** — 이 회차가
// `doctor` 검사 4 에서 고친 것이 정확히 그 형태다.
//
// # 그래서 무엇을 세는가 — **집합이 같은가**
//
// 실패를 「없어야 하는 것」으로 안 본다. **등록된 외침 집합과 실제 실패 집합이 같은가**
// 를 본다. 그러면 셋 다 잡힌다:
//
// | 일어난 일 | 여기서 무엇이 나나 |
// |---|---|
// | 새 시험이 깨졌다 | **등록 안 된 실패** — 빨강 |
// | 외침이 승격돼 이제 통과한다 | **등록됐는데 안 났다** — 빨강. 등록을 지우라고 말한다 |
// | 그대로다 | 초록. 외침의 수와 까닭을 화면에 낸다 |
//
// 둘째 줄이 이 설계의 값이다 — **승격을 하고 등록을 안 지우면 걸린다.** 목록이 조용히
// 낡는 경로를 막는다.

/// **이 플랫폼에서 안 재지는 것** — `(cfg 이름, 시험 이름, 왜 못 재나)`.
///
/// ⚠ 여기 있는 것은 전부 **제품의 결함이 아니라 fixture 의 한계**다. 결함이면 고치지
/// 등록하지 않는다.
const 외침: &[(&str, &str, &str)] = &[
    (
        "windows",
        "모드와_심링크_보존이_이_플랫폼에서는_안_재진다",
        "모드 비트는 개념이 없고, 파일 심링크는 특권이 있어야 만들어진다. \
         그리고 DESIGN §12.9(D34)가 이 플랫폼의 파일 정체성 보존을 **일부러 포기했다**",
    ),
    (
        "windows",
        "쓰기_불가_디렉터리가_이_플랫폼에서는_안_재진다",
        "읽기 전용 속성이 디렉터리 쓰기를 안 막는다(실측 2026-08-17). \
         진짜 쓰기 불가 디렉터리는 ACL 이고 std 밖이다",
    ),
    (
        "windows",
        "파일_심링크_경계가_이_플랫폼에서는_안_재진다",
        "파일 심링크에 `SeCreateSymbolicLinkPrivilege` 가 필요하다. \
         **디렉터리 축은 junction 으로 재진다**",
    ),
    (
        "windows",
        "파이프_방어가_이_플랫폼에서는_안_재진다",
        "FIFO 가 이 플랫폼에 없다",
    ),
    (
        "windows",
        "끊었다는_말이_이_플랫폼에서는_안_나온다",
        "★ **방어가 아니라 서술이다.** 하드링크 방어는 이 플랫폼에서도 서고 \
         `설치가_하드링크를_끊고_밖을_안_고친다` 가 그것을 잰다. 못 하는 것은 \
         *\"끊었다\"* 를 화면에 내는 일뿐이다 — 링크 수를 셀 수 없어서 언제 끊었는지 모른다",
    ),
];

/// 지금 플랫폼에서 등록된 외침.
fn 등록된_외침() -> Vec<(&'static str, &'static str)> {
    let 여기 = if cfg!(windows) {
        "windows"
    } else if cfg!(unix) {
        "unix"
    } else {
        "그밖"
    };
    외침.iter().filter(|(p, ..)| *p == 여기).map(|(_, n, w)| (*n, *w)).collect()
}

fn test(root: &Path) -> Result<()> {
    println!("■ 시험 — 그리고 남는 실패가 등록된 외침과 같은지 본다");
    let out = Command::new(env!("CARGO"))
        .args(["test", "--workspace", "--all-targets", "--no-fail-fast"])
        .current_dir(root)
        .output()
        .context("cargo test 를 돌리지 못했다")?;

    // **stdout 과 stderr 를 그대로 흘린다.** 이 명령이 무엇을 감췄는지 사람이 볼 수
    // 있어야 한다 — 판정만 내고 증거를 숨기면 그것이 곧 조용한 실패다.
    print!("{}", String::from_utf8_lossy(&out.stdout));
    eprint!("{}", String::from_utf8_lossy(&out.stderr));

    // **같은 이름이 여러 시험 바이너리에서 날 수 있다** — 집합으로 센다.
    let 화면 = String::from_utf8_lossy(&out.stdout);
    let mut 실패: Vec<String> =
        실패한_시험들(&화면).into_iter().map(str::to_owned).collect();
    실패.sort();
    실패.dedup();
    let 등록 = 등록된_외침();
    let problems = 등록과_댄다(&실패, &등록);

    if !problems.is_empty() {
        bail!(
            "시험 결과가 등록과 다르다:\n    {}",
            problems.join("\n    ")
        );
    }

    println!();
    if 등록.is_empty() {
        println!("시험 통과 — 이 플랫폼에는 안 재지는 것이 없다");
    } else {
        println!("시험 통과 — 이 플랫폼에서 안 재지는 것 {}개(전부 등록됨):", 등록.len());
        for (name, 왜) in &등록 {
            println!("  · {name}\n      {왜}");
        }
    }
    Ok(())
}

/// 실제 실패와 등록된 외침을 **양방향으로** 댄다 — 어긋난 것마다 한 문장.
///
/// ★ **방향이 둘이어야 한다.** 한 방향만 보면 목록이 조용히 낡는다:
///
/// | 방향 | 무엇을 막나 |
/// |---|---|
/// | 실패 → 등록 | 새로 깨진 것을 「원래 빨갛던 것」으로 흘려보내는 것 |
/// | 등록 → 실패 | **승격을 하고 등록을 안 지우는 것.** 그러면 목록이 없는 사실을 계속 주장한다 |
///
/// **순수 함수다** — 그래야 음성 대조를 시험으로 세울 수 있다(`check_budget_constants`
/// 의 `looks_like_a_budget` 과 같은 규율).
fn 등록과_댄다(실패: &[String], 등록: &[(&'static str, &'static str)]) -> Vec<String> {
    let mut problems = Vec::new();
    for name in 실패 {
        if !등록.iter().any(|(n, _)| n == name) {
            problems.push(format!("등록되지 않은 실패: `{name}` — 진짜로 깨졌다"));
        }
    }
    for (name, 왜) in 등록 {
        if !실패.iter().any(|n| n == name) {
            problems.push(format!(
                "`{name}` 이 등록됐는데 **안 났다** — 승격됐으면 `xtask` 의 `외침` 목록에서 \
                 지우십시오(등록된 까닭: {왜})"
            ));
        }
    }
    problems
}

/// `cargo test` 의 출력에서 **실패한 시험 이름**을 뽑는다.
///
/// 한 줄 형태 하나만 본다: `test <이름> ... FAILED`. 요약 블록(`failures:`)은 **안 본다** —
/// 같은 이름이 두 번 세지고, 그러면 이 함수가 무엇을 세는지 흐려진다.
///
/// **순수 함수다** — 파일도 프로세스도 안 건드린다. 그래야 아래 시험이 선다.
fn 실패한_시험들(stdout: &str) -> Vec<&str> {
    stdout
        .lines()
        .filter_map(|l| l.strip_prefix("test ")?.strip_suffix(" ... FAILED"))
        .map(str::trim)
        .filter(|n| !n.is_empty())
        .collect()
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
        ("카탈로그 정합", check_catalog(root)),
        ("선택 필드 금지 (1단계)", check_optional_fields(root)),
        ("예산 상수 단일 위치", check_budget_constants(root)),
        ("벗어나는 경로 부재", check_no_escape_hatch(root)),
        ("앵커는 신고받지 않는다", check_anchor_is_measured(root)),
        ("낡음이 생성기를 안 부른다", check_no_regeneration(root)),
        ("인입이 자연어 유사도를 안 쓴다", check_no_similarity(root)),
        ("승격이 원본을 안 고친다", check_promotion_is_not_in_place(root)),
        ("설치 경로가 홈을 안 부른다", check_install_never_reaches_home(root)),
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
                hits.push(format!("{}:{}  {t}", 상대_경로(root, &file), n + 1));
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

/// 저장소 루트 기준 상대 경로 — **구분자를 언제나 `/` 로 낸다.**
///
/// ★ `read_dir` 이 낸 경로는 Windows 에서 `\` 를 쓰고, `root.join("crates/pal-core/src")`
/// 의 `/` 와 섞이면 `crates/pal-core/src\binding.rs` 같은 **혼종**이 나온다. 그것을
/// 등록된 자리(전부 `/`)와 `starts_with` 로 대면 절대 안 맞고, 검사는 *"자리가 늘었다"* 를
/// 외친다 — **플랫폼이 판정을 뒤집는 자리다.**
///
/// 화면에 내는 자리에도 같이 쓴다. 진단 문구가 플랫폼마다 다르면 그 문구를 기대하는
/// 시험이 한쪽에서만 선다.
fn 상대_경로(root: &Path, file: &Path) -> String {
    file.strip_prefix(root).unwrap_or(file).to_string_lossy().replace('\\', "/")
}

/// 파생 문서 대조 — **줄바꿈을 정규화해서 댄다.**
///
/// ★ `core.autocrlf=true` 인 워킹트리에서 체크아웃된 문서는 CRLF 이고 `render_*_doc()` 은
/// LF 를 낸다. 바이트로 대면 Windows 에서 **언제나** 실패하고, `cargo xtask schema-doc`
/// 으로 "고쳐도" 다음 체크아웃이 되돌린다 — 손쓸 수 없는 빨강이다.
///
/// `install/eol.rs` 가 같은 문제를 푼 자리이고 그 규율을 그대로 빌린다:
/// **판정은 내용으로 하고 바이트는 있던 대로 둔다.** 홑 `\r` 은 안 건드린다.
fn 줄바꿈_같은가(have: &str, want: &str) -> bool {
    have.replace("\r\n", "\n") == want.replace("\r\n", "\n")
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
        Ok(have) if 줄바꿈_같은가(&have, &want) => {}
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

// ── 검사 11 — 카탈로그 정합 (F06 §2 · `[f06.1.pass]` ①) ─────────────────────
//
// `surface/queries.toml` ↔ `pal_core::QueryName` 의 **양방향** 대조.
// 「스키마 정합」과 같은 형태이고 같은 자격이다 — F22-1 이 음성 대조 9/9 로 각 방향을
// **망가뜨려서** 세웠고, 여기서 그 자격을 낮추지 않는다(`scripts/f06-verify.py`).
//
// ⚠ **방향마다 루프를 따로 돈다.** 한 루프에서 두 방향을 돌면 한쪽의 `continue` 가
// 다른 쪽을 끄고, 하필 **통제가 필요한 표본에서만** 꺼진다 — F05 의 바깥 오라클이
// 정확히 그렇게 꺼졌다(대조가 꺼지는 **열두째** 형태). `check_schema` 가 이미 그
// 형태이고 여기서도 방향 1·2·3·4 가 각각 자기 루프다.
//
// # 방향 4 가 소스 스캔인 이유
//
// *"CLI 가 닿을 수 없는 이름이 있으면 실패"* 를 재려면 바이너리를 돌려야 하는데,
// 이 검사는 **정적**이어야 한다(`cargo xtask check` 는 빌드 산출에 의존하지 않는다).
// 그래서 여기서는 **CLI 가 자기 목록을 갖지 못하게** 막는다 — 소스에 질의 이름이
// 리터럴로 박히면 실패다. 목록이 두 곳에서 자라는 것을 원천에서 막는 쪽이 더 강하다.
// **산출 쪽 대조**(`pal query --list` 의 줄이 카탈로그와 같은가)는
// `crates/pal-cli/tests/catalog_surface.rs` 가 진다.

/// **하한** — 이보다 적으면 네 방향이 공짜로 통과한다.
const CATALOG_MIN_QUERIES: usize = 6;

fn check_catalog(root: &Path) -> Result<String> {
    let path = root.join("surface/queries.toml");
    let text = std::fs::read_to_string(&path)
        .with_context(|| format!("카탈로그를 읽지 못했다: {}", path.display()))?;

    // **로딩 시점 거부가 여기서 CI 실패가 된다** — `check_schema` 와 같은 규율.
    let catalog = pal_core::QueryCatalog::parse(&text).map_err(|e| anyhow::anyhow!("{e}"))?;

    // 하한. **시험되지 않은 대조는 `–` 가 아니라 실패다**(`2e2eb3f`).
    if catalog.queries.len() < CATALOG_MIN_QUERIES {
        bail!(
            "카탈로그의 질의가 {}개다 — {CATALOG_MIN_QUERIES}개 미만이면 아래 네 방향이 \
             전부 공짜로 통과한다",
            catalog.queries.len()
        );
    }

    let code: BTreeMap<&str, pal_core::QueryName> =
        pal_core::QueryName::ALL.into_iter().map(|q| (q.name(), q)).collect();

    let mut problems = Vec::new();

    // ── 방향 1 — 카탈로그에 있는데 코드에 없다 ──────────────────────────────
    for name in catalog.queries.keys() {
        if !code.contains_key(name.as_str()) {
            problems.push(format!(
                "카탈로그가 `{name}` 을 선언했는데 `QueryName::ALL` 에 없다 — \
                 카탈로그가 이 빌드가 답하지 않는 것을 약속하고 있다"
            ));
        }
    }

    // ── 방향 2 — 코드에 있는데 카탈로그에 없다 ──────────────────────────────
    for name in code.keys() {
        if !catalog.queries.contains_key(*name) {
            problems.push(format!(
                "코드가 `{name}` 에 답하는데 카탈로그에 없다 — \
                 질의 추가는 `surface/queries.toml` 변경으로만 일어난다(F06 §2 규칙 1)"
            ));
        }
    }

    // ── 방향 3 — 이름은 같은데 선언이 어긋난다 ──────────────────────────────
    for (name, decl) in &catalog.queries {
        let Some(q) = code.get(name.as_str()) else { continue };
        if decl.summary != q.summary() {
            problems.push(format!("`{name}` 의 요약이 어긋난다 — 코드 `{}`", q.summary()));
        }
        if decl.returns != q.returns() {
            problems.push(format!(
                "`{name}` 의 반환이 어긋난다 — 코드 `{}` · 카탈로그 `{}`",
                q.returns(),
                decl.returns
            ));
        }
        if decl.introduced != q.introduced() {
            problems.push(format!(
                "`{name}` 의 도입이 어긋난다 — 코드 `{}` · 카탈로그 `{}`",
                q.introduced(),
                decl.introduced
            ));
        }
        let 이름들: Vec<&str> = decl.args.iter().map(|a| a.name.as_str()).collect();
        let 타입들: Vec<&str> = decl.args.iter().map(|a| a.value_type.as_str()).collect();
        if 이름들 != q.arg_names() {
            problems.push(format!(
                "`{name}` 의 인자 이름이 어긋난다 — 코드 {:?} · 카탈로그 {이름들:?}",
                q.arg_names()
            ));
        }
        if 타입들 != q.arg_types() {
            problems.push(format!(
                "`{name}` 의 인자 타입이 어긋난다 — 코드 {:?} · 카탈로그 {타입들:?}",
                q.arg_types()
            ));
        }
    }

    // ── 방향 4 — 표면이 자기 목록을 갖는가 ──────────────────────────────────
    //
    // **CLI 소스에 질의 이름이 리터럴로 박히면 실패.** 박히는 순간 목록이 두 곳에서
    // 자라고, 그러면 카탈로그가 단일 진실이 아니다.
    let cli_src = root.join("crates/pal-cli/src");
    let mut 스캔 = 0usize;
    for file in rust_sources(&cli_src)? {
        let body = std::fs::read_to_string(&file)?;
        스캔 += 1;
        for name in catalog.queries.keys() {
            // **따옴표 안일 때만 잡는다.** `report.ledger.snapshot` 같은 필드 접근은
            // 이름이 아니라 경로다 — 그것까지 잡으면 이 검사가 무엇을 재는지 흐려진다.
            if body.contains(&format!("\"{name}\"")) {
                problems.push(format!(
                    "{} 에 질의 이름 `{name}` 이 리터럴로 있다 — 표면은 \
                     `QueryName::ALL` 에서 렌더링해야 하고, 리터럴은 두 번째 목록이다",
                    상대_경로(root, &file)
                ));
            }
        }
    }

    // ── 파생 — 문서 표가 카탈로그에서 나온 그대로인가 ────────────────────────
    let doc_path = root.join("docs/query-catalog.md");
    let want = render_catalog_doc(&catalog);
    match std::fs::read_to_string(&doc_path) {
        Ok(have) if 줄바꿈_같은가(&have, &want) => {}
        Ok(_) => problems.push(
            "docs/query-catalog.md 가 카탈로그와 다르다 — `cargo xtask query-doc` 으로 다시 낸다"
                .to_owned(),
        ),
        Err(_) => {
            problems.push("docs/query-catalog.md 가 없다 — `cargo xtask query-doc`".to_owned());
        }
    }

    if !problems.is_empty() {
        bail!("카탈로그와 코드가 어긋난다:\n    {}", problems.join("\n    "));
    }
    Ok(format!(
        "질의 {}개 · 양방향 0건 · CLI 소스 {스캔}개에 박힌 이름 0건",
        catalog.queries.len()
    ))
}

/// 파생 — 질의 표. **손으로 쓰지 않는다.**
fn render_catalog_doc(c: &pal_core::QueryCatalog) -> String {
    use std::fmt::Write as _;
    let mut o = String::new();
    o.push_str("<!-- 이 파일은 `cargo xtask query-doc` 이 낸다. 손으로 고치지 않는다. -->\n");
    o.push_str("<!-- 정본은 surface/queries.toml 이고 CI 가 둘의 일치를 센다. -->\n\n");
    let _ = writeln!(o, "# 질의 카탈로그 v{}\n", c.version);
    let _ = writeln!(
        o,
        "**이 빌드가 답하는 질의 {}개.** 여기 없는 것은 이 빌드가 답하지 않는다 — \
         [F06 §3](plan/features/F06-surface.md)의 표는 **로드맵이고 이 표의 상위집합이 \
         아니다**.\n",
        c.queries.len()
    );
    o.push_str(
        "이름을 받는 질의는 `Ambiguous`(여럿이라 못 좁혔다)와 `Unknown`(이 스냅샷에서 \
         못 찾았다)으로도 답한다. **둘 다 실패가 아니라 답이고 종료 코드 0 이다.**\n\n",
    );
    o.push_str("| 질의 | 인자 | 반환 | 도입 | 요약 |\n|---|---|---|---|---|\n");
    for q in c.queries.values() {
        let args = if q.args.is_empty() {
            "—".to_owned()
        } else {
            q.args.iter().map(|a| format!("`{}: {}`", a.name, a.value_type)).collect::<Vec<_>>().join(" · ")
        };
        let _ = writeln!(
            o,
            "| `{}` | {args} | `{}` | {} | {} |",
            q.name, q.returns, q.introduced, q.summary
        );
    }
    o
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

// ── 검사 9 — 예산 상수 단일 위치 (stack §5.5 · `[f05.1.pass]` ①) ─────────────
//
// > **단일 위치** — 전부 `pal-core::budget` 의 상수. 다른 곳에 리터럴로 나타나면 CI 실패
//
// # 왜 목록이 아니라 검사인가
//
// `budget.rs` 는 흩어진 자리의 **목록**을 주석으로 들고 있었다. 그 목록은 넷을 적었고
// 실물은 **열**이었다 — 그 뒤에 늘어난 넷(`DEFAULT_CACHE_BUDGET_BYTES`·`EXTRACT_CHUNK`·
// `MARKER_SCAN_BYTES`·`CORRUPT_NOTES`)과 애초에 빠뜨린 하나(`CANDIDATE_LIMIT`, 하필
// 예산 표의 `K` 다)가 거기 없었다. **사람이 세면 다음에 늘어난 것이 빠진다.**
//
// # 이 검사가 세는 두 방향
//
// | 방향 | 무엇을 막나 |
// |---|---|
// | 이름 → 자리 | `budget.rs` 의 이름이 **다른 곳에서 또 정의되는** 것(재수출·복제) |
// | 자리 → 이름 | **새 예산이 다른 크레이트에서 태어나는** 것 |
//
// 둘째가 이 검사의 요점이다. 첫째만 있으면 목록을 안 늘리는 한 통과한다.
//
// # 이 검사가 못 보는 것 — **적어 두지 않으면 완전한 척한다**
//
//   · 함수 **안**의 `const`(`fn` 지역 상수)와 `impl` 블록의 결합 상수는 이름 규칙에
//     안 걸리면 안 보인다
//   · **낱말로 알아본다.** 예산인데 이름에 아래 낱말이 하나도 없으면 못 잡는다.
//     그것을 막을 방법이 없고, 막는 척하지 않는 것이 여기서 지는 몫이다
//   · 리터럴 자체(코드 한가운데의 `2048`)는 안 본다 — 그것은 2 단계다

/// 예산으로 **알아보는** 이름의 낱말. 하나라도 들어 있으면 예산 후보다.
const BUDGET_WORDS: &[&str] =
    &["BUDGET", "LIMIT", "MAX", "DEPTH", "CHUNK", "OVERSIZE", "PROVISIONAL", "SCAN_BYTES", "NOTES"];

/// 낱말에 걸리지만 예산이 아닌 것 — **하나하나 이유를 적는다.**
///
/// 목록이 느는 것 자체가 관측 대상이다(`vocab.toml` 과 같은 규율).
const NOT_A_BUDGET: &[(&str, &str)] = &[
    // `Bucket::ALL`·`Provenance::ALL` 류의 결합 상수. 예산이 아니라 열거의 전수다.
    ("ALL", "열거의 전수 — 값이 아니라 목록이다"),
    // xtask 자신의 금지어 표. 이 파일이 자기를 검사하는 자리다.
    ("INTENT_DELETE_MARKERS", "검사 규칙의 표 — 예산이 아니다"),
    // **이 검사 자신의 규칙 표.** 처음 돌렸을 때 스스로에게 걸렸고, 걸린 것이 옳다 —
    // 규칙이 자기를 예외로 두려면 그 사실이 목록에 서야 한다.
    ("BUDGET_WORDS", "이 검사의 규칙 표 — 예산이 아니다"),
    ("NOT_A_BUDGET", "이 검사의 예외 표 — 예산이 아니다"),
    ("BUDGET_ESCAPES", "「벗어나는 경로 부재」 검사의 낱말 표 — 예산이 아니다"),
    // ★ **OS 가 정한 상수다. 우리가 고를 수 있는 값이 아니다.**
    //
    // 예산은 *"우리가 정한 한계이고, 넘으면 능력이 아니라 예산을 먼저 의심한다"*
    // (stack §5.5 · D16)이다. `MAX_PATH` 는 그 성질이 하나도 없다 — Windows 의 전통적
    // 경로 길이 한계 260 이고, 값을 바꾸면 그것은 조정이 아니라 **틀린 값**이 된다.
    // `pal-core::budget` 으로 옮기면 코어가 플랫폼 상수를 지게 되고(stack §4.1 의
    // 의존 방향), 그 자리에서 *"이 숫자를 늘려 볼까"* 라는 물음이 성립해 버린다.
    ("MAX_PATH", "Windows 가 정한 경로 길이 한계 — 우리가 고르는 값이 아니다"),
];

/// `budget.rs` 의 이름들과, 그 밖에서 태어난 예산 후보.
///
/// **순수 함수다** — 파일을 읽지 않는다. 그래야 음성 대조를 시험으로 세울 수 있다
/// (`[f05.1.pass]` ①: *"상수를 하나 옮겼다 되돌리면 검사가 걸리는지"*).
fn budget_names(source: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in source.lines() {
        let code = line.split("//").next().unwrap_or("");
        let Some(rest) = code.trim_start().strip_prefix("pub const ").or_else(|| {
            code.trim_start().strip_prefix("const ")
        }) else {
            continue;
        };
        let Some(name) = rest.split(':').next().map(str::trim) else { continue };
        if name.is_empty() || !name.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_') {
            continue;
        }
        out.push(name.to_owned());
    }
    out
}

/// 이 이름이 **예산으로 보이는가.**
fn looks_like_a_budget(name: &str) -> bool {
    if NOT_A_BUDGET.iter().any(|(n, _)| *n == name) {
        return false;
    }
    BUDGET_WORDS.iter().any(|w| name.contains(w))
}

fn check_budget_constants(root: &Path) -> Result<String> {
    let home = root.join("crates/pal-core/src/budget.rs");
    let declared = budget_names(
        &std::fs::read_to_string(&home)
            .with_context(|| format!("예산 모듈을 읽지 못했다: {}", home.display()))?,
    );

    // **하한이다.** 이 파일이 비면 아래 전부가 공짜로 통과한다(`2e2eb3f`).
    if declared.len() < 6 {
        bail!(
            "`pal-core::budget` 에 상수가 {}개뿐이다 — 시험되지 않은 검사다",
            declared.len()
        );
    }

    let mut scanned = 0usize;
    let mut strays = Vec::new();
    let mut roots = vec![root.join("xtask/src")];
    for entry in std::fs::read_dir(root.join("crates"))? {
        roots.push(entry?.path().join("src"));
    }
    for dir in roots {
        if !dir.exists() {
            continue;
        }
        for file in rust_sources(&dir)? {
            if file == home {
                continue;
            }
            scanned += 1;
            let text = std::fs::read_to_string(&file)?;
            for name in budget_names(&text) {
                if declared.contains(&name) {
                    strays.push(format!(
                        "{}: `{name}` 이 `pal-core::budget` 에도 있다 — 한 곳이 두 곳이 됐다",
                        file.display()
                    ));
                } else if looks_like_a_budget(&name) {
                    strays.push(format!(
                        "{}: `{name}` 이 예산으로 보이는데 `pal-core::budget` 밖에 있다",
                        file.display()
                    ));
                }
            }
        }
    }

    if !strays.is_empty() {
        bail!(
            "예산 상수가 한 곳에 있지 않다 (stack §5.5):\n    {}",
            strays.join("\n    ")
        );
    }
    Ok(format!("예산 상수 {}개 · 다른 파일 {scanned}개에 0건", declared.len()))
}

// ── 검사 10 — 벗어나는 경로 부재 (F05 §5.1·§5.2) ─────────────────────────────
//
// 둘을 한 검사로 센다. **같은 형태이기 때문이다** — 둘 다 *"이 값을 안 지고 나갈 수
// 있는 문"* 이고, 둘 다 **타입으로 100% 막히지 않는다.** F05 §5.1 이 그것을 인정했다:
// *"타입으로 100% 막히지 않는다는 것을 인정하고, 대신 **빠지면 골든이 깨지는** 자리에
// 검사를 둔다."* 여기가 그 검사의 정적인 절반이다.
//
// | | 무엇을 막나 | 합격선 |
// |---|---|---|
// | `Envelope` | 봉투를 버리고 `T` 만 들고 나가는 경로 | `[f05.3.pass]` ① |
// | `Budget` | 예산을 끄는 손잡이 | `[f05.1.pass]` ④ |
//
// # 이 검사가 지금 재는 것은 **회귀 방지**다
//
// 셋 다 지금 **없다**(확인했다). 그러므로 이 검사는 *"세운다"* 가 아니라
// *"없다는 것을 산출로 검사한다"* 이고, 생기는 순간 CI 가 걸린다.
//
// # 못 보는 것
//
//   · `answer` 만 담는 **생성자**는 이름이 자유로워 낱말로 못 잡는다. 그 자리는
//     골든이 진다(`[f05].pass.everything_that_answers_carries_an_envelope`)
//   · 다른 크레이트가 `Envelope` 를 감싸 벗기는 것 — `pal-core` 밖은 안 본다

/// 봉투를 벗기는 문. **낱말이 코드에 나타나면 실패.**
const ENVELOPE_ESCAPES: &[&str] = &["into_answer", "impl Deref", "Deref for Envelope", "into_inner"];

/// 예산을 끄는 손잡이.
const BUDGET_ESCAPES: &[&str] =
    &["impl Default for Budget", "fn unlimited", "fn unbounded", "fn no_budget"];

fn check_no_escape_hatch(root: &Path) -> Result<String> {
    let mut problems = Vec::new();

    let cases: [(&str, &str, &[&str]); 2] = [
        ("crates/pal-core/src/envelope.rs", "pub struct Envelope<T>", ENVELOPE_ESCAPES),
        ("crates/pal-core/src/budget.rs", "pub struct Budget", BUDGET_ESCAPES),
    ];

    for (rel, must_declare, escapes) in cases {
        let path = root.join(rel);
        let text = std::fs::read_to_string(&path)
            .with_context(|| format!("읽지 못했다: {}", path.display()))?;

        // **하한이다.** 파일이 비었거나 타입이 옮겨 갔으면 아래가 공짜로 통과한다.
        if !text.contains(must_declare) {
            bail!("{rel} 에 `{must_declare}` 가 없다 — 이 검사는 아무것도 안 세고 있다");
        }

        for (n, line) in text.lines().enumerate() {
            // 주석은 산문이다 — 어휘 검사와 같은 규율.
            let code = line.split("//").next().unwrap_or("");
            for e in escapes {
                if code.contains(e) {
                    problems.push(format!("{rel}:{} `{e}`", n + 1));
                }
            }
        }

        // `#[derive(..., Default, ...)]` 도 같은 문이다. 타입 선언 **바로 위**만 본다.
        let lines: Vec<&str> = text.lines().collect();
        if let Some(i) = lines.iter().position(|l| l.contains(must_declare)) {
            for l in &lines[i.saturating_sub(4)..i] {
                if l.contains("derive") && l.contains("Default") {
                    problems.push(format!("{rel}: `{must_declare}` 에 `Default` 가 파생됐다"));
                }
            }
        }
    }

    if !problems.is_empty() {
        bail!(
            "값을 안 지고 나가는 문이 생겼다 (F05 §5.1·§5.2):\n    {}",
            problems.join("\n    ")
        );
    }
    Ok(format!(
        "봉투 {}개 · 예산 {}개 낱말에 0건",
        ENVELOPE_ESCAPES.len(),
        BUDGET_ESCAPES.len()
    ))
}

#[cfg(test)]
mod budget_tests {
    use super::*;

    /// **음성 대조다.** 옮긴 것을 되돌리면 검사가 걸려야 한다 — 안 걸리면 이 검사는
    /// 아무것도 안 세고 있는 것이다(`[f05.1.pass]` ①).
    #[test]
    fn 예산이_밖에서_태어나면_잡힌다() {
        assert!(looks_like_a_budget("EXTRACT_CHUNK"));
        assert!(looks_like_a_budget("PROVISIONAL_SAMPLE_MAX"));
        assert!(looks_like_a_budget("CANDIDATE_LIMIT"));
        assert!(looks_like_a_budget("DEFAULT_CACHE_BUDGET_BYTES"));
        assert!(looks_like_a_budget("MARKER_SCAN_BYTES"));
        assert!(looks_like_a_budget("CORRUPT_NOTES"));
    }

    #[test]
    fn 예산이_아닌_것은_안_잡는다() {
        // 늘 참이면 이 검사는 통과할 수 없는 검사이고, 통과할 수 없는 검사는 지워진다.
        for name in ["ZSTD_LEVEL", "GRAMMAR_REV", "SYMBOL", "BY_NAME", "ALL", "TOKEN_SEPARATOR"] {
            assert!(!looks_like_a_budget(name), "`{name}` 을 예산으로 잡았다");
        }
    }

    #[test]
    fn 이름을_주석과_함께_읽지_않는다() {
        // 주석 안의 `const` 는 코드가 아니다 — 어휘 검사와 같은 규율이다.
        let src = "// const FAKE_MAX: usize = 1;\npub const REAL_MAX: usize = 2;\n";
        assert_eq!(budget_names(src), vec!["REAL_MAX".to_owned()]);
    }

    #[test]
    fn 소문자_이름은_상수가_아니다() {
        assert!(budget_names("const foo: usize = 1;\n").is_empty());
    }
}

// ── 검사 12 — 앵커는 신고받지 않는다 (F09 §4.1 · DESIGN §6.5 D32) ────────────
//
// > 결박을 만드는 주체가 *"이건 커밋 X 기준이야"* 라고 말해도 그 값이 앵커가 되지
// > 않는다 — **앵커는 결박 시점에 기계가 대상 좌표에서 읽은 digest 다.**
//
// **이 검사는 회귀 방지다.** 동작은 이미 참이고(`pal bind` 가 투영에서 읽는다) 없던
// 것은 그 부재를 세는 장치다 — `[f05].envelope_boundary` 와 같은 형태.
//
// # 이름을 세지 않고 **자리를 센다**
//
// 낱말 목록으로 세면 새 이름이 생길 때 조용히 빠진다. 그래서 `WatchEntry` 를 **만드는
// 자리의 수**를 등록하고, 그 수가 변하면 멈춘다 — 사람이 새 자리를 보고 판단한다.
// (`[f05.1]` 의 예산 상수 검사와 같은 형태.)

/// `WatchEntry { .. }` 리터럴이 허용되는 자리 — **`(파일, 왜)`.**
const WATCH_ENTRY_SITES: &[(&str, &str)] = &[
    ("crates/pal-core/src/binding.rs", "타입 선언과 그 단위 시험"),
    ("crates/pal-cli/src/bind.rs", "투영에서 읽어 만든다 — **기계가 잰 값이다**"),
    // ★ **F10 이 더한 자리이고, 이 검사가 그것을 잡아서 여기 적힌다.**
    // `pal narrative approve` 도 `pal bind` 와 **같은 자리에서 같은 값을 읽는다** —
    // 투영의 `symbol.body` 다. 제안이 지고 온 값을 앵커로 쓰는 경로가 **없다**:
    // 제안은 좌표까지만 낸다(`Classification`). 그것이 F09 §4.1(D32)이 요구한
    // *"`watch_snapshot` 은 신고받지 않는다"* 를 인입 경로에서도 지키는 형태다.
    ("crates/pal-cli/src/narrative.rs", "승인이 투영에서 읽어 만든다 — 제안이 지고 오지 않는다"),
];

fn check_anchor_is_measured(root: &Path) -> Result<String> {
    let mut sites: Vec<String> = Vec::new();
    for dir in ["crates/pal-core/src", "crates/pal-cli/src", "crates/pal-query/src",
                "crates/pal-store/src", "crates/pal-intent/src", "crates/pal-extract/src"] {
        for file in rust_sources(&root.join(dir))? {
            let text = std::fs::read_to_string(&file)?;
            for (n, line) in text.lines().enumerate() {
                let code = line.split("//").next().unwrap_or("");
                // 선언(`pub struct WatchEntry {`)은 리터럴이 아니다.
                if code.contains("WatchEntry {") && !code.contains("struct WatchEntry") {
                    sites.push(format!("{}:{}", 상대_경로(root, &file), n + 1));
                }
            }
        }
    }

    // **하한이다.** 자리가 0 이면 이 검사가 아무것도 안 세고 있다 — 타입이 옮겨 갔거나
    // 이름이 바뀐 것이고, 그러면 *"신고를 안 받는다"* 가 검사되지 않는다.
    if sites.is_empty() {
        bail!("`WatchEntry` 를 만드는 자리가 하나도 없다 — 이 검사는 아무것도 안 세고 있다");
    }

    let 허용 = |s: &str| WATCH_ENTRY_SITES.iter().any(|(f, _)| s.starts_with(f));
    let 새것: Vec<&String> = sites.iter().filter(|s| !허용(s)).collect();
    if !새것.is_empty() {
        bail!(
            "`WatchEntry` 를 만드는 자리가 늘었다 — **앵커가 어디서 오는지 사람이 봐야 한다**\n    \
             (F09 §4.1: 앵커는 결박 시점에 **기계가 대상 좌표에서 읽은** digest 다.\n    \
             생산자의 신고를 여기 넣으면 그 신고가 앵커가 된다):\n    {}",
            새것.iter().map(|s| s.as_str()).collect::<Vec<_>>().join("\n    ")
        );
    }
    Ok(format!("`WatchEntry` 생성 자리 {}개 · 등록된 자리 {}개", sites.len(), WATCH_ENTRY_SITES.len()))
}

// ── 검사 13 — 낡음이 생성기를 안 부른다 (F09 §4.1) ──────────────────────────
//
// > **낡음은 탐지만 한다.** `Stale` 이 재생성을 트리거하지 않는다 — 하면
// > ① 생산자 분리(F17)가 깨지고 ② 기록하되 통치하지 않는다는 경계가 무너지고
// > ③ **사람이 승인한 것이 승인 없이 교체된다.**
//
// `Stale` 을 다루는 파일에 쓰기·생성 낱말이 없어야 한다. `pal-intent` 의
// 「지우는 API 부재」와 같은 형태이고 같은 이유로 정적이다.

/// 낡음을 다루는 자리에 있으면 안 되는 낱말.
const REGENERATION_MARKERS: &[&str] =
    &["regenerate", "regen(", "rebuild_note", "write_note", "auto_fix", "autofix"];

fn check_no_regeneration(root: &Path) -> Result<String> {
    let files = ["crates/pal-core/src/binding.rs", "crates/pal-query/src/lib.rs"];
    let mut hits = Vec::new();
    let mut 봤나 = false;

    for rel in files {
        let path = root.join(rel);
        let text = std::fs::read_to_string(&path)
            .with_context(|| format!("읽지 못했다: {}", path.display()))?;
        // **하한** — `Stale` 을 안 다루는 파일을 검사하면 아무것도 안 센다.
        if text.contains("CodeFreshness::Stale") {
            봤나 = true;
        }
        for (n, line) in text.lines().enumerate() {
            let code = line.split("//").next().unwrap_or("");
            for m in REGENERATION_MARKERS {
                if code.to_lowercase().contains(m) {
                    hits.push(format!("{rel}:{} `{m}`", n + 1));
                }
            }
        }
    }

    if !봤나 {
        bail!("`CodeFreshness::Stale` 을 다루는 파일이 하나도 없다 — 이 검사는 아무것도 안 세고 있다");
    }
    if !hits.is_empty() {
        bail!(
            "낡음이 생성기를 부르는 경로가 생겼다 (F09 §4.1):\n    \
             ① 생산자 분리(F17)가 깨지고 ② 기록하되 통치하지 않는다는 경계가 무너지고\n    \
             ③ **사람이 승인한 것이 승인 없이 교체된다**:\n    {}",
            hits.join("\n    ")
        );
    }
    Ok(format!("낡음을 다루는 파일 {}개 · 생성 낱말 {}개에 0건", files.len(), REGENERATION_MARKERS.len()))
}

// ── 검사 14 — 인입이 자연어 유사도를 안 쓴다 (F10 §3.2 · §5) ────────────────
//
// 문서 §3.2 의 표가 여섯째 줄에 못 박았다:
//
// > **본문 자연어의 이름 유사도 — 쓰지 않는다.** 거짓 결박을 만든다.
// > *"주문 취소 로직"* 이 `cancelOrder` 인지 `OrderCanceller` 인지 **기계가 모른다.**
//
// **문장으로 두면 잊힌다.** `[f10.pass]` 가 그것을 CI 로 올린 근거는 이렇다:
// 거짓 결박률은 **표본 50 건의 손 검토**이고 표본은 표본 밖을 못 본다.
// *"유사도를 안 쓴다"* 는 **전수에 걸리는 성질**이라 그 빈자리를 덮는 유일한 수단이다.

/// 좌표 해소에 있으면 안 되는 낱말 — **전부 「비슷하다」를 계산하는 것들.**
const SIMILARITY_MARKERS: &[&str] = &[
    "levenshtein", "jaro", "edit_distance", "similarity", "fuzzy",
    "embedding", "cosine", "ngram", "trigram", "soundex",
];

/// 이 검사가 실제로 무언가를 세고 있다는 증거 — **하한.**
///
/// 없으면 파일이 옮겨 갔거나 이름이 바뀐 것이고, 그러면 이 검사는 **성한 자리를
/// 검사하며 통과한다.** F09 의 검사 12·13 이 세운 형태 그대로다.
const CASCADE_WITNESS: &str = "ResolutionSignal";

fn check_no_similarity(root: &Path) -> Result<String> {
    let files = ["crates/pal-core/src/narrative.rs", "crates/pal-extract/src/narrative.rs"];
    let mut hits = Vec::new();
    let mut 봤나 = false;
    let mut 센_파일 = 0;

    for rel in files {
        let path = root.join(rel);
        // **없는 파일은 건너뛰지 않고 센다** — 아래 하한이 그것을 잡는다.
        let Ok(text) = std::fs::read_to_string(&path) else { continue };
        센_파일 += 1;
        if text.contains(CASCADE_WITNESS) {
            봤나 = true;
        }
        for (n, line) in text.lines().enumerate() {
            // 주석은 산문이라 검사하지 않는다 — 금지 대상은 **코드의 어휘**다
            // (「코어 어휘 금지」와 같은 규율). 이 파일의 머리가 그 낱말들을 **설명**한다.
            let code = line.split("//").next().unwrap_or("");
            for m in SIMILARITY_MARKERS {
                if code.to_lowercase().contains(m) {
                    hits.push(format!("{rel}:{} `{m}`", n + 1));
                }
            }
        }
    }

    // **하한** — 계단식이 있는 파일을 안 보고 있으면 이 검사는 아무것도 안 센다.
    if !봤나 {
        bail!(
            "`{CASCADE_WITNESS}` 를 쓰는 파일이 하나도 없다 — 이 검사는 아무것도 안 세고 있다 \
             (검사한 파일 {센_파일}개)"
        );
    }
    if !hits.is_empty() {
        bail!(
            "좌표 해소가 자연어 유사도를 쓴다 (F10 §3.2 · §5):\n    \
             **거짓 결박을 대량 생산한다. 그리고 틀린 결박은 없는 결박보다 나쁘다.**\n    \
             동점은 좁히는 것이 아니라 **후보로 내고 승인을 요구한다**:\n    {}",
            hits.join("\n    ")
        );
    }
    Ok(format!("인입 파일 {센_파일}개 · 유사도 낱말 {}개에 0건", SIMILARITY_MARKERS.len()))
}

// ── 검사 15 — 승격이 원본을 안 고친다 (F10 §1 · §3.3) ──────────────────────
//
// > **승격은 필드를 고쳐 쓰는 것이 아니다.** `inferred` 노드를 승인하면 그것을 가리키는
// > **새 `asserted` 노드**가 생기고 원본은 `promoted_by` 와 함께 남는다.
//
// `Provenance` 에 setter 가 없는 것과 같은 규율이다(`graph.rs`: *"고쳐 쓰는 경로가 없는
// 것 자체가 세탁 방지의 구현 형태"*). 타입이 이미 `&` 로 받지만, **그 시그니처가
// `&mut` 로 되돌아가는 커밋을 이 검사가 멈춘다.**

/// 승격 경로에 있으면 안 되는 형태 — **제자리에서 고쳐 쓰는 것들.**
const IN_PLACE_PROMOTION: &[&str] =
    &["fn promote(&mut self", ".promoted_by =", ".provenance =", "fn set_promoted", "fn launder"];

/// 승격 함수가 실제로 있다는 증거 — **하한.**
const PROMOTION_WITNESS: &str = "pub fn promote(";

fn check_promotion_is_not_in_place(root: &Path) -> Result<String> {
    let files = ["crates/pal-core/src/binding.rs", "crates/pal-core/src/narrative.rs"];
    let mut hits = Vec::new();
    let mut 봤나 = false;

    for rel in files {
        let path = root.join(rel);
        let text = std::fs::read_to_string(&path)
            .with_context(|| format!("읽지 못했다: {}", path.display()))?;
        if text.contains(PROMOTION_WITNESS) {
            봤나 = true;
        }
        for (n, line) in text.lines().enumerate() {
            let code = line.split("//").next().unwrap_or("");
            for m in IN_PLACE_PROMOTION {
                if code.contains(m) {
                    hits.push(format!("{rel}:{} `{m}`", n + 1));
                }
            }
        }
    }

    // **하한** — 승격 함수가 없으면 이 검사는 아무것도 안 센다.
    if !봤나 {
        bail!("`{PROMOTION_WITNESS}` 가 어디에도 없다 — 이 검사는 아무것도 안 세고 있다");
    }
    if !hits.is_empty() {
        bail!(
            "승격이 원본을 제자리에서 고친다 (F10 §3.3):\n    \
             ① 되돌릴 수 없고 ② **원래 누구의 추론이었는가**가 계보에서 사라지고\n    \
             ③ *\"어디까지가 기록이고 어디부터가 재구성인지\"* 를 아무도 모르게 된다:\n    {}",
            hits.join("\n    ")
        );
    }
    Ok(format!("승격을 다루는 파일 {}개 · 제자리 수정 {}개 형태에 0건", files.len(), IN_PLACE_PROMOTION.len()))
}

// ── 검사 16 — 설치 경로가 홈을 안 부른다 (F24 §2 ⑦) ─────────────────────────
//
// 소유자의 문장이 이 검사를 낳았다:
//
// > **`~/.claude/` 하위에 기대는 구조는 절대 있어서는 안 돼**
//
// 「기대지 않는다」는 코드를 읽어서도 말할 수 있지만, **안 기댄다는 주장과 안 쓴다는
// 사실은 다르다.** 그래서 여기가 재는 것은 **구조 한 겹**이고 하중의 대부분은 아니다.
//
// # ⚠ 이 검사가 못 보는 것 — **적어 두지 않으면 완전한 척한다**
//
// F04 가 이미 같은 말을 했다 — *"그것은 문자열 스캔이라 「소스에 그 낱말이 없다」만
// 말한다 — **낱말 없이도 상위 디렉터리를 지울 수 있고 `..` 하나면 경계가 사라진다**."*
// **실물 하중은 스냅샷이 진다**(`crates/pal-cli/tests/install_stays_inside.rs`:
// 격리 HOME · 격리 TMPDIR · 대상의 부모 — 차이 0).

/// 홈을 유도하는 형태. **설치 경로의 코드에 나타나면 실패.**
const HOME_REACHING: &[&str] =
    &["home_dir", "dirs::", "directories::", "\"HOME\"", "$HOME", "expanduser", "shellexpand"];

/// 이 검사가 실제로 무언가를 세고 있다는 증거 — **하한.**
///
/// 없으면 파일이 옮겨 갔거나 이름이 바뀐 것이고, 그러면 이 검사는 **성한 자리를
/// 검사하며 통과한다.** 검사 12·13·14 가 세운 형태 그대로다.
const INSTALL_WITNESS: &str = "pub fn install(";

fn check_install_never_reaches_home(root: &Path) -> Result<String> {
    let dir = root.join("crates/pal-cli/src/install");
    let mut files = rust_sources(&dir)?;
    files.push(root.join("crates/pal-cli/src/install.rs"));
    // 빌드 스크립트도 설치 경로다 — 커밋을 박으려고 홈을 읽으면 같은 자리가 무너진다.
    files.push(root.join("crates/pal-cli/build.rs"));

    let mut hits = Vec::new();
    let mut 봤나 = false;
    let mut 센_파일 = 0;
    for file in &files {
        let Ok(text) = std::fs::read_to_string(file) else { continue };
        센_파일 += 1;
        if text.contains(INSTALL_WITNESS) {
            봤나 = true;
        }
        for (n, line) in text.lines().enumerate() {
            // 주석은 산문이라 검사하지 않는다 — 이 파일들의 머리가 그 낱말들을 **설명**한다.
            let code = line.split("//").next().unwrap_or("");
            for m in HOME_REACHING {
                if code.contains(m) {
                    hits.push(format!("{}:{} `{m}`", file.display(), n + 1));
                }
            }
        }
    }

    // **하한** — 설치 경로를 안 보고 있으면 이 검사는 아무것도 안 센다.
    if !봤나 {
        bail!(
            "`{INSTALL_WITNESS}` 가 어디에도 없다 — 이 검사는 아무것도 안 세고 있다 \
             (검사한 파일 {센_파일}개)"
        );
    }
    if !hits.is_empty() {
        bail!(
            "설치 경로가 홈을 부른다 (F24 ⑦):\n    \
             소유자의 문장은 **\"`~/.claude/` 하위에 기대는 구조는 절대 있어서는 안 돼\"** \
             였다.\n    설치·갱신·제거는 **대상 프로젝트 안에서만** 선다:\n    {}",
            hits.join("\n    ")
        );
    }
    Ok(format!("설치 소스 {센_파일}개 · 홈 낱말 {}개에 0건", HOME_REACHING.len()))
}

// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod 외침_판정_tests {
    use super::{등록과_댄다, 실패한_시험들, 외침};
    #[cfg(unix)]
    use super::등록된_외침;

    /// `cargo test` 의 실제 출력 형태에서 이름만 뽑는다.
    #[test]
    fn 실패한_이름만_뽑는다() {
        let out = "\
running 3 tests
test 통과하는것 ... ok
test 파이프_방어가_이_플랫폼에서는_안_재진다 ... FAILED
test common::eol::tests::맞추면_왕복한다 ... ok
test 또_깨진것 ... FAILED

failures:

---- 또_깨진것 stdout ----
test result: FAILED. 2 passed; 2 failed; 0 ignored
";
        assert_eq!(
            실패한_시험들(out),
            vec!["파이프_방어가_이_플랫폼에서는_안_재진다", "또_깨진것"]
        );
    }

    /// ★ **요약 블록을 두 번 안 센다.** 위 입력의 `---- 또_깨진것 stdout ----` 과
    /// `failures:` 목록은 `test … ... FAILED` 형태가 아니므로 안 걸려야 한다 — 걸리면
    /// 같은 이름이 두 번 세지고, 그러면 등록 대조가 뜻을 잃는다.
    #[test]
    fn 통과만_있으면_비어_있다() {
        let out = "test a ... ok\ntest b ... ok\ntest result: ok. 2 passed;\n";
        assert!(실패한_시험들(out).is_empty());
        // 그리고 「무시됨」도 실패가 아니다.
        assert!(실패한_시험들("test c ... ignored\n").is_empty());
    }

    /// ★ **등록 목록이 이 플랫폼에서 뜻이 있어야 한다.**
    ///
    /// 유닉스에서는 비어야 한다 — 거기서는 전부 재진다. 비어 있지 않으면 유닉스 쪽에
    /// 안 재지는 것이 생긴 것이고, 그것은 등록이 아니라 **결함**이다.
    #[test]
    #[cfg(unix)]
    fn 유닉스에는_안_재지는_것이_없다() {
        assert!(
            등록된_외침().is_empty(),
            "유닉스에 안 재지는 것이 등록됐다 — 그것은 등록할 것이 아니라 고칠 것이다: {:?}",
            등록된_외침()
        );
    }

    /// ★ **음성 대조 ① — 등록 안 된 것이 깨지면 걸린다.**
    ///
    /// 이 줄이 없으면 이 명령은 *"언제나 통과"* 일 수 있고, 그러면 CI 가 아무것도 안 센다.
    #[test]
    fn 등록_안_된_실패가_걸린다() {
        let 등록 = [("외침A", "까닭")];
        let 문제 = 등록과_댄다(&["외침A".to_owned(), "새로깨진것".to_owned()], &등록);
        assert_eq!(문제.len(), 1, "{문제:?}");
        assert!(문제[0].contains("새로깨진것"), "{문제:?}");
    }

    /// ★ **음성 대조 ② — 승격하고 등록을 안 지우면 걸린다.**
    ///
    /// 이쪽이 더 조용한 실패 경로다. 시험이 초록이 됐는데 목록이 *"이건 안 재진다"* 를
    /// 계속 주장하면 **없는 사실이 문서로 산다.**
    #[test]
    fn 승격됐는데_등록이_남으면_걸린다() {
        let 등록 = [("외침A", "까닭"), ("이제통과", "옛 까닭")];
        let 문제 = 등록과_댄다(&["외침A".to_owned()], &등록);
        assert_eq!(문제.len(), 1, "{문제:?}");
        assert!(문제[0].contains("이제통과") && 문제[0].contains("지우십시오"), "{문제:?}");
    }

    /// 같으면 아무 말도 안 한다 — 그리고 둘 다 비어도 조용하다(유닉스가 그 상태다).
    #[test]
    fn 같으면_조용하다() {
        let 등록 = [("외침A", "까닭"), ("외침B", "까닭")];
        assert!(등록과_댄다(&["외침A".to_owned(), "외침B".to_owned()], &등록).is_empty());
        assert!(등록과_댄다(&[], &[]).is_empty());
    }

    /// ★ **등록에는 언제나 까닭이 붙는다.** 까닭 없는 등록은 「그냥 빨간 것」이 된다.
    #[test]
    fn 등록에는_까닭이_있다() {
        assert!(!외침.is_empty(), "목록이 비었다 — 이 대조가 아무것도 안 센다");
        for (플랫폼, 이름, 왜) in 외침 {
            assert!(!플랫폼.is_empty() && !이름.is_empty(), "빈 등록이 있다");
            assert!(왜.len() > 20, "`{이름}` 의 까닭이 너무 짧다: {왜}");
        }
    }
}
