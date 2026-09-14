# 게이트 — 첫 릴리스를 낸다

**회차** `2026-09-14-first-release` · **이슈** #77 · #127 · #139 · #156 · #159 (R1) · 분할 [#160](https://github.com/hskim-ecoletree/palimpsest/issues/160)
**착수** `ec92b89` · **판정일** 2026-09-15 · **릴리스** [`v0.1.0`](https://github.com/hskim-ecoletree/palimpsest/releases/tag/v0.1.0)(`e80ca14`)

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

| 판정 | 조건 |
|---|---|
| 통과 | A1 A1-a A2 A2-a A2-b A3 A3-a B1 B1-a B2 C1 C2 D1 D1-a E1 E2 |
| 반증 | — |
| 대조불가 | — |
| 미측정 | F1 |

**검산** — 통과 16 · 반증 0 · 대조불가 0 · 미측정 1 = 17

### 근거 표

| 조건 | 무엇이 판정했나 |
|---|---|
| `A1` | 시험 `crates/pal-cli/tests/coverage_whole_graph.rs` `전_그래프_명령의_범위가_graph_dump_와_같은_값이다` — `doctor --json` · `export` 의 `coverage` 가 같은 인덱스의 `graph.dump` 와 같다 · `grep` 두 문자열 0 · RED `oracle/A1-red.txt`(`2 · l1` ↔ `0 · l0`) |
| `A1-a` | 같은 시험이 `unresolved ≥ 1` · `lowest_grade ≠ l0` 을 먼저 단언 · `범위는_질의마다_다른_값이다` 무수정 통과 |
| `A2` | 시험 `crates/pal-cli/tests/rust_decor_stale.rs` `a2_1`~`a2_5` — 다섯 갈래 각각 `stale` · RED `oracle/A2-red.txt`(다섯 다 `fresh`) |
| `A2-a` | ⑴ 시험 `a2a_1_속성_안의_주석과_공백만_바꾸면_fresh` ⑵ `oracle/A2a-ts-digest.txt` — TS 픽스처 요약값 다른 것 0/14(ditto 0/4,656) |
| `A2-b` | `oracle/A2b-verdicts.txt` — 42 = 42 · 모집단 20 · 판정 이동 0 · `stale`→`fresh` 0. ⚠ **이동 0 은 ㉡ 의 구조에서 거의 자동으로 나온다** — 옛 결박 42 는 `decor` 축이 「기록 안 됨」이라 그 축을 비교하지 않고 화면이 「속성·수신자 변경은 감시 안 함」을 드러낸다. 조건이 금한 「코드 변화 없이 뒤집힘」은 없지만, 옛 결박이 속성 변경을 잡게 된 것도 아니다 |
| `A3` | 시험 `cross_file_references.rs` `a3_크레이트_경계를_넘는_use_가_사적_mod_선언에_엣지를_안_잇는다` — 엣지 0 · 못 푼 참조 `no_symbol_at_crate_root` · RED `oracle/A3-red.txt`(엣지 1) |
| `A3-a` | ⑴ 시험 `a3a_크레이트_안에서_traverse_모듈을_부르는_엣지는_남는다` ⑵ `oracle/A3-repo-diff.txt` — 파일 간 엣지 1338 → 1336 · 사라진 것 = 고정 목록 둘 · 새로 생긴 것 0 |
| `B1` | 시험 `crates/pal-cli/tests/repo_identity.rs` `b1_…` · RED `oracle/B1-red.txt` |
| `B1-a` | 같은 파일 `b1a_1`~`b1a_4` (⑵ 는 착수 코드에서도 통과했다 — install 이 매니페스트를 아예 안 건드렸다) |
| `B2` | 시험 `crates/pal-cli/tests/caller_places.rs` `b2_…` — 손으로 박은 기대 자리 집합 · 동명 `--pick` · 안내 명령을 그대로 실행 · RED `oracle/B2-red.txt` |
| `C1` | 루트 `README.md` · `LICENSE-MIT` · `LICENSE-APACHE`(커밋 `4ac7c32` · `7894f54`) · `oracle/C1-readme-vs-d1.txt`(양방향 빠진 것 0 · 0 · 음성 대조 D1→README 1) · `oracle/C1-archive.txt`(자산 넷 전부 세 파일 · `SHA256SUMS` 대조 rc 0) |
| `C2` | `oracle/C2-release.txt` — ⑴ 수동 런 34859015942 success(첫 런 34856889486 은 권한 403 · 고침 `f16ab51`) ⑵ 태그 런 34861114858 success(받은 바이너리의 `touch` 걸음 포함) ⑶ 자산 넷 + `SHA256SUMS` · 받은 바이너리 `pal 0.1.0+e80ca144101d`(`effect/d1-release-download.txt`) |
| `D1` | `effect/d1-release/` — 릴리스 자산으로 README 걸음을 글자 그대로(어긋남 없음 · 전부 rc 0): 첫 클론 `■ 이 좌표에 걸린 것 (1)` · 호출자 자리 2곳 · 다른 이름 클론 들인 뒤 `(1)` |
| `D1-a` | ⑴ `effect/negative-start-binary.txt` — 착수 바이너리 · 결박 1 을 들이고도 `(0)` · README `git add` 한 줄 어긋남(개정 1) ⑵ `effect/origin-before.txt` = `effect/origin-after.txt`(HEAD · porcelain · `.palimpsest/` 목록 해시) |
| `E1` | `oracle/E1-issues.txt` — R1 5 닫힘(completed)·코멘트 · R2 20 열림 · R3 39 닫힘(not planned)·코멘트 · #127 코멘트가 `baseline/06-doctor-full.txt` · ADR-0007 을 싣는다 |
| `E2` | `oracle/E2-docs.txt` — #160 본문에 대상 · 세 칸 · 판정 문장(9/9) · `frontier.sh` 첫 줄 `#160 ← 순서표의 1 번` · 최단 경로 §4.3 의 끝의 정의와 언어 확장 순서 |
| `F1` | **미측정** — 코드를 바꾼 마지막 커밋(로컬 클론 `--no-hardlinks`)을 담은 push 의 CI 대기. 앞의 둘은 흔들림으로 빨갰다: 런 34859001054 ubuntu(어휘 시험 거짓 양성 · 개정 2) · 34861122337 macos(하드링크 클론 · 개정 3) |

### 차선책 — 등록된 셋 중 쓴 것

없다. #139 는 본선(스키마 무변경)으로 섰고, #77 은 계획 2 의 갈래 ㉡ 로 섰으며, 릴리스 워크플로의 타깃 넷이 전부 초록이다.

### 개정 셋

`intent.md ## 개정` 이 진다 — ① `D1-a` ⑴ 절차(착수 바이너리에서 README `git add` 줄) ② 앞 회차 장치 `user_vocabulary` 「기능 절」 패턴 ③ 시험 로컬 클론 `--no-hardlinks`. 셋 다 **정정**이고 재는 대상은 줄지 않았다.

## 효과

남의 저장소(ditto 새 복제본)에서 **릴리스 `v0.1.0` 자산**으로 루트 README 의 걸음을 글자 그대로 밟았다. 붙이는 산출은 시험이 아니다 —
`.palimpsest/rounds/2026-09-14-first-release/effect/d1-release/`.

- 설치 → 문서를 코드에 대 봄 → `pal touch codexHostAdapter` 가 **승인 대기와 그대로 칠 승인 줄**을 실었고, 승인 뒤 `■ 이 좌표에 걸린 것 (1)` 과
  **호출자 자리 2곳**(`src/cli/commands/setup.ts:258` · `src/core/setup.ts:291`)이 나왔다.
- 결박을 내보내 매니페스트와 함께 커밋하고 **다른 디렉터리 이름으로 클론한 팀원**이 들인 뒤 같은 심볼에 `(1)` 을 받았다.
  같은 걸음을 착수 바이너리로 밟으면 결박이 들어가고도 `(0)` 이었다 — 착수 때 팀원에게 나가던 거짓 0 이 이 릴리스에서 닫혔다.
- ⚠ **이 걸음이 README 를 고쳤다.** 첫 리허설에서 README 의 「결박은 `bindings.jsonl` 에 덧붙여집니다」가 **사실이 아니었다**(승인은 무시되는
  `intent.redb` 에만 쌓인다) — 「팀과 나누기」 절을 실제 걸음으로 다시 썼다(`7894f54`). 내보내기는 상위 디렉터리를 만들지 않아 `mkdir -p` 를 앞에 둔다.
- ⚠ **효과의 판정은 이 게이트가 아니다.** 「화면을 보고 작업이 달라졌나」는 사용 기록 이슈 [#160](https://github.com/hskim-ecoletree/palimpsest/issues/160) 이 진다.

## 범위 밖

- **언어 확장** — Kotlin · Java · JavaScript · Python. 소유자 `U25` 가 첫 릴리스 뒤 품질 단계로 뒀다(최단 경로 §4.3).
- **R2 20 건** — 열린 채 둔다(`U26`).
- **사용 기록의 판정** — 이슈 [#160](https://github.com/hskim-ecoletree/palimpsest/issues/160) 이 진다(`U26` 분할).
- **서명·공증 · 패키지 관리자 배포** — `SHA256SUMS` 만 낸다.
- **`coverage_of` 의 질의별 범위 설계** — `[f05.3.pass]` ⑤ 가 정했고 결함이 아니다.
