# 사전부검 R2 — 원 반환문

> 입력: `premortem/brief-r2.md` 하나. 격리 실측은 스크래치의 새 복제본(`pm-r2-<ts>`, ditto `aded7ce` 에서 `git clone --no-hardlinks`)에서
> `pal 0.0.0+6ee9eb30f2dc` 로 했고 끝나고 지웠다. 대상 복제본 `scratchpad/ditto` 와 원본 `~/dev/projects/ditto` 에는 쓰지 않았다.
> `pal` 그래프 질의로 못 묻는 것(ditto 의 지정자 모양·임포트 대상 심볼 모양)은 `git ls-files` + 정규식 + `pal symbols --json` 으로 쟀다.

## 시나리오

### 1. ㈄ 에서 자연스럽게 부를 `codexHostAdapter` 에 ADR-0003 이 안 걸린다 — 파일을 지우는 과제를 심볼 단위 질의로 잰다
- 어떻게 실패하나: ㈄ 의 과제는 **파일** `src/core/hosts/codex.ts` 를 지우는 일이다. 그런데 `touch` 는 **심볼 이름** 하나를 받고, 그 파일에는 심볼이 13 개 있다. 밖으로 내보내는 것은 `codexHostAdapter`·`buildCodexSpawnArgs` 둘이다. `pal narrative --json` 실측에서 ADR-0003 「결정」 조각의 후보(3 곳)는 **`mcpServersFromToml`** 이다. 이 함수는 내보내지 않는 내부 함수다. 반면 `codexHostAdapter` 에 걸린 후보는 `reports/design/dual-host-surface-adapter-plan.md` 의 진행 상태 절 하나(후보 7 곳)뿐이다. 그러면 결과는 둘 중 하나다. 사용자 걸음대로 `touch codexHostAdapter` 를 부르면 ADR-0003 이 화면에 안 나온다. `touch mcpServersFromToml` 을 부르면 그 선택 자체가 과제를 고른 쪽의 앞선 앎이다. 계획이 봉인 머리에 적겠다고 한 것은 「ADR-0003 이 codex.ts 에 걸린다」는 **파일 단위** 앎뿐이고, 그것이 **어느 심볼**인지는 안 적는다.
- 어디가 걸리나: `crates/pal-cli/src/touch.rs` `Args::name`(이름 하나) · ditto `src/core/hosts/codex.ts:25`(`mcpServersFromToml`)·`:102`(`codexHostAdapter`) · 계획 ㈄ 의 봉인 절차
- 획득: 조회 — 격리 복제본에서 `pal narrative --json` 의 `proposals` 중 codex.ts 심볼 id(`touch` 머리의 짧은 id)를 후보로 가진 것을 추렸다. `mcpServersFromToml`←`.ditto/knowledge/adr/ADR-0003-toml-parser.md#결정`(후보 3) · `codexHostAdapter`←설계 보고서 절 하나(후보 7) · `buildCodexSpawnArgs`←`host-adapter-contract.md#3-spawn-계약`(후보 7) · `scanCodexPluginRoot`←설계 보고서 절 하나(후보 1)
- 모집단: 원의도
- 유효성: 참
- 해악도: 거짓신호 — 효과 판정의 「덜 말했나/안 닿았나」가 어느 심볼을 불렀는지에 좌우되는데, 그 선택이 기록에 안 남는다
- 대상: 계획대상
- 얼마나 아픈가: ㈄ 한 번뿐이지만 되돌릴 수 없다(효과 확인은 한 번 보면 앞선 앎이 된다). 봉인 문서에 「어느 이름을 왜 부르나」를 사전 등록하면 닫힌다
- 무엇이 드러내나: 봉인 문서에 부를 이름 목록이 없는 것 · ㈄ 기록의 `touch` 인자가 `mcpServersFromToml` 인 것

### 2. `touch codexHostAdapter` 의 `호출자 N` 에는 제거가 실제로 깨뜨리는 자리가 거의 안 든다
- 어떻게 실패하나: ㈀ 가 서면 `호출자` 로 셀 수 있는 것은 함수 안에서 참조하는 두 자리다. `src/cli/commands/setup.ts:261` 과 `src/core/setup.ts:295`(`writeCodexSurfaceCatalog` 안)다. 그런데 제거의 핵심은 따로 있다. 하나는 `src/core/hosts/index.ts:5` `registerHostAdapter(codexHostAdapter);` 로, **파일 최상위** 라 엣지의 출발 심볼이 없다. 또 하나는 `:9` `export { …, codexHostAdapter }` 재수출이다. 레지스트리를 문자열 `'codex'` 로 찾는 자리도 `src/`·`tests/` 에 94 줄 있다. 계획은 「최상위 참조는 안 센다」를 화면이 말하게 했다. 그러나 장면 조건이 「`호출자` 가 파일 경계를 넘은 **실제 수**」라서, 2 가 서면 조건이 초록이 된다. 그 화면만 본 사람은 등록 줄을 놓친다.
- 어디가 걸리나: 계획 `## 계획` ㈀ 장면 조건 문면 · `crates/pal-cli/src/touch.rs:468`(`호출자 {}`) · ditto `src/core/hosts/index.ts:5`·`:9`
- 획득: 조회(ditto `git grep codexHostAdapter` · `git grep "'codex'"` 94 줄) + 추정(㈀ 뒤의 수 2 는 안 돌렸다 — 두 자리가 함수 안이라는 것은 읽었다)
- 모집단: 원의도
- 유효성: 참(자리 분포) · 추정(㈀ 뒤의 수)
- 해악도: 거짓신호
- 대상: 계획대상
- 얼마나 아픈가: 되돌릴 수 있다. 효과 판정 한 곳과 장면 조건 문면 한 곳이 걸린다
- 무엇이 드러내나: ㈄ 의 차이 기록에 `index.ts:5` 가 「화면이 안 말한 것」으로 나오는가 · 조건 문면이 「실제 수」인가 「하한」인가

### 3. ㈁ 이 `candidates` 분류만 싣고, 승인 안 된 `bound` 137 을 「걸린 것」에서도 「승인 대기」에서도 빠뜨린다
- 어떻게 실패하나: `pal narrative` 의 「결박됨 137」은 **의도 저장소의 결박이 아니다.** 인입 분류 `Classification::Bound` 이고 137 전부 `by = attached` 다. `touch` 의 「걸린 것」은 의도 저장소(`IntentIndex::bound_to`)만 읽으므로 이 137 은 승인 전까지 0 으로 나온다. 계획 ㈁ 은 「그 좌표를 **후보로** 가리키는 승인 안 된 조각」이다. 이것을 `Classification::Candidates` 로만 구현하면, 신호가 가장 강한 137 은 어느 구역에도 안 나온다. 사용자는 `narrative` 화면에서 「결박됨 137」을 보고, `touch` 에서는 「걸린 것 (0)」과 약한 후보만 본다.
- 어디가 걸리나: `crates/pal-core/src/narrative.rs:349-367`(`resolve` — 거리 0 신호 하나면 `Bound`) · `crates/pal-cli/src/narrative.rs:624-626`(「결박됨」 문구) · `crates/pal-cli/src/touch.rs:75-86`(`IntentIndex`) · `crates/pal-query/src/lib.rs:476`(`narrative.unbound` 만 `Bound` 를 센다)
- 획득: 조회(격리 복제본 `narrative --json` 의 `class` 분포 — `bound` 137 · `by` 전부 `attached`) + 추정(㈁ 이 `Candidates` 만 고를지는 계획 문면의 「후보로」에서 추정)
- 모집단: 원의도
- 유효성: 추정
- 해악도: 거짓신호
- 대상: 계획대상
- 얼마나 아픈가: 되돌릴 수 있다. `touch` 화면 한 구역과 `narrative` 화면의 「결박됨」 낱말 한 곳이 걸린다
- 무엇이 드러내나: `attached` 로 분류된 조각이 가리키는 심볼에 `touch` 를 불렀을 때 두 구역이 다 비는가

### 4. 승인한 조각이 `touch` 에 계속 「승인 대기」로 나온다 — 인입 분류가 의도 저장소를 안 본다
- 어떻게 실패하나: ㈄ 의 걸음은 `narrative → touch → 안내된 명령으로 승인 → touch` 이다. 인입 분류 `resolve(&f, projection)` 는 **투영만** 보고 의도 저장소를 안 본다(`narrative.rs:258`). 그래서 파생층에 남긴 목록의 그 조각은 승인 뒤에도 `candidates` 다. 승인은 코드 트리를 안 바꾸므로 스냅샷도 같다. 읽는 쪽이 의도 저장소로 걸러내지 않으면, 마지막 `touch` 에서 같은 조각이 「걸린 것」에도 「승인 대기(승인 명령 포함)」에도 동시에 나온다. 거절(`--refuse`)한 조각도 같다. 반대로 `bindings.jsonl` 이 워킹트리 digest 에 들어간다면, 승인할 때마다 「다른 스냅샷의 것」이 떠서 26.7 초 인입을 다시 요구한다.
- 어디가 걸리나: `crates/pal-cli/src/narrative.rs:245-260`(분류 시점) · `crates/pal-core/src/narrative.rs:349` · 계획 ㈁ 세부 「목록을 파생층에 남기고 `touch` 가 읽는다」
- 획득: 조회(분류가 의도 저장소를 인자로 안 받는 것 — 함수 서명) + 추정(읽기 시점 거르기를 할지 · `bindings.jsonl` 이 digest 에 드는지는 안 쟀다)
- 모집단: 원의도
- 유효성: 추정
- 해악도: 금지역 — 사실이_아닌_것을_사실로(승인된 것을 「승인 안 된」으로 적는다). 출처: 브리프 `## 금지역` 의 규약 §11 기본 다섯
- 대상: 계획대상
- 얼마나 아픈가: 되돌릴 수 있다. 그러나 ㈄ 의 마지막 화면이 곧 효과 판정 입력이라 판정 한 번이 오염된다
- 무엇이 드러내나: ㈄ 마지막 `touch` 에 같은 개체 id 가 두 구역에 나오는가 · 승인 뒤 「다른 스냅샷」 경고가 뜨는가

### 5. `touch` 자신의 재스티칭이 파생층의 후보 목록을 지워서, 첫 `touch` 부터 「아직 안 만들었다」가 나온다
- 어떻게 실패하나: `touch` 는 답하기 **전에** 매번 `attach::How::Stitching` 으로 세대를 갈아 끼운다(`touch.rs:118-121`). 교체는 정해진 표를 `delete_table` 하고 `*.staging` 을 rename 한다(`projection.rs:458-473`). 목록 표를 세대 목록에 넣으면, `touch` 의 스티칭은 목록을 안 쓰므로 빈 무대가 올라와 목록이 지워진다. 그 뒤 화면은 「`pal narrative` 를 먼저 돌리십시오」를 찍는다. 방금 돌렸는데도다. 세대 밖에 두면 계획이 적은 스냅샷 대조를 따로 세워야 한다. 두 길 중 앞의 것이 「스냅샷에 결박」을 공짜로 얻는 자연스러운 선택이라서 더 위험하다.
- 어디가 걸리나: `crates/pal-cli/src/touch.rs:118-121` · `crates/pal-cli/src/attach.rs:105-120` · `crates/pal-store/src/projection.rs:458-473`(`swap`)
- 획득: 조회(순서와 교체 대상을 읽었다) + 추정(목록을 어느 표에 둘지는 계획이 안 정했다)
- 모집단: 저장소
- 유효성: 추정
- 해악도: 금지역 — 사실이_아닌_것을_사실로(「아직 안 만들었다」가 거짓). 출처: 브리프 `## 금지역`
- 대상: 계획대상
- 얼마나 아픈가: 되돌릴 수 있다. 장면 조건 (a) 가 통째로 안 선다. 시험이 `narrative → touch` 를 한 프로세스 순서로 안 돌리면 안 잡힌다
- 무엇이 드러내나: 새 복제본에서 `narrative` 직후 `touch` 를 **두 번** 불러 두 화면이 같은가

### 6. `ts.resolveModuleName` 오라클이 ditto 에서 규칙 가지 대부분을 한 번도 안 지난다
- 어떻게 실패하나: 계획은 「옳은지는 TypeScript 컴파일러가 판정한다」고 하고 ditto 원본의 `typescript` 를 쓴다. ditto 의 실제 모양은 이렇다. `.tsx` 파일 0 · `.d.ts` 0 · 확장자 붙은 상대/별칭 지정자(`.js`·`.ts` 등) 0 · 디렉터리→`index.ts` 로 풀리는 상대 지정자 17. `extends` 사슬이 있는 `rebuild/tsconfig.json`·`rebuild/seam/tsconfig.json` 아래에서는 `~/` 임포트가 0 이다. 그래서 `.js→.ts` 대응 · `.tsx`·`.d.ts` 순서 · **상속된 `paths` 를 정의한 설정 파일 기준으로 푸는 규칙**(자식 디렉터리 기준으로 풀면 `rebuild/src/*` 로 틀린다)을 오라클이 한 번도 대조하지 않는다. 대조 일치율은 100% 로 나올 텐데, 계획이 「TS 일반」이라 부른 가지 절반 이상은 사람이 쓴 단위 시험만 판정한다.
- 어디가 걸리나: 계획 `### 계획의 세부` ㈀ 첫째·둘째 항 · ditto `tsconfig.json`(`baseUrl "."`·`paths ~/*`) · `rebuild/tsconfig.json`(`extends ../tsconfig.json`)
- 획득: 조회 — 격리 복제본에서 `git ls-files '*.tsx'`(0) · `'*.d.ts'`(0) · 지정자 정규식(확장자 붙음 0 · 디렉터리 index 17) · `git grep -c "from '~/" -- 'rebuild/*.ts'`(0)
- 모집단: 자기장치
- 유효성: 참(가지가 안 지나는 것) · 추정(그 가지에 실제 결함이 있는지)
- 해악도: 금지역 — 측정이_죽은_가지(오라클 대조가 판정하는 것처럼 적히는데 그 가지를 안 지난다). 출처: 브리프 `## 금지역`
- 대상: 계획자신
- 얼마나 아픈가: 되돌릴 수 있다. 가지 넷이 걸린다. 오라클에 합성 지정자 표본(가지마다 하나)을 먹이면 닫힌다
- 무엇이 드러내나: 오라클 대조 기록에 가지별 표본 수가 있는가. 없으면 0 인 가지가 안 보인다

### 7. 「어휘 없음」 모집단이 네 명령의 화면이라, 최상위 `pal --help` 의 `(F10)`·`(R-21)` 이 빠진다
- 어떻게 실패하나: 남의 저장소 사용자가 설치한 뒤 제일 먼저 보는 `pal --help` 목록에 `narrative … (F10)`·`cache … (R-21)` 이 찍힌다(실측). 계획의 모집단은 「장면 네 명령의 정상 화면 · `--help` · 거부·오류 화면」이고, 최상위 도움말은 어느 명령의 것도 아니다. 이 밖에 네 명령 안에서 실측으로 나온 것도 있다. `pal touch --help` 의 `옛 F11 §3.3`, `pal narrative --help` 의 `(F10)`, ㈂ 로 장면에 드는 `pal query --help` 의 `F17` 이다.
- 어디가 걸리나: `crates/pal-cli/src/main.rs`(서브커맨드 문서 주석) · 계획 ㈃ 세부의 모집단 문장
- 획득: 조회 — `pal --help` · `pal {install,ledger,narrative,touch,query} --help` 를 돌려 `F[0-9]{2}|R-[0-9]+|#[0-9]+|회차` 로 걸렀다
- 모집단: 저장소
- 유효성: 참
- 해악도: 거짓신호
- 대상: 계획자신
- 얼마나 아픈가: 되돌릴 수 있다. 최상위 줄 2 곳과 하위 도움말 3 곳이다
- 무엇이 드러내나: 어휘 검사가 `pal --help` 출력을 입력으로 받는가

### 8. 화면의 기능 코드는 `pal-cli` 의 리터럴이 아니라 데이터에서 온다 — 소스 문자열을 훑는 검사는 초록이다
- 어떻게 실패하나: `touch` 화면의 「(이 빌드에는 effects 능력이 없습니다 — F13 미구축)」과 근거 절 「미구축 F13 · F15」는 `pal-query` 가 만든 `CapabilityId::new("F13", "effects")` 값을 `format!` 으로 찍은 것이다. 카탈로그의 `"F10"`·`"F11"` 도 같다. 계획은 문자열 위치를 `grep` 으로 모았다. 조건이 「전수를 잰다」를 `crates/pal-cli/src` 의 문자열 리터럴 훑기로 구현하면 이 코드들은 안 걸리고 초록이 된다. `touch.rs:478-480` 처럼 `\` 로 이어진 줄에 든 「이 회차의 범위 밖」도 줄 단위 `println!` 정규식에는 안 걸린다.
- 어디가 걸리나: `crates/pal-query/src/lib.rs:680-681`·`:981-982` · `crates/pal-core/src/catalog.rs:244-245` · `crates/pal-cli/src/touch.rs:478-480`
- 획득: 조회(격리 복제본 `touch codexHostAdapter` 화면에서 `F13 미구축` 을 보고 출처를 `grep` 했다) + 추정(검사를 어떤 방식으로 만들지는 계획이 안 정했다)
- 모집단: 저장소
- 유효성: 추정
- 해악도: 금지역 — 측정이_죽은_가지. 출처: 브리프 `## 금지역` 이 「「어휘가 없다」 검사가 사람 화면이 아닌 것을 훑으면 초록이 거짓」으로 지목
- 대상: 계획자신
- 얼마나 아픈가: 되돌릴 수 있다. 데이터 출처 두 파일과 이어진 줄 리터럴이 걸린다
- 무엇이 드러내나: 검사가 **실행한 화면 출력**을 훑는가, 소스를 훑는가 · 음성 대조로 `F13` 을 심은 표본이 빨개지는가

### 9. `pal install` 이 남의 저장소에 까는 산출물에 회차·이슈 어휘와 ditto 에 없는 경로가 그대로 실린다
- 어떻게 실패하나: 대상 복제본(이미 설치된 상태)의 `.claude/skills/pal-round/SKILL.md` 에서 `회차|#1xx|F1x` 에 걸리는 줄이 96 개다. `.claude/agents/pal-*.md` 9 개도 같은 어휘를 싣는다. `pal-premortem-sweeper.md` 는 `../../docs/adr/0023-…` 링크(ditto 에는 `docs/adr` 가 없다)와 `cargo xtask check`(TS 저장소)를 지시한다. Claude Code 사용자는 이것을 에이전트 지시로 받는다. 인터뷰는 선을 「**화면**의 회차·이슈·기능 코드」로 잠갔고, ㈃ 의 모집단은 네 명령의 출력이다. 그래서 설치 산출물은 조건 밖으로 조용히 빠진다. 장면 조건은 초록인데 남의 저장소에는 이 저장소의 회차 규약이 수출돼 있다.
- 어디가 걸리나: `crates/pal-cli/src/install/`(까는 목록) · 대상 복제본 `.claude/skills/pal-round/SKILL.md` · `.claude/agents/pal-premortem-sweeper.md:37`·`:92`
- 획득: 조회 — 대상 복제본을 읽기로 `grep -c`·`grep -n`
- 모집단: 원의도
- 유효성: 참
- 해악도: 거짓신호
- 대상: 계획대상
- 얼마나 아픈가: 되돌릴 수 있다(`pal uninstall`). 산출물 10 파일이 걸린다. 범위 판정만 명시하면 된다(넣든 빼든 「철회」 사유로)
- 무엇이 드러내나: 종료 보고의 범위 밖 절에 설치 산출물이 적혀 있는가

### 10. ㈀㈁㈂㈃ 이 「서로 독립」이라는 순서 주장이 파일 단위로 거짓이다 — 같은 함수와 같은 단언을 둘이 고친다
- 어떻게 실패하나: ㈀ 는 「못 선 몫이 가는 문」 문구를 언어 중립으로 바꾸고, 새 까닭(「tsconfig 를 다 못 읽었다」·「재수출을 지나는 이름」)을 이관표에 더한다. ㈃ 는 같은 이관표의 `` `A5`·`A5-a` ``·`` `#133`(L2) ``·「이 회차」를 사용자 말로 옮긴다. 둘 다 `print_transfer`(`touch.rs:520-535`)와 그 화면을 재는 단언(`tests/cross_file_references.rs:318-326` — 문 목록에 `` `#133`(L2) `` 가 있다)을 고친다. ㈁·㈂ 도 `touch.rs` 의 `run` 과 후보 화면(`:275`)을 함께 만진다. 병렬 하위 작업으로 나누면 충돌하고, 뒤에 병합된 쪽이 앞 쪽의 문구나 단언을 되돌린다. 그러면 「옮기지 지우지 않는다」로 지키려던 구별 단언이 한쪽 병합에서 사라진다.
- 어디가 걸리나: 계획 `**순서**` 문단 · `crates/pal-cli/src/touch.rs:505-535` · `crates/pal-cli/tests/cross_file_references.rs:313-330` · `crates/pal-core/src/cross_file.rs` `UnresolvedReason::as_str`
- 획득: 조회 — 단언 문자열과 화면 함수를 읽었다. 테스트 파일에서 「이 회차」가 든 줄은 16
- 모집단: 자기장치
- 유효성: 참
- 해악도: 실패
- 대상: 계획자신
- 얼마나 아픈가: 되돌릴 수 있다. 파일 3 곳이 걸린다. ㈀→㈃ 순서 하나를 박으면 닫힌다
- 무엇이 드러내나: 병합 충돌 · 병합 뒤 `cross_file_references` 시험

### 11. 맨 지정자를 모양만 보고 `outside_repo` 로 둔다 — 모노레포에서는 저장소 안인데 화면이 「원리상 못 섭니다」라고 적는다
- 어떻게 실패하나: 계획은 「`node:`·외부 패키지는 계속 `outside_repo`」다. TS 일반에서 워크스페이스 패키지(`@scope/pkg` → `packages/pkg/src`)나 `package.json` `imports`(`#x`) 는 저장소 안의 파일이다. 그런데 이것을 맨 지정자라서 `outside_repo` 로 적으면, 화면 이관표가 「**경계** — 저장소 밖이라 원리상 못 섭니다」라고 단언한다. ditto 는 `package.json` 이 하나이고 `workspaces` 가 없다. deps 에 없는 맨 지정자도 `bun:test` 80 · `fs` 4 · `@playwright/test` 3 · `path` 2 뿐이라 **ditto 로는 이 가지를 못 드러낸다.** 「해소 규칙을 ditto 에만 맞추지 않는다」가 이 자리에서 측정 없이 선언으로만 남는다.
- 어디가 걸리나: `crates/pal-core/src/cross_file.rs:471-473`(`우리것` 판정) · `crates/pal-cli/src/touch.rs:528`(`outside_repo` 문구)
- 획득: 조회(ditto 의 맨 지정자와 `package.json` 대조) + 추정(다른 저장소의 워크스페이스 분포는 안 쟀다)
- 모집단: 원의도
- 유효성: 추정
- 해악도: 금지역 — 사실이_아닌_것을_사실로. 출처: 브리프 `## 금지역`
- 대상: 계획대상
- 얼마나 아픈가: 되돌릴 수 있다. 첫 릴리스 대상이 ditto 하나라 이번 장면은 안 막는다. 까닭 이름 하나를 「저장소 밖으로 **분류했다**」로 약하게 적으면 닫힌다
- 무엇이 드러내나: 워크스페이스 픽스처 하나를 두고 이관표 문구를 보는 시험

### 12. 「tsconfig 는 스냅샷 트리에서」가 커밋 트리로 구현되면, 워킹트리에서 읽은 소스와 갈린다
- 어떻게 실패하나: 스냅샷은 `TreeRef::Worktree { base, tree_digest }` 로 워킹트리 내용까지 담는다. 한편 기존 인입 코드는 이력을 `at.base()`(커밋)로 읽는다. ㈀ 가 tsconfig 를 `base` 커밋의 트리에서 읽으면, 커밋 안 된 `paths` 편집이 반영 안 된 규칙으로 워킹트리 소스를 푼다. 틀린 파일을 잡거나 `no_target_file` 이 된다. 오라클(`ts.resolveModuleName`)은 파일 시스템을 읽으므로, 같은 지정자에 두 답이 나오고 불일치가 해소기의 결함처럼 보인다.
- 어디가 걸리나: `crates/pal-core/src/repo.rs:229-232` · `crates/pal-cli/src/narrative.rs:184-190`(`at.base()` 사용례) · 계획 ㈀ 둘째 세부
- 획득: 조회(타입과 사용례) + 추정(㈀ 가 어느 트리를 읽을지)
- 모집단: 저장소
- 유효성: 추정
- 해악도: 거짓신호 — tsconfig 를 고친 채 커밋 안 한 드문 상태에서만 난다. 그때 거짓 엣지가 서면 금지역으로 올라간다
- 대상: 계획대상
- 얼마나 아픈가: 되돌릴 수 있다. 한 곳이다
- 무엇이 드러내나: 커밋 안 된 tsconfig 편집을 둔 픽스처에서 `touch` 와 `--at` 이 같은 답을 내는가

### 13. ㈂ 의 지목 문법이 `narrative --pick` 과 갈린다 — 한 장면에 좌표를 부르는 방법이 둘
- 어떻게 실패하나: `narrative --pick` 은 SymbolId 전체나 짧은 해시만 받는다(`narrative.rs:551-554`). `touch` 의 동명 후보 화면은 해시 없이 `kind name path:line` 만 찍는다(실측: `localDir` → `rebuild/util/paths.ts:24` · `src/core/ditto-paths.ts:24`, `claim` → `rebuild/verify/codex.test.ts:9` · `src/core/github-claim.ts:90`). ㈂ 가 경로나 `path:line` 으로 지목하게 만들면, 같은 장면의 ㈁ 승인 안내는 해시를 요구한다. 사용자는 후보 화면에서 본 표기를 승인 명령에 넣었다가 「후보 밖의 좌표」 거부를 받는다.
- 어디가 걸리나: `crates/pal-cli/src/narrative.rs:551-563` · `crates/pal-cli/src/touch.rs:275`·`:355` · `crates/pal-cli/src/query.rs:327`
- 획득: 조회(격리 복제본에서 `touch localDir`·`touch claim` 을 돌렸고 `좌표()` 를 읽었다) + 추정(㈂ 가 고를 표기)
- 모집단: 원의도
- 유효성: 추정
- 해악도: 미관
- 대상: 계획대상
- 얼마나 아픈가: 되돌릴 수 있다. 세 자리가 걸린다. ditto 의 두 사례는 다른 파일이라 경로로도 갈린다
- 무엇이 드러내나: ㈄ 에서 화면이 안내한 문자열을 **그대로 복사해** 승인 명령에 넣었을 때 통하는가

## 내가 기각한 것

| 제목 | 어떻게 실패하나(가정했던 것) | 어디가 걸리나 | 획득 | 모집단 | 유효성 | 해악도 | 대상 | 얼마나 아픈가 |
|---|---|---|---|---|---|---|---|---|
| TS 대상 파일에서 이름을 찾을 때 담은 것을 안 봐 메서드 동명에 거짓 엣지·모호가 선다 | ⓐ 가 `by_name[(target, name)]` 를 담은 것 없이 찾으므로, 최상위에 없고 메서드에만 있는 이름에 거짓 엣지가 서거나 최상위+메서드 동명이 모호로 떨어진다 | `crates/pal-core/src/cross_file.rs:529-562` | 조회 — 격리 복제본의 TS 전부에 `pal symbols --json` 을 돌리고, 정규식으로 이름 임포트를 풀어 대상 파일의 심볼 모양을 셌다. 최상위 하나·메서드 없음 **3141** · 최상위 없음(배럴) **71** · 메서드 동명 **0** · 최상위 여럿 **0** | 저장소 | 거짓 | 금지역 | 계획대상 | ditto 에서는 0 곳 |
| 대상 파일 목록을 「파싱한 파일 전부」로 넓히면 Rust 에서 심볼 0 파일이 1차 후보를 가로채 회귀한다 | `module_candidates` 1차에 지금은 안 잡히던 빈 파일이 잡혀 2차 폴백이 끊기거나 모호가 는다 | `crates/pal-core/src/cross_file.rs:382`·`:484-489` | 조회 — 이 저장소의 `.rs` 전부(`crates/` 안팎)에 `pal symbols` 를 돌려, 심볼 행이 0 인 파일 **0** | 저장소 | 거짓 | 실패 | 계획대상 | 0 곳 |
| 개체 id 가 인입마다 새로 민팅돼 `touch` 가 보인 승인 명령이 실패한다 | ㈁ 이 파생층 목록의 id 를 찍고, 다음 인입이 다른 id 를 민팅한다 | `crates/pal-cli/src/narrative.rs:236-257` | 조회 — `intent.entity_of(&origin)` 이 있으면 재사용하고 없을 때만 `keep_entity` 로 한 번 남긴다 | 저장소 | 거짓 | 실패 | 계획대상 | 0 곳 |
| 스냅샷이 커밋만 담아, 워킹트리 변경(㈄ 의 codex.ts 제거) 뒤 낡은 후보 목록을 현재 것으로 판정한다 | 목록의 스냅샷 표지가 `aded7ce+worktree` 문자열뿐이라 내용이 바뀌어도 같다 | `crates/pal-core/src/repo.rs:229-232` | 조회 — `TreeRef::Worktree` 가 `tree_digest` 를 진다 | 저장소 | 거짓 | 거짓신호 | 계획대상 | 0 곳 |
| ditto 에 `baseUrl` 기준 맨 지정자가 있어 외부 패키지로 잘못 분류된다 | `baseUrl "."` 이라 `src/…` 꼴 맨 지정자가 저장소 안인데 `outside_repo` 가 된다 | ditto `tsconfig.json` | 조회 — deps 에 없는 맨 지정자는 `bun:test` 80 · `fs` 4 · `@playwright/test` 3 · `path` 2 뿐이다(나머지는 정규식 잡음) | 원의도 | 거짓 | 금지역 | 계획대상 | ditto 에서는 0 곳(다른 저장소는 시나리오 11) |
| TS 에서도 ⓑ 꼬리가 `x.foo()` 의 `foo` 를 대상 파일의 동명 최상위 심볼에 잇는다 | `codexHostAdapter.loadSurfaceInventory()` 의 꼬리가 대상 파일에서 주인 없는 한 건으로 잡힌다(`cross_file.rs:597`) | `crates/pal-extract/src/scopes.rs:95` · `crates/pal-extract/src/ts_scopes.rs` | 조회 — `ts_scopes.rs` 가 `call_tail` 을 재정의하지 않아 기본값 `None`. 착수 화면도 `ⓑ path-resolution 0/0` | 저장소 | 거짓 | 금지역 | 계획대상 | 0 곳 |
| 착수 둘째 걸음(`NEXT-E-*.md` 지우기)이 링크를 깨뜨린다 | 루트의 두 파일을 지우면 그것을 가리키는 문서가 죽은 링크가 된다 | 저장소 뿌리 | 조회 — 두 파일은 이미 `e6ad3f2` 에서 지워졌고, 남은 언급은 앞 회차 기록 안의 코드 표기뿐이다 | 회차기록 | 거짓 | 실패 | 계획자신 | 0 곳 |
| ㈁ 의 파생층 목록 저장은 만들 필요가 없다 — `touch` 가 인입을 직접 돌리면 된다 | 더 작은 표면(새 저장 없이 `QueryCtx::narrative` 를 채움)으로 같은 답이 나온다 | `crates/pal-cli/src/touch.rs:173-174` | 추정 — 인입 비용 26.7 초(브리프 실측)를 `touch` 마다 치르면 같은 답이 아니다. 이번에 다시 시간을 재지는 않았다 | 원의도 | 거짓 | 미관 | 계획자신 | — |

새 범주: 과제 입자와 질의 입자의 어긋남(파일을 지우는 과제를 심볼 이름 하나로 묻는다) · 설치 산출물을 통한 호스트 내부 어휘의 수출 · 세대 교체가 곁 데이터를 지우는 자리
