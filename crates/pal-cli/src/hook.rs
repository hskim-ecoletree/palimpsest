//! `pal hook <event>` — **하네스가 부르는 자리**(`[f24]` ⑧).
//!
//! # 이 파일이 지는 것은 규약이고, 정책은 [`policy`] 가 진다
//!
//! 게이트 ⑧ 은 *"등록 형태를 합격선에 안 박는다"* 고 적었고, 같은 이유로 **정책도
//! 여기서 굳히지 않는다.** 이 회차의 정책은 최소이고 나중에 갈아끼워진다 — 그래서
//! 갈아끼우는 자리를 파일로 갈라 뒀다. 여기 남는 것은 **바이트와 종료 코드의 규약**뿐이다.
//!
//! # 실측된 규약 — 이 파일이 그 위에 선다
//!
//! - 훅 커맨드는 **`/bin/sh -c "<등록 문자열 원문>"`** 으로 실행된다. 바이너리 훅과 셸
//!   스크립트 훅은 **stdin 바이트가 SHA-256 까지 동일**하다.
//! - `SubagentStop` 의 차단은 넷 중 하나로 전달되고, 우리가 쓰는 것은 **`exit 0` +
//!   표준출력 `{"decision":"block","reason":…}`** 이다. `{"continue":false}` ·
//!   `exit 1/3/42` · plain text 는 **안 먹는다.**
//! - **`exit 0` 이면 표준오류는 무시되고 표준출력 JSON 이 이긴다.** 그래서 진단을
//!   표준오류에 늘 적어도 사용자의 작업에 안 섞인다.
//!
//! # ★ 가장 나쁜 실패는 훅이 오작동해서 사람의 작업을 막는 것이다
//!
//! 그래서 이 파일의 모든 실패 경로는 **조용한 통과**다 — 깨진 입력 · 모르는 사건 ·
//! 못 읽은 표준입력 어디서도 차단하지 않고, 까닭만 표준오류에 남긴다.
//!
//! # ★ 그리고 훅 실행 실패는 침묵한다
//!
//! 실행 파일이 없으면 **exit 127**, 실행 권한이 없으면 **exit 126** 인데 세션은 계속되고
//! `claude` 의 종료 코드는 **0** 이다. 트랜스크립트에도 대화형 화면에도 흔적이 없다.
//! 그래서 훅은 **부르면 언제나 표준오류에 한 줄**을 남기고, `pal doctor` 의 검사가
//! 그 줄을 찾아 「등록된 명령이 실제로 도는가」를 가른다.

mod policy;

use std::io::Read;

use serde_json::{Value, json};

pub use policy::EVENTS;

/// 부르면 언제나 표준오류에 나오는 표식. **`pal doctor` 가 이것을 찾는다.**
pub const ACK: &str = "pal hook";

/// 훅 하나를 처리한다.
///
/// **실패를 반환하지 않는다 — 타입이 그것을 진다.** 여기서 `Result` 를 내면 언젠가
/// `?` 하나가 섞이고, 그 하나가 종료 코드 1 이 되며, 실측상 **표준출력이 유효 JSON 이
/// 아닌 exit≠0 은 그대로 실패로 분류된다.** 훅의 실패는 사람의 작업을 막는다.
pub fn run(event: &str) {
    let mut raw = String::new();
    if let Err(e) = std::io::stdin().read_to_string(&mut raw) {
        적는다(event, &format!("통과 — 표준입력을 못 읽었다: {e}"));
        return;
    }
    let payload: Value = match serde_json::from_str(&raw) {
        Ok(v) => v,
        Err(e) => {
            적는다(event, &format!("통과 — 페이로드가 JSON 이 아니다: {e}"));
            return;
        }
    };

    match policy::decide(event, &payload) {
        policy::Decision::Pass(why) => 적는다(event, &format!("통과 — {why}")),
        policy::Decision::Block(reason) => {
            적는다(event, &format!("차단 — {reason}"));
            // **표준출력 JSON 이 이긴다.** 한 줄로 낸다 — 실측이 그 형태로 먹었다.
            println!("{}", json!({ "decision": "block", "reason": reason }));
        }
    }
}

/// 표준오류 한 줄 — **어느 빌드가 무엇을 판정했는가.**
fn 적는다(event: &str, what: &str) {
    eprintln!("{ACK} {event}  ·  pal {}  ·  {what}", crate::version::describe());
}
