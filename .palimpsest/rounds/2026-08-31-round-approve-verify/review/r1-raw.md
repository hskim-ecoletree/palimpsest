| # | 발견 | 모집단 | 유효성 | 해악도 | 좌표 | 조건 | 근거·권고 |
|---|---|---|---|---|---|---|---|
| 1 | approval·shell·projection 확인과 spawn이 원자적이지 않다 | 원의도 | 참 | 금지역 | `approval.rs:48-74,161-172`, `verify.rs:104-131,145-153` | A1, A2, A5 | 두 번째 `is_approved` 직후 record를 제거해도 spawn한다. shell 교체나 projected input 변경도 사후 evidence 폐기만 할 뿐 실행 부작용을 막지 못한다. 승인 record·실행 파일을 열린 handle 또는 불변 copy에 고정하고 승인된 immutable snapshot에서 실행하라. |
| 2 | projected digest가 index stat cache를 신뢰해 내용 변경을 놓칠 수 있다 | 저장소 | 참 | 거짓신호 | `pal-git/src/lib.rs:492-514,594-635`, 특히 `619-622` | A4, A5 | 같은 크기의 tracked 파일을 바꾸고 index와 같은 mtime으로 복원하면 index blob id가 재사용된다. approval/status용 projection은 stat fast path 없이 모든 tracked blob을 다시 해시하라. |
| 3 | 부모 shell이 먼저 종료하면 output cap·timeout·process-tree cleanup을 우회한다 | 원의도 | 참 | 실패 | `verify.rs:274-331,334-363` | A3 | stdout/stderr를 상속한 background descendant를 띄우고 부모가 exit 0 하면 wait loop가 끝난다. drain thread는 join하지 않고 20ms만 기다려 descendant·초과 출력·부정확한 evidence가 남을 수 있다. root exit와 pipe EOF를 함께 bounded 관측하고 process group 또는 Windows Job 전체를 회수하라. |
| 4 | 기존 성공 evidence를 ledger 편집만으로 negative control 성공으로 재분류할 수 있다 | 원의도 | 참 | 거짓신호 | `ledger.rs:168-178,211-216,255-263`, `status.rs:233-268`, `verify.rs:161-170,227-231` | A9 | `negative_for`는 oracle digest와 evidence에 결박되지 않고 ledger는 projection에서 제외된다. 기존 성공 oracle 행에 `negative_for`만 추가하면 재실행·재승인 없이 current control로 인정된다. schema 2 digest 또는 evidence binding에 역할을 포함하고 역할 변경 시 stale 처리하라. |
| 5 | Windows approval 저장소는 private owner·permission 계약을 검사하지 않는다 | 규약 | 참 | 금지역 | `approval.rs:205-215,234-244,296-303` | A1 | Windows 구현은 regular file과 symlink 여부만 검사하며, `LOCALAPPDATA`가 없으면 shared temp로 fallback한다. owner SID와 DACL을 검사·설정하고 private user directory가 없으면 fail-closed하라. |
| 6 | Windows cleanup helper가 승인 identity에 결박되지 않고 실행도 bounded하지 않다 | 규약 | 참 | 금지역 | `approval.rs:271-293`, `verify.rs:392-417` | A3 | 환경값 셋의 문자열 일치만으로 `taskkill.exe`를 신뢰한다. 별도 shell 사용 시 helper bytes/path는 승인되지 않으며 `.status()`에는 timeout도 없다. Windows Job Object를 사용하거나 helper identity를 결박하고 별도 timeout·kill 경계를 두라. |
| 7 | evidence append는 torn line을 남길 수 있고 currentness 확인과 직렬화되지 않는다 | 원의도 | 참 | 실패 | `verify.rs:133-171,420-449` | A6 | partial `Write::write`는 오류를 반환해도 prefix를 남긴다. post-state 확인은 append lock 전에 끝나므로 이후 교체된 ledger에도 append할 수 있다. lock을 재검사 전부터 유지하고 no-follow identity를 검증하며, atomic replace 또는 복구 가능한 framed journal을 사용하라. |

## 내가 기각한 것

| # | 발견 | 모집단 | 유효성 | 해악도 | 좌표 | 조건 | 근거·권고 |
|---|---|---|---|---|---|---|---|
| K1 | schema 1 호환이 파괴됐다 | 원의도 | 거짓 | 거짓신호 | `ledger.rs:159-160,195-203`, `round_status.rs` | A7 | schema 1은 `negative_for`와 `projected_digest`를 계속 거부하고 기존 suite가 통과했다. |
| K2 | 잠긴 oracle digest vector가 변경됐다 | 자기장치 | 거짓 | 거짓신호 | `ledger.rs:255-263,336-341` | A7 | 기존 vector가 유지된다. |
| K3 | 새 approve/verify 경로가 hook을 점등했다 | 원의도 | 거짓 | 금지역 | `main.rs:549-571` | A11 | hook과 round dispatch는 분리되어 있고 새 경로가 hook을 호출하지 않는다. |
| K4 | 동시 교체가 없는 정상 경로에서도 승인 없이 spawn한다 | 자기장치 | 거짓 | 금지역 | `verify.rs:104-131` | A2 | 정상 경로는 차단된다. 유효한 실패는 발견 1의 TOCTOU다. |
| K5 | portable repository identity에 절대 repo path가 포함됐다 | 원의도 | 거짓 | 실패 | `approval.rs:48-96` | A1 | repo 절대 경로 대신 first-parent root commit을 사용하며 ADR의 portable identity와 양립한다. |
| K6 | 기존 foreground timeout·cap 시험 자체가 실패한다 | 자기장치 | 거짓 | 실패 | `round_approve_verify.rs:367-407` | A3 | 기존 시험은 통과했다. 부모 선종료 descendant 모집단이 빠진 것이 발견 3이다. |
| K7 | effect 산출물이 절대 경로나 approval digest를 누출한다 | 회차기록 | 거짓 | 금지역 | `effect/observation.md:1-25` | A12 | 지정된 effect 파일에서 누출을 발견하지 못했다. |

## 미측정 목록

| # | 발견 | 모집단 | 유효성 | 해악도 | 좌표 | 조건 | 근거·권고 |
|---|---|---|---|---|---|---|---|
| M1 | 마지막 pushed SHA의 Ubuntu·macOS·Windows 및 양방향 상호운용 CI | 규약 | 추정 | 실패 | `intent.md:38`, `round-approve-verify.md:26` | A14 | 게이트 자체가 미측정으로 기록한다. 마지막 pushed SHA의 실제 job 결과가 필요하다. |
| M2 | 독립 리뷰 발견의 구조화 원장 처분과 회차 종료 장치 | 회차기록 | 추정 | 실패 | `intent.md:39`, `round-approve-verify.md:26` | A15 | 이 리뷰 시점에는 발견 처분·종료 보고·이슈 종료가 아직 측정되지 않았다. |

검토한 commit SHA: `d5216e81e5812aab26c1aff1f9c7a86b0851172f`

## 원문 전문

> 역할: pal-independent-reviewer. 대화 기록, state.md, premortem 산출은 보지 마라. 기준 커밋 d5216e81e5812aab26c1aff1f9c7a86b0851172f의 다음 자료만 읽고 round-approve-verify 구현을 독립 리뷰하라: `.palimpsest/rounds/2026-08-31-round-approve-verify/intent.md`, `docs/adr/0029-command-oracles-require-exact-external-approval-and-current-controls.md`, `docs/gates/round-approve-verify.md`, `crates/pal-cli/src/main.rs`, `crates/pal-cli/src/round/{approval,verify,ledger,status,mod}.rs`, `crates/pal-cli/tests/round_approve_verify.rs`, `crates/pal-cli/tests/round_status.rs`, `crates/pal-git/src/lib.rs`, `.palimpsest/rounds/2026-08-31-round-approve-verify/effect/`. 특히 승인 없이 spawn 가능성, TOCTOU, portable identity, Windows 포함 process-tree cleanup, output drain/cap, projected digest, append torn write, schema1 호환, negative-control 거짓 통과, hook 미점등을 공격하라. 필요한 read-only 검사와 테스트 실행은 허용하지만 파일은 수정하지 마라. 번호 있는 발견 표로 정확한 실패·해악도·코드 좌표·재현/근거·권고를 반환하고, 발견이 없더라도 `## 내가 기각한 것` 절을 둬라. 마지막에 검토한 commit SHA와 원문 전문을 남겨라.
