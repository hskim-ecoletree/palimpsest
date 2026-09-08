# 상태 — Rust 스코프와 참조 엣지

> 회차 `2026-09-07-rust-scope-references` · 착수 커밋 `a774053` · 이슈 [#130]
> 잠긴 의도는 [`intent.md`](intent.md) 가 진다. **이 파일은 요약이지 정본이 아니다.**

## 지금 단계

**검증.** 계획 아홉을 다 만졌고 게이트([`docs/gates/rust-scope-references.md`](../../../docs/gates/rust-scope-references.md))가 섰다.
아래 「계획」 표는 **이미 지나온 것**이니 다시 밟지 마라.

지금 남은 것은 넷이다 — ① 정반합 판 2 의 라운드 2(종료 판단이 금지역 하나를 근거로 열었다) ·
② 독립 리뷰 R3(상한 3 중 둘을 썼다) · ③ 종료 보고 `report.md`(지금 유일한 빨강) ·
④ `git push` 와 CI 초록 확인(`C5`).

착수 전 단계는 끝났다 — 소유자가 2026-09-08 에 승인했고(*"다음 세션에서 구현 착수할 거니까"*),
사전부검 2 라운드와 완수 조건 설계 평가 2 라운드를 상한까지 다 썼으며 소유자 승격 넷이
답을 받았다. 조건이 열여덟 → 서른하나 → **쉰둘**이 됐다.

⚠ **다음 컨텍스트가 먼저 읽을 것** — [`intent.md`](intent.md) 전문과 이 파일의
「실패한 접근」. 직전 산출물이 아니라 **잠긴 의도가 다음 걸음의 입력이다.**

## 계획 — 무엇을 어느 순서로 만지나

⚠ **앞 판의 계획 1 은 「새 파일」이었고 소유자가 「공용화」로 뒤집었다.**

| # | 무엇 | 어디 |
|---|---|---|
| 1 | `scopes.rs` 의 **뼈대를 언어 중립으로 뺀다** — 2 패스(선언·참조) · `scope_at` 을 `Node::id` 로 잡기 · `hoist_home` · `symbol_at` 을 시작 바이트로 잇기. TypeScript 가 실물에서 사서 넣은 방어 넷이 그 뼈대에 남는다 | `crates/pal-extract/src/scopes.rs` |
| 2 | **노드 종류 표만 언어별로 가른다** — 스코프를 여는 것 · **무엇이 이름을 선언하는가** · 참조로 세는 것 · 안 세는 것 · 호이스팅 규칙 · 이름 공간. ⚠ 둘째 열이 앞 판에 없었고 그것이 가짜 엣지 52~58 의 자리다 | 같은 파일 또는 언어별 표 모듈 |
| 3 | Rust 표를 채운다 — 아래 「갈리는 자리」 | 새 표 |
| 4 | `use` 를 `ImportSet` 으로, `pub` 를 `ExportSet` 으로 산출한다. **정렬·중복 제거한 뒤** `digest` 를 부른다 | `crates/pal-extract/src/rust.rs` |
| 5 | `extract_detailed` 의 `Capable::not_built` 셋을 `Capable::Present` 로 바꾼다 | `rust.rs:322-327` |
| 6 | `grade_of(Rust)` 는 **안 만지고** 시험으로 잠근다. 다만 **왜 `L1` 인지를 그 자리에 적는다** — `L2` 의 정의가 *"스코프 해소된 참조"* 라 이 회차 뒤 문면이 어긋난다 | `classify.rs:207-238` |
| 7 | `crates/pal-cli/tests/` 에 **`.rs` 픽스처**를 세운다 — 지금 픽스처가 전부 `.ts`·`.kt` 라 이 경로가 CI 를 안 지난다 | `stitching.rs` 또는 새 시험 |
| 8 | `pal doctor` 의 `REFERENCES` 부재 선언이 아직 참인지 판정한다 — 그 사유가 *"작아서 안 걸린 것"* 이었다 | `crates/pal-cli/src/doctor.rs:290-301` |
| 9 | 낡는 문면을 갱신한다 | `rust.rs:1,298,326` · `classify.rs:226` · `schema/graph.toml:207-210` · `docs/plan/02-order.md:49` · `docs/plan/03-shortest-path.md:79-81` |

### 사전부검 라운드 2 가 프로토타입으로 잡은 것 — **계획이 이것을 지고 간다**

격리 사본에 `scopes.rs` 의 뼈대를 이식해 이 저장소 134 파일에 변형 13 개로 돌린 실측이다.

| 무엇 | 실측 | 어디를 고치나 |
|---|--:|---|
| `hoist_home` 이 「`impl` 이 스코프를 연다」를 상쇄한다 | 가짜 78 · 누락 17 | 아이템 호이스팅을 `hoist_home` 이 아닌 다른 장치로 표현한다 |
| 선언 축이 표에 없어 지역 이름이 안 묶인다 | 가짜 52~58 | 표에 **여섯째 열(무엇이 이름을 선언하는가)**을 더한다 |
| `#[…]` 속성 안 이름이 참조가 된다 | 참조 4,258 · 엣지 60 | `attribute_item` 조상 배제 |
| `cfg` 쌍둥이를 조용히 하나로 고른다 | 모호 39 · 선언자리 13 | 둘 이상이면 해소하지 않는다 |
| `for x in` · `if let Some(x)` 가 스코프를 안 연다 | 지역 판정 346 건 이동 | 여는 목록에 `for_expression`·`let_condition` 을 더한다 |
| 엣지의 62% 가 호출이 아니다 | 타입 1,182 · 호출 2,305 · 그 밖 2,529 | `pal touch` 라벨에 사실을 적는다 |
| `grade = "scoped"` 가 *"L2 이상에서만"* 인데 L1 이 낸다 | 3,752 | 스키마와 `ResolutionGrade` 문서 |

★ **기준선 일치 신호**: 그 프로토타입이 심볼 3,103 개를 냈고 실제 관측치는 3,110 이다.
이 회차의 구현이 그 근처를 안 내면 이식이 어딘가 다른 것이다.

**위층은 안 만진다** — `pal_core::file_edges`(`crates/pal-core/src/projection.rs:214`)가
`ScopeChain` 하나를 받으면 엣지를 만들고, 화면 표시(`crates/pal-cli/src/touch.rs:398`)도
이미 서 있다.

### Rust 에서 TypeScript 와 갈리는 자리 — **문법을 직접 찍어 실측했다**

원 실측은 [`grammar-probe.md`](grammar-probe.md).

| 무엇 | TypeScript | Rust |
|---|---|---|
| 스코프를 여는 것 | 함수류 8 · 클래스류 4 · 중괄호 5 | `mod_item` · `function_item` · `block` · `closure_expression` · `match_arm` · **`impl_item`** · **`for_expression`** · **`let_condition`** |
| **무엇이 이름을 선언하나** | `formal_parameters` · `lexical_declaration` · `variable_declaration` · `import_statement` (뼈대에 박혀 있다) | **하나도 안 겹친다** — `parameters`/`parameter` · `closure_parameters` · `let_declaration` · `match_arm` 의 `tuple_struct_pattern` · `for_expression` · `let_condition` |
| 속성 | 없다 | **`attribute_item` 아래는 전부 배제한다** — `#[cfg(test)]` 도 `token_tree` 라 앞형제 규칙이 안 걸린다 |
| 같은 이름이 둘일 때 | 드물다 | **`cfg` 쌍둥이가 실재한다** — 해소하지 않는다 |
| `impl` | 없다 | **스코프를 연다** ⟨승격 · 소유자 답 2026-09-07⟩ |
| `trait` | 없다 | **`impl` 과 같은 `ScopeKind::Impl` 을 연다** — 둘이 담는 것이 같다(연관 항목). 등록 목록에 이름이 없었고 독립 리뷰 R3 이 잡았다 |
| 호이스팅 | `function` 은 되고 `let`/`const` 는 TDZ | 아이템은 순서 무관, `let` 은 순서 있다. ⚠ **`hoist_home` 으로 표현하면 안 된다** — 그 함수가 `impl` 스코프를 건너뛴다. 그리고 **TDZ 는 Rust 에 없다** |
| 섀도잉 | 드물다 | **관용이다**(`let x = 1; let x = f(x);`) |
| 선언 이름의 노드 | `identifier` 계열 | **종류마다 다르다** — `struct_item`·`trait_item` 은 `type_identifier`, `mod_item`·`function_item` 은 `identifier` |
| 이름 공간 | 클래스는 값·타입 **둘 다** | 타입도 값 자리에서 `identifier` 로 쓰인다(358 건) — **둘 다에 넣는다** |
| 참조로 세는 것 | `identifier` · `shorthand_property_identifier` · `type_identifier` | `identifier` · `type_identifier` · `scoped_identifier` 의 **첫 세그먼트만** |
| 안 세는 것 | `property_identifier` · `predefined_type` | `field_identifier` · `primitive_type` · 라이프타임 · **경로 꼬리**(6,961 건) · **`use` 절 안 이름**(2,601 건) |
| 매크로 | 없다 | **`token_tree` 안은 구조가 없다** — `field_identifier` 도 `scoped_identifier` 도 안 생기고 전부 `identifier` 다. 앞 형제 토큰(`.`·`::`)을 보고 걸러야 한다 |

★ **마지막 행이 이 회차에서 가장 조심할 자리다.** 같은 규칙이 매크로 안팎에서 반대로 돈다 —
사전부검이 이것을 **문법 표면의 비균질성**이라는 새 범주로 냈고, 안 거르면 같은 파일의
아이템 이름과 겹치는 **509 건**이 가짜 엣지가 된다.

## 실측해 둔 수 — 이 회차가 다시 안 잰다

| 무엇 | 수 | 어디서 |
|---|--:|---|
| 참조 후보(`identifier` + `type_identifier`) | 76,416 | 사전부검 R1 |
| `token_tree` 안 `identifier` | 13,579 | 사전부검 R1 |
| 그중 같은 파일 아이템 이름과 겹치는 것 | 509 (파일 47) | 사전부검 R1 |
| 매크로 밖 경로 꼬리 | 6,961 | 사전부검 R1 |
| `use` 절 안 이름 | 2,601 | 사전부검 R1 |
| `impl` 미개방 시 동명 재선언 | 72 (파일 22) | 사전부검 R1 |
| 타입 이름의 값 자리 쓰임 | 358 | 사전부검 R1 |
| `impl` 블록 | 220 | 조건 설계 평가 R1 |
| 인라인 포맷 캡처 후보 | 1,486 | 조건 설계 평가 R1 |
| `pub` 아이템 (최상위 / 중첩) | 592 / 365 | 사전부검 R1 |

## 남은 것

1. 계획 1~9 를 순서대로 — 뼈대 공용화 → 언어 표 → Rust 표 → imports·exports → 능력 자리 → 등급 잠금 → Rust 픽스처 → doctor 판정 → 낡는 문면
2. **`V` 절 열둘을 돌린다 — 이것이 완수의 증인이다**
3. 독립 리뷰 3 라운드 → 효과 → 종료 보고 → 게이트 → push

## 착수 시 첫 명령

    cd /Users/incognito/dev/projects/palimpsest
    cat .palimpsest/rounds/2026-09-07-rust-scope-references/intent.md
    ./target/release/pal round conditions --file .palimpsest/rounds/2026-09-07-rust-scope-references/intent.md --json

## 실패한 접근

- **`impl` 을 스코프에서 빼는 것** — 사전부검이 격리 사본에서 재현했다. `ScopeChain::resolve`
  가 *"가장 앞선 것"* 을 돌려주므로 둘째 `new` 의 선언 이름 토큰이 첫째로 해소되고,
  `file_edges` 의 선언 걸러내기(`bound.declared_at == r.at` 한 줄)가 그것을 못 거른다.
- **「`REFERENCES` ≥ 100」을 합격선으로 삼는 것** — 참조 후보가 76,416 이라 가짜 엣지
  581 건만으로도 통과한다.
- **Rust 스코프 빌더를 새 파일로 쓰는 것** — `scopes.rs` 가 이미 같은 기계이고, 그 파일이
  실물에서 사서 넣은 방어 넷이 새 판에 없다.
- **아이템 호이스팅을 `hoist_home` 으로 표현하는 것** — 그 함수가 `impl` 스코프를 건너뛰어
  「`impl` 이 스코프를 연다」를 정확히 상쇄한다. 두 변형의 엣지 집합이 비트 단위로 같았다.
- **「기여 파일 수」를 완수 증인으로 삼는 것** — 변형 13 개가 전부 126 을 낸다.
- **TypeScript 의 TDZ 방어를 그대로 받는 것** — Rust 아이템에는 TDZ 가 없어서
  비함수 스코프의 전방 참조 52 건이 「선언 전 참조」로 뒤집힌다.

[#130]: https://github.com/hskim-ecoletree/palimpsest/issues/130
