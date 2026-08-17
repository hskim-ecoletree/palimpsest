//! 계획 문서 인입 — **조각화는 [F10] 의 것을 그대로 쓰고, 좌표 추출만 F12 가 세운다**.
//!
//! # 왜 [F10] 의 신호 규칙을 그대로 못 쓰는가 — **실측이 그것을 막았다**
//!
//! [`crate::fragment`] 는 경로를 **펜스 안에서만**, 식별자를 **인라인 스팬에서만**
//! 모은다. 그것이 F10 에서 옳았던 이유는 **문서가 코드를 설명하는 글**이기 때문이다 —
//! 산문 한가운데의 낱말을 좌표로 삼으면 거짓 결박이 난다([ADR-0015]).
//!
//! **계획 문서는 다른 글이다.** ditto 의 계획 항목 **575** 중 백틱을 쓴 것이 **5** 건이고
//! (`[f12].measurement_already_seen`), F10 의 규칙을 그대로 대면 좌표 후보가 거의 0 이
//! 된다 — 그리고 **그 0 은 코퍼스의 성질이 아니라 우리가 고른 규칙의 성질이다.**
//!
//! 그래서 F12 는 범위를 넓히고 **넓혔다는 사실을 값에 싣는다**
//! ([`PatternSource`]). 게이트가 신호별로 갈라 세고, 잡음은
//! `[f12.pass]` ③(짝을 섞으면 떨어진다)이 잡는다.
//!
//! # 여기에 판단이 없다
//!
//! 이 모듈이 내는 것은 **패턴**이지 좌표가 아니다. *"이 이름이 기준선에 서는가"* 는
//! [`pal_core::resolve_plan`] 가 묻고, 그것이 [ADR-0019] 의 자격 검사다.
//! 조각화가 2 층을 안 타는 것과 같은 갈림이다.
//!
//! [F10]: ../../../docs/plan/disposal-map.md
//! [ADR-0015]: ../../../docs/adr/0015-a-machine-confirmed-signal-must-say-what-it-confirmed.md
//! [ADR-0019]: ../../../docs/adr/0019-the-site-of-the-repair-is-not-the-site-of-the-defect.md

use pal_core::{
    CoordPattern, Glob, PatternSource, Plan, PlanBaseline, PlanId, PlanItem, PlanItemId,
    PlanRefusal, RepoPath, VerificationStep,
};

/// 계획이 좌표를 **명시**하는 줄의 머리 — [F12 §4] 의 *"좌표를 요구하는 템플릿"*.
const 좌표_머리: [&str; 2] = ["좌표:", "coords:"];
/// 판정 방법을 적는 줄의 머리.
const 검증_머리: [&str; 2] = ["검증:", "verify:"];
/// 「아직 없다」를 명시하는 표시. **명시가 조건이다** — 자동 승격은 답을 보고 분류하는 것이다.
const 신규_표시: [&str; 2] = ["(신규)", "(new)"];

/// 마크다운 하나를 계획으로 읽는다.
///
/// # 항목은 무엇인가 — **한 규칙이다**
///
/// > **문서에 체크박스가 하나라도 있으면 항목은 체크박스 줄이고, 없으면 헤딩 조각이다.**
///
/// [F12 §3.4] 가 *"`docs/plan/features/*.md` 의 **완료 체크리스트 항목들**이 첫 `Plan`"*
/// 이라고 적었고, ditto 의 work item 계획에는 체크박스가 없다. 두 규칙을 섞으면
/// 같은 문서가 두 가지로 읽히므로 **하나로 정하고 그 하나가 둘을 덮는다.**
///
/// 첫 조각은 **기획**이다(§3.3 의 2 단 중 위) — 항목이 아니다.
///
/// # Errors
/// 항목이 하나도 없으면 [`PlanRefusal::NoItems`]. ★ **그 거부가 §3.3 의 1 단
/// (기획→결정) 미해소를 세는 자리다** — 조용히 빈 계획을 만들면 그 수가 사라진다.
pub fn ingest_plan(path: &RepoPath, source: &str) -> Result<Plan, PlanRefusal> {
    let frags = crate::fragment(path, source);
    let baseline = baseline_of(source);
    let headline = frags
        .first()
        .and_then(|f| f.body.lines().find(|l| !l.trim().is_empty()))
        .unwrap_or_else(|| path.as_str())
        .trim()
        .to_owned();

    let plan_id = PlanId::derive(path, &headline);
    let 체크박스가_있다 = frags.iter().any(|f| f.body.lines().any(|l| 체크박스(l).is_some()));

    let mut items = Vec::new();
    if 체크박스가_있다 {
        for f in &frags {
            let mut n = 0usize;
            for line in f.body.lines() {
                let Some(text) = 체크박스(line) else { continue };
                n += 1;
                items.push(항목(&plan_id, &format!("{}#{n}", f.anchor), text, text));
            }
        }
    } else {
        // 첫 조각은 기획이다. **머리말 하나뿐인 문서는 항목이 0 이고 거부된다.**
        for f in frags.iter().skip(1) {
            // ★ **첫 줄은 헤딩이다.** [`crate::fragment`] 가 헤딩 텍스트를 본문 앞에
            // 넣으므로(그래야 `제목본문` 이 한 낱말로 안 붙는다), 그대로 쓰면 모든
            // 항목의 문장이 **제목**이 된다. 좌표는 헤딩에서도 뽑으므로 **본문 전체**를
            // 넘기고 문장만 헤딩 다음 줄부터 고른다.
            items.push(항목(&plan_id, &f.anchor, &f.body, 헤딩_다음(&f.body)));
        }
    }

    Plan::new(path.clone(), headline, baseline, items)
}

/// `- [ ] 무엇` · `[x] 무엇` → `무엇`. **표시는 안 본다** — 체크 여부는 계획이 아니라
/// 그 문서를 쓴 사람의 주장이고, 이 기능이 재는 것은 **실제 변경**이다.
///
/// ⚠ **목록 표시(`- `·`* `)는 대개 이미 떼어져 있다.** [`crate::fragment`] 가
/// `pulldown-cmark` 의 이벤트에서 본문을 짓기 때문이다 — 그래서 둘 다 받는다.
/// (`ENABLE_TASKLISTS` 는 안 켜져 있으므로 `[ ]` 는 본문에 글자로 남는다.)
fn 체크박스(line: &str) -> Option<&str> {
    let t = line.trim_start();
    let t = t.strip_prefix("- ").or_else(|| t.strip_prefix("* ")).unwrap_or(t);
    let rest = t
        .strip_prefix("[ ]")
        .or_else(|| t.strip_prefix("[x]"))
        .or_else(|| t.strip_prefix("[X]"))?;
    let text = rest.trim();
    if text.is_empty() { None } else { Some(text) }
}

/// 헤딩 줄을 뗀 나머지. 헤딩뿐인 조각에서는 **헤딩 자신이 문장이다.**
fn 헤딩_다음(body: &str) -> &str {
    match body.split_once('\n') {
        Some((_, rest)) if !rest.trim().is_empty() => rest,
        _ => body,
    }
}

fn 항목(plan: &PlanId, anchor: &str, text: &str, 문장원: &str) -> PlanItem {
    let statement =
        문장원.lines().find(|l| !l.trim().is_empty()).unwrap_or("").trim().to_owned();
    PlanItem {
        id: PlanItemId::derive(plan, anchor, &statement),
        anchor: anchor.to_owned(),
        statement,
        expected: 예상_좌표(text),
        verification: 검증(text),
    }
}

/// 프론트매터의 `baseline:` — **[F12 §4] 가 요구한 「계획에 기록된 Snapshot」이다.**
///
/// ⚠ `--base <ref>` 를 안 만든다. 그 손잡이의 소유자는 [F23 §7] 이다.
///
/// # 왜 YAML 파서를 안 들이나
///
/// [`crate::fragment`] 의 `grounds_of` 와 같은 판단이다 — 읽는 것이 **열쇠 하나의
/// 문자열**이고, 그 이상을 읽으면 읽을 수 있는데 아무도 안 쓰는 자리가 생긴다.
fn baseline_of(source: &str) -> PlanBaseline {
    let mut 안 = false;
    for line in source.lines() {
        let t = line.trim_end();
        if t == "---" {
            if 안 {
                break;
            }
            안 = true;
            continue;
        }
        if !안 {
            // 프론트매터는 **문서 맨 앞**이다. 본문의 `baseline:` 는 안 읽는다 —
            // 읽으면 예시 코드 안의 한 줄이 이 계획의 기준선이 된다.
            break;
        }
        if let Some(v) = t.trim().strip_prefix("baseline:") {
            let rev = v.trim().trim_matches(|c| c == '"' || c == '\'').to_owned();
            if !rev.is_empty() {
                return PlanBaseline::Declared { rev };
            }
        }
    }
    PlanBaseline::NotDeclared
}

fn 검증(text: &str) -> VerificationStep {
    for line in text.lines() {
        let t = line.trim();
        for head in 검증_머리 {
            if let Some(v) = t.strip_prefix(head) {
                let how = v.trim().to_owned();
                if !how.is_empty() {
                    return VerificationStep::Stated { how };
                }
            }
        }
    }
    VerificationStep::NotStated
}

// ─────────────────────────────────────────────────────────────────────────────
// 예상 좌표 — **네 신호. 강한 것부터, 그리고 겹치면 강한 쪽이 이긴다**
// ─────────────────────────────────────────────────────────────────────────────

/// 항목 본문에서 예상 좌표를 뽑는다.
///
/// 같은 이름이 두 신호로 잡히면 **강한 쪽 하나만 남는다** — 안 그러면 한 항목이 같은
/// 좌표를 두 번 세고, 그 중복이 `as_planned` 의 층화를 흐린다.
fn 넣기(
    name: &str,
    by: PatternSource,
    신규: bool,
    out: &mut Vec<CoordPattern>,
    쓴: &mut std::collections::BTreeSet<String>,
) {
    if !쓴.insert(name.to_owned()) {
        return;
    }
    out.push(if 신규 {
        CoordPattern::NewSymbol { name: name.to_owned(), by }
    } else {
        CoordPattern::Symbol { name: name.to_owned(), by }
    });
}

fn 예상_좌표(text: &str) -> Vec<CoordPattern> {
    let mut out: Vec<CoordPattern> = Vec::new();
    let mut 쓴_이름: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    let mut 쓴_경로: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();

    // ① **명시** — `좌표:` 줄. 사람이 적은 것이라 가장 강하다.
    for line in text.lines() {
        let t = line.trim();
        let Some(v) = 좌표_머리.iter().find_map(|h| t.strip_prefix(h)) else { continue };
        let 신규 = 신규_표시.iter().any(|m| t.contains(m));
        for raw in v.split([',', ' ', '\t']) {
            let tok = raw.trim().trim_matches('`');
            if tok.is_empty() || 신규_표시.contains(&tok) {
                continue;
            }
            if let Some(g) = 경로_패턴(tok) {
                if 쓴_경로.insert(tok.to_owned()) {
                    out.push(CoordPattern::Paths { glob: g });
                }
            } else if 식별자처럼(tok) {
                넣기(tok, PatternSource::Declared, 신규, &mut out, &mut 쓴_이름);
            }
        }
    }

    // ② **인라인 스팬** — 백틱. [F10] 이 유일하게 보던 자리다.
    for (span, 신규) in 스팬들(text) {
        let tok = span.trim();
        if let Some(g) = 경로_패턴(tok) {
            if 쓴_경로.insert(tok.to_owned()) {
                out.push(CoordPattern::Paths { glob: g });
            }
        } else if 식별자처럼(tok) {
            넣기(tok, PatternSource::Span, 신규, &mut out, &mut 쓴_이름);
        }
    }

    // ③ **경로** — 본문 어디서나. ⚠ **범위를 넓힌 자리다**(모듈 머리).
    for tok in 토큰들(text) {
        if let Some(g) = 경로_패턴(tok) {
            if 쓴_경로.insert(tok.to_owned()) {
                out.push(CoordPattern::Paths { glob: g });
            }
        }
    }

    // ④ **식별자** — camelCase·PascalCase. ⚠ **가장 약하다.**
    //    자격은 여기서 안 본다 — *"기준선에 서는가"* 는 `pal_core::plan::resolve` 가 묻는다.
    for tok in 토큰들(text) {
        if 카멜인가(tok) {
            넣기(tok, PatternSource::Identifier, false, &mut out, &mut 쓴_이름);
        }
    }
    out
}

/// 백틱 스팬들과 **그 뒤에 `(신규)` 가 붙었는가.**
fn 스팬들(text: &str) -> Vec<(&str, bool)> {
    let mut out = Vec::new();
    let bytes = text.as_bytes();
    let mut i = 0usize;
    while let Some(open) = text[i..].find('`') {
        let s = i + open + 1;
        let Some(close) = text[s..].find('`') else { break };
        let e = s + close;
        let 뒤 = text[e + 1..].trim_start();
        let 신규 = 신규_표시.iter().any(|m| 뒤.starts_with(m));
        if e > s {
            out.push((&text[s..e], 신규));
        }
        i = e + 1;
        if i >= bytes.len() {
            break;
        }
    }
    out
}

/// 본문의 토큰들 — 구분자는 [`crate::fragment`] 의 `경로처럼` 과 같다.
fn 토큰들(text: &str) -> impl Iterator<Item = &str> {
    text.split(|c: char| c.is_whitespace() || "\"'`(),;[]{}<>|".contains(c))
        .map(|t| t.trim_matches(|c| c == '.' || c == ',' || c == ':'))
        .filter(|t| !t.is_empty())
}

/// 경로처럼 생겼으면 패턴으로.
///
/// **[F10] 의 판정을 그대로 쓰되 하나를 더한다** — `src/order/**` 처럼 **확장자가 없고
/// `*` 가 있는 것**도 받는다. [F12 §3.1] 이 세 형태 중 하나로 그것을 이름으로 적었고,
/// F10 의 규칙은 확장자를 요구하므로 그대로 대면 경로 패턴 형태가 **구조적으로 0** 이 된다.
fn 경로_패턴(tok: &str) -> Option<Glob> {
    if tok.len() < 3 || !tok.contains('/') {
        return None;
    }
    // URL 이 아니다 — [`crate::fragment`] 의 `경로처럼` 과 같은 판정이다.
    if tok.contains("://") || tok.starts_with("//") || tok.starts_with('-') {
        return None;
    }
    let t = crate::narrative::줄번호를_뗀다(tok).trim_start_matches("./");
    // **디렉터리 이름만 적힌 것은 좌표가 아니라 범위다** — 확장자나 `*` 가 있어야 한다.
    if RepoPath::new(t).extension().is_empty() && !t.contains('*') {
        return None;
    }
    Glob::new(t).ok()
}

/// 식별자처럼 생겼는가 — [`crate::fragment`] 가 스팬에 대는 규칙과 같다.
fn 식별자처럼(raw: &str) -> bool {
    !raw.is_empty()
        && raw.len() <= 128
        && raw.chars().all(|ch| ch.is_alphanumeric() || ch == '_' || ch == '.' || ch == '#')
        && raw.chars().next().is_some_and(|ch| ch.is_alphabetic() || ch == '_')
}

/// camelCase·PascalCase 인가 — **본문 토큰을 좌표 후보로 삼는 유일한 조건이다.**
///
/// ⚠ **한 낱말 소문자를 안 받는다.** `run`·`test` 같은 흔한 낱말이 좌표가 되면
/// `as_planned` 가 공짜로 늘고, 그것이 [ADR-0015] 가 반증한 형태의 재발이다.
/// **대문자가 하나 이상 섞여 있어야 한다** — 그것이 「코드에서 온 이름」의 최소 표시다.
fn 카멜인가(tok: &str) -> bool {
    if !식별자처럼(tok) || tok.len() < 3 {
        return false;
    }
    let 본체: &str = tok.rsplit(['.', '#']).next().unwrap_or(tok);
    본체.chars().any(char::is_uppercase)
        && 본체.chars().any(char::is_lowercase)
        && !본체.chars().all(|c| c.is_uppercase() || c == '_')
}

#[cfg(test)]
mod tests {
    use super::*;

    fn 계획(src: &str) -> Plan {
        ingest_plan(&RepoPath::new("docs/plan.md"), src).expect("항목이 있다")
    }

    #[test]
    fn 헤딩_조각이_항목이고_첫_조각은_기획이다() {
        let p = 계획("# 기획 머리\n무엇을 왜\n\n## ac-1\n첫 항목\n\n## ac-2\n둘째 항목\n");
        assert_eq!(p.headline, "기획 머리");
        assert_eq!(p.items().len(), 2);
        // ★ **헤딩이 문장이 아니다** — 헤딩을 문장으로 삼으면 모든 항목의
        // 문장이 제목이 되고, 계획이 무엇을 하겠다고 했는지가 사라진다.
        assert_eq!(p.items()[0].statement, "첫 항목");
    }

    #[test]
    fn 체크박스가_있으면_체크박스가_항목이다() {
        // [F12 §3.4] — *"완료 체크리스트 항목들이 첫 `Plan` 이다"*.
        let p = 계획(
            "# 머리\n설명\n\n## 완료 체크리스트\n\n- [ ] `Plan` 타입\n- [x] `Deviation` 타입\n",
        );
        assert_eq!(p.items().len(), 2, "{:?}", p.items());
        // **표시는 안 본다** — 체크 여부는 주장이고 이 기능은 실제 변경을 잰다.
        assert!(p.items()[1].statement.contains("Deviation"));
    }

    #[test]
    fn 항목이_없으면_거부한다() {
        // ★ §3.3 의 1 단(기획→결정) 미해소. 조용히 빈 계획을 만들면 그 수가 사라진다.
        let r = ingest_plan(&RepoPath::new("docs/x.md"), "# 머리만 있다\n본문\n");
        assert!(matches!(r, Err(PlanRefusal::NoItems { .. })));
    }

    #[test]
    fn 세_형태를_전부_낸다() {
        let p = 계획(
            "# 머리\n설명\n\n## a-1\n`OrderService.cancel` 을 고치고 \
             `OrderService.refund` (신규) 를 만든다. src/order/cancel.ts 도 만진다\n",
        );
        let e = &p.items()[0].expected;
        assert!(
            e.iter().any(|p| matches!(p, CoordPattern::Symbol { name, .. }
                                      if name == "OrderService.cancel")),
            "{e:?}"
        );
        assert!(
            e.iter().any(|p| matches!(p, CoordPattern::NewSymbol { name, .. }
                                      if name == "OrderService.refund")),
            "{e:?}"
        );
        assert!(e.iter().any(|p| matches!(p, CoordPattern::Paths { .. })), "{e:?}");
    }

    #[test]
    fn 백틱_없는_경로와_식별자도_잡는다() {
        // ★ **이것이 F10 과 갈리는 자리다.** ditto 계획 575 중 백틱은 5 건이다.
        let p = 계획("# 머리\n설명\n\n## a-1\nresolveClaimBranch 를 고치고 src/core/git.ts 를 만진다\n");
        let e = &p.items()[0].expected;
        assert!(
            e.iter().any(|x| matches!(x, CoordPattern::Symbol { name, by }
                                      if name == "resolveClaimBranch"
                                      && *by == PatternSource::Identifier)),
            "{e:?}"
        );
        assert!(e.iter().any(|x| matches!(x, CoordPattern::Paths { .. })), "{e:?}");
    }

    #[test]
    fn 한_낱말_소문자는_좌표가_아니다() {
        // ⚠ `run`·`test` 가 좌표가 되면 「계획대로」가 공짜로 는다 — ADR-0015 의 재발.
        let p = 계획("# 머리\n설명\n\n## a-1\n테스트를 run 하고 test 를 고친다\n");
        assert!(p.items()[0].expected.is_empty(), "{:?}", p.items()[0].expected);
    }

    #[test]
    fn 명시가_스팬을_이긴다() {
        let p = 계획("# 머리\n설명\n\n## a-1\n좌표: OrderService.cancel\n`OrderService.cancel` 을 고친다\n");
        let e = &p.items()[0].expected;
        assert_eq!(e.len(), 1, "{e:?}");
        assert_eq!(e[0].source(), PatternSource::Declared);
    }

    #[test]
    fn 검증_줄이_있으면_stated_다() {
        let p = 계획("# 머리\n설명\n\n## a-1\n무엇\n검증: bun test tests/a.test.ts\n");
        assert!(p.items()[0].verification.is_stated());
        let q = 계획("# 머리\n설명\n\n## a-1\n무엇\n");
        assert!(!q.items()[0].verification.is_stated());
    }

    #[test]
    fn 프론트매터의_기준선을_읽고_본문의_것은_안_읽는다() {
        let p = 계획("---\nbaseline: aded7ce7f88f\n---\n# 머리\n설명\n\n## a-1\n무엇\n");
        assert_eq!(p.baseline, PlanBaseline::Declared { rev: "aded7ce7f88f".to_owned() });
        // 본문의 한 줄이 기준선이 되면 예시 코드가 계획을 바꾼다.
        let q = 계획("# 머리\n설명\n\n## a-1\nbaseline: deadbeef\n");
        assert_eq!(q.baseline, PlanBaseline::NotDeclared);
    }

    #[test]
    fn 인입이_결정적이다() {
        let src = "---\nbaseline: abc\n---\n# 머리\n설명\n\n## a-1\n`X.y` 와 src/a.ts\n";
        assert_eq!(계획(src), 계획(src));
    }
}
