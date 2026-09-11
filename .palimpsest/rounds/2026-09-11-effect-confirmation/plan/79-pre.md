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
