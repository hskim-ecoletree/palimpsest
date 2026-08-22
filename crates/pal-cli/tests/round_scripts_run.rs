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
        // ★ **파이썬의 I/O 인코딩을 못 박는다.** Windows 러너에서 이 시험이 실제로
        //   죽었다 — 파이프로 들어온 UTF-8 을 로케일로 디코드해 surrogate 가 생겼고
        //   그것을 다시 UTF-8 로 쓸 때 터졌다(실측 2026-08-19 · CI 가 잡았다).
        //   스크립트 자신도 `sys.stdin.reconfigure` 로 박지만, **호출하는 쪽도 말해야**
        //   「양쪽이 할 수 있는 것」이 된다(옛 ADR-0023).
        .env("PYTHONIOENCODING", "utf-8")
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

/// **음성 대조 둘째** — 빈 커밋 범위에서 칸이 **사라지지 않고 「못 셌다」**를 낸다.
///
/// # 왜 (2026-08-22 · 사전부검 R2·R3)
///
/// 2026-08-19 가 조기 return 밖으로 **⑦⑧ 만** 뺐고 **②③④⑤⑥ 은 그대로 삼켜졌다** —
/// 갓 설치한 저장소·잘못된 인자·착수==종료 어디서나 그 다섯 칸이 **아예 안 떴다.**
/// 그리고 위 `계기판이_레코드가_없으면_못_셌다고_말한다` 는 착수로 `HEAD` 를 줘서
/// **언제나 빈 범위**라 그 경로를 한 번도 안 지났다 — 시험이 회귀를 못 봤다.
///
/// 걷기만 하는 것도 안 된다: 그러면 ③④⑤⑥ 이 **거짓 0** 을 낸다. 그래서 **양방향**으로
/// 잰다 — 빈 범위면 「못 셌다」, 안 비면 수.
#[test]
fn 계기판이_빈_범위에서_칸을_안_삼킨다() {
    let repo = 저장소("dashboard-empty-range");
    pal(&repo, &["install"]);

    let 계기판 = ".claude/skills/pal-round/bin/dashboard.py";
    let 회차 = repo.join(".palimpsest/rounds/시험회차");
    std::fs::create_dir_all(&회차).expect("회차 디렉터리");

    // ★ 파서 함정을 함께 심는다 — 진짜 조건은 **셋**(A1 닫힘 · A1-a·A2 열림)이고
    //   코드펜스 안의 예시 둘과 `## 범위 밖` 의 불릿 하나는 **조건이 아니다.**
    let 의도 = 회차.join("intent.md");
    std::fs::write(
        &의도,
        concat!(
            "## 완수 조건\n",
            "- [x] A1 진짜 조건 · 통과\n",
            "  - [ ] A1-a 들여쓴 하위 조건\n",
            "- [ ] A2 진짜 조건 둘\n",
            "\n```markdown\n- [ ] X1 예시다\n- [x] X2 예시다\n```\n\n",
            "## 범위 밖\n",
            "- [ ] 이것도 조건이 아니다\n",
        ),
    )
    .expect("의도 파일");
    let 의도_인자 = ".palimpsest/rounds/시험회차/intent.md";

    // ① 빈 범위 — 다섯 칸이 **뜨고** 「못 셌다」다.
    let (ok, 빈, err) = 돌린다(&repo, &[계기판, "HEAD", 의도_인자, "HEAD"]);
    assert!(ok, "계기판이 안 돈다:\n{err}");
    for 칸 in ["③ 진자", "④ 연쇄 깊이", "⑤ 라운드", "⑥ 승격 횟수"] {
        assert!(빈.contains(칸), "빈 범위에서 `{칸}` 이 사라졌다:\n{빈}");
    }
    for 칸 in ["③ 진자 (P1)     — **못 셌다**", "④ 연쇄 깊이      — **못 셌다**"] {
        assert!(빈.contains(칸), "빈 범위에서 0 을 냈다 — 「못 셌다」여야 한다:\n{빈}");
    }
    // ⑦⑧ 은 **한 번만** 난다 — 앞 판은 빈-범위 분기와 정상 분기에서 두 번 불렀다.
    assert_eq!(
        빈.matches("⑦ 원 의도 비율").count(),
        1,
        "⑦ 이 중복 출력됐다:\n{빈}"
    );

    // ② 파서 — 펜스 안과 `## 범위 밖` 을 안 세고, 들여쓴 상자는 센다.
    assert!(
        빈.contains("② 미판정 잔액    2 / 3"),
        "조건 셋(닫힘 1·열림 2)이어야 한다 — 펜스·범위 밖을 세거나 들여쓰기를 놓쳤다:\n{빈}"
    );

    // ③ 안 빈 범위 — 같은 칸이 **수**를 낸다. 한 방향만 재면 「언제나 못 셌다」를 못 가른다.
    let git = |args: &[&str]| {
        let st = Command::new("git").args(args).current_dir(&repo).status().expect("git");
        assert!(st.success(), "git {args:?} 실패");
    };
    git(&["add", "-A"]);
    git(&["-c", "user.email=t@t", "-c", "user.name=t", "commit", "-q", "-m", "시험 커밋"]);
    let (ok, 찬, err) = 돌린다(&repo, &[계기판, "HEAD~1", 의도_인자, "HEAD"]);
    assert!(ok, "계기판이 안 돈다:\n{err}");
    assert!(
        !찬.contains("③ 진자 (P1)     — **못 셌다**"),
        "범위가 안 비었는데 「못 셌다」다 — 이 칸은 죽은 가지다:\n{찬}"
    );
}
