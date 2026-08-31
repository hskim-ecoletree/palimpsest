# 사전부검 원 반환 — 2라운드

## 구조·활성화 위험

1. `EVENTS`에 Stop을 추가하는 순간 기본 install/update가 자동 등록해 활성화까지 된 것으로
   오인될 수 있다. catalog와 활성 인스턴스를 별도 타입으로 둘 것. **금지역**.
2. verification.log가 없는 열린 회차를 active 검색이 놓쳐 Stop이 통과할 수 있다. 명시 slug
   또는 ledger 없는 open round를 block하는 RED가 필요하다. **금지역**.
3. counter를 프로젝트 안에 쓰면 schema 2 projected digest를 바꿔 evidence를 stale로 만든다.
   projected tree 밖 private store에 둘 것. **금지역**.
4. raw ledger digest는 표현·순서·timestamp를 진행으로 오인하고 StatusView만 쓰면 evidence
   의미 일부가 사라진다. typed semantic projection을 둘 것. **실패**.
5. digest 변화만 진행으로 보면 A→B→A 진동과 regression이 상한을 피한다. 단조 진척 순위와
   bounded seen state를 둘 것. **실패**.
6. settings hook 존재로 activation을 추론하면 매니페스트·설정 손상과 사용자 항목이 섞인다.
   별도 activation record를 둘 것. **실패**.
7. disable/update가 exact owned entry 규율을 우회하면 사용자 hook을 지우거나 activation을
   잃는다. enable→update→disable 왕복을 잴 것. **실패**.

## 재시작·경합·적대 입력 위험

1. session ID만으로는 같은 세션의 새 시도와 stale replay를 못 가른다. transcript의 단조
   identity 또는 내용 hash를 사건 identity에 포함할 것. **금지역**.
2. create_new lock 잔해는 crash 뒤 영구 잠금이 될 수 있다. crash 후 재획득 가능한 lock과
   원자 replace를 쓸 것. **금지역**.
3. 상태 key가 round만이면 프로젝트가 충돌하고 session만이면 같은 session이 충돌한다.
   portable project+round identity로 가를 것. **실패**.
4. status read 중 verify가 append하면 혼합 snapshot을 볼 수 있다. bounded reread 또는 안정
   identity 대조 뒤 판정할 것. **실패**.
5. malformed를 전역 fail-closed로 바꾸면 기존 unknown/SubagentStop 계약을 깨고, 전역
   fail-open이면 활성 Stop이 빠져나간다. event→reentry→activation→active payload/state 순서로
   dispatch할 것. **금지역**.
6. reentry guard가 config/status/lock보다 뒤면 손상 상태에서 무한 루프가 난다. 최우선 시험을
   둘 것. **금지역**.
7. cap handoff가 intent/verification/report/folded를 건드리거나 기록 실패 후 pass하면 거짓
   종료다. operational state만 쓰고 기록 실패는 block할 것. **금지역**.
8. doctor probe는 transport만 재므로 실제 active block/pass 효과를 별도 격리 세션에서 잴 것.
   **실패**.
9. catalog 밖에 이벤트 문자열 목록이 남으면 #86이 재발한다. dispatch/install/doctor 집합
   동치를 시험할 것. **실패**.
10. block reason에 절대 경로·oracle command·raw output을 넣으면 정보가 샌다. slug와 bounded
    상태 요약만 넣고 길이·비밀 음성 대조를 둘 것. **거짓신호**.

## 처분

- 계획 수정: 3, 4, 5, 6, 재시작 1~7, 9, 10.
- 탐지 수단: 실제 Claude 효과 관측, 안정 snapshot stress, block reason 비밀·길이 대조.
- 완수 조건: ledger 없는 open round, reentry 최우선, cap의 round 파일 불변, catalog 집합 동치.
- 기각: Stop을 기본 설치 집합에서 빼자는 제안. 소유자는 등록과 활성화를 분리하고
  “Stop이 등록돼도 승인·활성화 전에는 차단되지 않음”을 직접 요구했다. 등록은 유지한다.
