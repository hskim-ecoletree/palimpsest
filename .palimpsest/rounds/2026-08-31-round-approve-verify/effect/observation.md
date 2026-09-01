# 실제 격리 효과 관측

보강 구현 `9019f93`의 빌드된 `pal`을 시험 harness가 아닌 새 임시 Git 저장소에서 다시
실행했다. approval 저장소도 그 저장소 밖의 별도 임시 디렉터리였다. 절대 경로, 사용자별 값,
approval digest는 공유 산출물에 보존하지 않았다.

## 전이

1. 승인 전 `pal round verify --id A1 --json`
   - exit `3`
   - `outcome=approval_required`
   - evidence `0`행
2. `pal round approve --id A1` 뒤 positive oracle 실행
   - 실행 exit `0`, EXPECT 일치
   - status aggregate `in_progress`
   - A1 `pending`, A1-n `pending`
   - positive 성공만으로 주 조건이 `met`이 되지 않았다
3. `pal round approve --id A1-n` 뒤 known-broken negative control 실행
   - 실행 exit `0`, EXPECT 일치
   - status aggregate `met`
   - A1 `met`, A1-n `met`
   - evidence `2`행

fixture의 `verification.log`를 제외한 tracked tree는 전후 동일했고, oracle이 만든 다른 변경은
없었다. `intent.md`와 초기 schema 2 원장을 포함한 재현 입력 원본은
[`template/`](template/)에 있다.
