# 음성 대조 — 규율의 순서를 뒤집으면 넓힘이 **정말** 사라지는가

> 회차 `2026-09-12-binding-radius-in-use` · 2026-09-13
> 조건 `B6-a`. **묻는 것: `import` → `export` 순서로 돌면 넓힘이 사라지는가.**
>
> ⚠ **저장소를 클론하지 않았다.** `crates/pal-cli/src/ledger.rs:581` 의 `repo_name` 이
> 디렉터리 이름을 `RepoId` 로 쓰므로 복제하면 `SymbolId` 가 전부 달라져 **어긋난 사유로**
> 빨개진다(`CA1-05`). `pal intent export/import` 가 `--intent <경로>` 를 받으므로
> **같은 디렉터리에서 임시 의도 저장소 사본**으로 돌았다.

## 무엇을 했나

1. 넓힌 뒤의 파생물(`intent.redb`)을 임시 자리로 복사했다 — 사본의 반경 분포
   **`symbol` 19 · `callers` 18**.
2. **넓히기 전의 정본**(커밋 `fd176a8` 의 `bindings.jsonl`)을 그 사본에 `import` 했다.
3. 그 사본을 `export` 했다.

## 결과 — 뒤집혔다

```
$ pal intent import <옛 정본> --intent <사본>
{ "schema_version": 2, "bindings": 37, "aliases": 0, "already_present": 37 }

$ pal intent export --intent <사본>   # 반경 분포
{'"symbol"': 37}                      ← callers 18 이 통째로 사라졌다
```

그리고 그 export 가 **옛 정본과 바이트로 같다** — 즉 정본에 썼다면 **`git diff` 가 빈다.**

> **규율이 세운 유일한 검사기가 일이 지워졌을 때 가장 깨끗한 초록을 낸다.**

`crates/pal-intent/src/store.rs` 의 `record` 가 **id 가 같으면 덮으므로**, 아직 옛 값인
추적본을 import 하면 넓힌 것이 되돌아가고 이어진 export 가 그 되돌아간 값을 파일에 다시 쓴다.

## ★ 대조가 새로 드러낸 것 — 보고조차 덮었다는 말을 안 한다

`import` 의 보고가 **`already_present: 37`** 이다. *"37 건이 이미 있었다"* 는 말이고
**「18 건의 반경을 덮었다」는 어디에도 없다.** 그러므로 이 사고는 ⑴ `git diff` 가 비고
⑵ import 보고가 초록이고 ⑶ 판정 분포도 원래대로 돌아가 **세 자리 모두 정상으로 읽힌다.**

**그래서 순서가 규율인 것이다** — `export` 를 먼저 하면 `git diff` 가 18 행을 내고, 그것이
넓힘이 실재한다는 유일한 신호다(`B6` 이 그것을 쟀다: 18 insertions / 18 deletions).
