# 독립 리뷰 R2 — 원 반환문

> 회차 `2026-09-15-clean-uninstall` · 착수 `acd7e82` · 판정 대상 HEAD `65d6c02` · 2026-09-16(UTC)
>
> 받은 것은 잠긴 의도(`intent.md` 원문·개정·승격) · 소유자 지시 `docs/instructions/2026-09-15-owner-direction.md` ·
> `approval.md` · 산출물 전부다. `review/r1-raw.md` 는 **안 읽었다.**
>
> **금지역 목록의 출처** — `.claude/pal/policy.toml` 은 **없다**(`ls .claude/pal/` → `No such file or directory`).
> 잠긴 의도는 금지역 목록을 등록하지 않는다. 그래서 `.claude/skills/round/SKILL.md:863-880` 의 **기본 다섯**을 썼다.
>
> **모집단 확인** — `git log --diff-filter=A acd7e82..HEAD --name-only` 를 돌려 이 회차가 새로 만든 파일을 실제로 확인했다
> (자기장치: `effect/e1-count.py` · `effect/f1-run.sh` · `oracle/*` · 새 시험 파일 다섯 · `tests/common/snapshot.rs`).
> `.claude/` 아래는 이 회차가 **하나도 안 바꿨다**(`git diff --name-only acd7e82..65d6c02 | grep '^\.claude/'` → 0 줄) — 규약 축의 발견은 없다.

## 측정의 바탕 — 어느 런이 무엇을 쟀나

- 런 `35120132604`(`46d4e03`) — 잡 **일곱 전부 `success`**(ubuntu·macos·windows 의 `cargo xtask check` · `cargo xtask test` ·
  `pal doctor --full`, 그리고 `놓는다` 둘 · `받는다` 둘). `gh run view 35120132604 --json jobs` 로 실측.
- **그 런이 잰 코드가 곧 HEAD 의 코드다** — `git diff --name-only 9f768d6..65d6c02 -- crates/` 는 시험 파일 둘뿐이고
  그 둘은 `66ed974`(=`46d4e03` 의 조상)에서 바뀌었다. 즉 `46d4e03..65d6c02` 의 `crates/` 갈림은 **0** 이고,
  `65d6c02` 는 `intent.md` 와 게이트만 바꾼 전사 커밋이다.
- HEAD `65d6c02` 자신의 런 `35121871836` 은 **아직 안 끝났다** — 2026-09-16T16:34Z 기준
  `ubuntu-latest success` · `macos-latest success` · **`windows-latest in_progress`**.
- `effect/e1-count.py --run 35120132604` 를 내가 다시 돌려 **파일 5 · OS 3 · 어긋남 0** 을 재현했고,
  스크립트의 `기대()` 를 직접 불러 소스 쪽 수(`linux/macos/windows` = 13/13/16 · 23/23/26 · 11/11/14 · 25/25/28 · 18/18/21)가
  로그의 `passed` 와 같음을 확인했다.

## 합격선 축

| 조건 | 판정 | 잰 수 | 근거 |
|---|---|---|---|
| A1 | 통과 | 형태 **19**(문면은 15 이상) × 3 OS | `clean_uninstall_settings.rs:143` `a1_모집단마다…` · 모집단 상수 19 개 · 픽스처 19 쌍 · `우리_몫이_생겼다` 로 생김 단언 · 음성 `oracle/T1-negative-A1-reserialize.txt`(빨강 9 줄) · `T1-negative-A1-hooks-only.txt`(9 줄) |
| A2 | 통과 | 골든 **19** | `clean_uninstall_settings.rs:181` · 골든은 `oracle/T1-golden-generation.txt` 의 착수 `acd7e82` 빌드 산출 · 빈 골든 방지 단언 있음 · 음성 `T1-negative-A2-drop-group.txt`(11) |
| A3 | 통과 | 방 **3** | `a3_1`·`a3_2`·`a3_3` · 기대 바이트를 **원본에 같은 텍스트 편집**으로 만든다(`줄_뒤에_넣는다`·`들여쓰기를_바꾼다`·`stop_배열_끝에_묶음을_넣는다`) · 음성 `T1-negative-A3-drop-changed-agent.txt`(3) · `A3-reserialize-uninstall.txt`(9) |
| A4 | 통과 | 방 **4** | `a4_1`~`a4_4` · fixture 매니페스트에 `edits` 가 없음을 단언(착수 형식) · ①만 `HEAD` 바이트 · ②③④ 는 「값만 되돌렸다」 출력 대조 · 음성 `T1-negative-A4-always-head.txt`(6) |
| B1 | 통과 | 시험 1 | `clean_uninstall_palimpsest.rs:173` · uninstall 전 `intent.redb` 개체 단언 · 음성 `T2-negative-B1.txt`(3) |
| B2 | 통과 | 시험 1 | `:196` · 정본·남의 파일 바이트 · 파생물은 지우고 출력 · 음성 `T2-negative-B2.txt`(4) |
| B3 | 통과 | 방 **2**(승인 · 거부) | `:242` `b3()` — 다시 열어 읽은 결박·거부 수 · **정확히 두 경로** · 사용자 `.gitignore` 바이트 · `git status --porcelain -uall` 동일 · 화면 문자열 넷 · 음성 `T2-negative-B3.txt`(7) |
| B4 | 통과 | 방 **4** | `:293`·`:332`·`:354`·`:389` 와 `clean_uninstall_external.rs:715`(③ 봉인·재설치 뒤 `complete`) · 음성 `T2-negative-B4-a/b.txt` · `T3-negative-B4-3.txt` |
| B5 | 통과 | 시험 **2** + 실바이너리 1 판 | `:409`(`git status` 0 줄) · `:450`(`update`→`uninstall` 왕복 · `git check-ignore` 로 새 목록 확인) · 실제 착수 바이너리 왕복은 `oracle/T2-b5-start-binary.txt`(갈린 경로 0) · 음성 `T2-negative-B5.txt`(3) ⟨아래 「있는데 틀린 것」 2 참조 — CI 쪽 방은 모사다⟩ |
| B6 | 통과 | 시험 1 | `:476` · 음성 `T2-negative-B6.txt`(3) |
| B7 | 통과 | 방 **2**(추적 · 비추적) | `:505` `b7()` · 음성 `T2-negative-B7.txt`(4) · `B7-b.txt`(7) |
| B8 | 통과 | 시험 **2** | `:539`(기본 뒤 곧바로 `--purge`) · `:570`(설치한 적 없는 방 · 사용자 `.claude/` 셋이 모집단에 있다) · RED `oracle/U29-red.txt` · 음성 `U29-negative.txt` |
| B9 | 통과 | 시험 1 | `:596` — 결박을 건 방에서 왕복 뒤 `.palimpsest/` 부재 단언 · 음성 `U29-negative.txt` |
| C1 | 통과 | 방 **4**(고친 2 · 그대로 1 · CRLF 1) | `clean_uninstall_blocks.rs:114`·`:134` — 검사 이름 존재 + **모든 검사 ok** 를 순회 단언 · `update` 출력이 파일을 지목 · 음성 `T2-negative-C1.txt`(4) |
| C2 | 통과 | 방 **3** | `:159`·`:176`·`:186` — rc=1 방 둘에서 **워킹트리 스냅샷 불변**까지 단언 · 음성 `T2-negative-C2.txt`(4) |
| D1 | 통과 | 시험 **3** | `clean_uninstall_external.rs:375` — v1 바이트를 `{"version":1,"digest":"…"}\n` 로 **문자열 대조** · 표시 파일 셋 · 표시 없이 `verify met`·`status complete` · 음성 `:416`·`:442` · `T3-negative-D1.txt`(7) |
| D2 | 통과 | 시험 **4** | `:463`·`:539`·`:544`(빈 조상 미리 만든 HOME)·`:549`(표시 파일 지운 방) · Windows 는 실제 `%LOCALAPPDATA%\palimpsest` 를 쓰고 `CI` 가드와 **시작 전 부재 단언**(`:47`)이 있다 — 차선책 ②(PAL_APPROVAL_DIR)를 안 썼다 · 음성 `T3-negative-D2-ledger.txt` · `ancestors-unconditional/unrecorded.txt` |
| D3 | 통과 | 시험 **2** | `:623`·`:629` — 수 2 · 줄 3(수 한 줄 + 경로 둘) · 들여다본 자리 출력 · Y 의 기록이 그 수에 안 듦 · 바이트 불변 · 음성 `T3-negative-D3.txt`(5) |
| D4 | 통과 | 시험 **4** | `:666`(rc 0)·`:672`(rc 1)·`:678`(연결된 worktree 쪽 rc 1)·`:685`(클론 경고 rc 0) · 방에 밖의 기록 다섯이 있음을 먼저 단언 · 음성 `T3-negative-D4.txt`(9) |
| **E1** | **미측정** | 런 **0**(대상 SHA 의 런이 안 끝났다) | 조건이 지목한 것은 **push 한 마지막 SHA = `65d6c02`**(판정 전사를 담은 커밋)의 런이다. 그 런 `35121871836` 은 2026-09-16T16:34Z 현재 `in_progress`(`windows-latest` 미완). 두 원장이 인용한 런 `35120132604` 의 head 는 **`46d4e03`**(그 커밋의 부모)이고 판정 전사를 안 담는다. `e1-count.py --run 35121871836` 은 로그를 못 받아 예외로 멈춘다 |
| F1 | 통과 | 걸음 **4**(이 회차 2 · 착수 2) | `effect/f1-default-round.txt` 말미 — 갈림 21(디렉터리 8) · ① `L` 밖 **0** · ② 화면에 없는 갈림 **0** · ③ 정본 13 자리 직전 바이트 · `f1-purge-round.txt` 갈림 **0** · RED `f1-default-start.txt`(`L` 밖 2711) · `f1-purge-start.txt`(2739) · 대상은 실제 저장소 `ditto` 새 클론(HEAD `aded7ce7`) · 테스트·CI 가 아닌 커밋된 스크립트가 돌렸다 |
| G1 | 통과 | 시험 **2** × 3 OS | 런 `35120132604` 에서 `clean_uninstall_flow` ubuntu 11 · macos 11 · **windows 14** 전부 초록(`e1-count.py` 로 재현) · `clean_uninstall_flow.rs:477`(purge — 워킹트리·HOME 둘 다 불변) · `:494`(기본 — `L` 안 · 정본 직전 바이트 · **운영 상태가 걷혔음까지** 단언) · 조상 기록은 「미리 있던 방/새로 만든 방」 양방향으로 건다(`:367`·`:452`) · 음성 `T4-negative-G1.txt` |

**검산** — 통과 21 · 반증 0 · 대조불가 0 · **미측정 1**(E1) = 22.
두 원장(`intent.md:135` · `docs/gates/clean-uninstall.md:50`·`:55`·`:73`)의 **「통과 22 · 미측정 0」과 어긋난다.**

**음성 대조** — 조건마다 등록됐고 `oracle/*negative*.txt` **26 개**가 전부 비어 있지 않으며 실패 줄을 담는다(파일별 2~11 줄).
E1 셈 스크립트 자신의 양성·음성·**0 개** 대조도 `oracle/E1-script-controls.txt` 에 있다(0 개 방에서 rc=2 로 멈춘다 — 0 을 훑고 초록을 내지 않는다).
**RED 가 실제로 빨갰던 것**도 관측된다 — `oracle/T1-red.txt`·`T2-red.txt`·`T3-red.txt`·`U29-red.txt` · 착수 바이너리 F1 두 판.

## 미측정 목록

| # | 안 잰 조건 | 모집단 | 유효 | 해악도 | 좌표(파일:줄) | 왜 못 쟀나 |
|---|---|---|---|---|---|---|
| 1 | E1 — push 한 마지막 SHA(`65d6c02`)의 CI 런이 세 OS `success` 이고 `e1-count.py` 가 그 런에서 어긋남 0 을 낸다 | 원의도 | 참 | 금지역 | `.palimpsest/rounds/2026-09-15-clean-uninstall/intent.md:135` | 그 SHA 의 런 `35121871836` 이 아직 도는 중이다(`windows-latest in_progress`). 원리상 못 재는 것이 아니라 **아직 안 끝난 것**이다 — 런이 끝나면 잴 수 있다 |

## 의도 축

### 빠진 것

| # | 발견 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 근거(명령·출력) |
|---|---|---|---|---|---|---|---|
| 2 | 없음 — 의도가 요구한 산출 가운데 부재한 것을 못 찾았다. 확대 「`narrative-pending.json` 을 `.gitignore` 블록에 넣는다」도 섰다 | 원의도 | 참 | 미관 | `crates/pal-cli/src/install/layout.rs:310` | `grep -n narrative-pending layout.rs` → `안의_자리 { 패턴: ".palimpsest/narrative-pending.json", 부류: 부류::파생물 }` · `clean_uninstall_palimpsest.rs:459` 가 `git check-ignore` 로 덮임을 단언 |

### 요구되지 않은 것

| # | 발견 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 근거(명령·출력) |
|---|---|---|---|---|---|---|---|
| 3 | 없음 — 새 표면은 전부 조건에 걸린다. `pal-git` 에 더한 `has_origin`·`other_worktrees` 는 D4 가, `--purge`·`--force` 는 B4·C2 가 요구한 것이다 | 원의도 | 참 | 미관 | `crates/pal-git/src/lib.rs` · `crates/pal-cli/src/main.rs` | `git diff acd7e82..65d6c02 -- crates/pal-cli/src/main.rs \| grep '^+'` → `purge: bool` · `force: bool` 둘뿐 · 새 모듈 둘은 부르는 자리가 있다(`install.rs:1328·1332·1355·1484·1593` · `install/hooks.rs:113` · `install/settings.rs:38`) |

### 있는데 틀린 것

| # | 발견 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 근거(명령·출력) |
|---|---|---|---|---|---|---|---|
| 4 | **B5 의 CI 쪽 방은 「착수 커밋 바이너리로 설치한 방」이 아니라 모사다.** 시험은 이 회차 바이너리로 install 한 뒤 매니페스트의 `pal_version` 과 `.gitignore` 블록 `inserted` 만 착수 값으로 되돌린다 — 그래서 settings 는 **위치 보존 편집기가 쓴 바이트**이고 매니페스트에 `edits` 가 남아 A4 의 「옛 설치」 갈래를 안 지난다. 문면의 방을 실제로 지난 것은 `oracle/T2-b5-start-binary.txt` 한 판(macOS·CI 아님)이다 | 원의도 | 참 | 거짓신호 | `crates/pal-cli/tests/clean_uninstall_palimpsest.rs:435`(`착수_바이너리의_방으로`) · `:450` | 파일을 읽었다 — `m["pal_version"] = "0.1.1+acd7e820fca3"` · `ig["inserted"] = 착수_블록` 만 바꾼다. 게이트 근거 표는 B5 를 `clean_uninstall_palimpsest` 25 로만 적어 이 갈림을 말하지 않는다(`docs/gates/clean-uninstall.md:69`) |

## 이번 라운드의 새 발견

| # | 발견 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 근거(명령·출력) |
|---|---|---|---|---|---|---|---|
| 5 | **두 원장이 E1 을 「통과」로 적었는데 그 판정의 근거가 조건이 지목한 커밋의 것이 아니다.** 조건은 「push 한 마지막 SHA(구현과 **판정 전사**를 함께 담은 커밋)의 런」을 요구하는데, 인용된 런 `35120132604` 의 head 는 **`46d4e03`** 으로 전사 커밋 `65d6c02` 의 **부모**다. `65d6c02` 의 런 `35121871836` 은 아직 `in_progress`(`windows-latest`)라 조건이 아직 안 섰다. R1 이 잡은 것과 **같은 자리가 한 칸 위로 옮겨간 것**이다 | 회차기록 | 참 | **금지역**(사실이 아닌 것을 사실로 적음) | E1 | `.palimpsest/rounds/2026-09-15-clean-uninstall/intent.md:135` · `docs/gates/clean-uninstall.md:50`·`:55`·`:73` | `gh api repos/:owner/:repo/commits/65d6c02…/check-runs` → `total 5` 중 `ubuntu-latest in_progress` 등 · `gh run view 35121871836` → `RUN in_progress` · `windows-latest in_progress` · `gh run view 35120132604 --json headSha` → `46d4e03…` · 게이트 머리(`:3`)는 「판정일 2026-09-16(E1 은 push 뒤)」라고 적으면서 표는 통과로 닫았다 |
| 6 | `effect/e1-count.py` 를 **끝나지 않은 런**에 걸면 계약된 `rc=2`(셀 수 없었다)가 아니라 **미포착 `CalledProcessError`** 로 죽는다. 파이썬의 기본 종료 코드는 1 이고, 그것은 이 스크립트에서 **「어긋났다」와 구별되지 않는다**(붉은 쪽이라 통과를 위조하지는 않는다) | 자기장치 | 참 | 거짓신호 | E1 | `.palimpsest/rounds/2026-09-15-clean-uninstall/effect/e1-count.py:196` | `python3 e1-count.py --run 35121871836` → `subprocess.CalledProcessError: … --job 104881384660 --log' returned non-zero exit status 1` (트레이스백). `셀수없다` 는 `잡_로그` 의 `missing` 분기에서만 던져진다 |

⟨#6 은 자기장치지만 **E1 의 판정 수단**이라 금지역 인접이다 — 그러나 실제 해악은 붉은 쪽 오탐이므로 `거짓신호` 로 적고, 처리 방침은 메인이 정한다.⟩

## 자기 산출에 대한 발견

| # | 발견 | 모집단 | 유효 | 해악도 | 조건 | 좌표(파일:줄) | 근거(명령·출력) |
|---|---|---|---|---|---|---|---|
| 7 | **종료 보고 `report.md` 가 없다.** 규약은 자리를 `.palimpsest/rounds/<slug>/report.md` 로 못박았고(`SKILL.md:786`), 종료 판정에 쓰는 `종료했나()` 가 그 파일만 본다 | 회차기록 | 참 | 미관 | `.palimpsest/rounds/2026-09-15-clean-uninstall/`(부재) · `xtask/src/main.rs:38` | `ls .palimpsest/rounds/2026-09-15-clean-uninstall/` → `report.md` 없음. 원장에 이미 열린 행으로 있다(`findings.jsonl:71` `IR1-09` · 상태 `열림` · `처분자리 null`). **이 회차에서 할 수 있는 일이다** — 잔여다 |
| 8 | `state.md` 의 단계 표기가 실물보다 뒤다 — 「독립 리뷰 R1 을 닫는 중(고침 끝 · **재측정과 R2 앞**)」인데 재측정과 두 원장 전사는 `65d6c02` 에서 이미 끝났다 | 회차기록 | 참 | 미관 | `.palimpsest/rounds/2026-09-15-clean-uninstall/state.md:9` | `git log --oneline -1` → `65d6c02 … 재측정한 판정을 두 원장에 전사한다` · `state.md` 머리 단계 문구와 대조 |

## 내가 기각한 것

| # | 기각한 것 | 모집단 | 유효 | 해악도 | 좌표(파일:줄) | 왜 아니었나 |
|---|---|---|---|---|---|---|
| 9 | 「워킹트리에 음성 대조 변이가 커밋 안 된 채 남아 `doctor` 검사 하나가 죽어 있다」 | 저장소 | 거짓 | 미관 | `crates/pal-core/src/doctor.rs:630`→`:728`→`:719` | 잔재가 아니라 **지금 도는 프로세스**다. 두 번 재는 사이에 변이 자리가 옮겨갔고(`:630` → `:728` → `:719`), `ps aux` 가 `cargo test -p pal-core --lib -- doctor::tests::인스턴스_출처가_선언과_다르면_잡힌다 --exact` 를 보여 준다. HEAD 는 깨끗하고 CI 는 커밋된 상태를 쟀다 |
| 10 | 「F1 산출을 낡은 바이너리로 떴다 — 머리글의 판이 `0.1.1+9f768d6c6dd6` 로 HEAD 가 아니다」 | 자기장치 | 거짓 | 미관 | `.palimpsest/rounds/2026-09-15-clean-uninstall/effect/f1-default-round.txt:6` | `git diff --name-only 9f768d6..65d6c02 -- crates/` → 시험 파일 **둘뿐**이고 제품 소스 갈림은 **0**. 그 뒤 커밋들(`27dac33`·`46d4e03`·`65d6c02`)은 원장·게이트만 바꾼다. 버전 문자열만 옛 SHA 를 담을 뿐 제품은 HEAD 와 같다 |
| 11 | 「게이트의 시험 수(18·25·13·23·11)가 파일의 `#[test]` 수(9·16·4·14·2)와 달라 부풀려져 있다」 | 회차기록 | 거짓 | 미관 | `docs/gates/clean-uninstall.md:66-72` | 그 수는 **시험 바이너리가 실제로 돌린 수**이고, E1 이 요구하는 정의(`mod` 선언을 따라간 뒤의 `#[test]`)와 같다. `e1-count.py` 의 `기대()` 를 직접 불러 소스 쪽 수가 로그의 `passed` 와 일치함을 확인했다(15 칸 전부 ✓). 부풀린 수가 아니라 다른 자의 수다 |
| 12 | 「게이트의 『로컬 44 묶음 초록』이 근거 없는 수다」 | 회차기록 | 거짓 | 미관 | `docs/gates/clean-uninstall.md:77` | `ls crates/pal-cli/tests/*.rs \| wc -l` → **43**. 여기에 `unittests` 축 하나를 더하면 44 로 맞는다. 수 자체는 정합이다(로컬 실행 자체를 내가 재현하지는 않았다 — 그래서 합격선 판정에는 안 썼다) |
| 13 | 「`effect/__pycache__/` 가 회차 기록에 커밋됐다」 | 회차기록 | 거짓 | 미관 | `.palimpsest/rounds/2026-09-15-clean-uninstall/effect/__pycache__/` | 디스크에는 있으나 **추적되지 않는다** — `git ls-files \| grep -c pycache` → `0` · `git ls-files effect/` 는 파일 여섯만 낸다 |
| 14 | 「D2 의 Windows 걸음이 차선책 ②(`PAL_APPROVAL_DIR` 우회)로 섰는데 게이트 `## 판정` 에 안 적혔다」 | 원의도 | 거짓 | 미관 | `crates/pal-cli/tests/clean_uninstall_external.rs:47`·`:104` | 반대다. 시험은 `env_remove("PAL_APPROVAL_DIR")` 로 **기본 저장소 자리**를 쓰고, Windows 에서는 `CI` 가드와 `!밖_뿌리().exists()` 부재 단언을 건다. 차선책을 안 썼으니 적을 것도 없다 |

## 끝내도 되는가

**안 된다** — 본 목록에 **금지역 하나**(#5 · E1 전사)가 남았다. 조건이 지목한 SHA `65d6c02` 의 런이 끝나 세 OS `success` 가 되고
`e1-count.py --run 35121871836` 이 어긋남 0 을 내면 그때 E1 이 서고, 그 전까지 두 원장의 「통과 22 · 미측정 0」은 사실이 아니다.
나머지 21 조건은 통과이고, 반증 0 · 대조불가 0 이다.
