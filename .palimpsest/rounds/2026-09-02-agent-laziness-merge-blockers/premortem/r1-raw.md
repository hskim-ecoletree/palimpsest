### schema 1 상태와 Stop 종료 자격을 구분할 칸이 없다

- 모집단: 원의도
- 유효성: 참
- 해악도: 금지역
- 어디가 걸리나: `crates/pal-cli/src/round/status.rs:46` `StatusView`; `status.rs:190-226`; `crates/pal-cli/src/round/stop.rs:279-293`; `crates/pal-cli/tests/round_stop.rs:20-38,261-335`

schema 1 성공 evidence는 `verification=met`이 되고 유효한 `report.md`가 있으면 Stop을 통과한다. A1을 고치면서 `status`의 schema 1 읽기 호환까지 깨뜨리거나, 기존 Stop 정상 fixture를 유지해 우회가 남을 수 있다.

획득은 조회다. `pal query binding.touch aggregate`와 코드를 대조했으며, 그래프는 `aggregate`에 걸린 ADR 둘이 stale임도 냈다. 계획대상이다. schema version/currentness와 별도의 Stop 종료 자격 축을 두고, schema 1 status golden은 보존하되 Stop 정상 fixture는 schema 2로 바꿔야 한다. schema 1+완전한 report 음성 대조가 필요하다.

### “빈 종료문” 시험이 영문 claim을 재지 않는다

- 모집단: 자기장치
- 유효성: 참
- 해악도: 금지역
- 어디가 걸리나: `crates/pal-cli/src/round/stop.rs:402-448` `valid_terminal_document`; 특히 `433-437`; `crates/pal-cli/tests/round_stop.rs:312-325`

현재 음성 fixture는 `# report`만 써서 “필수 heading 누락”만 잰다. 필수 heading을 모두 쓰고 각 본문을 비우면 현재 검사는 통과하므로 B1을 구현하지 않고도 시험 이름은 초록일 수 있다.

획득은 코드 조회다. 계획자신에 대한 시나리오다. 다음 동급 heading 전까지의 절 본문을 파싱하고, 공백·주석뿐이면 거부해야 한다. 모든 heading이 있으나 한 절씩 빈 fixture와 완전한 정상 fixture를 report/folded 각각 둬야 한다.

### 종료 직전 전수 재실행이 승인된 실행 profile을 복원할 수 없다

- 모집단: 원의도
- 유효성: 참
- 해악도: 금지역
- 어디가 걸리나: `crates/pal-cli/src/main.rs:423-459` `RoundCommand::{Approve,Verify}`; `crates/pal-cli/src/round/approval.rs:30-35` `Record`; `crates/pal-cli/src/round/verify.rs:73-180`; `crates/pal-cli/src/round/stop.rs:279-293`

D1은 이미 met인 모든 결정론 조건을 다시 실행해야 하지만 현재 `verify`는 `--id`, shell, timeout, output-limit을 호출자가 준다. 외부 approval record에는 digest만 있어 Stop 또는 finalizer가 조건별 승인 profile을 복원할 수 없다. 기본값으로 재실행하면 승인이 달라지고, 재실행을 생략하면 stale 외부 상태가 통과한다.

획득은 CLI와 approval 저장 형식 조회다. 계획대상이다. 재실행 profile의 단일 진실을 구현 전에 잠가야 한다. 승인 record가 canonical profile을 품거나 finalizer가 정확한 승인 profile 집합을 받아야 하며, 서로 다른 timeout/shell을 가진 복수 조건과 마지막 조건 실패 시험이 필요하다.

### 결정론과 비결정론을 안전하게 가를 discriminator가 없다

- 모집단: 원의도
- 유효성: 참
- 해악도: 금지역
- 어디가 걸리나: `crates/pal-intent/src/round_condition.rs:63-78` `Condition`; `crates/pal-cli/src/round/status.rs:205-279`; `crates/pal-cli/tests/round_status.rs:210-228`

`Condition`에는 kind가 없고 reducer는 oracle이 없는 조건을 모두 unregistered로 본다. 체크된 `통과` verdict를 비결정론 evidence로 인정하면 아직 oracle을 등록하지 않은 결정론 조건도 사람 체크만으로 통과할 수 있고, 계속 무시하면 D2가 미구현으로 남는다.

획득은 `pal query binding.touch aggregate`와 Rust parser/reducer 조회다. 계획자신에 대한 시나리오다. 명시적 condition kind 또는 동등한 fail-closed 등록 규칙을 먼저 결정해야 한다. oracle 미등록 결정론+수동 통과는 거부하고, 명시적 dialectic 조건만 정반합 verdict로 같은 aggregate에 들어가는 대조가 필요하다.

### findings schema 2를 “열린 해악 0”으로 읽을 수 있다

- 모집단: 저장소
- 유효성: 참
- 해악도: 금지역
- 어디가 걸리나: `.claude/skills/round/bin/record.py:60-118`; `.claude/skills/round/bin/dashboard.py:278-309`; `crates/pal-cli/src/round/status.rs:145-286`

D3 구현이 schema 3의 `상태=열림`만 세면, 열림 축이 없는 schema 2 findings를 빈 열린 집합으로 오인해 complete가 된다. 기존 dashboard 계약은 이를 0이 아니라 “형식 이전”으로 낸다.

획득은 현재 스키마와 dashboard 판정 조회다. 계획대상이다. findings가 존재하되 열림 축이 없으면 Stop 종료 자격을 fail-closed해야 한다. schema 2/3, 열린 금지역·실패, 닫힌 finding, 미관만 열린 경우를 각각 시험해야 한다.

### shallow clone fixture와 repository identity가 함께 거짓 초록을 만들 수 있다

- 모집단: 자기장치
- 유효성: 참
- 해악도: 거짓신호
- 어디가 걸리나: `crates/pal-cli/src/round/approval.rs:94-106` `repository_root_identity`; `crates/pal-git/src/lib.rs:445-460` `first_parent_walk`; `crates/pal-cli/src/round/stop.rs:335-364`; 향후 shallow fixture

로컬 경로에 `git clone --depth 1 <path>`를 쓰면 Git이 depth를 무시한다. 격리 실측에서 path clone은 `is-shallow=false`, 581 commits였고 `file://` clone만 `true`, 1 commit이었다. 또한 missing parent에서 단순히 walk를 멈추면 depth-1 clone은 clone 당시 HEAD를, full clone은 최초 commit을 identity로 삼아 fetch/deepen 뒤 approval과 activation namespace가 달라질 수 있다.

획득은 격리 clone과 identity 코드 조회다. 계획자신에 대한 시나리오다. `file://` 또는 `--no-local`을 사용하고 `--is-shallow-repository=true`, commit count 1, parent object 부재를 전제 assertion으로 둬야 한다. depth-1→local commit→deepen/fetch 순서에서 enable/status/verify의 identity 계약도 잠가야 한다.

### 이슈 처분 게이트가 native dependency를 기계로 확인하지 않는다

- 모집단: 원의도
- 유효성: 참
- 해악도: 금지역
- 어디가 걸리나: `.palimpsest/rounds/2026-09-02-agent-laziness-merge-blockers/GATES.md:27-28`; `docs/agents/issue-tracker.md:20-49`; GitHub issues #95, #96, #101

현재 #101은 native `blocked_by=2`이고 #95와 #96은 각각 `blocking=1`이다. G5는 CHECK 없이 산문 evidence만 받아 dependency edge를 삭제하거나 #101을 먼저 닫고도 “처분”으로 적을 수 있다.

획득은 GitHub API 조회다. 계획자신에 대한 시나리오다. #95와 #96을 근거와 함께 먼저 닫거나 명시적으로 접고, edge를 삭제하지 않은 채 #101의 `blocked_by=0`을 확인한 뒤 #101을 닫아야 한다. API 출력과 issue URL을 외부 evidence로 남겨야 한다.

### #96 “흡수”가 이름만 같은 원장으로 축소될 수 있다

- 모집단: 원의도
- 유효성: 참
- 해악도: 금지역
- 어디가 걸리나: `docs/agent-laziness-executable-implementation-plan.md:345-346`; `crates/pal-cli/src/round/verify.rs:170-180`; GitHub issue #96

verification event ledger는 round condition evidence를 축약하지만 #96은 `P+장치` 실험의 진행 원장과 천장 아닌 효과 측정을 요구한다. 둘 다 append ledger라는 이유만으로 흡수 처리하면 원 요구의 측정 역할은 수행되지 않는다.

획득은 #96 본문과 현재 ledger event 계약 조회다. 계획대상이다. #96의 두 역할을 필드 단위로 대응시키고 실제 효과를 재측정해야 한다. 대응되지 않으면 흡수라 하지 말고, 우선순위 접힘 사유와 더 먼저인 항목을 명시해야 한다.

### 최종 SHA CI evidence를 GATES에 커밋하면 재귀가 다시 생긴다

- 모집단: 규약
- 유효성: 참
- 해악도: 금지역
- 어디가 걸리나: `.palimpsest/rounds/2026-09-02-agent-laziness-merge-blockers/GATES.md:33-34`; `.github/workflows/ci.yml:46-54`; GitHub issue #95

최종 SHA에 7 success가 붙은 뒤 `GATES.md` G7의 checkbox 또는 EVIDENCE를 갱신해 커밋하면 새 SHA가 생겨 방금 확인한 CI가 더는 최종 SHA의 것이 아니다. 갱신하지 않으면 tracked gate는 pending인 채 병합된다.

획득은 G7 tracked 좌표, CI concurrency, #95 계약 조회다. 계획자신에 대한 시나리오다. 최종 push 전에 tracked 산출물을 모두 확정하고, 이후에는 GitHub check-run이나 issue comment 같은 외부 terminal observation만 기록해야 한다. 최종 SHA 확인 뒤 저장소 write 없이 merge하고 origin/main 포함을 확인해야 한다.

### portable gate와 doctor 음성 대조가 CI에 연결되지 않았다

- 모집단: 자기장치
- 유효성: 참
- 해악도: 거짓신호
- 어디가 걸리나: `.palimpsest/rounds/2026-09-02-agent-laziness-merge-blockers/GATES.md:7-25`; `.github/workflows/ci.yml:121-133`; `.palimpsest/rounds/2026-08-30-agent-laziness-executable-plan/GATES.md:6-18`

새 G4 inline checker는 top-level 네 배열만 요구하고 `invariants`가 없거나 비어 있는 경우도 통과한다. intent가 요구한 고의 위반 JSON 음성 대조도 GATES에 없다. CI는 `cargo xtask check/test`만 실행해 focused gate와 doctor gate가 세 OS에서 같은 방법으로 재실행되는지 직접 증명하지 않는다. 옛 GATES에는 `/Users/...`, `/bin/sh`, 사라진 `gate-lint.mjs`도 남아 있다.

획득은 현재 doctor JSON shape 실행과 gate checker·CI steps 조회다. 계획자신에 대한 시나리오다. inline shell/Node를 repo-owned checker 또는 xtask로 옮기고, healthy JSON과 각 배열·invariant violation 변이 fixture를 둬야 한다. 그 단일 명령을 세 OS matrix에서 실행하고 옛 절대 경로·사라진 의존 evidence를 제거해 재발행해야 한다.

## 내가 기각한 것

### G4가 현재 doctor의 invariant 중첩을 잘못 읽어 성한 결과도 실패한다

- 모집단: 자기장치
- 유효성: 거짓
- 해악도: 미관
- 어디가 걸리나: `.palimpsest/rounds/2026-09-02-agent-laziness-merge-blockers/GATES.md:23`; `pal doctor --full --json`

`outcome.checked.violations` 경로가 실제 JSON과 다를 것이라 의심했으나 현재 `pal doctor --full --json` 표본은 정확히 그 shape이고 violations와 skipped가 모두 0이었다.

획득은 실제 JSON 실행 조회다. 현재 healthy output에는 영향이 없어 기각한다. 다만 `invariants` 부재를 통과시키는 문제는 위 portable gate 시나리오에 포함했다.

새 범주: 외부 terminal observation의 자기참조 재귀
