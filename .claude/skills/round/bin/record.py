#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""회차의 발견을 **구조화된 레코드**로 남긴다.

    사용: python3 record.py --schema
          python3 record.py add    <회차디렉터리> [--기준커밋 <sha>] < 한줄JSON들
          python3 record.py check  <파일…>
          python3 record.py count  <회차디렉터리>

★ **호출은 `python3 <경로>` 다.** 설치본은 파일 모드를 0644 로 놓아 직접 실행이 안 된다
(`crates/pal-cli/src/install.rs` 의 `guard::쓴다` 는 바이트만 쓴다). 그리고 Windows 에서는
모드로도 안 풀린다(옛 ADR-0023). **그래서 다섯 자리가 전부 이 형태로 적힌다** —
`SKILL.md` 두 줄 · 이 파일의 위 두 줄 · `NEXT-D-handoff.md`.

★ **enum 은 여기 한 자리에만 산다.** `xtask` 의 검사는 `--schema` 를 **불러서** 읽고
파이썬 소스를 정규식으로 안 긁는다. 두 곳에 적으면 갈리고, 갈린 것을 대는 장치가 없다
(옛 `layout.rs` 의 *"사본을 두면 갈리고 두 벌을 대는 검사가 없다"* 와 같은 자).

⚠ **`획득` 필드는 없다.** 에이전트가 내는 `획득`(조회/추정)과 `유효성`(참/추정/거짓)은
`추정` 을 같은 뜻으로 써서, 한 비트가 두 칸에 손으로 적히고 반드시 갈린다.
"""

import sys, json, os, subprocess

SCHEMA_VERSION = 1

# ── enum — 한 자리 ───────────────────────────────────────────────────────────
ENUM = {
    "출처":       ["독립리뷰", "사전부검", "인터뷰", "실측"],
    "모집단":     ["원의도", "저장소", "자기장치", "회차기록", "규약"],
    "유효성":     ["참", "추정", "거짓"],
    "해악도":     ["금지역", "실패", "거짓신호", "미관"],
    # 규약 §5 의 넷 + 범위밖 + 기각. **`막힘`·`승격` 은 처분이 아니다** — 아래 대응표.
    "처분":       ["정정", "확대", "축소", "전환", "범위밖", "기각"],
    "승격됨":     ["예", "아니오"],
    # ★ `완화` 가 여기 산다. 규약 §5: *"재는 의도의 양이 줄었으면 완화"*.
    #   위장한 정정을 가리는 축이고, `처분` 에 넣으면 `축소` 와 겹친다.
    "조건변경":   ["강화", "완화", "없음"],
    # 독립 리뷰 합격선 축의 **다섯째 어휘**. `처분` 과 섞지 않는다.
    "합격선판정": ["통과", "반증", "대조불가", "미측정", "해당없음"],
    # 규약 §2 사전부검의 넷. **§5 와 다른 축**이다.
    "사전처분":   ["계획수정", "탐지수단", "완수조건전환", "수용사유", "해당없음"],
}

REQUIRED = ["id", "라운드", "출처", "모집단", "유효성", "해악도", "처분", "경로", "요약"]
OPTIONAL_DEFAULT = {
    "승격됨": "아니오", "조건변경": "없음",
    "합격선판정": "해당없음", "사전처분": "해당없음",
    "조건": "없음", "줄": None, "기준커밋": None,
}
FIELDS = REQUIRED + list(OPTIONAL_DEFAULT)

# ── 규약 §5 ↔ `처분` 대응표 — 문서가 아니라 여기서 산다 ──────────────────────
대응표 = {
    "안개":   "레코드에 안 들어온다 — 규약이 「적지 않는다」고 명령한다. `기각` 과 다르다",
    "범위밖": "처분=범위밖",
    "정정":   "처분=정정 · 승격됨=아니오",
    "확대":   "처분=확대 · 승격됨=아니오",
    "축소":   "처분=축소 · 승격됨=예/아니오 (등록된 퇴로면 아니오)",
    "전환":   "처분=전환 · 승격됨=예 (전환은 항상 승격이다)",
    "완화":   "처분=축소 + 조건변경=완화",
    "승격":   "처분이 아니다 → 승격됨",
    "막힘":   "처분이 아니다 → 해악도(금지역·실패)에서 §11 ② 가 유도한다",
    "기각":   "처분=기각 — 발견자가 스스로 물린 것. #72 가 남기라고 요구한다",
}


def 스키마():
    return {
        "schema_version": SCHEMA_VERSION,
        "필드": FIELDS,
        "필수": REQUIRED,
        "기본값": OPTIONAL_DEFAULT,
        "enum": ENUM,
        "대응표": 대응표,
        "합계검산": "보존된 원 반환문의 `^### ` 항 + `## 내가 기각한 것` 아래 최상위 불릿",
    }


def 검증(줄번호, obj, out):
    """한 줄을 잰다. 문제를 `out` 에 담는다."""
    for k in REQUIRED:
        if k not in obj or obj[k] in (None, ""):
            out.append(f"{줄번호}행: 필수 필드 `{k}` 가 없다")
    for k, vals in ENUM.items():
        if k in obj and obj[k] is not None and obj[k] not in vals:
            out.append(f"{줄번호}행: `{k}` 값 `{obj[k]}` 는 enum 밖이다 ({' · '.join(vals)})")
    for k in obj:
        if k not in FIELDS:
            out.append(f"{줄번호}행: 모르는 필드 `{k}`")
    if obj.get("라운드") is not None and not isinstance(obj["라운드"], int):
        out.append(f"{줄번호}행: `라운드` 는 정수여야 한다")
    # 대응표가 금지하는 조합
    if obj.get("처분") == "전환" and obj.get("승격됨") == "아니오":
        out.append(f"{줄번호}행: `전환` 은 항상 승격이다 (대응표)")
    if obj.get("조건변경") == "완화" and obj.get("처분") != "축소":
        out.append(f"{줄번호}행: `완화` 는 `축소` 로 적는다 (대응표) — 위장한 정정을 가린다")


def 읽기(path):
    """머리 줄과 본문을 가른다."""
    머리, 행 = None, []
    with open(path, encoding="utf-8") as f:
        for i, line in enumerate(f, 1):
            line = line.strip()
            if not line:
                continue
            obj = json.loads(line)
            if i == 1 and "schema_version" in obj:
                머리 = obj
            else:
                행.append((i, obj))
    return 머리, 행


def cmd_check(paths):
    문제 = []
    총 = 0
    for p in paths:
        if not os.path.exists(p):
            문제.append(f"{p}: 없다")
            continue
        try:
            머리, 행 = 읽기(p)
        except json.JSONDecodeError as e:
            문제.append(f"{p}: JSON 이 아니다 — {e}")
            continue
        if 머리 is None:
            문제.append(f"{p}: 머리 줄에 `schema_version` 이 없다")
        elif 머리.get("schema_version") != SCHEMA_VERSION:
            문제.append(f"{p}: schema_version {머리.get('schema_version')} ≠ {SCHEMA_VERSION}")
        for 번호, obj in 행:
            검증(번호, obj, 문제)
        총 += len(행)
        print(f"{p}: {len(행)}행")
    if 문제:
        print("\n".join("  ✗ " + m for m in 문제), file=sys.stderr)
        return 1
    print(f"합계 {총}행 · 문제 없음")
    return 0


def cmd_add(회차, 기준커밋):
    path = os.path.join(회차, "findings.jsonl")
    새로 = not os.path.exists(path)
    if 기준커밋 is None:
        기준커밋 = subprocess.run(["git", "rev-parse", "--short", "HEAD"],
                                  capture_output=True, text=True).stdout.strip() or None
    줄들 = [l for l in sys.stdin.read().split("\n") if l.strip()]
    문제 = []
    파싱 = []
    for i, l in enumerate(줄들, 1):
        obj = json.loads(l)
        for k, v in OPTIONAL_DEFAULT.items():
            obj.setdefault(k, v)
        if obj.get("기준커밋") is None:
            obj["기준커밋"] = 기준커밋
        검증(i, obj, 문제)
        파싱.append(obj)
    if 문제:
        print("\n".join("  ✗ " + m for m in 문제), file=sys.stderr)
        return 1
    with open(path, "a", encoding="utf-8") as f:
        if 새로:
            f.write(json.dumps({"schema_version": SCHEMA_VERSION,
                                "회차": os.path.basename(os.path.normpath(회차))},
                               ensure_ascii=False) + "\n")
        for obj in 파싱:
            f.write(json.dumps({k: obj[k] for k in FIELDS if k in obj},
                               ensure_ascii=False) + "\n")
    print(f"{path}: {len(파싱)}행 추가")
    return 0


def cmd_count(회차):
    path = os.path.join(회차, "findings.jsonl")
    if not os.path.exists(path):
        print("레코드가 없다")
        return 1
    _, 행 = 읽기(path)
    print(f"레코드 {len(행)}행")
    for 축 in ("출처", "모집단", "유효성", "해악도", "처분"):
        표 = {}
        for _, o in 행:
            표[o.get(축)] = 표.get(o.get(축), 0) + 1
        print(f"  {축:6} " + " · ".join(f"{k} {v}" for k, v in sorted(표.items(), key=lambda x: -x[1])))
    return 0


def main(argv):
    if len(argv) < 2 or argv[1] in ("-h", "--help"):
        print(__doc__)
        return 0
    if argv[1] == "--schema":
        print(json.dumps(스키마(), ensure_ascii=False, indent=2))
        return 0
    if argv[1] == "check":
        return cmd_check(argv[2:])
    if argv[1] == "add":
        기준 = None
        args = argv[2:]
        if "--기준커밋" in args:
            i = args.index("--기준커밋")
            기준 = args[i + 1]
            args = args[:i] + args[i + 2:]
        return cmd_add(args[0], 기준)
    if argv[1] == "count":
        return cmd_count(argv[2])
    print(f"모르는 명령: {argv[1]}", file=sys.stderr)
    return 2


if __name__ == "__main__":
    sys.exit(main(sys.argv))
