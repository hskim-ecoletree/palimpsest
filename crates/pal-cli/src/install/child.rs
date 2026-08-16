//! **우리가 띄우는 자식 프로세스** — 시간 상한을 지고 돈다.
//!
//! # 왜 문이 둘인가
//!
//! *"우리가 읽는 자리는 일반 파일이거나 없거나 둘 중 하나다"*([`super::guard`])는
//! **우리 대신 읽는 프로세스에도** 선다. 그런데 그 규율을 세우려면 **그 프로세스가
//! 무엇을 읽는지 목록을 알아야** 하고, 목록은 언제나 불완전하다 — `git` 이 읽는
//! 자리만 해도 뿌리 `.gitignore` · 중첩 `.gitignore` · `.git/info/exclude` ·
//! 전역 `core.excludesFile` · `.git/config` 다섯이고, 마지막 둘은 대상 **밖**에
//! 살 수 있어서 우리 경계 안에서는 볼 수조차 없다.
//!
//! 그래서 문을 둘 세운다:
//!
//! | 문 | 무엇을 막나 | 한계 |
//! |---|---|---|
//! | 목록을 훑어 종류를 묻는다([`super::ignore`]) | **아는 자리**의 FIFO·장치·소켓 | 목록 밖은 못 본다 |
//! | **여기 — 시간 상한** | 어떤 까닭이든 **안 돌아오는 것** | 상한만큼은 기다린다 |
//!
//! 둘째가 없으면 목록에 한 칸이 빌 때마다 사용자 프로젝트가 **영영 매달린다.**
//! 첫째가 없으면 아는 고장에도 상한만큼 서 있게 된다. **둘 다 둔다.**
//!
//! # 파이프를 실로 비운다
//!
//! [`std::process::Child::try_wait`] 로 폴링하면서 파이프를 안 비우면, 자식이 파이프
//! 버퍼를 채운 채 **쓰기에서 막히고** 우리는 그것을 「안 끝났다」로 읽는다 — 상한이
//! **거짓 시간 초과**를 낸다. 그래서 stdout·stderr 를 각각 실 하나가 끝까지 읽는다.

use std::io::Read;
use std::process::{Child, ExitStatus};
use std::time::{Duration, Instant};

use anyhow::{Result, bail};

/// 자식 하나에 주는 시간. **로컬 `git` 한 번은 밀리초 단위다** — 이 값은 정상 동작을
/// 자르지 않을 만큼 넉넉하고, 매달림을 사람의 인내 안에서 끊을 만큼 짧다.
pub const 기본_상한: Duration = Duration::from_secs(30);

/// 폴링 간격. 1ms 면 빠른 `git` 한 번에 붙는 지연이 무시할 만하다.
const 간격: Duration = Duration::from_millis(1);

/// 자식이 낸 것 — [`std::process::Output`] 과 같은 모양이되 **우리가 상한을 걸고**
/// 모은 것이다.
pub struct 대답 {
    pub status: ExitStatus,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

/// 자식을 **상한 안에서** 기다린다. 넘기면 죽이고 그 사실을 말한다.
///
/// # Errors
/// 상한을 넘겨 죽였거나, 기다리는 데 실패하면.
pub fn 기다린다(mut child: Child, 상한: Duration, 무엇: &str) -> Result<대답> {
    let out = child.stdout.take().map(비운다);
    let err = child.stderr.take().map(비운다);

    let 시작 = Instant::now();
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break Some(status),
            Ok(None) if 시작.elapsed() >= 상한 => {
                let _ = child.kill();
                let _ = child.wait();
                break None;
            }
            Ok(None) => std::thread::sleep(간격),
            Err(e) => {
                let _ = child.kill();
                let _ = child.wait();
                return Err(e).map_err(|e| anyhow::anyhow!("{무엇} 을 기다리지 못했다 — {e}"));
            }
        }
    };

    let stdout = out.map(거둔다).unwrap_or_default();
    let stderr = err.map(거둔다).unwrap_or_default();
    let Some(status) = status else {
        bail!(
            "{무엇} 이 {}초 안에 안 끝났다 — **죽였다.**\n    \
             우리가 부른 프로그램이 매달리는 자리가 있다. 흔한 까닭은 그 프로그램이 \
             읽는 파일 중 하나가 **일반 파일이 아닌 것**(이름 있는 파이프·장치·소켓)인 \
             경우다. 사람이 봐야 한다",
            상한.as_secs()
        );
    };
    Ok(대답 { status, stdout, stderr })
}

fn 비운다<R: Read + Send + 'static>(mut r: R) -> std::thread::JoinHandle<Vec<u8>> {
    std::thread::spawn(move || {
        let mut buf = Vec::new();
        let _ = r.read_to_end(&mut buf);
        buf
    })
}

fn 거둔다(h: std::thread::JoinHandle<Vec<u8>>) -> Vec<u8> {
    h.join().unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::{기다린다, 기본_상한};
    use std::process::{Command, Stdio};
    use std::time::{Duration, Instant};

    /// ★ **안 돌아오는 자식을 끊는다.**
    ///
    /// `git hash-object --stdin` 은 표준입력이 닫힐 때까지 **영원히** 읽는다. 파이프를
    /// 열어 두고 아무것도 안 쓰면 그것이 곧 매달리는 자식이다 — 플랫폼을 안 가리는
    /// 형태이고, `git` 은 이 시험이 이미 요구하는 프로그램이다.
    #[test]
    fn 상한을_넘긴_자식은_죽인다() {
        let child = Command::new("git")
            .args(["hash-object", "--stdin"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("git 을 못 돌렸다");
        let t0 = Instant::now();
        let e = 기다린다(child, Duration::from_millis(300), "git hash-object")
            .expect_err("매달린 자식이 그냥 끝났다");
        assert!(t0.elapsed() < Duration::from_secs(5), "상한을 안 지켰다: {:?}", t0.elapsed());
        assert!(format!("{e}").contains("안 끝났다"), "까닭을 안 적었다: {e}");
    }

    /// **평범한 자식은 그대로 지나간다** — 상한이 정상 동작을 자르지 않는다.
    #[test]
    fn 끝나는_자식은_출력을_그대로_낸다() {
        let child = Command::new("git")
            .args(["--version"])
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("git 을 못 돌렸다");
        let 대답 = 기다린다(child, 기본_상한, "git --version").expect("끝나야 한다");
        assert!(대답.status.success());
        assert!(
            String::from_utf8_lossy(&대답.stdout).contains("git version"),
            "출력을 못 거뒀다"
        );
    }
}
