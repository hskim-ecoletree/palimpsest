# 독립 리뷰 R1 — 상태 불변식

> 잰 대상: 구현 커밋 `4ca7bc0`. 상태 reducer, terminal, progress, 동시성의 거짓 통과를
> 공격했다. 효과 관측·최종 CI·종료 보고는 아직 생기기 전이라 판정하지 않았다.

## 이번 라운드의 새 발견

| # | 발견 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 근거 |
|---|---|---|---|---|---|---|---|
| N1 | report.md 또는 folded.md 파일 존재만으로 terminal을 믿어 빈 골격 종료문도 통과한다 | 자기장치 | 참 | 금지역 | B3 | `crates/pal-cli/src/round/stop.rs:280` | reducer의 terminal enum 뒤에 종료문 내용 검증이 없다 |
| N2 | 최고 rank 진척 직후 no-progress counter를 0이 아니라 1로 둔다 | 자기장치 | 참 | 실패 | D2 | `crates/pal-cli/src/round/stop.rs:468` | 진척 사건 자체를 무진행 1회로 센다 |
| N3 | 문법상 valid JSON이지만 counter·handoff·rank가 모순된 progress record를 받아 거짓 blocked handoff를 만들 수 있다 | 자기장치 | 참 | 금지역 | E2 | `crates/pal-cli/src/round/stop.rs:500` | deserialize 뒤 semantic invariant 검증이 없다 |
| N4 | 오래된 lock 파일을 지운 뒤 새 lock을 만드는 lease는 다른 process의 새 inode를 지우는 ABA race가 있다 | 자기장치 | 참 | 실패 | E2 | `crates/pal-cli/src/round/stop.rs:571` | timestamp stale 판정과 remove/create 사이가 원자적이지 않다 |

## 내가 기각한 것

| # | 기각한 것 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 왜 아니었나 |
|---|---|---|---|---|---|---|---|
| X1 | registration이 Stop을 자동 활성화한다 | 자기장치 | 거짓 | 금지역 | A1 | `crates/pal-cli/src/hook/catalog.rs` | catalog에는 event와 command만 있고 activation은 private store의 별도 record다 |
| X2 | stop_hook_active guard보다 상태 판정이 먼저 돈다 | 자기장치 | 거짓 | 실패 | C1 | `crates/pal-cli/src/hook/policy.rs` | dispatch 직후 true를 먼저 통과시킨다 |
| X3 | Stop 판정이 approve/verify/oracle을 실행한다 | 자기장치 | 거짓 | 금지역 | F1 | `crates/pal-cli/src/round/stop.rs` | status reducer와 private record I/O만 호출한다 |
| X4 | timestamp나 JSON 순서가 semantic progress를 reset한다 | 자기장치 | 거짓 | 거짓신호 | D1 | `crates/pal-cli/src/round/stop.rs` | 정렬된 condition state와 aggregate/terminal만 digest한다 |
| X5 | 손상 activation 때문에 disable도 실패한다 | 자기장치 | 거짓 | 실패 | F2 | `crates/pal-cli/src/round/stop.rs` | disable은 activation body를 parse하지 않고 identity 경로를 제거한다 |

## 미측정 목록

없음. 효과 관측·최종 CI·종료 보고는 이 리뷰의 대상 commit에 아직 없어서 리뷰 모집단에서
제외했으며, 회차 종료 조건으로는 별도로 남아 있다.

## 끝내도 되는가

안 된다. N1~N4를 닫고 해당 음성 대조를 다시 실행해야 한다.
