# ADR-0021 — **한 모집단이 담을 수 없는 상태는 그 열거에 안 들어간다**

**상태**: 채택 (2026-08-16) · [F12](../gates/F12.md) 종료 시점에 발행 ·
근거 [F12 게이트](../gates/F12.md) §6 · `corpus/criteria.toml` `[f12].pending_ruling`

## 맥락

[F09 §2] 가 `CodeFreshness` 를 **여섯**으로 적었고 F09 는 그중 **넷**을 세웠다.
`Pending` 을 안 세운 근거를 `[f09].freshness_boundary` ⓐ 가 이렇게 적었다:

> `Pending` 은 *"좌표가 아직 없다"* 이다. 그런데 `Binding` 은 `target: SymbolId` 를
> **요구**하고, 그것이 [F03 §3.3] 이 **타입으로** 세운 것이다. …
> `Pending` 이 태어나려면 `subject` 만 있고 `target` 이 없는 결박이 있어야 하고,
> 그것은 **F10(문서 인입)·F12** 의 것이다.

**F12 가 그 만기다.** [F12 §3.1] 이 *"**`pending` 이 이 기능의 핵심 상태다**"* 라고
적었으므로, 이 회차는 그 상태를 세워야 했다.

**그런데 넣을 자리가 없다.** `CodeFreshness` 를 만드는 자리는
`BindingStatus::evaluate(&Binding, …)` **하나**이고, 그 함수의 입력인 `Binding` 은
[F03 §3.3] 의 강제 때문에 **정의상 `pending` 일 수 없다.** 그러므로 `Pending` 을
더하면 둘 중 하나가 된다:

  · **아무도 못 만드는 변형** — [ADR-0012] 가 금한 짝 없는 이름
  · **`evaluate` 밖에 두 번째 생산자를 두는 것** — 그러면 `CodeFreshness` 가
    **결박**과 **계획 항목** 두 모집단 위에 서게 된다

## 결정

**한 열거가 지는 모집단이 그 상태를 담을 수 없으면, 그 상태는 그 열거에 안 들어간다.
자기 열거를 세운다.**

F12 는 [`PlanBindingState`] 를 새로 세웠고 `Binding`·`CodeFreshness`·`Lineage` 와
`schema/graph.toml` 의 `Binding` 절을 **한 글자도 안 건드렸다.**

```text
CodeFreshness      결박      위에 선다   Live · Stale · Orphaned · Undeterminable
PlanBindingState   계획 항목  위에 선다   Bound · Pending · Unresolved
```

**[F12 §3.1] 의 *"좌표가 생기면 자동으로 `live`"* 는 `Pending → Bound` 다** —
상태 이름의 변경이 아니라 **「상태를 계산할 자격의 획득」**이고, 그 계산은 여전히
`BindingStatus` 의 것이다.

## 왜 이것이 새 규칙이 아닌가 — **같은 절이 이미 형태를 세웠다**

`[f09].freshness_boundary` ⓑ 가 `NodeFreshness` 와 `CodeFreshness` 를 **안 합친**
근거를 이렇게 적었다:

> **모집단이 다르다.** `NodeFreshness`(F22-4)는 **그래프 노드** 위에 서고
> `CodeFreshness` 는 **결박** 위에 선다. 합치면 … **`[f22.4]` 의 불변식 8 의 모집단이
> 바뀐다.** 남의 게이트의 판정을 움직이는 것이고, **한 줄이 싸다는 이유로 해서는 안 된다.**

★ **이 결정은 그 근거를 「합치기」에서 「더하기」로 옮긴 것이다.** 열거에 변형을
더하는 것도 그 열거가 지는 모집단을 넓히는 일이고, 넓히는 순간 그 열거를 읽는
**모든 소비자**가 새 모집단을 함께 읽게 된다 — `binding.status` 의 답, `doctor` 의
불변식 8, 그리고 그것들 위에 선 게이트들이다.

## 가르는 문장

> **그 열거를 만드는 함수의 입력이 이 상태를 가질 수 있는가.**

  · 가질 수 있으면 — 변형을 더한다. `Undeterminable` 이 그랬다(F09)
  · 가질 수 없으면 — **자기 열거를 세운다.** `PlanBindingState` 가 그것이다
  · 가질 수 없는데 더하면 — 아무도 못 만드는 이름([ADR-0012])이거나
    **한 열거가 두 모집단 위에 서는 것**이고, 둘 다 이 저장소가 금한 형태다

## 결과

  · **`[f22.4]` 의 불변식 8 의 모집단이 안 움직였다** — F12 가 남의 게이트의 판정을
    안 건드렸다. 골든 넷도 안 움직였다(게이트 §9)
  · **`[f09].freshness_boundary` ⓐ 의 만기가 「Pending 을 세운다」가 아니라
    「Pending 을 다른 곳에 세운다」로 닫혔다** — 그 절이 넘긴 문장(*"F10·F12 의 것"*)은
    **어느 열거에 넣으라는 뜻이 아니었다**
  · `PlanBindingState` 셋이 **전부 실 코퍼스에서 값을 냈다** — `Pending` 148 ·
    전이 80 · `Unresolved` 84(게이트 §6). **빈 변형이 없다**
  · ⚠ **`CodeFreshness` 의 남은 하나(`StaleDerived`)는 여전히 안 섰다.**
    `[f09].freshness_boundary` ⓑ 가 미룬 그대로다 — 결박의 파생 입력을 주는 것이
    이 빌드에 없다

## 안 고른 것

| 대안 | 왜 안 골랐나 |
|---|---|
| `CodeFreshness::Pending` 을 더한다 | 위 — 아무도 못 만드는 이름이거나 두 모집단 |
| `Binding.target` 을 `Option<SymbolId>` 로 | 「선택 필드 금지」가 잡는다. 그리고 규칙 이전에 **설계**다 — `None` 이 *"아직 없다"* 인지 *"모른다"* 인지 구별되지 않는다([ADR-0005]). [F03 §3.3] 이 타입으로 세운 강제가 통째로 무너진다 |
| `PlanBindingState` 를 `CodeFreshness` 의 별칭으로 | 이름만 갈리고 모집단은 하나다. 소비자가 같은 열거를 읽으므로 아무것도 안 막는다 |
| `pending` 을 `Undeterminable{reason}` 의 사유로 | *"판정할 수 없다"* 와 *"아직 안 만들었다"* 는 **사람이 다르게 처리한다.** 뭉개면 F09 가 `Orphaned` 를 `Stale` 에서 가른 이유가 사라진다 |

[F03 §3.3]: ../plan/features/F03-normalize.md
[F09 §2]: ../plan/features/F09-freshness.md
[F12 §3.1]: ../plan/features/F12-plan-binding.md
[ADR-0005]: 0005-absence-carries-its-kind.md
[ADR-0012]: 0012-a-single-truth-file-declares-only-what-has-a-counterpart-in-code.md
[`PlanBindingState`]: ../../crates/pal-core/src/plan.rs
