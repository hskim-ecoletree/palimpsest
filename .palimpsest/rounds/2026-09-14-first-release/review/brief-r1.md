# 독립 리뷰 R1 브리프 — 회차 `2026-09-14-first-release`

> 상한 **2 라운드**. 이 파일과 아래 산출물만 받는다. 대화 기록 · 메인의 판단 과정 · 앞 라운드 서사는 없다.

## 잠긴 의도

`.palimpsest/rounds/2026-09-14-first-release/intent.md` — `## 원문` 과 `## 개정` 이 원 의도다. 소유자 원문의 출처는
`docs/instructions/2026-09-14-owner-direction.md`(`U25` · `U26`). 승인은 같은 디렉터리의 `approval.md`.

## 산출물 — 좌표

| 무엇 | 어디 |
|---|---|
| 판정의 정본 | `docs/gates/first-release.md`(`## 판정` 표준 표 · 근거 표 · `## 효과`) |
| 종료 보고 | `.palimpsest/rounds/2026-09-14-first-release/report.md` |
| 커밋 범위 | `ec92b89..HEAD`(`main`) |
| 릴리스 | `v0.1.0` — `gh release view v0.1.0 -R hskim-ecoletree/palimpsest` · 태그 커밋 `e80ca14` |
| 착수 관측 · 오라클 · 효과 산출 | 같은 회차 디렉터리의 `baseline/` · `oracle/` · `effect/` |
| 이슈 처분 | `triage.md`(열린 이슈 64 의 R1 · R2 · R3 분류) · `oracle/E1-issues.txt` · 새 이슈 #160 ~ #166 |
| 발견 원장 | `findings.jsonl`(사전부검 R1 · 조건 설계 평가 R1) |
| 결박 | `.palimpsest/intent/bindings.jsonl`(이 회차의 결박 커밋 메시지를 함께 본다) |
| README 걸음과 효과 | 루트 `README.md` · `effect/d1-release/` · `effect/negative-start-binary.txt` |

## 두 축을 따로

- **합격선 축** — 완수 조건 17 을 재고 통과 · 반증 · 대조불가 · **미측정** 중 하나로 판정한다. 게이트 판정과 갈리면 근거를 대라.
- **의도 축** — 원문에 비춰 빠진 것 · 요구되지 않은 것 · 있는데 틀린 것.

발견마다 **모집단**(원의도 · 저장소 · 자기장치 · 회차기록 · 규약) · **유효성**(참 · 추정 · 거짓) · **해악도**(금지역 · 실패 · 거짓신호 · 미관)를 붙인다.
`.claude/pal/policy.toml` 이 없어 금지역은 규약 기본 다섯이다. 「없음」을 압박하지 않는다 — 정직하게만 낸다. 끝은 메인이 정한다.

## 재는 법 — 걸리는 것

- **`cargo xtask check` 를 메인 체크아웃에서 돌리지 마라.** 저장소 안 `.claude/worktrees/` 에 지난 구현 워크트리 사본이 남아 있어 문서 검사가
  사본까지 훑는다(죽은 링크 수백). `git clone --no-hardlinks` 사본에서 **그 사본에서 빌드해** 돌린다 — xtask 는 뿌리를 빌드 시점에 박는다.
- CI 에만 있는 걸음: `cargo run -q -p pal-cli -- doctor --full --json | node scripts/check-round-doctor.mjs` — **디렉터리 이름이 `palimpsest` 인 클론에서.**
- **`~/dev/projects/ditto` 원본에 아무것도 쓰지 마라**(캐시 포함). 실행이 필요하면 `git clone --no-hardlinks` 사본에서만.
- 이 저장소의 `.palimpsest/intent/` · `.palimpsest/*.redb` 를 쓰지 마라. 커밋 · push · 이슈 편집을 하지 마라.
- 원 반환문은 `review/r1-raw.md` 에 **네가 직접** 쓴다(에이전트 정의의 반환 형식).
