# 음성 대조 — `A7` ⟨`A7-a`⟩

> 회차 `2026-09-11-effect-confirmation` · 잰 날 2026-09-12 · 재는 자리 **㉠ `#79`**
> 조건 문면: *"사본 **둘**에서 판정이 **뒤집히는지** 재고 `effect/negative-A7.md` 에 둘 다
> 남긴다 — ⓐ `readnote` 커밋을 **변경 커밋 뒤로** 옮긴 사본에서 `A7` 이 빨개진다 ·
> ⓑ `readnote` 커밋을 **touch 커밋 앞으로** 옮긴 사본에서 `A7` 이 빨개진다 …
> 한 축만 초록이어도 `A7-a` 는 **불통과**다"*

★ **`A7` 은 경계를 둘 건다** — touch 커밋 **뒤**이고 변경 커밋 **전**이다. 두 사본이
경계 하나씩을 친다. 검사와 격리 조건은 `effect/negative-A1.md` 가 진다.

## 축 ⓐ — 뒤쪽 경계(변경 커밋 **전**)

사본 `neg6`: 봉인 → touch → **변경** → readnote.

```
FAIL  A7  — touch=d8a6300e6157… readnote=0c0142842f73… change=7eedb3fd4026…
```

**뒤집혔다.** `readnote` 가 변경의 조상이 아니다.

## 축 ⓑ — 앞쪽 경계(touch 커밋 **뒤**)

사본 `neg7`: 봉인 → **readnote** → touch → 변경. 산출이 서기 전에 「봤다」를 적은 것이라
인과가 뒤집힌다.

```
FAIL  A7  — touch=2bbf9a45ddc1… readnote=c5fb4956a086… change=b189144bfa32…
```

**뒤집혔다.** `touch` 가 `readnote` 의 조상이 아니다.

★ **두 사본에서 `A1`·`A1-b`·`A1-c`·`A3`·`A5` 는 전부 초록으로 남았다.** 대조가 노린 축
하나만 친다 — 검사가 아무 데서나 빨개지는 것이 아니라는 반대 방향의 증거다.

## `A7-a` 의 판정 — **통과**

두 경계가 각각 뒤집혔고 안 뒤집힌 축이 없다. 앞 판의 공백(*"대조가 뒤쪽만 뒤집었다"*)은
⟨승인 앞 정정 · `MS-05`⟩ 이 축 ⓑ 를 더해 메웠고, 이 문서가 그 축을 실제로 돌렸다.

## 전 출력

```
### 사본 neg6 — seal=255041b6f6c3 touch=d8a6300e6157 readnote=0c0142842f73 change=7eedb3fd4026
PASS  A1
PASS  A1-b
PASS  A1-c
PASS  A3
PASS  A5
FAIL  A7  — touch=d8a6300e615771b8b647564faae9e6f1e2506018 readnote=0c0142842f7373ef63e4e7264594af17368849e9 change=7eedb3fd40260ccd3cd26252613f5359f6f0877c

### 사본 neg7 — seal=14de9a2ffbd3 touch=2bbf9a45ddc1 readnote=c5fb4956a086 change=b189144bfa32
PASS  A1
PASS  A1-b
PASS  A1-c
PASS  A3
PASS  A5
FAIL  A7  — touch=2bbf9a45ddc102dc1c73dc6e9cc23a508b38d420 readnote=c5fb4956a08602069c4c44c71f16f09e209bd7fd change=b189144bfa32e2d21a667dc2656f7adf94c432cd

```
