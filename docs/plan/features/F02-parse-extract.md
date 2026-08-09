# F02 — 파싱과 심볼 추출 (C1 구조)

| 우선순위 | 의존 | 규모 | 크레이트 |
|---|---|---|---|
| **P0** | F01 | L | `pal-extract` |

---

## 1. 왜

**"무엇이 존재하는가"가 없으면 아무것도 없다.** 파일·모듈·타입·함수와 그 포함 관계. 이것이 없으면 에이전트는 이름 검색으로 코드를 찾고, 검색어에 안 걸리는 것은 존재하지 않는 것이 된다.

**달성 기여**: 모든 것의 토대. 좌표(F03)가 여기서 나오고, 참조(F07)가 여기 위에 선다.

**제약**: 대상을 **빌드하지 않는다.** 툴체인·비밀·환경변수를 갖지 못하고, 임의의 과거 커밋이 빌드된다는 보장도 없다. 그러므로 blob 하나만으로 파싱되는 것이 주경로다.

---

## 2. 입력 → 출력

```
입력:  (RepoPath, BlobHash, 내용 바이트, LanguageId)
출력:  FileGraph — 그 파일 하나에서 나온 전부
```

```rust
pub struct FileGraph {
    blob: BlobHash,
    extractor: ExtractorVersion,
    language: LanguageId,
    state: FileState,                    // parsed | partial(회복 지점)
    symbols: Vec<LocalSymbol>,           // 이 파일이 정의하는 것
    contains: Vec<(LocalIx, LocalIx)>,   // 포함 관계 (C1)
    scopes: ScopeTable,                  // 파일 내 스코프 체인 (L2a) — §3.5
    local_refs: Vec<LocalRef>,           // 같은 파일 안에서 해소된 참조 (L2a 산물)
    raw_refs: Vec<RawRef>,               // 파일 밖을 가리키는 참조 — F07 의 입력
    exports: ExportSet,                  // 이 파일이 밖에 노출하는 것
    imports: ImportSet,                  // 이 파일이 참조하는 외부 모듈
    export_digest: Blake3,               // R-05 의 무효화 전파용
}
```

**핵심 성질: `FileGraph`는 파일 하나에만 의존한다.** 다른 파일을 보지 않으므로 완전 병렬이고, 콘텐츠 주소 캐시(F04)의 값이 될 수 있다. **파일 간** 해소는 전부 F07(스티칭)로 미룬다.

**파일 내 스코프 해소(L2a)는 여기 있다** — 그것도 파일 하나만 보는 연산이기 때문이고, 무엇보다 **`body_digest` 정규화가 그것을 요구하기 때문이다**([R-22](../00-risks.md#r-22), §3.5).

---

## 3. 구현

### 3.1 tree-sitter

```rust
// 언어마다 하나. 코어는 이 트레잇만 안다.
pub trait LanguageExtractor: Send + Sync {
    fn language_id(&self) -> LanguageId;
    fn grade(&self) -> ExtractGrade;              // 이 추출기가 실제로 도달하는 등급
    fn ts_language(&self) -> tree_sitter::Language;
    fn extract(&self, tree: &Tree, src: &[u8]) -> Result<FileGraph>;
}
```

- 파서 인스턴스는 **스레드당 재사용**한다(`thread_local!`). `Parser::new()`가 싸지 않고, rayon 워커마다 하나면 충분하다.
- 문법은 `cc` 크레이트로 정적 링크된다. 별도 런타임·동적 라이브러리가 없다.

### 3.2 심볼 추출 — 쿼리가 아니라 순회

tree-sitter의 `Query`(S-expression 패턴)를 쓸 수도 있고 트리를 직접 순회할 수도 있다. **직접 순회를 택한다.**

| | 쿼리 | 직접 순회 |
|---|---|---|
| 선언적이라 짧다 | ✅ | ❌ |
| 컨테이너 체인(중첩 클래스→메서드) 추적 | ❌ 어렵다 | ✅ 자연스럽다 |
| 오류 노드 주변 회복 제어 | ❌ | ✅ |
| 디버깅 | 패턴이 안 맞으면 침묵 | 스택이 보인다 |

컨테이너 체인이 `symbol_id`의 성분(F03)이고 오류 회복이 `partial` 상태의 근거라서, 둘 다 제어해야 한다. 순회로 간다.

```rust
// 커서 기반 DFS. 컨테이너 스택을 들고 내려간다.
fn walk(cursor: &mut TreeCursor, ctx: &mut ExtractCtx) {
    loop {
        let node = cursor.node();
        match classify(node.kind()) {
            Kind::Container(k) => { ctx.push_container(name_of(node), k); /* 자식으로 */ }
            Kind::Declaration(k) => ctx.emit_symbol(node, k),
            Kind::Reference => ctx.emit_raw_ref(node),
            Kind::Error => ctx.emit_recovery_site(node),   // partial 의 근거
            Kind::Skip => {}
        }
        // ...
    }
}
```

### 3.3 TypeScript 추출기 (첫 언어)

| 추출 대상 | 노드 종류 | 비고 |
|---|---|---|
| 함수 선언 | `function_declaration`, `method_definition`, `arrow_function`(이름 있는 바인딩만) | 익명 함수는 **독립 심볼이 아니다** — 가장 가까운 이름 있는 조상에 귀속 |
| 클래스·인터페이스·타입 | `class_declaration`, `interface_declaration`, `type_alias_declaration` | |
| 변수 | `variable_declarator` — 최상위/모듈 스코프만 | 함수 내부 지역 변수는 심볼이 아니다(폭발) |
| enum | `enum_declaration` | |
| 포함 관계 | 컨테이너 스택 | 파일 → 클래스 → 메서드 |
| export | `export_statement`, `export_clause`, `export * from` | `ExportSet` |
| import | `import_statement`, `require()` 호출, 동적 `import()` | `ImportSet`. 동적 import는 리터럴 인자만 |
| 참조 | `call_expression`, `new_expression`, `member_expression`, 타입 주석의 타입 참조 | `RawRef` — 해소는 F07 |

**제네릭**: 선언 하나가 심볼 하나. 인스턴스화는 심볼이 아니다.
**재선언·조건부 컴파일**: 같은 좌표에 둘 이상이면 후보 집합으로 저장(F07의 `candidate`와 같은 처리).

### 3.4 `partial` — 오류 회복을 1급으로

tree-sitter는 파싱 실패 시 `ERROR` 노드를 넣고 계속 진행한다. 이것이 이 프로젝트에 중요하다 — **빌드 안 되는 코드, 미완성 코드, 문법이 못 따라가는 최신 구문에서도 부분 결과를 낸다.**

- `ERROR`/`MISSING` 노드의 `span`을 `Site`로 기록하고 파일 상태를 `Partial`로.
- **회복 지점이 있는 파일의 심볼은 버리지 않는다.** 파싱된 부분에서 나온 심볼은 유효하고, 그 파일에 공백이 있다는 사실이 대장에 남는다.
- `ERROR` 노드 비율이 임계(예: 30%)를 넘으면 `Partial` 대신 `Unsupported`로 낮춘다 — 문법이 그 언어 버전을 못 따라가는 경우.

### 3.5 파일 내 스코프 체인 (L2a) — **여기 있어야 하는 이유** ([R-22](../00-risks.md#r-22))

원래 이것은 F07(P1)에 있었다. 그런데 **F03의 `body_digest` 정규화가 이것 없이는 성립하지 않는다** — 지역 변수·파라미터명을 지우려면 어느 이름이 어느 선언을 가리키는지 알아야 하고, 그것을 P1까지 미루면 P0에서 만든 좌표와 digest가 F07 완료일에 **전부 이동한다.** 그 사이에 쌓인 결박은 전부 `orphaned`가 된다.

그리고 이 연산은 **파일 하나만 본다.** 1층의 성질을 깨지 않는다. F07에 남는 것은 파일 **간** 연산(L2b 모듈 해소·L2c 멤버)뿐이다.

```rust
pub struct ScopeTable {
    scopes: Vec<Scope>,                       // 블록 · 함수 · 클래스 · 모듈
    // 값 네임스페이스와 타입 네임스페이스가 분리된다 (TypeScript)
    //   interface Foo 와 const Foo 가 공존 가능
}
struct Scope { kind: ScopeKind, parent: ScopeIx, bindings: Vec<(Name, LocalIx)> }
```

다뤄야 하는 것: 호이스팅(함수 선언), TDZ(`let`/`const`), 섀도잉, 두 네임스페이스.

**해소 실패는 심볼 단위로 기록된다.** 파일 안에서 스코프 해소가 안 된 심볼은 `identity_grade = ordinal`로 떨어지고, **그 심볼에서는 지역 변수명을 지우지 않는다**(지우면 서로 다른 코드가 같은 digest를 갖는다). 언어 등급 표는 선언값이고 실제 등급은 심볼에 실린다.

### 3.6 병렬

```rust
let graphs: Vec<FileGraph> = files.par_iter()          // rayon
    .map(|(path, blob)| {
        if let Some(g) = cache.get((blob, ver))? { return Ok(g) }   // F04
        let src = git.read_blob(*blob)?;
        let g = extractor_for(lang).extract(&parse(&src)?, &src)?;
        cache.put((blob, ver), &g)?;
        Ok(g)
    })
    .collect::<Result<_>>()?;
```

파일 간 의존이 없으므로 완전 병렬. 1층 캐시가 파일시스템 콘텐츠 주소라 쓰기 락도 없다.

---

## 4. 이슈와 대응

| 이슈 | 왜 | 대응 | 안 되면 |
|---|---|---|---|
| **문법 버전 드리프트** | tree-sitter 문법이 최신 언어 구문을 못 따라감 | 문법 버전을 `extractor_version`에 넣는다 → 문법을 올리면 좌표가 움직이고 그것이 **관측된다** | `Partial` 비율이 대장에 보이므로 사용자가 안다 |
| **거대 파일** | 번들·미니파이 파일이 파서를 멈춤 | F01의 크기 상한 + 한 줄 길이 상한 | `Excluded{oversize}` |
| **메모리** | 10⁵ 파일 트리를 동시에 들면 터짐 | 파일당 트리를 **즉시 `FileGraph`로 변환하고 버린다.** 트리를 저장하지 않는다 | rayon 청크 크기 조절 |
| **익명 함수 폭발** | 콜백이 많은 코드에서 심볼이 수십 배 | 익명은 심볼이 아니다(가장 가까운 이름 있는 조상에 귀속) | — |
| **`ERROR` 노드가 컨테이너를 삼킴** | 오류 회복이 클래스 전체를 ERROR로 묶으면 그 안의 메서드가 사라짐 | ERROR 노드 **안쪽도 순회**해서 인식 가능한 선언을 건진다. 단 그 심볼들은 `partial` 파일 소속으로 표시 | 회복 지점이 대장에 남으므로 조용하지 않다 |
| **Kotlin** | [R-03](../00-risks.md#r-03) — PSI 대비 구조 일치율 61.2% | 착수 전 측정(P0-preflight T7), 등급을 낮춰 선언 | 등급 L1로 고정하고 그 사실을 대장에 노출 |

---

## 5. 고려한 대안

| 대안 | 기각 이유 |
|---|---|
| **언어별 공식 파서**(tsc, kotlinc) | 빌드/런타임 의존. 임의 커밋에서 균일하게 못 돈다. 정적 바이너리도 불가. **이 결정이 제품 정체성이다** |
| **LSP 서버를 띄워서 질의** | 대상 프로젝트의 의존성 설치를 요구한다. 과거 커밋에서 안 된다. 다만 나중에 `observed` 조달원으로는 가치 있음(F16) |
| **SCIP/LSIF 인덱스만 받아 쓴다** | 그것을 만들려면 빌드가 필요하다. 주경로가 될 수 없다. 보조 입력으로는 채택(F16) |
| **tree-sitter Query 사용** | §3.2 — 컨테이너 체인과 오류 회복 제어를 잃는다 |
| **AST를 캐시에 저장** | 부피가 크고, 우리가 필요한 것은 AST가 아니라 `FileGraph`다. 트리는 즉시 버린다 |
| **함수 내부 지역 변수도 심볼로** | 심볼 수가 한 자릿수 늘고 결박 대상으로서 값이 없다. 필요해지면 `Site`로 가리키면 된다 |

---

## 6. 검증

- **골든 심볼 스냅샷**(`insta`) — 코퍼스 파일별 심볼 목록·포함 관계를 커밋.
- **차등 재추출** — 같은 커밋을 전체 경로와 증분 경로로 추출해 대조. **증분 경로의 버그는 이것 없이는 안 보인다.**
- **`partial` 회복 테스트** — 의도적으로 깨뜨린 파일에서 온전한 부분의 심볼이 나오는가.
- **병렬 결정성** — 같은 입력을 여러 번 병렬 추출해 `FileGraph`가 바이트 단위로 동일한가(순서 의존 버그 검출).
- **심볼 리콜** — 코퍼스 표본 20파일의 선언을 손으로 세어 추출 심볼 수와 대조. 골든 스냅샷은 *변하지 않았음*만 말하고 *빠뜨리지 않았음*은 말하지 않는다.
- **L2a 해소율** — 파일 내 이름 참조 중 스코프 체인이 해소한 비율. **`identity_grade=exact` 비율의 하한이 여기서 정해진다.**
- **벤치** — 10³~10⁴ 파일 콜드 추출 시간과 **파일당 평균 · 파일 수에 대한 선형성**. 10⁵는 [R-24](../00-risks.md#r-24)에 따라 P1 종료 시점의 별도 게이트.

---

## 7. 완료 체크리스트

- [ ] `LanguageExtractor` 트레잇 + 등록 레지스트리
- [ ] TypeScript 추출기 — §3.3 표 전부
- [ ] 컨테이너 스택 순회 + 익명 귀속
- [ ] `partial` 회복 지점 기록 + ERROR 내부 순회
- [ ] **L2a 스코프 체인 + `ScopeTable` (값/타입 네임스페이스 분리) — F03 정규화의 전제**
- [ ] **심볼 단위 `identity_grade` 산출** (해소 실패 심볼은 `ordinal`)
- [ ] `ExportSet`/`ImportSet`/`export_digest` (F07·R-05 준비)
- [ ] rayon 병렬 + 트리 즉시 폐기
- [ ] 골든 스냅샷 + 차등 재추출 + 병렬 결정성 + 심볼 리콜 표본
- [ ] 10³~10⁴ 벤치 + 선형성 기록
- [ ] (후속) Kotlin 추출기 — 착수 전 P0-preflight T7 측정 확인
