<!-- 이 파일은 `cargo xtask schema-doc` 이 낸다. 손으로 고치지 않는다. -->
<!-- 정본은 schema/graph.toml 이고 CI 가 둘의 일치를 센다. -->

# 그래프 스키마 v1

노드 라벨 **7개** · 엣지 타입 **7개**. 자라는 것 자체가 관측 대상이다([DESIGN §1.2](DESIGN.md)).

## 노드

| 라벨 | 출처 | Rust 타입 | 키 | 상태 |
|---|---|---|---|---|
| `Actor` | `extracted` | `Actor` | `id` | 값이 선다 |
| `Binding` | `asserted` | `Binding` | `id` | 값이 선다 |
| `Change` | `extracted` | `Change` | `id` | 값이 선다 |
| `Defect` | `extracted` | `Defect` | `id` | 값이 선다 |
| `Journey` | `asserted` | `Journey` | `name` | **자리만** — F19 가 만든다 |
| `Symbol` | `extracted` | `SymbolNode` | `id` | 값이 선다 |
| `UnresolvedRef` | `extracted` | `UnresolvedRef` | `site`, `name` | **자리만** — F08 가 만든다 |

### 속성

| 노드 | 속성 | 형 | 생산자 | 필수 |
|---|---|---|---|---|
| `Actor` | `display` | `string` | `extractor` | 예 |
| `Binding` | `note` | `string` | `human` | 예 |
| `Binding` | `bound_at` | `snapshot` | `machine-record` | 예 |
| `Binding` | `watch` | `watch_entry[]` | `machine-record` | 예 |
| `Change` | `kind` | `enum:ChangeKind` | `extractor` | 예 |
| `Change` | `summary` | `string` | `extractor` | 예 |
| `Change` | `at` | `snapshot` | `machine-record` | 예 |
| `Defect` | `description` | `string` | `extractor` | 예 |
| `Defect` | `at` | `snapshot` | `machine-record` | 예 |
| `Journey` | `entry_points` | `coord[]` | `human` | 예 |
| `Journey` | `passes_through` | `coord[]` | `human` | 예 |
| `Journey` | `expected_effects` | `effect[]` | `human` | 예 |
| `Symbol` | `path` | `repo_path` | `extractor` | 예 |
| `Symbol` | `container` | `string[]` | `extractor` | 예 |
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
| `AUTHORED_BY` | `Change` | `Actor` | many-to-one | `exact` (고정) | `extracted` | 해당 없음 | `at` | `Change::author` |
| `BOUND_TO` | `Binding` | `Symbol` | many-to-one | `exact` (고정) | `asserted` | 해당 없음 | `bound_at` | `Binding::target` |
| `FOLLOWS` | `Change` | `Change` | many-to-many | `exact` (고정) | `extracted` | 해당 없음 | `at` | `Change::parents` |
| `INTRODUCED_BY` | `Defect` | `Change` | many-to-one | `candidate` (고정) | `extracted` | 해당 없음 | `at` | `Defect::introduced_by` |
| `MANIFESTS_AT` | `Defect` | `Symbol` | many-to-many | `exact` (고정) | `extracted` | 해당 없음 | `at` | `Defect::manifests_at` |
| `RESOLVED_BY` | `Defect` | `Change` | many-to-one | `exact` (고정) | `extracted` | 해당 없음 | `at` | `Defect::resolved_by` |
| `TOUCHES` | `Change` | `Symbol` | many-to-many | `exact` (고정) | `extracted` | 해당 없음 | `at` | `Change::touches` |
