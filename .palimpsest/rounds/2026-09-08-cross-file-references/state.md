# 상태 — 2026-09-08-cross-file-references

## 지금 단계
사전부검 1 라운드 대기 · 착수 기준선 대기

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
(아직 없다)
