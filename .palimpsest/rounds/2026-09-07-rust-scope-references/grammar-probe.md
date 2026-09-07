# tree-sitter-rust 문법 실측

> `tree-sitter-rust @ 77a3747` (Cargo.toml:155) 에서 파스 트리를 직접 찍어 잰 것.
> 잰 도구는 일회성 `examples/rust_grammar_probe.rs` 였고 **재고 나서 지웠다** —
> 회차가 장치를 안 만든다(§11 ③ (가)).

## 계획이 가정한 것 중 맞은 것

| 가정 | 실측 |
|---|---|
| `use` 의 네 갈래 | `scoped_identifier` · `scoped_use_list`(→`use_list`) · `use_as_clause` · `use_wildcard` — 넷 다 있다 |
| `mod_item` · `function_item` · `block` 이 스코프를 연다 | 있다. `mod_item`·`impl_item`·`trait_item` 의 본문은 `declaration_list`, 함수 본문은 `block` |
| `field_expression` 의 필드를 배제한다 | 필드는 **별도 노드 타입 `field_identifier`** 라 배제가 문법으로 갈린다 |
| `macro_invocation` 의 `token_tree` | 있다 |

## 계획이 못 본 것 — **셋**

### ① 선언 이름의 노드 타입이 종류마다 다르다

```
struct_item → type_identifier "S"        ← identifier 가 아니다
trait_item  → type_identifier "T"
mod_item    → identifier "m"
function_item → identifier "f"
```

**`identifier` 만 이름으로 보면 타입 선언이 통째로 스코프에 안 들어간다.** 그러면
`fn caller() { let s = S; }` 의 `S` 가 `OutsideFile` 로 떨어져 엣지가 안 선다.

### ② 매크로 안에서는 경로가 평평하다 — **`scoped_identifier` 가 아니다**

```
macro_invocation
  identifier "assert_eq"        ← 매크로 이름도 identifier 다
  token_tree
    identifier "m"  ::  identifier "f"   ← 구조가 없다. 둘 다 벗은 identifier
```

같은 자리를 실코드에서 쓰면 `scoped_identifier { identifier "m" :: identifier "f" }` 로
**구조가 선다.** 그러므로 *"`scoped_identifier` 의 첫 세그먼트만 참조로 센다"* 는 규칙이
**매크로 안에서는 안 통하고**, 뒷 세그먼트(`f`)도 참조가 된다.

**이것이 거짓 엣지의 자리다** — 같은 파일에 `f` 라는 이름의 선언이 있으면 `m::f` 의 `f`
가 그것으로 해소된다. 파일 안 해소만 하므로 대개는 `OutsideFile` 로 떨어지지만
**대개**는 판정이 아니다.

### ③ 매크로 이름 자체가 `identifier` 다

`assert_eq!` 의 `assert_eq` 가 `macro_invocation` 의 직계 `identifier` 다.
세면 같은 파일에 `macro_rules! assert_eq` 가 없는 한 `OutsideFile` 이고,
**`OutsideFile` 건수를 부풀린다.** 그 수는 `RefCounts::unresolved` 로 나가고
`pal touch` 의 「내가 모르는 것」이 그것을 쓰게 될 자리다.
