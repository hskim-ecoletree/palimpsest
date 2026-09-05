# 상태 — OpenMetadata 적용 판정

## 지금 단계
루프 — 실행. 의도는 잠겼고 소유자가 승인했다(2026-09-05).

## 사전부검
상한 2 라운드를 다 썼다. 원 반환문은 `premortem/r1-raw.md` · `premortem/r2-raw.md`.
1라운드 15 건 → 2라운드 17 건. 감소하지 않았다 — 1라운드의 처분이 새 표면을 냈다.
2라운드가 1라운드의 처방 둘을 뒤집었다: `pal export` 의 `missing` 은 인스턴스 수가 아니고,
「증거를 저장소 밖에」는 과잉이다(막히는 것은 확장자 `.json`·`.jsonl`·`.tsv` 뿐).

## 승격
1건. 이슈의 「실제 동작을 재서」 요구를 인터뷰의 「경량 실측만」이 축소한다 →
**소유자가 축소를 승인**했다. ① 은 추정으로 적는다(`B3`).

## 회차
- slug: `2026-09-05-openmetadata-decision`
- 이슈: [#104](https://github.com/hskim-ecoletree/palimpsest/issues/104)
- 착수 커밋: `930a295`

## 인터뷰로 잠근 것 (2 라운드에서 닫았다)
| 물음 | 소유자의 답 |
|---|---|
| 실측 수준 | **경량 실측만** — full stack 안 세운다. 스키마 정의·SDK 구조를 직접 읽는다 |
| 비교 범위 | **어휘 표준까지** — 제품 말고 RDF/OWL·schema.org·SPDX 도 후보 |
| 후보 목록 | **메인이 선정 규칙을 먼저 적고 그것으로 거른다.** 탈락도 이유와 함께 |
| 결정 대상 | **온톨로지 전체** — `schema/graph.toml` 의 코드·git 그래프까지 판정 대상 |
| 적용 | **이 회차가 안 한다.** `schema/graph.toml` 은 안 바뀐다. 적용은 새 이슈 |
| ① 이 막으면 | **다섯 전부 잰다.** 조기 종료 안 한다 |
| 재고 | **#102·#69 는 안 본다.** 관계만 ADR 에 한 줄 |
| 산출 | **ADR + 게이트 문서.** 「안 쓴다」여도 ADR 을 낸다 |

## 착수 시점에 잰 사실
- OpenMetadata 로컬 배포: Docker 에 **6 GiB · 4 vCPU**. MySQL/Postgres · Elasticsearch · Airflow · 서버 · 인제스션. 임베디드 모드는 공식 문서에 없다.
- 이 PC: 메모리 32 GB · Docker 29.6.1 설치 · **데몬 꺼짐**
- OpenMetadata **2.0.1** (2026-09-02 릴리스, 커밋 2026-09-05). `entity/data/ontologyAxiom.json` 이 1급 엔티티로 있고 OWL 공리 타입 여섯과 `provenance` 를 진다. `relationshipType.json`·`conversationSource.json`·`conceptMapping.json` 도 있다.

## 실패한 접근
(아직 없음)

## 남은 것
사전부검 처분 → 완수 조건 잠금 → 승인 → 조사 루프 → ADR·게이트
