# Next Work Package

## Step: Refresh `.crap-baseline.json` for the iter-121 `iscc_decode` change (fix RED CI)

## Goal

CI is RED on the develop tip: the iter-121 `iscc_decode` trailing-byte guard added one branch,
raising its cyclomatic complexity above the committed CRAP baseline (`cyclomatic 4.0 / crap 4.11`),
so the enforcing `Coverage + CRAP` job fails its `--fail-regression` gate. Regenerate the committed
`.crap-baseline.json` so the gate passes — the added complexity is legitimate and far below the 30.0
`--fail-above` cap, so no source revert is warranted. Green CI is a prerequisite for all other work.

## Scope

- **Modify**: `.crap-baseline.json` (regenerated committed CRAP regression baseline)
- **Reference**:
    - `.claude/context/state.md` (root-cause writeup, "Next Milestone")
    - `mise.toml` lines 118–130 (`coverage`, `crap`, `crap:baseline` tasks)
    - `.github/workflows/ci.yml` lines 350–395 (the enforcing `Coverage + CRAP` job command)
    - `.cargo-crap.toml` (threshold 30.0, `missing = "pessimistic"`, exclude list)

## Not In Scope

- **Do NOT revert or modify the iter-121 `iscc_decode` source fix** in `crates/iscc-lib/src/lib.rs`
    — the "too short" + "too long" two-branch guard is correct, conformance-safe (review PASS), and
    its complexity increase is legitimate.
- Do NOT widen any `--epsilon` tolerance or lower the `--fail-above` threshold in `.cargo-crap.toml`
    to mask the regression.
- Do NOT hand-tune other baseline entries' substantive metrics — only `iscc_decode`'s
    cyclomatic/coverage/crap should change materially (line-number shifts from iter 121 are fine).
- Do NOT start on #49 (aarch64 Python wheels) or the dependency refresh — those come only after CI
    is green.
- Do NOT commit a baseline downloaded from a CI artifact without regenerating locally first (the
    local run is the reviewable source of truth).

## Implementation Notes

- Regenerate the whole file the documented way: `mise run crap:baseline` (depends on `coverage`, so
    it runs `cargo llvm-cov -p iscc-lib --lcov --output-path lcov.info` then
    `cargo crap --lcov lcov.info --format json --output .crap-baseline.json`). cargo-crap 0.2.2 and
    cargo-llvm-cov 0.8.7 are installed in the devcontainer.
- After regenerating, inspect `git diff .crap-baseline.json`. Expected changes:
    - The `iscc_decode` entry (near line 277) gets a new `cyclomatic` (was `4.0`, the added branch
        makes it ~`5.0`), a fresh `coverage`, and a recomputed `crap`.
    - Many entries' `line:` fields shift because iter 121 added ~7 lines to `lib.rs` — these line-only
        shifts are legitimate and expected; leave them.
    - No OTHER entry's `cyclomatic`/`coverage`/`crap` should change materially. If one does, that is
        coverage-environment noise — investigate before committing rather than blindly accepting it.
- The gate CI runs is exactly:
    `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression --fail-above`. Run
    it locally against the freshly regenerated `lcov.info` and the new baseline; it must exit 0 with
    no `↑ N regressed`.
- Guard note: the CRAP regression gate is CI-only (not in `mise run check`/pre-commit), which is why
    iter 121 slipped through. Any future change that adds a branch/loop to a covered function needs
    a baseline refresh in the same step.
- Fallback if local regeneration still trips CI (coverage drift between devcontainer and CI): pull
    the `lcov.info`/baseline artifact from the failed CI run and regenerate from that instead of
    widening epsilon.

## Verification

- `mise run coverage` produces a fresh `lcov.info` (exit 0).
- `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression --fail-above` exits
    0 (no `↑ N regressed`, nothing above the 30.0 cap).
- `git diff .crap-baseline.json` shows the `iscc_decode` entry's `cyclomatic` is no longer `4.0` and
    its `crap` is no longer `4.110571212612029` (baseline refreshed to the current value).
- `cargo test -p iscc-lib` still passes (source unchanged; sanity check that the tree is intact).
- `cargo clippy -p iscc-lib --all-features --all-targets -- -D warnings` clean (unchanged source).

## Done When

`cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression --fail-above` exits 0
against a freshly regenerated `.crap-baseline.json`, so the `Coverage + CRAP` CI job goes green
while the iter-121 `iscc_decode` robustness fix stays in place.
