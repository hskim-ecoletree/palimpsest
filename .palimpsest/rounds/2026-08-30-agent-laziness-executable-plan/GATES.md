# Gates: agent laziness executable implementation plan

Scope: produce a researched Markdown plan from which the next session can begin the first implementation round without rediscovering scope or code coordinates

- [x] G0: this ledger states outcomes that can fail without a machine-specific tool path
  CHECK: node -e "const fs=require('fs');const p='.palimpsest/rounds/2026-08-30-agent-laziness-executable-plan/GATES.md';const s=fs.readFileSync(p,'utf8');const gates=[...s.matchAll(/^- \[[ x]\] (G[0-9]+):/gm)].map(x=>x[1]);if(new Set(gates).size!==gates.length||gates.length!==5)throw Error('gate ids');if(s.includes('/Us'+'ers/')||s.includes('gate-'+'lint.mjs'))throw Error('non-portable dependency');console.log('PORTABLE_LEDGER_OK')"
  EXPECT: PORTABLE_LEDGER_OK
  EVIDENCE: exit=0 and EXPECT matched on 2026-09-02; command is repository-relative and self-checks the removed dependencies

- [x] G1: the plan artifact contains the implementation-entry contract and no placeholder markers
  CHECK: node -e "const fs=require('fs');const p='docs/agent-laziness-executable-implementation-plan.md';const s=fs.readFileSync(p,'utf8');for(const h of ['## 1. 조사 판정','## 2. 첫 구현 회차','## 3. 잠글 결정','## 4. 코드 좌표','## 5. RED와 검증','## 6. 이슈 처분','## 7. 다음 세션 착수 절차'])if(!s.includes(h))throw Error('missing '+h);for(const x of ['TODO','TBD','나중에 정한다'])if(s.includes(x))throw Error('placeholder '+x);console.log('PLAN_CONTRACT_OK')"
  EXPECT: PLAN_CONTRACT_OK
  EVIDENCE: historical exit=0 and EXPECT matched; repository-relative command retained

- [x] G2: repository documentation and project checks accept the plan change
  CHECK: cargo xtask check
  EXPECT: 검사 23/23 통과
  EVIDENCE: historical exit=0 and EXPECT matched; current repository check is rerun by the merge-blocker round

- [x] G3: every consequential external claim and implementation coordinate is backed by a primary source or current repository evidence
  EVIDENCE: upstream unlazy commit 473d4b8 and official Claude Code Hooks reference checked; repository coordinates checked at 2ea99a3; independent review round 2 found no remaining source/coordinate contradiction

- [x] G4: the next session can begin the first implementation round without another broad comparison pass
  EVIDENCE: plan sections 2-7 fix the first vertical scope, closed schema/digest/CLI contracts, file ownership, RED fixtures, issue disposition, limited drift check, and implementation order; independent review round 2 left no blocking plan ambiguity after R2-01/R2-02 disposition
