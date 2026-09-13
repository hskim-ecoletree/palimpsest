# 차이 — 봉인한 계획 대 실제 편집

> 회차 `2026-09-13-first-release-elsewhere` · `E1` ⑥ · 복제본 커밋 A `898a479` · B `d0eff0d`
> 차이마다 **좌표 두 끝**을 붙인다 — 원인(`effect/…:줄`)과 결과(복제본 `파일:줄`, 줄은 B 의 헝크 머리).
> ⚠ **방향 판정(걸음을 없앴나 · 덜 말했나 · 안 닿았나)은 이 문서가 안 한다** — `E3` 의 정반합이 한다. 여기는 차이와 원인만 싣는다.

## 1. 봉인 §5 의 걸음과 실제

| 봉인 §5 | 실제 편집 (결과 좌표) | 같나 | 원인 좌표 | 원인의 종류 |
|---|---|---|---|---|
| 1 `codex.ts` 삭제 | A — `src/core/hosts/codex.ts` 239 줄 삭제 | 같다 | 봉인 §1 | 과제 |
| 2 `hosts/index.ts` 의 import · 등록 · 재수출 제거 | B — `src/core/hosts/index.ts` `@@ -2` · `@@ -5` · `@@ -9 +7` | 같다 | 봉인 §4 ③ | **사전 앎** — 화면은 이 파일을 안 댔다(`03-touch/05-codexHostAdapter.txt:15` 가 「파일 최상위 참조는 세지 않는다」고만 말했다). `oracle/E4-breakage.txt` 가 이 파일이 P∖(C∪D) 이고 오류 자리가 최상위 import 줄뿐임을 쟀다 |
| 3 `setup.ts` 둘의 `codexHostAdapter` 참조 제거 | B — `src/cli/commands/setup.ts` `@@ -8` (import) · `@@ -251,29` (`discoverCodexAgents` **함수째 삭제**) · `@@ -288,5 +258,5` (`discoverProjectAgents` 의 codex·both 갈래를 **시끄러운 실패**로) · `src/core/setup.ts` `@@ -7` (import) · `@@ -291,10 +290,3` (`writeCodexSurfaceCatalog` 를 시끄러운 실패로) | **넓어졌다** — 참조 한 줄이 아니라 함수 하나와 갈래 둘 | `03-touch/05-codexHostAdapter.txt:12` → `04-callers/05-codexHostAdapter.txt` → `05-callers-B/discoverCodexAgents.txt` · `05-callers-B/writeCodexSurfaceCatalog.txt` · 그리고 ditto 의 `ADR-20260722-claude-code-only-host` 결정 2 | **`pal` 질의** — 호출자를 두 겹 물어 그 함수가 쓰이는 사슬(`discoverProjectAgents` ← `runWizard` · `installCodexSurface` ← `setup`)을 보고 갈래째 막았다 |
| 4 `tsc` 를 따라 고친다 | `tsc` 0 → 4 → 0 (`05-change.txt`) | 같다 | A 뒤 `tsc` 새 오류 4 | `tsc` |
| 5 문자열 레지스트리 94 줄은 안 건드린다 | 안 건드렸다 | 같다 | 봉인 §5 | 과제 |
| **없음** | ★ B — `src/core/hosts/shared.ts` `@@ -3` (`smol-toml` import) · `@@ -32,4` (`parseToml` 삭제) · `.ditto/knowledge/adr/ADR-0003-toml-parser.md` `@@ -3 +3` (상태 줄) | **새 걸음** | `04-touch-after-approve.txt:8-10` — 승인한 결정 본문 「`parseToml` wrapper 한 함수만 유지 · 사용 지점은 두 곳: `codex.ts` 의 `loadPermissions` 와 `mcpServersFromToml`」 | **화면 줄** — 걸린 결정이 그 결정의 사용 지점이 지우려는 파일 안에 있다고 말했다 |

## 2. 계획에 있었는데 안 한 걸음

| 걸음 | 까닭 좌표 |
|---|---|
| 13 개 이름마다 저장소를 훑어 파일 밖 쓰임을 확인 | 봉인 §5 에 명시하지 않았으나 `tsc` 전에 할 법한 걸음 — `04-readnote.md` §2 가 「안 한다」로 적었고 근거는 `04-callers/01·02·03·04·11·12·13`(파일 안뿐) |

## 3. 화면이 답하지 않은 자리

| 자리 | 화면 좌표 | 누가 답했나 |
|---|---|---|
| `hosts/index.ts` 의 등록 줄과 재수출 | `03-touch/05-codexHostAdapter.txt:15`(하한 문구) | 사전 앎 · `tsc` |
| 메서드 다섯의 호출자 | `03-touch/06~10-*-pick.txt` · `04-callers/06~10`(0) | `tsc` — A 뒤 `src/cli/commands/setup.ts(262,50) TS7006` 한 줄로만 드러났다 |
| 문자열 `'codex'` 레지스트리 조회 | — | 봉인 §4 ③(사전부검 반환문) |

## 4. 수

- 복제본 편집 파일: 계획 **4**(`codex.ts` 포함) → 실제 **6**(`shared.ts` · ADR-0003 이 늘었다)
- 차이 넷 중 원인이 `pal` 인 것 **둘**(화면 줄 1 · 질의 1) · 사전 앎 **하나** · 차이 아닌 `tsc` 몫 **하나**
