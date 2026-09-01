import fs from "node:fs";

function check(document) {
  const answer = document?.answer;
  if (!answer || answer.scope !== "full" || !Array.isArray(answer.invariants)) {
    throw new Error("doctor full answer shape is missing");
  }
  if (answer.invariants.length === 0) {
    throw new Error("doctor did not enumerate invariants");
  }
  const numbers = answer.invariants.map((invariant) => invariant?.number);
  const expectedNumbers = [1, 2, 3, 4, 5, 6, 7, 8];
  if (JSON.stringify(numbers) !== JSON.stringify(expectedNumbers)) {
    throw new Error(`doctor invariant set is incomplete or duplicated: ${JSON.stringify(numbers)}`);
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
    if (checked) {
      if (!Number.isInteger(checked.checked) || checked.checked <= 0 || checked.violations !== 0 || checked.skipped !== 0) {
        throw new Error(`doctor invariant ${invariant.number} is not clean`);
      }
    } else if (invariant?.outcome === "not_built") {
      if (!Array.isArray(invariant.absent) || invariant.absent.length === 0) {
        throw new Error(`doctor invariant ${invariant.number} lacks capability evidence`);
      }
    } else {
      throw new Error(`doctor invariant ${invariant.number} has an unknown outcome`);
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

const cleanInvariants = Array.from({ length: 8 }, (_, index) => ({
  number: index + 1,
  outcome: "not_built",
  absent: ["fixture"],
}));
for (const malformed of [
  { answer: { scope: "full", invariants: [], violations: [], residuals: [], coverage_gaps: [], unanchored_cutoff: [] } },
  { answer: { scope: "full", invariants: cleanInvariants.map((item, index) => index === 0 ? { number: 1, outcome: {} } : item), violations: [], residuals: [], coverage_gaps: [], unanchored_cutoff: [] } },
  { answer: { scope: "full", invariants: cleanInvariants.map((item, index) => index === 0 ? { number: 1, outcome: "not_built", absent: [] } : item), violations: [], residuals: [], coverage_gaps: [], unanchored_cutoff: [] } },
  { answer: { scope: "full", invariants: Array(8).fill({ number: 1, outcome: "not_built", absent: ["fixture"] }), violations: [], residuals: [], coverage_gaps: [], unanchored_cutoff: [] } },
]) {
  let malformedRejected = false;
  try {
    check(malformed);
  } catch {
    malformedRejected = true;
  }
  if (!malformedRejected) {
    throw new Error("doctor malformed negative control was accepted");
  }
}

check(JSON.parse(fs.readFileSync(0, "utf8")));
console.log("MERGE_BLOCKER_DOCTOR_OK");
