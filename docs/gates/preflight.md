# preflight 게이트 — 판정 기록

[P0-preflight](../plan/features/P0-preflight.md)의 작업별 판정을 여기에 남긴다.
각 항목은 **통과 · 반증 · 대조 불가** 셋 중 하나를 명시한다. 생략하고 다음으로 가는 것이
이 계획의 가장 조용한 실패 경로다([계획 §7](../plan/README.md)).

**측정값은 실측만 적는다.** 재지 못한 것은 대표값으로 채우지 않고 "못 쟀다"고 적는다.

| 작업 | 판정 | 기록 |
|---|---|---|
| T1 boxwood 작업본 복원 | **통과** | [§T1](#t1--boxwood-작업본-복원) |

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
