# 효과 관측

구형 schema 1 성공, tracked tree 변경, 본문 없는 종료문, 주석·fence 안 가짜 heading, 열린
금지역·실패, malformed findings, 재실행 중 원장 변조, shallow identity와 store locator 갈림을
각각 정상 fixture와 같은 시험에서 대조했다. 교정 뒤에는 정상 경로만 complete/Stop 통과를
냈고 모든 공격 경로는 `in_progress`, `discarded`, 또는 차단으로 닫혔다.

`cargo xtask check` 23/23, `cargo xtask test`, 구조적 full doctor, 독립 R2가 성공했다. 세 OS의
동일 결과는 이 추적 산출을 고정한 최종 PR SHA의 CI 7개가 외부에서 판정한다.
