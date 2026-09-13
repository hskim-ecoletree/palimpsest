# 교대 상태 — 첫 릴리스를 남의 저장소에서 세운다

> 회차 `2026-09-13-first-release-elsewhere` · 착수 커밋 `6ee9eb3`

★ **새 컨텍스트가 받는 것은 「`intent.md` 전문 + 이 파일의 요약」이다.** 직전 산출물을
시드로 받지 마라.

## 지금 단계 — **판 셋 중 둘이 닫혔고(승격 둘), 판 p2 가 돈다. 그 뒤 종료 걸음**

| 단계 | 상태 |
|---|---|
| 착수 · 인터뷰 · 기준선 · 사전부검 · 조건 설계 평가 · 승인 | **닫혔다** — 조건 29 |
| ㈀~㈃ 구현 · ㈄ 효과 ①~⑥ | **커밋했다** |
| **판 `p1-binary-nul`** | **닫혔다** — 상한 뒤 실패 → 소유자 `[승격]`(`8852754`): NUL 규칙을 고친다 · f04 줄 판정 · partial 합격. 취합 `dialectic/r1-raw.md` · 레코드 DL1 13 · **실행 커밋 `ba0a983`** · 측정 `oracle/p1-X/`(`78ceb74`) — A5 ⑷ 59 → 0 · 골든 ditto 새 78 |
| **판 `e3-effect`** | **닫혔다** — 상한 뒤 금지역 1 · 실패 2 → 소유자 `[승격]`(`6145849`): **E3 대조 불가** · 봉인 §4 결함을 게이트 · 보고에 적는다 · grep 반사실 안 뺌. **취합 보고 진행 중** → `dialectic/r2-raw.md` · 레코드 뽑기 |
| **판 `p2-caller-sites`** | **정(正) 진행 중** — 설계 `dialectic/p2-design.md`(미커밋). 물음: `touch` 가 호출자 자리를 싣게 넓혀 `E2` ⑵ · `E4`(문면대로 반증 — `oracle/E2-literal-touch.txt` · `oracle/E4-literal-touch-c.txt`)를 닫나 |
| 종료 걸음 | `cargo xtask test` 진행 중(p1 뒤) · `G1` 다시 잼 0/1(`oracle/G1-coupling.txt`, 미커밋) · `F1` `#135` 코멘트 · `G2` 뒤 박제 · 조건 상자 전사 · 게이트 `docs/gates/first-release-elsewhere.md`(초안 미추적) · 독립 리뷰(상한 4) · 종료 기준선 · `report.md`(F4 `G5`·`Actions` 줄 · 봉인 결함 줄) · 결박 · push 한 번 · F2 |

★ **판 산출물은 읽기 전용 서브에이전트의 반환문을 스크립트로 옮긴다.** 긴 취합은 에이전트가 스크래치에 쓴 파일을 md5 확인 뒤 파일째 옮긴다.
★ **시험이 도는 동안 커밋하지 않는다** — 버전 시험이 바이너리의 커밋과 HEAD 를 댄다(`xtask-test-mid` 가 그렇게 빨갛었다).

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
- **변경 전 측정을 원장 쓰기와 겹쳐 돌렸다** — `cargo xtask check` 가 반쯤 쓴 레코드를 읽어 「회차 레코드」가 빨갰다. 변경 전 · 뒤 산출은 원장을 안 만지는 동안 뜬다.
- **오라클이 조건 문면과 다른 자리를 읽었다(E2 ⑵ · E4)** — 둘 다 C 를 `touch` 가 아니라 질의 출력에서 뽑았다. 오라클을 쓸 때 조건의 명사(「그 출력들」 · 「touch 가 낸」)를 입력 경로와 한 줄씩 댄다.
- **zsh 에서 `$args` 로 인자를 묶어 넘겼다(두 번째)** — 나뉘지 않는다. 인자는 풀어 적는다.
