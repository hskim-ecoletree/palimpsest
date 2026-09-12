# 음성 대조 — `A1`·`A1-b`·`A1-c` ⟨`A1-a`⟩

> 회차 `2026-09-11-effect-confirmation` · 잰 날 2026-09-12 · 재는 자리 **㉠ `#79`**
> 조건 문면: *"사본 **넷**에서 판정이 각각 뒤집히는지 재고 산출을 `effect/negative-A1.md`
> 로 남긴다 … **하나라도 안 뒤집히면 그 축의 조건이 항등식이고, 한 축만 초록이어도
> `A1-a` 는 불통과다**"*

## 검사를 먼저 세운다 — 그리고 원본에서 초록인지 본다

판정 절차를 뿌리를 인자로 받는 스크립트 하나로 세웠다. **뿌리가 컴파일 시점에 안 박히고
인자로 들어간다** — 규약 §7 이 못 박은 함정(*"사본으로 `cd` 해도 원본을 잰다"*)을 피한 자리다.

```bash
judge.sh <REPO> <ISSUE> <SEAL> <TOUCH> <READNOTE> <CHANGE> <프롬프트TIP>
  A1    git -C $REPO merge-base --is-ancestor $SEAL $TOUCH  가 0 이고  $SEAL != $TOUCH
  A1-b  touch/<이슈>.txt 머리의 `사전등록-blob`  ==  git rev-parse $SEAL:<plan 경로>
  A1-c  judge-prompt.md 의 ㉠ `## ① 사전 등록 전문` 절 바이트  ==  git cat-file blob <그 해시>
  A3    사전 등록에 `## N. 바꿀 좌표와 각 자리에 무엇을 쓰나` 절이 있고 `파일:줄` 이 1 건 이상
  A5    is-ancestor $TOUCH $CHANGE 가 0 이고  $TOUCH != $CHANGE
  A7    is-ancestor $TOUCH $READNOTE · is-ancestor $READNOTE $CHANGE · 셋 SHA 가 서로 다름
```

⚠ **검사가 실패할 수 있는 길이 있는지를 먼저 봤다.** 인자를 잘못 준 첫 실행에서 여섯이
전부 `FAIL` 로 났다(빈 SHA 를 받고 `git merge-base` 가 rc=129 를 냈다). **이 검사는
조용히 통과하지 않는다.**

## 사본 — 격리

`git clone` 으로 떴고 **디렉터리 이름을 `palimpsest` 로 뒀다**(이름이 `RepoId` 의 성분이라
다른 이름이면 심볼 id 가 통째로 어긋난다 ⟨`PM1-19`⟩). `cp -R` 이 아니라 `clone` 이라
이력이 산다 ⟨규약 §7⟩. 가지 뿌리는 `ac51103`(= `77dc977^`)이고, 같은 blob 을 순서만 바꿔
다시 커밋했다 — **파일 내용은 원본과 바이트로 같다.**

## 판정 — 넷이 각각 뒤집혔다

| 사본 | 무엇을 비틀었나 | 노린 축 | 결과 |
|---|---|---|---|
| **①** `neg1` | touch 를 **먼저** 커밋하고 봉인을 뒤에 | `A1` | **뒤집혔다** — `FAIL A1` (`is-ancestor rc=1`) |
| **②** `neg2` | 봉인 커밋에는 뼈대만 넣고 **내용을 뒤에 채웠다** | `A1-b` | **뒤집혔다** — `FAIL A1-b` (선언 `9b0f3a5a…` ↔ 실제 `98f0e807…`) |
| **③** `neg3` | 봉인과 touch 를 **같은 커밋**에 | `A1` | **뒤집혔다** — `FAIL A1` (`seal == touch == e62ddbd1…`) |
| **④** `neg4` | `judge-prompt.md` 의 ㉠ 인라인 사전 등록에서 **한 바이트**를 고쳤다(`:486` 끝 글자) | `A1-c` | **뒤집혔다** — `FAIL A1-c` (인라인 ↔ blob 이 2 줄 다르다) |

★ **한 바이트가 잡힌다.** ④ 는 152 줄 중 한 줄의 마지막 한 글자만 바꿨고 검사가 그것을
집었다 — `A1-c` 가 *"바이트로 같다"* 를 글자 그대로 재고 있다.

★ **딸려 나온 것 둘** — ② 에서는 `A1-c` 와 `A3` 도 함께 빨개졌다(봉인 blob 이 뼈대라
인라인과 153 줄 다르고 좌표절이 없다). 노린 축 밖이지만 **같은 방향**이라 그대로 적는다.

## `A1-a` 의 판정 — **통과**

넷이 각각 뒤집혔고 안 뒤집힌 축이 없다. 문면의 불통과 조건(*"하나라도 안 뒤집히면"* ·
*"한 축만 초록이어도"*)에 안 걸린다.

## 전 출력

```
### 원본 — ㉠ #79 · seal=77dc977 touch=88dbe13 readnote=f60df1d change=9f993cc · 프롬프트 TIP=0e1a876
PASS  A1
PASS  A1-b
PASS  A1-c
PASS  A3
PASS  A5
PASS  A7

### 사본 neg1 — seal=64036559e39c touch=549d98512b0f readnote=ded8de54e090 change=6c3a738f706a
FAIL  A1  — is-ancestor rc=1 · seal=64036559e39c9f1e28da3b0edceb7c1cb26bef33 touch=549d98512b0fff54e67a5386146083b517466c4a
PASS  A1-b
PASS  A1-c
PASS  A3
PASS  A5
PASS  A7

### 사본 neg2 — seal=90fc45e75a6c touch=254094495c9b readnote=9a1596659185 change=327fd5bd916b
PASS  A1
FAIL  A1-b  — 선언=9b0f3a5a87f60f02e43daf9e2fb49455a5c31466 실제=98f0e807a14871cb57f5ebf31a1b6b5e8c670bde
FAIL  A1-c  — 인라인 ↔ blob 이 153 줄 다르다
FAIL  A3  — 좌표절 0 · 파일:줄 0 건
PASS  A5
PASS  A7

### 사본 neg3 — seal=e62ddbd102c4 touch=e62ddbd102c4 readnote=25346acff5c5 change=24ca9457f32d
FAIL  A1  — is-ancestor rc=1 · seal=e62ddbd102c48898679237f99fcb9cb13a42fd2d touch=e62ddbd102c48898679237f99fcb9cb13a42fd2d
PASS  A1-b
PASS  A1-c
PASS  A3
PASS  A5
PASS  A7

### 사본 neg4 — seal=23b6ac8f2c16 touch=22cea5d0e797 readnote=9fbdca109ef8 change=acbe505f4b8a
PASS  A1
PASS  A1-b
FAIL  A1-c  — 인라인 ↔ blob 이 2 줄 다르다
PASS  A3
PASS  A5
PASS  A7

```
