# 착수 시점 관측 — 효과 확인

> 회차 `2026-09-11-effect-confirmation` · 착수 커밋 `8604d62` · 2026-09-11
> **착수를 잰 시점**(2026-09-11 · 이 회차 디렉터리를 만들기 전)에 워킹트리는 `8604d62` 와
> 같고 clean 이었다 — `git diff --stat 8604d62..HEAD` 0 줄 · `git status --porcelain` 0 줄.
> ⚠ **지금 다시 돌리면 그렇지 않다** — 이 회차 디렉터리 자신이 미추적으로 잡힌다.
> 관측은 잰 시점의 것이고, 이 줄은 그 시점을 못 박으려고 있다.

이 절은 **관측**이다. 판정도 계획도 아니다. 규약 §3 이 *"시작할 때 실제로 빨간 것을
관측한다. 0 개를 훑고 통과하는 검사는 아무것도 안 잰다"* 를 요구하는 그 자리다.

## 1. 이 단계는 한 번도 실행된 적이 없다

`docs/plan/03-shortest-path.md` §4 의 3 단계가 자기 「왜 이 자리인가」 칸에 그렇게 적는다:

> **3** | **효과 확인** — 이 저장소에서 `pal touch` 를 실제 작업 앞에 부르고, 그 산출을 본
> 뒤의 변경이 달라졌는지 잰다 | `00-goals.md` §0.1 — *"산출이 아니라 결과"*.
> **그리고 이 단계는 한 번도 실행된 적이 없다** | 그 산출을 보고 달라진 변경 **1 건 이상**,
> 좌표와 함께

## 2. 도구는 선다 — 능력 부재가 아니다

`./target/release/pal touch file_edges` (빌드 `0.0.0+8604d6280aee`):

```
  file_edges  ·  palimpsest@8604d62+worktree#e09b7d058d5c
  fun · crates/pal-core/src/projection.rs:321 · identity ordinal · body caedbf7754a0
  호출자 6 · 피호출자 13
  ※ 파일 간 해소 — ⓐ `cross-file-import` 1823/5767 · ⓑ `path-resolution` 281/1062 (선 것/짝)
```

⚠ **분모가 앞 세션의 인용과 갈린다** — 인용은 ⓐ `1823/5760` · ⓑ `281/1061` 이었고 이 회차가
다시 재서 얻은 것은 ⓐ `1823/5767` · ⓑ `281/1062` 다. 분자는 같다. 어느 쪽이 맞는지 이 회차는
판정하지 않는다 — **여기 적는 것은 이 회차가 잰 값이고**, 갈림 자체가 관측이다.

## 3. 착수 기준선 — 이 수들이 「멈추고 올리는 자리」의 기준이다

| 잰 것 | 값 | 무슨 명령으로 |
|---|---|---|
| `cargo xtask check` | **28/28 통과** | `cargo xtask check` |
| `cargo test --workspace` | 묶음 47 · 통과 **1028** · 실패 **0** · ignored **1** | `cargo test --workspace` (전 출력 1319 줄) |
| ignored 의 이름 | `사종_벤치와_선형성` (`tests/bench.rs`) | 위와 같음 |
| 컴파일 경고 | **0** | `.rs` 전량 mtime 갱신 후 `cargo build --release --workspace` · `cargo check --workspace --all-targets` 각각 |
| 결박 | **34** — fresh 27 · stale 7 | `pal query binding.status --json` |
| 대장 | 파일 1229 — parsed 141 · partial 0 · unsupported 763 · unrecognized 324 | `pal ledger` |
| 결박 가능한 파일 | Rust L1 **140** · TypeScript L2 **1** | 위와 같음 |
| 결박 불가 언어 | **7** — Markdown 557 · Python 165 · TOML 29 · Shell 6 · JSON 3 · YAML 2 · JavaScript 1 | 위와 같음 |
| 서사 | 문서 558 · 조각 **6279** · 결박된 조각 **57** · 미결박 4551 | `pal narrative` |
| CI | `8604d62` 에 `completed success` | `gh run list --limit 3` |
| 프론티어 | 열린 이슈 49 · 착수 가능 49 · 교착 0 | `./scripts/frontier.sh` |

★ **경고 0 은 캐시 적중 위에서 잰 값이 아니다.** `cargo test` 의 빌드는 `Compiling` 줄이
0 이라 그 출력의 경고 0 은 측정치가 아니고, 그래서 `.rs` 전량의 mtime 을 갱신해 8 크레이트를
강제 재컴파일한 뒤 두 번 따로 쟀다. `cargo check` 이므로 `[workspace.lints.clippy] pedantic`
은 **안 켜졌다** — 그 축은 이 관측이 안 잰다.

## 4. `#66` 의 전제가 낡았다 — 이것이 이 회차의 전제다

`#66` 본문이 적는 것:

> palimpsest 에서 결박 가능한 좌표   7 개
> 그 좌표가 사는 파일               1 개  — corpus/tasks/f03-normalize-seeds.ts
>
> **코어가 Rust이고 추출기는 Kotlin·Java·JavaScript·TypeScript 넷뿐이다**(`pal_core::Language`).

**위 3 절의 실측이 그것을 뒤집는다** — 결박 가능한 파일이 Rust 140 · TypeScript 1 이고,
`pal touch file_edges` 가 `crates/pal-core/src/projection.rs:321` 의 Rust 심볼을 짚는다.
그 능력 부재는 `#130`(2026-09-08) 과 `#135`(2026-09-10) 가 없앴다.

⚠ **`#66` 이 묻는 두 물음은 낡지 않았다.** 두 번째 댓글이 적은 이 줄이 이 회차의 자리다:

> ★ AGENTS.md 가 정한 것: *"`grep` 을 집으려는 순간에 그래프에 먼저 묻는다. `grep` 은
> 문자열을 맞히고 그래프는 **관계를 안다**."* **그 문장이 이 저장소에서 아직 한 번도 참이
> 된 적 없다**

## 5. 무엇이 빨간가 — 이 회차가 끌 것

전수 실측이 `observations/red-survey.md` 에 있다. 모집단은 **커밋된 21 회차 · 771 파일**
(`git ls-files`)이다. 이 회차 자신의 디렉터리는 미커밋이라 그 수 밖이다.

### 5.1 「무엇이 바뀔 것인가」를 미리 봉인하고 실제와 대조한 기록 — **0 건**

**「없다」이지 「못 찾았다」가 아니다.** 낱말이 아니라 **꼴**로 여섯 가지를 물었고 그중
「계획↔실제 대조표」가 0 히트다.

★ **봉인 자체는 넷 있다** — 그러나 **넷 다 봉인 대상이 「측정·판정 규칙 / 분모」이지
「무엇이 바뀔 것인가」가 아니다:**

| 좌표 | 무엇을 봉인했나 |
|---|---|
| `2026-08-23-agent-laziness-behavior/exp/prereg/` (12 파일 · `PREREG-SHA.log` = `1843feb`) | 실험의 판정 규칙 |
| `2026-09-08-cross-file-references/observations/a7-{call,denominator}-preregistration.md` | 분모 — *"⚠ 이 문서는 측정 전에 쓴다"* |
| `2026-08-18-completion-condition/retro/06-pendulum-metrics.md:3,54,65` | 지표 정의 |
| `2026-09-06-terrain-and-completion-scene/dialectic/2-design.md:219` | 토론 설계 |

가장 가까운 지시는 `2026-09-06-user-surface-vocabulary/intent.md:17` 의
*"**회차를 닫기 전에** `pal touch` 를 …"* 인데 그것은 **사후 부착**이다.

⚠ 측정 도구가 걸린 자리 하나 — `grep -E` 가 이 기계에서 `ugrep` 이라 긴 한글 교차 정규식
둘이 `exceeds complexity limits` 로 죽었다. **쪼개서 다시 돌렸다.** 빈 결과를 부재로
안 읽었다.

### 5.2 `## 효과` 에 `pal touch` 가 붙은 게이트 — 53 중 4 · 그중 **2**

**두 번 쟀다.** 첫 실측과 독립 재측정(`observations/red-verify.md`)이 **같은 값**을 냈다.

| | 값 |
|---|--:|
| 게이트 문서 총수 | **53** |
| `## 효과` 절이 있는 게이트 | **18** |
| 그 절에 `pal touch` **출력**이 붙은 게이트 | **4** |
| 그중 「그 산출을 보고 무엇이 달라졌다」를 적은 것 | **2** |

넷의 좌표 — `rust-scope-references.md:329` · `cross-file-references.md:342` ·
`terrain-and-completion-scene.md:147` · `user-surface-vocabulary.md:273`.

**독립 재측정이 쓴 기준**(먼저 적고 그 기준으로 셌다): *"출력을 **본 뒤** **구체적 산출물이
실제로 바뀌었다**를 **인과로** 적을 것. 전후 관측은 제외."* 그 기준으로 둘:

> `rust-scope-references.md:356-358` — *"그래서 `pal touch` 의 라벨에 줄을 하나 더 넣었다 …
> **효과 관측이 화면을 고치게 한 자리다.**"*
> `cross-file-references.md:366, 379` — *"**`pal touch` 의 답만 그것을 안 싣고 있었다.**
> 고친 뒤의 답:"* · *"★★ 그리고 고친 화면이 종료 시점에 실제로 답을 바꿨다."*

제외한 것 — `terrain-and-completion-scene.md:176·193`(바뀐 것이 **판정**이지 산출물이
아니다) · `user-surface-vocabulary.md:312`(전후 관측). 기준을 느슨히 하면 3, 더 느슨히
하면 4 가 된다.

★★ **그리고 이것이 이 회차의 축을 확정한다** — 독립 재측정의 원문:

> **센 둘 다 「달라진 것」이 `pal touch` 자신의 화면이고, 「하려던 작업이 달라졌다」는 0 건.**

**그러므로 합격선 문면은 착수 시점에 안 충족돼 있다.** 최단 경로 §4 3 단계의 문면은
*"이 저장소에서 `pal touch` 를 **실제 작업 앞에** 부르고, 그 산출을 본 뒤의 **변경**이
달라졌는지 잰다"* 이고, 그 「변경」은 **실제 작업의 변경**이다. 그 수가 **0** 이다.

⚠ **그러나 *"이 단계는 한 번도 실행된 적이 없다"* 도 통째로 참은 아니다.** 위 둘은 같은
도구로 같은 종류의 관측을 했고 **touch 자신의 화면**을 고쳤다. **이 회차는 합격선을 올리지
않는다** — 축을 정확히 긋고, 그 둘을 나란히 적을 뿐이다.

### 5.3 `.claude/pal/policy.toml` — **없다**

`.claude/pal/` 디렉터리 자체가 없고 `find . -name 'policy.toml' -not -path './target/*'`
가 빈 출력이다. 그러므로 금지역은 **규약의 기본 다섯**이 진다.

### 5.4 재는 자리 셋 — 좌표가 실재하고 아직 안 고쳐졌다

셋 다 `OPEN` · `assignees []`.

| 이슈 | 실재하는 좌표 |
|---|---|
| `#79` | `crates/pal-cli/src/ledger.rs:334` — `identity: discriminator.identity_ceiling().min(s.identity),` 원문 그대로 · `nodes_of` → `:301` · `identity_ceiling` 정의 → `crates/pal-core/src/coord.rs:259` |
| `#126` | `xtask/src/main.rs:5851` `check_ledger_pair`(호출 `:643`) · 결함 본체 `:5865 회차들.sort();` 와 `:6254-6258 .iter().rev().find(…)` · 판정문 `:6263`·`:6270` · 선언 `docs/gates/README.md:140` |
| `#129` | 본문에 `파일:줄` 이 0 개라 에러 문자열로 해소했다 — `narrative.rs:200 keep_entity` → `store.rs:424 self.write()` → `store.rs:167-171` 로 경로가 끊김 없이 이어진다 · 질의 이름 `query_log.rs:73,115` |

⚠ **`#79` 본문이 크레이트 경로를 안 적었고 `ledger.rs` 는 이 저장소에 셋이다.**
⚠ **`#129` 의 재현은 안 돌렸다** — 그 서베이는 읽기 전용이었다.

### 5.5 `#66` 의 두 수 — 방법이 지목된 근거에 **없다**

`#66` 은 판정 기록으로 `docs/gates/F11-touch.md` §10 과 `corpus/criteria.toml` 의
`[f11.pass].self_repo_grounds` 를 댄다. **그 두 자리에 명령이 0 줄이고, 「7」·「1」이라는
수가 합격선에 아예 없다** — 있는 것은 `self_repo_bound_min = 1` 뿐이다. 같은 수를 쓰는
다섯 자리가 서로를 인용한다.

재구성한 방법: `pal ledger --symbols --cache-dir <스크래치> .` 로 그 `.ts` 파일만 세면
**정확히 7** 이 나온다 — 수가 방법의 동치를 증언한다.

| | `#66` 이 적은 값 | 지금 값 |
|---|--:|--:|
| 결박 가능한 좌표 | 7 | **3,306** |
| 그 좌표가 사는 파일 | 1 | **141** (`.rs` 140 + `.ts` 1) |

둘째 방법(`--json` 의 l0 아닌 언어 파일 합)도 **141** 로 같다.

### 5.6 `pal_core::Language` 는 **다섯** · 대장이 등급을 매기는 언어는 **아홉** · 추출기는 **셋**

셋이 다른 수이고 **세 축이 다른 것을 센다.** 독립 재측정이 그것을 갈랐다.

| 축 | 값 | 좌표 |
|---|--:|---|
| `Language` enum 변종 | **5** — Kotlin · Java · JavaScript · TypeScript · **Rust** | `crates/pal-core/src/language.rs:24` · `:83-87`(`ALL`) |
| 이 저장소에서 `pal ledger` 가 등급을 매기는 언어 | **9** — Markdown L0 557 · Python L0 165 · **Rust L1 `ordinal` 140** · TOML L0 29 · Shell L0 6 · JSON L0 3 · YAML L0 2 · JavaScript L0 1 · **TypeScript L2 `exact` 1** | `pal ledger` |
| **오늘 HEAD 의 코드가 실제로 가진 추출기** | **3** — `Capable::Present` 가 Kotlin · TypeScript · Rust. `not_built` 가 Java · JavaScript | `crates/pal-extract/src/extractor.rs:74-80` · 모듈 `kotlin.rs`·`typescript.rs`·`rust.rs` |

교집합은 **셋**(Rust · TypeScript · JavaScript). enum 에 있고 파일이 0 인 것은
**Kotlin · Java**. 등급은 받는데 enum 에 없는 것이 **여섯**이다 — 까닭은 실측으로 났다:
**이름표를 정하는 자는 enum 이 아니라** `crates/pal-extract/src/recognize.rs:34
BY_EXTENSION` 과 `:69 BY_FILE_NAME` 이다.

**그러므로 `#66` 의 *"추출기는 Kotlin·Java·JavaScript·TypeScript 넷뿐이다"* 는 두 겹으로
틀렸다** — 수가 넷이 아니라 셋이고, 목록에서 Rust 가 빠졌고 Java·JavaScript 가 들어 있다.

### 5.7 갈린 것 — 이 회차가 관측만 하고 판정은 뒤로 미룬다

| | 무엇이 갈렸나 |
|---|---|
| ⓐ | 회차 수 **22 vs 21** · 파일 **775 vs 771** — 이 회차 디렉터리가 미커밋이라 생긴 차이다. 두 실측이 같은 값을 냈다 |
| ⓑ | **「추출기가 몇인가」를 저장소가 다섯 값으로 적는다** — 아래 표 |
| ⓒ | 5.2 의 마지막 수가 기준에 따라 **2 · 3 · 4** 로 움직인다. 독립 재측정이 **기준을 먼저 적고** 2 를 냈다 |
| ⓓ | `#79` 의 `ledger.rs` 가 이 저장소의 셋 중 어느 것인지 **본문이 안 적는다**. 실물은 `crates/pal-cli/src/ledger.rs:334` |

**ⓑ 의 전수** — *"추출기가 몇인가"* 를 수로 적은 자리:

| 수 | 자리 |
|--:|---|
| **셋** | `crates/pal-core/src/capable.rs:13` · `docs/overview.md:1026` · `docs/gates/F11-touch.md:318` = `docs/gates/F12.md:305` · `docs/plan/03-shortest-path.md:82` |
| **넷** | `docs/gates/F11-touch.md:316` · `scripts/f12-verify.py:559` · `corpus/criteria.toml:10520`·`:10799` · `docs/gates/F02-1-extractor.md:179` · `docs/gates/inventory-disposal.md:450` · `…/2026-08-18-inventory-disposal/state.md:307` |
| **둘** | `crates/pal-core/src/file_graph.rs:158`·`:123` · `scope.rs:369` · `parse.rs:124` · `lib.rs:150` · `docs/adr/0027-…:22`(과거형) · `…/2026-08-20-rust-extractor/intent.md:174` · `corpus/criteria.toml:11237` |
| **하나** | `scripts/f22-3-verify.py:171` |

**오늘 HEAD 의 코드가 내는 수는 셋이다**(§5.6). ⚠ `docs/gates/F11-touch.md` 는 **한 문서
안에서** `:316` 넷 · `:318` 셋으로 갈린다.

★ **저장소가 이 갈림을 이미 알고 적어 뒀다** — 앞 회차들의 원장에 살아 있다:
`…/2026-08-20-rust-extractor/findings.jsonl:11`(`PM1-S10` · 해악도 **금지역**) ·
`…/findings.jsonl:44`(`PM2-S14` · 처분 **범위밖**) — *"…현재형으로 말한 채 남고, 어떤
검사도 안 본다"*. 그리고 `review/r1-raw.md:82` 가 *"「선언된 1급 다섯 · 만들어진 추출기
셋 · 나머지 둘」이 정확하다"* 로 이미 판정했다.

⚠ **이 회차는 그 갈림을 안 고친다** — `D1` 이 `#66` 하나를 정정할 뿐이다. 나머지 자리들은
동결 판정 문서(ADR · 닫힌 게이트)이거나 합격선 파일이고, 앞 회차가 이미 `범위밖` 으로
처분했다. **처분이 이미 있는 것을 다시 여는 것은 이 회차의 자리가 아니다.**

**안 갈린 것**: 결박 가능한 좌표 **3,306** · 파일 **141**. 세 방법이 전부 일치했다 —
① `pal ledger --symbols` 의 distinct `id` ② 141 파일에 `pal symbols --json` 배열 길이 합산
③ `pal touch` 근거 상자의 *"2층 심볼 3306 색인됨"*.

### 5.8 ⚠ 메인이 이미 오염됐다 — 사전부검이 `pal touch` 를 먼저 돌렸다

**사전 등록을 쓰기 전에** 사전부검 라운드 1 이 세 심볼에 `pal touch` 를 돌리고 **그 결과를
메인에 요약해 돌려줬다.** 숨기지 않고 여기 전수로 적는다. 이 목록에 있는 것은 **`C2` 의
귀속에서 뺀다** — touch 가 아니라 사전부검이 말해 준 것이기 때문이다.

| 심볼 | 메인이 이미 아는 것 |
|---|---|
| `nodes_of` | 결박 **0** · 지켜보는 것 **0** · 산출이 52 줄 |
| `identity_ceiling` | 결박 **0** · `pal touch` 의 **호출자 0** · `pal query symbol.callers` 가 **(없음)** · ⚠ **그 0 이 거짓 음성이고 실제 호출 자리가 넷**이라는 것(`grep` 으로 재었다). 까닭은 touch 가 스스로 적는 *"`x.foo()` 는 아직 안 셉니다"* |
| `check_ledger_pair` | 결박 **0** · 산출이 46 줄 · 같은 파일 `xtask/src/main.rs` 안에 결박 **8** 건이 있고 그중 하나가 **한 칸 옆**이다 |
| 둘 사이 | `nodes_of` 와 `check_ledger_pair` 의 산출이 **38 줄 동일**하다 |

★ **이것은 회차를 무르게 하지 않는다 — 기록으로 세운다.** 인터뷰 3 이 잠근 답이
*"평소대로 조사하고, **무엇을 봤는지 기록한다**"* 이고, 사전부검은 이 회차의 평소 절차다.
**그러나 ㉠ 의 값은 실제로 깎였다** — `identity_ceiling` 의 거짓 음성을 touch 가 아니라
사전부검이 먼저 말했다.
