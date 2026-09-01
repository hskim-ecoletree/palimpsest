# 효과 관측 — 실제 진행 중 회차 소비

시험 runner가 아니라 빌드된 `pal`을 fixture 프로젝트의 작업 디렉터리에서 직접 실행했다.

```text
$ target/debug/pal round status --round effect-round --json
{"outcome":"status","round":"effect-round","verification":"in_progress","terminal":"open","conditions":[{"id":"A1","state":"met","oracle_digest":"4cf3cb926ab8249a040632d0c1e694509ab40eee2eacc8da15d1353392b026dd"},{"id":"A2","state":"pending","oracle_digest":"4cf3cb926ab8249a040632d0c1e694509ab40eee2eacc8da15d1353392b026dd"}]}

$ target/debug/pal round status --round effect-round
round: effect-round
verification: in_progress
terminal: open
A1: met
A2: pending
```

JSON과 사람 출력은 같은 fixture에서 A1을 `met`, A2를 `pending`, aggregate를
`in_progress`, terminal을 `open`으로 냈다. 이 관측은 상태를 실제 소비하는 장면이며
시험 통과를 효과로 다시 이름 붙인 것이 아니다.
