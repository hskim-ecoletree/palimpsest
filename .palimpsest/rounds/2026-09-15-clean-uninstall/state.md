# 교대 상태 — 제거하면 설치한 적 없던 것처럼

> 회차 `2026-09-15-clean-uninstall` · 착수 커밋 `acd7e82`

★ **새 컨텍스트가 받는 것은 「`intent.md` 전문 + 이 파일의 요약」이다.** 직전 산출물을
시드로 받지 마라.

## 지금 단계 — **루프 · 구현** (2026-09-16 승인)

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
  레코드가 못 선다. 원 반환문을 메인이 고치지 않고, **같은 에이전트에게 형식만 다시 내게 했다.** 첫 반환은 `premortem/r1-first-return.md` 로 남긴다.

- **사전부검 R1 두 번째 반환에 정의에 없는 `- 근거:` 불릿이 끼어 추출기가 legacy-2019 로 읽었다** — `자동프로필` 은 `- 대상:` 과 `- 근거:` 가 함께
  있으면 옛 형식으로 보고 모집단을 `대상` 에서, 유효성을 `근거` 문구에서 다시 계산한다. 그 산출을 그대로 받아 **틀린 판정값 16 칸을 `2aa9982` 에
  커밋했다**(check 는 초록 — 레코드와 추출기가 같으니까). 기계 칸을 추출기에서 뽑은 뒤에도 **원문의 판정값과 한 번 대조**한다. 세 번째 반환을 받았다.

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
