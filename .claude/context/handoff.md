## 2026-07-29 — Reject empty-cleaned codec input to close the `iscc_decompose` regression

**Done:** Added an empty-cleaned-input guard inside `codec::iscc_clean` — it now returns
`Err(InvalidInput("Empty ISCC string"))` when the cleaned result is `""`. This protects all four
call sites (`iscc_decompose`, `iscc_normalize`, `gen_iscc_code_v0`, `gen_mixed_code_v0`) at once and
closes the HEAD regression where `iscc_decompose("   ")` etc. silently returned `Ok([])`.

**Files changed:**

- `crates/iscc-lib/src/codec.rs`: `iscc_clean` restructured to bind a `cleaned` value, then guard
    for `is_empty()` before returning `Ok`. Logic for valid inputs (multibase dash-preservation,
    scheme check, malformed-colon) is unchanged; only the empty case is newly rejected.
- `crates/iscc-lib/tests/codec_clean.rs`: added 4 tests asserting `Err(InvalidInput)` for `"   "`,
    `"-"`, `"iscc:"`, `"----"` across `iscc_decompose`, `iscc_decode` (routes through the private
    `iscc_normalize`), `gen_iscc_code_v0`, and `gen_mixed_code_v0`.
- `.crap-baseline.json`: regenerated — `iscc_clean` cyclomatic 6→7 (crap 6→7, 100% covered),
    `iscc_decompose` line 559→570. No epsilon/threshold change.

**Verification:**

- `cargo test -p iscc-lib` — all pass, 0 failed (codec_clean 10/10, incl. the 4 new empty-input
    tests; existing valid-input differential cases still green).
- `cargo clippy -p iscc-lib --all-targets -- -D warnings` clean; `cargo fmt -p iscc-lib --check`
    clean (fmt reformatted the `_ =>` arm with braces + semicolon — cosmetic only).
- `mise run coverage` + `mise run crap:baseline` — baseline regenerated; `.cargo-crap.toml`
    untouched (threshold 30.0).
- `mise run check` (prek all-files) — all hooks Passed.

**Next:** The Go half of the codec `iscc_clean` divergence (issues.md — non-mechanical port of the
four `packages/go` sites) is now unblocked. It MUST NOT copy the empty-input gap: replicate this
empty-cleaned guard in the Go port. The Ruby `gen_iscc_id_v1` validation-order fix remains the last
`normal` codec-adjacent item.

**Notes:**

- Error message is a fixed `"Empty ISCC string"` (does not mirror Python's IndexError text, per
    next.md — only that the input errors matters).
- `iscc_normalize` is private (not Tier 1), so the empty-input test for it is driven through its
    public entry point `iscc_decode`. Confirmed `gen_*_v0` sites error via the same guard now rather
    than only downstream via `decode_header([])` / the `< 2` unit-count check.
- `iscc_decompose("")` (truly empty string) was `Ok([])` at HEAD~1 too — it is now also rejected by
    this guard, which tightens behavior toward the reference (not a behavior the step required, but
    consistent and correct).
