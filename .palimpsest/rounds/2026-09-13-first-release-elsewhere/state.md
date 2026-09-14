# 교대 상태 — 첫 릴리스를 남의 저장소에서 세운다

> 회차 `2026-09-13-first-release-elsewhere` · 착수 커밋 `6ee9eb3`

★ **새 컨텍스트가 받는 것은 「`intent.md` 전문 + 이 파일의 요약」이다.** 직전 산출물을
시드로 받지 마라.

## 지금 단계 — **종료 걸음. push 두 번이 Windows 에서 빨갰고, 고친 커밋을 PR #157 의 CI 로 먼저 잰다**

| 단계 | 상태 |
|---|---|
| 착수 ~ 승인 · 구현 · 효과 | **닫혔다** — 조건 29 |
| 판 `p1-binary-nul` · `e3-effect` · `p2-caller-sites` | **닫혔다** — 셋 다 상한에서 소유자 답 · 레코드 DL1 13 · DL2 19 · DL3 23 |
| 종료 기준선 | **돌았다** — `baseline/runner-post.md`(원본 쓰기 정정 줄 포함) · 이 회차가 깬 스크립트 둘을 `be2ccb5` 가 고쳤다 |
| **G2** | **반증** — `oracle/G2-violation.txt` · 소유자 결정으로 더해진 캐시만 지움 |
| 결박 | **닫혔다** — `66108c5` · `pal doctor` 위반 0 · 메모의 옛 수 7116 은 게이트 `### 결박` 이 정정한다 |
| 판정 | 통과 26 · 반증 1(G2) · 대조불가 1(E3) · 미측정 1(F2) — 정본은 게이트 `## 판정` |
| `F1` | **닫혔다** — `#135` 코멘트를 `7199/11099` 로 편집 |
| `A6` | **닫혔다** — `cargo xtask test` 1124 통과 · Rust 파일 간 엣지 1319 = 1319(HEAD `5b4a37f`) |
| `report.md` | **섰다** — `96bfcb7` 뒤 R2 정정 반영 |
| 독립 리뷰 | **R1 · R2 처분** — 상한 4. R1 정정 10 · 기각 19 / R2 는 원장 `IR2-*` |
| push | `701acb5` · `4398134` 둘 다 CI `windows-latest` 빨강(`oracle/F2-ci-701acb5.txt` · `oracle/F2-ci-4398134.txt`). 첫 수정 `1bcdc93` 은 HEAD 원장만 옮겨 불완전했다(`MS3-02`) · `3f69ccd` 가 원장 둘을 `.redb` 없이 계산 · 소유자 답 `## 승격` 9 · 10 · 11 |
| PR | [#157](https://github.com/hskim-ecoletree/palimpsest/pull/157) draft · 브랜치 `fix/radius-windows-lock` · head `51de0e8` · CI 런 `34802282312` |
| 남은 것 | PR CI 가 초록이면 main 에 push(세 번째) · 그 SHA 의 CI 로 `F2` · 전사는 스크래치 `close/f2-transcribe.py` 로 하고 커밋은 로컬. 빨강이면 main 은 그대로 두고 원인을 더 판다 |

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

- **레코드를 손으로 파싱했다** — 규약은 `extract.py` 를 요구한다(기계 칸이 갈려 `cargo xtask check` 가 빨개졌다). 원 반환문의 레코드는 **언제나** `python3 .claude/skills/round/bin/extract.py <출처> <라운드 번호(정수)> <raw>` 로 뽑고 판단 칸만 사람이 얹는다.
- **추출기의 첫 수정이 절대경로 좌표를 버렸다** — 끝난 회차 전부에 옛·새 판을 대 보고서야 드러났다. 추출기를 고치면 끝난 회차 전부에 대 본다.

- 임포트 지정자를 `grep -o` 한 줄 단위로 세면 여러 줄 `import {…} from` 이 빠진다(첫 셈 「other 681」). 파이썬 정규식(`re.S`)으로 다시 셌다.

- **E2 오라클이 저장소 상대 경로를 금지 패턴 시험에 넘겼다** — `cargo test` 는 크레이트 디렉터리에서 시험을 돌린다. 시험에 넘기는 파일 경로는 **절대 경로**다.
- **G1 을 착수 조건으로 안 재고 구현했다** — 시험 고정물이 선행 저장소의 별칭 꼴(`~/*`)과 이름을 실었다. 구현 뒤 곧바로 `G1` 명령을 돌린다.
- **정반합 역할 에이전트는 읽기 전용이다** — 파일을 쓰라고 해도 반환문으로만 낸다.
- **변경 전 측정을 원장 쓰기와 겹쳐 돌렸다** — `cargo xtask check` 가 반쯤 쓴 레코드를 읽어 「회차 레코드」가 빨갰다. 변경 전 · 뒤 산출은 원장을 안 만지는 동안 뜬다.
- **오라클이 조건 문면과 다른 자리를 읽었다(E2 ⑵ · E4)** — 둘 다 C 를 `touch` 가 아니라 질의 출력에서 뽑았다. 오라클을 쓸 때 조건의 명사(「그 출력들」 · 「touch 가 낸」)를 입력 경로와 한 줄씩 댄다.
- **zsh 에서 `$args` 로 인자를 묶어 넘겼다(두 번째)** — 나뉘지 않는다. 인자는 풀어 적는다.
- **검증 스크립트를 원본 코퍼스 경로에 대고 다시 돌렸다** — `f06-verify.py` 가 캐시 자리를 안 줘 원본 ditto 에 파일이 쓰였다(G2 반증). 남의 저장소를 읽는 스크립트는 **캐시 · 인덱스 · 의도 자리를 전부 임시로** 주는지 먼저 본다.
- **산출 파일의 이름을 옮기면서 그 이름으로 찾는 소비자를 안 훑었다** — `d1_scan` 이 `<파일>.json` 짝을 못 찾았다(`MS2-08`).
- **보고의 원인 문장을 기록 없이 적었다** — `G5` 런 0 의 원인을 「push 된 적이 없어서」로 적었는데 같은 회차의 착수 관측이 push 를 적고 있었다(`IR2-06` 금지역). 원인 문장은 그것을 대는 산출 줄이 있을 때만 쓴다.
- **「최종 바이너리」 산출을 코드 수정 전에 떴다** — 이름이 산출 시점을 앞질렀다(`IR2-08`). 최종이라 부르는 산출은 머리에 `pal --version` 을 싣는다.
- **계기판을 PATH 에 `pal` 없이 불렀다** — `dashboard.py` 는 `pal round conditions` 를 부른다. `PATH="$PWD/target/release:$PATH"` 로 부른다.
- **macOS 에서 못 보는 Windows 결함의 수정을 CI 로 재기 전에 main 에 올렸다** — 첫 수정이 원장 계산 하나만 옮겨 두 번째 push 도 빨갰다(`MS3-02`). 로컬에서 원리상 못 재는 수정은 브랜치 PR 의 CI 로 먼저 잰다.
