//! 질의 카탈로그 — **`surface/queries.toml` 하나가 목록의 단일 진실이다** (F06 §2).
//!
//! [`crate::schema`] 가 노드·엣지에 대해 하는 일을 **질의**에 대해 한다.
//!
//! # 카탈로그를 들이는 순간 단일 진실이 넷이 된다
//!
//! ```text
//! QueryName::ALL  →  pal_query::NamedQuery  →  pal-cli 서브커맨드  →  질의 로그
//! ```
//!
//! 넷이 카탈로그와 **양방향으로** 대조되지 않으면 *"표면이 계약이면 목록도 계약이다"*
//! 는 문장으로만 남는다. 대조는 `cargo xtask check` 의 「카탈로그 정합」이 지고,
//! **방향마다 루프를 따로 돈다** — 한 루프에서 두 방향을 돌면 한쪽의 `continue` 가
//! 다른 쪽을 끄고, 하필 **통제가 필요한 표본에서만** 꺼진다(F05 의 오라클이 그렇게
//! 꺼졌다 · `[f06.1.pass]` ①).
//!
//! # 런타임에 안 읽힌다
//!
//! `schema/graph.toml` 과 같은 자격이다. `pal query --list` 는 **코드의 선언**
//! ([`QueryName`] 의 메서드들)에서 렌더링되고, 이 파일은 그것을 **대조**한다.
//! 런타임에 읽으면 파일이 없을 때 답을 못 내고, 그것은 *"호스트 없이도 코어가 답한다"*
//! 를 깨는 **새 실패 경로**다.
//!
//! # 손으로 쓴다
//!
//! 코드에서 생성하면 아래의 대조가 전부 항등식이 되어 **영원히 통과한다.**
//! 그래서 [`QueryCatalog`] 는 **읽기만** 하고 쓰는 경로가 없다.

use std::collections::BTreeMap;
use std::fmt;

use serde::Deserialize;

use crate::query_log::QueryName;

/// 카탈로그 전체. **[`QueryCatalog::parse`] 를 통과한 것만 존재한다.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryCatalog {
    pub version: u32,
    /// 질의 이름 → 선언. **정렬돼 있다** — 검사 산출이 결정적이어야 한다.
    pub queries: BTreeMap<String, QueryDecl>,
}

/// 질의 하나의 선언.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryDecl {
    pub name: String,
    pub summary: String,
    pub args: Vec<ArgDecl>,
    /// **찾았을 때의** 답의 모양. `args` 가 비어 있지 않은 질의는 `Ambiguous`·`Unknown`
    /// 으로도 답하고, 그 둘은 실패가 아니라 답이다.
    pub returns: String,
    /// 이 질의를 세운 기능. **로드맵이 아니라 이미 일어난 사실이다.**
    pub introduced: String,
}

/// 인자 하나의 선언.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArgDecl {
    pub name: String,
    pub value_type: String,
    pub required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CatalogError {
    /// TOML 자체가 읽히지 않는다.
    Syntax(String),
    /// 이름이 비었다 — 이름이 없으면 세어지지 않는다.
    EmptyName,
    /// 선택 인자를 아직 안 만든다. **없는 자리를 미리 만들지 않는다.**
    OptionalArg { query: String, arg: String },
}

impl fmt::Display for CatalogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Syntax(m) => write!(f, "카탈로그를 읽지 못했다: {m}"),
            Self::EmptyName => write!(f, "이름이 빈 질의가 있다"),
            Self::OptionalArg { query, arg } => write!(
                f,
                "`{query}` 의 인자 `{arg}` 가 `required = false` 다 — 이 빌드에 선택 인자를 \
                 받는 질의가 없다. 자리를 미리 만들면 그것이 곧 \"있는데 안 쓰이는\" 자리다"
            ),
        }
    }
}

impl std::error::Error for CatalogError {}

// ─────────────────────────────────────────────────────────────────────────────
// 읽는 형태 — TOML 의 모양 그대로
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawCatalog {
    catalog_version: u32,
    #[serde(default)]
    query: BTreeMap<String, RawQuery>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawQuery {
    summary: String,
    #[serde(default)]
    args: Vec<RawArg>,
    returns: String,
    introduced: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawArg {
    name: String,
    #[serde(rename = "type")]
    value_type: String,
    required: bool,
}

impl QueryCatalog {
    /// 카탈로그를 읽는다. **검사하거나 거부하거나 둘 중 하나만 한다** —
    /// 검사를 건너뛰는 생성자를 제공하지 않는다([`crate::schema`] 와 같은 규율).
    ///
    /// # Errors
    /// TOML 이 안 읽히거나, 이름이 비었거나, 이 빌드가 안 만든 형태를 선언하면.
    pub fn parse(text: &str) -> Result<Self, CatalogError> {
        let raw: RawCatalog =
            toml::from_str(text).map_err(|e| CatalogError::Syntax(e.to_string()))?;

        let mut queries = BTreeMap::new();
        for (name, rq) in raw.query {
            if name.trim().is_empty() {
                return Err(CatalogError::EmptyName);
            }
            let mut args = Vec::new();
            for a in rq.args {
                if !a.required {
                    return Err(CatalogError::OptionalArg { query: name, arg: a.name });
                }
                args.push(ArgDecl { name: a.name, value_type: a.value_type, required: true });
            }
            queries.insert(
                name.clone(),
                QueryDecl {
                    name,
                    summary: rq.summary,
                    args,
                    returns: rq.returns,
                    introduced: rq.introduced,
                },
            );
        }

        Ok(Self { version: raw.catalog_version, queries })
    }

    /// 이름들. **정렬돼 있다.**
    #[must_use]
    pub fn names(&self) -> Vec<&str> {
        self.queries.keys().map(String::as_str).collect()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 코드 쪽 선언 — **대조의 반대편**
//
// 카탈로그가 이것에서 생성되면 대조가 항등식이 된다. 그래서 여기 있는 것은
// 카탈로그의 사본이 아니라 **코드가 스스로 아는 것**이다 — 표면이 `--list` 를 낼 때
// 읽는 것도 이쪽이고, 파일이 없어도 답이 나가야 하기 때문이다.
// ─────────────────────────────────────────────────────────────────────────────

impl QueryName {
    /// 이 질의가 무엇에 답하는가 — 한 줄.
    #[must_use]
    pub const fn summary(self) -> &'static str {
        match self {
            Self::LedgerSnapshot => "이 스냅샷의 관측 범위 대장",
            Self::SymbolResolve => "이름 하나 → 후보 심볼들. **여럿인 것이 정상이다**",
            Self::SymbolContains => "이 심볼이 담는 것들 — 컨테이너 체인으로",
            Self::SymbolCallers => "이 심볼을 가리키는 것들 — 1홉 역방향",
            Self::SymbolReaches => "이 심볼에서 닿는 것들 — **예산 절단이 있는 BFS**",
            Self::GraphDump => "노드와 엣지 전부 — 바깥 오라클이 읽는 창",
            Self::BindingStatus => "결박마다 상태 + **반경** + 무엇이 켰는가",
            Self::NarrativeUnbound => "좌표를 못 찾은 문서 조각들 — **사람의 작업 목록**",
            Self::BindingTouch => "좌표 하나를 만진다 — **걸린 것**과 ★ **지켜보는 것**을 함께 낸다",
        }
    }

    /// 인자 이름들. **비어 있으면 인자를 안 받는다.**
    ///
    /// 개수가 곧 arity 이고, 그것이 `NamedQuery::parse` 가 인자를 요구하는지와
    /// 같아야 한다(`[f06.1.pass]` ① 방향 3).
    #[must_use]
    pub const fn arg_names(self) -> &'static [&'static str] {
        match self {
            Self::LedgerSnapshot | Self::GraphDump | Self::BindingStatus
            | Self::NarrativeUnbound => &[],
            Self::SymbolResolve | Self::SymbolContains | Self::SymbolCallers
            | Self::SymbolReaches | Self::BindingTouch => &["name"],
        }
    }

    /// 인자의 타입 이름들 — [`Self::arg_names`] 와 같은 순서.
    #[must_use]
    pub const fn arg_types(self) -> &'static [&'static str] {
        match self {
            Self::LedgerSnapshot | Self::GraphDump | Self::BindingStatus
            | Self::NarrativeUnbound => &[],
            Self::SymbolResolve | Self::SymbolContains | Self::SymbolCallers
            | Self::SymbolReaches | Self::BindingTouch => &["SymbolName"],
        }
    }

    /// **찾았을 때의** 답의 모양.
    #[must_use]
    pub const fn returns(self) -> &'static str {
        match self {
            Self::LedgerSnapshot => "Ledger",
            Self::SymbolResolve | Self::SymbolContains | Self::SymbolCallers => "Symbols",
            Self::SymbolReaches => "Reached",
            Self::GraphDump => "Graph",
            Self::BindingStatus => "Bindings",
            Self::NarrativeUnbound => "Narrative",
            Self::BindingTouch => "Touch",
        }
    }

    /// 이 질의를 세운 기능. **로드맵이 아니라 이미 일어난 사실이다.**
    #[must_use]
    pub const fn introduced(self) -> &'static str {
        match self {
            Self::LedgerSnapshot => "F01",
            Self::SymbolContains => "F02",
            Self::SymbolResolve => "F03",
            Self::SymbolCallers | Self::SymbolReaches | Self::GraphDump => "F05",
            Self::BindingStatus => "F09",
            Self::NarrativeUnbound => "F10",
            Self::BindingTouch => "F11",
        }
    }

    /// 이름 하나를 받는가. **받으면 `Ambiguous`·`Unknown` 으로도 답한다.**
    #[must_use]
    pub const fn takes_a_name(self) -> bool {
        !self.arg_names().is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const 견본: &str = r#"
catalog_version = 1

[query."ledger.snapshot"]
summary    = "대장"
args       = []
returns    = "Ledger"
introduced = "F01"

[query."symbol.resolve"]
summary    = "이름 하나"
args       = [{ name = "name", type = "SymbolName", required = true }]
returns    = "Symbols"
introduced = "F03"
"#;

    #[test]
    fn 카탈로그를_읽는다() {
        let c = QueryCatalog::parse(견본).expect("읽힌다");
        assert_eq!(c.version, 1);
        assert_eq!(c.names(), vec!["ledger.snapshot", "symbol.resolve"]);
        assert_eq!(c.queries["symbol.resolve"].args.len(), 1);
        assert_eq!(c.queries["symbol.resolve"].args[0].value_type, "SymbolName");
        // 인자 없는 질의는 **비어 있다** — `None` 이 아니다.
        assert!(c.queries["ledger.snapshot"].args.is_empty());
    }

    #[test]
    fn 모르는_필드는_거부된다() {
        // 오타가 조용히 무시되면 카탈로그가 무엇을 선언했는지 알 수 없다.
        let 오타 = 견본.replace("introduced = \"F01\"", "introducd = \"F01\"");
        assert!(matches!(QueryCatalog::parse(&오타), Err(CatalogError::Syntax(_))));
    }

    #[test]
    fn 선택_인자는_거부된다() {
        // **없는 자리를 미리 만들지 않는다.** 이 빌드에 선택 인자를 받는 질의가 없다.
        let 선택 = 견본.replace("required = true", "required = false");
        assert!(matches!(QueryCatalog::parse(&선택), Err(CatalogError::OptionalArg { .. })));
    }

    #[test]
    fn 코드의_선언이_여섯을_전부_덮는다() {
        // 하나라도 빠지면 `match` 가 컴파일을 막지만, **값이 빈 문자열인 것**은 못 막는다.
        for q in QueryName::ALL {
            assert!(!q.summary().is_empty(), "{} 의 요약이 비었다", q.name());
            assert!(!q.returns().is_empty(), "{} 의 반환이 비었다", q.name());
            assert!(!q.introduced().is_empty(), "{} 의 도입이 비었다", q.name());
            assert_eq!(
                q.arg_names().len(),
                q.arg_types().len(),
                "{} 의 인자 이름과 타입 수가 다르다",
                q.name()
            );
        }
    }

    #[test]
    fn 이름을_받는_질의만_모호할_수_있다() {
        // `Ambiguous`·`Unknown` 은 이름을 좁히는 과정에서만 생긴다. 이름을 안 받는
        // 질의가 그 답을 내면 그것은 답이 아니라 결함이다.
        assert!(!QueryName::LedgerSnapshot.takes_a_name());
        assert!(!QueryName::GraphDump.takes_a_name());
        assert!(QueryName::SymbolResolve.takes_a_name());
        // **하한** — 둘 다 1 건 이상이어야 이 단언이 무언가를 가른다.
        let 받는 = QueryName::ALL.iter().filter(|q| q.takes_a_name()).count();
        assert!(받는 >= 1 && 받는 < QueryName::ALL.len(), "두 갈래가 다 서지 않는다");
    }
}
