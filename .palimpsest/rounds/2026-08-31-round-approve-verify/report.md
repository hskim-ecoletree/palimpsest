# 종료 보고 — round approve verify

> 회차 `2026-08-31-round-approve-verify` · 기준 `e9a1da7` · 이슈
> [#97](https://github.com/hskim-ecoletree/palimpsest/issues/97) · 판정
> [`docs/gates/round-approve-verify.md`](../../../docs/gates/round-approve-verify.md)

## 남지 않은 것

`pal round approve`/`verify`의 첫 소비 경로를 완결했다. 승인은 저장소 밖 사용자별 private
저장소에 exact repo·round·condition·oracle·CWD·shell·PATH·timeout·output identity로 남고,
승인·권한·identity가 틀리면 spawn 전에 닫힌다. 실행기는 Unix process group과 Windows
suspended Job Object로 자식 tree를 상한 내에 회수하고, stdout+stderr와 시간 예산을 같이
지킨다.

추적 파일 bytes를 다시 읽은 projected digest와 oracle·approval이 실행 전후 같을 때만
schema 2 evidence를 atomic complete-line ledger에 남긴다. schema 1 vector·reader·status JSON·exit
계약은 유지했다. positive와 관련된 negative control은 각각 현재 evidence가 있어야만
`met`이다. 등록만 된 control, stale evidence, role replay는 조건을 닫지 못한다.

잠긴 완수 조건은 **통과 15 · 반증 0 · 대조 불가 0 · 미측정 0**이다. 사전부검
21건과 독립 리뷰 2라운드 36건은 정정 33·기각 23·범위 밖 1, 열림 0으로 모두
처분했다. #97은 거짓 통과 대조와 실제 효과·세 OS 근거 뒤 CLOSED/COMPLETED로 닫혔다.

로컬은 approve/verify 22, round status 24, round scripts 15, hook 5, install hooks 20 시험,
`cargo xtask check` 23/23, workspace all-targets가 전부 통과했다. 기존 release 규모 benchmark
하나만 선언대로 ignored다. 구현 SHA `52a13bec5c53ff02a636429685d932f7e1bba713`의
[CI run 33346399805](https://github.com/hskim-ecoletree/palimpsest/actions/runs/33346399805)은
ubuntu·macOS·Windows, 두 producer와 양방향 consumer 일곱 job을 전부 성공시켰다.

## 효과

시험이 아닌 새 임시 Git 저장소에서 빌드된 `pal`을 쓰는 실제 회차를 돌렸다. 승인 전
verify는 exit 3으로 차단됐다. positive를 승인·실행해 evidence 하나를 남겨도 A1과
control은 모두 `pending`이었다. known-broken control을 별도 승인·실행해 두 번째 현재
evidence가 생긴 뒤에만 둘과 aggregate가 `met`이 됐다. 입력·출력 전문은
[`effect/observation.md`](effect/observation.md)에 보존했고 공유 산출물에 임시 절대 경로·승인
identity·raw oracle output을 싣지 않았다.

## 범위 밖

- Stop 등록·차단과 진행 인지형 자기 상한은 목표 안이지만 이 회차의 우선순위가 아니다.
  먼저였던 것은 #85가 소비할 approve/verify·negative-control 계약이며 이제 완성됐다.
- untracked·ignored filesystem 전체 snapshot과 oracle side effect의 transactional rollback은 목표 밖이다.
  이 표면의 계약은 tracked projection의 실행 전후 currentness 관측과 폐기이다.
- finding·judgment 조건 실행과 과거 schema 1 회차의 소급 migration은 목표 밖이다. 이 회차보다
  먼저였던 것은 command oracle의 실제 소비 경로와 #97 거짓 닫힘 차단이었다.

## 원리상 못 잰 것

없음. 이 회차가 등록한 A1~A15는 RED·음성 대조, 로컬 시험, 비시험 효과, 외부 CI와
구조화 원장으로 모두 측정했다.

## 능력 부재

착수 조회에서 그래프는 `oracle_digest`·`ConditionsReport`·CLI·worktree 좌표를 냈지만
`round::ledger`·`round::status`를 이름으로 답하지 못했고 caller/reaches 관계도 자기를 벗어나지
못했다. F07 cross-file resolution·F13 effects·F15 judgment를 추정으로 채우지 않고 관련
경로만 문자열 탐색했다.

구현 SHA의 `pal doctor --full` 그래프 위반·residual·coverage gap은 0이다. 현재 산출
능력으로는 F05 graph storage·F08 unresolved refs·F15 judgment·F17 synthesis·F20 conformance가
부재하다. 설치하지 않은 개발 worktree의 install residual은 그래프 판정과 분리했다.

## 의도 변화

schema 1, 기존 oracle digest vector, condition/status JSON·exit·상태 전이와 hook unknown-input
fail-open은 바꾸지 않았다. schema 2는 `negative_for`와 projected digest를 명시하는 새 회차만
쓰고 version별 unknown 거부와 schema 1 read compatibility를 같이 잰다.

독립 리뷰 R1 뒤 승인할 수 있는 shell을 임의 absolute executable에서 platform default 하나로
강화했다. R2 뒤 Windows helper를 환경 문자열·`icacls`·`taskkill`에서 Known Folders·current
token SID·protected DACL·suspended Job Object로 전환했다. 둘 다 실행 표면을 넓히거나
완수 조건을 완화한 변경이 아니라 같은 의도를 fail-closed 장치로 정정한 것이다.
