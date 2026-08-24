# 교대 — **검사가 「했나」를 안 본다** 회차를 이어받는다

> 작성 2026-08-24 · 브랜치 `round/agent-laziness` · 이 회차는 **진행 중**이다
> 회차 디렉터리 `.palimpsest/rounds/2026-08-23-check-verifies-work/`

★★ **규약 §교대 — 새 컨텍스트에 주는 것은 「잠긴 의도 전문 + `state.md`」다.**
직전 산출물을 시드로 주지 않는다. 이 문서도 **결론을 안 준다** — 어디를 읽고
무엇을 돌릴지만 준다.

> 의도가 변질되는 기제는 쪼개는 것 자체가 아니라, **다음 걸음의 입력이 원 목표가
> 아니라 직전 걸음의 산출물일 때**다.

---

## 다른 PC 에서 착수하는 절차

```bash
git clone https://github.com/hskim-ecoletree/palimpsest.git
cd palimpsest
git checkout round/agent-laziness
git pull

# ⚠ **이력 전부가 필요하다** — 검사 22 가 회차 레코드의 `닫은커밋` 을 `git show` 로
#    읽는다. 얕은 클론에서는 그 SHA 가 하나도 안 읽혀 전량이 빨개진다.
git rev-list --count HEAD    # 얕으면 `git fetch --unshallow`

cargo xtask check            # 여기가 초록이어야 착수 상태다
cargo test -p xtask
```

**파이썬은 `python3` 이면 된다.** 검사가 `PYTHONUTF8=1` 을 스스로 박는다.

---

## 새 세션 첫 메시지 — **그대로 붙인다**

```
/round

**진행 중인 회차를 이어받는다.** 새 회차가 아니다 —
`.palimpsest/rounds/2026-08-23-check-verifies-work/` 가 열려 있다.

## 먼저 읽어라

1. `.palimpsest/rounds/2026-08-23-check-verifies-work/intent.md` — **잠긴 의도 전문.**
   ★★ `## 개정` 이 정본이다. `## 잠근 결정` 표와 갈리면 개정이 이긴다.
   사전부검 셋과 독립 리뷰 셋이 **여섯 번** 고쳤다(①~㉮). 완수 조건 **60** 개.
2. `.palimpsest/rounds/2026-08-23-check-verifies-work/state.md` — **교대용 상태.**
   상한 소모 · 지금까지 선 것 · **실패한 접근**(같은 벽에 다시 부딪히지 마라) ·
   남은 것 · 처분 이력.

**그 둘 말고는 시드로 받지 마라.** 앞 라운드의 반환문도 내 서사도 안 받는다 —
필요하면 직접 읽어라. 그것이 §교대가 정한 것이다.

## 지금 어디까지 왔는지는 돌려서 안다

⚠ **수를 여기 안 적는다** — 개정 ㉛ 이 「글은 낡고 수는 안 낡는다」로 못 박았고,
이 회차가 그 자리에서 **두 번** 낡은 수를 적었다.

    cargo xtask check
    cargo test -p xtask
    python3 .claude/skills/round/bin/record.py conditions \
      .palimpsest/rounds/2026-08-23-check-verifies-work/intent.md
    python3 .claude/skills/round/bin/record.py count \
      .palimpsest/rounds/2026-08-23-check-verifies-work
    gh run list --branch round/agent-laziness --limit 3
    gh issue list --state open

## 남은 것

`state.md` 의 `## 남은 것` 이 정본이다. 큰 덩이는 넷:

1. **독립 리뷰 R4~R8** — 상한 8 중 셋을 썼다. ⚠ **상한을 다 쓴다** — 소유자 결정이고,
   앞 회차에서 **마지막 라운드까지 무언가를 잡았다.** 지금 회차도 R1·R2·R3 이
   각각 금지역을 냈다.
2. **게이트** `docs/gates/check-verifies-work.md` — §9 의 절 이름 넷. 조건 60 의
   표준 표. `A4`·`A10`·`G3` 가 그것을 기다린다.
3. **종료 보고** `report.md` — §10 의 다섯 절. **네 이름이 없어야 한다.**
4. **결박·그래프**(`K4`) · **착수 재료 삭제**(`K8`) · **마지막 커밋의 CI**(`K5`).

## ⚠ 이 회차가 자기 몸으로 겪은 것 — 반드시 알아라

**검사가 이 세션을 두 번 잡았다.**

- 레코드에 *"규약을 정정한다"* 고 적어 놓고 **안 고쳤다.** A 축이 빨갛게 만들었다.
- `state.md` 를 고친 커밋을 **한 칸 잘못 적었다.** 또 빨갛게 만들었다.

**착수 시점에는 둘 다 조용했다** — `cargo xtask check` 가 21/21 이었고 `닫은커밋` 은
어느 검사의 모집단에도 없었다. **그것이 이 회차가 닫으려는 것이다.**

그리고 **내가 `state.md` 에 적은 거짓 한 문장이 조건 셋을 「미측정」으로 묶어 뒀다** —
*"round 브랜치는 push 로 CI 가 안 돈다"* 는 거짓이다. **PR #91 이 열려 있고**
워크플로가 `pull_request` 에 걸려 있다. push 하면 세 OS 에서 돈다.

## 하지 말 것

- **앞 회차(`2026-08-23-agent-laziness-behavior`)의 판정을 다시 열지 마라.**
- **`intent.md` 의 `## 원문` 을 고치지 마라.** 절대 안 바뀐다.
- **수를 문서에 적지 마라.** 세는 자리는 명령이다.
- **「앞 라운드가 잡은 것」을 리뷰어에게 다시 내게 하지 마라** — 프롬프트에 요지를
  적어 막아라. 안 막으면 같은 것이 되돌아온다.
- **격리 사본을 `cp -R` 로 뜨지 마라.** `git clone --no-hardlinks` 여야 이력이 따라온다.
  그리고 **`--root <사본>`** 을 주거나 사본에서 재빌드해라 — 안 그러면 원본을 잰다.

## 상한

`state.md` 의 표가 정본이다. **승격해도 라운드 셈은 리셋하지 않는다.**
```

---

## 이 문서를 지우는 때

**회차가 닫힐 때다.** 착수 재료(`NEXT-E-*.md`)와 함께 지운다 — 그것이 `K8` 이다.
