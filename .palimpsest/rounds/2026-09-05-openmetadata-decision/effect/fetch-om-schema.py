"""OpenMetadata JSON Schema 를 $ref 를 따라가며 재귀적으로 받는다."""
import base64, json, os, subprocess, sys
from urllib.parse import urlparse

BASE = "openmetadata-spec/src/main/resources/json/schema"
REPO = "open-metadata/OpenMetadata"
OUT = "om-schema"

def fetch(rel):
    """rel: schema 루트 기준 상대경로 (예: 'entity/data/ontologyAxiom.json')"""
    dest = os.path.join(OUT, rel)
    if os.path.exists(dest):
        return False
    os.makedirs(os.path.dirname(dest), exist_ok=True)
    r = subprocess.run(
        ["gh", "api", f"repos/{REPO}/contents/{BASE}/{rel}", "--jq", ".content"],
        capture_output=True, text=True)
    if r.returncode != 0:
        print(f"  못 받음: {rel}", file=sys.stderr)
        return False
    open(dest, "wb").write(base64.b64decode(r.stdout))
    return True

def refs_of(path, rel):
    """파일 안의 $ref 를 schema 루트 기준 상대경로로 바꿔 낸다."""
    out = set()
    def walk(n):
        if isinstance(n, dict):
            for k, v in n.items():
                if k == "$ref" and isinstance(v, str) and not v.startswith("#"):
                    frag = v.split("#")[0]
                    if frag:
                        out.add(os.path.normpath(os.path.join(os.path.dirname(rel), frag)))
                else:
                    walk(v)
        elif isinstance(n, list):
            for v in n: walk(v)
    walk(json.load(open(path)))
    return out

seeds = sys.argv[1:]
todo, seen = list(seeds), set()
while todo:
    rel = todo.pop()
    if rel in seen: continue
    seen.add(rel)
    fetch(rel)
    p = os.path.join(OUT, rel)
    if os.path.exists(p):
        for r in refs_of(p, rel):
            if r not in seen: todo.append(r)
print(f"받은 스키마 파일 {len(seen)}개 → {OUT}/")
