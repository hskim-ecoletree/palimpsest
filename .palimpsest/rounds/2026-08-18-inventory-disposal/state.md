# 상태 — 재고 처분

**단계**: 인터뷰 소진(라운드 6) → 사전부검 진행 중.

## 착수 기준선 (2026-08-18)

| 잰 것 | 값 | 어떻게 |
|---|---|---|
| `pal --help` 서브커맨드 | **18** (`help` 제외) | `cargo run -q -p pal-cli -- --help` |
| MCP 가 걸린 파일 | **13** | `grep -rln 'mcp\|rmcp'` (코드 7 · 설정 3 · corpus 3) |
| `docs/DESIGN.md` | 2,138 줄 · **절 16** | `grep -c '^## '` |
| `docs/DESIGN.md` 를 가리키는 파일 | **46** | |
| `WHITEPAPER.md` | 480 줄 · `C1~C6`·`U`(§5) · `P1~P14`(§9) | |
| 백서를 가리키는 파일 | **26** | |
| `docs/gates/` | **33 파일** · 12,818 줄 | ★ NEXT-A 는 19 라고 적었다 — **틀린 수** |
| `docs/plan/features/` | **25 파일** · 4,319 줄 | 삭제 대상 |
| 살아있는 문서의 죽은 링크 | **2 건** (모집단 69 파일) | `/tmp/pal-inventory-20260818/deadlinks.sh` |
| `xtask` 등록 검사 | **16** | `xtask/src/main.rs:537` |
| 열린 이슈 | **22** = 에픽 13 + 결함 9 | |

## 확인된 사실 — 판정에 직접 걸리는 것

**① 에픽 이슈 13 건의 본체가 삭제 대상 파일에 산다.** 전부 같은 문장을 담는다:
*"어떻게 만드나: `docs/plan/features/F##-*.md` — 구현 방식·라이브러리·이슈·대안·완료
체크리스트가 **거기 있다. 이 이슈는 그것을 복제하지 않는다**."* F 문서를 지우면
이 이슈들은 빈 포인터가 된다. 결함 이슈 9 건은 측정치를 본문에 품어 자족적이다.

**② `docs/graph-schema.md`·`docs/query-catalog.md` 는 낡는 캐시가 아니다.**
`cargo xtask schema-doc`/`query-doc` 이 코드에서 렌더링하고 `check_schema`·
`check_catalog` 가 양방향 대조한다(`xtask/src/main.rs:1042,1235`). ADR-0024 가 요구하는
형태 그 자체다 → **범위 밖 · 남긴다.**

**③ `corpus/` 의 `mcp` 는 우리 것이 아니다.** 대상 프로젝트(portal-backend)의 Kotlin
경로 문자열이다. 그리고 `corpus/criteria.toml` `[f06.does_not_prove]:5912` 가 이미
*"`pal serve` 도 `rmcp` 도 여기 없다"* 고 적었다 → MCP 삭제가 코퍼스를 안 건드린다.

**④ `xtask` 가 MCP 에 걸린 자리는 둘.**
- `SURFACE_SOURCES:1085` = `["crates/pal-cli/src", "crates/pal-mcp/src"]` — 삭제하면
  `SURFACE_MIN` 검사가 훑는 표면이 하나 준다.
- `BANNED_HOST:24` 의 `"mcp"` — 코어 어휘 금지. **삭제해도 그대로 둔다**(오히려 더 맞다).

**⑤ CI 의 어댑터 부재 갈림은 `.github/workflows/ci.yml:129-144`** —
`cargo test -p pal-cli --no-default-features` · `cargo build --no-default-features`.

**⑥ `00-stack.md` 에 MCP 서술 5 곳** (`:147,198,248,266,433-435,454`),
**`00-risks.md` 에 2 곳** (`:276,279`).

## 실패한 접근

**① `mapfile` 을 썼다.** macOS 의 bash 는 3.2 라 `mapfile`·`readarray` 가 없다.
`set -u` 와 겹쳐 「unbound variable」로 조용히 죽었다. 배열 없이 임시 파일로 다시 썼다.

**② 첫 효과 탐침이 너무 넓었다.** *"이 저장소는 무엇을 만드는 물건인가"* 는
`AGENTS.md`·ADR-0025 가 이미 최신이라 **처분 전에도 맞게 답한다** — 거짓 신호가 안 잡힌다.
탐침을 *"에이전트에 물리려면 무엇을 하나 · P7 은 유효한가 · MCP 는 어떤 자리인가"* 로
좁히자 세션이 스스로 *"문서 세 층이 서로 다른 답을 갖고 있어서"* 라고 진단했다.
**효과 측정은 낡은 문서가 실제로 답을 오염시키는 물음을 골라야 한다.**

## 산출 파일 (회차 밖)

`/tmp/pal-inventory-20260818/` — `effect-before.txt` · `effect-before-2.txt` ·
`help-before.txt` · `deadlinks.sh` · `deadlinks-baseline.txt` · `deadlinks-negctl.txt` ·
`issues.json` · `issues.txt` · `wp-capabilities.md` · `wp-principles.md`

## 착수 기준선 — 실행 (2026-08-18)

| 잰 것 | 값 | 파일 |
|---|---|---|
| `cargo xtask check` | **16/16 통과 · rc=0** · 표면 2 곳 · 소스 35 | `xtask-check-before.txt` |
| `cargo xtask test` | **774 통과 · rc=0** — 이 기계는 초록 | `xtask-test-before.txt` |
| CI 회차 32049575037 @ `2989527` | **macos-latest failure** · 나머지 6 success | `ci-fail.txt` |
| `pal --help` 서브커맨드 | **18** (`help` 제외) | `help-before.txt` |
| 죽은 링크 | **2** (모집단 69 파일) | `deadlinks-baseline.txt` |
| 삭제 대상으로 가는 참조 | **115** = 문서 58 + 소스 주석 57 | |

**★ CI 실패는 내 것이 아니다.** `crates/pal-cli/tests/host_free.rs:302` —
`drop(읽기_하나); drop(읽기_둘)` 직후 쓰기로 여는데
`Database already open. Cannot acquire lock.` 이 난다. 이 기계에서 **3/3 통과**하고
전체 시험도 774/774 통과하므로 **부하 의존**이다. 앞 커밋 `2f70104` 는 7/7 초록이었고
`2989527` 은 문서만 바뀐 커밋이라 코드가 안 변했다.

**★ `docs/gates/F06b.md:38` 이 MCP 없는 세계를 이미 예측해 뒀다** — `755 · 41줄`
(`774 = 755 + 19` · `pal-mcp` 단위 5 + 세션 바이너리 14). 삭제 후 이 예측을 잰다.

## scripts/ 41 판정 (조사 완료)

| 처분 | 수 | 무엇 |
|---|---|---|
| **지운다** | 1 | `f06b-verify.py` — `:37` 이 `cargo test --test mcp_session` 을 첫 줄에서 돌리고 `:179-196` 이 `--help` 의 `serve` 유무를 잰다. 대상이 통째로 사라진다 |
| **고친다** | 1 | `f12-verify.py:477` — **조건 없는 `skip("⑦ MCP 경로", "crates/pal-mcp 가 없다")`.** 메시지가 지금 이미 거짓인데 조건문이 없어 아무도 못 잡는다 |
| **남긴다** | 39 | 접점 0. `f06-verify.py:246`(`if not exists(): 기권`)과 `f11-verify.py:472`(`else: skip`)는 **삭제가 스스로 고친다** — 확인한다 |

★ **오탐을 하나 걸렀다.** `f12`·`f10`·`f02-1` 의 `.ditto` 는 **외부 코퍼스
`~/dev/projects/ditto`** 이지 이 저장소의 `.ditto/` 가 아니다(`f10:51`
`Path.home()/"dev/projects/ditto"`). **이 저장소 `.ditto/` 11 파일을 읽는 스크립트는 0 개**다.

★ `scripts/` 중 **CI 가 실제로 부르는 것은 `interop-produce.sh`·`interop-receive.sh`
둘뿐**이다(`ci.yml:206,260`). 나머지 39 는 사람이 손으로 돌리는 게이트 검증기다.
