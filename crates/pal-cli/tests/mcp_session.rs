//! **실 MCP 세션** — `[f06b.pass]` ①②③⑧.
//!
//! # 왜 손으로 만든 JSON 을 파이프에 붓지 않는가
//!
//! 그것은 프로토콜이 아니라 **우리가 그 프로토콜이라고 믿는 것**을 잰다. 초기화 순서
//! 하나, 알림 하나가 틀려도 우리 손으로 만든 것은 그대로 통과한다. 그래서 여기서는
//! **같은 SDK 의 클라이언트**가 자식 프로세스(`pal serve`)를 몰고, 그 왕복이 성립하는
//! 것을 잰다.
//!
//! # 이 파일 전체가 feature 뒤에 있다
//!
//! 어댑터를 뺀 빌드에는 `pal serve` 가 없다. 그때 이 시험이 컴파일되면 **없는 것을
//! 부르고 실패하고**, 그 실패가 *"어댑터 부재 빌드가 안 선다"* 로 읽힌다 — ⑥ 이 재려는
//! 것과 정반대다.
#![cfg(feature = "mcp")]

mod common;

use common::저장소;
use rmcp::ServiceExt;
use rmcp::model::CallToolRequestParams;
use rmcp::transport::TokioChildProcess;

const PAL: &str = env!("CARGO_BIN_EXE_pal");

/// 카탈로그 정본. **런타임에 안 읽히지만 시험은 읽는다** — `catalog_surface.rs` 와 같은 자격.
const 카탈로그: &str = include_str!("../../../surface/queries.toml");

/// **하한** — 이보다 적으면 아래 세 방향이 공짜로 통과한다.
/// `CATALOG_MIN_QUERIES` 와 같은 수이고 같은 이유다.
const 최소_툴: usize = 6;

/// 세션 하나를 열고 클로저에 넘긴다. **저장소마다 다른 방**을 쓴다(`common` 의 규율).
async fn 세션<F, Fut, T>(tag: &str, f: F) -> T
where
    F: FnOnce(rmcp::service::RunningService<rmcp::RoleClient, ()>, std::path::PathBuf) -> Fut,
    Fut: Future<Output = T>,
{
    let repo = 저장소(tag);
    let mut cmd = tokio::process::Command::new(PAL);
    cmd.arg("serve").arg("--repo").arg(&repo);
    let client = ()
        .serve(TokioChildProcess::new(cmd).expect("자식 프로세스를 못 띄웠다"))
        .await
        .expect("MCP 세션이 안 섰다");
    let out = f(client, repo).await;
    out
}

/// ① **왕복한다** — `initialize` 는 `serve` 가 이미 했고, 여기서 `tools/list` 와
/// `tools/call` 을 **인자 없는 것 하나와 인자 받는 것 하나**로 각각 친다.
///
/// 둘의 형태가 다르므로(인자 스키마) 하나만 재면 다른 하나가 안 재어진다.
#[tokio::test]
async fn 세션이_왕복한다() {
    세션("mcp-roundtrip", |client, _repo| async move {
        let tools = client.list_tools(None).await.expect("tools/list");
        assert!(tools.tools.len() >= 최소_툴, "툴이 {}개다 — 하한 미만", tools.tools.len());

        // 인자 없는 질의 — 대장.
        let 대장 = client
            .call_tool_once(요청(pal_core::QueryName::LedgerSnapshot.name(), None))
            .await
            .expect("인자 없는 질의가 답해야 한다");
        let 본문 = 본문을_뜬다(&대장);
        let v: serde_json::Value = serde_json::from_str(&본문).expect("봉투가 JSON 이다");
        assert!(v.get("answer").is_some(), "봉투에 answer 가 없다");
        assert!(v.get("capabilities").is_some(), "봉투에 capabilities 가 없다");

        // 인자 받는 질의 — 코퍼스에 실제로 있는 이름.
        let 심볼 = client
            .call_tool_once(요청(
                pal_core::QueryName::SymbolResolve.name(),
                Some(serde_json::json!({ "name": "도움" })),
            ))
            .await
            .expect("인자 받는 질의가 답해야 한다");
        let v: serde_json::Value =
            serde_json::from_str(&본문을_뜬다(&심볼)).expect("봉투가 JSON 이다");
        assert!(v.get("answer").is_some(), "봉투에 answer 가 없다");

        client.cancel().await.expect("세션을 닫는다");
    })
    .await;
}

/// ② ★ **음성 대조 — 없는 툴 이름은 오류다.**
///
/// 성공이 오거나 빈 답이 오면 ①의 왕복은 *"무엇을 불러도 답이 온다"* 를 잰 것이고,
/// 그것은 표면이 아니라 메아리다.
///
/// ⚠ **rc 가 아니라 응답으로 잰다** — MCP 는 프로세스 종료 코드로 답하지 않는다.
#[tokio::test]
async fn 없는_툴_이름은_오류다() {
    세션("mcp-unknown", |client, _repo| async move {
        let r = client
            .call_tool_once(요청("no.such.query", None))
            .await;
        assert!(r.is_err(), "모르는 이름에 답이 왔다 — 표면이 메아리다: {r:?}");
        client.cancel().await.expect("세션을 닫는다");
    })
    .await;
}

/// ③ **툴 목록과 카탈로그가 양방향으로 같다** — 그리고 인자까지.
///
/// ⚠ **방향마다 루프를 따로 돈다**(`[f06.1.pass]` 와 같은 규율). 한 루프에서 두 방향을
/// 돌면 한쪽의 `continue` 가 다른 쪽을 끈다.
#[tokio::test]
async fn 툴_목록이_카탈로그와_양방향으로_같다() {
    세션("mcp-catalog", |client, _repo| async move {
        let c = pal_core::QueryCatalog::parse(카탈로그).expect("카탈로그가 읽힌다");
        let tools = client.list_tools(None).await.expect("tools/list");
        assert!(tools.tools.len() >= 최소_툴, "툴이 {}개다 — 하한 미만", tools.tools.len());

        let 툴_이름: Vec<&str> = tools.tools.iter().map(|t| t.name.as_ref()).collect();

        // ── 방향 1 — 카탈로그에 있는데 툴에 없다 ────────────────────────────
        for name in c.queries.keys() {
            assert!(
                툴_이름.contains(&name.as_str()),
                "카탈로그의 `{name}` 이 MCP 표면에 없다"
            );
        }

        // ── 방향 2 — 툴에 있는데 카탈로그에 없다 ────────────────────────────
        for name in &툴_이름 {
            assert!(
                c.queries.contains_key(*name),
                "MCP 가 `{name}` 을 내는데 카탈로그에 없다"
            );
        }

        // ── 방향 3 — 이름은 같은데 인자가 어긋난다 ──────────────────────────
        for t in &tools.tools {
            let decl = &c.queries[t.name.as_ref()];
            let required: Vec<String> = t.input_schema["required"]
                .as_array()
                .expect("required 는 배열")
                .iter()
                .map(|v| v.as_str().expect("이름은 문자열").to_owned())
                .collect();
            let 카탈로그의_인자: Vec<String> =
                decl.args.iter().map(|a| a.name.clone()).collect();
            assert_eq!(
                required, 카탈로그의_인자,
                "`{}` 의 인자가 카탈로그와 어긋난다",
                t.name
            );
        }

        client.cancel().await.expect("세션을 닫는다");
    })
    .await;
}

/// ⑧ **응답 크기 — 잰 것과 가정한 것이 갈려 있다.**
///
/// ⚠ **이 시험은 상한을 안 세운다**(운영 순서 4 · 합격선 상향 금지). 재는 것은
/// *"두 값이 분리되어 있는가"* 와 *"잰 값이 실제로 나간 바이트와 맞는가"* 다.
///
/// 실측치는 `--nocapture` 로 뜬다 — 게이트 문서 §판정 이 그 표를 진다.
#[tokio::test]
async fn 응답_크기가_잰_것과_가정한_것으로_갈려_있다() {
    세션("mcp-size", |client, _repo| async move {
        for q in [pal_core::QueryName::LedgerSnapshot, pal_core::QueryName::BindingStatus] {
            let r = client
                .call_tool_once(요청(q.name(), None))
                .await
                .expect("답이 온다");
            let 본문 = 본문을_뜬다(&r);
            let v: serde_json::Value = serde_json::from_str(&본문).expect("봉투가 JSON 이다");
            let t = &v["tokens"];

            let 잰_것 = t["serialized_bytes"].as_u64().expect("serialized_bytes 가 있다");
            let 가정 = t["bytes_per_token"].as_u64().expect("bytes_per_token 이 있다");
            let 추정 = t["approx_tokens"].as_u64().expect("approx_tokens 가 있다");

            // **셋이 다른 필드다.** 하나로 뭉개져 있으면 소비자가 어디까지 믿을지 모른다.
            assert!(잰_것 > 0, "{} 의 serialized_bytes 가 0 이다", q.name());
            assert!(가정 > 0, "{} 의 bytes_per_token 이 0 이다", q.name());
            assert_eq!(추정, 잰_것 / 가정, "{} 의 approx_tokens 가 파생이 아니다", q.name());

            // **신고한 크기가 실제로 나간 바이트와 맞는가.** 봉투는 자기 자신을 뺀
            // 나머지를 재므로 실제 본문이 조금 더 크다 — 그러나 **작을 수는 없다.**
            assert!(
                본문.len() >= 잰_것 as usize,
                "{} 의 신고 크기({})가 실제로 나간 바이트({})보다 크다",
                q.name(),
                잰_것,
                본문.len()
            );

            eprintln!(
                "[f06b ⑧] {:<18} 실제 {:>7} B · 신고 {:>7} B · 가정 {} B/tok · 추정 {} tok",
                q.name(),
                본문.len(),
                잰_것,
                가정,
                추정
            );
        }
        client.cancel().await.expect("세션을 닫는다");
    })
    .await;
}

/// 툴 호출 하나 — **`CallToolRequestParams` 가 `non_exhaustive` 라** 필드 대입으로 만든다.
fn 요청(name: &str, arguments: Option<serde_json::Value>) -> CallToolRequestParams {
    let mut p = CallToolRequestParams::default();
    p.name = name.to_owned().into();
    p.arguments = arguments.map(|v| v.as_object().expect("인자는 객체다").clone());
    p
}

/// 답의 본문 — **텍스트 블록 하나**를 꺼낸다.
fn 본문을_뜬다(r: &rmcp::model::CallToolResponse) -> String {
    let rmcp::model::CallToolResponse::Complete(result) = r else {
        panic!("답이 결과가 아니다: {r:?}");
    };
    assert_eq!(result.is_error, Some(false), "오류로 왔다: {result:?}");
    let block = result.content.first().expect("본문이 비었다");
    match block {
        rmcp::model::ContentBlock::Text(t) => t.text.clone(),
        other => panic!("본문이 텍스트가 아니다: {other:?}"),
    }
}
