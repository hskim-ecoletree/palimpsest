//! 마크다운을 **섹션 단위로 조각낸다** (F10 §3.1).
//!
//! > `pulldown-cmark` 로 마크다운을 순회하며 **섹션 단위**로 조각낸다(헤딩이 경계).
//! > 조각 하나가 `EntityId` 하나.
//!
//! # 왜 크레이트를 들였나 — **이 저장소의 최다 고장 형태가 여기 있다**
//!
//! F03 이 밟은 아홉 중 **다섯**이 *"어디가 코드이고 어디가 아닌가"* 였다(주석 안의
//! 숫자 · 템플릿 리터럴 안 · 주석의 아포스트로피 · 정규식 리터럴 · 바이트와 문자 자리).
//! **마크다운 조각화가 정확히 그 질문이다:**
//!
//! ```text
//! 펜스 안의 `#` 는 헤딩이 아니다
//! 코드 블록 안의 백틱은 인라인 스팬이 아니다
//! setext 헤딩의 밑줄이 `---` 이고 **그것이 프론트매터 구분자와 같은 글자다**
//! 들여쓴 네 칸도 코드 블록이다
//! 백틱 런의 길이가 맞아야 스팬이 닫힌다
//! ```
//!
//! 손으로 쓰면 그 다섯을 다시 밟고, 이번에는 **침묵으로** 밟는다 — 조각이 안 갈리면
//! 결박이 0 이 되고 **결박 0 은 거짓 결박률 0** 이다(합격선 ①이 공짜로 통과).
//!
//! 비용은 실측했다: `--no-default-features` 로 **실제로 느는 크레이트가 둘**
//! (213 → 215). 근거 전문은 `corpus/criteria.toml` `[f10].dependency_decision`.
//!
//! # 여기에 판단이 없다
//!
//! 이 모듈이 내는 것은 **신호의 날것**이다([`RawSignals`]). *"이 경로가 대장에 있는가"*
//! 도 *"이 이름이 유일한가"* 도 여기서 안 묻는다 — 묻는 것은
//! [`pal_core::resolve`] 이고, 그래야 **조각화가 2층을 안 탄다.**

use pal_core::{Fragment, RawSignals, RepoPath};
use pulldown_cmark::{CodeBlockKind, Event, MetadataBlockKind, Options, Parser, Tag, TagEnd};

/// 문서 하나를 조각으로 가른다. **헤딩이 경계다.**
///
/// # 조각이 하나도 안 나오는 경우
///
/// 빈 문서이거나 공백뿐이면 **빈 목록**이다. 억지로 하나를 만들면 그 개체가 결박 대상이
/// 되고, 본문이 없는 결박은 *"무엇이 낡았는가"* 에 답할 수 없다.
#[must_use]
pub fn fragment(path: &RepoPath, source: &str) -> Vec<Fragment> {
    let mut opts = Options::empty();
    // 프론트매터를 **본문으로 흘리지 않는다** — 켜지 않으면 `---` 이 setext 헤딩이나
    // 수평선으로 읽히고, 그러면 문서 첫 조각의 본문에 YAML 이 섞인다.
    opts.insert(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);

    let mut out: Vec<열린조각> = Vec::new();
    let mut 지금 = 열린조각::머리말();
    let mut 프론트매터 = String::new();
    let mut 어디: 자리 = 자리::본문;
    let mut 펜스 = String::new();

    for ev in Parser::new_ext(source, opts) {
        match ev {
            Event::Start(Tag::MetadataBlock(MetadataBlockKind::YamlStyle)) => {
                어디 = 자리::프론트매터;
            }
            Event::End(TagEnd::MetadataBlock(_)) => 어디 = 자리::본문,

            Event::Start(Tag::Heading { .. }) => {
                // **앞 조각을 닫는다.** 헤딩이 경계라는 것이 곧 이 한 줄이다.
                out.push(std::mem::replace(&mut 지금, 열린조각::새것()));
                어디 = 자리::헤딩;
            }
            Event::End(TagEnd::Heading(_)) => {
                // **헤딩 뒤에 줄을 넣는다** — 안 넣으면 `제목본문` 이 한 낱말로 붙고,
                // 그 조각의 첫 줄이 작업 목록에 그대로 실린다.
                지금.body.push('\n');
                어디 = 자리::본문;
            }

            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(_) | CodeBlockKind::Indented)) => {
                펜스.clear();
                어디 = 자리::펜스;
            }
            Event::End(TagEnd::CodeBlock) => {
                지금.signals.fenced_paths.extend(경로처럼(&펜스));
                // 펜스 안의 글자도 본문이다 — **결박의 메모가 그 조각 전체**이기 때문이다.
                지금.body.push_str(&펜스);
                지금.body.push('\n');
                어디 = 자리::본문;
            }

            // ★ **인라인 스팬은 `Code` 로 온다.** 펜스 안의 백틱은 여기 안 온다 —
            //   그것이 크레이트를 들인 이유의 절반이다.
            Event::Code(t) => {
                지금.signals.spans.push(t.to_string());
                지금.body.push('`');
                지금.body.push_str(&t);
                지금.body.push('`');
            }

            Event::Text(t) => match 어디 {
                자리::프론트매터 => 프론트매터.push_str(&t),
                자리::펜스 => 펜스.push_str(&t),
                자리::헤딩 => {
                    지금.heading.push_str(&t);
                    지금.body.push_str(&t);
                }
                자리::본문 => 지금.body.push_str(&t),
            },

            Event::SoftBreak
            | Event::HardBreak
            | Event::Rule
            | Event::End(TagEnd::Paragraph | TagEnd::Item) => 지금.body.push('\n'),
            _ => {}
        }
    }
    out.push(지금);

    // 프론트매터의 좌표는 **문서 첫 조각**에 붙는다 — 문서 전체에 대한 선언이기 때문이다.
    let grounds = grounds_of(&프론트매터);
    닫는다(path, out, &grounds)
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum 자리 {
    본문,
    헤딩,
    펜스,
    프론트매터,
}

struct 열린조각 {
    heading: String,
    body: String,
    signals: RawSignals,
}

impl 열린조각 {
    fn 새것() -> Self {
        Self { heading: String::new(), body: String::new(), signals: RawSignals::default() }
    }
    /// 첫 헤딩보다 앞에 있는 것. **헤딩이 없으므로 앵커도 다르다.**
    fn 머리말() -> Self {
        Self::새것()
    }
}

/// 열린 조각들을 [`Fragment`] 로 닫는다 — **앵커를 여기서 유일하게 만든다.**
fn 닫는다(path: &RepoPath, 열린: Vec<열린조각>, grounds: &[String]) -> Vec<Fragment> {
    let mut 쓴것: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
    let mut out = Vec::new();
    for (i, f) in 열린.into_iter().enumerate() {
        // **빈 조각은 안 낸다.** 본문이 없는 결박은 *"무엇이 낡았는가"* 에 답할 수 없다.
        if f.body.trim().is_empty() {
            continue;
        }
        let 바탕 = if f.heading.trim().is_empty() && i == 0 {
            "_preamble".to_owned()
        } else {
            slug(&f.heading)
        };
        // ★ **앵커는 문서 안에서 유일해야 한다** — 같으면 두 조각이 한 개체가 되고,
        //   그러면 결박이 조용히 덮인다. 같은 헤딩 텍스트는 실제 문서에 흔하다.
        let n = 쓴것.entry(바탕.clone()).or_insert(0);
        *n += 1;
        let anchor = if *n == 1 { 바탕 } else { format!("{바탕}-{n}") };

        let mut signals = f.signals;
        // 프론트매터는 **첫 조각에만** 붙는다.
        if out.is_empty() {
            signals.grounds = grounds.to_vec();
        }
        out.push(Fragment {
            path: path.clone(),
            anchor,
            body: f.body.trim().to_owned(),
            signals,
        });
    }
    out
}

/// 헤딩 텍스트 → 앵커. **GitHub 과 같은 모양이되 우리 규칙이 정본이다.**
///
/// 바깥과 정확히 같을 필요가 없다 — 이 값은 **우리 문서 안에서만 유일하면 된다**
/// ([`pal_core::EntityOrigin`] 이 *"추적용이고 정체성이 아니다"* 라고 적었다).
fn slug(heading: &str) -> String {
    let mut s = String::new();
    for ch in heading.chars() {
        if ch.is_alphanumeric() {
            s.extend(ch.to_lowercase());
        } else if (ch.is_whitespace() || ch == '-' || ch == '_') && !s.ends_with('-') {
            s.push('-');
        }
    }
    let s = s.trim_matches('-').to_owned();
    if s.is_empty() { "_".to_owned() } else { s }
}

/// 펜스 안에서 **경로처럼 생긴** 토큰들. **실재는 대장이 판정한다**(§3.2).
///
/// 여기서 거르는 것은 *"경로가 아닌 것"* 뿐이다 — URL·플래그·순수 식별자.
/// 남은 것을 대장에 물어보는 것이 [`pal_core::resolve`] 의 일이고, **없으면 신호가
/// 없는 것**이다. 그러므로 여기서 넓게 잡는 편이 안전하다.
fn 경로처럼(text: &str) -> Vec<RepoPath> {
    let mut out = Vec::new();
    // ⚠ **`:` 를 구분자에 넣으면 안 된다.** 넣으면 `https://a/b.ts` 가 `https` 와
    // `//a/b.ts` 로 쪼개지고, 그러면 아래의 `://` 검사가 **아무것도 못 본다** —
    // URL 의 뒷조각이 경로로 잡힌다. 실측으로 밟았다.
    for raw in text.split(|c: char| c.is_whitespace() || "\"'`(),;[]{}<>|".contains(c)) {
        let t = 줄번호를_뗀다(raw.trim_matches(|c| c == '.' || c == ','));
        if t.len() < 3 || !t.contains('/') {
            continue;
        }
        // URL 이 아니다 — `://` 가 있거나 `//` 로 시작하면 저장소 경로가 아니다.
        if t.contains("://") || t.starts_with("//") || t.starts_with('-') {
            continue;
        }
        // **확장자가 있어야 한다.** 디렉터리만 적힌 것은 좌표가 아니라 범위다.
        let p = RepoPath::new(t.trim_start_matches("./"));
        if p.extension().is_empty() {
            continue;
        }
        out.push(p);
    }
    out.sort();
    out.dedup();
    out
}

/// `src/a.ts:12` · `src/a.ts:12:3` 에서 줄 표시를 뗀다.
///
/// 문서는 **좌표를 줄 번호와 함께 적는 일이 흔하다.** 안 떼면 확장자가 `ts:12` 가 되어
/// 대장에 없는 경로가 되고, **신호가 조용히 사라진다.**
fn 줄번호를_뗀다(t: &str) -> &str {
    let mut cut = t;
    for _ in 0..2 {
        if let Some((head, tail)) = cut.rsplit_once(':') {
            if !tail.is_empty() && tail.bytes().all(|b| b.is_ascii_digit()) {
                cut = head;
                continue;
            }
        }
        break;
    }
    cut
}

/// 프론트매터에서 `grounds:` 를 읽는다 — **그 열쇠 하나뿐이다.**
///
/// # 왜 YAML 파서를 안 들이나
///
/// 읽는 것이 **열쇠 하나의 문자열 목록**이고, 두 모양뿐이다:
///
/// ```yaml
/// grounds: ["src/a.ts#A.b", "src/c.ts"]
/// grounds:
///   - src/a.ts#A.b
/// ```
///
/// 그 이상을 읽으면 **읽을 수 있는데 아무도 안 쓰는 자리**가 생긴다 —
/// 실 코퍼스 둘에 `grounds:` 를 쓴 문서가 **하나도 없다**(`[f10].input_quality`).
/// 못 읽는 모양이 나오면 그것은 **신호가 없는 것**이고 조각은 미결박으로 간다.
/// 조용히 틀리지 않는다.
fn grounds_of(yaml: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut 목록_안 = false;
    for line in yaml.lines() {
        let t = line.trim_end();
        if let Some(rest) = t.strip_prefix("grounds:") {
            목록_안 = true;
            let rest = rest.trim();
            if rest.is_empty() {
                continue;
            }
            // 한 줄 형태 — `["a", "b"]`.
            for item in rest.trim_start_matches('[').trim_end_matches(']').split(',') {
                let v = item.trim().trim_matches(|c| c == '"' || c == '\'');
                if !v.is_empty() {
                    out.push(v.to_owned());
                }
            }
            목록_안 = false;
            continue;
        }
        if 목록_안 {
            let d = t.trim_start();
            if let Some(v) = d.strip_prefix("- ") {
                let v = v.trim().trim_matches(|c| c == '"' || c == '\'');
                if !v.is_empty() {
                    out.push(v.to_owned());
                }
                continue;
            }
            // 들여쓰기가 끝났으면 이 열쇠도 끝났다.
            if !d.is_empty() {
                목록_안 = false;
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn 조각들(src: &str) -> Vec<Fragment> {
        fragment(&RepoPath::new("docs/x.md"), src)
    }

    #[test]
    fn 헤딩이_경계다() {
        let f = 조각들("# 하나\n첫째 본문\n\n# 둘\n둘째 본문\n");
        assert_eq!(f.len(), 2);
        assert_eq!(f[0].anchor, "하나");
        assert!(f[0].body.contains("첫째 본문"));
        // **헤딩과 본문이 한 낱말로 붙으면 안 된다** — 작업 목록의 첫 줄이 그것이다.
        assert_eq!(f[0].body.lines().next(), Some("하나"), "{:?}", f[0].body);
        assert!(!f[0].body.contains("둘째"), "경계가 안 섰다");
        assert_eq!(f[1].anchor, "둘");
    }

    #[test]
    fn 펜스_안의_샵은_헤딩이_아니다() {
        // **★ 이것이 크레이트를 들인 이유다.** 손으로 쓰면 여기서 조각이 갈라진다.
        let f = 조각들("# 하나\n\n```sh\n# 이것은 셸 주석이다\necho hi\n```\n\n뒷말\n");
        assert_eq!(f.len(), 1, "펜스 안의 `#` 가 헤딩으로 읽혔다: {:?}",
                   f.iter().map(|x| &x.anchor).collect::<Vec<_>>());
        assert!(f[0].body.contains("뒷말"));
    }

    #[test]
    fn 펜스_안의_백틱은_인라인_스팬이_아니다() {
        let f = 조각들("# 하나\n\n```\n`이건 스팬이 아니다`\n```\n\n`이건 스팬이다`\n");
        assert_eq!(f[0].signals.spans, vec!["이건 스팬이다".to_owned()]);
    }

    #[test]
    fn 프론트매터가_본문에_안_섞이고_좌표를_낸다() {
        let f = 조각들("---\ngrounds: [\"src/a.ts#A.b\", \"src/c.ts\"]\n---\n\n# 하나\n본문\n");
        assert_eq!(f.len(), 1);
        assert_eq!(f[0].signals.grounds, vec!["src/a.ts#A.b".to_owned(), "src/c.ts".to_owned()]);
        assert!(!f[0].body.contains("grounds"), "프론트매터가 본문에 샜다: {}", f[0].body);
    }

    #[test]
    fn 프론트매터의_목록_형태도_읽는다() {
        let f = 조각들("---\ntitle: 무엇\ngrounds:\n  - src/a.ts\n  - src/b.ts\nother: 1\n---\n# 하나\n본문\n");
        assert_eq!(f[0].signals.grounds, vec!["src/a.ts".to_owned(), "src/b.ts".to_owned()]);
    }

    #[test]
    fn 펜스_안의_경로를_모으고_url_은_안_모은다() {
        let f = 조각들(
            "# 하나\n\n```\nsrc/order/cancel.ts:12\nhttps://example.com/a/b.ts\n--flag=x/y.ts\n디렉터리/만\n```\n",
        );
        // ⚠ **줄 번호를 뗀다** — 안 떼면 확장자가 `ts:12` 가 되어 신호가 조용히 사라진다.
        // ⚠ **URL 의 뒷조각이 경로로 잡히면 안 된다** — 구분자에 `:` 를 넣었다가 실측으로 밟았다.
        assert_eq!(f[0].signals.fenced_paths, vec![RepoPath::new("src/order/cancel.ts")]);
    }

    #[test]
    fn 같은_헤딩이_두_번이면_앵커가_갈린다() {
        // **★ 같으면 두 조각이 한 개체가 되고 결박이 조용히 덮인다.** 실제 문서에 흔하다.
        let f = 조각들("# 같은 이름\n하나\n\n# 같은 이름\n둘\n");
        assert_eq!(f.len(), 2);
        assert_ne!(f[0].anchor, f[1].anchor, "앵커가 겹쳤다");
        assert_eq!(f[1].anchor, "같은-이름-2");
    }

    #[test]
    fn 머리말도_조각이다() {
        let f = 조각들("헤딩보다 앞의 말\n\n# 하나\n본문\n");
        assert_eq!(f.len(), 2);
        assert_eq!(f[0].anchor, "_preamble");
    }

    #[test]
    fn 빈_문서는_빈_목록이다() {
        // **억지로 하나를 만들면 본문 없는 결박이 생긴다** — *"무엇이 낡았는가"* 에
        // 답할 수 없는 결박이다.
        assert!(조각들("").is_empty());
        assert!(조각들("\n\n   \n").is_empty());
    }

    #[test]
    fn 조각화가_결정적이다() {
        // **회차마다 다르면 제안이 흔들리고, 흔들리는 제안 위의 거부 기록은 아무것도
        // 안 가린다**(`[f10].queue_placement`).
        let src = "---\ngrounds:\n  - src/a.ts\n---\n# 하나\n`Sym` 본문\n\n```\nsrc/b.ts\n```\n\n# 둘\n또\n";
        assert_eq!(조각들(src), 조각들(src));
    }

    #[test]
    fn 들여쓴_코드도_펜스다() {
        // CommonMark 는 네 칸 들여쓰기도 코드 블록으로 읽는다. 손으로 쓰면 놓친다.
        let f = 조각들("# 하나\n\n    src/deep/x.ts\n\n뒷말\n");
        assert_eq!(f[0].signals.fenced_paths, vec![RepoPath::new("src/deep/x.ts")]);
    }
}
