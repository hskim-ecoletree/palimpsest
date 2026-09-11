# `A5-c` 팔 ① — 「워킹트리」 용법 **전수**

> 정반합 라운드 1 · 판 2 의 `[판2·O4]` 를 **이번에 늘린 것**이다.
> 합(合)의 확장 판단 원문 — *"팔 ③ 이 미도래로 내려가 팔 ① 이 판정을 지는 축이 됐다.
> 원 의도(「증거로 안 쓴다」)의 **완결에 필요한 것**이지 새 측면이 아니다"*.
> 초안(`dialectic/2-thesis.md:27-46`)은 적중 **93** 줄 중 **36** 좌표만 짚고 스스로 「전수」라
> 불렀다. 여기서 **93 줄 전량**을 가른다.

## 모집단 — 어디서 왔나

`dialectic/preflight.md:94-230` 에 보존된 적중 목록이다. 그 명령:

```
$ grep -rn '워킹트리' .palimpsest/rounds/2026-09-11-effect-confirmation docs/gates/effect-confirmation.md
```

- 적중 줄 **135**
- 그중 **자기참조 42**(`dialectic/preflight.md:` 로 시작하는 줄 — preflight 가 자기 본문을 다시 집는다)
- **모집단 93**

⚠ **판정 중에 생긴 파일은 모집단이 아니다.** `2-design.md:73-74` 가 preflight 를 **설계된
공유 입력**으로 못 박았다. 2026-09-12 현재 같은 명령을 다시 돌리면 적중이 늘어나는데
(`dialectic/1-thesis.md` · `2-thesis.md` · `2-antithesis.md` · `2-synthesis.md` ·
`r1-reporter-run1.md`), 그 다섯은 전부 **이 판정 자체의 산출**이다.

## 갈래와 수 — **검산 20 + 3 + 3 + 33 + 10 + 24 = 93**

| 갈래 | 무엇인가 | 수 |
|---|---|---|
| **ⓐ** | 보존된 산출 **안**의 `워킹트리  일치` 줄 그 자체 — `A2` 가 바이트 보존을 요구한다. 지우면 오히려 조건 위반이다 | **20** |
| **ⓐ′** | 산출 머리의 「워킹트리 깨끗」 중 **출처를 명기한** 것(`git status --porcelain` 빈 출력) | **3** |
| **ⓑ** | **명시적으로 「증거로 안 쓴다」를 적은** 자리 | **3** |
| **ⓒ** | 조건·결함·설계를 **이름으로 부른** 자리 — 처분·설계 기록이지 사용이 아니다 | **33** |
| **ⓓ** | **하중을 지는 인용** — 「쓴 것일 수 있는」 자리 | **10** |
| **ⓕ** | **대상 밖** — `pal touch` 의 「워킹트리 일치」 줄이 아니다(워킹트리를 *상태*나 *다이제스트*로 말하거나, `pal ledger` 의 다른 줄이다) | **24** |
| | **합** | **93** |

★ **초안의 넷(ⓐ~ⓔ)에서 둘이 갈라졌다.**
- **ⓐ′** — 초안은 산출 머리의 「워킹트리 깨끗」을 ⓐ 에 섞었다. 그것은 **산출 바이트가
  아니라 내가 쓴 기입**이라 `A2` 의 보존 요구가 안 걸린다. 가른다.
- **ⓕ** — 초안에 없던 갈래다. grep 그물이 `pal touch` 의 그 줄이 **아닌** 것을 24 줄
  집는다. 조건 문면은 *"**`pal touch` 의** 「워킹트리 일치」 줄"* 로 표면을 명시했다.

⚠ **ⓔ(부재)는 이 표에 없다** — 적중이 아니라 **안 적힌 것**이라 grep 모집단에 원리상
안 들어온다. 초안이 든 ⓔ 한 자리(`effect/79-readnote.md` 가 `touch/79.txt:57` 을 언급조차
안 한다)는 그대로 서 있고, 이 전수가 그것을 뒤집지 않는다.

## ★ 전수가 산출한 것 — **「그 줄」을 논거로 끌어다 쓴 자리는 하나다**

ⓓ 10 줄을 다시 가르면 이렇다.

| 좌표 | 무엇을 인용하나 | 하중 |
|---|---|---|
| `effect/126-readnote.md:10` | `touch/126.txt:52` 의 **`워킹트리  일치` 줄 자체** | ★ **이것 하나뿐이다.** *"`+worktree` 가 붙은 것은 워킹트리 스냅숏이라는 뜻이고 `:52`(`워킹트리  일치`)와 짝이다"* — 같은 파일 `:18` 이 *"증거로 쓰지 않는다"* 라 적은 것과 **한 파일 안에서 어긋난다** |
| `touch/79.txt:9` | 머리의 「워킹트리 깨끗」 — 그 줄이 아니다 | 출처 **되살릴 수 없음**을 2026-09-12 에 기입했다 ⟨ⓓ2 처분⟩ |
| `touch/129.txt:11` | 같음 | 같음 |
| `effect/judge-prompt.md:649` | `touch/79.txt:9` 의 사본 | 원본과 같이 움직인다 |
| `effect/judge-prompt.md:1733` | `touch/129.txt:11` 의 사본 | 원본과 같이 움직인다 |
| `effect/rerun-79.txt:3` | *"같은 워킹트리(깨끗)에서"* — **상태** 주장 | `A2-b`(재실행 대조)의 전제라 하중을 진다. **그 줄의 인용이 아니다** |
| `effect/rerun-126.txt:3` | 같음 | 같음 |
| `effect/rerun-129.txt:3` | 같음 | 같음 |
| `observations/premortem-artifacts/README.md:21` | *"같은 HEAD·같은 워킹트리에서 두 번 돌린 산출이 바이트로 동일"* | `PM3-07` 의 전제. `rerun-*.txt:3` 과 **같은 문장 형식**이다 ⟨`[판2·O5]` 가 잡은 비대칭⟩ |
| `intent.md:182` | `A2-b` 조건 문면의 *"같은 HEAD·같은 워킹트리에서"* | 조건이 스스로 전제로 건다 ⟨`[판2·O10]`⟩ |

★★ **그래서 팔 ① 이 실제로 묻는 것이 하나로 좁혀진다** — `effect/126-readnote.md:10` 의
「짝이다」가 **상호참조**인가 **기댐**인가. 나머지 아홉은 「워킹트리 **깨끗**」이라는
*상태 주장*이지 `pal touch` 의 「워킹트리 **일치**」 줄의 인용이 아니다.

⚠ **이 파일은 판정하지 않는다.** 팔 ① 의 값은 정반합 라운드 2 가 정한다 — 재료만 놓는다.

## 전수 — 93 줄 전량


### ⓐ — 20 줄

```
touch/79.txt:57:  워킹트리  일치
touch/126.txt:52:  워킹트리  일치
effect/rerun-129.txt:20:  워킹트리  일치
touch/129.txt:30:  워킹트리  일치
effect/judge-prompt.md:228:  워킹트리  일치
effect/judge-prompt.md:697:  워킹트리  일치
effect/judge-prompt.md:1330:  워킹트리  일치
effect/judge-prompt.md:1752:  워킹트리  일치
effect/rerun-79.txt:52:  워킹트리  일치
effect/rerun-126.txt:51:  워킹트리  일치
observations/premortem-artifacts/grade.txt:14:  워킹트리  일치
observations/premortem-artifacts/clone-after.txt:42:  워킹트리  일치
observations/premortem-artifacts/nodes_of2.txt:44:  워킹트리  일치
observations/premortem-artifacts/of_normalized.txt:35:  워킹트리  일치
observations/premortem-artifacts/identity_ceiling.txt:35:  워킹트리  일치
observations/premortem-artifacts/check_intent_untouched.txt:38:  워킹트리  일치
observations/premortem-artifacts/container_chains.txt:37:  워킹트리  일치
observations/premortem-artifacts/clone-before.txt:42:  워킹트리  일치
observations/premortem-artifacts/nodes_of.txt:44:  워킹트리  일치
observations/premortem-artifacts/check_ledger_pair.txt:38:  워킹트리  일치
```

### ⓐ′ — 3 줄

```
touch/126.txt:9:# 종료값 0 · 표준오류 0 바이트 · 워킹트리 깨끗(`git status --porcelain` 빈 출력)
effect/judge-prompt.md:185:# 종료값 0 · 표준오류 0 바이트 · 워킹트리 깨끗(`git status --porcelain` 빈 출력)
effect/judge-prompt.md:1287:# 종료값 0 · 표준오류 0 바이트 · 워킹트리 깨끗(`git status --porcelain` 빈 출력)
```

### ⓑ — 3 줄

```
effect/129-readnote.md:15:| `:30` | `워킹트리 일치` | `A5-c` 가 잡아 둔 자리 — 증거로 쓰지 않는다 |
effect/129-readnote.md:26:- 나머지 줄은 전부 근거(스냅숏·대장·워킹트리)이고 이미 아는 값이거나 `MS-06`·`A5-c` 로
effect/126-readnote.md:18:| `:51-53` | `대장 parsed 141 … / 1258 파일` · `2층 심볼 3308 색인됨` · `워킹트리 일치` | `A5-c` 가 잡아 둔 자리 — **더러운 워킹트리에서도 `일치` 를 찍는다.** 지금은 실제로 깨끗하므로 이 줄을 증거로 쓰지 않는다 |
```

### ⓒ — 33 줄

```
findings.jsonl:58:{"id": "PM2-04", "라운드": 2, "출처": "사전부검", "모집단": "저장소", "유효성": "참", "해악도": "금지역", "처분": "정정", "경로": "crates/pal-cli/src/touch.rs", "요약": "pal touch 가 더러운 워킹트리에서 「워킹트리 일치」를 찍는다", "승격됨" …⟨줄임 — 원본은 그 좌표에 있다⟩
conditions-audit/r2-raw.md:154:- 어떻게 실패하나: 열거할 방법도, 댈 코드 좌표도 조건에 없다. ⟨결정론적⟩ 태그인데 확인 행위는 코드 독해다. ⟨전제 자체는 실측으로 참이었다 — 더러운 워킹트리에서도 `워킹트리 일치`, `--at` 을 붙이면 `워킹트리 다름`⟩
dialectic/4-design.md:23:| **판 2** | `A5-c` | 「워킹트리 일치」 줄을 증거로 **안 썼고**, 그 줄이 `--at` 없는 산출에서 **구조적으로 참**이라는 코드 독해 논증이 서며, 그 사실이 게이트에 적혔나 | 제품 코드 · 회차 산출물 전량 · 게이트 |
dialectic/4-design.md:66:| `A5-c` | 회차 산출물·게이트에서 「워킹트리」 문자열 전수(`rg`) · 게이트 파일의 존재와 바이트 | ① *"증거로 **안 썼다**"* 는 인용의 **용법** 판정 ② *"`--at` 없이 부른 **모든 산출**에서 **구조적으로 참**"* 은 **코드 독해 논증** ⟨`CA2-14`⟩ |
dialectic/4-design.md:78:- **판 2** — 회차 디렉터리와 `docs/gates/effect-confirmation.md` 에 대한 `rg -n "워킹트리"` **전량** · 아래 코드 다섯 자리의 원문 · 같은 줄을 찍는 다른 표면 목록(`crates/pal-cli/src/export.rs:376` · `crates/pal-cli/ …⟨줄임 — 원본은 그 좌표에 있다⟩
dialectic/4-design.md:104:- `crates/pal-cli/src/touch.rs:310-313` — 그 값이 `워킹트리  일치 / 다름` 으로 찍힌다
dialectic/4-design.md:131:| 판 2 | 코드 다섯 자리 원문 · `rg "워킹트리"` 전량 · 게이트 파일 바이트(또는 부재 산출) | 판 3·4 의 증거 묶음 |
dialectic/4-design.md:260:- **제품 결함의 수리** — 「워킹트리 일치」의 공허함(`A5-c`) · `크기` 줄(`MS-06`) ·
intent.md:191:- [ ] **A5-c** ⟨정반합⟩ **`pal touch` 의 「워킹트리 일치」 줄을 증거로 안 쓴다** ⟨`CA2-14` — *"모든 산출에서 구조적으로 참"* 은 열거가 아니라 코드 독해 논증이라 ⟨결정론적⟩이 아니다⟩ — 그 줄이 `--at` 없이 부른 모든 산출에서 **구조적으로 참**임을 확인하고 그 사실을 게이트에 적 …⟨줄임 — 원본은 그 좌표에 있다⟩
intent.md:313:  라 적어 **방향이 반대**다. `A5-c` 의 「워킹트리 일치」와 같은 부류 — **제품의 표시 결함이고
intent.md:350:| 2026-09-11 ⟨사전부검 R2·R3⟩ | 조건이 **43 → 46**. 넷을 세우고(`A5-b` 바이트 앵커 · `A2-b` 재실행 대조 · `A2-c` ㉢ 후보 목록 처분 · `A5-c` 워킹트리 줄 안 씀) `C2-c` 하나를 뺐다. `A6` 은 「보존본 하나」로 고쳤다 | 커밋 순서만으로는 인과가 안 선다(격리 사본  …⟨줄임 — 원본은 그 좌표에 있다⟩
dialectic/2-design.md:23:| **판 2** | `A5-c` | 「워킹트리 일치」 줄을 증거로 **안 썼고**, 그 줄이 `--at` 없는 산출에서 **구조적으로 참**이라는 코드 독해 논증이 서며, 그 사실이 게이트에 적혔나 | 제품 코드 · 회차 산출물 전량 · 게이트 |
dialectic/2-design.md:66:| `A5-c` | 회차 산출물·게이트에서 「워킹트리」 문자열 전수(`rg`) · 게이트 파일의 존재와 바이트 | ① *"증거로 **안 썼다**"* 는 인용의 **용법** 판정 ② *"`--at` 없이 부른 **모든 산출**에서 **구조적으로 참**"* 은 **코드 독해 논증** ⟨`CA2-14`⟩ |
dialectic/2-design.md:78:- **판 2** — 회차 디렉터리와 `docs/gates/effect-confirmation.md` 에 대한 `rg -n "워킹트리"` **전량** · 아래 코드 다섯 자리의 원문 · 같은 줄을 찍는 다른 표면 목록(`crates/pal-cli/src/export.rs:376` · `crates/pal-cli/ …⟨줄임 — 원본은 그 좌표에 있다⟩
dialectic/2-design.md:104:- `crates/pal-cli/src/touch.rs:310-313` — 그 값이 `워킹트리  일치 / 다름` 으로 찍힌다
dialectic/2-design.md:131:| 판 2 | 코드 다섯 자리 원문 · `rg "워킹트리"` 전량 · 게이트 파일 바이트(또는 부재 산출) | 판 3·4 의 증거 묶음 |
dialectic/2-design.md:260:- **제품 결함의 수리** — 「워킹트리 일치」의 공허함(`A5-c`) · `크기` 줄(`MS-06`) ·
dialectic/3-design.md:23:| **판 2** | `A5-c` | 「워킹트리 일치」 줄을 증거로 **안 썼고**, 그 줄이 `--at` 없는 산출에서 **구조적으로 참**이라는 코드 독해 논증이 서며, 그 사실이 게이트에 적혔나 | 제품 코드 · 회차 산출물 전량 · 게이트 |
dialectic/3-design.md:66:| `A5-c` | 회차 산출물·게이트에서 「워킹트리」 문자열 전수(`rg`) · 게이트 파일의 존재와 바이트 | ① *"증거로 **안 썼다**"* 는 인용의 **용법** 판정 ② *"`--at` 없이 부른 **모든 산출**에서 **구조적으로 참**"* 은 **코드 독해 논증** ⟨`CA2-14`⟩ |
dialectic/3-design.md:78:- **판 2** — 회차 디렉터리와 `docs/gates/effect-confirmation.md` 에 대한 `rg -n "워킹트리"` **전량** · 아래 코드 다섯 자리의 원문 · 같은 줄을 찍는 다른 표면 목록(`crates/pal-cli/src/export.rs:376` · `crates/pal-cli/ …⟨줄임 — 원본은 그 좌표에 있다⟩
dialectic/3-design.md:104:- `crates/pal-cli/src/touch.rs:310-313` — 그 값이 `워킹트리  일치 / 다름` 으로 찍힌다
dialectic/3-design.md:131:| 판 2 | 코드 다섯 자리 원문 · `rg "워킹트리"` 전량 · 게이트 파일 바이트(또는 부재 산출) | 판 3·4 의 증거 묶음 |
dialectic/3-design.md:260:- **제품 결함의 수리** — 「워킹트리 일치」의 공허함(`A5-c`) · `크기` 줄(`MS-06`) ·
dialectic/1-design.md:23:| **판 2** | `A5-c` | 「워킹트리 일치」 줄을 증거로 **안 썼고**, 그 줄이 `--at` 없는 산출에서 **구조적으로 참**이라는 코드 독해 논증이 서며, 그 사실이 게이트에 적혔나 | 제품 코드 · 회차 산출물 전량 · 게이트 |
dialectic/1-design.md:66:| `A5-c` | 회차 산출물·게이트에서 「워킹트리」 문자열 전수(`rg`) · 게이트 파일의 존재와 바이트 | ① *"증거로 **안 썼다**"* 는 인용의 **용법** 판정 ② *"`--at` 없이 부른 **모든 산출**에서 **구조적으로 참**"* 은 **코드 독해 논증** ⟨`CA2-14`⟩ |
dialectic/1-design.md:78:- **판 2** — 회차 디렉터리와 `docs/gates/effect-confirmation.md` 에 대한 `rg -n "워킹트리"` **전량** · 아래 코드 다섯 자리의 원문 · 같은 줄을 찍는 다른 표면 목록(`crates/pal-cli/src/export.rs:376` · `crates/pal-cli/ …⟨줄임 — 원본은 그 좌표에 있다⟩
dialectic/1-design.md:104:- `crates/pal-cli/src/touch.rs:310-313` — 그 값이 `워킹트리  일치 / 다름` 으로 찍힌다
dialectic/1-design.md:131:| 판 2 | 코드 다섯 자리 원문 · `rg "워킹트리"` 전량 · 게이트 파일 바이트(또는 부재 산출) | 판 3·4 의 증거 묶음 |
dialectic/1-design.md:260:- **제품 결함의 수리** — 「워킹트리 일치」의 공허함(`A5-c`) · `크기` 줄(`MS-06`) ·
plan/79-pre.md:91:**이 회차는 그것을 `## 범위 밖` 으로 처분한다** — `A5-c`(워킹트리 일치)·`MS-06`(크기 줄)과
effect/judge-prompt.md:574:**이 회차는 그것을 `## 범위 밖` 으로 처분한다** — `A5-c`(워킹트리 일치)·`MS-06`(크기 줄)과
premortem/r2-raw.md:40:### `pal touch` 가 더러운 워킹트리에서 「워킹트리 일치」를 찍는다
premortem/r2-raw.md:41:- 어떻게 실패하나: 같은 사본에서 `git status` 가 ` M xtask/src/main.rs` 인 상태로 touch 를 돌렸는데 근거 상자가 `워킹트리 일치`. `--at <sha>` 를 주면 `다름` 이 난다. 기본 경로는 워킹트리를 읽으므로 `matches_worktree` 가 **구조적으로 언제나 참* …⟨줄임 — 원본은 그 좌표에 있다⟩
```

### ⓓ — 10 줄

```
intent.md:182:- [ ] **A2-b** ⟨결정론적⟩ **재실행 대조가 선다** — 같은 HEAD·같은 워킹트리에서 그 명령을 다시 돌려 `diff` 가 **바이트로 동일**함을 보이고 산출을 `effect/rerun-<이슈>.txt` 로 남긴다. ⟨`PM2-10`·`PM3-07` — `CA1-17` 이 *"다시 돌려도 바이트가 안 맞는다"* 를 …⟨줄임 — 원본은 그 좌표에 있다⟩
touch/79.txt:9:# 종료값 0 · 표준오류 0 바이트 · 워킹트리 깨끗 · 산출 52 줄 · **4066 바이트**
effect/rerun-129.txt:3:# 같은 HEAD(`22cf488`) · 같은 워킹트리(깨끗)에서 두 번째 실행. **바이트로 동일하다.**
touch/129.txt:11:# 종료값 0 · 표준오류 0 바이트 · 워킹트리 깨끗 · 산출 21 줄 · **1201 바이트**
effect/judge-prompt.md:649:# 종료값 0 · 표준오류 0 바이트 · 워킹트리 깨끗 · 산출 52 줄 · **4066 바이트**
effect/judge-prompt.md:1733:# 종료값 0 · 표준오류 0 바이트 · 워킹트리 깨끗 · 산출 21 줄 · **1201 바이트**
effect/126-readnote.md:10:| `:16` | `check_ledger_pair · palimpsest@a74737b+worktree#076728deab4f` | 심볼 ID 가 **봉인 커밋의 트리**에 대해 섰다. `+worktree` 가 붙은 것은 워킹트리 스냅숏이라는 뜻이고 `:52`(`워킹트리  일치`)와 짝이다 |
effect/rerun-79.txt:3:# 같은 HEAD(`77dc977`) · 같은 워킹트리(깨끗)에서 같은 명령을 **두 번째로** 돌린 산출이다.
effect/rerun-126.txt:3:# 같은 HEAD(`a74737b`) · 같은 워킹트리(깨끗)에서 같은 명령을 **두 번째로** 돌린 산출이다.
observations/premortem-artifacts/README.md:21:| `nodes_of.txt` · `nodes_of2.txt` | **`PM3-07`** — 같은 HEAD·같은 워킹트리에서 두 번 돌린 산출이 **바이트로 동일**하다. `CA1-17` 의 근거가 거짓임을 보여 `A2-b`(재실행 대조)가 실제로 가능해졌다 |
```

### ⓕ — 24 줄

```
conditions-audit/r1-raw.md:179:- 어떻게 실패하나: 손으로 한 줄 더한 산출을 잡는 조건이 없고, 재실행 대조도 원리상 좁다 — 실측한 산출 머리가 `palimpsest@4ab56d0+worktree#a9dd338ebdd7` 로 **워킹트리 다이제스트**를 실어 같은 SHA 로 다시 돌려도 바이트가 안 맞는다. `A1-b` 는 계획 …⟨줄임 — 원본은 그 좌표에 있다⟩
conditions-audit/r2-raw.md:44:- 어떻게 실패하나: 격리 사본에서 재현했다. 본문을 한 줄로 병합하고 `// AUDIT PROBE` 를 넣어도 `body c5cfb1f369bc` 가 그대로였고(인덱스·캐시를 **지우고 다시 세운 뒤에도** 같았다), `self.ordinal == 0` → `== 7` 로 바꾸자 `d6f47e7b25a …⟨줄임 — 원본은 그 좌표에 있다⟩
conditions-audit/r2-raw.md:200:- 획득: 실측 — 두 번 돌려 `diff` 한 결과가 **바이트로 동일**했다. `#<hex>` 는 `crates/pal-core/src/coord.rs:294-297` 의 `Display` 가 `self.symbol.short()` 로 찍는 **심볼 ID** 이지 워킹트리 다이제스트가 아니다
dialectic/4-design.md:45:설계 시점(2026-09-12) 워킹트리는 **커밋 안 된 변경 넷**을 들고 있다:
intent.md:188:- [ ] **A5-b** ⟨결정론적⟩ ★★ **커밋 순서만으로는 인과가 안 선다 — 바이트 앵커를 건다.** `touch/<이슈>.txt` 안의 `fun · <파일>:<줄> · … body <hash>` 줄이 **봉인 커밋 시점의 그 심볼 본문**에서 나온 값임을 보인다. ⟨`PM2-03`(격리 사본에서 재현)·`PM3-14`(유 …⟨줄임 — 원본은 그 좌표에 있다⟩
intent.md:192:- [ ] **A5-a** ⟨결정론적 · `A5`·`A5-b` 의 음성 대조⟩ 사본 **둘**에서 판정이 **뒤집히는지** 재고 `effect/negative-A5.md` 에 둘 다 남긴다 — ⓐ touch 커밋을 변경 커밋 뒤로 옮긴 사본에서 `A5` 가 빨개진다 · ⓑ 코드 변경을 **워킹트리에 먼저 써 놓고** touch 를  …⟨줄임 — 원본은 그 좌표에 있다⟩
intent.md:338:| 2026-09-11 ⟨사전부검 R1⟩ | 착수 관측의 **죽은 링크를 고치고** 워킹트리 문장(`observations/red.md` 머리 인용구 — **절 이름이 아니다**)을 잰 시점에 묶었다 | 내 관측 파일이 `cargo xtask check` 를 **26/28 로 떨어뜨리고 있었다.** 실측으로 확인하고 고쳤다 ⟨`PM …⟨줄임 — 원본은 그 좌표에 있다⟩
intent.md:356:| 2026-09-11 ⟨기록 정합⟩ | `A5-a` 에 **축 ⓑ 를 더했다** — 워킹트리에 먼저 쓴 사본에서 `A5-b` 의 앵커가 빨개지는지 잰다. **조건 수는 50 그대로다** | ★★ 가 붙은 `A5-b` 에만 음성 대조가 없었다. `A5-d`·`A5-e` 는 대조가 아니라 적용 범위를 좁히는 단서다. 규약 `SKILL …⟨줄임 — 원본은 그 좌표에 있다⟩
dialectic/2-design.md:45:설계 시점(2026-09-12) 워킹트리는 **커밋 안 된 변경 넷**을 들고 있다:
dialectic/3-design.md:45:설계 시점(2026-09-12) 워킹트리는 **커밋 안 된 변경 넷**을 들고 있다:
dialectic/1-design.md:45:설계 시점(2026-09-12) 워킹트리는 **커밋 안 된 변경 넷**을 들고 있다:
effect/green-129.txt:3:# 잰 때: 2026-09-11T14:48:40Z · 코드는 워킹트리(커밋 직전)
premortem/r3-raw.md:77:- 어떻게 실패하나: `CA1-17` 은 *"워킹트리 다이제스트를 실어 **같은 SHA 로 다시 돌려도 바이트가 안 맞는다**"* 를 근거로 재실행 대조를 포기했다. 실측: `pal touch nodes_of` 두 번의 **전 출력이 `diff` 로 완전히 동일**하다. 게다가 같은 HEAD·같은 워킹트리에서 심볼마 …⟨줄임 — 원본은 그 좌표에 있다⟩
premortem/r3-raw.md:147:- 어떻게 실패하나: `CA1-02` 는 *"인과가 어디에서도 안 섰다"* 를 고치려고 `A5` 를 세웠다. 그러나 `git merge-base --is-ancestor` 가 대는 것은 **커밋 순서**뿐이다. 실제 변경을 워킹트리에서 먼저 다 쓰고, touch 를 먼저 커밋한 뒤 코드를 커밋하면 `A5` 가 통과 …⟨줄임 — 원본은 그 좌표에 있다⟩
premortem/r3-raw.md:187:- 어떻게 실패하나: `pal touch` 출력의 1 줄은 **빈 줄**이고 다이제스트는 2 줄에 있다. `A2` 는 *"**산출 첫 줄의** 워킹트리 다이제스트"* 를 요구한다
premortem/r2-raw.md:31:- 어떻게 실패하나: 격리 사본에서 실측했다. `check_ledger_pair` 본문에 두 줄을 **커밋하지 않고** 끼운 뒤 touch 를 다시 돌렸더니 `body 1f1846428f55 → f86e990b516b` 로 바뀌었고 **머리는 한 글자도 안 변했다.** 즉 ①워킹트리에 코드 변경을 써 놓고 ②touc …⟨줄임 — 원본은 그 좌표에 있다⟩
premortem/r2-raw.md:101:- 어떻게 실패하나: 세 산출의 1 줄은 **빈 줄**이고 머리는 2 줄이다. `#<hex>` 는 같은 HEAD·같은 워킹트리에서 **심볼마다 다르다** — 실물은 `coord.rs:296` 의 `"{repo}@{tree}#{symbol.short()}"` 이고 **심볼 ID** 다. 그리고 `pal touch n …⟨줄임 — 원본은 그 좌표에 있다⟩
premortem/r2-raw.md:151:- 어떻게 실패하나: `A1-c` 는 *"`C1`·`C3` 이 실제로 쓴 사전 등록이 그 blob 과 바이트로 같다"* 를 요구한다. 그런데 `C3` 이 보존하라는 것은 **프롬프트 전문**뿐이고, 프롬프트가 넷을 **경로로** 넘기면 전문 안에 사전 등록 바이트가 없다. 판정자가 읽은 것은 그 순간의 워킹트리이고 …⟨줄임 — 원본은 그 좌표에 있다⟩
observations/identity-after.txt:4:Snapshot  palimpsest@f60df1d+worktree  (워킹트리)
observations/identity-after.txt:8:워킹트리  f60df1d 와 다른 파일 2개  ·  인덱스 신뢰 1266 · 다시 잼 2
observations/issue-66-now.txt:51:Snapshot  palimpsest@49a6b6b+worktree  (워킹트리)
observations/red.md:4:> **착수를 잰 시점**(2026-09-11 · 이 회차 디렉터리를 만들기 전)에 워킹트리는 `8604d62` 와
observations/identity-before.txt:4:Snapshot  palimpsest@f60df1d+worktree  (워킹트리)
observations/identity-before.txt:8:워킹트리  f60df1d 와 같음  ·  인덱스 신뢰 1268 · 다시 잼 0
```
