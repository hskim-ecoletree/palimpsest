# palimpsest

환경에 종속되지 않는 코드 이해의 큐레이터. **코드가 서 있다** — `pal` 바이너리(`symbols`·`ledger`·`bind`·`touch`·`query`·`defect`·`doctor`·`cache`·`intent`)와 크레이트 일곱. 무엇이 실제로 서 있고 무엇이 자리뿐인지는 **문서가 아니라 산출이 말한다**: `pal doctor` 는 검사하지 못한 것을 `Residual` 로, 담을 수 없는 것을 능력 부재로 낸다.

## 지금 어디에 서 있는가

```bash
./scripts/frontier.sh
```

착수 가능한 이슈를 센다. **상태는 이슈에만 있고 문서는 그것을 복제하지 않는다** — 어디까지 왔는지 알고 싶으면 문서를 읽지 말고 이것을 실행할 것.

## 어디서부터 읽는가

| 알고 싶은 것 | 문서 |
|---|---|
| 무엇을 어느 순서로 만드나 (지형) | [docs/plan/README.md](docs/plan/README.md) — **전체를 관장한다** |
| 완성되면 실제로 어떻게 쓰이나 | [docs/how-it-works.md](docs/how-it-works.md) |
| 왜 만드나 / 모든 기능의 채점 기준 | [docs/plan/00-goals.md](docs/plan/00-goals.md) |
| 무슨 언어·라이브러리·구조로 만드나 | [docs/plan/00-stack.md](docs/plan/00-stack.md) |
| 무엇이 우리를 막을 것인가 | [docs/plan/00-risks.md](docs/plan/00-risks.md) |
| 설계 결정과 그 근거 | [docs/DESIGN.md](docs/DESIGN.md) · [docs/evidence-map.md](docs/evidence-map.md) |
| 왜 이런 제품이어야 하나 | [WHITEPAPER.md](WHITEPAPER.md) |

## 진행 규칙

전문은 [계획 §7](docs/plan/README.md)에 있다. 산출물을 쓰기 전에 걸리는 것 넷:

1. **게이트는 판정 기록을 커밋으로 남긴다** — `docs/gates/<기능>.md`에 **통과 · 반증 · 대조 불가** 셋 중 하나. 생략하고 다음으로 가는 것이 이 계획의 가장 조용한 실패 경로다.
2. **일정을 두지 않는다.** 순서는 게이트가 정하고 작업에는 상대 규모(S/M/L/XL)만 붙는다.
3. **반증은 실패가 아니다.** 기능이 취소되면 계획이 작동한 것이다.
4. **결정은 설계 문서에, 근거는 근거 대장에, 실행은 이슈에.** 같은 것을 두 곳에 적으면 그것이 곧 drift다.

**ADR은 기능 착수가 아니라 종료 시점에 발행한다.** 착수 시점의 ADR은 결정이 아니라 계획의 복사본이다.

## Agent skills

### Issue tracker

이슈는 `hskim-ecoletree/palimpsest`의 GitHub 이슈로 산다. 이 설치본의 `gh`는 2.90이라 하위 이슈와 blocking을 `gh api`로 걸어야 한다. `docs/agents/issue-tracker.md` 참조.

### Triage labels

정본 다섯 상태(`needs-triage`·`needs-info`·`ready-for-agent`·`ready-for-human`·`wontfix`)를 이름 그대로 쓴다. 라벨은 트래커에 이미 생성되어 있다. `docs/agents/triage-labels.md` 참조.

### Domain docs

단일 컨텍스트 — 루트의 `CONTEXT.md` + `docs/adr/`. **`CONTEXT.md` 는 아직 없고 없는 것이 정상 상태다.** ADR 은 **열** 있다(F22·F01·F02·F03·**F05** 종료 시 발행). `docs/agents/domain.md` 참조.
