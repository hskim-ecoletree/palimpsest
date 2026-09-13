# 교대 상태 — 첫 릴리스를 남의 저장소에서 세운다

> 회차 `2026-09-13-first-release-elsewhere` · 착수 커밋 `6ee9eb3`

★ **새 컨텍스트가 받는 것은 「`intent.md` 전문 + 이 파일의 요약」이다.** 직전 산출물을
시드로 받지 마라.

## 지금 단계 — **구현·효과 실행이 섰다. 판정 둘(정반합)과 종료 걸음이 남았다**

| 단계 | 상태 |
|---|---|
| 착수 · 인터뷰 · 기준선 · 사전부검 R1·R2 · 조건 설계 평가 R1·R2 · 승인 | **닫혔다** — 조건 **29** · `approval.md` |
| ㈀ TS 모듈 해소 · ㈃ 화면 어휘 · ㈁ 승인 대기 · ㈂ 동명 고르기 | **커밋했다** — 시험 `ts_cross_file` · `user_vocabulary` · `pending_and_pick` |
| ㈄ 효과 ①~⑥ | **커밋했다** — `effect/00-seal.md` … `effect/06-delta.md`(`b2e2205`) |
| 오라클 산출 | `A1-red` · `A3-fixture` · `A5-ditto`(+ `A5-2-shape` — ⑵ 를 조건 문면의 꼴로도 쟀다) · `A6-rust` · `B2-red` · `B4-timing` · `D1-red` · `D2-commit` · `E1-order` · `E2-scene` · `E4-breakage` · `F4-g5` · `G1-first-red` → `G1-coupling` |
| **판 `p1-binary-nul`** | 설계 `dialectic/p1-design.md` · M2 측정 `oracle/p1-M2/` · **정(正) 진행 중** → 반 → 합 → 종료 판단 → 취합(`dialectic/r1-raw.md`) · 물음: `A5` ⑷ 의 빠진 59(NUL 바이트 분류)를 확대로 닫나 반증+이슈인가 |
| **판 `e3-effect`** | 설계 `dialectic/e3-design.md` · **정(正) 진행 중** · 상한 2 를 다 쓴다 · 취합은 다음 번호(`r2-raw.md`) · ⚠ `E3` 원문에 합격선이 없어 `K0` 이 결박 못 하면 r1 뒤 승격 |
| 남은 결정론 걸음 | `F1`(`#135` 코멘트) · `F2`(마지막 SHA CI) · `G2` 뒤 박제 · `A6` ⑴ 최종 HEAD 의 `cargo xtask test` · `intent.md` 상자 전사 · 게이트 `docs/gates/first-release-elsewhere.md` · 독립 리뷰(상한 4) · 종료 기준선 · `report.md`(F4 가 `total_count 0` 이라 `G5`·`Actions` 줄 필수) · 결박 · push 한 번 |

★ **판 산출물은 전부 읽기 전용 서브에이전트가 반환문으로 내고 메인이 스크립트로 옮긴다** — 하위 에이전트 전사(`subagents/agent-*.jsonl`)의 마지막 반환문에서 ```markdown 블록을 뽑는다. 머리에 「옮겨 적은 자」를 단다.

## 착수 바이너리

`A1`·`B2`·`D1` 의 RED 와 `A6` ⑵ 가 쓴다 — 스크래치 워크트리 `wt-start`(`6ee9eb3`)에서 빌드한 `target/release/pal`(`pal 0.0.0+6ee9eb30f2dc`).
세션이 바뀌면 `git worktree add --detach <스크래치>/wt-start 6ee9eb3` 뒤 `cargo build --release -p pal-cli` 로 다시 뜬다.

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

- **화면이 안내한 명령을 zsh 변수로 쪼개 돌렸다** — 이 셸(zsh)은 따옴표 없는 `$변수` 를 낱말로 안 쪼갠다. 명령 한 줄이 인자 하나가 돼 `pal` 이 인자 없이 불렸다(`effect/04-approve-harness-error.txt`). 안내된 명령은 **`shlex.split` 으로 갈라** 돌린다.
- **ditto 의 `.gitignore` 는 `node_modules` 링크를 무시하지 않는다** — 효과 복제본에서 `git add -A` 를 쓰지 않는다. 경로를 지정해 커밋한다.

- **스냅샷의 화면 표기로 「같은 스냅샷인가」를 가렸다** — `Snapshot` 의 `Display` 는 `repo@abc1234+worktree` 로 **워킹트리 요약을 버린다.** 추적 파일을 고쳐도 같은 문자열이라 승인 대기 목록이 낡았는데도 실렸다(시험 `b3` 가 잡았다). 열쇠는 직렬화 값(`tree_digest` 포함)이다 — `pending::열쇠`.

- **레코드를 손으로 파싱했다** — 규약은 `extract.py` 를 요구한다(기계 칸이 갈려 `cargo xtask check` 가 빨개졌다). 원 반환문의 레코드는 **언제나** `python3 .claude/skills/round/bin/extract.py <출처> <라운드> <raw> <기존 findings.jsonl>` 로 뽑고 판단 칸만 사람이 얹는다.
- **추출기의 첫 수정이 절대경로 좌표를 버렸다** — 끝난 회차 전부에 옛·새 판을 대 보고서야 드러났다. 추출기를 고치면 끝난 회차 전부에 대 본다.

- 임포트 지정자를 `grep -o` 한 줄 단위로 세면 여러 줄 `import {…} from` 이 빠진다(첫 셈 「other 681」). 파이썬 정규식(`re.S`)으로 다시 셌다.

- **E2 오라클이 저장소 상대 경로를 금지 패턴 시험에 넘겼다** — `cargo test` 는 크레이트 디렉터리에서 시험을 돌린다. 시험에 넘기는 파일 경로는 **절대 경로**다.
- **G1 을 착수 조건으로 안 재고 구현했다** — 시험 고정물이 선행 저장소의 별칭 꼴(`~/*`)과 이름을 실었다. 구현 뒤 곧바로 `G1` 명령을 돌린다.
- **정반합 역할 에이전트는 읽기 전용이다** — 파일을 쓰라고 해도 반환문으로만 낸다.
