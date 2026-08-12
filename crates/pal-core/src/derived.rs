//! 파생 노드의 정체성 — **출처는 속성이 아니라 정체성의 성분이다** ([R14]).
//!
//! # 이 모듈이 막는 것은 필드 수정이 아니라 병합이다
//!
//! [DESIGN §3.1](../../../docs/DESIGN.md)이 *"출처 필드는 불변이고 승격은 새 노드 생성"*
//! 을 정했다. 그 규칙은 **필드를 고쳐 쓰는 것**을 막지만 **두 노드가 하나로 병합되는
//! 것**은 막지 못한다 — 후자에서는 아무도 필드를 고치지 않았는데 값이 바뀐다.
//!
//! 선행 구현이 정확히 그 자리에서 자기 설계를 unsound 로 판정했다
//! ([연구 G §2](../../../docs/research/2026-08-11-legacy-freshness-anchor-observation.md)):
//! 파생물 id 가 `제목 + 대상 커밋 + 대상 집합` 만 해시하고 **생산자를 제외**해서,
//! 정적 엔진이 만든 판정과 에이전트가 만든 판정이 **같은 노드로 붕괴**하고
//! last-writer-wins 로 생산자 표시가 덮어써졌다. 마커는 저장됐으나 정체성을 지지
//! 않았고, 그래서 소멸했다.
//!
//! > **파생 노드의 id = hash(네임스페이스, 대상, 출처, 생산자, 재현 입력).**
//! > 넷 중 하나라도 다르면 다른 노드다. **본문은 성분이 아니다.**
//!
//! # 넷과 하나를 함께 요구하는 것이 요점이다
//!
//! | 성분 | 다르면 | 왜 |
//! |---|---|---|
//! | 출처 · 생산자 | **두 노드** | 두 엔진의 같은 판정이 나란히 선다. **붕괴한 것은 어긋날 수 없다** |
//! | 재현 입력 | **두 노드** | 다른 시점·다른 범위의 산출이 덮어쓰지 않고 쌓인다 |
//! | **본문** | **한 노드** | 같은 근거로 다시 만든 요약이 문장만 달라졌다고 새 노드가 되면 **계보가 소음이 된다** |
//!
//! 앞의 셋만 보면 *"전부 다른 값을 내는"* 해시가 만점을 받고, 마지막 하나만 보면
//! 상수를 돌려주는 해시가 만점을 받는다.
//!
//! # 이 모듈은 아직 아무도 쓰지 않는다. **그것이 의도다**
//!
//! `Synthesis`(F17) · `Finding`(F15) · `Observation`(F16)을 만드는 기능이 없다.
//! F22 문서 §1 은 *"F01 이 첫 노드를 만드는 순간 스키마가 있어야 한다. 나중에 만들면
//! 그것은 스키마가 아니라 이미 만들어진 것의 목록이다"* 라고 적었고, 슬라이스 넷은
//! 그 순서를 지키지 못했다. **여기가 이 저장소에서 규칙이 처음으로 그것이 규율하는
//! 것보다 먼저 서는 자리다.**

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::coord::SymbolId;
use crate::graph::{Producer, Provenance};
use crate::repo::{ObjectName, RepoPath, TreeRef};

/// 파생 노드가 무엇에 대한 것인가.
///
/// **변형이 둘뿐인 것은 지금 가리킬 수 있는 것이 둘뿐이기 때문이다.** `Change` 와
/// `Actor` 는 아직 없고, 그것이 생기면 변형이 는다 — 미리 두지 않는다.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeRef {
    Symbol(SymbolId),
    /// 파생물의 파생물. 계보가 여기서 생긴다.
    Derived(DerivedId),
}

/// 재현 입력 — **`extracted` 사실은 자기 재현 입력을 실어야 한다** ([DESIGN §3](../../../docs/DESIGN.md)).
///
/// # 커밋 하나가 아니다 (U14-c)
///
/// 초안은 배정 규칙 1 을 `(커밋, 추출기 버전)` 으로 적었고, 그러면 **저장소 안에서
/// 결정론적으로 계산되는데 커밋 하나에서 나오지 않는 사실**이 규칙 2 로 떨어진다 —
/// 변경 결합도 · 수정 빈도 · 도입 커밋 같은 git 이력 파생 사실이다. 그것들은 환경에
/// 의존하지 않고 같은 저장소에서 언제나 같은 답을 낸다. `observed` 로 배정하면
/// 재조달을 요구하고 보존 정책에 걸리며 판정 입력에서 `stale-observation` 으로
/// 떨어진다 — **전부 틀린 처분이다.**
///
/// 이 값이 실리면 규칙 1 이 반증 가능해진다: *"이 입력과 이 버전으로 다시 돌려서
/// 다른 답이 나오면 배정이 틀린 것이다."*
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReproInput {
    /// 트리 하나. 가장 흔한 경우.
    Tree(TreeRef),
    /// `base..head` 의 이력. **범위가 좌표의 일부**이므로 base 가 바뀌면 낡는다.
    ///
    /// 정확히는 낡는 것이 아니라 **범위가 달라진 것**이고, 그래서 `stale` 이 아니라
    /// 재계산 대상이다(§6.4-3). 재계산 전까지 산출에 자기 범위를 달고 나온다.
    History { base: ObjectName, head: ObjectName },
    /// 경로 집합 @ 트리. 표적 재계산의 입력이다.
    Files { at: TreeRef, paths: Vec<RepoPath> },
}

/// 파생 노드의 정체성.
///
/// **[`SymbolId`] 와 다른 네임스페이스에 산다** — 접두어가 그 일을 한다. 없으면 파생
/// 노드 id 가 코드 좌표와 우연히 같아질 수 있고, 그러면 하나가 다른 하나를 덮는다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DerivedId([u8; 32]);

/// 파생 노드 정체성의 네임스페이스 접두어. **[`crate::SymbolId`] 의 것과 다르다.**
const DERIVED_NAMESPACE: &[u8] = b"pal-derived-v1\0";

impl DerivedId {
    /// 정체성을 계산한다.
    ///
    /// # 성분은 다섯이고 본문은 그중에 없다
    ///
    /// `label` 은 어느 종류의 파생물인가(`Synthesis` · `Finding` · …), `targets` 는
    /// 무엇에 대한 것인가, 나머지 셋이 [DESIGN §1.2 D33](../../../docs/DESIGN.md)이
    /// 못 박은 성분이다.
    ///
    /// # 길이를 앞에 붙이는 이유
    ///
    /// [`SymbolId`] 는 성분 사이에 `\0` 을 넣는다. 여기서는 그것으로 부족하다 —
    /// **목록의 길이가 가변**이라, 구분자만 쓰면 `[a, b]` 와 `[a‖b]` 가 같은 바이트열이
    /// 될 수 있다. 각 성분 앞에 길이를 붙이면 그 경로가 닫힌다.
    #[must_use]
    pub fn compute(
        label: &str,
        targets: &[NodeRef],
        provenance: Provenance,
        producer: &Producer,
        repro: &ReproInput,
    ) -> Self {
        let mut h = blake3::Hasher::new();
        h.update(DERIVED_NAMESPACE);
        field(&mut h, label.as_bytes());

        // 대상은 **순서에 의존하지 않아야 한다** — 같은 집합을 다른 순서로 준 두 산출이
        // 다른 노드가 되면 그것도 붕괴의 반대편 고장이다.
        let mut sorted: Vec<&NodeRef> = targets.iter().collect();
        sorted.sort();
        h.update(&u32::try_from(sorted.len()).unwrap_or(u32::MAX).to_le_bytes());
        for t in sorted {
            match t {
                NodeRef::Symbol(s) => {
                    h.update(b"s");
                    h.update(s.as_bytes());
                }
                NodeRef::Derived(d) => {
                    h.update(b"d");
                    h.update(&d.0);
                }
            }
        }

        field(&mut h, provenance.name().as_bytes());

        // 생산자는 **이름만이 아니라 인자까지** 성분이다 — `provider(sast-a)` 와
        // `provider(sast-b)` 의 판정이 한 노드로 접히면 엔진 간 불일치가 성립하지 않는다.
        field(&mut h, producer.name().as_bytes());
        match producer {
            Producer::Rule { at } => field(&mut h, at.as_bytes()),
            Producer::Provider { id } => field(&mut h, id.as_bytes()),
            _ => field(&mut h, b""),
        }

        repro_into(&mut h, repro);
        Self(*h.finalize().as_bytes())
    }

    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    #[must_use]
    pub fn to_hex(self) -> String {
        let mut s = String::with_capacity(64);
        for b in self.0 {
            use fmt::Write as _;
            let _ = write!(s, "{b:02x}");
        }
        s
    }

    /// 사람이 보는 짧은 형태. **비교에 쓰지 않는다.**
    #[must_use]
    pub fn short(self) -> String {
        self.to_hex()[..12].to_owned()
    }
}

fn field(h: &mut blake3::Hasher, bytes: &[u8]) {
    h.update(&u32::try_from(bytes.len()).unwrap_or(u32::MAX).to_le_bytes());
    h.update(bytes);
}

fn repro_into(h: &mut blake3::Hasher, r: &ReproInput) {
    match r {
        ReproInput::Tree(t) => {
            field(h, b"tree");
            field(h, t.base().as_bytes());
            field(h, if t.is_committed() { b"c" } else { b"w" });
        }
        ReproInput::History { base, head } => {
            field(h, b"history");
            field(h, base.as_bytes());
            field(h, head.as_bytes());
        }
        ReproInput::Files { at, paths } => {
            field(h, b"files");
            field(h, at.base().as_bytes());
            let mut sorted: Vec<&RepoPath> = paths.iter().collect();
            sorted.sort();
            h.update(&u32::try_from(sorted.len()).unwrap_or(u32::MAX).to_le_bytes());
            for p in sorted {
                field(h, p.as_str().as_bytes());
            }
        }
    }
}

impl fmt::Display for DerivedId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_hex())
    }
}

impl Serialize for DerivedId {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.to_hex())
    }
}

impl<'de> Deserialize<'de> for DerivedId {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let hex = String::deserialize(d)?;
        let raw = crate::repo::hex32(&hex)
            .ok_or_else(|| serde::de::Error::custom(format!("64자 16진이 아니다: {hex}")))?;
        Ok(Self(raw))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;
    use crate::coord::Discriminator;
    use crate::repo::{RepoId, RepoPath};
    use crate::symbol::SymbolKind;

    fn 대상() -> Vec<NodeRef> {
        vec![NodeRef::Symbol(SymbolId::compute(
            &RepoId::new("r"),
            &RepoPath::new("a/Auth.kt"),
            &[],
            "checkToken",
            &Discriminator::new(SymbolKind::Function, 0),
        ))]
    }

    fn 트리() -> ReproInput {
        ReproInput::Tree(TreeRef::Committed(ObjectName::from_bytes([7; 20])))
    }

    /// 선행 구현이 무너진 자리 — **정적 엔진의 판정과 에이전트의 판정.**
    fn 정적엔진() -> DerivedId {
        DerivedId::compute(
            "Finding",
            &대상(),
            Provenance::Observed,
            &Producer::Provider { id: "sast".into() },
            &트리(),
        )
    }

    fn 다른_정적엔진() -> DerivedId {
        // **출처가 같고 생산자만 다르다.** 이 셋째가 없으면 픽스처가 생산자를 격리하지
        // 못한다 — 아래 시험의 주석을 보라.
        DerivedId::compute(
            "Finding",
            &대상(),
            Provenance::Observed,
            &Producer::Provider { id: "sast-b".into() },
            &트리(),
        )
    }

    fn 에이전트() -> DerivedId {
        DerivedId::compute("Finding", &대상(), Provenance::Inferred, &Producer::Agent, &트리())
    }

    // ── 붕괴 픽스처 ─────────────────────────────────────────────────────────

    #[test]
    fn 같은_대상의_판정_셋이_세_노드로_선다() {
        // **이것이 이 게이트의 정수다.** 하나로 접히면 last-writer-wins 로 생산자
        // 표시가 덮어써지고, 마커는 저장됐으나 정체성을 지지 않아 소멸한다.
        //
        // # 왜 둘이 아니라 셋인가 — 음성 대조가 잡은 자리 (2026-08-12)
        //
        // 처음에는 둘이었다: 정적 엔진(`observed`·`provider`)과 에이전트(`inferred`·
        // `agent`). **그런데 그 둘은 출처도 다르다.** `scripts/f22-2-verify.sh` 가
        // 생산자를 성분에서 빼고 돌렸을 때 이 시험이 **여전히 통과했다** — 출처만으로
        // 갈렸기 때문이다. 즉 그 픽스처는 [R14] 가 지목한 성분(생산자)을 격리하지
        // 못했고, **선행 구현의 고장을 그대로 재현해도 초록이었다.**
        //
        // 셋째(`sast-b`)가 출처를 고정한 채 생산자만 바꾼다. 생산자가 빠지면 셋이
        // 둘로 접히고 이 시험이 무너진다.
        let mut 그래프: BTreeMap<DerivedId, &str> = BTreeMap::new();
        그래프.insert(정적엔진(), "정적 엔진 A 가 본 것");
        그래프.insert(다른_정적엔진(), "정적 엔진 B 가 본 것");
        그래프.insert(에이전트(), "에이전트가 추론한 것");
        assert_eq!(그래프.len(), 3, "같은 대상의 판정들이 한 노드로 붕괴했다");
    }

    #[test]
    fn 출처만_달라도_다른_노드다() {
        let a = DerivedId::compute("F", &대상(), Provenance::Observed, &Producer::Human, &트리());
        let b = DerivedId::compute("F", &대상(), Provenance::Asserted, &Producer::Human, &트리());
        assert_ne!(a, b);
    }

    #[test]
    fn 생산자만_달라도_다른_노드다() {
        let a = DerivedId::compute("F", &대상(), Provenance::Inferred, &Producer::Agent, &트리());
        let b = DerivedId::compute(
            "F",
            &대상(),
            Provenance::Inferred,
            &Producer::MachineRecord,
            &트리(),
        );
        assert_ne!(a, b);
    }

    #[test]
    fn 같은_종류_다른_엔진도_다른_노드다() {
        // `provider(sast-a)` 와 `provider(sast-b)` — 이름만 성분이면 여기서 접힌다.
        // 접히면 **엔진 간 불일치**(F16 §1.2 의 다섯째 몫)가 성립하지 않는다.
        let a = DerivedId::compute(
            "F",
            &대상(),
            Provenance::Observed,
            &Producer::Provider { id: "sast-a".into() },
            &트리(),
        );
        let b = DerivedId::compute(
            "F",
            &대상(),
            Provenance::Observed,
            &Producer::Provider { id: "sast-b".into() },
            &트리(),
        );
        assert_ne!(a, b);
    }

    #[test]
    fn 재현_입력만_달라도_다른_노드다() {
        // SAST 의 다중 taint 흐름을 가르는 것도 여기다 — F16 §4.2-3.
        let 아이디 = |r: &ReproInput| {
            DerivedId::compute("F", &대상(), Provenance::Observed, &Producer::Human, r)
        };
        let 트리하나 = ReproInput::Tree(TreeRef::Committed(ObjectName::from_bytes([1; 20])));
        let 트리둘 = ReproInput::Tree(TreeRef::Committed(ObjectName::from_bytes([2; 20])));
        assert_ne!(아이디(&트리하나), 아이디(&트리둘));

        // 종류가 다른 재현 입력도 갈린다.
        let 이력 = ReproInput::History {
            base: ObjectName::from_bytes([1; 20]),
            head: ObjectName::from_bytes([1; 20]),
        };
        assert_ne!(아이디(&트리하나), 아이디(&이력));

        // 범위가 다르면 다른 산출이다 — `History` 는 범위가 좌표의 일부다.
        let 넓은_이력 = ReproInput::History {
            base: ObjectName::from_bytes([1; 20]),
            head: ObjectName::from_bytes([2; 20]),
        };
        assert_ne!(아이디(&이력), 아이디(&넓은_이력));
    }

    // ── 음성 대조 — **반대 방향** ────────────────────────────────────────────

    #[test]
    fn 본문만_다른_둘은_한_노드다() {
        // **id 산출에 본문이 없다는 것이 이 시험의 전부다.** 위의 넷만 보면
        // *"전부 다른 값을 내는"* 해시가 만점을 받는다. 그러면 같은 근거로 다시 만든
        // 요약이 문장만 달라져도 새 노드가 되고 **계보가 소음이 된다**(DESIGN §1.2).
        let mut 그래프: BTreeMap<DerivedId, &str> = BTreeMap::new();
        let id1 = 에이전트();
        그래프.insert(id1, "이 함수는 토큰을 검증한다.");
        let id2 = 에이전트();
        그래프.insert(id2, "이 함수는 토큰의 유효성을 확인한다.");

        assert_eq!(id1, id2, "본문이 id 의 성분이 되어 있다");
        assert_eq!(그래프.len(), 1, "본문만 다른 둘이 두 노드가 됐다");
        // **갱신이다.** id 가 같으면 갱신, 다르면 새 노드(DESIGN §1.2 의 병합 규칙).
        assert_eq!(그래프[&id1], "이 함수는 토큰의 유효성을 확인한다.");
    }

    #[test]
    fn 대상의_순서는_정체성을_바꾸지_않는다() {
        // 붕괴의 반대편 고장 — 같은 집합을 다른 순서로 준 두 산출이 갈리면
        // 그것도 계보를 소음으로 만든다.
        let s1 = NodeRef::Symbol(SymbolId::compute(
            &RepoId::new("r"),
            &RepoPath::new("a.kt"),
            &[],
            "f",
            &Discriminator::new(SymbolKind::Function, 0),
        ));
        let s2 = NodeRef::Symbol(SymbolId::compute(
            &RepoId::new("r"),
            &RepoPath::new("b.kt"),
            &[],
            "g",
            &Discriminator::new(SymbolKind::Function, 0),
        ));
        let a = DerivedId::compute(
            "F",
            &[s1.clone(), s2.clone()],
            Provenance::Inferred,
            &Producer::Agent,
            &트리(),
        );
        let b =
            DerivedId::compute("F", &[s2, s1], Provenance::Inferred, &Producer::Agent, &트리());
        assert_eq!(a, b);
    }

    #[test]
    fn 대상_집합이_다르면_다른_노드다() {
        let s = NodeRef::Symbol(SymbolId::compute(
            &RepoId::new("r"),
            &RepoPath::new("b.kt"),
            &[],
            "g",
            &Discriminator::new(SymbolKind::Function, 0),
        ));
        let mut 둘 = 대상();
        둘.push(s);
        assert_ne!(
            DerivedId::compute("F", &대상(), Provenance::Inferred, &Producer::Agent, &트리()),
            DerivedId::compute("F", &둘, Provenance::Inferred, &Producer::Agent, &트리())
        );
    }

    #[test]
    fn 목록의_경계가_없으면_다른_집합이_하나가_된다() {
        // 길이를 앞에 붙이지 않으면 `["ab"]` 와 `["a","b"]` 가 같은 바이트열이 된다.
        let a = ReproInput::Files {
            at: TreeRef::Committed(ObjectName::from_bytes([0; 20])),
            paths: vec![RepoPath::new("ab")],
        };
        let b = ReproInput::Files {
            at: TreeRef::Committed(ObjectName::from_bytes([0; 20])),
            paths: vec![RepoPath::new("a"), RepoPath::new("b")],
        };
        assert_ne!(
            DerivedId::compute("F", &대상(), Provenance::Inferred, &Producer::Agent, &a),
            DerivedId::compute("F", &대상(), Provenance::Inferred, &Producer::Agent, &b)
        );
    }

    // ── 네임스페이스 ────────────────────────────────────────────────────────

    #[test]
    fn 네임스페이스가_코드_좌표와의_충돌을_막는다() {
        // 접두어가 하는 일의 전부를 그대로 보인다 — 없으면 같은 성분이 같은 값을 낸다.
        let 성분 = b"same-components";

        let mut 좌표_네임스페이스 = blake3::Hasher::new();
        좌표_네임스페이스.update(b"pal-symbol-v1\0");
        좌표_네임스페이스.update(성분);
        let mut 파생_네임스페이스 = blake3::Hasher::new();
        파생_네임스페이스.update(DERIVED_NAMESPACE);
        파생_네임스페이스.update(성분);
        assert_ne!(좌표_네임스페이스.finalize(), 파생_네임스페이스.finalize());

        let mut 민숭한_해시 = blake3::Hasher::new();
        민숭한_해시.update(성분);
        let mut 민숭한_해시_둘 = blake3::Hasher::new();
        민숭한_해시_둘.update(성분);
        assert_eq!(
            민숭한_해시.finalize(),
            민숭한_해시_둘.finalize(),
            "접두어가 없으면 파생 노드 id 가 코드 좌표와 같아질 수 있다"
        );
    }

    #[test]
    fn 라벨이_다르면_다른_노드다() {
        // 같은 대상·출처·생산자·입력의 `Finding` 과 `Synthesis` 는 다른 것이다.
        assert_ne!(
            DerivedId::compute("Finding", &대상(), Provenance::Inferred, &Producer::Agent, &트리()),
            DerivedId::compute("Synthesis", &대상(), Provenance::Inferred, &Producer::Agent, &트리())
        );
    }

    #[test]
    fn 아이디는_16진_왕복을_견딘다() {
        let d = 에이전트();
        let j = serde_json::to_string(&d).unwrap();
        assert_eq!(serde_json::from_str::<DerivedId>(&j).unwrap(), d);
    }
}
