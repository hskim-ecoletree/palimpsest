"""눈가림 파생물 — **채점자에게 전사 전문을 안 준다.**

⚠ 전사의 첫 `user` 항이 **브리핑 전문**이라, 팔 이름만 가려도 채점자는 열자마자 안다
  (사전부검 R2-05). 그래서 이렇게 깎는다:

  - `type=="user"` 항 **전량**을 버린다 (프롬프트 · 라운드 메시지 · 도구 결과)
  - `assistant` 항에서 **텍스트와 `tool_use` 의 도구 이름·대상만** 남긴다
  - 처치 문구를 치환한다 (아래 `치환` — **사전 등록물이다**)

★ 눈가림의 **음성 대조는 이 목록과 독립된 자다** — 채점자에게 먼저
  *"이 여섯이 세 처치 중 어느 것인지 맞혀 보라"* 를 시켜 정답률을 본다 (R3-06).

    python3 derive.py <전사.jsonl> <나갈파일>
"""
import json
import pathlib
import re
import sys

치환 = ["공격선 전환", "끝난 느낌", "적대적으로", "전수 훑기", "전수로 훑",
        "check.py", "미완성 표면", "문턱을 넘어", "덧붙이는 규율"]
_re = re.compile("|".join(re.escape(t) for t in 치환))


def 깎는다(경로):
    밖 = []
    for line in pathlib.Path(경로).read_text(errors="replace").splitlines():
        line = line.strip()
        if not line:
            continue
        try:
            항 = json.loads(line)
        except json.JSONDecodeError:
            continue
        if 항.get("type") != "assistant":
            continue                       # ★ user 항 전량을 버린다
        c = (항.get("message") or {}).get("content")
        if not isinstance(c, list):
            continue
        for b in c:
            if not isinstance(b, dict):
                continue
            if b.get("type") == "text":
                밖.append("· " + _re.sub("〔가림〕", b.get("text", "").strip()))
            elif b.get("type") == "tool_use":
                i = b.get("input", {})
                대상 = i.get("file_path") or i.get("path") or i.get("command") or ""
                밖.append(f"[{b.get('name')}] " + _re.sub("〔가림〕", str(대상))[:200])
    return "\n".join(밖)


if __name__ == "__main__":
    out = pathlib.Path(sys.argv[2])
    out.write_text(깎는다(sys.argv[1]))
    print(f"{out} · {len(out.read_text().encode())} 바이트")
