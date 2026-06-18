# Next Work Package

## Step: Confirm human-handoff state — no autonomous CID work package remains

## Goal

Record that every target section is met except three gaps that are all blocked on human input (the
v1.0.0 release cut and two HUMAN-REVIEW-REQUESTED spec amendments), so the advance agent makes no
code changes this iteration and the review agent can signal the human-handoff / IDLE state instead
of the loop inventing churn.

## Scope

- **Create**: (none)
- **Modify**: (none — no source, test, or doc files change this iteration)
- **Reference**: `.claude/context/state.md` (Next Milestone), `.claude/context/handoff.md` (Next),
    `.claude/context/issues.md` (the 2 `normal` [review] + 2 `low` [human] items),
    `.claude/agents/review.md` (Idle Iterations section).

## Not In Scope

- Do NOT implement the CRAP `--fail-above 30` gate — it is a HUMAN-REVIEW-REQUESTED spec amendment
    (`ci-cd.md`), not autonomous work.
- Do NOT wire up the `cargo deny`/`cargo audit` supply-chain gate — also HUMAN-REVIEW-REQUESTED (the
    requirement lives only in `notes/07`, not the CID specs).
- Do NOT flip the `Semver (cargo-semver-checks)` gate to enforcing or start any v1.0.0 release prep
    — both are tied to the human-driven v1.0.0 cut (`low` [human] issue).
- Do NOT pick up either `low` [human] issue (v1.0.0 cut, docs logos).
- Do NOT invent refactors, new tests, or doc edits to manufacture a work package.

## Implementation Notes

There is genuinely no fully-autonomous `normal`-or-higher work package left:

- CI is GREEN on the pushed tip (`6d6c594`, run 27753395707); all 19 functional jobs pass (the only
    job-level `failure` is the informational `continue-on-error` Semver job).
- All 12 language bindings, README, per-crate READMEs, docs, and benchmarks are met.
- The iai-callgrind perf gate is complete, enforcing, hardened, and CI-verified.
- The three remaining gaps are each blocked on human input (see Not In Scope).

The advance agent should:

1. Re-confirm the above by reading `state.md` / `issues.md` (a light sanity check such as
    `git status` clean and `cargo test -p iscc-lib` still green is fine) WITHOUT modifying any
    source, test, or doc file.
2. Make NO code changes. Leave `issues.md` untouched (the review agent owns issue resolution).
3. Hand off to the review agent so it can evaluate the Idle Iterations conditions. Note for review:
    strict IDLE condition #2 ("every issue is `low`") is NOT met because the 2 `normal` [review]
    issues remain, but both are HUMAN-REVIEW-REQUESTED spec amendments with no autonomous action
    for CID — this is a human-handoff point, not invented work.

If the advance agent believes it has found genuinely autonomous, non-spec-amending, in-scope work
that advances an unmet target criterion, it must STOP and surface that to the human rather than
silently expanding scope.

## Verification

- `git diff --stat` against the pre-iteration tip shows no changes to any file under `crates/`,
    `packages/`, `scripts/`, `docs/`, or `.claude/context/specs/` (advance made no code/doc
    changes).
- `issues.md` still lists exactly 4 issues (2 `normal` [review] HUMAN REVIEW REQUESTED, 2 `low`
    [human]) — none were auto-started.
- `cargo test -p iscc-lib` still passes (optional sanity; unchanged from the GREEN tip).

## Done When

The advance agent has made no code changes and confirmed the loop is at a human-handoff point, so
the review agent can assess the Idle Iterations / human-handoff signal.
