# 교대 상태 — 제거하면 설치한 적 없던 것처럼

> 회차 `2026-09-15-clean-uninstall` · 착수 커밋 `acd7e82`

★ **새 컨텍스트가 받는 것은 「`intent.md` 전문 + 이 파일의 요약」이다.** 직전 산출물을
시드로 받지 마라.

## 지금 단계 — **조건 설계 평가 R1** (승인 전)

| 단계 | 상태 |
|---|---|
| 인터뷰 | **닫혔다** — 2 라운드 · 소유자 칸 다섯(`U28`) |
| 착수 관측 | **확보** — `baseline/01-red.txt` · `02-red-approvals.txt` (R1~R11) |
| 사전부검 | **닫혔다** — 1 라운드(상한) · 시나리오 16 · 기각 3 · 원 반환문 `premortem/r1-raw.md` · 처리 방침은 `intent.md` `## 개정` |
| 승격 | **답을 받았다** — 3 칸(봉인·승인은 `--purge` 에서만 · v1 + 표시 파일 · worktree 있으면 거부) · `intent.md` `## 승격` |
| 조건 설계 평가 | R1 진행 중(상한 2) — 원 반환문은 `conditions-audit/r1-raw.md` 로 보존한다 |
| 착수 전 기준선 | 진행 중 — `baseline/00-baseline-start.md` |
| 승인 | 안 받았다 |

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
