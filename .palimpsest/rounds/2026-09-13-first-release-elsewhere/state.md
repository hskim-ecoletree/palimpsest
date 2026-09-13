# 교대 상태 — 첫 릴리스를 남의 저장소에서 세운다

> 회차 `2026-09-13-first-release-elsewhere` · 착수 커밋 `6ee9eb3`

★ **새 컨텍스트가 받는 것은 「`intent.md` 전문 + 이 파일의 요약」이다.** 직전 산출물을
시드로 받지 마라.

## 지금 단계 — **인터뷰가 닫혔다. 사전부검 R1 이 다음이다**

| 단계 | 상태 |
|---|---|
| 착수 첫 걸음 | **닫혔다** — `G5` 는 런 0 (이벤트는 있음 · Actions enabled) · `NEXT-E` 둘 지움(그 회차는 철회) · `pal` 빌드 · 관측 재현(`baseline/`) |
| 인터뷰 | **닫혔다** — 상한 2 소진 · 범주 다섯 열림 · `interview/r1.md`·`r2.md` |
| 기준선 스물넷 | **닫혔다** — `baseline/runner-pre.md`(`fd02e5b`) · 어긋남 12 · 대조 불가 14 · 이 회차가 만든 실패 하나(`NEXT-E-prompt.md` 삭제가 옛 레코드 넷을 끊음) |
| 장부 수정 | **닫혔다** — 레코드를 추출기 산출로 다시 세웠고(손 파싱이었다) · `xtask` 가 좌표를 `기준커밋` 트리에서도 잰다 · 추출기가 반환문 커밋의 트리에 대 좌표를 고른다 · `eea73db` · `4abfb96` · `cargo xtask check` 초록 |
| 사전부검 R1 | **닫혔다** — 11 항 · 처리 방침 `e6ad3f2` · 레코드 `fab0306` |
| 사전부검 R2 | **닫혔다** — 13 항 + 기각 8 · 처리 방침 `3041416` · 레코드 `d4a7361`(R1 기각 7 행을 함께 채움 · 합계 39) · **상한 2 소진** |
| 조건 설계 평가 R1 | **닫혔다** — 발견 15 · 기각 8 · 처리 방침 `88565f4` · 레코드 `8aa606d`(합계 62) |
| 조건 설계 평가 R2 | **처리 방침을 실었다** — 발견 17 · 기각 10 · 실패 2 · 금지역 1 전부 정정 · **상한 2 소진** · 원 반환문은 감사자가 직접 쓴다(처음엔 본문으로만 돌려줬다) |
| 완수 조건 | **29 개**(A10 · B5 · C2 · D3 · E4 · F3 · G2) · 형식오류 0 |
| 원본 박제 | `oracle/G2-ditto-origin-before.txt` — `G2` 의 「전」 |
| 승인 | **받았다** ⟨2026-09-13⟩ — *「전부 승인」* · `approval.md` |
| ㈀ TS 모듈 해소 | **커밋했다** — `f953a0c` · 규칙 준수 수정 `b8eeb1d` · ditto 복제본 `0/11010` → `7116/11010` · 시험 `ts_cross_file` · RED `oracle/A1-red.txt` |
| ㈃ 사용자 화면 어휘 | **커밋했다** — `9db40b7` · 시험 `user_vocabulary`(패턴은 `regex` 로 등록 그대로) · RED `oracle/D1-red.txt`(패턴별 64) |
| ㈁ 승인 대기 · ㈂ 동명 고르기 | **쓰는 중** — `pal-cli/src/pending.rs`(`.palimpsest/narrative-pending.json` · 파생물) · `touch`/`query --pick` · 시험 `pending_and_pick` |
| ㈄ 효과 · 오라클 `A3`·`A5`·`B4`·`E1`·`E2`·`E4` · `A6` · `F`·`G` | 그 뒤 |

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
