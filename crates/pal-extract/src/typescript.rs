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

use std::collections::HashMap;

use pal_core::{
    BodyDigest, BoundSymbol, Capable, Containment, ExportSet, ExtractGrade, FileGraph, IdentityGrade,
    ImportSet, Language, LanguageId, LocalIx, RecoverySite, RefResolution, ScopeIx, Span, Symbol,
    SymbolKind,
};
use tree_sitter::Node;

use crate::extractor::LanguageExtractor;
use crate::parse::{ExtractError, normalize, normalize_erasing, parse_with, recovery_sites};
use crate::scopes::{self, Scoped};

/// 레지스트리가 잡는 자리. **무상태다** — #49 가 이것을 `par_iter` 안에서 부른다.
pub(crate) static TYPESCRIPT: TypeScriptExtractor = TypeScriptExtractor;

/// **벗길 래퍼 넷** — `[f10.6].attachment_ruling` 처분 (다).
///
/// `export` 는 **가시성**을, `const`/`let`/`var` 는 **저장 종류**를 적을 뿐
/// **선언이 아니다.** 심볼은 안쪽 마디에서 선다 — `export_statement` 는
/// [`TsWalk::visit_export`] 를 거쳐 안쪽 선언에서, `lexical_declaration` 은
/// [`TsWalk::visit_declarators`] 를 거쳐 `variable_declarator` 에서.
/// **그 차이를 안 지우면 주석의 좌표가 한 바이트 어긋나 미결박이 된다**(#62).
const 래퍼: [&str; 4] = [
    "export_statement",
    "ambient_declaration",
    "lexical_declaration",
    "variable_declaration",
];

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

    fn marked_comments(
        &self,
        source: &[u8],
        markers: &[&str],
    ) -> Result<Vec<crate::parse::MarkedComment>, ExtractError> {
        let language = tree_sitter::Language::new(tree_sitter_typescript::LANGUAGE_TYPESCRIPT);
        let tree = crate::parse::parse_with(&language, source)?;
        Ok(crate::parse::marked_comments(tree.root_node(), source, markers, &래퍼))
    }
}

/// 선언 목록 · 포함 관계 · export/import · 회복 지점.
///
/// # Errors
/// 문법을 붙이지 못하거나 파싱이 중단되면 [`ExtractError`]. **깨진 소스는 오류가
/// 아니다** — 부분 결과와 회복 지점 수가 함께 나온다.
pub fn extract_detailed(source: &[u8]) -> Result<FileGraph, ExtractError> {
    let language = tree_sitter::Language::new(tree_sitter_typescript::LANGUAGE_TYPESCRIPT);
    // **스레드당 파서를 재사용한다**(#49 · F02 §3.1). `Parser::new()` 는 싸지 않다.
    let tree = parse_with(&language, source)?;

    let mut walk = Walk::new(source);
    walk.children(tree.root_node(), Scope::Module, None)?;

    // **선언 순회가 끝난 뒤에 스코프를 세운다.** 순서가 규율이다 — 스코프가 심볼 목록을
    // 건드리면 #46 의 리콜 172 개가 움직인다. 여기서 늘어나는 것은 각 심볼의 `identity`
    // 와 그것이 정하는 `body_digest` 뿐이다.
    let symbol_at: HashMap<usize, LocalIx> = walk
        .symbols
        .iter()
        .enumerate()
        .map(|(i, p)| (p.node.start_byte(), LocalIx(u32::try_from(i).unwrap_or(u32::MAX))))
        .collect();
    let scoped = scopes::build(tree.root_node(), source, &symbol_at);

    Ok(walk.finish(recovery_sites(tree.root_node()), scoped))
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

/// 순회가 낸 선언 하나 — **아직 요약이 없다.**
///
/// # 왜 [`Symbol`] 을 바로 만들지 않는가
///
/// 요약(`body_digest`)은 **그 심볼의 등급에 의존한다**(#48 · R-22). `exact` 인 심볼에서는
/// 지역 이름을 지우고 `ordinal` 에서는 지우지 않는데, 등급은 **파일 전체를 다 보아야**
/// 정해진다 — 순회 도중에는 그 심볼 안에 아직 안 본 구조가 남아 있다.
///
/// 그래서 순회는 **자리만 잡고**, 요약은 순회가 끝난 뒤 [`Walk::finish`] 가 만든다.
/// 이 커밋에서는 산출이 그대로다 — 계산 시점만 옮겼다.
struct Pending<'t> {
    name: String,
    kind: SymbolKind,
    node: Node<'t>,
}

struct Walk<'a, 't> {
    source: &'a [u8],
    symbols: Vec<Pending<'t>>,
    contains: Vec<Containment>,
    exports: ExportSet,
    imports: ImportSet,
}

impl<'a, 't> Walk<'a, 't> {
    fn new(source: &'a [u8]) -> Self {
        Self {
            source,
            symbols: Vec::new(),
            contains: Vec::new(),
            exports: ExportSet::default(),
            imports: ImportSet::default(),
        }
    }

    fn finish(mut self, recovery_sites: Vec<RecoverySite>, scoped: Scoped) -> FileGraph {
        // **집합이므로 정렬·중복 제거한다.** 소스 순서에 의존하면 `export {a}` 와
        // `export {a}` 두 번이 다른 값을 내고, 그러면 포매터가 export 를 재배열할 때
        // 산출이 움직인다 — `[f02.1.pass]` ③ 의 반대 방향이 무너지는 자리다.
        for v in [&mut self.exports.names, &mut self.exports.star_from, &mut self.imports.modules] {
            v.sort_unstable();
            v.dedup();
        }

        let symbols = self
            .symbols
            .iter()
            .map(|p| {
                let span = span_of(p.node);
                let identity = grade_of_symbol(&scoped, span.byte_start, span.byte_end);
                Symbol {
                    name: p.name.clone(),
                    kind: p.kind,
                    body: digest_of(&scoped, p.node, self.source, identity),
                    span,
                    identity,
                }
            })
            .collect();

        // **정렬·중복 제거가 끝난 뒤에 잰다.** 소스 순서 위에서 재면 포매터가 export 를
        // 재배열하는 것만으로 의존 파일 전체가 무효화된다(R-05).
        let export_digest = Capable::Present(self.exports.digest());
        FileGraph {
            language: LanguageId::new(Language::TypeScript.name()),
            grade: crate::grade_of(Language::TypeScript),
            symbols,
            contains: self.contains,
            exports: Capable::Present(self.exports),
            imports: Capable::Present(self.imports),
            export_digest,
            scopes: Capable::Present(scoped.chain),
            recovery_sites,
        }
    }

    /// 이름 있는 자식을 차례로 본다. 돌려주는 것은 **이 층에서 직접 낸 심볼**이다 —
    /// `export` 가 무엇을 내보냈는지 알아야 하기 때문이고, 중첩된 것은 포함되지 않는다.
    fn children(
        &mut self,
        node: Node<'t>,
        scope: Scope,
        container: Option<LocalIx>,
    ) -> Result<Vec<LocalIx>, ExtractError> {
        let mut cursor = node.walk();
        let kids: Vec<Node<'t>> = node.named_children(&mut cursor).collect();
        let mut emitted = Vec::new();
        for child in kids {
            emitted.extend(self.visit(child, scope, container)?);
        }
        Ok(emitted)
    }

    fn visit(
        &mut self,
        node: Node<'t>,
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
        node: Node<'t>,
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
        node: Node<'t>,
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
        self.symbols.push(Pending { name, kind, node });
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
        node: Node<'t>,
        scope: Scope,
        container: Option<LocalIx>,
    ) -> Result<Vec<LocalIx>, ExtractError> {
        let mut cursor = node.walk();
        let declarators: Vec<Node<'t>> = node
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
        node: Node<'t>,
        scope: Scope,
        container: Option<LocalIx>,
    ) -> Result<Vec<LocalIx>, ExtractError> {
        let module = self.record_source_module(node);

        let mut cursor = node.walk();
        let kids: Vec<Node<'t>> = node.children(&mut cursor).collect();
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

/// **이 심볼에서 실제로 도달한 등급** — `[f02.3.pass]` ② 가 판정하는 자리.
///
/// # `Exact` 의 조건 둘 — 그리고 **`OutsideFile` 은 실패가 아니다**
///
/// 1. 이 심볼 안에서 **이름을 못 잡은 바인딩이 없다.** 구조 분해(`const {a,b} = x`)가
///    하나라도 있으면 본문의 어떤 이름이 그것을 가리키는지 알 수 없고, **모르면 지우면
///    안 된다**(R-22 가 경고한 *"서로 다른 코드가 같은 digest"*).
/// 2. 이 심볼 안에 **선언 전 참조(TDZ)가 없다.** 그것은 우리가 해소에 실패한 자리다.
///
/// `import` 와 전역을 가리키는 참조([`RefResolution::OutsideFile`])는 **실패가 아니다.**
/// 그것을 실패로 세면 import 를 쓰는 거의 모든 심볼이 `ordinal` 로 떨어지고, 그러면 이
/// 등급이 *"이 심볼이 얼마나 자족적인가"* 를 재게 된다 — 지우기의 안전성과 무관한 값이다.
fn grade_of_symbol(scoped: &Scoped, start: usize, end: usize) -> IdentityGrade {
    let unnameable = scoped.unnameable.iter().any(|b| (start..end).contains(b));
    let tdz = scoped
        .chain
        .refs
        .iter()
        .any(|r| (start..end).contains(&r.at) && r.resolved == RefResolution::BeforeDeclaration);
    let measured =
        if unnameable || tdz { IdentityGrade::Ordinal } else { IdentityGrade::Exact };
    // **언어 등급이 선언 상한이다**(`[f02.3.pass]` ②). 실측이 그것을 넘지 못한다 —
    // 넘으면 대장 머리의 언어 표가 심볼이 실제로 가진 것보다 낮은 값을 광고하게 된다.
    measured.min(crate::grade_of(Language::TypeScript).identity())
}

/// 그 심볼의 요약 — **등급이 정규형을 정한다.**
///
/// `Exact` 면 **이 심볼 안에서 선언된, 심볼이 아닌 이름**(지역 변수 · 파라미터 · 타입
/// 파라미터)을 자리 번호로 지운다. `Ordinal` 이면 지우지 않는다.
///
/// # 심볼인 이름은 지우지 않는다
///
/// `class C { m() {} }` 의 `m` 은 `C` 안에서 선언되지만 **그 자체가 심볼이다.** 지우면
/// 메서드 이름을 바꿔도 클래스의 요약이 안 바뀌고, 그것은 정규화가 아니라 정보 손실이다.
///
/// # 자리 번호는 **선언 순서**다
///
/// 참조 순서로 매기면 본문에서 쓰는 순서만 바꿔도 요약이 바뀐다. 선언 순서로 매기면
/// 이름만 바꾼 두 소스가 같은 값을 내고(불변식 A), 선언 순서를 바꾸면 다른 값을 낸다.
fn digest_of(scoped: &Scoped, node: Node<'_>, source: &[u8], identity: IdentityGrade) -> BodyDigest {
    BodyDigest::of_normalized(&normalized_of(scoped, node, source, identity))
}

/// 그 심볼의 **정규형 바이트열.** [`digest_of`] 가 이것을 해싱한다.
///
/// **갈라 둔 이유는 시험이다** — 요약만 내면 *"두 소스가 같은 값을 갖는가"* 밖에 못
/// 묻고, F03 §3.1 이 적어 둔 **정규형 자체**(`function add(#0: number, …)`)가
/// 실물과 같은지는 물을 수 없다. 그것이 [`f03.2.pass`] ③ 의 판정 대상이다.
fn normalized_of(
    scoped: &Scoped,
    node: Node<'_>,
    source: &[u8],
    identity: IdentityGrade,
) -> Vec<u8> {
    if identity != IdentityGrade::Exact {
        return normalize(node, source);
    }
    let (start, end) = (node.start_byte(), node.end_byte());

    // 이 심볼이 **실제로 가리키는, 심볼이 아닌** 바인딩들 — 선언 순서로 자리 번호를 준다.
    //
    // # 「이 심볼 안에서 선언된」이 아니라 「이 심볼이 가리키는」이다
    //
    // 처음에는 선언 자리가 심볼 안인 것만 모았다. 그러면 **중첩된 심볼이 바깥 함수의
    // 지역을 가리킬 때 그 이름이 안 지워진다** — `function outer(){ const out=[];
    // function walk(){ out.push(1) } }` 에서 `walk` 의 요약에 `out` 이 이름 그대로
    // 남고, 바깥의 `out` 을 리네임하면 `walk` 가 `stale` 로 켜진다. 의미는 안 변했는데.
    //
    // ditto 실측에서 그 형태가 **10 건**이었고 전부 중첩 함수였다(`scanLocalJars.walk` ·
    // `walkFiles.walk` · `reduceEvents.effective` …).
    //
    // **선언 순서로 번호를 매기되 가리키는 것만 센다.** 파일 어딘가의 무관한 지역이
    // 늘어도 번호가 밀리지 않는다 — 밀리면 이유 없는 `stale` 이 된다.
    let mut order: Vec<(u32, u32, usize)> = Vec::new(); // (scope, binding, declared_at)
    for r in &scoped.chain.refs {
        if !(start..end).contains(&r.at) {
            continue;
        }
        let RefResolution::Bound { scope, binding } = r.resolved else { continue };
        let Some(b) = scoped
            .chain
            .scopes
            .get(scope.0 as usize)
            .and_then(|s| s.bindings.get(binding as usize))
        else {
            continue;
        };
        if b.symbol != BoundSymbol::NotASymbol {
            continue;
        }
        // **모듈 스코프의 이름은 지우지 않는다** — 거기 있는 「심볼 아닌」 바인딩은
        // 사실상 `import` 뿐이고, **import 이름은 밖에서 온 계약이다.**
        //
        // 지우면 `f(readFile)` 과 `f(readdir)` 이 같은 요약을 갖는다 — 호출 대상
        // 이름을 지우지 않는다는 §3.1 의 줄이 그대로 무너진다. 그리고 import 를
        // **재정렬하는 것만으로** 자리 번호가 뒤바뀌어 요약이 움직인다:
        // ditto 의 `7b571cb3`(*"import 정렬 (구조적)"*)이 실제로 그렇게 켰다.
        //
        // **①(합성 포매팅)은 이것을 못 잡았다** — 리네임 변형이 `exact` 심볼 **안**의
        // 바인딩만 건드리므로 import 를 건드릴 일이 없었다. 잡은 것은 ②(실 이력)의
        // 손 검토다. 두 측정을 가른 F03 §6.2 의 판단이 여기서 값을 냈다.
        if scope == ScopeIx(0) {
            continue;
        }
        if !order.iter().any(|(s, i, _)| *s == scope.0 && *i == binding) {
            order.push((scope.0, binding, b.declared_at));
        }
    }
    order.sort_by_key(|(_, _, at)| *at);
    let number: HashMap<(u32, u32), usize> =
        order.iter().enumerate().map(|(n, (s, b, _))| ((*s, *b), n)).collect();

    let erase = |at: usize| -> Option<usize> {
        // **객체 리터럴의 키는 계약이다** (F03 §4.2). 해소는 그대로 두고 여기서만 막는다.
        if scoped.protected.contains(&at) {
            return None;
        }
        let ix = scoped.ref_at.get(&at)?;
        let RefResolution::Bound { scope, binding } = scoped.chain.refs.get(*ix)?.resolved else {
            return None;
        };
        number.get(&(scope.0, binding)).copied()
    };
    normalize_erasing(node, source, &erase)
}

fn span_of(node: Node<'_>) -> Span {
    Span {
        byte_start: node.start_byte(),
        byte_end: node.end_byte(),
        line_start: u32::try_from(node.start_position().row).unwrap_or(u32::MAX) + 1,
        line_end: u32::try_from(node.end_position().row).unwrap_or(u32::MAX) + 1,
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
    use pal_core::Namespace;

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
    fn 익명은_가장_가까운_조상의_요약에_포함된다() {
        // **정체성 규칙 ②의 뒤쪽 절반이다** (F03 §3.4). 익명이 독립 심볼이 아니라는
        // 것만으로는 부족하다 — 그 본문의 변경이 **어딘가에는** 실려야 하고, 안 실리면
        // 익명 안의 코드가 통째로 감시 밖으로 사라진다(F02 가 넘긴 「입자 부재」).
        let 요약 = |s: &str| 그래프(s).symbols[0].body;
        let a = "export function outer() { const cb = () => { return 1; }; return cb; }";
        let b = "export function outer() { const cb = () => { return 2; }; return cb; }";
        assert_eq!(이름들(a).len(), 1, "익명이 심볼로 샜다");
        assert_ne!(요약(a), 요약(b), "익명 본문이 바뀌었는데 조상의 요약이 그대로다");
    }

    #[test]
    fn 제네릭은_선언_하나가_심볼_하나고_인스턴스화는_심볼이_아니다() {
        // **정체성 규칙 ③** (F03 §3.4).
        let g = 그래프("export function pick<T>(x: T): T { return x; }\nconst a = pick<number>(1);\nconst b = pick<string>('s');\n");
        let names: Vec<&str> = g.symbols.iter().map(|s| s.name.as_str()).collect();
        assert_eq!(names, vec!["pick", "a", "b"], "인스턴스화가 심볼로 샜다");
    }

    #[test]
    fn 타입_파라미터_이름은_exact_에서_지워진다() {
        // 타입 파라미터도 **그 심볼 안에서 선언된 심볼 아닌 이름**이다 — 지우지 않으면
        // `<T>` 를 `<U>` 로 바꾸는 리팩터가 결박을 `stale` 로 켠다.
        let 요약 = |s: &str| 그래프(s).symbols[0].body;
        assert_eq!(
            요약("function pick<T>(x: T): T { return x; }"),
            요약("function pick<U>(x: U): U { return x; }")
        );
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

    fn 등급(src: &str) -> IdentityGrade {
        그래프(src).symbols[0].identity
    }

    /// 첫 심볼의 **정규형** — 사람이 읽는 꼴로.
    fn 정규형(src: &str) -> String {
        let language = tree_sitter::Language::new(tree_sitter_typescript::LANGUAGE_TYPESCRIPT);
        let tree = crate::parse::parse_with(&language, src.as_bytes()).expect("파싱");
        let walk_symbols = 그래프(src).symbols;
        let symbol_at: HashMap<usize, LocalIx> = walk_symbols
            .iter()
            .enumerate()
            .map(|(i, s)| (s.span.byte_start, LocalIx(u32::try_from(i).unwrap_or(u32::MAX))))
            .collect();
        let scoped = scopes::build(tree.root_node(), src.as_bytes(), &symbol_at);
        let first = walk_symbols.first().expect("심볼이 없다");
        let node = node_at(tree.root_node(), first.span.byte_start, first.span.byte_end)
            .expect("선언 노드를 못 찾았다");
        let bytes = normalized_of(&scoped, node, src.as_bytes(), first.identity);
        // 마디 표식은 **구조**이지 글자가 아니다 — 사람이 읽는 꼴에서는 지운다.
        // 그것이 있다는 사실은 아래 `그룹_괄호는...` 시험이 따로 붙든다.
        String::from_utf8_lossy(&bytes)
            .chars()
            .filter_map(|c| match c {
                '\u{1f}' => Some(' '),
                '\u{1e}' => Some('#'),
                '\u{1d}' => Some('@'),
                '\u{1c}' | '\u{1a}' => None,
                other => Some(other),
            })
            .collect::<String>()
            .trim_end()
            .to_owned()
    }

    fn node_at<'t>(node: Node<'t>, start: usize, end: usize) -> Option<Node<'t>> {
        if node.start_byte() == start && node.end_byte() == end {
            return Some(node);
        }
        let mut cursor = node.walk();
        let kids: Vec<Node<'t>> = node.children(&mut cursor).collect();
        drop(cursor);
        kids.into_iter().find_map(|c| node_at(c, start, end))
    }

    #[test]
    fn 정규형이_문서에_적힌_그대로다() {
        // **F03 §3.1 이 예시를 적어 두었다** — 그 문장이 이 시험의 오라클이다:
        //
        // ```
        // 원본:   function add(a: number, b: number) { const s = a + b; return s }
        // 정규형: function add(#0: number, #1: number){const #2=#0+#1 return #2}
        // ```
        //
        // 자리 번호가 **선언 순서**이고 **심볼 안에서 하나로 이어진다**는 것이
        // 여기서 처음 눈에 보인다. `[f03.2.pass]` ③ 의 판정 근거다.
        //
        // **문서의 예시에는 `;` 가 남아 있었고 그것이 §3.1 의 표와 어긋났다** —
        // 같은 표가 *"선택적 세미콜론은 지운다"* 라고 적는다. 예시를 정정했다.
        let n = 정규형("function add(a: number, b: number) { const s = a + b; return s }");
        let 토큰: Vec<&str> = n.split_whitespace().collect();
        assert_eq!(
            토큰.join(""),
            "functionadd(#0:number,#1:number){const#2=#0+#1return#2}",
            "정규형이 문서의 예시와 다르다"
        );
    }

    #[test]
    fn 그룹_괄호는_지워지고_트리_모양은_남는다() {
        // **`prettier` 가 괄호를 지운다.** ditto 에서 심볼 58 개가 그것 하나로
        // 움직였다 — `(a, b) => (cond ? x : y)` → `(a, b) => cond ? x : y`.
        let 요약 = |s: &str| 그래프(s).symbols[0].body;
        assert_eq!(
            요약("const f = (a: N, b: N) => (a < b ? -1 : 1);"),
            요약("const f = (a: N, b: N) => a < b ? -1 : 1;"),
            "그룹 괄호가 요약에 남았다 — 포매터 한 번에 결박이 켜진다"
        );

        // **★ 반대 방향.** 평평한 토큰 열에서 괄호만 지우면 서로 다른 코드가
        // 같아진다 — R-22 가 경고한 충돌 그대로다. 트리 모양이 그것을 막는다.
        assert_ne!(
            요약("const g = (a: N, b: N, c: N) => (a + b) * c;"),
            요약("const g = (a: N, b: N, c: N) => a + b * c;"),
            "괄호를 지우면서 우선순위까지 지웠다"
        );
    }

    #[test]
    fn 자리_번호는_선언_순서이고_참조_순서가_아니다() {
        // 참조 순서로 매기면 **본문에서 쓰는 순서만 바꿔도 요약이 바뀐다.**
        assert!(정규형("function f(a: N, b: N) { return b + a; }").contains("#1 + #0"));
    }

    #[test]
    fn 불변식_a_exact_는_지역_이름을_지운다() {
        // **R-22 의 앞쪽 절반.** 지역 이름을 바꾸는 것은 의미 변경이 아니다 —
        // 안 지우면 `rename` 한 번에 결박이 무더기로 `stale` 이 된다(R-07).
        let 요약 = |s: &str| 그래프(s).symbols[0].body;
        let a = "function f(name: string) { const local = name; return local; }";
        let b = "function f(other: string) { const kept = other; return kept; }";
        assert_eq!(등급(a), IdentityGrade::Exact, "해소할 수 있는 심볼인데 exact 가 아니다");
        assert_eq!(요약(a), 요약(b), "exact 인데 지역 이름이 요약에 남았다");
    }

    #[test]
    fn 불변식_b_ordinal_은_지역_이름을_지우지_않는다() {
        // **R-22 의 뒤쪽 절반이고 이 조각의 다섯째다.** A 만 보면 *"항상 지운다"* 가
        // 만점을 받고, 그것이 곧 **서로 다른 코드가 같은 digest** 다.
        //
        // 구조 분해가 있으면 무슨 이름이 묶였는지 모른다 → `ordinal` → 지우지 않는다.
        let 요약 = |s: &str| 그래프(s).symbols[0].body;
        let a = "function f(src: Src) { const { alpha } = src; return alpha; }";
        let b = "function f(src: Src) { const { beta } = src; return beta; }";
        assert_eq!(등급(a), IdentityGrade::Ordinal, "이름을 못 잡은 바인딩이 있는데 exact 다");
        assert_ne!(요약(a), 요약(b), "ordinal 인데 이름을 지웠다 — R-22 의 충돌 그대로다");
    }

    #[test]
    fn 불변식_c_섀도잉은_다른_선언으로_해소된다() {
        // 안쪽 `x` 와 바깥쪽 `x` 가 **같은 선언으로 해소되면** 둘을 맞바꿔도 요약이 같다.
        let 요약 = |s: &str| 그래프(s).symbols[0].body;
        let 안쪽이_이김 = "function f() { const x = 1; { const x = 2; return x; } }";
        let 바깥이_이김 = "function f() { const x = 1; { const y = 2; return x; } }";
        assert_ne!(요약(안쪽이_이김), 요약(바깥이_이김), "섀도잉이 한 선언으로 뭉개졌다");
    }

    #[test]
    fn 불변식_d_의미가_바뀌면_두_등급_모두에서_요약이_바뀐다() {
        // **A·B 를 동시에 붙든다** — 의미가 변했는데 digest 가 같으면 그것은 정규화가
        // 아니라 정보 손실이다.
        let 요약 = |s: &str| 그래프(s).symbols[0].body;
        let e = "function f(n: string) { const a = n; return a; }";
        assert_eq!(등급(e), IdentityGrade::Exact);
        assert_ne!(요약(e), 요약("function f(n: string) { const a = n; return a + a; }"));
        assert_ne!(요약(e), 요약("function f(n: number) { const a = n; return a; }"));

        let o = "function f(s: S) { const { a } = s; return a; }";
        assert_eq!(등급(o), IdentityGrade::Ordinal);
        assert_ne!(요약(o), 요약("function f(s: S) { const { a } = s; return a + a; }"));
    }

    #[test]
    fn 지역의_자리는_구별된다() {
        // 전부 같은 바이트로 지우면 `f(a, b)` 와 `f(a, a)` 가 같아진다.
        let 요약 = |s: &str| 그래프(s).symbols[0].body;
        assert_ne!(
            요약("function f(a: N, b: N) { return a + b; }"),
            요약("function f(a: N, b: N) { return a + a; }"),
            "지역을 한 가지로 뭉개 지웠다"
        );
    }

    #[test]
    fn 객체_리터럴의_키는_지우지_않는다() {
        // **★ 반대 방향.** F03 §4.2 — *"객체 리터럴 키·구조분해 이름은 지우지 않는다
        // (외부에서 보이는 형태)"*. 축약 속성의 이름은 **동시에 지역 참조이고 키**다.
        let 요약 = |s: &str| 그래프(s).symbols[0].body;
        let a = "function f() { const alpha = 1; return { alpha }; }";
        let b = "function f() { const beta = 1; return { beta }; }";
        assert_eq!(등급(a), IdentityGrade::Exact, "해소되는 심볼인데 exact 가 아니다");
        assert_ne!(요약(a), 요약(b), "축약 속성의 키를 지웠다 — 밖에서 보이는 형태가 사라졌다");
    }

    #[test]
    fn 축약이_아닌_자리의_같은_이름은_여전히_지워진다() {
        // 보호가 **자리 단위**여야 한다. 이름 단위로 막으면 그 이름의 모든 쓰임이
        // 살아남고, 그러면 지역 리네임에서 요약이 움직인다.
        let 요약 = |s: &str| 그래프(s).symbols[0].body;
        assert_eq!(
            요약("function f() { const alpha = 1; return alpha + 1; }"),
            요약("function f() { const beta = 1; return beta + 1; }")
        );
    }

    #[test]
    fn 후행_쉼표는_요약을_바꾸지_않는다() {
        // F03 §3.1 — 스타일이다. `prettier` 가 매일 붙였다 뗀다.
        let 요약 = |s: &str| 그래프(s).symbols[0].body;
        assert_eq!(요약("function f(a: N, b: N) {}"), 요약("function f(a: N, b: N,) {}"));
        assert_eq!(요약("const x = [1, 2];"), 요약("const x = [1, 2,];"));
        assert_eq!(요약("const o = { a: 1, b: 2 };"), 요약("const o = { a: 1, b: 2, };"));
    }

    #[test]
    fn 희소_배열의_자리는_후행_쉼표가_아니다() {
        // `[a,]` 는 길이 1 이고 `[a,,]` 는 2 다. 마지막 쉼표만 지워야 둘이 갈린다.
        let 요약 = |s: &str| 그래프(s).symbols[0].body;
        assert_ne!(요약("const x = [1,];"), 요약("const x = [1,,];"));
    }

    #[test]
    fn 따옴표_종류는_요약을_바꾸지_않는다() {
        // F03 §3.1 — 스타일이다. **`prettier` 가 가장 자주 바꾸는 것이고**,
        // 이스케이프까지 풀지 않으면 따옴표를 뒤집는 변형에서 요약이 움직인다.
        let 요약 = |s: &str| 그래프(s).symbols[0].body;
        assert_eq!(요약("const x = 'hi';"), 요약("const x = \"hi\";"));
        assert_eq!(요약("const x = 'a\"b';"), 요약("const x = \"a\\\"b\";"));
        assert_eq!(요약("const x = '\\u0041';"), 요약("const x = \"A\";"));
    }

    #[test]
    fn 문자열_내용은_여전히_요약을_바꾼다() {
        // **★ 반대 방향.** 따옴표를 벗기면서 내용까지 뭉개면 그것은 의미 손실이다.
        let 요약 = |s: &str| 그래프(s).symbols[0].body;
        assert_ne!(요약("const x = 'a';"), 요약("const x = 'b';"));
        // **리터럴과 식별자가 같아지면 안 된다** — 표식이 그것을 막는다.
        assert_ne!(요약("const x = 'y';"), 요약("const x = y;"));
        // 이어 붙인 둘과 하나짜리도 갈려야 한다.
        assert_ne!(요약("const x = ['a', 'b'];"), 요약("const x = ['ab'];"));
    }

    #[test]
    fn 보간이_있는_템플릿은_벗기지_않는다() {
        // 백틱은 그 자리에서 **보간을 여는 문법**이다. 벗기면 뒤에 오는 표현식이
        // 문자열과 뭉개진다.
        let 요약 = |s: &str| 그래프(s).symbols[0].body;
        assert_ne!(요약("const x = `a${b}c`;"), 요약("const x = 'a';"));
        assert_ne!(요약("const x = `a${b}c`;"), 요약("const x = `a${d}c`;"));
    }

    #[test]
    fn 보간이_없는_템플릿은_문자열과_같다() {
        // `` `x` `` 와 `'x'` 는 값이 같고 린터가 둘을 서로 바꾼다.
        // **ditto 의 `0d0f4aab` 이 실제로 그렇게 결박을 켰다** — ①(합성)이 아니라
        // ②(실 이력)의 손 검토가 잡은 자리다.
        let 요약 = |s: &str| 그래프(s).symbols[0].body;
        assert_eq!(요약("const x = `hi`;"), 요약("const x = 'hi';"));
        assert_eq!(요약("const x = `hi`;"), 요약("const x = \"hi\";"));
        assert_ne!(요약("const x = `hi`;"), 요약("const x = 'bye';"));
    }

    #[test]
    fn 심볼인_이름은_지우지_않는다() {
        // `class C { m() {} }` 의 `m` 은 C 안에서 선언되지만 **그 자체가 심볼이다.**
        // 지우면 메서드 이름을 바꿔도 클래스 요약이 안 바뀐다.
        let 요약 = |s: &str| 그래프(s).symbols[0].body;
        assert_ne!(요약("class C { alpha() {} }"), 요약("class C { beta() {} }"));
    }

    #[test]
    fn 호이스팅과_tdz_가_갈린다() {
        // 함수 선언은 뒤에 있어도 해소되고 `let` 은 아니다. **TDZ 가 이 조각에서 가장
        // 반증 가능한 자리다** — 선언 전 참조를 해소해 버리면 스코프 체인이 아니라 이름 표다.
        let chain = |s: &str| 그래프(s).scopes.into_present().expect("TypeScript 는 스코프를 만든다");

        let 호이스팅 = chain("function outer() { return later(); }\nfunction later() { return 1; }\n");
        let l = 호이스팅.refs.iter().find(|r| r.name == "later").expect("참조가 없다");
        assert!(matches!(l.resolved, RefResolution::Bound { .. }), "함수 선언이 호이스팅되지 않았다");

        let tdz = chain("function outer() { const a = b; const b = 1; return a; }");
        let first = tdz.refs.iter().find(|r| r.name == "b").expect("참조가 없다");
        assert_eq!(first.resolved, RefResolution::BeforeDeclaration, "선언 전 참조가 해소됐다");
    }

    #[test]
    fn import_이름은_지우지_않는다() {
        // **★ 반대 방향.** import 는 밖에서 온 계약이다 — 지우면 `f(readFile)` 과
        // `f(readdir)` 이 같은 요약을 갖는다. 그리고 **재정렬만으로** 요약이 움직인다.
        //
        // ditto 의 `7b571cb3`(*"import 정렬 (구조적)"*)이 실제로 그것을 켰고,
        // **①(합성 포매팅)은 그것을 못 잡았다** — ②(실 이력)의 손 검토가 잡았다.
        let 요약 = |s: &str| 그래프(s).symbols[0].body;
        let a = "import { readFile, readdir } from 'fs';\nexport function f() { return readFile; }\n";
        let b = "import { readdir, readFile } from 'fs';\nexport function f() { return readFile; }\n";
        assert_eq!(요약(a), 요약(b), "import 를 재정렬했는데 요약이 움직였다");

        let c = "import { readFile, readdir } from 'fs';\nexport function f() { return readdir; }\n";
        assert_ne!(요약(a), 요약(c), "다른 import 를 가리키는데 요약이 같다");
    }

    #[test]
    fn 파라미터_목록의_주석은_선언이_아니다() {
        // **tree-sitter 에서 주석은 이름 있는 노드다** — 안 거르면 주석 한 줄이
        // 통째로 「선언된 이름」이 되고 스코프 표가 오염된다. ditto 에서 32 개였다.
        let g = 그래프("function f(\n  a: string,\n  // 왜 이 값이 여기 있는가\n  b: string,\n) { return a + b; }");
        let chain = g.scopes.into_present().expect("스코프");
        let names: Vec<&str> =
            chain.scopes.iter().flat_map(|s| &s.bindings).map(|b| b.name.as_str()).collect();
        assert!(
            !names.iter().any(|n| n.contains("//")),
            "주석이 선언으로 잡혔다: {names:?}"
        );
        assert!(names.contains(&"a") && names.contains(&"b"), "파라미터가 사라졌다");
    }

    #[test]
    fn 중첩된_심볼도_바깥_지역을_지운다() {
        // **처음에는 「이 심볼 안에서 선언된」 것만 지웠다.** 그러면 중첩 함수가
        // 바깥 함수의 지역을 이름 그대로 싣고, 바깥을 리네임하면 안쪽이 `stale` 로
        // 켜진다 — 의미는 안 변했는데. ditto 에서 10 건이 그 형태였다.
        let 요약 = |s: &str, i: usize| 그래프(s).symbols[i].body;
        let a = "export function outer() { const out: N[] = []; function walk() { out.push(1); } walk(); }";
        let b = "export function outer() { const kept: N[] = []; function walk() { kept.push(1); } walk(); }";
        assert_eq!(그래프(a).symbols[1].name, "walk", "중첩 함수가 심볼이 아니다");
        assert_eq!(요약(a, 1), 요약(b, 1), "바깥 지역을 리네임했더니 안쪽 심볼이 움직였다");
    }

    #[test]
    fn 값과_타입_이름_공간이_갈린다() {
        // `interface Foo` 와 `const Foo` 는 공존한다. 뭉개면 해소가 **조용히** 틀린다.
        let g = 그래프("interface Foo { a: string }\nconst Foo = 1;\nconst x: Foo = null;\nconst y = Foo;\n");
        let chain = g.scopes.into_present().unwrap();
        let 타입_자리 = chain.refs.iter().find(|r| r.name == "Foo" && r.namespace == Namespace::Type);
        let 값_자리 = chain.refs.iter().rev().find(|r| r.name == "Foo" && r.namespace == Namespace::Value);
        let (Some(t), Some(v)) = (타입_자리, 값_자리) else { panic!("두 자리가 다 안 잡혔다") };
        let (RefResolution::Bound { binding: tb, .. }, RefResolution::Bound { binding: vb, .. }) =
            (t.resolved, v.resolved)
        else {
            panic!("해소되지 않았다")
        };
        assert_ne!(tb, vb, "타입 자리와 값 자리가 같은 선언으로 해소됐다");
    }

    #[test]
    fn 스코프는_kotlin_이_아니라_typescript_에만_선다() {
        // `[f02.3.does_not_prove].not_kotlin_scope` — Kotlin 은 L1 로 남는다.
        assert!(그래프("const a = 1;").scopes.is_present());
        let kt = crate::kotlin::extract_detailed(b"class A\n").unwrap();
        assert!(!kt.scopes.is_present(), "Kotlin 에 스코프가 딸려 올라갔다");
        assert_eq!(kt.symbols[0].identity, IdentityGrade::Ordinal);
    }

    #[test]
    fn 깨진_소스도_건진다() {
        // 회복을 1급으로 다루는 것은 #47 이다. 여기서는 **버리지 않는다**는 것만 센다.
        let g = 그래프("export function ok() {}\nclass Broken { fun(\n");
        assert!(!g.is_whole(), "깨졌는데 성하다고 했다");
        assert!(g.symbols.iter().any(|s| s.name == "ok"), "성한 선언까지 버렸다");
    }

    #[test]
    fn 이름_없는_기본_내보내기는_심볼이_아니다() {
        let g = 그래프("export default function () {}");
        assert!(g.symbols.is_empty());
        assert!(g.exports.into_present().unwrap().has_default);
    }
}
