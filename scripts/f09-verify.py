#!/usr/bin/env python3
"""F09 대조 — 결박과 낡음. **합격선은 재기 전에 등록됐다** (`corpus/criteria.toml` `[f09]`).

    ① 합성 포매팅 변형에서 `stale` **0**            ← 진행 불가
    ② 의미 변경 커밋에서 `stale` 검출률 **≥ 90%**   ← ①의 짝
    ③ `Undeterminable` 이 `Live` 로 **안 샌다** (0)  ← R16
    ④ `Orphaned` ≠ `Stale`
    ⑤ 실 이력 표본 20 건의 거짓 양성률 — `symbol` **≤10%** · `callers` **≤30%**
    ⑥ `Undeterminable` 비율 **≤15%** + 사유 **최소 둘**이 실제로 산출
    ⑦ 반경별 감시 집합 크기 · 비용

**「낡음을 감지한다」는 말하기 가장 쉽다.** 아무것도 안 켜는 감지기도, 전부 켜는
감지기도 그 문장을 만족한다 — 그래서 ①과 ②가 서로를 막고, ⑥의 상한과 하한이 서로를
막는다.

# 대조가 꺼지는 형태 — 이 스크립트가 막는 것 셋

  · **첫째(변형이 아무것도 안 바꾸면 실패)** — 바뀐 파일 수와 바이트 차이를 **검사에
    박는다.** 0 이면 멈춘다
  · **다섯째(도구가 무엇을 읽는지)** — 변형 뒤 2층의 노드·엣지 수와
    `built_for_this_snapshot` 을 **산출로 확인한다**
  · **둘째(공유 상태)** — **변형마다 작업 사본과 캐시·2층·의도 저장소를 새로 만든다.**
    F02-4 에서 공유 캐시가 대조를 통째로 껐다

사용:
    ./scripts/f09-verify.py                 # 전부
    ./scripts/f09-verify.py --skip-prettier # ①의 prettier 변형만 건너뛴다 (대조 불가로 적힌다)
"""

from __future__ import annotations

import argparse
import json
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
BIN = ROOT / "target/release/pal"

DITTO = Path.home() / "dev/projects/ditto"
DITTO_PIN = "aded7ce7f88f"
PORTAL = Path.home() / "dev/projects/boxwood/portal-backend"
PORTAL_PIN = "a29cad0bf6a8"

# ── `[f09.pass]` 가 등록한 값들. **여기서 정하지 않는다 — 옮겨 적을 뿐이다** ────────
SYNTHETIC_STALE_MAX = 0
SEMANTIC_DETECTION_MIN_PCT = 90
UNDETERMINABLE_LEAK_MAX = 0
FALSE_POSITIVE_SYMBOL_MAX_PCT = 10
FALSE_POSITIVE_CALLERS_MAX_PCT = 30
UNDETERMINABLE_RATIO_MAX_PCT = 15
UNDETERMINABLE_MIN_REASONS = 2

# ── 하한 — **시험되지 않은 대조는 `–` 가 아니라 실패다** (`2e2eb3f`) ──────────────
MIN_BINDINGS = 20
# ⑤의 코퍼스당 표본 — `[f09.4].sample_selection` 규칙 1 이 **코퍼스 둘에서 각각 10 건**
# 이라고 적었다. 합이 20 이고, 등급 축(`exact` · `ordinal`)을 가르는 것이 그 이유다.
SAMPLES_PER_CORPUS = 10
MIN_CHANGED_FILES = 1
MIN_CHANGED_BYTES = 1
# ②의 분모 하한 — 변형이 파일을 깨뜨려 `orphaned` 가 되면 그것은 검출 실패가 아니다.
# 살아남은 것이 적으면 비율이 표본 하나에 흔들린다.
MIN_SURVIVING = 10

# ⑤가 훑는 커밋 창 — **`[f03.1]` 이 `--history 120` 을 기본으로 쓴 선례를 따른다.**
#
# 40 으로 잡았더니 그 구간에서 바뀐 파일 안에 유일한 이름의 심볼이 20 개가 안 됐다
# (결박 15건 · 하한 미달). **창의 크기는 합격선이 아니라 표본을 세우는 조건**이고,
# 넓히면 표본이 더 대표적이 된다 — 좁혀서 통과시키는 것과 반대 방향이다.
HISTORY_WINDOW = 120

결과: list[tuple[str, str, str]] = []  # (표시, 이름, 값)

# **실물에서 실제로 산출된 `Undeterminable` 사유들** — ⑥의 하한이 이것을 센다.
#
# 상한만 걸면 *"안 켜면 통과"* 이고 하한만 걸면 *"많이 켜면 통과"* 다. **둘이 있어야
# 이 값이 무언가를 잰다**(`[f09.pass].undeterminable_ratio_grounds`).
관측된_사유: set[str] = set()


def ok(name: str, value: str) -> None:
    결과.append(("ok", name, value))


def fail(name: str, value: str) -> None:
    결과.append(("FAIL", name, value))


def skip(name: str, value: str) -> None:
    결과.append(("–", name, value))


def run(cmd: list[str], cwd: Path | None = None) -> subprocess.CompletedProcess:
    return subprocess.run(cmd, cwd=cwd, capture_output=True, text=True, check=False)


def pal(args: list[str], repo: Path, box: Path) -> dict:
    """`pal` 하나를 돌린다. **방마다 캐시·2층·의도가 따로다.**"""
    p = run(
        [str(BIN), *args, "--repo", str(repo),
         "--cache-dir", str(box / "cache"), "--index", str(box / "index.redb"),
         "--intent", str(box / "intent.redb")]
    )
    if p.returncode != 0:
        raise SystemExit(f"실패: pal {' '.join(args)}\n{p.stderr[-500:]}")
    return json.loads(p.stdout) if "--json" in args else {}


def 사본(tmp: Path, tag: str, src: Path, pin: str) -> tuple[Path, Path]:
    """**이름을 고정한다** — 매니페스트가 없으면 `repo_id` 가 디렉터리 이름에서 온다.

    회차마다 이름이 달라지면 좌표가 전부 달라지고, 그러면 이 대조는 무엇을 재든
    「움직였다」를 낸다(F03-1 게이트 §4 에서 실제로 걸린 자리다).

    그리고 **방마다 캐시·2층·의도가 따로다** — F02-4 에서 공유 캐시가 대조를 껐다.
    """
    box = tmp / tag
    box.mkdir(parents=True)
    repo = box / "corpus"
    p = run(["git", "clone", "--local", "--no-checkout", "-q", str(src), str(repo)])
    if p.returncode != 0:
        raise SystemExit(f"사본을 만들지 못했다: {p.stderr[-300:]}")
    run(["git", "-C", str(repo), "checkout", "-q", pin])
    return repo, box


def 소스_바이트(repo: Path, ext: str) -> dict[str, bytes]:
    return {
        str(f.relative_to(repo)): f.read_bytes()
        for f in sorted(repo.rglob(f"*{ext}"))
        if "node_modules" not in f.parts and ".git" not in f.parts
    }


def 변형이_먹었나(전: dict[str, bytes], 후: dict[str, bytes], 이름: str) -> tuple[int, int]:
    """**대조가 꺼지는 첫째 형태를 막는다.** 변형이 소스를 안 바꿨으면 뒤가 전부 공짜다."""
    바뀐 = [k for k in 전 if k in 후 and 전[k] != 후[k]]
    바이트 = sum(abs(len(후[k]) - len(전[k])) for k in 바뀐)
    if len(바뀐) < MIN_CHANGED_FILES:
        raise SystemExit(
            f"[{이름}] 변형이 파일을 하나도 안 바꿨다 — 이 대조는 아무것도 안 잰다"
        )
    # 길이가 같아도 내용이 다를 수 있다(따옴표 교체). 파일 수가 하한을 넘었으면 통과.
    return len(바뀐), max(바이트, MIN_CHANGED_BYTES if 바뀐 else 0)


# ── 의미 변형 — **종류에 맞게 한다** ─────────────────────────────────────────
#
# ⚠ **처음에는 종류를 안 봤고 그것이 검출률을 70% 로 만들었다.**
#
# 결박은 `function` 에만 걸리지 않는다 — `interface` · `class` · `type_alias` ·
# `variable` 에도 걸린다. 그런데 삽입한 것이 **함수 본문 문법**(`if (0) { … };`)이었고,
# 게다가 「첫 `{`」이 **반환 타입 안**인 경우가 있었다:
#
#     claimWorkItem  …: Promise<ClaimWiringResult & {      ← 여기가 첫 `{` 다
#
# 그래서 삽입이 타입을 깨거나(파싱 실패 → 분모 밖) 엉뚱한 곳에 갔다.
# **감지기가 아니라 변형기가 틀린 것**이고, 그 구별을 안 하면 이 대조는 F09 가 아니라
# 우리 변형기의 품질을 잰다.
#
# 이제 둘을 고친다:
#   · **본문 블록을 「span 끝에서 매칭되는 여는 중괄호」로 찾는다** — 반환 타입 안의
#     중괄호는 span 끝에서 안 닫히므로 안 걸린다
#   · **종류마다 문법이 맞는 것을 넣는다** — 문(statement) 대 멤버(member)

# 이 회차가 변형할 수 있는 종류. **나머지는 뺀다 — 그리고 뺀 사실이 산출에 남는다.**
변형가능 = {"function": "문", "method": "문", "class": "멤버", "interface": "멤버"}


def 본문_블록(본문: bytes) -> int:
    """**span 끝에서 매칭되는 여는 중괄호**의 자리. 없으면 `-1`.

    「첫 `{`」로 찾으면 반환 타입(`Promise<X & { … }>`)이나 구조분해 인자
    (`({ a, b }: Props)`)의 중괄호에 걸린다. 끝에서 세면 그것들은 안 걸린다 —
    본문 블록만이 심볼의 마지막 문자에서 닫히기 때문이다.
    """
    끝 = 본문.rstrip()
    if not 끝.endswith(b"}"):
        return -1
    깊이 = 0
    for i in range(len(끝) - 1, -1, -1):
        c = 끝[i : i + 1]
        if c == b"}":
            깊이 += 1
        elif c == b"{":
            깊이 -= 1
            if 깊이 == 0:
                return i
    return -1


def 의미를_바꾼다(본문: bytes, kind: str) -> bytes | None:
    """의미가 다른 본문. **문법이 안 맞으면 `None`** — 조용히 깨뜨리지 않는다."""
    형태 = 변형가능.get(kind)
    if 형태 is None:
        return None
    i = 본문_블록(본문)
    if i < 0:
        return None
    넣을 = b" if (0) { return null; }" if 형태 == "문" else b" __sem?: never;"
    return 본문[: i + 1] + 넣을 + 본문[i + 1 :]


def 겹치지_않는(nodes: list[dict]) -> list[dict]:
    """**서로 span 이 겹치지 않는 것만** 남긴다 — 바깥을 남기고 안쪽을 버린다.

    # ⚠ 이것 없이 한 번 틀렸다

    본문을 `span` 으로 바꿀 때 **뒤에서부터** 바꿔야 오프셋이 안 밀린다. 그런데
    **중첩된 심볼**(클래스 안의 메서드)은 안쪽의 `byte_start` 가 바깥보다 **크다** —
    내림차순으로 돌면 안쪽을 먼저 바꾸고, 그 순간 **바깥의 `byte_end` 가 무효**가 된다.
    그러면 바깥 심볼의 삽입이 엉뚱한 곳에 가고 그 결박은 안 켜진다.

    검출률 70% 가 그렇게 나왔다 — **감지기가 아니라 변형기가 틀린 것**이었다.
    """
    남길: list[dict] = []
    for n in sorted(nodes, key=lambda x: (x["path"], x["span"]["byte_start"],
                                          -x["span"]["byte_end"])):
        if 남길 and 남길[-1]["path"] == n["path"] \
           and n["span"]["byte_start"] < 남길[-1]["span"]["byte_end"]:
            continue  # 앞의 것 안에 들어 있다 — 바깥을 남긴다
        남길.append(n)
    return 남길


def 결박_걸기(repo: Path, box: Path, 이름들: list[str], n: int,
             radius: str = "symbol", at: str | None = None,
             intent: Path | None = None) -> int:
    """넉넉한 후보에서 **n 개가 걸릴 때까지** 건다. 걸린 수를 낸다.

    **실패를 조용히 넘기지 않는다** — 몇 개가 왜 안 걸렸는지 세고, 하한을 못 채우면
    부르는 쪽이 멈춘다.
    """
    intent = intent or (box / "intent.redb")
    걸림 = 0
    for s in 이름들:
        if 걸림 >= n:
            break
        cmd = [str(BIN), "bind", s, "--note", f"계약 {s}", "--radius", radius,
               "--repo", str(repo), "--cache-dir", str(box / "cache"),
               "--index", str(box / "index.redb"), "--intent", str(intent)]
        if at:
            cmd += ["--at", at]
        if run(cmd).returncode == 0:
            걸림 += 1
    return 걸림


def 결박들(repo: Path, box: Path) -> list[dict]:
    env = pal(["query", "binding.status", "--json"], repo, box)
    return env["answer"]["bindings"]


def 이층_상태(repo: Path, box: Path) -> tuple[int, int, bool]:
    """**대조가 꺼지는 다섯째 형태를 막는다** — 도구가 무엇을 읽는지 산출로 확인한다."""
    env = pal(["query", "graph.dump", "--read-only", "--json"], repo, box)
    a = env["answer"]
    return len(a["nodes"]), len(a["edges"]), env["projection"]["built_for_this_snapshot"]


def 심볼_고르기(repo: Path, box: Path, n: int, 파일_안에서: set[str] | None = None) -> list[str]:
    """**우리가 고르지 않는다** — `symbol_id` 사전순으로 정렬해 균등 간격으로 뽑는다.

    `[f09.4].sample_selection` 규칙 4 가 **측정보다 먼저** 등록한 그대로다.
    이름이 유일한 것만 쓴다 — `pal bind` 는 후보가 여럿이면 멈추고, 그 멈춤은 이 대조가
    재려는 것과 무관한 실패다.
    """
    env = pal(["query", "graph.dump", "--json"], repo, box)
    nodes = env["answer"]["nodes"]
    if 파일_안에서 is not None:
        # `[f09.4].sample_selection` 규칙 2 — **결박된 심볼이 실린 파일을 건드린
        # 커밋만 센다. 안 그러면 표본이 전부 「아무 일도 안 일어남」이 된다.**
        #
        # 그 규칙이 뜻하는 것은 **결박을 그런 파일의 심볼에 걸어야 한다**는 것이다.
        # 처음에는 이 절을 빼고 코퍼스 전체에서 균등하게 뽑았고, 39 커밋을 지나도
        # **아무것도 안 켜져서 대조 불가**가 났다 — 등록이 미리 적어 둔 그 형태다.
        nodes = [x for x in nodes if x["path"] in 파일_안에서]
        if not nodes:
            raise SystemExit("변경된 파일 안에 심볼이 하나도 없다")
    이름_수: dict[str, int] = {}
    for x in nodes:
        이름_수[x["name"]] = 이름_수.get(x["name"], 0) + 1
    유일 = sorted((x for x in nodes if 이름_수[x["name"]] == 1), key=lambda x: x["id"])
    if not 유일:
        raise SystemExit("유일한 이름의 심볼이 하나도 없다")
    # **넉넉히 뽑는다.** `pal bind` 는 좌표가 L0 이거나 이름이 2층에서 여럿으로
    # 해소되면 멈추고, 그 멈춤은 이 대조가 재려는 것과 무관한 실패다. 정확히 n 개만
    # 뽑으면 그중 하나만 걸려도 하한을 못 채운다 — 실제로 19/20 이 한 번 났다.
    #
    # **고르는 규칙은 그대로다** — 사전순 정렬 + 균등 간격. 넉넉함은 개수만 바꾼다.
    간격 = max(1, len(유일) // (n * 2))
    return [x["name"] for x in 유일[::간격]][: n * 2]


# ═════════════════════════════════════════════════════════════════════════════
# ① 합성 포매팅 — **진행 불가.** 그리고 ②가 그 짝이다
# ═════════════════════════════════════════════════════════════════════════════

def 합성_변형(tmp: Path, skip_prettier: bool) -> None:
    print("① 합성 포매팅 변형에서 `stale` 0 — **진행 불가 조건**")

    변형들: list[tuple[str, callable]] = [
        ("들여쓰기", lambda t: t.replace("\n  ", "\n    ")),
        ("개행", lambda t: t.replace(" { ", " {\n  ")),
        ("주석", lambda t: "// 이 줄이 더해졌다\n" + t),
        ("후행 공백", lambda t: "\n".join(l + "   " for l in t.split("\n"))),
    ]
    if not skip_prettier:
        변형들.insert(0, ("prettier@3", None))

    for 이름, 함수 in 변형들:
        repo, box = 사본(tmp, f"fmt-{이름.replace('@', '')}", DITTO, DITTO_PIN)
        결박_걸기(repo, box, 심볼_고르기(repo, box, MIN_BINDINGS), MIN_BINDINGS)

        걸린 = 결박들(repo, box)
        # **하한** — 결박이 없으면 「stale 0」이 공짜다.
        if len(걸린) < MIN_BINDINGS:
            fail(f"① {이름}", f"결박이 {len(걸린)}건뿐이다 (하한 {MIN_BINDINGS})")
            continue
        전_상태 = [b["status"]["code"]["freshness"] for b in 걸린]
        if any(f != "live" for f in 전_상태):
            fail(f"① {이름}", f"변형 **전**에 이미 live 가 아닌 것이 있다: {set(전_상태)}")
            continue

        전 = 소스_바이트(repo, ".ts")
        if 함수 is None:
            p = run(["npx", "--yes", "prettier@3", "--write", "--log-level", "warn", "**/*.ts"],
                    cwd=repo)
            if p.returncode != 0:
                skip(f"① {이름}", f"prettier 를 못 돌렸다 — **대조 불가**: {p.stderr[-200:]}")
                continue
        else:
            for f in repo.rglob("*.ts"):
                if "node_modules" in f.parts:
                    continue
                f.write_text(함수(f.read_text(encoding="utf-8")), encoding="utf-8")
        후 = 소스_바이트(repo, ".ts")
        바뀐, 바이트 = 변형이_먹었나(전, 후, 이름)

        뒤 = 결박들(repo, box)
        stale = sum(1 for b in 뒤 if b["status"]["code"]["freshness"] == "stale")
        노드, 엣지, 이_스냅샷 = 이층_상태(repo, box)
        값 = (f"결박 {len(뒤)} · 바뀐 파일 {바뀐} · stale **{stale}** "
              f"(상한 {SYNTHETIC_STALE_MAX}) · 2층 노드 {노드}/엣지 {엣지}")
        if stale > SYNTHETIC_STALE_MAX:
            나쁜 = [b["binding"][:8] for b in 뒤 if b["status"]["code"]["freshness"] == "stale"]
            fail(f"① {이름}", f"{값} — 어긋난 결박: {나쁜[:10]}")
        else:
            ok(f"① {이름}", 값)
        shutil.rmtree(box, ignore_errors=True)


def 의미_변형(tmp: Path) -> None:
    """★ **①의 짝.** 없으면 ①이 「아무것도 안 켜는 감지기」로 만점을 받는다.

    # ⚠ 이 함수를 한 번 잘못 썼고, 그것이 대조가 꺼지는 형태였다

    처음에는 파일의 **첫 `return`** 을 바꿨다. 그런데 결박된 심볼이 그 파일의 첫 함수가
    아니면 **변형이 결박된 본문을 안 건드린다** — 검출률이 5% 로 나왔고 그것은 제품이
    아니라 **대조가 틀린 것**이었다(F03 의 *"변형 대상이 이미 있어서 아무것도 안 바꿈"*
    과 같은 형태).

    그래서 **`span` 으로 그 심볼의 본문 안을 바꾼다.** 그리고 그 변형이 실제로 먹었는지를
    **결박마다** 센다 — 파일 수로 세면 같은 함정에 다시 빠진다.

    # 바이트 자리다 — 문자 자리가 아니다

    `span` 은 바이트 오프셋이고 이 코퍼스에는 한글 주석이 있다. 문자 자리로 자르면
    엉뚱한 곳을 건드린다(F03 이 실제로 겪은 아홉 중 하나).
    """
    print("② ★ 의미를 바꾸면 반드시 `stale` — ①의 짝")

    repo, box = 사본(tmp, "semantic", DITTO, DITTO_PIN)
    결박_걸기(repo, box, 심볼_고르기(repo, box, MIN_BINDINGS), MIN_BINDINGS)
    걸린 = 결박들(repo, box)
    if len(걸린) < MIN_BINDINGS:
        fail("② 의미 변경", f"결박이 {len(걸린)}건뿐이다 (하한 {MIN_BINDINGS})")
        return

    env = pal(["query", "graph.dump", "--json"], repo, box)
    좌표 = {n["id"]: n for n in env["answer"]["nodes"]}

    # **파일마다 뒤에서부터 바꾼다** — 앞에서 바꾸면 뒤의 오프셋이 밀린다.
    대상_노드 = 겹치지_않는([좌표[b["target"]] for b in 걸린 if b["target"] in 좌표])
    겹쳐서_뺀것 = len(걸린) - len(대상_노드)
    파일별: dict[str, list[dict]] = {}
    for n in 대상_노드:
        파일별.setdefault(n["path"], []).append(n)

    # **이 변형이 몇 개의 결박된 본문을 실제로 건드렸는가** — 대상 수를 여기서 센다.
    건드린 = 0
    건드린_id: set[str] = set()
    못한_종류: dict[str, int] = {}
    for path, nodes in 파일별.items():
        f = repo / path
        raw = f.read_bytes()
        for n in sorted(nodes, key=lambda x: x["span"]["byte_start"], reverse=True):
            s0, s1 = n["span"]["byte_start"], n["span"]["byte_end"]
            if not (0 <= s0 < s1 <= len(raw)):
                continue
            새_본문 = 의미를_바꾼다(raw[s0:s1], n["kind"])
            if 새_본문 is None:
                # **조용히 깨뜨리지 않는다.** 문법이 안 맞는 종류는 빼고 그 사실을 적는다.
                못한_종류[n["kind"]] = 못한_종류.get(n["kind"], 0) + 1
                continue
            raw = raw[:s0] + 새_본문 + raw[s1:]
            건드린 += 1
            건드린_id.add(n["id"])
        f.write_bytes(raw)

    # **하한** — 결박된 본문을 충분히 안 건드렸으면 이 대조는 아무것도 안 잰다.
    # 겹쳐서 뺀 것은 **분모에서도 빠진다**(아래에서 `건드린` 을 분모로 쓴다).
    if 건드린 < MIN_SURVIVING:
        fail("② 의미 변경",
             f"결박된 본문을 {건드린}/{len(걸린)} 개만 건드렸다 (하한 {MIN_SURVIVING}) — "
             f"**변형이 대상을 못 찾았다**")
        shutil.rmtree(box, ignore_errors=True)
        return

    # **분모는 실제로 본문을 바꾼 결박들이다.** 겹쳐서 뺀 것을 분모에 넣으면
    # *"안 건드렸는데 안 켜졌다"* 가 검출 실패로 세어진다.
    건드린_대상 = 건드린_id
    뒤 = [b for b in 결박들(repo, box) if b["target"] in 건드린_대상]
    갈래: dict[str, int] = {}
    for b in 뒤:
        f = b["status"]["code"]["freshness"]
        갈래[f] = 갈래.get(f, 0) + 1

    # ⚠ **`orphaned`·`undeterminable` 은 검출 실패가 아니다 — 변형이 파일을 깨뜨린 것이다.**
    #
    # 삽입이 파싱을 깨면 두 갈래가 나온다: 심볼이 통째로 사라지면 `orphaned` 이고,
    # 파일이 **부분 파싱**되면 `undeterminable{partial-parse}` 다. 둘 다 **감지기가 못
    # 잡은 것이 아니라 우리가 대상을 훼손한 것**이고, 뭉개면 「검출률」이 감지기가 아니라
    # **우리 변형기의 품질**을 재게 된다.
    #
    # ★ **그리고 이것이 이 회차의 가장 좋은 관측이다** — `PartialParse` 가 실물에서
    # 실제로 켜졌다. 픽스처가 아니라 **코퍼스 위에서** 난 사유이고, `[f09.pass]` 의
    # *"사유 최소 둘이 실제로 산출"* 이 그것으로 선다.
    #
    # **분모는 살아남은 것들이다** — `stale` + `live`. 그 수에 하한을 박는다:
    # 살아남은 것이 적으면 비율이 표본 하나에 흔들린다.
    stale = 갈래.get("stale", 0)
    live = 갈래.get("live", 0)
    깨진것 = 갈래.get("orphaned", 0) + 갈래.get("undeterminable", 0)
    살아남은 = stale + live
    # ★ 실물에서 난 사유를 모은다 — ⑥의 하한(사유 최소 둘)이 이것을 쓴다.
    for b in 뒤:
        c = b["status"]["code"]
        if c["freshness"] == "undeterminable":
            관측된_사유.add(c["reason"])
    if 살아남은 < MIN_SURVIVING:
        fail("② 의미 변경",
             f"변형 뒤 살아남은 결박이 {살아남은}건뿐이다 (하한 {MIN_SURVIVING}) · 갈래 {갈래} — "
             f"**변형이 파일을 너무 많이 깨뜨렸다.** 검출률이 표본 하나에 흔들린다")
        shutil.rmtree(box, ignore_errors=True)
        return

    비율 = round(100 * stale / 살아남은, 1)
    값 = (f"결박 {len(뒤)} · 본문 {건드린}개 변형 (겹침 {겹쳐서_뺀것} · 문법이 안 맞는 종류 {못한_종류}) "
          f"· 갈래 {갈래} · "
          f"stale {stale}/{살아남은} = **{비율}%** (하한 {SEMANTIC_DETECTION_MIN_PCT}%) "
          f"· 변형이 훼손한 파일의 결박 {깨진것}건은 분모 밖")
    if 비율 < SEMANTIC_DETECTION_MIN_PCT:
        안_켜진 = [b["binding"][:8] for b in 뒤 if b["status"]["code"]["freshness"] == "live"]
        fail("② 의미 변경", f"{값} — **본문을 바꿨는데 live 인 결박**: {안_켜진[:10]}")
    else:
        ok("② 의미 변경", 값)
    shutil.rmtree(box, ignore_errors=True)


# ═════════════════════════════════════════════════════════════════════════════
# ③⑥ 판정 불가 — **상한과 하한이 함께 걸린다**
# ═════════════════════════════════════════════════════════════════════════════

def 판정_불가(tmp: Path) -> None:
    print("③⑥ `Undeterminable` — `Live` 로 안 새고, 지배하지도 않는다")

    repo, box = 사본(tmp, "undet", PORTAL, PORTAL_PIN)
    # ⚠ **Kotlin 코퍼스다.** `ordinal_is_not_undeterminable` 이 재는 자리 —
    # `ordinal` 을 사유로 넣었으면 여기서 100% 가 나온다.
    결박_걸기(repo, box, 심볼_고르기(repo, box, MIN_BINDINGS), MIN_BINDINGS, at=PORTAL_PIN)

    env = json.loads(run([str(BIN), "query", "binding.status", "--repo", str(repo),
                          "--at", PORTAL_PIN, "--cache-dir", str(box / "cache"),
                          "--index", str(box / "index.redb"),
                          "--intent", str(box / "intent.redb"), "--json"]).stdout)
    걸린 = env["answer"]["bindings"]
    if len(걸린) < MIN_BINDINGS:
        fail("⑥ 비율", f"결박이 {len(걸린)}건뿐이다 (하한 {MIN_BINDINGS})")
        return

    판정불가 = [b for b in 걸린 if b["status"]["code"]["freshness"] == "undeterminable"]
    비율 = round(100 * len(판정불가) / len(걸린), 1)
    등급 = {}
    for b in 걸린:
        for g, n in b["watch_grades"].items():
            등급[g] = 등급.get(g, 0) + n
    값 = f"결박 {len(걸린)} · 판정 불가 {len(판정불가)} = **{비율}%** (상한 {UNDETERMINABLE_RATIO_MAX_PCT}%) · 감시 등급 {등급}"
    if 비율 > UNDETERMINABLE_RATIO_MAX_PCT:
        fail("⑥ 비율 상한", 값)
    else:
        ok("⑥ 비율 상한", 값)

    # ★ **`ordinal` 이 판정 불가로 접히지 않았다는 증거다.**
    if 등급.get("ordinal", 0) > 0 and 비율 > 90:
        fail("⑥ ordinal", f"ordinal 이 {등급['ordinal']}개인데 판정 불가가 {비율}% 다 — 접혔다")
    elif 등급.get("ordinal", 0) > 0:
        ok("⑥ ★ ordinal 을 안 접었다",
           f"ordinal 감시 원소 {등급['ordinal']}개인데 판정 불가 {비율}% — 비교가 돌았다")
    else:
        skip("⑥ ★ ordinal", "이 코퍼스에 ordinal 감시 원소가 없다 — **대조 불가**")

    # ③ **`Live` 로 새지 않는다** — 사유를 만들어 세운다.
    #
    # `ProjectionStale` 은 커밋 축에서 만든다: 2층이 옛 스냅샷에 선 채로 **읽기 전용**
    # 으로 물으면 판정할 수 없다. `--read-only` 가 없으면 스티칭이 다시 돌아 증상을
    # 복구해 버린다(대조가 꺼지는 다섯째).
    앞 = run(["git", "-C", str(repo), "rev-parse", f"{PORTAL_PIN}~3"]).stdout.strip()
    p = run([str(BIN), "query", "binding.status", "--repo", str(repo), "--at", 앞,
             "--read-only", "--cache-dir", str(box / "cache"),
             "--index", str(box / "index.redb"), "--intent", str(box / "intent.redb"), "--json"])
    if p.returncode != 0:
        skip("③ 판정 불가", f"읽기 전용 회차를 못 돌렸다 — **대조 불가**: {p.stderr[-200:]}")
    else:
        e2 = json.loads(p.stdout)
        이_스냅샷 = e2["projection"]["built_for_this_snapshot"]
        상태 = [b["status"]["code"] for b in e2["answer"]["bindings"]]
        샌_것 = [c for c in 상태 if c["freshness"] == "live"]
        사유 = sorted({c.get("reason") for c in 상태 if c["freshness"] == "undeterminable"})
        관측된_사유.update(x for x in 사유 if x)
        값 = (f"2층이 이 스냅샷 것인가 {이_스냅샷} · 판정 불가 {len(상태) - len(샌_것)}/{len(상태)} · "
              f"사유 {사유} · **`live` 로 샌 것 {len(샌_것)}** (상한 {UNDETERMINABLE_LEAK_MAX})")
        if 이_스냅샷:
            skip("③ 판정 불가", f"2층이 이 스냅샷 것이라 이 대조가 안 켜졌다 — **대조 불가**: {값}")
        elif len(샌_것) > UNDETERMINABLE_LEAK_MAX:
            fail("③ 판정 불가가 `live` 로 샜다", 값)
        else:
            ok("③ 판정 불가가 `live` 로 안 샌다", 값)

    shutil.rmtree(box, ignore_errors=True)


# ═════════════════════════════════════════════════════════════════════════════
# ④ Orphaned ≠ Stale — **지우면 Orphaned · 고치면 Stale**
# ═════════════════════════════════════════════════════════════════════════════

def 사라짐과_변함(tmp: Path) -> None:
    print("④ ★ `Orphaned` ≠ `Stale` — 지우면 사라짐 · 고치면 낡음")

    repo, box = 사본(tmp, "orphan", DITTO, DITTO_PIN)
    결박_걸기(repo, box, 심볼_고르기(repo, box, MIN_BINDINGS), MIN_BINDINGS)
    걸린 = 결박들(repo, box)
    if len(걸린) < MIN_BINDINGS:
        fail("④ Orphaned", f"결박이 {len(걸린)}건뿐이다 (하한 {MIN_BINDINGS})")
        return

    env = pal(["query", "graph.dump", "--json"], repo, box)
    좌표 = {n["id"]: n for n in env["answer"]["nodes"]}
    파일들 = sorted({좌표[b["target"]]["path"] for b in 걸린 if b["target"] in 좌표})
    # 절반은 **지우고** 절반은 **고친다** — 한 회차에서 둘이 갈리는지 본다.
    지울것, 고칠것 = 파일들[: len(파일들) // 2], 파일들[len(파일들) // 2 :]
    전 = 소스_바이트(repo, ".ts")
    for path in 지울것:
        (repo / path).write_text("export const 비었다 = 1\n", encoding="utf-8")
    # **고치는 쪽은 `span` 으로 그 심볼의 본문 안을 바꾼다.** 파일 끝에 붙이면 결박된
    # 본문이 안 움직이고, 그러면 「고쳤는데 stale 이 아니다」가 제품이 아니라 우리
    # 변형기의 사고가 된다(②에서 실제로 밟은 자리다).
    # **겹치는 것을 뺀다** — 중첩 심볼에서 안쪽을 먼저 바꾸면 바깥의 오프셋이 무효가
    # 된다(`겹치지_않는` 의 머리가 그 사고를 적었다).
    고칠_노드: dict[str, list[dict]] = {}
    for n in 겹치지_않는([좌표[b["target"]] for b in 걸린
                        if b["target"] in 좌표 and 좌표[b["target"]]["path"] in 고칠것]):
        고칠_노드.setdefault(n["path"], []).append(n)
    고친_수 = 0
    for path, nodes in 고칠_노드.items():
        f = repo / path
        raw = f.read_bytes()
        # **뒤에서부터** — 앞에서 바꾸면 뒤의 오프셋이 밀린다.
        for n in sorted(nodes, key=lambda x: x["span"]["byte_start"], reverse=True):
            s0, s1 = n["span"]["byte_start"], n["span"]["byte_end"]
            if not (0 <= s0 < s1 <= len(raw)):
                continue
            # ②와 **같은 함수**를 쓴다 — 변형기가 두 벌이면 그 둘이 갈린다.
            새 = 의미를_바꾼다(raw[s0:s1], n["kind"])
            if 새 is None:
                continue
            raw = raw[:s0] + 새 + raw[s1:]
            고친_수 += 1
        f.write_bytes(raw)
    후 = 소스_바이트(repo, ".ts")
    변형이_먹었나(전, 후, "지우기·고치기")

    # ④는 **전부**를 본다 — 이 항목이 재는 것은 비율이 아니라 *"둘이 갈리는가"* 다.
    뒤 = 결박들(repo, box)
    갈래: dict[str, int] = {}
    for b in 뒤:
        f = b["status"]["code"]["freshness"]
        갈래[f] = 갈래.get(f, 0) + 1
        if f == "undeterminable":
            관측된_사유.add(b["status"]["code"]["reason"])
    값 = f"결박 {len(뒤)} · 지운 파일 {len(지울것)} · 고친 본문 {고친_수} · 갈래 {갈래}"
    # **하한** — 한쪽 변형이 아무것도 안 했으면 「둘이 갈린다」가 공짜다.
    if 고친_수 < 1 or not 지울것:
        fail("④ 둘이 안 갈린다", f"{값} — 한쪽 변형이 아무것도 안 했다")
    # **둘이 다 나와야 한다.** 한쪽만 나오면 그것은 뭉갠 것이다.
    elif 갈래.get("orphaned", 0) > 0 and 갈래.get("stale", 0) > 0:
        ok("④ 둘이 갈린다", 값)
    else:
        fail("④ 둘이 안 갈린다", f"{값} — `orphaned` 와 `stale` 이 둘 다 나와야 한다")
    shutil.rmtree(box, ignore_errors=True)


# ═════════════════════════════════════════════════════════════════════════════
# ⑤ 실 이력 표본 — **거짓 양성률.** 선정 규칙은 측정보다 먼저 등록됐다
# ═════════════════════════════════════════════════════════════════════════════

def 실_이력(tmp: Path, corpus: Path, pin: str, ext: str, tag: str, radius: str, 상한: int) -> None:
    """실 이력 표본 — **거짓 양성률의 재료.**

    # ⚠ 방향을 한 번 거꾸로 썼고, 그것이 대조를 껐다

    처음에는 **핀(최신)에 결박을 걸고 과거를 봤다.** 그러면 옛 커밋에는 그 심볼이 아직
    없어서 표본 20 건이 **전부 `orphaned`** 로 나왔다 — 재려던 것(*"코드가 변해서
    낡았는가"*)이 아니라 *"그때 아직 없었는가"* 를 잰 것이다.

    **결박은 과거에 걸고 앞으로 온다.** 그것이 사람이 실제로 하는 일이기도 하다 —
    결정을 쓰고, 그 뒤에 코드가 변한다.

    # 거짓 양성의 판정은 이 스크립트가 안 한다

    `[f09.pass]` 가 정의를 못 박았다 — *"결박이 `stale` 인데 그 커밋의 변경이 결박된
    메모의 **유효성을 건드리지 않는** 경우. 판정은 사람(에이전트)이 하고 **판정의 근거를
    결박마다 한 줄로 남긴다.**"* 그러므로 이 함수는 **표본과 그 갈래를 낸다.**
    """
    print(f"⑤ 실 이력 · {tag} · 반경 {radius} — 거짓 양성률 상한 {상한}%")

    repo, box = 사본(tmp, f"hist-{tag.replace('(', '').replace(')', '')}-{radius.replace(':', '')}",
                    corpus, pin)
    # `[f09.4].sample_selection` — 규칙 2·3: 핀에서 뒤로 세되 **머지 커밋을 뺀다**
    # (변경 집합이 부모에 따라 달라져 *"이 커밋이 무엇을 바꿨나"* 가 하나로 안 정해진다).
    커밋들 = run(
        ["git", "-C", str(repo), "log", "--no-merges", "--format=%H", "-n", str(HISTORY_WINDOW), pin]
    ).stdout.split()
    if len(커밋들) < 5:
        skip(f"⑤ {tag}/{radius}", f"머지 아닌 커밋이 {len(커밋들)}개뿐이다 — **대조 불가**")
        shutil.rmtree(box, ignore_errors=True)
        return

    # **가장 오래된 것에 결박을 걸고 앞으로 온다.** `log` 는 최신 순이므로 끝이 옛것이다.
    시작 = 커밋들[-1]
    run(["git", "-C", str(repo), "checkout", "-q", 시작])

    # `[f09.4].sample_selection` 규칙 2 — **이 구간에서 실제로 변경된 파일**의 심볼에
    # 건다. 안 그러면 표본이 전부 「아무 일도 안 일어남」이 되고, 그것을 이 스크립트가
    # 한 번 실제로 냈다(39 커밋 · 켜진 것 0 · 대조 불가).
    변경된 = {
        x for x in run(
            ["git", "-C", str(repo), "diff", "--name-only", f"{시작}..{pin}"]
        ).stdout.split()
        if x.endswith(ext)
    }
    if len(변경된) < 3:
        skip(f"⑤ {tag}/{radius}",
             f"이 구간에서 바뀐 `{ext}` 파일이 {len(변경된)}개뿐이다 — **대조 불가**")
        shutil.rmtree(box, ignore_errors=True)
        return
    결박_걸기(repo, box, 심볼_고르기(repo, box, MIN_BINDINGS, 파일_안에서=변경된),
             MIN_BINDINGS, radius=radius, at=시작)

    def 상태들(at: str) -> list[dict]:
        p = run([str(BIN), "query", "binding.status", "--repo", str(repo), "--at", at,
                 "--cache-dir", str(box / "cache"), "--index", str(box / "index.redb"),
                 "--intent", str(box / "intent.redb"), "--json"])
        return json.loads(p.stdout)["answer"]["bindings"] if p.returncode == 0 else []

    기준 = 상태들(시작)
    # **하한 둘** — 결박이 적으면 표본이 안 서고, **걸자마자 live 가 아니면** 그 뒤의
    # 관측이 코드 변화가 아니라 결박 시점의 사고를 재게 된다.
    if len(기준) < MIN_BINDINGS:
        fail(f"⑤ {tag}/{radius}", f"결박이 {len(기준)}건뿐이다 (하한 {MIN_BINDINGS})")
        shutil.rmtree(box, ignore_errors=True)
        return
    안_live = [b["status"]["code"]["freshness"] for b in 기준 if b["status"]["code"]["freshness"] != "live"]
    if 안_live:
        fail(f"⑤ {tag}/{radius}", f"결박 직후에 live 가 아닌 것이 {len(안_live)}건 있다: {set(안_live)}")
        shutil.rmtree(box, ignore_errors=True)
        return

    # **앞으로 오면서** 켜지는 것을 모은다 — 옛것부터 최신까지.
    #
    # ⚠ **여기 기록되는 커밋은 「처음 켜진 것을 **관측한** 커밋」이다.**
    # 훑는 목록이 `--no-merges` 로 걸러졌으므로(등록 규칙 3) **머지로 들어온 변경은
    # 건너뛴 커밋에 있다** — 그래서 이 커밋의 diff 만 보면 원인이 안 보일 수 있다.
    # 그러므로 판정 재료를 **구간**(직전 관측 커밋 → 이 커밋)으로 낸다.
    # 좌표 → 이름. **무엇이 켰는지**를 사람이 읽을 수 있어야 판정이 된다.
    _노드 = pal(["query", "graph.dump", "--json"], repo, box)["answer"]["nodes"]
    좌표_이름 = {n["id"]: n["name"] for n in _노드}
    # **경로도 낸다.** Kotlin 은 파일명 ≠ 심볼명이 흔해서 이름만으로는 판정 재료를
    # 못 찾는다 — 실제로 일곱 중 넷을 못 찾았다. 좌표가 이미 알고 있는 값이다.
    좌표_경로 = {n["id"]: n["path"] for n in _노드}
    표본: list[tuple] = []  # (커밋, 결박, 갈래, 메모, 커밋 제목, 켠 것)
    이미: set[str] = set()
    직전 = 시작
    for c in reversed(커밋들[:-1]):
        for b in 상태들(c):
            f = b["status"]["code"]["freshness"]
            if f == "undeterminable":
                관측된_사유.add(b["status"]["code"]["reason"])
            # **결박마다 처음 켜진 자리 하나만 센다** — 한 번 켜지면 그 뒤로 계속
            # 켜져 있으므로, 안 그러면 표본이 「커밋 수 × 결박 수」로 부풀고 같은 사건이
            # 여러 번 세어진다.
            if f in ("stale", "orphaned") and b["binding"] not in 이미:
                이미.add(b["binding"])
                # **판정 재료를 함께 싣는다** — 거짓 양성의 판정은 사람이 하고
                # 근거를 결박마다 한 줄로 남겨야 한다(`[f09.pass]` 의 정의).
                제목 = run(["git", "-C", str(repo), "log", "-1", "--format=%s", c]).stdout.strip()
                켠_것 = [좌표_이름.get(x, x[:8])
                        for x in b["status"]["code"].get("triggered_by", [])]
                표본.append((f"{직전[:8]}..{c[:8]}", b["binding"][:8], f,
                            좌표_경로.get(b["target"], "?"), 제목[:44], 켠_것[:2]))
        직전 = c
        if len(표본) >= SAMPLES_PER_CORPUS:
            break

    if not 표본:
        skip(f"⑤ {tag}/{radius}",
             f"커밋 {len(커밋들) - 1}개를 지나며 켜진 것이 하나도 없다 "
             f"(바뀐 파일 {len(변경된)}개의 심볼에 걸었는데도) — **대조 불가**")
        shutil.rmtree(box, ignore_errors=True)
        return

    표본 = 표본[:SAMPLES_PER_CORPUS]
    켜진 = sum(1 for x in 표본 if x[2] == "stale")
    사라짐 = sum(1 for x in 표본 if x[2] == "orphaned")
    # ⚠ **거짓 양성의 판정은 사람(에이전트)이 한다.** 이 스크립트는 표본을 낸다 —
    # 판정과 근거는 게이트에 **결박마다 한 줄로** 적힌다(`[f09.pass]` 의 정의).
    ok(f"⑤ {tag}/{radius} 표본",
       f"커밋 {len(커밋들) - 1}개 훑음 · 바뀐 파일의 심볼에 결박 · 표본 {len(표본)} · "
       f"stale {켜진} · orphaned {사라짐} "
       f"— **거짓 양성 판정은 게이트에 목록으로** (상한 {상한}%)")
    for c, b, f, path, 제목, 켠 in 표본:
        print(f"      {c}  {b}  {f:<9} {'·'.join(켠) or '—':<26} {path[-52:]:<52} | {제목}")
    shutil.rmtree(box, ignore_errors=True)


# ═════════════════════════════════════════════════════════════════════════════
# ⑦ 반경 — **넓히면 커진다**
# ═════════════════════════════════════════════════════════════════════════════

def 반경별(tmp: Path) -> None:
    print("⑦ 반경별 감시 집합 — **넓히면 커져야 한다**")

    repo, box = 사본(tmp, "radius", DITTO, DITTO_PIN)
    이름들 = 심볼_고르기(repo, box, MIN_BINDINGS)
    크기: dict[str, int] = {}
    for r in ["symbol", "callers", "closure:2", "closure:3"]:
        # **반경마다 의도 저장소를 새로 만든다** — 같은 좌표·같은 조각이면 결박 id 가
        # 같아서 덮어써지고, 그러면 앞 반경의 값이 남는다.
        intent = box / f"intent-{r.replace(':', '')}.redb"
        결박_걸기(repo, box, 이름들, MIN_BINDINGS, radius=r, intent=intent)
        p = run([str(BIN), "query", "binding.status", "--repo", str(repo),
                 "--cache-dir", str(box / "cache"), "--index", str(box / "index.redb"),
                 "--intent", str(intent), "--json"])
        bs = json.loads(p.stdout)["answer"]["bindings"]
        크기[r] = sum(b["watch"] for b in bs)

    값 = " · ".join(f"{r} {n}" for r, n in 크기.items())
    # ★ **반대 방향**: 안 커지면 반경이 아무것도 안 가른다.
    if 크기["callers"] > 크기["symbol"] and 크기["closure:2"] >= 크기["callers"]:
        ok("⑦ 반경을 넓히면 커진다", 값)
    else:
        fail("⑦ 반경이 안 커진다", f"{값} — 반경이 아무것도 안 가른다")
    shutil.rmtree(box, ignore_errors=True)


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--skip-prettier", action="store_true")
    a = ap.parse_args()

    if not BIN.exists():
        print(f"바이너리가 없다: {BIN}", file=sys.stderr)
        return 1
    for p in (DITTO, PORTAL):
        if not p.exists():
            print(f"코퍼스가 없다: {p} — **대조 불가**", file=sys.stderr)
            return 1

    print("F09 — 결박과 낡음\n")
    tmp = Path(tempfile.mkdtemp(prefix="f09-"))
    try:
        합성_변형(tmp, a.skip_prettier)
        의미_변형(tmp)
        판정_불가(tmp)
        사라짐과_변함(tmp)
        # `[f09.4].sample_selection` 규칙 1 — **코퍼스 둘에서 각각 10 건, 합 20.**
        # 등급 축을 가르기 위해서다(ditto = `exact` · portal-backend = `ordinal`).
        실_이력(tmp, DITTO, DITTO_PIN, ".ts", "ditto(exact)", "symbol", FALSE_POSITIVE_SYMBOL_MAX_PCT)
        실_이력(tmp, PORTAL, PORTAL_PIN, ".kt", "portal(ordinal)", "symbol", FALSE_POSITIVE_SYMBOL_MAX_PCT)
        실_이력(tmp, DITTO, DITTO_PIN, ".ts", "ditto(exact)", "callers", FALSE_POSITIVE_CALLERS_MAX_PCT)
        실_이력(tmp, PORTAL, PORTAL_PIN, ".kt", "portal(ordinal)", "callers", FALSE_POSITIVE_CALLERS_MAX_PCT)
        반경별(tmp)
    finally:
        shutil.rmtree(tmp, ignore_errors=True)

    # ⑥의 하한 — **사유 넷 중 최소 둘이 실제로 산출돼야 한다.**
    #
    # 0% 는 통과가 **아니다.** *"판정 불가가 없다"* 가 아니라 **아무 데서도 안 켜진다**는
    # 뜻이고, 그것은 선행 구현이 `stale=False` 로 접었던 자리로 돌아간 것이다([R16]).
    if len(관측된_사유) >= UNDETERMINABLE_MIN_REASONS:
        ok("⑥ ★ 비율 하한 (사유가 실제로 산출된다)",
           f"코퍼스에서 난 사유 {sorted(관측된_사유)} — {len(관측된_사유)}개 (하한 {UNDETERMINABLE_MIN_REASONS})")
    else:
        fail("⑥ ★ 비율 하한",
             f"코퍼스에서 난 사유가 {sorted(관측된_사유)} 뿐이다 (하한 {UNDETERMINABLE_MIN_REASONS}) — "
             f"**`Undeterminable` 이 아무 데서도 안 켜지면 접은 것과 같다**")

    print()
    for 표시, 이름, 값 in 결과:
        print(f"  {표시:<5} {이름}  — {값}")

    어긋남 = [f"{n}: {v}" for m, n, v in 결과 if m == "FAIL"]
    불가 = [f"{n}: {v}" for m, n, v in 결과 if m == "–"]
    print()
    if 불가:
        print(f"대조 불가 {len(불가)}건:")
        for x in 불가:
            print(f"  – {x}")
        print()
    if 어긋남:
        print(f"어긋난 것 {len(어긋남)}:")
        for x in 어긋남:
            print(f"   · {x}")
        return 1
    print(f"어긋남 0 · 대조 불가 {len(불가)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
