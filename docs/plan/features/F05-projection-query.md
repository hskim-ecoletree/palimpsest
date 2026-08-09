# F05 — 질의 투영 (2층)과 질의 실행기

| 우선순위 | 의존 | 규모 | 크레이트 |
|---|---|---|---|
| **P0** | F03 · F04 | L | `pal-store::projection` · `pal-intent` · `pal-query` |

---

## 0. 경계 — 여기서 어디까지 하고 어디부터 F07인가

원래 이 문서의 스티칭 절은 "RawRef 해소(F07)"를 포함했고, F07은 F05에 의존했다. **순환이다.** 그리고 "2패스 스티칭 + `export_digest` 무효화"가 F05·F07 양쪽 완료 체크리스트에 있어 어디까지 하면 F05가 끝인지 정의되지 않았다. 가른다.

| | F05 (P0) | F07 (P1) |
|---|---|---|
| 노드 삽입 (`SYMBOL`·`FILE`) | **소유** | — |
| `EXPORTS` 채우기 | **소유** | — |
| 파일 **내** 참조 엣지 (L2a 산물, [F02 §3.5](F02-parse-extract.md)) | **소유** — `FileGraph.local_refs`를 그대로 옮긴다 | — |
| 엣지 테이블 **스키마**와 순회 API | **소유** | — |
| 파일 **간** 해소 (`RawRef` → 엣지) · `MODULE_MAP` | 테이블만 정의, **비어 있다** | **소유** |
| `export_digest` 무효화 전파 · `IMPORTED_BY` | 테이블만 정의 | **소유** |
| `Envelope` · 예산 · 절단 · 질의 로그 | **소유** | 사용 |

**그래서 F05가 끝나면 무엇이 도나**: `ledger.snapshot` · `symbol.resolve` · `symbol.contains` · 파일 내 호출 관계. 파일 경계를 넘는 질의는 `UnresolvedRef{정의 없음}`으로 정직하게 답한다 — **빈 답이 아니라 등급이 낮은 답이고, 그것이 이 제품의 정상 상태다.**

---

## 1. 왜

1층은 파일 단위 조각이다. 질의는 **저장소 전체**를 가로지른다 — "이 심볼을 부르는 것 전부", "이 좌표에 걸린 결정 전부". 그 조회를 상수/로그 시간에 하려면 색인이 필요하다.

그리고 **모든 응답에 `Envelope`를 붙이는 실행기**가 여기 산다. 답만 돌려주는 경로가 타입 수준에 존재하지 않게 만드는 자리.

**달성 기여**: [목표 §3.1·3.2](../00-goals.md#3-이-도구가-다른-것과-다른-지점--넷) — 절단과 공백이 응답에 데이터로 실리는 것이 이 기능의 구현이다.

---

## 2. 왜 자체 인덱스인가 — 결정과 근거

[스택 §2.3](../00-stack.md#23-2층을-그래프-db에-얹지-않는-이유--이-문서에서-가장-중요한-결정)에 전문이 있다. 요약:

1. 필요한 연산은 넷뿐 — 키 조회 / 인접 순회 / 역방향 색인 / **예산 절단이 있는** 제한 깊이 탐색.
2. **결정적 이유**: 질의 도중 잘라내고 **얼마나 무슨 이유로 잘랐는지 응답에 실어야** 한다. Cypher/SQL의 `LIMIT`은 "한도에 걸린 지점의 사유별 분해"를 표현하지 못한다. 조용한 절단 금지는 이 제품의 정체성이다.
3. 임베디드 그래프 DB 지형이 실제로 무너졌다(Kuzu 아카이브 → 포크 4개).
4. **2층은 캐시**라서 자체 구현의 최대 비용(마이그레이션·백업)이 0이다. **의도 저장소는 이 면제를 받지 못한다** — 거기만 스키마 버전과 JSONL 내보내기를 진다(§3.1).
5. 그래프 DB 방향은 `pal export`(Cypher/GraphML/Parquet)로 만족시킨다.

---

## 3. 저장 레이아웃 — redb 테이블

```rust
// 노드
const SYMBOL:      TableDefinition<SymbolId, SymbolNode>       // id → (경로, 이름, 종류, body_digest, span, 등급)
const FILE:        TableDefinition<FileId, FileNode>
const UNRESOLVED:  TableDefinition<UnresolvedId, UnresolvedRef>

// 엣지 — 인접 리스트. 키에 방향과 종류를 접두어로 넣어 범위 스캔이 곧 인접 조회가 되게
const EDGE_OUT:    TableDefinition<(SymbolId, EdgeKind, SymbolId), EdgeAttrs>
const EDGE_IN:     TableDefinition<(SymbolId, EdgeKind, SymbolId), EdgeAttrs>   // ← touch 의 근간

// 결박 색인 (F09~F12) — ★ 실체가 아니라 색인이다. 실체는 의도 저장소에 있다
const BOUND_BY:    TableDefinition<(SymbolId, BindingId), ()>   // 역방향 — touch 가 이걸 읽는다
const WATCH:       TableDefinition<(BindingId, SymbolId), Blake3>  // 감시 집합의 digest 스냅샷

// 스티칭 보조
const EXPORTS:     TableDefinition<(FileId, Name), SymbolId>
const IMPORTED_BY: TableDefinition<(FileId, FileId), ()>        // 역 import — R-05 무효화 전파 (F07이 채운다)
const MODULE_MAP:  TableDefinition<ModuleSpecifier, FileId>     // R-04 해소 결과 캐시 (F07이 채운다)

// 대장·로그
const LEDGER:      TableDefinition<SnapshotId, Ledger>
const QUERY_LOG:   TableDefinition<(SnapshotId, Seq), QueryLogEntry>   // append-only. F17 커버리지의 입력
```

### 3.1 의도 저장소 — **별도 파일** ([R-21](../00-risks.md#r-21))

```rust
// intent.redb — pal-intent 크레이트가 소유. 지우는 API가 없다.
const BINDING:     TableDefinition<BindingId, Binding>          // 결박 실체
const ASSERTION:   TableDefinition<AssertionId, Asserted>       // 승인된 라벨·결정
const OBSERVATION: TableDefinition<ObservationId, Observed>     // 관측 원문 (F16)
const APPROVAL:    TableDefinition<(EntityId, Seq), ApprovalEvent>  // 승인·거부 이력
const ALIAS:       TableDefinition<RepoId, RepoId>              // 저장소 재배치 흡수 (R-08)
```

**왜 파일을 가르나**: 위 다섯은 1층에서 재생되지 않는다. git에도 없다. 사람이 지불한 노동 그 자체다. 같은 파일에 두면 "2층이 손상됐으니 지우고 재구축"이 그것을 지우는 명령이 되고, **재구축 등가성 검사가 그 유실을 확인하고 통과한다.** 파일이 갈린 것 자체가 대응이다.

`BOUND_BY`·`WATCH`가 2층에 남는 이유: **역방향 조회 성능은 파생이다.** 지워도 의도 저장소를 훑어 다시 만들 수 있다.

**설계 포인트**

- **`EDGE_IN`을 따로 유지한다.** 역방향 조회(`touch`, "누가 이걸 부르나")가 이 제품의 1순위 질의다. 정방향만 두고 스캔하면 O(전체)다. 저장 2배 비용을 지불하고 조회를 O(차수)로 만든다.
- **키에 튜플을 쓰고 접두 범위 스캔.** redb가 정렬 키를 지원하므로 `(id, kind, ..)` 범위 스캔이 곧 "이 심볼의 이 종류 엣지 전부"다. 별도 인덱스 구조가 필요 없다.
- **`SymbolId`는 blake3 32바이트.** 키가 크지만 전역 유일이라 조인이 필요 없다. u64 축약을 쓰면 매핑 테이블과 충돌 처리가 생긴다 — **먼저 32바이트로 만들고, 크기가 문제로 실측되면 그때 바꾼다.**

---

## 4. 스티칭 — 1층 → 2층

```
── 1패스 (F05 소유) ─────────────────────────── 파일 독립. 병렬 가능
for 각 파일 in Snapshot:
    FileGraph = cache.get(blob) or extract()        # F04
    SYMBOL / FILE 노드 삽입
    EXPORTS 채우기
    local_refs → EDGE_OUT / EDGE_IN 삽입             # 파일 내 엣지 (L2a, F02 §3.5)
    raw_refs  → UNRESOLVED{정의 없음} 로 일단 적재    # F07 이 나중에 승격시킨다

── 2패스 (F07 소유) ─────────────────────────── 전역 상태 필요
    RawRef 해소 — EXPORTS · MODULE_MAP 조회
    UNRESOLVED → EDGE 로 승격 · IMPORTED_BY 구축
```

**2패스가 필요한 이유**: 파일 A가 B를 참조하는데 B가 아직 안 읽혔을 수 있다. 1패스에서 모든 파일의 `EXPORTS`를 채우고, 2패스에서 해소한다.

**F05에서는 1패스만 만든다.** 그리고 2패스가 없는 상태가 **동작하는 상태**다 — 파일 밖 참조는 `UnresolvedRef`로 남고, 그것이 F07이 무엇을 만들지 알려주는 밀도 지도가 된다(F08). 이 순서가 성립하는 것이 "공백이 1급 사실"이라는 설계의 실질적 이득이다.

**쓰기 트랜잭션 배치**: redb 쓰기 트랜잭션은 커밋마다 fsync 비용이 있다. 파일 1,000개 단위로 묶어 커밋한다. 중단되면 그 배치만 잃고 다시 스티칭한다(1층이 있으므로 재파싱 없음).

**증분**: 무효화 계산(F07)이 준 영향 파일 집합에 대해서만 다시 돈다.

---

## 5. 질의 실행기 — `Envelope`와 예산 절단

### 5.1 모든 질의가 `Envelope`를 반환한다

```rust
pub struct Envelope<T> {
    answer: T,
    snapshot: Snapshot,
    projection: ProjectionFreshness,   // Fresh | StaleProjection{behind: usize} | Worktree
    coverage: Coverage,                // 미해소 N · 범위 밖 N · 경유 언어 등급
    capabilities: CapabilitySet,       // 이 빌드가 답할 수 있는 산출 목록 (스택 §5.3)
    ledger: LedgerRef,                 // 요약 한 줄 + 상세 질의로의 참조 (R-11)
    elision: Elision,                  // 잘린 것
}

// 실행기 진입점이 Envelope 만 반환한다 → 벌거벗은 답을 낼 방법이 없다
pub fn execute(q: NamedQuery, ctx: &QueryCtx) -> Result<Envelope<QueryResult>>;
```

**"벗길 수 없다"의 실제 형태.** 필드를 private으로 잠그면 CLI가 사람이 읽는 표를 그릴 수 없다 — 렌더링에는 읽기 접근이 필요하다. 그래서 규칙을 실행 가능한 형태로 적는다:

| | |
|---|---|
| 있는 것 | 모든 필드의 **읽기 접근자**. 표 렌더링·요약 생성에 필요하다 |
| **없는 것** | `into_answer()` · `Deref<Target=T>` · `answer`만 담는 생성자. **`Envelope`를 버리고 `T`만 들고 나가는 경로** |
| 검사 | 표면 크레이트의 **골든 JSON**에 `Envelope` 필드가 전부 있는가(F06). 타입이 아니라 산출로 검사한다 |

타입으로 100% 막히지 않는다는 것을 인정하고, 대신 **빠지면 골든이 깨지는** 자리에 검사를 둔다.

### 5.2 예산 절단 — 이 실행기가 존재하는 이유

```rust
pub struct Budget {
    candidate_set_max: usize,    // K = 32
    path_product_max: u64,       // B = 10^4
    depth_max: u8,               // 3 홉 (candidate 경유는 1홉으로 계산)
    node_max: usize,             // 뷰 500
}

pub struct Elision {
    truncated: Vec<(ElisionReason, usize)>,   // 사유별 건수
    limits_hit: Vec<(BudgetName, u64)>,       // 어느 상한에 얼마나
}
impl Elision { pub fn none() -> Self }        // 절단이 없어도 명시적으로 만들어야 한다
```

탐색은 예산을 들고 다니는 BFS다.

```rust
fn traverse(start: SymbolId, b: &Budget, el: &mut Elision) -> Vec<Path> {
    let mut product: u64 = 1;
    // ... 각 홉에서
    if edge.grade == Candidate {
        if edge.candidates.len() > b.candidate_set_max {
            el.push(ElisionReason::CandidateOverflow, 1);
            emit_unresolved(UnresolvedReason::TooManyCandidates);   // F08
            continue;                                                // 이 가지를 버린다
        }
        product = product.saturating_mul(edge.candidates.len() as u64);
        if product > b.path_product_max {
            el.push(ElisionReason::PathProductExceeded, 1);
            emit_residual(ResidualReason::CandidateExplosion);      // F15
            break;                                                   // 탐색 중단
        }
    }
}
```

**이 코드가 남의 질의 언어로는 표현되지 않는다.** 잘랐다는 사실, 왜 잘랐는지, 몇 건인지가 전부 결과에 실린다.

### 5.3 질의 로그

**모든 질의 실행이 접근한 좌표를 `QUERY_LOG`에 남긴다.** F17(커버리지 계산)이 나중에 새 장치를 만들지 않고 **F05부터 쌓인 로그를 읽게** 하기 위해서다. 처음부터 켜지 않으면 그 기능은 데이터가 없어 착수할 수 없다.

기록: `(snapshot, seq, query_name, args_hash, accessed_coords, elision, duration)`.

---

## 6. 이슈와 대응

| 이슈 | 왜 | 대응 | 안 되면 |
|---|---|---|---|
| **재귀 탐색이 규모에서 안 선다** | 10⁷ 엣지 | 예산이 이미 상한이다. 3홉 + B=10⁴. **무한 탐색 API를 제공하지 않는다** | 깊이를 2로 낮추고 그 사실을 `elision`에 |
| **`EDGE_IN` 저장 2배** | 양방향 유지 | 감수한다. 역방향이 1순위 질의 | 필요시 종류별 선택적 유지 |
| **쓰기 트랜잭션 병목** | redb는 단일 라이터 | 배치 커밋(1,000파일) + 읽기는 MVCC로 무관 | 배치 크기 튜닝 |
| **스티칭 메모리** | `EXPORTS` 전역 맵을 메모리에 | redb 테이블로 두고 디스크에 맡긴다(OS 캐시가 처리) | 샤딩 |
| **`SymbolId` 32바이트 키** | 인덱스 부피 | 먼저 실측. 문제면 u64 축약 + 충돌 테이블 | — |
| **2층 손상** | 프로세스 중단 | **지우고 1층 + 의도 저장소에서 재구축.** 2층에는 원본이 없다([R-21](../00-risks.md#r-21)) | — |
| **의도 저장소 손상** | 같은 원인, 다른 처분 | **재구축 불가.** JSONL 내보내기에서 복구. 그래서 내보내기가 상시 유지된다 | 사용자에게 유실 범위를 명시 — 조용히 빈 채로 열지 않는다 |
| **동시 접근** (CLI + MCP 서버) | redb 파일 락 | 읽기는 동시 가능, 쓰기는 하나. 재추출은 MCP 서버 쪽이 소유하고 CLI는 읽기 전용으로 붙는다 | 락 대기 타임아웃 + 명확한 에러 |

---

## 7. 고려한 대안

| 대안 | 기각 이유 |
|---|---|
| **LadybugDB / Kuzu 계열** | §2. 지형 리스크 + 절단 기록 불가 |
| **Neo4j 서버** | 상주 프로세스·별도 런타임이 설치 비용 제약과 정면 충돌. "그것이 없으면 답을 못 낸다"가 되면 호스트 독립성도 깨진다. **옵션 내보내기 대상으로만** |
| **SQLite + 재귀 CTE** | 기본에서 뺀다(C 컴파일). **대조군으로 유지** — 같은 질의를 CTE로 짜서 자체 인덱스와 답이 일치하는지 검사하면 인덱스 버그가 잡힌다 |
| **`petgraph` 인메모리 그래프** | 10⁶ 심볼을 매번 메모리에 올릴 수 없다. 다만 **부분 그래프 알고리즘(도달성·SCC)에는 쓸 수 있다** — 탐색 결과를 petgraph로 옮겨 계산하는 용도로 채택 |
| **전용 그래프 파일 포맷 직접 설계** | redb가 이미 B+tree·트랜잭션·MVCC를 준다. 바퀴를 다시 만들 이유가 없다 |
| **`Envelope` 없이 답만 반환하고 메타는 옵션** | 옵션은 비게 된다. 첨부 필수는 타입으로 강제한다 |

---

## 8. 검증

- **백엔드 대조** — 같은 질의 집합을 자체 인덱스와 SQLite CTE 구현에서 실행해 **답이 일치**하는가. 갈리면 인덱스 버그다.
- **재구축** — 2층 삭제 → **1층 + 의도 저장소**에서 재구축 → 골든과 일치 **+ 결박·승인 건수 불변**. **CI 상시.**
- **예산 절단 테스트** — 인위적으로 K·B를 낮춰 절단을 유발하고 `elision`이 정확한 건수·사유를 담는가.
- **`Envelope` 누락 불가 검사** — 표면이 내는 모든 질의 응답의 골든 JSON에 `Envelope` 필드가 전부 있는가(§5.1).
- **벤치**(`criterion`) — 코퍼스 규모에서 ① 심볼 조회 ② 1홉 역방향 ③ 3홉 BFS ④ 전체 재구축. 규모는 [R-24](../00-risks.md#r-24)에 따라 10⁴급에서 재고 **선형성**을 함께 기록한다.

---

## 9. 완료 체크리스트

- [ ] redb 테이블 정의 전부(§3) — **2층과 의도 저장소를 별도 파일로**
- [ ] `pal-intent` 크레이트 (지우는 API 없음) + JSONL 내보내기/읽기
- [ ] **1패스 스티칭**(노드·`EXPORTS`·파일 내 엣지) + 배치 커밋 — 2패스는 [F07](F07-reference-resolution.md)
- [ ] `Envelope<T>` + `capabilities` + 읽기 접근자만(벗기는 경로 없음)
- [ ] `Capable<T>` 타입과 표면 렌더링(“이 빌드에 없음”)
- [ ] `Budget` + `Elision` + 절단 BFS — 상수는 `pal-core::budget` 한 곳([스택 §5.5](../00-stack.md#55-예산-상수는-한-곳에-있고-초기값은-자리표시다))
- [ ] 질의 로그 append-only (**F05부터 켠다**)
- [ ] SQLite CTE 대조 구현(테스트 전용)
- [ ] 재구축 등가성 CI + **캐시 폐기 격리 CI**
- [ ] `Envelope` 누락 골든 검사
- [ ] 4종 벤치 + 선형성 기록
