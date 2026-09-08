# 정(正) — 「해소 깊이」 한 칸을 어떤 값으로 다시 잠그며, 그것은 무슨 처분인가

> `pal-decision-proposer` 가 냈다. 회차 `2026-09-08-cross-file-references` · 정반합 판 1 · **정(正) 자리**.
> 받은 것: 설계문 전문(`dialectic/1-design.md`) · 물음 · `intent.md` · `premortem/r1-raw.md` 전문 · 사전 등록 명령표 · 저장소.
> **안 받은 것**: 대화 기록 · 메인의 사고 과정 · 메인의 입장 · 앞 라운드의 반론 · 다른 회차의 결론.
> **`findings.jsonl` 은 한 번도 열지 않았다.** 그 파일의 `처분`·`사전처분`·`조건변경` 칸은 물론이고 `요약` 칸도 안 읽었다. 발견의 본문은 전부 `premortem/r1-raw.md` 에서 인용한다.
> 이 호출에 **대화 기록은 섞여 오지 않았다.**

---

## 0. 무엇으로 쟀나 — 사전 등록 명령표의 네 칸

설계문 「결정론적인가」 절이 요구한 넷을 먼저 적는다.

| 칸 | 값 |
|---|---|
| ③ `pal --version` | `pal 0.0.0+6b6cb6ddb837` |
| ④ 측정 직전 `cargo build --release` 를 돌렸나 | **안 돌렸다.** 대신 아래를 확인했다 |
| 기준 커밋 | 설계문이 못 박은 `dab1743`. **HEAD 는 `2ebda8a`**(설계문을 실은 커밋) |
| 모집단 | `crates/` 아래 `*.rs` **전량 136 파일** — 시험·픽스처 포함 |

★ **바이너리가 HEAD 보다 두 커밋 뒤인 것이 이 측정에 걸리지 않는 근거.** 코드가 같다:

```
$ git diff --name-only 6b6cb6d HEAD -- crates schema xtask Cargo.toml Cargo.lock scripts corpus
(출력 없음)

$ git diff --stat dab1743 HEAD
 .../dialectic/1-design.md    | 221 +++++++++++++++++++++
 .../findings.jsonl           |   2 +-
 .../state.md                 |  22 +-
 3 files changed, 242 insertions(+), 3 deletions(-)
```

`6b6cb6d`(바이너리를 세운 커밋) · `dab1743`(기준 커밋) · `2ebda8a`(HEAD) 셋의 **코드·스키마·xtask·스크립트·코퍼스가 바이트로 같다.** 세 커밋 사이 차이는 `.palimpsest/rounds/` 안 기록뿐이다. 그래서 재빌드는 측정값을 바꿀 수 없고, 안 돌린 것이 이 측정의 흠이 아니다. `pal doctor` 의 `Snapshot` 도 `palimpsest@2ebda8a+worktree` 로 HEAD 를 본다.

### M1~M6 — 명령 문자열 그대로, 출력 그대로

```
$ rg -n '^\s*(pub )?use ' crates --glob '*.rs' | wc -l
     723
$ rg -n '^\s*(pub )?use (crate|super|self)::' crates --glob '*.rs' | wc -l
     266
$ rg -n '^\s*(pub )?use (std|core|alloc)::' crates --glob '*.rs' | wc -l
     191
$ rg -n '^\s*(pub )?use pal_' crates --glob '*.rs' | wc -l
      82
$ rg -n '^\s*pub use ' crates --glob '*.rs' | wc -l
      51
$ rg -n '^\s*(pub )?use .* as ' crates --glob '*.rs' | wc -l
       9
```

⚠ **설계문이 예고한 갈림이 그대로 났다.** `premortem/r1-raw.md:26` 은 *"`use` 717 줄 분류: `crate::`/`super::`/`self::` **273** · `std::`/`core::` **180** · `pal_*` **82** · `pub use` 선언 **51**"* 라고 적었다. 오늘 같은 문자열로 재면 **723 · 266 · 191 · 82 · 51** 이다. `pal_*` 와 `pub use` 만 맞고 나머지 셋이 갈린다. 별칭도 갈린다 — 사전부검 `:50` 은 *"저장소 안 `use … as …` **7 건**"* 인데 M6 은 **9** 다.

**어느 쪽이 옳은지 이 초안은 판정하지 않는다.** 아래 모든 수는 위 여섯 명령과, 그 아래 새로 등록하는 M11~M16 의 명령으로만 잰다.

### M7 — 참조 엣지 총계

```
$ ./target/release/pal export --format cypher | grep -c REFERENCES
4258
$ ./target/release/pal export --format cypher | grep -c ':REFERENCES]'
4258
```

★ `intent.md:74` 는 *"참조 엣지 **4,259** 행"* 이라고 적는다. `observations/red.md:24` 는 같은 수에 *"(헤더 1 행 포함)"* 을 달았다. 오늘은 헤더를 세는 `grep -c REFERENCES` 와 엣지만 세는 `grep -c ':REFERENCES]'` 가 **둘 다 4258** 이다. 즉 `intent.md:74` 의 4,259 는 오늘 재현되지 않는다. **한 건 차이이고 이 판정의 어느 칸도 뒤집지 않는다** — 다만 잠긴 의도에 실린 수가 오늘 실측과 다르다는 사실은 `premortem/r1-raw.md:108`(PM1-09)가 이름 붙인 형태와 같은 자리다.

### M8 — 한 파일의 해소 갈래 분포 (`crates/pal-core/src/projection.rs`)

```
$ ./target/release/pal symbols crates/pal-core/src/projection.rs --graph --json
총 refs 478
  bound:not_a_symbol   336
  bound:symbol          87
  outside_file          55
outside_file 이름 분포: assert_eq 24 · Vec 7 · Some 4 · assert 4 · vec 4 · Self 2 ·
                        Option 2 · _ 2 · Result 1 · Ok 1 · None 1 · serde_json 1 · u32 1 · std 1
```

**`premortem/r1-raw.md:14`(PM1-01)와 한 건도 안 갈린다** — 336 · 87 · 55 와 그 55 의 이름 전량이 같다. PM1-01 은 오늘 재현된다.

### M9 — 결박 건수와 신선도

```
$ ./target/release/pal query binding.status --json
결박 30 · fresh 26 · stale 4 · radius 전량 "symbol"
$ grep -c '' .palimpsest/intent/bindings.jsonl
31   (첫 줄이 {"kind":"header","schema_version":2} 이므로 레코드 30)
```

★ **`premortem/r1-raw.md:120`(PM1-10, 「JSONL 이 5 건 뒤처져 있다」)는 오늘 닫혀 있다.** 산 저장소 30 과 커밋된 JSONL 30 의 차집합이 **양쪽 다 공집합**이다(`git log -- .palimpsest/intent/bindings.jsonl` 의 최신 커밋이 `dab1743`). PM1-09 의 「25 가 아니라 30」은 여전히 참이다 — `intent.md:46` 은 이미 30 으로 고쳐져 있다.

### M10 — 판정 축 여덟

```
$ ./target/release/pal doctor
  1  ok    검사 30 · 표본 밖 0 · 위반 0     담지 못하는 자리: … REFERENCES(F07) …
  2  ok    검사 467 · 표본 밖 2798 · 위반 0  담지 못하는 자리: … File(F07) · UnresolvedRef(F08) · REFERENCES(F07) …
  3  ok    검사 463 · 표본 밖 2772 · 위반 0  담지 못하는 자리: … File(F07) · UnresolvedRef(F08) …
  4  (모집단이 없습니다)
  5  (모집단이 없습니다)   담지 못하는 자리: INTRODUCED_BY(F05)
  6  (모집단이 없습니다)
  7  (모집단이 없습니다)
  8  (모집단이 없습니다)
  Snapshot  palimpsest@2ebda8a+worktree
  능력      ledger.snapshot · symbol.resolve · graph.doctor · 미구축 F05 · F08 · F15 · F17 · F20
```

`intent.md:77-79` 의 관측 넷째와 `premortem/r1-raw.md:74, 86`(PM1-06·PM1-07)가 그대로 재현된다.

---

## 1. 새로 등록하는 명령 — 왜 모집단을 바꿨는지 함께 적는다

설계문 `:74` 이 *"다른 모집단으로 잰 수를 쓰려면 그 명령을 새로 적고 왜 바꿨는지를 함께 적는다"* 로 허락한 자리다.

**왜 바꾸나.** M1~M6 은 **`use` 줄**을 센다. 그런데 이 판이 물어야 하는 것은 *"이 값으로 잠그면 산출이 0 이 아닌가"* 이고(설계문 「끝 조건 ④」), 그것은 **줄**이 아니라 **그 임포트를 실제로 가리키는 참조 자리**의 수다. 설계문 `:78` 이 못 박은 대로 「모집단 N 건」은 「엣지 N 건」이 아니다. 그래서 아래 넷을 더 등록한다.

| # | 무엇을 재나 |
|---|---|
| **M11** | `use` 의 뿌리 세그먼트가 **같은 파일이 `mod <뿌리>;` 로 선언한 모듈**인 줄 — Rust 2018 「균일 경로」 |
| **M14** | `use crate::`/`super::`/`self::` 항목 중 대상이 **최상위 제한 가시성**(`pub(crate)` 등)인 것 |
| **M16** | 참조가 **`use` 문 안에서 선언된 바인딩**에 걸리는 건수. 귀속은 `ScopeBinding.declared_at` 이 그 `use` 문의 **바이트 범위** 안에 드는가로 판정한다 |
| **M17** | 격리 사본 재현 — 심은 파일과 돌린 명령을 그대로 |

⚠ **M16 의 귀속 방법을 명시한다.** 이름 매칭이 아니라 `declared_at` 바이트 범위 포함이다. 이름 매칭을 쓰면 임포트 이름과 같은 이름의 **지역 변수**가 섞인다(`projection.rs` 한 파일에서만 `bound:not_a_symbol` 이 336 건인데 그 대부분이 지역이다). 그리고 `declared_at` 은 **바이트** 오프셋이므로 소스를 문자로 읽으면 한글 주석이 있는 파일에서 전부 어긋난다 — 처음에 그렇게 재서 틀린 표를 얻었고, 바이트로 다시 재서 고쳤다.

### M16 — 갈래별 실측 (모집단: `crates/` 아래 `*.rs` 136 파일 · 시험·픽스처 포함)

| 갈래 | `use` 줄 | 그 임포트를 가리키는 참조 | 파일 | 저장소 안 다른 파일을 가리키나 |
|---|---:|---:|---:|---|
| **A1** `crate::` | 174 | **1110** | 46 | **예** |
| **A2** `super::*` (glob) | 49 | 0 | 0 | 이름을 안 들여온다 |
| **A3a** 줄머리 `super::<항목>` | 21 | **143** | 11 | **예** — 부모 모듈이 다른 파일이다 |
| **A3b** 들여쓴 `super::<항목>` | 22 | 251 | 16 | 대개 아니다 — 인라인 `mod tests` |
| **B** `std::`/`core::`/`alloc::` | 191 | 1045 | 87 | 아니다 — 저장소 밖 |
| **C0** 형제 크레이트·평평 (`pal_core::X`) | 80 | **1708** | 50 | **예** — 단 `pub use` 를 지나야 한다 |
| **C1** 형제 크레이트·깊은 경로 | 2 | **2** | 2 | **예** — 재수출이 필요 없다 |
| **D** 균일 경로 형제 모듈 | 26 | **313** | 24 | **예** — 재수출이 필요 없다 |
| **E** `pub use` 재수출 **선언** | 51 | 3 | 1 | — |
| **F** 외부 크레이트 | 107 | 683 | 46 | 아니다 — 저장소 밖 |
| **합계** | **723** | 5258 | | |

줄 합계가 M1 의 723 과 정확히 같고 갈래가 서로 겹치지 않는다. A1+A2+A3a+A3b = 266 = M2. B = 191 = M3. C0+C1 = 82 = M4. E = 51 = M5.

---

## 2. 근거 — 좌표와 함께

### 2.1 ★ **잠근 범위의 모집단은 0 이 아니다.** `premortem/r1-raw.md:24`(PM1-02)의 머리 주장이 오늘 실측에서 무너진다

PM1-02 는 이렇게 적었다:

> *"Rust 2018 이후 크레이트 **안**의 임포트는 문법상 반드시 `crate::`/`super::`/`self::` 로 시작하고(범위 밖), 그렇지 않은 경로는 전부 **외부 크레이트**다."* — `premortem/r1-raw.md:24`

**그 문장이 거짓이다.** Rust 2018 의 「균일 경로(uniform paths)」는 `use` 경로가 **같은 모듈이 선언한 항목 이름으로 시작하는 것**을 허용한다. 이 저장소가 그것을 쓴다.

**M11 — 균일 경로 형제 모듈 임포트 26 줄** (좌표 전량):

- `crates/pal-cli/src/install.rs:48` `use inside::{Rel, Root};`
- `crates/pal-cli/src/install.rs:49` `use layout::{ AGENT_KEY, … SETTINGS };` (17 항목·여러 줄)
- `crates/pal-cli/src/install.rs:54` `use manifest::{BlockEntry, FileEntry, Manifest, Origin, Roots, SettingsEntry, 자리들};`
- 그리고 `crates/pal-cli/tests/` 의 23 파일이 `use common::{…}` — 예: `crates/pal-cli/tests/rust_references.rs:16` `use common::{git, pal};` · `crates/pal-cli/tests/install.rs:30` `use common::{PAL, git, path_앞에, 상대_경로, 해시};` · `crates/pal-cli/tests/prune_boundary.rs:25` `use common::{바이트_전부, 저장소, 캐시_엔트리_수, pal};`

**대상이 실재하고 최상위 `pub` 이다:**
- `crates/pal-cli/src/install.rs:24-38` 이 `mod blocks; … mod winpath;` 열다섯을 선언한다. 실물은 `crates/pal-cli/src/install/inside.rs` · `layout.rs` · `manifest.rs`.
- `crates/pal-cli/src/install/inside.rs:28` `pub struct Root(PathBuf);` · `:135` `pub struct Rel(String);` — 둘 다 최상위 `pub`.
- `crates/pal-cli/src/install/layout.rs` 최상위 `pub` 26 개 · 제한 가시성 0 개. `manifest.rs` 최상위 `pub` 17 개 · 제한 가시성 0 개.
- `crates/pal-cli/tests/common/mod.rs` 가 실재하고, 임포트되는 이름 전부가 최상위 `pub` 이다 — `:24 pub const PAL` · `:62 pub fn 해시` · `:73 pub fn 상대_경로` · `:82 pub fn path_앞에` · `:93 pub fn 저장소` · `:122 pub fn git` · `:128 pub fn pal` · `:145 pub fn 바이트_전부` · `:168 pub fn 캐시_엔트리_수`.

**그리고 `ImportSet` 이 그 모듈 지정자를 이미 싣고 있다:**

```
$ ./target/release/pal symbols crates/pal-cli/tests/rust_references.rs --graph --json
imports: {"present": {"modules": ["common", "pal_core", "pal_store", "std::path"]}}

$ ./target/release/pal symbols crates/pal-cli/src/install.rs --graph --json
imports: {"present": {"modules": ["anyhow","doctor","inside","layout","manifest",
                                  "serde_json","std::collections","std::os::unix::fs","std::path"]}}
```

**한 갈래가 더 있다 — C1.** `crates/pal-cli/src/round/status.rs:6` 과 `crates/pal-cli/src/round/mod.rs:13` 의 `use pal_intent::round_condition::ConditionsReport;` 는 형제 크레이트의 **깊은 경로**라 재수출을 안 지난다. 대상은 `crates/pal-intent/src/round_condition.rs:81` `pub struct ConditionsReport {` 다.

**그러므로 ㉮(유지)의 모집단은 M16 기준 D + C1 = `use` 28 줄 · 참조 315 자리 · 25 파일이다. 0 이 아니다.**

### 2.2 그러나 PM1-02 의 **부분** 주장은 오늘도 참이다 — `pal_core::X` 는 재수출을 지난다

C0 의 80 줄(참조 1708)은 `use pal_core::{Capable, SymbolId, …}` 형태다. 이것이 정의에 닿으려면 `crates/pal-core/src/lib.rs:48-133` 의 `pub use` 51 줄을 지나야 한다. **그리고 `pub use` 는 심볼을 만들지 않는다** — M17 격리 사본 재현:

```
심은 파일  src/a.rs   : pub fn helper(x: u32) -> u32 { x + 1 }
           src/lib.rs : mod a;
                        pub use a::helper;
$ pal symbols src/lib.rs --graph --json
symbols: [{"name": "a", "kind": "module"}]
exports: {"present": {"names": ["helper"], "star_from": [], "has_default": false}}
```

`helper` 는 `exports.names` 에 문자열로만 남고 **`SymbolId` 를 가진 심볼 노드가 아니다.** `REFERENCES` 는 `SymbolId → SymbolId` 다(`crates/pal-store/src/projection.rs:258-259`). 그래서 `pal_core::Capable` 은 **재수출 추적 없이는 걸 대상이 없다.** PM1-02 의 이 절반은 선다.

### 2.3 M17 — 균일 경로 형제 모듈의 격리 사본 재현 (심은 파일과 명령 그대로)

```
심은 파일  src/a.rs    : pub fn helper(x: u32) -> u32 { x + 1 }
           src/main.rs : mod a;
                         use a::helper;
                         fn main() { println!("{}", helper(1)); }
           Cargo.toml  : [package] name="iso" version="0.0.0" edition="2024"
           git init → add -A → commit

$ pal symbols src/main.rs --graph --json
imports: {"present": {"modules": ["a"]}}
refs:    [{"name":"a",      "resolved":{"bound":{"scope":0,"binding":0}}},
          {"name":"main",   "resolved":{"bound":{"scope":0,"binding":3}}},
          {"name":"println","resolved":"outside_file"},
          {"name":"helper", "resolved":{"bound":{"scope":0,"binding":1}}}]
bindings[1] = {"name":"helper","declared_at":14,"symbol":"not_a_symbol"}

$ pal symbols src/a.rs --graph --json
exports: {"present": {"names": ["helper"], …}}

$ pal touch helper --json
answer.symbol = { path: "src/a.rs", name: "helper", kind: "function" }
```

★ **착지점은 여기서도 PM1-01 이다.** `helper` 참조는 `outside_file` 이 아니라 `bound{not_a_symbol}` 로 간다 — `premortem/r1-raw.md:12` 가 적은 그대로다. **이것은 ㉮·㉯·㉰·㉱ 어느 값에서도 똑같이 걸린다.** 그래서 선택지를 가르는 근거가 아니고, 설계문 `:207` 이 이미 이송으로 보낸 자리다.

### 2.4 A1(`crate::`) 이 오늘 이미 `ImportSet.modules` 에 실려 있다

```
$ ./target/release/pal symbols crates/pal-core/src/projection.rs --graph --json
imports: {"present": {"modules": ["crate::capable","crate::coord","crate::ledger","crate::repo",
                                  "crate::scope","crate::slot","crate::symbol","crate::touch",
                                  "serde","super"]}}
```

`crates/pal-core/src/projection.rs:19-26` 의 여덟 줄이 그대로 실린다. `premortem/r1-raw.md:36, 38`(PM1-03)이 *"`crate::a` 는 이미 `ImportSet.modules` 에 통째로 실려 있다"* 라고 적은 것이 오늘 재현된다.

### 2.5 A1 을 여는 것과 `pal touch` 의 빈 칸이 같은 자리다

`premortem/r1-raw.md:26` 의 실측이 오늘도 산다 — *"`pal query symbol.callers Capable` → `{"outcome":"symbols","symbols":[]}` … `Capable` 은 `projection.rs` 한 파일에서만 14 번 참조되는 허브인데도 그렇다 — 그리고 그 14 번 전부 `use crate::capable::Capable` 을 지난다."*

즉 **㉮ 로 잠근 채로는 `pal touch Capable` 이 「호출자 0」인 채로 남는다.** 그것이 `intent.md:23-25` 가 목적 기여로 적은 자리이고 `docs/plan/03-shortest-path.md:123` 의 2 단계 합격선이다.

### 2.6 M14 — 기각된 `PM1-20` 이 A1 에서 되살아나는 크기는 **6 항목**이다

`premortem/r1-raw.md:210` 은 `ExportSet` 이 최상위 `pub` 만 담는 것을 기각하며 스스로 *"지금 범위에서는 발화 불가. 범위를 `crate::` 로 옮기면 그때 다시 재야 한다"* 라고 적었다. 그래서 다시 쟀다.

저장소 전체에서 **최상위** 제한 가시성 항목은 18 종 19 줄이다 — `Builder` · `KOTLIN` · `RUST` · `ScopeRules` · `Scoped` · `TYPESCRIPT` · `TypeScriptRules` · `build` · `disable` · `disable_if_supported` · `hex32` · `nodes_of` · `private_file` · `repo_name` · `repository_root_identity` · `store_location` · `valid_terminal_document` · `줄번호를_뗀다`.

그중 `use crate::`/`super::`/`self::` 로 실제 임포트되는 것은 **6 항목 4 종**뿐이다 — `Builder` 2 · `ScopeRules` 2 · `Scoped` 1 · `TypeScriptRules` 1. A1+A3a 의 임포트 항목 319 개 대비 **1.9%** 다. `premortem/r1-raw.md:210` 이 「대량이 아니다」로 기각한 판단은 `crate::` 를 열어도 유지된다 — 다만 **발화는 한다.**

`ExportSet` 문서(`crates/pal-extract/src/rust.rs:411-419` 부근의 주석)가 *"`pub(crate)`·`pub(super)`·`pub(in …)` 과 중첩 `pub` 은 **안 담는다**"* 로 그 자리를 이미 잠가 두었다.

---

## 3. 선택지 표 — 일곱 칸, 칸마다 여섯

> 설계문 `:107` — *"빈 칸이 있는 선택지는 살아남은 것으로 안 센다."*

### ㉮ **유지** — `use` 직접 임포트만. 잠근 그대로 간다

| 칸 | 값 |
|---|---|
| **1 모집단** | M16 의 **D + C1** = `use` **28 줄** · 참조 **315 자리** · **25 파일**. 명령·출력은 §1 표와 §2.1 좌표 |
| **2 비-0 증인** | **선다.** 합격선: 「이 저장소에서 `pal touch pal` 과 `pal touch Root` 가 각각 **파일 간 호출자 ≥ 1**」 + 「파일 간 `REFERENCES` 엣지 ≥ **100**」. 음성 대조: **그 하한을 0 으로 두면 산출이 0 인 구현이 통과한다** — `crates/pal-cli/tests/rust_references.rs:12-13` 과 `crates/pal-cli/tests/stitching.rs:12-15` 가 이미 쓰는 문형이다 |
| **3 다른 여섯 칸** | **안 움직인다.** 범위·언어·등급·`ImportSet`·증인·멈춤 그대로 |
| **4 살리는/죽이는 발견** | 살린다: PM1-03(비용 대 값). **315 / 3276 = 9.6%** 를 위해 `EXTRACTOR_REV` 승급 + 1 층 캐시 전량 무효를 치른다. PM1-16(2 단계 합격선을 조용히 좁힌다) — §2.5 가 그 형태다. 안 죽인다: PM1-04·05·06·07·08·12·13·15 전부 그대로 걸린다(전 선택지 공통). 잠재운다: **없다** |
| **5 기각 넷 재측정** | PM1-17 유지(픽스처가 단일 `lib.rs` — `crates/pal-cli/tests/rust_references.rs:57-67` 에서 확인) · PM1-18 유지 · PM1-19 유지 · **PM1-20 발화 안 함**(D+C1 대상이 전부 최상위 `pub`) |
| **6 처분 라벨** | **처분 없음** — 재잠금이 없다. 유일하게 규약 §5 를 안 지나는 칸 |

### ㉯ `crate::`·`super::`·`self::` 경로 해소를 넣는다

| 칸 | 값 |
|---|---|
| **1 모집단** | M16 의 **A1 + A3a** 를 더한다 = `use` **+195 줄** · 참조 **+1253 자리** · **+46/11 파일**. 합계 **223 줄 · 1568 참조** (A3b 251 은 인라인 `mod tests` 라 보수적으로 뺐다) |
| **2 비-0 증인** | **선다, 그리고 ㉮ 보다 강하게 선다.** 합격선: 「`pal query symbol.callers Capable` 이 **≥ 1**」 + 「파일 간 `REFERENCES` 엣지 ≥ **1,000**」 + 「기여한 `.rs` 파일 ≥ **50**」. 음성 대조: **하한을 0 으로 두면 §2.5 의 오늘 산출(`symbols: []`)이 그대로 통과한다** |
| **3 다른 여섯 칸** | **안 움직인다.** `intent.md:62` 의 `## 범위 밖` 한 줄에서 앞 절반만 뺀다. 언어(Rust) · 등급(exact) · 임포트 항목 이름 자리(`ImportSet`) · 증인(자기 저장소) · 멈춤 자리 그대로. 반증표 #2 의 「바뀐 칸 하나」를 지킨다 |
| **4 살리는/죽이는 발견** | **죽인다: PM1-03.** 값 내는 쪽이 1568 참조로 서므로 *"만들 필요가 없는 축에 가장 비싼 값"* 이 성립을 잃는다 — PM1-03 자신이 *"범위를 `crate::`/`super::` 로 바꾸면 모집단이 곧바로 273 개 `use` 로 선다"* 라고 적은 그 길이다(`premortem/r1-raw.md:36`). **약화한다: PM1-16** — 2 단계의 「호출자 N」 줄이 실제로 값을 돌려주므로 조용한 축소가 아니게 된다(`candidate` 절반은 그래도 남는다). **그대로 산다:** PM1-04(별칭 9 · 중첩 목록) · PM1-05(`ScopeBinding` 출처) · PM1-06(등급 칸) · PM1-07(`doctor` 배선) · PM1-08(`UnresolvedRef` 계약) · PM1-12(F07 을 가르는 것) · PM1-13(TS 빈 벡터) · PM1-15(postcard 판). **새로 요구한다:** 모듈 지정자 → 파일 사상(`crate::capable` → `crates/pal-core/src/capable.rs`). 설계문 `:122` 대로 **요구한다는 사실만** 적고 방법은 안 적는다 |
| **5 기각 넷 재측정** | PM1-17 유지 · PM1-18 유지(`crates/pal-core/src/coord.rs:106-130` 의 성분에 추출기 판이 없다) · PM1-19 유지 · **★ PM1-20 뒤집힌다** — 다만 크기는 M14 로 **6 항목 / 319(1.9%)** 다. 그 6 이 거짓 `UnresolvedRef` 가 되거나 거짓 엣지가 된다 |
| **6 처분 라벨** | §4 의 자로 계산한다 |

### ㉰ `pub use` 재수출 추적을 넣는다

| 칸 | 값 |
|---|---|
| **1 모집단** | M16 의 **C0** = `use` **+80 줄** · 참조 **+1708 자리** · **+50 파일**. 합계 **108 줄 · 2023 참조** |
| **2 비-0 증인** | **선다.** 합격선: 「`pal query symbol.callers RepoPath` ≥ 1」 + 「파일 간 엣지 ≥ **1,500**」. 음성 대조 같은 문형 |
| **3 다른 여섯 칸** | ⚠ **한 칸을 넘는다.** §2.2 에서 확인한 대로 `pub use` 는 심볼을 안 만들고 `exports.names` 문자열만 남긴다. 재수출 **원본**(`coord::SymbolId`)을 어딘가에 실어야 하는데 그 자리는 `ImportSet` 이 아니라 `ExportSet` 이다 — 그러면 `intent.md:16` 의 「임포트가 항목 이름을 실게 하는 자리 = `ImportSet` 에 이름」 칸이 함께 움직인다. 반증표 #2 에 걸린다 |
| **4 살리는/죽이는 발견** | 죽인다: PM1-03. **가장 세게 살린다: PM1-04** — `pub use` 51 줄이 **309 항목**을 담는 최대 중첩 목록이고, `premortem/r1-raw.md:48` 의 *"중첩 목록은 실모듈이 소실된다"* 가 여기서 최대로 발화한다. 새로 요구: 재수출 사슬을 따라가는 이행 해소(2 홉 이상) |
| **5 기각 넷 재측정** | PM1-17·18·19 유지. PM1-20 **발화 안 함** — 재수출 대상은 정의상 `pub` 이다 |
| **6 처분 라벨** | §4 로 계산하되, **3 번 칸이 한 칸을 넘으므로 이 값은 이 판의 물음 밖으로 새어 나간다** |

### ㉱ ㉯ 와 ㉰ 둘 다

| 칸 | 값 |
|---|---|
| **1 모집단** | A1 + A3a + C0 + D + C1 = `use` **303 줄** · 참조 **3276 자리** |
| **2 비-0 증인** | 선다 — 위 둘의 합. 하한 「파일 간 엣지 ≥ 3,000」 |
| **3 다른 여섯 칸** | ㉰ 와 같은 이유로 **넘는다** |
| **4 살리는/죽이는 발견** | ㉯ 와 ㉰ 의 합집합. PM1-04·05 가 동시에 최대로 발화하고, `premortem/r1-raw.md:55` 의 *"`EXTRACTOR_REV` 를 **두 번** 올리거나 첫 판을 버려야 한다"* 가 여기서 가장 가깝다 |
| **5 기각 넷 재측정** | PM1-20 뒤집힌다(6 항목). 나머지 셋 유지 |
| **6 처분 라벨** | §4 로 계산 |

### ㉲ 해소는 안 넓히고 **못 푼 참조 표시(F08) 쪽만** 남긴다

| 칸 | 값 |
|---|---|
| **1 모집단** | 해소 모집단 **0 으로 내린다**(의도적으로). `UnresolvedRef` 모집단은 M8 의 `outside_file` 갈래 — `projection.rs` 한 파일에서 55, 저장소 전체는 안 쟀다 |
| **2 비-0 증인** | 「`UnresolvedRef` 노드 ≥ N」으로는 선다. **그러나 「파일 간 엣지」로는 원리상 0 이고, `PM1-11` 이 겨눈 형태가 정확히 그것이다** |
| **3 다른 여섯 칸** | ⚠ **`intent.md:13` 의 「범위 = 경계 + 못 푼 참조」 칸을 직접 움직인다.** 「해소 깊이」 한 칸이 아니다. 반증표 #2 위반 |
| **4 살리는/죽이는 발견** | 살린다: PM1-08(`attempts` 원천 없음) · PM1-11 · PM1-16 을 최대로. 죽인다: PM1-04·05(임포트 짝이 필요 없어진다) |
| **5 기각 넷 재측정** | PM1-17 **주의** — `crates/pal-cli/tests/rust_references.rs:83` 의 `assert!(c.unresolved > 0)` 가 이 값에서 **더 안전해진다**. PM1-20 발화 안 함 |
| **6 처분 라벨** | 재는 의도의 양이 315 → 0 으로 **줄어든다** → **축소**. `intent.md:53-55` 의 `## 차선책` 이 비어 있으므로 **승격**이다 |

### ㉳ 값은 그대로 두고 **증인을 바꾼다**

| 칸 | 값 |
|---|---|
| **1 모집단** | 이 저장소에서는 ㉮ 와 같다(315). 다른 저장소는 **안 쟀다** |
| **2 비-0 증인** | **못 채운다.** 다른 모집단을 안 쟀으므로 증인이 0 이 아니라는 근거가 없다. 설계문 `:107`·`:177` 에 따라 **살아남은 것으로 안 센다** |
| **3 다른 여섯 칸** | ⚠ `intent.md:18` 의 「무엇이 도달을 증언하나 = 자기 저장소 + 정확도 표본」 칸을 직접 움직인다. 「해소 깊이」가 아니다 |
| **4 살리는/죽이는 발견** | PM1-03 을 안 건드린다(비용은 그대로, 이 저장소의 값도 그대로) |
| **5 기각 넷 재측정** | 안 움직인다 |
| **6 처분 라벨** | 해당 없음 — 다른 칸의 물음이다 |

★ 그리고 ㉳ 의 동기는 「모집단이 0 이라 자기 저장소가 증인이 못 된다」였는데 **§2.1 이 그 전제를 무너뜨렸다.** 증인은 315 자리를 갖고 있다.

### ㉴ **철회** — 회차를 닫고 순서표의 다른 자리를 연다

| 칸 | 값 |
|---|---|
| **1 모집단** | 해당 없음 |
| **2 비-0 증인** | **원리상 못 채운다**(산출이 없다) |
| **3 다른 여섯 칸** | 일곱 칸 전부를 접는다 |
| **4 살리는/죽이는 발견** | 열여섯 전부를 미결로 남긴다. PM1-10(데이터 손실)은 이미 닫혔으므로 잃는 것은 없다 |
| **5 기각 넷 재측정** | 해당 없음 |
| **6 처분 라벨** | 처분이 아니라 `AGENTS.md` 의 **나가는 문 셋** 중 「철회(folded)」다. 설계문 `:217` 대로 **판정이 아니라 승격문이 된다** |

★ **철회의 근거가 오늘 약하다.** 철회를 부르던 것은 「모집단 0」이었고 그것이 315 로 반증됐다. 그리고 `docs/plan/03-shortest-path.md:123` 이 이 자리를 *"여기가 제품이다"* 로 못 박았고 `:91` 이 *"남은 것은 §4 의 2 단계"* 로 적는다.

---

## 4. 자 — 값 → 자 → 라벨

⚠ 순서를 지킨다. 값을 먼저 골랐고, 그다음에 자를 대고, 라벨은 그 계산의 결과다.

### 4.1 고른 값

**㉯ — `crate::`·`super::`·`self::` 경로 해소를 넣는다.**

여섯 칸이 다 찼고, 3 번 칸(다른 여섯 칸)이 **안 움직이는 유일한 재잠금**이며, 2 번 칸의 비-0 증인이 가장 크다.

### 4.2 「의도의 양」을 무엇으로 세는가

**이 회차가 산출할 「파일 간 참조 해소 자리」의 수** — 즉 M16 의 「그 임포트를 가리키는 참조」 중 **저장소 안 다른 파일**을 가리키는 갈래의 합.

**왜 그것인가.** `intent.md:23-25` 가 이 회차의 목적 기여를 *"「여기를 바꾸면 무엇이 깨지나」에 답하는 **역방향 색인**이 지금 비어 있다"* 로 적는다. 역방향 색인의 크기는 곧 파일 간 참조 엣지의 수다. 그리고 `docs/plan/03-shortest-path.md:123` 의 2 단계 합격선(*"「호출자 7 (exact 5 · candidate 2)」 줄 … 이 자리다"*)이 재는 것도 같은 양이다. 후보였던 다른 둘을 안 고른 까닭도 적는다 — 「`UnresolvedRef` 표시 건수」는 ㉲ 를 빼면 모든 선택지에서 같아 변별하지 못하고, 「`03-shortest-path.md` §2 표의 우선순위 1 행 수」는 **행 단위라 눈금이 두 칸(호출자·모르는 것)뿐**이어서 늘고 주는 것을 잴 해상도가 없다.

### 4.3 자를 댄 계산

| | 원 의도 (㉮ 유지) | 새 값 (㉯) |
|---|---:|---:|
| 파일 간 해소 자리 | **315** | **1568** |
| 기여하는 `.rs` 파일 | 25 | 약 71 |
| 기여하는 크레이트 | **1 개**(`pal-cli`) | **7 개 전부** |
| `pal query symbol.callers Capable` | `[]` (오늘 그대로) | ≥ 1 |

**늘었다. 315 → 1568, 4.98 배.** 줄어든 칸이 없다.

규약의 자(`.claude/skills/round/SKILL.md:329`): *"고친 뒤 그 검사가 재는 의도의 양이 **늘었으면 정정, 줄었으면 완화**다."* 늘었으므로 **축소가 아니고 완화도 아니다.**

**전환인가.** `SKILL.md:325` — 전환은 *"의도나 프로젝트 방향을 **뒤엎는다**"*. 뒤엎지 않는다:
- 의도의 제목(`intent.md:1` 「파일 경계를 넘는 참조 해소」)이 그대로다.
- 목적 기여(`intent.md:23-25` 역방향 색인)가 그대로이고, 새 값은 그것을 **더 채운다**.
- 방향(`docs/plan/03-shortest-path.md:123` 의 2 단계)이 그대로다.
- 일곱 칸 중 여섯이 안 움직인다.

**정정인가 확대인가.** `SKILL.md:322-323`:
- **정정** = *"**완수 조건**이 의도를 제대로 못 잰다"* — 고치는 대상이 **재는 장치**다. 그런데 이 회차의 완수 조건은 **아직 안 쓰였다**(`intent.md:49-51` 「(사전부검과 조건 설계 평가 뒤에 쓴다)」). 재는 장치가 없으니 「그 장치가 못 잰다」가 성립할 자리가 없다.
- **확대** = *"착수 때 못 본 것이 나왔고 **원 의도 충족에 필요**하다 → 편입해서 이 회차에서 닫는다"*. 착수 때 못 본 것이 나왔다 — **균일 경로**(§2.1)와 **A1 의 1110 참조**(§1 표)다. 그리고 원 의도(역방향 색인)를 채우려면 필요하다(§2.5: `Capable` 이 오늘 `[]` 다). 고치는 대상은 **범위**이고, 실제로 `intent.md:62` 의 `## 범위 밖` 한 줄에서 앞 절반을 **범위 안으로 편입한다**.

**그러므로 확대다.**

---

## 5. `docs/plan/03-shortest-path.md` §4 2 단계의 *"여기가 제품이다"* 와의 관계

> 설계문 반증표 #6 이 이 문단을 요구한다.

그 칸의 합격선 전문은 이렇다 — *"§2 의 표에서 우선순위 1 행이 전부 값을 돌려준다 — **여기가 제품이다**"*(`docs/plan/03-shortest-path.md:123`). §2 표에서 오늘 값을 못 돌려주는 우선순위 1 행은 둘이다 — 「**호출자 · 피호출자**」(같은 문서 `:64`)와 「**모르는 것 (`UnresolvedRef`)**」(`:65`).

**㉮ 로 잠근 채 이 회차를 닫으면 「호출자 · 피호출자」 행은 값을 돌려준다고 적히지만, 그 값은 `pal-cli` 한 크레이트의 시험 도우미와 `install/` 하위 모듈에서만 나온다.** `pal touch Capable` · `pal touch SymbolId` · `pal touch RepoPath` 는 여전히 「호출자 0」이다(§2.5 의 실측). 그러면 그 행이 **초록인 채로 거짓**이 되고, 다음 회차가 *"2 단계는 닫혔다"* 를 입력으로 받는다 — `premortem/r1-raw.md:192`(PM1-16)이 이름 붙인 형태 그대로다.

**㉯ 로 다시 잠그면 그 행이 7 개 크레이트 전부에서 값을 돌려준다.** 다만 **㉯ 로도 그 칸은 완전히 닫히지 않는다.** 두 자리가 남는다:
1. `:123` 의 문면이 「호출자 7 (**exact 5 · candidate 2**)」인데 `intent.md:15` 가 `candidate` 를 안 싣는다 — PM1-16 의 절반은 ㉯ 로도 안 풀린다. 그것은 「엣지 등급」 칸의 물음이고 이 판 밖이다.
2. `use pal_core::X` 의 1708 참조(C0)는 ㉯ 로도 안 풀린다. `pal touch Capable` 은 `pal-core` **안**의 호출자를 보여 주지만 `pal-cli`·`pal-store` 에서 온 호출자는 못 보여 준다.

**그래서 이 초안은 「이 회차가 2 단계를 닫는다」를 주장하지 않는다.** ㉯ 는 그 칸을 **크게 채우되 다 채우지는 않는다**, 그리고 **그 사실을 회차 보고에 적는 것까지가 이 값의 조건**이다.

---

## 6. 이 초안이 틀렸다면 무엇이 먼저 드러나나

> 반(反)에게 주는 손잡이다. 약한 자리부터 적는다.

1. ★★ **가장 약한 자리 — 「확대」가 소유자가 인터뷰에서 잠근 답을 뒤집는다는 점.** `intent.md:16` 은 *"해소를 어디까지 따라가나? → **use 직접 임포트만**"* 이고, 이것은 사전부검이 아니라 **소유자가 3 라운드 인터뷰에서 고른 값**이다(`intent.md:11` 「인터뷰에서 잠근 것 넷」). 확대는 승격을 안 거치고 이 회차에서 닫는 길이므로, **소유자의 답을 소유자에게 안 물어보고 넓히는 형태**가 된다. `SKILL.md:239` 는 *"소유자가 잠긴 의도를 승인한다 … 승격이 일어나면 다시 받는다"* 로 적는데, 확대는 승격이 아니라 재승인 트리거가 아니다. **여기가 「확대」와 「전환」이 갈리는 진짜 자리이고, 이 초안은 그것을 「방향을 안 뒤엎는다」로만 넘었다.** 반(反)이 여기를 치면 이 초안의 처분 줄이 먼저 무너진다.
2. **M1~M6 이 사전부검과 갈렸다는 사실 자체를 이 초안이 안 풀었다.** 717 vs 723 · 273 vs 266 · 180 vs 191 · 7 vs 9. 갈린 이유를 못 댔다. 만약 사전부검 쪽이 옳고 내 `rg` 가 뭔가를 이중으로 세고 있다면, M16 의 갈래 표 전체가 같은 편향을 물려받는다. **반(反)은 M1~M6 을 그대로 다시 돌리는 것부터 하면 된다.**
3. **M16 의 「참조」는 엣지가 아니다.** 설계문 `:78` 이 못 박은 그 구별이다. 내가 센 것은 *"오늘 `use` 가 만든 `not_a_symbol` 바인딩에서 끝나는 참조 자리"* 이고, 해소기를 붙였을 때 실제로 서는 엣지의 수는 **아무도 안 쟀다**(구현이 없으므로 잴 수 없다). 별칭·중첩 목록(PM1-04)·동명 임포트(PM1-05)가 그 수를 깎는다. **315 와 1568 은 상한이지 산출 예측이 아니다.**
4. **A3b 251 을 「인라인 `mod tests` 라 파일 안」이라고 뺀 것이 거칠다.** 들여쓰기로 갈랐을 뿐 실제 `mod` 중첩을 안 봤다. `use super::inside::Root;` 같은 형태가 인라인 `mod` 안에 있으면 그것은 파일 간이다. 이 251 이 어디로 가느냐에 따라 ㉯ 의 수가 1568 ~ 1819 사이에서 움직인다.
5. **㉮ 의 315 중 313 이 두 덩어리에 몰려 있다** — `crates/pal-cli/tests/common/mod.rs`(23 파일) 와 `crates/pal-cli/src/install/`(1 파일 100 참조). 나는 이 편중을 「그러므로 ㉮ 는 약하다」의 근거로 썼는데, **반대로 「그러므로 315 는 실은 2 건짜리 사례이고 비-0 증인으로 못 쓴다」로도 읽힌다.** 그렇게 읽으면 ㉮ 의 2 번 칸이 비고, 설계문 `:177` 에 따라 ㉮ 가 살아남은 것으로 안 세어진다 — 그러면 이 판의 대안표가 바뀐다.
6. **㉯ 를 골랐지만 ㉰ 의 모집단(1708)이 ㉯ 의 것(1253)보다 크다.** 내가 ㉰ 를 뺀 근거는 「3 번 칸이 한 칸을 넘는다」인데, 그 판단은 §2.2 의 격리 사본 하나(`pub use` 가 심볼을 안 만든다)에 얹혀 있다. **재수출 원본을 `ImportSet` 쪽에 실을 길이 있다면 그 근거가 사라지고 ㉰ 또는 ㉱ 가 이긴다.**
7. **PM1-01 을 「전 선택지 공통이라 변별하지 않는다」로 이송했다.** 만약 착지점(`counts.locals` 로 흐르는 것)이 어떤 값에서는 고치기 쉽고 어떤 값에서는 어렵다면, 그것은 변별항이고 내가 잘못 이송한 것이다.
8. **처분이 「확대」면 이 판은 승격 없이 닫힌다.** 설계문 `:45` 는 *"이 판의 정상 종착 하나가 「소유자에게 올린다」이고, 그것은 판의 실패가 아니다"* 라고 적었다. 내 초안은 그 종착을 **안 고른** 쪽이다. 1 번이 서면 이 결론이 통째로 뒤집혀 「전환 → 승격」이 된다.

---

## 7. 내가 확인 못 한 것

- **`cargo build --release` 를 안 돌렸다.** 코드 동일성으로 대신했다(§0). 반(反)이 재빌드해서 수가 달라지면 그것이 반론이다.
- **해소기를 붙였을 때 실제로 서는 엣지 수를 못 쟀다.** 구현이 없다. 위 모든 수는 **참조 자리의 상한**이다.
- **A3b 251 참조의 `mod` 중첩 실체를 안 봤다**(들여쓰기로만 갈랐다).
- **㉳(증인 교체)의 다른 모집단을 안 쟀다.** 다른 저장소를 안 클론했다. 그래서 ㉳ 의 2 번 칸이 빈 채로 남았고, 나는 그것을 「살아남지 못했다」로 처리했다.
- **`crates/` 밖의 `.rs` 2 개(`xtask` 등)를 모집단에서 뺐다** — 사전 등록 명령표가 `crates` 로 못 박았기 때문이다.
- **저장소 전체의 `outside_file` 갈래 총계를 안 쟀다.** M8 은 `projection.rs` 한 파일(55)뿐이다. 그래서 ㉲ 의 1 번 칸이 부분적으로만 찼다.
- **`intent.md:74` 의 4,259 와 오늘의 4258 이 왜 갈리는지 못 밝혔다.** `observations/red.md:24` 의 「헤더 1 행 포함」 주석과도 안 맞는다.
- **M1~M6 이 사전부검과 갈린 이유를 못 밝혔다.**
- **`findings.jsonl` 은 열지 않았다.** 그래서 그 파일에 무엇이 적혀 있는지 모른다.

---

```
값:   「해소 깊이」를 **`use` 직접 임포트 + `crate::`·`super::`·`self::` 경로 해소** 로 다시 잠근다
      (`pub use` 재수출 추적은 범위 밖으로 남긴다).

처분: **확대**.
      자 — 「의도의 양」을 **파일 간 참조 해소 자리의 수**로 센다(근거: `intent.md:23-25` 의
      목적 기여가 「역방향 색인이 비었다」이고 `docs/plan/03-shortest-path.md:123` 의 2 단계
      합격선이 재는 것과 같은 양이기 때문).
      원 의도(㉮ 유지) = **315** 자리 · 25 파일 · 1 개 크레이트.
      새 값(㉯)       = **1568** 자리 · 약 71 파일 · 7 개 크레이트 전부.  → **4.98 배 늘었다.**
      `SKILL.md:329` 의 자로 「늘었으면 정정, 줄었으면 완화」 → 축소·완화가 아니다.
      `SKILL.md:325` 의 전환(「의도나 방향을 뒤엎는다」)에 안 걸린다 — 의도의 제목·목적 기여·
      방향이 그대로이고 일곱 칸 중 여섯이 안 움직인다.
      `SKILL.md:322` 의 정정(「완수 조건이 의도를 못 잰다」)에도 안 걸린다 — 이 회차의 완수
      조건은 `intent.md:49-51` 에서 아직 안 쓰였다.
      남는 것은 `SKILL.md:323` 의 확대(「착수 때 못 본 것이 나왔고 원 의도 충족에 필요하다」)
      이고, 못 본 것은 **Rust 2018 균일 경로**(§2.1)와 **A1 의 1110 참조**(§1 표)다.
```
