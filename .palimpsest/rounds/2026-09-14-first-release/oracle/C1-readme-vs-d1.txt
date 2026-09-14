# C1 오라클 — README 명령 줄 ↔ D1(릴리스 자산 v0.1.0) 이 친 명령 · 2026-09-14T15:29:49Z
# README: b0e8bb2:README.md · D1: .palimpsest/rounds/2026-09-14-first-release/effect/d1-release/commands.txt · 대조기: 세션 스크래치 c1-compare.py
# 음성 대조: README 에서 승인 줄을 뺀 사본에 대면 D1→README 가 「없다」 1 — 시운전으로 확인(아래 끝 절)
README 명령 줄 12 · D1 명령 줄 12

## README → D1 (README 에 적힌 걸음을 D1 이 밟았나)
  있다  pal --version   ← pal --version
  있다  pal install   ← pal install
  있다  pal doctor --install   ← pal doctor --install
  있다  pal narrative   ← pal narrative
  있다  pal touch <심볼 이름>   ← pal touch codexHostAdapter
  있다  pal narrative --approve <개체> --pick <짧은 해시>   ← pal narrative --approve decision/01M2G8J1K4YJ04EHNZY6Z0D5T1 --pick cbd4e6567928
  있다  pal touch <심볼 이름>   ← pal touch codexHostAdapter
  있다  pal intent export --out .palimpsest/intent/bindings.jsonl   ← pal intent export --out .palimpsest/intent/bindings.jsonl
  있다  git add .palimpsest/manifest.toml .palimpsest/intent/bindings.jsonl   ← git add .palimpsest/manifest.toml .palimpsest/intent/bindings.jsonl
  있다  git commit -m palimpsest 선언과 결박   ← git commit -m palimpsest 선언과 결박
  있다  pal intent import .palimpsest/intent/bindings.jsonl   ← pal intent import .palimpsest/intent/bindings.jsonl
  있다  pal touch <심볼 이름>   ← pal touch codexHostAdapter

## D1 → README (D1 이 밟은 걸음이 README 에 있나)
  있다  pal --version   ← pal --version
  있다  pal install   ← pal install
  있다  pal doctor --install   ← pal doctor --install
  있다  pal narrative   ← pal narrative
  있다  pal touch codexHostAdapter   ← pal touch <심볼 이름>
  있다  pal narrative --approve decision/01M2G8J1K4YJ04EHNZY6Z0D5T1 --pick cbd4e6567928   ← pal narrative --approve <개체> --pick <짧은 해시>
  있다  pal touch codexHostAdapter   ← pal touch <심볼 이름>
  있다  pal intent export --out .palimpsest/intent/bindings.jsonl   ← pal intent export --out .palimpsest/intent/bindings.jsonl
  있다  git add .palimpsest/manifest.toml .palimpsest/intent/bindings.jsonl   ← git add .palimpsest/manifest.toml .palimpsest/intent/bindings.jsonl
  있다  git commit -m palimpsest 선언과 결박   ← git commit -m palimpsest 선언과 결박
  있다  pal intent import .palimpsest/intent/bindings.jsonl   ← pal intent import .palimpsest/intent/bindings.jsonl
  있다  pal touch codexHostAdapter   ← pal touch <심볼 이름>

## 빠진 것 — README→D1 0 · D1→README 0
rc=0

## 음성 대조 — 승인 줄을 뺀 README 사본
  없다  pal narrative --approve decision/01M2G8J1K4YJ04EHNZY6Z0D5T1 --pick cbd4e6567928
## 빠진 것 — README→D1 0 · D1→README 1
rc=0
