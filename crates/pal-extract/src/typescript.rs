//! TypeScript 선언 추출 — **쿼리가 아니라 직접 순회**(F02 §3.2).
//!
//! # 왜 순회인가
//!
//! Kotlin 은 쿼리다. 그것으로 되는 이유는 S0 이 **최상위만** 세기 때문이고, 그래서
//! 컨테이너 체인이 필요 없었다. TypeScript 는 다르다 — 클래스 안의 메서드가 심볼이고,
//! 그 포함 관계가 `symbol_id` 의 성분이다(F03). F02 §3.2 의 표가 그 판단이다:
//! 컨테이너 체인 추적과 오류 회복 제어는 쿼리로 어렵고 순회로 자연스럽다.
//!
//! # 세는 단위 — **손 목록이 이것보다 먼저 커밋됐다**
//!
//! `corpus/tasks/f02-recall-sample.tsv` 가 규칙을 파일 머리에 적었고 그 파일은
//! 이 코드보다 먼저 커밋됐다(`[f02.1.oracle]`). 여기 있는 것은 그 규칙의 구현이다 —
//! **반대 방향이 아니다.** 어긋나면 게이트에 목록으로 적고 손 목록을 고치지 않는다.

use pal_core::{
    BodyDigest, Capable, Containment, ExportSet, ExtractGrade, FileGraph, ImportSet, Language,
    LanguageId, LocalIx, Span, Symbol, SymbolKind,
};
use tree_sitter::{Node, Parser};

use crate::extractor::LanguageExtractor;
use crate::parse::{ExtractError, count_error_nodes, normalize};

/// 레지스트리가 잡는 자리. **무상태다** — #49 가 이것을 `par_iter` 안에서 부른다.
pub(crate) static TYPESCRIPT: TypeScriptExtractor = TypeScriptExtractor;

/// TypeScript 추출기.
///
/// **`.tsx` 는 이 문법이 아니다.** `tree_sitter_typescript` 는 `LANGUAGE_TYPESCRIPT` 와
/// `LANGUAGE_TSX` 둘을 낸다. 지금 붙인 것은 앞쪽 하나이고, `.tsx` 를 같은 문법으로
/// 읽으면 JSX 가 통째로 `ERROR` 가 되어 **`partial` 이 문법 부재를 가린다.**
/// 그 자리는 빚으로 남긴다 — 판정은 `docs/gates/F02-1-extractor.md`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TypeScriptExtractor;

impl LanguageExtractor for TypeScriptExtractor {
    fn language(&self) -> Language {
        Language::TypeScript
    }

    fn grade(&self) -> ExtractGrade {
        crate::grade_of(Language::TypeScript)
    }

    fn extract(&self, source: &[u8]) -> Result<FileGraph, ExtractError> {
        extract_detailed(source)
    }
}

/// 선언 목록 · 포함 관계 · export/import · 회복 지점.
///
/// # Errors
/// 문법을 붙이지 못하거나 파싱이 중단되면 [`ExtractError`]. **깨진 소스는 오류가
/// 아니다** — 부분 결과와 회복 지점 수가 함께 나온다.
pub fn extract_detailed(source: &[u8]) -> Result<FileGraph, ExtractError> {
    let language = tree_sitter::Language::new(tree_sitter_typescript::LANGUAGE_TYPESCRIPT);

    let mut parser = Parser::new();
    parser.set_language(&language)?;
    let tree = parser.parse(source, None).ok_or(ExtractError::ParseAborted)?;

    let mut walk = Walk::new(source);
    walk.children(tree.root_node(), Scope::Module, None)?;

    Ok(walk.finish(count_error_nodes(tree.root_node())))
}

/// **모듈 스코프인가.** 이 하나가 *"함수 내부 지역 변수는 심볼이 아니다"* 를 진다.
///
/// 모듈 스코프는 **프로그램의 직계 자식**이다. `export` 는 벗겨서 보고(`declare` 도),
/// 블록은 벗기지 않는다 — `if (…) { const x = … }` 의 `x` 는 블록 스코프다.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Scope {
    Module,
    Inner,
}

/// 이름 자리에 올 수 있지만 **파일 하나만 보고 이름을 정할 수 없는** 것들.
///
/// 구조 분해(`const { a, b } = x`)는 이름이 여럿이고 어느 것이 선언인지 패턴을 풀어야
/// 안다. 계산된 이름(`[expr]() {}`)은 값을 알아야 안다. **모르는 것을 지어내지 않고
/// 심볼을 내지 않는다** — 그 사실은 게이트에 적힌다.
const UNNAMEABLE: [&str; 3] = ["computed_property_name", "array_pattern", "object_pattern"];

struct Walk<'a> {
    source: &'a [u8],
    symbols: Vec<Symbol>,
    contains: Vec<Containment>,
    exports: ExportSet,
    imports: ImportSet,
}

impl<'a> Walk<'a> {
    fn new(source: &'a [u8]) -> Self {
        Self {
            source,
            symbols: Vec::new(),
            contains: Vec::new(),
            exports: ExportSet::default(),
            imports: ImportSet::default(),
        }
    }

    fn finish(mut self, recovery_sites: usize) -> FileGraph {
        // **집합이므로 정렬·중복 제거한다.** 소스 순서에 의존하면 `export {a}` 와
        // `export {a}` 두 번이 다른 값을 내고, 그러면 포매터가 export 를 재배열할 때
        // 산출이 움직인다 — `[f02.1.pass]` ③ 의 반대 방향이 무너지는 자리다.
        for v in [&mut self.exports.names, &mut self.exports.star_from, &mut self.imports.modules] {
            v.sort_unstable();
            v.dedup();
        }
        FileGraph {
            language: LanguageId::new(Language::TypeScript.name()),
            grade: crate::grade_of(Language::TypeScript),
            symbols: self.symbols,
            contains: self.contains,
            exports: Capable::Present(self.exports),
            imports: Capable::Present(self.imports),
            recovery_sites,
        }
    }

    /// 이름 있는 자식을 차례로 본다. 돌려주는 것은 **이 층에서 직접 낸 심볼**이다 —
    /// `export` 가 무엇을 내보냈는지 알아야 하기 때문이고, 중첩된 것은 포함되지 않는다.
    fn children(
        &mut self,
        node: Node<'_>,
        scope: Scope,
        container: Option<LocalIx>,
    ) -> Result<Vec<LocalIx>, ExtractError> {
        let mut cursor = node.walk();
        let kids: Vec<Node<'_>> = node.named_children(&mut cursor).collect();
        let mut emitted = Vec::new();
        for child in kids {
            emitted.extend(self.visit(child, scope, container)?);
        }
        Ok(emitted)
    }

    fn visit(
        &mut self,
        node: Node<'_>,
        scope: Scope,
        container: Option<LocalIx>,
    ) -> Result<Vec<LocalIx>, ExtractError> {
        match node.kind() {
            "import_statement" => {
                self.record_source_module(node);
                Ok(Vec::new())
            }
            "export_statement" => self.visit_export(node, scope, container),
            // `declare …` 는 감쌀 뿐 스코프를 만들지 않는다.
            "ambient_declaration" => self.children(node, scope, container),

            "function_declaration" | "generator_function_declaration" | "function_signature" => {
                self.declare(node, SymbolKind::Function, container, true)
            }
            "class_declaration" | "abstract_class_declaration" => {
                self.declare(node, SymbolKind::Class, container, true)
            }
            "method_definition" => self.declare(node, SymbolKind::Method, container, true),

            // **본문으로 내려가지 않는다** — 인터페이스의 속성·메서드 시그니처와 enum
            // 멤버는 F02 §3.3 의 표에 없다. 내려가면 손 목록에 없는 것이 나온다.
            "interface_declaration" => self.declare(node, SymbolKind::Interface, container, false),
            "type_alias_declaration" => self.declare(node, SymbolKind::TypeAlias, container, false),
            "enum_declaration" => self.declare(node, SymbolKind::Enum, container, false),

            "lexical_declaration" | "variable_declaration" => {
                self.visit_declarators(node, scope, container)
            }

            _ => {
                self.record_dynamic_module(node);
                // **여기서 모듈 스코프가 끝난다.** 블록·문장·표현식 안쪽은 전부 `Inner` 다.
                self.children(node, Scope::Inner, container)?;
                Ok(Vec::new())
            }
        }
    }

    /// 선언 하나를 내고, 필요하면 그 아래로 내려간다(컨테이너를 자기로 바꿔서).
    fn declare(
        &mut self,
        node: Node<'_>,
        kind: SymbolKind,
        container: Option<LocalIx>,
        descend: bool,
    ) -> Result<Vec<LocalIx>, ExtractError> {
        let ix = self.emit(node, kind, container)?;
        if descend {
            self.children(node, Scope::Inner, ix.or(container))?;
        }
        Ok(ix.into_iter().collect())
    }

    fn emit(
        &mut self,
        node: Node<'_>,
        kind: SymbolKind,
        container: Option<LocalIx>,
    ) -> Result<Option<LocalIx>, ExtractError> {
        let Some(name_node) = node.child_by_field_name("name") else {
            // 이름 없는 선언(`export default function () {}`)은 심볼이 아니다.
            return Ok(None);
        };
        if UNNAMEABLE.contains(&name_node.kind()) {
            return Ok(None);
        }
        let name = name_node.utf8_text(self.source).map_err(|_| ExtractError::NotUtf8)?.to_owned();

        let Ok(ix) = u32::try_from(self.symbols.len()) else {
            // 심볼이 40 억 개인 파일. 자리를 만들 수 없으면 내지 않는다.
            return Ok(None);
        };
        let ix = LocalIx(ix);
        self.symbols.push(Symbol {
            name,
            kind,
            // **`export` 키워드는 선언 노드 밖이다** — 그래서 `export` 를 떼도 이 요약은
            // 안 바뀌고, 바뀌는 것은 `ExportSet` 이다. 그 둘을 가르는 것이 음성 대조다.
            body: BodyDigest::of_normalized(&normalize(node, self.source)),
            span: Span {
                byte_start: node.start_byte(),
                byte_end: node.end_byte(),
                line_start: u32::try_from(node.start_position().row).unwrap_or(u32::MAX) + 1,
                line_end: u32::try_from(node.end_position().row).unwrap_or(u32::MAX) + 1,
            },
        });
        if let Some(parent) = container {
            self.contains.push(Containment { parent, child: ix });
        }
        Ok(Some(ix))
    }

    /// `const a = 1, b = 2` — **선언자 하나가 심볼 하나다.**
    ///
    /// 모듈 스코프가 아니면 내지 않는다. 그래도 값 안으로는 내려간다 — 동적 `import()`
    /// 와 중첩 선언이 거기 있을 수 있고, 그것들은 스코프와 무관하게 존재한다.
    fn visit_declarators(
        &mut self,
        node: Node<'_>,
        scope: Scope,
        container: Option<LocalIx>,
    ) -> Result<Vec<LocalIx>, ExtractError> {
        let mut cursor = node.walk();
        let declarators: Vec<Node<'_>> = node
            .named_children(&mut cursor)
            .filter(|c| c.kind() == "variable_declarator")
            .collect();
        drop(cursor);

        let mut emitted = Vec::new();
        for d in declarators {
            let ix = if scope == Scope::Module {
                self.emit(d, SymbolKind::Variable, container)?
            } else {
                None
            };
            emitted.extend(ix);
            if let Some(value) = d.child_by_field_name("value") {
                self.visit(value, Scope::Inner, ix.or(container))?;
            }
        }
        Ok(emitted)
    }

    fn visit_export(
        &mut self,
        node: Node<'_>,
        scope: Scope,
        container: Option<LocalIx>,
    ) -> Result<Vec<LocalIx>, ExtractError> {
        let module = self.record_source_module(node);

        let mut cursor = node.walk();
        let kids: Vec<Node<'_>> = node.children(&mut cursor).collect();
        drop(cursor);

        let mut star = false;
        for child in &kids {
            match child.kind() {
                "default" => self.exports.has_default = true,
                "*" => star = true,
                _ => {}
            }
        }

        let mut named = Vec::new();
        for child in &kids {
            match child.kind() {
                "export_clause" => named.extend(self.clause_names(*child)?),
                // `export * as ns from '…'` — 별이지만 이름이 있다.
                "namespace_export" => {
                    star = false;
                    named.extend(self.clause_names(*child)?);
                }
                _ => {}
            }
        }

        // `export * from '…'` — **무슨 이름이 나가는지 이 파일만 보고는 모른다.**
        // 대상 모듈로 남긴다. 푸는 것은 F07(스티칭)이다.
        if star && let Some(m) = module {
            self.exports.star_from.push(m);
        }

        let emitted = if let Some(decl) = node.child_by_field_name("declaration") {
            self.visit(decl, scope, container)?
        } else if let Some(value) = node.child_by_field_name("value") {
            // `export default <표현식>` — 이름 있는 선언이 아니다. 안쪽만 훑는다.
            self.visit(value, Scope::Inner, container)?
        } else {
            Vec::new()
        };
        for ix in &emitted {
            named.push(self.symbols[ix.0 as usize].name.clone());
        }

        self.exports.names.extend(named);
        Ok(emitted)
    }

    /// `export { a, b as c }` · `export * as ns` 의 **밖으로 나가는 이름**.
    ///
    /// 별칭이 있으면 별칭이다 — 밖에서 보이는 것이 그것이다.
    fn clause_names(&self, clause: Node<'_>) -> Result<Vec<String>, ExtractError> {
        let mut cursor = clause.walk();
        let specs: Vec<Node<'_>> = clause.named_children(&mut cursor).collect();
        drop(cursor);

        let mut out = Vec::new();
        for spec in specs {
            let node = if spec.kind() == "export_specifier" {
                spec.child_by_field_name("alias").or_else(|| spec.child_by_field_name("name"))
            } else {
                Some(spec)
            };
            if let Some(n) = node
                && !UNNAMEABLE.contains(&n.kind())
            {
                out.push(n.utf8_text(self.source).map_err(|_| ExtractError::NotUtf8)?.to_owned());
            }
        }
        Ok(out)
    }

    /// `import … from '<모듈>'` · `export … from '<모듈>'` 의 지정자.
    fn record_source_module(&mut self, node: Node<'_>) -> Option<String> {
        let module = string_text(node.child_by_field_name("source")?, self.source)?;
        self.imports.modules.push(module.clone());
        Some(module)
    }

    /// 동적 `import('<모듈>')` 과 `require('<모듈>')` — **리터럴 인자만.**
    ///
    /// 인자가 변수면 그것이 어느 모듈인지 파일 하나만 보고 알 수 없다. 지어내지 않는다.
    fn record_dynamic_module(&mut self, node: Node<'_>) {
        if node.kind() != "call_expression" {
            return;
        }
        let Some(f) = node.child_by_field_name("function") else { return };
        let dynamic = f.kind() == "import"
            || (f.kind() == "identifier" && f.utf8_text(self.source) == Ok("require"));
        if !dynamic {
            return;
        }
        let Some(args) = node.child_by_field_name("arguments") else { return };
        let mut cursor = args.walk();
        let first: Vec<Node<'_>> = args.named_children(&mut cursor).take(1).collect();
        drop(cursor);
        if let Some(m) = first.first().and_then(|a| string_text(*a, self.source)) {
            self.imports.modules.push(m);
        }
    }
}

/// 문자열 리터럴의 **내용** — 따옴표를 뺀 것.
///
/// 빈 문자열(`''`)은 조각 노드가 없다. `None` 이 아니라 빈 문자열이어야 한다 —
/// *"모듈 지정자가 빈 문자열"* 과 *"문자열이 아니다"* 는 다르다.
fn string_text(node: Node<'_>, source: &[u8]) -> Option<String> {
    if node.kind() != "string" {
        return None;
    }
    let mut cursor = node.walk();
    let mut out = String::new();
    for child in node.named_children(&mut cursor) {
        if child.kind() == "string_fragment" {
            out.push_str(child.utf8_text(source).ok()?);
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn 그래프(src: &str) -> FileGraph {
        extract_detailed(src.as_bytes()).expect("추출이 실패했다")
    }

    fn 이름들(src: &str) -> Vec<(String, SymbolKind)> {
        그래프(src).symbols.into_iter().map(|s| (s.name, s.kind)).collect()
    }

    fn export_집합(src: &str) -> ExportSet {
        그래프(src).exports.into_present().expect("TypeScript 는 export 를 만든다")
    }

    #[test]
    fn 일곱_종류를_뽑는다() {
        let src = "\
export function f() {}
export class C { constructor() {} m() {} }
export interface I { a: string; b(): void }
export type T = string;
export enum E { A, B }
export const v = 1;
";
        assert_eq!(
            이름들(src),
            vec![
                ("f".to_owned(), SymbolKind::Function),
                ("C".to_owned(), SymbolKind::Class),
                ("constructor".to_owned(), SymbolKind::Method),
                ("m".to_owned(), SymbolKind::Method),
                ("I".to_owned(), SymbolKind::Interface),
                ("T".to_owned(), SymbolKind::TypeAlias),
                ("E".to_owned(), SymbolKind::Enum),
                ("v".to_owned(), SymbolKind::Variable),
            ],
            "인터페이스 멤버(a·b)나 enum 멤버(A·B)가 새어 나왔다면 표에 없는 것을 센 것이다"
        );
    }

    #[test]
    fn 익명_함수는_독립_심볼이_아니다() {
        // **손 목록의 규칙 ① 이다.** `describe`/`test` 콜백이 전부 여기 걸리고,
        // 그래서 표본 01 번 파일은 선언 0 이다.
        let src = "\
import { describe, test } from 'bun:test';
describe('a', () => { test('b', () => { const x = 1; }); });
";
        assert!(이름들(src).is_empty(), "익명 화살표가 심볼로 샜다");
    }

    #[test]
    fn 함수_안의_지역_변수는_심볼이_아니다() {
        let src = "export function f() { const local = 1; let other = 2; }";
        assert_eq!(이름들(src), vec![("f".to_owned(), SymbolKind::Function)]);
    }

    #[test]
    fn 블록_스코프의_const_는_모듈_스코프가_아니다() {
        // **손 목록의 규칙 ③.** 표본 07 번의 `if (import.meta.main) { … }` 가 이 형태다.
        let src = "const top = 1;\nif (true) { const inner = 2; }\n";
        assert_eq!(이름들(src), vec![("top".to_owned(), SymbolKind::Variable)]);
    }

    #[test]
    fn 객체_리터럴의_화살표는_심볼이_아니다() {
        // **손 목록의 규칙 ④.** 표본 10 번의 `run:` · 13 번의 `enforce:` 가 이 형태다.
        let src = "export const cmd = { run: async () => {}, meta: { name: 'x' } };";
        assert_eq!(이름들(src), vec![("cmd".to_owned(), SymbolKind::Variable)]);
    }

    #[test]
    fn 메서드는_클래스에_담긴다() {
        let g = 그래프("class C { m() {} }");
        assert_eq!(g.contains.len(), 1);
        assert_eq!(g.parent_of(LocalIx(1)), Some(LocalIx(0)));
        assert_eq!(g.parent_of(LocalIx(0)), None);
    }

    #[test]
    fn export_를_떼면_export_집합이_바뀐다() {
        // **음성 대조 ③ 의 넷째가 이 자리다** — `ExportSet` 이 상수인 추출을 잡는다.
        let 붙은 = export_집합("export function f() {}");
        let 뗀 = export_집합("function f() {}");
        assert_eq!(붙은.names, vec!["f".to_owned()]);
        assert!(뗀.names.is_empty(), "export 를 뗐는데 export 집합이 그대로다");
    }

    #[test]
    fn 심볼_목록은_export_와_무관하다() {
        // **그리고 심볼 자체는 안 바뀐다** — 둘이 다른 축이라는 것이 요점이다.
        assert_eq!(이름들("export function f() {}"), 이름들("function f() {}"));
    }

    #[test]
    fn 별_재수출은_이름이_아니라_모듈로_남는다() {
        // **모르는 것을 안다고 하지 않는다.** 무슨 이름이 나가는지는 F07 이 푼다.
        let e = export_집합("export * from './m';");
        assert!(e.names.is_empty());
        assert_eq!(e.star_from, vec!["./m".to_owned()]);

        let ns = export_집합("export * as m from './m';");
        assert_eq!(ns.names, vec!["m".to_owned()]);
        assert!(ns.star_from.is_empty(), "이름이 있는데 별로 접었다");
    }

    #[test]
    fn 별칭이_있으면_밖으로_나가는_이름은_별칭이다() {
        assert_eq!(export_집합("const a = 1; export { a as b };").names, vec!["b".to_owned()]);
    }

    #[test]
    fn 동적_import_는_리터럴_인자만_담는다() {
        let g = 그래프(
            "import 'a';\nconst m = './b';\nasync function f() { await import('./c'); await import(m); }\n",
        );
        let imports = g.imports.into_present().unwrap();
        assert_eq!(imports.modules, vec!["./c".to_owned(), "a".to_owned()]);
    }

    #[test]
    fn 구조_분해는_이름_있는_바인딩이_아니다() {
        // 파일 하나만 보고 어느 것이 선언인지 패턴을 풀어야 안다. **지어내지 않는다.**
        assert!(이름들("const { a, b } = x;").is_empty());
    }

    #[test]
    fn 주석과_포매팅은_요약을_바꾸지_않는다() {
        // **음성 대조의 반대 방향.** 어기면 포매터 한 번에 전 심볼이 stale 로 켜진다(R-07).
        let 요약 = |s: &str| 그래프(s).symbols[0].body;
        let 원본 = "export function greet(name: string): string {\n  return 'hi';\n}\n";
        assert_eq!(요약(원본), 요약("export function   greet( name : string ) : string {\n\n\treturn 'hi'\n\n}\n"));
        assert_eq!(요약(원본), 요약("// 인사한다\nexport function greet(name: string): string {\n  /* 왜 */ return 'hi';\n}\n"));
    }

    #[test]
    fn 의미가_바뀌면_요약이_바뀐다() {
        // **양방향이어야 한다.** 앞의 시험만 통과하는 것은 상수를 돌려주는 요약이다.
        let 요약 = |s: &str| 그래프(s).symbols[0].body;
        let 원본 = "function greet(name: string): string { return 'hi'; }";
        assert_ne!(요약(원본), 요약("function greet(name: string): string { return 'bye'; }"));
        assert_ne!(요약(원본), 요약("function greet(name: number): string { return 'hi'; }"));
    }

    #[test]
    fn 변수명은_지우지_않는다() {
        // **R-22** — 스코프 해소(#48)가 없는데 지우면 서로 다른 코드가 같은 요약을 갖는다.
        let 요약 = |s: &str| 그래프(s).symbols[0].body;
        assert_ne!(요약("function f() { const a = 1; }"), 요약("function f() { const b = 1; }"));
    }

    #[test]
    fn 깨진_소스도_건진다() {
        // 회복을 1급으로 다루는 것은 #47 이다. 여기서는 **버리지 않는다**는 것만 센다.
        let g = 그래프("export function ok() {}\nclass Broken { fun(\n");
        assert!(g.recovery_sites > 0, "깨졌는데 성하다고 했다");
        assert!(g.symbols.iter().any(|s| s.name == "ok"), "성한 선언까지 버렸다");
    }

    #[test]
    fn 이름_없는_기본_내보내기는_심볼이_아니다() {
        let g = 그래프("export default function () {}");
        assert!(g.symbols.is_empty());
        assert!(g.exports.into_present().unwrap().has_default);
    }
}
