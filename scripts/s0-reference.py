#!/usr/bin/env python3
"""S0 CLI 레퍼런스 — 파일별 최상위 선언 수 벡터를 만든다.

**이것이 대조의 한쪽이다.** 다른 쪽은 `pal symbols` 의 Rust 경로이고,
둘은 같은 쿼리 파일(`crates/pal-extract/queries/kotlin/top-level.scm`)과
**같은 문법**을 쓴다. 그래서 차이가 나면 그 차이는 **코드 경로의 것**이다.

**여기에 문법 rev 를 글자로 적지 않는다** — 산출의 머리말이 `--grammar` 클론에게 물어
적는다(문법 rev · 쿼리의 blake2b-128). 적어 두면 핀을 옮긴 회차에 이 문서가 옛 값을
그대로 말하고, 그러면 **산출이 자기 출처를 거짓으로 적는다.**

합격선은 `corpus/criteria.toml` `[s0]` 에 있다 — 불일치 **0건**.

왜 집계가 아니라 벡터인가:
    T7 의 94.30% 는 "선언을 하나라도 뽑는 파일"의 비율이라, 파일당 첫 선언만
    뽑는 고장 난 추출기도 1,058/1,122 를 정확히 재현한다. 벡터는 그것을 잡는다.

사용:
    ./scripts/s0-reference.py --grammar <ts-kotlin_클론> --corpus <s0-corpus.sh_출력> \\
        --query crates/pal-extract/queries/kotlin/top-level.scm --out <출력.tsv>

전제:
    tree-sitter CLI 0.26.12 (T7 과 동일) · 문법 클론이 `tree-sitter build` 된 상태
"""

from __future__ import annotations

import argparse
import hashlib
import os
import subprocess
import sys
from pathlib import Path

BATCH = 160  # 명령행 길이 한도를 넉넉히 피한다


def kt_files(corpus: Path) -> list[Path]:
    return sorted(corpus.rglob("*.kt"))


def isolated(grammar: Path) -> dict:
    """**파서 캐시를 문법 클론마다 가른다.**

    CLI 는 컴파일 결과를 `~/.cache/tree-sitter/lib/<문법이름>.dylib` 에 넣는데,
    Kotlin 문법은 포크가 달라도 **이름이 전부 `kotlin`** 이다. 격리하지 않으면
    다른 클론에서 만든 파서가 실린다 — G50 에서 실제로 그렇게 한 번 틀렸다
    (`scripts/g50-fork-oracle.py` 머리말). **핀을 옮기는 회차에서 특히 위험하다.**
    """
    home = grammar / ".s0-home"
    home.mkdir(exist_ok=True)
    return {**os.environ, "HOME": str(home), "XDG_CACHE_HOME": str(home / ".cache")}


def parse_failures(grammar: Path, files: list[Path]) -> set[Path]:
    """`tree-sitter parse --quiet` 는 **실패한 파일만** 한 줄씩 낸다."""
    listing = grammar / ".s0-paths.txt"
    listing.write_text("\n".join(str(f) for f in files), encoding="utf-8")
    try:
        out = subprocess.run(
            ["tree-sitter", "parse", "--quiet", "--paths", str(listing)],
            cwd=grammar, capture_output=True, text=True, check=False,
            env=isolated(grammar),
        ).stdout
    finally:
        listing.unlink(missing_ok=True)

    failed = set()
    for line in out.splitlines():
        head = line.split("\t", 1)[0].strip()
        if head.endswith(".kt"):
            failed.add(Path(head))
    return failed


def declaration_counts(grammar: Path, query: Path, files: list[Path]) -> dict[Path, int]:
    """한 매치가 선언 하나다. 파일 헤더 줄로 구간을 가른다."""
    counts: dict[Path, int] = {f: 0 for f in files}
    for i in range(0, len(files), BATCH):
        batch = files[i : i + BATCH]
        out = subprocess.run(
            ["tree-sitter", "query", str(query), *[str(f) for f in batch]],
            cwd=grammar, capture_output=True, text=True, check=False,
            env=isolated(grammar),
        ).stdout
        current: Path | None = None
        for line in out.splitlines():
            if not line.startswith((" ", "\t")) and line.strip().endswith(".kt"):
                current = Path(line.strip())
            elif current is not None and line.lstrip().startswith("pattern:"):
                counts[current] = counts.get(current, 0) + 1
    return counts


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--grammar", type=Path, required=True, help="tree-sitter-kotlin 클론 (빌드된 상태)")
    ap.add_argument("--corpus", type=Path, required=True, help="s0-corpus.sh 의 출력 디렉터리")
    ap.add_argument("--query", type=Path, required=True)
    ap.add_argument("--out", type=Path, required=True)
    a = ap.parse_args()

    # 산출물 헤더에는 **준 그대로** 적는다 — 절대 경로가 박히면 재현물이 기계에 매인다.
    query_as_given = a.query.as_posix()

    # 서브프로세스는 문법 디렉터리에서 돈다 — 상대 경로는 거기서 풀린다.
    a.grammar = a.grammar.resolve()
    a.corpus = a.corpus.resolve()
    a.query = a.query.resolve()
    a.out = a.out.resolve()

    files = kt_files(a.corpus)
    if len(files) != 1122:
        print(f"파일 수가 1,122 가 아니다: {len(files)}", file=sys.stderr)
        return 1

    failed = parse_failures(a.grammar, files)
    counts = declaration_counts(a.grammar, a.query, files)

    rows = []
    for f in files:
        rel = f.relative_to(a.corpus).as_posix()
        rows.append((rel, "fail" if f in failed else "ok", counts[f]))

    # **문법 rev 를 글자로 박지 않는다** — 클론에게 묻는다. 박아 두면 핀을 옮긴 회차에
    # 머리말이 옛 값을 그대로 말하고, 그러면 산출이 자기 출처를 거짓으로 적는다.
    rev = subprocess.run(["git", "rev-parse", "HEAD"], cwd=a.grammar,
                         capture_output=True, text=True, check=False).stdout.strip()
    # **쿼리는 경로가 아니라 내용으로 적는다.** 경로는 커밋마다 다른 것을 가리킬 수 있고,
    # 이 벡터는 **핀 커밋보다 앞선 커밋**에 실리므로 그때의 저장소에는 이 쿼리가 없다
    # (`[g50.pass]` ①). 해시는 그 순서를 지키면서도 출처를 검증 가능하게 남긴다.
    qdigest = hashlib.blake2b(a.query.read_bytes(), digest_size=16).hexdigest()

    with a.out.open("w", encoding="utf-8") as fh:
        fh.write("# S0 CLI 레퍼런스 — 파일별 최상위 선언 수\n")
        fh.write(f"# 문법 {rev[:7]} · {tree_sitter_version()}\n")
        fh.write(f"# 쿼리 {query_as_given} · blake2b-128 {qdigest}\n")
        fh.write("path\tparse\tdeclarations\n")
        for rel, parse, n in rows:
            fh.write(f"{rel}\t{parse}\t{n}\n")

    # ── 요약. T7 이 발표한 값과 같은 자리에 둔다 (부차 목적) ──
    n_files = len(rows)
    n_fail = sum(1 for _, p, _ in rows if p == "fail")
    n_decls = sum(n for _, _, n in rows)
    with_decl = sum(1 for _, _, n in rows if n > 0)
    zero_ok = sum(1 for _, p, n in rows if p == "ok" and n == 0)

    print(f"파일                {n_files:>6}   (T7: 1,122)")
    print(f"파싱 실패            {n_fail:>6}   (T7: 56)")
    print(f"선언 총수           {n_decls:>6}   (T7: 2,241)")
    print(f"선언 ≥1 파일        {with_decl:>6}   (T7: 1,058 = 94.30%)")
    print(f"  └ 그 비율        {with_decl / n_files:>6.2%}")
    print(f"파싱 성공했는데 0건  {zero_ok:>6}   (T7: 17 — 조용한 오파싱)")
    print(f"\n→ {a.out}")
    return 0


def tree_sitter_version() -> str:
    out = subprocess.run(["tree-sitter", "--version"], capture_output=True, text=True, check=False)
    return out.stdout.strip() or "unknown"


if __name__ == "__main__":
    raise SystemExit(main())
