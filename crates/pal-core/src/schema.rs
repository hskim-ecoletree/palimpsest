//! 그래프 스키마 — **`schema/graph.toml` 하나가 단일 진실이다.**
//!
//! `surface/queries.toml` 이 질의에 대해 하는 일을 노드·엣지에 대해 한다
//! ([DESIGN §1.2](../../../docs/plan/disposal-map.md) D25).
//!
//! # 왜 파일이고 왜 여기서 읽는가
//!
//! 스키마를 Rust 타입 정의로만 두면 문서·내보내기·검사가 각자 자라고, 무엇보다
//! **프로젝트가 자기 스키마를 읽을 수 없다.** 그리고 파일로 두면 파생이 생긴다 —
//! 타입 검사 · JSON 스키마 · 문서 표 · `pal export` 매핑 · `doctor` 의 불변식.
//!
//! **읽는 경로가 하나인 것이 이 모듈의 요점이다.** [DESIGN §3.4](../../../docs/plan/disposal-map.md)
//! 는 `producer` ↔ `provenance` 정합을 *"CI 검사가 아니라 파일 하나의 검사이며 어긋나면
//! **로딩 시점에 거부된다**"* 로 적었다. 읽는 곳이 둘이면 한쪽이 검사를 잊을 수 있고
//! 그 순간 그 문장이 거짓이 된다. 그래서 [`GraphSchema::parse`] 는 **검사하거나
//! 거부하거나** 둘 중 하나만 하고, 검사를 건너뛰는 생성자를 제공하지 않는다.
//!
//! # 이 모듈이 강제하는 것 셋
//!
//! 1. **속성 출처 동질성** — 한 노드의 모든 속성 `producer` 가 그 노드 `provenance` 와
//!    정합한다(§3.4). 어긋나면 노드를 쪼개고 엣지로 이어야 한다.
//! 2. **엣지 공통 넷** — 해소 등급 · 출처 · 근거 · 발생 `Snapshot`. **넷이 없는 엣지
//!    타입은 등록되지 않는다**(§1.2).
//! 3. **자리만 만든 노드는 값을 만들 수 없다** — `status = "not_built"` 인 노드의 Rust
//!    타입은 **거주 불가**여야 한다. 자리만 두고 값을 만들 수 있으면 그것이 곧
//!    *"있는데 안 나오는"* 상태가 된다(선행 구현의 `runs` 가 146 건 전부 비어 있던 자리).

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::Deserialize;

use crate::graph::{Producer, Provenance, ResolutionGrade};

// ─────────────────────────────────────────────────────────────────────────────
// 읽힌 것
// ─────────────────────────────────────────────────────────────────────────────

/// 스키마 전체. **[`GraphSchema::parse`] 를 통과한 것만 존재한다.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphSchema {
    pub version: u32,
    /// 라벨 → 선언. **정렬돼 있다** — 문서 표와 검사 산출이 결정적이어야 한다.
    pub nodes: BTreeMap<String, NodeDecl>,
    pub edges: BTreeMap<String, EdgeDecl>,
}

/// 노드 하나의 선언.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeDecl {
    pub label: String,
    /// **이 노드의 모든 속성이 지는 출처**(§3.4).
    pub provenance: Provenance,
    /// 이 라벨을 담는 Rust 타입. 양방향 대조의 다리다.
    pub rust_type: String,
    pub status: NodeStatus,
    /// 정체성의 성분. `key` 와 `attrs` 의 합이 Rust 타입의 `pub` 필드 전부여야 한다.
    pub key: Vec<String>,
    pub attrs: Vec<AttrDecl>,
}

/// 이 빌드가 그 노드의 값을 만들 수 있는가.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeStatus {
    /// 값이 만들어진다.
    Built,
    /// **자리만 있다.** Rust 타입이 거주 불가여야 하고, 산출에서는
    /// [`crate::Capable::NotBuilt`] 로 나온다.
    NotBuilt { by: String },
}

/// 속성 하나.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttrDecl {
    pub name: String,
    /// 값의 형태. v1 에서는 문자열이고 대조는 이름까지다 — 타입 대조는 F03 의
    /// 선택 필드 금지 검사가 서면 그 위에 얹는다.
    pub value_type: String,
    pub producer: Producer,
    pub required: Requirement,
}

/// 필수인가 — **선택 필드는 두지 않는다**(§3.1).
///
/// 셋째 값이 없다. `optional` 이라는 값 자체가 없는 것이 그 규칙의 구현 형태다.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Requirement {
    /// 비면 저장 거부.
    Always,
    /// 그 출처일 때만 필수 — 근거(`evidence`)가 유일한 실례다(§5.2).
    IfProvenance(Provenance),
}

/// 엣지 하나의 선언. **공통 넷을 전부 지지 않으면 만들어지지 않는다.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EdgeDecl {
    pub name: String,
    pub from: String,
    pub to: Vec<String>,
    pub cardinality: Cardinality,
    // ── 공통 넷 ──────────────────────────────────────────────────────────────
    /// ① 해소 등급.
    pub grade: GradeRule,
    /// ② 이 엣지가 설 수 있는 출처.
    pub provenance: Vec<Provenance>,
    /// ③ 근거.
    pub evidence: EvidenceRule,
    /// ④ 발생 `Snapshot` 을 싣는 속성 이름.
    pub snapshot: String,
    // ─────────────────────────────────────────────────────────────────────────
    /// 이 엣지가 **어디에 사는가** — 별도 자리인가, 노드의 필드에 실려 있는가.
    ///
    /// **`Option<Carrier>` 가 아니다.** `None` 은 *"실린 자리가 없다"* 만 말하고
    /// **그것이 「별도 자리다」인지 「아직 안 정했다」인지**를 말하지 않는다 —
    /// stack §5.4 가 금한 자리이고 `cargo xtask check` 가 잡는다.
    pub carried_by: Carried,
    pub attrs: Vec<AttrDecl>,
}

/// 엣지가 **어디에 사는가.**
///
/// [ADR-0005](../../../docs/adr/0005-absence-carries-its-kind.md) — 부재는 종류를
/// 싣는다. *"실린 자리가 없다"* 는 곧 **「별도 자리다」** 라는 사실이지 빈칸이 아니다.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Carried {
    /// 엣지가 자기 자리로 선다. **기본값이다.**
    #[default]
    Standalone,
    /// 노드의 필드에 실려 있다.
    By(Carrier),
}

impl Carried {
    /// 실려 있으면 그 자리.
    #[must_use]
    pub const fn carrier(&self) -> Option<&Carrier> {
        match self {
            Self::By(c) => Some(c),
            Self::Standalone => None,
        }
    }
}

/// 등급이 엣지마다 다른가, 구조상 고정인가.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GradeRule {
    /// 구조상 이 등급 하나뿐이다. 값을 실을 자리가 없어도 된다.
    Fixed(ResolutionGrade),
    /// 엣지마다 다르다. **값을 실을 자리가 있어야 한다.**
    PerEdge,
}

/// 근거를 언제 요구하는가.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvidenceRule {
    /// `inferred` 로 설 때 근거가 비면 저장 거부(§5.2).
    RequiredIfInferred { attr: String },
    /// 이 엣지는 `inferred` 로 설 수 없다 — 그래서 근거를 요구할 자리가 없다.
    NotApplicable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cardinality {
    OneToOne,
    ManyToOne,
    OneToMany,
    ManyToMany,
}

impl Cardinality {
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::OneToOne => "one-to-one",
            Self::ManyToOne => "many-to-one",
            Self::OneToMany => "one-to-many",
            Self::ManyToMany => "many-to-many",
        }
    }

    fn parse(raw: &str) -> Option<Self> {
        [Self::OneToOne, Self::ManyToOne, Self::OneToMany, Self::ManyToMany]
            .into_iter()
            .find(|c| c.name() == raw)
    }
}

/// 엣지를 싣고 있는 노드의 자리.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Carrier {
    pub rust_type: String,
    pub field: String,
}

// ─────────────────────────────────────────────────────────────────────────────
// 거부
// ─────────────────────────────────────────────────────────────────────────────

/// 스키마가 거부된 이유. **로딩 시점에 난다.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchemaError {
    /// TOML 자체가 읽히지 않는다.
    Syntax(String),
    /// 속성 출처 동질성 위반 — §3.4.
    ProducerMismatch { at: String, attr: String, producer: String, provenance: Provenance },
    /// 엣지 공통 넷 중 하나가 없다 — §1.2.
    MissingCommonField { edge: String, field: &'static str },
    /// 알 수 없는 값.
    UnknownValue { at: String, field: &'static str, value: String },
    /// 가리키는 노드 라벨이 없다.
    UnknownLabel { at: String, label: String },
    /// 그 밖의 규칙 위반.
    Rule(String),
}

impl fmt::Display for SchemaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Syntax(m) => write!(f, "스키마를 읽지 못했다: {m}"),
            Self::ProducerMismatch { at, attr, producer, provenance } => write!(
                f,
                "{at}.{attr} 의 생산자 `{producer}` 가 노드 출처 `{}` 와 어긋난다 — \
                 한 노드의 모든 속성은 같은 출처를 갖는다(DESIGN §3.4). \
                 섞으려면 노드를 쪼개고 엣지로 이어라",
                provenance.name()
            ),
            Self::MissingCommonField { edge, field } => write!(
                f,
                "엣지 `{edge}` 에 `{field}` 가 없다 — 모든 엣지는 공통 넷\
                 (grade · provenance · evidence · snapshot)을 진다. \
                 넷이 없는 엣지 타입은 등록되지 않는다(DESIGN §1.2)"
            ),
            Self::UnknownValue { at, field, value } => {
                write!(f, "{at} 의 `{field}` 값을 모른다: `{value}`")
            }
            Self::UnknownLabel { at, label } => {
                write!(f, "{at} 가 없는 노드 라벨을 가리킨다: `{label}`")
            }
            Self::Rule(m) => f.write_str(m),
        }
    }
}

impl std::error::Error for SchemaError {}

// ─────────────────────────────────────────────────────────────────────────────
// 읽기 — 원문
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawSchema {
    schema_version: u32,
    #[serde(default)]
    node: BTreeMap<String, RawNode>,
    #[serde(default)]
    edge: BTreeMap<String, RawEdge>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawNode {
    provenance: String,
    rust_type: String,
    #[serde(default)]
    status: Option<String>,
    #[serde(default)]
    built_by: Option<String>,
    key: Vec<String>,
    #[serde(default)]
    attrs: Vec<RawAttr>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawAttr {
    name: String,
    #[serde(rename = "type")]
    value_type: String,
    producer: String,
    #[serde(default)]
    required: Option<bool>,
    #[serde(default)]
    required_if_provenance: Option<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawEdge {
    from: String,
    to: Vec<String>,
    cardinality: String,
    // ── 공통 넷. **`Option` 인 것은 "없다"를 이름과 함께 보고하기 위해서다** ──
    #[serde(default)]
    grade: Option<String>,
    #[serde(default)]
    provenance: Option<Vec<String>>,
    #[serde(default)]
    evidence: Option<String>,
    #[serde(default)]
    snapshot: Option<String>,
    // ─────────────────────────────────────────────────────────────────────────
    #[serde(default)]
    carried_by: Option<RawCarrier>,
    #[serde(default)]
    attrs: Vec<RawAttr>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawCarrier {
    rust_type: String,
    field: String,
}

// ─────────────────────────────────────────────────────────────────────────────
// 읽기 — 검사
// ─────────────────────────────────────────────────────────────────────────────

impl GraphSchema {
    /// 스키마를 읽는다. **검사하거나 거부하거나 둘 중 하나다.**
    ///
    /// 검사를 건너뛰는 생성자는 없다 — 있으면 §3.4 의 *"로딩 시점에 거부된다"* 가
    /// 거짓이 된다.
    ///
    /// # Errors
    /// TOML 이 읽히지 않거나, 속성 출처 동질성이 깨졌거나, 엣지가 공통 넷을 지지
    /// 않거나, 가리키는 라벨이 없으면.
    pub fn parse(text: &str) -> Result<Self, SchemaError> {
        let raw: RawSchema =
            toml::from_str(text).map_err(|e| SchemaError::Syntax(e.to_string()))?;

        let mut nodes = BTreeMap::new();
        for (label, rn) in raw.node {
            nodes.insert(label.clone(), Self::node(&label, rn)?);
        }

        let known: BTreeSet<&str> = nodes.keys().map(String::as_str).collect();
        let mut edges = BTreeMap::new();
        for (name, re) in raw.edge {
            edges.insert(name.clone(), Self::edge(&name, re, &known)?);
        }

        Ok(Self { version: raw.schema_version, nodes, edges })
    }

    fn node(label: &str, rn: RawNode) -> Result<NodeDecl, SchemaError> {
        let provenance = Provenance::parse(&rn.provenance).ok_or_else(|| {
            SchemaError::UnknownValue {
                at: format!("node.{label}"),
                field: "provenance",
                value: rn.provenance.clone(),
            }
        })?;

        let status = match (rn.status.as_deref(), rn.built_by) {
            (None | Some("built"), None) => NodeStatus::Built,
            (Some("not_built"), Some(by)) => NodeStatus::NotBuilt { by },
            (Some("not_built"), None) => {
                return Err(SchemaError::Rule(format!(
                    "node.{label} 이 `not_built` 인데 `built_by` 가 없다 — \
                     능력 부재는 **어느 기능이 그것을 만드는지와 함께** 값이 된다(stack §5.3)"
                )));
            }
            (None | Some("built"), Some(_)) => {
                return Err(SchemaError::Rule(format!(
                    "node.{label} 에 `built_by` 가 있는데 `status` 가 `not_built` 가 아니다 — \
                     만들 수 있는 노드에 만드는 기능을 적으면 그 자리가 무엇을 뜻하는지 흐려진다"
                )));
            }
            (Some(other), _) => {
                return Err(SchemaError::UnknownValue {
                    at: format!("node.{label}"),
                    field: "status",
                    value: other.to_owned(),
                });
            }
        };

        if rn.key.is_empty() {
            return Err(SchemaError::Rule(format!("node.{label} 에 `key` 가 비어 있다")));
        }

        let attrs = rn
            .attrs
            .into_iter()
            .map(|a| Self::attr(&format!("node.{label}"), a))
            .collect::<Result<Vec<_>, _>>()?;

        // **속성 출처 동질성** — §3.4. 여기가 그 규칙이 파일 하나의 검사가 되는 자리다.
        for a in &attrs {
            if !a.producer.fits(provenance) {
                return Err(SchemaError::ProducerMismatch {
                    at: format!("node.{label}"),
                    attr: a.name.clone(),
                    producer: a.producer.name().to_owned(),
                    provenance,
                });
            }
        }

        // 키와 속성 이름이 겹치면 어느 쪽이 정체성인지가 흐려진다.
        for k in &rn.key {
            if attrs.iter().any(|a| &a.name == k) {
                return Err(SchemaError::Rule(format!(
                    "node.{label} 의 `{k}` 가 key 와 attrs 양쪽에 있다"
                )));
            }
        }

        Ok(NodeDecl {
            label: label.to_owned(),
            provenance,
            rust_type: rn.rust_type,
            status,
            key: rn.key,
            attrs,
        })
    }

    fn attr(at: &str, a: RawAttr) -> Result<AttrDecl, SchemaError> {
        let producer = Producer::parse(&a.producer).ok_or_else(|| SchemaError::UnknownValue {
            at: at.to_owned(),
            field: "producer",
            value: a.producer.clone(),
        })?;

        let required = match (a.required, a.required_if_provenance.as_deref()) {
            (Some(true), None) => Requirement::Always,
            (None, Some(p)) => Provenance::parse(p).map(Requirement::IfProvenance).ok_or_else(
                || SchemaError::UnknownValue {
                    at: at.to_owned(),
                    field: "required_if_provenance",
                    value: p.to_owned(),
                },
            )?,
            // **`required = false` 라는 값이 없다.** 선택 필드를 두지 않는다는 §3.1 이
            // 스키마 문법 수준에서 표현되는 자리다 — 적을 수 없으면 만들 수 없다.
            (Some(false), _) => {
                return Err(SchemaError::Rule(format!(
                    "{at}.{} 이 `required = false` 다 — 필수이거나 없거나 둘 중 하나다\
                     (DESIGN §3.1). 그만큼 중요하지 않으면 스키마에서 빼라",
                    a.name
                )));
            }
            (None, None) | (Some(true), Some(_)) => {
                return Err(SchemaError::Rule(format!(
                    "{at}.{} 은 `required = true` 또는 `required_if_provenance` 중 \
                     정확히 하나를 가져야 한다",
                    a.name
                )));
            }
        };

        Ok(AttrDecl { name: a.name, value_type: a.value_type, producer, required })
    }

    fn edge(
        name: &str,
        re: RawEdge,
        known: &BTreeSet<&str>,
    ) -> Result<EdgeDecl, SchemaError> {
        // ── 공통 넷이 **전부** 있어야 한다. 없는 것을 이름으로 보고한다 ──────────
        let grade_raw = re.grade.ok_or(SchemaError::MissingCommonField { edge: name.to_owned(), field: "grade" })?;
        let prov_raw = re.provenance.ok_or(SchemaError::MissingCommonField { edge: name.to_owned(), field: "provenance" })?;
        let ev_raw = re.evidence.ok_or(SchemaError::MissingCommonField { edge: name.to_owned(), field: "evidence" })?;
        let snapshot = re.snapshot.ok_or(SchemaError::MissingCommonField { edge: name.to_owned(), field: "snapshot" })?;

        let at = format!("edge.{name}");

        let grade = if grade_raw == "per_edge" {
            GradeRule::PerEdge
        } else {
            ResolutionGrade::parse(&grade_raw)
                .map(GradeRule::Fixed)
                .ok_or_else(|| SchemaError::UnknownValue {
                    at: at.clone(),
                    field: "grade",
                    value: grade_raw.clone(),
                })?
        };

        if prov_raw.is_empty() {
            return Err(SchemaError::MissingCommonField {
                edge: name.to_owned(),
                field: "provenance",
            });
        }
        let provenance = prov_raw
            .iter()
            .map(|p| {
                Provenance::parse(p).ok_or_else(|| SchemaError::UnknownValue {
                    at: at.clone(),
                    field: "provenance",
                    value: p.clone(),
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        let evidence = if ev_raw == "not_applicable" {
            EvidenceRule::NotApplicable
        } else if let Some(attr) = ev_raw.strip_prefix("required_if_inferred:") {
            EvidenceRule::RequiredIfInferred { attr: attr.trim().to_owned() }
        } else {
            return Err(SchemaError::UnknownValue {
                at: at.clone(),
                field: "evidence",
                value: ev_raw,
            });
        };

        // `inferred` 로 설 수 있는데 근거를 요구하지 않으면 P3 이 문장으로 되돌아간다.
        if provenance.contains(&Provenance::Inferred)
            && matches!(evidence, EvidenceRule::NotApplicable)
        {
            return Err(SchemaError::Rule(format!(
                "{at} 는 `inferred` 로 설 수 있는데 근거를 요구하지 않는다 — \
                 근거 없는 추론 엣지는 저장될 수 없다(DESIGN §5.2 · §9.1)"
            )));
        }

        // 등급이 설 수 없는 출처와 짝지어지면 그 엣지는 만들어질 수 없다.
        if let GradeRule::Fixed(g) = grade {
            if let Some(bad) = provenance.iter().find(|p| !g.allows(**p)) {
                return Err(SchemaError::Rule(format!(
                    "{at} 의 등급 `{}` 는 출처 `{}` 에 설 수 없다(DESIGN §5.1·§5.2)",
                    g.name(),
                    bad.name()
                )));
            }
        }

        let cardinality = Cardinality::parse(&re.cardinality).ok_or_else(|| {
            SchemaError::UnknownValue {
                at: at.clone(),
                field: "cardinality",
                value: re.cardinality.clone(),
            }
        })?;

        for label in std::iter::once(&re.from).chain(re.to.iter()) {
            if !known.contains(label.as_str()) {
                return Err(SchemaError::UnknownLabel { at: at.clone(), label: label.clone() });
            }
        }
        if re.to.is_empty() {
            return Err(SchemaError::Rule(format!("{at} 에 `to` 가 비어 있다")));
        }

        let attrs = re
            .attrs
            .into_iter()
            .map(|a| Self::attr(&at, a))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(EdgeDecl {
            name: name.to_owned(),
            from: re.from,
            to: re.to,
            cardinality,
            grade,
            provenance,
            evidence,
            snapshot,
            carried_by: re.carried_by.map_or(Carried::Standalone, |c| {
                Carried::By(Carrier { rust_type: c.rust_type, field: c.field })
            }),
            attrs,
        })
    }

    /// 이 라벨이 Rust 타입에서 지는 이름 전부 — `key` + `attrs`.
    ///
    /// **양방향 대조의 한쪽이다.** 다른 쪽은 소스의 `pub` 필드이고 그것은 `xtask` 가 센다.
    #[must_use]
    pub fn field_names(&self, label: &str) -> Vec<String> {
        let Some(n) = self.nodes.get(label) else { return Vec::new() };
        let mut out = n.key.clone();
        out.extend(n.attrs.iter().map(|a| a.name.clone()));
        // 이 노드가 싣고 있는 엣지의 자리도 필드다.
        for e in self.edges.values() {
            if let Some(c) = e.carried_by.carrier() {
                if c.rust_type == n.rust_type {
                    out.push(c.field.clone());
                }
            }
        }
        out.sort();
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 성한 스키마 — 아래 픽스처들이 전부 이것에서 한 자리만 바꾼다.
    const 성한: &str = r#"
schema_version = 1

[node.Symbol]
provenance = "extracted"
rust_type  = "SymbolNode"
key        = ["id"]
attrs = [
  { name = "name", type = "string", producer = "extractor", required = true },
]

[node.Binding]
provenance = "asserted"
rust_type  = "Binding"
key        = ["id"]
attrs = [
  { name = "note",     type = "string",   producer = "human",          required = true },
  { name = "bound_at", type = "snapshot", producer = "machine-record", required = true },
]

[edge.BOUND_TO]
from        = "Binding"
to          = ["Symbol"]
cardinality = "many-to-one"
grade       = "exact"
provenance  = ["asserted"]
evidence    = "not_applicable"
snapshot    = "bound_at"
carried_by  = { rust_type = "Binding", field = "target" }
"#;

    #[test]
    fn 성한_스키마는_읽힌다() {
        let s = GraphSchema::parse(성한).unwrap();
        assert_eq!(s.version, 1);
        assert_eq!(s.nodes.len(), 2);
        assert_eq!(s.edges.len(), 1);
        assert_eq!(s.nodes["Symbol"].provenance, Provenance::Extracted);
    }

    #[test]
    fn 출처가_섞인_노드는_로딩에서_거부된다() {
        // **DESIGN §3.4 의 실물.** `Symbol{name, summary}` 를 만들 수 없다는 것이
        // 규율이 아니라 파일 하나의 검사가 되는 자리다.
        let 섞인 = 성한.replace(
            r#"{ name = "name", type = "string", producer = "extractor", required = true },"#,
            r#"{ name = "name", type = "string", producer = "extractor", required = true },
  { name = "summary", type = "string", producer = "agent", required = true },"#,
        );
        let e = GraphSchema::parse(&섞인).unwrap_err();
        assert!(
            matches!(&e, SchemaError::ProducerMismatch { attr, .. } if attr == "summary"),
            "{e}"
        );
    }

    #[test]
    fn 생산자_여섯_각각이_어긋난_출처에서_거부된다() {
        // 다섯은 출처 하나에 묶이고 `machine-record` 만 노드를 따른다.
        let 짝 = [
            ("extractor", "asserted"),
            ("rule(packs/a.toml#R1)", "extracted"),
            ("provider(spring)", "asserted"),
            ("agent", "extracted"),
            ("human", "inferred"),
        ];
        for (producer, provenance) in 짝 {
            let s = format!(
                r#"
schema_version = 1
[node.X]
provenance = "{provenance}"
rust_type  = "X"
key        = ["id"]
attrs = [ {{ name = "a", type = "string", producer = "{producer}", required = true }} ]
"#
            );
            let e = GraphSchema::parse(&s).unwrap_err();
            assert!(
                matches!(e, SchemaError::ProducerMismatch { .. }),
                "{producer} × {provenance} 가 거부되지 않았다: {e}"
            );
        }
    }

    #[test]
    fn 기계_기록은_어느_출처에서도_거부되지_않는다() {
        // 여섯째의 자리. 이것까지 거부하면 좌표·`produced_by` 를 실을 곳이 없다.
        for p in Provenance::ALL {
            let s = format!(
                r#"
schema_version = 1
[node.X]
provenance = "{}"
rust_type  = "X"
key        = ["id"]
attrs = [ {{ name = "a", type = "string", producer = "machine-record", required = true }} ]
"#,
                p.name()
            );
            assert!(GraphSchema::parse(&s).is_ok(), "{p:?}");
        }
    }

    #[test]
    fn 공통_넷_각각이_없으면_엣지가_등록되지_않는다() {
        // **넷을 하나씩 뺀 픽스처 넷.** 넷이 전부 거부되어야 한다.
        for (field, 줄) in [
            ("grade", r#"grade       = "exact""#),
            ("provenance", r#"provenance  = ["asserted"]"#),
            ("evidence", r#"evidence    = "not_applicable""#),
            ("snapshot", r#"snapshot    = "bound_at""#),
        ] {
            let 뺀 = 성한.replace(줄, "");
            let e = GraphSchema::parse(&뺀).unwrap_err();
            assert!(
                matches!(&e, SchemaError::MissingCommonField { field: f, .. } if *f == field),
                "{field} 를 뺐는데 다른 이유로 거부됐다: {e}"
            );
        }
    }

    #[test]
    fn 추론으로_설_수_있는데_근거를_안_받으면_거부된다() {
        let s = 성한
            .replace(r#"provenance  = ["asserted"]"#, r#"provenance  = ["inferred"]"#)
            .replace(r#"grade       = "exact""#, r#"grade       = "contract""#);
        let e = GraphSchema::parse(&s).unwrap_err();
        assert!(matches!(e, SchemaError::Rule(_)), "{e}");
    }

    #[test]
    fn 등급이_설_수_없는_출처와_짝지어지면_거부된다() {
        // `contract` 는 `inferred` 뿐이다 — 명세는 호출의 실재를 보증하지 않는다(§5.2).
        let s = 성한.replace(r#"grade       = "exact""#, r#"grade       = "contract""#);
        assert!(GraphSchema::parse(&s).is_err());

        // 반대 방향 — 해소 등급 셋은 `inferred` 에 설 수 없다.
        let s = 성한
            .replace(r#"provenance  = ["asserted"]"#, r#"provenance  = ["inferred"]"#)
            .replace(r#"evidence    = "not_applicable""#, r#"evidence    = "required_if_inferred: why""#);
        assert!(GraphSchema::parse(&s).is_err());
    }

    #[test]
    fn 선택_필드는_적을_수_없다() {
        // `required = false` 라는 값이 문법에 없다 — §3.1 이 스키마 수준에서 강제된다.
        let s = 성한.replace(
            r#"producer = "human",          required = true"#,
            r#"producer = "human",          required = false"#,
        );
        let e = GraphSchema::parse(&s).unwrap_err();
        assert!(matches!(e, SchemaError::Rule(_)), "{e}");
    }

    #[test]
    fn 자리만_만든_노드는_만드는_기능을_적어야_한다() {
        let s = r#"
schema_version = 1
[node.X]
provenance = "extracted"
rust_type  = "X"
status     = "not_built"
key        = ["id"]
"#
        .to_owned();
        assert!(GraphSchema::parse(&s).is_err());
        let 적은 = s.replace(r#"status     = "not_built""#, "status     = \"not_built\"\nbuilt_by   = \"F08\"");
        let ok = GraphSchema::parse(&적은).unwrap();
        assert_eq!(ok.nodes["X"].status, NodeStatus::NotBuilt { by: "F08".into() });
    }

    #[test]
    fn 없는_라벨을_가리키는_엣지는_거부된다() {
        let s = 성한.replace(r#"to          = ["Symbol"]"#, r#"to          = ["Synthesis"]"#);
        let e = GraphSchema::parse(&s).unwrap_err();
        assert!(matches!(&e, SchemaError::UnknownLabel { label, .. } if label == "Synthesis"), "{e}");
    }

    #[test]
    fn 노드의_필드_이름은_키와_속성과_실린_엣지의_합이다() {
        let s = GraphSchema::parse(성한).unwrap();
        assert_eq!(s.field_names("Binding"), ["bound_at", "id", "note", "target"]);
        assert_eq!(s.field_names("Symbol"), ["id", "name"]);
    }

    #[test]
    fn 모르는_키는_거부된다() {
        // 오타가 조용히 무시되면 스키마는 자기가 뭘 선언했는지 모른다.
        let s = 성한.replace("rust_type  = \"SymbolNode\"", "rust_typ   = \"SymbolNode\"");
        assert!(matches!(GraphSchema::parse(&s), Err(SchemaError::Syntax(_))));
    }
}
