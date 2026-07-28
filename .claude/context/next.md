# Next Work Package

## Step: Re-baseline the CRAP gate for `decode_header`'s covered complexity increase

## Goal

Get CI green on `develop`. The pushed IDv1 batch reds exactly one enforcing gate — the CRAP
regression gate rejects `decode_header`'s cyclomatic increase (12→16, 100% covered) introduced by
the reviewed critical `u8::try_from` fix. Re-baselining `.crap-baseline.json` clears it. (The
`Semver (cargo-semver-checks)` red is a `continue-on-error: true` informational job, expected for
the intentional `#[non_exhaustive]` on `Version` pre-1.0 — NOT part of this step.)

## Alternatives Considered

- **Chosen:** re-baseline CRAP for the legitimate, fully-covered `decode_header` growth — it is the
    gate's own documented remedy (`mise run crap:baseline`), one generated file, zero source risk.
- **Rejected:** refactor `decode_header` to lower its CC — churns a critical fix that just passed
    review, hides four range-check branches that are inherent to the correctness fix, and re-opens
    codec.rs for no correctness gain (16 < the absolute `--fail-above` 30 threshold; fully covered).
- **Rejected (blocked):** start Part 2 `gen_iscc_id_v1` minting — CI-red-first; do not stack feature
    work onto a red develop tip.

## Scope

- **Modify**: `.crap-baseline.json` (generated artifact — outside the 3-file budget)
- **Reference**: `mise.toml` (`coverage`, `crap:baseline` tasks); `.github/workflows/ci.yml`
    L364-410 (the enforcing gate invocation); `crates/iscc-lib/src/codec.rs` `decode_header`

## Not In Scope

- **Do NOT touch `crates/iscc-lib/src/codec.rs`** — no refactor of the reviewed fix.
- **Do NOT drop `#[non_exhaustive]` from `codec::Version`, allowlist the semver lint, or edit the
    `semver` job.** Its red is informational-by-design pre-1.0; the spec mandates the marker.
- Do NOT begin `gen_iscc_id_v1` minting, the Go rename, or the Tier-1 32→33 doc sweep.
- Do NOT widen `.cargo-crap.toml` thresholds or the CRAP `--epsilon`.

## Implementation Notes

1. Regenerate: `mise run crap:baseline` (runs `cargo llvm-cov -p iscc-lib` then
    `cargo crap … --format json --output .crap-baseline.json`). Requires cargo-llvm-cov (0.8.7) +
    cargo-crap (0.2.2), both installed.
2. `git diff .crap-baseline.json`: the only **semantic** change (`cyclomatic`/`coverage`/`crap`)
    must be `decode_header` (~12→16). `line` fields for functions below the codec.rs edit may shift
    — benign. If any *other* function's crap/cyclomatic moved, stop: coverage drifted; do not bake
    it in.
3. The gate compares current-vs-baseline, so a fresh baseline yields zero regression and all scores
    stay < 30 → gate exits 0.

## Verification

- `mise run coverage` then
    `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression --fail-above`
    exits 0.
- `git diff --name-only` lists only `.claude/context/*` and `.crap-baseline.json` — NOT
    `crates/iscc-lib/src/codec.rs` (asserts no refactor on the working tree).
- In `.crap-baseline.json`, the `decode_header` entry's `cyclomatic` equals its current source value
    and no other function's `cyclomatic`/`crap` differs from the prior committed baseline.
- `cargo test -p iscc-lib` passes (no source change, sanity).

## Done When

`mise run coverage` + the CI CRAP gate command exit 0 against the regenerated `.crap-baseline.json`
with codec.rs untouched, so the `Coverage + CRAP` job goes green and the develop CI workflow
conclusion returns to success (the informational semver red aside).
