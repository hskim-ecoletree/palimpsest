# 사전부검 R1 처분

| 발견 | 처분 |
|---|---|
| P1 | G0에 로컬 HEAD·PR 최종 SHA·현재 main 포함 대조를 추가했다 |
| P2 | `origin/main`을 fetch해 브랜치에 먼저 병합했고, 그 결과에서 검사를 실행했다 |
| P3 | G3를 문자열 포함 검사에서 구조적 JSON 검사로 고쳤다 |
| P4 | `round_approve_verify`·`round_stop`·`install_hooks`의 공격 시험 목록과 결과를 A1 근거로 대조한다 |
| P5 | 실행 계획의 artifact 계약과 `cargo xtask check`를 다시 실행했다. 사라진 외부 `gate-lint.mjs`는 현재 `gate-check --status`의 strict parse로 대체 확인했다 |
| P6 | G4는 독립 반환문 원문과 발견별 처분을 증거로 삼는다 |
| P7 | G5는 최종 push 뒤 PR API의 exact head SHA와 일곱 check 이름·결론을 확인한다 |
| P8 | G6는 PR merge commit과 fetch한 `origin/main`의 조상 관계를 확인한다 |
