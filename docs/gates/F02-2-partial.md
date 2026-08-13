# F02-2 게이트 — `partial` 회복을 1급으로 · 판정 기록

**판정: 통과 (2026-08-13).**

**깨진 파일에서 부분 결과가 나온다. 그리고 못 읽은 자리가 산출에 남는다.**

그런데 이 조각이 실제로 잰 것은 그것만이 아니다 — **등록된 `partial` 30 건 중 27 이
「부분 결과」가 아니었다.** 그 27 이 낸 선언은 전부 합쳐 **1 개**이고 26 은 0 개다.
`partial`(*"일부는 읽었다"*)이 그 파일들에 대해 **거짓 문장**이었다.

| | 등록값 | 관측 |
|---|---|---|
| ① 깨진 영역 밖에서 잃은 선언 | **0** | **통과** — 변이 셋 · 밖의 선언 6·11·6 · 잃은 것 0 |
| ② `ERROR` 안쪽 순회로 건지는 선언 | 실측하고 기록 | **통과** — 두 코퍼스 **전수 1,618 파일 · 0 건**. 세우지 않고 F02 §4 를 정정 |
| ③ 회복 지점이 개수가 아니라 자리 | true | **통과** — `RecoverySite{kind, span}` · 소스 순서 · 너비 0 인 `error` 0 개 |
| ④ 임계 강등이 선다 | true | **통과** — `Unsupported{GrammarDefeated}` |
| ④ 임계값이 자리표시임을 명시 | true | **통과** — `pal-core::budget::PROVISIONAL_ERROR_RATIO_PERCENT` |
| ④ 실물에서 걸린 건수 | 기록 | **통과** — **27 / 997** (아래 전수 목록) |
| ⑤ 반드시 `partial` 을 만드는 변이 셋 | 3/3 | **통과** — 3/3 |
| ⑤ 안 깨뜨린 파일 — 코퍼스 실물 전수 | 0 이동 | **통과** — 워킹트리 997 전수 · **깨뜨린 것 말고 0** |
| ⑤ 변이 대상 부재 시 멈춤 | true | **통과** — 여섯 다 고정 SHA 의 실재 경로·실재 식별자 |
| ⑤ `partial` 개수를 합격선으로 삼지 않음 | true | **통과** — 기준선은 그 회차의 무변이 실행이다 |
| ⑥ Kotlin 문법 처분을 판단하고 근거를 적음 | true | **통과** — 아래 §5. **후보 넷을 실측으로 댔다** |

이슈 [#47](https://github.com/hskim-ecoletree/palimpsest/issues/47) (부모 [#5](https://github.com/hskim-ecoletree/palimpsest/issues/5)) ·
합격선 정본 [`corpus/criteria.toml`](../../corpus/criteria.toml) `[f02.2]` ·
대조 [`scripts/f02-2-verify.py`](../../scripts/f02-2-verify.py)

**합격선을 정한 것도 판정하는 것도 에이전트다** — [R-18](../plan/00-risks.md#r-18)은 닫히지
않는다. 줄인 것: 합격선이 **코드 이전에**(`f22017e`·`a5a5b13`) 등록됐고, 구조적 변경과
동작적 변경을 갈랐으며(`ce60f45` → `f587ccd`), 대조가 **자기 자신을** 음성 대조한다(§2).

---

## 1. 가장 무거운 관측 — **`partial` 30 은 부분 결과가 아니었다**

`[f02.2.oracle].corpus_has_no_broken_kotlin` 이 이미 절반을 적었다: *"30 건 전부 유효한
Kotlin 이고 못 읽는 것은 우리 문법이다."* 이 조각이 나머지 절반을 쟀다 — **그 파일들에서
우리가 실제로 건진 것이 얼마인가.**

| `ERROR` 가 삼킨 비율 | 파일 | 그 파일들이 낸 선언 |
|---|---:|---:|
| 100% | 23 | 0 |
| 43 ~ 82% | 4 | 1 |
| **30% 초과 소계 — 강등된다** | **27** | **1** |
| 0% (`MISSING` 둘, 너비 0) | 3 | 3 |

**23 개 파일이 `ERROR` 하나로 통째로 덮여 있다.** 그런데 대장은 그것을 `partial` 로
적었고, 그 낱말은 *"일부는 읽었다"* 라는 뜻이다. 읽은 것이 없다.

이것은 이 저장소가 반복해서 잡아 온 병의 또 다른 형태다:

| 언제 | 형태 | 무엇을 무엇으로 적었나 |
|---|---|---|
| F22-3 | **능력 부재** (`Capable` 로 고침) | *"추출기가 없다"* → *"변한 심볼 0 개"* |
| F02-1 | **입자 부재** (지목만, 열려 있음) | *"담을 심볼이 없다"* → *"변한 것이 없다"* |
| **F02-2** | **회복 위장** | *"통째로 못 읽었다"* → *"일부는 읽었다"* |

셋 다 **우리가 못 읽은 것을 읽은 것처럼 적는다.**

### 강등된 27 건 — **건수가 아니라 목록이다**

`portal-backend @ a29cad0bf6a8`. 경로의 `src/main/kotlin/kr/co/ecoletree/boxwood/` 를 `…/` 로 줄였다.

| 삼킨 비율 | 파일 |
|---:|---|
| 100 | `…/auth/pat/repository/ExposedPatTokenRepository.kt` |
| 100 | `…/auth/repository/impl/UserTokenRepositoryImpl.kt` |
| 100 | `…/auth/systempat/repository/SystemPatTokenRepository.kt` |
| 100 | `…/automation/connector/repository/impl/ConnectorDslRepository.kt` |
| 100 | `…/automation/email/templates/repository/impl/EmailTemplateDslRepository.kt` |
| 100 | `…/automation/llm/repository/impl/LlmPromptTemplateDslRepository.kt` |
| 100 | `…/automation/llm/repository/impl/LlmTaskMetaDslRepository.kt` |
| 100 | `…/automation/llm/repository/impl/LlmTaskVersionDslRepository.kt` |
| 100 | `…/automation/llm/repository/impl/McpToolConfigDslRepository.kt` |
| 100 | `…/automation/process/repository/impl/BpmnDslRepository.kt` |
| 100 | `…/automation/process/repository/impl/ProcessDslRepository.kt` |
| 100 | `…/automation/process/repository/impl/ProcessGlobalVariableDslRepository.kt` |
| 100 | `…/automation/process/repository/impl/ProcessTriggerDslRepository.kt` |
| 100 | `…/automation/process/repository/impl/ProcessTriggerEventVariableDslRepository.kt` |
| 100 | `…/automation/servicetask/repository/impl/ServiceTaskDslRepository.kt` |
| 100 | `…/group/base/repository/impl/MembershipTypeDslRepository.kt` |
| 100 | `…/organization/repository/impl/OrganizationGroupDslRepository.kt` |
| 100 | `…/organization/repository/impl/OrganizationMembershipDslRepository.kt` |
| 100 | `…/permission/repository/impl/ResourcePermissionMappingDslRepository.kt` |
| 100 | `…/permission/repository/impl/RolePermissionMappingDslRepository.kt` |
| 100 | `…/role/repository/impl/RoleGroupDslRepository.kt` |
| 100 | `…/role/repository/impl/RoleGroupMembershipDslRepository.kt` |
| 100 | `…/user/repository/impl/TenantUserDslRepository.kt` |
| 82 | `…/auth/repository/impl/TokenBlacklistRepositoryImpl.kt` |
| 79 | `…/auth/repository/impl/RefreshTokenFamilyDslRepository.kt` |
| 68 | `…/automation/llm/repository/impl/McpToolDslRepository.kt` |
| 43 | `…/permission/annotation/RequiresIntegratedPermission.kt` |

**`partial` 로 남은 셋**(전부 `annotation class` · `MISSING` 둘 · 선언 1 개씩):
`…/rbac/annotation/RequiresPermissions.kt` · `…/common/annotation/TenantTransactional.kt` ·
`…/common/annotation/SharedTransactional.kt`.

### ⚠ 임계값 30 은 **이 관측으로 확정되지 않는다**

걸린 값의 분포는 **0 · 43 · 68 · 79 · 82 · 100** 이고 **30 과 43 사이가 비어 있다.**
20 을 넣어도 40 을 넣어도 **같은 27 건**이 갈린다. 그러므로 이 관측이 말하는 것은
*"임계가 옳은 자리를 갈랐다"* 이지 *"30 이 옳은 값이다"* 가 아니다.
[DESIGN §5.5] 의 *"이 숫자들 중 실측에서 나온 것은 하나도 없다"* 가 그대로 유효하고,
상수가 `PROVISIONAL_` 접두어를 유지하는 이유가 그것이다. 확정은 **F05**(예산 회귀)다.

[DESIGN §5.5]: ../plan/00-stack.md

---

## 2. `Unsupported` 에 이유를 실었다 — **합격선 문면에 없는 판단이다**

④ 는 *"`Partial` 대신 `Unsupported` 로 낮춘다"* 라고만 적혀 있다. 그대로 하면 옛
`Unsupported{language}` 로 강등되는데, **그 변형의 문서가 *"추출기가 없다. 로드맵의
자리다"* 였다.** 그 문장은 `.sql` 에 대해 참이고 **Kotlin 파일에 대해 거짓이다** — 그
언어의 추출기는 있다.

뭉갰다면 대장 머리가 이렇게 적었을 것이다:

```
  unsupported        339    언어 인식됨, 추출기 없음
```

**사용자는 고칠 자리를 로드맵에서 찾는다.** 실제로 고칠 자리는 문법이다. 그래서
`UnsupportedReason{NoExtractor | GrammarDefeated{error_ratio_percent, recovery_sites}}`
를 세웠고 지금은 이렇게 적힌다:

```
  unsupported        339    추출기 없음(로드맵) 312 · 문법이 못 읽음 27
```

**상태는 여전히 일곱이고 칸도 일곱이다.** 늘어난 것은 한 칸 안의 **이유**이고, 그것이
`Capable`·`Residual`·`Uncapturable` 이 이 저장소에서 하는 일과 같다 — **없는 것의 종류를
값으로 남긴다.**

### 딸린 판단 — **강등된 파일의 심볼은 버린다**

27 건 중 하나(`RefreshTokenFamilyDslRepository.kt`, 79%)가 선언을 1 개 냈다. 그것을
대장에 실으면 *"못 읽었다"* 와 *"이 선언을 읽었다"* 가 같은 항목에 함께 선다. **범위가
그만큼 넓어 보인다.** 버렸고, 그 사실이 여기 적힌다 — 잃은 선언은 **1 개**다.

### ⚠ 이 판단이 **닫지 않은** 자리

대장 머리의 *"결박 불가 언어 10개 · 312 파일"* 은 **언어 단위 집계**라 강등된 27 건을
세지 않는다(Kotlin 은 다른 파일에서 L1 이므로 언어로서는 결박 가능하다). 그 27 개
**파일**에는 좌표가 없는데 그 줄은 그것을 말하지 않는다. 파일 단위 집계로 바꿀지는
이 조각의 합격선이 아니다 — **빚으로 적는다.**

---

## 3. `0` 이 "비교를 안 했다"가 아니라는 것 — **대조가 자기를 음성 대조한다**

①이 *"잃은 것 0"* 을 내면 그것이 *"다 살아남았다"* 인지 *"밖이 비어 있었다"* 인지
산출만으로는 갈리지 않는다. 그래서 같은 실행에서 둘을 함께 찍는다:

```
✓ 닫는 중괄호를 지운다        밖의 선언   6 · 잃은 것 0   (StringUtil.kt)
✓ 문법에 없는 토큰을 넣는다     밖의 선언  11 · 잃은 것 0   (coverage-manager.ts)
✓ 파일 끝을 잘라낸다         밖의 선언   6 · 잃은 것 0   (RoleGroupDto.kt)
  자기 대조   밖이 빈 변이 0(기대 0) · 없는 선언을 심으면 잡힌다 True(기대 True)
```

**첫 변이는 원래 밖이 1 이었다.** 파일 앞쪽(`nullToEmpty`, 열 중 둘째)을 깨뜨렸기
때문이고, 그러면 이 검사가 거의 아무것도 재지 않는다. 뒤쪽(`isEmpty`, 열 중 일곱째)으로
옮겨 6 으로 만들었다 — **그 교정을 여기 적는다.**

### "깨진 영역 밖"의 정의 — **이것이 판단이다**

깨뜨리면 뒤의 바이트가 전부 밀리므로 좌표로는 전후를 비교할 수 없다. 그래서:

> **회복 지점 중 가장 앞선 것보다 `byte_end` 가 완전히 앞선 선언** — 그것이 밖이고,
> 이름·종류·컨테이너·`body_digest` 가 하나도 안 바뀌어야 한다.

`byte_start` 가 아니라 `byte_end` 다. **깨진 자리를 품고 있는 컨테이너는 시작이 앞서도
밖이 아니다** — 그것을 밖으로 세면 통과할 수 없는 것을 통과시킨다.

**뒤쪽은 판정하지 않고 관측으로 적는다.** 닫는 중괄호를 지우면 그 컨테이너가 나머지를
삼키는 것이 정상 회복이고(F02 §4), 삼킨 범위 안에서 무엇이 살아남는지는 tree-sitter 의
회복 품질이지 우리 코드가 아니다(`[f02.2.does_not_prove].not_recovery_quality`):

```
닫는 중괄호를 지운다     삼킨 범위 뒤로 사라진 선언 4  [isEmpty, parseInt, parseInt, emptyToNull]
문법에 없는 토큰을 넣는다  삼킨 범위 뒤로 사라진 선언 0  []
파일 끝을 잘라낸다      삼킨 범위 뒤로 사라진 선언 8  [CreateRoleGroupRequest, …]
```

**이 해석은 합격선 문면에 없다. 여기 적어 판정에 싣는다.**

---

## 4. ② — **세우지 않았다. 실물이 0 이라서**

두 코퍼스 **전수**다. 표본이 아니다.

| | 파일 | 회복이 일어난 파일 | `ERROR` 안쪽에서 건진 선언 |
|---|---:|---:|---:|
| Kotlin (S0 코퍼스) | 1,122 | 56 | **0** |
| TypeScript (ditto @ `aded7ce7`) | 496 | 5 | **0** |

관측된 것은 **양 극단뿐이고 가운데가 없다** — `ERROR` 가 토큰 몇 개만 삼키거나(컨테이너도
그 안의 선언도 멀쩡히 나온다) 파일을 통째로 삼킨다(안쪽에 파싱된 것이 하나도 없다).
F02 §4 가 대응하려던 *"오류 회복이 클래스 전체를 ERROR 로 묶으면 그 안의 메서드가
사라진다"* 는 **일어나지 않는다.** 확인 삼아 직접 만들어 보았다:

```
class Broken { %%% ; m() {} n() {} }   →  심볼 [Broken, m, n] · 자리 1 (너비 3)
```

**세우지 않고 §4 를 정정했다** — `[f02.2.pass]` ② 가 요구한 그대로이고, F01 이 언어 인식
④(내용 휴리스틱)에 내린 것과 같은 형태다.

**추출기의 순회 자체는 `ERROR` 자식으로 내려간다**(TypeScript 순회의 기본 가지). 세우지
않은 것은 *"안쪽에서 건진 심볼을 따로 표시하는 것"* 이고, 표시할 심볼이 실물에 0 건이다.

---

## 5. ⑥ Kotlin 문법의 처분 — **후보 넷을 실측으로 댔다. 그리고 등록된 전제가 틀렸다**

### ⚠ 정정 — *"문법 업그레이드로 고쳐질 수 없다"* 는 **거짓이다**

[`s0.grammar`].`consequence` 가 적었다:

> 마지막 커밋 2025-01-16 · 19개월 정지 · **upstream HEAD 가 곧 우리가 핀한 커밋이다.**
> 최신이 곧 T7 이 쓴 그것이라 **문법 업그레이드로 고쳐질 수 없다.**

**뒤쪽 문장이 사실과 다르다.** crates.io 의 Kotlin 문법 여섯을 **각각 별도 바이너리로**
빌드해 S0 코퍼스 1,122 파일 전수에 댔다:

| 문법 | 성하게 파싱 | 깨짐 | 위 27 건 중 성함 |
|---|---:|---:|---:|
| **`ng` @ 우리 핀 `3dea6df`** | 1,066 | **56** | **0** |
| `tree-sitter-kotlin-sg` (ast-grep) | **1,113** | **9** | **26** |
| `brokk-tree-sitter-kotlin` | **1,113** | **9** | **26** |
| `tree-sitter-kotlin-codanna` | 1,107 | 15 | 26 |
| `tree-sitter-kotlin-updated` | 1,107 | 15 | 26 |
| `tree-sitter-kotlin-sqry` | 1,100 | 22 | 26 |
| `tree-sitter-kotlin` (fwcd 원본) | — | — | — |

> **측정이 성립하는 조건 하나를 먼저 틀렸다가 잡았다.** 여섯을 한 바이너리에 함께 링크하면
> 전부 같은 C 심볼(`tree_sitter_kotlin`)을 내보내므로 **링커가 하나를 고르고 여섯 행이
> 전부 같은 값이 된다.** 실제로 첫 실행이 그랬다 — 여섯 행이 글자까지 같았고, 그것이
> 신호였다. 문법마다 **별도 크레이트·별도 바이너리**로 다시 쟀다.
>
> fwcd 원본은 `tree-sitter >=0.21, <0.23` 을 요구해 우리 런타임(0.26)과 **링크되지
> 않는다.** 그것 자체가 관측이라 표에 남긴다.

**Exposed `} get` 관용구는 다섯 포크가 전부 읽는다.** 남는 1 건은 `annotation class` +
`@get:AliasFor`(`RequiresIntegratedPermission.kt`)이고 **어느 포크도 못 읽는다.**

### 처분 — **포크 채택이 옳다. 그러나 이 조각에서 하지 않는다**

| 후보 | 판단 | 근거 |
|---|---|---|
| **포크 채택** | **권고. 실행은 별도** | 실측이 56 → 9. 그러나 아래 세 비용이 이 조각 밖에 있다 |
| 자체 패치 | 기각 | 다섯이 이미 고쳤다. 우리가 유지 비용을 질 이유가 없다 |
| 관용구 우회 | 기각 | 추출기가 문법의 구멍을 흉내 내는 것이고, `ERROR` 안에서 이름을 주워 담게 된다. **못 읽은 것을 읽은 것처럼 적는 형태로 돌아간다** |
| 그대로 둔다 | 기각 | 27 건이 `unsupported` 로 남는다. **고칠 수 있는 것을 못 고친다고 적는 것**은 이 제품이 금지하는 형태다 |

**이 조각에서 핀을 움직이지 않는 이유 셋:**

1. **`s0-reference-vector.tsv`(1,126 줄)가 움직인다.** 지금 0 선언인 26 개 파일이 선언을
   내기 시작한다(최상위 선언 기준 약 **29 개**). 그 벡터는 **S0 게이트의 오라클**이고,
   F02-2 안에서 그것을 새로 만들면 **대조를 사후 조정하는 일**이다. 순서는
   `[f02.1.oracle]` 이 세운 그대로여야 한다 — **새 벡터를 CLI 레퍼런스로 먼저 만들어
   별도 커밋으로 등록하고, 그다음 핀을 옮긴다.**
2. **`ExtractorVersion` 의 문법 축이 바뀐다** → 1층 캐시가 **전량 무효화**되고 좌표가
   이동한다. F01 의 골든 997 도 함께 움직인다.
3. **어느 포크인지는 이 27 건으로 정할 수 없다.** `sg` 와 `brokk` 가 1,113 로 동률이고,
   **파싱이 성한 것과 트리가 옳은 것은 다른 문제다** — T7 이 센 *"조용한 오파싱 17 파일"*
   은 `ERROR` 없이 **틀린 트리**가 나오는 형태이고 이 표는 그것을 재지 못한다. 포크를
   고르려면 그것을 재는 합격선이 필요하고 그 오라클은 [R-03] · T7 의 것이다.

**그러므로 처분은 「포크 채택 · 별도 작업」이고, 그 작업의 첫 쓰기는 코드가 아니라
새 레퍼런스 벡터다.** 후속 이슈로 발행한다.

### 못 재는 것 — **`partial 0` 은 *"다 읽었다"* 가 아니다**

**조용한 오파싱을 이 조각은 세지 못한다.** 세려면 PSI 대조가 필요하고 그 오라클은
[R-03](../plan/00-risks.md#r-03) · T7 의 것이다. 위 표의 *"성하게 파싱 1,113"* 도 같은
한계를 진다 — `ERROR` 가 없다는 것이지 트리가 옳다는 것이 아니다.
**이것이 이 게이트의 가장 큰 구멍이다.**

---

## 6. 골든이 움직였다 — **회귀가 아니라 이 조각의 산출이다**

`corpus/golden/portal-backend.ledger.json` 을 다시 축복했다(`f01-verify --bless`).

| | 항목 |
|---|---:|
| `partial` → `unsupported{grammar_defeated}` | **27** (§1 의 목록) |
| `unsupported` 에 `reason: no_extractor` 필드 추가 | 312 |
| 움직이지 않은 항목 | 658 |
| 합 | 997 |

**심볼 상태가 그대로인 641 `parsed` 는 한 항목도 안 움직였다.** 그리고 `s0-compare` 는
영향받지 않는다 — `pal symbols` 는 `classify` 를 거치지 않으므로 강등이 선언 수를 바꾸지
않는다(1,122 파일 · 불일치 **0** 으로 확인).

---

## 7. 이 게이트가 **닿지 않은** 자리 · 넘기는 빚

| 빚 | 자리 |
|---|---|
| **Kotlin 문법 핀 교체** — 실측 56 → 9. 첫 쓰기는 새 S0 레퍼런스 벡터 | **후속 이슈** (§5) |
| 조용한 오파싱을 못 센다 — `partial 0` 이 *"다 읽었다"* 가 아니다 | [R-03] · T7 |
| 대장 머리의 *"결박 불가"* 가 언어 단위라 강등된 27 **파일**을 안 센다 | 열림 (§2) |
| `pal-core::budget` 에 상수가 하나뿐 — 흩어진 넷이 그대로 | 열림. 모듈 주석에 목록 |
| 회복 자리가 **1층 캐시에 안 실린다** — `FileOutcome` 은 개수만 담는다 | **F04** |
| **입자 부재** — 조상 없는 익명 함수. §8 | 열림 |
| `.tsx` 가 잘못된 문법으로 읽힌다 | 미배정 (`[f02.1.grammar]`) |
| 자연 발생한 깨진 Kotlin 이 코퍼스에 **0 건** — 회복 시험은 전부 우리가 깨뜨린 것 위에 선다 | 기록 (`not_broken_kotlin_in_the_wild`) |

---

## 8. **입자 부재** — #46 이 넘긴 것. 이웃이지만 여기서 고치지 않는다

F02-1 이 지목했다: 조상 없는 익명 함수(`describe`/`test` 콜백) 안의 변경이
`no_semantic_change` 로 사라진다. 그리고 *"조상 없는 익명 함수를 파일 단위 잔여로
남길지 — `partial` 회복(#47)의 이웃"* 이라고 적었다.

**판단: `recovery_sites` 에 담지 않는다.**

`RecoverySite` 는 *"문법이 이 바이트를 못 읽었다"* 를 뜻한다. 입자 부재는 그 반대다 —
**바이트는 완벽히 읽혔고, 담아 줄 심볼이 없을 뿐이다.** 둘을 한 목록에 담으면 *"못
읽었다"* 와 *"읽었는데 담을 곳이 없다"* 가 같은 출력이 되고, 그것이 이 조각이 §1 에서
고친 병 그 자체다.

**옳은 자리는 `Uncapturable` 이다** — 이미 `pal-core::chain` 에 서 있고 `NoSemanticChange`
변형을 갖는다. 그것을 *"변한 심볼이 없다"* 와 *"어느 심볼에도 담기지 않았다"* 로 가르는
것이 처분이고, **소유는 `no_semantic_change` 를 계산하는 쪽(`pal defect`)이지 추출기가
아니다.** 세는 규칙(`f02-recall-sample.tsv` 머리)은 코드보다 먼저 등록됐으므로 **건드리지
않았다.** 지목만 하고 넘긴다 — F02-1 이 한 것과 같은 형태다.

---

## 9. 재현

```bash
cargo build --workspace --release && cargo build --workspace
./scripts/s0-corpus.sh /tmp/s0-corpus ~/dev/projects/boxwood     # 1,122 파일 (저장소 셋)
./scripts/f02-2-verify.py --s0-corpus /tmp/s0-corpus             # 다섯 다 통과 (약 6분)
./scripts/f02-1-verify.py --s0-corpus /tmp/s0-corpus             # 다섯 다 통과
./scripts/f01-verify.py --repo ~/dev/projects/boxwood/portal-backend   # 여덟 다 통과
```

기준선(2026-08-13 · Darwin 25.5.0 · rustc 1.97.1): `cargo xtask check` 7/7 ·
테스트 **171**(161 → 새 시험 10) · clippy 경고 **2**(`chain.rs:224` · `ledger.rs:357` —
1.97 의 새 린트이고 회귀가 아니다. **이 조각은 하나도 더하지 않았다**).

커밋 둘 — **구조와 동작을 갈랐다**(Tidy First):

| | |
|---|---|
| `ce60f45` | **구조적** — 회복 지점을 개수에서 자리로. 산출 불변 증거를 커밋 메시지에 |
| `f587ccd` | **동작적** — 임계 강등 · `UnsupportedReason` · 골든 재축복 · 대조 |
