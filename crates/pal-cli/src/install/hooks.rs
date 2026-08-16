//! 대상 `settings.json` 의 훅 구역 — **등록 · 갱신 · 제거**(`[f24]` ⑧).
//!
//! # 실측이 이 파일의 모든 줄을 정했다 — **exec form 을 쓴다**
//!
//! 하네스(Claude Code 2.1.233 · darwin-arm64) 안의 스키마 원문:
//!
//! > *"Argument list for exec form. When present, `command` is resolved as an
//! > executable and spawned directly with these arguments — **no shell**. Path
//! > placeholders … are substituted per-element as plain strings, so paths with
//! > quotes, `$`, or backticks **never reach a shell parser**. When absent, `command`
//! > runs through a shell (bash on POSIX, **PowerShell on Windows without Git Bash**)."*
//!
//! 그래서 **`args` 를 언제나 둔다.** 얻는 것 셋:
//!
//! - **셸 인용이 통째로 사라진다.** 공백·`$`·따옴표·백틱이 든 경로가 그대로 argv 에
//!   도착한다(실행으로 확인: 조상 프로세스에 `sh` 가 없다).
//! - **Windows 에서 갈리지 않는다.** shell form 은 Git Bash 가 없으면 **PowerShell**
//!   로 도는데, POSIX 홑따옴표는 PowerShell 의 인용이 아니다. 소유자 결정
//!   (2026-08-16): *"windows 를 대응한다는 가정하에 앞으로 모든 설계와 개발이 되어야
//!   해."* 이 한 줄이 그 결정이 코드에 닿는 첫 자리다.
//! - **stdin JSON 과 종료 코드 해석이 shell form 과 동일하다**(SHA-256 까지 같다).
//!   그래서 [`crate::hook`] 의 규약은 한 글자도 안 움직인다.
//!
//! 그 밖의 실측:
//!
//! - **중복 제거는 명령 문자열의 완전 일치 기준이다. 공백 하나만 달라도 두 번 돈다.**
//!   그래서 이 파일이 만드는 항목은 **바이트 단위로 안정적**이어야 하고, 제거도
//!   완전 일치로만 한다. 우리는 **사건 하나에 항목 하나**만 걸므로 `command` 가 같고
//!   `args` 만 다른 두 항목을 같은 배열에 넣는 일이 없다.
//! - **훅은 전 레이어의 합집합**이다. 우리는 프로젝트 레이어 하나만 만지고, 남이
//!   같은 사건에 걸어 둔 것을 **하나도 안 건드린다.**
//!
//! # ⚠ 밟으면 조용히 죽는 자리 셋
//!
//! - **`args: []` 도 exec form 이다.** 빈 배열이면 `command` **문자열 전체**가 실행
//!   파일 경로가 되어 ENOENT 로 죽는다. [`더한다`] 가 그것을 거부한다.
//! - **`shell` 키에 enum(`bash`/`powershell`) 밖 값을 넣으면 그 훅 배열 **전체**가
//!   조용히 사라진다.** 어느 채널에도 흔적이 없다. **우리는 그 키를 안 쓴다.**
//! - **exec form 의 실행 실패는 종료 코드가 항상 1 이고 기본 채널에서 침묵한다**
//!   (shell form 의 126/127 이 안 나온다). `pal doctor` 가 유일한 문이라는 사실이
//!   여기서 한 겹 더 세진다.
//!
//! # ★ PATH 이름으로 등록하지 않는다
//!
//! 실측상 PATH 이름 등록도 동작한다(exec form 에서도 이름 탐색이 된다). 그런데
//! **실행 파일을 못 찾으면 그 실패가 완전히 침묵한다** — `claude -p` 의 stdout·
//! stderr·종료 코드 어디에도 안 나온다. 그래서 **설치 시점에 해석한 절대 경로**로
//! 등록한다.
//!
//! # 남의 구조를 고치려 들지 않는다
//!
//! `hooks` 가 객체가 아니거나 사건 자리가 배열이 아니면 **고치지 않고 멈춘다.**
//! 반쯤 고친 설정은 하네스의 `-p` 에서 완전히 침묵하고, 그러면 아무도 그 상태를 모른다.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use serde_json::{Map, Value, json};

use super::manifest::HookEntry;

/// 설정 안의 최상위 훅 구역.
const HOOKS: &str = "hooks";
/// 묶음 안의 명령 목록.
const GROUP: &str = "hooks";
const COMMAND: &str = "command";
const KIND: &str = "type";
const KIND_COMMAND: &str = "command";
/// **이 키가 있으면 exec form 이다.** 이름을 하네스 스키마에서 그대로 빌렸다.
const ARGS: &str = "args";
/// 우리 바이너리의 서브커맨드.
const SUBCOMMAND: &str = "hook";

// ─────────────────────────────────────────────────────────────────────────────
// 등록 항목 — **바이트 단위로 안정적이어야 한다**
// ─────────────────────────────────────────────────────────────────────────────

/// 이 설치가 등록할 것. 사건 목록이 비면 실행 파일도 안 찾는다.
///
/// # Errors
/// 자기 실행 파일의 경로를 못 알아내면.
pub fn desired(events: &[&str]) -> Result<Vec<HookEntry>> {
    if events.is_empty() {
        return Ok(Vec::new());
    }
    let exe = 실행_파일()?;
    Ok(events.iter().map(|e| entry(&exe, e)).collect())
}

/// **지금 도는 이 바이너리**의 절대 경로.
///
/// ⚠ 홈을 안 읽는다 — `[f24]` ⑦ 이 재는 자리다.
fn 실행_파일() -> Result<PathBuf> {
    let exe = std::env::current_exe().context(
        "자기 실행 파일의 경로를 못 알아냈다 — 훅을 등록할 절대 경로가 없다",
    )?;
    // 심링크를 풀어 둔다. 안 풀면 링크가 갈릴 때 등록 문자열이 **조용히** 딴 것을 가리킨다.
    Ok(exe.canonicalize().unwrap_or(exe))
}

/// 등록 항목 하나 — **exec form.**
///
/// `command` 는 실행 파일 경로 **그 자체**다. 따옴표도 이스케이프도 안 붙인다 —
/// 셸을 안 거치므로 붙이면 오히려 그 글자가 경로의 일부가 된다.
#[must_use]
pub fn entry(exe: &Path, event: &str) -> HookEntry {
    HookEntry {
        event: event.to_owned(),
        command: exe.to_string_lossy().into_owned(),
        args: Some(vec![SUBCOMMAND.to_owned(), event.to_owned()]),
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// ★ 탐침 — **「적혀 있다」로는 부족하다. 그러나 남의 문자열을 돌리지는 않는다**
// ─────────────────────────────────────────────────────────────────────────────

/// 이 항목이 **셸을 거치는 옛 형태**인가 — `args` 가 없으면 그렇다.
///
/// 이 회차 이전에 설치된 프로젝트가 여기 걸린다. **「우리 것이 아니다」와 다르다** —
/// 우리 것이 맞고, `pal update` 가 옮긴다.
#[must_use]
pub fn 옛_형태인가(entry: &HookEntry) -> bool {
    entry.args.is_none()
}

/// 등록 항목을 **우리 형태로 되읽는다.** 실패하면 우리가 쓴 것이 아니다.
///
/// # ★ 이 함수의 결과를 실행에 쓰지 않는다
///
/// `.claude/pal/manifest.json` 과 `.claude/settings.json` 은 **대상 프로젝트 안의
/// 평범한 파일**이고 `.gitignore` 에 없어서 **커밋되고 clone 과 함께 이동한다.**
/// 그 안의 문자열은 **입력이지 사실이 아니다** — 서명도 소유 확인도 없다. 그래서
/// 여기서 되읽은 경로는 **존재와 실행 권한을 `stat` 으로 보는 데만** 쓰고, 프로세스를
/// 띄우는 데는 [`probe`] 가 **지금 도는 이 실행 파일**만 쓴다.
///
/// ⚠ **exec form 에서 이 규율이 한 겹 더 중요해진다.** shell form 의 `command` 는
/// 셸 문법이라 「우리 형태인가」를 파싱으로 물어야 했지만, exec form 의 `command` 는
/// **실행 파일 경로 그 자체**다 — 그것을 그대로 띄우면 남이 커밋해 보낸 임의의
/// 바이너리가 돈다. 그래서 [`entry`] 가 만드는 형태 하나만 되읽는다:
/// `args == ["hook", "<사건>"]`.
#[must_use]
pub fn 되읽는다(entry: &HookEntry) -> Option<PathBuf> {
    let args = entry.args.as_deref()?;
    if args.len() != 2 || args[0] != SUBCOMMAND || args[1] != entry.event {
        return None;
    }
    if entry.command.is_empty() {
        return None;
    }
    Some(PathBuf::from(&entry.command))
}

/// 등록된 자리가 **실행될 수 있는가** — `stat` 으로만 본다. **안 돌린다.**
///
/// 실행 파일이 사라지면 하네스는 exit **127**, 실행 권한을 잃으면 exit **126** 을 내고
/// 그 실패를 **완전히 삼킨다** — 세션은 계속되고 `claude` 의 종료 코드는 0 이며
/// 트랜스크립트에도 대화형 화면에도 한 글자도 안 나온다. 그래서 *"`settings.json` 에
/// 적혀 있다"* 는 **아무것도 보증하지 않는다.**
///
/// # Errors
/// 그 자리에 파일이 없거나 실행 권한이 없으면.
pub fn 실행할_수_있나(path: &Path) -> Result<()> {
    let meta = std::fs::metadata(path).map_err(|e| {
        anyhow::anyhow!(
            "등록된 실행 파일이 없다: {} ({e}) — 하네스는 여기서 exit 127 을 내고 \
             그 실패를 완전히 삼킨다. `pal update` 가 등록을 지금 실행 파일로 맞춘다",
            path.display()
        )
    })?;
    if !meta.is_file() {
        bail!(
            "등록된 자리가 일반 파일이 아니다: {} — 하네스는 여기서 exit 126/127 을 내고 \
             그 실패를 완전히 삼킨다",
            path.display()
        );
    }
    // ★ **이 겹이 플랫폼마다 축이 다르다 — 그러나 어느 쪽에도 있다.**
    //
    // 유닉스에서 「실행될 수 있는가」를 정하는 것은 **모드 비트**이고, Windows 에서
    // 그것을 정하는 것은 **확장자**다(실측: `PATH` 의 확장자 없는 사본은
    // `Executable not found`, 등록된 절대 경로는 확장자가 없으면 `.exe` 가 붙는다).
    // 파일이 있고 바이트도 우리 것인데 **OS 가 못 띄우고 하네스가 그 실패를 삼킨다** —
    // 두 축에서 일어나는 사건이 같다.
    //
    // ⚠ 옛 코드는 모드 비트 검사를 `#[cfg(unix)]` 로 감싸고 *"다른 플랫폼에서는 이
    // 한 겹이 빠진다"* 를 주석으로만 적었다. 그 주석은 **초록을 내면서 아무것도 안
    // 재는 상태**였다. 분기는 [`super::exe`] 한 자리에 있고 여기서는 묻기만 한다.
    if super::exe::자리가_열리나(path).is_none() {
        bail!(
            "등록된 자리가 실행될 수 없다: {} — {}. 하네스는 여기서 exit 126 을 내고 \
             그 실패를 완전히 삼킨다",
            path.display(),
            super::exe::안_열리는_까닭(path)
        );
    }
    Ok(())
}

/// 등록된 자리가 **우리가 등록한 그것인가** — 바이트로 댄다. ⚠ **안 돌린다.**
///
/// # ★ 왜 「대상 안인가」가 아닌가
///
/// 경계 검사([`super::manifest::자리들`])는 매니페스트의 `Rel` 필드 전부를 훑는데,
/// **훅의 `command` 만 `Rel` 이 아니라 `String`** 이라 그 모집단 밖에 있다. 그래서
/// *"자리들에 넣자"* 가 첫 생각이 되는데 **그 문은 여기 원리상 안 선다** — 우리가
/// 등록하는 것은 [`실행_파일`] 이 설치 시점에 해석한 **`pal` 의 절대 경로**이고,
/// 그것은 **대상 프로젝트 밖인 것이 정상**이다(`/usr/local/bin/pal`).
///
/// 그러니 물을 것은 *"대상 안인가"* 가 아니라 **"우리가 등록한 그것인가"** 다.
///
/// # 그 판정을 무엇으로 하는가 — **경로가 아니라 바이트**
///
/// | 후보 | 왜 안 골랐나 |
/// |---|---|
/// | 경로 문자열 일치 | **너무 좁다.** 설치한 `pal` 과 지금 도는 `pal` 이 다른 자리에 있는 것은 정상이다(복사본·심링크 농장·`PATH` 앞뒤). 그때마다 빨강이면 그 빨강이 무의미해진다 |
/// | 같은 파일(dev·ino) | 위와 같은 이유로 좁고, **Windows 에 std 로 여는 문이 없다** |
/// | 서명·소유자 | 이 빌드에 서명이 없다. 없는 것을 근거로 삼지 않는다 |
/// | **바이트 일치** ← 고른 것 | 아래 한 문장이 이 결정의 전부다 |
///
/// ★ **[`probe`] 는 [`std::env::current_exe`] 를 돌려서 「훅 규약이 선다」고 말한다.
/// 그 증거가 등록된 것에 대해 무엇이라도 말하려면, 등록된 것이 그것과 같은 프로그램
/// 이어야 한다.** 바이트 일치가 정확히 그 다리다. 경로 일치는 필요하지도 충분하지도
/// 않다.
///
/// ⚠ **여전히 안 돌린다.** 읽어서 대기만 한다 — 저장소에서 읽은 문자열을 실행하는
/// 구멍은 앞 회차가 막았고([`probe`] 의 머리말) 이 문은 그 위에 **대조만** 더한다.
///
/// # Errors
/// 못 읽거나, 지금 도는 이 실행 파일과 **다른 프로그램**이면.
pub fn 우리가_등록한_것인가(path: &Path) -> Result<()> {
    let 지금 = 실행_파일()?;
    let Some(까닭) = 다른_점(path, &지금)? else { return Ok(()) };
    bail!(
        "등록된 것이 **지금 도는 이 `pal` 이 아니다**({까닭}).\n      등록: {}\n      지금: {}\n    \
         하네스는 등록된 자리를 **그대로 실행 파일로 띄운다**(exec form). 그런데 훅 규약을 \
         확인할 때 실제로 돌려 본 것은 **지금 이 실행 파일**이라, 둘이 다르면 그 확인이 \
         등록된 것에 대해 아무것도 말하지 않는다.\n    \
         `pal update` 가 등록을 지금 실행 파일로 되돌린다",
        path.display(),
        지금.display()
    )
}

/// 두 자리가 **같은 프로그램인가** — 다르면 그 까닭. 같으면 `None`.
///
/// 크기를 먼저 본다. 다르면 그것으로 끝이고, 남이 심어 둔 것은 대개 여기서 갈린다 —
/// **57MB 를 두 번 읽지 않는다.**
fn 다른_점(a: &Path, b: &Path) -> Result<Option<String>> {
    if a == b {
        return Ok(None);
    }
    let 크기 = |p: &Path| -> Result<u64> {
        Ok(std::fs::metadata(p)
            .with_context(|| format!("크기를 못 읽었다: {}", p.display()))?
            .len())
    };
    let (가, 나) = (크기(a)?, 크기(b)?);
    if 가 != 나 {
        return Ok(Some(format!("크기가 다르다 — {가} vs {나} 바이트")));
    }
    // ★ **여는 자리는 종류를 먼저 묻는다** — [`super::guard::읽는다`] 가 그 문이다.
    // 등록된 자리가 FIFO 면 그냥 읽는 것이 **영원히 매달린다.**
    if super::guard::읽는다(a)? != super::guard::읽는다(b)? {
        return Ok(Some("크기는 같은데 내용이 다르다".to_owned()));
    }
    Ok(None)
}

/// **훅 규약이 실제로 서는가** — 지금 도는 이 실행 파일을 **셸 없이** 직접 띄워 본다.
///
/// # ★ 저장소에서 읽은 문자열을 어떤 경로로도 실행하지 않는다
///
/// 옛 탐침은 `settings.json`·매니페스트의 `command` 를 `/bin/sh -c` 로 돌렸다. 그
/// 두 파일은 **남이 커밋해 보내는 파일**이고, *"우리 훅이 아니다"* 라는 판정은 **실행
/// 뒤에** 났다 — 즉 `pal doctor` 한 번이 임의 코드 실행이었다. 그래서 지금은
/// **우리가 아는 것만 실행한다**: [`std::env::current_exe`] 와 우리가 정한 인자.
/// 파일에서 읽은 문자열은 [`registered`]·[`되읽는다`] 의 **대조에만** 쓴다.
///
/// ★ 이제 하네스도 셸을 안 거친다(exec form). 그래서 **이 탐침이 하는 일이 하네스가
/// 하는 일과 같은 모양**이 됐다 — 실행 파일 하나와 인자 둘. 그래도 **경로의 출처는
/// 다르다**: 하네스는 `settings.json` 에서 읽고 우리는 [`std::env::current_exe`] 에서
/// 얻는다. 그 차이가 이 규율의 전부다.
///
/// # 탐침은 무슨 정책이 걸려 있어도 차단을 못 낸다
///
/// `stop_hook_active` 를 **참**으로 보낸다 — 그 한 줄이 정책보다 먼저 서므로, 정책이
/// 갈아끼워져도 이 검사는 안 움직인다.
///
/// # Errors
/// 못 돌리거나, 종료 코드가 0 이 아니거나, 대답에 우리 표식이 없으면.
pub fn probe(event: &str) -> Result<()> {
    let payload = json!({
        "session_id": "pal-doctor-probe",
        "transcript_path": "",
        "cwd": "",
        "hook_event_name": event,
        "stop_hook_active": true,
    })
    .to_string();

    let exe = 실행_파일()?;
    // **인자는 우리가 정한다.** 셸을 안 거치므로 따옴표도 메타문자도 없다.
    let mut child: std::process::Child = std::process::Command::new(&exe)
        .arg("hook")
        .arg(event)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .with_context(|| format!("{} 를 못 돌렸다", exe.display()))?;
    if let Some(mut sink) = child.stdin.take() {
        use std::io::Write;
        // 상대가 표준입력을 안 읽고 죽으면 여기가 깨진 파이프다 — 그것도 대답의 일부다.
        let _ = sink.write_all(payload.as_bytes());
    }
    // ★ **시간 상한을 지고 기다린다.** 훅이 안 돌아오면 `pal doctor` 도 안 돌아왔다 —
    // 우리가 띄우는 자식은 전부 [`super::child`] 를 지난다.
    let out = super::child::기다린다(child, super::child::기본_상한, "훅 탐침")
        .context("훅의 대답을 못 받았다")?;

    let stderr = String::from_utf8_lossy(&out.stderr);
    match out.status.code() {
        Some(0) => {}
        Some(code) => bail!("exit {code} — {}", stderr.trim()),
        None => bail!("신호로 죽었다 — {}", stderr.trim()),
    }
    if !stderr.contains(crate::hook::ACK) {
        bail!("대답에 `{}` 표식이 없다", crate::hook::ACK);
    }
    let 지금 = crate::version::describe();
    if !stderr.contains(지금) {
        bail!("대답이 pal {지금} 을 안 적었다");
    }
    Ok(())
}

// ─────────────────────────────────────────────────────────────────────────────
// 무엇을 더하고 무엇을 뺄 것인가 — **순수 함수. 그래서 시험이 잡는다**
// ─────────────────────────────────────────────────────────────────────────────

/// 더할 것과 뺄 것.
#[derive(Default)]
pub struct Plan {
    /// 아직 안 걸린 것.
    pub add: Vec<HookEntry>,
    /// **우리가 걸었는데 이제 바라지 않는 것** — 실행 파일이 옮겨 갔거나 사건 목록이
    /// 줄었을 때 여기가 찬다. 안 빼면 죽은 등록이 남고, 그 실패는 침묵한다.
    pub remove: Vec<HookEntry>,
}

impl Plan {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.add.is_empty() && self.remove.is_empty()
    }
}

/// 지금 설정 · 우리가 적어 둔 것 · 바라는 것 셋을 대서 계획을 낸다.
#[must_use]
pub fn plan(
    current: Option<&Map<String, Value>>,
    recorded: &[HookEntry],
    desired: &[HookEntry],
) -> Plan {
    let add = desired.iter().filter(|d| !registered(current, d)).cloned().collect();
    let remove = recorded
        .iter()
        .filter(|r| !desired.iter().any(|d| d == *r))
        .filter(|r| registered(current, r))
        .cloned()
        .collect();
    Plan { add, remove }
}

/// 그 항목이 그 사건에 **완전 일치로** 걸려 있는가.
#[must_use]
pub fn registered(current: Option<&Map<String, Value>>, entry: &HookEntry) -> bool {
    let Some(map) = current else { return false };
    let Some(Value::Array(groups)) = map.get(HOOKS).and_then(|h| h.get(&entry.event)) else {
        return false;
    };
    groups.iter().any(|g| {
        g.get(GROUP)
            .and_then(Value::as_array)
            .is_some_and(|cmds| cmds.iter().any(|c| 같은_등록인가(c, entry)))
    })
}

/// 설정 안의 항목 하나가 **우리가 적어 둔 그 항목인가.**
///
/// # 동등성을 어디까지 보는가 — **`command` 와 `args` 둘 다**
///
/// 하네스의 중복 제거는 **`command` 문자열 완전 일치**다. 그런데 exec form 에서 같은
/// `command` 에 다른 `args` 는 **다른 훅**이다. 둘 중 좁은 쪽(`command` 만)으로
/// 판정하면 우리가 안 건 항목을 우리 것으로 읽어 **제거가 남의 것을 걷고**, 넓은
/// 쪽(둘 다)으로 판정하면 최악의 경우 **우리 것을 못 알아보고 하나 더 건다.**
///
/// **넓은 쪽을 골랐다.** 못 알아본 것은 `doctor` 의 검사 여섯이 잡고 화면에 뜨지만,
/// 남의 등록을 걷는 것은 `[f24]` ⑦ 을 무너뜨리고 **조용하다.** 그리고 우리는 사건
/// 하나에 항목 하나만 걸므로 「하나 더 거는」 쪽이 실제로 일어나려면 사용자가 같은
/// 절대 경로를 손으로 다른 인자와 함께 걸어 뒀어야 한다.
///
/// **`args` 의 없음도 값이다** — 옛 형태와 새 형태는 여기서 갈리고, 그래서 `update`
/// 가 옛 것을 빼고 새 것을 걸 수 있다.
fn 같은_등록인가(c: &Value, entry: &HookEntry) -> bool {
    c.get(COMMAND).and_then(Value::as_str) == Some(entry.command.as_str())
        && c.get(ARGS) == 인자_값(entry).as_ref()
}

/// 이 항목의 `args` 를 JSON 으로. 옛 형태면 **없음**이다.
fn 인자_값(entry: &HookEntry) -> Option<Value> {
    entry.args.as_ref().map(|a| json!(a))
}

/// 계획을 설정 지도에 적용한다. **더한 것이 있으면 `hooks` 키를 우리가 만들었는지**를
/// 함께 낸다 — 제거가 그것만 되돌린다.
///
/// # Errors
/// `hooks` 가 객체가 아니거나 사건 자리가 배열이 아니면. **고치려 들지 않는다.**
pub fn apply(map: &mut Map<String, Value>, plan: &Plan) -> Result<bool> {
    for entry in &plan.remove {
        뺀다(map, entry);
    }
    let mut 우리가_만들었나 = false;
    for entry in &plan.add {
        우리가_만들었나 |= 더한다(map, entry)?;
    }
    치운다(map, false);
    Ok(우리가_만들었나)
}

/// 우리가 적어 둔 것을 전부 뺀다 — **제거의 자리.**
///
/// `hooks_key_created` 가 참일 때만 빈 `hooks` 키를 지운다. 사용자가 원래 `"hooks": {}`
/// 를 두었으면 그것은 사용자의 것이다.
pub fn strip(map: &mut Map<String, Value>, recorded: &[HookEntry], hooks_key_created: bool) -> bool {
    let mut 뺐다 = false;
    for entry in recorded {
        뺐다 |= 뺀다(map, entry);
    }
    치운다(map, hooks_key_created);
    뺐다
}

/// 설정에 넣을 항목 하나의 JSON.
///
/// ⚠ **`shell` 키를 안 쓴다.** 실측: enum(`bash`/`powershell`) 밖 값을 넣으면 그 훅
/// 배열 **전체**가 조용히 사라진다. 안 쓰는 키는 안 쓴다.
fn 항목(entry: &HookEntry) -> Value {
    let mut o = Map::new();
    o.insert(KIND.to_owned(), json!(KIND_COMMAND));
    o.insert(COMMAND.to_owned(), json!(entry.command));
    if let Some(args) = 인자_값(entry) {
        o.insert(ARGS.to_owned(), args);
    }
    Value::Object(o)
}

fn 더한다(map: &mut Map<String, Value>, entry: &HookEntry) -> Result<bool> {
    // ⚠ **`args: []` 는 exec form 이고 반드시 죽는다** — 빈 배열이면 `command` 문자열
    // **전체**가 실행 파일 경로가 되어 ENOENT 다. 그리고 그 실패는 기본 채널에서
    // 침묵한다. 우리 코드가 그것을 만들 길은 지금 없지만, **생기면 여기서 멈춘다.**
    if entry.args.as_ref().is_some_and(Vec::is_empty) {
        bail!(
            "`{}` 에 인자가 빈 배열인 항목을 걸려 했다 — 빈 배열도 exec form 이고, \
             그때는 명령 문자열 **전체**가 실행 파일 경로가 되어 죽는다. \
             그리고 그 실패는 침묵한다",
            entry.event
        );
    }
    let 없었다 = !map.contains_key(HOOKS);
    let hooks = map.entry(HOOKS).or_insert_with(|| json!({}));
    let Value::Object(hooks) = hooks else {
        bail!("`{HOOKS}` 가 객체가 아니다 — 남의 구조를 고치려 들지 않는다");
    };
    let event = &entry.event;
    let groups = hooks.entry(event).or_insert_with(|| json!([]));
    let Value::Array(groups) = groups else {
        bail!("`{HOOKS}.{event}` 이 배열이 아니다 — 남의 구조를 고치려 들지 않는다");
    };
    // **우리 묶음 하나를 따로 넣는다.** 남의 묶음에 끼워 넣으면 제거가 남의 것을 건드린다.
    groups.push(json!({ GROUP: [항목(entry)] }));
    Ok(없었다)
}

fn 뺀다(map: &mut Map<String, Value>, entry: &HookEntry) -> bool {
    let Some(Value::Object(hooks)) = map.get_mut(HOOKS) else { return false };
    let Some(Value::Array(groups)) = hooks.get_mut(&entry.event) else { return false };

    let mut 뺐다 = false;
    groups.retain_mut(|g| {
        let Some(Value::Array(cmds)) = g.get_mut(GROUP) else { return true };
        let 전 = cmds.len();
        cmds.retain(|c| !같은_등록인가(c, entry));
        if cmds.len() == 전 {
            return true;
        }
        뺐다 = true;
        // **우리가 비운 묶음만 지운다.** 남의 것이 함께 든 묶음은 남는다.
        !cmds.is_empty()
    });
    if 뺐다 && groups.is_empty() {
        hooks.remove(&entry.event);
    }
    뺐다
}

/// 빈 자리를 치운다. `hooks` 키 자체는 **우리가 만들었을 때만** 지운다.
fn 치운다(map: &mut Map<String, Value>, 우리가_만들었나: bool) {
    if !우리가_만들었나 {
        return;
    }
    if map.get(HOOKS).and_then(Value::as_object).is_some_and(Map::is_empty) {
        map.remove(HOOKS);
    }
}

#[cfg(test)]
mod tests {
    use super::{HookEntry, apply, entry, plan, registered, strip, 되읽는다, 옛_형태인가};
    use serde_json::{Map, Value, json};
    use std::path::Path;

    fn 지도(v: &Value) -> Map<String, Value> {
        v.as_object().expect("객체").clone()
    }

    /// 새 형태 하나.
    fn 하나(exe: &str) -> HookEntry {
        entry(Path::new(exe), "SubagentStop")
    }

    /// **옛 형태 하나** — 이 회차 이전의 설치본이 지고 있는 것.
    fn 옛것(command: &str) -> HookEntry {
        HookEntry { event: "SubagentStop".to_owned(), command: command.to_owned(), args: None }
    }

    fn 바람(exe: &str) -> Vec<HookEntry> {
        vec![하나(exe)]
    }

    /// 적어 둔 것 없이 새로 건다 — 시험마다 두 줄이 되는 자리를 접는다.
    fn 건다(map: &mut Map<String, Value>, 바람: &[HookEntry]) -> bool {
        let p = plan(Some(&*map), &[], 바람);
        apply(map, &p).expect("적용")
    }

    /// 그 사건에 걸린 항목 전부.
    fn 걸린(map: &Map<String, Value>, event: &str) -> Vec<Value> {
        map["hooks"][event]
            .as_array()
            .expect("배열")
            .iter()
            .filter_map(|g| g["hooks"].as_array())
            .flatten()
            .cloned()
            .collect()
    }

    /// ★ **공백·따옴표·`$`·백틱이 든 경로가 그대로 간다** — exec form 은 셸을 안 거치므로
    /// 인용이 필요 없고, **붙이면 오히려 그 글자가 경로의 일부가 된다.**
    #[test]
    fn 인용_없이_경로가_그대로_간다() {
        for 경로 in [
            "/bin/pal",
            "/opt/pal 도구/pal",
            "/opt/it's/pal",
            // ⚠ 셸 메타문자를 여기 그대로 둔다 — exec form 에서는 **전부 리터럴**이다.
            "/opt/$PWD `whoami`/pal",
            "/opt/a;b|c/pal",
            "/한글/경로/pal",
        ] {
            let e = 하나(경로);
            assert_eq!(e.command, 경로, "경로에 무엇이 덧붙었다");
            assert_eq!(e.args.as_deref(), Some(["hook".to_owned(), "SubagentStop".to_owned()].as_slice()));
        }
    }

    /// ★ **설정에 실제로 실리는 모양** — `args` 가 있고 `shell` 키가 없다.
    ///
    /// ⚠ `args: []` 는 exec form 이고 그때는 명령 문자열 **전체**가 실행 파일 경로가
    /// 되어 죽는다. 그래서 **비어 있지 않다**를 여기서 못박는다.
    #[test]
    fn 설정에_실리는_모양이_exec_form_이다() {
        let mut map = Map::new();
        건다(&mut map, &바람("/opt/pal 도구/pal"));
        let 항목들 = 걸린(&map, "SubagentStop");
        assert_eq!(항목들.len(), 1);
        assert_eq!(
            항목들[0],
            json!({
                "type": "command",
                "command": "/opt/pal 도구/pal",
                "args": ["hook", "SubagentStop"],
            })
        );
        assert!(항목들[0].get("shell").is_none(), "`shell` 키를 썼다 — 훅 배열이 통째로 사라진다");
    }

    /// **인자가 빈 배열이면 멈춘다** — 그 항목은 반드시 죽고, 그 실패는 침묵한다.
    #[test]
    fn 빈_인자_배열은_거절한다() {
        let mut map = Map::new();
        let 빈것 = vec![HookEntry {
            event: "SubagentStop".to_owned(),
            command: "/bin/pal hook SubagentStop".to_owned(),
            args: Some(Vec::new()),
        }];
        let p = plan(Some(&map), &[], &빈것);
        assert!(apply(&mut map, &p).is_err(), "빈 인자 배열을 그대로 걸었다");
    }

    /// ★ **우리가 만든 항목만 되읽힌다** — 되읽히지 않는 것은 우리 것이 아니다.
    #[test]
    fn 우리가_만든_항목만_되읽힌다() {
        for 경로 in ["/bin/pal", "/opt/pal 도구/pal", "/opt/it's/pal", "/한글/경로/pal"] {
            let e = 하나(경로);
            assert_eq!(되읽는다(&e).as_deref(), Some(Path::new(경로)), "왕복이 안 됐다: {경로}");
        }
    }

    /// ★ **남이 심은 항목은 우리 형태가 아니다.** 되읽기가 그것을 가른다 —
    /// 그리고 되읽은 경로는 `stat` 에만 쓰이지 실행에는 안 쓰인다.
    ///
    /// exec form 에서 이 줄이 더 세진다: `command` 가 **실행 파일 경로 그 자체**라
    /// 되읽기가 무르면 남이 커밋해 보낸 바이너리를 우리가 띄운다.
    #[test]
    fn 남이_심은_항목은_안_되읽힌다() {
        let 남의것: Vec<HookEntry> = vec![
            // 인자가 없다 — **옛 형태**이지 우리 새 형태가 아니다.
            옛것("'/bin/pal' hook SubagentStop"),
            옛것("touch /tmp/PWNED"),
            // 인자가 우리 것이 아니다.
            인자("/usr/bin/touch", &["/tmp/PWNED"]),
            인자("/bin/sh", &["-c", "touch /tmp/PWNED"]),
            인자("/bin/pal", &["hook", "SessionStart"]),
            인자("/bin/pal", &["hook"]),
            인자("/bin/pal", &["hook", "SubagentStop", "--그리고"]),
            인자("/bin/pal", &[]),
            // 경로가 비었다.
            인자("", &["hook", "SubagentStop"]),
        ];
        for e in &남의것 {
            assert!(되읽는다(e).is_none(), "`{}` 를 우리 것으로 읽었다", e.보임());
        }
    }

    fn 인자(command: &str, args: &[&str]) -> HookEntry {
        HookEntry {
            event: "SubagentStop".to_owned(),
            command: command.to_owned(),
            args: Some(args.iter().map(|s| (*s).to_owned()).collect()),
        }
    }

    /// ★ **옛 형태는 「우리 것이 아니다」가 아니라 「옛 형태」다.** 그 구분이
    /// `update` 의 안내와 `uninstall` 의 걷기를 가른다.
    #[test]
    fn 옛_형태를_종류로_가른다() {
        assert!(옛_형태인가(&옛것("'/bin/pal' hook SubagentStop")));
        assert!(!옛_형태인가(&하나("/bin/pal")));
    }

    /// ★ **항목 동등성은 `command` 와 `args` 둘 다 본다.**
    ///
    /// 같은 경로에 다른 인자가 걸려 있으면 **다른 훅**이다 — 우리 것으로 읽어서
    /// 걷어내면 그것이 곧 남의 등록을 지우는 일이다.
    #[test]
    fn 인자가_다르면_다른_등록이다() {
        let mut map = Map::new();
        건다(&mut map, &바람("/bin/pal"));

        assert!(registered(Some(&map), &하나("/bin/pal")));
        assert!(!registered(Some(&map), &인자("/bin/pal", &["hook", "SessionStart"])));
        assert!(!registered(Some(&map), &옛것("/bin/pal")), "`args` 없는 항목을 같다고 읽었다");
    }

    /// ★ **`update` 가 옛 형태를 빼고 새 형태를 건다.** 안 빼면 같은 훅이 두 번 돌고,
    /// 옛 것은 셸을 거친다.
    #[test]
    fn 갱신이_옛_형태를_빼고_새_형태를_건다() {
        let mut map = Map::new();
        let 옛 = vec![옛것("'/bin/pal' hook SubagentStop")];
        건다(&mut map, &옛);

        let 새 = 바람("/bin/pal");
        let p = plan(Some(&map), &옛, &새);
        assert_eq!(p.add.len(), 1, "새 형태를 안 걸려 한다");
        assert_eq!(p.remove.len(), 1, "옛 형태를 안 빼려 한다");
        apply(&mut map, &p).expect("적용");

        let 항목들 = 걸린(&map, "SubagentStop");
        assert_eq!(항목들.len(), 1, "옛 등록이 남았다: {항목들:?}");
        assert_eq!(항목들[0]["args"], json!(["hook", "SubagentStop"]));
    }

    /// ★ **`uninstall` 이 옛 형태도 걷어낸다.** 매니페스트가 적은 그대로 뺀다.
    #[test]
    fn 제거가_옛_형태도_걷어낸다() {
        let mut map = Map::new();
        let 옛 = vec![옛것("'/bin/pal' hook SubagentStop")];
        let 만들었나 = 건다(&mut map, &옛);
        strip(&mut map, &옛, 만들었나);
        assert!(map.is_empty(), "옛 형태가 남았다: {map:?}");
    }

    /// ★ **같은 설치에서 두 번 등록하면 두 번 돈다** — 중복 제거가 완전 일치 기준이므로
    /// 두 번째 계획은 비어야 한다.
    #[test]
    fn 두_번째_계획은_비어_있다() {
        let mut map = Map::new();
        let 바람 = 바람("/bin/pal");
        let p = plan(Some(&map), &[], &바람);
        assert_eq!(p.add.len(), 1);
        apply(&mut map, &p).expect("적용");

        let p2 = plan(Some(&map), &바람, &바람);
        assert!(p2.is_empty(), "두 번째가 또 등록하려 한다");
    }

    /// **실행 파일이 옮겨 가면 옛 등록을 뺀다** — 안 빼면 죽은 등록이 침묵으로 남는다.
    #[test]
    fn 옮겨_가면_옛_등록을_뺀다() {
        let mut map = Map::new();
        let 옛 = 바람("/옛/pal");
        건다(&mut map, &옛);

        let 새 = 바람("/새/pal");
        let p = plan(Some(&map), &옛, &새);
        assert_eq!(p.add.len(), 1);
        assert_eq!(p.remove.len(), 1);
        apply(&mut map, &p).expect("적용");

        assert!(!registered(Some(&map), &옛[0]));
        assert!(registered(Some(&map), &새[0]));
    }

    /// ★ **남이 같은 사건에 걸어 둔 것을 하나도 안 건드린다.**
    #[test]
    fn 남의_등록은_왕복해도_그대로다() {
        let 남의것 = json!({
            "hooks": {
                "SubagentStop": [{"hooks": [{"type": "command", "command": "남의 것.sh"}]}],
                "SessionStart": [{"hooks": [{"type": "command", "command": "남의 시작.sh"}]}]
            }
        });
        let mut map = 지도(&남의것);
        let 바람 = 바람("/bin/pal");
        건다(&mut map, &바람);
        assert_eq!(map["hooks"]["SubagentStop"].as_array().expect("배열").len(), 2);

        strip(&mut map, &바람, false);
        assert_eq!(Value::Object(map), 남의것, "왕복이 남의 것을 바꿨다");
    }

    /// **우리가 만든 `hooks` 키는 비면 사라진다.** 사용자가 만든 것은 안 사라진다.
    #[test]
    fn 우리가_만든_훅_키만_사라진다() {
        let 바람 = 바람("/bin/pal");

        let mut 우리것 = Map::new();
        let 만들었나 = 건다(&mut 우리것, &바람);
        assert!(만들었나);
        strip(&mut 우리것, &바람, 만들었나);
        assert!(우리것.is_empty(), "우리가 만든 키가 남았다: {우리것:?}");

        let mut 남의것 = 지도(&json!({"hooks": {}}));
        let 만들었나 = 건다(&mut 남의것, &바람);
        assert!(!만들었나, "남이 만든 키를 우리가 만들었다고 적었다");
        strip(&mut 남의것, &바람, 만들었나);
        assert_eq!(Value::Object(남의것), json!({"hooks": {}}));
    }

    /// **남의 구조를 고치려 들지 않는다** — 모양이 다르면 멈춘다.
    #[test]
    fn 모양이_다르면_멈춘다() {
        for 이상한 in [json!({"hooks": "문자열"}), json!({"hooks": {"SubagentStop": 1}})] {
            let mut map = 지도(&이상한);
            let 바람 = 바람("/bin/pal");
            let p = plan(Some(&map), &[], &바람);
            assert!(apply(&mut map, &p).is_err(), "{이상한} 에서 안 멈췄다");
        }
    }

    /// 사건 자리가 비면 그 열쇠도 사라진다 — 빈 배열이 남으면 그것이 곧 잔해다.
    #[test]
    fn 우리만_있던_사건_자리는_통째로_사라진다() {
        let mut map = Map::new();
        let 바람 = 바람("/bin/pal");
        let 만들었나 = 건다(&mut map, &바람);
        strip(&mut map, &바람, 만들었나);
        assert!(map.is_empty());
    }
}
