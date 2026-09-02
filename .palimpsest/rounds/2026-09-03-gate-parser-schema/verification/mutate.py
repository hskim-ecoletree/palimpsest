#!/usr/bin/env python3
"""#94 회차의 정상·음성 대조를 **다시 돌린다.**

    python3 .palimpsest/rounds/2026-09-03-gate-parser-schema/verification/mutate.py

저장소 뿌리는 `git rev-parse --show-toplevel` 로 찾는다 — 절대 경로를 안 박는다.
`record.py` 의 **선언 자리**를 하나씩 치환하거나 지우고, 매번 원본으로 되돌린다.
끝나면 `git status` 로 `record.py` 가 깨끗한지 확인한다.

★ **변이 연산자를 둘 사전 등록한다** — `값치환` 과 `키삭제`.
  사전부검 2 라운드가 잡았다: 같은 키가 값-치환에는 빨갛고 키-삭제에는 초록이다.
  하나만 쓰면 「소비되는가」의 답이 고른 연산자에 좌우된다.

★ **출력 프린터에 훅을 걸지 않는다.** 그렇게 재면 상수를 그대로 실은 키에 대해
  반대 답이 나온다(사전부검 1 라운드가 실측했다).
"""
import json, os, subprocess, sys

뿌리 = subprocess.run(["git", "rev-parse", "--show-toplevel"],
                      capture_output=True, text=True, check=True).stdout.strip()
RECORD = os.path.join(뿌리, ".claude/skills/round/bin/record.py")
IR = os.path.join(뿌리, ".palimpsest/rounds/2026-09-02-agent-laziness-merge-blockers/review/r1-raw.md")


def 돌린다(cmd):
    r = subprocess.run(cmd, cwd=뿌리, capture_output=True, text=True)
    return r.returncode, r.stdout + r.stderr


def 잰다():
    """오라클 셋 — 정책 검사 · 스키마를 실제로 읽는 시험 · 추출기."""
    rc, chk = 돌린다(["cargo", "xtask", "check"])
    진단 = next((l.strip()[:120] for l in chk.splitlines() if "FAIL" in l), "")
    # 진단문은 판정 목록 **뒤에** 별도 줄로 나온다 — 검사 이름 접두로 짚는다.
    상세 = next((l.strip()[:220] for l in chk.splitlines()
                 if l.startswith(("회차 레코드:", "원장 둘 대조:", "발견이 닫혔나:",
                                  "선언 목록이 닫혀 있나:"))), "")
    시험_rc, t = 돌린다(["cargo", "test", "-p", "pal-cli", "--test", "round_scripts_run"])
    ex_rc, ex = 돌린다([sys.executable,
                        os.path.join(뿌리, ".claude/skills/round/bin/extract.py"),
                        "독립리뷰", "1", IR])
    return {"검사_rc": rc, "검사_진단": 진단, "검사_상세": 상세,
            "시험_rc": 시험_rc, "추출_rc": ex_rc, "추출": ex}


변이 = [
    # (태그, 연산자, [(찾을 것, 바꿀 것)], 기대)
    ("B1-조건파서", "값치환",
     [('"형식": "- [x] <ID> <조건>  · <판정> ⟨전사 YYYY-MM-DD⟩"', '"형식": "PAL_거짓"'),
      ('"규칙": "펜스 안과 다른 절은 안 센다 · 들여쓰기를 받는다 · 태그는 첫 줄 끝"', '"규칙": "PAL_거짓"')],
     "불변 — `설명` 아래는 어느 검사도 값을 안 읽는다"),
    ("B1-게이트파서", "값치환",
     [('"표": "| 판정 | 조건 |  두 열 · **수 칸 없음** · 넷을 각각 한 행씩"', '"표": "PAL_거짓"'),
      ('"검산": "**검산** — 통과 N · 반증 N · 대조불가 N · 미측정 N = N"', '"검산": "PAL_거짓"')],
     "불변 — 같은 사유"),
    ("A2-설명밖-닫힘축뜻", "값치환",
     [('"뜻": "`닫은커밋` 은 **그 발견을 처분한 커밋**이지 고친 커밋이 아니다"', '"뜻": "PAL_거짓"')],
     "불변 — `설명` **밖**인데 읽는 자가 0 이다 (A2 반증의 근거)"),
    ("A2-설명밖-역사형식", "값치환",
     [('"legacy-2020": "명시 축 불릿과 기각 표를 함께 읽는다"', '"legacy-2020": "PAL_거짓"')],
     "불변 — 같은 사유. 이 값은 **이미 갈렸다**"),
    ("B2-열별칭", "값치환",
     [('"요약": ["발견", "안 잰 조건", "기각한 것", "시나리오", "제목", "항목"],', '"요약": ["PAL_없는열이름"],')],
     "검사는 불변인데 **추출 산출이 바뀐다** (B2 반증의 근거)"),
    ("B2-종류", "값치환", [('"종류": 종류,', '"종류": ["레코드"],')],
     "실패 — `회차 레코드` 가 고유 진단을 낸다"),
    ("B2-면제출처", "값치환", [('"면제출처": ["인터뷰", "실측"],', '"면제출처": ["실측"],')],
     "실패 — `합계검산.면제출처` 를 이름으로 짚는 진단"),
    ("연산자-키삭제", "키삭제",
     [('            "닫힘값": "닫힘",\n', ''), ('            "열림값": "열림",\n', ''),
      ('            "의도파일": "intent.md",\n', ''), ('            "경로없음": "(경로 없음)",\n', ''),
      ('            "사전처분없음": "해당없음",\n', ''), ('        "사전부검항": "### ",\n', ''),
      ('        "독립리뷰표머리": "| # |",\n', '')],
     "**초록** — 소비자가 `unwrap_or(\"<같은 문자열>\")` 이라 부재가 안 보인다"),
]


def main():
    원본 = open(RECORD, encoding="utf-8").read()
    기준 = 잰다()
    결과 = [{"태그": "기준", "연산자": "-", "검사_rc": 기준["검사_rc"], "시험_rc": 기준["시험_rc"]}]
    print(json.dumps(결과[0], ensure_ascii=False), flush=True)
    try:
        for 태그, 연산자, 쌍, 기대 in 변이:
            바뀐 = 원본
            for 찾을, 바꿀 in 쌍:
                if 찾을 not in 바뀐:
                    print(f"{태그}: 치환 자리를 못 찾았다 — 코드가 바뀌었다", flush=True)
                    바뀐 = None
                    break
                바뀐 = 바뀐.replace(찾을, 바꿀, 1)
            if 바뀐 is None:
                결과.append({"태그": 태그, "오류": "치환 자리 없음"})
                continue
            open(RECORD, "w", encoding="utf-8").write(바뀐)
            잰것 = 잰다()
            행 = {"태그": 태그, "연산자": 연산자, "기대": 기대,
                  "검사_rc": 잰것["검사_rc"], "검사_진단": 잰것["검사_진단"],
                  "검사_상세": 잰것["검사_상세"], "시험_rc": 잰것["시험_rc"],
                  "추출_바뀜": 잰것["추출"] != 기준["추출"]}
            결과.append(행)
            print(json.dumps(행, ensure_ascii=False), flush=True)
            open(RECORD, "w", encoding="utf-8").write(원본)
    finally:
        open(RECORD, "w", encoding="utf-8").write(원본)
        # ⚠ **`git status` 로 복원을 재지 않는다.** 회차 중에는 이 파일에 커밋 안 된
        #    정상 변경이 있을 수 있어 언제나 `M` 이 뜬다 — 그러면 이 대조가 항등식이다.
        #    **돌리기 전 바이트와 대조한다.**
        지금 = open(RECORD, encoding="utf-8").read()
        print(json.dumps({"복원됨": 지금 == 원본,
                          "바이트": [len(원본), len(지금)]}, ensure_ascii=False))
    # ⚠ **확장자가 `.txt` 인 까닭.** `docs/sunset.toml` 의 트리거가
    #    `.palimpsest/rounds/*/*.json` 에 걸려 있어 `.json` 으로 두면 `sunset 선언` 검사가
    #    즉시 빨개진다(실측 2026-09-03 · 앞서 `disposal-overrides.jsonl` 도 같은 이유로
    #    비켰다). `.jsonl` 도 안 된다 — 회차 레코드 검사가 그 확장자를 원장으로 읽는다.
    여기 = os.path.dirname(os.path.abspath(__file__))
    with open(os.path.join(여기, "mutate-result.txt"), "w", encoding="utf-8") as f:
        for 행 in 결과:
            f.write(json.dumps(행, ensure_ascii=False) + "\n")


main()
