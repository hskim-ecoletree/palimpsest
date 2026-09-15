# 교대 상태 — 첫 릴리스를 낸다

> 회차 `2026-09-14-first-release` · 착수 커밋 `ec92b89`

★ **새 컨텍스트가 받는 것은 「`intent.md` 전문 + 이 파일의 요약」이다.** 직전 산출물을
시드로 받지 마라.

## 지금 단계 — **독립 리뷰 R1 처리 → R2**

| 단계 | 상태 |
|---|---|
| 인터뷰 · 사전부검 · 조건 설계 평가 · 승인 | **닫혔다** — 각 1 라운드 · 승인 `approval.md` 「전부 승인」 |
| 구현 ㈎㈏㈐ · 병합 · CI 회귀 고침 | **닫혔다** — `1d67cb0` · `2de539e` · `c25eff1` · `06f6b96` |
| 릴리스 | **`v0.1.0`** 공개(`e80ca14`) · README 사실 줄 정정으로 **`v0.1.1`** 을 낸다 |
| 효과 D1 · 음성 대조 D1-a | **섰다** — `effect/d1-release/` · `effect/negative-start-binary.txt` |
| 처분 | R1 5 · R3 39 닫힘 · #160 · 남은 위험 #161~#166 |
| 결박 | **닫혔다** — 넷 · `e7423e2`(원장 판 이관 · 개정 4) |
| 판정 | 통과 16 · 미측정 1(F1 — R1 반증 · 문구 고친 push 에서 다시 잰다) |
| 독립 리뷰 | **R1 닫는 중** — 발견 6 · 소유자 승격 1(원장 규칙 · 「개정으로 받는다」) · 남은 상한 R2 |
| 종료 보고 | 초안 `report.md` · R2 뒤 마감 |

## 착수 바이너리

`target/release/pal` = `pal 0.0.0+ec92b899478d`. 세션이 바뀌면 `git worktree add --detach <스크래치>/wt-start ec92b89`
뒤 `cargo build --release -p pal-cli` 로 다시 뜬다. RED 재현 스크래치 클론은 세션 스크래치 `red/` 에 있었다(커밋 안 함).

## 실패한 접근

- `doctor --full` 을 `timeout` 으로 감싸 돌렸다 — macOS 에 `timeout` 이 없어 rc 127. 감싸지 않고 돌린다.
- **병합 검증을 `cargo xtask test`·`check` 로만 했다** — CI 의 `pal doctor full 구조 판정`(`cargo run -q -p pal-cli -- doctor --full --json | node scripts/check-round-doctor.mjs`)은 그 둘에 없다. ㈏ 가 `[node.Symbol]` 에 필수 `decor` 를 더하고 `doctor` 뷰가 안 실어 CI 런 `34854953489`(`2de539e`)의 ubuntu·macos 가 빨갰다. 고침 `06f6b96`. **push 전에 이 걸음을 `palimpsest` 이름 클론에서 함께 돌린다.**
- **공유 걸음을 결박 내보내기 없이 쟀다** — 승인은 `intent.redb`(무시되는 파생물)에만 쓰고 `bindings.jsonl` 을 안 만든다. 그래서 팀원 클론이 착수·새 바이너리 둘 다 `(0)` 이었고 음성 대조가 무효였다. 걸음에 `pal intent export --out .palimpsest/intent/bindings.jsonl` 을 넣었다. README 의 「결박은 `bindings.jsonl` 에 덧붙여집니다」는 사실이 아니었다 — 고친다.

## 구현 입력

코드 지도 요약은 세션 스크래치 `code-map.md` 에 있었다(커밋 안 함). 핵심 — 저장소 식별자가 `SymbolId` 해시 성분이다(`crates/pal-core/src/coord.rs:106-130`).
