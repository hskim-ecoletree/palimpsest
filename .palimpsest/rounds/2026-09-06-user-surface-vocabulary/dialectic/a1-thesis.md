# 정(正) — `A1`·`A2`·`A3` 상태 어휘의 영어 원 표기와 병기 형식

회차 `2026-09-06-user-surface-vocabulary` · 이슈 [#108](https://github.com/hskim-ecoletree/palimpsest/issues/108)

## 판정해야 하는 것

| | |
|---|---|
| **A1** | `CodeFreshness` 넷의 영어 원 표기 확정 |
| **A2** | `Lineage::Current` 와의 낱말 충돌 반영 |
| **A3** | 사용자 언어 병기 형식 — 괄호 안 대소문자 · 좁은 자리 · 영어권 생략 |

## 소유자가 잠근 것

> 이 프로젝트에서 관리하는 상태 4개 중 stale(낡음), orphaned 을 제외하고는 직관적으로
> 와닿지 않음 특히 'live' 는 최신, 신선한, 유효한이라는 뉘앙스가 약하기도 하고
> 한국어권 사용자들에게는 on-air 같은 느낌으로 받아들여짐.

> 용어 중 상태나 코드성 데이터는 영어를 우선으로 하고, 사용자 표면에 나갈 때 사용자
> 언어와 같이 표기하는 형식을 사용할 것. 예시, 최신 상태 아님(Staled)

> 영미권-라틴어 계열은 같은 언어권으로 쳐서 별도 표기 없음

## 실측한 제약

| 좌표 | 무엇 |
|---|---|
| `crates/pal-core/src/binding.rs:565` | `Lineage` 가 `Current`·`Superseded` 를 쓴다 |
| `crates/pal-core/src/binding.rs:574` | `BindingStatus { code: CodeFreshness, lineage: Lineage }` — 두 축이 **한 구조체에 함께 산다** |
| `crates/pal-core/src/binding.rs:510` | `#[serde(rename_all = "snake_case", tag = "freshness")]` — 직렬화 표기는 소문자 |
| `xtask/src/main.rs:24-27` | 금지 어휘 16개. `live`·`current`·`fresh`·`valid`·`unchanged` 중 **금지된 것은 없다** |
| `xtask/src/main.rs:1935` | `check_no_regeneration` 이 `"CodeFreshness::Stale"` 문자열을 하한으로 쓴다 — `Stale` 을 유지하면 안 깨진다 |
| `crates/pal-core/src/touch.rs:212,235` | `CodeFreshness::Live` 로 정렬 우선순위와 낡음 판정을 한다 |

## 판정 초안

### A1 — `Live → Fresh`. 나머지 셋은 유지한다.

| 지금 | 새 표기 | 왜 |
|---|---|---|
| `Live` | **`Fresh`** | 아래 |
| `Stale` | `Stale` | 소유자가 문제 없다고 했다 |
| `Orphaned` | `Orphaned` | 같음 |
| `Undeterminable` | `Undeterminable` | 소유자가 언급하지 않았고, 뜻과 표기가 어긋나지 않는다 |

**근거 넷:**

1. **`Fresh` ↔ `Stale` 은 영어에서 굳은 반의어 쌍이다.** 같은 축의 두 값이 자연스러운
   대립을 이루면 사용자가 축을 한 번에 읽는다. `Live` 는 `Stale` 의 반의어가 아니다 —
   그것이 소유자가 걸린 지점이다.
2. **`Current` 는 못 쓴다.** `Lineage::Current` 와 충돌하고, 둘은 `BindingStatus` 라는
   **한 구조체에 함께 실린다**(`binding.rs:574`). `BindingStatus { code: Current,
   lineage: Current }` 는 읽는 사람이 두 축을 못 가른다. 축이 둘인 것이 이 타입의
   설계 근거인데(`binding.rs:567`: *"코드 신선도는 계속 계산된다 — 그것이 축이 둘인
   이유이고, 한 열거에 넣으면 그 계산이 사라진다"*) 같은 낱말을 쓰면 그 근거가 화면에서
   지워진다.
3. **`Valid` 는 이 제품의 원칙과 부딪힌다.** [00-goals.md §3.2](../../../docs/plan/00-goals.md)
   가 *"`clean`을 출력하지 않는다"* 를 못 박았다. `Valid` 는 「유효하다」는 단언인데,
   이 도구가 실제로 아는 것은 **「감시 집합의 요약이 그대로다」** 뿐이다. 결정이
   여전히 옳은지는 모른다. `Valid` 로 쓰면 모르는 것을 안다고 하는 것이 된다 — §3.1 위반.
4. **`Unchanged` 는 정확하지만 대칭이 아니다.** 뜻은 맞다(감시 집합이 안 변했다).
   그러나 `Unchanged` ↔ `Stale` 은 대립 쌍으로 안 읽히고, 한국어 병기가
   「안 변함(unchanged)」이 되어 **무엇이 안 변했는지**가 사용자에게 안 통한다.
   `CodeFreshness` 라는 축 이름과도 어근이 어긋난다.

⚠ **`Freshness` 의 값이 `Fresh` 인 것은 동어반복이 아니다.** 축 이름이 재는 성질을
가리키고 값이 그 성질의 한쪽 끝을 가리키는 정상 형태다 — `Freshness ∈ {Fresh, Stale}`.
비교: `Lineage ∈ {Current, Superseded}` 는 축 이름과 값의 어근이 다르고, 그래서
`Current` 가 어느 축의 값인지 화면에서 안 보인다. 그 비대칭이 제약 2 를 낳았다.

### A2 — 충돌은 회피로 처리한다. `Lineage` 는 안 건드린다.

`Lineage::Current` 를 바꾸는 것도 길이지만 **범위를 넘는다.** 소유자가 지목한 것은
`live` 하나이고, `Lineage` 는 사용자 화면에 상태값으로 나가지 않는다. `A1` 이 `Current`
를 피하는 것으로 충돌이 해소되므로 `Lineage` 개명은 이 회차의 일이 아니다.

### A3 — 병기 형식

| 물음 | 판정 | 왜 |
|---|---|---|
| 괄호 안 대소문자 | **소문자** — `최신(fresh)` | `serde(rename_all = "snake_case")` 가 내는 직렬화 문자열과 **같은 문자열**이어야 사용자가 화면과 `--json` 출력을 눈으로 대조한다. `Fresh` 로 쓰면 둘이 다른 문자열이 되고, 그것을 grep 하는 사용자가 못 찾는다. ⚠ 소유자 예시는 `Staled` 였는데 그 형태는 영어에 없다 — 원 표기를 정본으로 삼으면 이 문제가 원리상 안 생긴다 |
| 좁은 자리 | **병기한다** | 범례를 한 번만 두면 사용자가 화면을 되돌아가야 한다. `pal touch` 의 출력은 한 번 읽고 버리는 화면이라 되돌아갈 자리가 없다. 대신 표의 **열 너비를 병기 후 길이로 잡는다** |
| 영어권 생략 | **이 회차에서는 한국어 고정** | 소유자 의도는 명확하지만 **로케일 판별 장치가 저장소에 없다.** `intent.md` 의 `## 범위 밖` 이 i18n 프레임워크를 제외했다. 생략 규칙을 지금 문서에만 적으면 그것이 「측정이 죽은 가지」다 — 아무것도 안 재는 선언이 된다. **로케일 판별은 별개 의도로 이슈를 세운다** |

### 한국어 대응

| 원 표기 | 병기 | 왜 |
|---|---|---|
| `fresh` | **최신(fresh)** | 소유자가 *"최신, 신선한, 유효한"* 중 「최신」을 먼저 들었다 |
| `stale` | **최신 아님(stale)** | 소유자 예시 *"최신 상태 아님(Staled)"* 의 뜻을 그대로 살리되 원 표기를 고쳤다 |
| `orphaned` | **좌표 없음(orphaned)** | 「고아」는 한국어에서 사람에게만 쓴다. 실제 뜻은 *"좌표가 사라졌다"*(`binding.rs:523`) |
| `undeterminable` | **판정 불가(undeterminable)** | doc 주석이 *"판정할 수 없다"*(`binding.rs:524`) 로 이미 적었다 |
