# G50 — Kotlin 문법 핀 교체 · **통과**. 그리고 어긋남 하나를 1급으로 적는다

**판정: 통과** (2026-08-13 · [#50](https://github.com/hskim-ecoletree/palimpsest/issues/50))
합격선 정본 `corpus/criteria.toml` `[g50]` — **코드보다도 측정보다도 먼저 등록됐다**
(`af157e7` · `registered_before_any_measurement = true`).

    tree-sitter-kotlin-ng      @ 3dea6df   1,122 중 56 을 못 읽음 · 2025-01-16 이후 정지
  → brokk-tree-sitter-kotlin   @ acb9630   1,122 을 **전부** 읽음 · upstream 뒤짐 0

이 판정이 답하는 문장은 하나다(`[g50].judges`):

> **포크를 무엇으로 골랐는가가 고르기 전에 적혀 있고, 그 근거가 「파싱이 성하다」보다
> 센가.**

**그렇다.** 그리고 그 장치가 실제로 일을 했다 — 골랐어야 할 근거(축 A)가 **아무것도
가르지 못했고**, 미리 정해 둔 순서(축 C)가 갈랐다. 만약 등록이 없었다면 `sg` 를
「1.88M 다운로드 · ast-grep 이 쓴다」로 골랐을 것이고, **그 근거는 측정이 아니라 인상이다.**

---

## 0. 합격선 일곱에 대한 판정

| | 합격선 | 판정 |
|---|---|---|
| ① | 레퍼런스 벡터가 핀보다 **앞선 커밋**에 | **통과** — `2ee4434` → `9e4e398` |
| ② | `s0-compare` 불일치 **0** | **통과** — 0 / 1,122 |
| ③ | ★ 쿼리 패턴 다섯 고정 · 이름 치환만 | **통과** — §6 |
| ④ | ★ 골든 넷 중 `ditto` 는 **안 움직인다** | **통과** — 0 / 4,578 |
| ⑤ | ★ 강등 27 중 **26** 이 돌아오고 1 은 남는다 | **⚠ 어긋났다 — 27 이 전부 돌아왔다. §4** |
| ⑥ | 문법 축의 비대칭을 **판단하고 적었다** | **통과** — §7 |
| ⑦ | 기준선이 안 움직인다 | **통과** — §9 |

**⑤ 가 어긋났고 그것을 숨기지 않는다.** `falsified_if` 가 반증으로 지정한 둘(② 불일치 ·
④ `ditto` 이동)은 **둘 다 안 걸렸으므로** 게이트는 통과로 닫힌다. ⑤ 는 **우리 코드에
대한 관측이 아니라 F02-2 의 실측에 대한 관측**이고, 그 처분을 §4 에 적는다.

---

## 1. 어느 포크인가 — **오라클이 고르기보다 먼저 있었다**

F02-2 는 실측을 끝내 놓고도 핀을 안 옮겼다. 이유가 이것이다:

> `sg` 와 `brokk` 가 1,113 으로 **동률**이고, **파싱이 성한 것과 트리가 옳은 것은
> 다른 문제다.**

그래서 `[g50.oracle]` 이 **축 셋과 팔 셋**을 등록했다. 실행은
`scripts/g50-fork-oracle.py` 이고 산출은 이렇다:

| 팔 | rev | 깨짐 | 선언 총수 | 강등 27 중 성함 | 조용한 오파싱 후보 |
|---|---|---:|---:|---:|---:|
| **`ng`** (★ 음성 대조) | `3dea6df` | **56** | **2,241** | **0** | 64 |
| `sg` | `1a6f9b1` | 0 | 2,315 | 27 | **0** |
| `brokk` | `acb9630` | 0 | 2,315 | 27 | **0** |

**★ 음성 대조가 섰다.** 현행 핀이 F02-2 의 **56** 과 T7 의 **2,241** 을 글자까지
재현한다. 그것이 재현되지 않았다면 후보의 값도 못 읽는다.

### 축 A — **차분 0. 「측정으로 가를 수 없다」가 판정이다**

`sg` ↔ `brokk`: 파싱 성패 **0** · 선언 수 **0** · **트리 S-표현식 0**.
1,122 파일에서 파싱 트리가 **바이트로 같다.** 그런데 `src/parser.c` 는 서로 다르다
(sha256 `04f52fe…` ↔ `70f193d…`). **소스가 다른데 이 코퍼스에서의 산출이 같다.**

그러므로 축 A 의 결론은 *"어느 쪽을 골라도 우리 산출은 같다"* 이고, 그것은 **축 C 의
근거가 약해도 손해가 없다**는 뜻이기도 하다.

### 축 B — 손으로 읽을 후보가 **둘 다 0**

트리가 옳은지는 문법에게 물으면 순환이다. 그래서 오라클을 문법 밖에 뒀다 —
`scripts/g50-kotlin-scan.py` 는 **tree-sitter 를 한 줄도 쓰지 않고** 0 열에서 시작하는
선언 머리를 센다(주석·문자열·원시 문자열·문자 리터럴을 한 스캐너에서 가른다).

표본 44(조용한 오파싱 17 · 강등 27). 판정 규칙: **독립 계수 ≥ 1 인데 문법이 0** 이면
손으로 읽는다.

    sg     후보 0
    brokk  후보 0
    ng     후보 64   ← 같은 잣대로 잰 현행 핀

**손으로 읽을 것이 나오지 않았다.** 덤으로, 독립 계수기가 코퍼스 전체에서 센 선언이
**2,315** 로 두 후보의 값과 같다(하한 2,000 을 검사에 박아 뒀다).

### 축 C — **③(상류 추종)이 갈랐다**

축 A 가 갈랐으면 이 축을 보지 않는다 — 그러면 「고른 뒤에 근거를 만드는 일」이 되기
때문이고, **그 조건을 스크립트에 코드로 박았다.** 못 갈랐으므로 등록된 순서대로:

| 순서 | 항목 | `sg` | `brokk` |
|---|---|---|---|
| ① | rev 로 핀 가능한 git 출처 | 있다 | 있다 |
| ② | 라이선스 (예외를 늘리는가) | MIT · 안 는다 | MIT · 안 는다 |
| **③** | **상류(fwcd) 보다 뒤진 커밋** | **5** (그중 문법 **3**) | **0** |
| ④ | `tree-sitter` 0.26 과 링크 | 된다 | 된다 |

`sg` 가 뒤진 문법 커밋 셋: `#280` 선언 대 중위식 우선 · `#279` 여러 줄 주석의 NUL
바이트에서 파싱이 멈추는 것 · `#278` 수식자 선행 클래스. 그리고 **`brokk` 는 `sg` 를
엄격히 포함한다**(`sg` HEAD~1 이 `brokk` 의 조상).

**두 후보 다 `fwcd/tree-sitter-kotlin` 의 포크다**(GitHub `parent`). upstream 은 살아
있고(2026-08-02) `brokk` 가 그것을 그대로 담고 있다.

> **인상으로 고르면 반대로 갔을 것이다.** `sg` 는 다운로드 1.88M · 2024 년부터 · ast-grep
> 이 쓰는 문법이고, `brokk` 는 **크레이트가 이틀 됐고 다운로드가 989** 다. 등록된 축이
> 없었으면 그 신호를 「유지보수 신호」라고 부르며 골랐을 것이고, **그것은 축 C 가 재는
> 것이 아니다.** 축 C ③ 이 재는 것은 인기가 아니라 **상류와의 거리**다.
>
> 이 선택이 지는 위험은 적는다 — **`brokk` 는 신생이고 관리 주체가 회사 하나다.**
> 우리는 **git rev 로 핀하므로 내용은 얼어 있고**, 다음 승급 때 이 축을 다시 댄다.
> 그때 `sg` 가 upstream 을 따라잡았으면 `sg` 가 이긴다.

---

## 2. ⚠ **대조가 또 꺼졌다 — 열 번째 형태다**

F03 지붕 §3 이 「대조가 자기도 모르게 꺼지는」 형태를 **아홉**으로 셌다. 여기서 **열 번째**가
나왔고, **F02-2 에서 난 사고의 두 번째 형태**다.

F02-2 는 문법 여섯을 한 바이너리에 링크했다가 전부 같은 C 심볼(`tree_sitter_kotlin`)을
내보내 **링커가 하나만 고르고 여섯 행이 글자까지 같아지는** 사고를 냈다. 처방은
**「문법마다 별도 크레이트 · 별도 바이너리」**였다.

**그 처방이 여기서는 안 듣는다.** 클론 셋을 서로 다른 디렉터리에 두고 각자에서
`tree-sitter build` 를 했는데도 **세 팔이 같은 파서를 실었다** — CLI 가 컴파일 결과를

    ~/.cache/tree-sitter/lib/<문법이름>.dylib

에 넣는데 **세 문법의 이름이 전부 `kotlin`** 이기 때문이다. 공유 상태의 자리가
「바이너리」에서 **「이름으로 키를 잡은 캐시」**로 옮겼다.

**드러난 방식은 그때와 같다** — 음성 대조 팔(`ng`)이 자기 값을 못 냈다(선언 총수 0).
**팔이 둘뿐이었으면 안 드러났다.** 두 후보는 같은 계보라 진짜로 값이 같고, 캐시가
공유돼도 「둘이 같다」로 읽혔을 것이다. **셋째 팔이 그것을 갈랐다.**

처방 둘을 박았다:

  · 팔마다 `HOME`·`XDG_CACHE_HOME` 을 따로 준다
  · **캐시 파일이 셋으로 갈렸는지를 검사한다** — 안 갈리면 멈춘다

그리고 같은 함정이 `scripts/s0-reference.py` 에도 있었다(`9f85c1f` 에서 고쳤다).
**핀을 옮기는 회차가 정확히 그 함정 위에 있다** — 격리 없이는 옛 문법으로 만든 파서가
새 레퍼런스 벡터를 만들 수 있고, 그러면 벡터가 자기 출처를 거짓으로 적는다.

---

## 3. 움직인 것 — **건수가 아니라 목록이다**

### 3.1 레퍼런스 벡터 (`2ee4434`)

    움직인 파일        79 / 1,122
      fail → ok        56   ← `} get` 관용구
      선언 0 → n       64   ← `ng` 의 「조용한 오파싱 후보 64」와 같은 집합
      선언이 준 파일     0   ★ 반대 방향 — 하나도 안 줄었다
    선언 총수      2,241 → 2,315      파싱 실패 56 → 0      파싱 성공했는데 0건 17 → 0

**「선언이 준 파일 0」이 이 항목의 음성 대조다.** 총수만 보면 *"늘었으니 좋아졌다"* 로
읽히는데, 옛 문법이 세던 것을 새 문법이 놓쳐도 총수는 늘 수 있다.

목록 전문은 **커밋 `2ee4434` 의 diff 자체**다 — 1,122 줄 전부가 파일별로 실려 있고,
움직인 79 줄이 그 안에 있다. 여기 다시 옮겨 적지 않는다(AGENTS.md 진행 규칙 4).

### 3.2 대장 골든 (`343dd5e` · `f01-verify --bless`)

**31 곳** — 감지기 1 + 파일 30.

    Kotlin  641 parsed · 27 unsupported{grammar_defeated} · 3 partial
          → Kotlin  **671 parsed** · 0 · 0
    unsupported 전체  339 → 312   (남은 것은 전부 `no_extractor` — 로드맵의 언어들)
    detector.grammar  3dea6df… → acb9630…

파일 30 = F02-2 가 강등한 **27** + 그때 `partial` 로 남은 **3**
(`SharedTransactional.kt` · `TenantTransactional.kt` · `RequiresPermissions.kt`).
**전부 `parsed{l1}` 이 됐다.**

**대장 머리의 *"문법이 못 읽음 27"* 이 사라졌다.** 그것이 이 작업의 산출이다.

### 3.3 심볼 골든 — portal-backend (`343dd5e` · `f03-3-verify --bless`)

    줄        1,296 → 1,340
    사라짐        0   ★ 반대 방향 — 새 문법이 옛 문법의 심볼을 하나도 안 놓쳤다
    새로         44
    좌표 이동      0   `symbol_id` 는 문법을 성분으로 안 갖는다 — 그것이 옳다
    요약 이동  1,199
    안 움직인 요약 68   ← §5 가 이것을 다룬다

**새로 44 의 전문** (경로의 `src/main/kotlin/kr/co/ecoletree/boxwood/` 를 `…/` 로 줄였다):

| 파일 | 심볼 | 종류 |
|---|---|---|
| `…/auth/pat/repository/ExposedPatTokenRepository.kt` | `ExposedPatTokenRepository` | class |
| `…/auth/repository/impl/RefreshTokenFamilyDslRepository.kt` | `RefreshTokenFamilyDslRepository` | class |
| `…/auth/repository/impl/RefreshTokenFamilyDslRepository.kt` | `logger` | property |
| `…/auth/repository/impl/TokenBlacklistRepositoryImpl.kt` | `TokenBlacklistRepositoryImpl` | class |
| `…/auth/repository/impl/UserTokenRepositoryImpl.kt` | `UserTokenRepositoryImpl` | class |
| `…/auth/repository/impl/UserTokenRepositoryImpl.kt` | `logger` | property |
| `…/auth/systempat/repository/SystemPatTokenRepository.kt` | `SystemPatTokenRepository` | class |
| `…/automation/connector/repository/impl/ConnectorDslRepository.kt` | `ConnectorDslRepository` | class |
| `…/automation/credentials/model/dto/CredentialDto.kt` | `CredentialValue` | class |
| `…/automation/email/templates/repository/impl/EmailTemplateDslRepository.kt` | `EmailTemplateDslRepository` | class |
| `…/automation/llm/repository/impl/LlmPromptTemplateDslRepository.kt` | `LlmPromptTemplateDslRepository` | class |
| `…/automation/llm/repository/impl/LlmTaskMetaDslRepository.kt` | `LlmTaskMetaDslRepository` | class |
| `…/automation/llm/repository/impl/LlmTaskVersionDslRepository.kt` | `LlmTaskVersionDslRepository` | class |
| `…/automation/llm/repository/impl/McpToolConfigDslRepository.kt` | `McpToolConfigDslRepository` | class |
| `…/automation/llm/repository/impl/McpToolDslRepository.kt` | `McpToolDslRepository` | class |
| `…/automation/process/repository/impl/BpmnDslRepository.kt` | `BpmnDslRepository` | class |
| `…/automation/process/repository/impl/ProcessDslRepository.kt` | `ProcessDslRepository` | class |
| `…/automation/process/repository/impl/ProcessGlobalVariableDslRepository.kt` | `ProcessGlobalVariableDslRepository` | class |
| `…/automation/process/repository/impl/ProcessTriggerDslRepository.kt` | `ProcessTriggerDslRepository` | class |
| `…/automation/process/repository/impl/ProcessTriggerEventVariableDslRepository.kt` | `ProcessTriggerEventVariableDslRepository` | class |
| `…/automation/servicetask/repository/impl/ServiceTaskDslRepository.kt` | `ServiceTaskDslRepository` | class |
| `…/config/JwtProperties.kt` | `JwtProperties` | class |
| `…/config/RateLimitProperties.kt` | `RateLimitProperties` | class |
| `…/group/base/repository/impl/MembershipTypeDslRepository.kt` | `MembershipTypeDslRepository` | class |
| `…/organization/repository/impl/OrganizationGroupDslRepository.kt` | `OrganizationGroupDslRepository` | class |
| `…/organization/repository/impl/OrganizationMembershipDslRepository.kt` | `OrganizationMembershipDslRepository` | class |
| `…/permission/annotation/RequiresIntegratedPermission.kt` | `RequiresIntegratedPermission` | class |
| `…/permission/repository/impl/ResourcePermissionMappingDslRepository.kt` | `ResourcePermissionMappingDslRepository` | class |
| `…/permission/repository/impl/RolePermissionMappingDslRepository.kt` | `RolePermissionMappingDslRepository` | class |
| `…/role/repository/impl/RoleGroupDslRepository.kt` | `RoleGroupDslRepository` | class |
| `…/role/repository/impl/RoleGroupMembershipDslRepository.kt` | `RoleGroupMembershipDslRepository` | class |
| `…/user/repository/impl/TenantUserDslRepository.kt` | `TenantUserDslRepository` | class |
| `src/test/…/auth/admin/controller/TenantAdminAuthControllerTest.kt` | `TenantAdminAuthControllerTest` | class |
| `src/test/…/auth/admin/service/impl/TenantAdminAuthServiceImplTest.kt` | `TenantAdminAuthServiceImplTest` | class |
| `src/test/…/auth/core/service/RtrServiceTest.kt` | `RtrServiceTest` | class |
| `src/test/…/auth/service/SessionManagementServiceTest.kt` | `SessionManagementServiceTest` | class |
| `src/test/…/auth/systempat/filter/SystemPatAuthenticationFilterTest.kt` | `SystemPatAuthenticationFilterTest` | class |
| `src/test/…/auth/systempat/service/SystemPatServiceImplTest.kt` | `SystemPatServiceImplTest` | class |
| `src/test/…/auth/systempat/service/SystemPatUserResolverTest.kt` | `SystemPatUserResolverTest` | class |
| `src/test/…/automation/connector/OpenApiSchemaServiceTest.kt` | `OpenApiSchemaServiceTest` | class |
| `src/test/…/notification/service/NotificationServiceTest.kt` | `NotificationServiceTest` | class |
| `src/test/…/permission/aspect/IntegratedPermissionControllerIntegrationTest.kt` | `IntegratedPermissionControllerIntegrationTest` | class |
| `src/test/…/shared/service/SecurityEventServiceTest.kt` | `SecurityEventServiceTest` | class |
| `src/test/…/shared/service/TenantStateServiceTest.kt` | `TenantStateServiceTest` | class |

**요약 이동 1,199 의 목록은 골든 파일 자신이다** — `343dd5e` 의 diff 가 심볼별로
옛 요약과 새 요약을 나란히 싣는다. 여기 1,199 줄을 옮겨 적으면 그것이 곧 drift 다.

### 3.4 ★ `ditto.symbols.tsv` — **움직인 것 0 / 4,578**

**이것이 이 작업의 음성 대조다.** Kotlin 문법을 통째로 갈았는데 TypeScript 심볼의
`symbol_id` 도 `body_digest` 도 한 줄도 안 움직였다. §7 의 비대칭이 실물로 확인된 자리다.

---

## 4. ⚠ **어긋남 ⑤ — 등록된 기대가 F02-2 의 실측에서 왔고, 그 실측이 재현되지 않는다**

등록된 것: *"강등 27 중 **26** 이 돌아오고, 남는 1(`RequiresIntegratedPermission.kt`)은
그대로 `unsupported` 다. **27 이 전부 돌아오면 실패다.**"*

**실측: 27 이 전부 돌아왔다. 그리고 `partial` 3 도 함께 돌아왔다.**

어긋남을 등록된 반대 방향의 이유 둘에 대 봤다:

| 등록된 이유 | 확인 | 결과 |
|---|---|---|
| 강등 판정이 무력화됐다 (임계가 안 걸린다) | 단위 시험 `error_가_파일을_삼키면_partial_이_아니라_강등이다` | **그대로 통과한다.** 판정 장치는 살아 있다 |
| 세는 자리를 잘못 잡았다 | 대장 항목 997 · Kotlin 671 · 상태 분포 | **자리는 맞다.** 671 이 전부 `parsed` 다 |

**둘 다 아니다. 남는 설명은 하나 — F02-2 의 그 표가 재현되지 않는다.**

F02-2 §5 가 적은 두 값이 우리 실측과 다르다:

| F02-2 가 적은 것 | 이번 실측 |
|---|---|
| `sg`·`brokk` 깨짐 **9** | **0** |
| 강등 27 중 성함 **26** | **27** |
| *"남는 1 건은 `annotation class` + `@get:AliasFor`(`RequiresIntegratedPermission.kt`)이고 **어느 포크도 못 읽는다**"* | 두 후보 모두 **성하게 읽는다** |

**그리고 그 문장이 두 가지를 섞었다.** `RequiresIntegratedPermission.kt` 에는
`@get:AliasFor` 가 **없다**(전문을 읽었다). 코퍼스에서 `AliasFor` 를 쓰는 파일은
**둘**이고 둘 다 다른 파일이다:

    common/annotation/TenantTransactional.kt   AliasFor 7 곳
    common/annotation/SharedTransactional.kt   AliasFor 7 곳

그 둘은 F02-2 에서 **강등 27 이 아니라 `partial` 로 남은 3** 에 속한다. 즉 F02-2 의
그 문장은 **강등 목록의 마지막 항목**과 **`partial` 로 남은 관용구**를 한 문장에
붙였다. 셋 다 새 문법에서 `parsed` 다 — **`@get:AliasFor` 도 읽힌다.**

**우리 실측이 옳다고 볼 근거 셋:**

1. **음성 대조 팔이 섰다** — 같은 장치가 `ng` 에 대해 56 과 2,241 을 글자까지 재현한다
2. **문법 밖의 계수기가 같은 값을 낸다** — tree-sitter 를 안 쓰고 센 선언이 2,315
3. **우리 경로로도 확인됐다** — 핀을 옮긴 뒤 `pal ledger` 가 30 파일을 `parsed` 로 낸다.
   CLI 관측이 아니라 **제품 경로의 산출이다**

**처분: F02-2 의 표를 여기서 고치지 않는다.** 그 게이트는 자기 회차의 기록이고 끝난
판정이다 — 끝난 게이트를 새 사실로 덮으면 그 게이트가 소급으로 다른 게이트가 된다
(`[g50.s0_grammar_correction]` 이 `[s0.grammar]` 에 대해 판단한 것과 같은 형태다).
**대신 이 게이트가 그것을 지목하고, F02-2 의 harness 가 무엇을 「깨짐」으로 셌는지는
다시 재지 않는다** — 그 harness 는 그 회차에서 폐기됐고 재구성하는 것은 이 작업의
범위가 아니다. **넘기는 빚으로 적는다(§8).**

> **이 어긋남의 값어치**: ⑤ 는 *"27 이 전부 돌아오면 실패"* 라는 **반대 방향**을 갖고
> 있었기 때문에 이 재현 실패가 드러났다. 그 방향이 없었으면 **「27 이 다 고쳐졌다,
> 잘됐다」로 지나갔을 것이다.** 음성 대조는 우리 코드만 재는 것이 아니라 **우리가
> 근거로 삼은 남의 측정**도 잰다.

---

## 5. ⚠ **ADR-0007 의 한 문장이 과장이다 — 정정한다**

ADR-0007 «결과» 절:

> **정규형이 문법 노드 이름에 묶인다.** 마디 표식이 tree-sitter 의 `kind()` 다.
> 문법을 올리면 노드 이름이 바뀔 수 있고 그러면 **모든 요약이 이동한다.**

**「마디 표식이 `kind()` 다」가 사실과 다르다.** `pal-extract::parse::normalize_into`
가 미는 것은 **상수 바이트 둘**이다:

```rust
const NODE_OPEN:  u8 = 0x1c;
const NODE_CLOSE: u8 = 0x1a;
```

이름은 **한 글자도 안 실린다.** 정규형이 묶인 것은 **트리 모양**(마디의 여닫음)과
**잎 토큰의 바이트**이고, 노드 **이름**에 의존하는 자리는 다섯뿐이다 —
`kind.contains("comment")` · `kind == "string"` · `is_plain_template` · `kind == ";"` ·
`TRANSPARENT`/`TRANSPARENT_IF_SINGLE` 의 목록.

**실물이 그것을 증명한다: 68 개 심볼의 `body_digest` 가 안 움직였다.** 계보가 다른
문법으로 갈았는데도 그렇다(67 class · 1 type_alias).
**표식이 `kind()` 였다면 1,267 이 전부 움직였어야 한다** — `identifier` 라는 이름은
거의 모든 선언에 나온다.

**어느 68 인지는 목록으로 있지만 「왜 그 68 인가」는 안 쟀다.** 눈으로 본 몇은 본문이
메서드 시그니처뿐인 `interface` 였고, 두 문법이 그 모양에서 우연히 같은 트리를 만든
것으로 보인다. **그 「보인다」를 이 게이트는 증명하지 않는다** — 증명에 필요한 것은
두 문법의 정규형을 나란히 내는 장치이고, 그것은 이 작업의 범위 밖이다.
**정정에 필요한 것은 「전부는 아니다」 하나이고 그것은 68 로 이미 섰다.**

**뒤따르는 정정 하나**: *"포크가 노드 이름을 하나라도 바꾸면 Kotlin 의 `body_digest`
가 전부 움직인다"* 는 이 작업의 핸드오프가 예상한 비용인데, **그것도 참이 아니다.**
순수한 이름 변경은 요약을 **하나도** 안 움직인다. 이번에 1,199 가 움직인 이유는
이름이 바뀌어서가 아니라 **두 문법이 트리를 다르게 만들기 때문**이다(계보가 다르다).

**ADR 본문에 정정을 덧붙였다** — 지우지 않는다. 계획 §7 의 규율(*"모순되면 조용히
덮지 말고 명시할 것"*) 그대로다.

**결정 자체는 안 뒤집힌다.** ADR-0007 이 정한 것은 *"정규형은 트리의 직렬화다"* 이고
그것은 그대로 참이다. 틀린 것은 **그 결정이 무엇에 묶이는가**에 대한 설명이고,
바로잡으면 결박이 **약해진다** — 문법 승급의 비용이 예상보다 작다.

---

## 6. 쿼리를 고쳤다 — **「사후 조정」인지의 판단**

**계보가 바뀌었다.** `ng` 는 amaanq 의 **다시 쓰기**라 이름 마디가 `identifier` 이고
`name:`·`type:` 필드가 있다. fwcd 계열에는 **그 필드가 아예 없고** 이름 마디가 둘로
갈린다(`type_identifier` / `simple_identifier`). 공유 쿼리가 **컴파일조차 안 된다**
(`Query error: Invalid field name "name"`).

`[g50.pass]` ③ 이 **고르기 전에** 정한 규칙 안에서 옮겼다:

| 규칙 | 지켰나 |
|---|---|
| 패턴 수 **다섯** 유지 | 지켰다 |
| **이름 치환만** | 지켰다 — `identifier` → `type_identifier`/`simple_identifier` |
| 술어(`#eq?` 등)를 더하지 않는다 | 지켰다 — 0 개 |
| `source_file` **직계 자식**이라는 단위를 안 푼다 | 지켰다 |

**한 가지는 순수한 치환이 아니다** — `name:`·`type:` **필드 제약이 사라졌다.**
필드가 없는 문법이라 표현할 방법이 없다. 제약이 사라지면 **매치가 늘 수 있고**,
늘어난 선언은 문법이 좋아져서가 아니라 **우리가 더 세기로 해서** 생긴 것이 된다.

**그래서 반대 방향을 쟀다** ★ — **이름을 아예 안 보는 쿼리**를 따로 만들어
같은 코퍼스에 대고, 매치 수가 파일별로 같은지 확인한다:

```
(source_file (class_declaration) @decl)   … 다섯
```

**세 팔 모두 1,122 파일 전수에서 매치 수가 같다.** 즉 이름 마디가 매치를 **늘리지도
줄이지도 않았다.** 그 검사가 `g50-fork-oracle.py` 안에 상시로 박혀 있다.

**판정: 사후 조정이 아니다.** 근거는 둘 — ① 규칙이 **고르기 전에** 등록됐고 ② 느슨해진
만큼을 **반대 방향으로 쟀다.**

---

## 7. 문법 축이 하나인데 언어가 둘이다 — **가르지 않는다** (⑥)

`ExtractorVersion { grammar, extractor }` 의 `grammar` 는 상수 하나이고 두 언어가
함께 탄다. Kotlin 문법을 올리면:

| | 움직이나 |
|---|---|
| 1층 캐시 | **두 언어 모두 전량 무효화** |
| `Coord.extractor` | **두 언어 모두 움직인다** (좌표의 성분이므로) |
| Kotlin 의 `symbol_id`·`body_digest` | 움직인다 (요약 1,199) |
| **TypeScript 의 `symbol_id`·`body_digest`** | **안 움직인다** — §3.4 가 0 으로 확인 |

**세우는 것이 합격선이 아니라 판단하고 근거를 적는 것이 합격선이다.**
**판단: 가르지 않는다.** 근거 셋(전문은 `GRAMMAR_REV` 의 문서 주석):

1. **[ADR-0004] 가 요구하는 것은 「산출을 정하는 모든 입력이 키에 있다」이고, 지금
   형태는 그것을 어기지 않는다.** 어기는 방향은 **덜** 무효화하는 쪽이고 지금은
   **더** 무효화한다. **과잉 무효화는 느릴 뿐 틀리지 않는다**
2. **가르려면 캐시 키가 「이 블롭이 무슨 언어인가」를 알아야 하는데 그것은 우리 코드가
   내리는 판정이다**(`recognize`). 판정을 키의 성분으로 쓰면 **판정이 틀린 파일이 틀린
   키를 갖고 그 틀림이 캐시 뒤로 숨는다.** F03 이 찾은 「실코드인 `.ts` 다섯이
   `binary{nul_byte}`」가 축을 갈랐다면 **재분류돼도 옛 항목을 그대로 돌려받았을
   것이다**
3. **비용이 일회성이고 F04 의 것이다.** 무효화되는 것은 1층 캐시뿐이고 다시 채우는
   값은 이미 재고 있다

**비대칭은 남고, 남는다는 사실을 적는 것이 여기서 지는 몫이다.**
그리고 그 비대칭에는 **관측 장치가 있다** — `ditto` 골든이 안 움직이는 것이 그것이고,
움직이면 축이 새는 자리다.

**`EXTRACTOR_REV` 는 안 올렸다.** 축이 둘인 이유가 이것이다 — 추출기 코드는 안 바뀌었다.

[ADR-0004]: ../adr/0004-cache-key-covers-every-input-that-decides-the-output.md

---

## 8. 넘기는 빚 — **건수가 아니라 목록**

| 빚 | 자리 |
|---|---|
| **F02-2 의 「깨짐 9 · 26/27」이 재현되지 않는다** — 그 harness 가 무엇을 셌는지 안 잼 | **열림** (§4). 재구성 비용이 이 작업 밖이다 |
| **조용한 오파싱을 전수로 못 센다** — 축 B 는 표본이고 그 표본을 우리가 떴다 | [R-03] · T7. **F02-2 가 자기 가장 큰 구멍이라고 적은 그것이 그대로다** |
| 골든 diff 의 열쇠가 `(path, container, name, kind)` 라 **중복 열쇠 29 를 접는다** | 열림 — **탐지는 바이트 대조라 정확하고, 접히는 것은 보고뿐이다** |
| `pal doctor`·`pal ledger` 머리가 이제 `grammar_defeated` 를 **한 건도 안 낸다** — 그 표시 경로가 실물 하중 0 이 됐다 | 열림. 회귀 시험은 단위 시험이 진다 |
| Kotlin 이 여전히 **L1 · `ordinal`** 이고 컨테이너 체인이 빈다 | F03 이 넘긴 그대로. **문법을 갈아도 안 움직인다** |
| `.tsx` 가 잘못된 문법으로 읽힌다 | 미배정 (`[f02.1.grammar]`) |

---

## 9. 재현 · 기준선

```bash
cargo build --workspace --release && cargo build --workspace
./scripts/s0-corpus.sh /tmp/s0-corpus ~/dev/projects/boxwood     # 1,122 파일 (저장소 셋)

# 포크 오라클 — 클론 셋이 필요하다
git clone https://github.com/tree-sitter-grammars/tree-sitter-kotlin <부모>/ng
git clone https://github.com/ast-grep/tree-sitter-kotlin           <부모>/sg
git clone https://github.com/BrokkAi/tree-sitter-kotlin            <부모>/brokk
./scripts/g50-fork-oracle.py --arms <부모> --corpus /tmp/s0-corpus

./scripts/s0-compare.py --corpus /tmp/s0-corpus                   # 불일치 0
./scripts/f01-verify.py --repo ~/dev/projects/boxwood/portal-backend
./scripts/f03-3-verify.py
```

기준선(2026-08-13 · macOS 26.5.2 arm64 · rustc 1.97.1 · tree-sitter CLI 0.26.12 ·
전체 약 **90 분**):

| 대조 | 값 | F03 종료 시점 |
|---|---|---|
| `cargo xtask check` | **8/8** | 8/8 |
| `cargo test --workspace` | **228** | 228 |
| `cargo clippy` | 경고 **2** (1.97 의 새 린트) | 2 |
| `s0-compare` | **불일치 0 / 1,122** | 불일치 0 |
| — 선언 총수 | **2,315** | 2,241 |
| — 선언 ≥1 파일 | **1,122 (100.00%)** | 1,058 (94.30%) |
| `f22-1` · `f22-2` | 9/9 · 7/7 | 같음 |
| `f22-3` · `f22-4` | 음성 대조 실패 **0** · **0** | 같음 |
| `s1` · `s2` · `s3` | 4/4 · 6/6 · 5/5 | 같음 |
| `f01` | **8/8** | 8/8 |
| `f02-1` ~ `f02-4` | 각 **5/5** | 각 5/5 |
| `f03-1` · `f03-2` · `f03-3` | **6/6** · 둘 · **4/4** | 같음 |

**움직인 것은 S0 의 두 값뿐이고 그것이 이 작업의 산출이다.** 나머지는 전부 그대로다 —
`[g50.pass]` ⑦ 이 요구한 것이 이것이다.

확인해 둘 것 셋:

  · **`f02-2` 가 `partial 30` 을 정답으로 안 박고 있었다.** `[f02.2.pass]` ⑤ 가
    *"재는 것은 값이 아니라 차이"* 로 등록된 덕이다. 지금 그 스크립트는
    *"강등된 파일 0 · `partial` 로 남은 것 0 · **이 코퍼스에서는 임계가 켜지지
    않는다** — 그 사실이 기록이다"* 를 낸다
  · **`f03-1` ⑥(결정성)이 새 문법에서도 선다** — 회차 둘이 같다
    (ditto 4,578 · portal-backend **1,340**)
  · **`f03-2` ①의 불변율이 100.00% 그대로다** — 포매팅 변형 일곱에서 움직인 심볼 0,
    의미 변형에서 가시율 100.00%. **Kotlin 문법을 갈아도 TypeScript 정규화는 안 흔들린다**

---

## 10. ADR

**새 ADR 을 발행하지 않는다.** 이 작업은 결정을 새로 세우지 않았다 — ADR-0004(캐시 키)와
ADR-0007(정규형은 트리다)이 이미 세운 결정을 **실행**했고, 그중 하나의 **설명이 틀린
것을 고쳤다**(§5). 계획 §7 의 규율은 *"ADR 은 종료 시점에 발행한다"* 이지
*"종료마다 발행한다"* 가 아니다.

움직인 문서는 셋이다:

  · `docs/adr/0007-the-normal-form-is-a-tree.md` — 정정을 덧붙였다(§5)
  · `corpus/criteria.toml` `[s0.grammar]`·`[s0.secondary]` — 정정을 덧붙였다(`af157e7`)
  · `crates/pal-extract/src/lib.rs` `GRAMMAR_REV` — 축 비대칭의 판단(§7)
