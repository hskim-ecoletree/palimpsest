# 독립 리뷰 — 라운드 2 · 2026-09-12-binding-radius-in-use

> ⚠ **이 파일은 원 반환문이다. 메인이 요약하거나 고치지 않는다** — 합계 검산이 독립된
> 둘째 원천을 대야 하기 때문이다. **`pal-independent-reviewer` 가 냈고, 읽기 전용이라
> 파일을 못 써서 전문을 반환문에 실었고 메인이 옮겼다.** 옮기면서 바꾼 것은 없다 —
> 판정도 순서도 문면도 그대로다. 첫 네 줄의 영문 상태 보고만 이 머리말로 갈렸다.
>
> **리뷰어가 안 받은 것**: 대화 기록 · 메인의 사고 과정 · 왜 그렇게 만들었는지의 설명 ·
> 앞 라운드의 판정(시드로). ⚠ `review/r1-raw.md` 는 **회차 산출물로 저장소에 있고** 리뷰어가
> 그 존재를 확인했으나 **인용하지 않았다** — 모든 판정을 직접 재서 냈다.
>
> ★ **이 라운드는 게이트와 갈렸다** — 게이트가 통과 47 · 반증 1 · 미측정 2 를 주장하고
> 리뷰어가 **통과 43 · 반증 3 · 미측정 4** 를 냈다. 갈린 넷의 처분은 커밋 `742255a` 다.
>
> ★ **처분은 이 파일에 없다.** 처분은 `findings.jsonl` 의 레코드가 지고 §5 의 자로 메인이
> 정한다. 이 파일은 **무엇이 보고됐나**만 진다.

---

## 내가 받은 것 / 안 받은 것

| 받았다 | 안 받았다 |
|---|---|
| `.palimpsest/rounds/2026-09-12-binding-radius-in-use/intent.md` 전문 (415 줄) · `docs/gates/binding-radius-in-use.md` · `approval.md` · `state.md` · `widen/` · `bench/` · `effect/` · `observations/` · `dialectic/` · `premortem/` · `conditions-audit/` · `review/r1-raw.md` · 코드·`docs/`·`xtask/`·`.claude/skills/round/SKILL.md` · `git log 1276b8f..HEAD` · 빌드된 `target/release/pal` · `gh` | 대화 기록 · 메인의 사고 과정 · 왜 그렇게 만들었는지의 설명 · 앞 라운드의 판정(시드로) |

`review/r1-raw.md` 는 존재를 확인하고 **인용하지 않았다** — 아래 모든 판정은 내가 직접 재서 낸 것이다. R1 이 지나간 자취(정정 ⟨⟩ 주석)가 산출물 안에 있으므로 그 문면은 읽었다.

## 내가 실제로 돌린 명령

```
git status --porcelain / git log --oneline 1276b8f..HEAD / git log --diff-filter=A --name-only 1276b8f..HEAD
cargo xtask check                          → 28/28 통과 · rc=0 (전 출력 33 줄을 파일로 받아 읽었다)
cargo xtask test                           → rc=0 · test result 54 줄 · passed 합 1065 · failed 0 · ignored 2
                                             (warning 0 · error[E 0 · panicked 0 — 전 출력 1351 줄에서 셌다)
cargo check --workspace --all-targets --target-dir <격리>   → rc=0 · warning 0 (새 target-dir 로 전량 재컴파일)
pal query binding.status --json            → 결박 39 · callers 20 · symbol 19 · watch 합 193 · fresh 27 · stale 12 · undeterminable 0
pal touch extract_with (2 회)              → 「지켜보는 것 (2)」 · 두 번이 cmp 로 동일
pal touch with_radius                      → 「걸린 것 (1)」 callers 반경 · 감시 1 · 호출자 0
pal doctor --json                          → 불변식 1·2·3 위반 0 · 4~8 not_built
pal radius 8597d886cb7afc70 --to symbol --dry-run --json   → 감시전 2 · 감시후 2
pal radius 3d9979946eb321de --to symbol --dry-run --json   → 감시전 78 · 감시후 78   ★
git clone <repo> <scratchpad>/clone; cargo xtask check --root <clone>   → 28/28 (기준선 초록)
  ① 사본 게이트에서 A1 을 대조불가로 옮기고 검산 줄만 맞춤 → FAIL 원장 둘 대조 · rc=1
  ② 사본 게이트 대조불가 행에 `[f09.pass]` ⑤ 를 넣음     → FAIL 형식 오류 · rc=1   ★
gh issue list --limit 20 --state all       → #145 #146 #147 #148 #149 #150 #151 #152 전부 OPEN
gh issue view 58 --comments                → 새 게이트 경로 2 곳
git show 1276b8f:.palimpsest/intent/bindings.jsonl 를 떠서 현재 정본과 필드 대조 (python3)
git diff 1276b8f..HEAD -- docs/plan/02-order.md · crates/pal-cli/src/main.rs · docs/gates/F09.md · xtask/src/main.rs
git merge-base --is-ancestor fd176a8 e16dc61
grep 다수 (A6 · D2 · D3 · D4 · E6 · F1~F4 · check_budget · WATCH_ENTRY_SITES · 조건 ID 계수)
```

---

## 합격선 축

**등록 조건 50 개. 게이트 `## 판정` 은 통과 47 · 반증 1 · 미측정 2 를 주장한다.
내가 잰 값은 통과 43 · 반증 3 · 미측정 4 다** (갈린 넷: `B5` · `E1` 은 통과→반증, `A5` · `B6-a` 는 통과→내가 못 쟀다).

| 조건 | 판정 | 잰 수 | 근거 |
|---|---|---|---|
| A1 | 통과 | 37/37 · 위반 0 | `1276b8f` 정본 37 건과 현재 정본을 python 으로 대조. `subject`·`note`·`bound_at`·`bound_at_time`·`target` 전부 동일, 옛 `(symbol,digest)` 쌍이 전부 부분집합. `promoted_by` 도 보존 |
| A1-a | 통과 | 시험 1 | `pal_bind_재호출은_여섯을_안_보존하고_stale_을_지운다` 가 `cargo xtask test` 1065 패스 안에서 초록. 조건 문면이 지정한 `effect/negative-A1.md` 는 없고 시험이 그 자리를 진다 |
| A2 | 통과 | 7/7 | 착수 `stale` 7 건이 전부 아직 `stale` 이고 `triggered_by` 에 옛 대상 심볼이 그대로 있다(내가 base 정본의 `target` 과 현재 `triggered_by` 를 대었다). `undeterminable` 이동 0 이라 제외 집합이 비었다 |
| A2-a | 통과 | 시험 1 | 같은 시험이 이 축도 진다 |
| A3 | 통과 | 시험 1 · 코드 1 자리 | `새_감시_원소의_기준은_bound_at_의_base_커밋이다` 초록. `crates/pal-cli/src/radius.rs:155-160` 이 base 커밋을 **별도 임시 색인**에 붙여 읽고 `:196` 에서 그 값만 쓴다 |
| A3-a | 통과 | 시험 1 | `기준을_head_로_읽으면_fresh_가_난다` 초록 |
| A3-b | 통과 | 열 2 · 참 1 · 합 10 | `widen/decisions.tsv` 에 `대상digest차이`(참 1 · 거짓 36)와 `base에없던새원소`(합 10)가 있다. 참인 1 건 `49714573e0fa654c` 는 안 넓혀졌으므로 표시할 새 `stale` 이 0 이다 |
| A4 | 통과 | 시험 1 · 코드 1 | `없는_결박을_지목하면_실패한다` 초록. `radius.rs:141-147` 이 `bail!` 로 멈춘다 |
| A5 | 미측정 | 0 | 내가 안 쟀다 — 아래 `## 미측정 목록` 1 |
| A5-a | 통과 | 검사 12 초록 · 자리 14 · 등록 4 | `cargo xtask check` 의 「앵커는 신고받지 않는다」가 초록이고, `xtask/src/main.rs:2056` 이 `crates/pal-cli/src/radius.rs` 를 사유 *"반경만 바꿀 때 base 커밋 투영에서 읽어 만든다"* 로 등록했다 — 조건이 요구한 사유 그대로이고 「넷째 종류」가 아니다 |
| A6 | 통과 | grep 1 건 · diff 0 건 | `crates/pal-cli/src/main.rs:140` 에 `default_value = "symbol"` 그대로. `git diff 1276b8f..HEAD` 에 그 줄이 안 든다(든 것은 `default_value = "."` 뿐) |
| B1 | 통과 | 37 행 · 14 열 | 헤더 제외 37 행. 열 이름 14 개가 조건이 적은 목록과 순서까지 같다 |
| B1-a | 통과 | 18 대 37 | 내가 `새반경 == callers` 만 세었다 = 18. 18 ≠ 37 이므로 `B1` 이 빨개진다 |
| B2 | 미측정 | 0 | 상자가 `[ ]` 이고 판정자가 없다 — 아래 `## 미측정 목록` 3 |
| B2-a | 통과 | 커밋 2 · 조상 참 | `git merge-base --is-ancestor fd176a8 e16dc61` 이 참. `decisions.tsv` 가 `fd176a8`, 넓힘이 `e16dc61` |
| B3 | 통과 | 3/3 축 | ⑴ `callers` 20 · `symbol` 19 ⑵ 감시 합 193 그리고 착수 37 (대상 37 안에서는 190) ⑶ `pal touch extract_with` 가 「지켜보는 것 (2)」 — 내가 지금 돌렸다 |
| B3-a | 통과 | 관측 1 파일 · 커밋 순서 | `observations/red-B3.txt` 가 `750e3ea` 에서 37/37 `symbol` · 감시 37 · `fresh` 30 `stale` 7 · `pal touch extract_with` = `(0)` 을 싣고, 그 커밋이 넓힘 `e16dc61` 보다 앞이다 |
| B4 | 통과 | 7 건 · 합 190 = 190 | `새반경 == callers` 이고 `감시전 == 감시후` 인 행 = **7**. `감시후` 합 190 이 대상 37 의 `watch` 합 190 과 같다. ⚠ 명령이 지금 돌려주는 전체 합은 **193** 이다(회차가 결박 둘을 더했다) |
| B5 | **반증** | 오라클 2 중 1 | 앞 절반은 섰다 — `observations/red-B3.txt` §④ 가 `37 × 78 = 2886` 과 `1_000_000` 을 나란히 적는다. **뒤 절반이 없다** — `보존 경로가 check_budget 을 부르는지를 시험으로 잰다` 를 지는 시험이 저장소에 0 건이다(`grep -rn check_budget crates` 는 `radius.rs:221` 호출 자리와 `radius.rs:329-330` 의 함수 자신 단위시험만 낸다). 아래 발견 5 |
| B6 | 통과 | 18 삽입 18 삭제 | `git show e16dc61 --stat -- .palimpsest/intent/bindings.jsonl` 이 비어 있지 않다 |
| B6-a | 미측정 | 0 | 내가 안 쟀다 — 아래 `## 미측정 목록` 2 |
| B7 | 통과 | ⑴ 0 행 · ⑵ 모집단 0 | `판정 == 아니다` 이고 `새반경 != 옛반경` 인 행이 **0**. `판정 == 영향받는다` 이고 `새반경 == 옛반경` 인 행이 **0**(조건이 적은 대로 ⑵ 는 공전한다) |
| B8 | 통과 | 합 112 | `타파일새원소` 합 112. `widen/README.md` 와 `docs/plan/02-order.md:84` 가 싣는다. 0 이 아니므로 「게이트에 적는다」 절은 발화하지 않는다 |
| C1 | 통과 | 5 건 id 집합 | 내가 집계한 대상 37 의 분포가 `fresh` 25 · `stale` 12 · `orphaned` 0 · `undeterminable` 0 이고 `observations/after-widen.txt` 의 수와 같다. 바뀐 5 건 id 가 내가 센 신규 `stale` 집합과 정확히 일치 |
| C2 | 통과 | 0 건 · id 집합 `[]` | `status.code.reason == "watch_member_gone"` 집계 = 0. 0 이 산출에 적혔다 |
| C2-a | 통과 | 시험 1 | `사라진_감시_원소가_변한_것을_가린다`(`crates/pal-core/src/binding.rs:1028`)가 하한(①에서 `Stale` 이 난다)까지 함께 걸고 초록 |
| C3 | 통과 | 7 행 판정 | `dialectic/2-synthesis.md` 가 새 `stale` 5 + `fresh` 대조 2 를 술어 P 로 판정해 전부 `아니다`. 그 5 개 id 가 내가 센 신규 `stale` 집합과 일치. ⚠ 판정자가 사람이 아니다 — 아래 발견 8 |
| C4 | 통과 | 0 ≤ 5 | 내가 집계한 `undeterminable` = **0**, 새 `stale` = **5**. 되돌린 결박 0. ⚠ 조건의 되돌림 조항은 원리상 실행 불가 — 아래 발견 7 |
| D1 | 통과 | 검사 초록 · 산문 2 자리 | `cargo xtask check` 「원장 둘 대조」가 이 회차를 **(50개)** 로 훑고 초록. `[f09.pass]` ⑤ 는 게이트 `:107` 과 `:138` 에서 `대조불가` 다. ⚠ 오라클이 지목한 「표준 표의 대조불가 행」은 `—` 이고 원리상 채울 수 없다 — 아래 발견 10 |
| D1-a | 통과 | 기준선 초록 → 심은 뒤 rc=1 | **내가 재현했다.** `git clone` 사본에서 `cargo xtask check --root` 가 28/28 초록 → 게이트만 고쳐 `A1` 을 대조불가로 옮기니 `FAIL 원장 둘 대조: A1 — 게이트는 「대조불가」, intent.md 는 「통과」다` · rc=1 |
| D2 | 통과 | grep 3/3 | 새 게이트에 `이 코퍼스에서` 1 건 · `scripts/f09-verify.py:715` 1 건 · `scripts/f09-verify.py:709` 1 건. `observations/red-D2.txt` 가 RED 를 진다 |
| D3 | 통과 | diff 0 줄 · grep 3 · 코멘트 2 | `git diff 1276b8f..HEAD -- docs/gates/F09.md` 가 0 줄. 새 게이트에 `docs/gates/F09.md` 3 건. `gh issue view 58 --comments` 에 새 게이트 경로 2 곳 |
| D4 | 통과 | 절 1 · 표 1 · 문장 1 | `### ⑤ 와 ⑥ 이 양립하는 까닭`(`:134-141`)이 둘을 한 절에 놓고 `:141` 이 *"⑥ 의 통과가 ⑤ 의 대조 불가를 덮지 않는다"* 를 적는다 |
| E1 | **반증** | 10 행 / 요구 16 | 조건 문면은 **넷 × 두 규모 × 두 `changed` × 회차 3** 인데 `bench/radius.tsv` 는 10 행이다 — ①②③ 는 두 규모이고 `changed` 축이 없으며, ④ 는 `changed` 둘인데 **규모 8000 한 곳**만이다. `#[ignore]` 시험은 섰고(`cargo xtask test` 의 ignored 2 중 하나) 넷째 팔이 대리물임도 산출 머리가 찍는다. 게이트가 10 을 공개하고 *"넓게 적힌 것이 문장뿐"* 이라 논증하지만 **조건 문면은 안 고쳐졌다.** 아래 발견 11 |
| E2 | 통과 | 열 4 · 비 3 | `bench/radius.tsv` 에 `회차3최솟값ms`·`회차간분산`·`두규모의비`·`changed` 열이 있고, `bench/radius-run.txt` 머리가 3.84 · 3.87 · 4.02 를 다시 계산해 표와 같다 |
| E3 | 통과 | 두 비 2 · 1 을 건넌다 | `changed 1` 86.157배 · `changed 801` 0.572배 — 1 을 건너뛴다. 대리물 사유 ⑴ 도 머리에 적혔다 |
| E4 | 통과 | 2 파일 · 첫 줄 탭 | `bench/radius.tsv` 첫 줄이 탭으로 가른 열 이름 8 개. `widen/decisions.tsv` 도 같다 |
| E5 | 통과 | `.json` 0 개 | `find <회차 디렉터리> -name '*.json'` 이 0. sunset 검사 초록(*"트리거는 아직 0건"*). `xtask/src/main.rs:3025` 가 확장자 `json` 만 세므로 `findings.jsonl` 은 안 걸린다 |
| E6 | 통과 | grep 0 건 · 문장 2 · 출력 1 | `grep -F -- '--ignored' .github/workflows/ci.yml xtask/src/main.rs` = 0. `xtask/src/main.rs:384-387` 의 축이 둘뿐임을 읽었다. 게이트 `## 범위 밖` 에 그 사실, `## 효과` 에 벤치 출력 |
| F1 | 통과 | grep 4/4 · diff 1 행 | §2 3 번 행(`:138`)에 `F05`·`F13`·`F15`·`binding.rs:504` 가 다 있고 `git diff` 에 그 줄이 든다 |
| F2 | 통과 | 바이트 동일 | `docs/plan/02-order.md:53` 이 `1276b8f` 의 같은 줄과 동일하고 diff 의 어느 헌크에도 안 든다 |
| F3 | 통과 | 새 절 1 · 세 줄 불변 | 새 `### 그 뒤 무엇이 바뀌었나 — C6 행이 움직였다` 가 `:69` 뒤 · `### 실측이 갈린 자리 셋` 앞에 들어갔고, `:67-69` 는 diff 의 문맥 줄로만 나온다(안 바뀌었다) |
| F4 | 통과 | grep 0 건 · `:68` 불변 | `grep -F "파일 안의 관계만" docs/plan/02-order.md` = 0, `crates/` = 0. `:68` 이 diff 의 문맥 줄이다 |
| G1 | 통과 | 이슈 4/4 OPEN | `gh issue list` 로 #145 #146 #147 #148 넷 다 OPEN. 번호는 `47a0014` 커밋 메시지가 ⓐⓑⓒⓓ 라벨과 함께 싣는다 |
| G2 | 통과 | 28/28 · rc=0 | 내가 돌렸다. 검사 수 28 ≥ 28 |
| G3 | 통과 | passed 1065 ≥ 1047 | 내가 전 출력 1351 줄을 파일로 받아 `^test result:` 54 줄에서 합산했다. failed 0 · rc=0. 착수 1047 은 `observations/red-G3.txt` 가 진다 |
| G4 | 통과 | warning 0 | 격리 `--target-dir` 로 `cargo check --workspace --all-targets` 를 **전량 새로** 돌려 warning 0 · rc=0. ⚠ `cargo build --release --workspace` 축은 내가 다시 안 돌렸다 |
| G5 | 미측정 | 0 | `git rev-list --count origin/main..HEAD` = **26**. `origin/main` 이 아직 `1276b8f` 다 — push 전에는 원리상 못 잰다 |
| H1 | 반증 | 참인 재검토 0 / 5 | 새 `stale` 5 건(0 건이 아니므로 `대조불가` 길이 닫혔다). `dialectic/2-synthesis.md` 가 7 행 전부 `아니다` 로 판정해 참인 재검토 0. ⚠ 오라클 셋째 다리(이슈를 열었다)의 처분이 게이트에 없다 — 아래 발견 15 |
| H2 | 통과 | cmp 동일 · 차이 2 자리 | `pal touch extract_with` 를 두 번 돌려 `cmp` 통과. 산출 머리에 명령 전문과 `f1e8cdb` 가 적혔다. 지금 출력이 그 산출과 SHA 줄과 파일 수 두 줄 말고 같다 |

## 미측정 목록

| # | 안 잰 조건 | 모집단 | 유효 | 해악도 | 좌표(파일:줄) | 왜 못 쟀나 |
|---|---|---|---|---|---|---|
| 1 | `A5` 넓힘 재현이 바이트까지 같은가 | 회차기록 | 참 | 미관 | `.palimpsest/rounds/2026-09-12-binding-radius-in-use/effect/rerun-widen.txt:1` | 재현은 임시 의도 저장소를 세워 `pal intent import` 와 `pal radius` 18 회를 **쓰는** 경로다. 작업 트리를 안 고친다는 제약과 `import` 가 정본을 되돌리는 위험(`B6-a` 가 잰 것) 때문에 안 돌렸다. 그리고 지금 정본은 39 건이라 조건이 적은 `cmp` 대상이 이미 다르다 |
| 2 | `B6-a` `import`→`export` 순서로 넓힘이 사라지나 | 회차기록 | 참 | 미관 | `.palimpsest/rounds/2026-09-12-binding-radius-in-use/effect/negative-B6.md:1` | 같은 이유 — 쓰는 경로다. 산출은 명령과 출력을 싣고 있으나 내가 재현하지 않았으므로 「쟀다」로 안 적는다 |
| 3 | `B2` 근거가 결박문을 읽은 판정인가 | 회차기록 | 참 | 거짓신호 | `.palimpsest/rounds/2026-09-12-binding-radius-in-use/intent.md:308` | ⟨정반합⟩ 태그인데 어느 판도 이 조건을 판정 대상으로 안 적었다(`dialectic/2-design.md` 의 대상은 `C3`·`H1`·`A3-b`·`B2-a`). 규약 §5 가 혼자 판정하는 것을 금하므로 나도 못 판정한다. 실질은 `widen/decisions.tsv` 37 행의 `판정`·`근거` 와 `widen/README.md` 가 진다 |
| 4 | `G5` 마지막 커밋 SHA 에 `conclusion=success` | 회차기록 | 참 | 미관 | `.palimpsest/rounds/2026-09-12-binding-radius-in-use/intent.md:364` | push 가 아직 없다(26 커밋 미push). 원리상 push 뒤에만 선다 |

## 의도 축

### 빠진 것

| # | 무엇이 걸리나 | 좌표 | 모집단 | 유효성 | 해악도 | 조건 |
|---|---|---|---|---|---|---|
| 5 | `B5` 가 요구한 **「보존 경로가 `check_budget` 을 부르는지를 시험으로 잰다」를 지는 시험이 저장소에 0 건**이다. `pal radius` 는 `radius.rs:221` 에서 실제로 부르지만 그 호출을 붙드는 시험이 없어, 지우면 아무것도 빨개지지 않는다. `grep -rn check_budget crates` 의 다섯 자리 중 시험은 `crates/pal-core/src/radius.rs:329-330`(함수 자신의 단위시험)뿐이고 `crates/pal-cli/tests/` 에는 `예산`·`budget` 이 한 글자도 없다. 게이트는 `B5` 를 통과로 적었다 | `crates/pal-cli/tests/radius_preserves_verdict.rs:1` | 원의도 | 참 | 거짓신호 | `B5` |
| 6 | **㈁(불변식 ⑦)이 완성선에서 빠졌는데 그것을 되받는 소유자 발화가 잠긴 원장에 없다.** `## 원문` 의 인용 아홉은 *"완성선을 어디에 두나? → ㈀ + ㈁ + ㈂ 전부"* 만 싣고, 빠졌다는 사실을 지는 것은 **이 회차가 쓴 편집자 산문 셋**(`## 원문` 의 ⚠ · `## 범위 밖` 첫 항 · `## 개정` 첫 행)이다. 회차는 그 사실을 스스로 공개했으나 **원문 24 행이 잠근 집합의 1/3 이 회차 산출물로만 해제된 상태가 그대로 남는다.** #145 가 이슈로 섰다 | `.palimpsest/rounds/2026-09-12-binding-radius-in-use/intent.md:242` | 원의도 | 참 | 거짓신호 | 없음 |

### 요구되지 않은 것

없음 — ㈄(`F1`~`F4`)는 의도가 스스로 *"소유자의 「전부」에 없다. 이 회차가 더했다"* 로 표시하고 `approval.md` 가 소유자의 *"㈎ 전부 승인 — ㈄ 포함"* 을 원문으로 실으며, 실행 커밋 `8ceb8e9` 가 승인보다 뒤임을 확인했다. `--dry-run`·`--json` 손잡이는 `B2-a` 가 요구한 「넓히기 전에 같은 함수로 재기」의 수단이고 조건 안이다.

### 있는데 틀린 것

| # | 무엇이 걸리나 | 좌표 | 모집단 | 유효성 | 해악도 | 조건 |
|---|---|---|---|---|---|---|
| 7 | **`C4` 의 되돌림 기준이 원리상 실행 불가다.** `C4` 는 불통과일 때 *"그 결박들을 안 넓힌 상태로 되돌리고(보존 경로가 반경을 되돌리는 방향으로도 서야 한다)"* 를 요구하는데, 보존 경로는 **반경 이름만 좁히고 감시 집합은 안 줄인다.** 내가 실측했다 — `pal radius 3d9979946eb321de --to symbol --dry-run --json` 이 `감시전 78 · 감시후 78 · 새반경 symbol` 을 낸다(작은 건도 2→2). 그 결과 상태는 `symbol 반경 · 감시 78` 이라 선언과 실물이 어긋나고, 이것은 `B4` 가 막으려던 거짓 신호(*"callers 라 적히고 감시가 1"*)의 거울상이다. 결정 자체는 결박 `6c6e402d958fc5e1` 의 결박문이 *"감시 집합은 이 함수를 지나 줄지 않는다 … 선언과 실물은 다른 축이다"* 로 **의도로 적어 두었다** — 그러므로 틀린 것은 코드가 아니라 **`C4` 의 조항과 그 어긋남이 게이트에 안 적힌 것**이다. `C4` 는 합격선(0 ≤ 5)으로는 통과했으므로 이 조항은 잠들어 있다 | `crates/pal-core/src/binding.rs:439-446` 그리고 `.palimpsest/rounds/2026-09-12-binding-radius-in-use/intent.md:326` | 원의도 | 참 | 거짓신호 | `C4` |
| 8 | **원문 27 행이 잠근 「사람이 결박문을 읽고 판정한다」의 판정자가 사람이 아니다.** `widen/decisions.tsv` 의 `판정`·`근거` 는 *"서브에이전트 하나(2026-09-13)"* 가 냈고(`widen/README.md`), `C3`·`H1` 의 판정은 `pal-decision-synthesizer` 서브에이전트가 냈다(`dialectic/2-synthesis.md` 머리). 두 산출이 판정자를 정직하게 밝히지만, `intent.md` 의 「합이 원 의도를 덮나」 표는 27 행에 대해 **`C3` · 덮인다** 로만 적고 **치환을 적지 않는다.** 소유자도 다른 사람도 일곱 결박문을 읽지 않았다 | `.palimpsest/rounds/2026-09-12-binding-radius-in-use/intent.md:377` | 원의도 | 참 | 거짓신호 | `C3` · `B2` |

## 이번 라운드의 새 발견

| # | 무엇이 걸리나 | 좌표 | 모집단 | 유효성 | 해악도 | 조건 |
|---|---|---|---|---|---|---|
| 9 | **순서표의 「회차 뒤」 칸이 회차가 끝난 상태와 다르다.** `docs/plan/02-order.md:80-81` 이 `회차 2026-09-12-binding-radius-in-use 뒤` 칸에 `symbol 19 · callers 18` 과 `감시 집합 크기 합 190` 을 적는데, 내가 `pal query binding.status --json` 으로 잰 지금 값은 **`symbol 19 · callers 20` · 감시 합 193** 이다. 까닭은 회차가 그 표를 `8ceb8e9` 에 쓴 뒤 `b1c0d4f` 에서 **자기 결정 둘을 결박했다**(규약 §11 조건 4)는 것이다. `:82` 의 `fresh 25 · stale 12` 도 37 집합의 값이고 전체는 `fresh 27 · stale 12` 다. 이 파일은 회차 기록이 아니라 **살아 있는 계획 문서**이고, 다음 회차가 이 칸을 현재 상태로 읽는다 | `docs/plan/02-order.md:80` | 저장소 | 참 | 거짓신호 | `F3` |
| 10 | **`D1` 의 오라클 첫 다리가 원리상 초록이 될 수 없는 자리를 가리키는데 `D1` 이 통과로 적혔다.** `D1` 의 오라클은 *"새 게이트의 **표준 표**에 그 조건 ID 가 `대조불가` 행에 실리고"* 다. 게이트 `## 판정` 의 표준 표 `대조불가` 행은 **`—`** 이고 검산은 `대조불가 0` 이다. 그리고 그 자리에 `[f09.pass]` ⑤ 를 넣으면 검사가 빨개진다 — **내가 사본에 심어 확인했다**: `FAIL 원장 둘 대조: 형식 오류 · 대조불가 행의 [f09.pass] 이 조건 ID 꼴이 아니다` · rc=1. `record.py:235-237` 이 그 경계를 스스로 적는다(*"ⓑ 꼴의 합격선 표는 대조 밖이다"*). 그러므로 `D1` 의 통과는 **오라클이 낸 것이 아니다** — 실질은 산문(`:107`)과 §㈂-a 의 표(`:138`)가 지고 뒤 절반(원장 둘 대조)만 살아 있다. 「측정이 죽은 가지」이므로 미룰 수 없다 | `.palimpsest/rounds/2026-09-12-binding-radius-in-use/intent.md:334` 그리고 `docs/gates/binding-radius-in-use.md:235` | 회차기록 | 참 | 금지역 | `D1` |

## 자기 산출에 대한 발견

| # | 무엇이 걸리나 | 좌표 | 모집단 | 유효성 | 해악도 | 조건 |
|---|---|---|---|---|---|---|
| 11 | **`E1` 이 등록한 조합 16 중 10 만 돌았는데 통과로 적혔다.** ①②③ 는 두 규모이고 `changed` 축이 없으며 ④-가·④-나 는 `changed` 둘인데 **규모 8000 한 곳**뿐이다. 게이트 §㈂-b 가 표로 그 사실을 공개하고 *"쓰임에는 구멍이 없다 — 넓게 적힌 것이 문장뿐이었다"* 로 논증하지만, **조건 문면은 안 고쳐졌고 판정은 통과다.** 등록된 합격선과 판정이 갈린 자리가 남는다 | `bench/radius.tsv:1` 그리고 `.palimpsest/rounds/2026-09-12-binding-radius-in-use/intent.md:344` | 자기장치 | 참 | 거짓신호 | `E1` |
| 12 | **`state.md` 의 「지금 선 것」이 두 모집단의 수를 한 줄에 섞었다.** *"결박 **39** 건 (`callers` 20 · `symbol` 19) · 감시 합 **190**"* — 39 건의 감시 합은 내가 잰 값으로 **193** 이고, 190 은 대상 37 집합의 값이다. 다음 컨텍스트가 받는 요약이 이 줄이다 | `.palimpsest/rounds/2026-09-12-binding-radius-in-use/state.md:70` | 회차기록 | 참 | 거짓신호 | `B3` · `B4` |
| 13 | **`approval.md` 가 소유자에게 보인 미리보기의 합이 같은 파일이 못 박은 조건 수와 다르다.** 미리보기가 `A(11) B(10) C(5) D(5) E(6) F(4) G(5) H(2)` 인데 합이 **48** 이다. 내가 조건 ID 를 세면 **B 는 12**(`B1 B1-a B2 B2-a B3 B3-a B4 B5 B6 B6-a B7 B8`)이고 전체가 50 이다. `B(10)` 은 조건 평가 R2 **이전**(45 조건)의 수이고 `A(11)`·`C(5)`·`F(4)` 는 이후 수라 **한 블록 안에서 두 시점이 섞였다.** 그 아래 커밋 표는 50 을 정확히 못 박으므로 승인 자체의 근거는 선다. 이 파일은 R1 이 금지역 하나를 닫으려고 세운 자리다 | `.palimpsest/rounds/2026-09-12-binding-radius-in-use/approval.md:36` | 회차기록 | 참 | 거짓신호 | 없음 |
| 14 | **게이트가 「회차의 마지막 상태에서 쟀다」고 적은 `passed` 1065 를 지는 산출이 없다.** `G3` 의 종료 측정 산출 `observations/final-G3.txt` 는 머리에 `HEAD 69219be` 를 적고 **1064** 를 싣는다. 그 뒤 `b1c0d4f` 가 `radius_preserves_verdict.rs` 에 시험 하나를 더했고(`git diff 69219be..HEAD -- crates/` 에 `#[test]` 한 줄) 내가 HEAD 에서 재니 **1065** 다 — **수는 참인데 그것을 지는 파일이 없다.** `G3` 자신은 어느 값으로도 통과한다(1047 이상) | `.palimpsest/rounds/2026-09-12-binding-radius-in-use/observations/final-G3.txt:12` | 회차기록 | 참 | 미관 | `G3` |
| 15 | **`H1` 의 오라클 셋째 다리가 처분 없이 남았다.** `H1` 은 가리킨 결정에 대해 ⑴ 결박문을 고쳤다 ⑵ 재결박했다 ⑶ **이슈를 열었다** 중 하나의 좌표를 요구한다. 회차는 그 일곱을 읽고 **#149**(승격 메모 입자)와 **#150**(메모를 고칠 길이 없다)을 열었는데, 게이트 §`H1` 이 **반증**을 선언하면서 *"대조불가가 아니다 · 미측정이 아니다"* 만 적고 **왜 ⑶ 이 안 걸리는지를 안 적는다.** 판정 방향이 자기에게 불리한 쪽이라 해악은 작지만, 오라클 세 다리 중 하나가 논증 없이 지나갔다 | `docs/gates/binding-radius-in-use.md:244` | 회차기록 | 참 | 미관 | `H1` |

## 내가 기각한 것

| # | 기각한 것 | 모집단 | 유효 | 해악도 | 좌표(파일:줄) | 왜 아니었나 |
|---|---|---|---|---|---|---|
| 16 | *"게이트 `## 범위 밖` 의 `37 × 78 = 2,886` 이 지금 값과 다르다(39 × 78 = 3,042)"* | 회차기록 | 거짓 | 미관 | `docs/gates/binding-radius-in-use.md:316` | 문면이 *"**넓힌 뒤** `37 × 78`"* 이고 넓힘 시점의 결박 수는 실제로 37 이었다. 시점이 붙은 측정이라 낡지 않는다. 결론(`1_000_000` 에 하중 없음)도 3,042 에서 그대로다 |
| 17 | *"`D4` 의 `grep -F` 가 문자열 `[f09.pass] ⑤` 를 한 절 안에서 못 찾는다"* | 회차기록 | 거짓 | 미관 | `docs/gates/binding-radius-in-use.md:138` | `:138`·`:139` 는 `**⑤**`·`**⑥**` 로 굵게 써서 리터럴이 안 맞지만, 같은 절(`:134-141`)에 둘과 양립 문장이 다 있다. 조건의 뜻은 「한 절에서 둘을 양립으로 논증한다」이고 그것이 섰다. 굵게 표기까지 오라클로 읽는 것은 과잉 문자주의다 |
| 18 | *"`pal radius` 가 생겨 `bind.rs:3` 의 「의도층의 유일한 입구」가 거짓이 됐다"* | 저장소 | 거짓 | 미관 | `crates/pal-cli/src/bind.rs:3` | 그 문장은 이 회차 **전에** 이미 느슨했다 — `crates/pal-cli/src/narrative.rs` 가 승격 경로로 결박을 만들고 검사 12 에 등록돼 있다. 그리고 `radius.rs:24-29` 가 *"이 명령은 입구가 아니다 — 결박의 수를 못 늘린다"* 로 그 자리를 정면으로 다룬다. 회차가 새로 깬 것이 아니다 |
| 19 | *"회차가 만든 결박 `6c6e402d958fc5e1` 이 `callers` 반경인데 호출자 0 이라 거짓 신호다"* | 저장소 | 거짓 | 미관 | `.palimpsest/intent/bindings.jsonl` (`6c6e402d958fc5e1`) | `pal touch with_radius` 를 직접 돌려 보니 화면이 *"callers 반경 · **감시 1**"* 을 나란히 찍고 아래에 *"호출자 0 · 피호출자 2"* 를 찍는다. 오독 여지가 실제로 안 열린다. 그리고 `B4` 의 모집단은 대상 37 이고 이 둘은 그 밖이다 |
| 20 | *"`G1` 의 「번호가 산출에 남는다」가 2/4 다 — #146·#147 이 게이트에도 `intent.md` 에도 없다"* | 회차기록 | 거짓 | 미관 | `docs/gates/binding-radius-in-use.md:342` | `git log -1 47a0014 --format=%B` 가 ⓐ#145 ⓑ#146 ⓒ#147 ⓓ#148 넷을 라벨과 사유와 함께 싣고, 커밋 메시지는 못 고치는 산출이다. 네 이슈가 `gh` 로 전부 OPEN 이다. 추적성 취향 차이이고 조건 위반이 아니다 |
| 21 | *"`intent.md` 의 `## 범위 밖` 에 회차 중에 세 항(#150·#151·#152)이 더해졌고 규약이 「회차 중에 새로 생기지 않는다」로 금한다"* | 규약 | 거짓 | 미관 | `.claude/skills/round/SKILL.md:798` | 그 줄은 **종료 보고(`report.md`)의 `## 범위 밖`** 에 걸리는 규율이고 잠긴 의도의 같은 이름 절이 아니다. 그리고 `report.md` 가 아직 없으므로 지금 거짓인 진술이 없다. 셋은 `## 개정` 에 날짜와 사유가 적혔고 세 축이 다 다르다는 코드 좌표가 붙어 있다 |
| 22 | *"`radius.rs` 가 `.palimpsest/radius-base-*.redb` 를 실패 경로에서 안 지운다"* | 원의도 | 거짓 | 미관 | `crates/pal-cli/src/radius.rs:274` | `.palimpsest/*.redb` 는 `.gitignore:33-34` 가 빼는 파생물이고, `ls .palimpsest/` 에 남은 것이 없다. 다음 실행이 같은 이름을 덮으므로 누적도 안 한다. 아무것도 안 깨진다 |
| 23 | *"`A5` 의 `cmp` 가 지금 정본(39 건)에 대해 안 선다"* | 회차기록 | 거짓 | 미관 | `.palimpsest/rounds/2026-09-12-binding-radius-in-use/effect/rerun-widen.txt:24` | `A5` 는 *"착수 커밋 상태에서 다시 돌려"* 를 문면으로 못 박은 **시점 고정 측정**이고, 그때의 정본은 37 건이었다. 뒤에 결박 둘이 늘어 `cmp` 가 안 서는 것은 조건이 재는 것이 아니다 |

## 끝내도 되는가

**안 된다.** 본 목록에 **금지역 1 건**(발견 10 — `D1` 의 오라클이 원리상 초록이 될 수 없는 자리를 가리키는데 통과로 적혔다. 사본에 심어 `rc=1` 로 확인했다)과 **반증 2 건이 게이트의 통과와 갈린다**(`B5` — 등록된 시험이 저장소에 0 건 · `E1` — 등록 조합 16 중 10). 그리고 `A5`·`B6-a` 는 내가 못 쟀으므로 게이트의 통과 2 건이 이 라운드에서 독립 확인을 받지 못했다.

참고로 **깨진 것은 없다** — `cargo xtask check` 28/28 · `cargo xtask test` passed 1065 failed 0 · warning 0 · `pal doctor` 위반 0 · 작업 트리 깨끗. 남은 것은 「무엇이 통과인지」의 기록 문제이고, 그것이 이 저장소가 금지역으로 등록한 축이다.