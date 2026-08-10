# 선행 palimpsest 구현체 관측 — 신선도 앵커와 CodeQL 재유예

> **이 문서의 지위**
>
> - **성격**: **1차 관측**. 선행 palimpsest 구현체(`~/dev/projects/palimpsest-backup`)의 실물 코드와 설계 문서를 직접 읽고 옮긴 것이며, 추론과 관측을 구별해 적는다.
> - **요구사항이 아니다.** [research/README 33장](README.md)의 규율이 그대로 걸린다 — 증거이지 계약이 아니다. 다만 **승계 정책(U8: 개념만 승계, 구현·계약 비승계)**이 여기에도 걸리므로, 이 문서가 하는 일은 **무엇을 배우는가**이지 무엇을 옮겨오는가가 아니다.
> - **이 저장소 밖의 자료다.** 백업은 별도 저장소이고 현행 설계의 어떤 결정도 그 코드에 의존하지 않는다. 인용한 파일 좌표는 그 저장소의 것이다.
> - **왜 지금 읽었나**: 소유자가 *"레거시 백업을 보면 대부분의 코드나 근거에 커밋 해시를 붙여서 신선도를 확인할 수 있었다 — 신선도 유지라기보다 낡음을 드러내려고"*라고 지목했다. 그 장치를 확인하러 갔다가 **현행 설계를 직접 반증하는 것 둘**을 함께 발견했다(§2·§3).
> - **관측 일자**: 2026-08-11.

---

## 1. 신선도 앵커 — 두 개의 시간 축을 가른다

소유자가 지목한 장치의 실물은 **`source_commit`과 `code_bound_at` 두 필드**이고, **둘은 다른 일을 한다.**

| | `source_commit` | `code_bound_at` |
|---|---|---|
| 값 | 커밋 SHA | 대상 코드 노드의 `committed_at`(커밋 시각) |
| 누가 채우나 | **생성자가 신고한다** — 어느 커밋에 대해 만들었는가 | **로더가 계산한다.** 생성자는 이 필드를 가질 수 없다 |
| 하는 일 | **파생물 id의 성분** | **낡음 판정의 앵커** |
| 낡으면 | 낡지 않는다(불변 사실) | 대상의 현재 `committed_at`과 달라지면 `stale` |

### 1.1 관측 — 앵커는 필드가 아니다

`ir.py`의 네 개 파생 타입(`Summary`·`Risk`·`InferredRelation`·`DesignDecision`) 전부에 **같은 주석이 반복된다.**

> *"`code_bound_at` is deliberately NOT a field here: freshness must follow the code, not the generator's wall-clock, so the loader binds it to the resolved target node's `committed_at` (a git-less external summary has no meaningful commit time of its own)."* — `src/palimpsest/ir.py:567-570`

즉 **생성자는 자기 산출물이 얼마나 신선한지 신고할 수 없다.** 신선도는 그 산출물이 가리키는 **코드에서 온다.** 외부 LLM이 만든 요약에는 의미 있는 커밋 시각이 없기 때문이라는 것이 명시된 이유다.

### 1.2 관측 — detect-only

```python
def _stale(code_bound_at, target_committed_at) -> bool:
    """#4 detect-only freshness flag. ... Pure comparison — no LLM, no regeneration."""
    if code_bound_at is None or target_committed_at is None:
        return False
    return target_committed_at != code_bound_at
```
— `src/palimpsest/recall/graphrag.py:194-202`

세 가지가 한 함수에 있다.

1. **탐지만 한다.** 낡음을 발견해도 재생성하지 않고 LLM을 부르지 않는다.
2. **순수 비교다.** 임계도 휴리스틱도 없다.
3. **판정 불가일 때 낡음을 주장하지 않는다** — 둘 중 하나가 없으면 `False`.

세 번째가 §7의 관측 R16으로 이어진다. 방향은 옳으나 **`False`와 "판정 불가"를 구별하지 않았다.**

### 1.3 관측 — 앵커 선택이 결정적이다

파생물이 여러 코드 노드를 가리킬 때 어느 것이 앵커인가가 정해져 있다.

> *"Risk binds to `sorted(flags)[0]`; a decision binds to its first *code* DECIDES target (`decision:`-namespaced targets are other decisions, not code, so skipped)."* — `graphrag.py:302-308`

**앵커가 하나다.** 감시 집합이 아니라 대표 원소 하나이며, 그래서 판정이 상수 비용이다. 대신 앵커가 아닌 대상이 변한 것은 **보이지 않는다.**

### 1.4 관측 — 벽시계를 payload에서 뺀다

CodeQL 스파이크 문서가 같은 원리를 다른 각도에서 적는다.

> *"`created_at` 벽시계를 물질화 payload에 넣으면 같은 HEAD 재스캔이 비-byte-identical. 결정론 출처(`source_commit` 커밋시각) 파생 또는 payload에서 제외. (`code_bound_at`은 git-파생 결정론이라 무관.)"* — `docs/design/codeql-head-risk-spike-feasibility.md` §3 MEDIUM

괄호가 핵심이다 — **git에서 파생된 시각은 재실행에 흔들리지 않는다.** 벽시계는 흔들린다.

---

## 2. 파생물 id에 출처가 없으면 노드가 붕괴한다 — **선행 구현의 자기 반증**

이것은 소유자가 지목한 것이 아니라 그 옆에서 발견한 것이고, **현행 설계를 직접 겨눈다.**

### 2.1 관측

CodeQL 조달을 설계하며 착수 시 잠정 선택은 *"`edge_kind='inferred'` 유지 + `extracted_by='codeql'` provenance"*였다. pre-mortem이 그것을 **unsound로 반증했다.**

> *"`risk_id`는 `title + source_commit + sorted(flags)`만 해시하고 provenance를 제외한다. `_RISK_MERGE`는 MERGE-on-id + blind `SET`이라, 같은 `(title, source_commit, flags)`의 CodeQL Risk와 LLM Risk가 **동일 노드로 붕괴하고 last-writer-wins로 `generator`/`model`/`extracted_by`가 덮어써진다**. 즉 마커가 정체성 load-bearing이 아니다."* — 스파이크 문서 §2

그리고 두 번째 관측이 붙는다 — **마커가 write-only였다.** 회상 채널이 `edge_kind`는 읽고 `extracted_by`는 읽지 않아, 구분이 저장은 되되 아무 데도 나타나지 않았다.

**정정된 방향**: provenance 속성이 아니라 **subtype 또는 edge-level discriminator**. 최소안으로 `extracted_by`를 id 키에 편입.

### 2.2 이 관측이 말하는 것

> **출처를 속성으로만 두면 그것은 정체성이 아니고, 정체성이 아닌 구분은 병합에서 조용히 소멸한다.**

"출처 필드는 불변"이라는 규칙은 **필드를 고쳐 쓰는 것**을 막지만, **두 노드가 하나로 병합되는 것**은 막지 못한다. 후자에서는 아무도 필드를 고치지 않았는데 값이 바뀐다.

---

## 3. CodeQL 조달은 이미 한 번 시도됐고 코드 0줄로 재유예됐다

### 3.1 관측 — 결론과 그 이유

> *"happy-path 코드는 vacuously green: 합성 단일 finding fixture는 §3의 hard case(id 충돌·위치 정밀도·다중 흐름·resolver)를 전부 우회하므로, ac-2/ac-3가 green이어도 실효 증거가 못 된다. → 코드 미작성이 정직한 선택."* — 스파이크 문서 §1

**착수 게이트의 pre-mortem이 코드 한 줄 없이 실현가능성을 특성화했고, 그 결과가 "아직 아니다"였다.** 구조(격리 생산자 → git 선물질화 → 멱등 로더)는 성립한다고 판정했으나 §3의 선행조건이 먼저다.

### 3.2 관측 — 여덟 개의 선행조건

| 등급 | 조건 | 무엇이 걸리나 |
|---|---|---|
| HIGH | **SARIF 비결정성** — 내장 timestamp, 도구/쿼리팩 버전, 절대 `srcRoot`, 멀티스레드 result 순서 | 정규화 없이는 재조달 대조가 **거짓 불일치**를 낸다 |
| HIGH | **실패 vs clean 구별** — 생산자가 exit-code를 검사하지 않으면 빌드 실패가 "0 findings"로 위장 | *"보안 니치 최악의 false-clean"* |
| HIGH | **`file:line` → 노드 id resolver** — 엔진은 위치로 말하고 그래프는 심볼로 말한다. 이 어댑터는 **격리 생산자 밖(caller-side)**에 있어야 한다 | 계획이 이것을 **누락했다.** 그리고 resolver 버그와 정당한 미-grounding을 구별하는 fixture가 필요하다 |
| HIGH | **왕복 필드 부재** — 구분자가 직렬화 왕복에서 조용히 소실 | §2와 같은 실패의 다른 얼굴 |
| MEDIUM | **다중 흐름 붕괴** — 같은 sink·같은 룰의 서로 다른 taint 흐름이 같은 id로 붕괴 | **silent undercount = 보안 false-negative** |
| MEDIUM | **위치 정밀도** — SARIF는 expression/statement 단위, 그래프 최소 단위는 메서드. 감싸는 메서드가 없으면 드롭되고, 파일로 올리면 taint 정밀도(니치의 존재이유)를 잃는다 | 매핑 규칙을 확정해야 한다 |
| MEDIUM | **비트랜잭션 적재** — 노드와 엣지가 별도 커밋이라 둘째 실패 시 floating 노드 | 선재 결함 |
| MEDIUM | **time-clock** — §1.4 |

### 3.3 관측 — 주경로/보조의 분업이 같은 결론에 도달했다

> *"CodeQL 정확도는 빌드/컴파일 추적에 의존한다 — 옛 커밋은 빌드가 안 되고, 빌드 환경 의존이라 git만으로 재구축이 안 닫히며, per-commit DB 빌드는 비현실적. **결정 — 정밀도의 주경로는 palimpsest가 소유하는 build-less tree-sitter spine이고, CodeQL은 선택적 보조 overlay다.**"* — 재검토 문서 §6.2

그리고 **HEAD-only로 좁혔다.** 소유자의 soft 선호가 "직접 실행"이었는데도 전이력 균일성과의 충돌 때문에 보조로 내려갔다.

---

## 4. 두 번째 축 — 코드 신선도와 계보 통용성은 직교한다

```python
# Decision-lineage freshness (2nd axis) — decisions channel only.
# The entry is still SURFACED when superseded (전이력 보존);
# ``live`` is the current-currency judgment, derived as valid_to IS NULL.
```
— `graphrag.py:342-350`

- **축 1** `stale` — 결박된 코드가 그 뒤 바뀌었는가.
- **축 2** `live` — 이 결정이 다른 결정으로 대체되지 않았는가(`valid_to IS NULL`).

**대체된 결정도 여전히 표면화된다.** 숨기지 않고 `live=false`를 단다. 두 축이 독립이므로 `stale ∧ live`(유효한데 코드가 변함)와 `¬stale ∧ ¬live`(코드는 그대로인데 대체됨)가 둘 다 표현된다.

---

## 5. git 파생 사실이 결정론적으로 선 실물

`extract/provenance.py`가 `git show -s --format=%H%x1f%an <%ae>%x1f%cI`로 **한 번의 호출에서 세 필드**(sha·author·committed_at)를 읽는다 — *"so the three fields come from a single pinned SHA."*

그리고 `changed_paths`가 커밋 → 파일 엣지(`MODIFIES`)를 만드는데, **플래그 선택마다 무엇이 감춰지는지가 주석에 적혀 있다.**

| 플래그 | 없으면 |
|---|---|
| `--root` | 최초 커밋이 아무것도 보고하지 않는다 — *"silent under-capture"* |
| `--first-parent` | 머지 커밋이 자기가 합친 변경을 다시 세어 **churn/co-change를 이중 계상**한다. 대신 evil-merge가 어느 커밋에도 안 붙는 것을 **인정된 공백으로 적었다** |
| `--no-renames` | 레코드가 단일 경로가 아니게 된다 |

**관측**: 변경 결합도·churn 같은 이력 파생 신호가 실제로 계산됐고, 그 계산의 **은폐 경로가 코드 주석에 명시적으로 적혀 있다.**

---

## 6. 이 관측이 현재 설계에 대해 말하는 것 — R13~R17

| # | 관측 | 현행 설계와의 관계 |
|---|---|---|
| **R13** | **신선도 앵커를 생산자가 신고하지 못하게 한 구현이 실재한다.** `code_bound_at`은 필드가 아니라 로더가 대상 코드에서 가져온다 | 현행 [DESIGN §6](../DESIGN.md)의 `watch_snapshot`은 기계가 채우게 되어 있으나 **"생산자가 신고할 수 없다"가 규칙으로 적힌 적이 없다.** §9.3이 커버리지에 대해 세운 원리(*신고받지 않고 계산한다*)가 결박 층에는 적용되지 않았다 |
| **R14** | **출처를 속성으로만 두면 병합에서 소멸한다.** 선행 구현이 자기 id 산출 규칙 때문에 CodeQL/LLM 판정 구분을 unsound로 판정했다 | 현행 설계에 **파생 노드의 id 산출 규칙이 없다.** [DESIGN §7.5](../DESIGN.md)가 "엔진 간 불일치"를 다섯째 몫으로 세웠는데, id 규칙 없이는 **두 엔진의 판정이 애초에 한 노드로 붕괴해 불일치가 관측되지 않는다** |
| **R15** | **CodeQL 조달은 한 번 시도되어 코드 0줄로 재유예됐고, 여덟 개의 선행조건이 남았다** | [F16](../plan/features/F16-observation-intake.md)의 P1 승격이 이 목록을 모른 채 세워졌다. 특히 **실패 vs clean 구별**과 **resolver의 위치**(격리 생산자 밖)는 현행 계획에 없다 |
| **R16** | **낡음 판정 불가를 "낡지 않음"으로 접었다** — `code_bound_at`이나 대상 시각이 없으면 `stale=False` | 방향은 현행 설계와 같다(모르는 것을 낡았다고 하지 않는다). 그러나 **`False`와 "판정 불가"를 구별하지 않은 것은 [목표 §3.1](../plan/00-goals.md)의 정면 위반**이다. 현행 결박 5상태에도 그 자리가 없다 |
| **R17** | **git 파생 사실이 결정론적으로 서고, 그 계산의 은폐 경로가 주석에 명시됐다** | [DESIGN §3](../DESIGN.md)의 `ReproInput=History` 배정을 지지하는 실물. 그리고 **플래그 하나가 조용한 누락을 만든다**는 것이 `changed_paths` 주석의 실측이다 |

---

## 7. 이 관측이 주장하지 않는 것

1. **구현을 승계하자는 것이 아니다.** U8(개념만 승계)이 그대로 걸린다. `code_bound_at`을 그대로 가져오면 현행 설계의 `body_digest`(포매팅 커밋에 stale이 안 켜짐)보다 **약하다** — 커밋 시각 비교는 포매팅 커밋에도 켜진다.
2. **레거시가 옳았다는 것이 아니다.** §2는 선행 구현의 **자기 반증**이고 §1.2의 세 번째는 **불완전한 처리**다. 배울 것은 장치가 아니라 **그 장치가 무엇에 걸려 넘어졌는가**다.
3. **CodeQL을 채택하거나 기각하자는 것이 아니다.** R15는 조달의 **비용 목록**이지 판정이 아니다. 판정 자리는 F16의 게이트다.
4. **n=1이다.** 한 사람이 만든 한 구현체이며, 여기서 나온 실패가 일반적이라는 증거가 없다. 다만 §2·§3은 **그 구현체 자신의 사후 분석**이므로 자기보고 편향이 반대 방향으로 걸린다 — 자기 설계를 unsound로 판정한 기록이다.
5. **전수로 읽지 않았다.** `src/palimpsest`의 40여 파일 중 `ir.py`·`recall/graphrag.py`·`kg/summary.py`·`kg/risk.py`·`extract/provenance.py`와 `docs/design/`의 세 문서를 읽었다. 백필·커뮤니티·브랜치 스코프 계열은 읽지 않았다.

---

## 8. 참조한 좌표 (저장소 밖)

`~/dev/projects/palimpsest-backup` —
`src/palimpsest/ir.py`(파생 타입 넷의 `code_bound_at` 주석) ·
`src/palimpsest/recall/graphrag.py`(`_stale`·`_bound_anchor`·`_entity_channel`의 2축) ·
`src/palimpsest/kg/summary.py`(`summary_id`·`_SUMMARY_MERGE`) ·
`src/palimpsest/extract/provenance.py`(`read_provenance`·`changed_paths`) ·
`docs/design/codeql-head-risk-spike-feasibility.md`(재유예 판정과 여덟 게이트) ·
`docs/design/palimpsest-generative-curator-reexamination.md`(§1 drift map · §5 확정 설계 · §6.2 CodeQL 분업).
