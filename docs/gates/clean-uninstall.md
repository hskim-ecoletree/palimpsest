# 게이트 — 제거하면 설치한 적 없던 것처럼

**회차** `2026-09-15-clean-uninstall` · **소유자 지시** `U28`·`U29` · **착수** `acd7e82` · **판정일** 2026-09-16(E1 은 push 뒤)

> 잠긴 의도 [`intent.md`](../../.palimpsest/rounds/2026-09-15-clean-uninstall/intent.md) ·
> 승인 [`approval.md`](../../.palimpsest/rounds/2026-09-15-clean-uninstall/approval.md) ·
> 소유자 지시 [`2026-09-15-owner-direction.md`](../instructions/2026-09-15-owner-direction.md) `U28`

---

## 합격선

측정 전에 등록한다 — 잠긴 의도의 완수 조건 **22 개**(`A1`~`G1` · `B8`·`B9` 는 소유자 `U29` 가 확대했다). 사전부검 1 라운드와 조건 설계 평가 2 라운드(상한)가 문면을 고쳤고
소유자가 「승인」으로 잠갔다(`approval.md` · 2026-09-16). **조건의 문면은 `intent.md` 가 정본이고 여기 옮겨 적지 않는다.**

### 갈래

**전부 결정론적** — 세 OS CI 의 통합 시험(`crates/pal-cli/tests/clean_uninstall_*.rs`) · 커밋된 스크립트(`effect/e1-count.py` ·
`effect/f1-run.sh`) · CI 런 조회. **결정론적이 아님 0** — 판정에 정반합을 안 쓴다.

### 스냅샷의 자

모든 스냅샷 조건은 `crates/pal-cli/tests/common/snapshot.rs` 한 자리의 자를 쓴다(`2fe292e`) — 워킹트리는 `.git/` 을 빼되
`.git/config` · `.git/info/exclude` 는 바이트로 넣고, HOME 은 macOS·Linux 에서 격리 HOME 전체 · Windows 에서 `%LOCALAPPDATA%\palimpsest` 하위다.

### RED 관측 — 착수 시점

바이너리 릴리스 `pal 0.1.1+b56ef097f158` · 격리 HOME. 조건별 RED 는 구현 갈래가 `oracle/T<n>-red.txt` 에 남긴다.

| 착수 관측 | 무엇이 남았나 | 기록 |
|---|---|---|
| R1 | 사용자 `settings.json` 이 왕복 뒤 값만 같고 바이트가 다르다(`M`) | `baseline/01-red.txt` |
| R2 | 사용 뒤 uninstall 이 `.palimpsest/` 파생물 넷을 남긴다 · `narrative-pending.json` 이 `??` | `baseline/01-red.txt` |
| R3 | 손으로 고친 블록을 `update` · `doctor --install` 이 못 잡는다 | `baseline/01-red.txt` |
| R4 · R5 | Stop 활성화 디렉터리 · 승인 기록이 HOME 에 남는다 | `baseline/01-red.txt` · `02-red-approvals.txt` |

### 음성 대조

조건마다 `intent.md` 에 적힌 변이를 **임시로** 넣어 빨개지는 출력을 `oracle/T<n>-negative-<조건>.txt` 에 남긴다.
E1 셈 스크립트 자신의 양성 · 음성 · 0 개 대조는 `oracle/E1-script-controls.txt`(`65a811a`).

### 차선책 — 등록된 둘

`intent.md` `## 차선책` — ① 위치 보존 편집이 못 덮는 형태에서만 설치 전 바이트 보관 ② Windows 기본 저장소 걸음이 러너에서 못 서면 `PAL_APPROVAL_DIR` + `default_store` 단위 시험.

## 판정

| 판정 | 조건 |
|---|---|
| 통과 | A1 A2 A3 A4 B1 B2 B3 B4 B5 B6 B7 B8 B9 C1 C2 D1 D2 D3 D4 E1 F1 G1 |
| 반증 | — |
| 대조불가 | — |
| 미측정 | — |

**검산** — 통과 22 · 반증 0 · 대조불가 0 · 미측정 0 = 22 (`pal round conditions --file intent.md --json` 의 열림 22)

⚠ **전사가 두 번 틀렸고 두 번 다 고쳤다.** ① 첫 전사 「통과 21 · 미측정 1」 — 독립 리뷰 R1 이 뒤집었다:
**G1** 은 세 OS 를 요구하는데 macOS 한 판으로 통과를 적었고 Windows CI 에서 그 시험 둘이 빨갰다,
**E1** 은 구현 마지막 커밋에 런이 원리상 안 붙는 문면이었다. ② 그래서 「통과 20 · 반증 2」로 고쳤다.
지금 표는 **고친 뒤 다시 잰 것**이다 — 런 `35120132604`(`46d4e03`) 이 **세 OS 전부 `success`** 이고
`effect/e1-count.py` 가 그 런에 대해 **파일 5 · OS 3 · 어긋남 0** 을 산출했다(`ubuntu·macos 11 / windows 14` 등 열다섯 칸).

### 근거 표

| 조건 | 무엇이 판정했나 | 자리 |
|---|---|---|
| A1~A4 | `clean_uninstall_settings` 18 — 형태 19 의 왕복 바이트 · 착수 빌드 골든 · 사용자 편집 방 셋 · 옛 설치 방 넷 | 시험 · `oracle/T1-*.txt` |
| B1~B7 · C1 · C2 | `clean_uninstall_palimpsest` 25 · `clean_uninstall_blocks` 13 | 시험 · `oracle/T2-*.txt` |
| B8 · B9 | `clean_uninstall_palimpsest` 의 세 시험(곧바로 `--purge` · 설치한 적 없는 방 · 왕복) | 시험 · `oracle/U29-red.txt` · `oracle/U29-negative.txt` |
| D1~D4 | `clean_uninstall_external` 23 — 표시 파일 · 조상 기록 · 두 프로젝트 방 · worktree 넷 | 시험 · `oracle/T3-*.txt` |
| F1 | `effect/f1-run.sh` 두 걸음 — 이번 빌드 통과(기본: 갈림 21 · `L` 밖 0 · 화면에 없는 갈림 0 · 정본 13 자리 직전 바이트 / `--purge`: 갈림 0) · 착수 바이너리 두 걸음 어긋남(`L` 밖 2711 · 2739) | `effect/f1-*.txt` |
| G1 | **세 OS CI 런 `35120132604`(`46d4e03`)** — `clean_uninstall_flow` 가 ubuntu 11 · macos 11 · **windows 14** 전부 초록. 한 흐름을 밟고 `--purge` 뒤 워킹트리·저장소 자리가 설치 전과 같고, 기본 변형은 갈림이 `L` 안에만. ⚠ 앞 런에서는 이 시험 둘이 windows 에서 빨갰다 — 시험이 「조상 기록이 안 생기는 방」을 안 가렸다(독립 리뷰 R1 #1·#4). 고친 자리와 그 모양을 macOS 에서 모사한 양성·음성 대조는 `oracle/G1-windows-shape.txt` | CI 런 · 시험 · `oracle/T4-*.txt` |
| E1 | **통과** — 런 `35120132604`(`46d4e03`) 세 OS `success` · `effect/e1-count.py --run 35120132604` → 파일 5 · OS 3 · **어긋남 0**. 문면은 R1 뒤 「push 한 마지막 SHA 의 런」으로 정정한 것이다 | CI 런 · `intent.md` 의 `## 개정` |

전량(**CI 런 `35120132604` · `46d4e03`**): 잡 일곱 전부 `success` — `ubuntu-latest` · `macos-latest` · `windows-latest` 의
`cargo xtask check` **29/29** · `cargo xtask test` · `pal doctor --full` 구조 판정 `MERGE_BLOCKER_DOCTOR_OK`,
그리고 설치·상호운용 잡 넷(`놓는다` 둘 · `받는다` 둘). 로컬(macOS)은 `cargo test -p pal-cli` **44 묶음 초록**
(다시 빌드한 뒤 — `oracle/version-test-stale-build.txt`).

⚠ **로컬 초록을 CI 초록으로 적지 않는다.** 이 회차에서 그 갈림이 두 번 실물로 났다 —
① 런 `35104425250` 은 로컬 44 묶음 초록인데 windows 의 G1 시험 둘이 빨갰고(그 뒤 걸음 `doctor --full` 은 skipped),
② 런 `35117979526` 은 워킹트리 `check` 가 29/29 인데 **커밋된 상태**에서 「발견이 닫혔나」가 빨갰다
(닫은커밋이 그 발견의 좌표를 안 만졌다). **판정은 커밋된 상태에서, 세 OS 에서 잰다.**

## 효과

**대상** — `ditto`(TypeScript · ADR 있음 · HEAD `aded7ce7` · origin `https://github.com/incognito050924/ditto.git`)의 **새 클론**. 격리 `HOME` · macOS.
절차는 커밋된 `effect/f1-run.sh` 가 진다(설치 → `touch` → `narrative` → 결박 승인 → `round approve` → `round verify` → 종료 봉인 → `stop enable` → Stop 훅 → uninstall).

| 걸음 | 바이너리 | 산출 | 판정 |
|---|---|---|---|
| 기본 uninstall | 이 회차 | `effect/f1-default-round.txt` | **통과** — 갈림 21(디렉터리 8) · `L` 밖 **0** · 화면에 없는 갈림 **0** · 정본 13 자리가 직전 바이트 |
| `--purge` | 이 회차 | `effect/f1-purge-round.txt` | **통과** — 갈림 **0**(워킹트리 · HOME 둘 다 설치 전과 같다) |
| 기본 uninstall | 착수 `v0.1.1` | `effect/f1-default-start.txt` | 어긋남 — 갈림 2729 · `L` 밖 **2711** |
| `--purge` | 착수 `v0.1.1` | `effect/f1-purge-start.txt` | 어긋남 — 갈림 2758 · `L` 밖 **2739** · `--purge` 손잡이가 없어 uninstall rc=2 |

**틀린 답도 적는다** — 첫 판의 F1 은 화면 대조를 부모 디렉터리 경로로 덮어 통과를 산출했다. 그 고리를 걷고 다시 재니 기본 걸음이 어긋났고,
조건을 약하게 고치는 대신 **제품이 남은 것을 경로로 말하게** 고쳐 통과했다(`ecdb96b`).

## 범위 밖

`intent.md` `## 범위 밖` 이 정본이다 — 요약: `PATH` 의 `pal` 바이너리 · Claude Code 가 만든 파일 · git 이력 · 승인의 자동 내보내기(#166) ·
하위 디렉터리 설치 · 프로젝트 식별자의 정의 · 다른 저장소 자리와 옛 식별자의 기록 · 설치 전부터 있던 pal 파생물의 바이트 ·
pal 이 놓은 파일의 사용자 편집(지우고 경고) · git 이 추적 중인 정본 · Windows 의 HOME 전체 스냅샷.
