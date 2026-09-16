# 교대 상태 — 제거하면 설치한 적 없던 것처럼

> 회차 `2026-09-15-clean-uninstall` · 착수 커밋 `acd7e82`

★ **새 컨텍스트가 받는 것은 「`intent.md` 전문 + 이 파일의 요약」이다.** 직전 산출물을
시드로 받지 마라.

## 지금 단계 — **루프 · 독립 리뷰 R1 을 닫는 중** (고침 끝 · 재측정과 R2 앞)

| 단계 | 상태 |
|---|---|
| 인터뷰 | **닫혔다** — 2 라운드 · 소유자 칸 다섯(`U28`) |
| 착수 관측 | **확보** — `baseline/01-red.txt` · `02-red-approvals.txt` (R1~R11) |
| 사전부검 | **닫혔다** — 1 라운드(상한) · 시나리오 16 · 기각 3 · 원 반환문 `premortem/r1-raw.md` · 처리 방침은 `intent.md` `## 개정` |
| 승격 | **답을 받았다** — 3 칸(봉인·승인은 `--purge` 에서만 · v1 + 표시 파일 · worktree 있으면 거부) · `intent.md` `## 승격` |
| 조건 설계 평가 | R1 **닫혔다** — 걸림 15 · 기각 8 · 원 반환문 `conditions-audit/r1-raw.md` · 처리 방침은 `intent.md` `## 개정`(조건 20 · 허용 목록 `L`). R2 **닫혔다** — 걸림 13(실패 6 을 고침) · 기각 6 · `conditions-audit/r2-raw.md` · 조건 20(승인 요청에 「21」로 잘못 적었다 — 셈 착오 · 문면과 ID 는 그대로) · 스냅샷의 정의 · 상한 소진 |
| 착수 전 기준선 | **확보** — `baseline/00-baseline-start.md` · 항목 24 · 어긋남 15 · 대조불가 14 · ⚠ f06 ① 여섯이 새로 「망가뜨렸는데 통과」(코드 변경 0 · 앞 기준선이 그 자리까지 못 가 가를 수 없다 · 미해명) — 종료 기준선에서 댄다 |
| 승인 | **받았다** — `approval.md` · 승격 4(B6 지우고 경고) · 승격 5(D4 기본도 밖을 안 건드림) |

## 구현 배분 — 2026-09-16

접점 커밋 `2fe292e`(`round::external::걷는다` 스텁 · `tests/common/snapshot.rs` 스냅샷의 정의) 위에서 세 갈래가 격리 worktree 로 갈라졌다.

| 갈래 | 조건 | 파일 소유 | 시험 파일 |
|---|---|---|---|
| T1 설정 편집기 | A1~A4 | `install/settings.rs` · `install/hooks.rs` · 새 편집 모듈 · `manifest.rs` 의 `SettingsEntry` | `clean_uninstall_settings.rs` |
| T2 안쪽 | B1~B7 · C1 · C2 | `install.rs`(`밖의_보고를_싣는다` 밖) · `layout.rs` · `blocks.rs` · `doctor.rs` · `ignore.rs` · `main.rs` · `manifest.rs`(`SettingsEntry` 밖) | `clean_uninstall_palimpsest.rs` · `clean_uninstall_blocks.rs` |
| T3 바깥 | D1~D4 · B4 ③ 봉인 | `round/external.rs` · `approval.rs` · `stop.rs` · `verify.rs` · `install.rs` 의 `밖의_보고를_싣는다` | `clean_uninstall_external.rs` |
| T4 흐름·효과 | G1 · E1 · F1 | 병합 뒤 | `clean_uninstall_flow.rs` · `effect/e1-count.py` · `effect/f1-run.sh` |

증거는 `oracle/T<n>-red.txt` · `oracle/T<n>-negative-<조건>.txt`. 병합 순서는 T1 → T2 → T3(T3 의 `--purge` 시험은 T2 병합 뒤 초록).

- **접점 스텁 머리 주석의 지목된 표현 하나를 main 에서 고쳤다**(`a1d66ee`) — 세 갈래의 `check` 가 그 한 줄로 빨갰을 수 있다. T3 에게 같은 줄로 바꾸라고 알렸다.
- **E1 셈 스크립트는 섰다**(`65a811a` · `95c801c`) — 이미 초록인 시험 파일 여덟으로 양성 · 음성 · 0 개 대조를 댔다(`oracle/E1-script-controls.txt`).
- **F1 대상 — `~/dev/projects/ditto`**(TypeScript 496 · ADR 있음 · origin `https://github.com/incognito050924/ditto.git` · HEAD `aded7ce7`). 로컬 경로에서 새로 클론한 뒤
  origin 을 GitHub 주소로 맞춘다(식별자가 실사용과 같게). `.claude/` · `CLAUDE.md` 가 이미 커밋돼 있어 사용자 파일 보존을 실물로 잰다.
  boxwood 의 `external-client` · `automation-engine` 은 Java 라 결박 승인 걸음이 성립하지 않는다 — 고르지 않았다.
- **게이트 합격선을 측정 전에 등록했다**(`docs/gates/clean-uninstall.md` · 조건 20 전부 미측정). 승인 요청의 「21」은 셈 착오 — `approval.md` 머리에 정정.
- **F1 · G1 의 「결박 승인 하나」 걸음은 `pal bind --note <조각> <심볼>`** — 후보 목록에 기대지 않아 결정론적이다. 문서 조각 후보의 승인·거부는
  `pal narrative --approve` · `--refuse`(둘 다 `intent.redb` 에 기록 — B3 의 「거부만 한 방」이 쓴다).

## 병합 — 2026-09-16

세 갈래가 충돌 없이 main 에 들어왔다. `install.rs` 는 셋이 다 만졌는데 자리가 갈려 자동 병합됐다.

| 갈래 | 병합 커밋 | 시험 |
|---|---|---|
| T3 바깥 | `9f5bb52` | `clean_uninstall_external` 23(합치기 전에는 `--purge` 9 가 빨갛다) |
| T1 설정 | `a35ab51` | `clean_uninstall_settings` 18 · 모집단 19 · 골든과 옛 매니페스트 fixture 는 착수 빌드에서 떴다 |
| T2 안쪽 | `3efec44` | `clean_uninstall_palimpsest` · `clean_uninstall_blocks` · `cargo test -p pal-cli` 전량 초록 |

바뀐 표면 — `pal uninstall --purge` · `--force` · `install::uninstall(target, 제거{purge,force})` · `layout::안의_분류`(옛 `DERIVED` 를 대신한다) ·
`.gitignore` 블록 다섯 줄 · doctor 검사 7 「pal 블록이 넣은 그대로인가」 · `SettingsEntry.edits`(없으면 옛 설치) ·
밖의 표시 파일(`<digest>.project` · 진행 파일 옆) · 조상 기록 `palimpsest/created-ancestors.json` · `밖의_보고.다른_체크아웃_stop_비활성화`.

기존 시험을 고친 곳 셋(전부 강화 방향) — `install.rs` 의 사용자 수정 설정 키 시험(이제 남긴다) · 검사 수 여섯 → 일곱 ·
`install_hooks.rs` 의 훅 검사를 번호와 이름으로 집기. `round_approve_verify.rs` 의 변조 시험은 `.json` 만 고르도록 좁혔다(표시 파일이 생겨서).

**다음 회차 거리로 남길 것** — 기본 uninstall 뒤에는 매니페스트가 사라져 `--purge` 를 바로 못 돌린다(재설치가 필요하다). 조건 문면은 서지만
사용자 경로가 한 걸음 길다(T2 보고 2).

**T4 병합(`853e324`) 과 그 뒤 메인이 한 판정 셋**

1. `version_is_in_the_binary` 실패는 **낡은 빌드**였다 — T4 가 시험만 더해 cargo 가 `pal` 을 다시 안 빌드했고, 바이너리에 직전 HEAD 가 박혀 있었다.
   다시 빌드하니 HEAD 와 같아지고 초록이다. 회귀가 아니다.
2. **F1 ② 의 「각 경로가 화면에 나온다」를 접두로 덮던 자리를 걷었다** — `f1-run.sh` 의 `covered()` 가 부모 디렉터리 경로로도 덮어 주던 고리를 지우고
   **파일은 이름 그대로** 화면에 있어야 한다. 디렉터리 자리는 종류로 걸러 ② 에서 뺀다(조건 문면에도 그렇게 적었다).
3. 그래서 남던 **조상 기록을 제품이 화면에 이름으로 싣게** 고쳤다(`external.rs` 의 `조상_기록이_남았으면_말한다`) — 조건을 약하게 하는 대신 화면이 말한다.

## U29 확대 — 뒷정리까지 (2026-09-16)

소유자 *「uninstall 해서 깔끔하게 정리까지해」*. 「다음 회차 거리」로 미뤄 둔 자리 둘을 이 회차에서 닫았다(`3105fa9` · `22fef11`).

- **B8** — 기본 uninstall 이 정본을 남기고 매니페스트를 지운 뒤에도 `pal uninstall --purge` 가 곧바로 동작한다. 남은 정본(안)과 이 프로젝트의
  밖의 기록만 걷고 `.claude/` 는 안 건드린다. 설치한 적 없는 저장소에서는 걷을 것이 없다고 말하고 한 바이트도 안 쓴다.
- **B9** — 걷은 뒤 `.palimpsest/` 가 비면 우리가 만든 자리인지 안 묻고 지운다.
- 음성 대조 첫 판에서 **B9 가 안 빨개졌다** — 시험 방에 결박이 없어 결함이 안 드러났다. 결박 승인 걸음을 넣어 고쳤다(`oracle/U29-negative.txt`).
- `version_is_in_the_binary` 가 두 번 빨갰던 것은 **낡은 빌드**다(`oracle/version-test-stale-build.txt`). 판정에 쓰는 시험은 **다시 빌드한 뒤** 읽는다.

지금 수치 — `cargo test -p pal-cli` 44 묶음 초록 · `xtask check` 29/29 · F1 두 걸음 통과(착수 바이너리는 두 걸음 어긋남).

⚠ **한때 여기 적혔던 「통과 21 · 미측정 1」은 틀린 판정이었다.** 독립 리뷰 R1 이 실물로 뒤집었다 — 아래를 볼 것.

## 독립 리뷰 R1 — 2026-09-16 (상한 2 중 1)

원 반환문 `review/r1-raw.md`. 발견 17(유효 9 · 기각 8) — 금지역 1 · 실패 2 · 거짓신호 4 · 미관 10.
**두 원장의 「통과 21 · 미측정 1」을 실물이 뒤집었다: 통과 20 · 반증 2(E1 · G1).**

| 발견 | 무엇 | 어떻게 처분했나 |
|---|---|---|
| G1 반증 | Windows CI 에서 흐름 시험 둘이 빨갛다 — `조상 기록이 없다` | **제품이 옳고 시험이 틀렸다.** 조상 기록은 `palimpsest/` 위에 조상을 **새로 만든 방에서만** 쓴다(`approval.rs` 의 `created.is_empty()`). Windows 는 `%LOCALAPPDATA%` 가 이미 있어 안 생기는 것이 옳다. 시험이 그 갈림을 **양방향으로** 걸게 고쳤다(`oracle/G1-windows-shape.txt` — macOS 에서 그 모양을 모사한 양성·음성 대조) |
| E1 반증 | 구현 마지막 커밋(전사 커밋의 부모)에 CI 런이 **원리상** 안 붙는다(`check-runs total_count=0`) | 문면을 **「push 한 마지막 SHA 의 런」**으로 정정(`intent.md` `## 개정`). 잡 이름은 `ubuntu-latest`·`macos-latest`·`windows-latest` 로 `e1-count.py` 가 읽는 자리와 같다 |
| 금지역 1 | 게이트가 세 OS 를 요구하는 G1 을 **macOS 한 판**으로 통과로 적었다 | 게이트 근거 표에 그 사실을 적고, 판정을 세 OS 런으로 미뤘다 |
| 거짓신호 4 | 게이트 「전량 초록」이 CI 사실이 아니다 · `01-completion-scenes.md:88` 이 승격 4 와 반대 · `00-stack.md:496` 의 autocrlf 서술이 거짓 · `f1-run.sh` ② 가 문면보다 약하다 | 넷 다 고쳤다. F1 은 **디렉터리 자리도 「안의 파일이 화면에 있을 때만」 갈음**하게 조이고 두 판을 다시 쟀다 |
| 미관 2 | B8 의 「`.claude/` 를 안 건드린다」 모집단이 0 · 종료 보고가 아직 없다 | 사용자 `.claude/` 파일 셋을 두 방에 넣었다 · 보고는 종료 전에 쓴다(그 행만 열림으로 둔다) |

## 착수 바이너리

릴리스 `v0.1.1` = `pal 0.1.1+b56ef097f158`. 이 저장소의 `./.palimpsest/bin/pal`(없으면 `scripts/pal-release.sh v0.1.1`).
착수 소스 빌드가 필요하면 `git worktree add --detach <스크래치>/wt-start acd7e82` 뒤 `cargo build --release -p pal-cli`.

## 실패한 접근

- **zsh 에서 `for c in "round stop" …; do $P $c` 는 단어를 안 가른다** — `"round stop"` 이 인자 하나로 가서 「하위 명령이 없다」로 나왔다.
  릴리스에 `round approve`·`stop` 이 없다고 오독할 뻔했다. 실측 스크립트는 `bash <<'EOF'` 로 돌린다.
- **zsh 에서 `${PIPESTATUS[0]}` 는 없다**(`pipestatus` 소문자 · 1 부터) — 스크립트가 중간에 멈췄다. 같은 해법.
- **HOME 스냅샷에 `Library/Caches/com.apple.python` 이 끼었다** — 스크립트가 부른 `python3` 의 것이지 `pal` 의 것이 아니었다. HOME 을 재는 스크립트에서 `python3` 를 부르지 않는다.
- **R4 에서 활성화 파일이 사라진 것을 처음엔 원인 모름으로 봤다** — `install.rs:1178` 이 uninstall 앞에서 `disable_if_supported` 를 부른다.
- **이 워킹트리의 `cargo xtask check` 는 FAIL 넷이다 — 셋은 이 회차와 무관하다.** 추적 안 된 `.claude/worktrees/`(옛 에이전트 작업 트리 둘)가
  「죽은 링크 부재」·「사라진 문서를 현재형으로 안 부른다」·「어색한 표현 부재」의 앞자리를 채운다. **깨끗한 클론 + 이 회차 파일 복사**에서는
  FAIL 이 「완수 조건 설계 평가」 하나뿐이다(조건 설계 평가 `r1-raw.md` 가 아직 없어서). 검사는 깨끗한 클론에서 잰다 —
  `git clone -q <저장소> <스크래치>/palimpsest` → 회차 파일 복사 → `CARGO_TARGET_DIR=<저장소>/target cargo run -q -p xtask -- check`.
- **레코드의 기계 칸(`요약`·`경로`·`모집단`·`유효성`·`해악도`)을 손으로 채웠다** — 깨끗한 클론 `check` 의 「회차 레코드」가
  *"기계 칸이 추출기 산출과 갈린다 — 손으로 옮기지 않는다"* 로 빨갰다(사전부검 52 · 조건평가 39). **진행 중 회차는 예외가 없다.**
  기계 칸은 `python3 .claude/skills/round/bin/extract.py <출처> <n> <raw> [findings.jsonl]` 산출을 그대로 쓰고, 사람은 `처분`·`조건`·
  `조건변경`·`사전처분` 만 채운다.
- **사전부검 R1 반환문은 메타를 `- **유효성:** 참 · 모집단 원의도 · …` 한 줄로 적어 추출기가 `모집단` 을 빈칸으로 뽑는다** — 빈칸은 enum 밖이라
  레코드가 성립하지 않는다. 원 반환문을 메인이 고치지 않고, **같은 에이전트에게 형식만 다시 내게 했다.** 첫 반환은 `premortem/r1-first-return.md` 로 남긴다.

- **사전부검 R1 두 번째 반환에 정의에 없는 `- 근거:` 불릿이 끼어 추출기가 legacy-2019 로 읽었다** — `자동프로필` 은 `- 대상:` 과 `- 근거:` 가 함께
  있으면 옛 형식으로 보고 모집단을 `대상` 에서, 유효성을 `근거` 문구에서 다시 계산한다. 그 산출을 그대로 받아 **틀린 판정값 16 칸을 `2aa9982` 에
  커밋했다**(check 는 초록 — 레코드와 추출기가 같으니까). 기계 칸을 추출기에서 뽑은 뒤에도 **원문의 판정값과 한 번 대조**한다. 세 번째 반환을 받았다.

- **작업본이 있는 파일에 `git checkout -- <파일>` 을 썼다** — 음성 대조를 되돌리려다 `clean_uninstall_flow.rs` 의 수정 넷을 통째로 날렸다.
  같은 줄에 「이건 안 쓴다」는 경고를 붙여 뒀는데 **경고보다 실행이 먼저였다.** 임시 패치를 되돌릴 때는 **python 치환으로 넣은 것을 python 치환으로 뺀다.**
- **Windows 실패를 「제품 결함」으로 먼저 읽을 뻔했다** — 실물은 반대다. 조상 기록은 조상을 새로 만든 방에서만 쓰는 것이 설계고,
  OS 를 안 가리고 단언한 시험이 틀렸다. **세 OS 조건은 한 OS 로 판정하지 않는다.**

## 구현 입력

- 설치 목록의 단일 자리: `crates/pal-cli/src/install/layout.rs` (`PAYLOAD` · `OWNED_*` · `DIRS` · `DERIVED`)
- 흐름: `crates/pal-cli/src/install.rs` — `install` :271 · `update` :1001 · `uninstall` :1115
- `settings.json`: `install/settings.rs` `merge`·`unmerge`(둘 다 `to_string_pretty`) · 훅 `install/hooks.rs`
- 블록: `install/blocks.rs` · 매니페스트: `install/manifest.rs`
- 외부 저장소: `round/approval.rs` `default_store` :400 · `approve` :147 · `Record{version,digest}` :33 · `finalization_digest` :101 ·
  `round/stop.rs` `activation_path` :375 · `progress_path` :379 · `disable_if_supported` :166
- 파생물 쓰는 자리: `cache.rs:52` · `narrative.rs:154`(index) · `touch.rs:260`(intent.redb) · `pending.rs:44` · `radius.rs:292`
- 기존 시험: `tests/install*.rs` 7 묶음 141 개(착수 시점 전부 초록) · `tests/round_approve_verify.rs` · `tests/round_stop.rs`
- 프로젝트 식별자: `pal-git/src/lib.rs:315` `stable_repository_identity` — `remote.origin.url` 의 blake3, 없으면 루트 커밋.
  **같은 원격의 두 클론은 같은 프로젝트다** → D2 차선책의 전제가 실물이다
- B3 대조 재료: `pal-intent/src/store.rs` `all()` :398 · `refusals()` :501 · `aliases()` :560 · `export_jsonl()` :735
- `.gitignore` 등재 판정: `install/ignore.rs` `verdict` :153 — git `check-ignore` 에 **구체 경로**를 묻는다.
  `radius-base-*.redb` 는 이름이 변하므로 판정할 표본 경로가 따로 필요하다
- 사전부검 1 라운드: 백그라운드로 띄웠다 — 원 반환문은 `premortem/r1-raw.md` 로 보존한다
