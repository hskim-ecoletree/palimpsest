//! 봉투의 성분 셋을 사람이 읽는 화면에 적는다 — **접힘 · 로그 · 크기** (F06 §4.3).
//!
//! # 왜 한 곳인가
//!
//! 표면이 셋이다(`pal query`·`pal touch`·`pal doctor`). 각자 적으면 하나가 빠져도
//! 아무도 모르고, **빠진 것은 소비자가 셀 수 없다.** F05 가 `print_elision` 을 세우면서
//! 적은 규율이 그대로다 — *"산출에만 있고 화면에 없으면 사람은 그 공백을 못 본다."*
//!
//! # 접힘과 절단을 **다른 줄**에 적는다
//!
//! 한 줄에 뭉개면 *"부피를 옮겼다"* 와 *"못 봤다"* 가 같은 문장이 된다.
//! `[f06].fold_is_not_elision` 이 타입에서 가른 것을 화면에서도 가른다.

use pal_core::{Envelope, LogStatus, NotRecorded};

/// 접힌 것 · 로그 상태 · 대략적 크기를 적는다.
pub fn print<T>(e: &Envelope<T>) {
    if e.fold.is_none() {
        println!("  접힘      없음 (명시)");
    } else {
        println!("  접힘      {}건이 다른 질의로 옮겨졌습니다 — **잘린 것이 아닙니다**", e.fold.moved());
        for f in &e.fold.folded {
            println!("            {} {}건 → `{}` 가 폅니다", f.what.name(), f.count, f.unfolded_by.name());
        }
    }

    match e.log {
        LogStatus::Recorded => println!("  질의 로그  남았습니다"),
        LogStatus::NotRecorded { why } => {
            let 사유 = match why {
                // 조용히 안 남기면 F17 이 미조회를 **과대 계상**한다. 그래서 화면에도 적는다.
                NotRecorded::ReadOnlyAttach => "2층에 읽기 전용으로 붙어 남기지 못했습니다",
                NotRecorded::SurfaceDoesNotLog => "이 표면은 아직 질의 로그를 쓰지 않습니다",
            };
            println!("  질의 로그  **안 남았습니다** — {사유}");
        }
    }

    // **하한이라고 적는다.** 재는 것은 빈틈 없는 JSON 이고, 이 화면처럼 들여쓴
    // 산출은 그보다 크다. 하한임을 안 적으면 소비자가 상한으로 읽는다(백서 §6.3).
    println!(
        "  크기      약 {} 토큰 **이상** (잰 것: {} 바이트 · 가정: {} 바이트/토큰)",
        e.tokens.approx_tokens, e.tokens.serialized_bytes, e.tokens.bytes_per_token
    );
}
