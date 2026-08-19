//! 이 빌드가 **무엇을 만들고 무엇을 안 만드는가** — 그리고 그 사실이 캐시 키에 실린다.
//!
//! # 왜 이것이 캐시의 문제인가 (F02-1 이 F04 로 넘긴 빚)
//!
//! [`pal_core::Capable`] 은 **역직렬화되지 않는다.** 안 만든 능력을 빈 값으로 되읽으면
//! *"아무것도 없음"* 으로 위장하고, 그것이 F22-3 이 고친 병 그 자체다. 그래서 1층 캐시에
//! 실리는 것이 지금까지 `FileGraph` 가 아니라 그 일부였다.
//!
//! [F02-1 게이트](../../../docs/gates/F02-1-extractor.md)가 그 자리를 F04 로 넘기며 답을
//! 하나 적어 두었다 — *"능력 축을 키로"*. 이 파일이 그 답이다.
//!
//! # 목록을 **선언하지 않고 재현한다**
//!
//! [ADR-0004] 가 캐시 키의 규칙을 *"산출을 정하는 입력을 전부 담는다"* 로 정했고,
//! **이 빌드가 무슨 능력을 만드는가는 산출을 정하는 입력이다.** 그런데 같은 ADR 이
//! 경고한 자리가 바로 여기다:
//!
//! > 자라는 목록을 키에 **손으로 유지하면** 그 목록이 낡는 순간 같은 결함이 돌아온다.
//!
//! 그래서 능력 목록을 적지 않는다. **언어마다 빈 소스를 한 번 추출해** 어느 자리가
//! `NotBuilt` 로 나오는지 읽는다. 능력은 소스에 의존하지 않는 **빌드의 사실**이므로
//! (`kotlin.rs` 는 상수로 `not_built` 을 내고 `typescript.rs` 는 상수로 `Present` 를
//! 낸다) 빈 소스로 충분하고, **추출기가 스코프를 만들기 시작하면 다음 실행이 즉시 안다.**
//!
//! [ADR-0004]: ../../../docs/adr/0004-cache-key-covers-every-input-that-decides-the-output.md

use std::sync::OnceLock;

use pal_core::{Capable, CapabilityId, FileGraph, Language};

use crate::extractor::extractor_for;

/// 1급 언어 다섯 — 소유자 지시 2026-08-12 §1 · 2026-08-20 §1.
///
/// **표가 늘면 여기가 함께 늘어야 능력 축이 전수가 된다.** 안 늘면 새 언어의 능력이
/// 키에 안 실리고, 그 언어의 추출기를 세우는 커밋이 캐시를 무효화하지 못한다.
///
/// ⚠ **타입이 이 배열을 강제하지만 「올바른 편집」을 강제하지는 않는다.**
/// `index_of` 의 전수 `match` 는 컴파일 오류를 내지만, 새 팔에 다음 번호만 주고
/// 이 배열과 [`Shells`] 의 길이를 그대로 두면 **컴파일이 통과하고 `shell_of` 가
/// 인덱스 범위 초과로 패닉한다.** 셋이 함께 움직여야 한다(#66 사전부검).
pub const FIRST_CLASS: [Language; 5] =
    [Language::Kotlin, Language::Java, Language::JavaScript, Language::TypeScript, Language::Rust];

/// 빈 소스조차 못 읽는 빌드의 자리. **없을 자리이고, 없다고 가정하지 않는다.**
///
/// 여기 걸리면 그 언어의 캐시는 영원히 미스이고 되읽기는 실패한다 — 조용히 옛 값을
/// 돌려주는 것보다 낫다.
const PROBE_FAILED: CapabilityId = CapabilityId::new("F04", "capability-probe-failed");

/// [`FileGraph`] 의 능력 자리 넷 — **값 없이 정체만.**
///
/// `Capable<()>` 인 것이 요점이다. 값은 파일마다 다르고 **능력은 빌드마다 다르다.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphShell {
    pub exports: Capable<()>,
    pub imports: Capable<()>,
    pub export_digest: Capable<()>,
    pub scopes: Capable<()>,
}

/// 값을 버리고 정체만 남긴다.
fn strip<T>(c: &Capable<T>) -> Capable<()> {
    match c {
        Capable::Present(_) => Capable::Present(()),
        Capable::NotBuilt { capability } => Capable::NotBuilt { capability: *capability },
    }
}

impl GraphShell {
    fn of(graph: &FileGraph) -> Self {
        Self {
            exports: strip(&graph.exports),
            imports: strip(&graph.imports),
            export_digest: strip(&graph.export_digest),
            scopes: strip(&graph.scopes),
        }
    }

    /// 자리 넷을 **고정된 순서로.** 순서가 바뀌면 요약이 바뀌므로 여기가 유일한 정본이다.
    fn slots(&self) -> [(&'static str, &Capable<()>); 4] {
        [
            ("exports", &self.exports),
            ("imports", &self.imports),
            ("export_digest", &self.export_digest),
            ("scopes", &self.scopes),
        ]
    }
}

type Shells = [Capable<GraphShell>; 5];

static SHELLS: OnceLock<Shells> = OnceLock::new();
static AXIS: OnceLock<String> = OnceLock::new();

const fn index_of(language: Language) -> usize {
    match language {
        Language::Kotlin => 0,
        Language::Java => 1,
        Language::JavaScript => 2,
        Language::TypeScript => 3,
        Language::Rust => 4,
    }
}

fn probe(language: Language) -> Capable<GraphShell> {
    match extractor_for(language) {
        Capable::NotBuilt { capability } => Capable::NotBuilt { capability },
        // **빈 소스다.** 능력은 내용에 의존하지 않으므로 이것으로 충분하고, 파싱이
        // 가장 싼 입력이다.
        Capable::Present(e) => match e.extract(b"") {
            Ok(graph) => Capable::Present(GraphShell::of(&graph)),
            Err(_) => Capable::NotBuilt { capability: PROBE_FAILED },
        },
    }
}

fn shells() -> &'static Shells {
    SHELLS.get_or_init(|| FIRST_CLASS.map(probe))
}

/// 언어 하나에서 이 빌드가 만드는 자리와 안 만드는 자리.
///
/// 추출기가 없으면 껍데기도 없다 — 그 언어의 그래프가 애초에 생기지 않는다.
#[must_use]
pub fn shell_of(language: Language) -> Capable<&'static GraphShell> {
    match &shells()[index_of(language)] {
        Capable::Present(s) => Capable::Present(s),
        Capable::NotBuilt { capability } => Capable::NotBuilt { capability: *capability },
    }
}

/// **이 빌드의 능력 전부를 한 값으로** — 캐시 키의 성분이다.
///
/// 언어 다섯 × 자리 넷을 고정 순서로 훑어 **사람이 읽을 수 있는 정본 문자열**을 만든다.
/// 한 자리라도 달라지면 값이 달라지고, 그러면 **옛 빌드가 쓴 항목은 애초에 다른 키**가
/// 되어 되읽기가 안전해진다.
///
/// # 왜 요약(해시)이 아닌가
///
/// 둘째 쓰임이 **사람이 게이트에 적는 것**이기 때문이다. 해시로 적으면 축이 움직였다는
/// 것만 알고 **무엇이 움직였는지**는 모른다 — 이 저장소가 반복해서 적은 *"건수가 아니라
/// 목록"* 과 같은 규율이다. 키에 들어갈 때는 어차피 [`pal_store::CacheKey`] 가 다시
/// 요약하므로 길이는 비용이 아니다.
///
/// [`pal_store::CacheKey`]: https://docs.rs/
#[must_use]
pub fn capability_axis() -> &'static str {
    AXIS.get_or_init(|| {
        let mut out = String::new();
        for language in FIRST_CLASS {
            if !out.is_empty() {
                out.push(';');
            }
            out.push_str(language.name());
            match shell_of(language) {
                Capable::NotBuilt { capability } => {
                    out.push_str("|no-extractor=");
                    out.push_str(capability.feature);
                    out.push('/');
                    out.push_str(capability.what);
                }
                Capable::Present(shell) => {
                    for (name, slot) in shell.slots() {
                        out.push('|');
                        out.push_str(name);
                        out.push('=');
                        match slot {
                            Capable::Present(()) => out.push_str("built"),
                            Capable::NotBuilt { capability } => {
                                out.push_str("not-built:");
                                out.push_str(capability.feature);
                                out.push('/');
                                out.push_str(capability.what);
                            }
                        }
                    }
                }
            }
        }
        out
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 껍데기는_추출기의_상수와_같다() {
        // **이것이 「선언하지 않고 재현한다」가 참인지 보는 자리다.** Kotlin 은 넷 다
        // 안 만들고 TypeScript 는 넷 다 만든다 — 추출기 소스가 그렇게 적혀 있다.
        let Capable::Present(kotlin) = shell_of(Language::Kotlin) else {
            panic!("Kotlin 추출기가 사라졌다면 이 시험을 고쳐라");
        };
        for (name, slot) in kotlin.slots() {
            assert!(!slot.is_present(), "Kotlin 이 {name} 을 만든다고 나왔다");
        }

        let Capable::Present(ts) = shell_of(Language::TypeScript) else {
            panic!("TypeScript 추출기가 사라졌다면 이 시험을 고쳐라");
        };
        for (name, slot) in ts.slots() {
            assert!(slot.is_present(), "TypeScript 가 {name} 을 안 만든다고 나왔다");
        }
    }

    #[test]
    fn 추출기가_없는_언어는_껍데기가_없다() {
        // 빈 껍데기를 주면 *"자리는 넷인데 다 안 만듦"* 과 *"애초에 그래프가 없음"* 이
        // 같은 값이 된다.
        let Capable::NotBuilt { capability } = shell_of(Language::Java) else {
            panic!("Java 추출기가 생겼다면 이 시험을 고쳐라");
        };
        assert_eq!(capability.what, "java-extraction");
    }

    #[test]
    fn 능력_축은_결정적이고_네_언어를_전부_적는다() {
        assert_eq!(capability_axis(), capability_axis());
        for language in FIRST_CLASS {
            assert!(capability_axis().contains(language.name()), "{} 이 축에 없다", language.name());
        }
    }

    #[test]
    fn 능력_축은_자리를_구별한다() {
        // **★ 이것이 없으면 축이 무엇을 세는지 알 수 없다.** 자리 이름 없이 값만
        // 이으면 `exports` 와 `imports` 를 맞바꾼 두 빌드가 같은 축을 갖는다.
        let axis = capability_axis();
        assert!(axis.contains("exports=") && axis.contains("imports="), "자리 이름이 없다: {axis}");
        // Kotlin 은 안 만들고 TypeScript 는 만든다 — 축이 그 차이를 담는다.
        assert!(axis.contains("not-built:F02/kotlin-exports"), "{axis}");
        assert!(axis.contains("|exports=built"), "{axis}");
    }
}
