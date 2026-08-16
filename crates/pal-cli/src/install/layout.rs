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
];

/// 매니페스트의 자리.
pub const MANIFEST: &str = ".claude/pal/manifest.json";

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
pub const OWNED_FILES: &[&str] = &[".claude/agents/pal-orchestrator.md"];

/// 우리가 만들 수 있는 디렉터리 — **만든 것만 매니페스트에 적히고, 제거는 그것만
/// 되돌린다.**
pub const DIRS: &[&str] =
    &[".claude", ".claude/pal", ".claude/agents", ".claude/commands", ".claude/commands/pal"];

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

/// 등록할 훅 사건.
///
/// ★ **판정하는 목록을 그대로 쓴다.** 등록하는 자리와 판정하는 자리가 갈리면
/// 「등록만 남고 판정이 사라진 훅」이 조용히 돌고, 실측상 그 헛돎은 **어디에도 안
/// 보인다** — 훅 실행은 트랜스크립트에도 화면에도 흔적을 안 남긴다.
pub const HOOK_EVENTS: &[&str] = crate::hook::EVENTS;
