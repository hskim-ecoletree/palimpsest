# 사전부검 1 라운드 — 원 반환문

> `pal-premortem-sweeper` 가 냈다. 받은 것: 원 의도와 계획, 실측 좌표.
> 안 받은 것: 대화 기록 · 앞 라운드 결과 · 메인의 사고 과정.
> 2026-09-08 · 회차 `2026-09-08-cross-file-references`

조사 완료. `pal` 조회(`doctor` · `query binding.status` · `query symbol.callers/reaches` · `symbols --graph`)와 격리 사본 실행을 섞어 잰 결과다.

---

### `OutsideFile` 갈래에는 임포트된 이름이 하나도 없다 — 계획의 유일한 착지점이 빈 가지다
- 어떻게 실패하나: 계획이 지목한 자리(`counts.unresolved += 1`)에 훅을 걸고 구현을 마쳐도 **엣지가 0 건 나온다.** Rust 추출기는 `use a::b::C;` 를 모듈 스코프의 **바인딩으로 선언**하고(`rust_scopes.rs:258` `use_를_묶는다`), 그 이름에 대한 참조는 `RefResolution::Bound{ symbol: NotASymbol }` 로 해소된다. `file_edges` 는 그것을 `counts.locals` 로 흘려보낸다(`projection.rs:263-266`). `OutsideFile` 에 남는 것은 **임포트가 없는 이름뿐**이다.
- 어디가 걸리나: `crates/pal-core/src/projection.rs:248`(계획이 지목) vs `:263-266`(실제 모집단) · `crates/pal-extract/src/rust_scopes.rs:258` · `crates/pal-core/src/scope.rs:132` `BoundSymbol::NotASymbol`
- 획득: 조회 — ① `pal symbols crates/pal-core/src/projection.rs --graph --json` 로 갈래별 집계: `bound_not_a_symbol` 336 · `bound_symbol` 87 · `outside_file` **55**. 그 55 의 이름 전량: `assert_eq`24 · `Vec`7 · `Some`4 · `assert`4 · `vec`4 · `Self`2 · `Option`2 · `_`2 · `Result`·`Ok`·`None`·`serde_json`·`u32`·`std` 각 1 — **전부 prelude·std·매크로**이고 저장소 안에 정의가 없다. ② 격리 사본에 `a.rs`(`pub fn helper`)와 `b.rs`(`use crate::a::helper; helper(1)`)를 심고 `pal touch helper --json` → `coverage.unresolved = 0`, `helper` 는 `bound → symbol=not_a_symbol`
- 모집단: 저장소
- 유효성: 참
- 해악도: 금지역 (측정이 죽은 가지가 됨 — 훅은 서고 산출은 0 인데 검사가 그것을 못 가른다)
- 대상: 계획대상
- 얼마나 아픈가: 되돌릴 수 있다(코드는 안 버려짐). 걸리는 곳 셋 — `file_edges` 의 갈래 분기 · `RefCounts` 의 뜻(`locals` 주석이 *"지역 변수·파라미터"* 라고 적는데 실제로는 임포트 참조를 함께 센다) · `BoundSymbol::NotASymbol` 의 문서(임포트가 열거에 없다)

---

### 잠근 범위(`use` 직접만 · `crate::`/`super::` 제외 · `pub use` 제외)는 Rust 2018+ 에서 공집합이다
- 어떻게 실패하나: 합격선 증인이 **자기 저장소**인데, 이 저장소의 `use` 중 잠근 범위로 풀리는 것이 **0 건**이다. Rust 2018 이후 크레이트 **안**의 임포트는 문법상 반드시 `crate::`/`super::`/`self::` 로 시작하고(범위 밖), 그렇지 않은 경로는 전부 **외부 크레이트**다. 워크스페이스 안의 다른 크레이트(`pal_core::X`)는 전량 `lib.rs` 의 `pub use` 재수출을 지난다(범위 밖).
- 어디가 걸리나: `Cargo.toml:19` `edition = "2024"` · `crates/pal-core/src/lib.rs:48-133`(`pub use` 51 줄) · `crates/pal-store/src/lib.rs:23-30`(`mod projection;` + `pub use`) · `crates/pal-extract/src/rust.rs:500` `모듈_경로`
- 획득: 조회 — ① `use` 717 줄 분류: `crate::`/`super::`/`self::` **273** · `std::`/`core::` **180** · `pal_*` **82**(전부 `pub use` 경유) · `pub use` 선언 **51** · 나머지 외부 크레이트(serde·anyhow…) **131**. ② `pal query symbol.callers Capable` → `{"outcome":"symbols","symbols":[]}`, `pal query symbol.reaches Capable` → 도달 **2**(둘 다 `capable.rs` 안). `Capable` 은 `projection.rs` 한 파일에서만 14 번 참조되는 허브인데도 그렇다 — 그리고 그 14 번 전부 `use crate::capable::Capable` 을 지난다
- 모집단: 저장소
- 유효성: 참
- 해악도: 금지역 (사실이 아닌 것을 사실로 적음 — `03-shortest-path.md` §4 단계 2 의 *"여기가 제품이다"* 를 산출 0 으로 선언하게 된다)
- 대상: 계획대상
- 얼마나 아픈가: 되돌릴 수 있으나 **범위를 다시 잠가야** 한다. 걸리는 곳은 잠긴 의도의 「해소 깊이」 한 칸이고, 그 한 칸이 나머지 여섯 칸의 값을 전부 0 으로 만든다

---

### 만들 필요가 없는 축에 가장 비싼 값을 치른다 — 모집단 0 인 범위를 위해 공유 타입·`EXTRACTOR_REV`·1층 전량 무효
- 어떻게 실패하나: 잠긴 계획에서 **비싼 것**은 「`ImportSet` 에 필드 추가 → 세 언어 공유 타입 변경 → `EXTRACTOR_REV` 승급 → 1층 캐시 전량 무효」이고, **값을 내는 것**은 「어떤 임포트를 푸나」다. 그런데 값을 내는 쪽이 0 인 범위로 잠겨 있다. 범위를 `crate::`/`super::` 로 **바꾸면** 모집단이 곧바로 273 개 `use` 로 서고, `crate::a` 는 이미 `ImportSet.modules` 에 통째로 실려 있다.
- 어디가 걸리나: `crates/pal-core/src/file_graph.rs:129` `ImportSet` · `crates/pal-extract/src/lib.rs:147` `EXTRACTOR_REV`
- 획득: 조회 — 위 두 시나리오의 수치 · `pal symbols` 로 `use crate::a::helper;` → `imports.modules = ["crate::a"]` 가 이미 실림을 확인
- 모집단: 원의도
- 유효성: 참
- 해악도: 거짓신호 (회차가 「완성」으로 닫히는데 목적 기여 (d)가 그대로 빈다)
- 대상: 계획자신
- 얼마나 아픈가: 지금이면 공짜로 되돌린다. 구현 뒤면 REV 승급이 이미 저장소·CI 캐시에 박혀 있어 되돌리기가 또 한 번의 전량 무효다

---

### 「항목 이름 필드」를 더해도 (모듈, 항목) 짝이 안 선다 — 별칭과 중첩 목록에서 원본 이름·실모듈이 소실된다
- 어떻게 실패하나: 별칭은 **원본 이름이 아무 데도 안 남는다.** `use crate::a::Error as IoError;` → 바인딩 이름은 `IoError`, `ImportSet.modules = ["crate::a"]`, 어디에도 `Error` 가 없다. 이름으로 EXPORTS 를 뒤지면 `IoError` 는 없으므로 거짓 `UnresolvedRef` 가 되거나, 다른 파일의 동명 심볼에 **거짓 엣지**가 붙는다. 중첩 목록은 **실모듈이 소실된다** — `use crate::b::{c::C, d::D};` → `modules = ["crate::b"]` 뿐이고 실제 모듈 `crate::b::c`·`crate::b::d` 는 없다. 그리고 `modules` 는 정렬·중복 제거된 평평한 `Vec` 이라(`rust.rs:385`), 항목 이름을 **또 하나의 평평한 `Vec`** 으로 더하면 어느 항목이 어느 모듈에서 왔는지가 원리상 복원 불가다.
- 어디가 걸리나: `crates/pal-core/src/file_graph.rs:129-137` `ImportSet.modules` · `crates/pal-extract/src/rust.rs:500-522` `모듈_경로` · `crates/pal-extract/src/rust.rs:378-386`(정렬·dedup)
- 획득: 조회 — 격리 사본에 세 형태를 심고 `pal symbols --graph`: 바인딩 `[('IoError','value'),('IoError','type'),('C',…),('D',…)]` · `imports.modules = ["crate::a","crate::b"]`. 저장소 안 `use … as …` **7 건**
- 모집단: 저장소
- 유효성: 참
- 해악도: 실패 (거짓 엣지 · C2 *"틀린 엣지가 없는 엣지보다 나쁘다"* 정면)
- 대상: 계획대상
- 얼마나 아픈가: 타입 모양을 다시 잡아야 한다(`Vec<ImportItem{ module, name, alias }>`). 그러면 `EXTRACTOR_REV` 를 **두 번** 올리거나 첫 판을 버려야 한다

---

### `ScopeBinding` 이 임포트 출처를 안 싣는다 — 동명 임포트가 이름으로만 되짚어진다
- 어떻게 실패하나: 참조 → 바인딩까지는 서는데, 그 바인딩이 **어느 `use` 에서 왔는지**가 값에 없다. 필드는 `name`·`namespace`·`declared_at`·`visible_from`·`hoisted`·`symbol` 여섯뿐이고 `symbol` 은 `NotASymbol` 하나로 뭉개진다. 그래서 해소는 「바인딩 이름 ↔ `ImportSet` 항목 이름」의 문자열 매칭이 되고, 같은 이름이 두 모듈에서 오면(별칭 없이 `#[cfg]` 로 갈린 임포트, 조건부 `use`) **어느 쪽인지 모르는 채 하나를 고르게 된다** — `RefResolution::Ambiguous` 가 파일 안에서 막았던 것과 정확히 같은 형태가 파일 밖에서 다시 열린다.
- 어디가 걸리나: `crates/pal-core/src/scope.rs:97-131` `ScopeBinding` · `crates/pal-core/src/scope.rs:132-137` `BoundSymbol`
- 획득: 조회 — 타입 정의를 읽고, `pal symbols --graph` 산출에서 임포트 바인딩이 `symbol: not_a_symbol` 로만 나오는 것을 확인
- 모집단: 저장소
- 유효성: 참
- 해악도: 실패
- 대상: 계획대상
- 얼마나 아픈가: `ScopeBinding` 은 1층 캐시에 실리는 타입이라 여기를 고치면 REV 승급이 한 번 더 필요하다

---

### `[edge.REFERENCES]` 의 `grade` 는 고정값 하나다 — 「exact 만」이 스키마·저장·내보내기 어디에도 못 실린다
- 어떻게 실패하나: 스키마의 `grade` 는 `Fixed(ResolutionGrade)` 아니면 `PerEdge` 둘뿐이고 지금은 `"scoped"` 로 못 박혀 있다. 파일 간 엣지를 `REFERENCES` 로 넣으면 그것은 **스키마상 `scoped`** 이고 `docs/graph-schema.md` 도 그렇게 산출한다 — 계획이 「exact」라고 부르는 것이 산출 어디에도 안 나온다. 저장 쪽도 같다: `EDGE_OUT`/`EDGE_IN` 은 `SymbolId → SymbolId` 멀티맵이라 등급 칸이 아예 없고, `pal export` 는 `CREATE (a)-[:REFERENCES]->(b)` 로 등급 없이 찍는다. `per_edge` 로 바꾸면 이번엔 `pal doctor` 불변식 ⑤ 의 모집단에 `REFERENCES` 가 새로 들어온다(`needs()` 가 `GradeRule::PerEdge` 를 담는다) — **판정 축이 바뀌므로 계획의 「멈추고 올리는 자리」가 발화한다.**
- 어디가 걸리나: `schema/graph.toml` `[edge.REFERENCES] grade = "scoped"` · `crates/pal-core/src/schema.rs:471-479`(파싱) · `crates/pal-core/src/doctor.rs:391-397`(⑤ 모집단) · `crates/pal-store/src/projection.rs:258-259, 295-300` · `crates/pal-cli/src/export.rs:212-217`
- 획득: 조회 — 위 좌표 직독 + `pal doctor` 실행: 축 ⑤ 가 지금 *"(모집단이 없습니다) · 담지 못하는 자리: INTRODUCED_BY(F05)"*
- 모집단: 저장소
- 유효성: 참
- 해악도: 금지역 (사실이 아닌 것을 사실로 적음 — 파생 문서가 exact 엣지를 scoped 로 산출한다)
- 대상: 계획대상
- 얼마나 아픈가: 세 갈래 중 하나를 골라야 한다(등급 칸 신설 · 별도 엣지 라벨 · `per_edge`). 어느 쪽이든 `schema/graph.toml` + `docs/graph-schema.md` 재생성 + `check_schema` 양방향이 걸린다

---

### `pal doctor` 는 엣지를 **아예 안 읽는다** — 선언을 뒤집으면 CI 가 빨개지고, 안 뒤집으면 새 엣지가 어떤 불변식도 안 지난다
- 어떻게 실패하나: 두 길 다 막혀 있다. ① `.absent("REFERENCES", F07)` 를 유지하면 파일 간 엣지가 불변식 ①②③ 어디에도 안 들어가 **검사 없이 산다.** ② `.holding` 으로 뒤집으면, `build_view` 가 `symbols`·`bindings` 만 받고 `pal doctor` 는 `How::SymbolsOnly` 로 붙어 **엣지를 지운다** — 모집단이 0 이 되고, CI 게이트가 `checked.checked <= 0` 을 실패로 판정한다.
- 어디가 걸리나: `crates/pal-cli/src/doctor.rs:139`(`How::SymbolsOnly`) · `:192` `build_view` · `:315-316` `.absent("File"/"REFERENCES", F07)` · `crates/pal-cli/src/attach.rs:38-47` · `scripts/check-round-doctor.mjs:29-33`
- 획득: 조회 — `pal doctor` 실행 산출에 축 ①②③ 의 *"담지 못하는 자리"* 로 `REFERENCES(F07)` · `File(F07)` 이 실제로 찍힌다. 게이트 스크립트 직독
- 모집단: 저장소
- 유효성: 참
- 해악도: 금지역 (측정이 죽은 가지가 됨)
- 대상: 계획대상
- 얼마나 아픈가: 되돌릴 수 있으나, 옳게 하려면 `attach::How` 와 `build_view` 를 함께 넓혀야 하고 그 순간 `[f22.4]` 모집단이 움직인다 — `attach.rs:19-26` 이 *"남의 게이트를 건드리지 않는다"* 로 이미 잠가 둔 자리다

---

### `UnresolvedRef` 는 사전 등록된 네 칸 계약을 그대로 지어야 하고, `attempts` 는 이 회차에 원천이 없다
- 어떻게 실패하나: `check_schema` 방향 3 은 `Built` 노드에 대해 `key + attrs` 와 Rust 타입의 `pub` 필드가 **정확히** 같기를 요구한다. 그러므로 `UnresolvedRef` 를 세우려면 필드가 `{site, name, reason, attempts}` 여야 하고, `attempts` 는 `producer = "machine-record"` 인 **관측된 해소 시도**다 — 이 회차에는 그 원천이 없다. 빈 배열로 채우면 스키마 주석이 이름 붙인 그 형태가 그대로 재현된다: *"선행 구현의 `runs` 가 146 건 전부에서 비어 있던 자리가 정확히 그것이다"*. 그리고 노드를 세워도 **그것을 잇는 엣지가 스키마에 없다** — `[edge.UNRESOLVED]` 가 등록돼 있지 않다(등록된 엣지는 `BOUND_TO`·`REFERENCES`·`AUTHORED_BY`·`TOUCHES`·`FOLLOWS`·`MANIFESTS_AT`·`INTRODUCED_BY`·`RESOLVED_BY` 여덟뿐).
- 어디가 걸리나: `schema/graph.toml` `[node.UnresolvedRef]`(`key`·`attrs`·주석) · `xtask/src/main.rs:1258-1300`(방향 3) · `crates/pal-core/src/touch.rs:256` · `schema/graph.toml` 의 엣지 목록
- 획득: 조회 — 스키마·xtask·touch.rs 직독, `grep "^\[edge\." schema/graph.toml` 로 엣지 여덟 확인
- 모집단: 저장소
- 유효성: 참
- 해악도: 실패 (`cargo xtask check` 스키마 정합) + 거짓신호(`attempts` 를 빈 배열로 채우면)
- 대상: 계획대상
- 얼마나 아픈가: 사전 등록된 계약을 고치는 것이라 **초석 예외**에 가깝다. 걸리는 곳 넷 — 스키마 · 타입 · `docs/graph-schema.md` 재생성 · `pal doctor` 의 `absence()` 경로

---

### 멈추고 올리는 자리의 기준선이 틀렸고, 그 지표는 이 기능에 원리상 둔감하다
- 어떻게 실패하나: 계획은 *"결박 25 건의 낡음이 움직이면"* 으로 잠갔는데 실제 결박은 **30 건**이고 그중 **4 건이 이미 `stale`** 이다. 25 는 `03-shortest-path.md` §2 의 2026-09-07 실측이자 `.palimpsest/intent/bindings.jsonl` 의 건수이고, 산 저장소는 30 이다. 더 나쁜 것은 **둔감성**이다 — 30 건이 전부 `radius: "symbol"` 이고, `expand()` 는 `Radius::Callers`/`Closure` 에서만 `callers_of`(= REFERENCES 역방향)를 부른다. 즉 파일 간 엣지가 몇 건이 서든 **결박 낡음은 원리상 안 움직인다.** 이 멈춤 조건은 소스를 편집했을 때만 발화하고, 그것은 이 기능이 아니라 아무 회차나 발화시키는 신호다.
- 어디가 걸리나: `crates/pal-core/src/radius.rs:132-162` `expand` · `crates/pal-core/src/radius.rs:47-56` `Radius` · `docs/plan/03-shortest-path.md` §2 「결박 25건」
- 획득: 조회 — `pal query binding.status --json` → 30 건 · `fresh` 26 · `stale` 4 · 전량 `radius: "symbol"`. `bindings.jsonl` 은 25 건
- 모집단: 회차기록
- 유효성: 참
- 해악도: 금지역 (사실이 아닌 것을 사실로 적음 — 잠긴 의도의 수가 실측과 다르다)
- 대상: 계획자신
- 얼마나 아픈가: 문장 하나. 지금 고치면 공짜, 회차 끝에 발견하면 잠긴 의도를 사후 수정하는 형태가 된다

---

### 의도 저장소의 JSONL 내보내기가 5 건 뒤처져 있다 — 이 회차가 권할 「색인을 지운다」가 번지면 그 5 건이 사라진다
- 어떻게 실패하나: 산 저장소에 결박 30 건, 커밋된 `bindings.jsonl` 에 25 건. **query 에만 있는 5 건**: `1cf63ec5e0e9937a` · `39b92bd51bbc82c6` · `819fb73f9caf7f22` · `91cdc3add647a3f7` · `cbd70e3e8dfca3ba`(파일 mtime 2026-09-07 21:51 이후 생성). `pal intent` 의 자기 설명이 *"재구축 불가한 것의 유일한 복구 경로"* 다. 이 회차는 `ImportSet` 변경 + `EXTRACTOR_REV` 승급으로 **1층 캐시 전량 무효**를 일으키고, `RefCounts` 를 건드리면 2층 postcard 행도 움직인다 — `projection.rs:81-92` 가 그 구제책을 *"`rm -rf .palimpsest/index.redb .palimpsest/cache`"* 로 적어 두었다. 당황한 손이 `rm -rf .palimpsest` 로 한 칸 넓히면 5 건은 복구 경로가 없다.
- 어디가 걸리나: `.palimpsest/intent/bindings.jsonl` · `crates/pal-core/src/projection.rs:81-92`(구제 절차 주석) · `crates/pal-store/src/projection.rs:80,102-103`(`META` 에 `built_for` 만 있고 형식 판이 없다)
- 획득: 조회 — 두 집합의 차집합을 실제로 계산
- 모집단: 저장소
- 유효성: 참
- 해악도: 금지역 (데이터 손실)
- 대상: 계획대상
- 얼마나 아픈가: 지금 `pal intent` 로 한 번 내보내면 닫힌다. 잃으면 못 되돌린다

---

### 합격선의 「거짓 엣지율 표본」은 산출이 0 일 때 만점을 준다 — 음성 대조가 없다
- 어떻게 실패하나: 계획의 증인은 *"`pal touch` 가 다른 파일의 호출자를 찍고, 표본을 손으로 대조해 거짓 엣지율을 산출한다"* 다. 첫 시나리오와 둘째 시나리오가 참이면 산출이 0 이고, **0 건의 표본에서 거짓 엣지율은 0%** 다 — 합격선이 최고점으로 통과한다. 이 저장소의 확립된 관행(`gate-negative-control-pattern` · `stitching.rs`·`rust_references.rs` 의 *"엣지가 0 이면 아래 전부가 공짜로 통과한다"* 하한)이 이 합격선에는 없다. 그리고 `corpus/criteria.toml` 에 `[f07…]`·`[f08…]` 절이 **하나도 없어** 사전 등록된 하한을 물려받을 자리도 없다.
- 어디가 걸리나: 계획의 「합격선 증인」칸 · `corpus/criteria.toml`(f07/f08 부재) · 대조군: `crates/pal-cli/tests/rust_references.rs:12-13`, `crates/pal-cli/tests/stitching.rs:12-15`
- 획득: 조회 — `grep "^\[f07\|^\[f08" corpus/criteria.toml` 이 0 건, 두 통합 시험의 하한 문구 직독
- 모집단: 규약
- 유효성: 참
- 해악도: 금지역 (측정이 죽은 가지가 됨)
- 대상: 계획자신
- 얼마나 아픈가: 합격선에 「파일 간 엣지 ≥ N」과 「N 을 0 으로 두면 실패한다」를 사전 등록하면 닫힌다. 안 하면 회차가 초록으로 닫히고 기능은 없다

---

### `pal touch` 의 참인 경고 두 줄 — 지우면 거짓이 되고, 남기면 「제품이다」와 모순된다
- 어떻게 실패하나: 화면이 지금 두 줄을 찍는다. ⓐ *"`x.foo()` 와 `S::foo()` 는 아직 안 셉니다 — 멤버·경로 해소가 **F07 미구축**입니다"* ⓑ *"**파일 안의 관계만입니다** — 파일 경계를 넘는 것은 F07 미구축입니다"*. 이 회차는 **ⓑ 만** 다루고 ⓐ(L2c 멤버 해소)는 범위 밖이다. 그런데 근거가 둘 다 「F07 미구축」이라 F07 을 구축으로 선언하면 **ⓐ 가 참인 채로 근거를 잃는다.** `pal-query::capabilities()` 에서 `F07 cross-file-resolution` 을 빼면 `pal query --list` 의 「아직 못 만든 능력」에서도 사라져, 사용자는 `호출자 0` 을 *"아무도 안 부른다"* 로 읽는다 — 그 오독은 이 저장소에서 **이미 한 번 실측됐다**(`resolve_shadowing` 의 `호출자 0`).
- 어디가 걸리나: `crates/pal-cli/src/touch.rs:411-419` · `crates/pal-query/src/lib.rs:654` `CapabilityId::new("F07", "cross-file-resolution")`
- 획득: 조회 — 두 좌표 직독, `pal query --list` 실행 산출에 `F07 cross-file-resolution` 이 실제로 찍힘
- 모집단: 저장소
- 유효성: 참
- 해악도: 금지역 (사실이 아닌 것을 사실로 적음)
- 대상: 계획대상
- 얼마나 아픈가: 능력 축을 F07 하나로 두지 말고 갈라야 한다(`cross-file-import` / `member-resolution`). 걸리는 곳 셋 — `capabilities()` · `print_facts` · `doctor.rs` 의 `.absent(…, F07)` 둘

---

### TypeScript 의 `ImportSet` 새 필드는 빈 벡터가 되고, 그 빈 값을 「안 만듦」이라 적을 자리가 없다
- 어떻게 실패하나: `imports` 의 `Capable` 껍데기는 **타입 하나 단위**(`imports: Capable<ImportSet>`)이지 필드 단위가 아니다. TypeScript 추출기는 `modules` 를 채우고 `Capable::Present` 로 내보낸다. 여기에 항목 이름 필드를 더하면 TS 산출은 `modules` 는 값이고 새 필드는 **빈 벡터**가 되어 *"이 파일이 이름으로 아무것도 임포트 안 한다"* 와 *"이 빌드가 TS 항목 이름을 안 만든다"* 가 **같은 출력**이 된다 — ADR-0002 와 `file_graph.rs:15-22`(*"빈 값으로 세우지 않는다"*)가 정면으로 걸린다.
- 어디가 걸리나: `crates/pal-core/src/file_graph.rs:129`(`ImportSet`)·`:204`(`imports: Capable<ImportSet>`) · `crates/pal-extract/src/typescript.rs:145,155,163,193,416,438`
- 획득: 조회 — 타입과 두 추출기 직독. Kotlin 은 `kotlin.rs:158` 에서 `not_built` 라 이 문제에 안 걸린다
- 모집단: 저장소
- 유효성: 참
- 해악도: 거짓신호
- 대상: 계획대상
- 얼마나 아픈가: 필드마다 `Capable` 을 씌우거나 TS 도 함께 채워야 한다. 후자면 「Rust 만」이라는 잠금이 깨지고, 전자면 공유 타입의 모양이 또 한 번 움직인다

---

### `EXTRACTOR_REV` 를 올리면 `scripts/f04-verify.py` 의 치환 대상이 사라진다 — 그리고 그 스크립트는 CI 에 없다
- 어떻게 실패하나: 그 스크립트는 `'pub const EXTRACTOR_REV: &str = "f02-rust-scope";'` 라는 **리터럴 문자열**을 소스에서 찾아 바꿔 「축을 움직였는데 캐시 적중이 0 인가」를 잰다. REV 를 올리면 `찾을 not in text` 가 되어 `어긋남(…)` 을 적고 그 변이는 **건너뛴다** — F04 의 음성 대조 하나가 조용히 꺼진다. 그리고 `.github/workflows/ci.yml` 에 이 스크립트가 없어 **CI 는 그 사실을 안 말한다.**
- 어디가 걸리나: `scripts/f04-verify.py:184-190`(변이 표)·`:233-239`(치환 대상 부재 처리) · `.github/workflows/ci.yml`(`cargo xtask check`·`cargo xtask test`·`check-round-doctor.mjs` 뿐)
- 획득: 조회 — 스크립트와 워크플로 직독, `grep -rl "f02-rust-scope"` 로 박힌 자리 확인(코드 1 · 게이트 문서 1 · 스크립트 1 · 회차기록 4)
- 모집단: 자기장치
- 유효성: 참
- 해악도: 거짓신호 (수동 실행 시 실패로 드러나지만 CI 는 침묵)
- 대상: 계획대상 (낡음 범주)
- 얼마나 아픈가: 한 줄. 다만 「REV 를 올리면 여기도 고친다」가 어디에도 안 적혀 있어 다음 승급에서 또 난다

---

### `RefCounts` 에 갈래를 더하면 2층 postcard 행이 또 움직이고, 이번에도 판 표시가 없다
- 어떻게 실패하나: 파일 간 해소를 세려면 `RefCounts` 에 갈래가 하나 더 붙는다. 저장은 postcard(자리 기반)이고 `META` 에는 `built_for`(스냅샷 문자열) 하나만 있다 — **형식 판이 없다.** `pal touch`·`pal query`·`pal bind`·`pal narrative` 는 `How::Stitching` 으로 무대를 새로 쓰므로 살아남지만, `pal export` 는 `Projection::open_read_only` 로 붙어 **옛 행을 그대로 읽는다.** #130 이 `ambiguous` 를 더하며 같은 일을 냈고 그때 남긴 것은 **주석 하나**(구제 절차)이지 장치가 아니다.
- 어디가 걸리나: `crates/pal-core/src/projection.rs:81-92`(주석) · `crates/pal-store/src/projection.rs:80,102-103`(`META`) · `crates/pal-cli/src/export.rs:118-119`(read-only) · `crates/pal-cli/src/attach.rs:97`
- 획득: 조회(좌표·경로) + **추정**(실제 디코드 실패는 못 돌려 봤다 — 옛 판 바이너리로 세운 색인을 새 판으로 읽는 조합을 만들려면 두 번 빌드해야 하고 이 환경에서 못 했다)
- 모집단: 저장소
- 유효성: 추정
- 해악도: 실패
- 대상: 계획대상
- 얼마나 아픈가: `META` 에 형식 판 하나를 넣고 불일치를 「낡음」으로 산출하면 닫힌다. 안 넣으면 다음 갈래 추가에서 세 번째로 난다

---

### 잠긴 계획의 「exact 만」이 최단 경로 §4 단계 2 의 합격선 문면을 조용히 좁힌다
- 어떻게 실패하나: `03-shortest-path.md` §4 의 2 단계 합격선 칸은 *"「호출자 7 (**exact 5 · candidate 2**)」 줄과 「내가 모르는 것」 줄이 이 자리다"* 로 적혀 있다. 잠긴 계획은 *"exact 만. `candidate` 는 안 싣는다"* 다. 그러면 「제품이다」라고 적힌 그 줄의 절반이 안 서는데, 회차는 그 문서의 2 단계를 닫았다고 기록하게 된다. 사유를 안 적으면 그것이 조용한 축소다(`AGENTS.md` 의 철회 규칙).
- 어디가 걸리나: `docs/plan/03-shortest-path.md` §4 표의 2 단계 행 · 계획의 「엣지 등급」칸
- 획득: 조회 — 문서 직독
- 모집단: 원의도
- 유효성: 참
- 해악도: 거짓신호
- 대상: 계획자신
- 얼마나 아픈가: 사유 한 문단이면 철회로 성립한다. 안 적으면 다음 회차가 「2 단계는 닫혔다」를 입력으로 받는다

---

## 내가 기각한 것

| 제목 | 어떻게 실패하나(라고 적었던 것) | 어디가 걸리나 | 획득 | 모집단 | 유효성 | 해악도 | 대상 | 얼마나 아픈가 |
|---|---|---|---|---|---|---|---|---|
| `rust_references.rs` 의 `unresolved > 0` 이 깨진다 | 파일 간 해소가 미해소를 먹어 하한 단언이 0 이 된다 | `crates/pal-cli/tests/rust_references.rs:83` | 조회 — 픽스처의 미해소 원천이 `String::new()` 뿐임을 확인. `String` 은 prelude 라 어떤 `use` 로도 이 회차가 못 푼다 | 저장소 | 거짓 | 실패 | 계획대상 | 안 깨진다. 단 이 시험이 파일 간 회귀를 **하나도 못 잰다**는 사실은 남는다(픽스처가 단일 파일이다) |
| `EXTRACTOR_REV` 승급이 `SymbolId` 를 움직여 결박 30 건이 전부 `orphaned` 가 된다 | 좌표에 추출기 버전이 성분으로 들어 있으므로 REV 를 올리면 모든 결박이 끊긴다 | `crates/pal-core/src/coord.rs:106-130` | 조회 — `SymbolId::compute` 의 성분은 `repo`·`path`·`container_chain`·`name`·`kind`·`ordinal` 여섯뿐이고 추출기 버전이 **없다**. 버전은 `Coord` 와 1층 캐시 키에만 있다 | 저장소 | 거짓 | 금지역 | 계획대상 | 안 일어난다. 결박은 안 움직인다 |
| `stitching.rs` 의 `coverage.unresolved > 0` 이 깨진다 | 파일 간 해소가 TS 픽스처의 미해소를 먹는다 | `crates/pal-cli/tests/stitching.rs:112` | 조회 — 그 픽스처의 미해소 원천은 `console` 이고 이 회차는 **Rust 만**이라 TS 경로를 안 지난다 | 저장소 | 거짓 | 실패 | 계획대상 | 안 깨진다 |
| `ExportSet` 이 `pub` 만 담아 `pub(crate)` 대상이 대량의 거짓 `UnresolvedRef` 가 된다 | 크레이트 안 참조가 EXPORTS 에 없어 「정의 없음」으로 적힌다 | `crates/pal-extract/src/rust.rs:411-419` · `crates/pal-cli/src/ledger.rs:377` | 조회 — 세어 보니 저장소 전체에서 `pub(crate)` 26 · `pub(super)` 6 · `pub(in )` 1 로 **33 건**뿐이고 「대량」이 아니다. 게다가 잠긴 범위가 `crate::` 를 빼서 그 경로에 **애초에 도달하지 않는다** | 저장소 | 거짓 | 거짓신호 | 계획대상 | 지금 범위에서는 발화 불가. 범위를 `crate::` 로 옮기면 그때 다시 재야 한다 |

---

새 범주: **모집단이 공집합이 되는 범위 잠금**(잠근 축이 대상 언어의 문법과 어긋나 증인 저장소에서 산출이 0 이 되는 자리) · **원리상 둔감한 멈춤 조건**(멈춤 지표가 그 회차가 만드는 것에 대해 구조적으로 반응할 수 없는 자리)
