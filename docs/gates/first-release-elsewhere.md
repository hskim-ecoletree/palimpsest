# 게이트 — 첫 릴리스를 남의 저장소에서 세운다

**회차** `2026-09-13-first-release-elsewhere` · **이슈** [#135](https://github.com/hskim-ecoletree/palimpsest/issues/135)(TS 몫)
**착수** `6ee9eb3` · **판정일** 2026-09-14

> 잠긴 의도 [`intent.md`](../../.palimpsest/rounds/2026-09-13-first-release-elsewhere/intent.md) ·
> 종료 보고 [`report.md`](../../.palimpsest/rounds/2026-09-13-first-release-elsewhere/report.md) ·
> 승인 [`approval.md`](../../.palimpsest/rounds/2026-09-13-first-release-elsewhere/approval.md)

---

## 합격선

측정 전에 등록했다 — 잠긴 의도의 완수 조건 **29 개**(`A1`~`G2`). 조건 설계 평가 두 라운드가 문면을 고쳤고
소유자가 *「전부 승인」* 으로 잠갔다(`approval.md`).

### 갈래 — 29 를 둘로 다 가른다

**결정론적(28)** — `A1` `A1-a` `A2` `A2-a` `A3` `A3-a` `A4` `A5` `A6` `A7` `B1` `B1-a` `B2` `B3` `B4`
`C1` `C1-a` `D1` `D1-a` `D2` `E1` `E2` `E4` `F1` `F2` `F4` `G1` `G2`
(시험 · 기존 검사 · TypeScript 컴파일러 대조 · `git`·`gh` 조회로 답이 난다)

**결정론적이 아님(1)** — `E3` (그 화면이 변경에서 무엇을 바꿨나 — 정반합 판 `e3-effect`)

### RED 관측

| 조건 | 무엇을 관측했나 | 기록 |
|---|---|---|
| `A1` | 착수 바이너리로 TS 픽스처의 이름 임포트 10 개가 **0/10** 엣지 · 전부 `outside_repo` | `oracle/A1-red.txt` |
| `A5` | 착수 기준선 ditto 복제본 `0/11010` · 전부 `outside_repo` | `baseline/04-touch-resolveRepoRootForCreate.txt` |
| `B2` | 착수 바이너리가 승인 명령을 clap 에서 거부 · rc 2 | `oracle/B2-red.txt` |
| `D1` | 같은 모집단을 착수 바이너리로 돌리면 금지 패턴 **64** | `oracle/D1-red.txt` |
| `G1` | 구현 뒤 첫 측정이 더한 줄 **4** 를 잡았다(시험 고정물의 별칭 꼴 · 이름) | `oracle/G1-first-red.txt` |
| `F4` | 착수 시점에 `report.md` 도 게이트 문서를 고친 커밋도 없다 | `oracle/F4-g5.txt` |

### 음성 대조

`A1-a` · `A2-a` · `A3-a` · `B1-a` · `C1-a` · `D1-a` 가 각 주 조건의 음성 대조로 **조건으로 등록됐다.**
그 밖에 조건 문면이 스스로 음성 대조를 지는 것 — `A6`(한쪽 `--at` 을 앞 커밋으로) · `E1`(`--self-test` 셋) ·
`G1`(`+ // ditto` 한 줄) · `D2`(옛 단언을 지운 커밋의 새 단언 셈).

### 차선책 — 등록된 넷

`intent.md ## 차선책` 이 진다. 쓴 것은 판정 절에 적는다.

## 판정

| 판정 | 조건 |
|---|---|
| 통과 | A1 A1-a A2 A2-a A3 A3-a A4 A5 A6 A7 B1 B1-a B2 B3 B4 C1 C1-a D1 D1-a D2 E1 E2 E4 F1 F2 F4 G1 |
| 반증 | G2 |
| 대조불가 | E3 |
| 미측정 | — |

**검산** — 통과 27 · 반증 1 · 대조불가 1 · 미측정 0 = 29

### 근거 표

| 조건 | 무엇이 판정했나 |
|---|---|
| `A1` | 시험 `ts_cross_file::a1_지정자_여섯_꼴이_다른_파일의_심볼로_가는_엣지가_된다` · RED `oracle/A1-red.txt`(착수 바이너리 0/10) |
| `A1-a` | 같은 시험의 음성 대조 단언 — `outside_repo` · `bare_specifier` · `no_target_file` 셋이 갈린다 |
| `A2` | 시험 `ts_cross_file::a2_1` ~ `a2_6` |
| `A2-a` | 시험 `ts_cross_file::a2a_패키지_extends_에서만_오는_별칭은_저장소_밖이_아니라_못_읽음이다` |
| `A3` | `oracle/A3-fixture.txt` — 픽스처마다 불일치 0 · 합산 표본 0 인 가지 없음 · 해소 모드는 `ts.getModeForUsageLocation` |
| `A3-a` | 같은 산출의 `--corrupt` 절 — 불일치 1 |
| `A4` | 시험 `ts_cross_file::a4_재수출_배럴로_펴지는_임포트는_재수출을_지나는_이름이다` |
| `A5` | `oracle/A5-ditto.txt`(커밋된 바이너리 `ba0a983` · 새 복제본) — ⑴ 선 것 > 0 · ⑵ 0(꼴 셈도 0: `oracle/A5-2-shape.txt`) · ⑶ 불일치 0 · ⑷ 2915 중 빠진 것 0. 착수 규칙에서는 59 였고 판 `p1-binary-nul` 의 확대(`intent.md ## 개정` · `## 승격` 1~3)로 **제품을 고쳐** 닫았다 — `oracle/p1-X/`(데워진 캐시 · 음성 대조 · 골든 · f04 줄 판정) |
| `A6` | ⑴ `oracle/A6-xtask-test.txt` — `cargo xtask test` 통과 1124 · 실패 0(HEAD `5b4a37f`) ⑵ `oracle/A6-rust.txt` — 착수 바이너리와 `pal 0.0.0+5b4a37f08e8b` 의 파일 간 엣지 집합 1319 = 1319 · 한쪽에만 0 · 음성 대조(앞 커밋 `1276b8f`) 1294 로 다르다. 그 뒤 크레이트 변경(`c9e31a3`)은 시험 두 개뿐이고 마지막 커밋의 시험 전량은 `F2` 의 CI 가 진다 |
| `A7` | 시험 `cross_file_references` 의 「호출자 수는 하한입니다」 단언 |
| `B1` · `B1-a` · `B2` · `B3` | 시험 `pending_and_pick::b1` · `b2` · `b3`(B1-a 는 b1 안의 `lonely` 음성 대조) · RED `oracle/B2-red.txt` |
| `B4` | `oracle/B4-timing.txt`(규칙 변경 뒤 바이너리) — 표본 ①·② p95 2.2 ms < 500 ms · 표본 ② 대기 구역 50/50 · 후보 화면 0 |
| `C1` · `C1-a` | 시험 `pending_and_pick::c1` · `c1a` |
| `D1` · `D1-a` | 시험 `user_vocabulary::d1_장면_명령의_사람_화면에_작업_기록_어휘가_없다` · `d1a_*` 셋 · RED `oracle/D1-red.txt` |
| `D2` | 시험 `cross_file_references` 두 문자열 단언 · 한 커밋 `oracle/D2-commit.txt`(`9db40b7`) |
| `E1` | `oracle/E1-order.txt` — 여섯 산출 진조상 순서 · 앵커 · 음성 대조 셋 `oracle/E1-self-test.txt`(`--self-test` · ①②③ 「걸렸다」) |
| `E2` | `oracle/E2-scene.txt` — ⑴ ⑵ ⑶ ⑷ 전부 성립한다. ⑵ 는 **⟨개정 p2⟩ 뒤** 증인 심볼 규칙(`oracle/callers_rule.py` · `oracle/callers-rule-table.txt`)으로 증인 `05 codexHostAdapter` 가 성립한다 · 음성 대조 `oracle/E2-negative-count.txt`(수를 바꿔 넣으면 증인 0). 옛 문면대로는 반증이었다 — `oracle/E2-literal-touch.txt` |
| `E3` | **대조 불가** — 판 `e3-effect`(`dialectic/r2-raw.md`) 상한 뒤 소유자 결정(`intent.md ## 승격` 4~6) |
| `E4` | `oracle/E4-breakage.txt` — **⟨개정 p2⟩ 뒤** 같은 읽기 규칙으로 뽑은 C · A 워크트리(`898a479` · 변경 0)에서 선언 안 0. 음성 대조 셋: C 를 비우면 선언 안 1(`oracle/E4-negative-c-empty.txt`) · 수를 바꾸거나 목록 줄을 지우면 대조 불가(`oracle/E4-negative-count.txt` · `oracle/E4-negative-drop-line.txt`). 옛 문면대로는 반증이었다 — `oracle/E4-literal-touch-c.txt` |
| `F1` | `oracle/F1-comment.txt` — `#135` 코멘트가 `A5` 수(`7199/11099` · `oracle/A5-touch-final.txt` — `pal 0.0.0+5b4a37f08e8b` · 새 복제본)와 게이트 경로를 싣는다. 첫 판은 NUL 규칙 전 바이너리의 `7116/11010` 과 착수 칸의 재현율 59 를 섞어 적어 독립 리뷰 R1 뒤 편집했다 |
| `F2` | `oracle/F2-ci-067c7c4.txt` — main 에 올린 마지막 push 의 SHA `067c7c4` · 런 `34803197814` `success`. 앞의 두 push `701acb5` · `4398134` 는 `windows-latest` 빨강(`oracle/F2-ci-701acb5.txt` · `oracle/F2-ci-4398134.txt`) · 고친 커밋은 PR #157 에서 먼저 쟀다 · 소유자 답 `intent.md ## 승격` 9 · 10 · 11. 이 전사 커밋은 push 하지 않았다 |
| `F4` | `oracle/F4-g5.txt` — `total_count 0` → 종료 보고에 `G5` · `Actions` 줄 |
| `G1` | `oracle/G1-coupling.txt` — `6ee9eb3..c9e31a3` 더한 줄 0 · 음성 대조 1(첫 측정 `oracle/G1-first-red.txt` 는 4) |
| `G2` | **반증** — `oracle/G2-violation.txt`. `scripts/f06-verify.py` 가 원본 ditto 에 캐시 자리를 안 주고 `pal` 을 붙여, `EXTRACTOR_REV` 승급 뒤 재실행에서 원본 `.palimpsest/cache` 에 항목 2,451 개가 더해졌다. 소유자 결정으로 더해진 파일만 지웠고(`oracle/G2-deleted-cache-files.txt`) 디렉터리 시각은 되돌리지 않았다 — 목록 해시가 착수(`oracle/G2-ditto-origin-before.txt`)와 뒤(`oracle/G2-ditto-origin-after.txt`)에서 다르다. HEAD · porcelain · `node_modules` 는 같다 |

### 차선책 — 등록된 넷 중 쓴 것

없다. `extends` 사슬 · CI 가 뜨지 않음 · 표본 400 · ADR 후보 대체 모두 발동하지 않았다.

### 개정 둘 — 무엇을 왜 바꿨나

`intent.md ## 개정` 이 진다. 줄은 둘이다 — `p1` **확대**와 `p2` **정정**. `A5` ⑷ 는 **글자를 안 바꾸고 제품을 고쳤다**(`p1`). `E2` ⑵ · `E4` 는 호출자 집합을 「`touch` 가 **센** 호출자」로 **정정**했다 — ⚠ 그 읽기 규칙은 두 결과(질의 출력으로 잰 통과 · 문면대로 잰 반증)를 **다 본 뒤** 썼다. 사전 등록이 아니다.

## 효과

남의 저장소(ditto 복제본)에서 과제 하나(`codex.ts` 제거)를 `pal touch` 를 보며 했다. 붙이는 산출은 테스트가 아니다 —
`effect/03-touch/05-codexHostAdapter.txt` · `effect/04-touch-after-approve.txt`.

- 화면이 **결정 문서 조각을 승인 대기로 보였고**, 화면이 안내한 명령으로 승인한 뒤 `■ 이 좌표에 걸린 것 (1)` 이 ADR-0003 결정 본문을 실었다.
  그 본문을 읽고 봉인 계획에 없던 걸음 둘(`shared.ts` wrapper 제거 · ADR-0003 상태 줄)이 더해졌다.
- 그러나 **그 둘을 화면의 효과로 세지 않는다** — 봉인 계획을 쓴 자가 잠그기 전 실험에서 같은 본문을 이미 봤고,
  봉인 §4 는 그 노출을 적지 않았다(**봉인 기준선의 결함** · 소유자 결정). 이름별 grep 도 화면 없이 같은 본문에 닿는다.
- 화면 몫으로 「걸음을 없앴다」는 **0** 이다.
- ★ **원문의 「깨질 곳을 받는다」는 `touch` 화면에서 호출자 자리로는 서지 않았다.** `touch` 는 호출자의 **수**와 하한 단서만 싣고,
  효과 실행에서 호출자 파일은 안내 없는 `pal query symbol.callers` 가 댔다(`effect/04-readnote.md`). 소유자가 「수가 의도」라고 답했고
  자리를 싣는 것은 `#156` 으로 뗐다.

### 첫 릴리스가 섰나

**섰다 — 한 갈래를 뺀 채로.** 원문의 줄 「설치 → 코드를 만지려는 순간 → 걸린 결정과 깨질 곳을 받는다」 가운데
설치 · `touch` 를 부르는 순간 · **걸린 결정**(승인 대기 → 승인 → `■ 이 좌표에 걸린 것` 이 ADR 본문과 판정을 싣는다) ·
파일 경계를 넘는 호출자 **수** · 동명 지목 · 회차 어휘 0 은 남의 저장소 복제본에서 섰다(`E1` · `E2`).
**깨질 곳의 자리는 화면에서 서지 않았다** — 위 효과의 마지막 줄 · `#156`. 화면 몫의 효과는 대조 불가다(`E3`).
버전 태그와 릴리스 바이너리는 원문이 안 요구해 만들지 않았다.

### 결박

`crates/pal-core/src/ts_module.rs` `TsProject` · `crates/pal-extract/src/lib.rs` `EXTRACTOR_REV` · `crates/pal-cli/src/pending.rs` `열쇠` — 커밋 `66108c5`. 그래프 — `pal doctor` 불변식 위반 0.

⚠ **정정** — `TsProject` 결박의 메모(`bindings.jsonl` `af9b0a700c051f7c`)는 「선 것 7116」을 적었다. 그것은 NUL 규칙 전 바이너리의 값이고, 최종 바이너리에서는 `7199/11099` 다(`oracle/A5-touch-final.txt`). 결박 원장은 덧붙이기만 하는 원장이라 그 줄을 고치지 않고 여기 적는다.

### ditto 기호 골든이 움직였다

`corpus/golden/ditto.symbols.tsv` 가 이 회차에서 **새 78 행**을 받아 **4,656 행**이 됐다(분류 규칙 변경 · `oracle/p1-X/X5-*`). 이후 음성 대조는 4,656 을 댄다.

## 범위 밖

`intent.md ## 범위 밖` 이 진다 — 여기 옮겨 적지 않는다(두 곳에 적으면 갈린다).
