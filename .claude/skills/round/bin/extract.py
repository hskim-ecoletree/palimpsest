#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""보존된 원 반환문에서 발견을 **기계로** 뽑는다 — 손으로 옮기지 않는다.

    사용: python3 extract.py <출처> <라운드> <r<n>-raw.md> [기존-findings.jsonl]
          출처는 `사전부검` 또는 `독립리뷰`.

    넷째 인자를 주면 역사 ID·사람 판단을 보존한 재이주 행을 출력한다.

## 왜 이것이 있나 — #92

앞 판은 메인이 반환문을 **눈으로 읽고 손으로** 레코드를 적었다. 그 자리에서
**세 번 조용히 행이 떨어졌고** 두 번이 기각 행에 몰렸다 — 그러면 계기판 ⑧
(발견의 몇 %가 헛것인가)이 낮게 나온다. 그리고 합계 검산은 **행 수만** 세므로
**수만 맞고 내용이 다른 20**이 초록으로 지나갔다(실측).

**전사를 사람이 안 하면 그 병이 원리상 없어진다.**

## 무엇을 뽑고 무엇을 안 뽑나

| 뽑는다 (기계가 결정한다) | 안 뽑는다 (사람이 판단한다) |
|---|---|
| `id` · `라운드` · `출처` · **`조건`** | `처분` — §5 의 처분은 메인이 정한다 |
| `요약` · `경로` | `조건변경` · `승격됨` · `사전처분` |
| `모집단` · `유효성` · `해악도` | `상태` · `닫은커밋` — 처분 시점에 정해진다 |

★ **판단 칸은 예외표(`disposal-overrides.jsonl`)가 덮는다.** 그것이 이 구조의
설계다 — 기계가 뽑고 사람이 판단을 얹는다.

## 규칙은 여기 없다 — `record.py --schema` 의 `반환형식` 이 진다

**열은 이름으로 읽는다. 자리로 안 읽는다.** 절마다 열 배치가 다르지만 이름은
같다. 자리로 읽으면 절 목록을 여기 **손으로 베껴야** 하고, 그것이 회차마다
갈리는 「자라는 모집단의 거울」이다.

⚠ **옛 추출기를 안 물려받았다.** 그것은 `모집단`·`유효성` 을 `대상` 에서
**파생**하던 옛 형식용이라, 지금 반환문에 돌리면 **기각 행이 통째로 사라지고
유효성이 전부 「참」**이 된다(사전부검 R3 실측). 지금 형식은 그 칸들을 **직접
적으므로 읽으면 된다.**
"""
import sys, os, re, json, subprocess, difflib

여기 = os.path.dirname(os.path.abspath(__file__))


def 스키마():
    out = subprocess.run(
        [sys.executable, os.path.join(여기, "record.py"), "--schema"],
        # ⚠ **`encoding` 을 못 박는다.** `text=True` 만 주면 Windows 가 로케일
        #    인코딩(cp949·cp1252)으로 읽어 한글이 `UnicodeDecodeError` 를 낸다 —
        #    macOS 에서는 **원리상 안 보이는 자리**다(ADR-0023 · CI 실측 2026-08-24).
        capture_output=True, text=True, encoding="utf-8", check=True)
    return json.loads(out.stdout)["반환형식"]


def 정규화(s):
    """셀 하나를 값으로 — 굵게·백틱·링크 문법을 벗긴다."""
    s = re.sub(r"\[([^\]]*)\]\([^)]*\)", r"\1", s or "")
    return s.replace("**", "").replace("`", "").replace("\\|", "|").strip()


def 열이름(칭, 별칭):
    """헤더 셀 하나가 어느 칸인가. 못 찾으면 `None`."""
    c = 정규화(칭)
    for 칸, 이름들 in 별칭.items():
        if any(c == n or c.startswith(n) for n in 이름들):
            return 칸
    return None


def 펜스밖(text):
    """코드펜스 안을 빈 줄로 만든다.

    ★ 반환문이 **마크다운 형식을 예시로 인용**하면 그 안의 항 표시와 표 머리가
    발견으로 세어진다 — 항 수 계수기도 추출기도 펜스를 안 봤다. 그러면 **두 원장이
    같이 부풀고** 초록으로 만드는 자연스러운 길이 「없는 레코드를 지어내기」가 된다
    (독립 리뷰 R3 · 발견 6).
    """
    out, 안 = [], False
    for l in text.split("\n"):
        if l.lstrip().startswith("```"):
            안 = not 안
            out.append("")
            continue
        out.append("" if 안 else l)
    return "\n".join(out)


def 표들(text, 별칭):
    """표마다 (열이름목록, 데이터행들, 원문행들) 을 낸다.

    ⚠ **원문을 함께 낸다** — 좌표는 백틱 안에 살고, 정규화가 그것을 벗기면
    산문 첫 낱말을 좌표로 집게 된다(실측: 「커밋」·「네」·「잠근」).
    """
    lines = text.split("\n")
    i = 0
    절 = ""
    while i < len(lines):
        s = lines[i].lstrip()
        if s.startswith("#"):
            절 = 정규화(s.lstrip("# "))
        다음 = lines[i + 1].lstrip() if i + 1 < len(lines) else ""
        if s.startswith("|") and 다음.startswith(("|-", "| -", "|:")):
            헤더 = [열이름(c, 별칭) for c in s.strip().strip("|").split("|")]
            행들, 원문행들 = [], []
            j = i + 2
            while j < len(lines) and lines[j].lstrip().startswith("|"):
                셀 = lines[j].strip().strip("|").split("|")
                행들.append([정규화(c) for c in 셀])
                원문행들.append([c.strip() for c in 셀])
                j += 1
            yield 절, 헤더, 행들, 원문행들
            i = j
            continue
        i += 1


def 독립리뷰(text, 별칭, 없음, 프로필="current"):
    out = []
    for 절, 헤더, 행들, 원문행들 in 표들(text, 별칭):
        역사조건표 = 프로필 == "legacy-2022" and 헤더 and 헤더[0] is None and "조건" in 헤더
        if "요약" not in 헤더 and not 역사조건표:
            continue
        for 행, 원문 in zip(행들, 원문행들):
            d = {}
            if 헤더 and 헤더[0] is None and 행:
                d["_표시"] = 정규화(행[0])
            for k, v, raw in zip(헤더, 행, 원문):
                if k and k not in d:
                    d[k] = raw if k == "경로" else v
            요약 = d.get("요약", d.get("조건", "") if 역사조건표 else "")
            if 프로필 != "current" and 요약 in (없음, "—", "-"):
                요약 = f"{절} — 없음"
            d["요약"] = 요약
            if 역사조건표:
                d["요약"] = 요약
            # ★ **자리 채우기 행은 발견이 아니다** — #93. 「없음」만 적힌 행을 세면
            #   계기판 ⑦⑧ 이 조용히 커진다.
            if not 요약 or (프로필 == "current" and 요약 in (없음, "—", "-")):
                continue
            out.append(d)
    return out


def 사전부검(text, 별칭, 불릿, 없음, 프로필="current", 역사별칭=None):
    out = []
    기각불릿 = []
    현재 = None
    기각절 = False
    for line in text.split("\n"):
        if line.startswith("## "):
            기각절 = "내가 기각한 것" in line
            if 현재:
                out.append(현재)
                현재 = None
            continue
        if 기각절 and 프로필 != "current" and line.startswith("- "):
            원문 = line[2:].strip()
            d = {"요약": 원문}
            for 칸, 값들 in {
                "모집단": ("원의도", "저장소", "자기장치", "회차기록", "규약"),
                "유효성": ("참", "추정", "거짓"),
                "해악도": ("금지역", "실패", "거짓신호", "미관"),
            }.items():
                m = re.search(칸 + r"\s*:\s*(" + "|".join(값들) + r")", 정규화(원문))
                if m:
                    d[칸] = m.group(1)
            기각불릿.append(d)
            continue
        if line.startswith("### "):
            if 현재:
                out.append(현재)
            현재 = {"요약": 정규화(line[4:])}
            기각절 = False
            continue
        if 현재 is not None:
            m = re.match(r"\s*-\s*([^:]+):\s*(.*)$", line)
            if m:
                이름 = 정규화(m.group(1))
                후보들 = {k: [v] for k, v in 불릿.items()}
                if 프로필 != "current":
                    for 칸, 라벨들 in (역사별칭 or {}).items():
                        후보들.setdefault(칸, []).extend(라벨들)
                for 칸, 라벨들 in 후보들.items():
                    # ⚠ 좌표는 **원문**으로 담는다 — 백틱이 벗겨지면 못 찾는다.
                    값 = m.group(2).strip() if 칸 == "경로" else 정규화(m.group(2))
                    # ★ **접두로 맞춘다.** 실측: 같은 회차 안에서도 라벨이
                    #   「어디가 걸리나」와 「어디가 걸리나 (경로)」로 갈렸다.
                    if any(이름 == 라벨 or 이름.startswith(라벨) for 라벨 in 라벨들):
                        # 같은 항에 옛 보조 축(`획득`)과 정본 축(`유효성`)이 함께
                        # 있으면 정본 축이 이긴다. 보조 축을 먼저 본 순서 때문에
                        # 뒤의 명시 값을 삼키면 현재 형식의 산출까지 달라진다.
                        if 이름 == 불릿.get(칸) or 이름.startswith(불릿.get(칸, "\0")):
                            현재[칸] = 값
                        else:
                            현재.setdefault(칸, 값)
    if 현재:
        out.append(현재)
    # 기각 절의 표
    for _절, 헤더, 행들, 원문행들 in 표들(text, 별칭):
        if "요약" not in 헤더:
            continue
        for 행, 원문 in zip(행들, 원문행들):
            d = {}
            if 헤더 and 헤더[0] is None and 행:
                d["_표시"] = 정규화(행[0])
            for k, v, raw in zip(헤더, 행, 원문):
                if k and k not in d:
                    d[k] = raw if k == "경로" else v
            if d.get("요약") and d["요약"] not in (없음, "—", "-"):
                out.append(d)
    out.extend(기각불릿)
    if 프로필 == "legacy-2019":
        for d in out:
            if d in 기각불릿:
                d.setdefault("모집단", "자기장치")
                d.setdefault("유효성", "거짓")
                d.setdefault("해악도", "미관")
                continue
            대상 = 정규화(d.pop("모집단", ""))
            d["모집단"] = "자기장치" if "계획자신" in 대상 else "원의도"
            근거 = 정규화(d.pop("유효성", ""))
            d["유효성"] = "추정" if "추정" in 근거 else "참"
            if not d.get("해악도"):
                d["해악도"] = "거짓신호"
    elif 프로필 != "current":
        for d in out:
            합친값 = " · ".join(정규화(str(v)) for v in d.values())
            for 칸, 값들, 기본 in (
                ("모집단", ("원의도", "저장소", "자기장치", "회차기록", "규약"), "원의도"),
                ("유효성", ("참", "추정", "거짓"), "참"),
                ("해악도", ("금지역", "실패", "거짓신호", "미관"), "거짓신호"),
            ):
                직접 = 정규화(str(d.get(칸, "")))
                m = re.search(칸 + r"\s*:\s*(" + "|".join(값들) + r")", 합친값)
                d[칸] = (직접 if 직접 in 값들 else (m.group(1) if m else
                         next((v for v in 값들 if v in 직접), 기본)))
    return out


def _정체성없는비용(a, b):
    """원문 항 `a`와 역사 레코드 `b`의 결합 비용. 작을수록 가깝다."""
    sa, sb = 정규화(a.get("요약", "")), 정규화(b.get("요약", ""))
    비슷함 = difflib.SequenceMatcher(None, sa, sb).ratio()
    비용 = int((1.0 - 비슷함) * 10_000)
    표시 = a.get("_표시", "")
    if 표시:
        옛요약 = 정규화(str(b.get("요약", "")))
        표시일치 = (str(b.get("id", "")).endswith("-" + 표시) or
                옛요약 == 표시 or 옛요약.startswith(표시 + " ") or
                옛요약.startswith("#" + 표시 + " "))
        비용 += -20_000 if 표시일치 else 20_000
    for 칸, 가중 in (("경로", 1800), ("조건", 900), ("모집단", 300), ("해악도", 200)):
        av, bv = a.get(칸), b.get(칸)
        if av and bv and av != bv:
            비용 += 가중
    return 비용


def _헝가리(cost):
    """정사각 비용 행렬의 최소 배정 `(합, 행별 열)`을 낸다."""
    n = len(cost)
    u, v, p, way = [0] * (n + 1), [0] * (n + 1), [0] * (n + 1), [0] * (n + 1)
    for i in range(1, n + 1):
        p[0] = i
        minv, used = [10**18] * (n + 1), [False] * (n + 1)
        j0 = 0
        while True:
            used[j0] = True
            i0, delta, j1 = p[j0], 10**18, 0
            for j in range(1, n + 1):
                if not used[j]:
                    cur = cost[i0 - 1][j - 1] - u[i0] - v[j]
                    if cur < minv[j]:
                        minv[j], way[j] = cur, j0
                    if minv[j] < delta:
                        delta, j1 = minv[j], j
            for j in range(n + 1):
                if used[j]:
                    u[p[j]] += delta
                    v[j] -= delta
                else:
                    minv[j] -= delta
            j0 = j1
            if p[j0] == 0:
                break
        while True:
            j1 = way[j0]
            p[j0] = p[j1]
            j0 = j1
            if j0 == 0:
                break
    배정 = [0] * n
    for j in range(1, n + 1):
        배정[p[j] - 1] = j - 1
    return -v[0], 배정


def 역사병합(기존, 뽑은, 기계칸):
    """역사 ID·사람 판단은 보존하고 원문 순서와 기계 칸만 되돌린다.

    결합의 최소해가 둘이면 아무것도 쓰지 않고 실패한다.
    """
    if len(기존) != len(뽑은):
        raise ValueError(f"행 수가 다르다: 레코드 {len(기존)} · 원문 {len(뽑은)}")
    if not 기존:
        return []
    비용 = [[_정체성없는비용(e, r) for r in 기존] for e in 뽑은]
    최솟값, 배정 = _헝가리(비용)
    큰값 = 10**12
    for i, j in enumerate(배정):
        바꾼 = [row[:] for row in 비용]
        바꾼[i][j] = 큰값
        다음값, _ = _헝가리(바꾼)
        if 다음값 == 최솟값:
            raise ValueError(f"유일하게 결합할 수 없다: 원문 {i + 1}번째")
    out = []
    for i, j in enumerate(배정):
        row = dict(기존[j])
        for 칸 in 기계칸:
            row[칸] = 뽑은[i].get(칸, "")
        out.append(row)
    return out


def 자동프로필(text, 출처):
    """경로나 날짜가 아니라 원 반환문 자체의 옛 형식 표지로 가른다."""
    if 출처 == "사전부검":
        if re.search(r"(?m)^-\s*대상\s*:", text) and re.search(r"(?m)^-\s*근거\s*:", text):
            return "legacy-2019"
        if re.search(r"(?m)^-\s*(?:획득|아픔|얼마나 아픈가)\s*:", text):
            return "legacy-2022"
        return "current"
    if "| 조건 | 내 판정 | 게이트의 판정 |" in text:
        return "legacy-behavior"
    if ("| 조건 | 판정 | 잰 수 |" in text or
            "| 조건 | 판정 | 근거 |" in text or
            "| # | 조건 | 지금 | 근거 |" in text or
            "좌표(파일:줄)" in text):
        return "legacy-2022"
    return "current"


확장자 = r"\.(?:rs|py|md|toml|yml|yaml|jsonl|json|txt|log|sh|ts|tsv|lock)$"


def 좌표같은가(s):
    s = s.split(":")[0]
    return ("/" in s and " " not in s) or bool(re.search(확장자, s))


def 좌표만(s):
    """「어디가 걸리나」에서 **첫 좌표 하나**만.

    ★ **백틱 안을 먼저 본다.** 이 칸은 산문과 좌표가 섞여 있고, 앞 판은 첫
    낱말을 집어 「커밋」·「네」·「잠근」 같은 **산문 조각을 좌표로 적었다**(실측).
    좌표는 언제나 백틱 안에 있다.
    """
    if not s:
        return "(경로 없음)"
    for 안 in re.findall(r"`([^`]+)`", s):
        안 = 안.strip()
        if 좌표같은가(안):
            return 안.split(":")[0]
    for 낱말 in re.split(r"[\s·,]+", 정규화(s)):
        낱말 = 낱말.strip("(),;").rstrip(".")
        if 좌표같은가(낱말):
            return 낱말.split(":")[0]
    return "(경로 없음)"


def main(argv):
    if len(argv) not in (4, 5):
        print(__doc__.split("\n\n")[1].strip(), file=sys.stderr)
        return 2
    출처, 라운드, 경로 = argv[1], int(argv[2]), argv[3]
    s = 스키마()
    별칭, 불릿, 없음 = s["열별칭"], s["사전부검불릿"], s["없음표시"]
    text = 펜스밖(open(경로, encoding="utf-8").read())
    프로필 = 자동프로필(text, 출처)
    항 = (독립리뷰(text, 별칭, 없음, 프로필) if 출처 == "독립리뷰" else
          사전부검(text, 별칭, 불릿, 없음, 프로필, s.get("역사불릿별칭", {})))
    if 프로필 != "current":
        for d in 항:
            합친값 = " · ".join(정규화(str(v)) for v in d.values())
            for 칸, 값들, 기본 in (
                ("모집단", ("원의도", "저장소", "자기장치", "회차기록", "규약"), "저장소"),
                ("유효성", ("참", "추정", "거짓"), "참"),
                ("해악도", ("금지역", "실패", "거짓신호", "미관"), "미관"),
            ):
                직접 = 정규화(str(d.get(칸, "")))
                d[칸] = (직접 if 직접 in 값들 else
                         next((v for v in 값들 if re.search(r"(?:^|[ ·(])" + v, 직접)),
                              next((v for v in 값들 if re.search(칸 + r"\s*:\s*" + v, 합친값)), 기본)))
    접두 = {"독립리뷰": "IR", "사전부검": "PM"}[출처]
    산출 = []
    for i, d in enumerate(항, 1):
        산출.append({
            "id": f"{접두}{라운드}-{i:02d}",
            "라운드": 라운드,
            "출처": 출처,
            "모집단": d.get("모집단", ""),
            "유효성": d.get("유효성", ""),
            "해악도": d.get("해악도", ""),
            "경로": 좌표만(d.get("경로", "")),
            "요약": d.get("요약", ""),
            # ★ 리뷰어가 **어느 조건에 걸리나**를 직접 적는다 — 그것도 기계 칸이다.
            #   앞 판은 별칭만 있고 뽑지 않아 그 키의 소비자가 0 이었다(독립 리뷰 R2).
            **({"조건": d.get("조건", "없음") or "없음"}
               if "조건" in s["기계칸"].get(출처, []) else {}),
            **({"_표시": d["_표시"]} if d.get("_표시") else {}),
        })
    if len(argv) == 5:
        기존 = []
        with open(argv[4], encoding="utf-8") as f:
            for line in f:
                if not line.strip():
                    continue
                row = json.loads(line)
                if row.get("schema_version") is not None:
                    continue
                if row.get("출처") == 출처 and row.get("라운드") == 라운드:
                    기존.append(row)
        역사칸 = s.get("역사기계칸", {}).get(프로필, {}).get(출처, [])
        기계칸 = 역사칸 or s["기계칸"][출처]
        산출 = 역사병합(기존, 산출, 기계칸)
    for row in 산출:
        row.pop("_표시", None)
        if os.environ.get("PAL_ROUND_EXTRACT_REPORT_PROFILE"):
            row["_프로필"] = 프로필
        print(json.dumps(row, ensure_ascii=False))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
