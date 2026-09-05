# 정반합 1 — 후보 선정 판정

정(正) 초안은 `thesis.md`. 반(反)은 반론 17 건, 합(合)은 16 채택 · 1 기각.

## 합(合)의 판정: **수정**

살아남은 반론에 **금지역 아홉**(#1·#3·#4·#5·#6·#7·#9·#12·#15)과 **실패 하나**(#13)가 있다.
`/round` §5 의 게이트가 그것을 소유자에게 올리라고 한다.

## 무엇이 무너졌나

**① 초안의 R1 읽기가 사후 수정이었다.** 등록된 규칙은 *"틀 자체는 이 규칙을 자동으로
통과한다 — 무엇이든 **담을 수 있기** 때문이다"* 로 「담을 수 있는가」를 이미 못 박았는데,
초안이 판정 시점에 「지금 자리가 있는가」로 바꿔 읽었다. 그리고 **대칭으로 적용하지 않았다** —
RDF/OWL·PROV-O 에는 ⚠ 조항을 주고, 모집단 파일이 스스로 *"타입 시스템이 선언적이다"* 로
등록한 Apache Atlas 에는 안 줬다. R1 이 탈락 전량을 낸 유일한 규칙이므로, 그 읽기가 사후에
정해졌다는 것은 **A1(선등록) 보증을 통째로 무력화한다.**

**② 탈락 판정 셋이 조회로 반증됐다.** 합(合)이 파일을 직접 열어 확인했다.

| 후보 | 초안이 적은 것 | 실제 |
|---|---|---|
| Apache Atlas | *"사람이 쓴 글의 자리가 기본 타입에 없다"* | `Asset` 에 `description`·`userDescription`, `AtlasGlossaryTerm` 에 `shortDescription`·`longDescription`·`examples`·`usage` |
| OpenLineage | *"사람이 쓴 글의 자리가 없다"* | `DocumentationJobFacet`(`documentation`·`contentType`) · `SourceCodeLocationJobFacet`(`repoUrl`·`path`·`version`·`branch`) |
| CodeMeta | *"용어 83 개가 전부 저장소·릴리스 수준"* | `readme`·`releaseNotes`·`buildInstructions`·`softwareHelp`·`review`·`reviewBody` |

**③ 단서를 도입한 근거가 거짓이었다.** 초안은 *"심볼 노드 그 자체를 요구하면 열다섯이
전량 탈락"* 이라며 단서를 넣었는데, CodeQL 의 `semmlecode.dbscheme` 이 반증한다 —
`@locatable = … | @javadoc | @javadocTag | @javadocText | @ktcomment | …` 로 **심볼과
사람이 쓴 글이 한 좌표계에 있다.** 그 단서가 OpenMetadata 를 통과시킨 장치였다.

**④ 초안의 표가 성립하는 읽기가 없다.** 초안 본문이 OpenMetadata·DataHub 둘 다에
*"코드 심볼의 자리는 없다"* 라고 **스스로 적어 놓고** 통과시켰다. 합(合)이 확인한 것:
`contextFile.json` 의 속성에 `region`·`line`·`range` 류가 없다. 엄격히 읽으면 1·2 위가
탈락하고 느슨히 읽으면 탈락 근거가 무너진다.

**⑤ 모집단에 「소프트웨어 카탈로그」층이 통째로 없다.** 카탈로그 다섯이 전부 *데이터*
카탈로그다. 합(合)이 넷을 확인했다 — **SARIF**(`graph`·`node`·`edge` + `physicalLocation`·
`region` + `message` + `logicalLocation` + `versionControlDetails` 가 전부 있다) ·
**Backstage**(`Component`·`API`·`System`·`Domain`·`Resource` + TechDocs) · CycloneDX · Egeria.
앞의 둘은 **엄격한 읽기로도 R1 을 통과한다.** 이 공백을 그대로 두면 결론이
*"데이터 카탈로그·어휘 표준 중에서는 OpenMetadata"* 인데 ADR 은 *"OpenMetadata 를 적용한다"* 로 읽힌다.

**⑥ 넷째 칸이 규칙을 어긴다.** 규칙 원문이 *"⚠ 넷을 못 채우면 채우지 않는다. 빈자리를
메우려고 규칙을 낮추면 R1~R3 이 무력해진다"* 인데 「긴장 ① 한정」이 정확히 그 동작이다.
그리고 완수 조건 `B1`(*"남은 후보마다 긴장 ①~⑤ 판정이 있다"*)이 **원리상 안 닫힌다.**

**⑦ 음성 대조 넷 중 누락을 잡는 것이 하나도 없다.** ⓐ~ⓓ 는 *더한 것*만 잡고
*빠뜨린 것*은 못 잡는데, 모집단을 그린 손과 후보를 고른 손이 같다. ⑤가 사전 등록 장치를
그냥 통과한 이유가 이것이다.

## 합(合)이 기각한 것

**#17**(탈락 셋이 반증되면 갈림 2 가 불필요해진다) — 추론이 성립하지 않는다. 갈림 2 는
통과 수가 아니라 **층 수**에 걸린 문제이고, 반증된 셋은 이미 차 있는 세 층에 더해질 뿐이다.
갈림 2 를 실제로 무너뜨리는 것은 #6·#15(CodeQL·SARIF·Backstage 가 넷째 층을 만든다)다.

## 합(合)이 못 정한 것

- **Matrix · ActivityStreams · Dublin Core · LSIF 의 R1 실측** — 반(反)도 합(合)도 정의
  파일을 안 열었다. **미측정**이고 「탈락」으로 셀 수 없다. 다만 이 넷은 어느 쪽으로
  판정되든 고르는 넷을 안 바꾼다.
- **CodeMeta 의 최종 R1** — 아래 Q1 의 답에 종속된다.

## 규칙대로 읽었을 때 후보가 몇이 되나 — 합(合)이 직접 낸 것

| 읽기 | 통과 | 무엇이 달라지나 |
|---|---|---|
| **초안의 단서를 대칭 적용** | OpenMetadata · DataHub · SPDX · RDF/OWL · PROV-O · **Atlas** · **OpenLineage** · **CodeQL** (+CodeMeta 경계) | R1 이 거의 안 거르고 선별을 층 규칙이 한다 |
| **등록 원문대로 엄격히**(「기계 추출물(**코드 심볼**)」) | CodeQL · SARIF · … — **OpenMetadata 와 DataHub 가 탈락한다** | 회차가 「적용 안 한다」로 크게 기운다 |
| **등록 ⚠ 조항의 근거대로**(「담을 수 있는가」) | 선언적 타입 시스템을 가진 것 대부분 | R1 이 게이트가 아니라 통과 표시가 된다 |

**어느 읽기로도 초안의 「탈락 10 · 통과 5」는 재현되지 않는다.**
