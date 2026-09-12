# `G` — 열린 발견 55 의 처분 실행 감사

> 회차 `2026-09-11-effect-confirmation` · 잰 날 2026-09-12 · 잰 범위 `8604d62..f75b4b0`
> ⚠ **판정은 안 한다. 원장을 안 고친다.** 이 문서는 「처분이 어디까지 실행됐나」의 실측이다.

## 판정 규칙 — 먼저 못 박는다

세 값을 가르는 자는 **「그 처분이 닿을 자리에 바이트가 실제로 섰나」**다.

| 값 | 뜻 |
|---|---|
| **실행됨** | 코드·게이트·`intent.md` `## 승격`·이슈 본문·관측 파일 중 **한 곳에 실물 바이트가 섰다.** 커밋이 있다 |
| **부분** | 정정 내용이 **정반합 합(合) 문서의 `## 초안을 어떻게 고치나`** 에만 있다. 그 문서는 커밋돼 있으나 **회차가 정한 정오표 자리**(게이트 `### 정반합 산출물 — 정오표와 성격`)에는 안 섰다 |
| **안 됨** | 어느 자리에도 없다. 원장 행 자신이 유일한 기록이다 |

★ **「부분」의 까닭** — `C3-a` 계열이 초안(`dialectic/*-thesis*.md`)의 바이트 보존을 요구하므로
정정은 **옆에 붙는 정오표**로만 설 수 있다. 합(合)의 `## 초안을 어떻게 고치나` 는 그 정정의
**내용**을 이미 커밋된 바이트로 진다 — 그러나 독자가 초안을 만나는 자리에 표시가 없다.
회차가 `AT5-14` 에 대해 실제로 한 것(게이트에 정오표 절을 세움)이 이 계열의 **선례**다.

## 표

| id | 실행 | 닫은커밋 (40자) | 처분자리 (저장소 뿌리 기준) | 남은 것 |
|---|---|---|---|---|
| `DL1-01` | **실행됨** | `06e8d63f85a962d5339615e546408408f2efcfaf` | `crates/pal-cli/src/ledger.rs` | 없다. 버킷 ① 의 술어가 `ordinal > 0` → 「그룹 크기 > 1」로 바뀌어 약한 것과 취약한 것이 더는 같은 화면이 아니다 ⟨칸 1 집행⟩ |
| `DL1-02` | **안 됨** | 못 찾음 | — | `2026-08-20-rust-extractor` 의 `44 건(1.6%)` ↔ 지금 `20/3306` 의 4 배 어긋남을 **아무도 조정해 적지 않았다.** 게이트 정오표나 `effect/79-delta.md` 에 「모집단·계수기가 달라 어긋남이 설명된다」 한 문단을 쓰면 닫힌다 |
| `DL1-03` | **안 됨** | 못 찾음 | — | `xtask/src/main.rs:6434` 의 `종료_커밋_시각` 에 **여전히 시험이 0 이다**. 시험 모듈(`:6450 mod 최근_끝난_시험`)은 `use super::최근에_끝난;` 하나뿐이다. `git` 실패 시 `None` → 전 회차 `i64::MAX` → 사전순 회귀를 거는 시험을 그 모듈에 더하거나, 못 건다는 사실을 게이트에 적으면 닫힌다 |
| `DL1-04` | **부분** | `af981d1f45615c1369a65e378f425412dc3b67ef` (합 보존) | `.palimpsest/rounds/2026-09-11-effect-confirmation/dialectic/1-synthesis.md` | 정정 내용은 `1-synthesis.md:55` ①에 있다(*"이 고침이 실제로 다른 회차를 집는다"* 는 거짓). `1-thesis.md:84` 옆에 정오표가 없다 — 게이트 정오표 절에 한 줄 |
| `DL1-06` | **실행됨** | `06e8d63f85a962d5339615e546408408f2efcfaf` | `crates/pal-cli/src/ledger.rs` | 없다. 「정확」 버킷 화면이 *"이름으로 유일하고"* → *"이름이 겹치지 않았고"* 로 바뀌었고 술어도 그룹 크기 기준이 됐다 |
| `DL1-10` | **실행됨** | `7abb85902e3faac6d8a63331d5da652c675594e0` | `.palimpsest/rounds/2026-09-11-effect-confirmation/observations/a5c-usage-census.md` | 없다. 93 줄 전수를 ⓐ20·ⓐ′3·ⓑ3·ⓒ33·ⓓ10·ⓕ24 로 갈랐다 ⟨합 확장 판단 「이번에 늘린다」 집행⟩ |
| `DL1-14` | **부분** | `e44a87efd204b13efec5fcb127837a05f53b5d27` (합 보존) | `.palimpsest/rounds/2026-09-11-effect-confirmation/dialectic/2-synthesis.md` | 합 `## 초안을 어떻게 고치나` 6 이 *"O8·O9·O10 의 수·인용을 고친다"* 로 받았다. 「42 줄 ↔ 41 줄」의 정오표가 독자 자리에 없다 |
| `DL1-15` | **부분** | `e44a87efd204b13efec5fcb127837a05f53b5d27` (합 보존) | `.palimpsest/rounds/2026-09-11-effect-confirmation/dialectic/2-synthesis.md` | 같음 — ⓐ 17↔20 · ⓒ 7↔8 의 정오표 |
| `DL1-16` | **부분** | `e44a87efd204b13efec5fcb127837a05f53b5d27` (합 보존) | `.palimpsest/rounds/2026-09-11-effect-confirmation/dialectic/2-synthesis.md` | 같음 — ⓓ 가 셋이 아니라는 정오표(`intent.md:182` 좌표) |
| `DL1-17` | **부분** | `af981d1f45615c1369a65e378f425412dc3b67ef` (합 보존) | `.palimpsest/rounds/2026-09-11-effect-confirmation/dialectic/3-synthesis.md` | 합 fix 2 가 좌표 셋(`check_ledger_pair.txt:2→:3` · `:9→:10` · `README.md:24→:23`)을 적고 ★ **정오표를 옆에 붙여라**까지 지시했는데 **그 정오표가 안 섰다.** 게이트 정오표 절에 세 좌표 |
| `DL1-18` | **실행됨** | `06e8d63f85a962d5339615e546408408f2efcfaf` | `.palimpsest/rounds/2026-09-11-effect-confirmation/intent.md` | 없다. 칸 2 가 「미측정으로 닫고 문면 셋을 승격」을 냈고 `#140` ①(배제 집합의 정의역)이 그 자리다 |
| `DL1-19` | **실행됨** | `06e8d63f85a962d5339615e546408408f2efcfaf` | `.palimpsest/rounds/2026-09-11-effect-confirmation/intent.md` | 없다. 칸 2 → `#140` ②. ⚠ ② 의 문면은 그 뒤 칸 7(`30488ae`)이 실측 표(양쪽 3·한쪽 2·맨「없다」2)로 다시 썼다 |
| `DL1-20` | **실행됨** | `06e8d63f85a962d5339615e546408408f2efcfaf` | `.palimpsest/rounds/2026-09-11-effect-confirmation/intent.md` | 없다. 칸 2 → `#140` ③(사실 자르기의 주체) |
| `DL1-21` | **실행됨** | `06e8d63f85a962d5339615e546408408f2efcfaf` | `.palimpsest/rounds/2026-09-11-effect-confirmation/intent.md` | 없다. 칸 2 → `#140` ①(§5.8 의 전수가 시간상 불가능) |
| `DL1-22` | **부분** | `af981d1f45615c1369a65e378f425412dc3b67ef` (합 보존) | `.palimpsest/rounds/2026-09-11-effect-confirmation/dialectic/3-synthesis.md` | 합 fix 5 가 *"셈 표에서 수를 빼고 셈 규칙이 봉인 안에 없다는 사실을 판정의 일부로 올린다"* 로 받았다. `#140` 세 문면 **어디에도 없다** — 게이트 정오표나 `#140` 에 네 번째 항 |
| `DL1-23` | **부분** | `af981d1f45615c1369a65e378f425412dc3b67ef` (합 보존) | `.palimpsest/rounds/2026-09-11-effect-confirmation/dialectic/3-synthesis.md` | 합 fix 6 + 라운드 2 `DL2-15` 가 *"판정값으로는 닫히고 사실로는 산다"* 로 받았다. 그 「사실」이 `C4`·`D2` 의 입력이라 했는데 게이트 `## 효과` 에 안 실렸다 |
| `DL1-24` | **부분** | `af981d1f45615c1369a65e378f425412dc3b67ef` (합 보존) | `.palimpsest/rounds/2026-09-11-effect-confirmation/dialectic/3-synthesis.md` | 합 fix 7(*"`:91` 의 「음성 대조 자체는 실행됐고」에 R8 을 단다"*)만 있다. 되불러 주기가 바이트 동일을 안 세운다는 정오표 |
| `DL1-25` | **안 됨** | 못 찾음 | — | ★ 합의 `## 초안을 어떻게 고치나` **일곱 항 어디에도 R9 가 없다.** 「판정자가 `touch/<이슈>.txt:N` 의 `N` 을 한 번도 검증 안 했다」가 `#140` 세 문면에도 안 들어갔다. `#140` 에 항을 더하거나 게이트 정오표에 한 줄 |
| `DL1-27` | **부분** | `af981d1f45615c1369a65e378f425412dc3b67ef` (합 보존) | `.palimpsest/rounds/2026-09-11-effect-confirmation/dialectic/4-synthesis.md` | 합 fix 2 가 §5 ★뼈대와 `:82`·`:92` 철회를 지시했다. **금지역**인데 게이트 정오표에 안 섰다 — `effect/126-readnote.md:14` 가 변경 커밋 전에 같은 `grep` 을 적었다는 사실을 게이트에 |
| `DL1-28` | **부분** | `af981d1f45615c1369a65e378f425412dc3b67ef` (합 보존) | `.palimpsest/rounds/2026-09-11-effect-confirmation/dialectic/4-synthesis.md` | 합 fix 3(readnote 셋을 증거 묶음에 편입). **금지역**. 게이트 정오표 |
| `DL1-29` | **실행됨** | `06e8d63f85a962d5339615e546408408f2efcfaf` | `.palimpsest/rounds/2026-09-11-effect-confirmation/intent.md` | 없다. 칸 3 이 `C1` 모집단을 「사전 등록 ↔ 실제 변경 **전부**」로 못 박았다 |
| `DL1-30` | **부분** | `af981d1f45615c1369a65e378f425412dc3b67ef` (합 보존) | `.palimpsest/rounds/2026-09-11-effect-confirmation/dialectic/4-synthesis.md` | 합 fix 4(분모 재구성 · 「0/11」 삭제). 정오표 없음 |
| `DL1-31` | **부분** | `af981d1f45615c1369a65e378f425412dc3b67ef` (합 보존) | `.palimpsest/rounds/2026-09-11-effect-confirmation/dialectic/4-synthesis.md` | 합 fix 5(「라벨된」 삭제). 정오표 없음 |
| `DL1-32` | **부분** | `af981d1f45615c1369a65e378f425412dc3b67ef` (합 보존) | `.palimpsest/rounds/2026-09-11-effect-confirmation/dialectic/4-synthesis.md` | 합 fix 6(*"나오는 곳은 넷뿐"* → 열 파일). 정오표 없음 |
| `DL1-33` | **부분** | `af981d1f45615c1369a65e378f425412dc3b67ef` (합 보존) | `.palimpsest/rounds/2026-09-11-effect-confirmation/dialectic/4-synthesis.md` | 합 fix 7(ⓒ 칸 *"어긋남 없음(내가 대조함)"* 을 내린다). 정오표 없음 |
| `DL1-34` | **실행됨** | `06e8d63f85a962d5339615e546408408f2efcfaf` | `.palimpsest/rounds/2026-09-11-effect-confirmation/intent.md` | 없다. 칸 5 가 「`E2` 뒤로 미뤄 판정한다」를 냈고 그 순서가 실제로 돌았다(게이트 `## 효과` → 판 4 R3) |
| `DL1-35` | **실행됨** | `06e8d63f85a962d5339615e546408408f2efcfaf` | `.palimpsest/rounds/2026-09-11-effect-confirmation/effect/79-delta.md` | 없다. `:24-31` 에 ⟨2026-09-12 정정⟩ 단락이 서서 `:23` 머리글의 자기모순을 걷고 모집단을 「전부」로 잇는다 |
| `DL1-36` | **부분** | `af981d1f45615c1369a65e378f425412dc3b67ef` (합 보존) | `.palimpsest/rounds/2026-09-11-effect-confirmation/dialectic/4-synthesis.md` | 합 fix 9(「11 줄」 → 「행 열하나 · 인용 범위 열네 줄」). 정오표 없음 |
| `MS-10` | **안 됨** | 못 찾음 | — | `crates/pal-git/src/lib.rs:193-197` 의 `matches` doc 주석이 **그대로다** — *"대장을 계산하는 동안 사용자가 파일을 고칠 수 있고"* 가 `--at` 없는 경로에서 원리상 발화 못 하는 보증이다. 그 주석에 한정(「`--at` 을 줬을 때만 잰다」)을 더하면 닫힌다 |
| `MS-11` | **안 됨** | 못 찾음 | — | `ledger.rs:714`(`(워킹트리)`)·`:729-737`(더러움)·`pal touch` 근거 상자의 세 「워킹트리」가 **아직 안 갈렸다**. 게이트 `## 효과` 나 `ledger.rs` 주석에 셋의 뜻을 가르는 한 문단 |
| `MS-12` | **안 됨** | 못 찾음 | — | `3-design.md:238` 의 반증 조항 6 이 발화했다는 사실이 **게이트에도 `intent.md` 에도 없다**(`grep '조항 6'` 적중 0). `DL2-23` 과 같은 것이다 — 게이트에 「조항 6 이 두 라운드 연속 걸렸고 `C2` 가 `C2-b` 에 종속한다」 한 문단 |
| `MS-13` | **실행됨** | `f75b4b0058d7d700d27591a26ae95042e71425f7` | `.palimpsest/rounds/2026-09-11-effect-confirmation/effect/negative-C2.md` | 없다. 등록된 이름으로 파일이 섰고 두 축(ⓐ `126-judge-1.md` · ⓑ `126-judge-2.md`)을 그 자리로 옮겨 적었다 |
| `DL2-03` | **부분** | `017e5f1ae832620d25ad0d4471cd621c555f4fbe` | `.palimpsest/rounds/2026-09-11-effect-confirmation/dialectic/1-synthesis-r2.md` | ★ 원장 행 자신이 *"처분 자리는 합의 「초안을 어떻게 고치나」 ① 이다"* 라 적었고 그 ① 이 그 파일에 **실재한다.** 남은 것은 기장뿐 — 처분자리를 그 파일로 적고 `017e5f1` 로 닫으면 된다 |
| `DL2-04` | **안 됨** | 못 찾음 | — | `xtask/src/main.rs:6425` 의 `unwrap_or(i64::MAX)` + 사전순 동점 가름이 **그대로 살아 있고** 시험 `시각이_같으면_사전순_최대를_고른다` 가 그것을 고정한다. 판정문(`:6356-6359`)에 그 사실이 **아직 없다** — `근거` 문면에 「시각 없음이 여럿이면 사전순」을 적거나 게이트에 적으면 닫힌다 |
| `DL2-05` | **부분** | `017e5f1ae832620d25ad0d4471cd621c555f4fbe` | `.palimpsest/rounds/2026-09-11-effect-confirmation/dialectic/1-synthesis-r2.md` | 합 fix ④(`#79` 의 `464`·`7,156` ↔ 실측 `20`·`3306` 교차 참조)가 그 파일에 있다. 기장만 남았다 |
| `DL2-06` | **부분** | 같음 | `.palimpsest/rounds/2026-09-11-effect-confirmation/dialectic/1-synthesis-r2.md` | 합 fix ②. ⚠ 가르는 명령(`cargo build --release && ./target/release/pal ledger` 로 `순서에 취약` 20 확인)을 **아무도 안 돌렸다** — 돌리고 산출을 남기면 온전히 닫힌다 |
| `DL2-07` | **부분** | 같음 | `.palimpsest/rounds/2026-09-11-effect-confirmation/dialectic/1-synthesis-r2.md` | 합 fix ⑤(「101 줄」은 `06e8d63` 단독값). 기장만 남았다 |
| `DL2-08` | **실행됨** | `017e5f1ae832620d25ad0d4471cd621c555f4fbe` | `crates/pal-cli/src/ledger.rs` | 없다. `:307-320` 의 거짓 문단이 「한때 이렇게 적혀 있었고 거짓이었다」로 갈렸고 `:942`·`:959` 의 두 주석도 함께 고쳐졌다 |
| `DL2-09` | **실행됨** | `017e5f1ae832620d25ad0d4471cd621c555f4fbe` | `.palimpsest/rounds/2026-09-11-effect-confirmation/effect/79-delta.md` | 없다. 행 **㉳** 가 섰고 게이트 `## 효과` 차이 표(`:110` 근처)에도 실렸다 |
| `DL2-10` | **안 됨** | 못 찾음 | — | ★ `effect/negative-B5-group.txt` 를 다시 읽어 확인했다 — `합이_분모와_같다` 는 `ledger.rs:933`(짝 단언)에서 빨개졌고 **`:929`(합 == 분모) 축은 안 껐다.** `f75b4b0` 의 새 음성 대조 다섯(`A1`·`A3`·`A5`·`A7`·`C2`)에도 이 축이 **없다.** `ledger.rs:369` 의 `return` 을 지운 사본에서 `cargo test -p pal-cli ledger::tests::합이_분모와_같다` 를 돌려 산출을 남기거나, 「미측정」임을 게이트에 적으면 닫힌다 |
| `DL2-11` | **부분** | `017e5f1ae832620d25ad0d4471cd621c555f4fbe` | `.palimpsest/rounds/2026-09-11-effect-confirmation/dialectic/1-synthesis-r2.md` | 합 fix ⑦ 이 좌표 넷(`:358-361→:366-370` · `:413-418→:416-419` · `:421→:425` · `coord.rs:126-127→:128`)을 되잡았다. 기장만 남았다 |
| `DL2-12` | **실행됨** | `30488ae3924d7a7e158bcb45e2e7e08751f1c8dd` | `.palimpsest/rounds/2026-09-11-effect-confirmation/intent.md` | 없다. 칸 7 이 서고 **`#140` ② 본문이 실제로 고쳐졌다**(`gh issue view 140` — *"⚠ 2026-09-12 에 이 항의 문면을 고쳤다"* + 실측 표 3/2/2 · `updatedAt` `2026-09-12T03:10:32Z`) |
| `DL2-13` | **부분** | `017e5f1ae832620d25ad0d4471cd621c555f4fbe` | `.palimpsest/rounds/2026-09-11-effect-confirmation/dialectic/3-synthesis-r2.md` | 합 fix 2 ⑴(readnote 넷째 칸으로 7/7 선다). 기장만 남았다 |
| `DL2-14` | **부분** | 같음 | `.palimpsest/rounds/2026-09-11-effect-confirmation/dialectic/3-synthesis-r2.md` | 합 fix 3(`intent.md:428` 로 문면 읽기가 잠겼다). 기장만 남았다 |
| `DL2-15` | **부분** | 같음 | `.palimpsest/rounds/2026-09-11-effect-confirmation/dialectic/3-synthesis-r2.md` | 합 fix 4. ⚠ *"결과 좌표 셋이 봉인 지정 자리라는 관측은 **발견으로 옮긴다**(`C4`·`D2` 의 입력이다)"* 인데 그 관측이 게이트 `## 효과` 에 **안 실렸다** |
| `DL2-16` | **실행됨** | `30488ae3924d7a7e158bcb45e2e7e08751f1c8dd` | `.palimpsest/rounds/2026-09-11-effect-confirmation/intent.md` | 없다. 칸 6 이 조항 8 에 대해 「집행 면제 · 드리프트를 기록으로」를 냈고 대가까지 적었다. ⚠ 회차의 다른 발화는 칸 12(`a8c0df4`)가 **집행**했다 |
| `DL2-17` | **실행됨** | `c807f32e91e8af073108a2cefd793cf4fd880258` | `docs/gates/effect-confirmation.md` | 없다. `#### 성격 — 판 3 라운드 2 가 산출한 것은 판정이 아니라 근거다 ⟨칸 8⟩` 절이 게이트에 섰다(칸 8 원문은 `65cc7ac` 의 `intent.md`) |
| `DL2-19` | **부분** | `017e5f1ae832620d25ad0d4471cd621c555f4fbe` | `.palimpsest/rounds/2026-09-11-effect-confirmation/dialectic/3-synthesis-r2.md` | 합 fix 8(「내가 새로 세웠다」를 ㉠ⓐ·㉠ⓑ 로 좁힌다). 기장만 남았다 |
| `DL2-20` | **부분** | 같음 | `.palimpsest/rounds/2026-09-11-effect-confirmation/dialectic/3-synthesis-r2.md` | 합 fix 7(「세상에 대해 거짓인 문장」과 덧붙임 요구를 뺀다). 기장만 남았다 |
| `DL2-21` | **부분** | 같음 | `.palimpsest/rounds/2026-09-11-effect-confirmation/dialectic/3-synthesis-r2.md` | 합 fix 9(분모를 일곱으로 통일). 원장 행 자신이 그 자리를 지목했다 — 기장만 남았다 |
| `DL2-23` | **안 됨** | 못 찾음 | — | `MS-12` 와 같은 것. 반증 조항 6 이 두 라운드 연속 걸렸다는 사실이 게이트·`intent.md` **어디에도 없다** |
| `DL2-24` | **실행됨** | `65cc7ac45ff9c0fceff890ed8419b5d4a1c19b3a` | `.palimpsest/rounds/2026-09-11-effect-confirmation/intent.md` | 없다. 칸 9 가 「면제가 다섯과 판 1 까지 덮는다」를 내고 다섯을 이름으로 실었다. ⚠ 칸 12(`a8c0df4`)가 그 「다섯」이 전수로는 **여섯**임을 뒤에 정정했다 |
| `DL2-25` | **실행됨** | `65cc7ac45ff9c0fceff890ed8419b5d4a1c19b3a` | `.palimpsest/rounds/2026-09-11-effect-confirmation/intent.md` | 없다. 칸 9 의 후반(판 1 도 핀 밖 바이트에 기댔고 조항 8 을 안 불렀다)을 그대로 받았다 |
| `DL2-26` | **안 됨** | 못 찾음 | — | 「`AT2-04` 는 `DL1-03` 의 재발이고 원장에 두 번 세어질 위험이 있다」가 **어디에도 안 적혔다**(`grep '재발'`·`'두 번 세어'` 적중 0). 두 행 중 하나를 다른 하나에 묶어 닫거나, 게이트에 「같은 코드 자리를 두 행이 친다」를 적으면 닫힌다 |
| `DL2-27` | **실행됨** | `a1493b50fb0185840df7cc1cc973cb0b31377f83` | `.palimpsest/rounds/2026-09-11-effect-confirmation/state.md` | 없다. 「내가 판 3 의 반(反)에게 준 과업문의 거짓 하나」가 `state.md` 에 일곱 줄로 섰다 |

## 계수

**실행됨 19 · 부분 26 · 안 됨 10 = 55**

- 실행됨 19 — `DL1-01` `DL1-06` `DL1-10` `DL1-18` `DL1-19` `DL1-20` `DL1-21` `DL1-29` `DL1-34` `DL1-35` `DL2-08` `DL2-09` `DL2-12` `DL2-16` `DL2-17` `DL2-24` `DL2-25` `DL2-27` `MS-13`
- 부분 26 — `DL1-04` `DL1-14` `DL1-15` `DL1-16` `DL1-17` `DL1-22` `DL1-23` `DL1-24` `DL1-27` `DL1-28` `DL1-30` `DL1-31` `DL1-32` `DL1-33` `DL1-36` `DL2-03` `DL2-05` `DL2-06` `DL2-07` `DL2-11` `DL2-13` `DL2-14` `DL2-15` `DL2-19` `DL2-20` `DL2-21`
- 안 됨 10 — `DL1-02` `DL1-03` `DL1-25` `DL2-04` `DL2-10` `DL2-23` `DL2-26` `MS-10` `MS-11` `MS-12`

## 「안 됨」 열 — 실물이 하나도 없는 것

| id | 해악도 | 무엇을 어디에 쓰면 닫히나 |
|---|---|---|
| `MS-10` | 거짓신호 | `crates/pal-git/src/lib.rs:193-197` 의 doc 주석에 *"이 검사는 `--at` 을 줬을 때만 발화한다 — `--at` 없는 경로에서는 같은 `WorktreeState` 의 두 변을 견주므로 언제나 참이다"* |
| `MS-11` | 미관 | 「워킹트리」 세 뜻(`ledger.rs:714` 스냅숏 축 · `:729-737` 더러움 · `pal touch` 근거 상자)을 가르는 한 문단. `ledger.rs` 주석 또는 게이트 `## 효과` |
| `MS-12`·`DL2-23` | 거짓신호 | 게이트에 *"`3-design.md:238` 의 반증 조항 6 이 두 라운드 연속 걸렸다 — `C2` 가 `C2-b` 에 종속한다는 쪽으로 증거가 하나 늘었다"* 한 문단 (둘이 같은 관측이므로 한 문단이 둘을 닫는다) |
| `DL1-02` | 거짓신호 | `44 건(1.6%)` ↔ `20/3306` 의 4 배 어긋남 조정. 게이트 정오표 또는 `effect/79-delta.md` |
| `DL1-03` | 거짓신호 | `xtask/src/main.rs` 의 `mod 최근_끝난_시험` 에 `종료_커밋_시각` 을 거는 시험(비-git 트리에서 `None` 인지) |
| `DL1-25` | 거짓신호 | `#140` 에 네 번째 항, 또는 게이트 정오표에 *"`C2-b` 의 `N` 결박을 판정자가 한 번도 안 쟀다"* |
| `DL2-04` | 거짓신호 | `xtask/src/main.rs:6356-6359` 의 `근거` 문면에 「시각 없는 회차가 여럿이면 사전순으로 가른다」를 적는다 |
| `DL2-10` | 거짓신호 | `ledger.rs:369` 의 `return` 을 지운 사본에서 `cargo test -p pal-cli ledger::tests::합이_분모와_같다` 를 돌려 `effect/negative-B5-sum.txt` 로 남기거나, 「이 축은 미측정」을 게이트에 적는다 |
| `DL2-26` | 거짓신호 | `DL1-03` ↔ `DL2-04` 가 같은 코드 자리를 친다는 사실. 게이트 정오표 또는 두 행 중 하나를 묶어 닫는 기장 |

## 「부분」 스물여섯 — 한 자리만 더 서면 닫힌다

**전부 같은 형태다** — 정정 내용은 **합(合)의 `## 초안을 어떻게 고치나`** 에 커밋돼 있고,
빠진 것은 **독자가 초안을 만나는 자리의 정오표**다.

| 어느 합이 지나 | id | 개수 |
|---|---|---|
| `dialectic/1-synthesis.md` | `DL1-04` | 1 |
| `dialectic/2-synthesis.md` | `DL1-14` `DL1-15` `DL1-16` | 3 |
| `dialectic/3-synthesis.md` | `DL1-17` `DL1-22` `DL1-23` `DL1-24` | 4 |
| `dialectic/4-synthesis.md` | `DL1-27` `DL1-28` `DL1-30` `DL1-31` `DL1-32` `DL1-33` `DL1-36` | 7 |
| `dialectic/1-synthesis-r2.md` | `DL2-03` `DL2-05` `DL2-06` `DL2-07` `DL2-11` | 5 |
| `dialectic/3-synthesis-r2.md` | `DL2-13` `DL2-14` `DL2-15` `DL2-19` `DL2-20` `DL2-21` | 6 |

★ **갈림길 하나** — 이 스물여섯을 닫는 길이 둘이다.

1. **합 문서를 `처분자리` 로 적고 그 합을 보존한 커밋(`af981d1`·`e44a87e`·`017e5f1`)으로 닫는다.**
   원장 행 셋(`DL2-03`·`DL2-15`·`DL2-21`)이 **스스로 그 자리를 지목했으므로** 이 읽기는
   회차 자신의 문면에 근거가 있다. `check_finding_closure` 도 `처분자리` 를 최우선으로
   읽으므로 기계가 통과한다.
2. **게이트 `### 정반합 산출물 — 정오표와 성격` 에 스물여섯을 실어 닫는다.**
   `AT5-14` 가 받은 처분과 **같은 형태**이고, 합 fix 지시 중 하나(`3-synthesis.md:48`)가
   *"고치지 말고 정오표를 옆에 붙여라"* 로 **명시로 요구한 것**이다.

⚠ **둘 중 무엇을 고르든 `DL1-17` 만은 2 를 지나야 한다** — 그 행의 합 지시가 「정오표를
붙여라」 자체이므로, 1 로 닫으면 처분이 스스로를 안 지킨 것이 된다.

## 함께 잰 것 — 표 밖

- **`f75b4b0`** 가 감사 중에 들어왔다(음성 대조 다섯). `MS-13` 이 그 커밋으로 실행됨이 됐다.
- 워킹트리에 아직 커밋 안 된 것 둘 — `dialectic/r3-raw.md`(항목 **D**) · `observations/e1-measure.md`.
  `r3-raw.md` 가 실리면 **열린 발견이 55 보다 는다**(`state.md` 항목 `G` 가 예고한 그대로다).
- `docs/gates/effect-confirmation.md` 는 지금 **336 줄**이고 정오표 절이 지는 것은 넷뿐이다 —
  `AT5-14` · 판 5 R2 부수 발견 ①②③ · 칸 8 성격 · 칸 12 무효. **`DL1-*`·`DL2-*` 는 한 건도 없다.**
