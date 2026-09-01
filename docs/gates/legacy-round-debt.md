# 게이트 — legacy round debt

> 회차 [`2026-08-31-legacy-round-debt`](../../.palimpsest/rounds/2026-08-31-legacy-round-debt/) ·
> 잠긴 의도 [`intent.md`](../../.palimpsest/rounds/2026-08-31-legacy-round-debt/intent.md) ·
> 이슈 [#98](https://github.com/hskim-ecoletree/palimpsest/issues/98) ·
> [#99](https://github.com/hskim-ecoletree/palimpsest/issues/99) ·
> [#100](https://github.com/hskim-ecoletree/palimpsest/issues/100)

## 합격선

끝난 회차의 기계 칸은 보존된 원 반환문에서 다시 만들되 당시 ID와 사람 판단을 보존해야
한다. 닫힘 41행은 실제 처분 근거를 가져야 하고, 종료 판정은 게이트와 의도 두 원장만
가져야 한다. 세 빚 목록과 판정 발화는 0이어야 하며 각 결함을 되살린 음성 대조가 서로
다른 진단으로 실패해야 한다.

**RED** — 착수 SHA `1c3c4e2`의 `cargo xtask check`는 끝난 회차 손 전사 2530칸
(`갈린 칸 1934`·`빠진 행 596`), 감사 대기 41행, 종료 보고 검산 줄 유예 2회차와
보고 없음 유예 1회차를 냈다. 원문은
[`red/baseline.md`](../../.palimpsest/rounds/2026-08-31-legacy-round-debt/red/baseline.md)가 진다.

## 판정

| 판정 | 조건 |
|---|---|
| 통과 | A1 A2 A3 B1 B2 B3 C1 C2 C3 C4 D1 D2 D3 |
| 반증 | — |
| 대조불가 | — |
| 미측정 | — |

**검산** — 통과 13 · 반증 0 · 대조불가 0 · 미측정 0 = 13

### 현재 근거

- A1·A2: 네 역사 회차 52개 원문 그룹, 980행을 실제 재이주 CLI로 왕복해 ID와 사람
  판단 칸을 보존했다. 끝난 회차의 `갈린 칸`과 `빠진 행`은 모두 0이다.
- A3: 기계 칸 변조와 행 삭제가 각각 `갈린 칸 1`, `빠진 행 1`을 냈다.
- B1·B2: [`audit/issue-98.md`](../../.palimpsest/rounds/2026-08-31-legacy-round-debt/audit/issue-98.md)가
  41행의 처분 근거를 전수 열거한다. 닫힘 발화와 감사 대기 발화는 모두 0이다.
- B3: 감사 SHA를 다른 유효 SHA로 바꾸자 요구 좌표를 안 만졌다는 진단으로 실패했다.
- C1·C2: 옛 보고 두 곳의 셋째 판정 사본을 제거했다. `2026-08-18-round-protocol`은
  보존된 정확한 종료 표지와 당시 CI만 인정하며 보고를 소급 생성하지 않았다.
- C3·C4: 검산 줄 복원과 종료 표지 훼손은 서로 다른 진단으로 실패했다. 세 유예 선언은
  0개이고 빈 목록도 빚으로 오인하지 않는다.
- D1: 추출 회귀, xtask 42개 단위시험, `cargo xtask check` 23/23과 workspace all-targets가
  성공했다. 구현·리뷰 SHA `ac0cef86b70c6976c7140ded1b5e7ff4e5dfe4db`의
  [CI run 33404287032](https://github.com/hskim-ecoletree/palimpsest/actions/runs/33404287032)에서
  ubuntu·macOS·windows와 설치·양방향 호환 7개 작업이 모두 성공했다.
- D2: 시험이 아닌 실제 판정 출력은 손 전사·갈림·빠진 행·감사 대기·검산 줄 유예·보고
  없음 유예를 모두 0으로 냈다. 원문은
  [`effect/observation.md`](../../.palimpsest/rounds/2026-08-31-legacy-round-debt/effect/observation.md)에 보존했다.
- D3: 사전부검과 두 독립 리뷰의 발견은 구조화 원장에 닫혔다. 기존 결박 15개를 보존하고
  `check_round_records`·`check_finding_closure`·`check_ledger_pair`에 결박 3개를 더했다.
  전용 게이트와 종료 보고가 섰고 이슈 #98·#99·#100을 같은 증거로 닫았다.

## 효과

같은 실제 판정 명령에서 손 전사 2530칸, 감사 대기 41행, 종료 보고 유예 3회차가 모두
0이 됐다. 숫자를 숨긴 것이 아니라 다섯 음성 대조가 각 결함을 다시 드러내는 것을 확인했다.

## 범위 밖

- #93·#94·#95와 다른 프론티어 이슈 구현 — 이번 세 빚보다 소비 지점에서 멀다.
- 과거 회차의 합격선과 사람 판정을 새로 쓰는 일.
- 새 레코드 스키마, 새 사용자 표면, 새 일정 문서.
