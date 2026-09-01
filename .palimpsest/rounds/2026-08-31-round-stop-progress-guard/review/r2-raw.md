# 독립 리뷰 R2 — hook transport와 소비 경계

> 잰 대상: 구현 커밋 `4ca7bc0`. 실제 hook payload, cwd, transcript, install catalog와
> rollback 경계를 공격했다. 당시 아직 없던 효과 관측·최종 CI·종료 보고는 미측정으로
> 남겼다.

## 이번 라운드의 새 발견

| # | 발견 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 근거 |
|---|---|---|---|---|---|---|---|
| N1 | Claude Code가 nested cwd를 보내면 저장소 root를 open하지 못해 활성 Stop이 fail-open한다 | 자기장치 | 참 | 금지역 | B1 | `crates/pal-cli/src/round/stop.rs:312` | cwd 자체를 repository root로 가정해 GixRepo::open한다 |
| N2 | 8 MiB를 넘는 정상 transcript를 거부해 장기 session의 counter가 영구히 증가하지 않는다 | 자기장치 | 참 | 실패 | E1 | `crates/pal-cli/src/round/stop.rs:426` | transcript 전체 read 전에 고정 크기 상한으로 bail한다 |
| N3 | terminal enum만 믿고 종료문 필수 절을 읽지 않아 fixture용 빈 report가 실제 pass가 될 수 있다 | 자기장치 | 참 | 금지역 | B3 | `crates/pal-cli/src/round/stop.rs:280` | R1-N1과 독립적으로 같은 거짓 통과를 재현했다 |
| N4 | 의미 진행 event가 counter 1로 기록돼 여섯 번보다 일찍 상한에 닿는다 | 자기장치 | 참 | 실패 | D2 | `crates/pal-cli/src/round/stop.rs:468` | R1-N2와 독립적으로 counter 전이를 대조했다 |

## 내가 기각한 것

| # | 기각한 것 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 왜 아니었나 |
|---|---|---|---|---|---|---|---|
| X1 | install settings와 hook dispatch가 서로 다른 event 목록을 가진다 | 자기장치 | 거짓 | 실패 | G2 | `crates/pal-cli/src/hook/catalog.rs` | 양쪽 모두 같은 catalog iterator를 소비한다 |
| X2 | unknown event가 새 Stop 정책 때문에 fail-closed한다 | 자기장치 | 거짓 | 실패 | C2 | `crates/pal-cli/src/hook/policy.rs` | catalog 밖 event는 기존 UnknownEvent fail-open을 보존한다 |
| X3 | malformed active Stop payload가 transport 오류로 조용히 열린다 | 자기장치 | 거짓 | 금지역 | C2 | `crates/pal-cli/src/hook/policy.rs` | activation 확인 뒤 missing/wrong-type guard를 block한다 |
| X4 | uninstall이 activation을 남긴다 | 자기장치 | 거짓 | 실패 | F2 | `crates/pal-cli/src/install.rs` | uninstall 경로가 disable을 먼저 호출한다 |
| X5 | SubagentStop 결정이 Stop 구현으로 바뀌었다 | 자기장치 | 거짓 | 실패 | G1 | `crates/pal-cli/src/hook/policy.rs` | event별 dispatch가 기존 SubagentStop decide를 그대로 호출한다 |

## 미측정 목록

- 실제 Claude Code transport의 inactive/block/progress/pass/cap/re-entry/disable 장면.
- 마지막 pushed SHA의 세 운영체제와 양방향 상호운용 CI.
- ADR·게이트·그래프 결박·종료 보고. 리뷰 시점에 아직 산출되지 않았다.

## 끝내도 되는가

안 된다. N1~N4와 미측정 효과·CI·종료 산출을 닫아야 한다.
