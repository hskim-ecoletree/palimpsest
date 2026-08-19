#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""보존된 원 반환문에서 발견을 **기계로** 뽑는다 — 손으로 옮기지 않는다.

    사용: python3 extract.py <r1-raw.md> <라운드번호> …

★ 이 회차가 계수를 세 번 시도해 **둘이 어긋났다**(메인 17↔18 · 사전부검 11↔12).
  그래서 전사를 사람이 안 한다. 이 스크립트가 곧 #71 이 요구한 **「기계 원천」**이다.

매핑 규칙 — **여기 한 자리에 적고 갈리지 않게 한다:**
  대상 계획대상 → 모집단 원의도   (계획이 다루는 대상 = 원 의도가 만들 것)
  대상 계획자신 → 모집단 자기장치 (이 회차가 만드는 장치)
  시나리오 항          → 유효성 참 (실측 근거가 있다). 단 `근거: 추정` 이면 추정
  `## 내가 기각한 것`  → 유효성 거짓 · 처분 기각 (발견자가 스스로 물렸다)
⚠ 이 매핑은 **추측이다** — 사전부검의 반환 형식에 `모집단`·`유효성` 이 없기 때문이다.
  계획 F 가 그것을 고친다. 고쳐지면 이 매핑은 사라진다.
"""
import sys, re, json, os

해악도값 = ["금지역", "실패", "거짓신호", "미관"]


def 항들(text):
    """`### ` 시나리오와 `## 내가 기각한 것` 아래 최상위 불릿을 뽑는다 — 합계 검산의 규칙."""
    시나리오, 기각 = [], []
    현재, 모드 = None, None
    for line in text.split("\n"):
        if line.startswith("### "):
            if 현재:
                시나리오.append(현재)
            현재 = {"제목": line[4:].strip(), "본문": []}
            모드 = "s"
            continue
        if line.startswith("## "):
            if 현재:
                시나리오.append(현재)
                현재 = None
            모드 = "r" if "내가 기각한 것" in line else None
            continue
        if 모드 == "s" and 현재 is not None:
            현재["본문"].append(line)
        elif 모드 == "r" and line.startswith("- "):
            기각.append(line[2:].strip())
    if 현재:
        시나리오.append(현재)
    return 시나리오, 기각


def 첫경로(본문):
    m = re.search(r"`([^`\s]+\.(?:rs|py|md|toml|json|jsonl|tsv|txt|yml|ts))(?::(\d+))?", 본문)
    if m:
        return m.group(1), (int(m.group(2)) if m.group(2) else None)
    return "(경로 없음)", None


def 한줄요약(s):
    s = re.sub(r"\*\*|`|★|⚠", "", s).strip()
    return s[:160]


def main(argv):
    raw, 라운드 = argv[1], int(argv[2])
    text = open(raw, encoding="utf-8").read()
    시나리오, 기각 = 항들(text)
    out = []
    for i, s in enumerate(시나리오, 1):
        본문 = "\n".join(s["본문"])
        해악 = next((h for h in 해악도값 if re.search(r"해악도.*?" + h, 본문)), "거짓신호")
        대상 = "자기장치" if re.search(r"대상:\s*\*?\*?계획자신", 본문) else "원의도"
        유효 = "추정" if re.search(r"근거:\s*\*?\*?추정", 본문) else "참"
        경로, 줄 = 첫경로(본문)
        out.append({
            "id": f"PM{라운드}-S{i:02d}", "라운드": 라운드, "출처": "사전부검",
            "모집단": 대상, "유효성": 유효, "해악도": 해악,
            "처분": "정정", "사전처분": "계획수정",
            "경로": 경로, "줄": 줄, "요약": 한줄요약(s["제목"]),
        })
    for i, r in enumerate(기각, 1):
        경로, 줄 = 첫경로(r)
        out.append({
            "id": f"PM{라운드}-X{i:02d}", "라운드": 라운드, "출처": "사전부검",
            "모집단": "자기장치", "유효성": "거짓", "해악도": "미관",
            "처분": "기각", "사전처분": "해당없음",
            "경로": 경로, "줄": 줄, "요약": 한줄요약(r),
        })
    for o in out:
        print(json.dumps(o, ensure_ascii=False))
    print(f"# {os.path.basename(raw)}: 시나리오 {len(시나리오)} · 기각 {len(기각)} · 합 {len(out)}",
          file=sys.stderr)


if __name__ == "__main__":
    main(sys.argv)
