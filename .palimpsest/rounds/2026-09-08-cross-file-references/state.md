# 상태 — 2026-09-08-cross-file-references

## 지금 단계

**승인이 끝났고 루프(실행)에 아직 안 들어갔다.** 코드는 한 줄도 안 고쳤다 —
`scripts/f06-verify.py`·`f09-verify.py` 의 오타 둘만 고쳤고 그것은 착수 기준선을
재기 위한 확대다.

착수 커밋 `6b6cb6d` · 승인 시점 HEAD 는 `git log` 가 답한다.

## 다음 컨텍스트가 받는 것

**잠긴 의도 전문(`intent.md`)과 이 파일뿐이다.** 정반합 산출물·사전부검·조건 감사의
원 반환문을 시드로 주지 마라 — 규약 §5 「교대」가 금한다. 필요하면 발견 원장
(`findings.jsonl`, 123 행)을 조회한다.

## 무엇이 끝났나

| 단계 | 상한 | 쓴 것 | 산출물 |
|---|---|---|---|
| 인터뷰 | 3 | 3(소진) | `intent.md` 의 `## 원문` |
| 착수 기준선 | — | 1 | `observations/baseline-start.txt` |
| 사전부검 | 2 | 1 | `premortem/r1-raw.md` (20 항) |
| 범위 재잠금 정반합 | 2 | 2(소진) | `dialectic/` 여덟 · **소유자 승격 한 번** |
| 완수 조건 설계 평가 | 2 | 2(소진) | `conditions-audit/r1-raw.md`·`r2-raw.md` |
| 승인 | — | 완료 | `intent.md` 의 `## 승인` |

`cargo xtask check` **27/27 초록**. 조건 **37 개** 형식 오류 0.

## 실행에서 먼저 볼 자리 — 실측한 좌표

| 무엇 | 어디 |
|---|---|
| 파일 밖 참조를 표시하는 갈래 | `crates/pal-core/src/scope.rs:185` `RefResolution::OutsideFile` |
| 그 갈래가 버려지는 자리 | `crates/pal-core/src/projection.rs:248` |
| ⚠ **임포트 참조는 그리로 안 온다** | `rust_scopes.rs` 가 `use` 를 모듈 스코프 바인딩으로 선언 → `Bound{NotASymbol}` → `projection.rs:262-265` 의 `counts.locals` |
| 임포트 타입 | `crates/pal-core/src/file_graph.rs:129` `ImportSet` — `modules` 하나뿐이고 **항목 이름이 없다** |
| 익스포트 타입 | 같은 파일 `:76` `ExportSet` — `names`·`star_from`·`has_default` |
| Rust 추출기의 표면 | `crates/pal-extract/src/rust.rs:417` `표면()` · `:468` `재수출을_담는다` |
| 2층 잇기 | `crates/pal-cli/src/ledger.rs:354` `stitch_of` — `EXPORTS` 는 **유일하게 해소되는 최상위 이름만** 담는다 |
| 파일 안 엣지 산출 | `crates/pal-core/src/projection.rs:236` `file_edges` |
| 추출기 판번호 | `crates/pal-extract/src/lib.rs:147` `EXTRACTOR_REV` |

## 실패한 접근

- **`use` 직접 임포트만으로 범위를 잠근 것** — 소유자가 2026-09-09 에 전환을 승인해
  `crate::`·`super::`·`self::` 접두가 안으로 들어왔다.
- **`RefResolution::OutsideFile` 을 착지점으로 지목한 것** — 임포트한 이름은 그 갈래에
  안 온다(위 표).
- **모집단을 「315 자리 · 25 파일」로 적은 것** — 정·반·합 셋을 통과한 뒤 2 라운드에서
  깨졌다. 잠근 값 ㉯ 의 모집단은 **1611 자리 · 엣지 상한 1159 · 파일 81 · 크레이트 3**.
- **발견 레코드를 손으로 옮겨 적은 것** — `extract.py` 로 뽑아야 한다. 검사가 잡았다.
- **관측 파일을 `.jsonl` 로 회차 디렉터리에 둔 것** — 원장 검사가 그것을 읽는다.
- **`intent.md` 에 같은 절(`## 차선책`)을 두 번 둔 것** — 감사자가 빈 쪽을 읽고 오판했다.

## 사고 기록

**하위 에이전트가 원본 `schema/graph.toml` 에서 216 줄을 지웠다**(`[node.Binding]` 절 전량).
`git checkout` 으로 복구했다. 격리 사본이 아니라 원본에서 파괴 실험을 한 것으로 보인다.

## 아직 안 정해진 것 — 실행 중에 갈린다

**`A7`** — 축2 의 ⓑ(`S::foo()`)를 세우나. 세우면 `rust_scopes.rs` 의 `경로_꼬리_배제` 를
끄는 것이고 그 주석이 *"끄면 실측 후보 6,961 이 참조가 된다"* 로 적으므로 모집단과
`E2` 의 하한이 다시 서야 한다(`A7-a`). **안 세우면 축소이고 승격이다.**
