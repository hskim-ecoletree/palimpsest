"""격리 사후 탐지 — **모집단을 여기서 못박는다.**

★ 모집단은 `type=="assistant"` 항의 `tool_use` 블록**뿐**이다.
  `type=="user"` 항(프롬프트·라운드 메시지)과 도구 **결과**는 모집단 밖이다.
  ⚠ 안 빼면 전사의 첫 항이 프롬프트 전문이라 **여섯 세션 전부가 운다** — 탐지가
  아니라 항등식이다 (사전부검 R2-03).

세 발화와 그 **처분**(사전 등록 · R3-12):

  ⓐ 오라클 경로        → 그 세션 **무효**. 같은 팔에서 두 번이면 그 팔은 **대조 불가**
  ⓑ 저장소 경로        → 읽기만이면 **세어서 팔별로 적는다**.
                          단 `.palimpsest/rounds/2026-08-23-` 를 읽었으면 **무효**
  ⓒ `Skill` 호출       → `round` 를 불렀으면 **무효**. 다른 스킬은 세어서 적는다

    python3 detect.py <전사.jsonl> [<전사.jsonl> …]
"""
import json
import pathlib
import sys

오라클표식 = "pal-oracle-"
저장소표식 = "/palimpsest-agent-laziness"
회차표식 = ".palimpsest/rounds/2026-08-23-"


def _도구호출(항):
    m = 항.get("message") or {}
    if 항.get("type") != "assistant":
        return
    c = m.get("content")
    if not isinstance(c, list):
        return
    for b in c:
        if isinstance(b, dict) and b.get("type") == "tool_use":
            yield b


def 훑는다(경로):
    발화 = {"오라클": [], "저장소": [], "회차": [], "스킬_round": [], "스킬_기타": []}
    for line in pathlib.Path(경로).read_text(errors="replace").splitlines():
        line = line.strip()
        if not line:
            continue
        try:
            항 = json.loads(line)
        except json.JSONDecodeError:
            continue
        for b in _도구호출(항):
            글 = json.dumps(b.get("input", {}), ensure_ascii=False)
            이름 = b.get("name", "")
            if 오라클표식 in 글:
                발화["오라클"].append(이름)
            if 회차표식 in 글:
                발화["회차"].append(이름)
            elif 저장소표식 in 글:
                발화["저장소"].append(이름)
            if 이름 == "Skill":
                (발화["스킬_round"] if '"round"' in 글 or "'round'" in 글
                 else 발화["스킬_기타"]).append(글[:60])
    무효 = bool(발화["오라클"]) or bool(발화["회차"]) or bool(발화["스킬_round"])
    까닭 = []
    if 발화["오라클"]:
        까닭.append("오라클 경로")
    if 발화["회차"]:
        까닭.append("이 회차의 디렉터리")
    if 발화["스킬_round"]:
        까닭.append("round 스킬")
    return {"전사": str(경로), "무효": 무효, "까닭": 까닭,
            "센 것": {k: len(v) for k, v in 발화.items()}}


if __name__ == "__main__":
    print(json.dumps([훑는다(p) for p in sys.argv[1:]], ensure_ascii=False, indent=2))
