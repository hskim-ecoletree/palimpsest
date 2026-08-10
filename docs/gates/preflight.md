# preflight 게이트 — 판정 기록

[P0-preflight](../plan/features/P0-preflight.md)의 작업별 판정을 여기에 남긴다.
각 항목은 **통과 · 반증 · 대조 불가** 셋 중 하나를 명시한다. 생략하고 다음으로 가는 것이
이 계획의 가장 조용한 실패 경로다([계획 §7](../plan/README.md)).

**측정값은 실측만 적는다.** 재지 못한 것은 대표값으로 채우지 않고 "못 쟀다"고 적는다.

| 작업 | 판정 | 기록 |
|---|---|---|
| T1 boxwood 작업본 복원 | **통과** | [§T1](#t1--boxwood-작업본-복원) |
| T7 Kotlin 파싱 사전 측정 | **통과** (단서 있음) | [§T7](#t7--kotlin-파싱-사전-측정) |

---

## T1 — boxwood 작업본 복원

**판정: 통과**

이슈 [#2](https://github.com/hskim-ecoletree/palimpsest/issues/2) · 근거 [R-20](../plan/00-risks.md#r-20)

### 인수 기준 대조

| 기준 | 결과 |
|---|---|
| 복원된 저장소에서 `git log`가 이력을 보여준다 | **충족** — 저장소 12개 전부에서 이력이 나온다 (아래 표) |
| 실패 시 대체 코퍼스 선정 | **해당 없음** — 복원이 성공했다 |

### 무엇을 어떻게 복원했나

| | |
|---|---|
| 원본 | `~/dev/projects/boxwood-workspace_backup.zip` (1.5GB, 2026-01-31 작성) |
| 복원 위치 | `~/dev/projects/boxwood/` |
| 명령 | `unzip -q -o boxwood-workspace_backup.zip -x '__MACOSX/*' -d ~/dev/projects/` |
| 종료 코드 | 0 |

**`__MACOSX/`를 제외했다.** 아카이브 항목 575,305개 중 276,422개가 macOS 리소스 포크
(`._*`)였다. 저장소 내용이 아니라 zip 생성 환경의 산물이므로 제외했고, 이 제외가
복원 판정에 영향을 주지 않는다 — git 객체는 전부 `.git/` 아래에 있고 그것은 제외 대상이 아니다.

### 복원 결과 — 저장소별 이력

| 저장소 | HEAD | 커밋 수 |
|---|---|---|
| `boxwood` (최상위) | `main` | 51 |
| `automation-engine` | `master` | 129 |
| `boxwood-external-task-client-teams` | `main` | 5 |
| `boxwood-packages` | `main` | 160 |
| `external-client` | `mig/modern` | 84 |
| `frontend` | `main` | 1,183 |
| `portal-backend` | `branch/dwp` | 1,174 |
| `portal-backend-aa-task` | `feat/aa-task` | 874 |
| `worktrees/feat-openapi-connector/boxwood-packages` | `feat/openapi-connector` | 163 |
| `worktrees/feat-openapi-connector/external-client` | `feat/openapi-connector` | 84 |
| `worktrees/feat-openapi-connector/frontend` | `feat/openapi-connector` | 1,117 |
| `worktrees/feat-openapi-connector/portal-backend` | `feat/openapi-connector` | 1,006 |

`portal-backend` 이력 범위: 2025-07-25 ~ 2026-01-29.
최상위 저장소 remote: `https://github.com/hskim-ecoletree/boxwood-workspace.git`.

### 복원본의 규모

| | 값 |
|---|---|
| 총 크기 | 4.0GB |
| 총 파일 | 258,521 |
| `node_modules` 제외 | 24,261 |

확장자 구성 (`node_modules`·`build`·`.git` 제외, 상위 8):

| 확장자 | 수 |
|---|---|
| `.class` | 2,043 |
| `.kt` | 1,734 |
| `.java` | 1,267 |
| `.md` | 990 |
| `.ts` | 920 |
| `.svelte` | 662 |
| `.sql` | 516 |
| `.js` | 476 |

계획 §5.2가 boxwood를 "Kotlin/Spring/Exposed + Svelte"로 적은 것과 실측이 어긋나지 않는다.

### 후속 작업이 알아야 할 것 둘

1. **`worktrees/` 넷은 독립 클론이 아니라 진짜 git worktree다.** `.git`이 디렉터리가 아니라
   `gitdir: /Users/incognito/dev/projects/boxwood/<repo>/.git/worktrees/<repo>` 를 담은 파일이다.
   **절대 경로가 박혀 있으므로 복원 위치가 `~/dev/projects/boxwood/`가 아니면 이 넷의 이력이 끊긴다.**
   이번 복원은 그 경로와 일치해서 살아났다 — 우연이며, 다른 곳에 다시 풀면 재현되지 않는다.
2. **`worktrees/`는 상위 저장소의 다른 브랜치를 담고 있어 내용이 중복된다.** 파일을 세는 작업
   (T7 등)은 중복 포함/제외를 명시해야 한다. `.kt` 기준 전체 1,734개 중 `worktrees/` 몫이 612개다.

### 재현에 필요한 것

zip 원본은 저장소에 커밋되지 않는다(1.5GB). 위 명령과 경로가 재현 절차의 전부다.

---

## T7 — Kotlin 파싱 사전 측정

**판정: 통과 — 단, 파일 수준 성공률이 실제 추출 가능률을 과대평가한다**

이슈 [#32](https://github.com/hskim-ecoletree/palimpsest/issues/32) · 근거 [R-03](../plan/00-risks.md#r-03)

반증 조건은 "Kotlin 파싱 성공률이 낮으면 판정 코퍼스를 TypeScript로 교체"였다.
**성공률은 낮지 않다. 교체하지 않는다.** 다만 아래 §단서의 관측은 F07이 알아야 한다.

### 측정 환경

| | |
|---|---|
| 파서 | `tree-sitter` CLI 0.26.12 (Homebrew `tree-sitter-cli`) |
| 문법 | `tree-sitter-grammars/tree-sitter-kotlin` @ `3dea6dfa9c0129deb7c4315afbda806c85c41667` (2025-01-16) |
| 대상 | [T1](#t1--boxwood-작업본-복원)에서 복원한 boxwood의 `.kt` 파일 |
| 코드 | 쓰지 않았다. CLI만 썼다 |

**`brew install tree-sitter`는 라이브러리만 깐다. CLI는 `tree-sitter-cli`가 별도 포뮬러다.**

`.kts`는 0개라 측정 대상이 없다.

### 인수 기준 ① — 성공 / ERROR 노드 비율

**파일 수준** (`tree-sitter parse --quiet --stat`)

| 집합 | 대상 | 성공 | 실패 | 성공률 |
|---|---|---|---|---|
| `worktrees/` 제외 | 1,122 | 1,066 | 56 | **95.01%** |
| 전체 (`worktrees/` 포함) | 1,734 | 1,647 | 87 | **94.98%** |

`worktrees/`는 상위 저장소의 다른 브랜치라 내용이 중복된다([§T1](#t1--boxwood-작업본-복원)).
**아래 수치는 별도 표기가 없으면 `worktrees/` 제외 1,122개 기준이다.**

실패 87건(전체 기준)의 유형: **ERROR 노드 76건 · MISSING 노드 11건**.

**노드 수준**

| 집합 | 명명 노드 총수 | ERROR 노드 | ERROR 노드 비율 |
|---|---|---|---|
| `worktrees/` 제외 | 668,686 | 71 | **0.0106%** |
| 전체 | 1,025,514 | 113 | 0.0110% |

명명 노드(named node) 수는 `tree-sitter parse --no-ranges`의 S-표현식 출력 행 수로 셌다.
익명 토큰은 S-표현식에 나오지 않으므로 이 분모에 포함되지 않는다.

**MISSING 노드의 개수는 못 쟀다.** `tree-sitter parse`는 MISSING 노드를 S-표현식 본문에
출력하지 않고 파일당 첫 건만 진단 행으로 보고한다. 따라서 **MISSING을 가진 파일이 11건**
이라는 것까지만 실측이고, MISSING 노드가 몇 개인지는 이 도구로 재지 못했다.
추정값으로 채우지 않는다.

### 실패 87건의 원인 — 문법 결손 둘이 전부를 설명한다

| 구문 | 해당 파일 | 최소 재현 |
|---|---|---|
| **후행 람다 뒤의 infix 호출** — Exposed 관용구 `T.insert { … } get T.id` | 73 | `fun f() { val i = T.insert { x } get T.id }` → 실패 |
| **대괄호 리터럴** — 어노테이션 클래스의 `= []` | 14 | `annotation class A(val x: Array<B> = [])` → 실패 |
| 둘 다 없음 | **0** | |

73 + 14 = 87로 겹침 없이 전수를 덮는다. 두 구문 각각이 단독으로 파싱 실패를 일으키는 것을
최소 재현으로 확인했다. **다만 개별 파일의 실패 원인이 반드시 그 구문이라는 것까지 파일
단위로 검증하지는 않았다** — 동시 출현과 최소 재현까지가 실측이다.

실패는 저장소별로 뭉쳐 있다: `portal-backend` 27 · `portal-backend-aa-task` 22
(+ `worktrees/portal-backend` 27). Exposed DSL을 쓰는 리포지터리 구현체에 집중된다.

참고로 통과한 구문들(실측): 클래스 주생성자+상위타입 · 다중행 체인 후행 람다 · 행끝 infix
연속 · context receiver · trailing comma · `@JvmInline value class` · sealed interface.
`when` 가드(Kotlin 2.1)는 실패하지만 boxwood에는 나타나지 않는다.

### 인수 기준 ② — 표본 20파일 최상위 선언 추출 정확도 (손으로 센 값과 대조)

**표본 규칙 (결과를 보기 전에 고정)**: `worktrees/` 제외 `.kt` 1,122개를 경로 오름차순
정렬 → 56번째마다 → 앞 20개. 표본 20건 중 3건이 파싱 실패 파일이다.

**도구 쪽 추출**: `tree-sitter query`로 `source_file`의 직계 자식 중
`class_declaration` · `function_declaration` · `object_declaration` · `type_alias` ·
`property_declaration`을 뽑고 이름을 캡처한다. 이 다섯이 Kotlin 최상위 선언 10종
(class/interface/object/enum/data/annotation/typealias/fun/val/var)을 모두 덮는 것을
먼저 확인했다.

**손 계수**: 20파일 전부를 읽어 최상위 선언을 셌다.

| # | 파일 | 손 | 도구 | 일치 |
|---|---|---|---|---|
| 1 | `AuditLog.kt` (파싱 실패) | 1 | 1 | ○ |
| 2 | `UserTokenRepository.kt` | 1 | 1 | ○ |
| 3 | `ProcessGlobalVariableRepository.kt` | 1 | 1 | ○ |
| 4 | `CodeDslRepository.kt` | 1 | 1 | ○ |
| 5 | `ResultsDsl.kt` | 17 | 17 | ○ |
| 6 | `OrganizationGroupRepository.kt` | 1 | 1 | ○ |
| 7 | `ServiceTaskStatisticsDto.kt` | 1 | 1 | ○ |
| 8 | `TenantAdminService.kt` | 1 | 1 | ○ |
| 9 | `SecurityEventServiceTest.kt` | 1 | **0** | **✗** |
| 10 | `RefreshTokenFamilyDslRepository.kt` (파싱 실패) | 2 | **1** | **✗** |
| 11 | `ClaudeClient.kt` | 1 | 1 | ○ |
| 12 | `ProcessGlobalVariableDslRepository.kt` (파싱 실패) | 1 | **0** | **✗** |
| 13 | `AuditLogRepository.kt` | 1 | 1 | ○ |
| 14 | `CookieSecurityProperties.kt` | 1 | 1 | ○ |
| 15 | `TenantResetResponse.kt` | 1 | 1 | ○ |
| 16 | `GetProcessInfoTool.kt` | 4 | 4 | ○ |
| 17 | `OrganizationMembershipRepository.kt` | 1 | 1 | ○ |
| 18 | `ProcessingLogResponseDto.kt` | 2 | 2 | ○ |
| 19 | `TenantAdminRoleRepository.kt` | 1 | 1 | ○ |
| 20 | `SystemPatScopeTest.kt` | 1 | 1 | ○ |
| | **합계** | **41** | **38** | |

| | 값 |
|---|---|
| 재현율 (도구가 잡은 실제 선언 ÷ 손 계수) | 38/41 = **92.68%** |
| 정밀도 (실제인 것 ÷ 도구가 낸 것) | 38/38 = **100%** — 없는 선언을 만들어내지 않았다 |
| 파일 단위 완전 일치 | 17/20 = **85%** |

`ResultsDsl.kt`(17건)는 이름까지 전부 일치했다. 누락 3건은 전부 **누락**이고 **오검출은 0건**이다.

### 단서 — 조용한 오파싱: 성공률이 추출 가능률을 과대평가한다

표본 9번 `SecurityEventServiceTest.kt`는 **파싱에 성공한다**(실패 0, ERROR 노드 0).
그런데 최상위 `class`가 `class_declaration`이 아니라 `annotated_expression` →
`infix_expression`으로 붙는다 — `class`가 식별자로 먹혀 클래스 선언이 통째로 사라진다.
**오류를 내지 않으므로 성공 집계에 들어간다.**

그래서 이것을 코퍼스 전수로 셌다 (1,122개):

| | 파일 수 | |
|---|---|---|
| 최상위 선언 0건 | 64 | |
| ├ 파싱 실패한 것 | 47 | 예상된 손실 |
| └ **파싱 성공했는데 0건** | **17** | **조용한 오파싱** |
| 선언을 하나라도 뽑을 수 있는 파일 | 1,058 | **94.30%** |

조용한 오파싱 17건은 **전부 `src/test/` 아래**이고, 17건 모두 실제로는 최상위 클래스를
갖고 있다(육안 확인). 파싱 성공 파일 1,066건의 **1.59%**다.

이 17건에서 최상위 클래스를 잃게 만드는 정확한 구문은 **특정하지 못했다.** 어노테이션
조합(`@ExtendWith`+`@DisplayName`) · 백틱 함수명 · `@Nested inner class` · `lateinit`
· `argumentCaptor<T>()` · 3중따옴표 문자열은 각각 단독으로는 재현되지 않는다. 조합에서
발생하는 것으로 보이나 **최소 재현을 못 만들었고, 못 만들었다고 적는다.**

전수 기준 도구가 뽑은 최상위 선언은 1,122파일에서 **2,241건**이다.

### F07이 이 기록에서 가져가야 할 것 셋

1. **"파싱 성공률 95%"를 그대로 쓰면 안 된다.** palimpsest가 실제로 필요한 것은 선언 추출이고
   그 기준의 수치는 **94.30%**다. 차이 0.7%p는 오류를 내지 않고 사라진다.
2. **실패는 무작위가 아니라 구문에 뭉쳐 있다.** Exposed의 `} get` 관용구 하나가 실패의
   84%(73/87)를 만든다. 문법을 고치거나 그 구문을 우회하면 실패율이 한 자리로 떨어진다.
   비용 대비 효과가 큰 지점이다.
3. **조용한 오파싱은 detect-only로 못 잡는다.** 파서가 오류를 보고하지 않으므로, 추출 결과가
   비었는지를 별도로 확인하는 장치가 없으면 그 파일은 "선언 없는 파일"로 조용히 취급된다.

### 재현

```bash
brew install tree-sitter-cli
git clone https://github.com/tree-sitter-grammars/tree-sitter-kotlin   # 3dea6df
cd tree-sitter-kotlin && tree-sitter build          # parser-directories에 등록 후
find ~/dev/projects/boxwood -name '*.kt' | grep -v /worktrees/ | sort > kt.txt
tree-sitter parse --quiet --stat --paths kt.txt
```

`grep`으로 아카이브 목록·파싱 출력을 다룰 때 **`LC_ALL=C`와 `grep -a`가 필요하다** —
경로에 비UTF-8 바이트가 섞여 있어 기본 설정에서는 grep이 바이너리로 판단하고 조용히 0을 낸다.
