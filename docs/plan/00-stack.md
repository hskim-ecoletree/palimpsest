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
| **단일 정적 바이너리** | 사용자 머신에 런타임을 요구하지 않는다 — 설치 실패율이 곧 채택률이다. 릴리스가 실제로 내는 타깃은 **넷**이다 — `x86_64-pc-windows-msvc` · `x86_64-unknown-linux-gnu` · `aarch64-apple-darwin` · `x86_64-apple-darwin`(`.github/workflows/release.yml:57-72` 의 matrix). ⚠ **리눅스는 `musl` 이 아니라 `gnu` 다** — `zstd-sys` 와 tree-sitter 문법들이 C 컴파일러를 타서 `musl-gcc` 축이 하나 더 늘기 때문이고, 그래서 리눅스 바이너리는 glibc 를 탄다(같은 워크플로 `:23` 이 그 까닭을 적는다) |
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

## 2. 저장 — **파생 2층 + 원본 1층**. 재생 가능한 것과 아닌 것을 물리적으로 가른다

### 2.1 세 저장소

```
git (코드의 진실 원본)                    사람의 승인 (의도의 진실 원본)
  │                                              │
  │  blob 하나 → 파싱 → 파일 단위 부분 그래프     │
  ▼                                              ▼
┌─ 1층 · 추출 캐시 ────────────────────┐   ┌─ 의도 저장소 ─────────────────────┐
│  키:  (blob_hash, extractor_version) │   │  결박 · asserted 라벨 · 관측 원문  │
│  값:  그 파일 하나에서 나온 부분 그래프│   │  승인/거부 이력 · 별칭 테이블      │
│  성질: 콘텐츠 주소. 재생 가능          │   │  성질: **재생 불가. 원본.**        │
└──────────────────────────────────────┘   │  파일: intent.redb + 내보내기 JSONL│
  │                                        └───────────────────────────────────┘
  │  질의 시점 스티칭 (파일 간 참조 해소)       │
  ▼                                            │ 읽기 전용 참조
┌─ 2층 · 질의 투영 ────────────────────────────┴───────┐
│  심볼 인덱스 · 인접 리스트 · 역방향 색인 · 결박 **색인**│
│  성질: 캐시. 통째로 지워도 1층 + 의도 저장소에서 재구축 │
└──────────────────────────────────────────────────────┘
```

**1층이 있어야 하는 이유는 성능이 아니라 계약이다.** "임의의 과거 커밋을 변경 파일 수에 비례하는 비용으로 연다"는 요구는 콘텐츠 주소 캐시 위에서만 성립한다. 브랜치를 전환해도 바뀐 파일만 다시 파싱된다.

이 형태에는 선례가 있다 — stack-graphs가 파일마다 독립적으로 부분 그래프를 만들고 질의 시점에 스티칭하며 변경분만 재계산한다.

### 2.2 의도 저장소를 가르는 이유 — **"2층은 캐시"가 참이려면 필요하다**

2층에 결박·승인·관측이 **살면** "지우고 재구축"은 사람의 노동을 지우는 명령이 된다. R-10이 이 프로젝트에서 가장 비싼 것이라 부른 바로 그 노동이다. 그리고 재구축 등가성 검사는 그 상태에서도 **통과하므로**, 검사가 유실을 정상으로 승인하게 된다([R-21](00-risks.md#r-21)).

| | 1층 | 2층 | 의도 저장소 |
|---|---|---|---|
| 원본은 어디 | git | 1층 + 의도 저장소 | **자기 자신** |
| 지워도 되나 | 예 (재파싱) | 예 (재구축) | **아니오 — 유실** |
| 백업 대상 | 아니오 | 아니오 | **예. 그리고 텍스트로 내보내진다** |
| 담는 출처 | `extracted` | 전부의 **색인** | `asserted` · `observed` · `inferred` |

- 2층은 의도 저장소의 **색인**만 갖는다(`BOUND_BY` · `WATCH` 등). 실체는 의도 저장소에 있고 2층은 그것을 가리킨다.
- `pal cache prune` · `pal reindex`는 의도 저장소를 **건드리지 않는다.** 지우는 경로가 존재하지 않는 것이 대응이고, 그것을 CI가 검사한다.

#### 정본은 텍스트다 — 2026-08-10 정정 ([옛 F23](disposal-map.md) · [옛 DESIGN §12.8](disposal-map.md))

초안은 `intent.redb`를 원본으로 두고 JSONL을 **내보내기**로 두었다. 방향을 뒤집는다.

| | 초안 | 지금 |
|---|---|---|
| 원본 | `intent.redb` | **`.palimpsest/intent/*.jsonl`** (git에 커밋) |
| 파생 | JSONL 내보내기 | `intent.redb` (색인. 지워도 재구축) |

**뒤집는 이유 셋** — ① [R-21](00-risks.md#r-21)의 남은 구멍(*"사용자가 `.palimpsest/`를 통째로 지우는 것은 막을 수 없다"*)이 닫힌다. clone만으로 돌아온다. ② **승인 노동이 팀에 공유된다** — 지금은 한 사람의 로컬 파일이다. ③ 결박이 **변경과 같은 리뷰를 받는다.**

**성립 조건은 머지다** — 추가 전용 · 한 줄 한 레코드 · 결정론적 정렬 · 파일 분할. 바이너리를 정본으로 두면 이 넷 중 어느 것도 성립하지 않는다.

### 2.3 2층을 그래프 DB에 얹지 않는 이유 — 이 문서에서 가장 중요한 결정

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

**(4) 자체 구현의 최대 비용이 2층에서는 0이다 — 다만 의도 저장소에서는 아니다.**
DB를 직접 만들 때 가장 비싼 것은 스키마 마이그레이션과 백업/복구다. **2층에 한해** 그 비용이 0이다 — 포맷을 바꾸고 싶으면 지우고 다시 만들면 되고, 원본은 git과 의도 저장소에 있다.

**그러나 의도 저장소는 이 면제를 받지 못한다**(§2.2). 거기에는 마이그레이션도 백업도 필요하다. 그래서 그쪽만 ① 스키마 버전을 레코드에 싣고 ② JSONL 내보내기를 상시 유지한다 — **마이그레이션의 최후 수단이 내보내고 다시 읽는 것**이 되게. 이 비용은 2층이 아니라 여기에 지불된다.

**(5) 그래프 DB 방향은 이렇게 만족시킨다.**
`pal export --format=cypher|graphml|parquet`. 탐색·시각화·애드혹 질의는 사용자가 원하는 그래프 DB(LadybugDB·Neo4j)로 **내보내서** 한다. 코어의 명명된 질의는 자체 인덱스로 답하고, 자유 탐색은 내보내기로 연다. 이러면 코어가 특정 제품 지형에 묶이지 않으면서 그래프 DB의 값을 잃지 않는다.

### 2.4 저장 엔진 — `redb`

| 후보 | 판정 |
|---|---|
| **`redb`** ✅ | 순수 Rust, LMDB 계열 copy-on-write B+tree, ACID, 단일 파일. 활발히 유지되고 API가 안정적. 정적 링크에 아무 문제 없음 |
| `sled` ❌ | 수년째 알파이고 재작성이 미완 |
| `fjall` ❌ | LSM 기반으로 쓰기가 빠르지만, 2026년 들어 신규 기능 개발이 사실상 정지. 우리 워크로드는 쓰기보다 읽기 지배적 |
| `rusqlite`(SQLite) ⬜ | **대조군으로만 유지.** 재귀 CTE로 같은 질의를 짜서 자체 인덱스와 답이 일치하는지 검사하는 데 쓴다. 번들 SQLite는 C 컴파일이 필요하므로 기본 경로에서는 뺀다 |
| LadybugDB / Neo4j ⬜ | 내보내기 대상(§2.3-5) |

**1층 캐시는 redb에 넣지 않는다.** 파일시스템 콘텐츠 주소 저장(`.palimpsest/cache/ab/cdef...zst`)을 쓴다 — 병렬 쓰기가 락 없이 되고, OS 페이지 캐시가 그대로 값을 내며, 부분 손상이 그 파일 하나로 격리된다.

**2층과 의도 저장소는 redb를 쓰되 파일이 다르다** — `index.redb` / `intent.redb`. 같은 파일에 두면 "2층을 지운다"가 실수 하나로 의도를 지우게 된다. **파일이 갈린 것 자체가 대응이다.**

### 2.5 뒤집히는 조건

10⁵ 파일 · 10⁷ 엣지에서 ① 3홉 역방향 탐색이 대화 흐름을 끊거나 ② 2층 재구축이 10분을 넘으면, **먼저 예산(`K`·`B`·깊이)을 의심하고, 그다음 인덱스 레이아웃을, 마지막에 엔진을 의심한다.** 순서를 못 박는다.

---

## 3. 크레이트 선택 — 실제 의존 목록

### 3.1 P0에서 쓰는 것 (여기서 늘리지 않는다)

| 용도 | 크레이트 | 왜 이것인가 / 주의 |
|---|---|---|
| git 접근 | **`gix`** | 순수 Rust. blob·트리·커밋 읽기와 HEAD 비교만 쓴다. ⚠️ API가 아직 진화 중이므로 **`pal-git` 모듈로 감싸 접촉면을 20줄 이내로 유지**한다. 깨지면 그 모듈만 고친다. 대안 `git2`(libgit2)는 C 의존이라 정적 링크 비용이 오른다 |
| 파싱 | **`tree-sitter`** + 문법 넷 — `kotlin` · `java` · `javascript` · `typescript` | 빌드·툴체인 없이 blob 하나로 파싱된다. 오류 회복이 있어 `partial` 상태를 표현할 수 있다. **넷이 전부 1급이다
> ↳ **2026-08-20 이 Rust 를 더해 다섯이 됐다**([ADR-0027](../adr/0027-the-instrument-must-reach-its-own-floor.md)).
> ⚠ **1급 언어와 붙인 문법은 다른 수다** — 1급은 다섯, `Cargo.toml` 의 문법 의존은
> **셋**(kotlin·typescript·rust)이다. Java·JavaScript 는 선언만 있고 `NotBuilt` 다.**([지시 2026-08-12 §1](../instructions/2026-08-12-owner-direction.md)) |
| 해시 | **`blake3`** | `symbol_id`·`body_digest`·캐시 키. SHA-256보다 빠르고 병렬 친화적. 트리 해시라 큰 입력에 유리 |
| 캐시 직렬화 | **`postcard`** + `serde` | 1층 캐시 값. 컴팩트한 바이너리, no_std 호환, 스키마 버전을 키에 이미 넣으므로 호환성 부담 없음. ⚠️ `rkyv`(zero-copy)는 더 빠르지만 타입 제약이 크고 초심자에게 함정이 많다 — P1 이후 성능 실측 후 재론 |
| 캐시 압축 | **`zstd`** | 파일 부분 그래프는 반복이 많아 압축률이 높다. 레벨 3 고정 |
| 2층 저장 | **`redb`** | §2.3 |
| 표면 직렬화 | **`serde_json`** | CLI 출력 |
| 병렬 | **`rayon`** | 파일 단위 추출은 완전 병렬. `par_iter`로 충분하고 async가 필요 없다 |
| CLI | **`clap`** (derive) | 표준 |
| 에러 | **`thiserror`** | 라이브러리 경계. 바이너리 최상단에서만 `anyhow` |
| 로깅 | **`tracing`** + `tracing-subscriber` | 구조화 로그. 질의 로그는 별도 장치이며 이것과 혼동하지 않는다 |
| **스키마 읽기** | **`toml`** | **2026-08-12 · F22 가 추가.** 아래 참조 |

#### `toml` 이 목록에 늘었다 — 그 근거 (2026-08-12, F22)

이 절의 제목이 *"여기서 늘리지 않는다"* 이므로 늘어난 것은 근거와 함께 본문에 적는다
([§3.4](#34-의존-정책): *"P0에서 외부 크레이트 신규 추가는 커밋 메시지에 근거를 남긴다"*).

`schema/graph.toml` 이 노드·엣지의 **단일 진실**이고([옛 DESIGN §1.2](disposal-map.md) D25),
[옛 DESIGN §3.4](disposal-map.md)는 `producer`↔`provenance` 정합이 *"로딩 시점에 거부된다"* 고,
[옛 DESIGN §12.7](disposal-map.md)은 `doctor` 의 불변식 여덟이 *"`schema/graph.toml` 에서 파생되며 손으로
세지 않는다"* 고 적었다. 즉 **이 파일은 실행 시점에 읽혀야 한다.**

`xtask` 가 `vocab.toml` 을 파서 없이 읽은 것(*"이 한 줄을 읽자고 의존을 늘리지 않는다"*)과
다른 판단이고 다른 이유는 둘이다: 대상이 한 줄이 아니라 스키마 전체이고, **읽는 경로가
하나여야 로딩 시점 거부가 성립한다.** 그래서 `pal-core::schema` 하나가 읽고 `xtask` 는
자기 파서를 들지 않고 그것을 부른다 — 검사가 자기 파서를 들면 **CI 를 통과한 스키마가
실행 시점에 거부될 수 있다.**

`pal-core` 의 의존이 `serde`·`blake3` 둘에서 셋이 됐다. [§4.1](#41-의존-방향-규칙--ci가-기계로-검사한다)의
금지 목록(`tree-sitter`·`redb`·`gix`)에는 걸리지 않는다 — 파서·저장 기술이 아니라
**설정 형식**이고, `serde` 가 이미 같은 자격으로 거기 있다.

#### 언어 넷이 같은 층에 선다 — 착수 순서만 다르다 (2026-08-12)

초안은 P0 을 TypeScript 하나로 잡고 Kotlin 을 §3.2 의 *"P0 후반 · 두 번째 언어"* 로 미뤘다.
**적용 대상이 정해지면서 그 순서가 무효화됐다** — boxwood 는 언어가 저장소별로 갈려 있어,
한 언어만 뚫으면 도구가 닿는 저장소가 하나뿐이다(저장소별 실측은 [지시 §1](../instructions/2026-08-12-owner-direction.md)).

| | 착수 | 왜 |
|---|---|---|
| **Kotlin** | **S0** | 유일하게 **사전 등록된 대조값**이 있다 — [T7](../gates/preflight.md#t7--kotlin-파싱-사전-측정)의 선언 추출 94.30% |
| Java · JavaScript · TypeScript | F02 | **대조값이 없다.** 아무도 재본 적 없으므로 지금 숫자를 적지 않는다(옛 `README §7.5`) |

**우선순위가 아니라 측정 가능성이 순서를 정했다.** 넷은 같은 층에 선다.

**미구축 언어의 자리는 `Capable<T>` 다**(§5.3). 빈 목록을 내면 *"선언이 없는 파일"*과
*"이 빌드가 그 언어를 모른다"*가 같은 출력이 된다 — 그래서 S0 부터 이 타입이 하중을 진다.

**`.svelte` 는 다섯째 언어가 아니라 추출기 구조의 문제다.** `frontend` 의 634개가 Svelte-5 이고,
`<script>` 안의 js/ts 를 꺼내려면 **injection**(바깥 문법으로 파싱한 영역을 다른 문법에 넘기는 것)이
필요하다. 좌표·`body_digest`·스코프 체인이 전부 그 경계를 넘어야 한다. **소유 기능 미배정.**

### 3.2 나중에 붙는 것

| 시점 | 크레이트 | 용도 |
|---|---|---|
| ~~P1~~ | ~~`rmcp`~~ | ★ **2026-08-18 에 뺐다** — [ADR-0025](../adr/0025-the-harness-that-reads-the-graph-is-the-same-product.md) 가 호스트 중립을 초석에서 내렸고, *"MCP 는 호스트 중립성을 사는 값이고 호스트가 하나면 살 이유가 없다"* |
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
  - **예외 셋이 실제로 생겼다**(2026-08-12, S1). `Unicode-3.0`(`unicode-ident` — `AND` 조항이라 피할 수 없다) · `Zlib`(`foldhash` ← gix-pack) · `BSD-2-Clause`(`arrayref` ← **blake3**). 셋 다 permissive이고 copyleft가 아니다. **예외를 늘리기 전에 의존을 줄였다** — gix를 `features = ["sha1", "revision"]`로 좁혀 `uluru`(MPL-2.0 · copyleft)와 `encoding_rs`(BSD-3)를 트리에서 뺐다. 근거는 [`deny.toml`](../../deny.toml)의 예외 주석과 [S1 게이트](../gates/S1-ledger.md)에 있다.

---

## 4. 코드 구조 — 워크스페이스와 의존 방향

```
palimpsest/
├── Cargo.toml                      # workspace
├── crates/
│   ├── pal-core/                   # 도메인 타입·불변식·판정
│   │   │                           #   의존: std + serde + blake3 뿐. 그 외 워크스페이스 크레이트 0
│   │   ├── coord.rs                #   Coord · Site · Snapshot · SymbolId · BodyDigest
│   │   ├── ledger.rs               #   대장 · 파일 상태 7값 · 언어 등급
│   │   ├── graph.rs                #   노드 · 엣지 · 해소 등급 · UnresolvedRef
│   │   ├── binding.rs              #   결박 5상태 · 반경 · 감시 집합
│   │   ├── judgment.rs             #   Finding · Residual · OutOfScope · ScopeReduction
│   │   ├── narrative.rs            #   Synthesis · Narration · ViewModel · Elision
│   │   └── envelope.rs             #   Envelope<T> · Coverage · ProjectionFreshness
│   ├── pal-git/                    # gix 격리. 접촉면 최소. 의존: pal-core
│   ├── pal-extract/                # tree-sitter 추출기 · 언어 등급 · 정규화. 의존: pal-core
│   ├── pal-store/                  # 1층 캐시 + 2층 인덱스 + 스티칭. 의존: pal-core
│   │                               #   §2.1의 파생 두 층만. 지우는 API를 가진 유일한 크레이트
│   ├── pal-intent/                 # 의도 저장소 — 결박·승인·관측 원본 + JSONL 내보내기
│   │                               #   의존: pal-core. **지우는 API가 없다**(§2.2)
│   ├── pal-query/                  # 명명된 질의 · 실행기 · 예산 절단 · 질의 로그
│   │                               #   의존: pal-core, pal-store
│   ├── pal-cli/                    # JSON in/out. 의존: 위 전부. **1급 표면**
├── xtask/                          # CI 검사 구현
├── schema/
│   ├── graph.toml                  # 노드·엣지·속성·producer — 단일 진실 (F22)
├── surface/queries.toml            # 명명된 질의 카탈로그 — 단일 진실
├── corpus/                         # 평가 코퍼스·과제·성공 기준
└── docs/                           # ADR·게이트·이 계획 (백서·설계는 2026-08-18 에 지웠다)
```

**단일 진실 파일은 둘이다** — `schema/graph.toml`(무엇이 존재하는가) · `surface/queries.toml`(무엇을 물을 수 있는가). **둘 다 코드가 아니라 데이터이고, 코드는 거기서 파생되거나 거기에 대조된다.**

⚠ **셋째(`schema/provider.toml`)는 안 만들었다.** 옛 `F21 provider` 포트의 것이고 그 기능은 2026-08-18 에 처분됐다([처분표](disposal-map.md)).

### 4.1 의존 방향 규칙 — CI가 기계로 검사한다

| 규칙 | 무엇을 막는가 |
|---|---|
| `pal-core`는 워크스페이스 내 **어떤** 크레이트에도 의존하지 않는다 | 도메인이 저장·전송·호스트 개념을 내부화하는 것 |
| 어떤 크레이트도 `pal-cli`에 의존하지 않는다 | 소비자 어휘의 역류 |
| `pal-core`는 `tree-sitter`·`redb`·`gix`에 의존하지 않는다 | 파서·저장 기술이 좌표계에 새는 것 |
| **`pal-store`는 `pal-intent`에 쓰기 의존하지 않는다** (읽기만) | 캐시 폐기 경로가 의도 저장소에 닿는 것 — [R-21](00-risks.md#r-21) |

### 4.2 어휘 금지 목록 — `pal-core` 소스에 나타나면 CI 실패

- **호스트**: `claude`, `mcp`, `tool_call`, `session`, `prompt` (예외: `produced_by.prompt_hash`)
- **거버넌스**: `gate`, `risk_level`, `block`, `approve_and_merge`, `completion`, `change_contract`
- **저장 기술**: `cypher`, `sql`, `redb`, `table`, `node_label`

허용 목록은 `xtask/vocab.toml`. **허용 목록에 줄이 느는 것 자체가 관측 대상이다.**

### 4.3 검사를 언제 켜는가 — **전부 첫 커밋에 켜지 않는다**

"F01 첫 커밋에서 전부 켠다"는 실행 불가능하다. ① 검사 대상(`Envelope`·카탈로그·2층)이 그 시점에 존재하지 않고 ② `syn` 기반 AST 린터 자체가 M 규모 작업이며 ③ [R-01](00-risks.md#r-01)이 사실이면 초심자의 첫 작업이 Rust 린터가 된다. **그러면 검사를 켜는 대신 검사를 미루게 된다** — 가장 나쁜 결과다.

| 단계 | 검사 | 켜는 시점 | 구현 비용 |
|---|---|---|---|
| **1 — 첫 커밋** | 의존 방향 (`cargo metadata` 파싱) | F01 | S |
| | 어휘 금지 (`ripgrep` + `vocab.toml`) | F01 | S |
| | `#![forbid(unsafe_code)]` · `clippy::pedantic` · `cargo-deny` | F01 | S |
| | **의도 저장소 폐기 경로 부재** (`pal-store`에서 `intent` 경로 삭제 호출 없음 — 문자열 스캔) | F01 | S |
| **2 — 대상이 생길 때** | 선택 필드 금지 (`pal-core` `pub struct` 문자열 스캔 → 이후 `syn`) | F03 | M |
| | 재구축 등가성 (1층+의도 → 2층) | F05 | M |
| | `Envelope` 누락 불가 | F05 | M |
| | 표면 카탈로그 동기 | F06 | M |
| | 호스트 없는 코어 · 관측 0건 답변 | F06 | S |
| | 예산 회귀 벤치 | F05 | M |

**단계 2의 각 검사는 그 기능의 규모에 포함된다.** 어디에도 계상되지 않은 검사는 만들어지지 않는다.

---

## 5. 핵심 타입 — 실제 시그니처

### 5.1 버전 축 셋 — **하나로 합치면 캐시가 상시 전량 무효화된다**

버전이 하나면 규칙 한 줄을 고쳐도 10⁵ 파일이 재파싱된다. 무엇이 무엇을 무효화하는지가 다르므로 축을 가른다.

| 축 | 성분 | 바뀌면 무효화되는 것 | 좌표에 실리나 |
|---|---|---|---|
| `ExtractorVersion` | tree-sitter 문법 버전 + 추출기 코드 버전 | **1층 캐시 전량** · 2층 · 좌표 이동 | **예** (`Coord`의 성분) |
| `ProjectionVersion` | 해소 로직 버전 + 정규화 등급 | 2층만 (재파싱 없음) | 아니오 — `Envelope`에 실린다 |
| `PackFingerprint` | 팩 규칙·인식기·개념 선언의 해시 | 팩이 파생시킨 **라벨·판정·개념**만 | 아니오 — 라벨/판정에 실린다 |

**팩 지문은 `Coord`에 들어가지 않는다.** 팩은 심볼의 존재나 정체성을 바꾸지 않고 그것에 **붙는 것**을 바꾼다. 팩이 자라도 좌표는 제자리이고, 움직이는 것은 파생 라벨의 결박 상태다(규칙 좌표에 결박되어 `stale`이 되는 것이 이미 F18의 장치다).

**예외 하나**: 팩이 언어 문법이나 심볼 추출 규칙 자체를 바꾸는 경우. 그런 팩은 스키마에서 `affects = "extraction"`으로 분류되고 **그때만** `ExtractorVersion`에 합성된다. 분류가 없는 팩은 추출에 닿을 수 없다 — 로딩 시점에 거부한다.

### 5.2 타입

```rust
// ── 좌표 ──────────────────────────────────────────────────────────
// span 은 Coord 에 없다. 라인이 필요한 자리는 Site.
pub struct Coord {
    repo: RepoId,
    tree: TreeRef,                 // 커밋 | 워킹트리 (R-06)
    extractor: ExtractorVersion,   // (문법 버전, 추출기 코드 버전) — 팩 지문은 여기 없다
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
    capabilities: CapabilitySet,      // 이 빌드가 실제로 산출할 수 있는 것 (§5.3)
    ledger: LedgerRef,
    elision: Elision,                 // 예산에 걸려 잘린 것 — 없어도 명시적으로 none()
}

// ── 아직 만들지 않은 능력: "없음"과 구별된다 ─────────────────────────
// Option<T> 가 아니다. None 은 "값이 없다"이고 우리에게 필요한 것은 "이 빌드가 답하지 않는다"이다.
pub enum Capable<T> {
    Present(T),
    NotBuilt { capability: CapabilityId },   // 그 능력의 기능 번호가 실린다
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

### 5.3 `Capable<T>` — 점진 구축이 정직함을 깨지 않게

**이것이 없으면 P1에서 곧바로 규칙이 무너진다.** `TouchResult`(F11, P1)는 판정 요약(F15, P2)·효과(F13, P2)·범위 축소(F20, P3)를 자리로 갖는다. 아직 안 만든 것을 어떻게 표현하나?

| 후보 | 왜 안 되나 |
|---|---|
| `Option<T>` | 선택 필드 금지 위반. 그리고 `None`이 "없음"인지 "안 만듦"인지 구별 안 됨 |
| 빈 `Vec` | **거짓 안전.** `Finding 0`과 "감사를 안 만들었음"이 같은 출력이 된다 — [목표 §3.1](00-goals.md#31-모르는-것을-안다고-하지-않는다)의 정면 위반 |
| 필드를 나중에 추가 | 표면 스키마가 매 기능마다 깨진다. 소비자가 따라올 수 없다 |

`Capable<T>`가 답이다. **자리는 처음부터 있고, 값은 `NotBuilt{capability}`다.** 출력에는 이렇게 나온다:

```
■ 판정
  (이 빌드에는 감사 능력이 없습니다 — F15 미구축)
```

"`Finding 0`"이라고 말하지 않는다. 그리고 `Envelope.capabilities`가 응답마다 그 목록을 실으므로 **소비자가 능력 유무를 질의 없이 안다.**

### 5.4 타입으로 강제하는 **일곱**

| 규칙 | 구현 | 위반 시점 |
|---|---|---|
| `evidence_refs`가 비면 저장 불가 | `Synthesis::new(body, NonEmpty<Coord>, ..)` | 컴파일 |
| 선택 필드 금지 | **도메인 타입**에 `Option<T>` 금지(범위는 아래) | CI |
| **능력 부재를 값으로** | 미구축 산출의 자리는 `Capable<T>`. 빈 컬렉션으로 대신하지 않는다 | CI + 리뷰 |
| 출처 불변 | setter 없음. 승격은 새 노드 반환 | 컴파일 |
| **속성 출처 동질성** | 한 노드의 모든 속성이 같은 출처. 스키마의 `producer`가 노드 `provenance`와 정합([옛 DESIGN §3.4](disposal-map.md)) | 스키마 로딩 + CI |
| `clean` 없음 | `enum Judgment` 3변형 | 컴파일 |
| 조용한 절단 금지 | `Envelope` 생성에 `Elision` 필수, 없으면 `Elision::none()`을 명시 | 컴파일 |

**"도메인 타입"의 정의 — 이것이 없으면 CI 검사를 짤 수 없다.**

> `pal-core`에서 `pub`이고, `serde::Serialize`를 구현하며, 표면 응답에 실리는 타입. 즉 **사용자가 값을 보게 되는 타입**.

| `Option`이 금지되는 곳 | 허용되는 곳 |
|---|---|
| `Coord` · `Ledger` · `Binding` · `Judgment` · `Envelope` 등 응답에 실리는 전부 | 저장 포트 트레잇의 반환값(`fn symbol(..) -> Result<Option<SymbolNode>>` — "그 키가 없다"는 조회 결과이지 도메인 값이 아니다) |
| 결박·판정·대장의 필드 | 구현 내부 자료구조(`Scope{ parent: Option<ScopeIx> }` 같은 것 — `pub`이 아니고 직렬화되지 않는다) |

**검사는 단계적으로 켠다**(§4.3) — 1단계는 `pal-core`의 `pub struct` 필드에 대한 문자열 스캔, 2단계에서 `syn` AST로 승급.

### 5.5 예산 상수는 한 곳에 있고, 초기값은 **자리표시**다

`K=32` · `B=10⁴` · 깊이 3 · 뷰 500 · 캐시 2GB · 파일당 2KB · 오버사이즈 2MB · ERROR 30% · 폐포 10⁶ · 관측 M=10 · `touch` 상위 10 · 커버리지 임계 50% · p95 500ms.

**이 숫자들 중 실측에서 나온 것은 하나도 없다.** 그런데 여러 문서에 흩어져 박혀 있고, README §3의 "예산 회귀" 검사가 그것을 굳히는 압력이 된다.

| 규칙 | |
|---|---|
| **단일 위치** | 전부 `pal-core::budget`의 상수. 다른 곳에 리터럴로 나타나면 CI 실패 |
| **초기값은 자리표시** | 각 상수에 "어느 기능의 어느 측정이 확정하는가"를 주석으로 단다. 확정 전 값은 `PROVISIONAL_` 접두어 |
| **예산 회귀 검사의 뜻** | "값이 안 변했다"가 아니라 **"값이 바뀌면 벤치 결과와 함께 커밋된다"**. 변경 자체를 막지 않는다 |
| **응답에 실린다** | 사용된 예산이 `Elision.limits_hit`에 이름과 값으로 남는다 — 사용자가 어느 상한에 걸렸는지 안다 |

---

## 6. 표면 — 목록은 언제나 카탈로그에서 파생된다

`surface/queries.toml` 하나가 단일 진실이고, 여기서 ① Rust 질의 enum ② JSON 스키마 ③ CLI 서브커맨드 ④ 문서 표가 전부 생성된다. **코드에만 있고 카탈로그에 없는 질의는 CI 실패.**

★ 옛 판은 여기에 **MCP 툴 정의**가 있었다. 어댑터는 2026-08-18 에 지워졌지만 **규칙은 남는다** — *"표면을 더할 때 목록을 더하지 않는다"* ([ADR-0024](../adr/0024-an-adapter-that-can-diverge-is-a-second-core.md)). 스킬·에이전트 정의·훅에도 그대로 걸린다.

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
| 타깃 | 개발 `aarch64-apple-darwin` · **배포는 넷** — `x86_64-pc-windows-msvc` · `x86_64-unknown-linux-gnu` · `aarch64-apple-darwin` · `x86_64-apple-darwin`(`.github/workflows/release.yml:57-72` 의 matrix 이고 릴리스 자산이 그 넷이다). ⚠ **`x86_64-unknown-linux-musl` 은 배포 타깃이 아니다** — 이 표가 그렇게 적고 있었고, 실제 산출과 어긋났다. **`x86_64-pc-windows-msvc`(지원 대상 · 아직 미검증 — 아래를 볼 것)** |
| 바이너리 | `pal` 하나. 서브커맨드 **17** (2026-08-18 에 `serve` 를 빼면서 18 → 17) |
| 설치 데이터 위치 | 대상 저장소의 `.palimpsest/` — `cache/`·`index.redb`·**`intent.redb`는 gitignore**(전부 파생), **`intent/*.jsonl`은 커밋**(정본, §2.2) |
| 릴리스 | **P2까지 릴리스 아티팩트를 만들지 않는다.** 자기 저장소와 코퍼스에서만 돈다 |

### ★ Windows 는 **지원 대상이다. 그리고 아직 하나도 검증되지 않았다**

소유자 결정(2026-08-16) — 원문:

> **그럼 windows 를 대응한다는 가정하에 앞으로 모든 설계와 개발이 되어야 해**

그 전까지 이 표는 타깃을 둘로 적었고 Windows 를 **선언하지도 배제하지도 않았다.**
저장소 전체 grep 에서 Windows 는 3줄뿐이었고 그중 둘은 남의 문서 인용이었다.
**침묵이 곧 미결이었고, 이 줄이 그것을 닫는다.**

**그런데 「지원 대상」은 「검증됐다」가 아니다.** 지금 아는 것을 정확히 적는다:

| | 지금 사실 |
|---|---|
| Windows 빌드 | **한 번도 안 했다.** 이 저장소에서 `x86_64-pc-windows-msvc` 로 빌드한 기록이 없다 |
| Windows 시험 | **한 번도 안 돌렸다.** CI 가 없고(`.github/` 자체가 없다) 이 기계는 darwin-arm64 다 |
| 유닉스 전용 가정 | **코드에 남아 있다.** 모드 비트(실행 권한 검사) · `nlink`(하드링크 방어) · `FileTypeExt`(FIFO·소켓·장치 구분) 셋이 `#[cfg(unix)]` 안에 있고, 그쪽에서는 **그 방어가 없다** |
| 짝 없는 시험 | **없다.** 유닉스 fixture 위에 선 시험은 전부 `#[cfg(not(unix))]` 짝이 **시끄럽게 실패**하도록 달아 뒀다 — 다른 플랫폼에서 초록을 내면서 아무것도 안 재는 상태가 안 되게 |
| 이미 옮긴 것 | **훅 등록이 exec form 이다.** shell form 은 Git Bash 가 없으면 **PowerShell** 로 돌고 POSIX 홑따옴표가 안 통한다 — 그 갈림을 등록 형태로 없앴다(`install/hooks.rs`) |
| ⚠ **재서 깨진 것** | **`core.autocrlf=true` 가 매니페스트의 sha256 대조를 깬다.** 아래를 볼 것 |

#### ⚠ `core.autocrlf` — 실측된 반증 하나

Windows git 의 기본값은 `core.autocrlf=true` 이고, 그러면 **체크아웃 때 텍스트 파일이
CRLF 로 바뀐다.** 매니페스트는 sha256 으로 판정한다. 이 둘은 같이 못 선다.

macOS 에서 `git config core.autocrlf=true` 를 건 clone 으로 **모사해서 쟀다**(2026-08-16):

| 재본 것 | 결과 |
|---|---|
| `pal doctor --install` 검사 2 | **빨강** — 설치된 리소스 **다섯 전부**의 sha256 이 다르다. `git status` 는 **깨끗하다** |
| `pal update` | *"이미 최신입니다"* — 버전이 같아서 **파일을 보지도 않는다.** 초록으로 갈 길이 없다 |
| `pal install` 재실행 | **다섯을 전부 `user_modified` 로 다시 도장 찍는다.** 그러면 `doctor` 는 초록이 되지만 **그 다섯은 영원히 갱신 대상에서 빠진다** |
| `pal uninstall` | **통째로 거부한다** — *"블록이 손으로 고쳐졌거나 마커가 훼손됐다. 아무것도 지우지 않았다: CLAUDE.md"*. 블록 제거는 우리가 넣은 바이트열 완전 일치인데 실물이 CRLF 다. **걷어낼 방법이 없다** |

**이 회차는 고치지 않았다.** 설계 결정이 필요한 자리이고, 고를 수 있는 길이 최소
셋이다 — (a) 리소스를 `.gitattributes` 로 `-text` 선언 (b) sha256 을 정규화된
바이트(LF)에 대해 계산 (c) 우리 파일을 아예 커밋 대상에서 뺀다. **셋이 서로 다른
것을 잃는다**(각각 남의 `.gitattributes` 를 건드림 · 실물과 기록의 대응이 한 겹
멀어짐 · 매니페스트가 clone 을 따라가지 않게 됨). 고르는 것은 이 문서의 일이 아니다.
