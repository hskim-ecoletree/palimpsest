# 종료 쪽 기준선 — 회차 2026-09-13-first-release-elsewhere

- 착수 커밋 `fee673b` · 끝날 때 `da01784`
- `pal --version` = `pal 0.0.0+fee673be3202` (release 빌드 직후)
- 잰 시각 2026-09-14 07:21:25 ~ 08:38:05 KST (직렬 · 약 77분)
- 러너: 스크래치패드 `baseline-post/run.sh`. 착수 쪽 `runner/run.sh` 를 복사했고, 스크립트 목록·인자·코퍼스 경로는 같다. 더한 것은 출력 경로, `pal --version`, ditto `git status` 전후 스냅샷 셋뿐이다. 저장소에 두지 않았다. 전체 출력은 `baseline-post/out/<항목>.log` 에만 있다.
- 판정하지 않는다. 적은 수는 스크립트가 산출한 셈이다. exit 0 은 판정이 아니다.

## 환경

| 항목 | 값 |
|---|---|
| rustc / cargo | 1.94.1 (e408947bf 2026-03-25) (Homebrew) · rustup 없음 (착수와 같다) |
| 빌드 | release ok (8s) · debug ok (32s) |
| portal-backend | HEAD `a29cad0bf6a8` · 더러움 1 (착수와 같다) |
| portal-backend-aa-task | HEAD `10185f804ad8` · worktree(`.git` 이 파일) |
| boxwood-packages | HEAD `2e9198716796` · 더러움 6 |
| ditto | HEAD `aded7ce7f88f` · 추적 밖 278 · **도는 전후 `git status --porcelain` 가 같다** |
| 핀 넷 | 착수와 같다 · S0 코퍼스 `~/dev/projects/boxwood` → 1122 성립 |
| ditto 원본에 쓰는 스크립트 | 없음. 스물넷은 `git show`·`archive`·`ls-tree`·`log` 를 쓰거나 임시 디렉터리에 `clone --local` 을 만든다. 그래서 건너뛴 항목이 없다 |

> ⚠ **정정(2026-09-14 · 독립 리뷰 R1)** — 위 칸 「ditto 원본에 쓰는 스크립트 없음」은 **거짓이었다.** `scripts/f06-verify.py` ③ 이 원본에 `pal query graph.dump` · `pal export` 를 `--cache-dir` 없이 붙여 원본의 `.palimpsest/cache` 에 캐시 항목 2451 개를 더했다(08:26). `git status --porcelain` 는 그 디렉터리를 안 보므로 「전후가 같다」가 그것을 못 잡았다. 경위 · 소유자 결정 · 지운 목록은 `oracle/G2-violation.txt` 가 지고, 스크립트는 `be2ccb5` 가 캐시를 격리했다. 이 칸은 러너의 박제라 고치지 않고 이 줄을 더한다.

## ⚠ 오염 고지 — 도는 동안 HEAD 가 움직였다

다른 세션이 실행 중 다섯 번 커밋했다: `254505e`·`403d589`(07:45) · `eb9c936`·`4f4a176`(07:52) · `da01784`(08:02).
바뀐 경로는 `.palimpsest/rounds/2026-09-13-first-release-elsewhere/` 뿐이다(17파일).
**`crates/`·`scripts/`·`corpus/`·`schema/`·`surface/`·`xtask/` 는 안 움직였다.**

- 항목별 HEAD: build~f22-3 `fee673b` · f22-4~f02-4 `4f4a176` · f03-1~f11 `da01784`.
- 제품을 재는 값은 오염되지 않았다.
- `cargo xtask check` 를 안에서 읽는 자리(준비 칸 check · `f22-1` 성한 상태 · `f03-3` ④ · `f06` ①)는 **그 시점의 회차 문서 상태와 추적 밖 `docs/gates/first-release-elsewhere.md`** 를 잰다.

## 준비 (스물넷에 안 셈)

| 항목 | exit | 시간 | 값 | 착수 → 종료 |
|---|---|---|---|---|
| build-release | 0 | 8s | ok | 같다 |
| build-debug | 0 | 32s | ok | 같다 |
| `cargo xtask check` | 1 | 75s | FAIL 2 — 「죽은 링크 부재」(추적 밖 `docs/gates/first-release-elsewhere.md` → `.palimpsest/rounds/2026-09-13-first-release-elsewhere/report.md` 없음) · 「원장 둘 대조」(이 회차 A1~G1 스물셋이 게이트 「통과」/E3 「대조불가」인데 `intent.md` 는 「미측정」이고 상자가 비었다) | rc 같다. **실패한 검사 둘이 바뀌었다**: 착수의 「회차 레코드」·「완수 조건 설계 평가」는 이제 ok. 둘 다 회차 그림자 |
| `cargo xtask test` | 0 | 330s | 통과 1123 · 실패 0 · 무시 3 (`test result` 57줄 합) | 1065/0/2 (54줄) → 1123/0/3 (57줄) |
| `cargo clippy --workspace --all-targets` | 0 | 248s | `^warning` 줄 154 · 중복 뺀 87 | 같은 셈법으로 착수 로그를 다시 세면 152 · 88. 착수 표의 「약 125」는 셈법이 달라 이것과 비교하지 않는다 |
| s0-corpus | 0 | 0s | 671 + 434 + 17 = 1122 (기대 1122) | 같다 |

## 표 — 항목 24

| # | 항목 | exit | 시간 | 어긋남 | 대조 불가 | 값 |
|---|---|---|---|---|---|---|
| 1 | s0-compare | 0 | 31s | 0 | 0 | 파일 1122/1122 · 선언 2315 · 불일치 0 |
| 2 | f22-1 | 2 | 583s | 2 | 0 | 성한 상태 실패 · ⑦ 「거주 불가 타입을 `built` 로」 변형이 아무것도 안 바꿨다 · 나머지 음성 7/7 |
| 3 | f22-2 | 0 | 63s | 0 | 0 | 성한 12 · 음성 7/7 |
| 4 | f22-3 | 0 | 47s | 0 | 0 | 음성 5/5 · Kotlin 발현 4/5 · 도입 4/5 |
| 5 | f22-4 | 0 | 506s | 0 | 0 | 격리 100회 부분갱신 0 · 음성 실패 0 |
| 6 | s1 | 0 | 3s | 0 | 0 | 경로 997 · parsed 671 · binary 8 · 캐시 적중 997 · 넷 통과 |
| 7 | s2 | 1 | 120s | 1 | 0 | 성한 응답 묶음에서 `② unresolved 이 not_built 가 아니다: {'present': []}` · 나머지 음성 8/8 |
| 8 | s3 | 0 | 1s | 0 | 0 | 다섯 통과 |
| 9 | f01 | 1 | 86s | 1 | 0 | ⑦ 골든 대장 다름 — detector 추출기 `f03-2` → `f10-nul-source` · ⑧ 10배에서 파일당 0.96배 |
| 10 | f02-1 | 0 | 32s | 0 | 0 | 음성 6/6 · S0 불일치 0 · 다섯 통과 |
| 11 | f02-2 | 0 | 50s | 0 | 0 | 음성 5/5 · 다섯 통과 |
| 12 | f02-3 | 0 | 13s | 0 | 0 | 불변식 5/5 · 해소 파일안 76048 / 파일밖 9862 |
| 13 | f02-4 | 0 | 192s | 0 | 0 | 음성 통과 · 10배에서 파일당 0.96배 · 상주 1.42배 |
| 14 | f03-1 | 0 | 754s | 0 | 0 | 여섯 통과 · 결정성 ditto 4656 · pb 1340 |
| 15 | f03-2 | 0 | 35s | 0 | 0 | 불변율 100.00% ×7 (심볼 4656) · 가시율 100.00% (짝 357/357) |
| 16 | f03-3 | 1 | 360s | 1 | 0 | ④ `cargo xtask check` 실패 (로그가 잘려 어느 검사인지 안 남음) · ①② ok · ③ ditto 4656 · pb 1340 움직인 것 0 |
| 17 | f04 | 1 | 111s | 1 | 0 | ⑨ ditto 고정비 뺀 9.3배 (선 10 · 옛 선 6.0배) · pb 87.2배 · ①③④⑤⑦⑧ ②⑥ ok |
| 18 | f05 | 1 | 14s | 1 | 1 | ⑨ ditto 8.9배 (선 10) · pb 90.5배 · ditto 노드 4656 · 엣지 9764 · 생략 실린 답 7 / 대조 불가 ③ pb 엣지 0 |
| 19 | f06 | 1 | 215s | 3 | 2 | ① 변형 전 xtask check 가 이미 실패 (위 준비 칸의 두 검사) · ③ 코퍼스 대조값 ditto 노드 4656 · 엣지 9764 (F05 는 4578·4601) · ③ 내보내기 건수 Symbol 4656 · REFERENCES 4682 (`graph.dump` 는 4656·9764) / 불가 ② pal-mcp 폐기 · ③ Cypher 파서 없음 |
| 20 | f09 | 1 | 147s | 5 | 0 | ① 형식 변형 다섯(prettier@3·들여쓰기·개행·주석·후행 공백) 전부 「변형 **전**에 이미 fresh 가 아닌 것이 있다 {'fresh','undeterminable'}」 · ⑥ 비율 하한 ok (사유 `partial_parse`·`projection_stale` 2) · ⑦ symbol 20 · callers 45 · closure:2 70 · closure:3 87 |
| 21 | f10 | 1 | 256s | 3 | 3 | ② 강 신호 미결박 ditto 63.7% (1884/2956) · pb 87.3% · pb 3분류 `bound` 없음 / 불가 ① 층화 · ⑨ ditto · ⑨ pb |
| 22 | f10-5 | 0 | 118s | 0 | 1 | 불가 ⑧ 도달 불가가 된 사유 (모집단 0) · ditto 후보 935 · 미결박 3197 |
| 23 | f10-6 | 0 | 139s | 0 | 4 | 불가 ② pb 표식주석 · ⑥ Kotlin 실코퍼스 · ⑨ ditto-6 · ⑨ pb-6 |
| 24 | f11 | 0 | 30s | 0 | 3 | 불가 ① 재발 재현(떴다 2 · 안 떴다 1 · 불가 2) · ④ 상한과 생략 · ⑥ MCP(폐기) · ⑦ p95 1.9ms |
| | **합** | | | **18** | **14** | |

- 어긋남 18 = f22-1 2 · s2 1 · f01 1 · f03-3 1 · f04 1 · f05 1 · f06 3 · f09 5 · f10 3
- 대조 불가 14 = f05 1 · f06 2 · f10 3 · f10-5 1 · f10-6 4 · f11 3
- exit: 0 이 14, 1 이 9, 2 가 1 (f22-1). **스크립트마다 rc 가 착수와 전부 같다.**

## 착수 대비 차이

### rc
스물넷 모두 착수 → 종료 같음. 준비 칸의 rc 도 전부 같음.

### 새로 생긴 어긋남 (7)

| 항목 | 어긋남 | 원인 후보 (판정 아님) |
|---|---|---|
| f06 ③ 코퍼스 대조값 | ditto 노드 4578→4656 · 엣지 4601→9764 가 F05 에 박힌 4578·4601 과 다르다 | 노드 +78: 예고된 `ba0a983` 분류 변경(ditto 기호 골든 4,578 → 4,656 · `78ceb74` 가 골든 행 추가). 엣지 두 배는 예고 목록에 없다. 이 사이 `crates/` 를 바꾼 커밋 중 후보는 `f953a0c`(TS 지정자 파일 간 해소) · `b8eeb1d` · `d734c04` |
| f06 ③ 내보내기 건수 | 내보내기 REFERENCES 4682 인데 `graph.dump` 는 9764 | 같은 후보. 착수 때는 내보내기 REFERENCES 와 dump 엣지가 4601 로 같았다 |
| f09 ① ×5 | 형식 변형 다섯 모두 변형 전 결박에 이미 `undeterminable` 이 있다 | `ba0a983`. 전에 binary 였던 ditto 파일이 partial 이 되면서 `partial_parse` 사유가 새로 난다(아래 ⑥ 해소와 같은 뿌리로 보인다) |

### 사라진 어긋남 (1)

| 항목 | 착수 | 종료 | 원인 후보 |
|---|---|---|---|
| f09 ⑥ 비율 하한 [#58] | 사유가 `projection_stale` 하나뿐 | 사유 `partial_parse`·`projection_stale` 둘 | `ba0a983` (partial +5) |

### 그대로인 어긋남 (10 · 값이 움직인 것은 적는다)

| 항목 | 착수 → 종료 |
|---|---|
| f22-1 성한 상태 | 같다. 다만 안에서 읽는 xtask check 의 실패 검사가 바뀌었다(준비 칸) |
| f22-1 ⑦ 변형 무효 | 같다 |
| s2 ② `unresolved` | 같다 |
| f01 ⑦ 골든 대장 | detector 추출기 `f09-import-sites` → `f10-nul-source` (예고된 `EXTRACTOR_REV` 변경) |
| f03-3 ④ | 같다. 로그가 잘리는 것도 같다 |
| f04 ⑨ ditto [#61] | 8.8배 → 9.3배 (옛 선 5.7 → 6.0) · 착수 전부터 어긋남 |
| f05 ⑨ ditto [#61] | 8.5배 → 8.9배 · 추출 대상 491 → 496 파일 (예고된 binary→partial 5 와 수가 맞는다) |
| f06 ① 기준선 | 실패 사유가 「이 회차 raw 셋 기계 칸 갈림」에서 「죽은 링크 부재 · 원장 둘 대조」로 바뀌었다. 둘 다 회차 그림자 |
| f10 ② ditto 강 신호 미결박 | 64.4% (1905) → 63.7% (1884) · 후보 914 → 935 · 미결박 3218 → 3197 |
| f10 ② pb 둘 | 같다 (87.3% · `bound` 없음) |

### 대조 불가
14 → 14. 항목과 사유가 전부 같다.

### 어긋남·불가 밖에서 움직인 기록값

- 심볼 4578 → 4656: f03-1 ⑥ · f03-2 · f03-3 ③ · f05 · f06 ③ 이 모두 옮겨졌다 (예고된 골든 4,656). pb 1340 은 그대로.
- f03-2 짝지어진 파일 354 → 357 (요약 움직임 357 · 가시율 100%).
- f04 ⑨ pb 120.5배 → 87.2배 (선 위) · ⑧ ditto 평균 489 → 495 B.
- f05 ⑨ pb 86.0 → 90.5배 · ditto 생략 실린 답 2 → 7.
- f02-4 파일당 0.94 → 0.96배 · 상주 1.31 → 1.42배. f01 ⑧ 1.02 → 0.96배.
- f06 ② 코어 시험 267 → 277건 · ④ graph.dump 토큰 557775 → 755376.
- f09 ② 결박 16 → 14 (stale 13/13 · undeterminable 1) · ④ 고친 본문 7 → 8 · ⑦ callers 43→45 · closure:2 50→70 · closure:3 54→87.
- f10 ⑤ 스팬 동점 79 → 84 · f10-5 후보 셋 이하 640 → 655 · 후보 여럿 987 → 1008.
- f11 ⑦ p95 9.9ms → 1.9ms (결박 실린 좌표 2 → 1) · ③ 모집단 3566 → 3628 · ④ 3601 → 3665 · ⑧ 좌표 3373 → 3472 · 파일 145 → 150.
- f22-4 시간 286s → 506s.

## 정리

- f04 ③ 등의 소스 변이는 스크립트가 되돌렸다. 끝난 뒤 `crates/ scripts/ corpus/ schema/ surface/ xtask/` 가 HEAD 와 같다(`git diff --quiet HEAD` 확인).
- `git checkout -- corpus/tasks/` 로 f10·f10-5·f10-6·f11 이 덮어쓴 넷(`f10-5-binding-sample` · `f10-6-binding-sample` · `f10-f09-remeasure-ditto` · `f10-false-binding-sample`)을 되돌렸다. 추적 안 되는 `f10-f09-remeasure-ditto-6.tsv` 는 지웠다.
- ⚠ 정리 중 실수가 하나 있었다. `rm -f corpus/tasks/f10-f09-remeasure-*.tsv` 글롭이 **추적되는** `f10-f09-remeasure-ditto.tsv` · `f10-f09-remeasure-portal-backend.tsv` 도 지웠다. 곧바로 `git checkout -- corpus/tasks/` 로 HEAD 내용 그대로 되살렸고, 두 파일이 `git ls-files` 에 있고 diff 가 0 인 것을 확인했다.
- 끝난 뒤 `git status`: 착수 전부터 있던 추적 밖 `docs/gates/first-release-elsewhere.md` 하나뿐이다. 이 러너는 커밋하지 않았다.

[#58]: https://github.com/hskim-ecoletree/palimpsest/issues/58
[#61]: https://github.com/hskim-ecoletree/palimpsest/issues/61
