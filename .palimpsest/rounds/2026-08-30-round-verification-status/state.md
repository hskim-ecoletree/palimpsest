# 상태 — round verification status

## 지금 단계

구현과 로컬 전량 검증, 효과 관측, 독립 리뷰 R1 처분을 마쳤다. A9를 닫을 마지막 SHA의
ubuntu·macOS·Windows CI와 독립 리뷰 R2, ADR·게이트·종료 보고를 남겼다.

## 인터뷰

- 상한: 1라운드. 소유자의 실행 지시와 잠긴 계획으로 다섯 범주를 모두 열었다.
- 경계: 계획 §2의 읽기 전용 status 수직 경로만 구현한다.
- 의도: verification 조건 상태를 한 Rust reducer가 JSON과 사람 출력에 동일하게 낸다.
- 자율: 잠긴 계약 안의 구현·시험·이슈 갱신·ADR·회차 종료는 승인됨. 계약 축소·전환은 승격한다.
- 종료: A1~A10 판정, §5.2 전량 통과, 실제 소비 출력, 독립 리뷰 종료, ADR·게이트·보고, 마지막 SHA CI 성공.
- 재고: 계획 기준 SHA `2ea99a3ec15fb4f74c97d7541ad152127fdb2e5d`, #88, 기존 Python parser와 round script tests.

## 기준 drift

- 착수 HEAD는 계획 기준 SHA와 같은 `2ea99a3ec15fb4f74c97d7541ad152127fdb2e5d`다.
- 계획 §7의 제한 diff는 비었다. 계약·코드 좌표 정정은 필요 없다.
- 구현 중 `cargo xtask check`가 schema 크기 상수의 중복 소유를 잡았다. 값과 계약은 바꾸지
  않고 `pal-core::budget`으로 소유 좌표만 모았고 계획 §4.1에 근거를 기록했다.

## 이슈와 관계

- `./scripts/frontier.sh`에서 #88이 ready임을 확인하고 제목·본문·담당자·라벨을 현재 수직
  경로와 완수 조건으로 정리했다.
- native blocking은 #85 ← #88, #97 ← #88로 등록했다.
- #92는 `2bd9cd5`의 기계 추출과 이번 회차의 실제 raw↔record 소비, `cargo xtask check`
  23/23을 대조하고 종료했다.

## 그래프 우선 조회

- `Command`는 `crates/pal-cli/src/main.rs`, `decide`는 hook policy, `HOOK_EVENTS`는 install
  layout 좌표를 답했다. caller/callee는 0, `조건들`은 unknown이었다.
- cross-file resolution·effects·judgment가 현재 능력 부재라 답하지 못한 관련 경로에만
  `rg`를 사용했다. 병렬 `pal touch`의 redb write lock 충돌 뒤에는 조회를 순차 실행했다.

## RED와 구현

- `record.py` 수정 전에 Python parser 출력을 golden으로 보존했다. fixture는 code fence,
  nested indentation, duplicate ID, reversed transfer tag를 포함하고 기존 parser exit은 1이었다.
- `cargo test -p pal-cli --test round_status`의 최초 13개 시험은 `round` subcommand 부재로
  전부 실패(exit 101)했다. 구현 뒤 24개가 통과했다.
- 기대 상태 하나를 `met`에서 존재하지 않는 값으로 바꾼 음성 대조도 exit 101로 실패했고,
  복구 뒤 전량 통과했다. 전문은 `red-observation.md`가 진다.
- 구현 순서는 `pal-intent::round_condition` → `pal-cli::round::{ledger,status}` → CLI →
  Python wrapper였다. xtask는 condition read 한 자리만 `pal-intent`에 직접 의존한다.
- `pal-cli` library target과 `xtask → pal-cli` 의존은 만들지 않았다.

## 사전부검

- 상한: 1라운드. 완료. 원 반환문은 `premortem/r1-raw.md`에 보존했다.
- P1 `pal-intent/src/lib.rs` 누락 → 계획 §4.1 좌표 정정 + compile check.
- P2 round/intent/ledger 부재 우선순위 → RED fixture와 계획 §3.1 정정.
- P3 동일 oracle 재등록 → `stale` RED 추가.
- P4 후행 old digest → 최신 oracle 뒤 마지막 evidence가 다르면 `stale`로 계획·RED 고정.
- P5 terminal 충돌 자동 해소 → resolver가 ledger 후보의 terminal 충돌을 먼저 검사.
- P6 읽기 전용 → sentinel 미생성·intent/ledger 바이트 불변 black-box 추가.
- P7 크기·행·문자열·blank·partial·duplicate → ledger 경계 단위 시험 추가. i32/u64는 serde 타입 경계가 거부한다.
- P8 renderer drift → 공용 `StatusView`와 JSON/사람 전 state 검사를 둠.
- P9 세 OS → 동일 checked-in fixture를 `round_status` integration test로 두고 마지막 SHA 세 job을 확인.
- P10 PATH fallback → `PAL_BIN` 주입과 격리 PATH 복사본 왕복 시험 둘을 둠.
- P11 malformed intent → `invalid_schema`로 기존 code 안에서 고정.
- P12 xtask 모호성 → 원장 둘 대조의 condition 읽기만 `pal-intent`로 이전하고 gate parser는 유지.
- 첫 `cargo xtask check`가 schema 크기 상수 셋의 저장 위치를 stack §5.5 위반으로 잡았다.
  값은 바꾸지 않고 `pal-core::budget` 단일 위치로 옮겼으며 계획 §4.1 소유 좌표를 정정했다.

## 독립 리뷰 R1

- 모집단은 잠긴 intent와 구현·시험·fixture로 제한했고 state·계획·사전부검은 주지 않았다.
- R1-01 A4 대조 계약 부재 → `intent.md`에 schema 1 필드·상한·digest 바이트·상태·exit를
  직접 잠갔다. 외부 계획 링크 없이 대조 가능하다.
- R1-02 세 OS 증거 부재 → 정당한 미측정이다. 코드 시험으로 닫지 않고 마지막 SHA의 세
  named CI checks가 실제 성공할 때만 A9를 통과시킨다.
- R1-03 RED 역사 증거 부재 → 최초 실패와 음성 대조의 명령·종료 코드·핵심 출력을
  `red-observation.md`에 보존했다.
- 리뷰가 기각한 Invalid enum, stale 회복, golden 독립성, 공용 view 관련 의심은 구현 변경
  없이 근거를 보존했다. R2는 정정된 산출과 마지막 CI 증거만 새로 본다.

## 효과

- `effect/fixture`의 실제 진행 중 round를 빌드된 `pal`로 직접 소비했다.
- JSON과 사람 출력 모두 A1 `met`, A2 `pending`, aggregate `in_progress`, terminal `open`을
  냈다. 입력과 출력 전문은 `effect/`가 진다.

## 결정과 결박

- ADR-0028은 조건 parser=`pal-intent`, 원장 reducer=`pal-cli`, append-only current evidence,
  읽기 전용 공용 view, CI의 외부 terminal observation을 채택했다.
- `ConditionsReport`, `oracle_digest`, `read_round`에 ADR 결정을 각각 결박했고 `pal touch`가
  세 결박을 `live`로 다시 냈다.
- 그래프는 Rust L1 좌표와 files 반경은 담았지만 cross-file resolution(F07), unresolved
  refs(F08), effects(F13), judgment(F15)는 능력 부재다. 이 네 축을 추정으로 채우지 않는다.
- unlazy gate의 최초 승인 실행은 기본 CWD가 회차 디렉터리임을 드러냈다. G0~G6에
  `CWD: ../../..`를 명시해 저장소 루트로 잠갔다. 정정 뒤 G0~G4·G6은 통과했고 G5는
  A1~A8·A10의 판정 상자를 켜지 않아 한 번 빨개졌다. gate 표와 같은 아홉 조건만
  `통과`로 켰고 A9는 세 플랫폼 CI 전까지 미측정으로 남긴다.

## 로컬 검증

- `cargo test -p pal-cli --test round_status` — 24 통과
- `cargo test -p pal-cli --test round_scripts_run` — 15 통과
- `cargo test -p pal-cli --test hook` — 5 통과
- `cargo test -p pal-cli --test install_hooks` — 20 통과
- `cargo xtask check` — 23/23 통과
- `cargo test --workspace --all-targets` — exit 0, 전체 통과(기존 ignored benchmark 1개)

## 실패한 접근

- `pal touch` 네 건을 병렬 호출해 redb write lock이 셋에서 충돌했다. 그래프 조회는 순차 실행한다.
- 그래프는 `Command`·`decide`·`HOOK_EVENTS` 파일 좌표만 찾고 caller/callee는 0으로 냈으며,
  `조건들`은 unknown이었다. cross-file resolution·effects·judgment가 능력 부재라 관련 파일로만 `rg`를 내렸다.

## 남은 것

- 독립 리뷰 R2로 R1-01·R1-03 처분을 대조하고, A9는 마지막 SHA 세 OS CI로 닫는다.
- ADR·게이트·종료 보고와 findings 원장을 완결한다.
- 새 Rust·Python·문서 좌표를 그래프에 결박하고 #88을 닫는다.
- 최종 push 뒤 GitHub CI를 전량 확인한다.
