# 반(反) — 판 3 · `C2`·`C2-b` ⟨`pal-decision-opponent`⟩

> 아래는 에이전트의 **반환문 전문**이다. 부르는 쪽이 한 글자도 안 고쳤다.
> **안 받은 것** — `intent.md` 의 `## 차선책`·`## 범위 밖` · `findings.jsonl` 의 처분 ·
> 메인의 사고 과정 · 다른 판의 반론표 · 상한과 끝 조건.

---

프롬프트에 사고 과정이나 대화 기록은 섞여 있지 않았다. 받은 것은 초안 전문·조건 문면·저장소뿐이다. 새 실행은 안 했다 — 읽기 전용 `git`·`grep`·`sed` 만 썼다.

## 초안이 서는 자리

무너뜨리지 못한 것들이다. 전부 직접 열어 대조했다.

- **좌표 검증 대부분이 맞다.** `touch/126.txt:17`·`:24`, `touch/79.txt:23`·`:36-46`·`:56`, `touch/129.txt:21`·`:23` 이 초안이 적은 내용 그대로다.
- **㉠ⓔ 의 프롬프트 좌표 오류가 실재한다.** `effect/judge-prompt.md:1095`(초안이 `:1098` 이라 적은 절) 이 `(:19-20)` 인데 `touch/79.txt:18` 이 `■ 이 좌표에 걸린 것 (0)` · `:19` 가 `아직 없습니다.` · `:20` 이 `■ 이 좌표를 지켜보는 것 (0)` 이다. 초안이 옳다.
- **결과 좌표 둘의 실물이 맞다.** `git show 398d233:xtask/src/main.rs` 에서 `:643` 이 `("원장 둘 대조", check_ledger_pair(root)),` 이고 `:6405` 가 `}` · `:6407` 이 새 함수 문서 주석 머리다.
- **유효 라벨의 독립 재확인이 맞다.** `grep -n "ordinal" plan/126-pre.md` 가 0 건이고, `plan/126-pre.md` 의 「호출자」는 `:49`(§5.8 사본의 `identity_ceiling` 행)·`:82`(다른 뜻) 둘뿐이다.
- **`premortem/r2-raw.md:51`·`:31` 인용이 정확하다.** 그리고 `C2`(귀속된 차이) ↔ 「유효 N」(새 사실) 의 구별을 `effect/129-judge-2.md:56-59` 대로 지킨 것도 맞다.

## 반론

| # | 반론 | 좌표 | 유효성 | 해악도 |
|---|---|---|---|---|
| R1 | 초안의 가장 무거운 문단(손잡이 1)이 든 좌표 셋이 **전부 한 줄씩 어긋난다** — `check_ledger_pair.txt:2` 는 심볼 ID 줄이고 `fun … identity ordinal` 은 `:3` 이며, `:9` 는 `■ 이 심볼이 하는 것` 이고 `호출자 1 · 피호출자 12` 는 `:10` 이다. `README.md:24` 는 `identity_ceiling.txt` 행이고 `호출자 1` 을 적은 것은 `:23` 이다 | `observations/premortem-artifacts/check_ledger_pair.txt:3`·`:10` · `observations/premortem-artifacts/README.md:23` ↔ `dialectic/3-thesis.md:84` | 참 | 거짓신호 |
| R2 | **㉡ⓑ 도 오염돼 있고 초안이 근거를 못 찾았다고 한 그 근거가 실재한다** — `clone-before.txt:3` 과 `check_ledger_pair.txt:3` 이 `identity ordinal` 을 글자 그대로 담는다. 더 아픈 것은 `effect/126-readnote.md:67` 이 **같은 파일을 ⓒ 의 오염원으로만 공시**하고 **같은 줄이 담은 ⓑ 는 안 적었다**는 것이다. 배제 집합을 「메인이 이미 안 것」으로 읽으면 분자는 초안이 적은 1 이 아니라 **0** 이다 | `observations/premortem-artifacts/clone-before.txt:3` · `effect/126-readnote.md:67` ↔ `dialectic/3-thesis.md:85` | 참 | 금지역 |
| R3 | **형식 미달은 ㉡ⓑ 하나가 아니라 일곱 중 여섯이다.** 판정자 반환 형식(`effect/judge-prompt.md:28-29`)이 *"있다(**인용**) \| 없다"* 라 **「없다」쪽엔 인용을 아예 안 요구한다** — 유효 판정은 구조상 인용을 못 낳는다. 두 문서를 다 인용한 것은 `effect/79-judge-1.md:21-22` 하나뿐이고, `126-judge-1.md:32-33`(ⓑ)·`:39-40`(ⓒ) 은 둘 다 맨 *"없다"*, `:25`(ⓐ) 는 §5.8 만, `79-judge-1.md:36`(ⓒ) 은 사전 등록만, `129-judge-1.md:47-48` 은 §5.8 을 안 인용하고 사전 등록 §2.3 의 말을 인용한다. 초안의 *"일곱 전부에 규정 형식의 내용 시험이 적혔다"* 는 **다섯 칸이 채워졌다**를 **두 문서 인용으로 보였다**로 바꿔 부른 것이다 | `effect/judge-prompt.md:28-29` · `effect/126-judge-1.md:32-33`·`:39-40` ↔ `dialectic/3-thesis.md:16`·`:26` | 참 | 금지역 |
| R4 | **`C2-b` 는 증거가 아니라 메인의 자르기를 잰다.** `touch/126.txt:17` **한 줄**이 `effect/126-judge-1.md:30`(ⓑ 유효)·`:37`(ⓒ 유효)·`:44`(ⓓ 무효) 로 **인용한 줄이 세 번 다 글자까지 같은데 판정이 갈린다.** 가른 것은 그 줄이 아니라 메인이 `effect/judge-prompt.md:413-418` 에 **미리 써 준 「주장하는 사실」**이다. 즉 분자를 혼자 지는 ⓑ 는 「사실을 어떻게 자르면 사전 등록 밖이 되나」를 메인이 고른 결과다 | `effect/126-judge-1.md:30`·`:37`·`:44` · `effect/judge-prompt.md:413-418` | 참 | 금지역 |
| R5 | **§5.8 의 「전수」는 내용이 빠진 게 아니라 시간상 불가능하다.** `observations/red.md:231` 이 스스로 *"사전부검 **라운드 1** 이"* 라 적고, 그 파일은 `7b4ddaa`(2026-09-11 10:43:43)에 한 번 커밋된 뒤 **고쳐진 적이 없다**(`git log -- observations/red.md` 가 커밋 하나). 사전부검 R2·R3 은 `fbec3dc`(16:17:50)이고 `check_intent_untouched`·`of_normalized`·`grade`·격리 사본까지 더 돌았다(`observations/premortem-artifacts/README.md:23-26`). 배제 오라클이 **오염의 3 분의 1 만 덮도록 구성돼 있다** — 초안은 이것을 「새는 사실 하나」로 다뤘다 | `observations/red.md:231` · `git log --format='%h %ci' -- observations/red.md` → `7b4ddaa 2026-09-11 10:43:43` ↔ `dialectic/3-thesis.md:80` | 참 | 금지역 |
| R6 | **초안의 세는 규칙이 ㉡ 과 ㉠ 에 비대칭으로 걸렸다.** `effect/79-delta.md:37-40` 이 *"그래도 touch 가 바꾼 것이 하나 있다 — 갈래 선택의 **근거**다(`touch/79.txt:36-46`)"* 라 적고 그것을 **차이 표 밖에 둔다.** `effect/126-delta.md:26` 의 ㉰ 는 **똑같은 범주**(근거가 바뀌었다)인데 표 안에 있어 분자로 센다. 같은 자를 대면 합은 3 이거나 1 이고, 초안이 적은 **2 는 어느 읽기에서도 안 나온다** | `effect/79-delta.md:37-40` ↔ `effect/126-delta.md:26` ↔ `dialectic/3-thesis.md:57`·`:87` | 참 | 거짓신호 |
| R7 | **분자 둘의 「결과 좌표」가 둘 다 「안 바뀐 자리」다.** `effect/126-delta.md:27` 이 ㉱ 의 결과를 *"`:643`(**안 건드린** 자리)"* 라 스스로 적고, ㉰ 의 `:6407` 은 `plan/126-pre.md:82` 가 봉인 전에 *"`check_ledger_pair` **뒤** (새 자리)"* 로 이미 지정한 바로 그 자리다. 그런데 초안은 `dialectic/3-thesis.md:62` 에서 **둘을 열어 「안 바뀐 것」을 확인하고 그것을 확증으로 적었다** — 차이의 결과 칸에서 무변화를 확인하는 것은 확증이 아니다 | `effect/126-delta.md:27` · `plan/126-pre.md:82` ↔ `dialectic/3-thesis.md:62` | 참 | 거짓신호 |
| R8 | **초안이 음성 대조가 「실행됐다」고 기댄 근거가 같은 회차에서 반증돼 있다.** `effect/126-judge-2.md:51-52` 는 되불러 준 수 넷이 같은 것을 *"두 인스턴스가 같은 바이트를 받았다는 **기계 대조**"* 라 적는데, `effect/79-judge-1.md:70-72` 가 같은 회차에서 *"판정자의 계수와 인용이 **근사**"*(③ 345↔337 · `+` 232↔233)임을 실측하고 `MS-10` 으로 싣는다. 게다가 `effect/129-judge-2.md:31` 이 §5.8 마지막 줄을 `129-judge-1.md:31`·`126-judge-1.md:18` 과 **다른 형태(줄바꿈 없이 이어 붙인 문장)**로 되불렀다 — 넷이 인스턴스 간 같은 바이트였다는 주장 자체가 흔들린다. 초안은 이것을 `:97-98` 의 「확인 못 한 것」으로만 적고 `:91` 에서는 실행됐다고 세었다 | `effect/126-judge-2.md:51-52` ↔ `effect/79-judge-1.md:70-72` · `effect/129-judge-2.md:31` ↔ `dialectic/3-thesis.md:91` | 참 | 거짓신호 |
| R9 | **`C2-b` 가 요구한 「인용한 `touch/<이슈>.txt:N` 의 줄」에서 판정자가 `N` 을 검증한 적이 한 번도 없다.** `effect/126-judge-1.md:23`·`:30`·`:37` 의 「인용한 줄」엔 줄 번호가 아예 없고, 번호는 `effect/judge-prompt.md:413`·`:417` 에서 메인이 괄호로 붙인 것이다. 초안이 찾은 ㉠ⓔ 의 오류는 **고립된 실수가 아니라 검증된 적 없는 모집단의 표본**이다 — 초안은 *"미끼라 분자에 안 들어간다"* 로 처분하고 모집단을 안 물었다 | `effect/126-judge-1.md:23`·`:30`·`:37` · `effect/judge-prompt.md:413`·`:417` ↔ `dialectic/3-thesis.md:38` | 참 | 거짓신호 |
| R10 | **초안은 자기가 안 연 문서 때문에 여유의 크기를 못 잰 채 통과를 냈다.** `dialectic/3-thesis.md:101` 이 *"`C2` 가 0 이면 무슨 일이 생기나를 모른 채 세었다"* 고 적는데, `premortem/r2-raw.md:58` 이 이미 *"`C2` 가 0 이면 차선책 1 이 발동해 「반증」으로 회차가 닫힌다 … **잘못된 반증은 되돌리기가 가장 비싸다**"* 라 적어 두었다. R2 와 겹치면 여유 2 가 아니라 **여유 0 일 수 있는 판정**을 여유 2 로 보고한 것이 된다 | `premortem/r2-raw.md:58` ↔ `dialectic/3-thesis.md:101` | 참 | 거짓신호 |

## 내가 스스로 물린 것

- **「판정자가 받은 touch 가 한 줄 잘렸다」** — 산출했다가 접었다. `touch/126.txt` 는 `wc -l` 60 이지만 `:60` 이 꼬리 빈 줄이고, 판정자가 되불러 준 첫 줄(`:1`)·마지막 줄(`:59` 의 `능력 …`)이 파일과 일치한다. 내용이 빠진 자리를 못 짚었다 — 좌표 없음.
- **「§5.8 이 봉인 뒤에 고쳐졌다」** — 의심했으나 `git log -- observations/red.md` 가 커밋 하나(`7b4ddaa`)뿐이라 거짓이다. 대신 R5 로 방향을 바꿨다.
- **「`effect/negative-C2.md` 부재가 `C2-b` 를 깬다」** — 초안 손잡이 4 가 이미 다뤘고, 부재가 `C2-b` 에 무엇을 뜻하는지 새로 댈 좌표를 못 찾았다. 유효성 거짓이라 표에 안 실었다.
- **「㉡ⓐ 의 `호출자 1` 이 거짓이다」** — 새 실행 금지라 못 쟀다. 다만 `effect/126-readnote.md:14` 의 유일한 확인이 `grep -n "check_ledger_pair" xtask/src/main.rs` **한 파일 grep** 이어서 `effect/79-delta.md:94`(`MS-08`)가 적은 파일 간 누락에 원리상 눈이 멀어 있다는 것까지는 좌표로 선다. 그러나 참·거짓은 `C4` 의 것이고 이 판의 값을 안 바꾸므로 표에 안 실었다.

관련 파일 절대 경로:
- `/Users/incognito/dev/projects/palimpsest/.palimpsest/rounds/2026-09-11-effect-confirmation/dialectic/3-thesis.md`
- `/Users/incognito/dev/projects/palimpsest/.palimpsest/rounds/2026-09-11-effect-confirmation/effect/judge-prompt.md`
- `/Users/incognito/dev/projects/palimpsest/.palimpsest/rounds/2026-09-11-effect-confirmation/effect/126-judge-1.md`
- `/Users/incognito/dev/projects/palimpsest/.palimpsest/rounds/2026-09-11-effect-confirmation/observations/premortem-artifacts/clone-before.txt`
- `/Users/incognito/dev/projects/palimpsest/.palimpsest/rounds/2026-09-11-effect-confirmation/observations/red.md`
