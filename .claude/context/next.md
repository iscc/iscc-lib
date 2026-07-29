# Next Work Package

## Step: Reject empty-cleaned codec input to close the `iscc_decompose` regression

## Goal

Close the NEEDS_WORK regression at HEAD (`2537e18`, unpushed): after routing through the shared
`iscc_clean` helper, `iscc_decompose` silently returns `Ok([])` for inputs that clean to `""`
(`"   "`, `"-"`, `"iscc:"`, `"----"`) where both HEAD~1 and the reference error. Landing this makes
the Rust `iscc_clean` routing a clean PASS so it can push under green CI.

## Alternatives Considered

- **Chosen:** guard empty-cleaned input — it is the sole blocker keeping the in-flight Rust routing
    off `origin/develop`; a small, reviewer-specified fix that unblocks the whole work package.
- **Rejected:** the Go half of the same `iscc_clean` divergence (issues.md) — it is a separate
    non-mechanical port that must not copy this empty-input gap, so it can only start once the Rust
    half lands clean and is pushed.

## Scope

- **Modify**: `crates/iscc-lib/src/codec.rs` — add the empty-cleaned-code guard
- **Modify (tests, excl. budget)**: `crates/iscc-lib/tests/codec_clean.rs` — assert the four forms
    error across all four codec-input sites
- **Modify (generated, excl. budget)**: `.crap-baseline.json` — regen after the new branch/tests
- **Reference**: `reference/iscc-core/iscc_core/codec.py:644` (`iscc_clean`, `code[0]` IndexError on
    empty); handoff.md (regression detail); learnings.md (`decode_base32("")` returns `Ok(empty)`)

## Not In Scope

- The Go `iscc_clean` port (`packages/go`) — separate step, blocked on this landing.
- Re-deriving or altering the codec-cleaning design — it matches the reference and is settled; only
    the empty-input edge needs closing.
- Any change to the multibase dash-preservation logic or the valid-input differential cases.

## Implementation Notes

- Put the guard **inside `codec::iscc_clean`** (codec.rs:529): after computing the cleaned string,
    return `Err(IsccError::InvalidInput(...))` when it is empty. This is DRY — it protects all four
    call sites (`iscc_decompose`, `iscc_normalize`, `gen_iscc_code_v0`, `gen_mixed_code_v0`) at
    once, and mirrors the reference erroring on empty (IndexError in the one-part branch; downstream
    for the `"iscc:"` two-part form). Alternatively guard before the loop in `iscc_decompose`, but
    the single-site guard is preferred.
- `decode_base32("")` returns `Ok(empty)`, NOT an error — do not rely on it to reject empty.
- Error message can be a fixed string (e.g. `"Empty ISCC string"`); it need not match Python's
    exception text verbatim, only that the input errors.
- The other three sites already error on empty downstream (via `decode_header([])` / the `< 2` unit
    count check); the guard just makes them error earlier and consistently — confirm with a test.
- The existing valid-input differential cases (hyphenated / padded / lowercase-scheme) must stay
    green — the guard only fires on a truly empty cleaned result.
- Adding a branch to the covered `iscc_clean` shifts its CRAP value → regenerate
    `.crap-baseline.json` via `mise run coverage` then `mise run crap:baseline`; do NOT widen the
    epsilon or the 30.0 threshold in `.cargo-crap.toml`.

## Verification

- `cargo test -p iscc-lib` passes (all existing tests + new empty-input assertions, 0 failed).
- New tests in `codec_clean.rs` assert `iscc_decompose` returns `Err(InvalidInput)` for `"   "`,
    `"-"`, `"iscc:"`, `"----"`, and that `iscc_normalize` / `gen_iscc_code_v0` / `gen_mixed_code_v0`
    also error (not `Ok`) on a whitespace/dash-only form.
- Existing valid-input differential cases still pass (hyphenated composite → 4 units; padded and
    lowercase-scheme single → 1 unit each); multibase `u`-prefixed dashes still preserved.
- `cargo clippy -p iscc-lib --all-targets -- -D warnings` clean; `cargo fmt -p iscc-lib --check`
    clean.
- `mise run check` (prek all-files) passes; `.crap-baseline.json` regenerated with no
    epsilon/threshold change (`.cargo-crap.toml` untouched).

## Done When

All verification criteria pass and the four empty-cleaned forms error through every codec-input
site.
