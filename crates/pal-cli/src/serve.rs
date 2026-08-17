//! `pal serve` — MCP 어댑터를 stdio 로 세운다 (F06 §4b).
//!
//! # 여기 있는 것은 조립뿐이다
//!
//! 프로토콜은 [`pal_mcp`] 가 지고 질의는 [`crate::query::answer`] 가 진다. 이 파일은
//! **둘을 잇는다** — 그리고 그 이음이 이 방향이어야 하는 이유가 [`pal_mcp::Answers`]
//! 의 머리에 적혀 있다(`pal-mcp` 는 `pal-cli` 에 의존할 수 없다).
//!
//! # 왜 `--json` 이 없는가
//!
//! 다른 명령은 사람용 표와 `--json` 을 가른다. 여기서는 **소비자가 언제나 기계**이고
//! 답은 언제나 봉투의 JSON 이다. 손잡이를 두면 *"사람이 읽는 MCP"* 라는 없는 것을
//! 시사한다.

use std::path::PathBuf;

use anyhow::Result;
use pal_core::QueryName;
use pal_query::NamedQuery;

pub struct Args {
    pub repo: PathBuf,
    pub rev: Option<String>,
    pub cache_dir: Option<PathBuf>,
    pub index: Option<PathBuf>,
    pub intent: Option<PathBuf>,
    pub depth_max: Option<usize>,
    pub node_max: Option<usize>,
    pub read_only: bool,
}

/// 답하는 쪽 — **`pal query` 와 같은 조립을 지난다.**
///
/// 손잡이를 통째로 들고 있다가 질의마다 [`crate::query::Args`] 를 세운다. 세션 하나가
/// 여러 질의를 받으므로 조립 결과를 붙들어 둘 수도 있겠지만 **안 붙든다** —
/// 붙들면 그 세션 동안 대장이 얼어붙고, 워킹트리가 바뀌어도 같은 답이 나간다.
/// 낡음은 답마다 다시 재어져야 한다(`ProjectionFreshness`).
struct 조립기 {
    args: Args,
}

impl pal_mcp::Answers for 조립기 {
    fn answer(&self, name: QueryName, arg: Option<&str>) -> Result<String, String> {
        // **이름을 문자열로 되돌려 `NamedQuery` 를 만든다.** 어댑터가 준 것은
        // `QueryName`(인자 없는 이름)이고 실행이 요구하는 것은 `NamedQuery`(인자 붙은
        // 것)다. 그 변환은 `parse` 하나뿐이고 CLI 도 같은 것을 지난다.
        let Some(query) = NamedQuery::parse(name.name(), arg) else {
            return Err(format!(
                "질의 `{}` 에 인자가 맞지 않는다 — 필요한 것은 {:?}",
                name.name(),
                name.arg_names()
            ));
        };

        let a = crate::query::Args {
            name: name.name(),
            arg,
            list: false,
            repo: &self.args.repo,
            rev: self.args.rev.as_deref(),
            cache_dir: self.args.cache_dir.clone(),
            index: self.args.index.clone(),
            intent: self.args.intent.clone(),
            depth_max: self.args.depth_max,
            node_max: self.args.node_max,
            read_only: self.args.read_only,
            json: true,
        };

        let envelope = crate::query::answer(&a, &query).map_err(|e| format!("{e:#}"))?;
        // **빈틈 없이 직렬화한다** — `Envelope::new` 가 `serialized_bytes` 를 그렇게
        // 재기 때문이다. 여기서 예쁘게 내면 봉투가 신고한 크기와 실제로 나간 바이트가
        // 갈리고, 그때 `[f06b.pass]` ⑧ 이 재는 값이 답과 어긋난다.
        serde_json::to_string(&envelope).map_err(|e| format!("답을 직렬화하지 못했다: {e}"))
    }
}

/// # Errors
/// 런타임을 세우지 못하거나 세션이 끊기면.
pub fn run(a: Args) -> Result<()> {
    pal_mcp::serve_stdio(조립기 { args: a }, crate::version::describe())
}
