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
; 문법: BrokkAi/tree-sitter-kotlin @ acb9630 (= brokk-tree-sitter-kotlin 0.4.0)
;       fwcd/tree-sitter-kotlin 계열. 판정과 근거는 docs/gates/G50-kotlin-grammar-pin.md
;
; ⚠ **2026-08-13 · #50 — 어휘가 통째로 바뀌었다.** 옛 핀(tree-sitter-kotlin-ng)은
;   amaanq 의 **다시 쓰기**라 이름 마디가 `identifier` 이고 `name:`·`type:` 필드가
;   있었다. fwcd 계열에는 **그 필드가 아예 없고** 이름 마디가 둘로 갈린다:
;
;     class/object/type_alias → `type_identifier`   function/property → `simple_identifier`
;
;   [g50.pass] ③ 이 미리 정한 규칙 안에서 옮겼다 — **패턴은 다섯 그대로 · 이름 치환만 ·
;   술어를 더하지 않았다 · `source_file` 직계 자식이라는 단위를 안 풀었다.**
;   필드 제약이 사라진 만큼 느슨해졌으므로 **반대 방향을 쟀다**: 이름을 아예 안 보는
;   쿼리와 매치 수가 1,122 파일 전수에서 같다(`scripts/g50-fork-oracle.py`).

(source_file (class_declaration    (type_identifier) @name) @decl)
(source_file (function_declaration (simple_identifier) @name) @decl)
(source_file (object_declaration   (type_identifier) @name) @decl)
(source_file (type_alias           (type_identifier) @name) @decl)
(source_file (property_declaration (variable_declaration (simple_identifier) @name)) @decl)
