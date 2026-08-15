<!-- 이 파일은 `cargo xtask query-doc` 이 낸다. 손으로 고치지 않는다. -->
<!-- 정본은 surface/queries.toml 이고 CI 가 둘의 일치를 센다. -->

# 질의 카탈로그 v1

**이 빌드가 답하는 질의 10개.** 여기 없는 것은 이 빌드가 답하지 않는다 — [F06 §3](plan/features/F06-surface.md)의 표는 **로드맵이고 이 표의 상위집합이 아니다**.

이름을 받는 질의는 `Ambiguous`(여럿이라 못 좁혔다)와 `Unknown`(이 스냅샷에서 못 찾았다)으로도 답한다. **둘 다 실패가 아니라 답이고 종료 코드 0 이다.**

| 질의 | 인자 | 반환 | 도입 | 요약 |
|---|---|---|---|---|
| `binding.status` | — | `Bindings` | F09 | 결박마다 상태 + **반경** + 무엇이 켰는가 |
| `binding.touch` | `name: SymbolName` | `Touch` | F11 | 좌표 하나를 만진다 — **걸린 것**과 ★ **지켜보는 것**을 함께 낸다 |
| `graph.dump` | — | `Graph` | F05 | 노드와 엣지 전부 — 바깥 오라클이 읽는 창 |
| `ledger.snapshot` | — | `Ledger` | F01 | 이 스냅샷의 관측 범위 대장 |
| `narrative.unbound` | — | `Narrative` | F10 | 좌표를 못 찾은 문서 조각들 — **사람의 작업 목록** |
| `plan.deviation` | `plan: RepoPath` | `Deviation` | F12 | 계획과 실제의 갈림 — 넷이고 ★ **못 잰 것이 분리돼 있다** |
| `symbol.callers` | `name: SymbolName` | `Symbols` | F05 | 이 심볼을 가리키는 것들 — 1홉 역방향 |
| `symbol.contains` | `name: SymbolName` | `Symbols` | F02 | 이 심볼이 담는 것들 — 컨테이너 체인으로 |
| `symbol.reaches` | `name: SymbolName` | `Reached` | F05 | 이 심볼에서 닿는 것들 — **예산 절단이 있는 BFS** |
| `symbol.resolve` | `name: SymbolName` | `Symbols` | F03 | 이름 하나 → 후보 심볼들. **여럿인 것이 정상이다** |
