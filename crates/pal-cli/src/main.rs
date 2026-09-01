//! `pal` — 1급 표면.
//!
//! S0 이 뚫은 것: `pal symbols <파일>` — blob 하나 → tree-sitter → 심볼 목록.
//! S1 이 뚫는 것: `pal ledger` — 저장소 하나 → git 접근 · 분류 · 캐시 → 관측 범위 대장.

#![forbid(unsafe_code)]

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use pal_core::{Capable, Language};

mod attach;
mod bind;
mod cache;
mod defect;
mod doctor;
mod evidence;
mod export;
mod hook;
mod install;
mod intent;
mod ledger;
mod narrative;
mod plan;
mod query;
mod round;
mod touch;
mod version;

#[derive(Parser)]
#[command(
    name = "pal",
    version = version::describe(),
    // ★ **2026-08-18 — 옛 문구는 "환경에 종속되지 않는 코드 이해의 큐레이터" 였다.**
    //   그 「환경에 종속되지 않는」이 설계 원리 `P7` 이고, ADR-0025 §3 이 **뒤집었다**
    //   (호스트 중립을 초석에서 내리고 Claude Code 전용으로). 문서에서는 취소선으로
    //   처분했는데 **바이너리는 매 `pal --help` 마다 그것을 제품 정체성으로 내고
    //   있었다** — 살아 있는 표면에 박힌 거짓 신호이고, 문서만 보는 검사도 효과 세션도
    //   못 봤다(독립 리뷰 4 라운드가 잡았다).
    //
    //   ⚠ **코어는 여전히 호스트 없이 선다** — 내려간 것은 하네스 층이다. 그래서
    //   「종속되지 않는다」를 「없이도 선다」로 바꾼다. 재는 것은 `host_free.rs` 다.
    about = "코드 좌표에 결박된 사실과 의도를 내는 상태 관리자 — 호스트 없이도 선다"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

/// `pal narrative` 의 손잡이들 — **평탄화해서 변형 하나로 든다.**
#[derive(clap::Args)]
struct NarrativeArgs {
    /// 저장소 경로
    #[arg(long, default_value = ".")]
    repo: PathBuf,
    /// 어느 커밋인가
    #[arg(long)]
    at: Option<String>,
    #[arg(long)]
    cache_dir: Option<PathBuf>,
    #[arg(long)]
    index: Option<PathBuf>,
    #[arg(long)]
    intent: Option<PathBuf>,
    /// 이 개체의 제안을 **승인한다** — 새 `asserted` 결박이 생긴다
    #[arg(long, conflicts_with = "refuse")]
    approve: Option<String>,
    /// 이 개체의 제안을 **거부한다** — 다시 묻지 않는다
    #[arg(long)]
    refuse: Option<String>,
    /// 후보가 여럿일 때 사람이 고르는 좌표. **기계가 고르지 않는다**
    #[arg(long)]
    pick: Option<String>,
    /// 왜 거부하는가. **`--refuse` 에 필수다** — 이유 없는 거부는 다음 사람에게
    /// 아무 말도 안 한다
    #[arg(long)]
    reason: Option<String>,
    /// 이 경로 아래의 조각을 **일괄 승인**한다 — 하나라도 걸리면 묶음 전체가 거부된다
    #[arg(long)]
    all_of: Option<String>,
    /// 사람이 읽는 화면 대신 JSON 으로 낸다
    #[arg(long)]
    json: bool,
}

/// `pal plan` · `pal deviation` 의 손잡이들 — **평탄화해서 변형 둘이 함께 든다.**
///
/// 둘이 받는 것이 같다. 변형마다 늘어놓으면 `main` 의 팔이 둘 다 다섯 줄이 되고,
/// 그때 `main` 이 *"조립"* 이 아니라 *"손잡이 목록"* 이 된다(`NarrativeArgs` 와 같은 판단).
#[derive(clap::Args)]
struct PlanArgs {
    /// 계획 문서
    file: PathBuf,
    /// 저장소 경로. 기본값은 현재 디렉터리
    #[arg(long, default_value = ".")]
    repo: PathBuf,
    /// 어느 커밋인가. 기본값은 워킹트리
    #[arg(long)]
    at: Option<String>,
    #[arg(long)]
    cache_dir: Option<PathBuf>,
    /// 사람이 읽는 화면 대신 JSON 으로 낸다
    #[arg(long)]
    json: bool,
}

#[derive(Subcommand)]
enum Command {
    /// 파일 하나의 최상위 심볼을 낸다
    Symbols {
        /// 대상 파일
        path: PathBuf,
        /// 사람이 읽는 표 대신 JSON 으로 낸다
        #[arg(long)]
        json: bool,
        /// 심볼 목록이 아니라 **파일 그래프 전부**를 JSON 으로 낸다.
        ///
        /// `--json` 의 형태를 건드리지 않는 이유: `scripts/s0-compare.py` 가 그것을
        /// **JSON 배열**로 파싱하고, 배열의 길이가 S0 대조의 선언 수다. 형태를 바꾸면
        /// 1,122 파일 대조가 깨진다.
        #[arg(long)]
        graph: bool,
    },
    /// 조각 하나를 좌표에 손으로 건다 — **사람이 넣는 자리**
    Bind {
        /// 심볼 이름
        name: String,
        /// 걸 조각
        #[arg(long)]
        note: String,
        /// 무엇까지 지켜보나 — `symbol`(기본) · `callers` · `closure:<k>` · `files:<경로,…>`.
        ///
        /// **넓을수록 거짓 음성이 줄고 거짓 양성이 는다.** 선언한 값이 판정 결과에
        /// 함께 출력된다 — *"이 결정은 `symbol` 반경에서 live"* 는 *"이 결정은
        /// 유효하다"* 와 다른 문장이다(옛 F09 §3).
        #[arg(long, default_value = "symbol")]
        radius: String,
        /// 저장소 경로
        #[arg(long, default_value = ".")]
        repo: PathBuf,
        /// 어느 커밋인가
        #[arg(long)]
        at: Option<String>,
        #[arg(long)]
        cache_dir: Option<PathBuf>,
        #[arg(long)]
        index: Option<PathBuf>,
        /// 의도 저장소 위치. 기본값은 `<저장소>/.palimpsest/intent.redb`
        #[arg(long)]
        intent: Option<PathBuf>,
    },
    /// 문서를 코드 좌표에 건다 — **아무것도 승인하지 않는다** (F10)
    ///
    /// 인자 없이 부르면 3분류를 낸다. `--approve` 는 사람의 승인이고 `--refuse` 는
    /// 거부이며 **둘 다 기록된다.**
    ///
    /// 손잡이를 **평탄화한 구조체**로 받는다 — 다른 명령들처럼 변형 안에 늘어놓으면
    /// `main` 의 한 팔이 열한 줄이 되고, 그때 `main` 이 *"조립"* 이 아니라
    /// *"손잡이 목록"* 이 된다.
    Narrative(NarrativeArgs),
    /// 수정 커밋 하나에서 결함을 소급 결박한다 — **못 담은 것도 센다**
    Defect {
        /// 수정 커밋
        rev: String,
        /// 저장소 경로
        #[arg(long, default_value = ".")]
        repo: PathBuf,
        /// 이력을 얼마나 거슬러 올라가는가. 걸리면 그 사실이 산출에 남는다
        #[arg(long)]
        history_limit: Option<usize>,
        /// 사람이 읽는 화면 대신 JSON 으로 낸다
        #[arg(long)]
        json: bool,
    },
    /// 계획 문서를 읽고 지금 스냅샷에 대 본다 — **이탈은 안 잰다**
    ///
    /// 이탈을 재려면 기준선이 필요하고, 그것은 계획 문서의 프론트매터가 진다
    /// (`baseline: <rev>` · 옛 F12 §4). `pal deviation` 이 그 자리다.
    Plan(PlanArgs),
    /// 계획과 실제의 갈림 — **넷이고 ★ 못 잰 것이 분리돼 있다**
    ///
    /// ⚠ **`--base <ref>` 가 없다.** 기준선은 계획 문서의 프론트매터가 진다
    /// (옛 F12 §4) — 그 손잡이의 소유자는 F23 이다.
    Deviation(PlanArgs),
    /// 좌표 하나를 만진다 — **빈 답도 정직하게 낸다**
    Touch {
        /// 심볼 이름
        name: String,
        /// 저장소 경로. 기본값은 현재 디렉터리
        #[arg(long, default_value = ".")]
        repo: PathBuf,
        /// 어느 커밋인가. 기본값은 HEAD
        #[arg(long)]
        at: Option<String>,
        /// 1층 캐시 위치
        #[arg(long)]
        cache_dir: Option<PathBuf>,
        /// 2층 인덱스 위치. 기본값은 `<저장소>/.palimpsest/index.redb`
        #[arg(long)]
        index: Option<PathBuf>,
        /// 의도 저장소 위치. 기본값은 `<저장소>/.palimpsest/intent.redb`
        #[arg(long)]
        intent: Option<PathBuf>,
        /// 한 구역이 싣는 결박의 상한. 기본값은 자리표시 10 (옛 F11 §3.3)
        ///
        /// **낡은 것은 이 상한에 안 걸린다** — 낡은 것이 안 보이면 이 명령이 존재할
        /// 이유가 없다. 잘린 수는 `elision` 에 실린다.
        #[arg(long)]
        binding_max: Option<usize>,
        /// 걸린 시간을 **표준오류**로 낸다 — 산출에 안 섞는다
        #[arg(long)]
        timing: bool,
        /// 사람이 읽는 화면 대신 JSON 으로 낸다
        #[arg(long)]
        json: bool,
    },
    /// 저장된 그래프가 자기 규칙을 지키는지 본다 — **기본은 표본이고 전수는 명시적이다**
    Doctor {
        /// 저장소 경로. 기본값은 현재 디렉터리
        #[arg(long, default_value = ".")]
        repo: PathBuf,
        /// 어느 커밋인가. 기본값은 HEAD
        #[arg(long)]
        at: Option<String>,
        #[arg(long)]
        cache_dir: Option<PathBuf>,
        #[arg(long)]
        index: Option<PathBuf>,
        #[arg(long)]
        intent: Option<PathBuf>,
        /// **전수로 본다.** 기본은 표본이고 그 사실이 산출에 잔여로 실린다
        #[arg(long)]
        full: bool,
        /// 불변식마다 볼 단위 수의 상한. `--full` 과 함께 쓰면 `--full` 이 이긴다
        #[arg(long)]
        sample: Option<usize>,
        /// **설치 검사 다섯만** 본다 — 저장소 그래프를 안 세운다.
        ///
        /// 안 주면 둘 다 나온다. 설치 검사는 그래프가 없어도 서므로 이 손잡이가
        /// 있는 자리와 없는 자리가 다르다.
        #[arg(long)]
        install: bool,
        /// 사람이 읽는 화면 대신 JSON 으로 낸다
        #[arg(long)]
        json: bool,
    },
    /// 이 프로젝트에 palimpsest 를 놓는다 — **대상 바깥은 안 건드린다**
    Install {
        /// 대상 프로젝트. 기본값은 현재 디렉터리
        #[arg(long, default_value = ".")]
        target: PathBuf,
    },
    /// 놓은 것을 갱신한다 — **사람이 고친 것은 밟지 않고 보고한다**
    Update {
        /// 대상 프로젝트. 기본값은 현재 디렉터리
        #[arg(long, default_value = ".")]
        target: PathBuf,
    },
    /// 놓은 것을 걷어낸다 — **매니페스트에 적힌 것만**
    Uninstall {
        /// 대상 프로젝트. 기본값은 현재 디렉터리
        #[arg(long, default_value = ".")]
        target: PathBuf,
    },
    /// 하네스의 훅이 부르는 자리 — **표준입력으로 페이로드를 받는다**
    ///
    /// 사람이 손으로 부를 일이 없다. `pal install` 이 이 커맨드를 대상 프로젝트의
    /// `settings.json` 에 exec form 으로 등록하고, 하네스가 셸 없이 실행한다.
    Hook {
        /// 사건 이름. **모르는 것은 조용히 통과시킨다**
        event: String,
    },
    /// 회차의 조건과 verification 상태를 읽는다 — 명령을 실행하지 않는다
    Round {
        #[command(subcommand)]
        what: RoundCommand,
    },
    /// 의도 저장소를 JSONL 로 내고 되읽는다 — **재구축 불가한 것의 유일한 복구 경로**
    Intent {
        #[command(subcommand)]
        what: IntentCommand,
    },
    /// 1층 캐시를 본다 — **`prune` 이 닿는 곳은 `cache/` 뿐이다**(R-21)
    Cache {
        #[command(subcommand)]
        what: CacheCommand,
    },
    /// 이름 붙은 질의 하나를 돌린다 — **답이 봉투를 지고 나온다**
    Query {
        /// 질의 이름. `--list` 로 전부 본다
        #[arg(default_value = "")]
        name: String,
        /// 인자 — 심볼 이름
        arg: Option<String>,
        /// 이 빌드가 답하는 질의를 전부 낸다
        #[arg(long)]
        list: bool,
        /// 저장소 경로. 기본값은 현재 디렉터리
        #[arg(long, default_value = ".")]
        repo: PathBuf,
        /// 어느 커밋인가. 기본값은 워킹트리
        #[arg(long)]
        at: Option<String>,
        #[arg(long)]
        cache_dir: Option<PathBuf>,
        #[arg(long)]
        index: Option<PathBuf>,
        /// 의도 저장소 위치. 기본값은 `<저장소>/.palimpsest/intent.redb`
        #[arg(long)]
        intent: Option<PathBuf>,
        /// 몇 홉까지 — **낮추면 절단이 답에 실린다.** 기본값은 자리표시 3
        #[arg(long)]
        depth_max: Option<usize>,
        /// 답이 담는 노드 수의 상한. 기본값은 자리표시 500
        #[arg(long)]
        node_max: Option<usize>,
        /// **2층에 읽기 전용으로 붙는다** — 여럿이 동시에 붙을 수 있다.
        ///
        /// 스티칭을 안 하므로 2층이 이 스냅샷에 대해 **이미 서 있어야** 하고,
        /// 아니면 답이 낡는다(그 사실이 봉투에 실린다). **질의 로그를 못 남기고**
        /// 그것도 봉투에 실린다 — 조용히 빠지면 F17 이 미조회를 과대 계상한다.
        #[arg(long)]
        read_only: bool,
        /// 사람이 읽는 화면 대신 JSON 으로 낸다
        #[arg(long)]
        json: bool,
    },
    /// 2층을 우리 밖 도구가 읽는 형식으로 낸다 — **못 낸 라벨을 함께 적는다**
    Export {
        /// 저장소 경로. 기본값은 현재 디렉터리
        #[arg(long, default_value = ".")]
        repo: PathBuf,
        /// 어느 커밋인가. 기본값은 워킹트리
        #[arg(long)]
        at: Option<String>,
        #[arg(long)]
        cache_dir: Option<PathBuf>,
        #[arg(long)]
        index: Option<PathBuf>,
        /// 형식. **이 빌드가 아는 것은 하나다** — 나머지는 크레이트를 요구한다
        #[arg(long, value_enum, default_value_t = export::Format::Cypher)]
        format: export::Format,
        /// 낼 파일. 없으면 표준출력으로 가고 근거는 표준오류로 간다
        #[arg(long)]
        out: Option<PathBuf>,
        /// 근거를 JSON 봉투로 낸다. **`--out` 이 있어야 한다**
        #[arg(long)]
        json: bool,
    },
    /// 저장소 하나의 관측 범위 대장을 낸다 — **무엇을 보았고 무엇을 보지 않았는가**
    Ledger {
        /// 저장소 경로. 기본값은 현재 디렉터리
        #[arg(default_value = ".")]
        path: PathBuf,
        /// 어느 커밋인가. 기본값은 HEAD
        #[arg(long)]
        at: Option<String>,
        /// 1층 캐시 위치. 기본값은 `<저장소>/.palimpsest/cache`
        #[arg(long)]
        cache_dir: Option<PathBuf>,
        /// 사람이 읽는 표 대신 JSON 으로 낸다
        #[arg(long)]
        json: bool,
        /// 대장이 아니라 **좌표를 붙인 심볼 전부**를 한 줄에 하나씩 JSON 으로 낸다.
        ///
        /// 옛 F03 §6.3 의 골든(`(symbol_id, body_digest)` 스냅샷)이 읽는 표면이다.
        /// **줄 단위인 이유**: 골든의 일은 *"얼마나 움직였는가"* 를 보이는 것이고,
        /// 한 덩어리 JSON 은 한 심볼이 움직여도 전체가 달라 보인다. 줄로 내면
        /// `diff` 가 곧 움직인 것의 목록이다.
        ///
        /// **`--json` 의 형태를 건드리지 않는다** — `s0-compare.py` 가 그것을 파싱한다.
        #[arg(long)]
        symbols: bool,
    },
}

#[derive(Subcommand)]
enum IntentCommand {
    /// 전부를 JSONL 로 낸다 — **상시 유지되지 않는 내보내기는 없는 것과 같다**
    Export {
        /// 저장소 경로. 기본값은 현재 디렉터리
        #[arg(long, default_value = ".")]
        repo: PathBuf,
        /// 의도 저장소 위치. 기본값은 `<저장소>/.palimpsest/intent.redb`
        #[arg(long)]
        intent: Option<PathBuf>,
        /// 낼 파일. 없으면 표준출력
        #[arg(long)]
        out: Option<PathBuf>,
    },
    /// JSONL 을 읽어 **더한다** — 바꿔치기가 아니다(R-21)
    Import {
        /// 읽을 파일
        file: PathBuf,
        /// 저장소 경로. 기본값은 현재 디렉터리
        #[arg(long, default_value = ".")]
        repo: PathBuf,
        #[arg(long)]
        intent: Option<PathBuf>,
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum RoundCommand {
    /// `intent.md`의 완수 조건을 Rust 정본으로 읽는다
    Conditions {
        #[arg(long)]
        file: PathBuf,
        #[arg(long)]
        json: bool,
    },
    /// verification 원장을 읽기 전용으로 축약한다
    Status {
        #[arg(long)]
        round: Option<String>,
        #[arg(long)]
        json: bool,
    },
    /// exact command oracle를 외부 사용자 저장소에 승인한다 — 실행하지 않는다
    Approve {
        #[arg(long)]
        round: String,
        #[arg(long)]
        id: String,
        #[arg(long, default_value = ".")]
        repo: PathBuf,
        #[arg(long)]
        approval_dir: Option<PathBuf>,
        #[arg(long)]
        shell: Option<PathBuf>,
        #[arg(long, default_value_t = pal_core::PROVISIONAL_ROUND_ORACLE_TIMEOUT_SECS)]
        timeout: u64,
        #[arg(long, default_value_t = pal_core::PROVISIONAL_ROUND_ORACLE_OUTPUT_BYTES)]
        output_limit: usize,
        #[arg(long)]
        json: bool,
    },
    /// 승인된 exact command oracle를 실행하고 current evidence를 append한다
    Verify {
        #[arg(long)]
        round: String,
        #[arg(long, required_unless_present = "all", conflicts_with = "all")]
        id: Option<String>,
        /// 이미 met인 command까지 전부 재실행하고 completion checkpoint를 쓴다
        #[arg(long, required_unless_present = "id", conflicts_with = "id")]
        all: bool,
        #[arg(long, default_value = ".")]
        repo: PathBuf,
        #[arg(long)]
        approval_dir: Option<PathBuf>,
        #[arg(long)]
        shell: Option<PathBuf>,
        #[arg(long, default_value_t = pal_core::PROVISIONAL_ROUND_ORACLE_TIMEOUT_SECS)]
        timeout: u64,
        #[arg(long, default_value_t = pal_core::PROVISIONAL_ROUND_ORACLE_OUTPUT_BYTES)]
        output_limit: usize,
        #[arg(long)]
        json: bool,
    },
    /// Stop 정책의 명시적 활성화·비활성화·상태 조회
    Stop {
        #[command(subcommand)]
        what: RoundStopCommand,
    },
}

#[derive(Subcommand)]
enum RoundStopCommand {
    /// 현재 프로젝트의 지정 회차에 Stop 정책을 명시적으로 활성화한다
    Enable {
        #[arg(long)]
        round: String,
        #[arg(long, default_value = ".")]
        repo: PathBuf,
        #[arg(long)]
        approval_dir: Option<PathBuf>,
        #[arg(long)]
        json: bool,
    },
    /// activation record 내용이 손상됐어도 Stop 정책을 즉시 비활성화한다
    Disable {
        #[arg(long, default_value = ".")]
        repo: PathBuf,
        #[arg(long)]
        approval_dir: Option<PathBuf>,
        #[arg(long)]
        json: bool,
    },
    /// Stop activation과 operational progress 상태를 읽는다
    Status {
        #[arg(long, default_value = ".")]
        repo: PathBuf,
        #[arg(long)]
        approval_dir: Option<PathBuf>,
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum CacheCommand {
    /// 얼마나 차 있는가
    Stats {
        /// 저장소 경로. 기본값은 현재 디렉터리
        #[arg(long, default_value = ".")]
        repo: PathBuf,
        /// 1층 캐시 위치. 기본값은 `<저장소>/.palimpsest/cache`
        #[arg(long)]
        cache_dir: Option<PathBuf>,
        #[arg(long)]
        json: bool,
    },
    /// 예산까지 줄인다 — **되돌릴 수 없다.** 닿는 곳은 캐시 디렉터리뿐이다
    Prune {
        /// 저장소 경로. 기본값은 현재 디렉터리
        #[arg(long, default_value = ".")]
        repo: PathBuf,
        /// 1층 캐시 위치. 기본값은 `<저장소>/.palimpsest/cache`
        #[arg(long)]
        cache_dir: Option<PathBuf>,
        /// 남길 바이트. 기본값은 2GiB (옛 F04 §3.4 · **자리표시다**)
        #[arg(long)]
        budget: Option<u64>,
        /// **격리 방**(`.corrupt/`)을 이 바이트까지 줄인다.
        ///
        /// **주지 않으면 한 바이트도 안 지운다** — 격리된 바이트는 결함의 증거다.
        #[arg(long)]
        sweep_quarantine: Option<u64>,
        /// **죽은 `.tmp`** 를 지운다. 주지 않으면 한 개도 안 지운다
        #[arg(long)]
        sweep_stray: bool,
        /// `.tmp` 를 죽은 것으로 보기까지의 나이(초). 기본값은 자리표시 3600
        #[arg(long)]
        stray_age: Option<u64>,
        #[arg(long)]
        json: bool,
    },
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Command::Symbols { path, json, graph } => symbols(&path, json, graph),
        Command::Narrative(a) => 서술물(&a),
        Command::Bind { name, note, radius, repo, at, cache_dir, index, intent } => bind::run(
            bind::Args { repo: &repo, rev: at.as_deref(), cache_dir, index, intent,
                         name: &name, note: &note, radius: &radius },
        ),
        Command::Defect { rev, repo, history_limit, json } => {
            let report = defect::run(
                &rev,
                &repo,
                history_limit.unwrap_or_else(defect::default_budget),
            )?;
            if json {
                defect::print_json(&report)
            } else {
                defect::print(&report);
                Ok(())
            }
        }
        Command::Plan(a) => plan::plan(&계획_인자(&a)),
        Command::Deviation(a) => plan::deviation(&계획_인자(&a)),
        Command::Touch { name, repo, at, cache_dir, index, intent, binding_max, timing, json } =>
            touch::run(touch::Args { repo: &repo, rev: at.as_deref(), cache_dir, index, intent,
                                     name: &name, binding_max, timing, json }),
        Command::Doctor { repo, at, cache_dir, index, intent, full, sample, install, json } => {
            let scope = if full {
                pal_core::DoctorScope::Full
            } else {
                pal_core::DoctorScope::Sample { max: sample.unwrap_or(pal_core::PROVISIONAL_SAMPLE_MAX) }
            };
            doctor::run(doctor::Args {
                repo: &repo,
                rev: at.as_deref(),
                cache_dir,
                index,
                intent,
                scope,
                install_only: install,
                json,
            })
        }
        Command::Install { target } => install::install(&target),
        Command::Update { target } => install::update(&target),
        Command::Uninstall { target } => install::uninstall(&target),
        // **훅은 실패를 안 낸다** — 그 사실이 `hook::run` 의 타입에 적혀 있다.
        Command::Hook { event } => {
            hook::run(&event);
            Ok(())
        }
        Command::Round { what } => match what {
            RoundCommand::Conditions { file, json } => round::conditions(&file, json),
            RoundCommand::Status { round: slug, json } => {
                round::round_status(slug.as_deref(), json)
            }
            RoundCommand::Approve {
                round: slug, id, repo, approval_dir, shell, timeout, output_limit, json,
            } => round::round_approve(
                &repo, &slug, &id, approval_dir.as_deref(), shell.as_deref(), timeout,
                output_limit, json,
            ),
            RoundCommand::Verify {
                round: slug, id, all, repo, approval_dir, shell, timeout, output_limit, json,
            } => {
                if all {
                    round::round_finalize(
                        &repo, &slug, approval_dir.as_deref(), shell.as_deref(), timeout,
                        output_limit, json,
                    )
                } else {
                    round::round_verify(
                        &repo, &slug, id.as_deref().expect("clap requires id"),
                        approval_dir.as_deref(), shell.as_deref(), timeout, output_limit, json,
                    )
                }
            }
            RoundCommand::Stop { what } => match what {
                RoundStopCommand::Enable { round: slug, repo, approval_dir, json } => {
                    round::stop::command_enable(&repo, &slug, approval_dir.as_deref(), json)
                }
                RoundStopCommand::Disable { repo, approval_dir, json } => {
                    round::stop::command_disable(&repo, approval_dir.as_deref(), json)
                }
                RoundStopCommand::Status { repo, approval_dir, json } => {
                    round::stop::command_status(&repo, approval_dir.as_deref(), json)
                }
            },
        },
        Command::Intent { what } => match what {
            IntentCommand::Export { repo, intent, out } => intent::export(&repo, intent, out),
            IntentCommand::Import { file, repo, intent, json } => {
                intent::import(&repo, intent, &file, json)
            }
        },
        Command::Cache { what } => 캐시(what),
        Command::Query {
            name,
            arg,
            list,
            repo,
            at,
            cache_dir,
            index,
            intent: intent_path,
            depth_max,
            node_max,
            read_only,
            json,
        } => query::run(&query::Args {
            name: &name,
            arg: arg.as_deref(),
            list,
            repo: &repo,
            rev: at.as_deref(),
            cache_dir,
            index,
            intent: intent_path,
            depth_max,
            node_max,
            read_only,
            json,
        }),
        Command::Export { repo, at, cache_dir, index, format, out, json } => {
            export::run(export::Args { repo, rev: at, cache_dir, index, format, out, json })
        }
        Command::Ledger { path, at, cache_dir, json, symbols } => {
            let report = ledger::compute(&path, at.as_deref(), cache_dir)?;
            if symbols {
                ledger::print_symbols(&report)?;
            } else if json {
                println!("{}", serde_json::to_string_pretty(&report)?);
            } else {
                ledger::print_table(&report);
            }
            Ok(())
        }
    }
}

fn symbols(path: &Path, json: bool, graph: bool) -> Result<()> {
    let source = std::fs::read(path).with_context(|| format!("읽지 못했다: {}", path.display()))?;

    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    let Some(language) = Language::from_extension(ext) else {
        // **"언어를 모른다"와 "추출기가 없다"는 다르다.** 여기는 전자다.
        // **목록을 손으로 적지 않는다**(ADR-0024). 코드가 이미 아는 것에서 렌더링한다 —
        // 앞 판은 네 이름이 문자열에 박혀 있었고 어떤 시험도 그것을 안 봤다(#66 사전부검).
        let 아는것: Vec<&str> = pal_extract::FIRST_CLASS.iter().map(|l| l.name()).collect();
        anyhow::bail!(
            "확장자 `.{ext}` 를 언어로 알지 못한다 — 아는 것은 {} {} 이다",
            아는것.join(" · "),
            match 아는것.len() {
                1 => "하나",
                2 => "둘",
                3 => "셋",
                4 => "넷",
                5 => "다섯",
                _ => "전부",
            }
        );
    };

    if graph {
        return file_graph(&source, language);
    }

    match pal_extract::extract(language, &source) {
        Capable::Present(result) => {
            let found = result.with_context(|| format!("추출 실패: {}", path.display()))?;
            if json {
                println!("{}", serde_json::to_string_pretty(&found)?);
            } else {
                print_table(path, language, &found);
            }
        }
        Capable::NotBuilt { capability } => {
            // **빈 목록을 내지 않는다.** `Finding 0` 과 "안 만들었음"이 같은 출력이 되는 것을
            // 목표 §3.1 이 금지한다.
            if json {
                let not_built: Capable<Vec<pal_core::Symbol>> = Capable::NotBuilt { capability };
                println!("{}", serde_json::to_string_pretty(&not_built)?);
            } else {
                println!(
                    "  (이 빌드에 {} 추출 능력이 없습니다 — {} 미구축)",
                    language.name(),
                    capability.feature
                );
            }
        }
    }
    Ok(())
}

/// 파일 그래프 전부를 JSON 으로.
///
/// **이것이 `[f02.1.pass]` ② 가 밖에서 잴 수 있는 유일한 창이다** — 같은 blob 을 다른
/// 저장소·다른 경로에 두고 이 산출이 바이트 단위로 같은지 본다. 그러려면 경로가 산출에
/// **실리면 안 되고**, 그래서 여기서 `path` 를 찍지 않는다.
fn file_graph(source: &[u8], language: Language) -> Result<()> {
    match pal_extract::extractor_for(language) {
        Capable::Present(extractor) => {
            let graph = extractor.extract(source).context("추출 실패")?;
            println!("{}", serde_json::to_string_pretty(&graph)?);
        }
        Capable::NotBuilt { capability } => {
            // **빈 그래프를 내지 않는다.** 선언이 없는 파일과 같은 출력이 된다.
            let not_built: Capable<pal_core::FileGraph> = Capable::NotBuilt { capability };
            println!("{}", serde_json::to_string_pretty(&not_built)?);
        }
    }
    Ok(())
}

fn print_table(path: &Path, language: Language, found: &[pal_core::Symbol]) {
    let v = pal_extract::version();
    println!("{}  ·  {}", path.display(), language.name());
    println!("문법 {}  ·  추출기 {}", &v.grammar[..7], v.extractor);
    println!();
    if found.is_empty() {
        println!("  최상위 선언 없음");
    } else {
        for s in found {
            println!("  {:>5}  {:<10}  {}", s.span.line_start, s.kind.name(), s.name);
        }
    }
    println!();
    println!("  선언 {}", found.len());
}

/// `pal cache` 의 갈래 둘 — **`main` 밖으로 뗀다.**
///
/// `main` 은 조립이고, 팔 하나가 스무 줄이면 그것은 조립이 아니라 손잡이 목록이다.
///
/// # Errors
/// 캐시를 읽지 못하거나 줄이지 못하면.
fn 캐시(what: CacheCommand) -> Result<()> {
    match what {
        CacheCommand::Stats { repo, cache_dir, json } => cache::stats(&repo, cache_dir, json),
        CacheCommand::Prune {
            repo,
            cache_dir,
            budget,
            sweep_quarantine,
            sweep_stray,
            stray_age,
            json,
        } => cache::prune(cache::PruneArgs {
            repo,
            cache_dir,
            budget: budget.unwrap_or(pal_core::DEFAULT_CACHE_BUDGET_BYTES),
            sweep_quarantine,
            sweep_stray,
            stray_age: stray_age.unwrap_or(pal_core::PROVISIONAL_STRAY_TMP_MAX_AGE_SECS),
            json,
        }),
    }
}

/// `pal plan`·`pal deviation` 의 손잡이를 실행부의 인자로 바꾼다.
fn 계획_인자(a: &PlanArgs) -> plan::Args<'_> {
    plan::Args {
        repo: &a.repo,
        rev: a.at.as_deref(),
        cache_dir: a.cache_dir.clone(),
        plan: &a.file,
        json: a.json,
    }
}

/// `pal narrative` — 손잡이를 갈래로 바꿔 넘긴다.
///
/// # Errors
/// 갈래를 못 가르거나 인입이 실패하면.
fn 서술물(a: &NarrativeArgs) -> Result<()> {
    narrative::run(narrative::Args {
        repo: &a.repo,
        rev: a.at.as_deref(),
        cache_dir: a.cache_dir.clone(),
        index: a.index.clone(),
        intent: a.intent.clone(),
        json: a.json,
        what: 서술물_갈래(
            a.approve.as_deref(),
            a.refuse.as_deref(),
            a.pick.as_deref(),
            a.reason.as_deref(),
            a.all_of.as_deref(),
        )?,
    })
}

/// `pal narrative` 의 갈래 셋을 손잡이에서 가른다.
///
/// **`--refuse` 는 이유를 요구한다** — 이유 없는 거부는 다음 사람에게 아무 말도 안
/// 한다(옛 F10 §3.3: *"재질문 제거가 승인 비용 절감의 대부분"*).
fn 서술물_갈래<'a>(
    approve: Option<&'a str>,
    refuse: Option<&'a str>,
    pick: Option<&'a str>,
    reason: Option<&'a str>,
    all_of: Option<&'a str>,
) -> Result<narrative::What<'a>> {
    if let Some(item) = refuse {
        let (Some(pick), Some(reason)) = (pick, reason) else {
            anyhow::bail!(
                "`--refuse` 는 `--pick <좌표>` 와 `--reason <왜>` 를 요구합니다 — \
                 거부는 **(조각, 좌표) 짝**에 대한 것이고, 이유가 없으면 다음 사람이 \
                 같은 후보를 다시 봅니다"
            );
        };
        return Ok(narrative::What::Refuse { item, pick, reason });
    }
    if let Some(item) = approve {
        return Ok(narrative::What::Approve { item, pick, all_of: None });
    }
    if let Some(prefix) = all_of {
        return Ok(narrative::What::Approve { item: "", pick: None, all_of: Some(prefix) });
    }
    Ok(narrative::What::Ingest)
}
