# `A5-a` 관측 — 산출된 파일 간 엣지가 재수출 경로를 지났나

> 뜬 자: **메인** · 2026-09-10 · 커밋 `ff6f985`
>
> `A5-a` 원문 — *"`crates/pal-core/src/lib.rs` 는 최상위 직접 정의가 0 이고 `pub use`
> 재수출이 34 줄이라 `use pal_core::{…}` 로 들어오는 이름이 거의 전부 재수출 경유다.
> **산출된 파일 간 엣지 중 그 경로를 지난 것이 0 임을 저장소 산출에서 확인한다.**"*
>
> ⚠ **픽스처가 아니라 저장소 산출에서 쟀다** — 그 조건이 요구한 자리다.

## 어떻게 쟀나

`pal query graph.dump --json` 으로 노드와 엣지를 전부 받아, 양 끝 심볼의 파일이 다른
엣지만 골라 **대상 파일이 크레이트 뿌리 `lib.rs` 인 것**을 셌다. 재수출은 `SymbolNode`
를 안 만들므로, 재수출 경유 엣지가 있다면 대상이 뿌리 파일로 찍힌다.

| | 값 |
|---|---:|
| 파일 간 엣지(서로 다른 파일 · 중복 제거) | 1259 |
| 그중 대상이 크레이트 뿌리 `lib.rs` | **40** |
| 그중 **재수출을 지난 것** | **0** |

## 40 건이 전부 뿌리 파일의 **직접 정의**다

| 대상 파일 | 엣지 | 대상 심볼 |
|---|---:|---|
| `crates/pal-git/src/lib.rs` | 23 | `GixRepo`(struct) · `GixRepo::open` · `GixRepo::discover` · `WorktreeState` |
| `crates/pal-query/src/lib.rs` | 14 | `BoundIndex`·`NamedQuery`·`NamedQuery::parse`·`QueryCtx`·`QueryError`·`QueryResult` |
| `crates/pal-core/src/lib.rs` | 2 | `traverse`(module) |
| `crates/pal-extract/src/lib.rs` | 1 | `scopes`(module) |

`pal-git`·`pal-query` 의 뿌리는 **정의를 직접 담는 파일**이다 — 잠근 축1 의 셋째 칸
(*"형제 크레이트 평평 경로 중 대상이 그 크레이트 루트 `lib.rs` 의 최상위 `pub` 인 것"*)
이 그 자리를 이미 안으로 넣었다.

`pal-core` 의 둘은 **`mod traverse;`**(`lib.rs:45`)를 가리킨다 — 모듈 선언이고 재수출이
아니다. `pal-extract` 의 하나도 같다.

**그러므로 `A5-a` 의 요구는 섰다 — 재수출 경유 엣지 0.** 구조가 그것을 보증한다:
해소기는 대상 파일의 `SymbolNode` 만 보고 `pub use` 는 심볼을 안 만든다.

## ⚠ 같은 측정에서 나온 다른 것 — **`E3` 의 표본에 들어갈 자리**

`crates/pal-core/src/lib.rs` 는 같은 이름을 **두 곳**에 싣는다:

- `lib.rs:45` — `mod traverse;` (**타입 이름 공간**)
- `lib.rs:142` — `pub use traverse::{Step, traverse};` (**값 이름 공간**의 함수)

`use pal_core::{…, traverse}` 한 줄이 Rust 에서 **둘 다** 들여온다. 그런데 참조 자리는
`traverse(&start.id, …)`(`pal-query/src/lib.rs:572`)와 `traverse(id, &budget, …)`
(`pal-query/tests/bench.rs:158`)로 **함수를 부른다.** 오늘 선 엣지는 **모듈**을 가리킨다.

**대상 파일 안에서 후보가 유일했으므로 `A6` 이 안 걸린다** — 함수 `traverse` 의 심볼은
`traverse.rs` 에 살고 `lib.rs` 에는 재수출 줄만 있다. 그래서 `by_name[(lib.rs,
"traverse")]` 의 답이 모듈 하나뿐이었다.

⚠ **이것을 「거짓 엣지」로 셀지는 이 관측이 정하지 않는다.** `use` 가 두 이름 공간을 다
들여오므로 모듈로 가는 엣지도 그 `use` 에 대해서는 참이고, 어긋난 것은 **참조 자리가
고른 이름 공간**이다. 그 판정은 `E3` 의 자리이고, 잠긴 의도가 그것을 **사람이 판정하는
잔여**로 이미 선언했다. **표본 30 을 뽑을 때 이 형태가 모집단에 있다는 것을 여기 적어
둔다** — 안 적으면 표본이 그것을 만나도 무엇인지 모른다.
