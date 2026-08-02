# 사전계산된 코드 그래프 — 선례 대조

> **이 문서의 지위**
>
> - **성격**: 문헌 재조사. **판정 있음** — 각 선례에 대해 "이 설계가 무엇을 빌리고 무엇을 빌리지 않는가"까지 적는다.
> - **방법**: 레거시 palimpsest의 조사 문서 하나(아래 "원본")를 출발점으로, 1차 출처를 **다시 열어** 현행 상태를 확인하고, 원본에 없던 축 넷을 추가 조사했다. 검증은 **1차 출처 직접 확인**이며, 원본 §1~§6이 거쳤던 3표 적대검증은 **이 라운드에 없다.**
> - **원본**: `palimpsest-backup/docs/research/precompute-hugrag-kg.md` (2026-06-30 작성 + 2026-07-01 확장). **이 문서가 그것을 대체한다.** 원본은 레거시 저장소에 남으며 이 저장소로 복사하지 않는다 — 원본의 §4(Neo4j 노드/엣지 제안)는 현행 설계가 자체 스키마로 이미 지나간 자리이고(§7.2), 그대로 옮기면 [`DESIGN.md`](../DESIGN.md)와 경합하는 두 번째 스키마가 생긴다.
> - **백서·설계와의 관계**: 이 연구는 **증거이지 요구사항이 아니다**([README 33장](README.md#33장-이-연구를-백서에-합치지-않는-이유)). 능력 계약을 늘리는 데 쓰지 말 것. 이 문서가 설계에 닿는 지점은 §9에 네 개(R9~R12)로 한정했고, 그 외는 전부 **근거 보강이거나 반대 증거**다.
> - **갱신 조건**: 여기 인용한 1차 출처의 상태가 바뀔 때(CDT의 평가 등장, Codebase-Memory의 독립 재현 등).

**원본 대비 무엇이 달라졌나**

| | 원본(2026-06/07) | 이 문서(2026-08-03) |
|---|---|---|
| **Glean의 지위** | "우리 projection 모델의 **직접 선례**" | **뒤집힘** — Glean의 fact는 컴파일러 통합 인덱서에서 나온다. 현행 설계의 배정 규칙을 적용하면 `extracted`가 아니라 `observed`이며, **횡단요구 U의 선례가 아니다**(§1) |
| **U(빌드 비의존)의 선례** | 없음 — 원본은 U를 검사 축으로 세우지 않았다 | **stack-graphs**를 추가(§2). 빌드·설정 없이 파일 단위 사전계산 + 질의 시점 스티칭 + 증분. **그 대가까지 명시된 유일한 선례** |
| **증분·무효화** | Glean stacking을 한 줄로 언급 | **ownership/unit + derived fact 가시성 규칙**까지 확인(§3). 규칙 파생 라벨의 stale 전파에 검증된 선례가 생겼고, **비용 수치가 D16 예산의 첫 외부 기준선**이 된다 |
| **물리 설계의 선례** | 없음 | **Codebase-Memory**(§4) — tree-sitter + 단일 SQLite + 단일 정적 바이너리 + MCP. 현행 §12.1과 거의 동형이고 **측정치가 있으며, 그 측정치가 이 설계에 불리하다** |
| **Meta pre-compute** | §7.1에서 "별개 시스템"으로 분리 | 유지·재확인. 여기에 **결박이 없다는 것**이 palimpsest와의 유일한 실질 차이임을 명시(§5) |
| **HugRAG/CDT/Neo4j 설계 제안** | 접목 대상 | **채택하지 않음**(§7). 현행 D5·D3이 더 엄격한 답을 이미 냈다 |

---

## 0. 한 줄 결론

**사전계산 자체는 2026년의 기본값이고 선례가 넘친다. 비어 있는 칸은 신선도·출처·확신도의 계약이다** — 조사한 어느 시스템도 "이 사실이 언제 것이고, 무엇이 만들었고, 어디까지 유효한가"를 산출에 달지 않는다. 그리고 **가장 가까운 동형 시스템의 실측은 그래프 경유가 파일 탐색보다 답변 품질이 낮다고 말한다**(83% vs 92%) — 이것이 이 조사가 가져온 가장 중요한 한 줄이며, 설계에 유리한 쪽이 아니다.

---

## 1. Glean — 원본의 판정이 뒤집히는 지점

### 1.1 무엇이 확인됐나

Meta의 Glean은 소스코드 정보를 **predicate(≈테이블)의 인스턴스인 fact(≈행)**로 사전 색인하고 **Angle**(Datalog 계열)로 질의한다. 재파싱이 아니라 색인된 사실을 질의한다는 모델은 원본이 적은 그대로다. 엣지도 1급 fact이고(`Parent{child, parent}`), derived fact는 다른 fact로부터 파생된다.

### 1.2 원본이 묻지 않은 질문 — 그 fact는 어디서 나오나

**인덱서가 컴파일러다.**

- C++ 인덱서는 **clang·libclang·llvm을 요구**하고, 생성 코드를 색인하려면 **먼저 빌드되어야 한다**("all generated code must be built first").
- Hack 인덱서는 **Hack 타입체커에 내장**되어 있다.
- Go·Java·Rust·TypeScript는 자체 인덱서가 아니라 **LSIF/SCIP 수용**으로 커버된다 — 즉 그 언어들의 fact는 **다른 빌드 통합 도구가 만든 것**을 받는 것이다.

**현행 설계의 배정 규칙**([DESIGN.md §3](../DESIGN.md))을 그대로 적용하면 판정은 하나다.

> 1. `(커밋, 추출기 버전)`만으로 재현되는가 → **아니다.** 빌드 환경이 좌표에 필요하다.
> 2. 환경에 의존하는 절차가 실제로 산출한 것인가 → **그렇다.**
>
> → **`observed`.**

이것은 설계가 이미 명시한 경계 사례("빌드 의존 정밀 엔진의 산물 → `observed`")와 **같은 판정**이다. 새 규칙이 필요하지 않았다는 것이 오히려 배정 규칙의 첫 외부 검사 통과다.

### 1.3 그래서 무엇이 바뀌나

**원본의 "직접 선례"라는 문장은 U 아래에서 성립하지 않는다.** 원본은 Glean에 대해 *"백엔드는 RocksDB지 Neo4j가 아니다 — 구현이 아니라 모델을 차용"*이라는 주의만 달았는데, **실제 문제는 저장 기술이 아니라 입력 조달 경로였다.** Glean이 실증한 것은 "빌드된 프로젝트에서 사실을 미리 계산해 질의하면 잘 된다"이고, 백서 U가 요구하는 것은 **"빌드되지 않는 임의의 과거 커밋에서"**다. 후자에 대해 Glean은 아무 말도 하지 않는다.

**빌리는 것과 두고 오는 것을 갈라 적는다.**

| Glean의 것 | 처분 | 근거 |
|---|---|---|
| predicate/fact 분리, 엣지도 1급 fact | **빌린다** — 이미 이 설계의 형태다 | §5의 엣지 속성(등급·출처·증거) |
| **derived fact의 가시성 규칙** | **빌린다** — §3의 검증된 선례 | 아래 §3.2 |
| **unit 기반 증분 + ownership** | **빌린다** | 아래 §3.1 |
| 컴파일러 통합 인덱서 | **두고 온다** | U 위반. 산물은 `observed`로 수용(§12.2 관측 수용 API) |
| Angle(Datalog) 질의어 | **두고 온다** | §12.5 — 질의 **언어**는 표면 계약이 아니다 |

**그리고 하나가 열린다**: LSIF/SCIP를 수용 포맷으로 쓰는 것은 Glean이 이미 하는 일이다. 현행 설계의 관측 수용 API(§12.2)가 **조달자를 묻지 않는다**고 선언했으므로, SCIP 인덱스는 그 API의 **가장 구체적인 첫 조달원 후보**다. 이것이 R9다(§9).

> **출처**: [engineering.fb.com/2024/12/19](https://engineering.fb.com/2024/12/19/developer-tools/glean-open-source-code-indexing/) · [glean.software/docs/indexer/cxx](https://glean.software/docs/indexer/cxx/) · [glean.software/docs/indexer/hack](https://glean.software/docs/indexer/hack/) · [github.com/facebookincubator/Glean](https://github.com/facebookincubator/Glean)

---

## 2. stack-graphs — U 계약의 진짜 선례

원본에 없던 축이고, 이 조사에서 가장 큰 수확이다.

### 2.1 무엇인가

GitHub의 stack-graphs는 임의 언어의 **이름 해소 규칙을 선언적으로 정의**하되, 그 실행이 **효율적이고 증분적이며 기존 빌드·프로그램 분석 도구에 기대지 않는** 방식으로 되게 한다. GitHub는 이것으로 **저장소 소유자의 설정 없이, 빌드나 CI 작업 없이** 코드 내비게이션 데이터를 생성한다. tree-sitter 파서 생태계 위에 선다.

### 2.2 형태가 이 설계와 같은 곳

| stack-graphs | 현행 설계의 대응 |
|---|---|
| **파일마다 독립적으로 분석**해 부분 그래프를 만든다 | **D2 콘텐츠 주소 캐시** — 키가 `(blob 해시, 추출기 버전)`인 이유가 바로 이것이다(§12.1·§2.3) |
| 질의 시점에 파일 단위 그래프를 **커밋 전체로 스티칭**해 경로 탐색 | **§12.3 지연 평가** — "모든 커밋이 미리 추출되어 있음"이 아니라 "임의 커밋에 대해 같은 방식으로 답할 수 있음" |
| 커밋마다 변경 파일이 소수이므로 **변경분만 재계산하고 나머지는 재사용** | **§12.4 증분 재추출 목표**(변경 파일 10²에서 2초) |
| 심볼 스택 push/pop으로 **잘못된 경로 해소를 막는다**; 엣지 precedence로 언어별 shadowing | **D5 해소 등급** — `exact`/`scoped`가 서는 자리 |

**이것이 R10이다**: 백서 U는 "빌드 없이 임의 커밋에서 균일하게"를 요구하고, 현행 설계 §12.1은 tree-sitter를 "U가 성립하는 유일한 부류"라고 적으면서 그 판단에 외부 증거를 대지 않았다. **stack-graphs가 그 증거다** — 프로덕션 규모에서 실제로 돌고 있다.

### 2.3 그리고 그 대가가 명시되어 있다 — 이쪽이 더 값지다

GitHub 자신이 미해결로 남긴 것:

- **데이터플로우가 없다.** 정의-사용 관계는 이 프레임의 답이 아니다.
- **제네릭 타입 파라미터 추적이 미해결 질문**으로 남아 있다.
- **크로스 저장소·의존성 해소는 다루지 않는다** — 단일 커밋 분석에 초점.

세 항목이 현행 설계의 세 자리에 **정확히** 떨어진다.

| stack-graphs가 못 하는 것 | 현행 설계의 자리 | 귀결 |
|---|---|---|
| 데이터플로우 | **C5 효과 집합**(설계가 지배관계보다 앞으로 옮긴 그것) | **가장 큰 미검증 구간이 여기임을 외부가 확인해준다.** S4의 반증 조건이 이 설계에서 가장 비싼 검사인 이유 |
| 제네릭 타입 파라미터 | §2.2 `identity_grade`의 L1/L2 경계 | 판별자가 언어 등급에 의존한다는 결정이 임의가 아니었음 |
| 크로스 저장소 | **C4 경계 통과** + `Snapshot`(커밋 집합) | 경계 엣지를 `contract` 등급 + 계약 아티팩트 증거로 한정한 것(§5.2)이 회피가 아니라 **선례가 비워둔 칸을 정직하게 표시한 것** |

> **정직하게**: stack-graphs가 이 셋을 못 한다는 사실은 palimpsest가 그것을 할 수 있다는 근거가 **아니다.** 같은 기반(tree-sitter, 무빌드) 위에서 같은 벽에 부딪힐 개연성이 높다는 쪽이 자연스러운 독해다. 이 관측의 정당한 용법은 **S4·S6의 반증 조건을 진지하게 받는 것**이지, 능력 약속을 늘리는 것이 아니다.

> **출처**: [github.blog/introducing-stack-graphs](https://github.blog/open-source/introducing-stack-graphs/) · [github.com/github/stack-graphs](https://github.com/github/stack-graphs)

---

## 3. 증분과 무효화 — Glean의 ownership이 파생 사실에 대해 답한 것

### 3.1 unit·ownership

- 재색인 단위는 **unit**이고, unit은 그냥 문자열이다 — Glean은 의미를 강제하지 않으며 보통 파일명이나 모듈명이다.
- **ownership set**이 "어떤 fact가 어떤 unit에 속하는가"를 들고 있고, unit을 숨길 때 dangling reference가 생기지 않게 한다. Elias-Fano 코딩 + 구간 맵으로 저장하며 **DB 크기의 약 7% 증가**.
- 스택은 임의 깊이로 쌓이고 트리를 이루며 중간 노드도 동시에 질의 가능하다.

### 3.2 derived fact의 가시성 — 이 설계가 규칙 파생 라벨에 대해 정한 것과 같다

> **"derived fact는 언제 보이는가? 그것이 파생된 fact가 **전부** 보일 때."**

현행 설계 §3.2는 선언 팩의 규칙 하나가 라벨 300개를 파생시키는 경우에 대해 이렇게 정했다 — *"규칙 파생 라벨은 규칙 좌표에 결박된다. 규칙이 바뀌면 파생된 전부가 `stale`이 된다."* **같은 계약이다.** 차이는 방향뿐이다: Glean은 근거 fact의 가시성을 따라가고, 이 설계는 근거 좌표의 digest 변경을 따라간다.

**이것이 §3.2에 주는 것**: 그 결정은 지금까지 내적 논증뿐이었고 구현 선례가 없었다. 이제 **프로덕션 규모(수십억 fact 모노레포)에서 도는 선례가 있다.** 다만 Glean 자신이 *"일반적인 경우의 증분 파생 성능은 최적화되지 않았다"*고 적는다 — 이 설계가 낙관해도 되는 지점이 아니라는 것까지 함께 온다.

### 3.3 숫자 — D16 예산의 첫 외부 기준선

현행 §12.4는 "값 없이 켜지지 않는다"를 규칙으로 세우고 초기값을 전부 미검증으로 적었다(잔여 #8). 외부 기준선이 생겼다.

| 축 | Glean 실측 | 이 설계의 대응 자리 |
|---|---|---|
| 증분 색인 오버헤드 | Python **2~3%**, Hack은 무시 가능 | §12.4 증분 재추출 목표 |
| ownership 저장 비용 | DB 크기 **+약 7%** | 좌표·출처·등급 메타데이터의 저장 비용 상한 감각 |
| 질의 오버헤드 | 일반 질의 **10% 미만**, 검색 집약 질의 **약 3배** | §12.4 후보 집합 상한 `K`·경로 곱 예산 `B`가 겨냥하는 그 지점 |

**"약 3배"가 중요하다.** 스택 깊이가 붙는 순간 비싸지는 것은 **탐색형 질의**이고, 이 설계에서 탐색형 질의는 `candidate` 경유 경로 질의(§5.1)와 결박 폐포 계산(§6.2)이다. 예산이 그 둘에 걸려 있는 것이 우연이 아니었음을 외부 수치가 지지한다.

> **출처**: [glean.software/blog/incremental](https://glean.software/blog/incremental/)

---

## 4. Codebase-Memory — 물리 설계의 동형 선례, 그리고 불리한 실측

### 4.1 형태가 거의 같다

arXiv 2603.27277(2026-03)의 Codebase-Memory는 tree-sitter로 66개 언어를 파싱해 **단일 SQLite 지식 그래프**를 만들고, **MCP로 14개 타입드 도구**(`search_graph`, `trace_call_path`, `query_graph`, `get_architecture` 등)를 노출한다. 후속 구현은 **158개 언어 · 단일 정적 C 바이너리 · 외부 의존 0**을 표방한다.

현행 [§12.1](../DESIGN.md)의 네 행 — tree-sitter 추출기 / 단일 정적 바이너리 / 단일 파일 임베디드 DB / MCP 표면 — 과 **거의 그대로 겹친다.** 이 설계가 그 표에 "ADR 후보이지 결정이 아니다"라고 적은 자리에 대해, **같은 네 선택을 한 시스템이 이미 있고 돌아간다**는 것이 확인됐다.

### 4.2 그리고 측정치가 있다 — 이 설계에 불리한 쪽으로

31개 실제 저장소 평가:

| 측정 | 값 |
|---|---|
| 답변 품질 — 그래프 시스템 | **83%** |
| 답변 품질 — 파일 탐색 에이전트(대조군) | **92%** |
| 토큰 | **10배 적음** |
| tool 호출 | **2.1배 적음** |
| 그래프 고유 질의(허브 탐지·호출자 순위)에서 대조군 이상 | 31개 중 **19개** |

**세 가지가 따라 나온다.**

1. **R8 기준선의 대조군이 하나 늘어난다.** 현행 §13은 각 슬라이스의 대조군을 "사람이 손으로 적은 마크다운 한 장"(S6·S7은 "기존 정적 분석 도구 + 문서 한 장")으로 정했다. **여기에 "tree-sitter 그래프 + MCP 도구" 부류가 추가된다** — 이미 존재하고, 무료이고, 설치가 가볍다. palimpsest가 이겨야 할 상대는 문서만이 아니다. 이것이 R11이다.

2. **D22(서술은 그래프를 경유한다)에 직접 반대되는 증거다.** 현행 §9.4는 U5를 받아 서술의 기본 경로를 그래프 질의로 정하면서, *"하한 위의 서술은 새 거짓 안전을 만든다"*는 위험을 스스로 적었다. **그 위험에 이제 수치가 붙었다 — 그래프만 본 쪽이 파일을 읽은 쪽보다 답변 품질이 9%p 낮다.** §9.4의 `narration_basis` 3값(`graph-only` / `graph+direct-read` / `undeclared`)이 이미 그 구별을 스키마에 두었지만, **그 세 값 중 어느 것을 기본으로 삼을지는 이 수치를 보고 다시 생각해야 한다.**

3. **그러나 이 수치를 과대 해석하면 안 된다.** 이 시스템에는 **결박도 신선도도 3분할도 없다** — 구조 사실을 싸게 주는 도구이지 판정을 내는 도구가 아니다. 83%가 낮은 것은 "그래프가 나쁘다"의 증거일 수도, "이 그래프가 얕다"의 증거일 수도 있고, 조사 범위에서 그 둘은 구별되지 않았다. **독립 재현도 없다.**

> **출처**: [arxiv.org/abs/2603.27277](https://arxiv.org/abs/2603.27277) · [github.com/DeusData/codebase-memory-mcp](https://github.com/DeusData/codebase-memory-mcp)

---

## 5. Meta의 pre-compute engine — 사전계산된 *서술*

이것이 원본 §7.1이 Glean과 분리해 적은 시스템이고, **"Meta의 pre-compute 기법"이라는 말이 실제로 가리키는 것**이다. Glean은 fact 색인 DB이고, 이것은 LLM 에이전트 스웜이 암묵지를 요약해 **context 파일**을 만드는 파이프라인이다. 1차 출처를 다시 열어 전부 재확인했다.

### 5.1 재확인된 사실

- **다섯 질문**(module analyst가 모듈마다 답한다): ① 이 모듈은 무엇을 설정하는가 ② 흔한 수정 패턴은 ③ **빌드 실패를 일으키는 비자명 패턴**은 ④ 모듈 간 의존은 ⑤ 코드 주석에 묻힌 tribal knowledge는.
- **context 파일 = "compass, not encyclopedia"**: **25~35줄(~1,000토큰)**, 4개 고정 섹션(Quick Commands / Key Files 3~5개 / Non-Obvious patterns / See Also).
- **로딩은 opt-in** — 관련할 때만 싣고 상시 로드하지 않는다.
- **에이전트 파이프라인**: explorer 2 → module analyst 11 → writer 2 → critic 10+(3라운드) → fixer 4 → upgrader 8 → prompt tester 3 → gap-filler 4 → final critic 3.
- **수치**: 커버리지 5%(5개 파일) → 100%(59개 파일), 내비게이션 대상 ~50 → **4,100+**, 비자명 패턴 0 → **50+**, critic 품질 **3.65 → 4.20/5.0**, 테스트된 프롬프트 55+.
- **헤드라인 "태스크당 tool 호출·토큰 40% 감소"는 n=6이다.** 원문이 preliminary로 적는다.
- **자가유지**: 몇 주마다 자동 job이 **경로 검증 · 커버리지 갭 탐지 · critic 재실행 · stale 참조 자동 수정**.
- **자인한 반례**: 최근 연구에서 AI 생성 context 파일이 Django·matplotlib 같은 **유명 OSS에서 에이전트 성공률을 떨어뜨렸다.** Meta의 설명은 "그 저장소들은 모델의 사전학습에 있어서 context가 중복 노이즈였다"는 것이다.

### 5.2 현행 설계와의 대조 — 같은 곳과 다른 곳

| Meta pre-compute engine | 현행 설계 | 판정 |
|---|---|---|
| 다섯 질문으로 모듈 요약을 **생성** | §11.2 서술물 인입 · §9.1 `Synthesis` | **같은 부류.** 다섯 질문은 §11.2의 인입 대상이 아니라 **생성 스키마 후보**다 |
| 25~35줄 예산 + opt-in 로딩 | P9 점진 회상 · §11.3 적시 제시 | **같은 방향.** 다만 Meta는 **모듈 단위**로 싣고 이 설계는 **좌표 단위**로 싣는다 |
| 자가유지 job이 **경로를 검증**하고 stale 참조를 고친다 | §6.3 낡음 감지기의 신선도 · §6.1 5상태 | **부분적으로 같다.** 아래 |
| — | **결박**(§3의 좌표·§6의 상태) | **여기가 유일한 실질 차이다** |

**세 번째 줄이 이 대조의 핵심이다.** Meta의 자가유지는 **파일 경로가 실재하는가**를 검증한다 — 원본이 적은 대로 *"파일 경로는 zero-hallucination 검증되나 의미(semantic) 정확성은 무보장"*이다. 현행 설계의 어휘로 옮기면, Meta의 검사는 **`orphaned` 판정만 하고 `stale` 판정을 하지 않는다.** 경로는 그대로인데 그 심볼의 `body_digest`가 바뀐 경우 — 즉 **서술이 낡는 가장 흔한 방식** — 을 그 job은 보지 못한다.

> **그러므로 palimpsest가 이 선례에 대해 주장할 수 있는 차이는 정확히 하나다: 서술이 좌표에 결박되어 있고, 그 좌표가 변하면 서술이 낡았다고 말한다.** 그 이상을 주장하면 — 더 나은 요약, 더 나은 커버리지 — Meta가 50+ 에이전트로 한 일을 더 잘한다는 주장이 되고, 근거가 없다.

**그리고 Django 반례가 §15-24를 강화한다.** 현행 잔여 대장 24번은 *"결박된 설명도 과의존을 줄이지 않는다 — 결박은 오히려 그럴듯함을 더한다"*이다. Meta의 자인은 그보다 강한 형태의 실물이다 — **생성된 컨텍스트가 순효능 음수인 조건이 실재한다.** 이 설계의 §11.5 브리핑·§9.4 서술이 정확히 그 부류의 산출이다.

> **출처**: [engineering.fb.com/2026/04/06](https://engineering.fb.com/2026/04/06/developer-tools/how-meta-used-ai-to-map-tribal-knowledge-in-large-scale-data-pipelines/)

---

## 6. 온톨로지 계열 — 현행 상태만 갱신

원본 §2·§3·§7.2가 다룬 것들이다. **현행 설계가 이미 자체 답을 가지고 있으므로 여기서는 상태 확인에 그친다.**

| 선례 | 2026-08 현재 | 현행 설계와의 관계 |
|---|---|---|
| **CPG / Joern** | 유지. **fuzzy parsing으로 빌드 환경 없이, 코드 일부가 없어도 임포트 가능** — 부분 프로그램 분석이 설계 목표다 | **U 정합 선례 둘째.** CPG의 엣지 어휘(`REACHING_DEF`=정의-사용, `CDG`=제어 의존)가 C5·C6이 서야 할 층의 검증된 이름이다. 다만 CPG는 **오버레이를 한 스키마에 쌓는** 모델이고, 이 설계는 **출처 축 4값으로 파티션**한다 — 층 구분의 축이 다르다(구문/흐름 vs 인식론적 지위) |
| **Graphiti / Zep** | 유지·성장(OSS 20k+ stars). bi-temporal 4 타임스탬프(`t_valid`/`t_invalid`/`t_created`/`t_expired`), 모순 시 **삭제가 아니라 invalidate** | **D7 5상태의 선례.** `superseded`가 정확히 Graphiti의 invalidate다. **그러나 palimpsest의 두 번째 축(코드-결박 신선도 — `body_digest` 변경으로 켜지는 `stale`)은 여전히 어느 선례에도 없다.** 원본의 판정이 유지된다 |
| **CDT (Code Digital Twin)** | arXiv 2503.07967 **v4(2026-02-02)**. 여전히 **vision paper**를 자칭하며 지속 갱신 중. **구현·평가 없음** | 물리층/개념층 2층 어휘(`justified-by`, `constrained-by`, `has-responsibility`)는 §7.1 규약층·의도층 어휘의 후보로 남는다. **평가가 0인 상태가 5개월째 유지되므로, 어휘 이상을 빌리면 안 된다** |

> **출처**: [docs.joern.io](https://docs.joern.io/) · [cpg.joern.io](https://cpg.joern.io/) · [github.com/getzep/graphiti](https://github.com/getzep/graphiti) · [arxiv.org/abs/2501.13956](https://arxiv.org/abs/2501.13956) · [arxiv.org/abs/2503.07967](https://arxiv.org/abs/2503.07967)

---

## 7. 채택하지 않는 것

### 7.1 HugRAG / CausalRAG2

식별자 문제는 확정됐다 — **arXiv 2602.05143은 v1에서 "HugRAG", v2부터 "CausalRAG2"이며 동일 논문이고 ICML 2026 게재**다. 메커니즘(unified edge space, hierarchical causal gating, confidence threshold)은 원본이 적은 대로이고, 원본이 기각한 것("세 엣지 클래스가 결정론/추론에 1:1 대응한다")도 그대로 거짓이다.

**채택하지 않는 이유는 현행 설계가 더 엄격한 답을 이미 냈기 때문이다.** HugRAG의 causal gate는 *LLM이 그럴듯함을 평가해 임계 이상이면 엣지를 만든다*이고, 현행 §5.2는 경계 엣지에 대해 **계약 아티팩트를 증거로 요구하고 이름 일치만으로는 엣지를 만들지 않는다**(`unresolved-boundary`로 대장에 남긴다). 후자가 전자를 포함한다 — confidence threshold는 증거 없는 엣지를 **점수와 함께 만들지만**, 이 설계는 **만들지 않는다.** 그리고 백서 C2("거짓 엣지는 없는 엣지보다 나쁘다")가 그 선택의 근거다.

> 덧붙여, 그래프 구조가 항상 이득이라는 전제 자체가 흔들리고 있다. [arXiv 2604.09666](https://arxiv.org/pdf/2604.09666)은 에이전틱 검색에서 **GraphRAG가 보편적으로 우월하지 않다**고 보고한다 — 멀티홉·관계 추론에서는 이득이고 단일 문서 조회나 그래프 연결성이 희박한 경우에는 아니다. **이 설계에서 그래프가 값을 내야 하는 자리가 어디인지를 좁히는 관측**이며, §11.6이 "이길 지점은 셋뿐"이라고 적은 것과 같은 방향이다.

### 7.2 원본 §4의 Neo4j 노드/엣지 설계 제안

원본은 CPG + HugRAG + Glean + Graphiti를 종합해 Neo4j 노드/엣지 타입과 속성(`edge_kind = deterministic|inferred`, `source`, `valid_from/valid_to`, `code_bound_at`)을 제안했고, 스스로 **confidence medium · v1 스파이크로 검증 대상**이라 적었다.

**현행 설계가 그 자리를 이미 지나갔다.** 대응은 이렇다.

| 원본 §4 제안 | 현행 설계 | 차이 |
|---|---|---|
| `edge_kind = deterministic \| inferred` (2값) | **출처 축 4값** `extracted / observed / asserted / inferred` + **배정 규칙(순서 있는 4단계)** | 2값은 가드 라벨과 실행 관측을 담지 못한다. 이 설계가 그것을 B2·§0.4에서 이미 확인했다 |
| `source` = commit SHA 역참조 | **3축 좌표** `(repo_id, commit_sha, extractor_version, symbol_id)` + `Snapshot` | 추출기 버전 축이 없으면 P5가 문자 그대로 거짓 |
| `valid_from`/`valid_to` | **5상태** `live/stale/pending/orphaned/superseded` + **결박 반경** | 유효 구간만으로는 `orphaned`와 `pending`이 구별되지 않는다 |
| `code_bound_at` | `body_digest` + 감시 집합 | 시각이 아니라 **내용 해시**여야 포매팅 변경에 거짓 양성이 나지 않는다 |
| Neo4j 백엔드 | §12.1 단일 파일 임베디드 DB(교체 조건 명시) | 그래프는 **교체 가능한 투영**이므로 백엔드는 표면 계약이 아니다(§12.5) |

**그러므로 원본 §4는 이 저장소로 옮기지 않는다.** 옮기면 DESIGN.md와 경합하는 두 번째 스키마가 생기고, 그 경합은 이 설계가 R7·R6에서 배운 것(자리를 만들어도 채워지지 않거나, 목록은 다시 열리지 않는다)과 같은 방식으로 실패한다.

---

## 8. 2026년 지형에서 비어 있는 칸

에이전트용 코드 인텔리전스 지형을 훑은 2차 정리 하나가 이 조사의 결론을 대신 말한다.

- **수렴한 패턴**: "구조를 사전계산하고, 도구로 노출하고, 에이전트가 좁은 사실을 질의하게 한다." **MCP가 지배적 전송층**이 됐다.
- **두 갈래**: 무빌드·구문 계열(tree-sitter — Aider repo map, Coograph, KotaDB, RepoMapper, codeindex)과 빌드 인지·의미 계열(SCIP — Sourcegraph, LSP 기반 — Serena). 전자는 싸고 이식성 있으나 타입·빌드 의미를 놓치고, 후자는 정확하나 언어 툴체인에 의존하고 유지 비용이 크다.
- **그리고 비어 있는 칸**:

> *"MCP는 신뢰보다 조합 가능성을 먼저 만든다. 프로토콜은 에이전트가 도구를 일관되게 호출하게 하지만, **코드 사실에 대한 보편적 확신도 모델도, 신선도 모델도, 출처 계약도 노출하지 않는다.**"*

같은 정리가 **어느 시스템도 명시적 신선도 추적이나 확신도 주석을 서술하지 않았다**고 적는다. 로컬 도구들은 수동 재색인을 요구한다.

**이것이 이 조사가 palimpsest에 대해 말할 수 있는 가장 강한 문장이다.** 사전계산은 선례가 넘치고 물리 설계까지 동형인 것이 이미 있다(§4). **차별 축은 사전계산 자체가 아니라 그 위에 붙는 세 계약** — 출처(§3), 신선도(§6), 하한 표시(§4.2·§8) — 이고, 그 셋을 함께 다루는 시스템을 조사 범위에서 찾지 못했다.

**다만 이것은 "빈 칸이 있다"는 존재 주장일 뿐, "그 칸을 채우면 값이 있다"의 증거가 아니다.** 아무도 안 하는 이유가 어렵거나 필요 없어서일 수도 있다. 그 판정은 §13의 R8 기준선이 한다.

> **출처**: [anthonywest.co.uk/research/code-intelligence-indexing-2026-openai](https://anthonywest.co.uk/research/code-intelligence-indexing-2026-openai) · [sourcegraph.com/blog/announcing-scip](https://sourcegraph.com/blog/announcing-scip) · [scip-code.org](https://scip-code.org/)

---

## 9. 설계에 미치는 영향 — 넷

이 문서가 [`DESIGN.md`](../DESIGN.md)에 넣기를 제안하는 것은 아래 넷뿐이다. **나머지는 근거 보강이며 설계를 바꾸지 않는다.**

| # | 관측 | 설계 영향 |
|---|---|---|
| **R9** | **Glean·SCIP·LSIF·Kythe 계열의 정밀 인덱스는 컴파일러/빌드 통합 위에 선다.** 배정 규칙을 적용하면 그 산물은 `observed`이고, Glean 자신이 Go·Java·Rust·TS를 **LSIF/SCIP 수용**으로 커버한다 | 관측 수용 API(§12.2)의 **조달원 목록에 "표준 코드 인덱스 포맷"을 명시**. 새 능력이 아니라 C8의 첫 구체적 조달 형태이며, `UnresolvedRef` 해소 경로(§5.4)에 한 줄이 는다 |
| **R10** | **stack-graphs가 빌드·설정 없이 파일 단위 사전계산 + 질의 시점 스티칭 + 증분을 프로덕션에서 돌린다.** 그리고 못 하는 셋(데이터플로우·제네릭·크로스레포)이 명시되어 있다 | §12.1 추출기 행과 §12.3 지연 평가의 **외부 근거**. 그리고 **S4(효과 집합)·S6(경계)의 반증 조건이 이 설계에서 가장 비싼 검사임을 확인** — 같은 기반 위의 선례가 정확히 그 둘에서 멈췄다 |
| **R11** | **동형 시스템(tree-sitter + 단일 SQLite + 정적 바이너리 + MCP)이 이미 있고 실측이 있다** — 31개 저장소에서 답변 품질 83% vs 파일 탐색 92%, 토큰 10배·tool 호출 2.1배 절감 | ① **R8 기준선의 대조군에 "기존 그래프 MCP 도구" 부류 추가**(§13). ② **D22의 기본 경로에 대한 반대 증거** — `narration_basis`의 기본값을 `graph-only`로 두는 것을 §15의 잔여로 등록 |
| **R12** | **2026 지형 어느 시스템도 신선도·출처·확신도 계약을 노출하지 않는다.** MCP는 조합 가능성을 만들지 신뢰를 만들지 않는다 | 차별 축의 외부 확인. **설계를 바꾸지 않는다** — §13의 R8 기준선이 그대로 판정한다. §16의 관찰로만 적는다 |

---

## 10. 이 조사의 한계

1. **3표 적대검증을 거치지 않았다.** 원본 §1~§6이 거친 절차가 이 라운드에는 없다. 1차 출처를 직접 열어 확인했으나, 그것은 **인용의 정확성**을 담보하지 검증의 강도를 담보하지 않는다.
2. **§4의 83% vs 92%는 논문 자기보고이며 독립 재현이 없다.** 그리고 "답변 품질"의 조작적 정의를 이 조사가 확인하지 못했다 — 어떤 질의 집합에서 어떤 채점으로 나온 값인지 초록 수준에서만 봤다.
3. **Meta의 40%는 n=6이다.** 원문이 preliminary로 적고, 2차 논평도 "벤치마크가 아니라 방향 신호"로 못 박는다. 이 문서는 그 수치를 어떤 논증의 근거로도 쓰지 않았다.
4. **stack-graphs의 "못 하는 셋"은 2021년 발표 시점 서술에 기반한다.** 이후 진전이 있었는지 이 조사는 확인하지 못했다 — 확인되면 §2.3의 대응 표가 바뀐다.
5. **Glean의 비용 수치는 Meta의 워크로드 위 값이다.** 저장소 규모·언어 구성이 다르면 그대로 옮겨지지 않는다. §3.3의 용법은 "자릿수 감각"까지다.
6. **CDT는 5개월째 평가가 없다.** "구현 예정"이 계속 갱신되는 상태이므로, 다음 갱신에서 실체가 나오면 §6의 판정이 바뀐다.
7. **조사 범위가 영어권 공개 자료에 한정된다.** 사내 시스템(Google Kythe의 현행 운용, Meta 내부의 Glean 소비 형태)은 공개된 만큼만 봤다.
8. **선례가 안 하는 일을 우리가 할 수 있다는 근거는 이 문서 어디에도 없다.** §8의 "빈 칸"은 존재 주장이고, §2.3에 적은 대로 같은 기반 위에서 같은 벽에 부딪힐 개연성이 오히려 자연스럽다.

---

## 부록 — 1차 출처

**사전계산 · 인덱싱**

- Glean: [engineering.fb.com/2024/12/19](https://engineering.fb.com/2024/12/19/developer-tools/glean-open-source-code-indexing/) · [glean.software/blog/incremental](https://glean.software/blog/incremental/) · [glean.software/docs/indexer/cxx](https://glean.software/docs/indexer/cxx/) · [glean.software/docs/indexer/hack](https://glean.software/docs/indexer/hack/) · [github.com/facebookincubator/Glean](https://github.com/facebookincubator/Glean)
- stack-graphs: [github.blog/introducing-stack-graphs](https://github.blog/open-source/introducing-stack-graphs/) · [github.com/github/stack-graphs](https://github.com/github/stack-graphs)
- SCIP: [sourcegraph.com/blog/announcing-scip](https://sourcegraph.com/blog/announcing-scip) · [scip-code.org](https://scip-code.org/)
- Codebase-Memory: [arxiv.org/abs/2603.27277](https://arxiv.org/abs/2603.27277) · [github.com/DeusData/codebase-memory-mcp](https://github.com/DeusData/codebase-memory-mcp)

**사전계산된 서술**

- Meta tribal knowledge pre-compute engine: [engineering.fb.com/2026/04/06](https://engineering.fb.com/2026/04/06/developer-tools/how-meta-used-ai-to-map-tribal-knowledge-in-large-scale-data-pipelines/)

**온톨로지 · 시간축**

- CPG / Joern: [cpg.joern.io](https://cpg.joern.io/) · [docs.joern.io](https://docs.joern.io/)
- Graphiti / Zep: [github.com/getzep/graphiti](https://github.com/getzep/graphiti) · [arxiv.org/abs/2501.13956](https://arxiv.org/abs/2501.13956)
- Code Digital Twin: [arxiv.org/abs/2503.07967](https://arxiv.org/abs/2503.07967) (v4, 2026-02-02)

**채택하지 않은 것**

- HugRAG / CausalRAG2: [arxiv.org/abs/2602.05143](https://arxiv.org/abs/2602.05143) (v1=HugRAG, v2=CausalRAG2, ICML 2026)
- GraphRAG 벤치마크: [arxiv.org/abs/2604.09666](https://arxiv.org/pdf/2604.09666)

**2차 · 지형**

- [anthonywest.co.uk/research/code-intelligence-indexing-2026-openai](https://anthonywest.co.uk/research/code-intelligence-indexing-2026-openai)

**레거시 원본** (이 저장소 밖)

- `~/dev/projects/palimpsest-backup/docs/research/precompute-hugrag-kg.md` — 2026-06-30 작성 + 2026-07-01 §7 확장. 방법: 5각도 fan-out 검색 → 22개 1차 자료 → 107개 주장 추출 → 상위 25개 3표 적대검증(23 confirmed · 2 refuted).
