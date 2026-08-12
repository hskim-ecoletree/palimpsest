<!-- 이 파일은 `cargo xtask schema-doc` 이 낸다. 손으로 고치지 않는다. -->
<!-- 정본은 schema/graph.toml 이고 CI 가 둘의 일치를 센다. -->

# 그래프 스키마 v1

노드 라벨 **3개** · 엣지 타입 **1개**. 자라는 것 자체가 관측 대상이다([DESIGN §1.2](DESIGN.md)).

## 노드

| 라벨 | 출처 | Rust 타입 | 키 | 상태 |
|---|---|---|---|---|
| `Binding` | `asserted` | `Binding` | `id` | 값이 선다 |
| `Symbol` | `extracted` | `SymbolNode` | `id` | 값이 선다 |
| `UnresolvedRef` | `extracted` | `UnresolvedRef` | `site`, `name` | **자리만** — F08 가 만든다 |

### 속성

| 노드 | 속성 | 형 | 생산자 | 필수 |
|---|---|---|---|---|
| `Binding` | `note` | `string` | `human` | 예 |
| `Binding` | `bound_at` | `snapshot` | `machine-record` | 예 |
| `Binding` | `watch` | `watch_entry[]` | `machine-record` | 예 |
| `Symbol` | `path` | `repo_path` | `extractor` | 예 |
| `Symbol` | `name` | `string` | `extractor` | 예 |
| `Symbol` | `kind` | `enum:SymbolKind` | `extractor` | 예 |
| `Symbol` | `body` | `digest` | `extractor` | 예 |
| `Symbol` | `span` | `span` | `extractor` | 예 |
| `Symbol` | `identity` | `enum:IdentityGrade` | `extractor` | 예 |
| `UnresolvedRef` | `reason` | `enum:UnresolvedReason` | `extractor` | 예 |
| `UnresolvedRef` | `attempts` | `attempt[]` | `machine-record` | 예 |

## 엣지

**모든 엣지가 공통 넷을 진다** — 해소 등급 · 출처 · 근거 · 발생 `Snapshot`.
넷이 없는 엣지 타입은 등록되지 않는다.

| 엣지 | from | to | 카디널리티 | 등급 | 출처 | 근거 | Snapshot | 실린 자리 |
|---|---|---|---|---|---|---|---|---|
| `BOUND_TO` | `Binding` | `Symbol` | many-to-one | `exact` (고정) | `asserted` | 해당 없음 | `bound_at` | `Binding::target` |
