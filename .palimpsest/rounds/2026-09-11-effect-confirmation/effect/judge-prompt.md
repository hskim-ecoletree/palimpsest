# `C3` 판정자 프롬프트 — ㉡ `#126` ⟨보존본 · 판 1⟩

> 이 파일은 **판정자에게 실제로 보낸 프롬프트 전문**이다 ⟨`C3`⟩.
> 넷을 **인라인으로** 싣는다 — 경로로 넘기지 않는다.
> 사전 등록 blob: `e6f401a05f1bb1cee563c49f8becdaf988997dfc` ⟨`A1-c` 가 이 절과 blob 의
> 바이트 같음을 요구한다⟩.

═════════ 여기서부터 프롬프트 ═════════

너는 **귀속 판정자**다. 아래 넷을 읽고 물음 하나에 답한다.

## 물음

제출된 **귀속 후보**마다, 그 후보가 인용한 줄이 담은 **사실**이
**① 사전 등록 전문에도 없고 ② §5.8 표에도 없는가.** 있으면 그 귀속은 **무효**다.

- 같은 낱말이 있는지가 아니라 **같은 사실이 있는지**를 본다.
- 사실이 부분적으로만 있으면 **어느 부분이 있고 어느 부분이 없는지** 적는다.
- **네가 스스로 후보를 더하지 마라.** 제출된 것만 판정한다.

## 반환 형식

후보마다 한 절씩:

    ### <후보 기호> — 유효 | 무효
    - 인용한 줄: <touch 산출의 그 줄>
    - 담은 사실: <한 문장>
    - 사전 등록에 있나: 있다(인용) | 없다
    - §5.8 표에 있나: 있다(인용) | 없다
    - 판정 근거: <두 문장 이내>

마지막에 `## 합계` 절을 두고 `유효 N · 무효 M` 을 적는다.

## 그리고 **받은 것을 되불러 준다** — 맨 앞에 `## 받은 것` 절을 둔다

아래 넷 각각에 대해 **첫 줄 · 마지막 줄 · 줄 수**를 적는다. 옮겨 적는 것이지 요약이
아니다. **이것이 네가 읽은 바이트를 사후에 대조하는 유일한 자리다** — 부르는 쪽이 그
셋을 파일과 기계로 댄다.

    ## 받은 것
    | 절 | 첫 줄 | 마지막 줄 | 줄 수 |
    |---|---|---|---|
    | ① 사전 등록 | … | … | N |
    | ② touch 산출 | … | … | N |
    | ③ diff | … | … | N |
    | ④ §5.8 표 | … | … | N |

⚠ **저장소를 읽지 마라.** 아래 인라인된 넷만으로 판정한다. 파일을 열거나 명령을
돌리지 않는다 — 네가 읽은 바이트가 사후에 복원되는 것이 이 판정의 조건이다.

────────────────────────────────────────────────────────────────────────
## ① 사전 등록 전문 (`plan/126-pre.md`)
────────────────────────────────────────────────────────────────────────
# 사전 등록 — ㉡ `#126`

> 회차 `2026-09-11-effect-confirmation` · 재는 자리 **㉡** · 이슈 **`#126`**
> **이 파일은 `pal touch` 를 돌리기 **전에** 봉인된다.** 봉인 뒤 고치지 않는다 —
> 고치면 `A1-b`·`A1-c` 가 빨개진다.

## 0. 이 파일이 지는 조건

`A3`(좌표) · `A4`(무엇을 봤나) · `A6`(심볼 선정) · `A5-d`·`A5-e`(앵커가 서는가) ·
`B1-e`(시험 ↔ RED 짝) · `B6`(두 갈래 중 어느 것) · 그리고 `C2-b` 의 **내용 시험의 기준선**.

★ **여기 적힌 사실은 `C2` 의 귀속에서 빠진다.** 그러므로 아는 것을 빠짐없이 적는다 —
덜 적으면 나중에 귀속이 부풀고, 그것이 이 회차가 재려는 것을 망친다.

## 1. 심볼 선정 ⟨`A6`⟩

**`check_ledger_pair`** 하나. `xtask/src/main.rs:5934` 의 함수다.
`pal touch check_ledger_pair` 를 **저장소 뿌리에서 `--at` 없이** 부른다.

## 2. touch 를 돌리기 전에 무엇을 봤나 ⟨`A4`⟩

### 2.1 읽은 것
- 이슈 **`#126`** 전문 (`gh issue view 126`) — 상태 `OPEN`.
- `xtask/src/main.rs` 의 다음 자리를 직접 읽었다:
  - `:5934` `fn check_ledger_pair(root: &Path) -> Result<String>` — 검사 본체의 머리
  - `:5941-5949` 회차 목록을 `read_dir` 로 모아 **`회차들.sort()`** 로 사전순 정렬
  - `:6335-6341` `최근_끝난` — `회차들.iter().rev().find(|회차| … report.md 가 있다)`.
    **`rev()` + 사전순 정렬**이라 사전순 최대가 뽑힌다
  - `:6342-6357` 하한 판정 — 뽑힌 회차가 `검사안` 에 없으면 `problems.push("하한 미충족: …")`
    이고 그것이 `:6379-6381` 의 `bail!` 로 간다. **판정에 물린다**
  - `:6369-6376` 빈-모집단 가드 — `최근_끝난.is_some()` 과 메시지에만 쓴다. **어느 회차를
    집든 판정이 안 바뀐다**
- 이 회차 `intent.md` 의 `## 순서` 절(같은 결론을 이미 적고 있다).

### 2.2 돌린 것
- `grep -n "최근_끝난\|최근 끝난" xtask/src/main.rs` → 여섯 자리(`:6337 :6342 :6346 :6353 :6369 :6373`).
- `grep -n "회차들" xtask/src/main.rs` → 열둘. 이 검사 안의 것은 `:5941 :5945 :5948 :6026 :6047
  :6069 :6337 :6384`.
- `cargo xtask check` → `28/28`.

### 2.3 §5.8 「메인이 이미 오염됐다」 표 전문 ⟨`observations/red.md` §5.8⟩

사전 등록을 쓰기 전에 **사전부검 라운드 1 이 세 심볼에 `pal touch` 를 돌리고 그 결과를
메인에 요약해 돌려줬다.** 이 목록에 있는 것은 `C2` 의 귀속에서 뺀다.

| 심볼 | 메인이 이미 아는 것 |
|---|---|
| `nodes_of` | 결박 **0** · 지켜보는 것 **0** · 산출이 52 줄 |
| `identity_ceiling` | 결박 **0** · `pal touch` 의 **호출자 0** · `pal query symbol.callers` 가 **(없음)** · ⚠ **그 0 이 거짓 음성이고 실제 호출 자리가 넷**이라는 것(`grep` 으로 재었다). 까닭은 touch 가 스스로 적는 *"`x.foo()` 는 아직 안 셉니다"* |
| `check_ledger_pair` | 결박 **0** · 산출이 46 줄 · 같은 파일 `xtask/src/main.rs` 안에 결박 **8** 건이 있고 그중 하나가 **한 칸 옆**이다 |
| 둘 사이 | `nodes_of` 와 `check_ledger_pair` 의 산출이 **38 줄 동일**하다 |

### 2.4 이 회차가 스스로 만든 것도 적는다
같은 파일을 **이 세션이 이미 고쳤다** — `8644835` 가 `저장소가_무시하는가` 를 세우고
좌표 면제를 `git check-ignore` 에 물렸다(`:4040-4066` 부근 · 시험은 `:5700-5745` 부근).
그래서 **`intent.md ## 순서` 가 인용한 줄 번호(`:6254-6258` 등)는 이미 밀려 있다** —
지금 값은 위 2.1 이 적은 것이다. 이 사실도 touch 가 아니라 내가 만든 것이다.

## 3. 두 갈래 중 어느 것을 고르나 ⟨`B6`⟩

`#126` 이 준 갈래: *"「최근」이 판정에 안 쓰이면 그 낱말을 판정문에서 빼거나, 쓰인다면
커밋 시각으로 고친다."*

**커밋 시각 갈래를 고른다.** 근거는 위 2.1 의 `:6342-6357` — 뽑힌 회차가 `검사안` 에
없으면 `problems` 에 들어가고 `bail!` 된다. **그 낱말은 판정에 쓰인다.** 그러므로
낱말을 빼는 갈래는 문면상 성립하지 않는다.

그래서 `B6` 의 뒷문장이 걸린다 — **「커밋이 0 인 회차」의 처분을 코드와 판정문에 명시하고
그것을 재는 시험을 세운다.**

**처분: 커밋 시각을 못 얻는 회차는 「가장 최근」으로 본다.** 까닭은 fail-closed 다 —
`report.md` 가 아직 커밋 안 된 회차는 **방금 끝난 것**이고, 그것을 가장 최근으로 보면
하한이 **더 엄해진다**. 뒤로 미루면 갓 끝난 회차가 검사 밖으로 조용히 빠진다.
시각이 없는 회차가 둘 이상이면 그중 **사전순 최대**를 고른다(결정론을 지키려고).
**판정문에 「커밋 시각 없음」을 적는다** — 침묵으로 두지 않는다.

## 4. 바꿀 좌표와 각 자리에 무엇을 쓰나 ⟨`A3`⟩

| # | 좌표 | 무엇을 쓰나 |
|---|---|---|
| ⑴ | `xtask/src/main.rs:6335-6341` | `최근_끝난` 의 선택을 **`최근에_끝난(&회차들, 시각)`** 호출로 바꾼다. `회차들.iter().rev().find(…)` 를 지운다 |
| ⑵ | `xtask/src/main.rs` · `check_ledger_pair` **뒤** (새 자리) | `fn 최근에_끝난(회차들: &[String], 시각: impl Fn(&str) -> Option<i64>) -> Option<String>` 을 세운다. **끝난 회차만** 후보이고(호출자가 `report.md` 존재로 걸러 넘긴다), 시각이 큰 것을 고르고, **`None` 은 무한대로 취급**하고, 동률·`None` 다수면 사전순 최대를 고른다 |
| ⑶ | `xtask/src/main.rs` · 위 함수 옆 (새 자리) | `fn 종료_커밋_시각(root: &Path, 회차: &str) -> Option<i64>` — `git -C <root> log -1 --format=%ct -- <회차뿌리>/<회차>/report.md`. 출력이 비면 `None` |
| ⑷ | `xtask/src/main.rs:6342-6357` 의 판정문 문자열 | 「최근 끝난 회차 `X` 가 검사에 들었다」에 **무엇으로 골랐는지**를 붙인다 — 커밋 시각(`%ct`)이면 그 값, 없으면 `커밋 시각 없음`. 하한 미충족 메시지에도 같은 근거를 싣는다 |
| ⑸ | `xtask/src/main.rs` · `#[cfg(test)] mod` (새 자리) | 아래 5 의 시험 |

⚠ **판정 수·검사 수는 안 바뀐다** — `B2` 의 28 은 그대로다(검사를 더하는 것이 아니다).

## 5. 시험과 그것이 재는 RED ⟨`B1-c`·`B1-e`⟩

| 시험 이름 | 재는 RED |
|---|---|
| `최근에_끝난_것은_사전순이_아니라_커밋_시각으로_고른다` | **지금 코드가 사전순 최대를 고른다.** 사전순 최대와 커밋 시각 최대가 **갈리는** 합성 입력(`2026-01-01-b` 가 시각 100, `2026-01-01-a` 가 시각 200)을 주면 지금 코드는 `…-b` 를, 고친 코드는 `…-a` 를 낸다 |
| `커밋_시각이_없는_회차는_가장_최근으로_본다` | **커밋 0 인 회차의 처분이 코드에 없다** ⟨`B6`⟩. 시각 `None` 인 회차가 시각 있는 회차보다 뒤로 밀리면 갓 끝난 회차가 검사 밖으로 빠진다 |
| `시각이_같으면_사전순_최대를_고른다` | 동률에서 **선택이 비결정이 되는** 것. `read_dir` 순서에 기대면 기계마다 갈린다 |

★ **음성 대조** — 고친 함수를 **끄면**(사전순으로 되돌리면) 첫 시험이 빨개진다. 그것을
실제로 돌려 보고 `effect/126-delta.md` 에 적는다.

⚠ **`before`/`after` 산출이 바이트로 같을 수 있다** ⟨`PM2-13`⟩ — 지금 저장소에서 사전순
최대와 커밋 시각 최대가 **둘 다 `2026-09-08-cross-file-references`** 일 수 있다. 같으면
같다고 적고, 그때 증언하는 것은 위 시험이다. ⚠ `-after.txt` 가 **RED 로 날 수 있다**
⟨`PM3-19`⟩ — 이 회차 디렉터리에 `report.md` 도 게이트도 없어서다. **RED 로 나면 그대로
보존하고 어느 축이 발화했는지 적는다.**

## 6. 앵커 ⟨`A5-d`·`A5-e`⟩

- **touch 를 건 `check_ledger_pair` 는 변경 대상이다** — 위 ⑴⑷ 가 그 함수 안이다.
  그러므로 `A5-b` 의 `body <hash>` 는 **변경 전후로 달라야 한다.**
- ⚠ **못 서는 자리**: ⓐ 후보 목록 답이면 `fun · … body` 줄 자체가 없다 — ㉡ 은 단일
  심볼이라 해당 없을 것으로 **예상**하되, 산출을 보고 판정한다.
  ⓑ `body <hash>` 는 공백·주석 변경에 불변이다 — ⑷ 의 판정문 문자열 변경은 **본문
  변경**이라 해시가 움직일 것으로 예상한다. 안 움직이면 그대로 적는다.

## 7. 이 사전 등록이 예상하는 것 — 나중에 대조한다

- touch 산출은 **46 줄** 안팎이고 결박 **0** 이다(§5.8 에서 이미 안다).
- **새로 알게 될 것이 없을 수도 있다.** ㉡ 은 그 가능성이 가장 큰 자리다 —
  이미 코드를 직접 읽었고 §5.8 이 이 심볼에 대해 넷을 알려줬다.
  **차이 0 이 나오면 0 으로 적는다** ⟨`C5`⟩.
────────────────────────────────────────────────────────────────────────
## ② `pal touch` 산출 전문 (`touch/126.txt`)
────────────────────────────────────────────────────────────────────────
# `pal touch` 산출 — ㉡ `#126`
#
# 부른 명령      ./target/release/pal touch check_ledger_pair
#                (저장소 뿌리에서 · `--at` 없음 · `--repo` 기본값 `.`)
# HEAD           a74737b086e1426c22a4b64ad5bc0da9e6960015  ⟨봉인 커밋 = plan/126-pre.md⟩
# 심볼 ID        palimpsest@a74737b+worktree#076728deab4f  ⟨산출 둘째 줄⟩
# 사전등록-blob  e6f401a05f1bb1cee563c49f8becdaf988997dfc  ⟨git rev-parse a74737b:….../plan/126-pre.md⟩
#
# 종료값 0 · 표준오류 0 바이트 · 워킹트리 깨끗(`git status --porcelain` 빈 출력)
# 산출 46 줄 · **3346 바이트**(마커 뒤 본문 · sha256 5c9ad8d7f150ccfc…)
# ⚠ 산출 안의 `크기  약 1027 토큰 **이상** (잰 것: 4108 바이트)` 는 이 본문 바이트 수와
#   **다르다**(4108 ≠ 3346). 그 수가 무엇을 잰 것인지 산출은 말하지 않고, 이 회차는
#   그것을 안 쟀다 — 여기 적어 두고 판정하지 않는다.
--- 전 출력 (여기서부터 바이트 그대로 · 위는 머리) ---

  check_ledger_pair  ·  palimpsest@a74737b+worktree#076728deab4f
  fun · xtask/src/main.rs:5934 · identity ordinal · body 1f1846428f55

■ 이 좌표에 걸린 것 (0)
  아직 없습니다.
■ 이 좌표를 지켜보는 것 (0)
  아직 없습니다.
■ 이 심볼이 하는 것
  호출자 1 · 피호출자 12
  ※ 「호출자·피호출자」는 **참조 엣지**입니다 — 호출뿐 아니라 타입 참조·구조체 리터럴·경로 머리도 셉니다
  ※ **`x.foo()` 는 아직 안 셉니다** — 멤버 해소(`member-resolution`)는 타입 추론이 필요해 **이 회차의 범위 밖**입니다. 능력 부재가 아니라 안 하기로 정한 자리입니다
  ※ 파일 간 해소 — ⓐ `cross-file-import` 1828/5781 · ⓑ `path-resolution` 281/1070 (선 것/짝)
  ※ ⓐ 가 못 선 까닭 — ambiguous 214 · no_symbol 668 · no_symbol_at_crate_root 1932 · outside_repo 1139
  ※ ⓑ 가 못 선 까닭 — ambiguous 14 · no_symbol 154 · no_symbol_at_crate_root 338 · outside_repo 283
  ※ 못 선 몫이 가는 문 — 이 회차가 안 세우기로 정한 자리와 못 세우는 자리를 가릅니다
      ⚠ 갈래 하나는 **아직 안 갈렸습니다** — 그 자리는 문이 아니라 「미측정」을 적습니다
      `outside_repo` → **경계** — 저장소 밖(`std::*` 등)이라 원리상 못 섭니다
      `no_symbol_at_crate_root` → `A5`·`A5-a` — 재수출 경유는 잠근 축1 의 **밖**입니다
      `no_symbol` → **갈래가 아직 안 갈렸습니다** — `#133`(L2)로 갈 몫과 이 회차의 구현 여지가 섞여 있고 그 크기는 **미측정**입니다
      `no_target_file` → **이 회차의 구현** — 모듈 경로를 파일로 못 폈습니다
      `ambiguous` → **후보 생성의 모호** — 모듈 경로가 여러 파일로 읽힙니다. 하나를 고르면 조용한 오답이라 **안 고르는 것이 설계입니다**
■ 내가 모르는 것
  3 건
  · Path — outside_repo · 지난 걸음 import_item(1→1) module_path(8→0)
  · Result — no_symbol_at_crate_root · 지난 걸음 import_item(1→1) module_path(8→1) target_symbol(1→0)
  · bail — no_symbol_at_crate_root · 지난 걸음 import_item(1→1) module_path(8→1) target_symbol(1→0)
■ 효과
  (이 빌드에는 effects 능력이 없습니다 — F13 미구축)
■ 판정
  (이 빌드에는 judgment 능력이 없습니다 — F15 미구축)

■ 이 답의 근거
  Snapshot  palimpsest@a74737b+worktree
  대장      parsed 141 · partial 0 · unsupported 775 · unrecognized 341 / 1258 파일
            결박 불가 언어 7개 — 그 파일들에는 좌표가 없습니다
  2층       심볼 3308 색인됨
  워킹트리  일치
  재구축    아님
  생략      없음 (명시)
  이관      1258건 — 본체를 다른 질의로 옮겼습니다. 생략된 것이 아닙니다
            ledger 1258건 → `ledger.snapshot` 로 조회할 수 있습니다
  질의 로그  남았습니다
  크기      약 1027 토큰 **이상** (잰 것: 4108 바이트 · 가정: 4 바이트/토큰)
  능력      ledger.snapshot · symbol.resolve · symbol.contains · symbol.callers · symbol.reaches · graph.dump · binding.status · narrative.unbound · binding.touch · plan.deviation · symbol.references · 미구축 F13 · F15

────────────────────────────────────────────────────────────────────────
## ③ 실제 변경 diff (`git diff a74737b..398d233 -- xtask/src/main.rs`)
────────────────────────────────────────────────────────────────────────
diff --git a/xtask/src/main.rs b/xtask/src/main.rs
index 02950c4..145b372 100644
--- a/xtask/src/main.rs
+++ b/xtask/src/main.rs
@@ -6334,23 +6334,38 @@ fn check_ledger_pair(root: &Path) -> Result<String> {
 
     // ⑥ **하한 — 끝난 회차 중 가장 최근 것이 검사에 들었는가.**
     //    전역 개수 하한은 과거 둘로 영구 충족되어 다시 발화하지 않는다.
-    let 최근_끝난 = 회차들
+    //
+    // ★★ **「최근」은 사전순이 아니다.** ([#126] · 2026-09-11)
+    //   앞 판은 `회차들.iter().rev().find(…)` 였고 `회차들` 은 위에서 `sort()` 된다 —
+    //   그래서 뽑히는 것은 **사전순 최대**였다. 같은 날짜에 회차 둘이 서면 실제로
+    //   나중에 끝난 것이 아니라 **이름이 뒤인 것**이 뽑힌다.
+    //   ⚠ 그리고 그 선택은 **판정에 물린다** — 뽑힌 회차가 `검사안` 에 없으면 아래가
+    //   `problems` 에 싣고 이 함수 끝의 `bail!` 로 간다. 판정문의 한 줄만 틀리는 것이
+    //   아니다. 그러므로 `#126` 이 준 두 갈래 중 **커밋 시각 갈래**를 고른다.
+    let 끝난: Vec<String> = 회차들
         .iter()
-        .rev()
-        .find(|회차| 뿌리.join(회차).join("report.md").is_file())
-        .cloned();
+        .filter(|회차| 뿌리.join(회차).join("report.md").is_file())
+        .cloned()
+        .collect();
+    let 최근_끝난 = 최근에_끝난(&끝난, |회차| 종료_커밋_시각(root, 회차));
     let 하한 = match &최근_끝난 {
         None => "끝난 회차가 아직 없다".to_string(),
         Some(회차) => {
+            // **무엇으로 골랐는지 함께 적는다** — 침묵하면 사전순이었던 시절과 판정문이
+            // 구별되지 않는다. 커밋 시각이 없으면 그 사실을 적는다(아래 함수의 처분).
+            let 근거 = match 종료_커밋_시각(root, 회차) {
+                Some(t) => format!("종료 커밋 시각 {t}"),
+                None => "커밋 시각 없음 — 가장 최근으로 본다".to_string(),
+            };
             if 검사안.iter().any(|s| s.starts_with(회차.as_str())) {
-                format!("최근 끝난 회차 `{회차}` 가 검사에 들었다")
+                format!("최근 끝난 회차 `{회차}` 가 검사에 들었다 ({근거})")
             } else {
                 problems.push(format!(
-                    "하한 미충족: 끝난 회차 중 가장 최근인 `{회차}` 가 이 검사 밖이다 — \
-                     표준 표를 세우거나 게이트를 짝지어야 한다. \
+                    "하한 미충족: 끝난 회차 중 가장 최근인 `{회차}` ({근거}) 가 이 검사 \
+                     밖이다 — 표준 표를 세우거나 게이트를 짝지어야 한다. \
                      전역 개수 하한은 과거로 영구 충족되므로 하한을 여기 건다"
                 ));
-                format!("최근 끝난 회차 `{회차}` 가 검사 밖이다")
+                format!("최근 끝난 회차 `{회차}` 가 검사 밖이다 ({근거})")
             }
         }
     };
@@ -6389,6 +6404,92 @@ fn check_ledger_pair(root: &Path) -> Result<String> {
     ))
 }
 
+/// 끝난 회차 중 **가장 최근**인 것 — 「최근」의 자는 **커밋 시각**이다. ([#126])
+///
+/// ★ **왜 사전순이 아닌가.** 회차 이름은 `YYYY-MM-DD-<슬러그>` 라 날짜까지는 사전순이
+/// 시간순과 같지만, **같은 날짜에 둘이 서면 슬러그가 순서를 정한다.** 그것은 시간이
+/// 아니다. `#126` 의 문장: *"사전순은 「최근」의 자가 아니다."*
+///
+/// ★ **시각을 못 얻는 회차는 「가장 최근」으로 본다** — fail-closed 다. `report.md` 가
+/// 아직 커밋되지 않은 회차는 **방금 끝난 것**이고, 뒤로 밀면 갓 끝난 회차가 하한의
+/// 대상에서 조용히 빠진다. 시각 없는 회차가 여럿이면 그중 **사전순 최대**를 고른다 —
+/// `read_dir` 순서에 기대면 기계마다 갈린다(ADR-0023).
+///
+/// ⚠ **후보를 여기서 거르지 않는다.** 「끝났나」의 자는 호출자가 진다(`report.md` 의
+/// 존재). 그 자를 두 곳에 적으면 갈린다.
+///
+/// [#126]: https://github.com/hskim-ecoletree/palimpsest/issues/126
+fn 최근에_끝난(끝난: &[String], 시각: impl Fn(&str) -> Option<i64>) -> Option<String> {
+    끝난
+        .iter()
+        .max_by_key(|회차| (시각(회차).unwrap_or(i64::MAX), (*회차).clone()))
+        .cloned()
+}
+
+/// 그 회차가 **끝난 시각** — `report.md` 를 마지막으로 만진 커밋의 커밋 시각(`%ct`).
+///
+/// 커밋이 없으면(추적 안 됨·방금 씀) `None` 이고, 그 처분은 [`최근에_끝난`] 이 진다.
+/// ⚠ **작성 시각(`%at`)이 아니라 커밋 시각(`%ct`)이다** — rebase·cherry-pick 이 작성
+/// 시각을 옛 값으로 들고 다니므로 그것으로 「최근」을 재면 이력을 고칠 때마다 뒤집힌다.
+fn 종료_커밋_시각(root: &Path, 회차: &str) -> Option<i64> {
+    let 경로 = format!("{회차_뿌리}/{회차}/report.md");
+    let out = std::process::Command::new("git")
+        .arg("-C")
+        .arg(root)
+        .args(["log", "-1", "--format=%ct", "--"])
+        .arg(&경로)
+        .output()
+        .ok()?;
+    if !out.status.success() {
+        return None;
+    }
+    String::from_utf8_lossy(&out.stdout).trim().parse::<i64>().ok()
+}
+
+#[cfg(test)]
+mod 최근_끝난_시험 {
+    use super::최근에_끝난;
+
+    fn 회차들(이름: &[&str]) -> Vec<String> {
+        이름.iter().map(|s| (*s).to_string()).collect()
+    }
+
+    /// ★ **RED 는 이것이었다** — 앞 판은 사전순 최대를 골랐다. 같은 날짜에 회차 둘이
+    /// 서고 **사전순이 시간순과 갈리면** 틀린 회차가 뽑힌다.
+    #[test]
+    fn 최근에_끝난_것은_사전순이_아니라_커밋_시각으로_고른다() {
+        let 목록 = 회차들(&["2026-01-01-a", "2026-01-01-b"]);
+        // `…-a` 가 **나중에** 끝났다. 사전순 최대는 `…-b` 다.
+        let 시각 = |회차: &str| match 회차 {
+            "2026-01-01-a" => Some(200),
+            "2026-01-01-b" => Some(100),
+            _ => None,
+        };
+        assert_eq!(최근에_끝난(&목록, 시각).as_deref(), Some("2026-01-01-a"));
+        // 음성 대조 — 사전순으로 고르면 이 값이 나온다. 그것이 앞 판의 답이다.
+        assert_ne!(목록.iter().max().map(String::as_str), Some("2026-01-01-a"));
+    }
+
+    /// `B6` 이 요구하는 **「커밋이 0 인 회차」의 처분**이다 — 가장 최근으로 본다.
+    #[test]
+    fn 커밋_시각이_없는_회차는_가장_최근으로_본다() {
+        let 목록 = 회차들(&["2026-01-01-a", "2026-09-09-z"]);
+        // `…-a` 는 커밋이 없다(방금 끝났다). `…-z` 는 시각이 있다.
+        let 시각 = |회차: &str| if 회차 == "2026-09-09-z" { Some(1_800_000_000) } else { None };
+        assert_eq!(최근에_끝난(&목록, 시각).as_deref(), Some("2026-01-01-a"));
+    }
+
+    /// 시각이 같으면(또는 둘 다 없으면) **사전순 최대**다 — 기계마다 갈리지 않게.
+    #[test]
+    fn 시각이_같으면_사전순_최대를_고른다() {
+        let 목록 = 회차들(&["2026-01-01-a", "2026-01-01-b", "2026-01-01-c"]);
+        assert_eq!(최근에_끝난(&목록, |_| Some(7)).as_deref(), Some("2026-01-01-c"));
+        assert_eq!(최근에_끝난(&목록, |_| None).as_deref(), Some("2026-01-01-c"));
+        // 끝난 회차가 없으면 `None` — 하한을 안 건다.
+        assert_eq!(최근에_끝난(&[], |_| Some(1)), None);
+    }
+}
+
 /// **원장 enum 이 두 자리에 적혀 있다 — 그 둘이 같은지 잰다.**
 ///
 /// 축 아홉(`출처`·`모집단`·`유효성`·`해악도`·`처분`·`승격됨`·`조건변경`·`사전처분`·`상태`)의
────────────────────────────────────────────────────────────────────────
## ④ §5.8 「메인이 이미 오염됐다」 표 전문 (`observations/red.md`)
────────────────────────────────────────────────────────────────────────
### 5.8 ⚠ 메인이 이미 오염됐다 — 사전부검이 `pal touch` 를 먼저 돌렸다

**사전 등록을 쓰기 전에** 사전부검 라운드 1 이 세 심볼에 `pal touch` 를 돌리고 **그 결과를
메인에 요약해 돌려줬다.** 숨기지 않고 여기 전수로 적는다. 이 목록에 있는 것은 **`C2` 의
귀속에서 뺀다** — touch 가 아니라 사전부검이 말해 준 것이기 때문이다.

| 심볼 | 메인이 이미 아는 것 |
|---|---|
| `nodes_of` | 결박 **0** · 지켜보는 것 **0** · 산출이 52 줄 |
| `identity_ceiling` | 결박 **0** · `pal touch` 의 **호출자 0** · `pal query symbol.callers` 가 **(없음)** · ⚠ **그 0 이 거짓 음성이고 실제 호출 자리가 넷**이라는 것(`grep` 으로 재었다). 까닭은 touch 가 스스로 적는 *"`x.foo()` 는 아직 안 셉니다"* |
| `check_ledger_pair` | 결박 **0** · 산출이 46 줄 · 같은 파일 `xtask/src/main.rs` 안에 결박 **8** 건이 있고 그중 하나가 **한 칸 옆**이다 |
| 둘 사이 | `nodes_of` 와 `check_ledger_pair` 의 산출이 **38 줄 동일**하다 |

★ **이것은 회차를 무르게 하지 않는다 — 기록으로 세운다.** 인터뷰 3 이 잠근 답이
*"평소대로 조사하고, **무엇을 봤는지 기록한다**"* 이고, 사전부검은 이 회차의 평소 절차다.
**그러나 ㉠ 의 값은 실제로 깎였다** — `identity_ceiling` 의 거짓 음성을 touch 가 아니라
사전부검이 먼저 말했다.
────────────────────────────────────────────────────────────────────────
## 제출된 귀속 후보
────────────────────────────────────────────────────────────────────────

### ⓐ
- 인용한 줄: `  호출자 1 · 피호출자 12`  (산출의 `:24`)
- 주장하는 사실: `check_ledger_pair` 의 호출자가 **1** 이다.

### ⓑ
- 인용한 줄: `  fun · xtask/src/main.rs:5934 · identity ordinal · body 1f1846428f55`  (`:17`)
- 주장하는 사실: 이 심볼의 **정체성 표시가 `ordinal`** 이다.

### ⓒ
- 인용한 줄: `  fun · xtask/src/main.rs:5934 · identity ordinal · body 1f1846428f55`  (`:17`)
- 주장하는 사실: 본문 앵커가 **`body 1f1846428f55`** 다.

### ⓓ
- 인용한 줄: `  fun · xtask/src/main.rs:5934 · identity ordinal · body 1f1846428f55`  (`:17`)
- 주장하는 사실: `check_ledger_pair` 의 **선언 자리가 `xtask/src/main.rs:5934`** 다.

### ⓔ
- 인용한 줄: `■ 이 좌표에 걸린 것 (0)` / `  아직 없습니다.`  (`:19-20`)
- 주장하는 사실: 이 심볼에 걸린 **결박이 0** 이다.

═════════ 프롬프트 끝 ═════════


════════════════════════════════════════════════════════════════════════
# `C3` 판정자 프롬프트 — ㉠ `#79` ⟨보존본 · 판 2⟩
════════════════════════════════════════════════════════════════════════

> 사전 등록 blob: `9b0f3a5a87f60f02e43daf9e2fb49455a5c31466`.
> 물음·반환 형식·되불러 주기는 **판 1 과 같은 문면**이고 인라인된 넷만 다르다.

═════════ 여기서부터 프롬프트 ═════════

너는 **귀속 판정자**다. 아래 넷을 읽고 물음 하나에 답한다.

## 물음

제출된 **귀속 후보**마다, 그 후보가 인용한 줄이 담은 **사실**이
**① 사전 등록 전문에도 없고 ② §5.8 표에도 없는가.** 있으면 그 귀속은 **무효**다.

- 같은 낱말이 있는지가 아니라 **같은 사실이 있는지**를 본다.
- 사실이 부분적으로만 있으면 **어느 부분이 있고 어느 부분이 없는지** 적는다.
- **네가 스스로 후보를 더하지 마라.** 제출된 것만 판정한다.

## 반환 형식

후보마다 한 절씩:

    ### <후보 기호> — 유효 | 무효
    - 인용한 줄: <touch 산출의 그 줄>
    - 담은 사실: <한 문장>
    - 사전 등록에 있나: 있다(인용) | 없다
    - §5.8 표에 있나: 있다(인용) | 없다
    - 판정 근거: <두 문장 이내>

마지막에 `## 합계` 절을 두고 `유효 N · 무효 M` 을 적는다.

## 그리고 **받은 것을 되불러 준다** — 맨 앞에 `## 받은 것` 절을 둔다

아래 넷 각각에 대해 **첫 줄 · 마지막 줄 · 줄 수**를 적는다. 옮겨 적는 것이지 요약이
아니다. **이것이 네가 읽은 바이트를 사후에 대조하는 유일한 자리다** — 부르는 쪽이 그
셋을 파일과 기계로 댄다.

    ## 받은 것
    | 절 | 첫 줄 | 마지막 줄 | 줄 수 |
    |---|---|---|---|
    | ① 사전 등록 | … | … | N |
    | ② touch 산출 | … | … | N |
    | ③ diff | … | … | N |
    | ④ §5.8 표 | … | … | N |

⚠ **저장소를 읽지 마라.** 아래 인라인된 넷만으로 판정한다. 파일을 열거나 명령을
돌리지 않는다 — 네가 읽은 바이트가 사후에 복원되는 것이 이 판정의 조건이다.

────────────────────────────────────────────────────────────────────────
## ① 사전 등록 전문 (`plan/79-pre.md`)
────────────────────────────────────────────────────────────────────────
# 사전 등록 — ㉠ `#79`

> 회차 `2026-09-11-effect-confirmation` · 재는 자리 **㉠** · 이슈 **`#79`**
> **이 파일은 `pal touch` 를 돌리기 전에 봉인된다.** 봉인 뒤 고치지 않는다.

## 0. 이 파일이 지는 조건

`A3`(좌표) · `A4`(무엇을 봤나) · `A6`(심볼 선정) · `A5-d`·`A5-e`(앵커) · `B1-e`(시험 ↔ RED) ·
★ **`B5-a`(모집단 정의 — 무엇을 분모로 삼나)** · 그리고 `C2-b` 의 내용 시험 기준선.

★ **여기 적힌 사실은 `C2` 의 귀속에서 빠진다.** 빠짐없이 적는다.

## 1. 심볼 선정 ⟨`A6`⟩

**`nodes_of`** 하나. `crates/pal-cli/src/ledger.rs:301` 의 함수이고 `#79` 가 이름으로
지목한 자리다(`ledger.rs:334` 가 그 안에 있다).
`pal touch nodes_of` 를 저장소 뿌리에서 `--at` 없이 부른다.

## 2. touch 를 돌리기 전에 무엇을 봤나 ⟨`A4`⟩

### 2.1 읽은 것
- 이슈 **`#79`** 전문 — 상태 `OPEN`. 본문이 적은 것: `nodes_of` 가
  `discriminator.identity_ceiling().min(s.identity)` 만 남기고 **ceiling 자체를 버린다** ·
  「순서에 취약한 464 건」과 「그냥 L1 이라 ordinal 인 7,156 건」이 대장에서 같은 글자가 된다 ·
  *"닫으려면 그 수를 내는 산출 경로가 필요하다."*
- `crates/pal-cli/src/ledger.rs`
  - `:39-63` `LedgerReport` — `symbols: Vec<SymbolNode>` 가 **`#[serde(skip)]`** 이고
    *"표에는 안 나오고 `pal touch` 가 쓴다"* 라 적혀 있다. **대장은 이미 심볼 전량을 손에
    쥐고 있다.**
  - `:301-340` `nodes_of` — `seen` 맵이 `(체인, 이름, 종류)` 마다 슬롯을 세어
    `Discriminator::new(s.kind, *slot)` 를 만들고, `:334` 에서 `min` 으로 **두 상한을 하나로
    접는다.**
  - `:212` 대장 계산이 파일마다 `nodes_of` 를 부르는 자리.
  - `:476-522` `language_capabilities` · `:557-` `print_table` — 화면이 **언어 단위** 등급을
    적는다(`Rust L1 ordinal · 구조 · 선언 순서`). **심볼 단위는 화면에 없다.**
- `crates/pal-core/src/coord.rs:235-266` — `Discriminator.ordinal` 은 *"같은 (이름, 종류)가
  여럿일 때의 선언 순서. **유일하면 0**"* 이고 `identity_ceiling()` 은 `ordinal == 0` 이면
  `Exact`, 아니면 `Ordinal` 이다. 주석이 *"`ordinal` 이 실린 것 자체가 위험의 표시다"* 라 적는다.
- `crates/pal-core/src/touch.rs:35-57` `SymbolNode` — 필드 일곱(`id` `path` `container`
  `name` `kind` `body` `span` `identity`). **`ordinal` 도 `ceiling` 도 없다** — 그것이
  `#79` 가 말하는 버림이다.
- `schema/graph.toml:44-67` `[node.Symbol]` — 속성이 `container` `name` `kind` `body` `span`
  `identity` 로 선언돼 있다. `xtask` 의 「스키마 정합」이 **스키마 속성 이름 ↔ Rust `pub`
  필드**를 대조하므로, 필드를 더하면 **스키마도 같이 고쳐야 한다.**
- `crates/pal-store/src/projection.rs:122-165` — 저장이 **postcard(자리 기반)** 라 옛 색인을
  새 바이너리로 읽으면 조용히 어긋난다. 그래서 `ROW_FORMAT_REV`(`:137`) 와 `META_FORMAT`
  도장이 있고, 안 맞으면 `RowFormat` 오류로 *"구제는 다시 세우는 것 하나다"* 를 낸다.
- `crates/pal-cli/src/main.rs:665-674` — `pal ledger` 의 `--json` 은 **`LedgerReport` 를
  그대로** 직렬화한다. 표와 JSON 이 같은 구조에서 나온다.
- `pal ledger` 를 실제로 돌려 화면을 봤다 — 파일 1264 · parsed 141 · 언어 표에 `Rust L1
  ordinal 140 파일` · `TypeScript L2 exact 1 파일` · *"결박 불가 언어 7개 · 777 파일.
  이 파일들에는 좌표가 없습니다"*.

### 2.2 돌린 것
- `gh issue view 79`
- `grep -rn "nodes_of"` → 호출 자리 넷(`defect.rs:322` · `ledger.rs:212` · 시험 둘)
- `grep -rn "SymbolNode {"` → **구조체 리터럴 13 자리**(대부분 시험·예제)
- `grep -o 'type = "[^"]*"' schema/graph.toml | sort | uniq -c` → 스키마 타입 어휘에
  **정수 타입이 없다**
- `./target/release/pal ledger` · `pal ledger --help` · `pal doctor --help`

### 2.3 §5.8 「메인이 이미 오염됐다」 표 전문 ⟨`observations/red.md` §5.8⟩

| 심볼 | 메인이 이미 아는 것 |
|---|---|
| `nodes_of` | 결박 **0** · 지켜보는 것 **0** · 산출이 52 줄 |
| `identity_ceiling` | 결박 **0** · `pal touch` 의 **호출자 0** · `pal query symbol.callers` 가 **(없음)** · ⚠ **그 0 이 거짓 음성이고 실제 호출 자리가 넷**이라는 것(`grep` 으로 재었다). 까닭은 touch 가 스스로 적는 *"`x.foo()` 는 아직 안 셉니다"* |
| `check_ledger_pair` | 결박 **0** · 산출이 46 줄 · 같은 파일 `xtask/src/main.rs` 안에 결박 **8** 건이 있고 그중 하나가 **한 칸 옆**이다 |
| 둘 사이 | `nodes_of` 와 `check_ledger_pair` 의 산출이 **38 줄 동일**하다 |

### 2.4 ㉡ 의 산출이 이미 말해 준 것 — **이것도 귀속에서 뺀다**

`touch/126.txt:17` 이 `check_ledger_pair` 에 대해 `identity ordinal` 을 찍었고, 그것을 따라가
`ledger.rs:334` 의 `min` 에 닿았다 ⟨`MS-07`⟩. **그 심볼은 판별자 상한이 `Exact`**(선언이
파일에 하나)인데 표시는 `ordinal` 이다 — 원인은 추출기 등급이다. 이 회차의 ㉡ 이 이미
산출했으므로 **㉠ 의 `C2` 귀속 후보가 아니다.**

## 3. 무엇을 고치나 — 두 갈래 중 고른 것

| 갈래 | 무엇 | 고르나 |
|---|---|---|
| ⓐ **저장한다** | `SymbolNode` 에 `ordinal`(또는 ceiling)을 싣고 스키마·`ROW_FORMAT_REV`·색인 재구축·리터럴 13 자리를 고친다. `pal touch` 가 심볼마다 원인을 말할 수 있게 된다 | **아니다** |
| ⓑ **버리지 않고 세어 올린다** | `nodes_of` 가 판별자 상한을 **버리지 않고** 대장으로 올린다. `pal ledger` 가 심볼 단위 정체성을 **넷으로 갈라** 화면과 `--json` 에 싣는다 | **고른다** |

**까닭.** `#79` 의 닫는 조건은 *"그 수를 내는 산출 경로"* 이고 `B5` 도 *"가르는 수가 명령으로
난다"* 다. ⓑ 가 그것을 만족하면서 **저장 형식·색인·리터럴 13 자리를 안 건드린다** — 회차의
기조가 *"목적을 달성하는 최소한의 수정"* 이다.

⚠ **ⓑ 가 남기는 것을 미리 적는다** — `pal touch` 의 심볼 한 줄은 **여전히 `ordinal` 만**
찍고 원인을 안 가른다. 그것은 ⓐ 없이는 원리상 못 고친다(2층에 그 값이 없다).
**이 회차는 그것을 `## 범위 밖` 으로 처분한다** — `A5-c`(워킹트리 일치)·`MS-06`(크기 줄)과
같은 자리다: **`pal` 의 표시는 관측만 하고 안 고친다.** 새 발견으로 원장에 싣는다.

## 4. 모집단 — 분모가 무엇인가 ★⟨`B5-a`⟩

**분모 = 그 실행에서 2층에 들어가는 심볼 전량**이다. `pal ledger` 가 같은 실행에서
`LedgerReport.symbols` 로 쥐는 것이고, 화면에 **`합 N`** 으로 함께 적는다.

- **`#79` 본문의 「464 · 7,156」은 안 쓴다.** 합이 7,620 인데 지금 저장소의 심볼 수와
  자릿수가 다르다(`pal touch` 의 근거 줄이 `2층 심볼 3308 색인됨` 이라 적었다).
  그 수는 **다른 시점·다른 코퍼스**의 것이다.
- 넷으로 가른다. **합이 분모와 같아야 한다** — 화면에 검산을 적는다.

| 버킷 | 정의 | 뜻 |
|---|---|---|
| ① **순서에 취약** | `ordinal > 0` | 같은 (체인·이름·종류)가 여럿이라 **선언 순서로 가렸다.** 순서가 바뀌면 정체성이 맞바뀐다 |
| ② **등급이 낮음** | `ordinal == 0` 이고 추출기 등급이 `ordinal` | 순서 위험은 **없다.** 추출기가 스코프를 못 풀어서 `ordinal` 이다 |
| ③ **정확** | `ordinal == 0` 이고 추출기 등급이 `exact` | |
| ④ **불가** | `ordinal == 0` 이고 추출기 등급이 `unavailable` | |

⚠ **①이 ④를 가릴 수 있다** — `ordinal > 0` 인데 언어가 L0 이면 최종 등급은 `unavailable`
이다. 다만 **L0 언어는 심볼을 안 낸다**(`pal ledger` 가 *"이 파일들에는 좌표가 없습니다"*
라 적는다). 그러므로 ④는 **0 일 것으로 예상**하고, 0 이 아니면 그 사실을 적는다.
⚠ **예상을 적지만 값은 명령이 낸다** — 손으로 센 수를 판정 표에 싣지 않는다(`#79` 의 요구).

## 5. 바꿀 좌표와 각 자리에 무엇을 쓰나 ⟨`A3`⟩

| # | 좌표 | 무엇을 쓰나 |
|---|---|---|
| ⑴ | `crates/pal-cli/src/ledger.rs` · `nodes_of` 앞 (새 자리) | `pub struct IdentityTally { 취약, 등급낮음, 정확, 불가 }`(전부 `usize`) 와 `fn 합(&self) -> usize` · `fn 더한다(&mut self, ceiling, grade)`. **가르는 규칙이 한 자리에만 있다** |
| ⑵ | `crates/pal-cli/src/ledger.rs:301-340` `nodes_of` | 반환을 `Vec<SymbolNode>` → **`(Vec<SymbolNode>, IdentityTally)`** 로 바꾼다. `:334` 의 `min` 은 **그대로 둔다**(소비자가 쓰는 값이다) — 버리지 않고 `identity_ceiling()` 과 `s.identity` 를 tally 로 올린다 |
| ⑶ | `crates/pal-cli/src/ledger.rs:212` · `crates/pal-cli/src/defect.rs:322` · 시험 둘(`:718` `:824-825`) | 호출자 넷을 새 반환에 맞춘다. 시험은 `.0` 만 쓴다 |
| ⑷ | `crates/pal-cli/src/ledger.rs` · `LedgerReport` | 필드 `pub identity: IdentityTally` 를 더한다. **`#[serde(skip)]` 을 안 붙인다** — `--json` 이 같은 구조를 직렬화하므로 이것으로 `B5` 의 「명령으로 난다」가 선다 |
| ⑸ | `crates/pal-cli/src/ledger.rs` · `print_table` | 「정체성」 블록을 더한다 — 넷과 **합(=분모)**, 그리고 ①②가 **같은 글자였다**는 사실을 한 줄로 |
| ⑹ | `crates/pal-cli/src/ledger.rs` · 시험 (새 자리) | 아래 6 의 시험 |

⚠ **안 건드리는 것**: `schema/graph.toml` · `ROW_FORMAT_REV` · `SymbolNode` · 리터럴 13 자리 ·
색인 재구축. ⓑ 를 고른 값이 그것이다.

## 6. 시험과 그것이 재는 RED ⟨`B1-c`·`B1-e`⟩

| 시험 이름 | 재는 RED |
|---|---|
| `순서로_가린_것과_등급이_낮은_것이_갈린다` | **지금은 둘이 같은 글자다.** 같은 파일에 같은 이름·종류 선언 둘 + 다른 이름 하나를 합성해 넣으면, 지금 코드는 세 심볼 전부 `identity == ordinal` 을 내고 **가를 수가 없다.** 고친 뒤에는 tally 가 `취약 1 · 등급낮음 2`(또는 그 파일의 실제 등급)로 갈라야 한다 |
| `합이_분모와_같다` | 넷의 합이 심볼 수와 다르면 **버킷이 겹치거나 빠진다.** 손으로 센 수가 판정 표에 실리는 길이 그렇게 열린다 |
| `check_ledger_pair_는_순서에_취약하지_않다` | ⟨`MS-07` 을 닫는 시험⟩ 그 심볼은 판별자 상한이 `Exact` 인데 표시가 `ordinal` 이다. tally 는 그것을 **②(등급이 낮음)** 로 세어야 하고 ①이 아니어야 한다. **㉡ 의 산출이 준 실물 입력이다** |

★ **음성 대조** — tally 의 가르는 자리를 **끄면**(`취약` 도 `등급낮음` 으로 세면) 첫 시험과
셋째 시험이 빨개진다. 실제로 돌려 `effect/79-delta.md` 에 적는다.

## 7. 앵커 ⟨`A5-d`·`A5-e`⟩

- **touch 를 건 `nodes_of` 는 변경 대상이다** — ⑵ 가 그 함수 본문이다. 그러므로 `A5-b` 의
  `body <hash>` 는 **변경 전후로 달라야 한다.**
- ⚠ 못 서는 자리: ⓐ 답이 후보 목록이면 `fun · … body` 줄이 없다 — `nodes_of` 는 단일
  선언이라 해당 없을 것으로 예상하되 산출을 보고 판정한다. ⓑ `body <hash>` 는 공백·주석
  변경에 불변 — ⑵ 는 **반환 타입과 본문**을 바꾸므로 움직일 것으로 예상한다.

## 8. 이 사전 등록이 예상하는 것 — 나중에 대조한다

- touch 산출은 **52 줄** 안팎이고 결박 **0** 이다(§5.8).
- `nodes_of` 의 호출자는 `grep` 으로 이미 넷을 알고 있다. **touch 가 그보다 더 말해 줄 것이
  있는지가 이 자리의 물음이다** — ㉡ 에서는 「호출자 1」이 값이었는데 여기서는 내가 먼저 셌다.
- 새로 알게 될 것이 없으면 **차이 0 으로 적는다** ⟨`C5`⟩.
────────────────────────────────────────────────────────────────────────
## ② `pal touch` 산출 전문 (`touch/79.txt`)
────────────────────────────────────────────────────────────────────────
# `pal touch` 산출 — ㉠ `#79`
#
# 부른 명령      ./target/release/pal touch nodes_of
#                (저장소 뿌리에서 · `--at` 없음 · `--repo` 기본값 `.`)
# HEAD           77dc97774ff87d0589ee77af821fa471ae517d02  ⟨봉인 커밋 = plan/79-pre.md⟩
# 심볼 ID        palimpsest@77dc977+worktree#a9dd338ebdd7  ⟨산출 둘째 줄⟩
# 사전등록-blob  9b0f3a5a87f60f02e43daf9e2fb49455a5c31466  ⟨git rev-parse 77dc977:….../plan/79-pre.md⟩
#
# 종료값 0 · 표준오류 0 바이트 · 워킹트리 깨끗 · 산출 52 줄 · **4066 바이트**
# 본문 sha256   1fdf18fb3c0730e0ec2ddd6ff8c524a0c8a46663d8ec28c24aeee95880bd9939
# ⚠ 산출 안의 `크기 … 잰 것: 7749 바이트` 는 이 본문 바이트 수와 다르다(7749 ≠ 4066).
#   ㉡ 에서 잡은 `MS-06` 의 **둘째 관측**이다 — 방향이 같다(잰 값 > 화면).
--- 전 출력 (여기서부터 바이트 그대로 · 위는 머리) ---

  nodes_of  ·  palimpsest@77dc977+worktree#a9dd338ebdd7
  fun · crates/pal-cli/src/ledger.rs:301 · identity ordinal · body 7058bb9c2fcf

■ 이 좌표에 걸린 것 (0)
  아직 없습니다.
■ 이 좌표를 지켜보는 것 (0)
  아직 없습니다.
■ 이 심볼이 하는 것
  호출자 3 · 피호출자 1
  ※ 「호출자·피호출자」는 **참조 엣지**입니다 — 호출뿐 아니라 타입 참조·구조체 리터럴·경로 머리도 셉니다
  ※ **`x.foo()` 는 아직 안 셉니다** — 멤버 해소(`member-resolution`)는 타입 추론이 필요해 **이 회차의 범위 밖**입니다. 능력 부재가 아니라 안 하기로 정한 자리입니다
  ※ 파일 간 해소 — ⓐ `cross-file-import` 1833/5787 · ⓑ `path-resolution` 281/1070 (선 것/짝)
  ※ ⓐ 가 못 선 까닭 — ambiguous 214 · no_symbol 668 · no_symbol_at_crate_root 1932 · outside_repo 1140
  ※ ⓑ 가 못 선 까닭 — ambiguous 14 · no_symbol 154 · no_symbol_at_crate_root 338 · outside_repo 283
  ※ 못 선 몫이 가는 문 — 이 회차가 안 세우기로 정한 자리와 못 세우는 자리를 가릅니다
      ⚠ 갈래 하나는 **아직 안 갈렸습니다** — 그 자리는 문이 아니라 「미측정」을 적습니다
      `outside_repo` → **경계** — 저장소 밖(`std::*` 등)이라 원리상 못 섭니다
      `no_symbol_at_crate_root` → `A5`·`A5-a` — 재수출 경유는 잠근 축1 의 **밖**입니다
      `no_symbol` → **갈래가 아직 안 갈렸습니다** — `#133`(L2)로 갈 몫과 이 회차의 구현 여지가 섞여 있고 그 크기는 **미측정**입니다
      `no_target_file` → **이 회차의 구현** — 모듈 경로를 파일로 못 폈습니다
      `ambiguous` → **후보 생성의 모호** — 모듈 경로가 여러 파일로 읽힙니다. 하나를 고르면 조용한 오답이라 **안 고르는 것이 설계입니다**
■ 내가 모르는 것
  9 건
  · RepoId — no_symbol_at_crate_root · 지난 걸음 import_item(1→1) module_path(4→1) target_symbol(1→0)
  · RepoPath — no_symbol_at_crate_root · 지난 걸음 import_item(1→1) module_path(4→1) target_symbol(1→0)
  · Containment — no_symbol_at_crate_root · 지난 걸음 import_item(1→1) module_path(4→1) target_symbol(1→0)
  · SymbolNode — no_symbol_at_crate_root · 지난 걸음 import_item(1→1) module_path(4→1) target_symbol(1→0)
  · BTreeMap — outside_repo · 지난 걸음 import_item(1→1) module_path(8→0)
  · Discriminator — no_symbol_at_crate_root · 지난 걸음 import_item(1→1) module_path(4→1) target_symbol(1→0)
  · new — no_symbol_at_crate_root · 지난 걸음 import_item(1→1) module_path(4→1) target_symbol(1→0)
  · SymbolId — no_symbol_at_crate_root · 지난 걸음 import_item(1→1) module_path(4→1) target_symbol(1→0)
  · compute — no_symbol_at_crate_root · 지난 걸음 import_item(1→1) module_path(4→1) target_symbol(1→0)
■ 효과
  (이 빌드에는 effects 능력이 없습니다 — F13 미구축)
■ 판정
  (이 빌드에는 judgment 능력이 없습니다 — F15 미구축)

■ 이 답의 근거
  Snapshot  palimpsest@77dc977+worktree
  대장      parsed 141 · partial 0 · unsupported 778 · unrecognized 345 / 1265 파일
            결박 불가 언어 7개 — 그 파일들에는 좌표가 없습니다
  2층       심볼 3315 색인됨
  워킹트리  일치
  재구축    아님
  생략      없음 (명시)
  이관      1265건 — 본체를 다른 질의로 옮겼습니다. 생략된 것이 아닙니다
            ledger 1265건 → `ledger.snapshot` 로 조회할 수 있습니다
  질의 로그  남았습니다
  크기      약 1937 토큰 **이상** (잰 것: 7749 바이트 · 가정: 4 바이트/토큰)
  능력      ledger.snapshot · symbol.resolve · symbol.contains · symbol.callers · symbol.reaches · graph.dump · binding.status · narrative.unbound · binding.touch · plan.deviation · symbol.references · 미구축 F13 · F15

────────────────────────────────────────────────────────────────────────
## ③ 실제 변경 diff (`git diff 77dc977..9f993cc -- crates/`)
────────────────────────────────────────────────────────────────────────
diff --git a/crates/pal-cli/src/defect.rs b/crates/pal-cli/src/defect.rs
index 6909d8a..5a945ac 100644
--- a/crates/pal-cli/src/defect.rs
+++ b/crates/pal-cli/src/defect.rs
@@ -319,7 +319,11 @@ fn digests_at(
         return Ok(BTreeMap::new());
     };
     let Ok(graph) = extractor.extract(&source) else { return Ok(BTreeMap::new()) };
+    // ⚠ **`.0` 이다** — `nodes_of` 는 정체성 tally 를 함께 돌려준다([#79]). 결함 계보는
+    //   좌표만 쓰므로 tally 를 버린다. **버리는 것을 여기 적는다** — 조용히 버리면 다음에
+    //   이 자리를 읽는 사람이 대장의 수와 이 경로의 수가 왜 다른지 못 짚는다.
     Ok(ledger::nodes_of(repo, path, &graph.symbols, &graph.contains)
+        .0
         .into_iter()
         .map(|n| (n.id, n.body))
         .collect())
diff --git a/crates/pal-cli/src/ledger.rs b/crates/pal-cli/src/ledger.rs
index 558dc69..d82be4c 100644
--- a/crates/pal-cli/src/ledger.rs
+++ b/crates/pal-cli/src/ledger.rs
@@ -16,7 +16,8 @@ use std::path::{Path, PathBuf};
 use anyhow::{Context, Result};
 use pal_core::{
     Attributes, Bucket, CORRUPT_NOTES, Containment, DetectorFreshness, Discriminator,
-    EXTRACT_CHUNK, ExtractGrade, FileRow, FileState, OVERSIZE_BYTES, RefCounts, ReferenceEdge,
+    EXTRACT_CHUNK, ExtractGrade, FileRow, FileState, IdentityGrade, OVERSIZE_BYTES, RefCounts,
+    ReferenceEdge,
     Slot,
     LanguageCapability, LanguageId, Ledger, LedgerEntry, Manifest, RepoId, RepoPath,
     ScopeSource, Snapshot, SymbolId, SymbolNode, TreeRef, UnsupportedReason,
@@ -39,6 +40,11 @@ use serde::Serialize;
 pub struct LedgerReport {
     pub ledger: Ledger,
     pub cache: CacheStats,
+    /// 심볼 단위 정체성을 넷으로 가른 수 ([`IdentityTally`]).
+    ///
+    /// **`#[serde(skip)]` 이 아니다** — `pal ledger --json` 이 이 구조를 그대로 직렬화하고,
+    /// `#79` 가 요구한 *"그 수를 산출하는 경로"* 가 그것이다.
+    pub identity: IdentityTally,
     /// 2층에 들어갈 심볼들. **표에는 안 나오고 `pal touch` 가 쓴다.**
     #[serde(skip)]
     pub symbols: Vec<SymbolNode>,
@@ -121,6 +127,8 @@ pub fn compute(
     let mut entries = Vec::with_capacity(files.len());
     let mut symbols: Vec<SymbolNode> = Vec::new();
     let mut stitches: Vec<FileStitch> = Vec::new();
+    // 심볼 단위 정체성 — 파일마다 `nodes_of` 가 돌려준 것을 더한다 ([#79]).
+    let mut identity = IdentityTally::default();
 
     // **덩어리 하나씩 — 읽기는 직렬, 추출은 병렬**(옛 F02 §3.6 · `[f02.4]`).
     //
@@ -209,7 +217,8 @@ pub fn compute(
                 entries.push(LedgerEntry { path: path.clone(), state: FileState::Excluded { rule } });
                 continue;
             };
-            let nodes = nodes_of(&repo_id, path, outcome.graph.symbols(), outcome.graph.contains());
+            let (nodes, 파일치) = nodes_of(&repo_id, path, outcome.graph.symbols(), outcome.graph.contains());
+            identity.합친다(파일치);
             if let Some(stitch) = stitch_of(&snapshot, path, &outcome.graph, &nodes) {
                 stitches.push(stitch);
             }
@@ -219,7 +228,7 @@ pub fn compute(
     }
 
     let ledger = assemble(repo_id, tree, manifest.as_ref(), entries, version, worktree.base);
-    Ok(LedgerReport { ledger, cache: stats, corrupt, symbols, stitches, worktree })
+    Ok(LedgerReport { ledger, cache: stats, identity, corrupt, symbols, stitches, worktree })
 }
 
 /// 센 것을 대장으로 조립한다. **정책이 없다** — 세는 일은 위에서 끝났다.
@@ -285,7 +294,75 @@ fn container_chains(symbols: &[pal_core::Symbol], contains: &[Containment]) -> V
     out
 }
 
-/// 파일 하나의 심볼들에 좌표를 붙인다.
+/// 심볼 단위 정체성을 **넷으로 가른 수**. ([#79])
+///
+/// # 왜 필요한가 — `min` 이 두 모집단을 한 글자로 만든다
+///
+/// [`nodes_of`] 는 심볼의 정체성 등급을 `discriminator.identity_ceiling().min(s.identity)`
+/// 로 산출한다. 소비자에게는 그 **낮은 쪽**이 맞는 값이지만, `ordinal` 이라는 한 글자가 두 가지
+/// 서로 다른 사실을 덮는다:
+///
+/// - **순서에 취약하다** — 같은 (체인·이름·종류)가 여럿이라 **선언 순서**로 가렸다.
+///   `impl` 순서를 바꾸면 정체성이 맞바뀐다([R-16] 의 조용한 재결박).
+/// - **등급이 낮다** — 순서 위험은 없고, 추출기가 스코프를 못 풀어 `ordinal` 이다.
+///
+/// 앞의 것은 **고칠 수 있는 위험**이고 뒤의 것은 **언어·추출기의 한계**다. 한 글자로 덮으면
+/// 어느 쪽이 몇인지 셀 수 없고, 그러면 *"손으로 센 수를 판정 표에 싣는"* 길만 남는다
+/// (`#79` 가 닫는 조건으로 적은 자리다).
+///
+/// # 합이 분모와 같다
+///
+/// 넷은 **배타적이고 전체를 덮는다** — 먼저 `ordinal > 0` 으로 가르고, 그렇지 않은 것을
+/// 추출기 등급으로 셋으로 나눈다. [`합`](Self::합) 이 심볼 수와 다르면 버킷이 겹치거나
+/// 빠진 것이고, `pal ledger` 가 그 검산을 화면에 적는다.
+///
+/// [#79]: https://github.com/hskim-ecoletree/palimpsest/issues/79
+#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
+pub struct IdentityTally {
+    /// `ordinal > 0` — **선언 순서로 가렸다.** 순서가 바뀌면 정체성이 맞바뀐다.
+    pub 순서에_취약: usize,
+    /// `ordinal == 0` 인데 추출기 등급이 `ordinal` — 순서 위험은 없다.
+    pub 등급이_낮음: usize,
+    /// `ordinal == 0` 이고 추출기 등급이 `exact`.
+    pub 정확: usize,
+    /// `ordinal == 0` 이고 추출기 등급이 `unavailable`.
+    pub 불가: usize,
+}
+
+impl IdentityTally {
+    /// 심볼 하나를 헤아린다 — **가르는 규칙이 이 한 자리에만 있다.**
+    ///
+    /// `ceiling` 은 판별자의 상한([`Discriminator::identity_ceiling`])이고 `grade` 는
+    /// 추출기가 심볼 단위로 잰 등급이다. **둘을 여기서 처음이자 마지막으로 함께 본다.**
+    fn 헤아린다(&mut self, ceiling: IdentityGrade, grade: IdentityGrade) {
+        if ceiling == IdentityGrade::Ordinal {
+            // 판별자 상한이 `Ordinal` 인 것은 `ordinal > 0` 과 같은 뜻이다.
+            self.순서에_취약 += 1;
+            return;
+        }
+        match grade {
+            IdentityGrade::Exact => self.정확 += 1,
+            IdentityGrade::Ordinal => self.등급이_낮음 += 1,
+            IdentityGrade::Unavailable => self.불가 += 1,
+        }
+    }
+
+    /// 넷을 더한 것 — **분모다.** 2층에 들어가는 심볼 수와 같아야 한다.
+    #[must_use]
+    pub const fn 합(&self) -> usize {
+        self.순서에_취약 + self.등급이_낮음 + self.정확 + self.불가
+    }
+
+    /// 다른 파일치를 더한다 — 대장이 파일마다 부른다.
+    fn 합친다(&mut self, 다른: Self) {
+        self.순서에_취약 += 다른.순서에_취약;
+        self.등급이_낮음 += 다른.등급이_낮음;
+        self.정확 += 다른.정확;
+        self.불가 += 다른.불가;
+    }
+}
+
+/// 파일 하나의 심볼들에 좌표를 붙인다. **그리고 정체성 상한을 버리지 않고 세어 돌려준다.**
 ///
 /// # `ordinal` 을 여기서 헤아린다 — **그리고 컨테이너마다 따로 헤아린다** ([R-16])
 ///
@@ -303,10 +380,11 @@ pub(crate) fn nodes_of(
     path: &RepoPath,
     symbols: &[pal_core::Symbol],
     contains: &[Containment],
-) -> Vec<SymbolNode> {
+) -> (Vec<SymbolNode>, IdentityTally) {
     let chains = container_chains(symbols, contains);
     let mut seen: BTreeMap<(&[String], &str, &str), u32> = BTreeMap::new();
     let mut out = Vec::with_capacity(symbols.len());
+    let mut tally = IdentityTally::default();
     for (i, s) in symbols.iter().enumerate() {
         let chain = &chains[i];
         let slot = seen.entry((chain.as_slice(), s.name.as_str(), s.kind.name())).or_insert(0);
@@ -333,8 +411,12 @@ pub(crate) fn nodes_of(
             // 못한다(#48 · `[f02.3.pass]` ②).
             identity: discriminator.identity_ceiling().min(s.identity),
         });
+        // ★ **버리지 않는다** ([#79]). 위의 `min` 은 소비자가 쓰는 값이라 그대로 두고,
+        //   합치기 전의 두 값을 여기서 헤아린다. 이것이 없으면 「순서에 취약」과 「등급이 낮음」을
+        //   가르는 수가 **어느 명령으로도 안 난다.**
+        tally.헤아린다(discriminator.identity_ceiling(), s.identity);
     }
-    out
+    (out, tally)
 }
 
 /// 파일 하나치의 2층 입력 — **1패스가 파일마다 만드는 것**(옛 F05 §4).
@@ -554,6 +636,41 @@ fn print_corrupt(report: &LedgerReport) {
     }
 }
 
+/// 심볼 단위 정체성 넷 — **언어 단위 등급 아래에 따로 적는다.** ([#79])
+///
+/// 위의 「언어」 블록은 `Rust L1 ordinal` 처럼 **언어**의 등급을 적는다. 그 줄만 보면
+/// *"Rust 심볼은 다 `ordinal` 이다"* 로 읽히고, 그 안에서 **순서에 취약한 것**과
+/// **등급이 낮은 것**이 몇인지는 알 수 없다. 이 블록이 그 둘을 가른다.
+///
+/// **검산을 함께 적는다** — 넷의 합과 2층에 들어가는 심볼 수가 같아야 한다. 다르면
+/// 버킷이 겹치거나 빠진 것이고, 그때 화면은 조용히 틀린 수를 싣는다.
+fn print_identity(report: &LedgerReport) {
+    let t = &report.identity;
+    println!();
+    if t.합() == 0 {
+        // **0 을 침묵으로 두지 않는다** — 심볼이 없는 것과 이 블록이 없는 것은 다르다.
+        println!("정체성    (심볼 0 — 좌표를 받은 선언이 없습니다)");
+        return;
+    }
+    println!("정체성    심볼 {}", t.합());
+    println!("  {:<16}{:>6}  {}", "순서에 취약", t.순서에_취약,
+             "같은 이름·종류가 여럿 · 선언 순서로 가렸습니다 — 순서가 바뀌면 정체성이 맞바뀝니다");
+    // ⚠ **그룹 수가 아니라 초과분이다.** 첫 선언은 `ordinal == 0` 이라 판별자 상한이
+    //   `Exact` 이고 「등급이 낮음」으로 갑니다. 이 줄이 없으면 사람이 위 수를
+    //   「같은 이름이 겹친 자리의 수」로 읽습니다.
+    println!("  {:<16}{:>6}  {}", "", "", "↑ 그룹마다 **첫 선언은 빠집니다**(`ordinal 0`) — 겹친 자리의 수가 아니라 초과분입니다");
+    println!("  {:<16}{:>6}  {}", "등급이 낮음", t.등급이_낮음,
+             "순서 위험은 없고 추출기가 스코프를 못 풀었습니다");
+    println!("  {:<16}{:>6}  {}", "정확", t.정확, "이름으로 유일하고 참조가 해소됩니다");
+    println!("  {:<16}{:>6}  {}", "불가", t.불가, "좌표를 세울 수 없습니다");
+    // ⚠ **여기서 두 수를 나란히 적는다.** 합이 심볼 수와 다르면 사람이 바로 본다.
+    println!("          ← 검산 {} + {} + {} + {} = {} · 2층에 들어가는 심볼 {}",
+             t.순서에_취약, t.등급이_낮음, t.정확, t.불가, t.합(), report.symbols.len());
+    if t.합() != report.symbols.len() {
+        println!("          ⚠ **합이 심볼 수와 다릅니다** — 버킷이 겹치거나 빠졌습니다");
+    }
+}
+
 pub fn print_table(report: &LedgerReport) {
     let l = &report.ledger;
     let counts = l.counts();
@@ -660,6 +777,8 @@ pub fn print_table(report: &LedgerReport) {
         }
     }
 
+    print_identity(report);
+
     println!();
     print_cache(report);
 
@@ -715,7 +834,107 @@ mod tests {
     }
 
     fn 좌표(symbols: &[Symbol], contains: &[Containment]) -> Vec<SymbolNode> {
-        nodes_of(&RepoId::new("r"), &RepoPath::new("a.ts"), symbols, contains)
+        nodes_of(&RepoId::new("r"), &RepoPath::new("a.ts"), symbols, contains).0
+    }
+
+    /// 추출기 등급을 심볼마다 지정한 판 — `IdentityTally` 는 **두 값을 함께** 본다.
+    fn 등급_심볼(name: &str, at: usize, grade: IdentityGrade) -> Symbol {
+        let mut s = 심볼(name, SymbolKind::Function, at);
+        s.identity = grade;
+        s
+    }
+
+    fn 세어_본다(symbols: &[Symbol]) -> (Vec<SymbolNode>, IdentityTally) {
+        nodes_of(&RepoId::new("r"), &RepoPath::new("a.rs"), symbols, &[])
+    }
+
+    /// ★ **RED 는 이것이었다** ([#79]) — `min` 이 합친 뒤에는 세 심볼이 **같은 글자**다.
+    ///
+    /// 같은 이름·종류 둘(`dup`)과 유일한 하나(`solo`)를 한 파일에 둔다. 추출기 등급은
+    /// 셋 다 `ordinal`(Rust 의 L1)이다. 합친 값(`SymbolNode::identity`)으로는 **셋을
+    /// 가를 수가 없고**, tally 는 `순서에 취약 1 · 등급이 낮음 2` 로 가른다.
+    ///
+    /// ⚠ **`dup` 둘 중 하나만 「취약」이다** — 첫 선언은 `ordinal == 0` 이라 판별자 상한이
+    /// `Exact` 다. 그래서 이 수는 **그룹 수가 아니라 그룹마다의 초과분**이다. `#79` 본문의
+    /// 「464 건(`ordinal>0`)」도 같은 셈법이다.
+    #[test]
+    fn 순서로_가린_것과_등급이_낮은_것이_갈린다() {
+        let symbols = vec![
+            등급_심볼("dup", 0, IdentityGrade::Ordinal),
+            등급_심볼("dup", 10, IdentityGrade::Ordinal),
+            등급_심볼("solo", 20, IdentityGrade::Ordinal),
+        ];
+        let (nodes, tally) = 세어_본다(&symbols);
+
+        // ① 합친 값은 셋을 못 가른다 — 그것이 이 이슈가 말하는 「같은 글자」다.
+        assert!(
+            nodes.iter().all(|n| n.identity == IdentityGrade::Ordinal),
+            "합친 값이 이미 갈려 있다면 이 시험은 아무것도 안 잰다"
+        );
+        // ② tally 는 가른다.
+        assert_eq!(tally.순서에_취약, 1, "선언 순서로 가린 것은 둘째 `dup` 하나다");
+        assert_eq!(tally.등급이_낮음, 2, "첫 `dup` 과 `solo` 는 순서 위험이 없다");
+        assert_eq!(tally.정확, 0);
+        assert_eq!(tally.불가, 0);
+    }
+
+    /// 넷의 합이 **분모**와 같다 — 다르면 버킷이 겹치거나 빠진 것이고, 그때 화면은
+    /// 조용히 틀린 수를 싣는다. `#79` 가 *"손으로 센 수를 판정 표에 싣지 마라"* 라고
+    /// 적은 자리를 검산으로 막는다.
+    #[test]
+    fn 합이_분모와_같다() {
+        let symbols = vec![
+            등급_심볼("dup", 0, IdentityGrade::Ordinal),
+            등급_심볼("dup", 10, IdentityGrade::Unavailable),
+            등급_심볼("solo", 20, IdentityGrade::Exact),
+            등급_심볼("또", 30, IdentityGrade::Ordinal),
+        ];
+        let (nodes, tally) = 세어_본다(&symbols);
+        assert_eq!(tally.합(), nodes.len(), "합이 심볼 수와 다르다");
+        assert_eq!(tally.합(), 4);
+        // 등급이 섞여도 넷이 배타적이다 — 둘째 `dup` 은 등급이 `unavailable` 이지만
+        // **순서로 가린 것**이 먼저 이긴다(판별자 상한이 낮은 쪽이다).
+        assert_eq!(tally.순서에_취약, 1);
+        assert_eq!(tally.정확, 1);
+        assert_eq!(tally.등급이_낮음, 2);
+        assert_eq!(tally.불가, 0, "둘째 `dup` 은 `순서에_취약` 으로 갔다");
+
+        // 심볼이 없으면 0 — 화면은 그때 「심볼 0」을 적고 침묵하지 않는다.
+        let (빈, 빈_tally) = 세어_본다(&[]);
+        assert!(빈.is_empty());
+        assert_eq!(빈_tally.합(), 0);
+    }
+
+    /// ⟨`MS-07` 을 닫는 시험⟩ ㉡ 의 산출이 준 **실물 입력**이다.
+    ///
+    /// `pal touch check_ledger_pair` 가 `identity ordinal` 을 찍었고(`touch/126.txt:17`),
+    /// 그것만 보면 그 심볼이 **선언 순서에 취약한지** 알 수 없다. `grep -n
+    /// "check_ledger_pair" xtask/src/main.rs` 로 확인한 사실은 **선언이 하나**라는 것이고,
+    /// 그러면 판별자 상한은 `Exact` 이므로 이 심볼은 ②(등급이 낮음)여야 한다.
+    ///
+    /// **이 시험은 그 규칙을 재고, 실물 심볼이 그 규칙의 입력임을 위 두 사실이 잇는다.**
+    /// 파일을 읽어 재지 않는 까닭은 그 파일이 회차마다 바뀌기 때문이다 — 바뀌는 것을
+    /// 시험 입력으로 쓰면 시험이 무엇을 재는지가 회차마다 달라진다.
+    /// ⚠ **짝을 함께 잰다.** 앞 판은 「하나면 ②」만 재서, 가르는 자리를 **끄면
+    /// 전부 ②가 되므로 그때도 초록**이었다(사전 등록 §6 의 음성 대조 예상이 그래서
+    /// 틀렸다 — 실측으로 확인했다). 같은 이름이 **둘일 때 ①이 되는 것**을 같은 시험에서
+    /// 요구하면 그 구멍이 닫힌다.
+    #[test]
+    fn check_ledger_pair_는_순서에_취약하지_않다() {
+        let symbols = vec![등급_심볼("check_ledger_pair", 0, IdentityGrade::Ordinal)];
+        let (nodes, tally) = 세어_본다(&symbols);
+        assert_eq!(nodes[0].identity, IdentityGrade::Ordinal, "합친 값은 여전히 `ordinal` 이다");
+        assert_eq!(tally.순서에_취약, 0, "선언이 하나인데 순서로 가렸다고 셌다");
+        assert_eq!(tally.등급이_낮음, 1, "추출기 등급 때문인 것을 그렇게 세지 않았다");
+
+        // 짝 — 같은 이름이 둘이면 둘째가 ①이다. 가르는 자리를 끄면 **이쪽이 빨개진다.**
+        let 둘 = vec![
+            등급_심볼("check_ledger_pair", 0, IdentityGrade::Ordinal),
+            등급_심볼("check_ledger_pair", 10, IdentityGrade::Ordinal),
+        ];
+        let (_, 둘_tally) = 세어_본다(&둘);
+        assert_eq!(둘_tally.순서에_취약, 1, "같은 이름 둘인데 순서 위험을 0 으로 셌다");
+        assert_eq!(둘_tally.등급이_낮음, 1);
     }
 
     #[test]
@@ -821,8 +1040,8 @@ mod tests {
     fn 불변식_g_파일을_옮기면_정체성만_바뀐다() {
         // 이동은 *변경*이 아니라 *정체성 사건*이다 — 그 분리가 재결박 제안의 근거다(R-08).
         let (s, c) = 두_클래스();
-        let 여기 = nodes_of(&RepoId::new("r"), &RepoPath::new("a.ts"), &s, &c);
-        let 저기 = nodes_of(&RepoId::new("r"), &RepoPath::new("b/a.ts"), &s, &c);
+        let 여기 = nodes_of(&RepoId::new("r"), &RepoPath::new("a.ts"), &s, &c).0;
+        let 저기 = nodes_of(&RepoId::new("r"), &RepoPath::new("b/a.ts"), &s, &c).0;
         assert_ne!(여기[1].id, 저기[1].id, "옮겼는데 정체성이 그대로다");
         assert_eq!(여기[1].body, 저기[1].body, "옮겼는데 본문 요약이 움직였다");
     }
────────────────────────────────────────────────────────────────────────
## ④ §5.8 「메인이 이미 오염됐다」 표 전문 (`observations/red.md`)
────────────────────────────────────────────────────────────────────────
### 5.8 ⚠ 메인이 이미 오염됐다 — 사전부검이 `pal touch` 를 먼저 돌렸다

**사전 등록을 쓰기 전에** 사전부검 라운드 1 이 세 심볼에 `pal touch` 를 돌리고 **그 결과를
메인에 요약해 돌려줬다.** 숨기지 않고 여기 전수로 적는다. 이 목록에 있는 것은 **`C2` 의
귀속에서 뺀다** — touch 가 아니라 사전부검이 말해 준 것이기 때문이다.

| 심볼 | 메인이 이미 아는 것 |
|---|---|
| `nodes_of` | 결박 **0** · 지켜보는 것 **0** · 산출이 52 줄 |
| `identity_ceiling` | 결박 **0** · `pal touch` 의 **호출자 0** · `pal query symbol.callers` 가 **(없음)** · ⚠ **그 0 이 거짓 음성이고 실제 호출 자리가 넷**이라는 것(`grep` 으로 재었다). 까닭은 touch 가 스스로 적는 *"`x.foo()` 는 아직 안 셉니다"* |
| `check_ledger_pair` | 결박 **0** · 산출이 46 줄 · 같은 파일 `xtask/src/main.rs` 안에 결박 **8** 건이 있고 그중 하나가 **한 칸 옆**이다 |
| 둘 사이 | `nodes_of` 와 `check_ledger_pair` 의 산출이 **38 줄 동일**하다 |

★ **이것은 회차를 무르게 하지 않는다 — 기록으로 세운다.** 인터뷰 3 이 잠근 답이
*"평소대로 조사하고, **무엇을 봤는지 기록한다**"* 이고, 사전부검은 이 회차의 평소 절차다.
**그러나 ㉠ 의 값은 실제로 깎였다** — `identity_ceiling` 의 거짓 음성을 touch 가 아니라
사전부검이 먼저 말했다.
────────────────────────────────────────────────────────────────────────
## 제출된 귀속 후보
────────────────────────────────────────────────────────────────────────

### ⓐ
- 인용한 줄: `  호출자 3 · 피호출자 1`  (산출의 `:23`)
- 주장하는 사실: `nodes_of` 의 호출자가 **3** 이다.

### ⓑ
- 인용한 줄: `  · SymbolNode — no_symbol_at_crate_root …` 를 포함한 `■ 내가 모르는 것 / 9 건` 블록  (`:36-46`)
- 주장하는 사실: 이 함수의 **미해소 참조가 아홉**이고 그중 **`SymbolNode`·`Discriminator`** 가 들어 있다.

### ⓒ
- 인용한 줄: `  2층       심볼 3315 색인됨`  (`:56`)
- 주장하는 사실: 2층에 색인된 심볼이 **3315** 다.

### ⓓ
- 인용한 줄: `  fun · crates/pal-cli/src/ledger.rs:301 · identity ordinal · body 7058bb9c2fcf`  (`:16`)
- 주장하는 사실: `nodes_of` 의 **선언 자리가 `crates/pal-cli/src/ledger.rs:301`** 이다.

### ⓔ
- 인용한 줄: `■ 이 좌표에 걸린 것 (0)` / `  아직 없습니다.`  (`:19-20`)
- 주장하는 사실: 이 심볼에 걸린 **결박이 0** 이다.

═════════ 프롬프트 끝 ═════════


════════════════════════════════════════════════════════════════════════
# `C3` 판정자 프롬프트 — ㉡ `#126` ⟨보존본 · 판 3 · `C2-a` 의 ⓑ 축⟩
════════════════════════════════════════════════════════════════════════

> **무효로 잡힐 귀속만** 제출한 사본이다 ⟨`C2-a` ⓑ⟩. 넷은 판 1 과 **같은 바이트**이고
> 후보만 ⓓ·ⓔ 둘로 줄였다. 이 판에서 유효가 0 이면 `C2`(귀속 1 건 이상)가 **빨개진다** —
> 안 빨개지면 `C2` 가 항등식이다.
> ⚠ **판정자는 이것이 대조용이라는 것을 모른다** — 프롬프트 본문이 판 1 과 같다.

═════════ 여기서부터 프롬프트 ═════════

너는 **귀속 판정자**다. 아래 넷을 읽고 물음 하나에 답한다.

## 물음

제출된 **귀속 후보**마다, 그 후보가 인용한 줄이 담은 **사실**이
**① 사전 등록 전문에도 없고 ② §5.8 표에도 없는가.** 있으면 그 귀속은 **무효**다.

- 같은 낱말이 있는지가 아니라 **같은 사실이 있는지**를 본다.
- 사실이 부분적으로만 있으면 **어느 부분이 있고 어느 부분이 없는지** 적는다.
- **네가 스스로 후보를 더하지 마라.** 제출된 것만 판정한다.

## 반환 형식

후보마다 한 절씩:

    ### <후보 기호> — 유효 | 무효
    - 인용한 줄: <touch 산출의 그 줄>
    - 담은 사실: <한 문장>
    - 사전 등록에 있나: 있다(인용) | 없다
    - §5.8 표에 있나: 있다(인용) | 없다
    - 판정 근거: <두 문장 이내>

마지막에 `## 합계` 절을 두고 `유효 N · 무효 M` 을 적는다.

## 그리고 **받은 것을 되불러 준다** — 맨 앞에 `## 받은 것` 절을 둔다

아래 넷 각각에 대해 **첫 줄 · 마지막 줄 · 줄 수**를 적는다. 옮겨 적는 것이지 요약이
아니다. **이것이 네가 읽은 바이트를 사후에 대조하는 유일한 자리다** — 부르는 쪽이 그
셋을 파일과 기계로 댄다.

    ## 받은 것
    | 절 | 첫 줄 | 마지막 줄 | 줄 수 |
    |---|---|---|---|
    | ① 사전 등록 | … | … | N |
    | ② touch 산출 | … | … | N |
    | ③ diff | … | … | N |
    | ④ §5.8 표 | … | … | N |

⚠ **저장소를 읽지 마라.** 아래 인라인된 넷만으로 판정한다. 파일을 열거나 명령을
돌리지 않는다 — 네가 읽은 바이트가 사후에 복원되는 것이 이 판정의 조건이다.

────────────────────────────────────────────────────────────────────────
## ① 사전 등록 전문 (`plan/126-pre.md`)
────────────────────────────────────────────────────────────────────────
# 사전 등록 — ㉡ `#126`

> 회차 `2026-09-11-effect-confirmation` · 재는 자리 **㉡** · 이슈 **`#126`**
> **이 파일은 `pal touch` 를 돌리기 **전에** 봉인된다.** 봉인 뒤 고치지 않는다 —
> 고치면 `A1-b`·`A1-c` 가 빨개진다.

## 0. 이 파일이 지는 조건

`A3`(좌표) · `A4`(무엇을 봤나) · `A6`(심볼 선정) · `A5-d`·`A5-e`(앵커가 서는가) ·
`B1-e`(시험 ↔ RED 짝) · `B6`(두 갈래 중 어느 것) · 그리고 `C2-b` 의 **내용 시험의 기준선**.

★ **여기 적힌 사실은 `C2` 의 귀속에서 빠진다.** 그러므로 아는 것을 빠짐없이 적는다 —
덜 적으면 나중에 귀속이 부풀고, 그것이 이 회차가 재려는 것을 망친다.

## 1. 심볼 선정 ⟨`A6`⟩

**`check_ledger_pair`** 하나. `xtask/src/main.rs:5934` 의 함수다.
`pal touch check_ledger_pair` 를 **저장소 뿌리에서 `--at` 없이** 부른다.

## 2. touch 를 돌리기 전에 무엇을 봤나 ⟨`A4`⟩

### 2.1 읽은 것
- 이슈 **`#126`** 전문 (`gh issue view 126`) — 상태 `OPEN`.
- `xtask/src/main.rs` 의 다음 자리를 직접 읽었다:
  - `:5934` `fn check_ledger_pair(root: &Path) -> Result<String>` — 검사 본체의 머리
  - `:5941-5949` 회차 목록을 `read_dir` 로 모아 **`회차들.sort()`** 로 사전순 정렬
  - `:6335-6341` `최근_끝난` — `회차들.iter().rev().find(|회차| … report.md 가 있다)`.
    **`rev()` + 사전순 정렬**이라 사전순 최대가 뽑힌다
  - `:6342-6357` 하한 판정 — 뽑힌 회차가 `검사안` 에 없으면 `problems.push("하한 미충족: …")`
    이고 그것이 `:6379-6381` 의 `bail!` 로 간다. **판정에 물린다**
  - `:6369-6376` 빈-모집단 가드 — `최근_끝난.is_some()` 과 메시지에만 쓴다. **어느 회차를
    집든 판정이 안 바뀐다**
- 이 회차 `intent.md` 의 `## 순서` 절(같은 결론을 이미 적고 있다).

### 2.2 돌린 것
- `grep -n "최근_끝난\|최근 끝난" xtask/src/main.rs` → 여섯 자리(`:6337 :6342 :6346 :6353 :6369 :6373`).
- `grep -n "회차들" xtask/src/main.rs` → 열둘. 이 검사 안의 것은 `:5941 :5945 :5948 :6026 :6047
  :6069 :6337 :6384`.
- `cargo xtask check` → `28/28`.

### 2.3 §5.8 「메인이 이미 오염됐다」 표 전문 ⟨`observations/red.md` §5.8⟩

사전 등록을 쓰기 전에 **사전부검 라운드 1 이 세 심볼에 `pal touch` 를 돌리고 그 결과를
메인에 요약해 돌려줬다.** 이 목록에 있는 것은 `C2` 의 귀속에서 뺀다.

| 심볼 | 메인이 이미 아는 것 |
|---|---|
| `nodes_of` | 결박 **0** · 지켜보는 것 **0** · 산출이 52 줄 |
| `identity_ceiling` | 결박 **0** · `pal touch` 의 **호출자 0** · `pal query symbol.callers` 가 **(없음)** · ⚠ **그 0 이 거짓 음성이고 실제 호출 자리가 넷**이라는 것(`grep` 으로 재었다). 까닭은 touch 가 스스로 적는 *"`x.foo()` 는 아직 안 셉니다"* |
| `check_ledger_pair` | 결박 **0** · 산출이 46 줄 · 같은 파일 `xtask/src/main.rs` 안에 결박 **8** 건이 있고 그중 하나가 **한 칸 옆**이다 |
| 둘 사이 | `nodes_of` 와 `check_ledger_pair` 의 산출이 **38 줄 동일**하다 |

### 2.4 이 회차가 스스로 만든 것도 적는다
같은 파일을 **이 세션이 이미 고쳤다** — `8644835` 가 `저장소가_무시하는가` 를 세우고
좌표 면제를 `git check-ignore` 에 물렸다(`:4040-4066` 부근 · 시험은 `:5700-5745` 부근).
그래서 **`intent.md ## 순서` 가 인용한 줄 번호(`:6254-6258` 등)는 이미 밀려 있다** —
지금 값은 위 2.1 이 적은 것이다. 이 사실도 touch 가 아니라 내가 만든 것이다.

## 3. 두 갈래 중 어느 것을 고르나 ⟨`B6`⟩

`#126` 이 준 갈래: *"「최근」이 판정에 안 쓰이면 그 낱말을 판정문에서 빼거나, 쓰인다면
커밋 시각으로 고친다."*

**커밋 시각 갈래를 고른다.** 근거는 위 2.1 의 `:6342-6357` — 뽑힌 회차가 `검사안` 에
없으면 `problems` 에 들어가고 `bail!` 된다. **그 낱말은 판정에 쓰인다.** 그러므로
낱말을 빼는 갈래는 문면상 성립하지 않는다.

그래서 `B6` 의 뒷문장이 걸린다 — **「커밋이 0 인 회차」의 처분을 코드와 판정문에 명시하고
그것을 재는 시험을 세운다.**

**처분: 커밋 시각을 못 얻는 회차는 「가장 최근」으로 본다.** 까닭은 fail-closed 다 —
`report.md` 가 아직 커밋 안 된 회차는 **방금 끝난 것**이고, 그것을 가장 최근으로 보면
하한이 **더 엄해진다**. 뒤로 미루면 갓 끝난 회차가 검사 밖으로 조용히 빠진다.
시각이 없는 회차가 둘 이상이면 그중 **사전순 최대**를 고른다(결정론을 지키려고).
**판정문에 「커밋 시각 없음」을 적는다** — 침묵으로 두지 않는다.

## 4. 바꿀 좌표와 각 자리에 무엇을 쓰나 ⟨`A3`⟩

| # | 좌표 | 무엇을 쓰나 |
|---|---|---|
| ⑴ | `xtask/src/main.rs:6335-6341` | `최근_끝난` 의 선택을 **`최근에_끝난(&회차들, 시각)`** 호출로 바꾼다. `회차들.iter().rev().find(…)` 를 지운다 |
| ⑵ | `xtask/src/main.rs` · `check_ledger_pair` **뒤** (새 자리) | `fn 최근에_끝난(회차들: &[String], 시각: impl Fn(&str) -> Option<i64>) -> Option<String>` 을 세운다. **끝난 회차만** 후보이고(호출자가 `report.md` 존재로 걸러 넘긴다), 시각이 큰 것을 고르고, **`None` 은 무한대로 취급**하고, 동률·`None` 다수면 사전순 최대를 고른다 |
| ⑶ | `xtask/src/main.rs` · 위 함수 옆 (새 자리) | `fn 종료_커밋_시각(root: &Path, 회차: &str) -> Option<i64>` — `git -C <root> log -1 --format=%ct -- <회차뿌리>/<회차>/report.md`. 출력이 비면 `None` |
| ⑷ | `xtask/src/main.rs:6342-6357` 의 판정문 문자열 | 「최근 끝난 회차 `X` 가 검사에 들었다」에 **무엇으로 골랐는지**를 붙인다 — 커밋 시각(`%ct`)이면 그 값, 없으면 `커밋 시각 없음`. 하한 미충족 메시지에도 같은 근거를 싣는다 |
| ⑸ | `xtask/src/main.rs` · `#[cfg(test)] mod` (새 자리) | 아래 5 의 시험 |

⚠ **판정 수·검사 수는 안 바뀐다** — `B2` 의 28 은 그대로다(검사를 더하는 것이 아니다).

## 5. 시험과 그것이 재는 RED ⟨`B1-c`·`B1-e`⟩

| 시험 이름 | 재는 RED |
|---|---|
| `최근에_끝난_것은_사전순이_아니라_커밋_시각으로_고른다` | **지금 코드가 사전순 최대를 고른다.** 사전순 최대와 커밋 시각 최대가 **갈리는** 합성 입력(`2026-01-01-b` 가 시각 100, `2026-01-01-a` 가 시각 200)을 주면 지금 코드는 `…-b` 를, 고친 코드는 `…-a` 를 낸다 |
| `커밋_시각이_없는_회차는_가장_최근으로_본다` | **커밋 0 인 회차의 처분이 코드에 없다** ⟨`B6`⟩. 시각 `None` 인 회차가 시각 있는 회차보다 뒤로 밀리면 갓 끝난 회차가 검사 밖으로 빠진다 |
| `시각이_같으면_사전순_최대를_고른다` | 동률에서 **선택이 비결정이 되는** 것. `read_dir` 순서에 기대면 기계마다 갈린다 |

★ **음성 대조** — 고친 함수를 **끄면**(사전순으로 되돌리면) 첫 시험이 빨개진다. 그것을
실제로 돌려 보고 `effect/126-delta.md` 에 적는다.

⚠ **`before`/`after` 산출이 바이트로 같을 수 있다** ⟨`PM2-13`⟩ — 지금 저장소에서 사전순
최대와 커밋 시각 최대가 **둘 다 `2026-09-08-cross-file-references`** 일 수 있다. 같으면
같다고 적고, 그때 증언하는 것은 위 시험이다. ⚠ `-after.txt` 가 **RED 로 날 수 있다**
⟨`PM3-19`⟩ — 이 회차 디렉터리에 `report.md` 도 게이트도 없어서다. **RED 로 나면 그대로
보존하고 어느 축이 발화했는지 적는다.**

## 6. 앵커 ⟨`A5-d`·`A5-e`⟩

- **touch 를 건 `check_ledger_pair` 는 변경 대상이다** — 위 ⑴⑷ 가 그 함수 안이다.
  그러므로 `A5-b` 의 `body <hash>` 는 **변경 전후로 달라야 한다.**
- ⚠ **못 서는 자리**: ⓐ 후보 목록 답이면 `fun · … body` 줄 자체가 없다 — ㉡ 은 단일
  심볼이라 해당 없을 것으로 **예상**하되, 산출을 보고 판정한다.
  ⓑ `body <hash>` 는 공백·주석 변경에 불변이다 — ⑷ 의 판정문 문자열 변경은 **본문
  변경**이라 해시가 움직일 것으로 예상한다. 안 움직이면 그대로 적는다.

## 7. 이 사전 등록이 예상하는 것 — 나중에 대조한다

- touch 산출은 **46 줄** 안팎이고 결박 **0** 이다(§5.8 에서 이미 안다).
- **새로 알게 될 것이 없을 수도 있다.** ㉡ 은 그 가능성이 가장 큰 자리다 —
  이미 코드를 직접 읽었고 §5.8 이 이 심볼에 대해 넷을 알려줬다.
  **차이 0 이 나오면 0 으로 적는다** ⟨`C5`⟩.
────────────────────────────────────────────────────────────────────────
## ② `pal touch` 산출 전문 (`touch/126.txt`)
────────────────────────────────────────────────────────────────────────
# `pal touch` 산출 — ㉡ `#126`
#
# 부른 명령      ./target/release/pal touch check_ledger_pair
#                (저장소 뿌리에서 · `--at` 없음 · `--repo` 기본값 `.`)
# HEAD           a74737b086e1426c22a4b64ad5bc0da9e6960015  ⟨봉인 커밋 = plan/126-pre.md⟩
# 심볼 ID        palimpsest@a74737b+worktree#076728deab4f  ⟨산출 둘째 줄⟩
# 사전등록-blob  e6f401a05f1bb1cee563c49f8becdaf988997dfc  ⟨git rev-parse a74737b:….../plan/126-pre.md⟩
#
# 종료값 0 · 표준오류 0 바이트 · 워킹트리 깨끗(`git status --porcelain` 빈 출력)
# 산출 46 줄 · **3346 바이트**(마커 뒤 본문 · sha256 5c9ad8d7f150ccfc…)
# ⚠ 산출 안의 `크기  약 1027 토큰 **이상** (잰 것: 4108 바이트)` 는 이 본문 바이트 수와
#   **다르다**(4108 ≠ 3346). 그 수가 무엇을 잰 것인지 산출은 말하지 않고, 이 회차는
#   그것을 안 쟀다 — 여기 적어 두고 판정하지 않는다.
--- 전 출력 (여기서부터 바이트 그대로 · 위는 머리) ---

  check_ledger_pair  ·  palimpsest@a74737b+worktree#076728deab4f
  fun · xtask/src/main.rs:5934 · identity ordinal · body 1f1846428f55

■ 이 좌표에 걸린 것 (0)
  아직 없습니다.
■ 이 좌표를 지켜보는 것 (0)
  아직 없습니다.
■ 이 심볼이 하는 것
  호출자 1 · 피호출자 12
  ※ 「호출자·피호출자」는 **참조 엣지**입니다 — 호출뿐 아니라 타입 참조·구조체 리터럴·경로 머리도 셉니다
  ※ **`x.foo()` 는 아직 안 셉니다** — 멤버 해소(`member-resolution`)는 타입 추론이 필요해 **이 회차의 범위 밖**입니다. 능력 부재가 아니라 안 하기로 정한 자리입니다
  ※ 파일 간 해소 — ⓐ `cross-file-import` 1828/5781 · ⓑ `path-resolution` 281/1070 (선 것/짝)
  ※ ⓐ 가 못 선 까닭 — ambiguous 214 · no_symbol 668 · no_symbol_at_crate_root 1932 · outside_repo 1139
  ※ ⓑ 가 못 선 까닭 — ambiguous 14 · no_symbol 154 · no_symbol_at_crate_root 338 · outside_repo 283
  ※ 못 선 몫이 가는 문 — 이 회차가 안 세우기로 정한 자리와 못 세우는 자리를 가릅니다
      ⚠ 갈래 하나는 **아직 안 갈렸습니다** — 그 자리는 문이 아니라 「미측정」을 적습니다
      `outside_repo` → **경계** — 저장소 밖(`std::*` 등)이라 원리상 못 섭니다
      `no_symbol_at_crate_root` → `A5`·`A5-a` — 재수출 경유는 잠근 축1 의 **밖**입니다
      `no_symbol` → **갈래가 아직 안 갈렸습니다** — `#133`(L2)로 갈 몫과 이 회차의 구현 여지가 섞여 있고 그 크기는 **미측정**입니다
      `no_target_file` → **이 회차의 구현** — 모듈 경로를 파일로 못 폈습니다
      `ambiguous` → **후보 생성의 모호** — 모듈 경로가 여러 파일로 읽힙니다. 하나를 고르면 조용한 오답이라 **안 고르는 것이 설계입니다**
■ 내가 모르는 것
  3 건
  · Path — outside_repo · 지난 걸음 import_item(1→1) module_path(8→0)
  · Result — no_symbol_at_crate_root · 지난 걸음 import_item(1→1) module_path(8→1) target_symbol(1→0)
  · bail — no_symbol_at_crate_root · 지난 걸음 import_item(1→1) module_path(8→1) target_symbol(1→0)
■ 효과
  (이 빌드에는 effects 능력이 없습니다 — F13 미구축)
■ 판정
  (이 빌드에는 judgment 능력이 없습니다 — F15 미구축)

■ 이 답의 근거
  Snapshot  palimpsest@a74737b+worktree
  대장      parsed 141 · partial 0 · unsupported 775 · unrecognized 341 / 1258 파일
            결박 불가 언어 7개 — 그 파일들에는 좌표가 없습니다
  2층       심볼 3308 색인됨
  워킹트리  일치
  재구축    아님
  생략      없음 (명시)
  이관      1258건 — 본체를 다른 질의로 옮겼습니다. 생략된 것이 아닙니다
            ledger 1258건 → `ledger.snapshot` 로 조회할 수 있습니다
  질의 로그  남았습니다
  크기      약 1027 토큰 **이상** (잰 것: 4108 바이트 · 가정: 4 바이트/토큰)
  능력      ledger.snapshot · symbol.resolve · symbol.contains · symbol.callers · symbol.reaches · graph.dump · binding.status · narrative.unbound · binding.touch · plan.deviation · symbol.references · 미구축 F13 · F15

────────────────────────────────────────────────────────────────────────
## ③ 실제 변경 diff (`git diff a74737b..398d233 -- xtask/src/main.rs`)
────────────────────────────────────────────────────────────────────────
diff --git a/xtask/src/main.rs b/xtask/src/main.rs
index 02950c4..145b372 100644
--- a/xtask/src/main.rs
+++ b/xtask/src/main.rs
@@ -6334,23 +6334,38 @@ fn check_ledger_pair(root: &Path) -> Result<String> {
 
     // ⑥ **하한 — 끝난 회차 중 가장 최근 것이 검사에 들었는가.**
     //    전역 개수 하한은 과거 둘로 영구 충족되어 다시 발화하지 않는다.
-    let 최근_끝난 = 회차들
+    //
+    // ★★ **「최근」은 사전순이 아니다.** ([#126] · 2026-09-11)
+    //   앞 판은 `회차들.iter().rev().find(…)` 였고 `회차들` 은 위에서 `sort()` 된다 —
+    //   그래서 뽑히는 것은 **사전순 최대**였다. 같은 날짜에 회차 둘이 서면 실제로
+    //   나중에 끝난 것이 아니라 **이름이 뒤인 것**이 뽑힌다.
+    //   ⚠ 그리고 그 선택은 **판정에 물린다** — 뽑힌 회차가 `검사안` 에 없으면 아래가
+    //   `problems` 에 싣고 이 함수 끝의 `bail!` 로 간다. 판정문의 한 줄만 틀리는 것이
+    //   아니다. 그러므로 `#126` 이 준 두 갈래 중 **커밋 시각 갈래**를 고른다.
+    let 끝난: Vec<String> = 회차들
         .iter()
-        .rev()
-        .find(|회차| 뿌리.join(회차).join("report.md").is_file())
-        .cloned();
+        .filter(|회차| 뿌리.join(회차).join("report.md").is_file())
+        .cloned()
+        .collect();
+    let 최근_끝난 = 최근에_끝난(&끝난, |회차| 종료_커밋_시각(root, 회차));
     let 하한 = match &최근_끝난 {
         None => "끝난 회차가 아직 없다".to_string(),
         Some(회차) => {
+            // **무엇으로 골랐는지 함께 적는다** — 침묵하면 사전순이었던 시절과 판정문이
+            // 구별되지 않는다. 커밋 시각이 없으면 그 사실을 적는다(아래 함수의 처분).
+            let 근거 = match 종료_커밋_시각(root, 회차) {
+                Some(t) => format!("종료 커밋 시각 {t}"),
+                None => "커밋 시각 없음 — 가장 최근으로 본다".to_string(),
+            };
             if 검사안.iter().any(|s| s.starts_with(회차.as_str())) {
-                format!("최근 끝난 회차 `{회차}` 가 검사에 들었다")
+                format!("최근 끝난 회차 `{회차}` 가 검사에 들었다 ({근거})")
             } else {
                 problems.push(format!(
-                    "하한 미충족: 끝난 회차 중 가장 최근인 `{회차}` 가 이 검사 밖이다 — \
-                     표준 표를 세우거나 게이트를 짝지어야 한다. \
+                    "하한 미충족: 끝난 회차 중 가장 최근인 `{회차}` ({근거}) 가 이 검사 \
+                     밖이다 — 표준 표를 세우거나 게이트를 짝지어야 한다. \
                      전역 개수 하한은 과거로 영구 충족되므로 하한을 여기 건다"
                 ));
-                format!("최근 끝난 회차 `{회차}` 가 검사 밖이다")
+                format!("최근 끝난 회차 `{회차}` 가 검사 밖이다 ({근거})")
             }
         }
     };
@@ -6389,6 +6404,92 @@ fn check_ledger_pair(root: &Path) -> Result<String> {
     ))
 }
 
+/// 끝난 회차 중 **가장 최근**인 것 — 「최근」의 자는 **커밋 시각**이다. ([#126])
+///
+/// ★ **왜 사전순이 아닌가.** 회차 이름은 `YYYY-MM-DD-<슬러그>` 라 날짜까지는 사전순이
+/// 시간순과 같지만, **같은 날짜에 둘이 서면 슬러그가 순서를 정한다.** 그것은 시간이
+/// 아니다. `#126` 의 문장: *"사전순은 「최근」의 자가 아니다."*
+///
+/// ★ **시각을 못 얻는 회차는 「가장 최근」으로 본다** — fail-closed 다. `report.md` 가
+/// 아직 커밋되지 않은 회차는 **방금 끝난 것**이고, 뒤로 밀면 갓 끝난 회차가 하한의
+/// 대상에서 조용히 빠진다. 시각 없는 회차가 여럿이면 그중 **사전순 최대**를 고른다 —
+/// `read_dir` 순서에 기대면 기계마다 갈린다(ADR-0023).
+///
+/// ⚠ **후보를 여기서 거르지 않는다.** 「끝났나」의 자는 호출자가 진다(`report.md` 의
+/// 존재). 그 자를 두 곳에 적으면 갈린다.
+///
+/// [#126]: https://github.com/hskim-ecoletree/palimpsest/issues/126
+fn 최근에_끝난(끝난: &[String], 시각: impl Fn(&str) -> Option<i64>) -> Option<String> {
+    끝난
+        .iter()
+        .max_by_key(|회차| (시각(회차).unwrap_or(i64::MAX), (*회차).clone()))
+        .cloned()
+}
+
+/// 그 회차가 **끝난 시각** — `report.md` 를 마지막으로 만진 커밋의 커밋 시각(`%ct`).
+///
+/// 커밋이 없으면(추적 안 됨·방금 씀) `None` 이고, 그 처분은 [`최근에_끝난`] 이 진다.
+/// ⚠ **작성 시각(`%at`)이 아니라 커밋 시각(`%ct`)이다** — rebase·cherry-pick 이 작성
+/// 시각을 옛 값으로 들고 다니므로 그것으로 「최근」을 재면 이력을 고칠 때마다 뒤집힌다.
+fn 종료_커밋_시각(root: &Path, 회차: &str) -> Option<i64> {
+    let 경로 = format!("{회차_뿌리}/{회차}/report.md");
+    let out = std::process::Command::new("git")
+        .arg("-C")
+        .arg(root)
+        .args(["log", "-1", "--format=%ct", "--"])
+        .arg(&경로)
+        .output()
+        .ok()?;
+    if !out.status.success() {
+        return None;
+    }
+    String::from_utf8_lossy(&out.stdout).trim().parse::<i64>().ok()
+}
+
+#[cfg(test)]
+mod 최근_끝난_시험 {
+    use super::최근에_끝난;
+
+    fn 회차들(이름: &[&str]) -> Vec<String> {
+        이름.iter().map(|s| (*s).to_string()).collect()
+    }
+
+    /// ★ **RED 는 이것이었다** — 앞 판은 사전순 최대를 골랐다. 같은 날짜에 회차 둘이
+    /// 서고 **사전순이 시간순과 갈리면** 틀린 회차가 뽑힌다.
+    #[test]
+    fn 최근에_끝난_것은_사전순이_아니라_커밋_시각으로_고른다() {
+        let 목록 = 회차들(&["2026-01-01-a", "2026-01-01-b"]);
+        // `…-a` 가 **나중에** 끝났다. 사전순 최대는 `…-b` 다.
+        let 시각 = |회차: &str| match 회차 {
+            "2026-01-01-a" => Some(200),
+            "2026-01-01-b" => Some(100),
+            _ => None,
+        };
+        assert_eq!(최근에_끝난(&목록, 시각).as_deref(), Some("2026-01-01-a"));
+        // 음성 대조 — 사전순으로 고르면 이 값이 나온다. 그것이 앞 판의 답이다.
+        assert_ne!(목록.iter().max().map(String::as_str), Some("2026-01-01-a"));
+    }
+
+    /// `B6` 이 요구하는 **「커밋이 0 인 회차」의 처분**이다 — 가장 최근으로 본다.
+    #[test]
+    fn 커밋_시각이_없는_회차는_가장_최근으로_본다() {
+        let 목록 = 회차들(&["2026-01-01-a", "2026-09-09-z"]);
+        // `…-a` 는 커밋이 없다(방금 끝났다). `…-z` 는 시각이 있다.
+        let 시각 = |회차: &str| if 회차 == "2026-09-09-z" { Some(1_800_000_000) } else { None };
+        assert_eq!(최근에_끝난(&목록, 시각).as_deref(), Some("2026-01-01-a"));
+    }
+
+    /// 시각이 같으면(또는 둘 다 없으면) **사전순 최대**다 — 기계마다 갈리지 않게.
+    #[test]
+    fn 시각이_같으면_사전순_최대를_고른다() {
+        let 목록 = 회차들(&["2026-01-01-a", "2026-01-01-b", "2026-01-01-c"]);
+        assert_eq!(최근에_끝난(&목록, |_| Some(7)).as_deref(), Some("2026-01-01-c"));
+        assert_eq!(최근에_끝난(&목록, |_| None).as_deref(), Some("2026-01-01-c"));
+        // 끝난 회차가 없으면 `None` — 하한을 안 건다.
+        assert_eq!(최근에_끝난(&[], |_| Some(1)), None);
+    }
+}
+
 /// **원장 enum 이 두 자리에 적혀 있다 — 그 둘이 같은지 잰다.**
 ///
 /// 축 아홉(`출처`·`모집단`·`유효성`·`해악도`·`처분`·`승격됨`·`조건변경`·`사전처분`·`상태`)의
────────────────────────────────────────────────────────────────────────
## ④ §5.8 「메인이 이미 오염됐다」 표 전문 (`observations/red.md`)
────────────────────────────────────────────────────────────────────────
### 5.8 ⚠ 메인이 이미 오염됐다 — 사전부검이 `pal touch` 를 먼저 돌렸다

**사전 등록을 쓰기 전에** 사전부검 라운드 1 이 세 심볼에 `pal touch` 를 돌리고 **그 결과를
메인에 요약해 돌려줬다.** 숨기지 않고 여기 전수로 적는다. 이 목록에 있는 것은 **`C2` 의
귀속에서 뺀다** — touch 가 아니라 사전부검이 말해 준 것이기 때문이다.

| 심볼 | 메인이 이미 아는 것 |
|---|---|
| `nodes_of` | 결박 **0** · 지켜보는 것 **0** · 산출이 52 줄 |
| `identity_ceiling` | 결박 **0** · `pal touch` 의 **호출자 0** · `pal query symbol.callers` 가 **(없음)** · ⚠ **그 0 이 거짓 음성이고 실제 호출 자리가 넷**이라는 것(`grep` 으로 재었다). 까닭은 touch 가 스스로 적는 *"`x.foo()` 는 아직 안 셉니다"* |
| `check_ledger_pair` | 결박 **0** · 산출이 46 줄 · 같은 파일 `xtask/src/main.rs` 안에 결박 **8** 건이 있고 그중 하나가 **한 칸 옆**이다 |
| 둘 사이 | `nodes_of` 와 `check_ledger_pair` 의 산출이 **38 줄 동일**하다 |

★ **이것은 회차를 무르게 하지 않는다 — 기록으로 세운다.** 인터뷰 3 이 잠근 답이
*"평소대로 조사하고, **무엇을 봤는지 기록한다**"* 이고, 사전부검은 이 회차의 평소 절차다.
**그러나 ㉠ 의 값은 실제로 깎였다** — `identity_ceiling` 의 거짓 음성을 touch 가 아니라
사전부검이 먼저 말했다.
────────────────────────────────────────────────────────────────────────
## 제출된 귀속 후보
────────────────────────────────────────────────────────────────────────

### ⓓ
- 인용한 줄: `  fun · xtask/src/main.rs:5934 · identity ordinal · body 1f1846428f55`  (`:17`)
- 주장하는 사실: `check_ledger_pair` 의 **선언 자리가 `xtask/src/main.rs:5934`** 다.

### ⓔ
- 인용한 줄: `■ 이 좌표에 걸린 것 (0)` / `  아직 없습니다.`  (`:19-20`)
- 주장하는 사실: 이 심볼에 걸린 **결박이 0** 이다.

═════════ 프롬프트 끝 ═════════


════════════════════════════════════════════════════════════════════════
# `C3` 판정자 프롬프트 — ㉢ `#129` ⟨보존본 · 판 4⟩
════════════════════════════════════════════════════════════════════════

> 사전 등록 blob: `00c807cd4ecda4ca54ef22a6ffdc0e5e85e54f24`.

═════════ 여기서부터 프롬프트 ═════════

너는 **귀속 판정자**다. 아래 넷을 읽고 물음 하나에 답한다.

## 물음

제출된 **귀속 후보**마다, 그 후보가 인용한 줄이 담은 **사실**이
**① 사전 등록 전문에도 없고 ② §5.8 표에도 없는가.** 있으면 그 귀속은 **무효**다.

- 같은 낱말이 있는지가 아니라 **같은 사실이 있는지**를 본다.
- 사실이 부분적으로만 있으면 **어느 부분이 있고 어느 부분이 없는지** 적는다.
- **네가 스스로 후보를 더하지 마라.** 제출된 것만 판정한다.

## 반환 형식

후보마다 한 절씩:

    ### <후보 기호> — 유효 | 무효
    - 인용한 줄: <touch 산출의 그 줄>
    - 담은 사실: <한 문장>
    - 사전 등록에 있나: 있다(인용) | 없다
    - §5.8 표에 있나: 있다(인용) | 없다
    - 판정 근거: <두 문장 이내>

마지막에 `## 합계` 절을 두고 `유효 N · 무효 M` 을 적는다.

## 그리고 **받은 것을 되불러 준다** — 맨 앞에 `## 받은 것` 절을 둔다

아래 넷 각각에 대해 **첫 줄 · 마지막 줄 · 줄 수**를 적는다. 옮겨 적는 것이지 요약이
아니다. **이것이 네가 읽은 바이트를 사후에 대조하는 유일한 자리다** — 부르는 쪽이 그
셋을 파일과 기계로 댄다.

    ## 받은 것
    | 절 | 첫 줄 | 마지막 줄 | 줄 수 |
    |---|---|---|---|
    | ① 사전 등록 | … | … | N |
    | ② touch 산출 | … | … | N |
    | ③ diff | … | … | N |
    | ④ §5.8 표 | … | … | N |

⚠ **저장소를 읽지 마라.** 아래 인라인된 넷만으로 판정한다. 파일을 열거나 명령을
돌리지 않는다 — 네가 읽은 바이트가 사후에 복원되는 것이 이 판정의 조건이다.

────────────────────────────────────────────────────────────────────────
## ① 사전 등록 전문 (`plan/129-pre.md`)
────────────────────────────────────────────────────────────────────────
# 사전 등록 — ㉢ `#129`

> 회차 `2026-09-11-effect-confirmation` · 재는 자리 **㉢** · 이슈 **`#129`**
> **이 파일은 `pal touch` 를 돌리기 전에 봉인된다.**

## 0. 이 파일이 지는 조건

`A3` · `A4` · `A6` · `A2-c`(후보 목록이면 그 사실을 적는다) · `A5-d`·`A5-e`(앵커) ·
`B1-a`(RED 는 **이미 관측했다** — 아래 §2.4) · `B1-d`(GREEN) · `B1-e`(시험 ↔ RED) ·
`C2-b` 의 내용 시험 기준선.

★ 여기 적힌 사실은 `C2` 의 귀속에서 빠진다.
⚠ **이 자리는 「touch 가 쓸모없었다」가 나올 수 있는 음성 방향의 표본**으로 고른 자리다
(`intent.md ## 재는 자리 셋`). 그리고 **`C2` 의 귀속 모집단에 착수부터 0 으로 들어간다** —
후보 목록 가지에서는 사실 줄이 안 찍히기 때문이다.

## 1. 심볼 선정 ⟨`A6`⟩

**`write`** 하나. `pal touch write` 를 저장소 뿌리에서 `--at` 없이 부른다.
`#129` 의 뿌리가 `crates/pal-intent/src/store.rs:163-172` 의 `fn write` 이고,
그 함수가 `Handle::Absent | Handle::Reading(_)` 을 **둘 다** 거부한다.

⚠ **답이 후보 목록일 것으로 예상한다** ⟨`A2-c`⟩ — `write` 는 흔한 이름이라 여러 심볼이
맞을 것이다. 그러면 `fun · … body` 줄이 없고 `A5-b` 의 앵커가 **원리상 못 선다**
⟨`A5-e` ⓐ⟩. 산출을 보고 판정한다.

## 2. touch 를 돌리기 전에 무엇을 봤나 ⟨`A4`⟩

### 2.1 읽은 것
- 이슈 **`#129`** 전문 — 상태 `OPEN`. 재현 명령과 오류 문구, 그리고 *"광고된 질의 하나가
  어떤 입력으로도 안 돈다"*.
- `crates/pal-intent/src/store.rs:162-172` — `fn write` 가 `Handle::Writing` 에만 쓰기
  트랜잭션을 주고 `Absent` 와 `Reading` 을 거부한다. **파일이 없어도 같은 오류**다
  ⟨`CA1-01` 이 정정한 자리⟩.
- `crates/pal-cli/src/query.rs:109-128` — ⚠ **주석이 계약을 선언한다**:
  *"의도 저장소는 **읽기로만 연다** — 이 명령은 결박을 안 만든다."* 그리고 `:127` 이
  `IntentStore::open_read_only` 를 부른다.
- `crates/pal-cli/src/query.rs:166-169` — `NarrativeUnbound` 일 때만
  `crate::narrative::ingest(...)` 를 부른다. 주석: *"이 질의에서만 문서를 읽는다."*
- `crates/pal-cli/src/narrative.rs:130-220` `ingest` — 조각마다 `intent.entity_of(&origin)`
  을 묻고, **없으면 `EntityId::mint` 로 새 개체를 만들어 `intent.keep_entity(...)` 로
  남긴다**(`:187-201`). ★ **그 한 줄이 읽기 전용 저장소에 쓰는 자리**다.
- `crates/pal-cli/src/narrative.rs:112` — `pal narrative` 명령은 `IntentStore::open`(쓰기)
  으로 연다. **같은 `ingest` 를 두 표면이 부르는데 하나는 쓰기, 하나는 읽기다.**
- `crates/pal-query/src/lib.rs:150-162` `UnboundItem` — `item` 필드 주석이
  *"개체의 이름 — `decision/01J…`. **승인·거부가 이 이름으로 부른다**"* 다.
  **그래서 이름을 아무렇게나 지어 낼 수 없다** — 지속되지 않는 이름을 찍으면 그 이름으로
  승인하려다 실패한다.
- `crates/pal-core/src/envelope.rs:90-113` `ElisionReason` — 넷이 전부 **탐색 예산**의
  사유다(후보 넘침·경로 곱·깊이·노드). 「개체가 아직 없다」는 그 축이 아니다.
- `crates/pal-core/src/narrative.rs:476-481` `Proposal` — `item: EntityId` 로 **필수**다.
  `Option` 은 「선택 필드 금지 (1단계)」가 막는다.
- `crates/pal-cli/tests/query_envelope.rs:1-40` 과 `tests/common` — 통합 시험이 임시
  저장소에서 **바이너리를 실제로 돌린다**. ㉢ 의 시험이 설 자리다.

### 2.2 돌린 것
- `gh issue view 129`
- `./target/release/pal query narrative.unbound` — **RED 를 실제로 관측했다**(아래 §2.4)
- `./target/release/pal narrative --help`
- `grep -rn "narrative.unbound|IntentStore::|mint|keep_entity"` 계열

### 2.3 §5.8 표 전문 ⟨`observations/red.md` §5.8⟩

| 심볼 | 메인이 이미 아는 것 |
|---|---|
| `nodes_of` | 결박 **0** · 지켜보는 것 **0** · 산출이 52 줄 |
| `identity_ceiling` | 결박 **0** · `pal touch` 의 **호출자 0** · `pal query symbol.callers` 가 **(없음)** · ⚠ **그 0 이 거짓 음성이고 실제 호출 자리가 넷**이라는 것(`grep` 으로 재었다). 까닭은 touch 가 스스로 적는 *"`x.foo()` 는 아직 안 셉니다"* |
| `check_ledger_pair` | 결박 **0** · 산출이 46 줄 · 같은 파일 `xtask/src/main.rs` 안에 결박 **8** 건이 있고 그중 하나가 **한 칸 옆**이다 |
| 둘 사이 | `nodes_of` 와 `check_ledger_pair` 의 산출이 **38 줄 동일**하다 |

⚠ 이 표에 **`write` 는 없다** — 사전부검이 안 돌린 심볼이다.

### 2.4 RED 를 이미 관측했다 ⟨`B1-a`⟩ — `observations/red-129.txt`

`.palimpsest/intent.redb` 가 **있는 상태**(1,351,680 바이트 · 2026-09-08)에서 쟀다:

    $ ./target/release/pal query narrative.unbound
    Error: 개체를 남기지 못했다
    Caused by: 의도 저장소 트랜잭션이 실패했다: 읽기로 연 의도 저장소에 쓰려 했다
    rc=1

**종료값은 1 이다.** ⟨앞 판이 `MS-02` 에서 *"종료값 0"* 이라 적었고 그 뒤 `B1-b` 를 철회하며
거짓으로 판정한 자리다 — 이번 관측도 **1** 이다.⟩

### 2.5 이 회차의 앞 자리들이 이미 말해 준 것 — 귀속에서 뺀다
- `MS-06` — `크기` 줄의 바이트가 화면 본문과 다르다(㉡·㉠ 에서 두 번).
- `MS-08` — 「호출자 N」이 파일 간 미해소 몫을 표시 없이 뺀다(㉠).
- `MS-07`·`MS-09` — `identity ordinal` 이 원인을 안 가른다(㉡·㉠).

## 3. 두 갈래 중 고른 것

| 갈래 | 무엇 | 고르나 |
|---|---|---|
| ⓐ **질의를 쓰기로 연다** | `query.rs:127` 을 `IntentStore::open` 으로 바꾼다. 한 줄이다 | **아니다** |
| ⓑ **질의가 안 민팅한다** | `ingest` 에 민팅 스위치를 달고, 읽기 경로에서는 **개체가 없는 조각을 목록에서 빼고 그 수를 답에 싣는다** | **고른다** |

**까닭.** ⓐ 는 **코드가 그 자리에 적어 둔 계약을 깬다** — `query.rs:109` 이
*"의도 저장소는 읽기로만 연다 — 이 명령은 결박을 안 만든다"* 라고 선언한다. 질의가
민팅하면 **읽기가 `intent.redb` 를 불린다**. 그 저장소는 *"재구축 불가한 것의 유일한 복구
경로"*(`pal --help`)라 질의의 부작용으로 자라면 안 된다. 그리고 민팅은 **`pal narrative`
라는 제 표면이 이미 있다**.

⚠ **ⓑ 가 남기는 것을 미리 적는다** — 아직 `pal narrative` 를 안 지난 조각은 **목록에
안 나온다.** 그러므로 **수를 답에 싣는다**(`unminted`). 안 실으면 목록이 조용히 짧아지고,
그것이 이 저장소가 「거짓신호」라 부르는 형태다. 그리고 **이름을 지어내지 않는다** —
지속 안 되는 이름을 찍으면 승인이 실패한다(`UnboundItem.item` 주석).

## 4. 바꿀 좌표와 각 자리에 무엇을 쓰나 ⟨`A3`⟩

| # | 좌표 | 무엇을 쓰나 |
|---|---|---|
| ⑴ | `crates/pal-cli/src/narrative.rs` · `ingest` 앞 (새 자리) | `pub enum 민팅 { 한다, 안한다 }` — **읽기 표면과 쓰기 표면을 타입으로 가른다** |
| ⑵ | `crates/pal-cli/src/narrative.rs:130-135` | `ingest(..., 민팅: 민팅)` 로 받는다 |
| ⑶ | `crates/pal-cli/src/narrative.rs:186-203` | `entity_of` 가 `None` 일 때 **민팅::안한다면 `keep_entity` 를 안 부르고** `개체_없음 += 1` 하고 `continue` 한다 |
| ⑷ | `crates/pal-cli/src/narrative.rs` · `Ingested`(`:60-80` 부근) | 필드 `개체_없음: usize` 를 더한다 |
| ⑸ | `crates/pal-cli/src/query.rs:166-169` | `민팅::안한다` 를 넘기고 `개체_없음` 을 받아 `QueryCtx` 로 올린다 |
| ⑹ | `crates/pal-query/src/lib.rs` · `QueryCtx` · `QueryResult::Narrative` | `unminted: usize` 를 싣는다 — **답이 자기가 뺀 것을 진다** |
| ⑺ | `crates/pal-cli/src/query.rs` · `print_narrative` | `unminted` 를 화면에 적는다. 0 이면 0 이라 적는다 |
| ⑻ | `crates/pal-cli/src/narrative.rs:112` 쪽 호출 | `민팅::한다` — **기존 행동 그대로** |
| ⑼ | `crates/pal-cli/tests/` (새 파일 또는 기존) | 아래 §5 의 시험 |

⚠ **안 건드리는 것**: `store.rs` 의 `write()` 거부 규칙(그것이 옳다) · `pal narrative` 의
행동 · `EntityId` 생성 방식 · `ElisionReason` enum.

## 5. 시험과 그것이 재는 RED ⟨`B1-c`·`B1-e`⟩

| 시험 이름 | 재는 RED |
|---|---|
| `narrative_unbound_는_읽기로도_돈다` | **지금은 어떤 입력으로도 안 돈다** — 임시 저장소에 조각이 있는 문서를 두고 `pal query narrative.unbound` 를 돌리면 지금은 `rc=1` 에 *"읽기로 연 의도 저장소에 쓰려 했다"* 가 난다. 고친 뒤에는 **rc 0** 이고 `미결박`·`개체 없음` 이 화면에 뜬다 ⟨`B1-d` 와 같은 자⟩ |
| `읽기_경로는_의도_저장소를_안_불린다` | **민팅이 질의의 부작용으로 남는 것.** 질의 전후로 `.palimpsest/intent.redb` 의 **바이트가 같아야** 한다(파일이 없으면 없는 채로). 안 재면 ⓐ 로 슬쩍 고쳐도 첫 시험이 초록이다 |

★ **음성 대조** — ⑶ 의 분기를 **끄면**(안한다에서도 민팅하면) 첫 시험이 다시 빨개진다.
실제로 돌려 `effect/129-delta.md` 에 적는다.

## 6. 앵커 ⟨`A5-d`·`A5-e`⟩

- **touch 를 건 `write` 는 변경 대상이 아니다** — §4 의 「안 건드리는 것」에 `store.rs` 의
  `write()` 가 들어 있다. 그러므로 `A5-d` 의 **뒤쪽 갈래**가 걸린다: 그 사실과 까닭을 여기
  적고 **`A5-b` 를 「대조불가」로 판정한다.**
- ⚠ 그리고 답이 **후보 목록**이면 `body` 줄 자체가 없어 `A5-e` ⓐ 도 함께 걸린다.

## 7. 이 사전 등록이 예상하는 것

- `write` 는 흔한 이름이라 **후보 목록**이 나올 것이다(3 건 안팎).
- 그러면 **touch 가 이 자리에 줄 것이 거의 없다** — 「후보가 여럿입니다」와 좌표 목록뿐이다.
- **차이 0 이 나오면 0 으로 적는다** ⟨`C5`⟩. 이 회차는 그 경우를 **음성 방향의 표본**으로
  미리 골라 뒀다.
────────────────────────────────────────────────────────────────────────
## ② `pal touch` 산출 전문 (`touch/129.txt`)
────────────────────────────────────────────────────────────────────────
# `pal touch` 산출 — ㉢ `#129`
#
# 부른 명령      ./target/release/pal touch write
#                (저장소 뿌리에서 · `--at` 없음 · `--repo` 기본값 `.`)
# HEAD           22cf4889261ca69938f40c22002bfa96f24d0f6e  ⟨봉인 커밋 = plan/129-pre.md⟩
# 심볼 ID        **원리상 없다** ⟨`A2-c`⟩ — 답이 **후보 목록**이라 둘째 줄이
#                「`write` 의 후보가 3건입니다」 이고 `palimpsest@…#<hex>` 가 안 찍힌다.
#                근거 줄도 `Snapshot palimpsest@22cf488+worktree` 로 `#<hex>` 가 없다.
# 사전등록-blob  00c807cd4ecda4ca54ef22a6ffdc0e5e85e54f24  ⟨git rev-parse 22cf488:….../plan/129-pre.md⟩
#
# 종료값 0 · 표준오류 0 바이트 · 워킹트리 깨끗 · 산출 21 줄 · **1201 바이트**
# 본문 sha256   0766ec2faa24ca44990f1cc14d07d61976ad1c6bf8ca319cfbb314f764531bc3
# ⚠ 산출 안의 `크기 … 잰 것: 2152 바이트` ↔ 본문 1201 바이트. `MS-06` 의 **셋째 관측**이고
#   방향이 같다(잰 값 > 화면). ㉡ 4108↔3346 · ㉠ 7749↔4066 · ㉢ 2152↔1201.
#
# ★ **`A5-b` 의 앵커가 원리상 못 선다** ⟨`A5-e` ⓐ⟩ — `fun · … body <hash>` 줄이 없다.
--- 전 출력 (여기서부터 바이트 그대로 · 위는 머리) ---

  `write` 의 후보가 3건입니다. 하나를 고르지 않습니다.

  fun        write                    crates/pal-cli/src/install/manifest.rs:373
  fun        write                    crates/pal-intent/src/store.rs:163
  fun        write                    crates/pal-store/src/projection.rs:251

■ 이 답의 근거
  Snapshot  palimpsest@22cf488+worktree
  대장      parsed 141 · partial 0 · unsupported 781 · unrecognized 350 / 1273 파일
            결박 불가 언어 7개 — 그 파일들에는 좌표가 없습니다
  2층       심볼 3325 색인됨
  워킹트리  일치
  재구축    아님
  생략      없음 (명시)
  이관      1273건 — 본체를 다른 질의로 옮겼습니다. 생략된 것이 아닙니다
            ledger 1273건 → `ledger.snapshot` 로 조회할 수 있습니다
  질의 로그  남았습니다
  크기      약 538 토큰 **이상** (잰 것: 2152 바이트 · 가정: 4 바이트/토큰)
  능력      ledger.snapshot · symbol.resolve · symbol.contains · symbol.callers · symbol.reaches · graph.dump · binding.status · narrative.unbound · binding.touch · plan.deviation · symbol.references · 미구축 F13 · F15

────────────────────────────────────────────────────────────────────────
## ③ 실제 변경 diff (`git diff 22cf488..1abc0e9 -- crates/`)
────────────────────────────────────────────────────────────────────────
diff --git a/crates/pal-cli/src/narrative.rs b/crates/pal-cli/src/narrative.rs
index 24a7531..90668d3 100644
--- a/crates/pal-cli/src/narrative.rs
+++ b/crates/pal-cli/src/narrative.rs
@@ -63,6 +63,29 @@ pub enum What<'a> {
     Refuse { item: &'a str, pick: &'a str, reason: &'a str },
 }
 
+/// **민팅을 하는 표면인가** — 읽기 표면과 쓰기 표면을 타입으로 가른다. ([#129])
+///
+/// # 왜 불리언이 아닌가
+///
+/// `ingest(…, true)` 는 부르는 자리에서 **무엇이 참인지 안 읽힌다.** 이 스위치가 가르는
+/// 것은 *"이 표면이 의도 저장소를 불려도 되는가"* 이고, 그것은 두 표면의 **계약**이다.
+///
+/// # 두 표면이 같은 `ingest` 를 부른다
+///
+/// `pal narrative` 는 쓰기로 열고([`run`]), `pal query narrative.unbound` 는 **읽기로
+/// 연다** — `query.rs` 가 그 자리에 *"의도 저장소는 읽기로만 연다 — 이 명령은 결박을 안
+/// 만든다"* 라고 적어 두었다. 그런데 `ingest` 는 개체가 없으면 **민팅해서 남겼고**,
+/// 그래서 광고된 질의 하나가 **어떤 입력으로도 안 돌았다**([#129]).
+///
+/// [#129]: https://github.com/hskim-ecoletree/palimpsest/issues/129
+#[derive(Debug, Clone, Copy, PartialEq, Eq)]
+pub enum 민팅 {
+    /// 개체가 없으면 만들어 남긴다 — `pal narrative` 의 계약.
+    한다,
+    /// **아무것도 안 남긴다.** 개체가 없는 조각은 목록에서 빼고 수만 헤아린다 — 질의의 계약.
+    안한다,
+}
+
 /// 인입 한 회차의 산출 — **건수가 아니라 회계다.**
 pub struct Ingested {
     pub proposals: Vec<Proposal>,
@@ -75,9 +98,31 @@ pub struct Ingested {
     pub history_window: usize,
     /// 그 창 안에서 마지막 변경을 못 찾은 문서 수.
     pub outside_window: usize,
+    /// [`민팅::안한다`] 라서 **목록에서 뺀** 조각 수 — 아직 개체가 없는 것들이다. ([#129])
+    ///
+    /// ★ **0 이 아닌 값을 침묵으로 두지 않는다.** 이 수를 안 실으면 읽기 표면의 목록이
+    /// **조용히 짧아지고**, 보는 사람은 그것을 *"미결박이 그만큼뿐"* 으로 읽는다.
+    /// 그 조각들은 `pal narrative` 를 한 번 돌리면 이름을 받는다.
+    pub 개체_없음: usize,
 }
 
 impl Ingested {
+    /// **묻지 않은 질의의 값** — 빈 인입. ([#129])
+    ///
+    /// `Vec::new()` 를 세 자리에 흩어 두면 *"안 물었다"* 와 *"물었는데 0"* 이 같은 글자가
+    /// 된다. 이름을 붙여 그 구별을 부르는 자리에 남긴다.
+    pub const fn 비어_있다() -> Self {
+        Self {
+            proposals: Vec::new(),
+            docs: 0,
+            fragments: 0,
+            minted: 0,
+            history_window: 0,
+            outside_window: 0,
+            개체_없음: 0,
+        }
+    }
+
     /// 분류별 건수 — **셋이 전부 실린다.** 하나라도 0 이면 그 사실이 보인다.
     #[must_use]
     pub fn counts(&self) -> BTreeMap<&'static str, usize> {
@@ -112,7 +157,8 @@ pub fn run(a: Args) -> Result<()> {
     let intent = IntentStore::open(&touch::intent_file(a.repo, a.intent))
         .context("의도 저장소를 열지 못했다")?;
 
-    let got = ingest(a.repo, &report, &attached.projection, &intent)?;
+    // **쓰기 표면이다** — 위에서 `IntentStore::open`(쓰기)으로 열었다 ([#129]).
+    let got = ingest(a.repo, &report, &attached.projection, &intent, 민팅::한다)?;
 
     match a.what {
         What::Ingest => 화면(&got, a.json),
@@ -132,6 +178,7 @@ pub fn ingest(
     report: &ledger::LedgerReport,
     projection: &pal_store::Projection,
     intent: &IntentStore,
+    민팅: 민팅,
 ) -> Result<Ingested> {
     let git = GixRepo::open(repo).context("저장소를 열지 못했다")?;
     let at = &report.ledger.snapshot_tree();
@@ -146,6 +193,7 @@ pub fn ingest(
     let mut docs = 0;
     let mut fragments = 0;
     let mut minted = 0;
+    let mut 개체_없음 = 0;
     let mut outside = 0;
 
     for entry in &report.ledger.entries {
@@ -187,6 +235,13 @@ pub fn ingest(
             let 이미 = intent.entity_of(&origin).context("개체를 읽지 못했다")?;
             let item = if let Some(id) = 이미 {
                 id
+            } else if 민팅 == self::민팅::안한다 {
+                // ★ **읽기 표면은 이름을 지어내지 않는다** ([#129]).
+                //   `UnboundItem::item` 은 *"승인·거부가 이 이름으로 부른다"* 라 적혀 있고,
+                //   지속되지 않는 이름을 찍으면 그 이름으로 승인하려다 실패한다.
+                //   **빼되 세고**, 답이 그 수를 싣는다.
+                개체_없음 += 1;
+                continue;
             } else {
                     // ★ **민팅은 처음 한 번뿐이다.** 매번 뽑으면 같은 문서를 두 번 읽을 때
                     //   개체가 둘이 되고, **읽기가 더하기가 아니라 복제가 된다**.
@@ -218,6 +273,7 @@ pub fn ingest(
         docs,
         fragments,
         minted,
+        개체_없음,
         history_window: PROVISIONAL_HISTORY_BUDGET,
         outside_window: outside,
     })
diff --git a/crates/pal-cli/src/query.rs b/crates/pal-cli/src/query.rs
index 3ab2b33..bed4954 100644
--- a/crates/pal-cli/src/query.rs
+++ b/crates/pal-cli/src/query.rs
@@ -140,6 +140,25 @@ pub fn answer(a: &Args, query: &NamedQuery) -> Result<Envelope<QueryResult>> {
         - counts.get(&pal_core::Bucket::Parsed).copied().unwrap_or(0)
         - counts.get(&pal_core::Bucket::Partial).copied().unwrap_or(0);
 
+    // **이 질의에서만 문서를 읽는다.** 다른 질의에서 비어 있는 것은 *"미결박이 0"* 이
+    // 아니라 *"안 물었다"* 이고, 그 구별이 `QueryCtx::narrative` 의 머리에 적혀 있다.
+    // 인입은 저장소 전체의 문서를 읽으므로 **묻지 않은 질의에 그 비용을 지우지 않는다.**
+    //
+    // ★ **`민팅::안한다` 다** ([#129]). 위에서 의도 저장소를 **읽기로** 열었고, 그
+    //   계약대로 이 경로는 개체를 **안 만든다**. 앞 판은 여기서 민팅해서 *"읽기로 연
+    //   의도 저장소에 쓰려 했다"* 로 **어떤 입력으로도 안 돌았다.**
+    let 인입 = if matches!(query, NamedQuery::NarrativeUnbound) {
+        crate::narrative::ingest(
+            a.repo,
+            &report,
+            &projection,
+            &intent,
+            crate::narrative::민팅::안한다,
+        )?
+    } else {
+        crate::narrative::Ingested::비어_있다()
+    };
+
     let ctx = QueryCtx {
         projection: &projection,
         snapshot: report.ledger.snapshot.clone(),
@@ -163,11 +182,10 @@ pub fn answer(a: &Args, query: &NamedQuery) -> Result<Envelope<QueryResult>> {
         // 0"* 이 아니라 *"안 물었다"* 이고, 그 구별이 `QueryCtx::narrative` 의 머리에
         // 적혀 있다. 인입은 저장소 전체의 문서를 읽으므로 **묻지 않은 질의에 그 비용을
         // 지우지 않는다.**
-        narrative: if matches!(query, NamedQuery::NarrativeUnbound) {
-            crate::narrative::ingest(a.repo, &report, &projection, &intent)?.proposals
-        } else {
-            Vec::new()
-        },
+        narrative: 인입.proposals,
+        // **답이 자기가 뺀 것을 진다** ([#129]). 읽기 표면이라 개체를 안 만들고,
+        // 그래서 이름 없는 조각은 목록에 안 실린다 — 그 수를 여기로 올린다.
+        narrative_unminted: 인입.개체_없음,
         bindings,
         // ★ **계산은 표면의 일이다** — 이탈은 **두 스냅샷**을 요구하는데 `QueryCtx` 는
         // 투영 하나만 든다. `narrative` 와 같은 자리이고 이유가 하나 더 있다.
@@ -302,8 +320,8 @@ fn print_screen(q: &NamedQuery, e: &Envelope<QueryResult>) {
         QueryResult::Bindings { bindings, detector, store } => {
             print_bindings(bindings, detector, store);
         }
-        QueryResult::Narrative { unbound, candidates, bound, candidate_sizes } => {
-            print_narrative(unbound, *candidates, *bound, candidate_sizes);
+        QueryResult::Narrative { unbound, candidates, bound, unminted, candidate_sizes } => {
+            print_narrative(unbound, *candidates, *bound, *unminted, candidate_sizes);
         }
         QueryResult::Ambiguous { name, candidates } => {
             println!("  `{name}` 의 후보가 {}건입니다. 하나를 고르지 않습니다.", candidates.len());
@@ -476,9 +494,16 @@ fn print_narrative(
     unbound: &[pal_query::UnboundItem],
     candidates: usize,
     bound: usize,
+    unminted: usize,
     spread: &[pal_query::CandidateSpread],
 ) {
     println!("  결박됨 {bound} · 후보 있음 {candidates} · **미결박 {}**", unbound.len());
+    // ★ **뺀 것을 같은 줄 아래에 적는다** ([#129]). 이 질의는 읽기라 개체를 안 만들고,
+    //   이름이 없는 조각은 **부를 수가 없어** 목록에 안 실린다. 0 이어도 적는다 —
+    //   *"뺀 것이 없다"* 와 *"그 축을 안 본다"* 는 다르다.
+    println!(
+        "  이름이 아직 없어 뺀 조각 **{unminted}** — `pal narrative` 를 한 번 돌리면 이름이 섭니다"
+    );
     println!();
     if !spread.is_empty() {
         // ★ **수만 내면 「후보 있음 1,563」이 「승인 대기 1,563 건」으로 읽힌다.**
diff --git a/crates/pal-cli/src/touch.rs b/crates/pal-cli/src/touch.rs
index 89457ee..683e81a 100644
--- a/crates/pal-cli/src/touch.rs
+++ b/crates/pal-cli/src/touch.rs
@@ -149,6 +149,8 @@ pub fn run(a: Args) -> Result<()> {
 
     let ctx = QueryCtx {
         projection: &projection,
+        // **`pal touch` 는 인입을 안 부른다** — 그래서 뺀 것도 0 이다 ([#129]).
+        narrative_unminted: 0,
         snapshot: report.ledger.snapshot.clone(),
         ledger: pal_core::LedgerRef::of(&report.ledger),
         freshness: pal_query::freshness(
diff --git a/crates/pal-cli/tests/narrative_read_only.rs b/crates/pal-cli/tests/narrative_read_only.rs
new file mode 100644
index 0000000..61a7c0d
--- /dev/null
+++ b/crates/pal-cli/tests/narrative_read_only.rs
@@ -0,0 +1,99 @@
+//! **읽기 표면은 의도 저장소를 안 불린다** — [#129].
+//!
+//! `pal query narrative.unbound` 는 `IntentStore::open_read_only` 로 열면서도 인입이
+//! 개체를 **민팅해서 남기려** 했고, 그래서 *"읽기로 연 의도 저장소에 쓰려 했다"* 로
+//! **어떤 입력으로도 안 돌았다.** 광고된 질의 하나가 통째로 죽은 자리다.
+//!
+//! # 시험 둘이 서로를 받친다
+//!
+//! 첫째만 두면 **질의를 쓰기로 여는 고침**(한 줄)으로도 초록이 된다. 그 고침은
+//! `query.rs` 가 스스로 적어 둔 계약(*"의도 저장소는 읽기로만 연다"*)을 깨고, **읽기가
+//! `intent.redb` 를 불리게** 만든다. 둘째가 그 길을 막는다.
+//!
+//! [#129]: https://github.com/hskim-ecoletree/palimpsest/issues/129
+
+mod common;
+
+use common::{git, PAL};
+use std::path::{Path, PathBuf};
+use std::process::Command;
+
+/// 문서 조각이 **실재하는** 저장소. 조각이 없으면 민팅할 것도 없어 이 시험이
+/// 아무것도 안 잰다.
+fn 저장소(tag: &str) -> PathBuf {
+    let root = std::env::temp_dir().join(format!("pal-129-{tag}-{}", std::process::id()));
+    let _ = std::fs::remove_dir_all(&root);
+    std::fs::create_dir_all(root.join("docs")).expect("임시 저장소");
+    std::fs::write(root.join("alpha.ts"), "export function 도움() { return 1 }\n")
+        .expect("alpha.ts");
+    std::fs::write(
+        root.join("docs/결정.md"),
+        "# 결정 하나\n\n`도움` 을 남긴다. 이 문단이 조각이 된다.\n\n## 둘째 결정\n\n또 하나.\n",
+    )
+    .expect("결정.md");
+    git(&root, &["init", "-q", "."]);
+    git(&root, &["add", "-A"]);
+    git(&root, &["-c", "user.email=t@example.com", "-c", "user.name=t", "commit", "-qm", "첫"]);
+    root
+}
+
+fn 돌린다(cwd: &Path, args: &[&str]) -> (bool, String, String) {
+    let out = Command::new(PAL).args(args).current_dir(cwd).output().expect("pal 을 못 돌렸다");
+    (
+        out.status.success(),
+        String::from_utf8_lossy(&out.stdout).into_owned(),
+        String::from_utf8_lossy(&out.stderr).into_owned(),
+    )
+}
+
+/// ★ **RED 는 이것이었다** — `rc=1` · *"읽기로 연 의도 저장소에 쓰려 했다"*.
+#[test]
+fn narrative_unbound_는_읽기로도_돈다() {
+    let repo = 저장소("runs");
+    let (ok, out, err) = 돌린다(&repo, &["query", "narrative.unbound"]);
+    assert!(ok, "질의가 실패했다\nstdout: {out}\nstderr: {err}");
+    assert!(
+        !err.contains("읽기로 연 의도 저장소에 쓰려 했다"),
+        "#129 의 그 오류가 그대로다: {err}"
+    );
+    // **답이 자기가 뺀 것을 진다** — 0 이어도 줄이 있어야 한다.
+    assert!(
+        out.contains("이름이 아직 없어 뺀 조각"),
+        "뺀 조각 수가 답에 없다 — 목록이 조용히 짧아진다\n{out}"
+    );
+    assert!(out.contains("미결박"), "미결박 줄이 없다\n{out}");
+    let _ = std::fs::remove_dir_all(&repo);
+}
+
+/// ★★ **이 시험이 「쓰기로 열면 된다」는 길을 막는다.**
+///
+/// 질의를 돌려도 의도 저장소가 **생기지 않아야** 한다. 생기면 읽기가 쓰기가 된 것이고,
+/// `query.rs` 가 그 자리에 적어 둔 계약이 거짓이 된다.
+#[test]
+fn 읽기_경로는_의도_저장소를_안_불린다() {
+    let repo = 저장소("readonly");
+    let 의도 = repo.join(".palimpsest/intent.redb");
+    assert!(!의도.exists(), "시작 상태가 이미 틀렸다");
+
+    let (ok, _, err) = 돌린다(&repo, &["query", "narrative.unbound"]);
+    assert!(ok, "질의가 실패했다: {err}");
+    assert!(
+        !의도.exists(),
+        "질의가 의도 저장소를 **만들었다** — 읽기 표면이 쓰고 있다"
+    );
+
+    // 그리고 저장소가 **있을 때**도 바이트가 안 움직여야 한다.
+    // `pal narrative` 로 한 번 세우고(그쪽은 쓰기 표면이 맞다) 그 뒤 질의를 돌린다.
+    let (세웠나, _, err2) = 돌린다(&repo, &["narrative"]);
+    assert!(세웠나, "`pal narrative` 가 실패했다: {err2}");
+    assert!(의도.exists(), "쓰기 표면이 저장소를 안 세웠다");
+    let 전 = std::fs::read(&의도).expect("읽기");
+
+    let (ok2, _, err3) = 돌린다(&repo, &["query", "narrative.unbound"]);
+    assert!(ok2, "둘째 질의가 실패했다: {err3}");
+    let 후 = std::fs::read(&의도).expect("읽기");
+    assert_eq!(전.len(), 후.len(), "질의 뒤 의도 저장소의 크기가 움직였다");
+    assert!(전 == 후, "질의 뒤 의도 저장소의 바이트가 움직였다");
+
+    let _ = std::fs::remove_dir_all(&repo);
+}
diff --git a/crates/pal-query/src/lib.rs b/crates/pal-query/src/lib.rs
index d86dec0..f9df88b 100644
--- a/crates/pal-query/src/lib.rs
+++ b/crates/pal-query/src/lib.rs
@@ -216,6 +216,15 @@ pub enum QueryResult {
         unbound: Vec<UnboundItem>,
         candidates: usize,
         bound: usize,
+        /// **답이 목록에서 뺀 조각 수** — 아직 개체 이름이 없어서다. ([#129])
+        ///
+        /// 읽기 표면은 개체를 **안 만든다**. 그래서 `pal narrative` 를 한 번도 안 지난
+        /// 조각은 부를 이름이 없고, 이 답은 그것을 **목록에 안 싣는다.**
+        ///
+        /// ★ **그 수를 여기 싣는 것이 이 필드의 전부다.** 안 실으면 목록이 조용히
+        /// 짧아지고 보는 사람은 *"미결박이 그만큼뿐"* 으로 읽는다 — 이 저장소가
+        /// 「거짓신호」라 부르는 형태다.
+        unminted: usize,
         /// ★ **후보가 몇 개짜리인가** — 신호별로.
         ///
         /// # 왜 수만으로는 거짓말이 되는가 (F10 실측 · 2026-08-15)
@@ -320,6 +329,10 @@ pub struct QueryCtx<'a> {
     /// **`narrative.unbound` 가 아닌 질의에서는 비어 있고, 그것이 정확한 값이다** —
     /// 문서를 안 읽었으므로 *"미결박이 0"* 이 아니라 *"안 물었다"* 다.
     pub narrative: Vec<pal_core::Proposal>,
+    /// 인입이 **이름이 없어 뺀** 조각 수 ([#129]). 읽기 표면에서만 0 이 아니다.
+    ///
+    /// [#129]: https://github.com/hskim-ecoletree/palimpsest/issues/129
+    pub narrative_unminted: usize,
     /// 이 저장소의 결박 전부 — **부르는 쪽이 지고 온다.**
     ///
     /// # 왜 이 크레이트가 `pal-intent` 에 의존하지 않는가
@@ -483,6 +496,7 @@ fn 미결박(ctx: &QueryCtx, accessed: &mut Vec<SymbolId>) -> QueryResult {
         unbound,
         candidates,
         bound,
+        unminted: ctx.narrative_unminted,
         candidate_sizes: 후보_퍼짐(&ctx.narrative),
     }
 }
────────────────────────────────────────────────────────────────────────
## ④ §5.8 「메인이 이미 오염됐다」 표 전문 (`observations/red.md`)
────────────────────────────────────────────────────────────────────────
### 5.8 ⚠ 메인이 이미 오염됐다 — 사전부검이 `pal touch` 를 먼저 돌렸다

**사전 등록을 쓰기 전에** 사전부검 라운드 1 이 세 심볼에 `pal touch` 를 돌리고 **그 결과를
메인에 요약해 돌려줬다.** 숨기지 않고 여기 전수로 적는다. 이 목록에 있는 것은 **`C2` 의
귀속에서 뺀다** — touch 가 아니라 사전부검이 말해 준 것이기 때문이다.

| 심볼 | 메인이 이미 아는 것 |
|---|---|
| `nodes_of` | 결박 **0** · 지켜보는 것 **0** · 산출이 52 줄 |
| `identity_ceiling` | 결박 **0** · `pal touch` 의 **호출자 0** · `pal query symbol.callers` 가 **(없음)** · ⚠ **그 0 이 거짓 음성이고 실제 호출 자리가 넷**이라는 것(`grep` 으로 재었다). 까닭은 touch 가 스스로 적는 *"`x.foo()` 는 아직 안 셉니다"* |
| `check_ledger_pair` | 결박 **0** · 산출이 46 줄 · 같은 파일 `xtask/src/main.rs` 안에 결박 **8** 건이 있고 그중 하나가 **한 칸 옆**이다 |
| 둘 사이 | `nodes_of` 와 `check_ledger_pair` 의 산출이 **38 줄 동일**하다 |

★ **이것은 회차를 무르게 하지 않는다 — 기록으로 세운다.** 인터뷰 3 이 잠근 답이
*"평소대로 조사하고, **무엇을 봤는지 기록한다**"* 이고, 사전부검은 이 회차의 평소 절차다.
**그러나 ㉠ 의 값은 실제로 깎였다** — `identity_ceiling` 의 거짓 음성을 touch 가 아니라
사전부검이 먼저 말했다.
────────────────────────────────────────────────────────────────────────
## 제출된 귀속 후보
────────────────────────────────────────────────────────────────────────

### ⓐ
- 인용한 줄: `  fun        write                    crates/pal-cli/src/install/manifest.rs:373` 과
  `  fun        write                    crates/pal-store/src/projection.rs:251`  (`:21`·`:23`)
- 주장하는 사실: `write` 라는 이름의 함수가 **셋**이고 그중 둘이 `install/manifest.rs:373` ·
  `pal-store/src/projection.rs:251` 이다.

═════════ 프롬프트 끝 ═════════
