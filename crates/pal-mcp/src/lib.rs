//! 어댑터 — MCP. **능력을 하나도 안 더한다** (F06 §4b).
//!
//! # 이 크레이트가 존재하는 이유, 그리고 존재하지 않아도 되는 이유
//!
//! [F06 §1.1](../../../docs/plan/features/F06-surface.md) 이 지위를 정했다 —
//! ***"1급은 CLI 다. 프로토콜은 어댑터다."*** 그러므로 여기 있는 것은 **경로**이지
//! 능력이 아니다. 이 크레이트를 통째로 빼도 `pal` 의 어떤 답도 사라지지 않고,
//! 그것을 `[f06b.pass]` ⑥ 이 **실제로 빼고 돌려서** 잰다.
//!
//! # 툴 목록을 손으로 안 적는다
//!
//! F06 §4.2 의 스케치는 `#[tool_router]` + 질의마다 함수 하나인데, **그 형태는 질의
//! 열 개에 함수 열 개를 손으로 적는다.** 그것이 곧 두 번째 목록이고, `queries.toml` 이
//! 변해도 안 따라온다 — 문서 §2 규칙 1(*"질의 추가는 `queries.toml` 변경으로만
//! 일어난다"*)과 정면으로 부딪힌다.
//!
//! 그래서 [`ServerHandler::list_tools`] 를 직접 구현하고 **[`pal_core::QueryName::ALL`]
//! 을 순회해서** 툴을 만든다. 질의가 늘면 툴이 따라 늘고, **갈릴 방법이 없다.**
//!
//! # 여기 없는 것 — 조립
//!
//! 질의를 돌리려면 대장·2층·의도 저장소·예산의 조립이 필요한데 **그것은 `pal-cli` 에
//! 산다**(`query::answer`). 이 크레이트는 `pal-cli` 에 의존할 수 없으므로
//! (`cargo xtask check` 의 의존 방향 규칙 3 — *"소비자 어휘의 역류"*), 답을 내는 쪽을
//! [`Answers`] 로 받는다. **조립이 한 곳에 남는 것이 요점이다** — 어댑터가 자기 조립을
//! 쓰면 예산 하나가 달라도 같은 질의가 다른 답을 낸다.

#![forbid(unsafe_code)]

use std::borrow::Cow;
use std::sync::Arc;

use pal_core::QueryName;
use rmcp::handler::server::ServerHandler;
use rmcp::model::{
    CallToolRequestParams, CallToolResponse, CallToolResult, ContentBlock, Implementation,
    JsonObject, ListToolsResult, PaginatedRequestParams, ProtocolVersion, ServerCapabilities,
    ServerInfo, Tool,
};
use rmcp::service::RequestContext;
use rmcp::{ErrorData as McpError, RoleServer, ServiceExt};

/// 질의 하나에 답하는 쪽.
///
/// # 왜 `Envelope` 가 아니라 `String` 인가
///
/// 이 크레이트가 봉투의 **타입**을 알면 `pal-query` 에 의존해야 하고, 그러면 어댑터가
/// 답의 모양을 아는 자리가 된다. 아는 순간 그 모양을 **고칠 수 있는 자리**이기도 하다.
/// 직렬화된 것을 그대로 나르면 어댑터는 **운반만** 한다.
///
/// 그리고 `[f06b.pass]` ⑧ 이 여기 붙는다 — 응답 크기는 이 문자열의 바이트이고,
/// 그 안에 `Envelope::tokens`(잰 것 `serialized_bytes` · 가정한 것 `bytes_per_token`)가
/// **이미 실려 있다.** 어댑터가 새로 세지 않는다.
pub trait Answers: Send + Sync + 'static {
    /// 이름과 인자로 답을 낸다 — **직렬화된 봉투**를 돌려준다.
    ///
    /// # Errors
    /// 조립이나 실행이 실패하면 사람이 읽을 수 있는 까닭을.
    fn answer(&self, name: QueryName, arg: Option<&str>) -> Result<String, String>;
}

/// 이 질의의 입력 스키마 — **`arg_names` 와 `arg_types` 에서 만든다.**
///
/// 인자가 없으면 `properties` 가 비고 `required` 도 빈다. 그것이 정확한 표현이다 —
/// *"인자를 안 받는다"* 이지 *"아무거나 받는다"* 가 아니다.
fn 입력_스키마(q: QueryName) -> Arc<JsonObject> {
    let mut properties = serde_json::Map::new();
    for (name, ty) in q.arg_names().iter().zip(q.arg_types()) {
        properties.insert(
            (*name).to_owned(),
            serde_json::json!({
                "type": "string",
                // **타입 이름을 설명으로 싣는다.** `SymbolName` 과 `RepoPath` 는 둘 다
                // JSON 으로는 문자열이지만 **같은 것이 아니고**, 부르는 쪽이 그것을
                // 알아야 한다(`plan.deviation` 에 심볼 이름을 넣으면 답이 안 선다).
                "description": *ty,
            }),
        );
    }
    let required: Vec<&str> = q.arg_names().to_vec();

    let mut schema = serde_json::Map::new();
    schema.insert("type".to_owned(), serde_json::json!("object"));
    schema.insert("properties".to_owned(), serde_json::Value::Object(properties));
    schema.insert("required".to_owned(), serde_json::json!(required));
    Arc::new(schema)
}

/// 이 빌드가 내는 툴 전부 — **[`QueryName::ALL`] 에서 나온다.**
///
/// 목록이 여기서 자라지 않는다. 자랄 수 있는 자리는 `QueryName` 하나뿐이고,
/// 그것이 `surface/queries.toml` 과 `cargo xtask check` 로 묶여 있다.
#[must_use]
pub fn tools() -> Vec<Tool> {
    QueryName::ALL
        .into_iter()
        .map(|q| {
            Tool::new(
                Cow::Borrowed(q.name()),
                Cow::Borrowed(q.summary()),
                입력_스키마(q),
            )
        })
        .collect()
}

/// MCP 서버 — 질의 표면의 어댑터.
pub struct Server<A: Answers> {
    answers: A,
    version: String,
}

impl<A: Answers> Server<A> {
    /// 답하는 쪽과 이 빌드의 판을 받는다.
    pub fn new(answers: A, version: impl Into<String>) -> Self {
        Self { answers, version: version.into() }
    }
}

impl<A: Answers> ServerHandler for Server<A> {
    // **`Default::default()` 뒤에 필드를 대입한다** — clippy 는 `S { field, ..Default }` 를
    // 권하는데 `rmcp` 의 이 타입들은 `#[non_exhaustive]` 라 **바깥 크레이트에서 구조체
    // 표현식 자체를 못 쓴다**(E0639). 제안을 따를 수 없는 자리이므로 여기서만 끈다.
    #[allow(clippy::field_reassign_with_default)]
    fn get_info(&self) -> ServerInfo {
        let mut info = ServerInfo::default();
        info.protocol_version = ProtocolVersion::LATEST;
        info.capabilities = ServerCapabilities::builder().enable_tools().build();
        info.server_info = Implementation::new("palimpsest", self.version.clone());
        // **능력 목록을 여기 안 적는다.** 무엇을 못 만들었는지는 답의 봉투가 지고
        // 나간다(`capabilities`) — 두 곳에 적으면 한쪽이 조용히 낡는다.
        info.instructions = Some(
            "코드 좌표에 걸린 결정·계획·서술물과 그 낡음을 낸다. \
             모든 답은 봉투를 지고 나온다 — 무엇을 못 보았고 무엇을 잘랐는지가 함께 실린다."
                .to_owned(),
        );
        info
    }

    #[allow(clippy::field_reassign_with_default)] // 위와 같은 이유 — `non_exhaustive`
    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, McpError> {
        let mut out = ListToolsResult::default();
        out.tools = tools();
        Ok(out)
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<CallToolResponse, McpError> {
        // **모르는 이름은 오류다** — `[f06b.pass]` ②. 빈 답으로 답하면 부르는 쪽이
        // *"그런 것이 없다"* 와 *"있는데 비었다"* 를 못 가른다.
        let Some(query) = QueryName::parse(&request.name) else {
            return Err(McpError::invalid_params(
                format!(
                    "질의 `{}` 를 모른다 — 아는 것은 {} 다",
                    request.name,
                    QueryName::ALL.map(QueryName::name).join(" · ")
                ),
                None,
            ));
        };

        // 인자는 **하나 이하**다(`arg_names` 의 길이). 그 사실은 `NamedQuery::parse` 의
        // 모양이 이미 지고 있고, `인자가_하나_이하다` 가 그것을 못 박는다.
        //
        // ★ **「안 왔다」와 「타입이 틀렸다」를 가른다.** 뭉치면 부르는 쪽이 무엇을
        // 고칠지 모른다 — 인자를 숫자로 보낸 사람에게 *"인자가 필요하다"* 라고 답하는 것은
        // 거짓이다. `pal query` 의 *"모르는 이름"* 과 *"인자가 없다"* 를 가르는 것과
        // 같은 규율이고, 그 자리에 이미 그렇게 적혀 있다(`query.rs` 의 `run`).
        let mut arg = None;
        if let Some(name) = query.arg_names().first() {
            let 값 = request.arguments.as_ref().and_then(|m| m.get(*name));
            match 값 {
                Some(serde_json::Value::String(s)) => arg = Some(s.clone()),
                Some(other) => {
                    return Err(McpError::invalid_params(
                        format!(
                            "질의 `{}` 의 인자 `{name}` 은 문자열이어야 한다 — 온 것은 {}",
                            query.name(),
                            타입_이름(other)
                        ),
                        None,
                    ));
                }
                None => {
                    return Err(McpError::invalid_params(
                        format!("질의 `{}` 에 인자 `{name}` 가 필요하다", query.name()),
                        None,
                    ));
                }
            }
        }

        match self.answers.answer(query, arg.as_deref()) {
            Ok(body) => Ok(CallToolResult::success(vec![ContentBlock::text(body)]).into()),
            // **답하지 못한 것은 「빈 답」이 아니라 오류다.** 조립이 실패한 것을 빈
            // 결과로 내면 *"관측이 0 건"* 과 구별되지 않는다.
            Err(why) => Err(McpError::internal_error(why, None)),
        }
    }
}

/// JSON 값의 갈래 이름 — 오류 메시지가 *"온 것은 무엇인가"* 를 말하기 위해.
fn 타입_이름(v: &serde_json::Value) -> &'static str {
    match v {
        serde_json::Value::Null => "null",
        serde_json::Value::Bool(_) => "참거짓",
        serde_json::Value::Number(_) => "수",
        serde_json::Value::String(_) => "문자열",
        serde_json::Value::Array(_) => "배열",
        serde_json::Value::Object(_) => "객체",
    }
}

/// stdio 로 선다 — **동기 함수다.**
///
/// 런타임을 여기서 세우고 여기서 끝낸다. `pal-cli` 는 동기이고 그대로 둔다 —
/// 어댑터의 실행 모델이 표면으로 새면 *"어댑터를 빼도 아무것도 안 죽는다"* 가
/// 거짓이 된다.
///
/// # Errors
/// 런타임을 세우지 못하거나 트랜스포트가 끊기면.
pub fn serve_stdio<A: Answers>(answers: A, version: impl Into<String>) -> anyhow::Result<()> {
    // **`enable_io()` 를 안 부른다.** 그것은 tokio 의 IO 드라이버(`net` feature)를
    // 켜는 것이고, 우리가 쓰는 것은 `tokio::io::stdin`/`stdout` — 그 둘은 드라이버가
    // 아니라 **블로킹 풀** 위에서 돈다. 부르면 feature 를 하나 더 켜야 하고, 그
    // feature 는 소켓을 여는 능력이다. **상주 서버가 아니라는 것이 P12 의 실질**이므로
    // 소켓을 열 수 있는 상태로 두지 않는다.
    //
    // ★ **그러나 `enable_time()` 은 부른다 — 안 부르면 답이 사라진다.**
    //
    // 위 문단은 *"필요한 것만 켠다"* 를 논증하는데, **그 논증이 「무엇이 필요한가」를
    // 틀리게 셌다.** `rmcp` 는 세션을 닫기 전에 *"Drain in-flight handler responses
    // before closing the transport"* 를 하고 그 자리가 `tokio::time::timeout` 을 탄다
    // (`service.rs:1736`). 타이머 드라이버가 없으면 **거기서 패닉**하고, 드레인이
    // 패닉하므로 **아직 못 내보낸 응답이 통째로 사라진다.**
    //
    // 실측(2026-08-17): `initialize` → `initialized` → `tools/call` 을 파이프로 넣으면
    // **응답이 1 줄**(initialize 만)이고 `tools/call` 의 답은 안 나갔다. rc=**1**.
    // 켠 뒤: **응답 2 줄** · rc=**0** · stderr **빈 출력**.
    //
    // ⚠ **이것이 시험 넷 전부에서 초록으로 읽혔다** — 클라이언트가 답을 받은 뒤
    // `cancel()` 하므로 시험은 통과하고, **서버의 rc=1 과 패닉은 아무도 안 봤다.**
    // `종료가_깨끗하다` 가 이제 그것을 본다.
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()
        .map_err(|e| anyhow::anyhow!("런타임을 세우지 못했다: {e}"))?;

    runtime.block_on(async move {
        let service = Server::new(answers, version)
            .serve(rmcp::transport::io::stdio())
            .await
            .map_err(|e| anyhow::anyhow!("MCP 세션을 세우지 못했다: {e}"))?;
        service.waiting().await.map_err(|e| anyhow::anyhow!("세션이 끊겼다: {e}"))?;
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **툴 목록이 코드의 질의 목록과 같다** — `[f06b.pass]` ③ 의 단위 층.
    ///
    /// 세션을 통과한 산출 쪽 대조는 `pal-cli/tests/mcp_session.rs` 가 진다.
    #[test]
    fn 툴_목록이_질의_목록에서_나온다() {
        let tools = tools();
        let names: Vec<&str> = tools.iter().map(|t| t.name.as_ref()).collect();
        let want: Vec<&str> = QueryName::ALL.iter().map(|q| q.name()).collect();
        assert_eq!(names, want, "툴 목록이 QueryName::ALL 과 갈렸다");
    }

    /// **하한** — 적으면 위의 대조가 공짜로 통과한다. `CATALOG_MIN_QUERIES` 와 같은 수다.
    #[test]
    fn 툴이_여섯_이상이다() {
        let n = tools().len();
        assert!(n >= 6, "툴이 {n}개다 — 하한 미만");
    }

    /// 인자가 **하나 이하**라는 것에 이 어댑터가 기대고 있다.
    ///
    /// 둘 이상인 질의가 생기면 `call_tool` 의 인자 뽑기가 조용히 첫 번째만 읽는다.
    /// **그때 이 시험이 먼저 빨개진다.**
    #[test]
    fn 인자가_하나_이하다() {
        for q in QueryName::ALL {
            assert!(
                q.arg_names().len() <= 1,
                "{} 의 인자가 {}개다 — 어댑터가 하나 이하를 가정한다",
                q.name(),
                q.arg_names().len()
            );
        }
    }

    /// **「안 왔다」와 「타입이 틀렸다」가 다른 말을 한다.**
    ///
    /// 뭉치면 인자를 숫자로 보낸 사람이 *"인자가 필요하다"* 를 받고, 그것은 거짓이다.
    #[test]
    fn 타입_이름이_갈래마다_다르다() {
        let 갈래 = [
            serde_json::json!(null),
            serde_json::json!(true),
            serde_json::json!(1),
            serde_json::json!("x"),
            serde_json::json!([]),
            serde_json::json!({}),
        ];
        let 이름들: Vec<&str> = 갈래.iter().map(타입_이름).collect();
        let 중복 = 이름들.iter().collect::<std::collections::BTreeSet<_>>();
        assert_eq!(중복.len(), 이름들.len(), "갈래 이름이 겹친다 — 뭉개진다: {이름들:?}");
    }

    /// 스키마가 인자를 **필수로** 싣는다 — 안 실으면 부르는 쪽이 안 보내고, 그러면
    /// `Ambiguous`·`Unknown` 이 아니라 오류가 난다.
    #[test]
    fn 인자_받는_질의는_스키마가_그것을_필수로_적는다() {
        for t in tools() {
            let q = QueryName::parse(t.name.as_ref()).expect("이름이 선다");
            let required = t.input_schema["required"].as_array().expect("required 는 배열");
            assert_eq!(
                required.len(),
                q.arg_names().len(),
                "{} 의 required 가 인자 수와 다르다",
                q.name()
            );
        }
    }
}
