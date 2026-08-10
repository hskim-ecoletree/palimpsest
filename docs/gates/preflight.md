# preflight 게이트 — 판정 기록

[P0-preflight](../plan/features/P0-preflight.md)의 작업별 판정을 여기에 남긴다.
각 항목은 **통과 · 반증 · 대조 불가** 셋 중 하나를 명시한다. 생략하고 다음으로 가는 것이
이 계획의 가장 조용한 실패 경로다([계획 §7](../plan/README.md)).

**측정값은 실측만 적는다.** 재지 못한 것은 대표값으로 채우지 않고 "못 쟀다"고 적는다.

| 작업 | 판정 | 기록 |
|---|---|---|
| T1 boxwood 작업본 복원 | **통과** | [§T1](#t1--boxwood-작업본-복원) |
| T7 Kotlin 파싱 사전 측정 | **통과** (단서 있음) | [§T7](#t7--kotlin-파싱-사전-측정) |
| T8 재발 사례 5건 확보 | **통과** | [§T8](#t8--재발-사례-5건-확보) |
| T9 조달 가능성 실측 | **반증** | [§T9](#t9--조달-가능성-실측) |
| T10 여정·결함의 올라탈 곳 | **반증** (ⓑ 여정) — ⓐ 결함은 통과 | [§T10](#t10--여정결함의-올라탈-곳) |
| T11 G7 대조 장치 등록 | **통과** — 지표 4 중 3 등록, 1 기권 | [§T11](#t11--g7-대조-장치-등록) |

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

---

## T8 — 재발 사례 5건 확보

**판정: 통과**

이슈 [#33](https://github.com/hskim-ecoletree/palimpsest/issues/33) · 근거 [R-18](../plan/00-risks.md#r-18)

산출: [`corpus/tasks/recurrence.toml`](../../corpus/tasks/recurrence.toml).

### 인수 기준 대조

| 기준 | 결과 |
|---|---|
| 5건이 `corpus/tasks/recurrence.toml`에 커밋된다 | **충족** — 5건 |
| 각 건이 (커밋, 파일, 심볼명, 떴어야 할 규칙) 네 열을 모두 갖는다 | **충족** — 5건 전부. 심볼을 못 찾아 파일 수준으로 물러난 건은 0건 |

### 선정 규칙을 **먼저** 커밋했다

R-18이 경계하는 것은 결과를 보고 사례를 고르는 것이다. 그래서 규칙만 담은 파일을
먼저 커밋하고 그 해시를 등록 증거로 삼았다.

| | |
|---|---|
| 규칙 등록 커밋 | `10d0e37b` — `[selection]` 절만 있고 사례는 비어 있다 |
| 규칙 고정 전에 본 것 | **어떤 산출물이 존재하는가**(저장소 크기·커밋 접두사 분포·ADR 존재 여부)뿐 |
| 규칙 고정 후에 잰 것 | 재발 클래스, 사례, 좌표 |

**`touch`를 아직 돌릴 수 없으므로 볼 수 있는 결과가 애초에 없었다** — palimpsest에는 코드가
없다. 그러나 규칙을 뒤에 쓰면 사례에 맞춰 규칙을 쓰게 되므로 순서를 지켰다.

### 무엇을 어떻게 골랐나

| | |
|---|---|
| 대상 | `~/dev/projects/ditto` @ `aded7ce7f88f` (main, 2026-07-24) |
| 모집단 U | `--no-merges`, 제목이 `fix`/`revert`로 시작, 스코프 `(docs)`·`비-코드` 제외 → **149건** |
| 클래스 축 | 파일이 아니라 **증상 토큰** — 같은 파일을 여러 번 고친 것이 같은 종류의 실수를 뜻하지 않는다 |
| 후보 클래스 | `|C(t)| ≥ 3` 인 토큰 **19개** |
| 검사 순서 | `|C(t)|` 내림차순 → 최초 커밋 시각 오름차순 → 사전순 |
| 실제로 검사한 것 | 상위 4개 (`handoff` → `verdict` → `completion` → `changed_files`). 5건이 여기서 찼고 나머지 15개는 검사하지 않았다 |
| 최초 발생 제외 | 최초는 재발이 아니다 — 그 시점에 `touch`가 띄울 선행 결정이 아직 없다 |

`completion`(3번째)에서는 동일 불변식을 2건 이상 묶을 수 없어 클래스를 버렸다. 건너뛴 것이
아니라 검사하고 탈락시켰다.

### 규칙이 정하지 않았던 것 하나 — 채택의 '단위'

하나의 불변식이 여러 토큰 클래스에 걸쳐 나타난다. 규칙 문언은 채택 단위를 정하지 않았고
해석이 둘이다.

| | |
|---|---|
| **해석 B (채택)** | 클래스는 검사 **순서**만 정하고, 채택 단위는 불변식. 구성원은 U 전체에서 모은다 |
| 해석 A | 클래스 내부로 한정. 같은 불변식의 최초 발생이 클래스마다 반복 제외된다 |

**두 해석 모두 5건을 채우고, 5건 중 4건이 같다.** 다른 것은 한 건뿐이다
(B: `0aa73f5c` ↔ A: `7a261606`). 해석이 결과를 만들지 않았다.

### 재발 불변식 둘

| | I1 | I2 |
|---|---|---|
| 시드 클래스 | `handoff` | `changed_files` |
| 규칙 | work item의 AC 판정은 `deriveAcVerdicts`(그래프 파생) 하나가 정본이다. 다른 완료 경로는 같은 인자로 같은 함수를 부르거나 그 결과를 그대로 반영해야 한다 | 워킹트리 `git status --porcelain`을 `changed_files`·tree-clean 판정의 소스로 쓰는 **모든** 경로는 외래 변경(`started_untracked_baseline`)을 배제해야 한다 |
| 최초 발생 | `4f8d9289` (2026-06-07) | `6f04625e` (2026-06-20) |
| 총 발생 | 4 | 5 |
| 재발 | **3** | **4** (그중 2건은 5건이 차서 쓰지 않았다) |

### 사례 5건

| # | 커밋 | 날짜 | 파일 | 심볼 | 떴어야 할 규칙의 근거 |
|---|---|---|---|---|---|
| 1 | `0aa73f5c` | 06-24 | `src/core/work-item-handoff.ts` | `writeWorkItemHandoff` | `4f8d9289` — 같은 좌표의 주석 |
| 2 | `264c3371` | 06-28 | `src/core/completion-store.ts` | `mirrorAcceptanceVerdicts` | `4f8d9289` |
| 3 | `992dcd0e` | 07-10 | `src/core/work-item-handoff.ts` | `writeWorkItemHandoff` | `4f8d9289` · `0aa73f5c` |
| 4 | `d2940f7f` | 07-11 | `src/core/git.ts` | `dequotePorcelainPath` | `6f04625e` |
| 5 | `9843bd91` | 07-14 | `src/core/work-item-handoff.ts` | `collectChangedFiles` | `6f04625e` · `d2940f7f` |

**케이스 1과 3은 같은 파일의 같은 심볼, 같은 호출 지점이다.** 3주 간격으로 같은 규칙이
두 번 깨졌다. 그리고 그 규칙은 그 줄 바로 위에 주석으로 있었다:

> `derive the per-AC verdicts from the graph (same evidence-gated source as ditto autopilot complete) so the two completion paths AGREE — handoff cannot clobber a graph-based pass with a stale work-item-AC partial.` — `4f8d9289`가 심은 주석

**케이스 5는 커밋 자신이 재발이라고 적었다:**

> `근원: d2940f7(#22)이 changed_files 생산 3경로 중 autopilot-loop의 둘만 started_untracked_baseline에 배선하고, 이 terminal 재계산 경로는 놓쳤다. 캡처된 baseline은 정상 존재했으나 이 경로가 배제 집합을 참조하지 않아 재발.`

### 어긋난 것 하나 — 좌표 규칙이 결함의 소재지를 빗나간다

케이스 4에서 규칙(`변경 줄 수가 가장 많은 src 파일`)이 고른 것은 `src/core/git.ts`의
새 헬퍼 `dequotePorcelainPath`다. 그런데 커밋 메시지가 지목하는 결함 위치는
`src/core/autopilot-loop.ts:3263`과 `:731`이다.

**가장 많이 바뀐 파일은 수리의 소재지이지 결함의 소재지가 아니다.** 규칙대로 적고
어긋남을 남긴다. 5건 중 1건에서 발생했다.

### F11이 이 표에서 가져가야 할 것 셋

1. **재발 5건 중 3건에서 깨진 규칙이 그 좌표에 이미 글자로 있었다** — 주석 또는 직전 커밋.
   `touch`가 할 일은 새 지식을 만드는 게 아니라 **있는 것을 그 순간에 옮기는 것**이다.
   뒤집어 말하면 글자가 코드 옆에 있어도 읽히지 않았고, 그것이 F11의 존재 이유다.
2. **재발의 지배적 형태는 '몰랐다'가 아니라 '경로 하나를 빠뜨렸다'다.** I1·I2 둘 다
   그렇다. `touch(좌표)` 하나로는 부족하고 *"이 규칙을 지켜야 하는 다른 좌표가 어디인가"* 라는
   역방향 질의가 필요하다. F05의 `BOUND_BY` 역색인이 그 질의를 받는 자리다.
3. **케이스 4의 어긋남** — 결함 좌표를 "가장 많이 바뀐 파일"로 정하면 헬퍼를 가리킨다.

### 남는 잔여

**이 5건은 ditto 하나에서 나왔다.** 코퍼스 편의 표본이고([R-19](../plan/00-risks.md#r-19)와 같은
성질), boxwood에서 같은 규칙을 돌린 결과는 아직 없다. 여기서 재는 것은 "재발이 존재하는가"
까지이고 "재발이 프로젝트 일반의 성질인가"는 이 표가 답하지 않는다.

### 재현

```bash
cd ~/dev/projects/ditto
LC_ALL=C git log --no-merges --format='%H%x09%at%x09%s' aded7ce7f88f > all.tsv
# 모집단·클래스 산출은 corpus/tasks/recurrence.toml 의 [selection] 그대로
```

---

## T9 — 조달 가능성 실측

**판정: 반증**

이슈 [#34](https://github.com/hskim-ecoletree/palimpsest/issues/34) · 근거 [R-25](../plan/00-risks.md#r-25)

산출: [`corpus/manifest.toml`](../../corpus/manifest.toml)의 `[procurement]` 절.

### 무엇을 어떻게 쟀나

코드를 쓰지 않았다. CI 설정 파일과 **GitHub Actions의 실제 실행 기록·로그·아티팩트**,
그리고 code-scanning API를 읽었다.

**①은 설정 파일의 존재가 아니라 실행 이력으로 판정했다.** 워크플로 파일이 있는 것과
그것이 도는 것은 다르고, 조달은 남이 이미 하고 있어야 성립하는 것이기 때문이다([R-25](../plan/00-risks.md#r-25)).

### 네 항목 — 코퍼스별

| | ① CI가 도는가 | ② 무엇이 도는가 | ③ 산출 형식 | ④ 파일:라인 |
|---|---|---|---|---|
| **boxwood** | 부분 — 저장소 7개 중 분석 CI는 2개 | 린터·타입체커 (frontend만) · SAST 없음 · 테스트 없음 | 콘솔 텍스트(로그)뿐 · SARIF 0 · 아티팩트 0 | 로그에서만 |
| **ditto** | **예** — CI 319회 · npx-smoke 103회 | biome lint · `tsc --noEmit` · 자체 게이트 2 · SAST 없음 · 테스트 없음 | 콘솔 텍스트(로그)뿐 · SARIF 0 · 아티팩트 0 | **예** (규칙 종류에 따라 다름) |
| **palimpsest** | 아니오 — `.github/` 없음, 실행 0건 | 없음 | 해당 없음 | 해당 없음 |
| **규모 코퍼스** | **대조 불가** | 미선정(T5) | | |

| 집계 | 값 |
|---|---|
| SAST 조달원을 가진 코퍼스 | **0 / 4** |
| 좌표 있는 진단을 실제로 내고 있는 코퍼스 | **1 / 4** (ditto) |
| 진단을 아티팩트로 보존하는 코퍼스 | **0 / 4** |
| SARIF를 내는 코퍼스 | **0 / 4** |

### boxwood — 분석 CI는 사실상 없다

저장소 7개 중 워크플로가 있는 것은 5개. 그중 **5개가 CD**(docker build + scp/ssh 배포)이고
분석을 하는 것은 둘뿐이다.

| 저장소 · 워크플로 | 실행 | 성공 | 실패 | 최근 |
|---|---|---|---|---|
| `boxwood-packages` · CI | 85 | 84 | 1 | 2026-05-19 |
| `boxwood-portal-svelte` · **Automation CI** | **2** | **0** | 1 | 2026-06-11 |
| `boxwood-portal-svelte` · CD 3종 | 98 | 93 | 3 | 2026-08-04 |
| `boxwood-portal-kotlin` · CD 2종 | 51 | 41 | 9 | 2026-07-30 |
| `boxwood-automation-engine` · CD 2종 | 22 | 20 | 1 | 2026-07-29 |
| `hanwha-boxwood-external-client` | 0 | | | 워크플로 없음 |
| `boxwood-external-task-client-teams` | 0 | | | 워크플로 없음 |
| `boxwood-workspace` | 0 | | | 워크플로 없음 |

세 가지가 걸린다.

1. **`boxwood-packages`의 CI는 `mvn clean verify -DskipTests`다.** 빌드만 하고 테스트를
   건너뛴다. 린터도 SAST도 없다. 85회 도는 것은 컴파일이 되는지 보는 것이다.
2. **린터·타입체커가 실제로 도는 워크플로는 `Automation CI` 하나이고, 그것은 지금까지
   2회 돌았으며 한 번도 성공한 적이 없다** — 1회 실패(Type check 단계), 1회 취소.
3. **T7이 파싱한 Kotlin 1,122파일이 있는 `boxwood-portal-kotlin`에는 분석 CI가 없다.**
   CD만 51회 돈다. 감사(F15)와 효과 집합(F13)의 판정 대상이 바로 이 저장소다.

분석기 설정도 없다. `detekt` · `ktlint` · `sonar` · `spotbugs` · `checkstyle` · `pmd` ·
`semgrep` · `snyk` · `trivy` — Gradle·Maven 빌드 파일 어디에도 언급이 없다. 있는 것은
`.eslintrc` 둘과 `tsconfig.json` 여덟, 전부 frontend다.

디스크에 남은 리포트도 없다. Gradle 바이너리 `test-results`, surefire `dumpstream` 1건,
빈 `test-results` 디렉터리가 전부다. **파일:라인을 담은 리포트 파일은 0건이다.**

**그럼에도 좌표는 존재한다 — 로그 안에.** Automation CI의 그 실패 실행 로그에
`svelte-check found 495 errors and 958 warnings in 206 files`가 있고,
좌표 있는 진단 행이 1,453개다 (예: `packages/ui/src/lib/basic/inputs/EcoletreeSlider.svelte:89:10`).
**아티팩트가 아니라 실행 로그이고, 로그는 보존 기간에 걸린다.**

### ditto — 조달원이 있다. 하나뿐이고, 로그뿐이다

| 워크플로 | 실행 | 실패 | 최근 |
|---|---|---|---|
| CI | **319** | 23 | 2026-07-24 |
| npx-smoke | 103 | 0 | 2026-07-24 |

`push(main)`과 **모든 PR**에서 돈다. 도는 것: `biome check`(367파일) · `tsc --noEmit` ·
`adr:guard` · `check:no-design-doc-refs`.

**테스트는 게이트에 없다.** `ci.yml` 주석이 이유를 적어 두었다 — 환경의존 테스트가 CI에서
깨져 아직 넣지 않았고, 테스트 CI 독립화는 별도 과제다.

**SAST는 없다.** `reports/codeql/`이 있지만 그것은 CodeQL 도입 조사·계획 문서이지
CodeQL 산출이 아니다. code-scanning API는 `no analysis found`를 준다.

④ 파일:라인은 **도구와 규칙 종류에 따라 갈린다**:

| 도구 | 형식 | 좌표 |
|---|---|---|
| `tsc --noEmit` | `file(line,col)` — `tests/schemas/ac-oracle.test.ts(139,30): error TS2532` | ○ |
| biome — **lint** 규칙 | `file:line:col` — `tests/gate-coverage/drive/harness.test.ts:6:46 lint/correctness/noUnusedImports` | ○ |
| biome — **format** 규칙 | `scripts/build-bin.mjs format` — 파일만, 행은 diff 거터에만 | ✗ |

실패 20건을 표본으로 보면 **19건이 Lint(biome), 1건이 Typecheck(tsc)**다. 즉 실제로
가장 자주 나오는 산출이 좌표가 가장 약한 쪽이다. biome은 `--reporter=json`/`github`로
좌표를 낼 수 있으나 **현재 설정은 쓰지 않는다.**

아티팩트는 0건이다. 여기서도 조달 경로는 로그뿐이다.

### 왜 반증인가

[F16을 P1로 올린 근거](../plan/features/F16-observation-intake.md)는
*"외부 엔진의 산물을 받으면 XL 둘 없이 감사가 선다"* 였다. 실측은 그것을 받치지 못한다.

1. **SAST 조달원이 0/4다.** 감사(F15)를 대신할 `Finding`을 낼 엔진이 어느 코퍼스에도 없다.
   GitHub 코드 스캐닝도 4개 저장소 전부 비활성이다. F16이 P1로 올라온 근거는
   *SAST 산물을 받는 것*이었고, 받을 것이 없다.
2. **F16의 승격 근거가 걸린 boxwood에 조달원이 없다.** 감사·효과 집합의 판정 대상인
   Kotlin 백엔드는 분석 CI가 0이다. 조달로 우회하려던 뿌리가 바로 그 대상에서 비어 있다.
3. **하나 있는 조달원(ditto)도 `observed` 사실을 세우기엔 형태가 약하다.** SARIF도
   아티팩트도 없고 경로는 실행 로그뿐이다. 그리고 실패의 95%(19/20)를 차지하는 biome
   format 진단은 파일까지만 준다.

**이것은 "때때로 있는 능력"이다 — [R-25](../plan/00-risks.md#r-25)가 경계한 바로 그 형태.**

### 반증의 처분 — **적용됐다 (2026-08-11, 소유자 지시)**

[P0-preflight §4](../plan/features/P0-preflight.md)와 이슈 [#34](https://github.com/hskim-ecoletree/palimpsest/issues/34)가 정한 처분:

> **[F16](../plan/features/F16-observation-intake.md)을 P2로 되돌리고 P1의 약속을 (a)(d)로 줄인다.**
> 줄었다는 사실을 [goals §0.1](../plan/00-goals.md)에 적는다.

T9의 인수 기준 자체는 `[procurement]` 기록 하나였고 이 처분은 그 밖이었다. 판정 커밋
시점에는 빚으로 남겨 두었으나, **소유자가 적용을 지시해 같은 날 반영했다.**

| 어디 | 무엇을 바꿨나 |
|---|---|
| [goals §0.1](../plan/00-goals.md) | (b) 행을 "P1 아님"으로. **P1의 약속이 (a)(d)로 줄었다는 사실을 명시** |
| [goals §2](../plan/00-goals.md) | 목표 4(감사): "조달 경로 P1 / 자체 산출 P2" → **둘 다 P2** |
| [goals §5](../plan/00-goals.md) | **G8을 발동으로 표시** |
| **[F16](../plan/features/F16-observation-intake.md) → 둘로 분할** | 수용 API는 **P1로 남고**, 조달이 **[F16b](../plan/features/F16b-engine-procurement.md)(P2)** 로 갈라졌다 (아래 §분할) |
| [계획 README](../plan/README.md) | 우선순위 표에서 F16을 P2 구역으로 · P1 정의에서 조달 삭제 · T1·T7·T9 판정 표 |
| [DESIGN D24 · §7.5](../DESIGN.md) | **D24 결정 자체는 유지**하고 F16의 P1 자리가 무너졌음을 결정 자리에 기록 |

**D24를 폐기하지 않은 이유**: 다섯 몫(결박·낡음·3분할·억제 이력·엔진 간 불일치)은
반증된 것이 아니라 **적용될 데가 없다.** 성질이 틀린 것과 환경이 빈 것은 다르고,
그 둘을 같게 적으면 환경이 바뀌었을 때 되돌릴 근거가 사라진다.

### 그 처분이 드러낸 모순과 그 해소 — F16의 분할

강등 직후 **[F21](../plan/features/F21-provider-ports.md)(P1)이 P2가 된 F16에 의존하는 모순**이 남았다.
F21의 몫은 조달만이 아니라 포트 일곱이므로 자동으로 강등되지 않는다.

**갈라 보니 모순이 아니라 문서 하나에 두 기능이 들어 있던 것이었다.** F21이 F16을
참조하는 자리 셋 중 둘이 조달이 아니라 수용 API였다:

| F21의 어디 | F16의 무엇 | T9가 반증했나 |
|---|---|---|
| §3.1 규칙 1 | *"코어는 F16의 **수용 API**로 받는다"* | 아니다 |
| §7 체크리스트 | *"F16의 **intake**로 합류, 배정 `observed` 고정"* | 아니다 |
| §2 포트 6 (관측 조달자) | 조달원 | **그렇다** |

포트 일곱 중 조달자는 하나뿐이고 나머지 여섯(언어 추출기·진입점·경계 계약·효과 어휘·
경로 규약·개념 팩)은 조달원이 없어도 성립한다. **그래서 F16을 둘로 쪼갰다:**

| | 무엇 | 우선순위 | 의존 | 규모 |
|---|---|---|---|---|
| [**F16**](../plan/features/F16-observation-intake.md) | 수용 계약 · 출처 배정 · `observed` 무효화 · 좌표 결박 | **P1** | F22 | M |
| [**F16b**](../plan/features/F16b-engine-procurement.md) | 외부 엔진 조달 — SAST·린터·SCIP + 다섯 몫 | **P2** | F16·F08 | M |

**경계는 "무엇을 받느냐"가 아니라 "받는 장치냐 받을 것을 고르는 일이냐"다.**
선행 시도가 남긴 착수 조건 다섯도 이 선으로 갈렸다 — 셋(실패≠침묵 · resolver 위치 ·
정규화)은 입력 종류와 무관해 F16으로, 둘(흐름 식별자 · SARIF 입도)은 엔진 고유라 F16b로.

**분할의 이득 하나**: 다섯 몫 중 1·2(결박·낡음)는 F16의 장치를 alert에 적용한 것이다.
장치가 P1에 서 있으므로 조달이 P2에서 착수해도 그 부분은 이미 있다.

**모순은 남지 않았다.** F21(P1) → F16(P1) · F14(P2) → F16(P1) · F16b(P2) → F16(P1).

### 뒤집을 수 있는 관측 셋 — 이 반증은 환경의 성질이지 영구 사실이 아니다

이 판정은 "조달이 원리적으로 불가능하다"가 아니라 "지금 대상 환경에서 아무도 하고 있지
않다"이다. 아래가 바뀌면 재측정 대상이다.

1. **`boxwood-portal-kotlin`에 detekt를 붙이면** Kotlin 쪽 조달원이 0에서 1이 된다.
   설정이 아예 없으므로 새로 만드는 비용이다 — 남이 이미 하고 있는 것을 받는 게 아니다.
2. **biome에 `--reporter=json`을 주면** ditto의 가장 흔한 산출이 좌표를 갖는다. 한 줄이다.
3. **아티팩트 업로드가 없다는 것이 공통 병목이다.** 셋 다 로그에만 있고, 로그는 보존
   기간에 걸린다. 조달을 실제 경로로 쓰려면 이것부터 걸린다.

**그리고 이 셋은 전부 "우리가 대상 저장소를 고쳐야 성립한다"에 해당한다.**
조달의 전제가 *남이 이미 하고 있는 것을 받는다*였다는 점에서, 셋 다 전제를 무너뜨린다.

---

## T10 — 여정·결함의 올라탈 곳

**판정: 반증 — ⓑ 여정. ⓐ 결함은 통과.**

이슈 [#35](https://github.com/hskim-ecoletree/palimpsest/issues/35) · 근거 [R-27](../plan/00-risks.md#r-27)

산출: [`corpus/tasks/outcomes.toml`](../../corpus/tasks/outcomes.toml).

선정 규칙은 [T8](#t8--재발-사례-5건-확보)과 같은 커밋 `10d0e37b`에 미리 등록했다.

### 인수 기준 대조

| 기준 | 결과 |
|---|---|
| 결함 5건·여정 3건의 지목 가능률이 `corpus/tasks/outcomes.toml`에 커밋된다 | **충족** |

### 한 줄로

| | 지목 가능률 |
|---|---|
| **결함** — 발현 좌표 | **4/5 = 80%** (전부 라인 수준) |
| **결함** — 도입 커밋 | **4/5 = 80%** — 단, 아래 §임계값 민감도 |
| **결함** — 트래커 결박 | 4/5 = 80% |
| **여정** — 진입점 좌표 | **0/3 = 0%** |

**`Defect`는 소급 결박된다. `Journey`는 안 된다.**

### ⓐ 결함 — 표본과 측정

| | |
|---|---|
| 모집단 | T8과 같은 U — ditto `aded7ce7`의 `fix`/`revert` 커밋 149건 |
| 표본 | 시각 오름차순 등간격, `g = floor(149/5) = 29` → 인덱스 0·29·58·87·116 |

| # | 커밋 | 날짜 | 발현 좌표 | 도입 커밋 | 트래커 |
|---|---|---|---|---|---|
| 1 | `add91871` | 05-24 | 라인 (45줄) | ○ `7d6451a5` 84% | ○ |
| 2 | `2550118a` | 06-08 | 라인 (6줄) | ○ `5616f46e` **100%** | **✗ 참조 없음** |
| 3 | `afcfefab` | 06-20 | **✗ `src/**` 변경 0** | ✗ 누락형 | ○ |
| 4 | `7f6b0a58` | 06-28 | 라인 (2줄) | △ `59fb3d8f` **50%** | ○ |
| 5 | `9dc1af1e` | 07-11 | 라인 (10줄) | △ `760e4763` **50%** | ○ |

#### 임계값 민감도 — 80%를 그대로 믿으면 안 된다

도입 커밋 성공 4건 중 **2건이 정확히 50%**다. 등록한 임계가 "과반(≥50%)"이라 성공으로
셌지만, 임계를 `>50%`로 바꾸면 **2/5 = 40%**로 떨어진다.

`7f6b0a58`은 삭제 줄이 2개이고 그 둘이 서로 다른 커밋에서 왔다 — "최빈 1줄"은 신호가 아니다.
**삭제 줄이 적을수록 blame 과반 판정은 무의미해진다.**

**80%는 상한이고 40%가 하한이다. 대표값 하나로 적지 않는다.**

#### 표본이 드러낸 것 셋

1. **결함이 코드가 아니라 프롬프트에 있는 경우가 5건 중 1건** (`afcfefab`).
   바뀐 것은 `agents/reviewer.md`·`agents/security-reviewer.md`와 테스트 하나뿐이고
   `src/**`는 0줄이다. **`Defect`를 코드 좌표에만 걸 수 있게 설계하면 이 종류는
   통째로 담기지 않는다.** 에이전트 하네스에서 프롬프트는 실행되는 산출물이므로
   드문 경우라고 볼 근거가 없다.
2. **트래커만으로 결함을 모으면 5건 중 1건을 잃는다** (`2550118a` — 이슈·작업항목 참조 없음).
   그리고 그 건은 도입 커밋 결박이 **100%로 가장 깨끗한** 건이다. 트래커 유무와
   결박 가능성은 무관하다.
3. **결함이 결함 수정에서 태어난다** (`9dc1af1e`의 도입 커밋 `760e4763`은 그 자체가 `fix`다).
   T8의 I1 계열과 같은 자리다.

### ⓑ 여정 — 0/3

J1(`*.journey.md`)이 정확히 3건이라 규칙대로 J2로 내려가지 않았다. **결과가 나쁘다고
모집단을 바꾸면 그것이 결과를 보고 고르는 것이다.**

| # | 여정 | 판정 | 왜 |
|---|---|---|---|
| 1 | `jrn-codex-dogfood-digest` | **실패** | `page:/dogfood`에 대응하는 제품 코드가 ditto에 없다. digest 결정성 검사용 **합성 최소 여정**이다 |
| 2 | `jrn-checkout-coupon` | **실패** (관대히 봐도 부분) | 대상이 `.git`을 따로 가진 **외부 target-app**. 그 앱 기준으로 봐도 진입점은 인라인 `<script>`의 익명 핸들러라 `repo:path#name`으로 지목 못 한다 |
| 3 | `jrn-promo-banner` | **실패** | 같은 외부 앱. 게다가 대상 요소가 **일부러 없다** — `app/promo.html`에 `의도적 실패 실증용: [data-testid=promo-banner] 요소를 일부러 넣지 않았다` |

**셋 다 `.ditto/local/` 아래의 gitignored 런타임 산물이다** — 저장소에 커밋조차 되지 않는다.
[T9](#t9--조달-가능성-실측)가 조달에서 본 것과 같은 형태다: 있긴 한데 보존되지 않는다.

#### 반증의 진짜 내용 — "없다"가 아니라 "그것도 새로 쓴 것이다"

ditto가 가진 여정 3건은 **e2e 기능을 실증하려고 사람이 새로 쓴 것**이다. 하나는 합성
최소 파일이고 둘은 그 실증을 위해 만든 장난감 앱의 여정이다.

R-27이 물은 것은 *"새 저작을 요구하지 않는 경로가 있는가"* 였다. **답은 아니오다.**
여정이 존재하는 유일한 곳에서도 그 여정은 저작의 산물이었다.

#### 규칙 밖 부가 관측 — 판정에 넣지 않는다

표본이 아니고 지목 가능률에 세지 않는다. 처분을 정할 때 쓰라고 적는다.

J2(`docs/features/<name>.md` ↔ `src/cli/commands/<name>.ts`)는 **38쌍**이 성립하고 각
명령 파일이 `export const <name>Command`라는 명명된 최상위 선언을 갖는다
(`autopilot` → `autopilotCommand`). **진입점 좌표 자체는 기계적으로 나온다.**
나오지 않는 것은 *"이 진입점들이 한 여정이다"* 라는 선언이다 — R-27 §4가 예상한 그대로다.

### 저작 뿌리는 넷이 아니라 셋이다

[R-27](../plan/00-risks.md#r-27)은 뿌리가 둘에서 넷으로 늘어 남는 것이 16분의 1이 된다고
적었다. 실측은 그것을 부분적으로 완화한다.

| 노드 | 저작이 필요한가 | 근거 |
|---|---|---|
| `Change` · `Actor` | 아니오 | git에서 결정론적 (R-27 §대응2) |
| `Defect` | **아니오** | 이 측정 — 발현 좌표 80% · 도입 커밋 80%(하한 40%) |
| `Journey` | **예** | 이 측정 — 0/3 |

**넷이 아니라 셋이고, 저작 뿌리는 하나 늘었다.** 16분의 1이 아니라 8분의 1이다.

### 처분 — **적용됐다 (2026-08-11, 소유자 지시)**

이슈 [#35](https://github.com/hskim-ecoletree/palimpsest/issues/35)와 [P0-preflight §4](../plan/features/P0-preflight.md)가 정한 처분:

> **`Journey`를 내리고 (c)를 "진입점에서 시작하는 도달 하한"으로 축소한다.
> `Defect`는 자리만 두고 필수로 만들지 않는다.** 축소했다는 사실을 [goals §0.1](../plan/00-goals.md)에 적는다.

T10의 인수 기준은 `outcomes.toml` 기록 하나였고 이 처분은 그 밖이었다. 판정 커밋 시점에는
빚으로 남겨 두었으나, **소유자가 적용을 지시해 같은 날 반영했다** —
[T9](#t9--조달-가능성-실측) 때와 같은 절차다.

**두 절 중 앞 절만 발동했다.**

| 절 | 조건 | 발동 | 왜 |
|---|---|---|---|
| `Journey`를 내리고 (c)를 축소 | 여정을 소급 결박할 수 없으면 | **예** | 진입점 좌표 0/3 |
| `Defect`는 자리만 두고 필수로 만들지 않는다 | 결함을 소급 결박할 수 없으면 | **아니오** | 발현 좌표 4/5 · 도입 커밋 4/5 — 조건절이 성립하지 않는다 |

**`Defect`는 내리지 않았다.** 측정이 [R-27 §대응3](../plan/00-risks.md#r-27)의 *"`Defect`는
올라탈 곳이 있다"* 를 확인했으므로, 그 절을 적용하는 것은 실측에 반하는 축소가 된다.

| 어디 | 무엇을 바꿨나 |
|---|---|
| [goals §0.1](../plan/00-goals.md) | (c) 행을 **"진입점에서 시작하는 도달 하한"** 으로. **줄었다는 사실을 명시**. (d) 행에는 T8·T10ⓐ가 받쳤다고 기록 |
| [R-27](../plan/00-risks.md#r-27) | 제목을 **"둘에서 셋으로"** 로. §대응3 확인 · §대응4 반증을 각 자리에 기입 |
| [DESIGN §0.8](../DESIGN.md) | (c)·(d) 행에 판정 반영 |
| [DESIGN §1.1](../DESIGN.md) | *"`Journey`·`Defect`는 저작 노동의 새 뿌리다"* → **`Journey`만**. 자리만 두는 것은 그대로이되 **이유가 갈렸다**(`Journey`는 저작이 없어서, `Defect`는 인입이 아직 없어서) |
| [DESIGN D26](../DESIGN.md) | **결정 자체는 유지**하고 `Journey`의 뿌리가 반증됐음을 결정 자리에 기록 |
| [DESIGN §15 잔여 · §15-34](../DESIGN.md) | "새 뿌리 후보 둘" → **뿌리는 `Journey` 하나**. 곱이 둘이 아니라 하나 늘었다 |
| [F22](../plan/features/F22-graph-schema.md) | §2.2 `Journey` 행·저작 노동 항목 · §4 이슈 표 · §7 체크리스트. **프롬프트 결함은 §4 잔여로만 기록**(아래) |
| [P0-preflight §4](../plan/features/P0-preflight.md) | 처분 표에 **부분 발동**을 명시 |

#### 프롬프트 결함은 잔여로만 남겼다 — 스키마를 지금 넓히지 않는다

표본 5건 중 1건(`afcfefab`)이 코드 밖에 있었다는 관측으로 `Coord`의 대상 범위를 지금
넓히지는 않는다. **관측 1건은 스키마를 바꿀 근거로 얇다.** [F22 §4](../plan/features/F22-graph-schema.md)에
잔여로 적고, 대상 범위를 실제로 정하는 것은 [T6](../plan/features/P0-preflight.md)(온톨로지
저작 가능성)의 판단으로 넘긴다.

그때까지의 대응은 하나다: **담기지 않는 결함을 세어서 표시한다.** 조용히 빠지는 것만 막는다.

### 뒤집을 수 있는 관측 — 이 반증도 환경의 성질이다

[T9](#t9--조달-가능성-실측)와 같은 성질이다. 이 판정은 *"여정을 소급 결박하는 것이
원리적으로 불가능하다"* 가 아니라 *"지금 이 코퍼스에 올라탈 여정이 없다"* 이다.

1. **E2E 스위트를 실제로 운영하는 코퍼스가 들어오면** 재측정 대상이다. ditto의 여정 3건은
   기능 실증용이었고 제품의 여정이 아니었다 — 여정을 **쓰는** 프로젝트에서는 다를 수 있다.
2. **다만 그것도 "남이 이미 하고 있는가"에 걸린다.** ditto에서 확인된 것은 여정이 존재하는
   곳에서조차 그것이 새 저작의 산물이었다는 것이고, 이 점은 코퍼스를 바꿔도 쉽게 뒤집히지 않는다.
3. **boxwood에서는 재지 않았다.** 이 판정은 ditto 하나의 실측이다([R-19](../plan/00-risks.md#r-19)와
   같은 편의 표본 문제).

### 재현

```bash
# ⓐ 표본: U를 시각 오름차순 정렬 후 인덱스 0·29·58·87·116
# ⓑ 모집단: find . -name '*.journey.md' (node_modules·dist 제외) | sort
```

측정 스크립트는 저장소에 남기지 않았다 — 규칙(`corpus/tasks/outcomes.toml`의 `[selection]`)이
재현 절차의 정본이고, 스크립트는 그것의 일회용 구현이다.

---

## T11 — G7 대조 장치 등록

**판정: 통과 — 지표 넷 중 셋을 등록하고 하나를 기권했다**

이슈 [#36](https://github.com/hskim-ecoletree/palimpsest/issues/36) · 근거 [R-29](../plan/00-risks.md#r-29)

산출: [`corpus/criteria.toml`](../../corpus/criteria.toml)의 `[outcome]` 절.

### 인수 기준 대조

| 기준 | 결과 |
|---|---|
| 지표·대조군·채점 절차가 `corpus/criteria.toml`의 `[outcome]` 절에 커밋된다 | **충족** — 지표 4(등록 3·기권 1) · 대조군 2 · 채점 절차 5단계 + 기권 규칙 |
| 커밋 시각이 어떤 결과보다 앞선다 | **충족** — 아래 §무엇보다 앞서는가 |

### 무엇보다 앞서는가 — 기준선을 **일부러 계산하지 않았다**

palimpsest에는 코드가 한 줄도 없으므로 arm B(도구 있음)의 결과는 존재할 수 없다.
그것만으로는 부족하다.

**arm A(도구 없음)의 기준선 수치도 계산하지 않았다.** 데이터는 있다 — 아래 표의 `n`이
그것을 센 것이다 — 그러나 **수치를 알고 나서 합격선을 그으면 그것이 [R-18](../plan/00-risks.md#r-18)이
경고한 자기 판정이다.** 기준선은 각 지표의 수치 합격선을 등록하는 커밋과 **같은 커밋에서**
계산하도록 절차에 못 박았다(`step_2_baseline`).

여기서 등록한 것은 **지표의 정의 · 대조군 · 채점 절차**이고 **숫자가 아니다**
([P0-preflight §5.1](../plan/features/P0-preflight.md)이 정한 두 단계 등록의 앞 단계).

### 지표 — 넷을 검토해 셋을 등록했다

단일 지표는 게이밍된다([R-29 §대응2](../plan/00-risks.md#r-29)) — 재발률만 재면 재발을
재정의하는 압력이 생긴다. 그래서 여럿을 두고, 못 재는 것은 기권으로 남겼다.

| | 지표 | 판정 | n | arm B를 회고로? |
|---|---|---|---|---|
| **M1** | 재발 검출률 | 등록 | **5** — `recurrence.toml` | **예** |
| **M2** | 계획 이탈률 | 등록 | **17** — ditto의 선언·실제 쌍 | 아니오 |
| **M3** | 수정 소요 | 등록 (신뢰도 상한 동반) | 도입 결박 4/5, 하한 2/5 | 아니오 |
| **M4** | 리뷰 지적 수 | **기권 — 대조 불가** | **0** | — |

#### M2의 데이터는 실제로 있다 — 그리고 커밋되지 않는다

계획 이탈률의 어려운 쪽은 *선언*이다. 실제 변경(`changed_files`)은 흔하지만 **착수
시점에 좌표로 선언된 계획**은 드물다. ditto에는 그것이 있다.

| | |
|---|---|
| 선언 D | autopilot 그래프의 `change_surface` (승인 시점에 동결) |
| 실제 A | `record.json`의 `changed_files` |
| 둘 다 비어 있지 않은 작업 단위 | **17건** (autopilot 그래프 보유 54건 중 · 작업 단위 총 146건) |

`acceptance_criteria`는 **좌표가 아니라 문장**이다(`statement`+`verdict`+`evidence`).
그쪽으로는 이탈률이 나오지 않는다 — 계산 가능한 것은 `change_surface` 쪽뿐이다.

**그리고 D는 gitignored `.ditto/local/` 아래에 있다.** [T9](#t9--조달-가능성-실측)의 조달원,
[T10ⓑ](#t10--여정결함의-올라탈-곳)의 여정과 **같은 형태의 취약함**이다 — 있긴 한데
보존되지 않는다. **이 작업본이 지워지면 M2의 arm A는 재현되지 않는다.** 측정 시점에
D·A 쌍을 `corpus/`로 떠 두는 것을 지표의 caveat에 적었다.

#### M4를 기권한 이유 — 리뷰 트레일이 없다

| | 값 |
|---|---|
| ditto의 PR | **1건** |
| 커밋 제목·본문에 '지적' | 3건 |
| 커밋 제목에 'finding' | 6건 |
| boxwood의 리뷰 산출 아티팩트 | 0건 ([T9](#t9--조달-가능성-실측)) |

n ≥ 5([P0-preflight §5 규칙 2](../plan/features/P0-preflight.md))를 만족할 표본이 어느
코퍼스에도 없다. **대표값으로 채우지 않고 기권했다.**

등록에서 아예 빼지 않고 `기권`으로 남긴 것은 [R-29 §대응3](../plan/00-risks.md#r-29) 때문이다 —
**"대조 불가"는 1급 판정이고, 생략하고 통과로 처리하는 것이 가장 조용한 실패 경로다.**

### 등록이 드러낸 것 둘 — 이 저울은 생각보다 약하다

#### 1. M1은 효과 추정치가 아니라 검출률이다

`recurrence.toml`의 5건은 **'재발한 것'이라는 조건으로 골랐다.** 따라서 arm A(도구 없음)의
재발률은 정의상 5/5이고, **그것은 측정이 아니라 선정 규칙의 산물이다** — 종속변수 위에서
표본을 고른 것.

그러므로 M1을 *"palimpsest를 썼으면 재발이 n% 줄었다"* 로 읽으면 안 된다. 읽을 수 있는
것은 *"재발이 일어난 그 순간에 규칙이 눈앞에 왔는가"* 뿐이다. **이 문장을 지표 정의에
붙여 둔 것이 오독을 막는 유일한 장치다.**

#### 2. 회고로 두 팔을 다 얻는 지표는 M1 하나뿐이다

M1은 과거 스냅샷에서 `touch`를 돌릴 수 있어 arm B가 회고로 성립한다. **M2·M3은 과거
작업을 palimpsest로 다시 돌릴 수 없으므로 arm B가 전향적 실행을 요구한다** — 사람이
실제로 도구를 쓰면서 일해야 값이 생긴다.

**이것을 등록에 적어 두지 않으면 나중에 "M2·M3도 쟀다"가 회고 계산 하나로 채워진다.**
회고 계산은 arm A이지 대조가 아니다.

### 되돌림 조건은 발동하지 않았다 — 다만 조건이 얇다

반증 조항은 *"G7 지표를 정할 수 없으면 채점 기준을 [G2](../plan/00-goals.md)로 되돌린다"* 였다.
**넷 중 셋을 등록했으므로 되돌리지 않는다.**

그러나 그 셋이 서 있는 조건은 얇다 — M1은 n=5, M2는 gitignored 산물 위에 서 있고,
M3은 신뢰도 상한이 붙는다. **조건이 무너지면 그때 되돌린다**는 것을 `falsified_if`에 적었다.

[R-29](../plan/00-risks.md#r-29)가 *"n이 작다"* 고 적은 것은 여전히 사실이다. **저울이
생겼다는 것과 저울이 정밀하다는 것은 다르고, 이 판정은 앞의 것만 말한다.**

### 남는 잔여

**`arms.toml`(T5)이 아직 없다.** 대조군 일반의 단일 진실은 그 파일이 될 예정이고, 지금은
결과 축이 쓰는 팔 둘(A0·A1)만 `[outcome.arm]`에 정의했다. `arms.toml`이 서면 정의는
그쪽으로 옮기고 이 절은 id로만 참조해야 한다 — **같은 것을 두 곳에 적으면 그것이
drift다**([README §7](../plan/README.md)). 옮기지 않으면 이 자리가 drift의 시작점이 된다.
