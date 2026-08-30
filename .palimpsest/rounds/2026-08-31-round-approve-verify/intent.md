# 회차 — round approve verify

## 원문

> AGENTS.md와 `/round` 절차를 따른다.
>
> 직전 회차 `2026-08-30-round-verification-status`의 계약을 이어받아 다음 수직 경로
> `round-approve-verify`를 완결한다. 읽기 전용 status reducer 위에 승인된 command oracle
> 실행과 append-only evidence 기록을 세우고, 실제 negative control 없이는 조건이 `met`으로
> 닫히지 않게 한다. #97을 착수 이슈로 정리하고 assign하며, RED·격리 효과·독립 리뷰·세 OS
> CI·ADR·게이트·그래프 결박·종료 보고까지 중간에서 멈추지 않는다.
>
> 사용자 승인 없이 실제 프로젝트 명령을 실행하거나 훅을 점등하지 않는다. 시험에서는 격리
> fixture와 명시적 승인 fixture만 사용한다. 기존 schema 1, oracle digest, status JSON과 exit
> 계약, `pal-intent`/`pal-cli` 소유 경계, hook unknown-input fail-open을 임의로 바꾸지 않는다.

## 목적 기여

[00-goals.md](../../../docs/plan/00-goals.md)의 하네스 목표와 P14에 기여한다. status 다음의
실행·증거 수직 경로는 실제 소비 가능한 완료 판정과 #97의 거짓 닫힘 차단에 가장 가깝고,
#85 Stop 경로가 소비할 선행 계약이다.

## 완수 조건

- [x] A1 사용자별 외부 approval 저장소는 저장소 밖의 private 위치에서 exact repo·round·condition·oracle·CWD·shell·PATH·timeout·output identity만 승인하고 malformed·symlink·권한 불일치를 spawn 전에 fail-closed한다 · 통과
- [x] A2 `pal round approve`는 명령을 실행하지 않고 승인을 기록하며 `pal round verify`는 exact 승인이 없는 oracle, PATH·CWD·shell·budget이 달라진 oracle을 실행하지 않는다 · 통과
- [x] A3 executor는 기본 timeout 120초와 stdout+stderr 1 MiB 상한을 지키고 cap·timeout 때 POSIX process group 또는 신뢰된 Windows `taskkill` 경로로 자식 tree를 종료한 뒤 bounded하게 회수한다 · 통과
- [x] A4 projected content-tree digest는 정렬된 tracked `(repo-relative path, blob identity)`에서 현재 round의 `verification.log` 하나만 제외하며 HEAD·절대 경로·줄바꿈 표현을 상태 입력으로 쓰지 않는다 · 통과
- [x] A5 실행 전후 oracle과 projected digest가 같을 때만 evidence를 append하고 중간 변경·승인 교체·append 실패에서는 결과를 폐기하며 명령을 자동 재실행하지 않는다 · 통과
- [x] A6 evidence writer는 단일 완전 JSON line만 append하고 동시 append·직전/도중 실패·trailing partial line을 성공으로 축약하지 않는다 · 통과
- [x] A7 schema 1 reader·oracle digest vector·condition/status JSON·exit code는 그대로이며 필요한 schema 2는 version별 unknown 거부, schema 1 read compatibility, 명시적 새 회차 migration 퇴로를 갖는다 · 통과
- [x] A8 positive oracle은 exit 0과 non-empty EXPECT의 combined stdout/stderr 일치와 실행 fault 없음이 함께 있어야 `met`이다 · 통과
- [x] A9 등록된 negative control은 자신이 실제 결함 탐지를 확인해 exit 0+EXPECT를 낸 현재 evidence가 있어야 하며, 미실행·stale·unmet control이 하나라도 있으면 연결된 주 조건은 `met`이 아니다 · 통과
- [x] A10 같은 oracle 재실행은 새 evidence를 append하고 과거 round·oracle·projected snapshot evidence는 현재 상태를 만족시키지 않는다 · 통과
- [x] A11 approve/verify CLI black-box와 기존 round status·Python/dashboard·hook/install 소비 경로는 같은 reducer·소유 경계를 유지하고 `xtask → pal-cli` 또는 `pal-cli` library target을 만들지 않는다 · 통과
- [x] A12 실제 격리 회차에서 승인 전 block, 승인, positive 실행, negative-control 실행, evidence append, status `met`의 순서를 테스트 아닌 출력으로 보존한다 · 통과
- [x] A13 새 suite와 `round_status`·`round_scripts_run`·`hook`·`install_hooks`·`cargo xtask check`·workspace all-targets가 전부 통과한다 · 통과
- [ ] A14 ubuntu·macOS·Windows와 양방향 상호운용 job이 마지막 pushed SHA에서 모두 성공한다
- [ ] A15 사전부검·독립 리뷰 발견이 구조화 원장에 전부 처분되고 ADR·게이트·그래프 결박·효과·종료 보고와 #97 종료가 실제 장치 근거를 가진다

**RED 관측**: 새 approve/verify black-box 공격은 구현 전 subcommand 부재로 실패해야 한다.

**음성 대조**: 최소 공격 모집단은 미승인, exit 0/no EXPECT, nonzero/marker, PATH·CWD·shell,
timeout/output cap, 실행 중 oracle/snapshot 변화, append 실패/partial line, rerun/stale,
Windows process tree, 미실행 negative-control false pass다. 각 검사에는 known-broken fixture가
실제로 거부되는 장면을 둔다.

### 인터뷰에서 잠근 결정

- 저장소: `PAL_APPROVAL_DIR` 명시값 또는 OS별 사용자 data directory 아래 `palimpsest/approvals`.
  canonical target은 repository 밖이어야 하며 shared artifact에는 위치·비밀을 싣지 않는다.
- 실행: schema oracle의 command는 OS 기본 shell 하나로만 실행한다. 선택된 shell과 전체 PATH,
  repo identity, 상대 CWD, timeout/output budget을 external approval identity에 결박한다.
- 예산: 기본 120초, combined stdout+stderr 1 MiB. CLI override는 승인 identity를 바꾸므로 재승인한다.
- 종료: Unix는 새 process group, Windows는 일치하는 `SystemRoot`·`WINDIR`·`SystemDrive`가
  가리키는 `System32/taskkill.exe /t /f`; 못 믿으면 held child handle fallback이며 cleanup 실패는 evidence 성공이 아니다.
- snapshot: `pal-git`의 tracked worktree projection을 재사용해 현재 round verification ledger만
  제외한다. untracked·ignored 파일은 이 projection의 모집단이 아니며 raw evidence output도 아니다.
- 폐기: spawn 직전과 종료 직후 approval/oracle/projected identity를 다시 읽고 하나라도 다르면
  evidence를 쓰지 않는다. append 실패 뒤 command 자동 재실행은 금지한다.

## 퇴로

- schema 2가 필요하면 schema 1을 바꾸지 않고 version별 reader를 병존시키며 새 approve/verify
  회차만 명시적으로 schema 2를 쓴다. 기존 회차를 소급 이주하지 않는다.
- Windows descendant cleanup을 원리상 같은 방법으로 잴 수 없으면 주석으로 면제하지 않고
  `xtask` 외침과 CI의 failing counterpart를 먼저 세운 뒤 승격한다.

## 범위 밖

- Stop 등록·차단과 진행 인지형 자기 상한(#85)
- finding·judgment를 포함한 전체 RoundState
- 과거 회차 verification 원장의 소급 이주
- ignored/untracked build output을 projected tracked content에 포함하는 일반 filesystem snapshot

## 개정

- 없음.

## 승격

- 없음.
