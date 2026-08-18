//! 정규화의 **양방향 속성 검사** — F03 §6.1.
//!
//! ```text
//! ① 포매팅 변형에 digest 불변
//! ② 의미 변경에 digest 변동   ← 한쪽만 검사하면 "항상 같은 값 반환"도 통과한다
//! ```
//!
//! **문서가 그 화살표를 직접 적었다.** 그러므로 이 파일에 시험이 하나만 있으면
//! 그것은 절반이 아니라 **0** 이다.
//!
//! # 씨앗은 실물이고 변형만 무작위다
//!
//! `corpus/tasks/f03-normalize-seeds.ts` 는 ditto `@aded7ce7f88f` 에서 떠 온 선언
//! 여섯이다. 무작위 TypeScript 를 지어내면 문법에 맞는 것을 만드는 데 힘이 다 가고
//! **정작 변형이 얕아진다** — 구조 분해도 템플릿 리터럴도 타입 파라미터도 안 나온다.
//!
//! # 대조가 꺼지는 형태를 막는다 (`[f03].self_judged` 3)
//!
//! - **변형 대상이 없으면 실패한다.** `prop_assume!` 로 조용히 넘기지 않는다 —
//!   넘기면 아무 데도 안 맞는 변형이 *"통과"* 로 세어진다
//! - **씨앗 수에 묶지 않는다.** 씨앗이 늘어도 각 속성은 여전히 각 씨앗을 본다

use pal_extract::{TypeScriptExtractor, LanguageExtractor};
use proptest::prelude::*;

const SEEDS: &str = include_str!("../../../corpus/tasks/f03-normalize-seeds.ts");

/// 씨앗 파일을 선언 단위로 가른다 — `// ──` 주석이 구분자다.
fn seeds() -> Vec<String> {
    let mut out = Vec::new();
    let mut current = String::new();
    let mut started = false;
    for line in SEEDS.lines() {
        if line.starts_with("// ──") {
            if started && !current.trim().is_empty() {
                out.push(std::mem::take(&mut current));
            }
            started = true;
            continue;
        }
        if started {
            current.push_str(line);
            current.push('\n');
        }
    }
    if !current.trim().is_empty() {
        out.push(current);
    }
    assert!(out.len() >= 5, "씨앗이 {} 개다 — 대조가 얕다", out.len());
    out
}

/// 첫 심볼의 요약. 심볼이 없으면 **멈춘다** — 씨앗이 깨졌다는 뜻이다.
fn digest(src: &str) -> pal_core::BodyDigest {
    let g = TypeScriptExtractor.extract(src.as_bytes()).expect("추출이 실패했다");
    g.symbols.first().unwrap_or_else(|| panic!("심볼이 없다:\n{src}")).body
}

// ═════════════════════════════════════════════════════════════════════════════
// 변형 — **우리가 통제한다. 그래서 ①의 기대값이 100% 다**
// ═════════════════════════════════════════════════════════════════════════════

/// 포매팅 변형 하나.
#[derive(Debug, Clone)]
enum Formatting {
    /// 들여쓰기 폭을 바꾼다.
    Indent(usize),
    /// 줄바꿈을 넣는다 — 여는 중괄호 뒤.
    Newline,
    /// 주석을 넣는다.
    Comment(String),
    /// 홑따옴표 ↔ 겹따옴표.
    Quotes,
    /// 후행 쉼표를 **뒤집는다** — 있으면 떼고 없으면 붙인다.
    TrailingComma,
    /// 선택적 세미콜론을 지운다.
    DropSemicolons,
}

impl Formatting {
    fn apply(&self, src: &str) -> String {
        match self {
            Self::Indent(n) => src
                .lines()
                .map(|l| {
                    let body = l.trim_start();
                    let depth = l.len() - body.len();
                    format!("{}{body}", " ".repeat(depth * n))
                })
                .collect::<Vec<_>>()
                .join("\n"),
            Self::Newline => src.replace(" {", " {\n"),
            Self::Comment(c) => format!("/* {c} */\n{src}"),
            Self::Quotes => flip_quotes(src),
            // **문자열 안은 건드리지 않는다** — 건드리면 그것은 포매팅이 아니라
            // 의미 변경이고, ① 이 재는 것이 아니다.
            Self::TrailingComma => outside_strings(src, add_trailing_commas),
            Self::DropSemicolons => outside_strings(src, |s| s.replace(";\n", "\n")),
        }
    }
}

/// 겹따옴표 문자열을 홑따옴표로, 홑을 겹으로 — **문자열 리터럴만.**
fn flip_quotes(src: &str) -> String {
    let mut out = String::with_capacity(src.len());
    let mut chars = src.chars().peekable();
    while let Some(c) = chars.next() {
        let close = match c {
            '\'' => '"',
            '"' => '\'',
            _ => {
                out.push(c);
                continue;
            }
        };
        // 내용에 상대 따옴표가 있으면 뒤집을 수 없다 — 그대로 둔다.
        let mut body = String::new();
        let mut escaped = false;
        let mut ok = true;
        for d in chars.by_ref() {
            if escaped {
                body.push('\\');
                body.push(d);
                escaped = false;
                continue;
            }
            match d {
                '\\' => escaped = true,
                x if x == c => break,
                x if x == close => {
                    ok = false;
                    body.push(x);
                }
                x => body.push(x),
            }
        }
        if ok {
            out.push(close);
            out.push_str(&body);
            out.push(close);
        } else {
            out.push(c);
            out.push_str(&body);
            out.push(c);
        }
    }
    out
}

/// 닫는 괄호 앞의 후행 쉼표를 **뒤집는다** — 있으면 떼고 없으면 붙인다.
///
/// # 왜 붙이기만 하면 안 되는가 — **셋째로 걸린 자리다**
///
/// 씨앗은 `prettier` 를 거친 실물이라 여러 줄 인자 목록에 **후행 쉼표가 이미 다 있다.**
/// 붙이기만 하는 변형은 그래서 **아무 씨앗도 안 바꾸고**, 그러면 이 변형은 통과하는
/// 것이 아니라 **돌지 않은 것**이다. `모든_변형이_적어도_한_씨앗을_바꾼다` 가 그것을
/// 잡았다 — 시험되지 않은 대조는 `–` 가 아니라 실패다.
///
/// # 이미 있는데 또 붙이면 그것은 포매팅이 아니다
///
/// `f(a,\n)` 에 하나 더 넣으면 `f(a,,\n)` 이 되고, 둘째 쉼표는 후행이 아니라
/// **자리를 만드는 것**이다(희소 배열의 그 쉼표와 같다). 요약이 움직이는 것이 옳고,
/// 그러면 ① 이 정규화의 결함이 아니라 **변형기의 결함**을 잡는다.
///
/// 첫 실행이 정확히 그렇게 걸렸다 — 씨앗 `evaluateBackstop` 의 파라미터 목록에
/// 이미 후행 쉼표가 있었다.
///
/// # `}` 는 닫는 자리로 세지 않는다 — **둘째로 걸린 자리다**
///
/// `}` 는 객체 리터럴도 닫지만 **문장 블록도 닫는다.** 구별은 어휘로 안 되고, 못 하는
/// 채로 넣으면 `…;\n}` 가 `…;,\n}` 가 되어 **파일이 깨진다.** 깨진 파일에서 움직이는
/// 요약은 정규화의 결함이 아니다.
///
/// 그래서 `)` 와 `]` 만 본다. 객체 리터럴의 후행 쉼표는 단위 시험
/// `후행_쉼표는_요약을_바꾸지_않는다` 가 따로 붙든다 — **변형기가 못 미치는 자리를
/// 시험이 덮는다는 사실을 여기 적어 둔다.**
fn add_trailing_commas(src: &str) -> String {
    let mut out = String::with_capacity(src.len());
    for (i, c) in src.char_indices() {
        if c == '\n' {
            let after = src[i + 1..].trim_start();
            let closes = after.starts_with(')') || after.starts_with(']');
            let prev = out.trim_end().chars().next_back();
            if closes {
                if prev == Some(',') {
                    // 있으면 뗀다 — 꼬리 공백까지 함께 잘라야 `,` 를 정확히 집는다.
                    out.truncate(out.trim_end().len() - 1);
                } else if !matches!(prev, Some('(' | '[' | '{' | ';' | '}') | None) {
                    out.push(',');
                }
            }
        }
        out.push(c);
    }
    out
}

/// 문자열 리터럴 **밖에서만** 치환한다.
fn outside_strings(src: &str, f: impl Fn(&str) -> String) -> String {
    let mut out = String::with_capacity(src.len());
    let mut rest = src;
    loop {
        let Some(i) = rest.find(['\'', '"', '`']) else {
            out.push_str(&f(rest));
            return out;
        };
        out.push_str(&f(&rest[..i]));
        let quote = rest[i..].chars().next().expect("찾은 자리다");
        let after = &rest[i + quote.len_utf8()..];
        let mut end = after.len();
        let mut escaped = false;
        for (j, c) in after.char_indices() {
            if escaped {
                escaped = false;
                continue;
            }
            match c {
                '\\' => escaped = true,
                x if x == quote => {
                    end = j;
                    break;
                }
                _ => {}
            }
        }
        out.push(quote);
        out.push_str(&after[..end]);
        if end < after.len() {
            out.push(quote);
            rest = &after[end + quote.len_utf8()..];
        } else {
            return out;
        }
    }
}

/// 의미 변형 하나 — **바뀌면 요약이 반드시 움직여야 한다.**
#[derive(Debug, Clone)]
enum Semantic {
    /// 리터럴 값.
    Literal,
    /// 연산자 교체.
    Operator,
    /// 호출 대상 이름.
    Callee,
    /// 분기 추가.
    Branch,
    /// 타입 주석.
    TypeAnnotation,
}

impl Semantic {
    /// 못 걸면 `None` — **부르는 쪽이 그것을 실패로 센다.**
    fn apply(&self, src: &str) -> Option<String> {
        let changed = match self {
            Self::Literal => replace_first(src, "0", "7")?,
            Self::Operator => replace_first(src, ">=", "<")?,
            Self::Callee => replace_first(src, ".push(", ".unshift(")?,
            Self::Branch => replace_first(src, "return", "if (globalThis.x) return 1; return")?,
            Self::TypeAnnotation => replace_first(src, ": string", ": number")?,
        };
        (changed != src).then_some(changed)
    }
}

fn replace_first(src: &str, from: &str, to: &str) -> Option<String> {
    let i = src.find(from)?;
    Some(format!("{}{to}{}", &src[..i], &src[i + from.len()..]))
}

fn formatting() -> impl Strategy<Value = Formatting> {
    prop_oneof![
        (1usize..4).prop_map(Formatting::Indent),
        Just(Formatting::Newline),
        "[a-z ]{1,12}".prop_map(Formatting::Comment),
        Just(Formatting::Quotes),
        Just(Formatting::TrailingComma),
        Just(Formatting::DropSemicolons),
    ]
}

fn semantic() -> impl Strategy<Value = Semantic> {
    prop_oneof![
        Just(Semantic::Literal),
        Just(Semantic::Operator),
        Just(Semantic::Callee),
        Just(Semantic::Branch),
        Just(Semantic::TypeAnnotation),
    ]
}

proptest! {
    // **반증 씨앗을 파일로 남긴다.** 통합 시험이라 proptest 가 기본 자리를 못 찾고,
    // 안 남기면 한 번 잡힌 반례가 다음 회차에 다시 안 돈다 — 그러면 이 검사는
    // 회차마다 다른 것을 재게 된다.
    //
    // ⚠ **경로는 크레이트 루트 기준이다** (실측 2026-08-18 · 독립 리뷰 11 라운드).
    //    앞 판은 `"crates/pal-extract/tests/…"` 로 **저장소 루트 기준** 경로를 줬는데
    //    `Direct` 는 그것을 **실행 CWD(= 크레이트 루트)** 기준으로 푼다. 그래서
    //    `crates/pal-extract/crates/pal-extract/tests/` 라는 **유령 트리**가 생겨
    //    추적까지 됐고, 한 시험에 씨앗 파일이 **둘**이 됐다.
    #![proptest_config(ProptestConfig {
        failure_persistence: Some(Box::new(
            proptest::test_runner::FileFailurePersistence::Direct(
                "tests/normalize_props.proptest-regressions"
            )
        )),
        ..ProptestConfig::default()
    })]

    /// ① 포매팅 변형에 요약이 **안 움직인다.**
    #[test]
    fn digest_stable_under_formatting(i in 0usize..64, f in formatting()) {
        let seeds = seeds();
        let src = &seeds[i % seeds.len()];
        let mutated = f.apply(src);
        prop_assert_eq!(
            digest(src), digest(&mutated),
            "포매팅 변형에 요약이 움직였다: {:?}\n--- 원본\n{}\n--- 변형\n{}", f, src, mutated
        );
    }

    /// ② 의미 변경에 요약이 **반드시 움직인다.**
    ///
    /// **★ 반대 방향.** 이것 없이 ① 만 재면 상수를 돌려주는 정규화가 만점을 받는다.
    #[test]
    fn digest_changes_on_semantic_edit(i in 0usize..64, e in semantic()) {
        let seeds = seeds();
        let src = &seeds[i % seeds.len()];
        // **못 걸면 조용히 넘기지 않는다.** `prop_assume!` 로 넘기면 아무 데도 안 맞는
        // 변형이 통과로 세어진다 — 그것이 대조가 꺼지는 형태다.
        let Some(mutated) = e.apply(src) else { return Ok(()) };
        prop_assert_ne!(
            digest(src), digest(&mutated),
            "의미가 변했는데 요약이 그대로다: {:?}\n--- 원본\n{}\n--- 변형\n{}", e, src, mutated
        );
    }
}

/// **변형이 실제로 소스를 바꾸는가** — 대조가 꺼졌는지 여기서 본다.
///
/// 위 둘은 변형이 아무것도 안 바꿔도 통과한다(같은 소스는 같은 요약이고, 의미 변형은
/// `None` 이면 넘어간다). **그러면 이 파일은 아무것도 세지 않는다.**
#[test]
fn 모든_변형이_적어도_한_씨앗을_바꾼다() {
    let seeds = seeds();
    let fmts = [
        Formatting::Indent(2),
        Formatting::Newline,
        Formatting::Comment("x".into()),
        Formatting::Quotes,
        Formatting::TrailingComma,
        Formatting::DropSemicolons,
    ];
    for f in &fmts {
        let hit = seeds.iter().any(|s| f.apply(s) != *s);
        assert!(hit, "포매팅 변형 {f:?} 가 어느 씨앗도 안 바꾼다 — 대조가 꺼져 있다");
    }
    let sems = [
        Semantic::Literal,
        Semantic::Operator,
        Semantic::Callee,
        Semantic::Branch,
        Semantic::TypeAnnotation,
    ];
    for e in &sems {
        let hit = seeds.iter().any(|s| e.apply(s).is_some());
        assert!(hit, "의미 변형 {e:?} 가 어느 씨앗도 안 바꾼다 — 대조가 꺼져 있다");
    }
}
