# 교대 상태 — 첫 릴리스를 남의 저장소에서 세운다

> 회차 `2026-09-13-first-release-elsewhere` · 착수 커밋 `6ee9eb3`

★ **새 컨텍스트가 받는 것은 「`intent.md` 전문 + 이 파일의 요약」이다.** 직전 산출물을
시드로 받지 마라.

## 지금 단계 — **인터뷰가 닫혔다. 사전부검 R1 이 다음이다**

| 단계 | 상태 |
|---|---|
| 착수 첫 걸음 | **닫혔다** — `G5` 는 런 0 (이벤트는 있음 · Actions enabled) · `NEXT-E` 둘 지움(그 회차는 철회) · `pal` 빌드 · 관측 재현(`baseline/`) |
| 인터뷰 | **닫혔다** — 상한 2 소진 · 범주 다섯 열림 · `interview/r1.md`·`r2.md` |
| 기준선 스물넷 | 러너가 돈다 → `baseline/runner-pre.md` |
| 사전부검 R1 | **닫혔다** — 11 항 · 처리 방침 `e6ad3f2` · 레코드 `fab0306` |
| 사전부검 R2 | **닫혔다** — 13 항 + 기각 8 · 처리 방침 `3041416` · 레코드 `d4a7361`(R1 기각 7 행을 함께 채움 · 합계 39) · **상한 2 소진** |
| 완수 조건 | **섰다** — 26 개 · 형식오류 0 |
| 조건 설계 평가 R1 | 돈다 — 브리프 `conditions-audit/brief-r1.md` → `conditions-audit/r1-raw.md` |
| 승인 | 그 뒤 |

## 실행 순서 — 사전부검 R2 가 못 박았다

**한 손이 ㈀ → ㈃ → ㈁ → ㈂ 순서로** 하고 ㈄ 는 넷이 선 뒤다. 병렬 하위 작업으로 나누지 않는다(`touch.rs` 의 이관표·후보 화면과 시험 단언을 여럿이 고친다).

## 오라클 전제 — 실측했다

- **TypeScript 컴파일러 대조**: `node` 가 `~/dev/projects/ditto/node_modules/typescript`(5.9.3)를 불러 `ts.resolveModuleName` 이
  `./shared` → `src/core/hosts/shared.ts` · `~/core/fs` → `src/core/fs.ts` · `node:fs` → 못 풂 을 냈다.
- **ditto 타입 검사**: 새 사본에 원본 `node_modules` 를 링크로 걸고 `node_modules/.bin/tsc --noEmit` → 오류 0 · rc 0 · 3.8 초(`aded7ce`).

## 작업 규율

- **ditto 원본(`~/dev/projects/ditto`)을 만지지 않는다.** 복제본은 세션 스크래치의 `ditto/` 다 —
  세션이 바뀌면 `git clone ~/dev/projects/ditto <스크래치>/ditto` 로 다시 뜨고 HEAD `aded7ce` 를 확인한다.
- 복제본에 `pal install` 과 ADR-0003 조각 승인 하나를 이미 했다(기준선 **뒤**). 효과 확인 ㈄ 는 새 복제본에서 한다.
- **ditto 에 대한 결박을 이 저장소의 `.palimpsest/intent/` 에 쓰지 않는다.**

## 실패한 접근

- 임포트 지정자를 `grep -o` 한 줄 단위로 세면 여러 줄 `import {…} from` 이 빠진다(첫 셈 「other 681」). 파이썬 정규식(`re.S`)으로 다시 셌다.
