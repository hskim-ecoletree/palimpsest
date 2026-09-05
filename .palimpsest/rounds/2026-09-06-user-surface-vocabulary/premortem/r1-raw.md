# 사전부검 라운드 1 — 원 반환문

`pal-premortem-sweeper`. 2026-09-06. 금지역 기준은 `SKILL.md:732-745` 기본 다섯(`policy.toml` 없음 · `intent.md` 에 등록 없음).

## 시나리오 (채택 대상)

| # | 제목 | 좌표 | 모집단 | 유효성 | 해악도 | 대상 |
|---|---|---|---|---|---|---|
| PM1-01 | **착수 기준선이 이미 빨갛다** — 이 회차의 정반합 산출물이 「죽은 링크 부재」를 깼고 HEAD 에 커밋됐다. `dialectic/` 이 한 겹 깊어 `../../../` 이 아니라 `../../../../` 이어야 한다. 지금 push 하면 세 OS 전부 빨갛고 `H2` 가 못 닫힌다. intent §RED 관측의 전제(*"착수 시점이 23/23 GREEN"*)가 이미 참이 아니다 | `dialectic/a1-thesis.md:57` · `d1-thesis.md:81` · `xtask/src/main.rs:2461` · `intent.md:75,85` | 회차기록 | 참 | 실패 | 계획자신 |
| PM1-02 | **「23/23」이 완수 조건 셋에 손으로 박혔는데 같은 회차가 검사를 늘린다.** `C2-a`·`G2-a` 가 새 검사를 걸면 `checks.len()` 이 24 가 되어 `B4`·`D4`·`G3` 가 문면 그대로 반증된다. `ci.yml` 이 *"검사 수는 여기 안 적는다 … 세는 자리는 `xtask` 의 `checks.len()` 이다"* 로 이 거울을 명시적으로 금지한다 | `intent.md:104,119,143` · `xtask/src/main.rs:621-646` · `.github/workflows/ci.yml` | 원의도 | 참 | 실패 | 계획자신 |
| PM1-03 | **`C2-a` 의 검사가 실 데이터 16행에서 첫 실행에 발화한다.** `pal query binding.status --json` 의 결박 16건 전부 `note` 에 한국어가 있고 그것은 **사용자가 손으로 쓴 본문**이다. 「JSON 에 한국어 금지」를 문면대로 구현하면 사용자 데이터를 금지하는 검사가 된다. 막으려던 것은 enum 토큰 필드뿐이다. 그리고 `pal touch <sym> --json` 은 지금 한국어 0자라 `C2` 의 「안 갈려 있으면 가른다」는 발동하지 않는다 | `intent.md:110` · `pal-core/src/touch.rs:113` · `pal-cli/src/touch.rs:188-189` · `binding.rs:510` | 자기장치 | 참 | 실패 | 계획자신 |
| PM1-04 | **`B2-a` 음성 대조는 심을 자리가 원리상 없어 답이 미리 정해져 있다.** `CodeFreshness` 를 역직렬화하는 경로가 이 빌드에 없다 — `bindings.jsonl` 18행의 키에 `status` 도 `freshness` 도 없고 상태는 `evaluate` 가 매번 계산한다. 픽스처를 어디 넣든 읽기가 실패하지 않고, 그 침묵으로 결론하면 **양성 대조 없는 음성 대조**다 | `intent.md:102` · `bindings.jsonl` · `binding.rs:575,580` | 자기장치 | 참 | **금지역**(측정이 죽은 가지) | 계획자신 |
| PM1-05 | **`B1` 의 개명 대상 목록에 `xtask` 가 없는데 검사 13 의 하한이 `"CodeFreshness::Stale"` 문자열이다.** `CodeFreshness` 든 `Stale` 이든 개명하면 `bail!("…이 검사는 아무것도 안 세고 있다")` 로 죽고, 판정문이 개명과 무관한 문구라 원인 추적이 한 번 더 든다 | `xtask/src/main.rs:1936,1951` · `intent.md:100` | 저장소 | 참 | 실패 | 계획대상 |
| PM1-06 | **`touch_recall.rs` 가 존재하지 않는 JSON 키를 읽는다.** `:198` 이 `["code"]["state"]` 를 읽는데 실제 태그는 `"freshness"` 다. `Null != "live"` 가 항상 참이라 필터가 항등이고 `:206` 의 `assert_ne!` 도 항상 통과한다. 옆의 `binding_status.rs:86` 은 옳게 읽어 **두 시험이 모순**이다. `B3` 를 「테스트가 개명을 잡았다」로 읽으면 거짓이다 | `pal-cli/tests/touch_recall.rs:198,206` · `binding.rs:510` · `tests/binding_status.rs:86,110` | 저장소 | 참 | **금지역**(측정이 죽은 가지) | 계획대상 |
| PM1-07 | **「처분」의 파서 계약이 세 벌인데 `D2` 는 둘만 이름 댄다.** 셋째가 `xtask/src/main.rs:4642-4644` 이고 시험 여덟 개가 enum 값을 하드코딩한다. 「전량 이주」를 고르면 이 자리를 안 고치는 순간 검사 22 의 **모집단이 통째로 빠지고** 판정문이 초록이 될 수 있다. 「새 회차부터만」은 `status.rs:495` 의 `matches!` 가 닫힌 집합이라 한쪽만 고치면 침묵한다 | `xtask/src/main.rs:4642-4644,4392-4432,4425-4512` · `record.py:75,99` · `status.rs:466-522` | 저장소 | 참 | 실패 | 계획대상 |
| PM1-08 | **「오라클」도 두 뜻이고 그중 하나가 사용자 화면이다.** `surface/queries.toml:99` 의 `"노드와 엣지 전부 — 바깥 오라클이 읽는 창"` 이 `pal query --list` 와 `docs/query-catalog.md` 로 렌더링되고, `check_catalog` 방향 3 이 그것을 `catalog.rs:184` 와 **바이트로 대조**한다. `D3` 는 「접힘」만 갈랐다 | `surface/queries.toml:99` · `catalog.rs:184` · `xtask/src/main.rs:1243-1247` · `intent.md:117` | 저장소 | 참 | 실패 | 계획대상 |
| PM1-09 | **`E`·`F` 가 새 역할을 세우는데 발견 원장이 새 출처를 받으려면 여섯 자리가 함께 움직인다.** `record.py:70` 의 `ENUM["출처"]` 가 닫힌 집합이고, 기존 값으로 밀면 `xtask:3401` 의 `반환문_자리` 가 원 반환문을 못 찾아 합계 검산이 실패한다. 게다가 `extract.py` 는 `PAYLOAD` 에 **없어** 설치본에는 추출기가 애초에 없다. `E1~E5`·`F1~F5` 는 `layout.rs` 두 목록만 적는다 | `record.py:70,538-544,575-578,598-604` · `xtask/src/main.rs:3401` · `status.rs:485-488` · `install/layout.rs:27-107` | 규약 | 참 | 실패 | 계획대상 |
| PM1-10 | **`E1`「정(正)을 서브에이전트로」가 SKILL.md §5 에 인용된 소유자 지시를 정면으로 뒤집는데 계획이 그 충돌을 안 적는다.** `SKILL.md:311` 의 *"정(正) — 메인이 먼저 낸다 … 「스스로 먼저 결정해보라는 거야」"* 와 `intent.md:153` 의 퇴로가 충돌한다. 구현자가 `:311` 을 조용히 지우면 저장소는 소유자가 한때 반대를 지시했다는 사실을 잃는다 | `SKILL.md:256-262,310-313` · `docs/instructions/2026-08-24-owner-direction.md` · `intent.md:126,153` | 규약 | 참 | 거짓신호 | 계획대상 |
| PM1-11 | **`C1` 의 병기가 `.name()` 을 지나면 `pal export` 의 Cypher 값으로 샌다.** `Bucket::name()`·`ExtractGrade::name()`·`IdentityGrade::name()` 을 사람 화면과 `export.rs:206,249-253` 이 함께 부른다. `quote()` 가 감싸므로 문법은 안 깨지고 **값만 오염돼 조용하다.** `C2`·`C2-a` 는 `--json` 만 본다 | `pal-core/src/ledger.rs:270-281` · `pal-cli/src/export.rs:206,249-253` · `intent.md:108-110` | 저장소 | 참 | 거짓신호 | 계획대상 |
| PM1-12 | **`G2` 의 교정 대상 패턴이 파생 문서의 생성기 안에 산다.** `docs/graph-schema.md:12-20` 의 「값이 선다」는 `xtask:1525` 가 낸 것이고, `docs/query-catalog.md:13` 의 「…함께 낸다」는 `queries.toml:170` ↔ `catalog.rs:187` 의 바이트 대조를 지난다. `.md` 를 고치면 검사 7·8 이 실패하고 생성기만 고치고 재생성을 잊어도 같다. `G1` 의 「고치지 않을 목록」에 파생 문서 둘이 들어가야 하는데 후보에 없다 | `xtask/src/main.rs:1507-1525,1127-1136,1332-1341` · `queries.toml:170` · `catalog.rs:187` · `intent.md:141-143` | 저장소 | 참 | 실패 | 계획대상 |
| PM1-13 | **`A2` 의 낱말 충돌이 「근거에 적는다」로만 닫히는데 실제 충돌은 와이어 위에 있다.** `Lineage` 도 `snake_case` 라 `current` 를 와이어에 낸다. `A1` 이 `Current` 를 고르면 `{"freshness":"current","lineage":"current"}` 가 되는데 `A2` 는 ADR 문면으로 닫히므로 **직렬화 충돌 자체는 아무도 안 잰다** | `binding.rs:510,563-566` · `intent.md:94` | 원의도 | 참 | 거짓신호 | 계획대상 |
| PM1-14 | **`F2` 의 세 갈래가 SKILL.md §5 가 이미 선언한 축을 둘째 자리에 다시 적는다.** 같은 가름이 `SKILL.md:271-278` 에 있고 §3 이 조건 형식을 진다. `F1` 이 단계를 새로 세우면 같은 규칙이 셋에 살고 AGENTS.md 의 *"같은 것을 두 곳에 적으면 그것이 곧 drift"* 를 회차가 스스로 어긴다. 더 작은 표면 — §3 에 셋째 갈래를 한 줄 더하고 판정을 이미 선 `pal-decision-opponent`/`synthesizer` 에 태우면 새 에이전트 0 | `SKILL.md:57-105,271-278` · `intent.md:132-136` · `AGENTS.md` | 규약 | 참 | 거짓신호 | 계획대상 |

## 사전부검이 기각한 것

| 제목 | 왜 기각했나 | 유효성 |
|---|---|---|
| 「처분」 개명이 `pal round stop` 의 해악 게이트를 조용히 통과시킨다 | `verify.rs:259` 가 `!findings_current` 를 **먼저** 봐 `VerifyError::Invalid` 를 낸다. 남는 것은 판정문 오도뿐 | 거짓 |
| `CodeFreshness` 가 redb 에 영속돼 있어 착수 관측 「0건」이 틀렸다 | `strings` 히트는 전부 **심볼 이름**(`freshness_consistent`)과 **문서 경로**다. 직렬화된 상태가 아니다 | 거짓 |
| `bindings.jsonl` 0건 관측이 틀렸다 — 18행이다 | 문맥이 「`CodeFreshness` **영속 저장** 0건」이라 그 읽기로는 참이다 | 거짓 |
| `G2` 가 마크다운 앵커를 대량으로 깬다 | 실측: 고유 앵커 **164 중 패턴을 진 것은 3**. 「대량」이 아니다 | 거짓 |
| `D3-a` 의 ⚠(`--root` 가 조용히 무시되는지) 가 아직 열린 위험이다 | **이미 닫혔다.** `xtask:113-125` 가 `--root` 값 부재와 모르는 인자를 거부한다 | 거짓 |

## 새 범주

**진행 중 산출물이 상시 검사를 이미 깼다** — 회차가 자기 절차(정반합·사전부검)로 만든 파일이 착수 기준선을 회차 도중에 무효화하는데, 착수 시점 관측표는 그것을 못 본다.
