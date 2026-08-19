//! 대상 프로젝트에 무엇이 어디에 놓이는가 — **한 곳에서만 적는다.**
//!
//! # 왜 이름이 `pal-orchestrator` 이고 커맨드가 `commands/pal/` 아래인가
//!
//! **실측이 정했다**: 에이전트는 **프로젝트가 홈을 이기고**, 슬래시 커맨드는 **홈이
//! 프로젝트를 이긴다.** 그래서 에이전트는 이름을 갈라 남의 동명 에이전트를 안 덮고,
//! 커맨드는 디렉터리로 `pal:` 네임스페이스를 만든다.
//!
//! # 규율을 `AGENTS.md` 에 안 담는다
//!
//! **자동 주입되지 않는다**(실측). `CLAUDE.md` 의 `@` 임포트 한 줄이 우리 파일을
//! 끌어오고, 그 파일 하나가 지시의 단일 진실이다.
//!
//! # 여기 적힌 이름 목록은 **놓을 것**이지 **셀 것**이 아니다
//!
//! F04 게이트가 같은 자리에서 적었다 — *"이름으로 적으면 다음에 생기는 파일이 빠지고,
//! 낡은 검사는 통과한다."* 그래서 **검증은 이 목록을 안 쓴다.** 매니페스트가 지는
//! 뿌리를 훑어서 집합을 뜬다([`crate::install::manifest::walk`]).

/// 대상에 놓는 파일 하나 — 상대 경로와 본문.
pub struct Resource {
    pub path: &'static str,
    pub body: &'static str,
}

/// 우리가 **통째로 소유하는** 파일들.
pub const PAYLOAD: &[Resource] = &[
    Resource {
        path: ".claude/pal/INSTRUCTIONS.md",
        body: include_str!("../../assets/INSTRUCTIONS.md"),
    },
    Resource {
        path: ".claude/agents/pal-orchestrator.md",
        body: include_str!("../../assets/pal-orchestrator.md"),
    },
    Resource {
        path: ".claude/commands/pal/touch.md",
        body: include_str!("../../assets/commands/touch.md"),
    },
    Resource {
        path: ".claude/commands/pal/plan.md",
        body: include_str!("../../assets/commands/plan.md"),
    },
    Resource {
        path: ".claude/commands/pal/doctor.md",
        body: include_str!("../../assets/commands/doctor.md"),
    },
    // ★ **2026-08-18 에 `surface/claude-plugin/` 에서 승계했다.** 그 디렉터리는 코드가
    // 한 번도 안 읽는 죽은 표면이었는데(설치는 여기 `assets/` 를 `include_str!` 한다),
    // 그 안의 `ledger.md` 만 **여기 짝이 없었다.** 담고 있는 문장이 이 제품의 것이다 —
    // *"`unsupported`·`unrecognized` 는 「그 파일에 아무 일도 없다」가 아니라
    // **「우리가 안 봤다」**"*. 지우면 그 문장이 코드 어디에도 안 남는다(실측: 0 건).
    Resource {
        path: ".claude/commands/pal/ledger.md",
        body: include_str!("../../assets/commands/ledger.md"),
    },
    // ── 회차 규약 ── (2026-08-19 · [#70](https://github.com/hskim-ecoletree/palimpsest/issues/70))
    //
    // ★ **사본을 만들지 않는다.** 아래 셋은 `assets/` 가 아니라 **이 저장소가 실제로 도는
    // 파일**을 그대로 싣는다. 사본을 두면 갈리고, 두 벌을 대는 검사가 없다 — 그것이 직전
    // 회차가 15 라운드에서 잡은 *"검사의 모집단을 손으로 베낀 거울이 갈렸다"* 와 같은 종이다.
    //
    // ⚠ **전제: 워크스페이스가 `publish = false` 다.** 크레이트 경계를 넘는
    // `include_str!` 는 **빌드는 되지만 `cargo package` 를 깨뜨린다**(실측 2026-08-19).
    // 그 전제가 깨지면 여기가 먼저 무너지므로 시험이 그것을 지킨다
    // ([`crate::install::layout::tests::발행하지_않는다는_전제`]).
    Resource {
        path: ".claude/skills/pal-round/SKILL.md",
        body: include_str!("../../../../.claude/skills/round/SKILL.md"),
    },
    Resource {
        path: ".claude/agents/pal-premortem-sweeper.md",
        body: include_str!("../../../../.claude/agents/pal-premortem-sweeper.md"),
    },
    Resource {
        path: ".claude/agents/pal-independent-reviewer.md",
        body: include_str!("../../../../.claude/agents/pal-independent-reviewer.md"),
    },
    // 계기판 — 규약 §5 「검증 → 수정 착수」 자리가 이것을 부른다.
    // ★ **스킬과 함께 놓아야 한다.** 규약만 놓고 이것을 안 놓으면 설치본의 그 자리가
    // **태어나면서 죽은 가지**가 된다(실측 2026-08-19 · 독립 리뷰 2 라운드가 잡았다).
    Resource {
        path: ".claude/skills/pal-round/bin/dashboard.py",
        body: include_str!("../../../../.claude/skills/round/bin/dashboard.py"),
    },
    // 발견 레코드를 쓰는 자 — 규약 §7 「발견을 레코드에 적는다」가 이것을 부른다.
    // ★ **계기판과 같은 사유로 함께 놓는다.** 규약이 부르는데 물건이 없으면 그 자리가
    // 태어나면서 죽은 가지다. 그리고 이것은 **읽는 자가 아니라 쓰는 자**라 없으면
    // 레코드가 아예 안 쌓인다 — 설치본에는 `xtask` 가 없어 비었다는 것을 말해 줄
    // 하한도 없다.
    Resource {
        path: ".claude/skills/pal-round/bin/record.py",
        body: include_str!("../../../../.claude/skills/round/bin/record.py"),
    },
];

// ★ **정책 금지역 파일은 놓지 않는다.** (2026-08-19 · 독립 리뷰 2 라운드)
//
// 한 번 `assets/policy.toml` 을 씨앗으로 놓았다가 걷었다. 까닭이 둘이다 —
//
// ① **아무도 안 읽었다.** 놓기만 하고 규약이 그것을 가리키지 않아, 사용자가 고쳐도
//    아무 판정도 안 바뀌었다. 소유자가 든 금지역 「측정이 죽은 가지가 됨」이다.
// ② **읽게 만들면 `uninstall` 이 사용자의 편집을 지운다.** `파일_하나_걷기` 는
//    `Origin` 을 안 보고 지우고, 그것이 옳다 — 게이트 ⑥ 이 `S2 == S0` 을 요구하고
//    설치 전에 그 파일은 **없었다.** 우리가 소유한 자리에 **사람이 고치는 것이 정상인
//    내용**을 두는 것 자체가 모순이다.
//
// **그래서 규약이 「있으면 읽는다」로 적는다.** 파일은 사용자가 만들고 우리는 안 만든다.
// 없으면 규약의 기본 목록을 쓴다. 그러면 지울 것이 없고 죽은 가지도 안 생긴다.

/// 매니페스트의 자리.
pub const MANIFEST: &str = ".claude/pal/manifest.json";

/// **`PATH` 에서 우리를 부르는 이름.** 확장자는 안 적는다 — 붙이는 규칙은 플랫폼이
/// 정하고 그 결정은 [`super::exe`] 한 자리에 산다.
///
/// 이 이름을 지나는 자리 둘: `doctor` 검사 4 의 `PATH` 탐색과, **훅에 등록하는
/// 문자열**([`super::hooks::entry`]). 둘이 같은 이름을 써야 *"검사 4 가 초록이면 훅이
/// 뜬다"* 가 성립한다.
pub const COMMAND_NAME: &str = "pal";

/// 하네스가 프로젝트 설정을 두는 디렉터리 — **잠금도 여기 산다.**
pub const CLAUDE_DIR: &str = ".claude";

/// 잠금 디렉터리의 자리.
pub const LOCK: &str = ".claude/.pal.lock";

/// 대상 설정 파일의 자리.
///
/// ⚠ **실측**: 이 파일은 **cwd 한 곳만** 탐색되고 상위로 안 올라간다.
/// `include`·`extends`·`settings.d` 는 이 빌드에 없다.
pub const SETTINGS: &str = ".claude/settings.json";

/// 지시를 끌어오는 파일.
pub const ROOT_INSTRUCTION_FILE: &str = "CLAUDE.md";

/// 파생 경로를 등재하는 파일.
pub const IGNORE_FILE: &str = ".gitignore";

/// **통째로 소유하는 디렉터리 뿌리** — 검증은 여기를 재귀로 훑는다.
pub const OWNED_DIRS: &[&str] = &[".claude/pal", ".claude/commands/pal"];

/// **파일 하나짜리 뿌리** — 그 디렉터리에는 남의 것이 함께 산다.
///
/// ★ **이 목록이 곧 「되돌릴 수 있는 것」의 상한이다**([`super::manifest::우리_자리인가`]).
/// `PAYLOAD` 에 더하면서 여기 안 더하면 **놓기는 하는데 `update`·`uninstall` 이 거부한다** —
/// 실측 2026-08-19: 회차 규약 넷을 `PAYLOAD` 에만 더했더니 설치 뒤 **되돌릴 수 없는
/// 상태**가 됐다. 두 목록은 함께 움직인다.
pub const OWNED_FILES: &[&str] = &[
    ".claude/agents/pal-orchestrator.md",
    // 회차 규약 — `PAYLOAD` 의 짝이다.
    ".claude/agents/pal-premortem-sweeper.md",
    ".claude/agents/pal-independent-reviewer.md",
    ".claude/skills/pal-round/SKILL.md",
    ".claude/skills/pal-round/bin/dashboard.py",
    ".claude/skills/pal-round/bin/record.py",
];

/// 우리가 만들 수 있는 디렉터리 — **만든 것만 매니페스트에 적히고, 제거는 그것만
/// 되돌린다.**
pub const DIRS: &[&str] = &[
    ".claude",
    ".claude/pal",
    ".claude/agents",
    ".claude/commands",
    ".claude/commands/pal",
    // 회차 규약이 사는 자리. `.claude/skills` 에는 남의 스킬이 함께 살고,
    // `pal-round` 는 우리 것이라 이름을 갈랐다.
    ".claude/skills",
    ".claude/skills/pal-round",
    ".claude/skills/pal-round/bin",
];

/// **매니페스트가 사는 집** — 나머지보다 먼저 세운다.
///
/// 기록이 살 자리가 없으면 기록을 못 쓰고, 그 사이에 죽으면 **기록 없는 잔해**가
/// 남는다. 그 창을 이 둘로 줄인다(`[f24]` ② · 부분 설치).
pub const MANIFEST_HOME: &[&str] = &[".claude", ".claude/pal"];

/// `.gitignore` 에 등재할 파생 경로.
///
/// stack §7 이 가른 셋이다 — `cache/`·`index.redb`·`intent.redb` 는 파생이고
/// `intent/*.jsonl` 은 정본이다. **디렉터리째 무시하면 정본까지 사라진다.**
pub const DERIVED: &[&str] =
    &[".palimpsest/cache/", ".palimpsest/index.redb", ".palimpsest/intent.redb"];

/// 블록의 여는 표식과 닫는 표식 — 파일 종류마다 주석 문법이 다르다.
pub struct Markers {
    pub begin: &'static str,
    pub end: &'static str,
}

/// 마크다운 — HTML 주석.
pub const MD_MARKERS: Markers = Markers {
    begin: "<!-- pal:begin — palimpsest. `pal uninstall` 이 이 블록을 걷어낸다 -->",
    end: "<!-- pal:end -->",
};

/// `.gitignore` — `#` 주석.
pub const IGNORE_MARKERS: Markers = Markers {
    begin: "# pal:begin — palimpsest. `pal uninstall` 이 이 블록을 걷어낸다",
    end: "# pal:end",
};

/// `CLAUDE.md` 에 넣는 한 줄. **`@` 임포트가 실제로 파일을 끌어온다**(실측).
pub const IMPORT_LINE: &str = "@.claude/pal/INSTRUCTIONS.md";

/// 설정에 더하는 최상위 키.
pub const AGENT_KEY: &str = "agent";

/// 그 키의 값.
pub const AGENT_VALUE: &str = "pal-orchestrator";

// ─────────────────────────────────────────────────────────────────────────────
// ★ **우리가 놓을 수 있는 자리** — 되돌리기의 상한
// ─────────────────────────────────────────────────────────────────────────────
//
// 매니페스트는 **대상 프로젝트 안에 사는 파일**이고 `.gitignore` 에 없어서 커밋되고
// clone 과 함께 이동한다. 그 안의 경로는 **입력이지 사실이 아니다** — 악성 PR 하나가
// `files: [".git/config"]` 을 적어 두면 `pal uninstall` 한 번이 저장소를 부순다
// (실측: `.git/config` 와 `README.md` 를 각각 지웠고 **rc=0** 이었다).
//
// 그래서 상한을 **매니페스트가 아니라 여기 컴파일된 상수**로 잡는다. 매니페스트의
// `roots` 를 상한으로 쓰면 그것도 남이 쓴 값이라 상한이 아니다.
//
// ⚠ **경로 구분자 가정**: [`crate::install::inside::Rel`] 은 언제나 `/` 로 갈린다 —
// 여기 상수도 그렇고, 실물을 훑어 만드는 자리도 `\` 를 `/` 로 바꾼다
// ([`crate::install::manifest::walk`]). Windows 에서도 이 규칙 하나만 선다.

/// 첫 조각이 이것이면 **무슨 종류로 적혔든** 안 건드린다.
///
/// 위 목록들이 이미 `.git/` 을 안 덮지만, *"최소한 `.git/` 은 어떤 경우에도"* 를
/// 목록의 부수효과로 두지 않고 **따로 못박는다.** 목록이 자라도 이 줄은 안 움직인다.
pub const 절대_금지: &[&str] = &[".git"];

/// 그 경로의 첫 조각.
fn 첫_조각(rel: &str) -> &str {
    rel.split('/').next().unwrap_or(rel)
}

/// **우리가 놓을 수 있는 파일**인가 — 통째로 소유하는 디렉터리 아래이거나,
/// 남의 것이 함께 사는 곳의 **그 파일 하나**.
#[must_use]
pub fn 놓을_수_있는_파일인가(rel: &str) -> bool {
    OWNED_FILES.contains(&rel)
        || OWNED_DIRS.iter().any(|d| rel.strip_prefix(d).is_some_and(|r| r.starts_with('/')))
}

/// **우리가 만들 수 있는 디렉터리**인가.
#[must_use]
pub fn 만들_수_있는_디렉터리인가(rel: &str) -> bool {
    DIRS.contains(&rel)
}

/// **우리가 블록을 넣을 수 있는 남의 파일**인가.
#[must_use]
pub fn 블록을_넣을_수_있는_파일인가(rel: &str) -> bool {
    rel == ROOT_INSTRUCTION_FILE || rel == IGNORE_FILE
}

/// **대상 설정 파일**인가.
#[must_use]
pub fn 설정_파일인가(rel: &str) -> bool {
    rel == SETTINGS
}

/// 어떤 종류로도 못 건드리는 자리인가.
#[must_use]
pub fn 절대_안_건드리나(rel: &str) -> bool {
    절대_금지.contains(&첫_조각(rel))
}

/// 등록할 훅 사건.
///
/// ★ **판정하는 목록을 그대로 쓴다.** 등록하는 자리와 판정하는 자리가 갈리면
/// 「등록만 남고 판정이 사라진 훅」이 조용히 돌고, 실측상 그 헛돎은 **어디에도 안
/// 보인다** — 훅 실행은 트랜스크립트에도 화면에도 흔적을 안 남긴다.
pub const HOOK_EVENTS: &[&str] = crate::hook::EVENTS;

#[cfg(test)]
mod tests {
    use super::*;

    /// **놓는 목록과 되돌리는 목록은 함께 움직인다.**
    ///
    /// 실측 2026-08-19: 회차 규약 넷을 [`PAYLOAD`] 에만 더했더니 `install` 은 놓는데
    /// `update`·`uninstall` 이 *"우리가 놓을 수 있는 자리가 아니다"* 로 **거부**했다 —
    /// 설치 뒤 **되돌릴 수 없는 상태**가 됐다. 이 시험이 그 자리를 지킨다.
    #[test]
    fn 놓는_것은_전부_되돌릴_수_있다() {
        for r in PAYLOAD {
            assert!(
                놓을_수_있는_파일인가(r.path),
                "`{}` 을 PAYLOAD 가 놓는데 `놓을_수_있는_파일인가` 가 거부한다 — \
                 `OWNED_FILES` 나 `OWNED_DIRS` 에 짝이 없다. \
                 그러면 설치는 되고 제거는 안 된다",
                r.path
            );
        }
    }

    /// **놓는 파일의 부모 디렉터리는 전부 우리가 만들 수 있어야 한다.**
    ///
    /// 못 만들면 설치가 그 자리에서 멈추거나, 만들고도 매니페스트에 못 적어
    /// **제거가 빈 디렉터리를 남긴다.**
    #[test]
    fn 놓는_것의_부모를_전부_만들_수_있다() {
        for r in PAYLOAD {
            let mut 조각: Vec<&str> = r.path.split('/').collect();
            조각.pop();
            let mut 쌓임 = String::new();
            for c in 조각 {
                if !쌓임.is_empty() {
                    쌓임.push('/');
                }
                쌓임.push_str(c);
                assert!(
                    만들_수_있는_디렉터리인가(&쌓임),
                    "`{}` 을 놓으려면 `{}` 를 만들어야 하는데 `DIRS` 에 없다",
                    r.path,
                    쌓임
                );
            }
        }
    }

    /// **크레이트 경계를 넘는 `include_str!` 는 `publish = false` 를 전제한다.**
    ///
    /// 회차 규약 셋(`pal-round/SKILL.md`·에이전트 둘)은 사본을 안 만들려고
    /// `../../../../.claude/…` 를 싣는다. 그것은 **빌드는 되지만 `cargo package` 를
    /// 깨뜨린다**(실측 2026-08-19, 격리 사본에서 확인).
    ///
    /// 그 전제가 깨지면 **여기가 먼저 무너지므로** 워크스페이스 선언을 직접 읽어 지킨다.
    #[test]
    fn 발행하지_않는다는_전제() {
        let ws = include_str!("../../../../Cargo.toml");
        assert!(
            ws.contains("publish = false"),
            "워크스페이스가 `publish = false` 가 아니다 — 그런데 `layout.rs` 가 \
             크레이트 밖 파일을 `include_str!` 한다. `cargo package` 가 깨진다. \
             사본을 만들 것인가 발행을 안 할 것인가를 사람이 정해야 한다"
        );
    }
}
