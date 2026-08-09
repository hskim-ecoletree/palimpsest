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
언어      TypeScript  L3  identity=exact   1,180 파일
          SQL         L0  결박 불가        12 파일   ← 이 파일들에는 좌표가 없습니다
```

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

**언어 인식 순서**: ① 확장자 ② 셔뱅 ③ `.gitattributes`의 `linguist-language` ④ 내용 휴리스틱(최후). `Generated` 판정은 경로 패턴(`dist/`, `*.generated.*`)과 파일 머리의 생성 표식 주석 둘 다를 증거로 요구한다 — 추측으로 파일을 범위 밖에 두지 않는다.

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
}
```

**`detector_freshness`가 있는 이유**: 낡음 감지기 자신이 낡을 수 있다. 감지기가 3주 낡았으면 낡음 표시들도 3주 낡았다는 사실이 응답에 붙는다. 이 검사는 상수 시간(HEAD 비교)이므로 무한 후퇴하지 않는다.

**저장 위치**: 2층 인덱스의 `ledger` 테이블. `Snapshot`마다 재계산되고 이전 것은 유지(대장 diff가 나중에 `ScopeReduction` 감지의 입력이 된다).

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
- **범위 반증 테스트** — 코퍼스에서 무작위 심볼 100개를 뽑아 "범위 안인데 그래프에 없는 것"이 0인지 확인.
- **성능** — 10⁵ 파일 대장 계산 시간(목표: 파싱 제외 10초 이내).

---

## 8. 완료 체크리스트

- [ ] `GitAccess` 트레잇 5개 + gix 구현
- [ ] `TreeRef` 2변형 + 워킹트리 머클 계산 + 캐시
- [ ] `FileState` 7값 + 언어 인식 4단계
- [ ] 언어 능력 등급 표 + L0 언어의 "결박 불가" 표기
- [ ] 매니페스트 로딩 + 제외 규칙 ID
- [ ] `detector_freshness`
- [ ] `pal ledger` 명령 (표면 카탈로그에 `ledger.snapshot` 등록)
- [ ] 골든 대장 스냅샷 커밋
- [ ] 10⁵ 파일 벤치 통과
