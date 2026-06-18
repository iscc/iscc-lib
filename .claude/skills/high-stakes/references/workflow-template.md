# High-Stakes Adversarial-Verification Workflow Template

This is the reusable orchestration script the `high-stakes` skill runs for medium/large tasks. Copy
the script below and adapt it — do not re-derive the structure each invocation. The adaptation
points are all near the top:

- **Fill the inputs** (`TASK`, `ARTIFACT`, `PLAN`, `DIFF`, `ARTIFACT_KIND`, `TIER`) from the run.
    `PLAN` is your Phase-1 plan verbatim (Assumptions with their `[verified:…]`/`[UNVERIFIED]` tags,
    Questions, Conflicts) — the gauntlet checks the work against it. `DIFF` is the `git diff` of the
    edits (or before/after of each touched file); orthogonal damage and dead code are only visible
    here. `ARTIFACT_KIND` is `"code"` or `"prose"` and switches the correctness check.
- **Scale the panel** via the `.filter(...)` on `PANEL`: drop a mode that genuinely cannot apply to
    this task, but never prune `dead-code-sweeper` or `scope-diff-reviewer` when the artifact is
    code.
- The `export const meta` header and everything below the inputs are fixed machinery: the verdict
    schema, the guilty-until-clean prompts, the user-decision escalation, and the two-clean-rounds
    loop. Leave them intact (`meta` is required — every Workflow script must start with it).

## Design contract

- **Guilty until proven clean.** Every skeptic assumes its mode IS present and tries to prove it. It
    returns `clean: true` only when it has actively confirmed the artifact is free of its mode, with
    the evidence its schema demands — otherwise it defaults to a finding.
- **One skeptic, one mode.** Single-assignment agents catch what a generalist rationalizes away.
- **Evidence, not assertion.** Correctness must paste a real test command + result; simplicity must
    name the simpler construction it searched for; tradeoffs must enumerate the alternatives first;
    sycophancy runs the strip-test on the conversational prose. A bare `clean: true` is rejected.
- **User-decision findings escalate, never repair.** A genuine 50/50, an unresolved contradiction,
    or a flawed premise (modes #2, #3, #5) is put to the user with `AskUserQuestion` — the
    autonomous fixer may not pick a side, because that just launders a guess into settled prose.
- **Two-clean-rounds stop.** Loop until two consecutive fully clean rounds, or the budget is
    exhausted (then report unresolved findings — never fake convergence).
- **Determinism.** No wall-clock time, no randomness; fan-out is fixed by the triage decision, not
    computed from anything nondeterministic.

## The panel

| Skeptic              | Failure mode (code)      | What it must prove / produce                                                                                         |
| -------------------- | ------------------------ | -------------------------------------------------------------------------------------------------------------------- |
| assumption-hunter    | silent-assumptions       | Re-open each `[verified:…]` evidence; flag any decision absent from the plan or any `[UNVERIFIED]` built on silently |
| confusion-auditor    | unmanaged-confusion      | A genuine 50/50 was silently resolved instead of asked (user-decision)                                               |
| inconsistency-finder | hidden-inconsistencies   | Re-derive contradictions from the independent sources; each must be surfaced, not reconciled (user-decision)         |
| tradeoff-checker     | unsurfaced-tradeoffs     | First enumerate the alternatives, then show each was surfaced or rejected                                            |
| premise-challenger   | no-pushback              | A flawed premise was implemented faithfully instead of challenged (user-decision)                                    |
| sycophancy-detector  | sycophancy               | Strip-test the plan + report prose; flag if no judgment survives the cut                                             |
| correctness-tester   | subtle-conceptual-errors | Paste the real edge/boundary/concurrency test command, exit code, and cases                                          |
| simplicity-reviewer  | overcomplication         | Name the simplest construction found; show the artifact is at or near it                                             |
| dead-code-sweeper    | dead-code                | Diff for symbols/imports/branches/comments introduced or orphaned, never used                                        |
| scope-diff-reviewer  | orthogonal-damage        | Walk the diff hunk by hunk; flag any change the task did not require                                                 |

For prose/research artifacts, remap 7–10: correctness-tester → fact/logic checker (trace each claim
to a source), simplicity-reviewer → padding/hedging cutter, dead-code-sweeper → unsupported-claim
remover, scope-diff-reviewer → out-of-scope-edit reviewer.

## Script

```js
export const meta = {
    name: "high-stakes-gauntlet",
    description: "Adversarial per-mode verification gauntlet: one skeptic per failure mode, looping to two clean rounds, escalating user-decision findings instead of guessing.",
};

// === Adaptation points: fill from the run (all deterministic — no clocks/random) ===========
const TASK = `<the $ARGUMENTS task text; if it was empty, the concrete task you confirmed>`;
const ARTIFACT = `<path(s) or inline reference to the executor's draft>`;
const PLAN = `<your Phase-1 plan verbatim: Assumptions with [verified:<evidence>]/[UNVERIFIED] tags,
Questions, Conflicts, Tradeoffs, Success criteria>`;
const DIFF = `<git diff of the executor's edits, or before/after of each touched file>`;
const ARTIFACT_KIND = "code"; // "code" | "prose" — switches the correctness check
const TIER = "large"; // "medium" | "large" — from triage
const MAX_ROUNDS = TIER === "large" ? 5 : 3;

// Modes the user must decide — the fixer may never close these; they escalate instead.
const USER_DECISION_MODES = ["unmanaged-confusion", "hidden-inconsistencies", "no-pushback"];

// Per-mode prove strings. The correctness entry is chosen by artifact kind.
const correctnessProve = ARTIFACT_KIND === "code" ?
    "the logic is wrong on an edge/boundary/concurrency case — you may return clean:true ONLY by pasting the real test command, its exit code, and the specific edge cases it covered; if you did not run such a test, return clean:false" :
    "a claim is factually or logically wrong — trace each claim to its cited source or the real inputs; you may return clean:true ONLY by naming the claims you traced and where; do not certify by fluent reading";

// Select the panel for this tier. NEVER prune dead-code-sweeper or scope-diff-reviewer for code.
const PANEL = [{
    label: "assumption-hunter",
    mode: "silent-assumptions",
    prove: "an unstated detail was guessed rather than verified: re-open every [verified:<evidence>] item in the plan and confirm the evidence supports it; for each [UNVERIFIED] item confirm it was surfaced as a caveat/question, not silently built on; and flag any path/shape/version/default/edge-case the task did not dictate that is absent from the plan entirely"
}, {
    label: "confusion-auditor",
    mode: "unmanaged-confusion",
    prove: "a genuinely ambiguous choice that materially changes the result was silently resolved instead of asked about"
}, {
    label: "inconsistency-finder",
    mode: "hidden-inconsistencies",
    prove: "a contradiction was reconciled silently: enumerate every independent source the artifact must agree with (existing code, spec/IEP, configs, adjacent comments, the original task), diff each against the artifact and the others, flag any side picked without the contradiction being surfaced, and verify each conflict the plan claimed to find was actually surfaced"
}, {
    label: "tradeoff-checker",
    mode: "unsurfaced-tradeoffs",
    prove: "a viable alternative was buried: FIRST independently enumerate every other reasonable approach (data structure, sync vs async, library vs hand-rolled, algorithm), then for each show whether it was chosen-and-justified or surfaced; clean:true requires listing the alternatives you generated, or showing only one viable approach genuinely exists"
}, {
    label: "premise-challenger",
    mode: "no-pushback",
    prove: "a flawed premise or approach was implemented faithfully instead of challenged with a concrete technical reason"
}, {
    label: "sycophancy-detector",
    mode: "sycophancy",
    prove: "reflexive praise/agreement or instant capitulation substitutes for judgment — run the strip-test: mentally delete every praise/agreement phrase ('great', 'absolutely right', 'of course', 'good catch') and check whether concrete technical judgment remains; flag if not"
}, {
    label: "correctness-tester",
    mode: "subtle-conceptual-errors",
    prove: correctnessProve
}, {
    label: "simplicity-reviewer",
    mode: "overcomplication",
    prove: "a materially simpler construction fully solves the problem: NAME it concretely (the layers/abstractions/knobs/lines it removes and the resulting size); clean:true requires stating the simplest construction you searched for and showing the artifact is at/near it, with a present named caller for every layer, abstraction, and knob"
}, {
    label: "dead-code-sweeper",
    mode: "dead-code",
    prove: "scaffolding, unused symbols/imports, dead branches, or commented-out experiments remain — diff the artifact against its pre-change state and flag anything introduced or orphaned that is never reached or referenced"
}, {
    label: "scope-diff-reviewer",
    mode: "orthogonal-damage",
    prove: "an edit appears in the DIFF the task did not require — a deleted/rewritten/reformatted comment or any touched line outside scope; judge from the DIFF, not the final file: a deleted comment is invisible in the final artifact"
}, ].filter(s => true); // relevance filter — but NEVER prune dead-code-sweeper or scope-diff-reviewer for code

const verdictSchema = {
    type: "object",
    additionalProperties: false,
    required: ["mode", "clean", "needs_user_decision", "findings", "testEvidence", "simplerConstruction"],
    properties: {
        mode: {
            type: "string"
        },
        clean: {
            type: "boolean",
            description: "true ONLY if you actively confirmed the artifact is free of this mode with the evidence below"
        },
        needs_user_decision: {
            type: "boolean",
            description: "true if any finding is a genuine ambiguity, contradiction, or flawed premise only the user can resolve"
        },
        testEvidence: {
            type: "object",
            description: "correctness-tester only; otherwise empty strings/0/[]",
            additionalProperties: false,
            required: ["command", "exitCode", "passed", "failed", "edgeCasesCovered"],
            properties: {
                command: {
                    type: "string"
                },
                exitCode: {
                    type: "integer"
                },
                passed: {
                    type: "integer"
                },
                failed: {
                    type: "integer"
                },
                edgeCasesCovered: {
                    type: "array",
                    items: {
                        type: "string"
                    }
                },
            },
        },
        simplerConstruction: {
            type: "string",
            description: "simplicity-reviewer only: the concrete simpler design you found (what it removes + its size), or 'none found after active search'; empty for other skeptics"
        },
        findings: {
            type: "array",
            items: {
                type: "object",
                additionalProperties: false,
                required: ["location", "instance", "severity"],
                properties: {
                    location: {
                        type: "string",
                        description: "exact file:line, section, or claim"
                    },
                    instance: {
                        type: "string",
                        description: "the specific failure, concretely described"
                    },
                    severity: {
                        type: "string",
                        enum: ["blocker", "major", "minor"]
                    },
                },
            },
        },
    },
};

function skepticPrompt(s) {
    return `You are the ${s.label}. Assume the artifact is GUILTY of "${s.mode}" and prove it.
Specifically, hunt for: ${s.prove}.

Task the artifact was meant to fulfill:
${TASK}

Artifact under review:
${ARTIFACT}

Phase-1 plan (assumptions claimed with their verification status, questions, conflicts):
${PLAN}

Diff of the edits (review THIS for anything outside scope; the final file hides deletions):
${DIFF}

Rules of engagement:
- Inspect the REAL inputs (open the actual files/data/sources). Do not judge from assumptions.
- Your review target includes the agent's plan and delivery prose, not just the code — for the
  sycophancy check, run the strip-test on that conversational text.
- For correctness on code, verify behaviorally and paste the real test command, exit code, and edge
  cases into testEvidence; clean:true is invalid without it. For prose, trace claims to sources.
- For overcomplication, clean:true is invalid unless simplerConstruction names the minimal design you
  searched for and shows the artifact is at/near it; flag every layer/abstraction/knob with one caller.
- For unsurfaced-tradeoffs, list the alternatives you generated before judging coverage; an empty
  findings list is invalid without that enumeration.
- For orthogonal-damage, walk the DIFF hunk by hunk and state for each whether the task required it;
  if no diff was provided, that itself is a blocker finding.
- Set needs_user_decision: true if a finding is a genuine 50/50, an unresolved contradiction, or a
  flawed premise that only the user can settle — do not pick a side yourself.
- Default to a finding when you cannot positively confirm the artifact is free of your mode. Silence
  must be earned. An empty findings list means you actively looked and there is genuinely nothing.
Return only the schema object.`;
}

function correctnessUnverified(v) {
    // A correctness verdict claiming clean without a real edge-case run is itself a blocker.
    if (v.mode !== "subtle-conceptual-errors" || !v.clean || ARTIFACT_KIND !== "code") return false;
    const e = v.testEvidence;
    return e.command === "" || e.exitCode !== 0 || e.failed !== 0 || e.edgeCasesCovered.length === 0;
}

async function runGauntlet(round) {
    phase(`Adversarial round ${round}`);
    const verdicts = await parallel(
        PANEL.map(s => () =>
            agent(skepticPrompt(s), {
                label: s.label,
                phase: `round-${round}`,
                schema: verdictSchema
            })
        )
    );
    const open = verdicts.flatMap(v =>
        v.findings.map(f => ({
            ...f,
            mode: v.mode,
            needsUserDecision: v.needs_user_decision
        }))
    );
    // Promote an unverified correctness claim to a real blocker finding.
    for (const v of verdicts) {
        if (correctnessUnverified(v)) {
            open.push({
                mode: v.mode,
                location: ARTIFACT,
                severity: "blocker",
                needsUserDecision: false,
                instance: "unverified correctness: claimed clean without running an edge-case test",
            });
        }
    }
    log(`Round ${round}: ${open.length} finding(s) across ${verdicts.filter(v => !v.clean).length} mode(s).`);
    return open;
}

async function repair(findings, round) {
    phase(`Repair after round ${round}`);
    const fixPrompt = `Fix exactly these verified findings in the artifact and nothing else.
Confine edits strictly to what each finding requires — do not touch unrelated code or comments
(that would itself be orthogonal-damage), and delete any scaffolding you introduce.
If a finding is wrong, a pure style preference, or contradicts the success criteria, do NOT silently
comply — reject it with a one-line technical reason instead of editing (faithfully implementing an
incorrect finding is sycophancy toward the skeptic).
If a finding cannot be fixed without choosing between materially different interpretations the user
must decide, return it unchanged and flag it as requiring a user decision — never pick a side.

Artifact: ${ARTIFACT}
Findings:
${JSON.stringify(findings, null, 2)}

Return a short summary of what changed, by finding.`;
    return agent(fixPrompt, {
        label: "fixer",
        phase: `repair-${round}`
    });
}

// Main loop: two consecutive clean rounds, or escalate to the user, or exhaust the budget.
let cleanStreak = 0;
let lastFindings = [];
let escalated = [];
for (let round = 1; round <= MAX_ROUNDS && cleanStreak < 2; round++) {
    const findings = await runGauntlet(round);

    // User-decision findings cannot be silently repaired — halt and escalate.
    const blocking = findings.filter(
        f => f.needsUserDecision || USER_DECISION_MODES.includes(f.mode)
    );
    if (blocking.length) {
        escalated = blocking;
        log("STOP: findings require a user decision, not a silent repair — escalating, NOT guessing:");
        log(JSON.stringify(blocking, null, 2));
        break; // exit WITHOUT declaring convergence; ask the user with AskUserQuestion outside the script
    }

    if (findings.length === 0) {
        cleanStreak += 1;
        log(`Clean round (streak ${cleanStreak}/2).`);
        continue;
    }
    cleanStreak = 0;
    lastFindings = findings;
    await repair(findings, round);
}

if (escalated.length) {
    log("HALTED for user decision. Put each finding to the user with AskUserQuestion; do not converge " +
        "until they accept, revise, or overrule it. This is NOT a passed gauntlet.");
} else if (cleanStreak >= 2) {
    log("Converged: two consecutive clean rounds. Artifact passed the gauntlet.");
    log("Before reporting success, confirm no Phase-1 [UNVERIFIED] assumption is still unstated.");
} else {
    log("Did NOT converge within budget. Report these unresolved findings honestly — do not declare victory:");
    log(JSON.stringify(lastFindings, null, 2));
}
```

## Notes

- `parallel(...)` is used because the per-mode skeptics are independent and each wants the whole
    artifact, plan, and diff. Use `pipeline(...)` only if you stream many separate artifacts (e.g.
    file-by-file) through the same skeptic stages.
- For large code changes, give skeptics `isolation: "worktree"` if they run builds/tests so they do
    not step on each other.
- Pick `model`/`agentType` per skeptic if cost matters: cheap models for mechanical sweeps (dead
    code, scope diff), stronger models for conceptual correctness, tradeoff, and premise judgment.
- The two-clean-rounds rule is the margin against a skeptic that went easy once; do not lower it to
    one on a high-stakes task.
- The loop deliberately has three terminal states: converged, halted-for-user-decision, and
    budget-exhausted. Only the first is a pass — report the other two honestly.
