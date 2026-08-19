#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""독립 리뷰의 **원 반환문**에서 발견을 기계로 뽑는다.

    사용: python3 extract-review.py <r1-raw.md> <라운드번호>

★ 사전부검용 `premortem/extract.py` 와 **갈래가 다르다** — 리뷰어는 표로 내고
사전부검자는 `### 항`으로 낸다. 그래서 항 수의 규칙도 다르다:

    사전부검  `^### ` 항 + `## 내가 기각한 것` 아래 최상위 불릿
    독립 리뷰  **`| # |` 헤더를 가진 표의 데이터 행**

⚠ `## 합격선 축` 표는 `| 조건 |` 로 시작하므로 **안 걸린다** — 그것은 발견이 아니라
등록된 조건에 대한 판정이고 **게이트가 진다.** (레코드에는 그 칸이 없다 — 한 번
`합격선판정` 으로 넣었다가 걷었다. 94 행 전부가 `해당없음` 이었고 두 산출이 모순이었다.)

⚠ **리뷰어의 반환 표에는 `처분` 칸이 없다**(독립 리뷰 W1 이 지적한 것). 그래서
여기서 기본값을 넣고 메인이 예외표로 덮는다 — 사전부검과 같은 자리다.
"""
import sys, re, json


def 표들(text):
    """`| # |` 헤더를 가진 표를 (절이름, 헤더, 행들) 로 뽑는다."""
    out = []
    절 = ""
    헤더 = None
    행 = []
    for line in text.split("\n"):
        if line.startswith("#"):
            if 헤더:
                out.append((절, 헤더, 행)); 헤더, 행 = None, []
            절 = line.lstrip("# ").strip()
            continue
        if line.startswith("| # |"):
            if 헤더:
                out.append((절, 헤더, 행))
            헤더 = [c.strip() for c in line.strip("|").split("|")]
            행 = []
            continue
        if 헤더 is not None:
            if re.match(r"^\|\s*-+", line):
                continue
            if line.startswith("|"):
                행.append([c.strip() for c in line.strip("|").split("|")])
                continue
            out.append((절, 헤더, 행)); 헤더, 행 = None, []
    if 헤더:
        out.append((절, 헤더, 행))
    return out


def 값(헤더, 행, 이름들, 기본=None):
    for 이름 in 이름들:
        if 이름 in 헤더:
            i = 헤더.index(이름)
            if i < len(행):
                v = re.sub(r"\*\*|`|★|⚠", "", 행[i]).strip()
                return v or 기본
    return 기본


ENUM = {
    "모집단": ["원의도", "저장소", "자기장치", "회차기록", "규약"],
    "유효성": ["참", "추정", "거짓"],
    "해악도": ["금지역", "실패", "거짓신호", "미관"],
}


def 맞추기(축, v, 기본):
    if not v:
        return 기본
    for 후보 in ENUM[축]:
        if 후보 in v:
            return 후보
    return 기본


def 첫경로(s):
    """좌표에서 첫 경로를 뽑는다.

    ⚠ **경로가 아닌 것 셋을 거른다** (실측 2026-08-19 · 새 검사가 잡았다):
      · 글롭·플레이스홀더 (`*` · `<slug>`)
      · **중괄호 확장** (`{premortem/extract.py,review/…}`) — 쉘 표기지 경로가 아니다
      · **삭제된 파일** — 뒤에 「삭제됨」이 붙는다. 그 좌표는 `기준커밋` 시점에만 실재한다
    """
    if "삭제됨" in s:
        return "(경로 없음)", None
    for m in re.finditer(r"`?([^\s`,·{}]+\.(?:jsonl|json|rs|py|md|toml|tsv|txt|yml|ts))(?::(\d+))?", s):
        경로 = m.group(1)
        #  같은 **축약 표기**도 경로가 아니다 (실측: 새 검사가 잡았다)
        if any(x in 경로 for x in ("*", "<", "{", "}", "...")):
            continue
        # 절대 경로는 기계 고유라 상대로 깎는다 — 다른 기계에서 안 맞는다.
        경로 = re.sub(r"^.*/palimpsest/", "", 경로)
        return 경로, (int(m.group(2)) if m.group(2) else None)
    return "(경로 없음)", None


def main(argv):
    raw, 라운드 = argv[1], int(argv[2])
    text = open(raw, encoding="utf-8").read()
    n = 0
    for 절, 헤더, 행들 in 표들(text):
        기각절 = "기각" in 절
        for 행 in 행들:
            n += 1
            요약 = 값(헤더, 행, ["발견", "기각한 것", "안 잰 조건"], "(요약 없음)") or "(요약 없음)"
            좌표 = 값(헤더, 행, ["좌표(파일:줄)", "좌표"], "") or ""
            경로, 줄 = 첫경로(좌표 or 요약)
            print(json.dumps({
                "id": f"IR{라운드}-{n:02d}",
                "라운드": 라운드,
                "출처": "독립리뷰",
                "모집단": 맞추기("모집단", 값(헤더, 행, ["모집단"]), "저장소"),
                "유효성": "거짓" if 기각절 else 맞추기("유효성", 값(헤더, 행, ["유효", "유효성"]), "참"),
                "해악도": 맞추기("해악도", 값(헤더, 행, ["해악도"]), "미관"),
                "처분": "기각" if 기각절 else "정정",
                "조건": (값(헤더, 행, ["조건"]) or "없음").split("·")[0].strip() or "없음",
                "경로": 경로,
                "줄": 줄,
                "요약": 요약[:160],
            }, ensure_ascii=False))
    print(f"# {raw}: 발견 {n}", file=sys.stderr)


if __name__ == "__main__":
    main(sys.argv)
