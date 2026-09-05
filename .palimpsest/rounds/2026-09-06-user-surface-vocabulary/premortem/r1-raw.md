# 사전부검 라운드 1 — 원 반환문

`pal-premortem-sweeper`. 2026-09-06. 금지역 기준은 `SKILL.md:732-745` 기본 다섯
(`.claude/pal/policy.toml` 없음 · `intent.md` 에 금지역 등록 없음 → 3 순위가 진다).

---

### 착수 기준선이 이미 빨갛다 — 이 회차가 만든 정반합 산출물이 검사 17 을 깨뜨렸고 HEAD 에 커밋돼 있다

- 어떻게 실패하나: `cargo xtask check` 를 지금 돌리면 **rc=1 · 22/23** 이다. `죽은 링크 부재` 가 둘을 낸다 — `dialectic/a1-thesis.md → ../../../docs/plan/00-goals.md` 와 `dialectic/d1-thesis.md → ../../../.claude/skills/round/SKILL.md`. `dialectic/` 이 한 겹 깊어서 `../../../../` 이어야 한다. **a1-thesis.md 는 `f91bea2` 에 이미 커밋됐다.** 그러므로 지금 push 하면 세 OS 전부 빨갛고 `H2` 가 못 닫힌다. intent §「RED 관측」의 전제 — *"착수 시점이 23/23 GREEN"* — 는 **이미 참이 아니다.**
- 어디가 걸리나: `dialectic/a1-thesis.md:57` · `dialectic/d1-thesis.md:81` · `xtask/src/main.rs:2461` · `intent.md:75,85`
- 획득: 조회 — `cargo xtask check` 실행(rc=1) · `ls` 로 두 깊이 대조 · `git show` 로 커밋 여부 확인
- 모집단: 회차기록 · 유효성: 참 · 해악도: 실패 · 대상: 계획자신
- 얼마나 아픈가: 되돌릴 수 있다(링크 두 줄). 다만 **RED 관측의 전제 문장**을 고치는 것은 의도 개정이다.

### 「23/23」이 완수 조건 셋에 손으로 박혀 있는데, 같은 회차의 `C2-a`·`D3-a`·`G2-a` 가 검사를 늘린다

- 어떻게 실패하나: `B4`·`D4`·`G3` 가 문면으로 *"`cargo xtask check` **23/23** 통과"* 다. `C2-a` 는 「JSON 에 한국어가 섞이면 실패하는 검사」를, `G2-a` 는 「교정 대상 패턴 검출 수단」을 새로 걸라고 요구한다. `checks` 배열에 하나라도 더하면 `total = checks.len()` 이 24 가 되고 판정문이 `24/24` 를 낸다 — **세 조건이 문면 그대로 반증된다.** `.github/workflows/ci.yml` 이 *"검사 수는 여기 안 적는다 … **세는 자리는 `xtask` 의 `checks.len()` 이다**"* 로 이 거울을 명시적으로 금지한다. 계획이 그 금지를 세 자리에서 되살렸다.
- 어디가 걸리나: `intent.md:104,119,143` ↔ `xtask/src/main.rs:621-646` ↔ `.github/workflows/ci.yml`
- 획득: 조회 — `grep -n "23/23" intent.md` · `sed -n '600,700p' xtask/src/main.rs` · `ci.yml` 전문
- 모집단: 원의도 · 유효성: 참 · 해악도: 실패 · 대상: 계획자신
- 얼마나 아픈가: 되돌릴 수 있다(문면 수정 3 곳). 안 고치면 게이트 표에서 셋이 「반증」이 된다.

### `C2-a` 의 검사는 이 저장소의 실 데이터 16 행에서 첫 실행에 발화한다 — 금지 대상이 사용자가 쓴 결박 노트다

- 어떻게 실패하나: `pal query binding.status --json` 을 이 저장소에서 돌리면 결박 **16 건 전부**의 `note` 필드에 한국어가 있다. `note: String` 은 **사용자가 손으로 쓴 본문**이고 `--json` 으로 그대로 나간다. 「JSON 에 한국어 금지」를 문면대로 구현하면 **사용자 데이터를 금지하는 검사**가 되어 첫 실행부터 빨갛고, 초록으로 만드는 유일한 길이 사용자 노트를 지우는 것이 된다. 막으려던 것은 `{"code":{"freshness":"live"},"lineage":"current"}` 라는 **enum 토큰 필드**에만 있다. `pal touch <sym> --json` 은 지금 한국어 0 자라 `C2` 의 「안 갈려 있으면 가른다」는 발동하지 않는다.
- 어디가 걸리나: `intent.md:110` · `crates/pal-core/src/touch.rs:113` · `crates/pal-cli/src/touch.rs:188-189` · `crates/pal-core/src/binding.rs:510`
- 획득: 조회 — `pal query binding.status --json` 실행 후 `note` 한글 유무 계수(16/16) · `pal touch check_vocabulary --json | grep -c '[가-힣]'` = 0
- 모집단: 자기장치 · 유효성: 참 · 해악도: 실패 · 대상: 계획자신
- 얼마나 아픈가: 되돌릴 수 있다. **범위를 「enum 토큰 필드」로 안 좁히면 그 검사는 영구히 빨갛거나 영구히 꺼져 있다.**

### `B2-a` 음성 대조는 심을 자리가 원리상 없어 답이 미리 정해져 있다

- 어떻게 실패하나: `B2-a` 는 *"옛 표기가 든 픽스처를 넣고 읽기가 실제로 실패하는지 먼저 관측한다. 실패하지 않으면 저장 자리가 없다는 뜻"* 이라 적는다. 그런데 **`CodeFreshness` 를 역직렬화하는 경로가 이 빌드에 없다** — `.palimpsest/intent/bindings.jsonl` 18 행의 키는 `{kind,id,subject,target,note,radius,watch,bound_at,bound_at_time,promoted_by}` 뿐이고 `status` 도 `freshness` 도 없다. 상태는 `BindingStatus::evaluate` 가 매번 계산한다. 픽스처를 어디에 넣든 읽기가 실패하지 않고, 그 침묵으로 「저장 자리가 없다」를 결론하면 **양성 대조가 없는 음성 대조**가 된다.
- 어디가 걸리나: `intent.md:102` · `.palimpsest/intent/bindings.jsonl` · `crates/pal-core/src/binding.rs:575,580`
- 획득: 조회 — `bindings.jsonl` 전 행 파싱해 키 집합 산출 · `grep -o freshness` 계수(0) · `strings *.redb | grep freshness` 로 히트 정체 확인
- 모집단: 자기장치 · 유효성: 참 · 해악도: 금지역(측정이 죽은 가지) · 대상: 계획자신
- 얼마나 아픈가: 되돌릴 수 있다. 고치는 길은 **양성 대조를 붙이는 것** — 같은 픽스처를 실제로 읽는 자리에 심으면 빨개진다를 함께 관측하면 침묵이 정보가 된다.

### `B1` 의 개명 대상 목록에 `xtask` 가 없는데, 검사 13 의 하한이 `"CodeFreshness::Stale"` 문자열이다

- 어떻게 실패하나: `B1` 은 바꿀 자리를 여섯으로 적는다. 그런데 `check_no_regeneration` 이 `binding.rs` 본문에 리터럴 `"CodeFreshness::Stale"` 이 있는지를 **자기가 무언가를 세고 있다는 유일한 증거**로 삼는다. `CodeFreshness` 든 `Stale` 이든 개명하면 `bail!("…이 검사는 아무것도 안 세고 있다")` 로 죽고, 판정문이 개명과 무관한 문구라 원인 추적이 한 번 더 든다.
- 어디가 걸리나: `xtask/src/main.rs:1936,1951` · `intent.md:100` · 부수로 `schema/graph.toml:96,101` · `corpus/criteria.toml:7099,8446`
- 획득: 조회 — `sed -n '1925,1960p' xtask/src/main.rs` · `grep -rn CodeFreshness` 전수
- 모집단: 저장소 · 유효성: 참 · 해악도: 실패 · 대상: 계획대상
- 얼마나 아픈가: 되돌릴 수 있다. 1 곳(+`corpus/criteria.toml` 2 곳은 안 고치는 것이 규칙이라 문서 갈림으로 남는다).

### `touch_recall.rs` 가 존재하지 않는 JSON 키를 읽는다 — `B3`「cargo test 통과」가 개명을 관측하지 못한다

- 어떻게 실패하나: `touch_recall.rs:198` 이 `i["status"]["code"]["state"] != "live"` 로 낡은 것을 거른다. 실제 직렬화 태그는 `"freshness"` 다. `["state"]` 는 **언제나 `Null`** 이고 `Null != "live"` 는 **항상 참**이라 필터가 항등이다. `:206` 의 `assert_ne!` 도 항상 통과한다. 옆의 `binding_status.rs:86` 은 `["freshness"]` 를 옳게 읽어 **두 시험이 서로 모순**이다. `Live` 를 개명해도 `touch_recall.rs` 는 초록이다.
- 어디가 걸리나: `crates/pal-cli/tests/touch_recall.rs:198,206` · `crates/pal-core/src/binding.rs:510` · `crates/pal-cli/tests/binding_status.rs:86,110`
- 획득: 조회 — serde 속성 · 실제 `--json` 산출 · 두 시험 파일 대조. ⚠ 시험을 **돌려서** 확인하지는 않았다.
- 모집단: 저장소 · 유효성: 참 · 해악도: 금지역(측정이 죽은 가지) · 대상: 계획대상
- 얼마나 아픈가: 되돌릴 수 있다(2 줄). 다만 **고치는 순간 시험이 빨개질 수 있다** — 지금 그 단언들이 무엇을 재고 있었는지는 안 재어졌다.

### 「처분」의 파서 계약이 **세 벌**인데 `D2` 는 둘만 이름 댄다 — 셋째가 `xtask` 다

- 어떻게 실패하나: `D2` 는 `record.py:99` 와 `round/status.rs:466,469` 둘을 적는다. 실제로는 셋째 소비자가 있다 — `xtask/src/main.rs:4642-4644` 가 레코드 행에서 `v.get("출처")`·`v.get("처분")`·`v.get("사전처분")` 을 **리터럴 문자열로** 읽는다. 그 함수의 시험 여덟 개가 enum 값을 하드코딩한다. 「전량 이주」를 고르면 이 자리를 안 고치는 순간 검사 22 의 **모집단이 통째로 빠지고** 판정문이 초록이 될 수 있다. 「새 회차부터만」은 `status.rs:495` 의 `matches!` 가 닫힌 집합이라 한쪽만 고치면 `Ok((false, 0))` 로 침묵한다.
- 어디가 걸리나: `xtask/src/main.rs:4642-4644,4392-4432,4425-4512` · `.claude/skills/round/bin/record.py:75,99,120-131` · `crates/pal-cli/src/round/status.rs:466-522`
- 획득: 조회 — `grep -n '"처분"' xtask/src/main.rs` · `record.py` 스키마 전문 · `status.rs` 검증 블록 전문
- 모집단: 저장소 · 유효성: 참 · 해악도: 실패 · 대상: 계획대상
- 얼마나 아픈가: 되돌릴 수 있다. 최소 **6 파일 · 레코드 1462 행 · 예외표 123 행**. 「표시와 이름 분리」가 걸리는 자리를 0 으로 만드는 유일한 갈래다.

### 「오라클」도 두 뜻이고 그중 하나가 **사용자 화면**이다 — `D3` 는 「접힘」만 갈랐다

- 어떻게 실패하나: 「오라클」은 ① 회차의 **완수 조건 판정 오라클**과 ② **「바깥 오라클」**(SQLite CTE 로 우리 인덱스를 채점하는 대조 장치) 둘이다. ②는 사용자 화면에 나간다 — `surface/queries.toml:99` 의 `summary` 가 `pal query --list` 와 `docs/query-catalog.md` 로 렌더링된다. `check_catalog` 방향 3 이 그것을 `catalog.rs:184` 와 **바이트로 대조**하고, 넷째로 `docs/query-catalog.md` 가 생성 산출과 바이트로 대조된다. ①만 개명하면 사용자는 같은 낱말의 다른 뜻을 계속 본다. ②까지 개명하면 **네 자리가 동시에 움직여야** 한다.
- 어디가 걸리나: `surface/queries.toml:99` · `crates/pal-core/src/catalog.rs:184` · `crates/pal-query/src/lib.rs:78` · `crates/pal-store/src/projection.rs:537,540` · `xtask/src/main.rs:1243-1247` · `intent.md:117`
- 획득: 조회 — `grep -rn 오라클` 을 rs/py/toml/yml 로 한정 · `check_catalog` 전문
- 모집단: 저장소 · 유효성: 참 · 해악도: 실패 · 대상: 계획대상
- 얼마나 아픈가: 되돌릴 수 있다. 개명하면 6 곳 + 파생 문서 1(재생성). 안 하면 #111 의 범위가 조용히 좁아진다.

### `E`·`F` 가 새 역할을 세우는데, 발견 원장이 새 출처를 받으려면 **여섯 자리**가 함께 움직인다

- 어떻게 실패하나: 새 역할이 발견을 하나라도 내면 레코드에 실어야 하는데, `record.py:70` 의 `ENUM["출처"]` 는 `["독립리뷰","사전부검","인터뷰","실측"]` 로 **닫힌 집합**이다. 기존 값 하나로 밀어 넣으면 `xtask/src/main.rs:3401` 의 `반환문_자리` 가 그 회차 디렉터리에서 원 반환문을 못 찾아 **검사 21 의 합계 검산이 실패**한다. 새 출처를 정식으로 들이려면 최소 여섯 자리다. 게다가 `.claude/skills/round/bin/extract.py` 는 `PAYLOAD` 에 **없어** 설치본에는 추출기가 애초에 없다. `E1~E5`·`F1~F5` 는 **`install/layout.rs` 두 목록(E4)** 만 적는다.
- 어디가 걸리나: `record.py:70,538-544,575-578,598-604` · `xtask/src/main.rs:3401,3973-4002` · `crates/pal-cli/src/round/status.rs:485-488` · `crates/pal-cli/src/install/layout.rs:27-107` · `intent.md:124-136`
- 획득: 조회 — `record.py` 스키마 전문 · `반환문_자리` 상수 · `layout.rs` 의 `include_str!` 목록 전수(`extract.py` 없음 확인)
- 모집단: 규약 · 유효성: 참 · 해악도: 실패 · 대상: 계획대상
- 얼마나 아픈가: 되돌릴 수 있다(6~7 곳). 「새 역할은 발견을 안 낸다」로 못 박으면 0 곳 — 그러면 그것을 `E2` 와 짝으로 적어야 한다.

### `E1`「정(正)을 서브에이전트로」가 SKILL.md §5 에 **인용된 소유자 지시**를 정면으로 뒤집는데, 계획이 그 충돌을 안 적는다

- 어떻게 실패하나: `SKILL.md:311` 은 *"**정(正) — 메인이 먼저 낸다.** 판정 초안과 근거. 소유자: 「스스로 먼저 결정해보라는 거야」"* 이고, 그 인용의 원문은 동결 문서 `docs/instructions/2026-08-24-owner-direction.md` 에 산다. `intent.md:153` 의 퇴로는 반대로 *"「정(正)을 메인에서 뗀다」는 지킨다"* 라 적는다. **두 소유자 지시가 충돌한다.** 계획이 이 충돌을 어디에도 안 적으므로, 구현자가 `:311` 을 조용히 지우면 저장소는 소유자가 한때 반대를 지시했다는 사실을 잃는다.
- 어디가 걸리나: `.claude/skills/round/SKILL.md:256-262,310-313` · `docs/instructions/2026-08-24-owner-direction.md` · `intent.md:126,153`
- 획득: 조회 — SKILL.md §5 전문 · `ls docs/instructions/`. ⚠ 그 원문 파일의 본문은 안 읽었다.
- 모집단: 규약 · 유효성: 참 · 해악도: 거짓신호 · 대상: 계획대상
- 얼마나 아픈가: 되돌릴 수 있다(2~3 곳). ADR 에 **뒤집힘과 두 지시의 날짜**를 명시하면 닫힌다.

### `C1` 의 병기가 `.name()` 한 자리를 지나면 `pal export` 의 Cypher 값으로 새고, `C2` 의 방어는 `--json` 뿐이다

- 어떻게 실패하나: `C1` 이 요구한 문자열의 단일 생산자는 `Bucket::name()` 과 `ExtractGrade::name()`·`IdentityGrade::name()` 이다. 그 셋을 **사람 화면과 기계 산출 둘 다**가 부른다 — 기계 쪽은 `export.rs:206,249-253` 이 Cypher 노드 속성 값으로 낸다. `.name()` 에 `유효(live)` 꼴을 넣으면 그래프 DB 에 `grade: "완전(full)"` 같은 값이 실린다. `quote()` 가 감싸므로 문법은 안 깨지고 **값만 오염돼 조용하다.**
- 어디가 걸리나: `crates/pal-core/src/ledger.rs:270-281` · `crates/pal-cli/src/export.rs:206,249-253` · `crates/pal-cli/src/ledger.rs:609,620-622` · `intent.md:108-110`
- 획득: 조회 — `grep -rn '\.name()'` 로 소비자 전수 · `export.rs` 본문
- 모집단: 저장소 · 유효성: 참 · 해악도: 거짓신호 · 대상: 계획대상
- 얼마나 아픈가: 되돌릴 수 있다(생산자 3 개). 갈래는 「병기는 `print_*` 에서만 한다」 — 그러면 0 곳이다.

### `G2` 의 교정 대상 패턴이 **파생 문서의 생성기 안**에 산다 — 손으로 고치면 검사 7·8 이 빨개진다

- 어떻게 실패하나: `docs/graph-schema.md:12-20` 의 `값이 선다` 는 `xtask/src/main.rs:1525` 가 낸 것이고, `docs/query-catalog.md:13` 의 `…함께 낸다` 는 `surface/queries.toml:170` ↔ `catalog.rs:187` 의 **바이트 동일 대조**를 지난다. 교정자가 `.md` 를 고치면 `check_schema`/`check_catalog` 가 실패하고, 생성기만 고치고 재생성을 잊어도 같은 실패다. `G3` 는 「죽은 링크 · 사라진 문서 인용」 둘만 이름 대고 파생 문서 대조를 안 적는다.
- 어디가 걸리나: `xtask/src/main.rs:1507-1525,1127-1136,1332-1341` · `surface/queries.toml:170` · `crates/pal-core/src/catalog.rs:187` · `docs/graph-schema.md:12-20` · `docs/query-catalog.md:13` · `intent.md:141-143`
- 획득: 조회 — 두 파생 문서·`queries.toml`·`catalog.rs` 에 패턴 grep · `render_schema_doc` 전문 · 패턴별 파일 수 계수
- 모집단: 저장소 · 유효성: 참 · 해악도: 실패 · 대상: 계획대상
- 얼마나 아픈가: 되돌릴 수 있다(파생 2 + 정본 3). `G1` 의 「고치지 않을 대상 목록」에 **파생 문서 둘**이 들어가야 한다.

### `A2` 의 낱말 충돌이 「근거에 적는다」로만 닫히는데, 실제 충돌은 **와이어 위**에 있고 그것을 재는 것이 없다

- 어떻게 실패하나: 실측한 직렬화 형태는 `{"code":{"freshness":"live"},"lineage":"current"}` 다 — `Lineage` 도 `snake_case` 라 `current` 를 **와이어에** 낸다. `A1` 이 `Current` 를 고르면 기계 소비자가 보는 것은 `{"freshness":"current","lineage":"current"}` 이고 두 축의 값이 같은 토큰이 된다. `A2` 의 완수는 ADR 문면으로 닫히므로 **직렬화 충돌 자체는 아무도 안 잰다.**
- 어디가 걸리나: `crates/pal-core/src/binding.rs:510,563-566` · `intent.md:94`
- 획득: 조회 — `pal query binding.status --json` 실행 후 `status` 필드 덤프 · 두 enum 의 serde 속성
- 모집단: 원의도 · 유효성: 참 · 해악도: 거짓신호 · 대상: 계획대상
- 얼마나 아픈가: 되돌릴 수 있다. 조건 한 줄(「두 축의 **직렬화 토큰**이 겹치지 않는다」를 결정론적으로 잴 것)을 `A2` 에 붙이면 닫힌다.

### `F2` 가 세우는 「체크 가능한 것」의 세 갈래는 SKILL.md §5 가 이미 선언한 축을 **둘째 자리에** 다시 적는다

- 어떻게 실패하나: 같은 가름이 이미 `SKILL.md:271-278` 에 있고 §3 이 조건 형식을 진다. `F1` 이 「단계」를 새로 세우면 **같은 규칙이 §3·§5·새 단계 셋에 살고**, `AGENTS.md` 의 *"같은 것을 두 곳에 적으면 그것이 곧 drift"* 를 회차가 스스로 어긴다. 더 작은 표면 — §3 에 셋째 갈래를 **한 줄 더하고**, 그 판정을 이미 서 있는 `pal-decision-opponent`/`synthesizer` 에 태우는 것. 새 에이전트 0, 새 설치 항목 0.
- 어디가 걸리나: `.claude/skills/round/SKILL.md:57-105,271-278` · `intent.md:132-136` · `AGENTS.md`
- 획득: 조회 — SKILL.md §3·§5 전문 대조 · `AGENTS.md`
- 모집단: 규약 · 유효성: 참 · 해악도: 거짓신호 · 대상: 계획대상
- 얼마나 아픈가: 되돌릴 수 있다. 지금 정하면 0 곳, 만들고 나서 되돌리면 새 에이전트 1 + `layout.rs` 2 목록 + `DIRS` + 시험 둘.

---

## 내가 기각한 것

| 제목 | 어떻게 실패한다고 봤나 | 어디가 걸리나 | 획득 | 모집단 | 유효성 | 해악도 | 대상 | 얼마나 아픈가 |
|---|---|---|---|---|---|---|---|---|
| 「처분」 개명이 `pal round stop` 의 해악 게이트를 조용히 통과시킨다 | `findings_state` 가 스키마 어긋남에 `Ok((false, 0))` 로 **열린 금지역 0** 을 내므로 게이트가 공짜로 열린다고 봤다 | `crates/pal-cli/src/round/status.rs:441-548` · `verify.rs:259-269` | 조회 — `open_harmful_findings` 소비자 전수 추적 | 저장소 | 거짓 | 거짓신호 | 계획대상 | **막힌다.** `verify.rs:259` 가 `!findings_current` 를 먼저 봐 `VerifyError::Invalid` 를 낸다 |
| `CodeFreshness` 가 redb 에 영속돼 있고 착수 관측 「0건」이 틀렸다 | `strings *.redb \| grep -c freshness` 가 10·38 을 내서 상태가 저장돼 있다고 봤다 | `.palimpsest/index.redb` · `intent.redb` | 조회 — `strings` 히트를 실제로 열어 봄 | 저장소 | 거짓 | 미관 | 계획대상 | 히트는 전부 **심볼 이름**과 **문서 경로**다. 직렬화된 상태가 아니다 |
| `bindings.jsonl` 0건이라는 착수 관측이 틀렸다 — 18 행이다 | 파일에 결박 18 행이 실재하므로 「0건」이 사실 아님이라고 봤다 | `intent.md:76` · `bindings.jsonl` | 조회 — 행 파싱(18) 후 `grep -o freshness \| wc -l`(0) | 원의도 | 거짓 | 미관 | 계획대상 | 문맥이 「`CodeFreshness` **영속 저장** 0건」이라 그 읽기로는 참이다 |
| `G2` 의 표현 교정이 마크다운 앵커를 대량으로 깬다 | 제목을 고치면 한국어 앵커가 어긋나 `check_dead_links` 가 다수 발화한다고 봤다 | `xtask/src/main.rs:2732-2762` | 조회 — 앵커 164 개를 뽑아 패턴 교집합 계수 | 저장소 | 거짓 | 미관 | 계획대상 | 실측: 고유 앵커 **164 중 패턴을 진 것은 3**. 「대량」이 아니다 |
| `D3-a` 의 ⚠(「`--root` 가 조용히 무시되는지 잰다」)가 아직 열린 위험이다 | 프리빌트 `xtask` 가 사본을 재는 척하고 원본을 잰다고 봤다 | `xtask/src/main.rs:113-125` | 조회 — `main()` 인자 처리 전문 | 저장소 | 거짓 | 미관 | 계획자신 | **이미 닫혔다.** `--root` 값 부재와 모르는 인자를 거부한다 |

---

## 새 범주

**진행 중 산출물이 상시 검사를 이미 깼다** — 회차가 자기 절차(정반합·사전부검)로 만든
파일이 착수 기준선을 회차 도중에 무효화하는데, 착수 시점 관측표는 그것을 못 본다.
