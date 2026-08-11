# T2 — 라벨 판정표 (소유자가 채운다)

> ## ⛔ 아직 채우지 마라 — 2026-08-11
>
> **이 표에 결함이 둘 있고 처분이 정해지지 않았다.**
>
> 1. **고유 좌표가 28개다. 30이 아니다.** 층화 표집이 층을 가로질러 같은 좌표를
>    두 번 뽑았다 — 시퀀스 4·12 와 15·19 가 각각 같은 건이다. 이슈가 요구한
>    "판정 30건"이 이 표로는 채워지지 않는다.
> 2. **재질문율은 이 표로 잴 수 없다.** 심은 중복의 자리가 소유자에게 공개됐다
>    (결함 보고 과정에서 났다). 어디가 중복인지 아는 상태의 판정은 재질문율이 아니다.
>
> **소요 시간 측정은 아직 유효하다** — 그쪽은 블라인드를 요구하지 않는다.
>
> 처분 선택지와 각각의 대가: [`docs/instructions/2026-08-11-pending-decisions.md`](../../docs/instructions/2026-08-11-pending-decisions.md) §1
>
> 판단이 내려지면 이 블록을 지우고 진행한다.

**후보는 `corpus/tasks/label-candidates.toml`에 고정되어 있고 그 커밋(`4d0c97e`)이 측정 시작보다 앞선다.**
이 표를 채우는 것이 T2의 남은 절반이다.

## 규칙

1. **후보를 찾는 시간은 재지 않는다.** 후보는 이미 고정됐다 — 그것이 [R-23](../../docs/plan/00-risks.md#r-23)의 대응이다.
2. **아래 33건을 순서대로** 판정한다. 건너뛰거나 되돌아가지 않는다.
3. 각 건마다 묻는 것은 둘이다 — **이것이 인가 가드인가** · **근거 좌표는 어디인가**.
4. 건당 소요를 초 단위로 적는다. 시계는 그 건을 열 때 시작해 답을 적을 때 멈춘다.
5. **판정 중에 `label-candidates.toml`의 `[answer_key]`를 열지 않는다.** 열면 재질문율이 무효가 된다.
6. 어떤 건에서 *다시 물었다*(같은 후보를 앞에서 본 것 같아 확인했다)면 `재질문` 칸에 `○`.

> 33건 중 3건은 앞에 나온 것과 **같은 후보**다. 어느 것인지는 알려주지 않는다 — 그것이 재질문율의 측정 방법이다.

## 판정표

| # | 후보 | 가드? | 근거 좌표 | 소요(초) | 재질문 |
|---|---|---|---|---|---|
| 1 | `auth/config/AccountSecurityProperties.kt:102` **validate** | | | | |
| 2 | `auth/web/dto/AuthenticationDto.kt:267` **validate** | | | | |
| 3 | `common/util/ValidationUtil.kt:140` **requireNonNullValue** | | | | |
| 4 | `config/MultiTenantTransactionManager.kt:166` **validateTenantRegistration** | | | | |
| 5 | `permission/aspect/PermissionCheckAspect.kt:71` **checkResourcePermission** | | | | |
| 6 | `shared/service/FlywayProvisioningService.kt:136` **validateSchema** | | | | |
| 7 | `auth/core/service/DwpAuthService.kt:21` **validateDwpTokenFromRequest** | | | | |
| 8 | `auth/jwt/filter/JwtAuthenticationFilter.kt:342` **validateDwpSession** | | | | |
| 9 | `automation/connector/service/OpenApiSchemaService.kt:148` **validateConnectorSpec** | | | | |
| 10 | `common/util/ValidationUtil.kt:154` **requireNoNullElements** | | | | |
| 11 | `permission/aspect/PermissionCheckAspect.kt:71` **checkResourcePermission** | | | | |
| 12 | `config/MultiTenantTransactionManager.kt:166` **validateTenantRegistration** | | | | |
| 13 | `permission/aspect/IntegratedPermissionCheckAspect.kt:137` **checkRbacFeaturePermissions** | | | | |
| 14 | `auth/filter/RateLimitFilter.kt:53` **RateLimitFilter** | | | | |
| 15 | `automation/dashboard/model/dto/DashboardDto.kt:22` **ProcessSummaryDto** | | | | |
| 16 | `config/TenantInterceptor.kt:25` **TenantInterceptor** | | | | |
| 17 | `shared/i18n/model/dto/MessageResourceDto.kt:9` **MessageResourceDto** | | | | |
| 18 | `auth/jwt/filter/JwtAuthenticationFilter.kt:50` **JwtAuthenticationFilter** | | | | |
| 19 | `automation/dashboard/model/dto/DashboardDto.kt:22` **ProcessSummaryDto** | | | | |
| 20 | `commons/audit/annotation/AuditLog.kt:24` **AuditLog** | | | | |
| 21 | `automation/connector/service/ConnectorService.kt:67` **TenantTransactional** | | | | |
| 22 | `config/TenantInterceptor.kt:25` **TenantInterceptor** | | | | |
| 23 | `automation/process/tracking/ProcessChangeTrackingProvider.kt:25` **TenantTransactional** | | | | |
| 24 | `code/service/CodeService.kt:384` **TenantTransactional** | | | | |
| 25 | `permission/service/impl/PermissionServiceImpl.kt:264` **TenantTransactional** | | | | |
| 26 | `user/service/impl/TenantUserServiceImpl.kt:42` **TenantTransactional** | | | | |
| 27 | `auth/systempat/web/controller/SystemPatController.kt:83` **AuthenticationPrincipal** | | | | |
| 28 | `automation/llm/service/McpToolConfigService.kt:29` **TenantTransactional** | | | | |
| 29 | `automation/servicetask/service/ServiceTaskService.kt:176` **TenantTransactional** | | | | |
| 30 | `code/tracking/CodeCategoryChangeTrackingProvider.kt:43` **AuditLog** | | | | |
| 31 | `notification/service/NotificationService.kt:87` **TenantTransactional** | | | | |
| 32 | `role/controller/RoleGroupController.kt:404` **TenantTransactional** | | | | |
| 33 | `auth/systempat/web/controller/SystemPatController.kt:83` **AuthenticationPrincipal** | | | | |

## 채우고 나서

- 건당 소요 33개 · 합계 · 중앙값을 `docs/gates/preflight.md` §T2에 적는다.
- `[answer_key]`를 열어 중복 3건의 자리를 확인하고 **재질문율 = 다시 물은 건수 / 3**을 적는다.
- 판정이 감당 불가한 시간을 먹으면 **감사(F15)·구조 평가(F20)를 산출 목록에서 내린다** — 그것이 반증 시 처분이다.
- **편향 방향은 이미 적혀 있다**(`label-candidates.toml`의 `[bias_direction]`). 결과와 함께 게이트 기록으로 옮긴다.
