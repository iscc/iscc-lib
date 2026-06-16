---
name: address-feedback
description: >-
  Triage and act on a code review. Takes reviewer feedback — from a human, a PR, or an external
  agent that lacked your plan and chat history — and works through it without assuming the reviewer
  is right. Verifies each item against the actual current code, applies the straightforward, valid,
  relevant, small, and uncontested fixes directly, and escalates the load-bearing or risky ones —
  anything that affects performance, backward compatibility, public API/behavior, or needs a larger
  refactor — to you as easy-to-read options, then runs the project's quality gates before reporting.
  Use when you say "address this review", "apply the review feedback", "go through the review
  comments", "handle the PR feedback", "act on this code review", "fix what the reviewer found", or
  after /code-review or an external agent produces findings. Do NOT use to *produce* a review (use
  code-review or high-stakes for that), nor to apply every suggested change blindly without verifying.
user-invocable: true
argument-hint: <paste the review, a path/PR to it, or leave empty to use the review in this conversation>
allowed-tools: Bash, Read, Edit, Write, Glob, Grep, AskUserQuestion
---

# Address Feedback

You have a code review and a codebase. Your job is to act on the review **responsibly**: apply the
fixes that are clearly safe, hand the consequential calls back to the user, and leave the result
green. The organizing principle is that **the reviewer is a witness, not a judge** — feedback often
comes from an external agent that never saw your plan, the chat history, deliberate design
decisions, or the latest code. So every item is a *claim to verify against the real code*, never an
instruction to execute on faith.

Two failures are equally bad and this skill defends against both: **applying a wrong or unwanted
change** because the reviewer said so, and **silently dropping a real issue** because verifying it
was inconvenient. The discipline below — verify, classify, apply-or-escalate, gate — keeps you off
both.

## Input — get the review

**$ARGUMENTS** may be: pasted review text, a path or URL to a review file, a PR number/link, or
empty. If empty (a bare `/address-feedback`, or "address this review" about findings already on
screen), use the most recent code review in the conversation. If you cannot find a review anywhere,
say so and stop — do not invent feedback.

Normalize the review into a **numbered list of atomic items**. Split bundled comments so each item
makes exactly one claim. Preserve the reviewer's wording and any `file:line` references — you will
need them to locate the code.

## Step 1 — Verify each item against the current code

Trust nothing until you've looked. For each item:

- **Locate** the exact code it refers to (`Grep`/`Read`). If you can't find it, the review may be
    stale or about a different branch — note that and lean toward rejecting.
- **Reproduce the reasoning** against reality: can the value actually be null? Is the race
    reachable? Is the API truly misused? Read the surrounding context, the callers, the tests,
    nearby comments, and `CLAUDE.md`/project conventions before believing the claim.
- **Render a validity verdict**: **Confirmed** (the problem is real), **Refuted** (the code is
    actually correct — the reviewer misread or worked from a stale snapshot), or **Needs-decision**
    (real only under an assumption or product choice you can't make alone).
- If the claim is right but the reviewer's **suggested fix is wrong or suboptimal**, fix the real
    problem — don't paste a bad patch just because a review proposed it.

Use whatever context this session *does* have (the conversation, the plan, `CLAUDE.md`, code
comments) to judge relevance. Where the context that would settle a call is genuinely missing and
the call is load-bearing, that's an escalation, not a guess.

## Step 2 — Classify each verified item

Sort every item into exactly one bucket. When torn between **apply** and **escalate**, escalate —
under-applying is cheap to correct; a wrong silent change is not.

### Apply directly — *all* of these must hold

- **Valid** (verified Confirmed) **and relevant** (in scope for the change under review).
- **Small** — localized, low blast radius, easily reversible.
- **Uncontested** — one obviously-correct fix; no reasonable engineer would disagree.
- **Behavior-preserving for anything public** — no change to an API, contract, output format, or
    naming that callers depend on.

Typical: typo, missing null/bounds guard, off-by-one with one correct bound, wrong error message,
unused import, missing `await`, incorrect comment, a clearly-missing test for existing behavior,
obvious resource leak (unclosed handle).

### Escalate to the user — *any* of these triggers it

- A **load-bearing decision** (design choice, data model, public contract, naming users depend on).
- Affects **performance**, **backward compatibility**, or **public API/behavior**.
- Needs a **larger refactor** or touches many files.
- Genuinely **ambiguous / multiple valid approaches / a contested tradeoff**.
- A **real problem whose right fix needs context you don't have** (the plan, product intent).
- **Security-sensitive or destructive** changes.

### Reject — with a concrete technical reason

- **Refuted** — the code is correct; show why (the guard that already exists, the invariant that
    holds).
- **Out of scope** — unrelated to the change under review.
- **Already handled / duplicate** of another item or existing code.
- **Deliberate decision** the reviewer lacked context for — state the decision plainly.
- **Pure style** that fights an existing, consistent project convention.

Never reject with a hand-wave. "The reviewer is wrong" without the reason is not a rejection — it's
the same unverified trust you're guarding against, just inverted.

For the expanded criteria, more examples, option-writing patterns, and a worked end-to-end example,
read `references/triage-playbook.md` when a call is non-obvious.

## Step 3 — Apply the direct fixes

- Capture a **baseline** first (confirm a clean `git status` or note the current diff) so the final
    change set is reviewable and orthogonal edits are visible.
- Make the **smallest** change that fixes the real issue; match the surrounding file's style.
- **Don't overengineer the fix.** Right-size it to the problem: no new abstraction, generalization,
    config knob, helper layer, interface, or defensive scaffolding the issue doesn't call for. A
    one-line correction stays one line; code with a single caller needs no abstraction; a fix needs
    no handling for inputs that can't occur. The simplest change that *fully* resolves the verified
    issue is the target — anything beyond it is the overcomplication you'd flag in someone else's
    diff. If a correct fix genuinely seems to require new structure or a broad refactor, that's an
    escalation signal (Step 2), not a licence to build it inline.
- Keep each fix **isolated** — no opportunistic refactoring, no reformatting unrelated lines, no
    fixing problems the review didn't raise (note those separately instead).

## Step 4 — Escalate the rest with grounded options

Use **`AskUserQuestion`** for the escalated items. Make each one trivial to decide:

- **One question per item.** Batch up to 4 per call. If there are more, do multiple rounds and put
    the load-bearing ones first.
- State the item and **what you verified**, so the user trusts your framing rather than re-checking.
- Give **2–4 concrete options**, each with its real implication/tradeoff in the description — e.g.
    *"Apply the reviewer's fix"*, *"Apply alternative X (keeps the API stable)"*, *"Leave as-is
    (reason)"*. When you have a defensible pick, make it the first option and append
    "(Recommended)".
- Keep options **mutually exclusive and skimmable** — a sentence each, not a paragraph. ("Other" is
    always available to the user for a custom answer.)

Apply exactly what the user chose; don't reopen settled choices. If a chosen option is itself large,
apply it with the same right-sized, smallest-change discipline as Step 3 — implement what was
chosen, not a more general version of it.

## Step 5 — Run the quality gates

After all fixes are in, leave the tree green. **Discover** the project's gates — don't hardcode
them: check `CLAUDE.md`'s commands section, `package.json` scripts, `Makefile`,
`pyproject.toml`/`uv`, a pre-commit/`prek` config, and CI workflow files. Run the relevant ones —
formatter/linter, type-check, tests, build — scoping to changed files where the tool allows.

- A gate that fails **because of a fix** → fix it, or revert that one fix and re-escalate, and say
    so.
- A **pre-existing** failure unrelated to your changes → report it, don't fix it (out of scope).
- This may run on **Windows** as well as Linux/macOS — prefer the project's own scripts over
    OS-specific shell commands.

If no gates exist, say so rather than claiming a pass you didn't run.

## Step 6 — Report

Give the user a ledger they can scan in seconds, then verify with the diff.

```markdown
## Applied (N)
| # | Item | What I verified | Fix |
|---|------|-----------------|-----|
| 1 | <reviewer's point> | <why it's real> | <the change, file:line> |

## Escalated (N)
| # | Item | Decision | Your choice → action |
|---|------|----------|----------------------|
| 2 | <point> | <the call you couldn't make alone> | <chosen option → what was done> |

## Rejected (N)
| # | Item | Why (technical reason) |
|---|------|------------------------|
| 3 | <point> | <Refuted/out-of-scope/deliberate — with evidence> |

## Quality gates
- <gate>: ✅/❌ `<command>` — <detail>

## Diff
<files touched — e.g. `git diff --stat`>
```

## Guardrails

- **Proportional.** A 3-item review gets three lines and maybe one question — not a ceremony.
- **No silent scope creep.** Issues you notice but the review didn't raise go in a short "Also
    spotted (not addressed)" note, never into the diff.
- **Honest reporting.** State gate failures with their output; never report a pass you didn't run.
- **Composes with siblings.** `code-review`/`high-stakes` *produce* a review; this skill *acts on*
    one. If the act-on step surfaces deep uncertainty about correctness, escalate or hand off to
    `high-stakes` rather than guessing.
