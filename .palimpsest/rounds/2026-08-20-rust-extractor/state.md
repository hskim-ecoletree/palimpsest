# 상태 — 회차 E · Rust 추출기

> **교대용이다.** 새 컨텍스트에 주는 것은 **잠긴 의도 전문 + 이 요약**이다.
> 직전 산출물을 시드로 주지 않는다.

## 지금 어느 단계인가

**§7 독립 리뷰** — 라운드 3/5 처분 완료, 라운드 4 대기.

★ **CI 는 이미 초록이다**(`966850b`). 이후 커밋이 나오면 **그 SHA 에 대해 다시 재야** 한다 — §11-6 은 「회차의 **마지막** 커밋」을 요구한다.

- ✔ §1 인터뷰 — **4 라운드 상한 소진**
- ✔ §2 사전부검 — **3 라운드 상한 소진** (20+9 · 16+9 · 12+10 = 76 건)
- ✔ §3 완수 조건 44 잠금
- ✔ §4 승인
- ⏳ §5 루프 — 추출기·오라클·게이트·ADR·이슈 다섯이 섰다
- ✔ §8 효과 — 넷 붙였다(ledger 전후 · narrative · 질의 둘 · ADR 결박)
- ✔ §10 종료 보고 — `report.md`
- ✔ §11-6 push · CI — `966850b` 에 `conclusion=success` (일곱 잡)

## 착수 좌표

| | |
|---|---|
| 착수 커밋 | `56926aa` |
| 이슈 | [#66] · 분할 [#77]~[#81] |
| §11③ 선택 | **(나)** |

## 상한

| 계열 | 상한 | 실제 |
|---|--:|--:|
| 인터뷰 | 4 | **4 소진** |
| 사전부검 | 3 | **3 소진** |
| 독립 리뷰 | 5 | **3** |

## 이 회차가 세운 것

- `crates/pal-extract/src/rust.rs` — 중첩 순회 · L1
- `parse.rs` — `attribute_item` 건너뛰기 · `doc_comment` 접기 · 시험 넷
- `Language::Rust` · `SymbolKind` 일곱 · `FIRST_CLASS` 다섯
- `corpus/tasks/rust-recall-sample.tsv` — 손 표본 13 파일 · 268 선언
- `corpus/criteria.toml` `[rust]` · `docs/gates/rust-extractor.md`
- `docs/adr/0027-…` · `docs/instructions/2026-08-20-owner-direction.md`
- `scripts/rust-recall-verify.py` · `scripts/syn-oracle/`(음성 대조군)
- `crates/pal-extract/examples/count_marked.rs`

## 지금까지의 수

| | |
|---|---|
| `pal ledger` 의 Rust | `L0` → **`L1`** (착수 117 파일이 전부 빠졌다) |
| `pal narrative` 결박됨 | 0 → **46** (접기 끄면 73) |
| 손 표본 재현율 / 정밀도 | **94.40% / 94.40%** (cargo 핀 고정이라 안 변한다) |
| 발견 레코드 | `python3 .claude/skills/round/bin/record.py count <회차>` |

⚠ **변하는 수는 여기 안 적는다.** 파일 수·심볼 수·레코드 행 수는 커밋마다 변하고,
이 회차에서 **세 번 갈렸다.** 세는 명령만 적는다.

## 실패한 접근 — 다음 컨텍스트가 같은 벽에 안 부딪히도록

- **「합계를 읽으며 센다」 — 세 번 다 틀렸다.** 라운드 1 을 19(→20), 라운드 2 를
  17(→16) 로 세었다. **세는 자리는 `grep -c "^### "` 이다.**
- **`rc` 를 판정으로 읽으면 안 된다.** `cargo xtask check` 가 죽은 링크로 실패했는데
  exit code 는 0 이었다. 독립 리뷰어도 같은 자리에서 물렸다(`timeout` 미설치).
- **`f10-6-verify.py` 를 돌리면 손 표본의 `판정`·`근거` 열이 지워진다.** `--bless`
  조차 필요 없다. 되돌렸고 [#81] 로 분할했다.
- **`f01-verify.py --repo` 에 `~/dev/projects/boxwood` 를 주면 SHA 를 못 찾는다.**
  `boxwood/portal-backend` 를 줘야 한다(8 저장소 워크스페이스다).
- **`사이에_빈_줄` 로 doc 접기의 경계를 판정하면 틀린다.** `line_comment` 가 끝의
  줄바꿈을 마디에 담아 `\n` 이 하나만 남는다 — **줄 번호로 봐야 한다.**
- **순회 중에 `impl` 대상을 찾으면 선언 순서에 매인다.** 파일 전체를 본 뒤
  해소해야 한다(독립 리뷰 R1 이 격리 파일로 잡았다).

## 능력 부재 — 이 회차가 겪은 것

- **표식 위치를 `grep` 으로 셌다**(착수 시점). 그래프가 Rust 를 못 읽어서였고,
  **이 회차가 그것을 고쳤다** — 지금은 `pal query symbol.contains` 가 답한다.

[#66]: https://github.com/hskim-ecoletree/palimpsest/issues/66
[#77]: https://github.com/hskim-ecoletree/palimpsest/issues/77
[#81]: https://github.com/hskim-ecoletree/palimpsest/issues/81
