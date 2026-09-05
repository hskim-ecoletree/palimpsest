"""palimpsest 의 실제 노드를 OpenMetadata JSON Schema 에 넣어 돌린다.

회차 `2026-09-05-openmetadata-decision` 의 §8 효과. 일회성이다 — 장치로 남기지 않는다.

세 팔로 돈다. **음성 대조가 여기 산다**:

  P (양성 대조) — OpenMetadata 스펙대로 만든 최소 인스턴스. **통과해야 한다.**
                  전부 실패하면 검증기나 스키마 배치가 고장난 것이다.
  T (판정 대상) — palimpsest 의 실제 노드를 가장 가까운 엔티티에 매핑한 것.
  N (음성 대조) — required 를 빼거나 규격 밖 속성을 넣은 것. **실패해야 한다.**
                  전부 통과하면 이 검증기는 아무것도 안 재고 있다.

쓰기:
    python3 fetch-om-schema.py entity/data/contextFile.json entity/data/ontologyAxiom.json type/entityRelationship.json
    python3 validate.py <om-schema 디렉터리> <export.cypher> <bindings.jsonl>
"""
import json, re, sys, pathlib
from referencing import Registry, Resource
from jsonschema import Draft7Validator

ROOT, CYPHER, BINDINGS = (pathlib.Path(a) for a in sys.argv[1:4])

# ── 스키마 배치 — $ref 가 상대경로라 파일 경로를 그대로 URI 로 쓴다 ──────────────
registry = Registry()
for p in ROOT.rglob("*.json"):
    registry = registry.with_resource(
        str(p.resolve().as_uri()), Resource.from_contents(json.load(p.open()), default_specification=__import__("referencing").jsonschema.DRAFT7))

def validator(rel):
    p = (ROOT / rel).resolve()
    return Draft7Validator(json.load(p.open()), registry=registry).evolve(
        _resolver=registry.resolver(base_uri=str(p.as_uri())))

def check(name, arm, rel, inst):
    errs = sorted(validator(rel).iter_errors(inst), key=lambda e: list(e.path))
    ok = not errs
    why = "" if ok else f"{errs[0].message[:150]}"
    return {"이름": name, "팔": arm, "스키마": rel, "통과": ok, "첫 오류": why}

# ── palimpsest 의 실제 노드를 읽는다 ────────────────────────────────────────────
files, symbols = [], []
for line in CYPHER.read_text().splitlines():
    m = re.match(r'CREATE \(:File \{path: "([^"]+)", language: "([^"]+)", grade: "([^"]+)"\}\);', line)
    if m: files.append(dict(zip(("path", "language", "grade"), m.groups())))
    m = re.match(r'CREATE \(:Symbol \{id: "([^"]+)"', line)
    if m: symbols.append({"id": m.group(1), "raw": line})
bindings = [json.loads(l) for l in BINDINGS.read_text().splitlines()[1:] if l.strip()]

UUID = "00000000-0000-4000-8000-000000000001"
rows = []

# ── P — 양성 대조 ──────────────────────────────────────────────────────────────
rows.append(check("최소 contextFile", "P", "entity/data/contextFile.json",
                  {"id": UUID, "name": "spec.md"}))
rows.append(check("최소 entityRelationship", "P", "type/entityRelationship.json",
                  {"fromEntity": "table", "toEntity": "table", "relationshipType": "relatedTo"}))

# ── T — palimpsest 실제 노드 ──────────────────────────────────────────────────
f0 = files[0]
rows.append(check(f"File 노드를 contextFile 로 (path={f0['path']})", "T",
                  "entity/data/contextFile.json",
                  {"id": UUID, "name": f0["path"], "language": f0["language"], "grade": f0["grade"]}))
rows.append(check("File 노드에서 language·grade 를 버리고", "T",
                  "entity/data/contextFile.json", {"id": UUID, "name": f0["path"]}))

s0 = symbols[0]
rows.append(check(f"Symbol 노드를 contextFile 로 (id={s0['id'][:12]}…)", "T",
                  "entity/data/contextFile.json", {"id": UUID, "name": s0["id"]}))

b0 = bindings[0]
rows.append(check(f"결박을 ontologyAxiom 으로 (id={b0['id']})", "T",
                  "entity/data/ontologyAxiom.json",
                  {"id": UUID, "name": b0["id"],
                   "axiomType": "OBJECT_PROPERTY_ASSERTION",
                   "subjectIri": f"urn:pal:decision:{b0['subject']['id']}",
                   "targetIri": f"urn:pal:symbol:{b0['target']}",
                   "description": b0["note"][:80]}))

rows.append(check("결박을 ontologyAxiom 으로 · required 11 개를 전부 채운다", "T",
                  "entity/data/ontologyAxiom.json",
                  {"id": UUID, "name": b0["id"], "displayName": b0["id"],
                   "fullyQualifiedName": f"pal.binding.{b0['id']}",
                   "description": b0["note"][:80],
                   "glossary": {"id": UUID, "type": "glossary"},
                   "axiomType": "OBJECT_PROPERTY_ASSERTION",
                   "subjectIri": f"urn:pal:decision:{b0['subject']['id']}",
                   "targetIri": f"urn:pal:symbol:{b0['target']}",
                   "expressions": [],
                   "provenance": "Derived",
                   "entityStatus": "Approved"}))

ref = next((l for l in CYPHER.read_text().splitlines() if ":REFERENCES]" in l), None)
a, b = re.findall(r'id: "([^"]+)"', ref)[:2]
rows.append(check("REFERENCES 엣지를 entityRelationship 으로", "T",
                  "type/entityRelationship.json",
                  {"fromEntity": "symbol", "toEntity": "symbol",
                   "fromFQN": a, "toFQN": b, "relationshipType": "REFERENCES"}))

# ── N — 음성 대조 ──────────────────────────────────────────────────────────────
rows.append(check("contextFile 에서 required `name` 을 뺀다", "N",
                  "entity/data/contextFile.json", {"id": UUID}))
rows.append(check("contextFile 에 규격 밖 속성 `palimpsestGrade` 를 넣는다", "N",
                  "entity/data/contextFile.json",
                  {"id": UUID, "name": "x", "palimpsestGrade": "L2"}))
rows.append(check("entityRelationship 에서 required `relationshipType` 을 뺀다", "N",
                  "type/entityRelationship.json", {"fromEntity": "a", "toEntity": "b"}))

# ── 더한 축 — 엣지 어휘 대조 ─────────────────────────────────────────────────
# palimpsest 의 엣지 라벨은 `schema/graph.toml` 이 정본이다. 손으로 안 적고 거기서 읽는다.
import tomllib
GRAPH = pathlib.Path(sys.argv[4]) if len(sys.argv) > 4 else pathlib.Path("schema/graph.toml")
edges = [k for k in tomllib.loads(GRAPH.read_text()).get("edge", {})]
enum = json.load((ROOT / "type/entityRelationship.json").open())["definitions"]["relationshipType"]["enum"]
lower = {e.lower(): e for e in enum}
edge_rows = []
for e in edges:
    cand = e.replace("_", "").lower()
    hit = lower.get(cand)
    edge_rows.append((e, hit))

# ── 판정 ──────────────────────────────────────────────────────────────────────
print(f"palimpsest 실측: File {len(files)} · Symbol {len(symbols)} · 결박 {len(bindings)}\n")
print(f"{'팔':<3} {'통과':<5} 이름")
print("─" * 96)
for r in rows:
    print(f"{r['팔']:<3} {'○' if r['통과'] else '×':<5} {r['이름']}")
    if r["첫 오류"]: print(f"{'':<9} └ {r['첫 오류']}")

P = [r for r in rows if r["팔"] == "P"]
N = [r for r in rows if r["팔"] == "N"]
T = [r for r in rows if r["팔"] == "T"]
print("\n── 음성 대조 ──")
print(f"P (통과해야 한다): {sum(r['통과'] for r in P)}/{len(P)} 통과 "
      f"→ {'검증기가 선다' if all(r['통과'] for r in P) else '★ 검증기나 스키마 배치가 고장났다'}")
print(f"N (실패해야 한다): {sum(not r['통과'] for r in N)}/{len(N)} 실패 "
      f"→ {'검증기가 실제로 잰다' if all(not r['통과'] for r in N) else '★ 검증기가 아무것도 안 잰다'}")
print(f"\nT (판정 대상): {sum(r['통과'] for r in T)}/{len(T)} 통과")

print(f"\n── 엣지 어휘 대조 — palimpsest 엣지 {len(edges)} vs OpenMetadata relationshipType enum {len(enum)} ──")
for e, hit in edge_rows:
    print(f"  {e:<16} → {hit if hit else '(없다)'}")
print(f"\n정확히 대응하는 엣지: {sum(1 for _, h in edge_rows if h)}/{len(edge_rows)}")
