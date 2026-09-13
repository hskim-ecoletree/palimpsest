#!/usr/bin/env python3
"""`E2` — 효과 확인의 `touch` 출력들에서 장면 넷이 서는가.

⑴ 승인 전 대기 구역 ≥ 1 → 화면이 안내한 명령으로 **결정 문서 조각**을 승인 → `■ 이 좌표에 걸린 것` ≥ 1
   이고 그 줄이 판정(최신 상태 · 낡음)을 싣고 승인한 조각의 본문이 그 심볼 이름을 담는다
⑵ 호출자에 **다른 파일의 참조** ≥ 1(하한)
⑶ 봉인 목록의 동명 심볼을 **지목 문자열로** 부른 호출 ≥ 1
⑷ 금지 패턴 0 — `D1` 과 **같은 검사 함수**(`PAL_VOCAB_SCAN`)를 댄다

사용: python3 E2-scene.py <effect 디렉터리> <palimpsest 저장소>
"""
import os
import re
import shlex
import subprocess
import sys
from pathlib import Path

fx = Path(sys.argv[1]).resolve()
repo = Path(sys.argv[2]).resolve()
# ⚠ **절대 경로다** — `cargo test` 는 크레이트 디렉터리에서 시험을 돌려서 상대 경로를 못 찾는다.
seal_file = "src/core/hosts/codex.ts"
touch = sorted((fx / "03-touch").glob("[0-9][0-9]-*.txt"))
out = ["# E2 — 장면 넷", ""]
ok = {}

# ⑴
first = (fx / "03-touch" / "01-mcpServersFromToml.txt").read_text(encoding="utf-8")
m = re.search(r"■ 승인 대기 — 이 좌표를 가리키는 문서 조각 \((\d+)\)", first)
대기_전 = int(m.group(1)) if m else 0
안내 = next((l.split("승인:", 1)[1].strip() for l in first.splitlines() if "승인: pal narrative" in l), "")
approve = (fx / "04-approve.txt").read_text(encoding="utf-8")
돌린_명령 = next((l[2:].strip() for l in approve.splitlines() if l.startswith("$ pal narrative")), "")
승인_성공 = "승인했습니다" in approve and re.search(r"^rc=0$", approve, re.M) is not None
after = (fx / "04-touch-after-approve.txt").read_text(encoding="utf-8")
m = re.search(r"■ 이 좌표에 걸린 것 \((\d+)\)", after)
걸린 = int(m.group(1)) if m else 0
판정 = re.search(r"\] (최신 상태|낡음)", after) is not None
조각_줄 = next((l for l in first.splitlines() if "후보" in l and "·" in l and "승인" not in l), "")
문서 = re.search(r"(\.ditto/knowledge/adr/|docs/)\S+\.md", 조각_줄) is not None
걸린_구역 = after.split("■ 이 좌표를 지켜보는 것", 1)[0]
본문에_이름 = "mcpServersFromToml" in 걸린_구역.split("■ 이 좌표에 걸린 것", 1)[-1]
ok["⑴"] = 대기_전 >= 1 and shlex.split(안내) == shlex.split(돌린_명령) and 승인_성공 and 걸린 >= 1 and 판정 and 문서 and 본문에_이름
out += [
    "## ⑴ 승인 대기 → 화면의 명령으로 승인 → 걸린 것",
    f"- 승인 전 대기 {대기_전} · 조각 `{조각_줄.strip()}` · 결정 문서인가 {문서}",
    f"- 안내된 명령과 돌린 명령이 같은가 {shlex.split(안내) == shlex.split(돌린_명령)} · 승인 성공 {승인_성공}",
    f"- 승인 뒤 걸린 것 {걸린} · 판정 줄 {판정} · 본문에 심볼 이름 {본문에_이름}",
    f"- **{'선다' if ok['⑴'] else '안 선다'}**",
    "",
]

# ⑵
다른_파일 = []
for f in sorted((fx / "04-callers").glob("*.txt")):
    for l in f.read_text(encoding="utf-8").splitlines():
        mm = re.match(r"^\s{2}\S+\s+(\S+)\s+(\S+):(\d+)\s*$", l)
        if mm and mm.group(2) != seal_file:
            다른_파일.append(f"{f.stem} ← {mm.group(1)} {mm.group(2)}:{mm.group(3)}")
ok["⑵"] = len(다른_파일) >= 1
out += ["## ⑵ 다른 파일의 호출자", f"- {len(다른_파일)} 건: " + " · ".join(다른_파일), f"- **{'선다' if ok['⑵'] else '안 선다'}**", ""]

# ⑶
지목 = []
for f in touch:
    if f.stem.endswith("-pick"):
        base = f.with_name(f.name.replace("-pick.txt", ".txt"))
        if base.exists() and "의 후보가" in base.read_text(encoding="utf-8") and f"· {seal_file}:" in f.read_text(encoding="utf-8"):
            지목.append(f.stem)
ok["⑶"] = len(지목) >= 1
out += ["## ⑶ 동명 후보를 지목해 부른 호출", f"- {len(지목)} 건: " + " · ".join(지목), f"- **{'선다' if ok['⑶'] else '안 선다'}**", ""]

# ⑷
files = [str(p) for p in touch] + [str(fx / "04-touch-after-approve.txt")]
env = dict(os.environ, PAL_VOCAB_SCAN=":".join(files))
r = subprocess.run(
    ["cargo", "test", "-q", "-p", "pal-cli", "--test", "user_vocabulary", "--", "--ignored", "--nocapture", "d1_scan"],
    cwd=repo, env=env, capture_output=True, text=True,
)
걸린_자리 = re.search(r"걸린 자리 (\d+)", r.stdout + r.stderr)
ok["⑷"] = r.returncode == 0
out += [
    "## ⑷ 금지 패턴 — `D1` 과 같은 함수",
    f"- 잰 화면 {len(files)} · 걸린 자리 {걸린_자리.group(1) if 걸린_자리 else '(못 읽음)'} · rc {r.returncode}",
    f"- **{'선다' if ok['⑷'] else '안 선다'}**",
    "",
]
out.append(f"## 판정: {'통과' if all(ok.values()) else '반증'} — " + " · ".join(f"{k} {'✓' if v else '✗'}" for k, v in ok.items()))
print("\n".join(out))
if not ok["⑷"]:
    print("\n```\n" + (r.stdout + r.stderr)[-3000:] + "\n```")
sys.exit(0 if all(ok.values()) else 1)
