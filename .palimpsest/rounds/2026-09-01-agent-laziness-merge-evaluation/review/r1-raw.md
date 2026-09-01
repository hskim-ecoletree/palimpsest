# 독립 검토 R1 원문

| # | 요약 | 경로 | 모집단 | 유효성 | 해악도 | 조건 |
|---|---|---|---|---|---|---|
| R1-01 | 승인·실행 출처를 확인하지 않은 schema 1 evidence가 met가 된다 | crates/pal-cli/src/round/status.rs | 자기장치 | 참 | 거짓신호 | A1 |
| R1-02 | schema 1 evidence가 tracked 변경 뒤에도 met이고 Stop이 완료 근거로 쓴다 | crates/pal-cli/src/round/stop.rs | 자기장치 | 참 | 금지역 | A1 |
| R1-03 | 필수 heading만 있고 본문이 빈 report·folded가 Stop을 통과한다 | crates/pal-cli/src/round/stop.rs | 자기장치 | 참 | 금지역 | A1 |
| R1-04 | shallow clone에서 missing parent object로 approve·Stop enable이 서지 않는다 | crates/pal-cli/src/round/approval.rs | 저장소 | 참 | 실패 | A1 |
| R1-05 | 실행 계획 §8의 재실행·비결정론·finding·이슈 처분 종료선이 닫히지 않았다 | docs/agent-laziness-executable-implementation-plan.md | 원의도 | 참 | 금지역 | A2 |
| R1-06 | 기존 실행 계획 gate가 절대 경로와 사라진 외부 도구를 담는다 | .palimpsest/rounds/2026-08-30-agent-laziness-executable-plan/GATES.md | 회차기록 | 참 | 실패 | A2 |

## 내가 기각한 것

- “shallow clone에서 enable 후 deepen하면 identity가 바뀌어 inactive fail-open이 된다”는 가설은 기각했다. enable 자체가 missing parent object로 먼저 실패했다.

## 미측정 목록

- 없음. 최종 PR SHA CI는 선행 합격선이 반증되어 의도적으로 push하지 않았으며 A5에 대조불가로 판정했다.
