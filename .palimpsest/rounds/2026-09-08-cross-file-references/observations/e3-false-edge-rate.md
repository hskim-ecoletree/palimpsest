# 거짓 엣지율 — `E3`·`E3-a`·`E3-b`·`E3-c`

> 회차 `2026-09-08-cross-file-references` · 2026-09-10 · 스냅샷 `palimpsest@957b9d8+worktree`
>
> **사람이 판정한다.** 잠긴 의도의 `## 잔여를 여기서 미리 선언한다` 가 그렇게 적었다 —
> *"`E3` 의 거짓 엣지율은 사람이 판정한다 — 표본 30 건의 대조를 기계가 대신 못 한다."*

## 모집단

`pal export --format cypher` 의 `REFERENCES_ACROSS` — **1262 건**.
동반 산출도 같은 수를 적는다(`REFERENCES 4393 / REFERENCES_ACROSS 1262`).

행은 `from` id 오름차순, 같은 `from` 안에서는 `to` id 오름차순으로 **완전히 정렬돼 있고
중복 쌍이 0** 이다. 그래서 정렬 키의 동률 처리까지 결정론적이다.

## ★ 등록된 규칙이 유일한 표본을 안 정한다 — `E3-b` 의 반증

`E3-b` 원문: *"정렬 키(`from` 심볼 id 오름차순)와 뽑는 위치(전체를 30 등분해 각 구간의
첫 엣지)를 **미리** 적는다."*

**1262 는 30 으로 안 나눠떨어진다**(42.0667). 「30 등분」의 렌더링이 갈린다:

| 해석 | 식 | 0-based 위치 |
|---|---|---|
| **A** | `floor(i·N/30)` | 0, 42, 84, …, 588, **631**, … , 1219 |
| **B** | `i·floor(N/30)`(마지막 구간만 64) | 0, 42, …, 588, **630**, …, 1218 |
| **C** | `ceil(i·N/30)` | 0, **43**, 85, …, 631, …, 1220 |

실제 엣지로 A↔B 가 30 중 **15 자리**, A↔C 가 **28 자리**, B↔C 가 **29 자리** 다르다
(세 해석의 합집합은 서로 다른 엣지 73 건).

★★ **그러므로 `E3-b` 는 반증이다.** 규칙은 엣지가 서기 전에 등록됐다 — 그 절반은 참이다.
그러나 그 조건이 막으려던 것은 *"여러 번 뽑고 유리한 판을 남기는 것"* 이고, **유일한
표본을 안 정하는 규칙은 그것을 못 막는다.**

⚠ **그리고 정반합 판 3 의 합이 채택한 재등록이 실행되지 않았다** —
`dialectic/3-synthesis.md:58` 이 `E3-b` 를 **ⓐ20·ⓑ10 층화**로 다시 등록하라고 채택했는데
`intent.md` 의 `E3-b` 는 `8c3eae3` 이후 안 바뀌었다. **채택이 원장에만 살고 조건에 안
닿았다.**

★ **지금 고치지 않는다.** 엣지가 선 뒤에 추출 규칙을 다시 등록하는 것은 `E3-b` 자신이
금지한 형태다. 반증으로 적고 남긴다.

## 무엇으로 판정했나 — **해석 A**

**엣지를 하나도 보기 전에 골랐다.** 근거는 문면뿐이다 — 「등분」은 나눈 조각의 폭이
같다는 뜻이고, `B` 는 마지막 구간이 64 라 그 뜻에 안 맞는다. `A` 와 `C` 는 반올림
방향만 다르고 `A` 가 첫 구간부터 채우는 쪽이다.

⚠ **이 고름 자체가 `E3-b` 반증의 내용이다** — 고를 자리가 있으면 안 됐다.

## 판정 — **거짓 엣지 0 / 30 · 0%**

상한은 10% 이고 **측정 전에 등록됐다**(`8e0259b`).

대조 방법: 출발 심볼의 파일에서 그 이름을 들여오는 `use` 줄을 찾고, 도착 심볼의 선언
줄을 읽어 **엣지의 대상이 그 이름의 실제 정의인가**를 본다.

- **ⓐ(임포트 이름 자체) 24 건** — `use` 줄이 그 이름을 들여오고 도착이 그 정의다.
  별칭 하나가 표본에 들었다(23 번 `use crate::shell::FIRST_CLASS as 일급;` → 도착이
  **원본** `FIRST_CLASS`). `A4` 가 요구한 형태 그대로다.
- **ⓑ(경로 호출의 꼬리) 6 건** — `use` 줄에 도착 이름이 없다. **그것이 정상이다** —
  ⓑ 는 임포트된 **머리**에 붙은 꼬리라 꼬리 이름은 임포트 목록에 없다. 여섯 다 함수
  본문에서 참조 자리를 직접 읽어 확인했다:

  | # | 출발 | 본문에서 읽은 참조 자리 | 도착 |
  |---|---|---|---|
  | 1 | `조각` `narrative.rs:619` | `path: RepoPath::new(path)` (`:621`) | `new` `repo.rs:45` |
  | 3 | `심볼` `plan.rs:987` | `body: BodyDigest::of_normalized(&[body])` (`:1001`) | `of_normalized` `coord.rs:137` |
  | 6 | `걸리지_않는_파일은_범위_안이다` `manifest.rs:215` | `RepoId::new("order-svc")` (`:218`) | `new` `repo.rs:20` |
  | 8 | `손으로_건_것은_승격이_아니다` `binding.rs:1104` | `BodyDigest::of_normalized(b"x")` (`:1108`) | `of_normalized` `coord.rs:137` |
  | 17 | `변경이_0_이면_이탈률이_정의되지_않는다` `plan.rs:1243` | `RepoPath::new("src/a.ts")` (`:1246`) | `new` `repo.rs:45` |
  | 28 | `불변식_1_엣지의_양_끝_노드가_존재한다` `doctor.rs:1228` | `EdgeInstance::one(` (`:1231`) | `one` `view.rs:195` |

**거짓으로 판정한 것이 하나도 없다.**

## `E3-c` — 이 표본의 검정력

**낮다. 그 사실을 판정에 적는다** — 조건이 그것을 요구한다.

n=30 에서 참 오류율이 15% 여도 관측 거짓이 3 건 이하일 확률이 절반을 넘는다.
그러므로 이 표본은 「10% 이하」를 **입증하지 못하고** 「10% 를 크게 넘지는 않는다」만
말한다. 0/30 은 95% 상한이 약 **11.6%**(규칙 3/n 의 어림)이라 **상한 10% 를 표본만으로는
못 배제한다.**

⚠ 그리고 `E3-b` 가 반증됐으므로 이 표본은 **유일하게 정해진 표본이 아니다.**
다른 해석(B·C)에서는 30 중 최대 29 자리가 다른 엣지가 뽑힌다 — **그 표본들은 안 쟀다.**

## 못 잰 것 — 이름을 댄다

- **엣지가 참조가 일어난 줄 번호를 안 싣는다.** cypher 에도 `graph.dump` 에도 없다.
  그래서 ⓑ 여섯은 함수 본문을 다시 읽어 참조 자리를 찾아야 했다. 표본이 커지면 그
  방법은 안 선다.
- **산출물이 ⓐ/ⓑ 를 안 가른다.** `REFERENCES_ACROSS` 행에도 `graph.dump` 의 엣지
  (`from`/`to` 둘뿐)에도 갈래 속성이 없다. **판 3 의 합이 채택한 층화가 지금 산출로는
  원리상 못 선다** — 위 여섯을 갈래로 안 것은 `use` 줄의 부재에서 추론한 것이다.

---

# 다시 뽑았다 — 모집단이 움직였다 ⟨2026-09-10 · 커밋 `fbccfe1`⟩

## 왜 다시 뽑나

위 판정은 `REFERENCES_ACROSS` **1262** 에서 뽑은 표본을 잰 것이다. 그 뒤 `9e04334` 가
`DL4-11` 을 고쳐 — 모듈 경로를 펼 때 `main.rs` 를 후보에 안 넣던 자리 — **모집단이
1297 로 늘었다.** 늘어난 35 건은 옛 표본에 원리상 없다.

★ **규칙을 새로 정하지 않았다.** 정렬 키도 해석 A 도 위와 같다. 위치만 `floor(i·1297/30)`
로 다시 계산했다. **한 번 뽑고 그것을 냈다.**

## 모집단과 겹침

`REFERENCES_ACROSS` **1297**(앞 1262 · **+35**). 행은 이번에도 `from` id 오름차순,
같은 `from` 안에서 `to` id 오름차순으로 완전히 정렬돼 있고 중복 쌍이 0 이다.

**앞 표본과 겹치는 것은 7 / 30** — 번호 1·12·15·16·24·28·30(새 번호 기준).
겹침은 엣지 id 쌍으로 대조했고 `(출발 이름·파일, 도착 이름·파일)` 키로 대조해도 같은
일곱이다.

## 판정 — **거짓 엣지 0 / 30 · 0%** (상한 10%)

- **ⓐ(임포트 이름 자체) 26 건** — `use` 줄이 그 이름을 들여오고 도착이 그 정의다.
- **ⓑ(경로 호출의 꼬리) 4 건** — `use` 줄에 도착 이름이 없는 것이 정상이고, 넷 다 출발
  심볼의 함수 본문에서 `<머리>::<도착이름>` 자리를 찾았다:

  | # | 출발 | 본문의 참조 자리 | 도착 |
  |---|---|---|---|
  | 1 | `조각` `narrative.rs:619` | `path: RepoPath::new(path),` (`:621`) | `new` `repo.rs:45` |
  | 2 | `스냅샷` `cascade.rs:243` | `Snapshot::single(…)` (`:244`) | `single` `repo.rs:293` |
  | 3 | `심볼` `plan.rs:987` | `&RepoPath::new(path),` (`:990`) | `new` `repo.rs:45` |
  | 7 | `감시_원소가_사라진_것과_대상이_사라진_것은_다른_사건이다` `binding.rs:943` | `BodyDigest::of_normalized(b"x")` (`:948`) | `of_normalized` `coord.rs:137` |

★ **모듈을 가리키는 엣지 둘이 표본에 들었고 둘 다 참이다** — 19 번(`use super::sha256;`
→ `mod sha256;` `install.rs:37`)과 22 번(`use crate::ledger;` → `mod ledger;`
`main.rs:25`). 참조가 가리킨 것이 **모듈 자체**이고 엣지가 그 선언을 가리킨다.

⚠ **`state.md` 가 잡아 둔 모호한 형태는 이 표본에 안 들었다** — `mod X;` 와
`pub use X::X;` 가 **같은 뿌리 파일에 있어** `use` 한 줄이 두 이름 공간을 다 들여오는
자리(`pal_core::traverse` 2 건). 위 둘은 그 짝이 없어 모호하지 않다. **그러므로 이
표본은 그 형태에 대해 아무것도 말하지 않는다.**

## 이 판정이 안 바꾸는 것

`E3-b` 는 여전히 **반증**이다 — 규칙이 유일한 표본을 안 정한다는 사실은 모집단이 바뀌어도
그대로다. `E3-c` 의 검정력 한계도 그대로다: 0/30 의 95% 상한은 약 **11.6%** 라
**상한 10% 를 표본만으로는 못 배제한다.**

## 표본 30 건 전수 — 좌표와 대조 재료 ⟨2026-09-10 · 독립 리뷰 항 11 이 요구했다⟩

앞 판은 ⓑ 넷만 표로 실었다. **「0/30」을 원장이 재검하려면 서른 건의 좌표가 다 있어야
한다** — 없으면 그 스냅샷의 색인을 다시 세워 정렬을 재현해야 하고, 그것은 재검이 아니라
재실행이다.

| # | 위치 | 출발 (이름 @ 파일:줄) | 도착 (이름 @ 파일:줄) | `use` 줄 또는 본문의 참조 자리 |
|---|---:|---|---|---|
| 1 | 0 | `조각` @ `crates/pal-core/src/narrative.rs:619` | `new` @ `crates/pal-core/src/repo.rs:45` | 본문 `:621` `path: RepoPath::new(path),` |
| 2 | 43 | `스냅샷` @ `crates/pal-core/src/cascade.rs:243` | `single` @ `crates/pal-core/src/repo.rs:293` | 본문 `:244` `Snapshot::single(…)` |
| 3 | 86 | `심볼` @ `crates/pal-core/src/plan.rs:987` | `new` @ `crates/pal-core/src/repo.rs:45` | 본문 `:990` `&RepoPath::new(path),` |
| 4 | 129 | `응답묶음을_지는_표면_전부가_아홉을_진다` @ `crates/pal-cli/tests/envelope_two_layer.rs:257` | `pal` @ `crates/pal-cli/tests/common/mod.rs:128` | `:10` `use common::{PAL, git, pal};` |
| 5 | 172 | `재귀는_선언_거르기에_안_걸린다` @ `crates/pal-core/src/projection.rs:727` | `SymbolKind` @ `crates/pal-core/src/symbol.rs:34` | `:465` `use crate::symbol::{Span, SymbolKind};` |
| 6 | 216 | `use_를_묶는다` @ `crates/pal-extract/src/rust_scopes.rs:527` | `Builder` @ `crates/pal-extract/src/scopes.rs:162` | `:33` `use crate::scopes::{Builder, ScopeRules};` |
| 7 | 259 | `감시_원소가_사라진_것과_대상이_사라진_것은_다른_사건이다` @ `crates/pal-core/src/binding.rs:943` | `of_normalized` @ `crates/pal-core/src/coord.rs:137` | 본문 `:948` `BodyDigest::of_normalized(b"x")` |
| 8 | 302 | `블록_하나` @ `crates/pal-cli/src/install.rs:790` | `Manifest` @ `crates/pal-cli/src/install/manifest.rs:171` | `:54` `use manifest::{… Manifest …};` |
| 9 | 345 | `좌표` @ `crates/pal-core/src/cascade.rs:247` | `Discriminator` @ `crates/pal-core/src/coord.rs:243` | `:236` `use crate::coord::{Discriminator, SymbolId};` |
| 10 | 389 | `노드` @ `crates/pal-core/src/rebind.rs:130` | `RepoId` @ `crates/pal-core/src/repo.rs:16` | `:127` `use crate::repo::{RepoId, RepoPath};` |
| 11 | 432 | `모르는_이름은_…_0_이다` @ `crates/pal-cli/tests/catalog_surface.rs:112` | `PAL` @ `crates/pal-cli/tests/common/mod.rs:24` | `:14` `use common::{PAL, 저장소};` |
| 12 | 475 | `유일_해소는_경로_곱을_안_올린다` @ `crates/pal-core/src/traverse.rs:276` | `Budget` @ `crates/pal-core/src/budget.rs:278` | `:34` `use crate::budget::Budget;` |
| 13 | 518 | `심볼` @ `crates/pal-core/src/file_graph.rs:467` | `SymbolKind` @ `crates/pal-core/src/symbol.rs:34` | `:465` `use crate::symbol::{Span, SymbolKind};` |
| 14 | 562 | `저장소` @ `crates/pal-cli/tests/touch_recall.rs:21` | `git` @ `crates/pal-cli/tests/common/mod.rs:122` | `:14` `use common::{git, pal};` |
| 15 | 605 | `질의` @ `crates/pal-cli/tests/envelope_two_layer.rs:113` | `pal` @ `crates/pal-cli/tests/common/mod.rs:128` | `:10` `use common::{PAL, git, pal};` |
| 16 | 648 | `capability` @ `crates/pal-extract/src/lib.rs:228` | `extractor_for` @ `crates/pal-extract/src/extractor.rs:72` | `:28` `pub use extractor::{LanguageExtractor, extractor_for};` |
| 17 | 691 | `변경이_0_이면_이탈률이_정의되지_않는다` @ `crates/pal-core/src/plan.rs:1243` | `RepoPath` @ `crates/pal-core/src/repo.rs:41` | `:63` `use crate::repo::RepoPath;` |
| 18 | 734 | `명시적_oracle_store와_default_finalization_store를_같이_쓴다` @ `crates/pal-cli/tests/round_approve_verify.rs:446` | `PAL` @ `crates/pal-cli/tests/common/mod.rs:24` | `:11` `use common::PAL;` |
| 19 | 778 | `훑기` @ `crates/pal-cli/src/install/manifest.rs:404` | `sha256` @ `crates/pal-cli/src/install.rs:37` | `:24` `use super::sha256;` |
| 20 | 821 | `매니페스트가_실물과_양방향으로_맞는다` @ `crates/pal-cli/tests/install.rs:232` | `해시` @ `crates/pal-cli/tests/common/mod.rs:62` | `:30` `use common::{… 해시};` |
| 21 | 864 | `경로_곱만_낮추면_탐색이_멈추고_남은_대기열이_세어진다` @ `crates/pal-core/src/traverse.rs:237` | `Budget` @ `crates/pal-core/src/budget.rs:278` | `:34` `use crate::budget::Budget;` |
| 22 | 907 | `스냅샷을_잰다` @ `crates/pal-cli/src/plan.rs:53` | `ledger` @ `crates/pal-cli/src/main.rs:25` | `:28` `use crate::ledger;` |
| 23 | 951 | `rust_참조가_2층까지_도착하고_화면이_답한다` @ `crates/pal-cli/tests/rust_references.rs:71` | `pal` @ `crates/pal-cli/tests/common/mod.rs:128` | `:16` `use common::{git, pal};` |
| 24 | 994 | `설치_루트` @ `crates/pal-cli/src/install/doctor.rs:93` | `MANIFEST` @ `crates/pal-cli/src/install/layout.rs:150` | `:31` `use super::layout::{DERIVED, MANIFEST, SETTINGS};` |
| 25 | 1037 | `제안` @ `crates/pal-core/src/rebind.rs:278` | `RepoPath` @ `crates/pal-core/src/repo.rs:41` | `:275` `use crate::repo::{RepoId, RepoPath};` |
| 26 | 1080 | `블록_넣기` @ `crates/pal-cli/src/install.rs:713` | `IGNORE_MARKERS` @ `crates/pal-cli/src/install/layout.rs:245` | `:50` `use layout::{… IGNORE_MARKERS, …};` |
| 27 | 1124 | `GradeRule` @ `crates/pal-core/src/schema.rs:146` | `ResolutionGrade` @ `crates/pal-core/src/graph.rs:163` | `:33` `use crate::graph::{Producer, Provenance, ResolutionGrade};` |
| 28 | 1167 | `불변식_1_엣지의_양_끝_노드가_존재한다` @ `crates/pal-core/src/doctor.rs:1319` | `one` @ `crates/pal-core/src/view.rs:195` | 본문 `:1322` `EdgeInstance::one(` |
| 29 | 1210 | `성한` @ `crates/pal-core/src/doctor.rs:1136` | `Provenance` @ `crates/pal-core/src/graph.rs:32` | `:31` `use crate::graph::{Provenance, ResolutionGrade};` |
| 30 | 1253 | `후보가_상한을_넘으면_잡힌다` @ `crates/pal-core/src/doctor.rs:1418` | `Provenance` @ `crates/pal-core/src/graph.rs:32` | `:31` `use crate::graph::{Provenance, ResolutionGrade};` |

★ **앞 표본과 겹치는 것은 7** — 번호 1·12·15·16·24·28·30.
