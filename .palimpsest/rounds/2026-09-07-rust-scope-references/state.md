# 상태 — Rust 스코프와 참조 엣지

> 회차 `2026-09-07-rust-scope-references` · 착수 커밋 `a774053` · 이슈 [#130]
> 잠긴 의도는 [`intent.md`](intent.md) 가 진다. **이 파일은 요약이지 정본이 아니다.**

## 지금 단계

**완수 조건 설계 평가 라운드 2** (승인 앞). 사전부검 1 라운드와 조건 설계 평가 1 라운드가
끝났고 소유자 승격 셋이 답을 받았다.

## 계획 — 무엇을 어느 순서로 만지나

⚠ **앞 판의 계획 1 은 「새 파일」이었고 소유자가 「공용화」로 뒤집었다.**

| # | 무엇 | 어디 |
|---|---|---|
| 1 | `scopes.rs` 의 **뼈대를 언어 중립으로 뺀다** — 2 패스(선언·참조) · `scope_at` 을 `Node::id` 로 잡기 · `hoist_home` · `symbol_at` 을 시작 바이트로 잇기. TypeScript 가 실물에서 사서 넣은 방어 넷이 그 뼈대에 남는다 | `crates/pal-extract/src/scopes.rs` |
| 2 | **노드 종류 표만 언어별로 가른다** — 스코프를 여는 것 · 참조로 세는 것 · 안 세는 것 · 호이스팅 규칙 · 이름 공간 | 같은 파일 또는 언어별 표 모듈 |
| 3 | Rust 표를 채운다 — 아래 「갈리는 자리」 | 새 표 |
| 4 | `use` 를 `ImportSet` 으로, `pub` 를 `ExportSet` 으로 산출한다. **정렬·중복 제거한 뒤** `digest` 를 부른다 | `crates/pal-extract/src/rust.rs` |
| 5 | `extract_detailed` 의 `Capable::not_built` 셋을 `Capable::Present` 로 바꾼다 | `rust.rs:322-327` |
| 6 | `grade_of(Rust)` 는 **안 만지고** 시험으로 잠근다. 다만 **왜 `L1` 인지를 그 자리에 적는다** — `L2` 의 정의가 *"스코프 해소된 참조"* 라 이 회차 뒤 문면이 어긋난다 | `classify.rs:207-238` |
| 7 | `crates/pal-cli/tests/` 에 **`.rs` 픽스처**를 세운다 — 지금 픽스처가 전부 `.ts`·`.kt` 라 이 경로가 CI 를 안 지난다 | `stitching.rs` 또는 새 시험 |
| 8 | `pal doctor` 의 `REFERENCES` 부재 선언이 아직 참인지 판정한다 — 그 사유가 *"작아서 안 걸린 것"* 이었다 | `crates/pal-cli/src/doctor.rs:290-301` |
| 9 | 낡는 문면을 갱신한다 | `rust.rs:1,298,326` · `classify.rs:226` · `schema/graph.toml:207-210` · `docs/plan/02-order.md:49` · `docs/plan/03-shortest-path.md:79-81` |

**위층은 안 만진다** — `pal_core::file_edges`(`crates/pal-core/src/projection.rs:214`)가
`ScopeChain` 하나를 받으면 엣지를 만들고, 화면 표시(`crates/pal-cli/src/touch.rs:398`)도
이미 서 있다.

### Rust 에서 TypeScript 와 갈리는 자리 — **문법을 직접 찍어 실측했다**

원 실측은 [`grammar-probe.md`](grammar-probe.md).

| 무엇 | TypeScript | Rust |
|---|---|---|
| 스코프를 여는 것 | 함수류 8 · 클래스류 4 · 중괄호 5 | `mod_item` · `function_item` · `block` · `closure_expression` · `match_arm` · **`impl_item`** |
| `impl` | 없다 | **스코프를 연다** ⟨승격 · 소유자 답 2026-09-07⟩ |
| 호이스팅 | `function` 은 되고 `let`/`const` 는 TDZ | 아이템은 순서 무관, `let` 은 순서 있다 |
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

1. 완수 조건 설계 평가 라운드 2 — 개정한 조건을 다시 건다
2. 소유자 승인
3. 실행 → 검증 → 효과 → 종료

## 실패한 접근

- **`impl` 을 스코프에서 빼는 것** — 사전부검이 격리 사본에서 재현했다. `ScopeChain::resolve`
  가 *"가장 앞선 것"* 을 돌려주므로 둘째 `new` 의 선언 이름 토큰이 첫째로 해소되고,
  `file_edges` 의 선언 걸러내기(`bound.declared_at == r.at` 한 줄)가 그것을 못 거른다.
- **「`REFERENCES` ≥ 100」을 합격선으로 삼는 것** — 참조 후보가 76,416 이라 가짜 엣지
  581 건만으로도 통과한다.
- **Rust 스코프 빌더를 새 파일로 쓰는 것** — `scopes.rs` 가 이미 같은 기계이고, 그 파일이
  실물에서 사서 넣은 방어 넷이 새 판에 없다.

[#130]: https://github.com/hskim-ecoletree/palimpsest/issues/130
