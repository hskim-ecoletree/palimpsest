# R2 측정 ⑴ — `exact` 축에서 두 반경의 **판정**이 갈리나

**판정하지 않는다. 수와 산출만 적는다.**

## 0. 모집단과 실행 조건

| | |
|---|---|
| 코퍼스 | `ditto` (`exact` 축) · `~/dev/projects/ditto` · 핀 `aded7ce7f88f` |
| `pal` | `target/release/pal 0.0.0+e05ca36a811e` (= HEAD `e05ca36`) |
| 절차 | `scripts/f09-verify.py` 의 `실_이력()` — `심볼_고르기`(:265) · `결박_걸기`(:230) |
| 창 | `HISTORY_WINDOW=120`, `--no-merges` → **훑은 커밋 119** |
| 시작(결박 시점) | `d6ee4ad6` (창의 가장 오래된 커밋) |
| 결박 하한 | `MIN_BINDINGS=20` — 두 반경 모두 **20/20 걸림** (후보 40 중) |
| 표본 상한 | `SAMPLES_PER_CORPUS=10` |
| 사본 | `git clone --local --no-checkout` → ditto 워킹트리의 오염은 안 들어간다 |
| 반경 | `symbol` · `callers` — **같은 표본·같은 시작 커밋·같은 20 결박** |

⚠ **코퍼스 클론 루트 주의는 이 스크립트에 안 걸린다.** `f09-verify.py --help` 는 코퍼스 인자를
**하나도 노출하지 않고**, 경로가 소스에 박혀 있다(`:44 DITTO`, `:46 PORTAL`). 둘 다 실재한다.
(`~/dev/projects/boxwood` 를 인자로 받는 것은 `s0-corpus.sh` 다.)

---

## 1. ★ **#58 의 옛 실패 형태는 글자까지 그대로 재현된다** — 등록된 실행체의 산출

`실_이력()` 을 `symbol`·`callers` 로 한 번씩 부른 원문:

```text
ok  ⑤ ditto(exact)/symbol  표본 — 커밋 119개 훑음 · 표본 10 · stale 0 · orphaned 10 (상한 10%)
ok  ⑤ ditto(exact)/callers 표본 — 커밋 119개 훑음 · 표본 10 · stale 0 · orphaned 10 (상한 30%)
```

**요약줄만이 아니다 — 표본 10 줄이 결박 id·구간·갈래·경로·커밋 제목까지 전부 같다:**

```text
3b1e8563..6f298c8e  3656b7e8  orphaned  tests/core/charter-region.test.ts
3b1e8563..6f298c8e  70566199  orphaned  tests/cli/push-gate-hook-provenance.test.ts
3b1e8563..6f298c8e  75b07766  orphaned  tests/cli/prism-cli.test.ts
3b1e8563..6f298c8e  83acbda6  orphaned  tests/core/autopilot-approval-artifact.test.ts
3b1e8563..6f298c8e  c05bda51  orphaned  tests/acg/fitness-injected-provider.test.ts
3b1e8563..6f298c8e  c5060762  orphaned  tests/scripts/check-npx-distribution.test.ts
3b1e8563..6f298c8e  d4aeffd7  orphaned  tests/core/land-commit.test.ts
3b1e8563..6f298c8e  d7935ead  orphaned  tests/core/provisioner.test.ts
3b1e8563..6f298c8e  dc57271e  orphaned  tests/cli/autopilot-loop-cli.test.ts
3b1e8563..6f298c8e  dc978e1d  orphaned  tests/integration/coverage-depth-dial.test.ts
```

두 반경 모두 `chore(rebuild): 옛 src 계층 테스트 전부 제거 — tests/` 커밋 하나에서 열이 전부 켜졌고
`triggered_by` 는 둘 다 비었다(`—`). **`diff` 가 0 바이트다.**
→ `#58` 이 적은 *"두 반경의 표본이 글자까지 같았다 — 표본 10 · stale 0 · orphaned 10"* 은 **오늘도 참이다.**

---

## 2. ★ 그러나 **판정은 갈린다** — 표본 상한 밖에서

같은 절차를 독립 재현하되 **상한을 안 걸고 20 결박 × 119 커밋 전부**의 갈래를 기록했다.

### 2.1 감시 집합 (정·반·합이 세운 자리 — 확인)

| | `symbol` | `callers` |
|---|---:|---:|
| 결박 | 20 | 20 |
| **감시 원소 합계** | **20** | **27** (+7) |
| 감시 원소가 늘어난 결박 | — | **6** (`59632e02` 1→2 · `7965dd2b` 1→2 · `95ed86da` 1→**3** · `a88539ce` 1→2 · `ced475c1` 1→2 · `fb57cbbb` 1→2) |
| 결박 직후(`d6ee4ad6`) 분포 | `fresh 20` | `fresh 20` |

### 2.2 ★ 핀(`aded7ce7f88f`)에서의 분포 — **여기서 갈린다**

| 갈래 | `symbol` | `callers` |
|---|---:|---:|
| `orphaned` | 13 | 13 |
| `stale` | **1** | **1** |
| `fresh` | **6** | **5** |
| `undeterminable` | **0** | **1** |
| 합 | 20 | 20 |

**갈린 결박은 1/20 이다:**

```text
a88539ce   src/core/handoff-store.ts   watch: symbol 1 · callers 2
   symbol  → fresh
   callers → undeterminable  (reason = watch_member_gone)
   처음 갈라진 커밋: a0db9e1a  (훑은 119 커밋 중 36 번째)
```

나머지 19 결박은 **핀에서 갈래가 같다.**

### 2.3 `stale` / `orphaned` 축만 보면 **안 갈린다**

| | `symbol` | `callers` |
|---|---:|---:|
| 창을 지나며 **처음 켜진**(`stale`∪`orphaned`) 결박 | **14** | **14** |
| 그 결박의 **id 집합** | 같다 | 같다 |
| 그 결박의 **구간(커밋)** | 같다 | 같다 |
| `stale` 로 켜진 것 | `ced475c1` (1건) | `ced475c1` (1건) |

→ **`callers` 가 더한 감시 원소 7 개는 이 119 커밋에서 `stale` 을 *하나도* 더 켜지 않았다.**
`#58` 의 *"`callers` 가 더한 감시 원소들이 그 커밋들에서 안 변했다"* 는 **정정이 필요하다** —
안 변한 것이 아니라 **사라졌고**(`watch_member_gone`), 그 결과가 `stale` 이 아니라 `undeterminable` 이다.

### 2.4 `undeterminable` 사유 — **`callers` 만이 산출한다**

| 반경 | `watch_member_gone` 관측 횟수(결박×커밋) | 다른 사유 |
|---|---:|---|
| `symbol` | **0** | 없음 |
| `callers` | **84** | 없음 |

(등록된 실행체를 `ditto`+`symbol`/`callers` 둘만 돌린 이 회차에서 `관측된_사유` 는 `[]` 였다 —
`실_이력()` 이 사유를 모으는 루프가 `표본 10` 이 차면 `break` 로 빠져나가기 때문이다. §3 참조.)

---

## 3. 갈린 것 — **두 방법이 다른 값을 낸 자리**

| | 방법 A (등록된 `실_이력()`) | 방법 B (같은 절차 · 상한 없음 · 전 분포) |
|---|---|---|
| 두 반경이 갈리나 | **안 갈린다 (글자까지 같다)** | **갈린다 (1/20)** |
| 관측된 `undeterminable` 사유 | `[]` (0개) | `watch_member_gone` (`callers` 84회) |

**갈린 까닭이 코드에 있다 — 두 자리다:**

1. **`f09-verify.py:715`** — `if f in ("stale", "orphaned") and …`.
   표본기는 **`undeterminable` 을 표본에 안 담는다.** 이 코퍼스에서 두 반경이 갈리는 **유일한 축**이
   `undeterminable` 이므로, **표본기가 구조적으로 그 차이를 못 본다.**
2. **`f09-verify.py:725`** — `if len(표본) >= SAMPLES_PER_CORPUS: break`.
   표본이 10 에서 차서 **119 커밋 중 앞 구간 하나**에서 멈춘다. 갈라지는 커밋 `a0db9e1a` 는
   그 `break` **뒤**에 있다.

⚠ **상한 10 을 올려도 안 갈린다.** 두 반경의 `전이` 집합은 **14 건 전부**가 같으므로
`SAMPLES_PER_CORPUS` 를 14 로 올려도 표본은 여전히 글자까지 같다.
**막는 것은 상한이 아니라 ①의 갈래 필터다.** (`#63` ⓕ-1 이 지목한 것은 상한 쪽이다.)

---

## 4. 못 잰 것

- **거짓 양성률 자체는 안 쟀다.** `[f09.pass]` 가 *"판정은 사람(에이전트)이 하고 근거를 결박마다
  한 줄로 남긴다"* 로 정의했고, 이 작업의 지시는 *"판정하지 마라"* 다. 표본과 갈래만 산출했다.
- **`callers` 의 고유 위험(*"호출자가 변해서 `stale` 이 켜진다"*)은 이번에도 안 재어졌다** —
  이 코퍼스·이 창에서 그 사건이 **0 건**이다(§2.3). 대신 **호출자가 *사라져서* `undeterminable` 이
  켜지는 사건이 1 건** 산출됐다(§2.2). **둘은 다른 사건이다.**
- **`portal(ordinal)` 축은 안 돌렸다.** 지시가 `exact` 축이다.

## 5. 원본 산출물

- `scratchpad/m1_registered.out` — 방법 A 원문(등록된 `실_이력()` 두 번)
- `m1_full.jsonl.txt` — 방법 B 전 궤적(119 커밋 × 20 결박 × 2 반경)
- `scratchpad/m1_diff.out` — 결박별 핀 갈래 대조표
