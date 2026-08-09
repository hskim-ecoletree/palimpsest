# 스택과 구조 — 무엇으로 만드는가

> 전 기능 공통. 기능 문서는 이 문서의 선택을 전제하고, 뒤집을 근거가 생기면 여기로 되돌린다.
> 각 선택에 **① 왜 ② 무엇을 기각했나 ③ 언제 뒤집히나**를 붙인다. 셋이 없는 선택은 선택이 아니라 취향이다.

---

## 1. 언어 — Rust (edition 2024)

### 왜

| 근거 | 내용 |
|---|---|
| **제약을 타입으로 옮길 수 있다** | 이 설계의 핵심 규칙 다섯(선택 필드 금지 / `evidence_refs` 빈 값 저장 거부 / 출처 필드 불변 / `clean` 부재 / 절단 기록 필수)이 전부 *"그런 값은 애초에 만들 수 없다"*로 표현될 때만 규율에서 벗어난다. Rust의 뉴타입·private 생성자·소유권·비-exhaustive 열거가 이것을 컴파일 타임으로 옮긴다. Go·TypeScript에서는 같은 것이 런타임 검사, 즉 규율로 남는다 |
| **파서 생태계** | tree-sitter가 Rust 네이티브다. 문법은 C이지만 `cc` 크레이트로 정적 링크되며 별도 런타임이 없다. Go 바인딩은 CGO를 요구해 "단일 정적 바이너리"의 비용이 오른다 |
| **단일 정적 바이너리** | `x86_64-unknown-linux-musl` / `aarch64-apple-darwin` 정적 링크. 사용자 머신에 런타임을 요구하지 않는다 — 설치 실패율이 곧 채택률이다 |
| **git 접근이 순수 Rust로 가능** | `gix`(gitoxide)가 libgit2 없이 blob·트리·커밋을 읽는다 |

### 기각한 것

| 대안 | 기각 이유 |
|---|---|
| **Go** | 위 첫 행이 성립하지 않는다. `Option` 부재를 컴파일러가 아니라 리뷰가 막게 된다. CGO 없이는 tree-sitter를 못 쓰고, CGO를 켜면 정적 링크가 어려워진다 |
| **TypeScript/Node** | 대상 프로젝트가 Node를 안 쓰면 런타임을 설치시켜야 한다. 10⁵ 파일 파싱의 메모리·GC 특성도 나쁘다 |
| **Python** | 위와 같고 더 느리다. 배포 형태가 특히 나쁘다 |
| **Zig / C++** | 생태계(tree-sitter 상위 바인딩·직렬화·테스트)를 직접 만들어야 한다 |

### 뒤집히는 조건

**소유자가 Rust에 미숙하다** — 이 계획의 최대 실행 위험이다([R-01](00-risks.md#r-01)). P0 단계가 언어 때문에 막히면 재론한다. 그때 바뀌는 것은 언어이지 아래 §4의 구조가 아니다. 구조는 언어 중립으로 적혀 있다.

---

## 2. 저장 — 2층이고, 2층은 **자체 인덱스**다

### 2.1 왜 2층인가

```
git (진실의 원본)
  │
  │  파일 blob 하나 → 파싱 → 파일 단위 부분 그래프
  ▼
┌─ 1층 · 추출 캐시 ────────────────────────────────────┐
│  키:  (blob_hash, extractor_version)                 │
│  값:  그 파일 하나에서 나온 심볼·엣지·미해소참조      │
│  성질: 콘텐츠 주소. 파일 내용이 같으면 항상 캐시 적중  │
└──────────────────────────────────────────────────────┘
  │
  │  질의 시점에 Snapshot 단위로 스티칭 (파일 간 참조 해소)
  ▼
┌─ 2층 · 질의 투영 ────────────────────────────────────┐
│  심볼 인덱스 · 인접 리스트 · 역방향 색인 · 결박 색인   │
│  성질: 캐시. 통째로 지워도 1층에서 재구축된다          │
└──────────────────────────────────────────────────────┘
```

**1층이 있어야 하는 이유는 성능이 아니라 계약이다.** "임의의 과거 커밋을 변경 파일 수에 비례하는 비용으로 연다"는 요구는 콘텐츠 주소 캐시 위에서만 성립한다. 브랜치를 전환해도 바뀐 파일만 다시 파싱된다. 그리고 2층을 통째로 버려도 재파싱 없이 복구된다.

이 형태에는 선례가 있다 — stack-graphs가 파일마다 독립적으로 부분 그래프를 만들고 질의 시점에 스티칭하며 변경분만 재계산한다.

### 2.2 2층을 그래프 DB에 얹지 않는 이유 — 이 문서에서 가장 중요한 결정

소유자의 방향은 그래프 DB였다. **그 방향을 다른 형태로 만족시킨다.** 정직하게 근거를 적는다.

**(1) 이 설계의 질의는 그래프 DB의 강점을 쓰지 않는다.**
실제로 필요한 연산은 넷뿐이다 — ① 좌표 키로 노드 조회 ② 인접 리스트 순회 ③ 역방향 색인 조회(`touch`) ④ **예산 절단이 있는** 제한 깊이 탐색. ①~③은 KV 위 인접 리스트가 최적이고, ④가 결정적이다.

**(2) 결정적 이유 — 절단을 기록할 수 없다.**
이 설계는 질의 실행 도중 잘라내고 **얼마나 무슨 이유로 잘랐는지를 응답에 실어야** 한다.

- 후보 집합 상한 `K=32` 초과 → 나열하지 않고 `UnresolvedRef{사유=후보 과다}`로 강등
- 경로 곱 예산 `B=10⁴` 초과 → 탐색 중단 + `Residual{사유=candidate 집합 과다}`
- 뷰 노드 상한 500 → 접되 `elision{잘린 수, 사유별 분해}`로 기록

Cypher에도 SQL에도 `LIMIT`은 있지만 **"한도에 걸린 지점의 사유별 분해를 돌려달라"** 는 표현이 없다. 남의 질의 엔진에 얹으면 이 계산이 엔진 밖으로 나와 두 번 순회하게 되거나, 더 나쁘게는 조용한 절단이 된다. 조용한 절단 금지는 이 제품의 정체성이다.

**(3) 지형 리스크가 실현됐다.**
임베디드 그래프 DB의 대표 주자였던 Kuzu는 2025년 Apple에 인수되며 저장소가 아카이브됐고, 커뮤니티가 LadybugDB·Ryu·Bighorn 등으로 갈렸다. 기본 저장 엔진을 여기 얹으면 1인 프로젝트가 통제할 수 없는 마이그레이션 위험을 진다.

**(4) 자체 구현의 최대 비용이 이 설계에서는 0이다.**
DB를 직접 만들 때 가장 비싼 것은 스키마 마이그레이션과 백업/복구다. 그런데 2층은 **캐시**다 — 포맷을 바꾸고 싶으면 지우고 다시 만들면 된다. 백업도 필요 없다(git이 원본). 자체 구현의 비용 구조가 통상과 다르다.

**(5) 그래프 DB 방향은 이렇게 만족시킨다.**
`pal export --format=cypher|graphml|parquet`. 탐색·시각화·애드혹 질의는 사용자가 원하는 그래프 DB(LadybugDB·Neo4j)로 **내보내서** 한다. 코어의 명명된 질의는 자체 인덱스로 답하고, 자유 탐색은 내보내기로 연다. 이러면 코어가 특정 제품 지형에 묶이지 않으면서 그래프 DB의 값을 잃지 않는다.

### 2.3 저장 엔진 — `redb`

| 후보 | 판정 |
|---|---|
| **`redb`** ✅ | 순수 Rust, LMDB 계열 copy-on-write B+tree, ACID, 단일 파일. 활발히 유지되고 API가 안정적. 정적 링크에 아무 문제 없음 |
| `sled` ❌ | 수년째 알파이고 재작성이 미완 |
| `fjall` ❌ | LSM 기반으로 쓰기가 빠르지만, 2026년 들어 신규 기능 개발이 사실상 정지. 우리 워크로드는 쓰기보다 읽기 지배적 |
| `rusqlite`(SQLite) ⬜ | **대조군으로만 유지.** 재귀 CTE로 같은 질의를 짜서 자체 인덱스와 답이 일치하는지 검사하는 데 쓴다. 번들 SQLite는 C 컴파일이 필요하므로 기본 경로에서는 뺀다 |
| LadybugDB / Neo4j ⬜ | 내보내기 대상(§2.2-5) |

**1층 캐시는 redb에 넣지 않는다.** 파일시스템 콘텐츠 주소 저장(`.palimpsest/cache/ab/cdef...zst`)을 쓴다 — 병렬 쓰기가 락 없이 되고, OS 페이지 캐시가 그대로 값을 내며, 부분 손상이 그 파일 하나로 격리된다. 트랜잭션이 필요한 것은 2층뿐이다.

### 2.4 뒤집히는 조건

10⁵ 파일 · 10⁷ 엣지에서 ① 3홉 역방향 탐색이 대화 흐름을 끊거나 ② 2층 재구축이 10분을 넘으면, **먼저 예산(`K`·`B`·깊이)을 의심하고, 그다음 인덱스 레이아웃을, 마지막에 엔진을 의심한다.** 순서를 못 박는다.

---

## 3. 크레이트 선택 — 실제 의존 목록

### 3.1 P0에서 쓰는 것 (여기서 늘리지 않는다)

| 용도 | 크레이트 | 왜 이것인가 / 주의 |
|---|---|---|
| git 접근 | **`gix`** | 순수 Rust. blob·트리·커밋 읽기와 HEAD 비교만 쓴다. ⚠️ API가 아직 진화 중이므로 **`pal-git` 모듈로 감싸 접촉면을 20줄 이내로 유지**한다. 깨지면 그 모듈만 고친다. 대안 `git2`(libgit2)는 C 의존이라 정적 링크 비용이 오른다 |
| 파싱 | **`tree-sitter`** + `tree-sitter-typescript` | 빌드·툴체인 없이 blob 하나로 파싱된다. 오류 회복이 있어 `partial` 상태를 표현할 수 있다 |
| 해시 | **`blake3`** | `symbol_id`·`body_digest`·캐시 키. SHA-256보다 빠르고 병렬 친화적. 트리 해시라 큰 입력에 유리 |
| 캐시 직렬화 | **`postcard`** + `serde` | 1층 캐시 값. 컴팩트한 바이너리, no_std 호환, 스키마 버전을 키에 이미 넣으므로 호환성 부담 없음. ⚠️ `rkyv`(zero-copy)는 더 빠르지만 타입 제약이 크고 초심자에게 함정이 많다 — P1 이후 성능 실측 후 재론 |
| 캐시 압축 | **`zstd`** | 파일 부분 그래프는 반복이 많아 압축률이 높다. 레벨 3 고정 |
| 2층 저장 | **`redb`** | §2.3 |
| 표면 직렬화 | **`serde_json`** | CLI/MCP 출력 |
| 병렬 | **`rayon`** | 파일 단위 추출은 완전 병렬. `par_iter`로 충분하고 async가 필요 없다 |
| CLI | **`clap`** (derive) | 표준 |
| 에러 | **`thiserror`** | 라이브러리 경계. 바이너리 최상단에서만 `anyhow` |
| 로깅 | **`tracing`** + `tracing-subscriber` | 구조화 로그. 질의 로그는 별도 장치이며 이것과 혼동하지 않는다 |

### 3.2 나중에 붙는 것

| 시점 | 크레이트 | 용도 |
|---|---|---|
| P0 후반 | `tree-sitter-kotlin` | 두 번째 언어. ⚠️ [R-03](00-risks.md#r-03) |
| P1 | `rmcp` | 공식 MCP Rust SDK. stdio 트랜스포트. 4.7M+ 다운로드, 현행 스펙 추종 |
| P1 | `pulldown-cmark` | 마크다운 인입(서술물 → 좌표) |
| P2 | `serde_yaml` / `toml` | 팩 스키마·OpenAPI 로딩 |
| P2 | `scip` 또는 protobuf 직접 파싱 | 표준 코드 인덱스 수용 |

### 3.3 테스트·검사

| 용도 | 크레이트 |
|---|---|
| 스냅샷 테스트(골든 대장) | **`insta`** |
| 속성 기반 테스트(정규화 불변식) | **`proptest`** |
| 벤치(예산 회귀) | **`criterion`** |
| 커스텀 CI 검사 | `xtask` 패턴(별도 크레이트, 외부 의존 아님) + `syn`(AST 스캔) |

### 3.4 의존 정책

- **P0에서 외부 크레이트 신규 추가는 커밋 메시지에 근거를 남긴다.** 크레이트 선택이 학습을 대체하는 것을 막는다.
- `unsafe` 금지(`#![forbid(unsafe_code)]`), `clippy::pedantic` 상시.
- 라이선스: MIT/Apache-2.0만. `cargo-deny`로 CI 검사.

---

## 4. 코드 구조 — 워크스페이스와 의존 방향

```
palimpsest/
├── Cargo.toml                      # workspace
├── crates/
│   ├── pal-core/                   # 도메인 타입·불변식·판정
│   │   │                           #   의존: std + serde + blake3 뿐. 그 외 워크스페이스 크레이트 0
│   │   ├── coord.rs                #   Coord · Site · Snapshot · SymbolId · BodyDigest
│   │   ├── provenance.rs           #   출처 4값 · 배정 규칙 · 승격
│   │   ├── ledger.rs               #   대장 · 파일 상태 7값 · 언어 등급
│   │   ├── graph.rs                #   노드 · 엣지 · 해소 등급 · UnresolvedRef
│   │   ├── binding.rs              #   결박 5상태 · 반경 · 감시 집합
│   │   ├── judgment.rs             #   Finding · Residual · OutOfScope · ScopeReduction
│   │   ├── narrative.rs            #   Synthesis · Narration · ViewModel · Elision
│   │   ├── intent.rs               #   Plan · Deviation · Briefing
│   │   └── envelope.rs             #   Envelope<T> · Coverage · ProjectionFreshness
│   ├── pal-git/                    # gix 격리. 접촉면 최소. 의존: pal-core
│   ├── pal-extract/                # tree-sitter 추출기 · 언어 등급 · 정규화. 의존: pal-core
│   ├── pal-store/                  # 1층 캐시 + 2층 인덱스 + 스티칭. 의존: pal-core
│   ├── pal-query/                  # 명명된 질의 · 실행기 · 예산 절단 · 질의 로그
│   │                               #   의존: pal-core, pal-store
│   ├── pal-intake/                 # 관측 수용 API. 의존: pal-core, pal-store
│   ├── pal-cli/                    # JSON in/out. 의존: 위 전부
│   └── pal-mcp/                    # MCP 서버. 의존: 위 전부
├── xtask/                          # CI 검사 구현
├── surface/queries.toml            # 명명된 질의 카탈로그 — 단일 진실
├── packs/schema/                   # 선언 팩 스키마
├── corpus/                         # 평가 코퍼스·과제·성공 기준
└── docs/                           # 백서·설계·근거·이 계획
```

### 4.1 의존 방향 규칙 — CI가 기계로 검사한다

| 규칙 | 무엇을 막는가 |
|---|---|
| `pal-core`는 워크스페이스 내 **어떤** 크레이트에도 의존하지 않는다 | 도메인이 저장·전송·호스트 개념을 내부화하는 것 |
| 어떤 크레이트도 `pal-cli`·`pal-mcp`에 의존하지 않는다 | 소비자 어휘의 역류 |
| `pal-core`는 `tree-sitter`·`redb`·`gix`에 의존하지 않는다 | 파서·저장 기술이 좌표계에 새는 것 |
| `pal-intake`는 `pal-extract`에 의존하지 않는다 | `observed`가 `extracted` 경로를 재사용해 출처 배정이 흐려지는 것 |

### 4.2 어휘 금지 목록 — `pal-core` 소스에 나타나면 CI 실패

- **호스트**: `claude`, `mcp`, `tool_call`, `session`, `prompt` (예외: `produced_by.prompt_hash`)
- **거버넌스**: `gate`, `risk_level`, `block`, `approve_and_merge`, `completion`, `change_contract`
- **저장 기술**: `cypher`, `sql`, `redb`, `table`, `node_label`

허용 목록은 `xtask/vocab.toml`. **허용 목록에 줄이 느는 것 자체가 관측 대상이다.**

---

## 5. 핵심 타입 — 실제 시그니처

```rust
// ── 좌표 ──────────────────────────────────────────────────────────
// span 은 Coord 에 없다. 라인이 필요한 자리는 Site.
pub struct Coord {
    repo: RepoId,
    commit: CommitSha,
    extractor: ExtractorVersion,   // (코어 버전, 팩 지문)
    symbol: SymbolId,
}
pub struct Site { coord: Coord, span: Span }          // span 필수

// 정체성 등급이 타입으로 구별된다 → L0 언어에서 결박하는 코드는 컴파일되지 않는다
pub enum SymbolIdentity { Exact(SymbolId), Ordinal(SymbolId), Unavailable }

// ── 모든 질의의 반환 타입 ──────────────────────────────────────────
// 답만 돌려주는 경로가 타입 수준에 존재하지 않는다.
pub struct Envelope<T> {
    answer: T,
    snapshot: Snapshot,
    projection: ProjectionFreshness,  // 워킹트리와 일치하는가
    coverage: Coverage,               // 미해소 N · 범위 밖 N · 경유 언어 등급
    ledger: LedgerRef,
    elision: Elision,                 // 예산에 걸려 잘린 것 — 없어도 명시적으로 none()
}

// ── 판정: clean 이 없다 ────────────────────────────────────────────
pub enum Judgment { Finding(Finding), Residual(Residual), OutOfScope(OutOfScope) }
//        ^ 넷째 변형이 없고 #[non_exhaustive] 도 아니다

// ── 출처: 불변. 승격은 새 노드 ─────────────────────────────────────
pub enum Provenance { Extracted, Observed(ObsMeta), Asserted(AssertMeta), Inferred(InferMeta) }
impl Inferred {
    // self 를 소비하지 않는다 — 원본이 promoted_by 와 함께 남아야 한다
    pub fn promote(&self, approval: Approval) -> (AssertedNode, PromotedByEdge);
}

// ── 저장 포트 ──────────────────────────────────────────────────────
pub trait ExtractCache {                       // 1층
    fn get(&self, key: BlobKey) -> Result<Option<FileGraph>>;
    fn put(&self, key: BlobKey, g: &FileGraph) -> Result<()>;
}
pub trait Projection {                         // 2층
    fn symbol(&self, id: SymbolId) -> Result<Option<SymbolNode>>;
    fn out_edges(&self, id: SymbolId) -> Result<EdgeCursor>;
    fn in_edges(&self, id: SymbolId) -> Result<EdgeCursor>;   // touch 의 근간
    fn bound_to(&self, id: SymbolId) -> Result<BindingCursor>;
    fn rebuild_from(&self, cache: &dyn ExtractCache, snap: &Snapshot) -> Result<RebuildReport>;
}
```

**타입으로 강제하는 다섯**

| 규칙 | 구현 | 위반 시점 |
|---|---|---|
| `evidence_refs`가 비면 저장 불가 | `Synthesis::new(body, NonEmpty<Coord>, ..)` | 컴파일 |
| 선택 필드 금지 | 도메인 타입에 `Option<T>` 금지. `xtask lint-schema`가 `syn`으로 AST 검출 | CI |
| 출처 불변 | setter 없음. 승격은 새 노드 반환 | 컴파일 |
| `clean` 없음 | `enum Judgment` 3변형 | 컴파일 |
| 조용한 절단 금지 | `Envelope` 생성에 `Elision` 필수, 없으면 `Elision::none()`을 명시 | 컴파일 |

---

## 6. 표면 — CLI와 MCP는 같은 카탈로그에서 파생된다

`surface/queries.toml` 하나가 단일 진실이고, 여기서 ① Rust 질의 enum ② JSON 스키마 ③ CLI 서브커맨드 ④ MCP 툴 정의 ⑤ 문서 표가 전부 생성된다. **코드에만 있고 카탈로그에 없는 질의는 CI 실패.**

```toml
[query.touch]
summary   = "좌표에 결박된 전부"
args      = [{ name = "coord", type = "CoordRef" }]
returns   = "TouchResult"
introduced = "F11"
direction = "consume"          # observation.intake 만 "provide"
```

---

## 7. 플랫폼·빌드

| 항목 | 값 |
|---|---|
| MSRV | 최신 stable − 2 (edition 2024 요구) |
| 타깃 | `aarch64-apple-darwin`(개발) · `x86_64-unknown-linux-musl`(배포) |
| 바이너리 | `pal`(CLI) · `pal-mcp`(MCP 서버). 실제로는 하나의 바이너리 + 서브커맨드 |
| 설치 데이터 위치 | 대상 저장소의 `.palimpsest/` (gitignore 권장, 커밋해도 무해) |
| 릴리스 | **P2까지 릴리스 아티팩트를 만들지 않는다.** 자기 저장소와 코퍼스에서만 돈다 |
