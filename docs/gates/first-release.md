# 게이트 — 첫 릴리스를 낸다

**회차** `2026-09-14-first-release` · **이슈** #77 · #127 · #139 · #156 · #159 (R1) · 분할 [#160](https://github.com/hskim-ecoletree/palimpsest/issues/160)
**착수** `ec92b89` · **판정일** —

> 잠긴 의도 [`intent.md`](../../.palimpsest/rounds/2026-09-14-first-release/intent.md) ·
> 승인 [`approval.md`](../../.palimpsest/rounds/2026-09-14-first-release/approval.md) ·
> 소유자 지시 [`2026-09-14-owner-direction.md`](../instructions/2026-09-14-owner-direction.md) `U25`·`U26`

---

## 합격선

측정 전에 등록했다 — 잠긴 의도의 완수 조건 **17 개**(`A1`~`F1`). 사전부검 1 라운드와 조건 설계 평가 1 라운드가 문면을 고쳤고
소유자가 *「전부 승인」* 으로 잠갔다(`approval.md`).

### 갈래

**결정론적(17)** — `A1` `A1-a` `A2` `A2-a` `A2-b` `A3` `A3-a` `B1` `B1-a` `B2` `C1` `C2` `D1` `D1-a` `E1` `E2` `F1`
(통합 시험 · 일회성 오라클 · `gh` 조회 · CI 런 결론). **결정론적이 아님 0** — 정반합을 안 쓴다.

### RED 관측

| 조건 | 무엇을 관측했나 | 기록 |
|---|---|---|
| `A1` | `pal doctor --json` 의 `coverage` 가 `unresolved 0 · lowest_grade l0`, 같은 인덱스의 `graph.dump` 는 `13404 · l1` | `baseline/04-coverage-hardcoded.txt` · 픽스처 `oracle/A1-red.txt` |
| `A2` | `#[derive]` 만 바꿔 커밋해도 결박이 `fresh` · 다섯 갈래 전부 `fresh` 로 실패 | `baseline/03-derive-change.txt` · `oracle/A2-red.txt` |
| `A3` | 착수 해소기가 크레이트 밖 `use` 를 사적 `mod traverse` 선언에 잇는다(엣지 1) | `oracle/A3-red.txt` |
| `B1` | 같은 커밋 · 같은 의도인데 디렉터리 `pal/` 에서 `■ 이 좌표에 걸린 것 (0)` | `baseline/01-touch-dir-pal.txt` · `oracle/B1-red.txt` |
| `B2` | `touch` 가 `호출자 2 · 피호출자 1` 수만 싣고 자리 0 | `baseline/01-touch-dir-palimpsest.txt` · `oracle/B2-red.txt` |
| `C2` | 착수 때 릴리스는 `v0.0.0-f24.1` 하나 · 릴리스 워크플로가 태그로 돈 적 없음 | `intent.md ## 착수 시점 관측` |

### 음성 대조

조건으로 등록된 것 — `A1-a`(픽스처가 하드코딩과 바이트로 같을 수 없음을 먼저 단언) · `A2-a`(주석·공백만 바꾸면 `fresh` · TS 요약 불변) ·
`A2-b`(착수 결박 42 의 판정 이동이 전부 설명되는가 · 모집단 20 이상) · `A3-a`(크레이트 안 엣지는 남고 사라진 것이 정확히 둘) ·
`B1-a`(네 갈래) · `D1-a`(착수 바이너리로 같은 절차 → 팀원 `(0)` · ditto 원본 불변).

### 차선책 — 등록된 셋

`intent.md ## 차선책` 이 진다. 쓴 것은 판정 절에 적는다.

## 판정

⟨측정 뒤 채운다⟩

## 효과

⟨`D1` 산출 뒤 채운다⟩

## 범위 밖

- **언어 확장** — Kotlin · Java · JavaScript · Python. 소유자 `U25` 가 첫 릴리스 뒤 품질 단계로 뒀다(최단 경로 §4.3).
- **R2 20 건** — 열린 채 둔다(`U26`).
- **사용 기록의 판정** — 이슈 [#160](https://github.com/hskim-ecoletree/palimpsest/issues/160) 이 진다(`U26` 분할).
- **서명·공증 · 패키지 관리자 배포** — `SHA256SUMS` 만 낸다.
- **`coverage_of` 의 질의별 범위 설계** — `[f05.3.pass]` ⑤ 가 정했고 결함이 아니다.
