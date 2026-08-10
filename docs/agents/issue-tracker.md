# 이슈 트래커: GitHub

이 저장소의 이슈와 스펙은 GitHub 이슈로 산다 — `hskim-ecoletree/palimpsest`(비공개). 모든 조작은 `gh` CLI로 한다.

## 관례

- **이슈 생성**: `gh issue create --title "..." --body "..."`. 여러 줄 본문은 heredoc을 쓴다.
- **이슈 읽기**: `gh issue view <번호> --comments`
- **목록**: `gh issue list --state open --json number,title,body,labels,comments --jq '[.[] | {number, title, body, labels: [.labels[].name], comments: [.comments[].body]}]'` — `--label`·`--state`로 좁힌다.
- **댓글**: `gh issue comment <번호> --body "..."`
- **라벨 적용/해제**: `gh issue edit <번호> --add-label "..."` / `--remove-label "..."`
- **닫기**: `gh issue close <번호> --comment "..."`

저장소는 `git remote -v`에서 추론된다 — 클론 안에서 실행하면 `gh`가 알아서 한다.

## 이 설치본의 `gh`는 2.90 — `--parent`도 `--blocked-by`도 없다

`gh issue create --parent`는 2.94부터다. 이 기계는 2.90이므로 **부모 링크와 blocking을 `gh api`로 걸어야 한다.** 이것을 생략하면 `to-tickets`의 가장 흔한 두 결함이 그대로 재현된다 — 하위 이슈가 안 만들어지고, "Blocked by"가 본문 텍스트로만 적혀 아무것도 그것을 읽지 못한다.

**필요한 것은 `#번호`가 아니라 데이터베이스 id다:**

```bash
gh api repos/hskim-ecoletree/palimpsest/issues/<번호> --jq .id
```

**하위 이슈로 붙이기** (부모의 `<번호>`, 자식의 데이터베이스 id):

```bash
gh api --method POST repos/hskim-ecoletree/palimpsest/issues/<부모번호>/sub_issues \
  -F sub_issue_id=<자식-db-id>
```

**blocking 엣지 걸기** (`<자식번호>`가 `<차단자>`에 막힌다):

```bash
gh api --method POST repos/hskim-ecoletree/palimpsest/issues/<자식번호>/dependencies/blocked_by \
  -F issue_id=<차단자-db-id>
```

차단자를 먼저 발행하면 그 번호와 id가 항상 손에 있다. **본문의 `Blocked by:` 줄은 네이티브 엣지가 없는 트래커를 위한 대체물이지 기본값이 아니다.**

## 착수 가능한 것(frontier) 조회

열려 있고, 열린 차단자가 없고, 담당자가 없는 이슈:

```bash
gh issue list --state open --json number,title,assignees,labels \
  --jq '[.[] | select(.assignees | length == 0)]'
```

열린 차단자 수는 이슈별로 `issue_dependencies_summary.blocked_by`가 보고한다:

```bash
gh api repos/hskim-ecoletree/palimpsest/issues/<번호> --jq '.issue_dependencies_summary.blocked_by'
```

**착수 표시(claim)는 세션의 첫 쓰기다**: `gh issue edit <번호> --add-assignee @me`.

## 스킬이 "이슈 트래커에 발행하라"고 할 때

GitHub 이슈를 만든다.

## 스킬이 "해당 티켓을 가져오라"고 할 때

`gh issue view <번호> --comments`.

## PR을 요청 표면으로 다루는가

**아니다.** 이 저장소는 비공개 1인 프로젝트이고 외부 PR이 없다. (`/triage`가 이 플래그를 읽는다. 외부 PR을 트리아지 큐에 넣고 싶어지면 여기를 `yes`로 바꾼다.)

## 이 저장소에만 있는 규칙 — 이슈가 무엇을 담고 무엇을 담지 않는가

계획 문서와 이슈가 같은 것을 두 곳에 적으면 그것이 곧 drift다([계획 §7.4](../plan/README.md)). 경계는 이렇다.

| | 어디 |
|---|---|
| 각 기능을 **어떻게** 만드는가 — 구현 방식·라이브러리·이슈·대안 | `docs/plan/features/F<NN>-*.md` (저장소) |
| **무엇을 어느 순서로**, 지금 **어디까지 왔는가** | GitHub 이슈 (상태의 단일 진실) |
| 왜 그렇게 정했는가 | `docs/DESIGN.md` |
| 무엇을 보고 정했는가 | `docs/evidence-map.md` · `docs/research/` |
| 수치 합격선 | `corpus/criteria.toml` |
| 게이트 판정 기록 (통과·반증·대조 불가) | `docs/gates/<기능>.md` — **커밋으로 남긴다**([계획 §7.1](../plan/README.md)) |

**이슈 본문은 기능 문서를 복제하지 않고 가리킨다.** 기능 문서가 200줄이면 그 200줄은 저장소에 남고, 이슈는 목적·인수 기준·의존만 담은 채 경로로 링크한다.

**`/implement`는 티켓을 닫지 않는다** (알려진 동작 — 커밋에서 끝난다). 이 저장소에서는 그것이 오히려 맞다: 게이트 판정은 사람이 `docs/gates/`에 기록하고, 그 커밋 뒤에 이슈를 닫는다.

## 쓰지 않는 것

- **`/wayfinder`** — 다중 세션 규모를 결정 티켓 지도로 그리는 스킬. 이 저장소는 그 지도를 이미 갖고 있다(`docs/plan/`). 실행하면 두 번째 지도가 생긴다.
- **`/to-spec`** — `docs/plan/features/`가 이미 스펙이다. `to-tickets`는 기능 문서를 직접 읽는다.
