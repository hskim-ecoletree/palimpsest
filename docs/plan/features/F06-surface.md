# F06 — 표면: 질의 카탈로그 · **CLI**(P0) · MCP·플러그인 어댑터(P1)

| 우선순위 | 의존 | 규모 | 크레이트 |
|---|---|---|---|
| **P0** — 카탈로그 + CLI | F05 | M | `surface/queries.toml` · `pal-cli` |
| **P1** — MCP·플러그인 (§4b) | F06 | S | `pal-mcp` |

> 분리 근거: [DESIGN §12.2](../../DESIGN.md) (D30) · [지시 U17](../../instructions/2026-08-10-owner-direction.md)

---

## 1. 왜

**표면이 계약이면 목록도 계약이다.** 질의가 코드 여기저기에 흩어져 자라면 ① 무엇이 제공되는지 아무도 모르고 ② CLI와 MCP가 갈라지고 ③ "호스트 없이도 코어가 답한다"는 검사가 무엇에 대해 도는지 정해지지 않는다.

그리고 **표면은 양방향이다** — 호스트는 질의하는 쪽이기만 한 것이 아니라 관측을 조달하는 쪽이기도 하다(F16). 그 사실이 어댑터 안에 숨으면 안 된다.

### 1.1 1급은 CLI다 — 프로토콜은 어댑터

*"MCP일 필요는 없다. 단순 하네스여도 된다"*(U17)는 이 설계를 뒤집지 않고 **P0의 구성을 정정한다.**

| | 지위 | 근거 |
|---|---|---|
| **CLI (JSON in/out)** | **1급. P0** | 스크립트·CI·git 훅·임의 도구가 쓴다. 프로토콜 SDK의 스펙 변동에 노출되지 않고, [R-01](../00-risks.md#r-01) 아래에서 P0에 SDK를 넣을 이유가 없다 |
| **배치·훅 경로** | CLI와 대등 | U17의 "하네스"가 가리키는 형태. **상주 프로세스가 없다는 것이 P12의 실질** |
| **MCP 서버** | **어댑터. P1** | 질의 로그([F17](F17-synthesis-coverage.md))를 얻는 경로로서 값이 있고 그것이 유일한 고유 이득 |
| **플러그인** | 어댑터의 어댑터 | 전달 경로이지 코어의 정의가 아니다 |

**"하네스"의 두 뜻을 여기서 못 박는다.** U17이 쓴 하네스는 **전달 형태**(상주 서버가 아니라 불러 쓰는 도구)이고, [DESIGN §0.5](../../DESIGN.md)가 거부한 하네스는 **작업을 모는 오케스트레이터**다. 전자는 채택하고 후자는 여전히 거부한다.

**달성 기여**: 제품이 실제로 사용 가능해지는 지점. F01~F05는 이 기능 없이는 라이브러리다.

---

## 2. 단일 진실 — `surface/queries.toml`

```toml
[query."ledger.snapshot"]
summary    = "관측 범위 대장 전체"
args       = []
returns    = "Ledger"
introduced = "F01"
direction  = "consume"

[query."touch"]
summary    = "이 좌표에 결박된 전부"
args       = [{ name = "coord", type = "CoordRef", required = true }]
returns    = "TouchResult"
introduced = "F11"
direction  = "consume"

[query."observation.intake"]
summary    = "환경 의존 절차의 산물을 수용한다"
args       = [{ name = "observation", type = "ObservationSubmission", required = true }]
returns    = "IntakeReceipt"
introduced = "F16"
direction  = "provide"        # ← 방향이 반대. 카탈로그에 표시된다
```

> **⚠ 실물이 이 예시와 다르다 (F06 P0 종료 · 2026-08-14).** 판정은 [게이트](../../gates/F06.md) §2.
>
> · **카탈로그는 이 빌드가 답하는 여섯만 담는다.** §3 의 표 26 은 **로드맵이고 이 파일의
>   상위집합이 아니다** — 스무 개는 코드 쪽 짝이 없고, 짝 없이 적으면 양방향 대조가 그
>   스무 개에 대해 **꺼진다**. 근거 전문은 `corpus/criteria.toml` `[f06].catalog_scope_decision`
> · **`direction` 필드를 안 만들었다.** 이 빌드의 여섯이 전부 `consume` 이고, 값이 하나뿐인
>   필드는 아무것도 안 가른다. **조달 방향은 F16 이 세운다**
> · **카탈로그는 런타임에 안 읽힌다.** `schema/graph.toml` 과 같은 자격이다 —
>   `pal query --list` 는 코드의 선언에서 렌더링되고 카탈로그는 그것을 **대조**한다
> · 생성되는 파생은 **문서 표 하나**다 — `cargo xtask query-doc` → [`docs/query-catalog.md`](../../query-catalog.md)

**여기서 파생되는 것 다섯**: ① Rust 질의 enum + 디스패치 ② JSON 스키마 ③ CLI 서브커맨드 ④ MCP 툴 정의 ⑤ 문서 표.

**규칙 넷**
1. 질의 추가는 `queries.toml` 변경으로만 일어난다. 코드에만 있고 카탈로그에 없으면 **CI 실패**.
2. 모든 질의는 `Envelope<_>`를 반환한다.
3. 모든 실행은 질의 로그에 접근 좌표를 남긴다.
4. `direction = "provide"`인 질의는 조달 방향이며 그 사실이 카탈로그에 표시된다.

---

## 3. 질의 표면 v0 — 도입 시점별 전체 목록

> **이 표는 로드맵이다.** 이 빌드가 답하는 것은 **여섯**이고 그 정본은
> [`surface/queries.toml`](../../../surface/queries.toml) 이다. 여기 있는데 거기 없는 것은
> **아직 없다** — `pal query --list` 가 못 만든 능력을 **기능 번호**로 함께 낸다.

| 질의 | 반환 | 도입 |
|---|---|---|
| `ledger.snapshot` | 대장 전체 — 파일 상태 7값 + 언어 등급 표 | F01 |
| `symbol.resolve` | 좌표 + `identity_grade` | F03 |
| `symbol.contains` | 포함 관계 | F02 |
| `refs.callers` / `refs.callees` | 등급 붙은 엣지 + 후보 집합 | F07 |
| `refs.unresolved` | `UnresolvedRef` 목록 — 사유별 밀도 | F08 |
| **`touch`** | **좌표에 결박된 전부** | **F11** |
| `binding.status` | 결박 5상태 + 반경 + 무엇이 켰는가 | F09 |
| `narrative.unbound` | 미결박 서술물 목록 | F10 |
| `plan.deviation` | `{계획대로, 계획에 없던, 계획했으나 없는}` | F12 |
| `effects.writes` | 경로별 쓰기 집합 | F13 |
| `entrypoints.list` | 진입점 + 인식기 버전 | F14 |
| `boundary.contracts` | `contract` 엣지 + 증거·강도 | F14 |
| `audit.judge` | `{Finding, Residual, OutOfScope}` | F15 |
| `observation.intake` | **조달 방향** — 수용 결과 + 출처 배정 | F16 |
| `coverage.report` | 질의 로그 기반 미조회 하한 | F17 |
| `pack.status` | 팩 지문·규칙 수·파생 라벨 수 | F18 |
| `briefing.prepare` | `Briefing` | F19 |
| `view.model` | `ViewModel` + `elision` | F19 |
| `conformance.delta` | 판정 델타 5성분 | F20 |
| `scope.reductions` | `ScopeReduction` 목록 | F20 |
| `graph.doctor` | 불변식 여덟의 결과 + 표본/전수 표시 | **F22** |
| `graph.schema` | 노드 라벨·엣지 타입·카디널리티·`producer` | **F22** |
| `defect.lineage` | 좌표의 결함 계보 — 도입·해소 `Change` | **F22** |
| `journey.trace` | 여정이 지나는 좌표와 효과 | **F22** |
| `provider.status` | 포트 일곱의 설정 여부 + 마지막 조달 | **F21** |
| `change.log` | `Change` 노드 — 커밋·PR과 그 결박 | **F23** |

**`--base <ref>`는 질의가 아니라 인자다** — `briefing.prepare`·`conformance.delta`·`plan.deviation`이 공통으로 받는다(F23).

---

## 4. 구현

### 4.1 CLI (`pal`)

```
pal ledger                              # ledger.snapshot
pal touch <coord>                       # touch
pal refs callers <coord> [--grade exact,scoped]
pal export --format cypher|graphml|parquet   # 그래프 DB 방향의 만족 형태
pal cache stats|prune
pal serve                               # MCP 서버 (= pal-mcp)
```

- **모든 명령이 `--json`을 지원한다.** 기본은 사람이 읽는 표, `--json`은 `Envelope` 전체 직렬화.
- 사람용 출력에서도 `Envelope`의 요약 한 줄이 **항상** 붙는다.
- `clap` derive + `queries.toml`에서 생성한 서브커맨드 정의.

### 4b (P1) — MCP 서버와 플러그인은 어댑터다

**아래 둘은 P0에 들지 않는다.** 없어도 코어의 어떤 능력도 죽지 않고, P0에서 검사해야 할 것(카탈로그 단일 진실·`Envelope`·호스트 없는 코어)은 CLI만으로 전부 검사된다.

#### 4.2 MCP 서버 (`rmcp`)

공식 Rust SDK `rmcp`, stdio 트랜스포트.

```rust
#[tool_router]
impl PalimpsestServer {
    #[tool(description = "이 코드 좌표에 걸린 결정·라벨·계획·잔여 전부를 반환한다")]
    async fn touch(&self, Parameters(a): Parameters<TouchArgs>) -> Result<CallToolResult, McpError> {
        let env = self.execute(NamedQuery::Touch(a.coord))?;
        Ok(CallToolResult::success(vec![Content::json(&env)?]))
    }
}
```

- 툴 정의는 `queries.toml`에서 생성 — CLI와 MCP가 갈라질 수 없다.
- **질의 로그가 여기서 나온다.** 에이전트가 무엇을 조회했는지가 F17 커버리지 계산의 입력이다.

### 4.3 응답 크기 ([R-11](../00-risks.md#r-11))

에이전트 컨텍스트는 유한하고 길수록 성능이 떨어진다. 정직함(첨부 필수)과 예산이 부딪히는 지점.

| 장치 | 내용 |
|---|---|
| **`Envelope` 2겹** | 기본은 요약 한 줄(`"parsed 1180/1204 · unresolved 2 · L0언어 1"`). 전체 대장은 `ledger.snapshot`으로 뺀다. **첨부 필수는 지키고 부피를 옮긴다**. **실물: `Fold{what, count, unfolded_by}` — [`Elision`]과 다른 필드다.** 옮긴 것과 못 본 것을 한 필드에 뭉개면 둘이 같은 출력이 된다 |
| **능력 목록은 접지 않는다** | `capabilities`는 요약에 남는다(`"능력: 큐레이션·컨설팅 / 미구축: 감사·효과"`). 부피가 작고, **이것을 접으면 소비자가 공백을 "이상 없음"으로 읽는다** |
| **응답 노드 상한** | 기본 500. 초과분은 `elision`에 건수·사유로 |
| **점진 회상** | `touch`는 요약 + 상위 N건. 상세는 후속 질의로. **한 번에 전부 싣지 않는다** |
| **토큰 추정치 노출** | 응답에 대략적 토큰 수를 실어 호스트가 판단할 수 있게. **실물: 잰 것(`serialized_bytes`)과 가정한 것(`bytes_per_token`)을 가른다** — 숫자 하나만 실으면 소비자가 어디까지 믿을지 모른다. **하한이다**(빈틈 없는 JSON 을 잰다) |

#### 4.4 Claude Code 플러그인 (얇은 어댑터)

`.claude-plugin/plugin.json` + MCP 서버 등록 + 슬래시 명령 두엇. **로직이 여기 들어가면 안 된다** — 플러그인은 전달 경로이지 코어의 정의가 아니다.

---

## 5. 이슈와 대응

| 이슈 | 왜 | 대응 | 안 되면 |
|---|---|---|---|
| **응답이 컨텍스트를 먹음** | [R-11](../00-risks.md#r-11) | §4.3 넷 | 상한을 낮추고 `elision`으로 알림 |
| **CLI/MCP 표면 분기** | 두 곳에서 각자 자람 | 카탈로그 단일 진실 + CI 동기 검사 | — |
| **rmcp 스펙 변동** | MCP 스펙이 계속 나감 | `pal-mcp` 크레이트에 격리. 코어는 MCP를 모른다(어휘 금지 검사) | SDK 버전 고정 후 계획적 승급 |
| **호스트 어휘 역류** | 편의를 위해 코어에 `session`·`tool_call`이 들어옴 | **어휘 금지 CI 검사**([스택 §4.2](../00-stack.md#42-어휘-금지-목록--pal-core-소스에-나타나면-ci-실패)) | 허용 목록 증가 자체가 관측 대상 |
| **MCP 서버가 재추출 중일 때 질의** | 긴 작업 | 진행 표시 + 부분 답(`ProjectionFreshness::Rebuilding`) | 타임아웃 후 `stale-projection`으로 답 |
| **인자 오타** | 좌표 문자열 표기 | `symbol.resolve`가 근접 후보를 제안 | — |

---

## 6. 고려한 대안

| 대안 | 기각 이유 |
|---|---|
| **MCP만 제공(CLI 없음)** | 스크립트·CI·다른 도구가 못 쓴다. 호스트 독립성 주장이 검사 불가능해진다 |
| **CLI만 제공** | 에이전트가 주 소비자인데 질의 로그(커버리지 계산의 입력)를 못 얻는다 |
| **REST/gRPC 서버** | 상주 프로세스 + 포트. 설치 비용 제약 위반 |
| **질의 목록을 코드에만** | 무엇이 계약인지 알 수 없고 검사할 수 없다 |
| **자유 질의 언어 노출(Cypher 패스스루)** | 저장 기술이 표면 계약이 되어 교체 불가능해진다. **자유 탐색은 `pal export`로 연다** |
| **`Envelope`를 옵션 플래그로** | 옵션은 비게 된다 |

---

## 7. 검증

- **카탈로그 동기 CI** — `queries.toml` ↔ 코드 ↔ MCP 툴 정의 일치.
- **미구축 능력 표현 검사** — 아직 안 만든 산출을 요구하는 질의가 **빈 결과가 아니라 `NotBuilt`** 를 내는가. `Finding 0`이 새는 경로가 있으면 실패.
- **호스트 없는 코어 검사(CI 상시)** — 호스트·거버넌스 크레이트를 뺀 상태에서 코어~질의 테스트 전건 통과, **관측 0건에서 모든 명명된 질의가 답(공백 포함)을 반환**. 하나라도 실패하면 호스트 독립성이 깨진 것이다.
- **어휘 금지 CI**.
- **응답 크기 실측** — 실제 MCP 세션에서 질의별 토큰 수 분포.
- **골든 JSON** — 질의별 응답 스키마 스냅샷.

---

## 8. 완료 체크리스트

**P0 — 카탈로그와 CLI** — **닫혔다** (2026-08-14 · [게이트](../../gates/F06.md))

- [x] `surface/queries.toml` + **대조**(생성이 아니다 — 생성하면 대조가 항등식이 된다).
      `cargo xtask check` 의 「카탈로그 정합」이 **방향 넷을 각각 자기 루프로** 돈다
- [x] `pal` CLI — **셋이 아니라 여섯** + `--json`. F05 가 셋을 더했다
- [x] `pal export` 골격 — `--format cypher`. **라벨이 `schema/graph.toml` 에서 온다**
- [x] `Envelope` 2겹(`Fold`) + 응답 상한(F05) + 토큰 추정 — 성분이 여섯에서 **아홉**이 됐다
- [x] 카탈로그 동기 CI · 호스트 없는 코어 CI · 어휘 금지 CI — **셋 다**
- [x] **비대화 경로 검사** — 파이프·종료 코드 두 갈래·`--json` 만으로 전 질의 도달

**P0 가 세운 것 하나 더** (F05 §6 이 넘겼다):

- [x] 2층의 **읽기 전용 경로** — `--read-only`. ⚠ **얻은 것은 읽기 여럿의 공존이고
      쓰기와의 공존이 아니다**([게이트](../../gates/F06.md) §4)

**P1 — 어댑터 (§4b)** — **닫혔다** (2026-08-17 · [게이트](../../gates/F06b.md))

- [x] `pal serve` MCP 서버(rmcp 3.1.2) + 툴 등록. ⚠ **툴은 `#[tool_router]` 로 안
      세운다** — §4.2 의 스케치는 질의마다 함수 하나이고 그것이 곧 두 번째 목록이다.
      `QueryName::ALL` 을 순회한다([ADR-0024](../../adr/0024-an-adapter-that-can-diverge-is-a-second-core.md))
- [x] Claude Code 플러그인 껍데기 — **`surface/claude-plugin/`**. 저장소 루트가 아닌
      이유는 그 디렉터리의 README 에
- [x] 실 MCP 세션 응답 크기 측정 기록 — `ledger.snapshot` **1,199 B** ·
      `binding.status` **1,272 B**(실제로 나간 바이트). 봉투의 신고는 각각 1,194·1,267 B
      이고 **차이 5 B 는 봉투가 자기 자신을 안 세기 때문**이다 — `serialized_bytes` 는 하한
- [x] **어댑터 부재 통과 검사** — `pal-mcp` 를 뺀 빌드에서 **381 통과**, 그 빌드의
      `--help` 에 `serve` **없음**. CI 스텝 둘이 상시로 잰다.
      ⚠ **워크스페이스 전체가 아니라 `-p pal-cli` 다**([게이트](../../gates/F06b.md) §판정-나-⑥)
