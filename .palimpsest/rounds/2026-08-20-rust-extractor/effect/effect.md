# 효과 — 테스트도 CI 도 아닌 것이 돌린 출력

> `pal` 을 직접 부른 산출이다. **틀린 답이어도 붙인다** — 틀렸다는 것이
> 그 회차가 얻은 가장 값진 것이다(규약 §8).

## ① `pal ledger` — 착수 시점과 지금

### 착수 (`56926aa`)
```
파일 458 · parsed 1 · partial 0 · unsupported 413 · unrecognized 43 · binary 1
Markdown    L0 결박 불가  219 파일
Rust        L0 결박 불가  117 파일   ← 표적
TypeScript  L2 identity: exact  1 파일   ← 지금 서 있는 전부
          ← 결박 불가 언어 7개 · 413 파일. 이 파일들에는 좌표가 없습니다
```

### 지금
```
파일      469
  parsed             120
  partial              0
  unsupported        303  추출기 없음(로드맵)
  unrecognized        45  언어 미인식
  excluded             0
  binary               1
  generated            0

언어      Markdown          L0  결박 불가            225 파일
          Rust              L1  identity: ordinal   119 파일
          Python            L0  결박 불가             41 파일
          TOML              L0  결박 불가             28 파일
          Shell             L0  결박 불가              6 파일
          YAML              L0  결박 불가              2 파일
          JSON              L0  결박 불가              1 파일
          TypeScript        L2  identity: exact     1 파일

          ← 결박 불가 언어 6개 · 303 파일. 이 파일들에는 좌표가 없습니다
```

## ② `pal narrative` — 결박이 실제로 선다
```

■ 서술물 인입
  문서 226 · 조각 2324 · 새 개체 0
  결박됨 43 · 후보 있음 615 · 미결박 1666

■ 무엇이 걸었나
  attached               43
  fenced-path            4
  span                   611
```

## ③ 실제 질의 — 이 회차가 만든 코드에 묻는다

**물음: `extract_detailed` 를 고치면 무엇이 걸리나.**
```
$ pal query symbol.resolve extract_detailed

■ symbol.resolve  extract_detailed

  fun        extract_detailed         crates/pal-extract/src/kotlin.rs:99
  fun        extract_detailed         crates/pal-extract/src/rust.rs:209
  fun        extract_detailed         crates/pal-extract/src/typescript.rs:84

■ 이 답의 근거
  Snapshot  palimpsest@bdd3934+worktree
  2층       심볼 2609 색인됨
  범위      미해소 0 · 범위 밖 349 파일 · 최저 등급 L1 · identity ordinal
  절단      없음 (명시)
  접힘      469건이 다른 질의로 옮겨졌습니다 — **잘린 것이 아닙니다**
            ledger 469건 → `ledger.snapshot` 가 폅니다
  질의 로그  남았습니다
  크기      약 549 토큰 **이상** (잰 것: 2197 바이트 · 가정: 4 바이트/토큰)
  능력      ledger.snapshot · symbol.resolve · symbol.contains · symbol.callers · symbol.reaches · graph.dump · binding.status · narrative.unbound · binding.touch · plan.deviation · 미구축 F07 · F08 · F13 · F15

```

**물음: `kind_of` 는 무엇에 담기나.**
```
$ pal query symbol.contains RustExtractor

■ symbol.contains  RustExtractor

  fun        language                 crates/pal-extract/src/rust.rs:47
  fun        grade                    crates/pal-extract/src/rust.rs:51
  fun        extract                  crates/pal-extract/src/rust.rs:55
  fun        marked_comments          crates/pal-extract/src/rust.rs:59

■ 이 답의 근거
  Snapshot  palimpsest@bdd3934+worktree
  2층       심볼 2609 색인됨
  범위      미해소 0 · 범위 밖 349 파일 · 최저 등급 L1 · identity ordinal
  절단      없음 (명시)
  접힘      469건이 다른 질의로 옮겨졌습니다 — **잘린 것이 아닙니다**
            ledger 469건 → `ledger.snapshot` 가 폅니다
  질의 로그  남았습니다
  크기      약 639 토큰 **이상** (잰 것: 2556 바이트 · 가정: 4 바이트/토큰)
  능력      ledger.snapshot · symbol.resolve · symbol.contains · symbol.callers · symbol.reaches · graph.dump · binding.status · narrative.unbound · binding.touch · plan.deviation · 미구축 F07 · F08 · F13 · F15

```

## ④ ★ 이 회차의 ADR 을 실제로 결박한다 — §11 조건 4

**앞 회차 셋은 이 자리를 전부 「능력 부재」로 끝냈다.** 이 회차는 그것을 시도한다.

### `pal narrative` 재인입 — ADR-0027 이 조각으로 서는가
```

■ 서술물 인입
  문서 226 · 조각 2324 · 새 개체 0
  결박됨 43 · 후보 있음 615 · 미결박 1666

■ 무엇이 걸었나
  attached               43
  fenced-path            4
  span                   611
```

### ★ 결박이 섰다 — 43 → 46

ADR 을 발행하는 것만으로는 **안 걸린다.** 조각 17 개가 생겼지만 결박은 43 그대로였다.
**코드가 그 ADR 을 인용해야 좌표가 생긴다** — 그것이 결박의 뜻이다.

`rust.rs`·`language.rs`·`symbol.rs`·`classify.rs`·`parse.rs` 다섯 자리에
`ADR-0027` 인용을 달자 **46** 이 됐다.

```
문서 228 · 조각 2346 · 새 개체 8
결박됨 46 · 후보 있음 622 · 미결박 1678
attached 46 · fenced-path 6 · span 616
```

**§11 조건 4 가 이 저장소에서 처음으로 섰다.** 앞 회차 셋은 이 자리를 전부
「능력 부재」로 끝냈고 그 수가 회차마다 커졌다 — 회차가 만든 산출이 전부
미결박이었기 때문이다.

### ⚠ 틀린 답도 붙인다 — `binding.status` 는 「결박이 아직 없습니다」를 낸다

같은 순간에 이것도 사실이다:

```
$ pal query binding.status

  결박이 아직 없습니다.
```

**두 수가 안 싸운다 — 다른 것을 센다.**

| | 무엇 | 지금 |
|---|---|--:|
| `pal narrative` 의 「결박됨」 | **제안된**(`inferred`) 결박 | 46 |
| `pal query binding.status` | **승인된**(`asserted`) 결박 | 0 |

`narrative` 자신이 그 자리를 적는다 — *"**아무것도 승인하지 않았습니다.**
`inferred` 는 사람의 승인으로만 `asserted` 가 됩니다"*.

★ **그래서 이 회차는 승인하지 않는다.** 그것은 설계된 경계이지 잔여가 아니다 —
「이 회차에서 할 수 있었는가」의 답이 *"할 수 있지만 그것은 사람의 자리다"* 이다.
소유자가 `pal narrative approve <개체>` 로 승인하면 `binding.status` 가 답하기
시작한다.

⚠ **그러므로 「§11 조건 4 가 섰다」는 절반의 문장이다.** 정확히는
**「결박 제안이 코드 좌표에 실제로 걸렸다」**이고, 앞 회차들이 못 한 것이 바로
그 절반이었다 — 그때는 **걸 좌표 자체가 없었다.**
