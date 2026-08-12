#!/usr/bin/env python3
"""[f01] 의 대조 — 대장이 커밋 전 순간에도 자기 범위와 자기 낡음을 말하는가.

합격선 정본은 `corpus/criteria.toml` `[f01]`. 여덟을 잰다.

  ① 워킹트리 좌표가 실재한다 — `matches_worktree` 가 `NotBuilt` 에서 값으로
  ② 왕복 — 수정 → 되돌리기 → 바이트 단위로 원래대로
  ③ **음성 대조** — 넷은 반드시 요약을 바꾸고 하나는 반드시 안 바꾼다
  ④ 매니페스트 — 규칙 ID 없는 제외 0 · 합 불변 · 안 걸린 파일은 안 움직인다
  ⑤ 감지기 자신의 낡음
  ⑥ 언어 인식 — 실물에서 각 단계가 무엇을 켜는지
  ⑦ 골든 대장 스냅샷
  ⑧ 선형성 — 절대 시간이 아니라 **제곱이 아님**

# 사용자의 저장소를 변형하지 않는다

워킹트리를 재려면 파일을 고쳐야 한다. `git clone --local` 로 뜬 **사본**에서 하고
끝나면 지운다. 사본이라도 997 파일 전부이므로 합성 픽스처가 아니다.

# 변이 대상은 자라는 값이 아니라 고정 경로에 묶는다

파일 수·상태별 개수 같은 것에 묶으면 코퍼스가 바뀔 때 조용히 꺼진다(`7fe6b62`).
**대상 경로가 코퍼스에 없으면 `✓` 를 내는 대신 멈춘다**(f22-4 의 규칙).
"""

from __future__ import annotations

import argparse
import json
import shutil
import subprocess
import sys
import tempfile
import time
from pathlib import Path

# ── 변이 대상 — `boxwood/portal-backend@a29cad0bf6a8` 에 실재하는 경로들 ──────
#
# **없으면 멈춘다.** 코퍼스가 바뀌었는데 대조가 조용히 계속되면 그 대조는 장식이다.
AT = "a29cad0bf6a8"
MUTATE_CONTENT = "src/main/resources/banner.txt"   # 내용 1바이트
MUTATE_RENAME = "src/main/resources/logback-spring.xml"
MUTATE_DELETE = ".gitignore"
ADD_PATH = "palimpsest-probe.txt"                   # 없던 경로 — 추가 변이
# eol 이 걸린 파일. **깨끗한 워킹트리에서 이것이 dirty 로 뜨면 clean 필터가 없는 것이다.**
EOL_FILE = "gradlew.bat"
# 셔뱅으로만 알 수 있는 파일 — 인식 ② 단계가 켜는 것.
SHEBANG_FILE = "gradlew"

GOLDEN = "corpus/golden/portal-backend.ledger.json"


def run(cmd, cwd=None, check=True):
    p = subprocess.run(cmd, cwd=cwd, capture_output=True, text=True, check=False)
    if check and p.returncode != 0:
        raise SystemExit(f"실패: {' '.join(map(str, cmd))}\n{p.stderr.strip()}")
    return p.stdout


def ledger(pal, repo, cache, at=None):
    cmd = [str(pal), "ledger", str(repo), "--cache-dir", str(cache), "--json"]
    if at:
        cmd += ["--at", at]
    return json.loads(run(cmd))


def worktree_digest(pal, repo, cache):
    return ledger(pal, repo, cache)["worktree"]["tree_digest"]


def kind(state):
    return state if isinstance(state, str) else next(iter(state))


def counts(led):
    from collections import Counter
    return Counter(kind(e["state"]) for e in led["ledger"]["entries"])


def main() -> int:
    root = Path(__file__).resolve().parent.parent
    ap = argparse.ArgumentParser()
    ap.add_argument("--repo", type=Path, required=True, help="코퍼스 원본 (읽기만 한다)")
    ap.add_argument("--at", default=AT)
    ap.add_argument("--bin", type=Path, default=root / "target/release/pal")
    ap.add_argument("--bless", action="store_true", help="골든을 지금 산출로 덮어쓴다")
    ap.add_argument("--scale-repos", type=Path, default=None,
                    help="⑧ 선형성을 잴 저장소들의 부모 디렉터리")
    a = ap.parse_args()

    pal, origin = a.bin.resolve(), a.repo.expanduser().resolve()
    if not pal.exists():
        print(f"바이너리가 없다: {pal}", file=sys.stderr)
        return 1

    failures: list[str] = []
    tmp = Path(tempfile.mkdtemp(prefix="f01-verify-"))
    try:
        # ── 사본을 뜬다 — 원본은 건드리지 않는다 ────────────────────────────
        repo = tmp / "pb"
        run(["git", "clone", "--quiet", "--local", "--no-hardlinks", str(origin), str(repo)])
        run(["git", "checkout", "--quiet", a.at], cwd=repo)
        cache = tmp / "cache"

        # **대상이 없으면 멈춘다.** 조용한 `✓` 를 만들지 않는다.
        tracked = set(run(["git", "ls-tree", "-r", a.at, "--name-only"], cwd=repo).split("\n"))
        for name, p in [("내용", MUTATE_CONTENT), ("이름변경", MUTATE_RENAME),
                        ("삭제", MUTATE_DELETE), ("eol", EOL_FILE), ("셔뱅", SHEBANG_FILE)]:
            if p not in tracked:
                raise SystemExit(
                    f"변이 대상이 코퍼스에 없다 ({name}): {p}\n"
                    f"  코퍼스가 바뀌었다. **대조를 고치기 전에는 이 스크립트가 통과를 내지 않는다.**")
        if ADD_PATH in tracked:
            raise SystemExit(f"추가 변이 경로가 이미 있다: {ADD_PATH}")

        print("── ① 워킹트리 좌표가 실재한다 ──────────────────────────────────")
        led = ledger(pal, repo, cache)
        snap = led["ledger"]["snapshot"][0][1]
        w = led["worktree"]
        is_worktree = "worktree" in snap
        print(f"  TreeRef  {'worktree' if is_worktree else snap}  ·  요약 {w['tree_digest'][:16]}")
        if not is_worktree:
            failures.append("① `--at` 없이 돌렸는데 `TreeRef::Worktree` 가 아니다")

        # **깨끗한 워킹트리에서 dirty 가 0 인 것이 blob 이름 전수 일치의 증거다.**
        # `dirty_paths` 는 HEAD 트리의 `(경로, blob)` 과 우리가 계산한 것의 차집합이므로,
        # 0 이면 997 개의 blob 이름이 git 의 것과 하나도 다르지 않다는 뜻이다.
        # `gradlew.bat`(eol=crlf)이 여기 있으면 clean 필터가 없는 것이다.
        print(f"  dirty    {len(w['dirty_paths'])}  ·  인덱스 신뢰 {w['trusted_from_index']}"
              f" · 다시 잼 {w['rehashed']}")
        if w["dirty_paths"]:
            failures.append(f"① 갓 체크아웃한 워킹트리가 dirty 다: {w['dirty_paths'][:5]}")

        # 파일 목록이 git 과 같은가 — 개수가 아니라 집합이다.
        ours = {e["path"] for e in led["ledger"]["entries"]}
        theirs = {p for p in tracked if p}
        print(f"  경로     git {len(theirs)} · 대장 {len(ours)}"
              f" · git에만 {len(theirs - ours)} · 대장에만 {len(ours - theirs)}")
        if ours != theirs:
            failures.append(f"① 경로 집합이 다르다: git에만 {sorted(theirs - ours)[:5]} · "
                            f"대장에만 {sorted(ours - theirs)[:5]}")

        # `matches_worktree` 가 값이어야 한다 — 이 기능이 켜는 것의 증거.
        env = json.loads(run([str(pal), "doctor", "--repo", str(repo),
                              "--cache-dir", str(cache), "--index", str(tmp / "i.redb"),
                              "--intent", str(tmp / "t.redb"), "--json"]))
        mw = env["projection"]["matches_worktree"]
        rb = env["projection"]["rebuild"]
        print(f"  봉투     matches_worktree={mw} · rebuild={'not_built' if 'not_built' in rb else rb}")
        if "present" not in mw:
            failures.append(f"① `matches_worktree` 가 아직 값이 아니다: {mw}")
        elif mw["present"] is not True:
            failures.append("① 깨끗한 워킹트리인데 `matches_worktree` 가 참이 아니다")
        if "not_built" not in rb:
            failures.append(f"① `rebuild` 가 `NotBuilt` 가 아니다 — 관측 경로는 F05 다: {rb}")

        print()
        print("── ② 왕복 — 되돌리면 원래대로 ──────────────────────────────────")
        base_digest = worktree_digest(pal, repo, cache)
        base_ledger = json.dumps(ledger(pal, repo, cache)["ledger"], sort_keys=True)
        target = repo / MUTATE_CONTENT
        original = target.read_bytes()
        target.write_bytes(original + b"x")
        changed_digest = worktree_digest(pal, repo, cache)
        target.write_bytes(original)
        back_digest = worktree_digest(pal, repo, cache)
        back_ledger = json.dumps(ledger(pal, repo, cache)["ledger"], sort_keys=True)
        print(f"  요약     {base_digest[:12]} → {changed_digest[:12]} → {back_digest[:12]}")
        if back_digest != base_digest:
            failures.append("② 되돌렸는데 요약이 원래대로 오지 않았다")
        if back_ledger != base_ledger:
            failures.append("② 되돌렸는데 대장이 원래대로 오지 않았다")

        print()
        print("── ③ 음성 대조 — 넷은 바꾸고 하나는 안 바꾼다 ──────────────────")
        mutations = []

        # (1) 내용 1바이트
        target.write_bytes(original + b"x")
        mutations.append(("내용 1바이트", worktree_digest(pal, repo, cache), True))
        target.write_bytes(original)

        # (2) 추적 파일 추가 — `git add` 해야 추적이다
        (repo / ADD_PATH).write_text("probe\n")
        run(["git", "add", ADD_PATH], cwd=repo)
        mutations.append(("추적 파일 추가", worktree_digest(pal, repo, cache), True))
        run(["git", "rm", "--quiet", "--force", ADD_PATH], cwd=repo)

        # (3) 삭제
        run(["git", "rm", "--quiet", MUTATE_DELETE], cwd=repo)
        mutations.append(("추적 파일 삭제", worktree_digest(pal, repo, cache), True))
        run(["git", "checkout", "--quiet", a.at, "--", MUTATE_DELETE], cwd=repo)

        # (4) 이름 변경 — **blob 집합은 그대로인데 목록은 다르다**
        renamed = MUTATE_RENAME + ".renamed"
        run(["git", "mv", MUTATE_RENAME, renamed], cwd=repo)
        mutations.append(("이름 변경", worktree_digest(pal, repo, cache), True))
        run(["git", "mv", renamed, MUTATE_RENAME], cwd=repo)

        # (5) **반대 방향** — mtime 만. 성한 것을 잡지 않는지.
        for p in repo.rglob("*"):
            if p.is_file() and ".git" not in p.parts and ".palimpsest" not in p.parts:
                p.touch()
        mutations.append(("mtime 만 갱신", worktree_digest(pal, repo, cache), False))

        restored = worktree_digest(pal, repo, cache)
        for name, digest, must_change in mutations:
            changed = digest != base_digest
            ok = changed == must_change
            mark = "✓" if ok else "✗"
            want = "바뀌어야" if must_change else "안 바뀌어야"
            print(f"  {mark} {name:<16} {want} 한다 — {'바뀌었다' if changed else '그대로다'}")
            if not ok:
                failures.append(f"③ `{name}` 가 {want} 하는데 "
                                f"{'바뀌었다' if changed else '그대로다'}")
        if restored != base_digest:
            failures.append("③ 변이를 전부 되돌렸는데 요약이 기준으로 돌아오지 않았다")

        print()
        print("── ④ 매니페스트 — 범위는 선언에서 온다 ─────────────────────────")
        before = ledger(pal, repo, cache, a.at)
        c0 = counts(before)
        if before["ledger"]["scope"] != "inferred_from_path":
            failures.append(f"④ 매니페스트가 없는데 선언으로 적혔다: {before['ledger']['scope']}")
        (repo / ".palimpsest").mkdir(exist_ok=True)
        (repo / ".palimpsest/manifest.toml").write_text(
            '[[repo]]\nid = "portal-backend"\npath = "."\n'
            '[repo.exclude]\nrules = [{ id = "docs", glob = "docs/**" },\n'
            '          { id = "wrapper", glob = "gradle/wrapper/**" }]\n')
        after = ledger(pal, repo, cache, a.at)
        c1 = counts(after)
        excluded = [(e["path"], e["state"]["excluded"]["rule"])
                    for e in after["ledger"]["entries"] if kind(e["state"]) == "excluded"]
        no_rule = [p for p, r in excluded if not r]
        from collections import Counter
        by_rule = Counter(r for _, r in excluded)
        print(f"  범위     {after['ledger']['scope']}")
        print(f"  제외     {len(excluded)} 건 · 규칙별 {dict(by_rule)} · 규칙 ID 없는 제외 {len(no_rule)}")
        print(f"  합       {sum(c0.values())} → {sum(c1.values())}")
        if sum(c0.values()) != sum(c1.values()):
            failures.append("④ 제외를 켰더니 합이 달라졌다 — 제외는 칸을 옮기는 것이지 없애는 것이 아니다")
        if no_rule:
            failures.append(f"④ 규칙 ID 없이 제외된 파일: {no_rule[:5]}")
        if not excluded:
            failures.append("④ 제외 규칙을 넣었는데 걸린 파일이 0 이다 — 규칙이 안 걸린다")
        # **성한 것을 잡지 않는가** — 걸리지 않은 파일의 상태는 하나도 안 움직여야 한다.
        b = {e["path"]: json.dumps(e["state"], sort_keys=True) for e in before["ledger"]["entries"]}
        moved = [p for p, r in ((e["path"], json.dumps(e["state"], sort_keys=True))
                                for e in after["ledger"]["entries"])
                 if kind(json.loads(r)) != "excluded" and b.get(p) != r]
        print(f"  성한 것  제외되지 않았는데 상태가 바뀐 파일 {len(moved)}")
        if moved:
            failures.append(f"④ 제외와 무관한 파일의 상태가 바뀌었다: {moved[:5]}")
        # 저장소 식별자가 선언에서 온다 (R-08)
        repo_id = after["ledger"]["snapshot"][0][0]
        print(f"  식별자   {repo_id}  (사본 디렉터리 이름은 `{repo.name}`)")
        if repo_id != "portal-backend":
            failures.append(f"④ 저장소 식별자가 선언이 아니라 경로에서 왔다: {repo_id}")
        # 깨진 매니페스트는 **오류다** — 없는 것으로 삼키지 않는다
        (repo / ".palimpsest/manifest.toml").write_text("garbage [[[\n")
        broken = subprocess.run([str(pal), "ledger", str(repo), "--at", a.at,
                                 "--cache-dir", str(cache), "--json"],
                                capture_output=True, text=True, check=False)
        print(f"  깨진 것  종료 코드 {broken.returncode}")
        if broken.returncode == 0:
            failures.append("④ 깨진 매니페스트를 없는 것으로 삼켰다")
        shutil.rmtree(repo / ".palimpsest")

        print()
        print("── ⑤ 감지기 자신의 낡음 ────────────────────────────────────────")
        led = ledger(pal, repo, cache, a.at)
        det = led["ledger"]["detector"]
        head = run(["git", "rev-parse", "HEAD"], cwd=repo).strip()
        print(f"  감지기   추출기 {det['extractor']} · 문법 {det['grammar'][:7]} · HEAD {det['head_now'][:7]}")
        if det["head_now"] != head:
            failures.append(f"⑤ 감지기의 HEAD 가 실제와 다르다: {det['head_now'][:7]} ≠ {head[:7]}")
        for field in ("grammar", "extractor"):
            if not det.get(field):
                failures.append(f"⑤ 감지기에 `{field}` 가 비어 있다")
        # HEAD 가 움직이면 그 사실이 드러나야 한다 — **상수 시간 비교다**
        run(["git", "checkout", "--quiet", f"{a.at}~1"], cwd=repo)
        moved_led = ledger(pal, repo, cache, a.at)
        run(["git", "checkout", "--quiet", a.at], cwd=repo)
        moved_head = moved_led["ledger"]["detector"]["head_now"]
        print(f"  HEAD 이동 후 감지기가 가리키는 것 {moved_head[:7]} "
              f"(대장이 선 트리 {a.at[:7]})")
        if moved_head == det["head_now"]:
            failures.append("⑤ HEAD 를 옮겼는데 감지기가 그대로다 — 낡음이 안 보인다")

        print()
        print("── ⑥ 언어 인식 — 실물에서 무엇을 켜는가 ────────────────────────")
        led = ledger(pal, repo, cache, a.at)
        by_path = {e["path"]: e["state"] for e in led["ledger"]["entries"]}
        shebang = by_path.get(SHEBANG_FILE)
        lang = shebang.get("unsupported", {}).get("language") if isinstance(shebang, dict) else None
        unrec = [p for p, s in by_path.items() if kind(s) == "unrecognized"]
        print(f"  ② 셔뱅   `{SHEBANG_FILE}` → {lang or kind(shebang)}")
        print(f"  미인식   {len(unrec)} 건 — {sorted(unrec)}")
        if lang != "Shell":
            failures.append(f"⑥ 셔뱅으로 `{SHEBANG_FILE}` 을 잡지 못했다: {shebang}")
        if SHEBANG_FILE in unrec:
            failures.append(f"⑥ `{SHEBANG_FILE}` 이 아직 미인식이다")

        print()
        print("── ⑦ 골든 대장 스냅샷 ──────────────────────────────────────────")
        (repo / ".palimpsest").mkdir(exist_ok=True)
        (repo / ".palimpsest/manifest.toml").write_text(
            '[[repo]]\nid = "portal-backend"\npath = "."\n')
        golden_now = ledger(pal, repo, cache, a.at)["ledger"]
        golden_file = root / GOLDEN
        text = json.dumps(golden_now, sort_keys=True, indent=1, ensure_ascii=False) + "\n"
        if a.bless:
            golden_file.parent.mkdir(parents=True, exist_ok=True)
            golden_file.write_text(text)
            print(f"  골든을 덮어썼다: {GOLDEN}")
        elif not golden_file.exists():
            failures.append(f"⑦ 골든이 없다 — `--bless` 로 만들어라: {GOLDEN}")
            print(f"  골든이 없다: {GOLDEN}")
        else:
            same = golden_file.read_text() == text
            print(f"  대조     {'동일' if same else '**다르다**'}  ({GOLDEN})")
            if not same:
                old = json.loads(golden_file.read_text())
                diffs = diff_ledger(old, golden_now)
                for d in diffs[:10]:
                    print(f"    · {d}")
                failures.append(f"⑦ 골든과 다르다 ({len(diffs)} 곳). "
                                f"의도한 변화면 `--bless` 로 승인하라")
        shutil.rmtree(repo / ".palimpsest")

        print()
        print("── ⑧ 선형성 — 절대 시간이 아니라 형태를 본다 ───────────────────")
        points = measure_scale(pal, tmp, a.scale_repos, repo, a.at)
        if len(points) < 4:
            print(f"  규모 점이 {len(points)} 개다 — **대조 불가**. `--scale-repos` 를 주면 는다")
            failures.append(f"⑧ 규모 점 {len(points)}/4 — 대조 불가")
        else:
            for n, secs in points:
                print(f"  {n:>6} 파일  {secs:7.3f}s  ·  파일당 {secs / n * 1000:6.3f}ms")
            (n0, t0), (n1, t1) = points[0], points[-1]
            per0, per1 = t0 / n0, t1 / n1
            ratio, scale = per1 / per0, n1 / n0
            print(f"  규모 {scale:.1f}배에서 파일당 시간 {ratio:.2f}배"
                  f"  (제곱이면 {scale:.1f}배가 된다)")
            if ratio >= scale:
                failures.append(f"⑧ 파일당 시간이 규모에 비례해 늘었다 — 제곱 비용이다 "
                                f"({ratio:.2f}배 ≥ {scale:.1f}배)")

    finally:
        shutil.rmtree(tmp, ignore_errors=True)

    print()
    if failures:
        print("어긋난 것:")
        for f in failures:
            print(f"  · {f}")
        print("\n반증이다")
        return 1
    print("여덟 다 통과")
    return 0


def diff_ledger(old, new):
    """골든이 다를 때 **무엇이** 다른지 — 건수가 아니라 목록이다."""
    out = []
    for key in ("scope", "repos_declared", "detector", "languages", "snapshot"):
        if old.get(key) != new.get(key):
            out.append(f"{key}: {json.dumps(old.get(key), ensure_ascii=False)[:120]}"
                       f" → {json.dumps(new.get(key), ensure_ascii=False)[:120]}")
    o = {e["path"]: e["state"] for e in old.get("entries", [])}
    n = {e["path"]: e["state"] for e in new.get("entries", [])}
    for p in sorted(set(o) - set(n)):
        out.append(f"사라진 파일: {p}")
    for p in sorted(set(n) - set(o)):
        out.append(f"새 파일: {p}")
    for p in sorted(set(o) & set(n)):
        if o[p] != n[p]:
            out.append(f"{p}: {json.dumps(o[p], ensure_ascii=False)} → "
                       f"{json.dumps(n[p], ensure_ascii=False)}")
    return out


def measure_scale(pal, tmp, scale_root, fallback_repo, at):
    """규모별 **콜드 캐시** 대장 계산 시간.

    # 실물 저장소들을 나란히 재면 그것은 파일 수의 함수가 아니다

    boxwood 의 저장소 여섯(133~1529 파일)을 재면 997 파일짜리가 1529 파일짜리보다
    **다섯 배 느리다.** 언어 구성이 다르기 때문이다 — `portal-backend` 는 Kotlin 이
    671 개라 tree-sitter 가 실제로 돌고, `frontend` 는 추출기가 없는 언어뿐이라 분류에서
    끝난다. **그 점들로 그린 선은 규모의 함수가 아니라 언어 구성의 함수다.**

    그래서 **같은 저장소를 복제해 규모만 바꾼다.** 언어·크기 분포가 원본 그대로이고
    파일 수만 는다.

    # 합성이 왜곡하는 것 — 숨기지 않는다

    · 사본마다 내용에 고유 표식을 넣어 blob 을 다르게 만든다. 안 그러면 **캐시가
      과도하게 적중**해서 재는 것이 대장 계산이 아니라 캐시 읽기가 된다
    · 디렉터리 깊이가 하나 는다
    · 같은 파일의 사본이므로 **구문 다양성이 원본만큼은 아니다.** 파싱 비용의 분포는
      같지만 파서가 만나는 경우의 수는 그렇지 않다

    **이것은 "실물 10⁴ 저장소" 가 아니다.** 그 사실이 게이트에 적힌다.
    """
    copies = [1, 2, 4, 10]
    synth = tmp / "scale"
    synth.mkdir()
    run(["git", "init", "--quiet", str(synth)])
    run(["git", "config", "user.email", "s@e"], cwd=synth)
    run(["git", "config", "user.name", "s"], cwd=synth)

    points = []
    made = 0
    for k in copies:
        # 필요한 사본만 더 만든다 — 매번 처음부터 만들면 10 배를 네 번 만든다.
        while made < k:
            root = synth / f"copy{made}"
            # 체크아웃된 사본을 그대로 복사한다. **바이트로 다룬다** — png·jar 가
            # 섞여 있고 그것을 문자열로 읽으면 코퍼스가 텍스트뿐이라고 가정하는 것이다.
            shutil.copytree(fallback_repo, root,
                            ignore=shutil.ignore_patterns(".git", ".palimpsest"))
            for f in root.rglob("*"):
                if f.is_file() and not f.is_symlink():
                    # **고유 표식** — 없으면 사본들이 같은 blob 이 되어 캐시가 다 적중한다.
                    with open(f, "ab") as fh:
                        fh.write(f"\n// palimpsest-scale-{made}\n".encode())
            made += 1
        run(["git", "add", "-A"], cwd=synth)
        run(["git", "commit", "--quiet", "--allow-empty", "-m", f"scale {k}"], cwd=synth)
        n = len([x for x in run(["git", "ls-tree", "-r", "HEAD", "--name-only"],
                                cwd=synth).split("\n") if x])
        cache = tmp / f"scale-cache-{k}"
        start = time.monotonic()
        p = subprocess.run([str(pal), "ledger", str(synth), "--cache-dir", str(cache), "--json"],
                           capture_output=True, text=True, check=False)
        elapsed = time.monotonic() - start
        if p.returncode != 0:
            print(f"  규모 {n} 에서 실패: {p.stderr.strip()[:200]}")
            continue
        points.append((n, elapsed))

    # 실물 저장소들은 **부차 기록**이다 — 판정에 쓰지 않는다.
    if scale_root:
        print("  (부차) 실물 저장소들 — 언어 구성이 달라 판정에 쓰지 않는다:")
        for d in sorted(Path(scale_root).expanduser().iterdir()):
            if not (d / ".git").is_dir():
                continue
            n = len([x for x in run(["git", "ls-tree", "-r", "HEAD", "--name-only"],
                                    cwd=d, check=False).split("\n") if x])
            if not n:
                continue
            cache = tmp / f"real-cache-{d.name}"
            start = time.monotonic()
            p = subprocess.run([str(pal), "ledger", str(d), "--cache-dir", str(cache), "--json"],
                               capture_output=True, text=True, check=False)
            if p.returncode == 0:
                secs = time.monotonic() - start
                print(f"    {d.name:<40} {n:>6} 파일  {secs:7.3f}s")

    return sorted(points)


if __name__ == "__main__":
    raise SystemExit(main())
