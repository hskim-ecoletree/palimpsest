//! `/round`의 `## 완수 조건` 문법.
//!
//! Python 하네스와 CLI가 같은 Markdown을 따로 해석하지 않도록 이 모듈이 단일 정본을 진다.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

const SECTION: &str = "완수 조건";

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ConditionId(String);

impl ConditionId {
    pub fn parse(value: &str) -> Option<Self> {
        let bytes = value.as_bytes();
        if bytes.len() < 2 || !bytes[0].is_ascii_uppercase() || !bytes[1].is_ascii_digit() {
            return None;
        }
        let mut i = 2;
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            i += 1;
        }
        if i == bytes.len() {
            return Some(Self(value.to_owned()));
        }
        if i + 2 == bytes.len() && bytes[i] == b'-' && bytes[i + 1].is_ascii_lowercase() {
            return Some(Self(value.to_owned()));
        }
        None
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Verdict {
    #[serde(rename = "통과")]
    Pass,
    #[serde(rename = "반증")]
    Disproved,
    #[serde(rename = "대조불가")]
    Uncomparable,
    #[serde(rename = "미측정")]
    Unmeasured,
}

impl Verdict {
    fn parse(value: &str) -> Option<Self> {
        match value {
            "통과" => Some(Self::Pass),
            "반증" => Some(Self::Disproved),
            "대조불가" => Some(Self::Uncomparable),
            "미측정" => Some(Self::Unmeasured),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Condition {
    pub id: Option<ConditionId>,
    #[serde(rename = "상자")]
    pub checked: bool,
    #[serde(rename = "판정")]
    pub verdict: Option<Verdict>,
    #[serde(rename = "전사")]
    pub transcribed: Option<String>,
    #[serde(rename = "줄")]
    pub line: usize,
    #[serde(rename = "원문")]
    pub source: String,
    #[serde(rename = "형식오류")]
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ConditionsReport {
    #[serde(rename = "파일")]
    pub file: String,
    #[serde(rename = "조건")]
    pub conditions: Vec<Condition>,
    #[serde(rename = "열림")]
    pub open: usize,
    #[serde(rename = "닫힘")]
    pub closed: usize,
    #[serde(rename = "형식오류")]
    pub error_count: usize,
}

impl ConditionsReport {
    pub fn parse(file: impl Into<String>, body: &str) -> Self {
        let conditions = parse_conditions(body);
        Self {
            file: file.into(),
            open: conditions.iter().filter(|c| !c.checked).count(),
            closed: conditions.iter().filter(|c| c.checked).count(),
            error_count: conditions.iter().map(|c| c.errors.len()).sum(),
            conditions,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.error_count == 0
    }
}

pub fn parse_conditions(body: &str) -> Vec<Condition> {
    let mut out = Vec::new();
    let mut seen = BTreeSet::new();
    let mut in_section = false;
    let mut in_fence = false;

    for (index, raw) in body.lines().enumerate() {
        let line = raw.trim_end_matches('\r');
        if line.trim_start().starts_with("```") {
            in_fence = !in_fence;
            continue;
        }
        if in_fence {
            continue;
        }
        if let Some((depth, title)) = heading(line) {
            if depth <= 2 {
                in_section = title.trim().starts_with(SECTION);
            }
            continue;
        }
        if !in_section {
            continue;
        }
        let Some((checked, body)) = checkbox(line) else {
            continue;
        };

        let mut errors = Vec::new();
        let id = leading_id(body);
        if id.is_none() {
            errors.push("ID 가 없다 (`A1` 꼴이 조건 첫 낱말이어야 한다)".to_owned());
        }

        let tag_surface = without_inline_code(body);
        let tag = verdict_tag(&tag_surface);
        if checked && tag.is_none() {
            errors.push("상자가 켜졌는데 판정 태그가 없다 (`· 통과` 꼴)".to_owned());
        }
        if !checked && tag.is_some() {
            errors.push("상자가 안 켜졌는데 판정 태그가 있다 — 안 켜짐은 미측정이다".to_owned());
        }
        if tag_surface.contains("⟨전사") && tag.as_ref().and_then(|(_, d)| d.as_ref()).is_none()
        {
            errors.push("`⟨전사 …⟩` 가 판정 뒤에 안 왔다 (`· <판정> ⟨전사 …⟩`)".to_owned());
        }
        if let Some(id) = &id {
            if !seen.insert(id.clone()) {
                errors.push(format!(
                    "조건 ID `{}` 가 두 번 있다 — ID 는 한 회차에 한 번이다",
                    id.as_str()
                ));
            }
        }

        let (verdict, transcribed) = match tag {
            Some((v, d)) => (Some(v), d),
            None if !checked => (Some(Verdict::Unmeasured), None),
            None => (None, None),
        };
        out.push(Condition {
            id,
            checked,
            verdict,
            transcribed,
            line: index + 1,
            source: line.trim_end().to_owned(),
            errors,
        });
    }
    out
}

fn heading(line: &str) -> Option<(usize, &str)> {
    let depth = line.bytes().take_while(|b| *b == b'#').count();
    if depth == 0 || line.as_bytes().get(depth) != Some(&b' ') {
        return None;
    }
    Some((depth, &line[depth + 1..]))
}

fn checkbox(line: &str) -> Option<(bool, &str)> {
    let trimmed = line.trim_start();
    let rest = trimmed.strip_prefix("- [")?;
    let marker = rest.as_bytes().first().copied()?;
    if !matches!(marker, b' ' | b'x' | b'X') || !rest.get(1..)?.starts_with("] ") {
        return None;
    }
    Some((marker != b' ', &rest[3..]))
}

fn leading_id(body: &str) -> Option<ConditionId> {
    let bare = body.trim_start_matches('*');
    let end = bare.find(char::is_whitespace)?;
    let token = bare[..end].trim_end_matches('*');
    ConditionId::parse(token)
}

fn without_inline_code(body: &str) -> String {
    let mut out = String::with_capacity(body.len());
    let mut chars = body.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch != '`' {
            out.push(ch);
            continue;
        }
        for inner in chars.by_ref() {
            if inner == '`' {
                break;
            }
        }
        out.push(' ');
    }
    out
}

fn verdict_tag(body: &str) -> Option<(Verdict, Option<String>)> {
    let (_, tail) = body.rsplit_once('·')?;
    let tail = tail.trim();
    let (word, rest) = tail.split_once(char::is_whitespace).unwrap_or((tail, ""));
    let verdict = Verdict::parse(word)?;
    let rest = rest.trim();
    if rest.is_empty() {
        return Some((verdict, None));
    }
    let date = rest.strip_prefix("⟨전사 ")?.strip_suffix('⟩')?;
    if valid_date(date) {
        Some((verdict, Some(date.to_owned())))
    } else {
        None
    }
}

fn valid_date(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() == 10
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes
            .iter()
            .enumerate()
            .all(|(i, b)| matches!(i, 4 | 7) || b.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_contract() {
        for good in ["A1", "C11", "C9-a"] {
            assert!(ConditionId::parse(good).is_some(), "{good}");
        }
        for bad in ["A", "a1", "A1-aa", "AA1", "A1-"] {
            assert!(ConditionId::parse(bad).is_none(), "{bad}");
        }
    }

    #[test]
    fn fences_nested_boxes_and_inline_tags_match_the_contract() {
        let got = parse_conditions(concat!(
            "## 완수 조건\n",
            "- [x] A1 pass · 통과\n",
            "  - [ ] A1-a nested\n",
            "- [ ] A2 says `· 반증`\n",
            "```\n- [x] X1 fake · 통과\n```\n",
            "## 범위 밖\n- [ ] X2 fake\n",
        ));
        assert_eq!(got.len(), 3);
        assert_eq!(got[0].verdict, Some(Verdict::Pass));
        assert_eq!(got[1].id.as_ref().map(ConditionId::as_str), Some("A1-a"));
        assert_eq!(got[2].verdict, Some(Verdict::Unmeasured));
    }
}
