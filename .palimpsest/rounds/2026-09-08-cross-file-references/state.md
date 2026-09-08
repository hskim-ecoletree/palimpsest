# 상태 — 2026-09-08-cross-file-references

## 지금 단계
**정반합 판 1 진행 중** — 판정할 물음: *"이 회차의 「해소 깊이」를 어디로 다시 잠그나,
그리고 그것이 전환인가 축소인가."* 설계자 산출 대기.
착수 기준선은 아직 돌고 있다.

⚠ **사전부검 1 라운드가 잠긴 계획을 무너뜨렸다** — `PM1-01`·`PM1-02` 둘 다 금지역.
잠근 「해소 깊이」(`use` 직접 임포트만)가 이 저장소에서 공집합이고, 계획이 지목한
착지점(`RefResolution::OutsideFile`)에 임포트된 이름이 하나도 없다.
**범위를 다시 잠그는 것은 전환이고 전환은 승격이다.**

## 착수 커밋
`6b6cb6d`

## 무엇이 끝났나
- 인터뷰 3 라운드 (상한 도달 · 다섯 범주 다 열림). 잠근 것 일곱은 `intent.md` 의 `## 원문`.
- 착수 시점 관측(RED) — `observations/red.md`

## 실행 중에 알아 둘 것 — 코드 실측
- `crates/pal-core/src/file_graph.rs` 의 `ImportSet` 주석: *"아직 읽는 쪽이 없다
  (2026-09-08 실측 · #130) — 어떤 질의·투영·화면도 이 값을 안 읽는다."*
  **이 회차가 `imports` 의 첫 소비자다.**
- `crates/pal-extract/src/rust.rs` 의 `표면()` 은 익스포트를 **최상위이고 정확히 `pub`**
  인 것만 담는다. `pub(crate)`·`pub(super)`·중첩 `pub` 은 안 담는다. 사유는 `stitch_of`
  가 최상위만 EXPORTS 로 옮기기 때문(`ledger.rs:377`).
  → **실측: 이 저장소의 `pub(crate)` 는 15 건이고 `pub` 은 646 건이다.** 한계는 있으나 작다.
- `ExportSet.digest()` 는 `names`·`star_from`·`has_default` 를 먹는다. 임포트에 이름을
  더해도 이 요약은 안 움직인다 — 익스포트 요약과 임포트는 별개 축이다.

## 실패한 접근

- **`use` 직접 임포트만으로 범위를 잠근 것** — Rust 2018+ 문법과 어긋나 모집단이 0 이다.
  `use` 717 줄 중 `crate::`/`super::`/`self::` 273 · `std::` 180 · 워크스페이스 다른
  크레이트 82(전부 `pub use` 경유) · `pub use` 선언 51 · 외부 크레이트 131.
- **`RefResolution::OutsideFile` 을 착지점으로 지목한 것** — 임포트한 이름은 그 갈래에
  안 온다. 추출기가 `use` 를 모듈 스코프 바인딩으로 선언해(`rust_scopes.rs:258`)
  `Bound{ NotASymbol }` → `counts.locals` 로 간다.

## 사고 기록

**하위 에이전트가 원본 `schema/graph.toml` 에서 216 줄을 지웠다**(`[node.Binding]` 절 전량).
`git checkout` 으로 복구했다. 격리 사본이 아니라 원본에서 파괴 실험을 한 것으로 보인다.
