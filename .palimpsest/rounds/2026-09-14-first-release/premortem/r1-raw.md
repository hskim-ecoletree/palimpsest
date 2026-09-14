# 사전부검 R1 — 원 반환문

> 회차 `2026-09-14-first-release` · 라운드 R1(상한 1) · HEAD `ec92b89` · 바이너리 `pal 0.0.0+ec92b899478d`
>
> 읽은 절: `intent.md` 전 절(원문 · 목적 기여 · 착수 시점 관측 · 계획 · 상한 · 모집단 분리 · 금지역 · 범위 밖 · 차선책) · `state.md` · `baseline/` 넷.
> 금지역 목록의 출처: `intent.md ## 금지역`(규약 §11 기본 다섯 · `.claude/pal/policy.toml` 없음).
>
> 실측 자리: 세션 스크래치 `pm-r1/` — 이 저장소를 `pal` 이름으로 클론한 사본(`pal/`) · 버전 올림 사본(`verbump/`) · Rust 합성 저장소(`amp/`).
> ditto 원본에는 `git ls-files` · `git remote -v` · `git log -1` 만 읽었다(쓰기 없음).

---

### #77 — 요약값에 속성을 넣으면 코드가 안 바뀐 결박 22 건이 `stale` 로 뒤집힌다
- 어떻게 실패하나: 계획 2 가 지키는 불변은 **「속성이 없는 심볼」** 의 요약값뿐이다. 이 저장소 결박 42 건이 감시하는 심볼 172 개 중 **112 개(65%)가 바로 앞 줄에 `#[...]`** 를 진다(`#[derive]` · `#[must_use]` · `#[test]` · `#[serde]`). 속성을 요약에 넣으면 이 112 개의 `watch.digest` 가 전부 달라지고, 그것을 감시하는 **결박 22/42** 가 코드 변화 없이 `최신 상태 아님(stale)` 이 된다. 차선책의 「승격」은 *속성 없는* 심볼이 흔들릴 때만 걸리므로 이 뒤집힘은 계획의 어느 문도 안 거친다. `intent.md ## 금지역` 이 이름으로 적은 「반대 방향의 거짓(코드가 안 바뀌었는데 stale)」이다. 저장소 전체로는 Rust 심볼 3,468 중 1,715 가 속성을 진다.
- 어디가 걸리나: `crates/pal-extract/src/parse.rs` `normalize_into` · `crates/pal-extract/src/lib.rs:215` `EXTRACTOR_REV` · `.palimpsest/intent/bindings.jsonl` 의 `watch[].digest`
- 획득: 조회 — 스크래치 `pal/` 에서 `pal query graph.dump --json` 의 심볼 `span.byte_start` 와 `bindings.jsonl` 의 `watch[].symbol` 을 이어 원문 앞 줄을 쟀다. 뒤집힘 자체는 구현 전이라 안 돌렸다(요약값이 속성을 담으면 digest 가 달라지는 것은 계획이 적은 설계 그대로다).
- 모집단: 저장소
- 유효성: 참
- 해악도: 금지역
- 대상: 계획대상
- 얼마나 아픈가: 재승인으로 되돌릴 수는 있으나 22 건을 사람이 하나씩 다시 봐야 한다. 소유자 저장소뿐 아니라 Rust 결박을 가진 모든 설치가 첫 갱신에서 같은 일을 겪는다.

### #77 ② — `is_leading_separator` 는 Rust 에서 죽은 가지가 아니다: 지우면 속성 없는 심볼 요약도 바뀐다
- 어떻게 실패하나: 계획은 ②를 「죽은 가지라 지운다」로 적었고 이슈의 「전수 참 0 건 · 삭제해도 바이트 불변」은 **ditto(TS)·boxwood(Kotlin)** 에서 잰 것이다. 합성 Rust 저장소에서 `impl A { pub fn ma(&self) -> u8 { 1 } }` 과 `ma(self)` 의 본문 digest 가 **같다**(`6524bbf8…` 두 번) — `self_parameter` 의 맨 앞 `&` 가 지금 이 함수로 벗겨진다. 지우면 속성이 없어도 `&self`·`&mut self` 를 가진 심볼 요약이 바뀐다(저장소 Rust 심볼 269 · 감시 심볼 10). 계획 2 의 불변 「속성 없는 심볼은 바이트로 불변」이 ② **자신에게** 깨지고, ①과 합치면 감시 심볼 122/172 · 결박 27/42 가 뒤집힌다. 그리고 「죽은 가지」라는 문장이 사실이 아닌 채 커밋 메시지·이슈 답에 실린다.
- 어디가 걸리나: `crates/pal-extract/src/parse.rs:214` · `:248` `is_leading_separator`
- 획득: 조회 — 스크래치 `amp/` 에 `a.rs`(`&self`·`&str`) · `b.rs`(`self`·`str`) · `c.rs`(= a) 를 커밋하고 `pal query graph.dump --json` 의 `body` 를 댔다. 결과 `ma` 셋이 같은 digest, `fa` 는 `&str`↔`str` 이 갈렸다. 저장소 전수 269 는 `(\s*&\s*(mut\s+)?self` 정규식으로 센 하한이다.
- 모집단: 저장소
- 유효성: 참
- 해악도: 금지역
- 대상: 계획대상
- 얼마나 아픈가: 커밋 하나로 되돌릴 수 있다. 다만 `&self`↔`self` 를 가르는 것 자체는 R-22 쪽으로 옳은 변화라, 지우든 안 지우든 「뒤집힘을 어떻게 알리나」가 남는다.

### #159 — `pal install` 이 디렉터리 이름을 식별자로 적으면 틀린 식별자가 「선언」으로 굳는다
- 어떻게 실패하나: 계획 4 는 install 이 `[[repo]] id` 를 적는데 **값을 어디서 얻는지 안 적었다.** 지금 쓸 수 있는 값은 `repo_name` = 디렉터리 이름뿐이다. 결박이 `palimpsest` 로 선 저장소를 `pal/` 로 받아 install 하면 `id = "pal"` 이 적힌다. 실측: 그 매니페스트로 `pal touch TsProject` 는 여전히 **`(0) 아직 없습니다`**, `id = "palimpsest"` 로 바꾸면 `(1) 최신 상태(fresh)`. 그런데 계획의 드러냄은 **「매니페스트 없는 저장소에서」** 만 걸리고 「이미 있으면 안 건드린다」가 붙어 있다. 결국 틀린 id 가 매니페스트에 선언으로 박히고, 드러냄은 꺼지고, 다음 install 도 안 고친다. `bindings.jsonl` 의 `bound_at[0][0]` 에 결박이 선 식별자가 이미 적혀 있는데 계획은 그것을 안 본다. 앞 회차의 ditto 결박은 `r4-ditto` 로 섰다(이슈 #159 본문).
- 어디가 걸리나: `crates/pal-cli/src/ledger.rs:125-128` · `:606` `repo_name` · `crates/pal-core/src/manifest.rs` `Manifest::parse` · install 의 새 쓰기 자리(미정)
- 획득: 조회 — 스크래치 `pal/` 에서 `pal intent import` 뒤 매니페스트 없음 · `id="pal"` · `id="palimpsest"` 세 상태로 `pal touch TsProject` 를 돌렸다.
- 모집단: 저장소
- 유효성: 참
- 해악도: 금지역
- 대상: 계획대상
- 얼마나 아픈가: 매니페스트 한 줄을 고치면 되돌아온다. 하지만 **사람이 틀렸다는 사실을 알 길이 없다** — 화면이 `0` 을 사실로 낸다. 팀원 클론 · 워크트리 · CI 체크아웃 디렉터리가 전부 걸린다.

### #159 — install 이 `.palimpsest/manifest.toml` 을 쓰면 릴리스 워크플로의 왕복 검사가 빨개지거나, uninstall 이 팀원의 식별자를 지운다
- 어떻게 실패하나: 되돌리기의 상한은 컴파일된 상수 `OWNED_DIRS`(`.claude/pal` · `.claude/commands/pal`)·`OWNED_FILES`·`DIRS` 이고 `.palimpsest/` 는 거기 없다. 갈래는 둘이다. ⑴ 그대로 두면 `release.yml` 의 `toolchain-free` 가 `install → doctor → update → uninstall` 뒤 `git status --porcelain` 이 비었는지 보는데, `?? .palimpsest/` 가 남아 **세 플랫폼 전부 빨갛다.** ⑵ 상한에 더해 uninstall 이 지우게 하면, 커밋돼 팀원이 기대는 식별자 선언을 uninstall 한 번이 지운다(계획의 「이미 있으면 안 건드린다」와 되돌리기 규율이 부딪친다). 그리고 이 왕복 검사는 `ci.yml` 에 없고 `release.yml` 에만 있다 — `interop-receive.sh` 는 설치 결과를 커밋한 뒤에 재서 이 형태를 못 본다. 그래서 **태그를 민 뒤에야** 드러난다.
- 어디가 걸리나: `crates/pal-cli/src/install/layout.rs:179` `OWNED_DIRS` · `:187` `OWNED_FILES` · `.github/workflows/release.yml` 「받은 바이너리 하나로 라이프사이클을 완주한다」 · `scripts/interop-receive.sh:45` · `:71`
- 획득: 추정 — 상한 상수와 두 스크립트의 검사 순서는 읽었다(조회). install 이 무엇을 쓸지는 구현 전이라 못 돌렸다.
- 모집단: 저장소
- 유효성: 추정
- 해악도: 실패
- 대상: 계획대상
- 얼마나 아픈가: ⑴ 은 태그 뒤 릴리스가 안 선다(태그를 지우고 다시 민다). ⑵ 는 팀 단위로 조용히 0 이 돌아온다.

### #127 — 「같은 인덱스면 모든 경로가 같은 값」이 게이트 `[f05.3.pass]` ⑤ 와 그 시험을 정면으로 어긴다
- 어떻게 실패하나: 착수 RED(`02-unresolved-two-paths.txt`)는 `ledger.snapshot`(`미해소 0 · L0`)과 `graph.dump`(`미해소 13404 · L1`)를 댔다. 그런데 이 둘은 하드코딩이 아니다. 둘 다 `coverage_of(ctx, accessed)` 이고 **질의가 만진 파일에서만 센다.** `ledger.snapshot` 은 심볼을 안 만져 `0 · L0`(주석: *"닿은 파일이 없으면 L0 — 그것이 정확하다"*)이고 `graph.dump` 는 심볼 있는 파일만 합한다. 같은 함수의 doc 이 *"질의마다 다른 값이어야 한다 (`[f05.3.pass]` ⑤) — 전역 합을 복사하면 답의 성질이 아니라 저장소의 성질이 된다"* 로 반대 결정을 적었고, `범위는_질의마다_다른_값이다` 가 `assert_ne!` 로 잠갔다. 계획 문장대로 두 경로를 같게 만들면 이 시험이 빨개진다. 시험을 고쳐 맞추면 앞 결정을 조용히 뒤집는다. 안 건드리면 착수 RED 가 안 닫힌다. 하드코딩은 `doctor.rs:186` · `export.rs:392` 두 자리뿐이고 RED 는 그 둘을 재지 않는다. 덧붙여 `graph.dump` 의 `L1` 은 `unsupported 991` 파일의 L0 를 뺀 값이라, 어느 쪽을 「참」으로 고르느냐가 곧 사실 판정이다.
- 어디가 걸리나: `crates/pal-query/src/lib.rs:656-690` `coverage_of` · `crates/pal-cli/tests/query_envelope.rs:170-190` · `crates/pal-cli/src/doctor.rs:186` · `crates/pal-cli/src/export.rs:392`
- 획득: 조회 — `grep -rn 'unresolved: 0'`(두 자리) · `coverage_of` 와 시험 본문을 읽고 착수 산출 `02` 와 댔다.
- 모집단: 저장소
- 유효성: 참
- 해악도: 실패
- 대상: 계획대상
- 얼마나 아픈가: 시험 하나가 빨개진다(되돌릴 수 있다). 시험을 고쳐 통과시키면 게이트 ⑤ 가 죽은 가지가 된다.

### 태그 push 가 한 달 가까이 안 돈 릴리스 워크플로의 첫 실측이 된다
- 어떻게 실패하나: `release.yml` 의 마지막 실행은 2026-08-16 두 번(첫 번은 `failure`, 둘째는 `success`)이고 둘 다 `workflow_dispatch` 였다. **태그 push 로 돈 적은 없다.** 그 뒤로 install 의 짐(에이전트 아홉 · 회차 규약)과 Windows 경로가 자랐다. 워크플로 머리는 *"태그 없이도 돌려 볼 수 있어야 한다 — 안 그러면 이 워크플로의 첫 실행이 곧 되돌릴 수 없는 태그가 된다"* 라고 적었는데, 계획 6 은 「CI 초록이면 묻지 않고 태그 push」다. 그런데 `ci.yml` 은 `toolchain-free` 왕복도, `x86_64-apple-darwin` 빌드도, 아카이브 담기도 안 돈다. CI 초록이 그 셋의 초록을 뜻하지 않는다.
- 어디가 걸리나: `.github/workflows/release.yml` `on.workflow_dispatch` · `toolchain-free` · `publish` · 계획 6
- 획득: 조회 — `gh run list --workflow release.yml -L 3` · `release.yml` · `ci.yml` 잡 목록을 읽었다.
- 모집단: 저장소
- 유효성: 참
- 해악도: 실패
- 대상: 계획대상
- 얼마나 아픈가: 공개 저장소의 `v0.1.0` 태그를 지우고 다시 밀어야 한다. 받은 사람이 있으면 같은 이름이 두 바이너리를 가리킨다.

### 효과 걸음의 「다른 디렉터리 이름으로 한 번 더」는 ditto 에서 `0 = 0` 항등식이다
- 어떻게 실패하나: ditto origin 의 추적 파일에 `.palimpsest/intent/` 가 없다(`git ls-files` 에 `reports/design/palimpsest-ditto-convergence-design.md` 하나뿐). 새 복제본 둘은 결박이 둘 다 0 이라 #159 가 고쳐졌든 안 고쳐졌든 `■ 이 좌표에 걸린 것 (0)` 이 같게 나온다. 첫 복제본에서 `narrative`·승인으로 결박을 만들어도, 매니페스트와 `bindings.jsonl` 을 **커밋하지 않고** `git clone` 으로 둘째를 뜨면 둘째엔 안 간다. 결과는 여전히 0 = 0 이고, 이것은 「같은 답을 받는다」로 읽힌다. `intent.md ## 금지역` 의 「측정이_죽은_가지 — 음성 대조가 항등식이면 안 된다」 그 자리다. 착수 RED 의 #159 는 이 저장소(결박 42)에서 쟀는데 효과는 결박 0 인 저장소에서 잰다.
- 어디가 걸리나: 계획 7 · `~/dev/projects/ditto` 추적 파일(읽기만)
- 획득: 조회(ditto 추적 상태 — `git ls-files | grep palimpsest`) + 추정(계획 7 을 실제로 어떻게 밟을지는 계획이 안 적었다)
- 모집단: 자기장치
- 유효성: 참
- 해악도: 금지역
- 대상: 계획자신
- 얼마나 아픈가: 다시 재면 되지만, 모르고 넘기면 「팀원도 같은 답」이 사실로 보고서에 실린다.

### 효과·릴리스 검사 어디에서도 #77·#139 와 릴리스 바이너리의 `touch` 가 안 지난다
- 어떻게 실패하나: 계획 7 의 사용 대상은 ditto(TypeScript)뿐이라 Rust 전용인 #77·#139 의 고침은 실사용 장면을 한 번도 안 지난다. 릴리스 워크플로가 받은 바이너리로 돌리는 것은 `install · doctor --install · update · uninstall` 뿐이다 — 제품의 핵심 장면인 `narrative`·`touch` 는 Windows·Linux 릴리스 바이너리에서 한 번도 안 돈다(ADR-0023). README 가 「지원 언어 TypeScript·Rust」·「세 플랫폼」을 적으면 그 문장의 절반은 소스 CI 로만 받쳐진다.
- 어디가 걸리나: 계획 6·7 · `.github/workflows/release.yml` `toolchain-free` 의 명령 넷 · `docs/adr/0023-consistent-method-and-result-across-platforms.md`
- 획득: 조회(워크플로 단계 · 계획 문면) — 소스 CI 의 Windows 시험이 같은 경로를 얼마나 덮는지는 안 쟀다.
- 모집단: 자기장치
- 유효성: 참
- 해악도: 거짓신호
- 대상: 계획자신
- 얼마나 아픈가: 코드는 안 깨진다. 「릴리스를 받아 썼다」가 macOS·TS 한 칸의 관측을 전 칸으로 읽게 만든다.

### #156 — 이관 항목이 `pal query symbol.callers <이름>` 만 가리키면 동명 심볼에서 안 펴진다
- 어떻게 실패하나: `touch` 와 `query` 는 둘 다 이름으로 받고, 후보가 여럿이면 거부한 뒤 `--pick <짧은 해시>` 를 요구한다. 실측: `new` 는 **32 후보**. 사용자가 `pal touch new --pick …` 로 한 심볼을 본 화면이 `pal query symbol.callers new` 를 안내하면, 그 명령은 자리가 아니라 후보 32 를 낸다. 이관 타입 `Fold::push(what, count, unfolded_by: QueryName)` 는 **질의 이름만** 지고 인자도 지목도 못 싣는다. 그래서 계획의 「`<이름>` 을 가리킨다」는 ADR-0011 의 모양을 넓히거나 화면 문자열로 우회해야 한다.
- 어디가 걸리나: `crates/pal-core/src/envelope.rs:295-311` `Fold`·`push` · `crates/pal-core/src/touch.rs:243` `SymbolFacts` · 계획 5
- 획득: 조회 — 스크래치 `pal/` 에서 `pal query symbol.callers new` · `pal touch new` · 두 명령의 `--help` · `Fold` 정의를 읽었다.
- 모집단: 저장소
- 유효성: 참
- 해악도: 거짓신호
- 대상: 계획대상
- 얼마나 아픈가: 화면 한 줄이다. 되돌리기 쉽다. 자주 쓰는 이름(`new`·`from`·`parse`)일수록 안내가 쓸모없어진다.

### #139 — 음성 대조를 다시 세우는 일이 계획 3 에서 빠졌다
- 어떻게 실패하나: 이슈 #139 는 고칠 조건을 ⑴ 가시성 · ⑵ 크레이트 경계에서 사적 대상 거부 · **⑶ 「재수출에 가린 사적 모듈」을 픽스처로 심은 음성 대조** 셋으로 적었고, *"고칠 때 음성 대조도 함께 다시 세워야 한다"* 고 못 박았다. 계획 3 은 ⑴⑵ 와 「참인 56 은 그대로」만 적었다. 착수 관측표도 #139 만 「착수 뒤 루프에서 다시 잰다」로 RED 산출 파일이 없다. 그러면 저장소 실측(58 중 2)만으로 GREEN 을 선언하게 되는데, 그 2 는 이 저장소의 `pal-core/src/lib.rs:45`·`:143` 한 쌍에 기댄다. 픽스처가 없으면 코드가 그 한 쌍의 모양(`mod x;` + `pub use x::{…}`)에만 맞춰져도 통과한다.
- 어디가 걸리나: 계획 3 · 이슈 #139 「무엇이 갖춰지면 고칠 수 있나」 · `crates/pal-cli/tests/cross_file_references.rs`
- 획득: 조회 — 이슈 본문(`gh issue view 139`)과 `intent.md ## 계획` 3 을 댔다.
- 모집단: 원의도
- 유효성: 참
- 해악도: 금지역
- 대상: 계획대상
- 얼마나 아픈가: 완수 조건에 한 줄 더하면 닫힌다. 빠진 채 가면 앞 회차의 `A5-a` 와 같은 병(원리상 0 을 내는 대조)이 반복된다.

### #139 — `SymbolNode` 에 칸을 더하면 `ROW_FORMAT_REV`·캐시 직렬화도 함께 올라야 하는데 계획이 안 적었다
- 어떻게 실패하나: 2층 행 형식은 `ROW_FORMAT_REV = "f09-imported-and-unresolved"` 로 문지기를 하고, 1층 캐시 키는 `ExtractorVersion` 이다. 계획 3 은 「스키마(`schema/graph.toml`)가 함께 움직인다」만 적었다. 계획 2 에서 `EXTRACTOR_REV` 를 이미 올린 뒤 3 을 다른 커밋으로 얹으면, 2 의 REV 로 세운 캐시·인덱스를 새 구조로 읽는다. postcard 는 스스로 모양을 설명하지 않는 형식이다. 끝에 붙은 칸이면 디코드 실패(시끄럽다)로 끝나지만, 중간에 끼면 **다른 칸 값을 밀어 읽을** 수 있다.
- 어디가 걸리나: `crates/pal-store/src/projection.rs:137` `ROW_FORMAT_REV` · `crates/pal-core/src/touch.rs:35` `SymbolNode` · `crates/pal-store/src/cache.rs:96`
- 획득: 추정 — 문지기 상수와 구조체는 읽었다. 캐시가 `SymbolNode` 를 postcard 로 싣는지는 끝까지 안 따라갔고, 새 칸으로 옛 행을 읽는 실험도 안 돌렸다.
- 모집단: 저장소
- 유효성: 추정
- 해악도: 실패
- 대상: 계획대상
- 얼마나 아픈가: 개발 기계와 앞 회차 스크래치 인덱스에만 걸린다(릴리스 사용자는 새 인덱스). 캐시를 지우면 복구된다.

### #139 — 거짓 엣지 2 건에 스키마·추출기·해소기를 함께 움직인다; 더 작은 표면이 이미 계획에 있다
- 어떻게 실패하나: 저장소 실측으로 도착이 `mod` 인 파일 간 엣지 58 중 거짓은 **2**, 출발 파일은 둘(`crates/pal-query/src/lib.rs` · `crates/pal-query/tests/bench.rs`)이다. 첫 릴리스의 사용 대상은 TypeScript 저장소다. 계획 본선은 `SymbolNode` 에 가시성 칸을 더하는 것인데, `SymbolNode` 는 TS·Kotlin 추출기도 채우는 공통 타입이다. 그래서 회귀 표면이 세 언어로 번진다 — 가시성이 id·digest 에 섞이면 TS 결박도 뒤집힌다. 그런데 차선책(「해소기가 대상 `mod` 선언 원문에서 `pub` 유무를 읽는다 · 스키마 무변경」)이 같은 2 건을 닫는 더 작은 답이고, 계획은 그것을 **실패했을 때만** 쓰도록 순서를 거꾸로 두었다.
- 어디가 걸리나: 계획 3 · `intent.md ## 차선책` 첫 항 · `crates/pal-core/src/touch.rs:35` · `crates/pal-core/src/cross_file.rs:575-598`
- 획득: 추정 — 모집단 58/2 는 이슈 본문의 실측이다(이번에 다시 안 쟀다). TS 추출기가 새 칸을 어떻게 채울지는 구현 전이라 못 쟀다.
- 모집단: 저장소
- 유효성: 추정
- 해악도: 미관
- 대상: 계획대상
- 얼마나 아픈가: 그 자체로는 아무것도 안 깨진다. 표면이 커질수록 독립 리뷰 상한 2 안에서 볼 것이 는다.

### 릴리스를 공개로 내는데 저장소에 `LICENSE` 파일이 없고, 아카이브에 루트 README 가 안 들어간다
- 어떻게 실패하나: `Cargo.toml:21` 은 `license = "MIT OR Apache-2.0"`, 같은 파일 `:147` 은 *"라이선스는 MIT"*, GitHub 는 `licenseInfo: null` 이고 루트에 `LICENSE*` 가 없다. 목적 기여가 적은 「남이 받아 쓸 수 있는 릴리스」에서 사용 허락이 파일로 안 선다. 또 계획 6 이 루트 `README.md` 를 만들어도 `release.yml` 「담는다」는 `README.txt` 를 **따로 합성**하고 루트 README 를 안 복사한다. 게다가 그 단계 주석 *"저장소에 `README.md` 도 `LICENSE` 도 없다"* 는 이 변경으로 사실이 아니게 된다(낡음). 받은 아카이브의 안내는 `pal install`·`doctor`·`update`·`uninstall` 넷뿐이고 `narrative`·`touch` 가 없다.
- 어디가 걸리나: `Cargo.toml:21` · `:147` · `.github/workflows/release.yml` 「담는다」 here-doc · 계획 6
- 획득: 조회 — `gh repo view --json visibility,licenseInfo` · `ls LICENSE*` · `release.yml` 을 읽었다.
- 모집단: 저장소
- 유효성: 참
- 해악도: 거짓신호
- 대상: 계획대상
- 얼마나 아픈가: 파일 하나와 단계 몇 줄로 닫힌다. 태그 뒤라면 아카이브를 다시 내야 한다.

---

## 내가 기각한 것

| 제목 | 어떻게 실패하나 | 어디가 걸리나 | 획득 | 모집단 | 유효성 | 해악도 | 대상 |
|---|---|---|---|---|---|---|---|
| 버전 `0.1.0` 올림이 `--locked` 빌드를 태그 뒤에야 죽인다 | 사본에서 `version = "0.1.0"` 뒤 `cargo metadata --locked --offline` 이 **rc=101**(lock 갱신 필요)인 것은 참이다. 그러나 `ci.yml` 이 `cargo build --locked -p pal-cli` 를 두 잡에서 돌리고 계획은 CI 초록 뒤에 태그를 민다 — **태그 전에 잡힌다** | `Cargo.lock` · `.github/workflows/ci.yml` 의 「pal 을 세운다」 두 자리 | 조회 — 스크래치 `verbump/` 실행 · `ci.yml` grep | 저장소 | 거짓 | 실패 | 계획대상 |
| #156 이관 안내가 touch 의 좌표 표기와 달라 안 이어진다 | `touch` 와 `query` 는 둘 다 **이름 + `--pick`** 로 같은 방식으로 받는다(`--help` 둘). 짧은 해시를 이름 자리에 넣으면 둘 다 똑같이 「찾지 못했습니다」다. 받는 방식이 갈리는 자리는 없다. 남는 문제는 이관 타입이 지목을 못 싣는 것이고 그것은 위 시나리오가 진다 | `pal touch --help` · `pal query --help` | 조회 — 스크래치 `pal/` 에서 `touch fd4f91646c34` · `query symbol.callers fd4f91646c34` | 저장소 | 거짓 | 거짓신호 | 계획대상 |
| #77 속성과 선언 사이에 주석이 끼면 형제 속성을 놓친다 | 저장소 Rust 원문 전수(속성 줄 2,079)에서 「속성 줄 → 일반 주석 → 선언」 0 · 「속성 줄 → `///`」 0. 이 저장소에는 그 형태가 없다(남의 Rust 저장소는 안 쟀다) | `crates/pal-extract/src/parse.rs:590` `건너뛴다` | 조회 — `git ls-files 'crates/*.rs'` 에 awk 세 줄 창 | 저장소 | 거짓 | 금지역 | 계획대상 |
| #159 매니페스트가 생기면 상호운용 CI(다른 OS 가 받는다)가 빨개진다 | `interop-produce.sh` 는 설치 결과를 **커밋한 뒤** 넘기고, `interop-receive.sh` 의 `porcelain` 검사는 clone 직후와 install·update 뒤에 잰다. 매니페스트가 추적 파일로 오고 install 이 「있으면 안 건드린다」면 diff 0 이다. 빨개지는 자리는 이 스크립트가 아니라 `release.yml` 이다(위 시나리오) | `scripts/interop-produce.sh` · `scripts/interop-receive.sh:45` · `:71` | 조회 — 두 스크립트를 읽었다(안 돌렸다) | 저장소 | 거짓 | 실패 | 계획대상 |
| 매니페스트를 적는 순간 `ScopeSource` 가 `Declared` 가 되어 touch 화면의 「추정」 표기가 사라진다 | touch 화면은 범위 출처를 **원래 안 싣는다.** 매니페스트 없음과 `id="pal"` 두 상태의 화면에서 「추정」·「선언」을 grep 하면 무관한 설명 줄 하나만 걸린다. 잃을 표기가 없다(드러냄이 없는 문제는 #159 식별자 시나리오가 진다) | `crates/pal-cli/src/ledger.rs:273` · `pal touch` 화면 | 조회 — 스크래치 `pal/` 에서 A/B 두 상태 `touch TsProject` 출력 grep | 저장소 | 거짓 | 거짓신호 | 계획대상 |

새 범주: 다른 모집단(언어·코퍼스)에서 잰 실측을 이 언어의 사실로 옮겨 쓰기 — #77 ② 「죽은 가지」 판정이 TS·Kotlin 전수에서 왔는데 계획은 Rust 변경의 근거로 쓴다
