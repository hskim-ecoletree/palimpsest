# Gates: agent laziness merge blockers

OWNS: crates/pal-cli/src/round/**, crates/pal-cli/tests/round_*.rs, crates/pal-git/**, scripts/**, .github/workflows/ci.yml, docs/agent-laziness-executable-implementation-plan.md, .palimpsest/rounds/2026-08-30-agent-laziness-executable-plan/GATES.md, .palimpsest/rounds/2026-09-02-agent-laziness-merge-blockers/**, docs/gates/**, docs/adr/**

Scope: close every issue 101 merge blocker and merge pull request 91 only after current independent and CI evidence succeeds

- [x] G1: focused round status, approve, and Stop tests cover every positive and negative blocker path
  CHECK: cargo test -p pal-cli --test round_status && cargo test -p pal-cli --test round_approve_verify && cargo test -p pal-cli --test round_stop && echo MERGE_BLOCKER_FOCUSED_OK
  EXPECT: MERGE_BLOCKER_FOCUSED_OK
  EVIDENCE: exit=0 and EXPECT matched; round_status 25, round_approve_verify 25, round_stop 22 passed

- [x] G2: repository policy checks accept code, round records, gates, and decisions
  CHECK: cargo xtask check && echo MERGE_BLOCKER_CHECK_OK
  EXPECT: MERGE_BLOCKER_CHECK_OK
  EVIDENCE: exit=0 and EXPECT matched; 23/23 policy checks passed after all premortem findings were closed

- [x] G3: the full repository test harness passes
  CHECK: cargo xtask test && echo MERGE_BLOCKER_TEST_OK
  EXPECT: MERGE_BLOCKER_TEST_OK
  EVIDENCE: exit=0 and EXPECT matched; workspace tests and doctests passed with only the registered release benchmark ignored

- [x] G4: full graph doctor classifies every invariant as clean checked or explicit capability absence and returns empty violation, residual, coverage-gap, and unanchored-cutoff sets
  CHECK: cargo run -q -p pal-cli -- doctor --full --json | node scripts/check-round-doctor.mjs
  EXPECT: MERGE_BLOCKER_DOCTOR_OK
  EVIDENCE: checker rejected its embedded violation fixture, then accepted the full doctor answer with enumerated invariants and four empty finding arrays

- [x] G5: issues 95 and 96 have explicit implemented, absorbed, or folded dispositions and issue 101 has no quietly removed item
  EVIDENCE: #95 CLOSED/COMPLETED, #96 CLOSED/NOT_PLANNED with the prior #95/#101 priority named; #101 remains OPEN and both native blocker nodes remain linked but CLOSED

- [ ] G6: independent artifact review leaves zero merge-blocking findings
  EVIDENCE: pending

- [x] G7: tracked release artifacts contain no self-referential claim that their own commit already passed CI, and the external terminal procedure names final-SHA CI, merge, and origin/main containment in order
  CHECK: node -e "const fs=require('fs');const p='.palimpsest/rounds/2026-09-02-agent-laziness-merge-blockers';const g=fs.readFileSync(p+'/GATES.md','utf8');const i=fs.readFileSync(p+'/intent.md','utf8');if(/EVIDENCE:.*(?:7.*success|CI.*success)/i.test(g))throw Error('self-referential CI evidence');for(const s of ['최종 PR SHA','CI 작업 7개 success','PR #91을 main에 병합'])if(!i.includes(s))throw Error('missing '+s);console.log('EXTERNAL_TERMINAL_PROCEDURE_OK')"
  EXPECT: EXTERNAL_TERMINAL_PROCEDURE_OK
  EVIDENCE: exit=0 and EXPECT matched; external observations remain outside tracked completion values
