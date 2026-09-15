# 게이트 — 제거하면 설치한 적 없던 것처럼

**회차** `2026-09-15-clean-uninstall` · **소유자 지시** `U28` · **착수** `acd7e82` · **판정일** 미정

> 잠긴 의도 [`intent.md`](../../.palimpsest/rounds/2026-09-15-clean-uninstall/intent.md) ·
> 승인 [`approval.md`](../../.palimpsest/rounds/2026-09-15-clean-uninstall/approval.md) ·
> 소유자 지시 [`2026-09-15-owner-direction.md`](../instructions/2026-09-15-owner-direction.md) `U28`

---

## 합격선

측정 전에 등록한다 — 잠긴 의도의 완수 조건 **20 개**(`A1`~`G1`). 사전부검 1 라운드와 조건 설계 평가 2 라운드(상한)가 문면을 고쳤고
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
| 통과 | — |
| 반증 | — |
| 대조불가 | — |
| 미측정 | A1 A2 A3 A4 B1 B2 B3 B4 B5 B6 B7 C1 C2 D1 D2 D3 D4 E1 F1 G1 |

**검산** — 통과 0 · 반증 0 · 대조불가 0 · 미측정 20 = 20 (`pal round conditions --file intent.md --json` 의 열림 20)

## 효과

미측정 — F1 이 `effect/` 에 붙인다.

## 범위 밖

`intent.md` `## 범위 밖` 이 정본이다 — 요약: `PATH` 의 `pal` 바이너리 · Claude Code 가 만든 파일 · git 이력 · 승인의 자동 내보내기(#166) ·
하위 디렉터리 설치 · 프로젝트 식별자의 정의 · 다른 저장소 자리와 옛 식별자의 기록 · 설치 전부터 있던 pal 파생물의 바이트 ·
pal 이 놓은 파일의 사용자 편집(지우고 경고) · git 이 추적 중인 정본 · Windows 의 HOME 전체 스냅샷.
