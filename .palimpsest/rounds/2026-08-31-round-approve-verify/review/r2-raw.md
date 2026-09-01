| # | 발견 | 모집단 | 유효성 | 해악도 | 좌표 | 조건 | 근거·권고 |
|---|---|---|---|---|---|---|---|
| 1 | Windows의 “trusted system root”는 일치하는 세 환경 문자열일 뿐이며, 승인 확인 전에 결박되지 않은 helper를 spawn한다 | 원의도 | 참 | 금지역 | `approval.rs:113-124,261-318,353-384`, `verify.rs:105-125` | A1, A2 | `SystemRoot`·`WINDIR`·`SystemDrive`를 모두 사용자 쓰기 가능한 `D:\\Windows` 등으로 맞추면 검사를 통과한다. `store_dir`는 approval record를 확인하기 전에 그 아래 `whoami.exe`와 `icacls.exe`를 실행하며, 둘은 approval identity에도 없다. taskkill과 shell bytes를 추가한 것만으로 이 spawn을 닫지 못했다. 환경 문자열 대신 OS API로 system directory를 구하고 binary identity를 검증하거나, ACL을 Win32 API로 직접 처리해 helper spawn을 없애라. |
| 2 | Windows private-store 검사는 DACL을 바꿀 뿐 owner를 확인·교체하지 않아 다른 owner가 권한을 되살릴 수 있다 | 원의도 | 참 | 금지역 | `approval.rs:220-230,249-318` | A1 | `/reset`, `/inheritance:r`, `/grant:r` 뒤 성공 코드만 보고 owner SID와 최종 security descriptor를 읽어 검증하지 않는다. Windows object owner는 DACL을 다시 바꿀 수 있는 `WRITE_DAC` 권한을 암묵적으로 가진다. 따라서 타 SID 소유의 요청 디렉터리를 현재 SID 전용이라고 판정할 수 있다. owner를 현재 SID로 고정하고 최종 owner·DACL·reparse/hard-link 조건을 OS API로 재검증하라. [Microsoft owner contract](https://learn.microsoft.com/en-us/windows/win32/secauthz/owner-of-a-new-object), [icacls semantics](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/icacls) |
| 3 | 부모가 먼저 끝난 detached descendant는 Windows에서 `taskkill /PID root /T`로 회수할 수 없다 | 규약 | 참 | 실패 | `verify.rs:301-335,416-448`, `round_approve_verify.rs:245-255,459-480` | A3 | root `cmd.exe`가 이미 종료됐지만 descendant가 pipe를 잡은 경우 deadline에서 죽은 root PID를 `taskkill /T`에 넘긴다. 명시 PID가 더는 존재하지 않으면 helper가 실패하고 descendant tree를 특정할 다른 handle·job이 없다. Unix process-group 방식과 달리 Windows new process group은 kill 가능한 lifetime container가 아니다. 테스트는 Windows에서도 실행되도록 쓰였지만 A14가 미측정이라 이 의미 차이를 잡았다는 근거가 없다. spawn 시 Windows Job Object에 root를 넣고 job handle로 전체 tree를 종료하라. [taskkill `/t` contract](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/taskkill) |
| 4 | Windows evidence replace는 atomic replace는 하지만 ADR이 요구한 directory sync·write-through를 구현하지 않는다 | 원의도 | 참 | 실패 | `verify.rs:450-488`, 특히 `484-487`의 `#[cfg(unix)]` | A6 | temp file과 persisted file은 `sync_all`하지만 parent directory sync는 Unix에서만 한다. 사용 중인 `tempfile` Windows 구현은 `MoveFileExW(MOVEFILE_REPLACE_EXISTING)`만 사용하고 `MOVEFILE_WRITE_THROUGH`를 쓰지 않는다. 따라서 Windows에서 “directory까지 sync한 뒤 성공”이라는 ADR 계약을 코드가 만족하지 않는다. Windows rename metadata의 durable completion을 보장하는 API/flag와 directory-handle flush를 구현하거나, 계약을 능력 부재로 명시해 승격을 막아라. [MoveFileEx flags](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-movefileexw) |
| 5 | append가 8 MiB reader 상한을 넘겨도 성공하여 방금 쓴 ledger를 즉시 unreadable로 만든다 | 자기장치 | 참 | 거짓신호 | `ledger.rs:99-106`, `verify.rs:450-488` | A6 | 실행 전 ledger가 상한 이하이면 load를 통과한다. `append_line`은 `current.len() + line.len() + 1`을 검사하지 않으므로 새 완전 행이 파일을 8 MiB 초과로 만든 뒤 `verified` 성공을 반환한다. 다음 status는 같은 파일을 schema 오류로 거부한다. replace 전에 최종 크기와 line 상한을 검사하고, 실패 시 기존 ledger를 그대로 유지하라. 경계값 black-box를 추가하라. |
| 6 | stdout/stderr read 오류가 execution fault가 아니라 정상 EOF처럼 처리된다 | 자기장치 | 참 | 거짓신호 | `verify.rs:355-385`, 특히 `366-368` | A8 | `reader.read`가 오류면 원인을 버리고 loop를 끝낸 뒤 `done=true`로 만든다. 다른 stream이나 이미 읽은 prefix에 EXPECT가 있고 exit 0이면 `fault=None`, `matched=true` evidence가 기록될 수 있다. drain 결과를 `Result`로 join해 read failure를 execution fault로 전파하라. |
| 7 | gate의 실행 모집단 수가 같은 문서 안에서 21개와 19개로 갈려 있다 | 회차기록 | 참 | 거짓신호 | `docs/gates/round-approve-verify.md:32-35,44-46` | A13 | 현재 근거는 black-box 21개가 실행됐다고 하나 A13 근거는 새 공격 모집단 19/19라고 한다. 실제 실행에서는 `round_approve_verify` 21개가 통과했다. 수동 숫자를 제거하고 test runner 산출에서 렌더링하거나 둘을 21/21로 일치시켜라. |

## 내가 기각한 것

| # | 발견 | 모집단 | 유효성 | 해악도 | 좌표 | 조건 | 근거·권고 |
|---|---|---|---|---|---|---|---|
| K1 | R1-01을 닫으려면 immutable transactional runner가 반드시 필요하다 | 원의도 | 거짓 | 거짓신호 | `intent.md:29,60-61`, `ADR-0029:30-33,64-68`, `verify.rs:121-175` | A5 | 잠긴 계약은 실행 전후 currentness를 재고 중간 변경 시 evidence를 폐기하는 검증 runner다. mutable build·배포 transaction이나 approval-file deletion을 즉시 취소로 정의하지 않았다. 따라서 immutable sandbox는 목표 밖 처분과 양립한다. 다만 exact system-helper spawn은 별개이며 발견 1이다. |
| K2 | R1-02 racy-stat 문제가 그대로다 | 저장소 | 거짓 | 거짓신호 | `pal-git/src/lib.rs:385-390,603-635`, `round_approve_verify.rs:575-592` | A4, A5 | security projection은 `scan_worktree_with_stat(false)`로 tracked bytes를 전부 다시 해시하며 same-size/mtime 회귀 시험도 통과했다. |
| K3 | R1-03 detached drain이 모든 플랫폼에서 그대로다 | 자기장치 | 거짓 | 거짓신호 | `verify.rs:278-392`, `round_approve_verify.rs:245-255,459-480` | A3 | drain 완료를 기다리고 thread를 join하도록 바뀌어 macOS 시험은 통과했다. 남은 Windows tree-containment 실패는 발견 3으로 분리했다. |
| K4 | R1-04 negative-control 역할 replay가 그대로다 | 원의도 | 거짓 | 거짓신호 | `ledger.rs:15-16,169-179,266-299`, `round_approve_verify.rs:381-410` | A9 | schema 2 전용 digest domain에 `negative_for`가 들어가며 과거 evidence 재분류가 stale이 되는 시험이 통과했다. |
| K5 | R1-07의 single-write torn line 구현이 그대로다 | 자기장치 | 거짓 | 거짓신호 | `verify.rs:450-488` | A6 | 기존 ledger와 새 행을 temp에 `write_all`·sync한 뒤 atomic replace한다. 남은 Windows durability와 크기 상한은 발견 4·5다. |
| K6 | schema 1 digest vector와 reader 호환이 깨졌다 | 원의도 | 거짓 | 거짓신호 | `ledger.rs:15-16,159-179,266-299,371-377,482-507` | A7 | schema 1은 기존 v1 domain과 잠긴 digest를 유지하고 schema 2만 별도 domain을 쓴다. vector 및 schema1 compatibility 시험이 통과했다. |
| K7 | schema 2에서 `negative_for` 누락·unknown-field 구분이 사라졌다 | 자기장치 | 거짓 | 거짓신호 | `ledger.rs:58-84,133-179,206-215` | A7, A9 | version별 금지와 projected digest 요구가 유지된다. |
| K8 | approve/verify가 hook을 점등했다 | 원의도 | 거짓 | 금지역 | `main.rs:549-571` | A11 | hook dispatch와 round dispatch는 계속 분리되어 있고 새 경로가 hook을 호출하지 않는다. |
| K9 | effect가 positive 하나만으로 aggregate `met`을 보인다 | 회차기록 | 거짓 | 거짓신호 | `effect/observation.md:9-25` | A12 | positive 뒤 `in_progress`, 실제 negative-control 뒤에만 `met`인 전이를 유지한다. |

## 미측정 목록

| # | 발견 | 모집단 | 유효성 | 해악도 | 좌표 | 조건 | 근거·권고 |
|---|---|---|---|---|---|---|---|
| M1 | Windows 전용 코드의 실제 compile | 규약 | 추정 | 실패 | `approval.rs:220-318,347-395`, `verify.rs:261-269,416-448` | A1, A3, A6, A14 | 로컬에는 `aarch64-apple-darwin`만 설치되어 있어 `x86_64-pc-windows-gnu` check가 `core/std target 없음`으로 실패했다. Windows CI 결과가 필요하다. |
| M2 | Windows ACL, dead-leader descendant, atomic replace의 실제 runtime | 규약 | 추정 | 실패 | 발견 1~4의 Windows 좌표 | A1, A3, A6, A14 | 소스 의미는 반증점을 보이지만 Windows 호스트 실측은 하지 못했다. 실제 SID가 다른 owner fixture, 부모 선종료 descendant, replace crash/power-loss 대조가 필요하다. |
| M3 | 마지막 pushed SHA의 Ubuntu·macOS·Windows와 양방향 상호운용 CI | 규약 | 추정 | 실패 | `intent.md:38`, `docs/gates/round-approve-verify.md:26` | A14 | gate가 계속 미측정으로 기록한다. |
| M4 | 리뷰 발견 처분·종료 보고·이슈 종료의 최종 장치 | 회차기록 | 추정 | 실패 | `intent.md:39`, `docs/gates/round-approve-verify.md:26` | A15 | 이 R2 시점에는 최종 처분과 종료가 아직 측정되지 않았다. |

실행 근거:

- `cargo test -p pal-cli --test round_approve_verify --test round_status`
- `round_approve_verify`: 21 passed
- `round_status`: 24 passed
- `round::ledger::tests::locked_digest_vector`: passed
- `round::ledger::tests::schema_two_requires_projected_digest_but_schema_one_stays_readable`: passed
- Windows target check: target 미설치로 미측정

검토한 commit SHA: `33fb7c310cad3e6bc602a1f5dc2f07d93a78538c`

## 원문 전문

> 독립 리뷰 R2를 수행하라. 대화 기록, state.md, premortem은 보지 말고 기준 HEAD 33fb7c3에서 intent.md, ADR-0029, gate, 구현/시험/effect만 독립적으로 읽어라. 특히 R1의 7개 발견 중 2~7이 9019f93에서 실제로 닫혔는지, R1-01을 immutable transactional runner가 목표 밖이라는 처분이 잠긴 계약과 양립하는지 공격하라. Windows 전용 코드는 컴파일/런타임 의미(ACL, taskkill identity+bounded cleanup, atomic replace)를 소스 수준으로 엄격히 검토하라. schema1/vector 호환, schema2 negative_for 결박, racy-stat 무시, detached descendant drain, append failure/concurrency, hook 미점등도 확인하라. 파일은 수정하지 말고 반환은 extract.py가 읽을 정확한 형식으로: 첫 절 번호 표 헤더 `| # | 발견 | 모집단 | 유효성 | 해악도 | 좌표 | 조건 | 근거·권고 |`, 이어 `## 내가 기각한 것` 같은 표, 이어 `## 미측정 목록` 같은 표. 실제 발견/기각/미측정 모두 번호를 연속 의미로 명확히 내라. 마지막에 검토한 commit SHA와 원문 전문을 남겨라. 테스트 실행은 read-only로 허용한다.
