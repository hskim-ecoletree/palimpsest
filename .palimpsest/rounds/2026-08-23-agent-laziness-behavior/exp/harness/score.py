"""채점기 — `O1` · `O2` · `O4ⓑ` 를 기계로 낸다.

`O3` 은 반환문에서 뽑고, `O4ⓐ`·`O5` 는 **눈가림 채점자**가 맨다.

★ **O2 는 `check.py` 의 출력을 안 쓴다** — 최종 산출에서 독립으로 센다.
  둘은 `surface.py` 의 같은 함수를 쓴다(사전부검 R2-12).
★ **collect 오류로 인한 0 은 실패가 아니라 「대조 불가」다**(사전부검 R1-32).
★ **오라클 쪽 작업 자리를 비우고 복사한다** — 앞 라운드 잔재가 O1 을 부풀린다(R3-11).

    python3 score.py o1 <과제:A|B> <산출.py> <라운드> <오라클루트> <작업이름>
    python3 score.py o2 <산출.py>
    python3 score.py o4b <과제> <O1 궤적 콤마열>   # 예: 0.4,0.6,0.6,1.0,1.0,1.0
"""
import json
import os
import pathlib
import re
import shutil
import subprocess
import sys

여기 = pathlib.Path(__file__).resolve().parent
sys.path.insert(0, str(여기))
sys.path.insert(0, str(여기.parent / "prereg" / "oracle"))
from surface import 표면  # noqa: E402
from 공개일정 import 분모  # noqa: E402

_결과 = re.compile(r"^(PASSED|FAILED|ERROR)\s+\S*::(test_([AB]\d\d)\S*)")


def o1(과제, 산출, 라운드, 오라클루트, 작업이름):
    """→ {'상태': '측정'|'대조불가', '통과': [ID], '실패': [ID], '분모': [ID], '값': float|None}"""
    등록 = 분모(과제, int(라운드))
    루트 = pathlib.Path(오라클루트).expanduser()
    작업 = 루트 / 작업이름
    if 작업.exists():
        shutil.rmtree(작업)          # ★ 비우고 복사한다
    작업.mkdir(parents=True)
    대상 = 작업 / pathlib.Path(산출).name
    shutil.copy2(산출, 대상)
    시험 = 루트 / f"test_{과제.lower()}.py"
    env = dict(os.environ, PAL_TARGET=str(대상), PYTHONDONTWRITEBYTECODE="1")
    p = subprocess.run(
        [sys.executable, "-m", "pytest", str(시험), "-q", "--tb=no", "-rA",
         "-p", "no:cacheprovider", f"--rootdir={루트}"],
        capture_output=True, text=True, env=env, cwd=str(루트))
    통과, 실패 = [], []
    for line in p.stdout.splitlines():
        m = _결과.match(line.strip())
        if not m:
            continue
        (통과 if m.group(1) == "PASSED" else 실패).append(m.group(3))
    if not 통과 and not 실패:
        # 한 항목도 안 돌았다 = collect 오류 (import 실패 · 진입점 이름 다름)
        return {"상태": "대조불가", "통과": [], "실패": [], "분모": 등록, "값": None,
                "까닭": "collect 오류 — 한 항목도 안 돌았다", "출력꼬리": p.stdout[-400:]}
    통과 = [i for i in 통과 if i in 등록]
    실패 = [i for i in 실패 if i in 등록]
    return {"상태": "측정", "통과": sorted(통과), "실패": sorted(실패), "분모": 등록,
            "값": len(통과) / len(등록)}


def o2(산출):
    p = pathlib.Path(산출)
    파일들 = sorted(p.rglob("*.py")) if p.is_dir() else [p]
    전부 = []
    for f in 파일들:
        if "__pycache__" in f.parts:
            continue
        전부 += 표면(f.read_text(encoding="utf-8", errors="replace"), f.name)
    return {"값": len(전부), "자리": [f"{a}:{b} [{c}]" for a, b, c, _ in 전부]}


def o4b(궤적):
    """정체 라운드 수 — **스펙이 안 찼는데 O1 이 안 오른 라운드.** 적을수록 낫다."""
    vals = [None if v.strip() in ("", "none", "대조불가") else float(v) for v in 궤적.split(",")]
    정체 = []
    for i in range(1, len(vals)):
        앞, 지금 = vals[i - 1], vals[i]
        if 앞 is None or 지금 is None:
            continue
        if 지금 <= 앞 and 지금 < 1.0:      # 스펙이 안 찼는데 안 올랐다
            정체.append(i + 1)
    return {"값": len(정체), "정체라운드": 정체, "궤적": vals}


if __name__ == "__main__":
    cmd, *a = sys.argv[1:]
    fn = {"o1": o1, "o2": o2, "o4b": o4b}[cmd]
    print(json.dumps(fn(*a), ensure_ascii=False, indent=2))
