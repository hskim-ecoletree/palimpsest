# 착수 전 기준선 — 회차 2026-09-13-first-release-elsewhere

- 착수 커밋 `6ee9eb3` (워킹트리: `NEXT-E-*.md` 둘 삭제 스테이징 · 회차 디렉터리 추가)
- 잰 시각 2026-09-13 19:35:43 ~ 20:45:56 KST (직렬 · 약 70분)
- 러너: 스크래치패드 `runner/run.sh`. 저장소에 두지 않았다. 전체 출력은 스크래치패드 `runner/out/<항목>.log` 에만 있다.
- 판정하지 않는다. 적은 수는 스크립트가 산출한 셈이다. exit 0 은 판정이 아니다.

## 환경

| 항목 | 값 |
|---|---|
| rustc / cargo | 1.94.1 (e408947bf 2026-03-25) (Homebrew) · rustup 없음 |
| python3 | 3.13.9 (anaconda) · 저장소 `.venv` 없음 |
| 빌드 | release ok (9s) · debug ok (27s) |
| portal-backend | HEAD `a29cad0bf6a8` · `.kt` 671 · 추적 밖 `.palimpsest/` 1 |
| portal-backend-aa-task | HEAD `10185f804ad8` · `.kt` 434 · worktree(`.git` 이 파일) |
| boxwood-packages | HEAD `2e9198716796` · `.kt` 17 · 더러움 6 (`openapi-client/` Java·pom, `.kt` 아님) |
| ditto | HEAD `aded7ce7f88f` · 추적 밖 278 |
| 핀 넷 | manifest 와 전부 일치 · S0 코퍼스 `~/dev/projects/boxwood` → 1122 성립 |

## ⚠ 오염 고지 — 도는 동안 HEAD 가 움직였다

다른 세션이 실행 중 여섯 번 커밋했다: `6ee9eb3` → `e6ad3f2`(19:58) · `fab0306`(19:59) · `3041416`·`d4a7361`(20:18) · `88565f4`·`8aa606d`(20:31).
바뀐 경로는 `.palimpsest/rounds/2026-09-13-first-release-elsewhere/` 와 루트 `NEXT-E-*.md` 둘의 삭제뿐이다.
**`crates/`·`scripts/`·`corpus/`·`schema/`·`surface/` 는 안 움직였다.**

- 제품을 재는 값은 오염되지 않았다.
- `cargo xtask check` 를 안에서 읽는 자리(`f22-1` 성한 상태 · `f03-3` ④ · `f06` ①)는 **그 시점의 회차 문서 상태**를 잰다. 오염됐다.
- 항목별 HEAD: s0-compare~f22-3 `6ee9eb3` · f22-4~f02-4 `fab0306` · f03-1~f05 `d4a7361` · f06~f11 `8aa606d`.

## 준비 (스물넷에 안 셈)

| 항목 | exit | 시간 | 값 |
|---|---|---|---|
| build-release | 0 | 9s | ok |
| build-debug | 0 | 27s | ok |
| `cargo xtask check` | 1 | 71s | FAIL 2 — 「회차 레코드」(`NEXT-E-prompt.md` 가 없다: 2026-08-23-check-verifies-work 원장 3행 · 2026-09-12 `IR3-31`) · 「완수 조건 설계 평가」(이 회차 `conditions-audit/r<n>-raw.md` 없음, 착수 시점) |
| `cargo xtask test` | 0 | 319s | 통과 1065 · 실패 0 · 무시 2 (cargo test 두 번 호출 · `test result` 54줄 합) — 옛 「417」과 형태가 달라 비교 안 함 |
| `cargo clippy --workspace --all-targets` | 0 | 235s | 경고 약 125 (중복 뺀 합) — 옛 「8」과 호출 형태가 달라 비교 안 함 |
| s0-corpus | 0 | 1s | 671 + 434 + 17 = 1122 (기대 1122) |

## 표 — 항목 24

| # | 항목 | exit | 시간 | 어긋남 | 대조 불가 | 값 |
|---|---|---|---|---|---|---|
| 1 | s0-compare | 0 | 33s | 0 | 0 | 파일 1122/1122 · 선언 2315 · 불일치 0 |
| 2 | f22-1 | 2 | 505s | 2 | 0 | 성한 상태 실패 · ⑦ 「거주 불가 타입을 `built` 로」 변형이 아무것도 안 바꿨다 · 나머지 음성 7/7 |
| 3 | f22-2 | 0 | 76s | 0 | 0 | 성한 12 · 음성 7/7 |
| 4 | f22-3 | 0 | 49s | 0 | 0 | 음성 5/5 · Kotlin 발현 4/5 · 도입 4/5 |
| 5 | f22-4 | 0 | 286s | 0 | 0 | 격리 100회 부분갱신 0 · 음성 16/16 |
| 6 | s1 | 0 | 3s | 0 | 0 | 경로 997 · parsed 671 · 캐시 적중 997 · 넷 통과 |
| 7 | s2 | 1 | 121s | 1 | 0 | 성한 응답 묶음에서 `② unresolved 이 not_built 가 아니다: {'present': []}` · 나머지 음성 8/8 |
| 8 | s3 | 0 | 1s | 0 | 0 | 다섯 통과 |
| 9 | f01 | 1 | 90s | 1 | 0 | ⑦ 골든 대장 다름 — detector 추출기 `f03-2` → `f09-import-sites` · ⑧ 10배에서 파일당 1.02배 |
| 10 | f02-1 | 0 | 33s | 0 | 0 | 음성 6/6 · S0 불일치 0 · 다섯 통과 |
| 11 | f02-2 | 0 | 52s | 0 | 0 | 음성 5/5 · 다섯 통과 |
| 12 | f02-3 | 0 | 14s | 0 | 0 | 불변식 5/5 · 해소 파일안 76048 / 파일밖 9862 |
| 13 | f02-4 | 0 | 202s | 0 | 0 | 음성 4/4 · 10배에서 파일당 0.94배 · 상주 1.31배 |
| 14 | f03-1 | 0 | 720s | 0 | 0 | 여섯 통과 · 결정성 ditto 4578 · pb 1340 |
| 15 | f03-2 | 0 | 33s | 0 | 0 | 불변율 100.00% ×7 · 가시율 100.00% |
| 16 | f03-3 | 1 | 309s | 1 | 0 | ④ `cargo xtask check` 실패 (로그가 잘려 어느 검사인지 안 남음) · ①②③ ok |
| 17 | f04 | 1 | 116s | 1 | 0 | ⑨ ditto 고정비 뺀 8.8배 (선 10 · 옛 선 5.7배) · pb 120.5배 · ②⑥ ok |
| 18 | f05 | 1 | 14s | 1 | 1 | ⑨ ditto 8.5배 (선 10) · pb 86.0배 / 대조 불가 ③ pb 엣지 0 |
| 19 | f06 | 1 | 200s | 1 | 2 | ① 변형 전 xtask check 가 이미 실패 (이 회차 `premortem/r1·r2-raw.md`·`conditions-audit/r1-raw.md` 기계 칸이 추출기 산출과 갈림) / 불가 ② pal-mcp 폐기 · ③ Cypher 파서 없음 |
| 20 | f09 | 1 | 156s | 1 | 0 | ⑥ 비율 하한 — 사유가 `projection_stale` 하나뿐 (하한 2) · 나머지 ok |
| 21 | f10 | 1 | 257s | 3 | 3 | ② 강 신호 미결박 ditto 64.4% · pb 87.3% · pb 3분류 `bound` 없음 / 불가 ① 층화 · ⑨ ditto · ⑨ pb |
| 22 | f10-5 | 0 | 113s | 0 | 1 | 불가 ⑧ 도달 불가가 된 사유 (모집단 0) |
| 23 | f10-6 | 0 | 137s | 0 | 4 | 불가 ② pb 표식주석 · ⑥ Kotlin 실코퍼스 · ⑨ ditto-6 · ⑨ pb-6 |
| 24 | f11 | 0 | 30s | 0 | 3 | 불가 ① 재발 재현(떴다 2 · 안 떴다 1 · 불가 2) · ④ 상한과 생략 · ⑥ MCP(폐기) · ⑦ p95 9.9ms |
| | **합** | | | **12** | **14** | |

- 어긋남 12 = f22-1 2 · s2 1 · f01 1 · f03-3 1 · f04 1 · f05 1 · f06 1 · f09 1 · f10 3
- 대조 불가 14 = f05 1 · f06 2 · f10 3 · f10-5 1 · f10-6 4 · f11 3
- exit: 0 이 14, 1 이 9, 2 가 1 (f22-1)

## 선 아래인 것 — 까닭 (분류만 적는다. 판정은 게이트가 한다)

**기존 빚**
- f04 ⑨ · f05 ⑨ — ditto 비율 선 10 아래. [#61] · Homebrew 1.94.1 환경.
- f09 ⑥ — 사유 한 갈래뿐. [#58] · `docs/gates/F09.md:488`.
- f10 ② 셋 — F10 §5 반증. `docs/gates/F10-6-attachment.md:317` 에 64.4% 로 적혀 있다.
- f01 ⑦ — 골든(`343dd5e`, 2026-08-13) 을 축복한 뒤 추출기 표가 두 번 바뀌었다(`f02-rust-scope` → `c9a6bae` 2026-09-10 `f09-import-sites`). 2026-09-08 발견 `BL1-03` 이 「범위밖」으로 처분했다.

**새것 — 2026-09-08 착수 기준선에는 없었다. 열린 이슈에서 못 찾았다**
- s2 ② 와 f22-1 ⑦ 은 **뿌리가 같다.** `37e3689`(2026-09-10 · `B1~B4 — 못 푼 참조가 값이 된다. F08 을 뒤집었다`) 가 `UnresolvedRef` 를 값으로 세웠다. 그런데 두 스크립트가 옛 「안 만듦」을 여전히 기대한다.
  - `scripts/s2-verify.py:55` `NOT_BUILT_SLOTS = ["unresolved", "effects", "judgments"]` — 산출은 `{'present': []}`.
  - `scripts/f22-1-verify.sh` ⑦ 은 `status     = "not_built"\nbuilt_by   = "F08"\n` 을 지운다. `schema/graph.toml` 에 `built_by   = "F08"` 가 0회라 **변형이 무효**다. 그 음성 대조는 지금 안 재어진다.
  - 2026-09-08 착수 기준선에서는 s2 초록 · f22-1 음성 8/8 이었다.

**회차 그림자 — 이 회차 자신의 상태를 잰다**
- xtask check 「회차 레코드」 FAIL — 이 회차가 `NEXT-E-prompt.md` 를 지워서, 옛 원장 4행의 `경로` 가 가리키는 파일이 사라졌다. 제품 회귀가 아니고, 이 회차가 만든 것이다.
- xtask check 「완수 조건 설계 평가」 FAIL — 착수 시점 `conditions-audit/` 가 비어 있었다. 20:15 무렵 f03-3 안의 check 에서는 이미 ok 였다.
- f22-1 성한 상태 · f03-3 ④ · f06 ① — 위 xtask check 실패를 안에서 읽는다. f06 시점(`8aa606d`)의 실패 사유는 이 회차 raw 셋의 기계 칸이 추출기 산출과 갈린 것이다.

## 정리

- `git checkout -- corpus/tasks/` 로 f10·f10-5·f10-6·f11 이 덮어쓴 넷(`f10-5-binding-sample` · `f10-6-binding-sample` · `f10-f09-remeasure-ditto` · `f10-false-binding-sample`)을 되돌렸다. 추적 안 되는 `f10-f09-remeasure-ditto-6.tsv` 는 지웠다.
- 끝난 뒤 `git status`: 다른 세션의 회차 디렉터리 변경(`state.md` M · `conditions-audit/brief-r2.md` · `oracle/`)만 남았다. 소스 변경 0 · `corpus/` 변경 0.
- 스테이징돼 있던 `NEXT-E-*.md` 삭제는 실행 중 다른 세션의 커밋이 가져갔다. 이 러너는 커밋하지 않았다.

## 비교 기준

직전 회차 `2026-09-12-binding-radius-in-use` 에는 종료 시 기준선이 없다. 그래서 차이는 계산하지 않았다.
위 「새것」의 비교 대상은 가장 최근의 스물넷 기준선인 `2026-09-08-cross-file-references/observations/baseline-start.txt` 이고, 분류 참고로만 썼다.

[#58]: https://github.com/hskim-ecoletree/palimpsest/issues/58
[#61]: https://github.com/hskim-ecoletree/palimpsest/issues/61
