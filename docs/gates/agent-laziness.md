# 게이트 — 에이전트 게으름 (회차 `2026-08-22-agent-laziness`)

> 착수 `e45e822` · 잠긴 의도는
> [`intent.md`](../../.palimpsest/rounds/2026-08-22-agent-laziness/intent.md) 의 `## 완수 조건`
> 계획 판 4 는 [`plan.md`](../../.palimpsest/rounds/2026-08-22-agent-laziness/plan.md)

## 합격선

**측정 전에 등록한다.** 등록된 조건은 `intent.md` 의 `## 완수 조건` A~K 열한 묶음이고
**이 문서가 그것을 복제하지 않는다** — 두 곳에 적으면 그것이 곧 drift 다.
세는 자리: `python3 .claude/skills/round/bin/record.py conditions <intent.md>`.

### RED — 착수 시점에 실제로 관측한 것

```
열린 완수 조건        90 (finding-records 46 + rust-extractor 44)
계기판 ② 미판정 잔액   44 / 44        게이트 판정 표는 「통과 43」이라 적는다
훅 EVENTS             ["SubagentStop"] 하나
완수 조건의 CHECK:     0 (회차 다섯 전부)
게이트→intent 링크     2 — 셋째(`rust-extractor.md`)가 없다
계기판 빈 범위         ②③④⑤⑥ 이 아예 안 뜬다
계기판 파서            함정 파일에서 `3 / 5` (실제 `1 / 2`)
```

### 음성 대조 — 검사가 고장이면 이렇게 드러난다

- **21 번째 검사**: 격리 사본에서 다섯이 발화해야 한다 — ① ID 삭제 ② 통과→반증
  ③ 상자 끄기 ④ 최근 회차를 형식 이전으로 ⑤ 게이트가 적은 수와 ID 목록 길이 불일치
- **조건 파서**: 뒤집힌 태그 · 켜졌는데 태그 없음 · 안 켜졌는데 태그 · ID 없음이 발화
- **계기판**: 빈 범위면 「못 셌다」, 안 비면 수 — **양방향**을 다 재야 한다
- **Stop 관측**: 같은 설정으로 `SubagentStop` 이 **먼저 먹어야** 한다. 안 먹으면
  「안 먹는다」와 「설정이 틀렸다」가 구별 불가다
- **§11 ③ 좁힘**: 별도 목록이 비면 안 가른 것 · 본 모집단이 비면 너무 센 것

### 퇴로 — 등록해 두면 쓸 때 승인이 필요 없다

`intent.md` 의 `## 퇴로` 다섯. 이 문서가 복제하지 않는다.

## 판정

<!-- 표준 표는 종료 직전에 선다 — 아래 근거 칸이 먼저 찬다 -->

### 근거 — **무엇을 돌려 그 판정이 났나**

수는 여기 안 적는다. **돌리는 명령**을 적는다.

| 묶음 | 무엇을 돌렸나 |
|---|---|
| **A** 계기판 파서 | `python3 .claude/skills/round/bin/dashboard.py e45e822 <intent.md> HEAD` — 빈 범위와 안 빈 범위 **양방향**. 시험은 `cargo test -p pal-cli --test round_scripts_run` 의 `계기판이_빈_범위에서_칸을_안_삼킨다` |
| **B** 조건·태그 파서 | `python3 .claude/skills/round/bin/record.py conditions <intent.md>` · **`git diff --diff-filter=A --name-only e45e822..HEAD -- .claude/`** (새 파일 0 — ⚠ `git ls-files` 는 스냅숏이라 「새 파일 0」을 **원리상 못 말한다**. 독립 리뷰 R1 이 잡았다) · `xtask` 는 `파서에_묻는다` 로 위임한다 |
| **C** 게이트 표준 표 | `git diff e45e822..HEAD -- docs/gates/README.md .claude/skills/round/SKILL.md docs/gates/rust-extractor.md **docs/gates/round-finding-records.md**` (C2 는 게이트 **둘 다** 대야 한다 — 앞 판이 하나를 빠뜨렸다) · `python3 … record.py --schema` 의 `게이트파서` |
| **D** 근거 칸 | 이 절 자체 |
| **E** 21 번째 검사 | `cargo xtask check` · RED 는 [`red/e8-red-observed.txt`](../../.palimpsest/rounds/2026-08-22-agent-laziness/red/e8-red-observed.txt) · 음성 대조 다섯은 [`red/e9-negative-controls.txt`](../../.palimpsest/rounds/2026-08-22-agent-laziness/red/e9-negative-controls.txt) |
| **F** Stop 관측 | [`effect/stop-hook-observed.txt`](../../.palimpsest/rounds/2026-08-22-agent-laziness/effect/stop-hook-observed.txt) — 격리 디렉터리에서 `claude -p` 두 번. `git status .claude/` · `git diff crates/pal-cli/src/hook/policy.rs` · `find .palimpsest/rounds -name '*.json'` |
| **G** 스키마 3 열림 축 | `python3 … record.py check .palimpsest/rounds/*/findings.jsonl **.palimpsest/rounds/*/*/disposal-overrides.jsonl**` — 앞 glob 은 예외표를 안 집어 **G6 에 대해 아무것도 안 냈다**(독립 리뷰 R1) · 계기판 ⑨ · 음성 대조는 [`red/g-negative-controls.txt`](../../.palimpsest/rounds/2026-08-22-agent-laziness/red/g-negative-controls.txt) |
| **H** 전사 | `python3 … record.py conditions` 를 두 회차에 · `python3 … record.py gate docs/gates/{round-finding-records,rust-extractor}.md` · 계기판 ② 를 두 회차에 |
| **I** R1 문면 전수 | [`review/r1-unlazy-line-by-line.md`](../../.palimpsest/rounds/2026-08-22-agent-laziness/review/r1-unlazy-line-by-line.md) — 규범 문장 **69** 전수. 고른 다섯과 기각 근거는 아래 |
| **J** 손으로 벤 수 | `grep -rn '검사 20\|지금 20' .github .claude docs` (0 건) · `git diff` 로 §9 명령 범위 |
| **K** 종료 | 이 문서 · `report.md` · `gh issue list` · `gh run list` |

### I3 — unlazy 에서 **가져오기로 고른 다섯** · 기각한 열하나

원 의도의 후반절(*"unlazy 에서 가져올 것을 **실측으로 정한다**"*)이 여기서 닫힌다.
모집단: `unlazy/SKILL.md`(`ed9e8d2` · v2.0.0) 본문 **98 문장 중 규범 문장 69**.
판정 분포 **있음 16 · 더 강함 17 · 약함 14 · 없음 22**.
⚠ **`references/`·`templates/`·`scripts/` 는 모집단 밖이다** — 그 파일들에만 있는 규범은 안 봤다.

| | 가져오는 것 | 어디 | 왜 |
|---|---|---|---|
| ① | **공격선 전환의 문턱** — 접근을 바꾸기 전에 «지금 것이 아직 낼 것» + «바꾸는 편이 나은 까닭»을 말한다 | §5 막힘 앞 | 이 회차의 「실패한 접근 여덟」 중 **넷을 실제로 밟았다**. §5 막힘은 *그만둘 상한*, 이것은 *그만두기 위한 조건* |
| ② | **통과한 판정 하나를 적대적으로 재검** | §7 | `rust-extractor.md` 가 스스로 적었다 — *"그 수정을 다시 잰 리뷰는 없다"* |
| ③ | **전수 훑기 + 훑은 수 검산 + 표본 선언** | §7 · §9 | 이 회차의 R1 이 정확히 그것을 필요로 했다. §7 의 *"모집단이 비면 실패"* 가 절반을 지고 **나머지 절반이 검산**이다 |
| ④ | **「끝난 느낌」을 방아쇠로 — 결론 대신 검사** | §11 | 가장 싸다. §11 이 **바깥에서** 끝을 정하고 이것은 **안에서** 결론을 미룬다 |
| ⑤ | **값싼 모델 라우팅** | 오케스트레이션 표 | 이 회차가 서브에이전트를 셋 썼고 그 축이 표에 없다 |

**기각한 것 열하나**(근거는 [#88] 에 전문): Depth Tree 전체 · 「개선 없는 패스 = 종료선」
(**B 가 실측으로 반증했다** — dry 0 회) · 모드 선택 기준(§5 교대와 정면 충돌) ·
무료 광택(§11② 「미관 → 닫을 수 있다」와 충돌) · `CHECK`/`EXPECT`/`EVIDENCE` 토큰
(**승격 4 가 이미 판정**) · 전문가 재독(독립 리뷰가 그 자리를 진다) · 부모의 검사 재실행
(합계 검산의 둘째 원천이 답한다) · 하네스 개입 고지 · 결함 사냥 모집단 넷 ·
「다음 미판정 조건을 집는다」 · 한 줄 수정 면제.

★ **기입은 이 회차가 안 한다** — 승격 5(*"고르는 것까지 · 기입은 다음"*). [#88] 이 진다.

### ⚠ 이 게이트가 **주장하지 않는 것** (D2 · D3)

**「재현 가능하게 적혔다」라고 주장하지 않는다.**

위 근거 칸은 **명령이 적혀 있다**는 것만 보인다. 그 명령이 **그 판정을 실제로 내는지**는
아무 검사도 안 읽는다 — 21 번째 검사는 ID 집합과 판정과 검산을 볼 뿐이다.

★ **그렇게 주장하면 unlazy `tail()` 결함과 같은 편이다.** 그 결함은 `EXPECT` 가 맞은
줄이 아니라 **마지막 두 줄**을 증거로 박아, 증거가 판정을 재현하지 못하면서도 형식은
갖춘 것이다. 형식이 채워졌다는 것과 증거가 증거라는 것은 다른 자다.

★ **옳은 명령인지의 판정은 독립 리뷰가 진다.** 기계는 그 자리를 원리상 못 진다.

## 효과

**CI 가 안 돌리는 것**이 돌린 출력. `cargo xtask check` 는 [`ci.yml`](../../.github/workflows/ci.yml)
이 돌리므로 **여기 못 쓴다** — 이 회차가 그 자리를 두 판 연속 밟았다.

### ① 격리 사본에서 음성 대조가 발화했다 — CI 는 흠집 낸 사본을 안 돌린다

[`red/e9-negative-controls.txt`](../../.palimpsest/rounds/2026-08-22-agent-laziness/red/e9-negative-controls.txt)
· [`red/g-negative-controls.txt`](../../.palimpsest/rounds/2026-08-22-agent-laziness/red/g-negative-controls.txt)

기준선 **21/21 초록** 위에 하나씩 흠집을 냈고 다섯이 각각 발화했다 — ID 삭제 ·
통과→반증 · 상자 끄기 · 최근 회차를 형식 이전으로 · 검산 수 불일치. G 쪽에서 셋 더:
`add` 가 옛 스키마 파일을 거부 · 시점 없는 닫힘이 형식 오류 · **계기판 ⑨ 가 「막힘」을 낸다.**

### ② 계기판을 이 회차의 `intent.md` 에 직접 댔다 — CI 는 **이 파일들에** 안 댄다

```
$ python3 .claude/skills/round/bin/dashboard.py e45e822 \
    .palimpsest/rounds/2026-08-22-agent-laziness/intent.md HEAD
```

전문은 [`effect/dashboard-on-this-round.txt`](../../.palimpsest/rounds/2026-08-22-agent-laziness/effect/dashboard-on-this-round.txt).

⚠ **사유를 정정한다** (독립 리뷰 R1 · N2). 앞 판은 *"CI 는 `dashboard.py` 를 안 부른다"*
라고 적었고 **그것은 거짓이다** — `ci.yml` 의 `cargo xtask test` 가 `round_scripts_run.rs`
를 통해 부른다 (세는 자리: `grep -c dashboard.py crates/pal-cli/tests/round_scripts_run.rs`). 무너진 것은 사유 문장이고 **효과 자체는 선다**: CI 가 부르는
것은 **합성한 함정 파일**이지 이 회차의 실제 `intent.md`·`findings.jsonl` 이 아니다.

### ③ `Stop` 훅이 실제로 막는지 태웠다 — CI 는 `claude -p` 를 안 돌린다

[`effect/stop-hook-observed.txt`](../../.palimpsest/rounds/2026-08-22-agent-laziness/effect/stop-hook-observed.txt)

**먹는다.** 그리고 **같은 `settings.json` 으로 `SubagentStop` 도 막혔다**(양성 대조) —
그것이 없으면 「Stop 이 안 받는다」와 「내 설정이 틀렸다」가 구별 불가다.

[#88]: https://github.com/hskim-ecoletree/palimpsest/issues/88

## 범위 밖

`intent.md` 의 `## 범위 밖` 아홉. 이 문서가 복제하지 않는다.
