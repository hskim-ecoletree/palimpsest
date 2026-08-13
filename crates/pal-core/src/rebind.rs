//! 재결박 **제안 신호** — F03 §4.2 · §7.
//!
//! # 이 파일에 적용하는 함수가 없다. 그것이 대응이다
//!
//! F03 §5 는 *"파일 이동 시 자동 재결박"* 을 기각했다 — **의미가 다른 동일 코드에
//! 잘못 붙는다.** 그래서 여기는 후보를 **세는 데까지**이고, 무엇을 할지는
//! [F09](../../../docs/plan/features/F09-grounding.md)다.
//!
//! # 신호가 셋인 이유
//!
//! `body_digest` 하나로 고르면 **본문이 같은 서로 다른 심볼**이 전부 후보가 된다 —
//! 한 줄짜리 접근자(`get x() { return this.x }`)가 대표적이고, 실제 코퍼스에 흔하다.
//! 이름과 컨테이너 체인이 그 집합을 좁힌다.
//!
//! **좁히기만 하고 지어내지 않는다** — 요약이 다르면 후보가 아니다. 이름 유사도로
//! 후보를 만들지 않는 것이 §5 의 기각을 지키는 자리다.

use crate::coord::{BodyDigest, SymbolId};
use crate::touch::SymbolNode;

/// 사라진 좌표 하나에 대한 후보 하나.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct RebindProposal {
    /// 사라진 좌표.
    pub orphaned: SymbolId,
    /// 새 좌표 후보.
    pub candidate: SymbolId,
    /// 왜 이것을 골랐나 — **사람이 승인할 때 읽는 값이다.**
    pub signals: MatchSignals,
}

/// 무엇이 맞았나. **셋 다 값으로 남는다** — *"믿을 만하다"* 같은 한 숫자로 접으면
/// 사람이 무엇을 승인하는지 모른다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub struct MatchSignals {
    /// 본문 요약이 같다. **후보의 필요조건이다** — 이것이 거짓이면 후보가 아니다.
    pub body: bool,
    /// 이름이 같다.
    pub name: bool,
    /// 컨테이너 체인이 같다.
    pub container: bool,
}

impl MatchSignals {
    /// 맞은 신호의 수 — **정렬에만 쓴다. 합격선이 아니다.**
    #[must_use]
    pub const fn strength(&self) -> u8 {
        self.body as u8 + self.name as u8 + self.container as u8
    }
}

/// 사라진 심볼들에 대한 재결박 후보.
///
/// `was` 는 결박 시점의 `(좌표, 요약)` 이고 `now` 는 지금 스냅샷의 심볼 전부다.
///
/// # 후보가 없으면 **빈 목록을 낸다**
///
/// 억지로 채우지 않는다. 채우면 그것이 §5 가 기각한 *"의미가 다른 동일 코드에 잘못
/// 붙는"* 자리다. **빈 것이 정확한 답이다** — 흡수 안 된 재배치는 `orphaned` 라는
/// 관측 가능한 사건이 되고, F03 §4.2 가 그것을 *"조용한 정체성 유실보다 낫다"* 로
/// 판단했다.
///
/// # 지금 좌표에 아직 살아 있는 심볼은 후보를 만들지 않는다
///
/// 사라지지 않았으면 재결박할 일이 없다. 그 확인 없이 돌면 이 함수는
/// *"같은 요약을 가진 심볼 쌍"* 을 세는 것이지 재결박 제안이 아니다.
#[must_use]
pub fn propose(was: &[(SymbolId, BodyDigest)], now: &[SymbolNode]) -> Vec<RebindProposal> {
    let mut out = Vec::new();
    for (orphaned, digest) in was {
        if now.iter().any(|n| n.id == *orphaned) {
            continue;
        }
        for n in now.iter().filter(|n| n.body == *digest) {
            out.push(RebindProposal {
                orphaned: *orphaned,
                candidate: n.id,
                signals: MatchSignals { body: true, name: false, container: false },
            });
        }
    }
    out
}

/// 이름과 컨테이너까지 아는 경우 — **신호 셋을 전부 채운다.**
///
/// 결박이 좌표만 들고 있으면 [`propose`] 가 낼 수 있는 것은 요약뿐이다. 이름과
/// 컨테이너를 함께 저장해 두면 후보가 좁아지고, **무엇 때문에 좁아졌는지가 값으로
/// 남는다.** 그 저장은 F09 의 결박 스키마가 정한다.
#[must_use]
pub fn propose_with_shape(
    was: &[(SymbolId, BodyDigest, String, Vec<String>)],
    now: &[SymbolNode],
) -> Vec<RebindProposal> {
    let mut out = Vec::new();
    for (orphaned, digest, name, container) in was {
        if now.iter().any(|n| n.id == *orphaned) {
            continue;
        }
        for n in now.iter().filter(|n| n.body == *digest) {
            out.push(RebindProposal {
                orphaned: *orphaned,
                candidate: n.id,
                signals: MatchSignals {
                    body: true,
                    name: n.name == *name,
                    container: n.container == *container,
                },
            });
        }
    }
    // **신호가 많은 것부터.** 같은 세기면 좌표 순 — 회차마다 순서가 달라지면
    // 사람이 보는 목록이 흔들리고, 흔들리는 목록은 승인의 근거가 못 된다.
    out.sort_by(|a, b| {
        b.signals
            .strength()
            .cmp(&a.signals.strength())
            .then_with(|| a.candidate.cmp(&b.candidate))
    });
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coord::Discriminator;
    use crate::repo::{RepoId, RepoPath};
    use crate::symbol::{Span, SymbolKind};

    fn 노드(path: &str, container: &[&str], name: &str, body: &str) -> SymbolNode {
        let chain: Vec<&str> = container.to_vec();
        SymbolNode {
            id: SymbolId::compute(
                &RepoId::new("r"),
                &RepoPath::new(path),
                &chain,
                name,
                &Discriminator::new(SymbolKind::Function, 0),
            ),
            path: RepoPath::new(path),
            container: container.iter().map(|s| (*s).to_owned()).collect(),
            name: name.to_owned(),
            kind: SymbolKind::Function,
            body: BodyDigest::of_normalized(body.as_bytes()),
            span: Span { byte_start: 0, byte_end: 1, line_start: 1, line_end: 1 },
            identity: crate::ledger::IdentityGrade::Exact,
        }
    }

    #[test]
    fn 옮긴_심볼이_후보로_나온다() {
        let 옛 = 노드("a.ts", &[], "f", "본문");
        let 새 = 노드("b/a.ts", &[], "f", "본문");
        let p = propose(&[(옛.id, 옛.body)], std::slice::from_ref(&새));
        assert_eq!(p.len(), 1);
        assert_eq!(p[0].candidate, 새.id);
        assert!(p[0].signals.body);
    }

    #[test]
    fn 후보가_없으면_빈_목록이다() {
        // **★ 반대 방향.** 억지로 채우면 §5 가 기각한 「잘못 붙는」 자리다.
        let 옛 = 노드("a.ts", &[], "f", "본문");
        let 남 = 노드("b/a.ts", &[], "g", "다른 본문");
        assert!(propose(&[(옛.id, 옛.body)], &[남]).is_empty());
    }

    #[test]
    fn 살아_있는_좌표는_제안하지_않는다() {
        // 사라지지 않았으면 재결박할 일이 없다. 이 확인이 없으면 이 함수는
        // *"같은 요약을 가진 심볼 쌍"* 을 세는 것이지 재결박 제안이 아니다.
        let 그대로 = 노드("a.ts", &[], "f", "본문");
        assert!(propose(&[(그대로.id, 그대로.body)], std::slice::from_ref(&그대로)).is_empty());
    }

    #[test]
    fn 신호_셋이_후보를_좁히고_그_이유가_값으로_남는다() {
        let 옛 = 노드("a.ts", &["C"], "m", "본문");
        let 같은_이름 = 노드("b.ts", &["C"], "m", "본문");
        let 다른_이름 = 노드("c.ts", &["D"], "n", "본문");
        let p = propose_with_shape(
            &[(옛.id, 옛.body, "m".to_owned(), vec!["C".to_owned()])],
            &[다른_이름.clone(), 같은_이름.clone()],
        );
        assert_eq!(p.len(), 2, "요약이 같은 둘이 다 후보여야 한다");
        // **신호가 많은 것이 먼저다** — 그리고 무엇이 맞았는지가 값으로 남는다.
        assert_eq!(p[0].candidate, 같은_이름.id);
        assert_eq!(p[0].signals.strength(), 3);
        assert_eq!(p[1].signals.strength(), 1);
        assert!(!p[1].signals.name && !p[1].signals.container);
    }
}
