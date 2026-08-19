# 상태 — 교대용 (§5 「교대」)

> **새 컨텍스트에 주는 것은 「잠긴 의도 전문 + 이 파일」이다.**
> 직전 산출물을 시드로 주지 마라 — 의도가 변질되는 기제가 거기 있다.

## 지금 어느 단계인가

**§5 루프의 종료**다. 독립 리뷰 **라운드 5 / 상한 5 — 소진**.
A~H 여덟 덩어리와 종료 산출(게이트·`report.md`·`state.md`)이 **전부 섰다.**

## 남은 것

- [x] 독립 리뷰 라운드 1~4 의 처분 — **금지역 열여섯이 났고 열여섯 다 닫혔다**
- [x] 게이트 `docs/gates/round-finding-records.md` — 절 넷 · 판정 **41+0+2+3 = 46** ✔
- [x] 종료 보고 `report.md` (§10 — **네 이름이 없다**)
- [x] 독립 리뷰 라운드 5 (상한) 이후의 처분 — 금지역 5·실패 1 을 닫았다. ⚠ **그 수정을 다시 잰 리뷰는 없다**
- [ ] push 한 번 · CI 초록 확인 (**`cancel-in-progress: true` 라 push 는 한 번**)

## 실패한 접근 — 다음 컨텍스트가 같은 벽에 다시 안 부딪히게

1. **음성 대조를 원 저장소의 `xtask` 바이너리로 돌리면 안 선다.**
   `repo_root()` 가 `env!("CARGO_MANIFEST_DIR")` 로 **컴파일 시점 경로**를 박아 cwd 와
   무관하게 자기가 빌드된 저장소를 잰다. **격리 사본에서 `cargo build -p xtask` 를 다시
   해야** 대조가 선다. 두 번 물렸고 두 번 다 「20/20 통과」로 보였다.
2. **`git clone --depth 1 file://` 는 커밋된 것만 가져온다.** 워킹트리 상태를 재려면
   `rsync -a --exclude target --exclude .git` 로 통째 복사한다.
3. **처분 예외표를 `.json` 으로 두면 sunset 트리거가 즉시 발화한다.** `.jsonl` 로 둔다.
4. **`라운드` 는 출처 안에서의 셈이다.** 커밋 태그 `[R<n>]` 과 대면 안 된다 —
   사전부검 R1~R3 · 독립 리뷰 R1~R5 · 커밋 R1~R7 이 서로 다른 셈이다.
5. **한글 출력 스크립트는 Windows 파이프에서 죽는다.** 출력은 `sys.stdout.reconfigure`,
   **입력은 `subprocess(encoding="utf-8", errors="replace")`** — 둘 다 못 박아야 한다.
   앞 판은 출력만 막았고 죽는 자리는 입력이었다. `PYTHONIOENCODING=cp1252` 로 재현된다.
6. **게이트를 문자열 치환으로 고치면 절이 통째로 사라진다.** 실제로 `## 효과` 와 판정
   표 두 행이 사라졌고 머리 줄은 「46 ✔」를 유지했다. **고친 뒤 `grep -n '^## '` 로
   세라.** [#76] 이 그것을 장치로 만드는 일을 진다.
7. **`build.py` 처럼 「돌려서 만드는」 스크립트는 rc 와 stderr 를 버리면 안 된다.**
   도구가 없으면 빈 코드블록을 내고 rc=0 으로 끝난다 — 조용한 거짓이다.

[#76]: https://github.com/hskim-ecoletree/palimpsest/issues/76

## 세는 자리

    레코드          python3 .claude/skills/round/bin/record.py count .palimpsest/rounds/2026-08-19-finding-records
    검사            cargo xtask check          (20 개 · 20 번째가 「회차 레코드」)
    계기판          python3 .claude/skills/round/bin/dashboard.py 47a6770 <의도파일>
    시험            cargo test -p pal-cli      (397)

★ **베끼지 말고 돌려라.** 이 회차가 「손으로 벤 거울」을 셋 더 찾았다.
