#!/usr/bin/env python3
"""S0 전수 대조 — Rust 경로의 파일별 선언 수 벡터를 CLI 레퍼런스에 댄다.

**이것이 대조의 나머지 한쪽이다.** 다른 쪽은 `corpus/tasks/s0-reference-vector.tsv`
이고 `scripts/s0-reference.py` 가 만들었다. 둘은 같은 쿼리 파일
(`crates/pal-extract/queries/kotlin/top-level.scm`)과 같은 문법(3dea6df)을 쓴다.
그래서 차이가 나면 그 차이는 **코드 경로의 것**이다 — 그것이 R-01 의 관측이다.

합격선은 `corpus/criteria.toml` `[s0]` 에 있다 — 불일치 **0건**.

왜 서브커맨드가 아니라 바이너리를 1,122번 부르는가:
    합격선이 묻는 것은 *"출하되는 코드가 레퍼런스와 같은 값을 내는가"* 이고,
    이 방식은 출하물(`pal` 바이너리)을 그대로 몰기 때문에 대조 전용 경로를
    **하나도** 만들지 않는다. 전수 대조용 서브커맨드를 새로 두면 그 명령의
    존재 이유가 게이트가 되고, 게이트를 통과시키려고 1급 표면이 늘어난다.
    레퍼런스 쪽도 스크립트다 — 대조의 양쪽이 같은 층위에 선다.
    비용은 쟀다: 1,122 회 호출이 20초 안쪽이다.

사용:
    ./scripts/s0-compare.py --corpus /tmp/s0-corpus
    ./scripts/s0-compare.py --corpus /tmp/s0-corpus --bin target/debug/pal --out /tmp/observed.tsv

종료 코드:
    0  불일치 0건 — 합격선을 만족한다
    1  불일치가 있다 · 또는 대조가 성립하지 않았다 (파일 수·경로 집합·실행 실패)
"""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
from pathlib import Path

EXPECTED_FILES = 1122


def kt_files(corpus: Path) -> list[Path]:
    """**레퍼런스와 같은 규칙이어야 한다** — `s0-reference.py` 의 같은 이름 함수와 한 줄까지 같다."""
    return sorted(corpus.rglob("*.kt"))


def read_reference(path: Path) -> dict[str, int]:
    """레퍼런스 TSV → {상대경로: 선언 수}. `parse` 컬럼은 대조에 쓰지 않는다(아래 주석)."""
    rows: dict[str, int] = {}
    with path.open(encoding="utf-8") as fh:
        for line in fh:
            line = line.rstrip("\n")
            if not line or line.startswith("#"):
                continue
            parts = line.split("\t")
            if len(parts) != 3 or parts[0] == "path":  # 헤더
                continue
            rows[parts[0]] = int(parts[2])
    return rows


def observe(bin_path: Path, corpus: Path, files: list[Path]) -> tuple[dict[str, int], list[str]]:
    """`pal symbols --json` 을 파일마다 부른다. **JSON 배열의 길이가 선언 수다.**"""
    counts: dict[str, int] = {}
    errors: list[str] = []
    for i, f in enumerate(files, 1):
        rel = f.relative_to(corpus).as_posix()
        proc = subprocess.run(
            [str(bin_path), "symbols", "--json", str(f)],
            capture_output=True, text=True, check=False,
        )
        if proc.returncode != 0:
            # **실행 실패는 선언 0 이 아니다.** 0 으로 적으면 대조가 그 사실을 삼킨다.
            errors.append(f"{rel}\t종료코드 {proc.returncode}\t{proc.stderr.strip().splitlines()[-1] if proc.stderr.strip() else ''}")
            continue
        try:
            found = json.loads(proc.stdout)
        except json.JSONDecodeError as e:
            errors.append(f"{rel}\tJSON 아님\t{e}")
            continue
        if not isinstance(found, list):
            # `Capable::NotBuilt` 는 객체로 나온다 — Kotlin 에서는 나올 수 없다.
            errors.append(f"{rel}\t배열이 아니다\t{proc.stdout.strip()[:120]}")
            continue
        counts[rel] = len(found)
        if i % 200 == 0:
            print(f"  … {i}/{len(files)}", file=sys.stderr)
    return counts, errors


def main() -> int:
    repo = Path(__file__).resolve().parent.parent
    ap = argparse.ArgumentParser()
    ap.add_argument("--corpus", type=Path, required=True, help="s0-corpus.sh 의 출력 디렉터리")
    ap.add_argument("--reference", type=Path, default=repo / "corpus/tasks/s0-reference-vector.tsv")
    ap.add_argument("--bin", type=Path, default=repo / "target/release/pal")
    ap.add_argument("--out", type=Path, help="관측 벡터를 레퍼런스와 같은 형식으로 남긴다 (선택)")
    a = ap.parse_args()

    corpus = a.corpus.resolve()
    bin_path = a.bin.resolve()
    if not bin_path.exists():
        print(f"바이너리가 없다: {bin_path}  — `cargo build --release` 를 먼저 하라", file=sys.stderr)
        return 1

    files = kt_files(corpus)
    if len(files) != EXPECTED_FILES:
        print(f"파일 수가 {EXPECTED_FILES} 가 아니다: {len(files)}", file=sys.stderr)
        return 1

    reference = read_reference(a.reference)
    if len(reference) != EXPECTED_FILES:
        print(f"레퍼런스 행이 {EXPECTED_FILES} 가 아니다: {len(reference)}", file=sys.stderr)
        return 1

    print(f"바이너리 {bin_path}", file=sys.stderr)
    observed, errors = observe(bin_path, corpus, files)

    # ── 경로 집합부터 본다. 여기가 어긋나면 값 비교는 의미가 없다 ──
    only_ref = sorted(set(reference) - set(observed) - {e.split("\t")[0] for e in errors})
    only_obs = sorted(set(observed) - set(reference))

    mismatches = [
        (rel, reference[rel], observed[rel])
        for rel in sorted(observed)
        if rel in reference and observed[rel] != reference[rel]
    ]

    # ── 요약 ──
    n_decls = sum(observed.values())
    with_decl = sum(1 for n in observed.values() if n > 0)
    print()
    print(f"관측 파일          {len(observed):>6}   (기대 {EXPECTED_FILES})")
    print(f"선언 총수          {n_decls:>6}   (T7: 2,241)")
    print(f"선언 ≥1 파일       {with_decl:>6}   (T7: 1,058 = 94.30%)")
    print(f"실행 실패          {len(errors):>6}")
    print(f"레퍼런스에만 있음   {len(only_ref):>6}")
    print(f"관측에만 있음      {len(only_obs):>6}")
    print(f"─────────────────────────")
    print(f"불일치            {len(mismatches):>6}   (합격선: 0)")

    # ── **목록을 전부 적는다.** 건수만 적는 것은 [s0.pass].on_failure 위반이다 ──
    if errors:
        print("\n## 실행 실패")
        for e in errors:
            print(f"  {e}")
    for label, rows in (("레퍼런스에만 있는 경로", only_ref), ("관측에만 있는 경로", only_obs)):
        if rows:
            print(f"\n## {label}")
            for r in rows:
                print(f"  {r}")
    if mismatches:
        print("\n## 불일치 — 파일별 (레퍼런스 → 관측)")
        for rel, want, got in mismatches:
            print(f"  {want:>4} → {got:<4}  {rel}")

    if a.out:
        with a.out.open("w", encoding="utf-8") as fh:
            fh.write("# S0 관측 벡터 — Rust 경로(`pal symbols --json`)의 파일별 선언 수\n")
            fh.write(f"# 바이너리 {bin_path}\n")
            fh.write("path\tdeclarations\n")
            for rel in sorted(observed):
                fh.write(f"{rel}\t{observed[rel]}\n")
        print(f"\n→ {a.out}")

    ok = not mismatches and not errors and not only_ref and not only_obs
    print(f"\n{'통과 — 불일치 0건' if ok else '불일치가 있다'}")
    return 0 if ok else 1


if __name__ == "__main__":
    raise SystemExit(main())
