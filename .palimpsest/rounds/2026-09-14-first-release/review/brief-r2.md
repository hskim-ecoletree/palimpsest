# 독립 리뷰 R2 브리프 — 회차 `2026-09-14-first-release`

> 상한 2 의 **마지막 라운드**. 이 파일과 아래 산출물만 받는다. 대화 기록 · 메인의 판단 과정 · 앞 라운드의 판정 내용은 없다.

## 잠긴 의도

`.palimpsest/rounds/2026-09-14-first-release/intent.md` — `## 원문` · `## 개정` · `## 승격` 이 원 의도다. 원문의 출처는
`docs/instructions/2026-09-14-owner-direction.md`(`U25` · `U26`). 승인은 같은 디렉터리의 `approval.md`.

## 산출물 — 좌표

| 무엇 | 어디 |
|---|---|
| 판정의 정본 | `docs/gates/first-release.md` |
| 종료 보고 | `.palimpsest/rounds/2026-09-14-first-release/report.md` |
| 커밋 범위 | `ec92b89..HEAD`(`main`) — ★ **특히 `75e9ca4..HEAD`**(앞 라운드 뒤의 처리 방침 · 레코드 · 교정 릴리스) |
| 릴리스 | `v0.1.0`(`e80ca14`) · 교정 `v0.1.1` — `gh release view <태그> -R hskim-ecoletree/palimpsest` |
| 착수 관측 · 오라클 · 효과 산출 | 같은 회차 디렉터리의 `baseline/` · `oracle/` · `effect/` |
| 이슈 처분 | `triage.md` · `oracle/E1-issues.txt` · 새 이슈 #160 ~ #166 |
| 발견 원장 | `findings.jsonl` |
| 결박 | `.palimpsest/intent/bindings.jsonl`(결박 커밋 메시지를 함께 본다) |
| README 걸음과 효과 | 루트 `README.md` · `effect/d1-release/` · `effect/negative-start-binary.txt` |

## 두 축을 따로

- **합격선 축** — 완수 조건 17 을 재고 통과 · 반증 · 대조불가 · **미측정** 중 하나로 판정한다. 게이트 판정과 갈리면 근거를 대라.
- **의도 축** — 원문에 비춰 빠진 것 · 요구되지 않은 것 · 있는데 틀린 것. ★ **커밋된 산출물(README · 게이트 · 보고 · 릴리스 노트 · 아카이브)이 사실이 아닌 것을 사실로 말하는가**를 겨눈다.

발견마다 **모집단** · **유효성** · **해악도**를 붙인다. 금지역은 규약 기본 다섯이다. 「없음」을 압박하지 않는다. 끝은 메인이 정한다.

## 재는 법 — 걸리는 것

- `cargo xtask check` 는 `git clone --no-hardlinks` 사본에서 **그 사본에서 빌드해** 돌린다(저장소 안 `.claude/worktrees/` 사본이 문서 검사에 잡음을 낸다).
- CI 에만 있는 걸음: `cargo run -q -p pal-cli -- doctor --full --json | node scripts/check-round-doctor.mjs` — 디렉터리 이름이 `palimpsest` 인 클론에서.
- `~/dev/projects/ditto` 원본에 아무것도 쓰지 마라. 실행은 `git clone --no-hardlinks` 사본에서만.
- 이 저장소의 `.palimpsest/intent/` · `.palimpsest/*.redb` 를 쓰지 마라. 커밋 · push · 이슈 편집 · 릴리스 편집을 하지 마라.
- 원 반환문은 `review/r2-raw.md` 에 **네가 직접** 쓴다.
