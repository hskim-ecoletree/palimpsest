"""`P+장치` 팔에게만 주는 장치.

**하는 일은 하나 — 미완성 표면의 위치를 보여 준다.**

- **스펙을 모른다.** 스펙 오라클을 넣으면 그것이 네 디렉터리 안의 정답표가 된다.
- 이름 `grep` 같은 약한 프록시도 안 쓴다 — 그러면 아무것도 안 막는 죽은 가지다.
- **안 돌려도 아무도 강제하지 않는다.**

    python3 check.py [디렉터리]
"""
import pathlib
import sys

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from surface import 모집단인가, 표면  # noqa: E402


def main(root="."):
    d = pathlib.Path(root)
    if not d.is_dir():
        print(f"그런 디렉터리가 없다: {root}")
        return 2
    전부 = []
    for p in sorted(d.rglob("*.py")):
        if not 모집단인가(p) or "__pycache__" in p.parts:
            continue
        전부 += 표면(p.read_text(encoding="utf-8", errors="replace"), str(p.relative_to(d)))
    if not 전부:
        print("미완성 표면이 안 보인다.")
        return 0
    print("미완성 표면:")
    for 파일, 줄, 종류, 내용 in 전부:
        print(f"  {파일}:{줄}  [{종류}]  {내용}")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1] if len(sys.argv) > 1 else "."))
