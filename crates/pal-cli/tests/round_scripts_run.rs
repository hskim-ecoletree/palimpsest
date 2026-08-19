//! **설치본에 실린 회차 스크립트가 실제로 도는가** — 그리고 안 돌면 그것이 드러나는가.
//!
//! # 왜 이 시험이 생겼나 (2026-08-19 · 사전부검)
//!
//! `dashboard.py` 는 `install/layout.rs` 의 세 줄 말고 **저장소 어디에서도 안 불렸다** —
//! 시험 0 · CI 잡 0. 즉 설치본에 실린 스크립트가 **한 번도 실행된 적 없이** 「완료」로
//! 잡혀 있었다. 그리고 실제로 돌려 보면 **안 돈다**: `install.rs` 의 `guard::쓴다` 는
//! 바이트만 쓰고 모드를 안 세워 파일이 `0644` 로 놓이는데, 규약은 그것을 **직접
//! 실행**하라고 적고 있었다.
//!
//! ★ **고친 축은 호출 형태다.** 모드를 세우는 길은 Windows 에서 안 풀린다(옛 ADR-0023 이
//! 가른 대로 고를 축은 「볼 수 있는 쪽」이 아니라 **양쪽이 할 수 있는 것**이다). 그래서
//! 규약과 docstring 이 전부 `python3 <경로>` 로 부르고, **이 시험이 그 형태를 잰다.**
//!
//! # 음성 대조 — 이 시험이 고장이면 이렇게 드러난다
//!
//! `계기판이_레코드가_없으면_못_셌다고_말한다` 가 그 자리다. 갓 설치한 프로젝트에는
//! `.palimpsest/` 가 **없어서** ⑦⑧ 이 언제나 「못 셌다」인데, 그것이 **정상인지 고장인지
//! 가릴 장치가 없으면** 경로가 틀렸든 파서가 깨졌든 영영 같은 화면이다. 그래서 레코드를
//! **놓기 전과 놓은 뒤**를 둘 다 재고, 놓은 뒤에는 **수가 나와야** 한다.

mod common;

use std::path::Path;
use std::process::Command;

use common::{pal, 저장소};

/// 파이썬 실행자 — 이름은 플랫폼이 정한다. **한 자리에서 정한다.**
fn 파이썬() -> &'static str {
    for 이름 in ["python3", "python"] {
        let ok = Command::new(이름)
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);
        if ok {
            return 이름;
        }
    }
    panic!("파이썬 실행자를 못 찾았다 — `python3` 도 `python` 도 안 선다");
}

fn 돌린다(cwd: &Path, args: &[&str]) -> (bool, String, String) {
    let out = Command::new(파이썬())
        .args(args)
        .current_dir(cwd)
        .output()
        .expect("파이썬을 못 띄웠다");
    (
        out.status.success(),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

/// ★ **설치본의 쓰는 자가 실제로 돈다.**
#[test]
fn 설치본의_record_가_스키마를_낸다() {
    let repo = 저장소("record-run");
    pal(&repo, &["install"]);

    let 경로 = ".claude/skills/pal-round/bin/record.py";
    assert!(repo.join(경로).is_file(), "설치가 `{경로}` 를 안 놓았다");

    let (ok, out, err) = 돌린다(&repo, &[경로, "--schema"]);
    assert!(ok, "설치본의 `record.py` 가 안 돈다:\n{err}");
    // 스키마가 실제로 서는지 — 문자열이 아니라 **내용**을 본다.
    for 낱말 in ["schema_version", "필수", "enum", "대응표", "합계검산"] {
        assert!(out.contains(낱말), "`--schema` 출력에 `{낱말}` 이 없다:\n{out}");
    }
}

/// ★ **설치본의 읽는 자도 실제로 돈다.**
#[test]
fn 설치본의_계기판이_돈다() {
    let repo = 저장소("dashboard-run");
    pal(&repo, &["install"]);

    let 경로 = ".claude/skills/pal-round/bin/dashboard.py";
    assert!(repo.join(경로).is_file(), "설치가 `{경로}` 를 안 놓았다");

    let (ok, out, err) = 돌린다(&repo, &[경로, "HEAD"]);
    assert!(ok, "설치본의 `dashboard.py` 가 안 돈다:\n{err}");
    assert!(out.contains("계기판"), "계기판 화면이 아니다:\n{out}");
}

/// **음성 대조** — 「못 셌다」가 정상인지 고장인지 가린다.
///
/// 갓 설치한 프로젝트에는 `.palimpsest/` 가 없다. 그 상태에서 ⑦⑧ 은 **「못 셌다」**여야
/// 하고, 레코드를 놓으면 **수가 나와야** 한다. 둘 다 재야 이 칸이 살아 있는지 안다 —
/// 하나만 재면 「언제나 못 셌다」인 고장을 정상으로 읽는다.
#[test]
fn 계기판이_레코드가_없으면_못_셌다고_말한다() {
    let repo = 저장소("dashboard-negctl");
    pal(&repo, &["install"]);

    let 계기판 = ".claude/skills/pal-round/bin/dashboard.py";
    let 레코드 = ".claude/skills/pal-round/bin/record.py";
    let 회차 = repo.join(".palimpsest/rounds/시험회차");
    std::fs::create_dir_all(&회차).expect("회차 디렉터리");
    let 의도 = 회차.join("intent.md");
    std::fs::write(&의도, "## 완수 조건\n- [ ] **X-a** 아무거나\n").expect("의도 파일");
    let 의도_인자 = ".palimpsest/rounds/시험회차/intent.md";

    // ① 레코드가 없을 때 — **「못 셌다」**여야 한다. 0 이면 안 된다.
    let (ok, 전, err) = 돌린다(&repo, &[계기판, "HEAD", 의도_인자]);
    assert!(ok, "계기판이 안 돈다:\n{err}");
    assert!(
        전.contains("못 셌다"),
        "레코드가 없는데 「못 셌다」가 아니다 — 0 으로 말하면 거짓 신호다:\n{전}"
    );

    // ② 레코드를 놓으면 — **수가 나와야** 한다.
    let 한줄 = r#"{"id":"T-1","라운드":1,"출처":"실측","모집단":"원의도","유효성":"참","해악도":"미관","처분":"정정","경로":"CLAUDE.md","요약":"시험용 한 줄"}"#;
    let mut 자식 = Command::new(파이썬())
        .args([레코드, "add", ".palimpsest/rounds/시험회차"])
        .current_dir(&repo)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("record.py 를 못 띄웠다");
    {
        use std::io::Write;
        자식.stdin.as_mut().expect("stdin").write_all(한줄.as_bytes()).expect("stdin 쓰기");
    }
    let 결과 = 자식.wait_with_output().expect("record.py 종료");
    assert!(
        결과.status.success(),
        "설치본의 `record.py add` 가 실패했다:\n{}",
        String::from_utf8_lossy(&결과.stderr)
    );

    let (ok, 후, err) = 돌린다(&repo, &[계기판, "HEAD", 의도_인자]);
    assert!(ok, "계기판이 안 돈다:\n{err}");
    assert!(
        !후.contains("⑦ 원 의도 비율    — **못 셌다**"),
        "레코드를 놓았는데도 「못 셌다」다 — 이 칸은 죽은 가지다:\n{후}"
    );
    assert!(
        후.contains("⑦ 원 의도 비율") && 후.contains("1/1"),
        "레코드를 놓았는데 수가 안 나온다:\n{후}"
    );
}
