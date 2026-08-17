#!/usr/bin/env python3
"""[f06b] 의 대조 — 어댑터가 CLI 와 갈릴 수 있는가. 그리고 없어도 아무것도 안 죽는가.

합격선 정본은 `corpus/criteria.toml` `[f06b.pass]`. 이 스크립트가 지는 것은 **둘**이다:

  ④ **음성 대조** — 카탈로그를 망가뜨리면 세션 시험이 실패한다(변형 셋 · 매번 복원)
  ⑥ **어댑터 부재** — 뺀 빌드에서 전건 통과 + 그 빌드에 `serve` 가 **없다**

①②③⑧ 은 `crates/pal-cli/tests/mcp_session.rs` 가 지고 ⑤⑦ 은 `cargo xtask check` 가
진다. **여기서 다시 재지 않는다** — 같은 것을 두 곳에서 재면 한쪽이 조용히 낡는다.

# 왜 ④ 가 스크립트인가

*"카탈로그를 망가뜨리면 실패한다"* 는 **시험 스스로 잴 수 없다.** 시험이 자기가 읽는
파일을 고치면 그 시험은 자기를 재는 것이고, 게다가 `include_str!` 은 컴파일 시점에
읽히므로 재빌드가 필요하다. 그래서 밖에서 고치고 밖에서 돌린다.

⚠ **변형이 아무것도 안 바꾸면 실패로 적는다** — 변형 뒤 파일이 원본과 같으면 멈춘다
(대조가 꺼지는 형태 ①). 그리고 **변형 전에 초록인 것을 먼저 확인한다** — 원래 빨간
것을 망가뜨리고 *"빨개졌다"* 고 적으면 아무것도 안 잰 것이다.
"""

from __future__ import annotations

import argparse
import shutil
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
CATALOG = ROOT / "surface" / "queries.toml"

# ⚠ **`--nocapture` 를 단다** — ⑧ 의 실측표가 `eprintln!` 로 나오고, 없으면 그 표를
# **저장소의 어떤 명령으로도 재현할 수 없다**(판정에 인용된 수가 사람이 한 번 손으로
# 띄운 산출이 된다).
SESSION_TEST = ["cargo", "test", "-p", "pal-cli", "--test", "mcp_session", "--", "--nocapture"]


# **하한** — 「전건 통과」의 바닥. 지금 서 있는 수(381)보다 넉넉히 낮게 잡는다:
# 시험이 줄면 그것대로 드러나야 하지만, 몇 개 늘고 주는 것으로 이 대조가 깨지면
# **자라는 값에 묶인 변이**가 되어 조용히 꺼진다(F22-1 의 `7fe6b62` 형태).
뺀_빌드_최소_시험 = 300


def 돌린다(args: list[str], **kw) -> subprocess.CompletedProcess:
    """전 출력을 잡아 둔다 — **파이프로 잘라서 `error[E` 를 날리지 않는다.**"""
    return subprocess.run(args, cwd=ROOT, capture_output=True, text=True, **kw)


# ─────────────────────────────────────────────────────────────────────────────
# ④ 카탈로그를 망가뜨리면 세션 시험이 실패한다
# ─────────────────────────────────────────────────────────────────────────────

def 이름_하나를_뺀다(text: str) -> str:
    """방향 2 — **툴에 있는데 카탈로그에 없다.**

    코드는 그대로 열 개를 내고 카탈로그가 아홉이 된다. 정본(`[f06b.pass]` ④ 의 표)이
    이 변형을 **방향 2** 로 적는다.
    """
    i = text.index('[query."symbol.contains"]')
    j = text.index("[query.", i + 10)
    return text[:i] + text[j:]


def 없는_이름을_더한다(text: str) -> str:
    """방향 1 — **카탈로그에 있는데 툴에 없다.** 정본이 이 변형을 방향 1 로 적는다."""
    return text + (
        '\n[query."no.such.query"]\n'
        'summary    = "없는 것"\n'
        "args       = []\n"
        'returns    = "Nothing"\n'
        'introduced = "F99"\n'
    )


def 인자를_비운다(text: str) -> str:
    """방향 3 — 이름은 같은데 인자가 어긋난다."""
    old = '[query."symbol.resolve"]\nsummary    = "이름 하나 → 후보 심볼들. **여럿인 것이 정상이다**"\nargs       = [{ name = "name", type = "SymbolName", required = true }]'
    if old not in text:
        raise SystemExit("변형 3 의 치환 대상이 카탈로그에 없다 — 변형이 낡았다")
    return text.replace(
        old,
        '[query."symbol.resolve"]\nsummary    = "이름 하나 → 후보 심볼들. **여럿인 것이 정상이다**"\nargs       = []',
        1,
    )


# ★ **방향 라벨은 정본을 그대로 옮긴다** — `corpus/criteria.toml` `[f06b.pass]` ④ 의 표.
#
# ⚠ **앞 판은 1 과 2 의 라벨이 뒤집혀 있었다.** 카탈로그에서 이름을 빼면 **툴이 더
# 많아지므로** 그것은 방향 2 다. rc 만 보던 동안 그 뒤집힘이 아무 산출도 안 바꿨고,
# 그래서 **아무도 안 울었다** — 라벨이 아무것도 안 세고 있었다는 증거다.
#
# 셋째 칸은 **그 방향이 울었다는 증거 문자열**이다. `mcp_session.rs` 의 assert 메시지에서
# 그대로 왔다. **rc 만 보면 컴파일 실패·카탈로그 파싱 실패·다른 시험의 실패가 전부
# 「걸렸다」로 적힌다** — 이 저장소가 이미 등록해 둔 `대조 exit 0 ≠ 판정` 의 짝이다.
변형들 = [
    ("1 · 이름 하나를 뺀다", 이름_하나를_뺀다, "방향 2", "을 내는데 카탈로그에 없다"),
    ("2 · 없는 이름을 더한다", 없는_이름을_더한다, "방향 1", "이 MCP 표면에 없다"),
    ("3 · 인자를 비운다", 인자를_비운다, "방향 3", "의 인자가 카탈로그와 어긋난다"),
]


def 음성_대조() -> list[str]:
    원본 = CATALOG.read_text(encoding="utf-8")
    문제 = []

    print("── ④ 카탈로그를 망가뜨리면 세션 시험이 실패하는가 ──")
    for 이름, 변형, 걸려야_하는_방향, 증거 in 변형들:
        바뀐 = 변형(원본)
        # **변형이 아무것도 안 바꾸면 그 자체가 실패다** (대조가 꺼지는 형태 ①)
        if 바뀐 == 원본:
            문제.append(f"변형 「{이름}」 이 파일을 안 바꿨다 — 변형이 낡았다")
            print(f"  FAIL  {이름} — 아무것도 안 바뀌었다")
            continue
        try:
            CATALOG.write_text(바뀐, encoding="utf-8")
            r = 돌린다(SESSION_TEST)
            화면 = r.stdout + r.stderr
            if r.returncode == 0:
                문제.append(f"변형 「{이름}」 에서 세션 시험이 통과했다 — {걸려야_하는_방향} 이 안 세고 있다")
                print(f"  FAIL  {이름} — 통과해 버렸다 ({걸려야_하는_방향})")
            elif 증거 not in 화면:
                # ★ **rc 는 판정이 아니다.** 빨간 것과 **그 방향이 빨간 것**은 다른 문장이고,
                # 컴파일 실패도 rc=101 을 낸다.
                문제.append(
                    f"변형 「{이름}」 이 rc={r.returncode} 로 실패했지만 "
                    f"{걸려야_하는_방향} 의 흔적(`{증거}`)이 없다 — 다른 것이 깨진 것이다"
                )
                print(f"  FAIL  {이름} — rc={r.returncode} 인데 {걸려야_하는_방향} 이 안 울었다")
            else:
                print(f"  ok    {이름} — {걸려야_하는_방향} 이 울었다 (rc={r.returncode})")
        finally:
            # **매번 복원한다.** 다음 변형이 앞 변형 위에 서면 무엇이 걸렸는지 모른다.
            CATALOG.write_text(원본, encoding="utf-8")

    if CATALOG.read_text(encoding="utf-8") != 원본:
        문제.append("카탈로그가 원본으로 안 돌아왔다")
    return 문제


# ─────────────────────────────────────────────────────────────────────────────
# ⑥ 어댑터를 뺀 빌드
# ─────────────────────────────────────────────────────────────────────────────

def 부재_빌드() -> list[str]:
    문제 = []
    print("\n── ⑥ 어댑터를 뺀 빌드 ──")

    # (a) 뺀 빌드에서 전건 통과
    r = 돌린다(["cargo", "test", "-p", "pal-cli", "--no-default-features"])
    통과 = sum(
        int(l.split()[3]) for l in r.stdout.splitlines() if l.startswith("test result:")
    )
    if r.returncode != 0:
        문제.append(f"어댑터 없는 빌드에서 시험이 실패했다 (rc={r.returncode})")
        print(f"  FAIL  뺀 빌드 시험 — rc={r.returncode}")
        for line in r.stdout.splitlines():
            if "FAILED" in line or line.startswith("error"):
                print(f"        {line}")
    # ★ **하한** — rc=0 하나만 보면 **시험이 하나도 안 돌아도 초록**이다(실측: 매칭
    # 안 되는 필터를 끼우니 *"0 통과"* 로 「어긋남 0 건」이 났다). *"전건 통과"* 의
    # 「전건」에 바닥이 없으면 그 문장은 아무것도 요구하지 않는다.
    elif 통과 < 뺀_빌드_최소_시험:
        문제.append(f"어댑터 없는 빌드가 {통과} 개만 돌았다 — {뺀_빌드_최소_시험} 미만이면 「전건」이 빈 말이다")
        print(f"  FAIL  뺀 빌드 시험 — {통과} 통과 (하한 {뺀_빌드_최소_시험})")
    else:
        print(f"  ok    뺀 빌드 시험 — {통과} 통과 (하한 {뺀_빌드_최소_시험})")

    # (b)(c) 두 빌드의 `--help` 를 각각 떠서 댄다.
    #
    # ⚠ **이것이 없으면 이 검사는 「빌드가 되는가」만 잰다.** 있는 쪽과 없는 쪽이
    # 실제로 갈리는 것을 봐야 «부재» 가 재어진다.
    # ⚠ **없는 쪽을 먼저 세우고 있는 쪽으로 끝낸다.** 순서가 반대면 이 스크립트가
    # `target/debug/pal` 을 **어댑터 없는 빌드로 남기고** 끝나고, 다음 사람이 그 바이너리를
    # 손으로 쓰면 `serve` 가 없는 것을 보고 **없는 것을 없다고 오해한다.**
    화면 = {}
    for 라벨, extra in (("없는 쪽", ["--no-default-features"]), ("있는 쪽", [])):
        b = 돌린다(["cargo", "build", "-p", "pal-cli", "--bin", "pal", *extra])
        if b.returncode != 0:
            문제.append(f"{라벨} 빌드가 실패했다")
            print(f"  FAIL  {라벨} 빌드 — rc={b.returncode}")
            continue
        h = 돌린다([str(ROOT / "target" / "debug" / "pal"), "--help"])
        화면[라벨] = h.stdout

    if "있는 쪽" in 화면 and "없는 쪽" in 화면:
        있다 = "serve" in 화면["있는 쪽"]
        없다 = "serve" not in 화면["없는 쪽"]
        if not 있다:
            문제.append("어댑터 있는 빌드의 `--help` 에 serve 가 없다")
        if not 없다:
            문제.append("어댑터 없는 빌드의 `--help` 에 serve 가 있다 — 뺀 것이 아니다")
        print(f"  {'ok  ' if 있다 else 'FAIL'}  있는 쪽 `--help` 에 serve 가 있다")
        print(f"  {'ok  ' if 없다 else 'FAIL'}  없는 쪽 `--help` 에 serve 가 없다")

    return 문제


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--only", choices=["negative", "absent"], help="하나만 돈다")
    a = ap.parse_args()

    # **먼저 초록인 것을 확인한다.** 원래 빨간 것을 망가뜨리고 「빨개졌다」고 적으면
    # 아무것도 안 잰 것이다.
    print("── 기준선 — 손대기 전에 초록인가 ──")
    r = 돌린다(SESSION_TEST)
    if r.returncode != 0:
        print(f"  FAIL  세션 시험이 손대기 전부터 빨갛다 (rc={r.returncode})")
        print(r.stdout[-3000:])
        return 1
    print("  ok    세션 시험이 초록이다")

    문제: list[str] = []
    if a.only in (None, "negative"):
        문제 += 음성_대조()
    if a.only in (None, "absent"):
        문제 += 부재_빌드()

    print()
    if 문제:
        print(f"어긋남 {len(문제)}건:")
        for m in 문제:
            print(f"  · {m}")
        return 1
    print("어긋남 0 건.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
