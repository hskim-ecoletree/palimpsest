# 봉인 — 효과 확인 ㈄ · ditto#70 의 한 조각

> 회차 `2026-09-13-first-release-elsewhere` · `E1` ⑴ · 2026-09-13
> **이 파일은 `pal touch` 를 부르기 전에 커밋된다.** 뒤의 산출(설치 · 인입 · `touch` · 읽은 줄 · 변경 · 차이)은
> 전부 이 커밋의 뒤에 서로 다른 커밋으로 들어간다.

## 1. 대상과 과제

- **대상**: ditto 새 복제본(세션 스크래치 `ditto-effect`) · HEAD `aded7ce7f88feb3c03238c5f9760f3a2ade4a6c1`.
  원본 `~/dev/projects/ditto` 는 만지지 않는다.
- **과제**: ditto#70 「Codex 표면 제거」의 한 조각 — **`src/core/hosts/codex.ts` 를 지우고, 그 때문에 깨지는
  곳을 고친다.** ditto 자신의 결정 `ADR-20260722-claude-code-only-host` 가 방향을 확정했고 #70 이 그 실행이다.
- **바이너리**: `pal 0.0.0+db6c366564e3`(이 회차의 ㈀㈁㈂㈃ 이 들어간 커밋에서 빌드).

## 2. 착수 관측 — 복제본이 깨끗하다

| 시각 | `.palimpsest/` | HEAD | `git status --porcelain` 줄 수 |
|---|---|---|---|
| 선언 목록을 뜨기 전 | 없음 | `aded7ce7…` | 0 |
| `pal symbols` 뒤(저장소 밖에서 불렀다) | 없음 | `aded7ce7…` | 0 |

## 3. 부를 심볼 — 규칙으로 정했다

**`pal symbols src/core/hosts/codex.ts` 가 내는 선언 전부(메서드 포함)** — 13 개. 골라 부르지 않는다.

| 줄 | 종류 | 이름 |
|--:|---|---|
| 25 | function | `mcpServersFromToml` |
| 58 | function | `scanCodexPluginRoot` |
| 85 | function | `codexPluginRoots` |
| 93 | function | `scanProjectCodexAgents` |
| 102 | variable | `codexHostAdapter` |
| 120 | method | `loadInstructions` |
| 132 | method | `loadPermissions` |
| 162 | method | `loadMcpServers` |
| 185 | method | `loadSurfaceInventory` |
| 202 | method | `spawnRun` |
| 215 | variable | `CODEX_PROFILE_SANDBOX_FLAGS` |
| 223 | variable | `CODEX_PROFILE_UNVERIFIED` |
| 231 | function | `buildCodexSpawnArgs` |

동명 후보 화면이 나오면(다른 파일에 같은 이름) 그 화면의 지목 문자열로 **`codex.ts` 의 것**을 다시 부른다.

## 4. ⚠ 과제 선택자의 사전 앎 — 숨기지 않는다

이 과제를 고른 자(메인)는 `touch` 를 부르기 전에 이미 아래를 알았다. 효과 판정(`E3`)의 입력이다.

1. **ADR-0003 「결정」 조각의 좌표 후보가 `codex.ts` 에 있다** — 착수 관측(`pal narrative --json`)에서
   후보가 `mcpServersFromToml` · `loadPermissions`(codex.ts · claude-code.ts)였다. 잠그기 전 실험으로 그 조각을
   `mcpServersFromToml` 에 한 번 승인해 봤다(다른 복제본에서).
2. **ADR-0016(dual-host · 폐기됨)의 후보가 `HostAdapter` · `resolveRepoRoot` 다.**
3. **`codexHostAdapter` 를 들이는 자리** — 사전부검 R2 의 반환문이 `src/core/hosts/index.ts:5`(등록 줄 ·
   파일 최상위) · `:9`(재수출) · `src/cli/commands/setup.ts:261` · `src/core/setup.ts:295` 를 적었다.
   레지스트리를 문자열 `'codex'` 로 찾는 자리가 `src/`·`tests/` 에 94 줄 있다는 것도 적었다.
4. ditto 의 `tsc --noEmit` 은 `aded7ce` 에서 오류 0 이다.

## 5. 도구 없이 세웠을 편집 계획 — 비교의 기준선

`touch` 를 안 봤다면 이렇게 했을 것이다(위 사전 앎만으로):

1. `src/core/hosts/codex.ts` 삭제.
2. `src/core/hosts/index.ts` 에서 import · `registerHostAdapter(codexHostAdapter)` · 재수출 제거.
3. `src/cli/commands/setup.ts` · `src/core/setup.ts` 의 `codexHostAdapter` 참조 제거.
4. `tsc --noEmit` 을 돌려 남은 오류를 따라 고친다.
5. 문자열 `'codex'` 레지스트리 조회 94 줄은 **이번 조각에서 안 건드린다**(시끄러운 실패로 이미 막혀 있다 — ADR-20260722 결정 2).

## 6. 걸음 — 순서를 못 박는다

| # | 걸음 | 산출 | 커밋 |
|--:|---|---|---|
| ① | 이 봉인 | `effect/00-seal.md` | 이 커밋 |
| ② | 복제본에 `pal install` → 설치 산출을 **복제본에 커밋** · `pal narrative` | `effect/01-install.txt` · `effect/02-narrative.txt` | 다음 |
| ③ | 13 개에 `pal touch` — 같은 스크립트가 복제본의 `git rev-parse HEAD` 와 `git status --porcelain -- src tests scripts`(비어야 한다)를 함께 적는다 | `effect/03-touch/*.txt` · `effect/03-touch/head.txt` | 그 다음 |
| ④ | 읽은 줄 기록 — 화면의 줄마다 무엇을 알았고 계획이 바뀌었나 | `effect/04-readnote.md` | 그 다음 |
| ⑤ | 복제본 변경 — **A: 파일 삭제만**(부모 = ③ 의 HEAD) → `tsc` 오류를 뜬다 · **B: 깨진 곳을 고친다** | `effect/05-change.txt`(두 SHA · diffstat · `tsc` 전후) | 그 다음 |
| ⑥ | 차이 문서 — 5 절의 계획 대 실제 편집, 화면 줄 → 변경 줄 | `effect/06-delta.md` | 그 다음 |

승인이 필요하면 ③ 의 화면이 안내한 명령을 **그대로** 부르고, 그 전후 `touch` 화면을 함께 남긴다.
