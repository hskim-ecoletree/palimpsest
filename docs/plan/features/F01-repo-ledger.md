# F01 — 저장소 접근과 관측 범위 대장

| 우선순위 | 의존 | 규모 | 크레이트 |
|---|---|---|---|
| **P0** | P0-preflight | M | `pal-git` · `pal-core::ledger` |

---

## 1. 왜 — 대장이 첫 산출인 이유

**모든 응답이 "내가 무엇을 보았고 무엇을 보지 않았는가"를 동반해야 한다.** 그것 없이 나온 답은 하한인지 전부인지 알 수 없고, 알 수 없는 답은 거짓 안전이다.

완전성을 반증 가능한 형태로 재정의한 것이 대장이다:

> 기계는 **선언된 관측 범위 안에서** 전수임을 보증한다. 범위 밖은 "없음"이 아니라 **"보지 않음"** 으로 산출된다.

이제 반증 가능하다 — 범위 안인데 그래프에 없는 심볼을 하나라도 제시하면 계약 위반이다.

**달성 기여**: [목표 §3.1](../00-goals.md#31-모르는-것을-안다고-하지-않는다)의 토대. 이 기능이 없으면 나머지 전부가 "얼마나 믿을 수 있는지 모르는 답"이 된다.

---

## 2. 입력 → 출력

```
입력:  저장소 경로 · 매니페스트(선택된 저장소 집합·제외 규칙) · TreeRef
출력:  Snapshot · Ledger · (경로, BlobHash) 목록
```

```
$ pal ledger
Snapshot  order-svc@a3f21c9  (워킹트리 일치)
저장소    1 (선언됨)
파일      1,204
  parsed        1,180
  partial           9   회복 지점 기록됨
  unsupported      15   언어 인식됨(SQL 12 · Dockerfile 3), 추출기 없음
  unrecognized      0
  excluded          0
  binary/generated  0
언어      TypeScript  L2  identity=exact 1,142 · ordinal 38   1,180 파일
          SQL         L0  결박 불가                              12 파일   ← 이 파일들에는 좌표가 없습니다
```

**두 가지에 주의.** ① 등급은 **선언 상한**이고 P0 시점 TypeScript의 상한은 L2다(L3 정의-사용은 F13, P2). [목표 §1의 완성 장면](../00-goals.md#1-완성-장면--이-화면이-제품이다)이 `L3`인 것은 P2까지 갔을 때의 그림이다. ② `identity`는 언어당 한 값이 아니라 **심볼 단위 실측의 분포**다 — 같은 언어 안에서도 스코프 해소에 실패한 심볼은 `ordinal`로 떨어진다([R-22](../00-risks.md#r-22)).

---

## 3. 구현

### 3.1 git 접근 — `pal-git`으로 격리

[R-15](../00-risks.md#r-15)에 따라 gix 접촉면을 다섯 메서드로 좁힌다.

```rust
pub trait GitAccess {
    fn head(&self) -> Result<CommitSha>;
    fn list_tree(&self, at: &TreeRef) -> Result<Vec<(RepoPath, BlobHash)>>;
    fn read_blob(&self, h: BlobHash) -> Result<Vec<u8>>;
    fn worktree_state(&self) -> Result<WorktreeState>;
    fn changed_between(&self, a: &TreeRef, b: &TreeRef) -> Result<Vec<RepoPath>>;
}
```

`gix`로 구현하되 트레잇 뒤에 둔다. 최악의 경우 `git` CLI 호출 구현으로 대체 가능하다.

> **정정 (2026-08-13 · F01 종료).** 위 다섯은 **상한이었고 실제로 선 것은 여섯이며 목록이 다르다.** `changed_between`은 **세우지 않았다** — 워킹트리 축에서는 `WorktreeState::dirty_paths`가 그 자리를 이미 지고, 커밋↔커밋 비교를 쓰는 것은 F05의 증분 재추출이다. 없는 소비자를 위한 것을 만들면 그것이 곧 검사되지 않는 산출이다. 대신 `commit`·`path_at`·`first_parent_walk`(F22-3)와 `read_worktree_file`(F01)이 섰다. **`read_worktree_file`이 `read_blob`으로 대신될 수 없는 이유**: 아직 커밋되지 않은 파일은 객체 저장소에 없고, 심볼릭 링크의 blob 내용은 링크 대상 문자열이다.

### 3.2 `TreeRef` — 워킹트리를 좌표화한다 ([R-06](../00-risks.md#r-06))

**이 기능이 내리는 가장 중요한 결정.** 설계는 커밋을 시간축으로 삼았지만 이 제품의 1순위 사용 장면(적시 제시)은 **커밋 전 순간**에 일어난다.

```rust
pub enum TreeRef {
    Committed(CommitSha),
    Worktree { base: CommitSha, tree_digest: Blake3 },
}

pub struct WorktreeState {
    base: CommitSha,
    tree_digest: Blake3,          // 추적 파일 (경로, blob_hash) 목록의 머클 루트
    dirty_paths: Vec<RepoPath>,   // base 와 다른 파일
}
```

> **정정 셋 (2026-08-13 · F01 종료).**
>
> **① 머클 트리가 아니다.** 정렬된 `(경로, blob 이름)` 목록의 순차 blake3 요약이다. 부분 재계산을 쓰는 소비자가 없기 때문이고(증분은 F05), 필요한 성질은 *"같은 목록 → 같은 요약, 다른 목록 → 다른 요약"* 하나다. **길이 접두사가 있어야 이름 변경이 잡힌다** — 없으면 `("ab",X),("c",Y)`와 `("a",X),("bc",Y)`가 같은 요약을 낸다.
>
> **② 아래 3번(`.palimpsest/worktree.state` 캐시)을 만들지 않았다. 그 설계가 틀렸다.** *"인덱스 mtime으로 무효화"*는 성립하지 않는다 — 파일을 고치고 `git add`하지 않으면 인덱스 mtime은 그대로이고 워킹트리만 변한다. 그 캐시는 낡은 요약을 돌려주고, `[f01.pass]` ③의 첫째 변이(내용 1바이트)가 그것을 반증한다. 1·2번(인덱스 stat 신뢰)만 세웠다.
>
> **③ blob 이름은 워킹트리 파일이 아니라 git이 저장할 내용에서 나온다.** `.gitattributes`에 `text`가 걸린 파일은 체크아웃에서 CRLF가 들어가고 저장소의 blob은 LF다. 읽은 그대로 해시하면 **아무것도 안 고친 워킹트리가 dirty로 보인다** — 코퍼스의 `gradlew.bat`에서 실제로 그랬다(2937 ↔ 2843바이트). git의 clean 필터(CRLF→LF)를 적용한다.

**왜 이것이 공짜로 성립하는가** — 1층 캐시 키가 `(blob_hash, extractor_version)`이지 커밋이 아니다. 워킹트리 파일의 blob 해시를 직접 계산하면 파싱 파이프라인은 커밋을 **전혀 모른 채** 그대로 돈다. 커밋 축이 필요한 곳은 좌표 표기와 결박뿐이다.

`tree_digest` 계산 비용을 낮추는 방법(10⁵ 파일에서 매 질의마다 재계산하면 안 된다):
1. git 인덱스의 (mtime, size, blob_hash) 캐시를 먼저 신뢰한다 — git 자신이 쓰는 방법.
2. 인덱스가 stale인 파일만 실제로 해시한다.
3. 결과를 `.palimpsest/worktree.state`에 캐시하고 인덱스 mtime으로 무효화.

### 3.3 파일 분류 — 7상태

```rust
pub enum FileState {
    Parsed { language: LanguageId, grade: ExtractGrade },
    Partial { language: LanguageId, grade: ExtractGrade, recovery: Vec<Site> },
    Unsupported { language: LanguageId },      // 언어는 알지만 추출기가 없다
    Unrecognized,                              // 언어를 모른다
    Excluded { rule_id: ExclusionRuleId },     // 규칙 ID 필수 — 나중에 이 규칙이 ScopeReduction 이 된다
    Binary { reason: BinaryReason },
    Generated { evidence: GeneratedEvidence },
}
```

**`Excluded`가 규칙 ID를 갖는 이유**: 제외 규칙을 넓히면 판정 대상이 줄고 "잔여가 줄었다"로 보인다. 그것이 게이트 오염의 형태다. 규칙 ID가 있어야 나중에 "범위가 줄어서 사라진 것"을 "판정되어 사라진 것"과 구별할 수 있다.

**언어 인식 순서**: ① 확장자 ② 셔뱅 ③ `.gitattributes`의 `linguist-language` ④ 내용 휴리스틱(최후).

> **정정 둘 (2026-08-13 · F01 종료).**
>
> **③이 ①보다 앞선다.** `linguist-language`는 사람이 그 파일에 대해 **선언한** 것이고 확장자는 규약일 뿐이다. 위 순서를 그대로 두면 선언이 규약에 져서 §5의 *"사용자가 매니페스트로 덮어쓸 수 있게"*가 성립하지 않는다 — 덮어쓸 수 없는 것은 덮어쓰기가 아니다.
>
> **④를 세우지 않았다.** 실물 코퍼스에서 ④가 켤 수 있는 것은 `…Test.kt.bak` 하나뿐이고, 그것을 Kotlin으로 인식하면 **백업 파일이 추출 대상이 된다.** `recognize.rs`가 적은 원칙이 그대로 걸린다 — *"틀리게 인식하는 것보다 모른다고 적는 것이 낫다."* 실물에서 ②가 켜는 것은 `gradlew` 하나(`unrecognized` 7 → 6)이고, ③은 이 코퍼스에서 하중이 0이라 **픽스처에서만 시험된다.** `Generated` 판정은 경로 패턴(`dist/`, `*.generated.*`)과 파일 머리의 생성 표식 주석 둘 다를 증거로 요구한다 — 추측으로 파일을 범위 밖에 두지 않는다.

### 3.4 언어 능력 등급

| 등급 | 뜻 | `identity_grade` |
|---|---|---|
| L0 | 텍스트만 — 심볼을 만들 수 없다 | 없음 |
| L1 | 구조(선언·포함 관계) | `ordinal` |
| L2 | 스코프 해소된 참조 | `exact` |
| L3 | 정의-사용(읽기/쓰기 집합) | `exact` |
| L4 | 제어흐름(경로·지배관계) | `exact` |

등급은 **언어별로** 선언되고 대장이 표로 동반한다. L0 언어가 있으면 대장 머리에 **"결박 불가 언어 N개"** 가 적힌다 — 그 파일들에서는 결박·적시 제시가 성립하지 않는다는 사실을 숨기지 않는 형태.

### 3.5 매니페스트

```toml
# .palimpsest/manifest.toml — 출처는 asserted(사람이 선언)
[[repo]]
id     = "order-svc"          # 안정 식별자. 경로도 원격 URL도 아니다 (R-08)
path   = "."
[repo.exclude]
rules  = [{ id = "vendor",   glob = "vendor/**" },
          { id = "fixtures", glob = "**/__fixtures__/**" }]
```

"어떤 저장소들이 한 프로젝트인가"는 코드에 없다. 그러므로 매니페스트는 `asserted`이고, **대장은 항상 "선언된 저장소 N개"를 머리에 적는다.** 저장소 하나가 빠지면 그것을 지나는 경로가 조용히 사라지는 대신 대장이 계속 말한다. (완전한 해법이 아니다 — 애초에 빠뜨린 저장소는 여전히 안 보인다.)

---

## 4. 데이터

```rust
pub struct Ledger {
    snapshot: Snapshot,
    repos_declared: NonZeroUsize,
    entries: Vec<LedgerEntry>,              // 파일당 1
    language_table: Vec<LanguageCapability>, // (언어, 추출등급, identity_grade, 파일 수)
    detector_freshness: DetectorFreshness,   // 마지막 재추출 Snapshot · 추출기 버전 · 이후 커밋 수
    scope: ScopeSource,                      // 선언인가 추정인가 (2026-08-13 신설)
}
```

**`detector_freshness`가 있는 이유**: 낡음 감지기 자신이 낡을 수 있다. 감지기가 3주 낡았으면 낡음 표시들도 3주 낡았다는 사실이 응답에 붙는다. 이 검사는 상수 시간(HEAD 비교)이므로 무한 후퇴하지 않는다.

> **정정 (2026-08-13 · F01 종료) — 이 문단이 자기 안에서 어긋나 있었다.** 위 필드 주석은 *"마지막 재추출 Snapshot · 추출기 버전 · **이후 커밋 수**"*를 요구하는데 같은 문단이 *"상수 시간(HEAD 비교)"*이라고 못 박는다. **커밋 수를 세는 것은 상수 시간이 아니다** — 이력 깊이에 비례하고, 그러면 예산이 필요하고, 예산은 [DESIGN §12.4](../../DESIGN.md)에 값이 있어야 켜진다(D16). 그래서 상수 시간에 답할 수 있는 것만 싣는다: **문법 커밋 · 추출기 버전 · 지금 HEAD**. 대장이 선 트리와 HEAD를 대는 것이 `Ledger::head_moved()`이고 그것이 상수 시간이다. 커밋 수가 필요해지면 예산과 함께 F05가 낸다.

**`scope`가 함께 있는 이유**: 매니페스트가 없을 때 조용히 경로에서 유도하면 **선언(`asserted`)과 추정이 같아 보인다.** `ScopeSource::{Declared, InferredFromPath}`가 그 둘을 가르고, `Capable`이 산출에 하는 일을 이것이 범위에 한다.

**저장 위치**: 최종적으로 2층 인덱스의 `LEDGER` 테이블이다. 그런데 **2층은 F05이고 이 기능은 그보다 앞선다** — 그래서 F01 단계에서는 `.palimpsest/ledger/<snapshot>.json`으로 두고, F05 도착 시 `LEDGER` 테이블로 이관한다. 이관은 재계산이므로 마이그레이션이 아니다.

**이 기능만으로 무엇이 도나**: `pal ledger` 한 명령. 표면 카탈로그(F06)와 `Envelope`(F05)는 아직 없으므로, **출력은 사람이 읽는 표 하나**다. 카탈로그 등록·`Envelope` 첨부는 각각 F06·F05의 완료 조건이지 여기가 아니다. `Snapshot`마다 재계산되고 이전 것은 유지한다(대장 diff가 나중에 `ScopeReduction` 감지의 입력이 된다).

---

## 5. 이슈와 대응

| 이슈 | 왜 | 대응 | 안 되면 |
|---|---|---|---|
| 워킹트리 좌표 부재 | [R-06](../00-risks.md#r-06) | §3.2 `TreeRef` 확장 | git 인덱스 mtime 캐시로 하향 |
| `tree_digest` 계산 비용 | 10⁵ 파일 stat | git 인덱스 신뢰 + 결과 캐시 | 질의별이 아니라 세션당 1회로 낮춤 |
| gix API 변동 | [R-15](../00-risks.md#r-15) | 트레잇 5개로 격리 | `git` CLI 호출 구현 |
| 언어 오인식 | 확장자만으로는 부족(`.ts`가 TypeScript인가 Qt 번역인가) | 4단계 인식 + 결과를 대장에 노출 | 사용자가 매니페스트로 덮어쓸 수 있게 |
| `Generated` 오판 | 생성물을 놓치면 그래프가 오염되고, 과잉 판정하면 실코드가 사라짐 | 증거 2개(경로 패턴 + 파일 내 표식) 요구 | 기본을 보수적으로(생성물로 안 봄) |
| 거대 파일 | 번들·미니파이 파일이 파서를 멈춤 | 크기 상한(기본 2MB) 초과 시 `Excluded{rule_id="oversize"}` — **규칙 ID로 기록되므로 조용하지 않다** | — |

---

## 6. 고려한 대안

| 대안 | 기각 이유 |
|---|---|
| **`git2`(libgit2)** | C 의존이라 정적 링크 비용이 오른다. 다만 gix가 막히면 즉시 대체 가능한 후보로 유지 |
| **`git` CLI 서브프로세스** | 이식성은 최고지만 blob 하나당 프로세스 생성 비용이 크고, 10⁵ 파일에서 치명적. 폴백으로만 |
| **워킹트리를 무시하고 HEAD만 다룬다** | 1순위 사용 장면(편집 중 적시 제시)이 죽는다. 기각 |
| **파일 상태를 3값(ok/skip/error)으로 단순화** | "언어를 모른다"와 "언어는 아는데 추출기가 없다"는 사용자가 다르게 처리한다(후자는 로드맵, 전자는 설정). 뭉개면 대장이 거짓말한다 |
| **대장을 응답에 붙이지 않고 별도 명령으로만** | 첨부 필수가 이 제품의 정체성. 대신 [R-11](../00-risks.md#r-11)의 요약 한 줄 + 상세 별도 질의로 부피를 옮긴다 |

---

## 7. 검증

- **골든 대장 스냅샷**(`insta`) — 코퍼스별 대장을 커밋. 차이가 나면 diff를 보여주고 승인을 요구한다. **추출 등급 하락 같은 조용한 회귀를 잡는 유일한 장치.**
- **워킹트리 왕복 테스트** — 파일 수정 → 대장 재계산 → 되돌리기 → 대장이 원래대로인가.
- **범위 반증 테스트** — 코퍼스에서 무작위 심볼 100개를 뽑아 "범위 안인데 그래프에 없는 것"이 0인지 확인. (심볼이 F02 산물이므로 **실행은 F02 완료 후**, 검사 자체는 여기서 정의한다.)
- **성능** — 10³~10⁴ 파일 대장 계산 시간 + **파일 수에 대한 선형성**. 10⁵ 게이트는 [R-24](../00-risks.md#r-24)에 따라 P1 종료 시점.

---

## 8. 완료 체크리스트

- [ ] `GitAccess` 트레잇 5개 + gix 구현
- [ ] `TreeRef` 2변형 + 워킹트리 머클 계산 + 캐시
- [ ] `FileState` 7값 + 언어 인식 4단계
- [ ] 언어 능력 등급 표 + L0 언어의 "결박 불가" 표기
- [ ] 매니페스트 로딩 + 제외 규칙 ID
- [ ] `detector_freshness`
- [ ] `pal ledger` 명령 (표 출력. 카탈로그 등록은 F06)
- [ ] **CI 1단계 켜기** — 의존 방향 · 어휘 금지(ripgrep) · `forbid(unsafe_code)` · `cargo-deny` · 의도 저장소 폐기 경로 부재 ([스택 §4.3](../00-stack.md#43-검사를-언제-켜는가--전부-첫-커밋에-켜지-않는다))
- [ ] 골든 대장 스냅샷 커밋
- [ ] 10³~10⁴ 벤치 + 선형성 기록
