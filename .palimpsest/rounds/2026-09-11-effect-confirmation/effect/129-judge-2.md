# 귀속 판정 원문 — ㉢ `#129` · 인스턴스 2 ⟨`C3-b`⟩ · **전 diff 로 다시**

> 인스턴스 1 의 ③ 이 **네 파일 중 둘**뿐이어서 다시 띄웠다. 이 판의 ③ 은 **전문**이고,
> 판정자가 되불러 준 `diff --git` **다섯 줄이 순서까지 파일과 같다**(아래).
> 판정자: `general-purpose` · sonnet · 도구 사용 **0 회**. 프롬프트는 `judge-prompt.md` 판 5.

---

## 받은 것

**① 사전 등록 전문 (`plan/129-pre.md`)**
- 첫 줄: `# 사전 등록 — ㉢ #129`
- 마지막 줄: `- **차이 0 이 나오면 0 으로 적는다** ⟨C5⟩. 이 회차는 그 경우를 **음성 방향의 표본**으로 미리 골라 뒀다.`

**② `pal touch` 산출 전문 (`touch/129.txt`)**
- 첫 줄: `# `pal touch` 산출 — ㉢ #129`
- 마지막 줄: `  능력      ledger.snapshot · symbol.resolve · symbol.contains · symbol.callers · symbol.reaches · graph.dump · binding.status · narrative.unbound · binding.touch · plan.deviation · symbol.references · 미구축 F13 · F15`

**③ 실제 변경 diff (`git diff 22cf488..1abc0e9 -- crates/`)**
- 첫 줄: `diff --git a/crates/pal-cli/src/narrative.rs b/crates/pal-cli/src/narrative.rs`
- 마지막 줄: `}` (`fn 미결박` 닫는 줄)
- `diff --git` 으로 시작하는 줄 전부 (순서대로):
  1. `diff --git a/crates/pal-cli/src/narrative.rs b/crates/pal-cli/src/narrative.rs`
  2. `diff --git a/crates/pal-cli/src/query.rs b/crates/pal-cli/src/query.rs`
  3. `diff --git a/crates/pal-cli/src/touch.rs b/crates/pal-cli/src/touch.rs`
  4. `diff --git a/crates/pal-cli/tests/narrative_read_only.rs b/crates/pal-cli/tests/narrative_read_only.rs`
  5. `diff --git a/crates/pal-query/src/lib.rs b/crates/pal-query/src/lib.rs`

**④ §5.8 「메인이 이미 오염됐다」 표 전문 (`observations/red.md`)**
- 첫 줄: `### 5.8 ⚠ 메인이 이미 오염됐다 — 사전부검이 `pal touch` 를 먼저 돌렸다`
- 마지막 줄: `**그러나 ㉠ 의 값은 실제로 깎였다** — `identity_ceiling` 의 거짓 음성을 touch 가 아니라 사전부검이 먼저 말했다.`

---

### ⓐ — 유효

- 인용한 줄: `fun write crates/pal-cli/src/install/manifest.rs:373` 과 `fun write crates/pal-store/src/projection.rs:251`
- 담은 사실: `write` 라는 이름의 함수가 셋이고, 그중 둘의 구체적 좌표가 `install/manifest.rs:373` 과 `pal-store/src/projection.rs:251` 이다.
- 사전 등록에 있나: 없다 — §1·§7 이 *"후보 목록이 나올 것이다(3 건 안팎)"* 라 **예측**만 했고, 구체적 좌표로는 `store.rs:163-172` 의 `write` 하나만 §1에 적었다. `install/manifest.rs:373` 과 `projection.rs:251` 이라는 두 좌표는 어디에도 없다.
- §5.8 표에 있나: 없다 — 표는 `nodes_of` · `identity_ceiling` · `check_ledger_pair` 만 다루고, 사전 등록 §2.3 스스로 *"이 표에 `write` 는 없다"* 고 적어 둔다.
- 판정 근거: 개수(3건 안팎)는 예측된 범위지만 그것은 "이미 아는 사실"이 아니라 형태에 대한 기대였고, 인용된 두 구체 좌표는 두 원천 어디에도 나오지 않는 새 정보다. 따라서 이 귀속이 지는 사실은 사전 등록·§5.8 표 밖의 것이므로 유효하다.

## 합계

유효 1 · 무효 0

---

## 부르는 쪽이 적는 것

★ **두 인스턴스가 같은 판정을 냈다**(유효 1 · 무효 0) — ③ 이 잘린 판과 전문인 판에서
결론이 같다. 그것은 **③ 이 이 판정의 근거가 아니었다**는 뜻이고, `C3` 이 넷을 요구하는
까닭이 「판정에 쓰이기 때문」이 아니라 **「판정자가 본 바이트를 복원할 수 있게」** 이기
때문임을 보여 준다.

★★ **그리고 「유효 1」은 「귀속된 차이 1」이 아니다.** `effect/129-delta.md` 가 차이 셋을
적고 셋 다 원인이 **도구**(컴파일러·검사)라고 적는다. `C2` 는 *"그 **차이** 중 touch 에
귀속되는 것"* 을 묻고, ㉢ 에서 그 수는 **0** 이다. 이 판이 세운 것은 *"touch 가 새 사실을
주긴 했다"* 이고, **그 새 사실이 아무 차이도 안 만들었다**는 것이 이 자리의 결론이다.
