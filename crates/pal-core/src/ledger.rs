//! 관측 범위 대장 — D4. **모든 응답이 이것을 동반한다.**
//!
//! 완전성을 반증 가능한 형태로 재정의한 것이 대장이다(DESIGN §4):
//!
//! > 기계는 **선언된 관측 범위 안에서** 전수임을 보증한다.
//! > 범위 밖은 "없음"이 아니라 **"보지 않음"** 으로 산출된다.
//!
//! 그래서 파일 상태가 3값(ok/skip/error)이 아니라 **일곱**이다. *"언어를 모른다"* 와
//! *"언어는 아는데 추출기가 없다"* 를 사용자가 다르게 처리하기 때문이다 — 후자는
//! 로드맵이고 전자는 설정이다. 뭉개면 대장이 거짓말을 한다([F01 §6](../../../docs/plan/features/F01-repo-ledger.md)).

use std::collections::BTreeMap;
use std::num::NonZeroUsize;

use serde::{Deserialize, Serialize};

use crate::manifest::ScopeSource;
use crate::repo::{ObjectName, RepoPath, Snapshot, TreeRef};

/// 언어의 이름. **추출 대상 넷보다 넓다.**
///
/// [`crate::Language`] 는 *"이 빌드가 추출할 수 있는가"* 를 묻는 닫힌 넷이고, 이것은
/// *"이 파일이 무슨 언어인가"* 를 묻는 열린 이름이다. 대장은 후자가 필요하다 —
/// 추출기가 없는 언어도 **인식됐다는 사실**이 산출되어야 하기 때문이다.
///
/// **어느 확장자가 어느 이름인지는 여기 없다.** 그 표는 이 크레이트 밖에 산다
/// (`pal-extract::recognize`). `pal-core` 가 언어 목록을 내부화하면 언어를 늘리는 일이
/// 도메인 타입을 고치는 일이 된다.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct LanguageId(String);

impl LanguageId {
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// 언어 능력 등급 — 백서 §2.2-2 *"하한은 전역이 아니라 언어별"*.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtractGrade {
    /// 텍스트만 — 심볼을 만들 수 없다.
    L0,
    /// 구조(선언·포함 관계).
    L1,
    /// 스코프 해소된 참조.
    L2,
    /// 정의-사용(읽기/쓰기 집합).
    L3,
    /// 제어흐름(경로·지배관계).
    L4,
}

impl ExtractGrade {
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::L0 => "L0",
            Self::L1 => "L1",
            Self::L2 => "L2",
            Self::L3 => "L3",
            Self::L4 => "L4",
        }
    }

    /// 추출 등급이 **심볼 정체성의 계산 가능성을 정한다**(DESIGN §2.2).
    ///
    /// 둘을 따로 적는 이유가 이것이다 — L0 은 정체성이 없어 결박 자체가 성립하지 않는다.
    #[must_use]
    pub const fn identity(self) -> IdentityGrade {
        match self {
            Self::L0 => IdentityGrade::Unavailable,
            Self::L1 => IdentityGrade::Ordinal,
            Self::L2 | Self::L3 | Self::L4 => IdentityGrade::Exact,
        }
    }
}

/// 심볼 정체성의 등급.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IdentityGrade {
    /// 없다 — 좌표가 성립하지 않는다. 대장 머리에 "결박 불가 언어 N개"로 적힌다.
    Unavailable,
    /// 선언 순서에 의존한다. 결박이 조금 덜 믿을 만하고 그 사실이 답에 실린다.
    Ordinal,
    /// 스코프 해소로 유일하다.
    Exact,
}

impl IdentityGrade {
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Unavailable => "없음",
            Self::Ordinal => "ordinal",
            Self::Exact => "exact",
        }
    }
}

/// 제외 규칙의 식별자. **필수다.**
///
/// 제외 규칙을 넓히면 판정 대상이 줄고 *"잔여가 줄었다"* 로 보인다. 그것이 게이트
/// 오염의 형태다. 규칙 ID 가 있어야 나중에 **"범위가 줄어서 사라진 것"** 을
/// **"판정되어 사라진 것"** 과 구별할 수 있다([F01 §3.3]).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ExclusionRuleId(String);

impl ExclusionRuleId {
    #[must_use]
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// 왜 대상이 아닌가 — 판정 근거와 함께.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BinaryReason {
    /// 내용에 NUL 바이트가 있다. git 이 쓰는 것과 같은 판정이다.
    NulByte,
}

impl BinaryReason {
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::NulByte => "NUL 바이트",
        }
    }
}

/// 생성물 판정의 증거. **둘 다 필요하다.**
///
/// 생성물을 놓치면 그래프가 오염되고, 과잉 판정하면 실코드가 사라진다. 그래서 경로
/// 패턴과 파일 머리의 표식을 **둘 다** 요구한다 — 추측으로 파일을 범위 밖에 두지 않는다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratedEvidence {
    /// 경로가 걸린 패턴.
    pub path_pattern: String,
    /// 파일 머리에서 찾은 생성 표식.
    pub marker: String,
}

/// [`FileState::Unsupported`] 가 된 이유 — **둘은 서로 다른 자리를 가리킨다.**
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnsupportedReason {
    /// 이 빌드에 그 언어의 추출기가 없다. **로드맵의 자리다.**
    NoExtractor,
    /// 추출기는 있는데 **문법이 이 파일을 읽지 못했다** — `ERROR` 가 삼킨 비율이
    /// [`crate::PROVISIONAL_ERROR_RATIO_PERCENT`] 를 넘었다.
    ///
    /// **문법의 자리다. 로드맵이 아니다.** 처분은 문법을 바꾸는 것이고, 그러면
    /// `ExtractorVersion` 의 문법 축이 움직여 1층 캐시가 전량 무효화된다.
    GrammarDefeated { error_ratio_percent: usize, recovery_sites: usize },
}

/// 파일 하나의 상태 — **일곱이고, 정확히 하나다.**
///
/// # 왜 `tag = "state"` 가 아닌가
///
/// 한때 그렇게 적었다. JSON 에서 `{"state":"parsed", "language":…}` 로 평평하게 나오게
/// 하려던 것인데, **1층 캐시가 이 타입을 담는 순간 그것이 성립하지 않는다** —
/// internally tagged 표현은 self-describing 형식에서만 되고 `postcard`(stack §3.1 이
/// 고른 캐시 직렬화)는 그것이 아니다.
///
/// 형식의 제약이 도메인 타입을 규정하게 두지 않는 것이 원칙이지만, 여기서는 방향이
/// 반대다 — **평평한 JSON 은 표현의 편의였고 캐시 가능성은 이 타입의 성질이다.**
/// 기본 표현(`{"parsed":{…}}`)이 enum 임을 더 분명히 하기도 한다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FileState {
    /// 파싱 성공, 선언된 언어 능력 등급까지 추출됨.
    Parsed { language: LanguageId, grade: ExtractGrade },
    /// 파싱은 됐으나 일부 구문에서 회복 — 회복 지점의 수.
    ///
    /// **자리(`span`)는 대장에 싣지 않는다.** 자리는 파일 하나의 사실이고 대장은 저장소
    /// 하나의 표다 — 997 줄에 범위를 실으면 대장이 읽히지 않는다. 자리를 보는 창은
    /// `pal symbols --graph` 이고 값은 `FileGraph::recovery_sites` 다(#47).
    Partial { language: LanguageId, grade: ExtractGrade, recovery_sites: usize },
    /// 언어는 인식됐는데 이 빌드가 그 파일을 읽지 못했다 — **이유와 함께.**
    ///
    /// # 이유가 없으면 이 칸이 두 가지를 뭉갠다
    ///
    /// 한때 이 변형은 `{ language }` 뿐이었고 문서가 *"추출기가 없다. 로드맵의 자리다"*
    /// 라고 적었다. 그 문장은 `.sql` 에 대해 참이고 **문법이 통째로 못 읽은 Kotlin 파일에
    /// 대해 거짓이다** — 그 언어의 추출기는 있다. 뭉개면 대장 머리가 *"언어 인식됨,
    /// 추출기 없음"* 이라고 적고, 사용자는 **고칠 자리를 로드맵에서 찾는다.**
    ///
    /// 이 저장소가 [`crate::Capable`] · [`crate::Residual`] · [`crate::Uncapturable`] 에서
    /// 일관되게 내린 판단이 이것이다 — **없는 것의 종류를 값으로 남긴다.**
    Unsupported { language: LanguageId, reason: UnsupportedReason },
    /// 언어를 모른다. **설정의 자리다.**
    Unrecognized,
    /// 설정으로 제외 — 규칙 ID 필수.
    Excluded { rule: ExclusionRuleId },
    /// 대상이 아니다.
    Binary { reason: BinaryReason },
    /// 생성물이다 — 증거 둘과 함께.
    Generated { evidence: GeneratedEvidence },
}

impl FileState {
    /// 집계에서 이 상태가 서는 칸.
    #[must_use]
    pub const fn bucket(&self) -> Bucket {
        match self {
            Self::Parsed { .. } => Bucket::Parsed,
            Self::Partial { .. } => Bucket::Partial,
            Self::Unsupported { .. } => Bucket::Unsupported,
            Self::Unrecognized => Bucket::Unrecognized,
            Self::Excluded { .. } => Bucket::Excluded,
            Self::Binary { .. } => Bucket::Binary,
            Self::Generated { .. } => Bucket::Generated,
        }
    }

    /// 인식된 언어가 있으면 그것.
    #[must_use]
    pub const fn language(&self) -> Option<&LanguageId> {
        // `Option` 이 여기 있는 것은 조회 결과이지 도메인 값이 아니다(stack §5.4).
        // 대장에 실리는 것은 위의 `FileState` 자신이고, 이 메서드는 집계용 조회다.
        match self {
            Self::Parsed { language, .. }
            | Self::Partial { language, .. }
            | Self::Unsupported { language, .. } => Some(language),
            _ => None,
        }
    }
}

/// 집계 칸 일곱. **`FileState` 와 1:1 이다.**
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Bucket {
    Parsed,
    Partial,
    Unsupported,
    Unrecognized,
    Excluded,
    Binary,
    Generated,
}

impl Bucket {
    /// 대장이 출력하는 순서. **일곱이 전부 여기 있다** — 하나라도 빠지면 합이 안 맞는다.
    pub const ALL: [Self; 7] = [
        Self::Parsed,
        Self::Partial,
        Self::Unsupported,
        Self::Unrecognized,
        Self::Excluded,
        Self::Binary,
        Self::Generated,
    ];

    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Parsed => "parsed",
            Self::Partial => "partial",
            Self::Unsupported => "unsupported",
            Self::Unrecognized => "unrecognized",
            Self::Excluded => "excluded",
            Self::Binary => "binary",
            Self::Generated => "generated",
        }
    }
}

/// 대장 항목 — 파일당 하나.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LedgerEntry {
    pub path: RepoPath,
    pub state: FileState,
}

/// 언어 하나의 능력 — `(언어 → 추출 등급, identity_grade, 파일 수)`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LanguageCapability {
    pub language: LanguageId,
    pub grade: ExtractGrade,
    pub identity: IdentityGrade,
    pub files: usize,
}

/// 낡음을 재는 자의 낡음 — **감지기 자신이 낡을 수 있다** (DESIGN §6.3 · F01 §4).
///
/// 감지기가 3주 낡았으면 낡음 표시들도 3주 낡았다는 사실이 응답에 붙어야 한다.
///
/// # "이후 커밋 수" 를 싣지 않는다 — 그 자리에서 문서가 어긋나 있었다
///
/// F01 §4 는 *"마지막 재추출 `Snapshot` · 추출기 버전 · **이후 커밋 수**"* 를 적으면서
/// 같은 문단에서 *"이 검사는 **상수 시간**(HEAD 비교)이므로 무한 후퇴하지 않는다"* 고
/// 못 박았다. **커밋 수를 세는 것은 상수 시간이 아니다** — 이력 깊이에 비례하고, 그러면
/// 예산이 필요하고, 예산은 §12.4 의 표에 값이 있어야 켜진다(D16).
///
/// 그래서 상수 시간에 답할 수 있는 것만 싣는다: **추출기 버전과 지금 HEAD.**
/// 세어야 할 커밋 수가 필요해지면 예산과 함께 F05 가 낸다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DetectorFreshness {
    /// 이 대장을 만든 문법의 고정 커밋.
    pub grammar: String,
    /// 이 대장을 만든 추출기 코드 버전.
    pub extractor: String,
    /// 이 대장을 계산할 때의 HEAD.
    ///
    /// 대장이 선 트리와 다르면 **대장이 그 사이의 커밋들을 보지 않았다**는 뜻이다.
    pub head_now: ObjectName,
}

/// 관측 범위 대장.
///
/// **저장 위치**: 최종적으로 2층 인덱스지만 2층은 F05 이고 이 기능이 그보다 앞선다.
/// S1 은 계산해서 바로 낸다 — 이관은 재계산이므로 마이그레이션이 아니다.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ledger {
    pub snapshot: Snapshot,
    /// **선언된** 저장소 수. 대장이 항상 머리에 적는다(DESIGN §4.3).
    ///
    /// 저장소 하나가 빠지면 그것을 지나는 경로가 조용히 사라지는 대신 대장이 계속 말한다.
    pub repos_declared: NonZeroUsize,
    pub entries: Vec<LedgerEntry>,
    /// **`table` 이 아니라 `languages` 다** — 어휘 금지(stack §4.2)에 걸린다.
    pub languages: Vec<LanguageCapability>,
    /// 이 범위가 **선언**에서 왔는가 추정에서 왔는가 (DESIGN §4.3).
    pub scope: ScopeSource,
    /// 낡음을 재는 자의 낡음.
    pub detector: DetectorFreshness,
}

impl Ledger {
    /// 칸별 개수. **합은 항상 `entries.len()` 이다.**
    ///
    /// 이 메서드가 `entries` 에서 매번 세는 것이 곧 전수 분할의 보증이다 — 개수를
    /// 따로 들고 있으면 그것이 어긋날 자리가 생긴다.
    #[must_use]
    pub fn counts(&self) -> BTreeMap<Bucket, usize> {
        let mut out: BTreeMap<Bucket, usize> = Bucket::ALL.iter().map(|b| (*b, 0)).collect();
        for e in &self.entries {
            *out.entry(e.state.bucket()).or_insert(0) += 1;
        }
        out
    }

    /// 이 대장이 선 트리.
    ///
    /// **저장소가 하나일 때만 뜻이 있다.** 스냅샷은 집합이고(DESIGN §1.1) 멀티레포에서는
    /// 저장소마다 트리가 다르다 — 그때 첫 것을 고르면 나머지가 조용히 감춰진다.
    /// 이 빌드는 `repos_declared` 가 언제나 1 이고(멀티레포는 F14) 그 사실을 여기 적는다.
    ///
    /// # Panics
    /// 스냅샷이 비어 있으면. [`Snapshot::of`] 가 빈 것을 만들지 않으므로 일어나지 않는다.
    #[must_use]
    pub fn snapshot_tree(&self) -> TreeRef {
        self.snapshot
            .entries()
            .next()
            .expect("스냅샷은 비어 있을 수 없다")
            .1
    }

    /// 대장이 선 뒤로 HEAD 가 움직였는가 — **상수 시간이다.**
    ///
    /// 참이면 이 대장은 지금 HEAD 의 것이 아니고, 그 사실이 산출에 실린다.
    #[must_use]
    pub fn head_moved(&self) -> bool {
        self.snapshot_tree().base() != self.detector.head_now
    }

    /// 파일 총수.
    #[must_use]
    pub fn total(&self) -> usize {
        self.entries.len()
    }

    /// 결박이 성립하지 않는 언어들 — 대장 **머리**에 적힌다(DESIGN §4.1).
    #[must_use]
    pub fn unbindable_languages(&self) -> Vec<&LanguageCapability> {
        self.languages
            .iter()
            .filter(|c| c.identity == IdentityGrade::Unavailable)
            .collect()
    }

    /// 제외된 파일을 규칙별로 센다. **규칙 ID 없이 제외는 없다.**
    #[must_use]
    pub fn exclusions_by_rule(&self) -> BTreeMap<&str, usize> {
        let mut out: BTreeMap<&str, usize> = BTreeMap::new();
        for e in &self.entries {
            if let FileState::Excluded { rule } = &e.state {
                *out.entry(rule.as_str()).or_insert(0) += 1;
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn 대장(states: Vec<FileState>) -> Ledger {
        Ledger {
            snapshot: Snapshot::single(
                crate::RepoId::new("t"),
                TreeRef::Committed(ObjectName::from_bytes([0; 20])),
            ),
            repos_declared: NonZeroUsize::new(1).unwrap(),
            entries: states
                .into_iter()
                .enumerate()
                .map(|(i, state)| LedgerEntry { path: RepoPath::new(format!("f{i}")), state })
                .collect(),
            languages: vec![],
            scope: ScopeSource::InferredFromPath,
            detector: DetectorFreshness {
                grammar: "g".to_owned(),
                extractor: "e".to_owned(),
                head_now: ObjectName::from_bytes([0; 20]),
            },
        }
    }

    #[test]
    fn 칸의_합은_언제나_파일_총수다() {
        let l = 대장(vec![
            FileState::Parsed {
                language: LanguageId::new("Kotlin"),
                grade: ExtractGrade::L1,
            },
            FileState::Unrecognized,
            FileState::Binary { reason: BinaryReason::NulByte },
            FileState::Excluded { rule: ExclusionRuleId::new("oversize") },
        ]);
        assert_eq!(l.counts().values().sum::<usize>(), l.total());
        assert_eq!(l.total(), 4);
    }

    #[test]
    fn 일곱_칸이_비어도_전부_보고된다() {
        // 0 을 생략하면 "그 칸이 없다"와 "0 건이다"가 같은 출력이 된다.
        let l = 대장(vec![]);
        assert_eq!(l.counts().len(), 7);
        assert!(l.counts().values().all(|n| *n == 0));
    }

    #[test]
    fn 등급이_정체성을_정한다() {
        assert_eq!(ExtractGrade::L0.identity(), IdentityGrade::Unavailable);
        assert_eq!(ExtractGrade::L1.identity(), IdentityGrade::Ordinal);
        assert_eq!(ExtractGrade::L2.identity(), IdentityGrade::Exact);
    }

    #[test]
    fn 제외는_규칙별로_세어진다() {
        let l = 대장(vec![
            FileState::Excluded { rule: ExclusionRuleId::new("vendor") },
            FileState::Excluded { rule: ExclusionRuleId::new("vendor") },
            FileState::Excluded { rule: ExclusionRuleId::new("oversize") },
        ]);
        let by = l.exclusions_by_rule();
        assert_eq!(by["vendor"], 2);
        assert_eq!(by["oversize"], 1);
    }
}
