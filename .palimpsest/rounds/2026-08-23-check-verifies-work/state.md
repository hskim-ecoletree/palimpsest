# 상태 — 검사가 「했나」를 안 본다

> 교대용. 새 컨텍스트에 주는 것은 **잠긴 의도 전문 + 이 요약**이다.
> **직전 산출물을 시드로 주지 않는다.**

## 지금 단계

**사전부검** (상한 3 라운드)

## 상한과 소모

| 계열 | 상한 | 쓴 것 |
|---|---:|---:|
| 인터뷰 | 4 | **3 — 닫혔다** (R3 에서 소유자가 판단을 위임했다) |
| 사전부검 | 3 | 0 |
| 독립 리뷰 | 8 | 0 |
| 사후 검증 | 1 (상한 밖) | 0 |

## 착수 좌표

- 착수커밋 `b9fbef7`
- 회차 디렉터리 `.palimpsest/rounds/2026-08-23-check-verifies-work/`
- 게이트(예정) `docs/gates/check-verifies-work.md`

## 만질 자리

| 파일 | 무엇 |
|---|---|
| `xtask/src/main.rs` | `check_round_records`(합계 검산 · 「했나」 축) · `check_ledger_pair`(모집단 자격 · `report.md`) |
| `.claude/skills/round/bin/record.py` | `--schema` 파서 사전(#94) · 열림축 검증 |
| `.claude/skills/round/SKILL.md` | 규약 문면 — 반환 형식 · 「했나」 축 · 음성 대조 규율 |
| `.claude/agents/pal-independent-reviewer.md` | 「없음」을 표 밖 문장으로(#93) · 반환 형식 규율 |
| `.claude/agents/pal-premortem-sweeper.md` | 반환 형식 규율 |
| `docs/gates/README.md` | 「형식 이전」 닫힌 선언 목록(#90) |

## 실패한 접근

- **「끝난 회차 = 모집단」을 그대로 걸기.** `report.md` 가 일곱 중 여섯에 있어서(RED-7)
  옛 게이트 둘이 통째로 실패한다. → 닫힌 예외 선언으로 우회했다 (`C2`).

## 남은 것

- 사전부검 3 라운드 → 처분
- 실행: A·B·C·D·E·F·G 클러스터
- 음성 대조 열셋을 **전부 건다** (`F2`·`F3`)
- 독립 리뷰 8 라운드
- 효과(§8): 실제 거짓 닫힘이 빨개지는 것 · R4 의 「다른 20」이 빨개지는 것
- 종료 보고 `report.md`
