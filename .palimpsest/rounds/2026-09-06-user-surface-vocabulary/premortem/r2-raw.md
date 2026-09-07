# 사전부검 라운드 2 — 원 반환문

`pal-premortem-sweeper` 원 반환문. 2026-09-06 · 회차 `2026-09-06-user-surface-vocabulary` · HEAD `1467c2f`.

> **자리 고지.** 받은 것은 잠긴 의도 전문(개정 R1) · 정반합 판정문 셋 · 소유자 1차 원문 사본 · 저장소 전체다.
> 라운드 1 의 반환문·`state.md`·`findings.jsonl` 은 안 읽었다. 좌표가 붙은 것은 전부 직접 열었고,
> 수치는 이 워킹트리에서 다시 셌다.

---

### `ResolutionGrade::name()` 에 병기를 얹으면 `parse` 가 조용히 전멸한다 — `C1` 의 「해소 등급」이 그 자리를 겨눈다

- 어떻게 실패하나: `C1` 이 병기 대상으로 「해소 등급」을 든다. 저장소에서 그 이름을 지는 것은 `ResolutionGrade` 뿐이고, 그 `name()` 은 화면 함수가 아니라 **왕복 파서의 키**다 — `crates/pal-core/src/graph.rs:190` 이 `Self::ALL.into_iter().find(|g| g.name() == raw)` 로 문자열을 되돌린다. `A3` 이 확정한 형식(`좌표 없음(orphaned)` 꼴)을 `name()` 에 얹는 순간 `parse("exact")` 가 `None` 이 되고, 저장된 그래프 엣지의 해소 등급이 전부 되읽기에서 사라진다. 같은 형태가 셋 더 있다 — `graph.rs:63`(`Provenance`) · `query_log.rs:137`(`QueryName`) · `schema.rs:184`.
- 어디가 걸리나: `crates/pal-core/src/graph.rs:179-191` · `crates/pal-core/src/graph.rs:63` · `crates/pal-core/src/query_log.rs:137` · `crates/pal-core/src/schema.rs:184` · `intent.md:109`(`C1`)
- 획득: 조회 — `grep -rn "\.name() ==" --include='*.rs' crates/` 로 왕복 파서 넷을 셌고 `graph.rs:175-195` 를 직접 열었다
- 모집단: 저장소
- 유효성: 참
- 해악도: 금지역
- 대상: 계획대상
- 얼마나 아픈가: 되돌릴 수 있다(문자열 한 줄). 그러나 **아무 검사도 안 잡는다** — `C2-a` 는 「JSON 에 한국어」를 재지 왕복 실패를 안 재고, `parse` 는 `Option` 을 돌려주므로 `None` 이 조용히 「모른다」로 흡수된다. 걸리는 자리 넷

### `C1` 의 「해소 등급」은 사용자 화면에 나가는 자리가 없다 — 병기했다고 적어도 아무도 못 본다

- 어떻게 실패하나: `C1` 이 병기 대상 넷을 든다. 그중 셋(`CodeFreshness` · `pal ledger` 파일 상태 7종 · 추출 등급)은 실제 `println!` 자리가 있다(`touch.rs:346-356` · `query.rs:354-366` · `ledger.rs:609` · `ledger.rs:622` · `query.rs:325`). **「해소 등급」만 없다** — `ResolutionGrade` 는 `crates/pal-cli/src/doctor.rs:254` 에서 엣지를 만들 때 쓰이고 어느 CLI 출력에도 안 찍힌다. 그러면 `C1` 의 넷 중 하나는 「병기했다」가 도달 불가능한 상태이거나, 위 시나리오처럼 `name()` 을 고쳐 파서를 깨뜨리는 것 둘 중 하나다.
- 어디가 걸리나: `intent.md:109`(`C1`) · `crates/pal-cli/src/doctor.rs:254` · `crates/pal-core/src/graph.rs:155,179`
- 획득: 조회 — `grep -rn "ResolutionGrade" crates/pal-cli/src` → `doctor.rs:35,254` 둘뿐. `println!`·`format!` 안에 `ResolutionGrade` 가 든 자리 0건
- 모집단: 저장소
- 유효성: 참
- 해악도: 거짓신호
- 대상: 계획대상
- 얼마나 아픈가: 되돌릴 필요가 없다 — 조건 문면을 고치면 된다. 다만 `A3` 이 *"도달 불가능한 분기에 검사를 거는 것이 「측정이 죽은 가지」다"* 라고 영어권 생략을 이월시킨 근거와 **같은 논리가 여기에도 걸린다** — 그 논리를 여기 안 적용한 것이 어긋남이다

### `D1` 이 일곱 이름을 「확정」만 하고, 그 이름을 코드·문서에 **적용하는 조건이 D 어디에도 없다**

- 어떻게 실패하나: `B` 는 `B1` 이 *"`CodeFreshness` 와 관련 표기가 A1 대로 바뀐다"* 로 **적용**을 진다. `D` 에는 그 짝이 없다 — `D1`(이름 확정) · `D2`(`findings.jsonl` 필드 처리) · `D3`(두 뜻 가르기) · `D4`(검사 통과) · `D5`(ADR). 일곱 중 「퇴로」·「막힘」은 어느 조건에도 적용 좌표가 없고, 「절단」·「봉투」도 없다. `H3` 이 #111 을 닫으면 **이슈 제목이 「회차 어휘 개명」인데 개명된 문자열이 0 건**일 수 있다.
- 어디가 걸리나: `intent.md:117`(`D1`) · `:118`(`D2`) · `:119`(`D3`) · `:121`(`D4`) · `:122`(`D5`) ↔ `intent.md:101`(`B1`)
- 획득: 조회 — 의도 `D` 절 전문 대조. 그리고 대상 낱말의 실재 좌표를 셌다: `.claude/skills/round/SKILL.md` 에 「퇴로」 4 · 「막힘」 11, `.claude/skills/round/bin/dashboard.py` 에 「막힘」 5
- 모집단: 원의도
- 유효성: 참
- 해악도: 거짓신호
- 대상: 계획자신
- 얼마나 아픈가: 되돌릴 것이 없다(아무것도 안 바뀌므로). 걸리는 곳은 조건 다섯. 다음 회차가 「이름은 정해졌는데 왜 안 바뀌었지」로 같은 자리를 다시 연다

### 「절단」이 개정 R1 에서 `D3` 의 셋에서 빠졌다 — 소유자가 이름으로 지목한 낱말이 `H1` 의 증거 출력에 그대로 남는다

- 어떻게 실패하나: `d1-synthesis.md:116,130-134` 가 「절단→생략(elision)」을 `D3` 의 확대로 편입하고 *"한쪽만 고치면 가르기가 성립하지 않는다"* 라고 못 박았다. 그런데 개정 R1 의 `D3` 은 셋을 「`folded.md` · `Fold` · **오라클**」로 적었고 **「절단」이 사라졌다**(개정표도 *"두 뜻 → 셋(「오라클」 추가)"* 로만 적는다). 「절단」은 `D1` 의 일곱에 이름으로만 남고 적용 조건이 없다. 그 낱말은 사용자 화면 넷에 실제로 찍힌다 — 그중 `crates/pal-cli/src/touch.rs:296` 은 **`pal touch` 출력**이고, `H1` 이 그 출력을 게이트 `## 효과` 에 붙이라고 요구한다. 회차 종료 문서가 *"사용자 언어로 읽힌다"* 라고 적으면서 소유자가 *"문맥 상 의도와는 맞지 않아"* 라고 거부한 낱말이 든 출력을 증거로 싣는다.
- 어디가 걸리나: `intent.md:119`(`D3`) · `intent.md:152`(`H1`) · `crates/pal-cli/src/touch.rs:296` · `crates/pal-cli/src/doctor.rs:444` · `crates/pal-cli/src/query.rs:412,415` · `owner-source-9eec834.txt:44`
- 획득: 조회 — 네 좌표를 열어 `println!("  절단      …")` 를 확인했고, `crates/pal-cli/src/touch.rs:301` 이 `evidence::print(e)` 를 부르는 것까지 따라갔다
- 모집단: 원의도
- 유효성: 참
- 해악도: 금지역
- 대상: 계획자신
- 얼마나 아픈가: 되돌릴 수 있다(네 줄). 그러나 되돌릴 기회가 없다 — 어느 검사도 「절단」을 안 세고, `G2` 의 일곱 패턴에도 없다. 소유자가 원문에서 **이름으로 지목하고 대안까지 준** 항목이 회차를 통과해 나간다

### `D3` 의 「접힘」 이주가 **지난 회차 종결 기록 다섯 파일**을 요구하는데, 「범위 밖」이 그것을 영구 금지로 잠갔다

- 어떻게 실패하나: `D3` 이 회차의 `folded.md`(기계 표시)를 대상에 든다. 기계 표시를 실제로 지는 파일은 이 저장소에 다섯이고 **전부 지난 회차의 종결 기록**이다 — `folded.md` 둘(`2026-08-23-check-verifies-work` · `2026-09-01-agent-laziness-merge-evaluation`)과 `state.md` 셋(앞의 둘 + `2026-09-02-agent-laziness-merge-blockers`). 검사 둘이 그 문자열을 요구한다: `xtask/src/main.rs:5036-5041` 이 `folded.md` 에서 `## 왜 접었나` 를, `crates/pal-cli/src/round/status.rs:637-641` 이 `state.md` 의 `## 지금 단계` 본문에서 `contains("접힘")` 를 요구한다. 그런데 `intent.md:169` 「범위 밖」이 *"지난 17회차 종결 기록 … 영구 범위 밖"* 으로 잠갔다. 검사만 고치면 다섯 파일이 빨개지고, 파일을 고치면 잠긴 문장을 어긴다.
- 어디가 걸리나: `intent.md:119`(`D3`) ↔ `intent.md:169`(범위 밖) · `xtask/src/main.rs:5036-5041` · `crates/pal-cli/src/round/status.rs:637-641`
- 획득: 조회 — `ls .palimpsest/rounds/*/folded.md` → 2건 · `grep -ln "접힘" .palimpsest/rounds/*/state.md` → 3건. 검사 둘의 본문을 직접 열었다
- 모집단: 자기장치
- 유효성: 참
- 해악도: 실패
- 대상: 계획자신
- 얼마나 아픈가: 되돌릴 수 있다. 걸리는 곳은 파일 다섯 + 검사 둘 + 잠긴 문장 하나. `D4`(`cargo xtask check` 전량 통과)가 이 충돌을 **강제로 드러낸다** — 회차 중간에 `D` 가 못 닫히고 멈춘다

### `D` 와 `E` 와 `F` 어디에도 `cargo test` 가 없다 — 이 셋이 시험이 잡는 자리를 만진다

- 어떻게 실패하나: 완수 조건에서 `cargo test` 를 요구하는 것은 `B3` 과 `G3` 둘뿐이다. 실행 순서가 A→B→C·D→E→F→G 이므로 `B3` 이후 첫 `cargo test` 는 **회차 맨 끝의 `G3`** 이다. 그 사이에 ① `D3` 이 `status.rs:640` 의 `contains("접힘")` 를 만지면 `crates/pal-cli/tests/round_stop.rs:119,124,464,478` 의 픽스처 넷이 깨진다 ② `E4` 가 `PAYLOAD` 에 새 에이전트를 더하면서 `OWNED_FILES` 를 빠뜨리는 것을 잡는 자는 **`xtask` 가 아니라 유닛 시험** `crates/pal-cli/src/install/layout.rs` 의 `놓는_것은_전부_되돌릴_수_있다` 다(`xtask` 의 검사 23 개 목록에 `PAYLOAD`↔`OWNED_FILES` 짝을 재는 것이 없다). `D4`·`E5`·`F5` 는 전부 초록을 낸 채 #111·#112·#113 을 닫는다.
- 어디가 걸리나: `intent.md:104`(`B3`) · `:121`(`D4`) · `:126-133`(`E`) · `:136-140`(`F`) · `:147`(`G3`) · `xtask/src/main.rs:621-644`(검사 23) · `crates/pal-cli/src/install/layout.rs:296-311` · `crates/pal-cli/tests/round_stop.rs:119,124,464,478`
- 획득: 조회 — `xtask/src/main.rs` 의 `checks` 배열 전문을 읽어 짝 검사가 없음을 확인했고, `layout.rs` 의 `mod tests` 에서 짝 시험을 찾았다
- 모집단: 자기장치
- 유효성: 참
- 해악도: 실패
- 대상: 계획대상
- 얼마나 아픈가: 되돌릴 수 있다. 걸리는 곳은 조건 넷(`D4`·`E`·`F`·`H3`). `H3` 이 이슈 일곱을 닫는데, 그중 셋이 **시험이 빨간 상태로 닫힐 수 있다**

### `B1` 이 `xtask/src/main.rs:1935` 를 넣은 근거가 사실이 아니다 — `A1` 이 `Stale` 을 유지하기로 했다

- 어떻게 실패하나: `B1` 이 *"`check_no_regeneration` 의 하한이 `"CodeFreshness::Stale"` 문자열이다 — **안 고치면 검사 하나가 아무것도 안 세게 된다**"* 라고 적었다. 그런데 같은 의도의 `A1` 이 **`Stale` 을 유지**로 확정했다. `Live → Fresh` 만 바꾸면 `crates/pal-core/src/binding.rs` 에 `CodeFreshness::Stale` 이 그대로 남아(641·731·864·867 행 넷) 하한이 계속 참이다. 근거가 거짓인 채로 잠긴 의도에 실려 있고, 그것을 곧이곧대로 「고쳐야 한다」고 읽으면 **멀쩡한 하한을 건드린다.**
- 어디가 걸리나: `intent.md:101`(`B1`) ↔ `intent.md:93`(`A1`) · `xtask/src/main.rs:1935,1949` · `crates/pal-core/src/binding.rs:641,731,864,867`
- 획득: 조회 — `grep -rn "CodeFreshness::Stale" crates/pal-core/src/binding.rs crates/pal-query/src/lib.rs` → 5건 실재
- 모집단: 자기장치
- 유효성: 참
- 해악도: 거짓신호
- 대상: 계획자신
- 얼마나 아픈가: 되돌릴 수 있다. 한 곳. 그러나 `D5`·`A4` 의 ADR 이 이 문장을 근거로 옮겨 적으면 **동결 문서에 거짓이 들어간다**

### `B1` 의 `schema/graph.toml` 은 `live` 가 딱 하나이고 그것은 **인용문**이다 — 고치면 얼린 정본과 갈린다

- 어떻게 실패하나: 개정 R1 이 `B1` 목록에서 `surface/queries.toml`(0건)을 뺐다. 그런데 같은 목록의 `schema/graph.toml` 은 `live` 가 **1건**이고, 그 1건은 인용문이다 — `schema/graph.toml:101` *"이 결정은 `symbol` 반경에서 live"* 는 `crates/pal-cli/src/touch.rs:357-358` · `crates/pal-cli/src/query.rs:373-374` 의 주석, 그리고 **`corpus/criteria.toml:6950-6951` 과 `:8373`** 이 같은 문장으로 나른다. `G1` 이 `corpus/**` 를 **얼렸으므로**, `B1` 이 코드 쪽 인용만 `fresh` 로 바꾸면 얼린 정본과 인용이 문장에서 갈린다. 이것은 `G1` 이 `corpus/**` 를 얼릴 때 든 근거(*"게이트가 인용하는 정본을 고치면 인용과 원문이 갈린다"*)의 **정확한 역방향**이고, 그 방향을 아무도 안 봤다.
- 어디가 걸리나: `intent.md:101`(`B1`) ↔ `intent.md:144`(`G1`) · `schema/graph.toml:100-102` · `crates/pal-cli/src/touch.rs:357-358` · `crates/pal-cli/src/query.rs:373-374` · `corpus/criteria.toml:6950,6951,8373`
- 획득: 조회 — `grep -c "live" schema/graph.toml` → 1 · 그 줄을 열어 인용문임을 확인 · `grep -n "live" corpus/criteria.toml` 로 같은 문장 셋을 찾았다
- 모집단: 저장소
- 유효성: 참
- 해악도: 거짓신호
- 대상: 계획대상
- 얼마나 아픈가: 되돌릴 수 있다. 걸리는 곳 다섯. 어느 검사도 이 갈림을 안 잰다 — `check_stale_citation` 은 **사라진 문서**를 보지 문장 대조를 안 한다

### `A2-a` 는 새 검사인데 「RED 관측」 목록에 없다 — 태어나면서 초록인 채로 서도 아무도 안 잰다

- 어떻게 실패하나: `intent.md:85-87` 이 *"RED 는 `C2-a`·`D3-a`·`G2-a` 가 거는 새 검사가 처음 돌 때 실제로 발화하는 것으로 관측한다. 발화하지 않으면 그 검사는 아무것도 안 재는 것"* 이라고 적었다. 개정 R1 이 새로 세운 `A2-a` 도 **새 검사를 거는 조건**인데 그 목록에 안 들어갔다. 그리고 `A2-a` 가 재려는 명제(두 축의 직렬화 토큰이 서로소)는 `A1` 이 `Current` 를 피하기로 한 뒤에는 **자명하게 참**이다 — `CodeFreshness` 는 `{fresh,stale,orphaned,undeterminable}`, `Lineage` 는 `{current,superseded}`. 음성 대조 요구가 없으므로 그 검사는 한 번도 안 빨개지고, 그래도 아무도 그것을 이상하게 안 본다.
- 어디가 걸리나: `intent.md:86`(RED 관측) ↔ `intent.md:95`(`A2-a`) · `crates/pal-core/src/binding.rs:509-510,565-568`
- 획득: 조회 — 의도 문면 대조 + `binding.rs` 의 두 `#[serde]` 특성을 직접 확인. `xtask/Cargo.toml:11-16` 이 `pal-core` 에 의존하므로 실행형 검사는 **구현 가능**하다(그래서 「못 만든다」가 아니라 「안 발화한다」가 위험이다)
- 모집단: 자기장치
- 유효성: 참
- 해악도: 금지역
- 대상: 계획자신
- 얼마나 아픈가: 되돌릴 수 있다 — 목록에 한 줄 더하면 된다. 안 더하면 검사 하나가 **측정이 죽은 가지**로 CI 에 영구히 남고, 그 뒤 `Lineage` 를 손대는 회차가 「검사가 있으니 안전하다」고 잘못 믿는다

### `D2` 의 모집단 1481 행이 `disposal-overrides.jsonl` **89 행 두 파일**을 안 센다 — 같은 `처분` 키를 지는데

- 어떻게 실패하나: `D2` 가 *"레코드는 **1481행**이다"* 로 모집단을 못 박았다. 그 수는 `findings.jsonl` 만 센 것이다. 같은 `"처분"` 키를 지고 **같은 파서 계약**(`record.py` 의 `종류` 축이 `["레코드","예외표"]` 둘을 선언한다)에 걸리는 파일이 둘 더 있다 — `.palimpsest/rounds/2026-08-19-finding-records/review/disposal-overrides.jsonl` 82 행 · `…/premortem/disposal-overrides.jsonl` 7 행. `D2` 가 「전량 이주」로 가면 89 행이 남고, `xtask` 의 「발견이 닫혔나」가 `*.jsonl` 산출 전부를 훑으므로 거기서 터진다. ⚠ 그리고 이 두 파일은 `d1-synthesis.md:92-93` 이 *"영어 원 표기는 이미 이 저장소에 있다"* 의 실물 증거로 든 바로 그 파일이다 — 근거로는 세고 모집단에서는 뺐다.
- 어디가 걸리나: `intent.md:118`(`D2`) · `.claude/skills/round/bin/record.py:99` · `xtask/src/main.rs:4514`(`check_finding_closure`) · `xtask/src/main.rs:4642-4644`
- 획득: 조회 — 파일별로 `grep -c '"처분"'` 를 돌렸다. `findings.jsonl` 만 = **1481** · 전체 `*.jsonl` = **1570** · 차이 **89** (두 파일)
- 모집단: 자기장치
- 유효성: 참
- 해악도: 실패
- 대상: 계획자신
- 얼마나 아픈가: 되돌릴 수 있다. 걸리는 곳은 파일 둘 + 조건 하나. 「전량 이주」를 안 고르면 안 터지지만, 그러면 **모집단을 잘못 세고도 안 터진 것**이라 다음에 같은 실수가 반복된다

### 개정 R1 이 조건 문면에 못 박은 수 둘(`1481` · `969`)은 회차가 진행되면 반드시 낡는다 — 그리고 자기 갱신 규칙이 없다

- 어떻게 실패하나: 착수 관측의 `1462` 가 이미 한 번 `1481` 로 정정됐다. 그 뒤에도 레코드는 계속 쌓인다 — 이 회차 자신의 `findings.jsonl` 이 지금 19 행이고 사전부검 R2·독립 리뷰 5 라운드가 남았다. `G2` 의 `969` 도 같다: 지금 HEAD 에서 정확히 재현되지만(`.claude` 77 + `assets` 4 = 81 · `xtask` 98 · 루트 `*.md` 13 · `schema`+`surface` 19 · `docs` 나머지 98 · `crates` 477−4 = 473 · `scripts` 174 · `.github` 13 = **969**), **`E` 와 `F` 가 `G` 보다 먼저 돌면서 순위 1(설치 페이로드)에 새 에이전트 정의 넷 이상을 더한다.** 지금 있는 에이전트 정의는 파일마다 패턴을 1~9 건씩 진다(`pal-independent-reviewer.md` 9 · `pal-premortem-sweeper.md` 6 · `pal-decision-synthesizer.md` 4). `G2` 가 「969곳에서 사라졌다」로 닫히면 그 문장은 종료 시점에 거짓이다.
- 어디가 걸리나: `intent.md:118`(`D2` 의 1481) · `intent.md:145`(`G2` 의 969) · `intent.md:230-231`(개정의 관측 정정) · `.claude/agents/`
- 획득: 조회 — `g1-synthesis.md` 의 정규식을 그대로 이 워킹트리에서 다시 돌려 뿌리별 수를 재현했고, `findings.jsonl` 행을 다시 셌다. **`E`·`F` 가 더할 발생 수는 추정이다**(파일이 아직 없다)
- 모집단: 자기장치
- 유효성: 참
- 해악도: 거짓신호
- 대상: 계획자신
- 얼마나 아픈가: 되돌릴 수 있다 — `B4`·`D4`·`G3` 에서 「검사 수를 여기 안 적는다」로 이미 같은 병을 고쳤는데, **같은 회차에서 다른 두 자리에 손으로 벤 거울을 새로 세웠다.** 걸리는 곳 둘

### 설치 페이로드의 `.py` 둘이 「처분」·「막힘」을 **실행 출력으로** 찍는데, 그 자리를 고치는 조건이 없다

- 어떻게 실패하나: `D2` 는 `findings.jsonl` **필드**만 다루고 좌표를 셋으로 못 박았다(`record.py:99` · `status.rs:466,469` · `xtask:4642-4644`). 그런데 같은 낱말이 **사람이 읽는 출력**으로 나가는 자리가 그 셋 밖에 있다 — `.claude/skills/round/bin/dashboard.py:303-305` 가 `print(f"⑨ 해악 게이트    **막힘** — …")` 를, `record.py:787` 이 축 이름 「처분」을 표에 찍고, `record.py:633` 이 ``필수 필드 `처분` 가 없다`` 를 낸다. 둘 다 `crates/pal-cli/src/install/layout.rs:94-95,103-104` 로 남의 프로젝트에 설치된다. `G2` 의 일곱 패턴에 「처분」·「막힘」은 없고, `C1` 은 CLI 상태값만 본다. **일곱을 개명하기로 하고도 사용자에게 실제로 나가는 자리는 아무 조건도 안 받는다.**
- 어디가 걸리나: `intent.md:118`(`D2`) · `intent.md:145`(`G2`) · `.claude/skills/round/bin/dashboard.py:303-305` · `.claude/skills/round/bin/record.py:633,787` · `crates/pal-cli/src/install/layout.rs:94-95,103-104,168-169`
- 획득: 조회 — 세 좌표를 직접 열었고 `PAYLOAD` 에 `.py` 둘이 실린 것을 `layout.rs` 에서 확인했다
- 모집단: 저장소
- 유효성: 참
- 해악도: 거짓신호
- 대상: 계획대상
- 얼마나 아픈가: 되돌릴 것이 없다(안 고치므로). 걸리는 곳 셋. `docs/overview.md:991` 이 표면을 *"사용자·에이전트가 닿는 자리"* 로 정의했으므로 이 셋은 정의상 사용자 표면이고, `d1-synthesis.md:75-85` 가 그것을 이미 반증으로 적었는데 조건이 안 받았다

### `C2-a` 의 「enum·토큰 필드 값에 한국어 금지」는 **이미 있는 위반 둘**에 먼저 걸린다 — 그리고 그 둘은 「범위 밖」이다

- 어떻게 실패하나: `C2-a` 가 enum·토큰 필드 값에 한국어가 섞이면 실패하는 검사를 걸라고 한다. 그런데 저장소에 이미 있다 — `crates/pal-core/src/ledger.rs:101` `IdentityGrade::Unavailable => "없음"` 과 `:140` `ExclusionRule::NulByte => "NUL 바이트"`. 앞엣것은 `crates/pal-cli/src/export.rs:253` 을 통해 **Cypher 속성 값**으로 나간다(`identity: "없음"`). `IdentityGrade` 도 `ExclusionRule` 도 `C1` 의 넷에 없고, `intent.md:168` 「범위 밖」이 *"그 밖의 타입 이름은 안 건드린다"* 로 밀어냈다. 그러면 `C2-a` 는 착지하자마자 병기와 무관한 사유로 빨개지고, 조건 셋 중 하나를 골라야 한다 — 범위를 넓히거나(잠긴 문장 위반), 예외표를 두거나(검사를 속 비게 만듦), 검사를 `CodeFreshness` 만으로 좁히거나(그러면 `C2-a` 가 재는 것이 `C2` 와 같아진다).
- 어디가 걸리나: `intent.md:112`(`C2-a`) ↔ `intent.md:168`(범위 밖) · `crates/pal-core/src/ledger.rs:99-105,138-141` · `crates/pal-cli/src/export.rs:249-253`
- 획득: 조회 — `grep -rn "pub const fn name(self)" -A12 crates/pal-core/src | grep -P '[가-힣]'` 로 한국어를 내는 `name()` 넷을 전수로 찾았다(`ledger.rs:101,140` · `touch.rs:152,153`)
- 모집단: 저장소
- 유효성: 참
- 해악도: 실패
- 대상: 계획대상
- 얼마나 아픈가: 되돌릴 수 있다. 걸리는 곳 넷. `C2-a` 는 음성 대조를 요구하므로 **반드시 한 번은 돌려야 하고**, 그때 이 셋 중 하나를 고르는 판단이 조건 없이 즉흥으로 내려진다

### `F` 에는 `E3` 의 짝이 없다 — 「무엇이 그것을 강제하나」를 `E` 에만 물었다

- 어떻게 실패하나: `E3` 이 *"메인의 관여가 발동과 수신뿐임이 스킬에 명시되고, **무엇이 그것을 강제하는지**가 함께 적힌다 (문서만으로 안 지켜진 전례가 이 저장소에 있다)"* 를 요구한다. `F1`~`F5` 는 완수 조건 설계 평가 단계를 **스킬 절차에 세우는 것**이 전부이고 강제 수단을 안 묻는다. 같은 전례가 `F` 에도 그대로 걸린다 — 스킬에 절차를 한 줄 더하는 것으로 `F1`·`F2`·`F3`·`F4` 가 전부 닫히고, 다음 회차가 그 단계를 건너뛰어도 아무것도 안 빨개진다. `F5` 의 「효과」는 **한 번 돌려 본 출력**이라 반복을 보장하지 않는다.
- 어디가 걸리나: `intent.md:128`(`E3`) ↔ `intent.md:136-140`(`F1`~`F5`) · `xtask/src/main.rs:621-644`(검사 23 개 중 스킬 절차 준수를 재는 것 0)
- 획득: 조회 — 의도 `E`·`F` 절 대조 + `xtask` 의 검사 목록 전수. 회차 산출을 읽는 검사 넷(`check_round_records` · `check_ledger_pair` · `check_finding_closure` · `check_declared_lists`) 중 「어느 단계를 밟았나」를 재는 것은 없다
- 모집단: 자기장치
- 유효성: 참
- 해악도: 거짓신호
- 대상: 계획자신
- 얼마나 아픈가: 되돌릴 수 있다. 걸리는 곳은 조건 다섯. `E`·`F` 는 「모집단 분리 (나)」가 *"장치를 만드는 것 자체가 산출물"* 이라고 인정한 자리이므로, 강제 수단 없는 장치는 **다음 회차의 먹이**가 된다

### `E3-a` 가 `SKILL.md` 의 자기모순 좌표를 하나만 짚었다 — 같은 선언이 두 곳에 있다

- 어떻게 실패하나: `E3-a` 가 *"`SKILL.md` §5 가 「정(正) — 메인이 먼저 낸다」로 소유자 지시를 인용하고 있다"* 로 한 자리를 짚는다. 실제로는 둘이다 — `.claude/skills/round/SKILL.md:312`(§5 루프 안) 과 `:852`(§9 아래 에이전트 표의 각주 *"★ **정(正)은 메인이다** — 판정 초안을 쓰는 자리"*). 852 행은 **에이전트 표 바로 아래**라 `E1` 이 새 역할을 그 표에 더하면 같은 화면에서 자기를 부정한다. 312 만 고치면 스킬이 여전히 모순이고, 그 파일은 `PAYLOAD` 로 남의 프로젝트에 나간다.
- 어디가 걸리나: `intent.md:129`(`E3-a`) · `.claude/skills/round/SKILL.md:312,323,852` · `crates/pal-cli/src/install/layout.rs:67-70,167`
- 획득: 조회 — `grep -n "정(正)" .claude/skills/round/SKILL.md` → 312·323·852 셋. 852 앞뒤(845-860)를 열어 에이전트 표의 각주임을 확인했다
- 모집단: 저장소
- 유효성: 참
- 해악도: 거짓신호
- 대상: 계획대상
- 얼마나 아픈가: 되돌릴 수 있다. 두 곳. 다만 852 행은 `E1` 이 반드시 만지는 표 바로 옆이라 **놓치면 눈에 띈다** — 해악은 크지 않다

### `G4` 가 넘기는 곳(`cut-the-bullshit`)은 **이 환경에서 이미 켜져 돌고 있는 플러그인**이다 — 넘김이 결함을 안 고친다는 관측이 이미 있다

- 어떻게 실패하나: `G4` 가 문장 등록 결함의 현상·설계를 문서로 내고 *"고치는 것은 `cut-the-bullshit` 이 맡는다"* 로 닫는다. 그런데 `.gitignore:36-38` 이 *"Claude Code 플러그인 활성화 선언. `enabledPlugins` 는 `cut-bs@cut-the-bullshit` … `~/.claude/settings.json` 에서 절대 경로로 풀린다"* 라고 적는다 — **소유자가 걸려 넘어진 문장들은 그 플러그인이 켜진 채로 생산됐다.** `G4` 는 그 플러그인이 무엇을 못 잡았는지를 재는 조건이 없고, 문서를 내는 것으로 `H3` 이 #114 를 닫는다. 회차 종료 시점에 소유자가 가장 세게 지적한 결함(*"이 문장을 보고 이해할 수 있는 한국인은 없을거라고 단언"*)은 **한 문장도 안 고쳐진 채 남고**, 종료 보고는 이슈 일곱이 다 닫혔다고 적는다.
- 어디가 걸리나: `intent.md:148`(`G4`) · `intent.md:170`(범위 밖) · `intent.md:154`(`H3`) · `.gitignore:36-38` · `owner-source-9eec834.txt:33`
- 획득: 조회 — `grep -rn "cut-the-bullshit"` 로 저장소 안의 유일한 실물 언급이 `.gitignore` 임을 확인했고, `~/dev/projects/cut-the-bullshit` 이 실재하는 것을 `ls` 로 확인했다. **그 플러그인이 실제로 무엇을 잡는지는 안 봤다**
- 모집단: 원의도
- 유효성: 참
- 해악도: 거짓신호
- 대상: 계획대상
- 얼마나 아픈가: 되돌릴 것이 없다 — 소유자가 그 넘김을 직접 골랐다(승격 4번). 아픈 것은 **`H3` 이 「이슈 일곱 전부 닫힘」을 완수로 세는 것**이다. `G4` 만 성질이 다른데 같은 칸에 들어가 있고, 종료 보고에 그 차이를 적으라는 조건이 없다

### `G2` 가 「대상 969곳에서 사라진다」인데 `G1` 이 얼린 곳에 소유자가 실제로 걸린 문장이 남는다 — `corpus/**` 316곳

- 어떻게 실패하나: `g1-synthesis.md:54` 의 실측이 소유자가 든 문장 「상태는 … 이슈에 산다」의 실재 좌표로 `corpus/criteria.toml:5725,9664` 를, 「그것을 센다」로 `corpus/criteria.toml:5820,6060` 을 들었다. 그리고 같은 판정문이 `corpus/**` 를 **동결**로 옮겼다(개정 R1 이 `G1` 에 반영). 지금 이 워킹트리에서 재면 `corpus` 의 패턴 발생은 **320곳**이다. 그러면 `G2` 가 초록이 된 뒤에도 **소유자가 이름을 들어 지목한 바로 그 문장이 저장소에 남아 있고**, `G2` 문면은 그것을 「대상 밖」이라 적을 뿐 「남는다」고 적지 않는다. 소유자가 다시 열면 회차가 그 문장을 못 세는 상태다.
- 어디가 걸리나: `intent.md:144`(`G1`) · `intent.md:145`(`G2`) · `corpus/criteria.toml:5725,5820,6060,9664` · `g1-synthesis.md:54-55`
- 획득: 조회 — `g1-synthesis.md` 의 정규식으로 `corpus` 를 다시 재서 **320곳**을 얻었다(판정문의 316 과 4 차이 — 이 회차가 그 뒤 아무것도 안 더했으므로 세는 include 목록의 차이로 보이나 **원인은 확인 못 했다**)
- 모집단: 회차기록
- 유효성: 추정
- 해악도: 거짓신호
- 대상: 계획대상
- 얼마나 아픈가: 되돌릴 수 있다 — `G2` 문면에 「`G2` 가 안 재는 것」을 한 줄 더하면 닫힌다(`G2` 는 이미 *"문장 등록 결함은 `G2` 가 안 잰다"* 를 그렇게 적었다. 같은 형태를 동결분에도 적으면 된다). 안 적으면 `G2` 의 초록이 소유자 불만 해소로 오독된다

---

## 내가 기각한 것

### `H2` 는 이 브랜치에서 도달 불가능하다 — CI 가 `push: branches: [main]` 만 받는다

- 어떻게 실패하나: (기각) `.github/workflows/ci.yml:46-48` 이 `on: push: branches: [main]` 과 `pull_request` 뿐이라, `docs/overview-user-feedback` 에 push 해도 런이 안 붙고 `H2`(마지막 커밋 SHA 에 `conclusion=success`)가 못 닫힌다고 적으려 했다.
- 기각 사유: **돌려서 반증됐다.** `gh pr list --head docs/overview-user-feedback` → PR **#115 가 OPEN**. 그리고 `gh run list` 실측에서 이 브랜치의 `pull_request` 런이 `headSha=b1654455…`(착수 커밋 `b165445`)로 **브랜치 머리 SHA 에 붙어** `conclusion=success` 다. 지난 회차들도 전부 같은 형태(`round/104-…` · `round/94-…`)로 닫혔다. `H2` 는 도달 가능하다.
- 어디가 걸리나: `.github/workflows/ci.yml:46-54` · `intent.md:153`(`H2`) · PR #115
- 획득: 조회 — `gh pr list --state all --head docs/overview-user-feedback` · `gh run list --limit 8 --json headBranch,headSha,conclusion,event`
- 모집단: 규약
- 유효성: 거짓
- 해악도: 미관
- 대상: 계획대상
- 얼마나 아픈가: 없음 — 적기 전에 돌려서 걷었다

### `B2-a` 의 양성 대조는 심을 자리가 없어 원리상 못 돈다

- 어떻게 실패하나: (기각) 착수 관측이 *"`CodeFreshness` 영속 저장 0건 — `pal-store`·`pal-intent` 참조 없음"* 이라 적었고 나도 `grep -rn "CodeFreshness" crates/pal-store crates/pal-intent` → 0건을 확인했다. **읽기 경로가 아예 없으므로** *"옛 토큰이 든 값을 읽기 경로에 직접 먹여 serde 가 거부하는 것을 관측한다"* 가 불가능하다고 적으려 했다.
- 기각 사유: **읽고 확인해 보니 아니었다.** `crates/pal-core/src/binding.rs:509` 가 `Deserialize` 를 파생하고 있어 `serde_json::from_str::<CodeFreshness>(r#"{"freshness":"live"}"#)` 가 **그 자체로 읽기 경로**다. `xtask/Cargo.toml:11-16` 이 `pal-core` 에 의존하므로 `xtask` 안에서도 돌릴 수 있다. 「제품이 안 쓰는 경로를 잰다」는 약점은 남지만 **조건이 못 도는 것은 아니다.** 조건을 무효로 만들 정도가 아니다.
- 어디가 걸리나: `intent.md:103`(`B2-a`) · `crates/pal-core/src/binding.rs:509-510` · `xtask/Cargo.toml:11-16`
- 획득: 조회 — `grep -rn "CodeFreshness\|BindingStatus" crates/pal-store crates/pal-intent` → 0건 · `binding.rs` 의 derive 목록 확인
- 모집단: 자기장치
- 유효성: 거짓
- 해악도: 미관
- 대상: 계획자신
- 얼마나 아픈가: 없음

### `G4` 의 「교정한 낱말 목록」은 palimpsest 도메인 용어를 빼고 나면 빈다

- 어떻게 실패하나: (기각) `G4` 가 *"문서에는 **교정한 낱말 목록**이 들어가고 **palimpsest 도메인 용어는 뺀다**"* 라고 적었는데, 이 회차가 실제로 교정하는 낱말(`Live→Fresh` · 처분 · 퇴로 · 접힘 · 오라클 · 막힘 · 절단 · 봉투)이 **전부 palimpsest 도메인 용어**라서 목록이 공집합이 되고, 조건이 빈 문서로 충족된다고 적으려 했다.
- 기각 사유: **원문을 다시 읽고 걷었다.** `owner-source-9eec834.txt:25-36` 이 든 교정 대상은 「~에 산다」·「~ 선다」·「~를 낸다」·「~ 센다」·「접다」이고, 소유자 자신이 대안까지 줬다(*"'존재한다', '~에서 의미를 가진다'"* · *"'하네스와 별개로 동작한다'"* · *"제출/제시한다"* · *"측정한다/감지한다/확인한다"*). 이것들은 도메인 용어가 아니라 **일반 한국어 용법**이고, `G2` 가 969곳에서 그것을 고치므로 「교정한 낱말」은 최소 다섯이다. 목록은 안 빈다.
- 어디가 걸리나: `intent.md:148`(`G4`) · `owner-source-9eec834.txt:25-36`
- 획득: 조회 — 소유자 원문 25-36 행 전수
- 모집단: 원의도
- 유효성: 거짓
- 해악도: 미관
- 대상: 계획대상
- 얼마나 아픈가: 없음

---

새 범주: **왕복 파서를 진 표시 함수** — `.name()` 이 화면 문자열이면서 동시에 `parse` 의 키인 자리. 「사람 화면 ↔ 기계 출력」의 두 갈래(`C1` ↔ `C2`)로는 안 잡히는 셋째 갈래다.
