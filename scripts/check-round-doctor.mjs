import fs from "node:fs";

function check(document) {
  const answer = document?.answer;
  if (!answer || answer.scope !== "full" || !Array.isArray(answer.invariants)) {
    throw new Error("doctor full answer shape is missing");
  }
  if (answer.invariants.length === 0) {
    throw new Error("doctor did not enumerate invariants");
  }
  for (const field of ["violations", "residuals", "coverage_gaps", "unanchored_cutoff"]) {
    if (!Array.isArray(answer[field])) {
      throw new Error(`doctor ${field} is not an array`);
    }
    if (answer[field].length !== 0) {
      throw new Error(`doctor ${field} is not empty`);
    }
  }
  for (const invariant of answer.invariants) {
    const checked = invariant?.outcome?.checked;
    if (checked && (checked.violations !== 0 || checked.skipped !== 0)) {
      throw new Error(`doctor invariant ${invariant.number} is not clean`);
    }
  }
}

const broken = {
  answer: {
    scope: "full",
    invariants: [{ number: 1, outcome: { checked: { violations: 1, skipped: 0 } } }],
    violations: [{ invariant: 1 }],
    residuals: [],
    coverage_gaps: [],
    unanchored_cutoff: [],
  },
};
let rejected = false;
try {
  check(broken);
} catch {
  rejected = true;
}
if (!rejected) {
  throw new Error("doctor negative control was accepted");
}

check(JSON.parse(fs.readFileSync(0, "utf8")));
console.log("MERGE_BLOCKER_DOCTOR_OK");
