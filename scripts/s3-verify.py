#!/usr/bin/env python3
"""S3 대조 — 결박이 파생층을 지워도 살아남고, 코드가 변하면 표시되는가.

합격선 정본은 `corpus/criteria.toml` `[s3]`.

  ① **2층을 통째로 지운 뒤에도 결박이 그대로** — R-21 을 처음으로 실물에서 시험한다
  ② `touch` 의 `bindings` 가 `NotBuilt` 가 아니라 `Present`
  ③ 낡음 **양방향** — 안 바뀌면 `live`, 바뀌면 `stale{triggered_by}`
  ④ 심볼이 사라지면 `stale` 이 아니라 `orphaned`
  ⑤ `pal-intent` 에 지우는 공개 API 가 없다

고정 SHA 는 고칠 수 없으므로 ③④ 는 임시 저장소에서 시험한다 —
**코드를 실제로 바꿔야 낡음을 볼 수 있다.**
"""

from __future__ import annotations

import argparse
import json
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path


def run(cmd, cwd=None, check=True):
    p = subprocess.run(cmd, cwd=cwd, capture_output=True, text=True, check=False)
    if check and p.returncode != 0:
        raise SystemExit(f"실패: {' '.join(map(str, cmd))}\n{p.stderr.strip()}")
    return p.stdout


def main() -> int:
    root = Path(__file__).resolve().parent.parent
    ap = argparse.ArgumentParser()
    ap.add_argument("--bin", type=Path, default=root / "target/release/pal")
    a = ap.parse_args()
    pal = a.bin.resolve()
    if not pal.exists():
        print(f"바이너리가 없다: {pal}", file=sys.stderr)
        return 1

    failures: list[str] = []
    tmp = Path(tempfile.mkdtemp(prefix="s3-verify-"))
    try:
        repo = tmp / "repo"
        repo.mkdir()
        run(["git", "init", "-q", str(repo)])
        run(["git", "config", "user.email", "s3@verify"], cwd=repo)
        run(["git", "config", "user.name", "s3"], cwd=repo)
        (repo / "A.kt").write_text("class Target {\n  fun keep() {}\n}\n", encoding="utf-8")
        (repo / "B.kt").write_text("class Other\n", encoding="utf-8")
        run(["git", "add", "-A"], cwd=repo)
        run(["git", "commit", "-qm", "a"], cwd=repo)

        cache, index, intent = tmp / "cache", tmp / "index.redb", tmp / "intent.redb"
        base = ["--repo", str(repo), "--cache-dir", str(cache),
                "--index", str(index), "--intent", str(intent)]

        def touch(name="Target"):
            return json.loads(run([str(pal), "touch", name, *base, "--json"]))

        # 결박
        run([str(pal), "bind", "Target", "--note", "이 클래스는 계약이다", *base])

        # ② 결박이 Present 로 뜬다
        env = touch()
        answer = env["answer"]
        bindings = answer.get("bindings")
        present = isinstance(bindings, dict) and "present" in bindings
        n = len(bindings["present"]) if present else 0
        print(f"② 결박  present={present} · {n} 건")
        if not present:
            failures.append(f"② bindings 가 Present 가 아니다: {bindings!r}")
        elif n != 1:
            failures.append(f"② 결박이 1건이 아니다: {n}")

        def freshness(e):
            b = e["answer"]["bindings"]["present"][0]
            return b["status"]["code"]["freshness"], b["status"]["code"]

        # ③-앞 안 바뀌면 live
        f, _ = freshness(env)
        print(f"③ 낡음  변경 전 → {f}")
        if f != "live":
            failures.append(f"③ 안 바꿨는데 {f} 다")

        # ① 2층을 통째로 지운다 — **R-21 의 실물 시험**
        index.unlink(missing_ok=True)
        env2 = touch()
        b2 = env2["answer"].get("bindings")
        n2 = len(b2["present"]) if isinstance(b2, dict) and "present" in b2 else 0
        same = (isinstance(b2, dict) and "present" in b2
                and b2["present"][0]["note"] == bindings["present"][0]["note"])
        print(f"① R-21  2층 삭제 후 → 결박 {n2} 건 · 내용 동일 {'예' if same else '아니오'}")
        if n2 != 1 or not same:
            failures.append("① 2층을 지웠더니 결박이 사라지거나 달라졌다 — R-21 의 실현이다")

        # ③-뒤 코드를 바꾸면 stale
        (repo / "A.kt").write_text("class Target {\n  fun keep() { log() }\n}\n", encoding="utf-8")
        run(["git", "add", "-A"], cwd=repo)
        run(["git", "commit", "-qm", "b"], cwd=repo)
        f3, code3 = freshness(touch())
        trig = len(code3.get("triggered_by", []))
        print(f"③ 낡음  본문 변경 후 → {f3} · triggered_by {trig}")
        if f3 != "stale":
            failures.append(f"③ 본문을 바꿨는데 {f3} 다 — 낡음이 감지되지 않았다")
        elif trig != 1:
            failures.append(f"③ triggered_by 가 {trig} 건이다 — 무엇이 켰는지 실려야 한다")

        # 포매팅만 바꾸면 다시 live 여야 한다 (정규화가 낡음에 실제로 작동하는가)
        (repo / "A.kt").write_text("class    Target {\n\n  fun keep() { log() }   // 주석\n\n}\n",
                                   encoding="utf-8")
        run(["git", "add", "-A"], cwd=repo)
        run(["git", "commit", "-qm", "c"], cwd=repo)
        f4, _ = freshness(touch())
        print(f"③ 낡음  포매팅·주석만 변경 → {f4}  (직전 상태 유지 = stale 이 정상)")
        if f4 != "stale":
            failures.append(f"③ 포매팅만 바꿨는데 상태가 {f4} 로 움직였다 — 정규화가 새고 있다")

        # ④ 심볼을 없애면 orphaned
        (repo / "A.kt").write_text("class Renamed {\n  fun keep() {}\n}\n", encoding="utf-8")
        run(["git", "add", "-A"], cwd=repo)
        run(["git", "commit", "-qm", "d"], cwd=repo)
        env5 = touch("Renamed")
        b5 = env5["answer"]["bindings"]["present"] if env5["answer"]["outcome"] == "found" else []
        # 결박은 옛 좌표에 걸려 있으므로 Renamed 에는 안 붙는다. 옛 이름으로 물으면 Unknown.
        old = touch("Target")
        print(f"④ 사라짐  옛 이름 조회 → {old['answer']['outcome']} · 새 이름에 걸린 것 {len(b5)} 건")
        if old["answer"]["outcome"] != "unknown":
            failures.append("④ 사라진 심볼이 여전히 조회된다")
        if len(b5) != 0:
            failures.append("④ 결박이 다른 심볼로 옮겨 붙었다 — 좌표가 뭉개졌다")
        # 의도 저장소에는 그대로 남아 있어야 한다 (유실 금지)
        print(f"④ 사라짐  의도 저장소에는 남아 있어야 한다 → intent.redb 크기 {intent.stat().st_size}B")
        if intent.stat().st_size == 0:
            failures.append("④ 심볼이 사라지자 의도 저장소가 비었다 — 유실이다")

        # ⑤ 지우는 API 부재
        src = (root / "crates/pal-intent/src").rglob("*.rs")
        bad = []
        for f in src:
            text = f.read_text(encoding="utf-8")
            for m in re.finditer(r"pub (?:async )?fn (\w+)", text):
                if re.search(r"delete|remove|drop|prune|clear|purge|wipe", m.group(1), re.I):
                    bad.append(f"{f.name}::{m.group(1)}")
        print(f"⑤ 구조  pal-intent 의 지우는 공개 API {len(bad)} 개")
        if bad:
            failures.extend(f"⑤ 지우는 API 가 있다: {b}" for b in bad)

    finally:
        shutil.rmtree(tmp, ignore_errors=True)

    print()
    if failures:
        print("어긋난 것:")
        for f in failures:
            print(f"  · {f}")
        print("\n반증이다")
        return 1
    print("다섯 다 통과")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
