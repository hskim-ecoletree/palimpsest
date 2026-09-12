# 음성 대조 — `A5`·`A5-b` ⟨`A5-a`⟩

> 회차 `2026-09-11-effect-confirmation` · 잰 날 2026-09-12 · 재는 자리 **㉠ `#79`**
> 조건 문면: *"사본 **둘**에서 판정이 **뒤집히는지** 재고 `effect/negative-A5.md` 에 둘 다
> 남긴다 — ⓐ touch 커밋을 변경 커밋 뒤로 옮긴 사본에서 `A5` 가 빨개진다 · ⓑ 코드 변경을
> **워킹트리에 먼저 써 놓고** touch 를 돌린 사본에서 `A5-b` 의 `body <hash>` 가 봉인 커밋
> 시점의 본문과 **안 맞는다**. … 한 축만 초록이어도 `A5-a` 는 **불통과**다"*

검사와 격리 조건은 `effect/negative-A1.md` 가 진다.

## 축 ⓐ — touch 커밋을 변경 커밋 뒤로

사본 `neg5`: 봉인 → **변경** → touch → readnote 순으로 커밋했다.

```
FAIL  A5  — touch=a0a39b98dc5f26ec942fed1618ab1af49c7e9141 change=6eb05e0e3bd3be1df6d1e508f97718e4e59bd73e
FAIL  A7  — touch=a0a39b98dc5f… readnote=ef9b86265eef… change=6eb05e0e3bd3…
```

**뒤집혔다.** `A7` 도 함께 빨개졌다 — 노린 축 밖이지만 같은 방향이라 그대로 적는다.

## 축 ⓑ — 변경을 워킹트리에 먼저 쓰고 touch 를 돌린다

사본 `neg9`: HEAD 를 **봉인 커밋 `77dc977`** 에 두고, `9f993cc` 의 코드 변경
(`crates/pal-cli/src/defect.rs` · `crates/pal-cli/src/ledger.rs`)을 **커밋하지 않고 워킹트리에만**
써 놓은 뒤 `pal touch nodes_of` 를 돌렸다.

| | 원본 `touch/79.txt:16` | 사본 `neg9` |
|---|---|---|
| 심볼 ID | `palimpsest@77dc977+worktree#a9dd338ebdd7` | `palimpsest@77dc977+worktree#a9dd338ebdd7` ← **같다** |
| 앵커 줄 | `fun · crates/pal-cli/src/ledger.rs:301 · identity ordinal · body 7058bb9c2fcf` | `fun · crates/pal-cli/src/ledger.rs:378 · identity ordinal · body 305e595b853e` |

**뒤집혔다** — `body` 가 `7058bb9c2fcf` → `305e595b853e` 로 갈렸다. 봉인 커밋 시점의 본문에서
나온 값이 아니다.

★★ **머리는 한 글자도 안 갈렸다.** 심볼 ID 도 HEAD SHA 도 같다. **그러므로 `A2` 의 머리
넷으로는 이 오염을 못 잡고, 잡는 것은 `A5-b` 의 바이트 앵커 하나다.** 그것이 `A5-b` 가
선 까닭이고 ⟨`PM2-03`⟩ 이 격리 사본에서 처음 재현한 것과 같은 값이다.

★★ **`A5-c` 가 관측한 결함이 이 자리에서 다시 났다.** 워킹트리에 수정 파일이 **둘**
(`git status --porcelain` 이 ` M` 둘을 낸다)인데 산출 `:48` 은 그대로 **`워킹트리  일치`**
를 찍는다. 사람이 그 줄을 *"touch 를 돌릴 때 고친 것이 없었다"* 로 읽으면 정확히 이
사본이 만든 오염을 못 본다. **이 회차는 관측만 하고 안 고친다** ⟨`## 범위 밖`⟩.

## `A5-a` 의 판정 — **통과**

두 축이 각각 뒤집혔고 안 뒤집힌 축이 없다.

## 전 출력 — 축 ⓐ

```
### 사본 neg5 — seal=b24222e04399 touch=a0a39b98dc5f readnote=ef9b86265eef change=6eb05e0e3bd3
PASS  A1
PASS  A1-b
PASS  A1-c
PASS  A3
FAIL  A5  — touch=a0a39b98dc5f26ec942fed1618ab1af49c7e9141 change=6eb05e0e3bd3be1df6d1e508f97718e4e59bd73e
FAIL  A7  — touch=a0a39b98dc5f26ec942fed1618ab1af49c7e9141 readnote=ef9b86265eefe031ad16c4b08780e30da814271a change=6eb05e0e3bd3be1df6d1e508f97718e4e59bd73e

```

## 전 출력 — 축 ⓑ

```
$ git -C <사본> checkout -q -B neg9 77dc977
$ git -C <사본> checkout 9f993cc -- crates/pal-cli/src/defect.rs crates/pal-cli/src/ledger.rs
$ git -C <사본> reset -q            # 스테이지에서 내려 워킹트리에만 둔다
$ git -C <사본> status --porcelain
 M crates/pal-cli/src/defect.rs
 M crates/pal-cli/src/ledger.rs
$ cd <사본> && /Users/incognito/dev/projects/palimpsest/target/release/pal touch nodes_of
rc=0 · stdout 4532 바이트 · stderr 0 바이트
```

```

  nodes_of  ·  palimpsest@77dc977+worktree#a9dd338ebdd7
  fun · crates/pal-cli/src/ledger.rs:378 · identity ordinal · body 305e595b853e

■ 이 좌표에 걸린 것 (0)
  **못 읽었습니다 — 「0 건」이 아닙니다.** 파생 저장소가 없습니다: ./.palimpsest/intent.redb
  정본은 있습니다: ./.palimpsest/intent/bindings.jsonl
  세우려면 — `pal intent import ./.palimpsest/intent/bindings.jsonl`
■ 이 좌표를 지켜보는 것 (0)
  **못 읽었습니다 — 「0 건」이 아닙니다.** 파생 저장소가 없습니다: ./.palimpsest/intent.redb
  정본은 있습니다: ./.palimpsest/intent/bindings.jsonl
  세우려면 — `pal intent import ./.palimpsest/intent/bindings.jsonl`
■ 이 심볼이 하는 것
  호출자 4 · 피호출자 2
  ※ 「호출자·피호출자」는 **참조 엣지**입니다 — 호출뿐 아니라 타입 참조·구조체 리터럴·경로 머리도 셉니다
  ※ **`x.foo()` 는 아직 안 셉니다** — 멤버 해소(`member-resolution`)는 타입 추론이 필요해 **이 회차의 범위 밖**입니다. 능력 부재가 아니라 안 하기로 정한 자리입니다
  ※ 파일 간 해소 — ⓐ `cross-file-import` 1833/5812 · ⓑ `path-resolution` 281/1072 (선 것/짝)
  ※ ⓐ 가 못 선 까닭 — ambiguous 214 · no_symbol 668 · no_symbol_at_crate_root 1957 · outside_repo 1140
  ※ ⓑ 가 못 선 까닭 — ambiguous 14 · no_symbol 154 · no_symbol_at_crate_root 340 · outside_repo 283
  ※ 못 선 몫이 가는 문 — 이 회차가 안 세우기로 정한 자리와 못 세우는 자리를 가릅니다
      ⚠ 갈래 하나는 **아직 안 갈렸습니다** — 그 자리는 문이 아니라 「미측정」을 적습니다
      `outside_repo` → **경계** — 저장소 밖(`std::*` 등)이라 원리상 못 섭니다
      `no_symbol_at_crate_root` → `A5`·`A5-a` — 재수출 경유는 잠근 축1 의 **밖**입니다
      `no_symbol` → **갈래가 아직 안 갈렸습니다** — `#133`(L2)로 갈 몫과 이 회차의 구현 여지가 섞여 있고 그 크기는 **미측정**입니다
      `no_target_file` → **이 회차의 구현** — 모듈 경로를 파일로 못 폈습니다
      `ambiguous` → **후보 생성의 모호** — 모듈 경로가 여러 파일로 읽힙니다. 하나를 고르면 조용한 오답이라 **안 고르는 것이 설계입니다**
■ 내가 모르는 것
  9 건
  · RepoId — no_symbol_at_crate_root · 지난 걸음 import_item(1→1) module_path(4→1) target_symbol(1→0)
  · RepoPath — no_symbol_at_crate_root · 지난 걸음 import_item(1→1) module_path(4→1) target_symbol(1→0)
  · Containment — no_symbol_at_crate_root · 지난 걸음 import_item(1→1) module_path(4→1) target_symbol(1→0)
  · SymbolNode — no_symbol_at_crate_root · 지난 걸음 import_item(1→1) module_path(4→1) target_symbol(1→0)
  · BTreeMap — outside_repo · 지난 걸음 import_item(1→1) module_path(8→0)
  · Discriminator — no_symbol_at_crate_root · 지난 걸음 import_item(1→1) module_path(4→1) target_symbol(1→0)
  · new — no_symbol_at_crate_root · 지난 걸음 import_item(1→1) module_path(4→1) target_symbol(1→0)
  · SymbolId — no_symbol_at_crate_root · 지난 걸음 import_item(1→1) module_path(4→1) target_symbol(1→0)
  · compute — no_symbol_at_crate_root · 지난 걸음 import_item(1→1) module_path(4→1) target_symbol(1→0)
■ 효과
  (이 빌드에는 effects 능력이 없습니다 — F13 미구축)
■ 판정
  (이 빌드에는 judgment 능력이 없습니다 — F15 미구축)

■ 이 답의 근거
  Snapshot  palimpsest@77dc977+worktree
  대장      parsed 141 · partial 0 · unsupported 778 · unrecognized 345 / 1265 파일
            결박 불가 언어 7개 — 그 파일들에는 좌표가 없습니다
  2층       심볼 3325 색인됨
  워킹트리  일치
  재구축    아님
  생략      없음 (명시)
  이관      1265건 — 본체를 다른 질의로 옮겼습니다. 생략된 것이 아닙니다
            ledger 1265건 → `ledger.snapshot` 로 조회할 수 있습니다
  질의 로그  남았습니다
  크기      약 1937 토큰 **이상** (잰 것: 7750 바이트 · 가정: 4 바이트/토큰)
  능력      ledger.snapshot · symbol.resolve · symbol.contains · symbol.callers · symbol.reaches · graph.dump · binding.status · narrative.unbound · binding.touch · plan.deviation · symbol.references · 미구축 F13 · F15

```
