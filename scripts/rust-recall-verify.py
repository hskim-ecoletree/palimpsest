#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""손 표본과 추출기 산출을 **다중집합으로** 댄다 — `[rust.pass]` ①②.

⚠ **집합이 아니라 다중집합이다.** 앞 판은 `set` 을 써서 열쇠가 겹치는 행을 삼켰다 —
산출 268 행을 266 으로 보고했고, 삼켜진 둘이 하필 **R-16(좌표 충돌)이 사는 축**이라
오라클이 자기가 재야 할 축에 눈이 멀어 있었다(독립 리뷰 R1).

    python3 scripts/rust-recall-verify.py --repo ~/dev/projects/cargo [--bin target/release/pal]

**개수로 안 잰다.** 하나를 빠뜨리고 하나를 잘못 잡은 파일이 개수로는 통과한다.
열쇠는 `(path, container, name, kind)` 이고 빠뜨린 것 0 · 잘못 잡은 것 0 이 둘 다 합격선이다.

⚠ **어긋나면 손 표본을 고치지 않는다.** 어긋난 것을 게이트에 목록으로 적는다 —
그것이 이 회차가 등록한 금지역이다(`intent.md` §금지역).
"""

import argparse, collections, json, subprocess, sys
from pathlib import Path

for _s in (sys.stdin, sys.stdout, sys.stderr):
    try:
        _s.reconfigure(encoding="utf-8")
    except (AttributeError, ValueError):
        pass

ROOT = Path(__file__).resolve().parent.parent
표본 = ROOT / "corpus/tasks/rust-recall-sample.tsv"
# **핀은 표본 머리말이 정본이다.** 여기 베끼지 않고 읽는다.
def 핀_읽기() -> str:
    for line in 표본.read_text(encoding="utf-8").splitlines():
        if "코퍼스  rust-lang/cargo @" in line:
            return line.split("@")[1].strip()
    raise SystemExit("표본 머리말에서 코퍼스 핀을 못 찾았다")


def 손표본():
    """(path, container, name, kind) **다중집합** + 파일 목록."""
    keys, files = collections.Counter(), []
    for line in 표본.read_text(encoding="utf-8").splitlines():
        if line.startswith("#") or line.startswith("path\t") or not line.strip():
            continue
        p, _ord, container, name, kind = line.split("\t")
        if p not in files:
            files.append(p)
        if kind == "none":
            continue
        keys[(p, container, name, kind)] += 1
    return keys, files


def 체인(graph, ix, memo):
    """이 심볼의 컨테이너 체인 — `.` 로 이은 조상 이름. 최상위면 `-`."""
    if ix in memo:
        return memo[ix]
    부모 = next((c["parent"] for c in graph.get("contains", []) if c["child"] == ix), None)
    if 부모 is None:
        memo[ix] = "-"
    else:
        위 = 체인(graph, 부모, memo)
        이름 = graph["symbols"][부모]["name"]
        memo[ix] = 이름 if 위 == "-" else f"{위}.{이름}"
    return memo[ix]


# 표본의 `kind` 이름 ↔ `SymbolKind` 의 serde 이름. **둘이 다른 자리이므로 여기서 잇는다.**
KIND = {
    "function": "function", "struct": "struct", "enum": "enum", "trait": "trait",
    "type_alias": "type_alias", "const": "const", "static": "static",
    "module": "module", "macro": "macro", "union": "union",
}


def 산출(binary: Path, repo: Path, files, at: str):
    keys = collections.Counter()
    for rel in files:
        blob = subprocess.run(
            ["git", "-C", str(repo), "show", f"{at}:{rel}"],
            capture_output=True, check=True,
        ).stdout
        tmp = Path("/tmp") / f"_rrv_{abs(hash(rel))}.rs"
        tmp.write_bytes(blob)
        try:
            out = subprocess.run(
                [str(binary), "symbols", "--graph", str(tmp)],
                capture_output=True, check=True, text=True, encoding="utf-8",
            ).stdout
        finally:
            tmp.unlink(missing_ok=True)
        g = json.loads(out)
        memo = {}
        for i, s in enumerate(g["symbols"]):
            keys[(rel, 체인(g, i, memo), s["name"], KIND.get(s["kind"], s["kind"]))] += 1
    return keys


def syn_대조(repo: Path, files, at: str):
    """**음성 대조군** — 다른 파서로 같은 규칙을 적용한다.

    손 표본이 고장이면 여기서 드러난다. 둘이 갈리면 **표본을 고치는 것이 아니라
    갈렸다는 사실을 게이트에 적는다.**
    """
    import tempfile
    oracle = ROOT / "scripts/syn-oracle"
    tmp = Path(tempfile.mkdtemp(prefix="_syn_"))
    mapping = {}
    for i, rel in enumerate(files):
        blob = subprocess.run(["git", "-C", str(repo), "show", f"{at}:{rel}"],
                              capture_output=True, check=True).stdout
        f = tmp / f"{i:02d}.rs"; f.write_bytes(blob); mapping[str(f)] = rel
    out = subprocess.run(
        ["cargo", "run", "-q", "--release", "--manifest-path", str(oracle / "Cargo.toml"),
         "--", *mapping],
        capture_output=True, text=True, encoding="utf-8",
    )
    keys = collections.Counter()
    for line in out.stdout.splitlines():
        if not line.strip():
            continue
        f, c, n, k = line.split("\t")
        keys[(mapping[f], c, n, k)] += 1
    for f in mapping:
        Path(f).unlink(missing_ok=True)
    tmp.rmdir()
    return keys


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--repo", type=Path, required=True, help="cargo 코퍼스 (읽기만 한다)")
    ap.add_argument("--bin", type=Path, default=ROOT / "target/release/pal")
    ap.add_argument("--syn", action="store_true",
                    help="음성 대조군도 돌린다 — 손 표본이 고장이면 여기서 드러난다")
    a = ap.parse_args()

    at = 핀_읽기()
    손, files = 손표본()
    기계 = 산출(a.bin, a.repo, files, at)

    if a.syn:
        syn = syn_대조(a.repo, files, at)
        손_only = sorted((손 - syn).elements()); syn_only = sorted((syn - 손).elements())
        print("── 음성 대조 (syn) ──")
        print(f"  손 표본 {sum(손.values())} · syn {sum(syn.values())}")
        print(f"  손에만 {len(손_only)} · syn 에만 {len(syn_only)}")
        for t, xs in (("손에만", 손_only), ("syn 에만", syn_only)):
            for k in xs[:10]:
                print(f"    {t}\t" + "\t".join(k))
        print()

    # **다중집합 뺄셈** — 겹치는 열쇠가 상쇄돼 사라지지 않는다.
    빠뜨림 = sorted((손 - 기계).elements())
    과잉 = sorted((기계 - 손).elements())

    print(f"코퍼스   cargo @ {at}")
    print(f"표본     파일 {len(files)} · 손 표본 선언 {sum(손.values())} (열쇠 {len(손)})")
    print(f"산출     선언 {sum(기계.values())} (열쇠 {len(기계)})")
    print(f"① 빠뜨린 것  {len(빠뜨림)}")
    print(f"② 잘못 잡은 것 {len(과잉)}")
    n손, n기계 = sum(손.values()), sum(기계.values())
    if n손:
        print(f"재현율   {(n손 - len(빠뜨림)) / n손 * 100:.2f}%")
    if n기계:
        print(f"정밀도   {(n기계 - len(과잉)) / n기계 * 100:.2f}%")

    for 이름, 목록 in (("빠뜨린 것", 빠뜨림), ("잘못 잡은 것", 과잉)):
        if not 목록:
            continue
        print(f"\n── {이름} {len(목록)} ──")
        for k in 목록:
            print(f"  {k[0]}\t{k[1]}\t{k[2]}\t{k[3]}")

    # **rc 는 판정이 아니다** — 게이트가 판정한다. 그러나 어긋남이 있으면 알린다.
    return 1 if (빠뜨림 or 과잉) else 0


if __name__ == "__main__":
    raise SystemExit(main())
