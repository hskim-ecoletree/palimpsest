//! JSON 본문의 **위치 보존 편집기** — 재직렬화하지 않는다(회차 `2026-09-15-clean-uninstall` 계획 2).
//!
//! # 왜 이 자리가 따로 있나
//!
//! 착수 동작은 `settings.json` 을 `serde_json` 값으로 읽고 `to_string_pretty` 로 되썼다. 그러면 사용자 파일의
//! 들여쓰기 · 키 순서 · 이스케이프 · 수 표기가 **우리 것이 되고**, 값을 되돌려도 바이트는 안 돌아온다
//! (착수 관측 R1 — `git status` 에 `M .claude/settings.json` 이 남는다).
//!
//! 여기서는 **텍스트 자리**로 더하고 뺀다:
//!
//! - 더하기는 컨테이너의 **마지막 원소 끝 바로 뒤**에 `,` + (그 파일의 줄바꿈 · 들여쓰기) + 새 원소를 끼운다.
//! - 빼기는 그 **정확한 역**이다 — 앞 원소의 끝부터 이 원소의 끝까지를 걷는다.
//! - 비어 있던 컨테이너에 처음 넣을 때만 안쪽 전체를 갈아 끼우고, 원래 안쪽 바이트를 돌려준다 —
//!   부르는 쪽이 기록해 두었다가 마지막 원소를 뺄 때 되돌린다.
//!
//! 우리가 안 만든 바이트는 한 글자도 다시 쓰지 않는다. `serde_json` 은 **검증과 값 대조**에만 쓴다.
//!
//! # 양식은 그 파일에서 읽는다
//!
//! | 무엇 | 어디서 |
//! |---|---|
//! | 줄바꿈 | 첫 줄바꿈([`super::eol::crlf_인가`]) |
//! | 들여쓰기 단위 | 처음 만나는 여러 줄 컨테이너의 (원소 들여쓰기 − 그 컨테이너 줄 들여쓰기) — 없으면 두 칸 |
//! | 여러 줄인가 | 비어 있지 않은 컨테이너는 여는 괄호와 첫 원소 사이에 줄바꿈이 있나 · 빈 컨테이너는 문서 전체 |
//! | 키와 값 사이 | 처음 만나는 멤버의 그 자리 바이트 — 없으면 `": "` |
//! | 한 줄 컨테이너의 원소 사이 | 처음 만나는 한 줄 컨테이너의 그 자리 바이트 — 없으면 `", "` |
//!
//! 끝 개행 유무는 우리가 루트 괄호 밖을 안 건드리므로 **그대로 남는다.**
//!
//! ⚠ 플랫폼 분기가 없다 — 줄바꿈은 파일이 정하고 기계가 정하지 않는다(ADR-0023).

use anyhow::{Context, Result, bail};
use serde_json::Value;

/// 경로의 한 마디 — 객체의 키 또는 배열의 번호.
#[derive(Clone, Copy, Debug)]
pub enum 길목<'a> {
    키(&'a str),
    번호(usize),
}

/// 새로 끼울 값의 모양 — **키 순서를 우리가 정한다**(`serde_json::Map` 의 순서에 안 맡긴다).
#[derive(Clone, Debug)]
pub enum 조각 {
    /// 이미 JSON 으로 적힌 원자(문자열 · 수 · 참거짓 · null).
    원자(String),
    객체(Vec<(String, 조각)>),
    배열(Vec<조각>),
}

impl 조각 {
    /// `serde_json` 값에서 — 객체 키 순서는 그 값의 순서다.
    #[must_use]
    pub fn 값에서(v: &Value) -> Self {
        match v {
            Value::Object(m) => Self::객체(m.iter().map(|(k, v)| (k.clone(), Self::값에서(v))).collect()),
            Value::Array(a) => Self::배열(a.iter().map(Self::값에서).collect()),
            other => Self::원자(other.to_string()),
        }
    }

    /// 문자열 원자.
    #[must_use]
    pub fn 문자열(s: &str) -> Self {
        Self::원자(Value::String(s.to_owned()).to_string())
    }
}

/// 자리 목록으로 읽은 마디.
#[derive(Debug)]
enum 마디 {
    /// `열림` 은 `{` 의 자리 · `닫힘` 은 `}` 의 자리.
    객체 { 열림: usize, 닫힘: usize, 멤버: Vec<멤버> },
    배열 { 열림: usize, 닫힘: usize, 원소: Vec<마디> },
    원자 { 시작: usize, 끝: usize },
}

#[derive(Debug)]
struct 멤버 {
    키: String,
    키_시작: usize,
    키_끝: usize,
    값: 마디,
}

impl 마디 {
    fn 시작(&self) -> usize {
        match self {
            Self::객체 { 열림, .. } | Self::배열 { 열림, .. } => *열림,
            Self::원자 { 시작, .. } => *시작,
        }
    }

    fn 끝(&self) -> usize {
        match self {
            Self::객체 { 닫힘, .. } | Self::배열 { 닫힘, .. } => 닫힘 + 1,
            Self::원자 { 끝, .. } => *끝,
        }
    }

    /// 자식들의 `(시작, 끝)` — 객체는 멤버의 **키 시작**부터 값 끝까지.
    fn 자식_자리(&self) -> Vec<(usize, usize)> {
        match self {
            Self::객체 { 멤버, .. } => 멤버.iter().map(|m| (m.키_시작, m.값.끝())).collect(),
            Self::배열 { 원소, .. } => 원소.iter().map(|e| (e.시작(), e.끝())).collect(),
            Self::원자 { .. } => Vec::new(),
        }
    }

    fn 괄호(&self) -> Option<(usize, usize)> {
        match self {
            Self::객체 { 열림, 닫힘, .. } | Self::배열 { 열림, 닫힘, .. } => Some((*열림, *닫힘)),
            Self::원자 { .. } => None,
        }
    }

    fn 찾는다(&self, 길: &[길목<'_>]) -> Option<&Self> {
        let Some((첫, 나머지)) = 길.split_first() else { return Some(self) };
        match (self, 첫) {
            // 같은 키가 둘이면 `serde_json` 처럼 **뒤의 것**이 값이다.
            (Self::객체 { 멤버, .. }, 길목::키(k)) => {
                멤버.iter().rev().find(|m| m.키 == *k).and_then(|m| m.값.찾는다(나머지))
            }
            (Self::배열 { 원소, .. }, 길목::번호(i)) => 원소.get(*i).and_then(|e| e.찾는다(나머지)),
            _ => None,
        }
    }
}

/// 마디의 종류 — 부르는 쪽이 「남의 구조를 고치려 들지 않는다」를 가르는 데 쓴다.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum 종류 {
    객체,
    배열,
    원자,
}

/// 편집 중인 본문. 편집마다 다시 읽는다 — 설정 파일은 작고, 자리 목록이 낡을 틈을 안 둔다.
pub struct 본문 {
    text: String,
}

impl 본문 {
    /// 읽고 검증한다.
    ///
    /// # Errors
    /// JSON 이 아니거나 최상위가 객체가 아니면.
    pub fn 읽는다(text: String) -> Result<Self> {
        let v: Value = serde_json::from_str(&text).context("JSON 이 아니다")?;
        if !v.is_object() {
            bail!("최상위가 객체가 아니다");
        }
        let 본 = Self { text };
        본.나무()?;
        Ok(본)
    }

    #[must_use]
    pub fn 텍스트(&self) -> &str {
        &self.text
    }

    /// 그 자리의 종류. 없으면 `None`.
    #[must_use]
    pub fn 종류(&self, 길: &[길목<'_>]) -> Option<종류> {
        let 나무 = self.나무().ok()?;
        나무.찾는다(길).map(|m| match m {
            마디::객체 { .. } => 종류::객체,
            마디::배열 { .. } => 종류::배열,
            마디::원자 { .. } => 종류::원자,
        })
    }

    /// 그 자리의 자식 수(객체는 멤버 · 배열은 원소). 컨테이너가 아니거나 없으면 `None`.
    #[must_use]
    pub fn 자식_수(&self, 길: &[길목<'_>]) -> Option<usize> {
        let 나무 = self.나무().ok()?;
        match 나무.찾는다(길)? {
            마디::원자 { .. } => None,
            m => Some(m.자식_자리().len()),
        }
    }

    /// 그 자리의 값.
    #[must_use]
    pub fn 값(&self, 길: &[길목<'_>]) -> Option<Value> {
        let 나무 = self.나무().ok()?;
        let m = 나무.찾는다(길)?;
        serde_json::from_str(&self.text[m.시작()..m.끝()]).ok()
    }

    /// 객체 끝에 멤버 하나를 더한다. **비어 있던 객체였으면 원래 안쪽 바이트**를 돌려준다.
    ///
    /// # Errors
    /// 그 자리가 객체가 아니면.
    pub fn 멤버를_더한다(&mut self, 길: &[길목<'_>], 키: &str, 값: &조각) -> Result<Option<String>> {
        let 키_json = Value::String(키.to_owned()).to_string();
        self.끼운다(길, true, |양식, 들여| format!("{키_json}{}{}", 양식.콜론, 양식.그린다(값, 들여)))
    }

    /// 배열 끝에 원소 하나를 더한다. **비어 있던 배열이었으면 원래 안쪽 바이트**를 돌려준다.
    ///
    /// # Errors
    /// 그 자리가 배열이 아니면.
    pub fn 원소를_더한다(&mut self, 길: &[길목<'_>], 값: &조각) -> Result<Option<String>> {
        self.끼운다(길, false, |양식, 들여| 양식.그린다(값, 들여))
    }

    /// 객체의 멤버 하나를 뺀다(같은 키가 여럿이면 뒤의 것). 마지막 멤버였으면 안쪽을 `빈_안쪽` 으로 되돌린다.
    /// 뺐으면 참.
    ///
    /// # Errors
    /// 그 자리가 객체가 아니면.
    pub fn 멤버를_뺀다(&mut self, 길: &[길목<'_>], 키: &str, 빈_안쪽: Option<&str>) -> Result<bool> {
        let 나무 = self.나무()?;
        let Some(마디::객체 { 멤버, .. }) = 나무.찾는다(길) else {
            bail!("멤버를 뺄 자리가 객체가 아니다");
        };
        let Some(번) = 멤버.iter().rposition(|m| m.키 == 키) else { return Ok(false) };
        self.걷는다(길, 번, 빈_안쪽)?;
        Ok(true)
    }

    /// 배열의 원소 하나를 뺀다. 마지막 원소였으면 안쪽을 `빈_안쪽` 으로 되돌린다. 뺐으면 참.
    ///
    /// # Errors
    /// 그 자리가 배열이 아니면.
    pub fn 원소를_뺀다(&mut self, 길: &[길목<'_>], 번호: usize, 빈_안쪽: Option<&str>) -> Result<bool> {
        let 나무 = self.나무()?;
        let Some(마디::배열 { 원소, .. }) = 나무.찾는다(길) else {
            bail!("원소를 뺄 자리가 배열이 아니다");
        };
        if 번호 >= 원소.len() {
            return Ok(false);
        }
        self.걷는다(길, 번호, 빈_안쪽)?;
        Ok(true)
    }

    // ── 안쪽 ────────────────────────────────────────────────────────────────

    fn 나무(&self) -> Result<마디> {
        let mut p = 파서 { b: self.text.as_bytes(), s: &self.text, i: 0 };
        p.공백();
        let 뿌리 = p.값()?;
        p.공백();
        if p.i != p.b.len() {
            bail!("{} 번째 바이트 뒤에 남는 것이 있다", p.i);
        }
        Ok(뿌리)
    }

    fn 끼운다(
        &mut self,
        길: &[길목<'_>],
        객체여야: bool,
        원소를_그린다: impl Fn(&양식, &str) -> String,
    ) -> Result<Option<String>> {
        let 나무 = self.나무()?;
        let 양식 = 양식::읽는다(&self.text, &나무);
        let Some(자리) = 나무.찾는다(길) else { bail!("더할 자리가 없다") };
        let 맞다 = matches!((자리, 객체여야), (마디::객체 { .. }, true) | (마디::배열 { .. }, false));
        if !맞다 {
            bail!("더할 자리가 {} 가 아니다", if 객체여야 { "객체" } else { "배열" });
        }
        let (열림, 닫힘) = 자리.괄호().expect("컨테이너");
        let 자식 = 자리.자식_자리();
        let 괄호_줄 = 줄_들여쓰기(&self.text, 열림).to_owned();

        if let Some(&(첫_시작, _)) = 자식.first() {
            let (_, 마지막_끝) = *자식.last().expect("마지막");
            let 여러_줄 = self.text[열림 + 1..첫_시작].contains('\n');
            let 끼울 = if 여러_줄 {
                let 들여 = 줄_들여쓰기(&self.text, 첫_시작).to_owned();
                format!(",{}{들여}{}", 양식.줄바꿈, 원소를_그린다(&양식, &들여))
            } else {
                let 한줄 = 양식.한_줄로();
                format!("{}{}", 양식.쉼표, 원소를_그린다(&한줄, &괄호_줄))
            };
            self.text.insert_str(마지막_끝, &끼울);
            return Ok(None);
        }

        let 원래_안쪽 = self.text[열림 + 1..닫힘].to_owned();
        let 새_안쪽 = if 양식.여러_줄 {
            let 들여 = format!("{괄호_줄}{}", 양식.단위);
            format!("{nl}{들여}{}{nl}{괄호_줄}", 원소를_그린다(&양식, &들여), nl = 양식.줄바꿈)
        } else {
            원소를_그린다(&양식.한_줄로(), &괄호_줄)
        };
        self.text.replace_range(열림 + 1..닫힘, &새_안쪽);
        Ok(Some(원래_안쪽))
    }

    fn 걷는다(&mut self, 길: &[길목<'_>], 번: usize, 빈_안쪽: Option<&str>) -> Result<()> {
        let 나무 = self.나무()?;
        let 자리 = 나무.찾는다(길).context("뺄 자리가 없다")?;
        let (열림, 닫힘) = 자리.괄호().context("뺄 자리가 컨테이너가 아니다")?;
        let 자식 = 자리.자식_자리();
        let 줄바꿈 = if super::eol::crlf_인가(self.text.as_bytes()) { "\r\n" } else { "\n" };
        if 번 > 0 {
            // 앞 원소의 끝부터 이 원소의 끝까지 — 더하기의 정확한 역.
            self.text.replace_range(자식[번 - 1].1..자식[번].1, "");
        } else if let Some(&(다음_시작, _)) = 자식.get(1) {
            self.text.replace_range(자식[0].0..다음_시작, "");
        } else {
            // 마지막 하나 — 안쪽을 원래 바이트로. 기록은 LF 로 적혀 있으니 이 파일의 줄바꿈으로 되돌린다.
            let 안쪽 = 빈_안쪽.map_or_else(String::new, |s| {
                String::from_utf8(super::eol::맞춘다(s.as_bytes(), 줄바꿈 == "\r\n"))
                    .unwrap_or_else(|_| s.to_owned())
            });
            self.text.replace_range(열림 + 1..닫힘, &안쪽);
        }
        Ok(())
    }
}

/// 그 자리가 든 줄의 앞 공백(스페이스 · 탭).
fn 줄_들여쓰기(text: &str, 자리: usize) -> &str {
    let 줄_시작 = text[..자리].rfind('\n').map_or(0, |p| p + 1);
    let 나머지 = &text[줄_시작..];
    let n = 나머지.bytes().take_while(|c| *c == b' ' || *c == b'\t').count();
    &나머지[..n]
}

/// 그 파일에서 읽은 양식.
#[derive(Clone)]
struct 양식 {
    줄바꿈: &'static str,
    단위: String,
    콜론: String,
    쉼표: String,
    /// 새 조각을 여러 줄로 그리나.
    여러_줄: bool,
}

impl 양식 {
    fn 읽는다(text: &str, 뿌리: &마디) -> Self {
        let 줄바꿈 = if super::eol::crlf_인가(text.as_bytes()) { "\r\n" } else { "\n" };
        let (열림, 닫힘) = 뿌리.괄호().unwrap_or((0, 0));
        let 여러_줄 = 뿌리.자식_자리().is_empty() || text[열림..닫힘].contains('\n');
        let mut 단위 = None;
        let mut 콜론 = None;
        let mut 쉼표 = None;
        훑는다(text, 뿌리, &mut 단위, &mut 콜론, &mut 쉼표);
        let 콜론 = 콜론.unwrap_or_else(|| ": ".to_owned());
        // 원소 사이를 볼 컨테이너가 없으면 콜론의 띄어쓰기를 따른다 — `{"a":[]}` 에 `, ` 을 섞지 않는다.
        let 쉼표 = 쉼표.unwrap_or_else(|| if 콜론 == ":" { ",".to_owned() } else { ", ".to_owned() });
        Self { 줄바꿈, 단위: 단위.unwrap_or_else(|| "  ".to_owned()), 콜론, 쉼표, 여러_줄 }
    }

    fn 한_줄로(&self) -> Self {
        Self { 여러_줄: false, ..self.clone() }
    }

    /// 조각을 그린다. `들여` 는 그 조각이 시작하는 줄의 들여쓰기다.
    fn 그린다(&self, 조각: &조각, 들여: &str) -> String {
        let 안 = format!("{들여}{}", self.단위);
        let 줄로 = |items: Vec<String>, 여는: char, 닫는: char| {
            if items.is_empty() {
                format!("{여는}{닫는}")
            } else if self.여러_줄 {
                let 사이 = format!(",{}{안}", self.줄바꿈);
                format!("{여는}{nl}{안}{}{nl}{들여}{닫는}", items.join(&사이), nl = self.줄바꿈)
            } else {
                format!("{여는}{}{닫는}", items.join(&self.쉼표))
            }
        };
        match 조각 {
            조각::원자(s) => s.clone(),
            조각::객체(멤버) => 줄로(
                멤버.iter()
                    .map(|(k, v)| {
                        format!("{}{}{}", Value::String(k.clone()), self.콜론, self.그린다(v, &안))
                    })
                    .collect(),
                '{',
                '}',
            ),
            조각::배열(원소) => 줄로(원소.iter().map(|v| self.그린다(v, &안)).collect(), '[', ']'),
        }
    }
}

/// 문서를 앞에서부터 훑어 처음 만나는 단위 · 콜론 · 한 줄 쉼표를 줍는다.
fn 훑는다(
    text: &str,
    m: &마디,
    단위: &mut Option<String>,
    콜론: &mut Option<String>,
    쉼표: &mut Option<String>,
) {
    let 자식 = m.자식_자리();
    if let (Some((열림, _)), Some(&(첫, _))) = (m.괄호(), 자식.first()) {
        let 여러_줄 = text[열림 + 1..첫].contains('\n');
        if 여러_줄 && 단위.is_none() {
            let 바깥 = 줄_들여쓰기(text, 열림);
            let 안 = 줄_들여쓰기(text, 첫);
            if 안.len() > 바깥.len() && 안.starts_with(바깥) {
                *단위 = Some(안[바깥.len()..].to_owned());
            }
        }
        if !여러_줄 && 쉼표.is_none() && 자식.len() >= 2 {
            let 사이 = &text[자식[0].1..자식[1].0];
            if !사이.contains('\n') {
                *쉼표 = Some(사이.to_owned());
            }
        }
    }
    match m {
        마디::객체 { 멤버, .. } => {
            for x in 멤버 {
                if 콜론.is_none() {
                    let 사이 = &text[x.키_끝..x.값.시작()];
                    if !사이.contains('\n') {
                        *콜론 = Some(사이.to_owned());
                    }
                }
                훑는다(text, &x.값, 단위, 콜론, 쉼표);
            }
        }
        마디::배열 { 원소, .. } => {
            for x in 원소 {
                훑는다(text, x, 단위, 콜론, 쉼표);
            }
        }
        마디::원자 { .. } => {}
    }
}

/// 자리를 기억하는 파서. 검증은 `serde_json` 이 먼저 했으므로 여기서는 모양만 따라간다.
struct 파서<'a> {
    b: &'a [u8],
    s: &'a str,
    i: usize,
}

impl 파서<'_> {
    fn 공백(&mut self) {
        while self.b.get(self.i).is_some_and(|c| matches!(c, b' ' | b'\t' | b'\n' | b'\r')) {
            self.i += 1;
        }
    }

    fn 기대(&mut self, c: u8) -> Result<()> {
        if self.b.get(self.i) != Some(&c) {
            bail!("{} 번째 바이트에 `{}` 이 와야 한다", self.i, c as char);
        }
        self.i += 1;
        Ok(())
    }

    fn 값(&mut self) -> Result<마디> {
        self.공백();
        match self.b.get(self.i) {
            Some(b'{') => self.객체(),
            Some(b'[') => self.배열(),
            Some(b'"') => {
                let (시작, 끝) = self.문자열()?;
                Ok(마디::원자 { 시작, 끝 })
            }
            Some(_) => {
                let 시작 = self.i;
                while self
                    .b
                    .get(self.i)
                    .is_some_and(|c| !matches!(c, b',' | b']' | b'}' | b' ' | b'\t' | b'\n' | b'\r'))
                {
                    self.i += 1;
                }
                Ok(마디::원자 { 시작, 끝: self.i })
            }
            None => bail!("값이 와야 할 자리에서 끝났다"),
        }
    }

    fn 문자열(&mut self) -> Result<(usize, usize)> {
        let 시작 = self.i;
        self.기대(b'"')?;
        loop {
            match self.b.get(self.i) {
                None => bail!("문자열이 안 닫혔다"),
                Some(b'\\') => self.i += 2,
                Some(b'"') => {
                    self.i += 1;
                    return Ok((시작, self.i));
                }
                Some(_) => self.i += 1,
            }
        }
    }

    fn 객체(&mut self) -> Result<마디> {
        let 열림 = self.i;
        self.기대(b'{')?;
        let mut 멤버 = Vec::new();
        self.공백();
        if self.b.get(self.i) == Some(&b'}') {
            let 닫힘 = self.i;
            self.i += 1;
            return Ok(마디::객체 { 열림, 닫힘, 멤버 });
        }
        loop {
            self.공백();
            let (키_시작, 키_끝) = self.문자열()?;
            let 키: String = serde_json::from_str(&self.s[키_시작..키_끝]).context("키를 못 읽었다")?;
            self.공백();
            self.기대(b':')?;
            let 값 = self.값()?;
            멤버.push(멤버 { 키, 키_시작, 키_끝, 값 });
            self.공백();
            match self.b.get(self.i) {
                Some(b',') => self.i += 1,
                Some(b'}') => {
                    let 닫힘 = self.i;
                    self.i += 1;
                    return Ok(마디::객체 { 열림, 닫힘, 멤버 });
                }
                _ => bail!("{} 번째 바이트에서 객체가 이어지지 않는다", self.i),
            }
        }
    }

    fn 배열(&mut self) -> Result<마디> {
        let 열림 = self.i;
        self.기대(b'[')?;
        let mut 원소 = Vec::new();
        self.공백();
        if self.b.get(self.i) == Some(&b']') {
            let 닫힘 = self.i;
            self.i += 1;
            return Ok(마디::배열 { 열림, 닫힘, 원소 });
        }
        loop {
            원소.push(self.값()?);
            self.공백();
            match self.b.get(self.i) {
                Some(b',') => self.i += 1,
                Some(b']') => {
                    let 닫힘 = self.i;
                    self.i += 1;
                    return Ok(마디::배열 { 열림, 닫힘, 원소 });
                }
                _ => bail!("{} 번째 바이트에서 배열이 이어지지 않는다", self.i),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{길목, 본문, 조각};
    use serde_json::json;

    fn 왕복(원본: &str) {
        let mut 본 = 본문::읽는다(원본.to_owned()).expect("읽기");
        let 안1 = 본.멤버를_더한다(&[], "agent", &조각::문자열("pal")).expect("더하기");
        let 묶음 = 조각::값에서(&json!({"hooks": [{"command": "pal"}]}));
        let 안2 = 본
            .멤버를_더한다(&[], "hooks", &조각::객체(vec![("Stop".to_owned(), 조각::배열(vec![묶음]))]))
            .expect("더하기");
        assert!(안2.is_none());
        let v: serde_json::Value = serde_json::from_str(본.텍스트()).expect("더한 뒤도 JSON");
        assert_eq!(v["agent"], "pal", "{}", 본.텍스트());
        assert_eq!(v["hooks"]["Stop"][0]["hooks"][0]["command"], "pal", "{}", 본.텍스트());

        assert!(본.멤버를_뺀다(&[], "hooks", 안1.as_deref()).expect("빼기"));
        assert!(본.멤버를_뺀다(&[], "agent", 안1.as_deref()).expect("빼기"));
        assert_eq!(본.텍스트(), 원본, "왕복이 바이트로 안 돌아왔다");
    }

    #[test]
    fn 형태마다_왕복하면_바이트가_돌아온다() {
        for 원본 in [
            "{}",
            "{}\n",
            "{ }",
            "{\n}\n",
            "{\"a\":1}",
            "{\"a\": 1, \"b\": [1, 2]}\n",
            "{\n  \"a\": 1\n}\n",
            "{\n    \"a\": {\n        \"b\": 1e3\n    }\n}",
            "{\r\n\t\"a\": \"\\u0041\\/\"\r\n}\r\n",
        ] {
            왕복(원본);
        }
    }

    #[test]
    fn 파일의_양식으로_그린다() {
        let mut 본 = 본문::읽는다("{\r\n\t\"a\": 1\r\n}".to_owned()).expect("읽기");
        본.멤버를_더한다(&[], "b", &조각::값에서(&json!({"c": [1]}))).expect("더하기");
        assert_eq!(본.텍스트(), "{\r\n\t\"a\": 1,\r\n\t\"b\": {\r\n\t\t\"c\": [\r\n\t\t\t1\r\n\t\t]\r\n\t}\r\n}");

        let mut 한줄 = 본문::읽는다("{\"a\":1,\"b\":2}".to_owned()).expect("읽기");
        한줄.멤버를_더한다(&[], "c", &조각::값에서(&json!([1, 2]))).expect("더하기");
        assert_eq!(한줄.텍스트(), "{\"a\":1,\"b\":2,\"c\":[1,2]}");
    }

    #[test]
    fn 빈_배열의_안쪽을_되돌린다() {
        let 원본 = "{\n  \"hooks\": {\n    \"Stop\": [ ]\n  }\n}\n";
        let mut 본 = 본문::읽는다(원본.to_owned()).expect("읽기");
        let 길 = [길목::키("hooks"), 길목::키("Stop")];
        let 안 = 본.원소를_더한다(&길, &조각::문자열("x")).expect("더하기");
        assert_eq!(안.as_deref(), Some(" "));
        assert_eq!(본.텍스트(), "{\n  \"hooks\": {\n    \"Stop\": [\n      \"x\"\n    ]\n  }\n}\n");
        assert!(본.원소를_뺀다(&길, 0, 안.as_deref()).expect("빼기"));
        assert_eq!(본.텍스트(), 원본);
    }

    #[test]
    fn 자리가_모양이_다르면_멈춘다() {
        let mut 본 = 본문::읽는다("{\"hooks\": 1}".to_owned()).expect("읽기");
        assert!(본.멤버를_더한다(&[길목::키("hooks")], "Stop", &조각::문자열("x")).is_err());
        assert!(본문::읽는다("[1]".to_owned()).is_err());
        assert!(본문::읽는다("{".to_owned()).is_err());
    }
}
