# 사전부검 R1 — 원 반환문

> 에이전트 `pal-premortem-sweeper` · 2026-09-15~16 · 받은 것: `intent.md`(원문 · 목적 기여 · 착수 관측 · 계획 · 완수 조건 · 차선책 · 범위 밖)와 저장소.
> 안 받은 것: 대화 기록 · 인터뷰 과정 · 앞 라운드 결과.
>
> ⚠ **이 파일은 같은 에이전트의 두 번째 반환이다.** 첫 반환(`r1-first-return.md`)은 메타를 한 줄에 콜론 없이 적어
> 원장 추출기가 `모집단` 을 빈칸으로 뽑았고, 빈칸은 enum 밖이라 레코드가 서지 못했다. 메인이 원문을 고치지 않고
> 같은 에이전트에게 **내용은 그대로 두고 역할 정의의 반환 형식으로만** 다시 내게 했다. 두 반환의 번호 · 제목 ·
> 판정값(모집단 · 유효성 · 해악도)은 같다. 좌표가 없던 14 · 15 번에는 저장소에 있는 파일(`intent.md`)이 첫 좌표로 붙었다.
> 전달 경로가 `<` `>` `&` 를 엔티티로 바꿨던 것만 원래 글자로 되돌렸다.

---

# 사전부검 1 라운드 — 제거하면 설치한 적 없던 것처럼

계획대로 가면 무엇이 깨지는지 16개 시나리오로 정리했다. 가장 무거운 둘은 1번과 4번이다. 둘 다 계획이 사람의 기록을 기본으로 지우게 만든다.
- **1번:** 계획 6이 기본 uninstall에서 종료 봉인까지 지운다.
- **4번:** 계획 1의 `intent.redb` 판정 기준이 narrative 개체와 거부 기록을 못 본다.

셋은 격리 사본에서 직접 돌려 확인했다(4·5·7번). 나머지는 코드를 읽어 얻었다. 돌리지 못한 것은 `추정`이라고 적었다. 저장소 파일은 하나도 고치지 않았고, 실측은 전부 `/private/tmp/claude-501/-Users-incognito-dev-projects-palimpsest/c7c1de0a-52f2-48d8-b5d1-8926cb99dbb6/scratchpad/premortem/` 아래에서 격리 HOME으로 했다.

---

### 1. 기본 uninstall이 종료 봉인을 지워서, 끝난 회차가 끝나지 않은 것으로 보인다
- 어떻게 실패하나: 계획 6은 플래그 없는 uninstall에서도 "이 프로젝트의 v2 기록"을 걷는다. 종료 봉인도 이 기록에 들어 있다. 그런데 `rounds/`는 남으니, 다시 설치하면 보고까지 끝난 회차가 전부 `completion=in_progress`로 읽힌다.
- 어디가 걸리나: `crates/pal-cli/src/round/status.rs:352-381` · `crates/pal-cli/src/round/verify.rs:300-301` · 계획 3과 계획 6이 서로 어긋난다(3은 외부 저장소를 `--purge`에만 넣는데 6은 기본에서 걷는다) · 완수 조건 D2
- 획득: 조회
- 근거: 조회(코드). `Complete`가 되려면 `approval::is_approved(&store, &checkpoint.finalization_seal)`가 참이어야 한다. 이 store는 기본 저장소(`store_location(None)`)이고, verify도 봉인을 늘 `store_dir(config.repo, None)`에 쓴다. 소유자 칸은 "정본 기본 보존, 플래그로만 삭제"다. 게다가 D2는 이 삭제를 성공 조건으로 박는다.
- 관측 신호: 재설치 뒤 `pal round status`에서만 보인다. 시험은 초록이다.
- 모집단: 원의도
- 유효성: 참
- 해악도: 금지역
- 대상: 계획대상
- 얼마나 아픈가: 금지역(끝난 것을 안 끝났다고 적음 · 봉인은 그때의 투영으로만 다시 만들 수 있다)

### 2. 같은 저장소의 다른 클론이나 git worktree에서 uninstall하면, 이쪽 체크아웃의 활성화·진행·봉인이 사라진다
- 어떻게 실패하나: 프로젝트 식별자는 `remote.origin.url`의 해시다(없으면 뿌리 커밋). worktree는 config를 공유하고, 클론은 origin이 같다. 그래서 둘이 한 프로젝트로 셈된다. worktree 하나에서 uninstall하면 메인 체크아웃의 Stop 정책이 조용히 꺼지고, 1번의 완료 판정도 거기서 깨진다. 이 저장소에도 `.claude/worktrees/`가 있다.
- 어디가 걸리나: `crates/pal-git/src/lib.rs:315-331` · 계획 6 · 차선책 둘째 문단 · D2
- 획득: 조회
- 근거: 조회(코드). 차선책은 "한 프로젝트로 보고 화면에 그 사실을 적는다"뿐이고, 지우는 동작은 그대로다.
- 관측 신호: 다른 체크아웃에서는 아무것도 드러나지 않는다(침묵).
- 모집단: 저장소
- 유효성: 참
- 해악도: 금지역
- 대상: 계획대상
- 얼마나 아픈가: 금지역(다른 체크아웃의 데이터 손실)

### 3. 진행 파일과 `.lock` 파일은 걷을 수 없는데, 시험도 효과 확인도 그 경로를 안 지난다
- 어떻게 실패하나: 실제 Claude Code 세션에서 Stop 훅이 돌면 HOME에 두 파일이 남는다. 그러면 `approvals`가 비지 않아 디렉터리 넷도 남는다.
  - **`.lock`:** `round-stop-progress-<digest>.lock`은 만들기만 하고 아무도 안 지운다. 계획 6의 목록에도 없다.
  - **진행 파일:** 파일 이름과 내용에 `activation_digest`(프로젝트+slug)만 있다. slug를 쥔 활성화 파일은 먼저 지워진다(`install.rs:1178`의 `disable_if_supported`, 또는 앞서 `stop disable`). 다른 회차를 enable하면 같은 slug의 진행 파일만 지워서(`stop.rs:121`), 옛 회차 것은 고아가 된다.
- 어디가 걸리나: `crates/pal-cli/src/round/stop.rs:464` · `crates/pal-cli/src/round/stop.rs:152-164` · `crates/pal-cli/src/round/stop.rs:379` · 계획 6 · D2 · F1
- 획득: 조회
- 근거: 조회(코드). `Lease::acquire(&path.with_extension("lock"))`에 대응하는 삭제가 없다. D2와 F1은 `stop enable`까지만 밟는다. 착수 관측 R6에서 보듯, 읽을 수 없는 transcript로는 훅이 먼저 차단해서 이 파일들이 생기지 않는다.
- 관측 신호: 침묵. v1 기록이 아니라서 D3 출력에도 안 잡힌다.
- 모집단: 저장소
- 유효성: 참
- 해악도: 거짓신호
- 대상: 계획대상
- 얼마나 아픈가: 첫 반환에 따로 적지 않았다

### 4. `intent.redb` 조건부 판정이 narrative 개체·거부와 부딪힌다 — B1이 영영 빨갛거나, 사람 기록이 지워진다
- 어떻게 실패하나: `pal narrative`는 문서 조각마다 개체 ID를 새로 만들어 `intent.redb`에 적는다. 사람의 거부 기록(이유 필수)도 여기에 적는다. `intent export`는 결박과 별칭만 내보낸다. 그래서 계획 1의 기준 "사람 기록이 전부 jsonl에 있나"는 둘 중 하나로 실패한다.
  - **(a) 개체·거부까지 센다:** 문서가 있는 저장소에서는 `intent.redb`가 늘 정본이 된다. B1은 빨갛다. 초록이 나온다면 시험 저장소에 문서가 없어서다. R2의 방이 바로 그랬다(문서 0, 개체 0).
  - **(b) 결박만 센다:** B3의 "승인" 표현이 이쪽으로 이끈다. 이러면 거부 기록과 개체 ID가 지워진다. 재설치 뒤 다시 인입하면 다른 ID가 만들어진다. 커밋된 `bindings.jsonl`의 `subject`는 옛 ID를 가리키니 고아가 되고, 거부했던 것을 다시 묻는다.
- 어디가 걸리나: `crates/pal-intent/src/store.rs:735-752` · `crates/pal-intent/src/store.rs:116-126` · `crates/pal-cli/src/narrative.rs:258` · `crates/pal-cli/src/narrative.rs:534` · 계획 1 · 계획 2 · B1 · B3
- 획득: 조회
- 근거: 실측 `premortem/d-*`. `docs/adr/0001-order.md`가 있는 저장소에서 touch 뒤 narrative를 돌리니 `decision/01M2JPXF3WYA2GRMNWZ76HJCEN`이 만들어졌다(`strings intent.redb`에 문서 경로 4건). 그런데 `pal intent export` 출력은 `{"kind":"header","schema_version":3}` 한 줄뿐이었다.
- 관측 신호: (b) 쪽이면 침묵. 재인입 때 같은 질문이 다시 나오는 것으로만 드러난다.
- 모집단: 저장소
- 유효성: 참
- 해악도: 금지역
- 대상: 계획대상
- 얼마나 아픈가: 금지역(데이터 손실)

### 5. B2(설치 전부터 있던 파생물은 바이트 그대로)는 한 번이라도 쓰면 성립할 수 없다
- 어떻게 실패하나: touch는 같은 트리에서도 `index.redb`를 다시 쓰고, `cache/`에 파일을 더한다. 그래서 둘 중 하나다.
  - B2 시험이 사용 걸음 없이 돌아서 공허하게 초록이다. 이 경우 "설치 전에 있었고 그 뒤 썼다"는 실제 경우는 아무도 안 잰다.
  - "안 지운다"가 불어난 `cache/`와 바뀐 index를 남긴다.

  게다가 "설치 전에 있었나"를 적는 매니페스트(`.claude/pal/manifest.json`)는 커밋되어 클론을 따라간다. 다른 사람의 클론에서는 설치한 사람 기계의 답이 쓰인다.
- 어디가 걸리나: `crates/pal-cli/src/attach.rs:106-122` · `crates/pal-cli/src/install/layout.rs:279-282` · 계획 2 · B2
- 획득: 조회
- 근거: 실측 `premortem/e-*`. 같은 트리에서 touch를 다시 돌리자 index.redb 해시가 `0c1e73c5…`에서 `8e7048b2…`로 바뀌었다. 커밋 뒤 touch에서는 `56138b29…`가 됐고, cache 파일은 3개에서 4개로 늘었다.
- 관측 신호: 침묵(시험 초록)
- 모집단: 원의도
- 유효성: 참
- 해악도: 거짓신호
- 대상: 계획대상
- 얼마나 아픈가: 첫 반환에 따로 적지 않았다

### 6. v0.1.1로 이미 설치한 저장소는 `settings.json`을 바이트로 되돌릴 방법이 없다
- 어떻게 실패하나: 옛 install은 이미 파일을 재직렬화했다(R1: 37줄 추가·2줄 삭제). `SettingsEntry`에는 원본 바이트가 없다. 업그레이드 뒤 위치 보존 uninstall을 돌려도 결과는 "재직렬화본에서 우리 것만 뺀 것"이다. 이행 계획은 B5의 `.gitignore` 블록뿐이다. 사용 기록 기간(#160)에 v0.1.1로 설치한 저장소가 여기에 해당한다.
- 어디가 걸리나: `crates/pal-cli/src/install/manifest.rs:149-167` · `crates/pal-cli/src/install/settings.rs:118` · 계획 4 · B5 · F1
- 획득: 조회
- 근거: 조회(코드) + 착수 관측 R1. F1은 새 클론에서만 재서 이 경우를 못 본다.
- 관측 신호: 사용자의 `git status`에 ` M .claude/settings.json`이 남는다. 시험에는 안 잡힌다.
- 모집단: 원의도
- 유효성: 참
- 해악도: 거짓신호
- 대상: 계획대상
- 얼마나 아픈가: 첫 반환에 따로 적지 않았다

### 7. 사용자가 미리 둔 빈 훅 배열이 값째 사라진다 — A1 모집단에 이 형태가 없다
- 어떻게 실패하나: `{"hooks":{"Stop":[]}}`에 install 뒤 uninstall을 하면 `"Stop"` 키가 없어진다. 이벤트 키를 누가 만들었는지 안 보고 지우기 때문이다. A2가 "착수 커밋 규칙과 JSON 값으로 같다"를 박으니, 텍스트 편집도 이 규칙을 따를 것이다. A1의 열 가지 형태에는 빈 이벤트 배열이 없어서 초록이 나온다.
- 어디가 걸리나: `crates/pal-cli/src/install/hooks.rs` `뺀다`(`if 뺐다 && groups.is_empty() { hooks.remove(&entry.event); }`) · 계획 4 · A1 · A2
- 획득: 조회
- 근거: 실측 `premortem/a-*`.
  - before `{"hooks":{"Stop":[]}}` → after `{"hooks": {}}` · ` M .claude/settings.json`
  - before `{"hooks":{"Stop":[],"SubagentStop":[]},"model":"x"}` → after `{"hooks": {}, "model": "x"}`
- 관측 신호: 사용자의 git diff에서만 보인다.
- 모집단: 저장소
- 유효성: 참
- 해악도: 거짓신호
- 대상: 계획대상
- 얼마나 아픈가: 거짓신호(사용자 파일의 값이 바뀐다)

### 8. Windows에서는 D2의 격리 HOME이 기본 저장소를 가두지 못할 수 있다 — 측정이 죽은 가지가 된다
- 어떻게 실패하나: Windows의 기본 저장소는 Known Folder API(`LocalAppData`)로 정해지고, `HOME`·`USERPROFILE` 환경변수로 정해지지 않는다. 이 API가 환경변수를 안 따르면 D2는 러너의 실제 `%LOCALAPPDATA%`에 쓰고 지운다. 그러면 격리 HOME 스냅샷은 늘 같아서 남은 것이 있어도 초록이다. 게다가 지금 시험 중 기본 저장소를 밟는 것이 세 OS 어디에도 없다. 전부 `PAL_APPROVAL_DIR`을 쓴다.
- 어디가 걸리나: `crates/pal-cli/src/round/approval.rs:393-399` · `crates/pal-cli/tests/round_stop.rs:150` · `crates/pal-cli/tests/round_approve_verify.rs:114` · `crates/pal-cli/tests/round_approve_verify.rs:474` · D2
- 획득: 추정
- 근거: 기본 저장소를 안 밟는다는 것은 조회(grep). API가 환경변수를 무시하는지는 추정이다 — Windows에서 돌려 보지 못했다.
- 관측 신호: 침묵
- 모집단: 저장소
- 유효성: 추정
- 해악도: 금지역
- 대상: 계획대상
- 얼마나 아픈가: 금지역(측정이 죽은 가지)

### 9. Linux 실사용 HOME에서는 `.local/share`가 남는다 — D2는 그것을 미리 만들어 두어 숨긴다
- 어떻게 실패하나: 기록을 쓸 때 `create_dir_all`이 `~/.local`과 `~/.local/share`까지 만든다(새 컨테이너·CI 사용자). 계획 6은 `palimpsest/approvals`와 `palimpsest` 둘만 지운다. 그런데 D2는 그 부모를 미리 만든 HOME에서만 잰다.
- 어디가 걸리나: `crates/pal-cli/src/round/approval.rs:119` · `crates/pal-cli/src/round/stop.rs:605` · 계획 6 · D2
- 획득: 조회
- 근거: 조회(코드)
- 관측 신호: 침묵
- 모집단: 원의도
- 유효성: 참
- 해악도: 거짓신호
- 대상: 계획자신
- 얼마나 아픈가: 첫 반환에 따로 적지 않았다 · 대상 표시 원문: 계획자신(조건 설계)

### 10. uninstall은 한 저장소 자리, 한 식별자만 본다 — 다른 자리나 옛 식별자의 기록은 조용히 남는다
- 어떻게 실패하나: 기록이 다른 자리에 있으면 uninstall은 못 찾는다. v1이 아니라서 D3 출력에도 안 나온다.
  - **다른 저장소 자리:** 승인 때 `--approval-dir`나 `PAL_APPROVAL_DIR`를 줬거나, Linux에서 승인과 uninstall 사이에 `XDG_DATA_HOME`이 달라진 경우. uninstall은 `store_location(None)`만 본다.
  - **다른 식별자:** origin URL이 바뀐 경우(https→ssh, 조직 이름 변경). 옛 식별자의 v2 기록과 활성화 파일은 "이 프로젝트 몫"으로 안 잡힌다.
- 어디가 걸리나: `crates/pal-cli/src/round/stop.rs:174` · `crates/pal-cli/src/main.rs:474` · `crates/pal-cli/src/main.rs:496` · `crates/pal-git/src/lib.rs:316` · 계획 6 · D3
- 획득: 조회
- 근거: 조회(코드)
- 관측 신호: 침묵
- 모집단: 저장소
- 유효성: 참
- 해악도: 거짓신호
- 대상: 계획대상
- 얼마나 아픈가: 첫 반환에 따로 적지 않았다

### 11. 옛 바이너리가 v2 기록을 만나면 "승인 없음"으로 읽는다
- 어떻게 실패하나: `Record`는 모르는 필드를 거부하고(`deny_unknown_fields`) `version == 1`만 받는다. v0.1.1이 v2 기록을 읽으면 malformed 오류가 나고, status는 `unwrap_or(false)`로 완료가 아니라고 답한다. 옛 바이너리를 만나는 길은 셋이다.
  - 이 저장소의 `./.palimpsest/bin/pal`(v0.1.1)
  - PATH에 남은 옛 `pal`을 부르는 훅
  - 다운그레이드

  D1은 "새 바이너리가 v1을 읽는다" 방향만 잰다.
- 어디가 걸리나: `crates/pal-cli/src/round/approval.rs:31-36` · `crates/pal-cli/src/round/approval.rs:195` · `crates/pal-cli/src/round/status.rs:375-379` · `crates/pal-cli/src/round/verify.rs:132` · 계획 6 · D1
- 획득: 조회
- 근거: 조회(코드)
- 관측 신호: 옛 바이너리의 status·verify 출력에만 드러난다.
- 모집단: 저장소
- 유효성: 참
- 해악도: 거짓신호
- 대상: 계획대상
- 얼마나 아픈가: 첫 반환에 따로 적지 않았다

### 12. 블록을 원시 바이트로 대조하면, Windows autocrlf 클론이 전부 "손으로 고침"으로 빨개진다
- 어떻게 실패하나: 기존 블록 찾기는 줄바꿈을 맞춘 공간에서 한다. autocrlf 클론의 블록은 CRLF이고 매니페스트의 `inserted`는 LF이기 때문이다(소유자 결정 2026-08-16). 계획 5의 "블록 바이트가 매니페스트와 다르면"을 글자 그대로 구현하면 이 클론들이 모두 빨개진다. 시험은 파일을 직접 쓰고 git checkout 변환을 안 거치니 Windows CI도 초록이다.
- 어디가 걸리나: `crates/pal-cli/src/install/blocks.rs` `자리` · 계획 5 · C1
- 획득: 추정
- 근거: 기존 코드는 조회. 새 검사가 원시 대조를 하리라는 것은 추정이다.
- 관측 신호: 실사용 Windows에서 `doctor --install`이 빨갛다. CI에서는 침묵.
- 모집단: 저장소
- 유효성: 추정
- 해악도: 거짓신호
- 대상: 계획대상
- 얼마나 아픈가: 첫 반환에 따로 적지 않았다

### 13. 끝 마커가 없는 블록에 `uninstall --force`를 쓰면 "마커 사이"가 정해지지 않는다
- 어떻게 실패하나: "훼손" 판정은 여는 마커 하나만 본다(`마커가_있나`가 begin만 찾는다). 사용자가 `pal:end`를 지웠거나 옮겼다면 `--force`의 "마커 사이를 통째로"가 어디까지인지 없다. 파일 끝까지 지우거나, 뒤에 있는 남의 `pal:end`까지 지울 수 있다. 지운 줄을 출력한다고 되돌려지지는 않는다. C2 모집단에는 이 형태가 없다.
- 어디가 걸리나: `crates/pal-cli/src/install/blocks.rs:231-239` · 계획 5 · C2
- 획득: 추정
- 근거: 판정 기준은 조회. 구현이 어떻게 자를지는 추정이다.
- 관측 신호: 화면에 긴 삭제 목록이 나온다.
- 모집단: 원의도
- 유효성: 추정
- 해악도: 금지역
- 대상: 계획대상
- 얼마나 아픈가: 금지역(사용자 CLAUDE.md 데이터 손실)

### 14. 기본 uninstall 뒤 남긴 `intent.redb`가 무시 목록에서 빠져 git에 노출된다
- 어떻게 실패하나: `.gitignore` 블록은 걷히고 정본으로 남긴 `intent.redb`(1,056,768바이트 바이너리)는 그대로 있다. `git status`에 `?? .palimpsest/intent.redb`가 뜨고, `git add -A` 한 번이면 커밋된다.
- 어디가 걸리나: `.palimpsest/rounds/2026-09-15-clean-uninstall/intent.md` · 계획 2 · B3 · F1
- 획득: 조회
- 근거: 조회(코드) + 실측 `d-*`의 파일 크기
- 관측 신호: `git status`
- 모집단: 원의도
- 유효성: 참
- 해악도: 미관
- 대상: 계획대상
- 얼마나 아픈가: 첫 반환에 따로 적지 않았다

### 15. E1 "마지막 커밋에 CI success"를, 판정을 적는 커밋이 스스로 깬다
- 어떻게 실패하나: 종료 보고나 판정을 적는 커밋이 CI가 초록인 커밋 뒤에 붙으면 그것이 새 마지막 SHA가 된다. 그 커밋에는 CI 런이 없다.
- 어디가 걸리나: `.palimpsest/rounds/2026-09-15-clean-uninstall/intent.md` · E1
- 획득: 추정
- 근거: 추정 — 메모리의 앞 회차 기록(ci-last-sha-transcription-conflict)을 근거로 했고, 이번에 다시 재지 않았다.
- 관측 신호: 종료 판정 때 E1이 빨갛다.
- 모집단: 회차기록
- 유효성: 추정
- 해악도: 실패
- 대상: 계획자신
- 얼마나 아픈가: 첫 반환에 따로 적지 않았다

### 16. 봉인 기록만 보면 v2 형식은 필요 없다 — 원장에 이미 목록이 있다
- 어떻게 실패하나: 종료 봉인 digest는 회차 원장의 checkpoint 이벤트(`finalization_seal`)에 이미 적혀 있다. 그러니 `rounds/`를 훑으면 이 프로젝트의 봉인을 형식 변경 없이 셀 수 있다. v2가 꼭 필요한 것은 명령 승인 기록뿐이다. 명령 승인 digest는 PATH·shell 해시를 섞어서 나중에 다시 계산할 수 없다. 반면 v2 형식은 11번의 버전 섞임 비용을 새로 만든다.
- 어디가 걸리나: `crates/pal-cli/src/round/verify.rs:291-300` · 계획 6 · D1
- 획득: 조회
- 근거: 조회(코드)
- 관측 신호: 해당 없음(설계 표면의 크기 문제)
- 모집단: 저장소
- 유효성: 참
- 해악도: 미관
- 대상: 계획대상
- 얼마나 아픈가: 첫 반환에 따로 적지 않았다

---

## 내가 기각한 것

| 제목 | 어떻게 실패하나 | 어디가 걸리나 | 획득 | 모집단 | 유효성 | 해악도 | 대상 | 얼마나 아픈가 |
|---|---|---|---|---|---|---|---|---|
| 글롭 항목이 `.gitignore` 점검을 깬다 | DERIVED에 `radius-base-*.redb`를 더하면 `git check-ignore`에 문자 그대로의 글롭 경로가 가서 검사 5가 틀린다고 봤다 | `crates/pal-cli/src/install/ignore.rs:120` · `crates/pal-cli/src/install/doctor.rs:312` | 조회(실측 `premortem/c-*`: 글롭 경로·실제 이름·디렉터리째 무시 셋 다 rc=0으로 덮였다) | 저장소 | 거짓 | 거짓신호 | 계획대상 | 첫 반환에 따로 적지 않았다 |
| 훅이 남의 묶음 안으로 들어가 텍스트 편집이 중첩 배열을 다뤄야 한다 | 계획이 말하는 "`hooks.<사건>` 배열 원소"가 실제 병합 규칙과 다르다고 봤다 | `crates/pal-cli/src/install/hooks.rs` `더한다` | 조회(코드: `groups.push(json!({ GROUP: [항목(entry)] }))`로 별도 묶음을 넣는다) | 저장소 | 거짓 | 거짓신호 | 계획대상 | 첫 반환에 따로 적지 않았다 |
| 사용자가 이미 둔 `agent` 키를 install이 덮어써서 uninstall이 원래 값을 되살려야 한다 | 원래 값을 보관하지 않으니 바이트 동일이 불가능하다고 봤다 | `crates/pal-cli/src/install/settings.rs:106-111` | 조회(코드: `if !map.contains_key(key)`일 때만 더한다) | 저장소 | 거짓 | 거짓신호 | 계획대상 | 첫 반환에 따로 적지 않았다 |

새 범주: 파일 하나에 파생물과 사람의 기록이 섞여 있어 파일 단위로 분류할 수 없는 자리(4번) · 한 기계의 여러 체크아웃이 식별자 하나를 나눠 쓰는 자리(2번)

시나리오 수: 16(기각 3 별도)
