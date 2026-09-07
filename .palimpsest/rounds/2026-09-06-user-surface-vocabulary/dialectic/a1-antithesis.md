# 반(反) — `A1` 상태 어휘 판정 초안에 대한 반론

`pal-decision-opponent` 원 반환문. 2026-09-06.

## 초안이 서는 자리

- **괄호 안 소문자 판정의 근거가 실재한다.** 임시 저장소에 `pal bind` 후 `pal touch --json` 이 `{"code": {"freshness": "live"}, "lineage": "current"}` 를 낸다.
- **한국어 대응 둘이 doc 문장에 정확히 결박됐다.** `binding.rs:517` *"좌표가 사라졌다"* · `:524` *"판정할 수 없다"*.
- **`Live` 좌표 셋이 전부 실재한다.** `touch.rs:212`(정렬 우선순위 3) · `:235`(`낡았나`) · `xtask/src/main.rs:23-26`(금지 어휘 16 = 5+6+5, `live`·`fresh`·`current`·`valid`·`unchanged` 없음) · `:1935`(`"CodeFreshness::Stale"` 하한).
- **`Fresh` 는 소유자 어휘 안에 있다.** 소유자가 *"최신, **신선한**, 유효한"* 을 직접 열거했다.

## 반론

| # | 반론 | 좌표 | 유효성 | 해악도 |
|---|---|---|---|---|
| 1 | **초안이 소유자 원문을 잘못 셌다.** 「4개 중 stale·orphaned 을 제외하고는 직관적으로 와닿지 않음」은 산술상 `live` **와 `undeterminable` 둘**을 지목한다. *"특히 live 는"* 은 그중 하나를 강조한 것이지 나머지를 빼는 말이 아니다. 초안은 `Undeterminable` 유지 사유를 *"소유자가 언급하지 않았고"* 로 적었다 — 사실이 아니고, 그 문장이 ADR 로 발행된다 | `a1-thesis.md` 「소유자가 잠근 것」 첫 인용 · 이슈 #108 본문 같은 인용 | 참 | 금지역 |
| 2 | **A2 의 사실 주장이 실측과 어긋난다.** *"`Lineage` 는 사용자 화면에 상태값으로 나가지 않는다"* — `--json` 은 `"lineage": "current"` 를 낸다. 초안 자신이 A3 에서 `--json` 을 사용자가 눈으로 대조하는 표면으로 세웠다. 두 문장이 같이 못 선다 | 돌린 명령: `pal bind mirrorVerdicts …` → `pal touch mirrorVerdicts … --json` → `{"code":{"freshness":"live"},"lineage":"current"}` | 참 | 금지역 |
| 3 | **`Current` 기각의 근거 2 는 사람 화면에서 성립하지 않는다.** `Lineage::Current` 는 빈 문자열로 렌더된다(`String::new()`). `pal touch` 의 결박 줄은 lineage 를 아예 안 찍는다. 초안이 겁낸 `{code: Current, lineage: Current}` 화면은 **사람 화면에 발생할 수 없다** | `query.rs:369` · `touch.rs:345-363` | 참 | 거짓신호 |
| 4 | **`Live` 는 개명 뒤에도 사용자 화면에 남는다.** `NodeFreshness::Live` 가 별개 열거로 있고, `pal doctor` 가 불변식 8 문장 *"낡음 등급이 전파 규칙과 정합한다 — **live** 노드의 입력에 stale 이 없다"* 를 찍고, 위반 상세가 `name()` = `"live"` 를 찍는다. 초안은 이 열거를 한 번도 안 언급한다 | `cascade.rs:56-57,74` · `pal-core/src/doctor.rs:122,939` · `pal-cli/src/doctor.rs:375,384` | 참 | 거짓신호 |
| 5 | **`fresh`/`stale` 이 「굳은 반의어」인 도메인은 HTTP 캐시이고, 거기서 `fresh` 는 「TTL 안이라 재검증 없이 써도 된다」다** — 즉 「안 재봤지만 그렇다고 친다」다. 이 도구의 `Live` 는 감시 집합 전체 요약을 **실제로 다시 재서 같았다**이고, 「못 잰 경우」를 위해 `Undeterminable` 이 따로 있다. 초안이 근거 1 로 끌어온 「굳은 쌍」이 축의 뜻을 반대로 실어 온다 | `binding.rs:512`(정의) · `:524`(R16) · `:1046` | 참 | 거짓신호 |
| 6 | **`Valid` 기각이 엉뚱한 조항에 걸렸다.** `00-goals.md §3.2` 의 `clean` 은 **판정 3분할** `{Finding, Residual, OutOfScope}` 에 「위반 없음」 값이 없다는 뜻이고 신선도 축과 무관하다. 그 논증을 그대로 밀면 `Fresh` 도 같은 위반이다 | `docs/plan/00-goals.md:280-284` · `a1-thesis.md` 근거 3·4 | 참 | 거짓신호 |
| 7 | **후보표에 「유지」가 없다.** intent `A1` 과 이슈 #108 완료 조건이 둘 다 *"바꾸지 않기로 하는 것도 결정이다"* 를 명시했는데, 초안은 `Live` 를 한 줄도 안 재고 후보 셋만 기각한다. 소유자의 진단은 **한국어 독해**(on-air)이고 소유자 자신이 처방한 해법이 병기다 | `intent.md` `A1` 행 · `docs/overview.md:896` | 참 | 거짓신호 |
| 8 | **다섯째 후보를 안 봤다.** `evaluate` 가 실제로 하는 일은 **요약 대조**이므로 `Matched`·`Intact`·`InSync` 가 후보다. 특히 `Matched` 는 근거 3 의 정직성 기준과 안 부딪힌다 | 이슈 #108 후보 열 · `a1-thesis.md` 근거 넷 | 참 | 거짓신호 |
| 9 | **A3 의 「좁은 자리」 처방이 존재하지 않는 표를 겨냥한다.** `pal touch` 의 결박 줄은 고정폭 열이 아니다. 실제 고정폭 표는 `pal ledger` 이고 `C1` 이 거기 파일 상태 7종 병기를 요구한다 | `touch.rs:363` · `ledger.rs:609,622` | 참 | 거짓신호 |
| 10 | **그 표에서 「열 너비를 병기 후 길이로 잡는다」는 원리상 안 된다.** Rust 의 `{:<N}` 은 char 수로 채우고 한글은 2 열을 먹는다. `pal ledger` 실측 — 한국어가 든 언어표 행은 61 열, ASCII 행은 60·58 열로 **지금 이미 어긋나 있다.** `unicode-width` 의존이 저장소에 없다 | `pal ledger` 실행 · `ledger.rs:622` · `plan.rs:339` | 참 | 거짓신호 |
| 11 | **영어권 생략 이월의 근거가 실측과 어긋난다.** *"로케일 판별 장치가 저장소에 없다"* — CLI 는 이미 환경변수를 읽는다(`std::env::var("PATHEXT")`). `LANG`/`LC_ALL` 판별은 한 줄이고, intent 의 「범위 밖」이 뺀 것은 **언어 전환 장치**다 | `install/exe.rs:52` · `intent.md` 「범위 밖」 | 참 | 거짓신호 |
| 12 | **한국어 대응이 소유자 예시·기병합 문서와 갈렸다.** 소유자 예시와 `docs/overview.md:896` 이 둘 다 「최신 상태 아님(stale)」인데 초안은 「최신 아님(stale)」으로 줄였다 | `docs/overview.md:896` | 참 | 미관 |
| 13 | **개명의 파급이 덜 측정됐다.** 「실측한 제약」표와 intent `B1` 의 파일 목록 둘 다 `corpus/criteria.toml` 이 없다. 거기 `live` 가 **기계 판독 키**로 들어 있다 — `undeterminable_leaks_to_live = 0`(6781) · `undeterminable_reasons_never_live = true`(7125) · `pending → live` 전이 규정(10291-10292, 10634) | `corpus/criteria.toml:6781,7125,10291,10634` | 참 | 거짓신호 |
| 14 | **소유자가 읽은 화면은 상태가 넷이 아니다.** `docs/overview.md:106` 의 `pal touch` 예시가 다섯째 값 `pending` 을 찍는다 | `docs/overview.md:104-106` · `binding.rs:499-501` | 참 | 미관 |
| 15 | **(초안 밖 · 검증 중 실측) 정렬 검사가 죽어 있다.** `touch_recall.rs` 가 `i["status"]["code"]["state"]` 를 읽는데 실측 키는 `freshness` 뿐이다. `Null != "live"` 라 필터가 전부 통과하고 `assert_ne!(…, "live")` 도 항상 참이다. `cargo test -p pal-cli --test touch_recall 낡은_것은_상한에` 가 ok 로 지나간다 — 아무것도 안 재고 | `pal-cli/tests/touch_recall.rs:198,206` · `binding.rs:510` | 참 | 금지역 |
| 16 | **「화면 문자열 = 직렬화 문자열」 원칙에 이미 반례가 있다.** `NodeFreshness::name()` 은 `"stale-derived"`(하이픈)인데 serde `snake_case` 는 `stale_derived` 를 낸다 | `cascade.rs:55,74-77` | 참 | 미관 |

## 대립역이 스스로 물린 것

- **「`Fresh` 가 소유자 뜻 밖이다」** — 못 세운다. 소유자가 *"최신, 신선한, 유효한"* 을 직접 들었다.
- **「`Undeterminable` 은 영어에 없는 형태다」** — 못 세운다. 사전에 있는 낱말이다.
- **「이슈 #108 이 `surface/queries.toml` 에 `live` 가 있다고 적은 것이 틀렸다」** — `grep` 0건으로 실제로 틀렸지만 초안은 그 주장을 옮겨 적지 않았다.
- **「이슈 #108 이 *「소유자 결정이 필요하다」* 고 적었으니 에이전트가 정하면 안 된다」** — 못 세운다. `intent.md` 원문이 위임을 기록했다.
- **「`Live` 유지가 `check_no_regeneration` 을 깬다」** — 못 세운다. 그 검사의 하한은 `CodeFreshness::Stale` 이다.
