# 합 — 최종 판정

교정 SHA `45d483d`에서 독립 R2가 focused 25/28/23, 정책 23/23, 전체 시험과 doctor 구조 검사를
다시 실행했다. 첫 최종-SHA CI가 드러낸 Windows fixture 수명 결함은 `2c068aa`에서 닫았고,
독립 R3는 writer 종료 뒤에도 직접 위조·전수 재실행·경쟁·currentness 공격이 유지되며 제품의
열린-writer 경로는 fail-closed임을 확인했다. 새 병합 차단 발견은 0이다. 로컬 aggregate는
met이고, 최종 SHA CI 7개·병합·origin/main 포함은 추적 파일을 다시 쓰지 않는 외부 terminal
순서로만 판정한다.
