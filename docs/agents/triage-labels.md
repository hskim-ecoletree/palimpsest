# 트리아지 라벨

스킬은 다섯 개의 정본 트리아지 역할로 말한다. 이 파일은 그 역할을 이 저장소 트래커의 실제 라벨 문자열에 대응시킨다.

| mattpocock/skills의 라벨 | 우리 트래커의 라벨 | 뜻 |
| --- | --- | --- |
| `needs-triage` | `needs-triage` | 평가가 필요하다 |
| `needs-info` | `needs-info` | 보고자의 응답을 기다린다 |
| `ready-for-agent` | `ready-for-agent` | 완전히 명세되어 에이전트가 착수할 수 있다 |
| `ready-for-human` | `ready-for-human` | 사람의 구현이 필요하다 |
| `wontfix` | `wontfix` | 처리하지 않는다 |

카테고리 역할 둘은 GitHub 기본 라벨을 그대로 쓴다 — `bug`, `enhancement`.

스킬이 역할을 언급하면(예: "AFK 착수 가능 라벨을 붙여라") 이 표의 대응 문자열을 쓴다.

**라벨 일곱은 이미 생성되어 있다.** `setup-matt-pocock-skills`는 이 매핑 파일만 쓰고 `gh label create`를 실행하지 않는다(알려진 결함). 라벨이 없으면 `gh issue create --label`은 라벨을 만들지 않고 그대로 실패한다.

## 이 저장소에서 `/triage`를 쓰는가

거의 쓰지 않는다. `/triage`는 **남이 만든** 이슈를 위한 것이다 — 외부에서 들어온 버그 보고, 기능 요청, 예고 없이 도착한 PR. 이 저장소는 비공개 1인 프로젝트이고 모든 이슈가 자체 계획에서 나온다. `to-tickets`가 발행한 티켓은 구성상 이미 `ready-for-agent`이므로 그 위에 트리아지를 돌리는 것은 낭비다.

이 매핑이 존재하는 이유는 `to-tickets`가 발행 시점에 `ready-for-agent`를 붙이기 위해서다.
