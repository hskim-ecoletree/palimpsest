# 정반합 취합 — 2026-09-13-first-release-elsewhere 라운드 3 · p2-caller-sites · `touch` 에 호출자 자리를 싣도록 넓혀 `E2` ⑵ · `E4` 를 닫나, 반증으로 적고 원인을 이슈로 떼나

> 판 **`p2-caller-sites`** · 판 안 라운드 **R2/2**(상한 소진) · 낸 자리: 취합 보고(서브에이전트, 읽기 전용 — **이 본문은 반환문이고 메인이 옮긴다**)
> 이 파일의 원장 라운드 번호는 발주가 준 **3** 이다(`extract.py 정반합 3`). 판 안의 라운드(R1 · R2)는 항마다 `판 라운드` 불릿에 따로 적었다. 두 번호를 하나로 맞추지 않았다.
> 제목 뒤 물음은 설계문 제목(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-design.md:1`)의 「토론 설계 — 」 뒤를 그대로 옮겼다.
> **옮겨 적은 자:** 메인. 취합 보고 에이전트가 스크래치 `p2-report/out.md`(md5 `29e3ac25470ea27b0fb85248c6a65ccb`)에 쓴 같은 바이트를 파일째 복사했다 — 이 인용 줄 하나만 더했다.

## 내가 받은 것 / 안 받은 것

**받았다:** 이 판의 산출물 **아홉** 전문 — `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/` 의 `p2-design.md` · `p2-thesis.md` · `p2-antithesis.md` · `p2-synthesis.md` · `p2-referee.md` · `p2-thesis-r2.md` · `p2-antithesis-r2.md` · `p2-synthesis-r2.md` · `p2-referee-r2.md`. 발주 시점에 뒤 넷 가운데 `p2-antithesis-r2.md` · `p2-synthesis-r2.md` · `p2-referee-r2.md` 는 작업 트리에만 있었다(커밋 전 — `git status` 의 `??`). 이 보고를 만드는 사이 셋은 `p2-brief-referee-r2.md` 와 함께 커밋 `254505e`(2026-09-14 07:45:14)로 들어갔다 — 내가 읽은 바이트와 그 커밋의 바이트를 대 보지는 않았다. 입력 경계 확인용으로 역할 발췌 넷(`p2-brief-opponent.md` · `p2-brief-synthesis.md` · `p2-brief-referee-r1.md` · `p2-brief-referee-r2.md`)을 열었다. 회차 이름 · 판 이름 · 원장 스키마 `.claude/skills/round/bin/record.py --schema`.

**형식 확인에 연 것:** `.claude/skills/round/bin/extract.py`(전문) · `xtask/src/main.rs` 의 `반환문_항_수` · `펜스_밖` · `기각_표_헤더` · 같은 회차 `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/r1-raw.md` · `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/r2-raw.md`(형식 참고로만 — 그 판들의 내용은 이 보고에 안 넣었다).

**내가 직접 잰 것(전부 읽기):** ⑴ 반론표 · 채택표 · 기각표 · 분포표 행을 스크립트로 뽑아 칸 수를 댔다(이스케이프 안 된 파이프로 가름 — 반(反) 행 5 칸 · 합(合) 채택 행 4 칸 · 기각 행 3 칸 전부). ⑵ 신호 6 · 7 · 9 를 스크립트로 셌다(규칙은 신호 표에 적었다). ⑶ `어디가 걸리나` 좌표가 저장소 트리에 있는지 `git ls-files --cached --others --exclude-standard` 에 댔다. ⑷ 이 본문을 스크래치패드에 써서 추출기와 계수기 사본을 돌렸다(`## 계수` 끝). **반론 · 판정 · 좌표 문장은 스크립트로 원 파일의 칸과 줄을 뽑아 붙였다 — 손으로 옮기지 않았다.** 산출물이 인용한 저장소 좌표(`intent.md` · `effect/` · `oracle/` · 코드)의 원문 줄은 열지 않았다 — 그래서 그 좌표가 가리키는 글자는 이 자리가 확인하지 않았다.

**안 받았다 · 안 열었다:** 대화 기록 · 메인의 사고 과정 · 메인의 입장 · 소유자의 답 · 이 판정을 실행할 계획 · `.palimpsest/rounds/2026-09-13-first-release-elsewhere/state.md` · `findings.jsonl` · `premortem/` · `conditions-audit/` · `p1-*` · `e3-*`. `ls` 와 `git status` 에 `p1-*` · `e3-*` 이름과 `docs/gates/first-release-elsewhere.md` 이름이 찍혔지만 열지 않았다. ⚠ HEAD 가 움직인 것을 보려고 `git log --oneline -5` 를 돌렸고, 판이 닫힌 뒤의 커밋 제목 셋(`403d589` · `eb9c936` · `4f4a176`)이 찍혔다. 소유자의 답 · 판정 실행에 닿는 제목이라 **이 보고 어디에도 그 내용을 쓰지 않았다** — 그 커밋들은 열지 않았다.

**판정을 다시 하지 않았다.** 내가 보탠 값은 넷이고 이름으로 신고한다. ⑴ **`모집단` 은 원 산출물 어디에도 없다 — 내가 매겼다.** 규칙: 반론의 과녁이 회차 산출물(정(正) 초안)이면 `회차기록`. 스물셋 반론 전부가 그 과녁이라(반(反) 두 파일의 머리 「받은 것」이 초안이다) 전부 `회차기록` 이다. ⑵ **`어디가 걸리나` 는 반(反)의 좌표 칸에서 뽑았다.** 규칙: 백틱 안이 공백 없는 파일 좌표이고 그 파일이 저장소 트리에 있는 첫 좌표. `R/` 는 회차 디렉터리로, `SKILL.md` 는 `.claude/skills/round/SKILL.md` 로, `p2-*.md` 와 「초안 `:줄`」은 그 라운드 초안의 `dialectic/` 경로로 풀었다. 명령문(`git show --stat …` · `ls …`) · 해시 · 저장소 밖 좌표(`…/scratchpad/…`)는 건넜다. ⑶ **`유효성` 은 반(反)의 값이다** — 칸 원문에서 처음 나온 enum 값. 합(合)은 유효성 칸을 따로 두지 않았다. ⑷ **`해악도` 는 합(合)의 값이다** — 채택 스물둘에서 합(合)과 반(反)의 등급이 전부 같다(`## 계수`). 기각 하나는 합(合)이 등급을 안 적어 반(反)의 값이다.

**입력 경계 — 설계문 표(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-design.md:54-63`)와 각 산출물 머리를 댄 결과.** 넘었다고 판정하지 않는다. 머리가 스스로 신고한 줄을 그대로 옮긴다.

`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-thesis.md:6` 그대로:

> > **안 받은 것 · 안 연 것**: 대화 기록 · 메인의 사고 과정 · `R/state.md` · `R/findings.jsonl` · `R/premortem/` · `R/conditions-audit/` · `R/dialectic/` 의 `p1-*` · `e3-*` · `r1-raw.md` · 원장 레코드 본문 · `E3` 판의 승격 내용(`R/intent.md:389-391` 의 승격 표 행은 전문을 받으면서 눈에 들어왔다. 근거로 쓰지 않았다) · `docs/gates/first-release-elsewhere.md` · `R/effect/06-delta.md`

`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-antithesis.md:6-7` 그대로:

> > **우연히 눈에 든 것 (근거로 쓰지 않았다)**: `grep` 출력에 `p2-design.md:29`(F4 행, 발췌와 같은 글) 한 줄과 `R/dialectic/` 파일 이름 목록이 찍혔다. 다른 회차 `2026-09-12-binding-radius-in-use` 의 dialectic 줄도 몇 개 찍혔다.
> > **열람 범위 밖에서 연 것 (밝혀 둔다)**: `docs/adr/0011-moved-volume-and-unseen-volume-are-different-fields.md` · `scripts/s2-verify.py:125-150` · `corpus/criteria.toml` grep · 커밋 `2d22dd4` · `42af275` · `5a4b2fd` · `1e54679` 의 `git show --stat` · `git log --diff-filter=A`(`effect/04-callers` · `00-seal.md` · `E2-scene.py`). 초안이 인용한 좌표를 치거나 받치는 데 필요했다.

`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis.md:5-7` 그대로:

> > **안 받은 것 · 안 연 것**: `R/dialectic/p2-design.md` · `R/dialectic/p2-brief-opponent.md` · `R/state.md` · `R/findings.jsonl` · `R/dialectic/` 의 `p1-*` · `e3-*` · `r1-raw.md` · 원장 레코드 본문 · `docs/gates/first-release-elsewhere.md` · 대화 기록 · 메인의 사고 과정. 프롬프트에 대화 기록이나 사고 과정은 섞여 오지 않았다. (세션 시작 때 받은 git 상태에 `docs/gates/first-release-elsewhere.md` 의 **파일 이름**이 찍혀 있었다. 열지 않았다.)
> > **다툼 좌표를 확인하려고 연 것**: `R/intent.md`(1-90 · 120-400) · `.claude/skills/round/SKILL.md:295-400` · `R/oracle/` 의 `E1-order.sh` · `E1-order.txt` · `E2-scene.py` · `E2-scene.txt` · `E2-literal-touch.txt` · `E4-breakage.mjs` · `E4-breakage.txt` · `E4-literal-touch-c.txt` · `R/effect/` 의 `00-seal.md` · `03-touch/05-codexHostAdapter.txt` · `.json.txt` · `03-touch/head.txt` · `04-callers/01` · `05` · `06` · `04-readnote.md` · `05-tsc-after-A.txt` · `05-change.txt:1-20` · `crates/pal-query/src/lib.rs:565-590` · `:825-850` · `crates/pal-core/src/touch.rs:235-250` · `:335-350` · `crates/pal-cli/src/touch.rs:490-515` · `docs/plan/00-goals.md:165-195` · `docs/adr/0011-moved-volume-and-unseen-volume-are-different-fields.md` 전문 · `scripts/s2-verify.py:120-150` · `git show --stat 2d22dd4 42af275 5a4b2fd 1e54679 c43f159` · `git log --diff-filter=A -- R/effect/03-touch`
> > **내가 직접 돌린 것 (읽기만)**: 봉인 심볼 13 마다 `R/effect/03-touch/NN-*.json.txt`(`-pick` 이 있으면 그것)의 `facts.present.callers` 와 `R/effect/04-callers/NN-*.txt` 의 호출자 줄 수를 댔다. **13 쌍이 전부 같다**(01~04 · 11~13 은 1=1 · 05 는 2=2 · 06~10 은 0=0). 이것은 일회성 확인이다. 오라클 산출로 커밋된 것이 아니다.

`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-thesis-r2.md:5-7` 그대로:

> > **안 받은 것 · 안 연 것**: `R/dialectic/p2-antithesis.md` · `p2-thesis.md` · `p2-referee.md` · `p2-brief-*` · `p1-*` · `e3-*` · `r1-raw.md` · `r2-raw.md` · `R/state.md` · `R/findings.jsonl` · `R/premortem/` · `R/conditions-audit/` · 원장 레코드 본문 · `docs/gates/first-release-elsewhere.md` · 대화 기록 · 메인의 사고 과정. `R/dialectic/` 은 `ls` 로 **파일 이름만** 봤다. 세션 머리의 git 상태에 `docs/gates/first-release-elsewhere.md` 의 이름이 찍혀 있었지만 열지 않았다. 프롬프트에 대화 기록은 섞여 오지 않았다.
> > **좌표를 확인하려고 연 것**: `R/intent.md` 전문 · `R/interview/r1.md` · `r2.md`(금지 목록에 없다) · `docs/plan/00-goals.md:92-186` · `SKILL.md:305-400` · `:718-835` · `crates/pal-core/src/touch.rs:235-250` · `:335-348` · `crates/pal-query/src/lib.rs:560-595` · `:820-850` · `crates/pal-store/src/projection.rs:700-734` · `crates/pal-cli/src/touch.rs:485-560` · `:54` · `crates/pal-cli/src/query.rs:325-345` · `:466-486` · `scripts/s2-verify.py:132-142` · `R/oracle/` 의 `E2-literal-touch.txt` · `E2-scene.txt` · `E2-scene.py:40-70` · `E4-breakage.mjs` 전문 · `E4-breakage.txt` · `E4-literal-touch-c.txt` · `R/effect/` 의 `00-seal.md:44-78` · `03-touch/05-codexHostAdapter.txt` · `.json.txt:14-18` · `:100-118` · `03-touch/06~10-*-pick.txt:2-3` · `04-callers/01` · `05` · `06~11` · `04-readnote.md` · `05-change.txt:1-40` · `05-tsc-after-A.txt` · `docs/adr/0011-…md` 의 `unfolded_by` 줄 · `git show -s c43f159` · `git log` 커밋 제목 `5a4b2fd` · `1e54679` · `8575526` · `3d25dce`.
> > **내가 직접 돌린 것(읽기만)**: 13 심볼마다 `03-touch/NN-*.json.txt`(`-pick` 이 있으면 그것)의 `"callers"` 값과 `04-callers/NN-*.txt` 의 `경로:줄` 줄 수를 댔다. 13 쌍 모두 같다(01~04 · 11~13 은 1=1 · 05 는 2=2 · 06~10 은 0=0). 이것은 일회성 확인이고 산출로 커밋되지 않았다. ⚠ 06~10 의 0=0 이 **무엇을 증명하지 않는지**는 아래 근거 5 에 적었다.

`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-antithesis-r2.md:5-9` 그대로:

> > **안 받은 것 · 안 연 것**: `R/dialectic/p2-design.md` · `p2-antithesis.md` · `p2-synthesis.md` · `p2-thesis.md` · `p2-referee.md` · `p2-brief-synthesis.md` · `p2-brief-referee-*` · `p1-*` · `e3-*` · `r1-raw.md` · `r2-raw.md` · `R/state.md` · `R/findings.jsonl` · `R/premortem/` · `R/conditions-audit/` · 원장 레코드 본문 · `docs/gates/first-release-elsewhere.md` · 대화 기록 · 메인의 사고 과정. `R/dialectic/` 은 `ls` 로 이름만 봤다. 프롬프트에 대화 기록이나 사고 과정은 섞여 오지 않았다.
> > **좌표를 확인하려고 연 것**: `R/oracle/` 의 `E2-scene.py` · `E2-scene.txt` · `E2-literal-touch.txt` · `E4-breakage.mjs` · `E4-breakage.txt` · `E4-literal-touch-c.txt` · `R/effect/04-callers/01~13` · `04-readnote.md` · `03-touch/05-codexHostAdapter.txt` · `06-loadInstructions.txt` · `06-loadInstructions-pick.txt` · `head.txt` · `05-change.txt` 머리 · `R/interview/r1.md` · `r2.md`(grep) · `.claude/skills/round/SKILL.md:55-72` · `:300-400` · `:715-840` · `:885-935` · `docs/plan/00-goals.md:92-190` · `docs/plan/01-completion-scenes.md:1-140` · `crates/pal-query/src/lib.rs:555-651` · `:815-850` · `crates/pal-cli/src/query.rs:315-350` · `:460-490` · `crates/pal-cli/src/touch.rs:50-58` · `:490-560` · `crates/pal-core/src/touch.rs:240-248` · `docs/adr/0011-*.md:38-46`.
> > ⚠ **허용 목록 밖에서 연 것 하나**: `R/approval.md`. 금지 목록에 없었고, 초안의 좌표를 확인하는 자리가 아니었다. 반론 1 이 거기에 기댄다. 합(合)이 이 좌표를 받지 않기로 하면 반론 1 은 떨어진다.
> > **돌린 명령(읽기만)**: `git log --format='%h %ad %s' --date=iso` 를 `R/intent.md` · `R/approval.md` · `R/oracle/E4-*` · `E2-*` · `R/effect/04-callers` · `05-change.txt` 에 · `git log -S'이슈로 세운다'` · `git log -S'안 센다 — 대조 불가'` · 복제본 `ditto-effect` · `ditto-effect-at-A` 의 `git rev-parse HEAD` · `.git/logs/HEAD` · `git log -3` · `git worktree list` · 인터뷰 두 파일 grep.
> > ⚠ 작업 디렉터리 뿌리를 `ls` 하다가 `e4-control-query-c.txt` · `e4-c-query.txt` · `e4-c-empty.txt` · `e4-literal-c-empty.txt` 라는 **이름**을 봤다. 허용 밖이라 **열지 않았다**. 아래 어디에도 그 내용을 쓰지 않는다.

`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis-r2.md:5-7` 그대로:

> > **안 받은 것 · 안 연 것**: `R/dialectic/p2-design.md` · `p2-synthesis.md` · `p2-thesis.md` · `p2-antithesis.md` · `p2-referee.md` · `p2-brief-opponent.md` · `p2-brief-referee-*` · `p1-*` · `e3-*` · `r1-raw.md` · `r2-raw.md` · `R/state.md` · `R/findings.jsonl` · 원장 레코드 본문 · `docs/gates/first-release-elsewhere.md` · 대화 기록 · 메인의 사고 과정. 반(反)이 이름만 봤다고 적은 작업 디렉터리 뿌리의 `e4-*` 파일들도 열지 않았고 쓰지 않는다.
> > **좌표를 확인하려고 연 것**: `R/approval.md` 전문 · `R/intent.md`(`:15-75` · `:125-200` · `:255-395`) · `R/effect/04-readnote.md` 전문 · `R/effect/03-touch/01-mcpServersFromToml.txt:1-20` · `05-codexHostAdapter.txt:1-16` · `06-loadInstructions-pick.txt:1-4` · `R/effect/04-callers/05-codexHostAdapter.txt` · `06-loadInstructions.txt:1-6` · `R/oracle/E2-literal-touch.txt` · `E4-literal-touch-c.txt` · `E4-breakage.txt` · `E2-scene.txt` · `crates/pal-query/src/lib.rs:570-582` · `:615-645` · `:835-842` · `.claude/skills/round/SKILL.md:300-400` · `:725-800` · `:895-920` · `docs/plan/00-goals.md:100-120` · `:135-146` · `:175-186` · `docs/plan/01-completion-scenes.md:15-55`.
> > **돌린 명령(읽기만)**: 커밋 `42af275` · `6145849` · `b61df62` · `1d7b3b8` · `2d22dd4` · `d1f8ddb` · `c43f159` 의 `git log --format='%h %ad %s' --date=iso -1` · 복제본 `ditto-effect/.git/logs/HEAD` 끝 다섯 줄 · `date -r 1789307699` · `date -r 1789307927` · `R/approval.md` 의 `git log`.

`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-referee.md:5-7` 그대로:

> > **받은 것.** `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-brief-referee-r1.md` 한 파일이다. 그 안에 네 가지가 있다. 설계문 「상한과 끝 조건」 절 원문, 살아남은 반론의 해악도 분포(등급별 개수와 반론 ID), 합(合) 판정의 §5 분류 한 낱말, 쓴 라운드 수.
> >
> > **안 받은 것.** 판정문 본문(`p2-thesis.md` · `p2-antithesis.md` · `p2-synthesis.md`) · 설계문 전문 · 사실표 · 물음 본문 · 대화 기록 · 회차의 다른 파일은 받지 않았고 열지 않았다. 판정 내용이 옳은지는 보지 않았다.

`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-referee-r2.md:5-7` 그대로:

> > **받은 것은 하나다.** `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-brief-referee-r2.md` (작업 트리, 커밋 전). 그 안에는 설계문 「상한과 끝 조건」 절 원문, 살아남은 반론의 해악도 분포(등급별 수와 반론 ID), 합(合) 판정의 §5 분류 칸, 쓴 라운드 수가 있다.
> >
> > **안 받은 것.** 판정문 본문(`p2-thesis*.md` · `p2-antithesis*.md` · `p2-synthesis*.md`), 설계문 전문, 앞 종료 판단 `p2-referee.md`, 사실표, 물음 본문, 대화 기록, 회차의 다른 파일 전부. 열지 않았다. 그래서 반론 1 의 내용과 분류 판정의 내용은 모른다. 이 판단은 수·분류 칸·라운드 수에만 기댄다.

합(合) R2 가 반(反) R2 의 `R/approval.md` 좌표를 받은 까닭 그대로(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis-r2.md:36-39`):

> **반론 1 이 연 `R/approval.md` 를 받는다.**
> - 까닭 하나: 그 파일은 토론 산출이 아니라 규약 §4 의 승인 기록이다. 메인이 준 내 허용 목록에 들어 있다.
> - 까닭 둘: 내가 직접 열어 문장을 확인했다(`R/approval.md:13-14`). 커밋 `66fc201`(2026-09-13 20:49:39 · 「잠긴 의도가 승인됐다」)의 파일인 것도 확인했다.
> - 까닭 셋: 반(反)이 허용 밖에서 열었다는 사실을 스스로 밝혔다. 그 좌표는 원문 확인으로 오염 우려가 풀린다. 이 판의 독립성이 막으려는 것은 다른 역할의 산출이 섞이는 것이지, 잠긴 1차 기록을 읽는 것이 아니다.

설계문 표 5 행은 종료 판단에게 「합(合) 판정의 §5 분류 칸 **한 낱말**」을 준다고 적었다(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-design.md:62`). 종료 판단 R2 에게 준 발췌의 그 줄 그대로(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-brief-referee-r2.md:15`):

> - 합(合) 판정의 §5 분류 칸: **정정** — 단 합이 같은 칸에 「**완화와 갈라내지 못한다**」고 명시했다

## 판정

합(合) R2 의 판정 절 그대로(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis-r2.md:12-45`):

> **보류** — 게이트가 아니다. 아래 「소유자에게」가 게이트다.
>
> **고른 길**: 초안의 길인 **조건 문면 정정**이다. `crates/` 는 바꾸지 않는다. 다만 아래 「초안을 어떻게 고치나」를 반영한다. 이 길은 소유자가 칸 하나에 「받는다」로 답한 뒤에만 실행한다.
>
> **§5 분류 한 낱말**: **정정**. ⚠ **나는 이것을 완화와 갈라내지 못한다.** 게이트(발췌 `p2-brief-synthesis.md:58`)와 `SKILL.md:381-383` 에 따라 올린다. 갈라내지 못하는 까닭은 잠긴 기록의 두 묶음이 서로 반대쪽을 가리키기 때문이다.
>
> - **정정 쪽 좌표**
>   - 원문의 완성 장면과 계획이 요구한 것은 **수**다(`R/intent.md:49` · `:131` · `:165`).
>   - 정본 화면의 호출자 줄도 수다(`docs/plan/00-goals.md:106`). 두 문서가 갈리면 이 절이 이긴다(`:137-139` · `docs/plan/01-completion-scenes.md:29`).
>   - `E4` 는 스스로 **P∩C** 를 산출 칸으로 둔다(`R/intent.md:363`). 이 구조는 C 가 「실제로 있는 호출자 파일」이라는 뜻을 전제한다. 문면대로 C = 없음으로 재면 P∩C 칸은 구조상 늘 비고, `E4` 는 「파손이 전부 최상위인가」로 퇴화한다(`R/oracle/E4-literal-touch-c.txt:7` · `:9`).
> - **완화 쪽 좌표**
>   - 소유자는 「이 29 로 끝을 재는 것」을 승인했다(`R/approval.md:11`). 그 전제는 `E4` 를 이름으로 짚어 「반증으로 적는다 — 조건을 약하게 고치지 않는다」였다(`:13-14`).
>   - 승인된 조건 문면 자체가 `touch` **출력에서** 파일을 요구한다. `E2` ⑵ 는 「그 출력들에서 … 다른 파일의 참조」(`R/intent.md:361`)이고, `E4` 는 「`touch` 가 호출자로 낸 파일 집합」(`:363`)이다. ⚠ 초안의 ⓒ 행(`p2-thesis-r2.md:87`)은 「잠겨 있지 않다」의 근거로 `:49` · `:131` · `:165` · `00-goals.md:106` 을 댔다. 잠긴 조건 문면 `:361` · `:363` 은 대지 않았다.
>   - 원문 머리는 「깨질 곳을 받는다」다(`R/intent.md:20-21`). 완성 장면 문서는 「무엇이 이것을 부르는지」라고 적었다(`01-completion-scenes.md:46-47`).
>   - 효과 기록은 호출자 **자리**가 걸음 하나를 없앴다고 적었다(`R/effect/04-readnote.md:15` · `:29`).
>   - 고친 C 는 옛 C 를 포함한다. 그래서 통과 조건이 기계적으로 약해진다. 게다가 두 결과를 다 본 뒤에 빨강을 초록으로 뒤집는다(반론 4). 이것은 `SKILL.md:329` 가 경계한 모양 그대로다.
>
> **반증 + 이슈 길은 어느 갈래에서도 밀린다.**
> - ⓒ 가 의도 **밖**이면: 옛 문면은 의도를 제대로 못 잰다. 그러면 처방은 정정이다(`SKILL.md:322` 「고치고 계속 간다」).
> - ⓒ 가 의도 **안**이면: 이슈로 떼는 것은 다음 회차의 짐이다. 자기 완결성이 이번 회차의 확대를 요구한다(`SKILL.md:366-368` · `:378`).
> - 그래서 반증 + 이슈를 고르지 않는다.
>
> **반론 1 이 연 `R/approval.md` 를 받는다.**
> - 까닭 하나: 그 파일은 토론 산출이 아니라 규약 §4 의 승인 기록이다. 메인이 준 내 허용 목록에 들어 있다.
> - 까닭 둘: 내가 직접 열어 문장을 확인했다(`R/approval.md:13-14`). 커밋 `66fc201`(2026-09-13 20:49:39 · 「잠긴 의도가 승인됐다」)의 파일인 것도 확인했다.
> - 까닭 셋: 반(反)이 허용 밖에서 열었다는 사실을 스스로 밝혔다. 그 좌표는 원문 확인으로 오염 우려가 풀린다. 이 판의 독립성이 막으려는 것은 다른 역할의 산출이 섞이는 것이지, 잠긴 1차 기록을 읽는 것이 아니다.
>
> **자기 완결성 시험의 답**(물음: 「`touch` 에 자리를 싣지 않으면 다음 회차에 짐이 되거나 의도가 변질되나」)
> - **장면·계획 문면으로 재면 짐도 변질도 없다.** 요구는 수이고 `touch` 가 이미 싣는다(`R/effect/03-touch/05-codexHostAdapter.txt:13` 「호출자 2」). 그 둘이 경계를 넘었다는 것은 같은 원천 목록이 보인다(`R/effect/04-callers/05-codexHostAdapter.txt:4-5`). 코드도 같은 해소 함수와 같은 원천을 쓴다(`crates/pal-query/src/lib.rs:571-580` · `:838-841`).
> - **잠긴 조건 문면과 원문 머리로 재면 짐이 된다.** 좌표는 `R/intent.md:361` · `:363` · `:20-21` · `R/approval.md:11` · `:13-14` 다.
> - **초안이 댄 변질 사유 (ii) 는 서지 않는다.** `E3` 는 문면대로의 `E4` 가 커밋된 뒤에 이미 소유자가 닫았다(커밋 `42af275` 2026-09-13 23:51:45 → `6145849` 2026-09-14 06:40:10 「[승격] E3 는 대조 불가」).
> - **답은 ⓒ 가 의도 안인가에 달린 조건부다.** 그 갈림은 소유자의 뜻을 묻는 물음이고, 합(合)이 혼자 정하지 않는다.

합(合) R2 가 판정과 함께 낸 「초안을 어떻게 고치나」 그대로(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis-r2.md:70-111`):

> 소유자가 칸 1 에 **「받는다」**로 답한 경우에 실행한다. 「안 받는다」면 아래 「소유자에게」의 둘째 갈래로 간다.
>
> 1. **분류 근거를 바꾼다**(반론 3).
>    - 「ⓓ 만큼 는다 → 정정」을 지운다.
>    - 대신 적는다: 「옛 문면은 의도 밖 몫 ⓒ 를 재어 의도를 제대로 못 쟀다(`SKILL.md:322`). 고친 뒤 의도의 양(ⓐ·ⓑ)은 같다. ⓓ 는 측정의 정직성으로 따로 적는다.」
>    - 그리고 소유자가 칸 1 에서 ⓒ 를 의도 밖으로 답했다는 좌표(`R/intent.md` `## 승격` 새 행)를 단다.
> 2. **사전 등록 주장을 지운다**(반론 4). `## 개정` 행에 「이 읽기 규칙은 질의 출력으로 잰 통과(`b61df62` · `1d7b3b8`)와 문면대로 잰 반증(`42af275` · `2d22dd4`)을 **둘 다 본 뒤** 썼다」를 적는다. 개정 커밋이 재측정보다 앞서는 순서는 지킨다. 다만 그것을 사전 등록이라 부르지 않는다.
> 3. **`E2` ⑵ 고친 문면**(반론 10 · 12)
>    > `호출자` 에 다른 파일의 참조 ≥ 1(하한). `touch` 는 호출자의 수만 싣는다(`--json` 의 `facts.present.callers`). ⑵ 는 **증인 심볼이 하나 이상** 있을 때 선다. 증인 심볼은 넷을 모두 채운다.
>    > - `touch` 의 수가 ≥ 1 이다.
>    > - 그 `touch` 가 후보 화면이 아닌 한 심볼의 답이다.
>    > - 같은 스냅샷 꼬리표의 `pal query symbol.callers` 목록이 비지 않았고 줄 수가 그 수와 같다.
>    > - 목록의 경로 가운데 심볼 자신의 파일이 아닌 것이 ≥ 1 이다.
>    >
>    > 규칙을 못 채운 심볼은 산출에 이름으로 적는다. 증인의 성립은 뒤집지 않는다.
>    - 「같은 지목 문자열」 갈래는 뺀다. 이 회차 산출로 설 수 없다.
> 4. **`E4` 의 C 고친 문면**
>    > 봉인 심볼들의 `touch` 가 **센** 호출자의 파일 집합이다.
>    > - 수가 0 인 심볼은 빈 집합이다.
>    > - 수가 ≥ 1 인 심볼은 위 규칙의 앞 셋(한 심볼 답 · 같은 스냅샷 · 줄 수 같음)을 채운 목록의 경로다.
>    > - 수가 ≥ 1 인 심볼 가운데 **하나라도** 규칙을 못 채우면 `E4` 는 대조 불가다.
>    - 나머지 글자와 끝 문장은 그대로 둔다.
> 5. **오라클과 음성 대조**
>    - `R/oracle/E2-scene.py` ⑵ 와 `R/oracle/E4-breakage.mjs` C 를 위 규칙에 맞춘다.
>    - 산출 머리에 심볼마다 네 칸을 싣는다: 수 · 한 심볼 답인가 · 목록 줄 수 · 지목 여부. 지목 여부의 출처는 파일 이름 `-pick` 과 후보 화면이지, 머리 해시가 아니다(반론 12).
>    - **음성 대조 ①**: 증인 `05` 의 수를 바꿔 넣은 입력으로 ⑵ 가 증인을 잃는다. `E4` 는 대조 불가다.
>    - **음성 대조 ②**: 목록 줄 하나를 지운 입력으로 `E4` 가 대조 불가다.
>    - **음성 대조 ③**: 같은 A 워크트리에서 C = 없음으로 돌린 실행이 `src/cli/commands/setup.ts` 의 `262:50` 을 **선언 안 1** 로 다시 낸다(`R/oracle/E4-literal-touch-c.txt:14` · `:22` 재현). 이것이 없으면 새 실행의 「0」이 오라클 고장과 갈리지 않는다.
>    - 두 실행은 A(`898a479`) 워크트리에서 나란히 돌린다. 산출 머리에 `git rev-parse HEAD` 와 `git status --porcelain` 을 싣는다.
> 6. **옛 `E4-breakage.txt` 를 판정에서 빼는 까닭을 고친다**(반론 11). 「복제본이 뒤에 B 로 갔다」를 지운다. 대신 적는다: 「reflog 상 실행 무렵 HEAD 는 A 였다(`ditto-effect/.git/logs/HEAD` · `b61df62`). 그러나 커밋 전 워크트리 편집이 산출에 안 적혀 있다.」
> 7. **변질 사유 (ii) 를 지운다**(반론 2). 사유는 (i) 하나로 적는다.
> 8. **대안표의 확대 행**(반론 5 · 6)
>    - 「빠지는 근거」 칸에서 ②③④ 를 뺀다. ①(원 의도의 완결에 필요하지 않다 — 소유자 칸 1 의 답)만 둔다. ②는 채택되면 따를 **검증 표면**으로 옮긴다.
>    - `:102` 와 `:172` ④ 를 「자리의 값은 관측됐다 — 안내 없는 질의로 왔다(`R/effect/04-readnote.md:15` · `:29`)」로 고친다.
> 9. **이슈 문장과 자리**(반론 7 · 8)
>    - 문장: 「`pal touch` 는 호출자의 수와 하한 단서만 싣고, 호출자 자리(파일·줄)도 그 자리를 펴는 `symbol.callers` 로 가는 안내도 싣지 않는다 — 자리는 안내 없는 다른 명령으로만 얻는다.」 이 회차 약속의 미충족으로 적지 않는다. 소유자 답의 좌표를 단다.
>    - 적는 자리: `R/intent.md` `## 범위 밖` 에 이슈 번호와 함께 한 줄. 선례는 `:291`(`d1f8ddb` 가 승인 `66fc201` 뒤에 더했다)이다.
>    - 게이트 `## 범위 밖`(`SKILL.md:741`)에도 한 줄 싣는다.
>    - 종료 보고의 `## 다음 회차가 받는 것` 에는 싣지 않는다.
> 10. **기계에 넘기는 것.** 문면 오라클 재실행이 판정한다. `crates/` 가 안 바뀌므로 `cargo xtask test` · `cargo xtask check` · `D1` · `B4` 행은 이 길에서 새로 잴 변경이 없다(발췌 기계 표). 게이트와 종료 보고에는 「`touch` 화면에서 자리로 받는다는 서지 않았다」를 싣는다(초안 `:138` 그대로).

**앞 라운드 기록.** 합(合) R1 의 판정 절 그대로(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis.md:13-57`):

> **수정** — 초안이 고른 길(§5 확대)을 **받지 않는다.** 대신 반(反) 행 4 가 받친 **셋째 길**을 고른다.
>
> **고른 길**: `touch` 에 호출자 자리를 싣지 않는다. `E2` ⑵ 와 `E4` 의 문면을 **§5 정정**으로 고친다. 두 조건이 원래 재려던 것은 `touch` 가 **센** 호출자이고, 그 목록은 **같은 `p.callers`** 를 펴는 `pal query symbol.callers` 에서 뽑는다. 이 읽기는 **수가 같다는 동치 검사**가 설 때만 쓴다. 고친 문면을 `## 개정` 에 먼저 커밋한 뒤 두 오라클을 **같은 워크트리**에서 다시 돌린다. `touch` 가 자리를 싣는 것과 자리를 펴는 질의로 가는 안내는 `## 범위 밖` 에 한 줄로 두고 이슈 하나로 뗀다. 이것은 빚이 아니다.
>
> **§5 분류 한 낱말: 정정.**
>
> ### 왜 확대가 아닌가 — 규약의 자로
>
> 규약에서 확대 행의 조건은 「원 의도 충족에 필요하다」(`SKILL.md:323`)다. 안 늘리는 사유는 「원 의도의 완결에 필요하지 않다」 하나뿐이다(`SKILL.md:386-387`). 잠긴 의도가 호출자에 대해 요구한 것은 네 자리에서 모두 **수**다.
>
> - 원문의 완성 장면: 「`호출자` 가 파일 경계를 넘은 실제 **수**로 나온다」(`R/intent.md:49`).
> - 계획 ㈀ 의 장면 조건: 「파일 경계를 넘은 실제 수」(`R/intent.md:131`). ㈀ 는 해소기 작업이다. 화면을 바꾸는 계획 줄은 ㈁ · ㈂ · ㈃ 뿐이고, 호출자 자리를 싣는 줄은 계획에 없다(`:131-135`).
> - 계획 세부가 바로 이 긴장을 이미 풀었다: 「장면 조건의 「실제 수」는 「파일 경계를 넘은 하한」으로 읽고, 화면이 그 사실을 말한다. **실제 파손 자리와의 대조는 `E3` 가 진다**」(`R/intent.md:165`). 사전부검 R2 2 의 처리 방침도 같다(`:195`).
> - `E4` 는 문면 끝에 제 목적을 적었다: 「화면이 「세지 않는다」고 말한 꼴 밖이 **0**」(`R/intent.md:363`). 오라클 머리 주석도 같은 목적이다(`R/oracle/E4-breakage.mjs:8-11`). 그러니 `E4` 는 하한 단서가 정직한지 재는 도구다.
>
> 초안의 핵심 근거는 원문 머리의 한 줄 「깨질 곳을 받는다」(`R/intent.md:20-21`)다. 그런데 소유자는 같은 원문의 「하려는 것」에서 그 줄을 수로 구체화했다(`:49`). 범위 규칙은 「위 장면을 막는 것만」 들인다고 했다(`:59`). 계획 세부 `:165` 는 자리 대조를 화면 밖(`E3` → CA2-07 이 뗀 `E4`, `:243`)에 두었다. 그러므로 「`touch` 가 파일을 낸다」는 `E2` ⑵ · `E4` 의 문면은 의도를 옮겨 적은 것이 아니다. **제품 출력을 잘못 가정한 측정 결함**이다. 결함이 조건 쪽에 있으므로 고치는 자리도 조건이다.
>
> ### 왜 정정인가 — 「고친 뒤 재는 의도의 양」
>
> 규약의 자는 「고친 뒤 그 검사가 재는 의도의 양이 늘었으면 정정, 줄었으면 완화」다(`SKILL.md:329`).
>
> - **빠지는 것**: 「사용자가 `touch` 화면에서 호출자 파일을 **본다**」. 이것은 장면(`:49`) · 계획 장면 조건(`:131`) · 계획 세부(`:165`) 어디에도 잠겨 있지 않다. 그러니 조건이 의도 **밖**을 재던 몫이다. 빠져도 의도의 양은 줄지 않는다.
> - **그대로인 것**: `E2` ⑵ 가 재는 「`touch` 의 호출자 수가 다른 파일의 참조를 담는다」와 `E4` 가 재는 「`touch` 가 센 호출자 밖의 깨진 자리는 전부 최상위다」.
>   - `touch` 의 수는 `p.callers(symbol.id)?.len()` 이다(`crates/pal-query/src/lib.rs:839`).
>   - `symbol.callers` 는 같은 `p.callers` 를 펴되 `p.symbol(id)` 가 `None` 인 것을 뺀다(`lib.rs:576-580`).
>   - 그래서 **심볼마다 두 수가 같을 때만** 목록이 곧 `touch` 가 센 것이다. 동치 검사가 그 조건을 기계로 건다.
> - **늘어나는 것**: 동치 검사. 옛 오라클(`R/oracle/E2-scene.py:53-60` · `R/oracle/E4-breakage.mjs:39-45`)은 이 검사 없이 질의 출력을 읽었다.
>
> 결론: 의도의 양은 **줄지 않고**, 동치 검사만큼 **는다**. 그러므로 정정이다. 게이트의 「분류를 갈라내지 못하면 올린다」에는 걸리지 않는다. 남는 짐 하나는 아래 「내가 못 정한 것」 1 에 적는다.
>
> ### 자기 완결성 시험 — 「안 하면 다음 회차에 짐이 되거나 의도가 변질되나」
>
> **답: 아니다.** 단, 아래 두 가드를 지킨다는 조건에서다.
>
> 1. **짐이 남지 않는다.**
>    - 정정 뒤 두 조건은 이 회차의 산출로 잰다. P 는 `R/effect/05-tsc-after-A.txt`, D 는 봉인 과제, C 의 원천은 `R/effect/04-callers/` 다.
>    - 새 봉인 · 새 복제본 · 새 `touch` 호출이 들지 않는다.
>    - 이 의도의 조건 중 다음 회차로 넘어가는 것이 없다.
> 2. **의도가 변질될 자리는 초안이 짚은 흐름 하나다.** 앞 오라클 둘이 등록 없이 질의 출력을 읽었다(F6 · F7 · F17). 그 흐름을 막는 방법은 둘이다.
>    - (가) 읽는 자리를 **문면에 등록한 뒤** 잰다. 재고 나서 문면을 맞추지 않는다.
>    - (나) 게이트와 종료 보고가 「`touch` 는 호출자 수와 하한 단서를 싣고, 호출자 자리는 싣지 않는다. 효과 실행에서 자리는 안내 없는 `symbol.callers` 에서 왔다(`R/effect/04-readnote.md:12` · `:29`)」를 **사실로** 적는다. 원문 한 줄 「깨질 곳을 받는다」(`:20-21`)가 `touch` 화면에서 **자리로는** 서지 않았다는 것도 함께 적는다.
>    - 둘 중 하나라도 빠지면 사실이_아닌_것을_사실로 다.
> 3. 자리를 싣는 것의 값은 효과 실행에서 **관측되지 않았다**(초안 `p2-thesis.md:87`). 계획은 그 값을 요구하지 않았다(`:165`). 그러니 이것은 「하면 좋아진다」다(`SKILL.md:377`). 규약은 이 의도가 답할 물음이 아닌 것을 빚으로 두지 않고 `## 범위 밖` 에 한 줄로 둔다고 정한다(`SKILL.md:316`).

합(合) R1 의 「초안을 어떻게 고치나」 그대로(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis.md:80-110`):

> 1. **판정 절을 바꾼다.** 「§5 확대 — `touch` 가 자리를 싣는다」를 빼고, 「§5 정정 — `E2` ⑵ · `E4` 의 C 를 『`touch` 가 센 호출자』로 고치고 동치 검사를 건다」로 적는다. `D1` 모집단 목록 수정(초안 ① 의 딸린 정정)은 새 화면이 없으므로 뺀다.
>
> 2. **`## 개정` 에 한 행을 먼저 커밋한다.** 재측정보다 앞서야 한다. 그래야 오라클이 문면이 아닌 자리를 읽지 않는다(측정이_죽은_가지 방지). 고친 문면은 아래와 같다.
>    - **`E2` ⑵ (고친 문면)**: 「`호출자` 에 다른 파일의 참조 ≥ 1(하한). `touch` 는 호출자의 **수**만 싣는다. 그 수의 내용은 같은 스냅샷에서 같은 지목으로 부른 `pal query symbol.callers` 목록으로 읽는다. 이 목록은 같은 호출자 집합을 편다. 봉인 심볼 **전부**에서 `touch --json` 의 `facts.present.callers` 와 그 목록의 줄 수가 **같을 때만** 목록을 `touch` 의 수의 내용으로 읽는다. 하나라도 다르면 ⑵ 는 `대조 불가` 다.」
>    - **`E4` (고친 문면의 C 한 구절)**: 「봉인 심볼들의 `touch` 가 **센** 호출자의 파일 집합 C. `touch` 는 수만 싣는다. 그래서 같은 스냅샷의 `pal query symbol.callers` 목록에서 뽑고, 심볼마다 목록 줄 수가 `touch --json` 의 `facts.present.callers` 와 같음을 산출에 싣는다. 다르면 `대조 불가` 다.」 나머지 글자와 끝 문장(「화면이 「세지 않는다」고 말한 꼴 밖이 **0**」)은 그대로 둔다.
>    - 개정 행의 「왜」 칸에는 이렇게 적는다: 조건이 `touch` 가 파일을 싣는다고 잘못 가정했다(F1~F4). 잠긴 읽기는 수 · 하한 · 파손 대조를 효과 쪽에 둔다(`R/intent.md:49` · `:131` · `:165` · `:363` 끝). 동치 검사를 더해 재는 양이 줄지 않는다(`SKILL.md:329`).
>
> 3. **오라클 둘을 고친다.** 새 측정 장치가 아니라 등록 오라클을 고친 문면에 맞추는 것이다(`R/intent.md:69`).
>    - `R/oracle/E2-scene.py` ⑵(`:53-60`)에 동치 검사를 더한다. 13 심볼의 `03-touch/*.json.txt` 수와 `04-callers/*.txt` 줄 수를 대고, 하나라도 다르면 ⑵ 를 `대조 불가` 로 낸다.
>    - `R/oracle/E4-breakage.mjs` 에 같은 동치 검사를 넣고, 산출 머리에 13 쌍을 싣는다.
>    - **음성 대조**: 이미 `R/oracle/E4-literal-touch-c.txt` 가 있다. C = 없음이면 선언 안 1 이다. 이 산출을 **같은 워크트리에서** 질의 C 실행과 **나란히 다시 돌려** 남긴다. 그러면 F8 의 「주석 — 다시 돌리지 않았다」가 실행으로 바뀐다.
>    - `E2` ⑵ 에도 음성 대조를 둔다. 동치 검사에 수 하나를 바꿔 넣은 입력을 주면 `대조 불가` 가 나와야 한다.
>
> 4. **판정값을 적는 법.** 재실행 전에는 두 조건 다 **미측정**이다. 앞 산출은 이렇게 **예고**한다.
>    - `E2` ⑵: `R/oracle/E2-scene.txt:9-11` 의 2 건 · 동치 13/13(내 일회성 확인) → 통과.
>    - `E4`: `R/oracle/E4-breakage.txt:14` → 선언 안 0 → 통과. 단 이 산출은 워크트리 `ditto-effect` 에서 났고, P 가 같다는 것은 두 파일에서 읽은 관측이다(초안 `p2-thesis.md:106`).
>    - **예고를 판정으로 세지 않는다.** 판정값은 3 의 재실행 산출이 정한다.
>
> 5. **`## 범위 밖` 에 한 줄, 이슈 하나.**
>    - 이슈 한 문장: 「`pal touch` 는 호출자의 수와 하한 단서만 싣고, 호출자 자리(파일·줄)도 그 자리를 펴는 질의 `symbol.callers` 로 가는 안내(ADR-0011 의 `Fold.unfolded_by` 꼴)도 싣지 않는다. 그래서 사용자는 「무엇이 깨지나」의 자리를 안내 없는 다른 명령으로만 얻는다(`R/effect/04-readnote.md:12` · `:29` · `R/effect/03-touch/05-codexHostAdapter.txt:48`).」
>    - 범위 밖 사유: 장면(`:49`)과 계획 세부(`:165`)가 호출자를 하한 수로 잠갔고, 자리를 싣는 것은 장면을 막지 않는다(`:59`). 규약상 이 의도가 답할 물음이 아니다(`SKILL.md:316`).
>
> 6. **게이트와 종료 보고에 사실 두 줄을 싣는다**(자기 완결성 가드 (나)).
>    - `touch` 는 자리를 싣지 않았고, 효과 실행에서 자리는 질의 출력에서 왔다.
>    - 원문 한 줄 「깨질 곳을 받는다」는 `touch` 화면에서 **자리로는** 서지 않았고, 계획 세부 `:165` 의 읽기(하한 수 · 단서 · `E3`/`E4` 대조)로 섰다.
>
> 7. **초안의 사실 주장 하나를 고친다.** 초안 ⑤ 의 「원문 장면 F9 는 글머리 「실제 수」로는 선다(`03-touch/05:13`)」에는 「그 수가 파일 경계를 넘었다는 것은 `touch` 출력이 아니라 같은 원천의 질의 출력(`04-callers/05:4-5`)이 보인다」를 덧붙인다.
>
> 8. **회귀 표면.** 셋째 길은 `crates/` 를 안 바꾼다. 기계 표 2~4 행(`cargo xtask test` · `check` · `D1` · `B4`)이 새로 잴 변경이 없다. 행 11 의 `scripts/s2-verify.py:140-141` 도 칸 꼴이 안 바뀌어 그대로다. `E1` 은 새 `touch` 출력이 없으므로 행 9 가 걱정한 덮어쓰기 자리가 생기지 않는다.

**정(正)의 판정 문장.** R1 그대로(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-thesis.md:12-14`):

> **`touch` 의 사람 화면과 `--json` 에 호출자 자리(파일·줄)를 싣는 변경을 §5 「확대」로 이 회차에 넣는다. 새 바이너리로 봉인 HEAD 에서 봉인 심볼 13 의 `touch` 를 다시 불러, `E2` ⑵ · `E4` 는 글자를 안 바꾸고 문면 오라클로 다시 잰다. 효과 장면(`E1` 걸음 순서 · `E3` 판정)은 다시 뜨지 않는다.**

R2 그대로(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-thesis-r2.md:12-19`):

> **`touch` 에 호출자 자리를 싣지 않는다. 대신 `E2` ⑵ 와 `E4` 의 C 를 §5 정정으로 「`touch` 가 센 호출자」로 고친다. 그 내용을 읽는 규칙은 둘로 잠근다 — 수가 0 이면 빈 집합이고 목록을 읽지 않는다. 수가 1 이상이면 같은 심볼로 풀린 `symbol.callers` 목록을 읽되, 줄 수가 같을 때만 읽는다. 이 규칙을 `## 개정` 에 먼저 커밋한 뒤 두 오라클을 다시 돌린다. 자리 싣기는 이 의도 밖이므로 `## 범위 밖` 에 한 줄 적고 이슈 하나로 뗀다.**
>
> - **§5 분류: 정정.** 고친 뒤 재는 의도의 양이 줄지 않고 는다. 까닭은 아래 대안표의 「정정」 행과 답 1 에 있다.
> - **R1 합(合)과의 관계.** 방향(정정)은 받는다. 두 곳은 **뒤집는다.**
>   - (가) R1 합의 동치 검사는 「심볼 **전부**에서 수와 목록 줄 수가 같다」다(`R/dialectic/p2-synthesis.md:85-86`). 이 검사는 06~10 에서 **빈 검사**다. 근거 5 에 적었다.
>   - (나) 이슈를 종료 보고의 어느 절에 싣나를 R1 합이 정하지 않았다. `SKILL.md:798` 과 부딪친다. 답 5 에 적었다.

## 반론 처리 — 전부

반(反)이 번호를 붙인 반론은 **R1 열하나 · R2 열둘 · 모두 스물셋**이다. 아래 `### ` 항 **스물둘**이 합(合)이 채택한 것이다 — R1 의 1 · 2 · 4 · 5 · 6 · 7 · 8 · 9 · 10 · 11(합(合) R1 `## 채택한 반론` 표 행 순서) · R2 의 1 · 2 · 3 · 4 · 5 · 6 · 7 · 8 · 9 · 10 · 11 · 12(합(合) R2 같은 표 행 순서). **기각된 하나(R1 반론 3)는 `## 내가 기각한 것` 표에만 싣는다** — 두 자리에 올리면 원장에서 두 번 세어진다. 항 제목은 합(合)이 채택 표의 반론 칸에 옮겨 적은 문장 그대로다. 불릿의 원문은 스크립트로 원 파일의 표 칸을 뽑아 붙였다.

### 장면과 계획이 요구하는 것은 수다. 자리 싣기는 「장면을 막는 것」이 아니므로 §5 확대가 서지 않는다

- 모집단: 회차기록
- 유효성: 참
- 해악도: 실패
- 어디가 걸리나: `.palimpsest/rounds/2026-09-13-first-release-elsewhere/intent.md:49`
- 판 라운드: R1 · 반(反) 반론 1 · **채택**
- 원문 자리: 반(反) `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-antithesis.md:28` · 합(合) `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis.md:63`
- 반(反) 반론 칸 원문: **초안의 반증 신호 2 를 친다.** 원문의 장면 줄과 계획이 요구하는 것은 파일 경계를 넘은 **수**이고, 그 수는 이미 선다. 범위 규칙 「장면을 막는 것만」으로 재면 자리 싣기는 장면을 막는 것이 아니다. 원문은 이 걸음이 「기능 추가」가 아니라고도 적었다. 그러니 §5 확대 행의 「원 의도 충족에 필요하다」가 서지 않는다. 초안은 원문의 한 줄 머리 「깨질 곳을 받는다」를 소유자가 그 줄을 구체화한 장면·계획보다 앞에 두었다. 이것은 기본값 「안 늘린다」를 넘는 근거가 못 된다.
- 반(反) 좌표 칸 원문: `R/intent.md:49`(장면 「실제 수」) · `:131`(㈀ 장면 조건 「파일 경계를 넘은 실제 수」) · `:59`(범위 규칙) · `:19`(「기능 추가도 … 아니다」) · `R/effect/03-touch/05-codexHostAdapter.txt:13`(「호출자 2」) · `SKILL.md:323` · `:377` · 초안 `p2-thesis.md:124`(반증 신호로 적기만 하고 답하지 않았다)
- 반(反) 유효성 칸 원문: 참 — 초안은 이 자리를 반증 신호로만 적었다. `:131` · `:19` 는 초안에 없는 새 좌표다
- 반(反) 해악도 칸 원문: 실패 (§5 분류를 어긴다 — 「하면 좋아진다」를 확대로 편입)
- 합(合) 채택 사유 원문: 좌표를 열어 확인했다(`R/intent.md:49` · `:59` · `:131` · `:19`). 반론이 대지 않은 `:165` · `:195` 가 이 반론을 더 세게 받친다(위 「왜 확대가 아닌가」). 초안은 이 자리를 반증 신호 2 로 적기만 하고 답하지 않았다(`p2-thesis.md:124`). ⚠ 한 군데 고친다: 반론의 「그 수는 이미 선다」는 `touch` 출력만으로는 서지 않는다. `R/effect/03-touch/05-codexHostAdapter.txt:13` 「호출자 2」 는 그 둘이 다른 파일인지 말하지 않는다. 경계를 넘었다는 것은 `R/effect/04-callers/05-codexHostAdapter.txt:4-5` 에서 온다. 이 결함은 정정 문면의 동치 검사가 메운다
- 합(合) 해악도 칸 원문: 실패 — 반(反)과 같다
- 이 항의 매김: 모집단은 취합자 · 어디가 걸리나는 반(反) 좌표 칸에서 저장소 트리에 있는 첫 파일 좌표 · 유효성은 반(反) 값 · 해악도는 합(合) 값

### `:165` 는 「실제 수」를 하한으로 읽고 파손 대조를 `E3` → `E4` 에 넘겼다. `E4` 는 하한 단서의 정직성을 재는 도구다

- 모집단: 회차기록
- 유효성: 참
- 해악도: 거짓신호
- 어디가 걸리나: `.palimpsest/rounds/2026-09-13-first-release-elsewhere/intent.md:165`
- 판 라운드: R1 · 반(反) 반론 2 · **채택**
- 원문 자리: 반(反) `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-antithesis.md:29` · 합(合) `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis.md:64`
- 반(反) 반론 칸 원문: **초안의 반증 신호 1 을 친다.** `:165` 는 「실제 수」를 하한으로 읽는다. 「실제 파손 자리와의 대조」는 화면이 아니라 효과 판정(`E3`)에 넘겼고, CA2-07 이 그 부분을 `E4` 로 뗐다. `E4` 의 합격선도 「화면이 『세지 않는다』고 말한 꼴 밖이 0」이다. 즉 `E4` 는 A7 하한 단서가 **정직한가**를 재는 도구다. C 는 그 도구의 입력이고, `touch` 가 사용자에게 파일을 건네는가를 재는 조건이 아니다. 초안 ② 의 1 「측정 꼴이 수를 『`touch` 가 낸 파일』로 읽었다」는 이 목적을 뺀 읽기다.
- 반(反) 좌표 칸 원문: `R/intent.md:165` · `:195`(R2 2 처리 방침 「하한으로 읽고 화면이 말한다 · 파손 대조는 E3」) · `:243`(CA2-07) · `:363` 끝 문장 · `R/oracle/E4-breakage.mjs:8-11`(합격선 주석이 같은 목적을 적는다) · 초안 `p2-thesis.md:37`
- 반(反) 유효성 칸 원문: 참 — 초안은 `:165` 를 반증 신호로만 적었다. `:195` · `:363` 끝 문장 · `E4-breakage.mjs:8-11` 은 새 좌표다
- 반(反) 해악도 칸 원문: 거짓신호 (근거 인용의 해석이 틀렸다)
- 합(合) 채택 사유 원문: `R/intent.md:165` · `:195` · `:243` · `:363` 끝 문장 · `R/oracle/E4-breakage.mjs:8-11` 가 반론이 적은 그대로다. 초안 ② 의 1(`p2-thesis.md:37`)은 이 목적 문장을 빼고 읽었다
- 합(合) 해악도 칸 원문: 거짓신호 — 반(反)과 같다
- 이 항의 매김: 모집단은 취합자 · 어디가 걸리나는 반(反) 좌표 칸에서 저장소 트리에 있는 첫 파일 좌표 · 유효성은 반(反) 값 · 해악도는 합(合) 값

### 초안이 「조건 문면 고치기」를 완화로 기각한 것은 순환이다. 같은 원천이면 재는 양이 같아 정정이다

- 모집단: 회차기록
- 유효성: 참
- 해악도: 실패
- 어디가 걸리나: `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-thesis.md:115`
- 판 라운드: R1 · 반(反) 반론 4 · **채택**
- 원문 자리: 반(反) `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-antithesis.md:31` · 합(合) `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis.md:65`
- 반(反) 반론 칸 원문: **초안이 버린 「조건 문면 고치기」를 받치는 가장 강한 근거.** 초안은 이 갈래를 「`touch` 가 자리를 안 내도 통과하므로 재는 의도가 준다 → 완화」로 기각했다. 이 기각은 순환이다. 의도의 양이 준다는 말은 「의도가 `touch` 가 자리를 내는 것을 담는다」는 전제에서만 서고, 그 전제가 바로 행 1·2 가 다투는 자리다. `E4` 가 하한 단서의 정직성을 재는 조건이면(행 2), C 를 같은 `p.callers` 에서 나온 `symbol.callers` 출력으로 받아도 재는 양은 같다. 원천이 같다는 것은 초안이 스스로 인정했다. 그러면 이 갈래는 정정이다. 게다가 규약은 정정인지 축소인지가 갈리면 이 자리(정반합)로 가져오라고 했다. 자동으로 완화가 되는 것이 아니다.
- 반(反) 좌표 칸 원문: 초안 `p2-thesis.md:115`(기각 근거) · `:65`(「같은 `p.callers`」) · `crates/pal-query/src/lib.rs:576` · `:839` · `SKILL.md:329` · `:381-383` · `R/intent.md:363`
- 반(反) 유효성 칸 원문: 참
- 반(反) 해악도 칸 원문: 실패 (행 1·2 가 서면 초안은 §5 가 정정이라 부르는 자리에 확대를 고른다)
- 합(合) 채택 사유 원문: 초안의 기각 근거(`p2-thesis.md:115` 「`touch` 가 자리를 안 내도 통과 → 양이 준다」)는 「의도가 자리를 담는다」를 전제로 깐다. 그 전제가 행 1·2 로 무너진다. 같은 원천은 `lib.rs:576` · `:839` 에서 확인했다. 반론에 없던 받침을 하나 더한다: 13 심볼 전부에서 `touch` 의 수와 질의 목록 길이가 같다(내 일회성 확인). 이 반론이 서면 초안은 정정 자리에 확대를 고른 것이 된다
- 합(合) 해악도 칸 원문: 실패 — 반(反)과 같다
- 이 항의 매김: 모집단은 취합자 · 어디가 걸리나는 반(反) 좌표 칸에서 저장소 트리에 있는 첫 파일 좌표 · 유효성은 반(反) 값 · 해악도는 합(合) 값

### 「안내 줄만」 갈래를 초안이 자유 문구로만 상정했다. ADR-0011 의 `Fold{…, unfolded_by: QueryName}` 이 부피를 다른 질의로 옮기는 타입 있는 설계다

- 모집단: 회차기록
- 유효성: 참
- 해악도: 거짓신호
- 어디가 걸리나: `docs/adr/0011-moved-volume-and-unseen-volume-are-different-fields.md:40-45`
- 판 라운드: R1 · 반(反) 반론 5 · **채택**
- 원문 자리: 반(反) `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-antithesis.md:32` · 합(合) `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis.md:66`
- 반(反) 반론 칸 원문: **초안이 버린 「안내 줄만」을 받치는 가장 강한 근거.** 이 제품에는 부피를 다른 질의로 옮기는 설계가 이미 있다. 타입이 있는 `Fold{what, count, unfolded_by: QueryName}` 이다(ADR-0011). ADR 은 「`touch` 의 상위 N 도 서면 `Fold` 항목이 되고 `unfolded_by` 가 후속 질의를 가리킨다」고 방향을 정했다. `touch` 화면은 지금도 「이관 … → `ledger.snapshot` 로 조회할 수 있습니다」를 찍는다. 초안은 이 갈래를 자유 문구 한 줄로만 상정했다. 부를 수 있는 질의 이름을 싣는 이관 항목은 F5(안내가 없다)를 푼다. 그리고 목록을 `touch` 본체에 싣는 초안의 길이 ADR-0011 의 방향과 어긋나는지는 따지지 않았다.
- 반(反) 좌표 칸 원문: `docs/adr/0011-moved-volume-and-unseen-volume-are-different-fields.md:40-45` · `:74-75` · `R/effect/03-touch/05-codexHostAdapter.txt:44-45` · `R/effect/03-touch/05-codexHostAdapter.json.txt:107-115` · 초안 `p2-thesis.md:116`
- 반(反) 유효성 칸 원문: 참
- 반(反) 해악도 칸 원문: 거짓신호 (대안 기각의 근거가 불완전하다)
- 합(合) 채택 사유 원문: 초안의 대안 표(`p2-thesis.md:116`)가 이관 항목 꼴을 따지지 않은 것은 맞다. `touch` 가 이미 `fold.folded[].unfolded_by` 를 싣는다(`R/effect/03-touch/05-codexHostAdapter.json.txt:107-115`). ⚠ 좁혀 받는다: ADR-0011 `:74-75` 의 「`touch` 의 상위 N 건」은 결박 목록의 점진 회상이다(`lib.rs:836-837` 이 `here` · `watching` 에 거는 `회상`). 호출자 목록을 가리키지 않는다. 그러니 「초안의 길이 ADR 방향과 어긋난다」는 입증되지 않았다. 받는 것은 「대안 검토가 불완전하다」까지다
- 합(合) 해악도 칸 원문: 거짓신호 — 반(反)과 같다
- 이 항의 매김: 모집단은 취합자 · 어디가 걸리나는 반(反) 좌표 칸에서 저장소 트리에 있는 첫 파일 좌표 · 유효성은 반(反) 값 · 해악도는 합(合) 값

### 「사용자가 스스로 닿는다」는 ㈁ 의 장면 조건인데, 초안이 목표 2 에 옮겨 붙였다

- 모집단: 회차기록
- 유효성: 참
- 해악도: 거짓신호
- 어디가 걸리나: `.palimpsest/rounds/2026-09-13-first-release-elsewhere/intent.md:131`
- 판 라운드: R1 · 반(反) 반론 6 · **채택**
- 원문 자리: 반(反) `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-antithesis.md:33` · 합(合) `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis.md:67`
- 반(反) 반론 칸 원문: **초안 ② 의 2 는 목표 1 의 장면 조건을 목표 2 에 옮겨 붙였다.** 「사용자가 스스로 닿는다」는 ㈁(승인 대기)의 장면 조건이다. ㈀ 의 장면 조건은 「파일 경계를 넘은 실제 수」뿐이다. 그러니 「목표 2 쪽의 깨질 곳이 그 기준을 못 넘는다」는 잠긴 계획에 없는 기준으로 잰 것이다.
- 반(反) 좌표 칸 원문: `R/intent.md:131` ↔ `:132` · 초안 `p2-thesis.md:42`
- 반(反) 유효성 칸 원문: 참
- 반(反) 해악도 칸 원문: 거짓신호
- 합(合) 채택 사유 원문: `R/intent.md:131`(㈀ · 수) ↔ `:132`(㈁ · 스스로 닿는다). 초안 `p2-thesis.md:42` 가 목표 1 의 자를 목표 2 의 확대 근거로 썼다
- 합(合) 해악도 칸 원문: 거짓신호 — 반(反)과 같다
- 이 항의 매김: 모집단은 취합자 · 어디가 걸리나는 반(反) 좌표 칸에서 저장소 트리에 있는 첫 파일 좌표 · 유효성은 반(反) 값 · 해악도는 합(合) 값

### 닫힘을 잴 「문면 절차」가 실행체로 없다. 새 출력 형식을 정한 손이 파서를 나중에 쓴다

- 모집단: 회차기록
- 유효성: 참
- 해악도: 실패
- 어디가 걸리나: `.palimpsest/rounds/2026-09-13-first-release-elsewhere/oracle/E4-breakage.mjs:5`
- 판 라운드: R1 · 반(反) 반론 7 · **채택**
- 원문 자리: 반(反) `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-antithesis.md:34` · 합(合) `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis.md:68`
- 반(反) 반론 칸 원문: **닫힘을 잴 「문면 절차」가 실행체로 없다.** 초안은 `E2-literal-touch.txt` · `E4-literal-touch-c.txt` 를 낸 절차를 다시 돌린다고 했다. 그런데 두 산출을 넣은 커밋은 `.txt` 만 더했고 스크립트는 `R/oracle/` 에 없다. 등록된 `E4` 오라클은 C 를 질의 출력의 줄 꼴(정규식)로만 뽑는다. 새 `touch` 출력에서 C 를 뽑으려면 파서를 새로 써야 한다. 그 출력의 형식·필드 이름은 초안이 실행 몫으로 미뤘다. 결국 출력 형식을 정한 같은 손이 그 형식에 맞춘 파서를 나중에 쓴다. 음성 대조도 없다.
- 반(反) 좌표 칸 원문: `git show --stat 2d22dd4`(`E2-literal-touch.txt` 한 파일) · `git show --stat 42af275`(`E4-literal-touch-c.txt` · `e3-referee.md`) · `ls R/oracle` · `R/oracle/E4-breakage.mjs:5` · `:40-45` · 초안 `p2-thesis.md:60` · `:68`
- 반(反) 유효성 칸 원문: 참
- 반(反) 해악도 칸 원문: 실패 (회귀·닫힘을 검증할 기록된 경로가 없다. 파서를 출력에 맞춰 쓰면 측정이_죽은_가지로 간다)
- 합(合) 채택 사유 원문: `git show --stat 2d22dd4` 은 `.txt` 한 파일이고, `42af275` 은 `.txt` 와 `e3-referee.md` 다. `R/oracle/` 에 문면용 추출 스크립트가 없다. 등록 오라클은 질의 출력의 줄 꼴만 뽑는다(`E4-breakage.mjs:40-45`). 초안은 형식을 실행 몫으로 뺐다(`p2-thesis.md:60`). 확대 길에서는 닫힘 경로가 기록되지 않은 채 남는다. (셋째 길에서는 이 반론이 사라진다 — 새 출력이 없고, 기존 오라클을 등록 문면에 맞춰 고친다)
- 합(合) 해악도 칸 원문: 실패 — 반(反)과 같다
- 이 항의 매김: 모집단은 취합자 · 어디가 걸리나는 반(反) 좌표 칸에서 저장소 트리에 있는 첫 파일 좌표 · 유효성은 반(反) 값 · 해악도는 합(合) 값

### 자리 목록에 상한이 걸리면 `E4` 닫힘이 정렬·상한에 달린다

- 모집단: 회차기록
- 유효성: 참
- 해악도: 거짓신호
- 어디가 걸리나: `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-thesis.md:60`
- 판 라운드: R1 · 반(反) 반론 8 · **채택**
- 원문 자리: 반(反) `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-antithesis.md:35` · 합(合) `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis.md:69`
- 반(反) 반론 칸 원문: **자리 목록에 상한이 걸리면 `E4` 닫힘이 정렬·상한 선택에 달린다.** 초안은 상한·정렬을 실행 몫으로 뺐다. `touch` 는 이미 결박 목록을 상한과 생략으로 자른다. 그런데 ⑥ 의 닫힘 조건은 「C 가 `setup.ts` 를 담으면」이라고만 적었고, 목록이 **전량**이라는 요구가 없다. 초안의 반증 신호 3 은 `p.symbol` 이 `None` 인 것을 빼는 문제만 다루고, 상한은 다루지 않는다.
- 반(反) 좌표 칸 원문: 초안 `p2-thesis.md:60` · `:104` · `:126` · `crates/pal-query/src/lib.rs:836-837`(`회상(…, ctx.binding_max, elision)`) · `docs/adr/0011-moved-volume-and-unseen-volume-are-different-fields.md:74-75`
- 반(反) 유효성 칸 원문: 참
- 반(反) 해악도 칸 원문: 거짓신호
- 합(合) 채택 사유 원문: 초안 ⑥ 의 닫힘 조건(`p2-thesis.md:104`)에 「전량」 요구가 없다. `touch` 는 목록을 상한으로 자르는 기존 꼴을 쓴다(`lib.rs:836-837`)
- 합(合) 해악도 칸 원문: 거짓신호 — 반(反)과 같다
- 이 항의 매김: 모집단은 취합자 · 어디가 걸리나는 반(反) 좌표 칸에서 저장소 트리에 있는 첫 파일 좌표 · 유효성은 반(反) 값 · 해악도는 합(合) 값

### 새 `touch` 출력을 둘 자리가 정해지지 않았다. `03-touch/` 를 덮어쓰면 `E1` 이 거짓으로 닫힌다

- 모집단: 회차기록
- 유효성: 참
- 해악도: 실패
- 어디가 걸리나: `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-thesis.md:69`
- 판 라운드: R1 · 반(反) 반론 9 · **채택**
- 원문 자리: 반(反) `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-antithesis.md:36` · 합(合) `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis.md:70`
- 반(反) 반론 칸 원문: **새 `touch` 출력을 둘 자리가 정해지지 않았다. 한쪽 실행은 `E1` 을 거짓으로 닫는다.** 초안은 새 출력을 봉인 HEAD 에서 부른 「그 출력들」이라 했다. 새 출력이 `effect/03-touch/` 를 덮어쓰면 문제가 생긴다. `E1` 오라클은 파일이 **처음 들어간** 커밋(`--diff-filter=A`)만 보므로 초록으로 남는다. 그런데 파일 내용은 변경 A·B 뒤에 뜬 것이 된다. 원문 장면의 「손대기 전에」와 `E1` 앵커의 뜻이 어긋난다. 다른 자리에 두면 `E2` 오라클의 파일 목록(glob)과 「문면 절차」를 다시 가리켜야 한다. 초안의 반증 신호 4 는 `E2` ⑴·⑵ 를 섞는 문제만 적었고 `E1` 앵커는 다루지 않았다.
- 반(反) 좌표 칸 원문: 초안 `p2-thesis.md:69` · `:85` · `R/oracle/E1-order.sh:43` · `:47` · `R/oracle/E2-scene.py:23` · `R/effect/05-change.txt:4-5` · `R/intent.md:46` · `:360`
- 반(反) 유효성 칸 원문: 참
- 반(反) 해악도 칸 원문: 실패 (덮어쓰기로 실행하면 완수 조건이 거짓으로 닫힌다)
- 합(合) 채택 사유 원문: `R/oracle/E1-order.sh:43` 은 `--diff-filter=A` 의 첫 커밋만 보고, `:47` 은 `03-touch/head.txt` 를 앵커로 쓴다. 덮어써도 초록이 남는데 내용은 변경 A·B 뒤의 것이 된다(`R/effect/05-change.txt:4-5`). 질문의 형식 요구도 「새 출력을 둘 자리」를 확대 길의 필수로 적었는데 초안에 없다(`p2-thesis.md:69` · `:85`). 덧붙여 효과 복제본은 이미 승인 결박이 있다(`R/effect/04-approve.txt`). 그래서 새 출력의 ⑴ 구역이 옛 출력과 다르다(초안 반증 신호 4)
- 합(合) 해악도 칸 원문: 실패 — 반(反)과 같다
- 이 항의 매김: 모집단은 취합자 · 어디가 걸리나는 반(反) 좌표 칸에서 저장소 트리에 있는 첫 파일 좌표 · 유효성은 반(反) 값 · 해악도는 합(合) 값

### 초안 ② 의 2 가 든 값(사용자가 자리에 닿는다)을 판정은 재지 않는다. 그러면 확대의 받침은 조건 문면 하나만 남는다

- 모집단: 회차기록
- 유효성: 참
- 해악도: 거짓신호
- 어디가 걸리나: `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-thesis.md:87`
- 판 라운드: R1 · 반(反) 반론 10 · **채택**
- 원문 자리: 반(反) `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-antithesis.md:37` · 합(合) `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis.md:71`
- 반(反) 반론 칸 원문: **초안 ② 의 2 가 확대의 값으로 든 것은 사용자가 자리에 닿는다는 효과인데, 판정은 그 효과를 재지 않는다고 스스로 적었다.** 이 회차 효과 실행에서 화면의 기여는 소유자가 이미 「대조 불가 · grep 반사실을 뺄 수 없다」로 받았다. 읽은 줄 기록도 이름별 grep 걸음을 질의 출력 때문에 건넜다고 적었다. 그러면 확대를 받치는 것은 행 1~4 가 치는 조건 문면 하나만 남는다. 효과 쪽 근거는 「하면 좋아진다」와 가려지지 않는다.
- 반(反) 좌표 칸 원문: 초안 `p2-thesis.md:87` · `:132` · `R/intent.md:389-391` · `R/effect/04-readnote.md:15` · `:29` · `SKILL.md:377`
- 반(反) 유효성 칸 원문: 참 — 초안은 `:389-391` 을 봤지만 근거로 쓰지 않았다고 적었다. 반론의 근거로 쓰는 것은 새롭다
- 반(反) 해악도 칸 원문: 거짓신호
- 합(合) 채택 사유 원문: 초안이 스스로 「새 자리 줄의 효과는 관측되지 않았다」고 적었다(`p2-thesis.md:87` · `:132`). 관측되지 않은 값은 「하면 좋아진다」와 가려지지 않는다(`SKILL.md:377`). ⚠ 좌표 하나는 넘쳐 읽었다: `R/intent.md:389-391` 의 승격 4~6 은 ADR-0003 결정 본문 · `shared.ts` wrapper · 결정 본문에 닿는 grep 반사실이다. 호출자 자리에 대한 판정이 아니다. 핵심 논지는 초안 자신의 두 줄로 선다
- 합(合) 해악도 칸 원문: 거짓신호 — 반(反)과 같다
- 이 항의 매김: 모집단은 취합자 · 어디가 걸리나는 반(反) 좌표 칸에서 저장소 트리에 있는 첫 파일 좌표 · 유효성은 반(反) 값 · 해악도는 합(合) 값

### 회귀 표면에 Rust 밖 소비자 `scripts/s2-verify.py` 가 빠졌다

- 모집단: 회차기록
- 유효성: 참
- 해악도: 미관
- 어디가 걸리나: `scripts/s2-verify.py:140-141`
- 판 라운드: R1 · 반(反) 반론 11 · **채택**
- 원문 자리: 반(反) `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-antithesis.md:38` · 합(合) `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis.md:72`
- 반(反) 반론 칸 원문: **회귀 표면에 저장소 안 Rust 밖 소비자가 빠졌다.** `scripts/s2-verify.py` 가 `touch --json` 의 `facts.present.callers` 를 수로 비교한다(`== 0`). 칸의 꼴이 바뀌면 이 발견 노트가 조용히 멈춘다. 기계 표 2~4 행과 초안의 확인 못 한 것 목록은 Rust 만 훑었다.
- 반(反) 좌표 칸 원문: `scripts/s2-verify.py:140-141` · 초안 `p2-thesis.md:142` · `:73-82`
- 반(反) 유효성 칸 원문: 참
- 반(反) 해악도 칸 원문: 미관 (합격선이 아니라 노트다)
- 합(合) 채택 사유 원문: `scripts/s2-verify.py:140-141` 이 `facts.present.callers == 0` 을 읽는다. 칸 꼴이 바뀌면 이 노트가 조용히 멈춘다. 합격선이 아니라 노트다(`:134` 「합격선이 아니다」)
- 합(合) 해악도 칸 원문: 미관 — 반(反)과 같다
- 이 항의 매김: 모집단은 취합자 · 어디가 걸리나는 반(反) 좌표 칸에서 저장소 트리에 있는 첫 파일 좌표 · 유효성은 반(反) 값 · 해악도는 합(合) 값

### 승인 전제가 `E4` 를 짚어 「조건을 약하게 고치지 않는다」였다. 그러니 C 문면을 고치는 것을 혼자 정정으로 닫을 수 없다

- 모집단: 회차기록
- 유효성: 참
- 해악도: 실패
- 어디가 걸리나: `.palimpsest/rounds/2026-09-13-first-release-elsewhere/approval.md:13-14`
- 판 라운드: R2 · 반(反) 반론 1 · **채택**
- 원문 자리: 반(反) `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-antithesis-r2.md:34` · 합(合) `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis-r2.md:51`
- 반(反) 반론 칸 원문: **소유자가 잠긴 의도를 승인한 전제가 `E4` 를 이름으로 짚어 「반증으로 끝나면 반증으로 적는다 — 조건을 약하게 고치지 않는다」였다.** 초안은 문면대로의 반증(F7)을 본 **뒤** `E4` 의 C 문면을 고치고, 스스로 정정으로 분류해 승격 없이 간다. 정정인지 완화인지가 갈리는 자리에서 승인 전제와 부딪치는 변경이다. 그러니 혼자 정정으로 닫을 수 없고 올라가야 한다(`SKILL.md:381-383`). ⚠ 한계: 승인문이 예고한 반증의 기제는 「파일 최상위 참조」이지, 이번의 「C = 없음」이 아니다
- 반(反) 좌표 칸 원문: `R/approval.md:13-14` · `R/approval.md` 커밋 `66fc201` · 초안 `:159`(「`E4`: C 를 … 로 바꾼다」) · `:110`(「필요하지 않다」)
- 반(反) 유효성 칸 원문: 참
- 반(反) 해악도 칸 원문: 실패
- 합(合) 채택 사유 원문: `R/approval.md:13-14` 를 열어 문장이 그대로임을 확인했다. 초안 `:110` 은 승격이 필요 없다고 했고 `:159` 는 C 를 고친다. 고친 C 는 옛 C 를 포함하므로 통과 조건이 약해진다(`E4-literal-touch-c.txt:7` 「없음」 → `E4-breakage.txt:6`). 판정 「반증 1」이 「0」으로 뒤집힌다(`E4-literal-touch-c.txt:22` ↔ `E4-breakage.txt:14`). 완화라면 승격 없이 가는 것이 §5 위반이다(`SKILL.md:324` · `:329`). 반(反)이 적은 한계는 맞다. 승인문이 예고한 기제는 「파일 최상위 참조」다. 그러나 승인문의 금지는 기제를 가리지 않고 「조건을 약하게」에 걸린다. 위 「판정」 절처럼 정정과 완화를 갈라내지 못하므로 위험이 실재한다
- 합(合) 해악도 칸 원문: 실패 — 반(反)과 같다
- 이 항의 매김: 모집단은 취합자 · 어디가 걸리나는 반(反) 좌표 칸에서 저장소 트리에 있는 첫 파일 좌표 · 유효성은 반(反) 값 · 해악도는 합(合) 값

### 답 2 (ii) 가 적은 결과(`E3` 가 틀린 전제를 받는다)는 이미 지나간 일이다

- 모집단: 회차기록
- 유효성: 참
- 해악도: 거짓신호
- 어디가 걸리나: `.palimpsest/rounds/2026-09-13-first-release-elsewhere/intent.md:389-391`
- 판 라운드: R2 · 반(反) 반론 2 · **채택**
- 원문 자리: 반(反) `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-antithesis-r2.md:35` · 합(合) `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis-r2.md:52`
- 반(反) 반론 칸 원문: **답 2 (ii) 가 적은 결과는 이미 일어난 일이다.** 초안은 「정정을 안 하면 `E3` 가 문면대로의 `E4` 산출을 받아 판정이 틀린 전제를 받는다」를 이번에 고칠 사유로 댄다. 그러나 `E3` 는 이미 소유자가 「대조 불가」로 답했다. 그 답은 문면대로의 `E4` 산출이 커밋된 **뒤**에 나왔다. 게다가 초안은 `E3` 가 입력으로 받았던 `E4-breakage.txt` 를 판정 산출에서 뺀다면서, 이미 닫힌 `E3` 에 그것이 무엇을 뜻하는지는 말하지 않는다. (i) 만으로도 사유는 서므로 판정은 안 넘어지지만, 근거 하나가 서지 않는다
- 반(反) 좌표 칸 원문: `R/intent.md:389-391`(승격 4~6 · 소유자의 답) · 커밋 `42af275`(2026-09-13 23:51:45 · 「E4 를 조건 문면대로 다시 쟀다」) → `6145849`(2026-09-14 06:40:10 · 「[승격] E3 는 대조 불가」) · 초안 `:105` · `:144`
- 반(反) 유효성 칸 원문: 참
- 반(反) 해악도 칸 원문: 거짓신호
- 합(合) 채택 사유 원문: 커밋 시각을 확인했다. `42af275`(23:51:45 · 「E4 를 조건 문면대로 다시 쟀다」) 다음이 `6145849`(06:40:10 · 「[승격] E3 는 대조 불가」)다. 승격 답은 `R/intent.md:389-391` 에 있다. (ii) 는 변질 사유로 서지 않는다
- 합(合) 해악도 칸 원문: 거짓신호 — 반(反)과 같다
- 이 항의 매김: 모집단은 취합자 · 어디가 걸리나는 반(反) 좌표 칸에서 저장소 트리에 있는 첫 파일 좌표 · 유효성은 반(反) 값 · 해악도는 합(合) 값

### ⓓ 는 초안 스스로 「측정 자체의 정직성」이라 적었다. 그러니 「ⓓ 만큼 는다 → 정정」은 `SKILL.md:329` 의 자(의도의 양)를 잘못 댔다

- 모집단: 회차기록
- 유효성: 참
- 해악도: 거짓신호
- 어디가 걸리나: `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-thesis-r2.md:88`
- 판 라운드: R2 · 반(反) 반론 3 · **채택**
- 원문 자리: 반(反) `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-antithesis-r2.md:36` · 합(合) `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis-r2.md:53`
- 반(反) 반론 칸 원문: **답 1 의 「ⓓ 만큼 는다 → 정정」은 자를 잘못 댔다.** 규약의 자는 「재는 **의도의** 양」이다. 그런데 초안의 표는 ⓓ 에 「측정 자체의 정직성」이라 적어 의도에 잠기지 않았음을 스스로 인정했다. ⓓ 를 빼면 ⓐ·ⓑ 는 그대로이고 ⓒ 는 빠진다. 그러니 의도의 양은 느는 쪽이 아니라 같거나 줄어든다. 「늘었으면 정정」 쪽에 들지 않으니 정정·완화 판정이 애매한 칸이고, 그 칸은 혼자 정하지 않는다
- 반(反) 좌표 칸 원문: 초안 `:88`(ⓓ 행 · 「측정 자체의 정직성」) · `:90` · `SKILL.md:329` · `:381-383`
- 반(反) 유효성 칸 원문: 참
- 반(反) 해악도 칸 원문: 거짓신호
- 합(合) 채택 사유 원문: 초안 `:88` · `:90` 과 `SKILL.md:329` 를 대 보았다. ⓓ 를 빼면 의도의 양은 기껏 같다. 「늘었으면 정정」 쪽 논증이 서지 않는다. 같은 결론을 받치려면 `:322` 행(「의도를 제대로 못 잰다」)과 ⓒ 가 의도 밖이라는 전제가 필요한데, 그 전제가 이 판의 갈림이다
- 합(合) 해악도 칸 원문: 거짓신호 — 반(反)과 같다
- 이 항의 매김: 모집단은 취합자 · 어디가 걸리나는 반(反) 좌표 칸에서 저장소 트리에 있는 첫 파일 좌표 · 유효성은 반(反) 값 · 해악도는 합(合) 값

### 「`## 개정` 을 재측정보다 먼저 커밋한다」는 형식만 사전 등록이다. 두 결과가 이미 손에 있다

- 모집단: 회차기록
- 유효성: 참
- 해악도: 거짓신호
- 어디가 걸리나: `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-thesis-r2.md:154`
- 판 라운드: R2 · 반(反) 반론 4 · **채택**
- 원문 자리: 반(反) `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-antithesis-r2.md:37` · 합(合) `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis-r2.md:54`
- 반(反) 반론 칸 원문: **「`## 개정` 을 재측정보다 먼저 커밋한다」는 형식만 사전 등록이다.** 두 문면의 결과가 이미 손에 있다. 질의 출력으로 잰 통과(`E2-scene.txt:10` · `:21`, `E4-breakage.txt:14`)와 문면대로 잰 반증(`E2-literal-touch.txt:39`, `E4-literal-touch-c.txt:22`)이다. 초안 스스로 두 조건의 「예고 — 통과」를 적었다. 규칙은 결과를 본 뒤 설계되고, 커밋 순서는 그것을 가리지 못한다. 규약이 「테스트를 약하게 만들어 GREEN 을 얻는 길이 정정으로 위장한다」고 경고한 모양과 같은 모양이다
- 반(反) 좌표 칸 원문: 초안 `:154`(「재고 나서 문면을 맞추면 측정이_죽은_가지다」) · `:131-133` · 커밋 `1d7b3b8`(E2-scene.txt) · `b61df62`(E4-breakage.txt) · `42af275` · `2d22dd4`(문면 재측정) · `SKILL.md:329`
- 반(反) 유효성 칸 원문: 참
- 반(反) 해악도 칸 원문: 거짓신호
- 합(合) 채택 사유 원문: 통과 산출이 먼저 있었다(`b61df62` 22:56:58 · `1d7b3b8` 23:04:40). 문면대로의 반증 산출도 있었다(`42af275` 23:51:45 · `2d22dd4` 06:36:54). 초안은 개정 전에 이미 「예고 — 통과」를 적었다(`:131-133`). 커밋 순서는 결과를 본 뒤의 설계를 가리지 못한다
- 합(合) 해악도 칸 원문: 거짓신호 — 반(反)과 같다
- 이 항의 매김: 모집단은 취합자 · 어디가 걸리나는 반(反) 좌표 칸에서 저장소 트리에 있는 첫 파일 좌표 · 유효성은 반(反) 값 · 해악도는 합(合) 값

### 대안표의 확대 행이 규약이 금한 사유(넓다·어렵다·잴 수 없다)를 「빠지는 근거」로 싣는다

- 모집단: 회차기록
- 유효성: 참
- 해악도: 거짓신호
- 어디가 걸리나: `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-thesis-r2.md:172`
- 판 라운드: R2 · 반(反) 반론 5 · **채택**
- 원문 자리: 반(反) `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-antithesis-r2.md:38` · 합(合) `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis-r2.md:55`
- 반(反) 반론 칸 원문: **대안표의 확대 행이 규약이 금한 사유를 댄다.** ②「바뀌는 표면이 넓다」 · ③「다시 불러야 한다 · 자리를 새로 정해야 한다」는 「생각보다 넓다 · 어렵다」의 꼴이다. ④「이 회차에서 잴 수 없다」도 같다. 초안은 답 2 에서 유일한 사유가 ①이라 적었지만, 표의 「빠지는 근거」 칸은 넷을 나란히 싣는다
- 반(反) 좌표 칸 원문: 초안 `:172` · `:95` · `SKILL.md:331` · `:385-387`
- 반(反) 유효성 칸 원문: 참
- 반(反) 해악도 칸 원문: 거짓신호
- 합(合) 채택 사유 원문: 초안 `:172` 의 ②③④ 는 `SKILL.md:331` · `:385-387` 이 이월 사유가 아니라고 못 박은 꼴이다. 초안 `:95` 가 스스로 유일 사유를 ①로 적은 것과도 어긋난다
- 합(合) 해악도 칸 원문: 거짓신호 — 반(反)과 같다
- 이 항의 매김: 모집단은 취합자 · 어디가 걸리나는 반(反) 좌표 칸에서 저장소 트리에 있는 첫 파일 좌표 · 유효성은 반(反) 값 · 해악도는 합(合) 값

### 「자리를 싣는 것의 값은 효과 실행에서 관측되지 않았다」가 효과 기록과 어긋난다

- 모집단: 회차기록
- 유효성: 참
- 해악도: 거짓신호
- 어디가 걸리나: `.palimpsest/rounds/2026-09-13-first-release-elsewhere/effect/04-readnote.md:15`
- 판 라운드: R2 · 반(反) 반론 6 · **채택**
- 원문 자리: 반(反) `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-antithesis-r2.md:39` · 합(合) `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis-r2.md:56`
- 반(反) 반론 칸 원문: **「자리를 싣는 것의 값은 효과 실행에서 관측되지 않았다」가 효과 기록과 어긋난다.** 읽은 줄 기록은 **자리** 때문에 걸음 하나를 없앴다고 적었다: 함수·상수 여덟의 호출자가 전부 `codex.ts` 안이다 → 이름마다 저장소를 훑는 걸음을 안 한다. 같은 심볼들의 `touch` 는 `호출자 1` 이라는 수만 줬다. 수만으로는 호출자가 파일 안인지 밖인지 모르므로, 이 걸음은 수로는 못 없앤다. 목표 2 의 값이 **자리**에서 관측됐고, 다만 안내 없는 질의로 왔다. 초안은 `:12` · `:25` 만 인용했다
- 반(反) 좌표 칸 원문: `R/effect/04-readnote.md:15` · `:29` · `R/effect/03-touch/01-mcpServersFromToml.txt`(호출자 1 · `R/oracle/E2-literal-touch.txt:7-10` · `:22-24`) · 초안 `:102` · `:172` ④
- 반(反) 유효성 칸 원문: 참
- 반(反) 해악도 칸 원문: 거짓신호
- 합(合) 채택 사유 원문: `R/effect/04-readnote.md:15` 와 `:29` 를 열었다. 호출자가 전부 `codex.ts` 안이라는 자리 정보로 「이름마다 저장소를 훑는 걸음」을 안 했다고 적혀 있다. 같은 심볼의 `touch` 화면은 수만 준다(`R/effect/03-touch/01-mcpServersFromToml.txt:13` 「호출자 1」). 값은 관측됐다. 다만 안내 없는 질의에서 왔다
- 합(合) 해악도 칸 원문: 거짓신호 — 반(反)과 같다
- 이 항의 매김: 모집단은 취합자 · 어디가 걸리나는 반(反) 좌표 칸에서 저장소 트리에 있는 첫 파일 좌표 · 유효성은 반(反) 값 · 해악도는 합(合) 값

### 이슈 문장이 「여기를 바꾸면 무엇이 깨지나」의 자리 공백을 적으면서, 같은 공백을 「이 의도가 답할 물음이 아니다」로 분류한다

- 모집단: 회차기록
- 유효성: 참
- 해악도: 거짓신호
- 어디가 걸리나: `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-thesis-r2.md:120`
- 판 라운드: R2 · 반(反) 반론 7 · **채택**
- 원문 자리: 반(反) `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-antithesis-r2.md:40` · 합(合) `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis-r2.md:57`
- 반(反) 반론 칸 원문: **초안의 이슈 문장과 `## 범위 밖` 분류가 서로 부딪친다.** 이슈 문장은 「「여기를 바꾸면 무엇이 깨지나」의 자리를 사용자가 안내 없는 다른 명령으로만 얻는다」다. 이것은 원문 머리의 약속 「걸린 결정과 **깨질 곳을 받는다**」를 이름으로 짚는 결함 기술이다. 그런 물음을 「이 의도가 답할 물음이 아니다」(`SKILL.md:316`)로 `## 범위 밖` 에 싣는다. 이 판의 급소(ⓒ)가 초안 문장 안에서 이미 반대로 적혀 있다
- 반(反) 좌표 칸 원문: 초안 `:120` · `:126` · `R/intent.md:20-21` · `docs/plan/00-goals.md:177`
- 반(反) 유효성 칸 원문: 참
- 반(反) 해악도 칸 원문: 거짓신호
- 합(合) 채택 사유 원문: 초안 `:120` 과 `:126` 을 `R/intent.md:20-21` 에 대 보았다. 제품 목표 수준의 결함이 이 의도 밖일 수는 있다(`R/intent.md:287` 이 목표를 뺀 선례). 그러나 이 회차의 원문 머리가 바로 「깨질 곳을 받는다」다. 초안 문장이 급소(ⓒ)를 풀지 않은 채 한쪽으로 적었다. 독립 결함이라기보다 반론 1 이 짚은 갈림의 증상이다
- 합(合) 해악도 칸 원문: 거짓신호 — 반(反)과 같다
- 이 항의 매김: 모집단은 취합자 · 어디가 걸리나는 반(反) 좌표 칸에서 저장소 트리에 있는 첫 파일 좌표 · 유효성은 반(反) 값 · 해악도는 합(合) 값

### 종료 보고의 `## 다음 회차가 받는 것` 에 싣는 것은 자리가 틀렸다

- 모집단: 회차기록
- 유효성: 참
- 해악도: 거짓신호
- 어디가 걸리나: `.claude/skills/round/SKILL.md:797`
- 판 라운드: R2 · 반(反) 반론 8 · **채택**
- 원문 자리: 반(反) `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-antithesis-r2.md:41` · 합(合) `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis-r2.md:58`
- 반(反) 반론 칸 원문: **종료 보고의 자리가 틀렸다.** `## 다음 회차가 받는 것` 은 §11 ③ 의 「별도 목록」이다(`SKILL.md:797`). 그 목록은 이 회차가 만든 **장치·회차 기록**에 대한 발견을 담는다(`:906-907`). 이 의도는 (가) 「장치를 안 만든다 — 넘길 것이 애초에 없다」를 골랐다(`R/intent.md:265`, `SKILL.md:914`). `touch` 의 호출자 자리 공백은 **원 의도·저장소** 모집단의 발견이지 별도 목록 몫이 아니다. 그리고 `:316` 대로 범위 밖이면 「빚이 아니라서 닫을 것이 없다」이니 보고에 실을 칸이 없다. 초안의 답 5 는 `:316`↔`:798` 만 대고 이 짝은 안 댔다
- 반(反) 좌표 칸 원문: `SKILL.md:797` · `:906-907` · `:914` · `:316` · `R/intent.md:265-266` · 초안 `:127`
- 반(反) 유효성 칸 원문: 참
- 반(反) 해악도 칸 원문: 거짓신호
- 합(合) 채택 사유 원문: `SKILL.md:797` 은 그 절을 「§11 ③ 의 별도 목록」으로 정했다. `:906-907` 은 별도 목록을 이 회차가 만든 장치와 회차 기록에 대한 발견으로 정했다. 이 의도는 (가)를 골랐다(`R/intent.md:265`, `SKILL.md:914`). 호출자 자리 공백은 원 의도와 저장소 모집단의 발견이다
- 합(合) 해악도 칸 원문: 거짓신호 — 반(反)과 같다
- 이 항의 매김: 모집단은 취합자 · 어디가 걸리나는 반(反) 좌표 칸에서 저장소 트리에 있는 첫 파일 좌표 · 유효성은 반(反) 값 · 해악도는 합(合) 값

### 초안이 안 연 `01-completion-scenes.md` 에 ⓒ 쪽 문장 「무엇이 이것을 부르는지」가 있다

- 모집단: 회차기록
- 유효성: 추정
- 해악도: 거짓신호
- 어디가 걸리나: `docs/plan/01-completion-scenes.md:46-47`
- 판 라운드: R2 · 반(反) 반론 9 · **채택**
- 원문 자리: 반(反) `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-antithesis-r2.md:42` · 합(合) `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis-r2.md:59`
- 반(反) 반론 칸 원문: **초안이 안 연 문서에 ⓒ 쪽 문장이 있다.** 완성 장면의 바깥쪽 순간에서 사용자가 받는 것은 「이 심볼이 무엇을 부르고 **무엇이 이것을 부르는지**」다. 「몇이」가 아니라 「무엇이」다. 다만 같은 문서가 화면은 다시 그리지 않고 `00-goals.md` §1 에 맡기며, 두 문서가 갈리면 그쪽이 이긴다고 적었다. 그 정본 화면(`00-goals.md:106`)은 수다. 그래서 초안을 뒤집지는 못하고, 「틀렸다면」 1 이 짚은 자리에 **가리키는 문장**이 실재한다는 것까지만 선다
- 반(反) 좌표 칸 원문: `docs/plan/01-completion-scenes.md:46-47` · `:18` · `:29` · `:49-50` · 초안 `:181`
- 반(反) 유효성 칸 원문: 추정
- 반(反) 해악도 칸 원문: 거짓신호
- 합(合) 채택 사유 원문: `docs/plan/01-completion-scenes.md:46-47` 을 열어 확인했다. 같은 문서 `:29` 가 `00-goals.md` §1 에 지고, 그 화면은 수다(`00-goals.md:106`). 그래서 초안을 뒤집지 못한다. 초안 「틀렸다면」 1 이 짚은 자리에 가리키는 문장이 실재한다는 것까지만 선다
- 합(合) 해악도 칸 원문: 거짓신호 — 반(反)과 같다
- 이 항의 매김: 모집단은 취합자 · 어디가 걸리나는 반(反) 좌표 칸에서 저장소 트리에 있는 첫 파일 좌표 · 유효성은 반(反) 값 · 해악도는 합(合) 값

### `E2` ⑵ 고친 문면의 「어긋나는 심볼 하나라도 있으면 대조 불가」가 존재 조건(≥ 1)에 비해 넓다

- 모집단: 회차기록
- 유효성: 참
- 해악도: 거짓신호
- 어디가 걸리나: `.palimpsest/rounds/2026-09-13-first-release-elsewhere/intent.md:361`
- 판 라운드: R2 · 반(反) 반론 10 · **채택**
- 원문 자리: 반(反) `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-antithesis-r2.md:43` · 합(合) `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis-r2.md:60`
- 반(反) 반론 칸 원문: **`E2` ⑵ 고친 문면의 「어긋나는 심볼이 하나라도 있으면 ⑵ 는 대조 불가」가 존재 조건에 비해 넓다.** ⑵ 는 「다른 파일의 참조 ≥ 1(하한)」, 곧 존재 조건이다. 수와 줄 수가 같은 증인 심볼(`05`) 하나로 이미 섰는데, 증인과 무관한 심볼 하나의 어긋남이 그것을 대조 불가로 뒤집는다. 선 사실이 「못 쟀다」로 적힌다. `E4` 는 C 가 완전해야 하므로 같은 규칙이 필요하지만, `E2` ⑵ 에는 필요하지 않다
- 반(反) 좌표 칸 원문: `R/intent.md:361` · 초안 `:158` · `:159`
- 반(反) 유효성 칸 원문: 참
- 반(反) 해악도 칸 원문: 거짓신호
- 합(合) 채택 사유 원문: 목록은 `p.symbol(id)` 가 `None` 인 호출자를 심볼마다 따로 뺀다(`crates/pal-query/src/lib.rs:577-580`). 그러니 한 심볼의 어긋남은 그 심볼 안의 원인일 수 있다. 증인 심볼(`05`) 하나의 성립을 무너뜨리지 않는다. 계통의 어긋남(다른 스냅샷)은 「같은 스냅샷」 요건이 따로 막는다. `E4` 는 C 가 완전해야 하므로 전 심볼 규칙이 맞다
- 합(合) 해악도 칸 원문: 거짓신호 — 반(反)과 같다
- 이 항의 매김: 모집단은 취합자 · 어디가 걸리나는 반(反) 좌표 칸에서 저장소 트리에 있는 첫 파일 좌표 · 유효성은 반(反) 값 · 해악도는 합(合) 값

### `E4-breakage.txt` 가 난 순간 `ditto-effect` 의 HEAD 는 A 였다

- 모집단: 회차기록
- 유효성: 참
- 해악도: 미관
- 어디가 걸리나: `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-thesis-r2.md:74`
- 판 라운드: R2 · 반(反) 반론 11 · **채택**
- 원문 자리: 반(反) `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-antithesis-r2.md:44` · 합(合) `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis-r2.md:61`
- 반(反) 반론 칸 원문: **`E4-breakage.txt` 가 난 순간 `ditto-effect` 의 HEAD 는 A 였다.** 초안은 이것을 「확인 못 한 것」 2 로 두었지만 커밋 순서로 가려진다. 복제본 reflog 에서 A 는 22:54:59, B 는 22:58:47 이다. `E4-breakage.txt` 는 그 사이인 `b61df62`(22:56:58)에서 들어왔다. 「B 로 간 뒤 났을 수 있다」는 사실이 아니다. 남는 가능성은 **커밋 전 B 편집이 워크트리에 있었나** 하나다. 판정 산출에서 빼는 결론은 보수적이라 서지만, 초안이 댄 까닭(`05-change.txt:5` — 복제본이 뒤에 B 로 갔다)은 그 결론을 받치지 못한다
- 반(反) 좌표 칸 원문: `…/scratchpad/ditto-effect/.git/logs/HEAD`(A `1789307699` · B `1789307927`) · `git -C …/ditto-effect log -3`(A 22:54:59 · B 22:58:47) · 커밋 `b61df62` 22:56:58 · 초안 `:74` · `:192`
- 반(反) 유효성 칸 원문: 참
- 반(反) 해악도 칸 원문: 미관
- 합(合) 채택 사유 원문: 복제본 reflog 를 열었다. A 는 `1789307699` = 2026-09-13 22:54:59, B 는 `1789307927` = 22:58:47 이다. `b61df62` 는 22:56:58 이다. 초안의 까닭 「뒤에 B 로 갔다」(`:74`)는 그 결론을 받치지 못한다. 옛 산출을 판정에서 빼고 A 에서 다시 돌리는 결론은 커밋 전 워크트리 편집을 모른다는 까닭으로 여전히 선다
- 합(合) 해악도 칸 원문: 미관 — 반(反)과 같다
- 이 항의 매김: 모집단은 취합자 · 어디가 걸리나는 반(反) 좌표 칸에서 저장소 트리에 있는 첫 파일 좌표 · 유효성은 반(反) 값 · 해악도는 합(合) 값

### 머리의 해시는 지목 여부를 가르지 못한다. 동치 규칙의 둘째 갈래(「같은 지목 문자열」)는 이 회차 산출로 설 수 없다

- 모집단: 회차기록
- 유효성: 참
- 해악도: 미관
- 어디가 걸리나: `.palimpsest/rounds/2026-09-13-first-release-elsewhere/effect/03-touch/05-codexHostAdapter.txt:2`
- 판 라운드: R2 · 반(反) 반론 12 · **채택**
- 원문 자리: 반(反) `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-antithesis-r2.md:45` · 합(合) `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis-r2.md:62`
- 반(反) 반론 칸 원문: **근거 5 의 「지목한 `touch` 는 머리에 지목 해시를 싣는다」는 지목 여부를 가르지 못한다.** 지목하지 않은 `touch` 도 머리에 같은 꼴의 해시를 싣는다. 그러니 딸린 실행 2 가 싣겠다는 「지목 여부」는 출력 내용이 아니라 파일 이름 `-pick` 과 후보 화면(`E2-scene.py:66-68` 의 방식)에서만 온다. 또 동치 규칙의 둘째 갈래 「두 호출이 같은 지목 문자열을 썼음이 산출에 있다」는 이 회차 산출로는 설 수 없다. 목록 산출 머리에 심볼 식별자도 지목도 없다(`04-callers/06-loadInstructions.txt:2`)
- 반(反) 좌표 칸 원문: `R/effect/03-touch/05-codexHostAdapter.txt:2`(`#71a6da9d1e2b`) · `:11`(`--pick 71a6da9d1e2b`) · `R/effect/03-touch/06-loadInstructions-pick.txt:2` · 초안 `:57` · `:157` · `:162`
- 반(反) 유효성 칸 원문: 참
- 반(反) 해악도 칸 원문: 미관
- 합(合) 채택 사유 원문: 지목이 없는 `05` 화면도 머리에 `#71a6da9d1e2b` 를 싣는다(`R/effect/03-touch/05-codexHostAdapter.txt:2`). `-pick` 파일이 없는 심볼이다. 목록 머리에는 식별자도 지목도 없다(`R/effect/04-callers/06-loadInstructions.txt:2`). 06~10 은 수가 0 이라 규칙 적용이 필요 없고, 수 ≥ 1 인 심볼은 지목 없이 풀렸다. 그래서 판정에는 해가 없다
- 합(合) 해악도 칸 원문: 미관 — 반(反)과 같다
- 이 항의 매김: 모집단은 취합자 · 어디가 걸리나는 반(反) 좌표 칸에서 저장소 트리에 있는 첫 파일 좌표 · 유효성은 반(反) 값 · 해악도는 합(合) 값

## 내가 기각한 것

아래 하나는 **이 판의 합(合)이 근거를 대고 기각한 반론**이다. 취합자인 내가 기각한 것은 없다 — 판의 처분 자리가 기각한 것을 그대로 옮긴다. 합(合) R1 이 하나(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis.md:78`)를 기각했다. 합(合) R2 의 `## 기각한 반론` 표(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis-r2.md:66-68`)에는 데이터 행이 없고 첫 칸 `—` · 둘째 칸 `없음` 인 자리 채우기 행 하나만 있다 — 그래서 합(合) R2 가 기각한 것은 없음이다. 해악도 칸은 합(合)이 기각 표에 등급을 적지 않아 반(反)의 값이다.

| # | 기각한 것 | 어디가 걸리나 | 모집단 | 유효성 | 해악도 | 판 라운드 | 반(反) 반론 칸 원문 | 반(反) 좌표 칸 원문 | 반(反) 유효성 칸 원문 | 반(反) 해악도 칸 원문 | 합(合) 기각 사유 원문 |
|---|---|---|---|---|---|---|---|---|---|---|---|
| R1-3 | `E2` ⑵ 는 두 가지로 읽힌다. 오라클 문구 「`effect/` 의 출력 파일에서 뽑고」 안에 `04-callers/` 가 드니, 질의 출력을 읽은 것은 잠긴 문구 안의 일이다 | `.palimpsest/rounds/2026-09-13-first-release-elsewhere/intent.md:361` | 회차기록 | 참 | 거짓신호 | R1 · 반(反) `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-antithesis.md:30` · 합(合) `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis.md:78` | **`E2` ⑵ 는 두 가지로 읽힌다.** 초안은 한쪽만 들었다. 같은 행의 「≥ 1(하한)」은 수에 붙는 말이다. 같은 조건이 등록한 오라클 문구는 「`effect/` 의 출력 파일에서 뽑고」이고, `effect/04-callers/` 는 그 디렉터리 안이다. 앞 오라클이 질의 출력을 읽은 것은 조건 문면을 벗어난 흐름이라기보다 잠긴 오라클 문구 안의 일이다. 초안 ② 의 3 「측정이 이미 한 번 흘렀다」는 오라클 문구를 빼고 세운 말이다. (F6 의 「`touch` 산출만 재면 반증」이라는 사실은 다투지 않는다.) | `R/intent.md:361`(「(하한)」 · 「`oracle/E2-scene.py` 가 ⑴~⑶ 을 `effect/` 의 출력 파일에서 뽑고」) · `R/oracle/E2-scene.py:55` · 초안 `p2-thesis.md:35` · `:46-47` | 참 — 초안은 `:361` 의 오라클 절을 다루지 않았다 | 거짓신호 | ① 조건 본문이 읽는 대상을 먼저 묶었다: 「봉인 목록의 심볼 전부에 `touch` 를 불렀고, **그 출력들에서** 장면 넷이 선다」(`R/intent.md:361`). 「그 출력들」은 `touch` 출력이다. 오라클 절은 스크립트가 **어느 디렉터리의 파일을 여는가**를 적은 것이다. 무엇이 ⑵ 를 세우는지를 다시 정의하지 않는다. ② 조건을 잠글 때 `04-callers/` 는 계획된 산출이 아니었다. 봉인 걸음 ③ 은 `touch` 산출만 적었다(`R/effect/00-seal.md:73`). `04-callers/` 는 읽은 줄 커밋 `c43f159` 에 처음 들어갔다(`git show --stat c43f159`). 반(反)도 이 사실을 「초안이 서는 자리」에 스스로 적었다(`p2-antithesis.md:19-20`). 그러니 잠긴 오라클 문구가 `04-callers/` 를 뜻했을 수 없다. ③ 앞 오라클이 조건 문면과 다른 자리를 읽었다는 것은 사실표 F6 · F17 이다(커밋 `5a4b2fd` · `1e54679` 제목). 판은 이것을 다시 판정하지 않는다. ⚠ 기각하지만 결론 쪽의 쓸모는 남는다. 「질의 출력을 읽는다」가 조건을 **고친 뒤에** 정당해진다는 점은 행 4 가 받친다. 이 반론의 「이미 잠긴 문구 안이었다」는 서지 않는다 |

## 기각의 원천 · 세지 않은 것

**R1: 반론 11 → 채택 10 · 기각 1.** 합(合) R1 채택 표 데이터 행 10(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis.md:63-72`) · 기각 표 데이터 행 1(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis.md:78`). 10 더하기 1 은 11 로 반(反) R1 의 번호 붙은 행 11(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-antithesis.md:28-38`)과 닫힌다.

**R2: 반론 12 → 채택 12 · 기각 0.** 합(合) R2 채택 표 데이터 행 12(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis-r2.md:51-62`) · 기각 표는 자리 채우기 행 하나(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis-r2.md:68`). 12 더하기 0 은 12 로 반(反) R2 의 번호 붙은 행 12(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-antithesis-r2.md:34-45`)과 닫힌다.

**겹침.** R1 반론은 정(正) R1 초안을, R2 반론은 정(正) R2 초안을 친다. 정(正) R2 는 R1 합(合) 판정문을 받았다(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-thesis-r2.md:4`). 두 라운드의 번호는 따로 셌다.

**반(反)이 스스로 물린 것은 세지 않았다** — 반론표에 오르지 않아 채택 · 기각의 모집단 밖이다. R1 여섯 줄(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-antithesis.md:49-54`) · R2 다섯 줄(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-antithesis-r2.md:77-81`). 그 열하나를 기각으로 세면 기각 수가 1 이 아니라 12 가 된다. 원문 그대로:

> ## 내가 스스로 물린 것
>
> - **「착수 때 못 본 것이 아니다」로 확대 분류를 치려 했다.** `R/intent.md:178` · `:195` 가 최상위 참조와 하한을 이미 처리했다는 데 기댔다. 그러나 두 줄은 `touch` 가 자리를 싣지 않는다는 것을 적지 않았다. 추론을 문면이 받치지 않아 뺐다.
> - **`touch` 출력 크기나 토큰 상한 합격선이 있어 자리 목록이 그것을 넘는다는 반론.** `corpus/criteria.toml` 에서 `touch` 의 토큰·바이트 상한을 grep 으로 찾지 못했다(`approx_tokens` 는 `crates/pal-cli/tests/envelope_two_layer.rs:206-207` 의 모양 단언뿐). 뺐다.
> - **효과 복제본 `ditto-effect` 가 남아 있지 않아 재실행이 불가능하다는 반론.** 경로를 찾지 못했다(`R/effect/01-install.txt:2` 에 `/ditto-effect` 만 보인다). 존재 여부를 확인하지 못해 뺐다.
> - **`262:50` 이 `discoverCodexAgents` 선언 안이 아니라 닫힘 조건이 틀린다는 반론.** 복제본 소스를 열지 않았다. 초안도 확인 못 한 것으로 적었다. 뺐다.
> - **F8 (같은 워크트리 대조) 재실행.** 돌리지 않았다. 사실표를 다시 판정하지 않는다.
> - **`docs/plan/00-goals.md:177` 「무엇이 깨지나」가 자리를 뜻한다는 읽기.** 이것은 초안을 받치는 쪽이라 반론으로 내지 않았다.

> ## 내가 스스로 물린 것
>
> - **`04-callers/06~10` 의 `범위 미해소 55` · `identity` 값으로 목록 호출이 어느 후보를 골랐는지 가를 수 있다** — 물렸다. `unique` 는 이름으로 찾은 후보 **전부**를 `accessed` 에 넣는다(`crates/pal-query/src/lib.rs:619`). 그러니 그 칸은 두 후보 파일을 합친 값이라 어느 쪽을 골랐는지 말하지 않는다.
> - **「초안이 `00-goals.md:106` 을 충분조건으로 쓰면서 그 줄의 등급 내역(`exact 5 · scoped 0 · candidate 2`)은 지금 `touch` 에 없다는 것을 안 댔다」** — 물렸다. 완성 화면에도 자리가 없다는 것은 초안을 오히려 받친다. 등급 내역은 이 의도에 잠기지 않았다.
> - **「`E4` 오라클은 「세지 않는다」 꼴로 최상위만 인정하고 화면이 말한 `x.foo()` 는 인정하지 않는다」**(`R/oracle/E4-breakage.mjs:8-11` ↔ `R/effect/03-touch/05-codexHostAdapter.txt:15`) — 물렸다. 초안의 결정이 아니라 앞서 있던 오라클의 뜻이고, 이번 사례의 판정을 바꾸지 않는다.
> - **「두 표면의 스냅샷 꼬리표가 같다고 내용이 같은 것은 아니다」** — 새롭지 않다. 초안이 「틀렸다면」 4 에 적었다.
> - **작업 디렉터리의 `e4-control-query-c.txt` 가 F8 의 대조 실행 산출이다** — 이름만 봤고 열지 않았다. 좌표가 없으므로 주장하지 않는다.

**반(反)의 나머지 절도 항으로 세우지 않았다** — 번호가 없고 반론을 가리키는 번호로 적혔다. 원문 그대로 R1 「초안이 서는 자리」(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-antithesis.md:13-22`) · 「초안이 버린 갈래를 받치는 근거 — 요약 좌표」(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-antithesis.md:40-45`):

> ## 초안이 서는 자리
>
> - **코드 사실.** `touch` 는 `p.callers(symbol.id)?.len()` 만 남긴다(`crates/pal-query/src/lib.rs:839`). `symbol.callers` 도 같은 `p.callers` 를 풀어 경로를 낸다(`lib.rs:576-580`). 사람 화면은 수 한 줄만 찍는다(`crates/pal-cli/src/touch.rs:505`). 새 해소 규칙이 아니라는 초안의 말은 선다.
> - **`E4` 의 문면.** 「봉인 심볼들의 `touch` 가 호출자로 낸 파일 집합 C」(`R/intent.md:363`)는 글자 그대로 `touch` 를 이름으로 댄다. 글자대로 읽으면 반증이라는 쪽은 무너뜨리지 못했다.
> - **⑥ 의 산술.** 판정 수는 `rest = P−C−D` 의 파일에서만 센다(`R/oracle/E4-breakage.mjs:61` · `:66-76`). 선언 안 오류는 `262:50` 하나다(`R/oracle/E4-literal-touch-c.txt:14`). 나머지 새 오류는 최상위다(`:13` · `:17` · `:20`). 그러니 C 가 `src/cli/commands/setup.ts` 를 담으면 선언 안 = 0 이 된다. 이 계산은 맞다.
> - **② 의 2 (자리는 계획에 없던 명령에서 왔다).**
>   - 봉인의 걸음 표 ③ 은 `touch` 산출만 적었다(`R/effect/00-seal.md:73`).
>   - `effect/04-callers` 는 읽은 줄 커밋 `c43f159` 에 처음 들어갔다(`git log --diff-filter=A`). 봉인이 계획한 걸음이 아니었다.
>   - `touch` 화면은 능력 이름만 늘어놓는다(`R/effect/03-touch/05-codexHostAdapter.txt:48`).
> - **규약 인용.** `SKILL.md:312` · `:323` · `:327` · `:329` · `:331` · `:377` · `:386-387` 을 열어 보니 인용한 대로 적혀 있다.

> ## 초안이 버린 갈래를 받치는 근거 — 요약 좌표
>
> - **조건 문면 고치기(정정)**: 행 4. `E4` 의 목적 문장이 받친다(`R/intent.md:363` 끝). 같은 원천도 받친다(`lib.rs:576` · `:839`).
> - **안내 줄만(이관 항목으로)**: 행 5. `docs/adr/0011-moved-volume-and-unseen-volume-are-different-fields.md:74-75` 가 받친다.
> - **반증 + 이슈**: 행 1·2 가 서면 초안이 이 갈래를 축소로 분류한 근거(`p2-thesis.md:114`)가 무너진다. 「원 의도의 한 다리」가 장면(`:49` · `:131`) 밖이기 때문이다. 그때 이 갈래는 축소가 아니고, 결국 조건 문면 쪽 결함 처리로 모인다.
> - **효과 장면 다시 뜨기**: 행 10. 확대의 값이 효과(사용자가 닿는다)라면, 재지 않고 닫는 것은 원문의 「효과를 적는다」(`R/intent.md:53`)를 비껴간다. 다시 떠야 확대가 산출 이상이 된다.

R2 「초안이 서는 자리」(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-antithesis-r2.md:14-28`) · 「초안이 버린 갈래를 받치는 가장 강한 근거」(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-antithesis-r2.md:47-59`) · 「초안의 「틀렸다면」에 대한 좌표」(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-antithesis-r2.md:61-73`):

> ## 초안이 서는 자리
>
> - **「수 0 이면 빈 집합」은 코드로 선다.** `touch` 의 수는 `p.callers(symbol.id)?.len()` 이다(`crates/pal-query/src/lib.rs:839`). 수가 0 이면 센 호출자 집합은 목록과 무관하게 비었다.
> - **수 ≥ 1 이고 이름이 하나뿐인 심볼에서 「같은 심볼로 풀린다」는 지목 인자와 무관하게 선다.** 초안의 「틀렸다면」 4 가운데 `ctx.pick` 갈래가 여기서 닫힌다.
>   - `unique` 는 지목이 후보와 안 맞으면 `Ambiguous` 를 낸다(`lib.rs:620-632`).
>   - 지목이 없고 후보가 하나면 그 하나를 낸다(`lib.rs:639`).
>   - 그러니 목록 호출이 `(없음)` 이 아닌 목록을 냈다면, 지목을 붙였든 안 붙였든 그 하나의 심볼이다.
>   - 스냅샷 **내용**이 같았나는 여전히 안 닫힌다. 초안이 이미 적었다.
> - **줄 수가 같으면 집합도 같다.** 목록은 `p.callers` 에서 `p.symbol(id) == None` 인 것을 **빼기만** 한다(`lib.rs:576-580`).
> - **`E4` 의 예고 산수는 커밋된 산출로 선다.** P 는 `E4-literal-touch-c.txt:6` 이다. 문면대로의 선언 안 1 은 `:14` 의 `setup.ts` 다. `setup.ts` 는 `04-callers/05-codexHostAdapter.txt:4` 에 있다. `index.ts:2:34` 는 A 워크트리에서도 최상위다(`:17`).
> - **인터뷰에 호출자 자리를 다룬 물음이 없다.** 초안의 관측이 맞다. `호출자|깨질|깨지|callers|경로:줄|파일·줄` 은 두 파일 모두 0 건이다(`R/interview/r1.md` 21 줄 · `r2.md` 19 줄). `r2.md:19` 의 잠근 목록에도 없다.
> - **정본 화면이 다 만들어져도 호출자는 수와 등급이지 자리가 아니다**(`docs/plan/00-goals.md:106` · `:145`).
> - **「틀렸다면」 5 의 의도 쪽 절반은 선례로 선다.** `R/intent.md:291` 의 「이슈로 세운다(분할)」 줄은 커밋 `d1f8ddb`(2026-09-13 21:11:30)에서 들어왔다. 잠긴 의도 승인 커밋 `66fc201`(20:49:39)보다 **뒤**다. 회차 중에 `## 범위 밖` 에 줄을 더한 선례가 있다. 보고서 쪽은 반론 8 을 보라.
> - **「틀렸다면」 6(새 측정 장치)은 치지 못했다.** 동치 규칙은 조건 문면이 이미 이름으로 부른 두 오라클(`R/intent.md:361` · `:363`)의 입력 규칙이다. `:265-266` 의 (가) 「장치를 회차 안에서 만들지 않는다 · 일회성 측정」을 넘는다는 좌표를 못 찾았다.
> - **초안이 인용한 `SKILL.md` 좌표는 열어 본 대로다**(`:316` · `:322` · `:324` · `:327` · `:329` · `:366-368` · `:377-378` · `:381-387` · `:738` · `:774` · `:797-798`).

> ## 초안이 버린 갈래를 받치는 가장 강한 근거
>
> - **반증 + 이슈 / 승격.**
>   - 소유자의 승인 전제 「반증으로 적는다 — 조건을 약하게 고치지 않는다」(`R/approval.md:13-14`)가 `E4` 를 이름으로 짚었다.
>   - 그 전제에서는 문면대로의 반증(`E4-literal-touch-c.txt:22` · `E2-literal-touch.txt:39`)을 판정으로 두는 쪽이 기본값이다.
>   - 문면을 바꾸려면 소유자에게 올리는 쪽이 전제와 맞는다(반론 1).
> - **확대 — 자리 싣기 / 안내 줄.**
>   - 효과 실행에서 호출자에 관해 **없어진 걸음**은 자리에서 났다(`R/effect/04-readnote.md:15` · `:29`).
>   - `touch` 의 수(`호출자 1`)로는 그 걸음을 못 없앤다(반론 6).
>   - 잠긴 계획은 효과 실행을 「`touch` → **화면이 안내한 명령으로** 승인 → `touch` 가 그 사용자 걸음 그대로다」로 정의했다(`R/intent.md:166`).
>   - 그런데 호출자 자리는 화면이 안내하지 않은 명령(`symbol.callers`)에서 왔다(`04-readnote.md:12`, F5). 안내 줄이 있어야 그 걸음이 「사용자 걸음 그대로」에 든다.
>   - 안내의 꼴은 이미 제품에 있다. `Fold{what, count, unfolded_by}` — 「어느 질의가 펴는지가 값이다」(`docs/adr/0011-*.md:40` · `:44`).
>   - ⚠ `:166` 은 승인 걸음에 대한 문장이다. 호출자에 옮겨 읽는 것은 유비이지 문면이 아니다.

> ## 초안의 「틀렸다면」에 대한 좌표
>
> - **1 (ⓒ 가 의도 안이었나)**
>   - 초안이 안 연 `docs/plan/01-completion-scenes.md:46-47` 에 「무엇이 이것을 부르는지」가 있다.
>   - 같은 문서 `:29` · `:49-50` 이 정본을 `00-goals.md` §1 로 넘기고, 그 화면(`:106`)은 수다. → 반론 9(추정).
>   - ⓒ 쪽을 더 세게 받치는 좌표는 `R/approval.md:13-14`(반론 1)와 `R/effect/04-readnote.md:15` · `:29`(반론 6)다.
> - **4 (같은 이름이 같은 심볼로 풀리나)**
>   - `ctx.pick` 갈래는 `crates/pal-query/src/lib.rs:620-641` 로 닫힌다(「서는 자리」).
>   - 스냅샷 내용이 같았나는 안 닫힌다. `04-callers/*` 에는 `touch` 에 있는 `워킹트리 일치` 줄(`03-touch/05-codexHostAdapter.txt:41`)이 없다.
> - **5 (`SKILL.md:316` · `:798` 의 읽기)**
>   - 의도 쪽: `R/intent.md:291` 이 승인 뒤(`d1f8ddb` > `66fc201`)에 더해진 선례가 있어 선다.
>   - 보고 쪽: `## 다음 회차가 받는 것` 은 `:797` · `:906-907` · `:914` 와 부딪친다(반론 8).
> - **6 (새 측정 장치)**: 치지 못했다(「서는 자리」).

**합(合)이 `## 내가 못 정한 것` 으로 남긴 것도 발견 항으로 세우지 않았다.** 합(合) R1 세 항(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis.md:135-139`) · 합(合) R2 세 항(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis-r2.md:127-138`). 원문 그대로:

> ## 내가 못 정한 것
>
> 1. **원문 한 줄의 뜻.** 소유자가 「깨질 곳을 받는다」(`:20-21`)를 장면의 「실제 수」(`:49`)보다 넓게, 즉 화면이 자리를 싣는 것으로 뜻했을 가능성이 있다. 잠긴 문서 안의 무게는 수 쪽이다(`:49` · `:131` · `:165` · `:195`). 그래서 정정으로 판정했다. 그러나 소유자의 뜻을 확정할 원문은 회차 산출에 없다. **있어야 할 것**: 인터뷰 기록 `R/interview/r1.md` · `r2.md` 에 호출자 자리를 다룬 물음이 있었는지. 나는 받지 않았고 열지 않았다. 없으면 R2 합이 위 칸 1 로 올릴지를 정한다.
> 2. **F8 의 재실행.** 워크트리 `ditto-effect-at-A` 가 남아 있는지, 같은 워크트리에서 C = 없음 · C = 질의 목록 두 실행이 P 를 같게 내는지 확인하지 않았다. **있어야 할 것**: 「초안을 어떻게 고치나」 3 의 나란한 재실행 산출.
> 3. **동치 검사의 일반성.** 13/13 이 같다는 것은 이 봉인 심볼들에서의 관측이다. `p.symbol(id)` 가 `None` 인 호출자가 생기는 조건(`lib.rs:577`)을 코드로 추적하지 않았다. 이 회차의 두 조건에는 13 쌍 검사로 충분하다. 그러나 「`touch` 의 수 = `symbol.callers` 목록 길이」를 일반 불변식으로 적을 근거는 아니다. **있어야 할 것**: 투영에서 `callers` 가 낸 id 가 `symbol` 에서 빠지는 경로의 유무. 이것은 이슈 본문의 몫이다.

> ## 내가 못 정한 것
>
> - **정정인가 완화인가.**
>   - 무엇이 갈림인가: 잠긴 장면·계획·정본 화면은 수를 요구한다(`R/intent.md:49` · `:131` · `:165` · `docs/plan/00-goals.md:106`). 반면 승인된 조건 문면·원문 머리·승인 전제는 자리를 가리킨다(`R/intent.md:361` · `:363` · `:20-21` · `R/approval.md:11` · `:13-14`).
>   - 둘 다 소유자가 잠근 기록이라 어느 쪽이 이기는지는 문서가 정하지 않는다. `00-goals.md:137-139` 의 우선 규칙은 `01-completion-scenes.md` 와의 갈림에만 걸리고, 잠긴 조건 문면과의 갈림에는 걸리지 않는다.
>   - 무엇이 있어야 정하나: 칸 1 에 대한 소유자의 답.
> - **종료 보고의 어느 절에 이슈 줄을 싣나.**
>   - 무엇이 갈림인가: `SKILL.md:798` 은 보고의 `## 범위 밖` 이 「회차 중에 새로 생기지 않는다」라 한다. 그런데 이 회차는 이미 승인 뒤에 의도의 `## 범위 밖` 에 줄을 더했다(`R/intent.md:291` · `d1f8ddb`).
>   - 무엇이 있어야 정하나: 그 선례 줄을 보고에서 어디에 싣는지 정한 규약 문장이나 앞 회차 `report.md` 의 실례. 같은 처리를 이 줄에도 적용한다.
> - **음성 대조 ③ 이 실제로 선언 안 1 을 다시 내는지.**
>   - 무엇이 갈림인가: `E4-literal-touch-c.txt:1` 의 대조 실행 주석은 재실행 산출이 아니다(F8). A 워크트리 `ditto-effect-at-A` 가 지금 남아 있는지 나는 확인하지 않았다.
>   - 무엇이 있어야 정하나: 개정 커밋 뒤 A 에서 돌린 두 실행의 커밋된 산출.

## 계수 — 두 원천 대조

| 잰 것 | 반(反) | 합(合) | 같나 |
|---|---|---|---|
| R1 반론 수 — 번호 붙은 행 | 11 (`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-antithesis.md:28-38`) | 채택 10 더하기 기각 1 은 11 | 같다 |
| R1 채택 수 | 반(反)은 안 센다 | 10 (`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis.md:63-72`) | — |
| R1 기각 수 | 반(反)은 안 센다 | 1 (`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis.md:78` · 반론 3) | — |
| R1 반론별 해악도가 반(反)과 합(合)에서 같은 수 | 채택 10 중 10 | 채택 10 중 10 | 같다 — 합(合) R1 은 등급을 하나도 안 바꿨다 |
| R1 해악도 금지역 — 반(反)은 반론 11 전부 · 합(合)은 살아남은 10 | 0 (—) | 0 (—) | 같다 |
| R1 해악도 실패 — 반(反)은 반론 11 전부 · 합(合)은 살아남은 10 | 4 (1 · 4 · 7 · 9) | 4 (1 · 4 · 7 · 9) | 같다 |
| R1 해악도 거짓신호 — 반(反)은 반론 11 전부 · 합(合)은 살아남은 10 | 6 (2 · 3 · 5 · 6 · 8 · 10) | 5 (2 · 5 · 6 · 8 · 10) | **다르다** — 차이 1 는 기각된 반론 3 다 |
| R1 해악도 미관 — 반(反)은 반론 11 전부 · 합(合)은 살아남은 10 | 1 (11) | 1 (11) | 같다 |
| R1 유효성 | 참 11 | 합(合)은 유효성 칸이 없다 | — |
| R1 합(合) 분포 표 대 합(合) 채택 표에서 센 값 (금지역 · 실패 · 거짓신호 · 미관) | — | 표 0 · 4 · 5 · 1 (`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis.md:128-131`) · 채택 표에서 셈 0 · 4 · 5 · 1 | 같다 |
| R1 합(合) 분포 표의 기각 · 근거 없는 기각 | — | 기각 1 (3) · 근거 없는 기각 0 (`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis.md:132-133`) | 기각 표 행 수 1 과 같다 |
| R1 종료 판단이 받은 분포 대 합(合) 분포 표 | — | 발췌 0 · 4 · 5 · 1 (`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-brief-referee-r1.md:9-13`) · 종료 판단 표 0 · 4 · 5 · 1 (`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-referee.md:19-22`) | 같다 |
| R2 반론 수 — 번호 붙은 행 | 12 (`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-antithesis-r2.md:34-45`) | 채택 12 더하기 기각 0 은 12 | 같다 |
| R2 채택 수 | 반(反)은 안 센다 | 12 (`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis-r2.md:51-62`) | — |
| R2 기각 수 | 반(反)은 안 센다 | 0 (기각 표 `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis-r2.md:68` 은 자리 채우기 행) | — |
| R2 반론별 해악도가 반(反)과 합(合)에서 같은 수 | 채택 12 중 12 | 채택 12 중 12 | 같다 — 합(合) R2 는 등급을 하나도 안 바꿨다 |
| R2 해악도 금지역 — 반(反)은 반론 12 전부 · 합(合)은 살아남은 12 | 0 (—) | 0 (—) | 같다 |
| R2 해악도 실패 — 반(反)은 반론 12 전부 · 합(合)은 살아남은 12 | 1 (1) | 1 (1) | 같다 |
| R2 해악도 거짓신호 — 반(反)은 반론 12 전부 · 합(合)은 살아남은 12 | 9 (2 · 3 · 4 · 5 · 6 · 7 · 8 · 9 · 10) | 9 (2 · 3 · 4 · 5 · 6 · 7 · 8 · 9 · 10) | 같다 |
| R2 해악도 미관 — 반(反)은 반론 12 전부 · 합(合)은 살아남은 12 | 2 (11 · 12) | 2 (11 · 12) | 같다 |
| R2 유효성 | 참 11 · 추정 1 (추정은 반론 9) | 합(合)은 유효성 칸이 없다 | — |
| R2 합(合) 분포 표 대 합(合) R2 채택 표에서 센 값 | — | 표는 **ID 만** 적었다 — ID 를 세면 0 · 1 · 9 · 2 (`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis-r2.md:142-150`, `:150` *「수는 ID 를 세면 나온다 — 합(合) 지시문이 반환문에 수를 적지 말라 했다」*) · 채택 표에서 셈 0 · 1 · 9 · 2 | 같다 |
| R2 합(合) 분포 표의 모집단 | — | R2 반론만이다. 합(合) R1 에서 살아남은 10 은 그 표에 안 들어 있다 | ⚠ 아래 ⑴ |
| R2 종료 판단이 받은 분포 대 합(合) 분포 표 | — | 발췌 0 · 1 · 9 · 2 (`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-brief-referee-r2.md:9-13`, 발췌 머리 *「합은 ID 만 적었다 — 수는 ID 를 센 것」*) · 종료 판단 표 0 · 1 · 9 · 2 (`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-referee-r2.md:17-20`) | 같다 |
| R2 근거 없는 기각 | — | 합(合) R2 *「없음(기각 자체가 없음)」* (`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis-r2.md:149`) · 발췌 0 (`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-brief-referee-r2.md:13`) · 종료 판단 *「근거 없는 기각은 0 이다. 기각 자체가 없었다」* (`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-referee-r2.md:22`) | 같다 |
| 커밋 제목의 셈 — R1 | 반(反) `333f067` *「반론 열하나, 실패 넷」* · 반론표에서 셈 11 · 실패 4 | 합(合) `d75ed6b` *「살아남은 실패 넷」* · 종료 판단 `39ab3cd` *「실패 넷이 남아 R2 를 돈다」* · 분포 표 실패 4 | 같다 |
| 커밋 제목의 셈 — R2 | `254505e` *「판 p2 R2 — 반론 열둘, 합은 정정·완화를 못 갈라 올린다」* · 반론표에서 셈 12 | 같은 커밋(반(反) R2 · 합(合) R2 · 종료 판단 R2 · 그 발췌를 함께 더했다) · 합(合) R2 분류 칸 *「갈라내지 못한다」* · 칸 표 데이터 행 1 | 반론 수 같다 |
| 판 전체 해악도 — 반(反) 매김 (반론 23) 대 이 보고의 항 (채택 항은 합(合) 값 · 기각 행은 반(反) 값) | 금지역 0 · 실패 5 · 거짓신호 15 · 미관 3 | 금지역 0 · 실패 5 · 거짓신호 15 · 미관 3 | 같다 |
| 소유자에게 올릴 칸 수 | — | 합(合) R2 물음 표 데이터 행 1 (`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis-r2.md:121-123`) · 종료 판단 R2 칸 표 데이터 행 2 (`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-referee-r2.md:53-56`) | **다르다** — 아래 ⑵ |
| 판 전체 반론 수 대 이 보고의 항 수 | 11 더하기 12 는 23 | `### ` 항 22 더하기 기각 표 행 1 은 23 | 같다 |

**판 안 셈 문구 대조 — 정(正)과 합(合).** 반(反)의 원천이 아니라서 따로 싣는다.

| 잰 것 | 정(正) | 합(合) | 같나 |
|---|---|---|---|
| 판정의 길 · §5 분류 | R1 확대(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-thesis.md:20`) · R2 정정(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-thesis-r2.md:16`) | R1 정정 — 셋째 길(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis.md:15-19`) · R2 정정, *「나는 이것을 완화와 갈라내지 못한다」*(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis-r2.md:18`) | R1 에서 **다르다** · R2 에서 낱말은 같고 합(合)이 갈라내지 못한다고 적었다 |
| 승격이 필요한가 · 소유자에게 | R1 *「필요하지 않다」*(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-thesis.md:53`) · R2 *「필요하지 않다」*(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-thesis-r2.md:110`) | R1 *「안 올린다(지금은) — R2 를 돈다」*(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis.md:114`) · R2 *「올린다」*(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis-r2.md:115`) | R2 에서 **다르다** |
| 동치 일회성 확인 13 쌍의 분포 | R2 *「01~04 · 11~13 은 1=1 · 05 는 2=2 · 06~10 은 0=0」*(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-thesis-r2.md:7`) | R1 같은 문구(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis.md:7`) | 같다 |
| 수가 1 이상인 심볼 수 | R2 *「수가 1 이상인 심볼 8」*(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-thesis-r2.md:132`) | R1 분포(01~04 · 05 · 11~13)를 세면 8 (`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis.md:7`) | 같다 |
| `E2` ⑵ 예고의 다른 파일 참조 수와 좌표 | R2 2 건 · `E2-scene.txt:10`(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-thesis-r2.md:132`) | R1 2 건 · `E2-scene.txt:9-11`(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis.md:96`) | 수는 같고 줄 좌표가 다르다 |
| 소유자 칸 문장 수 (예고 포함) | R2 *「그때 칸은 하나다」*(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-thesis-r2.md:112`) | R1 칸 1 하나(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis.md:118-122`) · R2 칸 1 하나(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis-r2.md:123`) | 같다 |
| 합(合) R2 가 반론 1 의 좌표 `R/approval.md` 를 허용 목록 안이라 적은 것 대 반(反) R2 머리 | 반(反) R2 *「허용 목록 밖에서 연 것 하나: `R/approval.md`」*(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-antithesis-r2.md:7`) | 합(合) R2 *「메인이 준 내 허용 목록에 들어 있다」*(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis-r2.md:37`) | 두 역할의 목록이 달라 대 볼 수 없다 — 두 문장을 옮기기만 한다 |

**★ 갈린 자리를 맞추지 않고 적는다.**

**⑴ 합(合) R2 의 분포 표는 R2 반론만 센다.** 합(合) R1 에서 살아남은 반론은 10(금지역 0 · 실패 4 · 거짓신호 5 · 미관 1)이다. 두 라운드에서 살아남은 반론을 합치면 **R1 10 더하기 R2 12 는 22** 이고, 종료 판단 R2 가 받은 분포의 합은 **12** 다. 어느 쪽이 설계문이 뜻한 「살아남은 반론」인지는 내가 정하지 않는다. 합(合) R2 는 R1 반론을 번호로 부르지 않는다 — 초안이 인용한 R1 합(合)의 판정문을 거친다.

**⑵ 소유자에게 올릴 칸 수가 1 과 2 로 갈린다.** 합(合) R2 는 물음 하나를 올렸고(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis-r2.md:121-123`), 종료 판단 R2 는 판정문 본문을 안 받은 채 *「이 판에서 올리는 칸은 **2** 다」*(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-referee-r2.md:49`)로 적고 칸의 구체 내용은 메인이 채우라고 적었다(`:51`). 나는 칸을 채우지 않았고 둘을 하나로 맞추지 않았다 — 아래 `## 소유자에게 올릴 물음` 에 둘 다 옮겼다.

**이 보고의 항 수 — 두 장치를 실제로 돌려서 적는다.** 이 본문을 스크래치패드에 써서 ⑴ 추출기 `.claude/skills/round/bin/extract.py` 를 `정반합 3 <이 본문>` 으로 **그대로 돌렸다** — 프로필 `current` · **23 행**(`DL3-01`~`DL3-23`) · `모집단` · `유효성` · `해악도` 가 enum 밖인 행 0 · `경로` 가 `(경로 없음)` 인 행 0 · 해악도 금지역 0 · 실패 5 · 거짓신호 15 · 미관 3 · 유효성 참 22 · 추정 1 · 모집단 회차기록 23. 이 해악도 분포는 반(反) 두 라운드의 번호 붙은 행을 합친 값(금지역 0 · 실패 5 · 거짓신호 15 · 미관 3)과 같다. ⚠ 스크래치 경로는 저장소 밖이라 추출기가 트리 대조 없이 모양으로 좌표를 골랐다 — 항의 `어디가 걸리나` 23 개는 생성할 때 `git ls-files --cached --others --exclude-standard` 트리에 전부 있음을 댔다. ⑵ 계수기 `xtask/src/main.rs` 의 `반환문_항_수` 정반합 가지(`### ` 항 + 기각 절의 `- ` · 표 행 − 기각 절 표 헤더)를 파이썬으로 옮겨 돌렸다 — `### ` 22 · 기각 절 `|` 행 2 · 헤더 1 · **항 23**. 두 장치가 같은 23 을 낸다. ⚠ 계수기는 Rust 원본이 아니라 내가 옮긴 사본이다.

⚠ **그렇게 맞추려고 형식에서 지킨 것.** `### ` 는 발견 항에만 썼다(소제목은 `#### `). 항 불릿 이름이 추출기 라벨(`모집단` · `유효성` · `해악도` · `어디가 걸리나`)로 시작하는 것은 그 넷뿐이다. 원문 표 · 소제목은 전부 인용 블록(`> `) 안에 넣어 표나 항으로 안 읽히게 했다. 기각 절 안에는 `- ` 줄을 두지 않았다. 코드펜스를 쓰지 않았다.

## 판 안 반증 신호 아홉 — 셈

설계문 원문(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-design.md:107-119`):

> ## 무엇이 이 판을 반증하나
>
> 이 판이 헛돌았다는 신호다. 합(合) 뒤와 취합 보고에서 대 본다.
>
> 1. **분류가 빠졌다.** 합(合)의 판정이 길 하나를 고르되 §5 분류나 자기 완결성 시험의 답이 없다. 그러면 물음(「필요한가」)에 답하지 않은 것이다.
> 2. **기계 몫을 가져갔다.** 판정문이 `E2` ⑵ · `E4` 가 문면대로 통과인지 반증인지를 다시 판정했다(F6 · F7 과 다른 값을 냈다).
> 3. **사실표와 어긋난다.** 판정이 F1~F5 에 반하는 사실에 기댄다. 예: 「`touch` 가 이미 자리를 싣는다」, 「`--json` 에 목록이 있다」.
> 4. **확대인데 잴 수 없다.** 판정이 확대인데 기계 표 1 행(닫힘)이나 2~4 행(회귀)을 이름으로 대지 않는다.
> 5. **반증 + 이슈인데 넘길 것이 없다.** 이슈를 한 문장으로 못 쓰거나, 승격 필요 여부를 적지 않는다.
> 6. **반(反)이 헛돌았다.** 반론이 전부 좌표가 없거나, 전부 미관이다.
> 7. **격리가 깨졌다.** 정·반·합 중 어느 산출물이 「안 주는 것」에 적힌 자료를 인용한다(`e3-*` · `p1-*` · `R/state.md` · `R/findings.jsonl` · 원장 레코드 본문).
> 8. **사후 신호.** 확대로 판정하고 실행했는데 기계 표 1 행을 다시 돌려도 `E2` ⑵ 나 `E4` 가 반증이다. 그러면 판의 전제(원인이 하나로 모인다)가 틀렸던 것이다. `E4` 에 대해서는 F7 · F8 이 이미 그 전제를 시험할 재료다. 판정문이 F8 을 재실행 없이 사실로 받았다면, 이 신호가 나올 때 거기서부터 거슬러 본다.
> 9. **새 정보가 없었다(끝 조건이 아니라 기록 대상).** R2 반론이 R1 과 같은 좌표·같은 논지뿐이고 합(합)의 처분도 바뀌지 않았다. 취합 보고가 이것을 적는다.

| # | 신호 | 셈 — 원천 | 섰나 |
|---|---|---|---|
| 1 | 분류가 빠졌다 | 합(合) R1: §5 분류 칸 *「§5 분류 한 낱말: 정정.」*(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis.md:19`) · 자기 완결성 시험 절(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis.md:45`). 합(合) R2: §5 분류 칸(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis-r2.md:18`) · 자기 완결성 시험의 답(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis-r2.md:41-45`). 두 합(合)에서 둘 중 빠진 것 **0** | 안 섰다 |
| 2 | 기계 몫을 가져갔다 | 합(合) 판정문에서 `E2` ⑵ · `E4` 에 판정값 낱말(통과 · 반증)을 붙인 줄: 합(合) R1 **2**(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis.md:96` · `:97`, 둘 다 *「→ 통과」*) — 같은 항의 앞줄 `:95` *「재실행 전에는 두 조건 다 **미측정**이다. 앞 산출은 이렇게 **예고**한다」* · 뒷줄 `:98` *「**예고를 판정으로 세지 않는다.**」*. 합(合) R2 **0**(`:111` *「문면 오라클 재실행이 판정한다」*). 두 줄의 값은 고친 문면 아래의 예고로 적혔다 — F6 · F7(옛 문면)과 「다른 값」인지는 가르지 않는다 | 셈만 싣는다 |
| 3 | 사실표와 어긋난다 | 낱말 규칙 `이미 자리` · `자리를 싣는다` · `목록이 있다` · `배열` · `싣지 않` 에 걸린 합(合) 두 파일의 줄 **9** — `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis.md:17` · `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis.md:55` · `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis.md:82` · `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis.md:101` · `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis.md:105` · `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis-r2.md:41` · `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis-r2.md:107` · `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis-r2.md:110` · `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis-r2.md:123`. 그 가운데 `touch` 나 `--json` 이 지금 호출자 자리 · 목록을 싣는다고 적은 줄 **0**. 걸린 까닭: R1 `:82` 는 초안의 판정 절 *「§5 확대 — `touch` 가 자리를 싣는다」*를 **빼라는** 인용 · R2 `:123` 은 칸 1 「안 받는다」 갈래의 실행 문장 *「**확대**로 이 회차에서 `touch` 에 호출자 자리를 싣는다」* · R2 `:110` 은 종료 보고 절에 이슈를 *「싣지 않는다」* · 나머지 여섯(R1 `:17` · `:55` · `:101` · `:105` · R2 `:41` · `:107`)은 `touch` 가 자리를 싣지 않는다는 쪽 문장이다 | 안 섰다 |
| 4 | 확대인데 잴 수 없다 | 확대로 판정한 합(合) **0**(R1 정정 · R2 정정, 위 1). 합(合) R2 칸 1 의 「안 받는다」 갈래가 확대이고, 거기 적힌 검증 표면에 기계 표 행 이름이 나온 수: 1 행(문면 오라클 재실행) — *「새 `touch` 출력으로 다시 잰다」* · 2 행(`cargo xtask test` · `check`) **0** · 3 행(`D1`) **1** · 4 행(`B4`) **1**(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis-r2.md:123`) | 해당 판정 0 — 셈만 싣는다 |
| 5 | 반증 + 이슈인데 넘길 것이 없다 | 반증 + 이슈를 고른 합(合) **0**(합(合) R2 *「반증 + 이슈 길은 어느 갈래에서도 밀린다」* `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis-r2.md:31`). 이슈 한 문장을 적은 합(合) **2**(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis.md:101` · `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis-r2.md:107`). 승격 필요 여부를 적은 합(合) **2**(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis.md:114-118` · `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis-r2.md:115-119`) | 해당 판정 0 |
| 6 | 반(反)이 헛돌았다 | 좌표 칸에 백틱 좌표가 없는 반론: R1 **0 / 11** · R2 **0 / 12**. 반(反)이 미관으로 매긴 반론: R1 **1 / 11**(반론 11) · R2 **2 / 12**(반론 11 · 12) | 안 섰다 |
| 7 | 격리가 깨졌다 | 낱말 규칙 `e3-` · `p1-` · `state.md` · `findings.jsonl` · `원장 레코드` 가 정 · 반 · 합 여섯 파일에 나오는 줄: 머리(`>` 로 시작 — 안 받은 것 목록) **6**(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-thesis.md:6` · `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-antithesis.md:5` · `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis.md:5` · `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-thesis-r2.md:5` · `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-antithesis-r2.md:5` · `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis-r2.md:5`) · 본문 **2**(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-antithesis.md:34` · `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis.md:68`). 본문 줄은 둘 다 `git show --stat 42af275` 에 찍힌 파일 **이름** `e3-referee.md` 다. 산출물 머리가 스스로 신고한 그 밖의 경계 사항은 `## 내가 받은 것 / 안 받은 것` 에 원문으로 옮겼다 | 셈만 싣는다 |
| 8 | 사후 신호 | 판정이 실행되지 않았다 — 합(合) R2 판정값 *「보류」*(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis-r2.md:14`), 실행은 *「소유자가 칸 하나에 「받는다」로 답한 뒤에만」*(`:16`). 이 보고가 받은 산출물 안에 확대 실행 **0** · 기계 표 1 행 재실행 산출 **0**. F8 을 다룬 줄: 정(正) R1 `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-thesis.md:105` · 합(合) R1 `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis.md:92` · `:138` · 정(正) R2 `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-thesis-r2.md:143` · 합(合) R2 `.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis-r2.md:99` · `:137` | 미측정 |
| 9 | 새 정보가 없었다 | 반(反) R2 좌표 칸의 `파일:줄` 이 반(反) R1 좌표 칸의 `파일:줄` 과 하나라도 겹치는 반론 **5 / 12** — 반론 2(1/3: `R/intent.md:389-391`) · 반론 3(2/4: `.claude/skills/round/SKILL.md:329`, `.claude/skills/round/SKILL.md:381-383`) · 반론 4(1/3: `.claude/skills/round/SKILL.md:329`) · 반론 6(2/6: `R/effect/04-readnote.md:15`, `R/effect/04-readnote.md:29`) · 반론 10(1/3: `R/intent.md:361`). 전부 겹치는 반론 **0**. (규칙: 백틱 안 `파일:줄` 과 앞 파일에 붙는 `:줄` · 「초안 `:줄`」은 그 라운드 초안 · 초안이 라운드마다 달라 초안 좌표는 겹칠 수 없다.) 합(合) 처분: R1 *「**수정**」* · 소유자 *「안 올린다(지금은) — R2 를 돈다」*(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis.md:15` · `:114`) → R2 *「**보류**」* · *「올린다」*(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis-r2.md:14` · `:115`). 논지가 같은지는 가르지 않았다 | 셈만 싣는다 |

**셈으로 가른 신호 1 · 3 · 6 에서 선 것 0 · 해당 판정이 없는 신호 4 · 5 · 미측정 신호 8 · 셈만 실은 신호 2 · 7 · 9.**

## 종료 판단

종료 판단 R1 그대로(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-referee.md:11-60`):

> ## 답
>
> **한 라운드 더 (R2).**
>
> ## 근거 — 살아남은 반론의 해악도
>
> | 해악도 | 수 | 반론 ID |
> |---|---|---|
> | 금지역 | 0 | — |
> | 실패 | 4 | 1 · 4 · 7 · 9 |
> | 거짓신호 | 5 | 2 · 5 · 6 · 8 · 10 |
> | 미관 | 1 | 11 |
> | 근거 없는 기각 | 0 | — |
>
> - 쓴 라운드: 1. 라운드 상한: 2. 상한이 한 라운드 남았다.
> - 합(合) 판정의 §5 분류: **정정**
>
> ### 적용한 규칙 줄 (원문)
>
> 1. 게이트를 무엇으로 가르는가
>    > 「**게이트.** 판정값(`verdict`)이 아니라 해악도로 가른다.」
>
> 2. R1 종료 판단 — 이 줄이 답을 정했다
>    > 「살아남은 금지역·실패가 1 이상이면 **R2** 를 돈다.」
>
>    금지역 0 과 실패 4 를 더하면 4 다. 1 이상이므로 R2 를 돈다.
>    같은 절의 「살아남은 금지역·실패가 0 이면 **끝**이다」는 조건이 맞지 않아 적용되지 않는다.
>
> 3. 해악도와 무관하게 올리는 경우 — 해당하지 않는다
>    > 「합(合) 판정의 §5 분류가 축소 · 전환 · 완화이고 등록된 차선책이 아니면(F11) 소유자에게 올린다. … 분류가 확대·정정인지 축소·완화인지 합(合)이 갈라내지 못해도 올린다」
>
>    합(合)은 분류를 **정정**으로 확정했다. 축소·전환·완화가 아니고, 가르지 못한 경우도 아니다. 그래서 이 조항으로는 올리지 않는다.
>
> 4. 기각의 근거 — 셈을 바꾸지 않는다
>    > 「합(合)이 좌표 없이 반론을 기각하면, 그 반론은 원래 등급 그대로 살아남은 것으로 센다.」
>
>    받은 분포에서 근거 없는 기각은 0 이다. 살아남은 것으로 더 셀 반론이 없다.
>
> 5. 상한
>    > 「**라운드 상한 2** … 승격해도 셈을 리셋하지 않는다.」
>
>    1 라운드를 썼다. 상한은 늘리지 않았다.
>
> ### R2 뒤를 위해 적어 둔다 (규칙 인용일 뿐 판단이 아니다)
>
> > 「**R2 종료 판단** — 살아남은 금지역·실패가 0 이면 **끝**이다. 1 이상이면 **소유자에게 올린다.** 승격 칸은 이 물음 하나다. `E3` 판의 승격 칸과 묶지 않는다.」
>
> ## 소유자에게 올린다면 — 물음 한 문장
>
> 해당 없음. 이번 답은 R2 다.

종료 판단 R2 그대로(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-referee-r2.md:11-56`):

> ## 답
> **소유자에게 올린다.**
>
> ## 근거 — 살아남은 반론의 해악도
> | 해악도 | 수 |
> |---|---|
> | 금지역 | 0 |
> | 실패 | 1 (반론 1) |
> | 거짓신호 | 9 (반론 2 · 3 · 4 · 5 · 6 · 7 · 8 · 9 · 10) |
> | 미관 | 2 (반론 11 · 12) |
>
> - 근거 없는 기각은 0 이다. 기각 자체가 없었다. 원래 등급으로 되살려 셀 반론은 없다.
> - 쓴 라운드는 **2**, 상한도 **2** 다. 상한을 다 썼다. 상한은 늘리지 않는다.
>
> ### 적용한 규칙 줄 — 원문 인용
>
> **① 해악도 게이트.** 살아남은 금지역·실패가 0 + 1 = **1** 이다.
>
> > - **R2 종료 판단**
> >   - 살아남은 금지역·실패가 0 이면 **끝**이다.
> >   - 1 이상이면 **소유자에게 올린다.** 승격 칸은 이 물음 하나다. `E3` 판의 승격 칸과 묶지 않는다.
>
> → 1 이상이므로 **올린다.**
>
> **② 해악도와 무관한 게이트.** 합(合)의 §5 분류 칸은 「정정」이다. 그런데 합이 같은 칸에 「완화와 갈라내지 못한다」고 적었다.
>
> > 분류가 확대·정정인지 축소·완화인지 합(合)이 갈라내지 못해도 올린다(§5 「그 판정이 애매하면 혼자 정하지 않는다」).
>
> → 이 조건도 **올린다**를 가리킨다. ①이 없어도 올리게 된다.
>
> **③ 끝 조건이 아닌 것.**
>
> > 「새 반론이 안 나온다」는 끝 조건이 아니다. 라운드 상한과 위 게이트만이 끝을 정한다.
>
> → 거짓신호 9 와 미관 2 는 답을 바꾸지 않는다. 「끝났다」의 조건인 「금지역·실패 0」이 서지 않는다.
>
> ## 소유자에게 올린다면 — 물음 한 문장
>
> **칸 수.** 규칙이 칸 수를 정한 것은 ①뿐이다. ①은 「승격 칸은 이 물음 하나」이고, `E3` 판의 승격 칸과 묶지 않는다. ②는 「해악도와 무관하게 올리는 경우」로 따로 선 조건인데, 칸 수를 적지 않았다. 형식 규칙 「물음 하나에 칸 하나다. 둘을 묶으면 답 하나가 두 물음을 닫은 것처럼 보인다」를 따라, 이 판에서 올리는 칸은 **2** 다. ①이 1칸, ②가 1칸이고, `E3` 판 칸과 섞지 않는다.
>
> **아는 것과 모르는 것.** 반론 1 의 본문은 받지 않았다. 그래서 반론 1 이 ②의 분류 애매함과 같은 사안인지 판단할 수 없다. 같은 사안이더라도 두 칸을 합치는 것은 이 자리가 아니라 소유자가 정한다. 아래 두 문장은 받은 것만으로 쓴 틀이다. 구체 내용은 메인이 합(合) R2 판정문에서 옮겨 채운다.
>
> | 칸 | 물음 |
> |---|---|
> | 1 (해악도 게이트) | 상한 2 라운드를 다 쓴 뒤에도 실패 등급으로 살아남은 반론 1 을 안은 채 판 `p2-caller-sites` 의 판정을 실행해도 되는가? |
> | 2 (분류 게이트) | 합(合) R2 판정의 §5 분류를 「정정」으로 볼 것인가, 「완화」로 볼 것인가? 완화라면 등록된 차선책이 아니므로 승격 요건이 걸린다. |

## 소유자에게 올릴 물음

**종료 판단 R2 의 답이 「소유자에게 올린다」다**(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-referee-r2.md:12`). 칸 수가 합(合) R2 는 1, 종료 판단 R2 는 2 로 갈린다(위 `## 계수` ⑵). 둘 다 옮기고 하나로 맞추지 않는다.

#### 칸 A — 합(合) R2 가 올린 물음

낸 자리: 합(合) R2(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis-r2.md:113-125`) 그대로:

> ## 소유자에게
>
> **올린다.**
>
> 까닭 하나: R2 에서 살아남은 **실패**(반론 1)가 있다. 게이트는 「R2 종료 판단 — 1 이상이면 소유자에게 올린다」다.
>
> 까닭 둘: 합(合)이 이 길의 §5 분류를 **정정과 완화 사이에서 갈라내지 못했다.** 이것도 해악도와 무관하게 올리는 경우다(발췌 `p2-brief-synthesis.md:58` · `SKILL.md:381-383`).
>
> | 칸 | 물음 | 답에 따라 |
> |---|---|---|
> | 1 | `E2` ⑵ · `E4` 의 C 를 「`touch` 가 **센** 호출자의 파일(같은 스냅샷에서 한 심볼로 풀리고 줄 수가 같은 `symbol.callers` 목록으로 읽는다)」로 고치는 것을, 승인 전제 「조건을 약하게 고치지 않는다」(`R/approval.md:13-14`) 아래에서 **정정**으로 받는가. 곧 원문 「깨질 곳을 받는다」(`R/intent.md:20-21`)는 `touch` 출력이 호출자 **자리**를 싣는 것까지 뜻하지 않았나 | **받는다** → 위 「초안을 어떻게 고치나」 1~10 을 실행하고, 자리 싣기는 `## 범위 밖` 한 줄 + 이슈로 둔다. **안 받는다** → 자리 싣기가 원 의도 안이다. 이슈로 떼면 다음 회차의 짐이므로 **확대**로 이 회차에서 `touch` 에 호출자 자리를 싣는다(`SKILL.md:366-368` · `:378`). 조건 문면은 그대로 두고 새 `touch` 출력으로 다시 잰다. 검증 표면: `crates/pal-core/src/touch.rs:243-246` · `:344-346` · `crates/pal-cli/src/touch.rs:505` · `D1` 모집단 `R/intent.md:354` · `B4` `:345` · `scripts/s2-verify.py:140-141`. 새 출력은 `03-touch/` 밖에 둔다(`E1` `:360`) |
>
> `E3` 판의 승격 칸과 묶지 않는다.

#### 칸 B1 · B2 — 종료 판단 R2 가 틀로 낸 두 칸

낸 자리: 종료 판단 R2(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-referee-r2.md:47-56`) — 위 `## 종료 판단` 에 전문이 있다. 칸 수를 정한 문단과 표만 다시 옮긴다:

> **칸 수.** 규칙이 칸 수를 정한 것은 ①뿐이다. ①은 「승격 칸은 이 물음 하나」이고, `E3` 판의 승격 칸과 묶지 않는다. ②는 「해악도와 무관하게 올리는 경우」로 따로 선 조건인데, 칸 수를 적지 않았다. 형식 규칙 「물음 하나에 칸 하나다. 둘을 묶으면 답 하나가 두 물음을 닫은 것처럼 보인다」를 따라, 이 판에서 올리는 칸은 **2** 다. ①이 1칸, ②가 1칸이고, `E3` 판 칸과 섞지 않는다.
>
> **아는 것과 모르는 것.** 반론 1 의 본문은 받지 않았다. 그래서 반론 1 이 ②의 분류 애매함과 같은 사안인지 판단할 수 없다. 같은 사안이더라도 두 칸을 합치는 것은 이 자리가 아니라 소유자가 정한다. 아래 두 문장은 받은 것만으로 쓴 틀이다. 구체 내용은 메인이 합(合) R2 판정문에서 옮겨 채운다.
>
> | 칸 | 물음 |
> |---|---|
> | 1 (해악도 게이트) | 상한 2 라운드를 다 쓴 뒤에도 실패 등급으로 살아남은 반론 1 을 안은 채 판 `p2-caller-sites` 의 판정을 실행해도 되는가? |
> | 2 (분류 게이트) | 합(合) R2 판정의 §5 분류를 「정정」으로 볼 것인가, 「완화」로 볼 것인가? 완화라면 등록된 차선책이 아니므로 승격 요건이 걸린다. |

B1 · B2 의 구체 내용은 종료 판단 R2 가 *「메인이 합(合) R2 판정문에서 옮겨 채운다」*고 적었다. 나는 채우지 않았다.

#### 앞 라운드 기록 — 합(合) R1 이 R2 뒤를 위해 적어 둔 칸

낸 자리: 합(合) R1(`.palimpsest/rounds/2026-09-13-first-release-elsewhere/dialectic/p2-synthesis.md:112-122`) 그대로:

> ## 소유자에게
>
> **안 올린다(지금은) — R2 를 돈다.**
>
> - 살아남은 반론에 **실패**가 있다(행 1 · 4 · 7 · 9). 게이트의 「R1 종료 판단」은 이 경우 R2 를 돌라고 한다. R1 에서 소유자에게 가는 길은 게이트에 없다.
> - 「해악도와 무관하게 올리는 경우」에도 안 걸린다. 내가 고른 분류는 **정정**이다. 축소 · 전환 · 완화가 아니고, 정정인지 완화인지도 위 「왜 정정인가」의 자로 갈라냈다.
> - R2 합(合)이 정정과 완화를 갈라내지 못하거나, R2 뒤에도 금지역·실패가 남으면 그때 올린다. 그 경우의 칸은 하나다.
>
> | 칸 | 물음 |
> |---|---|
> | 1 | 원문의 「깨질 곳을 받는다」(`R/intent.md:20-21`)가 `touch` 화면이 호출자 **자리**를 싣는 것까지 뜻했는가. 뜻했다면 확대로 `touch` 에 자리를 싣고, 아니라면 계획 세부 `:165` 의 하한 수 읽기대로 `E2` ⑵ · `E4` 의 문면을 정정해 질의 목록과 동치 검사로 잰다 |

---

**작업 트리:** 발주 때 환경 머리의 HEAD 는 `fee673b` 였다. 이 보고를 끝낼 때 잰 `git status --porcelain` 은 `?? docs/gates/first-release-elsewhere.md` 이고 HEAD 는 `4f4a176` 이다 — 메인 쪽이 그 사이에 움직였다. 그 가운데 내가 만든 것은 없다. 저장소 파일을 한 글자도 고치지 않았고 커밋하지 않았다. 생성 스크립트 · 이 본문 · 추출 산출은 세션 스크래치패드 `p2-report/` 에만 썼다. 저장소 안 파일은 코드 표기로 적었다(마크다운 링크 없음).
