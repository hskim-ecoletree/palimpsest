import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

// ★★ **기준선 — 한 번 선 축은 다시 안 앉는다** (2026-09-10 · `BL3-02`).
//
// 아래 `check` 는 축이 `not_built` 여도 `absent` 만 비지 않으면 통과시킨다. 그래서
// **`checked{n}` → `not_built` 전이**가 그 그물을 지난다 — 모집단이 사라지면 위반도
// 0 이 되고 그 0 이 「깨끗하다」로 읽힌다. 금지역 「측정이 죽은 가지」다.
//
// ⚠ **기준선에 수를 안 적는다.** 모집단은 저장소가 자라면 늘고, 적으면 즉시 낡는다.
// 적는 것은 **처지**뿐이다.
const BASELINE = JSON.parse(
  fs.readFileSync(
    path.join(path.dirname(fileURLToPath(import.meta.url)), "doctor-axes.baseline.json"),
    "utf8",
  ),
);

function checkBaseline(document, baseline) {
  const 축 = baseline?.["축"];
  if (!축 || Object.keys(축).length !== 8) {
    throw new Error("doctor axis baseline is missing or incomplete");
  }
  for (const invariant of document.answer.invariants) {
    const want = 축[String(invariant.number)];
    if (!want) {
      throw new Error(`doctor axis ${invariant.number} is not in the baseline`);
    }
    const got = invariant?.outcome === "not_built" ? "not_built" : "checked";
    if (want === "checked" && got !== "checked") {
      throw new Error(
        `doctor axis ${invariant.number} regressed: baseline says checked, got ${got} — ` +
          "a population that disappears makes violations 0, and that 0 reads as clean",
      );
    }
  }
}

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

// ── 기준선의 음성 대조 — **떨어뜨리면 잡혀야 한다** ─────────────────────────
//
// 안 걸면 이 기준선은 「있는데 아무것도 안 재는」 파일이 되고, 그것이 바로 이 장치가
// 고치려는 형태다.
const 떨어진 = {
  answer: {
    scope: "full",
    invariants: Array.from({ length: 8 }, (_, i) => ({
      number: i + 1,
      outcome: "not_built",
      absent: ["fixture"],
    })),
  },
};
let 기준선_거절 = false;
try {
  checkBaseline(떨어진, BASELINE);
} catch {
  기준선_거절 = true;
}
if (!기준선_거절) {
  throw new Error("doctor axis baseline negative control was accepted");
}
// 그리고 **안 떨어뜨리면 통과해야 한다** — 아니면 이 자는 언제나 발화하는 것이다.
checkBaseline(
  {
    answer: {
      scope: "full",
      invariants: Array.from({ length: 8 }, (_, i) => ({
        number: i + 1,
        outcome: i < 3 ? { checked: { checked: 1, skipped: 0, violations: 0 } } : "not_built",
        absent: ["fixture"],
      })),
    },
  },
  BASELINE,
);

const 실물 = JSON.parse(fs.readFileSync(0, "utf8"));
check(실물);
checkBaseline(실물, BASELINE);
console.log("MERGE_BLOCKER_DOCTOR_OK");
