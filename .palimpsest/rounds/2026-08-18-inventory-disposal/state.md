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
| 열린 이슈 | **22** = 에픽 **12** + 작업 항목 1(`#41`) + 결함 9 | ★ 내가 처음 「에픽 13」으로 셌고 사전부검이 반증했다 — `#41` 에는 `epic` 라벨이 없다 |

## 확인된 사실 — 판정에 직접 걸리는 것

**① 에픽 이슈 12 건 + 작업 항목 하나(`#41`)의 본체가 삭제 대상 파일에 산다.**
(★ 처음엔 「에픽 13」으로 셌다 — 사전부검이 `#41` 에 `epic` 라벨이 없다는 것을 반증했다.) 전부 같은 문장을 담는다:
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

## 산출

⚠ **이 절을 두 번 고쳤다.** 처음엔 회차 중간(`6c5651f` · 커밋 9)의 수를 적고 그대로
두었다 — 독립 리뷰 두 라운드가 커밋 열을 더 냈는데 표가 안 따라왔다. **회차 문서가
자기 회차의 규모를 틀리게 적는 것**이 이 회차가 없애려던 형태다. 실측으로 다시 적는다.

```
착수 2989527 → 종료 (아래 「검증 기록」의 마지막 SHA)
커밋 19 · 109 files changed · 2,522 insertions(+) · 10,314 deletions(-)
docs/  28,314 → 21,837 줄
```

**생긴 것 열** — `intent.md` · `state.md` · `effect/before.txt` · `effect/after.txt` ·
`effect/after-pinned.txt` · `ADR-0026` · `docs/adr/README.md` ·
`docs/gates/README.md` · `docs/gates/inventory-disposal.md` ·
`docs/plan/disposal-map.md` · `docs/sunset.toml`

**프론티어가 바뀌었다** — 열린 이슈 22 → **11**, **막힘 11 → 0**. 에픽들이 막고 있던
의존이 전부 풀렸다. 착수 가능 11 건 중 **둘**(`#68` sunset · `#69` 문서 엣지)이
「만들 것」이고 아홉이 「고칠 것」이다.
