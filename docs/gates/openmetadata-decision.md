# 게이트 — OpenMetadata 를 적용할 것인가

**회차** `2026-09-05-openmetadata-decision` · **이슈** [#104](https://github.com/hskim-ecoletree/palimpsest/issues/104)
**착수** `930a295` · **판정일** 2026-09-05

> 잠긴 의도 [`intent.md`](../../.palimpsest/rounds/2026-09-05-openmetadata-decision/intent.md) ·
> 종료 보고 [`report.md`](../../.palimpsest/rounds/2026-09-05-openmetadata-decision/report.md)

---

## 합격선

측정 전에 등록했다 — 잠긴 의도의 완수 조건 17 개(`A1`·`A2`·`B1`~`B5`·`C1`~`C3`·`D1`~`D3`·`E1`·`E2`·`F1`·`F2`).
결정론으로 판정되는 것은 `D3`(스키마 파일 다이제스트) · `E1`(표준 표 파싱) · `E2`(`cargo xtask check`) 셋이다.

**RED 관측** — 착수 시점에 이 저장소는 OpenMetadata 를 한 번도 안 재 봤다. 이슈 #104 가
긴장 다섯을 등록했고 그 중 어느 것에도 답이 없었다.

**음성 대조 넷을 등록했다.**

| 어디에 | 무엇이 고장이면 드러나나 |
|---|---|
| `A2` | 선정 규칙이 탈락을 0 건 내면 그 규칙은 사후 정당화다 |
| `C1` | 낡은 2 층으로 export 를 돌리면 같은 커밋에서 다른 수가 난다 |
| `C3` (P 팔) | 규격대로 만든 인스턴스가 실패하면 검증기나 스키마 배치가 고장난 것이다 |
| `C3` (N 팔) | 일부러 어긋낸 인스턴스가 통과하면 그 검증기는 아무것도 안 잰다 |

**퇴로** — ① 검증이 안 서면 `C3` 를 「원리상 못 잰 것」으로 적는다(「돌렸다」로 위장하지 않는다)
② 탈락이 원리상 0 이면 K 를 낮추지 않고 모집단을 넓힌다 ③ 「안 쓴다」로 나도 ADR 을 낸다.

---

## 판정

| 판정 | 조건 |
|---|---|
| 통과 | A1 A2 C1 C2 C3 D1 D2 D3 E1 E2 F1 F2 |
| 반증 | B2 |
| 대조불가 | B3 B4 |
| 미측정 | B1 B5 |

**검산** — 통과 12 · 반증 1 · 대조불가 2 · 미측정 2 = 17

### 획득 — **표준 표와 분리한다**

표준 표에 열을 더하면 `record.py gate` 가 그것을 표준 표로 안 읽는다(사전부검 R2 실측).

| 조건 | 획득 | 왜 |
|---|---|---|
| A1 · A2 | 조회 | `git log` 로 커밋 순서 확인 · 모집단 열다섯의 정의 파일 조회 |
| B2 | **반증** | 판정 표기를 표준 표와 분리하는 데는 성공했으나, **정 초안이 열다섯 중 다섯 행에 획득 표기를 안 붙였다**(Dublin Core·LSIF·CodeQL·ActivityStreams·Matrix). 그 다섯이 산문뿐이었고, 그 중 CodeQL 에서 실제 오판이 나왔다 |
| B3 | **대조 불가** | ① 을 서버 없이 쟀다 — 소유자가 축소를 승인했다(2026-09-05). 공식 문서의 선언값이지 실행 관측이 아니다 |
| B4 | **대조 불가** | 이 저장소의 `REFERENCES` 실측이 **3 건**이다(`pal export` · `built_for_this_snapshot=true`). 엣지 정밀도를 3 건 위에서 판정할 수 없다 |
| B1 · B5 | **미측정** | 정반합이 후보 선정 자체를 무너뜨렸다 — 확정된 후보 집합이 없으므로 「후보마다」·「축마다」가 원리상 못 선다. 아래 「범위 밖」이 그 경계를 적는다 |
| C1 | 조회 | 2 층을 새로 세운 뒤 `pal export --format cypher --json` · 봉투가 `built_for_this_snapshot: true` |
| C2 | 조회 | `missing` 이 아니라 정본 파일을 셌다 — `.palimpsest/intent/bindings.jsonl` 에 결박 18 행 |
| C3 | 조회 | 아래 `## 효과` |
| D1 | 조회 | 아래 「비목표 두 행」 |
| D2 | 조회 | ADR 에 매핑 표가 없다. 매핑은 이 문서에 산다 |
| D3 | 조회 | `sha256(schema/graph.toml)` 이 착수 시점과 같다 |
| E1 · E2 | 조회 | 파일 이름이 소문자 · `record.py gate` 가 `표준표: true` · `cargo xtask check` 전량 통과 |
| F1 · F2 | 조회 | 판정 뒤에 처분했다 · 종료 보고에 §10 의 네 이름이 없다 |

---

## 무엇을 재서 무엇이 나왔나

### ① 설치·운영 비용 — `P12` 와 충돌한다 · **획득: 추정**

공식 문서가 로컬 배포에 **Docker 메모리 6 GiB · 4 vCPU** 를 요구하고, 구성요소는
MySQL/PostgreSQL · Elasticsearch · Airflow · 서버 · 인제스션이다. 임베디드·라이브러리 전용
모드는 문서에 없다.

`P12` 는 *"설치·운영 비용이 1급 제약이다. palimpsest 는 사용자의 머신에서 돌고 그 비용은
사용자가 낸다"* 이다. **상시 서버 다섯을 사용자에게 지우는 것은 이 원리와 정면으로 부딪힌다.**

⚠ **서버를 안 세웠으므로 이것은 추정이다.** 다만 6 GiB 라는 선언값은 서버를 세워도 안
작아지므로, 실측이 이 판정을 뒤집을 경로는 「문서가 과대 선언했다」 하나뿐이다.

### ②③④⑤ — 스키마 실측이 한꺼번에 답했다

`pal export` 로 뽑은 **실제 노드**를 OpenMetadata 의 JSON Schema 에 넣어 돌렸다.
전문은 `.palimpsest/rounds/2026-09-05-openmetadata-decision/effect/output.txt`.

| 잰 것 | 결과 | 어느 긴장 |
|---|---|---|
| 엣지 어휘 | palimpsest 엣지 8 종 중 **1 종만** OpenMetadata 의 닫힌 `relationshipType` enum(27 종)에 있다 — `FOLLOWS` → `follows` | ⑤ |
| 출처 축 | OpenMetadata 는 `Manual`·`Inferred`·`Imported`·`AiSuggested`. palimpsest 의 `extracted`·`inferred`·`asserted`·`observed` 와 **겹치는 것은 `Inferred` 하나** | ③ |
| 파일 속성 | `contextFile` 이 `additionalProperties: false` 라 `language`·`grade` 를 **버려야** 들어간다 | ④ |
| 코드 심볼 | `entity/data/` 어디에도 **담을 자리가 없다.** `contextFile` 에 `region`·`line`·`range` 류 속성이 없다 | ② ⑤ |
| 공리 엔티티 | `ontologyAxiom` 은 required 11 개고 그중 `glossary` 가 필수 — 공리가 Glossary 에 종속된다 | ③ |

**까닭은 단순하다.** OpenMetadata 는 데이터베이스 테이블·대시보드·파이프라인을 담는
카탈로그이고, palimpsest 는 **코드 심볼과 그것에 결박된 사람이 쓴 글**을 담는다.
2.0.1 이 `ontologyAxiom`(OWL 공리 여섯)·`conversationSource`·`contextFile` 을 들여 대상을
넓혔지만, **코드 좌표를 담는 축은 그 확장에 없다.**

### 축 셋의 판정

| 축 | 판정 | 근거 |
|---|---|---|
| **1. 런타임으로 채택** | **채택하지 않는다** | ① 이 `P12` 와 충돌한다 |
| **2. 어휘만 차용** | **채택하지 않는다** | 엣지 1/8 · 출처 1/4 · 속성 손실 · 코드 심볼 자리 없음. 차용하면 우리 데이터의 대부분을 버리거나 확장으로 메워야 하고, 확장으로 메운 어휘는 **더 이상 그 표준이 아니다** |
| **3. 내보내기 대상으로만** | **채택하지 않는다** | 기술적으로 가능하다(`pal export` 가 이미 선다). 그러나 **읽는 자가 0 이다** — [ADR-0012](../adr/0012-a-single-truth-file-declares-only-what-has-a-counterpart-in-code.md) 가 *"짝이 없으면 적지 않는다"* 로 금지한 자리이고, 그 병이 지금 #102 가 다루는 그것이다. 소비자가 생기면 그때 연다 |

**넷째 축은 조사가 내지 않았다.**

### 비목표 두 행 — **뒤집지 않는다** (`D1`)

[00-goals.md](../plan/00-goals.md) §4 의 두 행:

- *"범용 지식 그래프 — 노드 타입은 코드 좌표에 결박 가능한 것과, 그것에 결박되는 것뿐이다"*
- *"자기 자신의 온톨로지를 만드는 도구"*

**이 판정은 그 둘을 뒤집지 않는다.** 오히려 첫 행이 이 판정의 근거다 — OpenMetadata 를
못 쓰는 까닭이 *"코드 좌표에 결박 가능한 것"* 이라는 그 제약과 안 맞기 때문이다.
소유자가 말한 문서·채팅 온톨로지는 **로드맵이지 확정 설계가 아니고**(원문: *"확정된 설계라기보다
로드맵으로 생각만"*), 그것을 실제로 담을 때 이 두 행을 뒤집을지는 그 회차가 판정한다.

### 재고와의 관계

- **#102**(소비자 0 인 선언) — 축 3 을 접은 근거가 그 이슈와 같은 자리다. 처분은 안 바꾼다.
- **#69**(문서 간 결박) — OpenMetadata 가 답했을 물음이지만 스키마가 안 맞아 답이 안 된다. 처분은 안 바꾼다.

---

## 효과

**테스트도 CI 도 아닌 것이 이 회차의 판정 명제를 돌린 출력이다.** 이 회차가 쓴 검증
스크립트가 palimpsest 의 실제 노드를 OpenMetadata JSON Schema 에 넣었다.

```
palimpsest 실측: File 133 · Symbol 3018 · 결박 18

팔   통과    이름
────────────────────────────────────────────────────────────────────────
P   ○     최소 contextFile
P   ○     최소 entityRelationship
T   ×     File 노드를 contextFile 로 (path=corpus/tasks/f03-normalize-seeds.ts)
          └ Additional properties are not allowed ('grade', 'language' were unexpected)
T   ○     File 노드에서 language·grade 를 버리고
T   ○     Symbol 노드를 contextFile 로 (id=00227a4ddc51…)
T   ×     결박을 ontologyAxiom 으로 (id=262fef9e5483d7bf)
          └ 'displayName' is a required property
T   ×     결박을 ontologyAxiom 으로 · required 11 개를 전부 채운다
          └ 'Derived' is not one of ['Manual', 'Inferred', 'Imported', 'AiSuggested']
T   ×     REFERENCES 엣지를 entityRelationship 으로
          └ 'REFERENCES' is not one of ['contains', 'createdBy', 'repliedTo', …]
N   ×     contextFile 에서 required `name` 을 뺀다
N   ×     contextFile 에 규격 밖 속성 `palimpsestGrade` 를 넣는다
N   ×     entityRelationship 에서 required `relationshipType` 을 뺀다

── 음성 대조 ──
P (통과해야 한다): 2/2 통과 → 검증기가 선다
N (실패해야 한다): 3/3 실패 → 검증기가 실제로 잰다

T (판정 대상): 2/6 통과

── 엣지 어휘 대조 — palimpsest 엣지 8 vs OpenMetadata relationshipType enum 27 ──
  BOUND_TO → (없다) · AUTHORED_BY → (없다) · TOUCHES → (없다) · FOLLOWS → follows
  MANIFESTS_AT → (없다) · INTRODUCED_BY → (없다) · RESOLVED_BY → (없다) · REFERENCES → (없다)

정확히 대응하는 엣지: 1/8
```

★ **음성 대조가 실제로 발화했다.** 첫 실행에서 P 가 1/2 였다 — 내가 쓴 양성 대조가
`RELATED_TO` 를 썼는데 enum 은 `relatedTo` 다. **장치가 내 오기를 잡았고**, 고치면서
엣지 여덟을 enum 에 전량 대조하는 축을 더했다(재는 양이 늘었으므로 정정이다).

★ **통과한 판정 대상 둘은 전부 「속성을 버리고 이름만 남긴 것」이다.** 그것이 이 판정의
핵심 관측이다 — 들어가긴 하는데, 들어가려면 palimpsest 가 담는 것을 버려야 한다.

---

## 범위 밖

- **로드맵의 설계·구체화** — 문서·채팅 수집 파이프라인을 어떻게 만들지. 소유자가 원문에서 명시적으로 뺐다
- **`schema/graph.toml` 변경과 적용 구현** — 판정까지가 이 회차다
- **OpenMetadata 서버 실측** — 소유자가 축소를 승인했다. ① 은 문서 사실 기반 추정이다
- ★ **소프트웨어 카탈로그 층** — **이 판정은 데이터 카탈로그·어휘 표준 안에서만 유효하다.**
  정반합이 잡았다: 모집단 열다섯의 카탈로그 다섯이 전부 *데이터* 카탈로그였고, palimpsest 가
  실제로 서 있는 층(소프트웨어 산출물 + 코드 좌표 + 사람이 쓴 글)의 표본이 **0** 이었다.
  **SARIF**(`graph`·`node`·`edge` + `physicalLocation`·`region` + `message` + `logicalLocation`)와
  **Backstage** 는 엄격한 읽기로도 후보 자격을 갖는데 비교하지 않았다. 그 비교는 다음 이슈가 진다
- **§9 「게이트는 그 회차가 건드린 코드보다 작다」** — 이 회차는 `crates/**`·`xtask/**` 를
  안 바꾸므로 비교 대상이 0 줄이고 원리상 못 잰다
- **`crates/pal-cli/src/export.rs` 머리 주석의 틀린 수**(노드 여덟 · 실물 10) — 사전부검이
  찾았으나 이 회차가 안 고친다
- **선정 규칙의 음성 대조에 누락 탐지가 없다** — 등록된 넷(ⓐ~ⓓ)이 전부 *더한 것*만 잡고
  *빠뜨린 것*은 못 잡는다. `A2` 는 「탈락이 있었다」만 보증하고 「모집단이 옳았다」는 원리상 보증 못 한다
