# 접힘 — PR #91은 병합 할 수준이 아니다

> 회차 `2026-09-01-agent-laziness-merge-evaluation`를 2026-09-01에 접었다.
> 평가는 끝났지만 선행 합격선이 반증돼 병합 조건은 성립하지 않았다.

## 왜 접었나

목표 안이지만 지금 먼저 할 일은 병합이 아니라 [#101](https://github.com/hskim-ecoletree/palimpsest/issues/101)의 병합 차단 결함을 닫는 일이다. schema 1 stale evidence와 본문 없는 종료문이 Stop을 통과했고, shallow clone에서 핵심 명령이 실패했다. 더 크게는 이 구현이 스스로 잠근 실행 계획 §8의 전체 종료선을 아직 완수하지 않았다.

사유는 AGENTS.md의 둘째 접힘, “목표 안이지만 지금 우선순위가 아님”이다. 먼저인 것은 #101이며 #95·#96을 native blocking edge로 연결했다.

## 접으면서 남기는 것과 버리는 것

남긴다: 최신 main을 합친 로컬 후보, 성공한 `cargo xtask check`·`cargo xtask test`, 구조적 doctor 판정, 독립 공격 원문과 #101의 판정 가능한 합격선을 남긴다.

버린다: 이 상태를 “전체 테스트가 초록”이라는 이유로 병합하는 판정, 예전 PR SHA의 CI를 현재 로컬 merge 결과의 CI로 대체하는 판정을 버린다.

## 다음에 여는 것

#101이다. 그 이슈의 합격선은 우회 두 건, shallow clone 경계, 실행 계획 §8 미완수 조건, portable gate, 독립 재검토와 최종 SHA CI를 모두 담는다.
