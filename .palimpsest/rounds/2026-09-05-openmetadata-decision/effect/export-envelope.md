# pal export 봉투 — C1 의 근거

돌린 날: 2026-09-05 · 저장소 palimpsest (사본 아님)
명령: pal export --format cypher --out <파일> --json
---

{
 "answer": {
  "format": "cypher",
  "exported": [
   {
    "label": "File",
    "count": 133
   },
   {
    "label": "Symbol",
    "count": 3018
   },
   {
    "label": "REFERENCES",
    "count": 3
   }
  ],
  "missing": [
   {
    "label": "Actor",
    "why": "not_stored",
    "lives_in": "`pal defect` 가 계산만 하고 저장하지 않는다"
   },
   {
    "label": "Binding",
    "why": "not_stored",
    "lives_in": "의도 저장소 (intent.redb · R-21 로 파일이 갈려 있다)"
   },
   {
    "label": "Change",
    "why": "not_stored",
    "lives_in": "`pal defect` 가 계산만 하고 저장하지 않는다"
   },
   {
    "label": "Defect",
    "why": "not_stored",
    "lives_in": "`pal defect` 가 계산만 하고 저장하지 않는다"
   },
   {
    "label": "Journey",
    "why": "not_built",
    "by": "F19"
   },
   {
    "label": "NarrativeItem",
    "why": "not_stored",
    "lives_in": "이 빌드의 2층에 없다"
   },
   {
    "label": "NarrativeRefusal",
    "why": "not_stored",
    "lives_in": "이 빌드의 2층에 없다"
   },
   {
    "label": "UnresolvedRef",
    "why": "not_built",
    "by": "F08"
   },
   {
    "label": "AUTHORED_BY",
    "why": "not_stored",
    "lives_in": "`pal defect` 가 계산만 하고 저장하지 않는다"
   },
   {
    "label": "BOUND_TO",
    "why": "not_stored",
    "lives_in": "의도 저장소 (intent.redb · R-21 로 파일이 갈려 있다)"
   },
   {
    "label": "FOLLOWS",
    "why": "not_stored",
    "lives_in": "`pal defect` 가 계산만 하고 저장하지 않는다"
   },
   {
    "label": "INTRODUCED_BY",
    "why": "not_stored",
    "lives_in": "`pal defect` 가 계산만 하고 저장하지 않는다"
   },
   {
    "label": "MANIFESTS_AT",
    "why": "not_stored",
    "lives_in": "`pal defect` 가 계산만 하고 저장하지 않는다"
   },
   {
    "label": "RESOLVED_BY",
    "why": "not_stored",
    "lives_in": "`pal defect` 가 계산만 하고 저장하지 않는다"
   },
   {
    "label": "TOUCHES",
    "why": "not_stored",
    "lives_in": "`pal defect` 가 계산만 하고 저장하지 않는다"
   }
  ],
  "bytes": 652954
 },
 "snapshot": [
  [
   "palimpsest",
   {
    "worktree": {
     "base": "a26d468bd890cac7a8813d2d9c8d3126b5640417",
     "tree_digest": "d0cc8acd325f761a97cb46e9e35ee98cd81bf9d97220751b1c8b6fd93b2d0f6f"
    }
   }
  ]
 ],
 "projection": {
  "matches_worktree": {
   "present": true
  },
  "rebuild": {
   "present": "settled"
  },
  "built_for_this_snapshot": true,
  "symbols_indexed": 3018
 },
 "coverage": {
  "unresolved": 0,
  "out_of_scope_files": 883,
  "lowest_grade": "l0",
  "identity": "ordinal"
 },
 "capabilities": {
  "built": [
   "ledger.snapshot",
   "symbol.resolve",
   "symbol.contains",
   "symbol.callers",
   "symbol.reaches",
   "graph.dump",
   "binding.status",
   "narrative.unbound",
   "binding.touch",
   "plan.deviation"
  ],
  "not_built": [
   {
    "feature": "F07",
    "what": "cross-file-resolution"
   },
   {
    "feature": "F08",
    "what": "unresolved-refs"
   },
   {
    "feature": "F13",
    "what": "effects"
   },
   {
    "feature": "F15",
    "what": "judgment"
   }
  ]
 },
 "ledger": {
  "files_total": 1016,
  "parsed": 133,
  "partial": 0,
  "unsupported": 599,
  "unrecognized": 283,
  "unbindable_languages": 7
 },
 "elision": {
  "truncated": [],
  "limits_hit": []
 },
 "fold": {
  "folded": [
   {
    "what": "ledger",
    "count": 1016,
    "unfolded_by": "ledger.snapshot"
   }
  ]
 },
 "log": {
  "status": "not_recorded",
  "why": "read_only_attach"
 },
 "tokens": {
  "serialized_bytes": 2829,
  "bytes_per_token": 4,
  "approx_tokens": 707
 }
}

---

## 재현

`export.cypher` 자체는 652 KB 라 저장소에 안 싣는다. **이 저장소에서** 뜬 것의 다이제스트:

    sha256(export.cypher) = a107370abdcf6cd15f6354761c043d8a8ae68ca2eedccd5f111d2c4dd225c23b

⚠ **사본에서 뜨면 이 값이 안 맞는다** — 봉투의 `snapshot[0][0]` 이 저장소 디렉터리
이름이라 사본에서는 다른 값이 박힌다(ADR-0023 이 *"공유되는 산출물은 기계 고유의 값을
안 싣는다"* 로 못 박은 자리). 다이제스트가 재는 대상은 **`.cypher` 본문**이고 봉투가 아니다.
