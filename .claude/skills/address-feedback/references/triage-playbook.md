# Triage Playbook

Expanded criteria, examples, and patterns for the `address-feedback` skill. Pull this in when a
classification call is non-obvious or you want a model for writing escalation options. The
`SKILL.md` decision matrix is the contract; this file is the worked detail behind it.

## The verification checklist (Step 1, in full)

An external reviewer's comment can be wrong for mundane reasons. Before believing any item, rule out
these "false-positive" causes — each is itself a valid rejection reason when it's the explanation:

| Cause                    | What it looks like                                                             | How to check                                                              |
| ------------------------ | ------------------------------------------------------------------------------ | ------------------------------------------------------------------------- |
| **Stale code**           | Comment cites a bug already fixed, or a `file:line` that no longer matches     | `Grep` the cited symbol; compare to the line the reviewer quoted          |
| **Misread control flow** | "This can be null" / "this isn't awaited" but a guard or `await` exists nearby | Read the full function and its callers, not just the cited line           |
| **Missing context**      | Flags a deliberate choice (a polyfill, a compat shim, an intentional `any`)    | Look for an explaining comment, a CLAUDE.md rule, or conversation history |
| **Wrong mental model**   | Assumes a framework/library behaves differently than it does                   | Confirm against the actual API/docs, not the reviewer's assumption        |
| **Scope confusion**      | Comments on code the current change didn't touch                               | Diff the change under review; is the line even in scope?                  |

A "Confirmed" verdict means you reproduced the problem against the real code — not that the comment
sounded plausible. A "Refuted" verdict means you found the specific reason it's wrong and can cite
it.

## Apply-directly — more examples

Safe to apply without asking, *when verified and in scope*:

- Typos in identifiers (pre-release), strings, comments, docs.
- A genuinely missing null/undefined/bounds guard on a reachable path.
- Off-by-one or wrong comparison operator where the correct bound is unambiguous.
- Unused import / variable / dead local with no side effect.
- A missing `await` / unhandled promise on an already-async path.
- Wrong or misleading log/error *message* (text only, not error *type* or code).
- An obvious resource leak — unclosed file/socket/handle on the happy path.
- A clearly-missing unit test for behavior that already exists and is settled.
- Tightening an overly-broad `catch`/`except` to the specific error already being handled elsewhere.

The through-line: a competent engineer seeing the diff would say "yes, obviously" — there is no
*decision*, only a *correction*.

## Escalate — more examples, and why

Each of these hides a decision the user owns:

- **"Rename this public method / flag / env var for clarity."** → backward compatibility; callers
    break. Escalate even if the new name is better.
- **"Switch this list to a set for O(1) lookups."** → changes iteration order and possibly
    observable behavior; a performance/semantics tradeoff.
- **"Extract these three functions into a new module."** → larger refactor; touches imports across
    files; reasonable people structure it differently.
- **"Add input validation here."** → what's the contract? Reject silently, throw, or coerce? Product
    decision.
- **"This whole approach should use streaming instead."** → architectural; rewrites the change.
- **"Cache this result."** → correctness (invalidation) and memory tradeoffs; load-bearing.
- **"Loosen this type to `any` to fix the error."** → trades type safety; usually the wrong fix and
    worth surfacing the real options.

When the *problem* is real but the *fix* is a judgment call, escalate the problem with options —
don't pick for the user and don't drop the problem.

## Reject — write the reason like a reviewer would

A rejection is itself a small review comment back. Make it specific and evidence-bearing:

- ✅ "Refuted — `cfg.timeout` is defaulted to `30` in `config.ts:12`, so the null-deref the comment
    describes can't occur on this path."
- ✅ "Out of scope — `legacy_parser.py` wasn't touched by this change; flagging it here mixes
    concerns. Noted separately for a future cleanup."
- ✅ "Deliberate — the `setTimeout(0)` is an intentional yield to unblock the event loop; see the
    comment on `scheduler.ts:88`."
- ❌ "Disagree." / "Not needed." / "The reviewer is wrong." — no evidence; this is unverified trust
    inverted.

## Writing AskUserQuestion options (Step 4)

The user should be able to decide each escalation in a few seconds. Aim for this shape:

```
header:   <≤12 chars, e.g. "Rename API">
question: <the item + what you verified + the call you can't make alone>
options:
  - label: "Keep current name (Recommended)"
    description: "No caller breakage. The current name is acceptable; the reviewer's is marginally clearer."
  - label: "Rename + add deprecated alias"
    description: "Adopts the clearer name, keeps old callers working via a shim. Two symbols to maintain."
  - label: "Rename outright"
    description: "Cleanest result, but breaks external callers. Fine only if nothing depends on it yet."
```

Rules of thumb:

- **Ground the framing.** One clause of "what I verified" earns the user's trust in your options.
- **Each option carries its consequence**, not just its action — "(keeps the API stable)", "(breaks
    callers)", "(adds a dependency)".
- **Recommend when you can defend it.** First option, "(Recommended)" suffix. Don't recommend on a
    true 50/50 — present it neutrally.
- **Make them exclusive and short.** A sentence each. Long options read as work, and the user bails.
- **Always leave room to decline.** A "Leave as-is" option is legitimate; the user's "Other" is
    automatic.

## Worked example

**Review (from an external agent, no plan/context):**

1. "`parseConfig` can throw on missing file — should be wrapped in try/catch."
2. "Variable `usr` should be `user` for readability."
3. "This loop is O(n²); use a Map for the lookup."
4. "`MAX_RETRIES` is hardcoded to 3 — make it configurable."
5. "The `formatDate` import is unused."

**Triage:**

- **#1 → Escalate.** Verified `parseConfig` *can* throw, but the right behavior is a product call:
    fail fast with a clear message, or fall back to defaults? Options offered.
- **#2 → Reject (out of scope).** `usr` is used consistently across this module and 30 other call
    sites; renaming one is inconsistent and the rest is out of scope. Noted for a project-wide pass.
- **#3 → Escalate.** Confirmed the O(n²); but `n` is bounded to ~5 here (verified at the call site),
    so the Map adds complexity for no real gain. Offer "leave as-is (n is tiny)" as recommended,
    "use a Map" as the alternative.
- **#4 → Escalate.** Touches the public config surface — backward-compat and naming decision.
- **#5 → Apply.** Verified `formatDate` has no references in the file. Removed the import.

**Then:** run the project's gates (e.g. lint + tests), report the ledger with #5 under Applied,
#1/#3/#4 under Escalated with the user's choices, and #2 under Rejected.

Note how verification flips two "obvious" suggestions: #3 looks like a clear win until you check
that `n` is tiny, and #2 looks trivial until you check that it fights an established convention.
