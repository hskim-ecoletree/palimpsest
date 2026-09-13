# 음성 대조 — `B5` 의 예산 호출이 사라지면 빨개진다

> 회차 `2026-09-12-binding-radius-in-use` · 2026-09-13
> ⟨조건 `B5` · 독립 리뷰 R2 발견 5 의 처분⟩

## 왜 시험이 아니라 검사인가 — **RED 가 원리상 도달 불가다**

조건 `B5` 는 *"보존 경로가 `check_budget`(`crates/pal-core/src/radius.rs:175`)을 부르는지를
**시험으로** 잰다"* 를 요구했다. 독립 리뷰 R2 가 **그 시험이 저장소에 0 건**임을 잡았다 —
`pal radius` 는 `crates/pal-cli/src/radius.rs:221` 에서 실제로 부르지만 그 호출을 붙드는
것이 없어 **지워도 아무것도 빨개지지 않았다.**

시험으로는 못 잡는다. 거부선이 `PROVISIONAL_WATCH_PRODUCT_MAX` = **1_000_000**
(`crates/pal-core/src/budget.rs:330`)이고 인자가 `(결박 수, 한 결박의 감시 크기)` 이므로
픽스처가 RED 에 닿으려면 **결박 1,001 건 × 감시 1,000** 같은 규모가 필요하다 — `WatchEntry`
백만 행이고 통합 시험의 규모가 아니다. 그것이 `CA1-03` 이 이름 붙인 *"GREEN 이 도달 불가"* 의
**거울상**이다.

**그래서 재는 자를 바꿨다 — 「호출이 사라지면 빨개지는 자」다.**
`cargo xtask check` 의 검사 **「결박을 저장하는 경로가 예산을 지난다」**:

- **모집단** — **여섯 디렉터리**(`pal-core` · `pal-cli` · `pal-query` · `pal-store` ·
  `pal-intent` · `pal-extract`)에서 **`.record(&`** 를 부르는 파일. `grep` 으로 산출한다 —
  **손으로 베끼지 않는다.**
- **술어** — 같은 파일이 `check_budget(` 을 부른다. 안 부르는 까닭은 **이름 붙여 면제**한다
  (`예산을_안_지나도_되는_쓰기` — 지금 한 건).
- **하한** — 모집단이 0 이면 실패한다(저장 API 의 이름이 바뀐 것이다).

⚠⚠ **첫 판의 모집단 선언이 거짓이었다** ⟨정정 2026-09-13 · 독립 리뷰 R3 발견 12 ·
**금지역 `사실이_아닌_것을_사실로`**⟩. 앞 판은 훑는 디렉터리가 **셋**이고 리터럴이
**`intent.record(`** 였다. 그러면 ⑴ `crates/pal-intent/src` 를 아예 안 보고 ⑵ 수신자 이름이
`intent` 가 아닌 호출(`self.record(&b)` · `store.record(&b)`)을 **원리상 못 본다** — 그런데
*"결박을 정본에 쓰는 경로 **전부**"* 라 적고 있었다. R3 이 격리 사본에 탐침 둘을 심어
실측했고 그 검사는 **초록 그대로**였다. ⚠ 바로 위 검사 12(앵커)는 같은 파일에서 여섯
디렉터리를 훑는다 — **비대칭이 같은 함수 옆에 있었다.**

★ **넓히자 진짜 구멍이 드러났다** — `crates/pal-intent/src/store.rs:714` 의 `self.record(&b)`
(= `import_jsonl` 의 되짚기)가 **예산을 안 지난다.** 그것은 이 회차의 축(반경)이 아니라 결박
**문면·인입**의 축이므로 **이름 붙여 면제하고 이슈로 세웠다 — [#153]**. 면제가 없으면
넓히는 순간 빨개지고, 빨개지는 것을 피해 모집단을 좁히면 **선언이 다시 거짓**이 된다.

[#153]: https://github.com/hskim-ecoletree/palimpsest/issues/153

## 기준선 — 사본에서 초록

```
$ git clone -q . <사본>            # 아직 안 커밋된 검사와 radius.rs 를 사본에 복사
$ cargo xtask check --root <사본>
  ok    결박을 저장하는 경로가 예산을 지난다  — 결박을 쓰는 경로 3개 · 예산을 안 지나는 것 0개
        (crates/pal-cli/src/bind.rs · crates/pal-cli/src/narrative.rs · crates/pal-cli/src/radius.rs)
검사 29/29 통과
```

## 심은 뒤 — **세 방향으로 빨개진다**

**셋 다 종료값을 파이프 없이 받았다** — 규약 §7 이 금한 `| tail` 로 받지 않았다.

### ① 보존 경로의 호출을 지운다 — `rc=1`

`radius.rs:221` 의 `check_budget(건수, 새.watch.len())…?` 한 줄을 `let _ = 건수;` 로 바꿨다.

```
  FAIL  결박을 저장하는 경로가 예산을 지난다
    crates/pal-cli/src/radius.rs
Error: 1개 검사가 실패했다
```

### ② 모집단 밖에 쓰기 경로를 심는다 — `rc=1`

독립 리뷰 R3 의 탐침 둘을 사본에 넣었다 — `crates/pal-intent/src/probe_a.rs`
(`store.record(&…)` · 예산 없음)와 `crates/pal-cli/src/probe_b.rs`(수신자 이름이 `intent` 가
아니다). **첫 판의 검사는 이 둘에 초록이었다.** 넓힌 뒤:

```
  FAIL  결박을 저장하는 경로가 예산을 지난다
    crates/pal-cli/src/probe_b.rs
    crates/pal-intent/src/probe_a.rs
Error: 1개 검사가 실패했다
```

### ③ 등록된 면제를 뗀다 — `rc=1`

`예산을_안_지나도_되는_쓰기` 에서 `store.rs` 행을 지웠다. **면제는 상수라 컴파일에 박히므로
이 축만은 작업 트리에서 재고 바로 되돌렸다**(`cargo xtask check --root <사본>` 은 사본의
*파일*을 읽지만 상수는 이쪽 바이너리의 것이다 — 그 사실이 ③ 을 사본에서 못 재게 한다):

```
  FAIL  결박을 저장하는 경로가 예산을 지난다
    crates/pal-intent/src/store.rs
Error: 1개 검사가 실패했다
```

**셋이 서로 다른 것을 잰다** — ① 은 호출의 부재, ② 는 모집단의 넓이, ③ 은 면제가 실제로
무언가를 덮고 있다는 사실이다. ③ 이 초록이면 그 면제는 장식이다.

## 사본을 어떻게 만들었나

`git clone` 이다. **디렉터리 복제가 아니다** — `repo_id` 가 디렉터리 이름이라
(`crates/pal-cli/src/ledger.rs:581`) 복제하면 **어긋난 사유로** 빨개진다(`CA1-05`).
그리고 작업 트리는 한 글자도 안 고쳤다 — 사본만 고쳤고 끝나고 지웠다.
