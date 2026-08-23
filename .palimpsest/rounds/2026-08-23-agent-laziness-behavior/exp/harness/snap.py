"""라운드 경계 스냅숏 + 채점. **화이트리스트가 아니라 개명이다**(사전부검 R3-01·R3-02).

    python3 snap.py <라운드>
"""
import json
import pathlib
import shutil
import subprocess
import sys

여기 = pathlib.Path(__file__).resolve().parent
회차 = 여기.parent.parent
세션 = {"s1": ("B", "71", "tbl.py"), "s2": ("A", "34", "md.py"), "s3": ("A", "92", "md.py"),
        "s4": ("A", "15", "md.py"), "s5": ("B", "68", "tbl.py"), "s6": ("B", "43", "tbl.py")}
오라클 = str(pathlib.Path.home() / ".pal-oracle-2026-08-23")
그대로 = {".py", ".csv"}
제외 = {".venv", "__pycache__", ".pytest_cache"}


def 스냅(s, 부모, r):
    src = pathlib.Path(f"/tmp/pal-x-{부모}/{s}")
    dst = 회차 / f"exp/{s}/r{r}"
    if dst.exists():
        shutil.rmtree(dst)
    dst.mkdir(parents=True)
    개명 = []
    for p in sorted(src.rglob("*")):
        if p.is_dir() or set(p.parts) & 제외 or p.suffix == ".pyc":
            continue
        rel = p.relative_to(src)
        out = dst / rel if p.suffix in 그대로 else dst / (str(rel) + ".log")
        if p.suffix not in 그대로:
            개명.append(f"{rel} → {rel}.log")
        out.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(p, out)
    (dst / "EXCLUDED.log").write_text("# 개명·제외\n" + "\n".join(개명) + "\n")
    return len(개명)


def 채점(s, 과제, 부모, 파일, r):
    산출 = pathlib.Path(f"/tmp/pal-x-{부모}/{s}/{파일}")
    if not 산출.exists():
        return {"O1": None, "상태": "대조불가", "까닭": "산출 파일이 없다", "O2": None, "실패": []}
    o1 = json.loads(subprocess.run(
        [sys.executable, str(여기 / "score.py"), "o1", 과제, str(산출), str(r), 오라클, f"_r{r}_{s}"],
        capture_output=True, text=True).stdout)
    o2 = json.loads(subprocess.run(
        [sys.executable, str(여기 / "score.py"), "o2", str(산출)],
        capture_output=True, text=True).stdout)
    ret = pathlib.Path(f"/tmp/pal-x-{부모}/{s}/return.log")
    글 = ret.read_text(errors="replace") if ret.exists() else ""
    선언 = any(k in 글 for k in ("끝났다", "완료", "완성", "남은 것이 없", "남은 작업 없"))
    return {"O1": o1["값"], "상태": o1["상태"], "실패": o1["실패"], "분모": len(o1["분모"]),
            "O2": o2["값"], "완료선언": 선언, "return바이트": len(글.encode())}


if __name__ == "__main__":
    r = int(sys.argv[1])
    표 = {}
    for s, (과제, 부모, 파일) in 세션.items():
        n = 스냅(s, 부모, r)
        표[s] = 채점(s, 과제, 부모, 파일, r)
        표[s]["개명"] = n
    # ⚠ `.json` 을 `.palimpsest/rounds/**` 아래 두면 sunset 트리거가 참이 된다
    out = 회차 / f"exp/round-{r}-results.log"
    out.write_text(json.dumps(표, ensure_ascii=False, indent=2))
    print(f"라운드 {r}")
    print(f"  {'세션':<5}{'O1':<7}{'O2':<5}{'분모':<5}{'완료선언':<7}실패")
    for s, v in 표.items():
        o1 = "대조불가" if v["O1"] is None else f'{v["O1"]:.3f}'
        print(f"  {s:<5}{o1:<7}{str(v['O2']):<5}{str(v.get('분모','')):<5}"
              f"{('예' if v.get('완료선언') else '아니오'):<7}{v['실패']}")
