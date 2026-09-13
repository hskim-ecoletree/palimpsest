# 사전부검 R1 — 원 반환문

> 회차 `2026-09-13-first-release-elsewhere` · 라운드 1 (상한 2) · 읽은 절: 원문 · 목적 기여 · 착수 시점 관측 · 계획 · 금지역 · 범위 밖
> 저장소 HEAD `6ee9eb3` · 바이너리 `target/release/pal` · ditto 복제본 `aded7ce`(읽기만) · 격리 실측 사본 `scratchpad/pm-r1-1789296536/ditto`

## 시나리오

### ① 효과 장면의 조건이 착수 전 실험으로 이미 초록이다 — ㈄ 가 빨개질 수 없다
- 어떻게 실패하나: 잠그기 전 실험이 복제본 `.palimpsest/intent.redb`(19:31, 기준선 19:29 뒤)에 `ADR-0003`「결정」→ `mcpServersFromToml`(`src/core/hosts/codex.ts:25`)의 `asserted` 결박 `[bc2739f6362c6080]` 을 남겼다. 그리고 ㈄ 의 과제는 **바로 그 파일**(`codex.ts` 제거)이다. ㈄ 가 이 복제본에서 `touch` 를 부르면 `■ 이 좌표에 걸린 것 ≥ 1` 은 ㈁ 를 거치지 않고도 선다. ㈁ 의 장면 조건(「사용자가 **스스로** 닿는다」)은 이 복제본에서 원리상 빨개질 수 없다. 게다가 「계획을 먼저 봉인」도 오염됐다. 과제를 고른 쪽이 `adr-cands.txt`·`exp-touch-after.txt` 로 ADR-0003 이 `codex.ts` 에 걸린다는 것을 이미 알고 있었기 때문이다. 효과 판정(「걸음을 없앴나 · 덜 말했나」)이 앞선 앎에 기운다.
- 어디가 걸리나: 복제본 `.palimpsest/intent.redb` · scratchpad `exp-approve.txt`(`승인했습니다 … 결박 [bc2739f6362c6080]`) · `adr-cands.txt` 1행(ADR-0003 후보 셋이 전부 `hosts/codex.ts`·`claude-code.ts`) · 계획 ㈁·㈄
- 획득: 조회 — `exp-approve.txt`·`exp-touch-after.txt` 읽음, 복제본 `.palimpsest/` 의 mtime 과 `baseline/` mtime 대조, `adr-cands.txt` 파싱
- 모집단: 회차기록
- 유효성: 참
- 해악도: 금지역 — 측정이_죽은_가지 (`intent.md` `## 금지역` 이 규약 §11 기본 다섯으로 등록)
- 대상: 계획자신 (㈄ 의 측정 절차)
- 얼마나 아픈가: 되돌릴 수 있다 — 결박 없는 새 복제본에서 ㈄ 를 돌리면 된다. 걸리는 곳은 ㈁ 장면 조건과 ㈄ 효과 판정 둘이다. 모르고 가면 효과 기록이 한 번 오염돼 원장에 남는다.
- 드러내는 것: ㈄ 착수 전에 `pal intent export`(또는 `intent.redb` 존재 여부)를 보고 **결박 0 인 사본**인지 확인하는 것.

### ② 설치 직후의 사용자는 ㈁ 에서 빈 목록을 본다 — 복제본은 그 길을 안 지난다
- 어떻게 실패하나: 읽기 표면의 인입은 `민팅::안한다` 이고, 개체가 없는 조각은 **목록에서 빼고 수만 센다**(`#129`). `touch` 도 의도 저장소를 읽기로 연다. 그러니 ㈁ 를 같은 인입으로 세우면, `pal install` 뒤 `pal narrative` 를 한 번도 안 돌린 사용자의 `touch` 에는 승인 대기 후보가 0 건으로 나온다. 복제본은 이미 `pal narrative` 를 돌려 개체가 민팅돼 있다(`query narrative.unbound` 가 「이름이 아직 없어 뺀 조각 **0**」). 그래서 ㈄ 도 회차 안의 어떤 실측도 이 길을 안 지난다.
- 어디가 걸리나: `crates/pal-cli/src/narrative.rs:238-246`(`민팅::안한다` → `개체_없음 += 1; continue`) · `crates/pal-cli/src/query.rs:147-157` · `crates/pal-cli/src/touch.rs:137-140`(`IntentStore::open_read_only`) · `touch.rs:153`(`narrative_unminted: 0`)
- 획득: 조회 — 코드 읽기, 그리고 격리 사본에서 `pal query narrative.unbound` 실행(뺀 조각 0 확인). 새 저장소에서 `narrative` 없이 부르는 실측은 **안 돌렸다**.
- 모집단: 저장소
- 유효성: 추정 — 기제는 코드로 확인했고, ㈁ 가 이 인입을 재사용할지는 아직 안 정해졌다
- 해악도: 거짓신호 — 「걸린 것 0 · 대기 0」이 「이 좌표엔 결정이 없다」로 읽힌다
- 대상: 계획대상
- 얼마나 아픈가: 되돌릴 수 있다. 첫 사용 장면 하나가 걸리는데, 그것이 원문이 말하는 「남이 설치해서 쓰는 순간」이다.
- 드러내는 것: `narrative` 를 안 돌린 새 복제본에서 `touch` 를 부르는 실측 한 번. 또는 ㈁ 화면이 `개체_없음` 수와 `pal narrative` 안내를 싣는지 보는 것.

### ③ 모듈 최상위의 참조는 「호출자」에 안 든다 — `codex.ts` 제거의 주된 파손 자리가 빠진다
- 어떻게 실패하나: 임포트 참조가 **어느 심볼에도 안 담기면** `PendingImportRef` 를 안 만든다. 그러면 ㈀ 이 지정자를 아무리 잘 펴도 엣지가 안 선다. ditto `src/core/hosts/index.ts` 에는 심볼이 0 개이고(`syms.jsonl` 실측), `codexHostAdapter` 의 등록이 그 파일 최상위(`:5` `registerHostAdapter(codexHostAdapter)`)에 있다. 나머지 소비자는 `getHostAdapter('codex')` 같은 문자열 레지스트리 조회다(`run-with.ts`·`doctor.ts`). 그래서 ㈀ 뒤의 `touch codexHostAdapter` 는 함수 안 참조 둘(`src/cli/commands/setup.ts:261` · `src/core/setup.ts:295`)만 셀 것이다. 그런데 화면의 단서 문장은 `x.foo()` 만 「안 센다」고 적는다. 사람은 그 수를 「이 파일을 지우면 깨지는 곳 전부」로 읽는다.
- 어디가 걸리나: `crates/pal-core/src/projection.rs:362-372`(`innermost` 가 `None` 이면 짝을 안 만든다) · `crates/pal-cli/src/touch.rs:468-491`(단서 문장) · ditto `src/core/hosts/index.ts:5`
- 획득: 조회 — 코드 읽기, ditto `git grep codexHostAdapter`, `syms.jsonl` 에서 `hosts/index.ts` 심볼 0 확인, 격리 사본 `touch codexHostAdapter`(지금 호출자 0). ㈀ 뒤의 수 「2」는 **추정**이다(빌드 전).
- 모집단: 저장소
- 유효성: 참 — 최상위 참조가 짝을 못 만드는 기제는 코드와 심볼 0 으로 확정. 수만 추정이다.
- 해악도: 거짓신호
- 대상: 계획대상
- 얼마나 아픈가: 되돌릴 수 있다. TS 는 최상위 등록·배선 코드가 흔해서(추정) 모든 TS 좌표의 「호출자」 하한성이 걸린다. ㈄ 의 효과 판정이 이것을 「pal 이 덜 말했다」로 적을지 「pal 이 다 말했다」로 오독할지가 갈린다.
- 드러내는 것: ㈄ 에서 실제 제거 뒤 `bun tsc`/테스트가 빨개지는 자리와 `touch` 의 호출자 목록을 대조하는 것. 또는 `RefCounts::imported` 와 `pending` 길이의 차이를 TS 에서 한 번 재는 것.

### ④ 재수출 배럴은 해소기의 파일 목록에 없다 — 까닭이 사실이 아닌 문으로 간다
- 어떻게 실패하나: `cross_file_edges` 의 `paths` 는 **심볼이 있는 파일**에서만 만든다. 순수 배럴 `src/core/hosts/index.ts`·`src/schemas/index.ts`·`rebuild/schemas/index.ts`·`src/acg/change-map/index.ts` 는 심볼이 0 이다. 그래서 ㈀ 이 `./hosts` → `hosts/index.ts` 를 올바르게 펴도 후보가 `paths` 에 안 맞아 `hit=[]` 가 된다. 그러면 `NoTargetFile`(화면: 「모듈 경로를 파일로 못 폈습니다」)이나 `OutsideRepo`(화면: 「저장소 밖이라 원리상 못 섭니다」)로 떨어지는데, 둘 다 사실이 아니다 — 파일은 저장소 안에 있고 펴졌다. 선언과 재수출이 섞인 배럴(`journey-authoring/index.ts` 선언 3·재수출 4 · `icl/index.ts`)로 재수출 이름을 들여오면 `NoSymbol` 이 되고, 화면은 그 몫을 `#133`(Rust L2)의 문으로 보낸다. 해당 문은 `from './hosts'` 7 · `from '~/core/hosts'` 2 · `from '../schemas'` 10 이다. `./hosts` 7 중 5 가 `HostAdapter` 를 들여온다(ADR-0016·ADR-0008 의 후보 심볼, ㈄ 와 같은 디렉터리). `## 범위 밖` 의 「재수출 추적 ⟨사전부검 뒤 확정⟩」이 추적을 빼는 것은 되지만, **빠진 몫을 어느 까닭으로 적는가**는 안 정한다.
- 어디가 걸리나: `crates/pal-core/src/cross_file.rs:501`(`paths` = 심볼의 경로) · `:614-628`(빈 hit → `우리것` 으로 갈림) · `:543-549`(`심볼_없음` 이 크레이트 뿌리만 안다) · `crates/pal-cli/src/touch.rs:563-569`(이관표 문 문구)
- 획득: 조회 — 코드 읽기, `syms.jsonl` 에서 배럴 심볼 수, ditto `git grep` 으로 배럴 임포트 수
- 모집단: 저장소
- 유효성: 참 — `paths` 에 없어 빈 hit 이 되는 것은 확정이다. 어느 까닭으로 갈지는 ㈀ 의 `우리것` 구현에 달렸다.
- 해악도: 금지역 — 사실이_아닌_것을_사실로 (`intent.md` `## 금지역` 이 이 회차에 특히 걸린다고 지목)
- 대상: 계획대상
- 얼마나 아픈가: 되돌릴 수 있다. ditto 에서 약 20 문장, 그리고 `HostAdapter` 처럼 ADR 이 걸린 타입의 호출자 수가 걸린다.
- 드러내는 것: ㈀ 뒤 `touch HostAdapter` 의 「내가 모르는 것」에서 `./hosts` 발 항목이 무슨 까닭으로 찍히는지 보는 것.

### ⑤ ㈃ 가 회차 코드를 지우면 이관표·「범위 밖」 문장이 지키던 구별이 사라진다
- 어떻게 실패하나: 사람 화면의 회차 내부 어휘 상당수는 장식이 아니다. **구별 자체를 지는 문장**이다. 「못 선 몫이 가는 문」의 다섯 줄은 문이 곧 `A5`·`A5-a`·`잠근 축1`·`#133` 이고, `x.foo()` 줄의 「이 회차의 범위 밖」은 앞 회차가 「능력 부재와 범위 밖을 가른다」로 정해 넣은 문장이다(`touch.rs:506-512` 주석). 코드를 지우고 뜻을 안 옮기면 두 갈래로 끝난다. 시험 `cross_file_references.rs:291-294` 의 `contains("범위 밖")` 이 빨개지거나(실패), 그 단언을 같이 고쳐 초록이 되면서 구별이 조용히 사라진다(거짓신호). 게다가 문 문구(`std::*`·크레이트 뿌리·재수출 경유)는 Rust 의 뜻이다. 코드만 빼면 TS 화면에 Rust 설명이 남는다(④ 와 겹친다).
- 어디가 걸리나: `crates/pal-cli/src/touch.rs:487-491` · `:560-571`(`print_transfer`) · `crates/pal-cli/tests/cross_file_references.rs:291-294`
- 획득: 조회 — 코드와 시험 단언 읽기
- 모집단: 저장소
- 유효성: 참
- 해악도: 거짓신호
- 대상: 계획대상
- 얼마나 아픈가: 되돌릴 수 있다. 화면 두 구역과 시험 하나. 알아채기 어려운 쪽은 시험을 함께 고치는 경로다.
- 드러내는 것: ㈃ 의 diff 에서 `tests/` 단언이 **삭제**됐는지 **이동**됐는지 보는 리뷰.

### ⑥ 「어휘가 없다」 검사가 도움말·거부·오류 경로를 안 지난다
- 어떻게 실패하나: `intent.md` 의 grep 목록(「전수가 아니다」)에 없는 사람 화면에도 기능 코드가 나온다. `pal narrative --help` 첫 줄 「(F10)」, `pal touch --help` 의 `--binding-max` 「(옛 F11 §3.3)」, `pal ledger --help` 「옛 F03 §6.3」, 승인 거부 `narrative.rs:415`·`:525` 「(옛 F10 §3.3)」. 완수 조건이 장면 네 명령의 **정상 경로 표준출력**만 훑으면 초록이 선다. 그런데 사용자가 가장 먼저 보는 `--help` 와 `--approve` 거부 화면에는 코드가 남는다. 거꾸로 `grep` 을 소스에 대면 주석까지 잡혀 빨갛기만 하다.
- 어디가 걸리나: `crates/pal-cli/src/narrative.rs:415` · `:525` · clap 문서 주석(도움말) · 완수 조건(아직 없음)
- 획득: 조회 — `pal install|ledger|narrative|touch --help` 를 실행해 grep, `narrative.rs` 에서 `옛 F10` grep
- 모집단: 자기장치
- 유효성: 참 — 누수는 실측했다. 검사가 그것을 놓칠지는 조건이 안 섰으니 가능성이다.
- 해악도: 금지역 — 측정이_죽은_가지 (`intent.md` `## 금지역` 이 「어휘가 없다」 검사를 지목)
- 대상: 계획자신
- 얼마나 아픈가: 되돌릴 수 있다. 확인한 자리는 다섯이고 전수는 안 쟀다.
- 드러내는 것: 조건 설계 평가에서 검사의 모집단이 `--help` 와 오류 경로(표준오류)를 포함하는지 묻는 것. 음성 대조로 `--help` 에 코드 하나를 심어 빨개지는지 보는 것.

### ⑦ tsconfig 해석의 일반성은 ditto 로 관측되지 않는다 — 실패하면 기준선과 같은 `outside_repo` 로 조용히 떨어진다
- 어떻게 실패하나: ditto 의 tsconfig 는 주석 없는 순수 JSON 이고, `baseUrl "."` 에 별칭은 `~/*` 하나다. `extends` 를 쓰는 `rebuild/tsconfig.json`·`rebuild/seam/tsconfig.json` 아래 파일은 `~/` 를 **0 번** 쓴다. 그래서 JSONC(주석·꼬리 쉼표), `extends` 사슬, `baseUrl` 없는 `paths`, 파일마다 가장 가까운 tsconfig 고르기, BOM 은 하나도 안 지난다. 이 가운데 하나가 틀리면 별칭 임포트가 통째로 `OutsideRepo` 가 된다. 착수 관측과 **같은 증상**이고, 화면은 그것을 「원리상 못 섭니다」라는 경계로 적는다. 입력 배관도 새로 생긴다. 해소기의 입력은 심볼·임포트뿐이고(`CrossFileInput`) tsconfig 는 파싱 대상 언어가 아니다. 워킹트리에서 읽으면 `--at <rev>` 의 답과 스냅샷이 갈린다.
- 어디가 걸리나: `crates/pal-core/src/cross_file.rs:73-80`(`CrossFileInput`) · `crates/pal-store/src/projection.rs:393-409`(입력 조립) · ditto `tsconfig.json`·`rebuild/tsconfig.json`
- 획득: ditto 가 이 경우들을 안 지나간다는 것은 조회(tsconfig 세 벌 읽음, `git grep "from '~/" -- rebuild/` 0). 「실제 TS 저장소에서 JSONC·extends 가 흔하다」는 **추정**이다. 다른 TS 저장소를 안 쟀다.
- 모집단: 저장소
- 유효성: 추정
- 해악도: 거짓신호
- 대상: 계획대상
- 얼마나 아픈가: 되돌릴 수 있다. 첫 릴리스 뒤 두 번째 남의 저장소에서 목표 2 가 다시 0 이 될 수 있다.
- 드러내는 것: 주석·`extends`·`baseUrl` 없는 `paths` 를 심은 시험 픽스처 셋. 이 저장소 CI 에는 TS 임포트가 있는 파일이 없다(`corpus/tasks/f03-normalize-seeds.ts` 는 임포트 0).

### ⑧ 확장자·`index` 규칙을 한 층에 넣으면 모호로 잃고, 순서로 넣으면 틀린 파일을 잡을 수 있다 — ditto 에는 그 경우가 하나도 없다
- 어떻게 실패하나: 계획은 확장자 없는 지정자에 `.ts`·`.tsx`·`.d.ts`·`/index.*` 를 대고 `.js` → `.ts` 를 대응시킨다. 지금 해소기의 1 차 층은 **맞은 파일이 둘이면 모호**다(`cross_file.rs:614-639`). `x.ts` 와 `x.d.ts`, `x.ts` 와 `x/index.ts` 가 함께 있는 저장소에서 한 층에 넣으면 엣지를 잃는다(과소). TS 해석처럼 순서를 매겨 첫 것을 고르는 쪽은 `Ambiguous` 를 지키는 기존 설계와 부딪친다. 순서를 틀리면 틀린 파일로 간다(`intent.md` `## 금지역` 의 거짓 수). `allowImportingTsExtensions`(ditto 에서 켜짐)가 허용하는 `./x.ts` 명시 확장자는 계획 목록에 없다. ditto 실측으로는 확장자 붙은 지정자 0 · `.d.ts` 0 · `x.ts`/`x/index.ts` 충돌 0 이라 이 가운데 무엇도 장면에서 안 드러난다.
- 어디가 걸리나: `crates/pal-core/src/cross_file.rs:410-434`(층 구성) · `:614-639`(모호 판정) · 계획 ㈀
- 획득: ditto 에 그 경우가 없다는 것은 조회(`git grep` 으로 지정자 2,394 개 훑음, `git ls-files '*.d.ts'`, 충돌 루프). 다른 저장소에서의 결과는 **추정**이다.
- 모집단: 저장소
- 유효성: 추정
- 해악도: 금지역 — 사실이_아닌_것을_사실로 (순서가 틀린 갈래에 한해. 한 층 갈래는 거짓신호)
- 대상: 계획대상
- 얼마나 아픈가: 되돌릴 수 있다. 해소기 한 함수에 걸린다.
- 드러내는 것: `x.ts`+`x/index.ts`, `x.ts`+`x.d.ts` 를 함께 둔 픽스처에서 엣지가 무엇으로 서는지 보는 것.

### ⑨ 만들 필요가 없는 것 — `.js→.ts`·`.d.ts` 규칙은 장면에 관측 0 이다
- 어떻게 실패하나: ditto 저장소 안 임포트 1,694 개(상대 930 · 별칭 764)는 전부 확장자 없이 `.ts` 파일이나 `index.ts` 로 풀린다. 별칭은 파일 758 · index 6 · 없음 0 이다. `.js` 지정자와 `.d.ts` 파일은 0 이다. `/index.*` 도 순수 배럴은 심볼이 없어 ④ 때문에 어차피 엣지가 안 선다. 효과가 보이는 곳은 선언이 섞인 배럴 둘 뿐이다. 그러니 `.js→.ts`·`.d.ts` 규칙은 장면 어디서도 한 번도 안 지나고 CI 에도 입력이 없는 표면이다. 「TS 일반」이라는 주장이 시험 픽스처로만 선다. 원문의 「ditto 에만 맞추지 않는다」와 「장면을 막는 것만」이 여기서 부딪친다. 더 작은 표면(상대 경로 · `paths` 별칭 · 확장자 없음 → `.ts`/`.tsx`/`index.ts`)으로 장면의 답은 같다.
- 어디가 걸리나: 계획 ㈀ 의 확장자 규칙 목록
- 획득: 조회 — ditto 지정자 분포와 별칭 대상 해석을 격리 셸로 셈
- 모집단: 원의도
- 유효성: 참
- 해악도: 거짓신호 — 관측된 적 없는 지원을 지원한다고 적는다
- 대상: 계획대상
- 얼마나 아픈가: 되돌릴 수 있다. 추가 표면이 ⑧ 의 모호 위험을 키운다.
- 드러내는 것: ㈀ 의 각 규칙마다 「이 규칙으로 선 엣지 수」를 ditto 에서 한 번 세는 것.

### ⑩ ㈁ 가 흔한 지역 이름에 승인 명령을 붙인다
- 어떻게 실패하나: ditto ADR 조각 63 개의 좌표 후보 209 개 중 `path` 28 · `git` 12 · `src`·`open`·`block` 까지 다섯 자 이하 흔한 이름이 **44(21%)** 다. 두 파일 이상에 퍼진 조각이 63 중 34 다. 예를 들어 ADR-0007·ADR-0013 조각은 열네 파일의 지역 상수 `path` 를 후보로 든다. ㈁ 가 좌표마다 「승인 대기 · 승인 명령」을 한 줄씩 붙이면, 사용자가 `path` 류 좌표를 만질 때마다 무관할 가능성이 큰 ADR 승인을 권유받는다. 승인되면 `asserted` 결박이 되어 「걸린 것」에 섞인다.
- 어디가 걸리나: 계획 ㈁ · `crates/pal-query/src/lib.rs:1035`(`후보_퍼짐` — 「셋 이하」만 고를 수 있다고 이미 적는다) · scratchpad `adr-cands.txt`
- 획득: 후보 분포는 조회(`adr-cands.txt` 파싱). 그 후보들이 실제로 무관한지는 **추정**이다. ADR 본문과 대조하지 않았다.
- 모집단: 저장소
- 유효성: 추정
- 해악도: 거짓신호
- 대상: 계획대상
- 얼마나 아픈가: 결박은 거부·해제로 되돌릴 수 있지만 사람 승인이 섞이면 추적이 비싸다. `path` 류 좌표 수십 곳이 걸린다.
- 드러내는 것: ㈁ 화면이 후보 집합 크기(「이 조각의 후보 N 곳」)를 함께 싣는지, 그리고 `path` 좌표 하나로 화면을 떠 보는 것.

### ⑪ ㈂ 가 `touch` 만 고치면 `pal query` 는 같은 자리에서 막힌다
- 어떻게 실패하나: 「후보가 N건입니다. 하나를 고르지 않습니다」는 `touch.rs:275` 와 `query.rs:327` 두 곳에 있다. 계획 ㈂ 은 `touch` 만 이름으로 든다. 에이전트가 쓰는 표면(`AGENTS.md` 「grep 을 집으려는 순간 그래프에 먼저 묻는다」)은 동명 후보를 계속 못 고른다.
- 어디가 걸리나: `crates/pal-cli/src/query.rs:327` · `crates/pal-cli/src/touch.rs:275`
- 획득: 조회 — `grep "하나를 고르지 않습니다"`
- 모집단: 저장소
- 유효성: 참
- 해악도: 미관 — 장면은 `touch` 이고 원문도 그렇게 적었다
- 대상: 계획대상
- 얼마나 아픈가: 되돌릴 수 있다. 한 곳이다.
- 드러내는 것: `pal query symbol.resolve localDir` 를 ditto 에서 부르는 것.

## 내가 기각한 것

| 제목 | 어떻게 실패하나 | 어디가 걸리나 | 획득 | 모집단 | 유효성 | 해악도 | 대상 | 얼마나 아픈가 |
|---|---|---|---|---|---|---|---|---|
| 2층 인덱스가 옛 해소 회계를 재사용해 ㈀ 뒤에도 `0/11010` 이 찍힌다 | 해소기를 바꿔도 `ROW_FORMAT_REV` 가 그대로면 같은 스냅샷의 옛 투영을 읽을 것이라 봤다. **아니었다** — `touch` 는 매번 `How::Stitching` 으로 붙어 해소를 다시 돈다. 판 표시는 `ReadOnly` 문만 막는다 | `crates/pal-cli/src/touch.rs:127` · `crates/pal-cli/src/attach.rs:104-116` · `crates/pal-store/src/projection.rs:294-301` | 조회 — 코드 읽기 | 저장소 | 거짓 | 거짓신호 | 계획대상 | — |
| ㈁ 가 `touch` 에 인입을 붙여 26.7 초짜리 명령이 된다 | `pal narrative` 26.7 초를 인입 비용으로 봤다. **아니었다** — 격리 사본에서 읽기 인입(`pal query narrative.unbound`)은 **2.03 초**, `touch` 는 0.63 초였다(캐시 데워진 상태) | `crates/pal-cli/src/query.rs:150-157` · 격리 사본 `pm-r1-1789296536` | 조회 — 실행해 잼 | 저장소 | 거짓 | 미관 | 계획대상 | — |
| TS 기본값·네임스페이스 임포트가 ditto 에서 대량으로 새어 호출자가 준다 | `record_imported_items` 가 `named_imports` 만 담으므로 기본값·`* as` 가 빠진다고 봤다. 기제는 맞는데 ditto 저장소 안 지정자로는 **0 · 0** 이다 | `crates/pal-extract/src/typescript.rs:444-490` | 조회 — `git grep` | 저장소 | 거짓 | 거짓신호 | 계획대상 | — |
| 같은 파일의 동명 심볼(오버로드·타입/값 병합·중첩 프로퍼티)이 ⓐ 를 모호로 만든다 | `by_name` 이 `(파일, 이름)` 에 중첩 심볼까지 담으므로 부딪칠 것이라 봤다. ditto 심볼 4,578 에서 최상위 중복 0, 최상위 이름이 같은 파일 중첩과 겹치는 것 0 | `crates/pal-core/src/cross_file.rs:505-511` | 조회 — `syms.jsonl` 집계 | 저장소 | 거짓 | 거짓신호 | 계획대상 | — |
| ㈀ 이 이 저장소의 자기 기준선 수를 흔든다 | 이 저장소에도 TS 가 있어 파일 간 회계가 움직일 것이라 봤다. TS 파일은 `corpus/tasks/f03-normalize-seeds.ts` 하나이고 임포트가 0 이다 | `corpus/tasks/f03-normalize-seeds.ts` | 조회 — `git ls-files` · `grep` | 저장소 | 거짓 | 실패 | 계획대상 | — |
| ditto 의 `~/` 별칭 대상 파일이 여럿 없다 | 첫 셈에서 `MISS` 가 줄지어 나왔다. **내 셸의 틸드 전개 오류였다** — 다시 세니 파일 758 · `index.ts` 6 · 없음 0 | ditto `tsconfig.json` `paths` | 조회 — 스크립트를 고쳐 다시 셈 | 회차기록 | 거짓 | 거짓신호 | 계획대상 | — |
| 명시 확장자 `./x.ts` 지정자가 규칙에 없어 ditto 장면이 막힌다 | ditto 가 `allowImportingTsExtensions` 를 켜 두어 쓸 것이라 봤다. ditto 지정자 중 확장자 붙은 것 0 — 장면은 안 막는다(일반성 몫은 ⑧ 에 남겼다) | ditto `tsconfig.json` · 계획 ㈀ | 조회 — `git grep` | 저장소 | 거짓 | 실패 | 계획대상 | — |

새 범주: 측정 대상 사본의 선행 오염(잠그기 전 실험이 남긴 상태가 장면 조건을 미리 채움) · 한 언어의 의미로 쓰인 화면 문구를 다른 언어에 그대로 싣는 자리
