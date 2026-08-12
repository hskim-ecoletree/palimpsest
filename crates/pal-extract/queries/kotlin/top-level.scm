; Kotlin 최상위 선언 — S0 의 추출 단위
;
; **이 파일은 Rust 추출기와 CLI 레퍼런스가 함께 쓴다.**
; 대조(criteria.toml [s0])가 "코드 경로"만의 차이가 되려면 쿼리가 공유되어야 한다.
; 한쪽만 고치면 그것은 대조를 사후 조정하는 일이다.
;
; 단위: `source_file` 의 **직계 자식**. 한 매치가 선언 하나다.
; 이 다섯이 Kotlin 최상위 선언 10종(class/interface/object/enum/data/annotation/
; typealias/fun/val/var)을 전부 덮는다 — T7 이 먼저 확인했다.
;
; 문법: tree-sitter-grammars/tree-sitter-kotlin @ 3dea6df (= v1.1.0 = upstream HEAD)
; 이름 필드가 노드마다 다르다:
;   class/function/object → `name:` · type_alias → `type:` · property → variable_declaration 안

(source_file (class_declaration    name: (identifier) @name) @decl)
(source_file (function_declaration name: (identifier) @name) @decl)
(source_file (object_declaration   name: (identifier) @name) @decl)
(source_file (type_alias           type: (identifier) @name) @decl)
(source_file (property_declaration (variable_declaration (identifier) @name)) @decl)
