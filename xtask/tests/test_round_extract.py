#!/usr/bin/env python3
"""#99 회귀: 현재 형식 불변·역사 필드 보존·모호한 이주 중단."""
from pathlib import Path
import hashlib
import importlib.util
import json
import re
import subprocess

ROOT = Path(__file__).resolve().parents[2]
EXTRACT = ROOT / ".claude/skills/round/bin/extract.py"
spec = importlib.util.spec_from_file_location("round_extract", EXTRACT)
extract = importlib.util.module_from_spec(spec)
spec.loader.exec_module(extract)

CURRENT = [
    "2026-08-23-check-verifies-work",
    "2026-08-30-round-verification-status",
    "2026-08-31-legacy-round-debt",
    "2026-08-31-round-approve-verify",
    "2026-08-31-round-stop-progress-guard",
]
PROTECTED = ["id", "처분", "조건변경", "승격됨", "사전처분", "상태", "닫은커밋", "줄", "기준커밋"]
HISTORICAL = ("2026-08-19-finding-records", "2026-08-20-rust-extractor",
              "2026-08-22-agent-laziness", "2026-08-23-agent-laziness-behavior")
PROTECTED_SHA256 = "9375b354e4b2bab3605856c22c1dd7de8bfc92192772a9b6bfc4d5b6dda01c56"


def run(source, n, raw):
    text = subprocess.check_output(["python3", str(EXTRACT), source, str(n), str(raw)], text=True)
    return text, [json.loads(line) for line in text.splitlines()]


def current_digest():
    chunks = []
    for slug in CURRENT:
        directory = ROOT / ".palimpsest/rounds" / slug
        for place, source in (("premortem", "사전부검"), ("review", "독립리뷰")):
            for raw in sorted((directory / place).glob("r*-raw.md")):
                n = int(re.match(r"r(\d+)-", raw.name).group(1))
                text, rows = run(source, n, raw)
                assert all("_표시" not in row for row in rows)
                chunks.append(f"{slug} {place} {raw.name}\n{text}")
    return hashlib.sha256("".join(chunks).encode()).hexdigest()


def protected_digest():
    rows = []
    for slug in HISTORICAL:
        path = ROOT / ".palimpsest/rounds" / slug / "findings.jsonl"
        for row in map(json.loads, path.read_text().splitlines()[1:]):
            rows.append([slug, *[row.get(key) for key in PROTECTED]])
    payload = json.dumps(sorted(rows), ensure_ascii=False, separators=(",", ":"))
    return hashlib.sha256(payload.encode()).hexdigest()


def main():
    assert current_digest() == "1472ccb3084474e989fc8c6781709a5a4f1d3f494046d4641279083f022daddf"
    assert protected_digest() == PROTECTED_SHA256
    assert extract.자동프로필("- 대상: 계획자신\n- 근거: 조회", "사전부검") == "legacy-2019"
    assert extract.자동프로필("- 획득: 조회", "사전부검") == "legacy-2022"
    assert extract.자동프로필("| 조건 | 내 판정 | 게이트의 판정 |", "독립리뷰") == "legacy-behavior"
    try:
        extract.역사병합([{"id": "A", "요약": "같음"}, {"id": "B", "요약": "같음"}],
                         [{"요약": "같음"}, {"요약": "같음"}], ["요약"])
    except ValueError as error:
        assert "유일하게 결합할 수 없다" in str(error)
    else:
        raise AssertionError("모호한 역사 결합이 통과했다")
    print("round extract verification passed")


if __name__ == "__main__":
    main()
