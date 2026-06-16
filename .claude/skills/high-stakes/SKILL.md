---
name: high-stakes
description: >-
  Produce a reliable, high-stakes-grade result for ANY task — code, writing, research, ops — by
  treating the work as guilty until independent skeptics fail to break it. Runs the task through a
  dynamic many-subagent adversarial-verification Workflow that hunts for ten specific failure modes
  (silent assumptions, unmanaged confusion, hidden inconsistencies, unsurfaced tradeoffs, no
  pushback, sycophancy, subtle conceptual errors, overcomplication, dead code, orthogonal damage)
  and loops until they come back clean. Use this whenever the cost of a quiet mistake is high, when
  the user says "do this carefully / for real / no mistakes / high stakes", invokes /high-stakes, or
  asks for maximum effort / ultracode-grade rigor on a task they care about — even if they don't
  name the failure modes.
user-invocable: true
argument-hint: <the task to execute with high-stakes rigor>
allowed-tools: Bash, Read, Write, Edit, Glob, Grep, Workflow, Task, AskUserQuestion
---

# High-Stakes

The task to execute is in **$ARGUMENTS**. Treat that text as the work to deliver, not as a
description to discuss.

If $ARGUMENTS is empty (a bare `/high-stakes`, or "do this carefully" said about work already in the
conversation), the task is the most recent concrete request in the conversation. Name it explicitly
in your plan and confirm it before executing — never run the gauntlet against an empty task.

The organizing principle is **watch like a hawk**: a smart, hasty result is the default output of
any single agent, and its mistakes are no longer typos — they are subtle conceptual errors a sloppy
junior would make. The executor that wrote the artifact is its worst judge, so nothing it produces
is trusted until **independent skeptics try to break it and fail**. Your job is to define what
"correct and clean" means for this task, then loop that adversarial gauntlet until it comes back
empty — agents do not tire, so the looping against a fixed target is the leverage.

## The ten failure modes you are defending against

These are the specific bugs the skeptics hunt for. Every verifier maps to one. The plan-first gate
(step 2) targets modes 1–6, where a guess becomes invisible the moment you write confident prose
over it; execution and the gauntlet (steps 3–4) target modes 7–10.

1. **silent-assumptions** — filled an underspecified detail (path, data shape, edge case, version,
    default) with a guess and built on it without stating it. *The most common failure.*
2. **unmanaged-confusion** — hit genuine uncertainty and picked the first reading instead of
    asking.
3. **hidden-inconsistencies** — found a contradiction (spec vs. code, conflicting requirements) and
    silently picked a side instead of surfacing it.
4. **unsurfaced-tradeoffs** — several viable approaches existed; committed to one as if inevitable.
5. **no-pushback** — implemented a flawed premise faithfully instead of challenging it.
6. **sycophancy** — reflexive agreement and praise standing in for honest judgment.
7. **subtle-conceptual-errors** — compiles/reads clean but is wrong: off-by-one, bad boundary,
    misused API semantics, race, broken invariant.
8. **overcomplication** — 1000 lines where 100 would do; layers and knobs with one caller.
9. **dead-code** — leftover scaffolding, unused symbols, commented-out blocks, dead branches.
10. **orthogonal-damage** — touched, reformatted, or deleted code/comments unrelated to the task.

Not all apply to every task. For prose or research, 7–10 become "factual/logical errors", "padding
and hedging", "unsupported claims left in", and "edits beyond scope". Map them honestly to the
medium rather than skipping them.

## How to run it

### 1. Triage first — size the task before spending agents

Read $ARGUMENTS and skim the real inputs (the actual files, data, or sources — not your memory of
them). Then size the work, because the panel must scale to the risk, not balloon for its own sake
(an oversized panel is itself the overcomplication you are enforcing against):

- **Small / low-risk** (one file, a paragraph, a lookup): plan inline, do the work, run a single
    combined skeptic covering the modes that apply — always including dead-code and
    orthogonal-damage for any code change. No Workflow needed.
- **Medium** (a feature, a multi-section document, a focused investigation): run the Workflow with a
    trimmed panel — one skeptic per *relevant* mode.
- **Large / high-risk** (cross-cutting change, public artifact, anything irreversible): run the
    Workflow with the **full per-mode panel**, isolation worktrees for code, and a final adversarial
    re-check after fixes.

When unsure which bucket, pick the larger one — under-verifying a high-stakes task is the expensive
mistake. **Floor for code artifacts:** dead-code and orthogonal-damage checks are mandatory for any
change that touches code and may never be pruned as "not relevant" — leftover scaffolding and
out-of-scope edits pass all tests green and hide in any diff. Only pure prose/research may drop them
(using the 7–10 remap). Likewise, any task that fills in a path/shape/default/edge-case the prompt
did not dictate gets at least one independent assumption check against the plan before exit; the
authoring agent self-certifying that it "didn't guess" does not satisfy mode #1.

### 2. Plan-first gate — catch six modes before any work happens

Several failure modes are far cheaper to prevent than to detect later. Before executing, produce a
short plan (a tight list, not an essay) and **stop on it** if it surfaces blockers. Keep this plan —
the gauntlet checks the work against it, so it must be a real record, not aspiration:

- **Assumptions**: enumerate every detail $ARGUMENTS did not dictate that you are about to rely on.
    Verify each against the real code/data/sources and tag it `[verified: <evidence>]` or
    `[UNVERIFIED]`. Every `[UNVERIFIED]` item becomes a question to the user or an explicit caveat
    in the delivery — never a silent decision. (Defeats silent-assumptions.)
- **Questions**: name what is genuinely ambiguous and ask a specific question with `AskUserQuestion`
    when the choice materially changes the result. Do not paper a 50/50 over with confident prose.
    (Defeats unmanaged-confusion.)
- **Conflicts**: record every contradiction between the request, the existing code/data, and itself,
    verbatim, before choosing a resolution. Pass this list to the gauntlet so the inconsistency
    check can confirm each was surfaced rather than silently reconciled. (Defeats
    hidden-inconsistencies.)
- **Tradeoffs & merit**: name the main alternatives you considered *with the reason each was
    rejected*, and make your pick explicit; if you found only one viable approach, state that you
    looked and why nothing else qualifies. If the request's premise itself is flawed, say so with a
    concrete technical reason rather than complying. (Defeats unsurfaced-tradeoffs and no-pushback.)
- **Process cost**: when triage lands on medium or large, state in one line the verification cost
    you are about to incur (panel size, expected rounds, that it spins up many subagents) and let
    the user downscale or skip it. The cost of the workflow is itself a tradeoff (#4) — surface it;
    a user who said "do this carefully" has not necessarily authorized maximum spend.
- **Success criteria**: a short, checkable list defining "done correctly" for *this* task — the
    declarative target the gauntlet loops against, including the tests that must pass and the edge
    cases they must cover.

If a blocker is material, raise it with the user before burning the workflow on a wrong target. A
tiny task may have empty Questions/Conflicts — fine, but you still have to *look* and say so.

### 3. Execute the work

Before the first edit, **capture a baseline** of every file you will touch (ensure a clean `git`
state or snapshot the files) so a diff can be produced afterward — orthogonal damage lives in the
delta, not the final file. Then build to a first complete state. Favor the **naive-correct
construction first** (the simplest thing that fully solves the stated problem) and, for anything
testable, **write the check before the fix and watch it fail, then pass** — behavioral verification
beats reading clean-looking output. This is the executor's draft; assume it is guilty.

No artifact crosses into step 4 until at least one test exercising the success-criteria edge cases
exists and has been run. If none can be written, that is a stated limitation, not a pass.

### 4. Run the adversarial gauntlet (the Workflow)

For medium/large tasks you **must run the panel by issuing a `Workflow` tool call** built from
**`references/workflow-template.md`** — read it and adapt the panel to your triage decision rather
than re-deriving it. Do not substitute plain `Task` subagents or an inline self-review for the
panel: only the Workflow runtime gives the schema-forced verdicts, parallel fan-out, and
two-clean-rounds loop this skill depends on. (`Task` is fine for the executor/repair steps, never as
a stand-in for the gauntlet.) Its contract, briefly:

- **One skeptic per relevant failure mode**, each spawned via `agent(...)` with a `schema` that
    forces a verdict, and each handed the **Phase-1 plan and the diff of the edits** alongside the
    artifact — silent assumptions, reconciled contradictions, and deleted comments are invisible in
    the final file and only detectable against the plan and the diff. Prompt every skeptic to
    **assume the artifact is guilty of its mode and prove it**, and to **default to a finding when
    it cannot confirm cleanliness** — silence must be earned. The template gives each mode its
    evidentiary burden: correctness pastes a real test command and result; simplicity must name the
    simpler construction it searched for; tradeoffs must first enumerate the alternatives;
    sycophancy runs the strip-test on the conversational prose, not just the code.
- Run the skeptics with `parallel(...)` — they are independent and want the full artifact.
- **User-decision findings are not repairable.** A finding that names a genuine 50/50, an unresolved
    contradiction, or a flawed premise (modes unmanaged-confusion, hidden-inconsistencies,
    no-pushback) is a STOP condition, not a fixer task — the autonomous fixer "resolving" it just
    launders a guess into confident prose. Halt the loop and put the concrete objection to the user
    with `AskUserQuestion`, exactly as the Phase-1 gate would. The loop may never reach two clean
    rounds by having the fixer pick a side on a decision only the user can make.
- Feed the remaining findings into a **constrained repair pass** (fix only what each finding
    requires; introduce no new orthogonal damage or dead code; reject a finding that is wrong or a
    pure style nit with a one-line technical reason rather than complying — faithfully implementing
    a bad finding is sycophancy toward the skeptic). Then **loop**: re-run the skeptics. **Stop only
    after two consecutive clean rounds** — one can be luck. The gauntlet may not report convergence
    while any Phase-1 `[UNVERIFIED]` assumption remains neither verified nor stated as a caveat in
    the delivered artifact, nor while any user-decision finding is open. Cap the loop (the template
    carries a budget); if it will not converge, report the unresolved findings honestly rather than
    declaring victory.

### 5. Report honestly

Deliver the artifact (with absolute paths) plus a short ledger: which modes were checked, what the
skeptics found and what was fixed across rounds, the final all-clear or any finding still open, and
the assumptions made (verified vs. stated as caveats) and anything you pushed back on.

Before sending, **run the strip-test on this report**: delete every praise and agreement phrase and
confirm what remains is still complete, honest, technical content. If removing the filler leaves a
gap, you were using flattery to substitute for substance — rewrite it. No "you're absolutely right",
no smoothing — a stated limitation is worth more than confident prose hiding a guess.

## Guardrails on the skill itself

This skill must obey the rules it enforces. Match the panel to the triage size, prune skeptics that
have nothing to check, and offer the user the choice to downscale rather than silently committing
them to a maximal run. If following it ever feels like ritual out of proportion to the task, that is
the overcomplication check firing — shrink the fan-out and move on. The point is a clean result, not
a maximal process.
