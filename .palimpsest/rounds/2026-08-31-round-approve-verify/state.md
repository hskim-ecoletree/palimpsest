# 상태 — round approve verify

## 지금 단계

의도 잠금과 승인, 인터뷰 1라운드, 사전부검 1라운드, 그래프 우선 조회, RED, 구현, 실제 격리
효과와 로컬 전량 검증을 마쳤다. 기준 HEAD는
`e9a1da762e42d6cf1f4b2424387f8ec1fe668ee6`이다. 독립 리뷰 상한은 2라운드다.

## 승인

소유자의 첫 프롬프트가 수직 경로, 공격 모집단, 구현 순서, 검증과 종료까지 명시해 잠긴
의도를 승인한 것으로 기록한다. 축소·전환 또는 schema 1 의미 변경은 이 승인에 포함되지 않는다.

## 이슈

#97을 `round approve/verify — 실행된 음성 대조 없이는 조건을 met으로 닫지 않는다`로 갱신하고
`hskim-ecoletree`에 assign했다. 완료 조건은 이 intent의 실행·currentness·negative-control·세 OS
종료선을 기계 판정 가능하게 반영한다.

## 그래프 우선 조회

`oracle_digest`, `ConditionsReport`, `Command`, `RoundCommand`, `WorktreeState`의 Rust 파일과
span은 답했다. `round::ledger`와 `round::status`는 unknown, caller는 전부 0, reaches는 자기
자신뿐이었다. F07 cross-file resolution, F13 effects, F15 judgment가 능력 부재다. 그 뒤 탐색은
`crates/pal-cli/src/round/**`, `main.rs`, `round_status.rs`, `pal-git::WorktreeState`, 설치 Python
소비자로 제한했다.

## 인터뷰

상한 1라운드. 경계·의도·자율·종료·재고를 모두 열었고 잠근 값은 `intent.md`가 진다.

## 사전부검

상한 1라운드. 21개 시나리오를 받았다. approval 결박/권한/TOCTOU, projected digest 모집단,
pre/post currentness, EXPECT 극성, negative-control 비공허성, replay, torn append, process tree,
fixture 탈출, 기존 소비 호환과 CI skip을 RED·완수 조건으로 편입했다. 원 반환문은
`premortem/r1-raw.md`에 보존한다.

## 구현과 효과

외부 exact approval, tracked projection digest, bounded executor, append writer, schema 2,
approve/verify CLI와 current negative-control reducer를 세웠다. 실제 격리 fixture에서 승인 전
exit 3, positive 뒤 pending, negative-control 실행 뒤 met을 관측했다. hook은 점등하지 않았다.

## 검증

approve/verify 22, round status 24, round scripts 15, hook 5, install hooks 20개가 각각 통과했다.
`cargo xtask check` 23/23과 `cargo test --workspace --all-targets`도 통과했다. 기존 release 규모
benchmark 하나만 선언대로 ignored다.

## 독립 리뷰

R1은 승인 보류와 7개 발견을 냈다. racy stat, 부모 선종료 descendant, control role replay,
Windows ACL/temp fallback, unbounded·unbound cleanup helper, torn append를 정정했다. 임의 shell은
platform default 하나로 닫았다. mutable workspace를 immutable snapshot으로 실행하라는 항은
이 표면의 계약이 current workspace의 전후 동일성 관측과 폐기이며 side effect transaction이
아님을 ADR에 명시해 범위밖 판정했다. R2는 이 처분과 racy-stat·control-role 폐쇄를 인정했다.

R2의 새 반증은 Windows 환경 기반 ACL helper 선실행, owner 미검증, 죽은 root PID의 taskkill,
플랫폼 한쪽만의 directory sync 주장, append 뒤 reader 파일 상한 초과, drain read 오류 축약,
게이트 숫자 drift 일곱이다. helper를 없애고 Known Folders·current token SID owner/protected 단일
DACL·suspended Job Object로 전환했으며 atomic visibility 계약·append 상한·read 오류 전파와
대조를 세웠다. Windows GNU cross-check는 Rust std target을 설치한 뒤 제품 코드 전에 zstd C
compiler 부재로 멈췄고, 실제 Windows compile/runtime은 A14 CI가 판정한다.

## 실패한 접근

`rustfmt`를 crate root에 직접 주어 module tree의 기존 비정형 파일까지 포맷한 접근은 두 번
불필요한 diff를 만들었다. 매번 의미 변경 파일을 보존하고 나머지 포맷 diff만 역적용했으며,
최종 diff에서 관련 없는 파일이 0임을 다시 확인했다.

## 남은 것

독립 리뷰, 발견 원장 처분, 그래프 결박, 종료 보고, push/세 OS CI, #97 종료.
