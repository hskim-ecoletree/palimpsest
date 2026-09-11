# 귀속 판정 원문 — ㉢ `#129` · 인스턴스 1 ⟨`C3-b`⟩

> ⚠⚠ **이 판의 ③ 은 잘려 있었다 — 부르는 쪽의 잘못이다.**
> `C3` 이 *"넷 다 바이트로 그대로 싣는다 — **자르거나 요약하면**…"* ⟨`CA2-09`⟩ 을 못
> 박았는데, 이 인스턴스에 준 ③ 은 **네 파일 중 앞의 둘**(`narrative.rs` · `query.rs`)뿐이다.
> 절 머리에 *"— **앞의 두 파일치**"* 라고 적어 보냈으므로 숨긴 것은 아니지만 **조건은
> 문면대로 못 지켰다.** 그래서 **인스턴스 2 를 전 diff 로 다시 띄웠다**(`129-judge-2.md`).
> **이 반환문은 안 버린다** ⟨`C3-a`·`C3-b`⟩.
>
> 판정자: `general-purpose` · sonnet · 도구 사용 **0 회**.

---

## 받은 것

**① 사전 등록 전문 (`plan/129-pre.md`)**
- 첫 줄: `# 사전 등록 — ㉢ `#129``
- 마지막 줄: `  미리 골라 뒀다.`

**② `pal touch` 산출 전문 (`touch/129.txt`)**
- 첫 줄: `# `pal touch` 산출 — ㉢ `#129``
- 마지막 줄: `  능력      ledger.snapshot · symbol.resolve · symbol.contains · symbol.callers · symbol.reaches · graph.dump · binding.status · narrative.unbound · binding.touch · plan.deviation · symbol.references · 미구축 F13 · F15`

**③ 실제 변경 diff (`git diff 22cf488..1abc0e9 -- crates/`)**
- 첫 줄: `diff --git a/crates/pal-cli/src/narrative.rs b/crates/pal-cli/src/narrative.rs`
- 마지막 줄: `        // ★ **수만 내면 「후보 있음 1,563」이 「승인 대기 1,563 건」으로 읽힌다.**`
  ⚠ **이 마지막 줄이 잘린 자리를 그대로 증언한다** — 전 diff 의 마지막 줄은 `}` 이다.

**④ §5.8 「메인이 이미 오염됐다」 표 전문 (`observations/red.md`)**
- 첫 줄: `### 5.8 ⚠ 메인이 이미 오염됐다 — 사전부검이 `pal touch` 를 먼저 돌렸다`
- 마지막 줄: `사전부검이 먼저 말했다.`

---

## 판정

### ⓐ — 유효

- 인용한 줄: `  fun        write                    crates/pal-cli/src/install/manifest.rs:373` 과
  `  fun        write                    crates/pal-store/src/projection.rs:251`
- 담은 사실: `write` 이름의 함수가 셋이고, 그중 둘의 구체적 좌표가 `install/manifest.rs:373` 과
  `pal-store/src/projection.rs:251` 이다.
- 사전 등록에 있나: 부분만 있다(인용) — §1 이 아는 것은 `write` 의 뿌리가
  `crates/pal-intent/src/store.rs:163-172` 라는 것뿐이고, §7 은 *"`write` 는 흔한 이름이라
  **후보 목록**이 나올 것이다(3 건 안팎)"* 라고 **근사로 예상**할 뿐, 나머지 두 심볼의
  **구체 파일·줄 좌표**는 어디에도 안 적는다.
- §5.8 표에 있나: 없다 — 표는 `nodes_of`·`identity_ceiling`·`check_ledger_pair` 세 심볼만
  다루고, 사전 등록 §2.3 이 직접 *"이 표에 `write` 는 없다"* 라고 못박는다.
- 판정 근거: 두 인용 줄이 담은 핵심 사실 — `install/manifest.rs:373` 과 `projection.rs:251` 이라는
  **구체 좌표** — 는 사전 등록의 근사 예측("3 건 안팎")이 대신하지 못하는 새 정보이고, §5.8 표는
  이 심볼을 아예 다루지 않는다. 그러므로 이 귀속은 touch 산출이 실제로 새로 준 사실을 근거로 든다.

## 합계

유효 1 · 무효 0

---

## 부르는 쪽이 적는 것

★ **이 판은 「사실이 새로운가」에 답했고, 「차이에 귀속되는가」에는 답하지 않았다.**
㉢ 의 `effect/129-delta.md` 는 차이 셋을 적고 **셋 다 원인이 도구(컴파일러·검사)** 라고
적는다. 그러므로 이 판의 「유효 1」은 **새 사실 1** 이지 **귀속된 차이 1** 이 아니다.
`C2` 는 *"그 **차이** 중 touch 산출에 귀속되는 것"* 을 묻는다 — ㉢ 에서 그 수는 **0** 이다.
**둘을 섞지 않는 것이 이 절의 전부다.**
