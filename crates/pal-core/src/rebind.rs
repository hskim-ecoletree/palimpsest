//! 재결박 **제안 신호** — F03 §4.2 · §7.
//!
//! # 이 파일에 적용하는 함수가 없다. 그것이 대응이다
//!
//! F03 §5 는 *"파일 이동 시 자동 재결박"* 을 기각했다 — **의미가 다른 동일 코드에
//! 잘못 붙는다.** 그래서 여기는 후보를 **세는 데까지**이고, 무엇을 할지는
//! [옛 F09](../../../docs/plan/disposal-map.md)다.
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

// ─────────────────────────────────────────────────────────────────────────────
// 일괄 승인 — **F09 가 유일한 소유자다** (문서 §8 · §5 의 다섯째 행)
//
// > 같은 `body_digest` 의 새 심볼로 **재결박 제안**(자동 아님) + **같은 경로 접두사
// > 변경은 일괄 승인**.
//
// F03 은 **제안 신호 계산까지**이고(이 파일의 위쪽), 무엇을 승인할지는 여기다.
// ─────────────────────────────────────────────────────────────────────────────

/// 한 번에 승인할 수 있는 묶음 — **경로 접두어 하나의 이동.**
///
/// # 왜 접두어인가
///
/// F09 §5 가 [R-08](../../../docs/plan/00-risks.md#r-08)(`Orphaned` 폭발)의 원인을
/// **디렉터리 이동**이라 적었다. 그 사건 하나가 결박 수백 개를 한꺼번에 깨뜨리고,
/// 사람이 그것을 하나씩 승인하면 **승인이 형식이 된다** — 형식이 된 승인은 승인이 아니다.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct RebindBatch {
    /// 옛 경로 접두어 — `src/old/`.
    pub was: String,
    /// 새 경로 접두어 — `src/new/`.
    pub now: String,
    /// 이 묶음이 승인하면 옮겨 붙을 것들.
    pub proposals: Vec<RebindProposal>,
}

/// 일괄 승인이 **거부되는** 이유 — 값으로 남는다.
///
/// *"승인할 수 없다"* 만 적으면 사람이 무엇을 손으로 봐야 하는지 모른다.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BatchRefusal {
    /// 한 옛 좌표에 후보가 여럿이다. **하나를 골라주지 않는다** — 고르는 것은 사람의 일이다.
    Ambiguous { orphaned: SymbolId, candidates: usize },
    /// 신호가 약하다 — 이름이나 컨테이너가 다르다.
    ///
    /// **본문 요약이 같은 것만으로는 부족하다.** 한 줄짜리 접근자
    /// (`get x() { return this.x }`)가 실제 코퍼스에 흔하고, 그것들은 서로 본문이 같다
    /// (이 파일 머리 · F03 §4.2). **일괄에서는 그 위험이 N 배가 된다.**
    WeakSignals { orphaned: SymbolId, strength: u8 },
}

/// 일괄 승인의 판정 — **묶음 하나에 대해.**
///
/// # ★ 이 함수의 값은 「승인한다」가 아니라 **「승인할 수 없는 것을 가른다」**이다
///
/// F03 §5 가 기각한 것은 *"의미가 다른 동일 코드에 **잘못 붙는** 것"* 이고,
/// **일괄에서는 그 위험이 묶음 크기만큼 곱해진다.** 그래서 이 함수는 하나라도 걸리면
/// **묶음 전체를 거부한다** — 부분 승인은 *"어디까지 승인했나"* 를 사람이 다시 세게 한다.
///
/// # Errors
/// 후보가 여럿이거나 신호가 약한 제안이 하나라도 있으면 — **그 목록과 함께.**
pub fn approve_batch(batch: &RebindBatch) -> Result<&[RebindProposal], Vec<BatchRefusal>> {
    let mut refusals = Vec::new();

    for p in &batch.proposals {
        let 후보_수 = batch.proposals.iter().filter(|q| q.orphaned == p.orphaned).count();
        if 후보_수 > 1 {
            let r = BatchRefusal::Ambiguous { orphaned: p.orphaned, candidates: 후보_수 };
            if !refusals.contains(&r) {
                refusals.push(r);
            }
            continue;
        }
        // **신호 셋이 전부 맞아야 한다.** 본문만 같은 것은 일괄의 대상이 아니다 —
        // 손으로 하나씩 보는 것이 맞다.
        if p.signals.strength() < 3 {
            refusals.push(BatchRefusal::WeakSignals {
                orphaned: p.orphaned,
                strength: p.signals.strength(),
            });
        }
    }

    if refusals.is_empty() { Ok(&batch.proposals) } else { Err(refusals) }
}

#[cfg(test)]
mod batch_tests {
    use super::*;
    use crate::coord::Discriminator;
    use crate::repo::{RepoId, RepoPath};
    use crate::symbol::{Span, SymbolKind};

    fn 제안(path: &str, name: &str, body: &str, signals: MatchSignals) -> RebindProposal {
        let id = |p: &str, n: &str| {
            SymbolId::compute(
                &RepoId::new("r"),
                &RepoPath::new(p),
                &[],
                n,
                &Discriminator::new(SymbolKind::Function, 0),
            )
        };
        let _ = Span { byte_start: 0, byte_end: 1, line_start: 1, line_end: 1 };
        let _ = BodyDigest::of_normalized(body.as_bytes());
        RebindProposal { orphaned: id("src/old/a.ts", name), candidate: id(path, name), signals }
    }

    fn 묶음(proposals: Vec<RebindProposal>) -> RebindBatch {
        RebindBatch { was: "src/old/".to_owned(), now: "src/new/".to_owned(), proposals }
    }

    #[test]
    fn 신호_셋이_전부_맞으면_승인한다() {
        let b = 묶음(vec![제안(
            "src/new/a.ts",
            "f",
            "본문",
            MatchSignals { body: true, name: true, container: true },
        )]);
        assert_eq!(approve_batch(&b).expect("승인").len(), 1);
    }

    #[test]
    fn 신호가_약하면_묶음_전체를_거부한다() {
        // **★ 반대 방향이고 이 기능의 핵심이다.** 본문만 같은 것은 한 줄짜리 접근자에서
        // 흔하고, 일괄에서는 그 위험이 묶음 크기만큼 곱해진다(F03 §5).
        let b = 묶음(vec![
            제안("src/new/a.ts", "f", "본문", MatchSignals { body: true, name: true, container: true }),
            제안("src/new/b.ts", "g", "본문", MatchSignals { body: true, name: false, container: false }),
        ]);
        let e = approve_batch(&b).expect_err("약한 신호가 섞였는데 승인했다");
        // **부분 승인이 아니다** — 하나라도 걸리면 묶음 전체가 거부된다.
        assert!(matches!(e[0], BatchRefusal::WeakSignals { strength: 1, .. }), "{e:?}");
    }

    #[test]
    fn 후보가_여럿이면_고르지_않고_거부한다() {
        // 같은 옛 좌표에 둘 — **하나를 골라주지 않는다.**
        let 셋 = MatchSignals { body: true, name: true, container: true };
        let b = 묶음(vec![제안("src/new/a.ts", "f", "본문", 셋), 제안("src/new/c.ts", "f", "본문", 셋)]);
        let e = approve_batch(&b).expect_err("후보가 둘인데 승인했다");
        assert_eq!(e.len(), 1, "같은 좌표를 두 번 적었다");
        assert!(matches!(e[0], BatchRefusal::Ambiguous { candidates: 2, .. }), "{e:?}");
    }

    #[test]
    fn 빈_묶음은_승인할_것이_없다() {
        // **빈 것이 정확한 답이다** — 억지로 채우면 §5 가 기각한 자리다.
        assert!(approve_batch(&묶음(Vec::new())).expect("빈 묶음").is_empty());
    }
}
