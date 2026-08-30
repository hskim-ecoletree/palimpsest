# 에이전트의 거짓 완료를 끝내는 경로

> **성격**: 비교 보고서 + 구현 제안. **채택된 결정이 아니다.** 결정은 `docs/adr/`,
> 실행 상태는 GitHub 이슈가 정본이다.
>
> **기준 시점**: 2026-08-30
>
> **외부 대조**: `Leonxlnx/unlazy@473d4b80421c36d733042434cd4b938f81a19ef1`
> (README가 `2.1.0` 대상의 미태그 소스라고 밝힌 현재 `main` HEAD)
>
> **우리 기준**: `palimpsest@2ea99a3ec15fb4f74c97d7541ad152127fdb2e5d`
> (`round/agent-laziness`)

---

## 0. 요약 판정

palimpsest의 `/round`는 에이전트 게으름보다 넓은 문제를 푼다. 원문 의도 잠금, 목표 기여,
범위 축소의 승격, 독립 리뷰, 정반합, 발견 처분, 코드 좌표 결박은 unlazy에 없는 능력이다.
이 층은 유지할 가치가 있다.

그러나 **「완료 증거가 없는데 에이전트가 끝났다고 말하는 문제」만 떼어 보면 현재 unlazy가
더 완성돼 있다.** unlazy는 하나의 gate 원장에서 다음 경로를 기계로 잇는다.

```text
조건 등록 → 명령 승인 → 실행 → exit+EXPECT 판정 → 증거 기록
         → 재검증 → 전체 상태 축약 → 미완료 Stop 차단
```

palimpsest는 지금 다음 경로다.

```text
조건 등록 → 사람이 여러 검사를 실행 → 판정을 두 문서에 전사
         → 두 문서의 정합 검사 → 독립 리뷰가 실제 충족 여부를 사후 감사
```

따라서 지금 필요한 것은 규약 문장 추가가 아니다. **등록된 완수 조건을 직접 실행하고,
증거를 같은 상태 기계에 남기고, 그 상태 기계가 미완료 종료를 막는 짧은 수직 경로**다.

이 보고서의 권고는 다음과 같다.

1. 기존 [#88]의 「다섯 문장 약 13줄 기입」은 최신 unlazy 2.1 비교로 대체됐음을 이유로
   **그 형태로는 접는다.**
2. [#85]·[#95]·[#96]·[#97]의 직접 관련 부분을 **실행 가능한 완수 원장** 한 회차로 묶는다.
3. `pal` 안에 `round status/approve/verify` 상태 관리 경로를 만들고, `Stop` 훅이
   `round status`의 같은 축약값만 읽게 한다.
4. Depth Tree·모델 가격 라우팅·병렬 lease는 이 경로가 실사용 한 건에서 효과를 낸 뒤 판단한다.

[#85]: https://github.com/hskim-ecoletree/palimpsest/issues/85
[#88]: https://github.com/hskim-ecoletree/palimpsest/issues/88
[#95]: https://github.com/hskim-ecoletree/palimpsest/issues/95
[#96]: https://github.com/hskim-ecoletree/palimpsest/issues/96
[#97]: https://github.com/hskim-ecoletree/palimpsest/issues/97

---

# 제1부 — 브리핑

## 1. 무엇을 비교했나

비교 범위는 unlazy의 일반 기능 전부가 아니라 다음 실패에 대응하는 부분이다.

> 에이전트가 요청의 일부를 빠뜨리거나 검사를 하지 않고도 완료했다고 말한다. 반복 지시와
> 긴 규약은 그 선언을 안정적으로 막지 못한다.

비교한 unlazy는 앞 회차가 사용한 `ed9e8d2`의 `SKILL.md`만이 아니다. 현재 2.1 소스의
다음을 함께 봤다.

- [`SKILL.md`](https://github.com/Leonxlnx/unlazy/blob/473d4b80421c36d733042434cd4b938f81a19ef1/SKILL.md)
- [`README.md`](https://github.com/Leonxlnx/unlazy/blob/473d4b80421c36d733042434cd4b938f81a19ef1/README.md)
- [`scripts/gate-check.mjs`](https://github.com/Leonxlnx/unlazy/blob/473d4b80421c36d733042434cd4b938f81a19ef1/scripts/gate-check.mjs)
- [`scripts/stop-hook.mjs`](https://github.com/Leonxlnx/unlazy/blob/473d4b80421c36d733042434cd4b938f81a19ef1/scripts/stop-hook.mjs)
- gate·orchestration·parallel 참고 문서와 템플릿
- [`research/validation-protocol.md`](https://github.com/Leonxlnx/unlazy/blob/473d4b80421c36d733042434cd4b938f81a19ef1/research/validation-protocol.md)
- 현재 HEAD의 3 OS × Node 16/20/24 CI

우리 쪽은 다음 산출을 대조했다.

- `.claude/skills/round/SKILL.md`
- 두 agent-laziness 회차의 `intent.md`·`report.md`·게이트·발견 레코드
- `xtask`의 회차 레코드·원장 둘 대조·발견 닫힘 검사
- `crates/pal-cli/src/hook/policy.rs`
- 열린 이슈 #84~#100과 현재 프론티어
- 착수 뒤 커밋 이력과 현재 `cargo xtask check`, GitHub CI

## 2. 핵심 비교

| 축 | palimpsest | unlazy 2.1 | 판정 |
|---|---|---|---|
| 원 의도 보존 | 원문 불변, 목적 기여, 승격 | 요청 재독, contract inventory | **palimpsest가 강하다** |
| 조용한 축소 | 정정·확대·축소·전환을 구분하고 축소를 승인으로 올림 | 누락된 요구를 gate/handoff로 남김 | **palimpsest가 강하다** |
| 완수 조건 저장 | `intent.md` 조건 + 게이트 판정표 | `GATES.md` 한 원장 | unlazy가 단순하다 |
| 조건 실행 | 일반 실행자가 없다. 명령과 판정을 사람이 옮김 | `CHECK` 실행, exit 0 + `EXPECT` | **unlazy가 강하다** |
| 증거 | 근거·판정·레코드가 여러 파일에 분산 | shell·CWD·exit·match·출력 fingerprint를 gate에 기록 | **unlazy가 강하다** |
| 재검증 | 독립 리뷰가 다시 볼 수 있으나 통과 조건 전수 재실행 계약은 없음 | `--reverify`가 통과 조건도 다시 실행 | **unlazy가 강하다** |
| 조기 종료 차단 | 메인 `Stop`은 정책 밖 | 미충족 gate/dispatch면 `Stop` 차단 | **unlazy가 강하다** |
| 비결정론적 판정 | 독립 리뷰 + 정반합 + 해악도 | 수동 gate와 위험 비례 리뷰 | **palimpsest가 강하다** |
| 음성 대조 | 규약은 강하지만 실행 여부가 판정과 안 묶임 | positive control을 요구하나 lint는 권고 | 양쪽이 절반씩 갖는다 |
| 작업 분해 | 회차 교대와 역할 위임 | Depth Tree, leaf/branch, `OWNS`, dispatch wave | 대형 작업은 unlazy가 강하다 |
| 코드 지식 | 그래프 조회·결박·낡음 | 없음 | **palimpsest만의 층** |
| 보안·이식성 | Rust 단일 바이너리, 3 OS 계약 | 승인 저장소, shell/PATH 결박, 3 OS 테스트 | 둘 다 강점이 다르다 |

## 3. 우리 프로젝트가 이미 해결한 것

### 3.1 죽은 체크박스와 판정표의 불일치

첫 agent-laziness 회차는 최근 두 회차의 완수 조건 90개가 열린 채인데 게이트는 통과로
닫힌 상태를 찾았다. 지금 `cargo xtask check`의 「원장 둘 대조」가 다음을 검사한다.

- gate ID 집합 ↔ `intent.md` 조건 ID 집합의 양방향 같음
- 조건별 체크박스와 판정 태그
- 판정 표의 ID 목록과 검산 줄
- 최근 종료 회차가 검사 모집단에 들어왔는가

이것은 필요한 기반이다. unlazy의 단일 원장과 달리 두 원장을 유지하기로 한 설계에서
최소한의 정합을 만든다.

### 3.2 자기신고가 거짓이라는 실측

두 번째 회차는 여섯 세션 × 여섯 라운드를 돌려 다음을 관측했다.

- 완료 선언은 매 라운드 나왔다.
- 절반 넘는 완료 선언이 실제 산출과 갈렸다.
- 한 세션은 `1.000 → 0.357`로 무너진 뒤에도 계속 완료를 선언했다.
- 진행 장치는 스펙을 몰랐기 때문에 「미완성 표면 없음」을 냈다.
- 실제 누락은 독립 오라클과 사람이 잡았다.

이 결과는 **완료 자기신고를 판정 입력으로 쓰면 안 된다**는 존재 주장을 충분히 지지한다.
다만 아래 §6의 실험 결함 때문에 특정 문면이나 장치가 효과가 없다는 인과 판정에는 쓰지 않는다.

### 3.3 종료를 넓은 의미로 다루는 규약

`/round`는 unlazy보다 넓은 다음 문제를 이미 다룬다.

- 목적 밖 또는 지금 우선순위가 아닌 일을 이유와 함께 접는다.
- 어렵다·시간이 없다·컨텍스트가 부족하다는 이유로 조용히 축소하지 않는다.
- 결정론적 조건과 해석 조건을 분리한다.
- 해석 조건은 정반합으로 판정한다.
- 새 발견을 원 의도/저장소와 자기 장치/회차 기록으로 나눈다.
- 종료·막힘·접힘을 다른 terminal state로 둔다.
- 효과와 코드 좌표 결박을 종료 조건으로 둔다.

이 층을 unlazy로 교체할 이유는 없다. 아래 구현은 이 층 밑에 **결정론적 실행·증거 층**을
추가하는 제안이다.

## 4. 지금 실제로 비어 있는 것

### 4.1 조건을 실행하는 일반 경로

첫 회차 게이트의 착수 관측은 `완수 조건의 CHECK: 0`이었다. 현재 `xtask`는 조건 ID와
기록 형식, 전사 정합을 강하게 검사하지만, 각 조건이 말하는 결과를 내는 명령을 일반적으로
실행하지 않는다.

따라서 다음 둘은 다르다.

```text
현재 증명: "A1을 통과라고 적은 두 원장이 서로 같다."
필요한 증명: "A1의 등록된 검사가 지금 실행됐고 결과가 통과다."
```

### 4.2 현재 증거가 최신인지 판정하는 경로

과거에 실행한 명령과 현재의 명령·환경·산출이 달라도 판정 태그는 자동으로 무효가 되지 않는다.
unlazy의 `--reverify`와 oracle signature가 메우는 자리다.

### 4.3 메인 세션의 종료를 막는 경로

현재 훅이 판정하는 이벤트는 `SubagentStop` 하나다. 막는 조건도 서브에이전트의 마지막 말이
빈 경우뿐이다. `Stop`·`SessionStart`·`PreToolUse`는 명시적으로 통과한다.

즉 완료 조건이 비어 있거나 증거가 낡았어도 메인 에이전트의 종료는 막히지 않는다.

### 4.4 등록된 음성 대조와 실제 실행의 결박

두 번째 회차는 음성 대조 열둘을 등록하고 넷만 실행했지만, 실행하지 않은 대조가 걸린 조건
넷을 통과로 닫았다. 규약이 음성 대조를 요구하는 것과 **그 대조가 실행되지 않으면 통과할 수
없게 하는 것**은 다르다.

### 4.5 자기 저장소 하네스에 대한 그래프의 관측 범위

이 문제의 주요 산출은 Markdown 규약과 Python 하네스다. 현재 추출기는 둘을 코드 심볼로
읽지 못한다. 두 번째 회차도 이 능력 부재를 기록했다. 이번 조사에서도 읽기 전용 2층 인덱스가
없어 그래프 질의가 서지 않았고 텍스트 탐색으로 내려갔다.

이것은 실행 가능한 완수 원장의 선행 조건은 아니다. 다만 ADR-0025의 결합 근거를 실제로
보이려면 후속 실사용에서 `pal touch/query`가 적어도 새 Rust 실행 경로의 영향 범위를 답해야 한다.

## 5. unlazy에서 가져올 것과 가져오지 않을 것

### 5.1 가져올 의미

| 의미 | 가져오는 이유 | 그대로 복사하지 않을 부분 |
|---|---|---|
| 작업 전 gate 등록 | 완료 정의를 사후에 낮추는 것을 막음 | 별도 `GATES.md` 문면 형식 |
| exit 0 + 성공 전용 `EXPECT` | rc만 또는 문자열만 맞는 거짓 통과를 막음 | Node 실행기 |
| 정확한 oracle 승인 | 저장소가 임의 shell 명령을 실행시키는 경계를 분리 | `~/.unlazy` 경로와 파일 형식 |
| evidence fingerprint | 과거 실행과 현재 주장을 분리 | raw 성공 출력 비저장 정책은 ADR에서 결정 |
| `--reverify` | 오래된 통과를 종료 근거로 쓰지 않음 | unlazy CLI 문법 |
| aggregate completion | gate와 dispatch 일부만 보고 완료하는 것을 막음 | Depth Tree 전체 |
| `Stop` 차단 | 규약을 실제 경계로 내림 | Claude 설정 설치 방식은 기존 `pal install` 사용 |
| 의미적 진행 상한 | 무한 Stop 루프를 막음 | 여섯 번이라는 상수는 실측 후 결정 |
| abandonment는 성공 아님 | 접힘/막힘을 완료로 승격하지 않음 | `ABANDON:` 토큰 |

### 5.2 지금 가져오지 않을 것

| 항목 | 판정 | 이유 |
|---|---|---|
| Depth Tree 전체 | **이번 범위 밖** | 핵심은 단일 회차의 거짓 완료 차단이다. 리프 원장까지 만들면 새 선언면이 급증한다 |
| `tree N` | **이번 범위 밖** | 깊이가 품질을 보장한다는 증거가 없고 현재 회차 교대와도 계약을 맞춰야 한다 |
| 모델 가격 라우팅 | **이번 범위 밖** | 품질 하한과 독립적인 비용 최적화다 |
| 병렬 `OWNS` lease | **이번 범위 밖** | 동시 쓰기 문제가 실제 프론티어가 된 뒤 도입한다 |
| Node 런타임 | **기각 제안** | `00-stack.md`의 단일 Rust 바이너리와 3 OS 계약을 깨뜨린다 |
| 「개선 없는 패스 = 완료」 | **기각 유지** | 우리 네 실측 계열에서 dry가 오지 않았고 열린 생성 과정의 종료선이 되지 못했다 |
| 임의 shell을 Stop에서 실행 | **금지** | Stop은 상태만 읽고 검사를 실행하지 않는다. 명령 실행 동의와 종료 훅을 분리한다 |

## 6. 왜 여러 커밋 동안 닫히지 않았나

### 6.1 첫 회차가 「구현하지 않음」을 통과 조건으로 등록했다

첫 회차의 `I4`는 다음을 요구했다.

> 규약 문면을 기입하지 않았다 — 기입은 이슈로.

따라서 그 회차가 성공해도 게으름 방지 장치는 설치되지 않는다. 실제 구현은 #88, Stop은 #85로
분할됐다. **완수 조건이 문제 해결이 아니라 조사 종료를 인증했다.**

### 6.2 두 번째 회차도 구현보다 효과 측정을 먼저 했다

두 번째 회차는 고른 문면과 진행 장치의 효과를 실험했지만 다음 결함이 있었다.

1. 대조군이 처음부터 천장이어서 처치가 개선을 만들 수 없었다.
2. 다섯 축 중 독립적인 축은 둘뿐이었다.
3. 소유자가 고른 진행 원장이 승격 없이 실험에서 빠졌다.
4. 등록한 음성 대조 열둘 중 넷만 실행됐다.
5. 과제 둘뿐이라 효과 크기나 일반화를 말할 수 없었다.

따라서 두 팔의 합격선 미달은 「문면/장치가 가치 없다」는 판정이 아니다. 회차 자신도
그렇게 기록했고 #88을 열린 채 남겼다.

### 6.3 자체 검사가 실제 결과보다 기록 정합을 본다

`xtask`는 기록의 구조적 정합을 강하게 만든 대신 arbitrary project outcome을 실행하지 않는다.
그래서 조건을 전부 통과로 전사한 뒤 독립 리뷰가 실제 미수행 넷을 찾을 수 있었다.

### 6.4 강제 경계가 작업 완료 상태와 연결되지 않았다

Stop이 먹는다는 관측은 했지만 정책을 #85로 분리했다. 지금 훅은 빈 서브에이전트 반환만 막고
메인 완료는 막지 않는다. 관측과 점등 사이가 열린 채다.

### 6.5 분할이 프론티어 소비 없이 누적됐다

첫 회차가 만든 별도 이슈 #90·#92·#93·#94·#95는 다음 두 회차에서 열리지 않았다. 두 번째
회차 보고는 `/round`의 음성 대조대로 이것을 「분할이 아니라 버림」이라고 기록했다.

### 6.6 메타 장치가 원 문제보다 커졌다

착수 `e45e822` 뒤 현재 HEAD까지 관측값은 다음과 같다.

- 전체 커밋 104개
- 제목에 `round`가 있는 커밋 89개
- 관련 회차·게이트·하네스 범위 403파일, 66,927행 추가
- 두 게으름 회차와 관련 규약·게이트를 건드린 커밋 74개

커밋의 다수는 실험 원자료, 독립 리뷰, 발견 레코드, 그 레코드의 검사와 정정이다. 이 산출은
거짓 완료의 모양을 풍부하게 밝혔지만, 사용자가 체감하는 마지막 경로는 연결하지 않았다.

### 6.7 기존 unlazy 비교의 모집단이 낡았다

#88은 `ed9e8d2`의 `SKILL.md`만 전수했고 `references/`·`templates/`·`scripts/`는 보지 않았다.
현재 unlazy 2.1의 중요한 변화는 바로 제외된 실행·증거·보안·Stop 코드에 있다. 그러므로
「고른 다섯 문장 약 13줄」은 현재 upstream 대비 답이 아니다.

## 7. 현재 상태의 정직한 판정

2026-08-30 현재 다음은 참이다.

- `cargo xtask check`: 23/23 통과
- 현재 HEAD GitHub CI: success
- 워킹트리: clean
- 열린 이슈: 37개, 착수 가능 37개
- `xtask` 출력 안에는 종료 보고 유예와 A축 감사 대기가 여전히 명시된다.

따라서 **저장소의 현재 검사 계약은 통과하지만, 에이전트 게으름 문제는 닫히지 않았다.**
통과한 것은 현재 검사 계약이고, 빠진 것은 결정론적 완수 조건의 실행·증거·Stop 결박이다.

---

# 제2부 — 상세 구현 계획

## 8. 구현 목표와 비목표

### 8.1 목표

한 회차의 결정론적 완수 조건에 대해 다음 문장이 참이게 한다.

> 승인된 검사가 현재 스냅숏에서 실행돼 성공했고, 그 증거가 현재 oracle과 일치하며,
> 모든 필수 조건이 그 상태가 아니면 메인 에이전트가 완료로 종료할 수 없다.

### 8.2 비목표

- 모델의 심리적 게으름을 일반적으로 제거했다고 주장하지 않는다.
- 모든 조건을 명령으로 바꾸지 않는다.
- 기존 독립 리뷰·정반합을 없애지 않는다.
- 과거 회차 전량을 새 스키마로 소급 이주하지 않는다.
- Depth Tree·병렬 실행·모델 라우팅을 한꺼번에 만들지 않는다.
- CI 결과를 쓰는 순간 참인 체크박스로 커밋하려 하지 않는다.
- 새 대화 표면을 만들지 않는다. 기존 네 표면 중 `pal` CLI와 훅만 확장한다.

## 9. 제안 구조

### 9.1 상태 흐름

```text
intent.md의 조건 ID와 문장
          │
          │ ID 집합 같음
          ▼
verification.jsonl의 등록 oracle ── pal round approve ── 사용자별 승인 저장소
          │
          ├── pal round verify ── 실행 결과/evidence event 추가
          │
          └── pal round status ── 조건별 상태 + aggregate terminal state
                                      │
                                      ├── /round 계기판·종료 보고
                                      └── pal hook Stop → block/pass
```

### 9.2 정본 분리

| 사실 | 정본 | 중복 금지 |
|---|---|---|
| 무엇을 달성해야 하나 | `intent.md` 조건 ID + 문장 | verification 원장에 문장을 복제하지 않음 |
| 어떤 검사로 재나 | `verification.jsonl`의 oracle 등록 행 | gate 문서에 실행 명령을 다시 쓰지 않음 |
| 어떤 결과가 났나 | 같은 파일의 evidence event | 체크박스를 사람이 직접 켜지 않음 |
| 해석 조건의 판정 | 정반합의 구조화된 판정 event | shell 통과로 위장하지 않음 |
| 전체 상태 | `pal round status`의 계산값 | 산문에 캐시하지 않음 |
| 최종 CI | GitHub의 마지막 SHA run | 커밋 안 체크박스로 참을 선기록하지 않음 |

파일 이름 `verification.jsonl`은 제안명이다. ADR에서 확정한다. 중요한 것은 **append-only event와
계산된 현재 상태**이지 이름이 아니다.

### 9.3 최소 스키마 제안

```json
{"kind":"schema","version":1,"round":"2026-..."}
{"kind":"oracle","id":"A1","mode":"command","check":"cargo test ...","expect":{"literal":"A1_OK"},"cwd":".","negative_control":"A1-neg"}
{"kind":"oracle","id":"A1-neg","mode":"command","check":"... known broken fixture ...","expect":{"literal":"A1_NEGATIVE_CONTROL_OK"},"cwd":"."}
{"kind":"evidence","id":"A1","oracle_digest":"...","snapshot":"git:...","platform":"...","shell":"...","exit":0,"matched":true,"output_digest":"...","output_bytes":123,"at":"..."}
{"kind":"judgment","id":"B1","mode":"dialectic","verdict":"통과","evidence_refs":["..."]}
```

요구 불변식:

- ID는 `intent.md` 조건 ID에 반드시 존재한다.
- 결정론적 조건은 oracle이 정확히 하나다.
- 음성 대조가 등록된 조건은 대조 evidence도 현재여야 통과한다.
- `exit == 0 && matched == true`만 성공이다.
- oracle의 어느 필드든 바뀌면 digest가 바뀌고 기존 evidence는 stale이다.
- evidence는 어느 snapshot에서 났는지 진다.
- `judgment`는 정반합 반환 형식과 해악 게이트를 통과해야 한다.
- 접힘·막힘은 terminal이지만 complete가 아니다.
- 알 수 없는 스키마·필드·상태는 fail closed다.

## 10. CLI 계약 제안

### 10.1 `pal round status`

```text
pal round status --round <slug> [--json]
```

하는 일:

- `intent.md` 조건 집합을 읽는다.
- verification 원장을 읽고 스키마·ID·oracle·evidence를 검증한다.
- 조건별 `unregistered/pending/met/stale/unmeasured/contradicted`를 낸다.
- `findings.jsonl`의 열린 금지역·실패를 함께 축약한다.
- 회차 상태를 `in_progress/complete/blocked/folded` 중 하나로 계산한다.
- 명령은 실행하지 않고 파일도 고치지 않는다.

### 10.2 `pal round approve`

```text
pal round approve --round <slug> --id <ID>
```

하는 일:

- 실행할 정확한 command, EXPECT, CWD, shell, PATH와 제한을 화면에 낸다.
- 명시 호출에서만 승인 레코드를 쓴다.
- 승인 레코드는 저장소 밖 사용자 전용 데이터 디렉터리에 둔다.
- 승인 서명은 절대 원장 경로·round·ID·oracle digest·플랫폼·환경을 포함한다.

하지 않는 일:

- 승인과 실행을 묵시적으로 합치지 않는다.
- 호출된 스크립트의 안전을 보증한다고 주장하지 않는다.
- Stop 훅에서 승인을 만들지 않는다.

### 10.3 `pal round verify`

```text
pal round verify --round <slug> [--id <ID>] [--reverify]
```

하는 일:

- 승인된 결정론적 oracle만 실행한다.
- timeout과 출력 상한을 적용한다.
- exit와 EXPECT를 별개로 판정한다.
- 성공 출력은 digest와 byte count를 기록하고 실패 출력은 bounded diagnostic으로 낸다.
- 실행 중 oracle이 바뀌면 결과를 기록하지 않는다.
- `--reverify`면 met 조건도 다시 실행한다.
- 결과 event를 원자적으로 append한다.

### 10.4 `pal hook Stop`

하는 일은 하나다.

```text
pal round status --json의 aggregate state를 읽어 block/pass를 반환한다.
```

제약:

- Stop은 검사를 실행하지 않는다.
- `complete`만 완료 통과다.
- `blocked/folded`는 성공으로 바꾸지 않고 handoff를 허용한다.
- `stop_hook_active` 반복 입력을 안전하게 처리한다.
- 같은 semantic state가 반복되면 진행 없음 횟수를 올린다.
- 상한에 닿아 release하더라도 상태를 complete로 바꾸지 않고 blocker ID를 보고한다.
- 훅 메시지에 저장소가 준 자유문을 그대로 반사하지 않는다.

## 11. 코드 좌표별 변경 계획

아래는 구현 위치 제안이다. ADR에서 경계가 바뀌면 파일명도 함께 바뀔 수 있다.

### 11.1 도메인 타입과 원장 축약

**후보 위치**

- `crates/pal-cli/src/round/mod.rs`
- `crates/pal-cli/src/round/ledger.rs`
- `crates/pal-cli/src/round/status.rs`

**이유**

`xtask`는 `pal-core`에 `gate`·`completion` 같은 거버넌스 어휘가 들어가는 것을 금지한다.
이 기능은 그래프 도메인 타입이 아니라 하네스의 상태 관리이므로 우선 `pal-cli` 내부가 맞다.
새 crate는 수직 경로가 선 뒤 컴파일 경계가 필요하다는 실측이 있을 때만 분리한다.

**산출**

- `ConditionId`, `Oracle`, `Expectation`, `Evidence`, `ConditionState`, `RoundState`
- append-only JSONL parser/reducer
- unknown version/duplicate ID/impossible transition 거부
- deterministic serialization과 digest

### 11.2 CLI 배선

**후보 위치**

- `crates/pal-cli/src/main.rs` 또는 현재 command 정의 모듈
- `crates/pal-cli/tests/round_*.rs`

**산출**

- `pal round status/approve/verify`
- JSON 출력과 사람 출력의 같은 상태값
- repository root/round path 해소 한 자리
- Windows/macOS/Linux에서 같은 의미

### 11.3 기존 Markdown 조건 파서와의 경계

**현재 위치**

- `.claude/skills/round/bin/record.py`
- `xtask/src/main.rs`의 `check_ledger_pair`

**1차 구현**

- `record.py conditions`가 내는 조건 ID 집합을 Rust status가 소비한다.
- 새 verification 원장은 조건 문장을 복제하지 않고 ID만 가리킨다.
- `xtask`는 `pal round status --json`을 호출해 새 원장의 구조를 다시 구현하지 않는다.

**후속 이동 조건**

Python과 Rust가 같은 조건 문법을 두 벌로 해석하게 되는 순간, 조건 파서를 Rust의 한 자리로
내리고 `record.py`·dashboard·`xtask`가 그 출력을 소비하게 한다. 처음부터 이 이주까지 묶으면
범위가 커지므로, 1차 회차에서는 **두 번째 파서를 만들지 않는 것**을 합격선으로 둔다.

### 11.4 훅 정책과 등록

**현재 위치**

- `crates/pal-cli/src/hook/policy.rs`
- `crates/pal-cli/src/install/layout.rs`
- hook 설치/왕복 테스트

**변경**

- `EVENTS`에 `Stop`을 코드에서 추가한다.
- 등록 목록은 계속 `EVENTS`에서 렌더링한다. 손 목록을 둘로 만들지 않는다.
- `decide("Stop", payload)`가 현재 round를 해소하고 status reducer를 읽는다.
- 기존 `SubagentStop` 빈 반환 정책은 독립적으로 유지한다.
- Stop 반복 guard는 semantic state digest를 사용한다.

### 11.5 설치 자산과 `/round`

**현재 위치**

- `.claude/skills/round/SKILL.md`
- `crates/pal-cli/assets/`와 install layout/build 입력
- `crates/pal-cli/tests/round_scripts_run.rs`

**변경**

- §3의 결정론적 조건에 oracle 등록 규칙을 짧게 추가한다.
- §7 검증은 `pal round verify --reverify` 결과를 판정의 입력으로 사용한다.
- §11 종료는 aggregate `complete`를 요구한다.
- 승인·실행·Stop의 보안 경계를 반복 서술하지 않고 CLI `--help`를 정본으로 가리킨다.
- 설치본과 저장소본이 같은 명령·스키마를 쓰는 왕복 시험을 추가한다.

### 11.6 CI와 `xtask`

**현재 위치**

- `xtask/src/main.rs`
- `.github/workflows/ci.yml`

**변경**

- 새 원장 parser를 `xtask`에 복제하지 않는다.
- `pal round status --json`의 모든 종료 회차가 complete/folded 중 하나인지 검사한다.
- checked-but-no-evidence, stale evidence, 미실행 음성 대조를 실패시킨다.
- CI 자체의 결론은 커밋된 gate 값이 아니라 외부 마지막 SHA 상태로 남긴다.
- #95의 재귀를 없애기 위해 `K9`류 조건을 일반 condition 집합에서 제거하고 종료 절차로 둔다.

## 12. 구현 순서

### 단계 0 — 결정 잠금

**산출**: ADR 한 편.

결정할 것:

1. verification event schema와 정본 위치
2. 성공 출력 원문 보존 여부
3. snapshot 결박 단위: HEAD, worktree digest, 둘 다
4. 승인 저장소의 OS별 위치와 권한
5. shell 허용 범위와 기본 timeout/output 상한
6. Stop no-progress 상한
7. CI를 condition 밖 terminal observation으로 두는 계약

**종료 조건**: 미결 선택이 구현 코드의 임의 상수로 내려가지 않는다.

### 단계 1 — 읽기 전용 status reducer

**산출**: `pal round status`.

먼저 만드는 이유는 실행기와 훅이 같은 판정자를 공유해야 하기 때문이다.

필수 시험:

- 원장 없음 → `unregistered`, complete 아님
- 조건 ID 누락/초과 → 형식 오류
- evidence 없음 → pending
- oracle digest 불일치 → stale
- exit 0이지만 EXPECT 불일치 → unmet
- open 금지역/실패 finding → blocked
- folded marker → folded, complete 아님
- 알 수 없는 schema → fail closed

### 단계 2 — 승인과 실행

**산출**: `pal round approve`, `pal round verify`.

필수 시험:

- 승인 없는 명령은 출력만 하고 실행하지 않는다.
- command/EXPECT/CWD/shell/PATH 변경은 재승인을 요구한다.
- timeout·출력 상한이 실제 자식 프로세스에 걸린다.
- exit 0과 EXPECT가 모두 맞아야 evidence가 생긴다.
- 실행 중 oracle 변경 결과는 stale로 버린다.
- `--reverify` 실패는 기존 met를 stale/unmet로 내린다.
- LF/CRLF와 세 OS에서 같은 상태를 낸다.

### 단계 3 — 음성 대조와 비결정론 판정

**산출**: positive control 결박과 dialectic judgment event.

필수 시험:

- 음성 대조가 없는 조건은 정책에 따라 등록 거부 또는 명시 `대조불가`다.
- 음성 대조를 실행하지 않으면 주 조건이 성공해도 aggregate complete가 아니다.
- known-broken fixture에서 검사기가 실패하지 않으면 주 조건 evidence를 인정하지 않는다.
- 결정론 조건에 수동 판정을 넣어 우회할 수 없다.
- 비결정론 조건은 정반합 evidence reference가 없으면 미측정이다.

### 단계 4 — Stop 연결

**산출**: 메인 Stop 차단.

필수 시험:

- pending condition → block
- stale evidence → block
- unrun negative control → block
- 모든 조건 met + 해악 gate clear → pass
- blocked/folded → 성공 아님을 보존한 handoff
- 반복 훅이 무한 루프를 만들지 않음
- 주석·reflow·evidence 재작성만으로 진행 상한이 리셋되지 않음
- 조건 상태 변화는 진행으로 인식됨

### 단계 5 — 기존 검사 통합과 중복 제거

**산출**: `xtask`와 dashboard가 status reducer를 소비.

필수 시험:

- 같은 enum·schema·ID 규칙이 Rust/Python/xtask에 중복되지 않는다.
- 표 헤더 한 글자로 과거 검사 모집단에서 빠지는 #90 재현이 빨개진다.
- 수만 맞고 내용이 다른 #92 재현이 빨개진다.
- 스키마 선언과 소비자가 갈리는 #94 재현이 빨개진다.
- 현재 23개 검사와 전 플랫폼 CI가 계속 통과한다.

### 단계 6 — 실제 효과

**산출**: 프론티어의 작은 실제 이슈 하나를 새 경로로 완주한 기록.

선정 기준:

- 사용자 소비 지점에 가깝다.
- 결정론적 조건 2개 이상과 음성 대조 1개 이상이 있다.
- 시작 상태에서 실제 RED를 만들 수 있다.
- 한 회차에 닫을 수 있는 S/M 크기다.

반드시 볼 장면:

1. 미충족 상태에서 Stop이 실제로 block한다.
2. 검사를 실행하지 않고 체크박스만 켜도 block한다.
3. 검사나 EXPECT를 바꾸면 과거 evidence가 stale이 된다.
4. 구현 뒤 `--reverify`가 전부 새 evidence를 만든다.
5. Stop이 pass하고 종료 보고에는 잔여 목록이 없다.

이 장면까지 있어야 「에이전트 게으름 문제를 닫았다」고 말한다. 단위 시험과 CI만으로는 닫지 않는다.

## 13. 사전 등록 완수 조건

실제 회차를 열 때 아래를 ID와 함께 `intent.md`에 옮긴다. 문면은 제안이며 승인 때 잠근다.

### A — 단일 상태 기계

- [ ] 조건·oracle·evidence·terminal state의 정본이 각각 한 자리다.
- [ ] `pal round status --json` 하나가 조건별 상태와 aggregate 상태를 낸다.
- [ ] 다른 소비자는 상태 enum과 schema를 다시 구현하지 않는다.
- [ ] 알 수 없는 schema와 불가능한 transition은 fail closed다.

### B — 실행과 증거

- [ ] 승인하지 않은 `CHECK`는 어떤 경로에서도 실행되지 않는다.
- [ ] exit 0 + EXPECT match 둘이 함께 있어야 met다.
- [ ] evidence가 oracle digest·snapshot·platform·shell·CWD·출력 fingerprint를 진다.
- [ ] oracle 변경과 `--reverify` 실패가 과거 met를 무효화한다.
- [ ] 음성 대조 미실행 상태에서는 주 조건이 met가 아니다.

### C — 종료 강제

- [ ] 메인 `Stop`이 aggregate incomplete를 실제로 block한다.
- [ ] Stop은 명령을 실행하거나 승인을 만들지 않는다.
- [ ] 반복 guard가 무한 차단을 막되 incomplete를 complete로 바꾸지 않는다.
- [ ] blocked/folded는 성공이 아닌 terminal handoff로 남는다.

### D — 기존 규약 통합

- [ ] 결정론 조건은 기계 실행이 판정한다.
- [ ] 비결정론 조건은 기존 정반합이 판정한다.
- [ ] 열린 금지역·실패 finding이 있으면 complete가 아니다.
- [ ] 종료 보고·게이트·dashboard가 계산값을 손으로 복제하지 않는다.
- [ ] 최종 CI는 #95의 재귀 없이 마지막 SHA의 외부 상태로 판정한다.

### E — 회귀와 플랫폼

- [ ] #90·#92·#94·#95·#97의 실패 모양이 회귀 시험으로 선다.
- [ ] ubuntu·macOS·Windows에서 같은 방법과 같은 상태를 낸다.
- [ ] 설치·업데이트·제거 왕복이 사용자 승인과 evidence를 손상시키지 않는다.
- [ ] 현재 `cargo xtask check`와 전체 CI가 초록이다.

### F — 효과

- [ ] 실제 이슈에서 미완료 Stop 차단을 관측했다.
- [ ] 체크박스만 켠 거짓 완료를 차단했다.
- [ ] stale evidence 차단을 관측했다.
- [ ] 수정 후 전수 재검증과 Stop 통과를 관측했다.
- [ ] 그래프가 새 Rust 경로의 영향 범위를 답했거나 정확한 능력 부재가 기록됐다.

## 14. 음성 대조

| 보호하려는 것 | 고장 나면 이렇게 드러나야 한다 |
|---|---|
| 승인 경계 | 승인 파일 없이 marker를 만드는 fixture가 실행되지 않는다 |
| exit+EXPECT | exit 0/no marker와 exit 1/marker가 둘 다 실패한다 |
| stale 판정 | CHECK 한 글자 변경 뒤 기존 evidence가 met로 남지 않는다 |
| snapshot 결박 | 코드 변경 뒤 관련 evidence가 정책대로 stale이 된다 |
| 음성 대조 | known-broken fixture를 검사기가 놓치면 주 조건도 met가 아니다 |
| ID 집합 | intent 조건 하나를 삭제/추가하면 verification 원장과 불일치한다 |
| Stop | pending 하나를 심으면 실제 Stop 응답이 block이다 |
| 진행 guard | 문서 reflow만 반복해도 무한 block이 풀리되 complete는 아니다 |
| 훅 등록 | `EVENTS`와 설치 settings 중 하나만 바꾸는 길이 없다 |
| 플랫폼 | POSIX 전용 명령 fixture가 Windows에서 조용히 통과하지 않는다 |
| CI 재귀 | 마지막 커밋 전에 CI 통과라고 전사하는 필드가 존재하지 않는다 |

## 15. 보안과 실패 경계

### 15.1 `CHECK`는 코드다

- 저장소에서 읽은 명령을 자동 실행하지 않는다.
- 승인 화면에 exact command, EXPECT, CWD, shell, PATH, timeout, output limit을 낸다.
- 승인 저장소는 저장소 바깥 사용자 전용 디렉터리이며 symlink·권한·교체 공격에 fail closed한다.
- 승인은 sandbox가 아니고 transitive script 안전을 보증하지 않는다고 명시한다.

### 15.2 Stop은 read-only다

- Stop은 원장을 파싱하고 축약할 뿐 명령을 실행하지 않는다.
- malformed ledger는 빈 원장이나 complete로 해석하지 않는다.
- 저장소가 쓴 자유문을 hook 오류 메시지로 그대로 반사하지 않는다.

### 15.3 성공 증거와 실패 진단

- 성공 출력은 기본적으로 digest와 byte count만 영속한다.
- 실패 출력은 터미널에 bounded diagnostic으로 낸다.
- 원문 보존이 필요한 gate는 명시적 artifact 경로와 보존 정책을 등록한다.
- 비밀이 출력 digest 바깥의 로그·report로 새지 않는지 시험한다.

## 16. 이슈 처분 제안

이 표는 실행 상태가 아니다. 회차 승인 뒤 실제 GitHub 이슈에서 각각 닫힘·흡수·접힘을 기록한다.

| 이슈 | 처분 제안 | 이유 |
|---|---|---|
| #85 Stop 정책 | **흡수** | 단계 4가 직접 답한다 |
| #88 다섯 문장 기입 | **기존 형태는 접고 새 실행 원장 이슈로 대체** | 비교 모집단이 v2.0 `SKILL.md`뿐이고 최신 핵심은 실행 코드다 |
| #95 CI 재귀 | **흡수** | CI를 committed condition이 아닌 terminal observation으로 옮긴다 |
| #96 진행 원장 | **흡수** | verification event ledger가 진행 원장이다 |
| #97 음성 대조 미실행 | **흡수** | 음성 대조 evidence 없이는 met가 될 수 없게 한다 |
| #90 표 헤더 이탈 | **단계 5 회귀로 흡수 검토** | 새 status가 회차 모집단을 구조적으로 읽어야 한다 |
| #92 수만 맞는 검산 | **단계 5 회귀로 흡수 검토** | ID/event 같음으로 바뀐다 |
| #94 선언/구현 drift | **단계 5 회귀로 흡수 검토** | reducer 한 자리에서 schema를 소비한다 |
| #84·#89·#93 | **이번 회차에서 접힘 후보** | 역사 기록/계기판 품질이며 사용자 완료 차단보다 뒤다 |
| #98·#99·#100 | **별도 감사/기록 무결성 회차 유지** | 새 실행 원장의 선행 기능은 아니지만 역사 판정의 신뢰 문제다 |

우선순위 접힘의 판정문은 다음이어야 한다.

> #84·#89·#93은 목표 안이지만 지금 우선순위가 아니다. 더 먼저인 것은 미완료 상태를 실제
> Stop 경계까지 연결하는 실행 가능한 완수 원장이다. 이유는 그것만이 사용자가 체감하는
> 거짓 완료를 직접 줄이고, 세 이슈는 그 원장의 종료를 막는 선행 의존이 아니기 때문이다.

## 17. 커밋과 회차 경계

한 회차에서 전부 만들지 않는다. 그러나 각 회차는 소비 가능한 수직 결과로 닫는다.

| 회차 | 산출 | 그 회차에서 소비 가능한 장면 |
|---|---|---|
| R1 | ADR + status reducer | 현재 회차가 왜 incomplete인지 한 명령이 정확히 답함 |
| R2 | approve + verify + evidence | 등록 조건 하나를 승인·실행·재검증할 수 있음 |
| R3 | negative control + dialectic event | 결정론/비결정론 두 종류가 같은 aggregate에 들어옴 |
| R4 | Stop 연결 | 미완료 최종 응답이 실제로 차단됨 |
| R5 | 기존 검사 통합·중복 제거 | `xtask`·dashboard·Stop이 같은 reducer를 읽음 |
| R6 | 실제 이슈 효과 | 사용자 작업 한 건이 차단→수정→재검증→통과로 끝남 |

각 회차의 자기 장치 발견은 금지역이면 그 회차에서 닫고, 그렇지 않으면 다음 회차를 실제로
즉시 열 수 있을 때만 분할한다. 열지 않을 것은 목표/우선순위 사유로 접는다. 단순히 이슈를
만드는 행위는 처분으로 세지 않는다.

## 18. 예상 위험과 퇴로

| 위험 | 조기 신호 | 대응/퇴로 |
|---|---|---|
| 원장이 셋째 진실원이 됨 | intent/gate/verification에 같은 문장·판정이 반복 | verification에는 ID·oracle·event만, 상태는 계산 |
| 임의 shell 승인 위험 | clone 직후 verify가 명령을 실행 | 미승인 기본, 외부 사용자 승인 저장소, Stop read-only |
| Windows process cleanup 실패 | timeout 뒤 자식이 남아 CI hang | R2에서 실제 nested descendant 시험, 실패하면 command mode를 argv-only로 축소 |
| worktree digest 비용 | status가 대화 흐름을 끊음 | 파일/HEAD 정책을 ADR에서 나누고 캐시는 상태가 아닌 성능층으로 둠 |
| Stop 무한 루프 | 같은 blocker로 반복 block | semantic progress guard + blocked handoff, complete 승격 금지 |
| Python/Rust parser drift | 같은 intent에 status와 xtask가 다른 ID 집합 | 두 번째 파서를 만들지 않고 기존 parser 출력 소비, 후속에 한 자리로 이동 |
| 모든 조건을 명령화 | 해석 조건이 억지 marker로 통과 | mode를 command/dialectic으로 닫힌 enum화 |
| 회차가 다시 연구로 팽창 | 실제 Stop 장면 전에 실험 과제 추가 | 합성 행동 실험 금지, 단계 6은 실제 프론티어 이슈 하나만 |
| upstream 복사본 drift | unlazy 패치 추적 이슈가 생김 | 코드 복사 대신 의미만 ADR에 기록하고 Rust로 독립 구현 |

## 19. 종료 판정

이 문제는 다음 전부가 참일 때 닫는다.

1. 결정론적 조건이 실행 증거 없이 met가 될 수 없다.
2. 등록한 음성 대조를 실행하지 않으면 met가 될 수 없다.
3. oracle이나 관련 snapshot이 바뀌면 과거 증거가 stale이 된다.
4. 종료 직전 `--reverify`가 모든 결정론적 조건을 다시 실행한다.
5. 메인 Stop이 incomplete를 실제로 차단한다.
6. blocked/folded는 성공으로 승격되지 않는다.
7. 비결정론 조건은 정반합 evidence로 같은 aggregate에 들어간다.
8. 실제 프로젝트 이슈 하나에서 차단→수정→재검증→통과 효과를 관측했다.
9. 세 OS CI가 마지막 SHA에서 성공했다.
10. #85·#88·#95·#96·#97이 구현·대체·접힘 중 하나로 실제 처분됐고 조용히 열려 있지 않다.

반대로 다음은 종료 근거가 아니다.

- `/round` 문면에 경고 다섯 줄을 추가했다.
- 체크박스와 게이트 표가 서로 맞는다.
- `cargo xtask check`가 현재 계약에서 초록이다.
- 독립 리뷰가 더는 큰 결함을 찾지 못했다.
- 합성 실험에서 평균 점수가 올랐다.
- Stop 훅이 기술적으로 호출되는 것을 봤다.

---

## 부록 A — 근거 좌표

### 우리 저장소

- `.palimpsest/rounds/2026-08-22-agent-laziness/intent.md` — `I4`가 규약 기입을 하지 않는 것을 통과로 등록
- `.palimpsest/rounds/2026-08-22-agent-laziness/report.md` — 다섯을 고른 근거와 #83~#95 분할
- `docs/gates/agent-laziness.md` — `CHECK: 0`, unlazy 문면 대조와 다섯 후보
- `.palimpsest/rounds/2026-08-23-agent-laziness-behavior/report.md` — 실험 결함, #96·#97, 자기 거짓 완료
- `docs/gates/agent-laziness-behavior.md` — 축의 비독립성과 진행 원장 축소
- `crates/pal-cli/src/hook/policy.rs` — 현재 `SubagentStop` 빈 반환만 차단
- `xtask/src/main.rs` — 회차 레코드·원장 둘 대조·발견 닫힘 검사
- `.claude/skills/round/SKILL.md` — 종료·막힘·접힘, 정반합, 모집단 분리, 현재 종료 조건
- `docs/adr/0025-the-harness-that-reads-the-graph-is-the-same-product.md` — 하네스가 같은 제품이라는 결정
- `docs/plan/00-stack.md` — Rust 단일 바이너리와 세 플랫폼 계약

### 외부

- [unlazy README](https://github.com/Leonxlnx/unlazy/blob/473d4b80421c36d733042434cd4b938f81a19ef1/README.md)
- [unlazy SKILL](https://github.com/Leonxlnx/unlazy/blob/473d4b80421c36d733042434cd4b938f81a19ef1/SKILL.md)
- [gate checker](https://github.com/Leonxlnx/unlazy/blob/473d4b80421c36d733042434cd4b938f81a19ef1/scripts/gate-check.mjs)
- [Stop hook](https://github.com/Leonxlnx/unlazy/blob/473d4b80421c36d733042434cd4b938f81a19ef1/scripts/stop-hook.mjs)
- [security boundary](https://github.com/Leonxlnx/unlazy/blob/473d4b80421c36d733042434cd4b938f81a19ef1/SECURITY.md)
- [validation limitations](https://github.com/Leonxlnx/unlazy/blob/473d4b80421c36d733042434cd4b938f81a19ef1/research/validation-protocol.md)
- [current CI](https://github.com/Leonxlnx/unlazy/actions/runs/33250725396)

## 부록 B — 이 보고서가 하지 않은 판정

- unlazy가 일반적으로 에이전트 생산성을 높인다고 판정하지 않았다.
- palimpsest의 행동 실험이 특정 처치의 무효를 증명했다고 판정하지 않았다.
- 제안 스키마·명령명·상수를 채택된 결정으로 적지 않았다.
- 열린 이슈를 실제로 닫거나 수정하지 않았다.
- 구현 비용과 일정 날짜를 추정하지 않았다. 상대 단계와 의존만 적었다.
