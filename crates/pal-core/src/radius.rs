//! 결박 반경 — **닫히지 않는 문제를 선언으로 다룬다** (F09 §3).
//!
//! > **거짓 음성은 원리적으로 닫히지 않는다.** 좌표 바깥(호출자·설정·스키마)이 변해
//! > 결정이 무효가 돼도 `live` 로 남는다. *"이 결정과 관련 있는 코드 전부"* 는 결정
//! > 불가능하다. **그러므로 닫지 않고 선언한다.**
//!
//! | 반경 | 감시 집합 | 판정 비용 |
//! |---|---|---|
//! | [`Radius::Symbol`] (기본) | 대상 하나 | 상수 |
//! | [`Radius::Callers`] | + **직접** 호출자 | 인접 조회 1회 |
//! | [`Radius::Closure`] | + k-홉 호출자 폐포 | 폐포 크기 비례 · **저장 시점 예산** |
//! | [`Radius::Files`] | + 나열된 파일의 심볼 전부 | 열거 길이 |
//!
//! **반경이 판정 결과에 함께 출력된다.** *"이 결정은 `symbol` 반경에서 live"* 는
//! *"이 결정은 유효하다"* 와 **다른 문장**이고, 그 차이가 산출에 남는 것이 이 설계의
//! 요구다. 선언은 해결이 아니지만 **은폐보다 낫다**(§6).
//!
//! # 폐포가 호출자 방향인 이유
//!
//! [`Radius::Callers`] 가 *"직접 호출자"* 이므로 **`Closure{k:1}` 이 그것과 같아야
//! 한다** — 두 반경이 다른 방향으로 자라면 *"반경을 넓혔다"* 가 무엇을 뜻하는지
//! 알 수 없어진다. 그래서 폐포도 **역방향(호출자)** 으로 k 홉 간다.
//!
//! `[f09.1.pass]` 가 그것을 합격선으로 등록했다 — *"k=1 이 `callers` 와 같다 ·
//! k 를 올리면 커진다"*. **안 커지면 k 가 안 걸린 것이다.**
//!
//! # 이 모듈은 2층을 모른다
//!
//! 확장은 [`Neighborhood`] 위에서 돈다. [`crate::BindingStatus::evaluate`] 가 조회를
//! 클로저로 받는 것과 같은 형태이고, 같은 이유다 — **좌표계가 저장 기술을 알면 안 된다**
//! (stack §4.1).

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::budget::PROVISIONAL_WATCH_PRODUCT_MAX;
use crate::coord::SymbolId;
use crate::repo::RepoPath;

/// 무엇까지 지켜보는가. **선언이지 계산이 아니다.**
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
// **내부 태그를 안 쓴다.** 이 값이 `postcard` 로 의도 저장소에 저장되는데
// postcard 는 내부 태그 열거를 *"영원히 구현하지 않는다"* 고 말한다. 외부 태그면
// JSON 도 그대로 읽힌다 — `"symbol"` · `{"closure":{"k":2}}`.
#[serde(rename_all = "snake_case")]
pub enum Radius {
    /// 결박된 심볼의 요약 하나. **기본값이다.**
    Symbol,
    /// + **직접** 호출자.
    Callers,
    /// + k-홉 호출자 폐포. **명시적 승인이 필요하고 저장 시점에 예산이 걸린다.**
    Closure { k: usize },
    /// + 나열된 파일(설정·스키마·마이그레이션)의 심볼 전부. **사람이 지정한다.**
    Files { paths: Vec<RepoPath> },
}

impl Radius {
    /// 산출에 싣는 이름 — *"이 결정은 `<이것>` 반경에서 live"*.
    #[must_use]
    pub fn name(&self) -> String {
        match self {
            Self::Symbol => "symbol".to_owned(),
            Self::Callers => "callers".to_owned(),
            Self::Closure { k } => format!("closure:{k}"),
            Self::Files { paths } => format!("files:{}", paths.len()),
        }
    }

    /// 손잡이에서 읽는다 — `symbol` · `callers` · `closure:2` · `files:a.ts,b.ts`.
    ///
    /// **모르는 것은 `None` 이고 부르는 쪽이 멈춘다.** 조용히 기본값으로 되돌아가면
    /// 사용자가 **더 넓다고 믿는 반경**에서 좁은 감시를 받는다 — 그것이 거짓 음성을
    /// 선언으로 다룬다는 이 설계의 정면 위반이다.
    #[must_use]
    pub fn parse(raw: &str) -> Option<Self> {
        let raw = raw.trim();
        if raw == "symbol" {
            return Some(Self::Symbol);
        }
        if raw == "callers" {
            return Some(Self::Callers);
        }
        if let Some(k) = raw.strip_prefix("closure:") {
            // **0 을 거부한다.** `closure:0` 은 `symbol` 과 같은데 이름이 다르다 —
            // 같은 것에 두 이름을 주면 산출의 `radius` 가 두 갈래로 적힌다.
            return k.parse::<usize>().ok().filter(|k| *k >= 1).map(|k| Self::Closure { k });
        }
        if let Some(list) = raw.strip_prefix("files:") {
            let paths: Vec<RepoPath> = list
                .split(',')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(RepoPath::new)
                .collect();
            // **빈 목록을 거부한다.** `files:` 만 적으면 감시 집합이 대상 하나가 되는데
            // 사용자는 파일을 지켜본다고 믿는다.
            return (!paths.is_empty()).then_some(Self::Files { paths });
        }
        None
    }

    /// 이 반경이 붙일 수 있는 손잡이 값들 — **화면과 오류 메시지가 함께 쓴다.**
    pub const NAMES: [&'static str; 4] = ["symbol", "callers", "closure:<k>", "files:<경로,…>"];
}

/// 반경 확장이 요구하는 조회 — **2층을 모른다.**
///
/// 구현은 `pal-store` 의 투영이고, 시험은 손으로 만든 표다. **둘이 같은 함수를
/// 지나가는 것**이 이 트레잇의 전부다.
pub trait Neighborhood {
    /// 이 심볼을 **부르는** 것들.
    fn callers_of(&self, s: SymbolId) -> Vec<SymbolId>;
    /// 이 경로의 심볼 전부.
    fn symbols_in(&self, path: &RepoPath) -> Vec<SymbolId>;
}

/// 감시 집합을 낸다. **대상은 언제나 들어 있다.**
///
/// # 정렬한다
///
/// [`BTreeSet`] 을 쓰므로 회차마다 같은 순서가 나온다. 순서가 흔들리면
/// `watch_snapshot` 의 바이트가 흔들리고, 그러면 **왕복 항등이 못 선다**
/// (`[f09.3.pass]`).
///
/// # 빈 감시 집합은 만들 수 없다
///
/// 대상을 먼저 넣으므로 결과가 비지 않는다. **빈 감시 집합은 언제나 `Live` 이고,
/// 그러면 이 기능의 반대 방향 넷 중 셋이 공짜로 통과한다** —
/// `[f09].control_off_modes` 의 열다섯째 후보다.
#[must_use]
pub fn expand(target: SymbolId, radius: &Radius, n: &impl Neighborhood) -> Vec<SymbolId> {
    let mut out = BTreeSet::new();
    out.insert(target);
    match radius {
        Radius::Symbol => {}
        Radius::Callers => out.extend(n.callers_of(target)),
        Radius::Closure { k } => {
            // **너비 우선으로 k 홉.** 이미 본 것은 다시 안 편다 — 순환이 있으면
            // 그것 없이는 안 멈춘다.
            let mut frontier = vec![target];
            for _ in 0..*k {
                let mut next = Vec::new();
                for s in frontier {
                    for c in n.callers_of(s) {
                        if out.insert(c) {
                            next.push(c);
                        }
                    }
                }
                if next.is_empty() {
                    break;
                }
                frontier = next;
            }
        }
        Radius::Files { paths } => {
            for p in paths {
                out.extend(n.symbols_in(p));
            }
        }
    }
    out.into_iter().collect()
}

/// 저장 시점의 예산 판정 — **런타임에 조용히 느려지는 대신 여기서 실패한다** (F09 §3).
///
/// > `결박 수 × 폐포 크기 ≤ 10⁶` 를 넘으면 저장을 거부한다.
///
/// `bindings_after` 는 **이 결박을 더한 뒤의** 건수다. 더하기 전으로 세면 첫 결박에서
/// 곱이 0 이 되어 어떤 폐포든 통과한다.
///
/// # Errors
/// 곱이 [`PROVISIONAL_WATCH_PRODUCT_MAX`] 를 넘으면.
pub fn check_budget(bindings_after: usize, watch_size: usize) -> Result<(), BudgetRefusal> {
    let product = bindings_after.saturating_mul(watch_size);
    if product > PROVISIONAL_WATCH_PRODUCT_MAX {
        return Err(BudgetRefusal { bindings: bindings_after, watch: watch_size, product });
    }
    Ok(())
}

/// 저장이 거부됐다. **값이 실린다** — *"예산 초과"* 만 적으면 사용자가 무엇을 줄일지 모른다.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BudgetRefusal {
    pub bindings: usize,
    pub watch: usize,
    pub product: usize,
}

impl std::fmt::Display for BudgetRefusal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "반경이 예산을 넘는다 — 결박 {} × 감시 집합 {} = {} > {}. \
             **저장 시점에 거부한다** — 런타임에 조용히 느려지는 것보다 낫다(F09 §3). \
             반경을 좁히거나(`--radius callers`) k 를 낮춰라",
            self.bindings, self.watch, self.product, PROVISIONAL_WATCH_PRODUCT_MAX
        )
    }
}

impl std::error::Error for BudgetRefusal {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coord::Discriminator;
    use crate::repo::RepoId;
    use crate::symbol::SymbolKind;
    use std::collections::BTreeMap;

    fn 심볼(path: &str, n: &str) -> SymbolId {
        SymbolId::compute(
            &RepoId::new("r"),
            &RepoPath::new(path),
            &[],
            n,
            &Discriminator::new(SymbolKind::Function, 0),
        )
    }

    /// 손으로 만든 이웃 — **투영과 같은 함수를 지나간다.**
    #[derive(Default)]
    struct 표 {
        callers: BTreeMap<SymbolId, Vec<SymbolId>>,
        files: BTreeMap<String, Vec<SymbolId>>,
    }

    impl Neighborhood for 표 {
        fn callers_of(&self, s: SymbolId) -> Vec<SymbolId> {
            self.callers.get(&s).cloned().unwrap_or_default()
        }
        fn symbols_in(&self, path: &RepoPath) -> Vec<SymbolId> {
            self.files.get(path.as_str()).cloned().unwrap_or_default()
        }
    }

    /// `a ← b ← c ← d` — d 가 c 를, c 가 b 를, b 가 a 를 부른다.
    fn 사슬() -> (표, [SymbolId; 4]) {
        let a = 심볼("a.ts", "a");
        let b = 심볼("a.ts", "b");
        let c = 심볼("a.ts", "c");
        let d = 심볼("a.ts", "d");
        let mut t = 표::default();
        t.callers.insert(a, vec![b]);
        t.callers.insert(b, vec![c]);
        t.callers.insert(c, vec![d]);
        t.files.insert("a.ts".to_owned(), vec![a, b, c, d]);
        (t, [a, b, c, d])
    }

    #[test]
    fn 기본_반경은_대상_하나다() {
        let (t, [a, ..]) = 사슬();
        assert_eq!(expand(a, &Radius::Symbol, &t), vec![a]);
    }

    #[test]
    fn 감시_집합은_절대_비지_않는다() {
        // **빈 감시 집합은 언제나 `Live` 다** — 이웃이 하나도 없어도 대상은 남는다.
        let a = 심볼("없는.ts", "혼자");
        for r in [Radius::Symbol, Radius::Callers, Radius::Closure { k: 3 }] {
            assert_eq!(expand(a, &r, &표::default()), vec![a], "{} 에서 비었다", r.name());
        }
        assert_eq!(
            expand(a, &Radius::Files { paths: vec![RepoPath::new("없는.ts")] }, &표::default()),
            vec![a]
        );
    }

    #[test]
    fn 폐포_1홉이_직접_호출자와_같다() {
        // **두 반경이 다른 방향으로 자라면 「반경을 넓혔다」가 무엇인지 알 수 없다.**
        let (t, [a, ..]) = 사슬();
        // `expand` 는 `BTreeSet` 을 지나므로 이미 정렬돼 나온다 — 여기서 다시 안 정렬한다.
        let 직접 = expand(a, &Radius::Callers, &t);
        let 폐포 = expand(a, &Radius::Closure { k: 1 }, &t);
        assert_eq!(직접, 폐포);
        assert_eq!(직접.len(), 2, "대상 + 호출자 하나");
    }

    #[test]
    fn k_를_올리면_집합이_커진다() {
        // **★ 반대 방향.** 안 커지면 k 가 안 걸린 것이다(`[f09.1.pass]`).
        let (t, [a, ..]) = 사슬();
        let 크기 = |k| expand(a, &Radius::Closure { k }, &t).len();
        assert_eq!((크기(1), 크기(2), 크기(3)), (2, 3, 4));
        // 사슬이 끝나면 더 안 큰다 — 그것도 정확한 답이다.
        assert_eq!(크기(4), 4);
    }

    #[test]
    fn 순환이_있어도_멈춘다() {
        let a = 심볼("a.ts", "a");
        let b = 심볼("a.ts", "b");
        let mut t = 표::default();
        t.callers.insert(a, vec![b]);
        t.callers.insert(b, vec![a]);
        assert_eq!(expand(a, &Radius::Closure { k: 99 }, &t).len(), 2);
    }

    #[test]
    fn 파일_반경은_그_파일의_심볼_전부다() {
        let (t, [a, ..]) = 사슬();
        let w = expand(a, &Radius::Files { paths: vec![RepoPath::new("a.ts")] }, &t);
        assert_eq!(w.len(), 4);
    }

    #[test]
    fn 손잡이를_읽고_모르는_것은_거부한다() {
        assert_eq!(Radius::parse("symbol"), Some(Radius::Symbol));
        assert_eq!(Radius::parse("callers"), Some(Radius::Callers));
        assert_eq!(Radius::parse("closure:3"), Some(Radius::Closure { k: 3 }));
        assert_eq!(
            Radius::parse("files:a.ts, b.ts"),
            Some(Radius::Files { paths: vec![RepoPath::new("a.ts"), RepoPath::new("b.ts")] })
        );
        // **★ 조용히 기본값으로 되돌아가지 않는다.** 되돌아가면 사용자가 더 넓다고
        // 믿는 반경에서 좁은 감시를 받는다.
        for 나쁜_것 in ["", "SYMBOL", "closure", "closure:0", "closure:x", "files:", "files: , "] {
            assert_eq!(Radius::parse(나쁜_것), None, "`{나쁜_것}` 를 받아들였다");
        }
    }

    #[test]
    fn 예산은_저장_시점에_거부하고_넉넉하면_거부하지_않는다() {
        // **★ 반대 방향** — 거부만 세면 *"전부 거부"* 가 통과한다.
        assert!(check_budget(1_000, 1_000).is_ok(), "정확히 10⁶ 은 넘은 것이 아니다");
        let e = check_budget(1_000, 1_001).expect_err("넘었는데 통과했다");
        assert_eq!(e.product, 1_001_000);
        // 값이 실린다 — *"예산 초과"* 만 적으면 무엇을 줄일지 모른다.
        assert!(e.to_string().contains("1001000"), "{e}");
    }
}
