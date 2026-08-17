//! 좌표와 정체성 — **결박은 좌표 위에 서고, 좌표는 정체성 위에 선다.**
//!
//! # 값이 둘인 것이 이 모듈의 전부다 ([옛 F03 §2](../../../docs/plan/disposal-map.md))
//!
//! ```text
//! symbol_id    "같은 심볼인가"   바뀌면 → 정체성이 끊긴다 (orphaned)
//! body_digest  "변했는가"        바뀌면 → 결박이 stale
//! ```
//!
//! **분리한 것이 핵심이다.** 파일을 옮기면 `symbol_id` 가 바뀌지만 `body_digest` 는
//! 그대로다 — 그래서 재결박 제안이 가능하다([R-08]). 포매팅만 바꾸면 `body_digest` 가
//! 그대로다 — 그래서 [R-07] 의 거짓 양성이 죽는다. 하나로 합치면 둘 중 하나를 잃는다.

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::ledger::IdentityGrade;
use crate::repo::{RepoId, RepoPath, TreeRef};
use crate::symbol::SymbolKind;
use crate::version::ExtractorVersion;

/// 32바이트 blake3 요약을 실은 값. `SymbolId` 와 `BodyDigest` 가 이것을 공유한다.
macro_rules! digest_newtype {
    ($name:ident, $doc:literal) => {
        #[doc = $doc]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name([u8; 32]);

        impl $name {
            #[must_use]
            pub const fn from_bytes(raw: [u8; 32]) -> Self {
                Self(raw)
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

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.to_hex())
            }
        }

        impl Serialize for $name {
            fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
                s.serialize_str(&self.to_hex())
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
                let hex = String::deserialize(d)?;
                let raw = crate::repo::hex32(&hex)
                    .ok_or_else(|| serde::de::Error::custom(format!("64자 16진이 아니다: {hex}")))?;
                Ok(Self(raw))
            }
        }
    };
}

digest_newtype!(
    SymbolId,
    "심볼의 정체성. `blake3(repo ‖ 경로 ‖ 컨테이너 체인 ‖ 이름 ‖ 판별자)`.\n\n\
     **경로가 성분이므로 파일을 옮기면 바뀐다.** 그것이 의도다 — 이동은 *변경*이 아니라\n\
     *정체성 사건*이고, `body_digest` 가 그대로라는 사실이 재결박의 근거가 된다([R-08])."
);

digest_newtype!(
    BodyDigest,
    "본문이 변했는가. 구문 트리에서 주석·공백·포매팅을 제거한 정규형의 `blake3`.\n\n\
     **경로도 이름도 성분이 아니다.** 같은 코드가 다른 파일에 있으면 같은 값이고,\n\
     그것이 이동 감지를 가능하게 한다."
);

digest_newtype!(
    ExportDigest,
    "이 파일이 **밖에 노출하는 것**이 변했는가 — R-05 의 무효화 전파용.\n\n본문이 변해도 이 값은 안 변할 수 있고 그 반대도 된다. **그 차이가 요점이다** — 함수 본문만 고친 파일은 그것을 import 하는 파일들을 흔들지 않는다.\n\n**정렬·중복 제거된 집합 위에서 계산된다.** 소스 순서에 의존하면 포매터가 export 를 재배열하는 것만으로 의존 파일 전체가 무효화된다."
);

impl SymbolId {
    /// 정체성을 계산한다.
    ///
    /// 성분 사이에 `\0` 을 넣는다 — 넣지 않으면 `("ab","c")` 와 `("a","bc")` 가 같은
    /// 값이 되고, 그것이 **서로 다른 심볼을 하나로 만드는** 형태다.
    #[must_use]
    pub fn compute(
        repo: &RepoId,
        path: &RepoPath,
        container_chain: &[&str],
        name: &str,
        discriminator: &Discriminator,
    ) -> Self {
        let mut h = blake3::Hasher::new();
        h.update(b"pal-symbol-v1\0");
        h.update(repo.as_str().as_bytes());
        h.update(b"\0");
        h.update(path.as_str().as_bytes());
        h.update(b"\0");
        for c in container_chain {
            h.update(c.as_bytes());
            h.update(b".");
        }
        h.update(b"\0");
        h.update(name.as_bytes());
        h.update(b"\0");
        h.update(discriminator.kind.name().as_bytes());
        h.update(b"\0");
        h.update(&discriminator.ordinal.to_le_bytes());
        Self(*h.finalize().as_bytes())
    }
}

impl BodyDigest {
    /// 정규형 바이트열에서 계산한다. **정규화 자체는 `pal-extract` 의 일이다** —
    /// 그것이 구문 트리를 알아야 하기 때문이다.
    #[must_use]
    pub fn of_normalized(normalized: &[u8]) -> Self {
        let mut h = blake3::Hasher::new();
        h.update(b"pal-body-v1\0");
        h.update(normalized);
        Self(*h.finalize().as_bytes())
    }
}

/// 심볼 하나의 좌표 — **그리고 그것이 없을 수 있다는 사실.**
///
/// # 왜 `(SymbolId, IdentityGrade)` 쌍이 아닌가 ([옛 F03 §3.3])
///
/// [`IdentityGrade`] 에는 [`Unavailable`] 이 있는데 `SymbolId` 에는 그런 값이 없다.
/// 둘을 나란히 두면 **`Unavailable` 인데 `SymbolId` 가 있는 상태**를 타입이 허용하고,
/// 그 조합은 뜻이 없다 — 좌표가 성립하지 않는 심볼에 좌표가 있다는 말이기 때문이다.
///
/// 그래서 등급을 값의 **바깥**이 아니라 **변형**으로 둔다. `Unavailable` 에는 실을
/// 자리가 없고, 그러므로 **거기서 `SymbolId` 를 꺼낼 수 없다.**
///
/// # 그것이 「타입으로 강제」의 전부다
///
/// [`crate::Binding::new`] 는 `SymbolIdentity` 가 아니라 `SymbolId` 를 요구한다.
/// L0 에서 결박을 시도하는 코드는 **컴파일되지 않는다**:
///
/// ```compile_fail
/// # use pal_core::{Binding, BoundTime, EntityId, EntityKind, EntityOrigin, NewBinding,
/// #                ObjectName, Radius, RepoId, Snapshot, SymbolIdentity, TreeRef};
/// let snapshot = Snapshot::single(RepoId::new("r"), TreeRef::Committed(ObjectName::from_bytes([0; 20])));
/// let subject = EntityId::mint(EntityKind::new("decision"), EntityOrigin::Hand);
/// // `SymbolIdentity` 는 `SymbolId` 가 아니다 — 그리고 `Unavailable` 에서 꺼낼 수도 없다.
/// let _ = Binding::new(NewBinding {
///     subject, target: SymbolIdentity::Unavailable, note: "메모".to_owned(),
///     bound_at: snapshot, bound_at_time: BoundTime::Worktree,
///     radius: Radius::Symbol, watch: Vec::new(),
/// });
/// ```
///
/// 같은 코드가 좌표를 **꺼내고 나면** 컴파일된다 — 꺼내는 길이 `Exact` 와 `Ordinal`
/// 둘뿐이기 때문이다:
///
/// ```
/// # use pal_core::{Binding, BoundTime, Discriminator, EntityId, EntityKind, EntityOrigin,
/// #                NewBinding, ObjectName, Radius, RepoId, RepoPath, Snapshot, SymbolId,
/// #                SymbolIdentity, SymbolKind, TreeRef};
/// # let id = SymbolId::compute(&RepoId::new("r"), &RepoPath::new("a.ts"), &[], "f",
/// #     &Discriminator::new(SymbolKind::Function, 0));
/// # let snapshot = Snapshot::single(RepoId::new("r"), TreeRef::Committed(ObjectName::from_bytes([0; 20])));
/// # let subject = EntityId::mint(EntityKind::new("decision"), EntityOrigin::Hand);
/// let target = match SymbolIdentity::Exact(id) {
///     SymbolIdentity::Exact(id) | SymbolIdentity::Ordinal(id) => id,
///     // **이 팔에서 낼 수 있는 값이 없다.** 그래서 여기서 결박이 끝난다.
///     SymbolIdentity::Unavailable => return,
/// };
/// let _ = Binding::new(NewBinding {
///     subject, target, note: "메모".to_owned(), bound_at: snapshot,
///     bound_at_time: BoundTime::Worktree, radius: Radius::Symbol, watch: Vec::new(),
/// });
/// ```
///
/// [옛 F03 §3.3]: ../../../docs/plan/disposal-map.md
/// [`Unavailable`]: IdentityGrade::Unavailable
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case", tag = "grade", content = "id")]
pub enum SymbolIdentity {
    /// 스코프 해소로 유일하다.
    Exact(SymbolId),
    /// 선언 순서에 의존한다. **좌표는 있고**, 덜 믿을 만하다는 사실이 답에 실린다.
    Ordinal(SymbolId),
    /// 좌표가 없다 — L0. **결박도 적시 제시도 성립하지 않는다.**
    Unavailable,
}

impl SymbolIdentity {
    /// 등급과 좌표에서 만든다. **`Unavailable` 이면 좌표를 버린다.**
    ///
    /// 버리는 것이 요점이다 — 남겨 두면 *"좌표가 없다"* 고 적으면서 좌표를 들고 있게 된다.
    #[must_use]
    pub const fn new(grade: IdentityGrade, id: SymbolId) -> Self {
        match grade {
            IdentityGrade::Exact => Self::Exact(id),
            IdentityGrade::Ordinal => Self::Ordinal(id),
            IdentityGrade::Unavailable => Self::Unavailable,
        }
    }

    /// 이 좌표의 등급. **값을 꺼내는 길이 아니다.**
    #[must_use]
    pub const fn grade(&self) -> IdentityGrade {
        match self {
            Self::Exact(_) => IdentityGrade::Exact,
            Self::Ordinal(_) => IdentityGrade::Ordinal,
            Self::Unavailable => IdentityGrade::Unavailable,
        }
    }
}

/// 이름이 같은 선언을 가르는 것.
///
/// # `ordinal` 이 실린 것 자체가 위험의 표시다 ([R-16])
///
/// 같은 이름·같은 종류의 최상위 선언이 한 파일에 여럿이면(오버로드) 선언 **순서**로
/// 가른다. 그러면 **순서가 바뀌는 것만으로 정체성이 서로 뒤바뀐다** — 조용한 재결박이다.
///
/// 그래서 이 값이 0 이 아닌 심볼은 [`IdentityGrade::Ordinal`] 을 넘지 못한다.
/// 스코프 해소(L2)가 서면 시그니처로 가를 수 있고 그때 등급이 오른다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Discriminator {
    pub kind: SymbolKind,
    /// 같은 (이름, 종류)가 여럿일 때의 선언 순서. 유일하면 0.
    pub ordinal: u32,
}

impl Discriminator {
    #[must_use]
    pub const fn new(kind: SymbolKind, ordinal: u32) -> Self {
        Self { kind, ordinal }
    }

    /// 이 판별자가 허용하는 정체성 등급의 **상한**.
    ///
    /// 언어 등급이 아무리 높아도 순서로 가른 심볼은 `Ordinal` 을 넘을 수 없다.
    #[must_use]
    pub const fn identity_ceiling(&self) -> IdentityGrade {
        if self.ordinal == 0 {
            IdentityGrade::Exact
        } else {
            IdentityGrade::Ordinal
        }
    }
}

/// 심볼 하나를 가리키는 좌표.
///
/// **`span` 은 여기 없다.** 줄 번호는 포매팅으로 움직이므로 정체성의 성분이 될 수 없다 —
/// 라인이 필요한 자리는 [`crate::Site`] 다(stack §5.2).
///
/// # 되읽을 수 없다. 그것이 타입이 말하는 사실이다
///
/// `Serialize` 는 있고 `Deserialize` 는 없다. [`ExtractorVersion`] 이 `&'static str` —
/// **이 빌드에 박힌 상수**이기 때문이다.
///
/// 처음에는 `Snapshot` 때처럼 소유로 바꾸려 했다. 그런데 여기서는 그것이 틀린 답이다:
/// 밖에서 온 좌표의 추출기 버전은 **다른 빌드의 것일 수 있고**, 그것을 이 빌드의 상수로
/// 되읽으면 서로 다른 추출기의 산출이 같은 좌표계에 있는 것처럼 보인다. 좌표가 움직이는
/// 조건이 곧 추출기 버전이므로(stack §5.1) 그 확인 없이 역직렬화하는 경로는 있으면
/// 안 된다.
///
/// **밖에서 온 좌표를 받는 일은 버전 대조를 동반해야 하고, 그 경로는 F03 이 만든다.**
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Coord {
    pub repo: RepoId,
    pub tree: TreeRef,
    /// **추출기 버전이 좌표의 성분이다** — 문법이나 추출기가 바뀌면 좌표가 움직인다(stack §5.1).
    pub extractor: ExtractorVersion,
    pub symbol: SymbolId,
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}@{}#{}", self.repo, self.tree, self.symbol.short())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn 아이디(path: &str, name: &str, ord: u32) -> SymbolId {
        SymbolId::compute(
            &RepoId::new("r"),
            &RepoPath::new(path),
            &[],
            name,
            &Discriminator::new(SymbolKind::Function, ord),
        )
    }

    #[test]
    fn 같은_심볼은_같은_아이디다() {
        assert_eq!(아이디("a/b.kt", "f", 0), 아이디("a/b.kt", "f", 0));
    }

    #[test]
    fn 파일을_옮기면_아이디가_바뀐다() {
        // 이동은 *변경*이 아니라 *정체성 사건*이다 — R-08.
        assert_ne!(아이디("a/b.kt", "f", 0), 아이디("c/b.kt", "f", 0));
    }

    #[test]
    fn 성분_경계가_없으면_다른_심볼이_하나가_된다() {
        // 구분자 `\0` 이 없으면 ("ab","c") 와 ("a","bc") 가 같은 값이 된다.
        let a = 아이디("ab.kt", "c", 0);
        let b = 아이디("a.kt", "bc", 0);
        assert_ne!(a, b);
    }

    #[test]
    fn 순서가_다르면_다른_심볼이다() {
        assert_ne!(아이디("a.kt", "f", 0), 아이디("a.kt", "f", 1));
    }

    #[test]
    fn 순서로_가른_심볼은_정체성_등급이_묶인다() {
        // R-16 — 순서가 바뀌면 조용한 재결박이 일어난다. 등급이 그것을 표시한다.
        assert_eq!(
            Discriminator::new(SymbolKind::Function, 0).identity_ceiling(),
            IdentityGrade::Exact
        );
        assert_eq!(
            Discriminator::new(SymbolKind::Function, 1).identity_ceiling(),
            IdentityGrade::Ordinal
        );
    }

    #[test]
    fn 본문_요약은_경로를_모른다() {
        // 같은 코드가 다른 파일에 있으면 같은 값이어야 이동 감지가 성립한다.
        assert_eq!(
            BodyDigest::of_normalized(b"funf(){}"),
            BodyDigest::of_normalized(b"funf(){}")
        );
        assert_ne!(
            BodyDigest::of_normalized(b"funf(){}"),
            BodyDigest::of_normalized(b"fung(){}")
        );
    }

    #[test]
    fn 요약은_16진_왕복을_견딘다() {
        let d = BodyDigest::of_normalized(b"x");
        let j = serde_json::to_string(&d).unwrap();
        assert_eq!(serde_json::from_str::<BodyDigest>(&j).unwrap(), d);
    }
}
