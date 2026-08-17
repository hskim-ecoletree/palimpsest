//! 그래프의 어휘 — **출처 · 생산자 · 해소 등급.**
//!
//! # 이 모듈이 F22 에서야 생긴 이유
//!
//! stack §4 는 `pal-core/provenance.rs` 와 `graph.rs` 를 처음부터 적어 두었다. 그런데
//! 슬라이스 넷(S0~S3)이 그것 없이 노드를 만들었다 — 심볼도 결박도 **출처를 값으로
//! 지지 않은 채** 섰다. 하나뿐인 출처는 적을 필요가 없어 보였기 때문이다.
//!
//! **그 상태가 위험한 이유는 노드가 늘어날 때가 아니라 판정이 늘어날 때 드러난다.**
//! 선행 구현이 정확히 거기서 무너졌다([연구 G §2]) — 파생물이 생산자를 정체성의 성분으로
//! 지지 않아서 정적 엔진의 판정과 에이전트의 판정이 **같은 노드로 붕괴**했다.
//! 여기 있는 넷이 그 붕괴를 막는 성분이다.
//!
//! # 왜 값이 넷이고 다섯이 아닌가 ([DESIGN §3](../../../docs/plan/disposal-map.md))
//!
//! 백서 P2 는 *사실과 추론* 2분할을 요구했고 그것은 가드 라벨을 담지 못한다(추출되지
//! 않았으므로 사실이 아니고 사람이 확정했으므로 추론이 아니다). 4값으로 바꾸면 덮는다.
//! **강도 · 경로 같은 것을 출처 축에 넣지 않는 이유**도 같다 — 5·6값이 되면 파티션 규칙이
//! 다시 열리고 그 비용이 얻는 것보다 크다(§5.2).

use serde::{Deserialize, Serialize};

/// 출처 — **인식론적 지위.** 이 값을 무엇으로 믿을 수 있는가.
///
/// # 불변이다 ([DESIGN §3.1](../../../docs/plan/disposal-map.md))
///
/// 승격은 이 값을 고쳐 쓰는 것이 아니다. `inferred` 를 승인하면 그것을 가리키는 **새
/// `asserted` 노드**가 생기고 원본은 남는다. 그래서 이 열거에 setter 가 없고,
/// 고쳐 쓰는 경로가 없는 것 자체가 세탁 방지의 구현 형태다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Provenance {
    /// 코드에서 결정론적으로 계산됨. 같은 `(재현 입력, 추출기 버전)`은 같은 답.
    Extracted,
    /// 환경에 의존하는 절차(실행·빌드·계측)에서 얻음. **재현 보증이 없다.**
    ///
    /// 존재 주장에만 강하고 **부재 주장에는 쓸 수 없다** — 본 것이 전부라는 보증이
    /// 없기 때문이다. `observed` 의 침묵은 `Residual` 이지 "구멍 없음"이 아니다.
    Observed,
    /// 사람이 승인해 고정함. 결박 필수 + 만료·재확인 규칙.
    Asserted,
    /// 합성·추론의 산물. **근거 · 공백 · 확신도가 필수**이고 승인 없이 승격되지 않는다.
    Inferred,
}

impl Provenance {
    /// 넷이 전부 여기 있다. **하나라도 빠지면 파티션이 덮지 못한다.**
    pub const ALL: [Self; 4] = [Self::Extracted, Self::Observed, Self::Asserted, Self::Inferred];

    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Extracted => "extracted",
            Self::Observed => "observed",
            Self::Asserted => "asserted",
            Self::Inferred => "inferred",
        }
    }

    /// 이름으로 되읽는다. 스키마 파일이 문자열로 적기 때문이다.
    #[must_use]
    pub fn parse(raw: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|p| p.name() == raw)
    }

    /// 이 출처가 `Finding`(존재 주장)을 세울 수 있는가.
    ///
    /// **`observed` 가 여기 포함되는 유일한 비-`extracted` 출처다** — 트레이스가 A→B 를
    /// 실제로 봤다면 그것은 추측이 아니다(DESIGN §3).
    #[must_use]
    pub const fn can_assert_existence(self) -> bool {
        matches!(self, Self::Extracted | Self::Observed)
    }
}

/// 생산자 — **생산 경로.** 무엇이 이 값을 계산했는가.
///
/// # 출처의 중복이 아니다 ([DESIGN §3.4](../../../docs/plan/disposal-map.md))
///
/// 출처는 *이 값을 무엇으로 믿을 수 있는가*이고 생산자는 *무엇이 이 값을 계산했는가*다.
/// `asserted` 하나에 [`Self::Rule`] 과 [`Self::Human`] 이 함께 대응하는 것이 그 차이의
/// 실물이며, **규칙이 틀렸을 때의 파급 계산이 그 구별 위에서만 성립한다**(§3.2) —
/// 규칙 하나가 라벨 300 개를 파생시켰다면 그 300 개의 진단 좌표는 규칙이지 사람이 아니다.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Producer {
    /// 코어 추출기의 결정론적 계산.
    Extractor,
    /// 선언 팩 규칙의 결정론적 적용 — **규칙 좌표를 싣는다**(§3.2).
    Rule { at: String },
    /// 프로젝트 어댑터의 산물(§7.4).
    Provider { id: String },
    /// 에이전트의 합성·추론.
    Agent,
    /// 사람의 승인·입력.
    Human,
    /// 기계가 자동 기록하되 **채우지 못하면 쓰기 자체가 실패하는** 값(§3.1 의 예외).
    ///
    /// 좌표 · `produced_by` · 대장 항목이 이것이다. *"기계가 채울 예정인 필드"* 는
    /// 예외가 아니다 — 선행 구현의 `runs` 가 전형적인 기계 자동 기록 필드였고
    /// 146 건 전부에서 비어 있었다([연구 E §7]).
    MachineRecord,
}

impl Producer {
    /// 스키마 파일이 적는 이름. 인자를 갖는 둘은 접두어만 돌려준다.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Extractor => "extractor",
            Self::Rule { .. } => "rule",
            Self::Provider { .. } => "provider",
            Self::Agent => "agent",
            Self::Human => "human",
            Self::MachineRecord => "machine-record",
        }
    }

    /// 스키마 파일의 표기를 읽는다 — `extractor` · `rule(좌표)` · `provider(id)` 형태.
    #[must_use]
    pub fn parse(raw: &str) -> Option<Self> {
        let raw = raw.trim();
        if let Some(rest) = raw.strip_prefix("rule(").and_then(|r| r.strip_suffix(')')) {
            return Some(Self::Rule { at: rest.to_owned() });
        }
        if let Some(rest) = raw.strip_prefix("provider(").and_then(|r| r.strip_suffix(')')) {
            return Some(Self::Provider { id: rest.to_owned() });
        }
        match raw {
            "extractor" => Some(Self::Extractor),
            "agent" => Some(Self::Agent),
            "human" => Some(Self::Human),
            "machine-record" => Some(Self::MachineRecord),
            _ => None,
        }
    }

    /// 이 생산자가 그 출처의 노드에 설 수 있는가 — **DESIGN §3.4 의 대응표.**
    ///
    /// `machine-record` 만 **해당 노드를 따른다**. 나머지 다섯은 출처 하나에 묶인다.
    /// 어긋나면 스키마 로딩이 거부되고, 그 거부가 [`crate::schema`] 의 몫이다.
    #[must_use]
    pub const fn fits(&self, provenance: Provenance) -> bool {
        match self {
            Self::Extractor => matches!(provenance, Provenance::Extracted),
            Self::Rule { .. } | Self::Human => matches!(provenance, Provenance::Asserted),
            Self::Provider { .. } => matches!(provenance, Provenance::Observed),
            Self::Agent => matches!(provenance, Provenance::Inferred),
            // **어느 출처와도 어긋나지 않는다.** 그것이 이 값이 예외인 이유다.
            Self::MachineRecord => true,
        }
    }
}

/// 엣지 해소 등급 — **엣지에 자격을 박는다** ([DESIGN §5.1](../../../docs/plan/disposal-map.md)).
///
/// P6 은 넓게 잡으라 하고 C2 는 거짓 엣지가 없는 엣지보다 나쁘다고 한다. 둘 다 옳고,
/// 화해는 하나를 고르는 것이 아니라 **엣지가 자기 자격을 싣는 것**이다.
/// 그러면 감사(완전성 우선)는 `Candidate` 를 포함해 넓게 돌고 컨설팅(정밀도 우선)은
/// `Exact|Scoped` 만 쓰되 *"제외된 후보 N 건"* 을 대장으로 붙인다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResolutionGrade {
    /// 임포트·정의가 해소되어 대상이 유일.
    Exact,
    /// 스코프 해소로 후보가 유일 — **L2 이상에서만.**
    Scoped,
    /// 후보가 여럿. **엣지 N 개가 아니라 후보 집합 하나로 저장된다** — 그것이 C2 의
    /// 금지(거짓 엣지)와 P6 의 요구(넓게 잡기)를 함께 지키는 유일한 형태다.
    Candidate,
    /// 경계를 넘는 계약 매칭. **증거 아티팩트가 필수**다(§5.2).
    Contract,
}

impl ResolutionGrade {
    pub const ALL: [Self; 4] = [Self::Exact, Self::Scoped, Self::Candidate, Self::Contract];

    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Scoped => "scoped",
            Self::Candidate => "candidate",
            Self::Contract => "contract",
        }
    }

    #[must_use]
    pub fn parse(raw: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|g| g.name() == raw)
    }

    /// 이 등급이 설 수 있는 출처.
    ///
    /// # 이 함수가 §5.1 의 표보다 넓은 이유 — F22 가 내린 판단 (2026-08-12)
    ///
    /// [DESIGN §5.1](../../../docs/plan/disposal-map.md) 의 등급표는 출처 열에 `exact`·`scoped`·
    /// `candidate` → `extracted`, `contract` → `inferred` 만 적었다. 그런데 §1.2 는
    /// **"모든 엣지"** 가 해소 등급을 진다고 적었고, 그 둘을 그대로 합치면
    /// **`asserted` 엣지가 존재할 수 없다** — 결박(`BOUND_TO`)이 등록되지 못한다.
    ///
    /// 표를 다시 읽으면 그것은 **참조 해소**(코드→코드)를 두고 쓰인 표다. 그 맥락에서
    /// `extracted` 만 적힌 것이지 다른 출처를 금지한 것이 아니다. 반면 §5.2 는
    /// *"경계 엣지는 `extracted` 가 아니라 `inferred` 이며 `contract` 등급으로만
    /// 존재한다"* 를 **명시적 금지**로 적었다.
    ///
    /// 그래서 여기서 금지하는 것은 둘이다:
    ///
    /// | 금지 | 왜 |
    /// |---|---|
    /// | `Contract` × 비-`inferred` | §5.2 의 명시적 규칙. 명세는 계약을 선언할 뿐 호출의 실재를 보증하지 않는다 |
    /// | `Exact`·`Scoped`·`Candidate` × `inferred` | 셋은 **해소의 결과**다. 재현 보증이 없는 추론에 "유일하게 해소됐다"는 자격을 줄 수 없다 |
    ///
    /// **이것은 설계의 공백을 메운 판단이고 게이트에 적혀 있다**(`docs/gates/F22-1-schema.md`).
    #[must_use]
    pub const fn allows(self, p: Provenance) -> bool {
        match self {
            Self::Exact | Self::Scoped | Self::Candidate => !matches!(p, Provenance::Inferred),
            Self::Contract => matches!(p, Provenance::Inferred),
        }
    }
}

/// `asserted` 안의 두 갈래 — **좌표 하나하나에 붙인 것과 규칙 한 줄이 파생시킨 것.**
///
/// 틀렸을 때의 파급이 다르다. 규칙이 바뀌면 파생된 전부가 낡고, 규칙 하나의 재승인으로
/// 300 개가 함께 갱신되는 것이 일괄 승인의 기계 형태다(§3.2 · §11.4).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssertedVia {
    Direct,
    Rule { at: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 출처_넷이_이름_왕복을_견딘다() {
        for p in Provenance::ALL {
            assert_eq!(Provenance::parse(p.name()), Some(p));
        }
        assert_eq!(Provenance::parse("derived"), None);
    }

    #[test]
    fn 생산자_여섯이_표기_왕복을_견딘다() {
        let 여섯 = [
            Producer::Extractor,
            Producer::Rule { at: "packs/auth.toml#R7".into() },
            Producer::Provider { id: "spring-di".into() },
            Producer::Agent,
            Producer::Human,
            Producer::MachineRecord,
        ];
        assert_eq!(여섯.len(), 6);
        assert_eq!(Producer::parse("rule(packs/auth.toml#R7)"), Some(여섯[1].clone()));
        assert_eq!(Producer::parse("provider(spring-di)"), Some(여섯[2].clone()));
        for p in &여섯 {
            if matches!(p, Producer::Rule { .. } | Producer::Provider { .. }) {
                continue;
            }
            assert_eq!(Producer::parse(p.name()).as_ref(), Some(p));
        }
    }

    #[test]
    fn 생산자와_출처의_대응은_하나씩이다() {
        // DESIGN §3.4 의 표 그대로. 어긋나면 파티션이 거짓이 된다.
        assert!(Producer::Extractor.fits(Provenance::Extracted));
        assert!(!Producer::Extractor.fits(Provenance::Inferred));
        assert!(Producer::Agent.fits(Provenance::Inferred));
        assert!(!Producer::Agent.fits(Provenance::Asserted));
        assert!(Producer::Human.fits(Provenance::Asserted));
        assert!(Producer::Rule { at: "x".into() }.fits(Provenance::Asserted));
        assert!(Producer::Provider { id: "x".into() }.fits(Provenance::Observed));
    }

    #[test]
    fn 기계_기록만_어느_출처와도_어긋나지_않는다() {
        // 이것이 §3.1 이 인정한 **유일한 예외**다.
        for p in Provenance::ALL {
            assert!(Producer::MachineRecord.fits(p), "{p:?}");
        }
    }

    #[test]
    fn 관측은_존재를_주장하고_추론은_못_한다() {
        assert!(Provenance::Observed.can_assert_existence());
        assert!(!Provenance::Inferred.can_assert_existence());
        assert!(!Provenance::Asserted.can_assert_existence());
    }

    #[test]
    fn 계약_등급은_추론에서만_선다() {
        // 명세는 계약을 선언할 뿐 호출의 실재를 보증하지 않는다 — §5.2 의 명시적 금지.
        assert!(ResolutionGrade::Contract.allows(Provenance::Inferred));
        for p in [Provenance::Extracted, Provenance::Observed, Provenance::Asserted] {
            assert!(!ResolutionGrade::Contract.allows(p), "{p:?}");
        }
    }

    #[test]
    fn 해소_등급_셋은_추론에서만_막힌다() {
        // **`asserted` 가 막히면 결박 엣지가 등록되지 못한다** — F22 가 내린 판단.
        for g in [ResolutionGrade::Exact, ResolutionGrade::Scoped, ResolutionGrade::Candidate] {
            assert!(g.allows(Provenance::Extracted), "{g:?}");
            assert!(g.allows(Provenance::Observed), "{g:?}");
            assert!(g.allows(Provenance::Asserted), "{g:?}");
            assert!(!g.allows(Provenance::Inferred), "{g:?}");
        }
    }
}
