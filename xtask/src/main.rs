//! CI 검사 — **단계 1**(stack §4.3). 전부 S 규모이고 외부 의존을 늘리지 않는다.
//!
//! 여기 있는 일곱 중 둘은 옛 계획 §2 가 *"되돌릴 수 없는 것"* 으로 분류한 것이다.
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

/// 종료 보고의 파일 위치가 정해지기 전에 실제로 닫힌 유일한 회차.
const 종료보고_형식이전_회차: &str = "2026-08-18-round-protocol";

fn 종료했나(회차_디렉터리: &Path) -> bool {
    if 회차_디렉터리.join("report.md").is_file() {
        return true;
    }
    if 회차_디렉터리.file_name().and_then(|x| x.to_str()) != Some(종료보고_형식이전_회차) {
        return false;
    }
    std::fs::read_to_string(회차_디렉터리.join("state.md"))
        .map(|본문| 본문.lines().any(|줄| {
            줄.trim() == "**단계**: 종료. 완수 조건 전부 닫힘 · 효과 관측 · CI 초록."
        }))
        .unwrap_or(false)
}

/// 회차의 **기록이 확정됐나** — 종료(`report.md`)든 접힘(`folded.md`)이든.
///
/// ★ **둘을 같이 봐야 하는 자리와 갈라 봐야 하는 자리가 있다.** (2026-08-24)
/// 「진행 중인가」를 묻는 자리는 **둘 다** 확정으로 봐야 한다 — 접힌 회차를 「진행 중」으로
/// 두면 다음 사람이 그것을 이어받아야 할 일로 읽고 **접은 회차를 되살린다.**
/// 「종료 보고를 썼나」를 묻는 자리는 `report.md` 만 본다.
fn 기록이_확정됐나(회차_디렉터리: &std::path::Path) -> bool {
    종료했나(회차_디렉터리) || 회차_디렉터리.join("folded.md").is_file()
}

fn main() -> Result<()> {
    let root = 뿌리를_고른다()?;
    let 명령 = 명령을_고른다()?;
    match 명령.as_deref() {
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
// 인자 — **뿌리를 받고, 모르는 것을 거부한다** (사전부검 R2·R3 · 2026-08-24)
// ─────────────────────────────────────────────────────────────────────────────
//
// ★★ **왜 뿌리를 인자로 받나.** [`repo_root`] 는 `CARGO_MANIFEST_DIR` 로 **컴파일
//    시점에 박힌다.** 그래서 회차의 음성 대조를 「격리 사본」에서 걸어도, 프리빌트
//    바이너리를 사본에서 돌리면 **원본을 잰다** — 실측(사전부검 R2): 사본의
//    `findings.jsonl` 을 통째로 파괴하고 사본 cwd 에서 돌렸는데 **원본의 수 · 21/21
//    통과**가 나왔다. 그 관측이 「안 빨개졌다」로 읽히면 음성 대조가 통째로 헛것이 된다.
//
// ⚠⚠ **그리고 모르는 인자를 거부해야 한다.** 앞 판은 `args().nth(1)` 만 봤고 그 뒤를
//    **읽지도 거부하지도 않았다.** 그래서 `cargo xtask check --root /tmp` 가 **에러 없이
//    21/21 통과하고 원본의 수**를 냈다(실측 · 사전부검 R3). 플래그를 준 것처럼 보이면서
//    같은 일을 한다 — **뿌리가 없는 것보다 나쁘다.**
//
//    새 범주로 적는다: **「무시되는 인자」 — 「사본을 쟀다」가 문면으로만 참이 되는 자리.**
//
// ★ 사본에서 걸 때는 **사본에서 재빌드하거나** `--root <사본>` 을 준다. 둘 중 하나다.

/// `--root <경로>` 가 있으면 그것을, 없으면 [`repo_root`] 를 뿌리로 쓴다.
fn 뿌리를_고른다() -> Result<PathBuf> {
    let 인자: Vec<String> = std::env::args().skip(1).collect();
    let Some(i) = 인자.iter().position(|a| a == "--root") else {
        return repo_root();
    };
    let 값 = 인자.get(i + 1).context("`--root` 뒤에 경로가 없다")?;
    let p = PathBuf::from(값);
    if !p.is_dir() {
        bail!("`--root` 가 디렉터리가 아니다: {}", p.display());
    }
    Ok(p.canonicalize().unwrap_or(p))
}

/// 인자에서 명령 하나를 고른다. **남는 것이 있으면 실패다 — 조용히 안 무시한다.**
fn 명령을_고른다() -> Result<Option<String>> {
    let mut 남은: Vec<String> = Vec::new();
    let 인자: Vec<String> = std::env::args().skip(1).collect();
    let mut i = 0;
    while i < 인자.len() {
        if 인자[i] == "--root" {
            i += 2; // 값까지 건너뛴다 — 없으면 `뿌리를_고른다` 가 이미 실패했다
            continue;
        }
        남은.push(인자[i].clone());
        i += 1;
    }
    if 남은.len() > 1 {
        bail!(
            "인자가 남는다: {:?} — **조용히 무시하지 않는다.** 무시되는 인자는 \
             「사본을 쟀다」를 문면으로만 참으로 만든다 (사전부검 R3)",
            &남은[1..]
        );
    }
    Ok(남은.into_iter().next())
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
//
// # ⚠ 그런데 집합만으로는 부족했다 — **빈 집합이 초록으로 읽혔다** (2026-08-17)
//
// 앞 판은 자식의 `ExitStatus` 를 **어디서도 안 봤다.** 위 표는 *"stdout 에
// `test … ... FAILED` 가 난다"* 를 조용히 전제하고 있었고, **그 줄이 한 줄도 안 나는
// 실패**에 대해서는 아무 주장도 하지 않았다. 컴파일이 서지 못하면 시험 바이너리가
// 하나도 안 돌고, 그러면 「실패 0건 = 등록 0건」이 되어 **초록**이다. CI 세 OS 가 이
// 명령 하나로 판정하므로 **셋이 함께 속는다.**
//
// 그렇다고 종료 상태만 보면 안 된다 — **rc≠0 이 이 저장소에서는 초록의 정상 상태**다
// (등록된 외침이 그대로 나면 `cargo test` 는 실패로 끝난다). 그래서 [`판정한다`] 가
// 둘을 **함께** 보고, 「시험이 실패했다」와 「시험을 돌리지도 못했다」를 **다른 문구로**
// 낸다 — 사람이 할 일이 다르기 때문이다(앞은 시험을, 뒤는 `error[E…]` 를 읽는다).
//
// # ⚠ 그리고 그 첫 수선에도 같은 과의 구멍이 남아 있었다 — **rc=0 이 면제였다**
//
// 첫 수선은 보고 유무를 `r.섰나 || 시험이_돌았나(…)` 로 물었다. `||` 때문에 **rc=0 인
// 호출은 보고 검사를 통째로 건너뛴다.** 그런데 시험을 하나도 안 돌리고 rc=0 으로 끝나는
// 길이 있다 — 실측(2026-08-17): 축 하나의 인자를 `--no-run` 으로 바꾸자 통과 수가
// **753 → 3** 으로 줄고 `test result:` 가 **41줄 → 7줄** 이 됐는데도 `cargo xtask test`
// 는 **rc=0 · "시험 통과"** 를 냈다. 그래서 이제 **보고는 rc 와 무관하게 매 호출마다**
// 있어야 한다. 무동작과 무보고가 초록으로 읽히는 자리를 하나 더 닫는다.
//
// # 그리고 축이 둘이다 — **doctest 는 `--all-targets` 에 안 든다**
//
// `--doc` 과 `--all-targets` 는 같이 못 쓴다(`error: can't mix --doc with other target
// selecting options`). 그래서 `cargo test` 를 **두 번** 부르고 두 화면을 합쳐 센다.
//
// ★ **`--all-targets` 를 빼서 한 줄로 줄이지 않는다.** 빼면 명시적 타깃 계약을 암묵
// 기본값에 넘기는 것이고, `benches/` 가 생기는 날 조용히 커버리지가 준다.
//
// ⚠ **한계 셋을 적어 둔다:**
//
// 1. `--doc` 은 **lib 타깃만** 본다. `pal-cli` 와 `xtask` 는 lib 이 없으므로 **doctest 가
//    영원히 0** 이다 — 거기 `///` 예제를 적어도 **아무도 컴파일하지 않는다.** 사용자가
//    만나는 표면이 바로 그 `pal-cli` 다.
// 2. doctest 이름에는 **줄번호가 박힌다**(`… (line 177)`). 위에 주석 한 줄만 넣어도
//    이름이 바뀌므로, doctest 를 `외침` 에 등록하면 **양방향 판정이 동시에 운다.**
// 3. ★ **그 이름의 경로 구분자가 OS 마다 갈린다 — 이제 쟀다** (2026-08-17). CI 회차
//    `31997140887`(워크플로 `CI` · 커밋 `095c56c` · 7잡 전부 success)의 **로그 본문**:
//
//    | | ubuntu-latest | macos-latest | windows-latest |
//    |---|---|---|---|
//    | doctest 이름의 경로 구분자 | `/` | `/` | **`\`** |
//
//    windows 로그 원문 한 줄:
//    `test crates\pal-core\src\coord.rs - coord::SymbolIdentity (line 177) ... ok`
//    ubuntu·macos 는 같은 줄이 `crates/pal-core/src/coord.rs` 로 나고, 그 둘의 시험
//    이름 목록은 **바이트 단위로 동일**했다(`diff`). 셋 다 `Doc-tests` 여섯 줄 ·
//    doctest 결과 세 줄이다 — **갈리는 것은 수가 아니라 이름이다.**
//
//    ⚠ **귀결: 이름 문자열 하나로는 세 OS 를 못 덮는다.** [`등록과_댄다`] 가 `n == name`
//    **정확 일치**로 양방향을 대므로, 슬래시로 적은 등록은 windows 에서 「등록됐는데 안
//    났다」와 「등록되지 않은 실패」를 **한 번에** 낸다(역슬래시로 적으면 나머지 둘에서
//    같은 일이 난다). 그리고 이것은 2 의 줄번호와 **곱해진다.**
//
//    ★ **여기에 처방을 지어 적지 않는다.** 시험 이름을 정규화하는 장치는 **없다** —
//    [`상대_경로`] 가 같은 모양의 문제를 풀지만 그것은 **검사 쪽 파일 경로**용이고, 이
//    경로([`실패한_시험들`] → [`등록과_댄다`])에는 걸려 있지 않다. 없는 것을 있는 것처럼
//    적으면 다음 사람이 그것을 찾다가 시간을 버린다.

/// 등록이 가리킬 수 있는 **플랫폼 — 둘뿐이고, 여기 없는 것은 적을 수가 없다.**
///
/// # 왜 `&str` 이 아닌가 — **오타가 등록을 통째로 무동작으로 만들었다** (2026-08-17)
///
/// 앞 판은 이 자리가 문자열이었고 [`등록된_외침`] 이 `"windows"`·`"unix"`·`"그밖"` 과
/// **정확 일치**로 걸렀다. 그래서 `"linux"` 나 `"win"` 으로 적으면 **어느 플랫폼에서도
/// 안 걸러져 등록이 통째로 무동작**인데 아무것도 안 울었다:
///
/// - [`등록된_외침`] 이 빈 벡터를 내므로 [`어느_플랫폼에도_안_재지는_것이_없다`] 가 **초록**.
/// - [`등록은_원리상_불가능한_것만_담는다`] 는 날것의 `외침` 을 훑고 이 필드를 **보긴 봤다** —
///   `!플랫폼.is_empty()`. 그러나 **비었는지만 보고 값이 무엇인지는 안 봤다.** `"linux"` 는
///   비어 있지 않으므로 통과한다. 나머지 둘(까닭이 20 넘나 · 금지 낱말이 있나)도 이 필드와
///   무관하다. 그래서 **초록**.
/// - `외침` 이 그래도 비어 있지 않으니 [`등록과_댄다`] 는 **부를 일이 없다.**
///
/// 실측: `("linux", …)` 하나를 넣고 `cargo test -p xtask` → **15 통과 · 0 실패 · rc=0.**
///
/// # 타입으로 닫으면 무엇이 달라지나
///
/// **없는 변종은 적을 수가 없다** — 오타는 시험이 아니라 **컴파일이 운다**(`error[E0599]`).
/// 그리고 그것이 **다른 방어 하나를 되살린다**: 모든 등록이 [`여기`] 가 실제로 고르는 값
/// 둘 중 하나이므로, 시험 **이름** 이 틀리면 그 플랫폼에서 [`등록과_댄다`] 가 양방향으로
/// 운다(「등록되지 않은 실패」 + 「등록됐는데 안 났다」). 앞 판은 플랫폼 오타 하나가 그
/// 방어까지 함께 껐다. 그리고 CI 는 `windows-latest`·`ubuntu-latest`·`macos-latest` 셋을
/// 다 도므로 **두 값 다 실제로 골라진다.**
///
/// # ⚠ 그리고 **셋째 값을 지웠다** — 그것 하나가 같은 구멍을 그대로 남겼다 (2026-08-17)
///
/// 이 enum 의 첫 판에는 `그밖` 이 있었다. [`여기`] 는 `windows` 도 `unix` 도 아닌 호스트에서만
/// 그것을 고르는데 **이 저장소의 대상 셋 중 그런 것이 없다.** 그래서 `그밖` 으로 적은 등록은
/// **어느 플랫폼에서도 안 골라져 통째로 무동작**이고, 위 세 줄이 그대로 다시 성립했다 —
/// 오타가 아니라 **도달 불가능한 유효값**이라 컴파일도 안 울었다. 실측: `(플랫폼::그밖, …)`
/// 하나를 넣고 **16 통과 · rc=0**, 그리고 `cargo xtask test` 가 등록이 있는데도
/// *"이 플랫폼에는 안 재지는 것이 없다"* 를 냈다.
///
/// **그래서 변종을 지웠다.** 대상이 아닌 호스트는 이제 [`여기`] 옆의 `compile_error!` 가
/// 멈춘다 — 조용히 틀린 값을 고르는 것보다 서지 않는 편이 낫다.
#[derive(Clone, Copy, PartialEq, Eq)]
enum 플랫폼 {
    윈도우,
    유닉스,
}

/// 지금 이 빌드가 서 있는 플랫폼 — [`외침`] 의 등록을 거르는 값.
///
/// ★ **`cfg!` 로 고른다(`#[cfg]` 가 아니라).** 그래야 두 변종이 **모든 빌드에서** 지어지고,
/// 어느 한쪽이 `dead_code` 로 조용히 사라지지 않는다.
///
/// ★ **그리고 셋째 갈래가 없다.** 아래 `compile_error!` 가 대상 밖 호스트를 컴파일에서
/// 멈추므로 `else` 는 **유닉스임이 보장된 자리**다. 갈래를 하나 더 두면 그 갈래에 등록된
/// 것이 어디서도 안 골라져 **무동작이 초록으로 읽힌다** — 그것이 이 회차가 지운 것이다.
fn 여기() -> 플랫폼 {
    if cfg!(windows) { 플랫폼::윈도우 } else { 플랫폼::유닉스 }
}

#[cfg(not(any(windows, unix)))]
compile_error!(
    "이 저장소는 windows 와 unix 만 대상으로 한다(AGENTS.md · F24-크로스플랫폼 게이트). \
     그 밖의 호스트에서는 `여기` 가 고를 값이 없고, 그러면 `외침` 의 등록이 통째로 \
     무동작이 되면서 `cargo xtask test` 가 조용히 초록을 낸다. 서지 않는 편이 낫다."
);

/// **이 플랫폼에서 안 재지는 것** — `(플랫폼, 시험 이름, 왜 못 재나)`.
///
/// ⚠ 여기 있는 것은 전부 **제품의 결함이 아니라 fixture 의 한계**다. 결함이면 고치지
/// 등록하지 않는다.
/// ★ **비어 있다** (2026-08-17). 그리고 비어 있는 것이 이 목록의 목표 상태다.
///
/// # 다섯이 어떻게 없어졌는가 — **하나도 「고쳐서」가 아니라 「갈라서」였다**
///
/// | 없어진 등록 | 무엇이었나 | 어디로 갔나 |
/// |---|---|---|
/// | `모드와_심링크_보존이_…` | 성질 **셋이 한 덩어리**였다. 이식 가능한 둘이 못 재는 하나에 끌려 통째로 외침이었다 | 심링크 축은 `심링크가_살고_그_대상에_쓰인다`(이식) · 모드 축은 `모드가_살아_있다`(유닉스 인코딩) |
/// | `쓰기_불가_디렉터리가_…` | *"진짜 쓰기 불가 디렉터리는 ACL 이고 **std 밖이다**"* — 관측은 맞았고 **결론이 틀렸다** | `icacls` fixture. junction 이 `cmd` 를 쓰는 것과 같은 자격이다 |
/// | `파일_심링크_경계가_…` | **플랫폼의 한계가 아니라 기계의 준비 상태**였다 | 개발자 모드를 켜니 `symlink_file` 이 그 자리에서 섰다. fixture 를 `심링크()` 하나로 모았다 |
/// | `파이프_방어가_…` | 재려는 성질은 「FIFO」가 아니라 **「일반 파일이 아닌 자리」**였다 | 디렉터리로 이식(`일반_파일이_아닌_자리에서_매달리지_않고_실패한다`). FIFO 시험은 **더 센 사실**(매달림)을 위해 남는다 |
/// | `끊었다는_말이_…` | 결과는 이미 같았고 **말할 것이 더 많은 쪽이 침묵**했다 | 못 세는 플랫폼이 *"모르니 늘 끊는다"* 를 자리 목록과 함께 낸다 |
///
/// 다섯 중 **넷이 「안 한 것」이었고 하나(모드 비트)만 「없는 것」**이다. 그리고 그
/// 하나조차 이 목록에 안 남는다 — 성질을 갈라 보니 그 축의 이식 가능한 문장이
/// 이미 다른 시험에 있었다.
///
/// # 그래도 이 목록을 안 지우는 이유
///
/// 다음에 짝 없는 `cfg` 가 생기면 **여기 등록되지 않은 실패**로 걸린다. 목록이 빈
/// 것과 장치가 없는 것은 다르다.
///
/// ⚠ **여기 들어올 자격**: 「이 플랫폼에서 **원리상** 못 잰다」뿐이다. 「아직 안
/// 했다」는 자격이 없고, [`등록은_원리상_불가능한_것만_담는다`] 가 그것을 판정한다.
const 외침: &[(플랫폼, &str, &str)] = &[];

/// 지금 플랫폼에서 등록된 외침.
fn 등록된_외침() -> Vec<(&'static str, &'static str)> {
    let 여기 = 여기();
    외침.iter().filter(|(p, ..)| *p == 여기).map(|(_, n, w)| (*n, *w)).collect()
}

/// 자식 `cargo` 하나가 낸 것 중 **판정에 필요한 전부**.
struct 돌린_결과 {
    /// 종료 상태가 성공인가. ★ `false` 가 곧 빨강은 아니다 — [`판정한다`] 를 보라.
    섰나: bool,
    /// stdout 전문. 실패 이름([`실패한_시험들`])과 시험 보고([`시험이_돌았나`])를 여기서 긁는다.
    화면: String,
}

/// `cargo <args>` 를 `root` 에서 돌리고 **출력을 그대로 화면에 흘린다.**
///
/// ★ **흘리는 것을 없애지 마라.** 이 명령이 무엇을 감췄는지 사람이 볼 수 있어야 한다 —
/// 판정만 내고 증거를 숨기면 그것이 곧 조용한 실패다.
fn 돌린다(root: &Path, args: &[&str]) -> Result<돌린_결과> {
    let out = Command::new(env!("CARGO"))
        .args(args)
        .current_dir(root)
        .output()
        .context("cargo test 를 돌리지 못했다")?;

    print!("{}", String::from_utf8_lossy(&out.stdout));
    eprint!("{}", String::from_utf8_lossy(&out.stderr));

    Ok(돌린_결과 {
        섰나: out.status.success(),
        화면: String::from_utf8_lossy(&out.stdout).into_owned(),
    })
}

fn test(root: &Path) -> Result<()> {
    println!("■ 시험 — 그리고 남는 실패가 등록된 외침과 같은지 본다");

    // ★ **축이 둘이고, 둘을 한 번에 못 부른다.**
    //
    //     $ cargo test --workspace --all-targets --doc --no-fail-fast
    //     error: can't mix --doc with other target selecting options   (rc=101, 실측)
    //
    // ⚠ **`--all-targets` 를 빼서 한 줄로 줄이지 마라.** 그러면 명시적 타깃 계약을
    // 암묵 기본값에 넘기는 것이고, `benches/` 가 생기는 날 **조용히 커버리지가 준다.**
    // 두 번 부르는 쪽이 계약을 지킨다.
    //
    // ★ **그리고 이제 그 계약에 우는 것이 붙었다** (2026-08-17). 위 두 문단은 오래
    // **주석으로만** 있었고 아무것도 안 막았다 — `7036909` 이전이 정확히 그 상태다
    // (`--doc` 축이 없어 doctest 셋이 **CI 에서 한 번도 안 돌았고**, 그래도 초록이었다).
    // AGENTS.md 가 그것을 한 줄로 적는다: *"적어 둔 주석은 아무것도 안 막는다."*
    // 이제 둘이 막는다:
    //
    // 1. **길이가 타입에 있다**(`[_; 2]`). 축을 지우면 **컴파일이 운다.**
    // 2. **무엇을 도는지는 [`축과_댄다`] 가 댄다.** 한 축을 다른 축으로 바꿔치면 길이는
    //    그대로라 1 이 못 잡는다.
    let 축: [[&str; 4]; 2] = [
        ["test", "--workspace", "--all-targets", "--no-fail-fast"],
        ["test", "--workspace", "--doc", "--no-fail-fast"],
    ];
    // ★ **돌리기 전에 댄다.** 계약이 깨진 채로 돌면 그 산출은 무엇에 대한 초록인지
    // 아무도 모른다.
    let 어긋남 = 축과_댄다(&축);
    if !어긋남.is_empty() {
        bail!("축 집합이 계약과 다르다:\n    {}", 어긋남.join("\n    "));
    }

    let 결과: Vec<돌린_결과> =
        축.iter().map(|args| 돌린다(root, args)).collect::<Result<_>>()?;

    // ★ **하나라도 안 서면 안 선 것이다.**
    let 모두_섰나 = 결과.iter().all(|r| r.섰나);
    // ★ **보고 유무는 「호출마다」 따로 보고, `rc` 가 그것을 면제하지 못한다.** 합친
    // 화면으로 보면 한 축이 통과해 보고를 낸 것이 다른 축의 침묵을 덮어 준다.
    //
    // ⚠ 앞 판은 여기가 `r.섰나 || 시험이_돌았나(…)` 였다 — `||` 가 **rc=0 인 호출에
    // 대해 보고 검사를 통째로 건너뛰었다.** 그래서 한 축이 시험을 하나도 안 돌리고
    // rc=0 으로 끝나면 그 축은 아무 말도 안 하고, `외침` 이 비어 있으므로 집합 대조도
    // 침묵했다 — **빈 화면이 빈 집합으로 읽혔다.**
    //
    // 실측(2026-08-17, macOS): 축 하나의 인자를 `--no-fail-fast` 에서 `--no-run` 으로
    // 바꾸자 통과 수가 **753 → 3**(doctest 셋만) · `test result:` **41줄 → 7줄** 로
    // 줄었는데 `cargo xtask test` 는 **rc=0 · "시험 통과"** 를 냈다.
    let 보고가_다_있나 = 결과.iter().all(|r| 시험이_돌았나(&r.화면));

    // **같은 이름이 여러 시험 바이너리에서 날 수 있다** — 집합으로 센다.
    // doctest 도 `test <이름> ... FAILED` 한 줄 형태를 따르므로 파서는 그대로 쓴다.
    let mut 실패: Vec<String> = 결과
        .iter()
        .flat_map(|r| 실패한_시험들(&r.화면))
        .map(str::to_owned)
        .collect();
    실패.sort();
    실패.dedup();
    let 등록 = 등록된_외침();

    match 판정한다(모두_섰나, 보고가_다_있나, &실패, &등록) {
        판정::시험을_못_돌렸다 => bail!(
            "**시험을 돌리지도 못했다** — 어느 호출인가가 시험 바이너리의 \
             보고(`test result:`)를 **한 줄도 안 냈다.**\n    \
             ⚠ 이것은 「시험이 실패했다」가 **아니다.** 갈래가 둘이고 사람이 읽을 곳이 \
             다르다 — rc≠0 이면 컴파일·링크가 서지 못한 것이니 위 stderr 의 \
             `error[E…]` 를 읽고, **rc=0 이면 그 호출이 시험을 하나도 안 돌린 것**이니 \
             위 `축` 의 인자를 읽으십시오.\n    \
             실패 집합은 비어 있고, 그 빈 집합은 **초록의 근거가 아니다.**"
        ),
        판정::어긋났다(problems) => {
            bail!("시험 결과가 등록과 다르다:\n    {}", problems.join("\n    "))
        }
        판정::통과 => {}
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

/// 이 명령이 낼 수 있는 **셋** — 그리고 셋을 가르는 것이 이 커밋의 값이다.
///
/// ⚠ **뒤의 둘을 같은 문구로 내면 안 된다.** 「시험이 실패했다」와 「시험을 돌리지도
/// 못했다」는 사람이 할 일이 다르다 — 앞은 시험을 읽고, 뒤는 `error[E…]` 를 읽는다.
#[derive(Debug, PartialEq, Eq)]
enum 판정 {
    /// 초록. **rc≠0 이어도 여기 올 수 있다** — 등록된 외침이 그대로 났으면 그것이 정상이다.
    통과,
    /// **시험이 아예 안 돌았다.** 컴파일·링크가 서지 못했다.
    시험을_못_돌렸다,
    /// 시험은 돌았고, 실패 집합이 등록 집합과 어긋났다.
    어긋났다(Vec<String>),
}

/// 화면에 **시험 바이너리의 보고**가 있는가 — `test result:` 한 줄이라도.
///
/// 없으면 시험이 하나도 안 돈 것이다. 그때 stdout 에는 `test … ... FAILED` 가 **한 줄도
/// 안 난다** — 실패 집합이 비는 것이 아니라 **아무 집합도 없다.**
///
/// ⚠ **rc 로는 그 상태를 못 가른다.** 컴파일이 서지 못하면 rc≠0 이지만, 시험을 안 돌리고도
/// rc=0 으로 끝나는 길이 있다(실측: 축 하나에 `--no-run` 이 들어간 상태). 그래서
/// [`판정한다`] 는 이 물음을 **rc 와 무관하게** 묻는다.
///
/// **순수 함수다.**
fn 시험이_돌았나(화면: &str) -> bool {
    화면.lines().any(|l| l.starts_with("test result:"))
}

/// 자식이 낸 것 전부 — **종료 상태 · 보고 유무 · 실패 이름 · 등록** — 를 한 번에 판정한다.
///
/// # 왜 집합 대조만으로는 부족한가 — **빈 집합이 초록으로 읽힌다**
///
/// 앞 판은 자식의 `ExitStatus` 를 **어디서도 보지 않았다.** 오직 stdout 의
/// `test <이름> ... FAILED` 줄을 긁어 등록 집합과 댔다. 그래서 **그 줄이 하나도 안 나는
/// 실패** — 워크스페이스가 컴파일에 실패한 경우가 대표다 — 는 「실패 0건 = 등록 0건」이
/// 되어 **초록으로 통과했다.** CI 세 OS 가 이 명령 하나로 판정하므로 셋이 함께 속는다.
///
/// # 왜 종료 상태만으로도 부족한가 — **rc≠0 이 정상이다**
///
/// 이 저장소는 **일부러 실패하는 시험**을 둔다(위 `외침`). 등록된 외침이 하나라도 있으면
/// `cargo test` 는 rc≠0 을 내고, 그것이 **초록의 정상 상태**다. rc 만 보면 그 설계가 죽는다.
///
/// # ★ 그리고 **rc=0 이 보고를 면제하지 못한다**
///
/// 앞 판은 `!섰나 && !시험이_돌았나` 로 물어서, **rc=0 이면 보고 유무를 아예 안 봤다.**
/// 그런데 시험을 하나도 안 돌리고 rc=0 으로 끝나는 길이 있다(실측: 축 하나에 `--no-run`
/// 이 들어간 상태 — 통과 수 **753 → 3**, 그래도 초록이었다). 그 상태에서 실패 집합은
/// 비고, `외침` 이 비어 있으면 집합 대조도 침묵한다. **아무도 무언가를 잰 적이 없는데
/// 초록이 난다.** 그래서 보고 부재는 **rc 보다 앞서** 판정한다.
///
/// ⚠ **이것이 막는 것의 한계를 정확히 적는다.** 이 함수는 「보고가 **한 줄도** 없다」만
/// 잡는다. 보고가 있으면서 그 안의 통과 수가 0 인 것(예: 필터가 전부 걸러낸 경우)은 **여기
/// 안 걸린다** — 통과 수의 하한선은 이 저장소 어디에도 등록돼 있지 않고, 이 회차는 그
/// 합격선을 새로 세우지 않는다.
///
/// # 그래서 둘을 함께 본다
///
/// | rc | 시험 보고 | 긁힌 실패 | 판정 |
/// |---|---|---|---|
/// | 0 | **없다** | — | [`판정::시험을_못_돌렸다`] — ★ 이 회차가 막은 자리 |
/// | 0 | 있다 | — | 집합 대조에 맡긴다 |
/// | ≠0 | **없다** | — | [`판정::시험을_못_돌렸다`] |
/// | ≠0 | 있다 | **비었다** | [`판정::어긋났다`] — rc 가 실패인데 이름이 안 났다 |
/// | ≠0 | 있다 | 있다 | 집합 대조에 맡긴다 (등록과 같으면 **초록**) |
///
/// **순수 함수다** — 그래야 이 빈틈 하나하나를 시험으로 세울 수 있다.
fn 판정한다(
    섰나: bool,
    시험이_돌았나: bool,
    실패: &[String],
    등록: &[(&'static str, &'static str)],
) -> 판정 {
    if !시험이_돌았나 {
        return 판정::시험을_못_돌렸다;
    }
    let mut problems = 등록과_댄다(실패, 등록);
    if !섰나 && 실패.is_empty() {
        problems.push(
            "`cargo test` 가 rc≠0 인데 **실패한 시험 이름이 하나도 안 났다** — 시험 밖에서 \
             무언가 무너졌다(바이너리 abort · 링크 · 하네스). 위 출력 전문을 읽으십시오"
                .to_owned(),
        );
    }
    if problems.is_empty() { 판정::통과 } else { 판정::어긋났다(problems) }
}

/// 축 집합이 **계약대로인가** — 어긋난 것마다 한 문장.
///
/// # 왜 이것이 없으면 축이 조용히 사라지나
///
/// [`판정한다`] 는 **돌린 축들**만 본다. `보고가_다_있나` 가 `결과` 에 대한 `all()` 이라
/// **남은 축만 대고**, 빈 `iterator` 의 `all()` 은 `true` 다. 그래서 축을 하나 빼면 통과
/// 수가 **754 에서 3 으로** 떨어져도 초록이고, 축이 **0 개면 완전 초록**이다.
///
/// ⚠ **이것은 가상이 아니다** — `7036909` 이전이 정확히 그 상태였다. `--doc` 축이 없어
/// doctest 셋이 **CI 에서 한 번도 안 돌았고**, 세 OS 가 그것을 초록으로 셌다.
///
/// # ★ 이것은 새 합격선이 아니다
///
/// 「축이 둘이고 그 둘이 무엇인가」는 [`test`] 위에 **오래 ★ 표시로 적혀 온 계약**이다.
/// 없던 것은 그 계약에 **우는 것**뿐이었다. 통과 **수**의 하한선은 여전히 아무도
/// 주장하지 않는다 — 그것은 등록된 적 없는 수이고 여기서 세우지 않는다.
///
/// ⚠ **[`test`] 안에서 부른다. 시험에서만 부르면 안 된다** — `--all-targets` 축이
/// 빠지면 이 파일의 단위 시험 자체가 안 돌기 때문이다. 우는 자리는 **돌리는 경로 위**여야
/// 한다.
///
/// **순수 함수다** — 그래야 음성 대조를 시험으로 세울 수 있다.
fn 축과_댄다(축: &[[&str; 4]]) -> Vec<String> {
    let mut problems = Vec::new();
    for 있어야_하는_것 in ["--all-targets", "--doc"] {
        if !축.iter().any(|a| a.contains(&있어야_하는_것)) {
            problems.push(format!(
                "축에 `{있어야_하는_것}` 이 없다 — 그 축이 재던 것이 **조용히 안 돈다.** \
                 둘은 한 번에 못 부르므로(`can't mix --doc with other target selecting \
                 options`) `cargo test` 를 **두 번** 부르는 것이 이 명령의 계약이다"
            ));
        }
    }
    problems
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
        ("죽은 링크 부재", check_dead_links(root)),
        ("sunset 선언", check_sunset(root)),
        ("사라진 문서를 현재형으로 안 부른다", check_stale_citation(root)),
        ("회차 레코드", check_round_records(root)),
        ("원장 둘 대조", check_ledger_pair(root)),
        ("발견이 닫혔나", check_finding_closure(root)),
        ("선언 목록이 닫혀 있나", check_declared_lists(root)),
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

// ── 검사 7 — 스키마 정합 (stack §4.3 단계 2 · 옛 DESIGN §1.2) ────────────────────

/// `schema/graph.toml` ↔ 코드. **양방향이다.**
///
/// | 방향 | 무엇을 막나 |
/// |---|---|
/// | 코드 → 스키마 | 급할 때 코드에만 노드를 만드는 것(옛 F22 §4) |
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

    // **로딩 시점 거부가 여기서 CI 실패가 된다** (옛 DESIGN §3.4).
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

// ── 검사 11 — 카탈로그 정합 (옛 F06 §2 · `[f06.1.pass]` ①) ─────────────────────
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

/// 방향 4 가 훑는 **표면 소스 전부.**
///
/// 질의 이름을 리터럴로 쓸 수 있는 자리는 여기뿐이고, **여기 없는 표면은 안 재어진다.**
/// F06b 가 어댑터를 더하면서 둘이 됐다가, **2026-08-18 재고 처분이 어댑터를 지우면서
/// 다시 하나가 됐다**(ADR-0025 — 호스트가 하나면 MCP 는 값을 잃는다).
const SURFACE_SOURCES: &[&str] = &["crates/pal-cli/src"];

/// **하한** — 이 수보다 적은 표면을 훑으면 방향 4 는 아무것도 안 세고 통과한다.
///
/// ⚠ **이것이 없으면 목록을 비우는 것만으로 검사가 초록이 된다**(실측 2026-08-17:
/// `SURFACE_SOURCES` 를 `&[]` 로 두니 *"표면 0곳의 소스 0개에 박힌 이름 0건"* 으로
/// **16/16 통과**). 「없는 디렉터리를 문제로 적는다」는 **목록에 적힌 것**만 막고
/// **빈 목록**은 못 막는다 — `CATALOG_MIN_QUERIES` 와 같은 자리이고 같은 이유다.
///
/// 수는 **지금 서 있는 표면의 수**다. 표면이 늘면 이 수도 같이 올린다.
///
/// ★ **2026-08-18 에 2 → 1 로 내렸다.** 어댑터(`crates/pal-mcp/src`)가 사라졌으므로
/// 표면이 진짜로 하나가 됐고, **수를 따라 내리는 것이 올리는 것과 대칭**이다.
/// ⚠ **음성 대조는 안 약해졌다** — 위 실측이 잡은 형태는 목록을 `&[]` 로 **비우는**
/// 것이고 `0 < 1` 이라 여전히 실패한다. 이 상수가 막는 것은 「빈 목록」이지
/// 「표면이 하나뿐인 세계」가 아니다.
///
/// ⚠ **`crates/pal-cli/assets` 는 표면으로 못 넣는다** — `rust_sources` 가 `.rs` 만
/// 모으는데 거기엔 마크다운뿐이고, 넣으면 *"Rust 파일이 없다"* 로 이 검사가 죽는다.
/// 그리고 넣어도 잡을 것이 없다(실측 2026-08-18: 질의 10 개 이름이 `assets`·`.claude`
/// 어디에도 평문 0 건 · 따옴표 0 건). **허수 표면은 이 하한을 허수로 만든다.**
const SURFACE_MIN: usize = 1;

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
                 질의 추가는 `surface/queries.toml` 변경으로만 일어난다(옛 F06 §2 규칙 1)"
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
    // **표면 소스에 질의 이름이 리터럴로 박히면 실패.** 박히는 순간 목록이 두 곳에서
    // 자라고, 그러면 카탈로그가 단일 진실이 아니다.
    //
    // ★ **표면을 더하면 이 목록에 더한다** — 안 더하면 새 표면이 자기 목록을 갖고,
    // 그것을 아무도 안 센다. F06b 의 어댑터가 정확히 그 자리였고(방향 4 가 꺼진 채로
    // 자랐다), 2026-08-18 재고 처분이 그 어댑터를 지웠다. 꺼진 대조는 `–` 가 아니라
    // 실패다.
    let mut 스캔 = 0usize;
    let mut 훑은_표면 = 0usize;

    // **하한을 먼저 본다.** 아래 루프는 목록이 비면 **한 바퀴도 안 돌고** 통과한다 —
    // 0 건은 *"리터럴이 없다"* 가 아니라 *"안 봤다"* 이고, 둘을 뭉개면 이 검사가
    // 자기 대상이 사라진 것을 초록으로 낸다.
    if SURFACE_SOURCES.len() < SURFACE_MIN {
        problems.push(format!(
            "훑을 표면이 {}곳이다 — {SURFACE_MIN}곳 미만이면 방향 4 가 공짜로 통과한다",
            SURFACE_SOURCES.len()
        ));
    }

    for 표면 in SURFACE_SOURCES {
        let dir = root.join(표면);
        // **없으면 실패다.** feature 로 꺼서 디렉터리가 사라지는 일은 없고, 조용히
        // 건너뛰면 이 검사가 0 개를 훑고도 통과한다(대조가 꺼지는 형태 ①).
        if !dir.is_dir() {
            problems.push(format!("표면 소스 {표면} 가 없다 — 스캔이 조용히 꺼진다"));
            continue;
        }
        훑은_표면 += 1;
        // **파일이 0 개인 표면도 실패다.** 디렉터리는 있는데 `.rs` 가 없으면 그 표면에
        // 대해 아래 루프가 한 번도 안 돈다 — 위와 같은 침묵이다.
        let 파일들 = rust_sources(&dir)?;
        if 파일들.is_empty() {
            problems.push(format!("표면 소스 {표면} 에 Rust 파일이 없다 — 그 표면은 안 재어진다"));
        }
        for file in 파일들 {
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
        "질의 {}개 · 양방향 0건 · 표면 {훑은_표면}곳의 소스 {스캔}개에 박힌 이름 0건",
        catalog.queries.len()
    ))
}

/// 파생 — 질의 표. **손으로 쓰지 않는다.**
/// ⚠ **여기서 나가는 문자열에 상대 링크를 쓰지 않는다.**
///
/// 이 함수는 `xtask/src/` 에 살면서 `docs/` 아래에 파일을 낸다. 상대 링크를 담으면
/// **두 기준 중 하나에서 반드시 죽는다** — 죽은 링크 검사는 「발신 파일 기준」으로
/// 해석하므로 `xtask/src/…` 에서 보고, 사람은 `docs/…` 에서 본다. 둘을 동시에
/// 만족시킬 수 없다. 그래서 경로는 **코드 표기(`` ` ` ``)로 적고 링크로 안 만든다.**
///
fn render_catalog_doc(c: &pal_core::QueryCatalog) -> String {
    use std::fmt::Write as _;
    let mut o = String::new();
    o.push_str("<!-- 이 파일은 `cargo xtask query-doc` 이 낸다. 손으로 고치지 않는다. -->\n");
    o.push_str("<!-- 정본은 surface/queries.toml 이고 CI 가 둘의 일치를 센다. -->\n\n");
    let _ = writeln!(o, "# 질의 카탈로그 v{}\n", c.version);
    let _ = writeln!(
        o,
        "**이 빌드가 답하는 질의 {}개.** 여기 없는 것은 이 빌드가 답하지 않는다 — \
         옛 `F06 §3` 의 표는 **로드맵이고 이 표의 상위집합이 \
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
         자라는 것 자체가 관측 대상이다(옛 `DESIGN §1.2` · 처분은 `docs/plan/disposal-map.md`).\n",
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

// ── 검사 10 — 벗어나는 경로 부재 (옛 F05 §5.1·§5.2) ─────────────────────────────
//
// 둘을 한 검사로 센다. **같은 형태이기 때문이다** — 둘 다 *"이 값을 안 지고 나갈 수
// 있는 문"* 이고, 둘 다 **타입으로 100% 막히지 않는다.** 옛 F05 §5.1 이 그것을 인정했다:
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
            "값을 안 지고 나가는 문이 생겼다 (옛 F05 §5.1·§5.2):\n    {}",
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

// ── 검사 12 — 앵커는 신고받지 않는다 (옛 F09 §4.1 · 옛 DESIGN §6.5 D32) ────────────
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
    // 제안은 좌표까지만 낸다(`Classification`). 그것이 옛 F09 §4.1(D32)이 요구한
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
             (옛 F09 §4.1: 앵커는 결박 시점에 **기계가 대상 좌표에서 읽은** digest 다.\n    \
             생산자의 신고를 여기 넣으면 그 신고가 앵커가 된다):\n    {}",
            새것.iter().map(|s| s.as_str()).collect::<Vec<_>>().join("\n    ")
        );
    }
    Ok(format!("`WatchEntry` 생성 자리 {}개 · 등록된 자리 {}개", sites.len(), WATCH_ENTRY_SITES.len()))
}

// ── 검사 13 — 낡음이 생성기를 안 부른다 (옛 F09 §4.1) ──────────────────────────
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
            "낡음이 생성기를 부르는 경로가 생겼다 (옛 F09 §4.1):\n    \
             ① 생산자 분리(F17)가 깨지고 ② 기록하되 통치하지 않는다는 경계가 무너지고\n    \
             ③ **사람이 승인한 것이 승인 없이 교체된다**:\n    {}",
            hits.join("\n    ")
        );
    }
    Ok(format!("낡음을 다루는 파일 {}개 · 생성 낱말 {}개에 0건", files.len(), REGENERATION_MARKERS.len()))
}

// ── 검사 14 — 인입이 자연어 유사도를 안 쓴다 (옛 F10 §3.2 · §5) ────────────────
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
            "좌표 해소가 자연어 유사도를 쓴다 (옛 F10 §3.2 · §5):\n    \
             **거짓 결박을 대량 생산한다. 그리고 틀린 결박은 없는 결박보다 나쁘다.**\n    \
             동점은 좁히는 것이 아니라 **후보로 내고 승인을 요구한다**:\n    {}",
            hits.join("\n    ")
        );
    }
    Ok(format!("인입 파일 {센_파일}개 · 유사도 낱말 {}개에 0건", SIMILARITY_MARKERS.len()))
}

// ── 검사 15 — 승격이 원본을 안 고친다 (옛 F10 §1 · §3.3) ──────────────────────
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
            "승격이 원본을 제자리에서 고친다 (옛 F10 §3.3):\n    \
             ① 되돌릴 수 없고 ② **원래 누구의 추론이었는가**가 계보에서 사라지고\n    \
             ③ *\"어디까지가 기록이고 어디부터가 재구성인지\"* 를 아무도 모르게 된다:\n    {}",
            hits.join("\n    ")
        );
    }
    Ok(format!("승격을 다루는 파일 {}개 · 제자리 수정 {}개 형태에 0건", files.len(), IN_PLACE_PROMOTION.len()))
}

// ── 검사 16 — 설치 경로가 홈을 안 부른다 (옛 F24 §2 ⑦) ─────────────────────────
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
    use super::{
        등록과_댄다, 등록된_외침, 시험이_돌았나, 실패한_시험들, 외침, 축과_댄다, 판정, 판정한다,
    };

    /// `cargo test` 의 실제 출력 형태에서 이름만 뽑는다.
    #[test]
    fn 실패한_이름만_뽑는다() {
        let out = "\
running 3 tests
test 통과하는것 ... ok
test 파이프_방어가_이_플랫폼에서는_안_재진다 ... FAILED
test common::eol::tests::맞추면_왕복한다 ... ok
test 또_깨진것 ... FAILED
test crates/pal-core/src/coord.rs - coord::SymbolIdentity (line 177) - compile fail ... FAILED

failures:

---- 또_깨진것 stdout ----
test result: FAILED. 2 passed; 2 failed; 0 ignored
";
        assert_eq!(
            실패한_시험들(out),
            vec![
                "파이프_방어가_이_플랫폼에서는_안_재진다",
                "또_깨진것",
                // ★ **doctest 도 같은 한 줄 형태다** — 파서를 안 고쳐도 걸린다.
                // ⚠ 그리고 이름에 **줄번호가 박힌다**(`(line 177)`). 그 위에 주석 한 줄만
                // 넣어도 이름이 바뀌므로, 이 이름을 `외침` 에 등록하면 **양방향 판정이
                // 동시에 운다** — 등록되지 않은 실패 + 등록됐는데 안 남.
                "crates/pal-core/src/coord.rs - coord::SymbolIdentity (line 177) - compile fail",
            ]
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

    /// ★ **어느 플랫폼에도 안 재지는 것이 없다.**
    ///
    /// 앞 판은 이 단언이 `#[cfg(unix)]` 였다 — Windows 에 다섯이 등록돼 있었기 때문이다.
    /// **그 다섯이 없어졌으므로 이제 `cfg` 가 필요 없다**(2026-08-17). 그리고 `cfg` 를
    /// 떼는 것이 이 시험의 값이다: 어느 플랫폼에서든 새 외침이 등록되면 **거기서**
    /// 빨개진다. 앞 판은 Windows 에 무엇이 등록되든 이 시험이 아무 말도 안 했다.
    #[test]
    fn 어느_플랫폼에도_안_재지는_것이_없다() {
        assert!(
            등록된_외침().is_empty(),
            "안 재지는 것이 등록됐다 — 그것은 등록할 것이 아니라 **고칠 것**이다.\n    \
             정말로 원리상 불가능하면 등록하되, 이 단언을 그때 함께 움직여라: {:?}",
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

    /// ★ **음성 대조 ③ — 컴파일이 서지 못하면 초록이 아니다.**
    ///
    /// 이것이 앞 판의 구멍이었다. 워크스페이스가 컴파일에 실패하면 stdout 에
    /// `test … ... FAILED` 가 **한 줄도 안 나고**, 그러면 「실패 0건 = 등록 0건」이 되어
    /// 집합 대조가 **초록**을 냈다. CI 세 OS 가 이 명령 하나로 판정하므로 셋이 함께 속았다.
    #[test]
    fn 컴파일이_서지_못하면_초록이_아니다() {
        assert_eq!(판정한다(false, false, &[], &[]), 판정::시험을_못_돌렸다);
        // ★ 그리고 **집합 대조와 다른 문구**로 나야 한다 — 사람이 할 일이 다르다.
        assert_ne!(판정한다(false, false, &[], &[]), 판정::어긋났다(Vec::new()));
    }

    /// ★ **음성 대조 ④ — rc=0 이 보고 부재를 면제하지 못한다.**
    ///
    /// 이것이 앞 판에 남아 있던 같은 과의 구멍이다. `보고가_다_있나` 가
    /// `r.섰나 || 시험이_돌았나(…)` 였어서 **rc=0 인 호출은 보고 검사를 통째로
    /// 건너뛰었고**, `외침` 이 비어 있으므로 집합 대조도 침묵했다. 실측(2026-08-17):
    /// 축 하나를 `--no-run` 으로 바꾸니 통과 수가 **753 에서 3 으로** 줄었는데
    /// `cargo xtask test` 는 **rc=0 · "시험 통과"** 를 냈다.
    #[test]
    fn rc가_0이어도_보고가_없으면_초록이_아니다() {
        assert_eq!(판정한다(true, false, &[], &[]), 판정::시험을_못_돌렸다);
        // ★ 그리고 **집합 대조보다 앞선다** — 등록이 있어도 판정이 안 바뀐다.
        // 보고가 없으면 실패 집합은 「비었다」가 아니라 **모른다**이기 때문이다.
        let 등록 = [("외침A", "까닭")];
        assert_eq!(판정한다(true, false, &[], &등록), 판정::시험을_못_돌렸다);
    }

    /// ★ **음성 대조 ⑤ — 축이 사라지면 걸린다.**
    ///
    /// 앞 판에서 이 자리는 **아무도 안 봤다.** `보고가_다_있나` 는 돌린 축만 대므로 축을
    /// 빼면 통과 수가 754 에서 3 으로 떨어져도 초록이었고, 빈 축은 `all()` 이 `true` 라
    /// **완전 초록**이었다. `7036909` 이전이 정확히 그 상태다.
    #[test]
    fn 축이_사라지면_걸린다() {
        let 온전 = [
            ["test", "--workspace", "--all-targets", "--no-fail-fast"],
            ["test", "--workspace", "--doc", "--no-fail-fast"],
        ];
        assert!(축과_댄다(&온전).is_empty(), "계약대로면 조용하다");

        // ① 시험 본체 축이 빠졌다 — 남은 doc 축이 보고를 내므로 rc 로도 안 걸리던 자리.
        let 문제 = 축과_댄다(&온전[1..]);
        assert_eq!(문제.len(), 1, "{문제:?}");
        assert!(문제[0].contains("--all-targets"), "{문제:?}");

        // ② doc 축이 빠졌다 — `7036909` 이전의 실제 상태다.
        let 문제 = 축과_댄다(&온전[..1]);
        assert_eq!(문제.len(), 1, "{문제:?}");
        assert!(문제[0].contains("--doc"), "{문제:?}");

        // ③ ★ **빈 축이 최악이다** — `all()` 이 `true` 라 아무것도 안 돌고 초록이었다.
        assert_eq!(축과_댄다(&[]).len(), 2);

        // ④ 길이는 그대로인데 한 축이 다른 축으로 바뀌었다 — **타입이 못 잡는 자리**다.
        let 바뀜 = [
            ["test", "--workspace", "--all-targets", "--no-fail-fast"],
            ["test", "--workspace", "--all-targets", "--no-fail-fast"],
        ];
        let 문제 = 축과_댄다(&바뀜);
        assert_eq!(문제.len(), 1, "{문제:?}");
        assert!(문제[0].contains("--doc"), "{문제:?}");
    }

    /// ★ **rc≠0 이 초록의 정상 상태다** — 등록된 외침이 그대로 났으면.
    ///
    /// 종료 상태만 보게 고치면 이 저장소의 「시끄럽게 실패하는 짝」 규율이 죽는다.
    #[test]
    fn 등록된_외침이_그대로_나면_rc가_실패여도_초록이다() {
        let 등록 = [("외침A", "까닭")];
        assert_eq!(판정한다(false, true, &["외침A".to_owned()], &등록), 판정::통과);
    }

    /// ★ **rc≠0 인데 이름이 하나도 안 났다** — 시험 밖에서 무너진 것이고, 초록이 아니다.
    #[test]
    fn 이름이_안_났는데_rc가_실패면_걸린다() {
        let 판 = 판정한다(true, true, &[], &[]);
        assert_eq!(판, 판정::통과, "rc=0 이고 실패도 등록도 없으면 초록이다");

        let 판 = 판정한다(false, true, &[], &[]);
        match 판 {
            판정::어긋났다(문제) => {
                assert_eq!(문제.len(), 1, "{문제:?}");
                assert!(문제[0].contains("이름이 하나도 안 났다"), "{문제:?}");
            }
            other => panic!("어긋났다를 기대했다: {other:?}"),
        }
    }

    /// 시험 보고가 화면에 있는가 — 「돌았다」와 「못 돌렸다」를 가르는 유일한 표시.
    #[test]
    fn 시험_보고가_있는지_본다() {
        assert!(시험이_돌았나("test a ... ok\ntest result: ok. 1 passed; 0 failed\n"));
        // doctest 의 요약도 같은 접두사다.
        assert!(시험이_돌았나("test result: FAILED. 0 passed; 1 failed; 0 ignored\n"));
        // 컴파일이 서지 못하면 이 줄이 없다.
        assert!(!시험이_돌았나("   Compiling pal-core v0.0.0\nerror[E0432]: unresolved import\n"));
        assert!(!시험이_돌았나(""));
    }

    /// ★ **등록에는 언제나 까닭이 붙고, 그 까닭은 「원리상 불가능」이어야 한다.**
    ///
    /// ⚠ 앞 판은 여기서 `assert!(!외침.is_empty())` 를 했다 — *"목록이 비었으면 이
    /// 대조가 아무것도 안 센다"* 는 이유로. **그 단언은 이제 틀렸다.** 빈 목록은
    /// 「대조가 죽었다」가 아니라 **「안 재지는 것이 하나도 없다」**이고, 그것이 이
    /// 저장소가 가려던 자리다. 대조가 살아 있다는 것은 [`등록_안_된_실패가_걸린다`] 와
    /// [`승격됐는데_등록이_남으면_걸린다`] 가 **순수 함수 위에서** 잰다 — 목록의
    /// 길이에 안 기댄다.
    ///
    /// 대신 **자격**을 잰다: 「아직」·「나중에」로 끝나는 까닭은 등록될 자격이 없다.
    /// 그것은 **할 일이지 사실이 아니고**, 목록에 넣는 순간 CI 가 그것을 초록으로
    /// 세어 준다.
    ///
    /// ⚠ **첫 필드는 이제 여기서 안 잰다 — [`플랫폼`] 이 타입이라 잴 것이 없다.** 앞 판은
    /// `&str` 이었고 이 시험이 `!플랫폼.is_empty()` 만 봐서 `"linux"` 같은 오타를 **전부
    /// 통과시켰다**(실측: 15 통과 · rc=0). 그 오타는 이제 컴파일이 운다.
    ///
    /// ⚠ **`왜.len()` 은 바이트다** — 실측(2026-08-17): 한글 **7자(21바이트)는 통과**하고
    /// **6자(18바이트)는 실패**한다. 즉 한국어로는 사실상 「7자 이상」이다. **이 회차는 그
    /// 수를 안 올린다**(운영 순서 4 — 합격선 상향 금지). 게이트 문서에 적어 넘긴다.
    #[test]
    fn 등록은_원리상_불가능한_것만_담는다() {
        for (_, 이름, 왜) in 외침 {
            assert!(!이름.is_empty(), "빈 등록이 있다");
            assert!(왜.len() > 20, "`{이름}` 의 까닭이 너무 짧다: {왜}");
            for 금지 in ["아직", "나중에", "다음 회차", "미측정"] {
                assert!(
                    !왜.contains(금지),
                    "`{이름}` 의 까닭에 「{금지}」가 있다 — 그것은 **할 일이지 사실이 \
                     아니다.** 등록은 「원리상 못 잰다」만 담는다: {왜}"
                );
            }
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 죽은 링크 — **문서가 없는 문서를 가리키면 그것이 거짓 신호다**
// ─────────────────────────────────────────────────────────────────────────────
//
// # 왜 이 검사가 생겼나 (2026-08-18 · 재고 처분)
//
// 이 저장소의 논지가 *"낡은 문서의 문제는 쓸모없어지는 것이 아니라 **거짓 신호**가
// 되는 것"* 인데, **자기 저장소에 마크다운 링크 검사기가 없었다.** 그래서 문서 하나를
// 지우면 그것을 가리키던 자리 백여 곳이 조용히 죽고 아무도 안 셌다. `rustdoc` 의
// `broken_intra_doc_links` 는 **Rust 항목 링크만** 보고 파일 경로 링크는 안 본다.
//
// # ★ 모집단 규칙 — **측정 전에 등록한다**
//
// 규칙 없이 훑으면 시험 픽스처가 오탐으로 쏟아진다(실측: `.rs` 안의 경로형 문자열
// `"docs/x.md"` 11 건 · `"docs/order/x.md"` 3 건 · `"docs/plan.md"` 3 건 — 전부 임시
// 저장소에 만드는 가짜 경로다). 그래서 셋을 못 박는다:
//
//   1. **마크다운 링크 문법만** 본다 — `](경로)`. 산문 안의 경로 언급은 안 본다.
//   2. **발신 파일 기준**으로 상대 해석한다.
//   3. `http`·`mailto`·앵커 전용(`#…`)은 제외한다.
//
// # ★ 모집단 밖 — **왜 빠지는지가 각각 다르다**
//
// | 자리 | 왜 |
// |---|---|
// | `docs/gates/` 의 판정 문서 | **동결됐다.** 그때의 기록이고 사후에 안 고친다 |
// | `docs/adr/` | **채택 시점의 결정 기록**이다. 근거 절을 사후에 고치면 그 ADR 이 무엇에 근거해 채택됐는지가 사라진다 |
// | `docs/instructions/` | **소유자 지시 원문**이다. 낡을 수 없다 |
// | `corpus/` | 게이트가 읽는 **측정 장치**이자 사전 등록 대장이다 |
// | sunset 선언된 경로 | **지울 예정**이다. 지금 수리하면 두 장치가 서로를 당긴다 |
// | `target/` · `.git/` | 산출·내부 |
//
// ⚠ **이 목록이 은신처가 되지 않게 하는 것**: 빠지는 자리마다 **왜**가 위 표에 있고,
// 그 왜가 「고치기 귀찮다」인 항목은 하나도 없다. 새로 더할 때도 같은 자를 쓴다.
//
// # ★ 이 검사가 **못 보는 것** — 알고 두는 것과 모르고 두는 것은 다르다
//
// 2026-08-18 재고 처분이 문서 넷을 지우면서 링크 **161 건**이 죽었고, 수선 뒤에도
// 이 검사가 **구조적으로 못 보는** 부류가 남는다. 적어 둔다 — 안 적으면 다음 사람이
// 「0 건」을 「없다」로 읽는다.
//
// | 못 보는 것 | 왜 | 이 회차의 실물 |
// |---|---|---|
// | **텍스트가 부르는 절이 대상에 없다** | 대상 파일이 있으면 통과한다 | `[옛 DESIGN §12.4]` 이 `disposal-map.md` 를 가리키는데 거기 §12.4 절은 없다. **109 건**에 「옛」을 달아 *"그 문서는 사라졌다"* 를 텍스트가 말하게 했다 |
// | **정의가 없는 참조형** | `[라벨]: 경로` 줄이 아예 없으면 셀 것이 없다 | `crates/pal-extract/src/plan.rs` 의 `[옛 F12 §3.4]` — 정의 0 건이라 `rustdoc` 이 대괄호를 그대로 렌더한다. 「옛」 표기로 바꿔 링크가 아니게 했다 |
// | **산문 안의 경로 언급** | 마크다운 링크 문법만 본다 | `crates/pal-cli/src/ledger.rs` 의 *"옛 how-it-works §2.2 의 화면"* |
//
// ★ **규약**: 삭제된 문서를 계속 인용해야 하면 **링크가 아니라 코드 표기로 적고 앞에
// 「옛」을 단다.** 그러면 이 검사가 안 봐도 **읽는 사람이 안다** — 그것이 이 저장소가
// 말하는 *"낡은 문서의 문제는 거짓 신호가 되는 것"* 의 반대편이다.
//
// ⚠ **모집단이 비면 실패다** — 0 건은 *"죽은 링크가 없다"* 가 아니라 *"안 봤다"* 이고,
// 둘을 뭉개면 이 검사가 자기 대상이 사라진 것을 초록으로 낸다(`SURFACE_MIN` 과 같은 자리).

/// 훑지 않는 자리. 저장소 루트 기준 접두사이고 구분자는 언제나 `/` 다.
const 링크_모집단_밖: &[&str] = &[
    "docs/adr/",
    "docs/instructions/",
    "corpus/",
    "target/",
    ".git/",
];

/// 게이트 디렉터리에서 **판정 문서만** 뺀다.
///
/// ★ `docs/gates/` 를 통째로 빼면 이 회차가 거기 새로 놓는 `README.md` 와
/// `inventory-disposal.md` 가 **자기가 세우는 검사의 모집단 밖**에 선다.
fn 동결된_판정_문서인가(상대: &str) -> bool {
    let Some(이름) = 상대.strip_prefix("docs/gates/") else { return false };
    if 이름.contains('/') {
        return true;
    }
    let 첫 = 이름.as_bytes().first().copied().unwrap_or(b' ');
    matches!(첫, b'F' | b'G' | b'S') || 이름.starts_with("preflight")
}

fn check_dead_links(root: &Path) -> Result<String> {
    let sunset = sunset_선언(root)?;
    let mut 파일들 = Vec::new();
    모을_문서(root, root, &sunset, &mut 파일들)?;
    파일들.sort();

    // **모집단이 비면 실패다.**
    if 파일들.len() < 30 {
        bail!(
            "훑을 문서가 {}개다 — 30개 미만이면 이 검사는 아무것도 안 세고 통과한다",
            파일들.len()
        );
    }

    let mut 죽음 = Vec::new();
    let mut 링크수 = 0usize;
    for file in &파일들 {
        let body = std::fs::read_to_string(file)?;
        let dir = file.parent().unwrap_or(root);
        for 대상 in 마크다운_링크(&body) {
            링크수 += 1;
            let (경로, 앵커) = 대상.split_once('#').map_or((대상.as_str(), ""), |(a, b)| (a, b));
            let 붙인 = dir.join(경로);
            if !붙인.exists() {
                죽음.push(format!("{}  →  {대상}", 상대_경로(root, file)));
                continue;
            }
            // ★ **자리까지 본다.** 파일이 있어도 그 안에 그 조각이 없으면 죽은 링크다.
            //   `.md` 만 본다 — 다른 형식의 조각 규칙은 우리가 모른다.
            if !앵커.is_empty() && 경로.ends_with(".md") {
                let Ok(대상_본문) = std::fs::read_to_string(&붙인) else { continue };
                if !조각들(&대상_본문).contains(앵커) {
                    죽음.push(format!(
                        "{}  →  {대상}  (파일은 있는데 **그 조각이 없다**)",
                        상대_경로(root, file)
                    ));
                }
            }
        }
    }

    if !죽음.is_empty() {
        bail!("죽은 링크 {}건:\n    {}", 죽음.len(), 죽음.join("\n    "));
    }
    Ok(format!("문서 {}개 · 링크 {링크수}건 · 죽은 것 0건", 파일들.len()))
}

/// 모집단을 모은다 — `.md` 와 `.rs`.
fn 모을_문서(root: &Path, dir: &Path, sunset: &[String], out: &mut Vec<PathBuf>) -> Result<()> {
    for entry in std::fs::read_dir(dir).with_context(|| format!("읽지 못했다: {}", dir.display()))? {
        let p = entry?.path();
        let 상대 = 상대_경로(root, &p);
        if 링크_모집단_밖.iter().any(|x| 상대.starts_with(x))
            || sunset.iter().any(|x| 상대 == *x || 상대.starts_with(&format!("{x}/")))
        {
            continue;
        }
        if p.is_dir() {
            모을_문서(root, &p, sunset, out)?;
        } else if 동결된_판정_문서인가(&상대) {
            continue;
        } else {
            let ext = p.extension().and_then(|e| e.to_str()).unwrap_or("");
            // ⚠ 시험은 가짜 경로를 만든다 — 모집단에서 뺀다.
            let 시험인가 = 상대.contains("/tests/") || 상대.ends_with("/common.rs");
            // ★ **`.toml` 이 빠져 있었다** (독립 리뷰 2026-08-18). `surface/queries.toml`
            //    은 **질의 카탈로그의 단일 진실**인데 확장자 때문에 안 훑렸고, 그 안에서
            //    링크 하나가 죽어 있었다 — 계약 근거가 없는 문서를 가리켰다.
            //    등록된 모집단에 `surface/` 가 있었으므로 **선언과 구현이 갈린 자리**다.
            // ★ `.txt` 도 본다 — 회차의 **효과 전문**(`claude -p` 세션 출력)이 여기
            //   살고, 그 안의 경로 인용이 낡으면 게이트의 1 차 증거가 거짓이 된다.
            if ext == "md" || ext == "toml" || ext == "txt" || (ext == "rs" && !시험인가) {
                out.push(p);
            }
        }
    }
    Ok(())
}

/// 마크다운 링크에서 **파일 경로만** 뽑는다 — **두 형태를 다 본다.**
///
/// ★ **인라인만 보면 절반을 놓친다** (독립 리뷰 2026-08-18 · 실측 8 건). 마크다운에는
/// 형태가 둘이고 `rustdoc` 은 **똑같이 렌더링한다**:
///
/// ```text
/// 인라인      [라벨](경로)
/// 참조 정의   [라벨]: 경로        ← 줄 첫머리에 산다
/// ```
///
/// 앞 판은 `']' + '('` 만 찾아서 정의형을 통째로 안 셌다. 그 결과 같은 파일 안에
/// 인라인형은 고쳐지고 정의형은 안 고쳐진 자리가 나왔다(`crates/pal-core/src/budget.rs`
/// 의 `:55` 와 `:141`). **수선이 검사와 같은 눈으로 돌지 않으면 그 차이가 그대로 남는다.**
///
/// ★ **rustdoc 의 항목 링크를 걸러야 한다.** `.rs` 안에는 `[Envelope](Envelope)` ·
/// `[좌표](crate::Coord)` · `[심볼](Self::symbols)` 같은 **Rust 항목 링크**가 많고,
/// 그것들은 파일이 아니다. `rustdoc` 의 `broken_intra_doc_links` 가 이미 그 축을
/// 재고 있으므로 여기서 또 재면 **같은 것을 두 곳에서 세는 것**이고, 게다가 틀리게 센다.
///
/// 가르는 자 둘:
///   · `::` 가 있으면 Rust 경로다.
///   · 파일 경로는 **`/` 를 갖거나 아는 확장자로 끝난다.**
fn 마크다운_링크(body: &str) -> Vec<String> {
    let mut out = Vec::new();

    // ── 참조 정의형 — `[라벨]: 경로` ────────────────────────────────────────
    // 줄 첫머리(주석 접두사 뒤)에 오는 것만 본다. 산문 중간의 `[x]: y` 는 링크가 아니다.
    for line in body.lines() {
        let t = line
            .trim_start()
            .trim_start_matches("///")
            .trim_start_matches("//!")
            .trim_start_matches("//")
            .trim_start_matches('#')
            .trim_start();
        let Some(rest) = t.strip_prefix('[') else { continue };
        let Some((_라벨, 뒤)) = rest.split_once("]: ") else { continue };
        let 대상 = 뒤.split_whitespace().next().unwrap_or("");
        if let Some(링크) = 링크로(대상) {
            out.push(링크);
        }
    }

    // ── 인라인형 — `](경로)` ────────────────────────────────────────────────
    let b = body.as_bytes();
    let mut i = 0;
    while i + 1 < b.len() {
        if b[i] == b']' && b[i + 1] == b'(' {
            if let Some(close) = body[i + 2..].find(')') {
                let 대상 = &body[i + 2..i + 2 + close];
                i += 2 + close;
                if let Some(링크) = 링크로(대상) {
                    out.push(링크);
                }
                continue;
            }
        }
        i += 1;
    }
    out
}

// ─────────────────────────────────────────────────────────────────────────────
// sunset — **「나중에 지운다」를 장치로 잠근다**
// ─────────────────────────────────────────────────────────────────────────────
//
// 선언은 `docs/sunset.toml` 에 있고 **그 파일이 왜 그렇게 생겼는지도 거기 있다.**
// 여기서는 셋을 본다:
//
//   ① 트리거가 참인데 선언된 경로가 남아 있으면  → 실패 (지울 때가 됐다)
//   ② 선언된 경로가 이미 없으면                  → 실패 (선언이 잔재다)
//   ③ 선언이 하나도 없으면                       → 실패 (빈 껍데기다)
//
// ★ ②가 없으면 이 파일이 조용히 낡는다 — `SURFACE_MIN`·`외침` 레지스트리와 같은 자리다.
//
// `toml` 크레이트를 안 들인다. `xtask/Cargo.toml` 이 *"외부 의존을 늘리지 않는다"* 를
// 적었고, 선례가 있다 — `vocab.toml`(한 줄짜리 허용 목록)을 손으로 읽는다. 이 파일도
// **키가 셋뿐인 평평한 목록**이라 같은 자격이다.

fn sunset_선언(root: &Path) -> Result<Vec<String>> {
    let path = root.join("docs/sunset.toml");
    let text = std::fs::read_to_string(&path)
        .with_context(|| format!("sunset 선언을 읽지 못했다: {}", path.display()))?;
    Ok(text
        .lines()
        .filter_map(|l| l.trim().strip_prefix("경로 = "))
        .filter_map(|v| v.trim().strip_prefix('"')?.strip_suffix('"'))
        .map(str::to_owned)
        .collect())
}

fn sunset_트리거(root: &Path) -> Result<String> {
    let text = std::fs::read_to_string(root.join("docs/sunset.toml"))?;
    text.lines()
        .find_map(|l| l.trim().strip_prefix("트리거 = "))
        .and_then(|v| v.trim().strip_prefix('"')?.strip_suffix('"'))
        .map(str::to_owned)
        .context("`docs/sunset.toml` 에 `트리거 = ` 가 없다")
}

/// `.palimpsest/rounds/*/*.json` 형태의 글롭 하나만 푼다.
fn 트리거가_참인가(root: &Path, glob: &str) -> Result<Vec<String>> {
    let mut 조각 = glob.split('/');
    let (Some(a), Some(b), Some(c), Some(d), None) =
        (조각.next(), 조각.next(), 조각.next(), 조각.next(), 조각.next())
    else {
        bail!("트리거 글롭의 형태가 `a/b/*/*.ext` 가 아니다: {glob}");
    };
    let 확장 = c == "*" && d.starts_with("*.");
    if !확장 {
        bail!("트리거 글롭의 형태가 `a/b/*/*.ext` 가 아니다: {glob}");
    }
    let ext = &d[2..];
    let base = root.join(a).join(b);
    let mut 찾음 = Vec::new();
    let Ok(읽기) = std::fs::read_dir(&base) else { return Ok(찾음) };
    for entry in 읽기 {
        let dir = entry?.path();
        if !dir.is_dir() {
            continue;
        }
        // ★ **깊이를 안 묶는다** (독립 리뷰 2026-08-18). 앞 판은 `rounds/<회차>/` 바로
        //   아래만 봤는데, **이 회차 자신이 이미 `rounds/<회차>/effect/` 를 만들었다.**
        //   `pal` 이 레코드를 한 단계 안쪽에 쓰면 트리거가 **영영 안 뜨고**, 그러면
        //   검사는 초록인 채로 아무것도 안 지킨다 — 「꺼진 대조는 `–` 가 아니라 실패다」.
        //   이름을 안 묶은 것과 같은 이유로 깊이도 안 묶는다.
        아래_전부(&dir, ext, root, &mut 찾음)?;
    }
    찾음.sort();
    Ok(찾음)
}

fn check_sunset(root: &Path) -> Result<String> {
    let 선언 = sunset_선언(root)?;
    // ③ 빈 껍데기
    if 선언.is_empty() {
        bail!("`docs/sunset.toml` 에 선언이 하나도 없다 — 이 검사가 아무것도 안 잰다");
    }
    let mut problems = Vec::new();

    // ② 선언이 잔재인가
    for 경로 in &선언 {
        if !root.join(경로).exists() {
            problems.push(format!(
                "`{경로}` 가 이미 없는데 선언이 남아 있다 — 선언을 지워라"
            ));
        }
    }

    // ① 트리거가 참인가
    let glob = sunset_트리거(root)?;
    let 마커 = 트리거가_참인가(root, &glob)?;
    if !마커.is_empty() {
        for 경로 in &선언 {
            if root.join(경로).exists() {
                problems.push(format!(
                    "`{경로}` 를 지울 때가 됐다 — 트리거가 참이다 ({})",
                    마커.join(" · ")
                ));
            }
        }
    }

    if !problems.is_empty() {
        bail!("sunset:\n    {}", problems.join("\n    "));
    }
    Ok(format!(
        "선언 {}건 · 트리거 `{glob}` 는 아직 0건 — 그날이 오면 여기가 빨개진다",
        선언.len()
    ))
}

/// 링크 대상을 **`경로#앵커`** 로 정규화한다. 파일 경로가 아니면 `None`.
///
/// ★ **앵커를 버리면 안 된다** (독립 리뷰 2026-08-18 · 실측 3 건). 앞 판은
/// `대상.split('#').next()` 로 조각을 통째로 버렸고, 그래서 **이 회차가 제목 하나를
/// 고치면서 살아 있던 앵커 셋을 깼는데도 초록**이었다. 파일이 있으면 통과시키는 검사는
/// *"가리키는 것이 있는가"* 를 답하지 *"가리키는 자리가 있는가"* 를 답하지 않는다.
fn 링크로(대상: &str) -> Option<String> {
    let 대상 = 대상.trim();
    let (경로, 앵커) = 대상.split_once('#').map_or((대상, ""), |(a, b)| (a, b));
    let 경로 = 경로.trim();
    if !파일_경로인가(경로) {
        return None;
    }
    Some(if 앵커.is_empty() { 경로.to_owned() } else { format!("{경로}#{앵커}") })
}

/// 마크다운 제목에서 GitHub 이 만드는 조각 이름을 낸다.
///
/// 규칙(실측으로 맞춘 것): 링크는 **텍스트만** 남기고 · 강조와 코드 표기를 벗기고 ·
/// 소문자로 · 글자·숫자·공백·`-`·`_` 아닌 것을 버리고 · 공백을 `-` 로.
fn 조각_이름(제목: &str) -> String {
    // `[텍스트](대상)` → `텍스트`
    let mut t = String::new();
    let b: Vec<char> = 제목.chars().collect();
    let mut i = 0;
    while i < b.len() {
        if b[i] == '[' {
            if let Some(close) = b[i + 1..].iter().position(|c| *c == ']') {
                let 텍스트: String = b[i + 1..i + 1 + close].iter().collect();
                t.push_str(&텍스트);
                let mut j = i + 1 + close + 1;
                if j < b.len() && b[j] == '(' {
                    while j < b.len() && b[j] != ')' {
                        j += 1;
                    }
                    j += 1;
                }
                i = j;
                continue;
            }
        }
        t.push(b[i]);
        i += 1;
    }
    t.to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == ' ' || *c == '-' || *c == '_')
        .map(|c| if c == ' ' { '-' } else { c })
        .collect()
}

/// 그 문서가 가진 조각 이름 전부.
fn 조각들(body: &str) -> std::collections::HashSet<String> {
    let mut out = std::collections::HashSet::new();
    for line in body.lines() {
        let t = line.trim_start();
        if !t.starts_with('#') {
            continue;
        }
        let 제목 = t.trim_start_matches('#').trim();
        if 제목.is_empty() {
            continue;
        }
        out.insert(조각_이름(제목));
        // 손으로 단 `{#이름}` 도 받는다.
        if let Some(a) = 제목.rfind("{#") {
            if let Some(b) = 제목[a..].find('}') {
                out.insert(제목[a + 2..a + b].to_owned());
            }
        }
    }
    out
}

/// 링크 대상이 **파일 경로**인가 — Rust 항목 링크·URL·산문을 여기서 가른다.
fn 파일_경로인가(대상: &str) -> bool {
    const 확장자: &[&str] =
        &[".md", ".rs", ".toml", ".yml", ".yaml", ".json", ".py", ".sh", ".txt"];
    if 대상.is_empty()
        || 대상.starts_with("http")
        || 대상.starts_with("mailto:")
        || 대상.contains(' ')
        || 대상.contains("::")
    {
        return false;
    }
    대상.contains('/') || 확장자.iter().any(|e| 대상.ends_with(e))
}

/// 디렉터리 아래에서 그 확장자를 가진 파일을 **깊이 상관없이** 모은다.
fn 아래_전부(dir: &Path, ext: &str, root: &Path, out: &mut Vec<String>) -> Result<()> {
    for e in std::fs::read_dir(dir).with_context(|| format!("읽지 못했다: {}", dir.display()))? {
        let p = e?.path();
        if p.is_dir() {
            아래_전부(&p, ext, root, out)?;
        } else if p.extension().and_then(|x| x.to_str()) == Some(ext) {
            out.push(상대_경로(root, &p));
        }
    }
    Ok(())
}

// ─────────────────────────────────────────────────────────────────────────────
// 사라진 문서를 현재형으로 안 부른다 — **「옛」 규약을 장치로**
// ─────────────────────────────────────────────────────────────────────────────
//
// # 왜 이 검사가 생겼나 (2026-08-18 · 재고 처분 · 독립 리뷰 6 라운드)
//
// 회차가 문서 넷을 지우면서 **규약**을 하나 세웠다:
//
// > 삭제된 문서를 계속 인용해야 하면 **링크가 아니라 코드 표기로 적고 앞에 「옛」을
// > 단다.** 죽은 링크 검사가 안 봐도 **읽는 사람이 안다.**
//
// 그런데 그 규약을 **게이트 산문에만 적고 장치로 안 만들었더니 316 자리가 안 지켰다**
// (실측). 「옛」이 없으면 소스 주석이 *"`DESIGN §4` 가 이렇게 정했다"* 를 **현재형**으로
// 말하고, 그것이 이 저장소가 없애려는 거짓 신호다 — **없는 문서가 근거로 서 있다.**
//
// ★ **표시 대신 장치를 남긴다**(AGENTS.md). 규약을 산문에 적으면 다음 사람이 안 읽고,
// 검사에 넣으면 못 지나간다.
//
// # 무엇을 보나 — **문서가 아닌 것 전부**
//
// 문서 쪽(`docs/`·`.palimpsest/`)에서는 **처분 자체를 서술하는 자리**가 정당하게 그
// 이름을 부른다 — `disposal-map.md` 가 *"옛 `docs/plan/features/` 25 파일"* 을 적는 것은
// 낡음이 아니라 **기록**이다. 그 밖에는 그런 자리가 없다: **코드·스키마·설정·스크립트가
// 사라진 문서를 부르면 그것은 언제나 낡은 근거**다.
//
// ⚠ **파일 수는 환경마다 다르다** — gitignore 된 로컬 파일(`.t2-stamp.json` 등)이
// 워킹트리에 있으면 세어진다(실측: 로컬 148 · CI 146). **재는 축은 파일 수가 아니라
// 「인용 곳 수」이고 그것은 세 OS 와 로컬이 같다**(108). 하한(100)은 *"모집단이 통째로
// 비었다"* 만 막으면 되므로 이 차이에 안 흔들린다.
//
// ⚠ **처음엔 `.rs` 만 봤고 15 자리가 샜다**(독립 리뷰 7 라운드 · 실측):
// `schema/graph.toml` 10 · `Cargo.toml` 1 · `.gitignore` 1 · `scripts/` 3.
// 특히 `schema/graph.toml` 은 `pal-core::schema` 가 **실행 시점에 읽는** 단일 진실
// 파일이고, 그 머리가 사라진 문서를 근거로 세우고 있었다.

/// 사라진 문서를 부르는 토큰. 이 회차가 지운 것들이다.
///
/// ★ **한국어 표기도 담는다** (독립 리뷰 9 라운드). 앞 판은 라틴 표기만 담아서
/// **이 저장소가 실제로 쓰는 말**이 통째로 사각이었다 — 라틴 109 곳은 전부 「옛」이
/// 붙어 규약이 지켜진 것이 확인되는데 한국어 16 곳은 안 붙어 있었고 검사는 초록이었다.
/// **장치가 자기 저장소의 말을 모르면 그것은 대조가 아니다.**
///
/// ⚠ 아래 목록이 곧 그 말이다 — 여기 다시 안 적는다(적으면 검사가 자기 설명을
/// 위반으로 읽는다. 실측으로 한 번 걸렸다).
const 사라진_문서: &[&str] = &[
    // 라틴 표기
    "DESIGN §",
    "DESIGN.md",
    "WHITEPAPER",
    "how-it-works",
    "docs/plan/features/",
    // 한국어 표기 — 이 저장소가 실제로 쓰는 말
    "백서 §",
    "설계 문서",
];

// ⚠ **「계획 §」은 토큰이 아니다** (독립 리뷰 10 라운드). 한때 넣었다가 뺐다 —
//    `docs/plan/README.md` 는 **살아 있고** §4~§9 가 전부 선다. 넣었더니 검사가
//    **살아 있는 절을 인용한 일곱 자리에 거짓 「옛」을 강제했다.**
//    ★ **장치가 거짓말을 요구하면 그것은 대조가 아니다.** 사라진 것만 토큰이 된다.

fn check_stale_citation(root: &Path) -> Result<String> {
    let mut 문제 = Vec::new();
    let mut 센_자리 = 0usize;
    let mut 파일수 = 0usize;
    for file in 인용_모집단(root)? {
            let 상대 = 상대_경로(root, &file);
            // 시험은 가짜 경로를 만든다 — 죽은 링크 검사와 같은 모집단 규칙이다.
            // ★ **시험도 훑는다** — 사유는 아래 `인용_모집단` 의 주석에 있다.
            let _ = &상대;
            let Ok(body) = std::fs::read_to_string(&file) else { continue };
            파일수 += 1;
            // 이 검사 자신이 사는 파일인가 — 면제는 여기서만 선다.
            let 이_파일 = 상대 == "xtask/src/main.rs";
            // ★ **면제가 설 줄 범위** — 토큰 목록 선언 블록과 그 doc 주석뿐.
            //   ⚠ 앞 판은 주석에 *"블록 안에서만"* 이라 적고 **파일 전체**로 구현했다
            //   (독립 리뷰 12 라운드가 심어서 확인 — 블록 밖 52 줄이 면제됐다).
            //   **선언과 구현이 갈리면 그것이 곧 꺼진 대조다.**
            let 면제_범위 = 이_파일.then(|| 토큰_블록(&body)).flatten();
            for (n, line) in body.lines().enumerate() {
                // ★ **이 검사 자신의 토큰 목록은 인용이 아니다.** 검사가 자기 정의를
                //   위반으로 읽으면 그것은 대조가 아니라 자가당착이다.
                // ★ **면제는 이 파일의 토큰 목록 블록 안에서만** (독립 리뷰 10 라운드).
                //   앞 판은 파일을 안 가려서 **어느 파일이든** `// … 표기 …` 나
                //   `"…",` 로 생긴 줄이 통째로 면제됐다 — 심어서 확인한 **꺼진 대조**다.
                //   이 저장소는 규약을 말할 때 「**「옛」 표기**」라는 낱말을 쓰므로,
                //   그 면제는 **가장 잘 걸릴 문장 형태를 정확히 비켜 가고 있었다.**
                if 면제_범위.is_some_and(|(a, b)| n >= a && n <= b) {
                    continue;
                }
                // 옛 계획 문서를 **절 번호로** 부르는 자리 — 문자열 토큰으로는 못 잡는다.
                for at in 계획문서_인용(line) {
                    센_자리 += 1;
                    let 왼쪽: String = line[..at].chars().rev().take(20).collect();
                    if !왼쪽.contains('옛') {
                        문제.push(format!("{상대}:{}  {}", n + 1, line.trim()));
                    }
                }
                for tok in 사라진_문서 {
                    let mut from = 0;
                    while let Some(rel) = line[from..].find(tok) {
                        let at = from + rel;
                        센_자리 += 1;
                        // ⚠ **문자로 자른다.** 이 저장소는 주석을 한국어로 쓰므로
                        //    `&line[at-30..at]` 은 **글자 가운데를 잘라 패닉한다**
                        //    (실측: `is not a char boundary … inside '니'`).
                        let 왼쪽: String = line[..at].chars().rev().take(20).collect();
                        if !왼쪽.contains('옛') {
                            문제.push(format!("{상대}:{}  {}", n + 1, line.trim()));
                        }
                        from = at + tok.len();
                    }
                }
            }
    }
    // **모집단이 비면 실패다** — 0 건은 *"안 부른다"* 가 아니라 *"안 봤다"* 일 수 있다.
    if 센_자리 == 0 || 파일수 < 100 {
        bail!(
            "사라진 문서를 부르는 자리가 {센_자리} 곳이고 훑은 파일이 {파일수} 개다 — \
             토큰 목록이나 모집단이 비었다"
        );
    }
    if !문제.is_empty() {
        bail!(
            "사라진 문서를 **현재형**으로 부르는 자리 {}곳 — 앞에 「옛」을 달아라:\n    {}",
            문제.len(),
            문제.join("\n    ")
        );
    }
    Ok(format!("파일 {파일수}개 · 사라진 문서 인용 {센_자리}곳 · 전부 「옛」 표기"))
}

/// 검사 19 가 훑는 자리 — **문서가 아닌 것 전부.**
///
/// 빼는 것은 **다섯**이다: `docs/`·`.palimpsest/` 에는 **처분 자체를 서술하는 정당한
/// 자리**가 있고, `corpus/` 는 **사전 등록된 측정 대장**이라 회차가 안 만진다
/// (소유자: *"범위 밖 — 측정 장치다"*), `target/`·`.git/` 는 산출물이다.
///
/// ⚠ 앞 판은 doc 과 게이트 합격선 ③b 가 둘 다 *"`docs/`·`.palimpsest/` **만** 뺀다"*
///   라고 적었는데 구현은 다섯이었다 — **선언과 구현이 갈린 자리**이고, 독립 리뷰 13
///   라운드가 `corpus/criteria.toml` 에 심어 확인했다(19/19 초록). 세는 문장이 자기
///   사각을 안 적으면 그 문장이 곧 꺼진 대조다.
///
/// ★ **`docs/` 아래여도 코드가 실행 시점에 여는 파일은 들어온다** — 그것은 문서가
///   아니라 **설정**이다. 손으로 적지 않고 [`실행시점_docs`] 가 소스에서 뽑는다.
fn 인용_모집단(root: &Path) -> Result<Vec<PathBuf>> {
    const 밖: &[&str] = &["docs/", ".palimpsest/", "target/", ".git/", "corpus/"];
    let 예외 = 실행시점_docs(root)?;
    // ★ **`.md` 도 본다** (독립 리뷰 8 라운드). 앞 판은 `.md` 를 빼서
    //   **설치 자산**(`crates/pal-cli/assets/**/*.md` — `include_str!` 로 사용자
    //   프로젝트에 실려 나간다)과 **하네스 표면 둘**(`.claude/agents/`·`.claude/skills/`)이
    //   통째로 사각이었다. 심어서 확인한 **꺼진 대조**다 — 거기 「옛」 없는 인용을 넣어도
    //   19/19 초록이었다. `docs/`·`.palimpsest/` 만 빼는 것이 이 검사의 선언이었으므로
    //   **선언과 구현이 갈린 자리**이기도 하다.
    const 확장자: &[&str] = &["rs", "toml", "py", "sh", "yml", "yaml", "json", "md"];
    let mut out = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(d) = stack.pop() {
        let Ok(읽기) = std::fs::read_dir(&d) else { continue };
        for e in 읽기 {
            let p = e?.path();
            let 상대 = 상대_경로(root, &p);
            if 밖.iter().any(|x| 상대.starts_with(x)) && !예외.iter().any(|x| 상대.starts_with(x)) {
                continue;
            }
            if p.is_dir() {
                stack.push(p);
            } else {
                let 이름 = p.file_name().and_then(|x| x.to_str()).unwrap_or("");
                let ext = p.extension().and_then(|x| x.to_str()).unwrap_or("");
                // ★ **시험도 훑는다** (독립 리뷰 11 라운드). 앞 판은 죽은 링크 검사에서
                //   *"시험은 가짜 경로를 만든다"* 를 사유째 복사해 `tests/` 를 뺐는데,
                //   **그 사유는 경로를 파일시스템에 대 보는 검사의 것**이지 산문 토큰을
                //   세는 이 검사와 무관하다. 사각이 하나 생겼고 거기 실물이 있었다
                //   (`crates/pal-store/tests/isolation.rs:1` 이 사라진 문서를 현재형으로).
                let _ = 이름;
                if 확장자.contains(&ext) || 이름 == ".gitignore" {
                    out.push(p);
                }
            }
        }
    }
    out.sort();
    Ok(out)
}

/// `docs/` 아래인데 **코드가 실행 시점에 여는** 파일 — 문서가 아니라 설정이다.
///
/// ★ **손으로 안 적는다.** `xtask` 소스의 `join("docs/…")` 리터럴에서 뽑으므로,
///   다음에 같은 종이 하나 더 생겨도 **자동으로 모집단에 든다.** 손으로 적은
///   예외 목록은 그때 조용히 샌다 — 이 회차가 만든 `docs/sunset.toml` 이 정확히
///   그렇게 샜다(독립 리뷰 13 라운드가 심어서 확인: 19/19 초록).
///
/// 사유는 `schema/graph.toml` 을 넣을 때와 **같은 것**이다 — 실행 시점에 읽히는
/// 단일 진실 파일의 머리가 사라진 문서를 근거로 세우면, 그것은 낡은 산문이 아니라
/// **낡은 설정**이다.
///
/// 비면 실패다 — 뽑기가 고장 나면 `docs/` 전체가 도로 사각이 된다.
fn 실행시점_docs(root: &Path) -> Result<Vec<String>> {
    let mut out = Vec::new();
    for f in rust_sources(&root.join("xtask"))? {
        let Ok(body) = std::fs::read_to_string(&f) else { continue };
        for line in body.lines() {
            // ⚠ **주석은 코드가 아니다.** 첫 판은 줄을 안 갈라서, 이 함수의 doc 주석에
            //    적힌 `join("docs/…")` 자신이 목록을 채웠다 — **하한이 죽어 있었다**
            //    (음성 대조를 돌렸더니 뽑기를 고장 내도 초록이었다). 자기 설명이
            //    자기 대조를 끄는 형태다.
            let 코드 = line.split("//").next().unwrap_or("");
            let mut from = 0;
            while let Some(rel) = 코드[from..].find("join(\"docs/") {
                let at = from + rel + "join(\"".len();
                let Some(end) = 코드[at..].find('"') else { break };
                let 경로 = 코드[at..at + end].to_string();
                if !out.contains(&경로) {
                    out.push(경로);
                }
                from = at + end;
            }
        }
    }
    if out.is_empty() {
        bail!("`docs/` 아래에서 실행 시점에 열리는 파일을 하나도 못 뽑았다 — 뽑기가 고장 났다");
    }
    out.sort();
    Ok(out)
}

/// 옛 계획 문서를 절 단위로 부르는 자리 — `F〇〇 §N` 형태의 시작 오프셋들.
///
/// 이 저장소의 계획 문서 25 개는 전부 같은 서식이었다(§1 왜 · §2 데이터 · §3 구현 ·
/// §4 이슈 · §5 대안 · §6 검증 · §7 완료 체크리스트). 코드는 **그 절 번호**를 근거로
/// 적었고, 2026-08-18 재고 처분이 그 문서 전부를 지웠다.
///
/// ⚠ **문자열 토큰으로는 못 잡는다** — 번호가 변수라서다. 그래서 이 회차는 303 곳에
/// 「옛」을 **손으로** 달았고 **59 곳을 흘렸다**(독립 리뷰 15 라운드가 실측).
/// **손으로 단 규약은 장치가 재기 전까지 샌 줄도 모른다.**
///
/// ★ **게이트 문서의 절이 아니다.** 살아 있는 `docs/gates/F〇〇.md` 도 번호 절을 쓰지만,
/// 이 표기는 계획 문서를 가리킨다 — `docs/gates/F03.md:62` 가 자기 §2 의 제목에
/// *"완료 체크리스트 (옛 F03 §7)"* 이라 적어 그것을 스스로 증언한다. §7 은 계획 문서
/// 서식의 마지막 절이고 게이트에는 그런 절이 없다.
fn 계획문서_인용(line: &str) -> Vec<usize> {
    let b = line.as_bytes();
    let mut out = Vec::new();
    for (i, _) in line.match_indices('F') {
        let mut j = i + 1;
        let mut 숫자 = 0;
        while j < b.len() && b[j].is_ascii_digit() {
            j += 1;
            숫자 += 1;
        }
        if 숫자 != 2 {
            continue;
        }
        if j < b.len() && b[j].is_ascii_lowercase() {
            j += 1;
        }
        if j + 1 < b.len() && b[j] == b'-' && b[j + 1].is_ascii_digit() {
            j += 2;
        }
        if line[j..].starts_with(" §") {
            out.push(i);
        }
    }
    out
}

/// 토큰 목록 선언 블록의 줄 범위(0-기준, 양끝 포함) — **면제는 여기서만 선다.**
///
/// 앞의 doc 주석부터 `];` 까지. 그 밖은 아무리 그럴듯해 보여도 면제 안 한다 —
/// 이 저장소는 규약을 말할 때 「「옛」 표기」라는 낱말을 쓰므로, 「표기」가 든 줄을
/// 전역으로 면제하면 **가장 잘 걸릴 문장 형태를 정확히 비켜 간다.**
fn 토큰_블록(body: &str) -> Option<(usize, usize)> {
    let lines: Vec<&str> = body.lines().collect();
    let 시작 = lines.iter().position(|l| l.starts_with("const 사라진_문서"))?;
    // doc 주석을 위로 올라가며 포함한다.
    let mut a = 시작;
    while a > 0 && lines[a - 1].trim_start().starts_with("///") {
        a -= 1;
    }
    let b = lines[시작..].iter().position(|l| l.trim() == "];")? + 시작;
    Some((a, b))
}
// ─────────────────────────────────────────────────────────────────────────────
// 회차 레코드 — **발견이 사라지지 않는 자리**
// ─────────────────────────────────────────────────────────────────────────────
//
// # 왜 이 검사가 생겼나 (2026-08-19 · [#71] · [#72])
//
// 앞 회차가 발견 108 건을 분류해 `retro/02-classification.tsv` 에 담았는데 **그 파일이
// 어느 검사의 모집단에도 없었다.** 분류가 틀려도 기계가 아무 말을 안 했고, 합계 검산은
// **사람이 돌렸다.** 그리고 축 1(발견이 유효했나)이 **참 109 · 거짓 0** 으로 반증됐다 —
// 축이 고장난 것이 아니라 **모집단이 고장났다.** 커밋에 남는 것은 채택된 발견뿐이다.
//
// ★ **이 검사가 재는 회차에서 계수 시도 셋 중 둘이 어긋났다** — 메인이 17 이라 말한 것이
// 18 이었고, 사전부검이 11 이라 적은 것이 12 였다. **사람도 에이전트도 자기 산출을 잘못
// 센다.** 그래서 합계 검산이 **독립된 둘째 원천**(보존된 원 반환문)을 댄다. 자기가 쓴 것을
// 자기가 세면 그것은 검산이 아니라 항등식이다.
//
// ⚠ **모집단이 비면 실패다.** 0 건은 「안 부른다」가 아니라 「안 봤다」일 수 있다.
//
// [#71]: https://github.com/hskim-ecoletree/palimpsest/issues/71
// [#72]: https://github.com/hskim-ecoletree/palimpsest/issues/72

/// 회차 산출이 사는 자리.
const 회차_뿌리: &str = ".palimpsest/rounds";

/// 스키마와 **한 줄의 정합 규칙**이 사는 유일한 자리. 이 검사는 `check` 를 **불러서**
/// 위임하고 파이썬 소스를 정규식으로 안 긁는다. 두 곳에 적으면 갈리고 갈린 것을 대는
/// 장치가 없다.
///
/// ⚠ 앞 판은 여기에 *"`--schema` 를 불러서 읽는다"* 라 적었는데 **R9 의 위임 뒤로는
/// `check` 만 부른다.** 주석이 구현과 갈렸고 독립 리뷰가 그것을 「사실이 아닌 것을
/// 사실로」로 판정했다.
const 스키마_원천: &str = ".claude/skills/round/bin/record.py";

/// **파이썬 실행자를 찾는 한 자리.**
///
/// ★ 이름은 플랫폼이 정한다 — `python3` 가 서는 곳도 있고 `python` 뿐인 곳도 있다.
/// 옛 ADR-0023 이 가른 대로 고를 축은 「볼 수 있는 쪽」이 아니라 **양쪽이 할 수 있는
/// 것**이고, 그 분기는 **여기 한 번**이어야 한다. 두 곳에서 각자 답하면 한쪽이 조용히
/// 낡는다(설치 쪽의 [`실행자 이름`] 이 같은 사유로 한 자리다).
///
/// **찾은 이름을 판정에 실어 낸다** — 세 OS 의 답이 CI 로그에 남는 것이 이 저장소가
/// 「쟀다」고 말할 수 있는 유일한 근거다.
///
/// [`실행자 이름`]: https://github.com/hskim-ecoletree/palimpsest/blob/main/crates/pal-cli/src/install/exe.rs
fn 파이썬_실행자() -> Result<&'static str> {
    for 이름 in ["python3", "python"] {
        let ok = std::process::Command::new(이름)
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);
        if ok {
            return Ok(이름);
        }
    }
    bail!(
        "파이썬 실행자를 못 찾았다 — `python3` 도 `python` 도 안 선다. \
         회차 레코드 검사는 스키마를 `{스키마_원천} --schema` 에 물어본다"
    )
}

/// `.palimpsest/rounds/**` 아래의 **기계 판독 산출** 전부.
///
/// ⚠ `*.md` 는 안 든다 — 사람이 쓰는 것이다. 갈래는 확장자가 정한다.
fn 회차_산출(root: &Path) -> Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    let base = root.join(회차_뿌리);
    if base.is_dir() {
        아래_전부_확장자들(&base, &["jsonl", "tsv"], &mut out)?;
    }
    out.sort();
    Ok(out)
}

fn 아래_전부_확장자들(dir: &Path, exts: &[&str], out: &mut Vec<PathBuf>) -> Result<()> {
    for e in std::fs::read_dir(dir).with_context(|| format!("읽지 못했다: {}", dir.display()))? {
        let p = e?.path();
        if p.is_dir() {
            아래_전부_확장자들(&p, exts, out)?;
        } else if p
            .extension()
            .and_then(|x| x.to_str())
            .is_some_and(|x| exts.contains(&x))
        {
            out.push(p);
        }
    }
    Ok(())
}

/// 보존된 원 반환문에서 **항 수**를 센다 — 합계 검산의 둘째 원천.
///
/// ★ **출처마다 규칙이 다르다.** 사전부검자는 `### 항`으로 내고 리뷰어는 **표**로 낸다.
/// 규칙을 여기 적는 까닭은, 파일마다 절 구성이 달라서(실측: `##` 절이 1 개인 반환문과
/// 2 개인 반환문이 있다) **「전부 세기」가 원리상 안 되기 때문**이다.
///
/// | 출처 | 규칙 |
/// |---|---|
/// | 사전부검 | `^### ` 항 + `## 내가 기각한 것` 아래 최상위 항 (**불릿이든 표든**) |
/// | 독립리뷰 | **`| # |` 헤더를 가진 표**의 데이터 행 |
///
/// ⚠ 리뷰어의 `## 합격선 축` 표는 `| 조건 |` 로 시작하므로 **안 걸린다** — 그것은
/// 발견이 아니라 등록된 조건에 대한 판정이고 게이트가 진다.
fn 반환문_항_수(
    합계검산: &serde_json::Value,
    발견아닌절: &[String],
    출처: &str,
    text: &str,
) -> usize {
    // ★★ **코드펜스 안은 안 센다.** (독립 리뷰 R3 · 발견 6)
    //   반환문이 **마크다운 형식을 예시로 인용**하면 그 안의 항 표시와 표 머리가
    //   발견으로 세어진다 — 계수기도 추출기도 펜스를 안 봤다. 그러면 **두 원장이
    //   같이 부풀고** 초록으로 만드는 자연스러운 길이 「없는 레코드를 지어내기」다.
    let text = 펜스_밖(text);
    let text = text.as_str();
    // ★★ **판정이지 발견이 아닌 절은 안 센다.** (같은 라운드 · 발견 4)
    //   「미측정 목록」은 합격선 축 판정의 일부다. 그것을 항으로 세면 레코드가 되고,
    //   그러면 **A 축이 그 행을 닫을 때 발화한다** — `K1`·`K5` 행의 좌표는 워크플로
    //   파일인데 그 조건은 **CI 런으로** 재어지므로 그것을 만지는 커밋이 원리상 없다.
    let 규칙 = 합계검산;
    let _ = 발견아닌절;
    let 표머리 = 규칙["독립리뷰표머리"].as_str().unwrap_or("| # |");
    let 항머리 = 규칙["사전부검항"].as_str().unwrap_or("### ");
    let 기각절 = 규칙["사전부검기각절"].as_str().unwrap_or("내가 기각한 것");
    if 출처 == "독립리뷰" {
        let mut n = 0usize;
        let mut 표안 = false;
        for line in text.lines() {
            if line.starts_with("## ") {
                표안 = false;
                continue;
            }
            if line.starts_with(표머리) {
                표안 = true;
                continue;
            }
            if 표안 {
                let t = line.trim_start();
                if t.starts_with("|-") || t.starts_with("| -") || t.starts_with("|:") {
                    continue;
                }
                if t.starts_with('|') {
                    n += 1;
                    continue;
                }
                표안 = false;
            }
        }
        return n;
    }
    let mut 시나리오 = 0usize;
    let mut 기각 = 0usize;
    let mut 기각_절 = false;
    for line in text.lines() {
        if let Some(제목) = line.strip_prefix("## ") {
            기각_절 = 제목.contains(기각절);
            continue;
        }
        if line.starts_with(항머리) {
            시나리오 += 1;
            기각_절 = false;
        } else if 기각_절 {
            // ★ **불릿이든 표든 센다.** (정정 2026-08-19 · 독립 리뷰 3 라운드)
            // ★★ **헤더 행은 문구가 무엇이든 안 센다.** (2026-08-24 · `B5`)
            let s = line.trim_start();
            if s.starts_with("- ") {
                기각 += 1;
            } else if s.starts_with('|') && !s.starts_with("|-") && !s.starts_with("| -")
                && !s.starts_with("|:")
            {
                기각 += 1;
            }
        }
    }
    시나리오 + 기각 - 기각_표_헤더(text, 기각절)
}

/// 코드펜스 안을 빈 줄로 만든다 — **예시로 인용한 형식이 발견으로 세어지지 않게.**
///
/// **순수 함수다.**
fn 펜스_밖(text: &str) -> String {
    let mut out = Vec::new();
    let mut 안 = false;
    for l in text.lines() {
        if l.trim_start().starts_with("```") {
            안 = !안;
            out.push("");
            continue;
        }
        out.push(if 안 { "" } else { l });
    }
    out.join("\n")
}

#[cfg(test)]
mod 펜스_시험 {
    use super::{반환문_항_수, 펜스_밖};

    fn 규칙() -> serde_json::Value {
        serde_json::json!({
            "사전부검항": "### ", "사전부검기각절": "내가 기각한 것",
            "독립리뷰표머리": "| # |"
        })
    }

    #[test]
    fn 펜스_안을_비운다() {
        let t = "밖\n```\n### 안\n```\n밖2";
        assert!(!펜스_밖(t).contains("### 안"));
        assert!(펜스_밖(t).contains("밖2"));
    }

    /// ★★ **예시로 인용한 형식이 항으로 세어지면 두 원장이 같이 부푼다.**
    /// 그러면 초록으로 만드는 길이 「없는 레코드를 지어내기」가 된다.
    #[test]
    fn 펜스_안의_예시는_항이_아니다() {
        let t = "### 진짜\n\n```markdown\n### 예시\n```\n\n## 내가 기각한 것\n\n없음\n";
        assert_eq!(반환문_항_수(&규칙(), &[], "사전부검", t), 1);
    }

    /// ⚠ **판정이지 발견이 아닌 절도 센다** — 세는 자를 바꾸면 **옛 회차의 검산이
    /// 깨진다**(실측: 옛 회차 하나가 24↔25 로 갈렸다). 대신 **처분이 그것을 가른다** —
    /// 「미측정 목록」 행은 `처분=기각` 이고, `닫힘축` 이 그것을 원리상 못 잼으로 뺀다.
    #[test]
    fn 절이_달라도_표는_전부_센다() {
        let t = "## 미측정 목록\n\n| # | a |\n|---|---|\n| 1 | x |\n\n\
                 ## 새 발견\n\n| # | b |\n|---|---|\n| 1 | y |\n";
        assert_eq!(반환문_항_수(&규칙(), &[], "독립리뷰", t), 2);
    }
}

/// 기각 절 안의 **표 헤더 행 수** — 문구가 무엇이든 뺀다.
///
/// ★ **자리가 가른다.** 구분선(`|---`) 바로 위의 행이 헤더다. 앞 판은 `| #` 라는
/// **문구**로 갈랐고, 그래서 헤더를 다르게 적은 반환문에서 항이 부풀었다.
///
/// **순수 함수다.**
fn 기각_표_헤더(text: &str, 기각절: &str) -> usize {
    let mut n = 0usize;
    let mut 기각_절 = false;
    let lines: Vec<&str> = text.lines().collect();
    for (i, line) in lines.iter().enumerate() {
        if let Some(제목) = line.strip_prefix("## ") {
            기각_절 = 제목.contains(기각절);
            continue;
        }
        if line.starts_with("### ") {
            기각_절 = false;
            continue;
        }
        if !기각_절 {
            continue;
        }
        let s = line.trim_start();
        if !s.starts_with('|') || s.starts_with("|-") || s.starts_with("| -") || s.starts_with("|:")
        {
            continue;
        }
        let 다음 = lines.get(i + 1).map(|l| l.trim_start()).unwrap_or("");
        if 다음.starts_with("|-") || 다음.starts_with("| -") || 다음.starts_with("|:") {
            n += 1;
        }
    }
    n
}

#[cfg(test)]
mod 기각_표_헤더_시험 {
    use super::기각_표_헤더;

    /// ★ **문구가 무엇이든 헤더는 헤더다.** 실측: `| 기각한 것 |` 헤더가
    /// 데이터 행으로 세어져 항이 25 → 26 으로 부풀었다(`B5`).
    #[test]
    fn 문구가_달라도_헤더를_뺀다() {
        let t = "## 내가 기각한 것\n\n| 기각한 것 | 왜 |\n|---|---|\n| a | b |\n| c | d |\n";
        assert_eq!(기각_표_헤더(t, "내가 기각한 것"), 1);
    }

    #[test]
    fn 샾_헤더도_뺀다() {
        let t = "## 내가 기각한 것\n\n| # | 제목 |\n|---|---|\n| 1 | a |\n";
        assert_eq!(기각_표_헤더(t, "내가 기각한 것"), 1);
    }

    /// 불릿만 있는 기각 절에는 헤더가 없다.
    #[test]
    fn 불릿_절에는_헤더가_없다() {
        let t = "## 내가 기각한 것\n\n- 하나\n- 둘\n";
        assert_eq!(기각_표_헤더(t, "내가 기각한 것"), 0);
    }

    /// 다른 절의 표는 안 센다.
    #[test]
    fn 다른_절의_표는_안_센다() {
        let t = "## 다른 절\n\n| a | b |\n|---|---|\n| 1 | 2 |\n";
        assert_eq!(기각_표_헤더(t, "내가 기각한 것"), 0);
    }
}

/// 원 반환문이 사는 디렉터리 ↔ 그것을 낸 **출처**.
///
/// ★ **인터뷰와 실측은 여기 없다.** 인터뷰는 소유자와의 대화이고 실측은 메인의 관측이라
/// **에이전트 반환문이 원리상 없다.** 그래서 검산에서 면제하되 — 면제라는 사실을 판정에
/// 실어 낸다. 「안 잰 것」과 「잴 수 없는 것」을 같은 침묵으로 두지 않는다.
const 반환문_자리: &[(&str, &str)] = &[("premortem", "사전부검"), ("review", "독립리뷰")];

/// 산출 파일이 어느 회차의 것인가 — `.palimpsest/rounds/<회차>/…` 의 `<회차>`.
fn 회차_이름(root: &Path, p: &Path) -> String {
    상대_경로(root, p)
        .strip_prefix(&format!("{회차_뿌리}/"))
        .and_then(|s| s.split('/').next())
        .unwrap_or("")
        .to_string()
}

/// 그 절이 섰는가 — **데이터 표가 있거나, 「없음」이 명시돼 있거나.**
///
/// ★★ **이것이 「안 냈다」와 「낼 것이 없다」를 가르는 자다** (#93).
/// 둘을 같은 침묵으로 두면 계기판 ⑧(발견의 몇 %가 헛것인가)이 **조용히 빈다** —
/// 실측: 앞 회차 리뷰어 여덟 중 기각 절을 낸 것은 **둘뿐**이었다.
///
/// **순수 함수다.**
fn 절이_섰나(text: &str, 절: &str, 없음: &str) -> Result<(), String> {
    let mut 안 = false;
    let mut 데이터 = false;
    let mut 없음_적힘 = false;
    for line in text.lines() {
        if let Some(h) = line.strip_prefix("## ") {
            if 안 {
                break;
            }
            안 = h.trim().contains(절);
            continue;
        }
        if !안 {
            continue;
        }
        let s = line.trim();
        if s.is_empty() {
            continue;
        }
        if s.starts_with('|') {
            if !(s.starts_with("|-") || s.starts_with("| -") || s.starts_with("|:")) {
                데이터 = true;
            }
            continue;
        }
        if s.starts_with("- ") || s.contains(없음) {
            if s.contains(없음) {
                없음_적힘 = true;
            } else {
                데이터 = true;
            }
        }
    }
    if !안 && !데이터 && !없음_적힘 {
        return Err(format!(
            "`## {절}` 절이 없다 — **「안 냈다」와 「낼 것이 없다」는 다르다.** \
             낼 것이 없으면 그 절을 두고 **표 밖에 「{없음}」이라 적어라"
        ));
    }
    if !데이터 && !없음_적힘 {
        return Err(format!(
            "`## {절}` 절이 비었다 — 데이터도 「{없음}」 선언도 없다. \
             **빈 침묵은 「안 냈다」와 구별이 안 된다**"
        ));
    }
    Ok(())
}

#[cfg(test)]
mod 절이_섰나_시험 {
    use super::절이_섰나;

    #[test]
    fn 데이터가_있으면_선다() {
        let t = "## 내가 기각한 것\n\n| # | a |\n|---|---|\n| 1 | b |\n";
        assert!(절이_섰나(t, "내가 기각한 것", "없음").is_ok());
    }

    #[test]
    fn 없음이_명시되면_선다() {
        let t = "## 내가 기각한 것\n\n없음 — 전부 남겼다.\n";
        assert!(절이_섰나(t, "내가 기각한 것", "없음").is_ok());
    }

    /// ★★ **절이 아예 없으면 「안 냈다」다.** 그것과 「낼 것이 없다」를 가른다.
    #[test]
    fn 절이_없으면_걸린다() {
        let t = "## 다른 절\n\n- 하나\n";
        assert!(절이_섰나(t, "내가 기각한 것", "없음").is_err());
    }

    /// 절머리만 두고 비워 두는 것도 「안 냈다」와 구별이 안 된다.
    #[test]
    fn 절이_비면_걸린다() {
        let t = "## 내가 기각한 것\n\n## 그다음\n\n- 무언가\n";
        assert!(절이_섰나(t, "내가 기각한 것", "없음").is_err());
    }
}

/// 원 반환문에서 레코드를 **기계로** 뽑는 스크립트.
const 추출기: &str = ".claude/skills/round/bin/extract.py";

/// 추출기를 돌려 그 라운드의 기계 칸을 얻는다.
///
/// ★★ **전사를 사람이 안 하면 #92 의 병이 원리상 없어진다.** 앞 판은 메인이 반환문을
/// 눈으로 읽고 손으로 레코드를 적었고, 그 자리에서 **세 번 조용히 행이 떨어졌다** —
/// 두 번이 기각 행에 몰렸고 그러면 계기판 ⑧ 이 낮게 나온다. 그리고 합계 검산은
/// **행 수만** 세므로 **수만 맞고 내용이 다른 20**이 초록으로 지나갔다.
fn 추출기_산출(
    root: &Path,
    파이썬: &str,
    출처: &str,
    라운드: i64,
    raw: &Path,
) -> Result<Vec<serde_json::Value>> {
    let out = 파이썬_명령(파이썬)
        .current_dir(root)
        .arg(root.join(추출기))
        .arg(출처)
        .arg(라운드.to_string())
        .arg(raw)
        // 역사 형식은 원문 자체의 고유 표지로 추출기가 판별한다. 이 내부 표시는
        // 레코드에 저장하지 않고, 대조할 기계 칸 집합을 고르는 데만 쓴다.
        .env("PAL_ROUND_EXTRACT_REPORT_PROFILE", "1")
        .output()
        .with_context(|| format!("`{추출기}` 를 못 돌렸다"))?;
    if !out.status.success() {
        bail!("{}", String::from_utf8_lossy(&out.stderr).trim().to_string());
    }
    String::from_utf8_lossy(&out.stdout)
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).context("추출기 출력이 JSON 이 아니다"))
        .collect()
}

fn 원문순서_대조(
    뽑은: &[serde_json::Value],
    행들: &[serde_json::Value],
    기계_칸: &[String],
) -> (usize, usize, Vec<String>) {
    let mut 갈린_칸 = 0;
    let 빠진_행 = 뽑은.len().abs_diff(행들.len());
    let mut 갈림 = Vec::new();
    for (i, (e, r)) in 뽑은.iter().zip(행들).enumerate() {
        for 칸 in 기계_칸 {
            let (a, b) = (e[칸].as_str().unwrap_or(""), r[칸].as_str().unwrap_or(""));
            if a != b {
                갈린_칸 += 1;
                갈림.push(format!("원문 {}번째.{칸}", i + 1));
            }
        }
    }
    if 빠진_행 > 0 {
        갈림.push(format!("원문과 레코드의 행 수가 {}↔{}", 뽑은.len(), 행들.len()));
    }
    (갈린_칸, 빠진_행, 갈림)
}

#[cfg(test)]
mod 원문순서_대조_시험 {
    use super::원문순서_대조;
    use serde_json::json;

    #[test]
    fn 역사_id는_대조하지_않고_기계칸과_빠진행을_가른다() {
        let 칸 = vec!["요약".to_string()];
        let a = vec![json!({"id":"IR1-01", "요약":"같음"})];
        let b = vec![json!({"id":"IR1-M01", "요약":"같음"})];
        let 같음 = 원문순서_대조(&a, &b, &칸);
        assert_eq!((같음.0, 같음.1), (0, 0));
        let c = vec![json!({"요약":"다름"}), json!({"요약":"추가"})];
        let r = 원문순서_대조(&c, &b, &칸);
        assert_eq!((r.0, r.1), (1, 1));
    }
}

fn check_round_records(root: &Path) -> Result<String> {
    let 산출 = 회차_산출(root)?;

    // ⚠ 모집단이 비면 실패다.
    if 산출.is_empty() {
        bail!(
            "`{회차_뿌리}/**` 에 기계 판독 산출(`*.jsonl`·`*.tsv`)이 하나도 없다 — \
             이 검사가 아무것도 안 잰다. 0 건은 「안 부른다」가 아니라 「안 봤다」다"
        );
    }

    let mut problems = Vec::new();

    // 스키마 원천에 먼저 물어본다 — `종류` 목록도 거기 산다.
    let 파이썬 = 파이썬_실행자()?;
    let 원천 = root.join(스키마_원천);
    if !원천.exists() {
        bail!("스키마 원천이 없다: {스키마_원천}");
    }
    let 스키마_출력 = 파이썬_명령(파이썬)
        .arg(&원천)
        .arg("--schema")
        .output()
        .with_context(|| format!("`{파이썬} {스키마_원천} --schema` 를 못 돌렸다"))?;
    if !스키마_출력.status.success() {
        bail!(
            "`{스키마_원천} --schema` 가 실패했다:\n{}",
            String::from_utf8_lossy(&스키마_출력.stderr)
        );
    }
    let 스키마: serde_json::Value =
        serde_json::from_slice(&스키마_출력.stdout).context("`--schema` 출력이 JSON 이 아니다")?;
    // ★ **행과 칸은 다른 것이다.** `B4` 는 「라운드 수가 아니라 칸 수」를 요구했는데
    //   앞 판은 둘을 한 수로 합쳤다 — 2313 중 596 이 행이었다(독립 리뷰 R1 · 발견 8).
    let (mut 갈린_칸, mut 빠진_행) = (0usize, 0usize);
    // ★★ **면제 출처는 선언이 정한다.** 이 목록이 검산 면제의 **갈래**를 결정한다 —
    //   비면 그 출처가 「선언 어디에도 없다」로 빨개진다. 표시 전용이 아니다.
    let 면제_출처: Vec<String> = 문자열들(&스키마["합계검산"]["면제출처"]);

    let 종류_목록: Vec<String> = 스키마["종류"]
        .as_array()
        .context("`--schema` 에 `종류` 가 없다 — 가르는 축은 스스로 선언돼야 한다")?
        .iter()
        .filter_map(|v| v.as_str().map(str::to_owned))
        .collect();

    // ① 각 파일이 **자기 스키마를 선언**하는가.
    for p in &산출 {
        let 상대 = 상대_경로(root, p);
        let text = std::fs::read_to_string(p).with_context(|| format!("읽지 못했다: {상대}"))?;
        let 첫줄 = text.lines().find(|l| !l.trim().is_empty()).unwrap_or("");
        let ext = p.extension().and_then(|x| x.to_str()).unwrap_or("");
        match ext {
            "jsonl" => {
                if !첫줄.contains("\"schema_version\"") {
                    problems.push(format!("{상대}: 머리 줄에 `schema_version` 이 없다"));
                }
                // ★ **자기 「종류」를 선언해야 한다.** (2026-08-19 · 독립 리뷰 3 라운드)
                //   앞 판은 `findings.jsonl` 이라는 **이름**만 행 검증을 받았고, 다른
                //   `.jsonl` 은 머리 줄만 보고 통과하면서 `산출 N개` 에는 세어져
                //   **「쟀다」로 보였다.** 이름이 아니라 선언이 갈라야 한다.
                // ★ **`종류` 목록도 원천에 물어본다.** (정정 2026-08-19 · 독립 리뷰 5 라운드)
                //   앞 판은 여기에 `레코드`·`예외표` 를 **다시 적었고**, 그것이
                //   *"스키마의 유일한 자리"* 라는 C2-b 의 문장을 깨뜨렸다.
                // ⚠ **여기도 JSON 으로 읽는다.** (독립 리뷰 R5) 앞 판은 공백까지 박힌
                //    정확 문자열이라 `{"종류":"예외표"}` 처럼 공백만 빠져도
                //    *"원천이 선언한 목록 밖이다"* 라고 **거짓을 말했다** — 값은 목록 안인데.
                let 선언된_종류 = serde_json::from_str::<serde_json::Value>(첫줄)
                    .ok()
                    .and_then(|v| v.get("종류").and_then(|x| x.as_str()).map(str::to_owned));
                let 종류_ok = 선언된_종류.as_deref().is_some_and(|k| 종류_목록.iter().any(|x| x == k));
                if 선언된_종류.is_some() && !종류_ok {
                    problems.push(format!(
                        "{상대}: 머리 줄의 `종류` 가 원천이 선언한 {종류_목록:?} 밖이다"
                    ));
                }
                if 선언된_종류.is_none() {
                    problems.push(format!(
                        "{상대}: 머리 줄에 `종류` 가 없다 — `레코드` 인지 `예외표` 인지 \
                         선언해야 행 검증을 어느 자로 잴지 정해진다"
                    ));
                }
            }
            "tsv" => {
                if 첫줄.starts_with('#') || !첫줄.contains('\t') {
                    problems.push(format!(
                        "{상대}: 헤더 행이 없다 — 열 이름을 탭으로 가른 첫 줄이 스키마 선언이다"
                    ));
                }
                // 열 수가 갈리면 세는 자가 틀린다.
                let 열 = 첫줄.split('\t').count();
                for (i, line) in text.lines().enumerate().skip(1) {
                    if line.trim().is_empty() {
                        continue;
                    }
                    let n = line.split('\t').count();
                    if n != 열 {
                        problems.push(format!(
                            "{상대}:{}: 열이 {n} 인데 헤더는 {열} 이다",
                            i + 1
                        ));
                        break;
                    }
                }
            }
            _ => {}
        }
    }

    // ② **한 줄의 내부 정합은 원천에 위임한다.** (정정 2026-08-19 · 독립 리뷰 2 라운드)
    //
    // 앞 판은 `--schema` 로 enum 만 받아 **Rust 로 다시 검증**했다. 그래서 `record.py` 가
    // 아는 규칙 셋 — **대응표**(`전환`↛`승격됨=아니오` · `완화`↛`축소`)와 **모르는 필드** —
    // 이 CI 에 안 들어왔고, 그 규칙을 부르는 자가 **0** 이었다. C1-d 가 세운 「위장한 정정을
    // 가리는 축」이 **태어나면서 죽은 가지**였다(격리 사본에서 재현: `xtask` ok / `check` rc=1).
    //
    // ★ **역할을 가른다** — 한 줄 안의 정합은 `record.py check` 가, **파일 사이**의 정합
    // (모집단 · 좌표 해소 · 합계 검산 · tsv 열)은 여기가 잰다. 두 벌이 아니라 위임이다.
    // 이름이 아니라 **머리 줄의 선언**으로 고른다.
    let mut 레코드들: Vec<&PathBuf> = Vec::new();
    for p in &산출 {
        if p.extension().and_then(|x| x.to_str()) != Some("jsonl") {
            continue;
        }
        let text = std::fs::read_to_string(p)?;
        let 첫줄 = text.lines().find(|l| !l.trim().is_empty()).unwrap_or("");
        // ★ **예외표도 넘긴다.** (정정 2026-08-23 · 독립 리뷰 R2)
        //   `record.py` 에 `종류=예외표` 갈래를 만들어 놓고 여기서 `레코드` 만 넘겼더니
        //   **그 갈래의 호출자가 0** 이었다 — 태어나면서 죽은 가지다. 갈래를 만든
        //   손이 같은 자리에서 그것을 안 부르는 것이 이 저장소가 반복해 온 병이다.
        //   행 검증의 자는 `record.py` 가 머리 줄의 `종류` 로 고른다.
        // 같은 규율 — 선언이 **있으면** 넘긴다. 값은 `record.py` 가 읽는다.
        let 선언 = serde_json::from_str::<serde_json::Value>(첫줄)
            .ok()
            .and_then(|v| v.get("종류").and_then(|x| x.as_str()).map(str::to_owned));
        if 선언.is_some() {
            레코드들.push(p);
        }
    }
    if !레코드들.is_empty() {
        let out = 파이썬_명령(파이썬)
            .arg(&원천)
            .arg("check")
            .args(레코드들.iter().map(|p| p.as_os_str()))
            .output()
            .with_context(|| format!("`{파이썬} {스키마_원천} check` 를 못 돌렸다"))?;
        if !out.status.success() {
            let mut 실었나 = false;
            for line in String::from_utf8_lossy(&out.stderr).lines() {
                let l = line.trim();
                if !l.is_empty() {
                    problems.push(l.trim_start_matches("✗ ").to_string());
                    실었나 = true;
                }
            }
            // ★ **rc 를 버리지 않는다.** (정정 2026-08-19 · 독립 리뷰 3 라운드)
            //   앞 판은 실패 신호를 stderr 텍스트에서만 읽어서, **rc≠0 인데 stderr 가
            //   비면 검사가 초록**이었다. 규약이 *"`rc` 는 판정이 아니다"* 라고 적은 것은
            //   **rc 만으로 판정하지 말라**는 뜻이지 **rc 를 무시하라**는 뜻이 아니다.
            if !실었나 {
                problems.push(format!(
                    "`{스키마_원천} check` 가 rc={} 로 실패했는데 아무 말도 안 했다 — \
                     원천이 고장났다",
                    out.status.code().unwrap_or(-1)
                ));
            }
        }
    }

    // ③ 레코드의 각 행이 스키마를 지키는가 + `경로` 가 실재하는가.
    // ★ **회차로 먼저 가른다.** (정정 2026-08-19 · 독립 리뷰 2 라운드)
    //   앞 판은 저장소 전역 `(출처, 라운드)` 맵이라 **다음 회차가 자기 레코드를 놓는
    //   순간 두 회차가 서로를 거짓 실패시켰다** — 그리고 오류 문장이 그 회차에 없는
    //   수를 「레코드는 N 행이다」로 적었다. 사실이 아닌 것을 사실로 적는 자리다.
    let mut 쌍_수: std::collections::BTreeMap<(String, String, i64), usize> = Default::default();
    // ★ 기계 칸 대조용 — `(회차, 출처, 라운드)` → 그 라운드의 레코드 행들.
    let mut 쌍_행: std::collections::BTreeMap<(String, String, i64), Vec<serde_json::Value>> =
        Default::default();
    let mut 좌표_해소_실패 = Vec::new();
    let mut 좌표_면제 = 0usize;
    let mut 총_행 = 0usize;
    let mut 예외_행 = 0usize;
    for p in 레코드들.iter().copied() {
        let 상대 = 상대_경로(root, p);
        let text = std::fs::read_to_string(p)?;
        // 머리 줄의 선언으로 종류를 고른다 — 이름이 아니라 선언이 가른다.
        // ⚠ **공백에 기대지 않는다.** (독립 리뷰 R4) 앞 판은 `"종류": "예외표"` 라는
        //    **정확 문자열**을 찾아서, 공백 하나만 달라도 예외표가 레코드로 세어지고
        //    판정문의 두 수가 **조용히 갈렸다.** JSON 은 JSON 으로 읽는다.
        let 종류_ = serde_json::from_str::<serde_json::Value>(text.lines().next().unwrap_or(""))
            .ok()
            .and_then(|v| v.get("종류").and_then(|x| x.as_str()).map(str::to_owned))
            .unwrap_or_else(|| "레코드".to_string());
        let 종류_ = 종류_.as_str();
        for (i, line) in text.lines().enumerate() {
            if line.trim().is_empty() {
                continue;
            }
            let v: serde_json::Value = match serde_json::from_str(line) {
                Ok(v) => v,
                Err(e) => {
                    problems.push(format!("{상대}:{}: JSON 이 아니다 — {e}", i + 1));
                    continue;
                }
            };
            if i == 0 && v.get("schema_version").is_some() {
                continue;
            }
            // ★ **종류마다 따로 센다.** (독립 리뷰 R3) 예외표를 함께 넘기게 고친 뒤
            //   판정문이 예외표 행까지 「레코드 N행」이라 불렀다 — `--schema` 가 `종류` 를
            //   둘로 선언하는데 판정문은 하나로 뭉갰다.
            총_행 += 1;
            if 종류_ == "예외표" {
                예외_행 += 1;
            }
            if let (Some(r), Some(s)) = (
                v.get("라운드").and_then(|x| x.as_i64()),
                v.get("출처").and_then(|x| x.as_str()),
            ) {
                let 열쇠 = (회차_이름(root, p), s.to_string(), r);
                *쌍_수.entry(열쇠.clone()).or_default() += 1;
                if 종류_ == "레코드" {
                    쌍_행.entry(열쇠).or_default().push(v.clone());
                }
            }
            // ★ `경로` 만 해소한다. `줄` 은 안 잰다 — 회차가 자기 좌표를 밀어내기 때문이다
            //   (실측 2026-08-19: 에이전트 정의를 고치니 인용한 줄이 93 → 95 로 밀렸다).
            //   드리프트는 `기준커밋` 이 설명한다.
            if let Some(경로) = v.get("경로").and_then(|x| x.as_str()) {
                // ★★ **저장소 밖 절대경로는 좌표가 아니다.** (2026-08-24 · 사전부검 R3)
                //   앞 판은 `root.join(경로)` 를 그냥 불렀는데, 유닉스에서 절대경로는
                //   **뿌리를 통째로 대체**해 `/tmp` 가 그대로 서고 Windows 에서는
                //   `C:\tmp` 가 되어 없다. **한 행 때문에 windows-latest 만 죽었고
                //   CI 가 세 커밋 연속 실패였다** — 그리고 이 회차의 RED 표가 macOS 의
                //   `21/21` 을 착수 기준선으로 적었다(「사실이 아닌 것을 사실로」).
                //   ⚠ **면제하되 수를 판정문에 싣는다** — 「안 잰 것」과 「잴 수 없는 것」을
                //   같은 침묵으로 두지 않는다.
                if 경로 != "(경로 없음)" {
                    if 저장소_밖_절대경로(경로) {
                        좌표_면제 += 1;
                    } else if !좌표가_실재하는가(root, 경로) {
                        좌표_해소_실패.push(format!("{상대}:{}: `{경로}` 가 없다", i + 1));
                    }
                }
            }
        }
    }
    problems.extend(좌표_해소_실패);

    // ④ **합계 검산 — 독립된 둘째 원천을 댄다.**
    //
    // ★ **(출처, 라운드) 쌍으로 센다.** 라운드 번호만으로 세면 ① 같은 라운드에 두 출처가
    //   섞였을 때 **멀쩡한 레코드가 거짓 실패**를 내고, ② 반환문 파일이 없는 출처는
    //   **아무 검산도 안 받는다** — 그것이 「측정이 죽은 가지」다(독립 리뷰 2026-08-19).
    let mut 검산 = Vec::new();
    let mut 손_전사 = 0usize;
    // ★ 절 목록은 `--schema` 가 진다 — 검사 쪽에 손으로 안 벤다(#94 와 같은 병).
    let 반환형식 = &스키마["반환형식"];
    let 없음표시 = 반환형식["없음표시"].as_str().unwrap_or("없음");
    // ★ **판정이지 발견이 아닌 절** — 세지 않는다(독립 리뷰 R3 · 발견 4).
    let 발견아닌절: Vec<String> = Vec::new();
    if 반환형식["반드시있어야하는절"].as_object().map_or(true, |m| m.is_empty()) {
        bail!("`--schema` 의 `반환형식.반드시있어야하는절` 이 비었다 — 이 검사가 아무것도 안 잰다");
    }
    let mut 진행중_손_전사 = 0usize;
    let mut 면제: std::collections::BTreeMap<String, usize> = Default::default();
    let 회차들 = std::fs::read_dir(root.join(회차_뿌리))?;
    for e in 회차들 {
        let dir = e?.path();
        if !dir.is_dir() {
            continue;
        }
        for (자리, 출처) in 반환문_자리 {
            let d = dir.join(자리);
            if !d.is_dir() {
                continue;
            }
            for e2 in std::fs::read_dir(&d)? {
                let f = e2?.path();
                let name = f.file_name().and_then(|x| x.to_str()).unwrap_or("").to_string();
                let Some(n) = name
                    .strip_prefix('r')
                    .and_then(|s| s.strip_suffix("-raw.md"))
                    .and_then(|s| s.parse::<i64>().ok())
                else {
                    continue;
                };
                // ★ **기계 칸은 출처마다 다르다** — 선언이 정한다(독립 리뷰 R2).
                let 기계_칸: Vec<String> = 문자열들(&반환형식["기계칸"][출처]);
                if 기계_칸.is_empty() {
                    bail!("`--schema` 의 `반환형식.기계칸.{출처}` 가 비었다 — 이 대조가 아무것도 안 잰다");
                }
                let raw본문 = std::fs::read_to_string(&f)?;
                let 항 = 반환문_항_수(&스키마["합계검산"], &발견아닌절, 출처, &raw본문);

                // ── D2 — **「안 냈다」와 「낼 것이 없다」를 가른다** (#93) ────────
                //
                // ★ 앞 판은 에이전트에게 **일곱 표 전부**를 데이터 표로 내라고
                //   시켰고, 그래서 *"빠진 것 — 없음"* 같은 **자리 채우기 행**이
                //   검산 모집단에 들었다. 검산을 맞추려면 그것도 레코드가 되어야
                //   했고, 스키마에 「발견 아님」 칸이 없어 `유효성=거짓 · 처분=기각`
                //   이 붙었다 — **계기판 ⑦ 이 그것을 원 의도 발견으로 세고 ⑧ 이
                //   부풀었다.** 형식을 요구한 까닭이 *"⑦⑧ 이 조용히 작아진다"*
                //   였는데 **부호만 뒤집힌 셈**이었다.
                //
                //   이제 「없음」은 **표 밖의 명시 문장**이다. 그러면 자리 채우기
                //   행이 원천에서 사라지고, 「안 냈다」와 「낼 것이 없다」는
                //   **그 문장이** 가른다.
                //
                // ⚠ **닫힌 집합으로 안 본다.** 실측: 리뷰어들이 정의에 없는 절을
                //   추가한다(`## 어떻게 조사했나` 8/8). 닫힌 집합으로 검사하면
                //   그것이 전부 형식 오류가 되고, 그러면 **축을 무르게 하려는
                //   압력**이 회차 안에서 생긴다. **「반드시」만 본다.**
                // ★ **출처마다 다르다** — 없는 절을 요구하면 그 반환문이 전량
                //   형식 오류가 된다(실측).
                let 반드시_절: Vec<String> = 문자열들(&반환형식["반드시있어야하는절"][출처]);
                if 반드시_절.is_empty() {
                    bail!("`--schema` 의 `반환형식.반드시있어야하는절.{출처}` 가 비었다");
                }
                if !기록이_확정됐나(&dir) {
                    for 절 in 반드시_절.iter() {
                        if let Err(e) = 절이_섰나(&raw본문, 절, 없음표시) {
                            problems.push(format!("{}: {e}", 상대_경로(root, &f)));
                        }
                    }
                }
                let 회차 = dir
                    .file_name()
                    .and_then(|x| x.to_str())
                    .unwrap_or("")
                    .to_string();
                let 적힌 = 쌍_수
                    .get(&(회차.clone(), 출처.to_string(), n))
                    .copied()
                    .unwrap_or(0);
                검산.push(format!("{출처}R{n} {항}↔{적힌}"));

                // ── B2·B4 — **기계 칸을 추출기 산출과 댄다** (#92) ──────────────
                //
                // ★★ 합계 검산은 **수**만 댄다. 그래서 **수만 맞고 내용이 다른 20**이
                //    초록으로 지나갔다(실측). 이제 **추출기를 돌려 기계 칸을 댄다.**
                //
                //    | 기계가 결정한다 | 사람이 판단한다 |
                //    |---|---|
                //    | `요약`·`경로`·`모집단`·`유효성`·`해악도` | `처분`·`조건`·`사전처분` |
                //
                // ⚠ **끝난 회차는 보고만 한다.** 그 라운드들은 추출기가 없던 때에
                //    손으로 전사됐다 — 하한 없이 걸면 옛 기록을 대량으로 고쳐야 하고
                //    그것은 「앞 회차의 판정을 다시 열지 마라」에 걸린다.
                //    **진행 중인 회차(종료 보고가 없는 회차)만 실패로 낸다.**
                let 진행중 = !기록이_확정됐나(&dir);
                let 행들 = 쌍_행.get(&(회차.clone(), 출처.to_string(), n));
                if let Some(행들) = 행들 {
                    let mut 갈림 = Vec::new();
                    match 추출기_산출(root, 파이썬, 출처, n, &f) {
                        Ok(뽑은) => {
                            let 프로필 = 뽑은
                                .first()
                                .and_then(|행| 행.get("_프로필"))
                                .and_then(|값| 값.as_str())
                                .unwrap_or("current");
                            let 역사_칸 = 문자열들(&반환형식["역사기계칸"][프로필][출처]);
                            let 대조_칸 = if 역사_칸.is_empty() { &기계_칸 } else { &역사_칸 };
                            let (칸수, 행수, 세부) = 원문순서_대조(&뽑은, 행들, 대조_칸);
                            갈린_칸 += 칸수;
                            빠진_행 += 행수;
                            갈림.extend(세부);
                        }
                        Err(e) => 갈림.push(format!("추출기가 못 돌았다: {e:#}")),
                    }
                    if !갈림.is_empty() {
                        if 진행중 {
                            진행중_손_전사 += 갈림.len();
                            problems.push(format!(
                                "{}: 기계 칸이 추출기 산출과 갈린다 ({}건) — **손으로 옮기지 \
                                 않는다.** `python3 {추출기} {출처} {n} <raw>` 로 뽑아라: {}",
                                상대_경로(root, &f),
                                갈림.len(),
                                갈림.iter().take(6).cloned().collect::<Vec<_>>().join(" · ")
                            ));
                        } else {
                            손_전사 += 갈림.len();
                        }
                    }
                }
                if 항 != 적힌 {
                    problems.push(format!(
                        "합계 검산 어긋남 — {}: 원 반환문의 항이 {항} 인데 회차 `{회차}` 의 \
                         `출처={출처}` · `라운드={n}` 레코드는 {적힌} 행이다 \
                         (**빠진 행 {}**)",
                        상대_경로(root, &f),
                        항.abs_diff(적힌)
                    ));
                }
            }
        }
    }
    // ⑤ **에이전트 출처인데 반환문이 없으면 실패다.** 보존을 빠뜨리면 검산이 조용히 사라진다.
    for ((회차, 출처, n), 행) in &쌍_수 {
        // ★★ **면제는 선언이 정한다** — 여기가 아니다(독립 리뷰 R2 · 발견 5).
        //   앞 판은 `면제출처` 를 **세기만** 했고 갈래는 하드코딩된 자리 표가 정했다.
        //   그래서 그 키를 비워도 검사가 초록이었다 — **소비자가 표시 전용이면
        //   그것은 소비자가 아니다.**
        if 면제_출처.iter().any(|s| s == 출처) {
            *면제.entry(출처.clone()).or_insert(0usize) += 행;
            continue;
        }
        let 에이전트 = 반환문_자리.iter().any(|(_, s)| s == 출처);
        if !에이전트 {
            problems.push(format!(
                "회차 `{회차}` 의 `출처={출처}` 가 **선언 어디에도 없다** — \
                 `합계검산.면제출처` 에도 없고 반환문 자리에도 없다. \
                 **조용히 면제되지 않는다**"
            ));
            continue;
        }
        let 자리 = 반환문_자리.iter().find(|(_, s)| s == 출처).map(|(d, _)| *d).unwrap();
        // ★ **그 회차 안에서만** 찾는다. 남의 회차 반환문이 내 레코드를 덮으면 안 된다.
        let 있나 = root
            .join(회차_뿌리)
            .join(회차)
            .join(자리)
            .join(format!("r{n}-raw.md"))
            .exists();
        if !있나 {
            problems.push(format!(
                "회차 `{회차}` 의 `출처={출처}` · `라운드={n}` 레코드가 {행} 행인데 원 반환문이 \
                 없다 — `{회차}/{자리}/r{n}-raw.md` 를 보존해야 합계 검산이 선다"
            ));
        }
    }

    if !problems.is_empty() {
        bail!("{}", problems.join("\n    "));
    }
    Ok(format!(
        "산출 {}개 · 레코드 {}행 · 예외표 {예외_행}행 · 좌표 면제 {좌표_면제}행 \
         (저장소 밖 절대경로) · **손으로 채운 칸 — 진행 중 {진행중_손_전사} · 끝난 회차 {손_전사}(보고만) \
         [갈린 칸 {갈린_칸} · 빠진 행 {빠진_행}]** · \
         검산 {} · 검산 면제 {} · \
         파이썬 `{파이썬}`",
        산출.len(),
        총_행 - 예외_행,
        if 검산.is_empty() { "없음".to_string() } else { 검산.join(" · ") },
        if 면제.is_empty() {
            "없음".to_string()
        } else {
            // ★ **몇 행이 면제됐는지 낸다.** 라벨만 내면 「11% 가 아무 대조도 안 받았다」는
            //   사실이 화면에 안 뜬다(독립 리뷰 2 라운드).
            format!(
                "{} (반환문이 원리상 없다)",
                면제.iter().map(|(k, v)| format!("{k} {v}행")).collect::<Vec<_>>().join("·")
            )
        }
    ))
}


// ─────────────────────────────────────────────────────────────────────────────
// 검사 22 — **발견이 닫혔나** (「했나」 축 · 2026-08-24)
// ─────────────────────────────────────────────────────────────────────────────
//
// # 무엇을 재나
//
// 회차 레코드의 `닫은커밋` 을 **읽는다.** 앞 판은 그 칸을 **한 번도 안 읽었다** —
// `grep -c '닫은커밋' xtask/src/main.rs` 가 착수 시점에 **0** 이었고, 그런 채로
// 닫힘 행 수백 개가 SHA 를 지고 있었으며 검사는 초록이었다.
//
// **검사가 「형식이 맞나」만 보고 「했나」를 안 봤다.**
//
// # 자
//
// | 처분 | 요구하는 자리 |
// |---|---|
// | `기각` | **없다** — 아무것도 안 고치는 처분이다. 「원리상 못 잼」으로 따로 센다 |
// | `축소`·`전환`·`범위밖` | 그 회차의 `intent.md` (개정·승격·`## 범위 밖` 이 거기 산다) |
// | 출처가 `사전부검` | 그 회차의 `intent.md` — §2 의 처분 넷이 전부 계획 문서를 만진다 |
// | `사전처분` 이 붙은 것 | 같음 |
// | 나머지 `정정`·`확대` | 그 발견의 `경로` |
//
// ★ **사전부검 발견의 `경로` 는 「예측된 파손 지점」이지 고침 지점이 아니다.**
//   그것을 고침 지점으로 읽으면 참인 답이 원리상 없는 행이 대량으로 빨개지고,
//   초록으로 만드는 유일한 길이 **존재하지 않는 커밋을 지어내는 것**이 된다 —
//   금지역 「사실이 아닌 것을 사실로」다(사전부검 R1).
//
// # 이 자가 **못 보는 것** — 게이트 `## 범위 밖` 과 같은 글이다
//
// 1. **만졌지만 안 고친 것.** diff 가 파일을 스쳐도 그 발견은 남아 있을 수 있다.
// 2. **`경로` 가 없거나 디렉터리인 행.** 좌표가 굵으면 자가 무뎌진다.
// 3. **여러 발견을 한 커밋이 닫은 경우.** 그중 하나만 고쳐도 전부 초록이다.
// 4. **다른 커밋이 실제로 고친 경우.** `닫은커밋` 이 틀렸는데 우연히 그 파일을 만졌으면 통과한다.
//
// **그러므로 이 축은 「거짓 닫힘을 없앤다」가 아니라 「가장 싼 거짓 닫힘을 못 하게 한다」다.**
//
// # ★ 「열림으로 밀기」를 원천 차단한다
//
// `상태=열림` 이 되면 그 행은 이 검사의 모집단에서 **빠져 즉시 초록**이 된다 —
// **가장 싼 경로가 「발견을 열어 두고 방치」**다(사전부검 R3). 그래서
// **끝난 회차(`report.md` 가 있는 회차)에 열림 행이 있으면 실패**다.
// 끝난 회차에 열린 발견이 있다는 것은 **그 회차가 안 끝난 것**이다.
//
// # 성능 — SHA 단위로 캐시한다
//
// 행마다 `git show` 를 부르면 macOS 에서 10 초가 붙고 Windows 는 그 배수다(사전부검 R2).
// 서로 다른 SHA 는 수십 개뿐이라 **캐시가 20 배를 돌려준다.**

/// `record.py --schema` 를 읽는다 — **선언의 유일한 자리**(C2-b).
///
/// ★★ **읽는 것이 곧 정본으로 만드는 것이다.** (#94 · 2026-08-24)
///
/// 앞 판은 이 사전의 대부분을 **산문으로만** 두었고 `xtask` 는 `종류` 하나만 읽었다.
/// 그래서 선언과 코드가 갈려도 **아무것도 안 울었고**, 세 라운드가 연달아 같은 자리를
/// 냈다 — 매번 키를 더해도 다음 라운드가 또 잡았다. **소비자가 0 인 선언은 갈렸다는
/// 것을 원리상 못 잡는다.**
///
/// 이제 갈래마다 여기서 읽는다. **여기를 고치면 검사 동작이 바뀐다** — 그것이 이
/// 선언이 정본이라는 증인이다. 안 읽는 키는 `설명` 아래로 내려가 **「정본 아님」이
/// 선언**돼 있고, 갈려도 거짓이 아니다.
fn 스키마를_읽는다(root: &Path) -> Result<serde_json::Value> {
    let 파이썬 = 파이썬_실행자()?;
    let out = 파이썬_명령(파이썬)
        .arg(root.join(스키마_원천))
        .arg("--schema")
        .output()
        .with_context(|| format!("`{파이썬} {스키마_원천} --schema` 를 못 돌렸다"))?;
    if !out.status.success() {
        bail!(
            "`{스키마_원천} --schema` 가 실패했다:\n{}",
            String::from_utf8_lossy(&out.stderr)
        );
    }
    serde_json::from_slice(&out.stdout).context("`--schema` 출력이 JSON 이 아니다")
}

/// `docs/gates/README.md` 의 **닫힌 선언 목록** 하나를 읽는다 — 항목과 날짜 하한.
///
/// ★★ **왜 목록이 「손으로 벤 거울」이 아닌가.** 앞 회차가 실측한 병은 *"자라는
/// 모집단을 손으로 베껴 놓으면 놓은 날부터 갈린다"* 였다. **이 목록은 닫혀 있다** —
/// 날짜 하한 뒤에 연 회차는 못 들어가고, [`check_declared_lists`] 가 그것을 강제한다.
/// 거울은 자라고 예외 선언은 닫힌다. **그것이 가르는 문장이다.**
///
/// ⚠ **면제가 아니라 빚이다.** 부르는 자리가 **수를 판정문에 실어야** 한다 —
/// 안 실으면 그것이 조용한 fallback 이고, 이 회차가 닫으려는 병 그 자체다.
fn 선언_목록(
    root: &Path,
    제목: &str,
    빈항목_허용: bool,
) -> Result<(Vec<String>, String)> {
    let p = root.join("docs/gates/README.md");
    let text = std::fs::read_to_string(&p)
        .with_context(|| format!("선언 목록을 못 읽었다: {}", p.display()))?;
    let mut 안 = false;
    let mut 항목 = Vec::new();
    let mut 하한 = String::new();
    for line in text.lines() {
        if let Some(h) = line.strip_prefix("### ") {
            안 = h.starts_with(제목);
            continue;
        }
        if line.starts_with("## ") {
            안 = false;
        }
        if !안 {
            continue;
        }
        if let Some(rest) = line.trim().strip_prefix("- `") {
            if let Some(name) = rest.split('`').next() {
                항목.push(name.to_string());
            }
        }
        if let Some(i) = line.find("**하한: ") {
            하한 = line[i + "**하한: ".len()..]
                .split_whitespace()
                .next()
                .unwrap_or("")
                .to_string();
        }
    }
    if 하한.is_empty() || (!빈항목_허용 && 항목.is_empty()) {
        bail!(
            "`docs/gates/README.md` 의 `### {제목}` 목록에 날짜 하한이 없거나 빈 목록을 \
             허용하지 않는다 — 예외 목록의 0 건과 검사 모집단의 0 건은 다르다"
        );
    }
    Ok((항목, 하한))
}

#[cfg(test)]
mod 종료와_선언_목록_시험 {
    use super::{기록이_확정됐나, 선언_목록, 종료보고_형식이전_회차, 종료했나};
    use std::fs;

    fn 임시_뿌리(이름: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!("pal-xtask-{이름}-{}", std::process::id()))
    }

    #[test]
    fn 형식이전_종료는_정확한_회차와_표지만_받는다() {
        let 뿌리 = 임시_뿌리("legacy-close");
        let 회차 = 뿌리.join(종료보고_형식이전_회차);
        fs::create_dir_all(&회차).unwrap();
        fs::write(
            회차.join("state.md"),
            "# 상태\n\n**단계**: 종료. 완수 조건 전부 닫힘 · 효과 관측 · CI 초록.\n",
        )
        .unwrap();
        assert!(종료했나(&회차));
        assert!(기록이_확정됐나(&회차));

        let 다른_회차 = 뿌리.join("2026-08-18-not-the-legacy-round");
        fs::create_dir_all(&다른_회차).unwrap();
        fs::copy(회차.join("state.md"), 다른_회차.join("state.md")).unwrap();
        assert!(!종료했나(&다른_회차));

        fs::write(회차.join("state.md"), "**단계**: 종료.\n").unwrap();
        assert!(!종료했나(&회차));
        let _ = fs::remove_dir_all(&뿌리);
    }

    #[test]
    fn 빚_목록은_비어도_하한은_필수다() {
        let 뿌리 = 임시_뿌리("empty-declaration");
        // 실행 시점 docs 모집단은 소스의 `join("docs/…")` 리터럴에서 만들어진다.
        // 시험용 임시 경로가 제품 모집단을 넓히지 않도록 두 조각으로 만든다.
        fs::create_dir_all(뿌리.join("docs").join("gates")).unwrap();
        fs::write(
            뿌리.join("docs").join("gates/README.md"),
            "### 빚\n\n현재 항목 없음.\n\n**하한: 2026-08-24 이전에 연 회차만.**\n",
        )
        .unwrap();
        assert_eq!(선언_목록(&뿌리, "빚", true).unwrap().0.len(), 0);
        assert!(선언_목록(&뿌리, "빚", false).is_err());

        fs::write(
            뿌리.join("docs").join("gates/README.md"),
            "### 빚\n\n현재 항목 없음.\n",
        )
        .unwrap();
        assert!(선언_목록(&뿌리, "빚", true).is_err());
        let _ = fs::remove_dir_all(&뿌리);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 검사 23 — **선언 목록이 닫혀 있나** (`C5` · 2026-08-24)
// ─────────────────────────────────────────────────────────────────────────────
//
// ★ **목록을 쓰는 자리에서 검증하면 무발화한다.** 새 회차는 표준 표를 갖고 끝나므로
//   「표준 표가 없을 때만 목록을 읽는」 자연스러운 구현에서는 **그 회차를 목록에 넣어도
//   분기에 도달하지 않는다**(사전부검 R2). 그래서 **목록 자체를 독립으로 검증한다.**
fn check_declared_lists(root: &Path) -> Result<String> {
    let mut problems = Vec::new();
    let mut 셈 = Vec::new();
    for 제목 in [
        "형식 이전",
        "A 축 감사 대기",
        "종료 보고 검산 줄 유예",
        "종료 보고 없음 유예",
    ] {
        let 빈항목_허용 = 제목 != "형식 이전";
        let (항목, 하한) = 선언_목록(root, 제목, 빈항목_허용)?;
        for name in &항목 {
            if !root.join(회차_뿌리).join(name).is_dir() {
                problems.push(format!(
                    "`{제목}` 목록의 `{name}` 은 실재하는 회차가 아니다"
                ));
            }
            // ★ **날짜 하한.** 회차 이름이 `YYYY-MM-DD-…` 이므로 문자열 비교로 족하다.
            if name.as_str() >= 하한.as_str() {
                problems.push(format!(
                    "`{제목}` 목록의 `{name}` 이 하한 `{하한}` 이후다 — \
                     **이 목록은 닫혀 있다.** 새 회차는 못 들어간다"
                ));
            }
        }
        셈.push(format!("{제목} {}개 (하한 {하한})", 항목.len()));
    }
    if !problems.is_empty() {
        bail!("{}", problems.join("\n    "));
    }
    Ok(셈.join(" · "))
}

/// **이 저장소의 파이썬 호출은 여기 하나로 모인다.**
///
/// ★★ **`PYTHONUTF8=1` 을 못 박는다.** 안 주면 Windows 가 로케일 인코딩(cp949·cp1252)
/// 으로 표준 입출력과 파일을 읽어 한글이 `UnicodeDecodeError` 를 낸다 —
/// **macOS 에서는 원리상 안 보이는 자리**다. CI 가 실측으로 잡았다(2026-08-24):
/// 이 회차가 새로 쓴 추출기가 windows-latest 에서만 죽었고, 로컬은 내내 초록이었다.
///
/// ADR-0023 — *"플랫폼 분기는 한 자리에 산다."* `깃()` 과 같은 규율이다.
fn 파이썬_명령(파이썬_경로: &str) -> Command {
    let mut c = Command::new(파이썬_경로);
    c.env("PYTHONUTF8", "1").env("PYTHONIOENCODING", "utf-8");
    c
}

/// **이 저장소의 git 호출은 여기 하나로 모인다.**
///
/// ★ ADR-0023 — *"플랫폼 분기는 한 자리에 산다."* `gix 격리` 검사가 `xtask` 의 `gix`
/// 사용을 막으므로 CLI 셸아웃뿐이고, 그러면 **자리를 하나로 모으는 것**이 그 규율을
/// 지키는 유일한 길이다.
///
/// ⚠ 앞 판은 축을 하나 더하면서 **둘째 git 함수를 만들었다** — 그 자리는 캐시도 안
/// 쓰고 실패도 안 알렸다(독립 리뷰 R2 · 발견 6). **더할 때마다 여기로 모은다.**
fn 깃(root: &Path, args: &[&str]) -> Option<String> {
    let out = Command::new("git").current_dir(root).args(args).output().ok()?;
    if !out.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&out.stdout).to_string())
}

/// `닫은커밋` 이 만진 파일 집합. `None` 이면 **이 이력에서 안 보인다**(없는 커밋이 아니다).
///
/// ⚠ **「없는 커밋」과 「이력 밖 커밋」을 기계가 못 가른다.** 얕은 클론이나 squash 병합
/// 뒤에는 실재하는 커밋도 안 읽힌다. 그래서 판정문이 **「이 이력에서 안 보인다」**로
/// 적는다 — 「그런 커밋이 없다」로 적으면 사실이 아닌 것을 사실로 적는 것이다.
fn 커밋이_만진_것(
    root: &Path,
    sha: &str,
    캐시: &mut BTreeMap<String, Option<Vec<String>>>,
) -> Option<Vec<String>> {
    if let Some(v) = 캐시.get(sha) {
        return v.clone();
    }
    let v = 깃(root, &["show", "--name-only", "--format=", "--no-renames", sha]).map(|s| {
        s.lines()
            .map(|l| l.trim().replace('\\', "/"))
            .filter(|l| !l.is_empty())
            .collect::<Vec<_>>()
    });
    캐시.insert(sha.to_string(), v.clone());
    v
}

/// 지금 **열림**인 행 중, **이력의 어느 시점에 닫힘이었던** 행의 수.
///
/// ★★ 「다시 연다」가 도피로가 되는 것을 막는 **둘째 자**다. 끝난 회차는
/// 「열림이면 실패」가 막지만, **진행 중인 회차에서 닫힘을 열림으로 되돌리는 것**은
/// 그 자가 못 본다 — 그러면 그 행이 모집단에서 빠져 즉시 초록이다.
///
/// ⚠⚠ **기준을 한 커밋으로 잡으면 새는 길이 남는다.** (독립 리뷰 R2 · 발견 2·3)
/// ⓐ `main` 갈림점은 신선한 clone·CI 에 지역 ref 가 **없어** 조용히 0 을 내고,
///    **진행 중인 회차의 파일은 갈림점에 아예 없어** 건너뛴다.
/// ⓑ 「파일이 처음 들어온 커밋」은 그 뒤에 **닫힘으로 들어온 행**이 뒤집히는 것을
///    못 본다 — 실측으로 확인했다(셋을 뒤집었는데 둘만 잡혔다).
///
/// **그래서 그 파일의 이력 전부를 본다.** 한 번이라도 닫힘이었으면 잡는다.
///
/// ⚠ **못 재면 실패다.** 0 과 「못 쟀다」를 같은 글자로 내지 않는다.
///
/// ★ 열림 행이 **하나도 없는 파일은 이력을 안 훑는다** — 잴 것이 없다.
fn 닫힘에서_열림으로(
    root: &Path,
    산출: &[PathBuf],
    상태칸: &str,
    닫힘값: &str,
    열림값: &str,
) -> Result<usize> {
    let mut n = 0usize;
    for p in 산출 {
        if p.extension().and_then(|x| x.to_str()) != Some("jsonl") {
            continue;
        }
        let 본문 = std::fs::read_to_string(p)?;
        let 지금_열림: Vec<String> = 본문
            .lines()
            .filter_map(|l| serde_json::from_str::<serde_json::Value>(l).ok())
            .filter(|v| v[상태칸].as_str() == Some(열림값))
            .filter_map(|v| v["id"].as_str().map(str::to_string))
            .collect();
        if 지금_열림.is_empty() {
            continue;
        }
        let 상대 = 상대_경로(root, p);
        let Some(이력) = 깃(root, &["log", "--format=%H", "--", &상대]) else {
            bail!("{상대}: 이력을 못 읽는다 — **0 과 「못 쟀다」를 같은 글자로 안 낸다**");
        };
        let mut 닫힌_적_있다: std::collections::BTreeSet<String> = Default::default();
        for sha in 이력.lines().filter(|s| !s.trim().is_empty()) {
            let Some(옛) = 깃(root, &["show", &format!("{sha}:{상대}")]) else {
                continue;
            };
            for v in 옛.lines().filter_map(|l| serde_json::from_str::<serde_json::Value>(l).ok()) {
                if v[상태칸].as_str() == Some(닫힘값) {
                    if let Some(id) = v["id"].as_str() {
                        닫힌_적_있다.insert(id.to_string());
                    }
                }
            }
        }
        n += 지금_열림.iter().filter(|id| 닫힌_적_있다.contains(*id)).count();
    }
    Ok(n)
}

/// 그 처분이 **요구하는 자리**. `Err(사유)` 면 원리상 못 재는 행이다.
///
/// ★★ **규칙은 `record.py --schema` 의 `닫힘축` 이 진다** — 여기가 아니다(#94).
///   그 사전을 고치면 이 함수의 답이 바뀐다. 그것이 선언이 정본이라는 증인이다.
///
/// **순수 함수다** — 그래야 음성 대조를 시험으로 세울 수 있다.
fn 요구하는_자리(
    닫힘축: &serde_json::Value,
    회차: &str,
    출처: &str,
    처분: &str,
    사전처분: &str,
    경로: &str,
) -> Result<String, &'static str> {
    // ★★ **기각을 맨 먼저 본다.** (독립 리뷰 R1 · 발견 6)
    //   앞 판은 출처·사전처분 갈래가 먼저라, **기각인데 사전부검 출처인 59 행**이
    //   「잰 것」 쪽으로 새어 나갔다. 판정문은 「기각 141」만 보이므로 읽는 사람은
    //   **기각 전량이 빠진 줄로 믿는다** — ㉑ 이 막으려던 「모집단이 큰데 술어가
    //   항등」의 형태다. **아무것도 안 고치는 처분은 무엇이 요구하든 못 잰다.**
    if 닫힘축["요구하는자리"]
        .get(처분)
        .map(serde_json::Value::is_null)
        .unwrap_or(false)
    {
        return Err("기각");
    }
    let 의도 = format!(
        "{회차_뿌리}/{회차}/{}",
        닫힘축["의도파일"].as_str().unwrap_or("intent.md")
    );
    // ① 이 출처는 `경로` 가 **예측된 파손 지점**이라 계획 문서를 요구한다.
    let 의도_출처 = 닫힘축["의도를요구하는출처"]
        .as_array()
        .map(|a| a.iter().any(|x| x.as_str() == Some(출처)))
        .unwrap_or(false);
    // ② `사전처분` 이 붙었으면 §2 의 처분이라 계획 문서를 요구한다.
    let 사전처분_있음 = 사전처분 != 닫힘축["사전처분없음"].as_str().unwrap_or("해당없음");
    if 의도_출처 || 사전처분_있음 {
        return Ok(의도);
    }
    // ③ 처분별 사전. `null` 이면 원리상 못 잰다 — `기각` 이 그것이다.
    match 닫힘축["요구하는자리"].get(처분) {
        None => Err("선언 밖 처분"),
        Some(serde_json::Value::Null) => Err("기각"),
        Some(v) if v.as_str() == Some("의도") => Ok(의도),
        Some(_) => {
            if 경로 == 닫힘축["경로없음"].as_str().unwrap_or("(경로 없음)") {
                Err("경로 없음")
            } else if 저장소_밖_절대경로(경로) {
                Err("저장소 밖")
            } else {
                Ok(경로.to_string())
            }
        }
    }
}

#[cfg(test)]
mod 요구하는_자리_시험 {
    use super::요구하는_자리;

    fn 축() -> serde_json::Value {
        serde_json::json!({
            "요구하는자리": {"기각": null, "축소": "의도", "전환": "의도",
                             "범위밖": "의도", "정정": "경로", "확대": "경로"},
            "의도를요구하는출처": ["사전부검"],
            "사전처분없음": "해당없음",
            "의도파일": "intent.md",
            "경로없음": "(경로 없음)"
        })
    }

    /// ★ **`기각` 은 아무것도 안 고치는 처분이라 요구하는 자리가 원리상 없다.**
    /// 항상 참인 술어를 씌워 모집단만 채우면 판정문이 「N 행을 쟀다」로 보인다 —
    /// 그것이 「측정이 죽은 가지」다(사전부검 R2 · 실측 기각 200 행).
    #[test]
    fn 기각은_원리상_못_잰다() {
        assert!(요구하는_자리(&축(), "r", "독립리뷰", "기각", "해당없음", "a.rs").is_err());
    }

    /// ★★ **기각은 출처가 무엇이든 못 잰다** — 아무것도 안 고치는 처분이다.
    /// 실측: 앞 판은 출처 갈래가 먼저라 **기각인데 사전부검 출처인 59 행**이
    /// 「잰 것」 쪽으로 새어 나갔다(독립 리뷰 R1).
    #[test]
    fn 기각은_출처가_무엇이든_못_잰다() {
        assert!(요구하는_자리(&축(), "r", "사전부검", "기각", "해당없음", "a.rs").is_err());
        assert!(요구하는_자리(&축(), "r", "독립리뷰", "기각", "계획수정", "a.rs").is_err());
    }

    /// 사전부검 발견의 `경로` 는 **예측된 파손 지점**이지 고침 지점이 아니다.
    #[test]
    fn 사전부검은_계획_문서를_요구한다() {
        let 자 = 요구하는_자리(&축(), "r1", "사전부검", "정정", "해당없음", "xtask/src/main.rs").unwrap();
        assert!(자.ends_with("r1/intent.md"), "{자}");
    }

    #[test]
    fn 리뷰_정정은_좌표를_요구한다() {
        let 자 = 요구하는_자리(&축(), "r1", "독립리뷰", "정정", "해당없음", "xtask/src/main.rs").unwrap();
        assert_eq!(자, "xtask/src/main.rs");
    }

    #[test]
    fn 좌표가_없거나_밖이면_못_잰다() {
        assert!(요구하는_자리(&축(), "r", "독립리뷰", "정정", "해당없음", "(경로 없음)").is_err());
        assert!(요구하는_자리(&축(), "r", "독립리뷰", "정정", "해당없음", "/tmp").is_err());
    }

    #[test]
    fn 범위밖은_계획_문서를_요구한다() {
        for ch in ["축소", "전환", "범위밖"] {
            let 자 = 요구하는_자리(&축(), "r2", "독립리뷰", ch, "해당없음", "a.rs").unwrap();
            assert!(자.ends_with("r2/intent.md"), "{ch}: {자}");
        }
    }

    /// ★★ **선언을 고치면 답이 바뀐다** — 그것이 `E3` 의 증인이다.
    #[test]
    fn 선언을_고치면_답이_바뀐다() {
        let mut 축 = 축();
        assert!(요구하는_자리(&축, "r", "독립리뷰", "정정", "해당없음", "a.rs").is_ok());
        축["요구하는자리"]["정정"] = serde_json::Value::Null;
        assert!(요구하는_자리(&축, "r", "독립리뷰", "정정", "해당없음", "a.rs").is_err());
    }

    /// 선언 밖 처분은 **조용히 통과시키지 않는다.**
    #[test]
    fn 선언_밖_처분은_못_잰다() {
        assert!(요구하는_자리(&축(), "r", "독립리뷰", "새처분", "해당없음", "a.rs").is_err());
    }
}

fn check_finding_closure(root: &Path) -> Result<String> {
    let 산출 = 회차_산출(root)?;
    // ★ **하한은 닫힌 선언 목록이다.** 이 축이 2026-08-24 에 처음 섰고, 그 전에 닫힌
    //   행들은 **그 자를 모르는 채로 쓰였다.** ⚠ **면제가 아니라 빚이다** — 그 회차들의
    //   발화도 **세고 판정문에 낸다.** 조용히 안 재면 그것이 이 회차가 닫으려는 병이다.
    let (감사_대기, _) = 선언_목록(root, "A 축 감사 대기", true)?;
    // ★ 규칙은 선언이 진다 — 여기가 아니다(#94).
    let 스키마 = 스키마를_읽는다(root)?;
    let 닫힘축 = &스키마["닫힘축"];
    let 열림축 = &스키마["열림축"];
    let 열림값 = 열림축["열림값"].as_str().unwrap_or("열림");
    let 닫힘값 = 열림축["닫힘값"].as_str().unwrap_or("닫힘");
    // ★ `부터` — 이 버전 **미만**의 파일에는 열림 축이 원리상 없다(형식 이전).
    let 열림축_부터 = 열림축["부터"].as_i64().unwrap_or(3);
    let 상태칸 = 열림축["필드"][0].as_str().unwrap_or("상태").to_string();
    let 커밋칸 = 열림축["필드"][1].as_str().unwrap_or("닫은커밋").to_string();
    if 닫힘축["요구하는자리"].as_object().map_or(true, |m| m.is_empty()) {
        bail!("`--schema` 의 `닫힘축.요구하는자리` 가 비었다 — 이 검사의 규칙이 원리상 안 선다");
    }
    let mut 대기_발화 = 0usize;
    let mut problems = Vec::new();
    let mut 캐시: BTreeMap<String, Option<Vec<String>>> = Default::default();
    let (mut 잰_것, mut 안_보임, mut 열림) = (0usize, 0usize, 0usize);
    // ★ 끝난 회차의 열림은 **즉시 실패**라 구조상 0 이다 — 그 0 이 화면에 떠야
    //   「그 자가 실제로 돌았다」를 사람이 안다(독립 리뷰 R2 · 발견 1).
    let mut 끝난_회차_열림 = 0usize;
    // 접힌 회차의 열린 행 — **실패로 안 낸다. 세어서 보고만 한다**(아래 「접힌 회차는 여기서 빠진다」).
    let mut 접힌_회차_열림 = 0usize;
    let mut 못_잼: BTreeMap<&'static str, usize> = Default::default();
    let mut 발화 = Vec::new();

    for p in &산출 {
        if p.extension().and_then(|x| x.to_str()) != Some("jsonl") {
            continue;
        }
        let 상대 = 상대_경로(root, p);
        let text = std::fs::read_to_string(p)?;
        let 종류 = serde_json::from_str::<serde_json::Value>(text.lines().next().unwrap_or(""))
            .ok()
            .and_then(|v| v.get("종류").and_then(|x| x.as_str()).map(str::to_owned))
            .unwrap_or_else(|| "레코드".into());
        if 종류 != "레코드" {
            continue;
        }
        let 회차 = 회차_이름(root, p);
        let 끝났나 = 기록이_확정됐나(&root.join(회차_뿌리).join(&회차));
        let 종료보고를_썼나 = root.join(회차_뿌리).join(&회차).join("report.md").is_file();
        // ★ **버전이 낮으면 이 축이 원리상 없다** — 선언이 그 경계를 진다.
        let 버전 = serde_json::from_str::<serde_json::Value>(text.lines().next().unwrap_or(""))
            .ok()
            .and_then(|v| v["schema_version"].as_i64())
            .unwrap_or(0);
        let 축이_있나 = 버전 >= 열림축_부터;
        for (i, line) in text.lines().enumerate() {
            if line.trim().is_empty() || i == 0 {
                continue;
            }
            let Ok(v) = serde_json::from_str::<serde_json::Value>(line) else { continue };
            if v.get("id").is_none() {
                continue;
            }
            // ★ 스키마 2 회차는 이 축이 **원리상 없다** — 형식 이전이지 0 이 아니다.
            //   경계는 `--schema` 의 `열림축.부터` 가 진다.
            if !축이_있나 {
                *못_잼.entry("형식 이전").or_default() += 1;
                continue;
            }
            let Some(상태) = v.get(&상태칸).and_then(|x| x.as_str()) else {
                problems.push(format!(
                    "{상대}:{}: 스키마 {버전} 인데 `{상태칸}` 이 없다 — \
                     `열림축.부터` 가 {열림축_부터} 이므로 이 파일은 그 축을 져야 한다",
                    i + 1
                ));
                continue;
            };
            let id = v.get("id").and_then(|x| x.as_str()).unwrap_or("?");
            if 상태 != 닫힘값 && 상태 != 열림값 {
                problems.push(format!(
                    "{상대}:{}: `{상태칸}` 이 `{상태}` 다 — 선언은 `{닫힘값}`·`{열림값}` 둘뿐이다",
                    i + 1
                ));
                continue;
            }
            if 상태 == 열림값 {
                열림 += 1;
                // ★★ **접힌 회차는 여기서 빠진다.** (정정 2026-08-24)
                //
                //   접힘은 §11 을 안 지난다 — 남은 검증 라운드·게이트·종료 보고를
                //   치르지 않는다(규약 §5 「접힘」). 그런데 앞 판은 접힌 회차에
                //   **§11 보다 엄한 것**(발견 전량 닫힘)을 요구했다.
                //   빠져나가는 길이 **전부를 `기각` 으로 개칭하는 것**뿐이라,
                //   만든 유인이 **「접을지 모르는 회차에서는 레코드를 아예 안 쓴다」**였다.
                //   레코드를 죽이는 검사는 그 자체가 「측정이 죽은 가지」다.
                //
                //   ⚠ 대신 **판정문이 세어서 보고한다** — 조용히 사라지지 않는다.
                if 종료보고를_썼나 {
                    끝난_회차_열림 += 1;
                    problems.push(format!(
                        "{상대}:{}: `{id}` 가 **열림**인데 그 회차는 **종료**했다(`report.md` 가 있다) — \
                         끝난 회차에 열린 발견이 있으면 **그 회차가 안 끝난 것**이다. \
                         「열림으로 밀기」로 이 검사를 비껴가지 않는다",
                        i + 1
                    ));
                } else if 끝났나 {
                    접힌_회차_열림 += 1;
                }
                continue;
            }
            let Some(sha) = v.get(&커밋칸).and_then(|x| x.as_str()) else { continue };
            let Some(만진) = 커밋이_만진_것(root, sha, &mut 캐시) else {
                // ★★ **안 보이면 실패다 — 조용히 안 세고 넘어가지 않는다.**
                //   음성 대조가 잡았다: 파일 복사본(git 이력 없음)에서 이 축이
                //   **616 행을 전부 「안 보임」으로 세고도 초록**이었다 —
                //   「측정이 죽은 가지」 그 자체다.
                // ⚠ **까닭 둘을 기계가 못 가른다** — 「그런 커밋이 없다」와 「이 이력
                //   창 밖이다」. 그래서 문면이 **「이 이력에서 안 보인다」**로 적는다.
                안_보임 += 1;
                problems.push(format!(
                    "{상대}:{}: `{id}` 의 `닫은커밋` `{sha}` 가 **이 이력에서 안 보인다** — \
                     없는 커밋이거나, 얕은 클론·squash 병합으로 이력 밖이다. \
                     CI 라면 체크아웃 깊이를 보라",
                    i + 1
                ));
                continue;
            };
            let 자 = 요구하는_자리(
                닫힘축,
                &회차,
                v.get("출처").and_then(|x| x.as_str()).unwrap_or(""),
                v.get("처분").and_then(|x| x.as_str()).unwrap_or(""),
                v.get("사전처분").and_then(|x| x.as_str()).unwrap_or("해당없음"),
                v.get("경로").and_then(|x| x.as_str()).unwrap_or("(경로 없음)"),
            );
            let 자 = match 자 {
                Ok(자) => 자,
                Err(why) => {
                    *못_잼.entry(why).or_default() += 1;
                    continue;
                }
            };
            잰_것 += 1;
            // ★ **디렉터리 좌표는 면제가 아니라 접두 일치로 잰다.** 면제하면 자가
            //   무뎌지고, 굵은 좌표를 적는 것이 검사를 비껴가는 길이 된다.
            // ⚠ **끝 슬래시를 먼저 벗긴다.** 안 벗기면 접두가 `<자>//` 가 되어 절대 안
            //   맞고, 그러면 **디렉터리 좌표가 전부 거짓 발화**한다(실측으로 잡았다).
            let 자 = 자.trim_end_matches('/').to_string();
            let 디렉터리 = root.join(&자).is_dir();
            let 닿았나 = 만진.iter().any(|f| {
                f == &자
                    || f.ends_with(&format!("/{자}"))
                    || (디렉터리 && (f.starts_with(&format!("{자}/")) || f.contains(&format!("/{자}/"))))
            });
            if !닿았나 {
                if 감사_대기.iter().any(|x| x == &회차) {
                    대기_발화 += 1;
                } else {
                    발화.push(format!(
                        "{상대}:{}: `{id}` 가 `{sha}` 로 닫혔다는데 그 커밋이 `{자}` 를 안 만졌다",
                        i + 1
                    ));
                }
            }
        }
    }

    // ── A6 — **닫힘→열림 전환 수** ────────────────────────────────────────
    //
    // ★★ 「다시 연다」가 도피로가 되는 것을 막는 둘째 자다. `상태=열림` 이 되면
    //   그 행은 이 검사의 모집단에서 **빠져 즉시 초록**이 된다. 끝난 회차는
    //   위에서 실패로 막지만, **진행 중인 회차에서 닫힘을 열림으로 되돌리는 것**은
    //   그 자가 못 본다. 그래서 **이력과 대어 전환 수를 낸다.**
    //
    //   ⚠ 이것은 방어가 아니라 **계기**다 — 되돌리는 것이 정당할 때도 있다.
    //   숨지 않게만 한다.
    let 전환 = 닫힘에서_열림으로(root, &산출, &상태칸, 닫힘값, 열림값)?;

    // ★★★ **모집단이 비면 실패다.** (독립 리뷰 R2 · 발견 7 · 금지역)
    //
    //   앞 판은 이 가드가 **없었다.** 그래서 선언 한 자리를 고쳐 모집단을 0 으로
    //   비워도 **23/23 초록**이었다. **회차 레코드 검사와 원장 둘 대조에는 그 가드가
    //   있는데 이 축만 없었다** — 0 건은 「안 부른다」가 아니라 **「안 봤다」**다.
    //
    // ⚠⚠ **그러나 그것으로 `bail!` 하면 안 된다.** (독립 리뷰 R3 · 발견 1 · 금지역)
    //   얕은 클론·squash 뒤에는 SHA 가 하나도 안 풀려 **잰 것이 0 이 되는데**,
    //   그때 이 가드가 먼저 나가면 **「이 이력에서 안 보인다」 진단 수백 개가
    //   통째로 사라진다.** 남는 문장은 「아무것도 안 잰다」인데 **그것은 사실이
    //   아니다** — 행은 봤고 커밋을 못 푼 것이다. **까닭을 지우는 진단은
    //   사실이 아닌 것을 사실로 적는 것**이다.
    //
    //   그래서 **`problems` 에 넣는다** — 다른 진단과 **함께** 난다.
    if 잰_것 == 0 {
        problems.push(format!(
            "**잰 것이 0 행이다** — 이 축이 아무것도 안 잰다. 0 건은 「안 부른다」가 \
             아니라 「안 봤다」다. (원리상 못 잼 {} · 이 이력에서 안 보임 {안_보임} · 열림 {열림})",
            못_잼.values().sum::<usize>()
        ));
    }

    let 못_잼_합: usize = 못_잼.values().sum();
    let 못_잼_글 = if 못_잼.is_empty() {
        "0".to_string()
    } else {
        format!(
            "{못_잼_합} ({})",
            못_잼.iter().map(|(k, v)| format!("{k} {v}")).collect::<Vec<_>>().join(" · ")
        )
    };
    let 발화_수 = 발화.len();
    problems.extend(발화);
    if !problems.is_empty() {
        bail!(
            "{}\n\n    잰 것 {잰_것} · 발화 {발화_수} · 원리상 못 잼 {못_잼_글} · \
             이 이력에서 안 보임 {안_보임} · 열림 {열림}",
            problems.join("\n    ")
        );
    }
    Ok(format!(
        "잰 것 {잰_것}행 · 발화 {발화_수} · **감사 대기 발화 {대기_발화}** · \
         원리상 못 잼 {못_잼_글} · 이 이력에서 안 보임 {안_보임}행 · \
         열림 {열림}행 (끝난 회차 {끝난_회차_열림} · **접힌 회차 {접힌_회차_열림}** · **닫힘→열림 전환 {전환}**) · SHA {}개",
        캐시.len()
    ))
}


/// 저장소 **밖**을 가리키는 절대경로인가 — **플랫폼과 무관하게 문자열로 판정한다.**
///
/// ★ 왜 `Path::is_absolute` 를 안 쓰나: 그 자가 **플랫폼마다 다르다.** `/tmp` 는
/// 유닉스에서 절대이고 Windows 에서는 아니다. 그러면 같은 레코드가 OS 마다 다른
/// 판정을 받고, 그것이 ADR-0023 이 금지한 자리다. **여기서는 세 OS 가 같은 답을 낸다.**
fn 저장소_밖_절대경로(경로: &str) -> bool {
    let b = 경로.as_bytes();
    경로.starts_with('/')
        || 경로.starts_with('~')
        || 경로.starts_with("\\\\")
        || (b.len() > 2 && b[1] == b':' && (b[2] == b'/' || b[2] == b'\\'))
}

#[cfg(test)]
mod 좌표_면제_시험 {
    use super::저장소_밖_절대경로;

    /// ★ **세 OS 가 같은 답을 내야 한다.** 이 자는 파일시스템을 안 만지고 문자열만
    /// 본다 — `Path::is_absolute` 는 플랫폼마다 다르므로 안 쓴다(ADR-0023).
    #[test]
    fn 저장소_밖_절대경로를_가른다() {
        // 유닉스 절대경로 — **이것이 windows-latest 만 죽인 그 행이다.**
        assert!(저장소_밖_절대경로("/tmp"));
        assert!(저장소_밖_절대경로("/var/folders/x/y"));
        assert!(저장소_밖_절대경로("~/dev/projects/ditto"));
        // 윈도 드라이브·UNC
        assert!(저장소_밖_절대경로("C:/Users/x"));
        assert!(저장소_밖_절대경로("C:\\Users\\x"));
        assert!(저장소_밖_절대경로("\\\\server\\share"));
        // 저장소 안의 좌표는 면제가 아니다 — 면제가 넓으면 검사가 아무것도 안 잰다
        assert!(!저장소_밖_절대경로("xtask/src/main.rs"));
        assert!(!저장소_밖_절대경로(".claude/skills/round/SKILL.md"));
        assert!(!저장소_밖_절대경로("docs/gates/README.md"));
        assert!(!저장소_밖_절대경로("layout.rs"));
        // ⚠ **한 글자짜리 조각을 드라이브로 오인하지 않는다.**
        assert!(!저장소_밖_절대경로("a:b"));
    }
}

/// 좌표가 실재하는가 — 경로째로 있거나, **끝이 맞는 파일이 저장소에 있거나.**
///
/// ⚠ **접미 매칭을 여는 까닭**: 발견을 내는 자(에이전트)가 `layout.rs:161` 처럼 **파일
/// 이름만** 적거나 `retro/09-categories.md` 처럼 **회차 안에서의 상대 경로**로 적는 일이
/// 흔하다(실측: 좌표 59 개 중 그런 것이 17). 그것을 실패로 치면 검사가 실질을 안 재고
/// **형식만** 재게 된다.
///
/// ★ 이 검사가 재는 것은 **「그 파일이 이 저장소에 있는가」**이지 「인용이 정확한가」가
/// 아니다. 줄 번호는 **안 잰다** — 회차가 자기 좌표를 밀어내기 때문이다.
fn 좌표가_실재하는가(root: &Path, 경로: &str) -> bool {
    if root.join(경로).exists() {
        return true;
    }
    fn 찾는다(dir: &Path, 끝: &str, 깊이: usize) -> bool {
        if 깊이 == 0 {
            return false;
        }
        let Ok(읽기) = std::fs::read_dir(dir) else {
            return false;
        };
        for e in 읽기.flatten() {
            let p = e.path();
            let n = p.file_name().and_then(|x| x.to_str()).unwrap_or("");
            // `.github` 도 본다 — CI 정의가 거기 살고 발견이 그것을 가리킨다.
            if n == "target" || n == "node_modules" || n == ".git" {
                continue;
            }
            if p.is_dir() {
                if 찾는다(&p, 끝, 깊이 - 1) {
                    return true;
                }
            } else {
                // ⚠ **경계를 맞춰야 한다.** 앞 판은 맨 접미 비교라 `.md`·`s.rs` 같은
                //   조각이 전부 「실재」로 판정됐다(독립 리뷰 2026-08-19 실측).
                //   경로 구분자나 문자열 처음에서 시작해야 진짜 접미다.
                let 전체 = p.to_string_lossy().replace('\\', "/");
                if 전체 == 끝 || 전체.ends_with(&format!("/{끝}")) {
                    return true;
                }
            }
        }
        false
    }
    찾는다(root, 경로, 10)
}

// ── 검사 21 — 원장 둘을 댄다 (회차 `2026-08-22-agent-laziness` · E) ──────────
//
// # 무엇이 죽은 가지였나
//
// 판정 원장이 **두 자리**에 있었다. `intent.md` 의 완수 조건 상자와 게이트의 `## 판정`.
// 최근 두 회차가 조건 90 개를 **열린 채로** 끝냈고 계기판 ② 는 그것을 읽어
// 「미판정 44/44」라는 **거짓 신호**를 냈다 — 같은 회차의 게이트는 「통과 43」이라 적었다.
// **둘이 갈리는 것을 아무도 안 댔다.** 그것이 이 검사가 닫는 금지역이다.
//
// # 왜 수가 아니라 ID 인가
//
// 소유자가 잠근 문장: *"수를 안 적고 조건 ID를 적는다. ID를 적으면 수는 세면 나온다.
// 캐시가 아니다. 그리고 대조가 **집합 같음**이 되어 훨씬 강하다 — 수만 맞고 내용이
// 다른 경우를 잡는다."*
//
// # 모집단과 짝짓기
//
// **회차 디렉터리 전부**에서 출발하고, 게이트에서 **역인덱스**로 짝을 찾는다.
// 열쇠는 게이트 본문에 적힌 `.palimpsest/rounds/<회차>/intent.md` 다 —
// **디렉터리만 가리키는 것은 짝이 아니다.** 이 검사가 여는 것은 그 파일이고,
// 어느 원장의 짝인지를 선언하는 것이 게이트의 몫이기 때문이다.
//
// ⚠ **`intent.md` 에 새 frontmatter 를 안 만든다.** 반대 방향(의도 → 게이트)으로
// 걸면 회차가 도는 내내 죽은 링크 검사가 빨개진다(격리 사본에서 재현). 게이트는
// 회차 끝에 쓰이므로 **이 방향은 원리상 죽은 링크가 안 생긴다.**
//
// # 하한이 「가장 최근 회차」인 까닭
//
// 전역 개수 하한(*"표준 표가 N 개 이상"*)은 **과거 둘로 영구 충족되어 다시 발화하지
// 않는다.** 그러면 이 검사가 새 회차에 대해 아무것도 요구하지 않는다 — 장치가 놓인
// 날부터 죽은 가지다. 그래서 하한은 **「끝난 회차 중 가장 최근 것이 검사에 들었는가」**다.
//
// # 형식 이전은 오류가 아니다
//
// `docs/gates/README.md:46` 의 선례 — *"옛 게이트를 그 형식으로 옮기지 않는다. 지난
// 판정은 그때의 기록이라 형식을 바꿔도 새로 재는 것이 없다."* 표준 표가 없는 게이트는
// **검사 밖**이고, 그 사실을 판정문에 실어 낸다. 「안 잰 것」을 침묵으로 두지 않는다.

const 게이트_뿌리: &str = "docs/gates";

/// 파이썬 파서를 부르고 JSON 을 받는다.
///
/// ★ **rc 로 판정하지 않는다.** `record.py` 의 `conditions`·`gate` 는 **형식 오류가
/// 있으면 rc=1 을 내면서도 표준출력에 온전한 JSON 을 낸다** — 그것이 이 검사가 읽어야
/// 하는 내용이다. rc 는 「말할 것이 있다」는 신호이지 「출력이 없다」가 아니다.
fn 파서에_묻는다(파이썬: &str, 원천: &Path, 명령: &str, 대상: &Path) -> Result<serde_json::Value> {
    let out = 파이썬_명령(파이썬)
        .arg(원천)
        .arg(명령)
        .arg(대상)
        .output()
        .with_context(|| format!("`{파이썬} {스키마_원천} {명령} {}` 를 못 돌렸다", 대상.display()))?;
    if out.stdout.is_empty() {
        bail!(
            "`{스키마_원천} {명령} {}` 가 아무것도 안 냈다 (rc={}):\n{}",
            대상.display(),
            out.status.code().unwrap_or(-1),
            String::from_utf8_lossy(&out.stderr)
        );
    }
    serde_json::from_slice(&out.stdout)
        .with_context(|| format!("`{스키마_원천} {명령}` 출력이 JSON 이 아니다"))
}

fn 문자열들(v: &serde_json::Value) -> Vec<String> {
    v.as_array()
        .map(|a| a.iter().filter_map(|x| x.as_str().map(str::to_owned)).collect())
        .unwrap_or_default()
}

/// §10 이 금지한 네 이름은 **`--schema` 의 `반환형식.종료보고금지절` 이 진다.**
///
/// ★ 앞 판은 이 목록을 **코드 상수와 규약 문면 두 곳에 손으로** 적었고, 갈려도
/// 아무것도 안 울었다(독립 리뷰 R1). `D3` 가 리뷰어 절 목록에 대해 닫은 것과 같은
/// 표면인데 이 목록만 안 닫혀 있었다.
///
/// ★★ 그 이름들은 **할 수 있었는데 안 한 것**(잔여)과 **원리상 못 하는 것**(경계)을
/// 한 칸에 섞는다. 가르는 문장: **「이 회차에서 그것을 할 수 있었는가.」**
/// §9 의 **표준 검산 줄**인가 — 종료 보고가 이것을 담으면 원장이 셋이 된다.
///
/// 형식: `**검산** — 통과 N · 반증 N · 대조불가 N · 미측정 N = N`
///
/// ⚠ **서술문을 안 잡는다.** 실측: 옛 보고 둘이 *"게이트는 「통과 43」이라 적고"* 처럼
/// **수를 설명하는 문장**을 담는다. 그것은 셋째 원장이 아니라 회고다. 그래서
/// **판정 네 낱말이 한 줄에 다 있는 것**만 잡는다.
///
/// **순수 함수다** — 그래야 음성 대조를 시험으로 세울 수 있다.
fn 표준_검산_줄인가(line: &str) -> bool {
    let l = line.trim();
    if !l.contains('=') {
        return false;
    }
    ["통과", "반증", "미측정"].iter().all(|w| l.contains(w))
        && (l.contains("대조불가") || l.contains("대조 불가"))
}

#[cfg(test)]
mod 종료_보고_시험 {
    use super::표준_검산_줄인가;

    #[test]
    fn 표준_검산_줄을_잡는다() {
        assert!(표준_검산_줄인가("**검산** — 통과 3 · 반증 0 · 대조불가 2 · 미측정 0 = 5"));
        assert!(표준_검산_줄인가("**통과 44 · 반증 0 · 대조 불가 2 · 미측정 0 = 46** ✔"));
    }

    /// ⚠ **서술문은 셋째 원장이 아니다.** 실측한 두 문장으로 못 박는다.
    #[test]
    fn 서술문은_안_잡는다() {
        assert!(!표준_검산_줄인가("게이트는 「통과 43」이라 적고 종료 보고는 다르게 적었다"));
        assert!(!표준_검산_줄인가(
            "종료 보고는 「통과 29 · 미측정 2」였고 게이트는 「통과 31 · 미측정 0」이었다"
        ));
        assert!(!표준_검산_줄인가("판정을 커밋했다"));
    }


}

fn check_ledger_pair(root: &Path) -> Result<String> {
    let 뿌리 = root.join(회차_뿌리);
    if !뿌리.is_dir() {
        bail!("`{회차_뿌리}` 가 없다 — 이 검사의 모집단이 원리상 안 선다");
    }

    // ① 모집단 — 회차 디렉터리 **전부**. `intent.md` 가 있는 것이 회차다.
    let mut 회차들: Vec<String> = Vec::new();
    for e in std::fs::read_dir(&뿌리)? {
        let p = e?.path();
        if p.is_dir() && p.join("intent.md").is_file() {
            회차들.push(p.file_name().and_then(|x| x.to_str()).unwrap_or("").to_string());
        }
    }
    회차들.sort();
    if 회차들.is_empty() {
        bail!(
            "`{회차_뿌리}/*/intent.md` 가 하나도 없다 — 이 검사가 아무것도 안 잰다. \
             0 건은 「안 부른다」가 아니라 「안 봤다」다"
        );
    }

    // ② 역인덱스 — 게이트가 어느 회차의 `intent.md` 를 가리키나.
    let mut 게이트들: Vec<(String, String)> = Vec::new(); // (상대경로, 본문)
    for e in std::fs::read_dir(root.join(게이트_뿌리))? {
        let p = e?.path();
        if p.extension().and_then(|x| x.to_str()) == Some("md") {
            게이트들.push((상대_경로(root, &p), std::fs::read_to_string(&p)?));
        }
    }
    게이트들.sort();

    let 파이썬 = 파이썬_실행자()?;
    let 원천 = root.join(스키마_원천);
    if !원천.exists() {
        bail!("파서 원천이 없다: {스키마_원천}");
    }

    let (형식이전_선언, _) = 선언_목록(root, "형식 이전", false)?;
    let (검산줄_유예, _) = 선언_목록(root, "종료 보고 검산 줄 유예", true)?;
    let (보고없음_유예, _) = 선언_목록(root, "종료 보고 없음 유예", true)?;
    // ★ 금지 네 이름은 선언이 진다 — 여기가 아니다(독립 리뷰 R1 · 발견 12).
    let 스키마 = 스키마를_읽는다(root)?;
    let 금지_절: Vec<String> = 문자열들(&스키마["반환형식"]["종료보고금지절"]);
    if 금지_절.is_empty() {
        bail!("`--schema` 의 `반환형식.종료보고금지절` 이 비었다 — 이 검사가 아무것도 안 잰다");
    }

    let mut problems = Vec::new();
    let mut 검사안: Vec<String> = Vec::new();
    let mut 형식이전: Vec<String> = Vec::new();
    let mut 종료보고_검사 = 0usize;
    let mut 검산줄_유예_발화 = 0usize;
    let mut 보고없음_유예_발화 = 0usize;
    let mut 게이트없음: Vec<String> = Vec::new();
    // 접힌 회차 — 게이트를 안 진다. **세어서 보고만 한다**(위 「접힌 회차는 게이트를 안 진다」).
    let mut 접힘: Vec<String> = Vec::new();
    let mut 댄_조건 = 0usize;
    let mut 댄_미측정 = 0usize;

    // ── C7 — **「끝난 회차」의 정의를 안 쓰는 것으로 비껴가지 않는다** ──────────
    //
    // ★ 사전부검 R3 이 **더 넓은 문짝**을 찾았다: 「끝난 회차」의 유일한 기계 정의가
    //   `report.md` 존재인데, **그 파일이 있어야 한다고 요구하는 검사가 하나도
    //   없었다.** 그래서 **파일 하나를 안 쓰는 것**으로 `C1`·`G1`·`G2` 셋을 동시에
    //   무력화할 수 있었다.
    //
    // ★★ **산문도 사전순도 안 쓴다.** 첫 판은 `state.md` 의 「단계」 줄을 읽었는데
    //   그 표기가 회차마다 갈렸다. 둘째 판은 「가장 최근 회차」를 **디렉터리 이름의
    //   사전순**으로 잡았는데, 같은 날짜에 여는 다음 회차의 슬러그가 앞서면
    //   **새 회차가 거짓 실패하고 진짜 진행 중 회차가 대신 면제된다**
    //   (독립 리뷰 R2 · 발견 9 — 이번엔 우연히 안 걸렸다).
    //
    //   **그래서 순서를 안 쓴다. 세기만 한다:**
    //
    //   > **진행 중인 회차는 하나다.** 종료 보고가 없는 회차가 둘 이상이면,
    //   > 그중 적어도 하나는 **끝났는데 안 썼거나 버려진 것**이다.
    //
    //   ⚠ 그 회차의 종료 보고를 **지금 지어내지 않는다** — 그때 쓴 사람만 쓸 수 있다.
    //   그래서 선언 목록으로 **빚**을 세우고 판정문이 매 실행 수를 낸다.
    // ★★ **접힘 — `folded.md` 가 기계 표시다.** (2026-08-24)
    //
    //   접힌 회차는 종료 보고를 안 쓴다(규약 §5 「접힘」). 그래서 `report.md` 만 보면
    //   **접힌 회차가 영원히 「진행 중」**이 되고, 다음 회차를 여는 순간 이 검사가
    //   거짓으로 빨개진다(실측 2026-08-24 · 격리 클론에서 재현).
    //
    //   ⚠ **산문을 안 읽는다** — 위 C7 주석이 적은 그대로다. `state.md` 의 표기는
    //   회차마다 갈렸다. 그래서 **파일 하나의 존재**를 표시로 쓴다.
    //
    //   ★ **빈 파일로 비껴가지 못한다.** `## 왜 접었나` 가 없으면 표시로 안 쳐 주고
    //   빨개진다 — 그것이 접힘과 「조용한 축소」를 가르는 유일한 것이다.
    //   선행 하네스는 `abandoned` 를 열일곱 번 쓰고도 이유 필드가 없어 하나도 못 읽는다.
    for 회차 in &회차들 {
        let 접힘문서 = 뿌리.join(회차).join("folded.md");
        if 접힘문서.is_file() {
            let 본문 = std::fs::read_to_string(&접힘문서).unwrap_or_default();
            if !본문.contains("## 왜 접었나") {
                problems.push(format!(
                    "`{회차}/folded.md` 에 **`## 왜 접었나` 가 없다** — 사유 없는 접힘은                      접힘이 아니라 **조용한 축소**다. 규약 §5 「접힘」이 그 절을 요구한다"
                ));
            }
            if 뿌리.join(회차).join("report.md").is_file() {
                problems.push(format!(
                    "`{회차}` 에 `folded.md` 와 `report.md` 가 **둘 다** 있다 —                      접힘과 종료는 배타다. 하나가 거짓이다"
                ));
            }
        }
    }

    let 진행중: Vec<&String> = 회차들
        .iter()
        .filter(|r| !기록이_확정됐나(&뿌리.join(r)))
        .filter(|r| {
            if 보고없음_유예.iter().any(|x| &x == r) {
                보고없음_유예_발화 += 1;
                false
            } else {
                true
            }
        })
        .collect();
    if 진행중.len() > 1 {
        problems.push(format!(
            "종료 보고가 없는 회차가 **{}** 이다: {} — **진행 중인 회차는 하나다.** \
             §10 이 종료 보고의 자리를 정했고 그것이 「끝난 회차」의 유일한 기계 정의라, \
             안 쓰면 표준 표 검사와 종료 보고 검사를 **한 수로** 비껴간다",
            진행중.len(),
            진행중.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(" · ")
        ));
    }

    for 회차 in &회차들 {
        let 열쇠 = format!("{회차_뿌리}/{회차}/intent.md");
        let 짝: Vec<&(String, String)> =
            게이트들.iter().filter(|(_, 본문)| 본문.contains(&열쇠)).collect();
        // ★★ **접힌 회차는 게이트를 안 진다.** (2026-08-24)
        //
        //   이 자리가 요구하는 것은 *"끝난 회차의 판정 원장이 한 자리뿐이다"* 인데,
        //   **접힌 회차는 판정을 안 했다** — 완수 조건이 「통과」가 아니라 **「안 쟀다」**로
        //   남는다(규약 §5 「접힘」·§11). 그런 회차에 게이트 문서를 요구하면
        //   **「안 쟀다」를 「판정했다」로 위장하게 만든다** — 기본 금지역
        //   「사실이 아닌 것을 사실로」다. 그러므로 요구하지 않고 **세어서 보고만 한다.**
        //
        //   ⚠ 그래도 「끝났다」로는 봐야 한다. 안 그러면 접힌 회차가 「게이트 없음」에
        //   영구히 뜨고, 다음 사람이 그것을 「게이트를 써야 하는데 안 썼다」로 읽어
        //   **접은 회차를 되살린다.**
        let 접혔나 = 뿌리.join(회차).join("folded.md").is_file();
        let 종료보고를_썼나 = 뿌리.join(회차).join("report.md").is_file();
        let 끝났나 = 종료했나(&뿌리.join(회차));
        let 종료보고_형식이전 = 끝났나 && !종료보고를_썼나;

        // ── G1 · G2 — **종료 보고를 검사 모집단에** ──────────────────────────
        if 종료보고를_썼나 {
            let rp = format!("{회차_뿌리}/{회차}/report.md");
            let body = std::fs::read_to_string(뿌리.join(회차).join("report.md"))?;
            종료보고_검사 += 1;

            // G1 — **원장은 둘이고 셋이 아니다.**
            //
            // ★ 앞 판을 두 번 뒤집었다. 처음엔 「검산 줄을 게이트와 **댄다**」였는데,
            //   ⓐ 엄격하게 재면 모집단이 0 이라 **태어나면서 죽은 가지**이고
            //   ⓑ 느슨하게 재면 *"게이트는 통과 43 이라 적고"* 같은 **서술문**이 걸린다.
            //   그리고 앞 회차의 사후 검증이 *"여기에 수를 안 적는다"* 로 그 줄을
            //   **이미 지웠다.** 그러므로 검사가 할 일은 대조가 아니라
            //   **원장이 셋으로 늘어나는 것을 막는 것**이다.
            //   AGENTS.md — *"같은 것을 두 곳에 적으면 그것이 곧 drift 다."*
            if let Some((n, line)) = body
                .lines()
                .enumerate()
                .find(|(_, l)| 표준_검산_줄인가(l))
            {
                if 검산줄_유예.iter().any(|x| x == 회차) {
                    검산줄_유예_발화 += 1;
                } else {
                    problems.push(format!(
                        "{rp}:{}: 종료 보고가 §9 표준 검산 줄을 담는다 — **원장은 둘이다.** \
                         판정은 게이트의 표준 표와 `intent.md` 의 상자가 지고, 종료 보고가 \
                         셋째 사본이 되면 그것이 곧 drift 다: `{}`",
                        n + 1,
                        line.trim()
                    ));
                }
            }

            // G2 — §10 이 금지한 **네 이름**. **절머리만** 본다.
            //
            // ★ 본문 문자열로 재면 옛 보고 넷이 §10 을 **인용한 문장** 때문에 거짓
            //   실패한다(실측). 절머리로 재면 옛 보고 **여섯 전부 초록**이고,
            //   `## 다음 회차가 받는 것` 은 §10 이 **정한 이름**이라 우회가 아니다.
            for (n, l) in body.lines().enumerate() {
                let Some(h) = l.strip_prefix("## ") else { continue };
                let h = h.trim();
                if 금지_절.iter().any(|x| x == h) {
                    problems.push(format!(
                        "{rp}:{}: 종료 보고에 §10 이 금지한 절 이름이 있다: `## {h}` — \
                         그 이름들은 **할 수 있었는데 안 한 것**(잔여)과 **원리상 못 하는 \
                         것**(경계)을 한 칸에 섞는다",
                        n + 1
                    ));
                }
            }
        }

        if 짝.len() > 1 {
            problems.push(format!(
                "형식 오류 · {회차}: 게이트 {} 개가 같은 회차를 가리킨다 ({}) — \
                 원장의 짝은 하나여야 한다",
                짝.len(),
                짝.iter().map(|(p, _)| p.as_str()).collect::<Vec<_>>().join(" · ")
            ));
            continue;
        }
        // ★★ **요구하지 않는 것과 있는 것도 안 보는 것은 다르다.** (정정 2026-08-24)
        //
        //   앞 판은 여기서 바로 `continue` 했다. 그러면 **접힌 회차를 가리키는 게이트가
        //   실재해도 대조를 안 한다** — 실측: 없는 조건을 「통과」로 적은 게이트를 두어도
        //   전량 초록이었다. 이 자리 주석이 막겠다고 적은 *"「안 쟀다」를 「판정했다」로
        //   위장"* 이 **바로 그 자리에서 성립했다.**
        //
        //   그래서 갈랐다 — **없으면 요구하지 않고**(접힘은 판정을 안 했다),
        //   **있으면 끝까지 대조한다**(썼으면 그것은 판정을 주장하는 것이다).
        if 접혔나 && 짝.is_empty() {
            접힘.push(회차.clone());
            continue;
        }
        if 접혔나 {
            접힘.push(회차.clone());
        }
        let Some((게이트, _)) = 짝.first().copied() else {
            // ③ 짝이 없다 — 「게이트 없음」.
            게이트없음.push(회차.clone());
            if 끝났나 && !종료보고_형식이전 {
                problems.push(format!(
                    "{회차}: 게이트 없음 — `report.md` 가 있는데 `{열쇠}` 를 가리키는 \
                     게이트가 `{게이트_뿌리}/` 에 없다. 끝난 회차의 판정 원장이 한 자리뿐이다"
                ));
            }
            continue;
        };

        let g = 파서에_묻는다(파이썬, &원천, "gate", &root.join(게이트))?;
        if !g["표준표"].as_bool().unwrap_or(false) {
            // ④ 표준 표가 없다 — **선언됐으면** 형식 이전, **아니면 실패**.
            //
            // ★★ **#90 이 실측한 문을 여기서 닫는다.** 앞 판은 표 헤더가 `| 판정 | 조건 |`
            //   인지만 봤고, 못 찾으면 **조용히 검사 밖**이었다. 그래서 헤더의 `조건` 을
            //   `조건들` 로 **한 글자** 고치면 그 회차가 빠지고 판정문의 조건 수만 줄어든
            //   채 **초록**이었다(리뷰어 실측: 159 → 113).
            //
            //   이제 면제는 `docs/gates/README.md` 의 **닫힌 선언 목록**이 준다. 목록은
            //   날짜 하한을 지고 [`check_declared_lists`] 가 그것을 강제한다 —
            //   **거울은 자라는 모집단을 베끼고, 예외 선언은 닫혀 있다.**
            if 끝났나 && !형식이전_선언.iter().any(|x| x == 회차) {
                problems.push(format!(
                    "{회차}: **끝난 회차**(`report.md` 가 있다)인데 게이트 `{게이트}` 에 \
                     표준 표가 없고 `docs/gates/README.md` 의 「형식 이전」 선언 목록에도 \
                     없다 — 표 헤더 한 글자로 조용히 검사 밖으로 나가지 않는다"
                ));
                continue;
            }
            형식이전.push(format!("{회차} ({게이트})"));
            continue;
        }
        // ★ **절 넷을 갖는가** — [#76] 의 「만들 것」 첫 항이고, 이 회차가 *"#76 흡수"*
        //   를 고르고도 **절반만 했다**(독립 리뷰 R7). #76 이 태어난 사건이 정확히
        //   *"치환이 `## 효과` 절 전체를 삼켰다"* 인데, 그 상태가 여기서 초록이었다.
        //   ⚠ **표준 표를 가진 게이트만 잰다** — 옛 게이트는 형식 이전이라 검사 밖이다.
        let 게이트본문 = std::fs::read_to_string(root.join(게이트))?;
        // ⚠ **펜스를 본다.** (독립 리뷰 R8) 앞 판은 `lines().any(starts_with)` 라
        //    **코드펜스 안의 `## 효과` 한 줄로 속일 수 있었다** — 절을 통째로 지우고
        //    예시 블록에 그 글자만 남기면 21/21 초록이었다. `record.py` 의 두 파서는
        //    처음부터 펜스를 본다. **같은 규율을 여기에도 세운다.**
        let mut 펜스 = false;
        let 절머리: Vec<&str> = 게이트본문
            .lines()
            .filter(|l| {
                if l.trim_start().starts_with("```") {
                    펜스 = !펜스;
                    return false;
                }
                !펜스
            })
            .map(str::trim_start)
            .collect();
        for 절 in ["## 합격선", "## 판정", "## 효과", "## 범위 밖"] {
            if !절머리.iter().any(|l| l.starts_with(절)) {
                problems.push(format!(
                    "형식 오류 · {게이트}: 절 `{절}` 이 없다 — 규약 §9 가 절 이름 **넷**을 \
                     고정한다. 한 번은 문자열 치환이 `## 효과` 절 전체를 삼켰고 \
                     아무것도 그것을 못 잡았다"
                ));
            }
        }

        let 게이트오류 = 문자열들(&g["형식오류"]);
        if !게이트오류.is_empty() {
            for m in 게이트오류 {
                problems.push(format!("형식 오류 · {게이트}: {m}"));
            }
            continue;
        }

        let 의도경로 = 뿌리.join(회차).join("intent.md");
        let 의도본문 = std::fs::read_to_string(&의도경로)?;
        let c = serde_json::to_value(pal_intent::round_condition::ConditionsReport::parse(
            의도경로.to_string_lossy(),
            &의도본문,
        ))?;
        let 조건들 = c["조건"].as_array().cloned().unwrap_or_default();

        // ★ **모집단이 비면 실패다.** (독립 리뷰 R1 · 2026-08-23)
        //
        // 표준 표 넷을 `—` 로 두고 검산을 `0 · 0 · 0 · 0 = 0` 으로 적으면 집합 대조가
        // **공집합끼리** 맞아 초록이 된다 — `intent.md` 의 `## 완수 조건` 절이 통째로
        // 사라져도 그렇다. 격리 사본에서 재현했다: 조건 44 가 사라지고 판정문이
        // `(0개)` 를 찍은 채 **21/21 통과**.
        //
        // ⚠ **이 검사가 닫으려던 금지역이 바로 그것이다** — 「검사가 있는데 실제로는
        // 아무것도 재지 않는다」. 자기 자신에게 그 구멍을 남긴 채로 놓을 수 없다.
        // 같은 규율이 회차 레코드 검사에도 있다: *"0 건은 「안 부른다」가 아니라
        // 「안 봤다」다."*
        if 조건들.is_empty() {
            problems.push(format!(
                "형식 오류 · {회차_뿌리}/{회차}/intent.md: `## 완수 조건` 에 상자가 하나도 \
                 없다 — 게이트 `{게이트}` 는 표준 표를 세웠는데 **댈 것이 없다.** 0 건은 \
                 「안 부른다」가 아니라 「안 봤다」다"
            ));
            continue;
        }
        let mut 의도오류 = false;
        for 조건 in &조건들 {
            for m in 문자열들(&조건["형식오류"]) {
                problems.push(format!(
                    "형식 오류 · {회차_뿌리}/{회차}/intent.md:{}: {m}",
                    조건["줄"].as_i64().unwrap_or(0)
                ));
                의도오류 = true;
            }
        }
        if 의도오류 {
            continue;
        }

        // ⑤ **양방향 집합 같음** + 조건마다 상자·태그.
        let 게이트판정 = g["판정"].as_object().cloned().unwrap_or_default();
        let mut 의도판정: std::collections::BTreeMap<String, (bool, String)> = Default::default();
        for 조건 in &조건들 {
            let Some(id) = 조건["id"].as_str() else { continue };
            의도판정.insert(
                id.to_string(),
                (
                    조건["상자"].as_bool().unwrap_or(false),
                    조건["판정"].as_str().unwrap_or("미측정").to_string(),
                ),
            );
        }
        for (id, 판정) in &게이트판정 {
            let 게 = 판정.as_str().unwrap_or("");
            match 의도판정.get(id) {
                None => problems.push(format!(
                    "{회차}: 게이트가 `{id}` 를 「{게}」로 적었는데 `intent.md` 에 그 조건이 없다"
                )),
                Some((상자, 의)) => {
                    if 의 != 게 {
                        problems.push(format!(
                            "{회차}: `{id}` — 게이트는 「{게}」, `intent.md` 는 「{의}」다. \
                             원장 둘이 갈렸다"
                        ));
                    }
                    // 규약 §3: **상자 켜짐 = 판정이 났다.** 안 켜짐 = 미측정.
                    let 켜져야 = 게 != "미측정";
                    if *상자 != 켜져야 {
                        problems.push(format!(
                            "{회차}: `{id}` — 게이트는 「{게}」인데 상자가 {}. \
                             상자 켜짐 = 판정이 났다는 뜻이다",
                            if *상자 { "켜져 있다" } else { "안 켜져 있다" }
                        ));
                    }
                }
            }
        }
        for id in 의도판정.keys() {
            if !게이트판정.contains_key(id) {
                problems.push(format!(
                    "{회차}: `intent.md` 의 `{id}` 가 게이트 표준 표에 없다 — \
                     넷 중 어디에도 안 적히면 그 조건은 조용히 안 세어진다"
                ));
            }
        }
        댄_조건 += 의도판정.len();
        // ★ **「조건 N」은 판정한 수가 아니라 ID 집합 크기다.** (독립 리뷰 R3)
        //   양쪽이 전부 `미측정` 이어도 집합은 맞으므로 초록이고, 그때 `조건 N` 만
        //   보면 N 개를 **쟀다**고 읽힌다. 미측정 수를 함께 낸다 — 판정하지는 않는다.
        댄_미측정 += 의도판정.values().filter(|(_, v)| v == "미측정").count();
        검사안.push(format!("{회차} ({}개)", 의도판정.len()));
    }

    // ⑥ **하한 — 끝난 회차 중 가장 최근 것이 검사에 들었는가.**
    //    전역 개수 하한은 과거 둘로 영구 충족되어 다시 발화하지 않는다.
    let 최근_끝난 = 회차들
        .iter()
        .rev()
        .find(|회차| 뿌리.join(회차).join("report.md").is_file())
        .cloned();
    let 하한 = match &최근_끝난 {
        None => "끝난 회차가 아직 없다".to_string(),
        Some(회차) => {
            if 검사안.iter().any(|s| s.starts_with(회차.as_str())) {
                format!("최근 끝난 회차 `{회차}` 가 검사에 들었다")
            } else {
                problems.push(format!(
                    "하한 미충족: 끝난 회차 중 가장 최근인 `{회차}` 가 이 검사 밖이다 — \
                     표준 표를 세우거나 게이트를 짝지어야 한다. \
                     전역 개수 하한은 과거로 영구 충족되므로 하한을 여기 건다"
                ));
                format!("최근 끝난 회차 `{회차}` 가 검사 밖이다")
            }
        }
    };

    // ★ **이 검사가 아무것도 안 쟀으면 실패다.** (독립 리뷰 R2 · 2026-08-23)
    //
    // 하한은 *끝난 회차가 있을 때만* 걸고, 빈-모집단 가드는 *짝지어진 회차마다* 건다.
    // 그래서 `report.md` 를 전부 지우고 표준 표를 깨면 **둘 다 안 걸리고**
    // `검사 안 없음 (조건 0)` 으로 **초록**이 됐다. 판정문이 「조건 0」을 말하므로
    // 거짓은 아니지만, **아무것도 안 재는 검사가 통과하는 것**이 이 회차가 닫으러 온
    // 금지역이다. 회차 레코드 검사가 쓰는 규율과 같은 자로 막는다.
    // ⚠ **끝난 회차가 하나도 없으면 이 가드를 안 건다.** (독립 리뷰 R3)
    //   앞 판은 무조건 걸어서 「진행 중인 회차 하나뿐인 새 프로젝트」를 **거짓 실패**
    //   시켰다 — E3 가 *"회차 진행 중이면 보고"* 라고 등록한 자리와 정면으로 어긋난다.
    //   **잴 것이 있어야 할 때만** 「안 쟀다」를 실패로 낸다.
    if 댄_조건 == 0 && 최근_끝난.is_some() {
        problems.push(format!(
            "이 검사가 조건을 **하나도 안 쟀다** — 끝난 회차 `{}` 가 있는데 표준 표를 \
             가진 짝이 하나도 없다. 0 건은 「안 부른다」가 아니라 「안 봤다」다",
            최근_끝난.as_deref().unwrap_or("")
        ));
    }

    if !problems.is_empty() {
        bail!("{}", problems.join("\n    "));
    }
    Ok(format!(
        "회차 {} · 검사 안 {} (조건 {댄_조건} · 그중 미측정 {댄_미측정}) · \
         형식 이전 {} · 게이트 없음 {} · **접힘 {}** · 종료 보고 {종료보고_검사}개 검사 \
         (검산 줄 유예 발화 {검산줄_유예_발화} · 보고 없음 유예 발화 {보고없음_유예_발화}) · {하한}",
        회차들.len(),
        if 검사안.is_empty() { "없음".to_string() } else { 검사안.join(" · ") },
        if 형식이전.is_empty() { "0".to_string() } else { 형식이전.join(" · ") },
        if 게이트없음.is_empty() { "0".to_string() } else { 게이트없음.join(" · ") },
        if 접힘.is_empty() { "0".to_string() } else { 접힘.join(" · ") },
    ))
}
