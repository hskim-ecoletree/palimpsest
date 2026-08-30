# Gates: agent laziness executable implementation plan

Scope: produce a researched Markdown plan from which the next session can begin the first implementation round without rediscovering scope or code coordinates

- [x] G0: this ledger states outcomes that can fail
  CHECK: node /Users/incognito/.agents/skills/unlazy/scripts/gate-lint.mjs .palimpsest/rounds/2026-08-30-agent-laziness-executable-plan/GATES.md
  EXPECT: LINT OK
  EVIDENCE: exit=0; shell=/bin/sh; cwd=/Users/incognito/dev/projects/palimpsest-agent-laziness; path=5e69bf1f0c1c/30 entries; EXPECT=matched; output-sha256=7bb93e9fa0fb85a4ba10ae0e77551ee949389493b725fd67a1ec0a58ae63dd9c; output-bytes=329

- [x] G1: the plan artifact contains the implementation-entry contract and no placeholder markers
  CHECK: node -e "const fs=require('fs');const p='docs/agent-laziness-executable-implementation-plan.md';const s=fs.readFileSync(p,'utf8');for(const h of ['## 1. 조사 판정','## 2. 첫 구현 회차','## 3. 잠글 결정','## 4. 코드 좌표','## 5. RED와 검증','## 6. 이슈 처분','## 7. 다음 세션 착수 절차'])if(!s.includes(h))throw Error('missing '+h);for(const x of ['TODO','TBD','나중에 정한다'])if(s.includes(x))throw Error('placeholder '+x);console.log('PLAN_CONTRACT_OK')"
  EXPECT: PLAN_CONTRACT_OK
  EVIDENCE: exit=0; shell=/bin/sh; cwd=/Users/incognito/dev/projects/palimpsest-agent-laziness; path=5e69bf1f0c1c/30 entries; EXPECT=matched; output-sha256=f1d53c7785df79bd2d4a41d2d20131881f130e244870113e1692ae1da1e637e6; output-bytes=17

- [x] G2: repository documentation and project checks accept the plan change
  CHECK: cargo xtask check
  EXPECT: 검사 23/23 통과
  EVIDENCE: exit=0; shell=/bin/sh; cwd=/Users/incognito/dev/projects/palimpsest-agent-laziness; path=5e69bf1f0c1c/30 entries; EXPECT=matched; output-sha256=37b9ca27546715295c0d2ec453dc0af9f22d0a646018ae5021158e8a35c56cde; output-bytes=4698

- [x] G3: every consequential external claim and implementation coordinate is backed by a primary source or current repository evidence
  EVIDENCE: upstream unlazy commit 473d4b8 and official Claude Code Hooks reference checked; repository coordinates checked at 2ea99a3; independent review round 2 found no remaining source/coordinate contradiction

- [x] G4: the next session can begin the first implementation round without another broad comparison pass
  EVIDENCE: plan sections 2-7 fix the first vertical scope, closed schema/digest/CLI contracts, file ownership, RED fixtures, issue disposition, limited drift check, and implementation order; independent review round 2 left no blocking plan ambiguity after R2-01/R2-02 disposition
